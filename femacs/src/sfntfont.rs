// femacs: port of src/sfntfont.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct field, macro, file-scope object, local), the same control flow and the same error model.
// The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static`
// helpers stay plain `unsafe fn` except the two whose address C takes, which become
// `unsafe extern "C" fn` items (PORTING.md 1.4; the androidselect.rs `close_asset_fd` shape).
//
// **This file is P-86's unit `src/sfntfont.c` (4,222 lines), ported in slices: C 1-1240,
// C 1241-1426, C 1427-1604, C 1605-1871, C 1873-2050, C 2051-2172, C 2173-2559, C 2560-2742,
// C 2743-2915, C 2916-3113, C 3114-3472, C 3473-3660, C 3661-3799, C 3800-3983 and
// C 4112-4222.**  **All of C 1-4222 is transcribed** (the `HAVE_HARFBUZZ` arm of C 3984-4110 is the last
// slice, carried under that feature).  The ledger `docs/ports/sfntfont.md`
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
   no mirror in this tree, so this is P-88 ①'s declaration segment at the `#include` line: the
   names C 3989-4108 uses, each with the header line it comes from.  The headers are the ones
   `src/hbfont.rs`'s note measures on this machine (`/opt/homebrew/include/harfbuzz`, 14.4.0);
   an opaque HarfBuzz record is a pointer with an untyped interior (P-55, that file's 320-323
   `hb_face_t` reading), and `hb_tag_t`/`hb_memory_mode_t` are `uint32_t`/`enum` - the two
   spellings `hb-common.h` 165 and `hb-blob.h` 69-74 give.  */
/// C: hb-common.h 165: `typedef uint32_t hb_tag_t;`
#[cfg (feature = "harfbuzz")]
pub type hb_tag_t = u32;
/// C: hb-common.h 378: `typedef void (*hb_destroy_func_t) (void *user_data);`
#[cfg (feature = "harfbuzz")]
pub type hb_destroy_func_t = Option<unsafe extern "C" fn (user_data: *mut c_void)>;
/// C: hb-blob.h 69-74: `typedef enum { HB_MEMORY_MODE_DUPLICATE, HB_MEMORY_MODE_READONLY,
/// HB_MEMORY_MODE_WRITABLE, HB_MEMORY_MODE_READONLY_MAY_MAKE_WRITABLE } hb_memory_mode_t;`
#[cfg (feature = "harfbuzz")]
pub type hb_memory_mode_t = c_uint;
/// C: hb-blob.h 71: `HB_MEMORY_MODE_READONLY` - the enum's second member.
#[cfg (feature = "harfbuzz")]
pub const HB_MEMORY_MODE_READONLY: hb_memory_mode_t = 1;
/// C: hb-blob.h 72: `HB_MEMORY_MODE_WRITABLE` - the enum's third member.
#[cfg (feature = "harfbuzz")]
pub const HB_MEMORY_MODE_WRITABLE: hb_memory_mode_t = 2;
/// C: hb-blob.h 84: `typedef struct hb_blob_t hb_blob_t;` - opaque here (P-55).
#[cfg (feature = "harfbuzz")]
pub struct hb_blob_t
{
  pub _opaque: [u8; 0],
}
/// C: hb-face.h 56: `typedef struct hb_face_t hb_face_t;` - opaque here (P-55).
#[cfg (feature = "harfbuzz")]
pub struct hb_face_t
{
  pub _opaque: [u8; 0],
}
/// C: hb-common.h 520: `typedef struct hb_font_t hb_font_t;` - opaque here (P-55).
#[cfg (feature = "harfbuzz")]
pub struct hb_font_t
{
  pub _opaque: [u8; 0],
}
/// C: hb-face.h 102: `typedef hb_blob_t * (*hb_reference_table_func_t) (hb_face_t *face,
/// hb_tag_t tag, void *user_data);`
#[cfg (feature = "harfbuzz")]
pub type hb_reference_table_func_t
  = Option<unsafe extern "C" fn (face: *mut hb_face_t, tag: hb_tag_t, user_data: *mut c_void)
				 -> *mut hb_blob_t>;
/// C 4028-4030: `hb_blob_create (table->data, table->length, HB_MEMORY_MODE_READONLY, table,
/// sfntfont_unmap_blob)`; the prototype is hb-blob.h 87-92.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_blob_create (data: *const c_char, length: c_uint, mode: hb_memory_mode_t,
			 user_data: *mut c_void, destroy: hb_destroy_func_t) -> *mut hb_blob_t;
}
/// C 4075-4076: `hb_face_create_for_tables (sfntfont_get_font_table, font, NULL)`; the prototype
/// is hb-face.h 106-108.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_face_create_for_tables (reference_table_func: hb_reference_table_func_t,
				    user_data: *mut c_void, destroy: hb_destroy_func_t)
				    -> *mut hb_face_t;
}
/// C 4078: `hb_face_get_glyph_count (face)`; the prototype is hb-face.h 163.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_face_get_glyph_count (face: *mut hb_face_t) -> c_uint;
}
/// C 4103: `hb_face_destroy (face)`; the prototype is hb-face.h 117.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_face_destroy (face: *mut hb_face_t);
}
/// C 4080: `hb_font_create (face)`; the prototype is hb-font.h 1117.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_font_create (face: *mut hb_face_t) -> *mut hb_font_t;
}
/// C 4087: `hb_font_set_scale (info->hb_font, factor * 64, factor * 64)`; the prototype is
/// hb-font.h 1190-1192.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_font_set_scale (font: *mut hb_font_t, x_scale: c_int, y_scale: c_int);
}
/// C 4088: `hb_font_set_ppem (info->hb_font, factor, factor)`; the prototype is hb-font.h
/// 1203-1205.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_font_set_ppem (font: *mut hb_font_t, x_ppem: c_uint, y_ppem: c_uint);
}
/// C 4093-4094: `hb_font_set_var_named_instance (info->hb_font, info->instance)`; the prototype
/// is hb-font.h 1281-1282.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_font_set_var_named_instance (font: *mut hb_font_t, instance_index: c_uint);
}
/// C 4099: `hb_ot_font_set_funcs (info->hb_font)`; the prototype is hb-ot-font.h 40.
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hb_ot_font_set_funcs (font: *mut hb_font_t);
}

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
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

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

  SAFE_FREE! (sa_count, sa_mark);
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

/* ----------------------------------------------------------------------------------------------
   C 2560-2742: the "Opening fonts" block's first four items, in C's order - C 2561's form feed and
   C 2563's `/* Opening fonts.  */` banner, `struct sfnt_font_info` (C 2565-2645), the `open_fonts`
   list (C 2647-2653), `sfntfont_lookup_glyph` (C 2658-2679), C 2681-2682's forward declaration of
   `sfntfont_measure_pcm` and `sfntfont_probe_widths` (C 2692-2742).  **The next live definition is
   C 2753's `sfntfont_setup_interpreter`**, and the ledger's `## C line ranges` names both
   boundaries.

   `HAVE_MMAP` is 1 in the frozen configuration and its arms have no `#else`, so the two members it
   guards (C 2572's `next`, C 2632's `glyf_table_mapped`) are **ungated** - slice 1's `sfnt_font_tables`
   reading; the three `HAVE_HARFBUZZ` members (C 2637/2640/2643) are gated `harfbuzz` and C 2637's
   `hb_font_t *` is the untyped pointer the mirror uses for that header's records (P-72 (3), the
   `ftcrfont.rs` 591 alias).  C 2681-2682's forward declaration is C's own device for a caller that
   precedes its callee: `sfntfont_measure_pcm` is *defined* at C 3504, so its use site here carries
   a declaration (the `ftfont.rs` 2066 `hbotf_open` shape, P-91 ④) that the C 3504 slice replaces
   with the definition - the ledger's `## Owed` names it.
   ---------------------------------------------------------------------------------------------- */
/// C 2565-2645: `struct sfnt_font_info` - the record the "Opening fonts" block is built on.  Its
/// layout is **measured** (Evidence ⑲), not remembered (P-88 ②).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct sfnt_font_info                           // C 2565-2645
{
  /* Parent font structure.  */
  pub font: font,                                   // C 2568

  /* The next font in this chain.  */
  pub next: *mut sfnt_font_info,                    // C 2572 (C 2570's `#ifdef HAVE_MMAP`, live)

  /* The font description used to create this font.  Used to
     dereference tables associated with this font.  */
  pub desc: *mut sfnt_font_desc,                    // C 2577

  /* Various tables required to use the font.  */
  pub cmap: *mut sfnt_cmap_table,                   // C 2580
  pub hhea: *mut sfnt_hhea_table,                   // C 2581
  pub maxp: *mut sfnt_maxp_table,                   // C 2582
  pub head: *mut sfnt_head_table,                   // C 2583
  pub hmtx: *mut sfnt_hmtx_table,                   // C 2584
  pub glyf: *mut sfnt_glyf_table,                   // C 2585
  pub loca_short: *mut sfnt_loca_table_short,       // C 2586
  pub loca_long: *mut sfnt_loca_table_long,         // C 2587
  pub prep: *mut sfnt_prep_table,                   // C 2588
  pub fpgm: *mut sfnt_fpgm_table,                   // C 2589
  pub cvt: *mut sfnt_cvt_table,                     // C 2590

  /* The selected character map.  */
  pub cmap_data: *mut sfnt_cmap_encoding_subtable_data,   // C 2593

  /* Data identifying that character map.  */
  pub cmap_subtable: sfnt_cmap_encoding_subtable,   // C 2596

  /* The UVS context.  */
  pub uvs: *mut sfnt_uvs_context,                   // C 2599

  /* Outline cache.  */
  pub outline_cache: sfnt_outline_cache,            // C 2602

  /* Number of elements in the outline cache.  */
  pub outline_cache_size: c_int,                    // C 2605

  /* Raster cache.  */
  pub raster_cache: sfnt_raster_cache,              // C 2608

  /* Number of elements in the raster cache.  */
  pub raster_cache_size: c_int,                     // C 2611

  /* Interpreter for grid fitting (if enabled).  */
  pub interpreter: *mut sfnt_interpreter,           // C 2614

  /* Graphics state after the execution of the font and control value
     programs.  */
  pub state: sfnt_graphics_state,                   // C 2618

  /* Factor used to convert from em space to pixel space.  */
  pub scale: sfnt_fixed,                            // C 2621

  /* The blend (configuration of this multiple master font).  */
  pub blend: sfnt_blend,                            // C 2624

  /* The index of the named instance used to initialize BLEND.
     -1 if BLEND is not initialized.  */
  pub instance: c_int,                              // C 2628

  /* Whether or not the glyph table has been mmapped.  */
  pub glyf_table_mapped: bool,                      // C 2632 (C 2630's `#ifdef HAVE_MMAP`, live)

  #[cfg (feature = "harfbuzz")]
  /* HarfBuzz font object.  */
  pub hb_font: *mut c_void,                         // C 2637 (the `hb_font_t *` of C 2635's arm)

  #[cfg (feature = "harfbuzz")]
  /* File descriptor associated with this font.  */
  pub fd: c_int,                                    // C 2640

  #[cfg (feature = "harfbuzz")]
  /* The table directory of the font file.  */
  pub directory: *mut sfnt_offset_subtable,         // C 2643
}


/* clang, frozen configuration (`HAVE_MMAP` 1, `HAVE_HARFBUZZ` 0): `sizeof 576`, `_Alignof 8`,
   `font` 0, `next` 184, `desc` 192, `cmap` 200, `hhea` 208, `maxp` 216, `head` 224, `hmtx` 232,
   `glyf` 240, `loca_short` 248, `loca_long` 256, `prep` 264, `fpgm` 272, `cvt` 280, `cmap_data`
   288, `cmap_subtable` 296, `uvs` 304, `outline_cache` 312, `outline_cache_size` 352,
   `raster_cache` 360, `raster_cache_size` 392, `interpreter` 400, `state` 408, `scale` 512,
   `blend` 520, `instance` 568, `glyf_table_mapped` 572.  Two other conditions move this record's
   layout, as they do for `sfnt_font_tables` above: the three `HAVE_HARFBUZZ` members (C 2635-2644,
   24 bytes) and the eight `HAVE_WINDOW_SYSTEM` members of the embedded `struct font`
   (`font.h` 325-403, four bytes each, so 32 in all - `font.rs`'s `FONT_WS_SHIFT`).  The frozen
   Android configuration has `HAVE_WINDOW_SYSTEM` 1 and `HAVE_HARFBUZZ` undefined
   (`docs/baseline/config/android-arm64.json`), so *its* reading is the one asserted below the next
   comment; the host-shaped reading here is the one this file carried first.
    */
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	    not (feature = "harfbuzz"), not (feature = "have_window_system")))]
const _: () = {
  core::assert! (std::mem::size_of::<sfnt_font_info> () == 576);
  core::assert! (std::mem::align_of::<sfnt_font_info> () == 8);
  core::assert! (std::mem::offset_of! (sfnt_font_info, font) == 0);
  core::assert! (std::mem::offset_of! (sfnt_font_info, next) == 184);
  core::assert! (std::mem::offset_of! (sfnt_font_info, desc) == 192);
  core::assert! (std::mem::offset_of! (sfnt_font_info, cmap) == 200);
  core::assert! (std::mem::offset_of! (sfnt_font_info, hhea) == 208);
  core::assert! (std::mem::offset_of! (sfnt_font_info, maxp) == 216);
  core::assert! (std::mem::offset_of! (sfnt_font_info, head) == 224);
  core::assert! (std::mem::offset_of! (sfnt_font_info, hmtx) == 232);
  core::assert! (std::mem::offset_of! (sfnt_font_info, glyf) == 240);
  core::assert! (std::mem::offset_of! (sfnt_font_info, loca_short) == 248);
  core::assert! (std::mem::offset_of! (sfnt_font_info, loca_long) == 256);
  core::assert! (std::mem::offset_of! (sfnt_font_info, prep) == 264);
  core::assert! (std::mem::offset_of! (sfnt_font_info, fpgm) == 272);
  core::assert! (std::mem::offset_of! (sfnt_font_info, cvt) == 280);
  core::assert! (std::mem::offset_of! (sfnt_font_info, cmap_data) == 288);
  core::assert! (std::mem::offset_of! (sfnt_font_info, cmap_subtable) == 296);
  core::assert! (std::mem::offset_of! (sfnt_font_info, uvs) == 304);
  core::assert! (std::mem::offset_of! (sfnt_font_info, outline_cache) == 312);
  core::assert! (std::mem::offset_of! (sfnt_font_info, outline_cache_size) == 352);
  core::assert! (std::mem::offset_of! (sfnt_font_info, raster_cache) == 360);
  core::assert! (std::mem::offset_of! (sfnt_font_info, raster_cache_size) == 392);
  core::assert! (std::mem::offset_of! (sfnt_font_info, interpreter) == 400);
  core::assert! (std::mem::offset_of! (sfnt_font_info, state) == 408);
  core::assert! (std::mem::offset_of! (sfnt_font_info, scale) == 512);
  core::assert! (std::mem::offset_of! (sfnt_font_info, blend) == 520);
  core::assert! (std::mem::offset_of! (sfnt_font_info, instance) == 568);
  core::assert! (std::mem::offset_of! (sfnt_font_info, glyf_table_mapped) == 572);
};

/* The configuration Android builds (`HAVE_WINDOW_SYSTEM` 1, `HAVE_HARFBUZZ` undefined): every
   offset past the embedded `struct font` moves by the 32 bytes `font.h` 325-403 adds, so the record
   is 608 and `glyf_table_mapped` 604 - the number the port itself printed in `build-tu
   src/sfntfont.c` with a deliberately failing array length (status 4.832).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	    not (feature = "harfbuzz"), feature = "have_window_system"))]
const _: () = {
  core::assert! (std::mem::size_of::<sfnt_font_info> () == 608);
  core::assert! (std::mem::align_of::<sfnt_font_info> () == 8);
  core::assert! (std::mem::offset_of! (sfnt_font_info, font) == 0);
  core::assert! (std::mem::offset_of! (sfnt_font_info, next) == 216);
  core::assert! (std::mem::offset_of! (sfnt_font_info, desc) == 224);
  core::assert! (std::mem::offset_of! (sfnt_font_info, cmap) == 232);
  core::assert! (std::mem::offset_of! (sfnt_font_info, hhea) == 240);
  core::assert! (std::mem::offset_of! (sfnt_font_info, maxp) == 248);
  core::assert! (std::mem::offset_of! (sfnt_font_info, head) == 256);
  core::assert! (std::mem::offset_of! (sfnt_font_info, hmtx) == 264);
  core::assert! (std::mem::offset_of! (sfnt_font_info, glyf) == 272);
  core::assert! (std::mem::offset_of! (sfnt_font_info, loca_short) == 280);
  core::assert! (std::mem::offset_of! (sfnt_font_info, loca_long) == 288);
  core::assert! (std::mem::offset_of! (sfnt_font_info, prep) == 296);
  core::assert! (std::mem::offset_of! (sfnt_font_info, fpgm) == 304);
  core::assert! (std::mem::offset_of! (sfnt_font_info, cvt) == 312);
  core::assert! (std::mem::offset_of! (sfnt_font_info, cmap_data) == 320);
  core::assert! (std::mem::offset_of! (sfnt_font_info, cmap_subtable) == 328);
  core::assert! (std::mem::offset_of! (sfnt_font_info, uvs) == 336);
  core::assert! (std::mem::offset_of! (sfnt_font_info, outline_cache) == 344);
  core::assert! (std::mem::offset_of! (sfnt_font_info, outline_cache_size) == 384);
  core::assert! (std::mem::offset_of! (sfnt_font_info, raster_cache) == 392);
  core::assert! (std::mem::offset_of! (sfnt_font_info, raster_cache_size) == 424);
  core::assert! (std::mem::offset_of! (sfnt_font_info, interpreter) == 432);
  core::assert! (std::mem::offset_of! (sfnt_font_info, state) == 440);
  core::assert! (std::mem::offset_of! (sfnt_font_info, scale) == 544);
  core::assert! (std::mem::offset_of! (sfnt_font_info, blend) == 552);
  core::assert! (std::mem::offset_of! (sfnt_font_info, instance) == 600);
  core::assert! (std::mem::offset_of! (sfnt_font_info, glyf_table_mapped) == 604);
};

/* List of all open fonts.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut open_fonts: *mut sfnt_font_info = std::ptr::null_mut ();      // C 2651

/* C 2681-2682's forward declaration of `sfntfont_measure_pcm` has no Rust text: the C 3504
   definition (slice 12) replaces the use-site declaration slice 8 carried, which is Owed 9 closed.  */

/* Look up the glyph corresponding to the character C in FONT.  Return
   0 upon failure, and the glyph otherwise.  */

/// C 2658-2679: `sfntfont_lookup_glyph` - C's `static sfnt_glyph` (the scalar typedef, i.e.
/// `sfnt_glyph_index` in the port, Deviation 18 (a)).  It reads `font.encoding_charset`, one of the
/// eight members C guards with `HAVE_WINDOW_SYSTEM` (`font.h` 325-403, and the mirror gates them
/// with `have_window_system`), so the item carries that gate too - every branch that builds
/// `sfntfont.o` is a window-system build (`configure.ac` 2645's Android branch, the ledger's Gate
/// mapping), which is why C itself can read it unconditionally.
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	   feature = "have_window_system"))]
unsafe fn sfntfont_lookup_glyph (font_info: *mut sfnt_font_info, c: c_int) -> sfnt_glyph_index
{                                                          // C 2658-2679
  let mut charset: *mut charset;
  let mut character: sfnt_char;
  let mut glyph: sfnt_glyph_index;

  charset = CHARSET_FROM_ID! ((*font_info).font.encoding_charset);   // C 2665

  if charset.is_null ()
    { return 0; }                                          // C 2668

  character = ENCODE_CHAR! (charset, c);                   // C 2670

  if character == CHARSET_INVALID_CODE! (charset)
    { return 0; }                                          // C 2673

  /* Do the actual lookup with the encoded character.  */
  glyph = sfnt_lookup_glyph (character, (*font_info).cmap_data);   // C 2676

  return glyph;                                            // C 2678
}

/* Probe and set FONT_INFO->font.average_width,
   FONT_INFO->font.space_width, and FONT_INFO->font.min_width
   according to the tables contained therein.

   As this function generates outlines for all glyphs, outlines for
   all ASCII characters will be entered into the outline cache as
   well.  */

/// C 2692-2742: `sfntfont_probe_widths` - its callee `sfntfont_lookup_glyph` carries the
/// `have_window_system` gate above, so this item does too.
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	   feature = "have_window_system"))]
unsafe fn sfntfont_probe_widths (font_info: *mut sfnt_font_info)
{                                                          // C 2692-2742
  let mut i: c_int;
  let mut num_characters: c_int;
  let mut total_width: c_int;
  let mut glyph: sfnt_glyph_index;
  /* C 2697's `pcm` is filled by `sfntfont_measure_pcm` at C 2719 before it is read; zero-initialised
     because its address is taken (PORTING.md 1.5, Deviation 10).  */
  let mut pcm: font_metrics = std::mem::zeroed ();

  num_characters = 0;                                      // C 2699
  total_width = 0;                                         // C 2700

  /* First set some reasonable default values.  */
  (*font_info).font.average_width = (*font_info).font.pixel_size;   // C 2703
  (*font_info).font.space_width = (*font_info).font.pixel_size;     // C 2704
  (*font_info).font.min_width = 1;                         // C 2705

  /* Next, loop through the common ASCII characters.  Tally up their
     advance widths and set space_width if necessary.  */
  i = 32;                                                  // C 2709
  while i < 127
    {
      glyph = sfntfont_lookup_glyph (font_info, i);        // C 2711

      if glyph == 0
	{ i += 1; continue; }                              // C 2714

      /* Now look up the metrics of this glyph.  Data from the metrics
	 table doesn't fit the bill, since variations and instruction
	 code is not applied to it.  */
      if sfntfont_measure_pcm (font_info, glyph, &raw mut pcm) != 0
	{ i += 1; continue; }                              // C 2720

      /* Increase the number of characters.  */
      num_characters += 1;                                 // C 2723

      /* Add the advance to total_width.  */
      total_width += pcm.width as c_int;                   // C 2726

      /* Update min_width if it hasn't been set yet or is wider.  */
      if (*font_info).font.min_width == 1
	 || (*font_info).font.min_width > pcm.width as c_int   // C 2729-2730
	{ (*font_info).font.min_width = pcm.width as c_int; }   // C 2731

      /* If i is the space character, set the space width.  Make sure
	 to round this up.  */
      if i == 32
	{ (*font_info).font.space_width = pcm.width as c_int; }   // C 2736
      i += 1;
    }

  /* Now, if characters were found, set average_width.  */
  if num_characters != 0                                   // C 2740
    { (*font_info).font.average_width = total_width / num_characters; }   // C 2741
}

/* ----------------------------------------------------------------------------------------------
   C 2743-2915: `sfntfont_setup_interpreter` (C 2753-2860, under C 2744-2751's comment) and
   `sfnt_close_tables` (C 2865-2915, under C 2862-2863's comment).  **The next live definition is
   C 2920's `sfnt_open_tables`** (the comment is C 2917-2918), and the ledger's `## C line ranges`
   names both boundaries.

   C's two labels `bail1:` (C 2856) and `bail:` (C 2858) are the `'body: loop` + level variable
   shape slice 1's `sfnt_enum_font` uses (PORTING.md 2.2): `goto bail` skips the interpreter free,
   `goto bail1` runs it.  `message_with_string` is `lisp.h` 4397's prototype and no mirror carries
   it, so it is declared at its use site below; `HAVE_MMAP` is 1 and its arms have no `#else`, so
   C 2868's `rc` local and the C 2877-2887 `if`/`else` are written ungated - while C 2902-2914's
   `HAVE_HARFBUZZ` block is `#[cfg (feature = "harfbuzz")]` (its two members are that gate's).
   ---------------------------------------------------------------------------------------------- */
/* `message_with_string` is `lisp.h` 4397's `extern void message_with_string (const char *,
   Lisp_Object, bool);`, declared here at its use site (the `src/fileio.rs` 4466 shape, P-91 4);
   `sfntfont_setup_interpreter` is the user.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: lisp.h 4397: `extern void message_with_string (const char *, Lisp_Object, bool);`
  pub fn message_with_string (m: *const c_char, string: Lisp_Object, log: bool);
}

/// C 2753-2860: `sfntfont_setup_interpreter`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_setup_interpreter (info: *mut sfnt_font_info,
				      desc: *mut sfnt_font_desc,
				      point_size: c_int)
{                                                          // C 2753-2860
  let mut cvt: *mut sfnt_cvt_table;
  let mut fpgm: *mut sfnt_fpgm_table;
  let mut prep: *mut sfnt_prep_table;
  /* C 2761's `interpreter` is assigned at C 2801 before any path that reads it; zero-initialised
     because the `bail1` path's `xfree` reads it (PORTING.md 1.5, Deviation 10).  */
  let mut interpreter: *mut sfnt_interpreter = std::ptr::null_mut ();
  /* C 2762's `error` is assigned by *both* interpret calls (C 2817 in the `fpgm` arm and C 2835 in
     the `prep` arm) before C 2819/2838 read it, so the binding is `mut` (the arm flip read
     `E0384 cannot assign twice to immutable variable` before this).  */
  let mut error: *const c_char;
  /* C 2763's `state` is uninitialised in C and is read at C 2836 when `prep` is set but `fpgm` is
     not - a latent C uninitialised read that the port gives a defined value (Deviation 10's
     class; the ledger names it).  */
  let mut state: sfnt_graphics_state = std::mem::zeroed ();
  let regexp: Lisp_Object;

  /* C 2856-2859's `bail1:`/`bail:` pair: 0 is `goto bail`, 1 is `goto bail1` (the
     `sfnt_enum_font` shape, PORTING.md 2.2).  */
  let mut bail: c_int = 0;

  'body: loop
    {
      /* If Vsfnt_uninstructable_family_regexp matches this font, then
	 return.  */

      regexp = Vsfnt_uninstructable_family_regexp;         // C 2769

      if STRINGP (regexp)
	 && (fast_string_match_ignore_case (regexp,
					    (*desc).family)   // C 2772-2773
	     >= 0)
	{ return; }

      /* Load the cvt, fpgm and prep already read.  */

      cvt = (*info).cvt;                                   // C 2779
      fpgm = (*info).fpgm;                                 // C 2780
      prep = (*info).prep;                                 // C 2781

      /* If both fpgm and prep are NULL, this font likely has no
	 instructions, so don't bother setting up the interpreter.  */

      if fpgm.is_null () && prep.is_null ()
	{ break 'body; }                                   // C 2787: `goto bail;`

      /* If the interpreter does not use the operand stack at all, it is
	 useless.  In addition, some broken fonts specify some unnecessary
	 instructions in prep and set head->max_stack_elements to 0.

	 Don't create the interpreter in that case.  */

      if (*(*info).maxp).max_stack_elements == 0
	{ break 'body; }                                   // C 2796: `goto bail;`

      /* Now, create the interpreter using the limits in info->maxp and
	 info->head.  CVT can be NULL.  */

      interpreter = sfnt_make_interpreter ((*info).maxp, cvt, (*info).head,
					   (*(*desc).tables).fvar,
					   (*info).font.pixel_size,
					   point_size);        // C 2801-2804

      /* Bail if the interpreter couldn't be created.  */
      if interpreter.is_null ()
	{ break 'body; }                                   // C 2808: `goto bail;`

      if !fpgm.is_null ()                                  // C 2810
	{
	  /* Otherwise, evaluate the font and cvt programs.

	     FIXME: make sure infinite loops inside these programs
	     cannot lock up Emacs.  */

	  error = sfnt_interpret_font_program (interpreter, fpgm);   // C 2817

	  if !error.is_null ()                             // C 2819
	    {
	      /* If an error occurs, log it to the *Messages* buffer.  */
	      message_with_string (c"While interpreting font program: %s".as_ptr (),
				   build_string (error), true);   // C 2822-2823
	      bail = 1; break 'body;                       // C 2824: `goto bail1;`
	    }

	  /* Save the graphics state.  */
	  state = (*interpreter).state;                    // C 2828
	}

      if !prep.is_null ()                                  // C 2831
	{
	  /* This will overwrite state if the instruction control is set
	     appropriately.  */
	  error = sfnt_interpret_control_value_program (interpreter, prep,
							&raw mut state);   // C 2835-2836

	  if !error.is_null ()                             // C 2838
	    {
	      /* If an error occurs, log it to the *Messages* buffer.  */
	      message_with_string (c"While interpreting preprogram: %s".as_ptr (),
				   build_string (error), true);   // C 2841-2842
	      bail = 1; break 'body;                       // C 2843: `goto bail1;`
	    }
	}

      /* The interpreter has been properly set up.  */
      (*info).fpgm = fpgm;                                 // C 2848
      (*info).prep = prep;                                 // C 2849
      (*info).cvt = cvt;                                   // C 2850
      (*info).state = state;                               // C 2851
      (*info).interpreter = interpreter;                   // C 2852

      return;                                              // C 2854
    }

  /* C 2856-2859: `bail1:` frees the interpreter, `bail:` returns.  */
  if bail >= 1
    { xfree (interpreter as *mut c_void); }                // C 2857
}

/* Free each of the tables opened by `sfnt_open_tables', and possibly
   file descriptors as well.  Then, free TABLES itself.  */

/// C 2865-2915: `sfnt_close_tables`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_close_tables (tables: *mut sfnt_font_tables)
{                                                          // C 2865-2915
  /* C 2868-2870's `#ifdef HAVE_MMAP` local: live, since `HAVE_MMAP` is 1 and the arm has no
     `#else` (the file's gate mapping).  */
  let mut rc: c_int;

  xfree ((*tables).cmap as *mut c_void);                   // C 2872
  xfree ((*tables).hhea as *mut c_void);                   // C 2873
  xfree ((*tables).maxp as *mut c_void);                   // C 2874
  xfree ((*tables).head as *mut c_void);                   // C 2875
  xfree ((*tables).hmtx as *mut c_void);                   // C 2876
  if (*tables).glyf_table_mapped                           // C 2878
    {
      rc = sfnt_unmap_glyf_table ((*tables).glyf);         // C 2880

      if rc != 0
	{ emacs_abort (); }                                // C 2883
    }
  else
    { xfree ((*tables).glyf as *mut c_void); }             // C 2887
  xfree ((*tables).loca_short as *mut c_void);             // C 2888
  xfree ((*tables).loca_long as *mut c_void);              // C 2889
  xfree ((*tables).prep as *mut c_void);                   // C 2890
  xfree ((*tables).fpgm as *mut c_void);                   // C 2891
  xfree ((*tables).cvt as *mut c_void);                    // C 2892
  xfree ((*tables).fvar as *mut c_void);                   // C 2893
  xfree ((*tables).avar as *mut c_void);                   // C 2894
  xfree ((*tables).gvar as *mut c_void);                   // C 2895
  xfree ((*tables).cvar as *mut c_void);                   // C 2896
  xfree ((*tables).cmap_data as *mut c_void);              // C 2897

  if !(*tables).uvs.is_null ()                             // C 2899
    { sfnt_free_uvs_context ((*tables).uvs); }             // C 2900

  #[cfg (feature = "harfbuzz")]
  {
    /* Close the font file.  */

    if (*tables).fd != -1                                  // C 2905
      {
	emacs_close ((*tables).fd);                        // C 2907
	(*tables).fd = -1;                                 // C 2908
      }

    /* Free its table directory.  */
    xfree ((*tables).directory as *mut c_void);             // C 2912
    (*tables).directory = std::ptr::null_mut ();            // C 2913
  }
}

/* Open font tables associated with the specified font description
   DESC.  Return the font tables, or NULL upon failure.  */

/* ----------------------------------------------------------------------------------------------
   C 2916-3113: `sfnt_open_tables` (C 2920-3113, under C 2917-2918's comment).  **The next live
   definition is C 3119's `sfnt_reference_font_tables`** (the comment is C 3115-3116), and the
   ledger's `## C line ranges` names both boundaries.

   C's five labels `bail5:`/`bail4:`/`bail3:`/`bail2:`/`bail1:` plus `bail:` are the depth-carrying
   chain slice 1's `sfnt_enum_font_1` uses (PORTING.md 2.2): the nine `goto bailN`s at C 2939/2945
   (bail), C 2951 (bail1), C 2960 (bail2), C 2992 (bail3), C 3020/3031/3039 (bail4) and C 3047
   (bail5) become `break 'body` with `bail` set, and the six segments run in C's order.
   `HAVE_MMAP` is 1 and its arms have no `#else`, so C 2998-3013's cross-`#endif` `if`/`else` and
   C 3084-3094's are written out with their guards; C 3064-3073's `HAVE_HARFBUZZ` `#ifdef`/`#else`
   is two `#[cfg]`-selected blocks (its two members carry that gate).
   ---------------------------------------------------------------------------------------------- */
/// C 2920-3113: `sfnt_open_tables`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_open_tables (desc: *mut sfnt_font_desc) -> *mut sfnt_font_tables
{                                                          // C 2920-3113
  let mut tables: *mut sfnt_font_tables;
  /* C 2924's `subtable` is assigned at C 2948 and read by the `bail2` segment, which only the
     levels at or above 2 reach; C 2925's `fd` is the same for the `bail1` segment's
     `emacs_close`.  C's own flow guarantees both, Rust's analysis cannot see it through `bail`, so
     they are zero-initialised (PORTING.md 1.5, Deviation 10).  */
  let mut subtable: *mut sfnt_offset_subtable = std::ptr::null_mut ();
  let mut fd: c_int = 0;
  let mut i: c_int;
  /* C 2927's `rc` (C 2926-2928's `#ifdef HAVE_MMAP` local: live, since `HAVE_MMAP` is 1).  */
  let mut rc: c_int;
  /* C 2929-2930's two out-parameters of `sfnt_read_cmap_table`; zero-initialised for the same
     reason (their addresses are taken).  */
  let mut subtables: *mut sfnt_cmap_encoding_subtable = std::ptr::null_mut ();
  let mut data: *mut *mut sfnt_cmap_encoding_subtable_data = std::ptr::null_mut ();
  let mut format14: *mut sfnt_cmap_format_14;

  /* C 3077-3112's `bail5:` … `bail:` chain: the port carries the jump's depth in `bail` (0 is
     `goto bail`) and runs the six segments in C's order.  */
  let mut bail: c_int = 0;

  'body: loop
    {
      tables = xzalloc (std::mem::size_of::<sfnt_font_tables> ()) as *mut sfnt_font_tables;   // C 2933

      /* Open the font.  */
      fd = emacs_open ((*desc).path, O_RDONLY, 0);         // C 2936

      if fd == -1
	{ break 'body; }                                   // C 2939: `goto bail;`

      /* Seek to the offset specified to the table directory.  */

      if (*desc).offset != 0
	 && lseek (fd, (*desc).offset, SEEK_SET) != (*desc).offset   // C 2943-2944
	{ break 'body; }                                   // C 2945: `goto bail;`

      /* Read the offset subtable.  */
      subtable = sfnt_read_table_directory (fd);           // C 2948

      if subtable.is_null () || subtable == usize::MAX as *mut sfnt_offset_subtable   // C 2950
	{ bail = 1; break 'body; }                         // C 2951: `goto bail1;`

      /* Read required tables.  This font backend is supposed to be used
	 mostly on devices with flash memory, so the order in which they
	 are read is insignificant.  */

      (*tables).cmap = sfnt_read_cmap_table (fd, subtable, &raw mut subtables,
					     &raw mut data);   // C 2957-2958
      if (*tables).cmap.is_null ()
	{ bail = 2; break 'body; }                         // C 2960: `goto bail2;`

      format14 = std::ptr::null_mut ();                    // C 2962
      (*tables).cmap_data
	= sfntfont_select_cmap ((*tables).cmap,
				subtables, data,
				&raw mut (*tables).cmap_subtable,
				&raw mut format14);        // C 2963-2967

      if !format14.is_null ()                              // C 2969
	{
	  /* Build a UVS context from this format 14 mapping table.  A UVS
	     context contains each variation selector supported by the
	     font, and a list of ``non-default'' mappings between base
	     characters and variation glyph IDs.  */

	  (*tables).uvs = sfnt_create_uvs_context (format14, fd);   // C 2976
	  xfree (format14 as *mut c_void);                     // C 2977
	}

      i = 0;
      while i < (*(*tables).cmap).num_subtables as c_int    // C 2980
	{
	  if *(data.offset (i as isize)) != (*tables).cmap_data
	     /* format14 has already been freed.  */
	     && *(data.offset (i as isize)) != format14 as *mut sfnt_cmap_encoding_subtable_data   // C 2984
	    { xfree (*(data.offset (i as isize)) as *mut c_void); }   // C 2985
	  i += 1;
	}

      xfree (subtables as *mut c_void);                    // C 2988
      xfree (data as *mut c_void);                         // C 2989

      if (*tables).cmap_data.is_null ()
	{ bail = 3; break 'body; }                         // C 2992: `goto bail3;`

      /* Read the hhea, maxp, glyf, and head tables.  */
      (*tables).hhea = sfnt_read_hhea_table (fd, subtable);   // C 2995
      (*tables).maxp = sfnt_read_maxp_table (fd, subtable);   // C 2996

      /* First try to map the glyf table.  If that fails, then read the
	 glyf table.  */

      (*tables).glyf = sfnt_map_glyf_table (fd, subtable);   // C 3003

      /* Next, if this fails, read the glyf table.  */

      if (*tables).glyf.is_null ()                         // C 3007
	{ (*tables).glyf = sfnt_read_glyf_table (fd, subtable); }   // C 3009
      else
	{ (*tables).glyf_table_mapped = true; }            // C 3012

      (*tables).head = sfnt_read_head_table (fd, subtable);   // C 3015

      /* If any of those tables couldn't be read, bail.  */
      if (*tables).hhea.is_null () || (*tables).maxp.is_null ()
	 || (*tables).glyf.is_null ()
	 || (*tables).head.is_null ()                      // C 3018-3019
	{ bail = 4; break 'body; }                         // C 3020: `goto bail4;`

      /* Now figure out which kind of loca table must be read based on
	 head->index_to_loc_format.  */

      if (*(*tables).head).index_to_loc_format != 0    // C 3025
	{
	  (*tables).loca_long
	    = sfnt_read_loca_table_long (fd, subtable);    // C 3027-3028

	  if (*tables).loca_long.is_null ()
	    { bail = 4; break 'body; }                     // C 3031: `goto bail4;`
	}
      else
	{
	  (*tables).loca_short
	    = sfnt_read_loca_table_short (fd, subtable);   // C 3035-3036

	  if (*tables).loca_short.is_null ()
	    { bail = 4; break 'body; }                     // C 3039: `goto bail4;`
	}

      /* Read the horizontal metrics table.  */
      (*tables).hmtx = sfnt_read_hmtx_table (fd, subtable,
					     (*tables).hhea,
					     (*tables).maxp);   // C 3043-3045
      if (*tables).hmtx.is_null ()
	{ bail = 5; break 'body; }                         // C 3047: `goto bail5;`

      /* Read instruction related font tables.  These might not be
	 present, which is OK, since instructing fonts is optional.  */
      (*tables).prep = sfnt_read_prep_table (fd, subtable);   // C 3051
      (*tables).fpgm = sfnt_read_fpgm_table (fd, subtable);   // C 3052
      (*tables).cvt = sfnt_read_cvt_table (fd, subtable);     // C 3053

      /* Read distortion related tables.  These might not be present.  */
      (*tables).fvar = sfnt_read_fvar_table (fd, subtable);   // C 3056
      (*tables).avar = sfnt_read_avar_table (fd, subtable);   // C 3057
      (*tables).gvar = sfnt_read_gvar_table (fd, subtable);   // C 3058

      if !(*tables).cvt.is_null () && !(*tables).fvar.is_null ()   // C 3060
	{ (*tables).cvar = sfnt_read_cvar_table (fd, subtable, (*tables).fvar,
						 (*tables).cvt); }   // C 3061-3062

      #[cfg (feature = "harfbuzz")]
      {
	/* Now copy over the subtable if necessary, as it is needed to read
	   extra font tables required by HarfBuzz.  */
	(*tables).directory = subtable;                    // C 3067
	(*tables).fd = fd;                                 // C 3068
      }
      #[cfg (not (feature = "harfbuzz"))]
      {
	/* Otherwise, close the fd and free the table directory.  */
	xfree (subtable as *mut c_void);                   // C 3071
	emacs_close (fd);                                  // C 3072
      }

      return tables;                                       // C 3075
    }

  /* C 3077: `bail5:`.  */
  if bail >= 5
    {
      xfree ((*tables).loca_long as *mut c_void);           // C 3078
      xfree ((*tables).loca_short as *mut c_void);          // C 3079
    }

  /* C 3080: `bail4:`.  */
  if bail >= 4
    {
      xfree ((*tables).hhea as *mut c_void);                // C 3081
      xfree ((*tables).maxp as *mut c_void);                // C 3082

      if (*tables).glyf_table_mapped
	{
	  rc = sfnt_unmap_glyf_table ((*tables).glyf);      // C 3087

	  if rc != 0
	    { emacs_abort (); }                            // C 3090
	}
      else
	{ xfree ((*tables).glyf as *mut c_void); }         // C 3094

      xfree ((*tables).head as *mut c_void);                // C 3096

      /* This comes under bail4 due to a peculiarity of how the four
	 tables above are validated.  */
      xfree ((*tables).cmap_data as *mut c_void);           // C 3100
    }

  /* C 3101: `bail3:`.  */
  if bail >= 3
    {
      if !(*tables).uvs.is_null ()                         // C 3102
	{ sfnt_free_uvs_context ((*tables).uvs); }         // C 3103

      xfree ((*tables).cmap as *mut c_void);                // C 3105
    }

  /* C 3106: `bail2:`.  */
  if bail >= 2
    { xfree (subtable as *mut c_void); }                    // C 3107

  /* C 3108: `bail1:`.  */
  if bail >= 1
    { emacs_close (fd); }                                  // C 3109

  /* C 3110: `bail:`.  */
  xfree (tables as *mut c_void);                           // C 3111
  return std::ptr::null_mut ();                            // C 3112
}

/* Open or reference font tables corresponding to the specified font
   DESC.  Return NULL upon failure.  */

/// C 3119-3134: `sfnt_reference_font_tables`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_reference_font_tables (desc: *mut sfnt_font_desc) -> *mut sfnt_font_tables
{                                                          // C 3119-3134
  if (*desc).refcount != 0                                 // C 3121
    {
      (*desc).refcount += 1;                               // C 3123
      return (*desc).tables;                               // C 3124
    }

  (*desc).tables = sfnt_open_tables (desc);                // C 3127

  if (*desc).tables.is_null ()
    { return std::ptr::null_mut (); }                      // C 3130

  (*desc).refcount += 1;                                   // C 3132
  return (*desc).tables;                                   // C 3133
}

/* Dereference font tables corresponding to the specified font
   DESC.  */

/// C 3140-3151: `sfnt_dereference_font_tables`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfnt_dereference_font_tables (desc: *mut sfnt_font_desc)
{                                                          // C 3140-3151
  if (*desc).refcount == 0                                 // C 3142
    { emacs_abort (); }                                    // C 3143

  (*desc).refcount -= 1;                                   // C 3145's `--desc->refcount`

  if (*desc).refcount != 0
    { return; }                                            // C 3146

  sfnt_close_tables ((*desc).tables);                      // C 3148
  (*desc).tables = std::ptr::null_mut ();                  // C 3149
  return;                                                  // C 3150
}

/* Open the font corresponding to the font-entity FONT_ENTITY.  Return
   nil upon failure, else the opened font-object.  */

/* `font_make_object` is `font.h` 846's `extern Lisp_Object font_make_object (int, Lisp_Object,
   int);`, declared here at its use site (the `src/haikufont.rs` 123 shape, P-91 4);
   `sfntfont_open` is the user.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {

}

/// C 3156-3472: `sfntfont_open` - C's non-`static` driver entry point.  It reads the
/// `HAVE_WINDOW_SYSTEM` members of `struct font` (`encoding_charset`, `baseline_offset`,
/// `relative_compose`, `default_ascent`, `vertical_centering`, and `font.h`'s
/// `FRAME_FONT`/`FRAME_DISPLAY_INFO`), so the item carries that gate as well (Deviation 23 (a)'s
/// reading).
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	   feature = "have_window_system"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfntfont_open (f: *mut c_void, font_entity: Lisp_Object,
					mut pixel_size: c_int) -> Lisp_Object
{                                                          // C 3156-3472
  let mut font_info: *mut sfnt_font_info;
  let mut font: *mut font;
  let mut desc: *mut sfnt_font_desc;
  let mut font_object: Lisp_Object;
  let mut charset: *mut charset;
  let mut point_size: c_int;
  let mut instance: c_int;
  let mut i: c_int;
  let mut dpyinfo: *mut Display_Info;
  let mut tables: *mut sfnt_font_tables;
  let mut tem: Lisp_Object;

  if XFIXNUM (AREF (font_entity, FONT_SIZE_INDEX as isize)) != 0   // C 3170
    { pixel_size = XFIXNUM (AREF (font_entity, FONT_SIZE_INDEX as isize)) as c_int; }   // C 3171
  else if pixel_size == 0                                  // C 3172
    {
      /* This bit was copied from xfont.c.  The values might need
	 adjustment.  */

      if !FRAME_FONT! (f as *mut frame).is_null ()         // C 3177
	{ pixel_size = (*FRAME_FONT! (f as *mut frame)).pixel_size; }   // C 3178
      else
	{ pixel_size = 12; }                               // C 3180
    }

  /* Now find the font description corresponding to FONT_ENTITY.  */

  tem = AREF (font_entity, FONT_EXTRA_INDEX as isize);     // C 3185
  if NILP (tem)
    { return Qnil (); }                                    // C 3187

  desc = xmint_pointer (XCDR (XCAR (tem))) as *mut sfnt_font_desc;   // C 3189

  /* Finally, see if a specific instance is associated with
     FONT_ENTITY.  */

  instance = -1;                                           // C 3194
  if !NILP (XCDR (tem))
    { instance = XFIXNUM (XCDR (XCAR (XCDR (tem)))) as c_int; }   // C 3196

  /* Build the font object.  */
  font_object = font_make_object (VECSIZE (std::mem::size_of::<sfnt_font_info> ()) as c_int,
				  font_entity, pixel_size);   // C 3199-3200
  font_info = XFONT_OBJECT (font_object) as *mut sfnt_font_info;   // C 3201

  block_input ();                                          // C 3203

  /* C 3466/3469's `bail6:`/`bail:` pair: 0 is `goto bail` (C 3247), 6 is `goto bail6` (C 3279);
     the port carries the jump's depth (the `sfnt_enum_font_1` shape).  */
  let mut bail: c_int = 0;

  'body: loop
    {
      /* Initialize all the font driver specific data.  */

      (*font_info).cmap = std::ptr::null_mut ();           // C 3207
      (*font_info).hhea = std::ptr::null_mut ();           // C 3208
      (*font_info).maxp = std::ptr::null_mut ();           // C 3209
      (*font_info).head = std::ptr::null_mut ();           // C 3210
      (*font_info).glyf = std::ptr::null_mut ();           // C 3211
      (*font_info).hmtx = std::ptr::null_mut ();           // C 3212
      (*font_info).loca_short = std::ptr::null_mut ();     // C 3213
      (*font_info).loca_long = std::ptr::null_mut ();      // C 3214
      (*font_info).cmap_data = std::ptr::null_mut ();      // C 3215
      (*font_info).prep = std::ptr::null_mut ();           // C 3216
      (*font_info).fpgm = std::ptr::null_mut ();           // C 3217
      (*font_info).cvt = std::ptr::null_mut ();            // C 3218
      (*font_info).uvs = std::ptr::null_mut ();            // C 3219

      (*font_info).outline_cache.next = &raw mut (*font_info).outline_cache;   // C 3221
      (*font_info).outline_cache.last = &raw mut (*font_info).outline_cache;   // C 3222
      (*font_info).outline_cache_size = 0;                 // C 3223
      (*font_info).raster_cache.next = &raw mut (*font_info).raster_cache;   // C 3224
      (*font_info).raster_cache.last = &raw mut (*font_info).raster_cache;   // C 3225
      (*font_info).raster_cache_size = 0;                  // C 3226
      (*font_info).interpreter = std::ptr::null_mut ();    // C 3227
      (*font_info).scale = 0;                              // C 3228
      (*font_info).instance = -1;                          // C 3229
      (*font_info).blend.coords = std::ptr::null_mut ();   // C 3230
      /* C 3231-3233's `#ifdef HAVE_MMAP`: live (the file's gate mapping).  */
      (*font_info).glyf_table_mapped = false;              // C 3232
      #[cfg (feature = "harfbuzz")]
      {
	(*font_info).hb_font = std::ptr::null_mut ();      // C 3235
	(*font_info).fd = -1;                              // C 3236
	(*font_info).directory = std::ptr::null_mut ();    // C 3237
      }

      /* Read required tables.  This font backend is supposed to be used
	 mostly on devices with flash memory, so the order in which they
	 are read is insignificant.  */

      tables = sfnt_reference_font_tables (desc);          // C 3244

      if tables.is_null ()
	{ break 'body; }                                   // C 3247: `goto bail;`

      /* Copy fields from the table structure to the font for fast
	 access.  */
      (*font_info).cmap = (*tables).cmap;                  // C 3251
      (*font_info).hhea = (*tables).hhea;                  // C 3252
      (*font_info).maxp = (*tables).maxp;                  // C 3253
      (*font_info).head = (*tables).head;                  // C 3254
      (*font_info).hmtx = (*tables).hmtx;                  // C 3255
      (*font_info).glyf = (*tables).glyf;                  // C 3256
      (*font_info).loca_short = (*tables).loca_short;      // C 3257
      (*font_info).loca_long = (*tables).loca_long;        // C 3258
      (*font_info).prep = (*tables).prep;                  // C 3259
      (*font_info).fpgm = (*tables).fpgm;                  // C 3260
      (*font_info).cvt = (*tables).cvt;                    // C 3261
      (*font_info).cmap_data = (*tables).cmap_data;        // C 3262
      (*font_info).cmap_subtable = (*tables).cmap_subtable;   // C 3263
      (*font_info).uvs = (*tables).uvs;                    // C 3264

      /* Calculate the font's scaling factor.  */
      (*font_info).scale = sfnt_get_scale ((*font_info).head, pixel_size);   // C 3267

      /* Fill in font data.  */
      font = &raw mut (*font_info).font;                   // C 3270
      (*font).pixel_size = pixel_size;                     // C 3271
      (*font).driver = sfnt_font_driver as *const c_void;   // C 3272
      /* C 3273's chained assignment is two statements (PORTING.md 0.1-13).  */
      (*font).encoding_charset = -1;
      (*font).repertory_charset = -1;

      /* Figure out which character set to use.  */
      charset = sfntfont_charset_for_cmap ((*font_info).cmap_subtable);   // C 3276

      if charset.is_null ()
	{ bail = 6; break 'body; }                         // C 3279: `goto bail6;`

      /* Set the character set IDs.  */
      (*font).encoding_charset = (*charset).id;            // C 3282
      (*font).repertory_charset = (*charset).id;           // C 3283

      /* Figure out the font ascent and descent.  */
      (*font).ascent
	= ceil ((*(*font_info).hhea).ascent as c_double
		* pixel_size as c_double
		* (1.0 / (*(*font_info).head).units_per_em as c_double)) as c_int;   // C 3286-3289
      (*font).descent
	= ceil (-((*(*font_info).hhea).descent as c_double)
		* pixel_size as c_double
		* (1.0 / (*(*font_info).head).units_per_em as c_double)) as c_int;   // C 3290-3293
      (*font).height = (*font).ascent + (*font).descent;   // C 3294

      /* Set font->max_width to the maximum advance width.  */
      (*font).max_width
	= ((*(*font_info).hhea).advance_width_max as c_double
	   * pixel_size as c_double * 1.0
	   / (*(*font_info).head).units_per_em as c_double) as c_int;   // C 3297-3298

      /* Set generic attributes such as type and style.  */
      ASET (font_object, FONT_TYPE_INDEX as isize, sfnt_vendor_name);   // C 3301
      ASET (font_object, FONT_FOUNDRY_INDEX as isize, (*desc).designer);   // C 3302
      ASET (font_object, FONT_FAMILY_INDEX as isize, Fintern ((*desc).family, Qnil ()));   // C 3303
      ASET (font_object, FONT_ADSTYLE_INDEX as isize, (*desc).adstyle);   // C 3304
      ASET (font_object, FONT_REGISTRY_INDEX as isize,
	    sfntfont_registry_for_desc (desc));            // C 3305-3306

      /* Size of 0 means the font is scalable.  */
      ASET (font_object, FONT_SIZE_INDEX as isize, make_fixnum (0));   // C 3309
      ASET (font_object, FONT_AVGWIDTH_INDEX as isize, make_fixnum (0));   // C 3310
      ASET (font_object, FONT_SPACING_INDEX as isize,
	    make_fixnum ((*desc).spacing as EMACS_INT));   // C 3311

      /* Set the font style.  */

      FONT_SET_STYLE! (font_object, FONT_WIDTH_INDEX,
		       make_fixnum ((*desc).width as EMACS_INT));   // C 3315-3316
      FONT_SET_STYLE! (font_object, FONT_WEIGHT_INDEX,
		       make_fixnum ((*desc).weight as EMACS_INT));   // C 3317-3318
      FONT_SET_STYLE! (font_object, FONT_SLANT_INDEX,
		       make_fixnum ((*desc).slant as EMACS_INT));   // C 3319-3320

      /* Clear various offsets.  */
      (*font).baseline_offset = 0;                         // C 3323
      (*font).relative_compose = 0;                        // C 3324
      (*font).default_ascent = 0;                          // C 3325
      (*font).vertical_centering = false;                  // C 3326

      if (*desc).bits & SFNT_FONT_DESC_UNDERLINE_POSITION_SET == 0   // C 3328
	{
	  (*font).underline_position = -1;                 // C 3330
	  (*font).underline_thickness = 0;                 // C 3331
	}
      else
	{
	  (*font).underline_position
	    = (sfnt_coerce_fixed ((-((*desc).underline_position as sfnt_fixed))
				  * (*font_info).scale)
	       + 0.5) as c_int;                            // C 3335-3337
	  (*font).underline_thickness
	    = (sfnt_coerce_fixed ((*desc).underline_thickness as sfnt_fixed
				  * (*font_info).scale)
	       + 0.5) as c_int;                            // C 3338-3340
	}

      /* Now try to set up grid fitting for this font.  */
      dpyinfo = FRAME_DISPLAY_INFO! (f as *mut frame);     // C 3344
      /* C 3345-3347: `PT_PER_INCH` is a `double` in C too, so the macro's first argument is
	 converted (the `font.rs` 5379 call does the same).  */
      point_size = PIXEL_TO_POINT! (pixel_size as c_double, ((*dpyinfo).resx
							     * (*dpyinfo).resy
							     / 2.0)) as c_int;   // C 3345-3347
      sfntfont_setup_interpreter (font_info, desc, point_size);   // C 3348

      /* If an instance was specified and the font is distortable, set up
	 the blend.  */

      'cancel_blend: {
      if instance != -1
	 && !(*(*desc).tables).fvar.is_null () && !(*(*desc).tables).gvar.is_null ()   // C 3353-3354
	 /* Make sure the instance is within range.  */
	 && (instance as u32) < (*(*(*desc).tables).fvar).instance_count as u32   // C 3356
	{
	  tem = AREF ((*desc).instances, instance as isize);   // C 3358

	  if !NILP (tem)
	    {
	      sfnt_init_blend (&raw mut (*font_info).blend, (*(*desc).tables).fvar,
			       (*(*desc).tables).gvar, (*(*desc).tables).avar,
			       (*(*desc).tables).cvar);   // C 3362-3364

	      /* Copy over the coordinates.  */
	      i = 0;
	      while i < (*(*(*desc).tables).fvar).axis_count as c_int   // C 3367
		{
		  *(*font_info).blend.coords.add (i as usize)
		    = *((*(*(*(*desc).tables).fvar).instance.add (instance as usize))
			.coords.add (i as usize));         // C 3368-3369
		  i += 1;
		}

	      sfnt_normalize_blend (&raw mut (*font_info).blend);   // C 3371

	      /* Test whether or not the instance is actually redundant,
		 as all of its axis are at their default values.  If so,
		 free the instance.  */

	      i = 0;
	      while i < (*(*(*desc).tables).fvar).axis_count as c_int   // C 3377
		{
		  if *(*font_info).blend.norm_coords.add (i as usize) != 0
		    { break; }                                 // C 3380
		  i += 1;
		}

	      if i == (*(*(*desc).tables).fvar).axis_count as c_int   // C 3383
		{
		  sfnt_free_blend (&raw mut (*font_info).blend);   // C 3385
		  break 'cancel_blend;                         // C 3386: `goto cancel_blend;`
		}

	      /* If an interpreter was specified, distort it now.  */

	      if !(*font_info).interpreter.is_null ()          // C 3391
		{ sfnt_vary_interpreter ((*font_info).interpreter,
					 &raw mut (*font_info).blend); }   // C 3392-3393

	      (*font_info).instance = instance;                // C 3395

	      /* Replace the style information with that of the
		 instance.  */

	      FONT_SET_STYLE! (font_object, FONT_WIDTH_INDEX,
			       AREF (tem, 2));                 // C 3400-3401
	      FONT_SET_STYLE! (font_object, FONT_WEIGHT_INDEX,
			       AREF (tem, 3));                 // C 3402-3403
	      FONT_SET_STYLE! (font_object, FONT_SLANT_INDEX,
			       AREF (tem, 4));                 // C 3404-3405
	      ASET (font_object, FONT_ADSTYLE_INDEX as isize, AREF (tem, 1));   // C 3406
	    }
	}
      }                                                    /* C 3410: `cancel_blend:`.  */

      /* Find out the minimum, maximum and average widths.  */
      sfntfont_probe_widths (font_info);                   // C 3413

      /* Calculate the xfld name.  */
      (*font).props[FONT_NAME_INDEX as usize]
	= Ffont_xlfd_name (font_object, Qnil (), Qt ());   // C 3416

      #[cfg (feature = "harfbuzz")]
      {
	/* HarfBuzz will potentially read font tables after the font has
	   been opened by Emacs.  Keep the font open, and record its offset
	   subtable.  */
	(*font_info).fd = (*tables).fd;                    // C 3422
	(*font_info).directory = (*tables).directory;      // C 3423
      }

      /* Set font->desc so that font tables can be dereferenced if
	 anything goes wrong.  */
      (*font_info).desc = desc;                            // C 3428

      /* C 3430-3434's `#ifdef HAVE_MMAP`: live.  */
      /* Link the font onto the font table.  */
      (*font_info).next = open_fonts;                      // C 3432
      open_fonts = font_info;                              // C 3433

      /* Now ascertain if vertical centering is desired by matching the
	 font XLFD against vertical-centering-font-regexp.  */

      if !NILP ((*font).props[FONT_NAME_INDEX as usize])   // C 3439
	{
	  (*font).vertical_centering
	    = STRINGP (Vvertical_centering_font_regexp)
	      && (fast_string_match_ignore_case
		  (Vvertical_centering_font_regexp,
		   (*font).props[FONT_NAME_INDEX as usize]) >= 0);   // C 3440-3444
	}

      /* Set the name of the font file.  */
      (*font).props[FONT_FILE_INDEX as usize]
	= DECODE_FILE (build_unibyte_string ((*desc).path));   // C 3447-3448

      /* Encapsulate some information on the font useful while debugging
	 (along with being informative in general) in the font name.  */

      AUTO_STRING! (format, c"%s %s interpreted: %s upem: %s charset: %s instance: %s".as_ptr ());
      (*font).props[FONT_FULLNAME_INDEX as usize]
	= CALLN! (Fformat, format, (*desc).family, (*desc).style,
		  if !(*font_info).interpreter.is_null () { Qt () } else { Qnil () },
		  make_fixnum ((*(*font_info).head).units_per_em as EMACS_INT),
		  CHARSET_NAME! (charset),
		  make_fixnum (instance as EMACS_INT));    // C 3455-3460

      /* All done.  */
      unblock_input ();                                    // C 3463
      return font_object;                                  // C 3464
    }

  /* C 3466: `bail6:`.  */
  if bail >= 6
    {
      sfnt_dereference_font_tables (desc);                 // C 3467
      (*font_info).desc = std::ptr::null_mut ();           // C 3468
    }

  /* C 3469: `bail:`.  */
  unblock_input ();                                        // C 3470
  return Qnil ();                                          // C 3471
}

/* ----------------------------------------------------------------------------------------------
   C 3473-3660: the metrics family's first four items, in C's order - C 3476's
   `/* Metrics computation and other similar font backend functions.  */` banner,
   `sfntfont_encode_char` (C 3483-3494), `sfntfont_measure_pcm` (C 3504-3539, whose C 3504
   definition **replaces the use-site declaration** slice 8 carried - ledger Owed 9),
   `sfntfont_text_extents` (C 3544-3582) and `sfntfont_close` (C 3587-3660).  **The next live
   definition is C 3676's `sfntfont_draw`** (the `/* Glyph display.  */` banner is C 3664 and
   `sfnt_put_glyphs` is C 3667), and the ledger's `## C line ranges` names both boundaries.

   `sfntfont_encode_char` calls `sfntfont_lookup_glyph`, whose item carries `have_window_system`
   (it reads `font.encoding_charset`), so this item carries that gate too (Deviation 23 (a));
   `hb_font_destroy` is `hb-font.h` 1128-1129's prototype and no mirror carries it, so it is
   declared at its use site under the `harfbuzz` gate (the `ftcrfont.rs` 592 shape).
   ---------------------------------------------------------------------------------------------- */
/// C 3483-3494: `sfntfont_encode_char` - C's non-`static` driver entry point.
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	   feature = "have_window_system"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfntfont_encode_char (font: *mut font, c: c_int) -> c_uint
{                                                          // C 3483-3494
  let mut glyph: sfnt_glyph_index;

  /* Now look up the glyph.  */
  glyph = sfntfont_lookup_glyph (font as *mut sfnt_font_info, c);   // C 3488

  if glyph == 0
    { return FONT_INVALID_CODE; }                          // C 3491

  return glyph;                                            // C 3493
}

/// C 3504-3539: `sfntfont_measure_pcm` - C's `static`, called by name only (C 2719 and C 3559).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn sfntfont_measure_pcm (font: *mut sfnt_font_info, glyph: sfnt_glyph_index,
				pcm: *mut font_metrics) -> c_int
{                                                          // C 3504-3539
  /* C 3507's `metrics` is the out-parameter of `sfntfont_get_glyph_outline` (C 3522) and is read
     from C 3529 on; zero-initialised because its address is taken (PORTING.md 1.5, Deviation
     10).  */
  let mut metrics: sfnt_glyph_metrics = std::mem::zeroed ();
  let mut outline: *mut sfnt_glyph_outline;

  /* Now get the glyph outline, which is required to obtain the rsb,
     ascent and descent.  */
  outline = sfntfont_get_glyph_outline (glyph, &raw mut (*font).outline_cache,
					(*font).scale,
					&raw mut (*font).outline_cache_size,
					&raw mut (*font).blend,
					(*font).instance,
					(*font).glyf, (*font).head,
					(*font).hmtx, (*font).hhea,
					(*font).maxp,
					(*font).loca_short,
					(*font).loca_long,
					(*font).interpreter, &raw mut metrics,
					&raw mut (*font).state);   // C 3512-3523

  if outline.is_null ()
    { return 1; }                                          // C 3526

  /* The left side bearing has already been floored.  */
  (*pcm).lbearing = (metrics.lbearing / 65536) as c_short;   // C 3529
  (*pcm).rbearing = (SFNT_CEIL_FIXED ((*outline).xmax) / 65536) as c_short;   // C 3530

  /* The advance is already rounded; ceil the ascent and descent.  */
  (*pcm).width = (metrics.advance / 65536) as c_short;     // C 3533
  (*pcm).ascent = (SFNT_CEIL_FIXED ((*outline).ymax) / 65536) as c_short;   // C 3534
  (*pcm).descent = (SFNT_CEIL_FIXED (-((*outline).ymin)) / 65536) as c_short;   // C 3535

  sfntfont_dereference_outline (outline);                  // C 3537
  return 0;                                                // C 3538
}

/* Return the total text extents of NGLYPHS glyphs given as CODE in
   the single font metrics array METRICS.  */

/// C 3544-3582: `sfntfont_text_extents` - C's non-`static` driver entry point.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfntfont_text_extents (font: *mut font, code: *const c_uint,
						nglyphs: c_int, metrics: *mut font_metrics)
{                                                          // C 3544-3582
  let mut i: c_int;
  let mut total_width: c_int;
  /* C 3549's `pcm` is filled by `sfntfont_measure_pcm` before it is read; zero-initialised
     because its address is taken (PORTING.md 1.5, Deviation 10).  */
  let mut pcm: font_metrics = std::mem::zeroed ();

  total_width = 0;                                         // C 3551

  /* First clear the metrics array.  */
  memset (metrics as *mut c_void, 0, std::mem::size_of::<font_metrics> ());   // C 3554

  /* Get the metrcs one by one, then sum them up.  */
  i = 0;                                                   // C 3557
  while i < nglyphs
    {
      if sfntfont_measure_pcm (font as *mut sfnt_font_info,
			       *code.offset (i as isize), &raw mut pcm) == 0   // C 3559-3560
	{
	  /* Add the per-char metric (PCM) to the metrics in
	     METRICS.  */

	  /* The parenthesised comparisons are Rust's parse rule, not a change of C's arithmetic:
	     a cast followed by `<` would read as generic arguments (the `ftfont.rs` Finding 7
	     class), so both sides are bracketed and the `int`-typed sums are C's own.  */
	  if (total_width + pcm.lbearing as c_int) < ((*metrics).lbearing as c_int)   // C 3565
	    { (*metrics).lbearing = (total_width + pcm.lbearing as c_int) as c_short; }   // C 3566

	  if (total_width + pcm.rbearing as c_int) > ((*metrics).rbearing as c_int)   // C 3568
	    { (*metrics).rbearing = (total_width + pcm.rbearing as c_int) as c_short; }   // C 3569

	  if (pcm.ascent as c_int) > ((*metrics).ascent as c_int)   // C 3571
	    { (*metrics).ascent = pcm.ascent; }                // C 3572

	  if (pcm.descent as c_int) > ((*metrics).descent as c_int)   // C 3574
	    { (*metrics).descent = pcm.descent; }              // C 3575

	  total_width += pcm.width as c_int;                   // C 3577
	}
      i += 1;
    }

  (*metrics).width = total_width as c_short;               // C 3581
}

/* Close the font FONT, discarding all tables inside it and
   dereferencing all cached outlines and rasters.  */

/* `hb_font_destroy` is `hb-font.h` 1128-1129's `void hb_font_destroy (hb_font_t *font);`, declared
   here at its use site under the `harfbuzz` gate (the `src/ftcrfont.rs` 592 shape, P-91 4); the
   `hb_font_t` pointer is the untyped one (P-72 (3), `src/ftcrfont.rs` 591).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify", feature = "harfbuzz"))]
unsafe extern "C" {
  /// C: hb-font.h 1128-1129: `void hb_font_destroy (hb_font_t *font);`
  pub fn hb_font_destroy (font: *mut c_void);
}

/// C 3587-3660: `sfntfont_close` - C's non-`static` driver entry point.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfntfont_close (font: *mut font)
{                                                          // C 3587-3660
  let mut info: *mut sfnt_font_info;
  /* C 3592's `next` (C 3591-3593's `#ifdef HAVE_MMAP` local: live, since `HAVE_MMAP` is 1).  */
  let mut next: *mut *mut sfnt_font_info;

  info = font as *mut sfnt_font_info;                      // C 3595

  /* If info->desc is still set, dereference the font tables.  */
  if !(*info).desc.is_null ()                              // C 3598
    { sfnt_dereference_font_tables ((*info).desc); }       // C 3599
  (*info).desc = std::ptr::null_mut ();                    // C 3600

  /* Free the interpreter, which is created on a per font basis.  */
  xfree ((*info).interpreter as *mut c_void);              // C 3603

  /* Clear these fields.  It seems that close can be called twice,
     once during font driver destruction, and once during GC.  */

  (*info).cmap = std::ptr::null_mut ();                    // C 3608
  (*info).hhea = std::ptr::null_mut ();                    // C 3609
  (*info).maxp = std::ptr::null_mut ();                    // C 3610
  (*info).head = std::ptr::null_mut ();                    // C 3611
  (*info).hhea = std::ptr::null_mut ();                    // C 3612 (C clears `hhea` twice)
  (*info).glyf = std::ptr::null_mut ();                    // C 3613
  (*info).loca_short = std::ptr::null_mut ();              // C 3614
  (*info).loca_long = std::ptr::null_mut ();               // C 3615
  (*info).cmap_data = std::ptr::null_mut ();               // C 3616
  (*info).prep = std::ptr::null_mut ();                    // C 3617
  (*info).fpgm = std::ptr::null_mut ();                    // C 3618
  (*info).cvt = std::ptr::null_mut ();                     // C 3619
  (*info).interpreter = std::ptr::null_mut ();             // C 3620
  (*info).uvs = std::ptr::null_mut ();                     // C 3621

  /* Deinitialize the blend.  */
  if (*info).instance != -1 && !(*info).blend.coords.is_null ()   // C 3624
    { sfnt_free_blend (&raw mut (*info).blend); }          // C 3625
  (*info).instance = -1;                                   // C 3626

  /* C 3628-3640's `#ifdef HAVE_MMAP`: live.  */

  /* Unlink INFO.  */

  next = &raw mut open_fonts;                              // C 3632
  while !(*next).is_null () && (*next) != info             // C 3633
    { next = &raw mut (**next).next; }

  if !(*next).is_null ()                                   // C 3636
    { *next = (*info).next; }                              // C 3637
  (*info).next = std::ptr::null_mut ();                    // C 3638

  #[cfg (feature = "harfbuzz")]
  {
    /* These fields will be freed or closed by
       sfnt_dereference_font_tables, but clear them here for good
       measure.  */
    (*info).directory = std::ptr::null_mut ();             // C 3646
    (*info).fd = -1;                                       // C 3647

    /* Free any hb_font created.  */

    if !(*info).hb_font.is_null ()                         // C 3651
      {
	hb_font_destroy ((*info).hb_font);                 // C 3653
	(*info).hb_font = std::ptr::null_mut ();           // C 3654
      }
  }

  sfntfont_free_outline_cache (&raw mut (*info).outline_cache);   // C 3658
  sfntfont_free_raster_cache (&raw mut (*info).raster_cache);   // C 3659
}

/* ----------------------------------------------------------------------------------------------
   C 3661-3799: the draw/callback family's first two items, in C's order - C 3664's
   `/* Glyph display.  */` banner with C 3666-3667's comment and file-scope object
   `sfnt_put_glyphs`, C 3676-3753's `sfntfont_draw`, and after C 3757's `/* Other callbacks.  */`
   banner C 3762-3799's `sfntfont_list_family`.  **The next live definition is C 3823's
   `sfntfont_get_variation_glyphs`** (behind the comment at C 3801-3821), and the ledger's
   `## C line ranges` names both boundaries.

   `sfntfont_draw` carries `have_window_system` on top of the file gate: its first parameter is
   `dispextern.h`'s `struct glyph_string` (C 3676), whose mirror is gated by that feature
   (`include/dispextern.rs` 1601; C 42's `TERM_HEADER` -> `include/androidterm.rs` 44 is how this
   unit reaches it), and C reads three of its members (C 3688, C 3709, C 3720, C 3740).  C 3667's
   object and C 3742's call go through `sfntfont_put_glyph_proc`, whose first parameter the mirror
   spells `*mut c_void` (its own note), so the call converts `s` at the one site C's type
   information is absent from.
   ---------------------------------------------------------------------------------------------- */
/// C 3667: `static sfntfont_put_glyph_proc sfnt_put_glyphs;` - the one file-scope object of this
/// slice.  C assigns it at C 4221 (`syms_of_sfntfont`, still owed) and calls it at C 3742.
/// C's own text is unconditional, but the type is `sfntfont.h`'s typedef and this unit's include
/// of that header's mirror sits under the file gate (C 34), so the object carries it too - the
/// `system_fonts` (C 169) reading, and the reason `build-tu` (which passes no `--cfg`) is the
/// plain `rustc` that makes such a mismatch an `E0425` rather than a silent empty unit.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut sfnt_put_glyphs: sfntfont_put_glyph_proc = None;   // C 3667

/// C 3676-3753: `sfntfont_draw` - C's non-`static` driver entry point.
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	   feature = "have_window_system"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfntfont_draw (s: *mut glyph_string, from: c_int, to: c_int,
					x: c_int, y: c_int, with_background: bool) -> c_int
{                                                          // C 3676-3753
  let mut length: c_int;
  let mut rasters: *mut *mut sfnt_raster;
  let mut x_coords: *mut c_int;
  let mut current_x: c_int;
  let mut i: c_int;
  let mut outline: *mut sfnt_glyph_outline;
  let mut font: *mut font;
  let mut info: *mut sfnt_font_info;
  /* C 3685's `metrics` is the out-parameter of `sfntfont_get_glyph_outline` (C 3695) that C 3743
     reads; zero-initialised because its address is taken (PORTING.md 1.5, Deviation 10).  */
  let mut metrics: sfnt_glyph_metrics = std::mem::zeroed ();

  length = to - from;                                      // C 3687
  font = (*s).font;                                        // C 3688
  info = font as *mut sfnt_font_info;                      // C 3689

  /* C 3691-3692's two `alloca` calls: C's `length * sizeof *rasters` and
     `length * sizeof *x_coords` come from the port's stack pool (PORTING.md P-05).  */
  rasters = sa_stack_alloc (length as usize * std::mem::size_of::<*mut sfnt_raster> ())
    as *mut *mut sfnt_raster;                              // C 3691
  x_coords = sa_stack_alloc (length as usize * std::mem::size_of::<c_int> ())
    as *mut c_int;                                         // C 3692
  current_x = x;                                           // C 3693

  /* Get rasters and outlines for them.  */
  i = from;                                                // C 3696
  while i < to
    {
      /* Look up the outline.  */
      outline = sfntfont_get_glyph_outline (*(*s).char2b.offset (i as isize),
					    &raw mut (*info).outline_cache,
					    (*info).scale,
					    &raw mut (*info).outline_cache_size,
					    &raw mut (*info).blend,
					    (*info).instance,
					    (*info).glyf, (*info).head,
					    (*info).hmtx, (*info).hhea,
					    (*info).maxp,
					    (*info).loca_short,
					    (*info).loca_long,
					    (*info).interpreter,
					    &raw mut metrics,
					    &raw mut (*info).state);   // C 3698-3717
      *x_coords.offset ((i - from) as isize) = 0;          // C 3718

      if outline.is_null ()                                // C 3720
	{
	  *rasters.offset ((i - from) as isize) = std::ptr::null_mut ();   // C 3722
	  i += 1;
	  continue;                                        // C 3723
	}

      /* Rasterize the outline.  */
      *rasters.offset ((i - from) as isize)
	= sfntfont_get_glyph_raster (*(*s).char2b.offset (i as isize),
				     &raw mut (*info).raster_cache,
				     outline,
				     &raw mut (*info).raster_cache_size);   // C 3727-3731
      sfntfont_dereference_outline (outline);              // C 3732

      if (*rasters.offset ((i - from) as isize)).is_null ()   // C 3734
	{ i += 1; continue; }                              // C 3735

      /* Now work out where to put the outline.  */
      *x_coords.offset ((i - from) as isize) = current_x;   // C 3738

      if glyph_string_bool_p (&(*s), GLYPH_STRING_PADDING_P)   // C 3740
	{ current_x += 1; }                                // C 3741
      else
	{ current_x += metrics.advance / 65536; }          // C 3743
      i += 1;
    }

  /* Call the window system function to put the glyphs to the
     frame.  */
  if let Some (put_glyphs) = sfnt_put_glyphs              // C 3742-3743
    {
      put_glyphs (s as *mut c_void, from, to, x, y, with_background,
		  rasters, x_coords);
    }

  /* Dereference all the rasters.  */
  i = 0;                                                   // C 3746
  /* C's bound is `i < from - to`, the *negated* length: with `from < to` the loop body never
     runs, and the rasters C just handed to `sfnt_put_glyphs` are not dereferenced here.  The
     port keeps C's expression (the 3.691 reading of C 3612's duplicated clear).  */
  while i < from - to
    {
      if !(*rasters.offset (i as isize)).is_null ()        // C 3748
	{ sfntfont_dereference_raster (*rasters.offset (i as isize)); }   // C 3749
      i += 1;
    }

  return 1;                                                // C 3752
}

/// C 3762-3799: `sfntfont_list_family` - C's non-`static` driver entry point.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfntfont_list_family (f: *mut frame) -> Lisp_Object
{                                                          // C 3762-3799
  let mut families: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut next: Lisp_Object;
  let mut desc: *mut sfnt_font_desc;

  families = Qnil ();                                      // C 3768

  desc = system_fonts;                                     // C 3770
  while !desc.is_null ()
    {
      /* Add desc->family to the list.  */
      families = Fcons ((*desc).family, families);         // C 3772
      desc = (*desc).next;
    }

  /* Sort families in preparation for removing duplicates.  */
  families = CALLN! (Fsort, families, Qstring_lessp ());   // C 3775

  /* Remove each duplicate within families.  */

  tem = families;                                          // C 3779
  /* C 3780's assignment sits inside the condition; the block is that assignment and its value.  */
  while !NILP (tem) && !NILP ({ next = XCDR (tem); next })
    {
      /* If the two strings are equal.  */
      if !NILP (Fstring_equal (XCAR (tem), XCAR (next)))   // C 3783
	/* Set tem's cdr to the cons after the next item.  */
	{ XSETCDR (tem, XCDR (next)); }                    // C 3785
      else
	/* Otherwise, start considering the next item.  */
	{ tem = next; }                                    // C 3788
    }

  /* Intern each font family.  */

  tem = families;                                          // C 3793

  FOR_EACH_TAIL! (tem, {
    XSETCAR (tem, Fintern (XCAR (tem), Qnil ()));          // C 3796
  });

  return families;                                         // C 3798
}

/* ----------------------------------------------------------------------------------------------
   C 3800-3983: the callback family's tail, in C's order - C 3801-3821's
   `/* Unicode Variation Selector (UVS) support. ... */` comment with `sfntfont_get_variation_glyphs`
   (C 3823-3955), C 3957's form feed, C 3959's `/* mmap specific stuff.  */` banner and C 3963-3965's
   comment with `sfntfont_detect_sigbus` (C 3966-3981), the live arm of C 3961's `#ifdef HAVE_MMAP`
   (`HAVE_MMAP` is 1 and the block has no `#else`, so the text is ordinary - the C 85-88 reading).
   **The next live definition is C 3989's `sfntfont_begin_hb_font`**, the first name of the
   `HAVE_HARFBUZZ` arm, and the ledger's `## C line ranges` names both boundaries.

   `sfntfont_get_variation_glyphs` carries `have_window_system` on top of the file gate (C 3853
   calls `sfntfont_lookup_glyph`, whose item does); `sfntfont_detect_sigbus` needs the file gate alone.  C's four `goto next_selector`/`goto next_selector_1`
   sites jump to the statement just before the loop's `++i`, so each body is a labelled block that
   the `goto` becomes `break` of, with C 3898/3951's step after it - the shape the tree uses for a
   forward label at a loop tail; `sfnt_is_character_default` and `sfnt_variation_glyph_for_char` come
   from `include/sfnt.rs`'s `sfnt_decls` glob (that mirror's 2493).
   ---------------------------------------------------------------------------------------------- */
/// C 3823-3955: `sfntfont_get_variation_glyphs` - C's non-`static` driver entry point.
/// It carries `have_window_system` because C 3853 calls `sfntfont_lookup_glyph`, which does
/// (Deviation 23 (a), the Deviation 27 (c) class); `sfntfont_detect_sigbus` needs the file gate
/// alone.
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	   feature = "have_window_system"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfntfont_get_variation_glyphs (font: *mut font, c: c_int,
							variations: *mut c_uint) -> c_int
{                                                          // C 3823-3955
  let mut info: *mut sfnt_font_info;
  let mut i: usize;
  let mut index: usize;
  let mut n: c_int;
  let mut record: *mut sfnt_mapped_variation_selector_record;
  let mut default_glyph: sfnt_glyph_index;

  info = font as *mut sfnt_font_info;                      // C 3831
  n = 0;                                                   // C 3832

  /* Return 0 if there is no UVS mapping table.  */

  if (*info).uvs.is_null ()                                // C 3836
    { return 0; }                                          // C 3837

  /* Clear the variations array.  */

  memset (variations as *mut c_void, 0,
	  std::mem::size_of::<c_uint> () * 256);           // C 3841

  /* Find the first 0xFExx selector.  */

  i = 0;                                                   // C 3845
  while i < (*(*info).uvs).num_records                     // C 3846
	 && (*(*(*info).uvs).records.offset (i as isize)).selector < 0xfe00
    { i += 1; }                                            // C 3848

  /* Get the glyph represented by C, used when C is present within a
     default value table.  */

  default_glyph = sfntfont_lookup_glyph (info, c);        // C 3853

  /* Fill in selectors 0 to 15.  */

  while i < (*(*info).uvs).num_records                     // C 3857
	 && (*(*(*info).uvs).records.offset (i as isize)).selector <= 0xfe0f
    {
      /* C 3879/3886's `goto next_selector` jumps at the statement before C 3898's `++i`: the
	 labelled block is that tail and the step sits after it, so every path C has is C's.  */
      'next_selector: {
	record = &raw mut *(*(*info).uvs).records.offset (i as isize);   // C 3860
	index = ((*(*(*info).uvs).records.offset (i as isize)).selector
		 - 0xfe00 + 16) as usize;                  // C 3861

	/* Handle invalid unsorted tables.  */

	if (*record).selector < 0xfe00                      // C 3865
	  { return 0; }                                    // C 3866

	/* If there are default mappings in this record, ascertain if
	   this glyph matches one of them.  */

	if !(*record).default_uvs.is_null ()                // C 3871
	   && sfnt_is_character_default ((*record).default_uvs, c as sfnt_char)   // C 3872
	  {
	    *variations.offset (index as isize) = default_glyph;   // C 3874

	    if default_glyph != 0                          // C 3876
	      { n += 1; }                                  // C 3877

	    break 'next_selector;                          // C 3879
	  }

	/* If record has no non-default mappings, continue on to the
	   next selector.  */

	if (*record).nondefault_uvs.is_null ()              // C 3885
	  { break 'next_selector; }                        // C 3886

	/* Find the glyph ID associated with C and put it in
	   VARIATIONS.  */

	*variations.offset (index as isize)
	  = sfnt_variation_glyph_for_char ((*record).nondefault_uvs,
					   c as sfnt_char);   // C 3891-3892

	if *variations.offset (index as isize) != 0        // C 3894
	  { n += 1; }                                      // C 3895
      }
      i += 1;                                              // C 3898
    }

  /* Find the first 0xE0100 selector.  */

  i = 0;                                                   // C 3903
  while i < (*(*info).uvs).num_records                     // C 3904
	 && (*(*(*info).uvs).records.offset (i as isize)).selector < 0xe0100
    { i += 1; }                                            // C 3906

  /* Fill in selectors 16 to 255.  */

  while i < (*(*info).uvs).num_records                     // C 3910
	 && (*(*(*info).uvs).records.offset (i as isize)).selector <= 0xe01ef
    {
      /* C 3932/3939's `goto next_selector_1`, the same shape as above (C 3951's `++i`).  */
      'next_selector_1: {
	record = &raw mut *(*(*info).uvs).records.offset (i as isize);   // C 3913
	index = ((*(*(*info).uvs).records.offset (i as isize)).selector
		 - 0xe0100 + 16) as usize;                  // C 3914

	/* Handle invalid unsorted tables.  */

	if (*record).selector < 0xe0100                     // C 3918
	  { return 0; }                                    // C 3919

	/* If there are default mappings in this record, ascertain if
	   this glyph matches one of them.  */

	if !(*record).default_uvs.is_null ()                // C 3924
	   && sfnt_is_character_default ((*record).default_uvs, c as sfnt_char)   // C 3925
	  {
	    *variations.offset (index as isize) = default_glyph;   // C 3927

	    if default_glyph != 0                          // C 3929
	      { n += 1; }                                  // C 3930

	    break 'next_selector_1;                        // C 3932
	  }

	/* If record has no non-default mappings, continue on to the
	   next selector.  */

	if (*record).nondefault_uvs.is_null ()              // C 3938
	  { break 'next_selector_1; }                      // C 3939

	/* Find the glyph ID associated with C and put it in
	   VARIATIONS.  */

	*variations.offset (index as isize)
	  = sfnt_variation_glyph_for_char ((*record).nondefault_uvs,
					   c as sfnt_char);   // C 3944-3945

	if *variations.offset (index as isize) != 0        // C 3947
	  { n += 1; }                                      // C 3948
      }
      i += 1;                                              // C 3951
    }

  return n;                                                // C 3954
}

/// C 3966-3981: `sfntfont_detect_sigbus` - the live `HAVE_MMAP` arm's entry point.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfntfont_detect_sigbus (addr: *mut c_void) -> bool
{                                                          // C 3966-3981
  let mut info: *mut sfnt_font_info;

  info = open_fonts;                                       // C 3971
  while !info.is_null ()
    {
      if (*info).glyf_table_mapped                         // C 3973
	 && addr as *mut u8 >= (*(*info).glyf).glyphs      // C 3974
	 && (addr as *mut u8)
	    < (*(*info).glyf).glyphs.add ((*(*info).glyf).size)   // C 3975-3976
	{ return true; }                                   // C 3977
      info = (*info).next;
    }

  return false;                                            // C 3980
}

/* ----------------------------------------------------------------------------------------------
   C 4112-4222: the four initialization subrs, in C's order - `syms_of_sfntfont` (C 4114-4177:
   the seventeen `DEFSYM` notations, `sfnt_vendor_name`'s `staticpro`, the two `DEFVAR_LISP`s and
   the `DEFVAR_BOOL`), `mark_sfntfont` (C 4179-4196), `init_sfntfont` (C 4198-4202, whose body the
   reference itself writes empty) and `init_sfntfont_vendor` (C 4214-4222, which closes Owed 10 by
   assigning `sfnt_put_glyphs`).  **The only text this unit still owes is C 3984-4110**, the
   `HAVE_HARFBUZZ` arm; the ledger's `## C line ranges` names it row by row.

   C's `DEFSYM` has no Rust text (P-23: the symbols are `globals.rs`'s generated accessors), so
   each site is the `DEFSYM!` notation at C's own line and the docstrings are C's bytes - the
   `syms_of_alloc` shape of `src/alloc.rs`.  `init_sfntfont`'s empty body is not a stub this port
   invented: C has no statement between C 4201's `{` and C 4202's `}`, and the site says so.
   ---------------------------------------------------------------------------------------------- */
/* C's three `globals.f_<name>` fields this unit's `syms_of_sfntfont` registers (its
   `DEFVAR_LISP` calls at C 4148 and 4156, and its `DEFVAR_BOOL` at C 4171 - the sole `true`
   the C body writes is C 4176's own assignment, not this storage's initialiser), defined here
   as the port's storage - C's `struct emacs_globals` is zero-initialised and the DEFVAR macros
   only take each field's address, so the initialisers are `lisp_h_Qnil ()` / `false`
   (docs/status.md 3.29, the `src/cmds.rs` 489-492 row and the `src/image.rs` block shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut Vsfnt_default_family_alist: Lisp_Object = lisp_h_Qnil ();          // C 4148
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut Vsfnt_uninstructable_family_regexp: Lisp_Object = lisp_h_Qnil ();  // C 4156
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut sfnt_raster_glyphs_exactly: bool = false;                          // C 4171

/// C 4114-4177: `syms_of_sfntfont` - C's non-`static` entry point.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_sfntfont ()
{                                                          // C 4115-4177
  DEFSYM! (Qutf_16be, "utf-16be");                         // C 4117
  DEFSYM! (Qmac_roman, "mac-roman");                       // C 4118
  DEFSYM! (Qchinese_big5, "chinese-big5");                 // C 4119
  DEFSYM! (Qunicode_bmp, "unicode-bmp");                   // C 4120
  DEFSYM! (Qucs, "ucs");                                   // C 4121
  DEFSYM! (Qjapanese_jisx0208, "japanese-jisx0208");       // C 4122
  DEFSYM! (Qgbk, "gbk");                                   // C 4123
  DEFSYM! (Qkorean_ksc5601, "korean-ksc5601");             // C 4124
  DEFSYM! (Qapple_roman, "apple-roman");                   // C 4125
  DEFSYM! (Qjisx0208_1983_0, "jisx0208.1983-0");           // C 4126
  DEFSYM! (Qksc5601_1987_0, "ksc5601.1987-0");             // C 4127
  DEFSYM! (Qzh, "zh");                                     // C 4128
  DEFSYM! (Qja, "ja");                                     // C 4129
  DEFSYM! (Qko, "ko");                                     // C 4130
  DEFSYM! (Qfont_instance, "font-instance");               // C 4131

  /* Char-table purpose.  */
  DEFSYM! (Qfont_lookup_cache, "font-lookup-cache");       // C 4134

  /* Default foundry name.  */
  DEFSYM! (Qmisc, "misc");                                 // C 4137

  /* Predicated employed for sorting font family lists.  */
  DEFSYM! (Qstring_lessp, "string-lessp");                 // C 4140

  /* Set up staticpros.  */
  sfnt_vendor_name = Qnil ();                              // C 4143
  staticpro (&raw const sfnt_vendor_name);                 // C 4144

  /* This variable is supposed to be set by the platform specific part
     of the font backend.  */
  DEFVAR_LISP! (c"sfnt-default-family-alist", Vsfnt_default_family_alist,
    "Alist between \"emulated\" and actual font family names.\nMuch Emacs code assumes that font families named \"Monospace\" and \"Sans\nSerif\" exist, and map to the default monospace and Sans Serif fonts on\na system.  When the `sfnt' font driver is asked to look for a font\nwith one of the families in this alist, it uses its value instead.  ");   // C 4148-4153
  Vsfnt_default_family_alist = Qnil ();                    // C 4154

  DEFVAR_LISP! (c"sfnt-uninstructable-family-regexp", Vsfnt_uninstructable_family_regexp,
    "Regexp matching font families whose glyphs must not be instructed.\nIf nil, instruction code supplied by all fonts will be executed.  This\nvariable takes effect when a font entity is opened, not after, and\ntherefore won't affect the scaling of realized faces until their\nframes' font caches are cleared (see `clear-font-cache').\n\nTrueType fonts incorporate instruction code executed to fit each glyph\nto a pixel grid, so as to improve the visual fidelity of each glyph by\neliminating artifacts and chance effects consequent upon the direct\nupscaling of glyph outline data.  Instruction code is occasionally\nincompatible with Emacs and must be disregarded.  ");   // C 4156-4168
  Vsfnt_uninstructable_family_regexp = Qnil ();            // C 4169

  DEFVAR_BOOL! (c"sfnt-raster-glyphs-exactly", sfnt_raster_glyphs_exactly,
    "How font glyph outlines should be converted to graphics.\nIf non-nil, glyphs will be displayed in a more precise manner, at the\ncost of performance on devices where floating-point math operations\nare slow.  ");                                          // C 4171-4175
  sfnt_raster_glyphs_exactly = true;                       // C 4176
}

// C: lisp.h 4436: `extern void mark_object (Lisp_Object);` - no mirror of this unit's include list
// carries it (`include/lisp.rs` declares `mark_objects`, the counted form), so it is declared at
// its use site (P-72 rule 3 / P-55, the `src/thread.rs` 95-97 and `src/image.rs` 4925-4927 shape).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {

}

/// C 4179-4196: `mark_sfntfont` - C's non-`static` entry point.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_sfntfont ()
{                                                          // C 4180-4196
  let mut desc: *mut sfnt_font_desc;

  /* Mark each font desc.  */
  desc = system_fonts;                                     // C 4185
  while !desc.is_null ()
    {
      mark_object ((*desc).family);                        // C 4187
      mark_object ((*desc).style);                         // C 4188
      mark_object ((*desc).adstyle);                       // C 4189
      mark_object ((*desc).instances);                     // C 4190
      mark_object ((*desc).languages);                     // C 4191
      mark_object ((*desc).registry);                      // C 4192
      mark_object ((*desc).char_cache);                    // C 4193
      mark_object ((*desc).designer);                      // C 4194
      desc = (*desc).next;
    }
}

/// C 4198-4202: `init_sfntfont` - C's non-`static` entry point.  **Its body is empty in the
/// reference** (nothing between C 4201's `{` and C 4202's `}`), so the port carries no statement
/// either; the empty block is C's text, not a stub.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_sfntfont ()
{                                                          // C 4199-4202
                                                           // C 4201: C's body has no statement.
}

/// C 4214-4222: `init_sfntfont_vendor` - C's non-`static` entry point; it closes **Owed 10** by
/// assigning the object C 3667 declares.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_sfntfont_vendor (vendor_name: Lisp_Object,
					       driver: *const font_driver,
					       put_glyphs: sfntfont_put_glyph_proc)
{                                                          // C 4215-4222
  sfnt_vendor_name = vendor_name;                          // C 4219
  sfnt_font_driver = driver;                               // C 4220
  sfnt_put_glyphs = put_glyphs;                            // C 4221
}

/* ----------------------------------------------------------------------------------------------
   C 3984-4110: the file's last text - C 3985's form feed, C 3987's
   `/* Harfbuzz font support.  */` banner and C 3989's live `#ifdef HAVE_HARFBUZZ` block, carried
   under `#[cfg (feature = "harfbuzz")]` because `HAVE_HARFBUZZ` is `undef` on this target
   (P-51; the feature is the tree's registered name and `src/hbfont.rs` is the port that reads the
   same headers).  Inside it: C 3991's live `#ifdef HAVE_MMAP` with `sfntfont_unmap_blob`
   (C 3995-4002), `sfntfont_get_font_table` (C 4009-4052) and `sfntfont_begin_hb_font`
   (C 4058-4108, declared in `include/sfntfont.rs`'s `harfbuzz` arm).  The `hb_*` names come from
   the declaration segment at C 37-38 above, and `sfnt_map_table`/`sfnt_unmap_table`/
   `sfnt_read_table`/`sfnt_mapped_table` from `include/sfnt.rs` (2355-2367, 1310).

   The two dead arms inside are carried under the feature that names the non-frozen side and
   bound by an assertion (P-51): C 4038-4042's `#else` of a macro this configuration defines, and
   C 4090-4095's `#ifdef HAVE_HB_FONT_SET_VAR_NAMED_INSTANCE`, which the frozen table leaves
   `undef` (`docs/baseline/config/darwin-arm64.macros.tsv` 1390).  **This closes the file**: C 1-4222
   is now tiled by the ledger's rows, and the only reading left is the line-count question.
   ---------------------------------------------------------------------------------------------- */
/// The frozen configuration's `HAVE_MMAP`, read from `docs/baseline/config/darwin-arm64.macros.tsv`:
/// 1.  C 4014-4042's `#else` arm is therefore the non-frozen side, which the feature `no_mmap`
/// names (P-51).
const HAVE_MMAP: bool = true;
const _: () = core::assert! (!cfg! (feature = "no_mmap") || !HAVE_MMAP);
/// `HAVE_HB_FONT_SET_VAR_NAMED_INSTANCE` is `undef` in the same table (1390), so C 4090-4095's
/// arm is dead text whose feature names the non-frozen side, `have_hb_font_set_var_named_instance`.
const HAVE_HB_FONT_SET_VAR_NAMED_INSTANCE: bool = false;
const _: () = core::assert! (!cfg! (feature = "have_hb_font_set_var_named_instance")
			     || HAVE_HB_FONT_SET_VAR_NAMED_INSTANCE);

/// C 3995-4002: `sfntfont_unmap_blob` - C's `static`, and the address of a `static` in this
/// repository is taken through its name, so the definition stays a plain `unsafe fn` while
/// C 4030 hands its address to `hb_blob_create`.
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	   feature = "harfbuzz"))]
unsafe extern "C" fn sfntfont_unmap_blob (ptr: *mut c_void)
{                                                          // C 3995-4002
  if sfnt_unmap_table (ptr as *mut sfnt_mapped_table) != 0      // C 3998
    { emacs_abort (); }                                    // C 3999

  xfree (ptr);                                             // C 4001
}

/// C 4009-4052: `sfntfont_get_font_table` - C's `static`, handed to `hb_face_create_for_tables`
/// at C 4075.
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	   feature = "harfbuzz"))]
unsafe extern "C" fn sfntfont_get_font_table (face: *mut hb_face_t, tag: hb_tag_t,
					      mut data: *mut c_void) -> *mut hb_blob_t
{                                                          // C 4009-4052
  /* C 4012's `size` is the out-parameter of `sfnt_read_table` (C 4044-4045) and C 4050 reads it;
     zero-initialised because its address is taken (PORTING.md 1.5, Deviation 10).  C 4044
     reassigns the `data` parameter, which is why it is bound `mut` (the `xpalloc` shape).  */
  let mut size: usize = 0;
  let mut info: *mut sfnt_font_info;
  let mut table: *mut sfnt_mapped_table;
  let mut blob: *mut hb_blob_t;

  info = data as *mut sfnt_font_info;                      // C 4018
  table = xmalloc (std::mem::size_of::<sfnt_mapped_table> ()) as *mut sfnt_mapped_table;   // C 4019

  if sfnt_map_table ((*info).fd, (*info).directory, tag,   // C 4021-4022
		     table) == 0
    {
      /* Create an hb_blob_t and return it.
         TODO: record this mapping properly so that SIGBUS can
	 be handled.  */

      blob = hb_blob_create ((*table).data as *const c_char, (*table).length as c_uint,   // C 4028-4030
			     HB_MEMORY_MODE_READONLY,
			     table as *mut c_void,
			     Some (sfntfont_unmap_blob));

      /* Note that sfntfont_unmap_blob will be called if the empty
	 blob is returned.  */
      return blob;                                         // C 4034
    }

  xfree (table as *mut c_void);                            // C 4037
  /* C 4038-4042's `#else /* !HAVE_MMAP */` arm (`info = data;`, C 4041) is dead on this target:
     the condition's frozen value is 1, so the arm sits under the feature naming the other side
     (P-51, the assertion above).  */
  #[cfg (feature = "no_mmap")]
  { info = data as *mut sfnt_font_info; }                  // C 4041

  data = sfnt_read_table ((*info).fd, (*info).directory, tag,   // C 4044-4045
			  &mut size);

  if data.is_null ()                                       // C 4047
    { return std::ptr::null_mut (); }                      // C 4048

  return hb_blob_create (data as *const c_char, size as c_uint,   // C 4050-4051
			 HB_MEMORY_MODE_WRITABLE,
			 data, Some (xfree_wrapper));      // C 4051: C's `xfree`, the mirror's wrapper
}

/// C 4058-4108: `sfntfont_begin_hb_font` - C's non-`static` entry point of this arm.
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
	   feature = "harfbuzz"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sfntfont_begin_hb_font (font: *mut font,
						 position_unit: *mut c_double)
						 -> *mut c_void
{                                                          // C 4058-4108
  let mut info: *mut sfnt_font_info;
  let mut face: *mut hb_face_t;
  let mut factor: c_int;

  info = font as *mut sfnt_font_info;                      // C 4065

  if !(*info).hb_font.is_null ()                           // C 4067
    {
      /* Calculate the scale factor.  */
      *position_unit = 1.0 / 64.0;                         // C 4070
      return (*info).hb_font;                              // C 4071
    }

  /* Create a face and then a font.  */
  face = hb_face_create_for_tables (Some (sfntfont_get_font_table),   // C 4075-4076
				    font as *mut c_void,
				    None);

  /* C 4082's `goto bail` leaves the block and lands at C 4102's `bail:`, whose only statement is
     C 4103's `hb_face_destroy`; the labelled block is that region and the `break` is the jump
     (the `sfntfont_open` `cancel_blend` shape).  */
  'bail: {
    if hb_face_get_glyph_count (face) > 0                  // C 4078
      {
	(*info).hb_font = hb_font_create (face) as *mut c_void;   // C 4080
	if (*info).hb_font.is_null ()                      // C 4081
	  { break 'bail; }                                 // C 4082

	factor = (*font).pixel_size;                       // C 4084

	/* Set the scale and PPEM values.  */
	hb_font_set_scale ((*info).hb_font as *mut hb_font_t, factor * 64, factor * 64);   // C 4087
	hb_font_set_ppem ((*info).hb_font as *mut hb_font_t, factor as c_uint, factor as c_uint);   // C 4088

	/* C 4090-4095's `#ifdef HAVE_HB_FONT_SET_VAR_NAMED_INSTANCE`: the frozen table leaves the
	   macro `undef`, so the arm is dead text under the feature naming the non-frozen side
	   (P-51, the assertion above).  */
	#[cfg (feature = "have_hb_font_set_var_named_instance")]
	{
	  /* Set the instance if this is a distortable font.  */
	  if (*info).instance != -1                        // C 4092
	    { hb_font_set_var_named_instance ((*info).hb_font as *mut hb_font_t, (*info).instance as c_uint); }   // C 4093-4094
	}

	/* This is needed for HarfBuzz before 2.0.0; it is the default
	   in later versions.  */
	hb_ot_font_set_funcs ((*info).hb_font as *mut hb_font_t);   // C 4099
      }
  }
  hb_face_destroy (face);                                  // C 4103

  /* Calculate the scale factor.  */
  *position_unit = 1.0 / 64.0;                             // C 4106
  return (*info).hb_font;                                  // C 4107
}
