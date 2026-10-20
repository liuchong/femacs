// femacs: port of src/haikufont.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` and `extern "C"`
// (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn`.
//
// This file is one of P-86's class: the frozen configuration compiles no `haikufont.o`.  Its
// object variable is `HAIKU_OBJ`, which `configure.ac` 3097 clears and 3114 fills with the Haiku
// platform objects under `test "${HAVE_BE_APP}" = "yes"` - the same test that `AC_DEFINE`s
// `HAVE_HAIKU` (3108-3110) - and `build/emacs-ref/src/Makefile` reads `HAIKU_OBJ = ` (empty), so
// `docs/baseline/frozen-build-sources.txt` lists no `haikufont.o` and `extract-abi.py`'s checklist
// for the file is empty (P-86 ②).  `docs/ports/haikufont.md` therefore carries the substitute: the
// **C line-range table**, the `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "have_haiku"`, on every item *and on C 22-39's include
//     graph* (P-86 ⑧) - which is why the frozen configuration compiles nothing here;
//   * this file carries **one `#if` of its own** - C 1463/1465's `#ifdef USE_BE_CAIRO`, the
//     `be_cairo` feature `femacs/xtask/Cargo.toml` registers for it, with both arms (P-51/P-86 ①);
//     `tools/check-cond-eval.py src/haikufont.c` reads the file as `1 conditions, 0 unknown, 0
//     disagreements [VERIFIED]` and the condition takes its false arm on this target.
//
// **Acquisition note - this port is `text-only` (P-86 ③⑵).**  This machine has no Haiku SDK and
// no Haiku toolchain (the measurement is in `docs/ports/haikuterm.md` Evidence ②; it is the same
// machine).  So there is **no arm flip on a Haiku target**, and nothing here is compiled or linked
// against a Haiku system.  The `<InterfaceKit.h>`/`<SupportDefs.h>` surface this file's headers
// pull in is already mirrored by `include/haiku_support.rs`/`include/haikugui.rs` (both landed
// before this unit); every Haiku width, constant and enum value this port spells therefore carries
// the standing **UNVERIFIED** mark (P-88 ①/②), and the `BFont_*`/`be_*` prototypes it calls are
// read from `emacs/src/haiku_support.h`, which *is* part of the reference tree.
//
// `docs/ports/haikufont.md` carries the C line ranges, the acceptance readings, every deviation and
// the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 20: `#include <config.h>` (ungated)

/* C 21-36: the file's own include list.  Every header sits inside the file gate, as C's own build
   does - no translation unit the frozen configuration builds reaches any one of them.  */
#[cfg (feature = "have_haiku")]
include!("include/lisp.rs");                          // C 22: `#include "lisp.h"`
#[cfg (feature = "have_haiku")]
include!("include/dispextern.rs");                    // C 23: `#include "dispextern.h"`
#[cfg (feature = "have_haiku")]
include!("include/composite.rs");                     // C 24: `#include "composite.h"`
#[cfg (feature = "have_haiku")]
include!("include/blockinput.rs");                    // C 25: `#include "blockinput.h"`
#[cfg (feature = "have_haiku")]
include!("include/charset.rs");                       // C 26: `#include "charset.h"`
#[cfg (feature = "have_haiku")]
include!("include/frame.rs");                         // C 27: `#include "frame.h"`
#[cfg (feature = "have_haiku")]
include!("include/window.rs");                        // C 28: `#include "window.h"`
#[cfg (feature = "have_haiku")]
include!("include/fontset.rs");                       // C 29: `#include "fontset.h"`
#[cfg (feature = "have_haiku")]
include!("include/haikuterm.rs");                     // C 30: `#include "haikuterm.h"`
#[cfg (feature = "have_haiku")]
include!("include/character.rs");                     // C 31: `#include "character.h"`
#[cfg (feature = "have_haiku")]
include!("include/coding.rs");                        // C 32: `#include "coding.h"`
#[cfg (feature = "have_haiku")]
include!("include/font.rs");                          // C 33: `#include "font.h"`
#[cfg (feature = "have_haiku")]
include!("include/termchar.rs");                      // C 34: `#include "termchar.h"`
#[cfg (feature = "have_haiku")]
include!("include/pdumper.rs");                       // C 35: `#include "pdumper.h"`
/* `haiku_support.h` is at C 36, but `haikuterm.h` (C 30) already `#include`s it, so C reaches it
   through that header first and this entry is C's own second, guarded reach.  */
#[cfg (feature = "have_haiku")]
include!("include/haiku_support.rs");                 // C 36: `#include "haiku_support.h"`
#[cfg (feature = "have_haiku")]
include!("../lib/include/math.rs");                   // C 38: `#include <math.h>`
#[cfg (feature = "have_haiku")]
include!("../lib/include/stdlib.rs");                 // C 39: `#include <stdlib.h>`

/* ------------------------------------------------------------------------------------------
   C 38-39: `<math.h>`/`<stdlib.h>`'s standard names, and C 21-36's own declarations, that the
   frozen mirrors of this include graph do not carry yet (P-72 rule 4; each keeps C's prototype and
   the header line it was read from - the `src/haikuselect.rs` 165-190 / `src/androidfont.rs` 94-122
   shape).  `docs/ports/haikufont.md` records them for the integrator.
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /// C: string.h: `char *strncpy (char *dest, const char *src, size_t n);` (`include/string.rs`
  /// declares the other `<string.h>` names this file calls).
  pub fn strncpy (dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /// C: frame.h 1475-1476: `extern WINDOW_SYSTEM_RETURN struct frame * decode_window_system_frame
  /// (Lisp_Object);`
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
  /// C: dispextern.h 3781: `void request_sigio (void);`
  
  /// C: dispextern.h 3782: `void unrequest_sigio (void);`
  
  /// C: dispextern.h 3809-3810: `#ifdef HAVE_WINDOW_SYSTEM void prepare_face_for_display
  /// (struct frame *, struct face *);` - `struct face` is this unit's use-site record below, so
  /// the second parameter is the untyped pointer C's own call passes (P-72 (3)).
  pub fn prepare_face_for_display (f: *mut frame, face: *mut c_void);
  /// C: lisp.h 4786: `extern Lisp_Object quit (void);`
  pub fn quit () -> Lisp_Object;
  /// C: lisp.h 3880: `extern void process_pending_signals (void);`
  pub fn process_pending_signals ();
}

/* C 21-36: `font.h`'s own declarations this file calls that `include/font.rs` does not carry
   (`font_make_entity`, `font_make_object`, `register_font_driver`, `font_filter_properties`), and
   `font_style_to_value`/`font_style_symbolic`, which the mirror's `FONT_SET_STYLE`/`FONT_*` macro
   bodies name but no mirror declares (font.h 854/856; the `src/xfaces.rs` 526-529 precedent).  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /// C: font.h 842: `extern Lisp_Object font_make_entity (void);`
  pub fn font_make_entity () -> Lisp_Object;
  /// C: font.h 846: `extern Lisp_Object font_make_object (int, Lisp_Object, int);`
  pub fn font_make_object (size: c_int, entity: Lisp_Object, pixelsize: c_int) -> Lisp_Object;
  /// C: font.h 854: `extern int font_style_to_value (enum font_property_index prop, Lisp_Object
  /// name, bool noerror);`
  pub fn font_style_to_value (prop: font_property_index, name: Lisp_Object, noerror: bool)
    -> c_int;
  /// C: font.h 856-858: `extern Lisp_Object font_style_symbolic (Lisp_Object font, enum
  /// font_property_index prop, bool for_face);`
  pub fn font_style_symbolic (font: Lisp_Object, prop: font_property_index, for_face: bool)
    -> Lisp_Object;
  /// C: font.h 896: `extern void register_font_driver (struct font_driver const *,
  /// struct frame *);`
  pub fn register_font_driver (driver: *const font_driver, f: *mut frame);
  /// C: font.h 927-930: `extern void font_filter_properties (Lisp_Object font, Lisp_Object alist,
  /// const char *const boolean_properties[], const char *const non_boolean_properties[]);`
  pub fn font_filter_properties (font: Lisp_Object, alist: Lisp_Object,
                                 boolean_properties: *const *const c_char,
                                 non_boolean_properties: *const *const c_char);
}

/* `include/haiku_support.rs` 731 declares `be_lock_font_defaults` with no return type, while
   `haiku_support.h` 769 declares `extern bool be_lock_font_defaults (void);` - the mirror drops
   C's `bool`, and C 1341/1380 read that value (`if (!be_lock_font_defaults ())`).  A second
   crate-root item of the same name beside the mirror's would be an `E0428`, so this unit declares
   the same symbol with the reference's prototype inside a module and calls it from there (the
   `src/haikuselect.rs` 188-193 `block_input_abi` shape; `docs/ports/haikufont.md` `## Owed` names
   the mirror round that owes the return type).  */
#[cfg (feature = "have_haiku")]
mod haiku_support_abi {
  // C: haiku_support.h 769: `extern bool be_lock_font_defaults (void);`
  unsafe extern "C" {
    #[link_name = "be_lock_font_defaults"]
    pub fn be_lock_font_defaults () -> bool;
  }
}

/* `struct face` (dispextern.h 1787-1937) is this unit's use-site record (P-72 rule 3):
   `include/dispextern.rs`'s copy is behind `dispextern_face_records`, so a unit compiled on its
   own has no `face`.  C reads `foreground` (C 1123) and `background` (C 1124) through it and
   passes `s->face` itself on as an untyped argument (C 1127/1135), so those two members are
   pinned.  The numbers are `docs/baseline/face-layout.tsv`'s two measured window-system arms
   (`x11`/`ns_cocoa`/`x11_ext`: `sizeof` 296, align 8, `foreground`@184, `background`@192) - the
   same two numbers the frozen arm (280/8, `foreground`@168) gives after adding the sixteen bytes
   of C 1797-1811's `gc`/`stipple` pair, which is the only difference between the arms.  The
   Haiku arm itself is **UNVERIFIED** here (no Haiku SDK); `gc` is a pointer in every arm, so its
   size - the only thing this record's measurement depends on - is the same in all of them.  */
#[cfg (feature = "have_haiku")]
#[repr(C)]
pub struct face
{
  _prefix: [u8; 184],                        // dispextern.h 1791-1811 (`lface`, `id`, `gc`, `stipple`)
  pub foreground: c_ulong,                   // dispextern.h 1816: `unsigned long foreground;`
  pub background: c_ulong,                   // dispextern.h 1819: `unsigned long background;`
  _tail: [u8; 96],                           // dispextern.h 1822-1937 (clang: `sizeof` 296)
}
#[cfg (feature = "have_haiku")]
const _: () = {
  core::assert! (std::mem::size_of::<face> () == 296);
  core::assert! (std::mem::align_of::<face> () == 8);
  core::assert! (std::mem::offset_of! (face, foreground) == 184);
  core::assert! (std::mem::offset_of! (face, background) == 192);
};

// C 41: `static Lisp_Object font_cache;` - C's tentative definition zero-initialises, i.e. `Qnil`.
#[cfg (feature = "have_haiku")]
static mut font_cache: Lisp_Object = lisp_h_Qnil ();

/* C 43: `#define METRICS_NCOLS_PER_ROW	(128)`.  C's literal is an `int` and C 936/937 divide an
   `unsigned` by it, so the port keeps `c_int` and spells those two conversions where they occur.  */
#[cfg (feature = "have_haiku")]
const METRICS_NCOLS_PER_ROW: c_int = 128;

/* C 45-48: `enum metrics_status`.  */
/// C: `enum metrics_status { METRICS_INVALID = -1, };`
#[cfg (feature = "have_haiku")]
pub type metrics_status = c_int;
/// C 47: `METRICS_INVALID = -1,    /* metrics entry is invalid */`
#[cfg (feature = "have_haiku")]
const METRICS_INVALID: metrics_status = -1;

/* C 50: `#define METRICS_STATUS(metrics) ((metrics)->ascent + (metrics)->descent)`.  C's macro
   evaluates its argument twice, which the port keeps (PORTING.md 2.1/P-16).  */
#[cfg (feature = "have_haiku")]
macro_rules! METRICS_STATUS {
  ($metrics:expr) => { ((*$metrics).ascent + (*$metrics).descent) };
}

/* C 51-52: `#define METRICS_SET_STATUS(metrics, status) ((metrics)->ascent = 0,
   (metrics)->descent = (status))` - a comma expression, which Rust has no spelling for, so the
   body is a block of the same two assignments in C's order (P-37, the `include/haikugui.rs` 69
   shape).  Its value (the last assignment) is read by no caller, and the `status` narrowing is
   C's own (the only call site passes the `metrics_status` value `METRICS_INVALID`).  */
#[cfg (feature = "have_haiku")]
macro_rules! METRICS_SET_STATUS {
  ($metrics:expr, $status:expr) => { {
      (*$metrics).ascent = 0;
      (*$metrics).descent = $status as c_short;
    } };
}

/* C 54-104: the file-scope `em_charset_table` - an array of a C *anonymous* struct, which
   PORTING.md 1.3 requires the port to name; the name follows the object
   (`em_charset_table_entry`, the `fc_charset_table_entry`/`charset_map_entry` shape).  C's
   aggregate initialisation leaves the unused `uniquifier` slots and the `lang` pointers zero,
   which the port spells out.  The last element is C 103's `{ NULL }` sentinel: C 132/136 walk the
   array until `name` is null.  */
/// C: 54-62: the anonymous record's members (`name`, `uniquifier[6]`, `lang`).
#[cfg (feature = "have_haiku")]
#[repr(C)]
pub struct em_charset_table_entry
{
  /* registry name */
  pub name: *const c_char,
  /* characters to distinguish the charset from the others */
  pub uniquifier: [c_int; 6],
  /* additional constraint by language */
  pub lang: *const c_char,
}

/// C 62: `} em_charset_table[] =` - C 63-102's 40 records plus C 103's sentinel.
#[cfg (feature = "have_haiku")]
static mut em_charset_table: [em_charset_table_entry; 41] =
[
  em_charset_table_entry { name: c"iso8859-1".as_ptr (), uniquifier: [0x00A0, 0x00A1, 0x00B4, 0x00BC, 0x00D0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-2".as_ptr (), uniquifier: [0x00A0, 0x010E, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-3".as_ptr (), uniquifier: [0x00A0, 0x0108, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-4".as_ptr (), uniquifier: [0x00A0, 0x00AF, 0x0128, 0x0156, 0x02C7, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-5".as_ptr (), uniquifier: [0x00A0, 0x0401, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-6".as_ptr (), uniquifier: [0x00A0, 0x060C, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-7".as_ptr (), uniquifier: [0x00A0, 0x0384, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-8".as_ptr (), uniquifier: [0x00A0, 0x05D0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-9".as_ptr (), uniquifier: [0x00A0, 0x00A1, 0x00BC, 0x011E, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-10".as_ptr (), uniquifier: [0x00A0, 0x00D0, 0x0128, 0x2015, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-11".as_ptr (), uniquifier: [0x00A0, 0x0E01, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-13".as_ptr (), uniquifier: [0x00A0, 0x201C, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-14".as_ptr (), uniquifier: [0x00A0, 0x0174, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-15".as_ptr (), uniquifier: [0x00A0, 0x00A1, 0x00D0, 0x0152, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"iso8859-16".as_ptr (), uniquifier: [0x00A0, 0x0218, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"gb2312.1980-0".as_ptr (), uniquifier: [0x4E13, 0, 0, 0, 0, 0], lang: c"zh-cn".as_ptr () },
  em_charset_table_entry { name: c"big5-0".as_ptr (), uniquifier: [0x9C21, 0, 0, 0, 0, 0], lang: c"zh-tw".as_ptr () },
  em_charset_table_entry { name: c"jisx0208.1983-0".as_ptr (), uniquifier: [0x4E55, 0, 0, 0, 0, 0], lang: c"ja".as_ptr () },
  em_charset_table_entry { name: c"ksc5601.1985-0".as_ptr (), uniquifier: [0xAC00, 0, 0, 0, 0, 0], lang: c"ko".as_ptr () },
  em_charset_table_entry { name: c"cns11643.1992-1".as_ptr (), uniquifier: [0xFE32, 0, 0, 0, 0, 0], lang: c"zh-tw".as_ptr () },
  em_charset_table_entry { name: c"cns11643.1992-2".as_ptr (), uniquifier: [0x4E33, 0x7934, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"cns11643.1992-3".as_ptr (), uniquifier: [0x201A9, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"cns11643.1992-4".as_ptr (), uniquifier: [0x20057, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"cns11643.1992-5".as_ptr (), uniquifier: [0x20000, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"cns11643.1992-6".as_ptr (), uniquifier: [0x20003, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"cns11643.1992-7".as_ptr (), uniquifier: [0x20055, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"gbk-0".as_ptr (), uniquifier: [0x4E06, 0, 0, 0, 0, 0], lang: c"zh-cn".as_ptr () },
  em_charset_table_entry { name: c"jisx0212.1990-0".as_ptr (), uniquifier: [0x4E44, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"jisx0213.2000-1".as_ptr (), uniquifier: [0xFA10, 0, 0, 0, 0, 0], lang: c"ja".as_ptr () },
  em_charset_table_entry { name: c"jisx0213.2000-2".as_ptr (), uniquifier: [0xFA49, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"jisx0213.2004-1".as_ptr (), uniquifier: [0x20B9F, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"viscii1.1-1".as_ptr (), uniquifier: [0x1EA0, 0x1EAE, 0x1ED2, 0, 0, 0], lang: c"vi".as_ptr () },
  em_charset_table_entry { name: c"tis620.2529-1".as_ptr (), uniquifier: [0x0E01, 0, 0, 0, 0, 0], lang: c"th".as_ptr () },
  em_charset_table_entry { name: c"microsoft-cp1251".as_ptr (), uniquifier: [0x0401, 0x0490, 0, 0, 0, 0], lang: c"ru".as_ptr () },
  em_charset_table_entry { name: c"koi8-r".as_ptr (), uniquifier: [0x0401, 0x2219, 0, 0, 0, 0], lang: c"ru".as_ptr () },
  em_charset_table_entry { name: c"mulelao-1".as_ptr (), uniquifier: [0x0E81, 0, 0, 0, 0, 0], lang: c"lo".as_ptr () },
  em_charset_table_entry { name: c"unicode-sip".as_ptr (), uniquifier: [0x20000, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"mulearabic-0".as_ptr (), uniquifier: [0x628, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"mulearabic-1".as_ptr (), uniquifier: [0x628, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: c"mulearabic-2".as_ptr (), uniquifier: [0x628, 0, 0, 0, 0, 0], lang: std::ptr::null () },
  em_charset_table_entry { name: std::ptr::null (), uniquifier: [0, 0, 0, 0, 0, 0], lang: std::ptr::null () },
];

/* C 106-177: `haikufont_apply_registry`.  C's `char *str` is a keyword in Rust, so the local
   keeps C's name as the raw identifier `r#str` (PORTING.md 0.2).  */
/// C: `static void haikufont_apply_registry (struct haiku_font_pattern *pattern, Lisp_Object
/// registry);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_apply_registry (pattern: *mut haiku_font_pattern, registry: Lisp_Object)
{
  let r#str: *mut c_char = SSDATA (SYMBOL_NAME (registry));       // C 110
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                          // C 111
  let re: *mut c_char
    = SAFE_ALLOCA! (sa_avail, SBYTES (SYMBOL_NAME (registry)) * 2 + 1) as *mut c_char;  // C 112
  let mut i: c_int;                                               // C 113
  let mut j: c_int;

  i = 0;                                                          // C 115
  j = 0;
  while (i as isize) < SBYTES (SYMBOL_NAME (registry))
    {
      if *r#str.offset (i as isize) == b'.' as c_char
	{ *re.offset (j as isize) = b'\\' as c_char; j += 1; }
      else if *r#str.offset (i as isize) == b'*' as c_char
	{ *re.offset (j as isize) = b'.' as c_char; j += 1; }
      *re.offset (j as isize) = *r#str.offset (i as isize);
      if *re.offset (j as isize) == b'?' as c_char
	{ *re.offset (j as isize) = b'.' as c_char; }
      i += 1;
      j += 1;
    }
  *re.offset (j as isize) = 0;                                    // C 125
  AUTO_STRING_WITH_LEN! (regexp, re, j as isize);                 // C 126
  i = 0;                                                          // C 127
  while !em_charset_table[i as usize].name.is_null ()
    {
      if fast_c_string_match_ignore_case
	   (regexp, em_charset_table[i as usize].name,
	    strlen (em_charset_table[i as usize].name)) >= 0
	{ break; }
      i += 1;
    }
  SAFE_FREE! (sa_count, sa_mark);                                          // C 132
  if em_charset_table[i as usize].name.is_null ()                 // C 133
    { return; }
  let uniquifier: *mut c_int                                      // C 135
    = em_charset_table[i as usize].uniquifier.as_mut_ptr ();
  let mut l: c_int;                                               // C 136

  l = 0;                                                          // C 138
  while *uniquifier.offset (l as isize) != 0
    { l += 1; }

  let mut a: *mut c_int = xmalloc (l as usize * std::mem::size_of::<c_int> ()) as *mut c_int;  // C 140
  l = 0;                                                          // C 141
  while *uniquifier.offset (l as isize) != 0
    {
      *a.offset (l as isize) = *uniquifier.offset (l as isize);
      l += 1;
    }

  if ((*pattern).specified & FSPEC_WANTED) != 0                    // C 144
    {
      let old_l: c_int = l;                                        // C 146
      l += (*pattern).want_chars_len;                              // C 147
      a = xrealloc (a as *mut c_void, l as usize * std::mem::size_of::<c_int> ())
	    as *mut c_int;                                         // C 148
      memcpy (a.offset (old_l as isize) as *mut c_void,
	      (*pattern).wanted_chars as *const c_void,
	      (l - old_l) as usize * std::mem::size_of::<c_int> ());  // C 149
      xfree ((*pattern).wanted_chars as *mut c_void);              // C 150
    }

  (*pattern).specified |= FSPEC_WANTED;                            // C 153
  (*pattern).want_chars_len = l;                                   // C 154
  (*pattern).wanted_chars = a;                                     // C 155

  if !em_charset_table[i as usize].lang.is_null ()                 // C 157
    {
      if strncmp (em_charset_table[i as usize].lang, c"zh".as_ptr (), 2) == 0    // C 159
	{
	  (*pattern).specified |= FSPEC_LANGUAGE;                  // C 161
	  (*pattern).language = LANGUAGE_CN;                       // C 162
	}
      else if strncmp (em_charset_table[i as usize].lang, c"ko".as_ptr (), 2) == 0  // C 164
	{
	  (*pattern).specified |= FSPEC_LANGUAGE;                  // C 166
	  (*pattern).language = LANGUAGE_KO;                       // C 167
	}
      else if strncmp (em_charset_table[i as usize].lang, c"ja".as_ptr (), 2) == 0  // C 169
	{
	  (*pattern).specified |= FSPEC_LANGUAGE;                  // C 171
	  (*pattern).language = LANGUAGE_JP;                       // C 172
	}
    }

  return;                                                          // C 176
}

// C 179-196.
/// C: `static Lisp_Object haikufont_get_fallback_entity (void);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_get_fallback_entity () -> Lisp_Object
{
  let ent: Lisp_Object = font_make_entity ();                     // C 182
  ASET (ent, FONT_TYPE_INDEX as isize, Qhaiku ());                // C 183
  ASET (ent, FONT_FOUNDRY_INDEX as isize, Qhaiku ());             // C 184
  ASET (ent, FONT_FAMILY_INDEX as isize, Qnil ());                // C 185
  ASET (ent, FONT_ADSTYLE_INDEX as isize, Qnil ());               // C 186
  ASET (ent, FONT_REGISTRY_INDEX as isize, Qiso10646_1 ());       // C 187
  ASET (ent, FONT_SIZE_INDEX as isize, make_fixnum (0));          // C 188
  ASET (ent, FONT_AVGWIDTH_INDEX as isize, make_fixnum (0));      // C 189
  ASET (ent, FONT_SPACING_INDEX as isize,
	make_fixnum (FONT_SPACING_MONO as EMACS_INT));            // C 190
  FONT_SET_STYLE! (ent, FONT_WIDTH_INDEX, Qnil ());               // C 191
  FONT_SET_STYLE! (ent, FONT_WEIGHT_INDEX, Qnil ());              // C 192
  FONT_SET_STYLE! (ent, FONT_SLANT_INDEX, Qnil ());               // C 193

  return ent;                                                     // C 195
}

// C 198-202.  C's `struct frame *frame` is the mirror's untyped `get_cache` parameter (font.h 605
// spells it `struct frame *`; P-72 (3) degrades only the pointer, and this body never reads it).
/// C: `static Lisp_Object haikufont_get_cache (struct frame *frame);`
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_get_cache (frame: *mut c_void) -> Lisp_Object
{
  return font_cache;                                              // C 201
}

// C 204-237.
/// C: `static Lisp_Object haikufont_weight_to_lisp (int weight);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_weight_to_lisp (weight: c_int) -> Lisp_Object
{
  match weight                                                    // C 207
    {
      HAIKU_THIN => { return Qthin (); }                          // C 209-210
      HAIKU_EXTRALIGHT => { return Qextra_light (); }             // C 211-212
      HAIKU_LIGHT => { return Qlight (); }                        // C 213-214
      HAIKU_SEMI_LIGHT => { return Qsemi_light (); }              // C 215-216
      HAIKU_REGULAR => { return Qnormal (); }                     // C 217-218
      HAIKU_SEMI_BOLD => { return Qsemi_bold (); }                // C 219-220
      HAIKU_BOLD => { return Qbold (); }                          // C 221-222
      HAIKU_EXTRA_BOLD => { return Qextra_bold (); }              // C 223-224
      HAIKU_BOOK => { return Qbook (); }                          // C 225-226
      HAIKU_HEAVY => { return Qheavy (); }                        // C 227-228
      HAIKU_ULTRA_HEAVY => { return Qultra_heavy (); }            // C 229-230
      HAIKU_BLACK => { return Qblack (); }                        // C 231-232
      HAIKU_MEDIUM => { return Qmedium (); }                      // C 233-234
      _ => { }   // C's `switch` has no `default` arm (the fall-through Rust requires).
    }
  emacs_abort ();                                                 // C 236
}

// C 239-274.
/// C: `static int haikufont_lisp_to_weight (Lisp_Object weight);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_lisp_to_weight (weight: Lisp_Object) -> c_int
{
  if EQ (weight, Qthin ())                                        // C 241
    { return HAIKU_THIN; }
  if EQ (weight, Qultra_light ())                                 // C 243
    { return HAIKU_EXTRALIGHT; }
  if EQ (weight, Qextra_light ())                                 // C 245
    { return HAIKU_EXTRALIGHT; }
  if EQ (weight, Qlight ())                                       // C 247
    { return HAIKU_LIGHT; }
  if EQ (weight, Qsemi_light ())                                  // C 249
    { return HAIKU_SEMI_LIGHT; }
  if EQ (weight, Qnormal ()) || EQ (weight, Qregular ())          // C 251
    { return HAIKU_REGULAR; }
  if EQ (weight, Qsemi_bold ())                                   // C 253
    { return HAIKU_SEMI_BOLD; }
  if EQ (weight, Qbold ())                                        // C 255
    { return HAIKU_BOLD; }
  if EQ (weight, Qextra_bold ())                                  // C 257
    { return HAIKU_EXTRA_BOLD; }
  if EQ (weight, Qultra_bold ())                                  // C 259
    { return HAIKU_EXTRA_BOLD; }
  if EQ (weight, Qbook ())                                        // C 261
    { return HAIKU_BOOK; }
  if EQ (weight, Qheavy ())                                       // C 263
    { return HAIKU_HEAVY; }
  if EQ (weight, Qultra_heavy ())                                 // C 265
    { return HAIKU_ULTRA_HEAVY; }
  if EQ (weight, Qblack ())                                       // C 267
    { return HAIKU_BLACK; }
  if EQ (weight, Qmedium ())                                      // C 269
    { return HAIKU_MEDIUM; }

  return HAIKU_REGULAR;                                           // C 273
}

// C 276-291.  C's `case NO_SLANT:` falls into the `emacs_abort ()` line and on into the next case
// label; the port's arm keeps the abort and the fall-through is C's own unreachable one.
/// C: `static Lisp_Object haikufont_slant_to_lisp (enum haiku_font_slant slant);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_slant_to_lisp (slant: haiku_font_slant) -> Lisp_Object
{
  match slant                                                     // C 279
    {
      NO_SLANT => { emacs_abort (); }                             // C 281-282
      SLANT_ITALIC => { return Qitalic (); }                      // C 283-284
      SLANT_REGULAR => { return Qnormal (); }                     // C 285-286
      SLANT_OBLIQUE => { return Qoblique (); }                    // C 287-288
      _ => { }   // C's `switch` has no `default` arm (the fall-through Rust requires).
    }
  emacs_abort ();                                                 // C 290
}

// C 293-306.
/// C: `static enum haiku_font_slant haikufont_lisp_to_slant (Lisp_Object slant);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_lisp_to_slant (slant: Lisp_Object) -> haiku_font_slant
{
  if EQ (slant, Qitalic ())
      || EQ (slant, Qreverse_italic ())                           // C 296-297
    { return SLANT_ITALIC; }
  if EQ (slant, Qoblique ())
      || EQ (slant, Qreverse_oblique ())                          // C 299-300
    { return SLANT_OBLIQUE; }
  if EQ (slant, Qnormal ()) || EQ (slant, Qregular ())            // C 302
    { return SLANT_REGULAR; }

  return SLANT_REGULAR;                                           // C 305
}

// C 308-336.
/// C: `static Lisp_Object haikufont_width_to_lisp (enum haiku_font_width width);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_width_to_lisp (width: haiku_font_width) -> Lisp_Object
{
  match width                                                     // C 311
    {
      NO_WIDTH => { emacs_abort (); }                             // C 313-314
      ULTRA_CONDENSED => { return Qultra_condensed (); }          // C 315-316
      EXTRA_CONDENSED => { return Qextra_condensed (); }          // C 317-318
      CONDENSED => { return Qcondensed (); }                      // C 319-320
      SEMI_CONDENSED => { return Qsemi_condensed (); }            // C 321-322
      NORMAL_WIDTH => { return Qnormal (); }                      // C 323-324
      SEMI_EXPANDED => { return Qsemi_expanded (); }              // C 325-326
      EXPANDED => { return Qexpanded (); }                        // C 327-328
      EXTRA_EXPANDED => { return Qextra_expanded (); }            // C 329-330
      ULTRA_EXPANDED => { return Qultra_expanded (); }            // C 331-332
      _ => { }   // C's `switch` has no `default` arm (the fall-through Rust requires).
    }

  emacs_abort ();                                                 // C 335
}

// C 338-359.
/// C: `static enum haiku_font_width haikufont_lisp_to_width (Lisp_Object lisp);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_lisp_to_width (lisp: Lisp_Object) -> haiku_font_width
{
  if EQ (lisp, Qultra_condensed ())                               // C 340
    { return ULTRA_CONDENSED; }
  if EQ (lisp, Qextra_condensed ())                               // C 342
    { return EXTRA_CONDENSED; }
  if EQ (lisp, Qcondensed ())                                     // C 344
    { return CONDENSED; }
  if EQ (lisp, Qsemi_condensed ())                                // C 346
    { return SEMI_CONDENSED; }
  if EQ (lisp, Qnormal ()) || EQ (lisp, Qregular ())              // C 348
    { return NORMAL_WIDTH; }
  if EQ (lisp, Qexpanded ())                                      // C 350
    { return EXPANDED; }
  if EQ (lisp, Qextra_expanded ())                                // C 352
    { return EXTRA_EXPANDED; }
  if EQ (lisp, Qultra_expanded ())                                // C 354
    { return ULTRA_EXPANDED; }

  return NORMAL_WIDTH;                                            // C 357
}

// C 361-379.
/// C: `static int haikufont_maybe_handle_special_family (Lisp_Object family,
/// struct haiku_font_pattern *ptn);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_maybe_handle_special_family (family: Lisp_Object,
						 ptn: *mut haiku_font_pattern) -> c_int
{
  CHECK_SYMBOL (family);                                          // C 365

  if EQ (family, Qmonospace ()) || EQ (family, Qfixed ())
      || EQ (family, Qdefault ())                                 // C 367-368
    {
      BFont_populate_fixed_family (ptn);                          // C 370
      return 1;                                                   // C 371
    }
  else if EQ (family, QSans_Serif ())                             // C 373
    {
      BFont_populate_plain_family (ptn);                          // C 375
      return 1;                                                   // C 376
    }
  return 0;                                                       // C 378
}

// C 381-445.
/// C: `static Lisp_Object haikufont_pattern_to_entity (struct haiku_font_pattern *ptn);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_pattern_to_entity (ptn: *mut haiku_font_pattern) -> Lisp_Object
{
  let entity: Lisp_Object;                                        // C 384
  let mut extras: Lisp_Object;

  entity = font_make_entity ();                                   // C 386
  extras = Qnil ();                                               // C 387

  ASET (entity, FONT_TYPE_INDEX as isize, Qhaiku ());             // C 389
  ASET (entity, FONT_FOUNDRY_INDEX as isize, Qhaiku ());          // C 390
  ASET (entity, FONT_FAMILY_INDEX as isize, Qdefault ());         // C 391
  ASET (entity, FONT_ADSTYLE_INDEX as isize, Qnil ());            // C 392
  ASET (entity, FONT_REGISTRY_INDEX as isize, Qiso10646_1 ());    // C 393
  ASET (entity, FONT_SIZE_INDEX as isize, make_fixnum (0));       // C 394
  ASET (entity, FONT_AVGWIDTH_INDEX as isize, make_fixnum (0));   // C 395
  ASET (entity, FONT_SPACING_INDEX as isize,
	make_fixnum (FONT_SPACING_MONO as EMACS_INT));            // C 396

  /* FONT_EXTRA_INDEX in a font entity can contain a cons of two
     numbers (STYLE . IDX) under the key :indices that tell Emacs how
     to open a font.  */
  if ((*ptn).specified & FSPEC_INDICES) != 0                      // C 401
    { extras = Fcons (Fcons (QCindices (),
			   Fcons (make_fixnum ((*ptn).family_index as EMACS_INT),
				  make_fixnum ((*ptn).style_index as EMACS_INT))),
		    extras); }                                    // C 402-405

  if ((*ptn).specified & FSPEC_ANTIALIAS) != 0                    // C 407
    { extras = Fcons (Fcons (QCantialias (),
			   if (*ptn).use_antialiasing != 0 { Qt () } else { Qnil () }),
		    extras); }                                    // C 408-410

  ASET (entity, FONT_EXTRA_INDEX as isize, extras);               // C 412

  FONT_SET_STYLE! (entity, FONT_WIDTH_INDEX, Qnormal ());         // C 414
  FONT_SET_STYLE! (entity, FONT_WEIGHT_INDEX, Qnormal ());        // C 415
  FONT_SET_STYLE! (entity, FONT_SLANT_INDEX, Qnormal ());         // C 416

  if ((*ptn).specified & FSPEC_FAMILY) != 0                       // C 418
    { ASET (entity, FONT_FAMILY_INDEX as isize, intern ((*ptn).family.as_ptr ())); }  // C 419
  else
    { ASET (entity, FONT_FAMILY_INDEX as isize, Qdefault ()); }   // C 421

  if ((*ptn).specified & FSPEC_STYLE) != 0                        // C 423
    { ASET (entity, FONT_ADSTYLE_INDEX as isize, intern ((*ptn).style.as_ptr ())); }  // C 424
  else
    {
      if ((*ptn).specified & FSPEC_WEIGHT) != 0                   // C 427
	{ FONT_SET_STYLE! (entity, FONT_WEIGHT_INDEX,
			   haikufont_weight_to_lisp ((*ptn).weight)); }  // C 428-429
      if ((*ptn).specified & FSPEC_SLANT) != 0                    // C 430
	{ FONT_SET_STYLE! (entity, FONT_SLANT_INDEX,
			   haikufont_slant_to_lisp ((*ptn).slant)); }  // C 431-432
      if ((*ptn).specified & FSPEC_WIDTH) != 0                    // C 433
	{ FONT_SET_STYLE! (entity, FONT_WIDTH_INDEX,
			   haikufont_width_to_lisp ((*ptn).width)); }  // C 434-435
    }

  if ((*ptn).specified & FSPEC_SPACING) != 0                      // C 438
    { ASET (entity, FONT_SPACING_INDEX as isize,
	    make_fixnum ((if (*ptn).mono_spacing_p != 0
			    { FONT_SPACING_MONO } else { FONT_SPACING_PROPORTIONAL })
			 as EMACS_INT)); }                        // C 439-442

  return entity;                                                  // C 444
}

// C 447-503.  C's two `strncpy` calls write into the pattern's array members; `sizeof
// pattern->family` is the array type's size (the mirror's `haiku_font_family_or_style`), and the
// assignment of C's `!NILP` result to the `int` member `use_antialiasing` spells C's own
// bool-to-int conversion.
/// C: `static void haikufont_pattern_from_object (struct haiku_font_pattern *pattern,
/// Lisp_Object font_object);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_pattern_from_object (pattern: *mut haiku_font_pattern,
					 font_object: Lisp_Object)
{
  let mut val: Lisp_Object;                                       // C 451

  (*pattern).specified = 0;                                       // C 453

  val = AREF (font_object, FONT_FAMILY_INDEX as isize);           // C 455
  if !NILP (val)                                                  // C 456
    {
      (*pattern).specified |= FSPEC_FAMILY;                       // C 458
      strncpy ((*pattern).family.as_mut_ptr (),
	       SSDATA (SYMBOL_NAME (val)),
	       std::mem::size_of::<haiku_font_family_or_style> () - 1);  // C 459-461
      (*pattern).family[std::mem::size_of::<haiku_font_family_or_style> () - 1] = 0;  // C 462
    }

  val = AREF (font_object, FONT_ADSTYLE_INDEX as isize);          // C 465
  if !NILP (val)                                                  // C 466
    {
      (*pattern).specified |= FSPEC_STYLE;                        // C 468
      strncpy ((*pattern).style.as_mut_ptr (),
	       SSDATA (SYMBOL_NAME (val)),
	       std::mem::size_of::<haiku_font_family_or_style> () - 1);  // C 469-471
      (*pattern).style[std::mem::size_of::<haiku_font_family_or_style> () - 1] = 0;  // C 472
    }

  val = FONT_WEIGHT_FOR_FACE! (font_object);                      // C 475
  if !NILP (val) && !EQ (val, Qunspecified ())                    // C 476
    {
      (*pattern).specified |= FSPEC_WEIGHT;                       // C 478
      (*pattern).weight = haikufont_lisp_to_weight (val);         // C 479
    }

  val = FONT_SLANT_FOR_FACE! (font_object);                       // C 482
  if !NILP (val) && !EQ (val, Qunspecified ())                    // C 483
    {
      (*pattern).specified |= FSPEC_SLANT;                        // C 485
      (*pattern).slant = haikufont_lisp_to_slant (val);           // C 486
    }

  val = FONT_WIDTH_FOR_FACE! (font_object);                       // C 489
  if !NILP (val) && !EQ (val, Qunspecified ())                    // C 490
    {
      (*pattern).specified |= FSPEC_WIDTH;                        // C 492
      (*pattern).width = haikufont_lisp_to_width (val);           // C 493
    }

  val = assq_no_quit (QCantialias (),
		      AREF (font_object, FONT_EXTRA_INDEX as isize));  // C 496-497
  if CONSP (val)                                                  // C 498
    {
      (*pattern).specified |= FSPEC_ANTIALIAS;                    // C 500
      (*pattern).use_antialiasing = if !NILP (XCDR (val)) { 1 } else { 0 };  // C 501
    }
}

// C 505-647.
/// C: `static void haikufont_spec_or_entity_to_pattern (Lisp_Object ent, int list_p,
/// struct haiku_font_pattern *ptn);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_spec_or_entity_to_pattern (ent: Lisp_Object, list_p: c_int,
					       ptn: *mut haiku_font_pattern)
{
  let mut tem: Lisp_Object;                                       // C 509
  (*ptn).specified = 0;                                           // C 510

  tem = AREF (ent, FONT_ADSTYLE_INDEX as isize);                  // C 512
  if !NILP (tem)                                                  // C 513
    {
      (*ptn).specified |= FSPEC_STYLE;                            // C 515
      strncpy ((*ptn).style.as_mut_ptr (),
	       SSDATA (SYMBOL_NAME (tem)),
	       std::mem::size_of::<haiku_font_family_or_style> () - 1);  // C 516-518
      (*ptn).style[std::mem::size_of::<haiku_font_family_or_style> () - 1] = 0;  // C 519
    }

  tem = FONT_SLANT_SYMBOLIC! (ent);                               // C 522
  if !NILP (tem) && !EQ (tem, Qunspecified ())                    // C 523
    {
      (*ptn).specified |= FSPEC_SLANT;                            // C 525
      (*ptn).slant = haikufont_lisp_to_slant (tem);               // C 526
    }

  tem = FONT_WEIGHT_SYMBOLIC! (ent);                              // C 529
  if !NILP (tem) && !EQ (tem, Qunspecified ())                    // C 530
    {
      (*ptn).specified |= FSPEC_WEIGHT;                           // C 532
      (*ptn).weight = haikufont_lisp_to_weight (tem);             // C 533
    }

  tem = FONT_WIDTH_SYMBOLIC! (ent);                               // C 536
  if !NILP (tem) && !EQ (tem, Qunspecified ())                    // C 537
    {
      (*ptn).specified |= FSPEC_WIDTH;                            // C 539
      (*ptn).width = haikufont_lisp_to_width (tem);               // C 540
    }

  tem = AREF (ent, FONT_SPACING_INDEX as isize);                  // C 543
  if !NILP (tem) && !EQ (tem, Qunspecified ())                    // C 544
    {
      (*ptn).specified |= FSPEC_SPACING;                          // C 546
      (*ptn).mono_spacing_p
	= if XFIXNUM (tem) != FONT_SPACING_PROPORTIONAL as EMACS_INT { 1 } else { 0 };  // C 547
    }

  tem = AREF (ent, FONT_FAMILY_INDEX as isize);                   // C 550
  if !NILP (tem) && !EQ (tem, Qunspecified ())
      && (list_p != 0
	  && haikufont_maybe_handle_special_family (tem, ptn) == 0)  // C 551-553
    {
      (*ptn).specified |= FSPEC_FAMILY;                           // C 555
      strncpy ((*ptn).family.as_mut_ptr (),
	       SSDATA (SYMBOL_NAME (tem)),
	       std::mem::size_of::<haiku_font_family_or_style> () - 1);  // C 556-558
      (*ptn).family[std::mem::size_of::<haiku_font_family_or_style> () - 1] = 0;  // C 559
    }

  tem = assq_no_quit (QCscript (), AREF (ent, FONT_EXTRA_INDEX as isize));  // C 562
  if !NILP (tem)                                                  // C 563
    {
      tem = assq_no_quit (XCDR (tem), Vscript_representative_chars);  // C 565

      if CONSP (tem) && VECTORP (XCDR (tem))                      // C 567
	{
	  tem = XCDR (tem);                                       // C 569

	  let mut count: c_int = 0;                               // C 571
	  let mut j: c_int = 0;

	  while j < ASIZE (tem) as c_int                          // C 573
	    {
	      if TYPE_RANGED_FIXNUMP (0, AREF (tem, j as isize), UINT_MAX as EMACS_INT)  // C 574
		{ count += 1; }
	      j += 1;
	    }

	  if count != 0                                           // C 577
	    {
	      (*ptn).specified |= FSPEC_NEED_ONE_OF;              // C 579
	      (*ptn).need_one_of_len = count;                     // C 580
	      (*ptn).need_one_of
		= xmalloc (count as usize * std::mem::size_of::<c_int> ()) as *mut c_int;  // C 581
	      count = 0;                                          // C 582
	      let mut j: c_int = 0;
	      while j < ASIZE (tem) as c_int                      // C 583
		{
		  if TYPE_RANGED_FIXNUMP (0, AREF (tem, j as isize), UINT_MAX as EMACS_INT)  // C 584
		    {
		      *(*ptn).need_one_of.offset (j as isize)
			= XFIXNAT (AREF (tem, j as isize)) as c_int;  // C 586
		      count += 1;                                     // C 587
		    }
		  j += 1;
		}
	    }
	}
      else if CONSP (tem) && CONSP (XCDR (tem))                   // C 591
	{
	  let mut count: c_int = 0;                               // C 593

	  let mut it: Lisp_Object = XCDR (tem);                   // C 595
	  while CONSP (it)
	    {
	      if TYPE_RANGED_FIXNUMP (0, XCAR (it), UINT_MAX as EMACS_INT)  // C 596
		{ count += 1; }
	      it = XCDR (it);
	    }

	  if count != 0                                           // C 599
	    {
	      (*ptn).specified |= FSPEC_WANTED;                   // C 601
	      (*ptn).want_chars_len = count;                      // C 602
	      (*ptn).wanted_chars
		= xmalloc (count as usize * std::mem::size_of::<c_int> ()) as *mut c_int;  // C 603
	      count = 0;                                          // C 604

	      tem = XCDR (tem);                                   // C 606
	      while CONSP (tem)
		{
		  if TYPE_RANGED_FIXNUMP (0, XCAR (tem), UINT_MAX as EMACS_INT)  // C 607
		    {
		      *(*ptn).wanted_chars.offset (count as isize)
			= XFIXNAT (XCAR (tem)) as c_int;          // C 609
		      count += 1;                                     // C 610
		    }
		  tem = XCDR (tem);
		}
	    }
	}
    }

  tem = assq_no_quit (QClang (), AREF (ent, FONT_EXTRA_INDEX as isize));  // C 616
  if CONSP (tem)                                                  // C 617
    {
      tem = XCDR (tem);                                           // C 619
      if EQ (tem, Qzh ())                                         // C 620
	{
	  (*ptn).specified |= FSPEC_LANGUAGE;                     // C 622
	  (*ptn).language = LANGUAGE_CN;                          // C 623
	}
      else if EQ (tem, Qko ())                                    // C 625
	{
	  (*ptn).specified |= FSPEC_LANGUAGE;                     // C 627
	  (*ptn).language = LANGUAGE_KO;                          // C 628
	}
      else if EQ (tem, Qjp ())                                    // C 630
	{
	  (*ptn).specified |= FSPEC_LANGUAGE;                     // C 632
	  (*ptn).language = LANGUAGE_JP;                          // C 633
	}
    }

  tem = assq_no_quit (QCantialias (), AREF (ent, FONT_EXTRA_INDEX as isize));  // C 637
  if CONSP (tem)                                                  // C 638
    {
      (*ptn).specified |= FSPEC_ANTIALIAS;                        // C 640
      (*ptn).use_antialiasing = if !NILP (XCDR (tem)) { 1 } else { 0 };  // C 641
    }

  tem = AREF (ent, FONT_REGISTRY_INDEX as isize);                 // C 644
  if SYMBOLP (tem)                                                // C 645
    { haikufont_apply_registry (ptn, tem); }                      // C 646
}

// C 649-657.
/// C: `static void haikufont_done_with_query_pattern (struct haiku_font_pattern *ptn);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_done_with_query_pattern (ptn: *mut haiku_font_pattern)
{
  if ((*ptn).specified & FSPEC_WANTED) != 0                       // C 652
    { xfree ((*ptn).wanted_chars as *mut c_void); }               // C 653

  if ((*ptn).specified & FSPEC_NEED_ONE_OF) != 0                  // C 655
    { xfree ((*ptn).need_one_of as *mut c_void); }                // C 656
}

// C 659-676.  C's `struct haiku_font_pattern ptn;` is uninitialised and
// `haikufont_spec_or_entity_to_pattern` fills only the fields the spec names, so the port states
// the zero value C's own aggregate would start from (PORTING.md 1.5, the `## Deviations` item).
/// C: `static Lisp_Object haikufont_match (struct frame *f, Lisp_Object font_spec);` - the frame
/// is the mirror's untyped `match` parameter (font.h 632 spells it `struct frame *`; P-72 (3)).
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_match (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{
  block_input ();                                                 // C 662
  let mut tem: Lisp_Object = Qnil ();                             // C 663
  let mut ptn: haiku_font_pattern = std::mem::zeroed ();          // C 664
  haikufont_spec_or_entity_to_pattern (font_spec, 0, &mut ptn);   // C 665
  ptn.specified &= !FSPEC_FAMILY;                                 // C 666
  let found: *mut haiku_font_pattern = BFont_find (&mut ptn);     // C 667
  haikufont_done_with_query_pattern (&mut ptn);                   // C 668
  if !found.is_null ()                                            // C 669
    {
      tem = haikufont_pattern_to_entity (found);                  // C 671
      haiku_font_pattern_free (found);                            // C 672
    }
  unblock_input ();                                               // C 674
  return if !NILP (tem) { tem } else { haikufont_get_fallback_entity () };  // C 675
}

// C 678-709.
/// C: `static Lisp_Object haikufont_list (struct frame *f, Lisp_Object font_spec);` (untyped
/// frame, see haikufont_match).
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_list (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{
  let mut lst: Lisp_Object;                                       // C 681
  let mut tem: Lisp_Object;
  let mut ptn: haiku_font_pattern = std::mem::zeroed ();          // C 682
  let mut found: *mut haiku_font_pattern;
  let mut pt: *mut haiku_font_pattern;

  lst = Qnil ();                                                  // C 684

  block_input ();                                                 // C 686
  /* Returning irrelevant results on receiving an OTF form will cause
     fontset.c to loop over and over, making displaying some
     characters very slow.  */
  tem = assq_no_quit (QCotf (), AREF (font_spec, FONT_EXTRA_INDEX as isize));  // C 690

  if CONSP (tem) && !NILP (XCDR (tem))                            // C 692
    {
      unblock_input ();                                           // C 694
      return Qnil ();                                             // C 695
    }

  haikufont_spec_or_entity_to_pattern (font_spec, 1, &mut ptn);   // C 698
  found = BFont_find (&mut ptn);                                  // C 699
  haikufont_done_with_query_pattern (&mut ptn);                   // C 700
  if !found.is_null ()                                            // C 701
    {
      pt = found;                                                 // C 703
      while !pt.is_null ()
	{
	  lst = Fcons (haikufont_pattern_to_entity (pt), lst);    // C 704
	  pt = (*pt).next;
	}
      haiku_font_pattern_free (found);                            // C 705
    }
  unblock_input ();                                               // C 707
  return lst;                                                     // C 708
}

// C 711-738.  C's `unsigned short *unichars = xmalloc (0x101 * sizeof (*unichars))` is one
// statement here (the assignment C splits over C 714/C 719), and the two narrowings
// (`unsigned int` to `unsigned short` at C 724/C 734, `unsigned short` to `int32_t` at C 730/C 733)
// are C's own implicit conversions.
/// C: `static void haiku_bulk_encode (struct haikufont_info *font_info, int block);`
#[cfg (feature = "have_haiku")]
unsafe fn haiku_bulk_encode (font_info: *mut haikufont_info, block: c_int)
{
  let unichars: *mut c_ushort
    = xmalloc (0x101 * std::mem::size_of::<c_ushort> ()) as *mut c_ushort;  // C 714
  let mut i: c_uint;                                              // C 715
  let mut idx: c_uint;

  block_input ();                                                 // C 717

  *(*font_info).glyphs.offset (block as isize) = unichars;        // C 719
  if unichars.is_null ()                                          // C 720
    { emacs_abort (); }                                           // C 721

  idx = (block as c_uint) << 8;                                   // C 723
  i = 0;
  while i < 0x100
    {
      *unichars.offset (i as isize) = idx as c_ushort;            // C 724
      idx += 1;
      i += 1;
    }
  *unichars.offset (0x100) = 0;                                   // C 725


  /* If the font contains the entire block, just store it.  */
  if BFont_have_char_block ((*font_info).be_font,
			    *unichars as i32, *unichars.offset (0xff) as i32) == 0  // C 729-730
    {
      let mut i: c_int = 0;                                       // C 732
      while i < 0x100
	{
	  if BFont_have_char_p ((*font_info).be_font,
				*unichars.offset (i as isize) as i32) == 0  // C 733
	    { *unichars.offset (i as isize) = 0xFFFF; }           // C 734
	  i += 1;
	}
    }

  unblock_input ();                                               // C 737
}

// C 740-754.
/// C: `static unsigned int haikufont_encode_char (struct font *font, int c);`
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_encode_char (font: *mut font, c: c_int) -> c_uint
{
  let font_info: *mut haikufont_info = font as *mut haikufont_info;  // C 743
  let high: c_uchar = ((c & 0xff00) >> 8) as c_uchar;             // C 744
  let low: c_uchar = (c & 0x00ff) as c_uchar;
  let g: c_ushort;                                                // C 745

  if c > 0xFFFF                                                   // C 747
    { return FONT_INVALID_CODE; }                                 // C 748

  if (*((*font_info).glyphs.offset (high as isize))).is_null ()   // C 750
    { haiku_bulk_encode (font_info, high as c_int); }             // C 751
  g = *((*(*font_info).glyphs.offset (high as isize))).offset (low as isize);  // C 752
       // = `font_info->glyphs[high][low]`: the outer `*` has the postfix chain as its operand.
  return if g == 0xFFFF { FONT_INVALID_CODE } else { g as c_uint };  // C 753
}

// C 756-881.  C's `pixel_size` parameter is reassigned (C 768-780), its nine `int` outputs
// (C 764-766) are written by `BFont_metrics` before any read, and `struct haiku_font_pattern ptn`
// is filled only on the C 806 arm - the port states the zero values C's own declarations leave,
// and the `font.h` window-system members C 843-850/860-874 write are the ones the mirror carries
// behind `have_window_system` (a Haiku configuration always has it: `configure.ac` 3113 sets
// `window_system=haiku` and 7518 then `AC_DEFINE`s `HAVE_WINDOW_SYSTEM`).
/// C: `static Lisp_Object haikufont_open (struct frame *f, Lisp_Object font_entity, int
/// pixel_size);` (untyped frame, see haikufont_match; this is the one of the four that reads it,
/// so the cast P-72 (3) prescribes is at its use sites below).
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_open (f: *mut c_void, font_entity: Lisp_Object,
				     pixel_size: c_int) -> Lisp_Object
{
  let mut font_info: *mut haikufont_info;                         // C 759
  let mut ptn: haiku_font_pattern = std::mem::zeroed ();          // C 760
  let mut font: *mut font;                                        // C 761
  let mut be_font: *mut c_void = std::ptr::null_mut ();           // C 762
  let mut font_object: Lisp_Object;                               // C 763
  let mut extra: Lisp_Object;
  let mut indices: Lisp_Object;
  let mut antialias: Lisp_Object;
  let mut px_size: c_int = 0;                                     // C 764
  let mut min_width: c_int = 0;
  let mut max_width: c_int = 0;
  let mut avg_width: c_int = 0;                                   // C 765
  let mut height: c_int = 0;
  let mut space_width: c_int = 0;
  let mut ascent: c_int = 0;
  let mut descent: c_int = 0;                                     // C 766
  let mut underline_pos: c_int = 0;
  let mut underline_thickness: c_int = 0;
  let mut pixel_size = pixel_size;   // C 768-780 reassigns the parameter

  if XFIXNUM (AREF (font_entity, FONT_SIZE_INDEX as isize)) != 0  // C 768
    { pixel_size = XFIXNUM (AREF (font_entity, FONT_SIZE_INDEX as isize)) as c_int; }  // C 769
  else if pixel_size == 0                                         // C 770
    {
      /* Try to resolve a suitable size for the font, if the font size
	 has not already been specified.  First, if FRAME_FONT is set,
	 use its size.  Otherwise, use 12, which is the default on
	 Haiku.  */

      if !FRAME_FONT! ((f as *mut frame)).is_null ()              // C 777
	{ pixel_size = (*FRAME_FONT! ((f as *mut frame))).pixel_size; }  // C 778
      else
	{ pixel_size = 12; }                                      // C 780
    }

  extra = AREF (font_entity, FONT_EXTRA_INDEX as isize);          // C 783

  indices = assq_no_quit (QCindices (), extra);                   // C 785
  antialias = assq_no_quit (QCantialias (), extra);               // C 786

  if CONSP (indices)                                              // C 788
    { indices = XCDR (indices); }                                 // C 789

  /* If the font's indices is already available, open the font using
     those instead.  */

  if CONSP (indices) && FIXNUMP (XCAR (indices))
      && FIXNUMP (XCDR (indices))                                 // C 794-795
    {
      block_input ();                                             // C 797
      be_font = be_open_font_at_index (XFIXNUM (XCAR (indices)) as c_int,
				       XFIXNUM (XCDR (indices)) as c_int,
				       pixel_size as f32);        // C 798-800
      unblock_input ();                                           // C 801

      if be_font.is_null ()                                       // C 803
	{ return Qnil (); }                                       // C 804
    }
  else
    {
      block_input ();                                             // C 808
      haikufont_spec_or_entity_to_pattern (font_entity, 1, &mut ptn);  // C 809

      if BFont_open_pattern (&mut ptn, &mut be_font, pixel_size as f32) != 0  // C 811
	{
	  haikufont_done_with_query_pattern (&mut ptn);            // C 813
	  unblock_input ();                                       // C 814
	  return Qnil ();                                         // C 815
	}

      haikufont_done_with_query_pattern (&mut ptn);               // C 818
      unblock_input ();                                           // C 819
    }

  block_input ();                                                 // C 822

  font_object = font_make_object (VECSIZE (std::mem::size_of::<haikufont_info> ()) as c_int,
				  font_entity, pixel_size);       // C 824-825

  ASET (font_object, FONT_TYPE_INDEX as isize, Qhaiku ());        // C 827
  font_info = XFONT_OBJECT (font_object) as *mut haikufont_info;  // C 828
  font = font_info as *mut font;                                  // C 829

  if font.is_null ()                                              // C 831
    {
      unblock_input ();                                           // C 833
      return Qnil ();                                             // C 834
    }

  (*font_info).be_font = be_font;                                 // C 837
  (*font_info).glyphs
    = xzalloc (0x100 * std::mem::size_of::<*mut c_ushort> ()) as *mut *mut c_ushort;  // C 838

  if CONSP (antialias)                                            // C 840
    { be_set_font_antialiasing (be_font, !NILP (XCDR (antialias))); }  // C 841

  (*font).pixel_size = 0;                                         // C 843
  (*font).driver = &raw const haikufont_driver as *const c_void;  // C 844
  (*font).encoding_charset = -1;                                  // C 845
  (*font).repertory_charset = -1;                                 // C 846
  (*font).default_ascent = 0;                                     // C 847
  (*font).vertical_centering = false;                             // C 848
  (*font).baseline_offset = 0;                                    // C 849
  (*font).relative_compose = 0;                                   // C 850

  (*font_info).metrics = std::ptr::null_mut ();                   // C 852
  (*font_info).metrics_nrows = 0;                                 // C 853

  BFont_metrics (be_font, &mut px_size, &mut min_width,
		 &mut max_width, &mut avg_width, &mut height,
		 &mut space_width, &mut ascent, &mut descent,
		 &mut underline_pos, &mut underline_thickness);   // C 855-858

  (*font).pixel_size = px_size;                                   // C 860
  (*font).min_width = min_width;                                  // C 861
  (*font).max_width = max_width;                                  // C 862
  (*font).average_width = avg_width;                              // C 863
  (*font).height = height;                                        // C 864
  (*font).space_width = space_width;                              // C 865
  (*font).ascent = ascent;                                        // C 866
  (*font).descent = descent;                                      // C 867
  (*font).default_ascent = ascent;                                // C 868
  (*font).underline_position = underline_pos;                     // C 869
  (*font).underline_thickness = underline_thickness;              // C 870

  (*font).vertical_centering = false;                             // C 872
  (*font).baseline_offset = 0;                                    // C 873
  (*font).relative_compose = 0;                                   // C 874

  (*font).props[FONT_NAME_INDEX as usize]
    = Ffont_xlfd_name (font_object, Qnil (), Qt ());              // C 876-877

  unblock_input ();                                               // C 879
  return font_object;                                             // C 880
}

// C 883-920.
/// C: `static void haikufont_close (struct font *font);`
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_close (font: *mut font)
{
  let info: *mut haikufont_info = font as *mut haikufont_info;    // C 886
  let mut i: c_int;                                               // C 887

  if font_data_structures_may_be_ill_formed ()                    // C 889
    { return; }                                                   // C 890

  block_input ();                                                 // C 892
  if !(*info).be_font.is_null ()                                  // C 893
    { BFont_close ((*info).be_font); }                            // C 894

  if !(*info).metrics.is_null ()                                  // C 896
    {
      i = 0;                                                      // C 898
      while i < (*info).metrics_nrows as c_int
	{
	  if !(*(*info).metrics.offset (i as isize)).is_null ()   // C 900
	    { xfree (*(*info).metrics.offset (i as isize)); }     // C 901
	  i += 1;
	}
      xfree ((*info).metrics as *mut c_void);                     // C 903
    }

  if !(*info).glyphs.is_null ()                                   // C 906
    {
      i = 0;                                                      // C 908
      while i < 0x100
	{
	  if !(*(*info).glyphs.offset (i as isize)).is_null ()    // C 910
	    { xfree (*(*info).glyphs.offset (i as isize) as *mut c_void); }  // C 911
	  i += 1;
	}
      xfree ((*info).glyphs as *mut c_void);                      // C 913
    }

  (*info).metrics = std::ptr::null_mut ();                        // C 916
  (*info).glyphs = std::ptr::null_mut ();                         // C 917
  (*info).be_font = std::ptr::null_mut ();                        // C 918
  unblock_input ();                                               // C 919
}

// C 922-926.  C's body is empty; the port carries the same empty body (it is not a stub - the
// reference has nothing to port here).
/// C: `static void haikufont_prepare_face (struct frame *f, struct face *face);` (both parameters
/// are the `font_driver` table's untyped pointers; P-72 (3)).
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_prepare_face (f: *mut c_void, face: *mut c_void)
{

}

// C 928-979.  `info->metrics` is `struct font_metrics **` in C and `*mut *mut c_void` in
// `include/haikuterm.rs` 46 (the P-72 (3) untyped member: `struct font_metrics` is
// `include/font.rs`'s record, which landed after that mirror) - so the two places this function
// needs the element type (C 960's `+ col`, and the C 970-974 stores through `cache`) cast once,
// and `xrealloc`/`memset` keep C's pointer arithmetic, which the element size makes identical.
/// C: `static void haikufont_glyph_extents (struct font *font, unsigned code,
/// struct font_metrics *metrics);`
#[cfg (feature = "have_haiku")]
unsafe fn haikufont_glyph_extents (font: *mut font, code: c_uint,
				   metrics: *mut font_metrics)
{
  let info: *mut haikufont_info = font as *mut haikufont_info;    // C 932

  let mut cache: *mut font_metrics;                               // C 934
  let mut row: c_int;                                             // C 935
  let mut col: c_int;

  row = (code / METRICS_NCOLS_PER_ROW as c_uint) as c_int;        // C 937
  col = (code % METRICS_NCOLS_PER_ROW as c_uint) as c_int;        // C 938
  if row >= (*info).metrics_nrows as c_int                        // C 939
    {
      (*info).metrics
	= xrealloc ((*info).metrics as *mut c_void,
		    std::mem::size_of::<*mut font_metrics> () * (row + 1) as usize)
	    as *mut *mut c_void;                                  // C 941-943
      memset ((*info).metrics.offset ((*info).metrics_nrows as isize) as *mut c_void, 0,
	      (std::mem::size_of::<*mut font_metrics> () as c_int
	       * (row + 1 - (*info).metrics_nrows as c_int)) as usize);  // C 944-946
      (*info).metrics_nrows = (row + 1) as c_short;               // C 947
    }

  if (*(*info).metrics.offset (row as isize)).is_null ()          // C 950
    {
      let new: *mut font_metrics;                                 // C 952
      let mut i: c_int;                                           // C 953

      new = xmalloc (std::mem::size_of::<font_metrics> ()
		     * METRICS_NCOLS_PER_ROW as usize) as *mut font_metrics;  // C 955
      i = 0;                                                      // C 956
      while i < METRICS_NCOLS_PER_ROW
	{
	  METRICS_SET_STATUS! (new.offset (i as isize), METRICS_INVALID);  // C 957
	  i += 1;
	}
      *(*info).metrics.offset (row as isize) = new as *mut c_void;  // C 958
    }
  cache = ((*(*info).metrics.offset (row as isize)) as *mut font_metrics)
	    .offset (col as isize);                               // C 960

  if METRICS_STATUS! (cache) as c_int == METRICS_INVALID          // C 962
    {
      let mut utf8: [c_uchar; MAX_MULTIBYTE_LENGTH as usize]
	= [0; MAX_MULTIBYTE_LENGTH as usize];                     // C 964
      memset (utf8.as_mut_ptr () as *mut c_void, 0, MAX_MULTIBYTE_LENGTH as usize);  // C 965
      CHAR_STRING (code as c_int, utf8.as_mut_ptr ());            // C 966
      let mut advance: c_int = 0;                                 // C 967
      let mut lb: c_int = 0;
      let mut rb: c_int = 0;
      BFont_char_bounds ((*info).be_font, utf8.as_ptr () as *const c_char,
			 &mut advance, &mut lb, &mut rb);         // C 968

      (*cache).lbearing = lb as c_short;                          // C 970
      (*cache).rbearing = rb as c_short;                          // C 971
      (*cache).width = advance as c_short;                        // C 972
      (*cache).ascent = (*font).ascent as c_short;                // C 973
      (*cache).descent = (*font).descent as c_short;              // C 974
    }

  if !metrics.is_null ()                                          // C 977
    { *metrics = std::ptr::read (cache); }                        // C 978
}

// C 981-1011.
/// C: `static void haikufont_text_extents (struct font *font, const unsigned int *code,
/// int nglyphs, struct font_metrics *metrics);`
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_text_extents (font: *mut font, code: *const c_uint,
					  nglyphs: c_int, metrics: *mut font_metrics)
{
  let mut totalwidth: c_int = 0;                                  // C 985
  memset (metrics as *mut c_void, 0, std::mem::size_of::<font_metrics> ());  // C 986

  block_input ();                                                 // C 988
  let mut i: c_int = 0;                                           // C 989
  while i < nglyphs
    {
      let mut m: font_metrics = std::mem::zeroed ();              // C 991
      haikufont_glyph_extents (font, *code.offset (i as isize), &mut m);  // C 992
      if !metrics.is_null ()                                      // C 993
	{
	  if totalwidth + (m.lbearing as c_int) < ((*metrics).lbearing as c_int)  // C 995
	    { (*metrics).lbearing = (totalwidth + m.lbearing as c_int) as c_short; }  // C 996
	  if totalwidth + (m.rbearing as c_int) > ((*metrics).rbearing as c_int)  // C 997
	    { (*metrics).rbearing = (totalwidth + m.rbearing as c_int) as c_short; }  // C 998
	  if (m.ascent as c_int) > ((*metrics).ascent as c_int)   // C 999
	    { (*metrics).ascent = m.ascent; }                     // C 1000
	  if (m.descent as c_int) > ((*metrics).descent as c_int)  // C 1001
	    { (*metrics).descent = m.descent; }                   // C 1002
	}
      totalwidth += m.width as c_int;                             // C 1004
      i += 1;
    }

  unblock_input ();                                               // C 1007

  if !metrics.is_null ()                                          // C 1009
    { (*metrics).width = totalwidth as c_short; }                 // C 1010
}

// C 1013-1090.  C's three `alloca` calls (C 1042/1058-1060) are the port's stack pool
// (PORTING.md P-05, the `src/hbfont.rs` 1310 shape); the `b_len = (b_len + slen)` assignment
// inside C 1051's call is a block expression, and the `uint32_t`/`int`/`ptrdiff_t` conversions are
// C's own.
/// C: `static Lisp_Object haikufont_shape (Lisp_Object lgstring, Lisp_Object direction);`
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_shape (lgstring: Lisp_Object, direction: Lisp_Object)
	-> Lisp_Object
{
  let font: *mut haikufont_info
    = CHECK_FONT_GET_OBJECT (LGSTRING_FONT (lgstring)) as *mut haikufont_info;  // C 1016-1017
  let mut advance: *mut c_int;                                    // C 1018
  let mut lb: *mut c_int;
  let mut rb: *mut c_int;
  let mut glyph_len: ptrdiff_t;                                   // C 1019
  let mut len: ptrdiff_t;
  let mut i: ptrdiff_t;
  let mut b_len: ptrdiff_t;
  let mut tem: Lisp_Object;                                       // C 1020
  let mut b: *mut c_char;                                         // C 1021
  let mut mb_buf: *mut uint32_t;                                  // C 1022

  glyph_len = LGSTRING_GLYPH_LEN (lgstring);                      // C 1024
  i = 0;                                                          // C 1025
  while i < glyph_len
    {
      tem = LGSTRING_GLYPH (lgstring, i);                         // C 1027

      if NILP (tem)                                               // C 1029
	{ break; }                                                // C 1030
      i += 1;
    }

  len = i;                                                        // C 1033

  if ((INT_MAX / 2) as ptrdiff_t) < len                          // C 1035
    { memory_full_up (); }                                        // C 1036

  block_input ();                                                 // C 1038

  b_len = 0;                                                      // C 1040
  b = xmalloc (b_len as usize) as *mut c_char;                    // C 1041
  mb_buf = sa_stack_alloc (len as usize * std::mem::size_of::<uint32_t> ()) as *mut uint32_t;  // C 1042

  i = b_len;                                                      // C 1044
  while i < len
    {
      let c: uint32_t = LGLYPH_CHAR (LGSTRING_GLYPH (lgstring, i)) as uint32_t;  // C 1046
      *mb_buf.offset (i as isize) = c;                            // C 1047
      let mut mb: [c_uchar; MAX_MULTIBYTE_LENGTH as usize]
	= [0; MAX_MULTIBYTE_LENGTH as usize];                     // C 1048
      let slen: c_int = CHAR_STRING (c as c_int, mb.as_mut_ptr ());  // C 1049

      b = xrealloc (b as *mut c_void,
		    { b_len = b_len + slen as ptrdiff_t; b_len as usize })
	    as *mut c_char;                                       // C 1051
      if len == 1                                                 // C 1052
	{ *b.offset (b_len - slen as ptrdiff_t) = mb[0] as c_char; }  // C 1053
      else
	{ memcpy (b.offset (b_len - slen as ptrdiff_t) as *mut c_void,
		  mb.as_ptr () as *const c_void, slen as usize); }  // C 1055
      i += 1;
    }

  advance = sa_stack_alloc (len as usize * std::mem::size_of::<c_int> ()) as *mut c_int;  // C 1058
  lb = sa_stack_alloc (len as usize * std::mem::size_of::<c_int> ()) as *mut c_int;  // C 1059
  rb = sa_stack_alloc (len as usize * std::mem::size_of::<c_int> ()) as *mut c_int;  // C 1060

  eassert! (!(*font).be_font.is_null ());                         // C 1062
  BFont_nchar_bounds ((*font).be_font, b, advance, lb, rb, len as i32);  // C 1063
  xfree (b as *mut c_void);                                       // C 1064

  i = 0;                                                          // C 1066
  while i < len
    {
      tem = LGSTRING_GLYPH (lgstring, i);                         // C 1068
      if NILP (tem)                                               // C 1069
	{
	  tem = LGLYPH_NEW ();                                    // C 1071
	  LGSTRING_SET_GLYPH (lgstring, i, tem);                  // C 1072
	}

      LGLYPH_SET_FROM (tem, i);                                   // C 1075
      LGLYPH_SET_TO (tem, i);                                     // C 1076
      LGLYPH_SET_CHAR (tem, *mb_buf.offset (i as isize) as EMACS_INT);  // C 1077
      LGLYPH_SET_CODE! (tem, *mb_buf.offset (i as isize));         // C 1078

      LGLYPH_SET_WIDTH (tem, *advance.offset (i as isize) as EMACS_INT);  // C 1080
      LGLYPH_SET_LBEARING (tem, *lb.offset (i as isize) as EMACS_INT);  // C 1081
      LGLYPH_SET_RBEARING (tem, *rb.offset (i as isize) as EMACS_INT);  // C 1082
      LGLYPH_SET_ASCENT (tem, (*font).font.ascent as EMACS_INT);  // C 1083
      LGLYPH_SET_DESCENT (tem, (*font).font.descent as EMACS_INT);  // C 1084
      i += 1;
    }

  unblock_input ();                                               // C 1087

  return make_fixnum (len);                                       // C 1089
}

/* C's `s->first_glyph->u.glyphless.method` (C 1130-1131) is the mirror's packed `bits` field; the
   port reads it through a macro, the `src/w32font.rs` 1353 / `src/w32term.rs` 1733 spelling
   (P-80).  */
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

// C 1092-1170.  C's `struct face *face = s->face` and `(struct font_info *) s->font` are the two
// untyped members of `struct glyph_string` (`*mut c_void`, P-72 (3)): the port keeps C's casts,
// the first to this file's use-site `face` record and the second to `*mut c_void` (C reads only
// `((struct haikufont_info *) info)->be_font`, C 1142).  C 1152's bare `alloca` is the port's
// stack pool (P-05, the `src/w32font.rs` 1478 shape) and C 1157's `b_len += len` is C's own
// `ptrdiff_t`/`int` conversion.
/// C: `static int haikufont_draw (struct glyph_string *s, int from, int to, int x, int y,
/// bool with_background);`
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_draw (s: *mut glyph_string, from: c_int, to: c_int,
				     x: c_int, y: c_int, with_background: bool) -> c_int
{
  let f: *mut frame = (*s).f;                                     // C 1096
  let face: *mut c_void = (*s).face;                              // C 1097
  let info: *mut c_void = (*s).font as *mut c_void;               // C 1098
  let mut mb: [c_uchar; MAX_MULTIBYTE_LENGTH as usize]
    = [0; MAX_MULTIBYTE_LENGTH as usize];                         // C 1099
  let view: *mut c_void = FRAME_HAIKU_VIEW! (f);                  // C 1100
  let mut foreground: c_ulong = 0;                                // C 1101
  let mut background: c_ulong = 0;

  block_input ();                                                 // C 1103
  prepare_face_for_display ((*s).f, face);                        // C 1104

  if (*s).hl != DRAW_CURSOR                                       // C 1106
    {
      foreground = (*( (*s).face as *mut face )).foreground;      // C 1108
      background = (*( (*s).face as *mut face )).background;      // C 1109
    }
  else
    { haiku_merge_cursor_foreground (s as *mut c_void, &mut foreground,
				     &mut background); }  // C 1112

  /* Presumably the draw lock is already held by
     haiku_draw_glyph_string; */
  if with_background                                              // C 1116
    {
      let mut height: c_int = FONT_HEIGHT! ((*s).font);           // C 1118
      let mut ascent: c_int = FONT_BASE! ((*s).font);

      /* Font's global height and ascent values might be
	 preposterously large for some fonts.  We fix here the case
	 when those fonts are used for display of glyphless
	 characters, because drawing background with font dimensions
	 in those cases makes the display illegible.  There's only one
	 more call to the draw method with with_background set to
	 true, and that's in x_draw_glyph_string_foreground, when
	 drawing the cursor, where we have no such heuristics
	 available.  FIXME.  */
      if glyph_bits! (type, *(*s).first_glyph) == GLYPHLESS_GLYPH  // C 1129
	  && (glyphless_bits! (method, (*s).first_glyph) == dispextern_unit_names::GLYPHLESS_DISPLAY_HEX_CODE  // C 1130
	      || glyphless_bits! (method, (*s).first_glyph) == dispextern_unit_names::GLYPHLESS_DISPLAY_ACRONYM)  // C 1131
	/* C's chained assignment `height = ascent = <expr>`; a Rust assignment is `()`, so the
	   port states the two assignments C performs, in C's order.  */
	{ ascent = (*(*s).first_glyph).slice.glyphless.lower_yoff as c_int
	    - (*(*s).first_glyph).slice.glyphless.upper_yoff as c_int;
	  height = ascent; }  // C 1132-1134

      haiku_draw_background_rect (s as *mut c_void, (*s).face, x, y - ascent,
				  (*s).width, height);           // C 1136-1137
    }

  BView_SetHighColor (view, foreground as uint32_t);              // C 1140
  BView_MovePenTo (view, x, y);                                   // C 1141
  BView_SetFont (view, (*(info as *mut haikufont_info)).be_font);  // C 1142

  if from == to                                                   // C 1144
    {
      let len: c_int = CHAR_STRING (*(*s).char2b.offset (from as isize) as c_int,
				    mb.as_mut_ptr ());            // C 1146
      BView_DrawString (view, mb.as_ptr () as *const c_char, len as ptrdiff_t);  // C 1147
    }
  else
    {
      let mut b_len: ptrdiff_t = 0;                               // C 1151
      let b: *mut c_char
	= sa_stack_alloc ((to - from + 1) as usize * MAX_MULTIBYTE_LENGTH as usize)
	    as *mut c_char;                                       // C 1152

      let mut idx: c_int = from;                                  // C 1154
      while idx < to
	{
	  let len: c_int = CHAR_STRING (*(*s).char2b.offset (idx as isize) as c_int,
					mb.as_mut_ptr ());        // C 1156
	  b_len += len as ptrdiff_t;                                  // C 1157

	  if len == 1                                                 // C 1159
	    { *b.offset (b_len - len as ptrdiff_t) = mb[0] as c_char; }  // C 1160
	  else
	    { memcpy (b.offset (b_len - len as ptrdiff_t) as *mut c_void,
		      mb.as_ptr () as *const c_void, len as usize); }  // C 1162
	  idx += 1;
	}

      BView_DrawString (view, b, b_len);                          // C 1165
    }

  unblock_input ();                                               // C 1168
  return 1;                                                       // C 1169
}

// C 1172-1198.  C's `styles[idx][0]` and `(char *) &styles[idx]` are one array element of C's
// `haiku_font_family_or_style *` (the mirror's `*mut [c_char; 64]`), and `idx < length` compares
// C's `ptrdiff_t` with its `size_t`, which the port spells as the same unsigned comparison.
/// C: `static Lisp_Object haikufont_list_family (struct frame *f);` (untyped frame, see
/// haikufont_match).
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_list_family (f: *mut c_void) -> Lisp_Object
{
  let mut list: Lisp_Object = Qnil ();                            // C 1175
  let mut length: size_t = 0;                                     // C 1176
  let mut idx: ptrdiff_t;                                         // C 1177
  let mut styles: *mut haiku_font_family_or_style;                // C 1178

  block_input ();                                                 // C 1180
  styles = be_list_font_families (&mut length);                   // C 1181
  unblock_input ();                                               // C 1182

  if styles.is_null ()                                            // C 1184
    { return list; }                                              // C 1185

  block_input ();                                                 // C 1187
  idx = 0;                                                        // C 1188
  while (idx as usize) < length
    {
      if (*styles.offset (idx as isize))[0] != 0                  // C 1190
	{ list = Fcons (intern ((*styles.offset (idx as isize)).as_ptr ()), list); }  // C 1191
      idx += 1;
    }

  free (styles as *mut c_void);                                   // C 1194
  unblock_input ();                                               // C 1195

  return list;                                                    // C 1197
}

/* List of boolean properties in font names accepted by this font
   driver.  */
/// C 1202-1206: `static const char *const haikufont_booleans[] = { ":antialias", NULL, };`
#[cfg (feature = "have_haiku")]
static mut haikufont_booleans: [*const c_char; 2]
  = [c":antialias".as_ptr (), std::ptr::null ()];

/* List of non-boolean properties.  Currently empty.  */
/// C 1209: `static const char *const haikufont_non_booleans[1];` - C's tentative definition
/// zero-initialises, i.e. one null element.
#[cfg (feature = "have_haiku")]
static mut haikufont_non_booleans: [*const c_char; 1] = [std::ptr::null ()];

// C 1211-1216.
/// C: `static void haikufont_filter_properties (Lisp_Object font, Lisp_Object alist);`
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_filter_properties (font: Lisp_Object, alist: Lisp_Object)
{
  font_filter_properties (font, alist, haikufont_booleans.as_ptr (),
			  haikufont_non_booleans.as_ptr ());      // C 1214-1215
}

// C 1218-1234.  C's designated initialiser leaves the members it does not name zero, which the
// port writes out in the mirror's own declaration order; `draw`, `get_bitmap` and `free_bitmap`
// are the members C guards with HAVE_WINDOW_SYSTEM (font.h 682-698), and the two trailing ones
// are font.h 807-820's `#ifdef HAVE_HARFBUZZ` pair, which the mirror gates the same way (the
// `src/androidfont.rs` 1287-1336 shape).  `.type = LISPSYM_INITIALLY (Qhaiku)`:
// the mirror's `LISPSYM_INITIALLY` is not a `const fn`, so the port writes the same constant
// expression over the generated `iQhaiku` index.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut haikufont_driver: font_driver = font_driver {
  r#type: (((iQhaiku * std::mem::size_of::<Lisp_Symbol> ()) as EMACS_UINT
	    + Lisp_Type::Lisp_Symbol as EMACS_UINT) as Lisp_Object),  // C 1220
  case_sensitive: true,                                           // C 1221
  get_cache: Some (haikufont_get_cache),                          // C 1222
  list: Some (haikufont_list),                                    // C 1223
  r#match: Some (haikufont_match),                                // C 1224
  list_family: Some (haikufont_list_family),                      // C 1232
  free_entity: None,
  open_font: Some (haikufont_open),                               // C 1226
  close_font: Some (haikufont_close),                             // C 1227
  prepare_face: Some (haikufont_prepare_face),                    // C 1228
  done_face: None,
  has_char: None,
  encode_char: Some (haikufont_encode_char),                      // C 1229
  text_extents: Some (haikufont_text_extents),                    // C 1230
  #[cfg (feature = "have_window_system")]
  draw: Some (haikufont_draw),                                    // C 1225
  #[cfg (feature = "have_window_system")]
  get_bitmap: None,
  #[cfg (feature = "have_window_system")]
  free_bitmap: None,
  anchor_point: None,
  otf_capability: None,
  otf_drive: None,
  start_for_frame: None,
  end_for_frame: None,
  shape: Some (haikufont_shape),                                  // C 1231
  check: None,
  get_variation_glyphs: None,
  filter_properties: Some (haikufont_filter_properties),          // C 1233
  cached_font_ok: None,
  combining_capability: None,
  drop_xrender_surfaces: None,
  #[cfg (feature = "harfbuzz")]
  begin_hb_font: None,
  #[cfg (feature = "harfbuzz")]
  end_hb_font: None,
};

// C 1236-1240.
/// C: `static bool haikufont_should_quit_popup (void);`
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haikufont_should_quit_popup () -> bool
{
  return !NILP (Vquit_flag);                                      // C 1239
}

// C 1242-1328.  `initial_style`/`initial_family`/`initial_size`/`disable_antialiasing` are written
// before any read in every path (C 1266-1269), `family`/`style` are the two array locals
// `be_select_font` fills, and `bool`-valued `rc`/`use_antialiasing` conversions are C's own.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-select-font", Fx_select_font, Sx_select_font, 0, 2, 0,
       doc: "Read a font using a native dialog.\nReturn a font spec describing the font chosen by the user.\n\nFRAME is the frame on which to pop up the font chooser.  If omitted or\nnil, it defaults to the selected frame.\nIf EXCLUDE-PROPORTIONAL is non-nil, exclude proportional fonts\nin the font selection dialog.  ",
       (frame: Lisp_Object, exclude_proportional: Lisp_Object),
       {
  let f: *mut frame;                                              // C 1252
  let mut font: *mut font;                                        // C 1253
  let mut font_object: Lisp_Object;                               // C 1254
  let mut family: haiku_font_family_or_style = [0; 64];           // C 1255
  let mut style: haiku_font_family_or_style = [0; 64];
  let mut rc: c_int = 0;                                          // C 1256
  let mut size: c_int = 0;
  let mut initial_family: c_int = 0;
  let mut initial_style: c_int = 0;
  let mut initial_size: c_int = 0;
  let mut pattern: haiku_font_pattern = std::mem::zeroed ();       // C 1257
  let mut lfamily: Lisp_Object;                                   // C 1258
  let mut lweight: Lisp_Object;
  let mut lslant: Lisp_Object;
  let mut lwidth: Lisp_Object;
  let mut ladstyle: Lisp_Object;
  let mut lsize: Lisp_Object;
  let mut disable_antialiasing: bool = false;                     // C 1259
  let mut initial_antialias: bool = false;

  f = decode_window_system_frame (frame);                         // C 1261

  if popup_activated_p != 0                                       // C 1263
    { error (c"Trying to use a menu from within a menu-entry".as_ptr ()); }  // C 1264

  initial_style = -1;                                             // C 1266
  initial_family = -1;                                            // C 1267
  initial_size = -1;                                              // C 1268
  initial_antialias = true;                                       // C 1269

  font = FRAME_FONT! (f);                                         // C 1271

  if !font.is_null ()                                             // C 1273
    {
      XSETFONT! (font_object, font);                              // C 1275

      haikufont_pattern_from_object (&mut pattern, font_object);   // C 1277
      be_find_font_indices (&mut pattern, &mut initial_family,
			    &mut initial_style);                  // C 1278-1279
      haikufont_done_with_query_pattern (&mut pattern);            // C 1280

      initial_size = (*font).pixel_size;                           // C 1282

      /* This field is safe to access even after
	 haikufont_done_with_query_pattern.  */
      if (pattern.specified & FSPEC_ANTIALIAS) != 0                // C 1286
	{ initial_antialias = pattern.use_antialiasing != 0; }     // C 1287
    }

  popup_activated_p += 1;                                         // C 1290
  unrequest_sigio ();                                             // C 1291
  rc = be_select_font (Some (process_pending_signals),
		       Some (haikufont_should_quit_popup),
		       &mut family, &mut style, &mut size,
		       !NILP (exclude_proportional),
		       initial_family, initial_style,
		       initial_size, initial_antialias,
		       &mut disable_antialiasing) as c_int;        // C 1292-1298
  request_sigio ();                                               // C 1299
  popup_activated_p -= 1;                                         // C 1300

  if rc == 0                                                      // C 1302
    { quit (); }                                                  // C 1303

  be_font_style_to_flags (style.as_ptr (), &mut pattern);         // C 1305

  lfamily = build_string_from_utf8 (family.as_ptr ());            // C 1307
  lweight = if (pattern.specified & FSPEC_WEIGHT) != 0
	      { haikufont_weight_to_lisp (pattern.weight) } else { Qnil () };  // C 1308-1309
  lslant = if (pattern.specified & FSPEC_SLANT) != 0
	     { haikufont_slant_to_lisp (pattern.slant) } else { Qnil () };  // C 1310-1311
  lwidth = if (pattern.specified & FSPEC_WIDTH) != 0
	     { haikufont_width_to_lisp (pattern.width) } else { Qnil () };  // C 1312-1313
  ladstyle = if (pattern.specified & FSPEC_STYLE) != 0
	       { intern (pattern.style.as_ptr ()) } else { Qnil () };  // C 1314-1315
  lsize = if size >= 0 { make_fixnum (size as EMACS_INT) } else { Qnil () };  // C 1316

  if disable_antialiasing                                         // C 1318
    {
      return CALLN! (Ffont_spec, QCfamily (), lfamily,
		     QCweight (), lweight, QCslant (), lslant,
		     QCwidth (), lwidth, QCadstyle (), ladstyle,
		     QCsize (), lsize, QCantialias (), Qnil ());  // C 1319-1322
    }

  return CALLN! (Ffont_spec, QCfamily (), lfamily,
		 QCweight (), lweight, QCslant (), lslant,
		 QCwidth (), lwidth, QCadstyle (), ladstyle,
		 QCsize (), lsize);                              // C 1324-1327
});

// C 1330-1367.
#[cfg (feature = "have_haiku")]
DEFUN! ("font-get-system-normal-font", Ffont_get_system_normal_font,
       Sfont_get_system_normal_font, 0, 0, 0,
       doc: "SKIP: real doc in xsettings.c.  ",
       (),
       {
  let mut value: Lisp_Object;                                     // C 1335
  let mut name: *const c_char;                                    // C 1336
  let mut style: *const c_char;
  let mut pattern: haiku_font_pattern = std::mem::zeroed ();       // C 1337
  let mut lfamily: Lisp_Object;                                   // C 1338
  let mut lweight: Lisp_Object;
  let mut lslant: Lisp_Object;
  let mut lwidth: Lisp_Object;
  let mut ladstyle: Lisp_Object;
  let mut size: c_int = 0;                                        // C 1339

  if !haiku_support_abi::be_lock_font_defaults ()                 // C: if (!be_lock_font_defaults ())  (C 1341)
    { return Qnil (); }                                           // C 1342

  name = be_get_font_default (DEFAULT_FAMILY);                    // C 1344
  style = be_get_font_default (DEFAULT_STYLE);                    // C 1345
  size = be_get_font_size (DEFAULT_FAMILY);                       // C 1346

  be_font_style_to_flags (style, &mut pattern);                   // C 1348

  lfamily = build_string_from_utf8 (name);                        // C 1350
  lweight = if (pattern.specified & FSPEC_WEIGHT) != 0
	      { haikufont_weight_to_lisp (pattern.weight) } else { Qnil () };  // C 1351-1352
  lslant = if (pattern.specified & FSPEC_SLANT) != 0
	     { haikufont_slant_to_lisp (pattern.slant) } else { Qnil () };  // C 1353-1354
  lwidth = if (pattern.specified & FSPEC_WIDTH) != 0
	     { haikufont_width_to_lisp (pattern.width) } else { Qnil () };  // C 1355-1356
  ladstyle = if (pattern.specified & FSPEC_STYLE) != 0
	       { intern (pattern.style.as_ptr ()) } else { Qnil () };  // C 1357-1358

  value = CALLN! (Ffont_spec, QCfamily (), lfamily,
		  QCweight (), lweight, QCslant (), lslant,
		  QCwidth (), lwidth, QCadstyle (), ladstyle,
		  QCsize (), make_fixnum (size as EMACS_INT));   // C 1360-1363
  be_unlock_font_defaults ();                                     // C 1364

  return value;                                                   // C 1366
});

// C 1369-1406.
#[cfg (feature = "have_haiku")]
DEFUN! ("font-get-system-font", Ffont_get_system_font,
       Sfont_get_system_font, 0, 0, 0,
       doc: "SKIP: real doc in xsettings.c.  ",
       (),
       {
  let mut value: Lisp_Object;                                     // C 1374
  let mut name: *const c_char;                                    // C 1375
  let mut style: *const c_char;
  let mut pattern: haiku_font_pattern = std::mem::zeroed ();       // C 1376
  let mut lfamily: Lisp_Object;                                   // C 1377
  let mut lweight: Lisp_Object;
  let mut lslant: Lisp_Object;
  let mut lwidth: Lisp_Object;
  let mut ladstyle: Lisp_Object;
  let mut size: c_int = 0;                                        // C 1378

  if !haiku_support_abi::be_lock_font_defaults ()                 // C: if (!be_lock_font_defaults ())  (C 1380)
    { return Qnil (); }                                           // C 1381

  name = be_get_font_default (FIXED_FAMILY);                      // C 1383
  style = be_get_font_default (FIXED_STYLE);                      // C 1384
  size = be_get_font_size (FIXED_FAMILY);                         // C 1385

  be_font_style_to_flags (style, &mut pattern);                   // C 1387

  lfamily = build_string_from_utf8 (name);                        // C 1389
  lweight = if (pattern.specified & FSPEC_WEIGHT) != 0
	      { haikufont_weight_to_lisp (pattern.weight) } else { Qnil () };  // C 1390-1391
  lslant = if (pattern.specified & FSPEC_SLANT) != 0
	     { haikufont_slant_to_lisp (pattern.slant) } else { Qnil () };  // C 1392-1393
  lwidth = if (pattern.specified & FSPEC_WIDTH) != 0
	     { haikufont_width_to_lisp (pattern.width) } else { Qnil () };  // C 1394-1395
  ladstyle = if (pattern.specified & FSPEC_STYLE) != 0
	       { intern (pattern.style.as_ptr ()) } else { Qnil () };  // C 1396-1397

  value = CALLN! (Ffont_spec, QCfamily (), lfamily,
		 QCweight (), lweight, QCslant (), lslant,
		 QCwidth (), lwidth, QCadstyle (), ladstyle,
		 QCsize (), make_fixnum (size as EMACS_INT));   // C 1399-1402
  be_unlock_font_defaults ();                                     // C 1403

  return value;                                                   // C 1405
});

// C 1408-1420.  `ie->kind` is the mirror's `input_event::bits` unit, so C 1412's write goes
// through a unit-local `input_event_bits!` (P-80's member-named shape, the
// `src/haikuselect.rs` 1345 copy).
#[cfg (feature = "have_haiku")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
}

/// C: haikuterm.h 238: `void haiku_handle_font_change_event (struct haiku_font_change_event *,
/// struct input_event *);`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_handle_font_change_event (event: *mut haiku_font_change_event,
							 ie: *mut input_event)
{
  input_event_bits! (kind, ie, CONFIG_CHANGED_EVENT);             // C 1412

  /* This is the name of the display.  */
  (*ie).frame_or_window = XCAR ((*x_display_list).name_list_element);  // C 1415

  /* And this is the font that changed.  */
  (*ie).arg = if (*event).what == FIXED_FAMILY
		{ Qmonospace_font_name () } else { Qfont_name () };  // C 1418-1419
}

// C 1422-1426.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn syms_of_haikufont_for_pdumper ()
{
  register_font_driver (&raw const haikufont_driver, std::ptr::null_mut ());  // C 1425
}

// C 1428-1480.  The `DEFVAR_BOOL` variable is C's `globals.f_use_system_font`, so the port defines
// it here and `include/globals.rs` 7142's declaration is shadowed (P-21, the
// `src/haikufns.rs` 4149-4175 shape); `DEFSYM` is empty in the reference as well (P-17).
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut use_system_font: bool = false;

#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_haikufont ()
{
  DEFSYM! (QSans_Serif, "Sans Serif");                            // C 1431
  DEFSYM! (Qfontsize, "fontsize");                                // C 1432
  DEFSYM! (Qfixed, "fixed");                                      // C 1433
  DEFSYM! (Qplain, "plain");                                      // C 1434
  DEFSYM! (Qultra_light, "ultra-light");                          // C 1435
  DEFSYM! (Qthin, "thin");                                        // C 1436
  DEFSYM! (Qreverse_italic, "reverse-italic");                    // C 1437
  DEFSYM! (Qreverse_oblique, "reverse-oblique");                   // C 1438
  DEFSYM! (Qmonospace, "monospace");                              // C 1439
  DEFSYM! (Qultra_condensed, "ultra-condensed");                  // C 1440
  DEFSYM! (Qextra_condensed, "extra-condensed");                  // C 1441
  DEFSYM! (Qcondensed, "condensed");                              // C 1442
  DEFSYM! (Qsemi_condensed, "semi-condensed");                    // C 1443
  DEFSYM! (Qsemi_expanded, "semi-expanded");                      // C 1444
  DEFSYM! (Qexpanded, "expanded");                                // C 1445
  DEFSYM! (Qextra_expanded, "extra-expanded");                    // C 1446
  DEFSYM! (Qultra_expanded, "ultra-expanded");                    // C 1447
  DEFSYM! (Qregular, "regular");                                  // C 1448
  DEFSYM! (Qzh, "zh");                                            // C 1449
  DEFSYM! (Qko, "ko");                                            // C 1450
  DEFSYM! (Qjp, "jp");                                            // C 1451

  DEFSYM! (QCindices, ":indices");                                // C 1453

  DEFSYM! (Qmonospace_font_name, "monospace-font-name");           // C 1455
  DEFSYM! (Qfont_name, "font-name");                              // C 1456
  DEFSYM! (Qdynamic_setting, "dynamic-setting");                   // C 1457

  DEFVAR_BOOL! (c"font-use-system-font", use_system_font,
		"SKIP: real doc in xsettings.c.  ");              // C 1459-1460
  use_system_font = false;                                        // C 1461

  // C 1463-1465: `#ifdef USE_BE_CAIRO` - the `be_cairo` feature, whose arm this target does not
  // take (`tools/check-cond-eval.py src/haikufont.c`: `1 conditions, 0 unknown, 0 disagreements`).
  #[cfg (feature = "be_cairo")]
  { Fput (Qhaiku (), Qfont_driver_superseded_by (), Qftcr ()); }   // C 1464

  pdumper_do_now_and_after_load (Some (syms_of_haikufont_for_pdumper));  // C 1466

  font_cache = list! (Qnil ());                                   // C 1468
  staticpro (&raw mut font_cache);                                // C 1469

  defsubr (&raw mut Sx_select_font);                              // C 1471
  defsubr (&raw mut Sfont_get_system_normal_font);                // C 1472
  defsubr (&raw mut Sfont_get_system_font);                       // C 1473

  be_init_font_data ();                                           // C 1475

  /* This tells loadup to load dynamic-setting.el, which handles
     config-changed events.  */
  Fprovide (Qdynamic_setting (), Qnil ());                        // C 1479
}
