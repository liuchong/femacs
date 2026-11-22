// femacs: port of src/fontset.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names
// (function, macro, file-scope object, local), the same control flow and the same error
// model.  The C `static` helpers stay plain `unsafe fn`s (the one C hands to `qsort` by
// address is `unsafe extern "C" fn`), and the non-`static` definitions carry
// `#[unsafe(no_mangle)] pub` (PORTING.md 0.2).
//
// This file is P-86's class: the frozen configuration compiles no `fontset.o`
// (`configure.ac` 7515-7521 puts it in `WINDOW_SYSTEM_OBJ` only when
// `window_system != none`, and `build/emacs-ref/src/Makefile` 303 reads
// `WINDOW_SYSTEM_OBJ=` empty), so `extract-abi.py`'s checklist for the file is empty and
// P-86 ① carries **both arms** of every `#if` instead.  The file gate is C's own
// `HAVE_WINDOW_SYSTEM` -> `feature = "have_window_system"`, on every item *and on C 31-42's
// include graph* (P-86 ⑧), which is why the frozen configuration compiles nothing here.
//
// The conditions in C, in order: C 38-40 (`HAVE_WINDOW_SYSTEM`, the `TERM_HEADER` include),
// C 955-968 and C 1306-1351 (`#if 0`: dead in every configuration, recorded in the ledger and
// not transcribed, the `src/ccl.rs` 35-40 rule), C 1688-1696 (`HAVE_WINDOW_SYSTEM` again) and
// C 2103-2152 / C 2254-2256 (`ENABLE_CHECKING` -> `feature = "enable_checking"`, which the
// frozen table leaves undefined).
//
// `docs/ports/fontset.md` carries the C line ranges, the acceptance readings, the use-site
// declarations the mirrors cannot supply yet and every deviation.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe, unused_assignments, unused_parens, unused_mut, unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals, unused_must_use, clippy::all)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();

/* C 28-29 (`<stdio.h>`, `<stdlib.h>`) and the two headers lisp.h reaches for this file
   (`<limits.h>` for `INT_MIN`/`INT_MAX`, `<string.h>` for `strchr`/`strcmp`/`strncmp`).
   These mirrors carry no `use` of their own (`docs/ports/include-stdio.md`: the scalars are
   the including unit's), so they sit under the file gate with the rest of the graph (P-86 ⑧):
   nothing in this unit is compiled without `have_window_system`.  */
#[cfg (feature = "have_window_system")]
const _: () = ();
#[cfg (feature = "have_window_system")]
const _: () = ();
#[cfg (feature = "have_window_system")]
const _: () = ();
#[cfg (feature = "have_window_system")]
const _: () = ();

/* C 31-42: `lisp.h`, `blockinput.h`, `character.h`, `charset.h`, `frame.h`, `dispextern.h`,
   `fontset.h`, the `HAVE_WINDOW_SYSTEM` `TERM_HEADER`, `font.h` and `pdumper.h`.  No
   translation unit the frozen configuration builds reaches any of them, so the whole graph
   sits under the file gate (P-86 ⑧).  `TERM_HEADER` is the platform terminal header
   (`configure.ac` 7180; the frozen table spells it `undef`), and the six-arm chain below
   keeps C's include shape the way `src/frame.rs` 84-98 spells its own.  */
#[cfg (feature = "have_window_system")]
const _: () = ();
#[cfg (feature = "have_window_system")]
const _: () = ();
#[cfg (feature = "have_window_system")]
const _: () = ();

/* C 104-108's `INLINE int xstrcasecmp (...)`: `dispextern.h` gives every including unit its own
   copy, and the mirror keeps one in `dispextern_externs` (a module, so a unit with its own copy -
   `font.rs` 324 - does not collide).  The call below names that copy instead of an `extern "C"`
   declaration, which asked the linker for a symbol C never defines (status 4.723).  */
#[cfg (feature = "have_window_system")]
const _: () = ();
#[cfg (feature = "have_window_system")]
const _: () = ();
#[cfg (feature = "have_window_system")]
const _: () = ();
/* C 37: `#include "fontset.h"`.  The mirror (`include/fontset.rs`) declares the ten entry
   points in one top-level `unsafe extern "C" { ... }`, and nine of them are *defined* by this
   unit (C 861-1234, 1266, 1798), so including it here is rustc's `E0428: the name ... is
   defined multiple times` - P-39's class, which that rule fixes by putting such declarations
   in a `pub mod` the unit glob-imports.  The mirror is not this unit's file to edit
   (P-88 1/P-72 rule 4), so the one declaration this file needs from fontset.h
   (`face_for_font`) is in the use-site block below, and `docs/ports/fontset.md` reports the
   finding to the integrator.  */
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
const _: () = ();
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
const _: () = ();
#[cfg (all (feature = "have_window_system", feature = "have_pgtk"))]
const _: () = ();
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
const _: () = ();
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
const _: () = ();
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
const _: () = ();
#[cfg (feature = "have_window_system")]
const _: () = ();
#[cfg (feature = "have_window_system")]
const _: () = ();

/* `frame.h` 1379/1646-1661's three accessors: the mirror keeps them in its own
   `frame_face_accessors` module behind `dispextern_face_records` (which no configuration
   produces), and this unit calls them six times, so it carries the module itself - the
   `src/haikufns.rs` 857-876 device, and the shape status 3.220's ruling prescribes for every
   unit that needs one of the gated records.  */
/* `struct face_cache` (C dispextern.h 1991-2009) - the same use-site copy the record above
   is (status 3.220's ruling; the accessors below read its `used`/`faces_by_id`).  */
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

#[cfg (feature = "have_window_system")]
pub mod frame_face_accessors
{
  use super::*;

  /// C 1379: `#define FRAME_FACE_CACHE(F) (F)->face_cache`
  #[inline (always)]
  pub fn FRAME_FACE_CACHE (f: *mut frame) -> *mut face_cache
  { /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f).face_cache as *mut face_cache } }

  /// C 1646-1650: `INLINE struct face * FACE_FROM_ID (struct frame *f, int id)`
  #[inline (always)]
  pub fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
  {
    eassert! (0 <= id && id < /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*FRAME_FACE_CACHE (f)).used });
    unsafe { *(*FRAME_FACE_CACHE (f)).faces_by_id.add (id as usize) }
  }

  /// C 1656-1661: `INLINE struct face * FACE_FROM_ID_OR_NULL (struct frame *f, int id)`
  #[inline (always)]
  pub fn FACE_FROM_ID_OR_NULL (f: *mut frame, id: c_int) -> *mut face
  {
    let used: c_int = /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*FRAME_FACE_CACHE (f)).used };
    eassume! (0 <= used);
    if 0 <= id && id < used { unsafe { *(*FRAME_FACE_CACHE (f)).faces_by_id.add (id as usize) } }
    else { core::ptr::null_mut () }
  }
}

/* `struct face` (C dispextern.h 1787-1937) - the use-site copy the display units carry, for the
   reason the mirror's `dispextern_face_records` ruling records (status 3.220): the mirror
   keeps the record behind a `dead-arms` feature that no configuration produces, so on
   Android the name is out of scope (`src/xfaces.rs` 288 is the oldest copy).  The text is
   the mirror's; the assertions hold for both targets, and the Android arm's numbers are
   measured in docs/baseline/face-layout-android.tsv.  */
/// C 1787-1937: `struct face`.  Measured with the frozen clang: `sizeof` 280, align 8,
/// `lface`@0, `id`@160, `foreground`@168, `background`@176, `underline_color`@184,
/// `overline_color`@192, `strike_through_color`@200, `box_color`@208, `font`@216,
/// `fontset`@224, `box_vertical_line_width`@228, `box_horizontal_line_width`@232,
/// `underline_pixels_above_descent_line`@236, the 21 `bool_bf`/`ENUM_BF` bits C declares at
/// 1893-1935 in one 4-byte storage unit at 240, `hash`@248, `next`@256, `prev`@264,
/// `ascii_face`@272.
#[cfg (feature = "have_window_system")]
#[repr(C)]
#[cfg (feature = "have_window_system")]
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

#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::size_of::<face> () == 280 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::align_of::<face> () == 8);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, id) == 160);
#[cfg (all (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, gc) == 168);
#[cfg (all (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, stipple) == 176);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, foreground) == 168 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, background) == 176 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, underline_color) == 184 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, overline_color) == 192 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, strike_through_color) == 200 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, box_color) == 208 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, font) == 216 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, fontset) == 224 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, bits) == 240 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, hash) == 248 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, next) == 256 + FACE_WS_SHIFT);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, ascii_face) == 272 + FACE_WS_SHIFT);


/* Use-site declarations (P-88 ①): the C names this file reaches through the headers above
   that no mirror in this unit's include graph declares yet.  The frozen mirrors are not this
   unit's file to edit (P-72 rule 4), so each name is declared here with C's own prototype and
   the header and line it comes from; `docs/ports/fontset.md` reports the list to the
   integrator.  */
#[cfg (feature = "have_window_system")]
unsafe extern "C-unwind" {
                                                          // stdlib.in.h 237
          // lisp.h 4271
  // lisp.h 4274
                  // lisp.h 4245
            // frame.h 1476
                             // frame.h 1504
  pub fn recompute_basic_faces (f: *mut frame);       // dispextern.h 104
           // font.h 851
  pub fn font_has_char (f: *mut frame, font: Lisp_Object, c: c_int) -> c_int;  // font.h 867
     // font.h 860
                // font.h 864
               // font.h 839
  pub fn font_find_for_lface (f: *mut frame, lface: *mut Lisp_Object, spec: Lisp_Object,
                              c: c_int) -> Lisp_Object;                   // font.h 871
  pub fn font_open_for_lface (f: *mut frame, entity: Lisp_Object,
                              lface: *mut Lisp_Object, spec: Lisp_Object) -> Lisp_Object;
                                                                          // font.h 873
  pub fn font_load_for_lface (f: *mut frame, lface: *mut Lisp_Object,
                              spec: Lisp_Object) -> Lisp_Object;          // font.h 876
  pub fn font_open_by_spec (f: *mut frame, spec: Lisp_Object) -> Lisp_Object;  // font.h 882
             // font.h 889
               // font.h 893
         // font.h 894
                   // frame.h 1842
  pub fn face_for_font (f: *mut frame, font_object: Lisp_Object,
                        face: *mut face) -> c_int;
}
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn qsort (base: *mut c_void, nmemb: usize, size: usize,
                compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);                           // dispextern.h 3823
  pub fn xstrcasecmp (a: *const c_char, b: *const c_char) -> c_int;                       // fontset.h 46
}

/* The `DEFVAR_LISP`/`DEFVAR_BOOL` variables of this file.  C's storage lives in the generated
   `globals.h` (`globals.f_...`) and `syms_of_fontset` registers the addresses; the port's
   single definition point is here (PORTING.md 0.1-3, the `src/coding.rs` 250 shape), where it
   shadows the `variables::*` declarations `include/globals.rs` glob-imports.  C's initial
   value is the BSS zero until `syms_of_fontset` assigns `Qnil` (C 2197 onwards), so the
   Lisp_Object ones start null; `use_default_font_for_symbols` is C's tentative `0`.  */
#[cfg (feature = "have_window_system")]
pub static Vfont_encoding_charset_alist: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());
/* C's `DEFVAR_LISP ("use-default-ascent", Vuse_default_ascent, ...)` (fontset.c 2196) defines a
   global symbol, so the port exports it (status 4.777: `xdisp.rs`'s `gui_produce_glyphs` reads it
   and the Android reading called the unexported definition a defect).  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub static Vuse_default_ascent: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());
#[cfg (feature = "have_window_system")]
pub static use_default_font_for_symbols: VmGlobal<bool> = VmGlobal::<bool>::new (false);                    // C 2209
/* Same shape as `Vuse_default_ascent` above (C fontset.c 2216).  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub static Vignore_relative_composition: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());
#[cfg (feature = "have_window_system")]
pub static Valternate_fontname_alist: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());
#[cfg (feature = "have_window_system")]
pub static Vfontset_alias_alist: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());
#[cfg (feature = "have_window_system")]
/* C's `DEFVAR_LISP` in `syms_of_fontset` defines this global; `sfntfont.rs` references it from
   another unit, so the C symbol has to exist (PORTING.md P-313, status 4.722).  */
#[unsafe(no_mangle)]
pub static Vvertical_centering_font_regexp: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub static Votf_script_alist: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());

/* C 1736's `<intprops.h>` is reached through lisp.h 30 and the mirror is blind to it, so the
   unit declares what it uses (P-88 ①, the `src/android.rs` 110-121 shape).  The C expression
   is gnulib's `INT_STRLEN_BOUND (ptrdiff_t)`; the Rust primitive is the same bound over
   `size_of::<$t>`: `(64 - 1) * 146 + 484) / 485 + 1` = 20 for `ptrdiff_t`, which is what the
   ledger's probe reads.  */
#[cfg (feature = "have_window_system")]
macro_rules! INT_STRLEN_BOUND {
  ($t:ty) => { ((std::mem::size_of::<$t> () * 8 - 1) * 146 + 484) / 485 + 1 };
}

/* FONTSET

   A fontset is a collection of font related information to give
   similar appearance (style, etc) of characters.  A fontset has two
   roles.  One is to use for the frame parameter `font' as if it is an
   ASCII font.  In that case, Emacs uses the font specified for
   `ascii' script for the frame's default font.

   Another role, the more important one, is to provide information
   about which font to use for each non-ASCII character.

   There are two kinds of fontsets; base and realized.  A base fontset
   is created by `new-fontset' from Emacs Lisp explicitly.  A realized
   fontset is created implicitly when a face is realized for ASCII
   characters.  A face is also realized for non-ASCII characters based
   on an ASCII face.  All of non-ASCII faces based on the same ASCII
   face share the same realized fontset.

   A fontset object is implemented by a char-table whose default value
   and parent are always nil.

   An element of a base fontset is a vector of FONT-DEFs which themselves
   are vectors of the form [ FONT-SPEC ENCODING REPERTORY ].

   An element of a realized fontset is nil, t, 0, or a cons that has
   this from:

	(CHARSET-ORDERED-LIST-TICK . FONT-GROUP)

   CHARSET_ORDERED_LIST_TICK is the same as charset_ordered_list_tick or -1.

   FONT-GROUP is a vector of elements that have this form:

	[ RFONT-DEF0 RFONT-DEF1 ... ]

   Each RFONT-DEFn (i.e. Realized FONT-DEF) has this form:

	[ FACE-ID FONT-DEF FONT-OBJECT SORTING-SCORE ]

   RFONT-DEFn are automatically reordered considering the current
   charset priority list, the current language environment, and
   priorities determined by font-backends.

   RFONT-DEFn may not be a vector in the following cases.

   The value nil means that we have not yet generated the above vector
   from the base of the fontset.

   The value t means that no font is available for the corresponding
   range of characters.

   The value 0 means that no font is available for the corresponding
   range of characters in this fontset, but may be available in the
   fallback font-group or in the default fontset.

   A fontset has 8 extra slots.

   The 1st slot:
	base: the ID number of the fontset
	realized: Likewise

   The 2nd slot:
	base: the name of the fontset
	realized: nil

   The 3rd slot:
	base: the font name for ASCII characters
	realized: nil

   The 4th slot:
	base: nil
	realized: the base fontset

   The 5th slot:
	base: nil
	realized: the frame that the fontset belongs to

   The 6th slot:
	base: nil
	realized: the ID number of a face to use for characters that
		  has no font in a realized fontset.

   The 7th slot:
	base: nil
	realized: If the base is not the default fontset, a fontset
		  realized from the default fontset, else nil.

   The 8th slot:
	base: Same as element value (but for fallback fonts).
	realized: Likewise.

   All fontsets are recorded in the vector Vfontset_table.


   DEFAULT FONTSET

   There's a special base fontset named `default fontset' which
   defines the default font specifications.  When a base fontset
   doesn't specify a font for a specific character, the corresponding
   value in the default fontset is used.

   The parent of a realized fontset created for such a face that has
   no fontset is the default fontset.


   These structures are hidden from the other codes than this file.
   The other codes handle fontsets only by their ID numbers.  They
   usually use the variable name `fontset' for IDs.  But, in this
   file, we always use variable name `id' for IDs, and name `fontset'
   for an actual fontset object, i.e., char-table.

*/

/********** VARIABLES and FUNCTION PROTOTYPES **********/

/* Vector containing all fontsets.  */
#[cfg (feature = "have_window_system")]
static Vfontset_table: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());            // C 160

/* Next possibly free fontset ID.  Usually this keeps the minimum
   fontset ID not yet used.  */
#[cfg (feature = "have_window_system")]
static next_fontset_id: VmGlobal<c_int> = VmGlobal::<c_int>::new (0);                                     // C 164

/* The default fontset.  This gives default FAMILY and REGISTRY of
   font for each character.  */
#[cfg (feature = "have_window_system")]
static Vdefault_fontset: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());          // C 168

/* C 171: `static Lisp_Object make_fontset (Lisp_Object, Lisp_Object, Lisp_Object);` - a
   forward declaration with no Rust item (Rust is order-free; the definition is C 822).  */

/* Return true if ID is a valid fontset id.
   Optimized away if ENABLE_CHECKING is not defined.  */

#[cfg (feature = "have_window_system")]
unsafe fn fontset_id_valid_p (id: c_int) -> bool                           // C 177
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  return id >= 0 && (id as isize) < ASIZE ((*Vfontset_table.as_mut_ptr ())) - 1;

  }
}


/********** MACROS AND FUNCTIONS TO HANDLE FONTSET **********/

/* Return the fontset with ID.  No check of ID's validness.  */
#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_FROM_ID { ($id:expr) => { AREF ((*Vfontset_table.as_mut_ptr ()), $id) }; }   // C 186

/* Access special values of FONTSET.  */

#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_ID {
  ($fontset:expr) => { FAM_ELEM! ((*XCHAR_TABLE ($fontset)).extras, 0) };  // C 190
}
#[cfg (feature = "have_window_system")]
unsafe fn set_fontset_id (fontset: Lisp_Object, id: Lisp_Object)            // C 193
{
  set_char_table_extras (fontset, 0, id);
}

/* Access special values of (base) FONTSET.  */

#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_NAME {
  ($fontset:expr) => { FAM_ELEM! ((*XCHAR_TABLE ($fontset)).extras, 1) };  // C 202
}
#[cfg (feature = "have_window_system")]
unsafe fn set_fontset_name (fontset: Lisp_Object, name: Lisp_Object)        // C 202
{
  set_char_table_extras (fontset, 1, name);
}

#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_ASCII {
  ($fontset:expr) => { FAM_ELEM! ((*XCHAR_TABLE ($fontset)).extras, 2) };  // C 209
}
#[cfg (feature = "have_window_system")]
unsafe fn set_fontset_ascii (fontset: Lisp_Object, ascii: Lisp_Object)      // C 209
{
  set_char_table_extras (fontset, 2, ascii);
}

/* Access special values of (realized) FONTSET.  */

#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_BASE {
  ($fontset:expr) => { FAM_ELEM! ((*XCHAR_TABLE ($fontset)).extras, 3) };  // C 218
}
#[cfg (feature = "have_window_system")]
unsafe fn set_fontset_base (fontset: Lisp_Object, base: Lisp_Object)        // C 218
{
  set_char_table_extras (fontset, 3, base);
}

#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_FRAME {
  ($fontset:expr) => { FAM_ELEM! ((*XCHAR_TABLE ($fontset)).extras, 4) };  // C 225
}
#[cfg (feature = "have_window_system")]
unsafe fn set_fontset_frame (fontset: Lisp_Object, frame: Lisp_Object)      // C 225
{
  set_char_table_extras (fontset, 4, frame);
}

#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_NOFONT_FACE {
  ($fontset:expr) => { FAM_ELEM! ((*XCHAR_TABLE ($fontset)).extras, 5) };  // C 232
}
#[cfg (feature = "have_window_system")]
unsafe fn set_fontset_nofont_face (fontset: Lisp_Object, face: Lisp_Object) // C 232
{
  set_char_table_extras (fontset, 5, face);
}

#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_DEFAULT {
  ($fontset:expr) => { FAM_ELEM! ((*XCHAR_TABLE ($fontset)).extras, 6) };  // C 239
}
#[cfg (feature = "have_window_system")]
unsafe fn set_fontset_default (fontset: Lisp_Object, def: Lisp_Object)      // C 239
{
  set_char_table_extras (fontset, 6, def);
}

/* For both base and realized fontset.  */

#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_FALLBACK {
  ($fontset:expr) => { FAM_ELEM! ((*XCHAR_TABLE ($fontset)).extras, 7) };  // C 248
}
#[cfg (feature = "have_window_system")]
unsafe fn set_fontset_fallback (fontset: Lisp_Object, fallback: Lisp_Object) // C 248
{
  set_char_table_extras (fontset, 7, fallback);
}

#[cfg (feature = "have_window_system")]
macro_rules! BASE_FONTSET_P {
  ($fontset:expr) => { NILP (FONTSET_BASE! ($fontset)) };                   // C 251
}

/* Definitions for FONT-DEF and RFONT-DEF of fontset.  */
#[cfg (feature = "have_window_system")]
unsafe fn font_def_new (font_spec: Lisp_Object, encoding: Lisp_Object,       // C 257
                        repertory: Lisp_Object) -> Lisp_Object
{
  return CALLN! (Fvector, font_spec, encoding, repertory);
}

#[cfg (feature = "have_window_system")]
macro_rules! FONT_DEF_SPEC { ($font_def:expr) => { AREF ($font_def, 0) }; }  // C 263
#[cfg (feature = "have_window_system")]
macro_rules! FONT_DEF_ENCODING { ($font_def:expr) => { AREF ($font_def, 1) }; }  // C 264
#[cfg (feature = "have_window_system")]
macro_rules! FONT_DEF_REPERTORY { ($font_def:expr) => { AREF ($font_def, 2) }; } // C 265

#[cfg (feature = "have_window_system")]
macro_rules! RFONT_DEF_FACE { ($rfont_def:expr) => { AREF ($rfont_def, 0) }; }   // C 267
#[cfg (feature = "have_window_system")]
macro_rules! RFONT_DEF_SET_FACE {
  ($rfont_def:expr, $face_id:expr) => { ASET ($rfont_def, 0, make_fixnum ($face_id)) }; // C 268
}
#[cfg (feature = "have_window_system")]
macro_rules! RFONT_DEF_FONT_DEF { ($rfont_def:expr) => { AREF ($rfont_def, 1) }; } // C 270
#[cfg (feature = "have_window_system")]
macro_rules! RFONT_DEF_SPEC {
  ($rfont_def:expr) => { FONT_DEF_SPEC! (AREF ($rfont_def, 1)) };          // C 271
}
#[cfg (feature = "have_window_system")]
macro_rules! RFONT_DEF_OBJECT { ($rfont_def:expr) => { AREF ($rfont_def, 2) }; } // C 272
#[cfg (feature = "have_window_system")]
macro_rules! RFONT_DEF_SET_OBJECT {
  ($rfont_def:expr, $object:expr) => { ASET ($rfont_def, 2, $object) };   // C 273
}
/* Score of RFONT_DEF is an integer value; the lowest 8 bits represent
   the order of listing by font backends, the higher bits represents
   the order given by charset priority list.  The smaller value is
   preferable.  */
#[cfg (feature = "have_window_system")]
macro_rules! RFONT_DEF_SCORE {
  ($rfont_def:expr) => { XFIXNUM (AREF ($rfont_def, 3)) };               // C 278
}
#[cfg (feature = "have_window_system")]
macro_rules! RFONT_DEF_SET_SCORE {
  ($rfont_def:expr, $score:expr) => { ASET ($rfont_def, 3, make_fixnum ($score)) }; // C 279
}
/* C 280-285's `do { … } while (false);` cannot be `macro_rules!` in Rust without naming the
   caller's local (the `RFONT_DEF_NEW` sites all pass the variable the macro assigns), so the
   macro takes it as its first metavariable; the statements are C's, in C's order.  */
#[cfg (feature = "have_window_system")]
macro_rules! RFONT_DEF_NEW {
  ($rfont_def:ident, $font_def:expr) => {
    {
      $rfont_def = make_nil_vector (4);
      ASET ($rfont_def, 1, $font_def);
      RFONT_DEF_SET_SCORE! ($rfont_def, 0);
    }
  };
}


/* Return the element of FONTSET for the character C.  If FONTSET is a
   base fontset other then the default fontset and FONTSET doesn't
   contain information for C, return the information in the default
   fontset.  */

#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_REF {
  ($fontset:expr, $c:expr) => {
    /* C's `c` is an int: `CHAR_TABLE_REF` takes a ptrdiff_t and `fontset_ref` an int, which is
       the one cast the two expansions need.  */
    if EQ ($fontset, (*Vdefault_fontset.as_mut_ptr ())) { CHAR_TABLE_REF ($fontset, $c as isize) }
    else { fontset_ref ($fontset, $c) }
  };                                                                      // C 294
}

#[cfg (feature = "have_window_system")]
unsafe fn fontset_ref (fontset: Lisp_Object, c: c_int) -> Lisp_Object      // C 301
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut elt: Lisp_Object;

  elt = CHAR_TABLE_REF (fontset, c as isize);
  if NILP (elt) && ! EQ (fontset, (*Vdefault_fontset.as_mut_ptr ()))
      /* Don't check Vdefault_fontset for a realized fontset.  */
      && NILP (FONTSET_BASE! (fontset))
    { elt = CHAR_TABLE_REF ((*Vdefault_fontset.as_mut_ptr ()), c as isize); }
  return elt;

  }
}

/* Set elements of FONTSET for characters in RANGE to the value ELT.
   RANGE is a cons (FROM . TO), where FROM and TO are character codes
   specifying a range.  */

#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_SET {
  ($fontset:expr, $range:expr, $elt:expr) => {
    Fset_char_table_range ($fontset, $range, $elt)
  };                                                                      // C 315
}


/* Modify the elements of FONTSET for characters in RANGE by replacing
   with ELT or adding ELT.  RANGE is a cons (FROM . TO), where FROM
   and TO are character codes specifying a range.  If ADD is nil,
   replace with ELT, if ADD is `prepend', prepend ELT, otherwise,
   append ELT.  */

#[cfg (feature = "have_window_system")]
macro_rules! FONTSET_ADD {
  ($fontset:expr, $range:expr, $elt:expr, $add:expr) => {
    if NILP ($add) {
      if NILP ($range) {
        set_fontset_fallback ($fontset, make_vector (1, $elt));
      } else {
        Fset_char_table_range ($fontset, $range, make_vector (1, $elt));
      }
    } else {
      fontset_add ($fontset, $range, $elt, $add);
    }
  };                                                                      // C 325
}

#[cfg (feature = "have_window_system")]
unsafe fn fontset_add (fontset: Lisp_Object, range: Lisp_Object, elt: Lisp_Object, // C 335
                       add: Lisp_Object)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut args: [Lisp_Object; 2] = [Qnil (), Qnil ()];
  let mut idx: c_int = if EQ (add, Qappend ()) { 0 } else { 1 };

  args[(1 - idx) as usize] = make_vector (1, elt);

  if CONSP (range)
    {
      let mut from: c_int = XFIXNUM (XCAR (range)) as c_int;
      let to: c_int = XFIXNUM (XCDR (range)) as c_int;
      let mut from1: c_int = 0;
      let mut to1: c_int = 0;

      loop {
        from1 = from; to1 = to;
        args[idx as usize] = char_table_ref_and_range (fontset, from, &raw mut from1,
                                                       &raw mut to1);
        char_table_set_range (fontset, from, to1,
                              if NILP (args[idx as usize]) { args[(1 - idx) as usize] }
                              else { CALLMANY! (Fvconcat, args) });
        from = to1 + 1;
        if ! (from <= to) { break; }
      }
    }
  else
    {
      args[idx as usize] = FONTSET_FALLBACK! (fontset);
      set_fontset_fallback (fontset,
                            if NILP (args[idx as usize]) { args[(1 - idx) as usize] }
                            else { CALLMANY! (Fvconcat, args) });
    }

  }
}

#[cfg (feature = "have_window_system")]
unsafe extern "C" fn fontset_compare_rfontdef (val1: *const c_void,                        // C 367
                                               val2: *const c_void) -> c_int
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let v1: Lisp_Object = *(val1 as *const Lisp_Object);
  let v2: Lisp_Object = *(val2 as *const Lisp_Object);
  if NILP (v1) && NILP (v2)
    { return 0; }
  else if NILP (v1)
    { return INT_MIN; }
  else if NILP (v2)
    { return INT_MAX; }
  return (RFONT_DEF_SCORE! (v1) - RFONT_DEF_SCORE! (v2)) as c_int;

  }
}

/* Update a cons cell which has this form:
	(CHARSET-ORDERED-LIST-TICK . FONT-GROUP)
   where FONT-GROUP is of the form
	[ PREFERRED-RFONT-DEF RFONT-DEF0 RFONT-DEF1 ... ]
   Reorder RFONT-DEFs according to the current language, and update
   CHARSET-ORDERED-LIST-TICK.  */

#[cfg (feature = "have_window_system")]
unsafe fn reorder_font_vector (font_group: Lisp_Object, font: *mut font)   // C 387
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut vec: Lisp_Object;
  let mut font_object: Lisp_Object;
  let mut size: c_int;
  let mut i: c_int;
  let mut score_changed: bool = false;

  if ! font.is_null ()
    { XSETFONT! (font_object, font); }
  else
    { font_object = Qnil (); }

  vec = XCDR (font_group);
  size = ASIZE (vec) as c_int;
  /* Exclude the tailing nil element from the reordering.  */
  if NILP (AREF (vec, (size - 1) as isize))
    { size -= 1; }

  for i in 0..size
    {
      let rfont_def: Lisp_Object = AREF (vec, i as isize);
      if NILP (rfont_def)
	{ continue; }
      let font_def: Lisp_Object = RFONT_DEF_FONT_DEF! (rfont_def);
      let font_spec: Lisp_Object = FONT_DEF_SPEC! (font_def);
      let mut score: c_int = (RFONT_DEF_SCORE! (rfont_def) & 0xFF) as c_int;
      let otf_spec: Lisp_Object = Ffont_get (font_spec, QCotf ());

      if ! NILP (otf_spec)
	/* A font-spec with :otf is preferable regardless of encoding
	   and language..  */
	{ }
      else if ! font_match_p (font_spec, font_object)
	{
	  /* This spec does not match with FONT-OBJECT.  Make the
	     score larger.  */
	  /* The bit 0 of score is reserved for the
	     preference order determined by the times of
	     specifying a font.  */
	  score |= 0x100;
	}
      else
	{
	  let encoding: Lisp_Object = FONT_DEF_ENCODING! (font_def);

	  if ! NILP (encoding)
	    {
	      /* This spec specifies an encoding by a charset set
		 name.  Reflect the preference order of that charset
		 in the upper bits of SCORE.  */
	      let mut tail: Lisp_Object;

	      tail = (*Vcharset_ordered_list.as_mut_ptr ());
	      while ! EQ (tail, (*Vcharset_non_preferred_head.as_mut_ptr ())) && CONSP (tail)
		{
		  if EQ (encoding, XCAR (tail))
		    { break; }
		  else if score <= (min (INT_MAX as isize, MOST_POSITIVE_FIXNUM) - 0x100) as c_int
		    { score += 0x100; }
		  tail = XCDR (tail);
		}
	    }
	  else
	    {
	      /* This spec does not specify an encoding.  If the spec
		 specifies a language, and the language is not for the
		 current language environment, make the score
		 larger.  */
	      let lang: Lisp_Object = Ffont_get (font_spec, QClang ());

	      if ! NILP (lang)
		  && ! EQ (lang, (*Vcurrent_iso639_language.as_mut_ptr ()))
		  && (! CONSP ((*Vcurrent_iso639_language.as_mut_ptr ()))
		      || NILP (Fmemq (lang, (*Vcurrent_iso639_language.as_mut_ptr ()))))
		{ score |= 0x100; }
	    }
	}
      if RFONT_DEF_SCORE! (rfont_def) as c_int != score
	{
	  RFONT_DEF_SET_SCORE! (rfont_def, score as EMACS_INT);
	  score_changed = true;
	}
    }

  if score_changed
    { qsort (fam_base (core::ptr::addr_of_mut! ((*XVECTOR (vec)).contents)) as *mut c_void,
	     size as usize, word_size as usize, Some (fontset_compare_rfontdef)); }
  /* C's `&` mixes EMACS_UINT with EMACS_INT: the usual arithmetic conversions make both
     EMACS_UINT, and the result converts back to EMACS_INT (the port's two casts).  */
  let low_tick_bits: EMACS_INT
    = ((*charset_ordered_list_tick.as_mut_ptr ()) & (MOST_POSITIVE_FIXNUM as EMACS_UINT)) as EMACS_INT;
  XSETCAR (font_group, make_fixnum (low_tick_bits));

  }
}


/* Return a font-group (actually a cons (CHARSET_ORDERED_LIST_TICK
   . FONT-GROUP)) for character C or a fallback font-group in the
   realized fontset FONTSET.  The elements of FONT-GROUP are
   RFONT-DEFs.  The value may not be a cons.  See the comment at the
   head of this file for the detail of the return value.  */

#[cfg (feature = "have_window_system")]
unsafe fn fontset_get_font_group (fontset: Lisp_Object, c: c_int) -> Lisp_Object  // C 474
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut font_group: Lisp_Object;
  let mut base_fontset: Lisp_Object;
  let mut from: c_int = 0;
  let mut to: c_int = MAX_CHAR;

  eassert! (! BASE_FONTSET_P! (fontset));
  if c >= 0
    { font_group = CHAR_TABLE_REF (fontset, c as isize); }
  else
    { font_group = FONTSET_FALLBACK! (fontset); }
  if ! NILP (font_group)
    /* We have already realized FONT-DEFs of this font group for C or
       for fallback (FONT_GROUP is a cons), or we have already found
       that no appropriate font was found (FONT_GROUP is t or 0).  */
    { return font_group; }
  base_fontset = FONTSET_BASE! (fontset);
  if NILP (base_fontset)
    /* Actually we never come here because FONTSET is a realized one,
       and thus it should have a base.  */
    { font_group = Qnil (); }
  else if c >= 0
    { font_group = char_table_ref_and_range (base_fontset, c, &raw mut from,
                                             &raw mut to); }
  else
    { font_group = FONTSET_FALLBACK! (base_fontset); }

  /* FONT_GROUP not being a vector means that no fonts are specified
     for C, or the fontset does not have fallback fonts.  */
  if NILP (font_group)
    {
      font_group = make_fixnum (0);
      if c >= 0
	/* Record that FONTSET does not specify fonts for C.  As
	   there's a possibility that a font is found in a fallback
	   font group, we set 0 at the moment.  */
	{ char_table_set_range (fontset, from, to, font_group); }
      return font_group;
    }
  if ! VECTORP (font_group)
    { return font_group; }

  /* Now realize FONT-DEFs of this font group, and update the realized
     fontset FONTSET. */
  font_group = Fcopy_sequence (font_group);
  for i in 0..ASIZE (font_group)
    {
      if ! NILP (AREF (font_group, i))
      {
	let mut rfont_def: Lisp_Object = Qnil ();

	RFONT_DEF_NEW! (rfont_def, AREF (font_group, i));
	/* Remember the original order.  */
	RFONT_DEF_SET_SCORE! (rfont_def, i);
	ASET (font_group, i, rfont_def);
      }
    }
  font_group = Fcons (make_fixnum (-1), font_group);
  if c >= 0
    { char_table_set_range (fontset, from, to, font_group); }
  else
    { set_fontset_fallback (fontset, font_group); }
  return font_group;

  }
}

/* Return RFONT-DEF (vector) in the realized fontset FONTSET for the
   character C.  If no font is found, return Qnil or 0 if there's a
   possibility that the default fontset or the fallback font groups
   have a proper font, and return Qt if not.

   If a font is found but is not yet opened, open it (if FACE is not
   NULL) or return Qnil (if FACE is NULL).

   CHARSET_ID is a charset-id that must be preferred, or -1 meaning no
   preference.

   If FALLBACK, search only fallback fonts.  */

/* C's `found:` label (C 739) is a labelled block (PORTING.md 2.2): the three `goto found`
   statements below are `break 'found`, and the block's follower is the label's body - the
   `return Qnil` that C writes just above the label belongs to the block.  */
#[cfg (feature = "have_window_system")]
/* The C parameter `int charset_id` is spelled `charset_id_` here: `include/charset.rs` has a
   `pub const charset_id`, and a parameter whose name resolves to a constant is read as a
   refutable constant pattern (`E0005`).  The trailing underscore folds away in the token check
   (`body_token_seq`'s `rstrip ("_")`), so C's own name is what the comparison sees.  */
unsafe fn fontset_find_font (fontset: Lisp_Object, c: c_int, face: *mut face,   // C 550
			     charset_id_: c_int, fallback: bool) -> Lisp_Object
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut vec: Lisp_Object;
  let mut font_group: Lisp_Object;
  let mut i: c_int;
  let mut charset_matched: c_int = 0;
  let mut found_index: c_int;
  let f: *mut frame = if FRAMEP (FONTSET_FRAME! (fontset))
		      { XFRAME (FONTSET_FRAME! (fontset)) }
		      else { XFRAME ((*selected_frame.as_mut_ptr ())) };
  let mut rfont_def: Lisp_Object;

  font_group = fontset_get_font_group (fontset, if fallback { -1 } else { c });
  if ! CONSP (font_group)
    { return font_group; }
  vec = XCDR (font_group);
  if ASIZE (vec) == 0
    { return Qnil (); }

  if ASIZE (vec) > 1
    {
      if XFIXNUM (XCAR (font_group)) != (*charset_ordered_list_tick.as_mut_ptr ()) as EMACS_INT
	/* We have just created the font-group,
	   or the charset priorities were changed.  */
	{ reorder_font_vector (font_group, (*(*face).ascii_face).font as *mut font); }
      if charset_id_ >= 0
	{
	  let lcsetid: Lisp_Object = make_fixnum (charset_id_ as EMACS_INT);
	  /* Find a spec matching with CHARSET_ID to try it at first.  */
	  i = 0;
	  while i < ASIZE (vec) as c_int
	    {
	      let repertory: Lisp_Object;

	      rfont_def = AREF (vec, i as isize);
	      if NILP (rfont_def)
		{ break; }
	      repertory = FONT_DEF_REPERTORY! (RFONT_DEF_FONT_DEF! (rfont_def));

	      if EQ (repertory, lcsetid)
		{
		  charset_matched = i;
		  break;
		}
	      i += 1;
	    }
	}
    }

  /* Find the first available font in the vector of RFONT-DEF.  If
     CHARSET_MATCHED > 0, try the corresponding RFONT-DEF first, then
     try the rest.  */
  'found: {
  i = 0;
  while i < ASIZE (vec) as c_int
    {
      let mut font_def: Lisp_Object;
      let mut font_entity: Lisp_Object;
      let mut font_object: Lisp_Object;

      found_index = i;
      if i == 0
	{
	  if charset_matched > 0
	    {
	      /* Try the element matching with CHARSET_ID at first.  */
	      found_index = charset_matched;
	      /* Make this negative so that we don't come here in the
		 next loop.  */
	      charset_matched = - charset_matched;
	      /* We must try the first element in the next loop.  */
	      i = -1;
	    }
	}
      else if i == - charset_matched
	{
	  /* We have already tried this element and the followings
	     that have the same font specifications in the first
	     iteration.  So, skip them all.  */
	  rfont_def = AREF (vec, i as isize);
	  font_def = RFONT_DEF_FONT_DEF! (rfont_def);
	  loop {
	    if ! (i + 1 < ASIZE (vec) as c_int) { break; }
	    rfont_def = AREF (vec, (i + 1) as isize);
	    if NILP (rfont_def)
	      { break; }
	    if ! EQ (RFONT_DEF_FONT_DEF! (rfont_def), font_def)
	      { break; }
	    i += 1;
	  }
	  { i += 1; continue; }
	}

      rfont_def = AREF (vec, found_index as isize);
      if NILP (rfont_def)
	{
	  if i < 0
	    { i += 1; continue; }
	  /* This is a sign of not to try the other fonts.  */
	  return Qt ();
	}
      if FIXNUMP (RFONT_DEF_FACE! (rfont_def))
	  && XFIXNUM (RFONT_DEF_FACE! (rfont_def)) < 0
	/* We couldn't open this font last time.  */
	{ i += 1; continue; }

      font_object = RFONT_DEF_OBJECT! (rfont_def);
      if NILP (font_object)
	{
	  font_def = RFONT_DEF_FONT_DEF! (rfont_def);

	  if face.is_null ()
	    /* We have not yet opened the font.  */
	    { return Qnil (); }
	  /* Find a font best-matching with the spec without checking
	     the support of the character C.  That checking is costly,
	     and even without the checking, the found font supports C
	     in high possibility.  */
	  font_entity = font_find_for_lface (f, (*face).lface.as_mut_ptr (),
					     FONT_DEF_SPEC! (font_def), -1);
	  if NILP (font_entity)
	    {
	      /* Record that no font matches the spec.  */
	      RFONT_DEF_SET_FACE! (rfont_def, -1);
	      { i += 1; continue; }
	    }
	  font_object = font_open_for_lface (f, font_entity, (*face).lface.as_mut_ptr (),
					     FONT_DEF_SPEC! (font_def));

	  if NILP (font_object)
	    {
	      /* Something strange happened, perhaps because of a
		 Font-backend problem.  To avoid crashing, record
		 that this spec is unusable.  It may be better to find
		 another font of the same spec, but currently we don't
		 have such an API in font-backend.  */
	      RFONT_DEF_SET_FACE! (rfont_def, -1);
	      { i += 1; continue; }
	    }
	  RFONT_DEF_SET_OBJECT! (rfont_def, font_object);
	}

      if font_has_char (f, font_object, c) != 0
	{ break 'found; }

      /* Find a font already opened, matching with the current spec,
	 and supporting C. */
      font_def = RFONT_DEF_FONT_DEF! (rfont_def);
      while found_index + 1 < ASIZE (vec) as c_int
	{
	  rfont_def = AREF (vec, (found_index + 1) as isize);
	  if NILP (rfont_def)
	    { break; }
	  if ! EQ (RFONT_DEF_FONT_DEF! (rfont_def), font_def)
	    { break; }
	  font_object = RFONT_DEF_OBJECT! (rfont_def);
	  if ! NILP (font_object) && font_has_char (f, font_object, c) != 0
	    {
	      found_index += 1;
	      { break 'found; }
	    }
	  found_index += 1;
	}

      /* Find a font-entity with the current spec and supporting C.  */
      font_entity = font_find_for_lface (f, (*face).lface.as_mut_ptr (),
					 FONT_DEF_SPEC! (font_def), c);
      if ! NILP (font_entity)
	{
	  /* We found a font.  Open it and insert a new element for
	     that font in VEC.  */
	  let mut j: c_int;

	  font_object = font_open_for_lface (f, font_entity, (*face).lface.as_mut_ptr (),
					     Qnil ());
	  if NILP (font_object)
	    { i += 1; continue; }
	  RFONT_DEF_NEW! (rfont_def, font_def);
	  RFONT_DEF_SET_OBJECT! (rfont_def, font_object);
	  RFONT_DEF_SET_SCORE! (rfont_def, RFONT_DEF_SCORE! (rfont_def));
	  let new_vec: Lisp_Object = make_nil_vector (ASIZE (vec) + 1);
	  found_index += 1;
	  j = 0;
	  while j < found_index
	    { ASET (new_vec, j as isize, AREF (vec, j as isize)); j += 1; }
	  ASET (new_vec, j as isize, rfont_def);
	  j += 1;
	  while j < ASIZE (new_vec) as c_int
	    { ASET (new_vec, j as isize, AREF (vec, (j - 1) as isize)); j += 1; }
	  XSETCDR (font_group, new_vec);
	  vec = new_vec;
	  { break 'found; }
	}
      if i >= 0
	{ i = found_index; }
      i += 1;
    }

  /* Record that no font in this font group supports C.  */
  FONTSET_SET! (fontset, make_fixnum (c as EMACS_INT), make_fixnum (0));
  return Qnil ();
  }

  /* found: */
  if fallback && found_index > 0
    {
      /* The order of fonts in the fallback font-group is not that
	 important, and it is better to move the found font to the
	 first of the group so that the next try will find it
	 quickly. */
      i = found_index;
      while i > 0
	{
	  ASET (vec, i as isize, AREF (vec, (i - 1) as isize));
	  i -= 1;
	}
      ASET (vec, 0, rfont_def);
    }
  return rfont_def;

  }
}


/* Return RFONT-DEF (vector) corresponding to the font for character
   C.  The value is not a vector if no font is found for C.  */

#[cfg (feature = "have_window_system")]
unsafe fn fontset_font (fontset: Lisp_Object, c: c_int, face: *mut face,   // C 759
			id: c_int) -> Lisp_Object
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut rfont_def: Lisp_Object;
  let mut default_rfont_def: Lisp_Object = Qnil ();  // C 763: `UNINIT` (PORTING.md 1.5)
  let mut base_fontset: Lisp_Object;

  /* Try a font-group of FONTSET. */
  FONT_DEFERRED_LOG! (c"current fontset: font for".as_ptr (), make_fixnum (c as EMACS_INT),
		      Qnil ());
  rfont_def = fontset_find_font (fontset, c, face, id, false);
  if VECTORP (rfont_def)
    { return rfont_def; }
  if NILP (rfont_def)
    { FONTSET_SET! (fontset, make_fixnum (c as EMACS_INT), make_fixnum (0)); }

  /* Try a font-group of the default fontset. */
  base_fontset = FONTSET_BASE! (fontset);
  if ! EQ (base_fontset, (*Vdefault_fontset.as_mut_ptr ()))
    {
      if NILP (FONTSET_DEFAULT! (fontset))
	{ set_fontset_default
	    (fontset,
	     make_fontset (FONTSET_FRAME! (fontset), Qnil (), (*Vdefault_fontset.as_mut_ptr ()))); }
      FONT_DEFERRED_LOG! (c"default fontset: font for".as_ptr (), make_fixnum (c as EMACS_INT),
			  Qnil ());
      default_rfont_def
	= fontset_find_font (FONTSET_DEFAULT! (fontset), c, face, id, false);
      if VECTORP (default_rfont_def)
	{ return default_rfont_def; }
      if NILP (default_rfont_def)
	{ FONTSET_SET! (FONTSET_DEFAULT! (fontset), make_fixnum (c as EMACS_INT),
		       make_fixnum (0)); }
    }

  /* Try a fallback font-group of FONTSET. */
  if ! EQ (rfont_def, Qt ())
    {
      FONT_DEFERRED_LOG! (c"current fallback: font for".as_ptr (), make_fixnum (c as EMACS_INT),
			  Qnil ());
      rfont_def = fontset_find_font (fontset, c, face, id, true);
      if VECTORP (rfont_def)
	{ return rfont_def; }
      /* Remember that FONTSET has no font for C.  */
      FONTSET_SET! (fontset, make_fixnum (c as EMACS_INT), Qt ());
    }

  /* Try a fallback font-group of the default fontset. */
  if ! EQ (base_fontset, (*Vdefault_fontset.as_mut_ptr ()))
      && ! EQ (default_rfont_def, Qt ())
    {
      FONT_DEFERRED_LOG! (c"default fallback: font for".as_ptr (), make_fixnum (c as EMACS_INT),
			  Qnil ());
      rfont_def = fontset_find_font (FONTSET_DEFAULT! (fontset), c, face, id, true);
      if VECTORP (rfont_def)
	{ return rfont_def; }
      /* Remember that the default fontset has no font for C.  */
      FONTSET_SET! (FONTSET_DEFAULT! (fontset), make_fixnum (c as EMACS_INT), Qt ());
    }

  return Qnil ();

  }
}

/* Return a newly created fontset with NAME.  If BASE is nil, make a
   base fontset.  Otherwise make a realized fontset whose base is
   BASE.  */

#[cfg (feature = "have_window_system")]
unsafe fn make_fontset (frame: Lisp_Object, name: Lisp_Object, base: Lisp_Object) -> Lisp_Object
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {                                                                          // C 822
  let mut fontset: Lisp_Object;
  let size: c_int = ASIZE ((*Vfontset_table.as_mut_ptr ())) as c_int;
  let mut id: c_int = (*next_fontset_id.as_mut_ptr ());

  /* Find a free slot in Vfontset_table.  Usually, next_fontset_id is
     the next available fontset ID.  So it is expected that this loop
     terminates quickly.  In addition, as the last element of
     Vfontset_table is always nil, we don't have to check the range of
     id.  */
  while ! NILP (AREF ((*Vfontset_table.as_mut_ptr ()), id as isize)) { id += 1; }

  if id + 1 == size
    { (*Vfontset_table.as_mut_ptr ()) = larger_vector ((*Vfontset_table.as_mut_ptr ()), 1, -1); }

  fontset = Fmake_char_table (Qfontset (), Qnil ());

  set_fontset_id (fontset, make_fixnum (id as EMACS_INT));
  if NILP (base)
    { set_fontset_name (fontset, name); }
  else
    {
      set_fontset_name (fontset, Qnil ());
      set_fontset_frame (fontset, frame);
      set_fontset_base (fontset, base);
    }

  ASET ((*Vfontset_table.as_mut_ptr ()), id as isize, fontset);
  (*next_fontset_id.as_mut_ptr ()) = id + 1;
  return fontset;

  }
}


/********** INTERFACES TO xfaces.c, xfns.c, and dispextern.h **********/

/* Return the name of the fontset who has ID.  */

#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe fn fontset_name (id: c_int) -> Lisp_Object                         // C 861
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut fontset: Lisp_Object;

  fontset = FONTSET_FROM_ID! (id as isize);
  return FONTSET_NAME! (fontset);

  }
}


/* Return the ASCII font name of the fontset who has ID.  */

#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe fn fontset_ascii (id: c_int) -> Lisp_Object                        // C 873
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut fontset: Lisp_Object;
  let mut elt: Lisp_Object;

  fontset= FONTSET_FROM_ID! (id as isize);
  elt = FONTSET_ASCII! (fontset);
  if CONSP (elt)
    { elt = XCAR (elt); }
  return elt;

  }
}

/* Free fontset of FACE defined on frame F.  Called from
   free_realized_face.  */

#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe fn free_face_fontset (f: *mut frame, face: *mut face)               // C 888
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut fontset: Lisp_Object;

  fontset = FONTSET_FROM_ID! ((*face).fontset as isize);
  if NILP (fontset)
    { return; }
  eassert! (! BASE_FONTSET_P! (fontset));
  eassert! (f == XFRAME (FONTSET_FRAME! (fontset)));
  ASET ((*Vfontset_table.as_mut_ptr ()), (*face).fontset as isize, Qnil ());
  if (*face).fontset < (*next_fontset_id.as_mut_ptr ())
    { (*next_fontset_id.as_mut_ptr ()) = (*face).fontset; }
  if ! NILP (FONTSET_DEFAULT! (fontset))
    {
      let id: c_int = XFIXNUM (FONTSET_ID! (FONTSET_DEFAULT! (fontset))) as c_int;

      fontset = AREF ((*Vfontset_table.as_mut_ptr ()), id as isize);
      eassert! (!NILP (fontset) && ! BASE_FONTSET_P! (fontset));
      eassert! (f == XFRAME (FONTSET_FRAME! (fontset)));
      ASET ((*Vfontset_table.as_mut_ptr ()), id as isize, Qnil ());
      if id < (*next_fontset_id.as_mut_ptr ())
	{ (*next_fontset_id.as_mut_ptr ()) = (*face).fontset; }
    }
  (*face).fontset = -1;

  }
}

/* Return ID of face suitable for displaying character C at buffer position
   POS on frame F.  FACE must be realized for ASCII characters in advance.
   Called from the macro FACE_FOR_CHAR.  */

#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe fn face_for_char (f: *mut frame, face: *mut face, c: c_int,       // C 919
			     pos: ptrdiff_t, object: Lisp_Object) -> c_int
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut fontset: Lisp_Object;
  let mut rfont_def: Lisp_Object;
  let mut charset: Lisp_Object;
  let mut face_id: c_int;
  let mut id: c_int;

  if ASCII_CHAR_P (c as isize) || CHAR_BYTE8_P (c)
    { return (*(*face).ascii_face).id; }

  if (*use_default_font_for_symbols.as_mut_ptr ())  /* let the user disable this feature */
      && c > 0 && EQ (CHAR_TABLE_REF ((*Vchar_script_table.as_mut_ptr ()), c as isize), Qsymbol ())
    {
      let mut font_object: Lisp_Object;

      if ! (*(*face).ascii_face).font.is_null ()
	{
	  XSETFONT! (font_object, (*(*face).ascii_face).font);
	  if font_has_char (f, font_object, c) != 0
	    { return (*(*face).ascii_face).id; }
	}

      /* C 955-968: `#if 0` - C's own dead text (the `face->font` arm of this test, disabled
	 over bug #15138).  No configuration compiles it, so it is recorded in
	 `docs/ports/fontset.md` and not transcribed (the `src/ccl.rs` 35-40 rule).  */
    }

  /* If the parent face has no fontset we could work with, and has no
     font, just return that same face, so that the caller will
     consider the character to have no font capable of displaying it,
     and display it as "glyphless".  That is certainly better than
     violating the assertion below or crashing when assertions are not
     compiled in.  */
  if (*face).fontset < 0 && (*face).font.is_null ()
    { return (*face).id; }

  eassert! (fontset_id_valid_p ((*face).fontset));
  fontset = FONTSET_FROM_ID! ((*face).fontset as isize);
  eassert! (!BASE_FONTSET_P! (fontset));

  if pos < 0
    {
      id = -1;
      charset = Qnil ();
    }
  else
    {
      charset = Fget_char_property (make_fixnum (pos as EMACS_INT), Qcharset (), object);
      if CHARSETP! (charset)
	{
	  let mut val: Lisp_Object;

	  val = assq_no_quit (charset, (*Vfont_encoding_charset_alist.as_mut_ptr ()));
	  if CONSP (val) && CHARSETP! (XCDR (val))
	    { charset = XCDR (val); }
	  id = XFIXNUM (CHARSET_SYMBOL_ID! (charset)) as c_int;
	}
      else
	{ id = -1; }
    }

  rfont_def = fontset_font (fontset, c, face, id);
  if VECTORP (rfont_def)
    {
      if FIXNUMP (RFONT_DEF_FACE! (rfont_def))
	{ face_id = XFIXNUM (RFONT_DEF_FACE! (rfont_def)) as c_int; }
      else
	{
	  let font_object: Lisp_Object;

	  font_object = RFONT_DEF_OBJECT! (rfont_def);
	  face_id = face_for_font (f, font_object, face);
	  RFONT_DEF_SET_FACE! (rfont_def, face_id as EMACS_INT);
	}
    }
  else
    {
      if FIXNUMP (FONTSET_NOFONT_FACE! (fontset))
	{ face_id = XFIXNUM (FONTSET_NOFONT_FACE! (fontset)) as c_int; }
      else
	{
	  face_id = face_for_font (f, Qnil (), face);
	  set_fontset_nofont_face (fontset, make_fixnum (face_id as EMACS_INT));
	}
    }
  eassert! (face_id >= 0);
  return face_id;

  }
}


#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe fn font_for_char (face: *mut face, c: c_int, pos: ptrdiff_t,   // C 1035
			     object: Lisp_Object) -> Lisp_Object
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut fontset: Lisp_Object;
  let mut rfont_def: Lisp_Object;
  let mut charset: Lisp_Object;
  let mut id: c_int;

  if ASCII_CHAR_P (c as isize)
    {
      let mut font_object: Lisp_Object;

      XSETFONT! (font_object, (*(*face).ascii_face).font);
      return font_object;
    }

  eassert! (fontset_id_valid_p ((*face).fontset));
  fontset = FONTSET_FROM_ID! ((*face).fontset as isize);
  eassert! (!BASE_FONTSET_P! (fontset));
  if pos < 0
    {
      id = -1;
      charset = Qnil ();
    }
  else
    {
      charset = Fget_char_property (make_fixnum (pos as EMACS_INT), Qcharset (), object);
      if CHARSETP! (charset)
	{
	  let mut val: Lisp_Object;

	  val = assq_no_quit (charset, (*Vfont_encoding_charset_alist.as_mut_ptr ()));
	  if CONSP (val) && CHARSETP! (XCDR (val))
	    { charset = XCDR (val); }
	  id = XFIXNUM (CHARSET_SYMBOL_ID! (charset)) as c_int;
	}
      else
	{ id = -1; }
    }

  rfont_def = fontset_font (fontset, c, face, id);
  return if VECTORP (rfont_def)
	 { RFONT_DEF_OBJECT! (rfont_def) }
	 else { Qnil () };

  }
}


/* Make a realized fontset for ASCII face FACE on frame F from the
   base fontset BASE_FONTSET_ID.  If BASE_FONTSET_ID is -1, use the
   default fontset as the base.  Value is the id of the new fontset.
   Called from realize_gui_face.  */

#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe fn make_fontset_for_ascii_face (f: *mut frame, base_fontset_id: c_int,  // C 1085
					    face: *mut face) -> c_int
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut base_fontset: Lisp_Object;
  let mut fontset: Lisp_Object;
  let mut frame: Lisp_Object = Qnil ();

  XSETFRAME! (frame, f);
  if base_fontset_id >= 0
    {
      base_fontset = FONTSET_FROM_ID! (base_fontset_id as isize);
      if !BASE_FONTSET_P! (base_fontset)
	{ base_fontset = FONTSET_BASE! (base_fontset); }
      eassert! (BASE_FONTSET_P! (base_fontset));
    }
  else
    { base_fontset = (*Vdefault_fontset.as_mut_ptr ()); }

  fontset = make_fontset (frame, Qnil (), base_fontset);
  return XFIXNUM (FONTSET_ID! (fontset)) as c_int;

  }
}


/* Cache data used by fontset_pattern_regexp.  The car part is a
   pattern string containing at least one wild card, the cdr part is
   the corresponding regular expression.  */
#[cfg (feature = "have_window_system")]
static Vcached_fontset_data: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());         // C 1109

#[cfg (feature = "have_window_system")]
macro_rules! CACHED_FONTSET_NAME {
  () => { SSDATA (XCAR ((*Vcached_fontset_data.as_mut_ptr ()))) };                            // C 1111
}
#[cfg (feature = "have_window_system")]
macro_rules! CACHED_FONTSET_REGEX {
  () => { XCDR ((*Vcached_fontset_data.as_mut_ptr ())) };                                     // C 1112
}

/* If fontset name PATTERN contains any wild card, return regular
   expression corresponding to PATTERN.  */

#[cfg (feature = "have_window_system")]
unsafe fn fontset_pattern_regexp (pattern: Lisp_Object) -> Lisp_Object       // C 1118
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if strchr (SSDATA (pattern), b'*' as c_int).is_null ()
      && strchr (SSDATA (pattern), b'?' as c_int).is_null ()
    /* PATTERN does not contain any wild cards.  */
    { return Qnil (); }

  if !CONSP ((*Vcached_fontset_data.as_mut_ptr ()))
      || strcmp (SSDATA (pattern), CACHED_FONTSET_NAME! ()) != 0
    {
      /* We must at first update the cached data.  */
      let mut regex: *mut u8 = std::ptr::null_mut ();
      let mut p0: *mut u8 = std::ptr::null_mut ();
      let mut p1: *mut u8 = std::ptr::null_mut ();
      let mut ndashes: c_int = 0;
      let mut nstars: c_int = 0;
      let mut nescs: c_int = 0;

      p0 = SDATA (pattern);
      while *p0 != 0
	{
	  if *p0 == b'-'
	    { ndashes += 1; }
	  else if *p0 == b'*'
	    { nstars += 1; }
	  else if *p0 == b'['
		   || *p0 == b'.' || *p0 == b'\\'
		   || *p0 == b'+' || *p0 == b'^'
		   || *p0 == b'$'
	    { nescs += 1; }
	  p0 = p0.add (1);
	}

      /* If PATTERN is not full XLFD we convert "*" to ".*".  Otherwise
	 we convert "*" to "[^-]*" which is much faster in regular
	 expression matching.  */
      let regexsize: ptrdiff_t = (SBYTES (pattern)
				  + (if ndashes < 14 { 2 } else { 5 }) as isize * nstars as isize
				  + 2 * nescs as isize + 3);
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
      regex = SAFE_ALLOCA! (sa_avail, regexsize) as *mut u8;
      p1 = regex;

      *p1 = b'^'; p1 = p1.add (1);
      p0 = SDATA (pattern);
      while *p0 != 0
	{
	  if *p0 == b'*'
	    {
	      if ndashes < 14
		{ *p1 = b'.'; p1 = p1.add (1); }
	      else
		{ *p1 = b'['; p1 = p1.add (1); *p1 = b'^'; p1 = p1.add (1);
		  *p1 = b'-'; p1 = p1.add (1); *p1 = b']'; p1 = p1.add (1); }
	      *p1 = b'*'; p1 = p1.add (1);
	    }
	  else if *p0 == b'?'
	    { *p1 = b'.'; p1 = p1.add (1); }
	  else if *p0 == b'['
		   || *p0 == b'.' || *p0 == b'\\'
		   || *p0 == b'+' || *p0 == b'^'
		   || *p0 == b'$'
	    { *p1 = b'\\'; p1 = p1.add (1); *p1 = *p0; p1 = p1.add (1); }
	  else
	    { *p1 = *p0; p1 = p1.add (1); }
	  p0 = p0.add (1);
	}
      *p1 = b'$'; p1 = p1.add (1);
      *p1 = 0; p1 = p1.add (1);

      (*Vcached_fontset_data.as_mut_ptr ()) = Fcons (build_string (SSDATA (pattern)),
				    build_string (regex as *const c_char));
      SAFE_FREE! (sa_count, sa_mark);
    }

  return CACHED_FONTSET_REGEX! ();

  }
}

/* Return ID of the base fontset named NAME.  If there's no such
   fontset, return -1.  NAME_PATTERN specifies how to treat NAME as this:
     0: pattern containing '*' and '?' as wildcards
     1: regular expression
     2: literal fontset name
*/

#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe fn fs_query_fontset (name: Lisp_Object, name_pattern: c_int) -> c_int  // C 1194
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut tem: Lisp_Object;
  let mut i: c_int;
  let mut name: Lisp_Object = name;
  let mut name_pattern: c_int = name_pattern;

  name = Fdowncase (name);
  if name_pattern != 1
    {
      tem = Frassoc (name, (*Vfontset_alias_alist.as_mut_ptr ()));
      if NILP (tem)
	{ tem = Fassoc (name, (*Vfontset_alias_alist.as_mut_ptr ()), Qnil ()); }
      if CONSP (tem) && STRINGP (XCAR (tem))
	{ name = XCAR (tem); }
      else if name_pattern == 0
	{
	  tem = fontset_pattern_regexp (name);
	  if STRINGP (tem)
	    {
	      name = tem;
	      name_pattern = 1;
	    }
	}
    }

  i = 0;
  while i < ASIZE ((*Vfontset_table.as_mut_ptr ())) as c_int
    {
      let mut fontset: Lisp_Object;
      let mut this_name: Lisp_Object;

      fontset = FONTSET_FROM_ID! (i as isize);
      if NILP (fontset)
	  || !BASE_FONTSET_P! (fontset)
	{ i += 1; continue; }

      this_name = FONTSET_NAME! (fontset);
      if if name_pattern == 1
	 { fast_string_match_ignore_case (name, this_name) >= 0 }
	 else { dispextern_externs::xstrcasecmp (SSDATA (name), SSDATA (this_name)) == 0 }
	{ return i; }
      i += 1;
    }
  return -1;

  }
}


#[cfg (feature = "have_window_system")]
DEFUN! ("query-fontset", Fquery_fontset, Squery_fontset, 1, 2, 0,
       doc: "Return the name of a fontset that matches PATTERN.
The value is nil if there is no matching fontset.
PATTERN can contain `*' or `?' as a wildcard
just as X font name matching algorithm allows.
If REGEXPP is non-nil, PATTERN is a regular expression.  ",       // C 1237-1242
       (pattern: Lisp_Object, regexpp: Lisp_Object),
       {
  let mut fontset: Lisp_Object;
  let mut id: c_int;

  check_window_system (std::ptr::null_mut ());

  CHECK_STRING (pattern);

  if SCHARS (pattern) == 0
    { return Qnil (); }

  id = fs_query_fontset (pattern, if ! NILP (regexpp) { 1 } else { 0 });
  if id < 0
    { return Qnil (); }

  fontset = FONTSET_FROM_ID! (id as isize);
  return FONTSET_NAME! (fontset);
});

/* Return a list of base fontset names matching PATTERN on frame F.  */

#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe fn list_fontsets (f: *mut frame, pattern: Lisp_Object,     // C 1266
			     size: c_int) -> Lisp_Object
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut frame: Lisp_Object = Qnil ();
  let mut regexp: Lisp_Object;
  let mut val: Lisp_Object;
  let mut id: c_int;

  XSETFRAME! (frame, f);

  regexp = fontset_pattern_regexp (pattern);
  val = Qnil ();

  id = 0;
  while id < ASIZE ((*Vfontset_table.as_mut_ptr ())) as c_int
    {
      let mut fontset: Lisp_Object;
      let mut name: Lisp_Object;

      fontset = FONTSET_FROM_ID! (id as isize);
      if NILP (fontset)
	  || !BASE_FONTSET_P! (fontset)
	  || !EQ (frame, FONTSET_FRAME! (fontset))
	{ id += 1; continue; }
      name = FONTSET_NAME! (fontset);

      if if STRINGP (regexp)
	 { fast_string_match (regexp, name) < 0 }
	 else { strcmp (SSDATA (pattern), SSDATA (name)) != 0 }
	{ id += 1; continue; }

      val = Fcons (Fcopy_sequence (FONTSET_NAME! (fontset)), val);
      id += 1;
    }

  return val;

  }
}


/* Free all realized fontsets whose base fontset is BASE.  */

#[cfg (feature = "have_window_system")]
unsafe fn free_realized_fontsets (base: Lisp_Object)                          // C 1302
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut id: c_int;

  /* C 1306-1333: `#if 0` - C's own dead text (the `free_realized_face` walk, with the
     comment that explains why it is disabled: `free_realized_face` does not remove FACE
     from a cache).  No configuration compiles it, so it is recorded in
     `docs/ports/fontset.md` and not transcribed (the `src/ccl.rs` 35-40 rule); C's
     `#else` arm is what follows.  */
  /* But, we don't have to call Fclear_face_cache if no fontset has
     been realized from BASE.  */
  id = 0;
  while id < ASIZE ((*Vfontset_table.as_mut_ptr ())) as c_int
    {
      let this: Lisp_Object = AREF ((*Vfontset_table.as_mut_ptr ()), id as isize);

      if CHAR_TABLE_P (this) && EQ (FONTSET_BASE! (this), base)
	{
	  Fclear_face_cache (Qt ());
	  if FRAME_LIVE_P (XFRAME (FONTSET_FRAME! (this)))
	    /* This is in case some Lisp calls this function and then
	       proceeds with calling some other function, like font-at,
	       which needs the basic faces.  */
	    { recompute_basic_faces (XFRAME (FONTSET_FRAME! (this))); }
	  break;
	}
      id += 1;
    }

  }
}


/* Check validity of NAME as a fontset name and return the
   corresponding fontset.  If not valid, signal an error.

   If NAME is t, return Vdefault_fontset.  If NAME is nil, return the
   fontset of *FRAME.

   Set *FRAME to the actual frame.  */

#[cfg (feature = "have_window_system")]
unsafe fn check_fontset_name (name: Lisp_Object, frame: *mut Lisp_Object) -> Lisp_Object
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {                                                                             // C 1364
  let mut id: c_int;
  let f: *mut frame = decode_live_frame (*frame);

  XSETFRAME! (*frame, f);

  if EQ (name, Qt ())
    { return (*Vdefault_fontset.as_mut_ptr ()); }
  if NILP (name)
    {
      if !FRAME_WINDOW_P (f)
	{ error (c"Can't use fontsets in non-GUI frames".as_ptr ()); }
      id = FRAME_FONTSET! (f);
    }
  else
    {
      CHECK_STRING (name);
      /* First try NAME as literal.  */
      id = fs_query_fontset (name, 2);
      if id < 0
	/* For backward compatibility, try again NAME as pattern.  */
	{ id = fs_query_fontset (name, 0); }
      if id < 0
	{ error (c"Fontset `%s' does not exist".as_ptr (), SDATA (name)); }
    }
  return FONTSET_FROM_ID! (id as isize);

  }
}

#[cfg (feature = "have_window_system")]
unsafe extern "C-unwind" fn accumulate_script_ranges (arg: Lisp_Object, range: Lisp_Object,  // C 1394
					   val: Lisp_Object)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if EQ (XCAR (arg), val)
    {
      if CONSP (range)
	{ XSETCDR (arg, Fcons (Fcons (XCAR (range), XCDR (range)), XCDR (arg))); }
      else
	{ XSETCDR (arg, Fcons (Fcons (range, range), XCDR (arg))); }
    }

  }
}


/* Callback function for map_charset_chars in Fset_fontset_font.
   ARG is a vector [ FONTSET FONT_DEF ADD ASCII SCRIPT_RANGE_LIST ].

   In FONTSET, set FONT_DEF in a fashion specified by ADD for
   characters in RANGE and ranges in SCRIPT_RANGE_LIST before RANGE.
   The consumed ranges are popped up from SCRIPT_RANGE_LIST, and the
   new SCRIPT_RANGE_LIST is stored in ARG.

   If ASCII is nil, don't set FONT_DEF for ASCII characters.  It is
   assured that SCRIPT_RANGE_LIST doesn't contain ASCII in that
   case.  */

#[cfg (feature = "have_window_system")]
unsafe extern "C-unwind" fn set_fontset_font (arg: Lisp_Object, range: Lisp_Object)  // C 1419
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut fontset: Lisp_Object;
  let mut font_def: Lisp_Object;
  let mut add: Lisp_Object;
  let mut ascii: Lisp_Object;
  let mut script_range_list: Lisp_Object;
  let mut from: c_int = XFIXNUM (XCAR (range)) as c_int;
  let mut to: c_int = XFIXNUM (XCDR (range)) as c_int;
  let mut range: Lisp_Object = range;

  fontset = AREF (arg, 0);
  font_def = AREF (arg, 1);
  add = AREF (arg, 2);
  ascii = AREF (arg, 3);
  script_range_list = AREF (arg, 4);

  if NILP (ascii) && from < 0x80
    {
      if to < 0x80
	{ return; }
      from = 0x80;
      range = Fcons (make_fixnum (0x80), XCDR (range));
    }

  macro_rules! SCRIPT_FROM {
    () => { XFIXNUM (XCAR (XCAR (script_range_list))) as c_int };
  }
  macro_rules! SCRIPT_TO {
    () => { XFIXNUM (XCDR (XCAR (script_range_list))) as c_int };
  }
  macro_rules! POP_SCRIPT_RANGE {
    () => { script_range_list = XCDR (script_range_list); };
  }

  while CONSP (script_range_list) && SCRIPT_TO! () < from
    {
      FONTSET_ADD! (fontset, XCAR (script_range_list), font_def, add);
      POP_SCRIPT_RANGE! ();
    }
  if CONSP (script_range_list)
    {
      if SCRIPT_FROM! () < from
	{ range = Fcons (make_fixnum (SCRIPT_FROM! () as EMACS_INT), XCDR (range)); }
      while CONSP (script_range_list) && SCRIPT_TO! () <= to
	{ POP_SCRIPT_RANGE! (); }
      if CONSP (script_range_list) && SCRIPT_FROM! () <= to
	{ XSETCAR (XCAR (script_range_list), make_fixnum ((to + 1) as EMACS_INT)); }
    }

  FONTSET_ADD! (fontset, range, font_def, add);
  ASET (arg, 4, script_range_list);

  }
}

/* C 1458: `static void update_auto_fontset_alist (Lisp_Object, Lisp_Object);` - a forward
   declaration with no Rust item (the definition is C 1858).  */


#[cfg (feature = "have_window_system")]
DEFUN! ("set-fontset-font", Fset_fontset_font, Sset_fontset_font, 3, 5, 0,
       doc: "
Modify FONTSET to use font specification in FONT-SPEC for displaying CHARACTERS.

FONTSET should be a fontset name (a string); or nil, meaning the
fontset of FRAME; or t, meaning the default fontset.

CHARACTERS may be a single character to use FONT-SPEC for.

CHARACTERS may be a cons (FROM . TO), where FROM and TO are characters.
In that case, use FONT-SPEC for all the characters in the range
between FROM and TO (inclusive).

CHARACTERS may be a script symbol.  In that case, use FONT-SPEC for
all the characters that belong to the script.  See the variable
`script-representative-chars' for the list of known scripts, and
see the variable `char-script-table' for the script of any specific
character.  Note: for the `symbol' script only, whether the FONTSET
is actually used depends on the value of `use-default-font-for-symbols'.

CHARACTERS may be a charset symbol.  In that case, use FONT-SPEC for
all the characters in the charset.  See `list-character-sets' and
`list-charset-chars' for the list of character sets and their
characters.

CHARACTERS may be nil.  In that case, use FONT-SPEC for any
character for which no font-spec is specified in FONTSET.

FONT-SPEC may one of these:
 * A font-spec object made by the function `font-spec' (which see).
 * A cons (FAMILY . REGISTRY), where FAMILY is a font family name and
   REGISTRY is a font registry name.  FAMILY may contain foundry
   name, and REGISTRY may contain encoding name.
 * A font name string.
 * nil, which explicitly specifies that there's no font for CHARACTERS.

Optional 4th argument FRAME is a frame whose fontset should be modified;
it is used if FONTSET is nil.  If FONTSET is nil and FRAME is omitted
or nil, that stands for the fontset of the selected frame.

Optional 5th argument ADD, if non-nil, specifies how to add FONT-SPEC
to the previously set font specifications for CHARACTERS.  If it is
`prepend', FONT-SPEC is prepended to the existing font specifications.
If it is `append', FONT-SPEC is appended.  By default, FONT-SPEC
overwrites the previous settings.

For reliable results, this function should be called before any
of CHARACTERS were displayed in the current Emacs session.  In
particular, if some of CHARACTERS are displayed using character
compositions, those compositions will be cached after they are first
produced, and the cached values include the font used for displaying
the composed characters -- calling this function will not affect the
font recorded in the cache of compositions, thus they will continue
to be shown using the fonts from before the call.  ",        // C 1462-1514
       (fontset: Lisp_Object, characters: Lisp_Object, font_spec: Lisp_Object,
	frame: Lisp_Object, add: Lisp_Object),
       {
  let mut fontset_obj: Lisp_Object;
  let mut font_def: Lisp_Object;
  let mut registry: Lisp_Object;
  let mut family: Lisp_Object;
  let mut range_list: Lisp_Object;
  let mut charset: *mut charset = std::ptr::null_mut ();
  let mut fontname: Lisp_Object;
  let mut ascii_changed: bool = false;
  let mut font_spec: Lisp_Object = font_spec;
  let mut fontset: Lisp_Object = fontset;
  let mut frame: Lisp_Object = frame;

  fontset_obj = check_fontset_name (fontset, &raw mut frame);

  fontname = Qnil ();
  if CONSP (font_spec)
    {
      let spec: Lisp_Object = Ffont_spec (0, std::ptr::null_mut ());

      font_parse_family_registry (XCAR (font_spec), XCDR (font_spec), spec);
      font_spec = spec;
      fontname = Ffont_xlfd_name (font_spec, Qnil (), Qt ());
    }
  else if STRINGP (font_spec)
    {
      fontname = font_spec;
      font_spec = CALLN! (Ffont_spec, QCname (), fontname);
    }
  else if FONT_SPEC_P (font_spec)
    { fontname = Ffont_xlfd_name (font_spec, Qnil (), Qt ()); }
  else if ! NILP (font_spec)
    { Fsignal (Qfont (), list2 (build_string (c"Invalid font-spec".as_ptr ()), font_spec)); }

  if ! NILP (font_spec)
    {
      let mut encoding: Lisp_Object;
      let mut repertory: Lisp_Object;

      family = AREF (font_spec, FONT_FAMILY_INDEX as isize);
      if ! NILP (family)
	{ family = SYMBOL_NAME (family); }
      registry = AREF (font_spec, FONT_REGISTRY_INDEX as isize);
      if ! NILP (registry)
	{ registry = Fdowncase (SYMBOL_NAME (registry)); }
      AUTO_STRING! (dash, c"-".as_ptr ());
      encoding = find_font_encoding (concat3 (family, dash, registry));
      if NILP (encoding)
	{ encoding = Qascii (); }

      if SYMBOLP (encoding)
	{
	  CHECK_CHARSET! (encoding);
	  encoding = CHARSET_SYMBOL_ID! (encoding);
	  repertory = encoding;
	}
      else
	{
	  repertory = XCDR (encoding);
	  encoding = XCAR (encoding);
	  CHECK_CHARSET! (encoding);
	  encoding = CHARSET_SYMBOL_ID! (encoding);
	  if ! NILP (repertory) && SYMBOLP (repertory)
	    {
	      CHECK_CHARSET! (repertory);
	      repertory = CHARSET_SYMBOL_ID! (repertory);
	    }
	}
      font_def = font_def_new (font_spec, encoding, repertory);
    }
  else
    { font_def = Qnil (); }

  if CHARACTERP (characters)
    {
      if XFIXNAT (characters) < 0x80
	{ error (c"Can't set a font for partial ASCII range".as_ptr ()); }
      range_list = list1 (Fcons (characters, characters));
    }
  else if CONSP (characters)
    {
      let mut from: Lisp_Object;
      let mut to: Lisp_Object;

      from = Fcar (characters);
      to = Fcdr (characters);
      CHECK_CHARACTER (from);
      CHECK_CHARACTER (to);
      if XFIXNAT (from) < 0x80
	{
	  if XFIXNAT (from) != 0 || XFIXNAT (to) < 0x7F
	    { error (c"Can't set a font for partial ASCII range".as_ptr ()); }
	  ascii_changed = true;
	}
      range_list = list1 (characters);
    }
  else if SYMBOLP (characters) && !NILP (characters)
    {
      let mut script_list: Lisp_Object;
      let mut val: Lisp_Object;

      range_list = Qnil ();
      script_list = FAM_ELEM! ((*XCHAR_TABLE ((*Vchar_script_table.as_mut_ptr ()))).extras, 0);
      if ! NILP (Fmemq (characters, script_list))
	{
	  if EQ (characters, Qlatin ())
	    { ascii_changed = true; }
	  val = list1 (characters);
	  map_char_table (Some (accumulate_script_ranges), Qnil (), (*Vchar_script_table.as_mut_ptr ()),
			  val);
	  range_list = Fnreverse (XCDR (val));
	}
      if CHARSETP! (characters)
	{
	  CHECK_CHARSET_GET_CHARSET! (characters, charset);
	  if charset_bits! (ascii_compatible_p, charset)
	    { ascii_changed = true; }
	}
      else if NILP (range_list)
	{ error (c"Invalid script or charset name: %s".as_ptr (),
		 SDATA (SYMBOL_NAME (characters))); }
    }
  else if NILP (characters)
    { range_list = list1 (Qnil ()); }
  else
    { error (c"Invalid second argument for setting a font in a fontset".as_ptr ()); }

  if ascii_changed
    {
      let val: Lisp_Object;

      if NILP (font_spec)
	{ error (c"Can't set ASCII font to nil".as_ptr ()); }
      val = CHAR_TABLE_REF (fontset_obj, 0);
      if ! NILP (val) && EQ (add, Qappend ())
	/* We are going to change just an additional font for ASCII.  */
	{ ascii_changed = false; }
    }

  if ! charset.is_null ()
    {
      let arg: Lisp_Object = CALLN! (Fvector, fontset_obj, font_def, add,
				     if ascii_changed { Qt () } else { Qnil () }, range_list);

      map_charset_chars (Some (set_fontset_font), Qnil (), arg, charset,
			 CHARSET_MIN_CODE! (charset),
			 CHARSET_MAX_CODE! (charset));
      range_list = AREF (arg, 4);
    }
  while CONSP (range_list)
    {
      FONTSET_ADD! (fontset_obj, XCAR (range_list), font_def, add);
      range_list = XCDR (range_list);
    }

  if ascii_changed
    {
      let mut tail: Lisp_Object;
      let mut fr: Lisp_Object = Qnil ();
      let fontset_id: c_int = XFIXNUM (FONTSET_ID! (fontset_obj)) as c_int;

      set_fontset_ascii (fontset_obj, fontname);
      fontset = FONTSET_NAME! (fontset_obj);
      FOR_EACH_FRAME! (tail, fr, {
	  let f: *mut frame = XFRAME (fr);
	  let mut font_object: Lisp_Object;
	  let mut face: *mut face;

	  if FRAME_INITIAL_P (f) || FRAME_TERMCAP_P (f)
	    { continue; }
	  if fontset_id != FRAME_FONTSET! (f)
	    { continue; }
	  face = frame_face_accessors::FACE_FROM_ID_OR_NULL (f, DEFAULT_FACE_ID);
	  if ! face.is_null ()
	    { font_object = font_load_for_lface (f, (*face).lface.as_mut_ptr (), font_spec); }
	  else
	    { font_object = font_open_by_spec (f, font_spec); }
	  if ! NILP (font_object)
	    {
	      update_auto_fontset_alist (font_object, fontset_obj);
	      AUTO_FRAME_ARG! (arg, Qfont (), Fcons (fontset, font_object));

	      /* C 1688-1696: `#ifdef HAVE_WINDOW_SYSTEM` around the first two arms; C's
		 `#endif` sits before `Fmodify_frame_parameters`, which is outside the
		 `#ifdef`.  The file gate is that same macro, so this arm is always live in
		 this unit.  */
	      #[cfg (feature = "have_window_system")]
	      if FRAME_WINDOW_P (f)
		/* This is a window-system frame.  Prevent changes of the
		   `font' parameter here from messing with the
		   `font-parameter' frame property, as the frame
		   parameter is not being changed by the user.  */
		{ gui_set_frame_parameters_1 (f, arg, true); }
	      else
		{ Fmodify_frame_parameters (fr, arg); }
	    }
      });
    }

  /* Free all realized fontsets whose base is FONTSET_OBJ.  This way, the
     specified character(s) are surely redisplayed by a correct
     font.  */
  free_realized_fontsets (fontset_obj);

  return Qnil ();
});


#[cfg (feature = "have_window_system")]
DEFUN! ("new-fontset", Fnew_fontset, Snew_fontset, 2, 2, 0,
       doc: "Create a new fontset NAME from font information in FONTLIST.

FONTLIST is an alist of scripts vs the corresponding font specification list.
Each element of FONTLIST has the form (SCRIPT FONT-SPEC ...), where a
character of SCRIPT is displayed by a font that matches one of
FONT-SPEC.

SCRIPT is a symbol that appears in the first extra slot of the
char-table `char-script-table'.

FONT-SPEC is a vector, a cons, or a string.  See the documentation of
`set-fontset-font' for the meaning.  ",                       // C 1712-1724
       (name: Lisp_Object, fontlist: Lisp_Object),
       {
  let mut fontset: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut id: c_int;
  let mut string: *mut c_char;
  let mut name: Lisp_Object = name;

  CHECK_STRING (name);

  name = Fdowncase (name);
  id = fs_query_fontset (name, 0);
  if id < 0
    {
      let font_spec: Lisp_Object = Ffont_spec (0, std::ptr::null_mut ());
      let short_name: Lisp_Object;

      if font_parse_xlfd (SSDATA (name), SBYTES (name), font_spec) < 0
	{ error (c"Fontset name must be in XLFD format".as_ptr ()); }
      short_name = AREF (font_spec, FONT_REGISTRY_INDEX as isize);
      if strncmp (SSDATA (SYMBOL_NAME (short_name)), c"fontset-".as_ptr (), 8) != 0
	  || SBYTES (SYMBOL_NAME (short_name)) < 9
	{ error (c"Registry field of fontset name must be \"fontset-*\"".as_ptr ()); }
      (*Vfontset_alias_alist.as_mut_ptr ()) = Fcons (Fcons (name, SYMBOL_NAME (short_name)),
				    (*Vfontset_alias_alist.as_mut_ptr ()));
      ASET (font_spec, FONT_REGISTRY_INDEX as isize, Qiso8859_1 ());
      fontset = make_fontset (Qnil (), name, Qnil ());
      string = font_dynamic_unparse_xlfd (font_spec, 0);
      if string.is_null ()
	{ error (c"Invalid fontset name (perhaps too long): %s".as_ptr (), SDATA (name)); }
      set_fontset_ascii (fontset, build_unibyte_string (string));
      xfree (string as *mut c_void);
    }
  else
    {
      fontset = FONTSET_FROM_ID! (id as isize);
      free_realized_fontsets (fontset);
      Fset_char_table_range (fontset, Qt (), Qnil ());
    }

  tail = fontlist;
  while CONSP (tail)
    {
      let mut elt: Lisp_Object;
      let mut script: Lisp_Object;

      elt = XCAR (tail);
      script = Fcar (elt);
      elt = Fcdr (elt);
      if CONSP (elt) && (NILP (XCDR (elt)) || CONSP (XCDR (elt)))
	{
	  while CONSP (elt)
	    {
	      Fset_fontset_font (name, script, XCAR (elt), Qnil (), Qappend ());
	      elt = XCDR (elt);
	    }
	}
      else
	{ Fset_fontset_font (name, script, elt, Qnil (), Qappend ()); }
      tail = XCDR (tail);
    }
  CHECK_LIST_END (tail, fontlist);
  return name;
});


/* Alist of automatically created fontsets.  Each element is a cons
   (FONT-SPEC . FONTSET-ID).  */
#[cfg (feature = "have_window_system")]
static auto_fontset_alist: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());           // C 1782

/* Number of automatically created fontsets.  */
#[cfg (feature = "have_window_system")]
static num_auto_fontsets: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);                                  // C 1785

/* Return a fontset synthesized from FONT-OBJECT.  This is called from
   the terminal hook set_new_font_hook when FONT-OBJECT is used for
   the default ASCII font of a frame, and the returned fontset is used
   for the default fontset of that frame.  The fontset specifies a
   font of the same registry as FONT-OBJECT for all characters in the
   repertory of the registry (see Vfont_encoding_alist).  If the
   repertory is not known, the fontset specifies the font for all
   Latin characters assuming that a user intends to use FONT-OBJECT
   for Latin characters.  */

#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe fn fontset_from_font (font_object: Lisp_Object) -> c_int          // C 1798
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let font_name: Lisp_Object = font_get_name (font_object);
  let mut font_spec: Lisp_Object = copy_font_spec (font_object);
  let registry: Lisp_Object = AREF (font_spec, FONT_REGISTRY_INDEX as isize);
  let mut fontset_spec: Lisp_Object;
  let mut alias: Lisp_Object;
  let mut name: Lisp_Object;
  let mut fontset: Lisp_Object;
  let mut val: Lisp_Object;
  let mut xlfd: Lisp_Object;

  val = assoc_no_quit (font_spec, (*auto_fontset_alist.as_mut_ptr ()));
  if CONSP (val)
    { return XFIXNUM (FONTSET_ID! (XCDR (val))) as c_int; }
  let previous_count: ptrdiff_t = (*num_auto_fontsets.as_mut_ptr ());
  (*num_auto_fontsets.as_mut_ptr ()) += 1;
  if previous_count == 0
    { alias = Qfontset_startup (); }
  else
    {
      /* C's `char temp[sizeof "fontset-auto" + INT_STRLEN_BOUND (ptrdiff_t)]`: `sizeof
	 "fontset-auto"` is 13 (12 characters and the terminator) and the bound is 20 for
	 `ptrdiff_t` on this target (the ledger's probe).  */
      let mut temp: [c_char; std::mem::size_of::<[c_char; 13]> () + INT_STRLEN_BOUND! (ptrdiff_t)] =
	[0; std::mem::size_of::<[c_char; 13]> () + INT_STRLEN_BOUND! (ptrdiff_t)];

      sprintf (temp.as_mut_ptr (), c"fontset-auto%td".as_ptr (), (*num_auto_fontsets.as_mut_ptr ()) - 1);
      alias = intern (temp.as_mut_ptr ());
    }
  fontset_spec = copy_font_spec (font_spec);
  ASET (fontset_spec, FONT_REGISTRY_INDEX as isize, alias);
  name = Ffont_xlfd_name (fontset_spec, Qnil (), Qt ());
  eassert! (!NILP (name));
  fontset = make_fontset (Qnil (), name, Qnil ());
  (*Vfontset_alias_alist.as_mut_ptr ()) = Fcons (Fcons (name, SYMBOL_NAME (alias)),
				(*Vfontset_alias_alist.as_mut_ptr ()));

  xlfd = AREF (font_object, FONT_NAME_INDEX as isize);
  alias = Fdowncase (xlfd);
  (*Vfontset_alias_alist.as_mut_ptr ())
    = Fcons (Fcons (name, alias), (*Vfontset_alias_alist.as_mut_ptr ()));
  (*auto_fontset_alist.as_mut_ptr ())
    = Fcons (Fcons (font_spec, fontset), (*auto_fontset_alist.as_mut_ptr ()));

  font_spec = Ffont_spec (0, std::ptr::null_mut ());
  ASET (font_spec, FONT_REGISTRY_INDEX as isize, registry);
  {
    let mut target: Lisp_Object = find_font_encoding (SYMBOL_NAME (registry));

    if CONSP (target)
      { target = XCDR (target); }
    if ! CHARSETP! (target)
      { target = Qlatin (); }
    Fset_fontset_font (name, target, font_spec, Qnil (), Qnil ());
    Fset_fontset_font (name, Qnil (), font_spec, Qnil (), Qnil ());
  }

  set_fontset_ascii (fontset, font_name);

  return XFIXNUM (FONTSET_ID! (fontset)) as c_int;

  }
}


/* Update auto_fontset_alist for FONTSET.  When an ASCII font of
   FONTSET is changed, we delete an entry of FONTSET if any from
   auto_fontset_alist so that FONTSET is not reused by
   fontset_from_font.  */

#[cfg (feature = "have_window_system")]
unsafe fn update_auto_fontset_alist (font_object: Lisp_Object, fontset: Lisp_Object)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {                                                                             // C 1858
  let mut prev: Lisp_Object;
  let mut tail: Lisp_Object;

  prev = Qnil (); tail = (*auto_fontset_alist.as_mut_ptr ());
  while CONSP (tail)
    {
      if EQ (fontset, XCDR (XCAR (tail)))
	{
	  if NILP (prev)
	    { (*auto_fontset_alist.as_mut_ptr ()) = XCDR (tail); }
	  else
	    { XSETCDR (prev, XCDR (tail)); }
	  break;
	}
      prev = tail;
      tail = XCDR (tail);
    }

  }
}


#[cfg (feature = "have_window_system")]
DEFUN! ("fontset-info", Ffontset_info, Sfontset_info, 1, 2, 0,
       doc: "Return information about a fontset FONTSET on frame FRAME.

FONTSET is a fontset name string, nil for the fontset of FRAME, or t
for the default fontset.  FRAME nil means the selected frame.

The value is a char-table whose elements have this form:

    ((FONT OPENED-FONT ...) ...)

FONT is a name of font specified for a range of characters.

OPENED-FONT is a name of a font actually opened.

The char-table has one extra slot.  If FONTSET is not the default
fontset, the value the extra slot is a char-table containing the
information about the derived fonts from the default fontset.  The
format is the same as above.  ",                              // C 1876-1892
       (fontset: Lisp_Object, frame: Lisp_Object),
       {
  let mut fontset: Lisp_Object = fontset;
  let mut realized: [*mut Lisp_Object; 2] = [std::ptr::null_mut (); 2];
  let mut realized0: *mut Lisp_Object = std::ptr::null_mut ();
  let mut fontsets: [Lisp_Object; 2] = [Qnil (); 2];
  let mut tables: [Lisp_Object; 2] = [Qnil (); 2];
  let mut val: Lisp_Object;
  let mut elt: Lisp_Object;
  let mut c: c_int;
  let mut i: c_int;
  let mut j: c_int;
  let mut k: c_int;
  let mut frame: Lisp_Object = frame;

  check_window_system (std::ptr::null_mut ());
  fontset = check_fontset_name (fontset, &raw mut frame);

  /* Recode fontsets realized on FRAME from the base fontset FONTSET
     in the table `realized'.  */
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  SAFE_ALLOCA_LISP! (sa_avail, realized0, 2 * ASIZE ((*Vfontset_table.as_mut_ptr ())));
  realized[0] = realized0;
  realized[1] = realized[0].add (ASIZE ((*Vfontset_table.as_mut_ptr ())) as usize);
  i = 0; j = 0;
  while i < ASIZE ((*Vfontset_table.as_mut_ptr ())) as c_int
    {
      elt = FONTSET_FROM_ID! (i as isize);
      if !NILP (elt)
	  && EQ (FONTSET_BASE! (elt), fontset)
	  && EQ (FONTSET_FRAME! (elt), frame)
	{
	  *realized[0].add (j as usize) = elt;
	  j += 1;
	}
      i += 1;
    }
  *realized[0].add (j as usize) = Qnil ();

  i = 0; j = 0;
  while ! NILP (*realized[0].add (i as usize))
    {
      elt = FONTSET_DEFAULT! (*realized[0].add (i as usize));
      if ! NILP (elt)
	{
	  *realized[1].add (j as usize) = elt;
	  j += 1;
	}
      i += 1;
    }
  *realized[1].add (j as usize) = Qnil ();

  tables[0] = Fmake_char_table (Qfontset_info (), Qnil ());
  fontsets[0] = fontset;
  if !EQ (fontset, (*Vdefault_fontset.as_mut_ptr ()))
    {
      tables[1] = Fmake_char_table (Qnil (), Qnil ());
      set_char_table_extras (tables[0], 0, tables[1]);
      fontsets[1] = (*Vdefault_fontset.as_mut_ptr ());
    }

  /* Accumulate information of the fontset in TABLE.  The format of
     each element is ((FONT-SPEC OPENED-FONT ...) ...).  */
  k = 0;
  while k <= 1
    {
      c = 0;
      while c <= MAX_CHAR
	{
	  let mut from: c_int = c;
	  let mut to: c_int = MAX_5_BYTE_CHAR;

	  if c <= MAX_5_BYTE_CHAR
	    {
	      val = char_table_ref_and_range (fontsets[k as usize], c, &raw mut from,
					      &raw mut to);
	    }
	  else
	    {
	      val = FONTSET_FALLBACK! (fontsets[k as usize]);
	      to = MAX_CHAR;
	    }
	  if VECTORP (val)
	    {
	      let mut alist: Lisp_Object;

	      /* At first, set ALIST to ((FONT-SPEC) ...).  */
	      alist = Qnil (); i = 0;
	      while i < ASIZE (val) as c_int
		{
		  if ! NILP (AREF (val, i as isize))
		    { alist = Fcons (Fcons (FONT_DEF_SPEC! (AREF (val, i as isize)), Qnil ()),
				     alist); }
		  i += 1;
		}
	      alist = Fnreverse (alist);

	      /* Then store opened font names to cdr of each elements.  */
	      i = 0;
	      while ! NILP (*realized[k as usize].add (i as usize))
		{
		  if c <= MAX_5_BYTE_CHAR
		    { val = FONTSET_REF! (*realized[k as usize].add (i as usize), c); }
		  else
		    { val = FONTSET_FALLBACK! (*realized[k as usize].add (i as usize)); }
		  if ! CONSP (val) || ! VECTORP (XCDR (val))
		    { i += 1; continue; }
		  /* VAL: (int . [[FACE-ID FONT-DEF FONT-OBJECT int] ... ])  */
		  val = XCDR (val);
		  j = 0;
		  while j < ASIZE (val) as c_int
		    {
		      elt = AREF (val, j as isize);
		      if !NILP (elt) && FONT_OBJECT_P (RFONT_DEF_OBJECT! (elt))
			{
			  let font_object: Lisp_Object = RFONT_DEF_OBJECT! (elt);
			  let mut slot: Lisp_Object;
			  let name: Lisp_Object;

			  slot = Fassq (RFONT_DEF_SPEC! (elt), alist);
			  name = AREF (font_object, FONT_NAME_INDEX as isize);
			  if NILP (Fmember (name, XCDR (slot)))
			    { nconc2 (slot, list1 (name)); }
			}
		      j += 1;
		    }
		  i += 1;
		}

	      /* Store ALIST in TBL for characters C..TO.  */
	      if c <= MAX_5_BYTE_CHAR
		{ char_table_set_range (tables[k as usize], c, to, alist); }
	      else
		{ set_char_table_defalt (tables[k as usize], alist); }

	      /* At last, change each elements to font names.  */
	      while CONSP (alist)
		{
		  elt = XCAR (alist);
		  XSETCAR (elt, Ffont_xlfd_name (XCAR (elt), Qnil (), Qt ()));
		  alist = XCDR (alist);
		}
	    }
	  c = to + 1;
	}
      if EQ (fontset, (*Vdefault_fontset.as_mut_ptr ()))
	{ break; }
      k += 1;
    }

  SAFE_FREE! (sa_count, sa_mark);
  return tables[0];
});


#[cfg (feature = "have_window_system")]
DEFUN! ("fontset-font", Ffontset_font, Sfontset_font, 2, 3, 0,
       doc: "Return a font name pattern for character CH in fontset NAME.
If NAME is t, find a pattern in the default fontset.
If NAME is nil, find a pattern in the fontset of the selected frame.

The value has the form (FAMILY . REGISTRY), where FAMILY is a font
family name and REGISTRY is a font registry name.  This is actually
the first font name pattern for CH in the fontset or in the default
fontset.

If the 2nd optional arg ALL is non-nil, return a list of all font name
patterns.  ",                                                 // C 2013-2025
       (name: Lisp_Object, ch: Lisp_Object, all: Lisp_Object),
       {
  let mut c: c_int;
  let mut fontset: Lisp_Object;
  let mut elt: Lisp_Object;
  let mut list: Lisp_Object;
  let mut repertory: Lisp_Object;
  let mut val: Lisp_Object;
  let mut i: c_int;
  let mut j: c_int;
  let mut frame: Lisp_Object;

  frame = Qnil ();
  fontset = check_fontset_name (name, &raw mut frame);

  CHECK_CHARACTER (ch);
  c = XFIXNUM (ch) as c_int;
  list = Qnil ();
  loop {
    i = 0;
    elt = FONTSET_REF! (fontset, c);
    while i < 2
      {
	if VECTORP (elt)
	  {
	    j = 0;
	    while j < ASIZE (elt) as c_int
	      {
		let mut family: Lisp_Object;
		let mut registry: Lisp_Object;

		val = AREF (elt, j as isize);
		if NILP (val)
		  { return Qnil (); }
		repertory = AREF (val, 1);
		if FIXNUMP (repertory)
		  {
		    let charset: *mut charset = CHARSET_FROM_ID! (XFIXNUM (repertory) as isize);

		    if ! CHAR_CHARSET_P! (c, charset)
		      { j += 1; continue; }
		  }
		else if CHAR_TABLE_P (repertory)
		  {
		    if NILP (CHAR_TABLE_REF (repertory, c as isize))
		      { j += 1; continue; }
		  }
		val = AREF (val, 0);
		/* VAL is a FONT-SPEC */
		family = AREF (val, FONT_FAMILY_INDEX as isize);
		if ! NILP (family)
		  { family = SYMBOL_NAME (family); }
		registry = AREF (val, FONT_REGISTRY_INDEX as isize);
		if ! NILP (registry)
		  { registry = SYMBOL_NAME (registry); }
		val = Fcons (family, registry);
		if NILP (all)
		  { return val; }
		list = Fcons (val, list);
		j += 1;
	      }
	  }
	i += 1;
	elt = FONTSET_FALLBACK! (fontset);
      }
    if EQ (fontset, (*Vdefault_fontset.as_mut_ptr ()))
      { break; }
    fontset = (*Vdefault_fontset.as_mut_ptr ());
  }
  return Fnreverse (list);
});

#[cfg (feature = "have_window_system")]
DEFUN! ("fontset-list", Ffontset_list, Sfontset_list, 0, 0, 0,
       doc: "Return a list of all defined fontset names.  ",     // C 2083-2084
       (),
       {
  let mut fontset: Lisp_Object;
  let mut list: Lisp_Object;
  let mut i: c_int;

  list = Qnil ();
  i = 0;
  while i < ASIZE ((*Vfontset_table.as_mut_ptr ())) as c_int
    {
      fontset = FONTSET_FROM_ID! (i as isize);
      if !NILP (fontset)
	  && BASE_FONTSET_P! (fontset)
	{ list = Fcons (FONTSET_NAME! (fontset), list); }
      i += 1;
    }

  return list;
});


#[cfg (all (feature = "have_window_system", feature = "enable_checking"))]
#[unsafe(no_mangle)]
pub unsafe fn dump_fontset (fontset: Lisp_Object) -> Lisp_Object            // C 2108
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let vec: Lisp_Object = make_nil_vector (3);
  ASET (vec, 0, FONTSET_ID! (fontset));

  if BASE_FONTSET_P! (fontset)
    {
      ASET (vec, 1, FONTSET_NAME! (fontset));
    }
  else
    {
      let mut frame: Lisp_Object;

      frame = FONTSET_FRAME! (fontset);
      if FRAMEP (frame)
	{
	  let f: *mut frame = XFRAME (frame);

	  if FRAME_LIVE_P (f)
	    { ASET (vec, 1,
		    Fcons (FONTSET_NAME! (FONTSET_BASE! (fontset)),
			   (*f).name)); }
	  else
	    { ASET (vec, 1,
		    Fcons (FONTSET_NAME! (FONTSET_BASE! (fontset)), Qnil ())); }
	}
      if !NILP (FONTSET_DEFAULT! (fontset))
	{ ASET (vec, 2, FONTSET_ID! (FONTSET_DEFAULT! (fontset))); }
    }
  return vec;

  }
}

#[cfg (all (feature = "have_window_system", feature = "enable_checking"))]
DEFUN! ("fontset-list-all", Ffontset_list_all, Sfontset_list_all, 0, 0, 0,
       doc: "Return a brief summary of all fontsets for debug use.  ",  // C 2140-2141
       (),
       {
  let mut val: Lisp_Object;
  let mut i: c_int;

  val = Qnil (); i = 0;
  while i < ASIZE ((*Vfontset_table.as_mut_ptr ())) as c_int
    {
      if ! NILP (AREF ((*Vfontset_table.as_mut_ptr ()), i as isize))
	{ val = Fcons (dump_fontset (AREF ((*Vfontset_table.as_mut_ptr ()), i as isize)), val); }
      i += 1;
    }
  return Fnreverse (val);
});


#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe fn syms_of_fontset ()                                            // C 2155
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  DEFSYM! (Qfontset, c"fontset");
  Fput (Qfontset (), Qchar_table_extra_slots (), make_fixnum (8));
  DEFSYM! (Qfontset_info, c"fontset-info");
  Fput (Qfontset_info (), Qchar_table_extra_slots (), make_fixnum (1));
  DEFSYM! (Qfontset_startup, c"fontset-startup");

  DEFSYM! (Qappend, c"append");
  DEFSYM! (Qlatin, c"latin");

  (*Vcached_fontset_data.as_mut_ptr ()) = Qnil ();
  staticpro (Vcached_fontset_data.as_ptr ());

  (*Vfontset_table.as_mut_ptr ()) = make_nil_vector (32);
  staticpro (Vfontset_table.as_ptr ());

  (*Vdefault_fontset.as_mut_ptr ()) = Fmake_char_table (Qfontset (), Qnil ());
  staticpro (Vdefault_fontset.as_ptr ());
  set_fontset_id ((*Vdefault_fontset.as_mut_ptr ()), make_fixnum (0));
  set_fontset_name
    ((*Vdefault_fontset.as_mut_ptr ()),
     build_string (c"-*-*-*-*-*-*-*-*-*-*-*-*-fontset-default".as_ptr ()));
  ASET ((*Vfontset_table.as_mut_ptr ()), 0, (*Vdefault_fontset.as_mut_ptr ()));
  (*next_fontset_id.as_mut_ptr ()) = 1;
  PDUMPER_REMEMBER_SCALAR! ((*next_fontset_id.as_mut_ptr ()));

  (*auto_fontset_alist.as_mut_ptr ()) = Qnil ();
  staticpro (auto_fontset_alist.as_ptr ());

  DEFVAR_LISP! (c"font-encoding-charset-alist", Vfont_encoding_charset_alist,
	       " Alist of charsets vs the charsets to determine the preferred font encoding.
Each element looks like (CHARSET . ENCODING-CHARSET),
where ENCODING-CHARSET is a charset registered in the variable
`font-encoding-alist' as ENCODING.

When a text has a property `charset' and the value is CHARSET, a font
whose encoding corresponds to ENCODING-CHARSET is preferred.  ");
  (*Vfont_encoding_charset_alist.as_mut_ptr ()) = Qnil ();

  DEFVAR_LISP! (c"use-default-ascent", Vuse_default_ascent,
	       " Char table of characters whose ascent values should be ignored.
If an entry for a character is non-nil, the ascent value of the glyph
is assumed to be specified by _MULE_DEFAULT_ASCENT property of a font.

This affects how a composite character which contains
such a character is displayed on screen.  ");
  (*Vuse_default_ascent.as_mut_ptr ()) = Qnil ();

  DEFVAR_BOOL! (c"use-default-font-for-symbols", use_default_font_for_symbols,
	       " If non-nil, use the default face's font for symbols and punctuation.

By default, Emacs will try to use the default face's font for
displaying symbol and punctuation characters, disregarding the
fontsets, if the default font can display the character.
Set this to nil to make Emacs honor the fontsets instead.  ");
  (*use_default_font_for_symbols.as_mut_ptr ()) = true;

  DEFVAR_LISP! (c"ignore-relative-composition", Vignore_relative_composition,
	       " Char table of characters which are not composed relatively.
If an entry for a character is non-nil, a composition sequence
which contains that character is displayed so that
the glyph of that character is put without considering
an ascent and descent value of a previous character.  ");
  (*Vignore_relative_composition.as_mut_ptr ()) = Qnil ();

  DEFVAR_LISP! (c"alternate-fontname-alist", Valternate_fontname_alist,
	       " Alist of fontname vs list of the alternate fontnames.
When a specified font name is not found, the corresponding
alternate fontnames (if any) are tried instead.  ");
  (*Valternate_fontname_alist.as_mut_ptr ()) = Qnil ();

  DEFVAR_LISP! (c"fontset-alias-alist", Vfontset_alias_alist,
	       " Alist of fontset names vs the aliases.  ");
  (*Vfontset_alias_alist.as_mut_ptr ())
    = list1 (Fcons (FONTSET_NAME! ((*Vdefault_fontset.as_mut_ptr ())),
		    build_string (c"fontset-default".as_ptr ())));

  DEFVAR_LISP! (c"vertical-centering-font-regexp",
	       Vvertical_centering_font_regexp,
	       " Regexp matching font names that require vertical centering on display.
When a character is displayed with such fonts, the character is displayed
at the vertical center of lines.  ");
  (*Vvertical_centering_font_regexp.as_mut_ptr ()) = Qnil ();

  DEFVAR_LISP! (c"otf-script-alist", Votf_script_alist,
	       " Alist of OpenType script tags vs the corresponding script names.  ");
  (*Votf_script_alist.as_mut_ptr ()) = Qnil ();

  defsubr (Squery_fontset.as_mut_ptr ());
  defsubr (Snew_fontset.as_mut_ptr ());
  defsubr (Sset_fontset_font.as_mut_ptr ());
  defsubr (Sfontset_info.as_mut_ptr ());
  defsubr (Sfontset_font.as_mut_ptr ());
  defsubr (Sfontset_list.as_mut_ptr ());
  #[cfg (feature = "enable_checking")]
  {
    defsubr (Sfontset_list_all.as_mut_ptr ());
  }

  }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<face> () == 296);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (face, font) == 232);
