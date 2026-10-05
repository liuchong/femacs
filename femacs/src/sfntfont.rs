// femacs: port of src/sfntfont.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct field, macro, file-scope object, local), the same control flow and the same error model.
// The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static`
// helpers stay plain `unsafe fn` except the two whose address C takes, which become
// `unsafe extern "C" fn` items (PORTING.md 1.4; the androidselect.rs `close_asset_fd` shape).
//
// **This file is P-86's unit `src/sfntfont.c` (4,222 lines), ported in slices: C 1-1240,
// C 1241-1426, C 1427-1604, C 1605-1871, C 1873-2050, C 2051-2172 and C 2173-2559.**  The next live
// definition is `sfnt_open_tables` (C 2633, after `struct sfnt_font_info`).  The ledger `docs/ports/sfntfont.md`
// carries P-86 ②'s C line-range table for C 1-4222 (the transcribed rows and the remaining one),
// the `## Evidence` readings and the `## Deviations`.
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
/* `INT_MAX` is `lisp.h` 33's `#include <limits.h>`; `include/lisp.rs` does not carry that edge, so
   it is here, at the `#include "lisp.h"` that supplies it (the `androidfns.rs` 58 shape; slice 3's
   `sfntfont_lookup_char` is the user).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/limits.in.rs");             // lisp.h 33's `#include <limits.h>` (INT_MAX)
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

/* ----------------------------------------------------------------------------------------------
   C 1427-1604: the cmap reader and the character lookup, in C's order - C 1427-1430's comment and
   `sfntfont_read_cmap` (C 1431-1496), C 1498-1501's comment and `sfntfont_glyph_valid`
   (C 1502-1515), C 1517-1522's comment and `sfntfont_lookup_char` (C 1524-1585), and C 1587-1594's
   comment and `sfntfont_registries_compatible_p` (C 1596-1603).  **The next live definition is
   C 1613's `sfntfont_list_1`** (whose own comment is C 1605-1611), and the ledger's `## C line
   ranges` names both boundaries.  `emacs_open`/`emacs_close`/`lseek`/`xfree` come from the
   declarations and the `unistd.in.rs` edge slice 1's include graph already carries; `char_table_ref`
   (lisp.h 615) and `sfnt_lookup_glyph` (sfnt.h 1478-1479) are the `include/lisp.rs` 3833 and
   `include/sfnt.rs` 2220 extern blocks; `ENCODE_CHAR!`/`CHARSET_INVALID_CODE!` are
   `include/charset.rs`'s (charset.h), and `INT_MAX` is the `limits.in.rs` edge added at C 25.
   ---------------------------------------------------------------------------------------------- */
/// C 1431-1496: `sfntfont_read_cmap`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_read_cmap (desc: *mut sfnt_font_desc,
			      cmap: *mut *mut sfnt_cmap_encoding_subtable_data,
			      subtable: *mut sfnt_cmap_encoding_subtable)
{                                                          // C 1432-1496
  let mut font: *mut sfnt_offset_subtable;
  /* C 1437's `subtables` and C 1438's `data` are written by `sfnt_read_cmap_table` below, so both
     are zero-initialised (PORTING.md 1.5; Deviation 10's class, the `sfnt_grok_registry` shape).
      */
  let mut subtables: *mut sfnt_cmap_encoding_subtable = std::ptr::null_mut ();
  let mut data: *mut *mut sfnt_cmap_encoding_subtable_data = std::ptr::null_mut ();
  let mut table: *mut sfnt_cmap_table;
  let mut fd: c_int;
  let mut i: c_int;

  /* Pick a character map and place it in *CMAP.  */
  fd = emacs_open ((*desc).path, O_RDONLY, 0);             // C 1443

  if fd < 0
    { return; }                                            // C 1446

  /* Seek to the start of the font itself within its collection.  */

  if (*desc).offset != 0
     && lseek (fd, (*desc).offset, SEEK_SET) != (*desc).offset   // C 1450-1451
    {
      emacs_close (fd);
      return;
    }

  font = sfnt_read_table_directory (fd);                   // C 1457

  /* Return if FONT is a TrueType collection: the file pointer should
     already have been moved to the start of the table directory if
     so.  */

  if font.is_null () || font == usize::MAX as *mut sfnt_offset_subtable   // C 1463
    {
      emacs_close (fd);
      return;
    }

  table = sfnt_read_cmap_table (fd, font, &raw mut subtables, &raw mut data);   // C 1469
  xfree (font as *mut c_void);                             // C 1471

  if table.is_null ()
    {
      emacs_close (fd);
      return;
    }

  /* Now pick the best character map.  */

  *cmap = sfntfont_select_cmap (table, subtables, data, subtable,
				std::ptr::null_mut ());        // C 1481-1482

  /* Free the cmap data.  */

  i = 0;
  while i < (*table).num_subtables as c_int                // C 1486
    {
      if *(data.offset (i as isize)) != *cmap
	{ xfree (*(data.offset (i as isize)) as *mut c_void); }   // C 1489
      i += 1;
    }

  xfree (data as *mut c_void);                             // C 1492
  xfree (subtables as *mut c_void);                        // C 1493
  xfree (table as *mut c_void);                            // C 1494
  emacs_close (fd);                                        // C 1495
}

/// C 1502-1515: `sfntfont_glyph_valid`.  C's `sfnt_glyph` local is the *scalar typedef*
/// (`unsigned int`), which this port spells `sfnt_glyph_index` because `sfnt_glyph` is
/// `struct sfnt_glyph`'s name here - the rename `docs/ports/macro-body-notes.tsv`'s
/// `sfnt_lookup_glyph_4` row records.  C's `glyph <= desc->num_glyphs` is an *unsigned*
/// comparison (the usual arithmetic conversions), so the port casts `num_glyphs` to `c_uint`
/// rather than the other way round.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_glyph_valid (desc: *mut sfnt_font_desc, font_character: sfnt_char,
				cmap: *mut sfnt_cmap_encoding_subtable_data) -> bool
{                                                          // C 1503-1515
  let glyph: sfnt_glyph_index;

  glyph = sfnt_lookup_glyph (font_character, cmap);        // C 1509

  if glyph == 0
    { return false; }                                      // C 1512

  return glyph <= (*desc).num_glyphs as c_uint;            // C 1514
}

/// C 1524-1585: `sfntfont_lookup_char` - the character cache and the cmap it falls back to.  The
/// two bits C names (`char_cache`'s nil test and C 1546's `desc->cmap_invalid`) are the mirror's
/// field and the `SFNT_FONT_DESC_CMAP_INVALID` mask (C 164's 1-bit member, Deviation 14), and
/// C 1543's `?:` is Rust's `if`/`else` (Deviation 9).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_lookup_char (desc: *mut sfnt_font_desc, character: Lisp_Object,
				cmap: *mut *mut sfnt_cmap_encoding_subtable_data,
				subtable: *mut sfnt_cmap_encoding_subtable) -> bool
{                                                          // C 1525-1585
  let cached: Lisp_Object;
  let font_character: sfnt_char;
  let charset: *mut charset;
  let present: bool;

  /* Return false for characters that don't fit in a char table.  */
  if XFIXNUM (character) > INT_MAX as EMACS_INT || XFIXNUM (character) < 0
    { return false; }                                      // C 1536

  if !NILP ((*desc).char_cache)
    {
      cached = char_table_ref ((*desc).char_cache,
			       XFIXNUM (character) as c_int);   // C 1540-1541
      if !NILP (cached)
	{ return if EQ (cached, Qlambda ()) { false } else { true }; }   // C 1543
    }

  if (*cmap).is_null () && (*desc).bits & SFNT_FONT_DESC_CMAP_INVALID == 0
    { sfntfont_read_cmap (desc, cmap, subtable); }         // C 1547

  /* Check that a cmap is now present.  */
  if (*cmap).is_null ()
    {
      /* Opening the cmap failed.  Set desc->cmap_invalid to avoid
	 opening it again.  */
      (*desc).bits |= SFNT_FONT_DESC_CMAP_INVALID;         // C 1554
      return false;                                        // C 1555
    }

  /* Otherwise, encode the character.  */

  charset = sfntfont_charset_for_cmap (*subtable);         // C 1560
  if charset.is_null ()
    /* Emacs missing charsets? */
    { return false; }                                      // C 1563

  font_character = ENCODE_CHAR! (charset, XFIXNUM (character) as c_int);   // C 1565

  if font_character == CHARSET_INVALID_CODE! (charset)
    { return false; }                                      // C 1568

  /* Now return whether or not the glyph is present.  Noto Sans
     Georgian comes with a corrupt format 4 cmap table that somehow
     tries to express glyphs greater than 65565.  */
  present = sfntfont_glyph_valid (desc, font_character, *cmap);   // C 1573

  /* Cache the result.  Store Qlambda when not present, Qt
     otherwise.  */

  if NILP ((*desc).char_cache)
    { (*desc).char_cache = Fmake_char_table (Qfont_lookup_cache (), Qnil ()); }   // C 1579-1580

  Fset_char_table_range ((*desc).char_cache, character,
			 if present { Qt () } else { Qlambda () });   // C 1582-1583
  return present;                                          // C 1584
}

/// C 1596-1603: `sfntfont_registries_compatible_p`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_registries_compatible_p (a: Lisp_Object, b: Lisp_Object) -> bool
{                                                          // C 1597-1603
  if EQ (a, Qiso8859_1 ()) && EQ (b, Qiso10646_1 ())
    { return true; }                                       // C 1600

  return EQ (a, b);                                        // C 1602
}

/* ----------------------------------------------------------------------------------------------
   C 1605-1871: `sfntfont_list_1` (C 1613-1824, under C 1605-1611's comment), the two file-scope
   statics it sits between (C 1826-1830) and `sfntfont_registry_for_desc` (C 1841-1871) with its
   C 1832-1839 comment.  **The next live definition is C 1879's `sfntfont_desc_to_entity`** (whose
   comment is C 1873-1877), and the ledger's `## C line ranges` names both boundaries.  C's
   `FOR_EACH_TAIL_SAFE` is `include/lisp.rs` 4389's macro and its `break`/`continue` are the loop's
   (the `fns.rs` 375 shape); `goto fail` is the labelled block below (PORTING.md 2.2).
   ---------------------------------------------------------------------------------------------- */
/* Return whether or not the font description DESC satisfactorily
   matches the font specification FONT_SPEC.

   Value is 0 if there is no match, -1 if there is a match against
   DESC itself, and the number of matching instances if the style
   matches one or more instances defined in DESC.  Return the index
   of each matching instance in INSTANCES; it should be SIZE big.  */

/// C 1613-1824: `sfntfont_list_1`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_list_1 (desc: *mut sfnt_font_desc, spec: Lisp_Object,
			   instances: *mut c_int, size: c_int) -> c_int
{                                                          // C 1614-1824
  let mut tem: Lisp_Object;
  let mut extra: Lisp_Object;
  let mut tail: Lisp_Object;
  /* C 1618's `cmap` is C 1754's `cmap = NULL;`; the initialiser is that NULL.  */
  let mut cmap: *mut sfnt_cmap_encoding_subtable_data = std::ptr::null_mut ();
  let mut i: size_t;
  /* C 1620's `struct sfnt_cmap_encoding_subtable subtable;` is uninitialised in C and is copied
     into `desc->subtable` at C 1815 only when a cmap was read; zero-initialised here (PORTING.md
     1.5, Deviation 10).  */
  let mut subtable: sfnt_cmap_encoding_subtable = std::mem::zeroed ();
  let mut instance: c_int;
  let mut num_instance: c_int;
  let mut item: Lisp_Object;
  let mut matching: bool;

  /* cmap and subtable are caches for sfntfont_lookup_char.  */

  /* Check that the family name in SPEC matches DESC->family if it is
     specified.  */

  tem = AREF (spec, FONT_FAMILY_INDEX as isize);           // C 1630

  /* If TEM is a family listed in Vsfnt_default_family_alist,
     then use that instead.  */

  if SYMBOLP (tem) && CONSP (Vsfnt_default_family_alist)   // C 1635
    {
      tail = Vsfnt_default_family_alist;
      FOR_EACH_TAIL_SAFE! (tail, {
	  if !CONSP (XCAR (tail))
	    { continue; }                                  // C 1641

	  if STRINGP (XCAR (XCAR (tail)))
	     && STRINGP (XCDR (XCAR (tail)))
	     && !NILP (Fstring_equal (SYMBOL_NAME (tem),
				      XCAR (XCAR (tail))))  // C 1645-1646
	    {
	      /* Special family found.  */
	      tem = Fintern (XCDR (XCAR (tail)), Qnil ());  // C 1649
	      break;
	    }
      });
    }

  if !NILP (tem) && NILP (Fstring_equal (SYMBOL_NAME (tem),
					 (*desc).family))   // C 1655-1656
    { return 0; }

  instance = -1;                                           // C 1659

  /* If a registry is set and wrong, then reject the font desc
     immediately.  This detects 50% of mismatches from fontset.c.

     If DESC->registry is nil, then the registry couldn't be
     determined beforehand.  */

  tem = AREF (spec, FONT_REGISTRY_INDEX as isize);         // C 1667
  if !NILP (tem) && !NILP ((*desc).registry)
     && !sfntfont_registries_compatible_p (tem, (*desc).registry)
    { return 0; }                                          // C 1670

  /* If the font spacings disagree, reject this font also.  */

  tem = AREF (spec, FONT_SPACING_INDEX as isize);          // C 1674
  if FIXNUMP (tem) && (XFIXNUM (tem) != (*desc).spacing as EMACS_INT)
    { return 0; }                                          // C 1676

  /* Check the style.  If DESC is a fixed font, just check once.
     Otherwise, check each instance.  */

  'fail: {
  if NILP ((*desc).instances)                              // C 1681
    {
      tem = AREF (spec, FONT_ADSTYLE_INDEX as isize);      // C 1683
      if !NILP (tem) && !EQ (tem, (*desc).adstyle)
	{ return 0; }                                      // C 1685

      /* C compares the macro's `long` with the record's `int` member after the usual arithmetic
	 conversions, so the port widens the member rather than narrowing the macro (Deviation
	 19).  */
      if FONT_WIDTH_NUMERIC! (spec) != -1
	 && FONT_WIDTH_NUMERIC! (spec) != (*desc).width as isize
	{ return 0; }                                      // C 1689

      if FONT_WEIGHT_NUMERIC! (spec) != -1
	 && FONT_WEIGHT_NUMERIC! (spec) != (*desc).weight as isize
	{ return 0; }                                      // C 1693

      if FONT_SLANT_NUMERIC! (spec) != -1
	 && FONT_SLANT_NUMERIC! (spec) != (*desc).slant as isize
	{ return 0; }                                      // C 1697
    }
  else
    {
      num_instance = 0;                                    // C 1701

      /* Find the indices of instances in this distortable font which
	 match the given font spec.  */

      i = 0;
      while i < ASIZE ((*desc).instances) as usize         // C 1706
	{
	  item = AREF ((*desc).instances, i as isize);     // C 1708

	  if NILP (item)
	    { i += 1; continue; }                          // C 1711

	  /* Check that the adstyle specified matches.  */

	  tem = AREF (spec, FONT_ADSTYLE_INDEX as isize);  // C 1715
	  if !NILP (tem) && NILP (Fequal (tem, AREF (item, 1)))
	    { i += 1; continue; }                          // C 1717

	  /* Check the style.  */

	  if FONT_WIDTH_NUMERIC! (spec) != -1
	     && (FONT_WIDTH_NUMERIC! (spec)
		 != XFIXNUM (AREF (item, 2)))              // C 1721-1723
	    { i += 1; continue; }

	  if FONT_WEIGHT_NUMERIC! (spec) != -1
	     && (FONT_WEIGHT_NUMERIC! (spec)
		 != XFIXNUM (AREF (item, 3)))              // C 1726-1728
	    { i += 1; continue; }

	  if FONT_SLANT_NUMERIC! (spec) != -1
	     && (FONT_SLANT_NUMERIC! (spec)
		 != XFIXNUM (AREF (item, 4)))              // C 1731-1733
	    { i += 1; continue; }

	  if num_instance == size
	    { break; }                                     // C 1737

	  /* A matching instance has been found.  Set its index, then
	     go back to the rest of the font matching.  */
	  *instances.offset (num_instance as isize) = i as c_int;   // C 1741
	  num_instance += 1;                               /* C 1741's `num_instance++`.  */
	  i += 1;
	}

      instance = num_instance;                             // C 1744
    }

  /* Handle extras.  */
  extra = AREF (spec, FONT_EXTRA_INDEX as isize);          // C 1748

  if NILP (extra)
    { return instance; }                                   // C 1751

  tem = assq_no_quit (QCscript (), extra);                 // C 1753
  cmap = std::ptr::null_mut ();                            // C 1754

  if !NILP (tem)
    {
      /* If a script has been specified, look up its representative
	 characters and see if they are present in the font.  This
	 requires reading the cmap.  */
      tem = assq_no_quit (XCDR (tem), Vscript_representative_chars);   // C 1761

      if CONSP (tem) && VECTORP (XCDR (tem))               // C 1763
	{
	  tem = XCDR (tem);                                // C 1765

	  /* The vector contains characters, of which one must be
	     present in the font.  */
	  matching = false;                                // C 1769
	  i = 0;
	  while i < ASIZE (tem) as usize                   // C 1770
	    {
	      if FIXNUMP (AREF (tem, i as isize))          // C 1772
		{
		  if sfntfont_lookup_char (desc, AREF (tem, i as isize),
					   &raw mut cmap, &raw mut subtable)   // C 1774-1775
		    {
		      matching = true;                     // C 1777
		      break;
		    }
		}
	      i += 1;
	    }
	  if !matching
	    { break 'fail; }                               // C 1783: `goto fail;`.  */
	}
      else if CONSP (tem) && CONSP (XCDR (tem))            // C 1785
	{
	  tem = XCDR (tem);                                // C 1787

	  /* tem is a list of each characters, all of which must be
	     present in the font.  */
	  FOR_EACH_TAIL_SAFE! (tem, {
	      if FIXNUMP (XCAR (tem))
		 && !sfntfont_lookup_char (desc, XCAR (tem), &raw mut cmap,
					   &raw mut subtable)   // C 1793-1795
		{ break 'fail; }                           // C 1796: `goto fail;`.  */
	  });

	  /* One or more characters are missing.  */
	  if !NILP (tem)
	    { break 'fail; }                               // C 1801: `goto fail;`.  */
	}
      /* Fail if there are no matching fonts at all.  */
      else if NILP (tem)
	{ break 'fail; }                                   // C 1805: `goto fail;`.  */
    }

  /* Now check that the language is supported.  */
  tem = assq_no_quit (QClang (), extra);                   // C 1809
  if !NILP (tem) && NILP (Fmemq (tem, (*desc).languages))
    { break 'fail; }                                       // C 1811: `goto fail;`.  */

  /* Set desc->subtable if cmap was specified.  */
  if !cmap.is_null ()                                      // C 1814
    { (*desc).subtable = subtable; }                       // C 1815

  xfree (cmap as *mut c_void);                             // C 1817
  return instance;                                         // C 1818
  }                                                        /* C 1820: `fail:`.  */

  /* The cmap might've been read in and require deallocation.  */
  xfree (cmap as *mut c_void);                             // C 1822
  return 0;                                                // C 1823
}

/* Type of font entities and font objects created.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_vendor_name: Lisp_Object = std::ptr::null_mut ();        // C 1827

/* Font driver used in font objects created.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_font_driver: *const font_driver = std::ptr::null ();     // C 1830

/* Return the font registry corresponding to the font descriptor DESC.
   Under X, the font registry is an atom registered with the Open
   Group uniquely identifying the organization which defines the
   font's character set.

   In practice, the registry overlaps with the character set itself.
   So Emacs just uses the ``registry'' field to represent both
   instead.  */

/// C 1841-1871: `sfntfont_registry_for_desc`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_registry_for_desc (desc: *mut sfnt_font_desc) -> Lisp_Object
{                                                          // C 1842-1871
  let mut cmap: *mut sfnt_cmap_encoding_subtable_data;

  cmap = std::ptr::null_mut ();                            // C 1846

  if (*desc).bits & SFNT_FONT_DESC_CMAP_INVALID != 0       // C 1848
    { return Qnil (); }

  if (*desc).subtable.platform_id == 500                   // C 1851
    {
      /* Read in the cmap to determine the registry.  */
      sfntfont_read_cmap (desc, &raw mut cmap, &raw mut (*desc).subtable);   // C 1854

      if cmap.is_null ()                                   // C 1856
	{
	  (*desc).bits |= SFNT_FONT_DESC_CMAP_INVALID;     // C 1858
	  return Qnil ();                                  // C 1859
	}
    }

  xfree (cmap as *mut c_void);                             // C 1863

  if (*desc).subtable.platform_id != 500                   // C 1865
    /* desc->subtable.platform_id is now set.  CMAP is already free,
       because it is not actually used.  */
    { return sfnt_registry_for_subtable (&raw mut (*desc).subtable); }   // C 1868

  return Qnil ();                                          // C 1870
}

/* `font_make_entity` (C 1883) and `font_style_to_value` (C 1904's `FONT_SET_STYLE` macro) are two
   names `font.h` declares and no mirror carries; both are declared here at their use sites (the
   `src/ftfont.rs` 1306 and `src/haikufont.rs` 127 shapes, P-91 4).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: font.h 842: `extern Lisp_Object font_make_entity (void);` (font.c defines it).
  pub fn font_make_entity () -> Lisp_Object;
  /// C: font.h 854: `extern int font_style_to_value (enum font_property_index prop, Lisp_Object
  /// name, bool noerror);`
  pub fn font_style_to_value (prop: font_property_index, name: Lisp_Object, noerror: bool)
    -> c_int;
}

/* ----------------------------------------------------------------------------------------------
   C 1873-2050: the entity builder and the two driver entry points, in C's order - C 1873-1877's
   comment and `sfntfont_desc_to_entity` (C 1879-1933), C 1935-1937's comment and
   `sfntfont_compare_font_entities` (C 1939-1959), C 1961-1963's comment and the subr record
   `Scompare_font_entities` (C 1964-1971), C 1973-1976's comment and `sfntfont_list` (C 1977-2027),
   C 2029-2031's comment and `sfntfont_match` (C 2032-2042), and C 2044's form feed with C 2046-2050's
   anonymous enum (the two cache sizes).  **The next live definition is C 2108's
   `sfntfont_get_glyph`** (the caching subsystem's comment is C 2052-2058); the ledger's `## C line
   ranges` names both boundaries.  `font_make_entity` is the one name of this slice no mirror
   carries - it is declared at its use site below (the `ftfont.rs` 1306 shape, font.h 842) - and
   `Scompare_font_entities` is the `Aligned_Lisp_Subr` initialiser `alloc.rs` 8175 spells.
   ---------------------------------------------------------------------------------------------- */
/// C 1879-1933: `sfntfont_desc_to_entity` - C's `static Lisp_Object`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_desc_to_entity (desc: *mut sfnt_font_desc, instance: c_int) -> Lisp_Object
{                                                          // C 1880-1933
  let mut entity: Lisp_Object;
  let mut vector: Lisp_Object;

  entity = font_make_entity ();                            // C 1883

  ASET (entity, FONT_TYPE_INDEX as isize, sfnt_vendor_name);   // C 1885
  ASET (entity, FONT_FOUNDRY_INDEX as isize, (*desc).designer);   // C 1886
  ASET (entity, FONT_FAMILY_INDEX as isize, Fintern ((*desc).family, Qnil ()));   // C 1887
  ASET (entity, FONT_ADSTYLE_INDEX as isize, Qnil ());     // C 1888
  ASET (entity, FONT_REGISTRY_INDEX as isize,
	sfntfont_registry_for_desc (desc));                // C 1889-1890

  /* Size of 0 means the font is scalable.  */
  ASET (entity, FONT_SIZE_INDEX as isize, make_fixnum (0));   // C 1893
  ASET (entity, FONT_AVGWIDTH_INDEX as isize, make_fixnum (0));   // C 1894
  ASET (entity, FONT_SPACING_INDEX as isize,
	make_fixnum ((*desc).spacing as EMACS_INT));       // C 1895

  if instance >= 1
    {
      if NILP ((*desc).instances)
	 || (instance as isize) > ASIZE ((*desc).instances)   // C 1899-1900
	{ emacs_abort (); }                                // C 1901

      vector = AREF ((*desc).instances, (instance - 1) as isize);   // C 1903
      FONT_SET_STYLE! (entity, FONT_WIDTH_INDEX,
		       AREF (vector, 2));                  // C 1904-1905
      FONT_SET_STYLE! (entity, FONT_WEIGHT_INDEX,
		       AREF (vector, 3));                  // C 1906-1907
      FONT_SET_STYLE! (entity, FONT_SLANT_INDEX,
		       AREF (vector, 4));                  // C 1908-1909
      ASET (entity, FONT_ADSTYLE_INDEX as isize, AREF (vector, 1));   // C 1910
    }
  else
    {
      FONT_SET_STYLE! (entity, FONT_WIDTH_INDEX,
		       make_fixnum ((*desc).width as EMACS_INT));   // C 1914-1915
      FONT_SET_STYLE! (entity, FONT_WEIGHT_INDEX,
		       make_fixnum ((*desc).weight as EMACS_INT));   // C 1916-1917
      FONT_SET_STYLE! (entity, FONT_SLANT_INDEX,
		       make_fixnum ((*desc).slant as EMACS_INT));   // C 1918-1919
      ASET (entity, FONT_ADSTYLE_INDEX as isize, (*desc).adstyle);   // C 1920
    }

  /* Set FONT_EXTRA_INDEX to a pointer to the font description.  Font
     descriptions are never supposed to be freed.  */

  ASET (entity, FONT_EXTRA_INDEX as isize,
	/* C 1927-1930's `instance >= 1 ? … : …` is the `if`/`else` expression (Deviation 9).
	   */
	if instance >= 1
	{ list2 (Fcons (Qfont_entity (), make_mint_ptr (desc as *mut c_void)),
		 Fcons (Qfont_instance (), make_fixnum ((instance - 1) as EMACS_INT))) }
	else
	{ list1 (Fcons (Qfont_entity (), make_mint_ptr (desc as *mut c_void))) });

  return entity;                                           // C 1932
}

/// C 1939-1959: `sfntfont_compare_font_entities` - C's `static`, and its address is the subr
/// record's below, so it carries the C ABI (P-52, the `ftfont_list` shape).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfntfont_compare_font_entities (a: Lisp_Object, b: Lisp_Object)
  -> Lisp_Object
{                                                          // C 1940-1959
  let mut count_a: ptrdiff_t;
  let mut count_b: ptrdiff_t;
  let mut i: ptrdiff_t;

  count_a = 0;                                             // C 1943
  count_b = 0;                                             // C 1944

  i = 0;
  while i < FONT_ENTITY_MAX as ptrdiff_t                   // C 1946
    {
      if !NILP (AREF (a, i))
	{ count_a += 1; }                                  // C 1949
      i += 1;
    }

  i = 0;
  while i < FONT_ENTITY_MAX as ptrdiff_t                   // C 1952
    {
      if !NILP (AREF (b, i))
	{ count_b += 1; }                                  // C 1955
      i += 1;
    }

  return if count_a < count_b { Qt () } else { Qnil () };  // C 1958
}

/* Function that compares two font entities to return whether fewer
   fields are set within the first than in the second.  */

/// C 1964-1971: `static union Aligned_Lisp_Subr Scompare_font_entities` - C's record, whose
/// `defsubr` call lives in the unported `syms_of_sfntfont` (C 4142) and which `sfntfont_list`
/// below hands to `Fsort` through `XSETSUBR`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut Scompare_font_entities: Aligned_Lisp_Subr =
  Aligned_Lisp_Subr { s: Lisp_Subr {
    header: vectorlike_header {
      size: PSEUDOVECTOR_FLAG
	| ((pvec_type::PVEC_SUBR as isize) << PSEUDOVECTOR_AREA_BITS),
    },
    function: Lisp_Subr_function {
      a2: subr_function_pointer! (Lisp_Object, sfntfont_compare_font_entities,
				  (Lisp_Object, Lisp_Object)),
    },
    min_args: 2, max_args: 2,
    symbol_name: c"sfntfont_compare_font_entities".as_ptr (),
    intspec: Lisp_Subr_intspec { string: std::ptr::null () },
    command_modes: lisp_h_Qnil (),
    doc: 0,
    #[cfg (feature = "have_native_comp")]
    native_comp_u: lisp_h_Qnil (),
    #[cfg (feature = "have_native_comp")]
    native_c_name: std::ptr::null_mut (),
    #[cfg (feature = "have_native_comp")]
    lambda_list: lisp_h_Qnil (),
    #[cfg (feature = "have_native_comp")]
    r#type: lisp_h_Qnil (),
  } };

/* Return a list of font-entities matching the specified
   FONT_SPEC.  */

/// C 1977-2027: `sfntfont_list` - C's non-`static` driver entry point.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfntfont_list (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{                                                          // C 1978-2027
  let mut matching: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut compare_font_entities: Lisp_Object;
  let mut desc: *mut sfnt_font_desc;
  let mut i: c_int;
  let mut rc: c_int;
  /* C 1981's `int instances[100]` is written by `sfntfont_list_1` and only read as far as that
     call's return value says, so the zeroing is PORTING.md 1.5's / Deviation 10's class.  */
  let mut instances: [c_int; 100] = [0; 100];

  matching = Qnil ();                                      // C 1983

  block_input ();                                          // C 1985
  /* Returning irrelevant results on receiving an OTF form will cause
     fontset.c to loop over and over, making displaying some
     characters very slow.  */
  tem = assq_no_quit (QCotf (), AREF (font_spec, FONT_EXTRA_INDEX as isize));   // C 1989
  if CONSP (tem) && !NILP (XCDR (tem))
    {
      unblock_input ();                                    // C 1992
      return Qnil ();                                      // C 1993
    }

  /* Loop through known system fonts and add them one-by-one.  */

  desc = system_fonts;                                     // C 1998
  while !desc.is_null ()
    {
      rc = sfntfont_list_1 (desc, font_spec, &raw mut instances[0],
			    countof! (instances) as c_int);   // C 2000-2001

      if rc < 0
	{ matching = Fcons (sfntfont_desc_to_entity (desc, 0),
			    matching); }                   // C 2004-2005
      else if rc != 0
	{
	  /* Add each matching instance.  */

	  i = 0;
	  while i < rc                                      // C 2010
	    {
	      matching = Fcons (sfntfont_desc_to_entity (desc,
							 instances[i as usize] + 1),   // C 2011-2012
				matching);
	      i += 1;
	    }
	}

      desc = (*desc).next;                                 // C 1998's `desc = desc->next`.
    }
  unblock_input ();                                        // C 2016

  /* Sort matching by the number of fields set inside each element, so
     that values of FONT_SPECs that leave a number of fields
     unspecified will yield a list with the closest matches (that is
     to say, those whose fields are precisely as specified by the
     caller) ordered first.  */

  XSETSUBR! (compare_font_entities, &raw mut Scompare_font_entities.s);   // C 2024
  matching = CALLN! (Fsort, matching, compare_font_entities);   // C 2025
  return matching;                                         // C 2026
}

/* Return the first font-entity matching the specified FONT_SPEC.  */

/// C 2032-2042: `sfntfont_match` - C's non-`static` driver entry point.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfntfont_match (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{                                                          // C 2033-2042
  let mut matches: Lisp_Object;

  matches = sfntfont_list (f, font_spec);                  // C 2036

  if !NILP (matches)
    { return XCAR (matches); }                             // C 2039

  return Qnil ();                                          // C 2041
}

/* C 2044: the form feed.  C 2046-2050 is the file's anonymous enum - two cache sizes, both
   `int` (the constants the caching subsystem below sizes its tables with).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const SFNT_OUTLINE_CACHE_SIZE: c_int = 256;            // C 2048
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const SFNT_RASTER_CACHE_SIZE: c_int = 128;             // C 2049

/* ----------------------------------------------------------------------------------------------
   C 2051-2172: the caching subsystem's records and its four helpers, in C's order - C 2052-2058's
   comment, `struct sfnt_outline_cache` (C 2059-2072), `struct sfnt_raster_cache` (C 2074-2084),
   `struct sfntfont_get_glyph_outline_dcontext` (C 2086-2102), and then `sfntfont_get_glyph`
   (C 2108-2134), `sfntfont_free_glyph` (C 2139-2142), `sfntfont_get_metrics` (C 2150-2158) and
   `sfntfont_dereference_outline` (C 2164-2172), each with its own comment.  **The next live
   definition is C 2198's `sfntfont_get_glyph_outline`** (its comment is C 2174-2196), and the
   ledger's `## C line ranges` names both boundaries.

   The three records' layouts are **measured** (`clang -std=gnu23 -O0`, the frozen configuration,
   Evidence ⑮), not remembered (P-88 ②), and `sfnt_glyph`'s scalar typedef is `sfnt_glyph_index`
   here (Deviation 18 (a)).  Three of the four helpers have their address taken - C 2313-2314 and
   C 2355-2357 hand them to the interpret functions - so those carry the C ABI (P-52); only
   `sfntfont_dereference_outline` is called by name alone (C 2400/2431/3537/3726).
   ---------------------------------------------------------------------------------------------- */
/// C 2059-2072: `struct sfnt_outline_cache`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct sfnt_outline_cache                       // C 2059-2072
{
  /* Next and last cache buckets.  */
  pub next: *mut sfnt_outline_cache,                // C 2062
  pub last: *mut sfnt_outline_cache,                // C 2062's second declarator

  /* Pointer to outline.  */
  pub outline: *mut sfnt_glyph_outline,             // C 2065

  /* Reference to glyph metrics.  */
  pub metrics: sfnt_glyph_metrics,                  // C 2068

  /* What glyph this caches.  */
  pub glyph: sfnt_glyph_index,                      // C 2071 (the `sfnt_glyph` typedef, Deviation 18 (a))
}

/* clang, frozen configuration: `sizeof 40`, `_Alignof 8`, `next` 0, `last` 8, `outline` 16,
   `metrics` 24 (its own 8 bytes), `glyph` 32.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  core::assert! (std::mem::size_of::<sfnt_outline_cache> () == 40);
  core::assert! (std::mem::align_of::<sfnt_outline_cache> () == 8);
  core::assert! (std::mem::offset_of! (sfnt_outline_cache, next) == 0);
  core::assert! (std::mem::offset_of! (sfnt_outline_cache, last) == 8);
  core::assert! (std::mem::offset_of! (sfnt_outline_cache, outline) == 16);
  core::assert! (std::mem::offset_of! (sfnt_outline_cache, metrics) == 24);
  core::assert! (std::mem::offset_of! (sfnt_outline_cache, glyph) == 32);
};

/// C 2074-2084: `struct sfnt_raster_cache`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct sfnt_raster_cache                        // C 2074-2084
{
  /* Next and last cache buckets.  */
  pub next: *mut sfnt_raster_cache,                 // C 2077
  pub last: *mut sfnt_raster_cache,                 // C 2077's second declarator

  /* Pointer to raster.  */
  pub raster: *mut sfnt_raster,                     // C 2080

  /* What glyph this caches.  */
  pub glyph: sfnt_glyph_index,                      // C 2083 (the `sfnt_glyph` typedef)
}

/* clang, frozen configuration: `sizeof 32`, `_Alignof 8`, `next` 0, `last` 8, `raster` 16,
   `glyph` 24.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  core::assert! (std::mem::size_of::<sfnt_raster_cache> () == 32);
  core::assert! (std::mem::align_of::<sfnt_raster_cache> () == 8);
  core::assert! (std::mem::offset_of! (sfnt_raster_cache, next) == 0);
  core::assert! (std::mem::offset_of! (sfnt_raster_cache, last) == 8);
  core::assert! (std::mem::offset_of! (sfnt_raster_cache, raster) == 16);
  core::assert! (std::mem::offset_of! (sfnt_raster_cache, glyph) == 24);
};

/// C 2086-2102: `struct sfntfont_get_glyph_outline_dcontext`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct sfntfont_get_glyph_outline_dcontext      // C 2086-2102
{
  /* Long and short loca tables.  */
  pub loca_long: *mut sfnt_loca_table_long,         // C 2089
  pub loca_short: *mut sfnt_loca_table_short,       // C 2090

  /* glyf table.  */
  pub glyf: *mut sfnt_glyf_table,                   // C 2093

  /* hmtx, hhea and maxp tables utilized to acquire glyph metrics.  */
  pub hmtx: *mut sfnt_hmtx_table,                   // C 2096
  pub hhea: *mut sfnt_hhea_table,                   // C 2097
  pub maxp: *mut sfnt_maxp_table,                   // C 2098

  /* Variation settings, or NULL.  */
  pub blend: *mut sfnt_blend,                       // C 2101
}

/* clang, frozen configuration: `sizeof 56`, `_Alignof 8`, and the seven pointers at 0, 8, 16, 24,
   32, 40 and 48 in C's order.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  core::assert! (std::mem::size_of::<sfntfont_get_glyph_outline_dcontext> () == 56);
  core::assert! (std::mem::align_of::<sfntfont_get_glyph_outline_dcontext> () == 8);
  core::assert! (std::mem::offset_of! (sfntfont_get_glyph_outline_dcontext, loca_long) == 0);
  core::assert! (std::mem::offset_of! (sfntfont_get_glyph_outline_dcontext, loca_short) == 8);
  core::assert! (std::mem::offset_of! (sfntfont_get_glyph_outline_dcontext, glyf) == 16);
  core::assert! (std::mem::offset_of! (sfntfont_get_glyph_outline_dcontext, hmtx) == 24);
  core::assert! (std::mem::offset_of! (sfntfont_get_glyph_outline_dcontext, hhea) == 32);
  core::assert! (std::mem::offset_of! (sfntfont_get_glyph_outline_dcontext, maxp) == 40);
  core::assert! (std::mem::offset_of! (sfntfont_get_glyph_outline_dcontext, blend) == 48);
};

/// C 2108-2134: `sfntfont_get_glyph` - C's `static`, and C 2313/C 2355 hand it to the interpreter
/// as a callback, so it carries the C ABI (P-52).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfntfont_get_glyph (glyph_id: sfnt_glyph_index, dcontext: *mut c_void,
					 need_free: *mut bool) -> *mut sfnt_glyph
{                                                          // C 2108-2134
  let mut tables: *mut sfntfont_get_glyph_outline_dcontext;
  let mut glyph: *mut sfnt_glyph;
  /* C 2113's `distortion` is uninitialised and is only ever passed by address to the two
     variation functions; zero-initialised here (PORTING.md 1.5, Deviation 10).  */
  let mut distortion: sfnt_metrics_distortion = std::mem::zeroed ();

  tables = dcontext as *mut sfntfont_get_glyph_outline_dcontext;   // C 2115
  *need_free = true;                                       // C 2116

  glyph = sfnt_read_glyph (glyph_id, (*tables).glyf,
			   (*tables).loca_short,
			   (*tables).loca_long);           // C 2118-2120

  if !(*tables).blend.is_null () && !glyph.is_null ()       // C 2122
    {
      /* C 2124's `glyph->simple` is a *pointer* (`sfnt.h` 677), so C's truth test is the null
	 test here (Deviation 21 (a)).  */
      if !(*glyph).simple.is_null ()
	{ sfnt_vary_simple_glyph ((*tables).blend, glyph_id, glyph,
				  &raw mut distortion); }  // C 2125-2126
      else
	{ sfnt_vary_compound_glyph ((*tables).blend, glyph_id, glyph,
				    &raw mut distortion); }   // C 2128-2129
    }

  /* Note that the distortion is not relevant for compound glyphs.  */
  return glyph;                                            // C 2133
}

/// C 2139-2142: `sfntfont_free_glyph` - C's `static`, and C 2314/C 2356 hand it to the
/// interpreter as a callback (P-52).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfntfont_free_glyph (glyph: *mut sfnt_glyph, dcontext: *mut c_void)
{                                                          // C 2139-2142
  sfnt_free_glyph (glyph);                                 // C 2141
}

/// C 2150-2158: `sfntfont_get_metrics` - C's `static`, and C 2357 hands it to the interpreter as a
/// callback (P-52).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn sfntfont_get_metrics (glyph: sfnt_glyph_index,
					   metrics: *mut sfnt_glyph_metrics,
					   dcontext: *mut c_void) -> c_int
{                                                          // C 2150-2158
  let mut tables: *mut sfntfont_get_glyph_outline_dcontext;

  tables = dcontext as *mut sfntfont_get_glyph_outline_dcontext;   // C 2155
  return sfnt_lookup_glyph_metrics (glyph, metrics, (*tables).hmtx,
				    (*tables).hhea, (*tables).maxp);   // C 2156-2157
}

/// C 2164-2172: `sfntfont_dereference_outline` - C's `static`, called by name only
/// (C 2400/2431/3537/3726), so it stays a plain `unsafe fn`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_dereference_outline (outline: *mut sfnt_glyph_outline)
{                                                          // C 2164-2172
  eassert! ((*outline).refcount > 0);                      // C 2166

  (*outline).refcount -= 1;                                // C 2168's `--outline->refcount`

  if (*outline).refcount != 0
    { return; }                                            // C 2169

  xfree (outline as *mut c_void);                          // C 2171
}

/* ----------------------------------------------------------------------------------------------
   C 2173-2559: the rest of the caching subsystem, in C's order - C 2174-2196's comment and
   `sfntfont_get_glyph_outline` (C 2198-2412, the ring cache's hit move, the instruction path with
   `goto fail`, the outline build and the LRU eviction), C 2414-2416's comment and
   `sfntfont_free_outline_cache` (C 2418-2437), C 2439-2441's comment and
   `sfntfont_dereference_raster` (C 2443-2451), C 2453-2456's comment and `sfntfont_get_glyph_raster`
   (C 2458-2534, which keeps C 2518's `SFNT_OUTLINE_CACHE_SIZE` test in the *raster* cache exactly
   as C writes it), and C 2536-2538's comment with `sfntfont_free_raster_cache` (C 2540-2559).
   **The next live definition is C 2633's `sfnt_open_tables`** (the "Opening fonts" banner is
   C 2563 and `struct sfnt_font_info` is C 2565-2631); the ledger's `## C line ranges` names both
   boundaries.  All five are `static` and every call site is by name (C 3512/3658-3659/3699/3722/
   3749), so none carries an address and none needs the C ABI (P-52).
   ---------------------------------------------------------------------------------------------- */
/// C 2198-2412: `sfntfont_get_glyph_outline`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_get_glyph_outline (glyph_code: sfnt_glyph_index,
				      cache: *mut sfnt_outline_cache,
				      scale: sfnt_fixed, cache_size: *mut c_int,
				      blend: *mut sfnt_blend,
				      index: c_int,
				      glyf: *mut sfnt_glyf_table,
				      head: *mut sfnt_head_table,
				      hmtx: *mut sfnt_hmtx_table,
				      hhea: *mut sfnt_hhea_table,
				      maxp: *mut sfnt_maxp_table,
				      loca_short: *mut sfnt_loca_table_short,
				      loca_long: *mut sfnt_loca_table_long,
				      interpreter: *mut sfnt_interpreter,
				      metrics: *mut sfnt_glyph_metrics,
				      state: *mut sfnt_graphics_state)
  -> *mut sfnt_glyph_outline
{                                                          // C 2198-2412
  let mut start: *mut sfnt_outline_cache;
  let mut outline: *mut sfnt_glyph_outline;
  let mut glyph: *mut sfnt_glyph;
  /* C 2217's `dcontext` is assigned field by field from C 2283 on; zero-initialised because Rust
     wants the whole binding before the first field write (PORTING.md 1.5, Deviation 10).  */
  let mut dcontext: sfntfont_get_glyph_outline_dcontext = std::mem::zeroed ();
  /* C 2218's `value` is the out-parameter of the two interpret calls and is read only after one of
     them assigned it; zero-initialised because its address is taken (PORTING.md 1.5, Deviation
     10).  C 2316/C 2358's `&dcontext` is the untyped `void *` in the mirror (P-72 (3)), so both
     sites cast.  */
  let mut value: *mut sfnt_instructed_outline = std::ptr::null_mut ();
  /* C 2219's `error` is assigned by both arms of C 2296's `if` before C 2319 reads it.  */
  let error: *const c_char;
  /* C 2220's `temp` is filled by `sfnt_lookup_glyph_metrics` (C 2293) and read from C 2305 on;
     C 2221's `distortion` gets `advance = 0` at C 2225 and its address is passed at C 2265/2272.  */
  let mut temp: sfnt_glyph_metrics = std::mem::zeroed ();
  let mut distortion: sfnt_metrics_distortion = std::mem::zeroed ();
  /* C 2222's `advance` is the out-parameter of C 2326's `sfnt_build_instructed_outline`;
     zero-initialised because its address is taken (PORTING.md 1.5, Deviation 10).  */
  let mut advance: sfnt_fixed = 0;

  start = (*cache).next;                                   // C 2224
  distortion.advance = 0;                                  // C 2225

  /* See if the outline is already cached.  */
  while start != cache                                     // C 2228
    {
      if (*start).glyph == glyph_code                      // C 2230
	{
	  /* Move start to the start of the ring.  Then increase
	     start->outline->refcount and return it.  */

	  (*(*start).last).next = (*start).next;             // C 2235
	  (*(*start).next).last = (*start).last;             // C 2236

	  (*start).next = (*cache).next;                     // C 2238
	  (*start).last = cache;                             // C 2239
	  (*(*start).next).last = start;                     // C 2240
	  (*(*start).last).next = start;                     // C 2241
	  (*((*start).outline)).refcount += 1;               // C 2242

	  if !metrics.is_null ()
	    { *metrics = (*start).metrics; }                 // C 2245

	  return (*start).outline;                           // C 2247
	}
      start = (*start).next;                                 // C 2228's step
    }

  /* Not already cached.  Get the glyph.  */
  glyph = sfnt_read_glyph (glyph_code, glyf,
			   loca_short, loca_long);           // C 2252-2253

  if glyph.is_null ()
    { return std::ptr::null_mut (); }                      // C 2256

  /* Distort the glyph if necessary.  */

  if index != -1                                           // C 2260
    {
      if !(*glyph).simple.is_null ()                       // C 2262
	{
	  if sfnt_vary_simple_glyph (blend, glyph_code,
				     glyph, &raw mut distortion) != 0   // C 2264-2265
	    {
	      sfnt_free_glyph (glyph);                         // C 2267
	      return std::ptr::null_mut ();                    // C 2268
	    }
	}
      else if sfnt_vary_compound_glyph (blend, glyph_code,
					glyph, &raw mut distortion) != 0   // C 2271-2272
	{
	  sfnt_free_glyph (glyph);                             // C 2274
	  return std::ptr::null_mut ();                        // C 2275
	}
    }

  /* Try to instruct the glyph if INTERPRETER is specified.  */

  outline = std::ptr::null_mut ();                         // C 2281

  dcontext.loca_long = loca_long;                          // C 2283
  dcontext.loca_short = loca_short;                        // C 2284
  dcontext.glyf = glyf;                                    // C 2285
  dcontext.hhea = hhea;                                    // C 2286
  dcontext.hmtx = hmtx;                                    // C 2287
  dcontext.maxp = maxp;                                    // C 2288
  /* C 2289's `index != -1 ? blend : NULL` is the `if`/`else` expression (Deviation 9).  */
  dcontext.blend = if index != -1 { blend } else { std::ptr::null_mut () };

  /* Now load the glyph's unscaled metrics into TEMP.  */

  'fail: {
  if sfnt_lookup_glyph_metrics (glyph_code, &raw mut temp,
				hmtx, hhea, maxp) != 0    // C 2293
    { break 'fail; }                                       // C 2294: `goto fail;`.  */

  if !interpreter.is_null ()                               // C 2296
    {
      if !(*glyph).simple.is_null ()                       // C 2298
	{
	  /* Restore the interpreter state from the snapshot taken
	     after loading the preprogram.  */
	  (*interpreter).state = *state;                     // C 2302

	  error = sfnt_interpret_simple_glyph (glyph, interpreter,
					       &raw mut temp, &raw mut value);   // C 2304-2305
	}
      else
	/* Restoring the interpreter state is done by
	   sfnt_interpret_compound_glyph; all that must be done here
	   is to give the graphics state to that function.  */
	{
	  error = sfnt_interpret_compound_glyph (glyph, interpreter,
						 state,
						 Some (sfntfont_get_glyph),
						 Some (sfntfont_free_glyph),
						 hmtx, hhea, maxp,
						 &raw mut temp,
						 (&raw mut dcontext) as *mut c_void,
						 &raw mut value);   // C 2311-2317
	}

      if error.is_null ()                                  // C 2319
	{
	  /* Now record the advance with that measured from the
	     phantom points within the instructed glyph outline, and
	     subsequently replace it once metrics are scaled.  */

	  outline = sfnt_build_instructed_outline (value,
						   &raw mut advance);   // C 2325-2326
	  xfree (value as *mut c_void);                      // C 2327

	  if !outline.is_null ()                             // C 2329
	    {
	      /* Save the new advance width.  This advance width is
		 rounded again, as the instruction code executed might
		 have moved both phantom points such that they no
		 longer measure a fractional distance.  */
	      temp.advance = SFNT_ROUND_FIXED (advance);     // C 2335

	      /* Finally, adjust the left side bearing of the glyph
		 metrics by the origin point of the outline, should a
		 transformation have been applied by either
		 instruction code or glyph variation.  The left side
		 bearing is the distance from the origin point to the
		 left most point on the X axis.  */
	      temp.lbearing
		= SFNT_FLOOR_FIXED ((*outline).xmin - (*outline).origin);   // C 2343-2344
	    }
	}
    }

  if outline.is_null ()                                    // C 2349
    {
      /* Build the outline.  This will apply GX offsets within *GLYPH
	 to TEMP.  */
      outline = sfnt_build_glyph_outline (glyph, scale,
					  &raw mut temp,
					  Some (sfntfont_get_glyph),
					  Some (sfntfont_free_glyph),
					  Some (sfntfont_get_metrics),
					  (&raw mut dcontext) as *mut c_void);   // C 2353-2358

      /* At this point, the glyph metrics are unscaled.  Scale them
	 up.  If INTERPRETER is set, use the scale placed within.  */
      sfnt_scale_metrics (&raw mut temp, scale);             // C 2362
    }
  }                                                        /* C 2365: `fail:`.  */

  xfree (glyph as *mut c_void);                            // C 2367

  if outline.is_null ()
    { return std::ptr::null_mut (); }                      // C 2370

  start = xmalloc (std::mem::size_of::<sfnt_outline_cache> ()) as *mut sfnt_outline_cache;   // C 2372
  (*start).glyph = glyph_code;                             // C 2373
  (*start).outline = outline;                              // C 2374
  (*start).metrics = temp;                                 // C 2375

  /* One reference goes to the cache.  The second reference goes to
     the caller.  */
  (*outline).refcount = 2;                                 // C 2379

  /* Link start onto the cache.  */
  (*start).next = (*cache).next;                           // C 2382
  (*start).last = cache;                                   // C 2383
  (*(*start).next).last = start;                           // C 2384
  (*(*start).last).next = start;                           // C 2385

  /* Update the cache size.  */
  *cache_size += 1;                                        // C 2388

  /* Figure out if the least recently used element has to be
     evicted.  */
  if *cache_size > SFNT_OUTLINE_CACHE_SIZE                 // C 2392
    {
      start = (*cache).last;                               // C 2394
      eassert! (start != cache);                           // C 2395

      /* Free the least recently used entry in the cache.  */
      (*(*start).last).next = (*start).next;                // C 2398
      (*(*start).next).last = (*start).last;                // C 2399
      sfntfont_dereference_outline ((*start).outline);      // C 2400
      xfree (start as *mut c_void);                         // C 2401

      *cache_size -= 1;                                     // C 2403
    }

  /* Return the cached outline and metrics.  */

  if !metrics.is_null ()
    { *metrics = temp; }                                    // C 2409

  return outline;                                           // C 2411
}

/* Free the outline cache referred to by CACHE.  Dereference each
   outline contained therein.  */

/// C 2418-2437: `sfntfont_free_outline_cache`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_free_outline_cache (cache: *mut sfnt_outline_cache)
{                                                          // C 2418-2437
  let mut next: *mut sfnt_outline_cache;
  let mut last: *mut sfnt_outline_cache;

  /* Handle partly initialized fonts.  */
  if (*cache).next.is_null ()                              // C 2423
    { return; }

  next = (*cache).next;                                    // C 2426
  while next != cache
    {
      last = next;                                         // C 2428
      next = (*next).next;                                 // C 2429

      sfntfont_dereference_outline ((*last).outline);       // C 2431
      xfree (last as *mut c_void);                          // C 2432
    }

  (*cache).next = cache;                                   // C 2435
  (*cache).last = cache;                                   // C 2436
}

/* Dereference the raster RASTER.  Free it once refcount reaches
   0.  */

/// C 2443-2451: `sfntfont_dereference_raster`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_dereference_raster (raster: *mut sfnt_raster)
{                                                          // C 2443-2451
  eassert! ((*raster).refcount > 0);                       // C 2445

  (*raster).refcount -= 1;                                 // C 2447's `--raster->refcount`

  if (*raster).refcount != 0
    { return; }                                            // C 2448

  xfree (raster as *mut c_void);                           // C 2450
}

/* Get the raster corresponding to the specified GLYPH_CODE in CACHE.
   Use the outline named OUTLINE.  Keep *CACHE_SIZE updated with the
   number of elements in the cache.  */

/// C 2458-2534: `sfntfont_get_glyph_raster`.  C 2518's eviction test names
/// `SFNT_OUTLINE_CACHE_SIZE` even though this is the raster cache - the port keeps C's text (the
/// constant is the one the file has).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_get_glyph_raster (glyph_code: sfnt_glyph_index,
				     cache: *mut sfnt_raster_cache,
				     outline: *mut sfnt_glyph_outline,
				     cache_size: *mut c_int)
  -> *mut sfnt_raster
{                                                          // C 2458-2534
  let mut start: *mut sfnt_raster_cache;
  let mut raster: *mut sfnt_raster;

  /* See if the raster is already cached.  */
  start = (*cache).next;                                   // C 2467

  while start != cache                                     // C 2469
    {
      if (*start).glyph == glyph_code                      // C 2471
	{
	  /* Move start to the start of the ring.  Them, increase
	     start->raster->refcount and return it.  */

	  (*(*start).last).next = (*start).next;             // C 2476
	  (*(*start).next).last = (*start).last;             // C 2477

	  (*start).next = (*cache).next;                     // C 2479
	  (*start).last = cache;                             // C 2480
	  (*(*start).next).last = start;                     // C 2481
	  (*(*start).last).next = start;                     // C 2482
	  (*((*start).raster)).refcount += 1;                // C 2483

	  return (*start).raster;                            // C 2485
	}
      start = (*start).next;                                 // C 2469's step
    }

  /* Not already cached.  Raster the outline.  */

  if !sfnt_raster_glyphs_exactly                            // C 2491
    { raster = sfnt_raster_glyph_outline (outline); }      // C 2492
  else
    { raster = sfnt_raster_glyph_outline_exact (outline); }   // C 2494

  if raster.is_null ()
    { return std::ptr::null_mut (); }                      // C 2497

  start = xmalloc (std::mem::size_of::<sfnt_raster_cache> ()) as *mut sfnt_raster_cache;   // C 2499
  (*start).glyph = glyph_code;                             // C 2500
  (*start).raster = raster;                                // C 2501

  /* One reference goes to the cache.  The second reference goes to
     the caller.  */
  (*raster).refcount = 2;                                  // C 2505

  /* Link start onto the cache.  */
  (*start).next = (*cache).next;                           // C 2508
  (*start).last = cache;                                   // C 2509
  (*(*start).next).last = start;                           // C 2510
  (*(*start).last).next = start;                           // C 2511

  /* Update the cache size.  */
  *cache_size += 1;                                        // C 2514

  /* Figure out if the least recently used element has to be
     evicted.  */
  if *cache_size > SFNT_OUTLINE_CACHE_SIZE                 // C 2518
    {
      start = (*cache).last;                               // C 2520
      eassert! (start != cache);                           // C 2521

      /* Free the least recently used entry in the cache.  */
      (*(*start).last).next = (*start).next;                // C 2524
      (*(*start).next).last = (*start).last;                // C 2525
      sfntfont_dereference_raster ((*start).raster);        // C 2526
      xfree (start as *mut c_void);                         // C 2527

      *cache_size -= 1;                                     // C 2529
    }

  /* Return the cached raster.  */
  return raster;                                           // C 2533
}

/* Free the raster cache referred to by CACHE.  Dereference each
   raster contained therein.  */

/// C 2540-2559: `sfntfont_free_raster_cache`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_free_raster_cache (cache: *mut sfnt_raster_cache)
{                                                          // C 2540-2559
  let mut next: *mut sfnt_raster_cache;
  let mut last: *mut sfnt_raster_cache;

  /* Handle partly initialized fonts.  */
  if (*cache).next.is_null ()                              // C 2545
    { return; }

  next = (*cache).next;                                    // C 2548
  while next != cache
    {
      last = next;                                         // C 2550
      next = (*next).next;                                 // C 2551

      sfntfont_dereference_raster ((*last).raster);         // C 2553
      xfree (last as *mut c_void);                          // C 2554
    }

  (*cache).next = cache;                                   // C 2557
  (*cache).last = cache;                                   // C 2558
}
