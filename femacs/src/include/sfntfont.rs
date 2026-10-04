// femacs: port of src/sfntfont.h @ fd7afced403.
//
// NOTE (integrator 3.170, re-measured by this lane): `glyph_string` is `*mut c_void` at the two
// sites below, and the measurement says that is the *standing* spelling for this target rather than
// a temporary one.  The record **is** mirrored - `include/dispextern.rs` 1362-1507 - but it is
// declared there `#[cfg (feature = "window_system")]`, which is right: dispextern.h 1358 declares it
// inside `#ifdef HAVE_WINDOW_SYSTEM` and the frozen configuration has no window system, so the name
// exists in no crate this target builds.  Naming it un-gated is what reddens `xtask layout-check`'s
// single crate (the two `E0425`s the integrator's note recorded); the alternative - gating
// `sfntfont_draw` and the `sfntfont_put_glyph_proc` typedef with that same feature - would deviate
// from C's unconditional text.  So P-55/P-72 rule 3 is what the two `/// C:` lines above those
// declarations record, and docs/ports/include-sfntfont.md names the one condition under which the
// spelling changes.
//
// Header mirror (PORTING.md 0.1).  The SFNT font driver's interface: C 20-21 is the include
// guard, C 23-26 the four includes (`lisp.h`, `frame.h`, `font.h`, `sfnt.h`), C 28 the one entry
// point `sfnt.c` defines, C 31-44 the font-driver callbacks `sfntfont.c` defines, C 49-51 the
// `sfntfont_put_glyph_proc` typedef and C 53-57 the four initialization entry points, C 62-66 the
// `HAVE_MMAP` arm and C 72-76 the `HAVE_HARFBUZZ` arm.  It is the sibling of `include/sfnt.rs`
// (`src/sfnt.h`, the container format) and the two are the same batch's pair.
//
// **Conditions.**  `tools/check-cond-eval.py src/sfntfont.h` -> `3 conditions, 0 unknown to the
// evaluator, 0 disagreements`, and `src/sfntfont.h` is on
// docs/baseline/config/cond-eval-verified.txt, so P-08's frozen answer is what the mirror carries:
// `HAVE_MMAP` is 1 (`sfntfont_detect_sigbus`, C 64, is live) and `HAVE_HARFBUZZ` is `undef`
// (C 74's `sfntfont_begin_hb_font` is dead text, carried under `#[cfg (feature = "harfbuzz")]`,
// PORTING.md P-51).  Nothing on this target compiles `src/sfntfont.c` or `src/sfnt.c` - the frozen
// build's platform object variables are empty and `docs/baseline/frozen-build-sources.txt` lists
// neither - so no unit `include!`s this mirror yet and the declarations below are deferred to the
// Windows/X/SFNT targets with their definitions (the `include/sfnt.rs` reading).
//
// **Names.**  `Lisp_Object` stays named (lisp.h is every unit's include, PORTING.md 0.1-9);
// `struct frame`, `struct font` and `struct glyph_string` are reached through the includes C 24-26
// names (`frame.h`, `font.h`, and `dispextern.h`, which `font.h` pulls in for `glyph_string`), so
// those three mirrors are `include!`ed below.  Three records this header names cannot be named
// here, and each is recorded where it belongs: `struct font_metrics` and `struct font_driver` are
// `font.h`'s, and `include/font.rs` only *mentions* `font_driver` (its 129 says the record has not
// landed); `struct glyph_string` **is** `dispextern.rs`'s, but that mirror carries it under
// `#[cfg (feature = "window_system")]` (dispextern.h 1358's `#ifdef HAVE_WINDOW_SYSTEM`), so on
// this configuration the name does not exist - measured when `xtask layout-check` compiled this
// mirror (`error[E0425]: cannot find type glyph_string in this scope`, twice: C 41's `sfntfont_draw`
// and C 49-51's typedef).  All five pointers therefore keep C's pointer depth with an untyped
// interior and the C text in a `/// C:` line (PORTING.md P-55/P-72 rule 3; the `include/haikuterm.rs`
// 489-494 reading of `glyph_string`, and the dead arm's `hb_font_t`, which is `<hb.h>`'s record).

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const _SFNTFONT_H_: bool = true;

include!("frame.rs");        // C 24: `#include "frame.h"` (`struct frame`)
include!("font.rs");         // C 25: `#include "font.h"` (`struct font`)
include!("sfnt.rs");         // C 26: `#include "sfnt.h"` (`struct sfnt_raster`)

/// C 28: `extern int sfnt_enum_font (const char *);` - the one entry point `src/sfnt.c` defines.
unsafe extern "C" {
  pub fn sfnt_enum_font (file: *const c_char) -> c_int;

  /// C 33-35: the three `Lisp_Object`-valued callbacks.  `struct frame` comes from `frame.rs`.
  /// C: extern Lisp_Object sfntfont_list (struct frame *, Lisp_Object);
  /// C: extern Lisp_Object sfntfont_match (struct frame *, Lisp_Object);
  /// C: extern Lisp_Object sfntfont_open (struct frame *, Lisp_Object, int);
  pub fn sfntfont_list (f: *mut frame, spec: Lisp_Object) -> Lisp_Object;
  pub fn sfntfont_match (f: *mut frame, spec: Lisp_Object) -> Lisp_Object;
  pub fn sfntfont_open (f: *mut frame, font_entity: Lisp_Object, pixel_size: c_int)
                        -> Lisp_Object;

  /// C 37-44: the driver hooks.  C 37's `int` and C 44's `unsigned[256]` are `c_int`/`*mut c_uint`
  /// (an array parameter decays to a pointer, PORTING.md P-55); `struct font_metrics *` and
  /// `struct glyph_string *` are the stand-ins of the file note.
  /// C: extern unsigned int sfntfont_encode_char (struct font *, int);
  /// C: extern void sfntfont_text_extents (struct font *, const unsigned int *,
  /// C: 				   int, struct font_metrics *);
  /// C: extern void sfntfont_close (struct font *);
  /// C: extern int sfntfont_draw (struct glyph_string *, int, int,
  /// C: 			  int, int, bool);
  /// C: extern Lisp_Object sfntfont_list_family (struct frame *);
  /// C: extern int sfntfont_get_variation_glyphs (struct font *, int, unsigned[256]);
  pub fn sfntfont_encode_char (font: *mut font, char_: c_int) -> c_uint;
  pub fn sfntfont_text_extents (font: *mut font, code: *const c_uint, nchars: c_int,
                                metrics: *mut c_void);
  pub fn sfntfont_close (font: *mut font);
  pub fn sfntfont_draw (s: *mut c_void, from: c_int, to: c_int, x: c_int, y: c_int,
                        with_background: bool) -> c_int;
  pub fn sfntfont_list_family (f: *mut frame) -> Lisp_Object;
  pub fn sfntfont_get_variation_glyphs (font: *mut font, character: c_int,
                                        variation_selectors: *mut c_uint) -> c_int;

  /// C 53-57: the initialization entry points.  C 56-57's `const struct font_driver *` has no
  /// owner mirror (the file note) and `sfntfont_put_glyph_proc` is C 49-51's typedef below.
  /// C: extern void syms_of_sfntfont (void);
  /// C: extern void init_sfntfont (void);
  /// C: extern void mark_sfntfont (void);
  /// C: extern void init_sfntfont_vendor (Lisp_Object, const struct font_driver *,
  /// C: 				  sfntfont_put_glyph_proc);
  pub fn syms_of_sfntfont ();
  pub fn init_sfntfont ();
  pub fn mark_sfntfont ();
  pub fn init_sfntfont_vendor (name: Lisp_Object, driver: *const c_void,
                               put_glyph: sfntfont_put_glyph_proc);

  /// C 62-66: the `HAVE_MMAP` arm, live here (the file note).
  /// C: extern bool sfntfont_detect_sigbus (void *);
  pub fn sfntfont_detect_sigbus (addr: *mut c_void) -> bool;

  /// C 72-76: the `HAVE_HARFBUZZ` arm, dead text on this target (`HAVE_HARFBUZZ` is `undef`),
  /// carried under the feature that names the other side (PORTING.md P-51).  `hb_font_t` is
  /// `<hb.h>`'s record and the pointer keeps C's depth with an untyped interior.
  /// C: extern hb_font_t *sfntfont_begin_hb_font (struct font *, double *);
  #[cfg (feature = "harfbuzz")]
  pub fn sfntfont_begin_hb_font (font: *mut font, advance: *mut c_double) -> *mut c_void;
}

/// C 49-51: `typedef void (*sfntfont_put_glyph_proc) (struct glyph_string *, int, int, int, int,
/// bool, struct sfnt_raster **, int *)` - a C function pointer is nullable, so it is an `Option`
/// (PORTING.md P-27).  `struct sfnt_raster` is `sfnt.h`'s record and is reached through this
/// header's own include of it; the first parameter is the file note's stand-in.
/// C: typedef void (*sfntfont_put_glyph_proc) (struct glyph_string *, int, int,
/// C: 					 int, int, bool, struct sfnt_raster **,
/// C: 					 int *);
pub type sfntfont_put_glyph_proc
  = Option<unsafe extern "C" fn (s: *mut c_void, from: c_int, to: c_int, x: c_int, y: c_int,
                                 with_background: bool, raster: *mut *mut sfnt_raster,
                                 y_offset: *mut c_int)>;
