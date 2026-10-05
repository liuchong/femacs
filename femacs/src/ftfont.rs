// femacs: port of src/ftfont.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The C non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` (none of this slice has its address taken).
//
// **This file is P-86's unit `src/ftfont.c` (3,243 lines) and is ported in slices; this is slice 1
// = C 1-1352.**  The slice ends on the `}` of `ftfont_entity_pattern` (C 1352), which is the last
// of the three entry points `ftfont.h` declares; **the next definition is `ftfont_open` (C 1357),
// whose `#ifndef USE_CAIRO` arm opens at C 1354**.  The ledger `docs/ports/ftfont.md` carries P-86
// ②'s C line-range table for C 1-3243, the `## Evidence` readings and the `## Deviations`.
//
// The frozen configuration compiles no `ftfont.o`: `configure.ac` 7199-7203 adds it to `FONT_OBJ`
// only inside the X11 branch under `HAVE_CAIRO`, `HAVE_XFT` or `HAVE_FREETYPE`, and 7208/7213 do
// the same for the `window_system=pgtk` and be-app+cairo branches - every one of them has
// `HAVE_FREETYPE=yes` (cairo requires libfreetype; 4525), which is the file gate this port uses.
// `HAVE_FREETYPE` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1342,
// `FONT_OBJ=` is empty in `build/emacs-ref/src/Makefile` 342, and
// `docs/baseline/frozen-build-sources.txt` lists no `src/ftfont.o`, so the frozen configuration
// compiles not one line of this unit (the `## Evidence` 336-byte object).  `src/ftfont.c` *is* in
// `docs/baseline/config/cond-eval-verified.txt` (286), so `extract-abi.py` reads its `#if`s and the
// inventory is a live statement about the file (P-86 ⑩): the gates, in C's order:
//
//   * the file gate is `feature = "have_freetype"` (HAS_FREETYPE's own macro name), on every item
//     *and on C 22-47's include graph* (P-86 ⑧ - an include outside the gate compiles text the
//     frozen configuration never sees).
//   * C 29-32's `#ifdef HAVE_XFT` is `feature = "have_xft"`; C 33-35's `#ifdef USE_CAIRO` is
//     `feature = "use_cairo"`; C 50-52's `#ifdef HAVE_HARFBUZZ` is `feature = "harfbuzz"`
//     (`docs/baseline/feature-coverage.tsv` carries all three; `harfbuzz` is the registered
//     spelling of `HAVE_HARFBUZZ`, `femacs/xtask/Cargo.toml` 109).
//   * C 472-493 and C 570-590's `#ifdef HAVE_LIBOTF` / `#elif defined HAVE_HARFBUZZ` are
//     `feature = "have_libotf"` and `feature = "harfbuzz"`; `have_libotf` is **not** in the
//     feature table yet (the ledger's Owed 5 names it for the integrator).
//   * C 155/203's `#ifdef FC_FONTFORMAT` / `#ifdef FC_VARIABLE`, C 921/1005's `#ifdef FC_CAPABILITY`
//     and C 1297's `#ifdef FC_EMBOLDEN` are fontconfig's own header macros (`fc_fontformat`,
//     `fc_variable`, `fc_capability`, `fc_embolden`), and C 927/972's `defined HAVE_XFT &&
//     defined FC_COLOR` is `all (feature = "have_xft", feature = "fc_color")`.  These five names
//     are not in the tree's feature table either (Owed 5).
//   * C 659-663 (also 823-827, 972-976)'s nested `#if (XFT_MAJOR < 2 || … XFT_REVISION < 6)` is
//     the derived condition "the Xft headers are older than 2.3.6", which no Rust `#[cfg]` can
//     compute from the header's version macros: it is the one feature
//     `feature = "xft_version_lt_2_3_6"` (the `webkit_check_version_ge_2_16` /
//     `sqlite_version_number_ge_3007015` shape, `femacs/xtask/Cargo.toml` 683/700).  It is not
//     registered yet (Owed 5), and this machine has no Xft headers at all (Evidence).
//   * C 942-968's `#if 0` is dead in every configuration, so it is recorded at its place rather
//     than transcribed (the `src/xfns.rs` 958 `#if false` / `androidfns.md` Deviation 2 shape).
//   * C 1227-1229's `#ifndef FC_HINT_STYLE` and C 1235-1238's `#ifndef FC_LCD_FILTER` are the
//     header's *fallback* definitions of two string macros; the port declares each name once in
//     the fontconfig segment below (C 23-24), which is what the two `#ifndef`s stand for.
//
// `<fontconfig/fontconfig.h>`, `<fontconfig/fcfreetype.h>`, `<X11/Xlib.h>`, `<X11/Xft/Xft.h>`,
// `<cairo-ft.h>`, `otf.h`, `m17n-flt.h` and `<hb.h>` have no file in the reference tree, so the
// declarations C 1-1352 uses stand in this unit at the C `#include` line (PORTING.md P-88 ①: a
// unit-local segment, *not* a file under `femacs/src/include/`, which mirrors the reference's own
// headers).  Every declaration carries the header's line (P-55) and the records' layouts are clang
// readings from this machine's headers, not memory (P-88 ②: the probe and its output are in the
// ledger's `## Evidence`).  Fontconfig, FreeType and Xft each have three or more consumers in the
// tree (`src/ftfont.c`, `src/xftfont.c`, `src/ftcrfont.c`, `src/ftfont.h`; fontconfig also
// `src/xsettings.c` and `lwlib/lwlib-utils.h`), so P-90 ① wants a dedicated SDK mirror each
// (`femacs/src/include/fontconfig.rs`, `.../freetype.rs`, `.../xft.rs`): Owed 4 names them, and
// when one lands this segment moves into it and disappears from here (P-90 ③ / P-88 ⑤).
//
// Names the mirrors this unit includes do not carry are declared at the use site with the
// reference header's line - one citation per line, so a citation is not paired with the wrong name:
//   `font_make_entity`       font.h 842
//   `font_style_to_value`    font.h 854
//   `font_intern_prop`       font.h 885
//   `font_put_extra`         font.h 913
//   `font_add_log`           font.h 1001 (the mirror's own `FONT_ADD_LOG!` body calls it)
// and `strstr`, which the frozen `<string.h>` mirror (`femacs/lib/include/string.rs`) does not
// declare (it carries `strlen`/`strcmp`/`strcpy`).  Each is named in the ledger's Owed 3 with the
// mirror it belongs in.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

use std::ffi::CStr;                            // the two FC_* constants C 1227-1238 define

include!("include/config.rs");                 // C 22: `#include <config.h>` (every unit includes it)

/* ----------------------------------------------------------------------------------------------
   C 23-24: `#include <fontconfig/fontconfig.h>` and `#include <fontconfig/fcfreetype.h>` - the
   two Fontconfig headers, which the reference tree does not carry (P-88 ①: the declarations this
   unit uses stand here, at the `#include` line, rather than in a new file under
   `femacs/src/include/`, which is the *reference* headers' mirror directory).

   Measured with clang against this machine's header (`/opt/homebrew/include/fontconfig`,
   `FC_VERSION` 2.18.3; the probe and its output are in the ledger's `## Evidence`):
   `sizeof (FcChar8)` **1**, `FcBool` **4**, `FcChar32` **4**; `sizeof (FcFontSet)` **16**, align
   **8** (`nfont`@0, `sfont`@4, `fonts`@8); `sizeof (FcValue)` **16**, align **8** (`type`@0,
   `u`@8, every union member at 8); `FcResultMatch` **0**, `FcResultNoMatch` **1**,
   `FcResultTypeMismatch` **2**, `FcResultNoId` **3**, `FcResultOutOfMemory` **4**; `FcTypeInteger`
   **1**, `FcTypeDouble` **2**, `FcTypeString` **3**, `FcTypeBool` **4**, `FcTypeCharSet` **6**,
   `FcTypeLangSet` **8**, `FcTypeRange` **9**; `FcMatchPattern` **0**, `FcMatchFont` **1**,
   `FcMatchScan` **2**; `FcTrue` **1**, `FcFalse` **0**.  The string macros' byte values are the
   header's own literals (measured: `FC_HINT_STYLE` "hintstyle", `FC_LCD_FILTER` "lcdfilter",
   `FC_ANTIALIAS` "antialias", `FC_AUTOHINT` "autohint", `FC_RGBA` "rgba", `FC_DPI` "dpi", the
   rest as spelled below).  Every value above belongs to the *Fontconfig* headers, which the frozen
   configuration does not have; the numbers will move with the mirror (Owed 4).  */

/// C: fontconfig.h 45: `typedef unsigned char  FcChar8;`
#[cfg (feature = "have_freetype")]
pub type FcChar8 = c_uchar;
/// C: fontconfig.h 47: `typedef unsigned int   FcChar32;`
#[cfg (feature = "have_freetype")]
pub type FcChar32 = c_uint;
/// C: fontconfig.h 48: `typedef int            FcBool;`
#[cfg (feature = "have_freetype")]
pub type FcBool = c_int;
/// C: fontconfig.h 266: `typedef struct _FcCharSet FcCharSet;` - an incomplete type named only
/// through a pointer (PORTING.md 0.1-2's zero-sized stand-in, the `include/xlib.rs` 878 shape).
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct _FcCharSet { _private: [u8; 0] }
/// C: fontconfig.h 266: the pointer type C spells `FcCharSet *`.
#[cfg (feature = "have_freetype")]
pub type FcCharSet = _FcCharSet;
/// C: fontconfig.h 295: `typedef struct _FcPattern FcPattern;`
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct _FcPattern { _private: [u8; 0] }
/// C: fontconfig.h 295: the pointer type C spells `FcPattern *`.
#[cfg (feature = "have_freetype")]
pub type FcPattern = _FcPattern;
/// C: fontconfig.h 302: `typedef struct _FcLangSet FcLangSet;`
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct _FcLangSet { _private: [u8; 0] }
/// C: fontconfig.h 302: the pointer type C spells `FcLangSet *`.
#[cfg (feature = "have_freetype")]
pub type FcLangSet = _FcLangSet;
/// C: fontconfig.h 304: `typedef struct _FcRange FcRange;`
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct _FcRange { _private: [u8; 0] }
/// C: fontconfig.h 304: the pointer type C spells `FcRange *`.
#[cfg (feature = "have_freetype")]
pub type FcRange = _FcRange;
/// C: fontconfig.h 374: `typedef struct _FcConfig FcConfig;` - named only through a pointer
/// (`FcFontList`/`FcFontMatch`/`FcConfigSubstitute`'s first parameter, which C 437/349/347 pass as
/// `NULL`).
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct _FcConfig { _private: [u8; 0] }
/// C: fontconfig.h 268: the pointer type C spells `FcConfig *`.
#[cfg (feature = "have_freetype")]
pub type FcConfig = _FcConfig;
/// C: fontconfig.h 327-333: `typedef struct _FcObjectSet { int nobject; int sobject; const char
/// **objects; int nobjIds; FcObject *objIds; } FcObjectSet;`  C 425-452 only holds one by pointer
/// (no member is read), so the record stands in as the incomplete-type shape above.
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct _FcObjectSet { _private: [u8; 0] }
/// C: fontconfig.h 327-333: the pointer type C spells `FcObjectSet *`.
#[cfg (feature = "have_freetype")]
pub type FcObjectSet = _FcObjectSet;
/// C: fontconfig.h 321-325: `typedef struct _FcFontSet { int nfont; int sfont; FcPattern **fonts;
/// } FcFontSet;`  C 440-441 and C 1192-1193 read `nfont` and `fonts`, so the record is written out
/// and its layout asserted below.
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct FcFontSet
{
  pub nfont: c_int,
  pub sfont: c_int,
  pub fonts: *mut *mut FcPattern,
}
#[cfg (feature = "have_freetype")]
const _: () = {
  core::assert! (std::mem::size_of::<FcFontSet> () == 16);
  core::assert! (std::mem::align_of::<FcFontSet> () == 8);
  core::assert! (std::mem::offset_of! (FcFontSet, nfont) == 0);
  core::assert! (std::mem::offset_of! (FcFontSet, sfont) == 4);
  core::assert! (std::mem::offset_of! (FcFontSet, fonts) == 8);
};
/// C: fontconfig.h 238-250: `typedef enum _FcType { FcTypeUnknown = -1, FcTypeVoid,
/// FcTypeInteger, … FcTypeRange } FcType;`  C's enum ABI is `c_int` (PORTING.md 1.3) and the
/// members this slice names are the constants below (the `include/gtk.rs` 373-378 shape).
#[cfg (feature = "have_freetype")]
pub type FcType = c_int;
/// C: fontconfig.h 241-249: `FcTypeDouble,` is 2 (`FcTypeInteger` 1, `FcTypeString` 3, `FcTypeBool`
/// 4, `FcTypeCharSet` 6, `FcTypeLangSet` 8, `FcTypeRange` 9).
#[cfg (feature = "have_freetype")]
pub const FcTypeInteger: FcType = 1;
#[cfg (feature = "have_freetype")]
pub const FcTypeDouble: FcType = 2;
#[cfg (feature = "have_freetype")]
pub const FcTypeString: FcType = 3;
#[cfg (feature = "have_freetype")]
pub const FcTypeBool: FcType = 4;
#[cfg (feature = "have_freetype")]
pub const FcTypeCharSet: FcType = 6;
#[cfg (feature = "have_freetype")]
pub const FcTypeLangSet: FcType = 8;
#[cfg (feature = "have_freetype")]
pub const FcTypeRange: FcType = 9;
/// C: fontconfig.h 279-285: `typedef enum _FcResult { FcResultMatch, FcResultNoMatch,
/// FcResultTypeMismatch, FcResultNoId, FcResultOutOfMemory } FcResult;`
#[cfg (feature = "have_freetype")]
pub type FcResult = c_int;
#[cfg (feature = "have_freetype")]
pub const FcResultMatch: FcResult = 0;
#[cfg (feature = "have_freetype")]
pub const FcResultNoMatch: FcResult = 1;
#[cfg (feature = "have_freetype")]
pub const FcResultTypeMismatch: FcResult = 2;
#[cfg (feature = "have_freetype")]
pub const FcResultNoId: FcResult = 3;
#[cfg (feature = "have_freetype")]
pub const FcResultOutOfMemory: FcResult = 4;
/// C: fontconfig.h 335-341: `typedef enum _FcMatchKind { FcMatchPattern, FcMatchFont,
/// FcMatchScan, FcMatchKindEnd, FcMatchKindBegin = FcMatchPattern } FcMatchKind;`
#[cfg (feature = "have_freetype")]
pub type FcMatchKind = c_int;
#[cfg (feature = "have_freetype")]
pub const FcMatchPattern: FcMatchKind = 0;
#[cfg (feature = "have_freetype")]
pub const FcMatchFont: FcMatchKind = 1;
#[cfg (feature = "have_freetype")]
pub const FcMatchScan: FcMatchKind = 2;
/// C: fontconfig.h 306-318: `typedef struct _FcValue { FcType type; union { const FcChar8 *s;
/// int i; FcBool b; double d; const FcMatrix *m; const FcCharSet *c; void *f; const FcLangSet
/// *l; const FcRange *r; } u; } FcValue;`  C 1122-1125 builds one by value and passes it to
/// `FcPatternAdd`, so the record and the union are written out (union members must be `Copy`,
/// PORTING.md 0.1-9).
#[cfg (feature = "have_freetype")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union FcValue_u
{
  /// C: fontconfig.h 309: `const FcChar8   *s;`
  pub s: *const FcChar8,
  /// C: fontconfig.h 310: `int              i;`
  pub i: c_int,
  /// C: fontconfig.h 311: `FcBool           b;`
  pub b: FcBool,
  /// C: fontconfig.h 312: `double           d;`
  pub d: c_double,
  /// C: fontconfig.h 313-317: `const FcMatrix *m; const FcCharSet *c; void *f; const FcLangSet
  /// *l; const FcRange *r;` - the four records those pointers name are not this slice's.
  pub m: *const c_void,
  pub c: *const FcCharSet,
  pub f: *mut c_void,
  pub l: *const FcLangSet,
  pub r: *const FcRange,
}
/// C: fontconfig.h 306-318: the record itself.  C spells the first member `type`, a Rust keyword,
/// which the tree spells `r#type` (`src/alloc.rs` 563).
#[cfg (feature = "have_freetype")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FcValue
{
  pub r#type: FcType,
  pub u: FcValue_u,
}
#[cfg (feature = "have_freetype")]
const _: () = {
  core::assert! (std::mem::size_of::<FcValue> () == 16);
  core::assert! (std::mem::offset_of! (FcValue, r#type) == 0);
  core::assert! (std::mem::offset_of! (FcValue, u) == 8);
};
/// C: fontconfig.h 85-86: `#define FcFalse                 0` / `#define FcTrue                  1` (the header also spells the
/// pair `(char)1`/`(char)0`; `FcBool` is `int`, measured 4 and the values 1/0).
#[cfg (feature = "have_freetype")]
pub const FcTrue: FcBool = 1;
#[cfg (feature = "have_freetype")]
pub const FcFalse: FcBool = 0;

/* C 23-24's other effect: `<fontconfig/fcfreetype.h>` includes `<ft2build.h>`, which reaches
   FreeType's `freetype/config/ftstdlib.h` - the header that includes `<stdlib.h>` and `<string.h>`
   (measured with `clang -H`: `ftstdlib.h` 36 pulls `/usr/include/string.h`, 88 pulls
   `/usr/include/stdlib.h`).  That chain is where this slice's `strlen`, `strcmp`, `strcpy`,
   `malloc` and `free` come from in C, so the two mirrors are included here rather than at a line
   the file's own text does not have.  */
#[cfg (feature = "have_freetype")]
include!("../lib/include/string.rs");          // C 24 -> fcfreetype.h -> ftstdlib.h -> string.h
#[cfg (feature = "have_freetype")]
include!("../lib/include/stdlib.rs");          // C 24 -> fcfreetype.h -> ftstdlib.h -> stdlib.h

/* ----------------------------------------------------------------------------------------------
   C 23-24 (continued): the FreeType surface.  `<ft2build.h>` and `FT_FREETYPE_H` are declared by
   `src/ftfont.h`'s include chain in C (ftfont.h 27-31); this slice of the unit names four entry
   points and two record types, and nothing else, so that is what stands here (P-88 ①: "the
   enumerated surface is exactly what the C names").

   Measured with clang against this machine's headers (`/opt/homebrew/include/freetype2`,
   `FREETYPE_MAJOR` 2 `FREETYPE_MINOR` 14 `FREETYPE_PATCH` 3): `sizeof (FT_Library)` **8**,
   `sizeof (FT_Face)` **8** (both are incomplete records named through a pointer); FT_BDF_H is
   defined once `FT_FREETYPE_H` is in (so C 297's `BDF_PropertyRec` is available in C without an
   extra `#include`), `sizeof (BDF_PropertyRec)` **16**, align **8** (`type`@0, `u`@8,
   `u.integer`@8), `BDF_PROPERTY_TYPE_NONE` **0**, `BDF_PROPERTY_TYPE_ATOM` **1**,
   `BDF_PROPERTY_TYPE_INTEGER` **2**, `BDF_PROPERTY_TYPE_CARDINAL` **3**.  `FT_Error` is `int`
   (the port's `c_int`) and `FT_Long`/`FT_ULong` are the platform's `long`/`unsigned long`
   (ftconfig.h).  */

/// C: freetype/freetype.h 557: `typedef struct FT_LibraryRec_  *FT_Library;`
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct FT_LibraryRec_ { _private: [u8; 0] }
/// C: freetype/freetype.h 557: the pointer type C spells `FT_Library`.
#[cfg (feature = "have_freetype")]
pub type FT_Library = *mut FT_LibraryRec_;
/// C: freetype/freetype.h 641: `typedef struct FT_FaceRec_*  FT_Face;`
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct FT_FaceRec_ { _private: [u8; 0] }
/// C: freetype/freetype.h 641: the pointer type C spells `FT_Face`.
#[cfg (feature = "have_freetype")]
pub type FT_Face = *mut FT_FaceRec_;
/// C: freetype.h 395-398: `typedef unsigned long  FT_ULong;` / `typedef unsigned int   FT_UInt;`
/// (`typedef int FT_Error;` is 388-391).  Nothing in C 1-1352 names `FT_Error` in a declaration
/// the port writes out - the two entry points below return C's own `FT_Error`, spelled `c_int`.
#[cfg (feature = "have_freetype")]
pub type FT_ULong = c_ulong;
/// C: freetype.h 395-398: `FT_UInt` (the 32-bit half of the same pair).
#[cfg (feature = "have_freetype")]
pub type FT_UInt = c_uint;
/// C: freetype.h 387-390: `typedef unsigned int FT_Error;` - `FT_Error` is `int`.
#[cfg (feature = "have_freetype")]
pub type FT_Error = c_int;
/// C: ftbdf.h 74-81: `typedef enum  BDF_PropertyType_ { BDF_PROPERTY_TYPE_NONE = 0,
/// BDF_PROPERTY_TYPE_ATOM = 1, BDF_PROPERTY_TYPE_INTEGER = 2, BDF_PROPERTY_TYPE_CARDINAL = 3 }
/// BDF_PropertyType;`
#[cfg (feature = "have_freetype")]
pub type BDF_PropertyType = c_int;
#[cfg (feature = "have_freetype")]
pub const BDF_PROPERTY_TYPE_NONE: BDF_PropertyType = 0;
#[cfg (feature = "have_freetype")]
pub const BDF_PROPERTY_TYPE_ATOM: BDF_PropertyType = 1;
#[cfg (feature = "have_freetype")]
pub const BDF_PROPERTY_TYPE_INTEGER: BDF_PropertyType = 2;
#[cfg (feature = "have_freetype")]
pub const BDF_PROPERTY_TYPE_CARDINAL: BDF_PropertyType = 3;
/// C: ftbdf.h 118-128: `typedef struct  BDF_PropertyRec_ { BDF_PropertyType type; union { const
/// char *atom; FT_Int32 integer; FT_UInt32 cardinal; } u; } BDF_PropertyRec;`  C 297-301 reads
/// `type` and `u.integer`, so the record and the union are written out.
#[cfg (feature = "have_freetype")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union BDF_PropertyRec_u
{
  /// C: ftbdf.h 122: `const char*     atom;`
  pub atom: *const c_char,
  /// C: ftbdf.h 123: `FT_Int32        integer;`
  pub integer: c_int,
  /// C: ftbdf.h 124: `FT_UInt32       cardinal;`
  pub cardinal: c_uint,
}
/// C: ftbdf.h 118-128: the record itself.
#[cfg (feature = "have_freetype")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BDF_PropertyRec
{
  pub r#type: BDF_PropertyType,
  pub u: BDF_PropertyRec_u,
}
#[cfg (feature = "have_freetype")]
const _: () = {
  core::assert! (std::mem::size_of::<BDF_PropertyRec> () == 16);
  core::assert! (std::mem::align_of::<BDF_PropertyRec> () == 8);
  core::assert! (std::mem::offset_of! (BDF_PropertyRec, r#type) == 0);
  core::assert! (std::mem::offset_of! (BDF_PropertyRec, u) == 8);
};
/* C: ftbdf.h 206-208: `FT_EXPORT( FT_Error ) FT_Get_BDF_Property( FT_Face face, const char*
   prop_name, BDF_PropertyRec *aproperty );`  */
#[cfg (feature = "have_freetype")]
unsafe extern "C" {
  /// C: freetype.h 2350-2351: `FT_EXPORT( FT_Error ) FT_Init_FreeType( FT_Library *alibrary );`
  pub fn FT_Init_FreeType (alibrary: *mut FT_Library) -> FT_Error;
  /// C: freetype.h 2573-2576: `FT_EXPORT( FT_Error ) FT_New_Face( FT_Library library, const
  /// char* filepathname, FT_Long face_index, FT_Face *aface );`
  pub fn FT_New_Face (library: FT_Library, filepathname: *const c_char,
                      face_index: c_long, aface: *mut FT_Face) -> FT_Error;
  /// C: freetype.h 2878-2879: `FT_EXPORT( FT_Error ) FT_Done_Face( FT_Face face );`
  pub fn FT_Done_Face (face: FT_Face) -> FT_Error;
  /// C: ftbdf.h 206-208 (the line quoted above).
  pub fn FT_Get_BDF_Property (face: FT_Face, prop_name: *const c_char,
                              aproperty: *mut BDF_PropertyRec) -> FT_Error;
}

/* ----------------------------------------------------------------------------------------------
   C 23-24 (continued): Fontconfig's entry points.  Every declaration below is the header's own
   line, with `FC_ATTRIBUTE_SENTINEL`/`FC_ATTRIBUTE_DEPRECATED` dropped the way PORTING.md 8 drops
   attributes whose port counterpart is their absence (the `_GL_ATTRIBUTE_PURE` reading in
   `lib/include/c-strcase.rs`).  `FcConfig *config` is written `*mut FcConfig` (the `NULL` C 347
   passes is `std::ptr::null_mut ()`).  */
#[cfg (feature = "have_freetype")]
unsafe extern "C" {
  /// C: fontconfig.h 726: `FcPublic FcBool FcInit (void);`
  pub fn FcInit () -> FcBool;
  /// C: fontconfig.h 940: `FcPublic FcPattern * FcPatternCreate (void);`
  pub fn FcPatternCreate () -> *mut FcPattern;
  /// C: fontconfig.h 943: `FcPublic FcPattern * FcPatternDuplicate (const FcPattern *p);`
  pub fn FcPatternDuplicate (p: *const FcPattern) -> *mut FcPattern;
  /// C: fontconfig.h 961: `FcPublic void FcPatternDestroy (FcPattern *p);`
  pub fn FcPatternDestroy (p: *mut FcPattern);
  /// C: fontconfig.h 1045: `FcPublic FcPattern * FcPatternBuild (FcPattern *p, ...);`
  pub fn FcPatternBuild (p: *mut FcPattern, ...) -> *mut FcPattern;
  /// C: fontconfig.h 976: `FcPublic FcBool FcPatternAdd (FcPattern *p, const char *object,
  /// FcValue value, FcBool append);`
  pub fn FcPatternAdd (p: *mut FcPattern, object: *const c_char, value: FcValue,
                       append: FcBool) -> FcBool;
  /// C: fontconfig.h 1009: `FcPublic FcBool FcPatternAddBool (FcPattern *p, const char *object,
  /// FcBool b);`
  pub fn FcPatternAddBool (p: *mut FcPattern, object: *const c_char, b: FcBool) -> FcBool;
  /// C: fontconfig.h 994: `FcPublic FcBool FcPatternAddInteger (FcPattern *p, const char *object,
  /// int i);`
  pub fn FcPatternAddInteger (p: *mut FcPattern, object: *const c_char, i: c_int) -> FcBool;
  /// C: fontconfig.h 997: `FcPublic FcBool FcPatternAddDouble (FcPattern *p, const char *object,
  /// double d);`
  pub fn FcPatternAddDouble (p: *mut FcPattern, object: *const c_char, d: c_double) -> FcBool;
  /// C: fontconfig.h 1000: `FcPublic FcBool FcPatternAddString (FcPattern *p, const char *object,
  /// const FcChar8 *s);`
  pub fn FcPatternAddString (p: *mut FcPattern, object: *const c_char, s: *const FcChar8) -> FcBool;
  /// C: fontconfig.h 1006: `FcPublic FcBool FcPatternAddCharSet (FcPattern *p, const char *object,
  /// const FcCharSet *c);`
  pub fn FcPatternAddCharSet (p: *mut FcPattern, object: *const c_char,
                              c: *const FcCharSet) -> FcBool;
  /// C: fontconfig.h 1012: `FcPublic FcBool FcPatternAddLangSet (FcPattern *p, const char *object,
  /// const FcLangSet *ls);`
  pub fn FcPatternAddLangSet (p: *mut FcPattern, object: *const c_char,
                              ls: *const FcLangSet) -> FcBool;
  /// C: fontconfig.h 988: `FcPublic FcBool FcPatternDel (FcPattern *p, const char *object);`
  pub fn FcPatternDel (p: *mut FcPattern, object: *const c_char) -> FcBool;
  /// C: fontconfig.h 1024: `FcPublic FcResult FcPatternGetString (const FcPattern *p, const char
  /// *object, int n, FcChar8 **s);`
  pub fn FcPatternGetString (p: *const FcPattern, object: *const c_char, n: c_int,
                             s: *mut *mut FcChar8) -> FcResult;
  /// C: fontconfig.h 1018: `FcPublic FcResult FcPatternGetInteger (const FcPattern *p, const char
  /// *object, int n, int *i);`
  pub fn FcPatternGetInteger (p: *const FcPattern, object: *const c_char, n: c_int,
                              i: *mut c_int) -> FcResult;
  /// C: fontconfig.h 1021: `FcPublic FcResult FcPatternGetDouble (const FcPattern *p, const char
  /// *object, int n, double *d);`
  pub fn FcPatternGetDouble (p: *const FcPattern, object: *const c_char, n: c_int,
                             d: *mut c_double) -> FcResult;
  /// C: fontconfig.h 1033: `FcPublic FcResult FcPatternGetBool (const FcPattern *p, const char
  /// *object, int n, FcBool *b);`
  pub fn FcPatternGetBool (p: *const FcPattern, object: *const c_char, n: c_int,
                           b: *mut FcBool) -> FcResult;
  /// C: fontconfig.h 1030: `FcPublic FcResult FcPatternGetCharSet (const FcPattern *p, const char
  /// *object, int n, FcCharSet **c);`
  pub fn FcPatternGetCharSet (p: *const FcPattern, object: *const c_char, n: c_int,
                              c: *mut *mut FcCharSet) -> FcResult;
  /// C: fontconfig.h 1036: `FcPublic FcResult FcPatternGetLangSet (const FcPattern *p, const char
  /// *object, int n, FcLangSet **ls);`
  pub fn FcPatternGetLangSet (p: *const FcPattern, object: *const c_char, n: c_int,
                              ls: *mut *mut FcLangSet) -> FcResult;
  /// C: fontconfig.h 1039: `FcPublic FcResult FcPatternGetRange (const FcPattern *p, const char
  /// *object, int id, FcRange **r);`
  pub fn FcPatternGetRange (p: *const FcPattern, object: *const c_char, id: c_int,
                            r: *mut *mut FcRange) -> FcResult;
  /// C: fontconfig.h 803: `FcPublic FcObjectSet * FcObjectSetBuild (const char *first, ...);`
  pub fn FcObjectSetBuild (first: *const c_char, ...) -> *mut FcObjectSet;
  /// C: fontconfig.h 794: `FcPublic FcBool FcObjectSetAdd (FcObjectSet *os, const char *object);`
  pub fn FcObjectSetAdd (os: *mut FcObjectSet, object: *const c_char) -> FcBool;
  /// C: fontconfig.h 797: `FcPublic void FcObjectSetDestroy (FcObjectSet *os);`
  pub fn FcObjectSetDestroy (os: *mut FcObjectSet);
  /// C: fontconfig.h 813: `FcPublic FcFontSet * FcFontList (FcConfig *config, FcPattern *p,
  /// FcObjectSet *os);`
  pub fn FcFontList (config: *mut FcConfig, p: *mut FcPattern,
                     os: *mut FcObjectSet) -> *mut FcFontSet;
  /// C: fontconfig.h 852: `FcPublic FcPattern * FcFontMatch (FcConfig *config, FcPattern *p,
  /// FcResult *result);`
  pub fn FcFontMatch (config: *mut FcConfig, p: *mut FcPattern,
                      result: *mut FcResult) -> *mut FcPattern;
  /// C: fontconfig.h 713: `FcPublic void FcFontSetDestroy (FcFontSet *s);`
  pub fn FcFontSetDestroy (s: *mut FcFontSet);
  /// C: fontconfig.h 577: `FcPublic FcCharSet * FcCharSetCreate (void);`
  pub fn FcCharSetCreate () -> *mut FcCharSet;
  /// C: fontconfig.h 584: `FcPublic void FcCharSetDestroy (FcCharSet *fcs);`
  pub fn FcCharSetDestroy (fcs: *mut FcCharSet);
  /// C: fontconfig.h 587: `FcPublic FcBool FcCharSetAddChar (FcCharSet *fcs, FcChar32 ucs4);`
  pub fn FcCharSetAddChar (fcs: *mut FcCharSet, ucs4: FcChar32) -> FcBool;
  /// C: fontconfig.h 611: `FcPublic FcBool FcCharSetHasChar (const FcCharSet *fcs, FcChar32
  /// ucs4);`
  pub fn FcCharSetHasChar (fcs: *const FcCharSet, ucs4: FcChar32) -> FcBool;
  /// C: fontconfig.h 593: `FcPublic FcCharSet * FcCharSetCopy (FcCharSet *src);`
  pub fn FcCharSetCopy (src: *mut FcCharSet) -> *mut FcCharSet;
  /// C: fontconfig.h 751: `FcPublic FcLangSet * FcLangSetCreate (void);`
  pub fn FcLangSetCreate () -> *mut FcLangSet;
  /// C: fontconfig.h 760: `FcPublic FcBool FcLangSetAdd (FcLangSet *ls, const FcChar8 *lang);`
  pub fn FcLangSetAdd (ls: *mut FcLangSet, lang: *const FcChar8) -> FcBool;
  /// C: fontconfig.h 754: `FcPublic void FcLangSetDestroy (FcLangSet *ls);`
  pub fn FcLangSetDestroy (ls: *mut FcLangSet);
  /// C: fontconfig.h 543: `FcPublic FcBool FcConfigSubstitute (FcConfig *config, FcPattern *p,
  /// FcMatchKind kind);`
  pub fn FcConfigSubstitute (config: *mut FcConfig, p: *mut FcPattern,
                             kind: FcMatchKind) -> FcBool;
  /// C: fontconfig.h 667: `FcPublic void FcDefaultSubstitute (FcPattern *pattern);`
  pub fn FcDefaultSubstitute (pattern: *mut FcPattern);
  /// C: fontconfig.h 927: `FcPublic FcBool FcNameConstant (const FcChar8 *string, int *result);`
  pub fn FcNameConstant (string: *const FcChar8, result: *mut c_int) -> FcBool;
}

/* The Fontconfig *string* macros the slice names, each declared once.  C spells them as string
   literals (`#define FC_FILE "file"`, measured values in the segment comment above); C 1227-1229
   and C 1235-1238's `#ifndef FC_HINT_STYLE` / `#ifndef FC_LCD_FILTER` are the same two macros'
   fallback definitions, so the pair stands here with the header's value and the `#ifndef`s need no
   second copy (see the comment at C 1227 inside `ftfont_fix_match`).  */
/// C: fontconfig.h 97: `#define FC_FOUNDRY              "foundry"         /* String */`
#[cfg (feature = "have_freetype")]
pub const FC_FOUNDRY: *const c_char = c"foundry".as_ptr ();
/// C: fontconfig.h 89: `#define FC_FAMILY               "family"         /* String */`
#[cfg (feature = "have_freetype")]
pub const FC_FAMILY: *const c_char = c"family".as_ptr ();
/// C: fontconfig.h 92: `#define FC_WEIGHT               "weight"         /* Int */`
#[cfg (feature = "have_freetype")]
pub const FC_WEIGHT: *const c_char = c"weight".as_ptr ();
/// C: fontconfig.h 91: `#define FC_SLANT                "slant"          /* Int */`
#[cfg (feature = "have_freetype")]
pub const FC_SLANT: *const c_char = c"slant".as_ptr ();
/// C: fontconfig.h 105: `#define FC_WIDTH                "width"          /* Int */`
#[cfg (feature = "have_freetype")]
pub const FC_WIDTH: *const c_char = c"width".as_ptr ();
/// C: fontconfig.h 95: `#define FC_PIXEL_SIZE           "pixelsize"      /* Double */`
#[cfg (feature = "have_freetype")]
pub const FC_PIXEL_SIZE: *const c_char = c"pixelsize".as_ptr ();
/// C: fontconfig.h 96: `#define FC_SPACING              "spacing"        /* Int */`
#[cfg (feature = "have_freetype")]
pub const FC_SPACING: *const c_char = c"spacing".as_ptr ();
/// C: fontconfig.h 111: `#define FC_SCALABLE             "scalable"       /* Bool */`
#[cfg (feature = "have_freetype")]
pub const FC_SCALABLE: *const c_char = c"scalable".as_ptr ();
/// C: fontconfig.h 116: `#define FC_DPI                  "dpi"            /* double */`
#[cfg (feature = "have_freetype")]
pub const FC_DPI: *const c_char = c"dpi".as_ptr ();
/// C: fontconfig.h 98: `#define FC_ANTIALIAS            "antialias"      /* Bool (depends) */`
#[cfg (feature = "have_freetype")]
pub const FC_ANTIALIAS: *const c_char = c"antialias".as_ptr ();
/// C: fontconfig.h 99: `#define FC_HINTING              "hinting"        /* Bool (true) */`
#[cfg (feature = "have_freetype")]
pub const FC_HINTING: *const c_char = c"hinting".as_ptr ();
/// C: fontconfig.h 100: `#define FC_HINT_STYLE           "hintstyle"      /* Int */` - and C
/// 1227-1229 re-defines it when the header is older than the name.  This is one of the **two**
/// macros ftfont.c *defines itself*, so `extract-abi.py`'s constant rule compares its bytes with
/// C's literal and the value must be a literal the gate can read: `&CStr` (the `src/emacs.rs` 359
/// spelling - NUL-terminated, so the six call sites take `.as_ptr ()`).  The other FC_* names are
/// only *referenced* here, and they keep the `*const c_char` shape.
#[cfg (feature = "have_freetype")]
pub const FC_HINT_STYLE: &CStr = c"hintstyle";
/// C: fontconfig.h 102: `#define FC_AUTOHINT             "autohint"       /* Bool (false) */`
#[cfg (feature = "have_freetype")]
pub const FC_AUTOHINT: *const c_char = c"autohint".as_ptr ();
/// C: fontconfig.h 117: `#define FC_RGBA                 "rgba"           /* Int */`
#[cfg (feature = "have_freetype")]
pub const FC_RGBA: *const c_char = c"rgba".as_ptr ();
/// C: fontconfig.h 132: `#define FC_LCD_FILTER           "lcdfilter"      /* Int */` - the second
/// of the two macros C 1235-1238 re-defines ("Older fontconfig versions don't have
/// FC_LCD_FILTER").  `&CStr` for the reason `FC_HINT_STYLE` states.
#[cfg (feature = "have_freetype")]
pub const FC_LCD_FILTER: &CStr = c"lcdfilter";
/// C: fontconfig.h 90: `#define FC_STYLE                "style"          /* String */`
#[cfg (feature = "have_freetype")]
pub const FC_STYLE: *const c_char = c"style".as_ptr ();
/// C: fontconfig.h 106: `#define FC_FILE                 "file"           /* String */`
#[cfg (feature = "have_freetype")]
pub const FC_FILE: *const c_char = c"file".as_ptr ();
/// C: fontconfig.h 107: `#define FC_INDEX                "index"          /* Int */`
#[cfg (feature = "have_freetype")]
pub const FC_INDEX: *const c_char = c"index".as_ptr ();
/// C: fontconfig.h 121: `#define FC_LANG                 "lang"           /* LangSet Set of RFC 3066 langs */`
#[cfg (feature = "have_freetype")]
pub const FC_LANG: *const c_char = c"lang".as_ptr ();
/// C: fontconfig.h 120: `#define FC_CHARSET              "charset"        /* CharSet */`
#[cfg (feature = "have_freetype")]
pub const FC_CHARSET: *const c_char = c"charset".as_ptr ();
/// C: fontconfig.h 127: `#define FC_CAPABILITY           "capability"     /* String */`
#[cfg (feature = "have_freetype")]
pub const FC_CAPABILITY: *const c_char = c"capability".as_ptr ();
/// C: fontconfig.h 129: `#define FC_EMBOLDEN             "embolden"       /* Bool - true if emboldening needed*/`
#[cfg (feature = "have_freetype")]
pub const FC_EMBOLDEN: *const c_char = c"embolden".as_ptr ();
/// C: fontconfig.h 128: `#define FC_FONTFORMAT           "fontformat"     /* String */`
#[cfg (feature = "have_freetype")]
pub const FC_FONTFORMAT: *const c_char = c"fontformat".as_ptr ();
/// C: fontconfig.h 113: `#define FC_VARIABLE             "variable"       /* Bool */`
#[cfg (feature = "have_freetype")]
pub const FC_VARIABLE: *const c_char = c"variable".as_ptr ();
/// C: fontconfig.h 112: `#define FC_COLOR                "color"          /* Bool */`
#[cfg (feature = "have_freetype")]
pub const FC_COLOR: *const c_char = c"color".as_ptr ();

/* C 26-28: `These two blocks are here because this file is built when using XFT and when using
   Cairo, so struct font_info in ftfont.h needs access to the appropriate types.`
   C 29-32 (`#ifdef HAVE_XFT` -> `<X11/Xlib.h>`, `<X11/Xft/Xft.h>`) and C 33-35
   (`#ifdef USE_CAIRO` -> `<cairo-ft.h>`) are the two blocks.  Neither header is on this machine
   (there is no `Xft.h` under `/opt/homebrew/include/X11`), and **no name either one defines is
   used by C 1-1352's body**: the only Xft names the file has are the version macros of C 660-663
   (spelled as the derived feature `xft_version_lt_2_3_6`), and `<cairo-ft.h>`'s types reach
   `struct font_info`, which is `femacs/src/include/ftfont.rs`'s item (its `USE_CAIRO` arm).  The
   two blocks are therefore carried as this note; the X11 surface itself has the dedicated mirror
   `femacs/src/include/xlib.rs` (P-90 ①) and the Xft one is Owed 4.  */

/* C 37: `#include <c-strcase.h>` - gnulib's header, whose frozen mirror carries the two entry
   points this slice calls (`c_strncasecmp`), the same include `src/xfns.rs` writes.  */
#[cfg (feature = "have_freetype")]
include!("../lib/include/c-strcase.rs");       // C 37: `#include <c-strcase.h>`

/* C 39-47: the file's own Emacs headers, in C's order, each under the file gate (P-86 ⑧: the
   include graph sits inside the gate, which is why the frozen configuration compiles nothing
   here).  `include/ftfont.rs` is the `struct font_info` record's home; `include/font.rs` carries
   the `font_driver`/`font` records, the `FONT_*_INDEX` property indices and the `FONT_SET_STYLE`
   accessor this slice uses; `lisp.rs` carries `globals.rs` (the `Q*` accessors and the subr
   declarations) and the two `string.in`-family mirrors.  */
#[cfg (feature = "have_freetype")]
include!("include/lisp.rs");                   // C 39: `#include "lisp.h"`
#[cfg (feature = "have_freetype")]
include!("include/dispextern.rs");             // C 40: `#include "dispextern.h"` (`xstrcasecmp`)
#[cfg (feature = "have_freetype")]
include!("include/character.rs");              // C 41: `#include "character.h"`
#[cfg (feature = "have_freetype")]
include!("include/charset.rs");                // C 42: `#include "charset.h"`
#[cfg (feature = "have_freetype")]
include!("include/category.rs");               // C 43: `#include "category.h"`
#[cfg (feature = "have_freetype")]
include!("include/composite.rs");              // C 44: `#include "composite.h"`
#[cfg (feature = "have_freetype")]
include!("include/font.rs");                   // C 45: `#include "font.h"`
#[cfg (feature = "have_freetype")]
include!("include/ftfont.rs");                 // C 46: `#include "ftfont.h"`
#[cfg (feature = "have_freetype")]
include!("include/pdumper.rs");                // C 47: `#include "pdumper.h"`

/* Five declarations this slice calls that the mirrors it includes do not carry yet (P-72 rule 4 /
   P-88 ①: a mirror must reach what it needs through its own `include!`s; the four Emacs ones are
   `font.h` declarations whose mirror `include/font.rs` has no function-declaration section at all,
   so their consumers declare them - the `src/androidfont.rs` 99-120 shape).  Each line names the
   reference header it comes from, and the ledger's Owed 3 names the mirror it belongs in.  */
#[cfg (feature = "have_freetype")]
unsafe extern "C" {
  /// C: font.h 842: `extern Lisp_Object font_make_entity (void);` (font.c defines it).
  pub fn font_make_entity () -> Lisp_Object;
  /// C: font.h 885-886: `extern Lisp_Object font_intern_prop (const char *str, ptrdiff_t len,
  /// bool force_symbol);` (font.c defines it).
  pub fn font_intern_prop (str_: *const c_char, len: ptrdiff_t,
                           force_symbol: bool) -> Lisp_Object;
  /// C: font.h 913-914: `extern Lisp_Object font_put_extra (Lisp_Object font, Lisp_Object prop,
  /// Lisp_Object val);` (font.c defines it).
  pub fn font_put_extra (font: Lisp_Object, prop: Lisp_Object, val: Lisp_Object) -> Lisp_Object;
  /// C: font.h 1001: `extern void font_add_log (const char *, Lisp_Object, Lisp_Object);`
  /// (font.c defines it; `include/font.rs`'s `FONT_ADD_LOG!` body - C font.h 1004-1008 - expands
  /// to this name, which that mirror does not declare).
  pub fn font_add_log (action: *const c_char, arg: Lisp_Object, result: Lisp_Object);
  /// C: font.h 854-855: `extern int font_style_to_value (enum font_property_index prop,
  /// Lisp_Object name, bool noerror);` (font.c defines it; the mirror's `FONT_SET_STYLE!` body
  /// expands to this name, and C 183 calls it directly).
  pub fn font_style_to_value (prop: font_property_index, name: Lisp_Object,
                              noerror: bool) -> c_int;
  /// C: _string.h 110: `char *strstr(const char *__big, const char *__little);` - the frozen
  /// `<string.h>` mirror (`femacs/lib/include/string.rs`) carries `strlen`/`strcmp`/`strcpy` and
  /// not this one; the gnulib template's `strstr` is a `#define strstr rpl_strstr` that the frozen
  /// target does not take (`femacs/lib/include/string.in.rs` 248-255).
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

/* C 49-52: `static struct font_driver const ftfont_driver;` and, under `#ifdef HAVE_HARFBUZZ`,
   `static struct font_driver fthbfont_driver;`.  These are C *tentative definitions* of two
   file-scope objects whose real definitions (with the driver record's initialiser) are at C 3098
   (`fthbfont_driver`, inside the file's `HAVE_HARFBUZZ` block) and C 3102 (`ftfont_driver`);
   Rust has no tentative definition, so the two lines are recorded here and the objects land with
   their C initialisers (PORTING.md 0.1-12's object half, the P-91 ④ bookkeeping class).  Nothing
   in C 1-1352 reads either object.  */

/* C 54-64: the file's four state objects and the FreeType library handle.  C's definitions have no
   initialiser, so each is C's zero value: `bool` false, a null `FT_Library`, and `Qnil` (which is
   the null `Lisp_Object`, `lisp_h_Qnil`).  */
/* Flag to tell if FcInit is already called or not.  */
/// C: 55: `static bool fc_initialized;`
#[cfg (feature = "have_freetype")]
static mut fc_initialized: bool = false;

/* Handle to a FreeType library instance.  */
/// C: 58: `static FT_Library ft_library;`
#[cfg (feature = "have_freetype")]
static mut ft_library: FT_Library = std::ptr::null_mut ();

/* Cache for FreeType fonts.  */
/// C: 61: `static Lisp_Object freetype_font_cache;`
#[cfg (feature = "have_freetype")]
static mut freetype_font_cache: Lisp_Object = lisp_h_Qnil ();

/* Cache for FT_Face and FcCharSet. */
/// C: 64: `static Lisp_Object ft_face_cache;`
#[cfg (feature = "have_freetype")]
static mut ft_face_cache: Lisp_Object = lisp_h_Qnil ();

/* C 66-71: `enum ftfont_cache_for { FTFONT_CACHE_FOR_FACE, FTFONT_CACHE_FOR_CHARSET,
   FTFONT_CACHE_FOR_ENTITY };`  C's enum ABI is `c_int` (PORTING.md 1.3), so the tag keeps its name
   as the port's alias and the three members are its constants (the `include/gtk.rs` 373-378
   shape).  */
/// C: 66-71: the tag `enum ftfont_cache_for`.
#[cfg (feature = "have_freetype")]
pub type ftfont_cache_for = c_int;
/// C: 68: `FTFONT_CACHE_FOR_FACE,`
#[cfg (feature = "have_freetype")]
pub const FTFONT_CACHE_FOR_FACE: ftfont_cache_for = 0;
/// C: 69: `FTFONT_CACHE_FOR_CHARSET,`
#[cfg (feature = "have_freetype")]
pub const FTFONT_CACHE_FOR_CHARSET: ftfont_cache_for = 1;
/// C: 70: `FTFONT_CACHE_FOR_ENTITY`
#[cfg (feature = "have_freetype")]
pub const FTFONT_CACHE_FOR_ENTITY: ftfont_cache_for = 2;

/* C 73-74: `static Lisp_Object ftfont_lookup_cache (Lisp_Object, enum ftfont_cache_for);`.  A
   static function's forward declaration has no Rust counterpart (the item itself is at C 373),
   so the two lines are recorded rather than transcribed (PORTING.md 0.1-12, P-91 ②).  */

/* C 76: `#define SYMBOL_FcChar8(SYM) (FcChar8 *) SDATA (SYMBOL_NAME (SYM))` - the cast is
   `as *mut FcChar8` and the parameter keeps C's role.  */
/// C: 76: the macro, whose body C 337/752/760/782/789/949/1010 call.
#[cfg (feature = "have_freetype")]
macro_rules! SYMBOL_FcChar8 {
  ($sym:expr) => { SDATA (SYMBOL_NAME ($sym)) as *mut FcChar8 };
}

/* C 78-127: the file-scope `fc_charset_table` - an array of a C *anonymous* struct, which
   PORTING.md 1.3 requires the port to name; the name follows the object (`fc_charset_table_entry`,
   the `charset_map_entry`/`table_entry` shape).  C's aggregate initialisation leaves the unused
   `uniquifier` slots and the `lang` pointers zero, which the port spells out.  The last element is
   C 126's `{ NULL }` sentinel: C 521/527-528/637 walk the array until `name` is null.  */
/// C: 78-88: the anonymous record's members (`name`, `uniquifier[6]`, `lang`, `fc_charset`).
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct fc_charset_table_entry
{
  /* registry name */
  pub name: *const c_char,
  /* characters to distinguish the charset from the others */
  pub uniquifier: [c_int; 6],
  /* additional constraint by language */
  pub lang: *const c_char,
  /* set on demand */
  pub fc_charset: *mut FcCharSet,
}

/// C: 88: `} fc_charset_table[] =` - 37 records plus C 126's sentinel.
#[cfg (feature = "have_freetype")]
static mut fc_charset_table: [fc_charset_table_entry; 38] =
[
  fc_charset_table_entry { name: c"iso8859-1".as_ptr (), uniquifier: [0x00A0, 0x00A1, 0x00B4, 0x00BC, 0x00D0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-2".as_ptr (), uniquifier: [0x00A0, 0x010E, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-3".as_ptr (), uniquifier: [0x00A0, 0x0108, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-4".as_ptr (), uniquifier: [0x00A0, 0x00AF, 0x0128, 0x0156, 0x02C7, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-5".as_ptr (), uniquifier: [0x00A0, 0x0401, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-6".as_ptr (), uniquifier: [0x00A0, 0x060C, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-7".as_ptr (), uniquifier: [0x00A0, 0x0384, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-8".as_ptr (), uniquifier: [0x00A0, 0x05D0, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-9".as_ptr (), uniquifier: [0x00A0, 0x00A1, 0x00BC, 0x011E, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-10".as_ptr (), uniquifier: [0x00A0, 0x00D0, 0x0128, 0x2015, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-11".as_ptr (), uniquifier: [0x00A0, 0x0E01, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-13".as_ptr (), uniquifier: [0x00A0, 0x201C, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-14".as_ptr (), uniquifier: [0x00A0, 0x0174, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-15".as_ptr (), uniquifier: [0x00A0, 0x00A1, 0x00D0, 0x0152, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"iso8859-16".as_ptr (), uniquifier: [0x00A0, 0x0218, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"gb2312.1980-0".as_ptr (), uniquifier: [0x4E13, 0, 0, 0, 0, 0], lang: c"zh-cn".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"big5-0".as_ptr (), uniquifier: [0x9C21, 0, 0, 0, 0, 0], lang: c"zh-tw".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"jisx0208.1983-0".as_ptr (), uniquifier: [0x4E55, 0, 0, 0, 0, 0], lang: c"ja".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"ksc5601.1985-0".as_ptr (), uniquifier: [0xAC00, 0, 0, 0, 0, 0], lang: c"ko".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"cns11643.1992-1".as_ptr (), uniquifier: [0xFE32, 0, 0, 0, 0, 0], lang: c"zh-tw".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"cns11643.1992-2".as_ptr (), uniquifier: [0x4E33, 0x7934, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"cns11643.1992-3".as_ptr (), uniquifier: [0x201A9, 0, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"cns11643.1992-4".as_ptr (), uniquifier: [0x20057, 0, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"cns11643.1992-5".as_ptr (), uniquifier: [0x20000, 0, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"cns11643.1992-6".as_ptr (), uniquifier: [0x20003, 0, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"cns11643.1992-7".as_ptr (), uniquifier: [0x20055, 0, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"gbk-0".as_ptr (), uniquifier: [0x4E06, 0, 0, 0, 0, 0], lang: c"zh-cn".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"jisx0212.1990-0".as_ptr (), uniquifier: [0x4E44, 0, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"jisx0213.2000-1".as_ptr (), uniquifier: [0xFA10, 0, 0, 0, 0, 0], lang: c"ja".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"jisx0213.2000-2".as_ptr (), uniquifier: [0xFA49, 0, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"jisx0213.2004-1".as_ptr (), uniquifier: [0x20B9F, 0, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"viscii1.1-1".as_ptr (), uniquifier: [0x1EA0, 0x1EAE, 0x1ED2, 0, 0, 0], lang: c"vi".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"tis620.2529-1".as_ptr (), uniquifier: [0x0E01, 0, 0, 0, 0, 0], lang: c"th".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"microsoft-cp1251".as_ptr (), uniquifier: [0x0401, 0x0490, 0, 0, 0, 0], lang: c"ru".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"koi8-r".as_ptr (), uniquifier: [0x0401, 0x2219, 0, 0, 0, 0], lang: c"ru".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"mulelao-1".as_ptr (), uniquifier: [0x0E81, 0, 0, 0, 0, 0], lang: c"lo".as_ptr (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: c"unicode-sip".as_ptr (), uniquifier: [0x20000, 0, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
  fc_charset_table_entry { name: std::ptr::null (), uniquifier: [0, 0, 0, 0, 0, 0], lang: std::ptr::null (), fc_charset: std::ptr::null_mut () },
];

/* C 129-133: `matching_prefix` - the `static` helper C 166-169 calls four times.  C's `strlen`
   returns `size_t`, which the comparison against the `ptrdiff_t` parameter converts; the port
   writes that conversion.  */
/// C: 130-132: `matching_prefix (char const *str, ptrdiff_t len, char const *pat)` and its body.
#[cfg (feature = "have_freetype")]
unsafe fn matching_prefix (str_: *const c_char, len: ptrdiff_t, pat: *const c_char) -> bool
{
  len == strlen (pat) as ptrdiff_t && c_strncasecmp (str_, pat, len as usize) == 0
}

/* C 135-146: the "Dirty hack for handing ADSTYLE property" comment, which C 156-161 comment
   continues ("Not a BDF nor PCF font.").  C 147 is blank.  */
/* Dirty hack for handing ADSTYLE property.

   Fontconfig (actually the underlying FreeType) gives such ADSTYLE
   font property of PCF/BDF fonts in FC_STYLE.  And, "Bold",
   "Oblique", "Italic", or any non-normal SWIDTH property names
   (e.g. SemiCondensed) are appended.  In addition, if there's no
   ADSTYLE property nor non-normal WEIGHT/SLANT/SWIDTH properties,
   "Regular" is used for FC_STYLE (see the function
   pcf_interpret_style in src/pcf/pcfread.c of FreeType).

   Unfortunately this behavior is not documented, so the following
   code may fail if FreeType changes the behavior in the future.  */

/* C 148-186: `get_adstyle_property`.  C's locals are declared without an initialiser and the
   `#ifdef FC_FONTFORMAT` arm's `fcstr` is the one out-parameter; the port initialises each local
   to its zero value and notes the two `USE_SAFE_ALLOCA`/`SAFE_FREE` macro call sites, which the
   port spells with the two macro locals (PORTING.md 0.1-13; the `SAFE_ALLOCA` family's own
   spelling in `femacs/src/include/lisp.rs` 2986-3068).  */
/// C: 148-150: `static Lisp_Object` / `get_adstyle_property (FcPattern *p)`.
#[cfg (feature = "have_freetype")]
unsafe fn get_adstyle_property (p: *mut FcPattern) -> Lisp_Object
{
  let mut fcstr: *mut FcChar8 = std::ptr::null_mut ();
  let mut str_: *mut c_char;
  let mut end: *mut c_char;
  let mut tmp: *mut c_char;

  /* C 155-161: `#ifdef FC_FONTFORMAT` / the guard against non-BDF/PCF faces / `#endif`.  */
  #[cfg (feature = "fc_fontformat")]
  {
    if (FcPatternGetString (p, FC_FONTFORMAT, 0, &raw mut fcstr) == FcResultMatch)
        && dispextern_externs::xstrcasecmp (fcstr as *const c_char, c"bdf".as_ptr ()) != 0
        && dispextern_externs::xstrcasecmp (fcstr as *const c_char, c"pcf".as_ptr ()) != 0
      /* Not a BDF nor PCF font.  */
      { return Qnil (); }
  }
  if FcPatternGetString (p, FC_STYLE, 0, &raw mut fcstr) != FcResultMatch
    { return Qnil (); }
  str_ = fcstr as *mut c_char;
  /* C 165: `for (end = str; *end && *end != ' '; end++);` - the empty body's loop.  */
  end = str_;
  while *end != 0 && *end != b' ' as c_char
    { end = end.add (1); }
  if matching_prefix (str_, end.offset_from (str_), c"Regular".as_ptr ())
      || matching_prefix (str_, end.offset_from (str_), c"Bold".as_ptr ())
      || matching_prefix (str_, end.offset_from (str_), c"Oblique".as_ptr ())
      || matching_prefix (str_, end.offset_from (str_), c"Italic".as_ptr ())
    { return Qnil (); }
  /* C 171-172: the XLFD substitution comment (bug#70989).  */
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  tmp = SAFE_ALLOCA! (sa_avail, end.offset_from (str_)) as *mut c_char;
  /* C 175-180: `for (ptrdiff_t i = 0; i < end - str; i++)` with C's four-way character test.  */
  {
    let mut i: ptrdiff_t = 0;
    while i < end.offset_from (str_)
      {
        *tmp.add (i as usize) = if *str_.add (i as usize) != b'?' as c_char
                                   && *str_.add (i as usize) != b'*' as c_char
                                   && *str_.add (i as usize) != b'"' as c_char
                                   && *str_.add (i as usize) != b'-' as c_char
                                  { *str_.add (i as usize) } else { b' ' as c_char };
        i += 1;
      }
  }
  let adstyle: Lisp_Object = font_intern_prop (tmp, end.offset_from (str_), true);
  SAFE_FREE! (sa_count);
  if font_style_to_value (FONT_WIDTH_INDEX, adstyle, false) >= 0
    { return Qnil (); }
  adstyle
}

/* C 188-309: `ftfont_pattern_entity` - the Fontconfig pattern -> font-entity converter.  */
/// C: 188-189: `static Lisp_Object` / `ftfont_pattern_entity (FcPattern *p, Lisp_Object extra)`.
#[cfg (feature = "have_freetype")]
unsafe fn ftfont_pattern_entity (p: *mut FcPattern, extra: Lisp_Object) -> Lisp_Object
{
  let mut str_: *mut FcChar8 = std::ptr::null_mut ();
  let file: *mut c_char;
  let mut idx: c_int = 0;
  let mut numeric: c_int = 0;
  let mut dbl: c_double = 0.0;
  let mut b: FcBool = FcFalse;

  if FcPatternGetString (p, FC_FILE, 0, &raw mut str_) != FcResultMatch
    { return Qnil (); }
  if FcPatternGetInteger (p, FC_INDEX, 0, &raw mut idx) != FcResultMatch
    { return Qnil (); }
  /* C 203-220: `#ifdef FC_VARIABLE` / the virtual-meta-pattern skip / `#endif  /* FC_VARIABLE */`. */
  #[cfg (feature = "fc_variable")]
  {
    let mut range: *mut FcRange = std::ptr::null_mut ();
    if FcPatternGetRange (p, FC_WEIGHT, 0, &raw mut range) == FcResultMatch
        && FcPatternGetBool (p, FC_VARIABLE, 0, &raw mut b) == FcResultMatch
        && b == FcTrue
      { return Qnil (); }
  }

  file = str_ as *mut c_char;
  let key: Lisp_Object = Fcons (build_unibyte_string (file), make_fixnum (idx as EMACS_INT));
  let cache: Lisp_Object = ftfont_lookup_cache (key, FTFONT_CACHE_FOR_ENTITY);
  let mut entity: Lisp_Object = XCAR (cache);
  if ! NILP (entity)
    {
      let val: Lisp_Object = font_make_entity ();
      let mut i: c_int = 0;

      while i < FONT_OBJLIST_INDEX
        { ASET (val, i as isize, AREF (entity, i as isize)); i += 1; }

      ASET (val, FONT_EXTRA_INDEX as isize, Fcopy_sequence (extra));
      font_put_extra (val, QCfont_entity (), key);

      return val;
    }
  entity = font_make_entity ();
  XSETCAR (cache, entity);

  ASET (entity, FONT_TYPE_INDEX as isize, Qfreetype ());
  ASET (entity, FONT_REGISTRY_INDEX as isize, Qiso10646_1 ());

  if FcPatternGetString (p, FC_FOUNDRY, 0, &raw mut str_) == FcResultMatch
    {
      let s: *mut c_char = str_ as *mut c_char;
      ASET (entity, FONT_FOUNDRY_INDEX as isize,
            font_intern_prop (s, strlen (s) as ptrdiff_t, true));
    }
  if FcPatternGetString (p, FC_FAMILY, 0, &raw mut str_) == FcResultMatch
    {
      let s: *mut c_char = str_ as *mut c_char;
      ASET (entity, FONT_FAMILY_INDEX as isize,
            font_intern_prop (s, strlen (s) as ptrdiff_t, true));
    }
  if FcPatternGetInteger (p, FC_WEIGHT, 0, &raw mut numeric) == FcResultMatch
    { FONT_SET_STYLE! (entity, FONT_WEIGHT_INDEX, make_fixnum (numeric as EMACS_INT)); }
  if FcPatternGetInteger (p, FC_SLANT, 0, &raw mut numeric) == FcResultMatch
    {
      numeric += 100;
      FONT_SET_STYLE! (entity, FONT_SLANT_INDEX, make_fixnum (numeric as EMACS_INT));
    }
  if FcPatternGetInteger (p, FC_WIDTH, 0, &raw mut numeric) == FcResultMatch
    { FONT_SET_STYLE! (entity, FONT_WIDTH_INDEX, make_fixnum (numeric as EMACS_INT)); }
  if FcPatternGetDouble (p, FC_PIXEL_SIZE, 0, &raw mut dbl) == FcResultMatch
    { ASET (entity, FONT_SIZE_INDEX as isize, make_fixnum (dbl as EMACS_INT)); }
  else
    { ASET (entity, FONT_SIZE_INDEX as isize, make_fixnum (0)); }
  if FcPatternGetInteger (p, FC_SPACING, 0, &raw mut numeric) == FcResultMatch
    { ASET (entity, FONT_SPACING_INDEX as isize, make_fixnum (numeric as EMACS_INT)); }
  if FcPatternGetDouble (p, FC_DPI, 0, &raw mut dbl) == FcResultMatch
    {
      let dpi: c_int = dbl as c_int;
      ASET (entity, FONT_DPI_INDEX as isize, make_fixnum (dpi as EMACS_INT));
    }
  if FcPatternGetBool (p, FC_SCALABLE, 0, &raw mut b) == FcResultMatch
      && b == FcTrue
    {
      ASET (entity, FONT_SIZE_INDEX as isize, make_fixnum (0));
      ASET (entity, FONT_AVGWIDTH_INDEX as isize, make_fixnum (0));
    }
  else
    {
      /* As this font is not scalable, perhaps this is a BDF or PCF
	 font. */
      let mut ft_face: FT_Face = std::ptr::null_mut ();

      ASET (entity, FONT_ADSTYLE_INDEX as isize, get_adstyle_property (p));
      if (! ft_library.is_null ()
          || FT_Init_FreeType (&raw mut ft_library) == 0)
          && FT_New_Face (ft_library, file, idx as c_long, &raw mut ft_face) == 0
        {
          let mut rec: BDF_PropertyRec = unsafe { std::mem::zeroed () };

          if FT_Get_BDF_Property (ft_face, c"AVERAGE_WIDTH".as_ptr (), &raw mut rec) == 0
              && rec.r#type == BDF_PROPERTY_TYPE_INTEGER
            { ASET (entity, FONT_AVGWIDTH_INDEX as isize,
                    make_fixnum (rec.u.integer as EMACS_INT)); }
          FT_Done_Face (ft_face);
        }
    }

  ASET (entity, FONT_EXTRA_INDEX as isize, Fcopy_sequence (extra));
  font_put_extra (entity, QCfont_entity (), key);
  entity
}

/* C 312: `static Lisp_Object ftfont_generic_family_list;` - the generic-family cache C 327 and
   C 1140 read, `syms_of_ftfont`'s `DEFVAR_LISP` owns (outside this slice).  */
/// C: 312: the object C defines without an initialiser (`Qnil`).
#[cfg (feature = "have_freetype")]
static mut ftfont_generic_family_list: Lisp_Object = lisp_h_Qnil ();

/* C 314-364: `ftfont_resolve_generic_family`.  C 332-334's `if (! pattern) goto err;` jumps
   forward to C 361's `err:` label and the cleanup; the port writes the destination as a labelled
   block whose `break` is the jump (PORTING.md 2.2, the `androidterm.rs` 491 / `androidmenu.rs`
   616-651 shape), so the statements after the label run on both paths.  */
/// C: 314-315: `static Lisp_Object` / `ftfont_resolve_generic_family (Lisp_Object family,
/// FcPattern *pattern)`.
#[cfg (feature = "have_freetype")]
unsafe fn ftfont_resolve_generic_family (family: Lisp_Object, pattern: *mut FcPattern) -> Lisp_Object
{
  let mut family: Lisp_Object = family;
  let mut pattern: *mut FcPattern = pattern;
  let slot: Lisp_Object;
  let mut match_: *mut FcPattern = std::ptr::null_mut ();
  let mut result: FcResult = FcResultMatch;
  let mut langset: *mut FcLangSet = std::ptr::null_mut ();

  family = Fintern (Fdowncase (SYMBOL_NAME (family)), Qnil ());
  if EQ (family, Qmono ())
    { family = Qmonospace (); }
  else if EQ (family, Qsans ()) || EQ (family, Qsans__serif ())
    { family = Qsans_serif (); }
  slot = assq_no_quit (family, ftfont_generic_family_list);
  if ! CONSP (slot)
    { return Qnil (); }
  if ! EQ (XCDR (slot), Qt ())
    { return XCDR (slot); }

  'err: {
    pattern = FcPatternDuplicate (pattern);
    if pattern.is_null ()
      { break 'err; }
    FcPatternDel (pattern, FC_FOUNDRY);
    FcPatternDel (pattern, FC_FAMILY);
    FcPatternAddString (pattern, FC_FAMILY, SYMBOL_FcChar8! (family));
    if FcPatternGetLangSet (pattern, FC_LANG, 0, &raw mut langset) != FcResultMatch
      {
        /* This is to avoid the effect of locale.  */
        let lang: *const FcChar8 = c"en".as_ptr () as *const FcChar8;
        langset = FcLangSetCreate ();
        FcLangSetAdd (langset, lang);
        FcPatternAddLangSet (pattern, FC_LANG, langset);
        FcLangSetDestroy (langset);
      }
    FcConfigSubstitute (std::ptr::null_mut (), pattern, FcMatchPattern);
    FcDefaultSubstitute (pattern);
    match_ = FcFontMatch (std::ptr::null_mut (), pattern, &raw mut result);
    if ! match_.is_null ()
      {
        let mut fam: *mut FcChar8 = std::ptr::null_mut ();

        if FcPatternGetString (match_, FC_FAMILY, 0, &raw mut fam) == FcResultMatch
          { family = intern (fam as *const c_char); }
      }
    else
      { family = Qnil (); }
    XSETCDR (slot, family);
    if ! match_.is_null ()
      { FcPatternDestroy (match_); }
  }
  /* C 361-362: `err:` and its cleanup.  */
  if ! pattern.is_null ()
    { FcPatternDestroy (pattern); }
  family
}

/* C 366-371: `struct ftfont_cache_data` - the C 394 `xzalloc`ed record behind the mint pointer.
   `intptr_t` is `lisp.rs` 1072's alias (`isize`), which is how `src/alloc.rs` 361 spells C's
   `intptr_t` too.  */
/// C: 366-371: `struct ftfont_cache_data { FT_Face ft_face; FcCharSet *fc_charset;
/// intptr_t face_refcount; };`
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct ftfont_cache_data
{
  pub ft_face: FT_Face,
  pub fc_charset: *mut FcCharSet,
  pub face_refcount: intptr_t,
}

/* C 373-458: `ftfont_lookup_cache`.  C 433/436/439's three `goto finish` jumps all land on C 448
   (the Fontconfig cleanup), so the destination is a labelled block and the cleanup follows it.  */
/// C: 373-374: `static Lisp_Object` / `ftfont_lookup_cache (Lisp_Object key, enum
/// ftfont_cache_for cache_for)`.
#[cfg (feature = "have_freetype")]
unsafe fn ftfont_lookup_cache (key: Lisp_Object, cache_for: ftfont_cache_for) -> Lisp_Object
{
  let mut key: Lisp_Object = key;
  let mut cache: Lisp_Object = Qnil ();
  let mut val: Lisp_Object = Qnil ();
  let mut cache_data: *mut ftfont_cache_data = std::ptr::null_mut ();

  if FONT_ENTITY_P (key)
    {
      val = assq_no_quit (QCfont_entity (), AREF (key, FONT_EXTRA_INDEX as isize));
      eassert! (CONSP (val));
      key = XCDR (val);
    }

  if NILP (ft_face_cache)
    { cache = Qnil (); }
  else
    { cache = Fgethash (key, ft_face_cache, Qnil ()); }
  if NILP (cache)
    {
      if NILP (ft_face_cache)
        { ft_face_cache = CALLN! (Fmake_hash_table, QCtest (), Qequal ()); }
      cache_data = xzalloc (std::mem::size_of::<ftfont_cache_data> ())
                     as *mut ftfont_cache_data;
      val = make_mint_ptr (cache_data as *mut c_void);
      cache = Fcons (Qnil (), val);
      Fputhash (key, cache, ft_face_cache);
    }
  else
    {
      val = XCDR (cache);
      cache_data = xmint_pointer (val) as *mut ftfont_cache_data;
    }

  if cache_for == FTFONT_CACHE_FOR_ENTITY
    { return cache; }

  /* C 408-409: C's conditional expression picks which half of `cache_data` the test reads.  */
  if if cache_for == FTFONT_CACHE_FOR_FACE
       { (*cache_data).ft_face.is_null () } else { (*cache_data).fc_charset.is_null () }
    {
      let filename: *mut c_char = SSDATA (XCAR (key));
      let idx: c_int = XFIXNUM (XCDR (key)) as c_int;

      if cache_for == FTFONT_CACHE_FOR_FACE
        {
          if ft_library.is_null ()
              && FT_Init_FreeType (&raw mut ft_library) != 0
            { return Qnil (); }
          if FT_New_Face (ft_library, filename, idx as c_long,
                          &raw mut (*cache_data).ft_face) != 0
            { return Qnil (); }
        }
      else
        {
          let mut pat: *mut FcPattern = std::ptr::null_mut ();
          let mut fontset: *mut FcFontSet = std::ptr::null_mut ();
          let mut objset: *mut FcObjectSet = std::ptr::null_mut ();
          let mut charset: *mut FcCharSet = std::ptr::null_mut ();

          'finish: {
            pat = FcPatternBuild (std::ptr::null_mut (), FC_FILE, FcTypeString,
                                  filename as *mut FcChar8,
                                  FC_INDEX, FcTypeInteger, idx,
                                  std::ptr::null_mut::<c_void> ());
            if pat.is_null ()
              { break 'finish; }
            objset = FcObjectSetBuild (FC_CHARSET, FC_STYLE,
                                       std::ptr::null::<c_char> ());
            if objset.is_null ()
              { break 'finish; }
            fontset = FcFontList (std::ptr::null_mut (), pat, objset);
            if fontset.is_null ()
              { break 'finish; }
            if ! fontset.is_null () && (*fontset).nfont > 0
                && (FcPatternGetCharSet (*(*fontset).fonts.add (0), FC_CHARSET, 0,
                                         &raw mut charset)
                    == FcResultMatch)
              { (*cache_data).fc_charset = FcCharSetCopy (charset); }
            else
              { (*cache_data).fc_charset = FcCharSetCreate (); }
          }
          /* C 448-454: `finish:` and the three cleanups.  */
          if ! fontset.is_null ()
            { FcFontSetDestroy (fontset); }
          if ! objset.is_null ()
            { FcObjectSetDestroy (objset); }
          if ! pat.is_null ()
            { FcPatternDestroy (pat); }
        }
    }
  cache
}

/* C 460-470: `ftfont_get_fc_charset`.  */
/// C: 460-461: `static FcCharSet *` / `ftfont_get_fc_charset (Lisp_Object entity)`.
#[cfg (feature = "have_freetype")]
unsafe fn ftfont_get_fc_charset (entity: Lisp_Object) -> *mut FcCharSet
{
  let cache: Lisp_Object = ftfont_lookup_cache (entity, FTFONT_CACHE_FOR_CHARSET);
  let val: Lisp_Object = XCDR (cache);
  let cache_data: *mut ftfont_cache_data = xmint_pointer (val) as *mut ftfont_cache_data;
  (*cache_data).fc_charset
}

/* ----------------------------------------------------------------------------------------------
   C 471-590: the Libotf/HarfBuzz arm.  `HAVE_LIBOTF` is `undef` in
   `docs/baseline/config/darwin-arm64.macros.tsv` 1421 and `HAVE_M17N_FLT` is undef under it, so
   C 472-493 and C 570-577 are dead here and C 578-590's `#elif defined HAVE_HARFBUZZ` arm is
   dead too (`HAVE_HARFBUZZ` undef, 1389).  **Neither arm can be compiled on this machine**: the
   tree has no `otf.h`/`m17n-flt.h` mirror, libotf is not installed (`/opt/homebrew` has no
   `otf.h`; HarfBuzz 14 is, `hb-face.h`), and under `harfbuzz` the two `hbotf_*` statics C 584-585
   declares are *defined* at C 3006 and C 3024 - later in this same file, so those two call sites
   are P-91 ④'s allowed class ("a `static` whose C line is known and whose definition is not yet
   ported").  The declarations below are P-88 ①'s use-site segment for `<otf.h>` (C 578-586's
   names); the libotf signatures are **UNVERIFIED** (that toolchain is not on this machine, P-90
   ④) and are read off C's own call sites (C 482-486, C 1024-1037).  */

/// C: 472: `#ifdef HAVE_LIBOTF` - the file's `HAVE_LIBOTF` arm (see the note above).
#[cfg (feature = "have_libotf")]
#[repr(C)]
pub struct OTF { _private: [u8; 0] }
/// C: 482: `otf = OTF_open_ft_face (ftfont_info->ft_size->face);` - `OTF *` is the record above.
#[cfg (feature = "have_libotf")]
pub type OTF_ref = *mut OTF;
/* C 482-486's three Libotf entry points, plus the two C 1024/1029-1031 adds.  Signatures
   **UNVERIFIED** (the libotf toolchain is not on this machine, P-90 ④).  */
#[cfg (feature = "have_libotf")]
unsafe extern "C" {
  /// C: 482: `OTF_open_ft_face (FT_Face face)` returns the handle.
  pub fn OTF_open_ft_face (face: FT_Face) -> *mut OTF;
  /// C: 483: `OTF_get_table (otf, "head") < 0` - negative on failure.
  pub fn OTF_get_table (otf: *mut OTF, tag: *const c_char) -> c_int;
  /// C: 486: `OTF_close (otf);`
  pub fn OTF_close (otf: *mut OTF);
  /// C: 1024: `otf = OTF_open ((char *) file);`
  pub fn OTF_open (file: *const c_char) -> *mut OTF;
  /// C: 1027-1036: `OTF_check_features (otf, 1, otspec->script_tag, otspec->langsys_tag,
  /// otspec->features[0], otspec->nfeatures[0]) == 1`.
  pub fn OTF_check_features (otf: *mut OTF, gsubp: c_int, script: c_uint, langsys: c_uint,
                             features: *const c_uint, nfeatures: c_int) -> c_int;
}

/* C 472-493: `ftfont_get_otf` - the `HAVE_LIBOTF`-only helper.  Its parameter is `struct
   font_info *`, whose record is `femacs/src/include/ftfont.rs`'s and whose two `HAVE_LIBOTF`
   members (`maybe_otf`, `otf`) that mirror records as dead text under its `have_x_windows` gate;
   the arm therefore cannot be typed here (the ledger's Owed 3 names the mirror work).  */
/// C: 472-474: `#ifdef HAVE_LIBOTF` / `static OTF *` / `ftfont_get_otf (struct font_info
/// *ftfont_info)`.
#[cfg (feature = "have_libotf")]
unsafe fn ftfont_get_otf (ftfont_info: *mut font_info) -> *mut OTF
{
  let otf: *mut OTF;

  if ! (*ftfont_info).otf.is_null ()
    { return (*ftfont_info).otf; }
  if ! (*ftfont_info).maybe_otf
    { return std::ptr::null_mut (); }
  otf = OTF_open_ft_face ((*(*ftfont_info).ft_size).face);
  if otf.is_null () || OTF_get_table (otf, c"head".as_ptr ()) < 0
    {
      if ! otf.is_null ()
        { OTF_close (otf); }
      (*ftfont_info).maybe_otf = 0;
      return std::ptr::null_mut ();
    }
  (*ftfont_info).otf = otf;
  otf
}

/* C 495-499: `ftfont_get_cache` - one of this file's five non-`static` definitions (`font.h` 942
   declares it), so it carries `#[unsafe(no_mangle)]` (PORTING.md 0.2).  */
/// C: 495-498: `Lisp_Object` / `ftfont_get_cache (struct frame *f)` / `return
/// freetype_font_cache;`  `struct frame` is an incomplete type in this translation unit
/// (`ftfont.c` includes no `frame.h`; `lisp.h` 3928's `struct frame;` is the declaration C reaches
/// it through), and Rust has no forward declaration of a type defined in another file: the
/// parameter is the ABI-equivalent untyped pointer (PORTING.md 0.1-2/P-55), which is the spelling
/// all six of this slice's signatures use.
#[cfg (feature = "have_freetype")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ftfont_get_cache (f: *mut c_void) -> Lisp_Object
{
  freetype_font_cache
}

/* C 501-545: `ftfont_get_charset`.  C's `re` buffer and the `regexp` string are the two
   `SAFE_ALLOCA`/`AUTO_STRING_WITH_LEN` sites, and the table walk keeps C's element expression
   with one local standing for `fc_charset_table[i]` (C 521-542 reads and writes it four times). */
/// C: 501-502: `static int` / `ftfont_get_charset (Lisp_Object registry)`.
#[cfg (feature = "have_freetype")]
unsafe fn ftfont_get_charset (registry: Lisp_Object) -> c_int
{
  let str_: *mut c_char = SSDATA (SYMBOL_NAME (registry));
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let re: *mut c_char
    = SAFE_ALLOCA! (sa_avail, SBYTES (SYMBOL_NAME (registry)) * 2 + 1) as *mut c_char;
  let mut i: c_int;
  let mut j: c_int;

  /* C 509-518: `for (i = j = 0; i < SBYTES (SYMBOL_NAME (registry)); i++, j++)` - the two
     assignments and the two increments of C's chained form, written out.  */
  i = 0;
  j = 0;
  while i < SBYTES (SYMBOL_NAME (registry)) as c_int
    {
      if *str_.add (i as usize) == b'.' as c_char
        { j += 1; *re.add (j as usize) = b'\\' as c_char; }
      else if *str_.add (i as usize) == b'*' as c_char
        { j += 1; *re.add (j as usize) = b'.' as c_char; }
      *re.add (j as usize) = *str_.add (i as usize);
      if *re.add (j as usize) == b'?' as c_char
        { *re.add (j as usize) = b'.' as c_char; }
      i += 1;
      j += 1;
    }
  *re.add (j as usize) = 0;
  AUTO_STRING_WITH_LEN! (regexp, re, j as ptrdiff_t);
  i = 0;
  while ! (*fc_charset_table.as_ptr ().add (i as usize)).name.is_null ()
    {
      if fast_c_string_match_ignore_case
           (regexp, (*fc_charset_table.as_ptr ().add (i as usize)).name,
            strlen ((*fc_charset_table.as_ptr ().add (i as usize)).name)) >= 0
        { break; }
      i += 1;
    }
  SAFE_FREE! (sa_count);
  if (*fc_charset_table.as_ptr ().add (i as usize)).name.is_null ()
    { return -1; }
  if (*fc_charset_table.as_ptr ().add (i as usize)).fc_charset.is_null ()
    {
      let charset: *mut FcCharSet = FcCharSetCreate ();
      let uniquifier: *mut c_int =
        (*fc_charset_table.as_mut_ptr ().add (i as usize)).uniquifier.as_mut_ptr ();

      if charset.is_null ()
        { return -1; }
      j = 0;
      while *uniquifier.add (j as usize) != 0
        {
          if FcCharSetAddChar (charset, *uniquifier.add (j as usize) as FcChar32) == 0
            {
              FcCharSetDestroy (charset);
              return -1;
            }
          j += 1;
        }
      (*fc_charset_table.as_mut_ptr ().add (i as usize)).fc_charset = charset;
    }
  i
}

/* C 547-553: `struct OpenTypeSpec` - the record C 596 `malloc`s and C 604-654 fills.  Its two
   `unsigned int *features[2]` members are heap arrays whose length is `nfeatures[i]`.  */
/// C: 547-553: `struct OpenTypeSpec { Lisp_Object script; unsigned int script_tag, langsys_tag;
/// int nfeatures[2]; unsigned int *features[2]; };`
#[cfg (feature = "have_freetype")]
#[repr(C)]
pub struct OpenTypeSpec
{
  pub script: Lisp_Object,
  pub script_tag: c_uint,
  pub langsys_tag: c_uint,
  pub nfeatures: [c_int; 2],
  pub features: [*mut c_uint; 2],
}

/* C 555-568: the two tag macros.  C's `do { … } while (0)` wrappers have no Rust counterpart
   (PORTING.md 2.2's `do…while` class), and Rust has no macro metavariable for an l-value, so the
   destination is written `*&mut $place = value`, which is the statement C's `TAG = …` is.  */
/// C: 555-559: `#define OTF_SYM_TAG(SYM, TAG)` - the four `unsigned char` bytes of the symbol's
/// name as a big-endian tag.
#[cfg (feature = "have_freetype")]
macro_rules! OTF_SYM_TAG {
  ($sym:expr, $tag:expr) => {{
    let p = SDATA (SYMBOL_NAME ($sym));
    *&mut $tag = ((*p.add (0) as c_uint) << 24) | ((*p.add (1) as c_uint) << 16)
               | ((*p.add (2) as c_uint) << 8) | (*p.add (3) as c_uint);
  }};
}
/// C: 561-568: `#define OTF_TAG_STR(TAG, P)` - the tag's four bytes into `P`, NUL-terminated at
/// `P[4]`.
#[cfg (feature = "have_freetype")]
macro_rules! OTF_TAG_STR {
  ($tag:expr, $p:expr) => {{
    *$p.add (0) = ($tag >> 24) as c_char;
    *$p.add (1) = (($tag >> 16) & 0xFF) as c_char;
    *$p.add (2) = (($tag >> 8) & 0xFF) as c_char;
    *$p.add (3) = ($tag & 0xFF) as c_char;
    *$p.add (4) = b'\0' as c_char;
  }};
}

/* C 570-577: `#ifdef HAVE_LIBOTF` -> `#define OTF_TAG_SYM(SYM, TAG)`.  */
/// C: 570-577: the libotf arm's `OTF_TAG_SYM`, whose `OTF_TAG_STR` result is interned.
#[cfg (feature = "have_libotf")]
macro_rules! OTF_TAG_SYM {
  ($sym:expr, $tag:expr) => {{
    let mut str_: [c_char; 5] = [0; 5];

    OTF_TAG_STR! ($tag, str_.as_mut_ptr ());
    *&mut $sym = font_intern_prop (str_.as_ptr (), 4, true);
  }};
}

/* C 578-590: `#elif defined HAVE_HARFBUZZ` - the Libotf emulations on HarfBuzz that C 579-581's
   comment describes, the two `typedef`s, the two `static` prototypes and the three
   `#define`s that rename the entry points.  */
/// C: 582: `typedef hb_face_t OTF;`  `hb_face_t` is `<hb.h>`'s incomplete record; HarfBuzz is on
/// this machine (14.x, measured), so the record is declared rather than left to a mirror that
/// does not exist yet (Owed 4 names `femacs/src/include/hb.rs`).
#[cfg (all (not (feature = "have_libotf"), feature = "harfbuzz"))]
#[repr(C)]
pub struct hb_face_t { _private: [u8; 0] }
/// C: 582: the alias itself.
#[cfg (all (not (feature = "have_libotf"), feature = "harfbuzz"))]
pub type OTF = hb_face_t;
/// C: 583: `typedef unsigned int OTF_tag;`
#[cfg (all (not (feature = "have_libotf"), feature = "harfbuzz"))]
pub type OTF_tag = c_uint;
/* C: 584-585: `static OTF *hbotf_open (const char *);` and `static int
   hbotf_check_features (OTF *, int, OTF_tag, OTF_tag, const OTF_tag *, int);`  Both are *defined*
   at C 3006 and C 3024, so the two names are declared here and the definitions arrive with the
   file's `HAVE_HARFBUZZ` tail (P-91 ④: the ledger names both C lines at each site).  */
#[cfg (all (not (feature = "have_libotf"), feature = "harfbuzz"))]
unsafe extern "C" {
  /// C: 584: the `hbotf_open` prototype.
  pub fn hbotf_open (name: *const c_char) -> *mut OTF;
  /// C: 585-586: the `hbotf_check_features` prototype.
  pub fn hbotf_check_features (otf: *mut OTF, gsubp: c_int, script: OTF_tag,
                               langsys: OTF_tag, features: *const OTF_tag,
                               nfeatures: c_int) -> c_int;
}
/* C: 588: `#define OTF_close hb_face_destroy` - HarfBuzz's own entry point, declared from
   `hb-face.h` 116-117: `HB_EXTERN void hb_face_destroy (hb_face_t *face);`  */
#[cfg (all (not (feature = "have_libotf"), feature = "harfbuzz"))]
unsafe extern "C" {
  pub fn hb_face_destroy (face: *mut hb_face_t);
}

/* C 593-657: `ftfont_get_open_type_spec` - C 596's `malloc` and C 634/638-639's `free`s.  */
/// C: 593-594: `static struct OpenTypeSpec *` / `ftfont_get_open_type_spec (Lisp_Object
/// otf_spec)`.
#[cfg (feature = "have_freetype")]
unsafe fn ftfont_get_open_type_spec (otf_spec: Lisp_Object) -> *mut OpenTypeSpec
{
  let mut otf_spec: Lisp_Object = otf_spec;
  let spec: *mut OpenTypeSpec
    = malloc (std::mem::size_of::<OpenTypeSpec> ()) as *mut OpenTypeSpec;
  let mut val: Lisp_Object = Qnil ();
  let mut i: c_int;
  let mut j: c_int;
  let mut negative: bool;

  if spec.is_null ()
    { return std::ptr::null_mut (); }
  (*spec).script = XCAR (otf_spec);
  if ! NILP ((*spec).script)
    {
      OTF_SYM_TAG! ((*spec).script, (*spec).script_tag);
      val = assq_no_quit ((*spec).script, Votf_script_alist);
      if CONSP (val) && SYMBOLP (XCDR (val))
        { (*spec).script = XCDR (val); }
      else
        { (*spec).script = Qnil (); }
    }
  else
    { (*spec).script_tag = 0x44464C54; }	/* "DFLT" */
  otf_spec = XCDR (otf_spec);
  (*spec).langsys_tag = 0;
  if ! NILP (otf_spec)
    {
      val = XCAR (otf_spec);
      if ! NILP (val)
        { OTF_SYM_TAG! (val, (*spec).langsys_tag); }
      otf_spec = XCDR (otf_spec);
    }
  /* C 624: `spec->nfeatures[0] = spec->nfeatures[1] = 0;` - C's chained assignment, which the
     port writes as two statements (the `lisp.rs` `spec->nfeatures` shape).  */
  (*spec).nfeatures[0] = 0;
  (*spec).nfeatures[1] = 0;
  i = 0;
  while i < 2 && ! NILP (otf_spec)
    {
      val = XCAR (otf_spec);
      if NILP (val)
        {
          i += 1;
          otf_spec = XCDR (otf_spec);
          continue;
        }
      let len: ptrdiff_t = list_length (val);
      (*spec).features[i as usize]
        = if (min (PTRDIFF_MAX, SIZE_MAX as isize)
              / std::mem::size_of::<c_int> () as isize) < len
            { std::ptr::null_mut () }
            else
            { malloc ((len * std::mem::size_of::<c_int> () as isize) as usize) as *mut c_uint };
      if (*spec).features[i as usize].is_null ()
        {
          if i > 0 && ! (*spec).features[0].is_null ()
            { free ((*spec).features[0] as *mut c_void); }
          free (spec as *mut c_void);
          return std::ptr::null_mut ();
        }
      j = 0;
      negative = false;
      while CONSP (val)
        {
          if NILP (XCAR (val))
            { negative = true; }
          else
            {
              let mut tag: c_uint = 0;

              OTF_SYM_TAG! (XCAR (val), tag);
              *(*spec).features[i as usize].add (j as usize)
                = if negative { tag | 0x80000000 } else { tag };
              j += 1;
            }
          val = XCDR (val);
        }
      (*spec).nfeatures[i as usize] = j;
      i += 1;
      otf_spec = XCDR (otf_spec);
    }
  spec
}

/* C 659-684: `#if defined HAVE_XFT && defined FC_COLOR` / the Xft-older-than-2.3.6 test / the
   whitelist helper / the two `#endif`s.  The condition is the two features of the file's head
   note plus `xft_version_lt_2_3_6`; C 665-681's body is what C 985-991 calls.  */
/// C: 665-666: `static bool` / `xft_color_font_whitelisted_p (const char *family)`.
#[cfg (all (feature = "have_freetype", feature = "have_xft", feature = "fc_color",
            feature = "xft_version_lt_2_3_6"))]
unsafe fn xft_color_font_whitelisted_p (family: *const c_char) -> bool
{
  let mut tem: Lisp_Object = Vxft_color_font_whitelist;

  FOR_EACH_TAIL_SAFE! (tem, {
    let name: Lisp_Object = XCAR (tem);

    if STRINGP (name) && strcmp (family, SSDATA (name)) == 0
      { return true; }
  });

  false
}

/* C 686-862: `ftfont_spec_pattern` - the Lisp font-spec -> Fontconfig pattern converter.  C 701's
   assignment inside the condition and C 624-style chained forms are statements here; C 334/732's
   `goto err` shape is the `'err` block and C 838's `goto finish` is `break 'finish`, so C 840's
   cleanup runs on the error path only and C 858's `finish:` code runs on both.  */
/// C: 686-688: `static FcPattern *` / `ftfont_spec_pattern (Lisp_Object spec, char *otlayout,
/// struct OpenTypeSpec **otspec, const char **langname)`.
#[cfg (feature = "have_freetype")]
unsafe fn ftfont_spec_pattern (spec: Lisp_Object, otlayout: *mut c_char,
                               otspec: *mut *mut OpenTypeSpec,
                               langname: *mut *const c_char) -> *mut FcPattern
{
  let mut tmp: Lisp_Object = Qnil ();
  let mut extra: Lisp_Object;
  let mut pattern: *mut FcPattern = std::ptr::null_mut ();
  let mut charset: *mut FcCharSet = std::ptr::null_mut ();
  let mut langset: *mut FcLangSet = std::ptr::null_mut ();
  let mut n: c_int;
  let mut dpi: c_int = -1;
  let mut scalable: c_int = -1;
  let mut script: Lisp_Object = Qnil ();
  let registry: Lisp_Object;
  let mut fc_charset_idx: c_int = 0;

  /* C 701-704: the assignment inside C's condition is a statement here.  */
  n = FONT_SLANT_NUMERIC! (spec) as c_int;
  if n >= 0
      && n < 100
    /* Fontconfig doesn't support reverse-italic/oblique.  */
    { return std::ptr::null_mut (); }

  if FIXNUMP (AREF (spec, FONT_DPI_INDEX as isize))
    { dpi = XFIXNUM (AREF (spec, FONT_DPI_INDEX as isize)) as c_int; }
  if FIXNUMP (AREF (spec, FONT_AVGWIDTH_INDEX as isize))
      && XFIXNUM (AREF (spec, FONT_AVGWIDTH_INDEX as isize)) == 0
    { scalable = 1; }

  'finish: {
    'err: {
      registry = AREF (spec, FONT_REGISTRY_INDEX as isize);
      if NILP (registry)
          || EQ (registry, Qascii_0 ())
          || EQ (registry, Qiso10646_1 ())
          || EQ (registry, Qunicode_bmp ())
        { fc_charset_idx = -1; }
      else
        {
          let lang: *const FcChar8;

          fc_charset_idx = ftfont_get_charset (registry);
          if fc_charset_idx < 0
            { return std::ptr::null_mut (); }
          charset = (*fc_charset_table.as_mut_ptr ().add (fc_charset_idx as usize)).fc_charset;
          *langname = (*fc_charset_table.as_ptr ().add (fc_charset_idx as usize)).lang;
          lang = *langname as *const FcChar8;
          if ! lang.is_null ()
            {
              langset = FcLangSetCreate ();
              if langset.is_null ()
                { break 'err; }
              FcLangSetAdd (langset, lang);
            }
        }

      *otlayout = 0;
      extra = AREF (spec, FONT_EXTRA_INDEX as isize);
      while CONSP (extra)
        {
          let key: Lisp_Object = XCAR (XCAR (extra));
          let mut val: Lisp_Object = XCDR (XCAR (extra));

          if EQ (key, QCdpi ())
            {
              if FIXNUMP (val)
                { dpi = XFIXNUM (val) as c_int; }
            }
          else if EQ (key, QClang ())
            {
              if langset.is_null ()
                { langset = FcLangSetCreate (); }
              if langset.is_null ()
                { break 'err; }
              if SYMBOLP (val)
                {
                  if FcLangSetAdd (langset, SYMBOL_FcChar8! (val)) == 0
                    { break 'err; }
                }
              else
                {
                  while CONSP (val)
                    {
                      if SYMBOLP (XCAR (val))
                          && FcLangSetAdd (langset, SYMBOL_FcChar8! (XCAR (val))) == 0
                        { break 'err; }
                      val = XCDR (val);
                    }
                }
            }
          else if EQ (key, QCotf ())
            {
              if CONSP (val)
                {
                  *otspec = ftfont_get_open_type_spec (val);
                  if (*otspec).is_null ()
                    { return std::ptr::null_mut (); }
                  strcpy (otlayout, c"otlayout:".as_ptr ());
                  OTF_TAG_STR! ((**otspec).script_tag, otlayout.add (9));
                  script = (**otspec).script;
                }
            }
          else if EQ (key, QCscript ())
            { script = val; }
          else if EQ (key, QCscalable ())
            { scalable = if ! NILP (val) { 1 } else { 0 }; }
          extra = XCDR (extra);
        }

      if ! NILP (script) && charset.is_null ()
        {
          let mut chars: Lisp_Object
            = assq_no_quit (script, Vscript_representative_chars);

          if CONSP (chars) && CONSP (CDR (chars))
            {
              charset = FcCharSetCreate ();
              if charset.is_null ()
                { break 'err; }
              chars = XCDR (chars);
              while CONSP (chars)
                {
                  if CHARACTERP (XCAR (chars))
                      && FcCharSetAddChar (charset, XFIXNAT (XCAR (chars)) as FcChar32) == 0
                    { break 'err; }
                  chars = XCDR (chars);
                }
            }
        }

      pattern = FcPatternCreate ();
      if pattern.is_null ()
        { break 'err; }
      tmp = AREF (spec, FONT_FOUNDRY_INDEX as isize);
      if ! NILP (tmp)
          && FcPatternAddString (pattern, FC_FOUNDRY, SYMBOL_FcChar8! (tmp)) == 0
        { break 'err; }
      tmp = AREF (spec, FONT_FAMILY_INDEX as isize);
      if ! NILP (tmp)
          && FcPatternAddString (pattern, FC_FAMILY, SYMBOL_FcChar8! (tmp)) == 0
        { break 'err; }
      if ! charset.is_null ()
          && FcPatternAddCharSet (pattern, FC_CHARSET, charset) == 0
        { break 'err; }
      if ! langset.is_null ()
          && FcPatternAddLangSet (pattern, FC_LANG, langset) == 0
        { break 'err; }
      if dpi >= 0
          && FcPatternAddDouble (pattern, FC_DPI, dpi as c_double) == 0
        { break 'err; }
      if scalable >= 0
          && FcPatternAddBool (pattern, FC_SCALABLE,
                               if scalable != 0 { FcTrue } else { FcFalse }) == 0
        { break 'err; }
      /* C 823-836: `#if defined HAVE_XFT && defined FC_COLOR` and the Xft-older-than-2.3.6 test,
         which the two features spell.  C 828-829's comment is kept.  */
      #[cfg (all (feature = "have_xft", feature = "fc_color",
                  feature = "xft_version_lt_2_3_6"))]
      {
        /* We really don't like color fonts, they cause Xft crashes with
           releases older than 2.3.6.  See Bug#30874.  */
        if xft_ignore_color_fonts
            && (NILP (AREF (spec, FONT_FAMILY_INDEX as isize))
                || NILP (Vxft_color_font_whitelist))
            && FcPatternAddBool (pattern, FC_COLOR, FcFalse) == 0
          { break 'err; }
      }

      break 'finish;
    }

    /* C 840-856: `err:` and its two cleanups (the pattern and the open-type spec).  */
    /* We come here because of unexpected error in fontconfig API call
       (usually insufficient memory).  */
    if ! pattern.is_null ()
      {
        FcPatternDestroy (pattern);
        pattern = std::ptr::null_mut ();
      }
    if ! (*otspec).is_null ()
      {
        if (**otspec).nfeatures[0] > 0
          { free ((**otspec).features[0] as *mut c_void); }
        if (**otspec).nfeatures[1] > 0
          { free ((**otspec).features[1] as *mut c_void); }
        free (*otspec as *mut c_void);
        *otspec = std::ptr::null_mut ();
      }
  }
  /* C 858-861: `finish:`, the two destroys and the return.  */
  if ! langset.is_null ()
    { FcLangSetDestroy (langset); }
  if ! charset.is_null () && fc_charset_idx < 0
    { FcCharSetDestroy (charset); }
  pattern
}

/* ----------------------------------------------------------------------------------------------
   C 864-1089: `ftfont_list` - one of the file's non-`static` definitions.  Three notes:

   * C 909-940's `FcObjectSetBuild` argument list has four `#ifdef`-guarded members
     (`FC_CAPABILITY`, `FC_FONTFORMAT`, `FC_COLOR`, `FC_VARIABLE`).  Rust cannot put a `#[cfg]`
     on one argument of a variadic call, so the port keeps C's *fixed* eleven names in the call
     and appends the four under their own features with `FcObjectSetAdd` - the same set in the same
     order (they are C's last four before `NULL`, and C 944 already appends `FC_CHARSET` that way).
     The four features keep their C conditions at every site where a statement can carry one
     (C 927/972's `FC_COLOR` blocks, C 1005's `FC_CAPABILITY` test, C 1297's `FC_EMBOLDEN`).
   * C 942-968 is `#if 0`, dead in every configuration: recorded, not transcribed.
   * C 1016-1041's `#if defined HAVE_LIBOTF || defined HAVE_HARFBUZZ` arm calls the `OTF_*` names;
     `OTF_open`/`OTF_close` are macros over different functions per arm (C 587-589), and under
     `harfbuzz` the two `hbotf_*` statics are defined at C 3006/3024 - outside this slice, so
     those two call sites are P-91 ④'s allowed class and the ledger names both C lines.  */
/// C: 864-865: `static Lisp_Object` / `ftfont_list (struct frame *f, Lisp_Object spec)` (the
/// `struct frame *` is the untyped pointer, see `ftfont_get_cache`).
#[cfg (feature = "have_freetype")]
unsafe fn ftfont_list (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{
  let mut val: Lisp_Object = Qnil ();
  let mut family: Lisp_Object = Qnil ();
  let mut adstyle: Lisp_Object = Qnil ();
  let mut i: c_int;
  let mut pattern: *mut FcPattern;
  let mut fontset: *mut FcFontSet = std::ptr::null_mut ();
  let mut objset: *mut FcObjectSet = std::ptr::null_mut ();
  let mut charset: *mut FcCharSet = std::ptr::null_mut ();
  let mut chars: Lisp_Object = Qnil ();
  let mut otlayout: [c_char; 15] = [0; 15];	/* For "otlayout:XXXX" */
  let mut otspec: *mut OpenTypeSpec = std::ptr::null_mut ();
  let mut spacing: c_int = -1;
  let mut langname: *const c_char = std::ptr::null ();

  if ! fc_initialized
    {
      FcInit ();
      fc_initialized = true;
    }

  pattern = ftfont_spec_pattern (spec, otlayout.as_mut_ptr (), &raw mut otspec,
                                 &raw mut langname);
  if pattern.is_null ()
    { return Qnil (); }

  'finish: {
    'err: {
      if FcPatternGetCharSet (pattern, FC_CHARSET, 0, &raw mut charset) != FcResultMatch
        {
          val = assq_no_quit (QCscript (), AREF (spec, FONT_EXTRA_INDEX as isize));
          if ! NILP (val)
            {
              val = assq_no_quit (XCDR (val), Vscript_representative_chars);
              if CONSP (val) && VECTORP (XCDR (val))
                { chars = XCDR (val); }
            }
          val = Qnil ();
        }
      if FIXNUMP (AREF (spec, FONT_SPACING_INDEX as isize))
        { spacing = XFIXNUM (AREF (spec, FONT_SPACING_INDEX as isize)) as c_int; }
      family = AREF (spec, FONT_FAMILY_INDEX as isize);
      if ! NILP (family)
        {
          let resolved: Lisp_Object = ftfont_resolve_generic_family (family, pattern);

          if ! NILP (resolved)
            {
              FcPatternDel (pattern, FC_FAMILY);
              if FcPatternAddString (pattern, FC_FAMILY, SYMBOL_FcChar8! (resolved)) == 0
                { break 'err; }
            }
        }
      adstyle = AREF (spec, FONT_ADSTYLE_INDEX as isize);
      if ! NILP (adstyle) && SBYTES (SYMBOL_NAME (adstyle)) == 0
        { adstyle = Qnil (); }
      objset = FcObjectSetBuild (FC_FOUNDRY, FC_FAMILY, FC_WEIGHT, FC_SLANT,
                                 FC_WIDTH, FC_PIXEL_SIZE, FC_SPACING, FC_SCALABLE,
                                 FC_STYLE, FC_FILE, FC_INDEX,
                                 std::ptr::null::<c_char> ());
      /* C 921/: the four `#ifdef`-guarded members, appended under their own features (the
         function's head note states why).  */
      #[cfg (feature = "fc_capability")]
      { FcObjectSetAdd (objset, FC_CAPABILITY); }
      #[cfg (feature = "fc_fontformat")]
      { FcObjectSetAdd (objset, FC_FONTFORMAT); }
      #[cfg (all (feature = "have_xft", feature = "fc_color"))]
      { FcObjectSetAdd (objset, FC_COLOR); }
      #[cfg (feature = "fc_variable")]
      { FcObjectSetAdd (objset, FC_VARIABLE); }
      if objset.is_null ()
        { break 'err; }
      if ! NILP (chars)
        { FcObjectSetAdd (objset, FC_CHARSET); }

      fontset = FcFontList (std::ptr::null_mut (), pattern, objset);
      if fontset.is_null () || (*fontset).nfont == 0
        { break 'finish; }
      /* C 942-968: `#if 0` - dead in every configuration, recorded and not transcribed.  */
      i = 0;
      while i < (*fontset).nfont
        {
          let mut entity: Lisp_Object;
          /* C 972-994: the `#if defined HAVE_XFT && defined FC_COLOR` filter and its
             Xft-older-than-2.3.6 test.  C 979-983's comment is kept.  */
          #[cfg (all (feature = "have_xft", feature = "fc_color",
                      feature = "xft_version_lt_2_3_6"))]
          {
            /* Some fonts, notably NotoColorEmoji, have an FC_COLOR value
               that's neither FcTrue nor FcFalse, which means FcFontList
               returns them even when it shouldn't really do so, so we
               need to manually skip them here (Bug#37786).  */
            let mut b: FcBool = FcFalse;
            let mut str_: *mut FcChar8 = std::ptr::null_mut ();

            if xft_ignore_color_fonts
                && (FcPatternGetString (*(*fontset).fonts.add (i as usize), FC_FAMILY,
                                        0, &raw mut str_) != FcResultMatch
                    || ! xft_color_font_whitelisted_p (str_ as *const c_char))
                && FcPatternGetBool (*(*fontset).fonts.add (i as usize), FC_COLOR, 0,
                                     &raw mut b) == FcResultMatch
                && b != FcFalse
              { i += 1; continue; }	/* C 987-989: `continue;`.  */
          }
          if spacing >= 0
            {
              let mut this: c_int = 0;

              if (FcPatternGetInteger (*(*fontset).fonts.add (i as usize), FC_SPACING, 0,
                                       &raw mut this) == FcResultMatch)
                  && spacing != this
                { i += 1; continue; }	/* C 1000-1001: `continue;`.  */
            }

          /* C 1005-1014: `#ifdef FC_CAPABILITY` and its `otlayout` test.  */
          #[cfg (feature = "fc_capability")]
          {
            if *otlayout.as_ptr () != 0
              {
                let mut this_: *mut FcChar8 = std::ptr::null_mut ();

                if FcPatternGetString (*(*fontset).fonts.add (i as usize), FC_CAPABILITY, 0,
                                       &raw mut this_) != FcResultMatch
                    || strstr (this_ as *mut c_char, otlayout.as_ptr ()).is_null ()
                  { i += 1; continue; }	/* C 1012-1013: `continue;`.  */
              }
          }
          /* C 1016-1041: `#if defined HAVE_LIBOTF || defined HAVE_HARFBUZZ` - the required
             OpenType-feature test.  */
          #[cfg (any (feature = "have_libotf", feature = "harfbuzz"))]
          {
            if ! otspec.is_null ()
              {
                let mut file: *mut FcChar8 = std::ptr::null_mut ();
                let mut passed: bool;
                let mut otf: *mut OTF;

                if FcPatternGetString (*(*fontset).fonts.add (i as usize), FC_FILE, 0,
                                       &raw mut file) != FcResultMatch
                  { i += 1; continue; }	/* C 1025-1026: `continue;`.  */
                /* C 1029: `otf = OTF_open ((char *) file);` - `OTF_open` is `hbotf_open` under
                   `HAVE_HARFBUZZ` (C 587) and libotf's own entry point otherwise; an object-like
                   `#define` has no Rust counterpart, so each arm spells the site the way C's
                   preprocessor leaves it.  The `harfbuzz` arm's callee is C 3006's definition.  */
                #[cfg (feature = "have_libotf")]
                { otf = OTF_open (file as *const c_char); }
                #[cfg (all (not (feature = "have_libotf"), feature = "harfbuzz"))]
                { otf = hbotf_open (file as *const c_char); }	/* waits on C 3006.  */
                if otf.is_null ()
                  { i += 1; continue; }	/* C 1030-1031: `continue;`.  */
                /* C 1032-1037: the two `OTF_check_features` calls and their `OTF_close`.  */
                #[cfg (feature = "have_libotf")]
                { passed = OTF_check_features (otf, 1, (*otspec).script_tag,
                                               (*otspec).langsys_tag,
                                               (*otspec).features[0],
                                               (*otspec).nfeatures[0]) == 1; }
                #[cfg (all (not (feature = "have_libotf"), feature = "harfbuzz"))]
                { passed = hbotf_check_features (otf, 1, (*otspec).script_tag,
                                                 (*otspec).langsys_tag,
                                                 (*otspec).features[0],
                                                 (*otspec).nfeatures[0]) == 1; } /* C 3024.  */
                #[cfg (feature = "have_libotf")]
                { passed = passed && OTF_check_features (otf, 0, (*otspec).script_tag,
                                                         (*otspec).langsys_tag,
                                                         (*otspec).features[1],
                                                         (*otspec).nfeatures[1]) == 1; }
                #[cfg (all (not (feature = "have_libotf"), feature = "harfbuzz"))]
                { passed = passed && hbotf_check_features (otf, 0, (*otspec).script_tag,
                                                           (*otspec).langsys_tag,
                                                           (*otspec).features[1],
                                                           (*otspec).nfeatures[1]) == 1; }
                #[cfg (feature = "have_libotf")]
                { OTF_close (otf); }
                #[cfg (all (not (feature = "have_libotf"), feature = "harfbuzz"))]
                { hb_face_destroy (otf); }	/* C 588's `#define OTF_close hb_face_destroy`.  */
                if ! passed
                  { i += 1; continue; }	/* C 1038-1039: `continue;`.  */
              }
          }
          if VECTORP (chars)
            {
              let mut j: ptrdiff_t;

              if FcPatternGetCharSet (*(*fontset).fonts.add (i as usize), FC_CHARSET, 0,
                                      &raw mut charset) != FcResultMatch
                { i += 1; continue; }	/* C 1052-1053: `continue;`.  */
              j = 0;
              while j < ASIZE (chars)
                {
                  /* C 1055-1056: `TYPE_RANGED_FIXNUMP (FcChar32, AREF (chars, j))` - C's macro
                     takes the *type*, while the port's `TYPE_RANGED_FIXNUMP (lo, x, hi)` takes the
                     two bounds.  `FcChar32` is unsigned, so the range is 0 .. `UINT_MAX`
                     (`lisp.h` 3267-3270's `TYPE_SIGNED (type) ? … : 0 <= XFIXNUM (x)` arm).  */
                  if TYPE_RANGED_FIXNUMP (0, AREF (chars, j), UINT_MAX as EMACS_INT)
                      && FcCharSetHasChar (charset, XFIXNAT (AREF (chars, j)) as FcChar32) != 0
                    { break; }
                  j += 1;
                }
              if j == ASIZE (chars)
                { i += 1; continue; }	/* C 1057-1058: `continue;`.  */
            }
          if ! NILP (adstyle) || ! langname.is_null ()
            {
              let this_adstyle: Lisp_Object
                = get_adstyle_property (*(*fontset).fonts.add (i as usize));

              if ! NILP (adstyle)
                  && (NILP (this_adstyle)
                      || dispextern_externs::xstrcasecmp (SSDATA (SYMBOL_NAME (adstyle)),
                                      SSDATA (SYMBOL_NAME (this_adstyle))) != 0)
                { i += 1; continue; }	/* C 1069-1070: `continue;`.  */
              if ! langname.is_null ()
                  && ! NILP (this_adstyle)
                  && dispextern_externs::xstrcasecmp (langname, SSDATA (SYMBOL_NAME (this_adstyle))) != 0
                { i += 1; continue; }	/* C 1071-1074: `continue;`.  */
            }
          entity = ftfont_pattern_entity (*(*fontset).fonts.add (i as usize),
                                          AREF (spec, FONT_EXTRA_INDEX as isize));
          if ! NILP (entity)
            { val = Fcons (entity, val); }
          i += 1;
        }
      val = Fnreverse (val);
      break 'finish;			/* C 1076: `goto finish;`.  */
    }

    /* C 1078-1081: `err:` and its `val = Qnil;` - the unexpected-error path ("usually
       insufficient memory").  */
    /* We come here because of unexpected error in fontconfig API call
       (usually insufficient memory).  */
    val = Qnil ();
  }
  /* C 1083-1088: `finish:`, the log call and the three destroys.  */
  FONT_ADD_LOG! (c"ftfont-list".as_ptr (), spec, val);
  if ! objset.is_null ()
    { FcObjectSetDestroy (objset); }
  if ! fontset.is_null ()
    { FcFontSetDestroy (fontset); }
  if ! pattern.is_null ()
    { FcPatternDestroy (pattern); }
  val
}

/* C 1091-1099: `ftfont_list2` - non-`static` (`font.h` 941 declares it), so `#[unsafe(no_mangle)]`.
   C's `for (Lisp_Object tail = list; CONSP (tail); tail = XCDR (tail))` declares `tail` in the
   loop head, which the port writes as a `while` loop's preceding binding.  */
/// C: 1091-1097: the definition, the loop and the `ASET` of the `FONT_TYPE_INDEX` slot.
#[cfg (feature = "have_freetype")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ftfont_list2 (f: *mut c_void, spec: Lisp_Object,
                                       r#type: Lisp_Object) -> Lisp_Object
{
  let list: Lisp_Object = ftfont_list (f, spec);
  let mut tail: Lisp_Object = list;

  while CONSP (tail)
    {
      ASET (XCAR (tail), FONT_TYPE_INDEX as isize, r#type);
      tail = XCDR (tail);
    }
  list
}

/* C 1101-1149: `ftfont_match` - `static`, so a plain `unsafe fn`.  */
/// C: 1101-1102: `static Lisp_Object` / `ftfont_match (struct frame *f, Lisp_Object spec)` (the
/// untyped pointer, see `ftfont_get_cache`).
#[cfg (feature = "have_freetype")]
unsafe fn ftfont_match (f: *mut c_void, spec: Lisp_Object) -> Lisp_Object
{
  let mut entity: Lisp_Object = Qnil ();
  let mut pattern: *mut FcPattern;
  let mut match_: *mut FcPattern = std::ptr::null_mut ();
  let mut result: FcResult = FcResultMatch;
  let mut otlayout: [c_char; 15] = [0; 15];	/* For "otlayout:XXXX" */
  let mut otspec: *mut OpenTypeSpec = std::ptr::null_mut ();
  let mut langname: *const c_char = std::ptr::null ();

  if ! fc_initialized
    {
      FcInit ();
      fc_initialized = true;
    }

  pattern = ftfont_spec_pattern (spec, otlayout.as_mut_ptr (), &raw mut otspec,
                                 &raw mut langname);
  if pattern.is_null ()
    { return Qnil (); }

  if FIXNUMP (AREF (spec, FONT_SIZE_INDEX as isize))
    {
      let mut value: FcValue = unsafe { std::mem::zeroed () };

      value.r#type = FcTypeDouble;
      value.u.d = XFIXNUM (AREF (spec, FONT_SIZE_INDEX as isize)) as c_double;
      FcPatternAdd (pattern, FC_PIXEL_SIZE, value, FcFalse);
    }
  if FcConfigSubstitute (std::ptr::null_mut (), pattern, FcMatchPattern) == FcTrue
    {
      FcDefaultSubstitute (pattern);
      match_ = FcFontMatch (std::ptr::null_mut (), pattern, &raw mut result);
      if ! match_.is_null ()
        {
          entity = ftfont_pattern_entity (match_, AREF (spec, FONT_EXTRA_INDEX as isize));
          FcPatternDestroy (match_);
          if ! NILP (AREF (spec, FONT_FAMILY_INDEX as isize))
              && NILP (assq_no_quit (AREF (spec, FONT_FAMILY_INDEX as isize),
                                     ftfont_generic_family_list))
              && NILP (Fstring_equal (AREF (spec, FONT_FAMILY_INDEX as isize),
                                      AREF (entity, FONT_FAMILY_INDEX as isize)))
            { entity = Qnil (); }
        }
    }
  FcPatternDestroy (pattern);

  FONT_ADD_LOG! (c"ftfont-match".as_ptr (), spec, entity);
  entity
}

/* C 1151-1159: `ftfont_match2` - non-`static` (`font.h` 943 declares it).  */
/// C: 1151-1157: the definition and the `FONT_TYPE_INDEX` write.
#[cfg (feature = "have_freetype")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ftfont_match2 (f: *mut c_void, spec: Lisp_Object,
                                        r#type: Lisp_Object) -> Lisp_Object
{
  let mut entity: Lisp_Object = ftfont_match (f, spec);

  if ! NILP (entity)
    { ASET (entity, FONT_TYPE_INDEX as isize, r#type); }
  entity
}

/* C 1161-1201: `ftfont_list_family` - non-`static` (`font.h` 944 declares it).  */
/// C: 1161-1162: `Lisp_Object` / `ftfont_list_family (struct frame *f)`.
#[cfg (feature = "have_freetype")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ftfont_list_family (f: *mut c_void) -> Lisp_Object
{
  let mut list: Lisp_Object = Qnil ();
  let mut pattern: *mut FcPattern = std::ptr::null_mut ();
  let mut fontset: *mut FcFontSet = std::ptr::null_mut ();
  let mut objset: *mut FcObjectSet = std::ptr::null_mut ();
  let mut i: c_int;

  if ! fc_initialized
    {
      FcInit ();
      fc_initialized = true;
    }

  'finish: {
    pattern = FcPatternCreate ();
    if pattern.is_null ()
      { break 'finish; }
    objset = FcObjectSetBuild (FC_FAMILY, std::ptr::null::<c_char> ());
    if objset.is_null ()
      { break 'finish; }
    fontset = FcFontList (std::ptr::null_mut (), pattern, objset);
    if fontset.is_null ()
      { break 'finish; }

    i = 0;
    while i < (*fontset).nfont
      {
        let pat: *mut FcPattern = *(*fontset).fonts.add (i as usize);
        let mut str_: *mut FcChar8 = std::ptr::null_mut ();

        if FcPatternGetString (pat, FC_FAMILY, 0, &raw mut str_) == FcResultMatch
          { list = Fcons (intern (str_ as *const c_char), list); }
        i += 1;
      }
  }
  /* C 1196-1199: `finish:` and the three destroys.  */
  if ! objset.is_null ()
    { FcObjectSetDestroy (objset); }
  if ! fontset.is_null ()
    { FcFontSetDestroy (fontset); }
  if ! pattern.is_null ()
    { FcPatternDestroy (pattern); }

  list
}

/* C 1203-1254: `ftfont_fix_match` - the first of the three entry points `ftfont.h` 45-47
   declares, and therefore one of the file's non-`static` definitions.  C 1227-1229 and C 1235-1238
   are the header's two `#ifndef` fallback definitions of `FC_HINT_STYLE`/`FC_LCD_FILTER`; both
   names are declared once in the Fontconfig segment at C 23-24, so the `#ifndef`s have no second
   copy to make and the two comments below record C's text.  */
/// C: 1203-1204: `void` / `ftfont_fix_match (FcPattern *pat, FcPattern *match)`.
#[cfg (feature = "have_freetype")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ftfont_fix_match (pat: *mut FcPattern, match_: *mut FcPattern)
{
  /*  These values are not used for matching (except antialias), but for
      rendering, so make sure they are carried over to the match.
      We also put antialias here because most fonts are antialiased, so
      the match will have antialias true.  */

  let mut b: FcBool = FcTrue;
  let mut i: c_int = 0;
  let mut dpi: c_double = 0.0;

  FcPatternGetBool (pat, FC_ANTIALIAS, 0, &raw mut b);
  if b == 0
    {
      FcPatternDel (match_, FC_ANTIALIAS);
      FcPatternAddBool (match_, FC_ANTIALIAS, FcFalse);
    }
  FcPatternGetBool (pat, FC_HINTING, 0, &raw mut b);
  if b == 0
    {
      FcPatternDel (match_, FC_HINTING);
      FcPatternAddBool (match_, FC_HINTING, FcFalse);
    }
  /* C 1227-1229: `#ifndef FC_HINT_STYLE` / `# define FC_HINT_STYLE "hintstyle"` / `#endif`.  */
  /* C 1230: `if (FcResultMatch == FcPatternGetInteger (pat, FC_HINT_STYLE, 0, &i))` */
  if FcResultMatch == FcPatternGetInteger (pat, FC_HINT_STYLE.as_ptr (), 0, &raw mut i)
    {
      FcPatternDel (match_, FC_HINT_STYLE.as_ptr ());
      FcPatternAddInteger (match_, FC_HINT_STYLE.as_ptr (), i);
    }
  /* C 1235-1238: `#ifndef FC_LCD_FILTER` ( `/* Older fontconfig versions don't have
     FC_LCD_FILTER. */` ) / `#define FC_LCD_FILTER "lcdfilter"` / `#endif`.  */
  if FcResultMatch == FcPatternGetInteger (pat, FC_LCD_FILTER.as_ptr (), 0, &raw mut i)
    {
      FcPatternDel (match_, FC_LCD_FILTER.as_ptr ());
      FcPatternAddInteger (match_, FC_LCD_FILTER.as_ptr (), i);
    }
  if FcResultMatch == FcPatternGetInteger (pat, FC_RGBA, 0, &raw mut i)
    {
      FcPatternDel (match_, FC_RGBA);
      FcPatternAddInteger (match_, FC_RGBA, i);
    }
  if FcResultMatch == FcPatternGetDouble (pat, FC_DPI, 0, &raw mut dpi)
    {
      FcPatternDel (match_, FC_DPI);
      FcPatternAddDouble (match_, FC_DPI, dpi);
    }
}

/* C 1256-1302: `ftfont_add_rendering_parameters` - the second `ftfont.h` entry point.  */
/// C: 1256-1257: `void` / `ftfont_add_rendering_parameters (FcPattern *pat, Lisp_Object entity)`.
#[cfg (feature = "have_freetype")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ftfont_add_rendering_parameters (pat: *mut FcPattern, entity: Lisp_Object)
{
  let mut tail: Lisp_Object;
  let mut ival: c_int = 0;

  tail = AREF (entity, FONT_EXTRA_INDEX as isize);
  while CONSP (tail)
    {
      let key: Lisp_Object = XCAR (XCAR (tail));
      let val: Lisp_Object = XCDR (XCAR (tail));

      if EQ (key, QCantialias ())
        { FcPatternAddBool (pat, FC_ANTIALIAS,
                            if ! NILP (val) { FcTrue } else { FcFalse }); }
      else if EQ (key, QChinting ())
        { FcPatternAddBool (pat, FC_HINTING,
                            if ! NILP (val) { FcTrue } else { FcFalse }); }
      else if EQ (key, QCautohint ())
        { FcPatternAddBool (pat, FC_AUTOHINT,
                            if ! NILP (val) { FcTrue } else { FcFalse }); }
      else if EQ (key, QChintstyle ())
        {
          if FIXNUMP (val)
            { FcPatternAddInteger (pat, FC_HINT_STYLE.as_ptr (), XFIXNUM (val) as c_int); }
          else if SYMBOLP (val)
              && FcNameConstant (SDATA (SYMBOL_NAME (val)), &raw mut ival) != 0
            { FcPatternAddInteger (pat, FC_HINT_STYLE.as_ptr (), ival); }
        }
      else if EQ (key, QCrgba ())
        {
          if FIXNUMP (val)
            { FcPatternAddInteger (pat, FC_RGBA, XFIXNUM (val) as c_int); }
          else if SYMBOLP (val)
              && FcNameConstant (SDATA (SYMBOL_NAME (val)), &raw mut ival) != 0
            { FcPatternAddInteger (pat, FC_RGBA, ival); }
        }
      else if EQ (key, QClcdfilter ())
        {
          if FIXNUMP (val)
            { ival = XFIXNUM (val) as c_int; FcPatternAddInteger (pat, FC_LCD_FILTER.as_ptr (), ival); }
          else if SYMBOLP (val)
              && FcNameConstant (SDATA (SYMBOL_NAME (val)), &raw mut ival) != 0
            { FcPatternAddInteger (pat, FC_LCD_FILTER.as_ptr (), ival); }
        }
      else
        {
          /* C 1297-1300: `#ifdef FC_EMBOLDEN` and its `QCembolden` arm.  Rust cannot put a
             `#[cfg]` on an `else if` clause, so the arm is the last `else`'s body; it is the
             same test, and C's chain already reaches it only when no earlier key matched.  */
          #[cfg (feature = "fc_embolden")]
          {
            if EQ (key, QCembolden ())
              { FcPatternAddBool (pat, FC_EMBOLDEN,
                                  if ! NILP (val) { FcTrue } else { FcFalse }); }
          }
        }
      tail = XCDR (tail);
    }
}

/* C 1304-1352: `ftfont_entity_pattern` - the third `ftfont.h` entry point, and this slice's last
   definition (C 1354's `#ifndef USE_CAIRO` and C 1357's `ftfont_open` open the next one).  */
/// C: 1304-1305: `FcPattern *` / `ftfont_entity_pattern (Lisp_Object entity, int pixel_size)`.
#[cfg (feature = "have_freetype")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ftfont_entity_pattern (entity: Lisp_Object, pixel_size: c_int)
  -> *mut FcPattern
{
  let mut val: Lisp_Object;
  let filename: Lisp_Object;
  let idx: Lisp_Object;
  let pat: *mut FcPattern;
  let mut i: c_int;

  val = assq_no_quit (QCfont_entity (), AREF (entity, FONT_EXTRA_INDEX as isize));
  eassert! (CONSP (val));
  val = XCDR (val);
  filename = XCAR (val);
  idx = XCDR (val);
  pat = FcPatternCreate ();
  FcPatternAddInteger (pat, FC_WEIGHT, FONT_WEIGHT_NUMERIC! (entity) as c_int);
  i = FONT_SLANT_NUMERIC! (entity) as c_int - 100;
  if i < 0 { i = 0; }
  FcPatternAddInteger (pat, FC_SLANT, i);
  FcPatternAddInteger (pat, FC_WIDTH, FONT_WIDTH_NUMERIC! (entity) as c_int);
  FcPatternAddDouble (pat, FC_PIXEL_SIZE, pixel_size as c_double);
  val = AREF (entity, FONT_FAMILY_INDEX as isize);
  if ! NILP (val)
    { FcPatternAddString (pat, FC_FAMILY, SDATA (SYMBOL_NAME (val)) as *const FcChar8); }
  val = AREF (entity, FONT_FOUNDRY_INDEX as isize);
  if ! NILP (val)
    { FcPatternAddString (pat, FC_FOUNDRY, SDATA (SYMBOL_NAME (val)) as *const FcChar8); }
  val = AREF (entity, FONT_SPACING_INDEX as isize);
  if ! NILP (val)
    { FcPatternAddInteger (pat, FC_SPACING, XFIXNUM (val) as c_int); }
  val = AREF (entity, FONT_DPI_INDEX as isize);
  if ! NILP (val)
    {
      let dbl: c_double = XFIXNUM (val) as c_double;

      FcPatternAddDouble (pat, FC_DPI, dbl);
    }
  val = AREF (entity, FONT_AVGWIDTH_INDEX as isize);
  if FIXNUMP (val) && XFIXNUM (val) == 0
    { FcPatternAddBool (pat, FC_SCALABLE, FcTrue); }
  /* This is necessary to identify the exact font (e.g. 10x20.pcf.gz
     over 10x20-ISO8859-1.pcf.gz).  */
  FcPatternAddCharSet (pat, FC_CHARSET, ftfont_get_fc_charset (entity));

  ftfont_add_rendering_parameters (pat, entity);

  FcPatternAddString (pat, FC_FILE, SDATA (filename) as *const FcChar8);
  FcPatternAddInteger (pat, FC_INDEX, XFIXNUM (idx) as c_int);

  pat
}
