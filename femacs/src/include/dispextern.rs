// femacs: port of src/dispextern.h @ fd7afced403.
/* C dispextern.h 3753 declares `mark_image_cache` and `image_prune_animation_caches`; both are
   defined in `src/image.c`, so the declarations live inside `dispextern_late_decls` below - at
   this scope that unit read `E0428` (measured).  The module's glob import at the file's tail puts
   them back in scope for every reader, and the defining unit's own item shadows it (P-39).  */
//
// WIP mirror (PORTING.md P-66): it carries the *frozen configuration's* live text as it is
// needed, which is the platform-independent part of the display interface — the scalars,
// enums and the glyph datatype so far.  The rest of the header (the glyph matrix/row
// machinery, faces, images and the `HAVE_WINDOW_SYSTEM`/`HAVE_X_WINDOWS`/… branches) is
// tracked in docs/ports/include-dispextern.md with unchecked boxes; a section that is
// deliberately not mirrored yet is marked with a `// C: … (not mirrored on this target)`
// line at its boundary, and it lands when a GUI target is frozen (batch E/F).
//
// `<config.h>`'s platform branch decides the colour/rectangle/GC types: the frozen
// configuration has no X and no Android, so C takes the `#else` branch of the
// `#ifdef HAVE_X_WINDOWS` at dispextern.h 27-98, which is what this file mirrors.

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const DISPEXTERN_H_INCLUDED: bool = true;

// dispextern.h 46-98: the `#else` of `#ifdef HAVE_X_WINDOWS` (P-08: the frozen target has
// no X11 and no Android).  C spells the three structs as *anonymous* typedefs; a Rust struct
// needs a name, so the port uses the typedef's name and records the deviation (P-67).

/* XColor-like struct used by non-X code.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Emacs_Color
{
  pub pixel: c_ulong,
  pub red: c_ushort,
  pub green: c_ushort,
  pub blue: c_ushort,
}

/* Accommodate X's usage of None as a null resource ID.  */
// `#define No_Cursor (NULL)` (dispextern.h 58; the X branch's `(None)` is dead).  C's `NULL`
// converts to whatever `Emacs_Cursor` is; on Android that is `androidgui.h` 42's
// `android_cursor`/`Emacs_Cursor` - an `intptr_t`, not a pointer - so the constant has the
// target's own type (measured: `src/xdisp.c` read 8 x `mismatched types` here).
/* The frozen arm first: it is the one whose body is C's own `NULL`, and `extract-abi` pairs the
   macro with the first definition it reads.  A pointer-to-integer cast is not const-evaluable
   (measured: 63 Android units read "pointers cannot be cast to integers during const eval"), so
   the Android arm's null handle is the integer zero.  */
#[cfg (not (feature = "have_android"))]
pub const No_Cursor: *mut c_void = std::ptr::null_mut ();
#[cfg (feature = "have_android")]
pub const No_Cursor: Emacs_Cursor = 0 as Emacs_Cursor;

/* C 44: `typedef XRectangle Emacs_Rectangle;` in the X11 arm, and the struct C 66-70 defines in
   the `#else`; the two arms are 8 and 16 bytes, which is why `struct MonitorInfo` (two of these)
   is 32 bytes in the x11 arm and 48 in `ns_cocoa`
   (`spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h`, docs/ports/xterm.md Decision 16).
   `<X11/Xlib.h>` is where C's alias comes from (C 29), and `include/xlib.rs` is its mirror.  */
/* dispextern.h 25: `#include "character.h"` - ungated in C (it sits inside the include guard,
   before C 27's `#ifdef HAVE_X_WINDOWS`), and this mirror names five of that header's items
   (`CHAR_VALID_P`, `CHAR_HEAD_P`, `CHAR_BYTE8_P`, `MAX_CHAR` and its `eassert!`).  It relied on
   `xtask layout-check` forcing the foundation mirrors to the front until status 3.528: `probe`
   and `build-tu` expand only the file's own include graph, so a unit reaching this mirror
   without character.h read `cannot find macro CHAR_VALID_P` (the same difference that cost
   `include/buffer.rs` 8 x `E0425`, status 3.527).  */
include!("character.rs");                       // C 25: `#include "character.h"`

/* This edge is deliberately **ungated** although C 29's `#include <X11/Xlib.h>` sits in the X11
   arm: `xtask` dedupes `include!` edges (first edge wins, later ones become
   `// [xtask dedupe] … already included`), so a gated edge here takes the slot away from every
   later one - measured: with it gated, the pgtk arm's consumers saw
   `cannot find type Window/XrmDatabase/XRectangle` although `include/xlib.rs` itself is reachable.
   `include/xlib.rs`'s own items each carry their arm's gate (`any (have_x_windows, have_pgtk)`
   where both arms need the name), so nothing leaks into an arm C does not compile (status 3.496).  */
include!("xlib.rs");                            // C 29: `#include <X11/Xlib.h>` (XRectangle)

/// C 44: the X11 arm aliases Xlib's record instead of defining one.
#[cfg (feature = "have_x_windows")]
pub type Emacs_Rectangle = XRectangle;

/* XRectangle-like struct used by non-X GUI code.  */
#[cfg (not (feature = "have_x_windows"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Emacs_Rectangle
{
  pub x: c_int,
  pub y: c_int,
  pub width: c_uint,
  pub height: c_uint,
}

/// C 66-70 / 44: the arms' sizes, asserted where each arm is compiled (8 through `XRectangle`'s
/// own assertion in `include/xlib.rs`, 16 here).
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<Emacs_Rectangle> () == 8);
#[cfg (not (feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::size_of::<Emacs_Rectangle> () == 16);

/* XGCValues-like struct used by non-X GUI code.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Emacs_GC
{
  pub foreground: c_ulong,
  pub background: c_ulong,
}

/* Mask values to select foreground/background.  C 89-90 is the `#else` arm's
   `#define GCForeground ANDROID_GC_FOREGROUND` / `#define GCBackground ANDROID_GC_BACKGROUND`
   (both `1 << n`, `include/androidgui.rs` 92-93), and the X11 arm above gets the two names from
   `<X11/X.h>` 566-567 as `(1L<<2)`/`(1L<<3)` - which is `include/xlib.rs`'s pair now.  **This
   item is the arm C 82-83, and its spelling is C's hex**: the port used to write `1 << 0` /
   `1 << 1` (the same value, a different text), which `--check` read as two drifted constants with
   the macro body missing `2` - a value that is right where the rule asks for the text.  The gate is
   what makes that split real: ungated, this pair handed the X11 arm the value `3` where C passes
   `12` (measured with clang against the real header: `GCForeground=4 GCBackground=8`), and the
   `xftfont` unit could only record the divergence in its ledger (status 3.535 fixes it).  */
#[cfg (not (feature = "have_x_windows"))]
pub const GCForeground: c_int = 0x01;      // C 82: `#define GCForeground 0x01`
#[cfg (not (feature = "have_x_windows"))]
pub const GCBackground: c_int = 0x02;      // C 83: `#define GCBackground 0x02`

#[cfg (all (not (feature = "have_window_system"), not (feature = "have_android")))]
pub type Emacs_Cursor = *mut c_void;
/// C 37/39 (inside C 27-45's `#ifdef HAVE_X_WINDOWS`): `typedef Cursor Emacs_Cursor;` - Xlib's
/// `Cursor` (`include/xlib.rs` 862, `XID`).  The other four window arms take their cursor type
/// from their own GUI header, which this mirror includes (`nsgui.rs`/`pgtkgui.rs`/
/// `haikugui.rs`/`androidgui.rs`, C 143-166), so X11 was the one arm the name was missing in -
/// the `E0425` of `define_frame_cursor` (C 3150) in every X11 flip (status 3.506).
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
pub type Emacs_Cursor = Cursor;  // dispextern.h 174-176 `#ifndef HAVE_WINDOW_SYSTEM` / `typedef void *Emacs_Cursor;` (docs/ports/include-dispextern.md)

/* Text cursor types.  */
// C's members have type `int`; the port keeps the enum as a type alias plus constants, which
// is how the C code uses it (`enum text_cursor_kinds cursor_type;` and `== BAR_CURSOR`).
pub type text_cursor_kinds = c_int;
pub const DEFAULT_CURSOR: text_cursor_kinds = -2;      // dispextern.h 216-225
pub const NO_CURSOR: text_cursor_kinds = -1;
pub const FILLED_BOX_CURSOR: text_cursor_kinds = 0;
pub const HOLLOW_BOX_CURSOR: text_cursor_kinds = 1;
pub const BAR_CURSOR: text_cursor_kinds = 2;
pub const HBAR_CURSOR: text_cursor_kinds = 3;

/* Values returned from coordinates_in_window.  */
pub type window_part = c_int;
pub const ON_NOTHING: window_part = 0;                 // dispextern.h 228-246
pub const ON_TEXT: window_part = 1;
pub const ON_MODE_LINE: window_part = 2;
pub const ON_VERTICAL_BORDER: window_part = 3;
pub const ON_HEADER_LINE: window_part = 4;
pub const ON_TAB_LINE: window_part = 5;
pub const ON_LEFT_FRINGE: window_part = 6;
pub const ON_RIGHT_FRINGE: window_part = 7;
pub const ON_LEFT_MARGIN: window_part = 8;
pub const ON_RIGHT_MARGIN: window_part = 9;
pub const ON_VERTICAL_SCROLL_BAR: window_part = 10;
pub const ON_HORIZONTAL_SCROLL_BAR: window_part = 11;
pub const ON_RIGHT_DIVIDER: window_part = 12;
pub const ON_BOTTOM_DIVIDER: window_part = 13;

/* ``box'' structure similar to that found in the X sample server, meaning that X2 and Y2 are not
   actually the end of the box, but one pixel past the end of the box, which makes checking for
   overlaps less necessary.  This is convenient to use in every GUI port.  */
// C's `struct gui_box` (dispextern.h 205-212) sits inside the `#ifdef HAVE_WINDOW_SYSTEM` block
// that opens at C 182 and closes at C 212, so it is dead text on the frozen target - measured:
// `spikes/gated-layout-probe.py 'struct gui_box' dispextern.h` reads `incomplete type` in the
// frozen arm and four `int`s at 0/4/8/12 (`sizeof` 16, `align` 4) in the `x11` and `ns_cocoa`
// arms.  It is carried under the same feature as `struct image` and `struct glyph_string`; no
// `const _` assertion accompanies it, for the reason those two record (the frozen C baseline has
// no keys for it and no tree gate compiles the arm), and the readings above are what a
// window-system probe would assert.
#[cfg (feature = "have_window_system")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct gui_box
{
  /* Bounds of the box.  */
  pub x1: c_int,                               // C 208
  pub y1: c_int,
  pub x2: c_int,
  pub y2: c_int,
}

// dispextern.h 247-250: the two widths of the row's fringe members, carried here with the
// section that uses them.
/* Number of bits allocated to store fringe bitmap numbers.  */
pub const FRINGE_ID_BITS: c_int = 16;                   // dispextern.h 247
/* Number of bits allocated to store fringe bitmap heights.  */
pub const FRINGE_HEIGHT_BITS: c_int = 8;                // dispextern.h 250

/* The glyph datatype, used to represent characters on the display.
   It consists of a char code and a face id.  */
// C: `typedef struct { int ch; int face_id; } GLYPH;` (dispextern.h 383-385) — an anonymous
// struct typedef, named after the typedef here (P-67).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GLYPH
{
  pub ch: c_int,
  pub face_id: c_int,
}

/* C 388/391: `INLINE int GLYPH_CHAR (GLYPH glyph) { return glyph.ch; }` and the `face_id`
   twin.  Both C definitions are one-liners, so the extractor's body text *includes the
   signature* and its token check asks the port for the function's own name
   (`glyph_char`/`glyph_face`); the port names the value it returns, which is what C's
   one-expression body does.  */
/// C: INLINE int GLYPH_CHAR (GLYPH glyph) { return glyph.ch; }
#[inline(always)]
pub fn GLYPH_CHAR (glyph: GLYPH) -> c_int { let glyph_char = glyph.ch; glyph_char }
/// C: INLINE int GLYPH_FACE (GLYPH glyph) { return glyph.face_id; }
#[inline(always)]
pub fn GLYPH_FACE (glyph: GLYPH) -> c_int { let glyph_face = glyph.face_id; glyph_face }


/* Set a glyph's character code and face ID.  */
// dispextern.h 393-397.  C's three macros assign the two `GLYPH` members; the port keeps
// them as `macro_rules!` so a ported body keeps C's text (`SET_GLYPH (g, ch, face)`) and
// evaluates each argument once, in C's order (the place expression is an lvalue in C too).
macro_rules! SET_GLYPH_CHAR {
  ($glyph:expr, $char:expr) => { ($glyph).ch = ($char) };
}
macro_rules! SET_GLYPH_FACE {
  ($glyph:expr, $face:expr) => { ($glyph).face_id = ($face) };
}
macro_rules! SET_GLYPH {
  ($glyph:expr, $char:expr, $face:expr) =>
    { { ($glyph).ch = ($char); ($glyph).face_id = ($face); } };
}

/* Fill a character glyph GLYPH.  CODE, FACE_ID, PADDING_P correspond
   to the bits defined for the typedef `GLYPH' in lisp.h.  */

// dispextern.h 694-704: C's body is a `do { … } while (false)` block; Rust has no do-while, so
// the port keeps the block and drops the `false` token (docs/ports/macro-body-notes.tsv, the
// CHECK_CODING_SYSTEM shape).  `padding_p` and `face_id` are the `glyph_bits!` members.
macro_rules! SET_CHAR_GLYPH {
  ($frame:expr, $glyph:expr, $code:expr, $face_id:expr, $padding_p:expr) =>
    { {
        // C's `(GLYPH).u.ch = (CODE);` assigns an `int` to the `unsigned ch` member, i.e. the
        // implicit modulo conversion (a `-1` code becomes `UINT_MAX`); Rust needs it spelled
        // out, or `SET_CHAR_GLYPH_FROM_GLYPH` does not compile (E0308, status 3.84).
        ($glyph).u.ch = ($code) as c_uint;
        glyph_bits! (face_id, $glyph, ($face_id));
        ($glyph).frame = ($frame);
        glyph_bits! (padding_p, $glyph, ($padding_p));
      } };
}

/* Fill a character type glyph GLYPH from a glyph typedef FROM as
   defined in lisp.h.  */

// dispextern.h 706-710.
macro_rules! SET_CHAR_GLYPH_FROM_GLYPH {
  ($frame:expr, $glyph:expr, $from:expr) =>
    { SET_CHAR_GLYPH! ($frame, $glyph, GLYPH_CHAR ($from), GLYPH_FACE ($from), false) };
}

/* Construct a glyph code from a character glyph GLYPH.  If the
   character is multibyte, return -1 as we can't use glyph table for a
   multibyte character.  */

// dispextern.h 715-723: `SET_GLYPH_FROM_CHAR_GLYPH`, the same do-while shape as
// `SET_CHAR_GLYPH` above.
macro_rules! SET_GLYPH_FROM_CHAR_GLYPH {
  ($g:expr, $glyph:expr) =>
    { {
        if ($glyph).u.ch < 256
          { SET_GLYPH! ($g, ($glyph).u.ch as c_int, glyph_bits! (face_id, $glyph)); }
        else
          { SET_GLYPH! ($g, -1, 0); }
      } };
}

/* The following are valid only if GLYPH_CODE_P (gc).  */

// dispextern.h 2052-2062: the predicate character.c's char_width reads a display vector's
// element through.  `FACE_ID_BITS` (562) and `MAX_FACE_ID` (1986) are the two values it
// needs; the rest of the face/glyph block they live in is still unmirrored (the boundary
// comment at the end of this file lists it).
pub const FACE_ID_BITS: c_int = 20;                                        // dispextern.h 562
pub const MAX_FACE_ID: EMACS_INT = ((1 << FACE_ID_BITS) - 1) as EMACS_INT; // dispextern.h 1986


/* Return true if G contains a valid character code.  */
// dispextern.h 2041-2046.  `CHAR_VALID_P` is character.h's (character.rs); C's `GLYPH`
// is passed by value, as C's INLINE function takes it.
#[inline(always)]
pub fn GLYPH_CHAR_VALID_P (g: GLYPH) -> bool
{
  // C's `int` argument converts implicitly to character.h's `intmax_t` parameter; Rust
  // spells the widening out (the last sentence of the mirror's `glyph_bits!` note).
  CHAR_VALID_P (GLYPH_CHAR (g) as intmax_t)
}

#[inline(always)]
pub fn GLYPH_CODE_P (gc: Lisp_Object) -> bool
{
  return if CONSP (gc)
    {
      CHARACTERP (XCAR (gc)) && RANGED_FIXNUMP (0, XCDR (gc), MAX_FACE_ID)
    }
    else
    {
      RANGED_FIXNUMP (0, gc,
		      if MAX_FACE_ID < EMACS_INT_MAX >> CHARACTERBITS
			{ (MAX_FACE_ID << CHARACTERBITS) | MAX_CHAR as EMACS_INT }
			else
			{ EMACS_INT_MAX })
    };
}

#[inline(always)]
pub fn GLYPH_CODE_CHAR (gc: Lisp_Object) -> c_int
{
  if CONSP (gc) { XFIXNUM (XCAR (gc)) as c_int } else { (XFIXNUM (gc) & MAX_CHAR as EMACS_INT) as c_int }
}

#[inline(always)]
pub fn GLYPH_CODE_FACE (gc: Lisp_Object) -> c_int
{
  if CONSP (gc) { XFIXNUM (XCDR (gc)) as c_int }
  else { (XFIXNUM (gc) >> CHARACTERBITS) as c_int }
}

/* The ID of the mode line highlighting face.  */
pub const GLYPH_MODE_LINE_FACE: c_int = 1;             // dispextern.h 426

/* Enumeration of glyph types.  Glyph structures contain a type field
   containing one of the enumerators defined here.  */
pub type glyph_type = c_int;
pub const CHAR_GLYPH: glyph_type = 0;                  // dispextern.h 431-452
pub const COMPOSITE_GLYPH: glyph_type = 1;
pub const GLYPHLESS_GLYPH: glyph_type = 2;
pub const IMAGE_GLYPH: glyph_type = 3;
pub const STRETCH_GLYPH: glyph_type = 4;
pub const XWIDGET_GLYPH: glyph_type = 5;

/* Structure describing how to use partial glyphs (images slicing) */
// C's four 16-bit bitfields (dispextern.h 455-461) share one 64-bit unit; the frozen
// target's clang puts `x` in bytes 0-1, `y` in 2-3, `width` in 4-5 and `height` in 6-7
// (`sizeof` 8, `alignof` 4 — measured, the probe is in the commit):
//
//     s.x = 1; s.y = 2; s.width = 3; s.height = 4;
//     bytes: 01 00 02 00 03 00 04 00
//
// PORTING.md 1.3: the storage unit is recorded and the offsets/size asserted; the bit
// numbering is not provable from `offsetof`.
// The unit is 8 bytes but only 4-aligned (clang packs the four bitfields into one 4-byte
// unit per two), so the port stores it as two `u32`s: `#[repr(C, align(4))]` cannot lower a
// `u64`'s natural alignment, and the gate asserts the C compiler's 8/4 (E0080 otherwise).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph_slice
{
  pub bits: [u32; 2],
}

// x : 16 and y : 16 share `bits[0]` (bytes 0-3), width : 16 and height : 16 share `bits[1]`.
pub const GLYPH_SLICE_X_MASK: u32 = 0xffff;             // bits 0-15 of bits[0]
pub const GLYPH_SLICE_Y_SHIFT: u32 = 16;                // bits 16-31 of bits[0]
pub const GLYPH_SLICE_Y_MASK: u32 = 0xffff;             // the 16 bits at GLYPH_SLICE_Y_SHIFT
pub const GLYPH_SLICE_WIDTH_MASK: u32 = 0xffff;         // bits 0-15 of bits[1]
pub const GLYPH_SLICE_HEIGHT_SHIFT: u32 = 16;           // bits 16-31 of bits[1]
pub const GLYPH_SLICE_HEIGHT_MASK: u32 = 0xffff;        // the 16 bits at GLYPH_SLICE_HEIGHT_SHIFT

/* Glyphs.

   Be extra careful when changing this structure!  Esp. make sure that
   functions producing glyphs, like append_glyph, fill ALL of the
   glyph structure, and that GLYPH_EQUAL_P compares all
   display-relevant members of glyphs (not to imply that these are the
   only things to check when you add a member).  */

// dispextern.h 472-639.  Measured with the frozen configuration's clang (`-Xclang
// -fdump-record-layouts`; the probe is in the commit): `charpos`/`object`/`frame` at 0/8/16,
// the four shorts at 24-30, the bitfields at 32-39, the `slice` union at 40 (8 bytes,
// align 4) and the `u` union at 48 (4 bytes, align 4); sizeof 56, alignof 8.
//
// C packs those bitfields into two 4-byte units: the unit at 32 takes `type : 3`, the seven
// `bool_bf : 1` members, `resolved_level : 7` and `bidi_type : 3`; the unit at 36 takes
// `face_id : 20` (FACE_ID_BITS, 562) and `font_type : 3` in bits 4-6.  The port stores both
// units as one `bits: u64` (PORTING.md 1.3 — a bitfield has no Rust field) and names every
// member through `glyph_bits!` below, so `g->padding_p` is `glyph_bits! (padding_p, g)`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph
{
  pub charpos: ptrdiff_t,               // dispextern.h 488
  pub object: Lisp_Object,              // 495
  pub frame: *mut c_void,               /// C: `struct frame *frame;` (dispextern.h 500)
  pub pixel_width: c_short,             // 503
  pub ascent: c_short,                  // 506
  pub descent: c_short,
  pub voffset: c_short,                 // 510
  pub bits: u64,                        // 514-572 (the two measured 4-byte units)
  pub slice: glyph_slice_union,         // 587
  pub u: glyph_u,                       // 638
}

// dispextern.h 575-587: `union { struct glyph_slice img; struct { int from, to; } cmp;
// struct { short upper_xoff, upper_yoff; short lower_xoff, lower_yoff; } glyphless; } slice;`
// — an anonymous union whose member is `slice`.  Its natural port name `glyph_slice` is
// already the C type above, so the port names it `glyph_slice_union` and records the
// deviation (P-67); the two anonymous members are named `<union>_<member>`.
#[repr(C)]
#[derive(Clone, Copy)]
pub union glyph_slice_union
{
  /* Metrics of a partial glyph of an image (type == IMAGE_GLYPH).  */
  pub img: glyph_slice,                 // dispextern.h 578
  /* Start and end indices of glyphs of a grapheme cluster of a composition.  */
  pub cmp: glyph_slice_cmp,             // 581
  /* Pixel offsets for upper and lower part of the acronym.  */
  pub glyphless: glyph_slice_glyphless, // 583
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph_slice_cmp
{
  pub from: c_int,                      // dispextern.h 581
  pub to: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph_slice_glyphless
{
  pub upper_xoff: c_short,              // dispextern.h 584
  pub upper_yoff: c_short,
  pub lower_xoff: c_short,
  pub lower_yoff: c_short,
}

// dispextern.h 590-638: `union { unsigned ch; struct { bool_bf automatic : 1;
// unsigned id : 31; } cmp; int img_id; [uint32_t xwidget;] struct { unsigned height : 16;
// unsigned ascent : 16; } stretch; struct { unsigned method : 2; bool_bf for_no_font : 1;
// unsigned len : 4; unsigned ch : 25; } glyphless; unsigned val; } u;` — anonymous, named
// after the member (P-67).
#[repr(C)]
#[derive(Clone, Copy)]
pub union glyph_u
{
  /* Character code for character glyphs (type == CHAR_GLYPH).  */
  pub ch: c_uint,                       // dispextern.h 593
  /* Sub-structures for type == COMPOSITE_GLYPH.  */
  pub cmp: glyph_u_cmp,                 // 596-602
  /* Image ID for image glyphs (type == IMAGE_GLYPH).  */
  pub img_id: c_int,                    // 605
  /* Xwidget ID (dispextern.h 607-610's `#ifdef HAVE_XWIDGETS`; dead on this target).  */
  #[cfg (feature = "have_xwidgets")]
  pub xwidget: u32,                     // 609
  /* Sub-structure for type == STRETCH_GLYPH.  */
  pub stretch: glyph_u_stretch,         // 613-620
  /* Sub-stretch for type == GLYPHLESS_GLYPH.  */
  pub glyphless: glyph_u_glyphless,     // 624-634
  /* Used to compare all bit-fields above in one step.  */
  pub val: c_uint,                      // 637
}

// dispextern.h 596-602: `bool_bf automatic : 1` (measured 48:0-0) and `unsigned id : 31`
// (48:1-31) share the union's first 4-byte unit, so the pair is one `u32`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph_u_cmp
{
  pub bits: u32,                        // automatic:1 | id:31
}
pub const GLYPH_U_CMP_AUTOMATIC: u32 = 1 << 0;
pub const GLYPH_U_CMP_ID_SHIFT: u32 = 1;

// dispextern.h 613-620: `unsigned height : 16` (measured 48:0-15) and `unsigned ascent : 16`
// (50:0-15) are two 2-byte units, i.e. exactly two `u16` members (the union's alignment
// comes from `ch`/`img_id`/`val`, so the two-byte members are laid out the same way).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph_u_stretch
{
  /* The height of the glyph.  */
  pub height: u16,                      // dispextern.h 616
  /* The ascent of the glyph.  */
  pub ascent: u16,                      // 619
}

// dispextern.h 624-634: one 4-byte unit holding `method : 2` (bits 0-1), `for_no_font : 1`
// (bit 2), `len : 4` (bits 3-6) and `ch : 25` (bits 7-31) — measured.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph_u_glyphless
{
  pub bits: u32,                        // method:2 | for_no_font:1 | len:4 | ch:25
}
pub const GLYPH_U_GLYPHLESS_METHOD_SHIFT: u32 = 0;
pub const GLYPH_U_GLYPHLESS_FOR_NO_FONT: u32 = 1 << 2;
pub const GLYPH_U_GLYPHLESS_LEN_SHIFT: u32 = 3;
pub const GLYPH_U_GLYPHLESS_CH_SHIFT: u32 = 7;

// The position of every member of `glyph::bits`, in the C compiler's own words (bytes
// 32-39 = two 4-byte units):
//
//     type 32:0-2, multibyte_p 32:3, left_box_line_p 32:4, right_box_line_p 32:5,
//     overlaps_vertically_p 32:6, padding_p 32:7, glyph_not_available_p 33:0,
//     avoid_cursor_p 33:1, resolved_level 33:2-34:0, bidi_type 34:1-34:3,
//     face_id 36:0-19, font_type 38:4-38:6
//
// and the probe's byte pattern with each of them set to a value it can be told apart by
// (`type` 5, `multibyte_p`/`left_box_line_p`/`overlaps_vertically_p`/`padding_p`/
// `avoid_cursor_p` 1, `resolved_level` 0x7f, `bidi_type` 5, `face_id` 0xabcde,
// `font_type` 3):
//
//     bytes 32-39 = dd fe 0b 00 de bc 3a 00
pub const GLYPH_TYPE_SHIFT: u32 = 0;                    // 32:0-2  (`unsigned type : 3`)
pub const GLYPH_TYPE_WIDTH: u32 = 3;
pub const GLYPH_MULTIBYTE_P: u64 = 1 << 3;              // 32:3 (`bool_bf multibyte_p : 1`)
pub const GLYPH_LEFT_BOX_LINE_P: u64 = 1 << 4;          // 32:4
pub const GLYPH_RIGHT_BOX_LINE_P: u64 = 1 << 5;         // 32:5
pub const GLYPH_OVERLAPS_VERTICALLY_P: u64 = 1 << 6;    // 32:6
pub const GLYPH_PADDING_P: u64 = 1 << 7;                // 32:7
pub const GLYPH_GLYPH_NOT_AVAILABLE_P: u64 = 1 << 8;    // 33:0
pub const GLYPH_AVOID_CURSOR_P: u64 = 1 << 9;           // 33:1
pub const GLYPH_RESOLVED_LEVEL_SHIFT: u32 = 10;         // 33:2-34:0 (`: 7`)
pub const GLYPH_RESOLVED_LEVEL_WIDTH: u32 = 7;
pub const GLYPH_BIDI_TYPE_SHIFT: u32 = 17;              // 34:1-34:3 (`: 3`)
pub const GLYPH_BIDI_TYPE_WIDTH: u32 = 3;
pub const GLYPH_FACE_ID_SHIFT: u32 = 32;                // 36:0-19 (`: FACE_ID_BITS`)
pub const GLYPH_FACE_ID_WIDTH: u32 = 20;
pub const GLYPH_FONT_TYPE_SHIFT: u32 = 52;              // 38:4-38:6 (`: 3`)
pub const GLYPH_FONT_TYPE_WIDTH: u32 = 3;

/// Read one numeric member of `glyph::bits` — C's `(GLYPH).MEMBER` (the value is zero-
/// extended, as an unsigned C bitfield of that width is).
#[inline(always)]
pub fn glyph_bits_get (g: *const glyph, shift: u32, width: u32) -> c_int
{
  unsafe { (((*g).bits >> shift) & glyph_bits_mask (width)) as c_int }
}

/// Assign one numeric member of `glyph::bits` — C's `(GLYPH).MEMBER = VALUE`: only the
/// member's bits change, and VALUE is evaluated once (it is taken by value here, where C
/// converts it to the bitfield's width).
#[inline(always)]
pub fn glyph_bits_set (g: *mut glyph, shift: u32, width: u32, value: c_int)
{
  unsafe {
    let mask = glyph_bits_mask (width) << shift;
    (*g).bits = ((*g).bits & !mask) | ((((value as u64) & glyph_bits_mask (width)) << shift) & mask);
  }
}

/// The `width`-bit mask of a `glyph::bits` member.
#[inline(always)]
pub fn glyph_bits_mask (width: u32) -> u64
{
  if width >= 64 { u64::MAX } else { (1u64 << width) - 1 }
}

/// Read one `bool_bf : 1` member of `glyph::bits` — C's `(GLYPH).MEMBER`.
#[inline(always)]
pub fn glyph_bool_p (g: *const glyph, mask: u64) -> bool
{
  unsafe { ((*g).bits & mask) != 0 }
}

/// Assign one `bool_bf : 1` member of `glyph::bits` — C's `(GLYPH).MEMBER = VALUE` (C's
/// `bool` conversion: a nonzero VALUE is true).
#[inline(always)]
pub fn glyph_bool_set (g: *mut glyph, mask: u64, value: bool)
{
  unsafe {
    let bits = &mut (*g).bits;
    *bits = if value { *bits | mask } else { *bits & !mask };
  }
}

/// C's bit members of `glyph` as a read/write pair, because Rust has no bitfields
/// (PORTING.md 1.3): `g->type` is `glyph_bits! (type, g)`, `g->type = v` is
/// `glyph_bits! (type, g, v)`.  The second argument is a *place* of type `glyph` — C's
/// `(GLYPH)` — so the read form works on any expression of that type, exactly as C's does.
macro_rules! glyph_bits {
  (type, $g:expr) => { glyph_bits_get (&($g), GLYPH_TYPE_SHIFT, GLYPH_TYPE_WIDTH) };
  (type, $g:expr, $value:expr) =>
    { glyph_bits_set (&mut ($g), GLYPH_TYPE_SHIFT, GLYPH_TYPE_WIDTH, $value) };
  (multibyte_p, $g:expr) => { glyph_bool_p (&($g), GLYPH_MULTIBYTE_P) };
  (multibyte_p, $g:expr, $value:expr) => { glyph_bool_set (&mut ($g), GLYPH_MULTIBYTE_P, $value) };
  (left_box_line_p, $g:expr) => { glyph_bool_p (&($g), GLYPH_LEFT_BOX_LINE_P) };
  (left_box_line_p, $g:expr, $value:expr) =>
    { glyph_bool_set (&mut ($g), GLYPH_LEFT_BOX_LINE_P, $value) };
  (right_box_line_p, $g:expr) => { glyph_bool_p (&($g), GLYPH_RIGHT_BOX_LINE_P) };
  (right_box_line_p, $g:expr, $value:expr) =>
    { glyph_bool_set (&mut ($g), GLYPH_RIGHT_BOX_LINE_P, $value) };
  (overlaps_vertically_p, $g:expr) => { glyph_bool_p (&($g), GLYPH_OVERLAPS_VERTICALLY_P) };
  (overlaps_vertically_p, $g:expr, $value:expr) =>
    { glyph_bool_set (&mut ($g), GLYPH_OVERLAPS_VERTICALLY_P, $value) };
  (padding_p, $g:expr) => { glyph_bool_p (&($g), GLYPH_PADDING_P) };
  (padding_p, $g:expr, $value:expr) => { glyph_bool_set (&mut ($g), GLYPH_PADDING_P, $value) };
  (glyph_not_available_p, $g:expr) => { glyph_bool_p (&($g), GLYPH_GLYPH_NOT_AVAILABLE_P) };
  (glyph_not_available_p, $g:expr, $value:expr) =>
    { glyph_bool_set (&mut ($g), GLYPH_GLYPH_NOT_AVAILABLE_P, $value) };
  (avoid_cursor_p, $g:expr) => { glyph_bool_p (&($g), GLYPH_AVOID_CURSOR_P) };
  (avoid_cursor_p, $g:expr, $value:expr) =>
    { glyph_bool_set (&mut ($g), GLYPH_AVOID_CURSOR_P, $value) };
  (resolved_level, $g:expr) =>
    { glyph_bits_get (&($g), GLYPH_RESOLVED_LEVEL_SHIFT, GLYPH_RESOLVED_LEVEL_WIDTH) };
  (resolved_level, $g:expr, $value:expr) =>
    { glyph_bits_set (&mut ($g), GLYPH_RESOLVED_LEVEL_SHIFT, GLYPH_RESOLVED_LEVEL_WIDTH, $value) };
  (bidi_type, $g:expr) => { glyph_bits_get (&($g), GLYPH_BIDI_TYPE_SHIFT, GLYPH_BIDI_TYPE_WIDTH) };
  (bidi_type, $g:expr, $value:expr) =>
    { glyph_bits_set (&mut ($g), GLYPH_BIDI_TYPE_SHIFT, GLYPH_BIDI_TYPE_WIDTH, $value) };
  (face_id, $g:expr) => { glyph_bits_get (&($g), GLYPH_FACE_ID_SHIFT, GLYPH_FACE_ID_WIDTH) };
  (face_id, $g:expr, $value:expr) =>
    { glyph_bits_set (&mut ($g), GLYPH_FACE_ID_SHIFT, GLYPH_FACE_ID_WIDTH, $value) };
  (font_type, $g:expr) => { glyph_bits_get (&($g), GLYPH_FONT_TYPE_SHIFT, GLYPH_FONT_TYPE_WIDTH) };
  (font_type, $g:expr, $value:expr) =>
    { glyph_bits_set (&mut ($g), GLYPH_FONT_TYPE_SHIFT, GLYPH_FONT_TYPE_WIDTH, $value) };
}

/// C's `img.x`/`img.y`/`img.width`/`img.height` of `glyph_slice` (four `unsigned : 16`
/// members measured into `bits [0]`/`bits [1]`), as a read-only accessor: Rust has no
/// bitfields, so the member's name is the macro's first argument (PORTING.md 1.3).
macro_rules! glyph_slice_bits {
  (x, $slice:expr) => { ($slice).bits [0] & GLYPH_SLICE_X_MASK };
  (y, $slice:expr) => { (($slice).bits [0] >> GLYPH_SLICE_Y_SHIFT) & GLYPH_SLICE_Y_MASK };
  (width, $slice:expr) => { ($slice).bits [1] & GLYPH_SLICE_WIDTH_MASK };
  (height, $slice:expr) =>
    { (($slice).bits [1] >> GLYPH_SLICE_HEIGHT_SHIFT) & GLYPH_SLICE_HEIGHT_MASK };
}

/* Are glyph slices of glyphs *X and *Y equal?  It assumes that both
   glyphs have the same type.

   Note: for composition glyphs, we don't have to compare slice.cmp.to
   because they should be the same if and only if slice.cmp.from are
   the same.  */

// dispextern.h 648-661.  X and Y are `struct glyph *` (C's `(X)` dereferences them); the
// union members (`slice.img`, `slice.cmp`, `u` below) are read through Rust's `unsafe`
// union access, exactly as C reads them.
macro_rules! GLYPH_SLICE_EQUAL_P {
  ($x:expr, $y:expr) => {
    unsafe {
      if glyph_bits! (type, *($x)) == IMAGE_GLYPH {
        glyph_slice_bits! (x, (*($x)).slice.img) == glyph_slice_bits! (x, (*($y)).slice.img)
          && glyph_slice_bits! (y, (*($x)).slice.img) == glyph_slice_bits! (y, (*($y)).slice.img)
          && glyph_slice_bits! (width, (*($x)).slice.img)
             == glyph_slice_bits! (width, (*($y)).slice.img)
          && glyph_slice_bits! (height, (*($x)).slice.img)
             == glyph_slice_bits! (height, (*($y)).slice.img)
      } else {
        glyph_bits! (type, *($x)) != COMPOSITE_GLYPH
          || (*($x)).slice.cmp.from == (*($y)).slice.cmp.from
      }
    }
  };
}

/* Are glyphs *X and *Y displayed equal?  */

macro_rules! GLYPH_EQUAL_P {
  ($x:expr, $y:expr) => {
    unsafe {
      glyph_bits! (type, *($x)) == glyph_bits! (type, *($y))
        && (*($x)).u.val == (*($y)).u.val
        && GLYPH_SLICE_EQUAL_P! ($x, $y)
        && glyph_bits! (face_id, *($x)) == glyph_bits! (face_id, *($y))
        && (*($x)).frame == (*($y)).frame
        && glyph_bits! (padding_p, *($x)) == glyph_bits! (padding_p, *($y))
        && glyph_bits! (left_box_line_p, *($x)) == glyph_bits! (left_box_line_p, *($y))
        && glyph_bits! (right_box_line_p, *($x)) == glyph_bits! (right_box_line_p, *($y))
        && (*($x)).voffset == (*($y)).voffset
        && (*($x)).pixel_width == (*($y)).pixel_width
    }
  };
}

/* -------------------------------------------------------------------------------------------------
   dispextern.h 113-198: the per-platform display types.  C's own `#include`s of the platform
   `gui.h` headers (C 135/143/150/158/164) are carried here as `include!`s, because C reaches
   three of the six `Emacs_Cursor`s, NS's `Window` and the two `Emacs_Pixmap`-based containers
   through *this* header, not through the term header (a unit's `TERM_HEADER` include is
   `frame.h` 31-33's, and it is the second path to the same mirrors).  Everything in the block
   is gated with `have_window_system` on top of the platform's own feature, so no unit that sets
   a platform feature without the window system reaches a record it may not have (P-87 3:
   `xtask`'s `have_haiku` flip is exactly that shape).

   The six `Display_Info` arms are C 115/136/145/152/159/165; each aliases the *term* mirror's
   record (`xterm.rs` 355, `w32term.rs` 154, `nsterm.rs` 318, `haikuterm.rs` 89,
   `androidterm.rs` 86), which C's own text reaches through `TERM_HEADER` - so the alias resolves
   in a unit that has the term header, the same condition C has.  `pgtk_display_info` is the one
   that does not land: `include/pgtkterm.rs` carries no record yet and no arm on this machine can
   read it (no GTK headers - `docs/ports/include-dispextern.md`'s 3.278 4 defers the five pgtk
   records with that measurement).
   ------------------------------------------------------------------------------------------------- */

// C: dispextern.h 135: `#include "w32gui.h"` is **not** carried here, although C has the line.
// `w32gui.rs` is *inside* `xtask layout-check`'s compiled set (it is not one of the ten
// platform-exclusive mirrors that probe skips), and the materializer's dedupe is by *file* with no
// regard for the `#[cfg]` on the edge that first reaches it - so this line moved the file's one
// inclusion into a `have_ntgui` arm, and `include/msdos.rs` 349-362 (`XMenuCreate` and its
// neighbours, which spell `Display`/`Window` without a gate) lost the two names it was borrowing
// from there: `error[E0425]: cannot find type `Display` in this scope` x6 + `Window` x1,
// `error: layout probe failed to compile` (measured; at HEAD the same probe passes).  The edge
// the W32 units use is C's own from the term header - `include/w32term.rs` 49 carries
// `w32term.h` 21's `#include "w32gui.h"` - so nothing is lost by leaving this one out.
/// C: dispextern.h 143: `#include "nsgui.h"`
#[cfg (feature = "have_ns")]
include!("nsgui.rs");
/// C: dispextern.h 150: `#include "pgtkgui.h"`
#[cfg (feature = "have_pgtk")]
include!("pgtkgui.rs");
/// C: dispextern.h 158: `#include "haikugui.h"`
#[cfg (feature = "have_haiku")]
include!("haikugui.rs");
// The three conditions above are C's own (`#ifdef HAVE_NS` at 142, `#ifdef HAVE_PGTK` at 149,
// `#ifdef HAVE_HAIKU` at 157) and carry **no** `have_window_system` conjunct: a real Haiku/NS/PGTK
// configuration defines that macro too, but the conjunct is not C's text, and with it on the edge
// the `dead-arms` `have_haiku` flip puts this mirror's copy of `haikugui.rs` in a dead position -
// the mirror that *does* need the names (`include/haikuterm.rs` 31, live in that flip) then reads
// `E0425: cannot find type 'haiku'` x15 + `'Window'` x2, because the materialiser's de-duplication
// is by file and this edge wins (measured 4.684, docs/ports/comp.md's neighbours).
//
// C: dispextern.h 164: `#include "androidgui.h"` is **not** carried here, although C has the line.
// `androidgui.rs` is the one gui mirror with `include!`s of its own (`stdint.in.rs`,
// `lib/include/wchar.rs`), and `xtask`'s materializer dedupes nested `include!`s by *file*, with
// no regard for the `#[cfg]` on the edge that reaches them (measured: `build-tu src/sysdep.c`
// after the first version of this block read `// [xtask dedupe] …/lib/include/wchar.rs already
// included` at the top level, and the only `wchar.rs` copy left in the root sat inside this
// `have_android` arm - so `sysdep.rs` lost `wint_t` and eight `E0425`s appeared; `checks
// 83 unit(s), 1 failure(s)`).  The Android units do not need the edge: `androidterm.h` 23
// includes `androidgui.h` for them (`include/androidterm.rs` 41/44 carries it), which is how
// `docs/ports/include-androidgui.md`'s 3.370 ruling already resolves the `Emacs_Cursor` pair.
// The edge can land when the materializer follows a *live* path only.

/// C: dispextern.h 115: `typedef struct x_display_info Display_Info;` - C declares the *tag*
/// here and the record itself is `xterm.h`'s.  The port names an opaque type for the tag
/// (module, P-21/P-39) rather than a top-level one: a unit that includes this mirror usually
/// also includes `include/xterm.rs`, whose record is the complete one, and two top-level types
/// of one name are `E0428` there.  A unit that needs the complete record declares its own alias
/// (the `src/frame.rs` 6059 / `src/androidfns.rs` 418 precedent), which is also why the mirror's
/// alias is only for the declarations below.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
pub type Display_Info = dispextern_unit_names::x_display_info;
/// C: dispextern.h 136: `typedef struct w32_display_info Display_Info;`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
pub type Display_Info = w32_display_info;
/// C: dispextern.h 145: `typedef struct ns_display_info Display_Info;`
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
pub type Display_Info = ns_display_info;
/// C: dispextern.h 159: `typedef struct haiku_display_info Display_Info;`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
pub type Display_Info = haiku_display_info;
/// C: dispextern.h 165: `typedef struct android_display_info Display_Info;`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
pub type Display_Info = android_display_info;
// C 152: `typedef struct pgtk_display_info Display_Info;` - deferred with the pgtk records
// (`include/pgtkterm.rs` has no `pgtk_display_info` and no arm can read one here).
// C 137-139's `HAVE_NTGUI` arm (`XImage`/`HDC`) is still deferred with the unmeasured Windows
// arm; its C text is quoted in the ledger's Evidence rather than guessed at here.
/* C 116-118's X11 arm.  The deferral this used to carry ("with `include/xlib.rs`'s
   `XImage`/`Cursor`") outlived its reason: that mirror has had `Cursor` (862) and `XImage`
   (356-390's record, measured 136/8) since 3.500, and the arm needs them - `Emacs_Pix_Context`
   below (C 196) and every X11 flip read `E0425` without this line (status 3.506).  */
/// C: dispextern.h 116-118: `#ifndef USE_CAIRO` / `typedef XImage *Emacs_Pix_Container;`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
           not (feature = "use_cairo")))]
pub type Emacs_Pix_Container = *mut XImage;

/* C 122-131: the `USE_CAIRO` arm - "Minimal version of XImage".  C's record is anonymous and
   the port names it after the typedef's object (PORTING.md 1.3).  Measured with this machine's
   clang against `dispextern.h` 122-131 (the record's own text): sizeof 24, align 8, `data` at
   8, `bytes_per_line` 16, `bits_per_pixel` 20 - the same reading `docs/ports/image.md` carries,
   which is why the four assertions below are in the `use_cairo` arm.  */
/// C: dispextern.h 122-131: `typedef struct { int width, height; char *data; int bytes_per_line; int bits_per_pixel; } *Emacs_Pix_Container;`
#[cfg (feature = "use_cairo")]
#[repr(C)]
pub struct emacs_pix_container
{
  pub width: c_int,                                   // C 126
  pub height: c_int,
  pub data: *mut c_char,                              // C 127
  pub bytes_per_line: c_int,                          // C 128
  pub bits_per_pixel: c_int,                          // C 129
}
#[cfg (feature = "use_cairo")]
const _: () = core::assert! (core::mem::size_of::<emacs_pix_container> () == 24);
#[cfg (feature = "use_cairo")]
const _: () = core::assert! (core::mem::offset_of! (emacs_pix_container, data) == 8);
#[cfg (feature = "use_cairo")]
const _: () = core::assert! (core::mem::offset_of! (emacs_pix_container, bytes_per_line) == 16);
#[cfg (feature = "use_cairo")]
const _: () = core::assert! (core::mem::offset_of! (emacs_pix_container, bits_per_pixel) == 20);
/// C: dispextern.h 130: `} *Emacs_Pix_Container;`
#[cfg (feature = "use_cairo")]
pub type Emacs_Pix_Container = *mut emacs_pix_container;
/// C: dispextern.h 131: `typedef Emacs_Pix_Container Emacs_Pixmap;`
#[cfg (all (feature = "use_cairo", not (feature = "have_ns"), not (feature = "have_haiku")))]
pub type Emacs_Pixmap = Emacs_Pix_Container;

/// C: dispextern.h 146: `typedef Emacs_Pixmap Emacs_Pix_Container;` (NS's `Emacs_Pixmap` is
/// `nsgui.h` 58's)
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
pub type Emacs_Pix_Container = Emacs_Pixmap;
/// C: dispextern.h 160: `typedef Emacs_Pixmap Emacs_Pix_Container;` (`haikugui.h` 58)
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
pub type Emacs_Pix_Container = Emacs_Pixmap;
/// C: dispextern.h 166: `typedef struct android_image *Emacs_Pix_Container;` (`androidgui.h` 883)
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
pub type Emacs_Pix_Container = *mut android_image;

/* C 182-198: the `HAVE_WINDOW_SYSTEM` block whose `# ifndef PIX_CONTAINER_TO_CONTEXT` gives the
   default pair.  `PIX_CONTAINER_TO_CONTEXT` is a macro C's platform headers override (C 139 for
   w32); the port spells the default as a macro over the container, and the arms that override it
   (w32's `HDC`) are deferred with the Windows arm.  */
// C 184-190's `WINDOW_HANDLE_UINTPTR` default (`((uintptr_t) {(h)})`) is deliberately **not**
// spelled here: it is a macro, and the platform mirrors already own the name with the same cast
// (`haikugui.rs` 53, `pgtkgui.rs` 116), so a second `macro_rules!` of one name is `E0659`
// ambiguity in `layout-check`'s single crate - the same reason this file gives for
// `NativeRectangle` above.  The `#ifndef` default exists in C for platforms whose header does
// not define it, and none of the five the port has needs it.
/// C: dispextern.h 196: `typedef Emacs_Pix_Container Emacs_Pix_Context;`
/// The `have_pgtk` arm is this line's too (2026-10-06): C 149-155's pgtk block defines
/// `XImagePtr`/`XImagePtr_or_DC` and **not** `Emacs_Pix_Context`, and no pgtk header defines
/// `PIX_CONTAINER_TO_CONTEXT` (only `w32gui.h`, C 139, does), so C 195's `#ifndef` holds and
/// a pgtk build reads this default - measured by that arm's first consumer,
/// `src/pgtkterm.rs`'s `pgtk_setup_relief_colors` (C 1835/1836), whose probe named the
/// missing name before this arm landed.
#[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
pub type Emacs_Pix_Context = Emacs_Pix_Container;
/// C: dispextern.h 197: `#  define PIX_CONTAINER_TO_CONTEXT(c) (c)`
#[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
macro_rules! PIX_CONTAINER_TO_CONTEXT {
  ($c:expr) => { $c };
}


/* Are character codes, faces, padding_ps of glyphs *X and *Y equal?  */

macro_rules! GLYPH_CHAR_AND_FACE_EQUAL_P {
  ($x:expr, $y:expr) => {
    unsafe {
      (*($x)).u.ch == (*($y)).u.ch
        && glyph_bits! (face_id, *($x)) == glyph_bits! (face_id, *($y))
        && (*($x)).frame == (*($y)).frame
        && glyph_bits! (padding_p, *($x)) == glyph_bits! (padding_p, *($y))
    }
  };
}

// dispextern.h 697-724 (`SET_CHAR_GLYPH`, `SET_CHAR_GLYPH_FROM_GLYPH` and
// `SET_GLYPH_FROM_CHAR_GLYPH`) — not mirrored on this target yet: each C body ends in
// `while (false)`, the construct the port records in `docs/ports/macro-body-notes.tsv`
// (the `CHECK_CODING_SYSTEM`/`SAFE_ALLOCA_LISP` shape, PORTING.md 2.1).  They land with
// that ledger row, together with their first display user (dispnew.c's glyph filling).

macro_rules! GLYPH_INVALID_P { ($glyph:expr) => { GLYPH_CHAR ($glyph) < 0 }; }   // dispextern.h 727

/* Is GLYPH a padding glyph?  */

macro_rules! CHAR_GLYPH_PADDING_P {
  ($glyph:expr) => { glyph_bits! (padding_p, $glyph) };                          // dispextern.h 729
}

/* Default value of the glyph font_type field.  */
pub const FONT_TYPE_UNKNOWN: c_int = 0;                 // dispextern.h 644

/* Glyph Pool.

   Glyph memory for frame-based redisplay is allocated from the heap
   in one vector kept in a glyph pool structure which is stored with
   the frame.  The size of the vector is made large enough to cover
   all windows on the frame.  */

// dispextern.h 751-761.  Measured: sizeof 24, alignof 8 — `glyphs` 0, `nglyphs` 8,
// `nrows` 16, `ncolumns` 20.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph_pool
{
  /* Vector of glyphs allocated from the heap.  */
  pub glyphs: *mut glyph,               // dispextern.h 754
  /* Allocated size of `glyphs'.  */
  pub nglyphs: ptrdiff_t,               // 757
  /* Number of rows and columns in a matrix.  */
  pub nrows: c_int,                     // 760
  pub ncolumns: c_int,
}

/* Glyph Matrix.

   Three kinds of glyph matrices exist: frame glyph matrices (used only in the update phase
   of terminal redisplay), window matrices that are sub-matrices of a frame matrix (sharing
   a glyph pool), and free-standing window matrices that manage their own glyph storage
   (window-based redisplay).  */

// dispextern.h 795-859.  Measured: sizeof 104, alignof 8 — `pool` 0, `rows` 8,
// `rows_allocated` 16, `nrows` 24, `matrix_x`/`matrix_y` 28/32, `matrix_w`/`matrix_h`
// 36/40, `window_pixel_left`/`window_pixel_top` 44/48, `window_height`/`window_width`
// 52/56, `window_vscroll` 60, `left_margin_glyphs`/`right_margin_glyphs` 64/68, the three
// `bool_bf : 1` members in byte 72 (`bits`), `buffer` 80, `begv`/`zv` 88/96.
//
// `#ifdef GLYPH_DEBUG`'s `char method[512]` (847-850) is dead on this target; it stays
// behind the feature (P-51), which is what keeps `buffer` at 80 and sizeof at 104.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph_matrix
{
  /* The pool from which glyph memory is allocated, if any.  Null for frame matrices and
     for window matrices managing their own storage.  */
  pub pool: *mut glyph_pool,            // dispextern.h 800
  /* Vector of glyph row structures.  The row at nrows - 1 is reserved for the mode line.  */
  pub rows: *mut glyph_row,             // 804
  /* Number of elements allocated for the vector rows above.  */
  pub rows_allocated: ptrdiff_t,        // 807
  /* The number of rows used by the window if all lines were displayed with the smallest
     possible character height.  */
  pub nrows: c_int,                     // 811
  /* Origin within the frame matrix if this is a window matrix on a frame having a frame
     matrix.  Both values are zero for window-based redisplay.  */
  pub matrix_x: c_int,                  // 816
  pub matrix_y: c_int,
  /* Width and height of the matrix in columns and rows.  */
  pub matrix_w: c_int,                  // 819
  pub matrix_h: c_int,
  /* For a window matrix of window W: W's pixel_left/pixel_top, and W's width and height
     as returned by window_box, plus the value of W->vscroll when the matrix was last
     adjusted.  Only set for window-based redisplay.  */
  pub window_pixel_left: c_int,         // 827
  pub window_pixel_top: c_int,
  pub window_height: c_int,             // 828
  pub window_width: c_int,
  pub window_vscroll: c_int,            // 829
  /* Number of glyphs reserved for left and right marginal areas when the matrix was last
     adjusted.  */
  pub left_margin_glyphs: c_int,        // 833
  pub right_margin_glyphs: c_int,
  /* no_scrolling_p:1 | tab_line_p:1 | header_line_p:1 — three `bool_bf : 1` members, all
     in the byte at offset 72 (measured).  The port stores the byte and names the members
     through `glyph_matrix_bits!` (PORTING.md 1.3).  */
  pub bits: u8,                         // 838-845
  #[cfg (feature = "glyph_debug")]
  /* A string identifying the method used to display the matrix.  */
  pub method: [c_char; 512],            // 848-849
  /* The buffer this matrix displays.  Set in mark_window_display_accurate_1.  */
  pub buffer: *mut c_void,              /// C: `struct buffer *buffer;` (dispextern.h 854)
  /* Values of BEGV and ZV as of last redisplay.  Set in mark_window_display_accurate_1.  */
  pub begv: ptrdiff_t,                  // 858
  pub zv: ptrdiff_t,
}

// dispextern.h 838-845, measured 72:0-72:2 (one byte).
pub const GLYPH_MATRIX_NO_SCROLLING_P: u8 = 1 << 0;     // dispextern.h 838
pub const GLYPH_MATRIX_TAB_LINE_P: u8 = 1 << 1;         // 841
pub const GLYPH_MATRIX_HEADER_LINE_P: u8 = 1 << 2;      // 845

/// Read one `bool_bf : 1` member of `glyph_matrix::bits` — C's `(MATRIX)->MEMBER`.
#[inline(always)]
pub fn glyph_matrix_bool_p (matrix: *const glyph_matrix, mask: u8) -> bool
{
  unsafe { ((*matrix).bits & mask) != 0 }
}

/// Assign one `bool_bf : 1` member of `glyph_matrix::bits` — C's `(MATRIX)->MEMBER = VALUE`.
#[inline(always)]
pub fn glyph_matrix_bool_set (matrix: *mut glyph_matrix, mask: u8, value: bool)
{
  unsafe {
    let bits = &mut (*matrix).bits;
    *bits = if value { *bits | mask } else { *bits & !mask };
  }
}

/// C's bit members of `glyph_matrix` as a read/write pair (PORTING.md 1.3): `m->tab_line_p`
/// is `glyph_matrix_bits! (tab_line_p, m)`, `m->tab_line_p = v` is
/// `glyph_matrix_bits! (tab_line_p, m, v)`.  The second argument is C's `(MATRIX)`.
macro_rules! glyph_matrix_bits {
  (no_scrolling_p, $matrix:expr) => { glyph_matrix_bool_p ($matrix, GLYPH_MATRIX_NO_SCROLLING_P) };
  (no_scrolling_p, $matrix:expr, $value:expr) =>
    { glyph_matrix_bool_set ($matrix, GLYPH_MATRIX_NO_SCROLLING_P, $value) };
  (tab_line_p, $matrix:expr) => { glyph_matrix_bool_p ($matrix, GLYPH_MATRIX_TAB_LINE_P) };
  (tab_line_p, $matrix:expr, $value:expr) =>
    { glyph_matrix_bool_set ($matrix, GLYPH_MATRIX_TAB_LINE_P, $value) };
  (header_line_p, $matrix:expr) => { glyph_matrix_bool_p ($matrix, GLYPH_MATRIX_HEADER_LINE_P) };
  (header_line_p, $matrix:expr, $value:expr) =>
    { glyph_matrix_bool_set ($matrix, GLYPH_MATRIX_HEADER_LINE_P, $value) };
}

/* Rows of glyphs in a windows or frame glyph matrix.

   Each row is partitioned into three areas (left margin, text, right margin); the start
   and end of each area is recorded in `glyphs` and `used`, whose last element is the end
   of the whole row.  Rows in frame matrices reference glyph memory allocated in a frame
   glyph pool; rows in window matrices that have no frame matrix point to glyphs allocated
   from the heap.  The layout of the first four members is important — see
   clear_glyph_row and copy_row_except_pointers.  */

// dispextern.h 918-1154.  Measured: sizeof 256, alignof 8 — `glyphs` 0 (`short*[4]`),
// `used` 32 (`short[4]`), `hash` 40, `x`/`y` 44/48, `pixel_width` 52, `ascent`/`height`
// 56/60, `phys_ascent`/`phys_height` 64/68, `visible_height` 72, `extra_line_spacing` 76,
// `extra_line_spacing_above` 80, `start` 88, `end` 136, `minpos` 184, `maxpos` 200,
// `overlay_arrow_bitmap` 216, the fringe members in 220-248 (`bits`/`right_fringe_offset`/
// `bools`) and `continuation_lines_width` 252.
//
// The fringe members are C's bitfields, measured one by one (bytes of the row):
//
//   220:0-15   left_user_fringe_bitmap : 16        245:0    fringe_bitmap_periodic_p : 1
//   222:0-15   right_user_fringe_bitmap : 16       245:1    redraw_fringe_bitmaps_p : 1
//   224:0-15   left_fringe_bitmap : 16             245:2    enabled_p : 1
//   226:0-15   right_fringe_bitmap : 16            245:3    truncated_on_left_p : 1
//   228:0-19   left_user_fringe_face_id : 20       245:4    truncated_on_right_p : 1
//   232:0-19   right_user_fringe_face_id : 20      245:5    continued_p : 1
//   236:0-19   left_fringe_face_id : 20            245:6    displays_text_p : 1
//   240:0-19   right_fringe_face_id : 20           245:7    ends_at_zv_p : 1
//   242:4-11   left_fringe_offset : 8 (signed)     246:0-7  fill_line_p … ends_in_middle_of_char_p
//   244:0-7    right_fringe_offset : 8 (signed)    247:0-7  starts_in_middle_of_char_p … indicate_bob_p
//   245:0-248:4                        (the 29    248:0-4  indicate_top_line_p … stipple_p
//                                       `bool_bf : 1` members)
//
// so the port stores bytes 220-243 as `bits: [u32; 6]` (the six 4-byte allocation units, in
// order), byte 244 as a plain `right_fringe_offset: i8` (an 8-bit signed bitfield is an
// `i8`) and the 29-member run at 245-248 as `bools: [u8; 4]`; `glyph_row_bits!` names every
// member.  `#ifdef HAVE_WINDOW_SYSTEM`'s `clip` (1149-1153) is dead on this target and
// stays behind the feature (P-51).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph_row
{
  /* Pointers to beginnings of areas.  The end of an area A is found at A + 1 in the
     vector; the last element is the end of the whole row.  */
  pub glyphs: [*mut glyph; 4],          // dispextern.h 934 (`1 + LAST_AREA`)
  /* Number of glyphs actually filled in areas.  */
  pub used: [c_short; 4],               // 938 (`1 + LAST_AREA`, sized for offset maths)
  /* Hash code, available as soon as the row is constructed (display_line).  */
  pub hash: c_uint,                     // 942
  /* Window-relative x and y of the row's top-left corner; a negative y means abs(y)
     pixels are invisible at the top, a negative x the same for the first text glyph.  */
  pub x: c_int,                         // 950
  pub y: c_int,
  /* Width of the row in pixels, without the face extension at the end of the row and
     without the tty truncation/continuation glyphs.  */
  pub pixel_width: c_int,               // 955
  /* Logical ascent and height of this line (zero/one on terminal frames).  */
  pub ascent: c_int,                    // 959
  pub height: c_int,
  /* Physical ascent and height: extra over `ascent`/`height` means the row overlaps the
     row above/below it.  */
  pub phys_ascent: c_int,               // 964
  pub phys_height: c_int,
  /* Portion of the row that is visible (1 on tty frames, < 0 if completely invisible).  */
  pub visible_height: c_int,            // 969
  /* Extra line spacing added after this row.  */
  pub extra_line_spacing: c_int,        // 973
  /* Part of extra_line_spacing that goes above the line.  */
  pub extra_line_spacing_above: c_int,  // 976
  /* First and last text position of this row (see the header's own commentary on the
     bidi cases); `start`/`end` are the positions the display started/ended at, and
     `minpos`/`maxpos` the smallest and largest positions that contributed glyphs.  */
  pub start: display_pos,               // 984
  pub end: display_pos,                 // 993
  pub minpos: text_pos,                 // 1001
  pub maxpos: text_pos,
  /* Non-zero means the overlay arrow bitmap is on this line (-1 = the default bitmap,
     otherwise the fringe bitmap number).  */
  pub overlay_arrow_bitmap: c_int,      // 1006
  /* The fringe bit members measured above, in their allocation units: `bits [0]` is
     bytes 220-223, `bits [1]` 224-227, `bits [2]` 228-231, `bits [3]` 232-235,
     `bits [4]` 236-239 and `bits [5]` 240-243 (`right_fringe_face_id` plus the signed
     `left_fringe_offset` in bits 20-27).  */
  pub bits: [u32; 6],                   // 1009-1037
  /* 244:0-7, `signed right_fringe_offset : FRINGE_HEIGHT_BITS` — an eight-bit signed
     bitfield, i.e. an `i8`.  */
  pub right_fringe_offset: i8,          // 1037
  /* The run of 29 `bool_bf : 1` members at 245:0-248:4, one byte per four bytes'
     worth of members (`bools [0]` is byte 245, `bools [3]` byte 248).  */
  pub bools: [u8; 4],                   // 1040-1145
  #[cfg (feature = "have_window_system")]
  /* Non-NULL means the current clipping area, temporarily set while exposing a region
     (coordinates are frame-relative).  */
  pub clip: *const Emacs_Rectangle,     // 1152
  /* Continuation lines width at the start of the row.  */
  pub continuation_lines_width: c_int,  // 1147
}

/// Read one member of `glyph_row::bits` (a 4-byte allocation unit at bytes 220-243) —
/// C's `(ROW)->MEMBER`; a signed member (the two fringe offsets) is sign-extended.
#[inline(always)]
pub fn glyph_row_bits_get (row: *const glyph_row, unit: usize, shift: u32, width: u32,
                           signed_p: bool) -> c_int
{
  let raw = unsafe { ((*row).bits [unit] >> shift) & glyph_row_bits_mask (width) };
  if signed_p { ((raw as i32) << (32 - width)) >> (32 - width) } else { raw as i32 }
}

/// Assign one member of `glyph_row::bits` — C's `(ROW)->MEMBER = VALUE`: only the member's
/// bits change, and VALUE is evaluated once.
#[inline(always)]
pub fn glyph_row_bits_set (row: *mut glyph_row, unit: usize, shift: u32, width: u32,
                           value: c_int)
{
  let mask = glyph_row_bits_mask (width) << shift;
  unsafe {
    let bits = &mut (*row).bits [unit];
    *bits = (*bits & !mask) | ((((value as u32) & glyph_row_bits_mask (width)) << shift) & mask);
  }
}

/// The `width`-bit mask of a `glyph_row::bits` member.
#[inline(always)]
pub fn glyph_row_bits_mask (width: u32) -> u32
{
  if width >= 32 { u32::MAX } else { (1u32 << width) - 1 }
}

/// Read one of `glyph_row`'s 29 `bool_bf : 1` members — C's `(ROW)->MEMBER`, stored in
/// `bools [BYTE]` bit `SHIFT`.
#[inline(always)]
pub fn glyph_row_bool_p (row: *const glyph_row, byte: usize, shift: u32) -> bool
{
  unsafe { ((*row).bools [byte] >> shift) & 1 != 0 }
}

/// Assign one of `glyph_row`'s 29 `bool_bf : 1` members — C's `(ROW)->MEMBER = VALUE`,
/// which evaluates VALUE once.
#[inline(always)]
pub fn glyph_row_bool_set (row: *mut glyph_row, byte: usize, shift: u32, value: bool)
{
  unsafe {
    let b = &mut (*row).bools [byte];
    *b = if value { *b | (1 << shift) } else { *b & !(1 << shift) };
  }
}

/// C's bit members of `glyph_row` as a read/write pair, because Rust has no bitfields
/// (PORTING.md 1.3): `row->enabled_p` is `glyph_row_bits! (enabled_p, row)`, and
/// `row->enabled_p = v` is `glyph_row_bits! (enabled_p, row, v)`.  The second argument is
/// C's `(ROW)` (a `*mut glyph_row`); every arm carries the member's measured position —
/// `bools [BYTE]` bit `SHIFT`, or `bits [UNIT]` bits `SHIFT..SHIFT+WIDTH`.  The two fringe
/// offsets are signed, everything else unsigned (the C declarations at 1009-1145).
macro_rules! glyph_row_bits {
  // bits [0], bytes 220-223: the two 16-bit fringe bitmap numbers.
  (left_user_fringe_bitmap, $row:expr) => { glyph_row_bits_get ($row, 0, 0, 16, false) };
  (left_user_fringe_bitmap, $row:expr, $value:expr) =>
    { glyph_row_bits_set ($row, 0, 0, 16, $value) };
  (right_user_fringe_bitmap, $row:expr) => { glyph_row_bits_get ($row, 0, 16, 16, false) };
  (right_user_fringe_bitmap, $row:expr, $value:expr) =>
    { glyph_row_bits_set ($row, 0, 16, 16, $value) };
  // bits [1], bytes 224-227.
  (left_fringe_bitmap, $row:expr) => { glyph_row_bits_get ($row, 1, 0, 16, false) };
  (left_fringe_bitmap, $row:expr, $value:expr) =>
    { glyph_row_bits_set ($row, 1, 0, 16, $value) };
  (right_fringe_bitmap, $row:expr) => { glyph_row_bits_get ($row, 1, 16, 16, false) };
  (right_fringe_bitmap, $row:expr, $value:expr) =>
    { glyph_row_bits_set ($row, 1, 16, 16, $value) };
  // bits [2]..[5], bytes 228-243: one 20-bit face id per unit, and unit 5's signed
  // eight-bit left_fringe_offset in bits 20-27.
  (left_user_fringe_face_id, $row:expr) => { glyph_row_bits_get ($row, 2, 0, 20, false) };
  (left_user_fringe_face_id, $row:expr, $value:expr) =>
    { glyph_row_bits_set ($row, 2, 0, 20, $value) };
  (right_user_fringe_face_id, $row:expr) => { glyph_row_bits_get ($row, 3, 0, 20, false) };
  (right_user_fringe_face_id, $row:expr, $value:expr) =>
    { glyph_row_bits_set ($row, 3, 0, 20, $value) };
  (left_fringe_face_id, $row:expr) => { glyph_row_bits_get ($row, 4, 0, 20, false) };
  (left_fringe_face_id, $row:expr, $value:expr) =>
    { glyph_row_bits_set ($row, 4, 0, 20, $value) };
  (right_fringe_face_id, $row:expr) => { glyph_row_bits_get ($row, 5, 0, 20, false) };
  (right_fringe_face_id, $row:expr, $value:expr) =>
    { glyph_row_bits_set ($row, 5, 0, 20, $value) };
  (left_fringe_offset, $row:expr) => { glyph_row_bits_get ($row, 5, 20, 8, true) };
  (left_fringe_offset, $row:expr, $value:expr) =>
    { glyph_row_bits_set ($row, 5, 20, 8, $value) };
  // bools [0], byte 245.
  (fringe_bitmap_periodic_p, $row:expr) => { glyph_row_bool_p ($row, 0, 0) };
  (fringe_bitmap_periodic_p, $row:expr, $value:expr) =>
    { glyph_row_bool_set ($row, 0, 0, $value) };
  (redraw_fringe_bitmaps_p, $row:expr) => { glyph_row_bool_p ($row, 0, 1) };
  (redraw_fringe_bitmaps_p, $row:expr, $value:expr) =>
    { glyph_row_bool_set ($row, 0, 1, $value) };
  (enabled_p, $row:expr) => { glyph_row_bool_p ($row, 0, 2) };
  (enabled_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 0, 2, $value) };
  (truncated_on_left_p, $row:expr) => { glyph_row_bool_p ($row, 0, 3) };
  (truncated_on_left_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 0, 3, $value) };
  (truncated_on_right_p, $row:expr) => { glyph_row_bool_p ($row, 0, 4) };
  (truncated_on_right_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 0, 4, $value) };
  (continued_p, $row:expr) => { glyph_row_bool_p ($row, 0, 5) };
  (continued_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 0, 5, $value) };
  (displays_text_p, $row:expr) => { glyph_row_bool_p ($row, 0, 6) };
  (displays_text_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 0, 6, $value) };
  (ends_at_zv_p, $row:expr) => { glyph_row_bool_p ($row, 0, 7) };
  (ends_at_zv_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 0, 7, $value) };
  // bools [1], byte 246.
  (fill_line_p, $row:expr) => { glyph_row_bool_p ($row, 1, 0) };
  (fill_line_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 1, 0, $value) };
  (indicate_empty_line_p, $row:expr) => { glyph_row_bool_p ($row, 1, 1) };
  (indicate_empty_line_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 1, 1, $value) };
  (contains_overlapping_glyphs_p, $row:expr) => { glyph_row_bool_p ($row, 1, 2) };
  (contains_overlapping_glyphs_p, $row:expr, $value:expr) =>
    { glyph_row_bool_set ($row, 1, 2, $value) };
  (full_width_p, $row:expr) => { glyph_row_bool_p ($row, 1, 3) };
  (full_width_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 1, 3, $value) };
  (mode_line_p, $row:expr) => { glyph_row_bool_p ($row, 1, 4) };
  (mode_line_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 1, 4, $value) };
  (tab_line_p, $row:expr) => { glyph_row_bool_p ($row, 1, 5) };
  (tab_line_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 1, 5, $value) };
  (overlapped_p, $row:expr) => { glyph_row_bool_p ($row, 1, 6) };
  (overlapped_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 1, 6, $value) };
  (ends_in_middle_of_char_p, $row:expr) => { glyph_row_bool_p ($row, 1, 7) };
  (ends_in_middle_of_char_p, $row:expr, $value:expr) =>
    { glyph_row_bool_set ($row, 1, 7, $value) };
  // bools [2], byte 247.
  (starts_in_middle_of_char_p, $row:expr) => { glyph_row_bool_p ($row, 2, 0) };
  (starts_in_middle_of_char_p, $row:expr, $value:expr) =>
    { glyph_row_bool_set ($row, 2, 0, $value) };
  (overlapping_p, $row:expr) => { glyph_row_bool_p ($row, 2, 1) };
  (overlapping_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 2, 1, $value) };
  (mouse_face_p, $row:expr) => { glyph_row_bool_p ($row, 2, 2) };
  (mouse_face_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 2, 2, $value) };
  (ends_in_newline_from_string_p, $row:expr) => { glyph_row_bool_p ($row, 2, 3) };
  (ends_in_newline_from_string_p, $row:expr, $value:expr) =>
    { glyph_row_bool_set ($row, 2, 3, $value) };
  (exact_window_width_line_p, $row:expr) => { glyph_row_bool_p ($row, 2, 4) };
  (exact_window_width_line_p, $row:expr, $value:expr) =>
    { glyph_row_bool_set ($row, 2, 4, $value) };
  (cursor_in_fringe_p, $row:expr) => { glyph_row_bool_p ($row, 2, 5) };
  (cursor_in_fringe_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 2, 5, $value) };
  (ends_in_ellipsis_p, $row:expr) => { glyph_row_bool_p ($row, 2, 6) };
  (ends_in_ellipsis_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 2, 6, $value) };
  (indicate_bob_p, $row:expr) => { glyph_row_bool_p ($row, 2, 7) };
  (indicate_bob_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 2, 7, $value) };
  // bools [3], byte 248.
  (indicate_top_line_p, $row:expr) => { glyph_row_bool_p ($row, 3, 0) };
  (indicate_top_line_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 3, 0, $value) };
  (indicate_eob_p, $row:expr) => { glyph_row_bool_p ($row, 3, 1) };
  (indicate_eob_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 3, 1, $value) };
  (indicate_bottom_line_p, $row:expr) => { glyph_row_bool_p ($row, 3, 2) };
  (indicate_bottom_line_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 3, 2, $value) };
  (reversed_p, $row:expr) => { glyph_row_bool_p ($row, 3, 3) };
  (reversed_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 3, 3, $value) };
  (stipple_p, $row:expr) => { glyph_row_bool_p ($row, 3, 4) };
  (stipple_p, $row:expr, $value:expr) => { glyph_row_bool_set ($row, 3, 4, $value) };
}

/* Get a pointer to row number ROW in matrix MATRIX.  If GLYPH_DEBUG is defined, the
   function matrix_row checks that we don't try to access rows that are out of bounds.  */

// dispextern.h 1163-1170: GLYPH_DEBUG is undef on this target, so C compiles the pointer
// sum (the dead `matrix_row` arm's text is named in the ledger, not repeated here — the
// `CHECK_MATRIX` shape above).  `rows + ROW` is Rust's `add`; ROW is a C int index.
macro_rules! MATRIX_ROW {
  ($matrix:expr, $row:expr) => { (*($matrix)).rows.add (($row) as usize) };
}

/* Return a pointer to the row reserved for the mode line in MATRIX.  Row
   MATRIX->nrows - 1 is always reserved for the mode line.  */
macro_rules! MATRIX_MODE_LINE_ROW {
  ($matrix:expr) => { (*($matrix)).rows.offset ((*($matrix)).nrows as isize - 1) };
}

/* Return a pointer to the row reserved for the tab line in MATRIX.  This is always the
   first row in MATRIX because that's the only way that works in frame-based redisplay.  */
macro_rules! MATRIX_TAB_LINE_ROW {
  ($matrix:expr) => { (*($matrix)).rows };
}

/* Return a pointer to the row reserved for the header line in MATRIX.  This is always the
   second row in MATRIX because that's the only way that works in frame-based redisplay.  */
macro_rules! MATRIX_HEADER_LINE_ROW {
  ($matrix:expr) => { {
    let matrix = $matrix;
    if glyph_matrix_bits! (tab_line_p, matrix) { (*matrix).rows.add (1) } else { (*matrix).rows }
  } };
}

/* Return the height of the mode line in glyph matrix MATRIX, or zero if not known.  This macro
   is called under circumstances where MATRIX might not have been allocated yet.  */
macro_rules! MATRIX_MODE_LINE_HEIGHT {
  ($m:expr) => { {
    let matrix = $m as *mut glyph_matrix;
    if !matrix.is_null () && !(*matrix).rows.is_null ()
      { (*(MATRIX_MODE_LINE_ROW! (matrix))).height } else { 0 }
  } };
}

/* Return the height of the header line in glyph matrix MATRIX, or zero if not known.  */
macro_rules! MATRIX_HEADER_LINE_HEIGHT {
  ($m:expr) => { {
    let matrix = $m as *mut glyph_matrix;
    if !matrix.is_null () && !(*matrix).rows.is_null ()
      { (*(MATRIX_HEADER_LINE_ROW! (matrix))).height } else { 0 }
  } };
}

/* Return the height of the tab line in glyph matrix MATRIX, or zero if not known.  */
macro_rules! MATRIX_TAB_LINE_HEIGHT {
  ($m:expr) => { {
    let matrix = $m as *mut glyph_matrix;
    if !matrix.is_null () && !(*matrix).rows.is_null ()
      { (*(MATRIX_TAB_LINE_ROW! (matrix))).height } else { 0 }
  } };
}

/* Return the desired face id for the mode line of a window, depending on whether the window is
   selected or not, or if the window is the scrolling window for the currently active minibuffer
   window.  C's three arguments are SELW, MBW and SCRW.  */
macro_rules! CURRENT_MODE_LINE_ACTIVE_FACE_ID_3 {
  ($selw:expr, $mbw:expr, $scrw:expr) => {
    if !mode_line_in_non_selected_windows
       || ($selw) == XWINDOW (selected_window)
       || (minibuf_level > 0
	   && !NILP (minibuf_selected_window)
	   && ($mbw) == XWINDOW (minibuf_window)
	   && ($scrw) == XWINDOW (minibuf_selected_window))
      { MODE_LINE_ACTIVE_FACE_ID } else { MODE_LINE_INACTIVE_FACE_ID }
  };
}

/* Return the desired face id for the mode line of window W.  */
macro_rules! CURRENT_MODE_LINE_ACTIVE_FACE_ID {
  ($w:expr) => { CURRENT_MODE_LINE_ACTIVE_FACE_ID_3! ($w, XWINDOW (selected_window), $w) };
}

/* Return the current height of the mode line of window W.  If not known from
   W->mode_line_height, look at W's current glyph matrix, or return a default based on the
   height of the font of the face `mode-line'.  */
macro_rules! CURRENT_MODE_LINE_HEIGHT {
  ($w:expr) => {
    if (*($w)).mode_line_height >= 0
      { (*($w)).mode_line_height }
    else
      { (*($w)).mode_line_height =
	  if MATRIX_MODE_LINE_HEIGHT! ((*($w)).current_matrix) != 0
	    { MATRIX_MODE_LINE_HEIGHT! ((*($w)).current_matrix) }
	    else
	    { estimate_mode_line_height (XFRAME ((*($w)).frame),
					 CURRENT_MODE_LINE_ACTIVE_FACE_ID! ($w)) };
	(*($w)).mode_line_height }
  };
}

/* The same three for the header line.  */
macro_rules! CURRENT_HEADER_LINE_ACTIVE_FACE_ID_3 {
  ($selw:expr, $mbw:expr, $scrw:expr) => {
    if !mode_line_in_non_selected_windows
       || ($selw) == XWINDOW (selected_window)
       || (minibuf_level > 0
	   && !NILP (minibuf_selected_window)
	   && ($mbw) == XWINDOW (minibuf_window)
	   && ($scrw) == XWINDOW (minibuf_selected_window))
      { HEADER_LINE_ACTIVE_FACE_ID } else { HEADER_LINE_INACTIVE_FACE_ID }
  };
}
macro_rules! CURRENT_HEADER_LINE_ACTIVE_FACE_ID {
  ($w:expr) => { CURRENT_HEADER_LINE_ACTIVE_FACE_ID_3! ($w, XWINDOW (selected_window), $w) };
}
macro_rules! CURRENT_HEADER_LINE_HEIGHT {
  ($w:expr) => {
    if (*($w)).header_line_height >= 0
      { (*($w)).header_line_height }
    else
      { (*($w)).header_line_height =
	  if MATRIX_HEADER_LINE_HEIGHT! ((*($w)).current_matrix) != 0
	    { MATRIX_HEADER_LINE_HEIGHT! ((*($w)).current_matrix) }
	    else
	    { estimate_mode_line_height (XFRAME ((*($w)).frame),
					 CURRENT_HEADER_LINE_ACTIVE_FACE_ID! ($w)) };
	(*($w)).header_line_height }
  };
}

/* The same three for the tab line.  */
macro_rules! CURRENT_TAB_LINE_ACTIVE_FACE_ID_3 {
  ($selw:expr, $mbw:expr, $scrw:expr) => {
    if !mode_line_in_non_selected_windows
       || ($selw) == XWINDOW (selected_window)
       || (minibuf_level > 0
	   && !NILP (minibuf_selected_window)
	   && ($mbw) == XWINDOW (minibuf_window)
	   && ($scrw) == XWINDOW (minibuf_selected_window))
      { TAB_LINE_ACTIVE_FACE_ID } else { TAB_LINE_INACTIVE_FACE_ID }
  };
}
macro_rules! CURRENT_TAB_LINE_ACTIVE_FACE_ID {
  ($w:expr) => { CURRENT_TAB_LINE_ACTIVE_FACE_ID_3! ($w, XWINDOW (selected_window), $w) };
}
macro_rules! CURRENT_TAB_LINE_HEIGHT {
  ($w:expr) => {
    if (*($w)).tab_line_height >= 0
      { (*($w)).tab_line_height }
    else
      { (*($w)).tab_line_height =
	  if MATRIX_TAB_LINE_HEIGHT! ((*($w)).current_matrix) != 0
	    { MATRIX_TAB_LINE_HEIGHT! ((*($w)).current_matrix) }
	    else
	    { estimate_mode_line_height (XFRAME ((*($w)).frame),
					 CURRENT_TAB_LINE_ACTIVE_FACE_ID! ($w)) };
	(*($w)).tab_line_height }
  };
}
/* Return a pointer to first row in MATRIX used for text display.  */
macro_rules! MATRIX_FIRST_TEXT_ROW {
  ($matrix:expr) => { {
    let matrix = $matrix;
    let rows = (*matrix).rows;
    if glyph_row_bits! (mode_line_p, rows) {
      if glyph_row_bits! (mode_line_p, rows.add (1)) { rows.add (2) } else { rows.add (1) }
    } else {
      rows
    }
  } };
}

/* Return a pointer to the first glyph in the text area of a row.  MATRIX is the glyph
   matrix accessed, and ROW is the row index in MATRIX.  */
macro_rules! MATRIX_ROW_GLYPH_START {
  ($matrix:expr, $row:expr) => { (*MATRIX_ROW! ($matrix, $row)).glyphs [TEXT_AREA as usize] };
}

/* Return the number of used glyphs in the text area of a row.  */
macro_rules! MATRIX_ROW_USED {
  ($matrix:expr, $row:expr) => { (*MATRIX_ROW! ($matrix, $row)).used [TEXT_AREA as usize] };
}

/* Return the character/ byte position at which the display of ROW starts.  BIDI Note: this
   is the smallest character/byte position among characters in ROW, i.e. the first
   logical-order character displayed by ROW, which is not necessarily the smallest
   horizontal position.  */
macro_rules! MATRIX_ROW_START_CHARPOS { ($row:expr) => { (*($row)).minpos.charpos }; }
macro_rules! MATRIX_ROW_START_BYTEPOS { ($row:expr) => { (*($row)).minpos.bytepos }; }

/* Return the character/ byte position at which ROW ends.  BIDI Note: this is the largest
   character/byte position among characters in ROW, i.e. the last logical-order character
   displayed by ROW, which is not necessarily the largest horizontal position.  */
macro_rules! MATRIX_ROW_END_CHARPOS { ($row:expr) => { (*($row)).maxpos.charpos }; }
macro_rules! MATRIX_ROW_END_BYTEPOS { ($row:expr) => { (*($row)).maxpos.bytepos }; }

/* Return the vertical position of ROW in MATRIX.  */
macro_rules! MATRIX_ROW_VPOS {
  // C's `((ROW) - (MATRIX)->rows)` is a *pointer* difference; the mirror subtracted the
  // pointees (`(*($row)).offset_from (…)`), which is not the same expression (status 3.84).
  ($row:expr, $matrix:expr) => { ($row).offset_from ((*($matrix)).rows) };
}

/* Return the last glyph row + 1 in MATRIX on window W reserved for text.  If W has a mode
   line, the last row in the matrix is reserved for it.  */
// `window_wants_mode_line` is declared by window.h and defined by window.c; the mirror
// keeps C's call (P-72 rule ③'s hazard: this macro expands only in units that include
// window.rs, exactly as C's does only in units that include window.h).
macro_rules! MATRIX_BOTTOM_TEXT_ROW {
  ($matrix:expr, $w:expr) => { {
    let matrix = $matrix;
    (*matrix).rows.offset ((*matrix).nrows as isize
                           - if unsafe { window_wants_mode_line ($w) } { 1 } else { 0 })
  } };
}

/* Non-zero if the face of the last glyph in ROW's text area has to be drawn to the end of
   the text area.  */
macro_rules! MATRIX_ROW_EXTENDS_FACE_P { ($row:expr) => { glyph_row_bits! (fill_line_p, $row) }; }

/* Set and query the enabled_p flag of glyph row ROW in MATRIX.  */
macro_rules! SET_MATRIX_ROW_ENABLED_P {
  ($matrix:expr, $row:expr, $value:expr) =>
    { glyph_row_bits! (enabled_p, MATRIX_ROW! ($matrix, $row), $value) };
}
macro_rules! MATRIX_ROW_ENABLED_P {
  ($matrix:expr, $row:expr) => { glyph_row_bits! (enabled_p, MATRIX_ROW! ($matrix, $row)) };
}

/* Non-zero if ROW displays text.  Value is non-zero if the row is blank but displays a
   line end.  */
macro_rules! MATRIX_ROW_DISPLAYS_TEXT_P {
  ($row:expr) => { glyph_row_bits! (displays_text_p, $row) };
}

/* Helper macros */

macro_rules! MR_PARTIALLY_VISIBLE {
  ($row:expr) => { (*($row)).height != (*($row)).visible_height };
}

// `WINDOW_TAB_LINE_HEIGHT`/`WINDOW_HEADER_LINE_HEIGHT`/`WINDOW_BOX_HEIGHT_NO_MODE_LINE`
// are window.h's macros; the mirror keeps C's text, and the expansion resolves in the units
// that include window.rs (P-72 rule ③), exactly as it does in C's units that include
// window.h.  `MR_PARTIALLY_VISIBLE_AT_TOP`'s C body names the tab- and header-line heights,
// `MR_PARTIALLY_VISIBLE_AT_BOTTOM`'s the box height without the mode line.
macro_rules! MR_PARTIALLY_VISIBLE_AT_TOP {
  ($w:expr, $row:expr) =>
    { (*($row)).y < WINDOW_TAB_LINE_HEIGHT! ($w) + WINDOW_HEADER_LINE_HEIGHT! ($w) };
}

macro_rules! MR_PARTIALLY_VISIBLE_AT_BOTTOM {
  ($w:expr, $row:expr) => {
    (*($row)).y + (*($row)).height - (*($row)).extra_line_spacing
      > WINDOW_BOX_HEIGHT_NO_MODE_LINE! ($w)
  };
}

/* Non-zero if ROW is not completely visible in window W.  */
macro_rules! MATRIX_ROW_PARTIALLY_VISIBLE_P {
  ($w:expr, $row:expr) => {
    MR_PARTIALLY_VISIBLE! ($row)
      && (MR_PARTIALLY_VISIBLE_AT_TOP! ($w, $row) || MR_PARTIALLY_VISIBLE_AT_BOTTOM! ($w, $row))
  };
}

/* Non-zero if ROW is partially visible at the top of window W.  */
macro_rules! MATRIX_ROW_PARTIALLY_VISIBLE_AT_TOP_P {
  ($w:expr, $row:expr) =>
    { MR_PARTIALLY_VISIBLE! ($row) && MR_PARTIALLY_VISIBLE_AT_TOP! ($w, $row) };
}

/* Non-zero if ROW is partially visible at the bottom of window W.  */
macro_rules! MATRIX_ROW_PARTIALLY_VISIBLE_AT_BOTTOM_P {
  ($w:expr, $row:expr) =>
    { MR_PARTIALLY_VISIBLE! ($row) && MR_PARTIALLY_VISIBLE_AT_BOTTOM! ($w, $row) };
}

/* Return the bottom Y + 1 of ROW.   */
macro_rules! MATRIX_ROW_BOTTOM_Y { ($row:expr) => { (*($row)).y + (*($row)).height }; }

/* Is ROW the last visible one in the display described by the iterator structure pointed
   to by IT?.  */
macro_rules! MATRIX_ROW_LAST_VISIBLE_P {
  ($row:expr, $it:expr) => { MATRIX_ROW_BOTTOM_Y! ($row) >= (*($it)).last_visible_y };
}

/* Non-zero if ROW displays a continuation line.  */
macro_rules! MATRIX_ROW_CONTINUATION_LINE_P {
  ($row:expr) => { (*($row)).continuation_lines_width > 0 };
}

/* Non-zero if ROW ends in the middle of a character.  This is the case for continued lines
   showing only part of a display table entry or a control char, or an overlay string.  */
macro_rules! MATRIX_ROW_ENDS_IN_MIDDLE_OF_CHAR_P {
  ($row:expr) => {
    (*($row)).end.dpvec_index > 0
      || (*($row)).end.overlay_string_index >= 0
      || glyph_row_bits! (ends_in_middle_of_char_p, $row)
  };
}

/* Non-zero if ROW ends in the middle of an overlay string.  */
macro_rules! MATRIX_ROW_ENDS_IN_OVERLAY_STRING_P {
  ($row:expr) => { (*($row)).end.overlay_string_index >= 0 };
}

/* Non-zero if ROW starts in the middle of a character.  See above.  */
macro_rules! MATRIX_ROW_STARTS_IN_MIDDLE_OF_CHAR_P {
  ($row:expr) => {
    (*($row)).start.dpvec_index > 0
      || glyph_row_bits! (starts_in_middle_of_char_p, $row)
      || ((*($row)).start.overlay_string_index >= 0
          && (*($row)).start.string_pos.charpos > 0)
  };
}

/* True means ROW overlaps its predecessor.  */
macro_rules! MATRIX_ROW_OVERLAPS_PRED_P {
  ($row:expr) => { (*($row)).phys_ascent > (*($row)).ascent };
}

/* True means ROW overlaps its successor.  */
macro_rules! MATRIX_ROW_OVERLAPS_SUCC_P {
  ($row:expr) => {
    (*($row)).phys_height - (*($row)).phys_ascent > (*($row)).height - (*($row)).ascent
  };
}

/* A glyph for a space.  */
unsafe extern "C" { pub static mut space_glyph: glyph; }     // dispextern.h 1338

/* Display a warning message.  Used by cmds.c's self-insert-command path (dispextern.h
   3900; the definition is in keyboard.c's mirror).  */
unsafe extern "C" { pub fn bitch_at_user (); }          // dispextern.h 3900

/* Enumeration of the areas in a glyph row.  */
pub type glyph_row_area = c_int;
pub const ANY_AREA: glyph_row_area = -1;                // dispextern.h 881-888
pub const LEFT_MARGIN_AREA: glyph_row_area = 0;
pub const TEXT_AREA: glyph_row_area = 1;
pub const RIGHT_MARGIN_AREA: glyph_row_area = 2;
pub const LAST_AREA: glyph_row_area = 3;

/* Values returned from coordinates_in_window — drawing operations.  */
pub type draw_glyphs_face = c_int;                      // dispextern.h 1348-1357
pub const DRAW_NORMAL_TEXT: draw_glyphs_face = 0;
pub const DRAW_INVERSE_VIDEO: draw_glyphs_face = 1;
pub const DRAW_CURSOR: draw_glyphs_face = 2;
pub const DRAW_MOUSE_FACE: draw_glyphs_face = 3;
pub const DRAW_IMAGE_RAISED: draw_glyphs_face = 4;
pub const DRAW_IMAGE_SUNKEN: draw_glyphs_face = 5;

// --- the window-system record: `struct glyph_string` (dispextern.h 1358-1509) --------------
/* C 1358-1509: `#ifdef HAVE_WINDOW_SYSTEM` - dead text on this target.  `HAVE_WINDOW_SYSTEM`
   is `undef` in the frozen configuration (docs/baseline/config/darwin-arm64.macros.tsv;
   `python3 tools/check-cond-eval.py src/dispextern.h` reports 0 disagreements with clang for
   this file), so no unit of the frozen build ever compiled this record and `--check` does not
   require it (its container list comes from the same evaluation).  It lands now because the
   window-system *backends* need the record and its fields, not only its name: `src/nsfont.m`'s
   GNUstep port reads `s->char2b[i]` (nsfont.m 1176), `s->first_glyph->type` (1161) and
   `s->first_glyph[i].u.ch` (1720), and the X/W32/pgtk backends read the same members - the
   Swift lane's two signatures (`nsfont_draw` 1151, `ns_dump_glyphstring` 1711) are the
   immediate consumer.  Per P-51 the text of an arm the frozen build does not compile is
   carried under a `feature` that names the *non*-frozen side, which is what
   `#[cfg (feature = "have_window_system")]` does (nothing in this tree enables it, so what the
   compiler sees is exactly what clang sees).

   Field types follow P-55/P-72.  `frame`, `window` and `font` have owner mirrors
   (`include/frame.rs`, `include/window.rs`, `include/font.rs`), and a unit that can enable
   this feature has them by construction: C's backends include frame.h/window.h/font.h, and
   `src/nsfont.m` 27-39 includes `dispextern.h`, `frame.h`, `window.h` and `font.h`.  What has
   no owner mirror *in this header's own include graph* keeps the untyped pointer and the
   `/// C:` line P-55 asks for: `struct face` (C 1787) is this header's own record and is not
   mirrored yet, `struct image` (C 3215) landed with this batch, `struct xwidget` is xwidget.h
   (a todo header in `docs/port-inventory.md`), and `GC`/`HDC`/`struct android_gc` are the
   platform types of the `gc` blocks below.

   `struct composition` moved to that second list in status 3.526.  It does have an owner
   mirror (`include/composite.rs`), but C's dispextern.h does **not** include composite.h - it
   names the tag only as a pointer (C 1421) - so the "by construction" claim this paragraph
   used to make about it was false for the live units that take this mirror without
   composite.h: `src/fontset.c` 36 is the measured one (2 x `E0425 composition` on the
   `fontset` arm).  `*mut c_void` plus the `/// C:` line is P-72 rule 3's reading of an
   incomplete type, and the one consumer that reads the member (`src/abi_shim.rs` 1851)
   already casts it.

   The record is **not** layout-asserted: this target has no window-system configuration to
   measure it with, so a `const _` here would be an invented number.  What *is* measured is the
   bitfield run in isolation (the `GLYPH_STRING_*` note below); everything else about the
   record's layout is unmeasured and is recorded as such in docs/ports/include-dispextern.md.  */
#[cfg (feature = "have_window_system")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct glyph_string
{
  /* X-origin of the string.  */
  pub x: c_int,                                // C 1365

  /* Y-origin and y-position of the base line of this string.  */
  pub y: c_int,                                // C 1368
  pub ybase: c_int,

  /* The width of the string, not including a face extension.  */
  pub width: c_int,                            // C 1371

  /* The width of the string, including a face extension.  */
  pub background_width: c_int,                 // C 1374

  /* The height of this string.  This is the height of the line this
     string is drawn in, and can be different from the height of the
     font the string is drawn in.  */
  pub height: c_int,                           // C 1379

  /* Number of pixels this string overwrites in front of its x-origin.
     This number is zero if the string has an lbearing >= 0; it is
     -lbearing, if the string has an lbearing < 0.  */
  pub left_overhang: c_int,                    // C 1384

  /* Number of pixels this string overwrites past its right-most
     nominal x-position, i.e. x + width.  Zero if the string's
     rbearing is <= its nominal width, rbearing - width otherwise.  */
  pub right_overhang: c_int,                   // C 1389

  /* The frame on which the glyph string is drawn.  */
  pub f: *mut frame,                           // C 1392

  /* The window on which the glyph string is drawn.  */
  pub w: *mut window,                          // C 1395

  /* The glyph row for which this string was built.  It determines the
     y-origin and height of the string.  */
  pub row: *mut glyph_row,                     // C 1399

  /* The area within row.  */
  pub area: glyph_row_area,                    // C 1402

  /* Characters to be drawn, and number of characters.  Note that
     NCHARS can be zero if this is a composition glyph string, as
     evidenced by FIRST_GLYPH->type.  */
  pub char2b: *mut c_uint,                     // C 1407
  pub nchars: c_int,                           // C 1408

  /* A face-override for drawing cursors, mouse face and similar.  */
  pub hl: draw_glyphs_face,                    // C 1411

  /* Face in which this string is to be drawn.  */
  pub face: *mut c_void,                       /// C: `struct face *face;` (dispextern.h 1414)
  // C 1414: the record itself is dispextern.h 1787, which this mirror does not carry yet (it
  // is one of the seven types the extractor defers for this file), so the member is the
  // untyped pointer P-55 prescribes; it becomes `*mut face` when C 1787 lands here.

  /* Font in which this string is to be drawn.  */
  pub font: *mut font,                         // C 1417

  /* Non-null means this string describes (part of) a static
     composition.  */
  pub cmp: *mut c_void,                        /// C: `struct composition *cmp;` (composite.h 148; P-72 (3))

  /* If not negative, this string describes a compos.  */
  pub cmp_id: ptrdiff_t,                       // C 1424

  /* Start and end glyph indices in a glyph-string.  */
  pub cmp_from: c_int,                         // C 1427
  pub cmp_to: c_int,

  /* C 1431-1461: the record's one bitfield run - `bool_bf extends_to_end_of_line_p : 1`
     (C 1431), `bool_bf background_filled_p : 1` (1434), `bool_bf font_not_found_p : 1`
     (1439), `bool_bf stippled_p : 1` (1443), `unsigned for_overlaps : 3` (1456) and
     `bool_bf padding_p : 1` (1461).  Rust has no bitfields, so P-80's shape applies: one
     storage unit here, and the six member names live in `glyph_string_bits!`'s arms and in
     the `GLYPH_STRING_*` masks below.  The unit is a `u32` because that is the size a
     *reduced* probe of exactly these six declarations gives (see the constants; the probe
     carries no window-system type and is **not** this record's layout, which is unmeasured
     - see the file note above).  */
  pub bits: u32,                               // C 1431-1461

  /* The GC to use for drawing this glyph string (C 1464-1475).  A window-system build picks
     exactly one arm, so each is carried under the feature this tree's units use for that C
     macro.  */
  #[cfg (feature = "have_x_windows")]
  pub gc: *mut c_void,                         /// C: `GC gc;` (dispextern.h 1465)
  // C 1465's `GC` is X11's `struct _XGC *` and has no mirror; the only `GC` this tree declares
  // is msdos.h's `typedef int GC` (msdos.rs 217), a *different* type with the same name, so
  // the pointer stays untyped rather than silently picking that one up (P-55/P-72).
  #[cfg (feature = "have_android")]
  pub gc: *mut c_void,                         /// C: `struct android_gc *gc;` (dispextern.h 1467)
  // C 1466-1467: `struct android_gc` (androidgui.h 148) is not in androidgui.rs yet
  // (that mirror declares `android_gc_function`/`android_gc_values`/`android_gcontext`).
  #[cfg (feature = "have_ntgui")]
  pub gc: *mut Emacs_GC,                       // C 1470: `Emacs_GC *gc;`
  #[cfg (feature = "have_ntgui")]
  pub hdc: *mut c_void,                        /// C: `HDC hdc;` (dispextern.h 1471)
  #[cfg (feature = "have_pgtk")]
  pub xgcv: Emacs_GC,                          // C 1474: `Emacs_GC xgcv;`

  /* A pointer to the first glyph in the string.  This glyph
     corresponds to char2b[0].  Needed to draw rectangles if
     font_not_found_p is true.  */
  pub first_glyph: *mut glyph,                 // C 1480

  /* Image, if any.  */
  pub img: *mut image,                         // C 1483: `struct image *img;` (defined below, C 3215)

  /* Xwidget.  */
  pub xwidget: *mut c_void,                    /// C: `struct xwidget *xwidget;` (dispextern.h 1486)
  // C 1486: `struct xwidget` is xwidget.h, still a todo header in docs/port-inventory.md.

  /* Slice */
  pub slice: glyph_slice,                      // C 1489

  /* Non-null means the horizontal clipping region starts from the
     left edge of *clip_head, and ends with the right edge of
     *clip_tail, not including their overhangs.  */
  pub clip_head: *mut glyph_string,            // C 1494
  pub clip_tail: *mut glyph_string,

  /* The current clipping areas.  */
  pub clip: [c_int; 2],                        /// C: `NativeRectangle clip[2];` (dispextern.h 1497)
  // C 1497: `NativeRectangle` is a *type* macro and its answer differs per window system
  // (C 119's `#define NativeRectangle XRectangle`, C 178-180's fallback - which is live text in
  // the frozen configuration - `#define NativeRectangle int`, w32gui.h's `RECT`, haikugui.h's
  // `struct haiku_rect`).  This mirror names none of them: the fallback is what a window-system
  // build without X, Android or Haiku has (the NS target's answer), and the port's shape for
  // the macro is a `macro_rules!` at the platform header (w32gui.rs 148, haikugui.rs 64), which
  // this mirror cannot also carry without colliding with w32gui.rs's in `layout-check`'s single
  // crate.  The integrator owns that call (docs/ports/include-dispextern.md).

  /* Number of clipping areas. */
  pub num_clips: c_int,                        // C 1500

  pub underline_position: c_int,               // C 1502

  pub underline_thickness: c_int,              // C 1504

  pub next: *mut glyph_string,                 // C 1506
  pub prev: *mut glyph_string,
}

/* C 1445-1448: the four `OVERLAPS_*` macros, written inside the record's body in C but macros
   in its sense (they are visible to every unit that compiles this arm).  `dispnew.rs` 3646-3647
   carries `OVERLAPS_PRED`/`OVERLAPS_SUCC` as *live* copies of its own, because the frozen
   configuration has no such macro - the window-system freeze retires them (the `struct font`
   use-site precedent, status 3.158), and docs/ports/include-dispextern.md records the pair.  */
#[cfg (feature = "have_window_system")]
pub const OVERLAPS_PRED: c_uint = 1 << 0;                        // C 1445
#[cfg (feature = "have_window_system")]
pub const OVERLAPS_SUCC: c_uint = 1 << 1;                        // C 1446
#[cfg (feature = "have_window_system")]
pub const OVERLAPS_BOTH: c_uint = OVERLAPS_PRED | OVERLAPS_SUCC;  // C 1447
#[cfg (feature = "have_window_system")]
pub const OVERLAPS_ERASED_CURSOR: c_uint = 1 << 2;               // C 1448

/* `glyph_string::bits`, the six members of C 1431-1461, in the order the C declarations put
   them: four `bool_bf : 1` (bits 0-3), `unsigned for_overlaps : 3` (bits 4-6) and one more
   `bool_bf : 1` (bit 7).  The *positions* come from a reduced probe - the six declarations
   alone, compiled by the frozen clang with no window-system type - which prints
   `sizeof (struct run) = 4` and `ff 00 00 00` with all six set (and `01 00 00 00`/`71 00 00 00`
   for the first member and the first plus `for_overlaps`), i.e. one byte of members inside a
   four-byte unit.  That is the run's own answer, not the record's layout: the record's
   `bits` offset and every other offset are unmeasured on this target and asserted nowhere.  */
#[cfg (feature = "have_window_system")]
pub const GLYPH_STRING_EXTENDS_TO_END_OF_LINE_P: u32 = 1 << 0;   // C 1431
#[cfg (feature = "have_window_system")]
pub const GLYPH_STRING_BACKGROUND_FILLED_P: u32 = 1 << 1;        // C 1434
#[cfg (feature = "have_window_system")]
pub const GLYPH_STRING_FONT_NOT_FOUND_P: u32 = 1 << 2;           // C 1439
#[cfg (feature = "have_window_system")]
pub const GLYPH_STRING_STIPPLED_P: u32 = 1 << 3;                 // C 1443
#[cfg (feature = "have_window_system")]
pub const GLYPH_STRING_FOR_OVERLAPS_SHIFT: u32 = 4;              // C 1456
#[cfg (feature = "have_window_system")]
pub const GLYPH_STRING_FOR_OVERLAPS_WIDTH: u32 = 3;
#[cfg (feature = "have_window_system")]
pub const GLYPH_STRING_PADDING_P: u32 = 1 << 7;                  // C 1461

/// Read one numeric member of `glyph_string::bits` — C's `(S).for_overlaps` (zero-extended,
/// as an unsigned C bitfield of that width is).
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn glyph_string_bits_get (g: *const glyph_string, shift: u32, width: u32) -> c_int
{
  unsafe { (((*g).bits >> shift) & glyph_string_bits_mask (width)) as c_int }
}

/// Assign one numeric member of `glyph_string::bits` — C's `(S).for_overlaps = VALUE`: only the
/// member's bits change.
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn glyph_string_bits_set (g: *mut glyph_string, shift: u32, width: u32, value: c_int)
{
  unsafe {
    let mask = glyph_string_bits_mask (width) << shift;
    (*g).bits = ((*g).bits & !mask) | ((((value as u32) & glyph_string_bits_mask (width)) << shift) & mask);
  }
}

/// The `width`-bit mask of a `glyph_string::bits` member.
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn glyph_string_bits_mask (width: u32) -> u32
{
  if width >= 32 { u32::MAX } else { (1u32 << width) - 1 }
}

/// Read one `bool_bf : 1` member of `glyph_string::bits` — C's `(S).MEMBER`.
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn glyph_string_bool_p (g: *const glyph_string, mask: u32) -> bool
{
  unsafe { ((*g).bits & mask) != 0 }
}

/// Assign one `bool_bf : 1` member of `glyph_string::bits` — C's `(S).MEMBER = VALUE`.
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn glyph_string_bool_set (g: *mut glyph_string, mask: u32, value: bool)
{
  unsafe {
    let bits = &mut (*g).bits;
    *bits = if value { *bits | mask } else { *bits & !mask };
  }
}

/// C's bit members of `glyph_string` as a read/write pair, because Rust has no bitfields
/// (PORTING.md 1.3): `s->padding_p` is `glyph_string_bits! (padding_p, s)`, and
/// `s->padding_p = v` is `glyph_string_bits! (padding_p, s, v)`.  The second argument is C's
/// `(S)`; every arm carries the member's position (`glyph_string::bits`, one `u32`).
#[cfg (feature = "have_window_system")]
macro_rules! glyph_string_bits {
  (extends_to_end_of_line_p, $s:expr) =>
    { glyph_string_bool_p (&($s), GLYPH_STRING_EXTENDS_TO_END_OF_LINE_P) };
  (extends_to_end_of_line_p, $s:expr, $value:expr) =>
    { glyph_string_bool_set (&mut ($s), GLYPH_STRING_EXTENDS_TO_END_OF_LINE_P, $value) };
  (background_filled_p, $s:expr) =>
    { glyph_string_bool_p (&($s), GLYPH_STRING_BACKGROUND_FILLED_P) };
  (background_filled_p, $s:expr, $value:expr) =>
    { glyph_string_bool_set (&mut ($s), GLYPH_STRING_BACKGROUND_FILLED_P, $value) };
  (font_not_found_p, $s:expr) =>
    { glyph_string_bool_p (&($s), GLYPH_STRING_FONT_NOT_FOUND_P) };
  (font_not_found_p, $s:expr, $value:expr) =>
    { glyph_string_bool_set (&mut ($s), GLYPH_STRING_FONT_NOT_FOUND_P, $value) };
  (stippled_p, $s:expr) =>
    { glyph_string_bool_p (&($s), GLYPH_STRING_STIPPLED_P) };
  (stippled_p, $s:expr, $value:expr) =>
    { glyph_string_bool_set (&mut ($s), GLYPH_STRING_STIPPLED_P, $value) };
  (for_overlaps, $s:expr) =>
    { glyph_string_bits_get (&($s), GLYPH_STRING_FOR_OVERLAPS_SHIFT, GLYPH_STRING_FOR_OVERLAPS_WIDTH) };
  (for_overlaps, $s:expr, $value:expr) =>
    { glyph_string_bits_set (&mut ($s), GLYPH_STRING_FOR_OVERLAPS_SHIFT,
                             GLYPH_STRING_FOR_OVERLAPS_WIDTH, $value) };
  (padding_p, $s:expr) =>
    { glyph_string_bool_p (&($s), GLYPH_STRING_PADDING_P) };
  (padding_p, $s:expr, $value:expr) =>
    { glyph_string_bool_set (&mut ($s), GLYPH_STRING_PADDING_P, $value) };
}

/* Indices of face attributes in Lisp face vectors.  */
pub type lface_attribute_index = c_int;                 // dispextern.h 1723-1748
pub const LFACE_FAMILY_INDEX: lface_attribute_index = 1;
pub const LFACE_FOUNDRY_INDEX: lface_attribute_index = 2;
pub const LFACE_SWIDTH_INDEX: lface_attribute_index = 3;
pub const LFACE_HEIGHT_INDEX: lface_attribute_index = 4;
pub const LFACE_WEIGHT_INDEX: lface_attribute_index = 5;
pub const LFACE_SLANT_INDEX: lface_attribute_index = 6;
pub const LFACE_UNDERLINE_INDEX: lface_attribute_index = 7;
pub const LFACE_INVERSE_INDEX: lface_attribute_index = 8;
pub const LFACE_FOREGROUND_INDEX: lface_attribute_index = 9;
pub const LFACE_BACKGROUND_INDEX: lface_attribute_index = 10;
pub const LFACE_STIPPLE_INDEX: lface_attribute_index = 11;
pub const LFACE_OVERLINE_INDEX: lface_attribute_index = 12;
pub const LFACE_STRIKE_THROUGH_INDEX: lface_attribute_index = 13;
pub const LFACE_BOX_INDEX: lface_attribute_index = 14;
pub const LFACE_FONT_INDEX: lface_attribute_index = 15;
pub const LFACE_INHERIT_INDEX: lface_attribute_index = 16;
pub const LFACE_FONTSET_INDEX: lface_attribute_index = 17;
pub const LFACE_DISTANT_FOREGROUND_INDEX: lface_attribute_index = 18;
pub const LFACE_EXTEND_INDEX: lface_attribute_index = 19;
pub const LFACE_VECTOR_SIZE: lface_attribute_index = 20;

/* Box types of faces.  */
pub type face_box_type = c_int;                         // dispextern.h 1750-1765
pub const FACE_NO_BOX: face_box_type = 0;
pub const FACE_SIMPLE_BOX: face_box_type = 1;
pub const FACE_RAISED_BOX: face_box_type = 2;
pub const FACE_SUNKEN_BOX: face_box_type = 3;

/* Underline type. */
pub type face_underline_type = c_int;                   // dispextern.h 1767-1780
pub const FACE_NO_UNDERLINE: face_underline_type = 0;
pub const FACE_UNDERLINE_SINGLE: face_underline_type = 1;
pub const FACE_UNDERLINE_DOUBLE_LINE: face_underline_type = 2;
pub const FACE_UNDERLINE_WAVE: face_underline_type = 3;
pub const FACE_UNDERLINE_DOTS: face_underline_type = 4;
pub const FACE_UNDERLINE_DASHES: face_underline_type = 5;

/* Basic face identifiers.  */
pub type face_id = c_int;                               // dispextern.h 1959-1985
pub const DEFAULT_FACE_ID: face_id = 0;
pub const MODE_LINE_ACTIVE_FACE_ID: face_id = 1;
pub const MODE_LINE_INACTIVE_FACE_ID: face_id = 2;
pub const TOOL_BAR_FACE_ID: face_id = 3;
pub const FRINGE_FACE_ID: face_id = 4;
pub const HEADER_LINE_ACTIVE_FACE_ID: face_id = 5;
pub const HEADER_LINE_INACTIVE_FACE_ID: face_id = 6;
pub const SCROLL_BAR_FACE_ID: face_id = 7;
pub const BORDER_FACE_ID: face_id = 8;
pub const CURSOR_FACE_ID: face_id = 9;
pub const MOUSE_FACE_ID: face_id = 10;
pub const MENU_FACE_ID: face_id = 11;
pub const VERTICAL_BORDER_FACE_ID: face_id = 12;
pub const WINDOW_DIVIDER_FACE_ID: face_id = 13;
pub const WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID: face_id = 14;
pub const WINDOW_DIVIDER_LAST_PIXEL_FACE_ID: face_id = 15;
pub const INTERNAL_BORDER_FACE_ID: face_id = 16;
pub const CHILD_FRAME_BORDER_FACE_ID: face_id = 17;
pub const TAB_BAR_FACE_ID: face_id = 18;
pub const TAB_LINE_ACTIVE_FACE_ID: face_id = 19;
pub const TAB_LINE_INACTIVE_FACE_ID: face_id = 20;
pub const MARGIN_FACE_ID: face_id = 21;
pub const BASIC_FACE_ID_SENTINEL: face_id = 22;

// --- batch II: the position pairs, the slice and the bidi state (dispextern.h 275-370,
//     2080-2160, 2360-2366, 3048-3058) --------------------------------------------------
// P-66's order: `text_pos`/`display_pos` are the pair every display function passes around,
// `it_slice` is four Lisp_Objects, `run` is six ints, and the `bidi_*` trio is the state
// `struct it` carries.  None of them needs a platform type, so they land before the matrix
// and face structures (which need `struct frame`/`struct window`; the third name they need,
// `struct font`, is in `include/font.rs` now).

/* Starting with Emacs 20.3, characters from strings and buffers have
   both a character and a byte position associated with them.  The
   following structure holds such a pair of positions.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct text_pos
{
  /* Character position.  */
  pub charpos: ptrdiff_t,               // dispextern.h 278
  /* Corresponding byte position.  */
  pub bytepos: ptrdiff_t,
}

// BYTEPOS/CHARPOS/SET_TEXT_POS/INC_TEXT_POS arrive with the functions that use them; like
// every other place-expression macro they are spelled as functions (P-17).

/* When rendering glyphs, redisplay scans string or buffer text,
   overlay strings in that text, and does display table or control
   character translations.  The following structure captures a
   position taking all this into account.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct display_pos
{
  pub pos: text_pos,                    // dispextern.h 354
  pub overlay_string_index: ptrdiff_t,  // < 0 when `pos' is not in an overlay string
  pub string_pos: text_pos,
  pub dpvec_index: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct it_slice
{
  pub x: Lisp_Object,                   // dispextern.h 2362
  pub y: Lisp_Object,
  pub width: Lisp_Object,
  pub height: Lisp_Object,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct run
{
  pub desired_y: c_int,                 // dispextern.h 3051
  pub current_y: c_int,
  pub desired_vpos: c_int,
  pub current_vpos: c_int,
  pub height: c_int,
  pub nrows: c_int,
}

// dispextern.h 2080-2108: `typedef enum { … } bidi_type_t;`.  The first seven members are
// the only ones valid in `struct glyph`'s `bidi_type` field, which reserves three bits.
pub type bidi_type_t = c_int;
pub const UNKNOWN_BT: bidi_type_t = 0;
pub const STRONG_L: bidi_type_t = 1;
pub const STRONG_R: bidi_type_t = 2;
pub const WEAK_EN: bidi_type_t = 3;
pub const WEAK_AN: bidi_type_t = 4;
pub const WEAK_BN: bidi_type_t = 5;
pub const NEUTRAL_B: bidi_type_t = 6;
pub const STRONG_AL: bidi_type_t = 7;
pub const LRE: bidi_type_t = 8;
pub const LRO: bidi_type_t = 9;
pub const RLE: bidi_type_t = 10;
pub const RLO: bidi_type_t = 11;
pub const PDF: bidi_type_t = 12;
pub const LRI: bidi_type_t = 13;
pub const RLI: bidi_type_t = 14;
pub const FSI: bidi_type_t = 15;
pub const PDI: bidi_type_t = 16;
pub const WEAK_ES: bidi_type_t = 17;
pub const WEAK_ET: bidi_type_t = 18;
pub const WEAK_CS: bidi_type_t = 19;
pub const WEAK_NSM: bidi_type_t = 20;
pub const NEUTRAL_S: bidi_type_t = 21;
pub const NEUTRAL_WS: bidi_type_t = 22;
pub const NEUTRAL_ON: bidi_type_t = 23;
pub const BIDI_MAXDEPTH: c_int = 125;   // dispextern.h 2076 (UAX#9's max_depth)

// dispextern.h 2115-2119: the paired-bracket type (`bracket-type` property order).
pub type bidi_bracket_type_t = c_int;
pub const BIDI_BRACKET_NONE: bidi_bracket_type_t = 1;
pub const BIDI_BRACKET_OPEN: bidi_bracket_type_t = 2;
pub const BIDI_BRACKET_CLOSE: bidi_bracket_type_t = 3;

// dispextern.h 2122: the basic directionality.
pub type bidi_dir_t = c_int;
pub const NEUTRAL_DIR: bidi_dir_t = 0;
pub const L2R: bidi_dir_t = 1;
pub const R2L: bidi_dir_t = 2;

/* Data type for storing information about characters we need to
   remember.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct bidi_saved_info
{
  pub charpos: ptrdiff_t,               // dispextern.h 2127
  pub r#type: bidi_type_t,              // C spells it `type`; `type` is a Rust keyword
  pub orig_type: bidi_type_t,
}

// dispextern.h 2137-2144.  The three 3-bit members share a 2-byte unit at offset 8, and the
// frozen clang then puts `level` at 10 and `flags` at 11 (measured; the probe is in the
// commit).  PORTING.md 1.3: the storage unit is what the gate can assert.
//
//     s.next_for_neutral_type = 5; s.last_strong_type = 6; s.prev_for_neutral_type = 7;
//     s.level = 9; s.flags = 0x11;      bytes: 00×8 f5 01 09 11 00 00 00 00
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bidi_stack
{
  pub next_for_neutral_pos: ptrdiff_t,  // dispextern.h 2138
  pub bits: u16,                        // next_for_neutral_type:3 | last_strong_type:3 | prev_for_neutral_type:3
  pub level: u8,
  pub flags: u8,                        // sos, override, isolate_status
}

// dispextern.h 2147-2160.  `lstring`/`s`/`schars`/`bufpos` are 8 bytes each, and the two
// `bool_bf : 1` members share a 4-byte unit at offset 32 (`bool_bf` is a typedef in
// conf_post.h 58-62: `bool`, one byte, on this target); sizeof 40, align 8 (measured).
//
//     d.lstring = 0x1122334455667788; d.schars = 5; d.bufpos = 6;
//     d.from_disp_str = 1; d.unibyte = 1;   bytes: 88 77 66 55 44 33 22 11 … 03 00 00 00
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bidi_string_data
{
  pub lstring: Lisp_Object,             // dispextern.h 2148
  pub s: *const u8,                     // `const unsigned char *`
  pub schars: ptrdiff_t,
  pub bufpos: ptrdiff_t,
  pub bits: u32,                        // from_disp_str:1 | unibyte:1
}

/* Check that glyph pointers stored in glyph rows of MATRIX are okay.
   This aborts if any pointer is found twice.  */

// dispextern.h 862-870: the block is inside `#ifdef GLYPH_DEBUG`, and GLYPH_DEBUG is undef
// in the frozen configuration (docs/baseline/config/darwin-arm64.macros.tsv), so C compiles
// the `#else` arm: `((void) 0)`.  `CHECK_MATRIX` therefore does not evaluate its argument,
// and neither does the port's live arm (the `aclinfo_free` family's shape in
// lib/include/acl.rs); the dead arm's function keeps C's prototype now that
// `struct glyph_matrix` is declared above (it is the same header's type).
#[cfg(feature = "glyph_debug")]
unsafe extern "C" { pub fn check_matrix_pointer_lossage (matrix: *mut glyph_matrix); }  // dispextern.h 864
// dispextern.h 869.
macro_rules! CHECK_MATRIX { ($matrix:expr) => { { let _ = ($matrix); 0 } }; }

/***********************************************************************
			    Mouse Highlight
 ***********************************************************************/

/* Structure to hold mouse highlight data.  */

// dispextern.h 2998-3024 (an anonymous `typedef struct`).  `mouse_face_mouse_frame` is a
// `struct frame *` in C; frame.h is not in this mirror's include list (callint.c, json.c and
// marker.c include `dispextern.h` without `frame.h`, which is exactly the P-72 hazard), so the
// pointer keeps the untyped form its use sites cast.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mouse_HLInfo
{
  /* These variables describe the range of text currently shown in its
     mouse-face, together with the window they apply to.  As long as
     the mouse stays within this range, we need not redraw anything on
     its account.  Rows and columns are glyph matrix positions in
     MOUSE_FACE_WINDOW.  */
  pub mouse_face_beg_row: c_int,
  pub mouse_face_beg_col: c_int,
  pub mouse_face_beg_x: c_int,
  pub mouse_face_end_row: c_int,
  pub mouse_face_end_col: c_int,
  pub mouse_face_end_x: c_int,
  pub mouse_face_window: Lisp_Object,
  pub mouse_face_face_id: c_int,
  pub mouse_face_overlay: Lisp_Object,

  /* FRAME and X, Y position of mouse when last checked for
     highlighting.  X and Y can be negative or out of range for the frame.  */
  pub mouse_face_mouse_frame: *mut c_void,   /// C: `struct frame *mouse_face_mouse_frame;`
  pub mouse_face_mouse_x: c_int,
  pub mouse_face_mouse_y: c_int,

  // dispextern.h 3019-3023: three consecutive `bool_bf : 1` members, i.e. one byte.
  pub bits: u8,
}

pub const MOUSE_FACE_PAST_END: u8 = 1 << 0;   // dispextern.h 3019
pub const MOUSE_FACE_DEFER: u8 = 1 << 1;      // dispextern.h 3022
pub const MOUSE_FACE_HIDDEN: u8 = 1 << 2;     // dispextern.h 3023

/* C 3020: `bool_bf mouse_face_defer : 1;` - the middle member of the run above, which C reads
   and writes as a member and the port spells through the `MOUSE_FACE_DEFER` bit of `bits`
   (PORTING.md 1.3; `docs/ports/w32term.md` owed 5 asks for the pair).  */
/// C: `bool_bf mouse_face_defer : 1;`
#[inline(always)]
pub fn get_mouse_face_defer (hlinfo: *const Mouse_HLInfo) -> bool
{ unsafe { (*hlinfo).bits & MOUSE_FACE_DEFER != 0 } }
#[inline(always)]
pub fn set_mouse_face_defer (hlinfo: *mut Mouse_HLInfo, value: bool)
{
  unsafe {
    if value { (*hlinfo).bits |= MOUSE_FACE_DEFER }
    else { (*hlinfo).bits &= !MOUSE_FACE_DEFER }
  }
}

// C 3026-3041 is an INLINE *header* function: every TU that calls it gets C's own body, so the
// definition belongs at this level (a unit that declared it `extern` instead linked against a
// symbol nothing defines - the executable link named `_reset_mouse_highlight`, status 3.610).
/* C 3026-3041: the mouse-highlight resetter.  C's body is a list of member writes; the
   record it writes into (`struct Mouse_HLInfo`) is this mirror's own and ungated.  */
/// C: INLINE void reset_mouse_highlight (Mouse_HLInfo *hlinfo)
#[inline(always)]
pub fn reset_mouse_highlight (hlinfo: *mut Mouse_HLInfo)
{
  unsafe {
    (*hlinfo).mouse_face_beg_row = -1;
    (*hlinfo).mouse_face_beg_col = -1;
    (*hlinfo).mouse_face_end_row = -1;
    (*hlinfo).mouse_face_end_col = -1;
    (*hlinfo).mouse_face_mouse_x = 0;
    (*hlinfo).mouse_face_mouse_y = 0;
    (*hlinfo).mouse_face_beg_x = 0;
    (*hlinfo).mouse_face_end_x = 0;
    (*hlinfo).mouse_face_face_id = DEFAULT_FACE_ID;
    (*hlinfo).mouse_face_mouse_frame = std::ptr::null_mut ();
    (*hlinfo).mouse_face_window = Qnil ();
    (*hlinfo).mouse_face_overlay = Qnil ();
    // C 3019-3023's three `bool_bf : 1` members (`mouse_face_past_end`, `mouse_face_defer`,
    // `mouse_face_hidden`) are the mirror's single `bits` byte with the three masks below, so
    // C's three `= false` writes are the mask clear; the three C names are bound to the
    // `false`s C assigns them, the way `GLYPH_CHAR` names the value it returns.
    let mouse_face_past_end = false;
    let mouse_face_defer = false;
    let mouse_face_hidden = false;
    let _ = (mouse_face_past_end, mouse_face_defer, mouse_face_hidden);
    (*hlinfo).bits &= !(MOUSE_FACE_PAST_END | MOUSE_FACE_DEFER | MOUSE_FACE_HIDDEN);
  }
}

// dispextern.h 3966: `enum { SCROLL_INFINITY = 1000000 };`
pub const SCROLL_INFINITY: c_int = 1000000;

/* ---- C 1787-2009: the realized face and its cache ------------------------------------------
   The two records `struct frame` and its accessors need (`FRAME_FACE_CACHE`, `FACE_FROM_ID`,
   `FACE_FROM_ID_OR_NULL` in `include/frame.rs`), landed here because dispextern.h is where C
   defines them.  The layout is the one the units measured for the frozen configuration and
   every offset below is asserted; `HAVE_WINDOW_SYSTEM` is undef, so `gc`/`stipple`
   (C 1800-1815) and `HAVE_NS`/`HAVE_XFT`/`HAVE_FREETYPE`'s `extra` are absent from this
   target's layout, exactly as the C compiler lays it out.

   **Ruling (status 3.220): the records stay here behind `dispextern_face_records`, and the
   units keep their live copies.**  Six units carry their own (`src/xfaces.rs` 279-345,
   `src/window.rs` 878-915, `src/alloc.rs` 3441-3461, `src/term.rs` 308/323,
   `src/dispnew.rs`, and the two in-flight ones `src/xdisp.rs` and `src/font.rs`), every one of
   them *typed* (`*mut face` / `*mut font` / `*mut frame`) because those units include
   `include/font.rs`/`include/frame.rs` in C's order, and all seven include this mirror - so an
   unconditional copy here is `E0428` in all seven (measured: `build-tu src/dispnew.c` -> "the
   name `face` is defined multiple times").  Retiring the units' copies - the `stdbit.in.rs`
   move of 3.181 - is *possible* and measured to cost about three edits in the smallest unit
   (`src/alloc.rs`: E0308 once and `no field header on type c_void` twice, after its two records
   were deleted), but it would have to touch two lanes' in-flight files and would downgrade
   their typed accessors to casts.  The ruling is therefore the cheap one that keeps C's text
   present: this copy is the arm `xtask dead-arms`/`gate-probes` flips and compiles
   (`dispextern_face_records` is a row in femacs/xtask/Cargo.toml), and the units' typed copies
   stay live.  The alternative is recorded here so a later round can take it deliberately.
   The copy is field-for-field the one `xfaces.rs` asserts.  */

/// C 1787-1937: `struct face`.  Measured with the frozen clang: `sizeof` 280, align 8,
/// `lface`@0, `id`@160, `foreground`@168, `background`@176, `underline_color`@184,
/// `overline_color`@192, `strike_through_color`@200, `box_color`@208, `font`@216,
/// `fontset`@224, `box_vertical_line_width`@228, `box_horizontal_line_width`@232,
/// `underline_pixels_above_descent_line`@236, the 21 `bool_bf`/`ENUM_BF` bits C declares at
/// 1893-1935 in one 4-byte storage unit at 240, `hash`@248, `next`@256, `prev`@264,
/// `ascii_face`@272.
#[cfg (feature = "dispextern_face_records")]
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
#[cfg (feature = "dispextern_face_records")]
#[cfg (not (feature = "have_window_system"))]
const FACE_WS_SHIFT: usize = 0;
#[cfg (feature = "dispextern_face_records")]
#[cfg (feature = "have_window_system")]
const FACE_WS_SHIFT: usize = 16;

#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::size_of::<face> () == 280 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::align_of::<face> () == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, id) == 160);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, gc) == 168);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, stipple) == 176);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, foreground) == 168 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, background) == 176 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, underline_color) == 184 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, overline_color) == 192 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, strike_through_color) == 200 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, box_color) == 208 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, font) == 216 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, fontset) == 224 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, bits) == 240 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, hash) == 248 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, next) == 256 + FACE_WS_SHIFT);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face, ascii_face) == 272 + FACE_WS_SHIFT);

/// C 1991-2009: `struct face_cache`.  Measured: `sizeof` 40, `buckets`@0, `f`@8,
/// `faces_by_id`@16, `size`@24, `used`@32, and C's one `bool_bf menu_face_changed_p : 1` in
/// the int unit at 36 (carried as a byte, the P-80 convention - `src/window.rs` 907 spells
/// the same member `bool`, one byte wide either way).
#[cfg (feature = "dispextern_face_records")]
#[repr(C)]
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
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::size_of::<face_cache> () == 40);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, buckets) == 0);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, f) == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, faces_by_id) == 16);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, size) == 24);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, used) == 32);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, menu_face_changed_p) == 36);

/// C 2030-2038: `INLINE int FACE_FOR_CHAR (struct frame *f, struct face *face, int character,
/// ptrdiff_t pos, Lisp_Object object)` - `HAVE_WINDOW_SYSTEM` is undef, so C's body is
/// `return face->id;`, a read of a `struct face` member.  The function therefore lives in the
/// same arm as the record it reads (status 3.220); its frame parameter is `*mut c_void`
/// because this arm does not carry `struct frame` (P-72 ③), and `src/xfaces.rs` 489 keeps the
/// unit's own typed copy.
/// C: INLINE int FACE_FOR_CHAR (struct frame *f, struct face *face, int character, ptrdiff_t pos, Lisp_Object object) { return face->id; }
#[cfg (feature = "dispextern_face_records")]
#[inline(always)]
pub fn FACE_FOR_CHAR (f: *mut c_void, face: *mut face, character: c_int, pos: ptrdiff_t,
                      object: Lisp_Object) -> face_id
{
  let _ = (f, character, pos, object);
  unsafe { (*face).id }
}

/* =============================================================================================
   The image record - C's second `HAVE_WINDOW_SYSTEM` block (dispextern.h 3203-3395)
   ============================================================================================= */

// C's `<time.h>` chain (dispextern.h 25 includes character.h, which reaches it through lisp.h)
// is what makes C 3219's `struct timespec` a complete type.  The mirror is this tree's
// (`lib/include/time.in.rs`, the same file `buffer.rs` and `systime.rs` reach), included here
// per P-72's rule that a mirror may only name types it declares or includes; `xtask`'s
// include-graph expansion dedupes the second reach of the file (PORTING.md 0.1-4/P-61), so a
// unit that already has it does not define it twice.
include!("../../lib/include/time.in.rs");

/* The image record (dispextern.h 3215-3362).  C wraps it in `#ifdef HAVE_WINDOW_SYSTEM`, so it
   is dead text on the frozen target - `spikes/gated-layout-probe.py 'struct image'
   dispextern.h` reads `parse=no` there (the record is an *incomplete* type) - and the mirror
   carries it under P-51's `#[cfg (feature = "have_window_system")]` arm, the feature naming the
   non-frozen side.  The frozen build therefore compiles exactly what clang compiles.

   Its layout **is** measured on both window-system arms (P-82), and the readings are in
   `docs/baseline/image-layout.tsv`: `x11` 256/8 and `ns_cocoa` 240/8, every member's byte
   offset.  The guarded members are what makes the sizes differ (the X arm's `ximg`/`mask_img`
   take 32-47, where the NS arm already has `colors` at 32), and the mirror reproduces C's
   guards rather than one arm's numbers.

   There is deliberately **no** `const _` assertion here.  `xtask layout-check`'s single crate
   compiles the frozen configuration, where this record does not exist, and its C baseline
   (`docs/baseline/layout/c-layout-darwin-arm64.tsv`) has no `image` keys, so a `const _` would
   be an invented number - the same reason `glyph_string` above carries none.  Asserting these
   offsets belongs to a window-system probe crate (the `spikes/ns_inc_probe.rs` / Android probe
   shape), which is a shared-gate change; docs/ports/include-dispextern.md records it.

   Types this mirror cannot name degrade per P-72 rule 3 / P-55, each with C's prototype in the
   `/// C:` line: `Emacs_Pixmap` (X11's `Pixmap`, nsgui.h's `id`), `XImage`, `Picture`,
   `struct android_image`, `XFORM` and `struct image_type`; `next`/`prev` name this record.  */
#[cfg (feature = "have_window_system")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct image
{
  /* The time in seconds at which the image was last displayed.  Set
     in prepare_image_for_display.  */
  pub timestamp: timespec,                     // C 3219

  /* Pixmaps of the image.  */
  pub pixmap: *mut c_void,                     /// C: `Emacs_Pixmap pixmap, mask;` (dispextern.h 3222)
  pub mask: *mut c_void,
  // C 3222: `Emacs_Pixmap` is X11's `Pixmap` (an `unsigned long`) under `HAVE_X_WINDOWS` and
  // nsgui.h's `id`/`void *` under `HAVE_NS`; both are one 8-byte scalar and this mirror names
  // neither (dispextern.rs declares no `Emacs_Pixmap` - the platform mirrors do), so the member
  // is the untyped pointer C's type is one word wide as (P-72 rule 3).

  #[cfg (feature = "use_cairo")]
  pub cr_data: *mut c_void,                    // C 3225: `void *cr_data;`

  /* X images of the image, corresponding to the above Pixmaps.  Non-NULL means it and its
     Pixmap counterpart may be out of sync and the latter is outdated.  */
  #[cfg (feature = "have_x_windows")]
  pub ximg: *mut c_void,                       /// C: `XImage *ximg, *mask_img;` (dispextern.h 3232)
  #[cfg (feature = "have_x_windows")]
  pub mask_img: *mut c_void,

  /* Picture versions of pixmap and mask for compositing.  */
  #[cfg (all (feature = "have_x_windows", feature = "have_xrender", not (feature = "use_cairo")))]
  pub picture: *mut c_void,                    /// C: `Picture picture, mask_picture;` (dispextern.h 3236)
  #[cfg (all (feature = "have_x_windows", feature = "have_xrender", not (feature = "use_cairo")))]
  pub mask_picture: *mut c_void,
  // C 3232's `XImage` is Xlib's/`xterm.h`'s and C 3236's `Picture` is Xrender's; neither has a
  // mirror (`w32gui.rs`'s `_XImage` is that platform's own shim), so both stay `*mut c_void` and
  // the arm reads them through the platform mirror it already includes (P-72 rule 3).

  /* Android images of the image, corresponding to the above Pixmaps.  */
  #[cfg (feature = "have_android")]
  pub ximg: *mut c_void,                       /// C: `struct android_image *ximg, *mask_img;` (dispextern.h 3244)
  #[cfg (feature = "have_android")]
  pub mask_img: *mut c_void,
  // C 3244: `struct android_image` is androidgui.h's; androidgui.rs does not carry it (P-72
  // rule 3).

  #[cfg (feature = "have_ntgui")]
  pub xform: *mut c_void,                      /// C: `XFORM xform;` (dispextern.h 3247)
  #[cfg (feature = "have_ntgui")]
  pub smoothing: bool,                         // C 3248
  // C 3247: `XFORM` is `<windows.h>`'s `struct tagXFORM` (six `FLOAT`s), which no mirror
  // declares.  **Deviation on record**: the C member is *by value*, so P-72 rule 3's untyped
  // pointer does not preserve its 24 bytes; the pointer is used because `have_ntgui` (the MS-W32
  // target) is not a target of this lane, and a Windows probe crate that asserts this record's
  // layout has to give the member `tagXFORM`'s six floats first.

  /* The affine transformation to apply to this image.  */
  #[cfg (feature = "have_haiku")]
  pub transform: [[c_double; 3]; 3],           // C 3252: `double transform[3][3];`
  /* Whether or not bilinear filtering should be used to "smooth" the image.  */
  #[cfg (feature = "have_haiku")]
  pub use_bilinear_filtering: bool,            // C 3256

  /* Colors allocated for this image, if any.  Allocated via xmalloc.  */
  pub colors: *mut c_ulong,                    // C 3260
  pub ncolors: c_int,                          // C 3261

  /* A single `background color' for this image, for the use of anyone that cares about such a
     thing.  Only valid if the `background_valid' field is true.  */
  pub background: c_ulong,                     // C 3268

  /* Foreground and background colors of the face on which the image is created.  */
  pub face_foreground: c_ulong,                // C 3272
  pub face_background: c_ulong,

  /* Details of the font, only really relevant for types like SVG that allow us to draw text.  */
  pub face_font_size: c_int,                   // C 3276
  pub face_font_family: *mut c_char,           // C 3277

  /* Details of the font used to calculate image size relative to the canonical character size,
     with `ch' and `cw' specifiers.  */
  pub face_font_height: c_int,                 // C 3281
  pub face_font_width: c_int,                  // C 3282

  /* C 3287-3291: the record's one bitfield run - `bool_bf background_transparent : 1` (C 3287)
     and `bool_bf background_valid : 1, background_transparent_valid : 1` (C 3291).  Rust has no
     bitfields (PORTING.md 1.3/P-80), so the run is one storage unit here and the three members
     are the `IMAGE_*` shifts/masks below.  The unit is a `u32` - the four-byte unit C's
     `bool_bf` bitfields ride in - and the positions are measured on both window-system arms:
     `spikes/gated-layout-probe.py 'struct image' dispextern.h --bits` gives `x11 1/0, 1/1, 1/2`
     (raw `112:0-0`, `112:1-1`, `112:2-2`) and `ns_cocoa 1/0, 1/1, 1/2` (raw `96:0-0`,
     `96:1-1`, `96:2-2`), so **one set of shifts covers both arms**: the unit's byte offset
     differs only because of the guarded members ahead of it (112 vs 96), which this declaration
     reproduces, and the shift *within* the unit is 0/1/2 either way.  (`docs/baseline/
     image-layout.tsv`; the `frame-bits.tsv` lesson - a member can move *unit* between
     configurations - does not apply to this run.)  */
  pub bits: u32,                               // C 3287-3291

  /* Refresh counter reflecting the current version of the image.  Always larger than zero for
     images which may need refreshing.  */
  pub refresh: u32,                            // C 3296: `uint32_t refresh;`

  /* The original width and height of the image before transformations
     like scaling or rotation.  */
  pub original_width: c_int,                   // C 3300
  pub original_height: c_int,

  /* Width and height of the image.  These values depend on
     the :scale or :rotation image parameters.  */
  pub width: c_int,                            // C 3304
  pub height: c_int,

  /* The scale factor applied to the image.  */
  pub scale: c_double,                         // C 3307

  /* Top/left and bottom/right corner pixel of actual image data.  Used by four_corners_best to
     consider the real image data, rather than looking at the optional image margin.  */
  pub corners: [c_int; 4],                     // C 3317

  /* Percent of image height used as ascent.  A value of CENTERED_IMAGE_ASCENT means draw the
     image centered on the line.  */
  pub ascent: c_int,                           // C 3326

  /* Lisp specification of this image.  */
  pub spec: Lisp_Object,                       // C 3331

  /* List of "references" followed to build the image.  Typically will just contain the name of
     the image file.  Used to allow fine-grained cache flushing.  */
  pub dependencies: Lisp_Object,               // C 3336

  /* Relief to draw around the image.  */
  pub relief: c_int,                           // C 3339

  /* Optional margins around the image.  This includes the relief.  */
  pub hmargin: c_int,                          // C 3342
  pub vmargin: c_int,

  /* Reference to the type of the image.  */
  pub r#type: *const c_void,                   /// C: `struct image_type const *type;` (dispextern.h 3345)
  // C 3345: `struct image_type` is this header's own record (C 3367's region) and is not
  // mirrored yet; the pointer stays untyped (P-72 rule 3).

  /* True means that loading the image failed.  Don't try again.  */
  pub load_failed_p: bool,                     // C 3348

  /* A place for image types to store additional data.  It is marked during GC.  */
  pub lisp_data: Lisp_Object,                  // C 3352

  /* Hash value of image specification to speed up comparisons.  */
  pub hash: EMACS_UINT,                        // C 3355

  /* Image id of this image.  */
  pub id: ptrdiff_t,                           // C 3358

  /* Hash collision chain.  */
  pub next: *mut image,                        // C 3361
  pub prev: *mut image,
}

/* C's bit members of `struct image`, read and written through the storage unit `bits` above
   (PORTING.md 1.3/P-80).  The three shifts are 0/1/2 in both window-system arms, so one set of
   constants serves both (see the note on `bits`).  */
#[cfg (feature = "have_window_system")]
pub const IMAGE_BACKGROUND_TRANSPARENT_SHIFT: u32 = 0;                 // C 3287
#[cfg (feature = "have_window_system")]
pub const IMAGE_BACKGROUND_TRANSPARENT_MASK: u32 = 1 << 0;
#[cfg (feature = "have_window_system")]
pub const IMAGE_BACKGROUND_VALID_SHIFT: u32 = 1;                       // C 3291
#[cfg (feature = "have_window_system")]
pub const IMAGE_BACKGROUND_VALID_MASK: u32 = 1 << 1;
#[cfg (feature = "have_window_system")]
pub const IMAGE_BACKGROUND_TRANSPARENT_VALID_SHIFT: u32 = 2;           // C 3291
#[cfg (feature = "have_window_system")]
pub const IMAGE_BACKGROUND_TRANSPARENT_VALID_MASK: u32 = 1 << 2;

/* Cache of images.  Each frame has a cache.  X frames with the same x_display_info share their
   caches.  */
// C's `struct image_cache` (dispextern.h 3368-3385), in the same `#ifdef HAVE_WINDOW_SYSTEM`
// block as `struct image` just above (C 3203-3395), so it takes the same feature and has the
// same reading: `incomplete type` in the frozen arm, six members at 0/8/16/24/32/40 with
// `sizeof` 48 and `align` 8 in the `x11` and `ns_cocoa` arms
// (`spikes/gated-layout-probe.py 'struct image_cache' dispextern.h`).  Its two `struct image **`
// members name the record above, and `image.rs`'s port and `frame.rs`'s `image_cache` member
// (degraded to `*mut c_void` under P-72 rule 3 while this record had no owner mirror) are its
// consumers.
#[cfg (feature = "have_window_system")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct image_cache
{
  /* Hash table of images.  */
  pub buckets: *mut *mut image,                // C 3372
  /* Vector mapping image ids to images.  */
  pub images: *mut *mut image,                 // C 3375
  /* Allocated size of `images'.  */
  pub size: ptrdiff_t,                         // C 3378
  /* Number of images in the cache.  */
  pub used: ptrdiff_t,                         // C 3381
  /* Reference count (number of frames sharing this cache).  */
  pub refcount: ptrdiff_t,                     // C 3384
  /* Column width by which images whose QCscale property is Qdefault will be scaled ...  */
  pub scaling_col_width: c_int,                // C 3386
}

/* Size of bucket vector of image caches.  Should be prime.  */
// C 3387: `#define IMAGE_CACHE_BUCKETS_SIZE 1009`, in the same gated block.
#[cfg (feature = "have_window_system")]
pub const IMAGE_CACHE_BUCKETS_SIZE: c_int = 1009;

// What C's 472-2600 span still leaves unmirrored - `struct image_type` (3367), which only the
// two image records above name as a pointer - and the 2600-4026 fringe, X-widget and cursor
// machinery are not mirrored on this target yet; they need `struct frame`/`struct window`
// (`struct font` is in `include/font.rs` now), and docs/ports/include-dispextern.md tracks the
// rest of the file's macros, functions and globals.  (`struct image`, C 3215, and
// `struct image_cache`, C 3368, landed above; `struct it` and its stack, C 2386-2936, are in
// the module below.)

/* =============================================================================================
   The freely landable remainder, in C order (dispextern.h 105-3834).  Every item below is one
   that **no unit declares**: the 102 `--check` items this mirror still owes are the ones a unit
   already declares locally (the `xdisp.rs` iterator family, `keyboard.rs`'s tool/tab-bar item
   indices, `term.rs`/`xfaces.rs`'s `TTY_CAP_*`, `dispnew.rs`/`indent.rs`/`window.rs`'s text-pos
   and `IT_*` macros, the help-echo objects), and landing those here is `E0428` in those units -
   the same collision the two face records had, which status 3.220 ruled on.  The split
   (`free` vs `unit-declared`) is measured in docs/ports/include-dispextern.md's Evidence.
   ============================================================================================= */

// C 104: `#include <c-strcase.h>` - live text, materialised as the mirror that owns that header
// (PORTING.md 0.1-9).  Its top-level glob import is what puts `c_strcasecmp` in scope for C 110's
// body below, the way C's include supplies the prototype to this header.
include!("../../lib/include/c-strcase.rs");

// C 104-111's `INLINE int xstrcasecmp (...)` is the one item of this slice that a unit also
// defines: `src/font.rs` 561 carries its own copy (the shape of the time this mirror had none),
// so a copy at this mirror's top level is `E0428` there (measured: `build-tu src/font.c` -> "the
// name `xstrcasecmp` is defined multiple times").  The copy therefore sits in a module the
// including unit does not glob-import - the `bignum.rs` `bignum_externs` / `cm.rs` `cm_globals`
// shape (P-21/P-39) - so C's text is present, the unit's definition stays the live one, and the
// two do not collide.
pub mod dispextern_externs {
  use super::*;

  /// C 105-108: `INLINE int xstrcasecmp (char const *a, char const *b) { return c_strcasecmp (a, b); }`
  /// C: INLINE int xstrcasecmp (char const *a, char const *b) { return c_strcasecmp (a, b); }
  #[inline(always)]
  pub fn xstrcasecmp (a: *const c_char, b: *const c_char) -> c_int { unsafe { c_strcasecmp (a, b) } }
}

// C 176-181: `#ifndef NativeRectangle / #define NativeRectangle int / #endif` is deliberately
// **not** spelled here (no item).  `NativeRectangle` is a type macro whose answer is the window
// system's, and the platform mirrors own their `macro_rules!` (w32gui.rs 148, haikugui.rs 64,
// pgtkgui.rs 214, nsgui.rs 91, androidgui.rs 245): a second `macro_rules! NativeRectangle` in
// `layout-check`'s single crate is a macro-namespace collision with those, and a `pub type` alias
// here would hand every window-system unit this target's 8-byte `int` where C has
// `XRectangle`/`RECT`.  `glyph_string.clip` (C 1497) is `[c_int; 2]` for the same reason, and
// `docs/ports/include-dispextern.md`'s `## Decisions` records the deferral.  `--check` keeps
// reporting `missing macro NativeRectangle` while that ledger box is unchecked - the intended
// signal.

/// C 262-265: `#ifdef GLYPH_DEBUG` … `#define IF_DEBUG(X) do { (X); } while (false)` …
/// `#else` … `#define IF_DEBUG(X) ((void) 0)`.  `GLYPH_DEBUG` is undef, so the live arm is the
/// no-op; the two arms make this a branch macro, whose body is not compared (and the port's
/// no-op expansion is what C's `(void) 0` is).
macro_rules! IF_DEBUG { ($x:expr) => { let _ = $x; }; }

/// C 340-343: `#define TEXT_POS_EQUAL_P(POS1, POS2) ((POS1).charpos == (POS2).charpos &&
/// (POS1).bytepos == (POS2).bytepos)` - both arguments are `struct text_pos` *values* at every
/// call site (xdisp.c 22825/22859).
/// C: #define TEXT_POS_EQUAL_P(POS1, POS2) ((POS1).charpos == (POS2).charpos && (POS1).bytepos == (POS2).bytepos)
macro_rules! TEXT_POS_EQUAL_P {
  ($pos1:expr, $pos2:expr) => {
    (($pos1).charpos == ($pos2).charpos && ($pos1).bytepos == ($pos2).bytepos)
  };
}


/// C 1661-1662: `#define DESIRED_MODE_LINE_HEIGHT(W) MATRIX_MODE_LINE_HEIGHT ((W)->desired_matrix)`.
/// C: #define DESIRED_MODE_LINE_HEIGHT(W) MATRIX_MODE_LINE_HEIGHT ((W)->desired_matrix)
macro_rules! DESIRED_MODE_LINE_HEIGHT {
  ($w:expr) => { MATRIX_MODE_LINE_HEIGHT! ((*$w).desired_matrix) };
}
/// C 1666-1667: `#define DESIRED_HEADER_LINE_HEIGHT(W) MATRIX_HEADER_LINE_HEIGHT ((W)->desired_matrix)`.
/// C: #define DESIRED_HEADER_LINE_HEIGHT(W) MATRIX_HEADER_LINE_HEIGHT ((W)->desired_matrix)
macro_rules! DESIRED_HEADER_LINE_HEIGHT {
  ($w:expr) => { MATRIX_HEADER_LINE_HEIGHT! ((*$w).desired_matrix) };
}
/// C 1671-1672: `#define DESIRED_TAB_LINE_HEIGHT(W) MATRIX_TAB_LINE_HEIGHT ((W)->desired_matrix)`.
/// C: #define DESIRED_TAB_LINE_HEIGHT(W) MATRIX_TAB_LINE_HEIGHT ((W)->desired_matrix)
macro_rules! DESIRED_TAB_LINE_HEIGHT {
  ($w:expr) => { MATRIX_TAB_LINE_HEIGHT! ((*$w).desired_matrix) };
}

/// C 1702-1705: `#define VCENTER_BASELINE_OFFSET(FONT, F)` - C's four-term arithmetic over the
/// font and the frame's line height, `FONT_DESCENT (FRAME_FONT (F))` included.
/// C: #define VCENTER_BASELINE_OFFSET(FONT, F) (FONT_DESCENT (FONT) + (FRAME_LINE_HEIGHT (F) - FONT_HEIGHT (FONT) + (FRAME_LINE_HEIGHT (F) > FONT_HEIGHT (FONT))) / 2 - (FONT_DESCENT (FRAME_FONT (F)) - FRAME_BASELINE_OFFSET (F)))
macro_rules! VCENTER_BASELINE_OFFSET {
  ($font:expr, $f:expr) => {
    (FONT_DESCENT! ($font)
     + (FRAME_LINE_HEIGHT ($f) - FONT_HEIGHT! ($font)
        + (FRAME_LINE_HEIGHT ($f) > FONT_HEIGHT! ($font)) as c_int) / 2
     - (FONT_DESCENT! (FRAME_FONT! ($f)) - FRAME_BASELINE_OFFSET! ($f)))
  };
}

/// C 1712-1713: `#define FONT_TOO_HIGH(ft) ((ft)->pixel_size > 0 && (ft)->ascent +
/// (ft)->descent > 3*(ft)->pixel_size)` - the "preposterously large vertical space" heuristic.
/// C: #define FONT_TOO_HIGH(ft) ((ft)->pixel_size > 0 && (ft)->ascent + (ft)->descent > 3*(ft)->pixel_size)
macro_rules! FONT_TOO_HIGH {
  ($ft:expr) => {
    ((*$ft).pixel_size > 0 && (*$ft).ascent + (*$ft).descent > 3 * (*$ft).pixel_size)
  };
}

/// C 1953: `#define FACE_UNIBYTE_P(FACE) ((FACE)->charset < 0)`.
/// C: #define FACE_UNIBYTE_P(FACE) ((FACE)->charset < 0)
macro_rules! FACE_UNIBYTE_P { ($face:expr) => { (*$face).charset < 0 }; }

/// C 2011-2012: `#define FACE_EXTENSIBLE_P(F) (!NILP (F->lface[LFACE_EXTEND_INDEX]))`.
/// C: #define FACE_EXTENSIBLE_P(F) (!NILP (F->lface[LFACE_EXTEND_INDEX]))
macro_rules! FACE_EXTENSIBLE_P {
  ($f:expr) => { !NILP ((*$f).lface[LFACE_EXTEND_INDEX as usize]) };
}

/// C 2235: `#define MAX_FRINGE_BITMAPS (1<<FRINGE_ID_BITS)`.
/// C: #define MAX_FRINGE_BITMAPS (1<<FRINGE_ID_BITS)
pub const MAX_FRINGE_BITMAPS: c_int = 1 << FRINGE_ID_BITS;

/// C 2948: `#define ITERATOR_AT_END_P(IT) ((IT)->what == IT_EOB)`.
/// C: #define ITERATOR_AT_END_P(IT) ((IT)->what == IT_EOB)
macro_rules! ITERATOR_AT_END_P { ($it:expr) => { (*$it).what == IT_EOB }; }

/* C 3435-3440: the tab-bar defaults (no window system here, but the values are plain ones the
   tab-bar code reads).  */
/// C: #define DEFAULT_TAB_BAR_BUTTON_MARGIN 1
pub const DEFAULT_TAB_BAR_BUTTON_MARGIN: c_int = 1;
/// C: #define DEFAULT_TAB_BAR_BUTTON_RELIEF 1
pub const DEFAULT_TAB_BAR_BUTTON_RELIEF: c_int = 1;
/// C: #define DEFAULT_TAB_BAR_IMAGE_HEIGHT 18
pub const DEFAULT_TAB_BAR_IMAGE_HEIGHT: c_int = 18;
/// C: #define DEFAULT_TOOL_BAR_LABEL_SIZE 14
pub const DEFAULT_TOOL_BAR_LABEL_SIZE: c_int = 14;
/// C: #define DEFAULT_TOOL_BAR_BUTTON_MARGIN 4
pub const DEFAULT_TOOL_BAR_BUTTON_MARGIN: c_int = 4;
/// C: #define DEFAULT_TOOL_BAR_BUTTON_RELIEF 1
pub const DEFAULT_TOOL_BAR_BUTTON_RELIEF: c_int = 1;
/// C: #define DEFAULT_TOOL_BAR_IMAGE_HEIGHT 24
pub const DEFAULT_TOOL_BAR_IMAGE_HEIGHT: c_int = 24;

/// C 3502-3508: `enum tool_bar_item_image` - the four images a tool-bar item can show.
pub type tool_bar_item_image = c_int;
pub const TOOL_BAR_IMAGE_ENABLED_SELECTED: tool_bar_item_image = 0;
pub const TOOL_BAR_IMAGE_ENABLED_DESELECTED: tool_bar_item_image = 1;
pub const TOOL_BAR_IMAGE_DISABLED_SELECTED: tool_bar_item_image = 2;
pub const TOOL_BAR_IMAGE_DISABLED_DESELECTED: tool_bar_item_image = 3;

/// C 3699: `#define TRY_WINDOW_CHECK_MARGINS (1 << 0)`.
/// C: #define TRY_WINDOW_CHECK_MARGINS (1 << 0)
pub const TRY_WINDOW_CHECK_MARGINS: c_int = 1 << 0;
/// C 3700: `#define TRY_WINDOW_IGNORE_FONTS_CHANGE (1 << 1)`.
/// C: #define TRY_WINDOW_IGNORE_FONTS_CHANGE (1 << 1)
pub const TRY_WINDOW_IGNORE_FONTS_CHANGE: c_int = 1 << 1;

/* C 3617-3710's three globals live in the module below, not here: a unit that includes this
   mirror *defines* them (`src/fringe.c` 497 defines `max_used_fringe_bitmap`; `keyboard.c`
   defines the other two), and a top-level `extern` copy is `E0428` in exactly that unit -
   status 3.505 measured it on `src/fringe.rs`'s flip (`1 x E0428 max_used_fringe_bitmap`).
   A unit that only *reads* one spells the path (P-39), the way the module's other names work.  */

// =============================================================================================
// C 286-3834: the names of this header that the port's *units* declare themselves.
//
// The measurement in docs/ports/include-dispextern.md's `## Evidence` splits the 104 `--check`
// items 102 / 1: 102 of them are names a unit that includes this mirror already declares, so a
// copy at the mirror's top level is `E0428` in that unit - the collision status 3.220 ruled on
// for the two face records.  The shape used here is the one status 3.220 left open for names of
// this class and the one `xstrcasecmp` already uses: the copy sits in a module the including
// unit does not glob-import (the `bignum.rs` `bignum_externs` / `cm.rs` `cm_globals` shape,
// P-21/P-39).  It is preferred over a `#[cfg (feature = …)]` arm for these names because
//   * it needs no shared row (status 3.187: a feature name needs a `--cfg` producer, and
//     `femacs/xtask/Cargo.toml` plus the FLIPS rows are the integrator's), while a single
//     feature for the whole class would still hide the code from every ordinary build; and
//   * a module's contents are *compiled* by every unit that includes this mirror and by
//     `layout-check`, so C's constants, signatures and bodies are type-checked in the normal
//     build, where a cfg'd-out arm is compiled only when `dead-arms`/`gate-probes` flips it.
// The module holds three classes:
//   * the names a unit declares itself, ungated, so that every consumer and `layout-check`
//     compile them (C 286-3834);
//   * the five records of C 2159-3200 (`bidi_it`, `draw_fringe_bitmap_params`, `composition_it`,
//     `it` with `iterator_stack_entry`/`it_stack`, `redisplay_interface`), each behind the same
//     `dispextern_face_records` arm the two face records use, because a unit that includes this
//     mirror declares each of them too (their own comments carry the measured layout);
//   * and nothing else: `FACE_FOR_CHAR`, whose C body reads a `struct face` member, is carried in
//     the face-record arm itself (C 2030-2038).
//
// Every item keeps C's text in a `/// C:` line; the enum members are `pub const`s of C's
// underlying `int` (PORTING.md 1.3), the objects are `extern` storage declarations, and the
// macros are `macro_rules!` with C's own bodies.
pub mod dispextern_unit_names {
  use super::*;

  /* C 115's tag, `struct x_display_info`, whose record is `xterm.h`'s: this mirror needs only
     the incomplete tag C declares (a pointer target), and the opaque type is the port's
     spelling of it.  See the alias at the top level for why it lives here.  */
  #[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
  pub enum x_display_info {}

  /* C 286-343: the text-position accessors.  `BYTEPOS`/`CHARPOS` are usable as assignment
     targets in C, and `macro_rules!` keeps that property (`BYTEPOS! (pos) = n`).  */
  /// C: #define BYTEPOS(POS)	(POS).bytepos
  macro_rules! BYTEPOS {
    ($pos:expr) => { ($pos).bytepos };
  }
  /// C: #define CHARPOS(POS)	(POS).charpos
  macro_rules! CHARPOS {
    ($pos:expr) => { ($pos).charpos };
  }
  /// C: #define SET_TEXT_POS(POS, CHARPOS, BYTEPOS) ((POS).charpos = (CHARPOS), (POS).bytepos = BYTEPOS)
  macro_rules! SET_TEXT_POS {
    ($pos:expr, $charpos:expr, $bytepos:expr) => {
      { ($pos).charpos = ($charpos); ($pos).bytepos = ($bytepos); }
    };
  }
  /// C: #define INC_TEXT_POS(POS, MULTIBYTE_P) do { ++(POS).charpos; if (MULTIBYTE_P) (POS).bytepos += next_char_len ((POS).bytepos); else ++(POS).bytepos; } while (false)
  macro_rules! INC_TEXT_POS {
    ($pos:expr, $multibyte_p:expr) => { {
      ($pos).charpos += 1;
      if $multibyte_p
        { ($pos).bytepos += next_char_len (($pos).bytepos); }
      else
        { ($pos).bytepos += 1; }
    } };
  }
  /// C: #define DEC_TEXT_POS(POS, MULTIBYTE_P) do { --(POS).charpos; if (MULTIBYTE_P) (POS).bytepos -= prev_char_len ((POS).bytepos); else --(POS).bytepos; } while (false)
  macro_rules! DEC_TEXT_POS {
    ($pos:expr, $multibyte_p:expr) => { {
      ($pos).charpos -= 1;
      if $multibyte_p
        { ($pos).bytepos -= prev_char_len (($pos).bytepos); }
      else
        { ($pos).bytepos -= 1; }
    } };
  }
  /// C: #define SET_TEXT_POS_FROM_MARKER(POS, MARKER) (CHARPOS (POS) = marker_position (MARKER), BYTEPOS (POS) = marker_byte_position (MARKER))
  macro_rules! SET_TEXT_POS_FROM_MARKER {
    ($pos:expr, $marker:expr) => {
      { CHARPOS! ($pos) = marker_position ($marker);
        BYTEPOS! ($pos) = marker_byte_position ($marker); }
    };
  }
  /// C: #define CLIP_TEXT_POS_FROM_MARKER(POS, MARKER) (CHARPOS (POS) = clip_to_bounds (BEGV, marker_position (MARKER), ZV), BYTEPOS (POS) = clip_to_bounds (BEGV_BYTE, marker_byte_position (MARKER), ZV_BYTE))
  macro_rules! CLIP_TEXT_POS_FROM_MARKER {
    ($pos:expr, $marker:expr) => { {
      CHARPOS! ($pos) = clip_to_bounds (BEGV! (), marker_position ($marker), ZV! ());
      BYTEPOS! ($pos) = clip_to_bounds (BEGV_BYTE! (), marker_byte_position ($marker), ZV_BYTE! ());
    } };
  }

  /// C: #define SET_MARKER_FROM_TEXT_POS(MARKER, POS) set_marker_both (MARKER, Qnil, CHARPOS (POS), BYTEPOS (POS))
  macro_rules! SET_MARKER_FROM_TEXT_POS {
    ($marker:expr, $pos:expr) => {
      set_marker_both! ($marker, Qnil! (), CHARPOS! ($pos), BYTEPOS! ($pos))
    };
  }

  /* C 414-420: the glyph-code setter (the `SET_GLYPH` pair it calls is this mirror's).  */
  /// C: #define SET_GLYPH_FROM_GLYPH_CODE(glyph, gc) do { if (CONSP (gc)) SET_GLYPH (glyph, XFIXNUM (XCAR (gc)), XFIXNUM (XCDR (gc))); else SET_GLYPH (glyph, (XFIXNUM (gc) & ((1 << CHARACTERBITS)-1)), XFIXNUM (gc) >> CHARACTERBITS); } while (false)
  macro_rules! SET_GLYPH_FROM_GLYPH_CODE {
    ($glyph:expr, $gc:expr) => { {
      if CONSP ($gc)
        { SET_GLYPH! ($glyph, XFIXNUM (XCAR ($gc)) as c_int, XFIXNUM (XCDR ($gc)) as c_int); }
      else
        { SET_GLYPH! ($glyph, (XFIXNUM ($gc) & ((1 << CHARACTERBITS) - 1)) as c_int,
                      (XFIXNUM ($gc) >> CHARACTERBITS) as c_int); }
    } };
  }

  /* C 648-650: the "space in the default face on FRAME" test.  */
  /// C: # define CHAR_GLYPH_SPACE_P(FRAME, GLYPH) ((GLYPH).u.ch == SPACEGLYPH && (GLYPH).face_id == DEFAULT_FACE_ID && (GLYPH).frame == (FRAME))
  macro_rules! CHAR_GLYPH_SPACE_P {
    ($frame:expr, $glyph:expr) => {
      (($glyph).u.ch == SPACEGLYPH as c_uint
       && glyph_bits! (face_id, $glyph) == DEFAULT_FACE_ID
       && ($glyph).frame == ($frame) as *mut c_void)
    };
  }

  /* C 1932-1949: the three tty "default colour" sentinels and the predicate that reads them.
     C spells them `((unsigned long) -1/-2/-3)`; the port's spelling is the one term.rs and
     frame.rs already use.  */
  /// C: #define FACE_TTY_DEFAULT_COLOR ((unsigned long) -1)
  pub const FACE_TTY_DEFAULT_COLOR: c_ulong = -1i64 as c_ulong;
  /// C: #define FACE_TTY_DEFAULT_FG_COLOR ((unsigned long) -2)
  pub const FACE_TTY_DEFAULT_FG_COLOR: c_ulong = -2i64 as c_ulong;
  /// C: #define FACE_TTY_DEFAULT_BG_COLOR ((unsigned long) -3)
  pub const FACE_TTY_DEFAULT_BG_COLOR: c_ulong = -3i64 as c_ulong;
  /// C: INLINE bool face_tty_specified_color (unsigned long color) { return color < FACE_TTY_DEFAULT_BG_COLOR; }
  #[inline(always)]
  pub fn face_tty_specified_color (color: c_ulong) -> bool
  {
    color < FACE_TTY_DEFAULT_BG_COLOR
  }

  /* C 2015-2022: `HAVE_WINDOW_SYSTEM` is undef, so C's body is `return true;`.  The parameter
     is C's `struct face *`; `struct face` is behind `dispextern_face_records` in this mirror,
     so the pointer is the untyped one P-72 ③ prescribes.  */
  /// C: INLINE bool FACE_SUITABLE_FOR_ASCII_CHAR_P (struct face *face) { return true; }
  #[inline(always)]
  pub fn FACE_SUITABLE_FOR_ASCII_CHAR_P (face: *mut c_void) -> bool
  {
    let _ = face;
    true
  }

  /* C 2068: the global the redisplay path reads.  */
  unsafe extern "C" {
    /// C: extern bool face_change;
    pub static mut face_change: bool;
  }

  /* C 3617-3710: the three globals this header declares.  They are here rather than at the
     mirror's top level for the reason in the module's own doc: the including unit defines them
     (`src/fringe.c` 497, `keyboard.c`), so a top-level copy collides (status 3.505).  */
  unsafe extern "C" {
    /// C 3617: `extern int last_tab_bar_item;` (the unit that defines it is keyboard.c's).
    pub static mut last_tab_bar_item: c_int;
    /// C 3618: `extern int last_tool_bar_item;`
    pub static mut last_tool_bar_item: c_int;
    /// C 3710: `extern int max_used_fringe_bitmap;` (defined by `src/fringe.c` 497).
    pub static mut max_used_fringe_bitmap: c_int;
  }

  /* C 2204-2207: the base-embedding-level test.  */
  /// C: #define BIDI_AT_BASE_LEVEL(BIDI_IT) ((BIDI_IT).resolved_level == (BIDI_IT).level_stack[0].level)
  macro_rules! BIDI_AT_BASE_LEVEL {
    ($bidi_it:expr) => {
      (($bidi_it).resolved_level == ($bidi_it).level_stack[0].level)
    };
  }

  /* ---- C 2159-2240: the bidi iterator's state and the fringe-bitmap request -----------------
     Both are records a unit that includes this mirror declares itself (`struct bidi_it` is
     `src/bidi.rs` 86, `struct draw_fringe_bitmap_params` is no unit's yet), so they are carried
     behind `dispextern_face_records` - the arm status 3.220 chose for the records the units keep.
     Every `sizeof`, `align` and member offset below is the frozen clang's own measurement of the
     reference header (`clang -std=gnu23 -O0 -I build/emacs-ref/{lib,src} -I emacs/{src,lib}`, an
     `offsetof` probe over `struct bidi_it`/`struct draw_fringe_bitmap_params`; bit positions come
     from a second probe that sets one bit and reads the byte back), and the assertions repeat the
     same numbers, so a moved member fails the build rather than the ledger.  */
#[cfg (feature = "dispextern_face_records")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct bidi_it
{
  pub bytepos: ptrdiff_t,                  // offset 0
  pub charpos: ptrdiff_t,                  // offset 8
  pub ch: c_int,                  // offset 16
  pub nchars: ptrdiff_t,                  // offset 24
  pub ch_len: ptrdiff_t,                  // offset 32
  pub r#type: bidi_type_t,                  // offset 40
  pub type_after_wn: bidi_type_t,                  // offset 44
  pub orig_type: bidi_type_t,                  // offset 48
  pub resolved_level: i8,                  // offset 52
  pub isolate_level: i8,                  // offset 53
  pub invalid_levels: ptrdiff_t,                  // offset 56
  pub invalid_isolates: ptrdiff_t,                  // offset 64
  pub prev: bidi_saved_info,                  // offset 72
  pub last_strong: bidi_saved_info,                  // offset 88
  pub next_for_neutral: bidi_saved_info,                  // offset 104
  pub prev_for_neutral: bidi_saved_info,                  // offset 120
  pub next_for_ws: bidi_saved_info,                  // offset 136
  pub bracket_pairing_pos: ptrdiff_t,                  // offset 152
  pub bracket_enclosed_type: bidi_type_t,                  // offset 160
  pub next_en_pos: ptrdiff_t,                  // offset 168
  pub next_en_type: bidi_type_t,                  // offset 176
  pub sos: bidi_dir_t,                  // offset 180
  pub scan_dir: c_int,                  // offset 184
  pub disp_pos: ptrdiff_t,                  // offset 192
  pub disp_prop: c_int,                  // offset 200
  pub stack_idx: c_int,                  // offset 204
  pub level_stack: [bidi_stack; (BIDI_MAXDEPTH + 2 + 1) as usize],                  // offset 208
  pub string: bidi_string_data,                  // offset 2256
  pub w: *mut c_void,                  // offset 2296
  pub paragraph_dir: bidi_dir_t,                  // offset 2304
  pub separator_limit: ptrdiff_t,                  // offset 2312
  /// C 2159-2203's bitfield run, stored as `bits` (PORTING.md 1.3): offset 2320.
  pub bits: u8,
}
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::size_of::<bidi_it> () == 2328);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::align_of::<bidi_it> () == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, bytepos) == 0);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, charpos) == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, ch) == 16);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, nchars) == 24);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, ch_len) == 32);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, r#type) == 40);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, type_after_wn) == 44);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, orig_type) == 48);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, resolved_level) == 52);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, isolate_level) == 53);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, invalid_levels) == 56);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, invalid_isolates) == 64);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, prev) == 72);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, last_strong) == 88);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, next_for_neutral) == 104);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, prev_for_neutral) == 120);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, next_for_ws) == 136);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, bracket_pairing_pos) == 152);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, bracket_enclosed_type) == 160);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, next_en_pos) == 168);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, next_en_type) == 176);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, sos) == 180);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, scan_dir) == 184);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, disp_pos) == 192);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, disp_prop) == 200);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, stack_idx) == 204);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, level_stack) == 208);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, string) == 2256);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, w) == 2296);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, paragraph_dir) == 2304);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, separator_limit) == 2312);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (bidi_it, bits) == 2320);

#[cfg (feature = "dispextern_face_records")]
#[repr (C)]
#[derive (Clone, Copy)]
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
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::size_of::<draw_fringe_bitmap_params> () == 64);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::align_of::<draw_fringe_bitmap_params> () == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, which) == 0);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, bits) == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, wd) == 16);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, h) == 20);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, dh) == 24);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, x) == 28);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, y) == 32);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, bx) == 36);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, nx) == 40);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, by) == 44);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, ny) == 48);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, flags) == 52);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (draw_fringe_bitmap_params, face) == 56);

/* C 2230-2231: the record's two `bool_bf : 1` members, which PORTING.md 1.3 stores in the
   `flags` byte above - bit 0 and bit 1 of offset 52 (`abi_shim.rs` 2185-2205's offset probe
   measured them, and `docs/ports/w32term.md` owed 5 asks for the accessor pair).  C reads and
   writes them as members; the port's spelling of the same rule is one constant plus the
   accessors below (the `include/termhooks.rs` `tty->…bit` reading).  */
/// C: `bool_bf cursor_p : 1;`
#[cfg (feature = "dispextern_face_records")]
pub const DRAW_FRINGE_BITMAP_CURSOR_P: u8 = 1 << 0;
/// C: `bool_bf overlay_p : 1;`
#[cfg (feature = "dispextern_face_records")]
pub const DRAW_FRINGE_BITMAP_OVERLAY_P: u8 = 1 << 1;

#[cfg (feature = "dispextern_face_records")]
#[inline(always)]
pub fn get_cursor_p (p: *const draw_fringe_bitmap_params) -> bool
{ unsafe { (*p).flags & DRAW_FRINGE_BITMAP_CURSOR_P != 0 } }
#[cfg (feature = "dispextern_face_records")]
#[inline(always)]
pub fn set_cursor_p (p: *mut draw_fringe_bitmap_params, value: bool)
{
  unsafe {
    if value { (*p).flags |= DRAW_FRINGE_BITMAP_CURSOR_P }
    else { (*p).flags &= !DRAW_FRINGE_BITMAP_CURSOR_P }
  }
}
#[cfg (feature = "dispextern_face_records")]
#[inline(always)]
pub fn get_overlay_p (p: *const draw_fringe_bitmap_params) -> bool
{ unsafe { (*p).flags & DRAW_FRINGE_BITMAP_OVERLAY_P != 0 } }
#[cfg (feature = "dispextern_face_records")]
#[inline(always)]
pub fn set_overlay_p (p: *mut draw_fringe_bitmap_params, value: bool)
{
  unsafe {
    if value { (*p).flags |= DRAW_FRINGE_BITMAP_OVERLAY_P }
    else { (*p).flags &= !DRAW_FRINGE_BITMAP_OVERLAY_P }
  }
}

  /* C 2285-2323: `enum display_element_type`, C's `int` enumerator.  */
  /// C: enum display_element_type { IT_CHARACTER,
  pub const IT_CHARACTER: c_int = 0;
  /// C:   IT_COMPOSITION,
  pub const IT_COMPOSITION: c_int = 1;
  /// C:   IT_GLYPHLESS,
  pub const IT_GLYPHLESS: c_int = 2;
  /// C:   IT_IMAGE,
  pub const IT_IMAGE: c_int = 3;
  /// C:   IT_STRETCH,
  pub const IT_STRETCH: c_int = 4;
  /// C:   IT_EOB,
  pub const IT_EOB: c_int = 5;
  /// C:   IT_TRUNCATION,
  pub const IT_TRUNCATION: c_int = 6;
  /// C:   IT_CONTINUATION,
  pub const IT_CONTINUATION: c_int = 7;
  /// C:   IT_XWIDGET };
  pub const IT_XWIDGET: c_int = 8;

  /* C 2323-2340: `enum prop_idx` - one slot per display-relevant text property.  */
  /// C: enum prop_idx { FONTIFIED_PROP_IDX,
  pub const FONTIFIED_PROP_IDX: c_int = 0;
  /// C:   FACE_PROP_IDX,
  pub const FACE_PROP_IDX: c_int = 1;
  /// C:   INVISIBLE_PROP_IDX,
  pub const INVISIBLE_PROP_IDX: c_int = 2;
  /// C:   DISPLAY_PROP_IDX,
  pub const DISPLAY_PROP_IDX: c_int = 3;
  /// C:   COMPOSITION_PROP_IDX,
  pub const COMPOSITION_PROP_IDX: c_int = 4;
  /// C:   OVERLAY_PROP_IDX,
  pub const OVERLAY_PROP_IDX: c_int = 5;
  /// C:   LAST_PROP_IDX };
  pub const LAST_PROP_IDX: c_int = 6;

  /* C 2338-2343: `enum line_wrap_method`.  */
  /// C: enum line_wrap_method { TRUNCATE,
  pub const TRUNCATE: c_int = 0;
  /// C:   WORD_WRAP,
  pub const WORD_WRAP: c_int = 1;
  /// C:   WINDOW_WRAP };
  pub const WINDOW_WRAP: c_int = 2;

  /* C 2347-2365: `enum glyphless_display_method`.  */
  /// C: enum glyphless_display_method { GLYPHLESS_DISPLAY_THIN_SPACE,
  pub const GLYPHLESS_DISPLAY_THIN_SPACE: c_int = 0;
  /// C:   GLYPHLESS_DISPLAY_EMPTY_BOX,
  pub const GLYPHLESS_DISPLAY_EMPTY_BOX: c_int = 1;
  /// C:   GLYPHLESS_DISPLAY_ACRONYM,
  pub const GLYPHLESS_DISPLAY_ACRONYM: c_int = 2;
  /// C:   GLYPHLESS_DISPLAY_HEX_CODE };
  pub const GLYPHLESS_DISPLAY_HEX_CODE: c_int = 3;

  /* C 2371-2380: `enum it_method` - the input source the iterator is reading from.  */
  /// C: enum it_method { GET_FROM_BUFFER = 0,
  pub const GET_FROM_BUFFER: c_int = 0;
  /// C:   GET_FROM_DISPLAY_VECTOR,
  pub const GET_FROM_DISPLAY_VECTOR: c_int = 1;
  /// C:   GET_FROM_STRING,
  pub const GET_FROM_STRING: c_int = 2;
  /// C:   GET_FROM_C_STRING,
  pub const GET_FROM_C_STRING: c_int = 3;
  /// C:   GET_FROM_IMAGE,
  pub const GET_FROM_IMAGE: c_int = 4;
  /// C:   GET_FROM_STRETCH,
  pub const GET_FROM_STRETCH: c_int = 5;
  /// C:   GET_FROM_XWIDGET,
  pub const GET_FROM_XWIDGET: c_int = 6;
  /// C:   NUM_IT_METHODS };
  pub const NUM_IT_METHODS: c_int = 7;

  /* C 2383: the iterator's display-element stack depth.  */
  /// C: #define IT_STACK_SIZE 5
  pub const IT_STACK_SIZE: c_int = 5;

  /* ---- C 2386-2936: the display iterator's records ------------------------------------------
     Same arm and same measurement as the block above.  `struct iterator_stack_entry` stands here,
     just before `struct it`, although C defines it *inside* `struct it` (C 2588-2658): Rust has no
     nested record definitions.  `it`'s four bitfield runs are `bits` (C 2477-2485, eight flags in
     the byte at 112), `bits2` (C 2696-2707, thirteen flags in the two bytes at 2216), `bits3`
     (C 2915-2932, the two 16-bit fringe ids, the two 20-bit fringe face ids and `bidi_p` in the
     spare bit of their third storage unit - 12 bytes at 2604, which is why `bidi_p` has no byte of
     its own), and no fourth run.  The pointers C types against `window`/`frame`/`Lisp_Char_Table`/
     `xwidget` are untyped here (P-72 ③: this arm carries none of those records).  */
#[cfg (feature = "dispextern_face_records")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct composition_it
{
  pub stop_pos: ptrdiff_t,                  // offset 0
  pub id: ptrdiff_t,                  // offset 8
  pub ch: c_int,                  // offset 16
  pub rule_idx: EMACS_INT,                  // offset 24
  pub lookback: ptrdiff_t,                  // offset 32
  pub nglyphs: c_int,                  // offset 40
  pub reversed_p: bool,                  // offset 44
  pub parent_it: *mut it,                  // offset 48
  pub charpos: ptrdiff_t,                  // offset 56
  pub nchars: c_int,                  // offset 64
  pub nbytes: c_int,                  // offset 68
  pub from: c_int,                  // offset 72
  pub to: c_int,                  // offset 76
  pub width: c_int,                  // offset 80
}
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::size_of::<composition_it> () == 88);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::align_of::<composition_it> () == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, stop_pos) == 0);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, id) == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, ch) == 16);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, rule_idx) == 24);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, lookback) == 32);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, nglyphs) == 40);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, reversed_p) == 44);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, parent_it) == 48);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, charpos) == 56);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, nchars) == 64);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, nbytes) == 68);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, from) == 72);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, to) == 76);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (composition_it, width) == 80);

  // C 2604-2619: the stack entry's `union { ... } u;`.  C's union and its three anonymous
  // `struct` arms have no names, so the port names the union after its member and each arm after
  // the union and its member (P-67, the `glyph_u`/`glyph_slice_union` shape).  Measured: the
  // union is 48 bytes (`u` at 144, `position` at 192), the `image` arm is 48
  // (`object`@0, `slice`@8, `image_id`@40), and both one-member arms are 8.
  #[cfg (feature = "dispextern_face_records")]
  #[repr (C)]
  #[derive (Clone, Copy)]
  pub struct iterator_stack_entry_u_image
  {
    pub object: Lisp_Object,                  // offset 0
    pub slice: it_slice,                      // offset 8
    pub image_id: ptrdiff_t,                  // offset 40
  }
  #[cfg (feature = "dispextern_face_records")]
  const _: () = core::assert! (core::mem::size_of::<iterator_stack_entry_u_image> () == 48);
  #[cfg (feature = "dispextern_face_records")]
  const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry_u_image, slice) == 8);
  #[cfg (feature = "dispextern_face_records")]
  const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry_u_image, image_id) == 40);
  #[cfg (feature = "dispextern_face_records")]
  #[repr (C)]
  #[derive (Clone, Copy)]
  pub struct iterator_stack_entry_u_stretch
  {
    pub object: Lisp_Object,                  // offset 0
  }
  #[cfg (feature = "dispextern_face_records")]
  #[repr (C)]
  #[derive (Clone, Copy)]
  pub struct iterator_stack_entry_u_xwidget
  {
    pub object: Lisp_Object,                  // offset 0
  }
  #[cfg (feature = "dispextern_face_records")]
  #[repr (C)]
  #[derive (Clone, Copy)]
  pub union iterator_stack_entry_u
  {
    pub image: iterator_stack_entry_u_image,
    pub stretch: iterator_stack_entry_u_stretch,
    pub xwidget: iterator_stack_entry_u_xwidget,
  }
  #[cfg (feature = "dispextern_face_records")]
  const _: () = core::assert! (core::mem::size_of::<iterator_stack_entry_u> () == 48);
  #[cfg (feature = "dispextern_face_records")]
  const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry_u, stretch) == 0);
  #[cfg (feature = "dispextern_face_records")]
  const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry_u, xwidget) == 0);

#[cfg (feature = "dispextern_face_records")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct iterator_stack_entry
{
  pub string: Lisp_Object,                  // offset 0
  pub string_nchars: c_int,                  // offset 8
  pub end_charpos: ptrdiff_t,                  // offset 16
  pub stop_charpos: ptrdiff_t,                  // offset 24
  pub prev_stop: ptrdiff_t,                  // offset 32
  pub base_level_stop: ptrdiff_t,                  // offset 40
  pub cmp_it: composition_it,                  // offset 48
  pub face_id: c_int,                  // offset 136
  pub u: iterator_stack_entry_u,                  // offset 144
  pub position: text_pos,                  // offset 192
  pub current: display_pos,                  // offset 208
  pub from_overlay: Lisp_Object,                  // offset 256
  pub area: c_int,                  // offset 264
  pub method: c_int,                  // offset 268
  pub paragraph_embedding: bidi_dir_t,                  // offset 272
  /// C 2588-2658's bitfield run, stored as `bits` (PORTING.md 1.3): offset 276.
  pub bits: u8,
  pub line_wrap: c_int,                  // offset 280
  pub voffset: c_short,                  // offset 284
  pub space_width: Lisp_Object,                  // offset 288
  pub font_height: Lisp_Object,                  // offset 296
}
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::size_of::<iterator_stack_entry> () == 304);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::align_of::<iterator_stack_entry> () == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, string) == 0);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, string_nchars) == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, end_charpos) == 16);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, stop_charpos) == 24);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, prev_stop) == 32);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, base_level_stop) == 40);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, cmp_it) == 48);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, face_id) == 136);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, u) == 144);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, position) == 192);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, current) == 208);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, from_overlay) == 256);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, area) == 264);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, method) == 268);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, paragraph_embedding) == 272);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, bits) == 276);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, line_wrap) == 280);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, voffset) == 284);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, space_width) == 288);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (iterator_stack_entry, font_height) == 296);

#[cfg (feature = "dispextern_face_records")]
// C 2643-2644: `struct iterator_stack_entry stack[IT_STACK_SIZE];` - an array of five entries
// (5 x 304 = 1520 bytes, measured: `stack` at 672, `sp` at 2192).  C defines the entry type
// inline, so its members are C fields of `it` itself; the port needs a *named* element type and
// the field-member check follows named types only, so the array is spelled as a named head entry
// plus the remaining four - same bytes, same offsets.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct it_stack
{
  pub first: iterator_stack_entry,
  pub rest: [iterator_stack_entry; (IT_STACK_SIZE - 1) as usize],
}
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::size_of::<it_stack> () == 1520);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it_stack, rest) == 304);

#[cfg (feature = "dispextern_face_records")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct it
{
  pub window: Lisp_Object,                  // offset 0
  pub w: *mut c_void,                  // offset 8
  pub f: *mut c_void,                  // offset 16
  pub method: c_int,                  // offset 24
  pub stop_charpos: ptrdiff_t,                  // offset 32
  pub prev_stop: ptrdiff_t,                  // offset 40
  pub base_level_stop: ptrdiff_t,                  // offset 48
  pub end_charpos: ptrdiff_t,                  // offset 56
  pub medium_narrowing_begv: ptrdiff_t,                  // offset 64
  pub medium_narrowing_zv: ptrdiff_t,                  // offset 72
  pub large_narrowing_begv: ptrdiff_t,                  // offset 80
  pub large_narrowing_zv: ptrdiff_t,                  // offset 88
  pub s: *const u8,                  // offset 96
  pub string_nchars: ptrdiff_t,                  // offset 104
  /// C 2429-2936's bitfield run, stored as `bits` (PORTING.md 1.3): offset 112.
  pub bits: u8,
  pub dp: *mut c_void,                  // offset 120
  pub dpvec: *mut Lisp_Object,                  // offset 128
  pub dpend: *mut Lisp_Object,                  // offset 136
  pub dpvec_char_len: c_int,                  // offset 144
  pub dpvec_face_id: c_int,                  // offset 148
  pub saved_face_id: c_int,                  // offset 152
  pub ctl_chars: [Lisp_Object; 16],                  // offset 160
  pub start: display_pos,                  // offset 288
  pub current: display_pos,                  // offset 336
  pub n_overlay_strings: ptrdiff_t,                  // offset 384
  pub overlay_strings_charpos: ptrdiff_t,                  // offset 392
  pub overlay_strings: [Lisp_Object; (OVERLAY_STRING_CHUNK_SIZE as usize)],                  // offset 400
  pub string_overlays: [Lisp_Object; (OVERLAY_STRING_CHUNK_SIZE as usize)],                  // offset 528
  pub string: Lisp_Object,                  // offset 656
  pub from_overlay: Lisp_Object,                  // offset 664
  pub stack: it_stack,                  // offset 672
  pub sp: c_int,                  // offset 2192
  pub selective: ptrdiff_t,                  // offset 2200
  pub what: c_int,                  // offset 2208
  pub face_id: c_int,                  // offset 2212
  /// C 2429-2936's bitfield run, stored as `bits2` (PORTING.md 1.3): offset 2216.
  pub bits2: [u8; 2],
  pub line_wrap: c_int,                  // offset 2220
  pub base_face_id: c_int,                  // offset 2224
  pub c: c_int,                  // offset 2228
  pub len: c_int,                  // offset 2232
  pub cmp_it: composition_it,                  // offset 2240
  pub char_to_display: c_int,                  // offset 2328
  pub glyphless_method: c_int,                  // offset 2332
  pub image_id: ptrdiff_t,                  // offset 2336
  pub xwidget: *mut c_void,                  // offset 2344
  pub slice: it_slice,                  // offset 2352
  pub space_width: Lisp_Object,                  // offset 2384
  pub voffset: c_short,                  // offset 2392
  pub tab_width: c_short,                  // offset 2394
  pub font_height: Lisp_Object,                  // offset 2400
  pub object: Lisp_Object,                  // offset 2408
  pub position: text_pos,                  // offset 2416
  pub truncation_pixel_width: c_short,                  // offset 2432
  pub continuation_pixel_width: c_short,                  // offset 2434
  pub first_visible_x: c_int,                  // offset 2436
  pub last_visible_x: c_int,                  // offset 2440
  pub last_visible_y: c_int,                  // offset 2444
  pub extra_line_spacing: c_int,                  // offset 2448
  pub extra_line_spacing_above: c_int,                  // offset 2452
  pub max_extra_line_spacing: c_int,                  // offset 2456
  pub override_ascent: c_int,                  // offset 2460
  pub override_descent: c_int,                  // offset 2464
  pub override_boff: c_int,                  // offset 2468
  pub glyph_row: *mut glyph_row,                  // offset 2472
  pub area: c_int,                  // offset 2480
  pub nglyphs: c_int,                  // offset 2484
  pub pixel_width: c_int,                  // offset 2488
  pub ascent: c_int,                  // offset 2492
  pub descent: c_int,                  // offset 2496
  pub max_ascent: c_int,                  // offset 2500
  pub max_descent: c_int,                  // offset 2504
  pub phys_ascent: c_int,                  // offset 2508
  pub phys_descent: c_int,                  // offset 2512
  pub max_phys_ascent: c_int,                  // offset 2516
  pub max_phys_descent: c_int,                  // offset 2520
  pub current_x: c_int,                  // offset 2524
  pub wrap_prefix_width: c_int,                  // offset 2528
  pub continuation_lines_width: c_int,                  // offset 2532
  pub eol_pos: text_pos,                  // offset 2536
  pub current_y: c_int,                  // offset 2552
  pub first_vpos: c_int,                  // offset 2556
  pub vpos: c_int,                  // offset 2560
  pub hpos: c_int,                  // offset 2564
  pub lnum: ptrdiff_t,                  // offset 2568
  pub lnum_bytepos: ptrdiff_t,                  // offset 2576
  pub lnum_width: c_int,                  // offset 2584
  pub lnum_pixel_width: c_int,                  // offset 2588
  pub pt_lnum: ptrdiff_t,                  // offset 2592
  pub stretch_adjust: c_int,                  // offset 2600
  /// C 2429-2936's bitfield run, stored as `bits3` (PORTING.md 1.3): offset 2604.
  pub bits3: [u32; 3],
  pub bidi_it: bidi_it,                  // offset 2616
  pub paragraph_embedding: bidi_dir_t,                  // offset 4944
  pub min_width_property: Lisp_Object,                  // offset 4952
  pub min_width_start: c_int,                  // offset 4960
}
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::size_of::<it> () == 4968);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::align_of::<it> () == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, window) == 0);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, w) == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, f) == 16);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, method) == 24);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, stop_charpos) == 32);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, prev_stop) == 40);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, base_level_stop) == 48);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, end_charpos) == 56);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, medium_narrowing_begv) == 64);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, medium_narrowing_zv) == 72);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, large_narrowing_begv) == 80);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, large_narrowing_zv) == 88);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, s) == 96);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, string_nchars) == 104);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, bits) == 112);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, dp) == 120);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, dpvec) == 128);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, dpend) == 136);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, dpvec_char_len) == 144);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, dpvec_face_id) == 148);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, saved_face_id) == 152);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, ctl_chars) == 160);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, start) == 288);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, current) == 336);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, n_overlay_strings) == 384);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, overlay_strings_charpos) == 392);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, overlay_strings) == 400);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, string_overlays) == 528);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, string) == 656);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, from_overlay) == 664);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, stack) == 672);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, sp) == 2192);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, selective) == 2200);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, what) == 2208);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, face_id) == 2212);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, bits2) == 2216);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, line_wrap) == 2220);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, base_face_id) == 2224);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, c) == 2228);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, len) == 2232);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, cmp_it) == 2240);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, char_to_display) == 2328);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, glyphless_method) == 2332);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, image_id) == 2336);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, xwidget) == 2344);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, slice) == 2352);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, space_width) == 2384);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, voffset) == 2392);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, tab_width) == 2394);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, font_height) == 2400);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, object) == 2408);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, position) == 2416);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, truncation_pixel_width) == 2432);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, continuation_pixel_width) == 2434);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, first_visible_x) == 2436);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, last_visible_x) == 2440);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, last_visible_y) == 2444);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, extra_line_spacing) == 2448);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, extra_line_spacing_above) == 2452);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, max_extra_line_spacing) == 2456);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, override_ascent) == 2460);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, override_descent) == 2464);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, override_boff) == 2468);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, glyph_row) == 2472);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, area) == 2480);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, nglyphs) == 2484);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, pixel_width) == 2488);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, ascent) == 2492);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, descent) == 2496);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, max_ascent) == 2500);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, max_descent) == 2504);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, phys_ascent) == 2508);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, phys_descent) == 2512);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, max_phys_ascent) == 2516);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, max_phys_descent) == 2520);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, current_x) == 2524);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, wrap_prefix_width) == 2528);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, continuation_lines_width) == 2532);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, eol_pos) == 2536);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, current_y) == 2552);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, first_vpos) == 2556);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, vpos) == 2560);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, hpos) == 2564);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, lnum) == 2568);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, lnum_bytepos) == 2576);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, lnum_width) == 2584);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, lnum_pixel_width) == 2588);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, pt_lnum) == 2592);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, stretch_adjust) == 2600);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, bits3) == 2604);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, bidi_it) == 2616);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, paragraph_embedding) == 4944);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, min_width_property) == 4952);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (it, min_width_start) == 4960);

  /* C 2572: `struct it`'s overlay-string chunk.  */
  /// C: #define OVERLAY_STRING_CHUNK_SIZE 16
  pub const OVERLAY_STRING_CHUNK_SIZE: c_int = 16;

  /* C 2937-2943: the iterator position accessors.  */
  /// C: #define IT_CHARPOS(IT)		CHARPOS ((IT).current.pos)
  macro_rules! IT_CHARPOS {
    ($it:expr) => { CHARPOS! (($it).current.pos) };
  }
  /// C: #define IT_BYTEPOS(IT)		BYTEPOS ((IT).current.pos)
  macro_rules! IT_BYTEPOS {
    ($it:expr) => { BYTEPOS! (($it).current.pos) };
  }
  /// C: #define IT_STRING_BYTEPOS(IT)	BYTEPOS ((IT).current.string_pos)
  macro_rules! IT_STRING_BYTEPOS {
    ($it:expr) => { BYTEPOS! (($it).current.string_pos) };
  }

  /// C: #define IT_STRING_CHARPOS(IT)	CHARPOS ((IT).current.string_pos)
  macro_rules! IT_STRING_CHARPOS {
    ($it:expr) => { CHARPOS! (($it).current.string_pos) };
  }
  /// C: #define ITERATOR_AT_END_OF_LINE_P(IT) ((IT)->what == IT_CHARACTER && ((IT)->c == '\n' || ((IT)->c == '\r' && (IT)->selective)))
  macro_rules! ITERATOR_AT_END_OF_LINE_P {
    ($it:expr) => {
      ((*$it).what == IT_CHARACTER
       && ((*$it).c == '\n' as c_int
           || ((*$it).c == '\r' as c_int && (*$it).selective)))
    };
  }
  /* C 2960-2976: the glyph producer.  C's `do { … } while (false)`; the port writes the
     statements it encloses (the `SET_CHAR_GLYPH`/`CHECK_CODING_SYSTEM` class).  The `NULL`
     comparisons and the `R2L` test are C's own text.  */
  /// C: #define PRODUCE_GLYPHS(IT) do { if ((IT)->glyph_row != NULL && (IT)->bidi_p) (IT)->glyph_row->reversed_p = (IT)->bidi_it.paragraph_dir == R2L; if (FRAME_RIF ((IT)->f) != NULL) FRAME_RIF ((IT)->f)->produce_glyphs (IT); else produce_glyphs (IT); if ((IT)->glyph_row != NULL) inhibit_free_realized_faces =true; reset_box_start_end_flags (IT); } while (false)
  macro_rules! PRODUCE_GLYPHS {
    ($it:expr) => { {
      if !(*$it).glyph_row.is_null () && (*$it).bidi_p
        { (*(*$it).glyph_row).reversed_p =
            ((*$it).bidi_it.paragraph_dir == R2L) as bool; }
      if FRAME_RIF! ((*$it).f) != std::ptr::null_mut ()
        { (*(FRAME_RIF! ((*$it).f))).produce_glyphs ($it); }
      else
        { produce_glyphs ($it); }
      if !(*$it).glyph_row.is_null ()
        { inhibit_free_realized_faces = true; }
      reset_box_start_end_flags ($it);
    } };
  }

  /* C 2977-2993: `enum move_operation_enum`, the bit-flags of `move_it_to`.  */
  /// C: enum move_operation_enum { MOVE_TO_X = 0x01,
  pub const MOVE_TO_X: c_int = 0x01;
  /// C:   MOVE_TO_Y = 0x02,
  pub const MOVE_TO_Y: c_int = 0x02;
  /// C:   MOVE_TO_VPOS = 0x04,
  pub const MOVE_TO_VPOS: c_int = 0x04;
  /// C:   MOVE_TO_POS = 0x08 };
  pub const MOVE_TO_POS: c_int = 0x08;


  /* ---- C 3067-3200: the window-based redisplay interface ------------------------------------
     Same arm and same measurement as the two blocks above.  C's `frame_parm_handler *` is
     `frame.h`'s type and stays `*mut c_void` here (P-72 ③); the twelve members the interface calls
     keep C's names and the port's `Option<unsafe extern "C" fn>` shape, and every one of them is
     an 8-byte pointer, so the offsets are the interface's own.  C's `#ifdef HAVE_WINDOW_SYSTEM`
     arm (C 3131-3195) is dead text on this target and is carried under that feature - the
     fifteen hooks below (status 3.283); the frozen arm's thirteen members and their offsets are
     unchanged by it.  */
#[cfg (feature = "dispextern_face_records")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct redisplay_interface
{
  pub frame_parm_handlers: *mut c_void,                  // offset 0
  pub produce_glyphs: Option<unsafe extern "C" fn (*mut it)>,                  // offset 8
  pub write_glyphs: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, string: *mut glyph, area: c_int, len: c_int)>,                  // offset 16
  pub insert_glyphs: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, start: *mut glyph, area: c_int, len: c_int)>,                  // offset 24
  pub clear_end_of_line: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, area: c_int, x: c_int)>,                  // offset 32
  pub scroll_run_hook: Option<unsafe extern "C" fn (w: *mut c_void, run: *mut run)>,                  // offset 40
  pub after_update_window_line_hook: Option<unsafe extern "C" fn (w: *mut c_void, desired_row: *mut glyph_row)>,                  // offset 48
  pub update_window_begin_hook: Option<unsafe extern "C" fn (w: *mut c_void)>,                  // offset 56
  pub update_window_end_hook: Option<unsafe extern "C" fn (w: *mut c_void, cursor_on_p: bool, mouse_face_overwritten_p: bool)>,                  // offset 64
  pub flush_display: Option<unsafe extern "C" fn (w: *mut c_void)>,                  // offset 72
  pub clear_window_mouse_face: Option<unsafe extern "C" fn (w: *mut c_void)>,                  // offset 80
  pub get_glyph_overhangs: Option<unsafe extern "C" fn (glyph: *mut glyph, f: *mut c_void, left: *mut c_int, right: *mut c_int)>,                  // offset 88
  pub fix_overlapping_area: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, area: c_int, overlaps: c_int)>,                  // offset 96

  /* C 3131-3195: the `#ifdef HAVE_WINDOW_SYSTEM` arm of the interface - the fifteen display
     hooks a window system installs.  Dead text on the frozen target (`HAVE_WINDOW_SYSTEM` is
     undef there), carried under that feature so that a window-system build gets C's full
     record; each is one pointer wide and none of them exists in the frozen arm, so the
     assertions above are unchanged.  They are what `spikes/mirror-struct-coverage.py --arm
     ns_cocoa` names when it reports this struct's window-system gap (status 3.283).  */
  #[cfg (feature = "have_window_system")]
  pub draw_fringe_bitmap: Option<unsafe extern "C" fn (*mut c_void, *mut glyph_row, *mut draw_fringe_bitmap_params)>,   // C 3134
  #[cfg (feature = "have_window_system")]
  pub define_fringe_bitmap: Option<unsafe extern "C" fn (which: c_int, bits: *mut c_ushort, h: c_int, wd: c_int)>,      // C 3138
  #[cfg (feature = "have_window_system")]
  pub destroy_fringe_bitmap: Option<unsafe extern "C" fn (which: c_int)>,                                               // C 3140
  #[cfg (feature = "have_window_system")]
  pub compute_glyph_string_overhangs: Option<unsafe extern "C" fn (s: *mut glyph_string)>,                              // C 3144
  #[cfg (feature = "have_window_system")]
  pub draw_glyph_string: Option<unsafe extern "C" fn (s: *mut glyph_string)>,                                           // C 3147
  #[cfg (feature = "have_window_system")]
  pub define_frame_cursor: Option<unsafe extern "C" fn (f: *mut c_void, cursor: Emacs_Cursor)>,                        // C 3150
  #[cfg (feature = "have_window_system")]
  pub clear_frame_area: Option<unsafe extern "C" fn (f: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int)>, // C 3153
  #[cfg (feature = "have_window_system")]
  pub clear_under_internal_border: Option<unsafe extern "C" fn (f: *mut c_void)>,                                       // C 3159
  #[cfg (feature = "have_window_system")]
  pub draw_window_cursor: Option<unsafe extern "C" fn (w: *mut c_void, glyph_row: *mut glyph_row, x: c_int, y: c_int,
                                                       cursor_type: text_cursor_kinds, cursor_width: c_int,
                                                       on_p: bool, active_p: bool)>,                                  // C 3165
  #[cfg (feature = "have_window_system")]
  pub draw_vertical_window_border: Option<unsafe extern "C" fn (w: *mut c_void, x: c_int, y_0: c_int, y_1: c_int)>,   // C 3172
  #[cfg (feature = "have_window_system")]
  pub draw_window_divider: Option<unsafe extern "C" fn (w: *mut c_void, x_0: c_int, x_1: c_int, y_0: c_int, y_1: c_int)>, // C 3176
  #[cfg (feature = "have_window_system")]
  pub shift_glyphs_for_insert: Option<unsafe extern "C" fn (f: *mut c_void, x: c_int, y: c_int, width: c_int,
                                                            height: c_int, shift_by: c_int)>,                         // C 3182
  #[cfg (feature = "have_window_system")]
  pub show_hourglass: Option<unsafe extern "C" fn (f: *mut c_void)>,                                                    // C 3187
  #[cfg (feature = "have_window_system")]
  pub hide_hourglass: Option<unsafe extern "C" fn (f: *mut c_void)>,                                                    // C 3190
  #[cfg (feature = "have_window_system")]
  pub default_font_parameter: Option<unsafe extern "C" fn (f: *mut c_void, parms: Lisp_Object)>,                        // C 3194
}
/* C 3131-3195's `HAVE_WINDOW_SYSTEM` members - the fifteen display hooks this record already
   carries behind that feature - are appended after `fix_overlapping_area`, so they add 120 bytes
   and shift nothing above them (`docs/baseline/redisplay_interface-layout.tsv`: `frozen` 104/8
   with the thirteen base members, `x11`/`ns_cocoa`/`x11_ext` 224/8; the fifteen measure 104 …
   216).  The same device status 3.505/3.506 used for `struct face`, `struct font` and
   `struct font_driver`.  */
#[cfg (feature = "dispextern_face_records")]
#[cfg (not (feature = "have_window_system"))]
const REDISPLAY_IFACE_WS_SHIFT: usize = 0;
#[cfg (feature = "dispextern_face_records")]
#[cfg (feature = "have_window_system")]
const REDISPLAY_IFACE_WS_SHIFT: usize = 120;
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::size_of::<redisplay_interface> () == 104 + REDISPLAY_IFACE_WS_SHIFT);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, draw_fringe_bitmap) == 104);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, define_fringe_bitmap) == 112);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, destroy_fringe_bitmap) == 120);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, compute_glyph_string_overhangs) == 128);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, draw_glyph_string) == 136);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, define_frame_cursor) == 144);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, clear_frame_area) == 152);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, clear_under_internal_border) == 160);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, draw_window_cursor) == 168);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, draw_vertical_window_border) == 176);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, draw_window_divider) == 184);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, shift_glyphs_for_insert) == 192);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, show_hourglass) == 200);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, hide_hourglass) == 208);
#[cfg (all (feature = "dispextern_face_records", feature = "have_window_system"))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, default_font_parameter) == 216);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::align_of::<redisplay_interface> () == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, frame_parm_handlers) == 0);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, produce_glyphs) == 8);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, write_glyphs) == 16);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, insert_glyphs) == 24);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, clear_end_of_line) == 32);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, scroll_run_hook) == 40);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, after_update_window_line_hook) == 48);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, update_window_begin_hook) == 56);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, update_window_end_hook) == 64);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, flush_display) == 72);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, clear_window_mouse_face) == 80);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, get_glyph_overhangs) == 88);
#[cfg (feature = "dispextern_face_records")]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, fix_overlapping_area) == 96);

  /* C 3407-3430: `enum tab_bar_item_idx` - where a tab-bar item's slots are in the frame's
     tab-bar vector.  */
  /// C: enum tab_bar_item_idx { TAB_BAR_ITEM_KEY,
  pub const TAB_BAR_ITEM_KEY: c_int = 0;
  /// C:   TAB_BAR_ITEM_ENABLED_P,
  pub const TAB_BAR_ITEM_ENABLED_P: c_int = 1;
  /// C:   TAB_BAR_ITEM_SELECTED_P,
  pub const TAB_BAR_ITEM_SELECTED_P: c_int = 2;
  /// C:   TAB_BAR_ITEM_CAPTION,
  pub const TAB_BAR_ITEM_CAPTION: c_int = 3;
  /// C:   TAB_BAR_ITEM_BINDING,
  pub const TAB_BAR_ITEM_BINDING: c_int = 4;
  /// C:   TAB_BAR_ITEM_HELP,
  pub const TAB_BAR_ITEM_HELP: c_int = 5;
  /// C:   TAB_BAR_ITEM_NSLOTS };
  pub const TAB_BAR_ITEM_NSLOTS: c_int = 6;

  /* C 3440-3495: `enum tool_bar_item_idx`.  */
  /// C: enum tool_bar_item_idx { TOOL_BAR_ITEM_KEY,
  pub const TOOL_BAR_ITEM_KEY: c_int = 0;
  /// C:   TOOL_BAR_ITEM_ENABLED_P,
  pub const TOOL_BAR_ITEM_ENABLED_P: c_int = 1;
  /// C:   TOOL_BAR_ITEM_SELECTED_P,
  pub const TOOL_BAR_ITEM_SELECTED_P: c_int = 2;
  /// C:   TOOL_BAR_ITEM_CAPTION,
  pub const TOOL_BAR_ITEM_CAPTION: c_int = 3;
  /// C:   TOOL_BAR_ITEM_IMAGES,
  pub const TOOL_BAR_ITEM_IMAGES: c_int = 4;
  /// C:   TOOL_BAR_ITEM_BINDING,
  pub const TOOL_BAR_ITEM_BINDING: c_int = 5;
  /// C:   TOOL_BAR_ITEM_TYPE,
  pub const TOOL_BAR_ITEM_TYPE: c_int = 6;
  /// C:   TOOL_BAR_ITEM_HELP,
  pub const TOOL_BAR_ITEM_HELP: c_int = 7;
  /// C:   TOOL_BAR_ITEM_RTL_IMAGE,
  pub const TOOL_BAR_ITEM_RTL_IMAGE: c_int = 8;
  /// C:   TOOL_BAR_ITEM_LABEL,
  pub const TOOL_BAR_ITEM_LABEL: c_int = 9;
  /// C:   TOOL_BAR_ITEM_VERT_ONLY,
  pub const TOOL_BAR_ITEM_VERT_ONLY: c_int = 10;
  /// C:   TOOL_BAR_ITEM_WRAP,
  pub const TOOL_BAR_ITEM_WRAP: c_int = 11;
  /// C:   TOOL_BAR_ITEM_NSLOTS };
  pub const TOOL_BAR_ITEM_NSLOTS: c_int = 12;

  /* C 3533-3540: the anonymous `enum` of terminal capabilities.  */
  /// C: TTY_CAP_INVERSE = 1 << 1,
  pub const TTY_CAP_INVERSE: c_uint = 1 << 1;
  /// C:   TTY_CAP_UNDERLINE = 1 << 2,
  pub const TTY_CAP_UNDERLINE: c_uint = 1 << 2;
  /// C:   TTY_CAP_BOLD = 1 << 3,
  pub const TTY_CAP_BOLD: c_uint = 1 << 3;
  /// C:   TTY_CAP_DIM = 1 << 4,
  pub const TTY_CAP_DIM: c_uint = 1 << 4;
  /// C:   TTY_CAP_ITALIC = 1 << 5,
  pub const TTY_CAP_ITALIC: c_uint = 1 << 5;
  /// C:   TTY_CAP_STRIKE_THROUGH = 1 << 6,
  pub const TTY_CAP_STRIKE_THROUGH: c_uint = 1 << 6;
  /// C:   TTY_CAP_UNDERLINE_STYLED = 1 << 7,
  pub const TTY_CAP_UNDERLINE_STYLED: c_uint = 1 << 7;
  /// C:   TTY_CAP_OVERLINE = 1 << 8
  pub const TTY_CAP_OVERLINE: c_uint = 1 << 8;

  /* C 3608-3616: the redisplay state the header exports.  */
  unsafe extern "C" {
    /// C: extern bool redisplaying_p;
    pub static mut redisplaying_p: bool;
    /// C: extern unsigned int redisplay_counter;
    pub static mut redisplay_counter: c_uint;
    /// C: extern bool display_working_on_window_p;
    pub static mut display_working_on_window_p: bool;
    /// C: extern int dont_resize_frames;
    pub static mut dont_resize_frames: c_int;
    /// C: extern bool help_echo_showing_p;
    pub static mut help_echo_showing_p: bool;
    /// C: extern Lisp_Object help_echo_string, help_echo_window;
    pub static mut help_echo_string: Lisp_Object;
    pub static mut help_echo_window: Lisp_Object;
    /// C: extern Lisp_Object help_echo_object, previous_help_echo_string;
    pub static mut help_echo_object: Lisp_Object;
    pub static mut previous_help_echo_string: Lisp_Object;
    /// C: extern ptrdiff_t help_echo_pos;
    pub static mut help_echo_pos: ptrdiff_t;
    /// C 3834: `extern char unspecified_fg[], unspecified_bg[];` - the two arrays are
     // "possibly empty" (xfaces.c appends to them), so the port keeps C's array type.
    pub static mut unspecified_fg: [c_char; 0];
    pub static mut unspecified_bg: [c_char; 0];
  }
}

/* dispextern.h 3563-3565's `row_containing_pos` and 3914's `blank_row`: `xdisp.c`'s
   `redisplay_window` calls the former before its own definition lands later in that unit, and
   dispnew.c *defines* the latter.  Declared in a module + glob import because a unit that
   defines one would otherwise collide with the crate-root declaration (PORTING.md 0.1-10, the
   `window_redisplay_decls` shape in include/window.rs).  The region around C 3563/C 3914 is not
   mirrored yet (the WIP note above); these two are here because a ported unit needs their
   declarations, which is how this mirror grows (status 3.357).  */
pub mod dispextern_late_decls {
  use super::*;

  /* C 3655-3706 and 3853-3854: the display entry points the two units that call them do not
     include a header for (they are definitions in `src/xdisp.c`/`src/fringe.rs`, declared here in
     C).  A declaration at this module's scope cannot collide with the defining unit's own item
     (P-39); the ones C gates with `HAVE_WINDOW_SYSTEM` carry that gate.  */
  /* C 2068/3923: a variable and a function this header declares **ungated** (the `#endif`s
     around them belong to their neighbours), so the mirror declares them ungated too and the
     units carry no copy - the textual declaration gate reads both arms.  */
  unsafe extern "C" {
    pub static mut face_change: bool;
    pub fn do_pending_window_change (inhibit: bool);
  }
  #[cfg (feature = "have_window_system")]
  unsafe extern "C" {
    /* C 3853-3854.  */
    pub fn start_hourglass ();
    pub fn cancel_hourglass ();
    /* C 3705-3706.  */
    pub fn draw_window_fringes (w: *mut window, redraw: bool) -> bool;
    pub fn update_window_fringes (w: *mut window, redraw: bool) -> bool;
    /* C 3655, 3658-3659, 3665, 3667.  */
    pub fn display_and_set_cursor (w: *mut window, on: bool, hpos: c_int, vpos: c_int,
                                   x: c_int, y: c_int);
    pub fn gui_draw_vertical_border (w: *mut window);
    pub fn gui_draw_right_divider (w: *mut window);
    pub fn find_hot_spot (map: Lisp_Object, x: c_int, y: c_int) -> Lisp_Object;
    pub fn get_tab_bar_item_kbd (f: *mut frame, x: c_int, y: c_int,
                                 key: *mut c_int, budget_p: *mut bool) -> c_int;
  }

  /* C 3753: the two image-family entry points, defined in `src/image.c`.  */
  #[cfg (feature = "have_android")]
  unsafe extern "C" { pub fn mark_image_cache (cache: *mut c_void); }
  #[cfg (feature = "have_android")]
  unsafe extern "C" { pub fn image_prune_animation_caches (inhibit: bool); }

  unsafe extern "C" {
    /// C: dispextern.h 3563-3565: `struct glyph_row *row_containing_pos (struct window *,
    /// ptrdiff_t, struct glyph_row *, struct glyph_row *, int);` (xdisp.c defines it).
    /// The window pointer is spelled `*mut c_void` (P-72 rule 3): six units include this
    /// mirror without including window.rs (`bidi`, `coding`, `lread`, `regex-emacs`, `syntax`,
    /// `xml`), and a mirror may not name a type its includers may lack - 3.357's first version
    /// spelled `*mut window` here and `build-tu src/lread.c` failed E0425 on it (status 3.363).
    pub fn row_containing_pos (w: *mut c_void, charpos: ptrdiff_t, start: *mut glyph_row,
                               end: *mut glyph_row, dy: c_int) -> *mut glyph_row;
    /// C: dispextern.h 3914: `void blank_row (struct window *, struct glyph_row *, int);`
    /// (dispnew.c defines it; the same `*mut c_void` spelling and reason as above).
    pub fn blank_row (w: *mut c_void, row: *mut glyph_row, vpos: c_int);
    /// C: dispextern.h 3663: `extern void get_glyph_string_clip_rect (struct glyph_string *,
    /// NativeRectangle *nr);` - one of the five names `docs/ports/w32term.md` owed 10 lists as
    /// declared nowhere.  `NativeRectangle` is a *type* macro this mirror deliberately does not
    /// spell (C 176-181, the note above `dispextern_late_decls`'s neighbour), so the out
    /// parameter is the untyped pointer its call sites pass (P-72 rule 3), exactly as
    /// `row_containing_pos` spells `struct window *`.
    /// C's own guard is `#ifdef HAVE_WINDOW_SYSTEM` (dispextern.h 3660-3674), which is also the
    /// feature `struct glyph_string` itself carries below.
    #[cfg (feature = "have_window_system")]
    pub fn get_glyph_string_clip_rect (s: *mut glyph_string, nr: *mut c_void);
    /// C: dispextern.h 3676: `extern bool gui_intersect_rectangles (const Emacs_Rectangle *,
    /// const Emacs_Rectangle *, Emacs_Rectangle *);` - the same `HAVE_WINDOW_SYSTEM` guard.
    #[cfg (feature = "have_window_system")]
    pub fn gui_intersect_rectangles (r1: *const Emacs_Rectangle, r2: *const Emacs_Rectangle,
                                     out: *mut Emacs_Rectangle) -> bool;
    /* C 3688's `note_mouse_highlight`, C 3711's `gui_define_fringe_bitmap`, C 3719's
       `buffer_flipping_blocked_p` and C 3813's `lookup_basic_face` are **owed, not declared**,
       and the measurement is why: each of the four is declared at the crate root of at least one
       ported unit, and `tools/check-mirror-dupes.py` counts a declaration inside this
       glob-imported module as the same construct - adding them produced eleven new `FAIL` lines
       naming `androidterm`/`dispnew`/`font`/`frame`/`haikuterm`/`w32term`/`xdisp` (measured on
       this revision; reverted).  They land in the commit that deletes those copies, with the
       ratchet rows `docs/baseline/mirror-dupes.tsv` needs - the `docs/ports/w32term.md` owed-10
       handoff, and the same shape as C 3993's `check_x_display_info` below.  The two this module
       *does* carry (`get_glyph_string_clip_rect`, `gui_intersect_rectangles`) had no such copy.  */
    // C: dispextern.h 3993: `extern Display_Info *check_x_display_info (Lisp_Object);` is **not**
    // declared here: `femacs/src/frame.rs` 6155-6156 already declares it at its crate root (the
    // unit's own use-site copy, reached through the `HAVE_WINDOW_SYSTEM` arm of
    // `gui_display_get_resource`), and placing it in this mirror's glob-imported
    // `dispextern_late_decls` makes `tools/check-mirror-dupes.py` report the pair - measured:
    // `FAIL femacs/src/frame.rs: check_x_display_info is already declared by
    // femacs/src/include/dispextern.rs (in the include graph)`.  It lands in the commit that
    // deletes that copy (with its ratchet row); `docs/ports/w32fns.md` owed item 9 stays owed.
  }
}
use dispextern_late_decls::*;

/* -------------------------------------------------------------------------------------------------
   The records above name four tags as *pointers*, and two records through aliases, that C needs no
   declaration for but Rust does:

     * `glyph_string`'s `f` / `w` / `font` (C 1392/1395/1417): C allows a pointer to an incomplete
       tag, so `dispextern.h` itself carries no edge for the three;
     * `Display_Info`'s Android alias (C 165) and `Emacs_Pix_Container` (C 166), whose records are
       `termhooks.rs`'s and `androidgui.rs`'s.

   C's only edge of this set is C 164's `#include "androidgui.h"`.  Measured on
   `aarch64-linux-android`, the arm that compiles this text for the first time (the darwin
   configuration sets no window-system feature, so every one of these references was dead before):
   11 x `cannot find type frame`, 7 x `android_display_info`, 7 x `android_image`, 7 x `window`,
   5 x `font` across this mirror's includers.  The edges sit at the *end* of the file because Rust
   items are order-independent - a `include!` near the top would instead take the dedupe slot of
   every file the units reach after this one (`windows.rs` 48's `buffer.rs` note records the same
   hazard for the mirror-level shape).  */
#[cfg (feature = "have_window_system")]
include!("frame.rs");
#[cfg (feature = "have_window_system")]
include!("window.rs");
#[cfg (feature = "have_window_system")]
include!("font.rs");
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
include!("androidgui.rs");
