// femacs: port of src/sfntfont-android.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements, same order, same names.  P-86's unit, the whole
// file C 1-821.  Gates: `all (feature = "have_android", feature = "not_android_stubify")` on every
// item *and on C 20-37's include graph*; C 26/74/195/326/361/490's `__aarch64__` pairs as
// `feature = "aarch64"` / `not (feature = "aarch64")`; C 666's `HAVE_HARFBUZZ` as
// `feature = "harfbuzz"`; C 250's `#if 0` dead.  `docs/ports/sfntfont-android.md` carries the
// line-range table, the readings and the deviations.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn,
         unused_assignments, unused_parens, unused_mut, unreachable_code, unused_labels, while_true,
         unused_comparisons, overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 20: `#include <config.h>`

/* C 21-37: the file's own include list, in C's order, under the file gate (P-86 8).  C 26-28's
   `<arm_neon.h>` arm has no mirror (the intrinsics are `core::arch::aarch64`'s, on C 195-309's
   arm below) and C 30's `<android/api-level.h>` names nothing here, so both are comments (P-88 1).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/dirent.in.rs");   // C 21: `#include <dirent.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/string.rs");      // C 22: `#include <string.h>` (strcmp, memcmp, memset)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdio.rs");       // C 23: `#include <stdio.h>` (sprintf, snprintf)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdlib.rs");      // C 24: `#include <stdlib.h>` (free)
/* C 31: `#include <android/log.h>` - the NDK's header is not in the reference tree, so the one
   level and the one entry point this file names are declared in place (P-88 1).  Measured with this
   machine's NDK 29.0.14206865 (`--target=aarch64-linux-android24`, the ledger's `## Evidence`):
   `ANDROID_LOG_VERBOSE` 2, `log.h` 112 `int __android_log_print(int, const char*, const char*, ...)`.  */
/* `android.h` 157's `android_get_current_api_level ()` (the mirror, `include/android.rs` 189-196)
   names `__ANDROID_API__` and leaves it to the consumer unit - the NDK toolchain defines it
   (`-D__ANDROID_API__=24`); the `androidmenu.rs` 68-73 shape.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))] pub const __ANDROID_API__: c_int = 24;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))] pub const ANDROID_LOG_VERBOSE: c_int = 2;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: log.h 112: `int __android_log_print(int prio, const char* tag, const char* fmt, ...)`
  pub fn __android_log_print (prio: c_int, tag: *const c_char, fmt: *const c_char, ...) -> c_int;
}
/* C's `androidterm.h` 25 -> `character.h` 26 reaches `lisp.h` before `font.h`'s inline bodies
   (`FONT_OBJECT` and friends use `XUNTAG`/`eassert`) are read, and the port's
   `include/character.rs` carries no such edge yet (Owed: a mirror round adds it), so the unit
   places it here, where C's graph has it (P-72 rule 4).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/lisp.rs");               // androidterm.h 25 -> character.h 26: `#include "lisp.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/androidterm.rs");        // C 33: `#include "androidterm.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/sfntfont.rs");           // C 34: `#include "sfntfont.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/pdumper.rs");            // C 35: `#include "pdumper.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/blockinput.rs");         // C 36: `#include "blockinput.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/android.rs");            // C 37: `#include "android.h"`
/* Two `lisp.h` edges (C 34's `sfntfont.h` reaches that header) this file needs: `<stdcountof.h>`
   for C 704's `countof` and lisp.h 71-72's `min`/`max` for C 353/391-394 and C 103; the
   `include/lisp.rs` mirror carries neither (the `sfntfont.rs` 55-58 shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdcountof.rs");  // lisp.h 34's `<stdcountof.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/min-max.rs");     // lisp.h 71-72: `max`/`min`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/limits.in.rs");   // lisp.h 33's `<limits.h>` (PATH_MAX)

/* `<android/bitmap.h>` (C 37's `android.h` pulls it in at android.h 39): the NDK's header has no
   mirror and `include/android.rs` 44-46 keeps its tag opaque (android.h names it only through a
   pointer) while this file *reads* the five members, so the record is declared here (P-88 1) in a
   module of its own, which leaves the mirror its tag and this unit C's name and members.  Measured
   on the same NDK: `sizeof` 20, offsets 0/4/8/12/16, `ANDROID_BITMAP_FORMAT_RGBA_8888` 1,
   `bitmap.h` 156 `int AndroidBitmap_unlockPixels(JNIEnv*, jobject)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))] pub const ANDROID_BITMAP_FORMAT_RGBA_8888: c_int = 1;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: bitmap.h 156: `int AndroidBitmap_unlockPixels(JNIEnv* env, jobject jbitmap);`
  pub fn AndroidBitmap_unlockPixels (env: *mut JNIEnv, bitmap: jobject) -> c_int;
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub mod android_bitmap {
  /// C: bitmap.h 99-118: `typedef struct { uint32_t width; uint32_t height; uint32_t stride;
  /// int32_t format; uint32_t flags; } AndroidBitmapInfo;`
  #[repr(C)]
  #[derive(Clone, Copy)]
  pub struct AndroidBitmapInfo
  {
    pub width: u32, pub height: u32, pub stride: u32, pub format: i32, pub flags: u32,
  }
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))] type AndroidBitmapInfo_tag = AndroidBitmapInfo;

/* Nine declarations the headers above reach which the frozen mirrors do not carry yet (P-72 rule 4),
   each at its use site with the reference header's line (P-55): C 720's `strstr` (`string.rs` has
   `strcmp`/`memcmp`/`memset` but not it, and the frozen `<string.h>` does not rename it - `nm -u` on a
   probe that calls it prints `_strstr`), C 120's `posix_memalign` (`stdlib.rs` is the system header's
   mirror and lacks it, the `android.rs` 1367 shape), C 81/121's `memory_full`, C 747's
   `register_font_driver`, C 439's `prepare_face_for_display`, C 485's `gui_union_rectangles` (a
   use-site copy: it would be a new `docs/baseline/mirror-dupes.tsv` row) and C 670-672's three
   `hbfont_*` entry points.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `<string.h>`: `char *strstr (const char *, const char *);`
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
  /* C's `<stdlib.h>` declares `posix_memalign`; the `<stdlib.h>` mirror owns that declaration
     now (`lib/include/stdlib.rs`, added for `alloc.c` 4.713), so this unit's copy went away - two
     declarations of one name in one unit is E0428 (measured in the android sweep, status 4.725).  */
  /// C: lisp.h 4432: `extern AVOID memory_full (size_t);` (alloc.c defines it).
  pub fn memory_full (size: size_t) -> !;
  /// C: font.h 896: `extern void register_font_driver (struct font_driver const *,
  /// struct frame *);` (C 747).
  pub fn register_font_driver (driver: *const font_driver, f: *mut frame);
  /// C: dispextern.h 3809-3810: `#ifdef HAVE_WINDOW_SYSTEM void prepare_face_for_display
  /// (struct frame *, struct face *);` - `struct face` is the mirror's untyped pointer (P-72 3).
  pub fn prepare_face_for_display (f: *mut frame, face: *mut c_void);
  /// C: dispextern.h 3679: `extern void gui_union_rectangles (const Emacs_Rectangle *,
  /// const Emacs_Rectangle *, Emacs_Rectangle *);` (C 485).
  pub fn gui_union_rectangles (r1: *const Emacs_Rectangle, r2: *const Emacs_Rectangle,
                               out: *mut Emacs_Rectangle);
  /// C: font.h 917-919: the three HarfBuzz entry points `src/hbfont.rs` defines (C 670-672).
  pub fn hbfont_otf_capability (font: *mut font) -> Lisp_Object;
  pub fn hbfont_shape (lgstring: Lisp_Object, direction: Lisp_Object) -> Lisp_Object;
  pub fn hbfont_combining_capability (font: *mut font) -> Lisp_Object;
}

/* `struct dirent`'s record: C 21's `<dirent.h>` reaches it through the frozen
   `build/emacs-ref/lib/dirent.h`, and `lib/include/dirent.in.rs`/`dirent.rs` carry the calls and
   the opaque `DIR` but not the record (their own note says so) - the `asset-directory-tool.rs`
   44-60 shape.  Measured with the configuration's clang: `sizeof` 1048, `d_ino`@0, `d_seekoff`@8,
   `d_reclen`@16, `d_namlen`@18, `d_type`@20, `d_name`@21.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct dirent
{
  pub d_ino: u64, pub d_seekoff: u64, pub d_reclen: u16, pub d_namlen: u16, pub d_type: u8,
  pub d_name: [c_char; 1024],
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::size_of::<dirent> () == 1048);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (dirent, d_name) == 21);

/* `android.h` 340-342's `ANDROID_DELETE_LOCAL_REF` expands to a `<jni.h>` vtable read.  The mirror
   records it as an excluded macro whose premise ("`JNINativeInterface` has no mirror") predates
   `include/jni.rs`, so the port defines the expansion under C's own name (P-86 11, the
   `androidfont.rs` 136-139 shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! ANDROID_DELETE_LOCAL_REF {
  ($r:expr) => { ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, $r) };
}

/* C's `<stdckdint.h>` `ckd_mul` as C 81/115 use it: on the frozen target the name is clang's
   `__builtin_mul_overflow` (P-15), whose port counterpart is the `overflowing_*` pair with a
   write-back (the `gtkutil.rs` 1055 body).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! ckd_mul {
  ($dst:ident, $a:expr, $b:expr) => {{
    let (ckd_mul_value, ckd_mul_overflow) = ($a).overflowing_mul ($b);
    $dst = ckd_mul_value;
    ckd_mul_overflow
  }};
}

/* Structure describing a temporary buffer.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct sfntfont_android_scanline_buffer            // C 41-48
{
  /* Size of this buffer.  */
  pub buffer_size: size_t,

  /* Pointer to the buffer data.  */
  pub buffer_data: *mut c_void,
}

/* Array of directories to search for system fonts.  C 56's `(char[PATH_MAX]) { }` is a compound
   literal of array type with static storage duration, so the port writes the buffer and the array
   that points at it (`&raw mut` is C's array decay).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut system_font_directories_buf: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut system_font_directories: [*mut c_char; 3] =      // C 51-57
  [
    c"/system/fonts".as_ptr () as *mut c_char,
    c"/product/fonts".as_ptr () as *mut c_char,
    /* This should be filled in by init_sfntfont_android.  */
    &raw mut system_font_directories_buf as *mut c_char,
  ];

/* The font cache.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut font_cache: Lisp_Object = lisp_h_Qnil ();   // C 60 (tentative, i.e. Qnil)
/* The scanline buffer.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut scanline_buffer: sfntfont_android_scanline_buffer                       // C 63
  = sfntfont_android_scanline_buffer { buffer_size: 0, buffer_data: std::ptr::null_mut () };
/* The largest size of the scanline buffer since the last window
   update.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))] static mut max_scanline_buffer_size: size_t = 0;   // C 67

/* C 71-138: `GET_SCANLINE_BUFFER`, one definition per arm (P-86 1).  C 74's arm takes the block from
   `alloca` or the persistent buffer, C 107's from `posix_memalign`'d storage (16-byte alignment for
   the vector path).  The port has no `alloca` (P-05), so the 74 arm's block comes from the stack pool
   `sa_stack_alloc` serves (`msdos.rs` 1747-1749's shape).  */

/* Return a temporary buffer for storing scan lines.
   Set BUFFER to the buffer upon success.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify", not (feature = "aarch64")))]
macro_rules! GET_SCANLINE_BUFFER {                      // C 76-105
  ($buffer:ident, $height:expr, $stride:expr) => {{
    let mut _size: size_t = 0;

    if ckd_mul! (_size, $height, $stride)
      { memory_full_up (); }

    if _size < MAX_ALLOCA as size_t
      { $buffer = sa_stack_alloc (_size) as *mut c_void; }
    else
      {
	if _size > scanline_buffer.buffer_size
	  {
	    scanline_buffer.buffer_data = xrealloc (scanline_buffer.buffer_data, _size);
	    $buffer = scanline_buffer.buffer_data;
	    scanline_buffer.buffer_size = _size;
	  }
	else if _size <= scanline_buffer.buffer_size
	  { $buffer = scanline_buffer.buffer_data; }
	/* This is unreachable but clang says it isn't.  */
	else
	  { emacs_abort (); }

	max_scanline_buffer_size = max! (_size, max_scanline_buffer_size);
      }
  }};
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "aarch64"))]
macro_rules! GET_SCANLINE_BUFFER {                      // C 109-136
  ($buffer:ident, $height:expr, $stride:expr) => {{
    let mut _size: size_t = 0;
    let mut _temp: *mut c_void = std::ptr::null_mut ();

    if ckd_mul! (_size, $height, $stride)
      { memory_full_up (); }

    if _size > scanline_buffer.buffer_size
      {
	if posix_memalign (&raw mut _temp, 16, _size) != 0
	  { memory_full (_size); }
	free (scanline_buffer.buffer_data);
	scanline_buffer.buffer_data = _temp;
	$buffer = scanline_buffer.buffer_data;
	scanline_buffer.buffer_size = _size;
      }
    else if _size <= scanline_buffer.buffer_size
      { $buffer = scanline_buffer.buffer_data; }
    /* This is unreachable but clang says it isn't.  */
    else
      { emacs_abort (); }

    max_scanline_buffer_size = max! (_size, max_scanline_buffer_size);
  }};
}

/* Scale each of the four packed bytes in P in the low 16 bits of P by
   SCALE.  Return the result.

   SCALE is an integer between 0 and 256.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_android_scale32 (scale: c_uint, p: c_uint) -> c_uint  // C 147-160
{
  let mut ag: u32; let mut rb: u32; let mut scaled_ag: u32; let mut scaled_rb: u32;

  ag = (p & 0xFF00FF00) >> 8;
  rb = (p & 0x00FF00FF);

  scaled_ag = (scale * ag) & 0xFF00FF00;
  scaled_rb = (scale * rb) >> 8 & 0x00FF00FF;

  return scaled_ag | scaled_rb;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_android_mul8x2 (a8: c_uint, b32: c_uint) -> c_uint    // C 162-171
{
  let mut i: c_uint;
  let mut b32 = b32;

  b32 &= 0xff00ff;
  i = a8 * b32 + 0x800080;

  return (i + ((i >> 8) & 0xff00ff)) >> 8 & 0xff00ff;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! U255TO256 {                                // C 173
  ($x:expr) => { (($x) as u16 + (($x) as u16 >> 7)) };
}

/* Blend two pixels SRC and DST without utilizing any control flow.
   Both SRC and DST are expected to be in premultiplied ABGB8888
   format.  Value is returned in premultiplied ARGB8888 format.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_android_blend (src: c_uint, dst: c_uint) -> c_uint    // C 179-193
{
  let mut a: c_uint; let mut br_part: c_uint; let mut ag_part: c_uint; let mut both: c_uint;

  a = (src >> 24);
  br_part = sfntfont_android_mul8x2 (255 - a, dst);
  ag_part = sfntfont_android_mul8x2 (255 - a, dst >> 8) << 8;

  both = ag_part | br_part;

  /* This addition need not be saturating because both has already
     been multiplied by 255 - a.  */
  return both + src;
}

/* C 197-307: the NEON path.  `core::arch::aarch64` carries the ACLE names C's `<arm_neon.h>` defines,
   so the calls keep C's names; the immediate forms are Rust's const-generic spelling and C's
   `.val[i]` is the tuple's `.i` (one `macro-body-notes.tsv` row).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "aarch64"))]
use core::arch::aarch64::*;

/* Like U255TO256, but operates on vectors.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "aarch64"))]
unsafe fn sfntfont_android_u255to256 (r#in: uint8x8_t) -> uint16x8_t     // C 199-203
{
  return vaddl_u8 (vshr_n_u8::<7> (r#in), r#in);
}

/* Use processor features to efficiently composite four pixels at SRC
   to DST.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "aarch64"))]
unsafe fn sfntfont_android_over_8888_1 (src: *mut c_uint, dst: *mut c_uint)   // C 208-268
{
  let mut alpha: uint8x8_t; let mut alpha_c16: uint16x8_t; let mut v1: uint16x8_t;
  let mut v3: uint16x8_t; let v4: uint16x8_t; let mut b: uint8x8_t; let mut g: uint8x8_t;
  let mut r: uint8x8_t; let mut a: uint8x8_t; let mut v2: uint8x8_t; let v5: uint8x8_t;
  let mut _src: uint8x8x4_t; let mut _dst: uint8x8x4_t;

  /* Pull in src and dst.

     This loads bytes, not words, so little endian ABGR becomes
     RGBA.  */
  _src = vld4_u8 (src as *const u8);
  _dst = vld4_u8 (dst as *const u8);

  /* Load constants.  */
  v4 = vdupq_n_u16 (256);
  v5 = vdup_n_u8 (0);

  /* Load src alpha.  */
  alpha = _src.3;

  /* alpha_c16 = 256 - 255TO256 (alpha).  */
  alpha_c16 = sfntfont_android_u255to256 (alpha);
  alpha_c16 = vsubq_u16 (v4, alpha_c16);

  /* Cout = Csrc + Cdst * alpha_c.  */
  v1 = vaddl_u8 (_dst.2, v5);
  v2 = _src.2;
  v3 = vmulq_u16 (v1, alpha_c16);
  b = vqadd_u8 (v2, vshrn_n_u16::<8> (v3));

  v1 = vaddl_u8 (_dst.1, v5);
  v2 = _src.1;
  v3 = vmulq_u16 (v1, alpha_c16);
  g = vqadd_u8 (v2, vshrn_n_u16::<8> (v3));

  v1 = vaddl_u8 (_dst.0, v5);
  v2 = _src.0;
  v3 = vmulq_u16 (v1, alpha_c16);
  r = vqadd_u8 (v2, vshrn_n_u16::<8> (v3));

  /* C 250-260: `#if 0` / `#else` / `#endif` - dead in every configuration, so C's arm is kept as
     this comment (C's text: `Aout = Asrc + Adst * alpha_c.`; `v1 = vaddl_u8 (_dst.val[3], v5);`
     `v2 = _src.val[3];` `v3 = vmulq_u16 (v1, alpha_c16);`
     `a = vqadd_u8 (v2, vshrn_n_u16 (v3, 8));`).  */
  /* We know that Adst is always 1, so Asrc + Adst * (1 - Asrc) is
     always 1.  */
  a = vdup_n_u8 (255);

  /* Store back in dst.  */
  _dst.0 = r;
  _dst.1 = g;
  _dst.2 = b;
  _dst.3 = a;
  vst4_u8 (dst as *mut u8, _dst);
}

/* Use processor features to efficiently composite the buffer at SRC
   to DST.  Composite at most MAX - SRC words.

   If either SRC or DST are not yet properly aligned, value is 1.
   Otherwise, value is 0, and *X is incremented to the start of any
   trailing data which could not be composited due to data alignment
   constraints.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "aarch64"))]
unsafe fn sfntfont_android_over_8888 (src: *mut c_uint, dst: *mut c_uint, max: *mut c_uint,
				      x: *mut c_uint) -> c_int             // C 278-307
{
  let mut i: size_t; let mut how_much: isize; let mut s: *mut c_void; let mut d: *mut c_void;

  /* Figure out how much can be composited by this loop.  */
  how_much = (max.offset_from (src)) & !7;

  /* Return if there is not enough to vectorize.  */
  if how_much == 0
    { return 1; }

  /* Now increment *X by that much so the containing loop can process
     the remaining pixels one-by-one.  */

  *x += how_much as c_uint;

  for i in (0..how_much as usize).step_by (8)
    {
      s = src.wrapping_offset (i as isize) as *mut c_void;
      d = dst.wrapping_offset (i as isize) as *mut c_void;

      sfntfont_android_over_8888_1 (s as *mut c_uint, d as *mut c_uint);
    }

  return 0;
}

/* Composite the bitmap described by BUFFER, STRIDE and TEXT_RECTANGLE
   onto the native-endian ABGR8888 bitmap described by DEST and
   BITMAP_INFO.  RECT is the subset of the bitmap to composite.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_android_composite_bitmap (buffer: *mut c_uchar, stride: size_t,
					     dest: *mut c_uchar,
					     bitmap_info: *mut android_bitmap::AndroidBitmapInfo,
					     text_rectangle: *mut android_rectangle,
					     rect: *mut android_rectangle)   // C 315-382
{
  let mut src_row: *mut c_uint; let mut dst_row: *mut c_uint; let mut i: c_uint;
  let mut src_y: c_uint; let mut x: c_uint; let mut src_x: c_uint; let mut max_x: c_uint;
  let mut dst_x: c_uint;
  #[cfg (feature = "aarch64")] let mut lim_x: c_uint;

  if (dest as isize) & 3 != 0 || ((*bitmap_info).stride & 3) != 0
    /* This shouldn't be possible as Android is supposed to align the
       bitmap to at least a 4 byte boundary.  */
    { emacs_abort (); }
  else
    {
      /* C's `buffer + src_y * stride` is arithmetic in `unsigned int` units (`buffer`'s element)
	 and the port keeps that scale; C 522/551's two `row` computations cast to
	 `unsigned char *` first and keep that one.  */
      for i in 0..(*rect).height
	{
	  if i + (*rect).y as c_uint >= (*bitmap_info).height as c_uint
	    /* Done.  */
	    { return; }

	  src_y = i + ((*rect).y - (*text_rectangle).y) as c_uint;

	  if src_y > (*text_rectangle).height
	    /* Huh? */
	    { return; }

	  src_row = (buffer as *mut c_uint).wrapping_offset ((src_y as usize * stride) as isize);
	  dst_row = dest.wrapping_offset (((i + (*rect).y as c_uint) as usize
					   * (*bitmap_info).stride as usize) as isize)
	    as *mut c_uint;

	  /* Figure out where the loop below should end.  */
	  max_x = min! ((*rect).width, (*bitmap_info).width - (*rect).x as c_uint);

	  /* Keep this loop simple! */
	  x = 0;
	  while x < max_x
	    {
	      src_x = x + ((*rect).x - (*text_rectangle).x) as c_uint;
	      dst_x = x + (*rect).x as c_uint;

	      #[cfg (feature = "aarch64")]
	      {
		/* This is the largest value of src_x.  */
		lim_x = max_x + ((*rect).x - (*text_rectangle).x) as c_uint;

		if sfntfont_android_over_8888 (src_row.wrapping_offset (src_x as isize),
					       dst_row.wrapping_offset (dst_x as isize),
					       src_row.wrapping_offset (lim_x as isize),
					       &raw mut x) == 0
		  {
		    /* Decrement X by one so the for loop can increment
		       it again.  */
		    x = x.wrapping_sub (1);
		    x = x.wrapping_add (1);   /* C's `++x` of the `for` */
		    continue;
		  }
	      }
	      *dst_row.wrapping_offset (dst_x as isize)
		= sfntfont_android_blend (*src_row.wrapping_offset (src_x as isize),
					  *dst_row.wrapping_offset (dst_x as isize));

	      x += 1;
	    }
	}
    }
}

/* Calculate the union containing both A and B, both boxes.  Place the
   result in RESULT.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_android_union_boxes (a: gui_box, b: gui_box, result: *mut gui_box)  // C 387-395
{
  (*result).x1 = min! (a.x1, b.x1);
  (*result).y1 = min! (a.y1, b.y1);
  (*result).x2 = max! (a.x2, b.x2);
  (*result).y2 = max! (a.y2, b.y2);
}

/* Draw the specified glyph rasters from FROM to TO on behalf of S,
   using S->gc.  Fill the background if WITH_BACKGROUND is true.

   See init_sfntfont_vendor and sfntfont_draw for more details.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfntfont_android_put_glyphs (s: *mut c_void, from: c_int, to: c_int,
						  x: c_int, y: c_int, with_background: bool,
						  rasters: *mut *mut sfnt_raster,
						  x_coords: *mut c_int)                   // C 402-610
{
  /* C's first parameter is `struct glyph_string *s`, spelled untyped by `sfntfont.h`'s
     `sfntfont_put_glyph_proc` (what C 746 passes) and converted here (P-72 rule 3).  C 408-418's
     locals follow, zero-initialised where C leaves them to `memset`/callees (PORTING.md 1.5);
     `buffer` is C's `unsigned int *` whose arms assign a `void *` (P-72 rule 3).  */
  let s: *mut glyph_string = s as *mut glyph_string;
  let mut background: android_rectangle = std::mem::zeroed ();
  let mut text_rectangle: android_rectangle = std::mem::zeroed ();
  let mut rect: android_rectangle = std::mem::zeroed ();
  let mut text: gui_box = std::mem::zeroed (); let mut character: gui_box = std::mem::zeroed ();
  let mut buffer: *mut c_void; let mut row: *mut c_uint;
  let mut raster_row: *mut c_uchar;      /* C: `unsigned char *restrict raster_row;` */
  let mut stride: size_t = 0; let mut i: size_t;
  let mut bitmap_info: android_bitmap::AndroidBitmapInfo = std::mem::zeroed ();
  let mut bitmap_data: *mut c_uchar; let mut bitmap: jobject = std::ptr::null_mut ();
  let mut left: c_int; let mut top: c_int; let mut temp_y: c_int;
  let mut prod: c_uint; let mut raster_y: c_uint;
  let mut foreground: c_ulong; let mut back_pixel: c_ulong; let mut rb: c_ulong;

  if (*((*s).gc as *mut android_gc)).num_clip_rects == 0
    /* Clip region is empty.  */
    { return; }

  if from == to
    /* Nothing to draw.  */
    { return; }

  /* Swizzle the foreground and background in s->gc into BGR, then add
     an alpha channel.  */
  foreground = (*((*s).gc as *mut android_gc)).foreground;
  back_pixel = (*((*s).gc as *mut android_gc)).background;
  rb = foreground & 0x00ff00ff;
  foreground &= !0x00ff00ff;
  foreground |= rb >> 16 | rb << 16 | 0xff000000;
  rb = back_pixel & 0x00ff00ff;
  back_pixel &= !0x00ff00ff;
  back_pixel |= rb >> 16 | rb << 16 | 0xff000000;

  prepare_face_for_display ((*s).f, (*s).face);

  /* Build the scanline buffer.  Figure out the bounds of the
     background.  */
  memset ((&raw mut background) as *mut c_void, 0, std::mem::size_of::<android_rectangle> ());

  if with_background
    {
      background.x = x;
      background.y = y - FONT_BASE! ((*s).font);
      background.width = (*s).width as c_uint;
      background.height = FONT_HEIGHT! ((*s).font) as c_uint;
    }

  /* Now figure out the bounds of the text.  */

  if ! (*rasters).is_null ()
    {
      text.x1 = *x_coords.offset (0) + (**rasters).offx as c_int;
      text.x2 = text.x1 + (**rasters).width as c_int;
      text.y1 = y - (**rasters).height as c_int - (**rasters).offy as c_int;
      text.y2 = y - (**rasters).offy as c_int;
    }
  else
    { memset ((&raw mut text) as *mut c_void, 0, std::mem::size_of::<gui_box> ()); }

  for i in 1..(to - from) as usize
    {
      /* See if text has to be extended.  */

      if (*rasters.offset (i as isize)).is_null ()
	{ continue; }

      character.x1 = *x_coords.offset (i as isize)
	+ (**rasters.offset (i as isize)).offx as c_int;
      character.x2 = character.x1 + (**rasters.offset (i as isize)).width as c_int;
      character.y1 = y - (**rasters.offset (i as isize)).height as c_int
	- (**rasters.offset (i as isize)).offy as c_int;
      character.y2 = y - (**rasters.offset (i as isize)).offy as c_int;

      sfntfont_android_union_boxes (text, character, &raw mut text);
    }

  /* Union the background rect with the text rectangle.  */
  text_rectangle.x = text.x1;
  text_rectangle.y = text.y1;
  text_rectangle.width = (text.x2 - text.x1) as c_uint;
  text_rectangle.height = (text.y2 - text.y1) as c_uint;
  /* C's first and third pointers are `struct android_rectangle *` where the callee wants
     `Emacs_Rectangle *`; the two records are the same four members and the port spells the
     conversion (P-72 rule 3).  */
  gui_union_rectangles ((&raw const background) as *const Emacs_Rectangle,
			(&raw const text_rectangle) as *const Emacs_Rectangle,
			(&raw mut text_rectangle) as *mut Emacs_Rectangle);

  /* Allocate enough to hold text_rectangle.height, aligned to 8 (or
     16) bytes.  Then fill it with the background.  */
  #[cfg (not (feature = "aarch64"))]
  { stride = ((text_rectangle.width as usize * std::mem::size_of::<c_uint> ()) + 7) & !7; }
  #[cfg (feature = "aarch64")]
  { stride = ((text_rectangle.width as usize * std::mem::size_of::<c_uint> ()) + 15) & !15; }
  GET_SCANLINE_BUFFER! (buffer, text_rectangle.height as usize, stride);

  /* Try to optimize out this memset if the background rectangle
     contains the whole text rectangle.  */

  if ! with_background
    || memcmp ((&raw const background) as *const c_void,
	       (&raw const text_rectangle) as *const c_void,
	       std::mem::size_of::<android_rectangle> ()) != 0
    { memset (buffer, 0, text_rectangle.height as usize * stride); }

  if with_background
    {
      /* The background should have been filled in advance if a stipple
	 is enabled.  */
      eassert! ((*((*s).gc as *mut android_gc)).fill_style != ANDROID_FILL_OPAQUE_STIPPLED);

      /* Fill the background.  First, offset the background rectangle
	 to become relative from text_rectangle.x,
	 text_rectangle.y.  */
      background.x = background.x - text_rectangle.x;
      background.y = background.y - text_rectangle.y;
      eassert! (background.x >= 0 && background.y >= 0);

      for temp_y in background.y as c_uint..(background.y as c_uint + background.height)
	{
	  row = (buffer as *mut c_uchar)
	    .wrapping_offset ((stride * temp_y as usize) as isize) as *mut c_uint;

	  for x in background.x as c_uint..background.x as c_uint + background.width
	    { *row.wrapping_offset (x as isize) = back_pixel as c_uint; }
	}
    }

  /* Draw all the rasters onto the buffer.  */
  for i in 0..(to - from) as usize
    {
      if (*rasters.offset (i as isize)).is_null ()
	{ continue; }

      /* Figure out the top and left of the raster relative to
	 text_rectangle.  */
      left = *x_coords.offset (i as isize) + (**rasters.offset (i as isize)).offx as c_int
	- text_rectangle.x;

      /* Note that negative offy represents the part of the text that
	 lies below the baseline.  */
      top = y - ((**rasters.offset (i as isize)).height as c_int
		 + (**rasters.offset (i as isize)).offy as c_int)
	- text_rectangle.y;
      eassert! (left >= 0 && top >= 0);

      /* Draw the raster onto the temporary bitmap using the
	 foreground color scaled by the alpha map.  */

      for raster_y in 0..(**rasters.offset (i as isize)).height as c_uint
	{
	  row = (buffer as *mut c_uchar)
	    .wrapping_offset ((stride * (raster_y as usize + top as usize)) as isize)
	    as *mut c_uint;
	  raster_row = (**rasters.offset (i as isize)).cells
	    .wrapping_offset (((raster_y as usize)
			       * (**rasters.offset (i as isize)).stride as usize) as isize);

	  for x in 0..(**rasters.offset (i as isize)).width as c_uint
	    {
	      prod = sfntfont_android_scale32 (U255TO256! (*raster_row.offset (x as isize))
					       as c_uint, foreground as c_uint);
	      *row.wrapping_offset ((left + x as c_int) as isize)
		= sfntfont_android_blend (prod,
					  *row.wrapping_offset ((left + x as c_int) as isize));
	    }
	}
    }

  /* Lock the bitmap.  It must be unlocked later.  */
  bitmap_data = android_lock_bitmap (FRAME_ANDROID_DRAWABLE! ((*s).f),
				     (&raw mut bitmap_info) as *mut AndroidBitmapInfo_tag,
				     &raw mut bitmap);

  /* If locking the bitmap fails, just discard the data that was
     allocated.  */
  if bitmap_data.is_null ()
    { return; }

  /* Loop over each clip rect in the GC.  */
  eassert! (bitmap_info.format == ANDROID_BITMAP_FORMAT_RGBA_8888);

  if (*((*s).gc as *mut android_gc)).num_clip_rects > 0
    {
      for i in 0..(*((*s).gc as *mut android_gc)).num_clip_rects as usize
	{
	  if ! gui_intersect_rectangles
	       ((*((*s).gc as *mut android_gc)).clip_rects.wrapping_offset (i as isize)
		as *const Emacs_Rectangle,
		(&raw const text_rectangle) as *const Emacs_Rectangle,
		(&raw mut rect) as *mut Emacs_Rectangle)
	    /* Outside the clip region.  */
	    { continue; }

	  /* Composite the intersection onto the buffer.  */
	  sfntfont_android_composite_bitmap (buffer as *mut c_uchar, stride, bitmap_data,
					     &raw mut bitmap_info,
					     &raw mut text_rectangle, &raw mut rect);
	}
    }
  else /* gc->num_clip_rects < 0 */
    { sfntfont_android_composite_bitmap (buffer as *mut c_uchar, stride, bitmap_data,
					 &raw mut bitmap_info,
					 &raw mut text_rectangle,
					 &raw mut text_rectangle); }

  /* Release the bitmap.  */
  AndroidBitmap_unlockPixels (android_java_env, bitmap);
  ANDROID_DELETE_LOCAL_REF! (bitmap);

  /* Damage the window by the text rectangle.  */
  android_damage_window (FRAME_ANDROID_DRAWABLE! ((*s).f),
			 (&raw mut text_rectangle) as *mut android_rectangle);

  /* C 609: `#undef MAX_ALLOCA` - an *enum* constant (lisp.h 5616, the mirror's
     `pub const MAX_ALLOCA: isize`), so C's `#undef` is a no-op there too (one `## Deviations`).  */
}

/* Shrink the scanline buffer after a window update.  If
   max_scanline_buffer_size is not zero, and is less than
   scanline_buffer.buffer_size / 2, then resize the scanline buffer to
   max_scanline_buffer_size.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn sfntfont_android_shrink_scanline_buffer ()                       // C 619-636
{
  if max_scanline_buffer_size == 0
    { return; }

  if max_scanline_buffer_size < scanline_buffer.buffer_size / 2
    {
      scanline_buffer.buffer_size = max_scanline_buffer_size;
      scanline_buffer.buffer_data = xrealloc (scanline_buffer.buffer_data,
					      max_scanline_buffer_size);
    }

  max_scanline_buffer_size = 0;
}

/* Font driver definition.  */

/* Return the font cache for this font driver.  F is ignored.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfntfont_android_get_cache (f: *mut c_void) -> Lisp_Object   // C 644-648
{
  return font_cache;
}

/* `font.h`'s fields take untyped pointers where the mirror's `sfntfont_*` prototypes spell
   `*mut frame` / `*mut glyph_string` / `*mut font_metrics`, and Rust has no `as` cast between
   function pointers of different signatures; the six hooks are therefore declared a second time with
   the fields' own types - a module-local copy, which `tools/check-mirror-dupes.py` does not read as a
   duplicate - and the record takes them qualified (P-72 rule 3, the `xftfont.rs` 923-931 shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub mod font_driver_hooks {
  use super::*;

  unsafe extern "C" {
    /// C: sfntfont.h 33: `extern Lisp_Object sfntfont_list (struct frame *, Lisp_Object);`
    pub fn sfntfont_list (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object;
    /// C: sfntfont.h 34: `extern Lisp_Object sfntfont_match (struct frame *, Lisp_Object);`
    pub fn sfntfont_match (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object;
    /// C: sfntfont.h 41-42: `extern int sfntfont_draw (struct glyph_string *, int, int, int,
    /// int, bool);`
    pub fn sfntfont_draw (s: *mut glyph_string, from: c_int, to: c_int, x: c_int, y: c_int,
                          with_background: bool) -> c_int;
    /// C: sfntfont.h 35: `extern Lisp_Object sfntfont_open (struct frame *, Lisp_Object, int);`
    pub fn sfntfont_open (f: *mut c_void, font_entity: Lisp_Object, pixel_size: c_int)
                          -> Lisp_Object;
    /// C: sfntfont.h 38-39: `extern void sfntfont_text_extents (struct font *,
    /// const unsigned int *, int, struct font_metrics *);`
    pub fn sfntfont_text_extents (font: *mut font, code: *const c_uint, nchars: c_int,
                                  metrics: *mut font_metrics);
    /// C: sfntfont.h 43: `extern Lisp_Object sfntfont_list_family (struct frame *);`
    pub fn sfntfont_list_family (f: *mut c_void) -> Lisp_Object;
  }
}

/* The Android sfntfont driver.  Its six pointers are the module copies above.  The HarfBuzz members
   C 666-673 sets exist in the record in both configurations, so each is written under C's own
   condition (the `xftfont.rs` 1855-1875 shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_sfntfont_driver: font_driver = font_driver {            // C 651-674
  /* C 653: `.type = LISPSYM_INITIALLY (Qsfnt_android)`.  The mirror's `LISPSYM_INITIALLY` is not
     a `const fn`, so the port writes the same constant expression over the generated
     `iQsfnt_android` index (the `androidfont.rs` 1289 precedent).  */
  r#type: (((iQsfnt_android * std::mem::size_of::<Lisp_Symbol> ()) as EMACS_UINT
	    + Lisp_Type::Lisp_Symbol as EMACS_UINT) as Lisp_Object),
  case_sensitive: true,                                                     // C 654
  get_cache: Some (sfntfont_android_get_cache),                             // C 655
  list: Some (font_driver_hooks::sfntfont_list),                         // C 656
  r#match: Some (font_driver_hooks::sfntfont_match),                     // C 657
  draw: Some (font_driver_hooks::sfntfont_draw),                         // C 658
  open_font: Some (font_driver_hooks::sfntfont_open),                    // C 659
  close_font: Some (sfntfont_close),                                        // C 660
  encode_char: Some (sfntfont_encode_char),                                 // C 661
  text_extents: Some (font_driver_hooks::sfntfont_text_extents),         // C 662
  list_family: Some (font_driver_hooks::sfntfont_list_family),           // C 663
  get_variation_glyphs: Some (sfntfont_get_variation_glyphs),               // C 664
  /* C's designated initialiser leaves the other members implicit (zero); Rust has no designated
     initialisers, so they are written out.  `draw` above and `get_bitmap`/`free_bitmap` carry the
     mirror's `have_window_system` guard (font.h 676-698).  The HarfBuzz pair C 666-673 sets is
     font.h 807-820 (`begin_hb_font` and `end_hb_font`).  */
  free_entity: None, prepare_face: None, done_face: None, has_char: None, anchor_point: None,
  #[cfg (feature = "have_window_system")] get_bitmap: None,
  #[cfg (feature = "have_window_system")] free_bitmap: None,
  otf_drive: None, start_for_frame: None, end_for_frame: None, check: None,
  filter_properties: None, cached_font_ok: None, drop_xrender_surfaces: None,
  #[cfg (feature = "harfbuzz")] begin_hb_font: Some (sfntfont_begin_hb_font),   // C 669
  #[cfg (feature = "harfbuzz")] end_hb_font: None,
  #[cfg (feature = "harfbuzz")] combining_capability: Some (hbfont_combining_capability),   // C 670
  #[cfg (not (feature = "harfbuzz"))] combining_capability: None,
  #[cfg (feature = "harfbuzz")] shape: Some (hbfont_shape),                      // C 671
  #[cfg (not (feature = "harfbuzz"))] shape: None,
  #[cfg (feature = "harfbuzz")] otf_capability: Some (hbfont_otf_capability),    // C 672
  #[cfg (not (feature = "harfbuzz"))] otf_capability: None,
};

/* This is an ugly hack that should go away, but I can't think of
   how.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
DEFUN! ("android-enumerate-fonts", Fandroid_enumerate_fonts,
	Sandroid_enumerate_fonts, 0, 0, 0,
	doc: "Enumerate fonts present on the system.\n\nSignal an error if fonts have already been enumerated.  This would\nnormally have been done in C, but reading fonts require Lisp to be\nloaded before character sets are made available.",   // C 683-687
	(),
	{
  let mut dir: *mut DIR; let mut i: c_int; let mut dirent: *mut dirent;
  /* C's `char name[PATH_MAX * 2];` is written by `sprintf` below, so the port zero-initialises it
     (PORTING.md 1.5).  */
  let mut name: [c_char; (PATH_MAX * 2) as usize] = [0; (PATH_MAX * 2) as usize];
  static mut enumerated: bool = false;

  if enumerated
    { error (c"Fonts have already been enumerated".as_ptr ()); }
  enumerated = true;

  block_input ();

  /* Scan through each of the system font directories.  Enumerate each
     font that looks like a TrueType font.  */
  for i in 0..countof! (system_font_directories) as c_int
    {
      dir = opendir (system_font_directories[i as usize]);

      let __func__: *const c_char = c"Fandroid_enumerate_fonts".as_ptr ();
      __android_log_print (ANDROID_LOG_VERBOSE, __func__, c"Loading fonts from: %s".as_ptr (),
			   system_font_directories[i as usize]);

      if dir.is_null ()
	{ continue; }

      loop
	{
	  dirent = readdir (dir) as *mut dirent;

	  if dirent.is_null ()
	    { break; }

	  /* If it contains (not ends with!) with .ttf or .ttc, then
	     enumerate it.  */

	  if (! strstr ((*dirent).d_name.as_ptr (), c".ttf".as_ptr ()).is_null ()
	      || ! strstr ((*dirent).d_name.as_ptr (), c".ttc".as_ptr ()).is_null ())
	      /* Ignore the non-variable Roboto font.  */
	      && (i != 0 || strcmp ((*dirent).d_name.as_ptr (),
				    c"RobotoStatic-Regular.ttf".as_ptr ()) != 0)
	    {
	      sprintf (name.as_mut_ptr (), c"%s/%s".as_ptr (),
		       system_font_directories[i as usize], (*dirent).d_name.as_ptr ());
	      sfnt_enum_font (name.as_mut_ptr ());
	    }
	}

      closedir (dir);
    }

  unblock_input ();

  return Qnil ();
});

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn syms_of_sfntfont_android_for_pdumper ()                  // C 743-748
{
  init_sfntfont_vendor (Qsfnt_android (), &raw mut android_sfntfont_driver as *const c_void,
			Some (sfntfont_android_put_glyphs));
  register_font_driver (&raw mut android_sfntfont_driver, std::ptr::null_mut ());
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn init_sfntfont_android ()                                        // C 751-806
{
  let mut api_level: c_int;

  if ! android_init_gui
    { return; }

  api_level = android_get_current_api_level! ();

  /* Make sure to pick the proper Sans Serif and Serif fonts for the
     version of Android the device is running.  */

  if api_level >= 21
    /* Android 5.0 and later distribute Noto Serif in lieu of Droid
       Serif.  */
    { Vsfnt_default_family_alist
	= list4 (Fcons (build_string (c"Monospace".as_ptr ()),
			build_string (c"Droid Sans Mono".as_ptr ())),
		 /* Android doesn't come with a Monospace Serif font, so
		    this will have to do.  */
		 Fcons (build_string (c"Monospace Serif".as_ptr ()),
			build_string (c"Droid Sans Mono".as_ptr ())),
		 Fcons (build_string (c"Sans Serif".as_ptr ()),
			build_string (c"Roboto".as_ptr ())),
		 Fcons (build_string (c"DejaVu Serif".as_ptr ()),
			build_string (c"Noto Serif".as_ptr ()))); }
  else if api_level >= 14
    /* Android 4.0 and later distribute Roboto in lieu of Droid
       Sans.  */
    { Vsfnt_default_family_alist
	= list4 (Fcons (build_string (c"Monospace".as_ptr ()),
			build_string (c"Droid Sans Mono".as_ptr ())),
		 /* Android doesn't come with a Monospace Serif font, so
		    this will have to do.  */
		 Fcons (build_string (c"Monospace Serif".as_ptr ()),
			build_string (c"Droid Sans Mono".as_ptr ())),
		 Fcons (build_string (c"Sans Serif".as_ptr ()),
			build_string (c"Roboto".as_ptr ())),
		 Fcons (build_string (c"DejaVu Serif".as_ptr ()),
			build_string (c"Droid Serif".as_ptr ()))); }
  else
    { Vsfnt_default_family_alist
	= list4 (Fcons (build_string (c"Monospace".as_ptr ()),
			build_string (c"Droid Sans Mono".as_ptr ())),
		 Fcons (build_string (c"Monospace Serif".as_ptr ()),
			build_string (c"Droid Sans Mono".as_ptr ())),
		 Fcons (build_string (c"Sans Serif".as_ptr ()),
			build_string (c"Droid Sans".as_ptr ())),
		 Fcons (build_string (c"DejaVu Serif".as_ptr ()),
			build_string (c"Droid Serif".as_ptr ()))); }

  /* Set up the user fonts directory.  This directory is ``fonts'' in
     the Emacs files directory.  */
  snprintf (system_font_directories[2], PATH_MAX as usize, c"%s/fonts".as_ptr (),
	    android_get_home_directory ());
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn syms_of_sfntfont_android ()                                     // C 809-821
{
  DEFSYM! (Qsfnt_android, "sfnt-android");
  DEFSYM! (Qandroid_enumerate_fonts, "android-enumerate-fonts");
  Fput (Qandroid (), Qfont_driver_superseded_by (), Qsfnt_android ());

  font_cache = list! (Qnil ());
  staticpro (&raw mut font_cache);

  defsubr (&raw mut Sandroid_enumerate_fonts);

  pdumper_do_now_and_after_load (Some (syms_of_sfntfont_android_for_pdumper));
}
