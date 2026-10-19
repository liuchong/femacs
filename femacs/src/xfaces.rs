// femacs: port of src/xfaces.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  C's non-`static`
// definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2/P-52) and the `DEFUN`s use `DEFUN!`
// (PORTING.md 5.1); the `static` helpers stay plain `unsafe fn`.
//
// **The dead region is recorded, not transcribed.**  `src/xfaces.c` *is* in
// `docs/baseline/config/cond-eval-verified.txt`, so `tools/extract-abi.py` evaluates every
// `#if` against the frozen macro table and compares only the text the frozen build compiles
// (PORTING.md P-08/P-66).  The frozen configuration is the minimal macOS one
// (`docs/baseline/config/darwin-arm64.macros.tsv`): `HAVE_WINDOW_SYSTEM`, `HAVE_X_WINDOWS`,
// `HAVE_NTGUI`, `HAVE_NS`, `HAVE_PGTK`, `HAVE_HAIKU`, `HAVE_ANDROID`, `USE_MOTIF`,
// `USE_X_TOOLKIT`, `MSDOS`, `GLYPH_DEBUG` and `DEBUG_X_COLORS` are all undefined.  C compiles
// 5,476 of the file's 7,843 lines; the 2,350 dead ones are named in `docs/ports/xfaces.md`'s
// Decisions instead of being carried as `#[cfg]` text, and `line-drift-tolerance.tsv`'s row
// for this file therefore asks for the *live* baseline (the `frame` row's precedent).
//
// The two live exceptions inside that dead region are the files' own comments, which C keeps
// as ordinary text: the `#else` arms of `GLYPH_DEBUG` (`check_lface_attrs`/`check_lface` spell
// `((void) 0)`), and `load_color`'s `#ifndef MSDOS` (live, so `load_color` is the plain
// function).
//
// **Use-site declarations.**  `dispextern.h` has a mirror (`include/dispextern.rs`) that stops
// before its face block, so the records `struct face` and `struct face_cache` (whose `font`
// field is `include/font.rs`'s `struct font`, mirrored in this round), and the
// `FACE_SUITABLE_FOR_ASCII_CHAR_P`/`FACE_FOR_CHAR`/
// `FRAME_FACE_CACHE`/`FACE_FROM_ID`/`FRAME_FOREGROUND_PIXEL` family,
// and `load_color`/`free_frame_faces`/`lookup_derived_face`… are declared here with clang-measured
// offsets and `const _` assertions, the shape `term.rs`/`font.rs` established (PORTING.md
// P-66/P-72).  The mirrors themselves are not touched.
//
// **Definitions that a mirror declares.**  The `DEFVAR_*` carriers this file owns are defined in
// `xfaces_globals`/`xfaces_defs` modules that are glob-imported (an item shadows a glob import
// while a second crate-root item of the same name would be `E0428`; PORTING.md P-21, status
// 3.29), and so are the `Lisp_Object` globals `globals.rs` declares.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_assignments,
         unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn, unused_parens,
         clashing_extern_declarations)]

include!("include/config.rs");            // C 219: config.h
include!("../lib/include/stdlib.rs");     // C 220: strtod, free
include!("../lib/include/stdio.rs");      // C 221: sysstdio.h -> <stdio.h> (FILE, fgets)
include!("../lib/include/sys_types.in.rs");   // C 222: sys/types.h
include!("../lib/include/sys_stat.in.rs");    // C 223: sys/stat.h
include!("../lib/include/math.rs");       // C 224: <math.h> (lrint)
include!("../lib/include/c-ctype.rs");    // C 292: c-ctype.h (c_tolower)
include!("../lib/include/string.rs");     // lisp.h 30: strlen, strcmp, strncmp, strchr
include!("../lib/include/stdcountof.rs"); // lisp.h 30 (stdcountof.h: `countof`)
include!("../lib/include/limits.in.rs");  // lisp.h's <limits.h> (INT_MAX in the dead pixmap code)
include!("include/lisp.rs");              // C 226: lisp.h
include!("include/character.rs");         // C 227: character.h
include!("include/frame.rs");             // C 228: frame.h
include!("include/buffer.rs");            // C 263: buffer.h
include!("include/charset.rs");           // buffer.h 28-late: charset.h
include!("include/coding.rs");            // buffer.h 26: coding.h
include!("include/composite.rs");         // coding.h 284: composite.h
include!("include/keyboard.rs");          // frame.h/termhooks.h: keyboard.h
include!("include/systime.rs");           // termhooks.h: systime.h (`Time`)
include!("include/itree.rs");             // buffer.h 27: itree.h
include!("include/dispextern.rs");        // C 264: dispextern.h
include!("include/termhooks.rs");         // termhooks.h: struct terminal, FRAME_TERMINAL
include!("include/blockinput.rs");        // C 265: blockinput.h
include!("include/window.rs");            // C 266: window.h
include!("include/termchar.rs");          // C 267: termchar.h
include!("include/font.rs");              // C 269: font.h
include!("include/fontset.rs");            // C 270: fontset.h (`fontset_name`)
#[cfg (feature = "have_window_system")]
include!("include/androidterm.rs");         // frame.h's Android `FRAME_FONT`/`FRAME_FONTSET`
include!("include/keymap.rs");            // keyboard.h/window.h: keymap.h
include!("include/disptab.rs");           // frame.h: disptab.h
include!("include/commands.rs");          // lisp.h: commands.h (`noninteractive`)
include!("include/syssignal.rs");         // blockinput.h/window.h: syssignal.h
include!("include/pdumper.rs");           // lisp.h/frame.h: pdumper.h (`dumped_with_pdumper_p`)

/* C 20-217: the file's explanatory comment (`Faces.`).  */

/* Faces.

   When using Emacs with X, the display style of characters can be
   changed by defining `faces'.  Each face can specify the following
   display attributes:

   1. Font family name.

   2. Font foundry name.

   3. Relative proportionate width, aka character set width or set
   width (swidth), e.g. `semi-compressed'.

   4. Font height in 1/10pt.

   5. Font weight, e.g. `bold'.

   6. Font slant, e.g. `italic'.

   7. Foreground color.

   8. Background color.

   9. Whether or not characters should be underlined, and in what color.

   10. Whether or not characters should be displayed in inverse video.

   11. A background stipple, a bitmap.

   12. Whether or not characters should be overlined, and in what color.

   13. Whether or not characters should be strike-through, and in what
   color.

   14. Whether or not a box should be drawn around characters, the box
   type, and, for simple boxes, in what color.

   15. Font-spec, or nil.  This is a special attribute.

   A font-spec is a collection of font attributes (specs).

   When this attribute is specified, the face uses a font matching
   with the specs as is except for what overwritten by the specs in
   the fontset (see below).  In addition, the other font-related
   attributes (1st thru 5th) are updated from the spec.

   On the other hand, if one of the other font-related attributes are
   specified, the corresponding specs in this attribute is set to nil.

   16. A face name or list of face names from which to inherit attributes.

   17. A fontset name.  This is another special attribute.

   A fontset is a mappings from characters to font-specs, and the
   specs overwrite the font-spec in the 14th attribute.

   18. A "distant background" color, to be used when the foreground is
   too close to the background and is hard to read.

   19. Whether to extend the face to end of line when the face
   "covers" the newline that ends the line.

   On the C level, a Lisp face is completely represented by its array
   of attributes.  In that array, the zeroth element is Qface, and the
   rest are the 19 face attributes described above.  The
   lface_attribute_index enumeration, defined on dispextern.h, with
   values given by the LFACE_*_INDEX constants, is used to reference
   the individual attributes.

   Faces are frame-local by nature because Emacs allows you to define the
   same named face (face names are symbols) differently for different
   frames.  Each frame has an alist of face definitions for all named
   faces.  The value of a named face in such an alist is a Lisp vector
   with the symbol `face' in slot 0, and a slot for each of the face
   attributes mentioned above.

   There is also a global face map `Vface_new_frame_defaults',
   containing conses of (FACE_ID . FACE_DEFINITION).  Face definitions
   from this table are used to initialize faces of newly created
   frames.

   A face doesn't have to specify all attributes.  Those not specified
   have a value of `unspecified'.  Faces specifying all attributes but
   the 14th are called `fully-specified'.


   Face merging.

   The display style of a given character in the text is determined by
   combining several faces.  This process is called `face merging'.
   Face merging combines the attributes of each of the faces being
   merged such that the attributes of the face that is merged later
   override those of a face merged earlier in the process.  In
   particular, this replaces any 'unspecified' attributes with
   non-'unspecified' values.  Also, if a face inherits from another
   (via the :inherit attribute), the attributes of the parent face,
   recursively, are applied where the inheriting face doesn't specify
   non-'unspecified' values.  Any aspect of the display style that
   isn't specified by overlays or text properties is taken from the
   'default' face.  Since it is made sure that the default face is
   always fully-specified, face merging always results in a
   fully-specified face.


   Face realization.

   After all face attributes for a character have been determined by
   merging faces of that character, that face is `realized'.  The
   realization process maps face attributes to what is physically
   available on the system where Emacs runs.  The result is a
   `realized face' in the form of a struct face which is stored in the
   face cache of the frame on which it was realized.

   Face realization is done in the context of the character to display
   because different fonts may be used for different characters.  In
   other words, for characters that have different font
   specifications, different realized faces are needed to display
   them.

   Font specification is done by fontsets.  See the comment in
   fontset.c for the details.  In the current implementation, all ASCII
   characters share the same font in a fontset.

   Faces are at first realized for ASCII characters, and, at that
   time, assigned a specific realized fontset.  Hereafter, we call
   such a face as `ASCII face'.  When a face for a multibyte character
   is realized, it inherits (thus shares) a fontset of an ASCII face
   that has the same attributes other than font-related ones.

   Thus, all realized faces have a realized fontset.


   Unibyte text.

   Unibyte text (i.e. raw 8-bit characters) is displayed with the same
   font as ASCII characters.  That is because it is expected that
   unibyte text users specify a font that is suitable both for ASCII
   and raw 8-bit characters.


   Font selection.

   Font selection tries to find the best available matching font for a
   given (character, face) combination.

   If the face specifies a fontset name, that fontset determines a
   pattern for fonts of the given character.  If the face specifies a
   font name or the other font-related attributes, a fontset is
   realized from the default fontset.  In that case, that
   specification determines a pattern for ASCII characters and the
   default fontset determines a pattern for multibyte characters.

   Available fonts on the system on which Emacs runs are then matched
   against the font pattern.  The result of font selection is the best
   match for the given face attributes in this font list.

   Font selection can be influenced by the user.

   1. The user can specify the relative importance he gives the face
   attributes width, height, weight, and slant by setting
   face-font-selection-order (faces.el) to a list of face attribute
   names.  The default is '(:width :height :weight :slant), and means
   that font selection first tries to find a good match for the font
   width specified by a face, then---within fonts with that
   width---tries to find a best match for the specified font height,
   etc.

   2. Setting face-font-family-alternatives allows the user to
   specify alternative font families to try if a family specified by a
   face doesn't exist.

   3. Setting face-font-registry-alternatives allows the user to
   specify all alternative font registries to try for a face
   specifying a registry.

   4. Setting face-ignored-fonts allows the user to ignore specific
   fonts.


   Character composition.

   Usually, the realization process is already finished when Emacs
   actually reflects the desired glyph matrix on the screen.  However,
   on displaying a composition (sequence of characters to be composed
   on the screen), a suitable font for the components of the
   composition is selected and realized while drawing them on the
   screen, i.e.  the realization process is delayed but in principle
   the same.


   Initialization of basic faces.

   The faces `default', `modeline' are considered `basic faces'.
   When redisplay happens the first time for a newly created frame,
   basic faces are realized for CHARSET_ASCII.  Frame parameters are
   used to fill in unspecified attributes of the default face.  */

// --- use-site declarations ---------------------------------------------------------------
// The names below are declared by dispextern.h/frame.h/lisp.h/termhooks.h at points this
// unit's mirrors do not reach (PORTING.md P-66/P-72), so each is declared here where C's
// translation unit gets it, or reached through a clang-measured local record.

/// C: dispextern.h 1787-1937: `struct face`.  Measured with the frozen clang: `sizeof` 280,
/// align 8, `lface`@0, `id`@160, `foreground`@168, `background`@176, `underline_color`@184,
/// `overline_color`@192, `strike_through_color`@200, `box_color`@208, `font`@216,
/// `fontset`@224, `box_vertical_line_width`@228, `box_horizontal_line_width`@232,
/// `underline_pixels_above_descent_line`@236, the 21 `bool_bf`/`ENUM_BF` bits C declares at
/// 1893-1935 in one 4-byte storage unit at 240, `hash`@248, `next`@256, `prev`@264,
/// `ascii_face`@272.  `HAVE_WINDOW_SYSTEM` (`gc`, `stipple`), `HAVE_NS`, `HAVE_XFT`/
/// `HAVE_FREETYPE` (`extra`) are undef, so those members are absent from this layout.
#[repr(C)]
pub struct face
{
  pub lface: [Lisp_Object; LFACE_VECTOR_SIZE as usize],
  pub id: c_int,
  /* C 1797-1811: the two members `#ifdef HAVE_WINDOW_SYSTEM` adds between `id` and
     `foreground` - `gc` at 168 and `stipple` at 176 in both window arms, which is why
     `foreground` moves from 168 (frozen) to 184 there (`docs/baseline/face-layout.tsv`).  This
     unit's copy lacked them, so no arm that draws could be written (status 4.756).  */
  #[cfg (all (feature = "have_window_system", feature = "have_android"))]
  pub gc: *mut c_void,        /// C 1801: `struct android_gc *gc;`
  #[cfg (all (feature = "have_window_system",
              not (feature = "have_x_windows"), not (feature = "have_android")))]
  pub gc: *mut Emacs_GC,      // C 1803: `Emacs_GC *gc;`
  #[cfg (feature = "have_window_system")]
  pub stipple: ptrdiff_t,     // C 1810, measured 176
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
  /* C 1893-1935: `ENUM_BF (face_box_type) box : 2; ENUM_BF (face_underline_type)
     underline : 3;` and the seventeen 1-bit `bool_bf` members in one storage unit.  The
     port stores C's bit-field bytes and names every member through the masks below
     (PORTING.md 1.3/P-80).  */
  pub bits: u32,
  pub hash: uintptr_t,
  pub next: *mut face,
  pub prev: *mut face,
  pub ascii_face: *mut face,
}

#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::size_of::<face> () == 280);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::size_of::<face> () == 296);
const _: () = core::assert! (std::mem::align_of::<face> () == 8);
const _: () = core::assert! (std::mem::offset_of! (face, id) == 160);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, gc) == 168);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, stipple) == 176);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, foreground) == 168);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, foreground) == 184);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, background) == 176);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, background) == 192);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, underline_color) == 184);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, underline_color) == 200);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, overline_color) == 192);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, overline_color) == 208);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, strike_through_color) == 200);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, strike_through_color) == 216);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, box_color) == 208);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, box_color) == 224);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, font) == 216);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, font) == 232);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, fontset) == 224);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, fontset) == 240);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, bits) == 240);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, bits) == 256);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, hash) == 248);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, hash) == 264);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, next) == 256);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, next) == 272);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, ascii_face) == 272);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, ascii_face) == 288);

/* `struct font` (font.h 275-418) comes from `include/font.rs` now; this unit reads `props`,
   `pixel_size` and `average_width` of it (docs/ports/include-font.md).  */

/// C: dispextern.h 1991-2009: `struct face_cache`.  Measured: `sizeof` 40, `buckets`@0,
/// `f`@8, `faces_by_id`@16, `size`@24, `used`@32, and C's one `bool_bf
/// menu_face_changed_p : 1` in the int unit at 36 (carried as a byte, the P-80 convention).
#[repr(C)]
pub struct face_cache
{
  pub buckets: *mut *mut face,
  pub f: *mut frame,
  pub faces_by_id: *mut *mut face,
  pub size: ptrdiff_t,
  pub used: c_int,
  pub menu_face_changed_p: u8,
}

const _: () = core::assert! (std::mem::size_of::<face_cache> () == 40);
const _: () = core::assert! (std::mem::offset_of! (face_cache, faces_by_id) == 16);
const _: () = core::assert! (std::mem::offset_of! (face_cache, used) == 32);

/* The `face_box_type`/`face_underline_type` members and the `bool_bf` bits of `struct face`,
   in C's declaration order (dispextern.h 1893-1935).  Every reader and writer goes through
   these masked accessors, which is the P-80 convention for a `bool_bf` run; the member name
   stays the macro's first argument, so the ported text keeps C's token.  */
pub const FACE_BOX_SHIFT: u32 = 0;
pub const FACE_BOX_MASK: u32 = 0x3;
pub const FACE_UNDERLINE_SHIFT: u32 = 2;
pub const FACE_UNDERLINE_MASK: u32 = 0x7;
pub const FACE_USE_BOX_COLOR_FOR_SHADOWS_P: u32 = 1 << 5;
pub const FACE_OVERLINE_P: u32 = 1 << 6;
pub const FACE_STRIKE_THROUGH_P: u32 = 1 << 7;
pub const FACE_FOREGROUND_DEFAULTED_P: u32 = 1 << 8;
pub const FACE_BACKGROUND_DEFAULTED_P: u32 = 1 << 9;
pub const FACE_UNDERLINE_DEFAULTED_P: u32 = 1 << 10;
pub const FACE_OVERLINE_COLOR_DEFAULTED_P: u32 = 1 << 11;
pub const FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P: u32 = 1 << 12;
pub const FACE_BOX_COLOR_DEFAULTED_P: u32 = 1 << 13;
pub const FACE_UNDERLINE_AT_DESCENT_LINE_P: u32 = 1 << 14;
pub const FACE_TTY_BOLD_P: u32 = 1 << 15;
pub const FACE_TTY_ITALIC_P: u32 = 1 << 16;
pub const FACE_TTY_REVERSE_P: u32 = 1 << 17;
pub const FACE_TTY_STRIKE_THROUGH_P: u32 = 1 << 18;
pub const FACE_TTY_OVERLINE_P: u32 = 1 << 19;
pub const FACE_COLORS_COPIED_BITWISE_P: u32 = 1 << 20;
pub const FACE_OVERSTRIKE: u32 = 1 << 21;

macro_rules! face_bits {
  (box, $f:expr) => { unsafe { ((*$f).bits & FACE_BOX_MASK) >> FACE_BOX_SHIFT } };
  (box, $f:expr, $v:expr) => {{
    unsafe { (*$f).bits = ((*$f).bits & ! FACE_BOX_MASK) | ((($v) as u32) << FACE_BOX_SHIFT); } }};
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
  (foreground_defaulted_p, $f:expr) => { face_bit_p ($f, FACE_FOREGROUND_DEFAULTED_P) };
  (foreground_defaulted_p, $f:expr, $v:expr) =>
    { face_bit_set ($f, FACE_FOREGROUND_DEFAULTED_P, $v) };
  (background_defaulted_p, $f:expr) => { face_bit_p ($f, FACE_BACKGROUND_DEFAULTED_P) };
  (background_defaulted_p, $f:expr, $v:expr) =>
    { face_bit_set ($f, FACE_BACKGROUND_DEFAULTED_P, $v) };
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
  (box_color_defaulted_p, $f:expr) => { face_bit_p ($f, FACE_BOX_COLOR_DEFAULTED_P) };
  (box_color_defaulted_p, $f:expr, $v:expr) => { face_bit_set ($f, FACE_BOX_COLOR_DEFAULTED_P, $v) };
  (underline_at_descent_line_p, $f:expr) => { face_bit_p ($f, FACE_UNDERLINE_AT_DESCENT_LINE_P) };
  (underline_at_descent_line_p, $f:expr, $v:expr) =>
    { face_bit_set ($f, FACE_UNDERLINE_AT_DESCENT_LINE_P, $v) };
  (tty_bold_p, $f:expr) => { face_bit_p ($f, FACE_TTY_BOLD_P) };
  (tty_bold_p, $f:expr, $v:expr) => { face_bit_set ($f, FACE_TTY_BOLD_P, $v) };
  (tty_italic_p, $f:expr) => { face_bit_p ($f, FACE_TTY_ITALIC_P) };
  (tty_italic_p, $f:expr, $v:expr) => { face_bit_set ($f, FACE_TTY_ITALIC_P, $v) };
  (tty_reverse_p, $f:expr) => { face_bit_p ($f, FACE_TTY_REVERSE_P) };
  (tty_reverse_p, $f:expr, $v:expr) => { face_bit_set ($f, FACE_TTY_REVERSE_P, $v) };
  (tty_strike_through_p, $f:expr) => { face_bit_p ($f, FACE_TTY_STRIKE_THROUGH_P) };
  (tty_strike_through_p, $f:expr, $v:expr) => { face_bit_set ($f, FACE_TTY_STRIKE_THROUGH_P, $v) };
  (tty_overline_p, $f:expr) => { face_bit_p ($f, FACE_TTY_OVERLINE_P) };
  (tty_overline_p, $f:expr, $v:expr) => { face_bit_set ($f, FACE_TTY_OVERLINE_P, $v) };
  (colors_copied_bitwise_p, $f:expr) => { face_bit_p ($f, FACE_COLORS_COPIED_BITWISE_P) };
  (colors_copied_bitwise_p, $f:expr, $v:expr) =>
    { face_bit_set ($f, FACE_COLORS_COPIED_BITWISE_P, $v) };
  (overstrike, $f:expr) => { face_bit_p ($f, FACE_OVERSTRIKE) };
  (overstrike, $f:expr, $v:expr) => { face_bit_set ($f, FACE_OVERSTRIKE, $v) };
}

#[inline(always)]
pub unsafe fn face_bit_p (f: *mut face, mask: u32) -> bool { ((*f).bits & mask) != 0 }

#[inline(always)]
pub unsafe fn face_bit_set (f: *mut face, mask: u32, v: bool)
{ if v { (*f).bits |= mask; } else { (*f).bits &= ! mask; } }

/// C: frame.h 1379: `#define FRAME_FACE_CACHE(F) (F)->face_cache`.  The `frame` mirror
/// carries the field as `*mut c_void` ("C: `struct face_cache *face_cache`") because
/// `struct face_cache` has no mirror, so the cast is the use-site conversion P-72 rule 3
/// prescribes.  Reads keep C's call shape; the three *writes* (`init_frame_faces`,
/// `free_frame_faces` and the `Qmenu` arm of `internal-set-lisp-face-attribute` are this
/// file's only ones) go through the macro of the same name, because a cast is not an
/// assignment target in Rust:
macro_rules! FRAME_FACE_CACHE {
  ($f:expr, $v:expr) => { (*$f).face_cache = $v as *mut c_void };
}

#[inline(always)]
pub fn FRAME_FACE_CACHE (f: *mut frame) -> *mut face_cache
{ unsafe { (*f).face_cache as *mut face_cache } }

/* frame.h 517: `bool_bf face_change : 1` is packed into the mirror's `bits2` (P-80); this
   unit writes it and no mirror carries a setter, so the member-named accessor lives here
   (the term.rs `frame_bits_set!` shape).  */
macro_rules! frame_bits {
  (face_change, $f:expr, $v:expr) => {{
    if $v { (*$f).bits2 |= 1u32 << FRAME_FACE_CHANGE_SHIFT; }
    else { (*$f).bits2 &= ! (1u32 << FRAME_FACE_CHANGE_SHIFT); } }};
}

/// C: dispextern.h 1998-2010: `FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL`.
#[inline(always)]
pub fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
{
  eassert! (0 <= id && id < unsafe { (*FRAME_FACE_CACHE (f)).used });
  unsafe { *((*FRAME_FACE_CACHE (f)).faces_by_id).offset (id as isize) }
}

#[inline(always)]
pub fn FACE_FROM_ID_OR_NULL (f: *mut frame, id: c_int) -> *mut face
{
  let used: c_int = unsafe { (*FRAME_FACE_CACHE (f)).used };
  eassume! (0 <= used);
  if 0 <= id && id < used
    { unsafe { *((*FRAME_FACE_CACHE (f)).faces_by_id).offset (id as isize) } }
  else
    { std::ptr::null_mut () }
}

/// C: dispextern.h 2015-2022: `FACE_SUITABLE_FOR_ASCII_CHAR_P`.  `HAVE_WINDOW_SYSTEM` is
/// undef, so C's body is `return true;`.
#[inline(always)]
pub fn FACE_SUITABLE_FOR_ASCII_CHAR_P (face: *mut face) -> bool { let _ = face; true }

/// C: dispextern.h 2030-2038: `FACE_FOR_CHAR`.  `HAVE_WINDOW_SYSTEM` is undef, so C's body
/// is `return face->id;`.
#[inline(always)]
pub fn FACE_FOR_CHAR (f: *mut frame, face: *mut face, character: c_int, pos: ptrdiff_t,
                      object: Lisp_Object) -> face_id
{
  let _ = (f, character, pos, object);
  unsafe { (*face).id }
}

/* C 1932-1949 (dispextern.h): the three tty "default colour" sentinels.  */
pub const FACE_TTY_DEFAULT_COLOR: c_ulong = -1i64 as c_ulong;
pub const FACE_TTY_DEFAULT_FG_COLOR: c_ulong = -2i64 as c_ulong;
pub const FACE_TTY_DEFAULT_BG_COLOR: c_ulong = -3i64 as c_ulong;

/* C 1932-1949 (dispextern.h): the three tty "default colour" sentinels are above; the
   declarations below are the headers' own prototypes the mirrors stop short of.  */
unsafe extern "C" {
  /// C: frame.h 1476: `struct frame *decode_live_frame (Lisp_Object);` (frame.c).
  fn decode_live_frame (frame: Lisp_Object) -> *mut frame;
  /// C: lisp.h 2xxx: `bool FUNCTIONP (Lisp_Object)` is an INLINE in lisp.h that no mirror
  /// carries; the port declares the same predicate here.
  fn FUNCTIONP (a: Lisp_Object) -> bool;
  fn lrint (x: c_double) -> c_long;
  fn decode_any_frame (frame: Lisp_Object) -> *mut frame;
  /// C: dispextern.h 3661-3662: `extern void add_to_log (const char *, ...);`
  fn add_to_log (format: *const c_char, ...);
}

/* font.h 205-230: the six property accessors this unit expands are the mirror's now
   (`include/font.rs` 203-213: three `*_FOR_FACE` and three `*_NAME_NUMERIC`, C's own bodies);
   the unit's copies were redundancy, the first three body-identical and the second three
   differing only by the mirror's outer parentheses (P-26).
   frame.h 1387-1391: `CHECK_LIVE_FRAME` is `include/frame.rs` 983.  */

/* sysstdio.h 37-40: `FOPEN_TEXT` is `""` on this target (`O_BINARY` is 0).  */
macro_rules! FOPEN_TEXT { () => { "" }; }

/// C: font.h 265-286: the three style helpers, defined in font.c.
unsafe extern "C" {
  fn font_style_symbolic (font: Lisp_Object, prop: font_property_index,
			  for_face: bool) -> Lisp_Object;
  fn font_style_to_value (prop: font_property_index, val: Lisp_Object,
			  noerror: bool) -> c_int;
  /// C: font.h 288-291: `void font_clear_prop (Lisp_Object attrs[], enum font_property_index prop);`
  fn font_clear_prop (attrs: *mut Lisp_Object, prop: font_property_index);
  /// C: font.h 839: `extern Lisp_Object copy_font_spec (Lisp_Object);`
  fn copy_font_spec (font: Lisp_Object) -> Lisp_Object;
  /// C: font.h 840: `extern Lisp_Object merge_font_spec (Lisp_Object, Lisp_Object);`
  fn merge_font_spec (from: Lisp_Object, to: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 5322-5323: `extern FILE *emacs_fopen (char const *, char const *);`
  fn emacs_fopen (file: *const c_char, mode: *const c_char) -> *mut FILE;
  /// C: <stdio.h>: the two stdio entries `Fx_load_color_file` reads.
  fn fgets (s: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
  fn sscanf (s: *const c_char, format: *const c_char, ...) -> c_int;
  /// C: dispextern.h 3629: `forget_escape_and_glyphless_faces`, one of the two display
  /// entries `free_realized_faces` calls (the other is the next line's `clear_current_matrices`).
  fn forget_escape_and_glyphless_faces ();
  /// C: dispnew.h 88: `extern void clear_current_matrices (struct frame *f);`
  fn clear_current_matrices (f: *mut frame);
}

/* C 294-309: the attribute predicates and the cache-size constant.  */
macro_rules! UNSPECIFIEDP { ($attr:expr) => { EQ ($attr, Qunspecified ()) }; }
macro_rules! IGNORE_DEFFACE_P { ($attr:expr) => { EQ ($attr, QCignore_defface ()) }; }
macro_rules! RESET_P { ($attr:expr) => { EQ ($attr, Qreset ()) }; }
pub const FACE_CACHE_BUCKETS_SIZE: c_int = 1009;

/* C 311: the two default-color names.  */
#[unsafe(no_mangle)]
pub static mut unspecified_fg: [c_char; 15] = [
  b'u' as c_char, b'n' as c_char, b's' as c_char, b'p' as c_char, b'e' as c_char,
  b'c' as c_char, b'i' as c_char, b'f' as c_char, b'i' as c_char, b'e' as c_char,
  b'd' as c_char, b'-' as c_char, b'f' as c_char, b'g' as c_char, 0];
#[unsafe(no_mangle)]
pub static mut unspecified_bg: [c_char; 15] = [
  b'u' as c_char, b'n' as c_char, b's' as c_char, b'p' as c_char, b'e' as c_char,
  b'c' as c_char, b'i' as c_char, b'f' as c_char, b'i' as c_char, b'e' as c_char,
  b'd' as c_char, b'-' as c_char, b'b' as c_char, b'g' as c_char, 0];

/* C 317, 323: the two alternative-font alists this unit owns.  `globals.rs` declares both
   (`use variables::*`), and a crate-root definition shadows a glob import while a second
   glob of the same name would be ambiguous (PORTING.md P-21, the casetab.rs shape).  */
#[unsafe(no_mangle)]
pub static mut Vface_alternative_font_family_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vface_alternative_font_registry_alist: Lisp_Object = lisp_h_Qnil ();

/* C 327-332.  */
static mut next_lface_id: c_int = 0;
static mut lface_id_to_name: *mut Lisp_Object = std::ptr::null_mut ();
static mut lface_id_to_name_size: ptrdiff_t = 0;

/* C 349: exported.  */
#[unsafe(no_mangle)]
pub static mut face_change: bool = false;

/* C 356, 369.  */
static mut tty_suppress_bold_inverse_default_colors_p: bool = false;
static mut menu_face_changed_default: bool = false;

/* C 371-389: `struct named_merge_point;` and the ten function prototypes are C forward
   declarations; Rust items are order-independent, so they are deleted and the definitions
   stay at C's positions (PORTING.md 0.1-12, recorded in docs/ports/xfaces.md).  */

/***********************************************************************
			      Utilities
 ***********************************************************************/

/* C 638-666: `share_image_cache` - "Find an existing image cache registered for a frame on F's
   display and with a `scaling_col_width' of F's FRAME_COLUMN_WIDTH, or, in the absence of an
   eligible image cache, allocate an image cache with the same width value."  It sits inside
   C 637's `#ifdef HAVE_WINDOW_SYSTEM`: dead on the frozen target (the macro is undefined there),
   live on Android - and this unit is where C defines the C symbol, which the Android build
   imported with only a declaration in `frame.rs` (status 4.752).  */
/// C: image.c 2223-2225: `struct image_cache * make_image_cache (void);` - C's xfaces.c has the
/// prototype through its headers; this unit reads it from there.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  fn make_image_cache () -> *mut image_cache;
}

#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn share_image_cache (f: *mut frame) -> *mut image_cache
{
  let width: c_int = if 10 > FRAME_COLUMN_WIDTH (f) { 10 } else { FRAME_COLUMN_WIDTH (f) };
  let mut tail: Lisp_Object = Qnil ();                 // C 645: `Lisp_Object tail, frame;`
  let mut frame: Lisp_Object = Qnil ();
  let mut cache: *mut image_cache;                     // C 646

  FOR_EACH_FRAME! (tail, frame, {
      let x: *mut frame = XFRAME (frame);

      /* `FRAME_IMAGE_CACHE (x)` is C's `(x)->image_cache`, a `struct image_cache *`; the mirror
	 holds the member as `*mut c_void` (frame.h 289-292's own window-system arm).  */
      let xcache: *mut image_cache = FRAME_IMAGE_CACHE! (x) as *mut image_cache;
      /* C's `FRAME_TERMINAL` is `(F)->terminal` (termhooks.h 932); this unit does not include
	 the termhooks mirror (C gets the header through dispextern.h), so the field is spelled
	 out - the same reading the macro has.  */
      if (*x).terminal == (*f).terminal
	  && !xcache.is_null ()
	  && (*xcache).scaling_col_width == width
	{ return xcache; }
  });

  cache = make_image_cache ();
  (*cache).scaling_col_width = width;
  return cache;
}

/* C 670-689: initialize face cache and basic faces for frame F.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_frame_faces (f: *mut frame)
{
  /* Make a face cache, if F doesn't have one.  */
  if FRAME_FACE_CACHE (f) == std::ptr::null_mut ()
    { FRAME_FACE_CACHE! (f, make_face_cache (f)); }

  /* Make or share an image cache.  (C's `HAVE_WINDOW_SYSTEM` arm is dead.)  */

  /* Realize faces early (Bug#17889).  */
  if ! realize_basic_faces (f)
    { emacs_abort (); }
}

/* C 695-722: free face cache of frame F.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_frame_faces (f: *mut frame)
{
  let face_cache: *mut face_cache = FRAME_FACE_CACHE (f);
  if ! face_cache.is_null ()
    {
      free_face_cache (face_cache);
      FRAME_FACE_CACHE! (f, std::ptr::null_mut ());
    }

  /* C's `HAVE_WINDOW_SYSTEM` arm is dead.  */
}

/* C 730-748: clear face caches, and recompute basic faces for frame F.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn recompute_basic_faces (f: *mut frame)
{
  if ! FRAME_FACE_CACHE (f).is_null ()
    {
      let non_basic_faces_cached: bool =
	(*FRAME_FACE_CACHE (f)).used > BASIC_FACE_ID_SENTINEL;
      clear_face_cache (false);
      if ! realize_basic_faces (f)
	{ emacs_abort (); }
      /* The call to realize_basic_faces above recomputed the basic
         faces and freed their fontsets, but if there are non-ASCII
         faces in the cache, they might now be invalid, and they
         reference fontsets that are no longer in Vfontset_table.  We
         therefore must force complete regeneration of all frame faces.  */
      if non_basic_faces_cached
	{ frame_bits! (face_change, f, true); }
    }
}

/* C 754-792: clear the face caches of all frames.  CLEAR_FONTS_P means
   try to free unused fonts, too.  (The body is C's live text: the whole `HAVE_WINDOW_SYSTEM`
   block is dead, and C's frozen function is empty too.)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_face_cache (clear_fonts_p: bool)
{
  let _ = clear_fonts_p;
}

DEFUN! ("clear-face-cache", Fclear_face_cache, Sclear_face_cache, 0, 1, 0,
       doc: "Clear face caches on all frames.\nOptional THOROUGHLY non-nil means try to free unused fonts, too.",
       (thoroughly: Lisp_Object),
       {
  clear_face_cache (! NILP (thoroughly));
  face_change = true;
  windows_or_buffers_changed = 53;
  return Qnil ();
});


/***********************************************************************
			      X Pixmaps
 ***********************************************************************/

/* C 810-855: `bitmap-spec-p'.  The port had dropped it with the rest of the window block,
   but `load_pixmap` calls it and globals.rs already declares it as an import.  */
#[cfg (feature = "have_window_system")]
DEFUN! ("bitmap-spec-p", Fbitmap_spec_p, Sbitmap_spec_p, 1, 1, 0,
       doc: "Value is non-nil if OBJECT is a valid bitmap specification.\nA bitmap specification is either a string, a file name, or a list\n\\(WIDTH HEIGHT DATA) where WIDTH is the pixel width of the bitmap,\nHEIGHT is its height, and DATA is a string containing the bits of\nthe pixmap.  Bits are stored row by row, each row occupies\n\\(WIDTH + 7)/8 bytes.",
       (object: Lisp_Object),
       {
  let mut object: Lisp_Object = object;
  let mut pixmap_p: bool = false;

  if STRINGP (object)
    { pixmap_p = true; }
  else if CONSP (object)
    {
      let mut width: Lisp_Object;
      let mut height: Lisp_Object;
      let mut data: Lisp_Object;

      height = Qnil (); width = Qnil (); data = Qnil ();

      if CONSP (object)
	{
	  width = XCAR (object);
	  object = XCDR (object);
	  if CONSP (object)
	    {
	      height = XCAR (object);
	      object = XCDR (object);
	      if CONSP (object)
		{ data = XCAR (object); }
	    }
	}

      if STRINGP (data)
	  && RANGED_FIXNUMP (1, width, INT_MAX as EMACS_INT)
	  && RANGED_FIXNUMP (1, height, INT_MAX as EMACS_INT)
	{
	  let bytes_per_row: c_int
	    = (XFIXNUM (width) + CHAR_BIT as EMACS_INT - 1) as c_int / CHAR_BIT as c_int;
	  if XFIXNUM (height) <= SBYTES (data) / bytes_per_row as EMACS_INT
	    { pixmap_p = true; }
	}
    }

  return if pixmap_p { Qt () } else { Qnil () };
});

/* C 857-915: load a bitmap for frame F (a file name or a bitmap spec); 0 on failure.  */
#[cfg (feature = "have_window_system")]
unsafe fn load_pixmap (f: *mut frame, name: Lisp_Object) -> ptrdiff_t
{
  let bitmap_id: ptrdiff_t;

  if NILP (name)
    { return 0; }

  CHECK_TYPE (! NILP (Fbitmap_spec_p (name)), Qbitmap_spec_p (), name);

  block_input ();
  if CONSP (name)
    {
      /* Decode a bitmap spec into a bitmap.  */
      let w: c_int = XFIXNUM (Fcar (name)) as c_int;
      let h: c_int = XFIXNUM (Fcar (Fcdr (name))) as c_int;
      let bits: Lisp_Object = Fcar (Fcdr (Fcdr (name)));

      bitmap_id = image_create_bitmap_from_data (f, SSDATA (bits) as *mut c_char, w, h);
    }
  else
    {
      /* It must be a string -- a file name.  */
      bitmap_id = image_create_bitmap_from_file (f, name);
    }
  unblock_input ();

  if bitmap_id < 0
    {
      add_to_log (c"Invalid or undefined bitmap `%s'".as_ptr (), name);
      return 0;
    }
  return bitmap_id;
}

/* image.c's three bitmap entries; C declares them in dispextern.h 3729-3739, which this
   unit's mirror does not carry yet, so they are declared at the use site (PORTING.md P-55).  */
unsafe extern "C" {
  /// C: font.h 852: `extern Lisp_Object font_load_for_lface (struct frame *, Lisp_Object *,
  /// Lisp_Object);` - font.h's mirror does not carry it (the arm that calls it was dead).
  pub fn font_load_for_lface (f: *mut frame, attrs: *mut Lisp_Object,
			      spec: Lisp_Object) -> Lisp_Object;
  pub fn image_create_bitmap_from_data (f: *mut frame, data: *mut c_char,
					w: c_int, h: c_int) -> ptrdiff_t;
  pub fn image_create_bitmap_from_file (f: *mut frame, name: Lisp_Object) -> ptrdiff_t;
  pub fn image_destroy_bitmap (f: *mut frame, id: ptrdiff_t);
}


/***********************************************************************
                            Color Handling
 ***********************************************************************/

/* C 927-950: parse hex color component specification that starts at S and ends
   right before E.  Set *DST to the parsed value normalized so that the maximum value for
   the number of hex digits given becomes 65535, and return true on success, false
   otherwise.  */
unsafe fn parse_hex_color_comp (mut s: *const c_char, e: *const c_char,
                                dst: *mut c_ushort) -> bool
{
  let n: c_int = e.offset_from (s) as c_int;
  if n <= 0 || n > 4
    { return false; }
  let mut val: c_int = 0;
  while s < e
    {
      let digit: c_int;
      if *s >= '0' as c_char && *s <= '9' as c_char
        { digit = (*s - '0' as c_char) as c_int; }
      else if *s >= 'A' as c_char && *s <= 'F' as c_char
        { digit = (*s - 'A' as c_char) as c_int + 10; }
      else if *s >= 'a' as c_char && *s <= 'f' as c_char
        { digit = (*s - 'a' as c_char) as c_int + 10; }
      else
        { return false; }
      val = (val << 4) | digit;
      s = s.offset (1);
    }
  let maxval: c_int = (1 << (n * 4)) - 1;
  *dst = ((val as c_uint) * 65535 / maxval as c_uint) as c_ushort;
  return true;
}

/* C 955-972: parse floating-point color component specification that starts at S
   and ends right before E.  Put the integer near-equivalent of that into *DST.  Return true
   if successful, false otherwise.  */
unsafe fn parse_float_color_comp (s: *const c_char, e: *const c_char,
                                  dst: *mut c_ushort) -> bool
{
  /* Only allow decimal float literals without whitespace.  */
  let mut p: *const c_char = s;
  while p < e
    {
      if ! ((*p >= '0' as c_char && *p <= '9' as c_char)
	    || *p == '.' as c_char || *p == '+' as c_char || *p == '-' as c_char
	    || *p == 'e' as c_char || *p == 'E' as c_char)
	{ return false; }
      p = p.offset (1);
    }
  let mut end: *mut c_char = std::ptr::null_mut ();
  let x: c_double = strtod (s, &mut end);
  if end == e as *mut c_char && 0.0 <= x && x <= 1.0
    {
      *dst = lrint (x * 65535.0) as c_ushort;
      return true;
    }
  else
    { return false; }
}

/* C 986-1027: parse SPEC as a numeric color specification and set *R, *G and *B.
   Return true on success, false on failure.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_color_spec (spec: *const c_char,
                                           r: *mut c_ushort, g: *mut c_ushort,
                                           b: *mut c_ushort) -> bool
{
  let len: isize = strlen (spec) as isize;
  if *spec == '#' as c_char
    {
      if (len - 1) % 3 == 0
        {
          let n: isize = (len - 1) / 3;
          return (   parse_hex_color_comp (spec.offset (1 + 0 * n),
					   spec.offset (1 + 1 * n), r)
                  && parse_hex_color_comp (spec.offset (1 + 1 * n),
					   spec.offset (1 + 2 * n), g)
                  && parse_hex_color_comp (spec.offset (1 + 2 * n),
					   spec.offset (1 + 3 * n), b));
        }
    }
  else if strncmp (spec, c"rgb:".as_ptr (), 4) == 0
    {
      let sep1: *const c_char = strchr (spec.offset (4), '/' as c_int) as *const c_char;
      if sep1.is_null ()
	{ return false; }
      let sep2: *const c_char = strchr (sep1.offset (1), '/' as c_int) as *const c_char;
      return (! sep2.is_null ()
              && parse_hex_color_comp (spec.offset (4), sep1, r)
              && parse_hex_color_comp (sep1.offset (1), sep2, g)
              && parse_hex_color_comp (sep2.offset (1), spec.offset (len), b));
    }
  else if strncmp (spec, c"rgbi:".as_ptr (), 5) == 0
    {
      let sep1: *const c_char = strchr (spec.offset (5), '/' as c_int) as *const c_char;
      if sep1.is_null ()
	{ return false; }
      let sep2: *const c_char = strchr (sep1.offset (1), '/' as c_int) as *const c_char;
      return (! sep2.is_null ()
	      && parse_float_color_comp (spec.offset (5), sep1, r)
	      && parse_float_color_comp (sep1.offset (1), sep2, g)
	      && parse_float_color_comp (sep2.offset (1), spec.offset (len), b));
    }
  return false;
}

DEFUN! ("color-values-from-color-spec",
       Fcolor_values_from_color_spec,
       Scolor_values_from_color_spec,
       1, 1, 0,
       doc: "Parse color SPEC as a numeric color and return (RED GREEN BLUE).\nThis function recognizes the following formats for SPEC:\n\n #RGB, where R, G and B are hex numbers of equal length, 1-4 digits each.\n rgb:R/G/B, where R, G, and B are hex numbers, 1-4 digits each.\n rgbi:R/G/B, where R, G and B are floating-point numbers in [0,1].\n\nIf SPEC is not in one of the above forms, return nil.\n\nEach of the 3 integer members of the resulting list, RED, GREEN, and BLUE,\nis normalized to have its value in [0,65535].",
       (spec: Lisp_Object),
       {
  CHECK_STRING (spec);
  let mut r: c_ushort = 0;
  let mut g: c_ushort = 0;
  let mut b: c_ushort = 0;
  return if parse_color_spec (SSDATA (spec), &mut r, &mut g, &mut b)
          { list3i (r as isize, g as isize, b as isize) }
          else
          { Qnil () };
});

/* C 1057-1074: parse RGB_LIST, and fill in the RGB fields of COLOR.
   RGB_LIST should contain (at least) 3 lisp integers.  Return true iff RGB_LIST is OK.  */

unsafe fn parse_rgb_list (mut rgb_list: Lisp_Object, color: *mut Emacs_Color) -> bool
{
  /* C's statement macro assigns both its free variables (the caller's `rgb_list` and
     `color`) and its `field` parameter; a `macro_rules!` cannot name a caller's local
     (docs/status.md 3.8), so the three field names are the arms and the two free variables
     are passed in, which keeps C's tokens in the ported text.  */
  macro_rules! PARSE_RGB_LIST_FIELD {
    (red) => {
      if CONSP (rgb_list) && FIXNUMP (XCAR (rgb_list))
	{
	  (*color).red = XFIXNUM (XCAR (rgb_list)) as c_ushort;
	  rgb_list = XCDR (rgb_list);
	}
      else
	{ return false; }
    };
    (green) => {
      if CONSP (rgb_list) && FIXNUMP (XCAR (rgb_list))
	{
	  (*color).green = XFIXNUM (XCAR (rgb_list)) as c_ushort;
	  rgb_list = XCDR (rgb_list);
	}
      else
	{ return false; }
    };
    (blue) => {
      if CONSP (rgb_list) && FIXNUMP (XCAR (rgb_list))
	{
	  (*color).blue = XFIXNUM (XCAR (rgb_list)) as c_ushort;
	  rgb_list = XCDR (rgb_list);
	}
      else
	{ return false; }
    };
  }

  PARSE_RGB_LIST_FIELD! (red);
  PARSE_RGB_LIST_FIELD! (green);
  PARSE_RGB_LIST_FIELD! (blue);

  return true;
}

/* C 1082-1139: lookup on frame F the color described by the lisp string COLOR.  */

unsafe fn tty_lookup_color (f: *mut frame, color: Lisp_Object,
			    tty_color: *mut Emacs_Color,
			    std_color: *mut Emacs_Color) -> bool
{
  let mut frame: Lisp_Object = Qnil ();
  let mut color_desc: Lisp_Object;

  if ! STRINGP (color) || NILP (Ffboundp (Qtty_color_desc ()))
    { return false; }

  XSETFRAME! (frame, f);

  color_desc = calln! (Qtty_color_desc (), color, frame);
  if CONSP (color_desc) && CONSP (XCDR (color_desc))
    {
      let mut rgb: Lisp_Object;

      if ! FIXNUMP (XCAR (XCDR (color_desc)))
	{ return false; }

      (*tty_color).pixel = XFIXNUM (XCAR (XCDR (color_desc))) as c_ulong;

      rgb = XCDR (XCDR (color_desc));
      if ! parse_rgb_list (rgb, tty_color)
	{ return false; }

      /* Should we fill in STD_COLOR too?  */
      if ! std_color.is_null ()
	{
	  /* Default STD_COLOR to the same as TTY_COLOR.  */
	  *std_color = *tty_color;

	  /* Do a quick check to see if the returned descriptor is
	     actually _exactly_ equal to COLOR, otherwise we have to
	     lookup STD_COLOR separately.  If it's impossible to lookup
	     a standard color, we just give up and use TTY_COLOR.  */
	  if (! STRINGP (XCAR (color_desc))
	      || NILP (Fstring_equal (color, XCAR (color_desc))))
	      && ! NILP (Ffboundp (Qtty_color_standard_values ()))
	    {
	      /* Look up STD_COLOR separately.  */
	      rgb = calln! (Qtty_color_standard_values (), color);
	      if ! parse_rgb_list (rgb, std_color)
		{ return false; }
	    }
	}

      return true;
    }
  else if NILP (Fsymbol_value (Qtty_defined_color_alist ()))
    /* We were called early during startup, and the colors are not
       yet set up in tty-defined-color-alist.  Don't return a failure
       indication, since this produces the annoying "Unable to
       load color" messages in the *Messages* buffer.  */
    { return true; }
  else
    /* tty-color-desc seems to have returned a bad value.  */
    { return false; }
}

/* C 1143-1174: an implementation of defined_color_hook for tty frames.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tty_defined_color (f: *mut frame, color_name: *const c_char,
					    color_def: *mut Emacs_Color, alloc: bool,
					    _makeIndex: bool) -> bool
{
  let _ = (alloc, _makeIndex);
  let mut status: bool = true;

  /* Defaults.  */
  (*color_def).pixel = FACE_TTY_DEFAULT_COLOR;
  (*color_def).red = 0;
  (*color_def).blue = 0;
  (*color_def).green = 0;

  if *color_name != 0
    {
      let lcolor: Lisp_Object = build_string (color_name);
      status = tty_lookup_color (f, lcolor, color_def, std::ptr::null_mut ());

      if (*color_def).pixel == FACE_TTY_DEFAULT_COLOR
	{
	  let color_name: *const c_char = SSDATA (lcolor);
	  if strcmp (color_name, c"unspecified-fg".as_ptr ()) == 0
	    { (*color_def).pixel = FACE_TTY_DEFAULT_FG_COLOR; }
	  else if strcmp (color_name, c"unspecified-bg".as_ptr ()) == 0
	    { (*color_def).pixel = FACE_TTY_DEFAULT_BG_COLOR; }
	}
    }

  if (*color_def).pixel != FACE_TTY_DEFAULT_COLOR
    { status = true; }

  return status;
}

/* C 1179-1207: given the index IDX of a tty color on frame F, return its name, a
   Lisp string.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tty_color_name (f: *mut frame, idx: c_int) -> Lisp_Object
{
  if idx >= 0 && ! NILP (Ffboundp (Qtty_color_by_index ()))
    {
      let mut frame: Lisp_Object = Qnil ();
      let coldesc: Lisp_Object;

      XSETFRAME! (frame, f);
      coldesc = calln! (Qtty_color_by_index (), make_fixnum (idx as isize), frame);

      if ! NILP (coldesc)
	{ return XCAR (coldesc); }
    }

  /* C's MSDOS arm (`FRAME_MSDOS_P` / `msdos_stdcolor_name`) is dead.  */

  if idx == FACE_TTY_DEFAULT_FG_COLOR as c_int
    { return build_string (unspecified_fg.as_ptr ()); }
  if idx == FACE_TTY_DEFAULT_BG_COLOR as c_int
    { return build_string (unspecified_bg.as_ptr ()); }

  return Qunspecified ();
}


/* C 1214-1234: return true if COLOR_NAME is a shade of gray (or white or
   black) on frame F.  */

unsafe fn face_color_gray_p (f: *mut frame, color_name: *const c_char) -> bool
{
  let mut color: Emacs_Color = std::mem::zeroed ();
  let gray_p: bool;

  /* C's `FRAME_TERMINAL (f)->defined_color_hook (...)`: the mirror's hook is an
     `Option`, so the call is the `if let Some (hook)` shape every ported hook call uses
     (`.unwrap ()` is not allowed, hard constraint 4).  */
  let defined_color = if let Some (hook) = (*FRAME_TERMINAL! (f)).defined_color_hook
    { hook (f as *mut c_void, color_name, &mut color, false, true) } else { false };
  if defined_color
    { gray_p = /* Any color sufficiently close to black counts as gray.  */
	      (color.red < 5000 && color.green < 5000 && color.blue < 5000)
	      ||
	      ((eabs! (color.red as c_int - color.green as c_int)
		< std::cmp::max (color.red, color.green) as c_int / 20)
	       && (eabs! (color.green as c_int - color.blue as c_int)
		   < std::cmp::max (color.green, color.blue) as c_int / 20)
	       && (eabs! (color.blue as c_int - color.red as c_int)
		   < std::cmp::max (color.blue, color.red) as c_int / 20)); }
  else
    { gray_p = false; }

  return gray_p;
}


/* C 1241-1261: return true if color COLOR_NAME can be displayed on frame F.  */

unsafe fn face_color_supported_p (f: *mut frame, color_name: *const c_char,
				  background_p: bool) -> bool
{
  let mut frame: Lisp_Object = Qnil ();
  let mut not_used: Emacs_Color = std::mem::zeroed ();

  XSETFRAME! (frame, f);
  return
    /* C's `HAVE_WINDOW_SYSTEM` arm (`FRAME_WINDOW_P`, `Fxw_display_color_p`,
       `Fx_display_grayscale_p`) is dead.  */
    tty_defined_color (f, color_name, &mut not_used, false, false);
}

DEFUN! ("color-gray-p", Fcolor_gray_p, Scolor_gray_p, 1, 2, 0,
       doc: "Return non-nil if COLOR is a shade of gray (or white or black).\nFRAME specifies the frame and thus the display for interpreting COLOR.\nIf FRAME is nil or omitted, use the selected frame.",
       (color: Lisp_Object, frame: Lisp_Object),
       {
  CHECK_STRING (color);
  return if face_color_gray_p (decode_any_frame (frame), SSDATA (color))
	  { Qt () } else { Qnil () };
});

DEFUN! ("color-supported-p", Fcolor_supported_p,
       Scolor_supported_p, 1, 3, 0,
       doc: "Return non-nil if COLOR can be displayed on FRAME.\nBACKGROUND-P non-nil means COLOR is used as a background.\nOtherwise, this function tells whether it can be used as a foreground.\nIf FRAME is nil or omitted, use the selected frame.\nCOLOR must be a valid color name.",
       (color: Lisp_Object, frame: Lisp_Object, background_p: Lisp_Object),
       {
  CHECK_STRING (color);
  return if face_color_supported_p (decode_any_frame (frame),
				  SSDATA (color), ! NILP (background_p))
	  { Qt () } else { Qnil () };
});

/* C 1293-1367: load color with name NAME for use by face FACE on frame F.  */

unsafe fn load_color2 (f: *mut frame, face: *mut face, name: Lisp_Object,
                       target_index: lface_attribute_index,
		       color: *mut Emacs_Color) -> c_ulong
{
  eassert! (STRINGP (name));
  eassert! (target_index == LFACE_FOREGROUND_INDEX
	   || target_index == LFACE_BACKGROUND_INDEX
	   || target_index == LFACE_UNDERLINE_INDEX
	   || target_index == LFACE_OVERLINE_INDEX
	   || target_index == LFACE_STRIKE_THROUGH_INDEX
	   || target_index == LFACE_BOX_INDEX);

  /* if the color map is full, defined_color_hook will return a best match
     to the values in an existing cell. */
  let defined_color = if let Some (hook) = (*FRAME_TERMINAL! (f)).defined_color_hook
    { hook (f as *mut c_void, SSDATA (name), color, true, true) } else { false };
  if ! defined_color
    {
      add_to_log (c"Unable to load color \"%s\"".as_ptr (), name);

      match target_index
	{
	  LFACE_FOREGROUND_INDEX =>
	    {
	      face_bits! (foreground_defaulted_p, face, true);
	      (*color).pixel = FRAME_FOREGROUND_PIXEL (f);
	    }

	  LFACE_BACKGROUND_INDEX =>
	    {
	      face_bits! (background_defaulted_p, face, true);
	      (*color).pixel = FRAME_BACKGROUND_PIXEL (f);
	    }

	  LFACE_UNDERLINE_INDEX =>
	    {
	      face_bits! (underline_defaulted_p, face, true);
	      (*color).pixel = FRAME_FOREGROUND_PIXEL (f);
	    }

	  LFACE_OVERLINE_INDEX =>
	    {
	      face_bits! (overline_color_defaulted_p, face, true);
	      (*color).pixel = FRAME_FOREGROUND_PIXEL (f);
	    }

	  LFACE_STRIKE_THROUGH_INDEX =>
	    {
	      face_bits! (strike_through_color_defaulted_p, face, true);
	      (*color).pixel = FRAME_FOREGROUND_PIXEL (f);
	    }

	  LFACE_BOX_INDEX =>
	    {
	      face_bits! (box_color_defaulted_p, face, true);
	      (*color).pixel = FRAME_FOREGROUND_PIXEL (f);
	    }

	  _ =>
	    { emacs_abort (); }
	}
    }

  return (*color).pixel;
}

/* C 1365-1374: load color with name NAME for use by face FACE on frame F.  C spells
   `static` here (`#ifndef MSDOS`), so the port keeps internal linkage.  */

unsafe fn load_color (f: *mut frame, face: *mut face, name: Lisp_Object,
				    target_index: lface_attribute_index) -> c_ulong
{
  let mut color: Emacs_Color = std::mem::zeroed ();
  return load_color2 (f, face, name, target_index, &mut color);
}

/* C 1391-1428: load the foreground/background of FACE from ATTRS, with the stipple
   fallback for unsupported colors.  */
#[cfg (feature = "have_window_system")]
unsafe fn load_face_colors (f: *mut frame, face: *mut face, attrs: *mut Lisp_Object)
{
  let mut fg: Lisp_Object;
  let mut bg: Lisp_Object;
  let mut dfg: Lisp_Object;
  let mut xfg: Emacs_Color = std::mem::zeroed ();
  let mut xbg: Emacs_Color = std::mem::zeroed ();

  bg = *attrs.offset (LFACE_BACKGROUND_INDEX as isize);
  fg = *attrs.offset (LFACE_FOREGROUND_INDEX as isize);

  /* Swap colors if face is inverse-video.  */
  if EQ (*attrs.offset (LFACE_INVERSE_INDEX as isize), Qt ())
    {
      let tmp: Lisp_Object = fg;
      fg = bg;
      bg = tmp;
    }

  /* Check for support for foreground, not for background because
     face_color_supported_p is smart enough to know that grays are
     "supported" as background because we are supposed to use stipple
     for them.  */
  if ! face_color_supported_p (f, SSDATA (bg), false)
      && ! NILP (Fbitmap_spec_p (Vface_default_stipple))
    {
      image_destroy_bitmap (f, (*face).stipple);
      (*face).stipple = load_pixmap (f, Vface_default_stipple);
    }

  (*face).background = load_color2 (f, face, bg, LFACE_BACKGROUND_INDEX, &mut xbg);
  (*face).foreground = load_color2 (f, face, fg, LFACE_FOREGROUND_INDEX, &mut xfg);

  dfg = *attrs.offset (LFACE_DISTANT_FOREGROUND_INDEX as isize);
  if ! NILP (dfg) && ! UNSPECIFIEDP! (dfg)
      && color_distance (&mut xbg, &mut xfg) < face_near_same_color_threshold as c_int
    {
      if EQ (*attrs.offset (LFACE_INVERSE_INDEX as isize), Qt ())
	{ (*face).background = load_color (f, face, dfg, LFACE_BACKGROUND_INDEX); }
      else
	{ (*face).foreground = load_color (f, face, dfg, LFACE_FOREGROUND_INDEX); }
    }
}


/***********************************************************************
			   XLFD Font Names
 ***********************************************************************/

/* C 1510-1527: an enumerator for each field of an XLFD font name.  */
pub type xlfd_field = c_int;
pub const XLFD_FOUNDRY: xlfd_field = 0;
pub const XLFD_FAMILY: xlfd_field = 1;
pub const XLFD_WEIGHT: xlfd_field = 2;
pub const XLFD_SLANT: xlfd_field = 3;
pub const XLFD_SWIDTH: xlfd_field = 4;
pub const XLFD_ADSTYLE: xlfd_field = 5;
pub const XLFD_PIXEL_SIZE: xlfd_field = 6;
pub const XLFD_POINT_SIZE: xlfd_field = 7;
pub const XLFD_RESX: xlfd_field = 8;
pub const XLFD_RESY: xlfd_field = 9;
pub const XLFD_SPACING: xlfd_field = 10;
pub const XLFD_AVGWIDTH: xlfd_field = 11;
pub const XLFD_REGISTRY: xlfd_field = 12;
pub const XLFD_ENCODING: xlfd_field = 13;
pub const XLFD_LAST: xlfd_field = 14;

/* C 1534: order by which font selection chooses fonts.  */
static mut font_sort_order: [c_int; 4] = [0; 4];

/* C 1536-1792: the `HAVE_WINDOW_SYSTEM` block (`compare_fonts_by_sort_order`,
   `x-family-fonts`, `x-list-fonts`) is dead.  */


/***********************************************************************
			      Lisp Faces
 ***********************************************************************/

/* C 1795-1813: access face attributes of face LFACE, a Lisp vector.  */
macro_rules! LFACE_FAMILY { ($lface:expr) => { AREF ($lface, LFACE_FAMILY_INDEX as isize) }; }
macro_rules! LFACE_FOUNDRY { ($lface:expr) => { AREF ($lface, LFACE_FOUNDRY_INDEX as isize) }; }
macro_rules! LFACE_HEIGHT { ($lface:expr) => { AREF ($lface, LFACE_HEIGHT_INDEX as isize) }; }
macro_rules! LFACE_WEIGHT { ($lface:expr) => { AREF ($lface, LFACE_WEIGHT_INDEX as isize) }; }
macro_rules! LFACE_SLANT { ($lface:expr) => { AREF ($lface, LFACE_SLANT_INDEX as isize) }; }
macro_rules! LFACE_UNDERLINE { ($lface:expr) => { AREF ($lface, LFACE_UNDERLINE_INDEX as isize) }; }
macro_rules! LFACE_INVERSE { ($lface:expr) => { AREF ($lface, LFACE_INVERSE_INDEX as isize) }; }
macro_rules! LFACE_FOREGROUND { ($lface:expr) => { AREF ($lface, LFACE_FOREGROUND_INDEX as isize) }; }
macro_rules! LFACE_BACKGROUND { ($lface:expr) => { AREF ($lface, LFACE_BACKGROUND_INDEX as isize) }; }
macro_rules! LFACE_STIPPLE { ($lface:expr) => { AREF ($lface, LFACE_STIPPLE_INDEX as isize) }; }
macro_rules! LFACE_SWIDTH { ($lface:expr) => { AREF ($lface, LFACE_SWIDTH_INDEX as isize) }; }
macro_rules! LFACE_OVERLINE { ($lface:expr) => { AREF ($lface, LFACE_OVERLINE_INDEX as isize) }; }
macro_rules! LFACE_STRIKE_THROUGH { ($lface:expr) => { AREF ($lface, LFACE_STRIKE_THROUGH_INDEX as isize) }; }
macro_rules! LFACE_BOX { ($lface:expr) => { AREF ($lface, LFACE_BOX_INDEX as isize) }; }
macro_rules! LFACE_FONT { ($lface:expr) => { AREF ($lface, LFACE_FONT_INDEX as isize) }; }
macro_rules! LFACE_INHERIT { ($lface:expr) => { AREF ($lface, LFACE_INHERIT_INDEX as isize) }; }
macro_rules! LFACE_FONTSET { ($lface:expr) => { AREF ($lface, LFACE_FONTSET_INDEX as isize) }; }
macro_rules! LFACE_EXTEND { ($lface:expr) => { AREF ($lface, LFACE_EXTEND_INDEX as isize) }; }
macro_rules! LFACE_DISTANT_FOREGROUND {
  ($lface:expr) => { AREF ($lface, LFACE_DISTANT_FOREGROUND_INDEX as isize) };
}

/* C 1816-1819: true if LFACE is a Lisp face.  */
macro_rules! LFACEP {
  ($lface:expr) => {
    (VECTORP ($lface)
     && ASIZE ($lface) == LFACE_VECTOR_SIZE as isize
     && EQ (AREF ($lface, 0 as isize), Qface ()))
  };
}

/* C 1826: face attribute symbols for each value of LFACE_*_INDEX.  */
static mut face_attr_sym: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
  [lisp_h_Qnil (); LFACE_VECTOR_SIZE as usize];

/* C 1828-1949: the `GLYPH_DEBUG` block (`check_lface_attrs`'s checks,
   `check_lface`'s `XVECTOR` walk) is dead; the live `#else` arm spells the two macros as
   no-ops, and the port keeps that text.  */
macro_rules! check_lface_attrs { ($attrs:expr) => { { let _ = $attrs; 0 } }; }
macro_rules! check_lface { ($lface:expr) => { { let _ = $lface; 0 } }; }


/***********************************************************************
			      Face-merge cycle checking
 ***********************************************************************/

/* C 1946-1967: the cycle-checking stack.  */
pub type named_merge_point_kind = c_int;
pub const NAMED_MERGE_POINT_NORMAL: named_merge_point_kind = 0;
pub const NAMED_MERGE_POINT_REMAP: named_merge_point_kind = 1;

/* A `named merge point' is simply a point during face-merging where we
   look up a face by name.  We keep a stack of which named lookups we're
   currently processing so that we can easily detect cycles, using a
   linked- list of struct named_merge_point structures, typically
   allocated on the stack frame of the named lookup functions which are
   active (so no consing is required).  */
#[repr(C)]
pub struct named_merge_point
{
  pub face_name: Lisp_Object,
  pub named_merge_point_kind: named_merge_point_kind,
  pub prev: *mut named_merge_point,
}

const _: () = core::assert! (std::mem::offset_of! (named_merge_point, prev) == 16);
const _: () = core::assert! (std::mem::size_of::<named_merge_point> () == 24);

/* C 1975-2004: if a face merging cycle is detected for FACE_NAME, return false,
   otherwise add NEW_NAMED_MERGE_POINT … as the head of the linked list pointed to by
   NAMED_MERGE_POINTS, and return true.  */

unsafe fn push_named_merge_point (new_named_merge_point: *mut named_merge_point,
				  face_name: Lisp_Object,
				  named_merge_point_kind: named_merge_point_kind,
				  named_merge_points: *mut *mut named_merge_point) -> bool
{
  let mut prev: *mut named_merge_point;

  prev = *named_merge_points;
  while ! prev.is_null ()
    {
      if EQ (face_name, (*prev).face_name)
	{
	  if (*prev).named_merge_point_kind == named_merge_point_kind
	    /* A cycle, so fail.  */
	    { return false; }
	  else if (*prev).named_merge_point_kind == NAMED_MERGE_POINT_REMAP
	    /* A remap `hides ' any previous normal merge points
	       (because the remap means that it's actually different face),
	       so as we know the current merge point must be normal, we
	       can just assume it's OK.  */
	    { break; }
	}
      prev = (*prev).prev;
    }

  (*new_named_merge_point).face_name = face_name;
  (*new_named_merge_point).named_merge_point_kind = named_merge_point_kind;
  (*new_named_merge_point).prev = *named_merge_points;

  *named_merge_points = new_named_merge_point;

  return true;
}

/* C 2013-2050: resolve face name FACE_NAME.  */

unsafe fn resolve_face_name (face_name: Lisp_Object, signal_p: bool) -> Lisp_Object
{
  let mut face_name = face_name;
  let mut orig_face: Lisp_Object;
  let mut tortoise: Lisp_Object;
  let mut hare: Lisp_Object;

  if STRINGP (face_name)
    { face_name = Fintern (face_name, Qnil ()); }

  if NILP (face_name) || ! SYMBOLP (face_name)
    { return face_name; }

  orig_face = face_name;
  tortoise = face_name;
  hare = face_name;

  loop
    {
      face_name = hare;
      hare = Fget (hare, Qface_alias ());
      if NILP (hare) || ! SYMBOLP (hare)
	{ break; }

      face_name = hare;
      hare = Fget (hare, Qface_alias ());
      if NILP (hare) || ! SYMBOLP (hare)
	{ break; }

      tortoise = Fget (tortoise, Qface_alias ());
      if BASE_EQ (hare, tortoise)
	{
	  if signal_p
	    { circular_list (orig_face); }
	  return Qdefault ();
	}
    }

  return face_name;
}

/* C 2059-2076: return the face definition of FACE_NAME on frame F.  */

unsafe fn lface_from_face_name_no_resolve (f: *mut frame, face_name: Lisp_Object,
					   signal_p: bool) -> Lisp_Object
{
  let lface: Lisp_Object;

  if ! f.is_null ()
    { lface = Fgethash (face_name, (*f).face_hash_table, Qnil ()); }
  else
    { lface = CDR (Fgethash (face_name, Vface_new_frame_defaults, Qnil ())); }

  if signal_p && NILP (lface)
    { signal_error (c"Invalid face".as_ptr (), face_name); }

  check_lface! (lface);

  return lface;
}

/* C 2085-2090.  */
unsafe fn lface_from_face_name (f: *mut frame, face_name: Lisp_Object,
				signal_p: bool) -> Lisp_Object
{
  let face_name = resolve_face_name (face_name, signal_p);
  return lface_from_face_name_no_resolve (f, face_name, signal_p);
}

/* C 2099-2113: get face attributes of face FACE_NAME from frame-local faces on
   frame F.  */
unsafe fn get_lface_attributes_no_remap (f: *mut frame, face_name: Lisp_Object,
					 attrs: *mut Lisp_Object, signal_p: bool) -> bool
{
  let lface: Lisp_Object;

  lface = lface_from_face_name_no_resolve (f, face_name, signal_p);

  if ! NILP (lface)
    { memcpy (attrs as *mut c_void, xvector_contents (lface) as *const c_void,
	      LFACE_VECTOR_SIZE as usize * std::mem::size_of::<Lisp_Object> ()); }

  return ! NILP (lface);
}

/* C 2123-2164.  */
unsafe fn get_lface_attributes (w: *mut window, f: *mut frame, face_name: Lisp_Object,
				attrs: *mut Lisp_Object, signal_p: bool,
				mut named_merge_points: *mut named_merge_point) -> bool
{
  let face_remapping: Lisp_Object;
  eassert! (w.is_null () || WINDOW_XFRAME! (w) == f);

  let face_name = resolve_face_name (face_name, signal_p);

  /* See if SYMBOL has been remapped to some other face (usually this is
     done buffer-locally).  We only do that of F is non-NULL, because
     face remapping is not relevant for default attributes of faces for
     future frames, and because merge_face_ref cannot handle NULL frames
     anyway.  */
  if ! f.is_null ()
    {
      face_remapping = assq_no_quit (face_name, Vface_remapping_alist);
      if CONSP (face_remapping)
	{
	  let mut named_merge_point: named_merge_point = std::mem::zeroed ();

	  if push_named_merge_point (&mut named_merge_point,
				      face_name, NAMED_MERGE_POINT_REMAP,
				      &mut named_merge_points)
	    {
	      let mut i: c_int;

	      for i in 1 .. LFACE_VECTOR_SIZE
		{ *attrs.offset (i as isize) = Qunspecified (); }

	      return merge_face_ref (w, f, XCDR (face_remapping), attrs,
				     signal_p, named_merge_points,
				     0);
	    }
	}
    }

  /* Default case, no remapping.  */
  return get_lface_attributes_no_remap (f, face_name, attrs, signal_p);
}

/* C 2170-2182: true iff all attributes in face attribute vector ATTRS are
   specified, i.e. are non-nil.  */

unsafe fn lface_fully_specified_p (attrs: *const Lisp_Object) -> bool
{
  let mut i: c_int;

  i = 1;
  while i < LFACE_VECTOR_SIZE
    {
      if i != LFACE_FONT_INDEX && i != LFACE_INHERIT_INDEX
	  && i != LFACE_DISTANT_FOREGROUND_INDEX
	{
	  if UNSPECIFIEDP! (*attrs.offset (i as isize))
	      || IGNORE_DEFFACE_P! (*attrs.offset (i as isize))
	    { break; }
	}
      i += 1;
    }

  return i == LFACE_VECTOR_SIZE;
}


/* Return true if LFACE1 and LFACE2 specify the same font (without considering
   charsets/registries).  Both must be fully specified (C 4667-4708).  */
#[cfg (feature = "have_window_system")]
unsafe fn lface_same_font_attributes_p (lface1: *const Lisp_Object,
				       lface2: *const Lisp_Object) -> bool
{
  eassert! (lface_fully_specified_p (lface1) && lface_fully_specified_p (lface2));
  dispextern_externs::xstrcasecmp (SSDATA (*lface1.offset (LFACE_FAMILY_INDEX as isize)),
	       SSDATA (*lface2.offset (LFACE_FAMILY_INDEX as isize))) == 0
    && dispextern_externs::xstrcasecmp (SSDATA (*lface1.offset (LFACE_FOUNDRY_INDEX as isize)),
		    SSDATA (*lface2.offset (LFACE_FOUNDRY_INDEX as isize))) == 0
    && EQ (*lface1.offset (LFACE_HEIGHT_INDEX as isize),
	   *lface2.offset (LFACE_HEIGHT_INDEX as isize))
    && EQ (*lface1.offset (LFACE_SWIDTH_INDEX as isize),
	   *lface2.offset (LFACE_SWIDTH_INDEX as isize))
    && EQ (*lface1.offset (LFACE_WEIGHT_INDEX as isize),
	   *lface2.offset (LFACE_WEIGHT_INDEX as isize))
    && EQ (*lface1.offset (LFACE_SLANT_INDEX as isize),
	   *lface2.offset (LFACE_SLANT_INDEX as isize))
    && EQ (*lface1.offset (LFACE_FONT_INDEX as isize),
	   *lface2.offset (LFACE_FONT_INDEX as isize))
    && (EQ (*lface1.offset (LFACE_FONTSET_INDEX as isize),
	    *lface2.offset (LFACE_FONTSET_INDEX as isize))
	|| (STRINGP (*lface1.offset (LFACE_FONTSET_INDEX as isize))
	    && STRINGP (*lface2.offset (LFACE_FONTSET_INDEX as isize))
	    && dispextern_externs::xstrcasecmp (SSDATA (*lface1.offset (LFACE_FONTSET_INDEX as isize)),
			    SSDATA (*lface2.offset (LFACE_FONTSET_INDEX as isize))) == 0))
}

/* Return the fontset id named by ATTRS' fontset attribute, or -1 (C 5971-5986).  */
#[cfg (feature = "have_window_system")]
unsafe fn face_fontset (attrs: *const Lisp_Object) -> c_int
{
  let name: Lisp_Object = *attrs.offset (LFACE_FONTSET_INDEX as isize);
  if ! STRINGP (name)
    { return -1; }
  return fs_query_fontset (name, 0);
}

/* C 6305-6318: clear the attributes named in `face-font-lax-matched-attributes'.  */
#[cfg (feature = "have_window_system")]
unsafe fn font_maybe_unset_attribute (font_object: Lisp_Object,
				      index: font_property_index, symbol: Lisp_Object)
{
  let mut tail: Lisp_Object = Vface_font_lax_matched_attributes;
  eassert! (CONSP (tail));
  'tail: loop
    {
      if EQ (XCAR (tail), symbol)
	{ ASET (font_object, index as isize, Qnil ()); }
      tail = XCDR (tail);
      if ! CONSP (tail)
	{ break 'tail; }
    }
}

/* C's `f->default_face_done_p` (frame.h 340, a `bool_bf`): the per-unit reader/writer shape
   `msdos.rs` uses; `include/frame.rs` carries the measured shift and the field's unit (1).  */
#[cfg (feature = "have_window_system")]
macro_rules! set_frame_flag1 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}

#[cfg (feature = "have_window_system")]
macro_rules! default_face_done_p {
  ($f:expr) => { (*$f).bits1 >> FRAME_DEFAULT_FACE_DONE_P_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_DEFAULT_FACE_DONE_P_SHIFT, $v) };
}

/* Set font-related attributes of Lisp face LFACE from FONT-OBJECT (C 2181-2245).  If FORCE_P is
   zero, set only unspecified attributes of LFACE; the `font` attribute is always set.  */
#[cfg (feature = "have_window_system")]
unsafe fn set_lface_from_font (f: *mut frame, lface: Lisp_Object,
			       font_object: Lisp_Object, force_p: bool)
{
  let mut val: Lisp_Object;
  let font: *mut font = XFONT_OBJECT (font_object);

  if force_p || UNSPECIFIEDP! (LFACE_FAMILY! (lface))
    {
      let family: Lisp_Object = AREF (font_object, FONT_FAMILY_INDEX as isize);
      ASET (lface, LFACE_FAMILY_INDEX as isize, SYMBOL_NAME (family));
    }

  if force_p || UNSPECIFIEDP! (LFACE_FOUNDRY! (lface))
    {
      let foundry: Lisp_Object = AREF (font_object, FONT_FOUNDRY_INDEX as isize);
      ASET (lface, LFACE_FOUNDRY_INDEX as isize, SYMBOL_NAME (foundry));
    }

  if force_p || UNSPECIFIEDP! (LFACE_HEIGHT! (lface))
    {
      let pt: c_int = PIXEL_TO_POINT! (((*font).pixel_size * 10) as c_double, FRAME_RES (f)) as c_int;
      eassert! (pt > 0);
      ASET (lface, LFACE_HEIGHT_INDEX as isize, make_fixnum (pt as EMACS_INT));
    }

  if force_p || UNSPECIFIEDP! (LFACE_WEIGHT! (lface))
    {
      val = FONT_WEIGHT_FOR_FACE! (font_object);
      ASET (lface, LFACE_WEIGHT_INDEX as isize, if ! NILP (val) { val } else { Qnormal () });
    }
  if force_p || UNSPECIFIEDP! (LFACE_SLANT! (lface))
    {
      val = FONT_SLANT_FOR_FACE! (font_object);
      ASET (lface, LFACE_SLANT_INDEX as isize, if ! NILP (val) { val } else { Qnormal () });
    }
  if force_p || UNSPECIFIEDP! (LFACE_SWIDTH! (lface))
    {
      val = FONT_WIDTH_FOR_FACE! (font_object);
      ASET (lface, LFACE_SWIDTH_INDEX as isize, if ! NILP (val) { val } else { Qnormal () });
    }

  ASET (lface, LFACE_FONT_INDEX as isize, font_object);
}

/* C 2184-2243: the `HAVE_WINDOW_SYSTEM` block (`set_lface_from_font`) is dead.  */

/* C 2252-2286: merges the face height FROM with the face height TO, and returns the
   merged height.  */

unsafe fn merge_face_heights (from: Lisp_Object, to: Lisp_Object,
			      invalid: Lisp_Object) -> Lisp_Object
{
  let mut result: Lisp_Object = invalid;

  if FIXNUMP (from)
    /* FROM is absolute, just use it as is.  */
    { result = from; }
  else if FLOATP (from)
    /* FROM is a scale, use it to adjust TO.  */
    {
      if FIXNUMP (to)
	/* relative X absolute => absolute */
	{ result = make_fixnum ((XFLOAT_DATA (from) * XFIXNUM (to) as c_double) as isize); }
      else if FLOATP (to)
	/* relative X relative => relative */
	{ result = make_float (XFLOAT_DATA (from) * XFLOAT_DATA (to)); }
      else if UNSPECIFIEDP! (to)
	{ result = from; }
    }
  else if FUNCTIONP (from)
    /* FROM is a function, which use to adjust TO.  */
    {
      /* Call function with current height as argument.
	 From is the new height.  */
      result = safe_calln! (from, to);

      /* Ensure that if TO was absolute, so is the result.  */
      if FIXNUMP (to) && ! FIXNUMP (result)
	{ result = invalid; }
    }

  return result;
}

/* C 2297-2388: merge two Lisp face attribute vectors on frame F, FROM and TO, and
   store the resulting attributes in TO.  */

unsafe fn merge_face_vectors (w: *mut window, f: *mut frame, from: *const Lisp_Object,
			      to: *mut Lisp_Object,
			      named_merge_points: *mut named_merge_point)
{
  let mut i: c_int;
  let mut font: Lisp_Object = Qnil ();
  let mut tospec: Lisp_Object;
  let mut adstyle: Lisp_Object;

  /* If FROM inherits from some other faces, merge their attributes into
     TO before merging FROM's direct attributes.  Note that an :inherit
     attribute of `unspecified' is the same as one of nil; we never
     merge :inherit attributes, so nil is more correct, but lots of
     other code uses `unspecified' as a generic value for face attributes. */
  if ! UNSPECIFIEDP! (*from.offset (LFACE_INHERIT_INDEX as isize))
      && ! NILP (*from.offset (LFACE_INHERIT_INDEX as isize))
    { merge_face_ref (w, f, *from.offset (LFACE_INHERIT_INDEX as isize),
		      to, false, named_merge_points,
		      0); }

  if FONT_SPEC_P (*from.offset (LFACE_FONT_INDEX as isize))
    {
      if ! UNSPECIFIEDP! (*to.offset (LFACE_FONT_INDEX as isize))
	{ font = merge_font_spec (*from.offset (LFACE_FONT_INDEX as isize),
				  *to.offset (LFACE_FONT_INDEX as isize)); }
      else
	{ font = copy_font_spec (*from.offset (LFACE_FONT_INDEX as isize)); }
      *to.offset (LFACE_FONT_INDEX as isize) = font;
    }

  for i in 1 .. LFACE_VECTOR_SIZE
    {
      if ! UNSPECIFIEDP! (*from.offset (i as isize))
	{
	  if i == LFACE_HEIGHT_INDEX && ! FIXNUMP (*from.offset (i as isize))
	    {
	      *to.offset (i as isize) = merge_face_heights (*from.offset (i as isize),
							    *to.offset (i as isize),
							    *to.offset (i as isize));
	      font_clear_prop (to, FONT_SIZE_INDEX);
	    }
	  else if i != LFACE_FONT_INDEX
		  && ! EQ (*to.offset (i as isize), *from.offset (i as isize))
	    {
	      *to.offset (i as isize) = *from.offset (i as isize);
	      if i >= LFACE_FAMILY_INDEX && i <= LFACE_SLANT_INDEX
		{ font_clear_prop (to,
				   if i == LFACE_FAMILY_INDEX { FONT_FAMILY_INDEX }
				   else if i == LFACE_FOUNDRY_INDEX { FONT_FOUNDRY_INDEX }
				   else if i == LFACE_SWIDTH_INDEX { FONT_WIDTH_INDEX }
				   else if i == LFACE_HEIGHT_INDEX { FONT_SIZE_INDEX }
				   else if i == LFACE_WEIGHT_INDEX { FONT_WEIGHT_INDEX }
				   else { FONT_SLANT_INDEX }); }
	    }
	}
    }

  /* If FROM specifies a font spec, make its contents take precedence
     over :family and other attributes.  This is needed for face
     remapping using :font to work.  */

  if ! NILP (font)
    {
      if ! NILP (AREF (font, FONT_FOUNDRY_INDEX as isize))
	{ *to.offset (LFACE_FOUNDRY_INDEX as isize)
	    = SYMBOL_NAME (AREF (font, FONT_FOUNDRY_INDEX as isize)); }
      if ! NILP (AREF (font, FONT_FAMILY_INDEX as isize))
	{ *to.offset (LFACE_FAMILY_INDEX as isize)
	    = SYMBOL_NAME (AREF (font, FONT_FAMILY_INDEX as isize)); }
      if ! NILP (AREF (font, FONT_WEIGHT_INDEX as isize))
	{ *to.offset (LFACE_WEIGHT_INDEX as isize) = FONT_WEIGHT_FOR_FACE! (font); }
      if ! NILP (AREF (font, FONT_SLANT_INDEX as isize))
	{ *to.offset (LFACE_SLANT_INDEX as isize) = FONT_SLANT_FOR_FACE! (font); }
      if ! NILP (AREF (font, FONT_WIDTH_INDEX as isize))
	{ *to.offset (LFACE_SWIDTH_INDEX as isize) = FONT_WIDTH_FOR_FACE! (font); }

      if ! NILP (AREF (font, FONT_ADSTYLE_INDEX as isize))
	{
	  /* If an adstyle is specified in FROM's font spec, create a
	     font spec for TO if none exists, and transfer the adstyle
	     there.  */

	  tospec = *to.offset (LFACE_FONT_INDEX as isize);
	  adstyle = AREF (font, FONT_ADSTYLE_INDEX as isize);

	  if ! NILP (tospec)
	    { tospec = copy_font_spec (tospec); }
	  else
	    { tospec = Ffont_spec (0, std::ptr::null_mut ()); }

	  *to.offset (LFACE_FONT_INDEX as isize) = tospec;
	  ASET (tospec, FONT_ADSTYLE_INDEX as isize, adstyle);
	}

      ASET (font, FONT_SIZE_INDEX as isize, Qnil ());
    }

  /* TO is always an absolute face, which should inherit from nothing.
     We blindly copy the :inherit attribute above and fix it up here.  */
  *to.offset (LFACE_INHERIT_INDEX as isize) = Qnil ();
}

/* C 2394-2439: chase the chain of face inheritance of frame F's face whose
   attributes are in ATTRS … and return that value.  */
unsafe fn face_inherited_attr (w: *mut window, f: *mut frame,
			       attrs: *mut Lisp_Object,
			       attr_idx: lface_attribute_index,
			       named_merge_points: *mut named_merge_point) -> Lisp_Object
{
  let mut inherited_attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];
  let mut attr_val: Lisp_Object = *attrs.offset (attr_idx as isize);

  memcpy (inherited_attrs.as_mut_ptr () as *mut c_void, attrs as *const c_void,
	  LFACE_VECTOR_SIZE as usize * std::mem::size_of_val (&inherited_attrs[0]));
  while UNSPECIFIEDP! (attr_val)
	 && ! NILP (inherited_attrs[LFACE_INHERIT_INDEX as usize])
	 && ! UNSPECIFIEDP! (inherited_attrs[LFACE_INHERIT_INDEX as usize])
    {
      let parent_face: Lisp_Object = inherited_attrs[LFACE_INHERIT_INDEX as usize];
      let mut ok: bool;

      if CONSP (parent_face)
	{
	  let mut tail: Lisp_Object;
	  ok = false;
	  tail = parent_face;
	  while ! NILP (tail)
	    {
	      ok = get_lface_attributes (w, f, XCAR (tail), inherited_attrs.as_mut_ptr (),
					 false, named_merge_points);
	      if ! ok
		{ break; }
	      attr_val = face_inherited_attr (w, f, inherited_attrs.as_mut_ptr (),
					      attr_idx, named_merge_points);
	      if ! UNSPECIFIEDP! (attr_val)
		{ break; }
	      tail = XCDR (tail);
	    }
	  if ! ok	/* bad face? */
	    { break; }
	}
      else
	{
	  ok = get_lface_attributes (w, f, parent_face, inherited_attrs.as_mut_ptr (),
				     false, named_merge_points);
	  if ! ok
	    { break; }
	  attr_val = inherited_attrs[attr_idx as usize];
	}
    }
  return attr_val;
}

/* C 2446-2495: chase the chain of inheritance for FACE on frame F, and return
   non-zero if FACE inherits from its CHILD face, directly or indirectly.  */
unsafe fn face_inheritance_cycle (f: *mut frame, face: Lisp_Object,
				  child: Lisp_Object) -> bool
{
  let mut face_attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];
  let mut parent_face: Lisp_Object;
  let mut ok: bool;
  let mut cycle_found: bool = false;

  eassert! (SYMBOLP (child));
  if CONSP (face)
    {
      let mut tail: Lisp_Object;
      tail = face;
      while CONSP (tail)
	{
	  let member_face: Lisp_Object = XCAR (tail);
	  ok = get_lface_attributes (std::ptr::null_mut (), f, member_face,
				     face_attrs.as_mut_ptr (), false,
				     std::ptr::null_mut ());
	  if ! ok
	    { break; }
	  parent_face = face_attrs[LFACE_INHERIT_INDEX as usize];
	  if EQ (parent_face, member_face)
	      || EQ (parent_face, child)
	    { cycle_found = true; }
	  else if ! NILP (parent_face)
		   && ! UNSPECIFIEDP! (parent_face)
		   && ! IGNORE_DEFFACE_P! (parent_face)
		   && ! RESET_P! (parent_face)
	    { cycle_found = face_inheritance_cycle (f, parent_face, child); }
	  if cycle_found
	    { break; }
	  tail = XCDR (tail);
	}
    }
  else if SYMBOLP (face)
    {
      ok = get_lface_attributes (std::ptr::null_mut (), f, face,
				 face_attrs.as_mut_ptr (), false,
				 std::ptr::null_mut ());
      if ok
	{
	  parent_face = face_attrs[LFACE_INHERIT_INDEX as usize];
	  if EQ (parent_face, face)
	      || EQ (parent_face, child)
	    { cycle_found = true; }
	  else if ! NILP (parent_face)
		   && ! UNSPECIFIEDP! (parent_face)
		   && ! IGNORE_DEFFACE_P! (parent_face)
		   && ! RESET_P! (parent_face)
	    { cycle_found = face_inheritance_cycle (f, parent_face, child); }
	}
    }

  return cycle_found;
}

/* C 2503-2542: merge the named face FACE_NAME on frame F, into the vector of face
   attributes TO.  */
unsafe fn merge_named_face (w: *mut window, f: *mut frame, face_name: Lisp_Object,
			    to: *mut Lisp_Object,
			    mut named_merge_points: *mut named_merge_point,
			    attr_filter: lface_attribute_index) -> bool
{
  let mut named_merge_point: named_merge_point = std::mem::zeroed ();

  if push_named_merge_point (&mut named_merge_point,
			     face_name, NAMED_MERGE_POINT_NORMAL,
			     &mut named_merge_points)
    {
      let mut from: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
	[Qnil (); LFACE_VECTOR_SIZE as usize];
      let mut val: Lisp_Object;
      let mut ok: bool = get_lface_attributes (w, f, face_name, from.as_mut_ptr (),
					       false, named_merge_points);
      if ok && ! EQ (face_name, Qdefault ())
	{
	  let deflt: *mut face = FACE_FROM_ID (f, DEFAULT_FACE_ID);
	  let mut i: c_int;
	  for i in 1 .. LFACE_VECTOR_SIZE
	    {
	      if EQ (from[i as usize], Qreset ())
		{ from[i as usize] = (*deflt).lface[i as usize]; }
	    }
	}

      if ok && (attr_filter == 0	       /* No filter.  */
		|| (! NILP (from[attr_filter as usize]) /* Filter, but specified.  */
		    && ! UNSPECIFIEDP! (from[attr_filter as usize]))
		/* Filter, unspecified, but inherited.  */
		|| (! NILP (from[LFACE_INHERIT_INDEX as usize])
		    && ! UNSPECIFIEDP! (from[LFACE_INHERIT_INDEX as usize])
		    && { val = face_inherited_attr (w, f, from.as_mut_ptr (), attr_filter,
						    named_merge_points);
			 (! NILP (val) && ! UNSPECIFIEDP! (val)) }))
	{ merge_face_vectors (w, f, from.as_ptr (), to, named_merge_points); }

      return ok;
    }
  else
    { return false; }
}

/* C 2555-2606: determine whether the face filter FILTER evaluated in window W
   matches.  */
unsafe fn evaluate_face_filter (filter: Lisp_Object, w: *mut window,
				ok: *mut bool, err_msgs: bool) -> bool
{
  let orig_filter: Lisp_Object = filter;
  let mut filter = filter;

  /* Inner braces keep compiler happy about the goto skipping variable
     initialization.  */
  'err_block: {
    if NILP (filter)
      { return true; }

    if face_filters_always_match
      { return true; }

    if ! CONSP (filter)
      { break 'err_block; }

    if ! EQ (XCAR (filter), QCwindow ())
      { break 'err_block; }
    filter = XCDR (filter);

    let parameter: Lisp_Object = XCAR (filter);
    filter = XCDR (filter);
    if ! CONSP (filter)
      { break 'err_block; }

    let value: Lisp_Object = XCAR (filter);
    filter = XCDR (filter);
    if ! NILP (filter)
      { break 'err_block; }

    if NILP (Fget (parameter, QCfiltered ()))
      { Fput (parameter, QCfiltered (), Qt ()); }

    let mut match_: bool = false;
    if ! w.is_null ()
      {
	let found: Lisp_Object = assq_no_quit (parameter, (*w).window_parameters);
	if ! NILP (found) && EQ (XCDR (found), value)
	  { match_ = true; }
      }

    return match_;
  }

  /* err: */
  if err_msgs
    { add_to_log (c"Invalid face filter %S".as_ptr (), orig_filter); }
  *ok = false;
  return false;
}

/* C 2620-2659: determine whether FACE_REF is a "filter" face specification.  */
unsafe fn filter_face_ref (face_ref: Lisp_Object, w: *mut window,
			   ok: *mut bool, err_msgs: bool) -> Lisp_Object
{
  let orig_face_ref: Lisp_Object = face_ref;
  let mut face_ref = face_ref;
  if ! CONSP (face_ref)
    { return face_ref; }

  /* Inner braces keep compiler happy about the goto skipping variable
     initialization.  */
  'err_block: {
    if ! EQ (XCAR (face_ref), QCfiltered ())
      { return face_ref; }
    face_ref = XCDR (face_ref);

    if ! CONSP (face_ref)
      { break 'err_block; }
    let filter: Lisp_Object = XCAR (face_ref);
    face_ref = XCDR (face_ref);

    if ! CONSP (face_ref)
      { break 'err_block; }
    let filtered_face_ref: Lisp_Object = XCAR (face_ref);
    face_ref = XCDR (face_ref);

    if ! NILP (face_ref)
      { break 'err_block; }

    return if evaluate_face_filter (filter, w, ok, err_msgs)
	   { filtered_face_ref } else { Qnil () };
  }

  /* err: */
  if err_msgs
    { add_to_log (c"Invalid face ref %S".as_ptr (), orig_face_ref); }
  *ok = false;
  return Qnil ();
}

/* C 2699-3026: merge face attributes from the lisp `face reference' FACE_REF on
   frame F into the face attribute vector TO.  */
unsafe fn merge_face_ref (w: *mut window, f: *mut frame, face_ref: Lisp_Object,
			  to: *mut Lisp_Object,
			  err_msgs: bool, named_merge_points: *mut named_merge_point,
			  attr_filter: lface_attribute_index) -> bool
{
  let mut ok: bool = true;		/* Succeed without an error? */
  let mut filtered_face_ref: Lisp_Object;
  let mut attr_filter_passed: bool = false;
  let mut face_ref = face_ref;

  filtered_face_ref = face_ref;
  loop
    {
      face_ref = filtered_face_ref;
      filtered_face_ref = filter_face_ref (face_ref, w, &mut ok, err_msgs);
      if ! (ok && ! EQ (face_ref, filtered_face_ref))
	{ break; }
    }

  if ! ok
    { return false; }

  if NILP (face_ref)
    { return true; }

  if CONSP (face_ref)
    {
      let first: Lisp_Object = XCAR (face_ref);

      if EQ (first, Qforeground_color ())
	  || EQ (first, Qbackground_color ())
	{
	  /* One of (FOREGROUND-COLOR . COLOR) or (BACKGROUND-COLOR
	     . COLOR).  COLOR must be a string.  */
	  let color_name: Lisp_Object = XCDR (face_ref);
	  let color: Lisp_Object = first;

	  if STRINGP (color_name)
	    {
	      if EQ (color, Qforeground_color ())
		{ *to.offset (LFACE_FOREGROUND_INDEX as isize) = color_name; }
	      else
		{ *to.offset (LFACE_BACKGROUND_INDEX as isize) = color_name; }
	    }
	  else
	    {
	      if err_msgs
		{ add_to_log (c"Invalid face color %S".as_ptr (), color_name); }
	      ok = false;
	    }
	}
      else if SYMBOLP (first)
	       && *SDATA (SYMBOL_NAME (first)) == ':' as u8
	{
	  /* Assume this is the property list form.  */
	  if attr_filter > 0
	    {
	      eassert! (attr_filter < LFACE_VECTOR_SIZE);
	      /* ATTR_FILTER positive means don't merge this face if
		 the corresponding attribute is nil, or not mentioned,
		 or if it's unspecified and the face doesn't inherit
		 from a face whose attribute is non-nil.  The code
		 below determines whether a face given as a property
		 list shall be merged.  */
	      let mut parent_face: Lisp_Object = Qnil ();
	      let mut attr_filter_seen: bool = false;
	      let mut face_ref_tem: Lisp_Object = face_ref;
	      while CONSP (face_ref_tem) && CONSP (XCDR (face_ref_tem))
		{
		  let keyword: Lisp_Object = XCAR (face_ref_tem);
		  let value: Lisp_Object = XCAR (XCDR (face_ref_tem));

		  if EQ (keyword, face_attr_sym[attr_filter as usize])
		    {
		      attr_filter_seen = true;
		      if NILP (value)
			{ return true; }
		    }
		  else if EQ (keyword, QCinherit ())
		    { parent_face = value; }
		  face_ref_tem = XCDR (XCDR (face_ref_tem));
		}
	      if ! attr_filter_seen
		{
		  if NILP (parent_face)
		    { return true; }

		  let mut scratch_attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
		    [Qnil (); LFACE_VECTOR_SIZE as usize];
		  let mut i: c_int;

		  scratch_attrs[0] = Qface ();
		  for i in 1 .. LFACE_VECTOR_SIZE
		    { scratch_attrs[i as usize] = Qunspecified (); }
		  if ! merge_face_ref (w, f, parent_face, scratch_attrs.as_mut_ptr (),
				       err_msgs, named_merge_points, 0)
		    {
		      add_to_log (c"Invalid face attribute %S %S".as_ptr (),
				  QCinherit (), parent_face);
		      return false;
		    }
		  if NILP (scratch_attrs[attr_filter as usize])
		      || UNSPECIFIEDP! (scratch_attrs[attr_filter as usize])
		    { return true; }
		}
	      attr_filter_passed = true;
	    }
	  while CONSP (face_ref) && CONSP (XCDR (face_ref))
	    {
	      let keyword: Lisp_Object = XCAR (face_ref);
	      let mut value: Lisp_Object = XCAR (XCDR (face_ref));
	      let mut err: bool = false;

	      /* Specifying `unspecified' is a no-op.  */
	      if EQ (value, Qunspecified ())
		{ }
	      else if EQ (keyword, QCfamily ())
		{
		  if STRINGP (value)
		    {
		      *to.offset (LFACE_FAMILY_INDEX as isize) = value;
		      font_clear_prop (to, FONT_FAMILY_INDEX);
		    }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCfoundry ())
		{
		  if STRINGP (value)
		    {
		      *to.offset (LFACE_FOUNDRY_INDEX as isize) = value;
		      font_clear_prop (to, FONT_FOUNDRY_INDEX);
		    }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCheight ())
		{
		  let new_height: Lisp_Object =
		    merge_face_heights (value, *to.offset (LFACE_HEIGHT_INDEX as isize),
					Qnil ());

		  if ! NILP (new_height)
		    {
		      *to.offset (LFACE_HEIGHT_INDEX as isize) = new_height;
		      font_clear_prop (to, FONT_SIZE_INDEX);
		    }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCweight ())
		{
		  if SYMBOLP (value) && FONT_WEIGHT_NAME_NUMERIC! (value) >= 0
		    {
		      *to.offset (LFACE_WEIGHT_INDEX as isize) = value;
		      font_clear_prop (to, FONT_WEIGHT_INDEX);
		    }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCslant ())
		{
		  if SYMBOLP (value) && FONT_SLANT_NAME_NUMERIC! (value) >= 0
		    {
		      *to.offset (LFACE_SLANT_INDEX as isize) = value;
		      font_clear_prop (to, FONT_SLANT_INDEX);
		    }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCunderline ())
		{
		  if EQ (value, Qt ())
		      || NILP (value)
		      || STRINGP (value)
		      || CONSP (value)
		    { *to.offset (LFACE_UNDERLINE_INDEX as isize) = value; }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCoverline ())
		{
		  if EQ (value, Qt ())
		      || NILP (value)
		      || STRINGP (value)
		    { *to.offset (LFACE_OVERLINE_INDEX as isize) = value; }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCstrike_through ())
		{
		  if EQ (value, Qt ())
		      || NILP (value)
		      || STRINGP (value)
		    { *to.offset (LFACE_STRIKE_THROUGH_INDEX as isize) = value; }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCbox ())
		{
		  if EQ (value, Qt ())
		    { value = make_fixnum (1); }
		  if (FIXNUMP (value) && XFIXNUM (value) != 0)
		      || STRINGP (value)
		      || CONSP (value)
		      || NILP (value)
		    { *to.offset (LFACE_BOX_INDEX as isize) = value; }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCinverse_video ())
		{
		  if EQ (value, Qt ()) || NILP (value)
		    { *to.offset (LFACE_INVERSE_INDEX as isize) = value; }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCforeground ())
		{
		  if STRINGP (value)
		    { *to.offset (LFACE_FOREGROUND_INDEX as isize) = value; }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCdistant_foreground ())
		{
		  if STRINGP (value)
		    { *to.offset (LFACE_DISTANT_FOREGROUND_INDEX as isize) = value; }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCbackground ())
		{
		  if STRINGP (value)
		    { *to.offset (LFACE_BACKGROUND_INDEX as isize) = value; }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCstipple ())
		{
		  /* C's `HAVE_WINDOW_SYSTEM` arm is dead, so the whole clause is empty.  */
		}
	      else if EQ (keyword, QCwidth ())
		{
		  if SYMBOLP (value) && FONT_WIDTH_NAME_NUMERIC! (value) >= 0
		    {
		      *to.offset (LFACE_SWIDTH_INDEX as isize) = value;
		      font_clear_prop (to, FONT_WIDTH_INDEX);
		    }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCfont ())
		{
		  if FONTP (value)
		    { *to.offset (LFACE_FONT_INDEX as isize) = value; }
		  else
		    { err = true; }
		}
	      else if EQ (keyword, QCinherit ())
		{
		  /* This is not really very useful; it's just like a
		     normal face reference.  */
		  if attr_filter_passed
		    {
		      /* We already know that this face was tested
			 against attr_filter and was found applicable,
			 so don't pass attr_filter to merge_face_ref.
			 This is for when a face is specified like
			 (:inherit FACE :extend t), but the parent
			 FACE itself doesn't specify :extend.  */
		      if ! merge_face_ref (w, f, value, to,
					   err_msgs, named_merge_points, 0)
			{ err = true; }
		    }
		  else if ! merge_face_ref (w, f, value, to,
					    err_msgs, named_merge_points,
					    attr_filter)
		    { err = true; }
		}
	      else if EQ (keyword, QCextend ())
		{
		  if EQ (value, Qt ()) || NILP (value)
		    { *to.offset (LFACE_EXTEND_INDEX as isize) = value; }
		  else
		    { err = true; }
		}
	      else
		{ err = true; }

	      if err
		{
		  add_to_log (c"Invalid face attribute %S %S".as_ptr (), keyword, value);
		  ok = false;
		}

	      face_ref = XCDR (XCDR (face_ref));
	    }
	}
      else
	{
	  /* This is a list of face refs.  Those at the beginning of the
	     list take precedence over what follows, so we have to merge
	     from the end backwards.  */
	  let next: Lisp_Object = XCDR (face_ref);

	  if ! NILP (next)
	    { ok = merge_face_ref (w, f, next, to, err_msgs,
				   named_merge_points, attr_filter); }

	  if ! merge_face_ref (w, f, first, to, err_msgs,
			       named_merge_points, attr_filter)
	    { ok = false; }
	}
    }
  else
    {
      /* FACE_REF ought to be a face name.  */
      ok = merge_named_face (w, f, face_ref, to, named_merge_points,
			     attr_filter);
      if ! ok && err_msgs
	{ add_to_log (c"Invalid face reference: %s".as_ptr (), face_ref); }
    }

  return ok;
}


/* frame.h 1425-1426: `AUTO_FRAME_ARG` (the `AUTO_LIST1`/`AUTO_CONS_EXPR` pair).  The mirror's
   copy (`include/frame.rs` 1009) cannot expand - it binds the name `$name:expr` while
   `AUTO_LIST1!` takes an `ident` there - so the unit keeps C's two lines with C's own name
   position (reported to the mirror's owner, not changed there).  */

/// C: lisp.h 5118: `extern void store_in_alist (Lisp_Object *, Lisp_Object, Lisp_Object);`
unsafe extern "C" {
  fn store_in_alist (alist: *mut Lisp_Object, prop: Lisp_Object, val: Lisp_Object);
}

DEFUN! ("internal-make-lisp-face", Finternal_make_lisp_face,
       Sinternal_make_lisp_face, 1, 2, 0,
       doc: "Make FACE, a symbol, a Lisp face with all attributes nil.\nIf FACE was not known as a face before, create a new one.\nIf optional argument FRAME is specified, make a frame-local face\nfor that frame.  Otherwise operate on the global face definition.\nValue is a vector of face attributes.",
       (face: Lisp_Object, frame: Lisp_Object),
       {
  let mut global_lface: Lisp_Object;
  let mut lface: Lisp_Object;
  let mut f: *mut frame;
  let mut i: c_int;

  CHECK_SYMBOL (face);
  global_lface = lface_from_face_name (std::ptr::null_mut (), face, false);

  if ! NILP (frame)
    {
      CHECK_LIVE_FRAME! (frame);
      f = XFRAME (frame);
      lface = lface_from_face_name (f, face, false);
    }
  else
    {
      f = std::ptr::null_mut ();
      lface = Qnil ();
    }

  /* Add a global definition if there is none.  */
  if NILP (global_lface)
    {
      /* Assign the new Lisp face a unique ID.  The mapping from Lisp
	 face id to Lisp face is given by the vector lface_id_to_name.
	 The mapping from Lisp face to Lisp face id is given by the
	 property `face' of the Lisp face name.  */
      if next_lface_id as ptrdiff_t == lface_id_to_name_size
	{
	  lface_id_to_name =
	    xpalloc (lface_id_to_name as *mut c_void, &mut lface_id_to_name_size, 1,
		     MAX_FACE_ID, std::mem::size_of::<Lisp_Object> () as ptrdiff_t)
	    as *mut Lisp_Object;
	}

      let face_id: Lisp_Object = make_fixnum (next_lface_id as isize);
      *lface_id_to_name.offset (next_lface_id as isize) = face;
      Fput (face, Qface (), face_id);
      next_lface_id += 1;

      global_lface = make_vector (LFACE_VECTOR_SIZE as ptrdiff_t, Qunspecified ());
      ASET (global_lface, 0, Qface ());
      Fputhash (face, Fcons (face_id, global_lface), Vface_new_frame_defaults);
    }
  else if f.is_null ()
    {
      for i in 1 .. LFACE_VECTOR_SIZE
	{ ASET (global_lface, i as isize, Qunspecified ()); }
    }

  /* Add a frame-local definition.  */
  if ! f.is_null ()
    {
      if NILP (lface)
	{
	  lface = make_vector (LFACE_VECTOR_SIZE as ptrdiff_t, Qunspecified ());
	  ASET (lface, 0, Qface ());
          Fputhash (face, lface, (*f).face_hash_table);
	}
      else
	{
	  for i in 1 .. LFACE_VECTOR_SIZE
	    { ASET (lface, i as isize, Qunspecified ()); }
	}
    }
  else
    { lface = global_lface; }

  /* Changing a named face means that all realized faces depending on
     that face are invalid.  Since we cannot tell which realized faces
     depend on the face, make sure they are all removed.  This is done
     by setting face_change.  The next call to init_iterator will then
     free realized faces.  */
  if NILP (Fget (face, Qface_no_inherit ()))
    {
      if ! f.is_null ()
	{
	  frame_bits! (face_change, f, true);
	  fset_redisplay (f as *mut c_void);
	}
      else
	{
	  face_change = true;
	  windows_or_buffers_changed = 54;
	}
    }

  eassert! (LFACEP! (lface));
  check_lface! (lface);
  return lface;
});

DEFUN! ("internal-lisp-face-p", Finternal_lisp_face_p,
       Sinternal_lisp_face_p, 1, 2, 0,
       doc: "Return non-nil if FACE names a face.\nFACE should be a symbol or string.\nIf optional second argument FRAME is non-nil, check for the\nexistence of a frame-local face with name FACE on that frame.\nOtherwise check for the existence of a global face.",
       (face: Lisp_Object, frame: Lisp_Object),
       {
  let lface: Lisp_Object;
  let face = resolve_face_name (face, true);

  if ! NILP (frame)
    {
      CHECK_LIVE_FRAME! (frame);
      lface = lface_from_face_name (XFRAME (frame), face, false);
    }
  else
    { lface = lface_from_face_name (std::ptr::null_mut (), face, false); }

  return lface;
});

DEFUN! ("internal-copy-lisp-face", Finternal_copy_lisp_face,
       Sinternal_copy_lisp_face, 4, 4, 0,
       doc: "Copy face FROM to TO.\nIf FRAME is t, copy the global face definition of FROM.\nOtherwise, copy the frame-local definition of FROM on FRAME.\nIf NEW-FRAME is a frame, copy that data into the frame-local\ndefinition of TO on NEW-FRAME.  If NEW-FRAME is nil,\nFRAME controls where the data is copied to.\n\nThe value is TO.",
       (from: Lisp_Object, to: Lisp_Object, frame: Lisp_Object,
        new_frame: Lisp_Object),
       {
  let lface: Lisp_Object;
  let copy: Lisp_Object;
  let mut f: *mut frame;
  let mut new_frame = new_frame;

  CHECK_SYMBOL (from);
  CHECK_SYMBOL (to);

  if EQ (frame, Qt ())
    {
      /* Copy global definition of FROM.  We don't make copies of
	 strings etc. because 20.2 didn't do it either.  */
      lface = lface_from_face_name (std::ptr::null_mut (), from, true);
      copy = Finternal_make_lisp_face (to, Qnil ());
      f = std::ptr::null_mut ();
    }
  else
    {
      /* Copy frame-local definition of FROM.  */
      if NILP (new_frame)
	{ new_frame = frame; }
      CHECK_LIVE_FRAME! (frame);
      CHECK_LIVE_FRAME! (new_frame);
      lface = lface_from_face_name (XFRAME (frame), from, true);
      copy = Finternal_make_lisp_face (to, new_frame);
      f = XFRAME (new_frame);
    }

  vcopy (copy, 0, xvector_contents (lface) as *const Lisp_Object,
	LFACE_VECTOR_SIZE as isize);

  /* Changing a named face means that all realized faces depending on
     that face are invalid.  Since we cannot tell which realized faces
     depend on the face, make sure they are all removed.  This is done
     by setting face_change.  The next call to init_iterator will then
     free realized faces.  */
  if NILP (Fget (to, Qface_no_inherit ()))
    {
      if ! f.is_null ()
	{
	  frame_bits! (face_change, f, true);
	  fset_redisplay (f as *mut c_void);
	}
      else
	{
	  face_change = true;
	  windows_or_buffers_changed = 55;
	}
    }

  return to;
});

/* C 3208-3211: `HANDLE_INVALID_NIL_VALUE`.  C's macro assigns the caller's `value`, so the
   port passes it in as the first metavariable (the PARSE_RGB_LIST_FIELD shape, status
   3.8).  */
macro_rules! HANDLE_INVALID_NIL_VALUE {
  ($value:expr, $a:expr, $f:expr) => {
    if NILP ($value)
      {
	add_to_log (c"Warning: setting attribute `%s' of face `%s': nil value is invalid, use `unspecified' instead.".as_ptr (), $a, $f);
	/* Compatibility with 20.x.  */
	$value = Qunspecified ();
      }
  };
}

DEFUN! ("internal-set-lisp-face-attribute", Finternal_set_lisp_face_attribute,
       Sinternal_set_lisp_face_attribute, 3, 4, 0,
       doc: "Set attribute ATTR of FACE to VALUE.\nFRAME being a frame means change the face on that frame.\nFRAME nil means change the face of the selected frame.\nFRAME t means change the default for new frames.\nFRAME 0 means change the face on all frames, and change the default\n  for new frames.",
       (face: Lisp_Object, attr: Lisp_Object, value: Lisp_Object, frame: Lisp_Object),
       {
  let mut face = face;
  let mut attr = attr;
  let mut value = value;
  let mut frame = frame;
  let mut lface: Lisp_Object;
  let mut old_value: Lisp_Object = Qnil ();
  /* Set one of enum font_property_index (> 0) if ATTR is one of
     font-related attributes other than QCfont and QCfontset.  */
  let mut prop_index: font_property_index = 0;
  let mut f: *mut frame;

  CHECK_SYMBOL (face);
  CHECK_SYMBOL (attr);

  face = resolve_face_name (face, true);

  /* If FRAME is 0, change face on all frames, and change the
     default for new frames.  */
  if FIXNUMP (frame) && XFIXNUM (frame) == 0
    {
      let mut tail: Lisp_Object;
      Finternal_set_lisp_face_attribute (face, attr, value, Qt ());
      FOR_EACH_FRAME! (tail, frame, {
	Finternal_set_lisp_face_attribute (face, attr, value, frame);
      });
      return face;
    }

  /* Set lface to the Lisp attribute vector of FACE.  */
  if EQ (frame, Qt ())
    {
      f = std::ptr::null_mut ();
      lface = lface_from_face_name (std::ptr::null_mut (), face, true);

      /* When updating face--new-frame-defaults, we put :ignore-defface
	 where the caller wants `unspecified'.  This forces the frame
	 defaults to ignore the defface value.  Otherwise, the defface
	 will take effect, which is generally not what is intended.
	 The value of that attribute will be inherited from some other
	 face during face merging.  See internal_merge_in_global_face. */
      if UNSPECIFIEDP! (value)
	{ value = QCignore_defface (); }
    }
  else
    {
      if NILP (frame)
	{ frame = selected_frame; }

      CHECK_LIVE_FRAME! (frame);
      f = XFRAME (frame);

      lface = lface_from_face_name (f, face, false);

      /* If a frame-local face doesn't exist yet, create one.  */
      if NILP (lface)
	{ lface = Finternal_make_lisp_face (face, frame); }
    }

  if EQ (attr, QCfamily ())
    {
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  CHECK_STRING (value);
	  if SCHARS (value) == 0
	    { signal_error (c"Invalid face family".as_ptr (), value); }
	}
      old_value = LFACE_FAMILY! (lface);
      ASET (lface, LFACE_FAMILY_INDEX as isize, value);
      prop_index = FONT_FAMILY_INDEX;
    }
  else if EQ (attr, QCfoundry ())
    {
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  CHECK_STRING (value);
	  if SCHARS (value) == 0
	    { signal_error (c"Invalid face foundry".as_ptr (), value); }
	}
      old_value = LFACE_FOUNDRY! (lface);
      ASET (lface, LFACE_FOUNDRY_INDEX as isize, value);
      prop_index = FONT_FOUNDRY_INDEX;
    }
  else if EQ (attr, QCheight ())
    {
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  if EQ (face, Qdefault ())
	    {
	      /* The default face must have an absolute size.  */
	      if ! FIXNUMP (value) || XFIXNUM (value) <= 0
		{ signal_error (c"Default face height not absolute and positive".as_ptr (),
				value); }
	    }
	  else
	    {
	      /* For non-default faces, do a test merge with a random
		 height to see if VALUE's ok. */
	      let test: Lisp_Object = merge_face_heights (value,
							  make_fixnum (10),
							  Qnil ());
	      if ! FIXNUMP (test) || XFIXNUM (test) <= 0
		{ signal_error (c"Face height does not produce a positive integer".as_ptr (),
				value); }
	    }
	}

      old_value = LFACE_HEIGHT! (lface);
      ASET (lface, LFACE_HEIGHT_INDEX as isize, value);
      prop_index = FONT_SIZE_INDEX;
    }
  else if EQ (attr, QCweight ())
    {
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  CHECK_SYMBOL (value);
	  if FONT_WEIGHT_NAME_NUMERIC! (value) < 0
	    { signal_error (c"Invalid face weight".as_ptr (), value); }
	}
      old_value = LFACE_WEIGHT! (lface);
      ASET (lface, LFACE_WEIGHT_INDEX as isize, value);
      prop_index = FONT_WEIGHT_INDEX;
    }
  else if EQ (attr, QCslant ())
    {
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  CHECK_SYMBOL (value);
	  if FONT_SLANT_NAME_NUMERIC! (value) < 0
	    { signal_error (c"Invalid face slant".as_ptr (), value); }
	}
      old_value = LFACE_SLANT! (lface);
      ASET (lface, LFACE_SLANT_INDEX as isize, value);
      prop_index = FONT_SLANT_INDEX;
    }
  else if EQ (attr, QCunderline ())
    {
      let mut valid_p: bool = false;

      if UNSPECIFIEDP! (value) || IGNORE_DEFFACE_P! (value) || RESET_P! (value)
	{ valid_p = true; }
      else if NILP (value) || EQ (value, Qt ())
	{ valid_p = true; }
      else if STRINGP (value) && SCHARS (value) > 0
	{ valid_p = true; }
      else if CONSP (value)
	{
	  let mut key: Lisp_Object;
	  let mut val: Lisp_Object;
	  let mut list: Lisp_Object;

	  list = value;
	  /* FIXME?  This errs on the side of acceptance.  Eg it accepts:
	       (defface foo '((t :underline 'foo) "doc")
	     Maybe this is intentional, maybe it isn't.
	     Non-nil symbols other than t are not documented as being valid.
	     Eg compare with inverse-video, which explicitly rejects them.
	  */
	  valid_p = true;

	  while ! NILP (CAR_SAFE (list))
	    {
	      key = CAR_SAFE (list);
	      list = CDR_SAFE (list);
	      val = CAR_SAFE (list);
	      list = CDR_SAFE (list);

	      if NILP (key) || (NILP (val)
				&& ! EQ (key, QCposition ()))
		{
		  valid_p = false;
		  break;
		}

	      else if EQ (key, QCcolor ())
		       && ! (EQ (val, Qforeground_color ())
			    || (STRINGP (val) && SCHARS (val) > 0))
		{
		  valid_p = false;
		  break;
		}

	      else if EQ (key, QCstyle ())
		       && ! (EQ (val, Qline ())
			    || EQ (val, Qdouble_line ())
			    || EQ (val, Qwave ())
			    || EQ (val, Qdots ())
			    || EQ (val, Qdashes ()))
		{
		  valid_p = false;
		  break;
		}
	    }
	}

      if ! valid_p
	{ signal_error (c"Invalid face underline".as_ptr (), value); }

      old_value = LFACE_UNDERLINE! (lface);
      ASET (lface, LFACE_UNDERLINE_INDEX as isize, value);
    }
  else if EQ (attr, QCoverline ())
    {
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  if (SYMBOLP (value)
	      && ! EQ (value, Qt ())
	      && ! NILP (value))
	      /* Overline color.  */
	      || (STRINGP (value)
		  && SCHARS (value) == 0)
	    { signal_error (c"Invalid face overline".as_ptr (), value); }
	}

      old_value = LFACE_OVERLINE! (lface);
      ASET (lface, LFACE_OVERLINE_INDEX as isize, value);
    }
  else if EQ (attr, QCstrike_through ())
    {
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  if (SYMBOLP (value)
	      && ! EQ (value, Qt ())
	      && ! NILP (value))
	      /* Strike-through color.  */
	      || (STRINGP (value)
		  && SCHARS (value) == 0)
	    { signal_error (c"Invalid face strike-through".as_ptr (), value); }
	}

      old_value = LFACE_STRIKE_THROUGH! (lface);
      ASET (lface, LFACE_STRIKE_THROUGH_INDEX as isize, value);
    }
  else if EQ (attr, QCbox ())
    {
      let mut valid_p: bool;

      /* Allow t meaning a simple box of width 1 in foreground color
	 of the face.  */
      if EQ (value, Qt ())
	{ value = make_fixnum (1); }

      if UNSPECIFIEDP! (value) || IGNORE_DEFFACE_P! (value) || RESET_P! (value)
	{ valid_p = true; }
      else if NILP (value)
	{ valid_p = true; }
      else if FIXNUMP (value)
	{ valid_p = XFIXNUM (value) != 0; }
      else if STRINGP (value)
	{ valid_p = SCHARS (value) > 0; }
      else if CONSP (value) && FIXNUMP (XCAR (value)) && FIXNUMP (XCDR (value))
	{ valid_p = true; }
      else if CONSP (value)
	{
	  let mut tem: Lisp_Object;

	  tem = value;
	  while CONSP (tem)
	    {
	      let mut k: Lisp_Object;
	      let mut v: Lisp_Object;

	      k = XCAR (tem);
	      tem = XCDR (tem);
	      if ! CONSP (tem)
		{ break; }
	      v = XCAR (tem);

	      if EQ (k, QCline_width ())
		{
		  if (! CONSP (v)
		      || ! FIXNUMP (XCAR (v))
		      || XFIXNUM (XCAR (v)) == 0
		      || ! FIXNUMP (XCDR (v)) || XFIXNUM (XCDR (v)) == 0)
		      && (! FIXNUMP (v) || XFIXNUM (v) == 0)
		    { break; }
		}
	      else if EQ (k, QCcolor ())
		{
		  if ! NILP (v) && (! STRINGP (v) || SCHARS (v) == 0)
		    { break; }
		}
	      else if EQ (k, QCstyle ())
		{
		  if ! NILP (v)
		      && ! EQ (v, Qpressed_button ())
		      && ! EQ (v, Qreleased_button ())
		      && ! EQ (v, Qflat_button ())
		    { break; }
		}
	      else
		{ break; }

	      tem = XCDR (tem);
	    }

	  valid_p = NILP (tem);
	}
      else
	{ valid_p = false; }

      if ! valid_p
	{ signal_error (c"Invalid face box".as_ptr (), value); }

      old_value = LFACE_BOX! (lface);
      ASET (lface, LFACE_BOX_INDEX as isize, value);
    }
  else if EQ (attr, QCinverse_video ())
    {
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  CHECK_SYMBOL (value);
	  if ! EQ (value, Qt ()) && ! NILP (value)
	    { signal_error (c"Invalid inverse-video face attribute value".as_ptr (), value); }
	}
      old_value = LFACE_INVERSE! (lface);
      ASET (lface, LFACE_INVERSE_INDEX as isize, value);
    }
  else if EQ (attr, QCextend ())
    {
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  CHECK_SYMBOL (value);
	  if ! EQ (value, Qt ()) && ! NILP (value)
	    { signal_error (c"Invalid extend face attribute value".as_ptr (), value); }
	}
      old_value = LFACE_EXTEND! (lface);
      ASET (lface, LFACE_EXTEND_INDEX as isize, value);
    }
  else if EQ (attr, QCforeground ())
    {
      HANDLE_INVALID_NIL_VALUE! (value, QCforeground (), face);
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  /* Don't check for valid color names here because it depends
	     on the frame (display) whether the color will be valid
	     when the face is realized.  */
	  CHECK_STRING (value);
	  if SCHARS (value) == 0
	    { signal_error (c"Empty foreground color value".as_ptr (), value); }
	}
      old_value = LFACE_FOREGROUND! (lface);
      ASET (lface, LFACE_FOREGROUND_INDEX as isize, value);
    }
  else if EQ (attr, QCdistant_foreground ())
    {
      HANDLE_INVALID_NIL_VALUE! (value, QCdistant_foreground (), face);
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  /* Don't check for valid color names here because it depends
	     on the frame (display) whether the color will be valid
	     when the face is realized.  */
	  CHECK_STRING (value);
	  if SCHARS (value) == 0
	    { signal_error (c"Empty distant-foreground color value".as_ptr (), value); }
	}
      old_value = LFACE_DISTANT_FOREGROUND! (lface);
      ASET (lface, LFACE_DISTANT_FOREGROUND_INDEX as isize, value);
    }
  else if EQ (attr, QCbackground ())
    {
      HANDLE_INVALID_NIL_VALUE! (value, QCbackground (), face);
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  /* Don't check for valid color names here because it depends
	     on the frame (display) whether the color will be valid
	     when the face is realized.  */
	  CHECK_STRING (value);
	  if SCHARS (value) == 0
	    { signal_error (c"Empty background color value".as_ptr (), value); }
	}
      old_value = LFACE_BACKGROUND! (lface);
      ASET (lface, LFACE_BACKGROUND_INDEX as isize, value);
    }
  else if EQ (attr, QCstipple ())
    {
      /* C's `HAVE_WINDOW_SYSTEM` arm (the `Fbitmap_spec_p` check and the two assignments)
	 is dead, so the branch body is empty.  */
    }
  else if EQ (attr, QCwidth ())
    {
      if ! UNSPECIFIEDP! (value)
	  && ! IGNORE_DEFFACE_P! (value)
	  && ! RESET_P! (value)
	{
	  CHECK_SYMBOL (value);
	  if FONT_WIDTH_NAME_NUMERIC! (value) < 0
	    { signal_error (c"Invalid face width".as_ptr (), value); }
	}
      old_value = LFACE_SWIDTH! (lface);
      ASET (lface, LFACE_SWIDTH_INDEX as isize, value);
      prop_index = FONT_WIDTH_INDEX;
    }
  else if EQ (attr, QCfont ())
    {
      /* C's `HAVE_WINDOW_SYSTEM` arm (the fontset lookup, `font_load_for_lface` and
	 `set_lface_from_font`) is dead, so the branch body is empty.  */
    }
  else if EQ (attr, QCfontset ())
    {
      /* C's `HAVE_WINDOW_SYSTEM` arm (`Fquery_fontset` and the assignment) is dead, so the
	 branch body is empty.  */
    }
  else if EQ (attr, QCinherit ())
    {
      let mut tail: Lisp_Object;
      if SYMBOLP (value)
	{ tail = Qnil (); }
      else
	{
	  tail = value;
	  while CONSP (tail)
	    {
	      if ! SYMBOLP (XCAR (tail))
		{ break; }
	      tail = XCDR (tail);
	    }
	}
      if EQ (value, face) || face_inheritance_cycle (f, value, face)
	{ signal_error (c"Face inheritance results in inheritance cycle".as_ptr (), value); }
      else if NILP (tail)
	{ ASET (lface, LFACE_INHERIT_INDEX as isize, value); }
      else
	{ signal_error (c"Invalid face inheritance".as_ptr (), value); }
    }
  else if EQ (attr, QCbold ())
    {
      old_value = LFACE_WEIGHT! (lface);
      if RESET_P! (value)
	{ ASET (lface, LFACE_WEIGHT_INDEX as isize, value); }
      else
	{ ASET (lface, LFACE_WEIGHT_INDEX as isize, if NILP (value) { Qnormal () } else { Qbold () }); }
      prop_index = FONT_WEIGHT_INDEX;
    }
  else if EQ (attr, QCitalic ())
    {
      attr = QCslant ();
      old_value = LFACE_SLANT! (lface);
      if RESET_P! (value)
	{ ASET (lface, LFACE_SLANT_INDEX as isize, value); }
      else
	{ ASET (lface, LFACE_SLANT_INDEX as isize, if NILP (value) { Qnormal () } else { Qitalic () }); }
      prop_index = FONT_SLANT_INDEX;
    }
  else
    { signal_error (c"Invalid face attribute name".as_ptr (), attr); }

  if prop_index != 0
    {
      /* If a font-related attribute other than QCfont and QCfontset
	 is specified, and if the original QCfont attribute has a font
	 (font-spec or font-object), set the corresponding property in
	 the font to nil so that the font selector doesn't think that
	 the attribute is mandatory.  Also, clear the average
	 width.  */
      font_clear_prop ((*XVECTOR (lface)).contents.as_mut_ptr (), prop_index);
    }

  /* Changing a named face means that all realized faces depending on
     that face are invalid.  Since we cannot tell which realized faces
     depend on the face, make sure they are all removed.  This is done
     by setting face_change.  The next call to init_iterator will then
     free realized faces.  */
  if ! EQ (frame, Qt ())
      && NILP (Fget (face, Qface_no_inherit ()))
      && NILP (Fequal (old_value, value))
    {
      frame_bits! (face_change, f, true);
      fset_redisplay (f as *mut c_void);
    }

  if ! UNSPECIFIEDP! (value) && ! IGNORE_DEFFACE_P! (value)
      && NILP (Fequal (old_value, value))
    {
      let mut param: Lisp_Object;

      param = Qnil ();

      if EQ (face, Qdefault ())
	{
	  /* C's `HAVE_WINDOW_SYSTEM` arm (`set_font_frame_param`) is dead.  */

	  if EQ (attr, QCforeground ())
	    { param = Qforeground_color (); }
	  else if EQ (attr, QCbackground ())
	    { param = Qbackground_color (); }
	}
      /* C's `HAVE_WINDOW_SYSTEM` arms (`Qscroll_bar`, `Qborder`, `Qcursor`, `Qmouse`) are
	 dead.  */
      else if EQ (face, Qmenu ())
	{
	  /* Indicate that we have to update the menu bar when realizing
	     faces on FRAME.  FRAME t change the default for new frames.
	     We do this by setting the flag in new face caches.  */
	  if FRAMEP (frame)
	    {
	      let f: *mut frame = XFRAME (frame);
	      if FRAME_FACE_CACHE (f) == std::ptr::null_mut ()
		{ FRAME_FACE_CACHE! (f, make_face_cache (f)); }
	      (*FRAME_FACE_CACHE (f)).menu_face_changed_p = 1;
	    }
	  else
	    { menu_face_changed_default = true; }
	}

      if ! NILP (param)
	{
	  if EQ (frame, Qt ())
	    /* Update `default-frame-alist', which is used for new frames.  */
	    {
	      store_in_alist (&mut Vdefault_frame_alist, param, value);
	    }
	  else
	    /* Update the current frame's parameters.  */
	    {
	      AUTO_FRAME_ARG! (arg, param, value);
	      Fmodify_frame_parameters (frame, arg);
	    }
	}
    }

  return face;
});

/* C 3876-3929: update the corresponding face when frame parameter PARAM on frame F
   has been assigned the value NEW_VALUE.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_face_from_frame_parameter (f: *mut frame, param: Lisp_Object,
							   new_value: Lisp_Object)
{
  let mut face: Lisp_Object = Qnil ();
  let lface: Lisp_Object;

  /* If there are no faces yet, give up.  This is the case when called
     from Fx_create_frame, and we do the necessary things later in
     face-set-after-frame-defaults.  */
  if XFIXNAT (Fhash_table_count ((*f).face_hash_table)) == 0
    { return; }

  if EQ (param, Qforeground_color ())
    {
      face = Qdefault ();
      lface = lface_from_face_name (f, face, true);
      ASET (lface, LFACE_FOREGROUND_INDEX as isize,
	    if STRINGP (new_value) { new_value } else { Qunspecified () });
      realize_basic_faces (f);
    }
  else if EQ (param, Qbackground_color ())
    {
      let mut frame: Lisp_Object = Qnil ();

      /* Changing the background color might change the background
	 mode, so that we have to load new defface specs.
	 Call frame-set-background-mode to do that.  */
      XSETFRAME! (frame, f);
      calln! (Qframe_set_background_mode (), frame);

      face = Qdefault ();
      lface = lface_from_face_name (f, face, true);
      ASET (lface, LFACE_BACKGROUND_INDEX as isize,
	    if STRINGP (new_value) { new_value } else { Qunspecified () });
      realize_basic_faces (f);
    }
  /* C's `HAVE_WINDOW_SYSTEM` arms (`Qborder_color`, `Qcursor_color`, `Qmouse_color`) are
     dead.  */

  /* Changing a named face means that all realized faces depending on
     that face are invalid.  Since we cannot tell which realized faces
     depend on the face, make sure they are all removed.  This is done
     by setting face_change.  The next call to init_iterator will then
     free realized faces.  */
  if ! NILP (face)
      && NILP (Fget (face, Qface_no_inherit ()))
    {
      frame_bits! (face_change, f, true);
      fset_redisplay (f as *mut c_void);
    }
}

/* C 3931-3951: the `HAVE_WINDOW_SYSTEM` block (`set_font_frame_param`) is dead.  */


DEFUN! ("face-attribute-relative-p", Fface_attribute_relative_p,
       Sface_attribute_relative_p,
       2, 2, 0,
       doc: "Check whether a face attribute value is relative.\nSpecifically, this function returns t if the attribute ATTRIBUTE\nwith the value VALUE is relative.\n\nA relative value is one that doesn't entirely override whatever is\ninherited from another face.  For most possible attributes,\nthe only relative value that users see is `unspecified'.\nHowever, for :height, floating point values are also relative.",
       (attribute: Lisp_Object, value: Lisp_Object),
       {
  if EQ (value, Qunspecified ()) || (EQ (value, QCignore_defface ()))
    { return Qt (); }
  else if EQ (attribute, QCheight ())
    { return if FIXNUMP (value) { Qnil () } else { Qt () }; }
  else
    { return Qnil (); }
});

DEFUN! ("merge-face-attribute", Fmerge_face_attribute, Smerge_face_attribute,
       3, 3, 0,
       doc: "Return face ATTRIBUTE VALUE1 merged with VALUE2.\nIf VALUE1 or VALUE2 are absolute (see `face-attribute-relative-p'), then\nthe result will be absolute, otherwise it will be relative.",
       (attribute: Lisp_Object, value1: Lisp_Object, value2: Lisp_Object),
       {
  if EQ (value1, Qunspecified ()) || EQ (value1, QCignore_defface ())
    { return value2; }
  else if EQ (attribute, QCheight ())
    { return merge_face_heights (value1, value2, value1); }
  else
    { return value1; }
});

DEFUN! ("internal-get-lisp-face-attribute", Finternal_get_lisp_face_attribute,
       Sinternal_get_lisp_face_attribute,
       2, 3, 0,
       doc: "Return face attribute KEYWORD of face SYMBOL.\nIf SYMBOL does not name a valid Lisp face or KEYWORD isn't a valid\nface attribute name, signal an error.\nIf the optional argument FRAME is given, report on face SYMBOL in that\nframe.  If FRAME is t, report on the defaults for face SYMBOL (for new\nframes).  If FRAME is omitted or nil, use the selected frame.",
       (symbol: Lisp_Object, keyword: Lisp_Object, frame: Lisp_Object),
       {
  let f: *mut frame = if EQ (frame, Qt ()) { std::ptr::null_mut () }
                      else { decode_live_frame (frame) };
  let lface: Lisp_Object = lface_from_face_name (f, symbol, true);
  let mut value: Lisp_Object = Qnil ();

  CHECK_SYMBOL (symbol);
  CHECK_SYMBOL (keyword);

  if EQ (keyword, QCfamily ())
    { value = LFACE_FAMILY! (lface); }
  else if EQ (keyword, QCfoundry ())
    { value = LFACE_FOUNDRY! (lface); }
  else if EQ (keyword, QCheight ())
    { value = LFACE_HEIGHT! (lface); }
  else if EQ (keyword, QCweight ())
    { value = LFACE_WEIGHT! (lface); }
  else if EQ (keyword, QCslant ())
    { value = LFACE_SLANT! (lface); }
  else if EQ (keyword, QCunderline ())
    { value = LFACE_UNDERLINE! (lface); }
  else if EQ (keyword, QCoverline ())
    { value = LFACE_OVERLINE! (lface); }
  else if EQ (keyword, QCstrike_through ())
    { value = LFACE_STRIKE_THROUGH! (lface); }
  else if EQ (keyword, QCbox ())
    { value = LFACE_BOX! (lface); }
  else if EQ (keyword, QCinverse_video ())
    { value = LFACE_INVERSE! (lface); }
  else if EQ (keyword, QCforeground ())
    { value = LFACE_FOREGROUND! (lface); }
  else if EQ (keyword, QCdistant_foreground ())
    { value = LFACE_DISTANT_FOREGROUND! (lface); }
  else if EQ (keyword, QCbackground ())
    { value = LFACE_BACKGROUND! (lface); }
  else if EQ (keyword, QCstipple ())
    { value = LFACE_STIPPLE! (lface); }
  else if EQ (keyword, QCwidth ())
    { value = LFACE_SWIDTH! (lface); }
  else if EQ (keyword, QCinherit ())
    { value = LFACE_INHERIT! (lface); }
  else if EQ (keyword, QCextend ())
    { value = LFACE_EXTEND! (lface); }
  else if EQ (keyword, QCfont ())
    { value = LFACE_FONT! (lface); }
  else if EQ (keyword, QCfontset ())
    { value = LFACE_FONTSET! (lface); }
  else
    { signal_error (c"Invalid face attribute name".as_ptr (), keyword); }

  if IGNORE_DEFFACE_P! (value)
    { return Qunspecified (); }

  return value;
});

DEFUN! ("internal-lisp-face-attribute-values",
       Finternal_lisp_face_attribute_values,
       Sinternal_lisp_face_attribute_values, 1, 1, 0,
       doc: "Return a list of valid discrete values for face attribute ATTR.\nValue is nil if ATTR doesn't have a discrete set of valid values.",
       (attr: Lisp_Object),
       {
  let mut result: Lisp_Object = Qnil ();

  CHECK_SYMBOL (attr);

  if EQ (attr, QCunderline ()) || EQ (attr, QCoverline ())
      || EQ (attr, QCstrike_through ())
      || EQ (attr, QCinverse_video ())
      || EQ (attr, QCextend ())
    { result = list2 (Qt (), Qnil ()); }

  return result;
});

DEFUN! ("internal-merge-in-global-face", Finternal_merge_in_global_face,
       Sinternal_merge_in_global_face, 2, 2, 0,
       doc: "Add attributes from frame-default definition of FACE to FACE on FRAME.\nDefault face attributes override any local face attributes.",
       (face: Lisp_Object, frame: Lisp_Object),
       {
  let mut i: c_int;
  let global_lface: Lisp_Object;
  let mut local_lface: Lisp_Object;
  let gvec: *mut Lisp_Object;
  let lvec: *mut Lisp_Object;
  let f: *mut frame = XFRAME (frame);

  CHECK_LIVE_FRAME! (frame);
  global_lface = lface_from_face_name (std::ptr::null_mut (), face, true);
  local_lface = lface_from_face_name (f, face, false);
  if NILP (local_lface)
    { local_lface = Finternal_make_lisp_face (face, frame); }

  /* Make every specified global attribute override the local one.
     BEWARE!! This is only used from `face-set-after-frame-default' where
     the local frame is defined from default specs in `face-defface-spec'
     and those should be overridden by global settings.  Hence the strange
     "global before local" priority.  */
  lvec = (*XVECTOR (local_lface)).contents.as_mut_ptr ();
  gvec = (*XVECTOR (global_lface)).contents.as_mut_ptr ();
  for i in 1 .. LFACE_VECTOR_SIZE
    {
      if IGNORE_DEFFACE_P! (*gvec.offset (i as isize))
	{ ASET (local_lface, i as isize, Qunspecified ()); }
      else if ! UNSPECIFIEDP! (*gvec.offset (i as isize))
	{ ASET (local_lface, i as isize, AREF (global_lface, i as isize)); }
    }

  /* If the default face was changed, update the face cache and the
     `font' frame parameter.  */
  if EQ (face, Qdefault ())
    {
      let c: *mut face_cache = FRAME_FACE_CACHE (f);
      let newface: *mut face;
      let oldface: *mut face =
	if ! c.is_null () { FACE_FROM_ID_OR_NULL (f, DEFAULT_FACE_ID) }
	else { std::ptr::null_mut () };
      let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
	[Qnil (); LFACE_VECTOR_SIZE as usize];

      /* This can be NULL (e.g., in batch mode).  */
      if ! oldface.is_null ()
	{
	  /* In some cases, realize_face below can call Lisp, which could
             trigger redisplay.  But we are in the process of realizing
             the default face, and therefore are not ready to do display.  */
	  let count: specpdl_ref = SPECPDL_INDEX ();
	  specbind (Qinhibit_redisplay (), Qt ());

	  /* Ensure that the face vector is fully specified by merging
	     the previously-cached vector.  */
	  memcpy (attrs.as_mut_ptr () as *mut c_void, (*oldface).lface.as_mut_ptr ()
		  as *const c_void, std::mem::size_of_val (&attrs));

	  merge_face_vectors (std::ptr::null_mut (), f, lvec, attrs.as_mut_ptr (),
			      0 as *mut named_merge_point);
	  vcopy (local_lface, 0, attrs.as_mut_ptr (), LFACE_VECTOR_SIZE as isize);
	  newface = realize_face (c, lvec, DEFAULT_FACE_ID);

	  if (! UNSPECIFIEDP! (*gvec.offset (LFACE_FAMILY_INDEX as isize))
	      || ! UNSPECIFIEDP! (*gvec.offset (LFACE_FOUNDRY_INDEX as isize))
	      || ! UNSPECIFIEDP! (*gvec.offset (LFACE_HEIGHT_INDEX as isize))
	      || ! UNSPECIFIEDP! (*gvec.offset (LFACE_WEIGHT_INDEX as isize))
	      || ! UNSPECIFIEDP! (*gvec.offset (LFACE_SLANT_INDEX as isize))
	      || ! UNSPECIFIEDP! (*gvec.offset (LFACE_SWIDTH_INDEX as isize))
	      || ! UNSPECIFIEDP! (*gvec.offset (LFACE_FONT_INDEX as isize)))
	      && ! (*newface).font.is_null ()
	    {
	      let name: Lisp_Object = (*((*newface).font)).props[FONT_NAME_INDEX as usize];
	      AUTO_FRAME_ARG! (arg, Qfont (), name);

	      /* C's `HAVE_WINDOW_SYSTEM` arm (`gui_set_frame_parameters_1`) is dead.  */
	      Fmodify_frame_parameters (frame, arg);
	    }

	  if STRINGP (*gvec.offset (LFACE_FOREGROUND_INDEX as isize))
	    {
	      AUTO_FRAME_ARG! (arg, Qforeground_color (),
			       *gvec.offset (LFACE_FOREGROUND_INDEX as isize));
	      Fmodify_frame_parameters (frame, arg);
	    }

	  if STRINGP (*gvec.offset (LFACE_BACKGROUND_INDEX as isize))
	    {
	      AUTO_FRAME_ARG! (arg, Qbackground_color (),
			       *gvec.offset (LFACE_BACKGROUND_INDEX as isize));
	      Fmodify_frame_parameters (frame, arg);
	    }

	  unbind_to (count, Qnil ());
	}
    }

  return Qnil ();
});

/* C 4418-4465: the following function is implemented for compatibility with 20.2.  */

DEFUN! ("face-font", Fface_font, Sface_font, 1, 3, 0,
       doc: "Return the font name of face FACE, or nil if it is unspecified.\nThe font name is, by default, for ASCII characters.\nIf the optional argument FRAME is given, report on face FACE in that frame.\nIf FRAME is t, report on the defaults for face FACE (for new frames).\n  The font default for a face is either nil, or a list\n  of the form (bold), (italic) or (bold italic).\nIf FRAME is omitted or nil, use the selected frame.\nIf FRAME is anything but t, and the optional third argument CHARACTER\nis given, return the font name used by FACE for CHARACTER on FRAME.",
       (face: Lisp_Object, frame: Lisp_Object, character: Lisp_Object),
       {
  if EQ (frame, Qt ())
    {
      let mut result: Lisp_Object = Qnil ();
      let lface: Lisp_Object = lface_from_face_name (std::ptr::null_mut (), face, true);

      if ! UNSPECIFIEDP! (LFACE_WEIGHT! (lface))
	  && ! EQ (LFACE_WEIGHT! (lface), Qnormal ())
	{ result = Fcons (Qbold (), result); }

      if ! UNSPECIFIEDP! (LFACE_SLANT! (lface))
	  && ! EQ (LFACE_SLANT! (lface), Qnormal ())
	{ result = Fcons (Qitalic (), result); }

      return result;
    }
  else
    {
      let f: *mut frame = decode_live_frame (frame);
      let face_id: c_int = lookup_named_face (std::ptr::null_mut (), f, face, true);
      let fface: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);

      if fface.is_null ()
	{ return Qnil (); }
      /* C's `HAVE_WINDOW_SYSTEM` arm (the `FRAME_WINDOW_P`/`CHECK_CHARACTER` path and
	 `fface->font->props[FONT_NAME_INDEX]`) is dead.  */
      return build_string (if FRAME_MSDOS_P (f)
			   { c"ms-dos".as_ptr () }
			   else if FRAME_W32_P (f) { c"w32term".as_ptr () }
			   else { c"tty".as_ptr () });
    }
});

/* C 4479-4508: compare face-attribute values v1 and v2 for equality.  */

unsafe fn face_attr_equal_p (v1: Lisp_Object, v2: Lisp_Object) -> bool
{
  /* Type can differ, e.g. when one attribute is unspecified, i.e. nil,
     and the other is specified.  */
  if XTYPE (v1) != XTYPE (v2)
    { return false; }

  if EQ (v1, v2)
    { return true; }

  match XTYPE (v1)
    {
    Lisp_Type::Lisp_String =>
      {
	if SBYTES (v1) != SBYTES (v2)
	  { return false; }

	return memcmp (SDATA (v1) as *const c_void, SDATA (v2) as *const c_void,
		       SBYTES (v1) as usize) == 0;
      }

    Lisp_Type::Lisp_Int0 | Lisp_Type::Lisp_Int1 | Lisp_Type::Lisp_Symbol =>
      { return false; }

    _ =>
      { return ! NILP (Fequal (v1, v2)); }
    }
}

/* C 4516-4527: compare face vectors V1 and V2 for equality.  */

unsafe fn lface_equal_p (v1: *const Lisp_Object, v2: *const Lisp_Object) -> bool
{
  let mut i: c_int;
  let mut equal_p: bool = true;

  i = 1;
  while i < LFACE_VECTOR_SIZE && equal_p
    {
      equal_p = face_attr_equal_p (*v1.offset (i as isize), *v2.offset (i as isize));
      i += 1;
    }

  return equal_p;
}

DEFUN! ("internal-lisp-face-equal-p", Finternal_lisp_face_equal_p,
       Sinternal_lisp_face_equal_p, 2, 4, 0,
       doc: "True if FACE1 and FACE2 are equal.\nIf the optional argument FRAME is given, report on FACE1 and FACE2 in that frame.\nIf FRAME is t, report on the defaults for FACE1 and FACE2 (for new frames).\nIf FRAME is omitted or nil, use the selected frame.\nOptional fourth argument INHERIT, if non-nil, means the faces\nare considered equal if one inherits from the other in a way\nthat makes them have the same attributes when used on display.",
       (face1: Lisp_Object, face2: Lisp_Object, frame: Lisp_Object, inherit: Lisp_Object),
       {
  let mut equal_p: bool;
  let f: *mut frame;
  let lface1: Lisp_Object;
  let lface2: Lisp_Object;

  /* Don't use decode_window_system_frame here because this function
     is called before X frames exist.  At that time, if FRAME is nil,
     selected_frame will be used which is the frame dumped with
     Emacs.  That frame is not an X frame.  */
  f = if EQ (frame, Qt ()) { std::ptr::null_mut () } else { decode_live_frame (frame) };

  lface1 = lface_from_face_name (f, face1, true);
  lface2 = lface_from_face_name (f, face2, true);
  equal_p = lface_equal_p ((*XVECTOR (lface1)).contents.as_mut_ptr (), (*XVECTOR (lface2)).contents.as_mut_ptr ());

  if ! (NILP (inherit) || equal_p)
    {
      /* The below is a subset of merging the descendant face with its
         parent(s).  We only consider a direct inheritance (so no FACE1
         that inherits from some other face which inherits from FACE2),
         and the values of :inherit that are lists are not considered.
         This is enough in simple cases such as the line-number-current
         face that inherits from line-number.  */
      let mut attrs1: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
	[Qnil (); LFACE_VECTOR_SIZE as usize];
      let mut attrs2: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
	[Qnil (); LFACE_VECTOR_SIZE as usize];
      let mut i: c_int;
      memcpy (attrs1.as_mut_ptr () as *mut c_void, xvector_contents (lface1)
	      as *const c_void, std::mem::size_of_val (&attrs1));
      memcpy (attrs2.as_mut_ptr () as *mut c_void, xvector_contents (lface2)
	      as *const c_void, std::mem::size_of_val (&attrs2));
      /* If either face inherits from the other one, and all the other
         face attributes of the inheriting face are either unspecified
         or equal to those of the parent face, consider the faces equal.  */
      if EQ (attrs1[LFACE_INHERIT_INDEX as usize], face2)
	{
	  equal_p = true;
	  i = 1;
	  while i < LFACE_VECTOR_SIZE && equal_p
	    {
	      if i == LFACE_INHERIT_INDEX
		{ i += 1; continue; }
	      equal_p = face_attr_equal_p (attrs1[i as usize], attrs2[i as usize])
			|| UNSPECIFIEDP! (attrs1[i as usize]);
	      i += 1;
	    }
	}
      else if EQ (attrs2[LFACE_INHERIT_INDEX as usize], face1)
	{
	  equal_p = true;
	  i = 1;
	  while i < LFACE_VECTOR_SIZE && equal_p
	    {
	      if i == LFACE_INHERIT_INDEX
		{ i += 1; continue; }
	      equal_p = face_attr_equal_p (attrs1[i as usize], attrs2[i as usize])
			|| UNSPECIFIEDP! (attrs2[i as usize]);
	      i += 1;
	    }
	}

    }

  return if equal_p { Qt () } else { Qnil () };
});

DEFUN! ("internal-lisp-face-empty-p", Finternal_lisp_face_empty_p,
       Sinternal_lisp_face_empty_p, 1, 2, 0,
       doc: "True if FACE has no attribute specified.\nIf the optional argument FRAME is given, report on face FACE in that frame.\nIf FRAME is t, report on the defaults for face FACE (for new frames).\nIf FRAME is omitted or nil, use the selected frame.",
       (face: Lisp_Object, frame: Lisp_Object),
       {
  let f: *mut frame = if EQ (frame, Qt ()) { std::ptr::null_mut () }
                      else { decode_live_frame (frame) };
  let lface: Lisp_Object = lface_from_face_name (f, face, true);
  let mut i: c_int;

  i = 1;
  while i < LFACE_VECTOR_SIZE
    {
      if ! UNSPECIFIEDP! (AREF (lface, i as isize))
	{ break; }
      i += 1;
    }

  return if i == LFACE_VECTOR_SIZE { Qt () } else { Qnil () };
});

DEFUN! ("frame--face-hash-table", Fframe_face_hash_table, Sframe_face_hash_table,
       0, 1, 0,
       doc: "Return a hash table of frame-local faces defined on FRAME.\nFor internal use only.",
       (frame: Lisp_Object),
       {
  return (*decode_live_frame (frame)).face_hash_table;
});


/* C 4632-4641: return a hash code for Lisp string STRING with case ignored.  */

unsafe fn hash_string_case_insensitive (string: Lisp_Object) -> uintptr_t
{
  let mut s: *const c_uchar;
  let mut hash: uintptr_t = 0;
  eassert! (STRINGP (string));
  s = SDATA (string) as *const c_uchar;
  while *s != 0
    {
      hash = (hash << 1) ^ c_tolower (*s as c_int) as uintptr_t;
      s = s.offset (1);
    }
  return hash;
}

/* C 4646-4657: return a hash code for face attribute vector V.  */

unsafe fn lface_hash (v: *const Lisp_Object) -> uintptr_t
{
  return hash_string_case_insensitive (*v.offset (LFACE_FAMILY_INDEX as isize))
	 ^ hash_string_case_insensitive (*v.offset (LFACE_FOUNDRY_INDEX as isize))
	 ^ hash_string_case_insensitive (*v.offset (LFACE_FOREGROUND_INDEX as isize))
	 ^ hash_string_case_insensitive (*v.offset (LFACE_BACKGROUND_INDEX as isize))
	 ^ XHASH (*v.offset (LFACE_WEIGHT_INDEX as isize)) as uintptr_t
	 ^ XHASH (*v.offset (LFACE_SLANT_INDEX as isize)) as uintptr_t
	 ^ XHASH (*v.offset (LFACE_SWIDTH_INDEX as isize)) as uintptr_t
	 ^ XHASH (*v.offset (LFACE_HEIGHT_INDEX as isize)) as uintptr_t;
}

/* C 4659-4708: the `HAVE_WINDOW_SYSTEM` block (`lface_same_font_attributes_p`) is dead.  */


/***********************************************************************
			    Realized Faces
 ***********************************************************************/

/* C 4695-4708: allocate and return a new realized face for Lisp face attribute
   vector ATTR.  */

unsafe fn make_realized_face (attr: *const Lisp_Object) -> *mut face
{
  /* `enum { off = offsetof (struct face, id) };` — the compile-time offset the port
     keeps as a local the memset below uses.  */
  let off: usize = std::mem::offset_of! (face, id);
  let face: *mut face = xmalloc (std::mem::size_of::<face> ()) as *mut face;

  memcpy ((*face).lface.as_mut_ptr () as *mut c_void, attr as *const c_void,
	  std::mem::size_of_val (&(*face).lface));
  memset (&mut (*face).id as *mut c_int as *mut c_void, 0,
	  std::mem::size_of::<face> ()
	  - off);
  (*face).ascii_face = face;

  return face;
}

/* C 4714-4748: free realized face FACE, including its X resources.  FACE may be null.  */

unsafe fn free_realized_face (f: *mut frame, face: *mut face)
{
  let _ = f;
  if ! face.is_null ()
    {
      /* C's `HAVE_WINDOW_SYSTEM` body (the fontset, GC and bitmap releases) is dead.  */
      xfree (face as *mut c_void);
    }
}

/* C 4759-4805: `prepare_face_for_display` - "Prepare face FACE for subsequent display on frame F.
   This must be called before using X resources of FACE to allocate GCs if they haven't been
   allocated yet or have been freed by clearing the face cache."  Inside C 4746's
   `#ifdef HAVE_WINDOW_SYSTEM`: dead on the frozen target, live on Android, whose library
   imported the name (status 4.756).  */

/// C: dispextern.h 3862: `ptrdiff_t image_bitmap_pixmap (struct frame *, ptrdiff_t);` and
/// font.h 545's `void font_prepare_for_face (struct frame *, struct face *);` - the two calls
/// `prepare_face_for_display` makes; C has both prototypes through its headers.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  fn image_bitmap_pixmap (f: *mut frame, id: ptrdiff_t) -> ptrdiff_t;
  fn font_prepare_for_face (f: *mut frame, face: *mut face);
}

/// C 243-249's `#define GCGraphicsExposures 0` for this backend (C 247 is the `HAVE_ANDROID` arm).
#[cfg (feature = "have_window_system")]
const GCGraphicsExposures: c_int = 0;

/// C 618-624: the Android arm of `x_create_gc` (`static struct android_gc *x_create_gc (struct
/// frame *f, unsigned long value_mask, Emacs_GC *xgcv)`), with the `androidgui.h` entry C calls.
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
unsafe extern "C" {
  /// C: androidgui.h 926-927: `extern struct android_gc *android_create_gc (enum
  /// android_gc_value_mask, struct android_gc_values *);`
  fn android_create_gc (valuemask: android_gc_value_mask, values: *mut android_gc_values)
    -> *mut android_gc;
}

#[cfg (all (feature = "have_window_system", feature = "have_android"))]
unsafe fn x_create_gc (f: *mut frame, value_mask: c_ulong, xgcv: *mut Emacs_GC) -> *mut android_gc
{                                                                  // C 620-624
  let _ = f;
  return android_create_gc (value_mask as android_gc_value_mask, xgcv);
}

#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_face_for_display (f: *mut frame, face: *mut face)
{
  let mut egc: Emacs_GC = std::mem::zeroed ();            // C 4761
  let mut mask: c_ulong;                                  // C 4762

  eassert! (FRAME_WINDOW_P (f));

  if (*face).gc.is_null ()
    {
      mask = (GCForeground | GCBackground | GCGraphicsExposures) as c_ulong;

      egc.foreground = (*face).foreground;
      egc.background = (*face).background;
      /* C 4775-4787's `#ifdef HAVE_X_WINDOWS` arm (`graphics_exposures`/`line_width`) is dead.  */

      block_input ();
      /* C 4789-4800: `#if defined HAVE_X_WINDOWS || defined HAVE_ANDROID` - live on Android; its
	 inner `#ifndef ANDROID_STUBIFY` picks the bitmap, and C's `#else` arm aborts.  */
      {
	if (*face).stipple != 0
	  {
	    egc.fill_style = FillOpaqueStippled;
	    #[cfg (feature = "not_android_stubify")]
	    {
	      egc.stipple = image_bitmap_pixmap (f, (*face).stipple) as android_pixmap;
	    }
	    #[cfg (feature = "android_stubify")]
	    {
	      emacs_abort ();
	    }
	    mask |= (GCFillStyle | GCStipple) as c_ulong;
	  }
      }
      (*face).gc = x_create_gc (f, mask, &raw mut egc) as *mut c_void;
      if !(*face).font.is_null ()
	{ font_prepare_for_face (f, face); }
      unblock_input ();
    }
}

/* C 4799-4834: returns the `distance' between the colors X and Y.  */

unsafe fn color_distance (x: *mut Emacs_Color, y: *mut Emacs_Color) -> c_int
{
  /* This formula is from a paper titled `Colour metric' by Thiadmer Riemersma.  */
  let r: i64 = (*x).red as i64 - (*y).red as i64;
  let g: i64 = (*x).green as i64 - (*y).green as i64;
  let b: i64 = (*x).blue as i64 - (*y).blue as i64;
  let r_mean: i64 = ((*x).red as i64 + (*y).red as i64) >> 1;

  return (((((2 * 65536 + r_mean) * r * r) >> 16)
	   + 4 * g * g
	   + (((2 * 65536 + 65535 - r_mean) * b * b) >> 16))
	  >> 16) as c_int;
}

DEFUN! ("color-distance", Fcolor_distance, Scolor_distance, 2, 4, 0,
       doc: "Return an integer distance between COLOR1 and COLOR2 on FRAME.\nCOLOR1 and COLOR2 may be either strings containing the color name,\nor lists of the form (RED GREEN BLUE), each in the range 0 to 65535 inclusive.\nIf FRAME is unspecified or nil, the current frame is used.\nIf METRIC is specified, it should be a function that accepts\ntwo lists of the form (RED GREEN BLUE) aforementioned.\nDespite the name, this is not a true distance metric as it does not satisfy\nthe triangle inequality.",
       (color1: Lisp_Object, color2: Lisp_Object, frame: Lisp_Object,
        metric: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let mut cdef1: Emacs_Color = std::mem::zeroed ();
  let mut cdef2: Emacs_Color = std::mem::zeroed ();

  let defined1 = if let Some (hook) = (*FRAME_TERMINAL! (f)).defined_color_hook
    { hook (f as *mut c_void, SSDATA (color1), &mut cdef1, false, true) } else { false };
  if ! (CONSP (color1) && parse_rgb_list (color1, &mut cdef1))
      && ! (STRINGP (color1)
	    && defined1)
    { signal_error (c"Invalid color".as_ptr (), color1); }

  let defined2 = if let Some (hook) = (*FRAME_TERMINAL! (f)).defined_color_hook
    { hook (f as *mut c_void, SSDATA (color2), &mut cdef2, false, true) } else { false };
  if ! (CONSP (color2) && parse_rgb_list (color2, &mut cdef2))
      && ! (STRINGP (color2)
	    && defined2)
    { signal_error (c"Invalid color".as_ptr (), color2); }

  if NILP (metric)
    { return make_fixnum (color_distance (&mut cdef1, &mut cdef2) as isize); }
  else
    { return calln! (metric,
		     list3i (cdef1.red as isize, cdef1.green as isize, cdef1.blue as isize),
		     list3i (cdef2.red as isize, cdef2.green as isize,
			     cdef2.blue as isize)); }
});


/***********************************************************************
			      Face Cache
 ***********************************************************************/

/* C 4884-4896: return a new face cache for frame F.  */

unsafe fn make_face_cache (f: *mut frame) -> *mut face_cache
{
  let c: *mut face_cache = xmalloc (std::mem::size_of::<face_cache> ()) as *mut face_cache;

  (*c).buckets = xzalloc (FACE_CACHE_BUCKETS_SIZE as usize
			  * std::mem::size_of::<*mut face> ()) as *mut *mut face;
  (*c).size = 50;
  (*c).used = 0;
  (*c).faces_by_id = xmalloc ((*c).size as usize
			      * std::mem::size_of::<*mut face> ()) as *mut *mut face;
  (*c).f = f;
  (*c).menu_face_changed_p = if menu_face_changed_default { 1 } else { 0 };
  return c;
}

/* C 4898-4970: the `HAVE_WINDOW_SYSTEM` block (`clear_face_gcs`) is dead.  */

/* C 4934-4970: free all realized faces in face cache C, including basic faces.  */

unsafe fn free_realized_faces (c: *mut face_cache)
{
  if ! c.is_null () && (*c).used != 0
    {
      let mut i: c_int;
      let mut size: usize;
      let f: *mut frame = (*c).f;

      /* We must block input here because we can't process X events
	 safely while only some faces are freed, or when the frame's
	 current matrix still references freed faces.  */
      block_input ();

      for i in 0 .. (*c).used
	{
	  free_realized_face (f, *(*c).faces_by_id.offset (i as isize));
	  *(*c).faces_by_id.offset (i as isize) = std::ptr::null_mut ();
	}

      /* Forget the escape-glyph and glyphless-char faces.  */
      forget_escape_and_glyphless_faces ();
      (*c).used = 0;
      size = FACE_CACHE_BUCKETS_SIZE as usize * std::mem::size_of::<*mut face> ();
      memset ((*c).buckets as *mut c_void, 0, size);

      /* Must do a thorough redisplay the next time.  Mark current
	 matrices as invalid because they will reference faces freed
	 above.  This function is also called when a frame is
	 destroyed.  In this case, the root window of F is nil.  */
      if WINDOWP ((*f).root_window)
	{
	  clear_current_matrices (f);
	  fset_redisplay (f as *mut c_void);
	}

      unblock_input ();
    }
}

/* C 4977-4989: free all realized faces on FRAME or on all frames if FRAME is nil.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_all_realized_faces (frame: Lisp_Object)
{
  if NILP (frame)
    {
      let mut rest: Lisp_Object = Qnil ();
      let mut frame = frame;
      FOR_EACH_FRAME! (rest, frame, {
	free_realized_faces (FRAME_FACE_CACHE (XFRAME (frame)));
      });
      windows_or_buffers_changed = 58;
    }
  else
    { free_realized_faces (FRAME_FACE_CACHE (XFRAME (frame))); }
}

/* C 4994-5004: free face cache C and faces in it, including their X resources.  */

unsafe fn free_face_cache (c: *mut face_cache)
{
  if ! c.is_null ()
    {
      free_realized_faces (c);
      xfree ((*c).buckets as *mut c_void);
      xfree ((*c).faces_by_id as *mut c_void);
      xfree (c as *mut c_void);
    }
}

/* C 5014-5081: cache realized face FACE in face cache C.  HASH is the hash value
   of FACE.  */

unsafe fn cache_face (c: *mut face_cache, face: *mut face, hash: uintptr_t)
{
  let mut i: c_int = (hash % FACE_CACHE_BUCKETS_SIZE as uintptr_t) as c_int;

  (*face).hash = hash;

  if (*face).ascii_face != face
    {
      let mut last: *mut face = *(*c).buckets.offset (i as isize);
      if ! last.is_null ()
	{
	  while ! (*last).next.is_null ()
	    { last = (*last).next; }
	  (*last).next = face;
	  (*face).prev = last;
	  (*face).next = std::ptr::null_mut ();
	}
      else
	{
	  *(*c).buckets.offset (i as isize) = face;
	  (*face).prev = std::ptr::null_mut ();
	  (*face).next = std::ptr::null_mut ();
	}
    }
  else
    {
      (*face).prev = std::ptr::null_mut ();
      (*face).next = *(*c).buckets.offset (i as isize);
      if ! (*face).next.is_null ()
	{ (*(*face).next).prev = face; }
      *(*c).buckets.offset (i as isize) = face;
    }

  /* Find a free slot in C->faces_by_id and use the index of the free
     slot as FACE->id.  */
  i = 0;
  while i < (*c).used
    {
      if *(*c).faces_by_id.offset (i as isize) == std::ptr::null_mut ()
	{ break; }
      i += 1;
    }
  (*face).id = i;

  /* C's `GLYPH_DEBUG` block (the uniqueness check) is dead.  */

  /* Maybe enlarge C->faces_by_id.  */
  if i == (*c).used
    {
      if (*c).used as ptrdiff_t == (*c).size
	{ (*c).faces_by_id = xpalloc ((*c).faces_by_id as *mut c_void, &mut (*c).size, 1,
				      MAX_FACE_ID,
				      std::mem::size_of::<*mut face> () as ptrdiff_t)
	    as *mut *mut face; }
      (*c).used += 1;
    }

  *(*c).faces_by_id.offset (i as isize) = face;
}

/* C 5085-5100: remove face FACE from cache C.  */

unsafe fn uncache_face (c: *mut face_cache, face: *mut face)
{
  let i: c_int = ((*face).hash % FACE_CACHE_BUCKETS_SIZE as uintptr_t) as c_int;

  if ! (*face).prev.is_null ()
    { (*(*face).prev).next = (*face).next; }
  else
    { *(*c).buckets.offset (i as isize) = (*face).next; }

  if ! (*face).next.is_null ()
    { (*(*face).next).prev = (*face).prev; }

  *(*c).faces_by_id.offset ((*face).id as isize) = std::ptr::null_mut ();
  if (*face).id == (*c).used
    { (*c).used -= 1; }
}

/* C 5108-5143: look up a realized face with face attributes ATTR in the face cache
   of frame F.  */

unsafe fn lookup_face (f: *mut frame, attr: *mut Lisp_Object) -> c_int
{
  let cache: *mut face_cache = FRAME_FACE_CACHE (f);
  let mut face: *mut face;

  eassert! (cache != std::ptr::null_mut ());
  check_lface_attrs! (attr);

  /* Look up ATTR in the face cache.  */
  let hash: uintptr_t = lface_hash (attr);
  let i: c_int = (hash % FACE_CACHE_BUCKETS_SIZE as uintptr_t) as c_int;

  face = *(*cache).buckets.offset (i as isize);
  while ! face.is_null ()
    {
      if (*face).ascii_face != face
	{
	  /* There's no more ASCII face.  */
	  face = std::ptr::null_mut ();
	  break;
	}
      if (*face).hash == hash
	  && lface_equal_p ((*face).lface.as_ptr (), attr)
	{ break; }
      face = (*face).next;
    }

  /* If not found, realize a new face.  */
  if face.is_null ()
    { face = realize_face (cache, attr, -1); }

  /* C's `GLYPH_DEBUG` assertion is dead.  */

  return (*face).id;
}

/// C 5145-5181: "Look up a realized face that has the same attributes as BASE_FACE except for the
/// font in the face cache of frame F.  If FONT-OBJECT is not nil, it is an already opened font.
/// If FONT-OBJECT is nil, the face has no font.  Value is the ID of the face found.  If no suitable
/// face is found, realize a new one."  `fontset.c` 1015/1025 and `xdisp.c` 8710 call it through
/// `fontset.h` 46's declaration; Android's library imports the name (status 4.772).  The block used
/// to be recorded as dead (P-08); the port transcribes it now.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn face_for_font (f: *mut frame, font_object: Lisp_Object,
					mut base_face: *mut face) -> c_int
{
  let cache: *mut face_cache = FRAME_FACE_CACHE (f);
  let hash: uintptr_t;
  let i: c_int;
  let mut face: *mut face;

  eassert! (!cache.is_null ());
  base_face = (*base_face).ascii_face;
  hash = lface_hash ((*base_face).lface.as_mut_ptr ());
  i = (hash % FACE_CACHE_BUCKETS_SIZE as uintptr_t) as c_int;

  /* C's `for (face = cache->buckets[i]; face; face = face->next)`: the bucket's first entry, then
     the chain (the `while !face.is_null ()` below is that loop's test).  */
  face = *(*cache).buckets.offset (i as isize);
  while !face.is_null ()
    {
      if (*face).ascii_face == face
	{ face = (*face).next; continue; }
      if (*face).ascii_face == base_face
	  && (*face).font == (if NILP (font_object) { std::ptr::null_mut () }
			      else { XFONT_OBJECT (font_object) })
	  && lface_equal_p ((*face).lface.as_mut_ptr (), (*base_face).lface.as_mut_ptr ())
	{ return (*face).id; }
      face = (*face).next;
    }

  /* If not found, realize a new face.  */
  face = realize_non_ascii_face (f, font_object, base_face);
  return (*face).id;
}


/* C 5189-5225: return the display face-id of the face whose name is SYMBOL.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_named_face (w: *mut window, f: *mut frame,
					    symbol: Lisp_Object,
					    signal_p: bool) -> c_int
{
  let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];
  let mut symbol_attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];
  let mut default_face: *mut face = FACE_FROM_ID_OR_NULL (f, DEFAULT_FACE_ID);

  if default_face.is_null ()
    {
      if ! realize_basic_faces (f)
	{ return -1; }
      default_face = FACE_FROM_ID (f, DEFAULT_FACE_ID);
    }

  if ! get_lface_attributes (w, f, symbol, symbol_attrs.as_mut_ptr (), signal_p,
			     0 as *mut named_merge_point)
    { return -1; }

  memcpy (attrs.as_mut_ptr () as *mut c_void, (*default_face).lface.as_mut_ptr ()
	  as *const c_void, std::mem::size_of_val (&attrs));

  /* Make explicit any attributes whose value is 'reset'.  */
  let mut i: c_int;
  for i in 1 .. LFACE_VECTOR_SIZE
    {
      if EQ (symbol_attrs[i as usize], Qreset ())
	{ symbol_attrs[i as usize] = attrs[i as usize]; }
    }

  merge_face_vectors (w, f, symbol_attrs.as_mut_ptr (), attrs.as_mut_ptr (),
		      0 as *mut named_merge_point);

  return lookup_face (f, attrs.as_mut_ptr ());
}

/* C 5232-5288: return the display face-id of the basic face whose canonical face-id
   is FACE_ID.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_basic_face (w: *mut window, f: *mut frame,
					    face_id: c_int) -> c_int
{
  let mut name: Lisp_Object;
  let mut mapping: Lisp_Object;
  let mut remapped_face_id: c_int;

  if NILP (Vface_remapping_alist)
    { return face_id; }		/* Nothing to do.  */

  match face_id
    {
      DEFAULT_FACE_ID => { name = Qdefault (); }
      MODE_LINE_ACTIVE_FACE_ID => { name = Qmode_line_active (); }
      MODE_LINE_INACTIVE_FACE_ID => { name = Qmode_line_inactive (); }
      HEADER_LINE_ACTIVE_FACE_ID => { name = Qheader_line_active (); }
      HEADER_LINE_INACTIVE_FACE_ID => { name = Qheader_line_inactive (); }
      TAB_LINE_ACTIVE_FACE_ID => { name = Qtab_line_active (); }
      TAB_LINE_INACTIVE_FACE_ID => { name = Qtab_line_inactive (); }
      TAB_BAR_FACE_ID => { name = Qtab_bar (); }
      TOOL_BAR_FACE_ID => { name = Qtool_bar (); }
      FRINGE_FACE_ID => { name = Qfringe (); }
      SCROLL_BAR_FACE_ID => { name = Qscroll_bar (); }
      BORDER_FACE_ID => { name = Qborder (); }
      CURSOR_FACE_ID => { name = Qcursor (); }
      MOUSE_FACE_ID => { name = Qmouse (); }
      MENU_FACE_ID => { name = Qmenu (); }
      WINDOW_DIVIDER_FACE_ID => { name = Qwindow_divider (); }
      VERTICAL_BORDER_FACE_ID => { name = Qvertical_border (); }
      WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID => { name = Qwindow_divider_first_pixel (); }
      WINDOW_DIVIDER_LAST_PIXEL_FACE_ID => { name = Qwindow_divider_last_pixel (); }
      INTERNAL_BORDER_FACE_ID => { name = Qinternal_border (); }
      CHILD_FRAME_BORDER_FACE_ID => { name = Qchild_frame_border (); }
      MARGIN_FACE_ID => { name = Qmargin (); }

      _ =>
	{ emacs_abort (); /* the caller is supposed to pass us a basic face id */ }
    }

  /* Do a quick scan through Vface_remapping_alist, and return immediately
     if there is no remapping for face NAME.  This is just an optimization
     for the very common no-remapping case.  */
  mapping = assq_no_quit (name, Vface_remapping_alist);
  if NILP (mapping)
    {
      let mut face_attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
	[Qnil (); LFACE_VECTOR_SIZE as usize];

      /* If the face inherits from another, we need to realize it,
         because the parent face could be remapped.  */
      if ! get_lface_attributes (w, f, name, face_attrs.as_mut_ptr (), false,
				 0 as *mut named_merge_point)
	  || NILP (face_attrs[LFACE_INHERIT_INDEX as usize])
	  || UNSPECIFIEDP! (face_attrs[LFACE_INHERIT_INDEX as usize])
	{ return face_id; }		/* Give up.  */
    }

  /* If there is a remapping entry, or the face inherits from another,
     lookup the face using NAME, which will handle the remapping too.  */
  remapped_face_id = lookup_named_face (w, f, name, false);
  if remapped_face_id < 0
    { return face_id; }		/* Give up. */

  return remapped_face_id;
}

/* C 5296-5345: return a face for charset ASCII that is like the face with id
   FACE_ID on frame F, but has a font that is STEPS steps smaller.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn smaller_face (f: *mut frame, face_id: c_int,
				       steps: c_int) -> c_int
{
  let _ = (f, steps);
  /* C's `HAVE_WINDOW_SYSTEM` arm is dead; this is C's `#else` body.  */
  return face_id;
}

/* C 5355-5373: return a face for charset ASCII that is like the face with id
   FACE_ID on frame F, but has height HEIGHT.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn face_with_height (f: *mut frame, face_id: c_int,
					   height: c_int) -> c_int
{
  let _ = (f, height);
  /* C's `HAVE_WINDOW_SYSTEM` arm is dead; this is C's `#else` body.  */
  return face_id;
}

/* C 5384-5406: return the face id of the realized face for named face SYMBOL on
   frame F suitable for displaying ASCII characters.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_derived_face (w: *mut window, f: *mut frame,
					      symbol: Lisp_Object, face_id: c_int,
					      signal_p: bool) -> c_int
{
  let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];
  let mut symbol_attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];
  let default_face: *mut face;

  if ! get_lface_attributes (w, f, symbol, symbol_attrs.as_mut_ptr (), signal_p,
			     0 as *mut named_merge_point)
    { return -1; }

  default_face = FACE_FROM_ID (f, face_id);
  memcpy (attrs.as_mut_ptr () as *mut c_void, (*default_face).lface.as_mut_ptr ()
	  as *const c_void, std::mem::size_of_val (&attrs));

  /* Make explicit any attributes whose value is 'reset'.  */
  let mut i: c_int;
  for i in 1 .. LFACE_VECTOR_SIZE
    {
      if EQ (symbol_attrs[i as usize], Qreset ())
	{ symbol_attrs[i as usize] = attrs[i as usize]; }
    }

  merge_face_vectors (w, f, symbol_attrs.as_mut_ptr (), attrs.as_mut_ptr (),
		      0 as *mut named_merge_point);
  return lookup_face (f, attrs.as_mut_ptr ());
}

DEFUN! ("face-attributes-as-vector", Fface_attributes_as_vector,
       Sface_attributes_as_vector, 1, 1, 0,
       doc: "Return a vector of face attributes corresponding to PLIST.",
       (plist: Lisp_Object),
       {
  let lface: Lisp_Object = make_vector (LFACE_VECTOR_SIZE as ptrdiff_t, Qunspecified ());
  merge_face_ref (std::ptr::null_mut (), XFRAME (selected_frame),
		  plist, (*XVECTOR (lface)).contents.as_mut_ptr (),
		  true, std::ptr::null_mut (), 0);
  return lface;
});


/***********************************************************************
				Face capability testing
 ***********************************************************************/

/* C 5431: if the distance (as returned by color_distance) between two colors is
   less than this, then they are considered the same.  */
pub const TTY_SAME_COLOR_THRESHOLD: c_int = 10000;

/* dispextern.h 3532-3542: the `TTY_CAP_*` bits (the mirror stops before them).  */
pub const TTY_CAP_INVERSE: c_uint = 1 << 1;
pub const TTY_CAP_UNDERLINE: c_uint = 1 << 2;
pub const TTY_CAP_BOLD: c_uint = 1 << 3;
pub const TTY_CAP_DIM: c_uint = 1 << 4;
pub const TTY_CAP_ITALIC: c_uint = 1 << 5;
pub const TTY_CAP_STRIKE_THROUGH: c_uint = 1 << 6;
pub const TTY_CAP_UNDERLINE_STYLED: c_uint = 1 << 7;
pub const TTY_CAP_OVERLINE: c_uint = 1 << 8;

/// C: dispextern.h 4012: `bool is_tty_frame (struct frame *f);` (dispnew.c), and
/// dispextern.h 3997: `bool tty_capable_p (struct tty_display_info *, unsigned);`
/// (term.c) — neither is in a mirror this unit can use.
unsafe extern "C" {
  fn is_tty_frame (f: *mut frame) -> bool;
  fn tty_capable_p (tty: *mut tty_display_info, caps: c_uint) -> bool;
  /// C: font.h 887: `void font_update_sort_order (int *order);`
  fn font_update_sort_order (order: *mut c_int);
}

/* C 5433-5574: the `HAVE_WINDOW_SYSTEM` block (`gui_supports_face_attributes_p`) is dead.  */

/* C 5576-5761: return true if all the face attributes in ATTRS are supported
   on the tty frame F.  */

unsafe fn tty_supports_face_attributes_p (f: *mut frame, attrs: *mut Lisp_Object,
					   def_face: *mut face) -> bool
{
  let mut weight: c_int;
  let mut slant: c_int;
  let mut val: Lisp_Object;
  let mut fg: Lisp_Object;
  let mut bg: Lisp_Object;
  let mut fg_tty_color: Emacs_Color = std::mem::zeroed ();
  let mut fg_std_color: Emacs_Color = std::mem::zeroed ();
  let mut bg_tty_color: Emacs_Color = std::mem::zeroed ();
  let mut bg_std_color: Emacs_Color = std::mem::zeroed ();
  let mut test_caps: c_uint = 0;
  let def_attrs: *mut Lisp_Object = (*def_face).lface.as_mut_ptr ();

  /* First check some easy-to-check stuff; ttys support none of the
     following attributes, so we can just return false if any are requested
     (even if `nominal' values are specified, we should still return false,
     as that will be the same value that the default face uses).  We
     consider :slant unsupportable on ttys, even though the face code
     actually `fakes' them using a dim attribute if possible.  This is
     because the faked result is too different from what the face
     specifies.  */
  if ! UNSPECIFIEDP! (*attrs.offset (LFACE_FAMILY_INDEX as isize))
      || ! UNSPECIFIEDP! (*attrs.offset (LFACE_FOUNDRY_INDEX as isize))
      || ! UNSPECIFIEDP! (*attrs.offset (LFACE_STIPPLE_INDEX as isize))
      || ! UNSPECIFIEDP! (*attrs.offset (LFACE_HEIGHT_INDEX as isize))
      || ! UNSPECIFIEDP! (*attrs.offset (LFACE_SWIDTH_INDEX as isize))
      || ! UNSPECIFIEDP! (*attrs.offset (LFACE_BOX_INDEX as isize))
    { return false; }

  /* Test for terminal `capabilities' (non-color character attributes).  */

  /* font weight (bold/dim) */
  val = *attrs.offset (LFACE_WEIGHT_INDEX as isize);
  if ! UNSPECIFIEDP! (val)
      && { weight = FONT_WEIGHT_NAME_NUMERIC! (val); weight >= 0 }
    {
      let def_weight: c_int =
	FONT_WEIGHT_NAME_NUMERIC! (*def_attrs.offset (LFACE_WEIGHT_INDEX as isize));

      if weight > 100
	{
	  if def_weight > 100
	    { return false; }	/* same as default */
	  test_caps = TTY_CAP_BOLD;
	}
      else if weight < 100
	{
	  if def_weight < 100
	    { return false; }	/* same as default */
	  test_caps = TTY_CAP_DIM;
	}
      else if def_weight == 100
	{ return false; }	/* same as default */
    }

  /* font slant */
  val = *attrs.offset (LFACE_SLANT_INDEX as isize);
  if ! UNSPECIFIEDP! (val)
      && { slant = FONT_SLANT_NAME_NUMERIC! (val); slant >= 0 }
    {
      let def_slant: c_int =
	FONT_SLANT_NAME_NUMERIC! (*def_attrs.offset (LFACE_SLANT_INDEX as isize));
      if slant == 100 || slant == def_slant
	{ return false; }	/* same as default */
      else
	{ test_caps |= TTY_CAP_ITALIC; }
    }

  /* underlining */
  val = *attrs.offset (LFACE_UNDERLINE_INDEX as isize);
  if ! UNSPECIFIEDP! (val)
    {
      if STRINGP (val)
	{ test_caps |= TTY_CAP_UNDERLINE_STYLED; }
      else if EQ (CAR_SAFE (val), QCstyle ())
	{
	  if ! (EQ (CAR_SAFE (CDR_SAFE (val)), Qline ())
		|| EQ (CAR_SAFE (CDR_SAFE (val)), Qdouble_line ())
		|| EQ (CAR_SAFE (CDR_SAFE (val)), Qwave ())
		|| EQ (CAR_SAFE (CDR_SAFE (val)), Qdots ())
		|| EQ (CAR_SAFE (CDR_SAFE (val)), Qdashes ()))
	    { return false; } /* Face uses an unsupported underline style.  */

	  test_caps |= TTY_CAP_UNDERLINE_STYLED;
	}
      else if face_attr_equal_p (val, *def_attrs.offset (LFACE_UNDERLINE_INDEX as isize))
	{ return false; }	/* same as default */
      else
	{ test_caps |= TTY_CAP_UNDERLINE; }
    }

  /* inverse video */
  val = *attrs.offset (LFACE_INVERSE_INDEX as isize);
  if ! UNSPECIFIEDP! (val)
    {
      if face_attr_equal_p (val, *def_attrs.offset (LFACE_INVERSE_INDEX as isize))
	{ return false; }	/* same as default */
      else
	{ test_caps |= TTY_CAP_INVERSE; }
    }

  /* strike through */
  val = *attrs.offset (LFACE_STRIKE_THROUGH_INDEX as isize);
  if ! UNSPECIFIEDP! (val)
    {
      if face_attr_equal_p (val, *def_attrs.offset (LFACE_STRIKE_THROUGH_INDEX as isize))
	{ return false; }	/* same as default */
      else
	{ test_caps |= TTY_CAP_STRIKE_THROUGH; }
    }

  /* overline */
  val = *attrs.offset (LFACE_OVERLINE_INDEX as isize);
  if ! UNSPECIFIEDP! (val)
    {
      if face_attr_equal_p (val, *def_attrs.offset (LFACE_OVERLINE_INDEX as isize))
	{ return false; }	/* same as default */
      else
	{ test_caps |= TTY_CAP_OVERLINE; }
    }

  /* Color testing.  */

  /* Check if foreground color is close enough.  */
  fg = *attrs.offset (LFACE_FOREGROUND_INDEX as isize);
  if STRINGP (fg)
    {
      let def_fg: Lisp_Object = *def_attrs.offset (LFACE_FOREGROUND_INDEX as isize);

      if face_attr_equal_p (fg, def_fg)
	{ return false; }	/* same as default */
      else if ! tty_lookup_color (f, fg, &mut fg_tty_color, &mut fg_std_color)
	{ return false; }	/* not a valid color */
      else if color_distance (&mut fg_tty_color, &mut fg_std_color)
	       > TTY_SAME_COLOR_THRESHOLD
	{ return false; }	/* displayed color is too different */
      else
	/* Make sure the color is really different from the default.  */
	{
	  let mut def_fg_color: Emacs_Color = std::mem::zeroed ();
	  if tty_lookup_color (f, def_fg, &mut def_fg_color, std::ptr::null_mut ())
	      && (color_distance (&mut fg_tty_color, &mut def_fg_color)
		  <= TTY_SAME_COLOR_THRESHOLD)
	    { return false; }
	}
    }

  /* Check if background color is close enough.  */
  bg = *attrs.offset (LFACE_BACKGROUND_INDEX as isize);
  if STRINGP (bg)
    {
      let def_bg: Lisp_Object = *def_attrs.offset (LFACE_BACKGROUND_INDEX as isize);

      if face_attr_equal_p (bg, def_bg)
	{ return false; }	/* same as default */
      else if ! tty_lookup_color (f, bg, &mut bg_tty_color, &mut bg_std_color)
	{ return false; }	/* not a valid color */
      else if color_distance (&mut bg_tty_color, &mut bg_std_color)
	       > TTY_SAME_COLOR_THRESHOLD
	{ return false; }	/* displayed color is too different */
      else
	/* Make sure the color is really different from the default.  */
	{
	  let mut def_bg_color: Emacs_Color = std::mem::zeroed ();
	  if tty_lookup_color (f, def_bg, &mut def_bg_color, std::ptr::null_mut ())
	      && (color_distance (&mut bg_tty_color, &mut def_bg_color)
		  <= TTY_SAME_COLOR_THRESHOLD)
	    { return false; }
	}
    }

  /* If both foreground and background are requested, see if the
     distance between them is OK.  We just check to see if the distance
     between the tty's foreground and background is close enough to the
     distance between the standard foreground and background.  */
  if STRINGP (fg) && STRINGP (bg)
    {
      let delta_delta: c_int
	= color_distance (&mut fg_std_color, &mut bg_std_color)
	   - color_distance (&mut fg_tty_color, &mut bg_tty_color);
      if delta_delta > TTY_SAME_COLOR_THRESHOLD
	  || delta_delta < -TTY_SAME_COLOR_THRESHOLD
	{ return false; }
    }


  /* See if the capabilities we selected above are supported, with the
     given colors.  */
  return tty_capable_p (FRAME_TTY! (f), test_caps);
}

DEFUN! ("display-supports-face-attributes-p",
       Fdisplay_supports_face_attributes_p, Sdisplay_supports_face_attributes_p,
       1, 2, 0,
       doc: "Return non-nil if all the face attributes in ATTRIBUTES are supported.\nThe optional argument DISPLAY can be a display name, a frame, or\nnil (meaning the selected frame's display).\n\nFor instance, to check whether the display supports underlining:\n\n  (display-supports-face-attributes-p \\='(:underline t))\n\nThe definition of `supported' is somewhat heuristic, but basically means\nthat a face containing all the attributes in ATTRIBUTES, when merged\nwith the default face for display, can be represented in a way that's\n\n (1) different in appearance from the default face, and\n (2) `close in spirit' to what the attributes specify, if not exact.\n\nPoint (2) implies that a `:weight black' attribute will be satisfied by\nany display that can display bold, and a `:foreground \"yellow\"' as long\nas it can display a yellowish color, but `:slant italic' will _not_ be\nsatisfied by the tty display code's automatic substitution of a `dim'\nface for italic.",
       (attributes: Lisp_Object, display: Lisp_Object),
       {
  let mut supports: bool = false;
  let mut i: c_int;
  let mut frame: Lisp_Object;
  let f: *mut frame;
  let mut def_face: *mut face;
  let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];

  if noninteractive || ! initialized
    /* We may not be able to access low-level face information in batch
       mode, or before being dumped, and this function is not going to
       be very useful in those cases anyway, so just give up.  */
    { return Qnil (); }

  if NILP (display)
    { frame = selected_frame; }
  else if FRAMEP (display)
    { frame = display; }
  else
    {
      /* Find any frame on DISPLAY.  */
      let mut tail: Lisp_Object;

      frame = Qnil ();
      FOR_EACH_FRAME! (tail, frame, {
	if ! NILP (Fequal (Fcdr (Fassq (Qdisplay (),
					 (*XFRAME (frame)).param_alist)),
			   display))
	  { break; }
      });
    }

  CHECK_LIVE_FRAME! (frame);
  f = XFRAME (frame);

  for i in 0 .. LFACE_VECTOR_SIZE
    { attrs[i as usize] = Qunspecified (); }
  merge_face_ref (std::ptr::null_mut (), f, attributes, attrs.as_mut_ptr (), true,
		  std::ptr::null_mut (), 0);

  def_face = FACE_FROM_ID_OR_NULL (f, DEFAULT_FACE_ID);
  if def_face.is_null ()
    {
      if ! realize_basic_faces (f)
	{ error (c"Cannot realize default face".as_ptr ()); }
      def_face = FACE_FROM_ID (f, DEFAULT_FACE_ID);
    }

  /* Dispatch to the appropriate handler.  */
  if is_tty_frame (f)
    { supports = tty_supports_face_attributes_p (f, attrs.as_mut_ptr (), def_face); }
  /* C's `HAVE_WINDOW_SYSTEM` arm (`gui_supports_face_attributes_p`) is dead.  */

  return if supports { Qt () } else { Qnil () };
});


/***********************************************************************
			    Font selection
 ***********************************************************************/

DEFUN! ("internal-set-font-selection-order",
       Finternal_set_font_selection_order,
       Sinternal_set_font_selection_order, 1, 1, 0,
       doc: "Set font selection order for face font selection to ORDER.\nORDER must be a list of length 4 containing the symbols `:width',\n`:height', `:weight', and `:slant'.  Face attributes appearing\nfirst in ORDER are matched first, e.g. if `:height' appears before\n`:weight' in ORDER, font selection first tries to find a font with\na suitable height, and then tries to match the font weight.\nValue is ORDER.",
       (order: Lisp_Object),
       {
  let mut list: Lisp_Object;
  let mut i: c_int;
  /* C: `int indices[countof (font_sort_order)];` — four, the length of the static
     array below (Rust needs the literal for its array length).  */
  let mut indices: [c_int; 4] = [0; 4];

  CHECK_LIST (order);
  memset (indices.as_mut_ptr () as *mut c_void, 0, std::mem::size_of_val (&indices));
  i = 0;

  list = order;
  while CONSP (list) && (i as usize) < countof! (indices)
    {
      let attr: Lisp_Object = XCAR (list);
      let mut xlfd: c_int;

      if EQ (attr, QCwidth ())
	{ xlfd = XLFD_SWIDTH; }
      else if EQ (attr, QCheight ())
	{ xlfd = XLFD_POINT_SIZE; }
      else if EQ (attr, QCweight ())
	{ xlfd = XLFD_WEIGHT; }
      else if EQ (attr, QCslant ())
	{ xlfd = XLFD_SLANT; }
      else
	{ break; }

      if indices[i as usize] != 0
	{ break; }
      indices[i as usize] = xlfd;
      list = XCDR (list);
      i += 1;
    }

  if ! NILP (list) || (i as usize) != countof! (indices)
    { signal_error (c"Invalid font sort order".as_ptr (), order); }
  for i in 0 .. countof! (font_sort_order)
    {
      if indices[i as usize] == 0
	{ signal_error (c"Invalid font sort order".as_ptr (), order); }
    }

  if memcmp (indices.as_mut_ptr () as *const c_void,
	     std::ptr::addr_of! (font_sort_order) as *const c_void,
	     std::mem::size_of_val (&indices)) != 0
    {
      memcpy (std::ptr::addr_of_mut! (font_sort_order) as *mut c_void,
	      indices.as_mut_ptr () as *const c_void,
	      std::mem::size_of_val (&font_sort_order));
      free_all_realized_faces (Qnil ());
    }

  font_update_sort_order (std::ptr::addr_of_mut! (font_sort_order) as *mut c_int);

  return Qnil ();
});

DEFUN! ("internal-set-alternative-font-family-alist",
       Finternal_set_alternative_font_family_alist,
       Sinternal_set_alternative_font_family_alist, 1, 1, 0,
       doc: "Define alternative font families to try in face font selection.\nALIST is an alist of (FAMILY ALTERNATIVE1 ALTERNATIVE2 ...) entries.\nEach ALTERNATIVE is tried in order if no fonts of font family FAMILY can\nbe found.  Value is ALIST.",
       (alist: Lisp_Object),
       {
  let mut entry: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut tail2: Lisp_Object;
  let mut alist = alist;

  CHECK_LIST (alist);
  alist = Fcopy_sequence (alist);
  tail = alist;
  while CONSP (tail)
    {
      entry = XCAR (tail);
      CHECK_LIST (entry);
      entry = Fcopy_sequence (entry);
      XSETCAR (tail, entry);
      tail2 = entry;
      while CONSP (tail2)
	{
	  XSETCAR (tail2, Fintern (XCAR (tail2), Qnil ()));
	  tail2 = XCDR (tail2);
	}
      tail = XCDR (tail);
    }

  Vface_alternative_font_family_alist = alist;
  free_all_realized_faces (Qnil ());
  return alist;
});

DEFUN! ("internal-set-alternative-font-registry-alist",
       Finternal_set_alternative_font_registry_alist,
       Sinternal_set_alternative_font_registry_alist, 1, 1, 0,
       doc: "Define alternative font registries to try in face font selection.\nALIST is an alist of (REGISTRY ALTERNATIVE1 ALTERNATIVE2 ...) entries.\nEach ALTERNATIVE is tried in order if no fonts of font registry REGISTRY can\nbe found.  Value is ALIST.",
       (alist: Lisp_Object),
       {
  let mut entry: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut tail2: Lisp_Object;
  let mut alist = alist;

  CHECK_LIST (alist);
  alist = Fcopy_sequence (alist);
  tail = alist;
  while CONSP (tail)
    {
      entry = XCAR (tail);
      CHECK_LIST (entry);
      entry = Fcopy_sequence (entry);
      XSETCAR (tail, entry);
      tail2 = entry;
      while CONSP (tail2)
	{
	  XSETCAR (tail2, Fdowncase (XCAR (tail2)));
	  tail2 = XCDR (tail2);
	}
      tail = XCDR (tail);
    }

  Vface_alternative_font_registry_alist = alist;
  free_all_realized_faces (Qnil ());
  return alist;
});


/***********************************************************************
			   Face Realization
 ***********************************************************************/

/* frame.h 1173-1174: `SET_FRAME_GARBAGED` (C's three effects in order).  The mirror's copy
   (`include/frame.rs` 947) cannot expand here - it passes `*mut frame` where this tree
   declares `fset_redisplay (*mut c_void)` (E0308; the cast below is the working spelling) -
   so the unit keeps its own, reported to the mirror's owner.  */

/// C: lisp.h 4274: `Lisp_Object assoc_no_quit (Lisp_Object, Lisp_Object);` (fns.c).
unsafe extern "C" {
  fn assoc_no_quit (key: Lisp_Object, list: Lisp_Object) -> Lisp_Object;
}

/* C 5998-6054: realize basic faces on frame F.  */

unsafe fn realize_basic_faces (f: *mut frame) -> bool
{
  let mut success_p: bool = false;

  /* Block input here so that we won't be surprised by an X expose
     event, for instance, without having the faces set up.  */
  block_input ();

  if realize_default_face (f)
    {
      /* Basic faces must be realized disregarding face-remapping-alist,
         since otherwise face-remapping might affect the basic faces in the
         face cache, if this function happens to be invoked with current
	 buffer set to a buffer with a non-nil face-remapping-alist.  */
      let count: specpdl_ref = SPECPDL_INDEX ();
      specbind (Qface_remapping_alist (), Qnil ());
      realize_named_face (f, Qmode_line_active (), MODE_LINE_ACTIVE_FACE_ID);
      realize_named_face (f, Qmode_line_inactive (), MODE_LINE_INACTIVE_FACE_ID);
      realize_named_face (f, Qtool_bar (), TOOL_BAR_FACE_ID);
      realize_named_face (f, Qfringe (), FRINGE_FACE_ID);
      realize_named_face (f, Qheader_line_active (), HEADER_LINE_ACTIVE_FACE_ID);
      realize_named_face (f, Qheader_line_inactive (), HEADER_LINE_INACTIVE_FACE_ID);
      realize_named_face (f, Qscroll_bar (), SCROLL_BAR_FACE_ID);
      realize_named_face (f, Qborder (), BORDER_FACE_ID);
      realize_named_face (f, Qcursor (), CURSOR_FACE_ID);
      realize_named_face (f, Qmouse (), MOUSE_FACE_ID);
      realize_named_face (f, Qmenu (), MENU_FACE_ID);
      realize_named_face (f, Qvertical_border (), VERTICAL_BORDER_FACE_ID);
      realize_named_face (f, Qwindow_divider (), WINDOW_DIVIDER_FACE_ID);
      realize_named_face (f, Qwindow_divider_first_pixel (),
			  WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID);
      realize_named_face (f, Qwindow_divider_last_pixel (),
			  WINDOW_DIVIDER_LAST_PIXEL_FACE_ID);
      realize_named_face (f, Qinternal_border (), INTERNAL_BORDER_FACE_ID);
      realize_named_face (f, Qchild_frame_border (), CHILD_FRAME_BORDER_FACE_ID);
      realize_named_face (f, Qtab_bar (), TAB_BAR_FACE_ID);
      realize_named_face (f, Qtab_line_active (), TAB_LINE_ACTIVE_FACE_ID);
      realize_named_face (f, Qtab_line_inactive (), TAB_LINE_INACTIVE_FACE_ID);
      realize_named_face (f, Qmargin (), MARGIN_FACE_ID);
      unbind_to (count, Qnil ());

      /* Reflect changes in the `menu' face in menu bars.  */
      if (*FRAME_FACE_CACHE (f)).menu_face_changed_p != 0
	{
	  (*FRAME_FACE_CACHE (f)).menu_face_changed_p = 0;
	  /* C's `USE_X_TOOLKIT` arm (`x_update_menu_appearance`) is dead.  */
	}

      success_p = true;
    }

  unblock_input ();
  return success_p;
}

/* C 6062-6185: realize the default face on frame F.  */

unsafe fn realize_default_face (f: *mut frame) -> bool
{
  let c: *mut face_cache = FRAME_FACE_CACHE (f);
  let mut lface: Lisp_Object;
  let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];

  /* If the `default' face is not yet known, create it.  */
  lface = lface_from_face_name (f, Qdefault (), false);
  if NILP (lface)
    {
       let mut frame: Lisp_Object = Qnil ();
       XSETFRAME! (frame, f);
       lface = Finternal_make_lisp_face (Qdefault (), frame);
    }

  /* C 6081-6091: the `HAVE_WINDOW_SYSTEM` arm.  The frozen darwin configuration does not compile
     it (HAVE_WINDOW_SYSTEM is undefined there); Android's does, and without it the default face
     keeps family/foundry/height/weight/slant/swidth/fontset unspecified - the first frame then
     trips `lface_fully_specified_p` (device run, status 4.854).  */
  #[cfg (feature = "have_window_system")]
  {
    if FRAME_WINDOW_P (f)
      {
	let mut font_object: Lisp_Object = Qnil ();
	XSETFONT! (font_object, FRAME_FONT! (f));
	set_lface_from_font (f, lface, font_object, default_face_done_p! (f));
	ASET (lface, LFACE_FONTSET_INDEX as isize, fontset_name (FRAME_FONTSET! (f)));
	default_face_done_p! (f, true);
      }
  }

  if ! FRAME_WINDOW_P (f)
    {
      ASET (lface, LFACE_FAMILY_INDEX as isize, build_string (c"default".as_ptr ()));
      ASET (lface, LFACE_FOUNDRY_INDEX as isize, LFACE_FAMILY! (lface));
      ASET (lface, LFACE_SWIDTH_INDEX as isize, Qnormal ());
      ASET (lface, LFACE_HEIGHT_INDEX as isize, make_fixnum (1));
      if UNSPECIFIEDP! (LFACE_WEIGHT! (lface))
	{ ASET (lface, LFACE_WEIGHT_INDEX as isize, Qnormal ()); }
      if UNSPECIFIEDP! (LFACE_SLANT! (lface))
	{ ASET (lface, LFACE_SLANT_INDEX as isize, Qnormal ()); }
      if UNSPECIFIEDP! (LFACE_FONTSET! (lface))
	{ ASET (lface, LFACE_FONTSET_INDEX as isize, Qnil ()); }
    }

  if UNSPECIFIEDP! (LFACE_EXTEND! (lface))
    { ASET (lface, LFACE_EXTEND_INDEX as isize, Qnil ()); }

  if UNSPECIFIEDP! (LFACE_UNDERLINE! (lface))
    { ASET (lface, LFACE_UNDERLINE_INDEX as isize, Qnil ()); }

  if UNSPECIFIEDP! (LFACE_OVERLINE! (lface))
    { ASET (lface, LFACE_OVERLINE_INDEX as isize, Qnil ()); }

  if UNSPECIFIEDP! (LFACE_STRIKE_THROUGH! (lface))
    { ASET (lface, LFACE_STRIKE_THROUGH_INDEX as isize, Qnil ()); }

  if UNSPECIFIEDP! (LFACE_BOX! (lface))
    { ASET (lface, LFACE_BOX_INDEX as isize, Qnil ()); }

  if UNSPECIFIEDP! (LFACE_INVERSE! (lface))
    { ASET (lface, LFACE_INVERSE_INDEX as isize, Qnil ()); }

  if UNSPECIFIEDP! (LFACE_FOREGROUND! (lface))
    {
      /* This function is called so early that colors are not yet
	 set in the frame parameter list.  */
      let color: Lisp_Object = Fassq (Qforeground_color (), (*f).param_alist);

      if CONSP (color) && STRINGP (XCDR (color))
	{ ASET (lface, LFACE_FOREGROUND_INDEX as isize, XCDR (color)); }
      else if FRAME_WINDOW_P (f)
	{ return false; }
      else if FRAME_INITIAL_P (f) || is_tty_frame (f)
	{ ASET (lface, LFACE_FOREGROUND_INDEX as isize, build_string (unspecified_fg.as_ptr ())); }
      else
	{ emacs_abort (); }
    }

  if UNSPECIFIEDP! (LFACE_BACKGROUND! (lface))
    {
      /* This function is called so early that colors are not yet
	 set in the frame parameter list.  */
      let color: Lisp_Object = Fassq (Qbackground_color (), (*f).param_alist);
      if CONSP (color) && STRINGP (XCDR (color))
	{ ASET (lface, LFACE_BACKGROUND_INDEX as isize, XCDR (color)); }
      else if FRAME_WINDOW_P (f)
	{ return false; }
      else if FRAME_INITIAL_P (f) || is_tty_frame (f)
	{ ASET (lface, LFACE_BACKGROUND_INDEX as isize, build_string (unspecified_bg.as_ptr ())); }
      else
	{ emacs_abort (); }
    }

  if UNSPECIFIEDP! (LFACE_STIPPLE! (lface))
    { ASET (lface, LFACE_STIPPLE_INDEX as isize, Qnil ()); }

  /* Realize the face; it must be fully-specified now.  */
  eassert! (lface_fully_specified_p ((*XVECTOR (lface)).contents.as_mut_ptr ()));
  check_lface! (lface);
  memcpy (attrs.as_mut_ptr () as *mut c_void, xvector_contents (lface) as *const c_void,
	  std::mem::size_of_val (&attrs));
  /* In some cases, realize_face below can call Lisp, which could
     trigger redisplay.  But we are in the process of realizing
     the default face, and therefore are not ready to do display.  */
  let count: specpdl_ref = SPECPDL_INDEX ();
  specbind (Qinhibit_redisplay (), Qt ());
  let face: *mut face = realize_face (c, attrs.as_mut_ptr (), DEFAULT_FACE_ID);
  unbind_to (count, Qnil ());

  let _ = face;
  /* C's `HAVE_WINDOW_SYSTEM` arm (`gui_set_font`) is dead.  */
  return true;
}

/* C 6193-6227: realize basic faces other than the default face in face cache C.  */

unsafe fn realize_named_face (f: *mut frame, symbol: Lisp_Object, id: c_int)
{
  let c: *mut face_cache = FRAME_FACE_CACHE (f);
  let mut lface: Lisp_Object = lface_from_face_name (f, symbol, false);
  let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];
  let mut symbol_attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];

  /* The default face must exist and be fully specified.  */
  get_lface_attributes_no_remap (f, Qdefault (), attrs.as_mut_ptr (), true);
  check_lface_attrs! (attrs.as_mut_ptr ());
  eassert! (lface_fully_specified_p (attrs.as_ptr ()));

  /* If SYMBOL isn't know as a face, create it.  */
  if NILP (lface)
    {
      let mut frame: Lisp_Object = Qnil ();
      XSETFRAME! (frame, f);
      lface = Finternal_make_lisp_face (symbol, frame);
    }


  get_lface_attributes_no_remap (f, symbol, symbol_attrs.as_mut_ptr (), true);

  /* Handle the 'reset' pseudo-value of any attribute by replacing it
     with the corresponding value of the default face.  */
  let mut i: c_int;
  for i in 1 .. LFACE_VECTOR_SIZE
    {
      if EQ (symbol_attrs[i as usize], Qreset ())
	{ symbol_attrs[i as usize] = attrs[i as usize]; }
    }
  /* Merge SYMBOL's face with the default face.  */
  merge_face_vectors (std::ptr::null_mut (), f, symbol_attrs.as_mut_ptr (),
		      attrs.as_mut_ptr (), 0 as *mut named_merge_point);

  /* Realize the face.  */
  realize_face (c, attrs.as_mut_ptr (), id);
}

/* C 6235-6270: realize the fully-specified face with attributes ATTRS in face
   cache CACHE for ASCII characters.  */

unsafe fn realize_face (cache: *mut face_cache, attrs: *mut Lisp_Object,
			former_face_id: c_int) -> *mut face
{
  let face: *mut face;

  /* LFACE must be fully specified.  */
  eassert! (cache != std::ptr::null_mut ());
  check_lface_attrs! (attrs);

  if former_face_id >= 0 && (*cache).used > former_face_id
    {
      /* Remove the former face.  */
      let former_face: *mut face = *(*cache).faces_by_id.offset (former_face_id as isize);
      if ! former_face.is_null ()
	{ uncache_face (cache, former_face); }
      free_realized_face ((*cache).f, former_face);
      SET_FRAME_GARBAGED! ((*cache).f);
    }

  if FRAME_WINDOW_P ((*cache).f)
    { face = realize_gui_face (cache, attrs); }
  else if is_tty_frame ((*cache).f)
    { face = realize_tty_face (cache, attrs); }
  else if FRAME_INITIAL_P ((*cache).f)
    {
      /* Create a dummy face. */
      face = make_realized_face (attrs);
    }
  else
    { emacs_abort (); }

  /* Insert the new face.  */
  cache_face (cache, face, lface_hash (attrs));
  return face;
}

/* C 6331-6333: `realize_gui_face`'s declaration and its `face = NULL` initializer are
   live text (the body below them is inside the dead `HAVE_WINDOW_SYSTEM` arm).  */

unsafe fn realize_gui_face (cache: *mut face_cache, attrs: *mut Lisp_Object) -> *mut face
{
  #[cfg (not (feature = "have_window_system"))]
  {
    /* C 6329-6332 and 6662-6663: the declaration and `return face;` are the frozen
       configuration's live text.  */
    let _ = (cache, attrs);
    return std::ptr::null_mut ();
  }
  #[cfg (feature = "have_window_system")]
  {
  let mut face: *mut face = std::ptr::null_mut ();
  let default_face: *mut face;
  let f: *mut frame;
  let mut stipple: Lisp_Object;
  let mut underline: Lisp_Object;
  let overline: Lisp_Object;
  let strike_through: Lisp_Object;
  let mut box_: Lisp_Object;

  eassert! (FRAME_WINDOW_P ((*cache).f));

  /* Allocate a new realized face.  */
  face = make_realized_face (attrs);
  (*face).ascii_face = face;

  f = (*cache).f;

  /* Determine the font to use.  Most of the time, the font will be
     the same as the font of the default face, so try that first.  */
  default_face = FACE_FROM_ID_OR_NULL (f, DEFAULT_FACE_ID);
  if ! default_face.is_null ()
      && lface_same_font_attributes_p ((*default_face).lface.as_ptr (), attrs)
    {
      (*face).font = (*default_face).font;
      (*face).fontset
	= make_fontset_for_ascii_face (f as *mut c_void, (*default_face).fontset, face as *mut c_void);
    }
  else
    {
      /* If the face attribute ATTRS specifies a fontset, use it as
	 the base of a new realized fontset.  Otherwise, use the same
	 base fontset as of the default face.  */
      let mut fontset: c_int = face_fontset (attrs);

      if fontset == -1
	{
	  if ! default_face.is_null ()
	    { fontset = (*default_face).fontset; }
	  if fontset == -1
	    { emacs_abort (); }
	}
      if ! FONT_OBJECT_P (*attrs.offset (LFACE_FONT_INDEX as isize))
	{
	  let mut spec: Lisp_Object
	    = copy_font_spec (*attrs.offset (LFACE_FONT_INDEX as isize));

	  if EQ (Vface_font_lax_matched_attributes, Qt ())
	    {
	      ASET (spec, FONT_WEIGHT_INDEX as isize, Qnil ());
	      ASET (spec, FONT_SLANT_INDEX as isize, Qnil ());
	      ASET (spec, FONT_WIDTH_INDEX as isize, Qnil ());
	    }
	  else if ! NILP (Vface_font_lax_matched_attributes)
	    {
	      font_maybe_unset_attribute (spec, FONT_WEIGHT_INDEX, QCweight ());
	      font_maybe_unset_attribute (spec, FONT_SLANT_INDEX, QCslant ());
	      font_maybe_unset_attribute (spec, FONT_WIDTH_INDEX, QCwidth ());
	      font_maybe_unset_attribute (spec, FONT_FAMILY_INDEX, QCfamily ());
	      font_maybe_unset_attribute (spec, FONT_FOUNDRY_INDEX, QCfoundry ());
	      font_maybe_unset_attribute (spec, FONT_REGISTRY_INDEX, QCregistry ());
	      font_maybe_unset_attribute (spec, FONT_ADSTYLE_INDEX, QCadstyle ());
	      font_maybe_unset_attribute (spec, FONT_SIZE_INDEX, QCsize ());
	      font_maybe_unset_attribute (spec, FONT_DPI_INDEX, QCdpi ());
	      font_maybe_unset_attribute (spec, FONT_SPACING_INDEX, QCspacing ());
	      font_maybe_unset_attribute (spec, FONT_AVGWIDTH_INDEX, QCavgwidth ());
	    }

	  *attrs.offset (LFACE_FONT_INDEX as isize) = font_load_for_lface (f, attrs, spec);
	}
      if FONT_OBJECT_P (*attrs.offset (LFACE_FONT_INDEX as isize))
	{
	  (*face).font = XFONT_OBJECT (*attrs.offset (LFACE_FONT_INDEX as isize));
	  (*face).fontset = make_fontset_for_ascii_face (f as *mut c_void, fontset, face as *mut c_void);
	}
      else
	{
	  (*face).font = std::ptr::null_mut ();
	  (*face).fontset = -1;
	}
    }

  if ! (*face).font.is_null ()
      && FONT_WEIGHT_NAME_NUMERIC! (*attrs.offset (LFACE_WEIGHT_INDEX as isize)) > 100
      && FONT_WEIGHT_NUMERIC! (*attrs.offset (LFACE_FONT_INDEX as isize)) <= 100
    { face_bits! (overstrike, face, true); }

  /* Load colors, and set remaining attributes.  */

  load_face_colors (f, face, attrs);

  /* Set up box.  */
  box_ = *attrs.offset (LFACE_BOX_INDEX as isize);
  if STRINGP (box_)
    {
      /* A simple box of line width 1 drawn in color given by the string.  */
      (*face).box_color = load_color (f, face, *attrs.offset (LFACE_BOX_INDEX as isize),
				      LFACE_BOX_INDEX);
      face_bits! (box, face, FACE_SIMPLE_BOX);
      (*face).box_vertical_line_width = 1;
      (*face).box_horizontal_line_width = 1;
    }
  else if FIXNUMP (box_)
    {
      /* Simple box of specified line width in foreground color of the face.  */
      eassert! (XFIXNUM (box_) != 0);
      face_bits! (box, face, FACE_SIMPLE_BOX);
      (*face).box_vertical_line_width = XFIXNUM (box_).abs () as c_int;
      (*face).box_horizontal_line_width = XFIXNUM (box_) as c_int;
      (*face).box_color = (*face).foreground;
      face_bits! (box_color_defaulted_p, face, true);
    }
  else if CONSP (box_) && FIXNUMP (XCAR (box_)) && FIXNUMP (XCDR (box_))
    {
      /* `(VWIDTH . HWIDTH)'.  */
      face_bits! (box, face, FACE_SIMPLE_BOX);
      (*face).box_color = (*face).foreground;
      face_bits! (box_color_defaulted_p, face, true);
      (*face).box_vertical_line_width = XFIXNUM (XCAR (box_)) as c_int;
      (*face).box_horizontal_line_width = XFIXNUM (XCDR (box_)) as c_int;
    }
  else if CONSP (box_)
    {
      let mut set_color: bool = false;

      /* `(:width WIDTH :color COLOR :shadow SHADOW)'.  */
      face_bits! (box, face, FACE_SIMPLE_BOX);
      (*face).box_color = (*face).foreground;
      face_bits! (box_color_defaulted_p, face, true);
      (*face).box_vertical_line_width = 1;
      (*face).box_horizontal_line_width = 1;

      while CONSP (box_)
	{
	  let keyword: Lisp_Object;
	  let value: Lisp_Object;

	  keyword = XCAR (box_);
	  box_ = XCDR (box_);

	  if ! CONSP (box_)
	    { break; }
	  value = XCAR (box_);
	  box_ = XCDR (box_);

	  if EQ (keyword, QCline_width ())
	    {
	      if CONSP (value) && FIXNUMP (XCAR (value)) && FIXNUMP (XCDR (value))
		{
		  (*face).box_vertical_line_width = XFIXNUM (XCAR (value)) as c_int;
		  (*face).box_horizontal_line_width = XFIXNUM (XCDR (value)) as c_int;
		}
	      else if FIXNUMP (value) && XFIXNUM (value) != 0
		{
		  (*face).box_vertical_line_width = XFIXNUM (value).abs () as c_int;
		  (*face).box_horizontal_line_width = XFIXNUM (value) as c_int;
		}
	    }
	  else if EQ (keyword, QCcolor ())
	    {
	      if STRINGP (value)
		{
		  (*face).box_color = load_color (f, face, value, LFACE_BOX_INDEX);
		  face_bits! (use_box_color_for_shadows_p, face, true);
		  set_color = true;
		}
	    }
	  else if EQ (keyword, QCstyle ())
	    {
	      if EQ (value, Qreleased_button ())
		{ face_bits! (box, face, FACE_RAISED_BOX); }
	      else if EQ (value, Qpressed_button ())
		{ face_bits! (box, face, FACE_SUNKEN_BOX); }
	      else if EQ (value, Qflat_button ())
		{
		  face_bits! (box, face, FACE_SIMPLE_BOX);
		  /* Don't override colors set in this box. */
		  if ! set_color
		    { (*face).box_color = (*face).background; }
		}
	    }
	}
    }

  /* Text underline, overline, strike-through.  */

  underline = *attrs.offset (LFACE_UNDERLINE_INDEX as isize);
  if EQ (underline, Qt ())
    {
      /* Use default color (same as foreground color).  */
      face_bits! (underline, face, FACE_UNDERLINE_SINGLE);
      face_bits! (underline_defaulted_p, face, true);
      (*face).underline_color = 0;
      face_bits! (underline_at_descent_line_p, face, false);
      (*face).underline_pixels_above_descent_line = 0;
    }
  else if STRINGP (underline)
    {
      /* Use specified color.  */
      face_bits! (underline, face, FACE_UNDERLINE_SINGLE);
      face_bits! (underline_defaulted_p, face, false);
      (*face).underline_color = load_color (f, face, underline, LFACE_UNDERLINE_INDEX);
      face_bits! (underline_at_descent_line_p, face, false);
      (*face).underline_pixels_above_descent_line = 0;
    }
  else if NILP (underline)
    {
      face_bits! (underline, face, FACE_NO_UNDERLINE);
      face_bits! (underline_defaulted_p, face, false);
      (*face).underline_color = 0;
      face_bits! (underline_at_descent_line_p, face, false);
      (*face).underline_pixels_above_descent_line = 0;
    }
  else if CONSP (underline)
    {
      /* `(:color COLOR :style STYLE)'.  */
      face_bits! (underline, face, FACE_UNDERLINE_SINGLE);
      (*face).underline_color = 0;
      face_bits! (underline_defaulted_p, face, true);
      face_bits! (underline_at_descent_line_p, face, false);
      (*face).underline_pixels_above_descent_line = 0;

      while CONSP (underline)
	{
	  let keyword: Lisp_Object;
	  let value: Lisp_Object;

	  keyword = XCAR (underline);
	  underline = XCDR (underline);

	  if ! CONSP (underline)
	    { break; }
	  value = XCAR (underline);
	  underline = XCDR (underline);

	  if EQ (keyword, QCcolor ())
	    {
	      if EQ (value, Qforeground_color ())
		{
		  face_bits! (underline_defaulted_p, face, true);
		  (*face).underline_color = 0;
		}
	      else if STRINGP (value)
		{
		  face_bits! (underline_defaulted_p, face, false);
		  (*face).underline_color = load_color (f, face, value, LFACE_UNDERLINE_INDEX);
		}
	    }
	  else if EQ (keyword, QCstyle ())
	    {
	      if EQ (value, Qline ())
		{ face_bits! (underline, face, FACE_UNDERLINE_SINGLE); }
	      else if EQ (value, Qdouble_line ())
		{ face_bits! (underline, face, FACE_UNDERLINE_DOUBLE_LINE); }
	      else if EQ (value, Qwave ())
		{ face_bits! (underline, face, FACE_UNDERLINE_WAVE); }
	      else if EQ (value, Qdots ())
		{ face_bits! (underline, face, FACE_UNDERLINE_DOTS); }
	      else if EQ (value, Qdashes ())
		{ face_bits! (underline, face, FACE_UNDERLINE_DASHES); }
	      else
		{ face_bits! (underline, face, FACE_UNDERLINE_SINGLE); }
	    }
	  else if EQ (keyword, QCposition ())
	    {
	      face_bits! (underline_at_descent_line_p, face, ! NILP (value));

	      if FIXNATP (value)
		{ (*face).underline_pixels_above_descent_line = XFIXNAT (value) as c_int; }
	    }
	}
    }

  overline = *attrs.offset (LFACE_OVERLINE_INDEX as isize);
  if STRINGP (overline)
    {
      (*face).overline_color = load_color (f, face, *attrs.offset (LFACE_OVERLINE_INDEX as isize),
					   LFACE_OVERLINE_INDEX);
      face_bits! (overline_p, face, true);
    }
  else if EQ (overline, Qt ())
    {
      (*face).overline_color = (*face).foreground;
      face_bits! (overline_color_defaulted_p, face, true);
      face_bits! (overline_p, face, true);
    }

  strike_through = *attrs.offset (LFACE_STRIKE_THROUGH_INDEX as isize);
  if STRINGP (strike_through)
    {
      (*face).strike_through_color
	= load_color (f, face, *attrs.offset (LFACE_STRIKE_THROUGH_INDEX as isize),
		      LFACE_STRIKE_THROUGH_INDEX);
      face_bits! (strike_through_p, face, true);
    }
  else if EQ (strike_through, Qt ())
    {
      (*face).strike_through_color = (*face).foreground;
      face_bits! (strike_through_color_defaulted_p, face, true);
      face_bits! (strike_through_p, face, true);
    }

  stipple = *attrs.offset (LFACE_STIPPLE_INDEX as isize);
  if ! NILP (stipple)
    { (*face).stipple = load_pixmap (f, stipple); }

  return face;
  }
}

/// C 6277-6303: "Realize the fully-specified face that uses FONT-OBJECT and has the same
/// attributes as BASE_FACE except for the font on frame F.  FONT-OBJECT may be nil, in which case,
/// realized a face of no-font."  `static`, inside C 6273's `#ifdef HAVE_WINDOW_SYSTEM`: recorded as
/// dead until `face_for_font` was written (status 4.772); transcribed now.
#[cfg (feature = "have_window_system")]
unsafe fn realize_non_ascii_face (f: *mut frame, font_object: Lisp_Object,
				  base_face: *mut face) -> *mut face
{
  let cache: *mut face_cache = FRAME_FACE_CACHE (f);
  let face: *mut face;

  face = xmalloc (core::mem::size_of::<face> ()) as *mut face;
  /* C's `*face = *base_face;` is a struct assignment; the port copies the bytes (the `realize_face`
     shape, this unit's own).  */
  core::ptr::copy_nonoverlapping (base_face, face, 1);
  (*face).gc = std::ptr::null_mut ();
  /* C's `face->overstrike = ...` writes the one-bit member; the mirror keeps it in `bits` with
     `FACE_OVERSTRIKE_P` (the `face_bits!` family's shape, status 4.765).  */
  face_bits! (overstrike, face,
	      !NILP (font_object)
	      && FONT_WEIGHT_NAME_NUMERIC! ((*face).lface[LFACE_WEIGHT_INDEX as usize]) > 100
	      && FONT_WEIGHT_NUMERIC! (font_object) <= 100);

  /* Don't try to free the colors copied bitwise from BASE_FACE.  */
  face_bits! (colors_copied_bitwise_p, face, true);
  (*face).font = if NILP (font_object) { std::ptr::null_mut () } else { XFONT_OBJECT (font_object) };
  (*face).gc = std::ptr::null_mut ();

  cache_face (cache, face, (*face).hash);

  return face;
}

/* C 6305-6322: `font_maybe_unset_attribute` (and C 6324-6661's `realize_gui_face` body) stay
   recorded as dead: no delivered configuration reaches them (P-08).  */


/* C 6674-6747: map the specified color COLOR of face FACE on frame F to a tty
   color index.  */

unsafe fn map_tty_color (f: *mut frame, face: *mut face, color: Lisp_Object,
			 idx: lface_attribute_index, defaulted: *mut bool)
{
  let mut frame: Lisp_Object = Qnil ();
  let mut def: Lisp_Object;
  let foreground_p: bool = idx != LFACE_BACKGROUND_INDEX;
  let default_pixel: c_ulong =
    if foreground_p { FACE_TTY_DEFAULT_FG_COLOR } else { FACE_TTY_DEFAULT_BG_COLOR };
  let mut pixel: c_ulong = default_pixel;
  /* C's `MSDOS` arm (`default_other_pixel`) is dead.  */

  eassert! (idx == LFACE_FOREGROUND_INDEX
	    || idx == LFACE_BACKGROUND_INDEX
	    || idx == LFACE_UNDERLINE_INDEX);

  XSETFRAME! (frame, f);

  def = assoc_no_quit (color, calln! (Qtty_color_alist (), frame));
  if STRINGP (color)
      && SCHARS (color) != 0
      && CONSP (Vtty_defined_color_alist)
      && CONSP (def)
    {
      /* Associations in tty-defined-color-alist are of the form
	 (NAME INDEX R G B).  We need the INDEX part.  */
      pixel = XFIXNUM (XCAR (XCDR (def))) as c_ulong;
    }

  if pixel == default_pixel && STRINGP (color)
    {
      pixel = load_color (f, face, color, idx);

      /* C's `MSDOS` arm is dead.  */
    }

  match idx
    {
      LFACE_FOREGROUND_INDEX =>
	{ (*face).foreground = pixel; }
      LFACE_UNDERLINE_INDEX =>
	{ (*face).underline_color = pixel; }
      LFACE_BACKGROUND_INDEX | _ =>
	{ (*face).background = pixel; }
    }
}

/* C 6755-6878: realize the fully-specified face with attributes ATTRS in face
   cache CACHE for ASCII characters, for TTY frame CACHE->f.  */

unsafe fn realize_tty_face (cache: *mut face_cache, attrs: *mut Lisp_Object) -> *mut face
{
  let face: *mut face;
  let mut weight: c_int;
  let mut slant: c_int;
  let underline: Lisp_Object;
  let mut face_colors_defaulted: bool = false;
  let f: *mut frame = (*cache).f;

  /* Frame must be a termcap frame.  */
  eassert! (is_tty_frame ((*cache).f));

  /* Allocate a new realized face.  */
  face = make_realized_face (attrs);
  /* C's `#if false` arm (`face->font_name`) is dead in every configuration.  */

  /* Map face attributes to TTY appearances.  */
  weight = FONT_WEIGHT_NAME_NUMERIC! (*attrs.offset (LFACE_WEIGHT_INDEX as isize));
  slant = FONT_SLANT_NAME_NUMERIC! (*attrs.offset (LFACE_SLANT_INDEX as isize));
  if weight > 100
    { face_bits! (tty_bold_p, face, true); }
  if slant != 100
    { face_bits! (tty_italic_p, face, true); }
  if ! NILP (*attrs.offset (LFACE_INVERSE_INDEX as isize))
    { face_bits! (tty_reverse_p, face, true); }
  if ! NILP (*attrs.offset (LFACE_STRIKE_THROUGH_INDEX as isize))
    { face_bits! (tty_strike_through_p, face, true); }
  if ! NILP (*attrs.offset (LFACE_OVERLINE_INDEX as isize))
    { face_bits! (tty_overline_p, face, true); }

  underline = *attrs.offset (LFACE_UNDERLINE_INDEX as isize);
  if NILP (underline)
    {
      face_bits! (underline, face, FACE_NO_UNDERLINE);
      (*face).underline_color = 0;
    }
  else if EQ (underline, Qt ())
    {
      face_bits! (underline, face, FACE_UNDERLINE_SINGLE);
      (*face).underline_color = 0;
    }
  else if STRINGP (underline)
    {
      face_bits! (underline, face, FACE_UNDERLINE_SINGLE);
      let mut underline_color_defaulted: bool = false;
      map_tty_color (f, face, underline, LFACE_UNDERLINE_INDEX,
		     &mut underline_color_defaulted);
    }
  else if CONSP (underline)
    {
      /* `(:color COLOR :style STYLE)'.
	 STYLE being one of `line', `double-line', `wave', `dots' or `dashes'.  */
      face_bits! (underline, face, FACE_UNDERLINE_SINGLE);
      (*face).underline_color = 0;

      let mut underline = underline;
      while CONSP (underline)
	{
	  let keyword: Lisp_Object;
	  let value: Lisp_Object;

	  keyword = XCAR (underline);
	  underline = XCDR (underline);

	  if ! CONSP (underline)
	    { break; }
	  value = XCAR (underline);
	  underline = XCDR (underline);

	  if EQ (keyword, QCcolor ())
	    {
	      if EQ (value, Qforeground_color ())
		{ (*face).underline_color = 0; }
	      else if STRINGP (value)
		{
		  let mut underline_color_defaulted: bool = false;
		  map_tty_color (f, face, value, LFACE_UNDERLINE_INDEX,
				 &mut underline_color_defaulted);
		}
	    }
	  else if EQ (keyword, QCstyle ())
	    {
	      if EQ (value, Qline ())
		{ face_bits! (underline, face, FACE_UNDERLINE_SINGLE); }
	      else if EQ (value, Qdouble_line ())
		{ face_bits! (underline, face, FACE_UNDERLINE_DOUBLE_LINE); }
	      else if EQ (value, Qwave ())
		{ face_bits! (underline, face, FACE_UNDERLINE_WAVE); }
	      else if EQ (value, Qdots ())
		{ face_bits! (underline, face, FACE_UNDERLINE_DOTS); }
	      else if EQ (value, Qdashes ())
		{ face_bits! (underline, face, FACE_UNDERLINE_DASHES); }
	      else
		{ face_bits! (underline, face, FACE_UNDERLINE_SINGLE); }
	    }
	}
    }

  /* Map color names to color indices.  */
  map_tty_color (f, face, (*face).lface[LFACE_FOREGROUND_INDEX as usize],
		 LFACE_FOREGROUND_INDEX, &mut face_colors_defaulted);
  map_tty_color (f, face, (*face).lface[LFACE_BACKGROUND_INDEX as usize],
		 LFACE_BACKGROUND_INDEX, &mut face_colors_defaulted);

  /* Swap colors if face is inverse-video.  If the colors are taken
     from the frame colors, they are already inverted, since the
     frame-creation function calls x-handle-reverse-video.  */
  if face_bits! (tty_reverse_p, face) && ! face_colors_defaulted
    {
      let tem: c_ulong = (*face).foreground;
      (*face).foreground = (*face).background;
      (*face).background = tem;
    }

  if tty_suppress_bold_inverse_default_colors_p
      && face_bits! (tty_bold_p, face)
      && (*face).background == FACE_TTY_DEFAULT_FG_COLOR
      && (*face).foreground == FACE_TTY_DEFAULT_BG_COLOR
    { face_bits! (tty_bold_p, face, false); }

  return face;
}

DEFUN! ("tty-suppress-bold-inverse-default-colors",
       Ftty_suppress_bold_inverse_default_colors,
       Stty_suppress_bold_inverse_default_colors, 1, 1, 0,
       doc: "Suppress/allow boldness of faces with inverse default colors.\nSUPPRESS non-nil means suppress it.\nThis affects bold faces on TTYs whose foreground is the default background\ncolor of the display and whose background is the default foreground color.\nFor such faces, the bold face attribute is ignored if this variable\nis non-nil.",
       (suppress: Lisp_Object),
       {
  tty_suppress_bold_inverse_default_colors_p = ! NILP (suppress);
  face_change = true;
  return suppress;
});


/***********************************************************************
			   Computing Faces
 ***********************************************************************/

/* C 6906-6929: return the ID of the face to use to display character CH with face
   property PROP on frame F in current_buffer.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compute_char_face (f: *mut frame, mut ch: c_int,
					    prop: Lisp_Object) -> c_int
{
  let mut face_id: c_int;

  if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { ch = 0; }

  if NILP (prop)
    {
      let face: *mut face = FACE_FROM_ID (f, DEFAULT_FACE_ID);
      face_id = FACE_FOR_CHAR (f, face, ch, -1, Qnil ());
    }
  else
    {
      let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
	[Qnil (); LFACE_VECTOR_SIZE as usize];
      let default_face: *mut face = FACE_FROM_ID (f, DEFAULT_FACE_ID);
      memcpy (attrs.as_mut_ptr () as *mut c_void, (*default_face).lface.as_mut_ptr ()
	      as *const c_void, std::mem::size_of_val (&attrs));
      merge_face_ref (std::ptr::null_mut (), f, prop, attrs.as_mut_ptr (), true,
		      std::ptr::null_mut (), 0);
      face_id = lookup_face (f, attrs.as_mut_ptr ());
    }

  return face_id;
}

/* C 6954-7088: return the face ID associated with buffer position POS for
   displaying ASCII characters.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn face_at_buffer_position (w: *mut window, pos: ptrdiff_t,
						  endptr: *mut ptrdiff_t,
						  limit: ptrdiff_t,
						  mouse: bool, base_face_id: c_int,
						  attr_filter: lface_attribute_index)
  -> c_int
{
  let f: *mut frame = XFRAME ((*w).frame);
  let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];
  let mut prop: Lisp_Object;
  let mut position: Lisp_Object = Qnil ();
  let mut i: ptrdiff_t;
  let mut noverlays: ptrdiff_t;
  let mut overlay_vec: *mut Lisp_Object = std::ptr::null_mut ();
  let mut endpos: ptrdiff_t;
  let propname: Lisp_Object = if mouse { Qmouse_face () } else { Qface () };
  let mut limit1: Lisp_Object = Qnil ();
  let mut end: Lisp_Object;
  let mut default_face: *mut face;

  /* W must display the current buffer.  We could write this function
     to use the frame and buffer of W, but right now it doesn't.  */
  /* eassert (XBUFFER (w->contents) == current_buffer); */

  XSETFASTINT! (position, pos);

  endpos = ZV! ();

  /* Get the `face' or `mouse_face' text property at POS, and
     determine the next position at which the property changes.  */
  prop = Fget_text_property (position, propname, (*w).contents);
  XSETFASTINT! (limit1, std::cmp::min (limit, endpos));
  end = Fnext_single_property_change (position, propname, (*w).contents, limit1);
  if FIXNUMP (end)
    { endpos = XFIXNUM (end); }

  /* Look at properties from overlays.  */
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  {
    let mut next_overlay: ptrdiff_t = 0;
    GET_OVERLAYS_AT! (sa_avail, pos, overlay_vec, noverlays, &mut next_overlay);
    /* overlays_at can return next_overlay beyond the end of the current
       narrowing.  We don't want that to leak into the display code.  */
    if next_overlay > ZV! ()
      { next_overlay = ZV! (); }
    if next_overlay < endpos
      { endpos = next_overlay; }
  }

  *endptr = endpos;

  {
    let mut face_id: c_int;

    if base_face_id >= 0
      { face_id = base_face_id; }
    else if NILP (Vface_remapping_alist)
      { face_id = DEFAULT_FACE_ID; }
    else
      { face_id = lookup_basic_face (w, f, DEFAULT_FACE_ID); }

    default_face = FACE_FROM_ID_OR_NULL (f, face_id);
    /* Make sure the default face ID is usable: if someone freed the
       cached faces since we've looked up these faces, we need to look
       them up again.  */
    if default_face.is_null ()
      {
	if (*FRAME_FACE_CACHE (f)).used == 0
	  { recompute_basic_faces (f); }
	default_face = FACE_FROM_ID (f,
				     lookup_basic_face (w, f, DEFAULT_FACE_ID));
      }
  }

  /* Optimize common cases where we can use the default face.  */
  if noverlays == 0
      && NILP (prop)
    {
      SAFE_FREE! (sa_count, sa_mark);
      return (*default_face).id;
    }

  /* Begin with attributes from the default face.  */
  memcpy (attrs.as_mut_ptr () as *mut c_void, (*default_face).lface.as_mut_ptr ()
	  as *const c_void, std::mem::size_of_val (&attrs));

  /* Merge in attributes specified via text properties.  */
  if ! NILP (prop)
    { merge_face_ref (w, f, prop, attrs.as_mut_ptr (), true, std::ptr::null_mut (),
		      attr_filter); }

  /* Now merge the overlay data.  */
  noverlays = sort_overlays (overlay_vec, noverlays, w as *mut c_void);
  /* For mouse-face, we need only the single highest-priority face
     from the overlays, if any.  */
  if mouse
    {
      prop = Qnil ();
      i = noverlays - 1;
      while i >= 0 && NILP (prop)
	{
	  let mut oendpos: ptrdiff_t;

	  prop = Foverlay_get (*overlay_vec.offset (i as isize), propname);
	  if ! NILP (prop)
	    {
	      /* Overlays always take priority over text properties,
		 so discard the mouse-face text property, if any, and
		 use the overlay property instead.  */
	      memcpy (attrs.as_mut_ptr () as *mut c_void,
		      (*default_face).lface.as_mut_ptr () as *const c_void,
		      std::mem::size_of_val (&attrs));
	      merge_face_ref (w, f, prop, attrs.as_mut_ptr (), true,
			      std::ptr::null_mut (), attr_filter);
	    }

	  oendpos = OVERLAY_END (*overlay_vec.offset (i as isize));
	  if oendpos < endpos
	    { endpos = oendpos; }
	  i -= 1;
	}
    }
  else
    {
      for i in 0 .. noverlays
	{
	  let mut oendpos: ptrdiff_t;

	  prop = Foverlay_get (*overlay_vec.offset (i as isize), propname);

	  if ! NILP (prop)
	    { merge_face_ref (w, f, prop, attrs.as_mut_ptr (), true,
			      std::ptr::null_mut (), attr_filter); }

          oendpos = OVERLAY_END (*overlay_vec.offset (i as isize));
          if oendpos < endpos
            { endpos = oendpos; }
        }
    }

  *endptr = endpos;

  SAFE_FREE! (sa_count, sa_mark);

  /* Look up a realized face with the given face attributes,
     or realize a new one for ASCII characters.  */
  return lookup_face (f, attrs.as_mut_ptr ());
}

/* C 7096-7146: return the face ID at buffer position POS … for overlay OVERLAY.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn face_for_overlay_string (w: *mut window, pos: ptrdiff_t,
						  endptr: *mut ptrdiff_t,
						  limit: ptrdiff_t,
						  mouse: bool, overlay: Lisp_Object,
						  attr_filter: lface_attribute_index)
  -> c_int
{
  let f: *mut frame = XFRAME ((*w).frame);
  let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];
  let mut prop: Lisp_Object;
  let mut position: Lisp_Object = Qnil ();
  let mut endpos: ptrdiff_t;
  let propname: Lisp_Object = if mouse { Qmouse_face () } else { Qface () };
  let mut limit1: Lisp_Object = Qnil ();
  let mut end: Lisp_Object;
  let default_face: *mut face;
  let _ = overlay;

  /* W must display the current buffer.  We could write this function
     to use the frame and buffer of W, but right now it doesn't.  */
  /* eassert (XBUFFER (w->contents) == current_buffer); */

  XSETFASTINT! (position, pos);

  endpos = ZV! ();

  /* Get the `face' or `mouse_face' text property at POS, and
     determine the next position at which the property changes.  */
  prop = Fget_text_property (position, propname, (*w).contents);
  XSETFASTINT! (limit1, std::cmp::min (limit, endpos));
  end = Fnext_single_property_change (position, propname, (*w).contents, limit1);
  if FIXNUMP (end)
    { endpos = XFIXNUM (end); }

  *endptr = endpos;

  /* Optimize common case where we can use the default face.  */
  if NILP (prop)
      && NILP (Vface_remapping_alist)
    { return DEFAULT_FACE_ID; }

  /* Begin with attributes from the default face.  */
  default_face = FACE_FROM_ID (f, lookup_basic_face (w, f, DEFAULT_FACE_ID));
  memcpy (attrs.as_mut_ptr () as *mut c_void, (*default_face).lface.as_mut_ptr ()
	  as *const c_void, std::mem::size_of_val (&attrs));

  /* Merge in attributes specified via text properties.  */
  if ! NILP (prop)
    { merge_face_ref (w, f, prop, attrs.as_mut_ptr (), true, std::ptr::null_mut (),
		      attr_filter); }

  *endptr = endpos;

  /* Look up a realized face with the given face attributes,
     or realize a new one for ASCII characters.  */
  return lookup_face (f, attrs.as_mut_ptr ());
}


/* C 7168-7224: compute the face at character position POS in Lisp string STRING on
   window W, for ASCII characters.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn face_at_string_position (w: *mut window, string: Lisp_Object,
						  pos: ptrdiff_t, bufpos: ptrdiff_t,
						  endptr: *mut ptrdiff_t,
						  base_face_id: face_id,
						  mouse_p: bool,
						  attr_filter: lface_attribute_index)
  -> c_int
{
  let _ = bufpos;
  let mut prop: Lisp_Object;
  let mut position: Lisp_Object = Qnil ();
  let mut end: Lisp_Object;
  let mut limit: Lisp_Object = Qnil ();
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];
  let mut base_face: *mut face;
  let multibyte_p: bool = STRING_MULTIBYTE (string);
  let prop_name: Lisp_Object = if mouse_p { Qmouse_face () } else { Qface () };

  /* Get the value of the face property at the current position within
     STRING.  Value is nil if there is no face property.  */
  XSETFASTINT! (position, pos);
  prop = Fget_text_property (position, prop_name, string);

  /* Get the next position at which to check for faces.  Value of end
     is nil if face is constant all the way to the end of the string.
     Otherwise it is a string position where to check faces next.
     Limit is the maximum position up to which to check for property
     changes in Fnext_single_property_change.  Strings are usually
     short, so set the limit to the end of the string.  */
  XSETFASTINT! (limit, SCHARS (string));
  end = Fnext_single_property_change (position, prop_name, string, limit);
  if FIXNUMP (end)
    { *endptr = XFIXNAT (end); }
  else
    { *endptr = -1; }

  base_face = FACE_FROM_ID_OR_NULL (f, base_face_id);
  if base_face.is_null ()
    { base_face = FACE_FROM_ID (f, lookup_basic_face (w, f, DEFAULT_FACE_ID)); }

  /* Optimize the default case that there is no face property.  */
  if NILP (prop)
      && (multibyte_p
	  /* We can't realize faces for different charsets differently
	     if we don't have fonts, so we can stop here if not working
	     on a window-system frame.  */
	  || ! FRAME_WINDOW_P (f)
	  || FACE_SUITABLE_FOR_ASCII_CHAR_P (base_face))
    { return (*base_face).id; }

  /* Begin with attributes from the base face.  */
  memcpy (attrs.as_mut_ptr () as *mut c_void, (*base_face).lface.as_mut_ptr ()
	  as *const c_void, std::mem::size_of_val (&attrs));

  /* Merge in attributes specified via text properties.  */
  if ! NILP (prop)
    { merge_face_ref (w, f, prop, attrs.as_mut_ptr (), true, std::ptr::null_mut (),
		      attr_filter); }

  /* Look up a realized face with the given face attributes,
     or realize a new one for ASCII characters.  */
  return lookup_face (f, attrs.as_mut_ptr ());
}


/* C 7242-7302: merge a face into a realized face.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn merge_faces (w: *mut window, face_name: Lisp_Object,
				      face_id: c_int, base_face_id: c_int) -> c_int
{
  let f: *mut frame = WINDOW_XFRAME! (w);
  let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
    [Qnil (); LFACE_VECTOR_SIZE as usize];
  let base_face: *mut face = FACE_FROM_ID_OR_NULL (f, base_face_id);

  if base_face.is_null ()
    { return base_face_id; }

  if EQ (face_name, Qt ())
    {
      if face_id < 0 || (face_id as ptrdiff_t) >= lface_id_to_name_size
	{ return base_face_id; }
      let face_name: Lisp_Object = *lface_id_to_name.offset (face_id as isize);
      /* When called during make-frame, lookup_derived_face may fail
	 if the faces are uninitialized.  Don't signal an error.  */
      let face_id: c_int = lookup_derived_face (w, f, face_name, base_face_id, false);
      return if face_id >= 0 { face_id } else { base_face_id };
    }

  /* Begin with attributes from the base face.  */
  memcpy (attrs.as_mut_ptr () as *mut c_void, (*base_face).lface.as_mut_ptr ()
	  as *const c_void, std::mem::size_of_val (&attrs));

  if ! NILP (face_name)
    {
      if ! merge_named_face (w, f, face_name, attrs.as_mut_ptr (),
			     std::ptr::null_mut (), 0)
	{ return base_face_id; }
    }
  else
    {
      if face_id < 0
	{ return base_face_id; }

      let face: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);

      if face.is_null ()
	{ return base_face_id; }

      if face_id != DEFAULT_FACE_ID
	{
	  let deflt: *mut face = FACE_FROM_ID (f, DEFAULT_FACE_ID);
	  let mut lface_attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] =
	    [Qnil (); LFACE_VECTOR_SIZE as usize];
	  let mut i: c_int;

	  memcpy (lface_attrs.as_mut_ptr () as *mut c_void, (*face).lface.as_mut_ptr ()
		  as *const c_void, std::mem::size_of_val (&lface_attrs));
	  /* Make explicit any attributes whose value is 'reset'.  */
	  for i in 1 .. LFACE_VECTOR_SIZE
	    {
	      if EQ (lface_attrs[i as usize], Qreset ())
		{ lface_attrs[i as usize] = (*deflt).lface[i as usize]; }
	    }
	  merge_face_vectors (w, f, lface_attrs.as_mut_ptr (), attrs.as_mut_ptr (),
			      0 as *mut named_merge_point);
	}
      else
	{ merge_face_vectors (w, f, (*face).lface.as_mut_ptr (), attrs.as_mut_ptr (),
			      0 as *mut named_merge_point); }
    }

  /* Look up a realized face with the given face attributes,
     or realize a new one for ASCII characters.  */
  return lookup_face (f, attrs.as_mut_ptr ());
}

/* C 7306-7352: the `#ifndef HAVE_X_WINDOWS` DEFUN.  */

DEFUN! ("x-load-color-file", Fx_load_color_file,
       Sx_load_color_file, 1, 1, 0,
       doc: "Create an alist of color entries from an external file.\n\nThe file should define one named RGB color per line like so:\n  R G B   name\nwhere R,G,B are numbers between 0 and 255 and name is an arbitrary string.",
       (filename: Lisp_Object),
       {
  let fp: *mut FILE;
  let mut cmap: Lisp_Object = Qnil ();
  let abspath: Lisp_Object;

  CHECK_STRING (filename);
  abspath = Fexpand_file_name (filename, Qnil ());

  block_input ();
  fp = emacs_fopen (SSDATA (abspath),
		    concat! ("r", FOPEN_TEXT! (), "\0" ).as_ptr () as *const c_char);
  if ! fp.is_null ()
    {
      let mut buf: [c_char; 512] = [0; 512];
      let mut red: c_int = 0;
      let mut green: c_int = 0;
      let mut blue: c_int = 0;
      let mut num: c_int = 0;

      while ! fgets (buf.as_mut_ptr (), std::mem::size_of_val (&buf) as c_int, fp)
	.is_null ()
	{
	  if sscanf (buf.as_ptr (), c"%d %d %d %n".as_ptr (), &mut red, &mut green,
		     &mut blue, &mut num) == 3
	    {
	      /* C's `HAVE_NTGUI` arm (`RGB`) is dead.  */
	      let color: c_int = (red << 16) | (green << 8) | blue;
	      let name: *mut c_char = buf.as_mut_ptr ().offset (num as isize);
	      let mut len: ptrdiff_t = strlen (name) as ptrdiff_t;
	      if 0 < len && *name.offset (len - 1) == '\n' as c_char
		{ len -= 1; }
	      cmap = Fcons (Fcons (make_string (name, len), make_fixnum (color as isize)),
			    cmap);
	    }
	}
      emacs_fclose! (fp);
    }
  unblock_input ();
  return cmap;
});

/* C 7355-7434: the `GLYPH_DEBUG` block (`dump_realized_face`,
   `dump_face_lisp`, `Fdump_face`, `Fshow_face_resources`) is dead.  */


/***********************************************************************
			    Initialization
 ***********************************************************************/

/* C 7449-7500: set up the face ID table for the faces defined during loadup.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_xfaces ()
{
  /* `HAVE_PDUMPER` is defined in the frozen configuration, so C's arm is live.  */
  let mut nfaces: c_int;

  if dumped_with_pdumper_p ()
    {
      nfaces = XFIXNAT (Fhash_table_count (Vface_new_frame_defaults)) as c_int;
      if nfaces > 0
	{
	  /* Allocate the lface_id_to_name[] array.  */
	  lface_id_to_name_size = nfaces as ptrdiff_t;
	  next_lface_id = nfaces;
	  lface_id_to_name = xnmalloc (next_lface_id as isize,
				       std::mem::size_of::<Lisp_Object> () as isize)
	    as *mut Lisp_Object;

	  /* Store the faces.  */
	  let table: *mut Lisp_Hash_Table = XHASH_TABLE (Vface_new_frame_defaults);
	  let mut idx: ptrdiff_t = 0;
	  while idx < nfaces as ptrdiff_t
	    {
	      let lface: Lisp_Object = HASH_KEY (table, idx as hash_idx_t);
	      let face_id: Lisp_Object = CAR (HASH_VALUE (table, idx as hash_idx_t));
	      if FIXNATP (face_id)
		{
		  let id: c_int = XFIXNAT (face_id) as c_int;
		  eassert! (id >= 0);
		  *lface_id_to_name.offset (id as isize) = lface;
		}
	      idx += 1;
	    }
	}
    }

  face_attr_sym[0] = Qface ();
  face_attr_sym[LFACE_FAMILY_INDEX as usize] = QCfamily ();
  face_attr_sym[LFACE_FOUNDRY_INDEX as usize] = QCfoundry ();
  face_attr_sym[LFACE_SWIDTH_INDEX as usize] = QCwidth ();
  face_attr_sym[LFACE_HEIGHT_INDEX as usize] = QCheight ();
  face_attr_sym[LFACE_WEIGHT_INDEX as usize] = QCweight ();
  face_attr_sym[LFACE_SLANT_INDEX as usize] = QCslant ();
  face_attr_sym[LFACE_UNDERLINE_INDEX as usize] = QCunderline ();
  face_attr_sym[LFACE_INVERSE_INDEX as usize] = QCinverse_video ();
  face_attr_sym[LFACE_FOREGROUND_INDEX as usize] = QCforeground ();
  face_attr_sym[LFACE_BACKGROUND_INDEX as usize] = QCbackground ();
  face_attr_sym[LFACE_STIPPLE_INDEX as usize] = QCstipple ();
  face_attr_sym[LFACE_OVERLINE_INDEX as usize] = QCoverline ();
  face_attr_sym[LFACE_STRIKE_THROUGH_INDEX as usize] = QCstrike_through ();
  face_attr_sym[LFACE_BOX_INDEX as usize] = QCbox ();
  face_attr_sym[LFACE_FONT_INDEX as usize] = QCfont ();
  face_attr_sym[LFACE_INHERIT_INDEX as usize] = QCinherit ();
  face_attr_sym[LFACE_FONTSET_INDEX as usize] = QCfontset ();
  face_attr_sym[LFACE_DISTANT_FOREGROUND_INDEX as usize] = QCdistant_foreground ();
  face_attr_sym[LFACE_EXTEND_INDEX as usize] = QCextend ();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_xfaces ()
{
  /* The symbols `face' and `mouse-face' used as text properties.  */
  DEFSYM! (Qface, "face");

  /* Property for basic faces which other faces cannot inherit.  */
  DEFSYM! (Qface_no_inherit, "face-no-inherit");

  /* Error symbol for wrong_type_argument in load_pixmap.  */
  DEFSYM! (Qbitmap_spec_p, "bitmap-spec-p");

  /* The name of the function to call when the background of the frame
     has changed, frame_set_background_mode.  */
  DEFSYM! (Qframe_set_background_mode, "frame-set-background-mode");

  /* Lisp face attribute keywords.  */
  DEFSYM! (QCfamily, ":family");
  DEFSYM! (QCheight, ":height");
  DEFSYM! (QCweight, ":weight");
  DEFSYM! (QCslant, ":slant");
  DEFSYM! (QCunderline, ":underline");
  DEFSYM! (QCinverse_video, ":inverse-video");
  DEFSYM! (QCforeground, ":foreground");
  DEFSYM! (QCbackground, ":background");
  DEFSYM! (QCstipple, ":stipple");
  DEFSYM! (QCwidth, ":width");
  DEFSYM! (QCfont, ":font");
  DEFSYM! (QCfontset, ":fontset");
  DEFSYM! (QCdistant_foreground, ":distant-foreground");
  DEFSYM! (QCbold, ":bold");
  DEFSYM! (QCitalic, ":italic");
  DEFSYM! (QCoverline, ":overline");
  DEFSYM! (QCstrike_through, ":strike-through");
  DEFSYM! (QCbox, ":box");
  DEFSYM! (QCinherit, ":inherit");
  DEFSYM! (QCextend, ":extend");

  /* Symbols used for Lisp face attribute values.  */
  DEFSYM! (QCcolor, ":color");
  DEFSYM! (QCline_width, ":line-width");
  DEFSYM! (QCstyle, ":style");
  DEFSYM! (QCposition, ":position");
  DEFSYM! (Qline, "line");
  DEFSYM! (Qwave, "wave");
  DEFSYM! (Qdouble_line, "double-line");
  DEFSYM! (Qdots, "dots");
  DEFSYM! (Qdashes, "dashes");
  DEFSYM! (Qreleased_button, "released-button");
  DEFSYM! (Qpressed_button, "pressed-button");
  DEFSYM! (Qflat_button, "flat-button");
  DEFSYM! (Qnormal, "normal");
  DEFSYM! (Qthin, "thin");
  DEFSYM! (Qextra_light, "extra-light");
  DEFSYM! (Qultra_light, "ultra-light");
  DEFSYM! (Qlight, "light");
  DEFSYM! (Qsemi_light, "semi-light");
  DEFSYM! (Qmedium, "medium");
  DEFSYM! (Qsemi_bold, "semi-bold");
  DEFSYM! (Qbook, "book");
  DEFSYM! (Qbold, "bold");
  DEFSYM! (Qextra_bold, "extra-bold");
  DEFSYM! (Qultra_bold, "ultra-bold");
  DEFSYM! (Qheavy, "heavy");
  DEFSYM! (Qultra_heavy, "ultra-heavy");
  DEFSYM! (Qblack, "black");
  DEFSYM! (Qoblique, "oblique");
  DEFSYM! (Qitalic, "italic");
  DEFSYM! (Qreset, "reset");

  DEFSYM! (Qbackground_color, "background-color");
  DEFSYM! (Qforeground_color, "foreground-color");
  DEFSYM! (Qunspecified, "unspecified");
  DEFSYM! (QCignore_defface, ":ignore-defface");

  DEFSYM! (QCwindow, ":window");
  DEFSYM! (QCfiltered, ":filtered");

  DEFSYM! (Qface_alias, "face-alias");

  DEFSYM! (Qdefault, "default");
  DEFSYM! (Qtool_bar, "tool-bar");
  DEFSYM! (Qtab_bar, "tab-bar");
  DEFSYM! (Qfringe, "fringe");
  DEFSYM! (Qtab_line, "tab-line");
  DEFSYM! (Qtab_line_inactive, "tab-line-inactive");
  DEFSYM! (Qtab_line_active, "tab-line-active");
  DEFSYM! (Qheader_line, "header-line");
  DEFSYM! (Qheader_line_inactive, "header-line-inactive");
  DEFSYM! (Qheader_line_active, "header-line-active");
  DEFSYM! (Qscroll_bar, "scroll-bar");
  DEFSYM! (Qmenu, "menu");
  DEFSYM! (Qcursor, "cursor");
  DEFSYM! (Qborder, "border");
  DEFSYM! (Qmouse, "mouse");
  DEFSYM! (Qmode_line_inactive, "mode-line-inactive");
  DEFSYM! (Qmode_line_active, "mode-line-active");
  DEFSYM! (Qvertical_border, "vertical-border");
  DEFSYM! (Qwindow_divider, "window-divider");
  DEFSYM! (Qwindow_divider_first_pixel, "window-divider-first-pixel");
  DEFSYM! (Qwindow_divider_last_pixel, "window-divider-last-pixel");
  DEFSYM! (Qinternal_border, "internal-border");
  DEFSYM! (Qchild_frame_border, "child-frame-border");

  DEFSYM! (Qtty_color_desc, "tty-color-desc");
  DEFSYM! (Qtty_color_standard_values, "tty-color-standard-values");
  DEFSYM! (Qtty_color_by_index, "tty-color-by-index");

  DEFSYM! (Qtty_color_alist, "tty-color-alist");
  DEFSYM! (Qtty_defined_color_alist, "tty-defined-color-alist");

  /* C 7625-7628 has no `DEFVAR` for these two: they are only reset and protected, so no Lisp
     variable is registered and `etc/DOC` carries no `V` record for either.  The two
     `DEFVAR_LISP_NOPRO` calls that stood here were this port's invention (found by
     `tools/gen-doc.py`, which read them as records the reference does not have).  */
  Vface_alternative_font_family_alist = Qnil ();
  staticpro (&raw const Vface_alternative_font_family_alist);
  Vface_alternative_font_registry_alist = Qnil ();
  staticpro (&raw const Vface_alternative_font_registry_alist);

  defsubr (&raw mut Sinternal_make_lisp_face);
  defsubr (&raw mut Sinternal_lisp_face_p);
  defsubr (&raw mut Sinternal_set_lisp_face_attribute);
  /* C's `HAVE_WINDOW_SYSTEM` registration (`Sinternal_set_lisp_face_attribute_from_resource`)
     is dead.  */
  defsubr (&raw mut Scolor_gray_p);
  defsubr (&raw mut Scolor_supported_p);
  /* C's `#ifndef HAVE_X_WINDOWS` registration is live.  */
  defsubr (&raw mut Sx_load_color_file);
  defsubr (&raw mut Sface_attribute_relative_p);
  defsubr (&raw mut Smerge_face_attribute);
  defsubr (&raw mut Sinternal_get_lisp_face_attribute);
  defsubr (&raw mut Sinternal_lisp_face_attribute_values);
  defsubr (&raw mut Sinternal_lisp_face_equal_p);
  defsubr (&raw mut Sinternal_lisp_face_empty_p);
  defsubr (&raw mut Sinternal_copy_lisp_face);
  defsubr (&raw mut Sinternal_merge_in_global_face);
  defsubr (&raw mut Sface_font);
  defsubr (&raw mut Sframe_face_hash_table);
  defsubr (&raw mut Sdisplay_supports_face_attributes_p);
  defsubr (&raw mut Scolor_distance);
  defsubr (&raw mut Sinternal_set_font_selection_order);
  defsubr (&raw mut Sinternal_set_alternative_font_family_alist);
  defsubr (&raw mut Sinternal_set_alternative_font_registry_alist);
  defsubr (&raw mut Sface_attributes_as_vector);
  /* C's `GLYPH_DEBUG` registrations (`Sdump_face`, `Sshow_face_resources`) are dead.  */
  defsubr (&raw mut Sclear_face_cache);
  defsubr (&raw mut Stty_suppress_bold_inverse_default_colors);

  /* C's `DEBUG_X_COLORS && HAVE_X_WINDOWS` registration (`Sdump_colors`) is dead.  */

  DEFVAR_BOOL! (c"face-filters-always-match", face_filters_always_match,
    "Non-nil means that face filters are always deemed to match.\nThis variable is intended for use only by code that evaluates\nthe \"specificity\" of a face specification and should be let-bound\nonly for this purpose.");

  DEFVAR_LISP! (c"face--new-frame-defaults", Vface_new_frame_defaults,
    "Hash table of global face definitions (for internal use only.)");
  Vface_new_frame_defaults =
    /* 33 entries is enough to fit all basic faces */
    make_hash_table (&raw const hashtest_eq, 33, Weak_None);

  DEFVAR_LISP! (c"face-default-stipple", Vface_default_stipple,
    "Default stipple pattern used on monochrome displays.\nThis stipple pattern is used on monochrome displays\ninstead of shades of gray for a face background color.\nSee `set-face-stipple' for possible values for this variable.");
  Vface_default_stipple = build_string (c"gray3".as_ptr ());

  DEFVAR_LISP! (c"tty-defined-color-alist", Vtty_defined_color_alist,
   "An alist of defined terminal colors and their RGB values.\nSee the docstring of `tty-color-alist' for the details.");
  Vtty_defined_color_alist = Qnil ();

  DEFVAR_LISP! (c"scalable-fonts-allowed", Vscalable_fonts_allowed,
	       "Allowed scalable fonts.\nA value of nil means don't allow any scalable fonts.\nA value of t means allow any scalable font.\nOtherwise, value must be a list of regular expressions.  A font may be\nscaled if its name matches a regular expression in the list.\nNote that if value is nil, a scalable font might still be used, if no\nother font of the appropriate family and registry is available.");
  Vscalable_fonts_allowed = Qnil ();

  DEFVAR_LISP! (c"face-ignored-fonts", Vface_ignored_fonts,
	       "List of ignored fonts.\nEach element is a regular expression that matches names of fonts to\nignore.");
  /* C's `HAVE_XFT` and `HAVE_OTF_KANNADA_BUG` arms are dead.  */
  Vface_ignored_fonts = Qnil ();

  DEFVAR_LISP! (c"face-remapping-alist", Vface_remapping_alist,
	       " Alist of face remappings.\nEach element is of the form:\n\n   (FACE . REPLACEMENT),\n\nwhich causes display of the face FACE to use REPLACEMENT instead.\nREPLACEMENT is a face specification, i.e. one of the following:\n\n  (1) a face name\n  (2) a property list of attribute/value pairs, or\n  (3) a list in which each element has one of the above forms.\n\nList values for REPLACEMENT are merged to form the final face\nspecification, with earlier entries taking precedence, in the same way\nas with the `face' text property.\n\nFace-name remapping cycles are suppressed; recursive references use\nthe underlying face instead of the remapped face.  So a remapping of\nthe form:\n\n   (FACE EXTRA-FACE... FACE)\n\nor:\n\n   (FACE (FACE-ATTR VAL ...) FACE)\n\ncauses EXTRA-FACE... or (FACE-ATTR VAL ...) to be _merged_ with the\nexisting definition of FACE.  Note that this isn't necessary for the\ndefault face, since every face inherits from the default face.\n\nAn entry in the list can also be a filtered face expression of the\nform:\n\n  (:filtered FILTER FACE-SPECIFICATION)\n\nThis construct applies FACE-SPECIFICATION (which can have any of the\nforms allowed for face specifications generally) only if FILTER\nmatches at the moment Emacs wants to draw text with the combined face.\n\nThe only filters currently defined are NIL (which always matches) and\n(:window PARAMETER VALUE), which matches only in the context of a\nwindow with a parameter EQ-equal to VALUE.\n\nAn entry in the face list can also be nil, which does nothing.\n\nIf `face-remapping-alist' is made buffer-local, the face remapping\ntakes effect only in that buffer.  For instance, the mode my-mode\ncould define a face `my-mode-default', and then in the mode setup\nfunction, do:\n\n   (set (make-local-variable \\='face-remapping-alist)\n        (copy-tree \\='((default my-mode-default)))).\n\nYou probably want to use the face-remap package included in Emacs\ninstead of manipulating face-remapping-alist directly.  Note that many\nof the functions in that package modify the list destructively, so make\nsure you set it to a fresh value (for instance, use `copy-tree' as in\nthe example above) before modifying.\n\nBecause Emacs normally only redraws screen areas when the underlying\nbuffer contents change, you may need to call `redraw-display' after\nchanging this variable for it to take effect.  ");
  Vface_remapping_alist = Qnil ();
  DEFSYM! (Qface_remapping_alist,"face-remapping-alist");

  DEFVAR_LISP! (c"face-font-rescale-alist", Vface_font_rescale_alist,
	       " Alist of fonts vs the rescaling factors.\nEach element is a cons (FONT-PATTERN . RESCALE-RATIO), where\nFONT-PATTERN is a font-spec or a regular expression matching a font name, and\nRESCALE-RATIO is a floating point number to specify how much larger\n(or smaller) font we should use.  For instance, if a face requests\na font of 10 point, we actually use a font of 10 * RESCALE-RATIO point.  ");
  Vface_font_rescale_alist = Qnil ();

  DEFVAR_INT! (c"face-near-same-color-threshold", face_near_same_color_threshold,
	      "Threshold for using distant-foreground color instead of foreground.\n\nThe value should be an integer number providing the minimum distance\nbetween two colors that will still qualify them to be used as foreground\nand background.  If the value of `color-distance', invoked with a nil\nMETRIC argument, for the foreground and background colors of a face is\nless than this threshold, the distant-foreground color, if defined,\nwill be used for the face instead of the foreground color.\n\nLisp programs that change the value of this variable should also\nclear the face cache, see `clear-face-cache'.");
  face_near_same_color_threshold = 30000;

  DEFVAR_LISP! (c"face-font-lax-matched-attributes",
	       Vface_font_lax_matched_attributes,
	       "Whether to match some face attributes in lax manner when realizing faces.\n\nIf non-nil, some font-related face attributes will be matched in a lax\nmanner when looking for candidate fonts.\nIf the value is t, the default, the search for fonts will not insist\non exact match for 3 font attributes: weight, width, and slant.\nInstead, it will examine the available fonts with various values of\nthese attributes, and select the font that is the closest possible\nmatch.  (If an exact match is available, it will still be selected,\nas that is the closest match.)  For example, looking for a semi-bold\nfont might select a bold or a medium-weight font if no semi-bold font\nmatching other attributes can be found.  This is especially important\nwhen the `default' face specifies unusual values for one or more of\nthese 3 attributes, which other installed fonts don't support.\n\nThe value can also be a list of font-related face attribute symbols;\nsee `set-face-attribute' for the full list of attributes.  Then the\ncorresponding face attributes will be treated as \"soft\" constraints\nin the manner described above, instead of the default 3 attributes.\n\nIf the value is nil, candidate fonts might be rejected if the don't\nhave exactly the same values of attributes as the face requests.\n\nThis variable exists for debugging of the font-selection process,\nand we advise not to change it otherwise.");
  Vface_font_lax_matched_attributes = Qt ();

  /* C's `HAVE_WINDOW_SYSTEM` registrations (`Sbitmap_spec_p`, `Sx_list_fonts`,
     `Sinternal_face_x_get_resource`, `Sx_family_fonts`) are dead.  */
  defsubr (&raw mut Scolor_values_from_color_spec);
}

/* C 7800-7843: the `DEFVAR_*` carriers this C file owns.  C's storage is
   `globals.f_<name>` (lisp.h 3495-3530) and the port's `include/globals.rs` mirrors the
   names as declarations, so the unit that owns one defines it here (docs/status.md 3.29,
   the casetab.rs/term.rs shape).  */
#[unsafe(no_mangle)]
pub static mut Vface_new_frame_defaults: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vface_default_stipple: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vtty_defined_color_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vscalable_fonts_allowed: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vface_ignored_fonts: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vface_remapping_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vface_font_rescale_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vface_font_lax_matched_attributes: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut face_filters_always_match: bool = false;
#[unsafe(no_mangle)]
pub static mut face_near_same_color_threshold: EMACS_INT = 0;

/* `struct face` on Android: NDK-measured 296 with these offsets (docs/ports/layout-gaps.md,
   status 4.813).  The 15 assertions above use `FACE_WS_SHIFT`, which only travels with
   `feature = "dispextern_face_records"`; these are the ones that hold on this target.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (face, id) == 160);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (face, gc) == 168);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (face, stipple) == 176);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (face, font) == 232);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (face, fontset) == 240);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (face, box_vertical_line_width) == 244);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (face, underline_pixels_above_descent_line) == 252);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (face, hash) == 264);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (face, next) == 272);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (face, ascii_face) == 288);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<face> () == 296);
