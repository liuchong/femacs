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
  /// C: dispextern.h 3574-3575: `void window_box (struct window *, enum glyph_row_area, int *,
  /// int *, int *, int *);` - the last argument is C's `0`, so the port passes the null pointer.
  fn window_box (w: *mut window, area: glyph_row_area, x: *mut c_int, y: *mut c_int,
                 width: *mut c_int, height: *mut c_int);
  /// C: dispextern.h 3579-3580: `int window_box_left (struct window *, enum glyph_row_area);`
  /// and `int window_box_left_offset (struct window *, enum glyph_row_area);` - the two the
  /// stretch-glyph-string path reads (C 1414/1481).
  fn window_box_left (w: *mut window, area: glyph_row_area) -> c_int;
  fn window_box_left_offset (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3692: `extern bool cursor_in_mouse_face_p (struct window *w);` (xdisp.c)
  fn cursor_in_mouse_face_p (w: *mut window) -> bool;
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
   masked accessors of PORTING.md 1.3/P-80, named after C's own members.

   **`HAVE_WINDOW_SYSTEM` is defined in this configuration, and C 1175 is the proof**: it reads
   `s->face->stipple`, a member C declares only inside dispextern.h 1797-1811's
   `#ifdef HAVE_WINDOW_SYSTEM` (with `gc` in front of it).  Slice 2's
   `haiku_draw_stipple_background` fetches that id, so the copy carries both members and every
   offset above `id` moves by the same sixteen bytes the mirror's `FACE_WS_SHIFT` names
   (`include/dispextern.rs` 2280-2283): `docs/baseline/face-layout.tsv` measures `sizeof` 296,
   align 8, `gc`@168, `stipple`@176 and `foreground`@184 for both window-system arms, and its
   `ns_cocoa` arm is the same sixteen bytes with the record's Haiku `#else` spelling of `gc`
   (`Emacs_GC *`, the `ftcrfont.rs` 769-784 reading).  The arm without the window system keeps
   the 280-byte layout the frozen clang measured for the reference target, and both arms'
   offsets stay asserted.  */
#[cfg (feature = "have_haiku")]
#[cfg (not (feature = "have_window_system"))]
const FACE_WS_SHIFT: usize = 0;
#[cfg (feature = "have_haiku")]
#[cfg (feature = "have_window_system")]
const FACE_WS_SHIFT: usize = 16;

#[cfg (feature = "have_haiku")]
#[repr(C)]
pub struct face
{
  pub lface: [Lisp_Object; LFACE_VECTOR_SIZE as usize],
  pub id: c_int,
  /* dispextern.h 1797-1811: the `#ifdef HAVE_WINDOW_SYSTEM` pair this configuration defines.
     `gc`'s Haiku arm is C's `#else` (`Emacs_GC *gc;`), untyped here (P-72 rule 3) - the
     `ftcrfont.rs` 776 spelling.  */
  #[cfg (feature = "have_window_system")]
  pub gc: *mut c_void,
  #[cfg (feature = "have_window_system")]
  pub stipple: ptrdiff_t,                   // C 1810 (measured 176)
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
const _: () = core::assert! (std::mem::size_of::<face> () == 280 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::align_of::<face> () == 8);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, id) == 160);
#[cfg (all (feature = "have_haiku", feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, gc) == 168);
#[cfg (all (feature = "have_haiku", feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, stipple) == 176);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, foreground) == 168 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, background) == 176 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, underline_color) == 184 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, overline_color) == 192 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, strike_through_color) == 200 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, box_color) == 208 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, font) == 216 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, fontset) == 224 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, bits) == 240 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, hash) == 248 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, next) == 256 + FACE_WS_SHIFT);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, ascii_face) == 272 + FACE_WS_SHIFT);

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
  /* C 1121 reads `face->background_defaulted_p` (the flag next to the underline ones above).  */
  (background_defaulted_p, $f:expr) => { face_bit_p ($f, FACE_BACKGROUND_DEFAULTED_P) };
  (background_defaulted_p, $f:expr, $v:expr) =>
    { face_bit_set ($f, FACE_BACKGROUND_DEFAULTED_P, $v) };
  (underline_at_descent_line_p, $f:expr) => { face_bit_p ($f, FACE_UNDERLINE_AT_DESCENT_LINE_P) };
  (underline_at_descent_line_p, $f:expr, $v:expr) =>
    { face_bit_set ($f, FACE_UNDERLINE_AT_DESCENT_LINE_P, $v) };
  /* C 1304 reads `face->overstrike`, the `bool_bf overstrike : 1` at dispextern.h 1931.  */
  (overstrike, $f:expr) => { face_bit_p ($f, FACE_OVERSTRIKE) };
  /* C 1081/1099 read `face->box`, the `ENUM_BF (face_box_type) box : 2` at dispextern.h 1893 -
     the mask/shift above are this file's - and the arm is the `pgtkterm.rs` 1960 shape.  */
  (box, $f:expr) => { (((*$f).bits & FACE_BOX_MASK) >> FACE_BOX_SHIFT) as c_int };
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
unsafe extern "C" fn haiku_toolkit_position (f: *mut frame, x: c_int, y: c_int,
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
/* C frame.h 434: `bool_bf mouse_moved : 1;` - written at C 3007.  The mirror names the shift
   (`include/frame.rs` 529) and no accessor, so the unit reads and writes the bit itself (the
   `androidterm.rs` 234-238 copy of this same member).  */
#[cfg (feature = "have_haiku")]
macro_rules! mouse_moved {
  ($f:expr) => { (*$f).bits1 >> FRAME_MOUSE_MOVED_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_MOUSE_MOVED_SHIFT, $v) };
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
    { rgbin = (*((*s).face as *mut face)).box_color as uint32_t; }
  else if glyph_bits! (type, *(*s).first_glyph) == IMAGE_GLYPH
	   && ! (*(*s).img).pixmap.is_null ()
	   && IMAGE_BACKGROUND_TRANSPARENT! ((*s).img, (*s).f, 0 as Emacs_Pixmap) == 0
    { rgbin = IMAGE_BACKGROUND! ((*s).img, (*s).f, 0 as Emacs_Pixmap) as uint32_t; }
  else
    { rgbin = (*((*s).face as *mut face)).background as uint32_t; }

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
    { haiku_merge_cursor_foreground (s, &raw mut cursor_color,
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

/* C 1045-1198: `haiku_draw_string_box` (C 1046-1106), `haiku_draw_plain_background` (C 1108-1126),
   `haiku_get_bitmap_rec` (C 1128-1132), `haiku_update_bitmap_rec` (C 1134-1161) and
   `haiku_draw_stipple_background` (C 1163-1198).  The next item is C 1200's `haiku_draw_image`.
   C 1063-1074 is the file's only reader of the `glyph.u.cmp` bitfield run, so its accessor is
   declared below (the `pgtkterm.rs` 2102-2111 shape, whose comment cites the
   `src/xterm.rs`/`src/w32term.rs` copies of the same macro).  */

/* The one window bit this slice's text names (`w->pseudo_window_p`, C 1055).  `window.rs` has the
   reader function `WINDOW_PSEUDO_P` and the shift constant; a unit-local `window_bits!` keeps C's
   member name at the call site (the `src/xterm.rs` 8634-8643 shape).  */
#[cfg (feature = "have_haiku")]
macro_rules! window_bits {
  (pseudo_window_p, $w:expr) => { ((*($w)).bits >> WINDOW_PSEUDO_WINDOW_P_SHIFT) & 1 != 0 };
  /* C 438's `bool_bf phys_cursor_on_p : 1` - **written** at C 2326.  The mirror names the shift
     (`include/window.rs` 191) and carries no setter, so the unit writes the masked store (the
     `pgtkterm.rs` 2875-2882 copy of this same arm).  */
  (phys_cursor_on_p, $w:expr, $v:expr) =>
    { (*($w)).bits = if $v { (*($w)).bits | (1 << WINDOW_PHYS_CURSOR_ON_P_SHIFT) }
                     else { (*($w)).bits & ! (1 << WINDOW_PHYS_CURSOR_ON_P_SHIFT) } };
}

/* C 597-598's `bool_bf automatic : 1; unsigned id : 31;` are the `glyph.u.cmp` members C 1064 and
   C 1071 name; the constants are `include/dispextern.rs`'s.  */
#[cfg (feature = "have_haiku")]
macro_rules! glyph_u_cmp_bits {
  (automatic, $g:expr) => { (((*$g).u.cmp.bits & GLYPH_U_CMP_AUTOMATIC) != 0) };
  (id, $g:expr) => { (((*$g).u.cmp.bits & GLYPH_U_CMP_ID_SHIFT) as ptrdiff_t) };
}

/* C 1046-1106.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_string_box (s: *mut glyph_string)
{
  let mut hwidth: c_int;                                          // C 1049
  let mut vwidth: c_int;
  let mut left_x: c_int;
  let mut right_x: c_int;
  let mut top_y: c_int;
  let mut bottom_y: c_int;
  let mut last_x: c_int;
  let mut raised_p: bool;                                         // C 1050
  let mut left_p: bool;
  let mut right_p: bool;
  let mut last_glyph: *mut glyph;                                 // C 1051
  let face: *mut face = GS_FACE! (s);                             // C 1052

  last_x = if glyph_row_bits! (full_width_p, (*s).row)             // C 1054-1055
             && ! window_bits! (pseudo_window_p, (*s).w)
	   { WINDOW_RIGHT_EDGE_X! ((*s).w) }                      // C 1055
	   else
	   { window_box_right ((*s).w, (*s).area) };              // C 1056

  /* The glyph that may have a right box line.  For static
     compositions and images, the right-box flag is on the first glyph
     of the glyph string; for other types it's on the last glyph.  */   // C 1058-1060
  if !(*s).cmp.is_null () || !(*s).img.is_null ()                  // C 1061
    { last_glyph = (*s).first_glyph; }                             // C 1062
  else if glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH   // C 1063
	   && glyph_u_cmp_bits! (automatic, (*s).first_glyph)      // C 1064
    {
      /* For automatic compositions, we need to look up the last glyph
	 in the composition.  */                                  // C 1066-1067
      let end: *mut glyph = (*((*s).row)).glyphs[(*s).area as usize]   // C 1068
	.offset ((*((*s).row)).used[(*s).area as usize] as isize);
      let mut g: *mut glyph = (*s).first_glyph;                    // C 1069
      /* C 1070-1074's `for (last_glyph = g++; COND; last_glyph = g++) ;`: the increment is the
	 assignment itself, so the port writes C's two statements at the end of the loop's body
	 (the body is C's empty `;`).  */
      last_glyph = g;                                              // C 1070 (`last_glyph = g++`)
      g = g.offset (1);
      while g < end && glyph_u_cmp_bits! (automatic, g)            // C 1071
	    && glyph_u_cmp_bits! (id, g) == (*s).cmp_id             // C 1071
	    && (*g).slice.cmp.to < (*s).cmp_to                       // C 1072
	{
	  last_glyph = g;                                           // C 1073 (`last_glyph = g++`)
	  g = g.offset (1);
	}
    }
  else
    { last_glyph = (*s).first_glyph.offset ((*s).nchars as isize - 1); }   // C 1077

  vwidth = eabs! ((*face).box_vertical_line_width);                 // C 1079
  hwidth = eabs! ((*face).box_horizontal_line_width);               // C 1080
  raised_p = face_bits! (box, face) == FACE_RAISED_BOX;             // C 1081
  left_x = (*s).x;                                                  // C 1082
  right_x = if glyph_row_bits! (full_width_p, (*s).row)             // C 1083
	       && glyph_string_bits! (extends_to_end_of_line_p, (*s))
	      { last_x - 1 }                                        // C 1084
	      else
	      { std::cmp::min (last_x, (*s).x + (*s).background_width) - 1 };   // C 1085

  top_y = (*s).y;                                                   // C 1087
  bottom_y = top_y + (*s).height - 1;                               // C 1088

  left_p = glyph_bits! (left_box_line_p, (*(*s).first_glyph))       // C 1090
	    || ((*s).hl == DRAW_MOUSE_FACE                         // C 1091
		&& ((*s).prev.is_null ()                           // C 1092
		    || (*(*s).prev).hl != (*s).hl));                // C 1093
  right_p = glyph_bits! (right_box_line_p, (*last_glyph))           // C 1094
	     || ((*s).hl == DRAW_MOUSE_FACE                        // C 1095
		 && ((*s).next.is_null ()                          // C 1096
		     || (*(*s).next).hl != (*s).hl));               // C 1097

  if face_bits! (box, face) == FACE_SIMPLE_BOX                      // C 1099
    { haiku_draw_box_rect (s, left_x, top_y, right_x, bottom_y, hwidth,   // C 1100-1101
			   vwidth, left_p, right_p, std::ptr::null_mut ()); }
  else
    { haiku_draw_relief_rect (s, left_x, top_y, right_x, bottom_y, hwidth,   // C 1103-1104
			      vwidth, raised_p, true, true, left_p, right_p,
			      std::ptr::null_mut ()); }             // C 1105
}

/* C 1108-1126.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_plain_background (s: *mut glyph_string, face: *mut face,
				       x: c_int, y: c_int, width: c_int, height: c_int)
{
  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! ((*s).f);           // C 1112
  /* C 1113 declares `cursor_color` with no initialiser and C 1117 passes its address, so the port
     zero-initialises it (PORTING.md 1.5).  */
  let mut cursor_color: c_ulong = 0;
  let _ = &mut cursor_color;

  if (*s).hl == DRAW_CURSOR                                         // C 1115
    {
      /* The mirror's declaration takes `*mut c_void` where C's takes `struct glyph_string *`
	 (P-72 rule 3; the file's own 1391 call).  */
      haiku_merge_cursor_foreground (s, std::ptr::null_mut (),
				     &raw mut cursor_color);   // C 1117
      BView_SetHighColor (view, cursor_color as uint32_t);           // C 1118
    }
  else
    { BView_SetHighColor (view, if face_bits! (background_defaulted_p, face)   // C 1121
				{ FRAME_BACKGROUND_PIXEL ((*s).f) as uint32_t }   // C 1122
				else { (*face).background as uint32_t }); }   // C 1123

  BView_FillRectangle (view, x, y, width, height);                  // C 1125
}

/* C 1128-1132.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_get_bitmap_rec (f: *mut frame, id: ptrdiff_t) -> *mut haiku_bitmap_record
{
  /* C's `&FRAME_DISPLAY_INFO (f)->bitmaps[id - 1]`: the mirror types `bitmaps` as a raw
     pointer, so the element address is an offset (the `alloc.rs` 464 shape).  */
  return (*FRAME_DISPLAY_INFO! (f)).bitmaps.offset ((id - 1) as isize);   // C 1131
}

/* C 1134-1161.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_update_bitmap_rec (rec: *mut haiku_bitmap_record,
				   new_foreground: uint32_t,
				   new_background: uint32_t)
{
  let mut bits: *mut c_char;                                        // C 1139
  let mut x: c_int;                                                 // C 1140
  let mut y: c_int;
  let mut bytes_per_line: c_int;

  if new_foreground == (*rec).stipple_foreground                     // C 1142
      && new_background == (*rec).stipple_background                 // C 1143
    { return; }                                                     // C 1144

  bits = (*rec).stipple_bits as *mut c_char;                        // C 1146
  bytes_per_line = ((*rec).width + 7) / 8;                          // C 1147

  y = 0;                                                            // C 1149
  while y < (*rec).height
    {
      x = 0;                                                        // C 1151
      while x < (*rec).width
	{
	  haiku_put_pixel ((*rec).img, x, y,                         // C 1152
			   (if ((*bits.offset ((x / 8) as isize)) as c_uchar >> (x % 8)) & 1 != 0   // C 1153
			    { new_foreground } else { new_background }) as c_ulong);
	  x += 1;
	}

      bits = bits.offset (bytes_per_line as isize);                  // C 1156
      y += 1;
    }

  (*rec).stipple_foreground = new_foreground;                        // C 1159
  (*rec).stipple_background = new_background;                        // C 1160
}

/* C 1163-1198.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_stipple_background (s: *mut glyph_string, face: *mut face,
					 x: c_int, y: c_int, width: c_int, height: c_int,
					 explicit_colors_p: bool,
					 explicit_background: uint32_t,
					 explicit_foreground: uint32_t)
{
  let mut rec: *mut haiku_bitmap_record;                             // C 1170
  /* C 1171 declares the two colours with no initialiser and C 1183 hands the callee their
     addresses, so the port zero-initialises them (PORTING.md 1.5; the `cursor_color` reading of
     this same file's C 1113).  */
  let mut foreground: c_ulong = 0;                                  // C 1171
  let mut background: c_ulong = 0;
  let _ = (&mut foreground, &mut background);
  let mut view: *mut c_void;

  view = FRAME_HAIKU_DRAWABLE! ((*s).f);                             // C 1174
  rec = haiku_get_bitmap_rec ((*s).f, (*(GS_FACE! (s))).stipple);   // C 1175

  if explicit_colors_p                                             // C 1177
    {
      background = explicit_background as c_ulong;                  // C 1179
      foreground = explicit_foreground as c_ulong;                  // C 1180
    }
  else if (*s).hl == DRAW_CURSOR                                    // C 1182
    { haiku_merge_cursor_foreground (s, &raw mut foreground, &raw mut background); }   // C 1183
  else
    {
      foreground = (*(GS_FACE! (s))).foreground;                    // C 1186
      background = (*(GS_FACE! (s))).background;                    // C 1187
    }

  haiku_update_bitmap_rec (rec, foreground as uint32_t, background as uint32_t);   // C 1190

  BView_StartClip (view);                                          // C 1192
  haiku_clip_to_string (s);                                        // C 1193
  BView_ClipToRect (view, x, y, width, height);                    // C 1194
  BView_DrawBitmapTiled (view, (*rec).img, 0, 0, -1, -1,           // C 1195-1196
			 0, 0, x + width, y + height);
  BView_EndClip (view);                                            // C 1197
}

/* C 1199-1399: `haiku_draw_background_rect` (C 1200-1209; non-`static`, so its ABI is C's - the
   `get_keysym_name` reading of C 67), `haiku_maybe_draw_background` (C 1211-1231),
   `haiku_mouse_face_colors` (C 1233-1258), `haiku_draw_glyph_string_foreground` (C 1260-1307)
   and `haiku_draw_glyphless_glyph_string_foreground` (C 1309-1399, whose C 1330 `#ifdef
   GCC_LINT` pair carries both arms).  The next first definition is C 1401's
   `haiku_draw_stretch_glyph_string`.  The four inline accessors C 1242-1251 wants
   (`FRAME_FACE_CACHE` C 1379, `FACE_FROM_ID` C 1646, `FACE_FROM_ID_OR_NULL` C 1656,
   `FACE_FOR_CHAR` dispextern.h 2030-2038) are per-unit copies, because C pastes each into every
   unit that includes the header and `include/frame.rs` 1534-1552 keeps its own copy behind
   `dispextern_face_records`, which this unit's arm set does not carry - the `pgtkterm.rs`
   4155-4197 / `xdisp.rs` 1095-1115 shapes.  */

/* dispextern.h 1991-2009's `struct face_cache` as far as those three accessors reach (the
   `term.rs` 307-320 copy): the mirror's record is behind `dispextern_face_records` and its own
   comment carries the measurement this copy repeats - `sizeof` 40, `faces_by_id`@16, `used`@32,
   and C's one `bool_bf menu_face_changed_p : 1` in the int unit at 36.  */
#[cfg (feature = "have_haiku")]
#[repr(C)]
pub struct face_cache
{
  pub buckets: *mut *mut face,                  // C 1994, offset 0
  pub f: *mut c_void,                           // C 1997 (`struct frame *`, untyped - P-72 rule 3)
  pub faces_by_id: *mut *mut face,              // C 2000, offset 16
  pub size: ptrdiff_t,                          // C 2003
  pub used: c_int,                              // C 2004, offset 32
  pub menu_face_changed_p: u8,                  // C 2006 (a `bool_bf : 1`, a byte - P-80)
}
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::size_of::<face_cache> () == 40);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, faces_by_id) == 16);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, used) == 32);

/* C 1379: `#define FRAME_FACE_CACHE(F) ((struct frame *) (F))->face_cache`.  The mirror types the
   member `*mut c_void` (`include/frame.rs` 184), so the cast is the pointer's (P-72 rule 3).  */
#[cfg (feature = "have_haiku")]
#[inline(always)]
pub fn FRAME_FACE_CACHE (f: *mut frame) -> *mut face_cache
{ unsafe { (*f).face_cache as *mut face_cache } }

/// C 1646-1650: `INLINE struct face * FACE_FROM_ID (struct frame *f, int id)`
#[cfg (feature = "have_haiku")]
#[inline(always)]
pub fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
{
  eassert! (0 <= id && id < unsafe { (*FRAME_FACE_CACHE (f)).used });
  unsafe { *(*FRAME_FACE_CACHE (f)).faces_by_id.add (id as usize) }
}

/// C 1656-1661: `INLINE struct face * FACE_FROM_ID_OR_NULL (struct frame *f, int id)`
#[cfg (feature = "have_haiku")]
#[inline(always)]
pub fn FACE_FROM_ID_OR_NULL (f: *mut frame, id: c_int) -> *mut face
{
  let used: c_int = unsafe { (*FRAME_FACE_CACHE (f)).used };
  eassume! (0 <= used);
  if 0 <= id && id < used { unsafe { *(*FRAME_FACE_CACHE (f)).faces_by_id.add (id as usize) } }
  else { core::ptr::null_mut () }
}

/* C 2030-2038's `INLINE int FACE_FOR_CHAR (struct frame *f, struct face *face, int character,
   ptrdiff_t pos, Lisp_Object object)`: both arms carry (P-51).  The window-system arm calls
   `face_for_char`, which `include/fontset.rs` 59-60 declares with the untyped P-72 rule 3
   pointers, so this side casts.  */
#[cfg (all (feature = "have_haiku", feature = "have_window_system"))]
#[inline(always)]
pub fn FACE_FOR_CHAR (f: *mut frame, face: *mut face, character: c_int, pos: ptrdiff_t,
		      object: Lisp_Object) -> c_int
{ unsafe { face_for_char (f as *mut c_void, face as *mut c_void, character, pos, object) } }
#[cfg (all (feature = "have_haiku", not (feature = "have_window_system")))]
#[inline(always)]
pub fn FACE_FOR_CHAR (f: *mut frame, face: *mut face, character: c_int, pos: ptrdiff_t,
		      object: Lisp_Object) -> c_int
{ let _ = (f, character, pos, object); unsafe { (*face).id } }

/* dispextern.h 3810: `void prepare_face_for_display (struct frame *, struct face *);` - no
   mirror carries it, so the unit declares it at the use site (the `pgtkterm.rs` 1520-1526
   declaration of the same prototype).  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn prepare_face_for_display (f: *mut frame, face: *mut face);
}

/* C 1200-1209.  */
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_draw_background_rect (s: *mut glyph_string, face: *mut face,
						     x: c_int, y: c_int,
						     width: c_int, height: c_int)
{
  if ! glyph_string_bits! (stippled_p, (*s))                        // C 1204
    { haiku_draw_plain_background (s, face, x, y, width, height); } // C 1205
  else
    { haiku_draw_stipple_background (s, face, x, y, width, height,  // C 1207-1208
				     false, 0, 0); }
}

/* C 1211-1231.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_maybe_draw_background (s: *mut glyph_string, force_p: c_int)
{
  if glyph_bits! (type, (*(*s).first_glyph)) != IMAGE_GLYPH           // C 1214
      && ! glyph_string_bits! (background_filled_p, (*s))
    {
      let face: *mut face = GS_FACE! (s);                             // C 1216
      let box_line_width: c_int
	= std::cmp::max ((*face).box_horizontal_line_width, 0);       // C 1217
      let box_vline_width: c_int
	= std::cmp::max ((*face).box_vertical_line_width, 0);         // C 1218

      if FONT_HEIGHT! ((*s).font) < (*s).height - 2 * box_vline_width   // C 1220
	  || FONT_TOO_HIGH! ((*s).font)                                 // C 1221
	  || glyph_string_bits! (font_not_found_p, (*s))
	  || glyph_string_bits! (extends_to_end_of_line_p, (*s))        // C 1222
	  || force_p != 0
	{
	  haiku_draw_background_rect (s, GS_FACE! (s), (*s).x,          // C 1224
				      (*s).y + box_line_width,
				      (*s).background_width,
				      (*s).height - 2 * box_line_width);   // C 1226

	  glyph_string_bits! (background_filled_p, (*s), true);         // C 1228
	}
    }
}

/* C 1233-1258.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_mouse_face_colors (s: *mut glyph_string, fg: *mut uint32_t,
				   bg: *mut uint32_t)
{
  let mut face_id: c_int;                                             // C 1237
  let mut face: *mut face;                                            // C 1238

  /* What face has to be used last for the mouse face?  */             // C 1240
  face_id = (*(MOUSE_HL_INFO! ((*s).f))).mouse_face_face_id;          // C 1241
  face = FACE_FROM_ID_OR_NULL ((*s).f, face_id);                      // C 1242
  if face.is_null ()                                                  // C 1243
    { face = FACE_FROM_ID ((*s).f, MOUSE_FACE_ID); }                  // C 1244

  if glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH            // C 1246
    { face_id = FACE_FOR_CHAR ((*s).f, face,                          // C 1247
			       (*(*s).first_glyph).u.ch as c_int, -1, Qnil ()); }
  else
    { face_id = FACE_FOR_CHAR ((*s).f, face, 0, -1, Qnil ()); }       // C 1249

  face = FACE_FROM_ID ((*s).f, face_id);                              // C 1251
  prepare_face_for_display ((*s).f, GS_FACE! (s));                    // C 1252

  if ! fg.is_null ()                                                  // C 1254
    { *fg = (*face).foreground as uint32_t; }                         // C 1255
  if ! bg.is_null ()                                                  // C 1256
    { *bg = (*face).background as uint32_t; }                         // C 1257
}

/* C 1260-1307.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_glyph_string_foreground (s: *mut glyph_string)
{
  let face: *mut face = GS_FACE! (s);                                 // C 1263

  let mut i: c_int;                                                   // C 1265
  let mut x: c_int;
  if face_bits! (box, face) != FACE_NO_BOX                            // C 1266
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))           // C 1267
    { x = (*s).x + std::cmp::max ((*face).box_vertical_line_width, 0); }   // C 1268
  else
    { x = (*s).x; }                                                   // C 1270

  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! ((*s).f);              // C 1272

  if glyph_string_bits! (font_not_found_p, (*s))                       // C 1274
    {
      if (*s).hl == DRAW_CURSOR                                        // C 1276
	{ BView_SetHighColor (view,
			      (*(FRAME_OUTPUT_DATA! ((*s).f) as *mut haiku_output))
				.cursor_fg); }                 // C 1277
      else
	{ BView_SetHighColor (view, (*face).foreground as uint32_t); }   // C 1279
      i = 0;                                                           // C 1280
      while i < (*s).nchars
	{
	  let g: *mut glyph = (*s).first_glyph.offset (i as isize);      // C 1282

	  BView_SetPenSize (view, 1);                                   // C 1284
	  BView_StrokeRectangle (view, x, (*s).y,                        // C 1285
				 (*g).pixel_width as c_int, (*s).height);   // C 1286
	  x += (*g).pixel_width as c_int;                               // C 1287
	  i += 1;                                                       // C 1280's step
	}
    }
  else
    {
      let ft: *mut font = (*s).font;                                    // C 1292
      let mut off: c_int = (*ft).baseline_offset;                       // C 1293
      let mut y: c_int;                                                 // C 1294

      if (*ft).vertical_centering                                        // C 1296
	{ off = VCENTER_BASELINE_OFFSET! (ft, (*s).f) - off; }          // C 1297
      y = (*s).ybase - off;                                              // C 1298
      if glyph_string_bits! (for_overlaps, (*s)) != 0                    // C 1299
	  || (glyph_string_bits! (background_filled_p, (*s))
	      && (*s).hl != DRAW_CURSOR)
	{ ((*((*ft).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, (*s).nchars, x, y, false); }                          // C 1300
      else
	{ ((*((*ft).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, (*s).nchars, x, y, true); }                           // C 1302

      if face_bits! (overstrike, face)                                   // C 1304
	{ ((*((*ft).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, (*s).nchars, x + 1, y, false); }                      // C 1305
    }
}

/* C 1330-1334's `#ifdef GCC_LINT` / `#else` pair: both arms carry (P-51), as the `w32term.rs`
   2435 `char2b` and this file's own `face_ws` pair do.  The member the constant sizes is
   `bool_bf for_no_font : 1; unsigned ch : 25;`'s sibling in C 1335's `char buf[7 + …]`.  */

/* C 597-600's `glyph.u.glyphless` run (`ENUM_BF (glyphless_display_method) method : 2`,
   `unsigned len : 4`, `bool_bf for_no_font : 1`, `unsigned ch : 25`) is what C 1337-1383 names
   and this file's only reader of it, so its accessor stands here (the `haikufont.rs` 1447-1458
   shape, whose constants are `include/dispextern.rs`'s).  */
#[cfg (feature = "have_haiku")]
macro_rules! glyphless_bits {
  (method, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_METHOD_SHIFT) & 0x3) as c_int };
  (for_no_font, $g:expr) => { ((*$g).u.glyphless.bits & GLYPH_U_GLYPHLESS_FOR_NO_FONT) != 0 };
  (len, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_LEN_SHIFT) & 0xf) as c_int };
  (ch, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_CH_SHIFT) & 0x1ffffff) as c_int };
}

/* C 1309-1399.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_glyphless_glyph_string_foreground (s: *mut glyph_string)
{
  let mut glyph: *mut glyph = (*s).first_glyph;                       // C 1312
  static mut char2b: [c_uint; 8] = [0; 8];                            // C 1313
  let mut x: c_int;                                                   // C 1314
  let mut i: c_int;
  let mut j: c_int;
  let face: *mut face = GS_FACE! (s);                                 // C 1315
  /* C 1316 declares `color` with no initialiser and C 1386 passes its address, so the port
     zero-initialises it (PORTING.md 1.5; this file's C 1113 reading).  */
  let mut color: c_ulong = 0;
  let _ = &mut color;

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */                          // C 1318-1319
  if ! face.is_null () && face_bits! (box, face) != FACE_NO_BOX        // C 1320
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))            // C 1321
    { x = (*s).x + std::cmp::max ((*face).box_vertical_line_width, 0); }   // C 1322
  else
    { x = (*s).x; }                                                    // C 1324

  (*s).char2b = &raw mut char2b as *mut c_uint;                        // C 1326

  i = 0;                                                               // C 1328
  while i < (*s).nchars
    {
      /* C's `#ifdef GCC_LINT` pair (C 1330-1334).  */
      #[cfg (feature = "gcc_lint")]
      const PACIFY_GCC_BUG_81401: c_int = 1;
      #[cfg (not (feature = "gcc_lint"))]
      const PACIFY_GCC_BUG_81401: c_int = 0;

      let mut buf: [c_char; 7 + PACIFY_GCC_BUG_81401 as usize]
	= [0; 7 + PACIFY_GCC_BUG_81401 as usize];                      // C 1335
      let mut str: *mut c_char = std::ptr::null_mut ();                // C 1336
      let len: c_int = glyphless_bits! (len, glyph);                   // C 1337

      if glyphless_bits! (method, glyph)
	  == dispextern_unit_names::GLYPHLESS_DISPLAY_ACRONYM      // C 1339
	{
	  if len > 0                                                    // C 1341
	      && CHAR_TABLE_P (Vglyphless_char_display)                 // C 1342
	      && (CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (Vglyphless_char_display))   // C 1343
		  >= 1)
	    {
	      let mut acronym: Lisp_Object
		= if ! glyphless_bits! (for_no_font, glyph)             // C 1347
		  { CHAR_TABLE_REF (Vglyphless_char_display,            // C 1348
				    glyphless_bits! (ch, glyph) as isize) }   // C 1349
		  else { FAM_ELEM! ((*XCHAR_TABLE (Vglyphless_char_display)).extras, 0) };   // C 1350
	      if CONSP (acronym)                                        // C 1351
		{ acronym = XCAR (acronym); }                           // C 1352
	      if STRINGP (acronym)                                      // C 1353
		{ str = SSDATA (acronym); }                             // C 1354
	    }
	}
      else if glyphless_bits! (method, glyph)
	       == dispextern_unit_names::GLYPHLESS_DISPLAY_HEX_CODE   // C 1357
	{
	  let ch: c_uint = glyphless_bits! (ch, glyph) as c_uint;       // C 1359
	  eassume! (ch as c_int <= MAX_CHAR);                           // C 1360
	  sprintf (buf.as_mut_ptr (), c"%0*X".as_ptr (),                // C 1361
		   if ch < 0x10000 { 4 } else { 6 }, ch);
	  str = buf.as_mut_ptr ();                                      // C 1362
	}

      if ! str.is_null ()                                              // C 1365
	{
	  let upper_len: c_int = (len + 1) / 2;                         // C 1367

	  /* It is assured that all LEN characters in STR is ASCII.  */  // C 1369
	  j = 0;                                                        // C 1370
	  while j < len
	    {
	      char2b[j as usize]
		= ((*((*(*s).font).driver as *const font_driver)).encode_char.unwrap ())
		    ((*s).font, *str.offset (j as isize) as c_int) & 0xFFFF;   // C 1371
	      j += 1;                                                   // C 1370's step
	    }

	  ((*((*(*s).font).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, upper_len,                                           // C 1373
	     x + (*glyph).slice.glyphless.upper_xoff as c_int,          // C 1374
	     (*s).ybase + (*glyph).slice.glyphless.upper_yoff as c_int,  // C 1375
	     false);                                                    // C 1376
	  ((*((*(*s).font).driver as *const font_driver)).draw.unwrap ())
	    (s, upper_len, len,                                         // C 1377
	     x + (*glyph).slice.glyphless.lower_xoff as c_int,          // C 1378
	     (*s).ybase + (*glyph).slice.glyphless.lower_yoff as c_int,  // C 1379
	     false);                                                    // C 1380
	}

      if glyphless_bits! (method, glyph)
	  != dispextern_unit_names::GLYPHLESS_DISPLAY_THIN_SPACE   // C 1383
	{
	  if (*s).hl == DRAW_CURSOR                                     // C 1385
	    { haiku_merge_cursor_foreground (s,           // C 1386
					     std::ptr::null_mut (), &raw mut color); }
	  else
	    { color = (*GS_FACE! (s)).foreground; }                     // C 1388

	  BView_SetHighColor (FRAME_HAIKU_DRAWABLE! ((*s).f),
			      color as uint32_t);                          // C 1390
	  BView_SetPenSize (FRAME_HAIKU_DRAWABLE! ((*s).f), 1);          // C 1391
	  BView_StrokeRectangle (FRAME_HAIKU_DRAWABLE! ((*s).f),         // C 1392
				 x, (*s).ybase - (*glyph).ascent as c_int,   // C 1393
				 (*glyph).pixel_width as c_int,            // C 1394
				 (*glyph).ascent as c_int
				   + (*glyph).descent as c_int);           // C 1395
	}
      x += (*glyph).pixel_width as c_int;                              // C 1397
      i += 1; glyph = glyph.offset (1);                                // C 1328's step
    }
}

/* C 1400-1511: `haiku_draw_stretch_glyph_string` (C 1401-1497), `haiku_start_clip` (C 1499-1504)
   and `haiku_end_clip` (C 1506-1511).  The next first definition is C 1513's
   `haiku_clip_to_row`.  The three calls this slice makes to the display helpers
   `window_box_left_offset` (dispextern.h 3580), `window_box_left` (3579) and
   `cursor_in_mouse_face_p` (3692) are declared in this file's extern block above, the
   `window_box_right` entry's own rule (no mirror carries them).  */

/* C 1401-1497.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_stretch_glyph_string (s: *mut glyph_string)
{
  let face: *mut face = GS_FACE! (s);                                 // C 1404
  /* C 1405 declares `bkg` with no initialiser and C 1459/1469 pass its address, so the port
     zero-initialises it (PORTING.md 1.5; this file's C 1113 reading).  */
  let mut bkg: uint32_t = 0;
  let _ = &mut bkg;

  if (*s).hl == DRAW_CURSOR && ! x_stretch_cursor_p                   // C 1407
    {
      let mut width: c_int;                                           // C 1409
      let mut background_width: c_int = (*s).background_width;
      let mut x: c_int = (*s).x;                                      // C 1410

      if ! glyph_row_bits! (reversed_p, (*s).row)                     // C 1412
	{
	  let left_x: c_int = window_box_left_offset ((*s).w, TEXT_AREA);   // C 1414

	  if x < left_x                                                  // C 1416
	    {
	      background_width -= left_x - x;                            // C 1418
	      x = left_x;                                                // C 1419
	    }
	}
      else
	{
	  /* In R2L rows, draw the cursor on the right edge of the
	     stretch glyph.  */                                         // C 1424-1425
	  let right_x: c_int = window_box_right ((*s).w, TEXT_AREA);      // C 1426
	  if x + background_width > right_x                               // C 1427
	    { background_width -= x - right_x; }                          // C 1428
	  x += background_width;                                          // C 1429
	}

      width = std::cmp::min (FRAME_COLUMN_WIDTH ((*s).f), background_width);   // C 1432
      if glyph_row_bits! (reversed_p, (*s).row)                       // C 1433
	{ x -= width; }                                                // C 1434

      let view: *mut c_void = FRAME_HAIKU_DRAWABLE! ((*s).f);          // C 1436
      /* C 1437 declares `cursor_color` with no initialiser and C 1439 passes its address (the
	 same reading as `bkg` above).  */
      let mut cursor_color: c_ulong = 0;
      let _ = &mut cursor_color;

      haiku_merge_cursor_foreground (s, std::ptr::null_mut (),
				     &raw mut cursor_color);   // C 1439
      BView_SetHighColor (view, cursor_color as uint32_t);            // C 1440
      BView_FillRectangle (view, x, (*s).y, width, (*s).height);       // C 1441

      if width < background_width                                     // C 1443
	{
	  if ! glyph_row_bits! (reversed_p, (*s).row)                    // C 1445
	    { x += width; }                                            // C 1446
	  else
	    { x = (*s).x; }                                            // C 1448

	  let y: c_int = (*s).y;                                          // C 1450
	  let w: c_int = background_width - width;                         // C 1451
	  let h: c_int = (*s).height;

	  /* Draw stipples manually because we want the background
	     part of a stretch glyph to have a stipple even if the
	     cursor is visible on top.  */                             // C 1453-1455
	  if (*face).stipple == 0                                         // C 1456
	    {
	      if glyph_row_bits! (mouse_face_p, (*s).row)                 // C 1458
		  && cursor_in_mouse_face_p ((*s).w)
		{ haiku_mouse_face_colors (s, std::ptr::null_mut (), &raw mut bkg); }   // C 1459
	      else
		{ bkg = (*face).background as uint32_t; }                  // C 1461

	      BView_SetHighColor (view, bkg);                             // C 1463
	      BView_FillRectangle (view, x, y, w, h);                      // C 1464
	    }
	  else
	    {
	      if glyph_row_bits! (mouse_face_p, (*s).row)                 // C 1468
		  && cursor_in_mouse_face_p ((*s).w)
		{ haiku_mouse_face_colors (s, std::ptr::null_mut (), &raw mut bkg); }   // C 1469
	      else
		{ bkg = (*face).background as uint32_t; }                  // C 1471

	      haiku_draw_stipple_background (s, GS_FACE! (s), x, y, w, h,   // C 1473
					     true, bkg,                    // C 1474
					     (*face).foreground as uint32_t); }
	}
    }
  else if ! glyph_string_bits! (background_filled_p, (*s))             // C 1478
    {
      let mut background_width: c_int = (*s).background_width;         // C 1480
      let mut x: c_int = (*s).x;
      let text_left_x: c_int = window_box_left ((*s).w, TEXT_AREA);    // C 1481

      /* Don't draw into left fringe or scrollbar area except for
         header line and mode line.  */                              // C 1483-1484
      if (*s).area == TEXT_AREA                                        // C 1485
	  && x < text_left_x && ! glyph_row_bits! (mode_line_p, (*s).row)   // C 1486
	{
	  background_width -= text_left_x - x;                            // C 1488
	  x = text_left_x;                                               // C 1489
	}

      if background_width > 0                                          // C 1492
	{ haiku_draw_background_rect (s, GS_FACE! (s), x, (*s).y,      // C 1493
				      background_width, (*s).height); }   // C 1494
    }
  glyph_string_bits! (background_filled_p, (*s), true);                // C 1496
}

/* C 1499-1504.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_start_clip (s: *mut glyph_string)
{
  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! ((*s).f);              // C 1502
  BView_StartClip (view);                                            // C 1503
}

/* C 1506-1511.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_end_clip (s: *mut glyph_string)
{
  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! ((*s).f);              // C 1509
  BView_EndClip (view);                                              // C 1510
}

/* C 1512-1640: `haiku_clip_to_row` (C 1513-1530), `haiku_update_begin` (C 1532-1538),
   `haiku_update_end` (C 1540-1545) and `haiku_draw_composite_glyph_string_foreground`
   (C 1547-1640).  The next first definition is C 1642's `haiku_draw_image_relief`.
   `window_box` (dispextern.h 3574-3575) is declared in this file's extern block above, the
   `window_box_right` entry's rule.  */

/* C 1513-1530.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_clip_to_row (w: *mut window, row: *mut glyph_row,
			     area: glyph_row_area)
{
  let f: *mut frame = WINDOW_XFRAME! (w);                             // C 1517
  /* C 1518 declares the three outputs of C 1521's `window_box` with no initialiser and C 1521
     hands the callee their addresses, so the port zero-initialises them (PORTING.md 1.5).  */
  let mut window_x: c_int = 0;                                        // C 1518
  let mut window_y: c_int = 0;
  let mut window_width: c_int = 0;
  let mut x: c_int;                                                   // C 1519
  let mut y: c_int;
  let mut width: c_int;
  let mut height: c_int;

  window_box (w, area, &raw mut window_x, &raw mut window_y,           // C 1521
	      &raw mut window_width, std::ptr::null_mut ());

  x = window_x;                                                       // C 1523
  y = WINDOW_TO_FRAME_PIXEL_Y! (w, std::cmp::max (0, (*row).y));       // C 1524
  y = std::cmp::max (y, window_y);                                    // C 1525
  width = window_width;                                               // C 1526
  height = (*row).visible_height;                                     // C 1527

  BView_ClipToRect (FRAME_HAIKU_DRAWABLE! (f), x, y, width, height);   // C 1529
}

/* C 1532-1538.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_update_begin (f: *mut frame)
{
  /* Mark the frame as incomplete so it is not flushed upon handling
     input.  */                                                      // C 1535-1536
  FRAME_COMPLETE_P! (f, false);                                       // C 1537
}

/* C 1540-1545.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_update_end (f: *mut frame)
{
  set_mouse_face_defer (MOUSE_HL_INFO! (f), false);                    // C 1543
  BWindow_Flush (FRAME_HAIKU_WINDOW! (f));                             // C 1544
}

/* C 1547-1640.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_composite_glyph_string_foreground (s: *mut glyph_string)
{
  let mut i: c_int;                                                   // C 1550
  let mut j: c_int;
  let mut x: c_int;
  let font: *mut font = (*s).font;                                    // C 1551
  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! ((*s).f);              // C 1552
  let face: *mut face = GS_FACE! (s);                                 // C 1553

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */                         // C 1555-1556
  if ! face.is_null () && face_bits! (box, face) != FACE_NO_BOX       // C 1557
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))           // C 1558
    { x = (*s).x + std::cmp::max ((*face).box_vertical_line_width, 0); }   // C 1559
  else
    { x = (*s).x; }                                                   // C 1561

  /* S is a glyph string for a composition.  S->cmp_from is the index
     of the first character drawn for glyphs of this composition.
     S->cmp_from == 0 means we are drawing the very first character of
     this composition.  */                                           // C 1563-1566

  /* Draw a rectangle for the composition if the font for the very
     first character of the composition could not be loaded.  */     // C 1568-1569
  if glyph_string_bits! (font_not_found_p, (*s)) && (*s).cmp_from == 0   // C 1570
    {
      if (*s).hl == DRAW_CURSOR                                       // C 1572
	{ BView_SetHighColor (view, (*(FRAME_OUTPUT_DATA! ((*s).f) as *mut haiku_output))
				     .cursor_fg); }                   // C 1573
      else
	{ BView_SetHighColor (view, (*GS_FACE! (s)).foreground as uint32_t); }   // C 1575

      BView_SetPenSize (view, 1);                                     // C 1577
      BView_StrokeRectangle (view, (*s).x, (*s).y,                     // C 1578
			     (*s).width, (*s).height);                 // C 1579
    }
  else if ! glyph_u_cmp_bits! (automatic, (*s).first_glyph)            // C 1581
    {
      let y: c_int = (*s).ybase;                                       // C 1583

      i = 0;                                                           // C 1585
      j = (*s).cmp_from;
      while i < (*s).nchars
	{
	  /* TAB in a composition means display glyphs with padding
	     space on the left or right.  */                            // C 1586-1587
	  if COMPOSITION_GLYPH ((*s).cmp as *mut composition, j as ptrdiff_t)   // C 1588
	      != b'\t' as isize
	    {
	      let cmp: *mut composition = (*s).cmp as *mut composition;
	      let xx: c_int
		= x + *(*cmp).offsets.offset ((j * 2) as isize) as c_int;   // C 1590
	      let yy: c_int
		= y - *(*cmp).offsets.offset ((j * 2 + 1) as isize) as c_int;   // C 1591

	      ((*((*font).driver as *const font_driver)).draw.unwrap ())
		(s, j, j + 1, xx, yy, false);                            // C 1593
	      if face_bits! (overstrike, face)                            // C 1594
		{ ((*((*font).driver as *const font_driver)).draw.unwrap ())
		    (s, j, j + 1, xx + 1, yy, false); }                  // C 1595
	    }
	  i += 1; j += 1;                                                // C 1585's step
	}
    }
  else
    {
      let gstring: Lisp_Object = composition_gstring_from_id ((*s).cmp_id);   // C 1600
      let mut glyph: Lisp_Object;                                      // C 1601
      let y: c_int = (*s).ybase;                                       // C 1602
      let mut width: c_int = 0;                                        // C 1603

      i = (*s).cmp_from;                                               // C 1605
      j = (*s).cmp_from;
      while i < (*s).cmp_to
	{
	  glyph = LGSTRING_GLYPH (gstring, i as ptrdiff_t);             // C 1607
	  if NILP (LGLYPH_ADJUSTMENT (glyph))                           // C 1608
	    { width += LGLYPH_WIDTH (glyph) as c_int; }                 // C 1609
	  else
	    {
	      let mut xoff: c_int;                                       // C 1612
	      let mut yoff: c_int;
	      let mut wadjust: c_int;

	      if j < i                                                   // C 1614
		{
		  ((*((*font).driver as *const font_driver)).draw.unwrap ())
		    (s, j, i, x, y, false);                              // C 1616
		  if face_bits! (overstrike, face)                          // C 1617
		    { ((*((*font).driver as *const font_driver)).draw.unwrap ())
			(s, j, i, x + 1, y, false); }                    // C 1618
		  x += width;                                              // C 1619
		}
	      xoff = LGLYPH_XOFF (glyph) as c_int;                         // C 1621
	      yoff = LGLYPH_YOFF (glyph) as c_int;                         // C 1622
	      wadjust = LGLYPH_WADJUST (glyph) as c_int;                   // C 1623
	      ((*((*font).driver as *const font_driver)).draw.unwrap ())
		(s, i, i + 1, x + xoff, y + yoff, false);                // C 1624
	      if face_bits! (overstrike, face)                             // C 1625
		{ ((*((*font).driver as *const font_driver)).draw.unwrap ())
		    (s, i, i + 1, x + xoff + 1, y + yoff, false); }      // C 1626-1627
	      x += wadjust;                                              // C 1628
	      j = i + 1;                                                 // C 1629
	      width = 0;                                                 // C 1630
	    }
	  i += 1;                                                        // C 1605's step
	}
      if j < i                                                          // C 1633
	{
	  ((*((*font).driver as *const font_driver)).draw.unwrap ())
	    (s, j, i, x, y, false);                                     // C 1635
	  if face_bits! (overstrike, face)                                // C 1636
	    { ((*((*font).driver as *const font_driver)).draw.unwrap ())
		(s, j, i, x + 1, y, false); }                           // C 1637
	}
    }
}

/* C 1641-1732: `haiku_draw_image_relief` (C 1642-1732).  The next first definition is C 1734's
   `haiku_translate_transform`.  `image_ascent` (dispextern.h 3774) is declared at the use site
   below: `src/image.rs` defines it with `#[unsafe(no_mangle)]` and no mirror carries the
   prototype (the `pgtkterm.rs` 3041-3048 declaration of the same one).  */

/* C: dispextern.h 3774: `int image_ascent (struct image *, struct face *, struct glyph_slice
   *);` - the C 1650 call, C's `&s->slice` being the port's `&raw mut`.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn image_ascent (img: *mut image, face: *mut face, slice: *mut glyph_slice) -> c_int;
}

/* C 1642-1732.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_image_relief (s: *mut glyph_string)
{
  let mut x1: c_int;                                                  // C 1645
  let mut y1: c_int;
  let mut thick: c_int;
  let mut raised_p: bool;                                             // C 1646
  let mut top_p: bool;
  let mut bot_p: bool;
  let mut left_p: bool;
  let mut right_p: bool;
  let mut extra_x: c_int;                                             // C 1647
  let mut extra_y: c_int;
  let mut r: haiku_rect = unsafe { core::mem::zeroed () };             // C 1648
  let mut x: c_int = (*s).x;                                          // C 1649
  let mut y: c_int = (*s).ybase                                       // C 1650
    - image_ascent ((*s).img, GS_FACE! (s), &raw mut (*s).slice);

  let face: *mut face = GS_FACE! (s);                                 // C 1652

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */                                         // C 1654-1655
  if face_bits! (box, face) != FACE_NO_BOX                            // C 1656
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))            // C 1657
      && glyph_slice_bits! (x, (*s).slice) == 0                        // C 1658
    { x += std::cmp::max ((*face).box_vertical_line_width, 0); }       // C 1659

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */                                             // C 1661-1662
  if glyph_slice_bits! (x, (*s).slice) == 0                            // C 1663
    { x += (*(*s).img).hmargin; }                                      // C 1664
  if glyph_slice_bits! (y, (*s).slice) == 0                            // C 1665
    { y += (*(*s).img).vmargin; }                                      // C 1666

  if (*s).hl == DRAW_IMAGE_SUNKEN                                      // C 1668
      || (*s).hl == DRAW_IMAGE_RAISED                                  // C 1669
    {
      if (*face).id == TAB_BAR_FACE_ID                                 // C 1671
	{ thick = (if tab_bar_button_relief < 0                        // C 1672
		     { DEFAULT_TAB_BAR_BUTTON_RELIEF }                 // C 1673
		     else { std::cmp::min (tab_bar_button_relief, 1000000) as c_int }); }   // C 1674
      else
	{ thick = (if tool_bar_button_relief < 0                       // C 1676
		     { DEFAULT_TOOL_BAR_BUTTON_RELIEF }                // C 1677
		     else { std::cmp::min (tool_bar_button_relief, 1000000) as c_int }); }   // C 1678
      raised_p = (*s).hl == DRAW_IMAGE_RAISED;                          // C 1679
    }
  else
    {
      thick = eabs! ((*(*s).img).relief);                              // C 1683
      raised_p = (*(*s).img).relief > 0;                               // C 1684
    }

  x1 = x + glyph_slice_bits! (width, (*s).slice) as c_int - 1;          // C 1687
  y1 = y + glyph_slice_bits! (height, (*s).slice) as c_int - 1;         // C 1688

  extra_x = 0;                                                         // C 1690
  extra_y = 0;

  if (*face).id == TAB_BAR_FACE_ID                                     // C 1692
    {
      if CONSP (Vtab_bar_button_margin)                                // C 1694
	  && FIXNUMP (XCAR (Vtab_bar_button_margin))                   // C 1695
	  && FIXNUMP (XCDR (Vtab_bar_button_margin))                   // C 1696
	{
	  extra_x = XFIXNUM (XCAR (Vtab_bar_button_margin)) as c_int - thick;   // C 1698
	  extra_y = XFIXNUM (XCDR (Vtab_bar_button_margin)) as c_int - thick;   // C 1699
	}
      else if FIXNUMP (Vtab_bar_button_margin)                         // C 1701
	{
	  extra_x = XFIXNUM (Vtab_bar_button_margin) as c_int - thick;   // C 1702
	  extra_y = XFIXNUM (Vtab_bar_button_margin) as c_int - thick;
	}
    }

  if (*face).id == TOOL_BAR_FACE_ID                                    // C 1705
    {
      if CONSP (Vtool_bar_button_margin)                               // C 1707
	  && FIXNUMP (XCAR (Vtool_bar_button_margin))                  // C 1708
	  && FIXNUMP (XCDR (Vtool_bar_button_margin))                  // C 1709
	{
	  extra_x = XFIXNUM (XCAR (Vtool_bar_button_margin)) as c_int;   // C 1711
	  extra_y = XFIXNUM (XCDR (Vtool_bar_button_margin)) as c_int;   // C 1712
	}
      else if FIXNUMP (Vtool_bar_button_margin)                        // C 1714
	{
	  extra_x = XFIXNUM (Vtool_bar_button_margin) as c_int;          // C 1715
	  extra_y = XFIXNUM (Vtool_bar_button_margin) as c_int;
	}
    }

  top_p = false;                                                       // C 1718
  bot_p = false;
  left_p = false;
  right_p = false;

  if glyph_slice_bits! (x, (*s).slice) == 0                            // C 1720
    { x -= thick + extra_x; left_p = true; }                           // C 1721
  if glyph_slice_bits! (y, (*s).slice) == 0                            // C 1722
    { y -= thick + extra_y; top_p = true; }                            // C 1723
  if (glyph_slice_bits! (x, (*s).slice) as c_int                       // C 1724
      + glyph_slice_bits! (width, (*s).slice) as c_int) == (*(*s).img).width
    { x1 += thick + extra_x; right_p = true; }                         // C 1725
  if (glyph_slice_bits! (y, (*s).slice) as c_int                       // C 1726
      + glyph_slice_bits! (height, (*s).slice) as c_int) == (*(*s).img).height
    { y1 += thick + extra_y; bot_p = true; }                           // C 1727

  get_glyph_string_clip_rect (s, &raw mut r as *mut c_void);          // C 1729
  haiku_draw_relief_rect (s, x, y, x1, y1, thick, thick, raised_p,      // C 1730
			  top_p, bot_p, left_p, right_p, &raw mut r);   // C 1731
}

/* C 1733-1902: `haiku_translate_transform` (C 1734-1740) and `haiku_draw_image_glyph_string`
   (C 1742-1902).  The next first definition is C 1904's `haiku_draw_glyph_string`.  Everything
   this slice calls is in reach of the unit's own include graph: `Emacs_Rectangle`
   (`include/dispextern.rs` 68), `CONVERT_TO_EMACS_RECT!` (`include/haikugui.rs` 72),
   `gui_intersect_rectangles` (dispextern.rs 4048), the `BView_*`/`be_*` surface
   (`include/haiku_support.rs` 604-634) and `memcpy` (`lib/include/string.rs`).  */

/* C haikuterm.c 1735: `double (*transform)[3]` - the row type `haiku_translate_transform` and
   `haiku_draw_image_glyph_string` (C 1825/1838) pass.  The alias exists for the **checker's** sake
   and not the type's: `extract-abi.py`'s body reader skips any `fn` whose parameter list holds a
   `;` before its `{` (it reads that as a declaration), and `[c_double; 3]` is exactly that - the
   row stood as `missing function haiku_translate_transform` from slice 6 to slice 27, measured,
   while the function has been ported the whole time (the `src/image.rs` 4113 `matrix3x3_mult`
   spelling, which is a *live* file and so never hit the whole-file fallback that reads statics).  */
#[cfg (feature = "have_haiku")]
pub type transform_row = [c_double; 3];

/* C 1734-1740.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_translate_transform (transform: *mut transform_row, dx: c_double, dy: c_double)
{
  (*transform.offset (0))[2] += dx;                                   // C 1738 (C's own `0` token:
								      //  the port spells the
								      //  row index rather
								      //  than folding it into
								      //  the dereference)
  (*transform.offset (1))[2] += dy;                                   // C 1739
}

/* C 1742-1902.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_image_glyph_string (s: *mut glyph_string)
{
  let face: *mut face = GS_FACE! (s);                                 // C 1745
  let mut view: *mut c_void;                                          // C 1746
  let mut bitmap: *mut c_void;
  let mut mask: *mut c_void;
  let box_line_hwidth: c_int
    = std::cmp::max ((*face).box_vertical_line_width, 0);             // C 1747
  let box_line_vwidth: c_int
    = std::cmp::max ((*face).box_horizontal_line_width, 0);           // C 1748
  let mut x: c_int;                                                   // C 1749
  let mut y: c_int;
  let mut height: c_int;
  let mut width: c_int;
  let mut relief: c_int;
  let mut nr: haiku_rect = unsafe { core::mem::zeroed () };             // C 1750
  let mut cr: Emacs_Rectangle = unsafe { core::mem::zeroed () };        // C 1751
  let mut ir: Emacs_Rectangle = unsafe { core::mem::zeroed () };
  let mut r: Emacs_Rectangle = unsafe { core::mem::zeroed () };
  /* C 1752 declares `background` with no initialiser and C 1780 hands the callee its address,
     so the port zero-initialises it (PORTING.md 1.5; this file's C 1113 reading).  */
  let mut background: c_ulong = 0;
  let _ = &mut background;
  /* C 1753's `image_transform` is filled by C 1814's `memcpy`, which the port cannot follow, so
     it is zeroed and named too.  */
  let mut image_transform: [[c_double; 3]; 3] = [[0.0; 3]; 3];
  let _ = &mut image_transform;

  height = (*s).height;                                               // C 1755
  if glyph_slice_bits! (y, (*s).slice) == 0                           // C 1756
    { height -= box_line_vwidth; }                                    // C 1757
  if glyph_slice_bits! (y, (*s).slice) as c_int                       // C 1758
      + glyph_slice_bits! (height, (*s).slice) as c_int >= (*(*s).img).height
    { height -= box_line_vwidth; }                                    // C 1759

  width = (*s).background_width;                                      // C 1761
  x = (*s).x;                                                         // C 1762
  if glyph_bits! (left_box_line_p, (*(*s).first_glyph))                // C 1763
      && glyph_slice_bits! (x, (*s).slice) == 0                       // C 1764
    {
      x += box_line_hwidth;                                           // C 1766
      width -= box_line_hwidth;                                       // C 1767
    }

  y = (*s).y;                                                         // C 1770
  if glyph_slice_bits! (y, (*s).slice) == 0                           // C 1771
    { y += box_line_vwidth; }                                         // C 1772

  view = FRAME_HAIKU_DRAWABLE! ((*s).f);                              // C 1774
  bitmap = (*(*s).img).pixmap;                                        // C 1775

  glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);         // C 1777

  if (*s).hl == DRAW_CURSOR                                           // C 1779
    { haiku_merge_cursor_foreground (s, std::ptr::null_mut (),
				     &raw mut background); }  // C 1780
  else
    { background = (*face).background; }                              // C 1782

  haiku_draw_background_rect (s, GS_FACE! (s), x, y,                   // C 1784
			      width, height);                         // C 1785

  if ! bitmap.is_null ()                                              // C 1787
    {
      get_glyph_string_clip_rect (s, &raw mut nr as *mut c_void);      // C 1789
      CONVERT_TO_EMACS_RECT! (cr, nr);                                // C 1790
      x = (*s).x;                                                     // C 1791
      y = (*s).ybase                                                  // C 1792
	- image_ascent ((*s).img, face, &raw mut (*s).slice);

      if glyph_slice_bits! (x, (*s).slice) == 0                       // C 1794
	{ x += (*(*s).img).hmargin; }                                 // C 1795
      if glyph_slice_bits! (y, (*s).slice) == 0                       // C 1796
	{ y += (*(*s).img).vmargin; }                                 // C 1797

      if face_bits! (box, face) != FACE_NO_BOX                        // C 1799
	  && glyph_bits! (left_box_line_p, (*(*s).first_glyph))         // C 1800
	  && glyph_slice_bits! (x, (*s).slice) == 0                     // C 1801
	{ x += std::cmp::max ((*face).box_vertical_line_width, 0); }    // C 1802

      ir.x = x;                                                       // C 1804
      ir.y = y;                                                       // C 1805
      ir.width = glyph_slice_bits! (width, (*s).slice) as c_uint;      // C 1806
      ir.height = glyph_slice_bits! (height, (*s).slice) as c_uint;    // C 1807
      r = ir;                                                         // C 1808

      mask = (*(*s).img).mask;                                        // C 1810

      if gui_intersect_rectangles (&raw const cr, &raw const ir, &raw mut r)   // C 1812
	{
	  memcpy (&raw mut image_transform as *mut c_void,             // C 1814
		  &raw const (*(*s).img).transform as *const c_void,
		  core::mem::size_of::<[[c_double; 3]; 3]> ());       // C 1815

	  if glyph_slice_bits! (x, (*s).slice) as c_int != x            // C 1817
	      || glyph_slice_bits! (y, (*s).slice) as c_int != y        // C 1818
	      || glyph_slice_bits! (width, (*s).slice) as c_int != (*(*s).img).width    // C 1819
	      || glyph_slice_bits! (height, (*s).slice) as c_int != (*(*s).img).height  // C 1820
	    {
	      BView_StartClip (view);                                  // C 1821
	      BView_ClipToRect (view, r.x, r.y, r.width as c_int, r.height as c_int);   // C 1822
	    }

	  haiku_translate_transform (image_transform.as_mut_ptr (),     // C 1825
				     (x - glyph_slice_bits! (x, (*s).slice) as c_int) as c_double,   // C 1826
				     (y - glyph_slice_bits! (y, (*s).slice) as c_int) as c_double);  // C 1827

	  be_apply_affine_transform (view,                              // C 1829
				     image_transform[0][0],            // C 1830
				     image_transform[0][1],            // C 1831
				     image_transform[0][2],            // C 1832
				     image_transform[1][0],            // C 1833
				     image_transform[1][1],            // C 1834
				     image_transform[1][2]);           // C 1835

	  if ! glyph_string_bits! (stippled_p, (*s)) || mask.is_null ()  // C 1837
	    {
	      BView_DrawBitmap (view, bitmap, 0, 0,                      // C 1839
				(*(*s).img).original_width,               // C 1840
				(*(*s).img).original_height,              // C 1841
				0, 0,                                     // C 1842
				(*(*s).img).original_width,               // C 1843
				(*(*s).img).original_height,              // C 1844
				(*(*s).img).use_bilinear_filtering);      // C 1845

	      if ! mask.is_null ()                                      // C 1847
		{ be_draw_image_mask (mask, view, 0, 0,                  // C 1848
				      (*(*s).img).original_width,        // C 1849
				      (*(*s).img).original_height,       // C 1850
				      0, 0,                              // C 1851
				      (*(*s).img).original_width,        // C 1852
				      (*(*s).img).original_height,       // C 1853
				      background as uint32_t); }         // C 1854
	    }
	  else
	    /* In order to make sure the stipple background remains
	       visible, use the mask for the alpha channel of BITMAP
	       and composite it onto the view instead.  */             // C 1857-1859
	    { be_draw_bitmap_with_mask (view, bitmap, mask, 0, 0,        // C 1860
					(*(*s).img).original_width,     // C 1861
					(*(*s).img).original_height,    // C 1862
					0, 0,                           // C 1863
					(*(*s).img).original_width,     // C 1864
					(*(*s).img).original_height,    // C 1865
					(*(*s).img).use_bilinear_filtering); }   // C 1866

	  if glyph_slice_bits! (x, (*s).slice) as c_int != x            // C 1868
	      || glyph_slice_bits! (y, (*s).slice) as c_int != y        // C 1869
	      || glyph_slice_bits! (width, (*s).slice) as c_int != (*(*s).img).width    // C 1870
	      || glyph_slice_bits! (height, (*s).slice) as c_int != (*(*s).img).height  // C 1871
	    { BView_EndClip (view); }                                   // C 1872

	  be_apply_affine_transform (view, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0);   // C 1873
	}

      if (*(*s).img).mask.is_null ()                                   // C 1876
	{
	  /* When the image has a mask, we can expect that at
	     least part of a mouse highlight or a block cursor will
	     be visible.  If the image doesn't have a mask, make
	     a block cursor visible by drawing a rectangle around
	     the image.  I believe it's looking better if we do
	     nothing here for mouse-face.  */                          // C 1878-1883

	  if (*s).hl == DRAW_CURSOR                                     // C 1885
	    {
	      relief = eabs! ((*(*s).img).relief);                       // C 1887

	      BView_SetPenSize (view, 1);                                // C 1889
	      BView_SetHighColor (view, FRAME_CURSOR_COLOR! ((*s).f).pixel as uint32_t);   // C 1890
	      BView_StrokeRectangle (view, x - relief, y - relief,        // C 1891
				     glyph_slice_bits! (width, (*s).slice) as c_int + relief * 2,    // C 1892
				     glyph_slice_bits! (height, (*s).slice) as c_int + relief * 2);   // C 1893
	    }
	}
    }

  if (*(*s).img).relief != 0                                          // C 1898
      || (*s).hl == DRAW_IMAGE_RAISED                                 // C 1899
      || (*s).hl == DRAW_IMAGE_SUNKEN                                 // C 1900
    { haiku_draw_image_relief (s); }                                  // C 1901
}

/* C 1903-2074: `haiku_draw_glyph_string` (C 1904-2074).  The next first definition is C 2076's
   `haiku_draw_glyph_string_background`.  Every name this slice calls is one of this file's own
   (C 155-196's `haiku_clip_to_string`, C 198-206's `haiku_clip_to_string_exactly`, the five
   drawing functions of slices 1-7) or the mirrors'; `emacs_abort` is the `default:` arm C 1998
   spells.  */

/* C 1904-2074.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_draw_glyph_string (s: *mut glyph_string)
{
  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! ((*s).f);              // C 1907
  let face: *mut face = GS_FACE! (s);                                 // C 1908

  block_input ();                                                     // C 1910
  BView_draw_lock (view, false, 0, 0, 0, 0);                          // C 1911
  prepare_face_for_display ((*s).f, GS_FACE! (s));                    // C 1912

  glyph_string_bits! (stippled_p, (*s),                                // C 1914
		      (*s).hl != DRAW_CURSOR && (*face).stipple != 0);

  if ! (*s).next.is_null () && (*s).right_overhang != 0                // C 1916
      && glyph_string_bits! (for_overlaps, (*s)) == 0
    {
      let mut width: c_int;                                           // C 1918
      let mut next: *mut glyph_string;                                // C 1919

      width = 0;                                                      // C 1921
      next = (*s).next;
      while ! next.is_null () && width < (*s).right_overhang          // C 1922
	{
	  if glyph_bits! (type, (*(*next).first_glyph)) != IMAGE_GLYPH   // C 1924
	    {
	      prepare_face_for_display ((*s).f, GS_FACE! (next));       // C 1926
	      glyph_string_bits! (stippled_p, (*next),                   // C 1927
				  (*next).hl != DRAW_CURSOR
				  && (*(GS_FACE! (next))).stipple != 0);   // C 1928

	      haiku_start_clip (next);                                  // C 1930
	      haiku_clip_to_string (next);                              // C 1931
	      if glyph_bits! (type, (*(*next).first_glyph)) != STRETCH_GLYPH   // C 1932
		{ haiku_maybe_draw_background (next, 1); }              // C 1933
	      else
		{ haiku_draw_stretch_glyph_string (next); }             // C 1935
	      haiku_end_clip (s);                                       // C 1936
	    }
	  width += (*next).width;                                       // C 1923's step
	  next = (*next).next;
	}
    }

  haiku_start_clip (s);                                              // C 1940

  let mut box_filled_p: c_int = 0;                                    // C 1942

  if glyph_string_bits! (for_overlaps, (*s)) == 0                      // C 1944
      && face_bits! (box, face) != FACE_NO_BOX
      && (glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH       // C 1945
	  || glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH)   // C 1946
    {
      haiku_clip_to_string (s);                                       // C 1948
      haiku_maybe_draw_background (s, 1);                             // C 1949
      box_filled_p = 1;                                               // C 1950
      haiku_draw_string_box (s);                                      // C 1951
    }
  else if (*s).clip_head.is_null ()                                   // C 1953
	   && (*s).clip_tail.is_null ()                               // C 1954
	   && ((! (*s).prev.is_null () && (*(*s).prev).hl != (*s).hl  // C 1955
		&& (*s).left_overhang != 0)
	       || (! (*s).next.is_null () && (*(*s).next).hl != (*s).hl   // C 1956
		   && (*s).right_overhang != 0))
    /* We must clip just this glyph.  left_overhang part has already
       drawn when s->prev was drawn, and right_overhang part will be
       drawn later when s->next is drawn.  */                         // C 1957-1959
    { haiku_clip_to_string_exactly (s, s); }                          // C 1960
  else
    { haiku_clip_to_string (s); }                                     // C 1962

  if glyph_string_bits! (for_overlaps, (*s)) != 0                     // C 1964
    { glyph_string_bits! (background_filled_p, (*s), true); }         // C 1965

  match glyph_bits! (type, (*(*s).first_glyph))                       // C 1967
    {
      COMPOSITE_GLYPH =>
	{
	  if glyph_string_bits! (for_overlaps, (*s)) != 0             // C 1970
	      || ((*s).cmp_from > 0                                   // C 1971
		  && ! glyph_u_cmp_bits! (automatic, (*s).first_glyph))
	    { glyph_string_bits! (background_filled_p, (*s), true); }  // C 1972
	  else
	    { haiku_maybe_draw_background (s, 1); }                   // C 1974
	  haiku_draw_composite_glyph_string_foreground (s);           // C 1975
	}
      CHAR_GLYPH =>
	{
	  if glyph_string_bits! (for_overlaps, (*s)) != 0             // C 1978
	    { glyph_string_bits! (background_filled_p, (*s), true); }  // C 1979
	  else
	    { haiku_maybe_draw_background (s, 0); }                   // C 1981
	  haiku_draw_glyph_string_foreground (s);                     // C 1982
	}
      STRETCH_GLYPH =>
	{ haiku_draw_stretch_glyph_string (s); }                      // C 1985
      IMAGE_GLYPH =>
	{ haiku_draw_image_glyph_string (s); }                        // C 1988
      GLYPHLESS_GLYPH =>
	{
	  if glyph_string_bits! (for_overlaps, (*s)) != 0             // C 1991
	    { glyph_string_bits! (background_filled_p, (*s), true); }  // C 1992
	  else
	    { haiku_maybe_draw_background (s, 1); }                   // C 1994
	  haiku_draw_glyphless_glyph_string_foreground (s);           // C 1995
	}
      _ =>
	{ emacs_abort (); }                                           // C 1998
    }

  if glyph_string_bits! (for_overlaps, (*s)) == 0                      // C 2001
    {
      if box_filled_p == 0 && face_bits! (box, face) != FACE_NO_BOX    // C 2003
	{ haiku_draw_string_box (s); }                                // C 2004
      else
	{ haiku_draw_text_decoration (s, GS_FACE! (s), (*s).width, (*s).x); }   // C 2006

      if ! (*s).prev.is_null ()                                       // C 2008
	{
	  let mut prev: *mut glyph_string;                            // C 2010

	  prev = (*s).prev;                                           // C 2012
	  while ! prev.is_null ()
	    {
	      if (*prev).hl != (*s).hl                                // C 2013
		  && (*prev).x + (*prev).width + (*prev).right_overhang > (*s).x   // C 2014
		{
		  /* As prev was drawn while clipped to its own area, we
		     must draw the right_overhang part using s->hl now.  */   // C 2016-2017
		  let save: draw_glyphs_face = (*prev).hl;             // C 2018

		  (*prev).hl = (*s).hl;                                // C 2020
		  haiku_start_clip (s);                                // C 2021
		  haiku_clip_to_string (s);                            // C 2022
		  haiku_clip_to_string_exactly (s, prev);              // C 2023
		  if glyph_bits! (type, (*(*prev).first_glyph)) == CHAR_GLYPH   // C 2024
		    { haiku_draw_glyph_string_foreground (prev); }     // C 2025
		  else
		    { haiku_draw_composite_glyph_string_foreground (prev); }   // C 2027
		  haiku_end_clip (s);                                 // C 2028
		  (*prev).hl = save;                                  // C 2029
		}
	      prev = (*prev).prev;                                    // C 2012's step
	    }
	}

      if ! (*s).next.is_null ()                                       // C 2033
	{
	  let mut next: *mut glyph_string;                            // C 2035

	  next = (*s).next;                                           // C 2037
	  while ! next.is_null ()
	    {
	      if (*next).hl != (*s).hl                                // C 2038
		  && (*next).x - (*next).left_overhang < (*s).x + (*s).width   // C 2039
		{
		  /* As next will be drawn while clipped to its own area,
		     we must draw the left_overhang part using s->hl now.  */   // C 2041-2042
		  let save: draw_glyphs_face = (*next).hl;             // C 2043

		  (*next).hl = (*s).hl;                                // C 2045
		  haiku_start_clip (s);                                // C 2046
		  haiku_clip_to_string (s);                            // C 2047
		  haiku_clip_to_string_exactly (s, next);              // C 2048
		  if glyph_bits! (type, (*(*next).first_glyph)) == CHAR_GLYPH   // C 2049
		    { haiku_draw_glyph_string_foreground (next); }     // C 2050
		  else
		    { haiku_draw_composite_glyph_string_foreground (next); }   // C 2052
		  haiku_end_clip (s);                                 // C 2053

		  (*next).hl = save;                                  // C 2055
		  (*next).clip_head = (*s).next;                       // C 2056
		}
	      next = (*next).next;                                    // C 2037's step
	    }
	}
    }

  haiku_end_clip (s);                                                 // C 2061
  BView_draw_unlock (view);                                           // C 2062

  /* Set the stipple_p flag indicating whether or not a stipple was
     drawn in s->row.  That is the case either when s is a stretch
     glyph string and s->face->stipple is not NULL, or when
     s->face->stipple exists and s->hl is not DRAW_CURSOR.  */       // C 2064-2067
  if (*(GS_FACE! (s))).stipple != 0                                   // C 2068
      && (glyph_bits! (type, (*(*s).first_glyph)) == STRETCH_GLYPH     // C 2069
	  || (*s).hl != DRAW_CURSOR)                                  // C 2070
    { glyph_row_bits! (stipple_p, (*s).row, true); }                   // C 2071

  unblock_input ();                                                   // C 2073
}

/* C 2075-2219: `haiku_after_update_window_line` (C 2076-2129), `haiku_set_window_size`
   (C 2131-2167) and `haiku_draw_hollow_cursor` (C 2169-2219).  The next first definition is
   C 2221's `haiku_draw_bar_cursor`.  Three of the names this slice calls are declared at the use
   site below (`get_phys_cursor_glyph`, window.h 1186; `get_phys_cursor_geometry`, dispextern.h
   3652-3653; `do_pending_window_change`, dispextern.h 3923) because no mirror carries the
   prototypes - the `pgtkterm.rs` 4021-4028 and `process.rs` 5945 declarations of the same three.
   `f->want_fullscreen` (frame.h 460) has the mirror's unit/shift/mask constants and no accessor,
   so the unit reads the field itself (the `haikufns.rs` 1313-1321 copy of that same member).  */

/// C: frame.h 460: `ENUM_BF (fullscreen_type) want_fullscreen : 4;` - read at C 2144 and
/// **written at C 4096/4098** (`f->want_fullscreen = …;`), so the macro carries C's read and its
/// masked write on `bits2` - the `src/pgtkterm.rs` 6462-6470 pair, which this unit's read arm was
/// already one half of (the writer arm was added when C 4096 of the `haiku_read_socket` round
/// needed it; measured: with the read-only macro the two assignments were
/// `no rules expected \`,\``).
#[cfg (feature = "have_haiku")]
macro_rules! want_fullscreen {
  ($f:expr) => {
    frame_field ($f, FRAME_WANT_FULLSCREEN_UNIT, FRAME_WANT_FULLSCREEN_SHIFT,
		 FRAME_WANT_FULLSCREEN_MASK) as fullscreen_type };
  ($f:expr, $v:expr) => {
    (*$f).bits2 = ((*$f).bits2 & !(FRAME_WANT_FULLSCREEN_MASK << FRAME_WANT_FULLSCREEN_SHIFT))
		  | (((($v) as u32) & FRAME_WANT_FULLSCREEN_MASK) << FRAME_WANT_FULLSCREEN_SHIFT) };
}

/* C: window.h 1186: `struct glyph *get_phys_cursor_glyph (struct window *w);`
   C: dispextern.h 3652-3653: `extern void get_phys_cursor_geometry (struct window *,
   struct glyph_row *, struct glyph *, int *, int *, int *);`
   C: dispextern.h 3923: `void do_pending_window_change (bool);`
   (`XSETFRAME` is *not* declared here: `include/frame.rs` 776-780 carries it as C's own macro,
   `XSETFRAME! (frame, f)`, so the call site uses that form - `androidfns.rs` 1214's spelling.)  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn get_phys_cursor_glyph (w: *mut window) -> *mut glyph;
  pub fn get_phys_cursor_geometry (w: *mut window, row: *mut glyph_row, glyph: *mut glyph,
				   x: *mut c_int, y: *mut c_int, h: *mut c_int);
  pub fn do_pending_window_change (safe: bool);
  /// C: dispextern.h 3582: `int estimate_mode_line_height (struct frame *, enum face_id);` -
  /// reached through `WINDOW_BOX_HEIGHT_NO_MODE_LINE!` (include/dispextern.rs 1308 and the two
  /// siblings), which no mirror declares the callee of, so the unit names it here (P-72 rule 4;
  /// the `lookup_basic_face` entry below is the same class).
  pub fn estimate_mode_line_height (f: *mut frame, face_id: face_id) -> c_int;
  /// C: dispextern.h 3813: `int lookup_basic_face (struct window *, struct frame *, int);` -
  /// `include/dispextern.rs` 4051 records it as owed rather than declared, so the unit names it
  /// here (the `xterm.rs` 8692 use-site read of the same prototype).
  pub fn lookup_basic_face (w: *mut window, f: *mut frame, face_id: c_int) -> c_int;
}

/* C 2076-2129.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_after_update_window_line (w: *mut window, desired_row: *mut glyph_row)
{
  eassert! (! w.is_null ());                                          // C 2080
  let f: *mut frame;                                                  // C 2081
  let mut width: c_int;                                               // C 2082
  let mut height: c_int;

  if ! glyph_row_bits! (mode_line_p, desired_row)                     // C 2084
      && ! window_bits! (pseudo_window_p, w)
    { glyph_row_bits! (redraw_fringe_bitmaps_p, desired_row, true); }  // C 2085

  if windows_or_buffers_changed != 0                                  // C 2087
      && glyph_row_bits! (full_width_p, desired_row)                  // C 2088
      && {                                                            // C 2089-2091
	   f = XFRAME ((*w).frame);
	   width = FRAME_INTERNAL_BORDER_WIDTH (f);
	   width != 0
	 }
      && {                                                            // C 2092-2093
	   height = (*desired_row).visible_height;
	   height > 0
	 }
    {
      let y: c_int                                                    // C 2095
	= WINDOW_TO_FRAME_PIXEL_Y! (w, std::cmp::max (0, (*desired_row).y));
      let face_id: c_int                                              // C 2096
	= if ! NILP (Vface_remapping_alist)                           // C 2097
	  { lookup_basic_face (std::ptr::null_mut (), f, INTERNAL_BORDER_FACE_ID) }   // C 2098
	  else
	  { INTERNAL_BORDER_FACE_ID };                                // C 2099
      let face: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);         // C 2100

      block_input ();                                                 // C 2102
      if ! face.is_null ()                                            // C 2103
	{
	  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! (f);             // C 2105
	  BView_draw_lock (view, false, 0, 0, 0, 0);                      // C 2106
	  BView_StartClip (view);                                         // C 2107
	  BView_SetHighColor (view, if face_bits! (background_defaulted_p, face)   // C 2108
				    { FRAME_BACKGROUND_PIXEL (f) as uint32_t }   // C 2109
				    else { (*face).background as uint32_t });    // C 2110
	  BView_FillRectangle (view, 0, y, width, height);                 // C 2111
	  BView_FillRectangle (view, FRAME_PIXEL_WIDTH (f) - width,        // C 2112
			       y, width, height);                         // C 2113
	  BView_invalidate_region (FRAME_HAIKU_DRAWABLE! (f),              // C 2114
				   0, y, width, height);                  // C 2115
	  BView_invalidate_region (view, FRAME_PIXEL_WIDTH (f) - width,    // C 2116
				   y, width, height);                     // C 2117
	  BView_EndClip (view);                                           // C 2118
	  BView_draw_unlock (view);                                       // C 2119
	}
      else
	{
	  haiku_clear_frame_area (f, 0, y, width, height);               // C 2123
	  haiku_clear_frame_area (f, FRAME_PIXEL_WIDTH (f) - width,      // C 2124
				  y, width, height);                     // C 2125
	}
      unblock_input ();                                               // C 2127
    }
}

/* C 2131-2167.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_window_size (f: *mut frame, change_gravity: bool,
				 width: c_int, height: c_int)
{
  /* C 2135 declares `frame` with no initialiser and C 2140 hands the callee its address, so the
     port zero-initialises it (PORTING.md 1.5; the `haikufns.rs` 2090 reading of the same local).  */
  let mut frame: Lisp_Object = Qnil ();
  let _ = change_gravity;

  /* On X Windows, window managers typically disallow resizing a
     window when it is fullscreen.  Do the same here.  */            // C 2137-2138

  XSETFRAME! (frame, f);                                              // C 2140
  if ! NILP (Fframe_parameter (frame, Qfullscreen ()))                // C 2141
      /* Only do this if the fullscreen status has actually been
	 applied.  */                                                 // C 2142-2143
      && want_fullscreen! (f) == FULLSCREEN_NONE                      // C 2144
      /* And if the configury during frame creation has been
	 completed.  Otherwise, there will be no valid "old size" to
	 go back to.  */                                             // C 2145-2147
      && (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).configury_done   // C 2148
    { return; }                                                      // C 2149

  haiku_update_size_hints (f);                                       // C 2151

  if ! FRAME_HAIKU_WINDOW! (f).is_null ()                            // C 2153
    {
      block_input ();                                                // C 2155
      BWindow_resize (FRAME_HAIKU_WINDOW! (f),                        // C 2156
		      width, height);                                // C 2157

      if FRAME_VISIBLE_P (f)                                         // C 2159
	  && (width != FRAME_PIXEL_WIDTH (f)                         // C 2160
	      || height != FRAME_PIXEL_HEIGHT (f))                   // C 2161
	{ haiku_wait_for_event (f, FRAME_RESIZED); }                 // C 2162
      unblock_input ();                                              // C 2163
    }

  do_pending_window_change (false);                                  // C 2166
}

/* C 2169-2219.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_hollow_cursor (w: *mut window, row: *mut glyph_row)
{
  let f: *mut frame;                                                 // C 2172
  /* C 2173's `x`/`y`/`h` have no initialiser and C 2188 hands the callee their addresses (the
     same reading as `frame` above; `wd` C 2189 assigns before C 2205 reads it).  */
  let mut x: c_int = 0;
  let mut y: c_int = 0;
  let mut wd: c_int;
  let mut h: c_int = 0;
  let mut cursor_glyph: *mut glyph;                                  // C 2174
  let foreground: uint32_t;                                          // C 2175
  let view: *mut c_void;                                             // C 2176

  f = XFRAME (WINDOW_FRAME! (w));                                    // C 2178
  view = FRAME_HAIKU_DRAWABLE! (f);                                  // C 2179

  /* Get the glyph the cursor is on.  If we can't tell because
     the current matrix is invalid or such, give up.  */             // C 2181-2182
  cursor_glyph = get_phys_cursor_glyph (w);                          // C 2183
  if cursor_glyph.is_null ()                                         // C 2184
    { return; }                                                      // C 2185

  /* Compute frame-relative coordinates for phys cursor.  */         // C 2187
  get_phys_cursor_geometry (w, row, cursor_glyph, &raw mut x, &raw mut y, &raw mut h);   // C 2188
  wd = (*w).phys_cursor_width;                                       // C 2189

  /* The foreground of cursor_gc is typically the same as the normal
     background color, which can cause the cursor box to be invisible.  */   // C 2191-2192
  foreground = FRAME_CURSOR_COLOR! (f).pixel as uint32_t;            // C 2193

  /* When on R2L character, show cursor at the right edge of the
     glyph, unless the cursor box is as wide as the glyph or wider
     (the latter happens when x-stretch-cursor is non-nil).  */      // C 2195-2197
  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0        // C 2198
      && (*cursor_glyph).pixel_width as c_int > wd                   // C 2199
    { x += (*cursor_glyph).pixel_width as c_int - wd; }              // C 2200

  /* Set clipping, draw the rectangle, and reset clipping again.
     This also marks the region as invalidated.  */                  // C 2202-2203

  BView_draw_lock (view, true, x, y, wd, h);                         // C 2205
  BView_StartClip (view);                                            // C 2206
  haiku_clip_to_row (w, row, TEXT_AREA);                             // C 2207

  /* Now set the foreground color and pen size.  */                   // C 2209
  BView_SetHighColor (view, foreground);                             // C 2210
  BView_SetPenSize (view, 1);                                        // C 2211

  /* Actually draw the rectangle.  */                                 // C 2213
  BView_StrokeRectangle (view, x, y, wd, h);                         // C 2214

  /* Reset clipping.  */                                             // C 2216
  BView_EndClip (view);                                              // C 2217
  BView_draw_unlock (view);                                          // C 2218
}

/* C 2220-2397: `haiku_draw_bar_cursor` (C 2221-2316), `haiku_draw_window_cursor` (C 2318-2365),
   `haiku_show_hourglass` (C 2367-2381) and `haiku_hide_hourglass` (C 2383-2397).  The next first
   definition is C 2399's `haiku_compute_glyph_string_overhangs`.  Two names are declared at the
   use site below - `draw_phys_cursor_glyph` (dispextern.h 3649-3651; `src/abi_shim.rs` 2493 names
   it and no mirror carries the prototype) and `draw_fringe_bitmap` (dispextern.h 3703) - and
   `w->phys_cursor_on_p` (window.h 422) gets its unit-local `window_bits!` setter (the
   `pgtkterm.rs` 2875-2882 shape), while `FRAME_CURSOR_COLOR!` (include/haikuterm.rs 445) and
   `MATRIX_ROW!` (include/dispextern.rs 1223) are the mirrors'.  */

/* C: dispextern.h 3649-3651: `extern void draw_phys_cursor_glyph (struct window *,
   struct glyph_row *, enum draw_glyphs_face);`
   C: dispextern.h 3703: `void draw_fringe_bitmap (struct window *, struct glyph_row *, int);`  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn draw_phys_cursor_glyph (w: *mut window, row: *mut glyph_row, draw: draw_glyphs_face);
  pub fn draw_fringe_bitmap (w: *mut window, row: *mut glyph_row, left_p: c_int);
}

/* C 2221-2316.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_bar_cursor (w: *mut window, row: *mut glyph_row,
				 mut width: c_int, kind: text_cursor_kinds)
{
  let f: *mut frame;                                                  // C 2225
  let mut cursor_glyph: *mut glyph;                                   // C 2226
  let mut r: *mut glyph_row;                                          // C 2227
  let mut face: *mut face;                                            // C 2228
  let mut foreground: uint32_t;                                       // C 2229
  let mut view: *mut c_void;                                          // C 2230
  let mut x: c_int;                                                   // C 2231
  let mut y: c_int;
  /* C 2231's `dummy_x`/`dummy_y`/`dummy_h` have no initialiser and C 2300 hands the callee their
     addresses (PORTING.md 1.5; this file's C 1752/C 2135 reading).  */
  let mut dummy_x: c_int = 0;
  let mut dummy_y: c_int = 0;
  let mut dummy_h: c_int = 0;

  f = XFRAME ((*w).frame);                                            // C 2233

  /* If cursor is out of bounds, don't draw garbage.  This can happen
     in mini-buffer windows when switching between echo area glyphs
     and mini-buffer.  */                                            // C 2235-2237
  cursor_glyph = get_phys_cursor_glyph (w);                           // C 2238
  if cursor_glyph.is_null ()                                          // C 2239
    { return; }                                                       // C 2240

  /* If on an image, draw like a normal cursor.  That's usually better
     visible than drawing a bar, esp. if the image is large so that
     the bar might not be in the window.  */                         // C 2242-2244
  if glyph_bits! (type, (*cursor_glyph)) == IMAGE_GLYPH                // C 2245
    {
      /* The mirror types `current_matrix` as the untyped `*mut c_void` (window.rs 104), so
	 the pointer is cast for `MATRIX_ROW!` - the `dispnew.rs` 2807 spelling.  */
      r = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix,   // C 2247
		       (*w).phys_cursor.vpos);
      draw_phys_cursor_glyph (w, r, DRAW_CURSOR);                      // C 2248
    }
  else
    {
      view = FRAME_HAIKU_DRAWABLE! (f);                                // C 2252
      face = FACE_FROM_ID (f, glyph_bits! (face_id, (*cursor_glyph)));   // C 2253

      /* If the glyph's background equals the color we normally draw
	 the bars cursor in, the bar cursor in its normal color is
	 invisible.  Use the glyph's foreground color instead in this
	 case, on the assumption that the glyph's colors are chosen so
	 that the glyph is legible.  */                              // C 2255-2259
      if (*face).background == FRAME_CURSOR_COLOR! (f).pixel          // C 2260
	{ foreground = (*face).foreground as uint32_t; }              // C 2261
      else
	{ foreground = FRAME_CURSOR_COLOR! (f).pixel as uint32_t; }    // C 2263

      BView_draw_lock (view, false, 0, 0, 0, 0);                      // C 2265
      BView_StartClip (view);                                         // C 2266
      BView_SetHighColor (view, foreground);                          // C 2267
      haiku_clip_to_row (w, row, TEXT_AREA);                          // C 2268

      if kind == BAR_CURSOR                                           // C 2270
	{
	  x = WINDOW_TEXT_TO_FRAME_PIXEL_X! (w, (*w).phys_cursor.x);      // C 2272
	  y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*w).phys_cursor.y);           // C 2273

	  if width < 0                                                    // C 2275
	    { width = FRAME_CURSOR_WIDTH (f); }                          // C 2276
	  width = std::cmp::min ((*cursor_glyph).pixel_width as c_int, width);   // C 2277

	  (*w).phys_cursor_width = width;                                 // C 2279

	  /* If the character under cursor is R2L, draw the bar cursor
	     on the right of its glyph, rather than on the left.  */    // C 2281-2282
	  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0     // C 2283
	    { x += (*cursor_glyph).pixel_width as c_int - width; }        // C 2284

	  BView_FillRectangle (view, x, y, width, (*row).height);         // C 2286
	  BView_invalidate_region (view, x, y, width, (*row).height);     // C 2287
	}
      else /* HBAR_CURSOR */                                          // C 2289
	{
	  x = WINDOW_TEXT_TO_FRAME_PIXEL_X! (w, (*w).phys_cursor.x);      // C 2291
	  y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*w).phys_cursor.y            // C 2292
					   + (*row).height - width);      // C 2293

	  if width < 0                                                    // C 2295
	    { width = (*row).height; }                                    // C 2296

	  width = std::cmp::min ((*row).height, width);                   // C 2298

	  get_phys_cursor_geometry (w, row, cursor_glyph, &raw mut dummy_x,   // C 2300
				    &raw mut dummy_y, &raw mut dummy_h);  // C 2301

	  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0     // C 2303
	      && (*cursor_glyph).pixel_width as c_int > (*w).phys_cursor_width - 1   // C 2304
	    { x += (*cursor_glyph).pixel_width as c_int - (*w).phys_cursor_width + 1; }   // C 2305

	  BView_FillRectangle (view, x, y, (*w).phys_cursor_width - 1,    // C 2307
			       width);                                    // C 2308
	  BView_invalidate_region (view, x, y, (*w).phys_cursor_width - 1,   // C 2309
				   width);                                // C 2310
	}

      BView_EndClip (view);                                           // C 2313
      BView_draw_unlock (view);                                       // C 2314
    }
}

/* C 2318-2365.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_draw_window_cursor (w: *mut window, glyph_row: *mut glyph_row,
				    x: c_int, y: c_int, cursor_type: text_cursor_kinds,
				    cursor_width: c_int, on_p: bool, active_p: bool)
{
  let _ = (x, y, active_p);
  if on_p                                                             // C 2323
    {
      (*w).phys_cursor_type = cursor_type;                            // C 2325
      window_bits! (phys_cursor_on_p, w, true);                       // C 2326

      if glyph_row_bits! (exact_window_width_line_p, glyph_row)       // C 2328
	  && (if glyph_row_bits! (reversed_p, glyph_row)              // C 2329
	      { (*w).phys_cursor.hpos < 0 }                           // C 2330
	      else
	      { (*w).phys_cursor.hpos >= (*glyph_row).used[TEXT_AREA as usize] as c_int })   // C 2331
	{
	  glyph_row_bits! (cursor_in_fringe_p, glyph_row, true);          // C 2333
	  draw_fringe_bitmap (w, glyph_row,                          // C 2334
			      glyph_row_bits! (reversed_p, glyph_row) as c_int);
	}
      else
	{
	  match cursor_type                                             // C 2338
	    {
	      HOLLOW_BOX_CURSOR =>                                      // C 2340
		{ haiku_draw_hollow_cursor (w, glyph_row); }             // C 2341
	      FILLED_BOX_CURSOR =>                                      // C 2344
		{ draw_phys_cursor_glyph (w, glyph_row, DRAW_CURSOR); }  // C 2345
	      BAR_CURSOR =>                                             // C 2348
		{ haiku_draw_bar_cursor (w, glyph_row, cursor_width, BAR_CURSOR); }   // C 2349
	      HBAR_CURSOR =>                                            // C 2352
		{ haiku_draw_bar_cursor (w, glyph_row, cursor_width, HBAR_CURSOR); }  // C 2353
	      NO_CURSOR =>                                              // C 2356
		{ (*w).phys_cursor_width = 0; }                          // C 2357
	      _ =>                                                      // C 2360
		{ emacs_abort (); }                                      // C 2361
	    }
	}
    }
}

/* C 2367-2381.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_show_hourglass (f: *mut frame)
{
  if FRAME_TOOLTIP_P (f)                                              // C 2370
      || haiku_output_bits! (hourglass_p, FRAME_OUTPUT_DATA! (f))     // C 2371
    { return; }                                                       // C 2372

  block_input ();                                                     // C 2374
  haiku_output_bits! (hourglass_p, FRAME_OUTPUT_DATA! (f), 1);         // C 2375

  if ! FRAME_HAIKU_VIEW! (f).is_null ()                               // C 2377
    { BView_set_view_cursor (FRAME_HAIKU_VIEW! (f),                    // C 2378
			     (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output))
			       .hourglass_cursor); }                  // C 2379
  unblock_input ();                                                   // C 2380
}

/* C 2383-2397.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_hide_hourglass (f: *mut frame)
{
  if FRAME_TOOLTIP_P (f)                                              // C 2386
      || ! haiku_output_bits! (hourglass_p, FRAME_OUTPUT_DATA! (f))   // C 2387
    { return; }                                                       // C 2388

  block_input ();                                                     // C 2390
  haiku_output_bits! (hourglass_p, FRAME_OUTPUT_DATA! (f), 0);         // C 2391

  if ! FRAME_HAIKU_VIEW! (f).is_null ()                               // C 2393
    { BView_set_view_cursor (FRAME_HAIKU_VIEW! (f),                    // C 2394
			     (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output))
			       .current_cursor); }                    // C 2395
  unblock_input ();                                                   // C 2396
}

/* C 2398-2522: `haiku_compute_glyph_string_overhangs` (C 2399-2428),
   `haiku_draw_vertical_window_border` (C 2430-2446), `haiku_set_scroll_bar_default_width`
   (C 2448-2458), `haiku_set_scroll_bar_default_height` (C 2460-2470) and
   `haiku_draw_window_divider` (C 2472-2522).  The next first definition is C 2524's
   `haiku_condemn_scroll_bars`.  Three of the five are taken by the `redisplay_interface` table at
   C 3158/3164/3165, so their C function-pointer ABI belongs to that slice (the `androidfns.rs`
   slice-13 precedent; recorded in Deviations 5).  C 2456-2457 and C 2468-2469 write
   `FRAME_CONFIG_SCROLL_BAR_WIDTH`/`_HEIGHT`/`_LINES`, whose mirror forms are readers, so the unit
   adds the three setters in the `FRAME_CONFIG_SCROLL_BAR_COLS` shape above (C 711).  */

#[cfg (feature = "have_haiku")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_WIDTH {
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_width = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_HEIGHT {
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_height = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_LINES {
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_lines = $v };
}

/* C 2399-2428.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_compute_glyph_string_overhangs (s: *mut glyph_string)
{
  if (*s).cmp.is_null ()                                              // C 2402
      && (glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH        // C 2403
	  || glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH)   // C 2404
    {
      let mut metrics: font_metrics = unsafe { core::mem::zeroed () };   // C 2406

      if glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH         // C 2408
	{
	  let font: *mut font = (*s).font;                              // C 2410
	  ((*((*font).driver as *const font_driver)).text_extents.unwrap ())
	    (font, (*s).char2b, (*s).nchars, &raw mut metrics);           // C 2411
	}
      else
	{
	  let gstring: Lisp_Object = composition_gstring_from_id ((*s).cmp_id);   // C 2415

	  composition_gstring_width (gstring, (*s).cmp_from as ptrdiff_t,   // C 2417
				     (*s).cmp_to as ptrdiff_t,
				     &raw mut metrics as *mut c_void);
	}
      (*s).right_overhang = if metrics.rbearing > metrics.width         // C 2419
	{ (metrics.rbearing - metrics.width) as c_int } else { 0 };    // C 2420
      (*s).left_overhang = if metrics.lbearing < 0                      // C 2421
	{ (- metrics.lbearing) as c_int } else { 0 };
    }
  else if ! (*s).cmp.is_null ()                                       // C 2423
    {
      (*s).right_overhang = ((*( (*s).cmp as *mut composition )).rbearing   // C 2425
			     - (*( (*s).cmp as *mut composition )).pixel_width) as c_int;
      (*s).left_overhang = (- (*( (*s).cmp as *mut composition )).lbearing) as c_int;   // C 2426
    }
}

/* C 2430-2446.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_draw_vertical_window_border (w: *mut window, x: c_int, y_0: c_int, y_1: c_int)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                     // C 2434
  let face: *mut face;                                                // C 2435

  face = FACE_FROM_ID_OR_NULL (f, VERTICAL_BORDER_FACE_ID);            // C 2437
  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! (f);                    // C 2438
  BView_draw_lock (view, true, x, y_0, 1, y_1);                         // C 2439
  BView_StartClip (view);                                               // C 2440
  if ! face.is_null ()                                                  // C 2441
    { BView_SetHighColor (view, (*face).foreground as uint32_t); }       // C 2442
  BView_StrokeLine (view, x, y_0, x, y_1);                              // C 2443
  BView_EndClip (view);                                                 // C 2444
  BView_draw_unlock (view);                                             // C 2445
}

/* C 2448-2458.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_scroll_bar_default_width (f: *mut frame)
{
  let mut unit: c_int;                                                 // C 2451
  let mut size: c_int;

  unit = FRAME_COLUMN_WIDTH (f);                                        // C 2453
  size = BScrollBar_default_size (0) + 1;                               // C 2454

  FRAME_CONFIG_SCROLL_BAR_WIDTH! (f, size);                             // C 2456
  FRAME_CONFIG_SCROLL_BAR_COLS! (f, (size + unit - 1) / unit);          // C 2457
}

/* C 2460-2470.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_scroll_bar_default_height (f: *mut frame)
{
  let mut height: c_int;                                                // C 2463
  let mut size: c_int;

  height = FRAME_LINE_HEIGHT (f);                                       // C 2465
  /* C 2466 passes C's `true` to the `int` parameter C 2454/2466 declare (haiku_support.h
     626), so the port writes that conversion out (PORTING.md 1.1's rule; slice 1's
     Deviation 9).  */
  size = BScrollBar_default_size (true as c_int) + 1;                   // C 2466

  FRAME_CONFIG_SCROLL_BAR_HEIGHT! (f, size);                            // C 2468
  FRAME_CONFIG_SCROLL_BAR_LINES! (f, (size + height - 1) / height);     // C 2469
}

/* C 2472-2522.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_draw_window_divider (w: *mut window, x0: c_int, x1: c_int,
				     y0: c_int, y1: c_int)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                      // C 2475
  let face: *mut face = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_FACE_ID);   // C 2476
  let face_first: *mut face                                             // C 2477
    = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID);     // C 2478
  let face_last: *mut face                                              // C 2479
    = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_LAST_PIXEL_FACE_ID);      // C 2480
  let color: c_ulong = if ! face.is_null ()                             // C 2481
			 { (*face).foreground } else { FRAME_FOREGROUND_PIXEL (f) };
  let color_first: c_ulong = if ! face_first.is_null ()                  // C 2482
			       { (*face_first).foreground }              // C 2483
			       else { FRAME_FOREGROUND_PIXEL (f) };      // C 2484
  let color_last: c_ulong = if ! face_last.is_null ()                    // C 2485
			      { (*face_last).foreground }                // C 2486
			      else { FRAME_FOREGROUND_PIXEL (f) };       // C 2487
  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! (f);                     // C 2488

  BView_draw_lock (view, true, x0, y0, x1 - x0 + 1, y1 - y0 + 1);        // C 2490
  BView_StartClip (view);                                               // C 2491

  if (y1 - y0 > x1 - x0) && (x1 - x0 >= 3)                              // C 2493
    /* A vertical divider, at least three pixels wide: Draw first and
       last pixels differently.  */                                    // C 2494-2495
    {
      BView_SetHighColor (view, color_first as uint32_t);                // C 2497
      BView_StrokeLine (view, x0, y0, x0, y1 - 1);                       // C 2498
      BView_SetHighColor (view, color as uint32_t);                      // C 2499
      BView_FillRectangle (view, x0 + 1, y0, x1 - x0 - 2, y1 - y0);       // C 2500
      BView_SetHighColor (view, color_last as uint32_t);                 // C 2501
      BView_StrokeLine (view, x1 - 1, y0, x1 - 1, y1 - 1);               // C 2502
    }
  else if (x1 - x0 > y1 - y0) && (y1 - y0 >= 3)                         // C 2504
    /* A horizontal divider, at least three pixels high: Draw first and
       last pixels differently.  */                                    // C 2505-2506
    {
      BView_SetHighColor (view, color_first as uint32_t);                // C 2508
      BView_StrokeLine (view, x0, y0, x1 - 1, y0);                       // C 2509
      BView_SetHighColor (view, color as uint32_t);                      // C 2510
      BView_FillRectangle (view, x0, y0 + 1, x1 - x0, y1 - y0 - 2);       // C 2511
      BView_SetHighColor (view, color_last as uint32_t);                 // C 2512
      BView_FillRectangle (view, x0, y1 - 1, x1 - x0, 1);                // C 2513
    }
  else
    {
      BView_SetHighColor (view, color as uint32_t);                      // C 2517
      BView_FillRectangleAbs (view, x0, y0, x1, y1);                     // C 2518
    }
  BView_EndClip (view);                                                 // C 2520
  BView_draw_unlock (view);                                             // C 2521
}

/* C 2523-2645: `haiku_condemn_scroll_bars` (C 2524-2544), `haiku_redeem_scroll_bar`
   (C 2546-2620, whose C 2564 `goto horizontal` is the labelled block below) and
   `haiku_judge_scroll_bars` (C 2622-2645).  The next first definition is C 2647's
   `haiku_scroll_bar_create`.  Every name these three call is a mirror's or this file's own:
   `FRAME_SCROLL_BARS`/`FRAME_CONDEMNED_SCROLL_BARS`/`fset_*` (include/frame.rs), `XSCROLL_BAR!`
   (include/haikuterm.rs 390), `WINDOW_HAS_*_SCROLL_BAR!` (include/window.rs 691/...),
   `XSETVECTOR!` (include/lisp.rs) and `haiku_scroll_bar_remove` (the mirror's
   haikuterm_decls block, 470).  */

/* C 2524-2544.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_condemn_scroll_bars (frame: *mut frame)
{
  if ! NILP (FRAME_SCROLL_BARS (frame))                               // C 2527
    {
      if ! NILP (FRAME_CONDEMNED_SCROLL_BARS (frame))                  // C 2529
	{
	  /* Prepend scrollbars to already condemned ones.  */         // C 2531
	  let mut last: Lisp_Object = FRAME_SCROLL_BARS (frame);       // C 2532

	  while ! NILP ((*XSCROLL_BAR! (last)).next)                   // C 2534
	    { last = (*XSCROLL_BAR! (last)).next; }                    // C 2535

	  (*XSCROLL_BAR! (last)).next = FRAME_CONDEMNED_SCROLL_BARS (frame);   // C 2537
	  (*XSCROLL_BAR! (FRAME_CONDEMNED_SCROLL_BARS (frame))).prev = last;   // C 2538
	}

      fset_condemned_scroll_bars (frame, FRAME_SCROLL_BARS (frame));    // C 2541
      fset_scroll_bars (frame, Qnil ());                               // C 2542
    }
}

/* C 2546-2620.  C 2564's `goto horizontal` is a `break 'horizontal` out of the labelled block
   C 2553-2585, whose end is C's `horizontal:` label.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_redeem_scroll_bar (w: *mut window)
{
  let mut bar: *mut scroll_bar;                                       // C 2549
  let mut barobj: Lisp_Object;                                        // C 2550
  let mut f: *mut frame;                                              // C 2551

  'horizontal: {
    if ! NILP ((*w).vertical_scroll_bar)                               // C 2553
	&& WINDOW_HAS_VERTICAL_SCROLL_BAR! (w)
      {
	bar = XSCROLL_BAR! ((*w).vertical_scroll_bar);                  // C 2555
	/* Unlink it from the condemned list.  */                      // C 2556
	f = XFRAME (WINDOW_FRAME! (w));                                 // C 2557
	if NILP ((*bar).prev)                                           // C 2558
	  {
	    /* If the prev pointer is nil, it must be the first in one of
	       the lists.  */                                          // C 2560-2561
	    if EQ (FRAME_SCROLL_BARS (f), (*w).vertical_scroll_bar)      // C 2562
	      /* It's not condemned.  Everything's fine.  */           // C 2563
	      { break 'horizontal; }                                    // C 2564
	    else if EQ (FRAME_CONDEMNED_SCROLL_BARS (f),                // C 2565
			(*w).vertical_scroll_bar)                       // C 2566
	      { fset_condemned_scroll_bars (f, (*bar).next); }          // C 2567
	    else
	      /* If its prev pointer is nil, it must be at the front of
		 one or the other!  */                                  // C 2569-2570
	      { emacs_abort (); }                                       // C 2571
	  }
	else
	  { (*XSCROLL_BAR! ((*bar).prev)).next = (*bar).next; }         // C 2574

	if ! NILP ((*bar).next)                                         // C 2576
	  { (*XSCROLL_BAR! ((*bar).next)).prev = (*bar).prev; }         // C 2577

	(*bar).next = FRAME_SCROLL_BARS (f);                            // C 2579
	(*bar).prev = Qnil ();                                          // C 2580
	XSETVECTOR! (barobj, bar);                                      // C 2581
	fset_scroll_bars (f, barobj);                                   // C 2582
	if ! NILP ((*bar).next)                                         // C 2583
	  { XSETVECTOR! ((*XSCROLL_BAR! ((*bar).next)).prev, bar); }     // C 2584
      }
  }

  /* horizontal: */                                                   // C 2586
  if ! NILP ((*w).horizontal_scroll_bar)                              // C 2587
      && WINDOW_HAS_HORIZONTAL_SCROLL_BAR (w)
    {
      bar = XSCROLL_BAR! ((*w).horizontal_scroll_bar);                 // C 2589
      /* Unlink it from the condemned list.  */                       // C 2590
      f = XFRAME (WINDOW_FRAME! (w));                                  // C 2591
      if NILP ((*bar).prev)                                            // C 2592
	{
	  /* If the prev pointer is nil, it must be the first in one of
	     the lists.  */                                             // C 2594-2595
	  if EQ (FRAME_SCROLL_BARS (f), (*w).horizontal_scroll_bar)       // C 2596
	    /* It's not condemned.  Everything's fine.  */              // C 2597
	    { return; }                                                  // C 2598
	  else if EQ (FRAME_CONDEMNED_SCROLL_BARS (f),                   // C 2599
		      (*w).horizontal_scroll_bar)                        // C 2600
	    { fset_condemned_scroll_bars (f, (*bar).next); }             // C 2601
	  else
	    /* If its prev pointer is nil, it must be at the front of
	       one or the other!  */                                    // C 2603-2604
	    { emacs_abort (); }                                          // C 2605
	}
      else
	{ (*XSCROLL_BAR! ((*bar).prev)).next = (*bar).next; }            // C 2608

      if ! NILP ((*bar).next)                                           // C 2610
	{ (*XSCROLL_BAR! ((*bar).next)).prev = (*bar).prev; }            // C 2611

      (*bar).next = FRAME_SCROLL_BARS (f);                              // C 2613
      (*bar).prev = Qnil ();                                            // C 2614
      XSETVECTOR! (barobj, bar);                                        // C 2615
      fset_scroll_bars (f, barobj);                                     // C 2616
      if ! NILP ((*bar).next)                                           // C 2617
	{ XSETVECTOR! ((*XSCROLL_BAR! ((*bar).next)).prev, bar); }        // C 2618
    }
}

/* C 2622-2645.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_judge_scroll_bars (f: *mut frame)
{
  let mut bar: Lisp_Object;                                            // C 2625
  let mut next: Lisp_Object;

  bar = FRAME_CONDEMNED_SCROLL_BARS (f);                               // C 2627

  /* Clear out the condemned list now so we won't try to process any
     more events on the hapless scroll bars.  */                     // C 2629-2630
  fset_condemned_scroll_bars (f, Qnil ());                             // C 2631

  while ! NILP (bar)                                                   // C 2633
    {
      let mut b: *mut scroll_bar = XSCROLL_BAR! (bar);                  // C 2635

      haiku_scroll_bar_remove (b);                                      // C 2637

      next = (*b).next;                                                 // C 2639
      /* C 2640's chained assignment `b->next = b->prev = Qnil;`.  */
      (*b).next = Qnil ();
      (*b).prev = Qnil ();
      bar = next;                                                       // C 2633's step
    }

  /* Now there should be no references to the condemned scroll bars,
     and they should get garbage-collected.  */                      // C 2643-2644
}

/* C 2646-2745: `haiku_scroll_bar_create` (C 2647-2690) and `haiku_set_horizontal_scroll_bar`
   (C 2692-2745).  The next first definition is C 2747's `haiku_set_vertical_scroll_bar`.
   `haiku_set_horizontal_scroll_bar_thumb` (C 2739's call) is this file's own C 497-516
   definition, already landed at slice 1 (line 975); everything else is a mirror's:
   `ALLOCATE_PSEUDOVECTOR!` (include/lisp.rs 2970, the `alloc.rs` 4139 spelling),
   `XSETWINDOW!`/`XSETVECTOR!` (include/lisp.rs), `WINDOW_HAS_HORIZONTAL_SCROLL_BAR` (the
   mirror's *function*, window.rs 472), `WINDOW_SCROLL_BAR_AREA_Y` (window.rs), the
   `BView_*`/`be_*` scroll-bar surface (include/haiku_support.rs) and `wset_horizontal_scroll_bar`
   (window.rs).  */

/* C 2647-2690.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_scroll_bar_create (w: *mut window, left: c_int, top: c_int,
				   width: c_int, height: c_int,
				   horizontal_p: bool) -> *mut scroll_bar
{
  let f: *mut frame;                                                  // C 2651
  let mut barobj: Lisp_Object;                                         // C 2652
  let mut bar: *mut scroll_bar;                                        // C 2653
  let mut scroll_bar: *mut c_void;                                     // C 2654
  let view: *mut c_void;                                               // C 2655

  f = XFRAME (WINDOW_FRAME! (w));                                      // C 2657
  view = FRAME_HAIKU_DRAWABLE! (f);                                    // C 2658

  block_input ();                                                      // C 2660
  bar = ALLOCATE_PSEUDOVECTOR! (scroll_bar, prev, pvec_type::PVEC_OTHER);   // C 2661

  XSETWINDOW! ((*bar).window, w);                                      // C 2663
  (*bar).top = top;                                                    // C 2664
  (*bar).left = left;                                                  // C 2665
  (*bar).width = width;                                                // C 2666
  (*bar).height = height;                                              // C 2667
  (*bar).position = 0;                                                 // C 2668
  (*bar).total = 0;                                                    // C 2669
  (*bar).dragging = 0;                                                 // C 2670
  (*bar).update = -1;                                                  // C 2671
  (*bar).horizontal = horizontal_p;                                    // C 2672

  /* C's prototype takes an `int` for the flag (haiku_support.h 624), so the port writes C's own
     `bool` -> `int` conversion out (the `BScrollBar_default_size` reading of C 2466).  */
  scroll_bar = be_make_scroll_bar_for_view (view, horizontal_p as c_int,   // C 2674
					    left, top, left + width - 1,   // C 2675
					    top + height - 1);          // C 2676
  BView_publish_scroll_bar (view, left, top, width, height);            // C 2677

  (*bar).next = FRAME_SCROLL_BARS (f);                                 // C 2679
  (*bar).prev = Qnil ();                                               // C 2680
  (*bar).scroll_bar = scroll_bar;                                      // C 2681
  XSETVECTOR! (barobj, bar);                                           // C 2682
  fset_scroll_bars (f, barobj);                                        // C 2683

  if ! NILP ((*bar).next)                                             // C 2685
    { XSETVECTOR! ((*XSCROLL_BAR! ((*bar).next)).prev, bar); }          // C 2686

  unblock_input ();                                                    // C 2688
  return bar;                                                          // C 2689
}

/* C 2692-2745.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_horizontal_scroll_bar (w: *mut window, portion: c_int, whole: c_int,
					   position: c_int)
{
  let mut barobj: Lisp_Object;                                         // C 2695
  let mut bar: *mut scroll_bar;                                        // C 2696
  let mut top: c_int;                                                  // C 2697
  let mut height: c_int;
  let mut left: c_int;
  let mut width: c_int;
  /* C 2698 declares the two outputs of C 2703's `window_box` with no initialiser and C 2703 hands
     the callee their addresses, so the port zero-initialises them (PORTING.md 1.5).  */
  let mut window_x: c_int = 0;
  let mut window_width: c_int = 0;
  let view: *mut c_void;                                               // C 2699

  eassert! (WINDOW_HAS_HORIZONTAL_SCROLL_BAR (w));                      // C 2701
  /* Get window dimensions.  */                                       // C 2702
  window_box (w, ANY_AREA, &raw mut window_x, std::ptr::null_mut (),    // C 2703
	      &raw mut window_width, std::ptr::null_mut ());
  left = window_x;                                                      // C 2704
  width = window_width;                                                 // C 2705
  top = WINDOW_SCROLL_BAR_AREA_Y! (w);                                  // C 2706
  height = WINDOW_CONFIG_SCROLL_BAR_HEIGHT! (w);                        // C 2707
  view = FRAME_HAIKU_DRAWABLE! (WINDOW_XFRAME! (w));                     // C 2708

  block_input ();                                                       // C 2710

  if NILP ((*w).horizontal_scroll_bar)                                  // C 2712
    {
      bar = haiku_scroll_bar_create (w, left, top, width, height, true);   // C 2714
      (*bar).update = position;                                          // C 2715
      (*bar).position = position;                                        // C 2716
      (*bar).total = whole;                                              // C 2717
    }
  else
    {
      bar = XSCROLL_BAR! ((*w).horizontal_scroll_bar);                   // C 2721

      if (*bar).left != left || (*bar).top != top                        // C 2723
	  || (*bar).width != width || (*bar).height != height            // C 2724
	{
	  BView_forget_scroll_bar (view, (*bar).left, (*bar).top,          // C 2726
				   (*bar).width, (*bar).height);          // C 2727
	  BView_move_frame ((*bar).scroll_bar, left, top,                  // C 2728
			    left + width - 1, top + height - 1);          // C 2729
	  BView_publish_scroll_bar (view, left, top, width, height);       // C 2730

	  (*bar).left = left;                                              // C 2732
	  (*bar).top = top;                                                // C 2733
	  (*bar).width = width;                                            // C 2734
	  (*bar).height = height;                                          // C 2735
	}
    }

  haiku_set_horizontal_scroll_bar_thumb (bar, portion, position, whole);   // C 2739
  (*bar).position = position;                                          // C 2740
  (*bar).total = whole;                                                // C 2741
  XSETVECTOR! (barobj, bar);                                           // C 2742
  wset_horizontal_scroll_bar (w, barobj);                              // C 2743
  unblock_input ();                                                    // C 2744
}

/* C 2746-2803: `haiku_set_vertical_scroll_bar` (C 2747-2803) - the vertical twin of slice 13's
   horizontal function, with `WINDOW_SCROLL_BAR_AREA_X`/`_WIDTH` in place of the Y/height pair and
   this file's own C 455-495 `haiku_set_scroll_bar_thumb` (already landed at slice 1) in place of
   the horizontal one.  The next first definition is C 2805's `haiku_draw_fringe_bitmap`.  */

/* C 2747-2803.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_vertical_scroll_bar (w: *mut window, portion: c_int, whole: c_int,
					 position: c_int)
{
  let mut barobj: Lisp_Object;                                         // C 2750
  let mut bar: *mut scroll_bar;                                        // C 2751
  let mut top: c_int;                                                  // C 2752
  let mut height: c_int;
  let mut left: c_int;
  let mut width: c_int;
  /* C 2753 declares the two outputs of C 2759's `window_box` with no initialiser and C 2759 hands
     the callee their addresses, so the port zero-initialises them (PORTING.md 1.5).  */
  let mut window_y: c_int = 0;
  let mut window_height: c_int = 0;
  let view: *mut c_void;                                               // C 2754

  eassert! (WINDOW_HAS_VERTICAL_SCROLL_BAR! (w));                       // C 2756

  /* Get window dimensions.  */                                       // C 2758
  window_box (w, ANY_AREA, std::ptr::null_mut (), &raw mut window_y,    // C 2759
	      std::ptr::null_mut (), &raw mut window_height);
  top = window_y;                                                      // C 2760
  height = window_height;                                              // C 2761

  /* Compute the left edge and the width of the scroll bar area.  */   // C 2763
  left = WINDOW_SCROLL_BAR_AREA_X! (w);                                // C 2764
  width = WINDOW_SCROLL_BAR_AREA_WIDTH! (w);                           // C 2765

  view = FRAME_HAIKU_DRAWABLE! (WINDOW_XFRAME! (w));                     // C 2767

  block_input ();                                                       // C 2769
  if NILP ((*w).vertical_scroll_bar)                                   // C 2770
    {
      bar = haiku_scroll_bar_create (w, left, top, width, height, false);   // C 2772
      (*bar).position = position;                                      // C 2773
      (*bar).total = whole;                                            // C 2774
    }
  else
    {
      bar = XSCROLL_BAR! ((*w).vertical_scroll_bar);                   // C 2778

      if (*bar).left != left || (*bar).top != top                      // C 2780
	  || (*bar).width != width || (*bar).height != height          // C 2781
	{
	  BView_forget_scroll_bar (view, (*bar).left, (*bar).top,          // C 2783
				   (*bar).width, (*bar).height);        // C 2784
	  BView_move_frame ((*bar).scroll_bar, left, top,                  // C 2785
			    left + width - 1, top + height - 1);        // C 2786
	  BView_publish_scroll_bar (view, left, top, width, height);       // C 2787

	  (*bar).left = left;                                              // C 2789
	  (*bar).top = top;                                                // C 2790
	  (*bar).width = width;                                            // C 2791
	  (*bar).height = height;                                          // C 2792
	}
    }

  haiku_set_scroll_bar_thumb (bar, portion, position, whole);          // C 2796
  (*bar).position = position;                                          // C 2797
  (*bar).total = whole;                                                // C 2798

  XSETVECTOR! (barobj, bar);                                           // C 2800
  wset_vertical_scroll_bar (w, barobj);                                // C 2801
  unblock_input ();                                                    // C 2802
}

/* `struct draw_fringe_bitmap_params` (dispextern.h 2215-2240) is this slice's other use-site
   record (P-72 rule 3/P-80): `include/dispextern.rs`'s copy and its two readers are behind
   `dispextern_face_records`, which this unit's arm set does not carry, so the copy stands here -
   the `xterm.rs` 9295-9345 one, whose measured offsets (64/8, `flags`@52, `face`@56) are the same
   record's.  `max_used_fringe_bitmap`'s (dispextern.h 3710) and `gui_define_fringe_bitmap`'s
   (3711) declarations are in the mirror's gated block now that `xterm.rs`'s Owed round owns them,
   so the unit names them here too (P-72 rule 4).  */
#[cfg (feature = "have_haiku")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct draw_fringe_bitmap_params
{
  pub which: c_int,                 // offset 0
  pub bits: *mut c_ushort,          // offset 8
  pub wd: c_int,                    // offset 16
  pub h: c_int,                     // offset 20
  pub dh: c_int,                    // offset 24
  pub x: c_int,                     // offset 28
  pub y: c_int,                     // offset 32
  pub bx: c_int,                    // offset 36
  pub nx: c_int,                    // offset 40
  pub by: c_int,                    // offset 44
  pub ny: c_int,                    // offset 48
  /// C 2230-2231's two `bool_bf : 1` members, stored as one byte (PORTING.md 1.3/P-80).
  pub flags: u8,                    // offset 52
  pub face: *mut face,              // offset 56
}
#[cfg (feature = "have_haiku")]
const _: () = {
  core::assert! (std::mem::size_of::<draw_fringe_bitmap_params> () == 64);
  core::assert! (std::mem::align_of::<draw_fringe_bitmap_params> () == 8);
  core::assert! (std::mem::offset_of! (draw_fringe_bitmap_params, flags) == 52);
  core::assert! (std::mem::offset_of! (draw_fringe_bitmap_params, face) == 56);
};
/// C: `bool_bf cursor_p : 1;` (dispextern.h 2230) - bit 0 of the byte at 52.
#[cfg (feature = "have_haiku")]
pub const DRAW_FRINGE_BITMAP_CURSOR_P: u8 = 1 << 0;
/// C: `bool_bf overlay_p : 1;` (dispextern.h 2231) - bit 1 of the byte at 52.
#[cfg (feature = "have_haiku")]
pub const DRAW_FRINGE_BITMAP_OVERLAY_P: u8 = 1 << 1;
#[cfg (feature = "have_haiku")]
#[inline(always)]
pub fn get_cursor_p (p: *const draw_fringe_bitmap_params) -> bool
{ unsafe { ((*p).flags & DRAW_FRINGE_BITMAP_CURSOR_P) != 0 } }
#[cfg (feature = "have_haiku")]
#[inline(always)]
pub fn get_overlay_p (p: *const draw_fringe_bitmap_params) -> bool
{ unsafe { ((*p).flags & DRAW_FRINGE_BITMAP_OVERLAY_P) != 0 } }
/// C: dispextern.h 3710: `extern int max_used_fringe_bitmap;` (defined by `src/fringe.c` 497).
/// C: dispextern.h 3711: `void gui_define_fringe_bitmap (struct frame *, int);` (xdisp.c)
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub static mut max_used_fringe_bitmap: c_int;
  pub fn gui_define_fringe_bitmap (f: *mut frame, which: c_int);
}

/* C 2804-2892: `haiku_draw_fringe_bitmap` (C 2805-2892).  The next first definition is C 2894's
   `haiku_define_fringe_bitmap`.  `p`'s two `bool_bf` members are the mirror's readers
   (`get_overlay_p`/`get_cursor_p`, include/dispextern.rs 3124/3137), `row`'s `stipple_p` is
   `glyph_row_bits!`'s, `fringe_bmps`/`max_fringe_bmp` are this file's own C 55/58 statics,
   `max_used_fringe_bitmap` (include/dispextern.rs 2930) and `gui_define_fringe_bitmap` are
   mirrors', and `haiku_get_bitmap_rec`/`haiku_update_bitmap_rec`/`haiku_clip_to_row` are slices
   1-2 of this file.  */

/* C 2805-2892.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_draw_fringe_bitmap (w: *mut window, row: *mut glyph_row,
				    p: *mut draw_fringe_bitmap_params)
{
  let face: *mut face;                                                // C 2809
  let f: *mut frame;                                                  // C 2810
  let mut rec: *mut haiku_bitmap_record;                              // C 2811
  let mut view: *mut c_void;                                          // C 2812
  let mut bitmap: *mut c_void;
  let mut col: uint32_t;                                              // C 2813

  f = XFRAME (WINDOW_FRAME! (w));                                     // C 2815
  view = FRAME_HAIKU_DRAWABLE! (f);                                   // C 2816
  face = (*p).face;                                                   // C 2817

  block_input ();                                                     // C 2819
  BView_draw_lock (view, true, 0, 0, 0, 0);                           // C 2820
  BView_StartClip (view);                                             // C 2821

  if (*p).wd != 0 && (*p).h != 0                                      // C 2823
    { BView_invalidate_region (view, (*p).x, (*p).y, (*p).wd, (*p).h); }   // C 2824

  haiku_clip_to_row (w, row, ANY_AREA);                               // C 2826

  if (*p).bx >= 0 && ! get_overlay_p (p)                              // C 2828
    {
      BView_invalidate_region (view, (*p).bx, (*p).by, (*p).nx, (*p).ny);   // C 2830

      if (*face).stipple == 0                                         // C 2832
	{
	  BView_SetHighColor (view, (*face).background as uint32_t);     // C 2834
	  BView_FillRectangle (view, (*p).bx, (*p).by, (*p).nx, (*p).ny);   // C 2835
	}
      else
	{
	  rec = haiku_get_bitmap_rec (f, (*face).stipple);                // C 2839
	  haiku_update_bitmap_rec (rec, (*face).foreground as uint32_t,   // C 2840
				   (*face).background as uint32_t);     // C 2841

	  BView_StartClip (view);                                        // C 2843
	  haiku_clip_to_row (w, row, ANY_AREA);                          // C 2844
	  BView_ClipToRect (view, (*p).bx, (*p).by, (*p).nx, (*p).ny);    // C 2845
	  BView_DrawBitmapTiled (view, (*rec).img, 0, 0, -1, -1,          // C 2846
				 0, 0, FRAME_PIXEL_WIDTH (f),            // C 2847
				 FRAME_PIXEL_HEIGHT (f));                // C 2848
	  BView_EndClip (view);                                          // C 2849

	  glyph_row_bits! (stipple_p, row, true);                        // C 2851
	}
    }

  if (*p).which != 0                                                  // C 2855
      && (*p).which < max_fringe_bmp                                 // C 2856
      && (*p).which < max_used_fringe_bitmap                         // C 2857
    {
      bitmap = *fringe_bmps.offset ((*p).which as isize);             // C 2859

      if bitmap.is_null ()                                            // C 2861
	{
	  /* This fringe bitmap is known to fringe.c, but lacks the
	     BBitmap which shadows that bitmap.  This is typical to
	     define-fringe-bitmap being called when the selected frame
	     was not a GUI frame, for example, when packages that
	     define fringe bitmaps are loaded by a daemon Emacs.
	     Create the missing pattern now.  */                       // C 2863-2868
	  gui_define_fringe_bitmap (WINDOW_XFRAME! (w), (*p).which);      // C 2869
	  bitmap = *fringe_bmps.offset ((*p).which as isize);             // C 2870
	}

      if ! get_cursor_p (p)                                           // C 2873
	{ col = (*face).foreground as uint32_t; }                     // C 2874
      else if get_overlay_p (p)                                       // C 2875
	{ col = (*face).background as uint32_t; }                     // C 2876
      else
	{ col = FRAME_CURSOR_COLOR! (XFRAME (WINDOW_FRAME! (w))).pixel as uint32_t; }   // C 2878

      if ! get_overlay_p (p)                                          // C 2880
	{
	  BView_SetHighColor (view, (*face).background as uint32_t);      // C 2882
	  BView_FillRectangle (view, (*p).x, (*p).y, (*p).wd, (*p).h);     // C 2883
	}

      BView_SetLowColor (view, col);                                  // C 2886
      BView_DrawBitmapWithEraseOp (view, bitmap, (*p).x, (*p).y,       // C 2887
				   (*p).wd, (*p).h);
    }
  BView_EndClip (view);                                               // C 2889
  BView_draw_unlock (view);                                           // C 2890
  unblock_input ();                                                   // C 2891
}

/* C 2893-2980: `haiku_define_fringe_bitmap` (C 2894-2915), `haiku_destroy_fringe_bitmap`
   (C 2917-2926) and `haiku_scroll_run` (C 2928-2980).  The next first definition is C 2982's
   conditional block's first item (`haiku_translate_rect`).  `max_fringe_bmp`/`fringe_bmps` are this
   file's own C 55/58 statics, the `BBitmap_*` surface and `BView_CopyBits` are the mirror's
   (`include/haiku_support.rs`), `xmalloc`/`xrealloc`/`memory_full_up` are lisp.rs's, and
   `gui_clear_cursor` (dispextern.h 3657) is declared at the use site below - no mirror carries it
   (the `pgtkterm.rs` 4448 declaration of the same prototype).  */

/// C: dispextern.h 3657: `extern void gui_clear_cursor (struct window *);` (xdisp.c)
/// C: dispextern.h 3585-3587: `void remember_mouse_glyph (struct frame *, int, int,
/// NativeRectangle *);` (xdisp.c) - C 3036's call, whose fourth argument is C's
/// `&x_display_list->last_mouse_glyph`.
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn gui_clear_cursor (w: *mut window);
  pub fn remember_mouse_glyph (f: *mut frame, x: c_int, y: c_int, rect: *mut haiku_rect);
}

/* C 2894-2915.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_define_fringe_bitmap (which: c_int, bits: *mut c_ushort,
				      h: c_int, wd: c_int)
{
  if which >= max_fringe_bmp                                          // C 2898
    {
      let mut i: c_int = max_fringe_bmp;                              // C 2900
      max_fringe_bmp = which + 20;                                    // C 2901
      /* C 2902-2903's `!i ? xmalloc (…) : xrealloc (…)` and the port's two calls of the
	 same two functions.  */
      fringe_bmps = if i == 0
	{ xmalloc (max_fringe_bmp as usize * std::mem::size_of::<*mut c_void> ()) as *mut *mut c_void }
	else
	{ xrealloc (fringe_bmps as *mut c_void,
		    max_fringe_bmp as usize * std::mem::size_of::<*mut c_void> ()) as *mut *mut c_void };

      while i < max_fringe_bmp                                        // C 2905
	{
	  *fringe_bmps.offset (i as isize) = std::ptr::null_mut ();     // C 2906 (`fringe_bmps[i++] = NULL`)
	  i += 1;
	}
    }

  block_input ();                                                     // C 2909
  *fringe_bmps.offset (which as isize) = BBitmap_new (wd, h, 1);        // C 2910
  if (*fringe_bmps.offset (which as isize)).is_null ()                  // C 2911
    { memory_full_up (); }                                            // C 2912
  BBitmap_import_fringe_bitmap (*fringe_bmps.offset (which as isize),   // C 2913
				bits, wd, h);
  unblock_input ();                                                   // C 2914
}

/* C 2917-2926.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_destroy_fringe_bitmap (which: c_int)
{
  if which >= max_fringe_bmp                                          // C 2920
    { return; }                                                       // C 2921

  if ! (*fringe_bmps.offset (which as isize)).is_null ()                // C 2923
    { BBitmap_free (*fringe_bmps.offset (which as isize)); }            // C 2924
  *fringe_bmps.offset (which as isize) = std::ptr::null_mut ();         // C 2925
}

/* C 2928-2980.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_scroll_run (w: *mut window, run: *mut run)
{
  let f: *mut frame = XFRAME ((*w).frame);                            // C 2931
  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! (f);                   // C 2932
  /* C 2933 declares x/y/width/height with no initialiser and C 2934 hands the callee their
     addresses, so the port zero-initialises them (PORTING.md 1.5).  */
  let mut x: c_int = 0;
  let mut y: c_int = 0;
  let mut width: c_int = 0;
  let mut height: c_int = 0;
  let mut from_y: c_int;
  let mut to_y: c_int;
  let mut bottom_y: c_int;
  window_box (w, ANY_AREA, &raw mut x, &raw mut y, &raw mut width,     // C 2934
	      &raw mut height);

  from_y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*run).current_y);             // C 2936
  to_y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*run).desired_y);               // C 2937
  bottom_y = y + height;                                              // C 2938

  if to_y < from_y                                                    // C 2940
    {
      /* Scrolling up.  Make sure we don't copy part of the mode
	 line at the bottom.  */                                     // C 2942-2943
      if from_y + (*run).height > bottom_y                             // C 2944
	{ height = bottom_y - from_y; }                               // C 2945
      else
	{ height = (*run).height; }                                   // C 2947

      /* Don't draw over the display parts above if destination is off
         the top of the current window.  */                          // C 2949-2950
      if to_y < y                                                     // C 2951
	{
	  let d: c_int = y - to_y;                                      // C 2953
	  height -= d;                                                 // C 2954
	  to_y += d;                                                   // C 2955
	  from_y += d;                                                 // C 2956
	}
    }
  else
    {
      /* Scrolling down.  Make sure we don't copy over the mode line.
	 at the bottom.  */                                          // C 2961-2962
      if to_y + (*run).height > bottom_y                               // C 2963
	{ height = bottom_y - to_y; }                                 // C 2964
      else
	{ height = (*run).height; }                                   // C 2966
    }

  block_input ();                                                     // C 2969
  gui_clear_cursor (w);                                               // C 2970

  BView_draw_lock (view, true, x, to_y, width, height);                // C 2972
  BView_StartClip (view);                                             // C 2973
  BView_CopyBits (view, x, from_y, width, height,                      // C 2974
		  x, to_y, width, height);                            // C 2975
  BView_EndClip (view);                                               // C 2976
  BView_draw_unlock (view);                                           // C 2977

  unblock_input ();                                                   // C 2979
}

/* C 2981-3077: `haiku_mouse_position` (C 2988-3061) and `haiku_flush` (C 3063-3077), the latter
   under the C 2982-2987 comment block.  The next first definition is C 3080's
   `haiku_define_frame_cursor`.  **Both take a C function-pointer ABI later**: `haiku_flush` is the
   second entry of the `redisplay_interface` table (C 3151) and `haiku_mouse_position` is
   `terminal->mouse_position_hook` (C 4431), so each must become `unsafe extern "C" fn` when its
   referrer lands (the `androidfns.rs` slice-13 precedent; recorded in Deviations 4).  */

/* C 2988-3061.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_mouse_position (fp: *mut *mut frame, insist: c_int,
				bar_window: *mut Lisp_Object,
				part: *mut scroll_bar_part, x: *mut Lisp_Object,
				y: *mut Lisp_Object, timestamp: *mut Time)
{
  let mut frame: Lisp_Object;                                         // C 2993
  let mut tail: Lisp_Object;
  let mut f1: *mut frame;                                             // C 2994
  /* C 2995 declares the two outputs of C 3035's `BView_get_mouse` with no initialiser and C 3035
     hands the callee their addresses, so the port zero-initialises them (PORTING.md 1.5).  */
  let mut screen_x: c_int = 0;
  let mut screen_y: c_int = 0;
  let mut view: *mut c_void;                                          // C 2996

  if fp.is_null ()                                                    // C 2998
    { return; }                                                       // C 2999

  f1 = std::ptr::null_mut ();                                         // C 3001
  block_input ();                                                     // C 3002

  FOR_EACH_FRAME! (tail, frame, {                                     // C 3004
    if FRAME_HAIKU_P (XFRAME (frame))                                 // C 3006
      { mouse_moved! (XFRAME (frame), false); }                       // C 3007
  });

  if gui_mouse_grabbed (x_display_list)                              // C 3010
      && ! EQ (track_mouse, Qdropping ())                             // C 3011
      && ! EQ (track_mouse, Qdrag_source ())                          // C 3012
    { f1 = (*x_display_list).last_mouse_frame; }                      // C 3013
  else
    { f1 = (*x_display_list).last_mouse_motion_frame; }               // C 3015

  if f1.is_null () && FRAME_HAIKU_P (SELECTED_FRAME ())                // C 3017
    { f1 = SELECTED_FRAME (); }                                       // C 3018

  if f1.is_null () || (! FRAME_HAIKU_P (f1) && insist > 0)             // C 3020
    { FOR_EACH_FRAME! (tail, frame, {                                 // C 3021
	if FRAME_HAIKU_P (XFRAME (frame))                             // C 3022
	    && ! FRAME_TOOLTIP_P (XFRAME (frame))                      // C 3023
	  { f1 = XFRAME (frame); }                                    // C 3024
      }); }

  if ! f1.is_null () && FRAME_TOOLTIP_P (f1)                           // C 3026
    { f1 = std::ptr::null_mut (); }                                   // C 3027

  if ! f1.is_null () && FRAME_HAIKU_P (f1)                             // C 3029
    {
      view = FRAME_HAIKU_VIEW! (f1);                                   // C 3031

      if ! view.is_null ()                                             // C 3033
	{
	  BView_get_mouse (view, &raw mut screen_x, &raw mut screen_y);   // C 3035
	  remember_mouse_glyph (f1, screen_x, screen_y,                   // C 3036
				&raw mut (*x_display_list).last_mouse_glyph);   // C 3037
	  (*x_display_list).last_mouse_glyph_frame = f1;                  // C 3038

	  *bar_window = Qnil ();                                          // C 3040
	  *part = scroll_bar_nowhere;                                     // C 3041

	  /* If track-mouse is `drag-source' and the mouse pointer is
	     certain to not be actually under the chosen frame, return
	     NULL in FP to at least try being consistent with X.  */    // C 3043-3045
	  if EQ (track_mouse, Qdrag_source ())                           // C 3046
	      && (screen_x < 0 || screen_y < 0                           // C 3047
		  || screen_x >= FRAME_PIXEL_WIDTH (f1)                 // C 3048
		  || screen_y >= FRAME_PIXEL_HEIGHT (f1))               // C 3049
	    { *fp = std::ptr::null_mut (); }                              // C 3050
	  else
	    { *fp = f1; }                                                // C 3052

	  *timestamp = (*x_display_list).last_mouse_movement_time;         // C 3054
	  /* The mirror's `XSETINT!` hands its second argument to `make_fixnum`, which takes an
	     `isize`, so C's two `int` -> `EMACS_INT` conversions are written out (PORTING.md 1.1).  */
	  XSETINT! (*x, screen_x as isize);                               // C 3055
	  XSETINT! (*y, screen_y as isize);                               // C 3056
	}
    }

  unblock_input ();                                                   // C 3060
}

/* Haiku doesn't provide any way to get the frame actually underneath
   the pointer, so we typically return dpyinfo->last_mouse_frame if
   the display is grabbed and `track-mouse' is not `dropping' or
   `drag-source'; failing that, we return the selected frame, and
   finally a random window system frame (as long as `track-mouse' is
   not `drag-source') if that didn't work either.  */               // C 2982-2987

/* C 3063-3077.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_flush (f: *mut frame)
{
  /* This is needed for tooltip frames to work properly with double
     buffering.  */                                                   // C 3066-3067
  if FRAME_DIRTY_P! (f) && ! buffer_flipping_blocked_p ()              // C 3068
    { haiku_flip_buffers (f); }                                       // C 3069

  /* The frame is complete again as its contents were just
     flushed.  */                                                     // C 3071-3072
  FRAME_COMPLETE_P! (f, true);                                        // C 3073

  if FRAME_VISIBLE_P (f) && ! FRAME_TOOLTIP_P (f)                      // C 3075
    { BWindow_Flush (FRAME_HAIKU_WINDOW! (f)); }                       // C 3076
}

/* C 3078-3091: `haiku_define_frame_cursor` (C 3080-3091).  The next first definition is C 3093's
   `haiku_default_font_parameter`.  C 3086's `f->pointer_invisible` is a bit of `frame`'s flag run
   (frame.h 434-441's run; the mirror names the shift, `include/frame.rs` 542-546) whose accessor
   the unit writes itself - the `androidterm.rs` 240-243 copy of this same member.  **This function
   is also the `redisplay_interface` table's cursor entry (C 3160)**, so it joins that round's ABI
   conversion (Deviations 2).  */

/* C: `bool_bf pointer_invisible : 1;` - read at C 3086.  */
#[cfg (feature = "have_haiku")]
macro_rules! pointer_invisible {
  ($f:expr) => { (*$f).bits1 >> FRAME_POINTER_INVISIBLE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_POINTER_INVISIBLE_SHIFT, $v) };
}

/* C 3080-3091.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_define_frame_cursor (f: *mut frame, cursor: Emacs_Cursor)
{
  if FRAME_TOOLTIP_P (f)                                              // C 3082
    { return; }                                                       // C 3083

  block_input ();                                                     // C 3085
  if ! pointer_invisible! (f) && ! FRAME_HAIKU_VIEW! (f).is_null ()    // C 3086
      && ! haiku_output_bits! (hourglass_p, FRAME_OUTPUT_DATA! (f))   // C 3087
    { BView_set_view_cursor (FRAME_HAIKU_VIEW! (f), cursor); }         // C 3088
  unblock_input ();                                                   // C 3089
  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).current_cursor     // C 3090
    = cursor;
}

/* C 3092-3138: `haiku_default_font_parameter` (C 3093-3138).  The next first definition is the
   C 3140-3170 `redisplay_interface` table.  Four names this function calls, plus `enum
   resource_types` and its `RES_TYPE_STRING`, are declared at the use site below because no mirror
   carries them - the `androidfns.rs` 129-183 copies of the same five; each declaration's own
   `/// C:` note carries its reference line.  */

/// C: dispextern.h 3983: `enum resource_types { RES_TYPE_NUMBER, RES_TYPE_FLOAT, RES_TYPE_BOOLEAN,
/// RES_TYPE_STRING, RES_TYPE_SYMBOL, RES_TYPE_BOOLEAN_NUMBER };` - C's enum ABI is `c_int`
/// (PORTING.md 1.3); the unit names the member C 3098/3115/3137 pass.
#[cfg (feature = "have_haiku")]
pub type resource_types = c_int;
#[cfg (feature = "have_haiku")]
pub const RES_TYPE_STRING: resource_types = 3;          // C 3987

#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /// C: dispextern.h 3994: `extern Lisp_Object gui_display_get_arg (Display_Info *, Lisp_Object,
  /// Lisp_Object, const char *, const char *, enum resource_types);`
  pub fn gui_display_get_arg (dpyinfo: *mut Display_Info, alist: Lisp_Object, param: Lisp_Object,
			      name: *const c_char, x_name: *const c_char,
			      r#type: resource_types) -> Lisp_Object;
  /// C: dispextern.h 4001: `extern Lisp_Object gui_default_parameter (struct frame *, Lisp_Object,
  /// Lisp_Object, Lisp_Object, const char *, const char *, enum resource_types);`
  pub fn gui_default_parameter (f: *mut frame, alist: Lisp_Object, prop: Lisp_Object,
				deflt: Lisp_Object, name: *const c_char, x_name: *const c_char,
				r#type: resource_types) -> Lisp_Object;
  /// C: font.h 897: `extern Lisp_Object font_open_by_spec (struct frame *, Lisp_Object);`
  pub fn font_open_by_spec (f: *mut frame, spec: Lisp_Object) -> Lisp_Object;
  /// C: font.h 883: `extern Lisp_Object font_open_by_name (struct frame *f, Lisp_Object name);`
  pub fn font_open_by_name (f: *mut frame, name: Lisp_Object) -> Lisp_Object;
}

/* C 3093-3138.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_default_font_parameter (f: *mut frame, parms: Lisp_Object)
{
  let dpyinfo: *mut haiku_display_info = FRAME_DISPLAY_INFO! (f);      // C 3096
  let mut font_param: Lisp_Object                                      // C 3097-3098
    = gui_display_get_arg (dpyinfo, parms, Qfont (), std::ptr::null (), std::ptr::null (),
			   RES_TYPE_STRING);
  let mut font: Lisp_Object = Qnil ();                                 // C 3099
  if BASE_EQ (font_param, Qunbound ())                                 // C 3100
    { font_param = Qnil (); }                                         // C 3101

  if NILP (font_param)                                                 // C 3103
    /* System font should take precedence over X resources.  We
       suggest this regardless of font-use-system-font because .emacs
       may not have been read yet.  Returning a font-spec is Haiku
       specific behavior.  */                                         // C 3104-3107
    { font = font_open_by_spec (f, Ffont_get_system_font ()); }        // C 3108

  if NILP (font)                                                       // C 3110
    { font = if ! NILP (font_param) { font_param }                     // C 3112
	     else { gui_display_get_arg (dpyinfo, parms, Qfont (),     // C 3113
					 c"font".as_ptr (), c"Font".as_ptr (),
					 RES_TYPE_STRING) }; }         // C 3115

  if ! FONTP (font) && ! STRINGP (font)                                // C 3117
    {
      /* C 3119-3122's compound literal `(const char *[]) { … , NULL }`, kept as C's four-entry
	 array (the fourth entry is C's `NULL`).  */
      let names: [*const c_char; 4] = [c"monospace-12".as_ptr (),      // C 3119
				       c"Noto Sans Mono-12".as_ptr (),   // C 3120
				       c"Source Code Pro-12".as_ptr (),  // C 3121
				       std::ptr::null ()];               // C 3122
      let mut i: c_int;                                                // C 3123

      i = 0;                                                           // C 3125
      while ! names[i as usize].is_null ()                             // C 3125
	{
	  font = font_open_by_name (f, build_unibyte_string (names[i as usize]));   // C 3127-3128
	  if ! NILP (font)                                              // C 3129
	    { break; }                                                  // C 3130
	  i += 1;                                                       // C 3125's step
	}
      if NILP (font)                                                   // C 3132
	{ error (c"No suitable font was found".as_ptr ()); }           // C 3133
    }

  gui_default_parameter (f, parms, Qfont (), font, c"font".as_ptr (),   // C 3136
			 c"Font".as_ptr (), RES_TYPE_STRING);          // C 3137
}

/* C: lisp.h 5117: `extern void store_frame_param (struct frame *, Lisp_Object, Lisp_Object);`
   (C 3191's call; no mirror carries it, so the unit declares it here - P-72 rule 4).  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object);
}

/* dispextern.h 3069-3196's `struct redisplay_interface`: `include/dispextern.rs` carries the
   record behind `feature = "dispextern_face_records"` (off in every configuration today), so the
   unit carries C's record itself - the `src/haikufns.rs` 1331-1380 copy of the same record, with
   one difference: that copy types the `struct window *`/`struct frame *` slots `*mut c_void`
   because it only reads the record's offsets, while this unit's reader is the object below, whose
   entries are *this file's own functions* and whose types are therefore the *field's*; C's own
   prototype text names the pointees, so this copy does too, and no entry needs a conversion.
   `struct it` (C 3076) is the one pointee the unit has no record for, so it keeps the
   pointer-sized stand-in (P-72 rule 3) and the use-site declaration below spells it that way.
   C's initializer has 28 entries *inside* this record's `HAVE_WINDOW_SYSTEM` arm, which is also
   the proof that the arm is the live one for a Haiku build: `configure.ac` 7518 defines
   `HAVE_WINDOW_SYSTEM` and 3106-3110 `AC_DEFINE`s `HAVE_HAIKU` for the same `HAVE_BE_APP=yes`.
   Every field is one pointer wide, so the arm's fifteen hooks add 120 bytes *after*
   `fix_overlapping_area` and shift nothing above it (measured at 224/8).  */
#[cfg (feature = "have_haiku")]
#[repr(C)]
#[derive (Clone, Copy)]
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
   `docs/baseline/redisplay_interface-layout.tsv` `ns_cocoa` arm, which is this record's.  */
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (core::mem::size_of::<redisplay_interface> () == 224);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (core::mem::align_of::<redisplay_interface> () == 8);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, flush_display) == 72);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, draw_fringe_bitmap) == 104);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, clear_under_internal_border) == 160);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, default_font_parameter) == 216);

/* C: dispextern.h 3636-3638: `extern void gui_get_glyph_overhangs (struct glyph *, struct frame *,
   int *, int *);` - C 3153's entry.  dispextern.h 3639: `extern void gui_produce_glyphs (struct
   it *);` - C 3143.  dispextern.h 3641-3642 / 3643-3644: `extern void gui_write_glyphs (struct
   window *, struct glyph_row *, struct glyph *, enum glyph_row_area, int);` and the
   `gui_insert_glyphs` pair - C 3144/3145.  dispextern.h 3645-3646: `extern void
   gui_clear_end_of_line (struct window *, struct glyph_row *, enum glyph_row_area, int);` -
   C 3146.  dispextern.h 3647-3648: `extern void gui_fix_overlapping_area (struct window *, struct
   glyph_row *, enum glyph_row_area, int);` - C 3154.  dispextern.h 3689: `extern void
   gui_clear_window_mouse_face (struct window *);` - C 3152.  These seven are `xdisp.c`'s
   definitions and **no mirror carries them** (`include/dispextern.rs` names them as owed to the
   integrator), so the unit declares them at the use site, as `src/pgtkterm.rs` 5375-5395 does for
   its own table; `struct it` is the one pointee this unit has no record for, so that parameter
   keeps the pointer-sized stand-in (P-72 rule 3).  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn gui_get_glyph_overhangs (glyph: *mut glyph, f: *mut frame, left: *mut c_int,
				  right: *mut c_int);
  pub fn gui_produce_glyphs (it: *mut c_void);
  pub fn gui_write_glyphs (w: *mut window, row: *mut glyph_row, string: *mut glyph,
			   area: glyph_row_area, len: c_int);
  pub fn gui_insert_glyphs (w: *mut window, row: *mut glyph_row, start: *mut glyph,
			    area: glyph_row_area, len: c_int);
  pub fn gui_clear_end_of_line (w: *mut window, row: *mut glyph_row, area: glyph_row_area,
				x: c_int);
  pub fn gui_fix_overlapping_area (w: *mut window, row: *mut glyph_row, area: glyph_row_area,
				   overlaps: c_int);
  pub fn gui_clear_window_mouse_face (w: *mut window);
}

/* C 3139-3170: `static struct redisplay_interface haiku_redisplay_interface = { … }` - the window
   system's interface table.  C's initializer is **positional** and Rust has none, so the port
   writes a named-field literal whose names are the record's (the copy just above); C has exactly
   **28 entries** and the record exactly **28 fields, in the same order**, so every value lands in
   C's own slot and nothing is left to C's implicit zeroing.  C's three `NULL`s are `None`
   (C 3149, C 3150 and C 3166), the first entry is `haikuterm.h` 235's `haiku_frame_parm_handlers` -
   an object the *other* unit defines (`src/haikufns.rs` 4094) and this unit reaches through the
   mirror's pointer-sized declaration (`include/haikuterm.rs` 334) - and **every other entry is
   `Some (name)` with nothing converted**, because the record above spells C's own parameter types
   and an item's type is therefore the field's type.  That is where this table deviates from
   `src/pgtkterm.rs` 5412-5480, the port's first object of this record (Deviation 47 of that lane):
   *that* lane's same seventeen-ish entries need `transmute`s only because its record comes from
   the mirror, which types the `struct window *`/`struct frame *` slots `*mut c_void`; measured
   here, `flush_display` (C 3151) is the one entry the mirror's spelling would have got *wrong*
   anyway - dispextern.h 3115's slot takes `struct frame *`, not `struct window *`.
   Two of the twenty-five named entries have their C text outside the ported prefix:
   `haiku_clear_frame_area` (C 248-262, slice 1) and `haiku_clear_under_internal_border` (C 4571-
   4618), which is landed out of order at this file's end for exactly this object's sake.  */
// C 3140-3170.
#[cfg (feature = "have_haiku")]
static mut haiku_redisplay_interface: redisplay_interface =
  redisplay_interface {
    frame_parm_handlers: &raw mut haiku_frame_parm_handlers as *mut c_void,  // C 3142
    produce_glyphs: Some (gui_produce_glyphs),                          // C 3143
    write_glyphs: Some (gui_write_glyphs),                              // C 3144
    insert_glyphs: Some (gui_insert_glyphs),                            // C 3145
    clear_end_of_line: Some (gui_clear_end_of_line),                    // C 3146
    scroll_run_hook: Some (haiku_scroll_run),                           // C 3147
    after_update_window_line_hook: Some (haiku_after_update_window_line),  // C 3148
    update_window_begin_hook: None,                                     // C 3149 (`NULL`)
    update_window_end_hook: None,                                       // C 3150 (`NULL`)
    flush_display: Some (haiku_flush),                                  // C 3151
    clear_window_mouse_face: Some (gui_clear_window_mouse_face),        // C 3152
    get_glyph_overhangs: Some (gui_get_glyph_overhangs),                // C 3153
    fix_overlapping_area: Some (gui_fix_overlapping_area),              // C 3154
    draw_fringe_bitmap: Some (haiku_draw_fringe_bitmap),                // C 3155
    define_fringe_bitmap: Some (haiku_define_fringe_bitmap),            // C 3156
    destroy_fringe_bitmap: Some (haiku_destroy_fringe_bitmap),          // C 3157
    compute_glyph_string_overhangs: Some (haiku_compute_glyph_string_overhangs),  // C 3158
    draw_glyph_string: Some (haiku_draw_glyph_string),                  // C 3159
    define_frame_cursor: Some (haiku_define_frame_cursor),              // C 3160
    clear_frame_area: Some (haiku_clear_frame_area),                    // C 3161
    clear_under_internal_border: Some (haiku_clear_under_internal_border),  // C 3162
    draw_window_cursor: Some (haiku_draw_window_cursor),                // C 3163
    draw_vertical_window_border: Some (haiku_draw_vertical_window_border),  // C 3164
    draw_window_divider: Some (haiku_draw_window_divider),              // C 3165
    shift_glyphs_for_insert: None,                                      // C 3166 (`NULL`)
    show_hourglass: Some (haiku_show_hourglass),                        // C 3167
    hide_hourglass: Some (haiku_hide_hourglass),                        // C 3168
    default_font_parameter: Some (haiku_default_font_parameter),         // C 3169
  };


/* C 3171-3206: `haiku_make_fullscreen_consistent` (C 3173-3192) and
   `haiku_flush_dirty_back_buffer_on` (C 3194-3206).  The C 3140-3170 `redisplay_interface` table
   now stands above this comment (slice 22), so the file returns to C's own order here; the next
   block after these two is the **out-of-order** C 4572 one at the end of the file (there is no
   C 3208 item to fall through to: the prefix's next range is C 3207-4570, still owed).
   `FRAME_COMPLETE_P (f)` (C 3201) is the mirror's one-argument reader (its write arm is the
   two-argument form this file uses at C 1537).  */

/* C 3173-3192.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_make_fullscreen_consistent (f: *mut frame)
{
  let mut lval: Lisp_Object;                                          // C 3175
  let output: *mut haiku_output;                                      // C 3176

  output = FRAME_OUTPUT_DATA! (f) as *mut haiku_output;                // C 3178

  if (*output).fullscreen_mode == FULLSCREEN_MODE_BOTH                // C 3180
    { lval = Qfullboth (); }                                          // C 3181
  else if (*output).fullscreen_mode == FULLSCREEN_MODE_WIDTH           // C 3182
    { lval = Qfullwidth (); }                                         // C 3183
  else if (*output).fullscreen_mode == FULLSCREEN_MODE_HEIGHT          // C 3184
    { lval = Qfullheight (); }                                        // C 3185
  else if (*output).fullscreen_mode == FULLSCREEN_MODE_MAXIMIZED       // C 3186
    { lval = Qmaximized (); }                                         // C 3187
  else
    { lval = Qnil (); }                                               // C 3189

  store_frame_param (f, Qfullscreen (), lval);                        // C 3191
}

/* C 3194-3206.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_flush_dirty_back_buffer_on (f: *mut frame)
{
  if FRAME_GARBAGED_P (f)                                             // C 3197
      || buffer_flipping_blocked_p ()                                 // C 3198
      /* If the frame is not already up to date, do not flush buffers
	 on input, as that will result in flicker.  */                 // C 3199-3200
      || ! FRAME_COMPLETE_P! (f)                                      // C 3201
      || ! FRAME_DIRTY_P! (f)                                         // C 3202
    { return; }                                                       // C 3203

  haiku_flip_buffers (f);                                             // C 3205
}

/* C 3207-3240: the blank line C 3207, C's own comment block C 3208-3209 and
   `haiku_wait_for_event` (C 3211-3240) - the file's next definition after C 3171-3206.
   **Non-`static` in C** (`void haiku_wait_for_event (struct frame *f, int type)`), so the port
   carries `#[unsafe(no_mangle)] pub unsafe extern "C" fn` (PORTING.md 0.2); `haikuterm.h`'s
   declaration is the mirror's `haikuterm_decls` row (`include/haikuterm.rs` 485), which this
   definition shadows (PORTING.md 0.1-10/P-21).  This is the first item of the file to need Haiku
   SDK names of its own, and `<OS.h>` is not on this machine, so all four carry the standing
   `UNVERIFIED` mark (`PORTING.md` P-88 ①/②) - the `src/haiku_io.rs` 44-74 shape, which declares
   the same SDK's calls and the same `B_OK` for the same reason.  The C body's own conversions are
   C's: `interrupt_input_blocked` and `pending_signals` are `blockinput.rs`/`lisp.rs`, and C's
   `wait_for_objects (&info, 1) < B_OK` compares the `status_t` the SDK's call returns.  */

/* C: `<OS.h>`'s `struct object_wait_info`, the record C 3214 declares and C 3218-3220 fill in:
   `struct object_wait_info { int32 object; uint16 type; uint16 events; };` - the header has no
   mirror and is not on this machine, so the copy stands here with the SDK's own documented members
   (P-88 ①/②, the `src/haiku.rs` 55-61 scalar block's reading of `<kernel/OS.h>`).
   `int32` is `haiku_support.rs` 39's alias of the same header's scalar; C's `uint16` is 16 bits, so
   the two flag members take `u16` (PORTING.md 1.1).  */
#[cfg (feature = "have_haiku")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct object_wait_info
{
  pub object: int32,                    // offset 0
  pub r#type: u16,                      // offset 4
  pub events: u16,                      // offset 6
}
/// C: `status_t wait_for_objects (object_wait_info *infos, int numInfos);   [<OS.h>; UNVERIFIED
/// here]` - C 3229's call, and the reason the record above must exist.
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn wait_for_objects (infos: *mut object_wait_info, num_infos: c_int) -> status_t;
}
/// C: `#define B_EVENT_READ 0x0001` (`<OS.h>`; UNVERIFIED here - the value is the SDK's own
/// documented one, the mark `src/haiku_io.rs` 68-74 uses for `B_OK`/`B_TIMEOUT`).
#[cfg (feature = "have_haiku")]
pub const B_EVENT_READ: u16 = 0x0001;
/// C: `#define B_OBJECT_TYPE_PORT 2` (`<OS.h>`; UNVERIFIED here).
#[cfg (feature = "have_haiku")]
pub const B_OBJECT_TYPE_PORT: u16 = 2;
/// C: `#define B_OK 0` (`<OS.h>`; UNVERIFIED here - the same value and the same mark as
/// `src/haiku_io.rs` 68-70's copy, which this unit cannot reach: every ported unit is its own
/// translation unit).
#[cfg (feature = "have_haiku")]
pub const B_OK: status_t = 0;

/* N.B. that support for TYPE must be explicitly added to
   haiku_read_socket.  */                                             // C 3208-3209

/* C 3211-3240.  */
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_wait_for_event (f: *mut frame, r#type: c_int)
{
  let mut input_blocked_to: c_int;                                    // C 3213
  let mut info: object_wait_info = core::mem::zeroed ();              // C 3214 (PORTING.md 1.5: its
								      // address is handed to C 3229)
  let depth: specpdl_ref;                                             // C 3215

  input_blocked_to = interrupt_input_blocked;                         // C 3217
  info.object = port_application_to_emacs;                            // C 3218
  info.r#type = B_OBJECT_TYPE_PORT;                                   // C 3219
  info.events = B_EVENT_READ;                                         // C 3220

  depth = SPECPDL_INDEX ();                                           // C 3222
  specbind (Qinhibit_quit (), Qt ());                                 // C 3223

  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).wait_for_event_type = r#type;   // C 3225

  while (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).wait_for_event_type == r#type  // C 3227
    {
      if wait_for_objects (&raw mut info, 1) < B_OK                   // C 3229
	{ continue; }                                                 // C 3230

      pending_signals = true;                                         // C 3232
      /* This will call the read_socket_hook.  */                     // C 3233
      totally_unblock_input ();                                       // C 3234
      interrupt_input_blocked = input_blocked_to;                     // C 3235
      info.events = B_EVENT_READ;                                     // C 3236
    }

  unbind_to (depth, Qnil ());                                         // C 3239
}

/* C 3242-4212: `haiku_read_socket` (C 3242-4212, `static int`, 1,971 lines) - **the largest single
   item in this file**, and the one whose address C hands to `terminal->read_socket_hook`
   (C 4429), so the port defines it `unsafe extern "C" fn` (C's own function-pointer ABI; the
   slice-21 rule, which this item joins).  C's body is one `while` around a 27-case
   `switch (type)`; the port keeps C's shape with a `match` inside a labelled block, because C's
   two exits differ: `continue` leaves the *loop* (the port keeps the bare keyword, which in Rust
   names the enclosing `while` through a labelled block) and `break` leaves the *switch* (the port
   writes `break 'switch`, the `src/haikuselect.rs` 421-500 shape, whose note cites
   `src/keyboard.rs` 4632 as the first reading of it).  Three of C's cases fall through into the
   next (`BUTTON_UP` into `BUTTON_DOWN`'s body, `MENU_BAR_OPEN` into `MENU_BAR_CLOSE`, and
   `NOTIFICATION_CLICK_EVENT` into the `KEY_UP`/`DUMMY_EVENT` arm), which the port's `A | B =>`
   arms spell.  C 3257's `alloca (200)` is the port's stack pool (PORTING.md P-05's documented
   `alloca` -> `sa_stack_alloc` alias, `docs/ports/font.md` 315; the `src/msdos.rs` 4825 precedent
   for the same `void *buf = alloca (N)` shape).  */

/* termhooks.h 373-412's `enum event_kind kind : 16` shares `input_event`'s first word with
   `part : 16` (PORTING.md 1.3/P-80), and `include/termhooks.rs` carries no accessor for it
   (`EVENT_INIT` is what it does carry), so the unit brings its own - the `src/androidterm.rs`
   198-210 copy, body-identical, whose note names `androidselect.rs` 136-143 as the origin.
   C writes `inev.kind = X;` at twenty-four places in this item, and the read arm is what
   C 3383's `ev.code` test would need if a case ever read it back.  */
#[cfg (feature = "have_haiku")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
		 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/* frame.h 331-341's flag run: `f->new_size_p` is a `bool_bf : 1` member and C *writes* it in this
   item (no - it reads it at C 3329; the write is `dispnew.rs`'s).  `include/frame.rs` 1038/1353
   carry `FRAME_PIXEL_WIDTH`/`FRAME_COLUMN_WIDTH` as *functions* (read-only), and C assigns to
   `FRAME_PIXEL_WIDTH (f)` at C 3311-3312, so this unit brings the two-arm forms it needs, the
   same shape its own `FRAME_COLUMN_WIDTH!`/`FRAME_LINE_HEIGHT!` (C 707-712) already carry.  */
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_PIXEL_WIDTH {
  ($f:expr) => { (*$f).pixel_width };
  ($f:expr, $v:expr) => { (*$f).pixel_width = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_PIXEL_HEIGHT {
  ($f:expr) => { (*$f).pixel_height };
  ($f:expr, $v:expr) => { (*$f).pixel_height = $v };
}
/// frame.h 526 (`new_size_p`).  `frame.rs` defines this accessor for its own unit, which this
/// unit does not include, so the one this unit reads is repeated here - the `src/dispnew.rs`
/// 486-492 copy, whose `FRAME_NEW_SIZE_P_SHIFT` is the mirror's measured position.
#[cfg (feature = "have_haiku")]
macro_rules! new_size_p {
  ($f:expr) => { (*$f).bits2 >> FRAME_NEW_SIZE_P_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => {
    (*$f).bits2 = if $v { (*$f).bits2 | (1u32 << FRAME_NEW_SIZE_P_SHIFT) }
      else { (*$f).bits2 & !(1u32 << FRAME_NEW_SIZE_P_SHIFT) } };
}

/* C: dispextern.h 3924: `void change_frame_size (struct frame *, int, int, bool, bool, bool);`
   (C 3333's call; `src/dispnew.rs` 7512 defines it, and no mirror in this unit's graph declares
   it, so the unit declares it here - P-72 rule 4).  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn change_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
			    pretend: bool, delay: bool, safe: bool);
}

/* The nine prototypes this item calls that neither a mirror in this unit's graph nor the unit
   itself declares yet (P-72 rule 4, each with C's own header and line): dispextern.h 3571-3572's
   two scroll-bar setters (C 3919/3921), 3675's `expose_frame` (C 3349/3772), 3688's
   `note_mouse_highlight` (C 3565), 3690's `cancel_mouse_face` (C 3335), 3691's `clear_mouse_face`
   (C 3371/3474/3486/3557), 3668's `handle_tab_bar_click` and 3670's `handle_tool_bar_click`
   (C 3663/3683), and keyboard.h 513's `gen_help_event` (C 4198/4205).  **`remember_mouse_glyph`
   (C 3566) and `gui_mouse_grabbed` (C 3461/3515) are *not* declared here**: this unit's own
   preamble already carries both (dispextern.h 3586 at this file's line 4107 and 4006 at line 204),
   and a second copy would be `E0428` - measured in this round's first probe, which is how the
   duplicates were found.  `math.h`'s `powf`/`fabsf` (C 3961-3965) join the three the unit already
   declares at its head.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /// C: dispextern.h 3571: `void set_vertical_scroll_bar (struct window *);` (window.c)
  pub fn set_vertical_scroll_bar (w: *mut window);
  /// C: dispextern.h 3572: `void set_horizontal_scroll_bar (struct window *);` (window.c)
  pub fn set_horizontal_scroll_bar (w: *mut window);
  /// C: dispextern.h 3675: `extern void expose_frame (struct frame *, int, int, int, int);`
  pub fn expose_frame (f: *mut frame, x: c_int, y: c_int, w: c_int, h: c_int);
  /// C: dispextern.h 3688: `extern void note_mouse_highlight (struct frame *, int, int);`
  pub fn note_mouse_highlight (f: *mut frame, x: c_int, y: c_int);
  /// C: dispextern.h 3690: `extern void cancel_mouse_face (struct frame *);`
  pub fn cancel_mouse_face (f: *mut frame);
  /// C: dispextern.h 3691: `extern bool clear_mouse_face (Mouse_HLInfo *);`
  pub fn clear_mouse_face (hlinfo: *mut Mouse_HLInfo) -> bool;
  /// C: dispextern.h 3668: `extern Lisp_Object handle_tab_bar_click (struct frame *, int, int,
  /// bool, int);`
  pub fn handle_tab_bar_click (f: *mut frame, x: c_int, y: c_int, down_p: bool,
			       modifiers: c_int) -> Lisp_Object;
  /// C: dispextern.h 3670: `extern void handle_tool_bar_click (struct frame *, int, int, bool,
  /// int);`
  pub fn handle_tool_bar_click (f: *mut frame, x: c_int, y: c_int, down_p: bool,
				modifiers: c_int);
  /// C: keyboard.h 513-514: `extern void gen_help_event (Lisp_Object, Lisp_Object, Lisp_Object,
  /// Lisp_Object, ptrdiff_t);`
  pub fn gen_help_event (string: Lisp_Object, frame: Lisp_Object, window: Lisp_Object,
			 object: Lisp_Object, pos: ptrdiff_t);
  /// C: math.h: `float powf (float, float);`
  fn powf (x: f32, y: f32) -> f32;
  /// C: math.h: `float fabsf (float);`
  fn fabsf (x: f32) -> f32;
}

/* `Mouse_HLInfo`'s third flag bit (`dispextern.h` 3023's `bool_bf mouse_face_hidden : 1`, mask
   `include/dispextern.rs` 2139's `MOUSE_FACE_HIDDEN`).  The mirror carries the mask, the
   `mouse_face_defer` pair and no accessor for this bit, and C 3364/3372/3471/3473/3552/3556 read
   and write the member, so the unit names the pair - the `src/pgtkterm.rs` 7764-7773 copy, whose
   own note points at `src/msdos.rs` 3440-3445 for the same macro.  */
#[cfg (feature = "have_haiku")]
macro_rules! mouse_face_hidden {
  ($h:expr) => { (*$h).bits & MOUSE_FACE_HIDDEN != 0 };
  ($h:expr, $v:expr) => {
    (*$h).bits = if ($v) as c_int != 0 { (*$h).bits | MOUSE_FACE_HIDDEN }
		 else { (*$h).bits & ! MOUSE_FACE_HIDDEN } };
}

/* frame.h 1086-1087's `FRAME_MENU_BAR_LINES`/`FRAME_MENU_BAR_HEIGHT` are *functions* in the
   mirror (read-only) and C 4007-4009 assigns to both, so this unit brings the two write arms it
   needs - the same shape as its own `FRAME_PIXEL_WIDTH!` above.  (This citation is the round's
   one `check-citations` finding: the macros are at frame.h 1086-1087, not 1049-1069, which is
   `FRAME_EXTERNAL_MENU_BAR`'s block just above them - the gate caught the guessed pair.)  */
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_MENU_BAR_HEIGHT {
  ($f:expr) => { (*$f).menu_bar_height };
  ($f:expr, $v:expr) => { (*$f).menu_bar_height = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_MENU_BAR_LINES {
  ($f:expr) => { (*$f).menu_bar_lines };
  ($f:expr, $v:expr) => { (*$f).menu_bar_lines = $v };
}

/* C 4068's `find_and_call_menu_selection` (menu.h 41-42) and C 4171/4181's `system_time` - the
   latter is **not in the reference tree at all** (it is the Haiku SDK's own clock, in
   microseconds, which is why C divides by 1000), so it carries the standing `UNVERIFIED` mark
   the `src/haiku_io.rs` 44-74 block uses for the same header's calls and values.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /// C: src/menu.h 41-42: `extern void find_and_call_menu_selection (struct frame *, int,
  /// Lisp_Object, void *);` - `src/pgtkmenu.rs` 135-137 carries the same use-site copy.
  pub fn find_and_call_menu_selection (f: *mut frame, items: c_int, vector: Lisp_Object,
				       client_data: *mut c_void);
  /// C: `bigtime_t system_time (void);   [<OS.h>; UNVERIFIED here]`
  pub fn system_time () -> bigtime_t;
}

// C 3242-4212.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_read_socket (terminal: *mut terminal, hold_quit: *mut input_event)
					-> c_int
{
  let mut message_count: c_int;                                       // C 3245
  let mut buf: *mut c_void;                                           // C 3246
  let mut b_size: ssize_t = 0;                                       // C 3247 (PORTING.md
								      //  1.5: its
								      //  address is
								      //  handed to
								      //  C 3260)
  let mut button_or_motion_p: c_int;                                  // C 3248
  let mut do_help: c_int;
  let mut r#type: haiku_event_type = 0;                               // C 3249 (same rule: its
								      //  address is handed to C 3274)
  let mut inev: input_event = core::mem::zeroed ();                   // C 3250 (PORTING.md 1.5: both
  let mut inev2: input_event = core::mem::zeroed ();                  //  records are filled by C 3266)
  let mut mouse_frame: *mut frame;                                    // C 3251

  message_count = 0;                                                  // C 3253
  button_or_motion_p = 0;                                             // C 3254
  do_help = 0;                                                        // C 3255

  buf = sa_stack_alloc (200) as *mut c_void;                          // C 3257

  block_input ();                                                     // C 3259
  haiku_read_size (&raw mut b_size, false);                           // C 3260
  /* C 3261's `while (b_size >= 0)`: the loop carries a label because the `switch` below is the
     port's labelled block, and a `continue` that jumps out of a labelled block must name its
     target (measured: the first probe of this round reported `E0695 unlabeled \`continue\`
     inside of a labeled block` at every one of the twenty-nine sites - the `src/image.rs` 6735
     `continue 'loop_` and `src/xterm.rs` 11024 `continue 'start` shape).  */
  'read_socket: while b_size >= 0                                       // C 3261
    {
      if b_size > 200                                                  // C 3263
	{ emacs_abort (); }                                            // C 3264

      EVENT_INIT (&raw mut inev);                                      // C 3266
      EVENT_INIT (&raw mut inev2);                                     // C 3267
      input_event_bits! (kind, &raw mut inev, NO_EVENT);               // C 3268
      input_event_bits! (kind, &raw mut inev2, NO_EVENT);              // C 3269
      inev.arg = Qnil ();                                              // C 3270
      inev2.arg = Qnil ();                                             // C 3271

      button_or_motion_p = 0;                                          // C 3273
      haiku_read (&raw mut r#type, buf, b_size);                       // C 3274

      /* C 3276-4211: the 27-case `switch (type)` - `break` leaves the switch (the labelled
	 block) and `continue` leaves the `while` (C's own two exits, kept as the two Rust
	 keywords).  */
      'switch: {
      match r#type
	{
	  QUIT_REQUESTED =>                                            // C 3278
	    {
	      let b: *mut haiku_quit_requested_event = buf as *mut haiku_quit_requested_event;  // C 3280
	      let f: *mut frame = haiku_window_to_frame ((*b).window); // C 3281

	      if f.is_null ()                                          // C 3283
		{ continue 'read_socket; }                                          // C 3284

	      input_event_bits! (kind, &raw mut inev, DELETE_WINDOW_EVENT);  // C 3286
	      XSETFRAME! (inev.frame_or_window, f);                    // C 3287
	      break 'switch;                                           // C 3288
	    }
	  FRAME_RESIZED =>                                             // C 3290
	    {
	      let b: *mut haiku_resize_event = buf as *mut haiku_resize_event;  // C 3292
	      let f: *mut frame = haiku_window_to_frame ((*b).window); // C 3293

	      if f.is_null ()                                          // C 3295
		{ continue 'read_socket; }                                          // C 3296

	      let width: c_int = lrint ((*b).width as c_double) as c_int;    // C 3298
	      let height: c_int = lrint ((*b).height as c_double) as c_int;  // C 3299

	      if (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).wait_for_event_type  // C 3301-3302
		  == FRAME_RESIZED
		{ (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).wait_for_event_type = -1; }  // C 3303

	      if FRAME_TOOLTIP_P (f)                                   // C 3305
		{
		  if FRAME_PIXEL_WIDTH! (f) != width                   // C 3307
		      || FRAME_PIXEL_HEIGHT! (f) != height             // C 3308
		    { SET_FRAME_GARBAGED! (f); }                       // C 3309

		  FRAME_PIXEL_WIDTH! (f, width);                       // C 3311
		  FRAME_PIXEL_HEIGHT! (f, height);                     // C 3312

		  haiku_clear_under_internal_border (f);               // C 3314

		  /* Flush the frame and flip buffers here.  It is
		     necessary for tooltips displayed inside menus, as
		     redisplay cannot happen.  */                     // C 3316-3318
		  haiku_flush (f);                                     // C 3319
		  continue 'read_socket;                                            // C 3320
		}

	      BView_draw_lock (FRAME_HAIKU_DRAWABLE! (f), false, 0, 0, 0, 0);   // C 3323
	      BView_resize_to (FRAME_HAIKU_DRAWABLE! (f), width, height);      // C 3324
	      BView_draw_unlock (FRAME_HAIKU_DRAWABLE! (f));                   // C 3325

	      if width != FRAME_PIXEL_WIDTH! (f)                       // C 3327
		  || height != FRAME_PIXEL_HEIGHT! (f)                 // C 3328
		  || (new_size_p! (f)                                  // C 3329
		      && (((*f).new_width >= 0 && width != (*f).new_width)     // C 3330
			  || ((*f).new_height >= 0 && height != (*f).new_height)))   // C 3331
		{
		  change_frame_size (f, width, height, false, true, false);    // C 3333
		  SET_FRAME_GARBAGED! (f);                             // C 3334
		  cancel_mouse_face (f);                               // C 3335
		  haiku_clear_under_internal_border (f);               // C 3336
		}

	      break 'switch;                                           // C 3339
	    }
	  FRAME_EXPOSED =>                                             // C 3341
	    {
	      let b: *mut haiku_expose_event = buf as *mut haiku_expose_event;  // C 3343
	      let f: *mut frame = haiku_window_to_frame ((*b).window); // C 3344

	      if f.is_null ()                                          // C 3346
		{ continue 'read_socket; }                                          // C 3347

	      expose_frame (f, (*b).x, (*b).y, (*b).width, (*b).height);      // C 3349
	      haiku_clear_under_internal_border (f);                   // C 3350
	      break 'switch;                                           // C 3351
	    }
	  KEY_DOWN =>                                                  // C 3353
	    {
	      let b: *mut haiku_key_event = buf as *mut haiku_key_event;      // C 3355
	      let hlinfo: *mut Mouse_HLInfo
		= &raw mut (*(x_display_list)).mouse_highlight;        // C 3356
	      let f: *mut frame = haiku_window_to_frame ((*b).window); // C 3357

	      if f.is_null ()                                          // C 3359
		{ continue 'read_socket; }                                          // C 3360

	      /* If mouse-highlight is an integer, input clears out
		 mouse highlighting.  */                           // C 3362-3363
	      if ! mouse_face_hidden! (hlinfo) && FIXNUMP (Vmouse_highlight)   // C 3364
		  && (f.is_null ()                                     // C 3365
		      || (! EQ ((*f).tool_bar_window, (*hlinfo).mouse_face_window)  // C 3366
			  && ! EQ ((*f).tab_bar_window, (*hlinfo).mouse_face_window)))  // C 3367
		{
		  mouse_frame = (*hlinfo).mouse_face_mouse_frame           // C 3369
		    as *mut frame;

		  clear_mouse_face (hlinfo);                           // C 3371
		  mouse_face_hidden! (hlinfo, true);                    // C 3372

		  if ! mouse_frame.is_null ()                          // C 3374
		    { haiku_flush_dirty_back_buffer_on (mouse_frame); } // C 3375
		}

	      inev.code = if (*b).keysym != 0 { (*b).keysym } else { (*b).multibyte_char };  // C 3378

	      if (*b).keysym != 0                                      // C 3380
		{ input_event_bits! (kind, &raw mut inev, NON_ASCII_KEYSTROKE_EVENT); }  // C 3381
	      else
		{ input_event_bits! (kind, &raw mut inev,                   // C 3383-3384
				     if inev.code > 127
				       { MULTIBYTE_CHAR_KEYSTROKE_EVENT }
				     else { ASCII_KEYSTROKE_EVENT }); }

	      inev.timestamp = ((*b).time / 1000) as Time;             // C 3386
	      inev.modifiers = (haiku_modifiers_to_emacs ((*b).modifiers) as EMACS_INT   // C 3387
				| (extra_keyboard_modifiers                  // C 3388
				   & (meta_modifier                           // C 3389
				      | hyper_modifier                        // C 3390
				      | ctrl_modifier                         // C 3391
				      | alt_modifier                          // C 3392
				      | shift_modifier) as EMACS_INT)) as c_uint;  // C 3393

	      XSETFRAME! (inev.frame_or_window, f);                    // C 3395
	      break 'switch;                                           // C 3396
	    }
	  ACTIVATION =>                                                // C 3398
	    {
	      let b: *mut haiku_activation_event = buf as *mut haiku_activation_event;  // C 3400
	      let f: *mut frame = haiku_window_to_frame ((*b).window); // C 3401

	      if f.is_null ()                                          // C 3403
		{ continue 'read_socket; }                                          // C 3404

	      if (((*x_display_list).focus_event_frame != f && (*b).activated_p != 0)      // C 3406
		  || ((*x_display_list).focus_event_frame == f && (*b).activated_p == 0))     // C 3407
		{
		  haiku_new_focus_frame (if (*b).activated_p != 0 { f } else { std::ptr::null_mut () });  // C 3409
		  if (*b).activated_p != 0                             // C 3410
		    { (*x_display_list).focus_event_frame = f; }       // C 3411
		  else
		    { (*x_display_list).focus_event_frame = std::ptr::null_mut (); }   // C 3413
		  input_event_bits! (kind, &raw mut inev,              // C 3414
				     if (*b).activated_p != 0 { FOCUS_IN_EVENT }
				     else { FOCUS_OUT_EVENT });
		  XSETFRAME! (inev.frame_or_window, f);                // C 3415
		}

	      break 'switch;                                           // C 3418
	    }
	  MENU_BAR_LEFT =>                                             // C 3420
	    {
	      let b: *mut haiku_menu_bar_left_event = buf as *mut haiku_menu_bar_left_event;  // C 3422
	      let f: *mut frame = haiku_window_to_frame ((*b).window); // C 3423

	      if f.is_null ()                                          // C 3425
		{ continue 'read_socket; }                                          // C 3426

	      if (*b).y > 0 && (*b).y <= FRAME_PIXEL_HEIGHT! (f)       // C 3428
		  && (*b).x > 0 && (*b).x <= FRAME_PIXEL_WIDTH! (f)    // C 3429
		{ break 'switch; }                                     // C 3430

	      if auto_lower! (f) && popup_activated_p == 0             // C 3432
		{ haiku_frame_raise_lower (f, false); }                // C 3433

	      break 'switch;                                           // C 3435
	    }
	  MOUSE_MOTION =>                                              // C 3437
	    {
	      let b: *mut haiku_mouse_motion_event = buf as *mut haiku_mouse_motion_event;  // C 3439
	      let f: *mut frame = haiku_mouse_or_wdesc_frame ((*b).window, true);  // C 3440
	      let hlinfo: *mut Mouse_HLInfo
		= &raw mut (*(x_display_list)).mouse_highlight;        // C 3441
	      let mut frame: Lisp_Object;                              // C 3442

	      if f.is_null ()                                          // C 3444
		{ continue 'read_socket; }                                          // C 3445

	      if FRAME_TOOLTIP_P (f)                                   // C 3447
		{
		  /* Dismiss the tooltip if the mouse moves onto a
		     tooltip frame (except when drag-and-drop is in
		     progress and we are trying to move the tooltip
		     along with the mouse pointer).  FIXME: for some
		     reason we don't get leave notification events for
		     this.  */                                        // C 3449-3454

		  if any_help_event_p                                    // C 3456
		      && ! (be_drag_and_drop_in_progress ()              // C 3457
			    && haiku_dnd_follow_tooltip)                 // C 3458
		      && ! ((EQ (track_mouse, Qdrag_source ())           // C 3459
			      || EQ (track_mouse, Qdropping ()))         // C 3460
			     && gui_mouse_grabbed (x_display_list))      // C 3461
		    { do_help = -1; }                                  // C 3462
		  break 'switch;                                       // C 3463
		}

	      XSETFRAME! (frame, f);                                   // C 3466

	      (*x_display_list).last_mouse_movement_time = ((*b).time / 1000) as Time;  // C 3468
	      button_or_motion_p = 1;                                  // C 3469

	      if mouse_face_hidden! (hlinfo)                           // C 3471
		{
		  mouse_face_hidden! (hlinfo, false);                   // C 3473
		  clear_mouse_face (hlinfo);                            // C 3474
		  haiku_flush_dirty_back_buffer_on (f);                 // C 3475
		}

	      if (*b).just_exited_p                                    // C 3478
		{
		  let hlinfo: *mut Mouse_HLInfo = MOUSE_HL_INFO! (f);   // C 3480

		  if f as *mut c_void == (*hlinfo).mouse_face_mouse_frame   // C 3482
		    {
		      /* If we move outside the frame, then we're
			 certainly no longer on any text in the frame.  */  // C 3484-3485
		      clear_mouse_face (hlinfo);                        // C 3486
		      (*hlinfo).mouse_face_mouse_frame = std::ptr::null_mut ();  // C 3487

		      haiku_flush_dirty_back_buffer_on (f);             // C 3489
		    }

		  if (*x_display_list).last_mouse_glyph_frame == f     // C 3492
		    { (*x_display_list).last_mouse_glyph_frame = std::ptr::null_mut (); }  // C 3493

		  if auto_lower! (f) && popup_activated_p == 0          // C 3495
		      /* Don't do this if the mouse entered a scroll bar.  */   // C 3496
		      && ! BView_inside_scroll_bar (FRAME_HAIKU_VIEW! (f), // C 3497
						    (*b).x, (*b).y)    // C 3498
		    {
		      /* If we're leaving towards the menu bar, don't
			 auto-lower here, and wait for a exit
			 notification from the menu bar instead.  */   // C 3500-3502
		      if (*b).x > FRAME_PIXEL_WIDTH! (f)                // C 3503
			  || (*b).y >= FRAME_MENU_BAR_HEIGHT (f)        // C 3504
			  || (*b).x < 0                                 // C 3505
			  || (*b).y < 0                                 // C 3506
			{ haiku_frame_raise_lower (f, false); }         // C 3507
		    }

		  haiku_new_focus_frame ((*x_display_list).focused_frame);  // C 3510

		  if any_help_event_p                                   // C 3512
		      && ! ((EQ (track_mouse, Qdrag_source ())           // C 3513
			      || EQ (track_mouse, Qdropping ()))         // C 3514
			     && gui_mouse_grabbed (x_display_list))      // C 3515
		    { do_help = -1; }                                   // C 3516
		}
	      else
		{
		  let dpyinfo: *mut haiku_display_info = FRAME_DISPLAY_INFO! (f);  // C 3520
		  let r: haiku_rect = (*dpyinfo).last_mouse_glyph;      // C 3521

		  /* For an unknown reason Haiku sends phantom motion events when a
		     tooltip frame is visible.  FIXME */               // C 3523-3524
		  if FRAMEP (tip_frame)                                 // C 3525
		      && FRAME_LIVE_P (XFRAME (tip_frame))              // C 3526
		      && FRAME_VISIBLE_P (XFRAME (tip_frame))           // C 3527
		      && f == (*dpyinfo).last_mouse_motion_frame        // C 3528
		      && (*b).x == (*dpyinfo).last_mouse_motion_x       // C 3529
		      && (*b).y == (*dpyinfo).last_mouse_motion_y       // C 3530
		    { continue 'read_socket; }                                      // C 3531

		  (*dpyinfo).last_mouse_motion_x = (*b).x;              // C 3533
		  (*dpyinfo).last_mouse_motion_y = (*b).y;              // C 3534
		  (*dpyinfo).last_mouse_motion_frame = f;               // C 3535

		  dispextern_unit_names::previous_help_echo_string = dispextern_unit_names::help_echo_string;         // C 3537
		  dispextern_unit_names::help_echo_string = Qnil ();                           // C 3538

		  /* A crossing event might be sent out-of-order with
		     regard to motion events from other windows, such as
		     when the mouse pointer rapidly moves from an
		     undecorated child frame to its parent.  This can
		     cause a failure to clear the mouse face on the
		     former if an event for the latter is read by Emacs
		     first and ends up showing the mouse face there.

		     Work around the problem by clearing the mouse face
		     now if it is currently shown on a different
		     frame.  */                                        // C 3540-3550

		  if mouse_face_hidden! (hlinfo)                         // C 3552
		      || (f as *mut c_void != (*hlinfo).mouse_face_mouse_frame   // C 3553
			  && ! NILP ((*hlinfo).mouse_face_window))      // C 3554
		    {
		      mouse_face_hidden! (hlinfo, false);                // C 3556
		      clear_mouse_face (hlinfo);                        // C 3557
		    }

		  if (*dpyinfo).last_mouse_glyph_frame != f             // C 3560
		      || (*b).x < r.x || (*b).x >= r.x + r.width        // C 3561
		      || (*b).y < r.y || (*b).y >= r.y + r.height       // C 3562
		    {
		      mouse_moved! (f, true);                           // C 3564
		      note_mouse_highlight (f, (*b).x, (*b).y);         // C 3565
		      remember_mouse_glyph (f, (*b).x, (*b).y,          // C 3566
					    &raw mut (*(FRAME_DISPLAY_INFO! (f))).last_mouse_glyph);  // C 3567
		      (*dpyinfo).last_mouse_glyph_frame = f;            // C 3568
		    }
		  else
		    { dispextern_unit_names::help_echo_string = dispextern_unit_names::previous_help_echo_string; }   // C 3571

		  if ! NILP (Vmouse_autoselect_window)                  // C 3573
		    {
		      let window: Lisp_Object
			= window_from_coordinates (f as *mut c_void, (*b).x, (*b).y,   // C 3575
					   std::ptr::null_mut (), false, false, false);

		      if WINDOWP (window)                               // C 3577
			  && ! EQ (window, last_mouse_window)           // C 3578
			  && ! EQ (window, selected_window)             // C 3579
			  && popup_activated_p == 0                     // C 3580
			  && ! MINI_WINDOW_P (XWINDOW (selected_window))  // C 3581
			  && (! NILP (focus_follows_mouse)              // C 3582
			      || f == SELECTED_FRAME ())                // C 3583
			{
			  input_event_bits! (kind, &raw mut inev2, SELECT_WINDOW_EVENT);  // C 3585
			  inev2.frame_or_window = window;               // C 3586
			}

		      last_mouse_window = window;                       // C 3589
		    }

		  if auto_raise! (f)                                    // C 3592
		    {
		      if ! BWindow_is_active (FRAME_HAIKU_WINDOW! (f))  // C 3594
			{ haiku_frame_raise_lower (f, true); }          // C 3595
		    }

		  if ! NILP (dispextern_unit_names::help_echo_string)                          // C 3598
		      || ! NILP (dispextern_unit_names::previous_help_echo_string)             // C 3599
		    { do_help = 1; }                                    // C 3600

		  if (*b).dnd_message                                   // C 3602
		    {
		      /* It doesn't make sense to show tooltips when
			 another program is dragging stuff over us.  */   // C 3604-3605

		      if any_help_event_p || do_help != 0               // C 3607
			{ do_help = -1; }                               // C 3608

		      if ! be_drag_and_drop_in_progress ()              // C 3610
			{
			  input_event_bits! (kind, &raw mut inev, DRAG_N_DROP_EVENT);  // C 3612
			  inev.arg = Qlambda ();                        // C 3613

			  XSETINT! (inev.x, (*b).x as isize);           // C 3615
			  XSETINT! (inev.y, (*b).y as isize);           // C 3616
			  XSETFRAME! (inev.frame_or_window, f);         // C 3617
			}
		      else
			{ haiku_note_drag_motion (); }                  // C 3620

		      break 'switch;                                    // C 3622
		    }
		}

	      if FRAME_DIRTY_P! (f)                                     // C 3626
		{ haiku_flush_dirty_back_buffer_on (f); }               // C 3627
	      break 'switch;                                            // C 3628
	    }
	  BUTTON_UP | BUTTON_DOWN =>                                    // C 3630-3631
	    {
	      let b: *mut haiku_button_event = buf as *mut haiku_button_event;  // C 3633
	      let f: *mut frame = haiku_mouse_or_wdesc_frame ((*b).window, false);  // C 3634
	      let mut tab_bar_arg: Lisp_Object = Qnil ();               // C 3635 (PORTING.md 1.5: its
									//  address is not taken; C's own
									//  initialiser is the port's)
	      let mut tab_bar_p: c_int = 0;                             // C 3636
	      let mut tool_bar_p: c_int = 0;
	      let mut up_okay_p: bool = false;                          // C 3637
	      let bar: *mut scroll_bar;                                 // C 3638

	      if popup_activated_p != 0 || f.is_null ()                 // C 3640
		{ continue 'read_socket; }                                           // C 3641

	      inev.modifiers = haiku_modifiers_to_emacs ((*b).modifiers) as c_uint;  // C 3643
	      bar = haiku_scroll_bar_from_widget ((*b).scroll_bar, (*b).window);  // C 3644

	      (*x_display_list).last_mouse_glyph_frame = std::ptr::null_mut ();  // C 3646
	      (*x_display_list).last_mouse_movement_time = ((*b).time / 1000) as Time;  // C 3647
	      button_or_motion_p = 1;                                   // C 3648

	      /* Is this in the tab-bar?  */                            // C 3650
	      if WINDOWP ((*f).tab_bar_window)                          // C 3651
		  && WINDOW_TOTAL_LINES (XWINDOW ((*f).tab_bar_window)) != 0    // C 3652
		{
		  let mut window: Lisp_Object;                          // C 3654
		  let x: c_int = (*b).x;                                // C 3655
		  let y: c_int = (*b).y;                                // C 3656

		  window = window_from_coordinates (f as *mut c_void, x, y, std::ptr::null_mut (),   // C 3658
						    true, true, true);
		  tab_bar_p = EQ (window, (*f).tab_bar_window) as c_int;   // C 3659

		  if tab_bar_p != 0                                     // C 3661
		    {
		      tab_bar_arg = handle_tab_bar_click                // C 3663-3664
			(f, x, y, r#type == BUTTON_DOWN, inev.modifiers as c_int);
		      haiku_flush_dirty_back_buffer_on (f);             // C 3665
		    }
		}

	      if WINDOWP ((*f).tool_bar_window)                         // C 3669
		  && WINDOW_TOTAL_LINES (XWINDOW ((*f).tool_bar_window)) != 0    // C 3670
		{
		  let mut window: Lisp_Object;                          // C 3672
		  let x: c_int = (*b).x;                                // C 3673
		  let y: c_int = (*b).y;                                // C 3674

		  window = window_from_coordinates (f as *mut c_void, x, y, std::ptr::null_mut (),   // C 3676
						    true, true, true);
		  tool_bar_p = (EQ (window, (*f).tool_bar_window)        // C 3677
				&& (r#type != BUTTON_UP                  // C 3678
				    || (*f).last_tool_bar_item != -1)) as c_int;   // C 3679

		  if tool_bar_p != 0                                    // C 3681
		    {
		      handle_tool_bar_click                             // C 3683-3684
			(f, x, y, r#type == BUTTON_DOWN, inev.modifiers as c_int);
		      haiku_flush_dirty_back_buffer_on (f);             // C 3685
		    }
		}

	      if r#type == BUTTON_UP                                  // C 3689
		{
		  inev.modifiers |= up_modifier as c_uint;              // C 3691
		  up_okay_p = ((*x_display_list).grabbed & (1 << (*b).btn_no)) != 0;  // C 3692
		  (*x_display_list).grabbed &= ! (1 << (*b).btn_no);     // C 3693
		}
	      else
		{
		  up_okay_p = true;                                     // C 3697
		  inev.modifiers |= down_modifier as c_uint;            // C 3698
		  (*x_display_list).last_mouse_frame = f;               // C 3699
		  (*x_display_list).grabbed |= (1 << (*b).btn_no);       // C 3700
		  if ! f.is_null () && tab_bar_p == 0                   // C 3701
		    { (*f).last_tab_bar_item = -1; }                    // C 3702
		  if ! f.is_null () && tool_bar_p == 0                  // C 3703
		    { (*f).last_tool_bar_item = -1; }                   // C 3704
		}

	      if ! bar.is_null ()                                       // C 3707
		{
		  input_event_bits! (kind, &raw mut inev,                // C 3709-3711
				     if ((*bar).horizontal)
				       { HORIZONTAL_SCROLL_BAR_CLICK_EVENT }
				     else { SCROLL_BAR_CLICK_EVENT });
		  input_event_bits! (part, &raw mut inev,                // C 3712-3714
				     if ((*bar).horizontal)
				       { scroll_bar_horizontal_handle }
				     else { scroll_bar_handle });
		}
	      else if up_okay_p                                        // C 3716
		       && ! (tab_bar_p != 0 && NILP (tab_bar_arg))      // C 3717
		       && tool_bar_p == 0                               // C 3718
		{ input_event_bits! (kind, &raw mut inev, MOUSE_CLICK_EVENT); }  // C 3719

	      inev.arg = tab_bar_arg;                                   // C 3721
	      inev.code = (*b).btn_no as c_uint;                        // C 3722

	      mouse_moved! (f, false);                                  // C 3724

	      if ! bar.is_null ()                                       // C 3726
		{
		  if (*bar).horizontal                             // C 3728
		    {
		      XSETINT! (inev.x, std::cmp::min (std::cmp::max (0, (*b).x - (*bar).left),   // C 3730
						       (*bar).width) as isize);   // C 3731
		      XSETINT! (inev.y, (*bar).width as isize);         // C 3732
		    }
		  else
		    {
		      XSETINT! (inev.x, std::cmp::min (std::cmp::max (0, (*b).y - (*bar).top),    // C 3736
						       (*bar).height) as isize);  // C 3737
		      XSETINT! (inev.y, (*bar).height as isize);        // C 3738
		    }

		  inev.frame_or_window = (*bar).window;                 // C 3741
		}
	      else
		{
		  XSETINT! (inev.x, (*b).x as isize);                   // C 3745
		  XSETINT! (inev.y, (*b).y as isize);                   // C 3746
		  XSETFRAME! (inev.frame_or_window, f);                 // C 3747
		}

	      break 'switch;                                            // C 3750
	    }
	  ICONIFICATION =>                                             // C 3752
	    {
	      let b: *mut haiku_iconification_event = buf as *mut haiku_iconification_event;  // C 3754
	      let f: *mut frame = haiku_window_to_frame ((*b).window);  // C 3755

	      if f.is_null ()                                          // C 3757
		{ continue 'read_socket; }                                          // C 3758

	      if (*b).iconified_p == 0                                 // C 3760
		{
		  SET_FRAME_VISIBLE! (f, 1);                           // C 3762
		  SET_FRAME_ICONIFIED (f, 0);                          // C 3763
		  input_event_bits! (kind, &raw mut inev, DEICONIFY_EVENT);  // C 3764

		  /* Haiku doesn't expose frames on deiconification, but
		     if we are double-buffered, the previous screen
		     contents should have been preserved. */           // C 3766-3768
		  if EmacsView_double_buffered_p (FRAME_HAIKU_VIEW! (f)) == 0   // C 3769
		    {
		      SET_FRAME_GARBAGED! (f);                         // C 3771
		      expose_frame (f, 0, 0, 0, 0);                    // C 3772
		    }
		}
	      else
		{
		  SET_FRAME_VISIBLE! (f, 0);                           // C 3777
		  SET_FRAME_ICONIFIED (f, 1);                          // C 3778
		  input_event_bits! (kind, &raw mut inev, ICONIFY_EVENT);  // C 3779
		}

	      XSETFRAME! (inev.frame_or_window, f);                    // C 3782
	      break 'switch;                                           // C 3783
	    }
	  MOVE_EVENT =>                                                // C 3785
	    {
	      let b: *mut haiku_move_event = buf as *mut haiku_move_event;  // C 3787
	      let f: *mut frame = haiku_window_to_frame ((*b).window);  // C 3788
	      let mut top: c_int;                                      // C 3789
	      let mut left: c_int;
	      let mut p: *mut frame;                                   // C 3790

	      if f.is_null ()                                          // C 3792
		{ continue 'read_socket; }                                          // C 3793

	      (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).frame_x = (*b).x;  // C 3795
	      (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).frame_y = (*b).y;  // C 3796

	      if ! FRAME_PARENT_FRAME (f).is_null ()                   // C 3798
		{ haiku_coords_from_parent (f, &raw mut (*b).x, &raw mut (*b).y); }  // C 3799

	      left = (*b).x - (*b).decorator_width;                    // C 3801
	      top = (*b).y - (*b).decorator_height;                    // C 3802

	      if left != (*f).left_pos || top != (*f).top_pos          // C 3804
		{
		  input_event_bits! (kind, &raw mut inev, MOVE_FRAME_EVENT);  // C 3806

		  XSETINT! (inev.x, left as isize);                    // C 3808
		  XSETINT! (inev.y, top as isize);                     // C 3809

		  (*f).left_pos = left;                                // C 3811
		  (*f).top_pos = top;                                  // C 3812

		  p = FRAME_PARENT_FRAME (f);                          // C 3814

		  if ! p.is_null ()                                    // C 3816
		    { EmacsWindow_move_weak_child (FRAME_HAIKU_WINDOW! (p),  // C 3817
						   (*b).window, left, top); }  // C 3818

		  XSETFRAME! (inev.frame_or_window, f);                // C 3820
		}

	      haiku_make_fullscreen_consistent (f);                    // C 3823
	      break 'switch;                                           // C 3824
	    }
	  SCROLL_BAR_VALUE_EVENT =>                                    // C 3826
	    {
	      let b: *mut haiku_scroll_bar_value_event
		= buf as *mut haiku_scroll_bar_value_event;            // C 3828
	      let bar: *mut scroll_bar                              // C 3829-3830
		= haiku_scroll_bar_from_widget ((*b).scroll_bar, (*b).window);
	      let mut portion: c_int;                                 // C 3831
	      let mut whole: c_int;

	      if bar.is_null ()                                        // C 3833
		{ continue 'read_socket; }                                          // C 3834

	      let w: *mut window = XWINDOW ((*bar).window);            // C 3836

	      if (*bar).update != -1                                   // C 3838
		{
		  (*bar).update = -1;                                 // C 3840
		  break 'switch;                                       // C 3841
		}

	      if (*bar).position != (*b).position                      // C 3844
		{
		  input_event_bits! (kind, &raw mut inev,              // C 3846-3848
				     if (*bar).horizontal
				       { HORIZONTAL_SCROLL_BAR_CLICK_EVENT }
				     else { SCROLL_BAR_CLICK_EVENT });
		  input_event_bits! (part, &raw mut inev,              // C 3849-3851
				     if (*bar).horizontal
				       { scroll_bar_horizontal_handle }
				     else { scroll_bar_handle });

		  if (*bar).horizontal                            // C 3853
		    {
		      portion = ((*bar).total as f32                   // C 3855-3856
				 * ((*b).position as f32 / BE_SB_MAX as f32)) as c_int;
		      whole = ((*bar).total as f32                     // C 3857-3859
			       * ((BE_SB_MAX - (*bar).page_size) as f32 / BE_SB_MAX as f32)) as c_int;
		      portion = std::cmp::min (portion, whole);        // C 3860
		    }
		  else
		    {
		      whole = BE_SB_MAX - (*bar).page_size;            // C 3864
		      portion = std::cmp::min ((*b).position, whole);  // C 3865
		    }

		  XSETINT! (inev.x, portion as isize);                 // C 3868
		  XSETINT! (inev.y, whole as isize);                   // C 3869
		  XSETWINDOW! (inev.frame_or_window, w);               // C 3870
		}
	      break 'switch;                                           // C 3872
	    }
	  SCROLL_BAR_PART_EVENT =>                                     // C 3874
	    {
	      let b: *mut haiku_scroll_bar_part_event
		= buf as *mut haiku_scroll_bar_part_event;             // C 3876
	      let bar: *mut scroll_bar                              // C 3877-3878
		= haiku_scroll_bar_from_widget ((*b).scroll_bar, (*b).window);

	      if bar.is_null ()                                        // C 3880
		{ continue 'read_socket; }                                          // C 3881

	      input_event_bits! (kind, &raw mut inev,                  // C 3883-3884
				 if (*bar).horizontal
				   { HORIZONTAL_SCROLL_BAR_CLICK_EVENT }
				 else { SCROLL_BAR_CLICK_EVENT });

	      (*bar).dragging = 0;                                     // C 3886

	      /* C 3888-3900's inner `switch (b->part)`: its two `break`s leave the inner switch,
		 which is what a `match` arm already does - no label is needed, and the outer
		 `break 'switch` below is C 3906's.  */
	      match (*b).part
		{
		  HAIKU_SCROLL_BAR_UP_BUTTON =>                        // C 3890
		    {
		      input_event_bits! (part, &raw mut inev,          // C 3891-3893
					 if (*bar).horizontal
					   { scroll_bar_left_arrow }
					 else { scroll_bar_up_arrow });
		    }
		  HAIKU_SCROLL_BAR_DOWN_BUTTON =>                      // C 3895
		    {
		      input_event_bits! (part, &raw mut inev,          // C 3896-3898
					 if (*bar).horizontal
					   { scroll_bar_right_arrow }
					 else { scroll_bar_down_arrow });
		    }
		  _ => {}
		}

	      XSETINT! (inev.x, 0);                                    // C 3902
	      XSETINT! (inev.y, 0);                                    // C 3903
	      inev.frame_or_window = (*bar).window;                    // C 3904

	      break 'switch;                                           // C 3906
	    }
	  SCROLL_BAR_DRAG_EVENT =>                                     // C 3908
	    {
	      let b: *mut haiku_scroll_bar_drag_event
		= buf as *mut haiku_scroll_bar_drag_event;             // C 3910
	      let bar: *mut scroll_bar                              // C 3911-3912
		= haiku_scroll_bar_from_widget ((*b).scroll_bar, (*b).window);

	      if bar.is_null ()                                        // C 3914
		{ continue 'read_socket; }                                          // C 3915

	      (*bar).dragging = (*b).dragging_p;                       // C 3917
	      if (*b).dragging_p == 0 && (*bar).horizontal        // C 3918
		{ set_horizontal_scroll_bar (XWINDOW ((*bar).window)); }  // C 3919
	      else if (*b).dragging_p == 0                             // C 3920
		{ set_vertical_scroll_bar (XWINDOW ((*bar).window)); }  // C 3921
	      break 'switch;                                           // C 3922
	    }
	  WHEEL_MOVE_EVENT =>                                          // C 3924
	    {
	      let b: *mut haiku_wheel_move_event = buf as *mut haiku_wheel_move_event;  // C 3926
	      let f: *mut frame = haiku_window_to_frame ((*b).window);  // C 3927
	      let mut x: c_int = 0;                                    // C 3928 (PORTING.md 1.5: its
							      //  address is handed to C 3935)
	      let mut y: c_int = 0;
	      let mut scroll_width: c_int;
	      let mut scroll_height: c_int;
	      let mut wheel_window: Lisp_Object;                       // C 3930
	      /* C 3929's function-local `static float px = 0.0f, py = 0.0f;` - the two
		 accumulators persist across calls exactly as C's do (the unit's own
		 function-local `static mut` shape, `src/haikuterm.rs` 449).  */
	      static mut px: f32 = 0.0f32;                             // C 3929
	      static mut py: f32 = 0.0f32;

	      if f.is_null ()                                          // C 3932
		{ continue 'read_socket; }                                          // C 3933

	      BView_get_mouse (FRAME_HAIKU_VIEW! (f), &raw mut x, &raw mut y);  // C 3935

	      wheel_window = window_from_coordinates (f as *mut c_void, x, y, std::ptr::null_mut (),  // C 3937
						      false, false, false);

	      if NILP (wheel_window)                                   // C 3939
		{
		  scroll_width = FRAME_PIXEL_WIDTH! (f);                 // C 3941
		  scroll_height = FRAME_PIXEL_HEIGHT! (f);               // C 3942
		}
	      else
		{
		  scroll_width = (*XWINDOW (wheel_window)).pixel_width;   // C 3946
		  scroll_height = (*XWINDOW (wheel_window)).pixel_height; // C 3947
		}

	      inev.modifiers = haiku_modifiers_to_emacs ((*b).modifiers) as c_uint;  // C 3950

	      inev2.modifiers = inev.modifiers;                        // C 3952

	      if signbit! (px) != signbit! ((*b).delta_x)              // C 3954
		{ px = 0.0f32; }                                       // C 3955

	      if signbit! (py) != signbit! ((*b).delta_y)              // C 3957
		{ py = 0.0f32; }                                       // C 3958

	      px += ((*b).delta_x                                      // C 3960-3961
		     * powf (scroll_width as f32, 2.0f32 / 3.0f32));
	      py += ((*b).delta_y                                      // C 3962-3963
		     * powf (scroll_height as f32, 2.0f32 / 3.0f32));

	      if fabsf (py) >= FRAME_LINE_HEIGHT (f) as f32            // C 3965
		  || fabsf (px) >= FRAME_COLUMN_WIDTH (f) as f32       // C 3966
		  || ! mwheel_coalesce_scroll_events                // C 3967
		{
		  input_event_bits! (kind, &raw mut inev,              // C 3969-3971
				     if fabsf (px) > fabsf (py) { HORIZ_WHEEL_EVENT }
				     else { WHEEL_EVENT });
		  inev.code = 0;                                       // C 3972

		  XSETINT! (inev.x, x as isize);                       // C 3974
		  XSETINT! (inev.y, y as isize);                       // C 3975
		  inev.arg = list3 (Qnil (), make_float (-px as c_double),   // C 3976-3977
				    make_float (-py as c_double));
		  XSETFRAME! (inev.frame_or_window, f);                // C 3978

		  inev.modifiers |= (if signbit! (                      // C 3980-3983
					       if input_event_bits! (kind, &raw mut inev)
						  == HORIZ_WHEEL_EVENT { px } else { py })
				       { up_modifier } else { down_modifier }) as c_uint;
		  py = 0.0f32;                                         // C 3984
		  px = 0.0f32;                                         // C 3985

		  if be_drag_and_drop_in_progress ()                   // C 3987
		    { haiku_note_drag_wheel (&raw mut inev); }         // C 3988
		}

	      break 'switch;                                           // C 3991
	    }
	  MENU_BAR_RESIZE =>                                           // C 3993
	    {
	      let b: *mut haiku_menu_bar_resize_event
		= buf as *mut haiku_menu_bar_resize_event;             // C 3995
	      let f: *mut frame = haiku_window_to_frame ((*b).window);  // C 3996

	      if f.is_null () || ! FRAME_EXTERNAL_MENU_BAR (f)         // C 3998
		{ continue 'read_socket; }                                          // C 3999

	      if (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).wait_for_event_type  // C 4001-4002
		  == MENU_BAR_RESIZE
		{ (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).wait_for_event_type = -1; }  // C 4003

	      let old_height: c_int = FRAME_MENU_BAR_HEIGHT! (f);      // C 4005

	      FRAME_MENU_BAR_HEIGHT! (f, (*b).height);                 // C 4007
	      FRAME_MENU_BAR_LINES! (f,                                // C 4008-4009
				     ((*b).height + FRAME_LINE_HEIGHT (f)) / FRAME_LINE_HEIGHT (f));

	      if old_height != (*b).height                             // C 4011
		{
		  adjust_frame_size (f, -1, -1, 3, true, Qmenu_bar_lines ());  // C 4013
		  haiku_clear_under_internal_border (f);               // C 4014
		}
	      break 'switch;                                           // C 4016
	    }
	  MENU_BAR_CLICK =>                                            // C 4018
	    {
	      let b: *mut haiku_menu_bar_click_event = buf as *mut haiku_menu_bar_click_event;  // C 4020
	      let f: *mut frame = haiku_window_to_frame ((*b).window);  // C 4021

	      if f.is_null () || ! FRAME_EXTERNAL_MENU_BAR (f)         // C 4023
		{ continue 'read_socket; }                                          // C 4024

	      if (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).saved_menu_event.is_null ()  // C 4026
		{
		  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).saved_menu_event   // C 4027
		    = xmalloc (core::mem::size_of::<haiku_menu_bar_click_event> ()) as *mut haiku_menu_bar_click_event;
		}
	      *(*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).saved_menu_event = *b;  // C 4028
	      input_event_bits! (kind, &raw mut inev, MENU_BAR_ACTIVATE_EVENT);  // C 4029
	      XSETFRAME! (inev.frame_or_window, f);                    // C 4030
	      break 'switch;                                           // C 4031
	    }
	  MENU_BAR_OPEN | MENU_BAR_CLOSE =>                            // C 4033-4034
	    {
	      let b: *mut haiku_menu_bar_state_event = buf as *mut haiku_menu_bar_state_event;  // C 4036
	      let f: *mut frame = haiku_window_to_frame ((*b).window);  // C 4037

	      if f.is_null () || ! FRAME_EXTERNAL_MENU_BAR (f)         // C 4039
		{ continue 'read_socket; }                                          // C 4040

	      if r#type == MENU_BAR_OPEN                               // C 4042
		{
		  haiku_output_bits! (menu_bar_open_p, FRAME_OUTPUT_DATA! (f), 1);  // C 4044
		  popup_activated_p += 1;                              // C 4045
		}
	      else
		{
		  if popup_activated_p == 0                            // C 4049
		    { emacs_abort (); }                                // C 4050

		  if haiku_output_bits! (menu_bar_open_p, FRAME_OUTPUT_DATA! (f))   // C 4052
		    {
		      haiku_output_bits! (menu_bar_open_p, FRAME_OUTPUT_DATA! (f), 0);  // C 4054
		      popup_activated_p -= 1;                          // C 4055
		    }
		}
	      break 'switch;                                           // C 4058
	    }
	  MENU_BAR_SELECT_EVENT =>                                     // C 4060
	    {
	      let b: *mut haiku_menu_bar_select_event = buf as *mut haiku_menu_bar_select_event;  // C 4062
	      let f: *mut frame = haiku_window_to_frame ((*b).window);  // C 4063

	      if f.is_null () || ! FRAME_EXTERNAL_MENU_BAR (f)         // C 4065
		{ continue 'read_socket; }                                          // C 4066

	      find_and_call_menu_selection (f, (*f).menu_bar_items_used,   // C 4068
					    (*f).menu_bar_vector, (*b).ptr);   // C 4069
	      break 'switch;                                           // C 4070
	    }
	  MENU_BAR_HELP_EVENT =>                                       // C 4072
	    {
	      let b: *mut haiku_menu_bar_help_event = buf as *mut haiku_menu_bar_help_event;  // C 4074

	      if popup_activated_p == 0                                // C 4076
		{ continue 'read_socket; }                                          // C 4077

	      let f: *mut frame = haiku_window_to_frame ((*b).window);  // C 4079
	      if f.is_null () || ! FRAME_EXTERNAL_MENU_BAR (f)         // C 4080
		  || ! haiku_output_bits! (menu_bar_open_p, FRAME_OUTPUT_DATA! (f))  // C 4081
		{ continue 'read_socket; }                                          // C 4082

	      run_menu_bar_help_event (f, (*b).mb_idx);                // C 4084
	      break 'switch;                                           // C 4085
	    }
	  ZOOM_EVENT =>                                                // C 4087
	    {
	      let b: *mut haiku_zoom_event = buf as *mut haiku_zoom_event;  // C 4089
	      let f: *mut frame = haiku_window_to_frame ((*b).window);  // C 4090

	      if f.is_null ()                                          // C 4092
		{ continue 'read_socket; }                                          // C 4093

	      if (*b).fullscreen_mode == FULLSCREEN_MODE_MAXIMIZED     // C 4095
		{ want_fullscreen! (f, FULLSCREEN_NONE); }             // C 4096
	      else
		{ want_fullscreen! (f, FULLSCREEN_MAXIMIZED); }        // C 4098

	      (*(FRAME_TERMINAL! (f))).fullscreen_hook.unwrap () (f as *mut c_void);   // C 4100
	      break 'switch;                                           // C 4101
	    }
	  DRAG_AND_DROP_EVENT =>                                       // C 4103
	    {
	      let b: *mut haiku_drag_and_drop_event = buf as *mut haiku_drag_and_drop_event;  // C 4105
	      let f: *mut frame = haiku_window_to_frame ((*b).window);  // C 4106

	      if f.is_null ()                                          // C 4108
		{
		  BMessage_delete ((*b).message);                      // C 4110
		  continue 'read_socket;                                            // C 4111
		}

	      input_event_bits! (kind, &raw mut inev, DRAG_N_DROP_EVENT);  // C 4114
	      inev.arg = haiku_message_to_lisp ((*b).message);         // C 4115

	      XSETINT! (inev.x, (*b).x as isize);                      // C 4117
	      XSETINT! (inev.y, (*b).y as isize);                      // C 4118
	      XSETFRAME! (inev.frame_or_window, f);                    // C 4119

	      BMessage_delete ((*b).message);                          // C 4121
	      break 'switch;                                           // C 4122
	    }
	  SCREEN_CHANGED_EVENT =>                                      // C 4124
	    {
	      let b: *mut haiku_screen_changed_event = buf as *mut haiku_screen_changed_event;  // C 4126

	      input_event_bits! (kind, &raw mut inev, MONITORS_CHANGED_EVENT);  // C 4128
	      XSETTERMINAL! (inev.arg, (*x_display_list).terminal);    // C 4129
	      inev.timestamp = ((*b).when / 1000) as Time;             // C 4130
	      break 'switch;                                           // C 4131
	    }
	  CLIPBOARD_CHANGED_EVENT =>                                   // C 4133
	    {
	      be_handle_clipboard_changed_message ();                  // C 4134
	      break 'switch;                                           // C 4135
	    }
	  APP_QUIT_REQUESTED_EVENT =>                                  // C 4136
	    {
	      input_event_bits! (kind, &raw mut inev, SAVE_SESSION_EVENT);  // C 4137
	      inev.arg = Qt ();                                        // C 4138
	      break 'switch;                                           // C 4139
	    }
	  FONT_CHANGE_EVENT =>                                         // C 4140
	    {
	      /* This generates CONFIG_CHANGED_EVENTs, which are then
		 handled in Lisp.  */                              // C 4141-4142
	      haiku_handle_font_change_event (buf as *mut haiku_font_change_event, &raw mut inev);     // C 4143
	      break 'switch;                                           // C 4144
	    }

	  NOTIFICATION_CLICK_EVENT | KEY_UP | DUMMY_EVENT =>           // C 4146, 4158, 4159
	    {
	      /* This code doesn't function, but the why is unknown.  */  // C 4147
	      /* C 4148-4156 is `#if 0`: the frozen table leaves the condition false, and the port
		 carries the dead arm as this record rather than as text (P-08's rule for a dead
		 arm; the ledger's range table names C 4148 as the block).  */
	      break 'switch;                                           // C 4161
	    }
	  _ => {}                                                      // C 4160 (`default: break;`)
	}
      }

      haiku_read_size (&raw mut b_size, false);                        // C 4164

      if input_event_bits! (kind, &raw mut inev) != NO_EVENT           // C 4166
	{
	  if input_event_bits! (kind, &raw mut inev) != HELP_EVENT     // C 4168
	      && inev.timestamp == 0
	    {
	      inev.timestamp = (if button_or_motion_p != 0              // C 4169-4170
				  { (*x_display_list).last_mouse_movement_time }
				else { (system_time () / 1000) as Time });  // C 4171
	    }
	  kbd_buffer_store_event_hold (&raw mut inev, hold_quit);        // C 4172
	  message_count += 1;                                            // C 4173
	}

      if input_event_bits! (kind, &raw mut inev2) != NO_EVENT          // C 4176
	{
	  if input_event_bits! (kind, &raw mut inev2) != HELP_EVENT    // C 4178 (C's own test reads
	      && inev.timestamp == 0                                   //  `inev`, not `inev2` - kept
	    {                                                          //  verbatim)
	      inev2.timestamp = (if button_or_motion_p != 0              // C 4179-4180
				   { (*x_display_list).last_mouse_movement_time }
				 else { (system_time () / 1000) as Time });  // C 4181
	    }
	  kbd_buffer_store_event_hold (&raw mut inev2, hold_quit);       // C 4182
	  message_count += 1;                                            // C 4183
	}
    }

  if do_help != 0 && ! (hold_quit != std::ptr::null_mut ()           // C 4187
			&& input_event_bits! (kind, hold_quit) != NO_EVENT)
    {
      let mut help_frame: Lisp_Object = Qnil ();                     // C 4189

      if ! (*x_display_list).last_mouse_frame.is_null ()             // C 4191
	{
	  XSETFRAME! (help_frame,                                    // C 4192-4193
		      (*x_display_list).last_mouse_frame);
	}

      if do_help > 0                                                 // C 4195
	{
	  any_help_event_p = true;                                   // C 4197
	  gen_help_event (dispextern_unit_names::help_echo_string, help_frame,              // C 4198-4200
			  dispextern_unit_names::help_echo_window, dispextern_unit_names::help_echo_object,
			  dispextern_unit_names::help_echo_pos);
	}
      else
	{
	  dispextern_unit_names::help_echo_string = Qnil ();                                // C 4204
	  gen_help_event (Qnil (), help_frame, Qnil (), Qnil (), 0);  // C 4205
	}
    }

  unblock_input ();                                                 // C 4209

  return message_count;                                             // C 4211
}

/* C 4213-4405: the eight items between `haiku_read_socket` and `haiku_create_terminal` -
   `haiku_get_focus_frame` (C 4214-4227), `haiku_frame_rehighlight` (C 4229-4233),
   `haiku_delete_window` (C 4235-4240), `haiku_free_pixmap` (C 4242-4246), `haiku_flash`
   (C 4248-4341), `haiku_beep` (C 4343-4358), `haiku_toggle_invisible_pointer` (C 4360-4374) and
   `haiku_fullscreen` (C 4376-4405).  **Seven of the eight are `unsafe extern "C" fn`** and
   `haiku_flash` alone is a plain `unsafe fn`, because C 4418-4456 hands the other seven addresses
   to `struct terminal`'s hook fields (`ring_bell_hook`, `delete_frame_hook`,
   `frame_rehighlight_hook`, `free_pixmap`, `toggle_invisible_pointer_hook`, `fullscreen_hook`,
   `get_focus_frame`; `include/termhooks.rs` 242/332/333/275/278/273 carry them as
   `Option<unsafe extern "C" fn …>`) - the slice-21 conversion rule, and `haiku_flash` is reached
   only by C 4352's call.  The next first definition after this block is C 4407's
   `haiku_create_terminal`.  */

/// C: frame.h 1914-1921: `INLINE void flush_frame (struct frame *f)` - the mirror
/// `include/frame.rs` 1909's macro spells `FRAME_RIF` without the `*mut c_void` cast this unit's
/// `FRAME_RIF!` needs (`terminal.rif` is `*mut c_void`, `include/termhooks.rs` 230), so the unit
/// shadows it with C's own body - the `src/haikufns.rs` 2045-2057 copy of the same decision, made
/// for the same reason in this same lane (measured: `flush_frame!` from the mirror is two
/// `E0308`s at `include/termhooks.rs` 437, `expected *mut redisplay_interface, found *mut
/// c_void`, both anchored at this unit's two call sites C 4292 and C 4338).  **One cast less than
/// the `haikufns.rs` copy**: that unit's `redisplay_interface` types the hook `*mut c_void`, while
/// slice 22's copy in *this* unit spells C's own pointee types (dispextern.h 3115), so the call
/// passes `$f` unchanged - measured: carrying the cast over was two `E0308`s the other way
/// (`expected *mut frame, found *mut c_void`).
#[cfg (feature = "have_haiku")]
macro_rules! flush_frame {
  ($f:expr) => {
    {
      let rif: *mut redisplay_interface = FRAME_RIF! ($f) as *mut redisplay_interface;
      if !rif.is_null () && (*rif).flush_display.is_some ()
	{ ((*rif).flush_display.unwrap ()) ($f); }
    }
  };
}

/* C: frame.h 1504: `extern WINDOW_SYSTEM_RETURN void check_window_system (struct frame *);`
   (C 4238's call; `src/fontset.rs` 106 and `src/androidmenu.rs` 92 carry the same use-site copy).
   C: lisp.h 5095: `extern bool detect_input_pending (void);` (C 4299's loop condition).
   `<OS.h>`'s `wait_for_objects_etc` and `B_ABSOLUTE_TIMEOUT` (C 4306) are the SDK's own and carry
   the standing `UNVERIFIED` mark, as `src/haiku_io.rs` 44-74 does for the same header's calls and
   values; the `object_wait_info` record, `B_EVENT_READ` and `B_OBJECT_TYPE_PORT` this item also
   uses are the ones this unit declared for C 3214 (slice 23), so they are **not** repeated.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn check_window_system (f: *mut frame);
  pub fn detect_input_pending () -> bool;
  /// C: `status_t wait_for_objects_etc (object_wait_info *infos, int numInfos, uint32 flags,
  /// bigtime_t timeout);   [<OS.h>; UNVERIFIED here]`
  pub fn wait_for_objects_etc (infos: *mut object_wait_info, num_infos: c_int, flags: c_uint,
			       timeout: bigtime_t) -> status_t;
}
/// C: `#define B_ABSOLUTE_TIMEOUT 4` (`<OS.h>`; UNVERIFIED here - the value is the SDK's own
/// documented one).
#[cfg (feature = "have_haiku")]
pub const B_ABSOLUTE_TIMEOUT: c_uint = 4;

// C 4214-4227.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_get_focus_frame (f: *mut frame) -> Lisp_Object
{
  let mut lisp_focus: Lisp_Object;                                    // C 4217
  let focus: *mut frame;                                              // C 4218

  focus = (*FRAME_DISPLAY_INFO! (f)).focused_frame;                   // C 4220

  if focus.is_null ()                                                 // C 4222
    { return Qnil (); }                                               // C 4223

  XSETFRAME! (lisp_focus, focus);                                     // C 4225
  return lisp_focus;                                                  // C 4226
}

// C 4229-4233.  C's parameter name is `frame`, which the port keeps (the type `frame` and the
// value `frame` live in Rust's two namespaces).
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_frame_rehighlight (frame: *mut frame)
{
  haiku_rehighlight ();                                               // C 4232
}

// C 4235-4240.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_delete_window (f: *mut frame)
{
  check_window_system (f);                                            // C 4238
  haiku_free_frame_resources (f);                                     // C 4239
}

// C 4242-4246.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_free_pixmap (f: *mut frame, pixmap: Emacs_Pixmap)
{
  BBitmap_free (pixmap);                                              // C 4245
}

// C 4248-4341.
#[cfg (feature = "have_haiku")]
unsafe fn haiku_flash (f: *mut frame)
{
  /* Get the height not including a menu bar widget.  */               // C 4251
  let height: c_int = FRAME_PIXEL_HEIGHT! (f);                        // C 4252
  /* Height of each line to flash.  */                                // C 4253
  let flash_height: c_int = FRAME_LINE_HEIGHT (f);                    // C 4254
  /* These will be the left and right margins of the rectangles.  */  // C 4255
  let flash_left: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);            // C 4256
  let flash_right: c_int = FRAME_PIXEL_WIDTH! (f) - FRAME_INTERNAL_BORDER_WIDTH (f);  // C 4257
  let width: c_int = flash_right - flash_left;                        // C 4258
  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! (f);                  // C 4259
  let mut info: object_wait_info = core::mem::zeroed ();              // C 4260 (PORTING.md 1.5: its
								      //  address is handed to C 4306)
  let mut wakeup: bigtime_t;                                          // C 4261

  info.object = port_application_to_emacs;                            // C 4263
  info.r#type = B_OBJECT_TYPE_PORT;                                   // C 4264
  info.events = B_EVENT_READ;                                         // C 4265
  wakeup = system_time () + 150000;                                   // C 4266

  BView_draw_lock (view, true, 0, 0, FRAME_PIXEL_WIDTH! (f),          // C 4268
		   FRAME_PIXEL_HEIGHT! (f));                            // C 4269
  BView_StartClip (view);                                             // C 4270
  /* If window is tall, flash top and bottom line.  */                // C 4271
  if height > 3 * FRAME_LINE_HEIGHT (f)                               // C 4272
    {
      BView_InvertRect (view, flash_left,                             // C 4274
			(FRAME_INTERNAL_BORDER_WIDTH (f)              // C 4275
			 + FRAME_TOP_MARGIN_HEIGHT (f)),              // C 4276
			width, flash_height);                         // C 4277

      BView_InvertRect (view, flash_left,                             // C 4279
			(height - flash_height                        // C 4280
			 - FRAME_INTERNAL_BORDER_WIDTH (f)            // C 4281
			 - FRAME_BOTTOM_MARGIN_HEIGHT (f)),           // C 4282
			width, flash_height);                         // C 4283
    }
  else
    /* If it is short, flash it all.  */                              // C 4286
    { BView_InvertRect (view, flash_left, FRAME_INTERNAL_BORDER_WIDTH (f),   // C 4287
			width, height - 2 * FRAME_INTERNAL_BORDER_WIDTH (f)); }  // C 4288
  BView_EndClip (view);                                               // C 4289
  BView_draw_unlock (view);                                           // C 4290

  flush_frame! (f);                                                   // C 4292

  if EmacsView_double_buffered_p (view) != 0                          // C 4294
    { haiku_flip_buffers (f); }                                       // C 4295

  /* Keep waiting until past the time wakeup or any input gets
     available.  */                                                   // C 4297-4298
  while ! detect_input_pending ()                                     // C 4299
    {
      /* Break if result would not be positive.  */                   // C 4301
      if wakeup < system_time ()                                      // C 4302
	{ break; }                                                    // C 4303

      /* Try to wait that long--but we might wake up sooner.  */      // C 4305
      wait_for_objects_etc (&raw mut info, 1, B_ABSOLUTE_TIMEOUT, wakeup);  // C 4306

      if info.events & B_EVENT_READ != 0                              // C 4308
	{ break; }                                                    // C 4309

      info.events = B_EVENT_READ;                                     // C 4311
    }

  BView_draw_lock (view, true, 0, 0, FRAME_PIXEL_WIDTH! (f),          // C 4314
		   FRAME_PIXEL_HEIGHT! (f));                            // C 4315
  BView_StartClip (view);                                             // C 4316
  /* If window is tall, flash top and bottom line.  */                // C 4317
  if height > 3 * FRAME_LINE_HEIGHT (f)                               // C 4318
    {
      BView_InvertRect (view, flash_left,                             // C 4320
			(FRAME_INTERNAL_BORDER_WIDTH (f)              // C 4321
			 + FRAME_TOP_MARGIN_HEIGHT (f)),              // C 4322
			width, flash_height);                         // C 4323

      BView_InvertRect (view, flash_left,                             // C 4325
			(height - flash_height                        // C 4326
			 - FRAME_INTERNAL_BORDER_WIDTH (f)            // C 4327
			 - FRAME_BOTTOM_MARGIN_HEIGHT (f)),           // C 4328
			width, flash_height);                         // C 4329
    }
  else
    /* If it is short, flash it all.  */                              // C 4332
    { BView_InvertRect (view, flash_left, FRAME_INTERNAL_BORDER_WIDTH (f),   // C 4333
			width, height - 2 * FRAME_INTERNAL_BORDER_WIDTH (f)); }  // C 4334
  BView_EndClip (view);                                               // C 4335
  BView_draw_unlock (view);                                           // C 4336

  flush_frame! (f);                                                   // C 4338
  if EmacsView_double_buffered_p (view) != 0                          // C 4339
    { haiku_flip_buffers (f); }                                       // C 4340
}

// C 4343-4358.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_beep (f: *mut frame)
{
  if visible_bell                                                    // C 4346
    {
      let view: *mut c_void = FRAME_HAIKU_VIEW! (f);                  // C 4348
      if ! view.is_null ()                                            // C 4349
	{
	  block_input ();                                              // C 4351
	  haiku_flash (f);                                             // C 4352
	  unblock_input ();                                            // C 4353
	}
    }
  else
    { haiku_ring_bell (); }                                           // C 4357
}

// C 4360-4374.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_toggle_invisible_pointer (f: *mut frame, invisible_p: bool)
{
  let view: *mut c_void = FRAME_HAIKU_VIEW! (f);                      // C 4363

  if ! view.is_null () && ! FRAME_TOOLTIP_P (f)                       // C 4365
    {
      block_input ();                                                 // C 4367
      BView_set_view_cursor (view, if invisible_p                     // C 4368
			       { (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).no_cursor }  // C 4369
			     else
			       { (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).current_cursor });  // C 4370
      pointer_invisible! (f, invisible_p);                            // C 4371
      unblock_input ();                                               // C 4372
    }
}

// C 4376-4405.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_fullscreen (f: *mut frame)
{
  let mut mode: haiku_fullscreen_mode;                                // C 4379

  /* When FRAME_OUTPUT_DATA (f)->configury_done is false, the frame is
     being created, and its regular width and height have not yet been
     set.  This function will be called again by haiku_create_frame,
     so do nothing.  */                                               // C 4381-4384
  if ! (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).configury_done   // C 4385
    { return; }                                                       // C 4386

  if want_fullscreen! (f) == FULLSCREEN_MAXIMIZED                     // C 4388
    { mode = FULLSCREEN_MODE_MAXIMIZED; }                             // C 4389
  else if want_fullscreen! (f) == FULLSCREEN_BOTH                     // C 4390
    { mode = FULLSCREEN_MODE_BOTH; }                                  // C 4391
  else if want_fullscreen! (f) == FULLSCREEN_WIDTH                    // C 4392
    { mode = FULLSCREEN_MODE_WIDTH; }                                 // C 4393
  else if want_fullscreen! (f) == FULLSCREEN_HEIGHT                   // C 4394
    { mode = FULLSCREEN_MODE_HEIGHT; }                                // C 4395
  else
    { mode = FULLSCREEN_MODE_NONE; }                                  // C 4397

  want_fullscreen! (f, FULLSCREEN_NONE);                              // C 4399
  be_set_window_fullscreen_mode (FRAME_HAIKU_WINDOW! (f), mode);      // C 4400
  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).fullscreen_mode = mode;  // C 4401

  haiku_update_size_hints (f);                                        // C 4403
  haiku_make_fullscreen_consistent (f);                               // C 4404
}

/* C 4406-4569: the file's last three items before the out-of-order block - `haiku_create_terminal`
   (C 4407-4460), `haiku_term_init` (C 4461-4556) and `put_xrm_resource` (C 4558-4569).  **This is
   the item the C 3140-3170 object was written for**: C 4412 hands `&haiku_redisplay_interface` to
   `create_terminal`, and C 4418-4456 installs the thirty-nine terminal hooks - the eleven of them
   this unit defines itself are the eleven slice 26 converted to `unsafe extern "C" fn` in the same
   round (the slice-21 ABI rule, now applied to a referrer that *uses* the addresses rather than to
   the table).  */

/* C's hook assignments: termhooks.h degrades `struct frame *`/`struct window *` parameters to
   `*mut c_void` (P-72 rule 3, the mirror cannot name those types), while the definitions keep C's
   pointer - the conversion is C's own implicit one and the ABI does not depend on the pointee.
   The `src/pgtkterm.rs` 7027-7036 copy of this helper, body-identical.  */
#[cfg (feature = "have_haiku")]
macro_rules! hook {
  ($target:ty, $f:expr) => { unsafe {
    let p: *const c_void = $f as *const c_void;
    std::mem::transmute::<*const c_void, Option<$target>> (p) } };
}

/* C: termhooks.h 964: `extern struct terminal *create_terminal (enum output_method, struct
   redisplay_interface *);` - the C 4412 call (`src/terminal.rs` 366 defines it; the
   `src/pgtkterm.rs` 7038-7043 copy of this declaration is the same one).
   C: keyboard.h 477: `extern KBOARD *allocate_kboard (Lisp_Object);` - C 4416.
   C: dispextern.h 3708: `void gui_init_fringe (struct redisplay_interface *);` - C 4510, whose
   argument is `terminal->rif` (the mirror types that field `*mut c_void`, `include/termhooks.rs`
   230), so the declaration takes the pointer untyped (the `src/pgtkterm.rs` 10260 spelling).
   C: haiku_support.h 620: `extern void *be_create_cursor_from_id (int);` - C 4513's sixteen uses
   (the `src/haikufns.rs` 328-330 copy; `enum haiku_cursor`'s members are this unit's
   `CURSOR_ID_*`, `include/haiku_support.rs` 61-...).
   C: lisp.h 5371: `extern AVOID fatal (const char *msgid, ...) ATTRIBUTE_FORMAT_PRINTF (1, 2);` -
   the declaration keeps C's shape: the `...` parameter *is* the list (PORTING.md P-04).  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn create_terminal (r#type: output_method, rif: *mut c_void) -> *mut terminal;
  pub fn allocate_kboard (type_: Lisp_Object) -> *mut kboard;
  pub fn gui_init_fringe (rif: *mut c_void);
  pub fn be_create_cursor_from_id (id: c_int) -> *mut c_void;
  pub fn fatal (msgid: *const c_char, ...) -> !;
}

// C 4407-4460.
#[cfg (feature = "have_haiku")]
unsafe fn haiku_create_terminal (dpyinfo: *mut haiku_display_info) -> *mut terminal
{
  let terminal: *mut terminal;                                        // C 4410

  terminal = create_terminal (output_haiku,                               // C 4412
			      &raw mut haiku_redisplay_interface as *mut c_void);

  (*terminal).display_info.haiku = dpyinfo as *mut c_void;            // C 4414 (the union member is
								      //  `void *`, P-72 rule 3)
  (*dpyinfo).terminal = terminal;                                     // C 4415
  (*terminal).kboard = allocate_kboard (Qhaiku ()) as *mut c_void;     // C 4416

  (*terminal).iconify_frame_hook                                      // C 4418
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_iconify_frame);
  (*terminal).focus_frame_hook                                        // C 4419
    = hook! (unsafe extern "C" fn (*mut c_void, bool), haiku_focus_frame);
  (*terminal).ring_bell_hook                                          // C 4420
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_beep);
  (*terminal).popup_dialog_hook                                       // C 4421
    = hook! (unsafe extern "C" fn (*mut c_void, Lisp_Object, Lisp_Object) -> Lisp_Object,
	     haiku_popup_dialog);
  (*terminal).frame_visible_invisible_hook                            // C 4422
    = hook! (unsafe extern "C" fn (*mut c_void, bool), haiku_set_frame_visible_invisible);
  (*terminal).set_frame_offset_hook                                   // C 4423
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int), haiku_set_offset);
  (*terminal).delete_terminal_hook                                    // C 4424
    = hook! (unsafe extern "C" fn (*mut terminal), haiku_delete_terminal);
  (*terminal).get_string_resource_hook                                // C 4425
    = hook! (unsafe extern "C" fn (*mut c_void, *const c_char, *const c_char) -> *const c_char,
	     haiku_get_string_resource);
  (*terminal).set_new_font_hook                                       // C 4426
    = hook! (unsafe extern "C" fn (*mut c_void, Lisp_Object, c_int) -> Lisp_Object,
	     haiku_new_font);
  (*terminal).defined_color_hook                                      // C 4427
    = hook! (unsafe extern "C" fn (*mut c_void, *const c_char, *mut Emacs_Color, bool, bool)
			     -> bool, haiku_defined_color);
  (*terminal).set_window_size_hook                                    // C 4428
    = hook! (unsafe extern "C" fn (*mut c_void, bool, c_int, c_int), haiku_set_window_size);
  (*terminal).read_socket_hook                                        // C 4429
    = hook! (unsafe extern "C" fn (*mut terminal, *mut input_event) -> c_int, haiku_read_socket);
  (*terminal).implicit_set_name_hook                                  // C 4430
    = hook! (unsafe extern "C" fn (*mut c_void, Lisp_Object, Lisp_Object), haiku_implicitly_set_name);
  (*terminal).mouse_position_hook                                     // C 4431
    = hook! (unsafe extern "C" fn (*mut *mut c_void, c_int, *mut Lisp_Object,
				   *mut scroll_bar_part, *mut Lisp_Object, *mut Lisp_Object,
				   *mut Time), haiku_mouse_position);
  (*terminal).delete_frame_hook                                       // C 4432
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_delete_window);
  (*terminal).frame_up_to_date_hook                                   // C 4433
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_frame_up_to_date);
  (*terminal).buffer_flipping_unblocked_hook                          // C 4434
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_buffer_flipping_unblocked_hook);
  (*terminal).clear_frame_hook                                        // C 4435
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_clear_frame);
  (*terminal).change_tab_bar_height_hook                              // C 4436
    = hook! (unsafe extern "C" fn (*mut c_void, c_int), haiku_change_tab_bar_height);
  (*terminal).change_tool_bar_height_hook                             // C 4437
    = hook! (unsafe extern "C" fn (*mut c_void, c_int), haiku_change_tool_bar_height);
  (*terminal).set_vertical_scroll_bar_hook                            // C 4438
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int), haiku_set_vertical_scroll_bar);
  (*terminal).set_horizontal_scroll_bar_hook                          // C 4439
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int),
	     haiku_set_horizontal_scroll_bar);
  (*terminal).set_scroll_bar_default_height_hook                      // C 4440
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_set_scroll_bar_default_height);
  (*terminal).set_scroll_bar_default_width_hook                       // C 4441
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_set_scroll_bar_default_width);
  (*terminal).judge_scroll_bars_hook                                  // C 4442
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_judge_scroll_bars);
  (*terminal).condemn_scroll_bars_hook                                // C 4443
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_condemn_scroll_bars);
  (*terminal).redeem_scroll_bar_hook                                  // C 4444
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_redeem_scroll_bar);
  (*terminal).update_begin_hook                                       // C 4445
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_update_begin);
  (*terminal).update_end_hook                                         // C 4446
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_update_end);
  (*terminal).frame_rehighlight_hook                                  // C 4447
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_frame_rehighlight);
  (*terminal).query_frame_background_color                            // C 4448
    = hook! (unsafe extern "C" fn (*mut c_void, *mut Emacs_Color),
	     haiku_query_frame_background_color);
  /* The field carries termhooks.h 849-853's own condition, not C's unconditional text: without
     the window system the member does not exist (the `src/pgtkterm.rs` 7172-7174 / Deviation 60
     spelling of the same pair - the attribute needs a *statement*, and an attribute on an
     expression is `E0658`, measured in this round's first probe).  */
  #[cfg (feature = "have_window_system")]
  { (*terminal).free_pixmap                                           // C 4449
      = hook! (unsafe extern "C" fn (*mut c_void, *mut c_void), haiku_free_pixmap); }
  (*terminal).frame_raise_lower_hook                                  // C 4450
    = hook! (unsafe extern "C" fn (*mut c_void, bool), haiku_frame_raise_lower);
  (*terminal).menu_show_hook                                          // C 4451
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int, Lisp_Object,
				   *mut *const c_char) -> Lisp_Object, haiku_menu_show);
  (*terminal).toggle_invisible_pointer_hook                           // C 4452
    = hook! (unsafe extern "C" fn (*mut c_void, bool), haiku_toggle_invisible_pointer);
  (*terminal).fullscreen_hook                                         // C 4453
    = hook! (unsafe extern "C" fn (*mut c_void), haiku_fullscreen);
  (*terminal).toolkit_position_hook                                   // C 4454
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, *mut bool, *mut bool),
	     haiku_toolkit_position);
  /* The same rule as `free_pixmap`: termhooks.h 719-722's `#ifdef HAVE_EXT_MENU_BAR`, which
     `lisp.h` 3931-3934 defines for the Haiku window system.  */
  #[cfg (feature = "have_ext_menu_bar")]
  { (*terminal).activate_menubar_hook                                 // C 4455
      = hook! (unsafe extern "C" fn (*mut c_void), haiku_activate_menubar); }
  (*terminal).get_focus_frame                                         // C 4456
    = hook! (unsafe extern "C" fn (*mut c_void) -> Lisp_Object, haiku_get_focus_frame);

  return terminal;                                                    // C 4458
}

// C 4461-4556.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_term_init () -> *mut haiku_display_info
{
  let mut dpyinfo: *mut haiku_display_info;                            // C 4464
  let terminal: *mut terminal;                                        // C 4465
  let mut color_file: Lisp_Object;                                    // C 4466
  let mut color_map: Lisp_Object;
  let mut system_name: Lisp_Object;
  let mut nbytes: ptrdiff_t;                                          // C 4467
  let mut name_buffer: *mut c_void;                                   // C 4468

  block_input ();                                                     // C 4470

  Fset_input_interrupt_mode (Qt ());                                  // C 4472
  baud_rate = 19200;                                                  // C 4473
  dpyinfo = xzalloc (core::mem::size_of::<haiku_display_info> ()) as *mut haiku_display_info;  // C 4474
  haiku_io_init ();                                                   // C 4475

  if port_application_to_emacs < B_OK                                 // C 4477
      || port_emacs_to_session_manager < B_OK                         // C 4478
    { emacs_abort (); }                                               // C 4479

  color_file = Fexpand_file_name (build_string (c"rgb.txt".as_ptr ()),   // C 4481
				  Fsymbol_value (Qdata_directory ()));   // C 4482
  color_map = Fx_load_color_file (color_file);                        // C 4483

  if NILP (color_map)                                                 // C 4485
    { fatal (c"Could not read %s.\n".as_ptr (), SDATA (color_file)); }   // C 4486

  (*dpyinfo).color_map = color_map;                                   // C 4488
  (*dpyinfo).display = BApplication_setup ();                         // C 4489
  (*dpyinfo).next = x_display_list;                                   // C 4490
  (*dpyinfo).n_planes = be_get_display_planes ();                     // C 4491
  be_get_display_resolution (&raw mut (*dpyinfo).resx, &raw mut (*dpyinfo).resy);  // C 4492

  x_display_list = dpyinfo;                                            // C 4494

  terminal = haiku_create_terminal (dpyinfo);                          // C 4496
  if current_kboard == initial_kboard                                 // C 4497
    { current_kboard = (*terminal).kboard as *mut kboard; }            // C 4498

  let kb: *mut kboard = (*terminal).kboard as *mut kboard;             // C 4500
  (*kb).reference_count += 1;                                          // C 4500
  /* Never delete haiku displays -- there can only ever be one,
     anyhow.  */                                                     // C 4501-4502
  (*terminal).reference_count += 1;                                    // C 4503
  (*terminal).name = xstrdup (c"be".as_ptr ());                        // C 4504

  (*dpyinfo).name_list_element = Fcons (build_string (c"be".as_ptr ()), Qnil ());  // C 4506
  (*dpyinfo).smallest_font_height = 1;                                 // C 4507
  (*dpyinfo).smallest_char_width = 1;                                  // C 4508

  gui_init_fringe ((*terminal).rif);                                   // C 4510

  /* C 4512-4530: `#define ASSIGN_CURSOR(cursor, cursor_id)
     (dpyinfo->cursor = be_create_cursor_from_id (cursor_id))` and its sixteen uses.  The port
     keeps C's macro - a function-local `macro_rules!`, which is where C's own `#define` sits in
     the function - so that the macro's *name* and its two parameter names stay in the text; the
     member name travels as the `$cursor:ident` argument at each call, which is also what P-80's
     rule asks (a C member name may not hide inside a Rust macro *name*).  */
  macro_rules! ASSIGN_CURSOR {
    ($cursor:ident, $cursor_id:expr) => {
      (*dpyinfo).$cursor = be_create_cursor_from_id ($cursor_id) as Emacs_Cursor };
  }
  ASSIGN_CURSOR! (text_cursor, CURSOR_ID_I_BEAM);                      // C 4514
  ASSIGN_CURSOR! (nontext_cursor, CURSOR_ID_SYSTEM_DEFAULT);           // C 4515
  ASSIGN_CURSOR! (modeline_cursor, CURSOR_ID_CONTEXT_MENU);            // C 4516
  ASSIGN_CURSOR! (hand_cursor, CURSOR_ID_GRAB);                        // C 4517
  ASSIGN_CURSOR! (hourglass_cursor, CURSOR_ID_PROGRESS);               // C 4518
  ASSIGN_CURSOR! (horizontal_drag_cursor, CURSOR_ID_RESIZE_EAST_WEST);  // C 4519
  ASSIGN_CURSOR! (vertical_drag_cursor, CURSOR_ID_RESIZE_NORTH_SOUTH);  // C 4520
  ASSIGN_CURSOR! (left_edge_cursor, CURSOR_ID_RESIZE_WEST);            // C 4521
  ASSIGN_CURSOR! (top_left_corner_cursor, CURSOR_ID_RESIZE_NORTH_WEST);  // C 4522
  ASSIGN_CURSOR! (top_edge_cursor, CURSOR_ID_RESIZE_NORTH);            // C 4523
  ASSIGN_CURSOR! (top_right_corner_cursor, CURSOR_ID_RESIZE_NORTH_EAST);  // C 4524
  ASSIGN_CURSOR! (right_edge_cursor, CURSOR_ID_RESIZE_EAST);           // C 4525
  ASSIGN_CURSOR! (bottom_right_corner_cursor, CURSOR_ID_RESIZE_SOUTH_EAST);  // C 4526
  ASSIGN_CURSOR! (bottom_edge_cursor, CURSOR_ID_RESIZE_SOUTH);         // C 4527
  ASSIGN_CURSOR! (bottom_left_corner_cursor, CURSOR_ID_RESIZE_SOUTH_WEST);  // C 4528
  ASSIGN_CURSOR! (no_cursor, CURSOR_ID_NO_CURSOR);                     // C 4529

  /* C 4536/4542/4547's `"<PACKAGE_NAME>"` is the placeholder form of the name *this* program
     calls itself: PORTING.md 0.1-19's name half (status 3.433/3.599) puts this repository's own
     name in a default display name, and `src/androidfns.rs` 1277 (`fset_name`'s default) and
     `src/xterm.rs` 20484 (C 31017's `title[]`) carry the same substitution.  C 4536's `sizeof`
     keeps the reference's size, as `src/xterm.rs` 20095's note puts it for its own pair.  */
  system_name = Fsystem_name ();                                      // C 4532

  if STRINGP (system_name)                                            // C 4534
    {
      nbytes = (std::mem::size_of::<[c_char; 10]> ()                     // C 4536 (`sizeof "<PACKAGE_NAME>"`
		+ std::mem::size_of::<[c_char; 5]> ()) as ptrdiff_t;     //  + `sizeof " at "`)
      {                                                                 // C 4538
	let nbytes_ = nbytes.overflowing_add (SBYTES (system_name) as ptrdiff_t);
	nbytes = nbytes_.0;
	if nbytes_.1
	  { memory_full_up (); }                                        // C 4539
      }

      name_buffer = sa_stack_alloc (nbytes as usize) as *mut c_void;     // C 4541 (PORTING.md P-05's
									//  `alloca` alias)
      sprintf (name_buffer as *mut c_char, c"%s%s%s".as_ptr (), c"femacs".as_ptr (),   // C 4542
	       c" at ".as_ptr (), SDATA (system_name));                  // C 4543
      (*dpyinfo).default_name = build_string (name_buffer as *const c_char);   // C 4544
    }
  else
    { (*dpyinfo).default_name = build_string (c"femacs".as_ptr ()); }   // C 4547

  haiku_start_watching_selections ();                                 // C 4549

  /* Start listening for font configuration changes.  */              // C 4551
  be_listen_font_settings ();                                         // C 4552
  unblock_input ();                                                   // C 4553

  return dpyinfo;                                                     // C 4555
}

// C 4558-4569.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn put_xrm_resource (name: Lisp_Object, val: Lisp_Object)
{
  eassert! (STRINGP (name));                                          // C 4561
  eassert! (STRINGP (val) || NILP (val));                             // C 4562

  let lval: Lisp_Object = assoc_no_quit (name, rdb);                  // C 4564
  if ! NILP (lval)                                                    // C 4565
    { Fsetcdr (lval, val); }                                          // C 4566
  else
    { rdb = Fcons (Fcons (name, val), rdb); }                         // C 4568
}

/* C 4619-4898: the file's tail - `mark_haiku_display` (C 4620-4628), `haiku_scroll_bar_remove`
   (C 4630-4651), `haiku_calc_absolute_position` (C 4658-4728), `haiku_set_offset` (C 4730-4770),
   the `USE_BE_CAIRO` pair (C 4772-4793), `haiku_merge_cursor_foreground` (C 4795-4821) and
   `syms_of_haikuterm` (C 4823-4898).  With this round the file is **complete** and nothing is left
   owed on it.  */

/* C: lisp.h 4436: `extern void mark_object (Lisp_Object);` (C 4625/4626's two calls;
   `src/alloc.rs` 7308 defines it, and no mirror in this unit's graph declares it, so the unit
   declares it here - P-72 rule 4).  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn mark_object (obj: Lisp_Object);
}

/* C 4772-4793's `#ifdef USE_BE_CAIRO` arm - **dead on this target** (the frozen table leaves
   `USE_BE_CAIRO` undefined, exactly as C 40-42's `<cairo.h>` include is) but live under the
   `be_cairo` feature the port names for it (`femacs/xtask/Cargo.toml` 73).  `<cairo.h>` is a
   system header of the Haiku toolchain with no mirror (P-88's class), so the two calls and the
   opaque `cairo_t` are named at the use site with the `src/pgtkterm.rs` 3622-3636 spelling, and
   `EmacsView_cairo_context` is `include/haiku_support.rs`'s (haikuterm.h 297-301's
   `FRAME_CR_CONTEXT (f)` reads it).  */
#[cfg (feature = "be_cairo")]
#[repr(C)]
pub struct cairo_t
{
  _private: [u8; 0],
}
#[cfg (feature = "be_cairo")]
unsafe extern "C" {
  /// C: cairo.h 561: `void cairo_save (cairo_t *cr);` - C 4781's call.
  pub fn cairo_save (cr: *mut cairo_t);
  /// C: cairo.h 564: `void cairo_restore (cairo_t *cr);` - C 4791's call.
  pub fn cairo_restore (cr: *mut cairo_t);
  /// C: haiku_support.h: `cairo_t *EmacsView_cairo_context (void *);` - the accessor
  /// haikuterm.h 297-301's `FRAME_CR_CONTEXT` calls.
  pub fn EmacsView_cairo_context (view: *mut c_void) -> *mut cairo_t;
}
/// C: haikuterm.h 297-301: `#define FRAME_CR_CONTEXT(f) (FRAME_HAIKU_VIEW (f) ?
/// EmacsView_cairo_context (FRAME_HAIKU_DRAWABLE (f)) : NULL)` - the `USE_BE_CAIRO` arm of the
/// two, so it carries that condition (C's `#ifdef`).
#[cfg (feature = "be_cairo")]
macro_rules! FRAME_CR_CONTEXT {
  ($f:expr) => {
    if FRAME_HAIKU_VIEW! ($f).is_null () { std::ptr::null_mut () }
    else { EmacsView_cairo_context (FRAME_HAIKU_DRAWABLE! ($f)) } };
}

// C 4620-4628.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_haiku_display ()
{
  if ! x_display_list.is_null ()                                      // C 4623
    {
      mark_object ((*x_display_list).color_map);                      // C 4625
      mark_object ((*x_display_list).default_name);                   // C 4626
    }
}

// C 4630-4651.  C 4651's bare `;` after the closing brace has no port counterpart (C's own
// stray semicolon; there is no statement for it to terminate).
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_scroll_bar_remove (bar: *mut scroll_bar)
{
  let view: *mut c_void;                                              // C 4633
  let f: *mut frame;                                                  // C 4634

  f = WINDOW_XFRAME! (XWINDOW ((*bar).window));                        // C 4636
  view = FRAME_HAIKU_DRAWABLE! (f);                                   // C 4637

  block_input ();                                                     // C 4639
  BView_forget_scroll_bar (view, (*bar).left, (*bar).top,             // C 4640
			   (*bar).width, (*bar).height);              // C 4641
  BScrollBar_delete ((*bar).scroll_bar);                              // C 4642
  expose_frame (WINDOW_XFRAME! (XWINDOW ((*bar).window)),             // C 4643
		(*bar).left, (*bar).top, (*bar).width, (*bar).height);   // C 4644

  if (*bar).horizontal                                                // C 4646
    { wset_horizontal_scroll_bar (XWINDOW ((*bar).window), Qnil ()); }   // C 4647
  else
    { wset_vertical_scroll_bar (XWINDOW ((*bar).window), Qnil ()); }     // C 4649
  unblock_input ();                                                   // C 4650
}

/* Calculate the absolute position in frame F
   from its current recorded position values and gravity.  */        // C 4655-4656

// C 4658-4728.
#[cfg (feature = "have_haiku")]
unsafe fn haiku_calc_absolute_position (f: *mut frame)
{
  let mut flags: c_int = (*f).size_hint_flags;                        // C 4661
  let p: *mut frame = FRAME_PARENT_FRAME (f);                         // C 4662
  let mut screen_width: c_int = 0;                                    // C 4663 (PORTING.md 1.5:
								      //  their addresses go to C 4670)
  let mut screen_height: c_int = 0;

  /* We have nothing to do if the current position
     is already for the top-left corner.  */                         // C 4665-4666
  if ! ((flags & XNegative) != 0 || (flags & YNegative) != 0)         // C 4667
    { return; }                                                       // C 4668

  be_get_screen_dimensions (&raw mut screen_width, &raw mut screen_height);  // C 4670

  /* Treat negative positions as relative to the leftmost bottommost
     position that fits on the screen.  */                           // C 4672-4673
  if (flags & XNegative) != 0                                         // C 4674
    {
      let mut width: c_int = FRAME_PIXEL_WIDTH! (f);                  // C 4676

      /* A frame that has been visible at least once should have outer
	 edges.  */                                                   // C 4678-4679
      if p.is_null ()                                                 // C 4680
	{
	  let mut frame: Lisp_Object;                                 // C 4682
	  let mut edges: Lisp_Object = Qnil ();                       // C 4683

	  XSETFRAME! (frame, f);                                      // C 4685
	  edges = Fhaiku_frame_edges (frame, Qouter_edges ());         // C 4686
	  if ! NILP (edges)                                           // C 4687
	    {
	      width = (XFIXNUM (Fnth (make_fixnum (2), edges))        // C 4688
		       - XFIXNUM (Fnth (make_fixnum (0), edges))) as c_int;   // C 4689
	    }
	}

      if ! p.is_null ()                                               // C 4692
	{
	  (*f).left_pos = (FRAME_PIXEL_WIDTH! (p) - width - 2 * (*f).border_width   // C 4693
			   + (*f).left_pos);                          // C 4694
	}
      else
	{ (*f).left_pos = (screen_width - width + (*f).left_pos); }   // C 4696

    }

  if (flags & YNegative) != 0                                         // C 4700
    {
      let mut height: c_int = FRAME_PIXEL_HEIGHT! (f);                // C 4702

      if p.is_null ()                                                 // C 4704
	{
	  let mut frame: Lisp_Object;                                 // C 4706
	  let mut edges: Lisp_Object = Qnil ();                       // C 4707

	  XSETFRAME! (frame, f);                                      // C 4709
	  if NILP (edges)                                             // C 4710
	    { edges = Fhaiku_frame_edges (frame, Qouter_edges ()); }  // C 4711
	  if ! NILP (edges)                                           // C 4712
	    {
	      height = (XFIXNUM (Fnth (make_fixnum (3), edges))       // C 4713
			- XFIXNUM (Fnth (make_fixnum (1), edges))) as c_int;   // C 4714
	    }
	}

      if ! p.is_null ()                                               // C 4717
	{
	  (*f).top_pos = (FRAME_PIXEL_HEIGHT! (p) - height - 2 * (*f).border_width   // C 4718
			  + (*f).top_pos);                            // C 4719
	}
      else
	{ (*f).top_pos = (screen_height - height + (*f).top_pos); }   // C 4721
    }

  /* The left_pos and top_pos
     are now relative to the top and left screen edges,
     so the flags should correspond.  */                             // C 4724-4726
  (*f).size_hint_flags &= ! (XNegative | YNegative);                  // C 4727
}

// C 4730-4770.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_set_offset (frame: *mut frame, x: c_int, y: c_int,
					   change_gravity: c_int)
{
  let mut lframe: Lisp_Object;                                        // C 4734

  /* Don't allow moving a fullscreen frame: the semantics of that are
     unclear.  */                                                    // C 4736-4737

  XSETFRAME! (lframe, frame);                                         // C 4739
  if EQ (Fframe_parameter (lframe, Qfullscreen ()), Qfullboth ())     // C 4740
      /* Only do this if the fullscreen status has actually been
	 applied.  */                                                 // C 4741-4742
      && want_fullscreen! (frame) == FULLSCREEN_NONE                  // C 4743
      /* And if the configury during frame creation has been
	 completed.  Otherwise, there will be no valid "old position"
	 to go back to.  */                                           // C 4744-4746
      && (*(FRAME_OUTPUT_DATA! (frame) as *mut haiku_output)).configury_done   // C 4747
    { return; }                                                       // C 4748

  if change_gravity > 0                                               // C 4750
    {
      (*frame).top_pos = y;                                           // C 4752
      (*frame).left_pos = x;                                          // C 4753
      (*frame).size_hint_flags &= ! (XNegative | YNegative);          // C 4754
      if x < 0                                                        // C 4755
	{ (*frame).size_hint_flags |= XNegative; }                    // C 4756
      if y < 0                                                        // C 4757
	{ (*frame).size_hint_flags |= YNegative; }                    // C 4758
      (*frame).win_gravity = NorthWestGravity;                        // C 4759
    }

  haiku_update_size_hints (frame);                                    // C 4762
  haiku_calc_absolute_position (frame);                               // C 4763

  block_input ();                                                     // C 4765
  if change_gravity != 0                                              // C 4766
    {
      BWindow_set_offset (FRAME_HAIKU_WINDOW! (frame), (*frame).left_pos,   // C 4767
			  (*frame).top_pos);                          // C 4768
    }
  unblock_input ();                                                   // C 4769
}

/* C 4772-4793's `USE_BE_CAIRO` pair.  Both are non-`static` in C, so the port keeps
   `#[unsafe(no_mangle)] pub unsafe extern "C" fn` under the same condition the block carries -
   the arm `be_cairo`, which `xtask probe --cfg 'feature="be_cairo"'` can flip (the ledger's
   `## Slice 27` records that reading).  */

// C 4772-4783.
#[cfg (feature = "be_cairo")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_begin_cr_clip (f: *mut frame, s: *mut glyph_string)
						    -> *mut cairo_t
{
  let cr: *mut cairo_t = FRAME_CR_CONTEXT! (f);                        // C 4776

  if cr.is_null ()                                                    // C 4778
    { return std::ptr::null_mut (); }                                 // C 4779

  cairo_save (cr);                                                    // C 4781
  return cr;                                                          // C 4782
}

// C 4785-4792.
#[cfg (feature = "be_cairo")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_end_cr_clip (cr: *mut cairo_t)
{
  if cr.is_null ()                                                    // C 4788
    { return; }                                                       // C 4789

  cairo_restore (cr);                                                 // C 4791
}

// C 4795-4821.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_merge_cursor_foreground (s: *mut glyph_string,
							foreground_out: *mut c_ulong,
							background_out: *mut c_ulong)
{
  let mut background: c_ulong = FRAME_CURSOR_COLOR! ((*s).f).pixel;    // C 4800
  let mut foreground: c_ulong = (*GS_FACE! (s)).background;            // C 4801

  if background == foreground                                         // C 4803
    { foreground = (*GS_FACE! (s)).background; }                      // C 4804
  if background == foreground                                         // C 4805
    { foreground = (*(FRAME_OUTPUT_DATA! ((*s).f) as *mut haiku_output)).cursor_fg as c_ulong; }  // C 4806
  if background == foreground                                         // C 4807
    { foreground = (*GS_FACE! (s)).foreground; }                      // C 4808

  if background == (*GS_FACE! (s)).background                         // C 4810
      && foreground == (*GS_FACE! (s)).foreground                     // C 4811
    {
      background = (*GS_FACE! (s)).foreground;                        // C 4813
      foreground = (*GS_FACE! (s)).background;                        // C 4814
    }

  if ! foreground_out.is_null ()                                      // C 4817
    { *foreground_out = foreground; }                                 // C 4818
  if ! background_out.is_null ()                                      // C 4819
    { *background_out = background; }                                 // C 4820
}

// C 4823-4898.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_haikuterm ()
{
  DEFVAR_BOOL! (c"haiku-initialized", haiku_initialized,
     doc: "Non-nil if the Haiku terminal backend has been initialized.  ");   // C 4826-4827

  DEFVAR_BOOL! (c"x-use-underline-position-properties",
		x_use_underline_position_properties,                        // C 4829-4830
     doc: "SKIP: real doc in xterm.c.  ");                            // C 4831
  x_use_underline_position_properties = true;                          // C 4832

  DEFVAR_BOOL! (c"x-underline-at-descent-line",
		x_underline_at_descent_line,                                // C 4834-4835
     doc: "SKIP: real doc in xterm.c.  ");                            // C 4836
  x_underline_at_descent_line = false;                                // C 4837

  DEFVAR_LISP! (c"x-toolkit-scroll-bars", Vx_toolkit_scroll_bars,     // C 4839
     doc: "SKIP: real doc in xterm.c.  ");                            // C 4840
  Vx_toolkit_scroll_bars = Qt ();                                     // C 4841

  DEFVAR_BOOL! (c"haiku-debug-on-fatal-error", haiku_debug_on_fatal_error,   // C 4843
     doc: "If non-nil, Emacs will launch the system debugger upon a fatal error.  ");   // C 4844
  haiku_debug_on_fatal_error = true;                                  // C 4845

  DEFSYM! (Qshift, "shift");                                          // C 4847
  DEFSYM! (Qcontrol, "control");                                      // C 4848
  DEFSYM! (Qoption, "option");                                        // C 4849
  DEFSYM! (Qcommand, "command");                                      // C 4850

  DEFSYM! (Qdata_directory, "data-directory");                        // C 4852

  DEFVAR_LISP! (c"haiku-meta-keysym", Vhaiku_meta_keysym,             // C 4854
     doc: "Which key Emacs uses as the meta modifier.\nThis is either one of the symbols `shift', `control', `command', and\n`option', or nil, in which case it is treated as `command'.\n\nSetting it to any other value is equivalent to `command'.  ");   // C 4855-4859
  Vhaiku_meta_keysym = Qnil ();                                       // C 4860

  DEFVAR_LISP! (c"haiku-control-keysym", Vhaiku_control_keysym,       // C 4862
     doc: "Which key Emacs uses as the control modifier.\nThis is either one of the symbols `shift', `control', `command', and\n`option', or nil, in which case it is treated as `control'.\n\nSetting it to any other value is equivalent to `control'.  ");   // C 4863-4867
  Vhaiku_control_keysym = Qnil ();                                    // C 4868

  DEFVAR_LISP! (c"haiku-super-keysym", Vhaiku_super_keysym,           // C 4870
     doc: "Which key Emacs uses as the super modifier.\nThis is either one of the symbols `shift', `control', `command', and\n`option', or nil, in which case it is treated as `option'.\n\nSetting it to any other value is equivalent to `option'.  ");   // C 4871-4875
  Vhaiku_super_keysym = Qnil ();                                      // C 4876

  DEFVAR_LISP! (c"haiku-shift-keysym", Vhaiku_shift_keysym,           // C 4878
     doc: "Which key Emacs uses as the shift modifier.\nThis is either one of the symbols `shift', `control', `command', and\n`option', or nil, in which case it is treated as `shift'.\n\nSetting it to any other value is equivalent to `shift'.  ");   // C 4879-4883
  Vhaiku_shift_keysym = Qnil ();                                      // C 4884

  DEFSYM! (Qx_use_underline_position_properties,                      // C 4886
	   "x-use-underline-position-properties");                    // C 4887

  DEFSYM! (Qx_underline_at_descent_line, "x-underline-at-descent-line");   // C 4889

  rdb = Qnil ();                                                      // C 4891
  staticpro (&raw mut rdb);                                           // C 4892

  Fprovide (Qhaiku (), Qnil ());                                      // C 4894
  #[cfg (feature = "be_cairo")]
  { Fprovide (intern_c_string (c"cairo".as_ptr ()), Qnil ()); }       // C 4895-4897
}
/* C 4571-4618: `haiku_clear_under_internal_border` - **landed out of order** (the ledger's C range
   table records the row and the gap around it): the `haiku_redisplay_interface` object's
   twenty-first entry (C 3162) names this function, and an object cannot reference a name that does
   not exist yet, so C 4571-4618 is written here instead of waiting for the prefix to reach it;
   C 3207-4570 and C 4619-4898 stay owed.  C's `FRAME_INTERNAL_BORDER_WIDTH`/`FRAME_PIXEL_*`/
   `FRAME_*_MARGIN_HEIGHT`/`FRAME_PARENT_FRAME` are `include/frame.rs`'s readers, and the
   `if (… && FRAME_FACE_CACHE (f))` guard is the port's `! … .is_null ()` on this file's own
   `FRAME_FACE_CACHE` (C 1379).  `haikuterm.h` 236 declares this one, so the unit's definition
   shadows the mirror's `haikuterm_decls` row (PORTING.md 0.1-10/P-21).  */
// C 4572-4618.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_clear_under_internal_border (f: *mut frame)
{
  if FRAME_INTERNAL_BORDER_WIDTH (f) > 0                              // C 4574
      /* This is needed because tooltip frames set up the internal
	 border before init_frame_faces.  */                         // C 4575-4576
      && ! FRAME_FACE_CACHE (f).is_null ()                            // C 4577
    {
      let border: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);            // C 4579
      let width: c_int = FRAME_PIXEL_WIDTH (f);                       // C 4580
      let height: c_int = FRAME_PIXEL_HEIGHT (f);                     // C 4581
      let margin: c_int = FRAME_TOP_MARGIN_HEIGHT (f);                // C 4582
      let bottom_margin: c_int = FRAME_BOTTOM_MARGIN_HEIGHT (f);      // C 4583
      let face_id: c_int = if ! FRAME_PARENT_FRAME (f).is_null ()     // C 4584
	{ if ! NILP (Vface_remapping_alist)                          // C 4585
	    { lookup_basic_face (std::ptr::null_mut (), f,            // C 4586
				 CHILD_FRAME_BORDER_FACE_ID) }       // C 4587
	  else
	    { CHILD_FRAME_BORDER_FACE_ID } }                         // C 4588
	else
	{ if ! NILP (Vface_remapping_alist)                          // C 4589
	    { lookup_basic_face (std::ptr::null_mut (), f,            // C 4590
				 INTERNAL_BORDER_FACE_ID) }          // C 4591
	  else
	    { INTERNAL_BORDER_FACE_ID } };                           // C 4592
      let face: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);        // C 4593
      let view: *mut c_void = FRAME_HAIKU_DRAWABLE! (f);              // C 4594

      block_input ();                                                 // C 4596
      BView_draw_lock (view, true, 0, 0, FRAME_PIXEL_WIDTH (f),       // C 4597
		       FRAME_PIXEL_HEIGHT (f));                     // C 4598
      BView_StartClip (view);                                         // C 4599
      BView_ClipToRect (view, 0, 0, FRAME_PIXEL_WIDTH (f),            // C 4600
			FRAME_PIXEL_HEIGHT (f));                    // C 4601

      if ! face.is_null ()                                            // C 4603
	{ BView_SetHighColor (view, (*face).background as uint32_t); }  // C 4604
      else
	{ BView_SetHighColor (view, FRAME_BACKGROUND_PIXEL (f) as uint32_t); }  // C 4606

      BView_FillRectangle (view, 0, margin, width, border);           // C 4608
      BView_FillRectangle (view, 0, 0, border, height);               // C 4609
      BView_FillRectangle (view, 0, margin, width, border);           // C 4610
      BView_FillRectangle (view, width - border, 0, border, height);  // C 4611
      BView_FillRectangle (view, 0, height - bottom_margin - border,  // C 4612
			   width, border);                          // C 4613
      BView_EndClip (view);                                           // C 4614
      BView_draw_unlock (view);                                       // C 4615
      unblock_input ();                                               // C 4616
    }
}
