// femacs: port of src/font.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, file-scope object, local, enum member), same control flow,
// same error model.  The twenty-two `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's
// non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static`
// helpers stay plain `unsafe fn`.
//
// **Carried arms.**  `src/font.c` is *not* in `docs/baseline/config/cond-eval-verified.txt`
// and `python3 tools/check-cond-eval.py src/font.c` reports `45 conditions, 0 unknown,
// 2 disagreements [NOT VERIFIED]`: the frozen configuration's macro table cannot decide
// `FONT_DEBUG`, which `src/font.h` 997-998 defines itself (`#ifndef FONT_DEBUG / #define
// FONT_DEBUG / #endif`) — a §3.35/§3.51-class gap, measured with clang:
// `#include <config.h>` + `lisp.h` + `font.h` then `#ifdef FONT_DEBUG` is *live* (the
// probe prints the live branch).  Because `tools/extract-abi.py`'s `blind_conditions`
// keeps *both* arms of every `#if` live for an unverified file, the per-function token
// check demands the tokens of C's text on both sides, and per the integrator's 2026-10-03
// decision (PORTING.md P-51, status 3.49) this file *carries* the text of the arms the
// frozen build does not compile, each under a `#[cfg (feature = "…")]` that names the C
// condition's non-frozen side.  None of these features is ever set, so the compiled build
// is what clang compiles.  The arms, and the two that are *not* carried because a gate
// would reject the text:
//
//   * C 43-44 `#include TERM_HEADER` (HAVE_WINDOW_SYSTEM undef), C 181-182 and 199-201
//     (`HAVE_ANDROID && !ANDROID_STUBIFY`), C 188-206 (`HAVE_ANDROID`), C 242-277
//     (`HAVE_XFT || HAVE_FREETYPE || HAVE_NS`), C 135-154 (`ENABLE_CHECKING`, which
//     `src/bidi.rs` and `src/regex-emacs.rs` carry under the bare `#[cfg (enable_checking)]`
//     pass), and the platform arms at C 341-362, 1647-1658, 1874-2001, 2034-2041,
//     2748-2760, 2987-3013, 3032-3034, 3414-3421, 3529-3530, 3572-3575, 3730-3762,
//     3892-3993, 4276-4355, 5592-5699, 5911-5915, 5951-5952, 6031-6034 and 6053-6083.
//   * `#if 0` regions are recorded in `docs/ports/font.md` and *not* transcribed
//     (status 3.55's `ccl.rs` rule): C 4838-5236 (the experimental LGSTRING helpers and
//     `Ffont_drive_otf`/`Ffont_otf_alternates`), 5534-5589 (`Fdraw_string`), 5692, 5911-5915,
//     5933-5936, 5946-5948.
//   * C 5240-5589 (`#ifdef FONT_DEBUG`) is carried as **ordinary text**, because clang
//     compiles it (the probe above): the eight subrs it defines exist in the reference
//     object, so the port defines them too.  `tools/extract-abi.py` decides `defsubr`
//     liveness with the frozen macro table (`c_defsubr_live`, *not* under
//     `blind_conditions`), so the table has to know `FONT_DEBUG`: the row
//     (`font.h<TAB>FONT_DEBUG`, measured by clang - it is `font.h` 997-998's own `#define`,
//     which config.h cannot state) landed with status 3.76, `check-cond-eval.py src/font.c`
//     is `45 conditions, 0 unknown, 0 disagreements [VERIFIED]`, and C 5938-5949's eight
//     `defsubr`s are transcribed at their C position in `syms_of_font` like the rest of the
//     arm.  font.c itself stays *out* of `cond-eval-verified.txt`: it carries both arms of
//     every `#if` (status 3.49/P-51), and a verified file's checker demands only the live
//     one (status 3.51).
//
// **Use-site declarations (mirror gaps).**  `emacs/src/font.h` has a WIP mirror
// (`src/include/font.rs`) that carries `enum font_property_index` (as constants), the
// four predicates and `struct font` (font.h 275-418, landed in this round — the five font
// units that used to keep copies of it read that declaration now); the records, accessors
// and macros this unit still needs are declared below with C's own text and a
// `/// C: font.h <lines>` line above each (PORTING.md 0.1-2/P-72 rule 3) — a mirror is not
// this unit's file to edit.
// `docs/ports/font.md` reports the whole list to the integrator.  What the same rule owes
// `dispextern.h`/`frame.h` (`struct face`, `struct face_cache`, the `LFACE_*` enum,
// `FRAME_FACE_CACHE`, `FACE_FROM_ID`, `FACE_FOR_CHAR`, `xstrcasecmp`) and `<intprops.h>`
// (`INT_BUFSIZE_BOUND`, P-26) is declared there too.  Every record's layout is measured
// with the frozen clang and pinned with `const _` assertions (PORTING.md 1.2/P-73).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unused_comparisons, overflowing_literals, unused_must_use, unreachable_code,
         unused_labels, clippy::all)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/float.in.rs imported via common
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/stdlib.rs imported via common
// [U1] ../lib/include/c-ctype.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] ../lib/include/stdcountof.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/character.rs imported via common
// [U1] ../lib/include/limits.in.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/blockinput.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/frame.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/charset.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/font.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/pdumper.rs imported via common
/* C 44: `#include TERM_HEADER`, inside `#ifdef HAVE_WINDOW_SYSTEM` - the five-way gate
   `src/fontset.rs` 75-85 and `src/dispnew.rs` 66-71 carry.  The frozen configuration leaves
   `TERM_HEADER` undefined, so no unit had the edge and every `FRAME_DISPLAY_INFO`-shaped
   expansion read `cannot find macro` on Android (measured).  */
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
const _: () = ();

/* ==========================================================================================
   `font.h`'s records, accessors and macros (the mirror gap; see the header note).  */

/// C: font.h 406-412: `enum font_spacing`, whose members C compares and stores as ints
/// (`XFIXNUM (val) <= FONT_SPACING_CHARCELL`, `make_fixnum (FONT_SPACING_MONO)`), so the
/// port keeps the C type as an int alias and the members as constants — the same shape the
/// mirror uses for `enum font_property_index`.
pub type font_spacing = c_int;


/// C: font.h 271-273: `#define FONT_ENCODING_NOT_DECIDED 255`.

/// C: font.h 572-574: `#define FONT_INVALID_CODE 0xFFFFFFFF`.

/// C: font.h 584-586: `#define FONT_PIXEL_SIZE_QUANTUM 1`.

/* font.h 561-579: `PT_PER_INCH` (the frozen arm) and the two conversions.  The mirror carries
   both (`include/font.rs` 245/246, C's own parenthesised bodies), so the unit's copies -
   which differed only by the mirror's outer parentheses - are retired (P-26).  */

/// C: font.h 193-237: the style accessors.  The three `*_SYMBOLIC` and three `*_FOR_FACE`
/// queries are the mirror's (C's own bodies, byte-identical); the three `*_NUMERIC` keep a
/// unit copy because **the mirror's body cannot expand**: it is C's ternary
/// `(FIXNUMP (AREF (font, I)) ? (XFIXNUM (…) >> 8) : -1)`, and Rust has no `? :` - reported to
/// the mirror's owner rather than changed there.
/// C: font.h 251-254: `FONT_SET_STYLE (font, prop, val)` is `ASET (font, prop, make_fixnum
/// (font_style_to_value (prop, val, true)))`.  The mirror's copy (`include/font.rs` 216) cannot
/// expand here: `ASET`'s index parameter is `isize` in this tree while `FONT_*_INDEX` is `c_int`
/// (E0308, `expected isize, found i32`), and `make_fixnum` wants the `EMACS_INT` cast below - so
/// C 244-246 / 1004-1013 / 552: the metric accessors, the two logging statement macros and
/// `XSETFONT` are the mirror's now - its bodies were C's own spelling (unexpandable Rust) until
/// status 3.328 and 3.333 ported them from this unit's copies, and the unit's copies are gone.

const _: () = core::assert! (std::mem::size_of::<font_metrics> () == 10);


const _: () = core::assert! (std::mem::size_of::<font_bitmap> () == 40);
const _: () = core::assert! (std::mem::offset_of! (font_bitmap, buffer) == 16);
const _: () = core::assert! (std::mem::offset_of! (font_bitmap, left) == 24);


const _: () = core::assert! (std::mem::size_of::<font_spec> () == 112);
const _: () = core::assert! (std::mem::offset_of! (font_spec, props) == 8);


/* `include/font.rs` 337-360 carries C 264-268's `#if defined HAVE_ANDROID && !defined
   ANDROID_STUBIFY` `bool is_android` member, so the record is 120 bytes on the frozen target and
   128 on Android (measured: docs/baseline/font-entity-layout-android.tsv).  */
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<font_entity> () == 120);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<font_entity> () == 128);
const _: () = core::assert! (std::mem::offset_of! (font_entity, props) == 8);
/* C: `struct font` (font.h 275-418) — the record this unit reads through `(*font).driver` —
   comes from `include/font.rs` now (docs/ports/include-font.md); the records the mirror
   still lacks (`struct font_driver`, `struct font_driver_list`, the accessors) stay here.  */


/* C 585-772's three `#ifdef HAVE_WINDOW_SYSTEM` members (`draw`, `get_bitmap`, `free_bitmap`)
   add 24 bytes and shift everything after `text_extents` by 24; the record is `include/font.rs`
   553's, so the numbers here are that record's two arms.  Measured:
   docs/baseline/font-driver-layout-android.tsv (232, `anchor_point`@136,
   `drop_xrender_surfaces`@224) and the frozen 208/112/200.  */
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::size_of::<font_driver> () == 208);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::size_of::<font_driver> () == 232);
const _: () = core::assert! (std::mem::offset_of! (font_driver, case_sensitive) == 8);
const _: () = core::assert! (std::mem::offset_of! (font_driver, get_cache) == 16);
const _: () = core::assert! (std::mem::offset_of! (font_driver, text_extents) == 104);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (font_driver, anchor_point) == 112);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (font_driver, anchor_point) == 136);
#[cfg (not (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (font_driver, drop_xrender_surfaces) == 200);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (font_driver, drop_xrender_surfaces) == 224);


const _: () = core::assert! (std::mem::size_of::<font_driver_list> () == 24);
const _: () = core::assert! (std::mem::offset_of! (font_driver_list, driver) == 8);
const _: () = core::assert! (std::mem::offset_of! (font_driver_list, next) == 16);














/// C: font.h 553-554: `#define XSETFONT(a, b) XSETPSEUDOVECTOR (a, b, PVEC_FONT)`.



/// C: font.h 894-904 declares `valid_font_driver` twice (the `ENABLE_CHECKING` prototype
/// and the `#else` INLINE returning true).  `font.c` *defines* it under `#ifdef
/// ENABLE_CHECKING`, which is undef on the frozen target, so that definition is carried
/// under the bare `#[cfg (enable_checking)]` pass (the bidi.rs/regex-emacs.rs key, P-51) and
/// nothing in this unit calls it; the header's own fallback is not declared here, because
/// the reference object emits no symbol for it either (`INLINE` is internal linkage).

/* ==========================================================================================
   `dispextern.h`/`frame.h`: the face records and the three macros this unit reads
   (`face->lface`, `face->font`, `FRAME_FACE_CACHE`, `FACE_FROM_ID`, `FACE_FOR_CHAR`).  */

/// C: dispextern.h 1770-1937: `struct face`.  Measured with the frozen clang: `sizeof` 280,
/// align 8, `lface`@0, `id`@160, `foreground`@168, `background`@176, `underline_color`@184,
/// `overline_color`@192, `strike_through_color`@200, `box_color`@208, `font`@216,
/// `fontset`@224, `box_vertical_line_width`@228, `box_horizontal_line_width`@232,
/// `underline_pixels_above_descent_line`@236, the 21 `bool_bf`/`ENUM_BF` bits C declares at
/// 1893-1935 in one 4-byte `int` unit at 240 (the port packs them into `bits`, PORTING.md
/// 1.3), `hash`@248, `next`@256, `prev`@264, `ascii_face`@272.  `HAVE_WINDOW_SYSTEM`
/// (`gc`, `stipple`), `HAVE_NS`, `HAVE_XFT`/`HAVE_FREETYPE` (`extra`) are undef, so those
/// members are absent from this layout.  Only `lface` and `font` are read by font.c.
#[repr(C)]
pub struct face
{
  pub lface: [Lisp_Object; LFACE_VECTOR_SIZE as usize],
  pub id: c_int,
  /* dispextern.h 1797-1811: `gc` and `stipple` exist only in a window-system build, and they
     push every later member by sixteen bytes.  Android has them, so the host-shape pins this
     copy carried (`foreground` 168, `font` 216, size 280) read the wrong members (status
     4.814).  */
  #[cfg (feature = "have_window_system")]
  pub gc: *mut c_void,        /// C: `struct android_gc *gc;` (1801) on this target
  #[cfg (feature = "have_window_system")]
  pub stipple: ptrdiff_t,     // 1810 (measured 176)
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
     underline : 3;` and the seventeen 1-bit `bool_bf` members in one storage unit.  */
  pub bits: u32,
  pub hash: uintptr_t,
  pub next: *mut face,
  pub prev: *mut face,
  pub ascii_face: *mut face,
}

const _: () = core::assert! (std::mem::size_of::<face> () == 280 + (if cfg! (feature = "have_window_system") { 16 } else { 0 }));
const _: () = core::assert! (std::mem::align_of::<face> () == 8);
const _: () = core::assert! (std::mem::offset_of! (face, id) == 160);
const _: () = core::assert! (std::mem::offset_of! (face, foreground) == 168 + (if cfg! (feature = "have_window_system") { 16 } else { 0 }));
const _: () = core::assert! (std::mem::offset_of! (face, font) == 216 + (if cfg! (feature = "have_window_system") { 16 } else { 0 }));
const _: () = core::assert! (std::mem::offset_of! (face, bits) == 240 + (if cfg! (feature = "have_window_system") { 16 } else { 0 }));
const _: () = core::assert! (std::mem::offset_of! (face, hash) == 248 + (if cfg! (feature = "have_window_system") { 16 } else { 0 }));
const _: () = core::assert! (std::mem::offset_of! (face, ascii_face) == 272 + (if cfg! (feature = "have_window_system") { 16 } else { 0 }));

/* The `face_box_type`/`face_underline_type` members and the `bool_bf` bits of `struct
   face`, in C's declaration order (dispextern.h 1893-1935).  The port's readers/writers go
   through these masked accessors, which is the P-80 convention for a `bool_bf` run.  */
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

/// C: dispextern.h 1991-2009: `struct face_cache`.  Measured: `sizeof` 40, `buckets`@0,
/// `f`@8, `faces_by_id`@16, `size`@24, `used`@32, and C's one `bool_bf
/// menu_face_changed_p : 1` in the int unit at 36 (carried as `menu_face_changed_p`, the
/// P-80 convention).
#[repr(C)]
pub struct face_cache
{
  pub buckets: *mut *mut face,
  pub f: *mut frame,
  pub faces_by_id: *mut *mut face,
  pub size: ptrdiff_t,
  pub used: c_int,
  pub menu_face_changed_p: bool,
}

const _: () = core::assert! (std::mem::size_of::<face_cache> () == 40);
const _: () = core::assert! (std::mem::offset_of! (face_cache, faces_by_id) == 16);
const _: () = core::assert! (std::mem::offset_of! (face_cache, used) == 32);

/// C: frame.h 1379: `#define FRAME_FACE_CACHE(F) (F)->face_cache`.  The `frame` mirror
/// carries the field as `*mut c_void` ("C: `struct face_cache *face_cache`") because
/// `struct face_cache` has no mirror yet, so the cast is the use-site conversion P-72
/// rule 3 prescribes.
macro_rules! FRAME_FACE_CACHE { ($f:expr) => { (*$f).face_cache as *mut face_cache }; }

/// C: frame.h 1645-1650: `FACE_FROM_ID`, whose body is the `eassert` plus the lookup.
#[inline(always)]
pub fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
{
  eassert! (0 <= id && id < unsafe { (*FRAME_FACE_CACHE! (f)).used });
  unsafe { *((*FRAME_FACE_CACHE! (f)).faces_by_id).offset (id as isize) }
}

/* ==========================================================================================
   `<intprops.h>` (no mirror, P-26): the two bounds `Ffont_xlfd_name` and
   `Fquery_font` use.  C's `INT_BUFSIZE_BOUND (t)` is `INT_STRLEN_BOUND (t) + 2` with
   `INT_STRLEN_BOUND (t)` the decimal-digit count of the type's largest value; the port
   keeps the same arithmetic on the frozen widths (int 10 digits, EMACS_INT 19).  */

macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }
macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }
macro_rules! INT_STRLEN_BOUND { ($t:ty) => { (10 + std::mem::size_of::<$t> () * 3) }; }
macro_rules! INT_BUFSIZE_BOUND { ($t:ty) => { INT_STRLEN_BOUND! ($t) + 2 }; }

/// C: dispextern.h 105-111: the case-insensitive comparison `font_unparse_fcname` and
/// `font_parse_fcname` call.
#[inline(always)]
pub unsafe fn xstrcasecmp (a: *const c_char, b: *const c_char) -> c_int
{
  let mut p = a;
  let mut q = b;
  while *p != 0 && *q != 0
    {
      if *p != *q
	{
	  let mut c1 = *p;
	  let mut c2 = *q;
	  if 97 <= c1 && c1 <= 122 { c1 -= 32; }
	  if 97 <= c2 && c2 <= 122 { c2 -= 32; }
	  let diff = (c1 as c_int) - (c2 as c_int);
	  if diff != 0 { return diff; }
	}
      p = p.offset (1);
      q = q.offset (1);
    }
  return (*p as c_int) - (*q as c_int);
}

/* The functions this unit calls that no mirror declares, each with C's own prototype
   (PORTING.md 0.1-2/P-72 rule 3).  */

/// C: dispextern.h 3828-3830: `face_at_string_position`, xfaces.c's.
unsafe extern "C" {
  /// C: dispextern.h 3824-3825: `int face_at_buffer_position (struct window *,
  /// ptrdiff_t, ptrdiff_t *, ptrdiff_t, bool, int, enum lface_attribute_index);`
  pub fn face_at_buffer_position (w: *mut window, pos: ptrdiff_t, endptr: *mut ptrdiff_t,
                                  limit: ptrdiff_t, mouse: bool, base_face_id: c_int,
                                  attr_filter: lface_attribute_index) -> c_int;
  /// C: dispextern.h 3828-3830: `int face_at_string_position (struct window *,
  /// Lisp_Object, ptrdiff_t, ptrdiff_t, ptrdiff_t *, enum face_id, bool,
  /// enum lface_attribute_index);`
  pub fn face_at_string_position (w: *mut window, string: Lisp_Object,
                                  bufpos: ptrdiff_t, bufpos_byte: ptrdiff_t,
                                  endptr: *mut ptrdiff_t, base_face_id: c_int,
                                  mouse_p: bool,
                                  attr_filter: lface_attribute_index) -> c_int;
}

/// C: dispextern.h 2029-2037: `FACE_FOR_CHAR`.  `HAVE_WINDOW_SYSTEM` is undef, so the
/// frozen build compiles the `#else` arm (`return face->id`); both arms are carried.
#[inline(always)]
pub fn FACE_FOR_CHAR (f: *mut frame, face: *mut face, character: c_int,
                      pos: ptrdiff_t, object: Lisp_Object) -> c_int
{
  #[cfg(feature = "have_window_system")]
  {
    return unsafe { face_for_char (f, face, character, pos, object) };
  }
  #[cfg(not (feature = "have_window_system"))]
  {
    let _ = (f, character, pos, object);
    return unsafe { (*face).id };
  }
}

/// C: dispextern.h 3811: `int face_for_char (struct frame *, struct face *, int,
/// ptrdiff_t, Lisp_Object);`, xfaces.c's — reached only through the `HAVE_WINDOW_SYSTEM`
/// arm of `FACE_FOR_CHAR` above.
#[cfg(feature = "have_window_system")]
unsafe extern "C" {
  /// C: dispextern.h 3811: `int face_for_char (struct frame *, struct face *, int,
  /// ptrdiff_t, Lisp_Object);`
  pub fn face_for_char (f: *mut frame, face: *mut face, character: c_int,
                        pos: ptrdiff_t, object: Lisp_Object) -> c_int;
}

/// C: xfaces.c 3379: `enum face_id lookup_basic_face (struct window *, struct frame *,
/// enum face_id);` and dispnew.c's `recompute_basic_faces`.
unsafe extern "C" {
  /// C: dispextern.h 3813: `int lookup_basic_face (struct window *, struct frame *, int);`
  pub fn lookup_basic_face (w: *mut window, f: *mut frame, face_id: c_int) -> c_int;
  /// C: dispextern.h 3823: `void recompute_basic_faces (struct frame *);`
  pub fn recompute_basic_faces (f: *mut frame);
}

/// `lisp.h`'s own declarations that the mirror does not carry (status 3.57 records
/// `make_specified_string` as a known `lisp.rs` gap), plus `<stdlib.h>`'s `atoi`, which
/// `lib/include/stdlib.rs` does not declare either.
unsafe extern "C" {

  /// C: lisp.h 4077-4079: `extern Lisp_Object make_specified_string (const char *,
  /// ptrdiff_t, ptrdiff_t, bool);`
  pub fn make_specified_string (contents: *const c_char, nchars: ptrdiff_t,
                                nbytes: ptrdiff_t, multibyte: bool) -> Lisp_Object;
  /// C: `<stdlib.h>`: `int atoi (const char *);`
  pub fn atoi (nptr: *const c_char) -> c_int;
}

/// `frame.h`/`window.h`/`character.h`/`lisp.h`/`<stdio.h>` names this unit calls that no
/// mirror carries.
unsafe extern "C" {
  // (window.h 1210's `decode_live_window` is the window.rs mirror's now; the use-site copy is gone, P-26.)
  /// C: character.h 149-150: `extern void parse_str_as_multibyte (const unsigned char *,
  /// ptrdiff_t, ptrdiff_t *, ptrdiff_t *);`
  pub fn parse_str_as_multibyte (str_: *const c_uchar, len: ptrdiff_t,
                                 nchars: *mut ptrdiff_t, nbytes: *mut ptrdiff_t);

  // C: `<stdio.h>`: `int snprintf (char *, size_t, const char *, ...);` — the mirror
  // `lib/include/stdio.rs` carries it as of status 3.263, so this unit no longer declares
  // it itself (and the declaration that stood here was what made the GLYPH_DEBUG arms
  // uncompilable everywhere else).
}

/// C: frame.h 1317-1320: `INLINE void CHECK_FRAME (Lisp_Object x)`, which frame.h writes
/// with `CHECK_TYPE (FRAMEP (x), Qframe, x)` (the `font.rs` mirror's predicate family).
#[inline(always)]
pub fn CHECK_FRAME (x: Lisp_Object) { CHECK_TYPE (FRAMEP (x), Qframe (), x); }

/// C: frame.h 1214: `extern struct frame *decode_window_system_frame (Lisp_Object);`
/// (frame.c), reached only through `Ffont_info`'s `HAVE_WINDOW_SYSTEM` arm.
#[cfg(feature = "have_window_system")]
unsafe extern "C" {
  /// C: frame.h 1214: `extern struct frame *decode_window_system_frame (Lisp_Object);`
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
}

/// `composite.h` 335-340 and `lisp.h`'s `larger_vector` (a known `lisp.rs` gap,
/// status 3.57): the mirror's `composite_decls` module carries `composition_gstring_lookup_cache`
/// and `composition_gstring_cache_clear_font` only, and `lisp.rs` does not declare
/// `larger_vector` at all.
unsafe extern "C" {
  // composite.h 335's `composition_gstring_put_cache` is the include/composite.rs mirror's
  // now (status 3.363).

}

/// `frame.h`/`fontset.h`/`<string.h>` names this unit calls that no mirror carries.
unsafe extern "C" {

  /// C: `<string.h>`: `char *strstr (const char *, const char *);`
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

/// C: fontset.h 40, 44: `int fs_query_fontset (Lisp_Object, int);` and
/// `Lisp_Object fontset_ascii (int);` — `emacs/src/fontset.h` has no mirror at all, and the
/// two are reached only through `Ffont_face_attributes`'s `HAVE_WINDOW_SYSTEM` arm, which is
/// carried, so they are declared under the same feature.
#[cfg(feature = "have_window_system")]
unsafe extern "C" {
  /// C: fontset.h 40: `extern int fs_query_fontset (Lisp_Object, int);`
  pub fn fs_query_fontset (name: Lisp_Object, name_byte: c_int) -> c_int;
  /// C: fontset.h 44: `extern Lisp_Object fontset_ascii (int);`
  pub fn fontset_ascii (fontset: c_int) -> Lisp_Object;
}

/// C: font.h 868-869: `extern Lisp_Object get_font_ascent_descent (...)` — actually
/// `dispextern.h` 3631's `extern void get_font_ascent_descent (struct font *, int *,
/// int *);`, which `src/include/composite.rs` carries only inside the composite.c unit
/// (that unit's own use-site block), not in the mirror this unit includes.
unsafe extern "C" {
  /// C: dispextern.h 3631: `extern void get_font_ascent_descent (struct font *, int *,
  /// int *);`
  pub fn get_font_ascent_descent (font: *mut font, ascent: *mut c_int,
                                  descent: *mut c_int);
}

/* C: font.h 683-684 only *points* at `struct glyph_string`; its record is dispextern.h's and
   `include/dispextern.rs` 1614 carries it under `have_window_system`.  An opaque placeholder here
   was `E0428 glyph_string` on Android, where that arm is live (measured).  */

/// `lisp.h`'s `assoc_no_quit`, which the mirror does not carry.
unsafe extern "C" {

}

/// Every `(*driver).F.unwrap () (…)` below answers a driver method C calls unconditionally
/// (`struct font_driver`'s own comments mark the optional methods; PORTING.md 0.1-2 keeps
/// the field as `Option` because a C driver may leave it NULL and C's null checks are on the
/// *optional* members).  The node the port unwraps is the same node C dereferences, and
/// `docs/ports/font.md` records that no gate can see this (the atimer.rs shape).

/// C: `<stdlib.h>` 489: `extern void qsort (void *__base, size_t __nmemb, size_t __size,
/// __compar_fn_t __compar);` — the port's `stdlib.rs` mirror does not carry it (the
/// buffer.rs shape).
unsafe extern "C" {
  /// C: `<stdlib.h>` 489: `extern void qsort (void *__base, size_t __nmemb, size_t __size,
  /// __compar_fn_t __compar);`
  pub fn qsort (base: *mut c_void, nmemb: usize, size: usize,
                compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);
}

/// C: termhooks.h 968: `extern Lisp_Object terminal_glyph_code (struct terminal *, int);`,
/// term.c's.
unsafe extern "C" {
  /// C: termhooks.h 968: `extern Lisp_Object terminal_glyph_code (struct terminal *, int);`
  pub fn terminal_glyph_code (t: *mut terminal, codepoint: c_int) -> Lisp_Object;
}

/* ==========================================================================================
   C 47-154: the file-scope objects and the checking-mode driver check.  */

/* C 47: `#define DEFAULT_ENCODING Qiso8859_1`.  */
macro_rules! DEFAULT_ENCODING { () => { Qiso8859_1 () }; }

/* Vector of Vfont_weight_table, Vfont_slant_table, and Vfont_width_table. */
static mut font_style_table: Lisp_Object = lisp_h_Qnil ();

/* Structure used for tables mapping weight, slant, and width numeric
   values and their names.  */

#[repr(C)]
pub struct table_entry
{
  pub numeric: c_int,
  /* The first one is a valid name as a face attribute.
     The second one (if any) is a typical name in XLFD field.  */
  pub names: [*const c_char; 6],
}

/* The following tables should be in sync with 'custom-face-attributes'.  */

/* Table of weight numeric values and their names.  This table must be
   sorted by numeric values in ascending order and the numeric values
   must approximately match the weights in the font files.  */

/// Rust requires an immutable `static`'s type to be `Sync`, and a raw pointer is not; C's
/// `struct table_entry` is immutable data (the `names` point at string literals in this
/// unit), so the port states that with an explicit impl — the same reason the tables below
/// stay ordinary `static`s rather than becoming `static mut`.
unsafe impl Sync for table_entry {}

/// C's aggregate initialisation leaves the trailing `names` entries NULL; the port spells
/// the nulls out (`NIL_NAME`) because Rust has no zero-fill for an array member.
const NIL_NAME: *const c_char = std::ptr::null ();

static weight_table: [table_entry; 11] =
[
  table_entry { numeric: 0, names: [c"thin".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 40, names: [c"ultra-light".as_ptr (), c"ultralight".as_ptr (), c"extra-light".as_ptr (), c"extralight".as_ptr (), NIL_NAME, NIL_NAME] },
  table_entry { numeric: 50, names: [c"light".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 55, names: [c"semi-light".as_ptr (), c"semilight".as_ptr (), c"demilight".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 80, names: [c"regular".as_ptr (), c"normal".as_ptr (), c"unspecified".as_ptr (), c"book".as_ptr (), NIL_NAME, NIL_NAME] },
  table_entry { numeric: 100, names: [c"medium".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 180, names: [c"semi-bold".as_ptr (), c"semibold".as_ptr (), c"demibold".as_ptr (), c"demi-bold".as_ptr (), c"demi".as_ptr (), NIL_NAME] },
  table_entry { numeric: 200, names: [c"bold".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 205, names: [c"extra-bold".as_ptr (), c"extrabold".as_ptr (), c"ultra-bold".as_ptr (), c"ultrabold".as_ptr (), NIL_NAME, NIL_NAME] },
  table_entry { numeric: 210, names: [c"black".as_ptr (), c"heavy".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 250, names: [c"ultra-heavy".as_ptr (), c"ultraheavy".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] }
];

/* Table of slant numeric values and their names.  This table must be
   sorted by numeric values in ascending order.  */

static slant_table: [table_entry; 5] =
[
  table_entry { numeric: 0, names: [c"reverse-oblique".as_ptr (), c"ro".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 10, names: [c"reverse-italic".as_ptr (), c"ri".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 100, names: [c"normal".as_ptr (), c"r".as_ptr (), c"unspecified".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 200, names: [c"italic".as_ptr (), c"i".as_ptr (), c"ot".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 210, names: [c"oblique".as_ptr (), c"o".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] }
];

/* Table of width numeric values and their names.  This table must be
   sorted by numeric values in ascending order.  */

static width_table: [table_entry; 9] =
[
  table_entry { numeric: 50, names: [c"ultra-condensed".as_ptr (), c"ultracondensed".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 63, names: [c"extra-condensed".as_ptr (), c"extracondensed".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 75, names: [c"condensed".as_ptr (), c"compressed".as_ptr (), c"narrow".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 87, names: [c"semi-condensed".as_ptr (), c"semicondensed".as_ptr (), c"demicondensed".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 100, names: [c"normal".as_ptr (), c"medium".as_ptr (), c"regular".as_ptr (), c"unspecified".as_ptr (), NIL_NAME, NIL_NAME] },
  table_entry { numeric: 113, names: [c"semi-expanded".as_ptr (), c"semiexpanded".as_ptr (), c"demiexpanded".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 125, names: [c"expanded".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 150, names: [c"extra-expanded".as_ptr (), c"extraexpanded".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME, NIL_NAME] },
  table_entry { numeric: 200, names: [c"ultra-expanded".as_ptr (), c"ultraexpanded".as_ptr (), c"wide".as_ptr (), NIL_NAME, NIL_NAME, NIL_NAME] }
];

/* Alist of font registry symbols and the corresponding charset
   information.  The information is retrieved from
   Vfont_encoding_alist on demand.

   Eash element has the form:
	(REGISTRY . (ENCODING-CHARSET-ID . REPERTORY-CHARSET-ID))
   or
	(REGISTRY . nil)

   In the former form, ENCODING-CHARSET-ID is an ID of a charset that
   encodes a character code to a glyph code of a font, and
   REPERTORY-CHARSET-ID is an ID of a charset that tells if a
   character is supported by a font.

   The latter form means that the information for REGISTRY couldn't be
   retrieved.  */
static mut font_charset_alist: Lisp_Object = lisp_h_Qnil ();

/* List of all font drivers.  Each font-backend (XXXfont.c) calls
   register_font_driver in syms_of_XXXfont to register its font-driver
   here.  */
static mut font_driver_list: *mut font_driver_list = std::ptr::null_mut ();

#[cfg (enable_checking)]

/* Used to catch bogus pointers in font objects.  */

/* C's shape here is font.h's prototype plus this one definition; the mirror therefore does
   **not** declare the checking build's prototype (status 3.225) - the units that call it declare
   it themselves (the `mark_haiku_display` shape), because a declaration and a definition of the
   same Rust name in one crate is `E0428` (measured: `rustc --cfg enable_checking` on this unit's
   root).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn valid_font_driver (drv: *const font_driver) -> bool
{
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut fdl: *mut font_driver_list;

  fdl = font_driver_list;
  while !fdl.is_null ()
    {
      if (*fdl).driver == drv
	{ return true; }
      fdl = (*fdl).next;
    }
  FOR_EACH_FRAME! (tail, frame, {
    fdl = (*XFRAME (frame)).font_driver_list as *mut font_driver_list;
    while !fdl.is_null ()
      {
	if (*fdl).driver == drv
	  { return true; }
	fdl = (*fdl).next;
      }
  });
  return false;
}

/* Creators of font-related Lisp object.  */

unsafe fn font_make_spec () -> Lisp_Object
{
  let mut font_spec: Lisp_Object;
  let spec: *mut font_spec
    = allocate_pseudovector (VECSIZE (std::mem::size_of::<font_spec> ()) as c_int,
			     FONT_SPEC_MAX, FONT_SPEC_MAX,
			     pvec_type::PVEC_FONT) as *mut font_spec;
  XSETFONT! (font_spec, spec);
  return font_spec;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_make_entity () -> Lisp_Object
{
  let mut font_entity: Lisp_Object;
  let entity: *mut font_entity
    = allocate_pseudovector (VECSIZE (std::mem::size_of::<font_entity> ()) as c_int,
			     FONT_ENTITY_MAX, FONT_ENTITY_MAX,
			     pvec_type::PVEC_FONT) as *mut font_entity;
  XSETFONT! (font_entity, entity);

  /* C 181-182: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` (both undef on the
     frozen target), carried under the feature that names the condition's other side.  */
  #[cfg(feature = "have_android")]
  { (*entity).is_android = false; }

  return font_entity;
}

/* C 187: `#ifdef HAVE_ANDROID` (undef in the frozen configuration), carried.  */

#[cfg(feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_make_entity_android (size: c_int) -> Lisp_Object
{
  let mut font_entity: Lisp_Object;
  let entity: *mut font_entity
    = allocate_pseudovector (size, FONT_ENTITY_MAX, FONT_ENTITY_MAX,
			     pvec_type::PVEC_FONT) as *mut font_entity;

  /* C 199-201: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY`, carried under the
     same feature as `font_make_entity`'s.  */
  (*entity).is_android = true;

  XSETFONT! (font_entity, entity);
  return font_entity;
}

/* Create a font-object whose structure size is SIZE.  If ENTITY is
   not nil, copy properties from ENTITY to the font-object.  If
   PIXELSIZE is positive, set the `size' property to PIXELSIZE.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_make_object (size: c_int, entity: Lisp_Object,
					   pixelsize: c_int) -> Lisp_Object
{
  let mut font_object: Lisp_Object;
  let font: *mut font
    = allocate_pseudovector (size, FONT_OBJECT_MAX, FONT_OBJECT_MAX,
			     pvec_type::PVEC_FONT) as *mut font;
  let mut i: c_int;

  /* Poison the max_width, so we can detect when it hasn't been set.  */
  eassert! ({ (*font).max_width = 1024 * 1024 * 1024; true });

  /* GC can happen before the driver is set up,
     so avoid dangling pointer here (Bug#17771).  */
  (*font).driver = std::ptr::null ();
  XSETFONT! (font_object, font);

  if ! NILP (entity)
    {
      i = 1;
      while i < FONT_SPEC_MAX
	{
	  (*font).props[i as usize] = AREF (entity, i as isize);
	  i += 1;
	}
      if ! NILP (AREF (entity, FONT_EXTRA_INDEX as isize))
	{
	  (*font).props[FONT_EXTRA_INDEX as usize]
	    = Fcopy_alist (AREF (entity, FONT_EXTRA_INDEX as isize));
	}
    }
  if size > 0
    { (*font).props[FONT_SIZE_INDEX as usize] = make_fixnum (pixelsize as EMACS_INT); }
  return font_object;
}

/* C 240: `#if defined (HAVE_XFT) || defined (HAVE_FREETYPE) || defined (HAVE_NS)` (all
   undef in the frozen configuration), carried.  */

/* C 242: the `static int font_unparse_fcname (Lisp_Object, int, char *, int);` forward
   declaration is dropped (PORTING.md 0.1-12); its definition is C 1882's.  */

/* Like above, but also set `type', `name' and `fullname' properties
   of font-object.  */

#[cfg(feature = "xft_or_freetype_or_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_build_object (vectorsize: c_int, r#type: Lisp_Object,
					    entity: Lisp_Object,
					    pixelsize: c_double) -> Lisp_Object
{
  let mut len: c_int;
  let mut name: [c_char; 256] = [0; 256];
  let mut xlfd_name: *mut c_char;
  let mut font_object: Lisp_Object;

  font_object = font_make_object (vectorsize, entity, pixelsize as c_int);

  ASET (font_object, FONT_TYPE_INDEX as isize, r#type);
  xlfd_name = font_dynamic_unparse_xlfd (entity, pixelsize as c_int);

  if !xlfd_name.is_null ()
    {
      ASET (font_object, FONT_NAME_INDEX as isize, build_string (xlfd_name));
      xfree (xlfd_name as *mut c_void);
    }

  len = font_unparse_fcname (entity, pixelsize as c_int, name.as_mut_ptr (),
			     std::mem::size_of::<[c_char; 256]> () as c_int);
  if len > 0
    { ASET (font_object, FONT_FULLNAME_INDEX as isize,
	    make_string (name.as_ptr (), len as ptrdiff_t)); }
  else
    { ASET (font_object, FONT_FULLNAME_INDEX as isize,
	    AREF (font_object, FONT_NAME_INDEX as isize)); }
  return font_object;
}

/* C 280-281: the two `static` forward declarations (`font_unparse_fcname`,
   `font_pixel_size`/`font_open_entity`/`font_matching_entity`/`font_encode_char`) are
   dropped (PORTING.md 0.1-12); their definitions follow.  */

/* Number of registered font drivers.  */
static mut num_font_drivers: c_int = 0;

/* Return a Lispy value of a font property value at STR and LEN bytes.
   If STR is "*", return nil.  If FORCE_SYMBOL, or if STR does not
   consist entirely of one or more digits, return a symbol interned
   from STR.  Otherwise, return an integer.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_intern_prop (str_: *const c_char, len: ptrdiff_t,
					   force_symbol: bool) -> Lisp_Object
{
  let mut i: ptrdiff_t;
  let mut nbytes: ptrdiff_t = 0;
  let mut nchars: ptrdiff_t = 0;
  let mut tem: Lisp_Object;
  let mut name: Lisp_Object;
  let mut obarray: Lisp_Object;

  if len == 1 && *str_ == b'*' as c_char
    { return Qnil (); }
  if !force_symbol && 0 < len && b'0' as c_char <= *str_ && *str_ <= b'9' as c_char
    {
      i = 1;
      while i < len
	{
	  if ! (b'0' as c_char <= *str_.offset (i as isize)
		&& *str_.offset (i as isize) <= b'9' as c_char)
	    { break; }
	  i += 1;
	}
      if i == len
	{
	  i = 0;
	  let mut n: EMACS_INT = 0;
	  loop
	    {
	      /* C 326: `n += str[i++] - '0'` — the loop's own increment folded into the
		 assignment, as C's `for` clause writes it.  */
	      n += (*str_.offset (i as isize) - b'0' as c_char) as EMACS_INT;
	      i += 1;
	      if ! (n <= MOST_POSITIVE_FIXNUM)
		{ break; }
	      if i == len
		{ return make_fixnum (n); }
	      /* C's `ckd_mul (&n, n, 10)` (P-15: the `overflowing_*` pair).  */
	      let (n_, mul_overflow) = n.overflowing_mul (10);
	      n = n_;
	      if mul_overflow
		{ break; }
	    }

	  xsignal1 (Qoverflow_error (), make_string (str_, len));
	}
    }

  /* This code is similar to intern function from lread.c.  */
  obarray = check_obarray (Vobarray);
  parse_str_as_multibyte (str_ as *const c_uchar, len, &mut nchars, &mut nbytes);
  tem = oblookup (obarray, str_,
		  if len == nchars || len != nbytes { len } else { nchars }, len);
  if SYMBOLP (tem)
    { return tem; }
  name = make_specified_string (str_, nchars, len,
				len != nchars && len == nbytes);
  return intern_driver (name, obarray, tem);
}

/* Return a pixel size of font-spec SPEC on frame F.  */

unsafe fn font_pixel_size (f: *mut frame, spec: Lisp_Object) -> c_int
{
  #[cfg(feature = "have_window_system")]
  {
    let size: Lisp_Object = AREF (spec, FONT_SIZE_INDEX as isize);
    let mut point_size: c_double;
    let mut dpi: c_int;
    let mut pixel_size: c_int;
    let mut val: Lisp_Object;

    if FIXNUMP (size)
      { return XFIXNUM (size) as c_int; }
    if NILP (size)
      { return 0; }
    if FRAME_WINDOW_P (f)
      {
	eassert! (FLOATP (size));
	point_size = XFLOAT_DATA (size);
	val = AREF (spec, FONT_DPI_INDEX as isize);
	if FIXNUMP (val)
	  { dpi = XFIXNUM (val) as c_int; }
	else
	  { dpi = FRAME_RES (f) as c_int; }
	pixel_size = POINT_TO_PIXEL! (point_size, dpi as c_double) as c_int;
	return pixel_size;
      }
  }
  return 1;
}

/* Return a value of PROP's VAL (symbol or integer) to be stored in a
   font vector.  If VAL is not valid (i.e. not registered in
   font_style_table), return -1 if NOERROR is zero, and return a
   proper index if NOERROR is nonzero.  In that case, register VAL in
   font_style_table if VAL is a symbol, and return the closest index if
   VAL is an integer.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_style_to_value (prop: font_property_index, val: Lisp_Object,
					      noerror: bool) -> c_int
{
  let table: Lisp_Object = AREF (font_style_table, (prop - FONT_WEIGHT_INDEX) as isize);
  let mut len: c_int;

  CHECK_VECTOR (table);
  len = ASIZE (table) as c_int;

  if SYMBOLP (val)
    {
      let mut i: c_int;
      let mut j: c_int;
      let mut s: *const c_char;
      let mut elt: Lisp_Object;

      /* At first try exact match.  */
      i = 0;
      while i < len
	{
	  CHECK_VECTOR (AREF (table, i as isize));
	  j = 1;
	  while j < ASIZE (AREF (table, i as isize)) as c_int
	    {
	      if EQ (val, AREF (AREF (table, i as isize), j as isize))
		{
		  CHECK_FIXNUM (AREF (AREF (table, i as isize), 0));
		  return ((XFIXNUM (AREF (AREF (table, i as isize), 0)) << 8) as c_int)
			 | (i << 4) | (j - 1);
		}
	      j += 1;
	    }
	  i += 1;
	}
      /* Try also with case-folding match.  */
      s = SSDATA (SYMBOL_NAME (val));
      i = 0;
      while i < len
	{
	  j = 1;
	  while j < ASIZE (AREF (table, i as isize)) as c_int
	    {
	      elt = AREF (AREF (table, i as isize), j as isize);
	      if xstrcasecmp (s, SSDATA (SYMBOL_NAME (elt))) == 0
		{
		  CHECK_FIXNUM (AREF (AREF (table, i as isize), 0));
		  return ((XFIXNUM (AREF (AREF (table, i as isize), 0)) << 8) as c_int)
			 | (i << 4) | (j - 1);
		}
	      j += 1;
	    }
	  i += 1;
	}
      if ! noerror
	{ return -1; }
      eassert! (len < 255);
      elt = make_vector (2, make_fixnum (100));
      ASET (elt, 1, val);
      let new_table: Lisp_Object = CALLN! (Fvconcat, table, make_vector (1, elt));
      /* Update the corresponding variable with the new value of the
	 modified slot of font_style_table.  */
      match prop
	{
	  FONT_WEIGHT_INDEX => { Vfont_weight_table = new_table; }
	  FONT_SLANT_INDEX => { Vfont_slant_table = new_table; }
	  FONT_WIDTH_INDEX => { Vfont_width_table = new_table; }
	  _ => { }
	}
      ASET (font_style_table, (prop - FONT_WEIGHT_INDEX) as isize, new_table);
      return (100 << 8) | (i << 4);
    }
  else
    {
      let mut i: c_int;
      let mut last_n: c_int;
      let numeric: EMACS_INT = XFIXNUM (val);

      i = 0;
      last_n = -1;
      while i < len
	{
	  let mut n: c_int;

	  CHECK_VECTOR (AREF (table, i as isize));
	  CHECK_FIXNUM (AREF (AREF (table, i as isize), 0));
	  n = XFIXNUM (AREF (AREF (table, i as isize), 0)) as c_int;
	  if numeric == n as EMACS_INT
	    { return (n << 8) | (i << 4); }
	  if numeric < n as EMACS_INT
	    {
	      if ! noerror
		{ return -1; }
	      return if i == 0 || (n - numeric as c_int) < (numeric as c_int - last_n)
		     { (n << 8) | (i << 4) } else { (last_n << 8) | ((i - 1) << 4) };
	    }
	  last_n = n;
	  i += 1;
	}
      if ! noerror
	{ return -1; }
      return (last_n << 8) | ((i - 1) << 4);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_style_symbolic (font: Lisp_Object, prop: font_property_index,
					      for_face: bool) -> Lisp_Object
{
  let mut val: Lisp_Object = AREF (font, prop as isize);
  let mut table: Lisp_Object;
  let mut elt: Lisp_Object;
  let mut i: c_int;

  if NILP (val)
    { return Qnil (); }
  table = AREF (font_style_table, (prop - FONT_WEIGHT_INDEX) as isize);
  CHECK_VECTOR (table);
  i = XFIXNUM (val) as c_int & 0xFF;
  eassert! (((i >> 4) & 0xF) < ASIZE (table) as c_int);
  elt = AREF (table, ((i >> 4) & 0xF) as isize);
  CHECK_VECTOR (elt);
  eassert! ((i & 0xF) + 1 < ASIZE (elt) as c_int);
  elt = if for_face { AREF (elt, 1) } else { AREF (elt, ((i & 0xF) + 1) as isize) };
  CHECK_SYMBOL (elt);
  return elt;
}

/* Return ENCODING or a cons of ENCODING and REPERTORY of the font
   FONTNAME.  ENCODING is a charset symbol that specifies the encoding
   of the font.  REPERTORY is a charset symbol or nil.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_font_encoding (fontname: Lisp_Object) -> Lisp_Object
{
  let mut tail: Lisp_Object;
  let mut elt: Lisp_Object;

  tail = Vfont_encoding_alist;
  while CONSP (tail)
    {
      elt = XCAR (tail);
      if CONSP (elt)
	  && STRINGP (XCAR (elt))
	  && fast_string_match_ignore_case (XCAR (elt), fontname) >= 0
	  && (if SYMBOLP (XCDR (elt))
		{ CHARSETP! (XCDR (elt)) }
		else
		{ CONSP (XCDR (elt)) && CHARSETP! (XCAR (XCDR (elt))) })
	{ return XCDR (elt); }
      tail = XCDR (tail);
    }
  return Qnil ();
}

/* Return encoding charset and repertory charset for REGISTRY in
   ENCODING and REPERTORY correspondingly.  If correct information for
   REGISTRY is available, return 0.  Otherwise return -1.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_registry_charsets (registry: Lisp_Object,
						 encoding: *mut *mut charset,
						 repertory: *mut *mut charset) -> c_int
{
  let mut val: Lisp_Object;
  let mut encoding_id: c_int;
  let mut repertory_id: c_int;

  'block: {
  val = Fassoc_string (registry, font_charset_alist, Qt ());
  if ! NILP (val)
    {
      val = XCDR (val);
      if NILP (val)
	{ return -1; }
      encoding_id = XFIXNUM (XCAR (val)) as c_int;
      repertory_id = XFIXNUM (XCDR (val)) as c_int;
    }
  else
    {
      val = find_font_encoding (SYMBOL_NAME (registry));
      if SYMBOLP (val) && CHARSETP! (val)
	{
	  encoding_id = XFIXNUM (CHARSET_SYMBOL_ID! (val)) as c_int;
	  repertory_id = encoding_id;
	}
      else if CONSP (val)
	{
	  if ! CHARSETP! (XCAR (val))
	    { break 'block; }
	  encoding_id = XFIXNUM (CHARSET_SYMBOL_ID! (XCAR (val))) as c_int;
	  if NILP (XCDR (val))
	    { repertory_id = -1; }
	  else
	    {
	      if ! CHARSETP! (XCDR (val))
		{ break 'block; }
	      repertory_id = XFIXNUM (CHARSET_SYMBOL_ID! (XCDR (val))) as c_int;
	    }
	}
      else
	{ break 'block; }
      val = Fcons (make_fixnum (encoding_id as EMACS_INT),
		   make_fixnum (repertory_id as EMACS_INT));
      font_charset_alist
	= nconc2 (font_charset_alist, list1 (Fcons (registry, val)));
    }

  if !encoding.is_null ()
    { *encoding = CHARSET_FROM_ID! (encoding_id); }
  if !repertory.is_null ()
    { *repertory = if repertory_id >= 0 { CHARSET_FROM_ID! (repertory_id) }
		   else { std::ptr::null_mut () }; }
  return 0;
  }

  /* invalid_entry: C's forward `goto` lands here (PORTING.md 2.2).  */
  font_charset_alist
    = nconc2 (font_charset_alist, list1 (Fcons (registry, Qnil ())));
  return -1;
}

/* Font property value validators.  See the comment of
   font_property_table for the meaning of the arguments.  */

unsafe extern "C" fn font_prop_validate_symbol (prop: Lisp_Object, val: Lisp_Object)
						-> Lisp_Object
{
  let mut val: Lisp_Object = val;
  if STRINGP (val)
    { val = Fintern (val, Qnil ()); }
  if ! SYMBOLP (val)
    { val = Qerror (); }
  else if EQ (prop, QCregistry ())
    { val = Fintern (Fdowncase (SYMBOL_NAME (val)), Qnil ()); }
  return val;
}

unsafe extern "C" fn font_prop_validate_style (style: Lisp_Object, val: Lisp_Object)
					       -> Lisp_Object
{
  let prop: font_property_index = if EQ (style, QCweight ()) { FONT_WEIGHT_INDEX }
				  else if EQ (style, QCslant ()) { FONT_SLANT_INDEX }
				  else { FONT_WIDTH_INDEX };
  let mut val: Lisp_Object = val;
  if FIXNUMP (val)
    {
      let n: EMACS_INT = XFIXNUM (val);
      CHECK_VECTOR (AREF (font_style_table, (prop - FONT_WEIGHT_INDEX) as isize));
      if (((n >> 4) & 0xF) as isize
	  >= ASIZE (AREF (font_style_table, (prop - FONT_WEIGHT_INDEX) as isize)))
	{ val = Qerror (); }
      else
	{
	  let mut elt: Lisp_Object
	    = AREF (AREF (font_style_table, (prop - FONT_WEIGHT_INDEX) as isize),
		    ((n >> 4) & 0xF) as isize);

	  CHECK_VECTOR (elt);
	  if ((n & 0xF) + 1) as isize >= ASIZE (elt)
	    { val = Qerror (); }
	  else
	    {
	      CHECK_FIXNUM (AREF (elt, 0));
	      if XFIXNUM (AREF (elt, 0)) != (n >> 8)
		{ val = Qerror (); }
	    }
	}
    }
  else if SYMBOLP (val)
    {
      let n: c_int = font_style_to_value (prop, val, false);

      val = if n >= 0 { make_fixnum (n as EMACS_INT) } else { Qerror () };
    }
  else
    { val = Qerror (); }
  return val;
}

unsafe extern "C" fn font_prop_validate_non_neg (prop: Lisp_Object, val: Lisp_Object)
						 -> Lisp_Object
{
  let _ = prop;
  return if FIXNATP (val) || (FLOATP (val) && XFLOAT_DATA (val) >= 0.0)
	 { val } else { Qerror () };
}

unsafe extern "C" fn font_prop_validate_spacing (prop: Lisp_Object, val: Lisp_Object)
						 -> Lisp_Object
{
  let _ = prop;
  if NILP (val) || (FIXNATP (val) && XFIXNUM (val) <= FONT_SPACING_CHARCELL as EMACS_INT)
    { return val; }
  if SYMBOLP (val) && SBYTES (SYMBOL_NAME (val)) == 1
    {
      let spacing: u8 = *SDATA (SYMBOL_NAME (val));

      if spacing == b'c' || spacing == b'C'
	{ return make_fixnum (FONT_SPACING_CHARCELL as EMACS_INT); }
      if spacing == b'm' || spacing == b'M'
	{ return make_fixnum (FONT_SPACING_MONO as EMACS_INT); }
      if spacing == b'p' || spacing == b'P'
	{ return make_fixnum (FONT_SPACING_PROPORTIONAL as EMACS_INT); }
      if spacing == b'd' || spacing == b'D'
	{ return make_fixnum (FONT_SPACING_DUAL as EMACS_INT); }
    }
  return Qerror ();
}

unsafe extern "C" fn font_prop_validate_otf (prop: Lisp_Object, val: Lisp_Object)
					     -> Lisp_Object
{
  let mut tail: Lisp_Object;
  let mut tmp: Lisp_Object;
  let mut i: c_int;

  let _ = prop;
  /* VAL = (SCRIPT [ LANGSYS [ GSUB-FEATURES [ GPOS-FEATURES ]]])
     GSUB-FEATURES = (FEATURE ... [ nil FEATURE ... ]) | nil
     GPOS-FEATURES = (FEATURE ... [ nil FEATURE ... ]) | nil  */
  if ! CONSP (val)
    { return Qerror (); }
  if ! SYMBOLP (XCAR (val))
    { return Qerror (); }
  tail = XCDR (val);
  if NILP (tail)
    { return val; }
  if ! CONSP (tail) || ! SYMBOLP (XCAR (val))
    { return Qerror (); }
  i = 0;
  while i < 2
    {
      tail = XCDR (tail);
      if NILP (tail)
	{ return val; }
      if ! CONSP (tail)
	{ return Qerror (); }
      tmp = XCAR (tail);
      while CONSP (tmp)
	{
	  if ! SYMBOLP (XCAR (tmp))
	    { return Qerror (); }
	  tmp = XCDR (tmp);
	}
      if ! NILP (tmp)
	{ return Qerror (); }
      i += 1;
    }
  return val;
}

/* Structure of known font property keys and validator of the
   values.  */
/// C: font.c 683-693: the anonymous `struct { int key; Lisp_Object (*validator)
/// (Lisp_Object prop, Lisp_Object val); }` C declares for `font_property_table`.  Rust
/// needs a name for it; the C text is kept in the doc comment above the fields.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct font_property_table_entry
{
  /* Index of the key symbol.  */
  pub key: c_int,
  /* Function to validate PROP's value VAL, or NULL if any value is
     ok.  The value is VAL or its regularized value if VAL is valid,
     and Qerror if not.  */
  pub validator: unsafe extern "C" fn (Lisp_Object, Lisp_Object) -> Lisp_Object,
}

/* C's `SYMBOL_INDEX (QCtype)` is a compile-time integer constant; the port's `SYMBOL_INDEX`
   is a function over the generated symbol table, which a static initialiser cannot call, so
   each `key` is the generated index constant (`globals.rs`'s `iQ…`) that C's macro folds
   to — the same value C's initialiser holds.  */
static font_property_table: [font_property_table_entry; 15] =
  [ font_property_table_entry { key: iQCtype as c_int, validator: font_prop_validate_symbol },
    font_property_table_entry { key: iQCfoundry as c_int, validator: font_prop_validate_symbol },
    font_property_table_entry { key: iQCfamily as c_int, validator: font_prop_validate_symbol },
    font_property_table_entry { key: iQCadstyle as c_int, validator: font_prop_validate_symbol },
    font_property_table_entry { key: iQCregistry as c_int, validator: font_prop_validate_symbol },
    font_property_table_entry { key: iQCweight as c_int, validator: font_prop_validate_style },
    font_property_table_entry { key: iQCslant as c_int, validator: font_prop_validate_style },
    font_property_table_entry { key: iQCwidth as c_int, validator: font_prop_validate_style },
    font_property_table_entry { key: iQCsize as c_int, validator: font_prop_validate_non_neg },
    font_property_table_entry { key: iQCdpi as c_int, validator: font_prop_validate_non_neg },
    font_property_table_entry { key: iQCspacing as c_int, validator: font_prop_validate_spacing },
    font_property_table_entry { key: iQCavgwidth as c_int, validator: font_prop_validate_non_neg },
    /* The order of the above entries must match with enum
       font_property_index.  */
    font_property_table_entry { key: iQClang as c_int, validator: font_prop_validate_symbol },
    font_property_table_entry { key: iQCscript as c_int, validator: font_prop_validate_symbol },
    font_property_table_entry { key: iQCotf as c_int, validator: font_prop_validate_otf }
  ];

/* Return an index number of font property KEY or -1 if KEY is not an
   already known property.  */

unsafe fn get_font_prop_index (key: Lisp_Object) -> c_int
{
  let mut i: c_int;

  i = 0;
  while i < countof! (font_property_table) as c_int
    {
      if EQ (key, builtin_lisp_symbol (font_property_table[i as usize].key as usize))
	{ return i; }
      i += 1;
    }
  return -1;
}

/* Validate the font property.  The property key is specified by the
   symbol PROP, or the index IDX (if PROP is nil).  If VAL is invalid,
   signal an error.  The value is VAL or the regularized one.  */

unsafe fn font_prop_validate (idx: c_int, prop: Lisp_Object, val: Lisp_Object)
			      -> Lisp_Object
{
  let mut idx: c_int = idx;
  let mut prop: Lisp_Object = prop;
  let validated: Lisp_Object;

  if NILP (val)
    { return val; }
  if NILP (prop)
    { prop = builtin_lisp_symbol (font_property_table[idx as usize].key as usize); }
  else
    {
      idx = get_font_prop_index (prop);
      if idx < 0
	{ return val; }
    }
  validated = (font_property_table[idx as usize].validator) (prop, val);
  if EQ (validated, Qerror ())
    { signal_error (c"invalid font property".as_ptr (), Fcons (prop, val)); }
  return validated;
}


/* Store VAL as a value of extra font property PROP in FONT while
   keeping the sorting order.  Don't check the validity of VAL.  If
   VAL is Qunbound, delete the slot for PROP from the list of extra
   properties.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_put_extra (font: Lisp_Object, prop: Lisp_Object,
					 val: Lisp_Object) -> Lisp_Object
{
  let extra: Lisp_Object = AREF (font, FONT_EXTRA_INDEX as isize);
  let slot: Lisp_Object
    = if NILP (extra) { Qnil () } else { assq_no_quit (prop, extra) };

  if NILP (slot)
    {
      let mut prev: Lisp_Object = Qnil ();
      let mut extra: Lisp_Object = extra;

      if BASE_EQ (val, Qunbound ())
	{ return val; }
      while CONSP (extra)
	    && NILP (Fstring_lessp (prop, XCAR (XCAR (extra))))
	{
	  prev = extra;
	  extra = XCDR (extra);
	}

      if NILP (prev)
	{ ASET (font, FONT_EXTRA_INDEX as isize, Fcons (Fcons (prop, val), extra)); }
      else
	{ XSETCDR (prev, Fcons (Fcons (prop, val), extra)); }

      return val;
    }
  XSETCDR (slot, val);
  if BASE_EQ (val, Qunbound ())
    { ASET (font, FONT_EXTRA_INDEX as isize, Fdelq (slot, extra)); }
  return val;
}


/* Font name parser and unparser.  */

/* C 787-789's three `static` forward declarations (`parse_matrix`,
   `font_expand_wildcards`, `font_parse_name`) are dropped (PORTING.md 0.1-12).  */

/* An enumerator for each field of an XLFD font name.  */
pub type xlfd_field_index = c_int;

pub const XLFD_FOUNDRY_INDEX: xlfd_field_index = 0;
pub const XLFD_FAMILY_INDEX: xlfd_field_index = 1;
pub const XLFD_WEIGHT_INDEX: xlfd_field_index = 2;
pub const XLFD_SLANT_INDEX: xlfd_field_index = 3;
pub const XLFD_SWIDTH_INDEX: xlfd_field_index = 4;
pub const XLFD_ADSTYLE_INDEX: xlfd_field_index = 5;
pub const XLFD_PIXEL_INDEX: xlfd_field_index = 6;
pub const XLFD_POINT_INDEX: xlfd_field_index = 7;
pub const XLFD_RESX_INDEX: xlfd_field_index = 8;
pub const XLFD_RESY_INDEX: xlfd_field_index = 9;
pub const XLFD_SPACING_INDEX: xlfd_field_index = 10;
pub const XLFD_AVGWIDTH_INDEX: xlfd_field_index = 11;
pub const XLFD_REGISTRY_INDEX: xlfd_field_index = 12;
pub const XLFD_ENCODING_INDEX: xlfd_field_index = 13;
pub const XLFD_LAST_INDEX: xlfd_field_index = 14;

/* An enumerator for mask bit corresponding to each XLFD field.  */
pub type xlfd_field_mask = c_int;

pub const XLFD_FOUNDRY_MASK: xlfd_field_mask = 0x0001;
pub const XLFD_FAMILY_MASK: xlfd_field_mask = 0x0002;
pub const XLFD_WEIGHT_MASK: xlfd_field_mask = 0x0004;
pub const XLFD_SLANT_MASK: xlfd_field_mask = 0x0008;
pub const XLFD_SWIDTH_MASK: xlfd_field_mask = 0x0010;
pub const XLFD_ADSTYLE_MASK: xlfd_field_mask = 0x0020;
pub const XLFD_PIXEL_MASK: xlfd_field_mask = 0x0040;
pub const XLFD_POINT_MASK: xlfd_field_mask = 0x0080;
pub const XLFD_RESX_MASK: xlfd_field_mask = 0x0100;
pub const XLFD_RESY_MASK: xlfd_field_mask = 0x0200;
pub const XLFD_SPACING_MASK: xlfd_field_mask = 0x0400;
pub const XLFD_AVGWIDTH_MASK: xlfd_field_mask = 0x0800;
pub const XLFD_REGISTRY_MASK: xlfd_field_mask = 0x1000;
pub const XLFD_ENCODING_MASK: xlfd_field_mask = 0x2000;


/* Parse P pointing to the pixel/point size field of the form
   `[A B C D]' which specifies a transformation matrix:

	A  B  0
	C  D  0
	0  0  1

   by which all glyphs of the font are transformed.  The spec says
   that scalar value N for the pixel/point size is equivalent to:
   A = N * resx/resy, B = C = 0, D = N.

   Return the scalar value N if the form is valid.  Otherwise return
   -1.  */

unsafe fn parse_matrix (p: *const c_char) -> c_int
{
  let mut matrix: [c_double; 4] = [0.0; 4];
  let mut end: *mut c_char;
  let mut i: c_int;
  let mut p: *const c_char = p;

  i = 0;
  p = p.offset (1);
  while i < 4 && *p != 0 && *p != b']' as c_char
    {
      if *p == b'~' as c_char
	{
	  end = std::ptr::null_mut ();
	  matrix[i as usize] = -strtod (p.offset (1), &mut end);
	}
      else
	{
	  end = std::ptr::null_mut ();
	  matrix[i as usize] = strtod (p, &mut end);
	}
      p = end;
      i += 1;
    }
  return if i == 4 { matrix[3] as c_int } else { -1 };
}

/* Expand a wildcard field in FIELD (the first N fields are filled) to
   multiple fields to fill in all 14 XLFD fields while restricting a
   field position by its contents.  */

/// C: font.c 892-900: the anonymous `struct { int from; int to; int mask; }` C declares for
/// `range`; Rust needs a name for it.  The C text is the doc comment above the fields.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct font_expand_range
{
  /* Minimum possible field.  */
  pub from: c_int,
  /* Maximum possible field.  */
  pub to: c_int,
  /* Bit mask of possible field.  Nth bit corresponds to Nth field.  */
  pub mask: c_int,
}

unsafe fn font_expand_wildcards (field: *mut Lisp_Object, n: c_int) -> c_int
{
  /* Copy of FIELD.  */
  let mut tmp: [Lisp_Object; XLFD_LAST_INDEX as usize] = [Qnil (); XLFD_LAST_INDEX as usize];
  /* Array of information about where this element can go.  Nth
     element is for Nth element of FIELD. */
  let mut range: [font_expand_range; XLFD_LAST_INDEX as usize]
    = [font_expand_range { from: 0, to: 0, mask: 0 }; XLFD_LAST_INDEX as usize];
  let mut i: c_int;
  let mut j: c_int;
  let mut range_from: c_int;
  let mut range_to: c_int;
  let mut range_mask: c_uint;

/* C 905-910: the four field masks this function is named after.  */
macro_rules! XLFD_SYMBOL_MASK {
  () => { XLFD_FOUNDRY_MASK | XLFD_FAMILY_MASK
	  | XLFD_ADSTYLE_MASK | XLFD_REGISTRY_MASK };
}
macro_rules! XLFD_NULL_MASK {
  () => { XLFD_FOUNDRY_MASK | XLFD_ADSTYLE_MASK };
}
macro_rules! XLFD_LARGENUM_MASK {
  () => { XLFD_POINT_MASK | XLFD_RESX_MASK | XLFD_RESY_MASK
	  | XLFD_AVGWIDTH_MASK };
}
macro_rules! XLFD_REGENC_MASK {
  () => { XLFD_REGISTRY_MASK | XLFD_ENCODING_MASK };
}

  /* Initialize RANGE_MASK for FIELD[0] which can be 0th to (14 - N)th
     field.  The value is shifted to left one bit by one in the
     following loop.  */
  i = 0;
  range_mask = 0;
  while i <= 14 - n
    {
      range_mask = (range_mask << 1) | 1;
      i += 1;
    }

  /* The triplet RANGE_FROM, RANGE_TO, and RANGE_MASK is a
     position-based restriction for FIELD[I].  */
  i = 0;
  range_from = 0;
  range_to = 14 - n;
  while i < n
    {
      let val: Lisp_Object = *field.offset (i as isize);

      tmp[i as usize] = val;
      if NILP (val)
	{
	  /* Wildcard.  */
	  range[i as usize].from = range_from;
	  range[i as usize].to = range_to;
	  range[i as usize].mask = range_mask as c_int;
	}
      else
	{
	  /* The triplet FROM, TO, and MASK is a value-based
	     restriction for FIELD[I].  */
	  let mut from: c_int;
	  let mut to: c_int;
	  let mut mask: c_uint;

	  if FIXNUMP (val)
	    {
	      let numeric: EMACS_INT = XFIXNUM (val);

	      if i + 1 == n
		{
		  from = XLFD_ENCODING_INDEX;
		  to = from;
		  mask = XLFD_ENCODING_MASK as c_uint;
		}
	      else if numeric == 0
		{
		  from = XLFD_PIXEL_INDEX;
		  to = XLFD_AVGWIDTH_INDEX;
		  mask = (XLFD_PIXEL_MASK | XLFD_LARGENUM_MASK! ()) as c_uint;
		}
	      else if numeric <= 48
		{
		  from = XLFD_PIXEL_INDEX;
		  to = from;
		  mask = XLFD_PIXEL_MASK as c_uint;
		}
	      else
		{
		  from = XLFD_POINT_INDEX;
		  to = XLFD_AVGWIDTH_INDEX;
		  mask = XLFD_LARGENUM_MASK! () as c_uint;
		}
	    }
	  else if SBYTES (SYMBOL_NAME (val)) == 0
	    {
	      from = XLFD_FOUNDRY_INDEX;
	      to = XLFD_ADSTYLE_INDEX;
	      mask = XLFD_NULL_MASK! () as c_uint;
	    }
	  else if i == 0
	    {
	      from = XLFD_FOUNDRY_INDEX;
	      to = from;
	      mask = XLFD_FOUNDRY_MASK as c_uint;
	    }
	  else if i + 1 == n
	    {
	      let name: Lisp_Object = SYMBOL_NAME (val);

	      if *SDATA (name).offset (SBYTES (name) - 1) == b'*'
		{
		  from = XLFD_REGISTRY_INDEX;
		  to = XLFD_ENCODING_INDEX;
		  mask = XLFD_REGENC_MASK! () as c_uint;
		}
	      else
		{
		  from = XLFD_ENCODING_INDEX;
		  to = from;
		  mask = XLFD_ENCODING_MASK as c_uint;
		}
	    }
	  else if range_from <= XLFD_WEIGHT_INDEX
		   && range_to >= XLFD_WEIGHT_INDEX
		   && FONT_WEIGHT_NAME_NUMERIC! (val) >= 0
	    {
	      from = XLFD_WEIGHT_INDEX;
	      to = from;
	      mask = XLFD_WEIGHT_MASK as c_uint;
	    }
	  else if range_from <= XLFD_SLANT_INDEX
		   && range_to >= XLFD_SLANT_INDEX
		   && FONT_SLANT_NAME_NUMERIC! (val) >= 0
	    {
	      from = XLFD_SLANT_INDEX;
	      to = from;
	      mask = XLFD_SLANT_MASK as c_uint;
	    }
	  else if range_from <= XLFD_SWIDTH_INDEX
		   && range_to >= XLFD_SWIDTH_INDEX
		   && FONT_WIDTH_NAME_NUMERIC! (val) >= 0
	    {
	      from = XLFD_SWIDTH_INDEX;
	      to = from;
	      mask = XLFD_SWIDTH_MASK as c_uint;
	    }
	  else
	    {
	      if EQ (val, Qc ()) || EQ (val, Qm ()) || EQ (val, Qp ()) || EQ (val, Qd ())
		{
		  from = XLFD_SPACING_INDEX;
		  to = from;
		  mask = XLFD_SPACING_MASK as c_uint;
		}
	      else
		{
		  from = XLFD_FOUNDRY_INDEX;
		  to = XLFD_ENCODING_INDEX;
		  mask = XLFD_SYMBOL_MASK! () as c_uint;
		}
	    }

	  /* Merge position-based and value-based restrictions.  */
	  mask &= range_mask;
	  while from < range_from
	    {
	      mask &= !(1u32 << from);
	      from += 1;
	    }
	  while from < 14 && (mask & (1u32 << from)) == 0
	    { from += 1; }
	  while to > range_to
	    {
	      mask &= !(1u32 << to);
	      to -= 1;
	    }
	  while to >= 0 && (mask & (1u32 << to)) == 0
	    { to -= 1; }
	  if from > to
	    { return -1; }
	  range[i as usize].from = from;
	  range[i as usize].to = to;
	  range[i as usize].mask = mask as c_int;

	  if from > range_from || to < range_to
	    {
	      /* The range is narrowed by value-based restrictions.
		 Reflect it to the other fields.  */

	      /* Following fields should be after FROM.  */
	      range_from = from;
	      /* Preceding fields should be before TO.  */
	      j = i - 1;
	      from -= 1;
	      to -= 1;
	      while j >= 0
		{
		  /* Check FROM for non-wildcard field.  */
		  if ! NILP (tmp[j as usize]) && range[j as usize].from < from
		    {
		      while range[j as usize].from < from
			{
			  range[j as usize].mask &= !(1 << range[j as usize].from);
			  range[j as usize].from += 1;
			}
		      while from < 14 && (range[j as usize].mask & (1 << from)) == 0
			{ from += 1; }
		      range[j as usize].from = from;
		    }
		  else
		    { from = range[j as usize].from; }
		  if range[j as usize].to > to
		    {
		      while range[j as usize].to > to
			{
			  range[j as usize].mask &= !(1 << range[j as usize].to);
			  range[j as usize].to -= 1;
			}
		      while to >= 0 && (range[j as usize].mask & (1 << to)) == 0
			{ to -= 1; }
		      range[j as usize].to = to;
		    }
		  else
		    { to = range[j as usize].to; }
		  if from > to
		    { return -1; }
		  j -= 1;
		  from -= 1;
		  to -= 1;
		}
	    }
	}
      i += 1;
      range_from += 1;
      range_to += 1;
      range_mask <<= 1;
    }

  /* Decide all fields from restrictions in RANGE.  */
  i = 0;
  j = 0;
  while i < n
    {
      if j < range[i as usize].from
	{
	  if i == 0 || ! NILP (tmp[(i - 1) as usize])
	    /* None of TMP[X] corresponds to Jth field.  */
	    { return -1; }
	  memclear (field.offset (j as isize) as *mut c_void,
		    ((range[i as usize].from - j) as isize * word_size));
	  j = range[i as usize].from;
	}
      *field.offset (j as isize) = tmp[i as usize];
      j += 1;
      i += 1;
    }
  if ! NILP (tmp[(n - 1) as usize]) && j < XLFD_REGISTRY_INDEX
    { return -1; }
  memclear (field.offset (j as isize) as *mut c_void,
	    ((XLFD_LAST_INDEX - j) as isize * word_size));
  if FIXNUMP (*field.offset (XLFD_ENCODING_INDEX as isize))
    { *field.offset (XLFD_ENCODING_INDEX as isize)
	= Fintern (Fnumber_to_string (*field.offset (XLFD_ENCODING_INDEX as isize)),
		   Qnil ()); }
  return 0;
}


/* Parse NAME (null terminated) as XLFD and store information in FONT
   (font-spec or font-entity).  Size property of FONT is set as
   follows:
	specified XLFD fields		FONT property
	---------------------		-------------
	PIXEL_SIZE			PIXEL_SIZE (Lisp integer)
	POINT_SIZE and RESY		calculated pixel size (Lisp integer)
	POINT_SIZE			POINT_SIZE/10 (Lisp float)

   If NAME is successfully parsed, return 0.  Otherwise return -1.

   FONT is usually a font-spec, but when this function is called from
   X font backend driver, it is a font-entity.  In that case, NAME is
   a fully specified XLFD.  */

/// C 1121-1122: `INTERN_FIELD` and `INTERN_FIELD_SYM`.  A `macro_rules!` cannot name the
/// calling function's local `f` (hygiene, status 3.8), so C's two expression macros are
/// helpers taking the array and the index; the bodies keep C's pointer arithmetic
/// (`f[(N) + 1] - 1 - f[N]`) and the macro names are the function names.
#[inline(always)]
unsafe fn INTERN_FIELD (f: *mut *mut c_char, n: c_int) -> Lisp_Object
{
  font_intern_prop (*f.offset (n as isize),
		    (*f.offset (n as isize + 1)).offset (-1)
		      .offset_from (*f.offset (n as isize)),
		    false)
}

#[inline(always)]
unsafe fn INTERN_FIELD_SYM (f: *mut *mut c_char, n: c_int) -> Lisp_Object
{
  font_intern_prop (*f.offset (n as isize),
		    (*f.offset (n as isize + 1)).offset (-1)
		      .offset_from (*f.offset (n as isize)),
		    true)
}

unsafe fn font_parse_xlfd_1 (name: *mut c_char, len: ptrdiff_t, font: Lisp_Object,
			     mut segments: c_int) -> c_int
{
  let mut i: c_int;
  let mut j: c_int;
  let mut n: c_int;
  let mut f: [*mut c_char; XLFD_LAST_INDEX as usize + 1]
    = [std::ptr::null_mut (); XLFD_LAST_INDEX as usize + 1];
  let mut val: Lisp_Object;
  let mut p: *mut c_char;

  /* Reject empty XLFDs.  */
  if len == 0
    { return -1; }

  /* Accept "*-.." as a fully specified XLFD. */
  if *name == b'*' as c_char && (len == 1 || *name.offset (1) == b'-' as c_char)
    {
      i = 1;
      f[XLFD_FOUNDRY_INDEX as usize] = name;
    }
  else
    { i = 0; }

  /* Split into segments. */
  p = name.offset (i as isize);
  while *p != 0
    {
      if *p == b'-' as c_char
	{
	  /* If we have too many segments, then gather them up into the
	     FAMILY part of the name.  This allows using fonts with
	     dashes in the FAMILY bit. */
	  if segments > XLFD_LAST_INDEX && i == XLFD_WEIGHT_INDEX
	    { segments -= 1; }
	  else {
	    f[i as usize] = p.offset (1);
	    i += 1;
	    if i == XLFD_LAST_INDEX
	      { break; }
	  }
	}
      p = p.offset (1);
    }
  f[i as usize] = name.offset (len as isize);

  if i == XLFD_LAST_INDEX
    {
      /* Fully specified XLFD.  */
      /* PORTING.md 1.5: C assigns `pixel_size` only when the `&&`'s right operand runs,
	 so the port zero-initialises it to have an addressable place.  */
      let mut pixel_size: c_int = 0;

      ASET (font, FONT_FOUNDRY_INDEX as isize, INTERN_FIELD_SYM (f.as_mut_ptr (), XLFD_FOUNDRY_INDEX));
      ASET (font, FONT_FAMILY_INDEX as isize, INTERN_FIELD_SYM (f.as_mut_ptr (), XLFD_FAMILY_INDEX));
      i = XLFD_WEIGHT_INDEX;
      j = FONT_WEIGHT_INDEX;
      while i <= XLFD_SWIDTH_INDEX
	{
	  val = INTERN_FIELD_SYM (f.as_mut_ptr (), i);
	  if ! NILP (val)
	    {
	      n = font_style_to_value (j, INTERN_FIELD_SYM (f.as_mut_ptr (), i), false);
	      if n < 0
		{ return -1; }
	      ASET (font, j as isize, make_fixnum (n as EMACS_INT));
	    }
	  i += 1;
	  j += 1;
	}
      ASET (font, FONT_ADSTYLE_INDEX as isize, INTERN_FIELD_SYM (f.as_mut_ptr (), XLFD_ADSTYLE_INDEX));
      if strcmp (f[XLFD_REGISTRY_INDEX as usize], c"*-*".as_ptr ()) == 0
	{ ASET (font, FONT_REGISTRY_INDEX as isize, Qnil ()); }
      else
	{ ASET (font, FONT_REGISTRY_INDEX as isize,
		font_intern_prop (f[XLFD_REGISTRY_INDEX as usize],
				  f[XLFD_LAST_INDEX as usize]
				    .offset_from (f[XLFD_REGISTRY_INDEX as usize]),
				  true)); }
      p = f[XLFD_PIXEL_INDEX as usize];
      if *p == b'[' as c_char && { pixel_size = parse_matrix (p); pixel_size >= 0 }
	{ ASET (font, FONT_SIZE_INDEX as isize, make_fixnum (pixel_size as EMACS_INT)); }
      else
	{
	  val = INTERN_FIELD (f.as_mut_ptr (), XLFD_PIXEL_INDEX);
	  if FIXNUMP (val)
	    { ASET (font, FONT_SIZE_INDEX as isize, val); }
	  else if FONT_ENTITY_P (font)
	    { return -1; }
	  else
	    {
	      let mut point_size: c_double = -1.0;

	      eassert! (FONT_SPEC_P (font));
	      p = f[XLFD_POINT_INDEX as usize];
	      if *p == b'[' as c_char
		{ point_size = parse_matrix (p) as c_double; }
	      else if c_isdigit (*p as c_int)
		{
		  point_size = atoi (p) as c_double;
		  point_size /= 10.0;
		}
	      if point_size >= 0.0
		{ ASET (font, FONT_SIZE_INDEX as isize, make_float (point_size)); }
	    }
	}

      val = INTERN_FIELD (f.as_mut_ptr (), XLFD_RESY_INDEX);
      if ! NILP (val) && ! FIXNUMP (val)
	{ return -1; }
      ASET (font, FONT_DPI_INDEX as isize, val);
      val = INTERN_FIELD (f.as_mut_ptr (), XLFD_SPACING_INDEX);
      if ! NILP (val)
	{
	  val = font_prop_validate_spacing (QCspacing (), val);
	  if ! FIXNUMP (val)
	    { return -1; }
	  ASET (font, FONT_SPACING_INDEX as isize, val);
	}
      p = f[XLFD_AVGWIDTH_INDEX as usize];
      if *p == b'~' as c_char
	{ p = p.offset (1); }
      val = font_intern_prop (p,
			      f[XLFD_REGISTRY_INDEX as usize].offset (-1)
				.offset_from (p),
			      false);
      if ! NILP (val) && ! FIXNUMP (val)
	{ return -1; }
      ASET (font, FONT_AVGWIDTH_INDEX as isize, val);
    }
  else
    {
      let mut wild_card_found: bool = false;
      let mut prop: [Lisp_Object; XLFD_LAST_INDEX as usize]
	= [Qnil (); XLFD_LAST_INDEX as usize];

      if FONT_ENTITY_P (font)
	{ return -1; }
      j = 0;
      while j < i
	{
	  if *f[j as usize] == b'*' as c_char
	    {
	      if *f[j as usize].offset (1) != 0
		  && *f[j as usize].offset (1) != b'-' as c_char
		{ return -1; }
	      prop[j as usize] = Qnil ();
	      wild_card_found = true;
	    }
	  else if j + 1 < i
	    { prop[j as usize] = INTERN_FIELD (f.as_mut_ptr (), j); }
	  else
	    { prop[j as usize] = font_intern_prop (f[j as usize],
						   f[i as usize].offset_from (f[j as usize]),
						   false); }
	  j += 1;
	}
      if ! wild_card_found
	{ return -1; }
      if font_expand_wildcards (prop.as_mut_ptr (), i) < 0
	{ return -1; }

      ASET (font, FONT_FOUNDRY_INDEX as isize, prop[XLFD_FOUNDRY_INDEX as usize]);
      ASET (font, FONT_FAMILY_INDEX as isize, prop[XLFD_FAMILY_INDEX as usize]);
      i = XLFD_WEIGHT_INDEX;
      j = FONT_WEIGHT_INDEX;
      while i <= XLFD_SWIDTH_INDEX
	{
	  if ! NILP (prop[i as usize])
	    {
	      n = font_style_to_value (j, prop[i as usize], true);
	      if n < 0
		{ return -1; }
	      ASET (font, j as isize, make_fixnum (n as EMACS_INT));
	    }
	  i += 1;
	  j += 1;
	}
      ASET (font, FONT_ADSTYLE_INDEX as isize, prop[XLFD_ADSTYLE_INDEX as usize]);
      val = prop[XLFD_REGISTRY_INDEX as usize];
      if NILP (val)
	{
	  val = prop[XLFD_ENCODING_INDEX as usize];
	  if ! NILP (val)
	    {
	      AUTO_STRING! (star_dash, c"*-".as_ptr ());
	      val = concat2 (star_dash, SYMBOL_NAME (val));
	    }
	}
      else if NILP (prop[XLFD_ENCODING_INDEX as usize])
	{
	  AUTO_STRING! (dash_star, c"-*".as_ptr ());
	  val = concat2 (SYMBOL_NAME (val), dash_star);
	}
      else
	{
	  AUTO_STRING! (dash, c"-".as_ptr ());
	  val = concat3 (SYMBOL_NAME (val), dash,
			 SYMBOL_NAME (prop[XLFD_ENCODING_INDEX as usize]));
	}
      if ! NILP (val)
	{ ASET (font, FONT_REGISTRY_INDEX as isize, Fintern (val, Qnil ())); }

      if FIXNUMP (prop[XLFD_PIXEL_INDEX as usize])
	{ ASET (font, FONT_SIZE_INDEX as isize, prop[XLFD_PIXEL_INDEX as usize]); }
      else if FIXNUMP (prop[XLFD_POINT_INDEX as usize])
	{
	  let point_size: c_double = XFIXNUM (prop[XLFD_POINT_INDEX as usize]) as c_double;

	  ASET (font, FONT_SIZE_INDEX as isize, make_float (point_size / 10.0));
	}

      if FIXNUMP (prop[XLFD_RESX_INDEX as usize])
	{ ASET (font, FONT_DPI_INDEX as isize, prop[XLFD_RESY_INDEX as usize]); }
      if ! NILP (prop[XLFD_SPACING_INDEX as usize])
	{
	  val = font_prop_validate_spacing (QCspacing (),
					    prop[XLFD_SPACING_INDEX as usize]);
	  if ! FIXNUMP (val)
	    { return -1; }
	  ASET (font, FONT_SPACING_INDEX as isize, val);
	}
      if FIXNUMP (prop[XLFD_AVGWIDTH_INDEX as usize])
	{ ASET (font, FONT_AVGWIDTH_INDEX as isize, prop[XLFD_AVGWIDTH_INDEX as usize]); }
    }

  return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_parse_xlfd (name: *mut c_char, len: ptrdiff_t,
					  font: Lisp_Object) -> c_int
{
  let found: c_int = font_parse_xlfd_1 (name, len, font, -1);
  if found > -1
    { return found; }

  let mut segments: c_int = 0;
  /* Count how many segments we have. */
  let mut p: *mut c_char = name;
  while *p != 0
    {
      if *p == b'-' as c_char
	{ segments += 1; }
      p = p.offset (1);
    }

  /* If we have a surplus of segments, then we try to parse again, in
     case there's a font with dashes in the family name. */
  if segments > XLFD_LAST_INDEX
    { return font_parse_xlfd_1 (name, len, font, segments); }
  else
    { return -1; }
}

/* Return the XLFD name of FONT as a NULL terminated string, or NULL
   if the font is invalid.  If FONT is a scalable font, return
   PIXEL_SIZE as the XLFD's pixel size in lieu of its
   FONT_SIZE_INDEX.  */

/// `<inttypes.h>`: `#define PRIdMAX "jd"` on the frozen target (`intmax_t` is `long long`;
/// measured with clang: `printf ("%s", PRIdMAX)` prints `jd`).
const PRIdMAX: &str = "jd";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_dynamic_unparse_xlfd (font: Lisp_Object,
						    pixel_size: c_int) -> *mut c_char
{
  let mut p: *mut c_char;
  let mut f: [*const c_char; XLFD_REGISTRY_INDEX as usize + 1]
    = [std::ptr::null (); XLFD_REGISTRY_INDEX as usize + 1];
  let mut val: Lisp_Object;
  let mut i: c_int;
  let mut j: c_int;
  let mut name: *mut c_char;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  eassert! (FONTP (font));

  i = FONT_FOUNDRY_INDEX;
  j = XLFD_FOUNDRY_INDEX;
  while i <= FONT_REGISTRY_INDEX
    {
      if i == FONT_ADSTYLE_INDEX
	{ j = XLFD_ADSTYLE_INDEX; }
      else if i == FONT_REGISTRY_INDEX
	{ j = XLFD_REGISTRY_INDEX; }
      val = AREF (font, i as isize);
      if NILP (val)
	{
	  if j == XLFD_REGISTRY_INDEX
	    { f[j as usize] = c"*-*".as_ptr (); }
	  else
	    { f[j as usize] = c"*".as_ptr (); }
	}
      else
	{
	  if SYMBOLP (val)
	    { val = SYMBOL_NAME (val); }
	  if j == XLFD_REGISTRY_INDEX
	      && strchr (SSDATA (val), b'-' as c_int).is_null ()
	    {
	      let alloc: ptrdiff_t = SBYTES (val) + 4;

	      /* Change "jisx0208*" and "jisx0208" to "jisx0208*-*".  */
	      p = SAFE_ALLOCA! (sa_avail, alloc as usize) as *mut c_char;
	      f[j as usize] = p;
	      sprintf (p, c"%s%s-*".as_ptr (), SDATA (val),
		       c"*".as_ptr ().offset ((*SDATA (val).offset (SBYTES (val) - 1)
					       == b'*') as isize));
	    }
	  else
	    { f[j as usize] = SSDATA (val); }
	}
      i += 1;
      j += 1;
    }

  i = FONT_WEIGHT_INDEX;
  j = XLFD_WEIGHT_INDEX;
  while i <= FONT_WIDTH_INDEX
    {
      val = font_style_symbolic (font, i, false);
      if NILP (val)
	{ f[j as usize] = c"*".as_ptr (); }
      else
	{
	  let mut c: c_int;
	  let mut k: c_int;
	  let mut l: c_int;
	  let alloc: ptrdiff_t;

	  val = SYMBOL_NAME (val);
	  alloc = SBYTES (val) + 1;
	  p = SAFE_ALLOCA! (sa_avail, alloc as usize) as *mut c_char;
	  f[j as usize] = p;
	  /* Copy the name while excluding '-', '?', ',', and '"'.  */
	  k = 0;
	  l = 0;
	  while k < alloc as c_int
	    {
	      c = SREF (val, k as usize) as c_int;
	      if c != b'-' as c_int && c != b'?' as c_int && c != b',' as c_int
		  && c != b'"' as c_int
		{
		  *p.offset (l as isize) = c as c_char;
		  l += 1;
		}
	      k += 1;
	    }
	}
      i += 1;
      j += 1;
    }

  val = AREF (font, FONT_SIZE_INDEX as isize);
  eassert! (NUMBERP (val) || NILP (val));
  /* C's `char font_size_index_buf[sizeof "-*" + max (INT_STRLEN_BOUND (EMACS_INT),
     1 + DBL_MAX_10_EXP + 1)]`, measured with the frozen clang: 313 bytes.  */
  let mut font_size_index_buf: [c_char; 313] = [0; 313];
  if INTEGERP (val)
    {
      let mut v: intmax_t = 0;
      if ! (integer_to_intmax (val, &mut v) && 0 < v)
	{ v = pixel_size as intmax_t; }
      if v > 0
	{
	  p = font_size_index_buf.as_mut_ptr ();
	  f[XLFD_PIXEL_INDEX as usize] = p;
	  sprintf (p, c"%jd-*".as_ptr (), v);
	}
      else
	{ f[XLFD_PIXEL_INDEX as usize] = c"*-*".as_ptr (); }
    }
  else if FLOATP (val)
    {
      let v: c_double = XFLOAT_DATA (val) * 10.0;
      p = font_size_index_buf.as_mut_ptr ();
      f[XLFD_PIXEL_INDEX as usize] = p;
      sprintf (p, c"*-%.0f".as_ptr (), v);
    }
  else
    { f[XLFD_PIXEL_INDEX as usize] = c"*-*".as_ptr (); }

  /* C's `char dpi_index_buf[sizeof "-" + 2 * INT_STRLEN_BOUND (EMACS_INT)]`, measured:
     42 bytes.  */
  let mut dpi_index_buf: [c_char; 42] = [0; 42];
  if FIXNUMP (AREF (font, FONT_DPI_INDEX as isize))
    {
      let v: EMACS_INT = XFIXNUM (AREF (font, FONT_DPI_INDEX as isize));
      p = dpi_index_buf.as_mut_ptr ();
      f[XLFD_RESX_INDEX as usize] = p;
      sprintf (p, c"%ld-%ld".as_ptr (), v, v);
    }
  else
    { f[XLFD_RESX_INDEX as usize] = c"*-*".as_ptr (); }

  if FIXNUMP (AREF (font, FONT_SPACING_INDEX as isize))
    {
      let spacing: EMACS_INT = XFIXNUM (AREF (font, FONT_SPACING_INDEX as isize));

      f[XLFD_SPACING_INDEX as usize]
	= if spacing <= FONT_SPACING_PROPORTIONAL as EMACS_INT { c"p".as_ptr () }
	  else if spacing <= FONT_SPACING_DUAL as EMACS_INT { c"d".as_ptr () }
	  else if spacing <= FONT_SPACING_MONO as EMACS_INT { c"m".as_ptr () }
	  else { c"c".as_ptr () };
    }
  else
    { f[XLFD_SPACING_INDEX as usize] = c"*".as_ptr (); }

  /* C's `char avgwidth_index_buf[INT_BUFSIZE_BOUND (EMACS_INT)]`, measured: 21 bytes.  */
  let mut avgwidth_index_buf: [c_char; 21] = [0; 21];
  if FIXNUMP (AREF (font, FONT_AVGWIDTH_INDEX as isize))
    {
      p = avgwidth_index_buf.as_mut_ptr ();
      f[XLFD_AVGWIDTH_INDEX as usize] = p;
      sprintf (p, c"%ld".as_ptr (), XFIXNUM (AREF (font, FONT_AVGWIDTH_INDEX as isize)));
    }
  else
    { f[XLFD_AVGWIDTH_INDEX as usize] = c"*".as_ptr (); }

  /* Allocate a buffer large enough to accommodate the entire
     XLFD.  */

  name = xmalloc ((strlen (f[XLFD_FOUNDRY_INDEX as usize])
		   + strlen (f[XLFD_FAMILY_INDEX as usize])
		   + strlen (f[XLFD_WEIGHT_INDEX as usize])
		   + strlen (f[XLFD_SLANT_INDEX as usize])
		   + strlen (f[XLFD_SWIDTH_INDEX as usize])
		   + strlen (f[XLFD_ADSTYLE_INDEX as usize])
		   + strlen (f[XLFD_PIXEL_INDEX as usize])
		   + strlen (f[XLFD_RESX_INDEX as usize])
		   + strlen (f[XLFD_SPACING_INDEX as usize])
		   + strlen (f[XLFD_AVGWIDTH_INDEX as usize])
		   + strlen (f[XLFD_REGISTRY_INDEX as usize])
		   /* sizeof "-----------" = 12 */
		   + 12) as usize) as *mut c_char;

  /* Return the XLFD.  */

  sprintf (name, c"-%s-%s-%s-%s-%s-%s-%s-%s-%s-%s-%s".as_ptr (),
	   f[XLFD_FOUNDRY_INDEX as usize], f[XLFD_FAMILY_INDEX as usize],
	   f[XLFD_WEIGHT_INDEX as usize], f[XLFD_SLANT_INDEX as usize],
	   f[XLFD_SWIDTH_INDEX as usize], f[XLFD_ADSTYLE_INDEX as usize],
	   f[XLFD_PIXEL_INDEX as usize], f[XLFD_RESX_INDEX as usize],
	   f[XLFD_SPACING_INDEX as usize], f[XLFD_AVGWIDTH_INDEX as usize],
	   f[XLFD_REGISTRY_INDEX as usize]);
  SAFE_FREE! (sa_count, sa_mark);
  return name;
}

/* Store XLFD name of FONT (font-spec or font-entity) in NAME (NBYTES
   length), and return the name length.  If FONT_SIZE_INDEX of FONT is
   0, use PIXEL_SIZE instead.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_unparse_xlfd (font: Lisp_Object, pixel_size: c_int,
					    name: *mut c_char, nbytes: c_int) -> ptrdiff_t
{
  let mut p: *mut c_char;
  let mut f: [*const c_char; XLFD_REGISTRY_INDEX as usize + 1]
    = [std::ptr::null (); XLFD_REGISTRY_INDEX as usize + 1];
  let mut val: Lisp_Object;
  let mut i: c_int;
  let mut j: c_int;
  let mut len: c_int;

  eassert! (FONTP (font));

  i = FONT_FOUNDRY_INDEX;
  j = XLFD_FOUNDRY_INDEX;
  while i <= FONT_REGISTRY_INDEX
    {
      if i == FONT_ADSTYLE_INDEX
	{ j = XLFD_ADSTYLE_INDEX; }
      else if i == FONT_REGISTRY_INDEX
	{ j = XLFD_REGISTRY_INDEX; }
      val = AREF (font, i as isize);
      if NILP (val)
	{
	  if j == XLFD_REGISTRY_INDEX
	    { f[j as usize] = c"*-*".as_ptr (); }
	  else
	    { f[j as usize] = c"*".as_ptr (); }
	}
      else
	{
	  if SYMBOLP (val)
	    { val = SYMBOL_NAME (val); }
	  if j == XLFD_REGISTRY_INDEX
	      && strchr (SSDATA (val), b'-' as c_int).is_null ()
	    {
	      /* Change "jisx0208*" and "jisx0208" to "jisx0208*-*".  */
	      let alloc: ptrdiff_t = SBYTES (val) + 4;
	      if nbytes as ptrdiff_t <= alloc
		{ return -1; }
	      p = sa_stack_alloc (alloc as usize) as *mut c_char;
	      f[j as usize] = p;
	      sprintf (p, c"%s%s-*".as_ptr (), SDATA (val),
		       c"*".as_ptr ().offset ((*SDATA (val).offset (SBYTES (val) - 1)
					       == b'*') as isize));
	    }
	  else
	    { f[j as usize] = SSDATA (val); }
	}
      i += 1;
      j += 1;
    }

  i = FONT_WEIGHT_INDEX;
  j = XLFD_WEIGHT_INDEX;
  while i <= FONT_WIDTH_INDEX
    {
      val = font_style_symbolic (font, i, false);
      if NILP (val)
	{ f[j as usize] = c"*".as_ptr (); }
      else
	{
	  let mut c: c_int;
	  let mut k: c_int;
	  let mut l: c_int;
	  let alloc: ptrdiff_t;

	  val = SYMBOL_NAME (val);
	  alloc = SBYTES (val) + 1;
	  if nbytes as ptrdiff_t <= alloc
	    { return -1; }
	  p = sa_stack_alloc (alloc as usize) as *mut c_char;
	  f[j as usize] = p;
	  /* Copy the name while excluding '-', '?', ',', and '"'.  */
	  k = 0;
	  l = 0;
	  while k < alloc as c_int
	    {
	      c = SREF (val, k as usize) as c_int;
	      if c != b'-' as c_int && c != b'?' as c_int && c != b',' as c_int
		  && c != b'"' as c_int
		{
		  *p.offset (l as isize) = c as c_char;
		  l += 1;
		}
	      k += 1;
	    }
	}
      i += 1;
      j += 1;
    }

  val = AREF (font, FONT_SIZE_INDEX as isize);
  eassert! (NUMBERP (val) || NILP (val));
  /* C's `char font_size_index_buf[sizeof "-*" + max (INT_STRLEN_BOUND (EMACS_INT),
     1 + DBL_MAX_10_EXP + 1)]`, measured: 313 bytes (the font_dynamic_unparse_xlfd one).  */
  let mut font_size_index_buf: [c_char; 313] = [0; 313];
  if INTEGERP (val)
    {
      let mut v: intmax_t = 0;
      if ! (integer_to_intmax (val, &mut v) && 0 < v)
	{ v = pixel_size as intmax_t; }
      if v > 0
	{
	  p = font_size_index_buf.as_mut_ptr ();
	  f[XLFD_PIXEL_INDEX as usize] = p;
	  sprintf (p, c"%jd-*".as_ptr (), v);
	}
      else
	{ f[XLFD_PIXEL_INDEX as usize] = c"*-*".as_ptr (); }
    }
  else if FLOATP (val)
    {
      let v: c_double = XFLOAT_DATA (val) * 10.0;
      p = font_size_index_buf.as_mut_ptr ();
      f[XLFD_PIXEL_INDEX as usize] = p;
      sprintf (p, c"*-%.0f".as_ptr (), v);
    }
  else
    { f[XLFD_PIXEL_INDEX as usize] = c"*-*".as_ptr (); }

  /* C's `char dpi_index_buf[sizeof "-" + 2 * INT_STRLEN_BOUND (EMACS_INT)]`, measured:
     42 bytes.  */
  let mut dpi_index_buf: [c_char; 42] = [0; 42];
  if FIXNUMP (AREF (font, FONT_DPI_INDEX as isize))
    {
      let v: EMACS_INT = XFIXNUM (AREF (font, FONT_DPI_INDEX as isize));
      p = dpi_index_buf.as_mut_ptr ();
      f[XLFD_RESX_INDEX as usize] = p;
      sprintf (p, c"%ld-%ld".as_ptr (), v, v);
    }
  else
    { f[XLFD_RESX_INDEX as usize] = c"*-*".as_ptr (); }

  if FIXNUMP (AREF (font, FONT_SPACING_INDEX as isize))
    {
      let spacing: EMACS_INT = XFIXNUM (AREF (font, FONT_SPACING_INDEX as isize));

      f[XLFD_SPACING_INDEX as usize]
	= if spacing <= FONT_SPACING_PROPORTIONAL as EMACS_INT { c"p".as_ptr () }
	  else if spacing <= FONT_SPACING_DUAL as EMACS_INT { c"d".as_ptr () }
	  else if spacing <= FONT_SPACING_MONO as EMACS_INT { c"m".as_ptr () }
	  else { c"c".as_ptr () };
    }
  else
    { f[XLFD_SPACING_INDEX as usize] = c"*".as_ptr (); }

  /* C's `char avgwidth_index_buf[INT_BUFSIZE_BOUND (EMACS_INT)]`, measured: 21 bytes.  */
  let mut avgwidth_index_buf: [c_char; 21] = [0; 21];
  if FIXNUMP (AREF (font,  FONT_AVGWIDTH_INDEX as isize))
    {
      p = avgwidth_index_buf.as_mut_ptr ();
      f[XLFD_AVGWIDTH_INDEX as usize] = p;
      sprintf (p, c"%ld".as_ptr (), XFIXNUM (AREF (font, FONT_AVGWIDTH_INDEX as isize)));
    }
  else
    { f[XLFD_AVGWIDTH_INDEX as usize] = c"*".as_ptr (); }

  len = snprintf (name, nbytes as usize, c"-%s-%s-%s-%s-%s-%s-%s-%s-%s-%s-%s".as_ptr (),
		  f[XLFD_FOUNDRY_INDEX as usize], f[XLFD_FAMILY_INDEX as usize],
		  f[XLFD_WEIGHT_INDEX as usize], f[XLFD_SLANT_INDEX as usize],
		  f[XLFD_SWIDTH_INDEX as usize], f[XLFD_ADSTYLE_INDEX as usize],
		  f[XLFD_PIXEL_INDEX as usize], f[XLFD_RESX_INDEX as usize],
		  f[XLFD_SPACING_INDEX as usize], f[XLFD_AVGWIDTH_INDEX as usize],
		  f[XLFD_REGISTRY_INDEX as usize]);
  return if len < nbytes { len as ptrdiff_t } else { -1 };
}

/// C 1822-1826: `PROP_MATCH` and `PROP_SAVE`.  Both are expression macros over the calling
/// function's locals (`word_len`, `p`/`word_start`, and the variable PROP_SAVE assigns), and
/// a `macro_rules!` cannot name those (hygiene, status 3.8), so each takes them as arguments
/// while keeping C's own token sequence.
macro_rules! PROP_MATCH {
  ($word_len:expr, $word_start:expr, $str:expr) =>
    { ($word_len == $str.count_bytes () as ptrdiff_t
       && memcmp ($word_start as *const c_void, $str.as_ptr () as *const c_void,
		  $str.count_bytes ()) == 0) };
}
macro_rules! PROP_SAVE {
  ($var:expr, $str:expr) =>
    { if NILP ($var) { font_intern_prop ($str.as_ptr (), $str.count_bytes () as ptrdiff_t, true) } else { $var } };
}

unsafe fn font_parse_fcname (name: *mut c_char, len: ptrdiff_t, font: Lisp_Object) -> c_int
{
  let mut p: *mut c_char;
  let mut q: *mut c_char;
  let mut size_beg: *mut c_char = std::ptr::null_mut ();
  let mut size_end: *mut c_char = std::ptr::null_mut ();
  let mut props_beg: *mut c_char = std::ptr::null_mut ();
  let mut family_end: *mut c_char = std::ptr::null_mut ();

  if len == 0
    { return -1; }

  p = name;
  while *p != 0
    {
      if *p == b'\\' as c_char && *p.offset (1) != 0
	{ p = p.offset (1); }
      else if *p == b':' as c_char
	{
	  props_beg = p;
	  family_end = p;
	  break;
	}
      else if *p == b'-' as c_char
	{
	  let mut decimal: bool = false;
	  let mut size_found: bool = true;
	  q = p.offset (1);
	  while *q != 0 && *q != b':' as c_char
	    {
	      /* C 1647-1658: `#ifdef HAVE_NTGUI` (undef in the frozen configuration),
		 carried.  */
	      #[cfg(feature = "w32")]
	      {
		/* MS-Windows has several CJK fonts whose name ends in
		   "-ExtB".  It also has fonts whose names end in "-R" or
		   "-B", and one font whose name ends in "-SB".  */
		if q == p.offset (1) && (strncmp (q, c"ExtB".as_ptr (), 4) == 0
					 || strncmp (q, c"R".as_ptr (), 1) == 0
					 || strncmp (q, c"B".as_ptr (), 1) == 0
					 || strncmp (q, c"SB".as_ptr (), 2) == 0)
		  {
		    size_found = false;
		    break;
		  }
	      }
	      if ! c_isdigit (*q as c_int)
		{
		  if *q != b'.' as c_char || decimal
		    {
		      size_found = false;
		      break;
		    }
		  decimal = true;
		}
	      q = q.offset (1);
	    }
	  if size_found
	    {
	      family_end = p;
	      size_beg = p.offset (1);
	      size_end = q;
	      break;
	    }
	}
      p = p.offset (1);
    }

  if !family_end.is_null ()
    {
      let mut extra_props: Lisp_Object = Qnil ();

      /* A fontconfig name with size and/or property data.  */
      if family_end > name
	{
	  let family: Lisp_Object;

	  family = font_intern_prop (name, family_end.offset_from (name), true);
	  ASET (font, FONT_FAMILY_INDEX as isize, family);
	}
      if !size_beg.is_null ()
	{
	  let point_size: c_double = strtod (size_beg, &mut size_end);
	  ASET (font, FONT_SIZE_INDEX as isize, make_float (point_size));
	  if *size_end == b':' as c_char && *size_end.offset (1) != 0
	    { props_beg = size_end; }
	}
      if !props_beg.is_null ()
	{
	  /* Now parse ":KEY=VAL" patterns.  */
	  let mut val: Lisp_Object;

	  p = props_beg;
	  while *p != 0
	    {
	      q = p.offset (1);
	      while *q != 0 && *q != b'=' as c_char && *q != b':' as c_char
		{ q = q.offset (1); }
	      if *q != b'=' as c_char
		{
		  /* Must be an enumerated value.  */
		  let word_len: ptrdiff_t;
		  p = p.offset (1);
		  word_len = q.offset_from (p);
		  val = font_intern_prop (p, q.offset_from (p), true);

		  if PROP_MATCH! (word_len, p, c"thin")
		      || PROP_MATCH! (word_len, p, c"ultra-light")
		      || PROP_MATCH! (word_len, p, c"light")
		      || PROP_MATCH! (word_len, p, c"semi-light")
		      || PROP_MATCH! (word_len, p, c"book")
		      || PROP_MATCH! (word_len, p, c"medium")
		      || PROP_MATCH! (word_len, p, c"normal")
		      || PROP_MATCH! (word_len, p, c"semibold")
		      || PROP_MATCH! (word_len, p, c"demibold")
		      || PROP_MATCH! (word_len, p, c"bold")
		      || PROP_MATCH! (word_len, p, c"ultra-bold")
		      || PROP_MATCH! (word_len, p, c"black")
		      || PROP_MATCH! (word_len, p, c"heavy")
		      || PROP_MATCH! (word_len, p, c"ultra-heavy")
		    { FONT_SET_STYLE! (font, FONT_WEIGHT_INDEX, val); }
		  else if PROP_MATCH! (word_len, p, c"roman")
			   || PROP_MATCH! (word_len, p, c"italic")
			   || PROP_MATCH! (word_len, p, c"oblique")
		    { FONT_SET_STYLE! (font, FONT_SLANT_INDEX, val); }
		  else if PROP_MATCH! (word_len, p, c"charcell")
		    { ASET (font, FONT_SPACING_INDEX as isize,
			    make_fixnum (FONT_SPACING_CHARCELL as EMACS_INT)); }
		  else if PROP_MATCH! (word_len, p, c"mono")
		    { ASET (font, FONT_SPACING_INDEX as isize,
			    make_fixnum (FONT_SPACING_MONO as EMACS_INT)); }
		  else if PROP_MATCH! (word_len, p, c"proportional")
		    { ASET (font, FONT_SPACING_INDEX as isize,
			    make_fixnum (FONT_SPACING_PROPORTIONAL as EMACS_INT)); }
		}
	      else
		{
		  /* KEY=VAL pairs  */
		  /* PORTING.md 1.5: C declares `key` as `Lisp_Object key UNINIT;`, and
		     UNINIT is `= {0,}` in the frozen configuration (conf_post.h 369), so the
		     port writes the zeroed initialiser the macro stands for.  */
		  let mut key: Lisp_Object = Qnil ();
		  let mut prop: c_int;

		  if q.offset_from (p) == 10
		      && memcmp (p.offset (1) as *const c_void, c"pixelsize".as_ptr ()
				 as *const c_void, 9) == 0
		    { prop = FONT_SIZE_INDEX; }
		  else
		    {
		      key = font_intern_prop (p, q.offset_from (p), true);
		      prop = get_font_prop_index (key);
		    }

		  p = q.offset (1);
		  q = p;
		  while *q != 0 && *q != b':' as c_char
		    { q = q.offset (1); }
		  val = font_intern_prop (p, q.offset_from (p), false);

		  if prop >= FONT_FOUNDRY_INDEX
		      && prop < FONT_EXTRA_INDEX
		    { ASET (font, prop as isize, font_prop_validate (prop, Qnil (), val)); }
		  else
		    {
		      extra_props = nconc2 (extra_props,
					    list1 (Fcons (key, val)));
		    }
		}
	      p = q;
	    }
	}

      if ! NILP (extra_props)
	{
	  let mut driver_list: *mut font_driver_list = font_driver_list;
	  while !driver_list.is_null ()
	    {
	      if (*((*driver_list).driver)).filter_properties.is_some ()
		{
		  /* C: `(*driver_list->driver->filter_properties) (font, extra_props)`.  */
		  let filter_properties
		    = (*((*driver_list).driver)).filter_properties;
		  if let Some (f) = filter_properties
		    { f (font, extra_props); }
		}
	      driver_list = (*driver_list).next;
	    }
	}

    }
  else
    {
      /* Either a fontconfig-style name with no size and property
	 data, or a GTK-style name.  */
      let mut weight: Lisp_Object = Qnil ();
      let mut slant: Lisp_Object = Qnil ();
      let mut width: Lisp_Object = Qnil ();
      let mut size: Lisp_Object = Qnil ();
      let mut word_start: *mut c_char;
      let mut word_len: ptrdiff_t;

      /* Scan backwards from the end, looking for a size.  */
      p = name.offset (len - 1);
      while p >= name
	{
	  if !c_isdigit (*p as c_int)
	    { break; }
	  p = p.offset (-1);
	}

      if p < name.offset (len - 1) && (p.offset (1) == name || *p == b' ' as c_char)
	/* Found a font size.  */
	{ size = make_float (strtod (p.offset (1), std::ptr::null_mut ())); }
      else
	{ p = name.offset (len); }

      /* Now P points to the termination of the string, sans size.
	 Scan backwards, looking for font properties.  */
      while p > name
	{
	  q = p.offset (-1);
	  while q >= name
	    {
	      if q > name && *q.offset (-1) == b'\\' as c_char
		{ q = q.offset (-1); }   /* Skip quoting backslashes.  */
	      else if *q == b' ' as c_char
		{ break; }
	      q = q.offset (-1);
	    }

	  word_start = q.offset (1);
	  word_len = p.offset_from (word_start);

	  if PROP_MATCH! (word_len, word_start, c"Ultra-Light")
	    { weight = PROP_SAVE! (weight, c"ultra-light"); }
	  else if PROP_MATCH! (word_len, word_start, c"Light")
	    { weight = PROP_SAVE! (weight, c"light"); }
	  else if PROP_MATCH! (word_len, word_start, c"Book")
	    { weight = PROP_SAVE! (weight, c"book"); }
	  else if PROP_MATCH! (word_len, word_start, c"Medium")
	    { weight = PROP_SAVE! (weight, c"medium"); }
	  else if PROP_MATCH! (word_len, word_start, c"Semi-Bold")
	    { weight = PROP_SAVE! (weight, c"semi-bold"); }
	  else if PROP_MATCH! (word_len, word_start, c"Bold")
	    { weight = PROP_SAVE! (weight, c"bold"); }
	  else if PROP_MATCH! (word_len, word_start, c"Italic")
	    { slant = PROP_SAVE! (slant, c"italic"); }
	  else if PROP_MATCH! (word_len, word_start, c"Oblique")
	    { slant = PROP_SAVE! (slant, c"oblique"); }
	  else if PROP_MATCH! (word_len, word_start, c"Semi-Condensed")
	    { width = PROP_SAVE! (width, c"semi-condensed"); }
	  else if PROP_MATCH! (word_len, word_start, c"Condensed")
	    { width = PROP_SAVE! (width, c"condensed"); }
	  /* An unknown word must be part of the font name.  */
	  else
	    {
	      family_end = p;
	      break;
	    }
	  /* C's outer `for (; p > name; p = q)` advances to the word boundary the inner scan
	     found.  The port recomputed `q = p - 1` here, which overwrote that boundary and made
	     the next round rescan the same word minus its last character ("Oblique" then read as
	     the family "Obliqu", status 4.510).  */
	  p = q;
	}

      if !family_end.is_null ()
	{ ASET (font, FONT_FAMILY_INDEX as isize,
		font_intern_prop (name, family_end.offset_from (name), true)); }
      if !NILP (size)
	{ ASET (font, FONT_SIZE_INDEX as isize, size); }
      if !NILP (weight)
	{ FONT_SET_STYLE! (font, FONT_WEIGHT_INDEX, weight); }
      if !NILP (slant)
	{ FONT_SET_STYLE! (font, FONT_SLANT_INDEX, slant); }
      if !NILP (width)
	{ FONT_SET_STYLE! (font, FONT_WIDTH_INDEX, width); }
    }

  return 0;
}

/* C 1872: `#if defined HAVE_XFT || defined HAVE_FREETYPE || defined HAVE_NS` (all undef in
   the frozen configuration), carried; its definition is C 1882-2001.  */

/* Store fontconfig's font name of FONT (font-spec or font-entity) in
   NAME (NBYTES length), and return the name length.  If
   FONT_SIZE_INDEX of FONT is 0, use PIXEL_SIZE instead.
   Return a negative value on error.  */

#[cfg(feature = "xft_or_freetype_or_ns")]
unsafe fn font_unparse_fcname (font: Lisp_Object, mut pixel_size: c_int, name: *mut c_char,
			       nbytes: c_int) -> c_int
{
  let mut family: Lisp_Object;
  let mut foundry: Lisp_Object;
  let val: Lisp_Object;
  let mut point_size: c_int;
  let mut i: c_int;
  let mut p: *mut c_char;
  let mut lim: *mut c_char;
  let mut styles: [Lisp_Object; 3] = [Qnil (); 3];
  let style_names: [*const c_char; 3] = [c"weight".as_ptr (), c"slant".as_ptr (),
					 c"width".as_ptr ()];

  family = AREF (font, FONT_FAMILY_INDEX as isize);
  if ! NILP (family)
    {
      if SYMBOLP (family)
	{ family = SYMBOL_NAME (family); }
      else
	{ family = Qnil (); }
    }

  val = AREF (font, FONT_SIZE_INDEX as isize);
  if FIXNUMP (val)
    {
      if XFIXNUM (val) != 0
	{ pixel_size = XFIXNUM (val) as c_int; }
      point_size = -1;
    }
  else
    {
      eassert! (FLOATP (val));
      pixel_size = -1;
      point_size = XFLOAT_DATA (val) as c_int;
    }

  foundry = AREF (font, FONT_FOUNDRY_INDEX as isize);
  if ! NILP (foundry)
    {
      if SYMBOLP (foundry)
	{ foundry = SYMBOL_NAME (foundry); }
      else
	{ foundry = Qnil (); }
    }

  i = 0;
  while i < 3
    {
      styles[i as usize] = font_style_symbolic (font, FONT_WEIGHT_INDEX + i, false);
      i += 1;
    }

  p = name;
  lim = name.offset (nbytes as isize);
  if ! NILP (family)
    {
      let len: c_int = snprintf (p, lim.offset_from (p) as usize,
				 c"%s".as_ptr (), SSDATA (family));
      if ! (0 <= len && (len as ptrdiff_t) < lim.offset_from (p))
	{ return -1; }
      p = p.offset (len as isize);
    }
  if point_size > 0
    {
      let len: c_int = snprintf (p, lim.offset_from (p) as usize,
				 c"-%d".as_ptr ().offset ((p == name) as isize), point_size);
      if ! (0 <= len && (len as ptrdiff_t) < lim.offset_from (p))
	{ return -1; }
      p = p.offset (len as isize);
    }
  else if pixel_size > 0
    {
      let len: c_int = snprintf (p, lim.offset_from (p) as usize,
				 c":pixelsize=%d".as_ptr (), pixel_size);
      if ! (0 <= len && (len as ptrdiff_t) < lim.offset_from (p))
	{ return -1; }
      p = p.offset (len as isize);
    }
  if ! NILP (AREF (font, FONT_FOUNDRY_INDEX as isize))
    {
      let len: c_int = snprintf (p, lim.offset_from (p) as usize, c":foundry=%s".as_ptr (),
				 SSDATA (SYMBOL_NAME (AREF (font,
							    FONT_FOUNDRY_INDEX as isize))));
      if ! (0 <= len && (len as ptrdiff_t) < lim.offset_from (p))
	{ return -1; }
      p = p.offset (len as isize);
    }
  i = 0;
  while i < 3
    {
      if ! NILP (styles[i as usize])
	{
	  let len: c_int = snprintf (p, lim.offset_from (p) as usize, c":%s=%s".as_ptr (),
				     style_names[i as usize],
				     SSDATA (SYMBOL_NAME (styles[i as usize])));
	  if ! (0 <= len && (len as ptrdiff_t) < lim.offset_from (p))
	    { return -1; }
	  p = p.offset (len as isize);
	}
      i += 1;
    }

  if FIXNUMP (AREF (font, FONT_DPI_INDEX as isize))
    {
      let len: c_int = snprintf (p, lim.offset_from (p) as usize, c":dpi=%ld".as_ptr (),
				 XFIXNUM (AREF (font, FONT_DPI_INDEX as isize)));
      if ! (0 <= len && (len as ptrdiff_t) < lim.offset_from (p))
	{ return -1; }
      p = p.offset (len as isize);
    }

  if FIXNUMP (AREF (font, FONT_SPACING_INDEX as isize))
    {
      let len: c_int = snprintf (p, lim.offset_from (p) as usize, c":spacing=%ld".as_ptr (),
				 XFIXNUM (AREF (font, FONT_SPACING_INDEX as isize)));
      if ! (0 <= len && (len as ptrdiff_t) < lim.offset_from (p))
	{ return -1; }
      p = p.offset (len as isize);
    }

  if FIXNUMP (AREF (font, FONT_AVGWIDTH_INDEX as isize))
    {
      let len: c_int = snprintf (p, lim.offset_from (p) as usize,
				 (if XFIXNUM (AREF (font, FONT_AVGWIDTH_INDEX as isize)) == 0
				  { c":scalable=true" } else { c":scalable=false" }).as_ptr ());
      if ! (0 <= len && (len as ptrdiff_t) < lim.offset_from (p))
	{ return -1; }
      p = p.offset (len as isize);
    }

  return p.offset_from (name) as c_int;
}

/* Parse NAME (null terminated) and store information in FONT
   (font-spec or font-entity).  If NAME is successfully parsed, return
   0.  Otherwise return -1.  */

unsafe fn font_parse_name (name: *mut c_char, namelen: ptrdiff_t, font: Lisp_Object) -> c_int
{
  if *name == b'-' as c_char || !strchr (name, b'*' as c_int).is_null ()
      || !strchr (name, b'?' as c_int).is_null ()
    { return font_parse_xlfd (name, namelen, font); }
  return font_parse_fcname (name, namelen, font);
}


/* Merge FAMILY and REGISTRY into FONT_SPEC.  FAMILY may have the form
   "FAMILY-FOUNDRY".  REGISTRY may not contain charset-encoding
   part.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_parse_family_registry (family: Lisp_Object,
						     registry: Lisp_Object,
						     font_spec: Lisp_Object)
{
  let mut len: ptrdiff_t;
  let mut p0: *mut c_char;
  let mut p1: *mut c_char;
  let mut registry: Lisp_Object = registry;

  if ! NILP (family)
      && NILP (AREF (font_spec, FONT_FAMILY_INDEX as isize))
    {
      CHECK_STRING (family);
      len = SBYTES (family);
      p0 = SSDATA (family);
      p1 = strchr (p0, b'-' as c_int);
      /* C 2034-2041: `#ifdef HAVE_NTGUI` (undef in the frozen configuration), carried.  */
      #[cfg(feature = "w32")]
      {
	/* MS-Windows has fonts whose family name ends in "-ExtB" and
	   other suffixes which include a hyphen.  */
	if !p1.is_null () && (strcmp (p1, c"-ExtB".as_ptr ()) == 0
			      || strcmp (p1, c"-R".as_ptr ()) == 0
			      || strcmp (p1, c"-B".as_ptr ()) == 0
			      || strcmp (p1, c"-SB".as_ptr ()) == 0)
	  { p1 = std::ptr::null_mut (); }
      }
      if !p1.is_null ()
	{
	  if (*p0 != b'*' as c_char && p1.offset_from (p0) > 0)
	      && NILP (AREF (font_spec, FONT_FOUNDRY_INDEX as isize))
	    { Ffont_put (font_spec, QCfoundry (),
			 font_intern_prop (p0, p1.offset_from (p0), true)); }
	  p1 = p1.offset (1);
	  len -= p1.offset_from (p0);
	  Ffont_put (font_spec, QCfamily (), font_intern_prop (p1, len, true));
	}
      else
	{ ASET (font_spec, FONT_FAMILY_INDEX as isize, Fintern (family, Qnil ())); }
    }
  if ! NILP (registry)
    {
      /* Convert "XXX" and "XXX*" to "XXX*-*".  */
      CHECK_STRING (registry);
      len = SBYTES (registry);
      p0 = SSDATA (registry);
      p1 = strchr (p0, b'-' as c_int);
      if p1.is_null ()
	{
	  let asterisk: bool = len != 0 && *p0.offset (len - 1) == b'*' as c_char;
	  AUTO_STRING_WITH_LEN! (extra, c"*-*".as_ptr ().offset (asterisk as isize),
				 (3 - asterisk as c_int) as ptrdiff_t);
	  registry = concat2 (registry, extra);
	}
      registry = Fdowncase (registry);
      ASET (font_spec, FONT_REGISTRY_INDEX as isize, Fintern (registry, Qnil ()));
    }
}

/* Font sorting.  */

unsafe fn font_rescale_ratio (font_entity: Lisp_Object) -> c_double
{
  let mut tail: Lisp_Object;
  let mut elt: Lisp_Object;
  let mut name: Lisp_Object = Qnil ();

  tail = Vface_font_rescale_alist;
  while CONSP (tail)
    {
      elt = XCAR (tail);
      if FLOATP (XCDR (elt))
	{
	  if STRINGP (XCAR (elt))
	    {
	      if NILP (name)
		{ name = Ffont_xlfd_name (font_entity, Qnil (), Qt ()); }

	      /* N.B. that `name' is set to nil if the resulting XLFD
		 is too long.  */
	      if !NILP (name)
		  && fast_string_match_ignore_case (XCAR (elt), name) >= 0
		{ return XFLOAT_DATA (XCDR (elt)); }
	    }
	  else if FONT_SPEC_P (XCAR (elt))
	    {
	      if font_match_p (XCAR (elt), font_entity)
		{ return XFLOAT_DATA (XCDR (elt)); }
	    }
	}
      tail = XCDR (tail);
    }
  return 1.0;
}

/* We sort fonts by scoring each of them against a specified
   font-spec.  The score value is 32 bit (`unsigned'), and the smaller
   the value is, the closer the font is to the font-spec.

   The lowest 2 bits of the score are used for driver type.  The font
   available by the most preferred font driver is 0.

   The 4 7-bit fields in the higher 28 bits are used for numeric properties
   WEIGHT, SLANT, WIDTH, and SIZE.  */

/* How many bits to shift to store the difference value of each font
   property in a score.  Note that floats for FONT_TYPE_INDEX and
   FONT_REGISTRY_INDEX are not used.  */
static mut sort_shift_bits: [c_int; FONT_SIZE_INDEX as usize + 1]
  = [0; FONT_SIZE_INDEX as usize + 1];

/* Score font-entity ENTITY against properties of font-spec SPEC_PROP.
   The return value indicates how different ENTITY is compared with
   SPEC_PROP.  */

unsafe fn font_score (entity: Lisp_Object, spec_prop: *mut Lisp_Object) -> c_uint
{
  let mut score: c_uint = 0;
  let mut i: c_int;

  /* Score three style numeric fields.  Maximum difference is 127. */
  i = FONT_WEIGHT_INDEX;
  while i <= FONT_WIDTH_INDEX
    {
      if ! NILP (*spec_prop.offset (i as isize))
	  && ! EQ (AREF (entity, i as isize), *spec_prop.offset (i as isize))
	  && FIXNUMP (AREF (entity, i as isize))
	{
	  let diff: EMACS_INT = (XFIXNUM (AREF (entity, i as isize)) >> 8)
				- (XFIXNUM (*spec_prop.offset (i as isize)) >> 8);
	  score |= (min (eabs! (diff), 127) as c_uint) << sort_shift_bits[i as usize];
	}
      i += 1;
    }

  /* Score the size.  Maximum difference is 127.  */
  if ! NILP (*spec_prop.offset (FONT_SIZE_INDEX as isize))
      && XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize)) > 0
    {
      /* We use the higher 6-bit for the actual size difference.  The
	 lowest bit is set if the DPI is different.  */
      let mut diff: EMACS_INT;
      let mut pixel_size: EMACS_INT = XFIXNUM (*spec_prop.offset (FONT_SIZE_INDEX as isize));
      let entity_size: EMACS_INT = XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize));

      if CONSP (Vface_font_rescale_alist)
	{ pixel_size *= font_rescale_ratio (entity) as EMACS_INT; }
      if pixel_size * 2 < entity_size || entity_size * 2 < pixel_size
	/* This size is wrong by more than a factor 2: reject it!  */
	{ return 0xFFFFFFFF; }
      diff = eabs! (pixel_size - entity_size) << 1;
      if ! NILP (*spec_prop.offset (FONT_DPI_INDEX as isize))
	  && ! EQ (*spec_prop.offset (FONT_DPI_INDEX as isize),
		   AREF (entity, FONT_DPI_INDEX as isize))
	{ diff |= 1; }
      if ! NILP (*spec_prop.offset (FONT_AVGWIDTH_INDEX as isize))
	  && ! EQ (*spec_prop.offset (FONT_AVGWIDTH_INDEX as isize),
		   AREF (entity, FONT_AVGWIDTH_INDEX as isize))
	{ diff |= 1; }
      score |= (min (diff, 127) as c_uint) << sort_shift_bits[FONT_SIZE_INDEX as usize];
    }

  return score;
}


/* Concatenate all elements of LIST into one vector.  LIST is a list
   of font-entity vectors.  */

unsafe fn font_vconcat_entity_vectors (list: Lisp_Object) -> Lisp_Object
{
  let nargs: ptrdiff_t = list_length (list);
  let mut args: *mut Lisp_Object;
  let mut list: Lisp_Object = list;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  args = std::ptr::null_mut ();
  SAFE_ALLOCA_LISP! (sa_avail, args, nargs);

  let mut i: ptrdiff_t = 0;
  while i < nargs
    {
      *args.offset (i as isize) = XCAR (list);
      i += 1;
      list = XCDR (list);
    }
  let result: Lisp_Object = Fvconcat (nargs, args);
  SAFE_FREE! (sa_count, sa_mark);
  return result;
}


/* The structure for elements being sorted by qsort.  */
#[repr(C)]
pub struct font_sort_data
{
  pub score: c_uint,
  pub font_driver_preference: c_int,
  pub entity: Lisp_Object,
}


/* The comparison function for qsort.  */

pub unsafe extern "C" fn font_compare (d1: *const c_void, d2: *const c_void) -> c_int
{
  let data1: *const font_sort_data = d1 as *const font_sort_data;
  let data2: *const font_sort_data = d2 as *const font_sort_data;

  if (*data1).score < (*data2).score
    { return -1; }
  else if (*data1).score > (*data2).score
    { return 1; }
  return (*data1).font_driver_preference - (*data2).font_driver_preference;
}


/* Sort each font-entity vector in LIST by closeness to font-spec PREFER.
   If PREFER specifies a point-size, calculate the corresponding
   pixel-size from QCdpi property of PREFER or from the Y-resolution
   of FRAME before sorting.

   If BEST-ONLY is nonzero, return the best matching entity (that
   supports the character BEST-ONLY if BEST-ONLY is positive, or any
   if BEST-ONLY is negative).  Otherwise, return the sorted result as
   a single vector of font-entities.

   This function does no optimization for the case that the total
   number of elements is 1.  The caller should avoid calling this in
   such a case.  */

unsafe fn font_sort_entities (list: Lisp_Object, prefer: Lisp_Object, f: *mut frame,
			      best_only: c_int) -> Lisp_Object
{
  let mut prefer_prop: [Lisp_Object; FONT_SPEC_MAX as usize]
    = [Qnil (); FONT_SPEC_MAX as usize];
  let mut len: c_int;
  let mut maxlen: c_int;
  let mut i: c_int;
  let mut data: *mut font_sort_data = std::ptr::null_mut ();
  let mut best_score: c_uint;
  let mut best_entity: Lisp_Object;
  let mut tail: Lisp_Object;
  /* PORTING.md 1.5: C declares `Lisp_Object vec UNINIT;` (UNINIT is `= {0,}` in the frozen
     configuration), so the port zero-initialises it.  */
  let mut vec: Lisp_Object = Qnil ();
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  i = FONT_WEIGHT_INDEX;
  while i <= FONT_AVGWIDTH_INDEX
    {
      prefer_prop[i as usize] = AREF (prefer, i as isize);
      i += 1;
    }
  if FLOATP (prefer_prop[FONT_SIZE_INDEX as usize])
    { prefer_prop[FONT_SIZE_INDEX as usize]
	= make_fixnum (font_pixel_size (f, prefer) as EMACS_INT); }

  if NILP (XCDR (list))
    {
      /* What we have to take care of is this single vector.  */
      vec = XCAR (list);
      maxlen = ASIZE (vec) as c_int;
    }
  else if best_only != 0
    {
      /* We don't have to perform sort, so there's no need of creating
	 a single vector.  But, we must find the length of the longest
	 vector.  */
      maxlen = 0;
      tail = list;
      while CONSP (tail)
	{
	  if maxlen < ASIZE (XCAR (tail)) as c_int
	    { maxlen = ASIZE (XCAR (tail)) as c_int; }
	  tail = XCDR (tail);
	}
    }
  else
    {
      /* We have to create a single vector to sort it.  */
      vec = font_vconcat_entity_vectors (list);
      maxlen = ASIZE (vec) as c_int;
    }

  SAFE_NALLOCA! (sa_avail, data, 1, maxlen);
  best_score = 0xFFFFFFFF;
  best_entity = Qnil ();

  tail = list;
  while CONSP (tail)
    {
      let mut font_driver_preference: c_int = 0;
      let mut current_font_driver: Lisp_Object;

      if best_only != 0
	{ vec = XCAR (tail); }
      len = ASIZE (vec) as c_int;

      /* We are sure that the length of VEC > 0.  */
      current_font_driver = AREF (AREF (vec, 0), FONT_TYPE_INDEX as isize);
      /* Score the elements.  */
      i = 0;
      while i < len
	{
	  (*data.offset (i as isize)).entity = AREF (vec, i as isize);
	  (*data.offset (i as isize)).score
	    = if best_only <= 0 || font_has_char (f, (*data.offset (i as isize)).entity,
						  best_only) > 0
	      { font_score ((*data.offset (i as isize)).entity, prefer_prop.as_mut_ptr ()) }
	      else { 0xFFFFFFFF };
	  if best_only != 0 && best_score > (*data.offset (i as isize)).score
	    {
	      best_score = (*data.offset (i as isize)).score;
	      best_entity = (*data.offset (i as isize)).entity;
	      if best_score == 0
		{ break; }
	    }
	  if ! EQ (current_font_driver, AREF (AREF (vec, i as isize), FONT_TYPE_INDEX as isize))
	    {
	      current_font_driver = AREF (AREF (vec, i as isize), FONT_TYPE_INDEX as isize);
	      font_driver_preference += 1;
	    }
	  (*data.offset (i as isize)).font_driver_preference = font_driver_preference;
	  i += 1;
	}

      /* Sort if necessary.  */
      if best_only == 0
	{
	  qsort (data as *mut c_void, len as usize,
		 std::mem::size_of::<font_sort_data> (), Some (font_compare));
	  i = 0;
	  while i < len
	    {
	      ASET (vec, i as isize, (*data.offset (i as isize)).entity);
	      i += 1;
	    }
	  break;
	}
      else
	{ vec = best_entity; }
      tail = XCDR (tail);
    }

  SAFE_FREE! (sa_count, sa_mark);

  FONT_ADD_LOG! (c"sort-by".as_ptr (), prefer, vec);
  return vec;
}


/* API of Font Service Layer.  */

/* Reflect ORDER (see the variable font_sort_order in xfaces.c) to
   sort_shift_bits.  Finternal_set_font_selection_order calls this
   function with font_sort_order after setting up it.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_update_sort_order (order: *mut c_int)
{
  let mut i: c_int;
  let mut shift_bits: c_int;

  i = 0;
  shift_bits = 23;
  while i < 4
    {
      let xlfd_idx: c_int = *order.offset (i as isize);

      if xlfd_idx == XLFD_WEIGHT_INDEX
	{ sort_shift_bits[FONT_WEIGHT_INDEX as usize] = shift_bits; }
      else if xlfd_idx == XLFD_SLANT_INDEX
	{ sort_shift_bits[FONT_SLANT_INDEX as usize] = shift_bits; }
      else if xlfd_idx == XLFD_SWIDTH_INDEX
	{ sort_shift_bits[FONT_WIDTH_INDEX as usize] = shift_bits; }
      else
	{ sort_shift_bits[FONT_SIZE_INDEX as usize] = shift_bits; }
      i += 1;
      shift_bits -= 7;
    }
}

unsafe fn font_check_otf_features (script: Lisp_Object, langsys: Lisp_Object,
				   features: Lisp_Object, table: Lisp_Object) -> bool
{
  let mut val: Lisp_Object;
  let mut negative: bool;
  let mut table: Lisp_Object = table;
  let mut features: Lisp_Object = features;

  table = assq_no_quit (script, table);
  if NILP (table)
    { return false; }
  table = XCDR (table);
  if ! NILP (langsys)
    {
      table = assq_no_quit (langsys, table);
      if NILP (table)
	{ return false; }
    }
  else
    {
      val = assq_no_quit (Qnil (), table);
      if NILP (val)
	{ table = XCAR (table); }
      else
	{ table = val; }
    }
  table = XCDR (table);
  negative = false;
  while CONSP (features)
    {
      if NILP (XCAR (features))
	{
	  negative = true;
	  features = XCDR (features);
	  continue;
	}
      if NILP (Fmemq (XCAR (features), table)) != negative
	{ return false; }
      features = XCDR (features);
    }
  return true;
}

/* Check if OTF_CAPABILITY satisfies SPEC (otf-spec).  */

unsafe fn font_check_otf (spec: Lisp_Object, otf_capability: Lisp_Object) -> bool
{
  let mut script: Lisp_Object;
  let mut langsys: Lisp_Object = Qnil ();
  let mut gsub: Lisp_Object = Qnil ();
  let mut gpos: Lisp_Object = Qnil ();
  let mut spec: Lisp_Object = spec;

  script = XCAR (spec);
  spec = XCDR (spec);
  if ! NILP (spec)
    {
      langsys = XCAR (spec);
      spec = XCDR (spec);
      if ! NILP (spec)
	{
	  gsub = XCAR (spec);
	  spec = XCDR (spec);
	  if ! NILP (spec)
	    { gpos = XCAR (spec); }
	}
    }

  if ! NILP (gsub) && ! font_check_otf_features (script, langsys, gsub,
						 XCAR (otf_capability))
    { return false; }
  if ! NILP (gpos) && ! font_check_otf_features (script, langsys, gpos,
						 XCDR (otf_capability))
    { return false; }
  return true;
}



/* Check if FONT (font-entity or font-object) matches with the font
   specification SPEC.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_match_p (spec: Lisp_Object, font: Lisp_Object) -> bool
{
  let mut prop: [Lisp_Object; FONT_SPEC_MAX as usize]
    = [Qnil (); FONT_SPEC_MAX as usize];
  let mut props: *mut Lisp_Object;
  let mut extra: Lisp_Object;
  let font_extra: Lisp_Object;
  let mut i: c_int;

  i = FONT_FOUNDRY_INDEX;
  while i <= FONT_REGISTRY_INDEX
    {
      if ! NILP (AREF (spec, i as isize))
	  && ! NILP (AREF (font, i as isize))
	  && ! EQ (AREF (spec, i as isize), AREF (font, i as isize))
	{ return false; }
      i += 1;
    }
  props = (*XFONT_SPEC (spec)).props.as_mut_ptr ();
  if FLOATP (*props.offset (FONT_SIZE_INDEX as isize))
    {
      i = FONT_FOUNDRY_INDEX;
      while i < FONT_SIZE_INDEX
	{
	  prop[i as usize] = AREF (spec, i as isize);
	  i += 1;
	}
      prop[FONT_SIZE_INDEX as usize]
	= make_fixnum (font_pixel_size (XFRAME (selected_frame), spec) as EMACS_INT);
      props = prop.as_mut_ptr ();
    }

  if font_score (font, props) > 0
    { return false; }
  extra = AREF (spec, FONT_EXTRA_INDEX as isize);
  font_extra = AREF (font, FONT_EXTRA_INDEX as isize);
  while CONSP (extra)
    {
      let key: Lisp_Object = XCAR (XCAR (extra));
      let val: Lisp_Object = XCDR (XCAR (extra));
      let mut val2: Lisp_Object;

      if EQ (key, QClang ())
	{
	  val2 = assq_no_quit (key, font_extra);
	  if NILP (val2)
	    { return false; }
	  val2 = XCDR (val2);
	  if CONSP (val)
	    {
	      if ! CONSP (val2)
		{ return false; }
	      while CONSP (val)
		{
		  if NILP (Fmemq (val, val2))
		    { return false; }
		}
	    }
	  else
	    {
	      if if CONSP (val2) { NILP (Fmemq (val, XCDR (val2))) }
		 else { ! EQ (val, val2) }
		{ return false; }
	    }
	}
      else if EQ (key, QCscript ())
	{
	  val2 = assq_no_quit (val, Vscript_representative_chars);
	  if CONSP (val2)
	    {
	      val2 = XCDR (val2);
	      if CONSP (val2)
		{
		  if ! FONT_OBJECT_P (font)
		    { return false; }
		  /* All characters in the list must be supported.  */
		  while CONSP (val2)
		    {
		      if ! CHARACTERP (XCAR (val2))
			{
			  val2 = XCDR (val2);
			  continue;
			}
		      if font_encode_char (font, XFIXNAT (XCAR (val2)) as c_int)
			  == FONT_INVALID_CODE
			{ return false; }
		      val2 = XCDR (val2);
		    }
		}
	      else if VECTORP (val2)
		{
		  if ! FONT_OBJECT_P (font)
		    { return false; }
		  /* At most one character in the vector must be supported.  */
		  i = 0;
		  while i < ASIZE (val2) as c_int
		    {
		      if ! CHARACTERP (AREF (val2, i as isize))
			{
			  i += 1;
			  continue;
			}
		      if font_encode_char (font, XFIXNAT (AREF (val2, i as isize)) as c_int)
			  != FONT_INVALID_CODE
			{ break; }
		      i += 1;
		    }
		  if i == ASIZE (val2) as c_int
		    { return false; }
		}
	    }
	}
      else if EQ (key, QCotf ())
	{
	  let fontp: *mut font;

	  if ! FONT_OBJECT_P (font)
	    { return false; }
	  fontp = XFONT_OBJECT (font);
	  if (*((*fontp).driver as *const font_driver)).otf_capability.is_none ()
	    { return false; }
	  let otf_capability = (*((*fontp).driver as *const font_driver)).otf_capability;
	  if let Some (f) = otf_capability
	    { val2 = f (fontp); }
	  else
	    { val2 = Qnil (); }
	  if NILP (val2) || ! font_check_otf (val, val2)
	    { return false; }
	}
      extra = XCDR (extra);
    }

  return true;
}


/* Font cache

   Each font backend has the callback function get_cache, and it
   returns a cons cell of which cdr part can be freely used for
   caching fonts.  The cons cell may be shared by multiple frames
   and/or multiple font drivers.  So, we arrange the cdr part as this:

	((DRIVER-TYPE NUM-FRAMES FONT-CACHE-DATA ...) ...)

   where DRIVER-TYPE is a symbol such as `x', `xft', etc., NUM-FRAMES
   is a number frames sharing this cache, and FONT-CACHE-DATA is a
   cons (FONT-SPEC . [FONT-ENTITY ...]).  */

/* C 2546-2547's forward declaration of `font_clear_cache` is dropped (PORTING.md 0.1-12);
   its definition follows.  */

unsafe fn font_prepare_cache (f: *mut frame, driver: *const font_driver)
{
  let cache: Lisp_Object;
  let mut val: Lisp_Object;

  cache = ((*driver).get_cache.unwrap ()) (f as *mut c_void);
  val = XCDR (cache);
  while CONSP (val) && ! EQ (XCAR (XCAR (val)), (*driver).r#type)
    { val = XCDR (val); }
  if NILP (val)
    {
      val = list2 ((*driver).r#type, make_fixnum (1));
      XSETCDR (cache, Fcons (val, XCDR (cache)));
    }
  else
    {
      val = XCDR (XCAR (val));
      XSETCAR (val, make_fixnum (XFIXNUM (XCAR (val)) + 1));
    }
}


unsafe fn font_finish_cache (f: *mut frame, driver: *const font_driver)
{
  let mut cache: Lisp_Object;
  let mut val: Lisp_Object;
  let mut tmp: Lisp_Object;


  cache = ((*driver).get_cache.unwrap ()) (f as *mut c_void);
  val = XCDR (cache);
  while CONSP (val) && ! EQ (XCAR (XCAR (val)), (*driver).r#type)
    {
      cache = val;
      val = XCDR (val);
    }
  eassert! (! NILP (val));
  tmp = XCDR (XCAR (val));
  XSETCAR (tmp, make_fixnum (XFIXNUM (XCAR (tmp)) - 1));
  if XFIXNUM (XCAR (tmp)) == 0
    {
      font_clear_cache (f, XCAR (val), driver);
      XSETCDR (cache, XCDR (val));
    }
}


unsafe fn font_get_cache (f: *mut frame, driver: *const font_driver) -> Lisp_Object
{
  let mut val: Lisp_Object = ((*driver).get_cache.unwrap ()) (f as *mut c_void);
  let r#type: Lisp_Object = (*driver).r#type;

  eassert! (CONSP (val));
  val = XCDR (val);
  while ! EQ (XCAR (XCAR (val)), r#type)
    { val = XCDR (val); }
  eassert! (CONSP (val));
  /* VAL = ((DRIVER-TYPE NUM-FRAMES FONT-CACHE-DATA ...) ...) */
  val = XCDR (XCAR (val));
  return val;
}


unsafe fn font_clear_cache (f: *mut frame, cache: Lisp_Object, driver: *const font_driver)
{
  let mut tail: Lisp_Object;
  let mut elt: Lisp_Object;
  let mut entity: Lisp_Object;
  let mut i: ptrdiff_t;

  /* CACHE = (DRIVER-TYPE NUM-FRAMES FONT-CACHE-DATA ...) */
  tail = XCDR (XCDR (cache));
  while CONSP (tail)
    {
      elt = XCAR (tail);
      /* elt should have the form (FONT-SPEC . [FONT-ENTITY ...]) */
      if CONSP (elt) && FONT_SPEC_P (XCAR (elt))
	{
	  elt = XCDR (elt);
	  eassert! (VECTORP (elt));
	  i = 0;
	  while i < ASIZE (elt)
	    {
	      entity = AREF (elt, i);

	      if FONT_ENTITY_P (entity)
		  && EQ ((*driver).r#type, AREF (entity, FONT_TYPE_INDEX as isize))
		{
		  let mut objlist: Lisp_Object = AREF (entity, FONT_OBJLIST_INDEX as isize);

		  while CONSP (objlist)
		    {
		      let val: Lisp_Object = XCAR (objlist);
		      let font: *mut font = XFONT_OBJECT (val);

		      if ! NILP (AREF (val, FONT_TYPE_INDEX as isize))
			{
			  eassert! (!font.is_null () && driver == (*font).driver as *const font_driver);
			  /* We are going to close the font, so make
			     sure we don't have any lgstrings lying
			     around in lgstring cache that reference
			     the font.  */
			  composition_gstring_cache_clear_font (val);
			  ((*driver).close_font.unwrap ()) (font);
			}
		      objlist = XCDR (objlist);
		    }
		  if (*driver).free_entity.is_some ()
		    {
		      let free_entity = (*driver).free_entity;
		      if let Some (f) = free_entity
			{ f (entity); }
		    }
		}
	      i += 1;
	    }
	}
      tail = XCDR (tail);
    }
  XSETCDR (cache, Qnil ());
}


/* Check whether NAME should be ignored based on Vface_ignored_fonts.
   This is reused by xg_font_filter to apply the same checks to the
   GTK font chooser.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_is_ignored (name: *const c_char, namelen: ptrdiff_t) -> bool
{
  let mut tail: Lisp_Object = Vface_ignored_fonts;
  let mut regexp: Lisp_Object;

  FOR_EACH_TAIL_SAFE! (tail, {
    regexp = XCAR (tail);
    if STRINGP (regexp)
	&& fast_c_string_match_ignore_case (regexp, name, namelen as usize) >= 0
      { return true; }
  });
  return false;
}
static mut scratch_font_spec: Lisp_Object = lisp_h_Qnil ();
static mut scratch_font_prefer: Lisp_Object = lisp_h_Qnil ();

/* Check each font-entity in VEC, and return a list of font-entities
   that satisfy these conditions:
     (1) matches with SPEC and SIZE if SPEC is not nil, and
     (2) doesn't match with any regexps in Vface_ignored_fonts (if non-nil).
*/

unsafe fn font_delete_unmatched (vec: Lisp_Object, spec: Lisp_Object, size: c_int)
				 -> Lisp_Object
{
  let mut entity: Lisp_Object;
  let mut val: Lisp_Object;
  let mut prop: font_property_index;
  let mut i: ptrdiff_t;

  val = Qnil ();
  i = ASIZE (vec) - 1;
  while i >= 0
    {
      entity = AREF (vec, i);
      if ! NILP (Vface_ignored_fonts)
	{
	  let mut name: [c_char; 256] = [0; 256];
	  let mut xlfd: *mut c_char;
	  let namelen: ptrdiff_t;

	  namelen = font_unparse_xlfd (entity, 0, name.as_mut_ptr (), 256);

	  if namelen >= 0
	    {
	      if font_is_ignored (name.as_ptr (), namelen)
		{
		  i -= 1;
		  continue;
		}
	    }
	  else
	    {
	      /* The font family or foundry is too long for a 256
		 character xlfd to accommodate.  */

	      xlfd = font_dynamic_unparse_xlfd (entity, 0);

	      if !xlfd.is_null ()
		  && font_is_ignored (xlfd, std::mem::size_of::<*mut c_char> () as ptrdiff_t)
		{
		  xfree (xlfd as *mut c_void);
		  i -= 1;
		  continue;
		}

	      xfree (xlfd as *mut c_void);
	    }
	}
      if NILP (spec)
	{
	  val = Fcons (entity, val);
	  i -= 1;
	  continue;
	}
      prop = FONT_WEIGHT_INDEX;
      while prop < FONT_SIZE_INDEX
	{
	  if FIXNUMP (AREF (spec, prop as isize))
	    {
	      if !FIXNUMP (AREF (entity, prop as isize))
		{ prop = FONT_SPEC_MAX; }
	      else
		{
		  let required: c_int = (XFIXNUM (AREF (spec, prop as isize)) >> 8) as c_int;
		  let candidate: c_int = (XFIXNUM (AREF (entity, prop as isize)) >> 8) as c_int;
		  let mut mismatch: bool = candidate != required;

		  /* C 2748-2760: `#ifdef HAVE_NTGUI` (undef in the frozen configuration),
		     carried; with the arm absent C's condition is `candidate != required`.  */
		  #[cfg(feature = "w32")]
		  {
		    mismatch = candidate != required
			       /* A kludge for w32 font search, where listing a
				  family returns only 4 standard weights: regular,
				  italic, bold, bold-italic.  For other values one
				  must specify the font, not just the family in the
				  :family attribute of the face.  But specifying
				  :family in the face attributes looks for regular
				  weight, so if we require exact match, the
				  non-regular font will be rejected.  So we relax
				  the accuracy of the match here, and let
				  font_sort_entities find the best match.  */
			       && (prop != FONT_WEIGHT_INDEX
				   || eabs! (candidate - required) > 100);
		  }
		  if mismatch
		    { prop = FONT_SPEC_MAX; }
		}
	    }
	  prop += 1;
	}
      if prop < FONT_SPEC_MAX
	  && size != 0
	  && XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize)) > 0
	{
	  let diff: c_int = (XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize))
			     - size as EMACS_INT) as c_int;

	  if eabs! (diff) > FONT_PIXEL_SIZE_QUANTUM
	    { prop = FONT_SPEC_MAX; }
	}
      if prop < FONT_SPEC_MAX
	  && FIXNUMP (AREF (spec, FONT_DPI_INDEX as isize))
	  && FIXNUMP (AREF (entity, FONT_DPI_INDEX as isize))
	  && XFIXNUM (AREF (entity, FONT_DPI_INDEX as isize)) != 0
	  && ! EQ (AREF (spec, FONT_DPI_INDEX as isize), AREF (entity, FONT_DPI_INDEX as isize))
	{ prop = FONT_SPEC_MAX; }
      if prop < FONT_SPEC_MAX
	  && FIXNUMP (AREF (spec, FONT_AVGWIDTH_INDEX as isize))
	  && FIXNUMP (AREF (entity, FONT_AVGWIDTH_INDEX as isize))
	  && XFIXNUM (AREF (entity, FONT_AVGWIDTH_INDEX as isize)) != 0
	  && ! EQ (AREF (spec, FONT_AVGWIDTH_INDEX as isize),
		   AREF (entity, FONT_AVGWIDTH_INDEX as isize))
	{ prop = FONT_SPEC_MAX; }
      if prop < FONT_SPEC_MAX
	{ val = Fcons (entity, val); }
      i -= 1;
    }
  return Fvconcat (1, &mut val);
}


/* Return a list of vectors of font-entities matching with SPEC on
   FRAME.  Each elements in the list is a vector of entities from the
   same font-driver.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_list_entities (f: *mut frame, spec: Lisp_Object) -> Lisp_Object
{
  let mut driver_list: *mut font_driver_list = (*f).font_driver_list as *mut font_driver_list;
  let mut ftype: Lisp_Object;
  let mut val: Lisp_Object;
  let mut list: Lisp_Object = Qnil ();
  let mut size: c_int;
  let mut need_filtering: bool = false;
  let mut i: c_int;

  eassert! (FONT_SPEC_P (spec));

  if FIXNUMP (AREF (spec, FONT_SIZE_INDEX as isize))
    { size = XFIXNUM (AREF (spec, FONT_SIZE_INDEX as isize)) as c_int; }
  else if FLOATP (AREF (spec, FONT_SIZE_INDEX as isize))
    { size = font_pixel_size (f, spec); }
  else
    { size = 0; }

  ftype = AREF (spec, FONT_TYPE_INDEX as isize);
  i = FONT_FOUNDRY_INDEX;
  while i <= FONT_REGISTRY_INDEX
    {
      ASET (scratch_font_spec, i as isize, AREF (spec, i as isize));
      i += 1;
    }
  i = FONT_WEIGHT_INDEX;
  while i < FONT_EXTRA_INDEX
    {
      if i != FONT_SPACING_INDEX
	{
	  ASET (scratch_font_spec, i as isize, Qnil ());
	  if ! NILP (AREF (spec, i as isize))
	    { need_filtering = true; }
	}
      i += 1;
    }
  ASET (scratch_font_spec, FONT_SPACING_INDEX as isize,
	AREF (spec, FONT_SPACING_INDEX as isize));
  ASET (scratch_font_spec, FONT_EXTRA_INDEX as isize,
	AREF (spec, FONT_EXTRA_INDEX as isize));

  while !driver_list.is_null ()
    {
      if (*driver_list).on
	  && (NILP (ftype) || EQ ((*((*driver_list).driver)).r#type, ftype))
	{
	  let cache: Lisp_Object = font_get_cache (f, (*driver_list).driver);

	  ASET (scratch_font_spec, FONT_TYPE_INDEX as isize,
		(*((*driver_list).driver)).r#type);
	  val = assoc_no_quit (scratch_font_spec, XCDR (cache));
	  if CONSP (val)
	    { val = XCDR (val); }
	  else
	    {
	      let copy: Lisp_Object;

	      val = (((*(*driver_list).driver)).list.unwrap ()) (f as *mut c_void, scratch_font_spec);
	      /* We put zero_vector in the font-cache to indicate that
		 no fonts matching SPEC were found on the system.
		 Failure to have this indication in the font cache can
		 cause severe performance degradation in some rare
		 cases, see bug#21028.  */
	      if NILP (val)
		{ val = zero_vector; }
	      else
		{ val = Fvconcat (1, &mut val); }
	      copy = copy_font_spec (scratch_font_spec);
	      ASET (copy, FONT_TYPE_INDEX as isize, (*((*driver_list).driver)).r#type);
	      XSETCDR (cache, Fcons (Fcons (copy, val), XCDR (cache)));
	    }
	  if ASIZE (val) > 0
	      && (need_filtering
		  || ! NILP (Vface_ignored_fonts))
	    { val = font_delete_unmatched (val, if need_filtering { spec } else { Qnil () },
					   size); }
	  if ASIZE (val) > 0
	    {
	      list = Fcons (val, list);
	      /* Querying further backends can be very slow, so we only do
		 it if the user has explicitly requested it (Bug#43177).  */
	      if query_all_font_backends == false
		{ break; }
	    }
	}
      driver_list = (*driver_list).next;
    }

  list = Fnreverse (list);
  FONT_ADD_LOG! (c"list".as_ptr (), spec, list);
  return list;
}


/* Return a font entity matching with SPEC on FRAME.  ATTRS, if non
   nil, is an array of face's attributes, which specifies preferred
   font-related attributes.  */

unsafe fn font_matching_entity (f: *mut frame, attrs: *mut Lisp_Object,
				spec: Lisp_Object) -> Lisp_Object
{
  let mut driver_list: *mut font_driver_list = (*f).font_driver_list as *mut font_driver_list;
  let mut ftype: Lisp_Object;
  let size: Lisp_Object;
  let mut entity: Lisp_Object;
  let work: Lisp_Object = copy_font_spec (spec);

  ftype = AREF (spec, FONT_TYPE_INDEX as isize);
  size = AREF (spec, FONT_SIZE_INDEX as isize);

  if FLOATP (size)
    { ASET (work, FONT_SIZE_INDEX as isize, make_fixnum (font_pixel_size (f, spec) as EMACS_INT)); }
  FONT_SET_STYLE! (work, FONT_WEIGHT_INDEX, *attrs.offset (LFACE_WEIGHT_INDEX as isize));
  FONT_SET_STYLE! (work, FONT_SLANT_INDEX, *attrs.offset (LFACE_SLANT_INDEX as isize));
  FONT_SET_STYLE! (work, FONT_WIDTH_INDEX, *attrs.offset (LFACE_SWIDTH_INDEX as isize));

  entity = Qnil ();
  while !driver_list.is_null ()
    {
      if (*driver_list).on
	  && (NILP (ftype) || EQ ((*((*driver_list).driver)).r#type, ftype))
	{
	  let cache: Lisp_Object = font_get_cache (f, (*driver_list).driver);

	  ASET (work, FONT_TYPE_INDEX as isize, (*((*driver_list).driver)).r#type);
	  entity = assoc_no_quit (work, XCDR (cache));
	  if CONSP (entity)
	    { entity = AREF (XCDR (entity), 0); }
	  else
	    {
	      entity = ((*(*driver_list).driver)).r#match.unwrap () (f as *mut c_void, work);
	      if !NILP (entity)
		{
		  let copy: Lisp_Object = copy_font_spec (work);
		  let match_: Lisp_Object = Fvector (1, &mut entity);

		  ASET (copy, FONT_TYPE_INDEX as isize, (*((*driver_list).driver)).r#type);
		  XSETCDR (cache, Fcons (Fcons (copy, match_), XCDR (cache)));
		}
	    }
	  if ! NILP (entity)
	    { break; }
	}
      driver_list = (*driver_list).next;
    }
  FONT_ADD_LOG! (c"match".as_ptr (), work, entity);
  return entity;
}

/* Open a font of ENTITY and PIXEL_SIZE on frame F, and return the
   opened font object.  */

unsafe fn font_open_entity (f: *mut frame, entity: Lisp_Object, mut pixel_size: c_int)
			    -> Lisp_Object
{
  let mut driver_list: *mut font_driver_list;
  let mut objlist: Lisp_Object;
  let size: Lisp_Object;
  let mut val: Lisp_Object;
  let mut font_object: Lisp_Object;
  let mut font: *mut font;
  let mut height: c_int;
  let mut psize: c_int;

  eassert! (FONT_ENTITY_P (entity));
  size = AREF (entity, FONT_SIZE_INDEX as isize);
  if XFIXNUM (size) != 0
    { pixel_size = XFIXNUM (size) as c_int; }

  val = AREF (entity, FONT_TYPE_INDEX as isize);
  driver_list = (*f).font_driver_list as *mut font_driver_list;
  while !driver_list.is_null () && ! EQ ((*((*driver_list).driver)).r#type, val)
    { driver_list = (*driver_list).next; }
  if driver_list.is_null ()
    { return Qnil (); }

  objlist = AREF (entity, FONT_OBJLIST_INDEX as isize);
  while CONSP (objlist)
    {
      let fn_: Lisp_Object = XCAR (objlist);
      if ! NILP (AREF (fn_, FONT_TYPE_INDEX as isize))
	  && (*XFONT_OBJECT (fn_)).pixel_size == pixel_size
	{
	  if (*((*driver_list).driver)).cached_font_ok.is_none ()
	      || ((*((*driver_list).driver)).cached_font_ok.unwrap ()) (f as *mut c_void, fn_, entity)
	    { return fn_; }
	}
      objlist = XCDR (objlist);
    }

  /* We always open a font of manageable size; i.e non-zero average
     width and height.  */
  psize = pixel_size;
  loop
    {
      font_object = ((*((*driver_list).driver)).open_font.unwrap ()) (f as *mut c_void, entity, psize);
      if NILP (font_object)
	{ return Qnil (); }
      font = XFONT_OBJECT (font_object);
      if (*font).average_width > 0 && (*font).height > 0
	{ break; }
      /* Avoid an infinite loop.  */
      if psize > pixel_size + 15
	{ return Qnil (); }
      psize += 1;
    }
  ASET (font_object, FONT_SIZE_INDEX as isize, make_fixnum (pixel_size as EMACS_INT));
  FONT_ADD_LOG! (c"open".as_ptr (), entity, font_object);
  ASET (entity, FONT_OBJLIST_INDEX as isize,
	Fcons (font_object, AREF (entity, FONT_OBJLIST_INDEX as isize)));

  font = XFONT_OBJECT (font_object);
  /* C 2987-2991: `#ifdef HAVE_WINDOW_SYSTEM` (undef in the frozen configuration), carried.  */
  #[cfg(feature = "have_window_system")]
  let min_width: c_int = if (*font).min_width != 0 { (*font).min_width }
			 else if (*font).average_width != 0 { (*font).average_width }
			 else if (*font).space_width != 0 { (*font).space_width }
			 else { 1 };

  let mut font_ascent: c_int = 0;
  let mut font_descent: c_int = 0;
  get_font_ascent_descent (font, &mut font_ascent, &mut font_descent);
  height = font_ascent + font_descent;
  if height <= 0
    { height = 1; }
  /* C 2999-3013: `#ifdef HAVE_WINDOW_SYSTEM`, carried.  */
  #[cfg(feature = "have_window_system")]
  {
    (*FRAME_DISPLAY_INFO! (f)).n_fonts += 1;
    if (*FRAME_DISPLAY_INFO! (f)).n_fonts == 1
      {
	FRAME_SMALLEST_CHAR_WIDTH! (f, min_width);
	FRAME_SMALLEST_FONT_HEIGHT! (f, height);
	/* C 372: `f->fonts_changed = 1;` - the flag is a bit of the record's packed run.  */
	  let fonts_changed: u32 = 1;
	  frame_set_flag (f, FRAME_FONTS_CHANGED_UNIT, FRAME_FONTS_CHANGED_SHIFT, fonts_changed);
      }
    else
      {
	if FRAME_SMALLEST_CHAR_WIDTH! (f) > min_width
	  {
	    FRAME_SMALLEST_CHAR_WIDTH! (f, min_width);
	    /* C 372: `f->fonts_changed = 1;` - the flag is a bit of the record's packed run.  */
	  let fonts_changed: u32 = 1;
	  frame_set_flag (f, FRAME_FONTS_CHANGED_UNIT, FRAME_FONTS_CHANGED_SHIFT, fonts_changed);
	  }
	if FRAME_SMALLEST_FONT_HEIGHT! (f) > height
	  {
	    FRAME_SMALLEST_FONT_HEIGHT! (f, height);
	    /* C 372: `f->fonts_changed = 1;` - the flag is a bit of the record's packed run.  */
	  let fonts_changed: u32 = 1;
	  frame_set_flag (f, FRAME_FONTS_CHANGED_UNIT, FRAME_FONTS_CHANGED_SHIFT, fonts_changed);
	  }
      }
  }

  return font_object;
}


/* Close FONT_OBJECT that is opened on frame F.  */

unsafe fn font_close_object (f: *mut frame, font_object: Lisp_Object)
{
  let font: *mut font = XFONT_OBJECT (font_object);

  if NILP (AREF (font_object, FONT_TYPE_INDEX as isize))
    /* Already closed.  */
    { return; }
  FONT_ADD_LOG! (c"close".as_ptr (), font_object, Qnil ());
  ((*((*font).driver as *const font_driver)).close_font.unwrap ()) (font);
  /* C 3032-3034: `#ifdef HAVE_WINDOW_SYSTEM`, carried.  */
  #[cfg(feature = "have_window_system")]
  {
    eassert! ((*FRAME_DISPLAY_INFO! (f)).n_fonts != 0);
    (*FRAME_DISPLAY_INFO! (f)).n_fonts -= 1;
  }
}


/* Return 1 if FONT on F has a glyph for character C, 0 if not, -1 if
   FONT is a font-entity and it must be opened to check.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_has_char (f: *mut frame, font: Lisp_Object, c: c_int) -> c_int
{
  let mut fontp: *mut font;

  if FONT_ENTITY_P (font)
    {
      let r#type: Lisp_Object = AREF (font, FONT_TYPE_INDEX as isize);
      let mut driver_list: *mut font_driver_list;

      driver_list = (*f).font_driver_list as *mut font_driver_list;
      while !driver_list.is_null () && ! EQ ((*((*driver_list).driver)).r#type, r#type)
	{ driver_list = (*driver_list).next; }
      if driver_list.is_null ()
	{ return 0; }
      if (*((*driver_list).driver)).has_char.is_none ()
	{ return -1; }
      return ((*((*driver_list).driver)).has_char.unwrap ()) (font, c);
    }

  eassert! (FONT_OBJECT_P (font));
  fontp = XFONT_OBJECT (font);
  if (*((*fontp).driver as *const font_driver)).has_char.is_some ()
    {
      let result: c_int
	= ((*((*fontp).driver as *const font_driver)).has_char.unwrap ()) (font, c);

      if result >= 0
	{ return result; }
    }
  return (if ((*((*fontp).driver as *const font_driver)).encode_char.unwrap ()) (fontp, c)
	     != FONT_INVALID_CODE { 1 } else { 0 });
}


/* Return the glyph ID of FONT_OBJECT for character C.  */

unsafe fn font_encode_char (font_object: Lisp_Object, c: c_int) -> c_uint
{
  let font: *mut font;

  eassert! (FONT_OBJECT_P (font_object));
  font = XFONT_OBJECT (font_object);
  return ((*((*font).driver as *const font_driver)).encode_char.unwrap ()) (font, c);
}


/* Return the name of FONT_OBJECT.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_get_name (font_object: Lisp_Object) -> Lisp_Object
{
  eassert! (FONT_OBJECT_P (font_object));
  return AREF (font_object, FONT_NAME_INDEX as isize);
}


/* Create a new font spec from FONT_NAME, and return it.  If FONT_NAME
   could not be parsed by font_parse_name, return Qnil.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_spec_from_name (font_name: Lisp_Object) -> Lisp_Object
{
  let spec: Lisp_Object = Ffont_spec (0, std::ptr::null_mut ());

  CHECK_STRING (font_name);
  if font_parse_name (SSDATA (font_name), SBYTES (font_name), spec) == -1
    { return Qnil (); }
  font_put_extra (spec, QCname (), font_name);
  font_put_extra (spec, QCuser_spec (), font_name);
  return spec;
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_clear_prop (attrs: *mut Lisp_Object, prop: font_property_index)
{
  let mut font: Lisp_Object = *attrs.offset (LFACE_FONT_INDEX as isize);

  if ! FONTP (font)
    { return; }

  if ! NILP (Ffont_get (font, QCname ()))
    {
      font = copy_font_spec (font);
      font_put_extra (font, QCname (), Qunbound ());
    }

  if NILP (AREF (font, prop as isize))
      && prop != FONT_FAMILY_INDEX
      && prop != FONT_FOUNDRY_INDEX
      && prop != FONT_WIDTH_INDEX
      && prop != FONT_SIZE_INDEX
    { return; }
  if EQ (font, *attrs.offset (LFACE_FONT_INDEX as isize))
    { font = copy_font_spec (font); }
  ASET (font, prop as isize, Qnil ());
  if prop == FONT_FAMILY_INDEX || prop == FONT_FOUNDRY_INDEX
    {
      if prop == FONT_FAMILY_INDEX
	{
	  ASET (font, FONT_FOUNDRY_INDEX as isize, Qnil ());
	  /* If we are setting the font family, we must also clear
	     FONT_WIDTH_INDEX to avoid rejecting families that lack
	     support for some widths.  */
	  ASET (font, FONT_WIDTH_INDEX as isize, Qnil ());
	}
      ASET (font, FONT_ADSTYLE_INDEX as isize, Qnil ());
      ASET (font, FONT_REGISTRY_INDEX as isize, Qnil ());
      ASET (font, FONT_SIZE_INDEX as isize, Qnil ());
      ASET (font, FONT_DPI_INDEX as isize, Qnil ());
      ASET (font, FONT_SPACING_INDEX as isize, Qnil ());
      ASET (font, FONT_AVGWIDTH_INDEX as isize, Qnil ());
    }
  else if prop == FONT_SIZE_INDEX
    {
      ASET (font, FONT_DPI_INDEX as isize, Qnil ());
      ASET (font, FONT_SPACING_INDEX as isize, Qnil ());
      ASET (font, FONT_AVGWIDTH_INDEX as isize, Qnil ());
    }
  else if prop == FONT_WIDTH_INDEX
    { ASET (font, FONT_AVGWIDTH_INDEX as isize, Qnil ()); }
  *attrs.offset (LFACE_FONT_INDEX as isize) = font;
}

/* Select a font from ENTITIES (list of one or more font-entity
   vectors) that supports the character C (if non-negative) and is the
   best match for ATTRS and PIXEL_SIZE.  */

unsafe fn font_select_entity (f: *mut frame, entities: Lisp_Object,
			      attrs: *mut Lisp_Object, pixel_size: c_int,
			      c: c_int) -> Lisp_Object
{
  let font_entity: Lisp_Object;
  let prefer: Lisp_Object;
  let mut i: c_int;

  /* If we have a single candidate, return it if it supports C.  */
  if NILP (XCDR (entities))
      && ASIZE (XCAR (entities)) == 1
    {
      font_entity = AREF (XCAR (entities), 0);
      if c < 0 || font_has_char (f, font_entity, c) > 0
	{ return font_entity; }
      return Qnil ();
    }

  /* If we have several candidates, find the best match by sorting
     them by properties specified in ATTRS.  Style attributes (weight,
     slant, width, and size) are taken from the font spec in ATTRS (if
     that is non-nil), or from ATTRS, or left as nil.  */
  prefer = scratch_font_prefer;

  i = FONT_WEIGHT_INDEX;
  while i <= FONT_SIZE_INDEX
    {
      ASET (prefer, i as isize, Qnil ());
      i += 1;
    }
  if FONTP (*attrs.offset (LFACE_FONT_INDEX as isize))
    {
      let face_font: Lisp_Object = *attrs.offset (LFACE_FONT_INDEX as isize);

      i = FONT_WEIGHT_INDEX;
      while i <= FONT_SIZE_INDEX
	{
	  ASET (prefer, i as isize, AREF (face_font, i as isize));
	  i += 1;
	}
    }
  if NILP (AREF (prefer, FONT_WEIGHT_INDEX as isize))
    { FONT_SET_STYLE! (prefer, FONT_WEIGHT_INDEX, *attrs.offset (LFACE_WEIGHT_INDEX as isize)); }
  if NILP (AREF (prefer, FONT_SLANT_INDEX as isize))
    { FONT_SET_STYLE! (prefer, FONT_SLANT_INDEX, *attrs.offset (LFACE_SLANT_INDEX as isize)); }
  if NILP (AREF (prefer, FONT_WIDTH_INDEX as isize))
    { FONT_SET_STYLE! (prefer, FONT_WIDTH_INDEX, *attrs.offset (LFACE_SWIDTH_INDEX as isize)); }
  ASET (prefer, FONT_SIZE_INDEX as isize, make_fixnum (pixel_size as EMACS_INT));

  return font_sort_entities (entities, prefer, f, c);
}

/* Return a font-entity that satisfies SPEC and is the best match for
   face's font related attributes in ATTRS.  C, if not negative, is a
   character that the entity must support.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_find_for_lface (f: *mut frame, attrs: *mut Lisp_Object,
					      spec: Lisp_Object, c: c_int) -> Lisp_Object
{
  let work: Lisp_Object;
  let mut entities: Lisp_Object;
  let mut val: Lisp_Object;
  let mut foundry: [Lisp_Object; 3] = [Qnil (); 3];
  let mut family: *mut Lisp_Object;
  let mut registry: [Lisp_Object; 3] = [Qnil (); 3];
  let mut adstyle: [Lisp_Object; 3] = [Qnil (); 3];
  let mut pixel_size: c_int;
  let mut i: c_int;
  let mut j: c_int;
  let mut k: c_int;
  let mut l: c_int;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  /* Registry specification alternatives: from the most specific to
     the least specific and finally an unspecified one.  */
  registry[0] = AREF (spec, FONT_REGISTRY_INDEX as isize);
  if NILP (registry[0])
    {
      registry[0] = DEFAULT_ENCODING! ();
      registry[1] = Qascii_0 ();
      registry[2] = zero_vector;
    }
  else
    { registry[1] = zero_vector; }

  if c >= 0 && ! NILP (AREF (spec, FONT_REGISTRY_INDEX as isize))
    {
      /* PORTING.md 1.5: C declares both pointers and passes their addresses.  */
      let mut encoding: *mut charset = std::ptr::null_mut ();
      let mut repertory: *mut charset = std::ptr::null_mut ();

      if font_registry_charsets (AREF (spec, FONT_REGISTRY_INDEX as isize),
				 &mut encoding, &mut repertory) < 0
	{ return Qnil (); }
      if !repertory.is_null ()
	  && ENCODE_CHAR! (repertory, c) == CHARSET_INVALID_CODE! (repertory)
	{ return Qnil (); }
      else if c > (*encoding).max_char
	{ return Qnil (); }
    }

  work = copy_font_spec (spec);
  ASET (work, FONT_TYPE_INDEX as isize, AREF (spec, FONT_TYPE_INDEX as isize));
  pixel_size = font_pixel_size (f, spec);
  if pixel_size == 0 && FIXNUMP (*attrs.offset (LFACE_HEIGHT_INDEX as isize))
    {
      let pt: c_double = XFIXNUM (*attrs.offset (LFACE_HEIGHT_INDEX as isize)) as c_double;

      pixel_size = POINT_TO_PIXEL! (pt / 10.0, FRAME_RES (f)) as c_int;
      if pixel_size < 1
	{ pixel_size = 1; }
    }
  ASET (work, FONT_SIZE_INDEX as isize, Qnil ());

  /* Foundry specification alternatives: from the most specific to the
     least specific and finally an unspecified one.  */
  foundry[0] = AREF (work, FONT_FOUNDRY_INDEX as isize);
  if ! NILP (foundry[0])
    { foundry[1] = zero_vector; }
  else if STRINGP (*attrs.offset (LFACE_FOUNDRY_INDEX as isize))
    {
      val = *attrs.offset (LFACE_FOUNDRY_INDEX as isize);
      foundry[0] = font_intern_prop (SSDATA (val), SBYTES (val), true);
      foundry[1] = Qnil ();
      foundry[2] = zero_vector;
    }
  else
    {
      foundry[0] = Qnil ();
      foundry[1] = zero_vector;
    }

  /* Additional style specification alternatives: from the most
     specific to the least specific and finally an unspecified one.  */
  adstyle[0] = AREF (work, FONT_ADSTYLE_INDEX as isize);
  if ! NILP (adstyle[0])
    { adstyle[1] = zero_vector; }
  else if FONTP (*attrs.offset (LFACE_FONT_INDEX as isize))
    {
      let face_font: Lisp_Object = *attrs.offset (LFACE_FONT_INDEX as isize);

      if ! NILP (AREF (face_font, FONT_ADSTYLE_INDEX as isize))
	{
	  adstyle[0] = AREF (face_font, FONT_ADSTYLE_INDEX as isize);
	  adstyle[1] = Qnil ();
	  adstyle[2] = zero_vector;
	}
      else
	{
	  adstyle[0] = Qnil ();
	  adstyle[1] = zero_vector;
	}
    }
  else
    {
      adstyle[0] = Qnil ();
      adstyle[1] = zero_vector;
    }


  /* Family specification alternatives: from the most specific to
     the least specific and finally an unspecified one.  */
  val = AREF (work, FONT_FAMILY_INDEX as isize);
  if NILP (val) && STRINGP (*attrs.offset (LFACE_FAMILY_INDEX as isize))
    {
      val = *attrs.offset (LFACE_FAMILY_INDEX as isize);
      val = font_intern_prop (SSDATA (val), SBYTES (val), true);
    }
  let mut familybuf: [Lisp_Object; 3] = [Qnil (); 3];
  if NILP (val)
    {
      family = familybuf.as_mut_ptr ();
      *family.offset (0) = Qnil ();
      *family.offset (1) = zero_vector;	/* terminator.  */
    }
  else
    {
      let alters: Lisp_Object
	= Fassoc_string (val, Vface_alternative_font_family_alist, Qt ());

      if ! NILP (alters)
	{
	  let alterslen: EMACS_INT = list_length (alters) as EMACS_INT;
	  family = std::ptr::null_mut ();
	  SAFE_ALLOCA_LISP! (sa_avail, family, alterslen + 2);
	  i = 0;
	  let mut alters: Lisp_Object = alters;
	  while CONSP (alters)
	    {
	      *family.offset (i as isize) = XCAR (alters);
	      i += 1;
	      alters = XCDR (alters);
	    }
	  if NILP (AREF (spec, FONT_FAMILY_INDEX as isize))
	    {
	      *family.offset (i as isize) = Qnil ();
	      i += 1;
	    }
	  *family.offset (i as isize) = zero_vector;
	}
      else
	{
	  family = familybuf.as_mut_ptr ();
	  i = 0;
	  *family.offset (i as isize) = val;
	  i += 1;
	  if NILP (AREF (spec, FONT_FAMILY_INDEX as isize))
	    {
	      *family.offset (i as isize) = Qnil ();
	      i += 1;
	    }
	  *family.offset (i as isize) = zero_vector;
	}
    }

  /* Now look up suitable fonts, from the most specific spec to the
     least specific spec.  Accept the first one that matches.  */
  i = 0;
  while SYMBOLP (*family.offset (i as isize))
    {
      ASET (work, FONT_FAMILY_INDEX as isize, *family.offset (i as isize));
      j = 0;
      while SYMBOLP (foundry[j as usize])
	{
	  ASET (work, FONT_FOUNDRY_INDEX as isize, foundry[j as usize]);
	  k = 0;
	  while SYMBOLP (registry[k as usize])
	    {
	      ASET (work, FONT_REGISTRY_INDEX as isize, registry[k as usize]);
	      l = 0;
	      while SYMBOLP (adstyle[l as usize])
		{
		  ASET (work, FONT_ADSTYLE_INDEX as isize, adstyle[l as usize]);
		  /* Produce the list of candidates for the spec in WORK.  */
		  entities = font_list_entities (f, work);
		  if ! NILP (entities)
		    {
		      /* If there are several candidates, select the
			 best match for PIXEL_SIZE and attributes in ATTRS.  */
		      val = font_select_entity (f, entities,
						attrs, pixel_size, c);
		      if ! NILP (val)
			{
			  SAFE_FREE! (sa_count, sa_mark);
			  return val;
			}
		    }
		  l += 1;
		}
	      k += 1;
	    }
	  j += 1;
	}
      i += 1;
    }

  SAFE_FREE! (sa_count, sa_mark);
  return Qnil ();
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_open_for_lface (f: *mut frame, entity: Lisp_Object,
					      attrs: *mut Lisp_Object,
					      spec: Lisp_Object) -> Lisp_Object
{
  let mut size: c_int;

  if FIXNUMP (AREF (entity, FONT_SIZE_INDEX as isize))
      && XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize)) > 0
    { size = XFIXNUM (AREF (entity, FONT_SIZE_INDEX as isize)) as c_int; }
  else
    {
      if FONT_SPEC_P (spec) && ! NILP (AREF (spec, FONT_SIZE_INDEX as isize))
	{ size = font_pixel_size (f, spec); }
      else
	{
	  let mut pt: c_double;

	  if FIXNUMP (*attrs.offset (LFACE_HEIGHT_INDEX as isize))
	    { pt = XFIXNUM (*attrs.offset (LFACE_HEIGHT_INDEX as isize)) as c_double; }
	  else
	    {
	      /* We need the default face to be valid below.  */
	      if (*FRAME_FACE_CACHE! (f)).used == 0
		{ recompute_basic_faces (f); }

	      let def: *mut face = FACE_FROM_ID (f, DEFAULT_FACE_ID);
	      let height: Lisp_Object = (*def).lface[LFACE_HEIGHT_INDEX as usize];
	      eassert! (FIXNUMP (height));
	      pt = XFIXNUM (height) as c_double;
	    }

	  pt /= 10.0;
	  size = POINT_TO_PIXEL! (pt, FRAME_RES (f)) as c_int;
	  /* C 3414-3421: `#ifdef HAVE_NS` (undef in the frozen configuration), carried.  */
	  #[cfg(feature = "have_ns")]
	  {
	    if size == 0
	      {
		let ffsize: Lisp_Object = get_frame_param (f, Qfontsize ());
		size = (if NUMBERP (ffsize)
			{ POINT_TO_PIXEL! (XFLOATINT (ffsize), FRAME_RES (f)) as c_int }
			else { 0 });
	      }
	  }
	}
      size = (size as c_double * font_rescale_ratio (entity)) as c_int;
    }

  return font_open_entity (f, entity, size);
}


/* Find a font that satisfies SPEC and is the best match for
   face's attributes in ATTRS on FRAME, and return the opened
   font-object.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_load_for_lface (f: *mut frame, attrs: *mut Lisp_Object,
					      spec: Lisp_Object) -> Lisp_Object
{
  let mut entity: Lisp_Object;
  let mut name: Lisp_Object;

  entity = font_find_for_lface (f, attrs, spec, -1);
  if NILP (entity)
    {
      /* No font is listed for SPEC, but each font-backend may have
	 different criteria about "font matching".  So, try it.  */
      entity = font_matching_entity (f, attrs, spec);
      /* Perhaps the user asked for a font "Foobar-123", and we
	 interpreted "-123" as the size, whereas it really is part of
	 the name.  So we reset the size to nil and the family name to
	 the entire "Foobar-123" thing, and try again with that.  */
      if NILP (entity)
	{
	  name = Ffont_get (spec, QCuser_spec ());
	  if STRINGP (name)
	    {
	      let p: *mut c_char = SSDATA (name);
	      let q: *mut c_char = strrchr (p, b'-' as c_int);

	      if !q.is_null () && c_isdigit (*q.offset (1) as c_int)
		{
		  /* PORTING.md 1.5: C declares `char *tail;` and `strtod` writes it.  */
		  let mut tail: *mut c_char = std::ptr::null_mut ();
		  let font_size: c_double = strtod (q.offset (1), &mut tail);

		  if font_size > 0.0 && tail != q.offset (1)
		    {
		      let lsize: Lisp_Object = Ffont_get (spec, QCsize ());

		      if (FLOATP (lsize) && XFLOAT_DATA (lsize) == font_size)
			  || (FIXNUMP (lsize) && XFIXNUM (lsize) as c_double == font_size)
			{
			  ASET (spec, FONT_FAMILY_INDEX as isize,
				font_intern_prop (p, tail.offset_from (p), true));
			  ASET (spec, FONT_SIZE_INDEX as isize, Qnil ());
			  entity = font_matching_entity (f, attrs, spec);
			}
		    }
		}
	    }
	}
      if NILP (entity)
	{ return Qnil (); }
    }
  /* Don't lose the original name that was put in initially.  We need
     it to re-apply the font when font parameters (like hinting or dpi) have
     changed.  */
  entity = font_open_for_lface (f, entity, attrs, spec);
  if !NILP (entity)
    {
      name = Ffont_get (spec, QCuser_spec ());
      if STRINGP (name)
	{ font_put_extra (entity, QCuser_spec (), name); }
    }
  return entity;
}


/* Make FACE on frame F ready to use the font opened for FACE.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_prepare_for_face (f: *mut frame, face: *mut face)
{
  if (*((*(*face).font).driver as *const font_driver)).prepare_face.is_some ()
    {
      let prepare_face = (*((*(*face).font).driver as *const font_driver)).prepare_face;
      if let Some (p) = prepare_face
	{ p (f as *mut c_void, face as *mut c_void); }
    }
}


/* Make FACE on frame F stop using the font opened for FACE.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_done_for_face (f: *mut frame, face: *mut face)
{
  if (*((*(*face).font).driver as *const font_driver)).done_face.is_some ()
    {
      let done_face = (*((*(*face).font).driver as *const font_driver)).done_face;
      if let Some (d) = done_face
	{ d (f as *mut c_void, face as *mut c_void); }
    }
}


/* Open a font that is a match for font-spec SPEC on frame F.  If no proper
   font is found, return Qnil.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_open_by_spec (f: *mut frame, spec: Lisp_Object) -> Lisp_Object
{
  let mut attrs: [Lisp_Object; LFACE_VECTOR_SIZE as usize] = [Qnil (); LFACE_VECTOR_SIZE as usize];

  /* We set up the default font-related attributes of a face to prefer
     a moderate font.  */
  attrs[LFACE_FAMILY_INDEX as usize] = Qnil ();
  attrs[LFACE_FOUNDRY_INDEX as usize] = Qnil ();
  attrs[LFACE_SWIDTH_INDEX as usize] = Qnormal ();
  attrs[LFACE_WEIGHT_INDEX as usize] = Qnormal ();
  attrs[LFACE_SLANT_INDEX as usize] = Qnormal ();
  /* C 3527-3530: `#ifndef HAVE_NS` is live on the frozen target (`HAVE_NS` is undef); the
     `#else` arm is carried under the feature that names the condition's other side.  */
  #[cfg(not (feature = "have_ns"))]
  { attrs[LFACE_HEIGHT_INDEX as usize] = make_fixnum (120); }
  #[cfg(feature = "have_ns")]
  { attrs[LFACE_HEIGHT_INDEX as usize] = make_fixnum (0); }
  attrs[LFACE_FONT_INDEX as usize] = Qnil ();

  return font_load_for_lface (f, attrs.as_mut_ptr (), spec);
}


/* Open a font that matches NAME on frame F.  If no proper font is
   found, return Qnil.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_open_by_name (f: *mut frame, name: Lisp_Object) -> Lisp_Object
{
  let spec: Lisp_Object = CALLN! (Ffont_spec, QCname (), name);
  let ret: Lisp_Object = font_open_by_spec (f, spec);
  /* Do not lose name originally put in.  */
  if !NILP (ret)
    { font_put_extra (ret, QCuser_spec (), name); }

  return ret;
}


/* Register font-driver DRIVER.  This function is used in two ways.

   The first is with frame F non-NULL.  In this case, make DRIVER
   available (but not yet activated) on F.  All frame creators
   (e.g. Fx_create_frame) must call this function at least once with
   an available font-driver.

   The second is with frame F NULL.  In this case, DRIVER is globally
   registered in the variable `font_driver_list'.  All font-driver
   implementations must call this function in its
   syms_of_XXXX_for_pdumper (e.g. syms_of_xfont_for_pdumper).  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn register_font_driver (driver: *const font_driver, f: *mut frame)
{
  let root: *mut font_driver_list = if !f.is_null ()
				    { (*f).font_driver_list as *mut font_driver_list }
				    else { font_driver_list };
  let mut prev: *mut font_driver_list;
  let mut list: *mut font_driver_list;

  /* C 3572-3575: `#ifdef HAVE_WINDOW_SYSTEM` (undef in the frozen configuration), carried.  */
  #[cfg(feature = "have_window_system")]
  {
    if !f.is_null () && (*driver).draw.is_none ()
      { error (c"Unusable font driver for a frame: %s".as_ptr (),
	       SDATA (SYMBOL_NAME ((*driver).r#type))); }
  }

  prev = std::ptr::null_mut ();
  list = root;
  while !list.is_null ()
    {
      if EQ ((*((*list).driver)).r#type, (*driver).r#type)
	{ error (c"Duplicated font driver: %s".as_ptr (),
		 SDATA (SYMBOL_NAME ((*driver).r#type))); }
      prev = list;
      list = (*list).next;
    }

  list = xmalloc (std::mem::size_of::<font_driver_list> ()) as *mut font_driver_list;
  (*list).on = false;
  (*list).driver = driver;
  (*list).next = std::ptr::null_mut ();
  if !prev.is_null ()
    { (*prev).next = list; }
  else if !f.is_null ()
    { (*f).font_driver_list = list as *mut c_void; }
  else
    { font_driver_list = list; }
  if f.is_null ()
    { num_font_drivers += 1; }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_font_driver_list (f: *mut frame)
{
  let mut list: *mut font_driver_list;
  let mut next: *mut font_driver_list;

  list = (*f).font_driver_list as *mut font_driver_list;
  while !list.is_null ()
    {
      next = (*list).next;
      xfree (list as *mut c_void);
      list = next;
    }
  (*f).font_driver_list = std::ptr::null_mut ();
}


/* Make the frame F use font backends listed in NEW_DRIVERS (list of
   symbols, e.g. xft, x).  If NEW_DRIVERS is t, make F use all
   available font drivers that are not superseded by another driver.
   (A font driver SYMBOL is superseded by the driver specified by
   SYMBOL's 'font-driver-superseded-by property if it is a non-nil
   symbol.)  If NEW_DRIVERS is nil, finalize all drivers.

   A caller must free all realized faces if any in advance.  The
   return value is a list of font backends actually made used on
   F.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_update_drivers (f: *mut frame,
					      new_drivers: Lisp_Object) -> Lisp_Object
{
  let mut active_drivers: Lisp_Object = Qnil ();
  let mut default_drivers: Lisp_Object = Qnil ();
  let mut list: *mut font_driver_list;
  let mut new_drivers: Lisp_Object = new_drivers;

  /* Collect all unsuperseded driver symbols into
     `default_drivers'.  */
  let mut all_drivers: Lisp_Object = Qnil ();
  list = (*f).font_driver_list as *mut font_driver_list;
  while !list.is_null ()
    {
      all_drivers = Fcons ((*((*list).driver)).r#type, all_drivers);
      list = (*list).next;
    }
  let mut rest: Lisp_Object = all_drivers;
  while CONSP (rest)
    {
      let superseded_by: Lisp_Object
	= Fget (XCAR (rest), Qfont_driver_superseded_by ());

      if NILP (superseded_by)
	  || NILP (Fmemq (superseded_by, all_drivers))
	{ default_drivers = Fcons (XCAR (rest), default_drivers); }
      rest = XCDR (rest);
    }

  if EQ (new_drivers, Qt ())
    { new_drivers = default_drivers; }

  /* At first, turn off non-requested drivers, and turn on requested
     drivers.  */
  list = (*f).font_driver_list as *mut font_driver_list;
  while !list.is_null ()
    {
      let driver: *const font_driver = (*list).driver;
      if (! NILP (Fmemq ((*driver).r#type, new_drivers))) != (*list).on
	{
	  if (*list).on
	    {
	      if (*driver).end_for_frame.is_some ()
		{
		  let end_for_frame = (*driver).end_for_frame;
		  if let Some (e) = end_for_frame
		    { e (f as *mut c_void); }
		}
	      font_finish_cache (f, driver);
	      (*list).on = false;
	    }
	  else
	    {
	      if (*driver).start_for_frame.is_none ()
		  || ((*driver).start_for_frame.unwrap ()) (f as *mut c_void) == 0
		{
		  font_prepare_cache (f, driver);
		  (*list).on = true;
		}
	    }
	}
      list = (*list).next;
    }

  if NILP (new_drivers)
    { return Qnil (); }
  else
    {
      /* Re-order the driver list according to new_drivers.  */
      let mut list_table: *mut *mut font_driver_list = std::ptr::null_mut ();
      let mut next: *mut *mut c_void;
      let mut tail: Lisp_Object;
      let mut i: c_int;
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

      SAFE_NALLOCA! (sa_avail, list_table, 1, num_font_drivers + 1);
      i = 0;
      tail = new_drivers;
      while ! NILP (tail)
	{
	  list = (*f).font_driver_list as *mut font_driver_list;
	  while !list.is_null ()
	    {
	      if (*list).on && EQ ((*((*list).driver)).r#type, XCAR (tail))
		{ break; }
	      list = (*list).next;
	    }
	  if !list.is_null ()
	    {
	      *list_table.offset (i as isize) = list;
	      i += 1;
	    }
	  tail = XCDR (tail);
	}
      list = (*f).font_driver_list as *mut font_driver_list;
      while !list.is_null ()
	{
	  if ! (*list).on
	    {
	      *list_table.offset (i as isize) = list;
	      i += 1;
	    }
	  list = (*list).next;
	}
      *list_table.offset (i as isize) = std::ptr::null_mut ();

      next = &mut (*f).font_driver_list;
      i = 0;
      while !(*list_table.offset (i as isize)).is_null ()
	{
	  *next = *list_table.offset (i as isize) as *mut c_void;
	  next = &mut (**list_table.offset (i as isize)).next
	    as *mut *mut font_driver_list as *mut *mut c_void;
	  i += 1;
	}
      *next = std::ptr::null_mut ();
      SAFE_FREE! (sa_count, sa_mark);

      if !((*( (*f).font_driver_list as *mut font_driver_list )).on)
	{ /* None of the drivers is enabled: enable them all.
	     Happens if you set the list of drivers to (xft x) in your .emacs
	     and then use it under w32 or ns.  */
	  list = (*f).font_driver_list as *mut font_driver_list;
	  while !list.is_null ()
	    {
	      let driver: *const font_driver = (*list).driver;
	      eassert! (! (*list).on);
	      if NILP (Fmemq ((*driver).r#type, default_drivers))
		{
		  list = (*list).next;
		  continue;
		}
	      if (*driver).start_for_frame.is_none ()
		  || ((*driver).start_for_frame.unwrap ()) (f as *mut c_void) == 0
		{
		  font_prepare_cache (f, driver);
		  (*list).on = true;
		}
	      list = (*list).next;
	    }
	}
    }

  list = (*f).font_driver_list as *mut font_driver_list;
  while !list.is_null ()
    {
      if (*list).on
	{ active_drivers = nconc2 (active_drivers, list1 ((*((*list).driver)).r#type)); }
      list = (*list).next;
    }
  return active_drivers;
}

/* C 3729: `#if (defined HAVE_XFT || defined HAVE_FREETYPE) && !defined USE_CAIRO` (all undef
   in the frozen configuration), carried.  */

#[cfg(feature = "xft_or_freetype")]
unsafe fn fset_font_data (f: *mut frame, val: Lisp_Object)
{
  (*f).font_data = val;
}

#[cfg(feature = "xft_or_freetype")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_put_frame_data (f: *mut frame, driver: Lisp_Object,
					      data: *mut c_void)
{
  let val: Lisp_Object = assq_no_quit (driver, (*f).font_data);

  if data.is_null ()
    { fset_font_data (f, Fdelq (val, (*f).font_data)); }
  else
    {
      if NILP (val)
	{ fset_font_data (f, Fcons (Fcons (driver, make_mint_ptr (data)),
				    (*f).font_data)); }
      else
	{ XSETCDR (val, make_mint_ptr (data)); }
    }
}

#[cfg(feature = "xft_or_freetype")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_get_frame_data (f: *mut frame, driver: Lisp_Object)
					       -> *mut c_void
{
  let val: Lisp_Object = assq_no_quit (driver, (*f).font_data);

  return if NILP (val) { std::ptr::null_mut () } else { xmint_pointer (XCDR (val)) };
}

/* Sets attributes on a font.  Any properties that appear in ALIST and
   BOOLEAN_PROPERTIES or NON_BOOLEAN_PROPERTIES are set on the font.
   BOOLEAN_PROPERTIES and NON_BOOLEAN_PROPERTIES are NULL-terminated
   arrays of strings.  This function is intended for use by the font
   drivers to implement their specific font_filter_properties.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_filter_properties (font: Lisp_Object, alist: Lisp_Object,
						 boolean_properties: *const *const c_char,
						 non_boolean_properties: *const *const c_char)
{
  let mut it: Lisp_Object;
  let mut i: c_int;

  /* Set boolean values to Qt or Qnil.  */
  i = 0;
  while !(*boolean_properties.offset (i as isize)).is_null ()
    {
      it = alist;
      while ! NILP (it)
	{
	  let key: Lisp_Object = XCAR (XCAR (it));
	  let mut val: Lisp_Object = XCDR (XCAR (it));
	  let keystr: *mut c_char = SSDATA (SYMBOL_NAME (key));

	  if strcmp (*boolean_properties.offset (i as isize), keystr) == 0
	    {
	      let str_: *const c_char
		= if FIXNUMP (val) { if XFIXNUM (val) != 0 { c"true".as_ptr () }
				     else { c"false".as_ptr () } }
		  else if SYMBOLP (val) { SSDATA (SYMBOL_NAME (val)) }
		  else { c"true".as_ptr () };

	      if strcmp (c"false".as_ptr (), str_) == 0
		  || strcmp (c"False".as_ptr (), str_) == 0
		  || strcmp (c"FALSE".as_ptr (), str_) == 0
		  || strcmp (c"FcFalse".as_ptr (), str_) == 0
		  || strcmp (c"off".as_ptr (), str_) == 0
		  || strcmp (c"OFF".as_ptr (), str_) == 0
		  || strcmp (c"Off".as_ptr (), str_) == 0
		{ val = Qnil (); }
	      else
		{ val = Qt (); }

	      Ffont_put (font, key, val);
	    }
	  it = XCDR (it);
	}
      i += 1;
    }

  i = 0;
  while !(*non_boolean_properties.offset (i as isize)).is_null ()
    {
      it = alist;
      while ! NILP (it)
	{
	  let key: Lisp_Object = XCAR (XCAR (it));
	  let val: Lisp_Object = XCDR (XCAR (it));
	  let keystr: *mut c_char = SSDATA (SYMBOL_NAME (key));
	  if strcmp (*non_boolean_properties.offset (i as isize), keystr) == 0
	    { Ffont_put (font, key, val); }
	  it = XCDR (it);
	}
      i += 1;
    }
}

/* Return the font used to draw character C by FACE at buffer position
   POS in window W.  If STRING is non-nil, it is a string containing C
   at index POS.  If C is negative, get C from the current buffer or
   STRING.  */

unsafe fn font_at (c: c_int, pos: ptrdiff_t, face: *mut face, w: *mut window,
		   string: Lisp_Object) -> Lisp_Object
{
  let mut f: *mut frame;
  let mut multibyte: bool;
  let font_object: Lisp_Object;
  let mut c: c_int = c;
  let mut face: *mut face = face;

  multibyte = if NILP (string)
	      { ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)) }
	      else { STRING_MULTIBYTE (string) };
  if c < 0
    {
      if NILP (string)
	{
	  if multibyte
	    {
	      let pos_byte: ptrdiff_t = CHAR_TO_BYTE (pos);

	      c = FETCH_CHAR (pos_byte);
	    }
	  else
	    { c = FETCH_BYTE! (pos) as c_int; }
	}
      else
	{
	  let mut str: *mut c_uchar;

	  multibyte = STRING_MULTIBYTE (string);
	  if multibyte
	    {
	      let pos_byte: ptrdiff_t = string_char_to_byte (string, pos);

	      str = SDATA (string).offset (pos_byte);
	      c = STRING_CHAR (str);
	    }
	  else
	    { c = *SDATA (string).offset (pos) as c_int; }
	}
    }

  f = XFRAME ((*w).frame);
  if ! FRAME_WINDOW_P (f)
    { return Qnil (); }
  if face.is_null ()
    {
      let face_id: c_int;
      let mut endptr: ptrdiff_t = 0;

      if STRINGP (string)
	{ face_id = face_at_string_position (w, string, pos, 0, &mut endptr,
					     DEFAULT_FACE_ID, false, 0); }
      else
	{ face_id = face_at_buffer_position (w, pos, &mut endptr,
					     pos + 100, false, -1, 0); }
      face = FACE_FROM_ID (f, face_id);
    }
  if multibyte
    {
      let face_id: c_int = FACE_FOR_CHAR (f, face, c, pos, string);
      face = FACE_FROM_ID (f, face_id);
    }
  if (*face).font.is_null ()
    { return Qnil (); }

  XSETFONT! (font_object, (*face).font);
  return font_object;
}

/* C 3890: `#ifdef HAVE_WINDOW_SYSTEM` (undef in the frozen configuration), carried; the
   three definitions below are C 3892-3993's.  */

/// C: dispextern.h 3820: `Lisp_Object font_for_char (struct face *, int, ptrdiff_t,
/// Lisp_Object);`, xfaces.c's — reached only through the `HAVE_WINDOW_SYSTEM` arm of
/// `font_range`.
#[cfg(feature = "have_window_system")]
unsafe extern "C" {
  /// C: dispextern.h 3820: `Lisp_Object font_for_char (struct face *, int, ptrdiff_t,
  /// Lisp_Object);`
  pub fn font_for_char (face: *mut face, ch: c_int, pos: ptrdiff_t,
                        object: Lisp_Object) -> Lisp_Object;
}

/* Check if CH is a codepoint for which we should attempt to use the
   emoji font, even if the codepoint itself has Emoji_Presentation =
   No.  Vauto_composition_emoji_eligible_codepoints is filled in for
   us by admin/unidata/emoji-zwj.awk.  */
#[cfg(feature = "have_window_system")]
unsafe fn codepoint_is_emoji_eligible (ch: c_int) -> bool
{
  if EQ (CHAR_TABLE_REF (Vchar_script_table, ch as isize), Qemoji ())
    { return true; }

  if ! NILP (Fmemq (make_fixnum (ch as EMACS_INT),
		    Vauto_composition_emoji_eligible_codepoints))
    { return true; }

  return false;
}

/* Check how many characters after character/byte position POS/POS_BYTE
   (at most to *LIMIT) can be displayed by the same font in the window W.
   FACE, if non-NULL, is the face selected for the character at POS.
   If STRING is not nil, it is the string to check instead of the current
   buffer.  In that case, FACE must be not NULL.

   CH is the character that actually caused the composition
   process to start, it may be different from the character at POS.

   The return value is the font-object for the character at POS.
   *LIMIT is set to the position where that font can't be used.

   It is assured that the current buffer (or STRING) is multibyte.  */

#[cfg(feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_range (pos: ptrdiff_t, pos_byte: ptrdiff_t,
				     limit: *mut ptrdiff_t, w: *mut window,
				     face: *mut face, string: Lisp_Object,
				     ch: c_int) -> Lisp_Object
{
  let mut ignore: ptrdiff_t = 0;
  let mut c: c_int;
  let mut font_object: Lisp_Object = Qnil ();
  let f: *mut frame = XFRAME ((*w).frame);
  let mut pos: ptrdiff_t = pos;
  let mut pos_byte: ptrdiff_t = pos_byte;
  let mut face: *mut face = face;

  if face.is_null ()
    {
      let mut face_id: c_int = 0;

      if NILP (string)
	{ face_id = face_at_buffer_position (w, pos, &mut ignore, *limit,
					     false, -1, 0); }
      else
	{
	  face_id =
	    if NILP (Vface_remapping_alist)
	    { DEFAULT_FACE_ID }
	    else { lookup_basic_face (w, f, DEFAULT_FACE_ID) };

	  face_id = face_at_string_position (w, string, pos, 0, &mut ignore,
					     face_id, false, 0);
	}
      face = FACE_FROM_ID (f, face_id);
    }

  /* If the composition was triggered by an emoji, use a character
     from 'script-representative-chars', rather than the first
     character in the string, to determine the font to use.  */
  if codepoint_is_emoji_eligible (ch)
    {
      let mut val: Lisp_Object = assq_no_quit (Qemoji (), Vscript_representative_chars);
      if CONSP (val)
	{
	  val = XCDR (val);
	  if CONSP (val)
	    { val = XCAR (val); }
	  else if VECTORP (val)
	    { val = AREF (val, 0); }
	  font_object = font_for_char (face, XFIXNAT (val) as c_int, pos, string);
	}
    }

  while pos < *limit
    {
      c = if NILP (string)
	  { fetch_char_advance_no_check (&mut pos, &mut pos_byte) }
	  else { fetch_string_char_advance_no_check (string, &mut pos, &mut pos_byte) };
      let category: Lisp_Object = CHAR_TABLE_REF (Vunicode_category_table, c as isize);
      if FIXNUMP (category)
	  && (XFIXNUM (category) == unicode_category_t::UNICODE_CATEGORY_Cf as EMACS_INT
	      || CHAR_VARIATION_SELECTOR_P (c) != 0)
	{
	  continue;
	}
      if NILP (font_object)
	{
	  font_object = font_for_char (face, c, pos - 1, string);
	  if NILP (font_object)
	    { return Qnil (); }
	  continue;
	}
      if font_encode_char (font_object, c) == FONT_INVALID_CODE
	{ *limit = pos - 1; }
    }
  return font_object;
}


/* Lisp API.  */

DEFUN! ("fontp", Ffontp, Sfontp, 1, 2, 0,
       doc: "Return t if OBJECT is a font-spec, font-entity, or font-object.\nReturn nil otherwise.\nOptional 2nd argument EXTRA-TYPE, if non-nil, specifies to check\nwhich kind of font it is.  It must be one of `font-spec', `font-entity',\n`font-object'.",
       (object: Lisp_Object, extra_type: Lisp_Object),
       {
  if NILP (extra_type)
    { return if FONTP (object) { Qt () } else { Qnil () }; }
  if EQ (extra_type, Qfont_spec ())
    { return if FONT_SPEC_P (object) { Qt () } else { Qnil () }; }
  if EQ (extra_type, Qfont_entity ())
    { return if FONT_ENTITY_P (object) { Qt () } else { Qnil () }; }
  if EQ (extra_type, Qfont_object ())
    { return if FONT_OBJECT_P (object) { Qt () } else { Qnil () }; }
  wrong_type_argument (Qfont_extra_type (), extra_type);
});

DEFUN! ("font-spec", Ffont_spec, Sfont_spec, 0, MANY, 0,
       doc: "Return a newly created font-spec with arguments as properties.\n\nARGS must come in pairs KEY VALUE of font properties.  KEY must be a\nvalid font property name listed below:\n\n`:family', `:weight', `:slant', `:width'\n\nThey are the same as face attributes of the same name (see\n`set-face-attribute'), except that `:family' could also be a symbol,\nand the value of the other three properties can also be a number,\nthe numerical value of the style.\n\n`:foundry'\n\nVALUE must be a string or a symbol specifying the font foundry, e.g. `misc'.\n\n`:adstyle'\n\nVALUE must be a string or a symbol specifying the additional\ntypographic style information of a font, e.g. `sans'.\n\n`:registry'\n\nVALUE must be a string or a symbol specifying the charset registry and\nencoding of a font, e.g. `iso8859-1'.\n\n`:size'\n\nVALUE must be a non-negative integer or a floating point number\nspecifying the font size.  It specifies the font size in pixels (if\nVALUE is an integer), or in points (if VALUE is a float).\n\n`:dpi'\n\nVALUE must be a non-negative number that specifies the resolution\n(dot per inch) for which the font is designed.\n\n`:spacing'\n\nVALUE specifies the spacing of the font: mono, proportional, charcell,\nor dual.  It can be either a number (0 for proportional, 90 for dual,\n100 for mono, 110 for charcell) or a 1-letter symbol: `P', `D', `M',\nor `C' (lower-case variants are also accepted).\n\n`:avgwidth'\n\nVALUE must be a non-negative integer specifying the average width of\nthe font in 1/10 pixel units.\n\n`:name'\n\nVALUE must be a string of XLFD-style or fontconfig-style font name.\n\n`:script'\n\nVALUE must be a symbol representing a script that the font must\nsupport.  It may be a symbol representing a subgroup of a script\nlisted in the variable `script-representative-chars'.\n\n`:lang'\n\nVALUE must be a symbol whose name is a two-letter ISO-639 language\nname, e.g. `ja'.  The value is matched against the \"Additional Style\"\nfield of the XLFD spec of a font, if it's non-empty, on X, and\nagainst the codepages supported by the font on w32.\n\n`:otf'\n\nVALUE must be a list (SCRIPT-TAG LANGSYS-TAG GSUB [ GPOS ]) to specify\nrequired OpenType features.\n\n  SCRIPT-TAG: OpenType script tag symbol (e.g. `deva').\n  LANGSYS-TAG: OpenType language system tag symbol,\n     or nil for the default language system.\n  GSUB: List of OpenType GSUB feature tag symbols, or nil if none required.\n  GPOS: List of OpenType GPOS feature tag symbols, or nil if none required.\n\nGSUB and GPOS may contain nil elements.  In such a case, the font\nmust not have any of the remaining elements.\n\nFor instance, if the VALUE is `(thai nil nil (mark))', the font must\nbe an OpenType font whose GPOS table of `thai' script's default\nlanguage system must contain `mark' feature.\n\nusage: (font-spec ARGS...)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let spec: Lisp_Object = font_make_spec ();
  let mut i: ptrdiff_t;

  i = 0;
  while i < nargs
    {
      let key: Lisp_Object = *args.offset (i as isize);
      let mut val: Lisp_Object;

      CHECK_SYMBOL (key);
      if i + 1 >= nargs
	{ error (c"No value for key `%s'".as_ptr (), SDATA (SYMBOL_NAME (key))); }
      val = *args.offset ((i + 1) as isize);

      if EQ (key, QCname ())
	{
	  CHECK_STRING (val);
	  if font_parse_name (SSDATA (val), SBYTES (val), spec) < 0
	    { error (c"Invalid font name: %s".as_ptr (), SSDATA (val)); }
	  font_put_extra (spec, key, val);
	}
      else
	{
	  let idx: c_int = get_font_prop_index (key);

	  if idx >= 0
	    {
	      val = font_prop_validate (idx, Qnil (), val);
	      if idx < FONT_EXTRA_INDEX
		{ ASET (spec, idx as isize, val); }
	      else
		{ font_put_extra (spec, key, val); }
	    }
	  else
	    { font_put_extra (spec, key, font_prop_validate (0, key, val)); }
	}
      i += 2;
    }
  return spec;
});

/* Return a copy of FONT as a font-spec.  For the sake of speed, this code
   relies on an internal stuff exposed from alloc.c and should be handled
   with care. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn copy_font_spec (font: Lisp_Object) -> Lisp_Object
{
  const font_spec_size: isize = VECSIZE (std::mem::size_of::<font_spec> ());
  let mut new_spec: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut pcdr: *mut Lisp_Object;
  let spec: *mut font_spec;

  CHECK_FONT (font);

  /* Make an uninitialized font-spec object.  */
  spec = allocate_vector (font_spec_size) as *mut font_spec;
  XSETPVECTYPESIZE! (spec, pvec_type::PVEC_FONT, FONT_SPEC_MAX as isize,
		     font_spec_size - FONT_SPEC_MAX as isize);

  (*spec).props[FONT_TYPE_INDEX as usize] = Qnil ();
  (*spec).props[FONT_EXTRA_INDEX as usize] = Qnil ();

  /* Copy basic properties FONT_FOUNDRY_INDEX..FONT_AVGWIDTH_INDEX.  */
  /* C: `memcpy (spec->props + 1, XVECTOR (font)->contents + 1, (FONT_EXTRA_INDEX - 1) *
     word_size)`.  The port's counterpart is `copy_nonoverlapping`'s (src, dst, n) order
     (PORTING.md 2.1's memcpy family).  */
  std::ptr::copy_nonoverlapping ((*(XVECTOR (font))).contents.as_ptr ().offset (1) as *const u8,
				 (*spec).props.as_mut_ptr ().offset (1) as *mut u8,
				 (FONT_EXTRA_INDEX - 1) as usize * word_size as usize);

  /* Copy an alist of extra information but discard :font-entity property.  */
  pcdr = (*spec).props.as_mut_ptr ().offset (FONT_EXTRA_INDEX as isize);
  tail = AREF (font, FONT_EXTRA_INDEX as isize);
  while CONSP (tail)
    {
      if !EQ (XCAR (XCAR (tail)), QCfont_entity ())
	{
	  *pcdr = Fcons (Fcons (XCAR (XCAR (tail)), CDR (XCAR (tail))), Qnil ());
	  pcdr = xcdr_addr (*pcdr);
	}
      tail = XCDR (tail);
    }

  XSETFONT! (new_spec, spec);
  return new_spec;
}

/* Merge font-specs FROM and TO, and return a new font-spec.
   Every specified property in FROM overrides the corresponding
   property in TO.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn merge_font_spec (from: Lisp_Object, to: Lisp_Object) -> Lisp_Object
{
  let mut extra: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut i: c_int;
  let mut to: Lisp_Object = to;

  CHECK_FONT (from);
  CHECK_FONT (to);
  to = copy_font_spec (to);
  i = 0;
  while i < FONT_EXTRA_INDEX
    {
      ASET (to, i as isize, AREF (from, i as isize));
      i += 1;
    }
  extra = AREF (to, FONT_EXTRA_INDEX as isize);
  tail = AREF (from, FONT_EXTRA_INDEX as isize);
  while CONSP (tail)
    {
      if ! EQ (XCAR (XCAR (tail)), Qfont_entity ())
	{
	  let slot: Lisp_Object = assq_no_quit (XCAR (XCAR (tail)), extra);

	  if ! NILP (slot)
	    { XSETCDR (slot, XCDR (XCAR (tail))); }
	  else
	    { extra = Fcons (Fcons (XCAR (XCAR (tail)), XCDR (XCAR (tail))), extra); }
	}
      tail = XCDR (tail);
    }
  ASET (to, FONT_EXTRA_INDEX as isize, extra);
  return to;
}

DEFUN! ("font-get", Ffont_get, Sfont_get, 2, 2, 0,
       doc: "Return the value of FONT's property KEY.\nFONT is a font-spec, a font-entity, or a font-object.\nKEY can be any symbol, but these are reserved for specific meanings:\n  :foundry, :family, :adstyle, :registry, :weight, :slant, :width,\n  :size, :dpi, :spacing, :avgwidth, :script, :lang, :otf\nSee the documentation of `font-spec' for their meanings.\n\nIf FONT is a font-entity or a font-object, then values of\n:script and :otf properties are different from those of a font-spec\nas below:\n\n  The value of :script may be a list of scripts that are supported by\n  the font.\n\n  The value of :otf is a cons (GSUB . GPOS) where GSUB and GPOS are\n  lists representing the OpenType features supported by the font, of\n  this form: ((SCRIPT (LANGSYS FEATURE ...) ...) ...), where\n  SCRIPT, LANGSYS, and FEATURE are all symbols representing OpenType\n  Layout tags.  See `otf-script-alist' for the OpenType script tags.\n\nIn addition to the keys listed above, the following keys are reserved\nfor the specific meanings as below:\n\n  The value of :type is a symbol that identifies the font backend to be\n  used, such as `ftcrhb' or `xfthb' on X , `harfbuzz' or `uniscribe' on\n  MS-Windows, `ns' on Cocoa/GNUstep, etc.\n\n  The value of :combining-capability is non-nil if the font-backend of\n  FONT supports rendering of combining characters for non-OTF fonts.",
       (font: Lisp_Object, key: Lisp_Object),
       {
  let mut idx: c_int;
  let mut val: Lisp_Object;

  CHECK_FONT (font);
  CHECK_SYMBOL (key);

  idx = get_font_prop_index (key);
  if idx >= FONT_WEIGHT_INDEX && idx <= FONT_WIDTH_INDEX
    { return font_style_symbolic (font, idx, false); }
  if idx >= 0 && idx < FONT_EXTRA_INDEX
    { return AREF (font, idx as isize); }
  val = Fassq (key, AREF (font, FONT_EXTRA_INDEX as isize));
  if NILP (val) && FONT_OBJECT_P (font)
    {
      let fontp: *mut font = XFONT_OBJECT (font);

      if EQ (key, QCotf ())
	{
	  if (*((*fontp).driver as *const font_driver)).otf_capability.is_some ()
	    {
	      let otf_capability = (*((*fontp).driver as *const font_driver)).otf_capability;
	      if let Some (f) = otf_capability
		{ val = f (fontp); }
	    }
	  else
	    { val = Fcons (Qnil (), Qnil ()); }
	}
      else if EQ (key, QCcombining_capability ())
	{
	  if (*((*fontp).driver as *const font_driver)).combining_capability.is_some ()
	    {
	      let combining_capability = (*((*fontp).driver as *const font_driver)).combining_capability;
	      if let Some (f) = combining_capability
		{ val = f (fontp); }
	    }
	}
    }
  else
    { val = Fcdr (val); }
  return val;
});

/* C 4276: `#ifdef HAVE_WINDOW_SYSTEM` (undef in the frozen configuration), carried; the
   definition below is C 4278-4355's.  */

#[cfg(feature = "have_window_system")]
DEFUN! ("font-face-attributes", Ffont_face_attributes, Sfont_face_attributes, 1, 2, 0,
       doc: "Return a plist of face attributes generated by FONT.\nFONT is a font name, a font-spec, a font-entity, or a font-object.\nThe return value is a list of the form\n\n(:family FAMILY :height HEIGHT :weight WEIGHT :slant SLANT :width WIDTH)\n\nwhere FAMILY, HEIGHT, WEIGHT, SLANT, and WIDTH are face attribute values\ncompatible with `set-face-attribute'.  Some of these key-attribute pairs\nmay be omitted from the list if they are not specified by FONT.\n\nThe optional argument FRAME specifies the frame that the face attributes\nare to be displayed on.  If omitted, the selected frame is used.",
       (font: Lisp_Object, frame: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let mut plist: [Lisp_Object; 10] = [Qnil (); 10];
  let mut val: Lisp_Object;
  let mut n: c_int = 0;
  let mut font: Lisp_Object = font;

  if STRINGP (font)
    {
      let fontset: c_int = fs_query_fontset (font, 0);
      let name: Lisp_Object = font;
      if fontset >= 0
	{ font = fontset_ascii (fontset); }
      font = font_spec_from_name (name);
      if ! FONTP (font)
	{ signal_error (c"Invalid font name".as_ptr (), name); }
    }
  else if ! FONTP (font)
    { signal_error (c"Invalid font object".as_ptr (), font); }

  val = AREF (font, FONT_FAMILY_INDEX as isize);
  if ! NILP (val)
    {
      plist[n as usize] = QCfamily ();
      n += 1;
      plist[n as usize] = SYMBOL_NAME (val);
      n += 1;
    }

  val = AREF (font, FONT_SIZE_INDEX as isize);
  if FIXNUMP (val)
    {
      let font_dpi: Lisp_Object = AREF (font, FONT_DPI_INDEX as isize);
      let dpi: c_int = if FIXNUMP (font_dpi) { XFIXNUM (font_dpi) as c_int }
		       else { FRAME_RES (f) as c_int };
      plist[n as usize] = QCheight ();
      n += 1;
      plist[n as usize] = make_fixnum (PIXEL_TO_POINT! ((XFIXNUM (val) * 10) as c_double, dpi as c_double)
				       as EMACS_INT);
      n += 1;
    }
  else if FLOATP (val)
    {
      plist[n as usize] = QCheight ();
      n += 1;
      plist[n as usize] = make_fixnum ((10 * XFLOAT_DATA (val) as c_int) as EMACS_INT);
      n += 1;
    }

  val = FONT_WEIGHT_FOR_FACE! (font);
  if ! NILP (val)
    {
      plist[n as usize] = QCweight ();
      n += 1;
      plist[n as usize] = val;
      n += 1;
    }

  val = FONT_SLANT_FOR_FACE! (font);
  if ! NILP (val)
    {
      plist[n as usize] = QCslant ();
      n += 1;
      plist[n as usize] = val;
      n += 1;
    }

  val = FONT_WIDTH_FOR_FACE! (font);
  if ! NILP (val)
    {
      plist[n as usize] = QCwidth ();
      n += 1;
      plist[n as usize] = val;
      n += 1;
    }

  return Flist (n as ptrdiff_t, plist.as_mut_ptr ());
});

DEFUN! ("font-put", Ffont_put, Sfont_put, 3, 3, 0,
       doc: "Set one property of FONT: give property KEY value VAL.\nFONT is a font-spec, a font-entity, or a font-object.\n\nIf FONT is a font-spec, KEY can be any symbol.  But if KEY is the one\naccepted by the function `font-spec' (which see), VAL must be what\nallowed in `font-spec'.\n\nIf FONT is a font-entity or a font-object, KEY must not be the one\naccepted by `font-spec'.\n\nSee also `font-get' for KEYs that have special meanings.",
       (font: Lisp_Object, prop: Lisp_Object, val: Lisp_Object),
       {
  let idx: c_int;

  idx = get_font_prop_index (prop);
  if idx >= 0 && idx < FONT_EXTRA_INDEX
    {
      CHECK_FONT_SPEC (font);
      ASET (font, idx as isize, font_prop_validate (idx, Qnil (), val));
    }
  else
    {
      if EQ (prop, QCname ())
	  || EQ (prop, QCscript ())
	  || EQ (prop, QClang ())
	  || EQ (prop, QCotf ())
	{ CHECK_FONT_SPEC (font); }
      else
	{ CHECK_FONT (font); }
      font_put_extra (font, prop, font_prop_validate (0, prop, val));
    }
  return val;
});

DEFUN! ("list-fonts", Flist_fonts, Slist_fonts, 1, 4, 0,
       doc: "List available fonts matching FONT-SPEC on FRAME.\nIf FRAME is nil or omitted, it defaults to the selected frame,\nOptional 3rd argument NUM, if non-nil, limits the number of returned fonts.\nOptional 4th argument PREFER, if non-nil, is a font-spec to\ncontrol the order of the returned list.  Fonts are sorted by\nhow close they are to PREFER.\n\nThe return value is a list of font-entity objects describing available\nfonts which match FONT-SPEC.",
       (font_spec: Lisp_Object, frame: Lisp_Object, num: Lisp_Object, prefer: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let mut vec: Lisp_Object;
  let mut list: Lisp_Object;
  let mut n: EMACS_INT = 0;

  CHECK_FONT_SPEC (font_spec);
  if ! NILP (num)
    {
      CHECK_FIXNUM (num);
      n = XFIXNUM (num);
      if n <= 0
	{ return Qnil (); }
    }
  if ! NILP (prefer)
    { CHECK_FONT_SPEC (prefer); }

  list = font_list_entities (f, font_spec);
  if NILP (list)
    { return Qnil (); }
  if NILP (XCDR (list))
      && ASIZE (XCAR (list)) == 1
    { return list1 (AREF (XCAR (list), 0)); }

  if ! NILP (prefer)
    { vec = font_sort_entities (list, prefer, f, 0); }
  else
    { vec = font_vconcat_entity_vectors (list); }
  if n == 0 || n >= ASIZE (vec) as EMACS_INT
    { list = CALLN! (Fappend, vec, Qnil ()); }
  else
    {
      list = Qnil ();
      n -= 1;
      while n >= 0
	{
	  list = Fcons (AREF (vec, n as isize), list);
	  n -= 1;
	}
    }
  return list;
});

DEFUN! ("font-family-list", Ffont_family_list, Sfont_family_list, 0, 1, 0,
       doc: "List available font families on the current frame.\nIf FRAME is omitted or nil, the selected frame is used.",
       (frame: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let mut driver_list: *mut font_driver_list;
  let mut list: Lisp_Object = Qnil ();

  driver_list = (*f).font_driver_list as *mut font_driver_list;
  while !driver_list.is_null ()
    {
      if (*((*driver_list).driver)).list_family.is_some ()
	{
	  let val: Lisp_Object
	    = ((*((*driver_list).driver)).list_family.unwrap ()) (f as *mut c_void);
	  let tail: Lisp_Object = list;

	  let mut val: Lisp_Object = val;
	  while CONSP (val)
	    {
	      if NILP (Fmemq (XCAR (val), tail))
		  && SYMBOLP (XCAR (val))
		{ list = Fcons (SYMBOL_NAME (XCAR (val)), list); }
	      val = XCDR (val);
	    }
	}
      driver_list = (*driver_list).next;
    }
  return list;
});

DEFUN! ("find-font", Ffind_font, Sfind_font, 1, 2, 0,
       doc: "Return a font-entity matching with FONT-SPEC on the current frame.\nOptional 2nd argument FRAME, if non-nil, specifies the target frame.",
       (font_spec: Lisp_Object, frame: Lisp_Object),
       {
  let mut val: Lisp_Object = Flist_fonts (font_spec, frame, make_fixnum (1), Qnil ());

  if CONSP (val)
    { val = XCAR (val); }
  return val;
});

DEFUN! ("font-xlfd-name", Ffont_xlfd_name, Sfont_xlfd_name, 1, 3, 0,
       doc: "Return XLFD name of FONT.\nFONT is a font-spec, font-entity, or font-object.\n\nIf the name is too long to be represented as an XLFD (maximum 255\nchars) and LONG_XLFDS is nil, return nil.\n\nIf the 2nd optional arg FOLD-WILDCARDS is non-nil,\nthe consecutive wildcards are folded into one.",
       (font: Lisp_Object, fold_wildcards: Lisp_Object, long_xlfds: Lisp_Object),
       {
  let mut name_buffer: [c_char; 256] = [0; 256];
  let mut name: *mut c_char;
  let mut namelen: c_int;
  let mut pixel_size: c_int = 0;
  let mut string: Lisp_Object;

  CHECK_FONT (font);

  'block: {
  if FONT_OBJECT_P (font)
    {
      let font_name: Lisp_Object = AREF (font, FONT_NAME_INDEX as isize);

      if STRINGP (font_name)
	  && *SDATA (font_name) == b'-'
	{
	  if NILP (fold_wildcards)
	    { return font_name; }
	  name = name_buffer.as_mut_ptr ();
	  lispstpcpy (name, font_name);
	  namelen = SBYTES (font_name) as c_int;
	  break 'block;
	}
      pixel_size = (*XFONT_OBJECT (font)).pixel_size;
    }

  if NILP (long_xlfds)
    {
      name = name_buffer.as_mut_ptr ();
      namelen = font_unparse_xlfd (font, pixel_size, name, 256) as c_int;
      if namelen < 0
	{ return Qnil (); }
    }
  else
    {
      /* Dynamically allocate the XLFD.  */
      name = font_dynamic_unparse_xlfd (font, pixel_size);

      if name.is_null ()
	{ return Qnil (); }

      namelen = strlen (name) as c_int;
    }
  }

  /* done: C's forward `goto` lands here (PORTING.md 2.2).  */
  if ! NILP (fold_wildcards)
    {
      let mut p0: *mut c_char = name;
      let mut p1: *mut c_char;

      p1 = strstr (p0, c"-*-*".as_ptr ());
      while !p1.is_null ()
	{
	  memmove (p1 as *mut c_void, p1.offset (2) as *const c_void,
		   (name.offset ((namelen + 1) as isize)).offset_from (p1.offset (2))
		     as usize);
	  namelen -= 2;
	  p0 = p1;
	  p1 = strstr (p0, c"-*-*".as_ptr ());
	}
    }

  /* If NAME is dynamically allocated, free it.  */

  string = make_string (name, namelen as ptrdiff_t);

  if name != name_buffer.as_mut_ptr ()
    { xfree (name as *mut c_void); }

  return string;
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_font_cache (f: *mut frame)
{
  let mut driver_list: *mut font_driver_list = (*f).font_driver_list as *mut font_driver_list;

  while !driver_list.is_null ()
    {
      if (*driver_list).on
	{
	  let mut val: Lisp_Object;
	  let tmp: Lisp_Object;
	  let cache: Lisp_Object = ((*((*driver_list).driver)).get_cache.unwrap ()) (f as *mut c_void);

	  val = XCDR (cache);
	  loop
	    {
	      eassert! (CONSP (val));
	      if EQ (XCAR (XCAR (val)), (*((*driver_list).driver)).r#type)
		{ break; }
	      val = XCDR (val);
	    }
	  tmp = XCDR (XCAR (val));
	  if XFIXNUM (XCAR (tmp)) == 0
	    {
	      font_clear_cache (f, XCAR (val), (*driver_list).driver);
	      XSETCDR (cache, XCDR (val));
	    }
	}
      driver_list = (*driver_list).next;
    }
}

DEFUN! ("clear-font-cache", Fclear_font_cache, Sclear_font_cache, 0, 0, 0,
       doc: "Clear font cache of each frame.",
       (),
       {
  let mut list: Lisp_Object;
  let mut frame: Lisp_Object;

  FOR_EACH_FRAME! (list, frame, {
    clear_font_cache (XFRAME (frame));
  });

  return Qnil ();
});


#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_fill_lglyph_metrics (glyph: Lisp_Object, font: *mut font,
						   code: c_uint)
{
  let mut metrics: font_metrics = font_metrics { lbearing: 0, rbearing: 0, width: 0,
						 ascent: 0, descent: 0 };

  LGLYPH_SET_CODE! (glyph, code);
  ((*((*font).driver as *const font_driver)).text_extents.unwrap ()) (font, &code, 1, &mut metrics);
  LGLYPH_SET_LBEARING (glyph, metrics.lbearing as EMACS_INT);
  LGLYPH_SET_RBEARING (glyph, metrics.rbearing as EMACS_INT);
  LGLYPH_SET_WIDTH (glyph, metrics.width as EMACS_INT);
  LGLYPH_SET_ASCENT (glyph, metrics.ascent as EMACS_INT);
  LGLYPH_SET_DESCENT (glyph, metrics.descent as EMACS_INT);
}


DEFUN! ("font-shape-gstring", Ffont_shape_gstring, Sfont_shape_gstring, 2, 2, 0,
       doc: "Shape the glyph-string GSTRING subject to bidi DIRECTION.\nShaping means substituting glyphs and/or adjusting positions of glyphs\nto get the correct visual image of character sequences set in the\nheader of the glyph-string.\n\nDIRECTION should be produced by the UBA, the Unicode Bidirectional\nAlgorithm, and should be a symbol, either L2R or R2L.  It can also\nbe nil if the bidi context is unknown.\n\nIf the shaping was successful, the value is GSTRING itself or a newly\ncreated glyph-string.  Otherwise, the value is nil.\n\nSee the documentation of `composition-get-gstring' for the format of\nGSTRING.",
       (gstring: Lisp_Object, direction: Lisp_Object),
       {
  let font: *mut font;
  let mut font_object: Lisp_Object;
  /* PORTING.md 1.5: C declares `n` and `glyph` uninitialised; the `shape` call is the
     loop's first statement, and the loop always runs at least once (i < 3).  */
  let mut n: Lisp_Object = Qnil ();
  let mut glyph: Lisp_Object = Qnil ();
  let mut i: ptrdiff_t;
  let mut from: ptrdiff_t;
  let mut to: ptrdiff_t;
  let mut gstring: Lisp_Object = gstring;

  if ! composition_gstring_p (gstring)
    { signal_error (c"Invalid glyph-string: ".as_ptr (), gstring); }
  if ! NILP (LGSTRING_ID (gstring))
    { return gstring; }
  let cached_gstring: Lisp_Object
    = composition_gstring_lookup_cache (LGSTRING_HEADER (gstring));
  if ! NILP (cached_gstring)
    { return cached_gstring; }
  font_object = LGSTRING_FONT (gstring);
  CHECK_FONT_OBJECT (font_object);
  font = XFONT_OBJECT (font_object);
  if (*((*font).driver as *const font_driver)).shape.is_none ()
    { return Qnil (); }

  /* Try at most three times with larger gstring each time.  */
  i = 0;
  while i < 3
    {
      n = ((*((*font).driver as *const font_driver)).shape.unwrap ()) (gstring, direction);
      if FIXNUMP (n)
	{ break; }
      gstring = larger_vector (gstring,
			       LGSTRING_GLYPH_LEN (gstring), -1);
      i += 1;
    }
  if i == 3 || XFIXNUM (n) == 0
    { return Qnil (); }
  if XFIXNUM (n) < LGSTRING_GLYPH_LEN (gstring)
    { LGSTRING_SET_GLYPH (gstring, XFIXNUM (n), Qnil ()); }

  /* Check FROM_IDX and TO_IDX of each GLYPH in GSTRING to assure that
     GLYPHS covers all characters (except for the last few ones) in
     GSTRING.  More formally, provided that NCHARS is the number of
     characters in GSTRING and GLYPHS[i] is the ith glyph, FROM_IDX
     and TO_IDX of each glyph must satisfy these conditions:

       GLYPHS[0].FROM_IDX == 0
       GLYPHS[i].FROM_IDX <= GLYPHS[i].TO_IDX
       if (GLYPHS[i].FROM_IDX == GLYPHS[i-1].FROM_IDX)
         ;; GLYPHS[i] and GLYPHS[i-1] belongs to the same grapheme cluster
         GLYPHS[i].TO_IDX == GLYPHS[i-1].TO_IDX
       else
         ;; Be sure to cover all characters.
         GLYPHS[i].FROM_IDX == GLYPHS[i-1].TO_IDX + 1 */
  'block: {
  glyph = LGSTRING_GLYPH (gstring, 0);
  from = LGLYPH_FROM (glyph);
  to = LGLYPH_TO (glyph);
  if from != 0 || to < from
    { break 'block; }
  i = 1;
  while i < LGSTRING_GLYPH_LEN (gstring)
    {
      glyph = LGSTRING_GLYPH (gstring, i);
      if NILP (glyph)
	{ break; }
      if ! (LGLYPH_FROM (glyph) <= LGLYPH_TO (glyph)
	    && (if LGLYPH_FROM (glyph) == from
		{ LGLYPH_TO (glyph) == to }
		else { LGLYPH_FROM (glyph) == to + 1 }))
	{ break 'block; }
      from = LGLYPH_FROM (glyph);
      to = LGLYPH_TO (glyph);
      i += 1;
    }
  composition_gstring_adjust_zero_width (gstring);
  return composition_gstring_put_cache (gstring, XFIXNUM (n));
  }

  /* shaper_error: C's forward `goto` lands here (PORTING.md 2.2).  */
  return Qnil ();
});

DEFUN! ("font-variation-glyphs", Ffont_variation_glyphs, Sfont_variation_glyphs,
       2, 2, 0,
       doc: "Return a list of variation glyphs for CHARACTER in FONT-OBJECT.\nEach element of the value is a cons (VARIATION-SELECTOR . GLYPH-ID),\nwhere\n  VARIATION-SELECTOR is a character code of variation selector\n    (#xFE00..#xFE0F or #xE0100..#xE01EF).\n  GLYPH-ID is a glyph code of the corresponding variation glyph, an integer.",
       (font_object: Lisp_Object, character: Lisp_Object),
       {
  let mut variations: [c_uint; 256] = [0; 256];
  let font: *mut font;
  let mut i: c_int;
  let mut n: c_int;
  let mut val: Lisp_Object;

  CHECK_FONT_OBJECT (font_object);
  CHECK_CHARACTER (character);
  font = XFONT_OBJECT (font_object);
  if (*((*font).driver as *const font_driver)).get_variation_glyphs.is_none ()
    { return Qnil (); }
  n = ((*((*font).driver as *const font_driver)).get_variation_glyphs.unwrap ())
        (font, XFIXNUM (character) as c_int, variations.as_mut_ptr ());
  if n == 0
    { return Qnil (); }
  val = Qnil ();
  i = 0;
  while i < 255
    {
      if variations[i as usize] != 0
	{
	  let vs: c_int = if i < 16 { 0xFE00 + i } else { 0xE0100 + (i - 16) };
	  let code: Lisp_Object = INT_TO_INTEGER (variations[i as usize] as EMACS_INT);
	  val = Fcons (Fcons (make_fixnum (vs as EMACS_INT), code), val);
	}
      i += 1;
    }
  return val;
});

/* Return a description of the font at POSITION in the current buffer.
   If the 2nd optional arg CH is non-nil, it is a character to check
   the font instead of the character at POSITION.

   For a graphical display, return a cons (FONT-OBJECT . GLYPH-CODE).
   FONT-OBJECT is the font for the character at POSITION in the current
   buffer.  This is computed from all the text properties and overlays
   that apply to POSITION.  POSITION may be nil, in which case,
   FONT-SPEC is the font for displaying the character CH with the
   default face.  GLYPH-CODE is the glyph code in the font to use for
   the character, it is a fixnum, if it is small enough, otherwise a
   bignum.

   For a text terminal, return a nonnegative integer glyph code for
   the character, or a negative integer if the character is not
   displayable.  Terminal glyph codes are system-dependent integers
   that represent displayable characters: for example, on a Linux x86
   console they represent VGA code points.

   It returns nil in the following cases:

   (1) The window system doesn't have a font for the character (thus
   it is displayed by an empty box).

   (2) The character code is invalid.

   (3) If POSITION is not nil, and the current buffer is not displayed
   in any window.

   (4) For a text terminal, the terminal does not report glyph codes.

   In addition, the returned font name may not take into account of
   such redisplay engine hooks as what used in jit-lock-mode if
   POSITION is currently not visible.  */


DEFUN! ("internal-char-font", Finternal_char_font, Sinternal_char_font, 1, 2, 0,
       doc: "For internal use only.",
       (position: Lisp_Object, ch: Lisp_Object),
       {
  let mut pos: ptrdiff_t;
  let mut pos_byte: ptrdiff_t = 0;
  let mut dummy: ptrdiff_t = 0;
  let mut face_id: c_int;
  let mut c: c_int;
  let mut f: *mut frame;

  if NILP (position)
    {
      CHECK_CHARACTER (ch);
      c = XFIXNUM (ch) as c_int;
      f = XFRAME (selected_frame);
      face_id = lookup_basic_face (std::ptr::null_mut (), f, DEFAULT_FACE_ID);
      pos = -1;
    }
  else
    {
      let window: Lisp_Object;
      let w: *mut window;

      let fixed_pos: EMACS_INT = fix_position (position);
      if ! (BEGV! () <= fixed_pos && fixed_pos < ZV! ())
	{ args_out_of_range_3 (position, make_fixnum (BEGV! ()), make_fixnum (ZV! ())); }
      pos = fixed_pos;
      pos_byte = CHAR_TO_BYTE (pos);
      if NILP (ch)
	{ c = FETCH_CHAR (pos_byte); }
      else
	{
	  CHECK_FIXNAT (ch);
	  c = XFIXNUM (ch) as c_int;
	}
      window = Fget_buffer_window (Fcurrent_buffer (), Qnil ());
      if NILP (window)
	{ return Qnil (); }
      w = XWINDOW (window);
      f = XFRAME ((*w).frame);
      face_id = face_at_buffer_position (w, pos, &mut dummy,
					 pos + 100, false, -1, 0);
    }
  if ! CHAR_VALID_P (c as intmax_t)
    { return Qnil (); }

  if ! FRAME_WINDOW_P (f)
    { return terminal_glyph_code (FRAME_TERMINAL! (f), c); }

  /* We need the basic faces to be valid below, so recompute them if
     some code just happened to clear the face cache.  */
  if (*FRAME_FACE_CACHE! (f)).used == 0
    { recompute_basic_faces (f); }

  face_id = FACE_FOR_CHAR (f, FACE_FROM_ID (f, face_id), c, pos, Qnil ());
  let face: *mut face = FACE_FROM_ID (f, face_id);
  if (*face).font.is_null ()
    { return Qnil (); }
  let code: c_uint = (*((*(*face).font).driver as *const font_driver)).encode_char.unwrap () ((*face).font, c);
  if code == FONT_INVALID_CODE
    { return Qnil (); }
  let mut font_object: Lisp_Object;
  XSETFONT! (font_object, (*face).font);
  return Fcons (font_object, INT_TO_INTEGER (code as EMACS_INT));
});

/* C 5238: `#ifdef FONT_DEBUG` —**live text on this target**, because `font.h` 997-998
   defines `FONT_DEBUG` itself (measured with clang; see the file header).  The seven subrs
   and `Ffont_at` below are compiled by the reference build, so the port keeps them as
   ordinary text.  `Fdraw_string` (C 5534-5589) is inside that block's own `#if 0` and is
   recorded in `docs/ports/font.md` instead of transcribed.  */

DEFUN! ("open-font", Fopen_font, Sopen_font, 1, 3, 0,
       doc: "Open FONT-ENTITY.",
       (font_entity: Lisp_Object, size: Lisp_Object, frame: Lisp_Object),
       {
  let mut isize_: intmax_t = 0;
  let f: *mut frame = decode_live_frame (frame);

  CHECK_FONT_ENTITY (font_entity);

  if NILP (size)
    { isize_ = XFIXNUM (AREF (font_entity, FONT_SIZE_INDEX as isize)) as intmax_t; }
  else
    {
      CHECK_NUMBER (size);
      if FLOATP (size)
	{ isize_ = POINT_TO_PIXEL! (XFLOAT_DATA (size), FRAME_RES (f)) as intmax_t; }
      else if ! integer_to_intmax (size, &mut isize_)
	{ args_out_of_range (font_entity, size); }
      if ! (INT_MIN as intmax_t <= isize_ && isize_ <= INT_MAX as intmax_t)
	{ args_out_of_range (font_entity, size); }
      if isize_ == 0
	{ isize_ = 120; }
    }
  return font_open_entity (f, font_entity, isize_ as c_int);
});

DEFUN! ("close-font", Fclose_font, Sclose_font, 1, 2, 0,
       doc: "Close FONT-OBJECT.",
       (font_object: Lisp_Object, frame: Lisp_Object),
       {
  CHECK_FONT_OBJECT (font_object);
  font_close_object (decode_live_frame (frame), font_object);
  return Qnil ();
});

DEFUN! ("query-font", Fquery_font, Squery_font, 1, 1, 0,
       doc: "Return information about FONT-OBJECT.\nThe value is a vector:\n  [ NAME FILENAME PIXEL-SIZE SIZE ASCENT DESCENT SPACE-WIDTH AVERAGE-WIDTH\n    CAPABILITY ]\n\nNAME is the font name, a string (or nil if the font backend doesn't\nprovide a name).\n\nFILENAME is the font file name, a string (or nil if the font backend\ndoesn't provide a file name).\n\nPIXEL-SIZE is a pixel size by which the font is opened.\n\nSIZE is a maximum advance width of the font in pixels.\n\nASCENT, DESCENT, SPACE-WIDTH, AVERAGE-WIDTH are metrics of the font in\npixels.\n\nCAPABILITY is a list whose first element is a symbol representing the\nfont format (x, opentype, truetype, type1, pcf, or bdf) and the\nremaining elements describe the details of the font capability.\n\nIf the font is OpenType font, the form of the list is\n  (opentype GSUB GPOS)\nwhere GSUB shows which \"GSUB\" features the font supports, and GPOS\nshows which \"GPOS\" features the font supports.  Both GSUB and GPOS are\nlists of the format:\n  ((SCRIPT (LANGSYS FEATURE ...) ...) ...)\n\nIf the font is not OpenType font, currently the length of the form is\none.\n\nSCRIPT is a symbol representing OpenType script tag.\n\nLANGSYS is a symbol representing OpenType langsys tag, or nil\nrepresenting the default langsys.\n\nFEATURE is a symbol representing OpenType feature tag.\n\nIf the font is not OpenType font, CAPABILITY is nil.",
       (font_object: Lisp_Object),
       {
  let font: *mut font = CHECK_FONT_GET_OBJECT (font_object);
  return CALLN! (Fvector,
		 AREF (font_object, FONT_NAME_INDEX as isize),
		 AREF (font_object, FONT_FILE_INDEX as isize),
		 make_fixnum ((*font).pixel_size as EMACS_INT),
		 make_fixnum ((*font).max_width as EMACS_INT),
		 make_fixnum ((*font).ascent as EMACS_INT),
		 make_fixnum ((*font).descent as EMACS_INT),
		 make_fixnum ((*font).space_width as EMACS_INT),
		 make_fixnum ((*font).average_width as EMACS_INT),
		 (if (*((*font).driver as *const font_driver)).otf_capability.is_some ()
		  { Fcons (Qopentype (),
			   ((*((*font).driver as *const font_driver)).otf_capability.unwrap ()) (font)) }
		  else { Qnil () }));
});

DEFUN! ("font-has-char-p", Ffont_has_char_p, Sfont_has_char_p, 2, 3, 0,
       doc: "Return non-nil if FONT on FRAME has a glyph for character CH.\nFONT can be either a font-entity or a font-object.  If it is\na font-entity and the result is nil, it means the font needs to be\nopened (with `open-font') to check.\nFRAME defaults to the selected frame if it is nil or omitted.",
       (font: Lisp_Object, ch: Lisp_Object, frame: Lisp_Object),
       {
  let f: *mut frame;
  CHECK_FONT (font);
  CHECK_CHARACTER (ch);

  if NILP (frame)
    { f = XFRAME (selected_frame); }
  else
    {
      CHECK_FRAME (frame);
      f = XFRAME (frame);
    }

  if font_has_char (f, font, XFIXNAT (ch) as c_int) <= 0
    { return Qnil (); }
  else
    { return Qt (); }
});

DEFUN! ("font-get-glyphs", Ffont_get_glyphs, Sfont_get_glyphs, 3, 4, 0,
       doc: "Return a vector of FONT-OBJECT's glyphs for the specified characters.\nFROM and TO are positions (integers or markers) specifying a region\nof the current buffer, and can be in either order.  If the optional\nfourth arg OBJECT is not nil, it is a string or a vector containing\nthe target characters between indices FROM and TO, which are treated\nas in `substring'.\n\nEach element is a vector containing information of a glyph in this format:\n  [FROM-IDX TO-IDX C CODE WIDTH LBEARING RBEARING ASCENT DESCENT ADJUSTMENT]\nwhere\n  FROM is an index numbers of a character the glyph corresponds to.\n  TO is the same as FROM.\n  C is the character of the glyph.\n  CODE is the glyph-code of C in FONT-OBJECT.\n  WIDTH thru DESCENT are the metrics (in pixels) of the glyph.\n  ADJUSTMENT is always nil.\n\nIf FONT-OBJECT doesn't have a glyph for a character, the corresponding\nelement is nil.\n\nAlso see `font-has-char-p', which is more efficient than this function\nif you just want to check whether FONT-OBJECT has a glyph for a\ncharacter.",
       (font_object: Lisp_Object, from: Lisp_Object, to: Lisp_Object,
        object: Lisp_Object),
       {
  let font: *mut font = CHECK_FONT_GET_OBJECT (font_object);
  let mut len: ptrdiff_t;
  let mut chars: *mut Lisp_Object;
  let mut from: Lisp_Object = from;
  let mut to: Lisp_Object = to;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  chars = std::ptr::null_mut ();

  if NILP (object)
    {
      let mut charpos: ptrdiff_t;
      let mut bytepos: ptrdiff_t;

      validate_region (&mut from, &mut to);
      if EQ (from, to)
	{ return Qnil (); }
      len = XFIXNAT (to) - XFIXNAT (from);
      SAFE_ALLOCA_LISP! (sa_avail, chars, len);
      charpos = XFIXNAT (from);
      bytepos = CHAR_TO_BYTE (charpos);
      let mut i: ptrdiff_t = 0;
      while charpos < XFIXNAT (to)
	{
	  let c: c_int = fetch_char_advance (&mut charpos, &mut bytepos);
	  *chars.offset (i as isize) = make_fixnum (c as EMACS_INT);
	  i += 1;
	}
    }
  else if STRINGP (object)
    {
      let mut p: *const c_uchar;
      let mut ifrom: ptrdiff_t = 0;
      let mut ito: ptrdiff_t = 0;

      validate_subarray (object, from, to, SCHARS (object), &mut ifrom, &mut ito);
      if ifrom == ito
	{ return Qnil (); }
      len = ito - ifrom;
      SAFE_ALLOCA_LISP! (sa_avail, chars, len);
      p = SDATA (object);
      if STRING_MULTIBYTE (object)
	{
	  let mut c: c_int;

	  /* Skip IFROM characters from the beginning.  */
	  let mut i: ptrdiff_t = 0;
	  while i < ifrom
	    {
	      p = p.offset (BYTES_BY_CHAR_HEAD (*p as c_int) as isize);
	      i += 1;
	    }

	  /* Now fetch an interesting characters.  */
	  i = 0;
	  while i < len
	    {
	      c = string_char_advance (&mut p);
	      *chars.offset (i as isize) = make_fixnum (c as EMACS_INT);
	      i += 1;
	    }
	}
      else
	{
	  let mut i: ptrdiff_t = 0;
	  while i < len
	    {
	      *chars.offset (i as isize)
		= make_fixnum (*p.offset ((ifrom + i) as isize) as EMACS_INT);
	      i += 1;
	    }
	}
    }
  else if VECTORP (object)
    {
      let mut ifrom: ptrdiff_t = 0;
      let mut ito: ptrdiff_t = 0;

      validate_subarray (object, from, to, ASIZE (object), &mut ifrom, &mut ito);
      if ifrom == ito
	{ return Qnil (); }
      len = ito - ifrom;
      let mut i: ptrdiff_t = 0;
      while i < len
	{
	  let elt: Lisp_Object = AREF (object, ifrom + i);
	  CHECK_CHARACTER (elt);
	  i += 1;
	}
      chars = aref_addr (object, ifrom);
    }
  else
    { wrong_type_argument (Qarrayp (), object); }

  let vec: Lisp_Object = make_nil_vector (len);
  let mut i: ptrdiff_t = 0;
  while i < len
    {
      let mut g: Lisp_Object;
      let c: c_int = XFIXNAT (*chars.offset (i as isize)) as c_int;
      let mut code: c_uint;
      let mut metrics: font_metrics = font_metrics { lbearing: 0, rbearing: 0, width: 0,
						     ascent: 0, descent: 0 };

      code = (*((*font).driver as *const font_driver)).encode_char.unwrap () (font, c);
      if code == FONT_INVALID_CODE
	{
	  ASET (vec, i, Qnil ());
	  i += 1;
	  continue;
	}
      g = LGLYPH_NEW ();
      LGLYPH_SET_FROM (g, i);
      LGLYPH_SET_TO (g, i);
      LGLYPH_SET_CHAR (g, c as EMACS_INT);
      LGLYPH_SET_CODE! (g, code);
      ((*((*font).driver as *const font_driver)).text_extents.unwrap ()) (font, &code, 1, &mut metrics);
      LGLYPH_SET_WIDTH (g, metrics.width as EMACS_INT);
      LGLYPH_SET_LBEARING (g, metrics.lbearing as EMACS_INT);
      LGLYPH_SET_RBEARING (g, metrics.rbearing as EMACS_INT);
      LGLYPH_SET_ASCENT (g, metrics.ascent as EMACS_INT);
      LGLYPH_SET_DESCENT (g, metrics.descent as EMACS_INT);
      ASET (vec, i, g);
      i += 1;
    }
  if ! VECTORP (object)
    { SAFE_FREE! (sa_count, sa_mark); }
  return vec;
});

DEFUN! ("font-match-p", Ffont_match_p, Sfont_match_p, 2, 2, 0,
       doc: "Return t if and only if font-spec SPEC matches with FONT.\nFONT is a font-spec, font-entity, or font-object.",
       (spec: Lisp_Object, font: Lisp_Object),
       {
  CHECK_FONT_SPEC (spec);
  CHECK_FONT (font);

  return if font_match_p (spec, font) { Qt () } else { Qnil () };
});

DEFUN! ("font-at", Ffont_at, Sfont_at, 1, 3, 0,
       doc: "Return a font-object for displaying a character at POSITION.\nOptional second arg WINDOW, if non-nil, is a window displaying\nthe current buffer.  It defaults to the currently selected window.\nOptional third arg STRING, if non-nil, is a string containing the target\ncharacter at index specified by POSITION.",
       (position: Lisp_Object, window: Lisp_Object, string: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);
  let mut pos: EMACS_INT;

  if NILP (string)
    {
      if XBUFFER ((*w).contents) != (current_buffer! ())
	{ error (c"Specified window is not displaying the current buffer".as_ptr ()); }
      pos = fix_position (position);
      if ! (BEGV! () <= pos && pos < ZV! ())
	{ args_out_of_range_3 (position, make_fixnum (BEGV! ()), make_fixnum (ZV! ())); }
    }
  else
    {
      CHECK_FIXNUM (position);
      CHECK_STRING (string);
      pos = XFIXNUM (position);
      if ! (0 <= pos && pos < SCHARS (string))
	{ args_out_of_range (string, position); }
    }

  return font_at (-1, pos, std::ptr::null_mut (), w, string);
});

/* C 5534-5589: `#if 0` — `Fdraw_string` (and its `Sdraw_string` record).  Recorded in
   `docs/ports/font.md`, not transcribed (status 3.55's `#if 0` rule).  */

/* C 5590: `#endif	/* FONT_DEBUG */`.  The eight `defsubr` calls of C 5938-5949 are
   recorded in `docs/ports/font.md` too: they cannot be transcribed because
   `FONT_DEBUG` is a measured row of `source-header-macros.tsv` now (status 3.76), so its
   eight registrations are transcribed in `syms_of_font`.  */

DEFUN! ("frame-font-cache", Fframe_font_cache, Sframe_font_cache, 0, 1, 0,
       doc: "Return FRAME's font cache.  Mainly used for debugging.\nIf FRAME is omitted or nil, use the selected frame.",
       (frame: Lisp_Object),
       {
  /* C 5580-5588: `#ifdef HAVE_WINDOW_SYSTEM` (undef in the frozen configuration), carried.  */
  #[cfg(feature = "have_window_system")]
  {
    let f: *mut frame = decode_live_frame (frame);

    if FRAME_WINDOW_P (f)
      { return (*FRAME_DISPLAY_INFO! (f)).name_list_element; }
  }
  return Qnil ();
});

/* C 5592: `#ifdef HAVE_WINDOW_SYSTEM` (undef in the frozen configuration), carried; the
   definition below is C 5594-5699's.  Its own inner `#if 0` (C 5685-5689, the
   `font_close_object` call) is recorded in `docs/ports/font.md`.  */

#[cfg(feature = "have_window_system")]
DEFUN! ("font-info", Ffont_info, Sfont_info, 1, 2, 0,
       doc: "Return information about a font named NAME on frame FRAME.\nIf FRAME is omitted or nil, use the selected frame.\n\nThe returned value is a vector of 14 elements:\n  [ OPENED-NAME FULL-NAME SIZE HEIGHT BASELINE-OFFSET RELATIVE-COMPOSE\n    DEFAULT-ASCENT MAX-WIDTH ASCENT DESCENT SPACE-WIDTH AVERAGE-WIDTH\n    FILENAME CAPABILITY ]\nwhere\n  OPENED-NAME is the name used for opening the font,\n  FULL-NAME is the full name of the font,\n  SIZE is the pixelsize of the font,\n  HEIGHT is the pixel-height of the font (i.e., ascent + descent),\n  BASELINE-OFFSET is the upward offset pixels from ASCII baseline,\n  RELATIVE-COMPOSE and DEFAULT-ASCENT are the numbers controlling\n    how to compose characters,\n  MAX-WIDTH is the maximum advance width of the font,\n  ASCENT, DESCENT, SPACE-WIDTH, and AVERAGE-WIDTH are metrics of\n    the font in pixels,\n  FILENAME is the font file name, a string (or nil if the font backend\n    doesn't provide a file name).\n  CAPABILITY is a list whose first element is a symbol representing the\n    font format, one of `x', `opentype', `truetype', `type1', `pcf', or `bdf'.\n    The remaining elements describe the details of the font capabilities,\n    as follows:\n\n      If the font is OpenType font, the form of the list is\n        (opentype GSUB GPOS)\n      where GSUB shows which \"GSUB\" features the font supports, and GPOS\n      shows which \"GPOS\" features the font supports.  Both GSUB and GPOS are\n      lists of the form:\n\t((SCRIPT (LANGSYS FEATURE ...) ...) ...)\n\n      where\n        SCRIPT is a symbol representing OpenType script tag.\n        LANGSYS is a symbol representing OpenType langsys tag, or nil\n         representing the default langsys.\n        FEATURE is a symbol representing OpenType feature tag.\n\n      If the font is not an OpenType font, there are no elements\n      in CAPABILITY except the font format symbol.\n\nIf the named font cannot be opened and loaded, return nil.",
       (name: Lisp_Object, frame: Lisp_Object),
       {
  let f: *mut frame;
  let font: *mut font;
  let info: Lisp_Object;
  let mut font_object: Lisp_Object;
  let mut name: Lisp_Object = name;

  if ! FONTP (name)
    { CHECK_STRING (name); }
  f = decode_window_system_frame (frame);

  if STRINGP (name)
    {
      let fontset: c_int = fs_query_fontset (name, 0);

      if fontset >= 0
	{ name = fontset_ascii (fontset); }
      font_object = font_open_by_name (f, name);
    }
  else if FONT_OBJECT_P (name)
    { font_object = name; }
  else if FONT_ENTITY_P (name)
    { font_object = font_open_entity (f, name, 0); }
  else
    {
      let face: *mut face = FACE_FROM_ID (f, DEFAULT_FACE_ID);
      let entity: Lisp_Object = font_matching_entity (f, (*face).lface.as_mut_ptr (), name);

      font_object = if ! NILP (entity) { font_open_entity (f, entity, 0) } else { Qnil () };
    }
  if NILP (font_object)
    { return Qnil (); }
  font = XFONT_OBJECT (font_object);

  /* Sanity check to make sure we have initialized max_width.  */
  eassert! ((*XFONT_OBJECT (font_object)).max_width < 1024 * 1024 * 1024);

  info = CALLN! (Fvector,
		 AREF (font_object, FONT_NAME_INDEX as isize),
		 AREF (font_object, FONT_FULLNAME_INDEX as isize),
		 make_fixnum ((*font).pixel_size as EMACS_INT),
		 make_fixnum ((*font).height as EMACS_INT),
		 make_fixnum ((*font).baseline_offset as EMACS_INT),
		 make_fixnum ((*font).relative_compose as EMACS_INT),
		 make_fixnum ((*font).default_ascent as EMACS_INT),
		 make_fixnum ((*font).max_width as EMACS_INT),
		 make_fixnum ((*font).ascent as EMACS_INT),
		 make_fixnum ((*font).descent as EMACS_INT),
		 make_fixnum ((*font).space_width as EMACS_INT),
		 make_fixnum ((*font).average_width as EMACS_INT),
		 AREF (font_object, FONT_FILE_INDEX as isize),
		 (if (*((*font).driver as *const font_driver)).otf_capability.is_some ()
		  { Fcons (Qopentype (),
			   ((*((*font).driver as *const font_driver)).otf_capability.unwrap ()) (font)) }
		  else { Qnil () }));

  /* C 5685-5689: `#if 0` (the `font_close_object` call).  Recorded in docs/ports/font.md.  */
  return info;
});


/* C 5703: `#define BUILD_STYLE_TABLE(TBL) build_style_table (TBL, countof (TBL))`.  */
macro_rules! BUILD_STYLE_TABLE {
  ($tbl:expr) => { build_style_table ($tbl.as_ptr (), countof! ($tbl) as c_int) };
}

unsafe fn build_style_table (entry: *const table_entry, nelement: c_int) -> Lisp_Object
{
  let table: Lisp_Object = make_nil_vector (nelement as ptrdiff_t);
  let mut i: c_int = 0;
  while i < nelement
    {
      let mut j: c_int;
      j = 0;
      while !(*entry.offset (i as isize)).names[j as usize].is_null ()
	{ j += 1; }
      let elt: Lisp_Object = make_nil_vector (j as ptrdiff_t + 1);
      ASET (elt, 0, make_fixnum ((*entry.offset (i as isize)).numeric as EMACS_INT));
      j = 0;
      while !(*entry.offset (i as isize)).names[j as usize].is_null ()
	{
	  ASET (elt, j as isize + 1,
		intern_c_string ((*entry.offset (i as isize)).names[j as usize]));
	  j += 1;
	}
      ASET (table, i as isize, elt);
      i += 1;
    }
  return table;
}

/* The deferred font-log data of the form [ACTION ARG RESULT].
   If ACTION is not nil, that is added to the log when font_add_log is
   called next time.  At that time, ACTION is set back to nil.  */
static mut Vfont_log_deferred: Lisp_Object = lisp_h_Qnil ();

/* Prepend the font-related logging data in Vfont_log if it is not
   t.  ACTION describes a kind of font-related action (e.g. listing,
   opening), ARG is the argument for the action, and RESULT is the
   result of the action.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_add_log (action: *const c_char, arg: Lisp_Object,
				       result: Lisp_Object)
{
  let mut val: Lisp_Object;
  let mut i: c_int;
  let mut arg: Lisp_Object = arg;
  let mut result: Lisp_Object = result;

  if EQ (Vfont_log, Qt ())
    { return; }
  if STRINGP (AREF (Vfont_log_deferred, 0))
    {
      let str_: *mut c_char = SSDATA (AREF (Vfont_log_deferred, 0));

      ASET (Vfont_log_deferred, 0, Qnil ());
      font_add_log (str_, AREF (Vfont_log_deferred, 1),
		    AREF (Vfont_log_deferred, 2));
    }

  if FONTP (arg)
    {
      let mut tail: Lisp_Object;
      let mut elt: Lisp_Object;
      AUTO_STRING! (equal, c"=".as_ptr ());

      val = Ffont_xlfd_name (arg, Qt (), Qt ());
      tail = AREF (arg, FONT_EXTRA_INDEX as isize);
      while CONSP (tail)
	{
	  elt = XCAR (tail);
	  if EQ (XCAR (elt), QCscript ())
	      && SYMBOLP (XCDR (elt))
	    { val = concat3 (val, SYMBOL_NAME (QCscript ()),
			     concat2 (equal, SYMBOL_NAME (XCDR (elt)))); }
	  else if EQ (XCAR (elt), QClang ())
		   && SYMBOLP (XCDR (elt))
	    { val = concat3 (val, SYMBOL_NAME (QClang ()),
			     concat2 (equal, SYMBOL_NAME (XCDR (elt)))); }
	  else if EQ (XCAR (elt), QCotf ())
		   && CONSP (XCDR (elt)) && SYMBOLP (XCAR (XCDR (elt)))
	    { val = concat3 (val, SYMBOL_NAME (QCotf ()),
			     concat2 (equal, SYMBOL_NAME (XCAR (XCDR (elt))))); }
	  tail = XCDR (tail);
	}
      arg = val;
    }

  if CONSP (result)
      && VECTORP (XCAR (result))
      && ASIZE (XCAR (result)) > 0
      && FONTP (AREF (XCAR (result), 0))
    { result = font_vconcat_entity_vectors (result); }
  if FONTP (result)
    {
      val = Ffont_xlfd_name (result, Qt (), Qt ());
      if ! FONT_SPEC_P (result)
	{
	  AUTO_STRING! (colon, c":".as_ptr ());
	  val = concat3 (SYMBOL_NAME (AREF (result, FONT_TYPE_INDEX as isize)),
			 colon, val);
	}
      result = val;
    }
  else if CONSP (result)
    {
      let mut tail: Lisp_Object;
      result = Fcopy_sequence (result);
      tail = result;
      while CONSP (tail)
	{
	  val = XCAR (tail);
	  if FONTP (val)
	    { val = Ffont_xlfd_name (val, Qt (), Qt ()); }
	  XSETCAR (tail, val);
	  tail = XCDR (tail);
	}
    }
  else if VECTORP (result)
    {
      result = Fcopy_sequence (result);
      i = 0;
      while i < ASIZE (result) as c_int
	{
	  val = AREF (result, i as isize);
	  if FONTP (val)
	    { val = Ffont_xlfd_name (val, Qt (), Qt ()); }
	  ASET (result, i as isize, val);
	  i += 1;
	}
    }
  Vfont_log = Fcons (list3 (intern (action), arg, result), Vfont_log);
}

/* Record a font-related logging data to be added to Vfont_log when
   font_add_log is called next time.  ACTION, ARG, RESULT are the same
   as font_add_log.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_deferred_log (action: *const c_char, arg: Lisp_Object,
					    result: Lisp_Object)
{
  if EQ (Vfont_log, Qt ())
    { return; }
  ASET (Vfont_log_deferred, 0, build_string (action));
  ASET (Vfont_log_deferred, 1, arg);
  ASET (Vfont_log_deferred, 2, result);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn font_drop_xrender_surfaces (f: *mut frame)
{
  let mut list: *mut font_driver_list;

  list = (*f).font_driver_list as *mut font_driver_list;
  while !list.is_null ()
    {
      if (*list).on && (*((*list).driver)).drop_xrender_surfaces.is_some ()
	{
	  let drop_xrender_surfaces = (*((*list).driver)).drop_xrender_surfaces;
	  if let Some (d) = drop_xrender_surfaces
	    { d (f as *mut c_void); }
	}
      list = (*list).next;
    }
}

/* The `DEFVAR_*` storage this unit owns (status 3.29: the unit that owns the C definition
   owns the port's `static mut`).  `Vfont_weight_table`/`Vfont_slant_table`/
   `Vfont_width_table`/`Vfont_log` and the three `DEFVAR_BOOL`s are registered in
   `syms_of_font` below.  */
pub static mut Vfont_encoding_alist: Lisp_Object = lisp_h_Qnil ();
pub static mut Vfont_weight_table: Lisp_Object = lisp_h_Qnil ();
pub static mut Vfont_slant_table: Lisp_Object = lisp_h_Qnil ();
pub static mut Vfont_width_table: Lisp_Object = lisp_h_Qnil ();
/* C's `DEFVAR_LISP` in `syms_of_font` expands to the *definition* of the global plus its
   registration; `fontset.rs` references it from another unit, so the C symbol has to exist
   (`globals.rs` only declares it - status 4.721).  */
#[unsafe(no_mangle)]
pub static mut Vfont_log: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut inhibit_compacting_font_caches: bool = false;
pub static mut xft_ignore_color_fonts: bool = false;
pub static mut query_all_font_backends: bool = false;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_font ()
{
  sort_shift_bits[FONT_TYPE_INDEX as usize] = 0;
  sort_shift_bits[FONT_SLANT_INDEX as usize] = 2;
  sort_shift_bits[FONT_WEIGHT_INDEX as usize] = 9;
  sort_shift_bits[FONT_SIZE_INDEX as usize] = 16;
  sort_shift_bits[FONT_WIDTH_INDEX as usize] = 23;
  /* Note that the other elements in sort_shift_bits are not used.  */
  PDUMPER_REMEMBER_SCALAR! (sort_shift_bits);

  font_charset_alist = Qnil ();
  staticpro (&raw const font_charset_alist);

  DEFSYM! (Qopentype, c"opentype");

  /* Currently used by hbfont.c, which has no syms_of_hbfont function
     of its own.  */
  DEFSYM! (Qcanonical_combining_class, c"canonical-combining-class");

  /* Important character set symbols.  */
  DEFSYM! (Qascii_0, c"ascii-0");
  DEFSYM! (Qiso8859_1, c"iso8859-1");
  DEFSYM! (Qiso10646_1, c"iso10646-1");
  DEFSYM! (Qunicode_bmp, c"unicode-bmp");
  DEFSYM! (Qemoji, c"emoji");

  /* Symbols representing keys of font extra info.  */
  DEFSYM! (QCotf, c":otf");
  DEFSYM! (QClang, c":lang");
  DEFSYM! (QCscript, c":script");
  DEFSYM! (QCantialias, c":antialias");
  DEFSYM! (QCfoundry, c":foundry");
  DEFSYM! (QCadstyle, c":adstyle");
  DEFSYM! (QCregistry, c":registry");
  DEFSYM! (QCspacing, c":spacing");
  DEFSYM! (QCdpi, c":dpi");
  DEFSYM! (QCscalable, c":scalable");
  DEFSYM! (QCavgwidth, c":avgwidth");
  DEFSYM! (QCfont_entity, c":font-entity");
  DEFSYM! (QCcombining_capability, c":combining-capability");

  /* Symbols representing values of font spacing property.  */
  DEFSYM! (Qc, c"c");
  DEFSYM! (Qm, c"m");
  DEFSYM! (Qp, c"p");
  DEFSYM! (Qd, c"d");

  /* Special ADSTYLE properties to avoid fonts used for Latin
     characters; used in xfont.c and ftfont.c.  */
  DEFSYM! (Qja, c"ja");
  DEFSYM! (Qko, c"ko");

  DEFSYM! (QCuser_spec, c":user-spec");

  /* For shapers that need to know text directionality.  */
  DEFSYM! (QL2R, c"L2R");
  DEFSYM! (QR2L, c"R2L");

  DEFSYM! (Qfont_extra_type, c"font-extra-type");
  DEFSYM! (Qfont_driver_superseded_by, c"font-driver-superseded-by");

  scratch_font_spec = Ffont_spec (0, std::ptr::null_mut ());
  staticpro (&raw const scratch_font_spec);
  scratch_font_prefer = Ffont_spec (0, std::ptr::null_mut ());
  staticpro (&raw const scratch_font_prefer);

  Vfont_log_deferred = make_nil_vector (3);
  staticpro (&raw const Vfont_log_deferred);

  /* C 5911-5915: `#if 0` (the `otf_list` staticpro).  Recorded in docs/ports/font.md.  */

  defsubr (&raw mut Sfontp);
  defsubr (&raw mut Sfont_spec);
  defsubr (&raw mut Sfont_get);
  /* C 5921-5923: `#ifdef HAVE_WINDOW_SYSTEM` (undef in the frozen configuration), carried;
     its `defsubr (&Sfont_face_attributes)` is recorded, since the subr it names is defined
     only under that feature too.  */
  defsubr (&raw mut Sfont_put);
  defsubr (&raw mut Slist_fonts);
  defsubr (&raw mut Sfont_family_list);
  defsubr (&raw mut Sfind_font);
  defsubr (&raw mut Sfont_xlfd_name);
  defsubr (&raw mut Sclear_font_cache);
  defsubr (&raw mut Sfont_shape_gstring);
  defsubr (&raw mut Sfont_variation_glyphs);
  defsubr (&raw mut Sinternal_char_font);
  /* C 5933-5936: `#if 0` (Sfont_drive_otf, Sfont_otf_alternates).  Recorded in
     docs/ports/font.md.  */

  /* C 5938-5949: `#ifdef FONT_DEBUG`.  Live text: `font.h` 997-998 defines the macro and the
     row is measured in `docs/baseline/config/source-header-macros.tsv` (status 3.76), so
     `tools/extract-abi.py`'s `c_defsubr_live` demands these eight registrations too.  */
  defsubr (&raw mut Sopen_font);
  defsubr (&raw mut Sclose_font);
  defsubr (&raw mut Squery_font);
  defsubr (&raw mut Sfont_get_glyphs);
  defsubr (&raw mut Sfont_has_char_p);
  defsubr (&raw mut Sfont_match_p);
  defsubr (&raw mut Sfont_at);
  /* C 5946-5948: `#if 0` (`defsubr (&Sdraw_string)`).  Recorded in docs/ports/font.md.  */
  defsubr (&raw mut Sframe_font_cache);

  #[cfg(feature = "have_window_system")]
  defsubr (&raw mut Sfont_info);

  DEFVAR_LISP! (c"font-encoding-alist", Vfont_encoding_alist,
	       "\nAlist of fontname patterns vs the corresponding encoding and repertory info.\nEach element looks like (REGEXP . (ENCODING . REPERTORY)),\nwhere ENCODING is a charset or a char-table,\nand REPERTORY is a charset, a char-table, or nil.\n\nIf ENCODING and REPERTORY are the same, the element can have the form\n(REGEXP . ENCODING).\n\nENCODING is for converting a character to a glyph code of the font.\nIf ENCODING is a charset, encoding a character by the charset gives\nthe corresponding glyph code.  If ENCODING is a char-table, looking up\nthe table by a character gives the corresponding glyph code.\n\nREPERTORY specifies a repertory of characters supported by the font.\nIf REPERTORY is a charset, all characters belonging to the charset are\nsupported.  If REPERTORY is a char-table, all characters who have a\nnon-nil value in the table are supported.  If REPERTORY is nil, Emacs\ngets the repertory information by an opened font and ENCODING.  ");
  Vfont_encoding_alist = Qnil ();

  /* FIXME: These 3 vars are not quite what they appear: setq on them
     won't have any effect other than disconnect them from the style
     table used by the font display code.  So we make them read-only,
     to avoid this confusing situation.  */

  DEFVAR_LISP_NOPRO! (c"font-weight-table", Vfont_weight_table,
	       "Vector of valid font weight values.\nEach element has the form:\n    [NUMERIC-VALUE SYMBOLIC-NAME ALIAS-NAME ...]\nNUMERIC-VALUE is an integer, and SYMBOLIC-NAME and ALIAS-NAME are symbols.\nThis variable cannot be set; trying to do so will signal an error.");
  Vfont_weight_table = BUILD_STYLE_TABLE! (weight_table);
  make_symbol_constant (intern_c_string (c"font-weight-table".as_ptr ()));

  DEFVAR_LISP_NOPRO! (c"font-slant-table", Vfont_slant_table,
	       "Vector of font slant symbols vs the corresponding numeric values.\nSee `font-weight-table' for the format of the vector.\nThis variable cannot be set; trying to do so will signal an error.");
  Vfont_slant_table = BUILD_STYLE_TABLE! (slant_table);
  make_symbol_constant (intern_c_string (c"font-slant-table".as_ptr ()));

  DEFVAR_LISP_NOPRO! (c"font-width-table", Vfont_width_table,
	       "Alist of font width symbols vs the corresponding numeric values.\nSee `font-weight-table' for the format of the vector.\nThis variable cannot be set; trying to do so will signal an error.");
  Vfont_width_table = BUILD_STYLE_TABLE! (width_table);
  make_symbol_constant (intern_c_string (c"font-width-table".as_ptr ()));

  /* Because the above 3 variables are slots in the vector we create
     below, and because that vector is staticpro'd, we don't explicitly
     staticpro the variables, to avoid wasting slots in staticvec[].  */
  staticpro (&raw const font_style_table);
  font_style_table = CALLN! (Fvector, Vfont_weight_table, Vfont_slant_table,
			     Vfont_width_table);

  DEFVAR_LISP! (c"font-log", Vfont_log,
	       "A list that logs font-related actions and results, for debugging.\nThe default value is t, which means to suppress logging.\nSet it to nil to enable logging.  If the environment variable\nEMACS_FONT_LOG is set at startup, it defaults to nil.");
  Vfont_log = Qnil ();

  DEFVAR_BOOL! (c"inhibit-compacting-font-caches", inhibit_compacting_font_caches,
	       "If non-nil, don't compact font caches during GC.\nSome large fonts cause lots of consing and trigger GC.  If they\nare removed from the font caches, they will need to be opened\nagain during redisplay, which slows down redisplay.  If you\nsee font-related delays in displaying some special characters,\nand cannot switch to a smaller font for those characters, set\nthis variable non-nil.\nDisabling compaction of font caches might enlarge the Emacs memory\nfootprint in sessions that use lots of different fonts.");

  #[cfg(feature = "windowsnt")]
  {
    /* Compacting font caches causes slow redisplay on Windows with many
       large fonts, so we disable it by default.  */
    inhibit_compacting_font_caches = true;
  }
  #[cfg(not (feature = "windowsnt"))]
  { inhibit_compacting_font_caches = false; }

  DEFVAR_BOOL! (c"xft-ignore-color-fonts",
	       xft_ignore_color_fonts,
	       "Non-nil means don't query fontconfig for color fonts, since they often\ncause Xft crashes.  Only has an effect in Xft builds.");
  xft_ignore_color_fonts = true;

  DEFVAR_BOOL! (c"query-all-font-backends", query_all_font_backends,
               "If non-nil, attempt to query all available font backends.\nBy default Emacs will stop searching for a matching font at the first\nmatch.");
  query_all_font_backends = false;

  /* C 6053-6083: `#ifdef HAVE_WINDOW_SYSTEM` (undef in the frozen configuration), carried;
     the whole `syms_of_*font` chain is C's.  */
  #[cfg(feature = "have_window_system")]
  {
    #[cfg(feature = "freetype")]
    {
      syms_of_ftfont ();
      #[cfg(feature = "have_x_windows")]
      {
	syms_of_xfont ();
	#[cfg(feature = "use_cairo")]
	{ syms_of_ftcrfont (); }
	#[cfg(not (feature = "use_cairo"))]
	{
	  #[cfg(feature = "xft")]
	  { syms_of_xftfont (); }
	}
      }
      #[cfg(not (feature = "have_x_windows"))]
      {
	#[cfg(feature = "use_cairo")]
	{ syms_of_ftcrfont (); }
      }
    }
    #[cfg(not (feature = "freetype"))]
    {
      #[cfg(feature = "have_x_windows")]
      { syms_of_xfont (); }
    }
    #[cfg(feature = "bdffont")]
    { syms_of_bdffont (); }
    #[cfg(feature = "have_ntgui")]
    { syms_of_w32font (); }
    #[cfg(feature = "be_cairo")]
    { syms_of_ftcrfont (); }
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_font ()
{
  Vfont_log = if !egetenv (c"EMACS_FONT_LOG".as_ptr ()).is_null () { Qnil () } else { Qt () };
}

#[cfg(feature = "have_ns")]
unsafe extern "C" { fn get_frame_param(f: *mut frame, prop: Lisp_Object) -> Lisp_Object; }
