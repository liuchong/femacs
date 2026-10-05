// femacs: port of src/sfntfont.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct field, macro, file-scope object, local), the same control flow and the same error model.
// The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static`
// helpers stay plain `unsafe fn` except the two whose address C takes, which become
// `unsafe extern "C" fn` items (PORTING.md 1.4; the androidselect.rs `close_asset_fd` shape).
//
// **This file is P-86's unit `src/sfntfont.c` (4,222 lines), ported in slices.**  This slice is
// C 1-1240 (through the `}` of `sfnt_enum_font`); the next live definition is
// `sfntfont_charset_for_name` (C 1246-1267, under C 1244's `/* Font discovery and matching.  */`
// banner).  The ledger `docs/ports/sfntfont.md` carries P-86 ②'s C line-range table for C 1-1240
// plus the "the remainder" row, the `## Evidence` readings and the `## Deviations`.
//
// `configure.ac` 2645 adds `sfntfont.o` to `ANDROID_OBJ` only inside the `XCONFIGURE = android`
// branch (the `REALLY_ANDROID` one, 2620-2654: `sfnt.o` is on the same line and `android.o` one
// line earlier, at 2598), and `docs/baseline/frozen-build-sources.txt` lists no `sfntfont.o`, so
// the frozen configuration compiles nothing here.  Unlike the 11-line placeholder, though, this
// slice *does* have `fn`s, and `src/sfntfont.c` is in
// `docs/baseline/config/cond-eval-verified.txt` (`python3 tools/check-cond-eval.py src/sfntfont.c`
// reads `27 conditions, 0 unknown, 0 disagreements [VERIFIED]`), so `extract-abi.py --check` is a
// live coverage statement about the file here rather than a vacuous one (the `src/sfnt.c` reading).
// The gates, in C's order:
//
//   * the file gate (`HAVE_ANDROID && !defined ANDROID_STUBIFY`, i.e. `REALLY_ANDROID`) is
//     `all (feature = "have_android", feature = "not_android_stubify")` - on every item and on
//     C 22-42's include graph, which is why the frozen configuration compiles nothing here
//     (P-86 ⑧, the `sfnt.rs`/`androidselect.rs` shape for `configure.ac` 2645/2647-2648).
//     `not_android_stubify` is the feature the tree registered for that C condition
//     (`docs/baseline/feature-coverage.tsv` 640, first used by `alloc.rs` 6428).
//   * C 36-39's `#ifdef HAVE_HARFBUZZ` is `feature = "harfbuzz"` (the registered name,
//     `femacs/xtask/Cargo.toml` 93): dead on this target (`HAVE_HARFBUZZ` is `undef` in
//     `docs/baseline/config/darwin-arm64.macros.tsv`).  No name in C 1-1240 comes from
//     `<hb.h>`/`<hb-ot.h>`, so the pair is carried as a comment and the P-88 use-site
//     declaration segment is owed with the code that reads it (`hb_blob_t`/`hb_face_t`/
//     `hb_font_t` first appear around C 3989).
//   * C 42's `#include TERM_HEADER` (`configure.ac` 7180 chooses it) is `androidterm.h` in exactly
//     the configuration the file gate selects (`window_system=android`, 2594-2596), so the arm is
//     one gated `include!` of this tree's mirror (the `image.rs` 139-154 shape).
//   * C 85-88's `#ifdef HAVE_MMAP` carries **no gate**: `HAVE_MMAP` is 1 in
//     `docs/baseline/config/darwin-arm64.macros.tsv` (1168) and the condition has no `#else` arm,
//     so the live member `glyf_table_mapped` is ordinary text - the `src/sfnt.c` 41-43 ruling,
//     whose reading the clang probe below repeats.
//   * C 90-96's `#ifdef HAVE_HARFBUZZ` member pair (`fd`, `directory`) is the same
//     `feature = "harfbuzz"`.
//
// The layout of this file's three records is measured with the frozen clang (the probe and its
// readings are in the ledger's `## Evidence`) and pinned with `core::assert!` (PORTING.md 1.3).
//
// Function-extern gaps the mirrors do not carry are declared at their use sites with the
// reference header's line (P-72 rule 3 / P-55): `setup_coding_system` (coding.h 686),
// `internal_condition_case` (lisp.h 4805) and `strtok_r` (`string.in.h` 1224-1227).  The ledger
// names each, and the token-alias/macro-body-notes rows this slice owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                       // C 20: `#include <config.h>`

/* C 22-42: the file's own include list, in C's order.  It sits under the file gate alone (there
   is no `#if` around it in C), so no translation unit the frozen configuration builds reaches
   any of it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/fcntl.in.rs");              // C 22: `#include <fcntl.h>` (O_RDONLY)
/* C 22's gnulib `<fcntl.h>` pulls `<unistd.h>` in (`clang -H` on this file shows
   `build/emacs-ref/lib/unistd.h` one level under it), which is where `lseek`, `read` and
   `SEEK_SET` come from.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/unistd.in.rs");             // C 22's `<fcntl.h>` -> `<unistd.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/c-ctype.rs");               // C 23: `#include <c-ctype.h>` (c_toupper)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/lisp.rs");                         // C 25: `#include "lisp.h"`
/* lisp.h is where this unit gets `<string.h>` (memcpy, memset, strlen, strcmp, memmem) and
   `<stdcountof.h>` (countof); `include/lisp.rs` carries neither mirror, so the two edges are
   here, exactly as C's `#include "lisp.h"` supplies them.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/string.rs");                // lisp.h's `<string.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdcountof.rs");            // lisp.h's `<stdcountof.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/blockinput.rs");                   // C 27: `#include "blockinput.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/charset.rs");                      // C 28: `#include "charset.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/coding.rs");                       // C 29: `#include "coding.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/font.rs");                         // C 30: `#include "font.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/frame.rs");                        // C 31: `#include "frame.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/math.rs");                  // C 32: `#include "math.h"` (the system one)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/sfnt.rs");                         // C 33: `#include "sfnt.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/sfntfont.rs");                     // C 34: `#include "sfntfont.h"`

/* C 36-39: `#ifdef HAVE_HARFBUZZ` / `#include <hb.h>` / `#include <hb-ot.h>` / `#endif`.
   `HAVE_HARFBUZZ` is `undef` on this target, and the two headers are not in the reference tree
   (P-88's class), so the arm is this comment; C 1-1240 takes no name from either header.  */
/* C 37-38: `#include <hb.h>` and `#include <hb-ot.h>` (inside C 36's `#ifdef HAVE_HARFBUZZ`) -
   no mirror in this tree and no name used in C 1-1240, so the P-88 declaration segment is owed
   with the code that names an `hb_*` type (the first is C 3989's `hb_blob_t`).  */

/* C 42: `#include TERM_HEADER`.  `TERM_HEADER` is `androidterm.h` in the configuration the file
   gate selects (configure.ac 7180 with `window_system=android` at 2594-2596).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/androidterm.rs");                   // C 42: `#include TERM_HEADER`

/* --------------------------------------------------------------------------------------------
   C 44-49's banner, verbatim (the file's own text; the port keeps the C comment).
   -------------------------------------------------------------------------------------------- */
/* Generic font driver for sfnt-based fonts (currently TrueType, but
   it would be easy to add CFF support in the future with a PostScript
   renderer.)

   This is not a complete font driver.  Hooks must be supplied by the
   platform implementer to draw glyphs.  */

/* Tables associated with each font, be it distortable or not.  This
   allows different font objects sharing the same underlying font file
   to share tables.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct sfnt_font_tables                       // C 57-97
{
  /* Various tables required to use the font.  */
  pub cmap: *mut sfnt_cmap_table,                 // C 60
  pub hhea: *mut sfnt_hhea_table,                 // C 61
  pub maxp: *mut sfnt_maxp_table,                 // C 62
  pub head: *mut sfnt_head_table,                 // C 63
  pub hmtx: *mut sfnt_hmtx_table,                 // C 64
  pub glyf: *mut sfnt_glyf_table,                 // C 65
  pub loca_short: *mut sfnt_loca_table_short,     // C 66
  pub loca_long: *mut sfnt_loca_table_long,       // C 67
  pub prep: *mut sfnt_prep_table,                 // C 68
  pub fpgm: *mut sfnt_fpgm_table,                 // C 69
  pub cvt: *mut sfnt_cvt_table,                   // C 70
  pub fvar: *mut sfnt_fvar_table,                 // C 71
  pub avar: *mut sfnt_avar_table,                 // C 72
  pub gvar: *mut sfnt_gvar_table,                 // C 73
  pub cvar: *mut sfnt_cvar_table,                 // C 74

  /* The selected character map.  */
  pub cmap_data: *mut sfnt_cmap_encoding_subtable_data,  // C 77

  /* Data identifying that character map.  */
  pub cmap_subtable: sfnt_cmap_encoding_subtable, // C 80

  /* The UVS context.  */
  pub uvs: *mut sfnt_uvs_context,                 // C 83

  /* C 85-88: `#ifdef HAVE_MMAP` - live on this target (`HAVE_MMAP` is 1) and with no `#else`
     arm, so it is carried as ordinary text (the `sfnt.rs` C 41-43 ruling).  */
  /* Whether or not the glyph table has been mmapped.  */
  pub glyf_table_mapped: bool,                    // C 87

  /* C 90-96: `#ifdef HAVE_HARFBUZZ` - dead on this target, carried under the feature that
     names the C condition (P-51).  */
  /* File descriptor associated with this font.  */
  #[cfg (feature = "harfbuzz")]
  pub fd: c_int,                                  // C 92

  /* The table directory of the font file.  */
  #[cfg (feature = "harfbuzz")]
  pub directory: *mut sfnt_offset_subtable,       // C 95
}

/* clang, on the frozen configuration (`HAVE_MMAP` 1, `HAVE_HARFBUZZ` undef), reads
   `sizeof 152`, `_Alignof 8`, and these offsets; the ledger carries the probe.  The two
   `harfbuzz` members are outside that reading, so the assertions are the frozen record's.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
            not (feature = "harfbuzz")))]
const _: () = {
  core::assert! (std::mem::size_of::<sfnt_font_tables> () == 152);
  core::assert! (std::mem::align_of::<sfnt_font_tables> () == 8);
  core::assert! (std::mem::offset_of! (sfnt_font_tables, cmap) == 0);
  core::assert! (std::mem::offset_of! (sfnt_font_tables, cvar) == 112);
  core::assert! (std::mem::offset_of! (sfnt_font_tables, cmap_data) == 120);
  core::assert! (std::mem::offset_of! (sfnt_font_tables, cmap_subtable) == 128);
  core::assert! (std::mem::offset_of! (sfnt_font_tables, uvs) == 136);
  core::assert! (std::mem::offset_of! (sfnt_font_tables, glyf_table_mapped) == 144);
};

/* Description of a font that hasn't been opened.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct sfnt_font_desc                         // C 101-165
{
  /* Next font in this list.  */
  pub next: *mut sfnt_font_desc,                  // C 104

  /* Family name of the font.  */
  pub family: Lisp_Object,                        // C 107

  /* Style name of the font.  */
  pub style: Lisp_Object,                         // C 110

  /* The font foundry name, or `misc' if not present.  */
  pub designer: Lisp_Object,                      // C 113

  /* Style tokens that could not be parsed.  */
  pub adstyle: Lisp_Object,                       // C 116

  /* List of design languages.  */
  pub languages: Lisp_Object,                     // C 119

  /* Font registry that this font supports.  */
  pub registry: Lisp_Object,                      // C 122

  /* Vector of instances.  Each element is another of the instance's
     `style', `adstyle', and numeric width, weight, and slant.  May be
     nil.  */
  pub instances: Lisp_Object,                     // C 127

  /* Numeric width, weight, slant and spacing.  */
  pub width: c_int,                               // C 130
  pub weight: c_int,
  pub slant: c_int,
  pub spacing: c_int,

  /* Path to the font file.  */
  pub path: *mut c_char,                          // C 133

  /* char table consisting of characters already known to be
     present in the font.  */
  pub char_cache: Lisp_Object,                    // C 137

  /* The header of the cmap being used.  May be invalid, in which case
     platform_id will be 500.  */
  pub subtable: sfnt_cmap_encoding_subtable,      // C 141

  /* The offset of the table directory within PATH.  */
  pub offset: off_t,                              // C 144

  /* List of font tables.  */
  pub tables: *mut sfnt_font_tables,              // C 147

  /* The number of glyphs in this font.  Used to catch invalid cmap
     tables.  This is actually the number of glyphs - 1.  */
  pub num_glyphs: c_int,                          // C 151

  /* The number of references to the font tables below.  */
  pub refcount: c_int,                            // C 154

  /* The underline position and thickness if a post table supplies
     this information.  */
  pub underline_position: sfnt_fword,             // C 158
  pub underline_thickness: sfnt_fword,

  /* Whether an underline position is available.  */
  /* Whether or not the character map can't be used by Emacs.  */
  /// C 161: `bool_bf underline_position_set : 1;`
  /// C 164: `bool cmap_invalid : 1;`
  /// `bool_bf` is one byte on this target (clang: `sizeof (bool_bf)` = 1, no
  /// `NS_IMPL_GNUSTEP`/`__MINGW32__`), so the two 1-bit members share the byte at offset 132;
  /// the port keeps them in `bits`, LSB first (PORTING.md 1.3, the `lisp.rs` `Lisp_Marker`
  /// shape), and the writers in C 1052 name the mask below.
  pub bits: u8,
  /// C 165: the 3 bytes of padding to the record's 8-byte alignment (measured `sizeof` 136).
  pub _pad: [u8; 3],
}

/* clang, frozen configuration: `sizeof 136`, `_Alignof 8`, `underline_position` 128,
   `underline_thickness` 130 (the ledger's probe).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  core::assert! (std::mem::size_of::<sfnt_font_desc> () == 136);
  core::assert! (std::mem::align_of::<sfnt_font_desc> () == 8);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, next) == 0);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, family) == 8);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, instances) == 56);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, width) == 64);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, path) == 80);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, char_cache) == 88);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, subtable) == 96);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, offset) == 104);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, tables) == 112);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, num_glyphs) == 120);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, refcount) == 124);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, underline_position) == 128);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, underline_thickness) == 130);
  core::assert! (std::mem::offset_of! (sfnt_font_desc, bits) == 132);
};

/// C 161's `underline_position_set` bit, the byte `sfnt_font_desc.bits` above.
/// `docs/ports/token-aliases.tsv`'s `underline_position_set` family is owed (the ledger's
/// `## Owed`); until it lands, `extract-abi.py --check` reports the C body's token.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const SFNT_FONT_DESC_UNDERLINE_POSITION_SET: u8 = 1 << 0;
/// C 164's `cmap_invalid` bit (set in `sfntfont_open`, past this slice).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const SFNT_FONT_DESC_CMAP_INVALID: u8 = 1 << 1;

/* List of fonts.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut system_fonts: *mut sfnt_font_desc = std::ptr::null_mut ();   // C 169

/* Font enumeration and matching.  The sfnt driver assumes it can read
   data from each font at startup.  It then reads the head, meta and
   name tables to determine font data, and records the font in a list
   of system fonts that is then matched against.  */

/* Set up the coding system CODING to decode string data from the
   given platform id ID and platform specific id PLATFORM_SPECIFIC_ID.
   Value is 0 upon success, 1 upon failure.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_setup_coding_system (id: sfnt_platform_id, platform_specific_id: c_int,
                                    coding: *mut coding_system) -> c_int   // C 180-224
{
  let mut system: Lisp_Object;

  system = Qnil ();

  /* Figure out what coding system to use.  */

  match id
    {
      SFNT_PLATFORM_UNICODE =>                     /* C 190: `case SFNT_PLATFORM_UNICODE:`.  */
        { system = Qutf_16be (); }

      SFNT_PLATFORM_MACINTOSH =>                   /* C 195: `case SFNT_PLATFORM_MACINTOSH:`.  */
        {
          if platform_specific_id == SFNT_MACINTOSH_ROMAN
            { system = Qmac_roman (); }
          else
            /* MULE doesn't support the rest... */
            { system = Qnil (); }
        }

      SFNT_PLATFORM_MICROSOFT =>                   /* C 206: `case SFNT_PLATFORM_MICROSOFT:`.  */
        {
          system = Qutf_16be ();

          /* Not sure if this is right.  */
          if platform_specific_id == SFNT_MICROSOFT_BIG_FIVE
            { system = Qchinese_big5 (); }
        }

      _ =>                                         /* C 216: `default:`.  */
        { system = Qnil (); }
    }

  if NILP (system)
    { return 1; }

  setup_coding_system (system, coding);
  return 0;
}

/* `setup_coding_system` (C 222) is coding.h 686's declaration, which `include/coding.rs` does
   not carry; the unit declares it at the use site with the header's line (P-72 rule 3, the
   `fileio.rs` 2986 / `androidterm.rs` 174 shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: coding.h 686: `extern void setup_coding_system (Lisp_Object, struct coding_system *);`
  pub fn setup_coding_system (system: Lisp_Object, coding: *mut coding_system);
}

/* Globals used to communicate inside the condition-case wrapper.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_font_coding: *mut coding_system = std::ptr::null_mut ();        // C 227

/* The src_object being encoded from.  This should be on the stack as
   well, or it will get garbage collected.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_font_src_object: Lisp_Object = std::ptr::null_mut ();           // C 231

/* From-position.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_font_from: ptrdiff_t = 0;                                       // C 234
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_font_from_byte: ptrdiff_t = 0;

/* To-position.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_font_to: ptrdiff_t = 0;                                         // C 237
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_font_to_byte: ptrdiff_t = 0;

/* Destination object.  Once again, this should also be on the
   stack.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_font_dst_object: Lisp_Object = std::ptr::null_mut ();           // C 241

/* Error flag.  Set to true if a signal was caught.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_font_signal: bool = false;                                      // C 244

/* C 246-257.  `internal_condition_case` (C 286) takes the address of this helper, so it is an
   `unsafe extern "C" fn` item (PORTING.md 1.4).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_safe_decode_coding_object_1 () -> Lisp_Object
{
  decode_coding_object (sfnt_font_coding,
			sfnt_font_src_object,
			sfnt_font_from,
			sfnt_font_from_byte,
			sfnt_font_to,
			sfnt_font_to_byte,
			sfnt_font_dst_object);
  return Qnil ();
}

/* C 259-265.  The second helper, the handler `internal_condition_case` is given.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfnt_safe_decode_coding_object_2 (error: Lisp_Object) -> Lisp_Object
{
  sfnt_font_signal = true;

  return Qnil ();
}

/* Like decode_coding_object, but return 1 if a signal happens.  Value
   is otherwise 0.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_safe_decode_coding_object (coding: *mut coding_system,
                                          src_object: Lisp_Object,
                                          from: ptrdiff_t, from_byte: ptrdiff_t,
                                          to: ptrdiff_t, to_byte: ptrdiff_t,
                                          dst_object: Lisp_Object) -> c_int   // C 270-291
{
  sfnt_font_coding = coding;
  sfnt_font_src_object = src_object;
  sfnt_font_from = from;
  sfnt_font_from_byte = from_byte;
  sfnt_font_to = to;
  sfnt_font_to_byte = to_byte;
  sfnt_font_dst_object = dst_object;
  sfnt_font_signal = false;

  internal_condition_case (Some (sfnt_safe_decode_coding_object_1),
			   Qt (),
			   Some (sfnt_safe_decode_coding_object_2));

  return sfnt_font_signal as c_int;
}

/* `internal_condition_case` (C 286) is lisp.h 4805's declaration, which `include/lisp.rs` does
   not carry; the unit declares it at the use site with the header's line (P-72 rule 3, the
   `fileio.rs` 4973 shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: lisp.h 4805: `extern Lisp_Object internal_condition_case (Lisp_Object (*) (void),
  /// Lisp_Object, Lisp_Object (*) (Lisp_Object));`
  pub fn internal_condition_case (bfun: Option<unsafe extern "C" fn () -> Lisp_Object>,
				  handlers: Lisp_Object,
				  hfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>)
    -> Lisp_Object;
}

/* `strtok_r` (C 475) is `<string.h>`'s, which `lib/include/string.rs` does not carry for this
   target: gnulib's `string.in.h` 1206-1216 `REPLACE_STRTOK_R` arm is dead here
   (`build/emacs-ref/lib/string.h` 1753 is `#if 0`), so the declaration is the system header's
   and the symbol is plain `strtok_r` (a clang probe linked `_strtok_r`; the ledger carries the
   reading).  The unit declares it at the use site with the header's line (P-72 rule 3/P-55).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: string.in.h 1224-1227: `_GL_FUNCDECL_SYS (strtok_r, char *, (char *restrict s,
  /// char const *restrict delim, char **restrict save_ptr), _GL_ARG_NONNULL ((2, 3)));` - the
  /// non-replacement arm the frozen configuration takes.
  pub fn strtok_r (s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char)
    -> *mut c_char;
}

/* Decode the specified string DATA.  The encoding is determined based
   on PLATFORM_ID, PLATFORM_SPECIFIC_ID and LANGUAGE_ID.  Consult
   sfnt.h and the TrueType Reference Manual for more details.  LENGTH
   is the length of DATA in bytes.

   Value is nil upon failure, else the decoded string.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_decode_font_string (data: *mut u8, id: sfnt_platform_id,
				   platform_specific_id: c_int, language_id: c_int,
				   length: size_t) -> Lisp_Object          // C 300-320
{
  /* C 305: `struct coding_system coding;` is written by `memset` below (PORTING.md 1.5).  */
  let mut coding: coding_system = std::mem::zeroed ();

  memset (&raw mut coding as *mut c_void, 0, std::mem::size_of::<coding_system> ());
  sfnt_setup_coding_system (id, platform_specific_id, &raw mut coding);
  /* C's `coding.mode` and `coding.common_flags` are bitfields the mirror packs into `bits`
     (CODING_SYSTEM_MODE_SHIFT/CODING_SYSTEM_COMMON_FLAGS_SHIFT, PORTING.md 1.3); the two
     writers are these two statements.  */
  coding.bits |= (CODING_MODE_SAFE_ENCODING as u32) << CODING_SYSTEM_MODE_SHIFT;
  coding.bits |= (CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT;
  /* Suppress producing escape sequences for composition.  */
  coding.bits &= !((CODING_ANNOTATION_MASK as u32) << CODING_SYSTEM_COMMON_FLAGS_SHIFT);
  coding.source = data;

  if sfnt_safe_decode_coding_object (&raw mut coding, Qnil (), 0, 0,
				     length as ptrdiff_t, length as ptrdiff_t, Qt ()) != 0
    { return Qnil (); }

  return coding.dst_object;
}

/* Decode the family and style names from the name table NAME.  Return
   0 and the family and style names upon success, else 1.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_decode_family_style (name: *mut sfnt_name_table,
				    family: *mut Lisp_Object,
				    style: *mut Lisp_Object) -> c_int   // C 325-373
{
  /* C 329: the two records are filled in by `sfnt_find_name` below (PORTING.md 1.5).  */
  let mut family_rec: sfnt_name_record = std::mem::zeroed ();
  let mut style_rec: sfnt_name_record = std::mem::zeroed ();
  let mut family_data: *mut u8;
  let mut style_data: *mut u8;

  /* Because MS-Windows is incapable of treating font families
     comprising more than four styles correctly, the TrueType
     specification incorporates additional PREFERRED_FAMILY and
     PREFERRED_SUBFAMILY name resources that are meant to be consulted
     over the traditional family and subfamily resources.  When
     present within fonts supplying unusual styles, these names hold
     the ``actual'' typographic family and style of the font, in lieu
     of the font family with the style affixed to the front and
     Regular.  */

  family_data = sfnt_find_name (name, SFNT_NAME_PREFERRED_FAMILY,
				&raw mut family_rec);

  if family_data.is_null ()
    { family_data = sfnt_find_name (name, SFNT_NAME_FONT_FAMILY,
				    &raw mut family_rec); }

  style_data = sfnt_find_name (name, SFNT_NAME_PREFERRED_SUBFAMILY,
			       &raw mut style_rec);

  if style_data.is_null ()
    { style_data = sfnt_find_name (name, SFNT_NAME_FONT_SUBFAMILY,
				   &raw mut style_rec); }

  if family_data.is_null () || style_data.is_null ()
    { return 1; }

  /* Now decode the data.  */
  *family = sfnt_decode_font_string (family_data,
				     family_rec.platform_id as sfnt_platform_id,
				     family_rec.platform_specific_id as c_int,
				     family_rec.language_id as c_int,
				     family_rec.length as size_t);
  *style = sfnt_decode_font_string (style_data,
				    style_rec.platform_id as sfnt_platform_id,
				    style_rec.platform_specific_id as c_int,
				    style_rec.language_id as c_int,
				    style_rec.length as size_t);

  /* Return whether or not it was successful.  */
  return if !NILP (*family) && !NILP (*style) { 0 } else { 1 };
}

/* Decode the name of the specified font INSTANCE using the given NAME
   table.  Return the name of that instance, or nil upon failure.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_decode_instance_name (instance: *mut sfnt_instance,
				     name: *mut sfnt_name_table) -> Lisp_Object  // C 378-396
{
  /* C 382: filled in by `sfnt_find_name` below (PORTING.md 1.5).  */
  let mut name_rec: sfnt_name_record = std::mem::zeroed ();
  let mut name_data: *mut u8;

  name_data = sfnt_find_name (name, (*instance).name_id as sfnt_name_identifier_code,
			      &raw mut name_rec);

  if name_data.is_null ()
    { return Qnil (); }

  return sfnt_decode_font_string (name_data,
				  name_rec.platform_id as sfnt_platform_id,
				  name_rec.platform_specific_id as c_int,
				  name_rec.language_id as c_int,
				  name_rec.length as size_t);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct sfnt_style_desc                        // C 398-405
{
  /* The C string to match against.  */
  pub c_string: *const c_char,                    // C 401

  /* The value of the style field.  */
  pub value: c_int,                               // C 404
}

/* clang, frozen configuration: `sizeof 16`, `_Alignof 8`, `c_string` 0, `value` 8.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  core::assert! (std::mem::size_of::<sfnt_style_desc> () == 16);
  core::assert! (std::mem::align_of::<sfnt_style_desc> () == 8);
  core::assert! (std::mem::offset_of! (sfnt_style_desc, c_string) == 0);
  core::assert! (std::mem::offset_of! (sfnt_style_desc, value) == 8);
};

/* Array of style descriptions describing weight.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_weight_descriptions: [sfnt_style_desc; 17] = [   // C 408-427
  sfnt_style_desc { c_string: c"thin".as_ptr (), value: 0 },		// C 410
  sfnt_style_desc { c_string: c"extralight".as_ptr (), value: 40 },	// C 411
  sfnt_style_desc { c_string: c"ultralight".as_ptr (), value: 40 },	// C 412
  sfnt_style_desc { c_string: c"light".as_ptr (), value: 50 },		// C 413
  sfnt_style_desc { c_string: c"demilight".as_ptr (), value: 55 },	// C 414
  sfnt_style_desc { c_string: c"semilight".as_ptr (), value: 55 },	// C 415
  sfnt_style_desc { c_string: c"book".as_ptr (), value: 75 },		// C 416
  sfnt_style_desc { c_string: c"medium".as_ptr (), value: 100 },	// C 417
  sfnt_style_desc { c_string: c"demibold".as_ptr (), value: 180 },	// C 418
  sfnt_style_desc { c_string: c"semibold".as_ptr (), value: 180 },	// C 419
  sfnt_style_desc { c_string: c"bold".as_ptr (), value: 200 },		// C 420
  sfnt_style_desc { c_string: c"extrabold".as_ptr (), value: 205 },	// C 421
  sfnt_style_desc { c_string: c"ultrabold".as_ptr (), value: 205 },	// C 422
  sfnt_style_desc { c_string: c"black".as_ptr (), value: 210 },		// C 423
  sfnt_style_desc { c_string: c"heavy".as_ptr (), value: 210 },		// C 424
  sfnt_style_desc { c_string: c"extrablack".as_ptr (), value: 215 },	// C 425
  sfnt_style_desc { c_string: c"ultrablack".as_ptr (), value: 215 },	// C 426
];

/* Array of style descriptions describing slant.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_slant_descriptions: [sfnt_style_desc; 2] = [     // C 430-434
  sfnt_style_desc { c_string: c"italic".as_ptr (), value: 200 },	// C 432
  sfnt_style_desc { c_string: c"oblique".as_ptr (), value: 210 },	// C 433
];

/* Array of style descriptions describing width.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_width_descriptions: [sfnt_style_desc; 8] = [     // C 437-447
  sfnt_style_desc { c_string: c"ultracondensed".as_ptr (), value: 50 },	// C 439
  sfnt_style_desc { c_string: c"extracondensed".as_ptr (), value: 63 },	// C 440
  sfnt_style_desc { c_string: c"condensed".as_ptr (), value: 75 },	// C 441
  sfnt_style_desc { c_string: c"semicondensed".as_ptr (), value: 87 },	// C 442
  sfnt_style_desc { c_string: c"semiexpanded".as_ptr (), value: 113 },	// C 443
  sfnt_style_desc { c_string: c"expanded".as_ptr (), value: 125 },	// C 444
  sfnt_style_desc { c_string: c"extraexpanded".as_ptr (), value: 150 },	// C 445
  sfnt_style_desc { c_string: c"ultraexpanded".as_ptr (), value: 200 },	// C 446
];

/* Figure out DESC->width, DESC->weight, DESC->slant and DESC->spacing
   based on the style name passed as STYLE_NAME.

   Also append any unknown tokens to DESC->adstyle.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_parse_style (style_name: Lisp_Object, desc: *mut sfnt_font_desc)  // C 454-574
{
  let mut style: *mut c_char;
  let mut single: *mut c_char;
  let mut saveptr: *mut c_char;
  let mut c: c_char;
  let mut i: c_int;
  let mut x: ptrdiff_t;
  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  /* Fill in default values.  slant seems to not be consistent with
     Fontconfig.  */
  (*desc).weight = 80;
  (*desc).slant = 100;
  (*desc).width = 100;

  /* Split the style into tokens delimited by spaces.  Attempt to find
     a token specifying each of the weight, slant, or width attributes
     using their respective descriptions arrays as a reference.  */

  SAFE_ALLOCA_STRING! (sa_avail, style, Fdowncase (style_name));
  saveptr = std::ptr::null_mut ();

  while { single = strtok_r (style, c" ".as_ptr (), &raw mut saveptr);
          !single.is_null () }
    {
      'next: {
      style = std::ptr::null_mut ();

      if strcmp (single, c"regular".as_ptr ()) == 0
	/* ``Regular'' within a font family can represent either the
	   weight, slant or width of the font.  Leave each value as
	   its default, but never append it to the adstyle.  */
	{ break 'next; }                                /* C 483: `goto next;`.  */

      if (*desc).weight == 80
	{
	  /* Weight hasn't been found yet.  Scan through the weight
	     table.  */
	  i = 0;
	  while i < countof! (sfnt_weight_descriptions) as c_int
	    {
	      if strcmp (sfnt_weight_descriptions[i as usize].c_string,
			 single) == 0
		{
		  /* Weight found.  Continue on reading the slant and
		     width.  */
		  (*desc).weight = sfnt_weight_descriptions[i as usize].value;
		  break 'next;                          /* C 497: `goto next;`.  */
		}
	      i += 1;
	    }
	}

      if (*desc).slant == 100
	{
	  /* Slant hasn't been found yet.  Scan through the slant
	     table.  */
	  i = 0;
	  while i < countof! (sfnt_slant_descriptions) as c_int
	    {
	      if strcmp (sfnt_slant_descriptions[i as usize].c_string,
			 single) == 0
		{
		  /* Slant found.  Continue on reading the weight and
		     width.  */
		  (*desc).slant = sfnt_slant_descriptions[i as usize].value;
		  break 'next;                          /* C 514: `goto next;`.  */
		}
	      i += 1;
	    }
	}

      if (*desc).width == 100
	{
	  /* Width hasn't been found yet.  Scan through the width
	     table.  */
	  i = 0;
	  while i < countof! (sfnt_width_descriptions) as c_int
	    {
	      if strcmp (sfnt_width_descriptions[i as usize].c_string,
			 single) == 0
		{
		  /* Width found.  Continue on reading the slant and
		     weight.  */
		  (*desc).width = sfnt_width_descriptions[i as usize].value;
		  break 'next;                          /* C 531: `goto next;`.  */
		}
	      i += 1;
	    }
	}

      /* This token is extraneous or was not recognized.  Capitalize
	 the first letter if it's ASCII lowercase, then set the token as
	 the adstyle.  */

      if strlen (single) != 0
	{
	  *single = c_toupper (*single as c_int) as c_char;

	  if NILP ((*desc).adstyle)
	    { (*desc).adstyle = build_string (single); }
	  else
	    { (*desc).adstyle = CALLN! (Fconcat, (*desc).adstyle,
					build_string (c" ".as_ptr ()),
					build_string (single)); }
	}

      } /* 'next - C 552's `next:` and 553's `continue;` (the loop body's tail).  */
    }

  /* The adstyle must be a symbol, so intern it if it is set.  */

  if !NILP ((*desc).adstyle)
    {
      /* Characters that can't be represented in an XLFD must be
	 replaced.  */

      x = 0;
      while x < SBYTES ((*desc).adstyle)
	{
	  c = SREF ((*desc).adstyle, x as usize) as c_char;
	  if c == b'-' as c_char || c == b'*' as c_char
	     || c == b'?' as c_char || c == b'"' as c_char
	    { SSET ((*desc).adstyle, x as usize, b' '); }
	  x += 1;
	}

      (*desc).adstyle = Fintern ((*desc).adstyle, Qnil ());
    }

  SAFE_FREE! (sa_count);
}

/* Parse the list of design languages in META, a font metadata table,
   and place the results in DESC->languages.  Do nothing if there is
   no such metadata.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_parse_languages (meta: *mut sfnt_meta_table,
				desc: *mut sfnt_font_desc)            // C 580-616
{
  let mut data: *mut c_char;
  /* C 585: filled in by `sfnt_find_metadata` below (PORTING.md 1.5).  */
  let mut map: sfnt_meta_data_map = std::mem::zeroed ();

  /* Look up the ``design languages'' metadata.  This is a comma (and
     possibly space) separated list of scripts that the font was
     designed for.  Here is an example of one such tag:

       zh-Hans,Japn,Kore

     for a font that covers Simplified Chinese, along with Japanese
     and Korean text.  */

  data = sfnt_find_metadata (meta, SFNT_META_DATA_TAG_DLNG,
			     &raw mut map);

  if data.is_null ()
    {
      /* Fall back to the supported languages metadata.  */
      data = sfnt_find_metadata (meta, SFNT_META_DATA_TAG_SLNG,
				 &raw mut map);

      if data.is_null ()
	{ return; }
    }

  if !memmem (data as *const c_void, map.data_length as usize,
	      c"Hans".as_ptr () as *const c_void, 4).is_null ()
     || !memmem (data as *const c_void, map.data_length as usize,
		 c"Hant".as_ptr () as *const c_void, 4).is_null ()
    { (*desc).languages = Fcons (Qzh (), (*desc).languages); }
  if !memmem (data as *const c_void, map.data_length as usize,
	      c"Japn".as_ptr () as *const c_void, 4).is_null ()
    { (*desc).languages = Fcons (Qja (), (*desc).languages); }
  if !memmem (data as *const c_void, map.data_length as usize,
	      c"Kore".as_ptr () as *const c_void, 4).is_null ()
    { (*desc).languages = Fcons (Qko (), (*desc).languages); }
}

/* Return the font registry corresponding to the encoding subtable
   SUBTABLE.

   Under X, the font registry is an atom registered with the Open
   Group uniquely identifying the organization which defines the
   font's character set.

   In practice, the registry overlaps with the character set itself.
   So Emacs just uses the ``registry'' field of each font object and
   entity to represent both instead.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_registry_for_subtable (subtable: *mut sfnt_cmap_encoding_subtable)
  -> Lisp_Object                                   // C 629-684
{
  match (*subtable).platform_id as c_int
    {
      SFNT_PLATFORM_UNICODE =>                     /* C 634: `case SFNT_PLATFORM_UNICODE:`.  */
	{
	  /* Reject variation selector and last resort tables.  */
	  if ((*subtable).platform_specific_id as c_int
	      == SFNT_UNICODE_VARIATION_SEQUENCES)
	     || ((*subtable).platform_specific_id as c_int
		 == SFNT_UNICODE_LAST_RESORT)
	    { return Qnil (); }

	  return Qiso10646_1 ();
	}

      SFNT_PLATFORM_MACINTOSH =>                   /* C 644: `case SFNT_PLATFORM_MACINTOSH:`.  */
	{
	  match (*subtable).platform_specific_id as c_int
	    {
	      SFNT_MACINTOSH_ROMAN =>              /* C 648: `case SFNT_MACINTOSH_ROMAN:`.  */
		/* X calls mac-roman ``apple-roman''.  */
		{ return Qapple_roman (); }

	      _ =>                                 /* C 652: `default:`.  */
		/* Some other Macintosh charset not supported by Emacs.  */
		{ return Qnil (); }
	    }
	}

      SFNT_PLATFORM_MICROSOFT =>                   /* C 657: `case SFNT_PLATFORM_MICROSOFT:`.  */
	{
	  /* Microsoft specific encodings.  */

	  match (*subtable).platform_specific_id as c_int
	    {
	      SFNT_MICROSOFT_SYMBOL |              /* C 663: `case SFNT_MICROSOFT_SYMBOL:`.  */
	      SFNT_MICROSOFT_UNICODE_BMP =>        /* C 664: `case SFNT_MICROSOFT_UNICODE_BMP:`.  */
		/* Symbols in the Unicode PUA are still Unicode.  */
		{ return Qiso10646_1 (); }

	      SFNT_MICROSOFT_SHIFT_JIS =>          /* C 668: `case SFNT_MICROSOFT_SHIFT_JIS:`.  */
		{ return Qjisx0208_1983_0 (); }

	      SFNT_MICROSOFT_PRC =>                /* C 671: `case SFNT_MICROSOFT_PRC:`.  */
		{ return Qgbk (); }

	      SFNT_MICROSOFT_JOHAB =>              /* C 674: `case SFNT_MICROSOFT_JOHAB:`.  */
		{ return Qksc5601_1987_0 (); }

	      SFNT_MICROSOFT_UNICODE_UCS_4 =>      /* C 677: `case SFNT_MICROSOFT_UNICODE_UCS_4:`.  */
		{ return Qiso10646_1 (); }

	      _ => { }
	    }

	  /* C 681's `default:` is reached by fallthrough from the inner `switch`'s unmatched
	     cases; the port reaches this tail the same way.  */
	  return Qnil ();
	}

      _ => { }                                      /* C 681: `default:`.  */
    }

  /* C 681-682: `default: return Qnil;`.  The arm above falls through to it.  */
  return Qnil ();
}

/* Return the type of characters that the cmap subtable SUBTABLE maps
   from.  Value is:

   2 if SUBTABLE maps from Unicode characters, including those outside
   the Unicode Basic Multilingual Plane (BMP).

   1 if SUBTABLE maps from Unicode characters within the BMP.

   0 if SUBTABLE maps from some other character set that Emacs knows
   about.

   3 if SUBTABLE cannot be used by Emacs.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_identify_cmap (subtable: sfnt_cmap_encoding_subtable) -> c_int  // C 699-766
{
  match subtable.platform_id as c_int
    {
      SFNT_PLATFORM_UNICODE =>                     /* C 704: `case SFNT_PLATFORM_UNICODE:`.  */
	{
	  /* Reject variation selector and last resort tables.  */
	  if (subtable.platform_specific_id as c_int
	      == SFNT_UNICODE_VARIATION_SEQUENCES)
	     || (subtable.platform_specific_id as c_int
		 == SFNT_UNICODE_LAST_RESORT)
	    { return 3; }

	  /* 1.0, 1.1, ISO-10646-1993, and 2.0_BMP tables are all within
	     the BMP.  */
	  if (subtable.platform_specific_id as c_int) < SFNT_UNICODE_2_0
	    { return 1; }

	  return 2;
	}

      SFNT_PLATFORM_MACINTOSH =>                   /* C 720: `case SFNT_PLATFORM_MACINTOSH:`.  */
	{
	  match subtable.platform_specific_id as c_int
	    {
	      SFNT_MACINTOSH_ROMAN =>              /* C 724: `case SFNT_MACINTOSH_ROMAN:`.  */
		/* mac-roman */
		{ return 0; }

	      _ =>                                 /* C 728: `default:`.  */
		/* Some other Macintosh charset not supported by Emacs.  */
		{ return 3; }
	    }
	}

      SFNT_PLATFORM_MICROSOFT =>                   /* C 733: `case SFNT_PLATFORM_MICROSOFT:`.  */
	{
	  /* Microsoft specific encodings.  */

	  match subtable.platform_specific_id as c_int
	    {
	      SFNT_MICROSOFT_SYMBOL =>             /* C 739: `case SFNT_MICROSOFT_SYMBOL:`.  */
		/* Symbols in the Unicode PUA are still Unicode.  */
		{ return 1; }

	      SFNT_MICROSOFT_UNICODE_BMP =>        /* C 743: `case SFNT_MICROSOFT_UNICODE_BMP:`.  */
		{ return 1; }

	      SFNT_MICROSOFT_SHIFT_JIS =>          /* C 746: `case SFNT_MICROSOFT_SHIFT_JIS:`.  */
		/* PCK aka japanese-jisx0208.  */
		{ return 0; }

	      SFNT_MICROSOFT_PRC =>                /* C 750: `case SFNT_MICROSOFT_PRC:`.  */
		/* GBK, GB2312 or GB18030.  */
		{ return 0; }

	      SFNT_MICROSOFT_JOHAB =>              /* C 754: `case SFNT_MICROSOFT_JOHAB:`.  */
		/* KS C 5601-1992, aka korean-ksc5601.  */
		{ return 0; }

	      SFNT_MICROSOFT_UNICODE_UCS_4 =>      /* C 758: `case SFNT_MICROSOFT_UNICODE_UCS_4:`.  */
		/* Unicode past the BMP.  */
		{ return 2; }

	      _ => { }                 /* C 763's `default:` is reached by fallthrough below.  */
	    }

	  /* C's MICROSOFT case falls through into C 763's `default: return 3;`.  */
	  return 3;
	}

      _ =>                                         /* C 763: `default:`.  */
	{ return 3; }
    }
}

/* Figure out which registry DESC, backed by FD, whose table directory
   is SUBTABLE, is likely to support.

   Read the header of each subtable in the character map and compute
   the registry to use; then, set DESC->registry to that value.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_grok_registry (fd: c_int, desc: *mut sfnt_font_desc,
			      subtable: *mut sfnt_offset_subtable)       // C 774-836
{
  let mut cmap: *mut sfnt_cmap_table;
  /* C 779: `subtables` is written by `sfnt_read_cmap_table` below (PORTING.md 1.5).  */
  let mut subtables: *mut sfnt_cmap_encoding_subtable = std::ptr::null_mut ();
  let mut i: c_int;

  cmap = sfnt_read_cmap_table (fd, subtable, &raw mut subtables, std::ptr::null_mut ());

  if cmap.is_null ()
    { return; }

  /* Now pick the ``best'' character map the same way as sfntfont_open
     does.  The caveat is that since the subtable data has not been
     read, Emacs cannot determine whether or not the encoding subtable
     is valid.

     Once platform_id is set, that value becomes much more
     reliable.  */

  'done: {
  /* First look for a non-BMP Unicode cmap.  */

  i = 0;
  while i < (*cmap).num_subtables as c_int
    {
      if sfntfont_identify_cmap (*(subtables.offset (i as isize))) == 2
	{
	  (*desc).registry
	    = sfnt_registry_for_subtable (&raw mut *(subtables.offset (i as isize)));
	  break 'done;                                 /* C 803: `goto done;`.  */
	}
      i += 1;
    }

  /* Next, look for a BMP only Unicode cmap.  */

  i = 0;
  while i < (*cmap).num_subtables as c_int
    {
      if sfntfont_identify_cmap (*(subtables.offset (i as isize))) == 1
        {
	  (*desc).registry
	    = sfnt_registry_for_subtable (&raw mut *(subtables.offset (i as isize)));
	  break 'done;                                 /* C 815: `goto done;`.  */
	}
      i += 1;
    }

  /* Finally, use the first cmap that appears and can be
     identified.  */

  i = 0;
  while i < (*cmap).num_subtables as c_int
    {
      if sfntfont_identify_cmap (*(subtables.offset (i as isize))) == 0
        {
	  (*desc).registry
	    = sfnt_registry_for_subtable (&raw mut *(subtables.offset (i as isize)));
	  break 'done;                                 /* C 828: `goto done;`.  */
	}
      i += 1;
    }
  }

  /* There are no cmaps available to Emacs.  */
  /* C 833: `done:`.  */
  xfree (cmap as *mut c_void);
  xfree (subtables as *mut c_void);
}

/* Return whether or not the font description PREV conflicts with the
   newer font description DESC, and should be removed from the list of
   system fonts.

   If both PREV and DESC are variable fonts, remove styles within PREV
   that overlap with DESC and return false.

   If PREV is a variable font, potentially adjust its list of
   instances.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_replace_fonts_p (prev: *mut sfnt_font_desc,
				desc: *mut sfnt_font_desc) -> bool   // C 848-943
{
  let mut i: c_int;
  let mut j: c_int;
  let mut width: c_int;
  let mut weight: c_int;
  let mut slant: c_int;
  let mut count_instance: c_int;
  let mut tem: Lisp_Object;
  let mut tem1: Lisp_Object;
  let mut family_equal_p: bool;

  family_equal_p = !NILP (Fstring_equal ((*prev).family,
					 (*desc).family));

  if (!NILP ((*desc).instances)
      || !NILP (Fstring_equal ((*prev).style, (*desc).style)))
     && family_equal_p
    {
      /* If both inputs are GX fonts...  */
      if !NILP ((*desc).instances) && !NILP ((*prev).instances)
	{
	  /* ...iterate over each of the styles provided by PREV.  If
	     they match any styles within DESC, remove the old style
	     from PREV.  */

	  count_instance = 0;
	  i = 0;
	  /* The loop carries a label because C's `continue` at C 876 leaves the `'next` block
	     below, which Rust rejects without one (E0695).  */
	  'instances: while i < ASIZE ((*prev).instances) as c_int
	    {
	      'next: {
	      tem = AREF ((*prev).instances, i as isize);

	      if NILP (tem)
		{ i += 1; continue 'instances; }      /* C 875-876: `continue;`.  */

	      j = 0;
	      while j < ASIZE ((*desc).instances) as c_int
		{
		  tem1 = AREF ((*desc).instances, j as isize);

		  if NILP (tem1)
		    { j += 1; continue; }

		  if !NILP (Fequal (tem1, tem))
		    {
		      /* tem1 is identical to tem, so opt for it over
			 tem.  */
		      ASET ((*prev).instances, i as isize, Qnil ());
		      break 'next;                        /* C 890: `goto next;`.  */
		    }
		  j += 1;
		}

	      /* Increment the number of instances remaining within
		 PREV.  */
	      count_instance += 1;

	      } /* 'next - C 898's `next:` and 899's `;`.  */
	      i += 1;
	    }

	  /* Return true if no instances remain inside
	     PREV->instances, so that the now purposeless desc may be
	     removed.  */
	  return count_instance == 0;
	}

      return true;
    }

  if NILP ((*prev).instances) || !family_equal_p
    { return false; }

  /* Look through instances in PREV to see if DESC provides the same
     thing.  */

  count_instance = 0;
  i = 0;
  while i < ASIZE ((*prev).instances) as c_int
    {
      tem = AREF ((*prev).instances, i as isize);

      if NILP (tem)
	{ i += 1; continue; }

      width = XFIXNUM (AREF (tem, 2)) as c_int;
      weight = XFIXNUM (AREF (tem, 3)) as c_int;
      slant = XFIXNUM (AREF (tem, 4)) as c_int;

      if (*desc).width == width
	 && (*desc).weight == weight
	 && (*desc).slant == slant
	{
	  /* Remove this instance.  */
	  ASET ((*prev).instances, i as isize, Qnil ());
	  i += 1;
	  continue;
	}

      count_instance += 1;
      i += 1;
    }

  /* Remove this desc if there are no more instances.  */
  return count_instance < 1;
}

/* Enumerate the offset subtable SUBTABLES in the file FD, whose file
   name is FILE.  OFFSET should be the offset of the subtable within
   the font file, and is recorded for future use.  Value is 1 upon
   failure, else 0.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_enum_font_1 (fd: c_int, file: *const c_char,
			    subtables: *mut sfnt_offset_subtable,
			    offset: off_t) -> c_int                 // C 950-1153
{
  let mut desc: *mut sfnt_font_desc;
  let mut next: *mut *mut sfnt_font_desc;
  let mut prev: *mut sfnt_font_desc;
  /* C 956-959 leave these four uninitialized; the `bailN` ladder below frees them, and the
     port zero-initializes so the ladder's guards are what decide (PORTING.md 1.5).  On the
     paths C can reach, each one has been assigned before its label is jumped to.  */
  let mut head: *mut sfnt_head_table = std::ptr::null_mut ();
  let mut name: *mut sfnt_name_table = std::ptr::null_mut ();
  let mut meta: *mut sfnt_meta_table = std::ptr::null_mut ();
  let mut maxp: *mut sfnt_maxp_table = std::ptr::null_mut ();
  let mut fvar: *mut sfnt_fvar_table;
  let mut OS_2: *mut sfnt_OS_2_table;
  let mut post: *mut sfnt_post_table;
  /* C 963: `struct sfnt_font_desc temp;` - `sfnt_parse_style` writes the fields below
     (PORTING.md 1.5).  */
  let mut temp: sfnt_font_desc = std::mem::zeroed ();
  /* C 964: `Lisp_Object family, style, instance, style1;` - `family` and `style` are the
     out-parameters of `sfnt_decode_family_style` below (PORTING.md 1.5).  */
  let mut family: Lisp_Object = std::ptr::null_mut ();
  let mut style: Lisp_Object = std::ptr::null_mut ();
  let mut instance: Lisp_Object;
  let mut style1: Lisp_Object;
  let mut i: c_int;
  let mut buffer: [c_char; 5] = [0; 5];             // C 966

  /* C 1143-1152's `bail4:`/`bail3:`/`bail2:`/`bail1:` ladders are a fall-through chain the four
     `goto bailN`s at C 977/982/986/996 jump into.  The port carries the jump's depth in `bail`
     and runs the four segments in C's order at the tail (PORTING.md 2.2: the cleanup must not
     become an early `return`).  */
  let mut bail: c_int = 0;

  'body: loop
    {
      /* Create the font desc and copy in the file name.  */
      desc = xzalloc (std::mem::size_of::<sfnt_font_desc> () + strlen (file) + 1)
	as *mut sfnt_font_desc;
      (*desc).path = desc.offset (1) as *mut c_char;
      memcpy ((*desc).path as *mut c_void, file as *const c_void, strlen (file) + 1);
      (*desc).offset = offset;

      /* Check that this is a TrueType font.  */
      if (*subtables).scaler_type != SFNT_SCALER_TRUE as uint32_t
	 && (*subtables).scaler_type != SFNT_SCALER_VER1 as uint32_t
	{ bail = 1; break 'body; }                  /* C 977: `goto bail1;`.  */

      /* Read required tables.  */
      head = sfnt_read_head_table (fd, subtables);
      if head.is_null ()
	{ bail = 1; break 'body; }                  /* C 982: `goto bail1;`.  */

      name = sfnt_read_name_table (fd, subtables);
      if name.is_null ()
	{ bail = 2; break 'body; }                  /* C 986: `goto bail2;`.  */

      maxp = sfnt_read_maxp_table (fd, subtables);
      if maxp.is_null ()
	{ bail = 3; break 'body; }                  /* C 990: `goto bail3;`.  */

      /* meta is not required, nor present on many non-Apple fonts.  */
      meta = sfnt_read_meta_table (fd, subtables);

      /* Decode the family and style from the name table.  */
      if sfnt_decode_family_style (name, &raw mut family, &raw mut style) != 0
	{ bail = 4; break 'body; }                  /* C 996-997: `goto bail4;`.  */

      /* See if this is a distortable/variable/multiple master font (all
	 three terms mean the same time.)  */
      fvar = sfnt_read_fvar_table (fd, subtables);

      /* Set the family.  */
      (*desc).family = family;
      (*desc).char_cache = Qnil ();
      (*desc).subtable.platform_id = 500;

      /* Now set the font foundry name.  This information is located
	 within the OS/2 table's `ach_vendor_id' field, but use `misc' as
	 a recourse if it is not present.  */

      OS_2 = sfnt_read_OS_2_table (fd, subtables);

      if !OS_2.is_null ()
	{
	  memcpy (buffer.as_mut_ptr () as *mut c_void,
		  (*OS_2).ach_vendor_id.as_ptr () as *const c_void,
		  std::mem::size_of_val (&(*OS_2).ach_vendor_id));
	  buffer[std::mem::size_of_val (&(*OS_2).ach_vendor_id)] = 0;

	  /* If the foundry name is empty, use `misc' instead.  */

	  if buffer[0] == 0
	    { (*desc).designer = Qmisc (); }
	  else
	    { (*desc).designer = intern (buffer.as_ptr ()); }

	  xfree (OS_2 as *mut c_void);
	}
      else
	{ (*desc).designer = Qmisc (); }

      /* Set the largest glyph identifier.  */
      (*desc).num_glyphs = (*maxp).num_glyphs as c_int;

      /* Parse the style.  */
      sfnt_parse_style (style, desc);

      /* If the meta table exists, parse the list of design languages.  */
      if !meta.is_null ()
	{ sfnt_parse_languages (meta, desc); }

      /* Check whether the font claims to be a fixed pitch font and forgo
	 the rudimentary detection below if so.  */

      post = sfnt_read_post_table (fd, subtables);

      if !post.is_null ()
	{
	  (*desc).spacing = if (*post).is_fixed_pitch != 0 { 100 } else { 0 };
	  (*desc).underline_position = (*post).underline_position;
	  (*desc).underline_thickness = (*post).underline_thickness;
	  /* C 1052: `desc->underline_position_set = true;` - the 1-bit member is the byte
	     `bits` above (PORTING.md 1.3).  */
	  (*desc).bits = (*desc).bits | SFNT_FONT_DESC_UNDERLINE_POSITION_SET;
	  xfree (post as *mut c_void);
	}
      else
	{
	  /* Figure out the spacing.  Some fancy test like what Fontconfig
	     does is probably in order but not really necessary.  */
	  if !NILP (Fstring_search (Fdowncase (family),
				    build_string (c"mono".as_ptr ()),
				    Qnil ()))
	    { (*desc).spacing = 100; }                /* FC_MONO */
	}

      /* Finally add mac-style flags.  Allow them to override styles that
	 have not been found.  */

      /* C 1068 writes the mask `01`, C 1071 `02`.  */
      if (*head).mac_style & 1 != 0 && (*desc).weight == 80	/* Bold */
	{ (*desc).weight = 200; }

      if (*head).mac_style & 2 != 0 && (*desc).slant == 0	/* Italic */
	{ (*desc).slant = 100; }

      /* Figure out what registry this font is likely to support.  */
      sfnt_grok_registry (fd, desc, subtables);

      if !fvar.is_null () && (*fvar).instance_count != 0
	{
	  /* If there is an fvar table with instances, then this is a font
	     which defines different axes along which the points in each
	     glyph can be changed.

	     Instead of enumerating the font itself, enumerate each
	     instance within, which specifies how to configure each axis
	     to achieve a specified style.  */

	  (*desc).instances = make_vector ((*fvar).instance_count as ptrdiff_t, Qnil ());

	  i = 0;
	  while i < (*fvar).instance_count as c_int
	    {
	      style1 = sfnt_decode_instance_name
			 (&raw mut *((*fvar).instance.offset (i as isize)), name);

	      if NILP (style1)
		{ i += 1; continue; }

	      /* Now parse the style.  */
	      temp.adstyle = Qnil ();
	      sfnt_parse_style (style1, &raw mut temp);

	      /* Set each field of the vector.  */
	      instance = make_vector (5, Qnil ());
	      ASET (instance, 0, style1);
	      ASET (instance, 1, temp.adstyle);
	      ASET (instance, 2, make_fixnum (temp.width as EMACS_INT));
	      ASET (instance, 3, make_fixnum (temp.weight as EMACS_INT));
	      ASET (instance, 4, make_fixnum (temp.slant as EMACS_INT));

	      /* Place the vector in desc->instances.  */
	      ASET ((*desc).instances, i as isize, instance);
	      i += 1;
	    }
	}

      /* Set the style, link the desc onto system_fonts and return.  */
      (*desc).style = style;
      (*desc).next = system_fonts;
      system_fonts = desc;

      /* Remove any fonts which have the same style as this one.  For
	 distortable fonts, only remove overlapping styles, unless this is
	 also a distortable font.  */

      next = &raw mut (*system_fonts).next;
      prev = *next;
      while !(*next).is_null ()                       /* C 1125: `for (; *next; prev = *next)`.  */
	{
	  if sfnt_replace_fonts_p (prev, desc)
	    {
	      *next = (*prev).next;
	      xfree (prev as *mut c_void);
	    }
	  else
	    { next = &raw mut (*prev).next; }
	  prev = *next;
	}

      xfree (fvar as *mut c_void);
      xfree (meta as *mut c_void);
      xfree (maxp as *mut c_void);
      xfree (name as *mut c_void);
      xfree (head as *mut c_void);
      return 0;
    }

  /* C 1143: `bail4:`.  */
  if bail >= 4
    {
      xfree (meta as *mut c_void);
      xfree (maxp as *mut c_void);
    }
  /* C 1146: `bail3:`.  */
  if bail >= 3
    { xfree (name as *mut c_void); }
  /* C 1148: `bail2:`.  */
  if bail >= 2
    { xfree (head as *mut c_void); }
  /* C 1150: `bail1:`.  */
  xfree (desc as *mut c_void);
  return 1;
}

/* Enumerate the font FILE into the list of system fonts.  Return 1 if
   it could not be enumerated, 0 otherwise.

   Remove any font whose family and style is a duplicate of this one.

   FILE can either be a TrueType collection file containing TrueType
   fonts, or a TrueType font itself.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfnt_enum_font (file: *const c_char) -> c_int   // C 1163-1240
{
  let mut fd: c_int;
  let mut rc: c_int;
  let mut seek: off_t;
  let mut subtables: *mut sfnt_offset_subtable;
  let mut ttc: *mut sfnt_ttc_header;
  let mut i: size_t;

  /* C 1236-1239's `bail0:`/`bail:` pair: `goto bail` skips `emacs_close`, the three
     `goto bail0` sites (C 1199/1202/1213) run it first.  The port carries the level in `bail`
     and runs the two segments in C's order (PORTING.md 2.2).  */
  let mut bail: c_int = 0;

  'body: loop
    {
      /* Now open the font for reading.  */
      fd = emacs_open (file, O_RDONLY, 0);

      if fd == -1
	{ bail = 1; break 'body; }                  /* C 1176: `goto bail;`.  */

      /* Read the table directory.  */
      subtables = sfnt_read_table_directory (fd);

      if subtables == usize::MAX as *mut sfnt_offset_subtable   /* C 1182: `(T *) -1`.  */
	{
	  /* This is actually a TrueType container file.  Go back to the
	     beginning and read the TTC header.  */

	  if lseek (fd, 0, SEEK_SET) != 0
	    { bail = 2; break 'body; }              /* C 1199: `goto bail0;`.  */

	  ttc = sfnt_read_ttc_header (fd);

	  if ttc.is_null ()
	    { bail = 2; break 'body; }              /* C 1202: `goto bail0;`.  */

	  /* Enumerate each of the fonts in the collection.  */

	  i = 0;
	  while i < (*ttc).num_fonts as size_t
	    {
	      seek = lseek (fd,
			    *((*ttc).offset_table.offset (i as isize)) as off_t,
			    SEEK_SET);

	      if seek == -1 as off_t
		 || seek != *((*ttc).offset_table.offset (i as isize)) as off_t
		{ i += 1; continue; }

	      subtables = sfnt_read_table_directory (fd);

	      if subtables.is_null ()
		  /* This value means that FD was pointing at a TTC
		     header.  Since FD should already have been moved to
		     the beginning of the TrueType header above, it
		     follows that the font format is invalid.  */
		 || subtables == usize::MAX as *mut sfnt_offset_subtable
		{ i += 1; continue; }

	      sfnt_enum_font_1 (fd, file, subtables,
				*((*ttc).offset_table.offset (i as isize)) as off_t);
	      xfree (subtables as *mut c_void);
	      i += 1;
	    }

	  /* Always treat reading containers as having been
	     successful.  */

	  emacs_close (fd);
	  xfree (ttc as *mut c_void);
	  return 0;
	}

      if subtables.is_null ()
	{ bail = 2; break 'body; }                  /* C 1213: `goto bail0;`.  */

      /* Now actually enumerate this font.  */
      rc = sfnt_enum_font_1 (fd, file, subtables, 0);
      xfree (subtables as *mut c_void);
      emacs_close (fd);
      return rc;
    }

  /* C 1236: `bail0:`.  */
  if bail >= 2
    { emacs_close (fd); }
  /* C 1238: `bail:`.  */
  return 1;
}

/* ----------------------------------------------------------------------------------------------
   C 1241-1426: the blank line (C 1241), the form feed (C 1242), C 1243's blank and C 1244's
   `/* Font discovery and matching.  */` banner, then the three cmap selectors -
   `sfntfont_charset_for_name` (C 1246-1264), `sfntfont_charset_for_cmap` (C 1269-1334) and
   `sfntfont_select_cmap` (C 1344-1425), with C 1426's blank.  Slice 1 stopped on C 1240's `}`;
   **the next live definition is C 1432's `sfntfont_read_cmap`**, and the ledger's `## C line
   ranges` names both boundaries.

   Every item carries the file gate `all (have_android, not_android_stubify)` (slice 1's reading:
   `configure.ac` 2645 puts `sfntfont.o` in `ANDROID_OBJ` only in the `XCONFIGURE = android` branch,
   and `ANDROID_STUBIFY` is defined in the other one), and no `#if` inside these 186 C lines.
   ---------------------------------------------------------------------------------------------- */
/// C 1246-1264: `sfntfont_charset_for_name` - C's `static struct charset *`, the `CHARSET_*`
/// macros being `include/charset.rs`'s (`charset.h` 265/301).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_charset_for_name (symbol: Lisp_Object) -> *mut charset
{                                                          // C 1247-1264
  let mut idx: ptrdiff_t;
  let mut id: c_int;

  idx = CHARSET_SYMBOL_HASH_INDEX! (symbol);               // C 1252

  if idx == -1
    { return std::ptr::null_mut (); }                      // C 1255

  /* Vcharset_hash_table is not a real variable, so Lisp programs
     can't clobber it.  */
  id = XFIXNUM (AREF (HASH_VALUE (XHASH_TABLE (Vcharset_hash_table),   // C 1259-1261
				  idx as hash_idx_t),
		      charset_id as isize)) as c_int;

  return CHARSET_FROM_ID! (id);                            // C 1263
}

/// C 1269-1334: `sfntfont_charset_for_cmap` - C's `static struct charset *`, one `switch` over
/// `platform_id` with the two nested `switch`es (the `sfntfont_identify_cmap` shape above), whose
/// second has no `default` of its own and so falls out of the block into C 1332's outer
/// `default: return NULL;` - the `_ => { }` + explicit return below.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_charset_for_cmap (subtable: sfnt_cmap_encoding_subtable) -> *mut charset
{                                                          // C 1270-1334
  match subtable.platform_id as c_int
    {
      SFNT_PLATFORM_UNICODE =>                 /* C 1274: `case SFNT_PLATFORM_UNICODE:`.  */
	{
	  /* Reject variation selector and last resort tables.  */
	  if (subtable.platform_specific_id as c_int
	      == SFNT_UNICODE_VARIATION_SEQUENCES)
	     || (subtable.platform_specific_id as c_int
		 == SFNT_UNICODE_LAST_RESORT)
	    { return std::ptr::null_mut (); }          // C 1280

	  /* 1.0, 1.1, ISO-10646-1993, and 2.0_BMP tables are all within
	     the BMP.  */
	  if (subtable.platform_specific_id as c_int) < SFNT_UNICODE_2_0
	    { return sfntfont_charset_for_name (Qunicode_bmp ()); }   // C 1285

	  return sfntfont_charset_for_name (Qunicode ());             // C 1287
	}

      SFNT_PLATFORM_MACINTOSH =>               /* C 1289: `case SFNT_PLATFORM_MACINTOSH:`.  */
	{
	  match subtable.platform_specific_id as c_int
	    {
	      SFNT_MACINTOSH_ROMAN =>              /* C 1293.  */
		{ return sfntfont_charset_for_name (Qmac_roman ()); }

	      _ =>                                 /* C 1296: `default:`.  */
		/* Some other Macintosh charset not supported by Emacs.  */
		{ return std::ptr::null_mut (); }  // C 1298
	    }
	}

      SFNT_PLATFORM_MICROSOFT =>               /* C 1301: `case SFNT_PLATFORM_MICROSOFT:`.  */
	{
	  /* Microsoft specific encodings.  */

	  match subtable.platform_specific_id as c_int
	    {
	      SFNT_MICROSOFT_SYMBOL =>             /* C 1307.  */
		/* Symbols in the Unicode PUA are still Unicode.  */
		{ return sfntfont_charset_for_name (Qunicode ()); }   // C 1309

	      SFNT_MICROSOFT_UNICODE_BMP =>        /* C 1311.  */
		{ return sfntfont_charset_for_name (Qunicode_bmp ()); }   // C 1312

	      SFNT_MICROSOFT_SHIFT_JIS =>          /* C 1314.  */
		/* PCK aka japanese-jisx0208.  */
		{ return sfntfont_charset_for_name (Qjapanese_jisx0208 ()); }   // C 1316

	      SFNT_MICROSOFT_PRC =>                /* C 1318.  */
		/* GBK, GB2312 or GB18030.  */
		{ return sfntfont_charset_for_name (Qgbk ()); }   // C 1320

	      SFNT_MICROSOFT_JOHAB =>              /* C 1322.  */
		/* KS C 5601-1992, aka korean-ksc5601.  */
		{ return sfntfont_charset_for_name (Qkorean_ksc5601 ()); }   // C 1324

	      SFNT_MICROSOFT_UNICODE_UCS_4 =>      /* C 1326.  */
		/* Unicode past the BMP.  */
		{ return sfntfont_charset_for_name (Qucs ()); }   // C 1328

	      _ => { }             /* C 1331's `default:` is reached by fallthrough below.  */
	    }

	  /* C's MICROSOFT case falls out of its inner `switch` into C 1331's `default: return
	     NULL;`, exactly as `sfntfont_identify_cmap`'s does.  */
	  return std::ptr::null_mut ();
	}

      _ =>                                         /* C 1331: `default:`.  */
	{ return std::ptr::null_mut (); }              // C 1332
    }
}

/// C 1344-1425: `sfntfont_select_cmap` - C's `static struct sfnt_cmap_encoding_subtable_data *`,
/// three scans over `cmap->num_subtables` (non-BMP Unicode, BMP-only Unicode, then the first
/// identified cmap), the first two also looking for a format 14 table when `format14` is non-NULL.
/// C's `data[i]` truth tests are the pointer tests below; `*subtable`/`*format14` are C's writes
/// through the out-parameters.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_select_cmap (cmap: *mut sfnt_cmap_table,
				subtables: *mut sfnt_cmap_encoding_subtable,
				data: *mut *mut sfnt_cmap_encoding_subtable_data,
				subtable: *mut sfnt_cmap_encoding_subtable,
				format14: *mut *mut sfnt_cmap_format_14)
  -> *mut sfnt_cmap_encoding_subtable_data
{                                                          // C 1345-1425
  let mut i: c_int;
  let mut j: c_int;

  /* First look for a non-BMP Unicode cmap.  */

  i = 0;
  while i < (*cmap).num_subtables as c_int            // C 1355
    {
      if !(*(data.offset (i as isize))).is_null ()
	 && sfntfont_identify_cmap (*(subtables.offset (i as isize))) == 2
	{
	  *subtable = *(subtables.offset (i as isize));   // C 1359

	  if format14.is_null ()
	    { return *(data.offset (i as isize)); }       // C 1362

	  /* Search for a corresponding format 14 character map.
	     This is used in conjunction with the selected character
	     map to map variation sequences.  */

	  j = 0;
	  while j < (*cmap).num_subtables as c_int        // C 1368
	    {
	      if !(*(data.offset (j as isize))).is_null ()
		 && (*(subtables.offset (j as isize))).platform_id as c_int
		    == SFNT_PLATFORM_UNICODE
		 && ((*(subtables.offset (j as isize))).platform_specific_id as c_int
		     == SFNT_UNICODE_VARIATION_SEQUENCES)
		 && (*(*(data.offset (j as isize)))).format == 14
		{ *format14                                           // C 1375
		    = *(data.offset (j as isize)) as *mut sfnt_cmap_format_14; }
	      j += 1;
	    }

	  return *(data.offset (i as isize));             // C 1378
	}
      i += 1;
    }

  /* Next, look for a BMP only Unicode cmap.  */

  i = 0;
  while i < (*cmap).num_subtables as c_int            // C 1384
    {
      if !(*(data.offset (i as isize))).is_null ()
	 && sfntfont_identify_cmap (*(subtables.offset (i as isize))) == 1
	{
	  *subtable = *(subtables.offset (i as isize));   // C 1388

	  if format14.is_null ()
	    { return *(data.offset (i as isize)); }       // C 1391

	  /* Search for a corresponding format 14 character map.
	     This is used in conjunction with the selected character
	     map to map variation sequences.  */

	  j = 0;
	  while j < (*cmap).num_subtables as c_int        // C 1397
	    {
	      if !(*(data.offset (j as isize))).is_null ()
		 && (*(subtables.offset (j as isize))).platform_id as c_int
		    == SFNT_PLATFORM_UNICODE
		 && ((*(subtables.offset (j as isize))).platform_specific_id as c_int
		     == SFNT_UNICODE_VARIATION_SEQUENCES)
		 && (*(*(data.offset (j as isize)))).format == 14
		{ *format14                                           // C 1404
		    = *(data.offset (j as isize)) as *mut sfnt_cmap_format_14; }
	      j += 1;
	    }

	  return *(data.offset (i as isize));             // C 1407
	}
      i += 1;
    }

  /* Finally, use the first cmap that appears and can be
     identified.  */

  i = 0;
  while i < (*cmap).num_subtables as c_int            // C 1414
    {
      if !(*(data.offset (i as isize))).is_null ()
	 && sfntfont_identify_cmap (*(subtables.offset (i as isize))) == 0
	{
	  *subtable = *(subtables.offset (i as isize));   // C 1418
	  return *(data.offset (i as isize));             // C 1419
	}
      i += 1;
    }

  /* There are no cmaps available to Emacs.  */
  return std::ptr::null_mut ();                          // C 1424
}
