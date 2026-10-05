// femacs: port of src/fringe.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The C non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` (none of this slice has its address taken).
//
// **This file is P-86's unit `src/fringe.c` (1880 lines) and is ported in slices; this is slice 1
// = C 1-1364.**  The slice ends on the `}` of `update_window_fringes` (C 1364); **the next
// definition is `destroy_fringe_bitmap` (C 1370-1371)**.  The ledger `docs/ports/fringe.md`
// carries P-86 ②'s C line-range table, the `## Evidence` readings and the `## Deviations`.
//
// The *build-level* file gate is `feature = "have_window_system"` (P-86 ⑧), on every item *and on
// C 20-31's include graph*.  It is measured, not inferred: `configure.ac` 7513-7521 defines
// `HAVE_WINDOW_SYSTEM` and puts `fringe.o` in `WINDOW_SYSTEM_OBJ` exactly when
// `$window_system != none` (`WINDOW_SYSTEM_OBJ="fontset.o fringe.o image.o"` is that `if`'s whole
// body), so the frozen configuration (darwin-arm64, no window system) builds no `fringe.o`:
// `WINDOW_SYSTEM_OBJ=` in `build/emacs-ref/src/Makefile` 304, `HAVE_WINDOW_SYSTEM` `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv` 1678, and `docs/baseline/frozen-build-sources.txt`
// lists no `src/fringe.c`.  The old revision's conclusion ("not compiled by the frozen
// configuration, so the port is the empty unit plus a note", status 3.31) is withdrawn as a
// *conclusion* - its measurements hold and are where the file gate comes from; the text is carried
// **in full** (P-86 ①), because every window-system configuration compiles it (the `kqueue.rs` /
// `gfilenotify.rs` shape).
//
// The file's own `#if`s in this slice are one: C 33-35's `#ifdef HAVE_PGTK` + `pgtkterm.h`, whose
// gate is `feature = "have_pgtk"` (`docs/baseline/feature-coverage.tsv`; `femacs/xtask/Cargo.toml`).
// The four remaining arms - C 1433's `HAVE_X_WINDOWS || HAVE_PGTK || HAVE_ANDROID` (`have_x_windows`
// / `have_pgtk` / `have_android`), C 1448's `HAVE_X_WINDOWS || HAVE_ANDROID`, C 1498's
// `!HAVE_X_WINDOWS && HAVE_PGTK` and C 1517/1865's `HAVE_NTGUI` (`have_ntgui`) - are C 1433's and
// later, i.e. slice 2.
//
// The records this slice needs (`draw_fringe_bitmap_params`, `redisplay_interface`, `face`,
// `glyph_row`, `glyph_matrix`, `window`) are the mirror's, and the first three sit under
// `feature = "dispextern_face_records"` where C's dispextern.h is unconditional (status 3.220), so
// the arm flip passes that feature as well (the `w32term.rs` / `haikuterm.rs` shape; `## Evidence`
// in the ledger).  `struct fringe_bitmap`, its five bitfields, the bitmaps and the standard-bitmap
// table are this file's own (nothing else in the tree declares them).
//
// Names the mirrors this unit includes do not carry are declared at the use site with the
// reference header's line (PORTING.md 0.1-2 / P-72 rule 4) - `window_text_bottom_y`
// (dispextern.h 3577), `window_box_left` (3579), `window_box_right` (3581), `lookup_named_face`
// (3812), `lookup_derived_face` (3816) and `prepare_face_for_display` (3810).  Each is named in the
// ledger's Owed 3, with the mirror it belongs in.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn,
         unused_assignments, unused_parens, unused_mut, unreachable_code, unused_labels, while_true,
         unused_comparisons, overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 20: `#include <config.h>` (every unit includes it)

/* C 22-31 and C 33-35: the file's own include list, in C's order.  The whole graph sits on the
   file gate (P-86 ⑧: an include outside the gate compiles text the frozen configuration never
   sees), and C 33-35's `#ifdef HAVE_PGTK` arm carries `have_pgtk` on top of it.  `pgtkterm.h`'s
   mirror is `include/pgtkterm.rs` (P-90 ②: one surface, one mirror); this slice's body uses no
   name from it - the include is C 33-35's, and C 1443's X/PGTK arm is what needs it.  */
#[cfg (feature = "have_window_system")]
include!("../lib/include/byteswap.in.rs");     // C 22: `#include <byteswap.h>` (gnulib)
#[cfg (feature = "have_window_system")]
include!("include/lisp.rs");                   // C 24: `#include "lisp.h"`
#[cfg (feature = "have_window_system")]
include!("include/frame.rs");                  // C 25: `#include "frame.h"`
#[cfg (feature = "have_window_system")]
include!("include/window.rs");                 // C 26: `#include "window.h"`
#[cfg (feature = "have_window_system")]
include!("include/dispextern.rs");             // C 27: `#include "dispextern.h"`
#[cfg (feature = "have_window_system")]
include!("include/buffer.rs");                 // C 28: `#include "buffer.h"`
#[cfg (feature = "have_window_system")]
include!("include/blockinput.rs");             // C 29: `#include "blockinput.h"`
#[cfg (feature = "have_window_system")]
include!("include/termhooks.rs");              // C 30: `#include "termhooks.h"`
#[cfg (feature = "have_window_system")]
include!("include/pdumper.rs");                // C 31: `#include "pdumper.h"`
/* The edges C 24-30's chain reaches through headers these mirrors do not carry themselves
   (PORTING.md P-72 rule 4 - the `gfilenotify.rs` / `androidterm.rs` shape): dispextern.h 25
   includes `character.h` (`MAX_CHAR`), termhooks.h 28 includes `systime.h` (`Time`), and
   dispextern.h's `struct glyph_string` names `font.h`'s `struct font` and `composite.h`'s
   `struct composition`.  Each mirror carries the name the C chain supplies - `include/character.rs`
   (29), `include/systime.rs`, `include/font.rs` and `include/composite.rs` - so the unit spells the
   four edges here rather than editing the mirrors (P-88 ① / P-72 rule 4).

   **These four edges carry no gate (P-89's rule, measured).**  `pgtkterm.rs` (C 33-35, gated
   `have_pgtk` here) reaches all four of them first, and `xtask`'s materializer expands a file on its
   *first* edge; a gated edge of this unit's own then becomes `[xtask dedupe] … already included` and
   the file never expands in this arm - measured: with the four edges gated, the `have_window_system`
   + `have_x_windows` flip reports `MAX_CHAR` (dispextern.rs 264), `Time` (termhooks.rs 110) and
   `font` (dispextern.rs 1616) as missing, because the four copies are expanded inside `pgtkterm.rs`
   (C 33-35) although that file is itself gated off.  **The fix that works is position**: these four
   edges stand *before* C 33-35's, so the materializer expands each of the four on this unit's own
   edge (gated with the file gate like every other edge here) and `pgtkterm.rs`'s nested ones become
   the no-ops - measured: the same flip then reports none of the three names.  An *ungated* edge was
   tried first and is wrong for the frozen configuration (measured: `character.rs`'s own items need
   `lisp.rs`'s `eassert!`/`eassume!`, so the frozen object stops being the empty one).  */
#[cfg (feature = "have_window_system")]
include!("include/character.rs");              // C 27 -> dispextern.h 25: `character.h` (MAX_CHAR)
#[cfg (feature = "have_window_system")]
include!("include/systime.rs");                // C 30 -> termhooks.h 28: `systime.h` (Time)
#[cfg (feature = "have_window_system")]
include!("include/font.rs");                   // C 27 -> dispextern.h's `struct font`
#[cfg (feature = "have_window_system")]
include!("include/composite.rs");              // C 27 -> dispextern.h's `struct composition`

#[cfg (all (feature = "have_window_system", feature = "have_pgtk"))]
include!("include/pgtkterm.rs");               // C 33-35: `#ifdef HAVE_PGTK` + `pgtkterm.h`

/* C 37-69: the file's own commentary on the three ways a fringe bitmap is represented.  Carried
   verbatim (it is the file's only statement of the mapping the code below depends on).  */

/* Fringe bitmaps are represented in three different ways:

   Logical bitmaps are used internally to denote things like
   'end-of-buffer', 'left-truncation', 'overlay-arrow', etc.

   Physical bitmaps specify the visual appearance of the bitmap,
   e.g. 'bottom-left-angle', 'left-arrow', 'left-triangle', etc.
   User defined bitmaps are physical bitmaps.

   Internally, fringe bitmaps for a specific display row are
   represented as a simple integer that is used as an index
   into the table of all defined bitmaps.  This index is stored
   in the `fringe' property of the physical bitmap symbol.

   Logical bitmaps are mapped to physical bitmaps through the
   buffer-local `fringe-indicator-alist' variable.

   Each element of this alist is a cons (LOGICAL . PHYSICAL)
   mapping a logical bitmap to a physical bitmap.
   PHYSICAL is either a symbol to use in both left and right fringe,
   or a cons of two symbols (LEFT . RIGHT) denoting different
   bitmaps to use in left and right fringe.

   LOGICAL is first looked up in the window's buffer's buffer-local
   value of the fringe-indicator-alist variable, and if not present,
   in the global value of fringe-indicator-alist.

   If LOGICAL is not present in either alist, or the PHYSICAL value
   found is nil, no bitmap is shown for the logical bitmap.

   The `left-fringe' and `right-fringe' display properties
   must specify physical bitmap symbols.
*/

/* C 71-76: `enum fringe_bitmap_align`.  An enum is `c_int` with its members as constants
   (PORTING.md 1.3; the `include/dispextern.rs` `text_cursor_kinds` shape), and C 630 switches on
   the three values, so the `match` arms below are these constants.  */
/// C: 71-76: `enum fringe_bitmap_align { ALIGN_BITMAP_CENTER = 0, ALIGN_BITMAP_TOP,
/// ALIGN_BITMAP_BOTTOM };`
#[cfg (feature = "have_window_system")]
pub type fringe_bitmap_align = c_int;
#[cfg (feature = "have_window_system")]
pub const ALIGN_BITMAP_CENTER: fringe_bitmap_align = 0;
#[cfg (feature = "have_window_system")]
pub const ALIGN_BITMAP_TOP: fringe_bitmap_align = 1;
#[cfg (feature = "have_window_system")]
pub const ALIGN_BITMAP_BOTTOM: fringe_bitmap_align = 2;

/* C 78-85: `struct fringe_bitmap`.  C's five bitfields (`height : 8`, `width : 8`, `period : 8`,
   `align : 2`, `bool_bf dynamic : 1`) share the four-byte allocation unit at offset 8 (Rust has no
   bitfields, PORTING.md 1.3), so the port keeps C's `bits` pointer and folds the run into the
   `flags` word the way `include/dispextern.rs`'s `draw_fringe_bitmap_params` folds its own two -
   C's member name `bits` is the pointer's and cannot be reused for the run (Deviation 2).  The
   layout is a clang measurement on this machine (`## Evidence`): `sizeof` 16, `align` 8, `bits` at
   offset 0, the run at 8 with `height` in bits 0-7, `width` 8-15, `period` 16-23, `align` 24-25 and
   `dynamic` bit 26, and four bytes of trailing padding.  */
/// C: 78-80: `struct fringe_bitmap { unsigned short *bits;`
#[cfg (feature = "have_window_system")]
#[repr(C)]
pub struct fringe_bitmap
{
  pub bits: *mut c_ushort,
  /// C 81-85's bitfield run (offset 8): `height : 8`, `width : 8`, `period : 8`, `align : 2`,
  /// `bool_bf dynamic : 1`.
  pub flags: u32,
}
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (core::mem::size_of::<fringe_bitmap> () == 16);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (core::mem::offset_of! (fringe_bitmap, bits) == 0);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (core::mem::offset_of! (fringe_bitmap, flags) == 8);

/// C: 81: `unsigned height : 8;` - bits 0-7 of `fringe_bitmap::flags` (measured).
#[cfg (feature = "have_window_system")]
pub const FRINGE_BITMAP_HEIGHT_SHIFT: u32 = 0;
#[cfg (feature = "have_window_system")]
pub const FRINGE_BITMAP_HEIGHT_MASK: u32 = 0xff;
/// C: 82: `unsigned width : 8;` - bits 8-15.
#[cfg (feature = "have_window_system")]
pub const FRINGE_BITMAP_WIDTH_SHIFT: u32 = 8;
#[cfg (feature = "have_window_system")]
pub const FRINGE_BITMAP_WIDTH_MASK: u32 = 0xff;
/// C: 83: `unsigned period : 8;` - bits 16-23.
#[cfg (feature = "have_window_system")]
pub const FRINGE_BITMAP_PERIOD_SHIFT: u32 = 16;
#[cfg (feature = "have_window_system")]
pub const FRINGE_BITMAP_PERIOD_MASK: u32 = 0xff;
/// C: 84: `unsigned align : 2;` - bits 24-25.
#[cfg (feature = "have_window_system")]
pub const FRINGE_BITMAP_ALIGN_SHIFT: u32 = 24;
#[cfg (feature = "have_window_system")]
pub const FRINGE_BITMAP_ALIGN_MASK: u32 = 0x3;
/// C: 85: `bool_bf dynamic : 1;` - bit 26.
#[cfg (feature = "have_window_system")]
pub const FRINGE_BITMAP_DYNAMIC_SHIFT: u32 = 26;
#[cfg (feature = "have_window_system")]
pub const FRINGE_BITMAP_DYNAMIC_MASK: u32 = 0x1;

/// Read one bitfield of `fringe_bitmap::flags` - C's `(FB)->MEMBER`.
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub unsafe fn fringe_bitmap_bits_get (fb: *const fringe_bitmap, shift: u32, mask: u32) -> c_uint
{
  ((*fb).flags >> shift) & mask
}

/// Assign one bitfield of `fringe_bitmap::flags` - C's `(FB)->MEMBER = VALUE`, which evaluates
/// VALUE once.
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub unsafe fn fringe_bitmap_bits_set (fb: *mut fringe_bitmap, shift: u32, mask: u32, value: c_uint)
{
  (*fb).flags = ((*fb).flags & ! (mask << shift)) | ((value & mask) << shift);
}

/// C's five bit members of `fringe_bitmap` as a read/write pair, because Rust has no bitfields
/// (PORTING.md 1.3): `fb->period` is `fringe_bitmap_bits! (period, fb)` and `fb->period = v` is
/// `fringe_bitmap_bits! (period, fb, v)`.  `dynamic` is a `bool_bf` (C's `bool` on this target,
/// conf_post.h 58-62), so its reader answers `bool` and its writer takes one.
#[cfg (feature = "have_window_system")]
macro_rules! fringe_bitmap_bits {
  (height, $fb:expr) =>
    { fringe_bitmap_bits_get ($fb, FRINGE_BITMAP_HEIGHT_SHIFT, FRINGE_BITMAP_HEIGHT_MASK) };
  (height, $fb:expr, $value:expr) =>
    { fringe_bitmap_bits_set ($fb, FRINGE_BITMAP_HEIGHT_SHIFT, FRINGE_BITMAP_HEIGHT_MASK, $value) };
  (width, $fb:expr) =>
    { fringe_bitmap_bits_get ($fb, FRINGE_BITMAP_WIDTH_SHIFT, FRINGE_BITMAP_WIDTH_MASK) };
  (width, $fb:expr, $value:expr) =>
    { fringe_bitmap_bits_set ($fb, FRINGE_BITMAP_WIDTH_SHIFT, FRINGE_BITMAP_WIDTH_MASK, $value) };
  (period, $fb:expr) =>
    { fringe_bitmap_bits_get ($fb, FRINGE_BITMAP_PERIOD_SHIFT, FRINGE_BITMAP_PERIOD_MASK) };
  (period, $fb:expr, $value:expr) =>
    { fringe_bitmap_bits_set ($fb, FRINGE_BITMAP_PERIOD_SHIFT, FRINGE_BITMAP_PERIOD_MASK, $value) };
  (align, $fb:expr) =>
    { fringe_bitmap_bits_get ($fb, FRINGE_BITMAP_ALIGN_SHIFT, FRINGE_BITMAP_ALIGN_MASK) };
  (align, $fb:expr, $value:expr) =>
    { fringe_bitmap_bits_set ($fb, FRINGE_BITMAP_ALIGN_SHIFT, FRINGE_BITMAP_ALIGN_MASK, $value) };
  (dynamic, $fb:expr) =>
    { fringe_bitmap_bits_get ($fb, FRINGE_BITMAP_DYNAMIC_SHIFT, FRINGE_BITMAP_DYNAMIC_MASK) != 0 };
  (dynamic, $fb:expr, $value:expr) =>
    { fringe_bitmap_bits_set ($fb, FRINGE_BITMAP_DYNAMIC_SHIFT, FRINGE_BITMAP_DYNAMIC_MASK,
                              ($value) as c_uint) };
}
/* C 88-451: the thirty-four `static unsigned short` bitmap arrays and their pictures, in C 92 (a
   form feed) through C 451.  C names them `unsigned short` (this target: 2-byte elements) and
   leaves them `static` (internal linkage), so the port keeps both and no symbol is exported
   (PORTING.md 0.2).  The generated `C <n>-<m>` lines that follow the pictures carry each array
   transcribe a C line range of its own.  */

/***********************************************************************
			       Fringe bitmaps
 ***********************************************************************/

/* Undefined bitmap.  A question mark.  */
/*
  ..xxxx..
  .xxxxxx.
  xx....xx
  xx....xx
  ....xx..
  ...xx...
  ...xx...
  ........
  ...xx...
  ...xx...
*/
/* C 106-107: `static unsigned short question_mark_bits[]` (`10` elements).  */
#[cfg (feature = "have_window_system")]
static mut question_mark_bits: [c_ushort; 10] = [
  0x3c, 0x7e, 0xc3, 0xc3, 0x0c, 0x18, 0x18, 0x00, 0x18, 0x18
];

/* An exclamation mark.  */
/*
  ...XX...
  ...XX...
  ...XX...
  ...XX...
  ...XX...
  ...XX...
  ...XX...
  ........
  ...XX...
  ...XX...
*/
/* C 122-123: `static unsigned short exclamation_mark_bits[]` (`10` elements).  */
#[cfg (feature = "have_window_system")]
static mut exclamation_mark_bits: [c_ushort; 10] = [
  0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x18
];

/* An arrow like this: `<-'.  */
/*
  ...xx...
  ..xx....
  .xx.....
  xxxxxx..
  xxxxxx..
  .xx.....
  ..xx....
  ...xx...
*/
/* C 136-137: `static unsigned short left_arrow_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut left_arrow_bits: [c_ushort; 8] = [
  0x18, 0x30, 0x60, 0xfc, 0xfc, 0x60, 0x30, 0x18
];


/* Right truncation arrow bitmap `->'.  */
/*
  ...xx...
  ....xx..
  .....xx.
  ..xxxxxx
  ..xxxxxx
  .....xx.
  ....xx..
  ...xx...
*/
/* C 151-152: `static unsigned short right_arrow_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut right_arrow_bits: [c_ushort; 8] = [
  0x18, 0x0c, 0x06, 0x3f, 0x3f, 0x06, 0x0c, 0x18
];


/* Up arrow bitmap.  */
/*
  ...xx...
  ..xxxx..
  .xxxxxx.
  xxxxxxxx
  ...xx...
  ...xx...
  ...xx...
  ...xx...
*/
/* C 166-167: `static unsigned short up_arrow_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut up_arrow_bits: [c_ushort; 8] = [
  0x18, 0x3c, 0x7e, 0xff, 0x18, 0x18, 0x18, 0x18
];


/* Down arrow bitmap.  */
/*
  ...xx...
  ...xx...
  ...xx...
  ...xx...
  xxxxxxxx
  .xxxxxx.
  ..xxxx..
  ...xx...
*/
/* C 181-182: `static unsigned short down_arrow_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut down_arrow_bits: [c_ushort; 8] = [
  0x18, 0x18, 0x18, 0x18, 0xff, 0x7e, 0x3c, 0x18
];

/* Marker for continuation lines.  */
/*
  ..xxxx..
  .xxxxx..
  xx......
  xxx..x..
  xxxxxx..
  .xxxxx..
  ..xxxx..
  .xxxxx..
*/
/* C 195-196: `static unsigned short left_curly_arrow_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut left_curly_arrow_bits: [c_ushort; 8] = [
  0x3c, 0x7c, 0xc0, 0xe4, 0xfc, 0x7c, 0x3c, 0x7c
];

/* Marker for continued lines.  */
/*
  ..xxxx..
  ..xxxxx.
  ......xx
  ..x..xxx
  ..xxxxxx
  ..xxxxx.
  ..xxxx..
  ..xxxxx.
*/
/* C 209-210: `static unsigned short right_curly_arrow_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut right_curly_arrow_bits: [c_ushort; 8] = [
  0x3c, 0x3e, 0x03, 0x27, 0x3f, 0x3e, 0x3c, 0x3e
];

/* Large circle bitmap.  */
/*
  ..xxxx..
  .xxxxxx.
  xxxxxxxx
  xxxxxxxx
  xxxxxxxx
  xxxxxxxx
  .xxxxxx.
  ..xxxx..
*/
/* C 223-224: `static unsigned short large_circle_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut large_circle_bits: [c_ushort; 8] = [
  0x3c, 0x7e, 0xff, 0xff, 0xff, 0xff, 0x7e, 0x3c
];

/* Reverse Overlay arrow bitmap.  A triangular arrow.  */
/*
  ......xx
  ....xxxx
  ...xxxxx
  ..xxxxxx
  ..xxxxxx
  ...xxxxx
  ....xxxx
  ......xx
*/
/* C 237-238: `static unsigned short left_triangle_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut left_triangle_bits: [c_ushort; 8] = [
  0x03, 0x0f, 0x1f, 0x3f, 0x3f, 0x1f, 0x0f, 0x03
];

/* Overlay arrow bitmap.  A triangular arrow.  */
/*
  xx......
  xxxx....
  xxxxx...
  xxxxxx..
  xxxxxx..
  xxxxx...
  xxxx....
  xx......
*/
/* C 251-252: `static unsigned short right_triangle_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut right_triangle_bits: [c_ushort; 8] = [
  0xc0, 0xf0, 0xf8, 0xfc, 0xfc, 0xf8, 0xf0, 0xc0
];

/* First line bitmap.  An top-left angle.  */
/*
  xxxxxx..
  xxxxxx..
  xx......
  xx......
  xx......
  xx......
  xx......
  ........
*/
/* C 265-266: `static unsigned short top_left_angle_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut top_left_angle_bits: [c_ushort; 8] = [
  0xfc, 0xfc, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0x00
];

/* First line bitmap.  An right-up angle.  */
/*
  ..xxxxxx
  ..xxxxxx
  ......xx
  ......xx
  ......xx
  ......xx
  ......xx
  ........
*/
/* C 279-280: `static unsigned short top_right_angle_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut top_right_angle_bits: [c_ushort; 8] = [
  0x3f, 0x3f, 0x03, 0x03, 0x03, 0x03, 0x03, 0x00
];

/* Last line bitmap.  An left-down angle.  */
/*
  ........
  xx......
  xx......
  xx......
  xx......
  xx......
  xxxxxx..
  xxxxxx..
*/
/* C 293-294: `static unsigned short bottom_left_angle_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut bottom_left_angle_bits: [c_ushort; 8] = [
  0x00, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xfc, 0xfc
];

/* Last line bitmap.  An right-down angle.  */
/*
  ........
  ......xx
  ......xx
  ......xx
  ......xx
  ......xx
  ..xxxxxx
  ..xxxxxx
*/
/* C 307-308: `static unsigned short bottom_right_angle_bits[]` (`8` elements).  */
#[cfg (feature = "have_window_system")]
static mut bottom_right_angle_bits: [c_ushort; 8] = [
  0x00, 0x03, 0x03, 0x03, 0x03, 0x03, 0x3f, 0x3f
];

/* First/last line bitmap.  An left bracket.  */
/*
  xxxxxx..
  xxxxxx..
  xx......
  xx......
  xx......
  xx......
  xx......
  xx......
  xxxxxx..
  xxxxxx..
*/
/* C 323-324: `static unsigned short left_bracket_bits[]` (`10` elements).  */
#[cfg (feature = "have_window_system")]
static mut left_bracket_bits: [c_ushort; 10] = [
  0xfc, 0xfc, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xfc, 0xfc
];

/* First/last line bitmap.  An right bracket.  */
/*
  ..xxxxxx
  ..xxxxxx
  ......xx
  ......xx
  ......xx
  ......xx
  ......xx
  ......xx
  ..xxxxxx
  ..xxxxxx
*/
/* C 339-340: `static unsigned short right_bracket_bits[]` (`10` elements).  */
#[cfg (feature = "have_window_system")]
static mut right_bracket_bits: [c_ushort; 10] = [
  0x3f, 0x3f, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x3f, 0x3f
];

/* Filled box cursor bitmap.  A filled box; max 13 pixels high.  */
/*
  xxxxxxx.
  xxxxxxx.
  xxxxxxx.
  xxxxxxx.
  xxxxxxx.
  xxxxxxx.
  xxxxxxx.
  xxxxxxx.
  xxxxxxx.
  xxxxxxx.
  xxxxxxx.
  xxxxxxx.
  xxxxxxx.
*/
/* C 358-359: `static unsigned short filled_rectangle_bits[]` (`13` elements).  */
#[cfg (feature = "have_window_system")]
static mut filled_rectangle_bits: [c_ushort; 13] = [
  0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
  0xfe, 0xfe, 0xfe
];

/* Hollow box cursor bitmap.  A hollow box; max 13 pixels high.  */
/*
  xxxxxxx.
  x.....x.
  x.....x.
  x.....x.
  x.....x.
  x.....x.
  x.....x.
  x.....x.
  x.....x.
  x.....x.
  x.....x.
  x.....x.
  xxxxxxx.
*/
/* C 377-378: `static unsigned short hollow_rectangle_bits[]` (`13` elements).  */
#[cfg (feature = "have_window_system")]
static mut hollow_rectangle_bits: [c_ushort; 13] = [
  0xfe, 0x82, 0x82, 0x82, 0x82, 0x82, 0x82, 0x82, 0x82, 0x82,
  0x82, 0x82, 0xfe
];

/* Hollow square bitmap.  */
/*
  .xxxxxx.
  .x....x.
  .x....x.
  .x....x.
  .x....x.
  .xxxxxx.
*/
/* C 389-390: `static unsigned short hollow_square_bits[]` (`6` elements).  */
#[cfg (feature = "have_window_system")]
static mut hollow_square_bits: [c_ushort; 6] = [
  0x7e, 0x42, 0x42, 0x42, 0x42, 0x7e
];

/* Filled square bitmap.  */
/*
  .xxxxxx.
  .xxxxxx.
  .xxxxxx.
  .xxxxxx.
  .xxxxxx.
  .xxxxxx.
*/
/* C 401-402: `static unsigned short filled_square_bits[]` (`6` elements).  */
#[cfg (feature = "have_window_system")]
static mut filled_square_bits: [c_ushort; 6] = [
  0x7e, 0x7e, 0x7e, 0x7e, 0x7e, 0x7e
];

/* Bar cursor bitmap.  A vertical bar; max 13 pixels high.  */
/*
  xx......
  xx......
  xx......
  xx......
  xx......
  xx......
  xx......
  xx......
  xx......
  xx......
  xx......
  xx......
  xx......
*/
/* C 420-421: `static unsigned short vertical_bar_bits[]` (`13` elements).  */
#[cfg (feature = "have_window_system")]
static mut vertical_bar_bits: [c_ushort; 13] = [
  0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0,
  0xc0, 0xc0, 0xc0
];

/* HBar cursor bitmap.  A horizontal bar; 2 pixels high.  */
/*
  xxxxxxx.
  xxxxxxx.
*/
/* C 428-429: `static unsigned short horizontal_bar_bits[]` (`2` elements).  */
#[cfg (feature = "have_window_system")]
static mut horizontal_bar_bits: [c_ushort; 2] = [
  0xfe, 0xfe
];


/* Bitmap drawn to indicate lines not displaying text if
   `indicate-empty-lines' is non-nil.  */
/*
  ........
  ..xxxx..
  ........
  ........
  ..xxxx..
  ........
*/
/* C 442-450: `static unsigned short empty_line_bits[]` (`72` elements).  */
#[cfg (feature = "have_window_system")]
static mut empty_line_bits: [c_ushort; 72] = [
  0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00,
  0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x3c,
  0x00, 0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00,
  0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00,
  0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x3c,
  0x00, 0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00,
  0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x3c, 0x00, 0x00,
  0x3c, 0x00
];


/* C 453-455: the three macros the table below is built with.  `BYTES_PER_BITMAP_ROW` is C's
   `sizeof (unsigned short)` (the name says bytes per row, the macro is the element's size) and
   `STANDARD_BITMAP_HEIGHT (bits)` is C's `sizeof (bits) / BYTES_PER_BITMAP_ROW`, i.e. the array's
   element count - the bitmap's height in rows.  The measurement `sizeof` needs inside a *static
   initialiser* is `size_of_val_raw (addr_of! (bits))` (C's `sizeof` never reads the array), which
   is why the macro's body carries the one `unsafe` block a const context allows.  */

/// C: 460-487's initialisers write C's five bitfields in declaration order
/// (`{ FRBITS (question_mark_bits), 8, 0, ALIGN_BITMAP_CENTER, 0 }`), and the port packs them into
/// `fringe_bitmap::flags` with the shifts measured above.  A `const fn` so a static initialiser can
/// call it.
#[cfg (feature = "have_window_system")]
const fn fringe_bitmap_run (height: c_uint, width: c_uint, period: c_uint, align: c_uint,
                            dynamic: c_uint) -> u32
{
  ((height & FRINGE_BITMAP_HEIGHT_MASK) << FRINGE_BITMAP_HEIGHT_SHIFT)
  | ((width & FRINGE_BITMAP_WIDTH_MASK) << FRINGE_BITMAP_WIDTH_SHIFT)
  | ((period & FRINGE_BITMAP_PERIOD_MASK) << FRINGE_BITMAP_PERIOD_SHIFT)
  | ((align & FRINGE_BITMAP_ALIGN_MASK) << FRINGE_BITMAP_ALIGN_SHIFT)
  | ((dynamic & FRINGE_BITMAP_DYNAMIC_MASK) << FRINGE_BITMAP_DYNAMIC_SHIFT)
}

/// C: 453: `#define BYTES_PER_BITMAP_ROW  (sizeof (unsigned short))`
#[cfg (feature = "have_window_system")]
macro_rules! BYTES_PER_BITMAP_ROW { () => { core::mem::size_of::<c_ushort> () as usize }; }

/// C: 454: `#define STANDARD_BITMAP_HEIGHT(bits) (sizeof (bits)/BYTES_PER_BITMAP_ROW)`
#[cfg (feature = "have_window_system")]
macro_rules! STANDARD_BITMAP_HEIGHT {
  ($bits:expr) => {
    unsafe { core::mem::size_of_val_raw (core::ptr::addr_of! ($bits)) } / BYTES_PER_BITMAP_ROW! ()
  };
}

/// C: 455: `#define FRBITS(bits)  bits, STANDARD_BITMAP_HEIGHT (bits)` - a macro that expands to
/// two *members* of a `struct fringe_bitmap` initialiser, and Rust has no macro in that position
/// (measured: `S { mac!() }` is `expected one of `,`, `:`, or `}``), so the port's `FRBITS` takes
/// C's four remaining members and answers the whole record.  `bits` and the height it implies keep
/// C's order and meaning, and C's array-to-pointer decay is `addr_of_mut!` + the element-pointer cast
/// (Deviation 1).
#[cfg (feature = "have_window_system")]
macro_rules! FRBITS {
  ($bits:expr, $width:expr, $period:expr, $align:expr, $dynamic:expr) => {
    fringe_bitmap { bits: core::ptr::addr_of_mut! ($bits) as *mut c_ushort,
                    flags: fringe_bitmap_run (STANDARD_BITMAP_HEIGHT! ($bits) as c_uint,
                                              $width as c_uint, $period as c_uint,
                                              $align as c_uint, $dynamic as c_uint) }
  };
}

/* NOTE:  The order of these bitmaps must match the sequence
   used in fringe.el to define the corresponding symbols.  */

/// C: 460-487: `static struct fringe_bitmap standard_bitmaps[] = { ... };` - the twenty-five
/// entries (`NO_FRINGE_BITMAP`'s all-zero record plus twenty-four bitmaps), which is the count
/// `MAX_STANDARD_FRINGE_BITMAPS` below is `countof` of.
#[cfg (feature = "have_window_system")]
static mut standard_bitmaps: [fringe_bitmap; MAX_STANDARD_FRINGE_BITMAPS as usize] =
[
  fringe_bitmap { bits: core::ptr::null_mut (),
                  flags: fringe_bitmap_run (0, 0, 0, 0, 0) }, /* NO_FRINGE_BITMAP */
  FRBITS! (question_mark_bits,      8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (exclamation_mark_bits,   8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (left_arrow_bits,         8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (right_arrow_bits,        8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (up_arrow_bits,           8, 0, ALIGN_BITMAP_TOP,    0),
  FRBITS! (down_arrow_bits,         8, 0, ALIGN_BITMAP_BOTTOM, 0),
  FRBITS! (left_curly_arrow_bits,   8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (right_curly_arrow_bits,  8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (large_circle_bits,       8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (left_triangle_bits,      8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (right_triangle_bits,     8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (top_left_angle_bits,     8, 0, ALIGN_BITMAP_TOP,    0),
  FRBITS! (top_right_angle_bits,    8, 0, ALIGN_BITMAP_TOP,    0),
  FRBITS! (bottom_left_angle_bits,  8, 0, ALIGN_BITMAP_BOTTOM, 0),
  FRBITS! (bottom_right_angle_bits, 8, 0, ALIGN_BITMAP_BOTTOM, 0),
  FRBITS! (left_bracket_bits,       8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (right_bracket_bits,      8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (filled_rectangle_bits,   8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (hollow_rectangle_bits,   8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (filled_square_bits,      8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (hollow_square_bits,      8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (vertical_bar_bits,       8, 0, ALIGN_BITMAP_CENTER, 0),
  FRBITS! (horizontal_bar_bits,     8, 0, ALIGN_BITMAP_BOTTOM, 0),
  FRBITS! (empty_line_bits,         8, 3, ALIGN_BITMAP_TOP,    0),
];

/// C: 489: `#define NO_FRINGE_BITMAP 0`
#[cfg (feature = "have_window_system")]
pub const NO_FRINGE_BITMAP: c_int = 0;
/// C: 490: `#define UNDEF_FRINGE_BITMAP 1`
#[cfg (feature = "have_window_system")]
pub const UNDEF_FRINGE_BITMAP: c_int = 1;
/// C: 491: `#define MAX_STANDARD_FRINGE_BITMAPS countof (standard_bitmaps)` - counted at C 460-487:
/// `NO_FRINGE_BITMAP`'s record plus the twenty-four bitmaps.  It is also the length of
/// `standard_bitmaps` above, so the two cannot drift.
#[cfg (feature = "have_window_system")]
pub const MAX_STANDARD_FRINGE_BITMAPS: c_int = 25;

/// C: 493-495: `static struct fringe_bitmap **fringe_bitmaps;`, `static Lisp_Object *fringe_faces;`
/// and `static int max_fringe_bitmaps;` - three `static`s, zero-initialised, and none of them is
/// exported.
#[cfg (feature = "have_window_system")]
static mut fringe_bitmaps: *mut *mut fringe_bitmap = core::ptr::null_mut ();
#[cfg (feature = "have_window_system")]
static mut fringe_faces: *mut Lisp_Object = core::ptr::null_mut ();
#[cfg (feature = "have_window_system")]
static mut max_fringe_bitmaps: c_int = 0;

/// C: 497: `int max_used_fringe_bitmap = MAX_STANDARD_FRINGE_BITMAPS;` - a non-`static` object (its
/// own declaration is dispextern.h 3710), so it is exported.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub static mut max_used_fringe_bitmap: c_int = MAX_STANDARD_FRINGE_BITMAPS;


/* `include/dispextern.rs` carries the records a unit that includes it declares itself - the two
   face records, `bidi_it`, `draw_fringe_bitmap_params`, `composition_it`, `it` and
   `redisplay_interface` - inside `pub mod dispextern_unit_names`, so that *their* unit-side copies
   do not collide with them (the module's own note, status 3.220).  This unit declares none of them
   and needs three (`draw_fringe_bitmap_params`, `redisplay_interface` and the two `bool_bf`
   accessors), so it glob-imports the module - exactly what `src/msdos.rs` 530 does with the same
   mirror.  */
#[cfg (feature = "have_window_system")]
use dispextern_unit_names::*;

/* Declarations of functions that *other units* define and the mirrors this unit includes do not
   carry (PORTING.md 0.1-2 / P-72 rule 4).  They stand here, before the file's first item, and each
   keeps C's own prototype in a `/// C:` line with the header line it is at; the ledger's Owed 3
   names the mirror each of them belongs in.  Every one of them is inside dispextern.h's own
   `#ifdef HAVE_WINDOW_SYSTEM` region or is declared by it unconditionally, so the declarations are
   on the file gate like everything else.  */
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  /// C: dispextern.h 3577: `int window_text_bottom_y (struct window *);`
  pub fn window_text_bottom_y (w: *mut window) -> c_int;
  /// C: dispextern.h 3579: `int window_box_left (struct window *, enum glyph_row_area);`
  pub fn window_box_left (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3581: `int window_box_right (struct window *, enum glyph_row_area);`
  pub fn window_box_right (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3810: `void prepare_face_for_display (struct frame *, struct face *);` (the
  /// definition is xfaces.c 4764).
  pub fn prepare_face_for_display (f: *mut frame, face: *mut face);
  /// C: dispextern.h 3812: `int lookup_named_face (struct window *, struct frame *, Lisp_Object,
  /// bool);` (the definition is xfaces.c 5190).
  pub fn lookup_named_face (w: *mut window, f: *mut frame, symbol: Lisp_Object, ok: bool) -> c_int;
  /// C: dispextern.h 3816-3817: `int lookup_derived_face (struct window *, struct frame *,
  /// Lisp_Object, int, bool);` (the definition is xfaces.c 5384).
  pub fn lookup_derived_face (w: *mut window, f: *mut frame, symbol: Lisp_Object, face_id: c_int,
                              ok: bool) -> c_int;
}

/// `struct window`'s nineteen `bool_bf : 1` members (window.h 392-410) share the unit `bits` at
/// offset 500; the mirror carries the reader (`window.rs`'s own `window_bit`) and the shift
/// constants, but no writer, so this unit writes its own (the `xdisp.rs` 566 / `dispnew.rs` 393
/// shape - a mirror gap the ledger's Owed 3 names, and the block moves into the mirror when it gets
/// one).  C's `w->phys_cursor_on_p = v` is `window_bits! (phys_cursor_on_p, w, v)`; the read is
/// `window_bit (w, WINDOW_PHYS_CURSOR_ON_P_SHIFT)`.
#[cfg (feature = "have_window_system")]
macro_rules! window_bits {
  (phys_cursor_on_p, $w:expr, $value:expr) => {
    (*$w).bits = ((*$w).bits & ! (1u32 << WINDOW_PHYS_CURSOR_ON_P_SHIFT))
      | (((($value) as u32) & 1) << WINDOW_PHYS_CURSOR_ON_P_SHIFT)
  };
}

/// C: frame.h 1379: `#define FRAME_FACE_CACHE(F) (F)->face_cache` - a macro, and `include/frame.rs`
/// carries it as a function inside `pub mod frame_face_accessors` instead (that module's own note:
/// "the inline definition is one copy per including unit").  This unit's two readers of C's *macro*
/// spelling are the `FACE_FROM_ID_OR_NULL` copy below, so the copy stands here (the `xdisp.rs` 984 /
/// `window.rs` 913 / `font.rs` 274 shape; each of those units keeps its own).
#[cfg (feature = "have_window_system")]
macro_rules! FRAME_FACE_CACHE { ($f:expr) => { (*$f).face_cache as *mut face_cache }; }

/// C: frame.h 1656: `INLINE struct face *FACE_FROM_ID_OR_NULL (struct frame *f, int id);` - an
/// `INLINE` function, so no unit exports a symbol for it and each unit that calls one carries its
/// own copy (the `xdisp.rs` 1027 / `window.rs` shape).  `include/frame.rs`'s copy is inside
/// `pub mod frame_face_accessors`, a module this unit does not glob-import.
#[cfg (feature = "have_window_system")]
unsafe fn FACE_FROM_ID_OR_NULL (f: *mut frame, id: c_int) -> *mut face
{
  let used: c_int = (*FRAME_FACE_CACHE! (f)).used;
  eassume! (0 <= used);
  return if 0 <= id && id < used
    { *((*FRAME_FACE_CACHE! (f)).faces_by_id).offset (id as isize) }
  else { core::ptr::null_mut () };
}

/* Lookup bitmap number for symbol BITMAP.
   Return 0 if not a bitmap.  */

/// C: 503-504: `int` / `lookup_fringe_bitmap (Lisp_Object bitmap)` - non-`static`, so exported
/// (PORTING.md 0.2).
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_fringe_bitmap (mut bitmap: Lisp_Object) -> c_int
{
  let bn: EMACS_INT;

  bitmap = Fget (bitmap, Qfringe ());
  if !FIXNUMP (bitmap)
    { return 0; }

  bn = XFIXNUM (bitmap);
  if bn > NO_FRINGE_BITMAP as EMACS_INT
     && bn < max_used_fringe_bitmap as EMACS_INT
     && (bn < MAX_STANDARD_FRINGE_BITMAPS as EMACS_INT
	 || !(*fringe_bitmaps.add (bn as usize)).is_null ())
    { return bn as c_int; }

  return 0;
}

/* Get fringe bitmap name for bitmap number BN.

   Found by traversing Vfringe_bitmaps comparing BN to the
   fringe property for each symbol.

   Return BN if not found in Vfringe_bitmaps.  */

/// C: 529-530: `static Lisp_Object` / `get_fringe_bitmap_name (int bn)`
#[cfg (feature = "have_window_system")]
unsafe fn get_fringe_bitmap_name (bn: c_int) -> Lisp_Object
{
  let mut bitmaps: Lisp_Object;
  let num: Lisp_Object;

  /* Zero means no bitmap -- return nil.  */
  if bn <= 0
    { return Qnil (); }

  bitmaps = Vfringe_bitmaps;
  num = make_fixnum (bn as EMACS_INT);

  while CONSP (bitmaps)
    {
      let bitmap: Lisp_Object = XCAR (bitmaps);
      if EQ (num, Fget (bitmap, Qfringe ()))
	{ return bitmap; }
      bitmaps = XCDR (bitmaps);
    }

  return num;
}

/* Get fringe bitmap data for bitmap number BN.  */

/// C: 555-556: `static struct fringe_bitmap *` / `get_fringe_bitmap_data (int bn)`
#[cfg (feature = "have_window_system")]
unsafe fn get_fringe_bitmap_data (bn: c_int) -> *mut fringe_bitmap
{
  let mut fb: *mut fringe_bitmap;

  fb = *fringe_bitmaps.add (bn as usize);
  if fb.is_null ()
    { fb = core::ptr::addr_of_mut! (standard_bitmaps [if bn < MAX_STANDARD_FRINGE_BITMAPS
						       { bn } else { UNDEF_FRINGE_BITMAP }
						       as usize]); }

  return fb;
}

/* Draw the bitmap WHICH in one of the left or right fringes of
   window W.  ROW is the glyph row for which to display the bitmap; it
   determines the vertical position at which the bitmap has to be
   drawn.
   LEFT_P is 1 for left fringe, 0 for right fringe.
*/

/// C: 575-576: `static void` / `draw_fringe_bitmap_1 (struct window *w, struct glyph_row *row,
/// int left_p, int overlay, int which)`.  C's `struct draw_fringe_bitmap_params p;` (C 579) and its
/// five uninitialised locals are zero-initialised up front (PORTING.md 0.1-13): every one of them
/// is read only after the branch that assigns it.
#[cfg (feature = "have_window_system")]
unsafe fn draw_fringe_bitmap_1 (w: *mut window, row: *mut glyph_row, left_p: c_int,
				overlay: c_int, mut which: c_int)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let mut p: draw_fringe_bitmap_params = core::mem::zeroed ();
  let fb: *mut fringe_bitmap;
  let period: c_int;
  let mut face_id: c_int = DEFAULT_FACE_ID;
  let offset: c_int;
  let header_line_height: c_int;

  set_overlay_p (&mut p, (overlay & 1) == 1);
  set_cursor_p (&mut p, (overlay & 2) == 2);

  if which != NO_FRINGE_BITMAP
    {
      offset = 0;
    }
  else if left_p != 0
    {
      which = glyph_row_bits! (left_fringe_bitmap, row);
      face_id = glyph_row_bits! (left_fringe_face_id, row);
      offset = glyph_row_bits! (left_fringe_offset, row);
    }
  else
    {
      which = glyph_row_bits! (right_fringe_bitmap, row);
      face_id = glyph_row_bits! (right_fringe_face_id, row);
      /* C 602: `offset = row->right_fringe_offset;` - the mirror carries this one as a plain
	 `i8` field (its comment at dispextern.rs 1002 measures the eight-bit signed bitfield as a
	 whole byte), not through `glyph_row_bits!`.  */
      offset = (*row).right_fringe_offset as c_int;
    }

  if face_id == DEFAULT_FACE_ID
    {
      let face: Lisp_Object = *fringe_faces.add (which as usize);
      face_id = if NILP (face) { lookup_named_face (w, f, Qfringe (), false) }
		else { lookup_derived_face (w, f, face, FRINGE_FACE_ID, false) };
      if face_id < 0
	{ face_id = FRINGE_FACE_ID; }
    }

  fb = get_fringe_bitmap_data (which);

  period = fringe_bitmap_bits! (period, fb) as c_int;

  /* Convert row to frame coordinates.  */
  p.y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*row).y) + offset;

  p.which = which;
  p.bits = (*fb).bits;
  p.wd = fringe_bitmap_bits! (width, fb) as c_int;

  p.h = fringe_bitmap_bits! (height, fb) as c_int;
  p.dh = if period > 0 { p.y % period } else { 0 };
  p.h -= p.dh;

  /* Adjust y to the offset in the row to start drawing the bitmap.  */
  match fringe_bitmap_bits! (align, fb) as c_int
    {
      ALIGN_BITMAP_CENTER => p.y += ((*row).height - p.h) / 2,
      ALIGN_BITMAP_BOTTOM => p.y += (*row).visible_height - p.h,
      ALIGN_BITMAP_TOP => {},
      _ => {},
    }

  p.face = FACE_FROM_ID_OR_NULL (f, face_id);
  if p.face.is_null ()
    {
      /* This could happen after clearing face cache.
	 But it shouldn't happen anymore.  ++kfs */
      return;
    }

  prepare_face_for_display (f, p.face);

  /* Clear left fringe if no bitmap to draw or if bitmap doesn't fill
     the fringe.  */
  p.bx = -1;
  header_line_height = WINDOW_TAB_LINE_HEIGHT! (w)
		       + WINDOW_HEADER_LINE_HEIGHT! (w);
  p.by = WINDOW_TO_FRAME_PIXEL_Y! (w, max (header_line_height as isize,
					   (*row).y as isize) as c_int);
  p.ny = (*row).visible_height;
  if left_p != 0
    {
      let mut wd: c_int = WINDOW_LEFT_FRINGE_WIDTH! (w);
      let x: c_int = window_box_left (w, (if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
					  { LEFT_MARGIN_AREA }
					  else { TEXT_AREA }));
      if p.wd > wd
	{ p.wd = wd; }
      p.x = x - p.wd - (wd - p.wd) / 2;

      if p.wd < wd || p.y > p.by || p.y + p.h < p.by + p.ny
	{
	  /* If W has a vertical border to its left, don't draw over it.  */
	  wd -= if !WINDOW_LEFTMOST_P (w)
		/* This could be wrong when we allow window local
		   right dividers - but the window on the left is hard
		   to get.  */
		&& FRAME_RIGHT_DIVIDER_WIDTH (f) == 0
		&& !WINDOW_HAS_VERTICAL_SCROLL_BAR! (w)
		/* But don't reduce the fringe width if the window
		   has a left margin, because that means we are not
		   in danger of drawing over the vertical border,
		   and OTOH leaving out that one pixel leaves behind
		   traces of the cursor, if it was in column zero
		   before drawing non-empty margin area.  */
		&& (*w).left_margin_cols == 0
		{ 1 } else { 0 };
	  p.bx = x - wd;
	  p.nx = wd;
	}
    }
  else
    {
      let x: c_int = window_box_right (w, (if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
					   { RIGHT_MARGIN_AREA }
					   else { TEXT_AREA }));
      let wd: c_int = WINDOW_RIGHT_FRINGE_WIDTH! (w);
      if p.wd > wd
	{ p.wd = wd; }
      p.x = x + (wd - p.wd) / 2;
      /* Clear right fringe if no bitmap to draw of if bitmap doesn't fill
	 the fringe.  */
      if p.wd < wd || p.y > p.by || p.y + p.h < p.by + p.ny
	{
	  p.bx = x;
	  p.nx = wd;
	}
    }

  if p.x >= WINDOW_BOX_LEFT_EDGE_X! (w)
     && (p.x + p.wd) <= WINDOW_BOX_LEFT_EDGE_X! (w) + WINDOW_PIXEL_WIDTH (w)
    {
      let rif: *mut redisplay_interface = FRAME_RIF! (f) as *mut redisplay_interface;
      ((*rif).draw_fringe_bitmap.unwrap ()) (w as *mut c_void, row, &mut p);
    }
}

/// C: 714-715: `static int` / `get_logical_cursor_bitmap (struct window *w, Lisp_Object cursor)`.
/// C's two comma expressions at C 719 and C 732 are "assign, then test", so the port writes the
/// assignment as a statement and the test as the `if` (Deviation 5).
#[cfg (feature = "have_window_system")]
unsafe fn get_logical_cursor_bitmap (w: *mut window, cursor: Lisp_Object) -> c_int
{
  let mut cmap: Lisp_Object;
  let mut bm: Lisp_Object = Qnil ();

  cmap = BVAR! (XBUFFER ((*w).contents), fringe_cursor_alist_);
  if !NILP (cmap)
    {
      bm = Fassq (cursor, cmap);
      if CONSP (bm)
	{
	  bm = XCDR (bm);
	  if NILP (bm)
	    { return NO_FRINGE_BITMAP; }
	  return lookup_fringe_bitmap (bm);
	}
    }
  if EQ (cmap, BVAR! (&raw mut buffer_defaults, fringe_cursor_alist_))
    { return NO_FRINGE_BITMAP; }
  bm = Fassq (cursor, BVAR! (&raw mut buffer_defaults, fringe_cursor_alist_));
  if !CONSP (bm) || { bm = XCDR (bm); NILP (bm) }
    { return NO_FRINGE_BITMAP; }
  return lookup_fringe_bitmap (bm);
}

/// C: 737-738: `static int` / `get_logical_fringe_bitmap (struct window *w, Lisp_Object bitmap,
/// int right_p, int partial_p)`.  C's `found:` label (C 832) is a labelled block (PORTING.md 2.2):
/// the five `goto found` statements are `break 'found`, and the `return` that follows the label is
/// the block's follower.
#[cfg (feature = "have_window_system")]
unsafe fn get_logical_fringe_bitmap (w: *mut window, bitmap: Lisp_Object, right_p: c_int,
				     partial_p: c_int) -> c_int
{
  let mut cmap: Lisp_Object;
  let mut bm1: Lisp_Object = Qnil ();
  let mut bm2: Lisp_Object = Qnil ();
  let mut bm: Lisp_Object;
  let mut ln1: ptrdiff_t = 0;
  let mut ln2: ptrdiff_t = 0;
  let ix1: c_int = right_p;
  let ix2: c_int = ix1 + if partial_p != 0 { 2 } else { 0 };

  /* Lookup in buffer-local fringe-indicator-alist before global alist.

     Elements are:
	BITMAP		-- use for all
	(L R)		-- use for left right (whether partial or not)
	(L R PL PR)	-- use for left right partial-left partial-right
	If any value in local binding is not present or t, use global value.

     If partial, lookup partial bitmap in default value if not found here.
     If not partial, or no partial spec is present, use non-partial bitmap.  */

  'found: {
  cmap = BVAR! (XBUFFER ((*w).contents), fringe_indicator_alist_);
  if !NILP (cmap)
    {
      bm1 = Fassq (bitmap, cmap);
      if CONSP (bm1)
	{
	  bm1 = XCDR (bm1);
	  if NILP (bm1)
	    { return NO_FRINGE_BITMAP; }
	  if CONSP (bm1)
	    {
	      ln1 = list_length (bm1);
	      if partial_p != 0
		{
		  if ln1 > ix2 as ptrdiff_t
		    {
		      bm = Fnth (make_fixnum (ix2 as EMACS_INT), bm1);
		      if !EQ (bm, Qt ())
			{ break 'found; }
		    }
		}
	      else
		{
		  if ln1 > ix1 as ptrdiff_t
		    {
		      bm = Fnth (make_fixnum (ix1 as EMACS_INT), bm1);
		      if !EQ (bm, Qt ())
			{ break 'found; }
		    }
		}
	    }
	  else
	    {
	      bm = bm1;
	      if !EQ (bm, Qt ())
		{ break 'found; }
	    }
	}
    }

  if !EQ (cmap, BVAR! (&raw mut buffer_defaults, fringe_indicator_alist_))
     && !NILP (BVAR! (&raw mut buffer_defaults, fringe_indicator_alist_))
    {
      bm2 = Fassq (bitmap, BVAR! (&raw mut buffer_defaults, fringe_indicator_alist_));
      if CONSP (bm2)
	{
	  bm2 = XCDR (bm2);
	  if !NILP (bm2)
	    {
	      if CONSP (bm2)
		{
		  ln2 = list_length (bm2);
		  if partial_p != 0
		    {
		      if ln2 > ix2 as ptrdiff_t
			{
			  bm = Fnth (make_fixnum (ix2 as EMACS_INT), bm2);
			  if !EQ (bm, Qt ())
			    { break 'found; }
			}
		    }
		}
	    }
	}
    }

  if ln1 > ix1 as ptrdiff_t
    {
      bm = Fnth (make_fixnum (ix1 as EMACS_INT), bm1);
      if !EQ (bm, Qt ())
	{ break 'found; }
    }

  if ln2 > ix1 as ptrdiff_t
    {
      bm = Fnth (make_fixnum (ix1 as EMACS_INT), bm2);
      if !EQ (bm, Qt ())
	{ break 'found; }
      return NO_FRINGE_BITMAP;
    }
  else
    {
      bm = bm2;
      if NILP (bm)
	{ return NO_FRINGE_BITMAP; }
    }
  }

  return lookup_fringe_bitmap (bm);
}

/// C: 837-838: `void` / `draw_fringe_bitmap (struct window *w, struct glyph_row *row, int left_p)`
/// - non-`static`, so exported.  C 863-867's `case NO_CURSOR: default:` is a shared body, which Rust
/// has no way to spell inside one `match`: the two arms below carry it twice (Deviation 4).
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn draw_fringe_bitmap (w: *mut window, row: *mut glyph_row, left_p: c_int)
{
  let mut overlay: c_int = 0;

  if (left_p != 0) == glyph_row_bits! (reversed_p, row)
     && glyph_row_bits! (cursor_in_fringe_p, row)
    {
      let mut cursor: Lisp_Object = Qnil ();

      match (*w).phys_cursor_type
	{
	  HOLLOW_BOX_CURSOR => {
	    if (*row).visible_height as usize >= STANDARD_BITMAP_HEIGHT! (hollow_rectangle_bits)
	      { cursor = Qhollow (); }
	    else
	      { cursor = Qhollow_small (); }
	  }
	  FILLED_BOX_CURSOR => { cursor = Qbox (); }
	  BAR_CURSOR => { cursor = Qbar (); }
	  HBAR_CURSOR => { cursor = Qhbar (); }
	  NO_CURSOR => {
	    window_bits! (phys_cursor_on_p, w, false);
	    glyph_row_bits! (cursor_in_fringe_p, row, false);
	  }
	  _ => {
	    window_bits! (phys_cursor_on_p, w, false);
	    glyph_row_bits! (cursor_in_fringe_p, row, false);
	  }
	}
      if !NILP (cursor)
	{
	  let bm: c_int = get_logical_cursor_bitmap (w, cursor);
	  if bm != NO_FRINGE_BITMAP
	    {
	      draw_fringe_bitmap_1 (w, row, left_p, 2, bm);
	      overlay = if EQ (cursor, Qbox ()) { 3 } else { 1 };
	    }
	}
    }

  draw_fringe_bitmap_1 (w, row, left_p, overlay, NO_FRINGE_BITMAP);

  if left_p != 0 && (*row).overlay_arrow_bitmap != NO_FRINGE_BITMAP
    { draw_fringe_bitmap_1 (w, row, 1, 1, (*row).overlay_arrow_bitmap); }
}


/* Draw fringe bitmaps for glyph row ROW on window W.  Call this
   function with input blocked.  */

/// C: 890-891: `void` / `draw_row_fringe_bitmaps (struct window *w, struct glyph_row *row)` -
/// non-`static`, so exported.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn draw_row_fringe_bitmaps (w: *mut window, row: *mut glyph_row)
{
  eassert! (input_blocked_p ());

  /* If row is completely invisible, because of vscrolling, we
     don't have to draw anything.  */
  if (*row).visible_height <= 0
    { return; }

  if WINDOW_LEFT_FRINGE_WIDTH! (w) != 0
    { draw_fringe_bitmap (w, row, 1); }

  if WINDOW_RIGHT_FRINGE_WIDTH! (w) != 0
    { draw_fringe_bitmap (w, row, 0); }
}

/* Draw the fringes of window W.  Only fringes for rows marked for
   update in redraw_fringe_bitmaps_p are drawn.

   Return nonzero if left or right fringe was redrawn in any way.

   If NO_FRINGE_P is non-zero, also return nonzero if either fringe
   has zero width.

   A return nonzero value indicates that the vertical line between
   windows needs update (as it may be drawn in the fringe).
*/

/// C: 919-920: `bool` / `draw_window_fringes (struct window *w, bool no_fringe_p)` - non-`static`,
/// so exported.  C's `for` loop increments on `continue` too, which a Rust `while` with the
/// increment written at both sites spells (the `ftfont.rs` shape, Deviation 6).
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn draw_window_fringes (w: *mut window, no_fringe_p: bool) -> bool
{
  let mut row: *mut glyph_row;
  let yb: c_int = window_text_bottom_y (w);
  let nrows: c_int = (*( (*w).current_matrix as *mut glyph_matrix )).nrows;
  let mut y: c_int;
  let mut rn: c_int;
  let mut updated_p: bool = false;

  if window_bit (w, WINDOW_PSEUDO_WINDOW_P_SHIFT)
    { return updated_p; }

  /* We must switch to the window's buffer to use its local value of
     the fringe face, in case it's been remapped in face-remapping-alist.  */
  let window_buffer: Lisp_Object = (*w).contents;
  let oldbuf: *mut buffer = current_buffer;
  set_buffer_internal_1 (XBUFFER (window_buffer));

  /* Must draw line if no fringe */
  if no_fringe_p
     && (WINDOW_LEFT_FRINGE_WIDTH! (w) == 0
	 || WINDOW_RIGHT_FRINGE_WIDTH! (w) == 0)
    { updated_p = true; }

  y = (*w).vscroll;
  rn = 0;
  row = (*( (*w).current_matrix as *mut glyph_matrix )).rows;
  while y < yb && rn < nrows
    {
      if !glyph_row_bits! (redraw_fringe_bitmaps_p, row)
	{
	  y += (*row).height; row = row.add (1); rn += 1;
	  continue;
	}
      draw_row_fringe_bitmaps (w, row);
      glyph_row_bits! (redraw_fringe_bitmaps_p, row, false);
      updated_p = true;
      y += (*row).height; row = row.add (1); rn += 1;
    }

  set_buffer_internal_1 (oldbuf);

  return updated_p;
}

/* Recalculate the bitmaps to show in the fringes of window W.
   Only mark rows with modified bitmaps for update in redraw_fringe_bitmaps_p.

   If KEEP_CURRENT_P is 0, update current_matrix too.  */

/// C: 965-966: `bool` / `update_window_fringes (struct window *w, bool keep_current_p)` -
/// non-`static`, so exported.  C's `UNINIT` pair at C 986 (`top_row_ends_at_zv_p`,
/// `bot_row_ends_at_zv_p`) is read only on the path C 982-985's comment describes, which Rust's
/// definite-assignment analysis cannot see, so the two start at zero (PORTING.md 0.1-13/1.5,
/// Deviation 9).  C's three comma statements (`redraw_p = 1, row->... = 1;`) are two statements in
/// the port (Deviation 13).
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_window_fringes (w: *mut window, keep_current_p: bool) -> bool
{
  let mut row: *mut glyph_row;
  let mut cur: *mut glyph_row = core::ptr::null_mut ();
  let yb: c_int = window_text_bottom_y (w);
  let mut rn: c_int;
  let nrows: c_int = (*( (*w).current_matrix as *mut glyph_matrix )).nrows;
  let mut y: c_int;
  let mut redraw_p: bool = false;
  let mut boundary_top: Lisp_Object = Qnil ();
  let mut boundary_bot: Lisp_Object = Qnil ();
  let mut arrow_top: Lisp_Object = Qnil ();
  let mut arrow_bot: Lisp_Object = Qnil ();
  let mut empty_pos: Lisp_Object;
  let mut ind: Lisp_Object = Qnil ();
  /* C 977: `#define MAX_BITMAP_CACHE (8*4)`.  */
  const MAX_BITMAP_CACHE: usize = 8 * 4;
  let mut bitmap_cache: [c_int; MAX_BITMAP_CACHE] = [0; MAX_BITMAP_CACHE];
  let mut top_ind_rn: c_int;
  let mut bot_ind_rn: c_int;
  let mut top_ind_min_y: c_int;
  let mut bot_ind_max_y: c_int;

  /* top_ind_rn is set to a nonnegative value whenever
     row->indicate_bob_p is set, so it's OK that top_row_ends_at_zv_p
     is not initialized here.  Similarly for bot_ind_rn,
     row->indicate_eob_p and bot_row_ends_at_zv_p.  */
  let mut top_row_ends_at_zv_p: c_int = 0;
  let mut bot_row_ends_at_zv_p: c_int = 0;

  if window_bit (w, WINDOW_PSEUDO_WINDOW_P_SHIFT)
    { return false; }

  let count: specpdl_ref = SPECPDL_INDEX ();

  /* This function could be called for redisplaying non-selected
     windows, in which case point has been temporarily moved to that
     window's window-point.  So we cannot afford quitting out of here,
     as point is restored after this function returns.  */
  specbind (Qinhibit_quit (), Qt ());

  ind = BVAR! (XBUFFER ((*w).contents), indicate_buffer_boundaries_);
  if !MINI_WINDOW_P (w) && !NILP (ind)
    {
      if EQ (ind, Qleft ()) || EQ (ind, Qright ())
	{
	  boundary_top = ind;
	  boundary_bot = ind;
	  arrow_top = ind;
	  arrow_bot = ind;
	}
      else if CONSP (ind) && CONSP (XCAR (ind))
	{
	  let mut pos: Lisp_Object;
	  pos = Fassq (Qt (), ind);
	  if !NILP (pos)
	    {
	      boundary_top = XCDR (pos);
	      boundary_bot = XCDR (pos);
	      arrow_top = XCDR (pos);
	      arrow_bot = XCDR (pos);
	    }
	  pos = Fassq (Qtop (), ind);
	  if !NILP (pos)
	    { boundary_top = XCDR (pos); }
	  pos = Fassq (Qbottom (), ind);
	  if !NILP (pos)
	    { boundary_bot = XCDR (pos); }
	  pos = Fassq (Qup (), ind);
	  if !NILP (pos)
	    { arrow_top = XCDR (pos); }
	  pos = Fassq (Qdown (), ind);
	  if !NILP (pos)
	    { arrow_bot = XCDR (pos); }
	}
      else
	/* Anything else means boundary on left and no arrows.  */
	{
	  boundary_top = Qleft ();
	  boundary_bot = Qleft ();
	}
    }

  top_ind_rn = -1;
  bot_ind_rn = -1;
  if !NILP (ind)
    {
      y = (*w).vscroll;
      rn = 0;
      while y < yb && rn < nrows
	{
	  row = MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix, rn);
	  if !glyph_row_bits! (enabled_p, row)
	    { row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, rn); }

	  glyph_row_bits! (indicate_bob_p, row, false);
	  glyph_row_bits! (indicate_top_line_p, row, false);
	  glyph_row_bits! (indicate_eob_p, row, false);
	  glyph_row_bits! (indicate_bottom_line_p, row, false);

	  if !glyph_row_bits! (mode_line_p, row)
	    {
	      if top_ind_rn < 0 && (*row).visible_height > 0
		{
		  if MATRIX_ROW_START_CHARPOS! (row) <= BUF_BEGV (XBUFFER ((*w).contents))
		     && !MATRIX_ROW_PARTIALLY_VISIBLE_AT_TOP_P! (w, row)
		    { glyph_row_bits! (indicate_bob_p, row, !NILP (boundary_top)); }
		  else
		    { glyph_row_bits! (indicate_top_line_p, row, !NILP (arrow_top)); }
		  top_ind_rn = rn;
		}

	      if bot_ind_rn < 0
		{
		  if MATRIX_ROW_END_CHARPOS! (row) >= BUF_ZV (XBUFFER ((*w).contents))
		     && !MATRIX_ROW_PARTIALLY_VISIBLE_AT_BOTTOM_P! (w, row)
		    {
		      glyph_row_bits! (indicate_eob_p, row, !NILP (boundary_bot));
		      bot_ind_rn = rn;
		    }
		  else if y + (*row).height >= yb
		    {
		      glyph_row_bits! (indicate_bottom_line_p, row, !NILP (arrow_bot));
		      bot_ind_rn = rn;
		    }
		}
	    }
	  y += (*row).height; rn += 1;
	}
    }

  empty_pos = BVAR! (XBUFFER ((*w).contents), indicate_empty_lines_);
  if !NILP (empty_pos) && !EQ (empty_pos, Qright ())
    { empty_pos = if WINDOW_LEFT_FRINGE_WIDTH! (w) == 0 { Qright () } else { Qleft () }; }

  y = 0;
  while y < MAX_BITMAP_CACHE as c_int
    {
      bitmap_cache [y as usize] = -1;
      y += 1;
    }

  /* C 1068-1078: the two macros.  C's `(bitmap_cache[...] = f (...))` yields the assigned value,
     and a Rust assignment yields `()`, so each arm's `else` assigns and then reads the slot back:
     the same three evaluations of `cache`/`partial_p` and the same answer (Deviation 3).  */
  macro_rules! LEFT_FRINGE {
    ($cache:expr, $which:expr, $partial_p:expr) => {{
      if bitmap_cache [(($cache) as usize) * 4 + ($partial_p) as usize] >= 0
	{ bitmap_cache [(($cache) as usize) * 4 + ($partial_p) as usize] }
      else
	{
	  bitmap_cache [(($cache) as usize) * 4 + ($partial_p) as usize]
	    = get_logical_fringe_bitmap (w, $which, 0, ($partial_p) as c_int);
	  bitmap_cache [(($cache) as usize) * 4 + ($partial_p) as usize]
	}
    }};
  }

  macro_rules! RIGHT_FRINGE {
    ($cache:expr, $which:expr, $partial_p:expr) => {{
      if bitmap_cache [(($cache) as usize) * 4 + 2 + ($partial_p) as usize] >= 0
	{ bitmap_cache [(($cache) as usize) * 4 + 2 + ($partial_p) as usize] }
      else
	{
	  bitmap_cache [(($cache) as usize) * 4 + 2 + ($partial_p) as usize]
	    = get_logical_fringe_bitmap (w, $which, 1, ($partial_p) as c_int);
	  bitmap_cache [(($cache) as usize) * 4 + 2 + ($partial_p) as usize]
	}
    }};
  }


  /* Extend top-aligned top indicator (or bottom-aligned bottom
     indicator) to adjacent rows if it doesn't fit in one row.  */
  top_ind_min_y = -1;
  bot_ind_max_y = -1;
  if top_ind_rn >= 0
    {
      let mut bn: c_int = NO_FRINGE_BITMAP;

      row = MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix, top_ind_rn);
      if !glyph_row_bits! (enabled_p, row)
	{ row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, top_ind_rn); }

      top_row_ends_at_zv_p = glyph_row_bits! (ends_at_zv_p, row) as c_int;
      if glyph_row_bits! (indicate_bob_p, row)
	{
	  if EQ (boundary_top, Qleft ())
	    {
	      bn = if glyph_row_bits! (indicate_eob_p, row) && EQ (boundary_bot, Qleft ())
		   { LEFT_FRINGE! (1, Qtop_bottom (), glyph_row_bits! (ends_at_zv_p, row)) }
		   else { LEFT_FRINGE! (2, Qtop (), 0) };
	    }
	  else
	    {
	      bn = if glyph_row_bits! (indicate_eob_p, row) && EQ (boundary_bot, Qright ())
		   { RIGHT_FRINGE! (1, Qtop_bottom (), glyph_row_bits! (ends_at_zv_p, row)) }
		   else { RIGHT_FRINGE! (2, Qtop (), 0) };
	    }
	}
      else if glyph_row_bits! (indicate_top_line_p, row)
	{
	  if EQ (arrow_top, Qleft ())
	    { bn = LEFT_FRINGE! (6, Qup (), 0); }
	  else
	    { bn = RIGHT_FRINGE! (6, Qup (), 0); }
	}

      if bn != NO_FRINGE_BITMAP
	{
	  let fb: *mut fringe_bitmap = get_fringe_bitmap_data (bn);

	  if fringe_bitmap_bits! (align, fb) as c_int == ALIGN_BITMAP_TOP
	     && fringe_bitmap_bits! (period, fb) == 0
	    {
	      let mut row1: *mut glyph_row;
	      let mut top_ind_max_y: c_int;

	      top_ind_min_y = WINDOW_TAB_LINE_HEIGHT! (w)
			      + WINDOW_HEADER_LINE_HEIGHT! (w);
	      top_ind_max_y = top_ind_min_y + fringe_bitmap_bits! (height, fb) as c_int;
	      if top_ind_max_y > yb
		{ top_ind_max_y = yb; }

	      y = (*row).y + (*row).height;
	      rn = top_ind_rn + 1;
	      while y < top_ind_max_y && rn < nrows
		{
		  if bot_ind_rn >= 0 && rn >= bot_ind_rn
		    { break; }

		  row1 = MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix, rn);
		  if !glyph_row_bits! (enabled_p, row1)
		    { row1 = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, rn); }

		  glyph_row_bits! (indicate_bob_p, row1, glyph_row_bits! (indicate_bob_p, row));
		  glyph_row_bits! (indicate_top_line_p, row1,
				   glyph_row_bits! (indicate_top_line_p, row));
		  y += (*row1).height; rn += 1;
		}
	    }
	}
    }
  if bot_ind_rn >= 0
    {
      let mut bn: c_int = NO_FRINGE_BITMAP;

      row = MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix, bot_ind_rn);
      if !glyph_row_bits! (enabled_p, row)
	{ row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, bot_ind_rn); }

      bot_row_ends_at_zv_p = glyph_row_bits! (ends_at_zv_p, row) as c_int;
      if glyph_row_bits! (indicate_eob_p, row)
	{
	  if EQ (boundary_bot, Qleft ())
	    { bn = LEFT_FRINGE! (3, Qbottom (), glyph_row_bits! (ends_at_zv_p, row)); }
	  else
	    { bn = RIGHT_FRINGE! (3, Qbottom (), glyph_row_bits! (ends_at_zv_p, row)); }
	}
      else if glyph_row_bits! (indicate_bottom_line_p, row)
	{
	  if EQ (arrow_bot, Qleft ())
	    { bn = LEFT_FRINGE! (7, Qdown (), 0); }
	  else
	    { bn = RIGHT_FRINGE! (7, Qdown (), 0); }
	}

      if bn != NO_FRINGE_BITMAP
	{
	  let fb: *mut fringe_bitmap = get_fringe_bitmap_data (bn);

	  if fringe_bitmap_bits! (align, fb) as c_int == ALIGN_BITMAP_BOTTOM
	     && fringe_bitmap_bits! (period, fb) == 0
	    {
	      let mut row1: *mut glyph_row;
	      let mut bot_ind_min_y: c_int;

	      bot_ind_max_y = (*row).y + (*row).visible_height;
	      bot_ind_min_y = bot_ind_max_y - fringe_bitmap_bits! (height, fb) as c_int;
	      if bot_ind_min_y < WINDOW_TAB_LINE_HEIGHT! (w)
				  + WINDOW_HEADER_LINE_HEIGHT! (w)
		{
		  bot_ind_min_y = WINDOW_TAB_LINE_HEIGHT! (w)
				  + WINDOW_HEADER_LINE_HEIGHT! (w);
		}

	      y = (*row).y;
	      rn = bot_ind_rn - 1;
	      while y >= bot_ind_min_y && rn >= 0
		{
		  if top_ind_rn >= 0 && rn <= top_ind_rn
		    { break; }

		  row1 = MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix, rn);
		  if !glyph_row_bits! (enabled_p, row1)
		    { row1 = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, rn); }

		  glyph_row_bits! (indicate_eob_p, row1, glyph_row_bits! (indicate_eob_p, row));
		  glyph_row_bits! (indicate_bottom_line_p, row1,
				   glyph_row_bits! (indicate_bottom_line_p, row));
		  y -= (*row1).height; rn -= 1;
		}
	    }
	}
    }

  y = (*w).vscroll;
  rn = 0;
  while y < yb && rn < nrows
    {
      let mut left: c_int;
      let mut right: c_int;
      let mut left_face_id: c_uint;
      let mut right_face_id: c_uint;
      let mut left_offset: c_int;
      let mut right_offset: c_int;
      let mut periodic_p: bool;

      row = MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix, rn);
      cur = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, rn);
      if !glyph_row_bits! (enabled_p, row)
	{ row = cur; }

      left_face_id = DEFAULT_FACE_ID as c_uint;
      right_face_id = DEFAULT_FACE_ID as c_uint;
      left_offset = 0;
      right_offset = 0;
      periodic_p = false;

      /* Decide which bitmap to draw in the left fringe.  */
      if WINDOW_LEFT_FRINGE_WIDTH! (w) == 0
	{ left = NO_FRINGE_BITMAP; }
      else if glyph_row_bits! (left_user_fringe_bitmap, row) != NO_FRINGE_BITMAP
	{
	  left = glyph_row_bits! (left_user_fringe_bitmap, row);
	  left_face_id = glyph_row_bits! (left_user_fringe_face_id, row) as c_uint;
	}
      else if (!glyph_row_bits! (reversed_p, row)
	       && glyph_row_bits! (truncated_on_left_p, row))
	      || (glyph_row_bits! (reversed_p, row)
		  && glyph_row_bits! (truncated_on_right_p, row))
	{ left = LEFT_FRINGE! (0, Qtruncation (), 0); }
      else if glyph_row_bits! (indicate_bob_p, row) && EQ (boundary_top, Qleft ())
	{
	  left = if glyph_row_bits! (indicate_eob_p, row) && EQ (boundary_bot, Qleft ())
		 { LEFT_FRINGE! (1, Qtop_bottom (), top_row_ends_at_zv_p) }
		 else { LEFT_FRINGE! (2, Qtop (), 0) };
	  if top_ind_min_y >= 0
	    { left_offset = top_ind_min_y - (*row).y; }
	}
      else if glyph_row_bits! (indicate_eob_p, row) && EQ (boundary_bot, Qleft ())
	{
	  left = LEFT_FRINGE! (3, Qbottom (), bot_row_ends_at_zv_p);
	  if bot_ind_max_y >= 0
	    { left_offset = bot_ind_max_y - ((*row).y + (*row).visible_height); }
	}
      else if (!glyph_row_bits! (reversed_p, row) && MATRIX_ROW_CONTINUATION_LINE_P! (row))
	      || (glyph_row_bits! (reversed_p, row) && glyph_row_bits! (continued_p, row))
	{ left = LEFT_FRINGE! (4, Qcontinuation (), 0); }
      else if glyph_row_bits! (indicate_empty_line_p, row) && EQ (empty_pos, Qleft ())
	{ left = LEFT_FRINGE! (5, Qempty_line (), 0); }
      else if glyph_row_bits! (indicate_top_line_p, row) && EQ (arrow_top, Qleft ())
	{
	  left = LEFT_FRINGE! (6, Qup (), 0);
	  if top_ind_min_y >= 0
	    { left_offset = top_ind_min_y - (*row).y; }
	}
      else if glyph_row_bits! (indicate_bottom_line_p, row) && EQ (arrow_bot, Qleft ())
	{
	  left = LEFT_FRINGE! (7, Qdown (), 0);
	  if bot_ind_max_y >= 0
	    { left_offset = bot_ind_max_y - ((*row).y + (*row).visible_height); }
	}
      else
	{ left = NO_FRINGE_BITMAP; }

      /* Decide which bitmap to draw in the right fringe.  */
      if WINDOW_RIGHT_FRINGE_WIDTH! (w) == 0
	{ right = NO_FRINGE_BITMAP; }
      else if glyph_row_bits! (right_user_fringe_bitmap, row) != NO_FRINGE_BITMAP
	{
	  right = glyph_row_bits! (right_user_fringe_bitmap, row);
	  right_face_id = glyph_row_bits! (right_user_fringe_face_id, row) as c_uint;
	}
      else if (!glyph_row_bits! (reversed_p, row)
	       && glyph_row_bits! (truncated_on_right_p, row))
	      || (glyph_row_bits! (reversed_p, row)
		  && glyph_row_bits! (truncated_on_left_p, row))
	{ right = RIGHT_FRINGE! (0, Qtruncation (), 0); }
      else if glyph_row_bits! (indicate_bob_p, row) && EQ (boundary_top, Qright ())
	{
	  right = if glyph_row_bits! (indicate_eob_p, row) && EQ (boundary_bot, Qright ())
		  { RIGHT_FRINGE! (1, Qtop_bottom (), top_row_ends_at_zv_p) }
		  else { RIGHT_FRINGE! (2, Qtop (), 0) };
	  if top_ind_min_y >= 0
	    { right_offset = top_ind_min_y - (*row).y; }
	}
      else if glyph_row_bits! (indicate_eob_p, row) && EQ (boundary_bot, Qright ())
	{
	  right = RIGHT_FRINGE! (3, Qbottom (), bot_row_ends_at_zv_p);
	  if bot_ind_max_y >= 0
	    { right_offset = bot_ind_max_y - ((*row).y + (*row).visible_height); }
	}
      else if (!glyph_row_bits! (reversed_p, row) && glyph_row_bits! (continued_p, row))
	      || (glyph_row_bits! (reversed_p, row) && MATRIX_ROW_CONTINUATION_LINE_P! (row))
	{ right = RIGHT_FRINGE! (4, Qcontinuation (), 0); }
      else if glyph_row_bits! (indicate_top_line_p, row) && EQ (arrow_top, Qright ())
	{
	  right = RIGHT_FRINGE! (6, Qup (), 0);
	  if top_ind_min_y >= 0
	    { right_offset = top_ind_min_y - (*row).y; }
	}
      else if glyph_row_bits! (indicate_bottom_line_p, row) && EQ (arrow_bot, Qright ())
	{
	  right = RIGHT_FRINGE! (7, Qdown (), 0);
	  if bot_ind_max_y >= 0
	    { right_offset = bot_ind_max_y - ((*row).y + (*row).visible_height); }
	}
      else if glyph_row_bits! (indicate_empty_line_p, row) && EQ (empty_pos, Qright ())
	{ right = RIGHT_FRINGE! (5, Qempty_line (), 0); }
      else
	{ right = NO_FRINGE_BITMAP; }

      periodic_p = (fringe_bitmap_bits! (period, get_fringe_bitmap_data (left)) != 0
		    || fringe_bitmap_bits! (period, get_fringe_bitmap_data (right)) != 0);

      if (*row).y != (*cur).y
	 || (*row).visible_height != (*cur).visible_height
	 || glyph_row_bits! (ends_at_zv_p, row) != glyph_row_bits! (ends_at_zv_p, cur)
	 || left != glyph_row_bits! (left_fringe_bitmap, cur)
	 || right != glyph_row_bits! (right_fringe_bitmap, cur)
	 || left_face_id != glyph_row_bits! (left_fringe_face_id, cur) as c_uint
	 || right_face_id != glyph_row_bits! (right_fringe_face_id, cur) as c_uint
	 || left_offset != glyph_row_bits! (left_fringe_offset, cur)
	 || right_offset != (*cur).right_fringe_offset as c_int
	 || periodic_p != glyph_row_bits! (fringe_bitmap_periodic_p, cur)
	 || glyph_row_bits! (redraw_fringe_bitmaps_p, cur)
	{
	  redraw_p = true;
	  glyph_row_bits! (redraw_fringe_bitmaps_p, row, true);
	  if !keep_current_p
	    {
	      glyph_row_bits! (redraw_fringe_bitmaps_p, cur, true);
	      glyph_row_bits! (left_fringe_bitmap, cur, left);
	      glyph_row_bits! (right_fringe_bitmap, cur, right);
	      glyph_row_bits! (left_fringe_face_id, cur, left_face_id as c_int);
	      glyph_row_bits! (right_fringe_face_id, cur, right_face_id as c_int);
	      glyph_row_bits! (left_fringe_offset, cur, left_offset);
	      (*cur).right_fringe_offset = right_offset as i8;
	      glyph_row_bits! (fringe_bitmap_periodic_p, cur, periodic_p);
	    }
	}

      if (*row).overlay_arrow_bitmap < 0
	{ (*row).overlay_arrow_bitmap = get_logical_fringe_bitmap (w, Qoverlay_arrow (), 0, 0); }

      if (*row).overlay_arrow_bitmap != (*cur).overlay_arrow_bitmap
	{
	  redraw_p = true;
	  glyph_row_bits! (redraw_fringe_bitmaps_p, row, true);
	  if !keep_current_p
	    {
	      glyph_row_bits! (redraw_fringe_bitmaps_p, cur, true);
	      (*cur).overlay_arrow_bitmap = (*row).overlay_arrow_bitmap;
	    }
	}

      glyph_row_bits! (left_fringe_bitmap, row, left);
      glyph_row_bits! (right_fringe_bitmap, row, right);
      glyph_row_bits! (left_fringe_face_id, row, left_face_id as c_int);
      glyph_row_bits! (right_fringe_face_id, row, right_face_id as c_int);
      glyph_row_bits! (left_fringe_offset, row, left_offset);
      (*row).right_fringe_offset = right_offset as i8;
      glyph_row_bits! (fringe_bitmap_periodic_p, row, periodic_p);
      y += (*row).height; rn += 1;
    }

  unbind_to (count, Qnil ());

  return redraw_p && !keep_current_p;
}
