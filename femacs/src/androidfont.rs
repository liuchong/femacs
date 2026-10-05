// femacs: port of src/androidfont.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` except the eleven whose address C takes - the ten
// `androidfont_driver` methods and the pdumper hook - which are `unsafe extern "C" fn` items (a
// function-pointer member cannot be a Rust-ABI item; the androidmenu.rs / androidselect.rs shape).
//
// This is P-86's third unit: the frozen configuration compiles no `androidfont.o`.
// `configure.ac` 2597 puts `androidfont.o` in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])`
// (2594), and `docs/baseline/frozen-build-sources.txt` lists no `androidfont.o` for this target, so
// `extract-abi.py`'s checklist for the file is empty and its checkbox sections cannot be coverage
// evidence.  What follows is P-86 ②'s substitute: the ledger `docs/ports/androidfont.md` carries
// the C line-range table, the `## Evidence` readings and the `## Deviations`.  Two gates, in C's
// order:
//
//   * the file gate (`HAVE_ANDROID`; `androidfont.o` is in `ANDROID_OBJ` in *both* Android
//     branches, unlike `androidselect.o`, which 2647-2648 adds only in the `REALLY_ANDROID` one)
//     is `feature = "have_android"`, on every item *and on C 26-40's include graph* - which is why
//     the frozen configuration compiles nothing here;
//   * C 42's `#ifndef ANDROID_STUBIFY` ... C 1102's `#endif` (this file has no `#else` arm) is
//     `feature = "not_android_stubify"`, the feature the tree registered for C's
//     `HAVE_ANDROID && !defined ANDROID_STUBIFY` (the androidmenu.rs / androidselect.rs gate).
//
// A third C condition has no `#if` in this file and therefore no gate either: `HAVE_WINDOW_SYSTEM`
// (configure.ac 7518; the `window_system=android` branch at 2594-2596 is what puts this file in the
// build list at all, so an Android configuration always has it).  Three places in this file's text
// need it - C 646's `struct glyph_string *s` (dispextern.h 1361 sits under `#ifdef
// HAVE_WINDOW_SYSTEM`), C 1045's `.draw = androidfont_draw` (font.h 682) and the eight
// window-system members of `struct font` that C 758-772 writes (font.h 325-403) - so the arm-flip
// reading recorded in the ledger sets `have_window_system` as well and says so: without it the unit
// reports exactly the names C would not have without HAVE_WINDOW_SYSTEM, which is not a port defect.
//
// The ledger also carries the deviations: the NDK values for `<android/log.h>` (measured, not
// assumed), the six names the frozen mirrors do not carry yet, the `*mut c_void` the mirror's
// `font_driver` uses where C writes `struct frame *`, the two whole-struct `font_metrics`
// assignments, and the line-count basis.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 24: `#include <config.h>` (ungated)

/* C 26-40: the file's own include list.  It sits outside C 42's `#ifndef`, so the whole graph is
   on the file gate alone - no translation unit the frozen configuration builds reaches any of it.  */
#[cfg (feature = "have_android")]
include!("include/lisp.rs");                          // C 26: `#include "lisp.h"`
#[cfg (feature = "have_android")]
include!("include/dispextern.rs");                    // C 27: `#include "dispextern.h"`
#[cfg (feature = "have_android")]
include!("include/composite.rs");                     // C 28: `#include "composite.h"`
#[cfg (feature = "have_android")]
include!("include/blockinput.rs");                    // C 29: `#include "blockinput.h"`
#[cfg (feature = "have_android")]
include!("include/charset.rs");                       // C 30: `#include "charset.h"`
#[cfg (feature = "have_android")]
include!("include/frame.rs");                         // C 31: `#include "frame.h"`
#[cfg (feature = "have_android")]
include!("include/window.rs");                        // C 32: `#include "window.h"`
#[cfg (feature = "have_android")]
include!("include/fontset.rs");                       // C 33: `#include "fontset.h"`
#[cfg (feature = "have_android")]
include!("include/androidterm.rs");                   // C 34: `#include "androidterm.h"`
#[cfg (feature = "have_android")]
include!("include/character.rs");                     // C 35: `#include "character.h"`
#[cfg (feature = "have_android")]
include!("include/coding.rs");                        // C 36: `#include "coding.h"`
#[cfg (feature = "have_android")]
include!("include/font.rs");                          // C 37: `#include "font.h"`
#[cfg (feature = "have_android")]
include!("include/termchar.rs");                      // C 38: `#include "termchar.h"`
#[cfg (feature = "have_android")]
include!("include/pdumper.rs");                       // C 39: `#include "pdumper.h"`
#[cfg (feature = "have_android")]
include!("include/android.rs");                       // C 40: `#include "android.h"`
#[cfg (feature = "have_android")]
include!("../lib/include/limits.in.rs");              // lisp.h 33's `#include <limits.h>` (SHRT_MAX)

/* C 44: `#include <android/log.h>`.  The NDK's own header is not part of the reference tree, so it
   has no mirror and the port declares what it uses (P-88 ①/P-49, the androidmenu.rs `jni.h` class).
   Both values are measured on this machine's NDK 29.0.14206865 with its own clang for
   `--target=aarch64-linux-android24` (the target `spikes/jni-probe.sh` uses; Evidence in the
   ledger): `log.h` 74-89's `enum android_LogPriority` gives `ANDROID_LOG_WARN == 5` (UNKNOWN 0,
   DEFAULT 1, VERBOSE 2, DEBUG 3, INFO 4, WARN 5), and `log.h` 112 declares
   `int __android_log_print(int prio, const char *tag, const char *fmt, ...)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_LOG_WARN: c_int = 5;

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub fn __android_log_print (prio: c_int, tag: *const c_char, fmt: *const c_char, ...) -> c_int;
}

/* Seven declarations the fifteen headers above reach, which the frozen mirrors do not carry yet
   (P-72 rule 4: a mirror must reach what it needs through its own `include!`s).  Until each lands
   in its mirror the unit supplies it at the use site - the `alloc.rs` `mark_haiku_display` /
   `androidselect.rs` `FUNCTIONP` shape; every line names the C header it comes from, and
   `docs/ports/androidfont.md` records the gap for the integrator.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: lisp.h 4432: `extern AVOID memory_full (size_t);` (alloc.c defines it).
  pub fn memory_full (size: size_t) -> !;
  /// C: font.h 843-845: `#ifdef HAVE_ANDROID extern Lisp_Object font_make_entity_android (int);`
  pub fn font_make_entity_android (size: c_int) -> Lisp_Object;
  /// C: font.h 846: `extern Lisp_Object font_make_object (int, Lisp_Object, int);`
  pub fn font_make_object (size: c_int, entity: Lisp_Object, pixelsize: c_int) -> Lisp_Object;
  /// C: dispextern.h 3809-3810: `#ifdef HAVE_WINDOW_SYSTEM void prepare_face_for_display
  /// (struct frame *, struct face *);` - `struct face` is the mirror's untyped pointer (P-72 ③).
  pub fn prepare_face_for_display (f: *mut frame, face: *mut c_void);
  /// C: font.h 896: `extern void register_font_driver (struct font_driver const *,
  /// struct frame *);`
  pub fn register_font_driver (driver: *const font_driver, f: *mut frame);
  /// C: font.h 854: `extern int font_style_to_value (enum font_property_index prop,
  /// Lisp_Object name, bool noerror);`.  The mirror's `FONT_SET_STYLE!` (font.h 236) expands to
  /// this name (C 537 calls the macro), which no frozen mirror declares.
  pub fn font_style_to_value (prop: font_property_index, name: Lisp_Object, noerror: bool)
    -> c_int;
}

/* C's `ANDROID_DELETE_LOCAL_REF` (android.h 340-342) expands to a `<jni.h>` vtable read.  The
   mirror records it as an excluded macro whose premise ("`JNINativeInterface` has no mirror")
   predates `include/jni.rs`, which now carries that table (230 entries, `spikes/jni-probe.sh`); the
   port defines the expansion under C's own name so the token survives at every use site (the
   androidmenu.rs / androidselect.rs shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! ANDROID_DELETE_LOCAL_REF {
  ($r:expr) => { ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, $r) };
}

/* ------------------------------------------------------------------------------------------
   C 42-1102: the whole body, under C 42's `#ifndef ANDROID_STUBIFY`.  C writes no `#else` arm
   (C 1102's `#endif` closes the file), so the stub arm of this file is *nothing at all* - a
   configuration with `ANDROID_STUBIFY` compiles the include graph and no item.  */

/* C 42: `#ifndef ANDROID_STUBIFY` - every item below is `all (feature = "have_android",
   feature = "not_android_stubify")`.  */

/* C 44's `<android/log.h>` declarations are above, next to the include list.  */

/* Structure for the EmacsFontDriver class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_font_driver                       // C 46-60
{
  pub class: jclass,
  pub list: jmethodID,
  pub r#match: jmethodID,
  pub list_families: jmethodID,
  pub open_font: jmethodID,
  pub has_char: jmethodID,
  pub text_extents: jmethodID,
  pub encode_char: jmethodID,
  pub draw: jmethodID,

  /* Static methods.  */
  pub create_font_driver: jmethodID,
}

/* Structure for the EmacsFontDriver$FontSpec class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_font_spec                         // C 62-76
{
  pub class: jclass,
  pub foundry: jfieldID,
  pub family: jfieldID,
  pub adstyle: jfieldID,
  pub registry: jfieldID,
  pub width: jfieldID,
  pub weight: jfieldID,
  pub slant: jfieldID,
  pub size: jfieldID,
  pub spacing: jfieldID,
  pub avgwidth: jfieldID,
  pub dpi: jfieldID,
}

/* Structure for the EmacsFontDriver$FontMetrics class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_font_metrics                      // C 78-86
{
  pub class: jclass,
  pub lbearing: jfieldID,
  pub rbearing: jfieldID,
  pub width: jfieldID,
  pub ascent: jfieldID,
  pub descent: jfieldID,
}

/* Structure for the EmacsFontDriver$FontObject class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_font_object                       // C 88-106
{
  pub class: jclass,
  pub min_width: jfieldID,
  pub max_width: jfieldID,
  pub pixel_size: jfieldID,
  pub height: jfieldID,
  pub space_width: jfieldID,
  pub average_width: jfieldID,
  pub ascent: jfieldID,
  pub descent: jfieldID,
  pub underline_thickness: jfieldID,
  pub underline_position: jfieldID,
  pub baseline_offset: jfieldID,
  pub relative_compose: jfieldID,
  pub default_ascent: jfieldID,
  pub encoding_charset: jfieldID,
  pub repertory_charset: jfieldID,
}

/* Structure for the Integer class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_integer                                // C 108-113
{
  pub class: jclass,
  pub constructor: jmethodID,
  pub int_value: jmethodID,
}

/* Structure describing the font object.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct androidfont_info                                // C 115-125
{
  /* The font pseudo-vector object.  */
  pub font: font,

  /* The Java-side font.  */
  pub object: jobject,

  /* Cached glyph metrics arranged in a two dimensional array.  */
  pub metrics: *mut *mut font_metrics,
}

/* Structure describing a font entity.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct androidfont_entity                              // C 127-134
{
  /* The font entity pvec.  */
  pub font: font_entity,

  /* The Java-side font entity.  */
  pub object: jobject,
}

/* Method and class identifiers associated with the EmacsFontDriver class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut font_driver_class: android_emacs_font_driver    // C 139
  = android_emacs_font_driver { class: std::ptr::null_mut (),
                                list: std::ptr::null_mut (),
                                r#match: std::ptr::null_mut (),
                                list_families: std::ptr::null_mut (),
                                open_font: std::ptr::null_mut (),
                                has_char: std::ptr::null_mut (),
                                text_extents: std::ptr::null_mut (),
                                encode_char: std::ptr::null_mut (),
                                draw: std::ptr::null_mut (),
                                create_font_driver: std::ptr::null_mut () };

/* Field and class identifiers associated with the EmacsFontDriver$FontSpec class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut font_spec_class: android_emacs_font_spec        // C 144
  = android_emacs_font_spec { class: std::ptr::null_mut (),
                              foundry: std::ptr::null_mut (),
                              family: std::ptr::null_mut (),
                              adstyle: std::ptr::null_mut (),
                              registry: std::ptr::null_mut (),
                              width: std::ptr::null_mut (),
                              weight: std::ptr::null_mut (),
                              slant: std::ptr::null_mut (),
                              size: std::ptr::null_mut (),
                              spacing: std::ptr::null_mut (),
                              avgwidth: std::ptr::null_mut (),
                              dpi: std::ptr::null_mut () };

/* Method and class identifiers associated with the Integer class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut integer_class: android_integer                  // C 148
  = android_integer { class: std::ptr::null_mut (),
                      constructor: std::ptr::null_mut (),
                      int_value: std::ptr::null_mut () };

/* Field and class identifiers associated with the EmacsFontDriver$FontMetrics class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut font_metrics_class: android_emacs_font_metrics  // C 153
  = android_emacs_font_metrics { class: std::ptr::null_mut (),
                                 lbearing: std::ptr::null_mut (),
                                 rbearing: std::ptr::null_mut (),
                                 width: std::ptr::null_mut (),
                                 ascent: std::ptr::null_mut (),
                                 descent: std::ptr::null_mut () };

/* Field and class identifiers associated with the EmacsFontDriver$FontObject class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut font_object_class: android_emacs_font_object    // C 158
  = android_emacs_font_object { class: std::ptr::null_mut (),
                                min_width: std::ptr::null_mut (),
                                max_width: std::ptr::null_mut (),
                                pixel_size: std::ptr::null_mut (),
                                height: std::ptr::null_mut (),
                                space_width: std::ptr::null_mut (),
                                average_width: std::ptr::null_mut (),
                                ascent: std::ptr::null_mut (),
                                descent: std::ptr::null_mut (),
                                underline_thickness: std::ptr::null_mut (),
                                underline_position: std::ptr::null_mut (),
                                baseline_offset: std::ptr::null_mut (),
                                relative_compose: std::ptr::null_mut (),
                                default_ascent: std::ptr::null_mut (),
                                encoding_charset: std::ptr::null_mut (),
                                repertory_charset: std::ptr::null_mut () };

/* The font cache.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut font_cache: Lisp_Object = lisp_h_Qnil ();       // C 162 (tentative, i.e. Qnil)

/* The Java-side font driver.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut font_driver: jobject = std::ptr::null_mut ();   // C 166 (tentative, i.e. NULL)

/* Initialize the class and method identifiers for functions in the
   EmacsFontDriver class, and place them in `font_driver_class'.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_font_driver ()                     // C 173-223
{
  let mut old: jclass;

  font_driver_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
                                                    c"org/femacs/EmacsFontDriver".as_ptr ());
  eassert! (! font_driver_class.class.is_null ());

  old = font_driver_class.class;
  font_driver_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if font_driver_class.class.is_null ()
    { emacs_abort (); }

  /* C 192-197: `FIND_METHOD`, the macro this function defines and `#undef`s at C 222; Rust's
     `macro_rules!` scope ends with the block, so the `#undef` has no counterpart.  C 199-200,
     204-205, 206-207, 208-209, 210-211, 212-213 and 219-220 split their signature literals over
     two lines (translation phase 6 joins them); the port writes the joined literal, which is the
     same value, and every use names its C line.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      font_driver_class.$c_name
        = ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
                                                          font_driver_class.class,
                                                          $name, $signature);
      eassert! (! font_driver_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (list, c"list".as_ptr (),                   // C 199-200
                c"(Lorg/femacs/EmacsFontDriver$FontSpec;)[Lorg/femacs/EmacsFontDriver$FontEntity;".as_ptr ());
  FIND_METHOD! (r#match, c"match".as_ptr (),               // C 201-202
                c"(Lorg/femacs/EmacsFontDriver$FontSpec;)Lorg/femacs/EmacsFontDriver$FontEntity;".as_ptr ());
  FIND_METHOD! (list_families, c"listFamilies".as_ptr (),  // C 203
                c"()[Ljava/lang/String;".as_ptr ());
  FIND_METHOD! (open_font, c"openFont".as_ptr (),           // C 204-205
                c"(Lorg/femacs/EmacsFontDriver$FontEntity;I)Lorg/femacs/EmacsFontDriver$FontObject;".as_ptr ());
  FIND_METHOD! (has_char, c"hasChar".as_ptr (),             // C 206-207
                c"(Lorg/femacs/EmacsFontDriver$FontSpec;I)I".as_ptr ());
  FIND_METHOD! (text_extents, c"textExtents".as_ptr (),     // C 208-209
                c"(Lorg/femacs/EmacsFontDriver$FontObject;[ILorg/femacs/EmacsFontDriver$FontMetrics;)V".as_ptr ());
  FIND_METHOD! (encode_char, c"encodeChar".as_ptr (),       // C 210-211
                c"(Lorg/femacs/EmacsFontDriver$FontObject;I)I".as_ptr ());
  FIND_METHOD! (draw, c"draw".as_ptr (),                    // C 212-213
                c"(Lorg/femacs/EmacsFontDriver$FontObject;Lorg/femacs/EmacsGC;Lorg/femacs/EmacsDrawable;[IIIIZ)I".as_ptr ());

  /* C 215-221: the one place that is not a `FIND_METHOD`.  */
  font_driver_class.create_font_driver
    = ((*(*android_java_env)).GetStaticMethodID.unwrap ()) (android_java_env,
                                                            font_driver_class.class,
                                                            c"createFontDriver".as_ptr (),
                                                            c"()Lorg/femacs/EmacsFontDriver;"  // C 219-220
                                                             .as_ptr ());
  eassert! (! font_driver_class.create_font_driver.is_null ());
}

/* Initialize the class and field identifiers for functions in the
   EmacsFontDriver$FontSpec class, and place them in `font_spec_class'.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_font_spec ()                       // C 229-268
{
  let mut old: jclass;

  font_spec_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
                                                    c"org/femacs/EmacsFontDriver$FontSpec"  // C 236-237
                                                     .as_ptr ());
  eassert! (! font_spec_class.class.is_null ());

  old = font_spec_class.class;
  font_spec_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if font_spec_class.class.is_null ()
    { emacs_abort (); }

  /* C 249-254: `FIND_FIELD`, `#undef`ed at C 267 (see `FIND_METHOD` above).  */
  macro_rules! FIND_FIELD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      font_spec_class.$c_name
        = ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,
                                                         font_spec_class.class,
                                                         $name, $signature);
      eassert! (! font_spec_class.$c_name.is_null ());
    };
  }

  FIND_FIELD! (foundry, c"foundry".as_ptr (), c"Ljava/lang/String;".as_ptr ());  // C 256
  FIND_FIELD! (family, c"family".as_ptr (), c"Ljava/lang/String;".as_ptr ());    // C 257
  FIND_FIELD! (adstyle, c"adstyle".as_ptr (), c"Ljava/lang/String;".as_ptr ());  // C 258
  FIND_FIELD! (registry, c"registry".as_ptr (), c"Ljava/lang/String;".as_ptr ());  // C 259
  FIND_FIELD! (width, c"width".as_ptr (), c"Ljava/lang/Integer;".as_ptr ());     // C 260
  FIND_FIELD! (weight, c"weight".as_ptr (), c"Ljava/lang/Integer;".as_ptr ());   // C 261
  FIND_FIELD! (slant, c"slant".as_ptr (), c"Ljava/lang/Integer;".as_ptr ());     // C 262
  FIND_FIELD! (size, c"size".as_ptr (), c"Ljava/lang/Integer;".as_ptr ());       // C 263
  FIND_FIELD! (spacing, c"spacing".as_ptr (), c"Ljava/lang/Integer;".as_ptr ());  // C 264
  FIND_FIELD! (avgwidth, c"avgwidth".as_ptr (), c"Ljava/lang/Integer;".as_ptr ());  // C 265
  FIND_FIELD! (dpi, c"dpi".as_ptr (), c"Ljava/lang/Integer;".as_ptr ());         // C 266
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_font_metrics ()                    // C 270-303
{
  let mut old: jclass;

  font_metrics_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
                                                    c"org/femacs/EmacsFontDriver$FontMetrics"  // C 276-278
                                                     .as_ptr ());
  eassert! (! font_metrics_class.class.is_null ());

  old = font_metrics_class.class;
  font_metrics_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if font_metrics_class.class.is_null ()
    { emacs_abort (); }

  /* C 290-295: `FIND_FIELD`, `#undef`ed at C 302 (see `FIND_METHOD` above).  */
  macro_rules! FIND_FIELD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      font_metrics_class.$c_name
        = ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,
                                                         font_metrics_class.class,
                                                         $name, $signature);
      eassert! (! font_metrics_class.$c_name.is_null ());
    };
  }

  FIND_FIELD! (lbearing, c"lbearing".as_ptr (), c"S".as_ptr ());   // C 297
  FIND_FIELD! (rbearing, c"rbearing".as_ptr (), c"S".as_ptr ());   // C 298
  FIND_FIELD! (width, c"width".as_ptr (), c"S".as_ptr ());         // C 299
  FIND_FIELD! (ascent, c"ascent".as_ptr (), c"S".as_ptr ());       // C 300
  FIND_FIELD! (descent, c"descent".as_ptr (), c"S".as_ptr ());     // C 301
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_integer ()                         // C 305-334
{
  let mut old: jclass;

  integer_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
                                                    c"java/lang/Integer".as_ptr ());
  eassert! (! integer_class.class.is_null ());

  old = integer_class.class;
  integer_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if integer_class.class.is_null ()
    { emacs_abort (); }

  /* C 324-329: `FIND_METHOD`, `#undef`ed at C 333 (see `FIND_METHOD` above).  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      integer_class.$c_name
        = ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
                                                          integer_class.class,
                                                          $name, $signature);
      eassert! (! integer_class.$c_name.is_null ());
    };
  }

  FIND_METHOD! (constructor, c"<init>".as_ptr (), c"(I)V".as_ptr ());       // C 331
  FIND_METHOD! (int_value, c"intValue".as_ptr (), c"()I".as_ptr ());        // C 332
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_font_object ()                     // C 336-379
{
  let mut old: jclass;

  font_object_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
                                                    c"org/femacs/EmacsFontDriver$FontObject"  // C 341-344
                                                     .as_ptr ());
  eassert! (! font_object_class.class.is_null ());

  old = font_object_class.class;
  font_object_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if font_object_class.class.is_null ()
    { emacs_abort (); }

  /* C 356-361: `FIND_FIELD`, `#undef`ed at C 378 (see `FIND_METHOD` above).  */
  macro_rules! FIND_FIELD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      font_object_class.$c_name
        = ((*(*android_java_env)).GetFieldID.unwrap ()) (android_java_env,
                                                         font_object_class.class,
                                                         $name, $signature);
      eassert! (! font_object_class.$c_name.is_null ());
    };
  }

  FIND_FIELD! (min_width, c"minWidth".as_ptr (), c"I".as_ptr ());                    // C 363
  FIND_FIELD! (max_width, c"maxWidth".as_ptr (), c"I".as_ptr ());                    // C 364
  FIND_FIELD! (pixel_size, c"pixelSize".as_ptr (), c"I".as_ptr ());                  // C 365
  FIND_FIELD! (height, c"height".as_ptr (), c"I".as_ptr ());                         // C 366
  FIND_FIELD! (space_width, c"spaceWidth".as_ptr (), c"I".as_ptr ());                // C 367
  FIND_FIELD! (average_width, c"averageWidth".as_ptr (), c"I".as_ptr ());            // C 368
  FIND_FIELD! (ascent, c"ascent".as_ptr (), c"I".as_ptr ());                         // C 369
  FIND_FIELD! (descent, c"descent".as_ptr (), c"I".as_ptr ());                       // C 370
  FIND_FIELD! (underline_thickness, c"underlineThickness".as_ptr (), c"I".as_ptr ());  // C 371
  FIND_FIELD! (underline_position, c"underlinePosition".as_ptr (), c"I".as_ptr ());  // C 372
  FIND_FIELD! (baseline_offset, c"baselineOffset".as_ptr (), c"I".as_ptr ());        // C 373
  FIND_FIELD! (relative_compose, c"relativeCompose".as_ptr (), c"I".as_ptr ());      // C 374
  FIND_FIELD! (default_ascent, c"defaultAscent".as_ptr (), c"I".as_ptr ());          // C 375
  FIND_FIELD! (encoding_charset, c"encodingCharset".as_ptr (), c"I".as_ptr ());      // C 376
  FIND_FIELD! (repertory_charset, c"repertoryCharset".as_ptr (), c"I".as_ptr ());    // C 377
}

/* The address of this function is taken (C 1042), so it is `unsafe extern "C"`; `struct frame *`
   is `*mut c_void` in the mirror's `font_driver` record (P-72 ③), so the port spells the parameter
   the way every includer of the mirror can resolve it.  */
/// C: `static Lisp_Object androidfont_get_cache (struct frame *frame);`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn androidfont_get_cache (frame: *mut c_void) -> Lisp_Object   // C 381-385
{
  return font_cache;
}

/* Initialize the Java side of the font driver if it has not already
   been initialized.  This is only done whenever necessary because the
   font driver otherwise uses a lot of memory, as it has to keep every
   typeface open.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn androidfont_check_init ()                       // C 392-420
{
  let mut method: jmethodID;
  let mut old: jobject;

  if ! font_driver.is_null ()
    { return; }

  /* Log a loud message.  This font driver really should not be
     used.  */
  /* C 403-405: `__android_log_print (ANDROID_LOG_WARN, __func__, "…" "…")` - C's `__func__` is
     this function's own name, and the port keeps the name (the androidmenu.rs shape).  */
  let __func__: *const c_char = c"androidfont_check_init".as_ptr ();
  __android_log_print (ANDROID_LOG_WARN, __func__,
                       c"The Android font driver is being used.  Please investigate why this is so."
                        .as_ptr ());

  method = font_driver_class.create_font_driver;

  /* Initialize the font driver on the Java side.  */
  font_driver
    = ((*(*android_java_env)).CallStaticObjectMethod.unwrap ()) (android_java_env,
                                                                font_driver_class.class,
                                                                method);
  android_exception_check ();

  old = font_driver;
  font_driver
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, font_driver);
  ANDROID_DELETE_LOCAL_REF! (old);
}

/* Return a local reference to an instance of EmacsFontDriver$FontSpec
   with the same values as FONT.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn androidfont_from_lisp (font: Lisp_Object) -> jobject   // C 425-493
{
  let mut spec: jobject;
  let mut integer: jobject;
  let mut string: jstring;
  let mut tem: Lisp_Object;

  spec = ((*(*android_java_env)).AllocObject.unwrap ()) (android_java_env,
                                                         font_spec_class.class);
  android_exception_check ();

  /* C 436-450: `DO_SYMBOL_FIELD`, `#undef`ed at C 457 (see `FIND_METHOD` above).  */
  macro_rules! DO_SYMBOL_FIELD {
    ($field:ident, $index:expr) => {
      tem = AREF (font, $index as isize);
      if SYMBOLP (tem)
        {
          /* Java seems to DTRT with the Emacs string encoding, so this does
             not matter at all.  */
          string = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,
                                                                    SSDATA (SYMBOL_NAME (tem)));
          android_exception_check_1 (spec);

          ((*(*android_java_env)).SetObjectField.unwrap ()) (android_java_env, spec,
                                                             font_spec_class.$field,
                                                             string);
          ANDROID_DELETE_LOCAL_REF! (string);
        }
    };
  }

  DO_SYMBOL_FIELD! (foundry, FONT_FOUNDRY_INDEX);           // C 452
  DO_SYMBOL_FIELD! (family, FONT_FAMILY_INDEX);             // C 453
  DO_SYMBOL_FIELD! (adstyle, FONT_ADSTYLE_INDEX);           // C 454
  DO_SYMBOL_FIELD! (registry, FONT_REGISTRY_INDEX);         // C 455

  /* C 459-472: `DO_CARDINAL_FIELD`, `#undef`ed at C 490 (see `FIND_METHOD` above).  `NewObject`
     is a `<jni.h>` variadic entry point; the argument C passes after the method id is the value.  */
  macro_rules! DO_CARDINAL_FIELD {
    ($field:ident, $value:expr) => {
      if $value != -1
        {
          integer = ((*(*android_java_env)).NewObject.unwrap ()) (android_java_env,
                                                                  integer_class.class,
                                                                  integer_class.constructor,
                                                                  $value as jint);
          android_exception_check_1 (spec);

          ((*(*android_java_env)).SetObjectField.unwrap ()) (android_java_env, spec,
                                                             font_spec_class.$field,
                                                             integer);
          ANDROID_DELETE_LOCAL_REF! (integer);
        }
    };
  }

  DO_CARDINAL_FIELD! (width, FONT_WIDTH_NUMERIC! (font));            // C 474
  DO_CARDINAL_FIELD! (weight, FONT_WEIGHT_NUMERIC! (font));          // C 475
  DO_CARDINAL_FIELD! (slant, FONT_SLANT_NUMERIC! (font));            // C 476
  DO_CARDINAL_FIELD! (size, (if FIXNUMP (AREF (font, FONT_SIZE_INDEX as isize))    // C 477-479
                             { XFIXNUM (AREF (font, FONT_SIZE_INDEX as isize)) }
                             else { -1 }));
  DO_CARDINAL_FIELD! (spacing, (if FIXNUMP (AREF (font, FONT_SPACING_INDEX as isize))  // C 480-482
                                { XFIXNUM (AREF (font, FONT_SPACING_INDEX as isize)) }
                                else { -1 }));
  DO_CARDINAL_FIELD! (avgwidth, (if FIXNUMP (AREF (font, FONT_AVGWIDTH_INDEX as isize))  // C 483-485
                                 { XFIXNUM (AREF (font, FONT_AVGWIDTH_INDEX as isize)) }
                                 else { -1 }));
  DO_CARDINAL_FIELD! (dpi, (if FIXNUMP (AREF (font, FONT_DPI_INDEX as isize))      // C 486-488
                            { XFIXNUM (AREF (font, FONT_DPI_INDEX as isize)) }
                            else { -1 }));

  return spec;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn androidfont_from_java (spec: jobject, entity: Lisp_Object)   // C 495-550
{
  let mut tem: jobject;
  let mut value: jint;
  let mut string: *const c_char;

  /* C 502-516: `DO_SYMBOL_FIELD`, `#undef`ed at C 523 (see `FIND_METHOD` above).  */
  macro_rules! DO_SYMBOL_FIELD {
    ($field:ident, $index:expr) => {
      tem = ((*(*android_java_env)).GetObjectField.unwrap ()) (android_java_env,
                                                               spec,
                                                               font_spec_class.$field);
      if ! tem.is_null ()
        {
          string = ((*(*android_java_env)).GetStringUTFChars.unwrap ()) (android_java_env,
                                                                         tem,
                                                                         std::ptr::null_mut ());
          if string.is_null ()
            { memory_full (0); }
          ASET (entity, $index as isize, intern (string));
          ((*(*android_java_env)).ReleaseStringUTFChars.unwrap ()) (android_java_env,
                                                                    tem, string);
          ANDROID_DELETE_LOCAL_REF! (tem);
        }
    };
  }

  DO_SYMBOL_FIELD! (foundry, FONT_FOUNDRY_INDEX);           // C 518
  DO_SYMBOL_FIELD! (family, FONT_FAMILY_INDEX);             // C 519
  DO_SYMBOL_FIELD! (adstyle, FONT_ADSTYLE_INDEX);           // C 520
  DO_SYMBOL_FIELD! (registry, FONT_REGISTRY_INDEX);         // C 521

  /* C 524-539: `DO_CARDINAL_FIELD`, `#undef`ed at C 549 (see `FIND_METHOD` above).  */
  macro_rules! DO_CARDINAL_FIELD {
    ($field:ident, $index:expr, $is_style:expr) => {
      tem = ((*(*android_java_env)).GetObjectField.unwrap ()) (android_java_env,
                                                               spec,
                                                               font_spec_class.$field);
      if ! tem.is_null ()
        {
          value
            = ((*(*android_java_env)).CallIntMethod.unwrap ()) (android_java_env,
                                                                tem,
                                                                integer_class.int_value);
          if ! $is_style
            { ASET (entity, $index as isize, make_fixnum (value as EMACS_INT)); }
          else
            { FONT_SET_STYLE! (entity, $index, make_fixnum (value as EMACS_INT)); }
          ANDROID_DELETE_LOCAL_REF! (tem);
        }
    };
  }

  DO_CARDINAL_FIELD! (width, FONT_WIDTH_INDEX, true);       // C 541
  DO_CARDINAL_FIELD! (weight, FONT_WEIGHT_INDEX, true);     // C 542
  DO_CARDINAL_FIELD! (slant, FONT_SLANT_INDEX, true);       // C 543
  DO_CARDINAL_FIELD! (size, FONT_SIZE_INDEX, false);        // C 544
  DO_CARDINAL_FIELD! (spacing, FONT_SPACING_INDEX, false);  // C 545
  DO_CARDINAL_FIELD! (avgwidth, FONT_AVGWIDTH_INDEX, false);  // C 546
  DO_CARDINAL_FIELD! (dpi, FONT_DPI_INDEX, false);          // C 547
}

/* Transfer the values from FONT, which must be some kind of font
   entity, */

/// C: `static Lisp_Object androidfont_list (struct frame *f, Lisp_Object font_spec);` - the frame
/// is `*mut c_void` in the mirror's field (P-72 ③).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn androidfont_list (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{                                                          // C 555-608
  let mut spec: jobject;
  let mut array: jobject;
  let mut tem: jobject;
  let mut entities: jarray;
  let mut i: jsize;
  let mut size: jsize;
  let mut value: Lisp_Object;
  let mut entity: Lisp_Object;
  let mut info: *mut androidfont_entity;

  /* Maybe initialize the font driver.  */
  androidfont_check_init ();

  spec = androidfont_from_lisp (font_spec);
  array = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env,
                                                              font_driver,
                                                              font_driver_class.list,
                                                              spec);
  android_exception_check_1 (spec);
  ANDROID_DELETE_LOCAL_REF! (spec);

  entities = array as jarray;
  size = ((*(*android_java_env)).GetArrayLength.unwrap ()) (android_java_env, entities);
  value = Qnil ();

  i = 0;
  while i < size
    {
      entity = font_make_entity_android (VECSIZE (std::mem::size_of::<androidfont_entity> ())
                                         as c_int);
      info = XFONT_ENTITY (entity) as *mut androidfont_entity;

      /* The type must be set correctly, or font_open_entity won't be
	 able to find the right font driver.  */
      ASET (entity, FONT_TYPE_INDEX as isize, Qandroid ());

      /* Clear this now in case GC happens without it set, which can
	 happen if androidfont_from_java runs out of memory.  */
      (*info).object = std::ptr::null_mut ();

      tem = ((*(*android_java_env)).GetObjectArrayElement.unwrap ()) (android_java_env,
                                                                      entities, i);
      androidfont_from_java (tem, entity);

      /* Now, make a global reference to the Java font entity.  */
      (*info).object = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,
                                                                        tem as jobject);
      android_exception_check_2 (tem, entities);
      ANDROID_DELETE_LOCAL_REF! (tem);

      value = Fcons (entity, value);
      i += 1;
    }

  ANDROID_DELETE_LOCAL_REF! (entities);
  return Fnreverse (value);
}

/// C: `static Lisp_Object androidfont_match (struct frame *f, Lisp_Object font_spec);` (untyped
/// frame, see androidfont_list).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn androidfont_match (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{                                                          // C 610-643
  let mut spec: jobject;
  let mut result: jobject;
  let mut entity: Lisp_Object;
  let mut info: *mut androidfont_entity;

  /* Maybe initialize the font driver.  */
  androidfont_check_init ();

  spec = androidfont_from_lisp (font_spec);
  result = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env,
                                                               font_driver,
                                                               font_driver_class.r#match,
                                                               spec);
  android_exception_check_1 (spec);
  ANDROID_DELETE_LOCAL_REF! (spec);

  entity = font_make_entity_android (VECSIZE (std::mem::size_of::<androidfont_entity> ())
                                     as c_int);
  info = XFONT_ENTITY (entity) as *mut androidfont_entity;

  /* The type must be set correctly, or font_open_entity won't be able
     to find the right font driver.  */
  ASET (entity, FONT_TYPE_INDEX as isize, Qandroid ());

  (*info).object = std::ptr::null_mut ();
  androidfont_from_java (result, entity);
  (*info).object = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,
                                                                    result as jobject);
  android_exception_check_1 (result);
  ANDROID_DELETE_LOCAL_REF! (result);

  return entity;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn androidfont_draw (s: *mut glyph_string, from: c_int, to: c_int,
                                       x: c_int, y: c_int, with_background: bool) -> c_int
{                                                          // C 645-685
  let mut info: *mut androidfont_info;
  let mut chars: jarray;
  let mut rc: c_int;
  let mut gcontext: jobject;
  let mut drawable: jobject;

  /* Maybe initialize the font driver.  */
  androidfont_check_init ();

  static_assert! (std::mem::size_of::<c_uint> () == std::mem::size_of::<jint> ());
  info = (*s).font as *mut androidfont_info;

  /* C 1467's `struct android_gc *gc` is `*mut c_void` in the glyph_string record (the mirror's
     `have_android` arm, P-72 ③), so the port casts at the use site.  */
  gcontext = android_resolve_handle ((*( (*s).gc as *mut android_gc )).gcontext);
  /* C 661: `android_resolve_handle (FRAME_ANDROID_DRAWABLE (s->f))` - the macro's own body is
     androidterm.h 305-307's, carried by include/androidterm.rs.  */
  drawable = android_resolve_handle (FRAME_ANDROID_DRAWABLE! ((*s).f));
  chars = ((*(*android_java_env)).NewIntArray.unwrap ()) (android_java_env, to - from);
  android_exception_check ();

  ((*(*android_java_env)).SetIntArrayRegion.unwrap ())
    (android_java_env, chars, 0, to - from,
     ((*s).char2b as *const jint).offset (from as isize));

  info = (*s).font as *mut androidfont_info;
  prepare_face_for_display ((*s).f, (*s).face);

  /* C 678-680: `(jint) x`, `(jint) y`, `(jint) s->width` and `(jboolean) with_background` go
     through C's `<jni.h>` variadic call; the trailing `c_int` cast on the boolean is C's own
     default argument promotion (the androidmenu.md's `(jboolean)` rule).  */
  rc = ((*(*android_java_env)).CallIntMethod.unwrap ()) (android_java_env,
                                                         font_driver,
                                                         font_driver_class.draw,
                                                         (*info).object,
                                                         gcontext, drawable,
                                                         chars, x as jint, y as jint,
                                                         (*s).width as jint,
                                                         with_background as jboolean as c_int);
  android_exception_check_1 (chars);
  ANDROID_DELETE_LOCAL_REF! (chars);

  return rc;
}

/// C: `static Lisp_Object androidfont_open_font (struct frame *f, Lisp_Object font_entity, int
/// pixel_size);` (untyped frame, see androidfont_list; this is the one of the five that reads it,
/// so the cast P-72 ③ prescribes is at its two use sites below).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn androidfont_open_font (f: *mut c_void, font_entity: Lisp_Object,
                                            pixel_size: c_int) -> Lisp_Object
{                                                          // C 687-780
  let mut font_info: *mut androidfont_info;
  let mut entity: *mut androidfont_entity;
  let mut font: *mut font;
  let mut font_object: Lisp_Object;
  let mut old: jobject;
  let mut value: jint;
  let mut pixel_size = pixel_size;   // C 701-712 reassigns the parameter

  /* Maybe initialize the font driver.  */
  androidfont_check_init ();

  if XFIXNUM (AREF (font_entity, FONT_SIZE_INDEX as isize)) != 0
    { pixel_size = XFIXNUM (AREF (font_entity, FONT_SIZE_INDEX as isize)) as c_int; }
  else if pixel_size == 0
    {
      /* This bit was copied from xfont.c.  The values might need
	 adjustment.  */

      if ! FRAME_FONT! ((f as *mut frame)).is_null ()
	{ pixel_size = (*FRAME_FONT! ((f as *mut frame))).pixel_size; }
      else
	{ pixel_size = 12; }
    }

  entity = XFONT_ENTITY (font_entity) as *mut androidfont_entity;

  block_input ();
  font_object = font_make_object (VECSIZE (std::mem::size_of::<androidfont_info> ()) as c_int,
				  font_entity, pixel_size);
  ASET (font_object, FONT_TYPE_INDEX as isize, Qandroid ());
  font_info = XFONT_OBJECT (font_object) as *mut androidfont_info;
  font = &raw mut (*font_info).font;
  (*font).driver = &raw mut androidfont_driver as *const c_void;

  /* Clear font_info->object and font_info->metrics early in case GC
     happens later on! */
  (*font_info).object = std::ptr::null_mut ();
  (*font_info).metrics = std::ptr::null_mut ();
  unblock_input ();

  (*font_info).object
    = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env,
                                                           font_driver,
                                                           font_driver_class.open_font,
                                                           (*entity).object,
                                                           pixel_size as jint);
  android_exception_check ();

  old = (*font_info).object;
  (*font_info).object
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old);
  android_exception_check_1 (old);
  ANDROID_DELETE_LOCAL_REF! (old);

  if (*font_info).object.is_null ()
    { return Qnil (); }

  /* Copy the font attributes from the Java object.  */
  androidfont_from_java ((*font_info).object, font_object);

  /* Copy font attributes inside EmacsFontDriver$FontObject.  */
  /* C 751-756: `DO_CARDINAL_FIELD`, `#undef`ed at C 774 (see `FIND_METHOD` above).  */
  macro_rules! DO_CARDINAL_FIELD {
    ($field:ident) => {
      value
        = ((*(*android_java_env)).GetIntField.unwrap ()) (android_java_env,
                                                          (*font_info).object,
                                                          font_object_class.$field);
      (*font).$field = value;
    };
  }

  DO_CARDINAL_FIELD! (min_width);            // C 758
  DO_CARDINAL_FIELD! (max_width);            // C 759
  DO_CARDINAL_FIELD! (pixel_size);           // C 760
  DO_CARDINAL_FIELD! (height);               // C 761
  DO_CARDINAL_FIELD! (space_width);          // C 762
  DO_CARDINAL_FIELD! (average_width);        // C 763
  DO_CARDINAL_FIELD! (ascent);               // C 764
  DO_CARDINAL_FIELD! (descent);              // C 765
  DO_CARDINAL_FIELD! (underline_thickness);  // C 766
  DO_CARDINAL_FIELD! (underline_position);   // C 767
  DO_CARDINAL_FIELD! (baseline_offset);      // C 768
  DO_CARDINAL_FIELD! (relative_compose);     // C 769
  DO_CARDINAL_FIELD! (default_ascent);       // C 770
  DO_CARDINAL_FIELD! (encoding_charset);     // C 771
  DO_CARDINAL_FIELD! (repertory_charset);    // C 772

  /* This should eventually become unnecessary.  */
  (*font).props[FONT_NAME_INDEX as usize] = Ffont_xlfd_name (font_object, Qnil (), Qt ());

  return font_object;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn androidfont_close_font (font: *mut font)   // C 782-816
{
  let mut info: *mut androidfont_info;
  let mut i: c_int;

  /* Maybe initialize the font driver.  */
  androidfont_check_init ();

  info = font as *mut androidfont_info;

  /* Free the font metrics cache if it exists.  */

  if ! (*info).metrics.is_null ()
    {
      i = 0;
      while i < 256
	{
	  xfree (*(*info).metrics.offset (i as isize) as *mut c_void);
	  i += 1;
	}
      xfree ((*info).metrics as *mut c_void);
    }

  (*info).metrics = std::ptr::null_mut ();

  /* If info->object is NULL, then FONT was unsuccessfully created,
     and there is no global reference that has to be deleted.

     Alternatively, FONT may have been closed by font_close_object,
     with this function called from GC.  */

  if (*info).object.is_null ()
    { return; }

  ((*(*android_java_env)).DeleteGlobalRef.unwrap ()) (android_java_env, (*info).object);
  (*info).object = std::ptr::null_mut ();
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn androidfont_has_char (font: Lisp_Object, c: c_int) -> c_int  // C 818-845
{
  let mut info: *mut androidfont_info;
  let mut entity: *mut androidfont_entity;

  /* Maybe initialize the font driver.  */
  androidfont_check_init ();

  if FONT_ENTITY_P (font)
    {
      entity = XFONT_ENTITY (font) as *mut androidfont_entity;

      return ((*(*android_java_env)).CallIntMethod.unwrap ()) (android_java_env,
                                                               font_driver,
                                                               font_driver_class.has_char,
                                                               (*entity).object, c as jint);
    }
  else
    {
      info = XFONT_OBJECT (font) as *mut androidfont_info;

      return ((*(*android_java_env)).CallIntMethod.unwrap ()) (android_java_env,
                                                               font_driver,
                                                               font_driver_class.has_char,
                                                               (*info).object, c as jint);
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn androidfont_encode_char (font: *mut font, c: c_int) -> c_uint  // C 847-861
{
  let mut info: *mut androidfont_info;

  /* Maybe initialize the font driver.  */
  androidfont_check_init ();

  info = font as *mut androidfont_info;

  /* C 860: `(jchar) c` through the variadic call, promoted as above.  */
  return ((*(*android_java_env)).CallIntMethod.unwrap ()) (android_java_env,
                                                           font_driver,
                                                           font_driver_class.encode_char,
                                                           (*info).object, c as jchar as c_int)
    as c_uint;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn androidfont_cache_text_extents (info: *mut androidfont_info, glyph: c_uint,
					  metrics: *mut font_metrics)   // C 863-890
{
  let mut i: c_int;

  /* Glyphs larger than 65535 can't be cached.  */
  if glyph >= 256 * 256
    { return; }

  if (*info).metrics.is_null ()
    {
      (*info).metrics = xzalloc (256 * std::mem::size_of::<*mut font_metrics> ())
	as *mut *mut font_metrics;
    }

  if (*(*info).metrics.offset ((glyph / 256) as isize)).is_null ()
    {
      *(*info).metrics.offset ((glyph / 256) as isize)
	= xnmalloc (256, std::mem::size_of::<font_metrics> () as isize) as *mut font_metrics;

      /* Now, all the metrics in that array as invalid by setting
	 lbearing to SHRT_MAX.  */
      i = 0;
      while i < 256
	{
	  (*(*(*info).metrics.offset ((glyph / 256) as isize)).offset (i as isize)).lbearing
	    = SHRT_MAX as c_short;
	  i += 1;
	}
    }

  /* Finally, cache the glyph.  C 889's `info->metrics[a][b] = *metrics;` is a whole-C-struct
     assignment, which a non-`Copy` Rust record spells with `ptr::read` (the callproc.rs
     `coding_system` shape).  */
  *(*(*info).metrics.offset ((glyph / 256) as isize)).offset ((glyph % 256) as isize)
    = std::ptr::read (metrics);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn androidfont_check_cached_extents (info: *mut androidfont_info, glyph: c_uint,
					    metrics: *mut font_metrics) -> bool  // C 892-905
{
  if ! (*info).metrics.is_null ()
    && ! (*(*info).metrics.offset ((glyph / 256) as isize)).is_null ()
    && (*(*(*info).metrics.offset ((glyph / 256) as isize)).offset ((glyph % 256) as isize))
         .lbearing != SHRT_MAX as c_short
    {
      *metrics = std::ptr::read ((*(*info).metrics.offset ((glyph / 256) as isize))
                                 .offset ((glyph % 256) as isize));
      return true;
    }

  return false;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn androidfont_text_extents (font: *mut font, code: *const c_uint,
					       nglyphs: c_int,
					       metrics: *mut font_metrics)   // C 907-985
{
  let mut info: *mut androidfont_info;
  let mut codepoint_array: jarray;
  let mut metrics_object: jobject;
  let mut value: c_short;

  /* Maybe initialize the font driver.  */
  androidfont_check_init ();

  info = font as *mut androidfont_info;

  if nglyphs == 1
    && androidfont_check_cached_extents (info, *code, metrics)
    { return; }

  /* Allocate the arrays of code points and font metrics.  */
  codepoint_array
    = ((*(*android_java_env)).NewIntArray.unwrap ()) (android_java_env, nglyphs);
  if codepoint_array.is_null ()
    {
      ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);
      memory_full (0);
    }

  static_assert! (std::mem::size_of::<c_uint> () == std::mem::size_of::<jint> ());

  /* Always true on every Android device.  */
  ((*(*android_java_env)).SetIntArrayRegion.unwrap ())
    (android_java_env, codepoint_array, 0, nglyphs, code as *const jint);

  metrics_object
    = ((*(*android_java_env)).AllocObject.unwrap ()) (android_java_env,
                                                      font_metrics_class.class);

  ((*(*android_java_env)).CallVoidMethod.unwrap ()) (android_java_env,
                                                     font_driver,
                                                     font_driver_class.text_extents,
                                                     (*info).object, codepoint_array,
                                                     metrics_object);

  if (*(*android_java_env)).ExceptionCheck.unwrap () (android_java_env) != 0
    {
      ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);
      ANDROID_DELETE_LOCAL_REF! (metrics_object);
      ANDROID_DELETE_LOCAL_REF! (codepoint_array);
      memory_full (0);
    }

  /* C 961-966: `DO_CARDINAL_FIELD`, `#undef`ed at C 974 (see `FIND_METHOD` above).  C reads a
     `<jni.h>` `jshort` and stores it in the record's `short` member.  */
  macro_rules! DO_CARDINAL_FIELD {
    ($field:ident) => {
      value
        = ((*(*android_java_env)).GetShortField.unwrap ()) (android_java_env,
                                                            metrics_object,
                                                            font_metrics_class.$field);
      (*metrics).$field = value;
    };
  }

  DO_CARDINAL_FIELD! (lbearing);   // C 968
  DO_CARDINAL_FIELD! (rbearing);   // C 969
  DO_CARDINAL_FIELD! (width);      // C 970
  DO_CARDINAL_FIELD! (ascent);     // C 971
  DO_CARDINAL_FIELD! (descent);    // C 972

  ANDROID_DELETE_LOCAL_REF! (metrics_object);
  ANDROID_DELETE_LOCAL_REF! (codepoint_array);

  /* Emacs spends a lot of time in androidfont_text_extents, which
     makes calling JNI too slow.  Cache the metrics for this single
     glyph.  */

  if nglyphs == 1
    { androidfont_cache_text_extents (info, *code, metrics); }
}

/// C: `static Lisp_Object androidfont_list_family (struct frame *f);` (untyped frame, see
/// androidfont_list).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn androidfont_list_family (f: *mut c_void) -> Lisp_Object  // C 987-1036
{
  let mut families: Lisp_Object;
  let mut family_array: jarray;
  let mut string: jobject;
  let mut i: jsize;
  let mut length: jsize;
  let mut family: *const c_char;

  /* Return if the Android font driver is not initialized.  Loading
     every font under Android takes a non trivial amount of memory,
     and is not something that should be done when the user tries to
     list all of the font families.  */

  if font_driver.is_null ()
    { return Qnil (); }

  family_array
    = ((*(*android_java_env)).CallObjectMethod.unwrap ()) (android_java_env,
                                                           font_driver,
                                                           font_driver_class.list_families);
  android_exception_check ();

  length = ((*(*android_java_env)).GetArrayLength.unwrap ()) (android_java_env,
							      family_array);
  families = Qnil ();

  i = 0;
  while i < length
    {
      string = ((*(*android_java_env)).GetObjectArrayElement.unwrap ()) (android_java_env,
									 family_array, i);
      family = ((*(*android_java_env)).GetStringUTFChars.unwrap ()) (android_java_env,
								      string as jstring,
								      std::ptr::null_mut ());

      if family.is_null ()
	{
	  ANDROID_DELETE_LOCAL_REF! (string);
	  ANDROID_DELETE_LOCAL_REF! (family_array);
	}

      /* C 1027 passes `string` (a `jobject`) where `build_string_from_utf8` wants a
	 `const char *`: C converts the `void *` implicitly, and the port keeps C's own text
	 with the cast spelled out (see the ledger's Deviations - this is the reference's line,
	 not a port choice).  */
      families = Fcons (build_string_from_utf8 (string as *const c_char), families);
      ((*(*android_java_env)).ReleaseStringUTFChars.unwrap ()) (android_java_env,
								string as jstring,
								family);
      ANDROID_DELETE_LOCAL_REF! (string);
      i += 1;
    }

  ANDROID_DELETE_LOCAL_REF! (family_array);
  return Fnreverse (families);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut androidfont_driver: font_driver = font_driver {   // C 1038-1052
  /* C 1040: `.type = LISPSYM_INITIALLY (Qandroid)`.  The mirror's `LISPSYM_INITIALLY` is not a
     `const fn`, so the port writes the same constant expression over the generated `iQandroid`
     index (the fns.rs `hashtest_eq` precedent, lisp.rs 3211).  */
  r#type: (((iQandroid * std::mem::size_of::<Lisp_Symbol> ()) as EMACS_UINT
	    + Lisp_Type::Lisp_Symbol as EMACS_UINT) as Lisp_Object),
  case_sensitive: true,                                            // C 1041
  get_cache: Some (androidfont_get_cache),                         // C 1042
  list: Some (androidfont_list),                                   // C 1043
  r#match: Some (androidfont_match),                               // C 1044
  #[cfg (feature = "have_window_system")]
  draw: Some (androidfont_draw),                                   // C 1045
  open_font: Some (androidfont_open_font),                         // C 1046
  close_font: Some (androidfont_close_font),                       // C 1047
  has_char: Some (androidfont_has_char),                           // C 1048
  encode_char: Some (androidfont_encode_char),                     // C 1049
  text_extents: Some (androidfont_text_extents),                   // C 1050
  list_family: Some (androidfont_list_family),                     // C 1051

  /* The members C's designated initialiser leaves implicit (i.e. zero): the port writes them out
     because Rust has no designated initialisers.  `get_bitmap` and `free_bitmap` are the two of the
     three `font_driver` members C guards with `#ifdef HAVE_WINDOW_SYSTEM` (font.h 682-698, the
     same condition the mirror spells `feature = "have_window_system"`; `draw` above carries it
     too), so they cannot be named in a configuration without it - exactly as C cannot.  The last
     two carry the mirror's `harfbuzz` condition for the same reason.  */
  free_entity: None,
  prepare_face: None,
  done_face: None,
  anchor_point: None,
  #[cfg (feature = "have_window_system")]
  get_bitmap: None,
  #[cfg (feature = "have_window_system")]
  free_bitmap: None,
  otf_capability: None,
  otf_drive: None,
  start_for_frame: None,
  end_for_frame: None,
  shape: None,
  check: None,
  get_variation_glyphs: None,
  filter_properties: None,
  cached_font_ok: None,
  combining_capability: None,
  drop_xrender_surfaces: None,
  #[cfg (feature = "harfbuzz")]
  begin_hb_font: None,
  #[cfg (feature = "harfbuzz")]
  end_hb_font: None,
};

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn syms_of_androidfont_for_pdumper ()   // C 1054-1058
{
  register_font_driver (&raw mut androidfont_driver, std::ptr::null_mut ());
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn syms_of_androidfont ()                      // C 1060-1069
{
  DEFSYM! (Qfontsize, "fontsize");

  pdumper_do_now_and_after_load (Some (syms_of_androidfont_for_pdumper));

  font_cache = list! (Qnil ());
  staticpro (&raw mut font_cache);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn init_androidfont ()                         // C 1071-1085
{
  if ! android_init_gui
    { return; }

  android_init_font_driver ();
  android_init_font_spec ();
  android_init_font_metrics ();
  android_init_font_object ();
  android_init_integer ();

  /* The Java font driver is not initialized here because it uses a lot
     of memory.  */
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_finalize_font_entity (entity: *mut font_entity)   // C 1087-1100
{
  let mut info: *mut androidfont_entity;

  info = entity as *mut androidfont_entity;

  if ! (*info).object.is_null ()
    {
      ((*(*android_java_env)).DeleteGlobalRef.unwrap ()) (android_java_env, (*info).object);
    }

  /* Not sure if this can be called twice.  */
  (*info).object = std::ptr::null_mut ();
}

/* C 1102: `#endif` - this file has no `#else` arm, so C's stubify configuration compiles the
   include graph above and no item at all (the androidmenu.c contrast: that file's `#else` carries
   a stub `popup_activated`).  */
