// femacs: port of src/w32font.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `w32font.o`
// sits in `W32_OBJ` (`emacs/src/Makefile.in` 316-319, `W32_OBJ=@W32_OBJ@`, filled by
// `configure.ac` 3030 under the `HAVE_W32` test whose 3024 `AC_DEFINE`s `HAVE_NTGUI`), and
// `docs/baseline/frozen-build-sources.txt` lists no `w32font.o` for this target - so
// `extract-abi.py`'s checklist for the file is empty (P-86 (2)) and the ledger
// `docs/ports/w32font.md` carries that clause's substitute: the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "have_ntgui"`, on every item *and on the include graph*,
//     which is why the frozen configuration compiles nothing here.  `HAVE_NTGUI` is the macro the
//     Windows GUI build defines; it is *not* `WINDOWSNT` (the Cygwin GUI build compiles this file
//     with `HAVE_NTGUI` and without `WINDOWSNT`, and C 31/105/3073 gate on the latter), so the
//     file's own `#ifdef WINDOWSNT` is carried as `feature = "windowsnt"` inside the file gate;
//   * C 42/45/51/54's `#ifndef CLEARTYPE_QUALITY` / `CLEARTYPE_NATURAL_QUALITY` /
//     `VIETNAMESE_CHARSET` / `JOHAB_CHARSET` are "define it if the SDK has not" guards: the frozen
//     table has none of the four, so C's own `#define` is the arm carried, gated
//     `not (feature = "<name>")`; the SDK arm (the same values, `wingdi.h`'s) is carried too
//     (P-86 (1)'s both arms);
//   * C 105-230 is the file's `#ifdef WINDOWSNT` / `#else /* Cygwin */` pair: the four wide-API
//     wrapper functions are the `windowsnt` arm, the four `#define`s that call the SDK directly
//     are the `not (feature = "windowsnt")` arm - both carried;
//   * C 371-401 is an `#if 0` region: C's own preprocessor never compiles it, so the port keeps
//     C's text inside a comment (the `cm.rs` 86 reading);
//   * C 1583-1585 is `#ifndef WINDOWSNT` / `#define _strlwr strlwr` / `#endif`: the Cygwin arm
//     defines it, the MinGW arm takes ms-w32.h's.
//
// **Acquisition note - this piece is `text-only` (P-86 (3)(ii)).**  The machine has no Windows and
// no mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty,
// no Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`), so there is no
// `<windows.h>` to compile or measure against: **every record and every value of the stand-in
// sections below is `UNVERIFIED`** (the documented Windows ones, not measurements) and the layout
// assertions are owed.  What the round does show is that the `have_ntgui` + `windowsnt` arm
// type-checks under this machine's `rustc` - the ledger's `## Evidence` carries that reading and
// names the errors that belong to the include graph rather than to this unit.
//
// `docs/ports/w32font.md` carries the C line ranges, the acceptance readings, every deviation (the
// dropped forward declarations, the `<windows.h>`/`<commdlg.h>` stand-in sections, the in-unit
// declarations no mirror carries, the `#if 0` text) and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, unused_must_use,
         let_underscore_drop, clippy::all)]

/* C 1-17 is the file's opening ownership/title block, dropped (PORTING.md 0.1-19); the port's
   first line is the revision line above.  C 19-36 is the include graph and C 38-56 the SDK macro
   guards.  */

include!("include/config.rs");                  // C 19: `#include <config.h>`

/* ----------------------------------------------------------------------------------------------
   C 20: `#include <windows.h>`.

   No file in the reference tree carries `<windows.h>` (P-88 (1): the declaration site is C's own
   `#include` line, and no file is added under `femacs/src/include/`), so the names *this file*
   takes from it stand here, each with the SDK's own text above it.  The spellings repeat
   `src/w32term.rs`'s and `src/w32select.rs`'s sections (`DWORD` is the 32-bit Windows unsigned
   whatever this target's `c_long` is, `HANDLE`/`HDC`/`HFONT`/`HMODULE` are `void *`, `BOOL` is
   C's `int`, `WINAPI`/`CALLBACK` are `extern "system"`), so that a Windows translation unit sees
   one set of names.

   **No measurement was possible**: this machine has no `<windows.h>` (see the file note), so every
   scalar is spelled from PORTING.md 1.1's fixed Windows widths and every record below is
   **UNVERIFIED** - its size, offsets and values are the documented ones and the layout assertions
   are owed (the ledger's `## Owed`).
   ---------------------------------------------------------------------------------------------- */

/// C 20's `<windows.h>`: the scalar types (`windef.h`/`winnt.h`).
/// C: typedef unsigned char BYTE; typedef unsigned short WORD; typedef unsigned short WCHAR;
/// C: typedef unsigned long DWORD; typedef long LONG; typedef unsigned int UINT;
/// C: typedef int BOOL; typedef unsigned long ULONG;
#[cfg (feature = "have_ntgui")]
pub type BYTE = u8;
#[cfg (feature = "have_ntgui")]
pub type WORD = c_ushort;
#[cfg (feature = "have_ntgui")]
pub type WCHAR = c_ushort;
#[cfg (feature = "have_ntgui")]
pub type DWORD = u32;
#[cfg (feature = "have_ntgui")]
pub type LONG = i32;
#[cfg (feature = "have_ntgui")]
pub type ULONG = u32;
#[cfg (feature = "have_ntgui")]
pub type UINT = c_uint;
#[cfg (feature = "have_ntgui")]
pub type BOOL = c_int;
/// C's handle names are all `void *` at the ABI (the `include/w32gui.rs` reading); `HGDIOBJ` is
/// the GDI object handle `SelectObject` returns and `DeleteObject` takes, so `HFONT`/`HBRUSH`/
/// `HRGN` are aliases of it and C's implicit conversions at the assignment sites need no cast.
/// C: typedef void *HANDLE; typedef void *HWND; typedef void *HDC; typedef void *HFONT;
/// C: typedef void *HBRUSH; typedef void *HRGN; typedef void *HMODULE; typedef void *HGDIOBJ;
/// C: typedef void *LPVOID; typedef DWORD COLORREF;
#[cfg (feature = "have_ntgui")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HDC = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HGDIOBJ = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HFONT = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HBRUSH = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HRGN = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HMODULE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type COLORREF = DWORD;
/// C's `WINAPI` is `__stdcall`, which Rust spells `extern "system"` (the `src/w32fns.rs`
/// convention); the note stays on the declarations below.  The two `FALSE`/`TRUE` names are
/// `<windef.h>`'s, carried with C's values and types.
/// C: #define WINAPI __stdcall
/// C: #define FALSE 0
/// C: #define TRUE 1
#[cfg (feature = "have_ntgui")]
pub const FALSE: BOOL = 0;
#[cfg (feature = "have_ntgui")]
pub const TRUE: BOOL = 1;

/// C 71/74/77's callback parameter and C 148/168/186/205's out parameters name these pointer
/// spellings.
/// C: typedef UINT_PTR WPARAM; typedef LONG_PTR LPARAM;
/// C: typedef OUTLINETEXTMETRICW *LPOUTLINETEXTMETRICW; typedef TEXTMETRICW *LPTEXTMETRICW;
/// C: typedef GLYPHMETRICS *LPGLYPHMETRICS; typedef INT *LPINT;
#[cfg (feature = "have_ntgui")]
pub type LPARAM = isize;
#[cfg (feature = "have_ntgui")]
pub type LPOUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
#[cfg (feature = "have_ntgui")]
pub type LPTEXTMETRICW = *mut TEXTMETRICW;
#[cfg (feature = "have_ntgui")]
pub type LPGLYPHMETRICS = *mut GLYPHMETRICS;
#[cfg (feature = "have_ntgui")]
pub type LPINT = *mut c_int;

/// C 20's `<windows.h>`: `tagPOINT`/`tagRECT`/`SIZE` (`windef.h`), `LF_FACESIZE`/`LF_FULLFACESIZE`
/// (`wingdi.h`) and the `POINTL`/`FIXED` pair the `MAT2`/`GLYPHMETRICS` records below name.
/// C: typedef struct tagPOINT { LONG x; LONG y; } POINT;
/// C: typedef struct tagSIZE { LONG cx; LONG cy; } SIZE;
/// C: typedef struct tagRECT { LONG left; LONG top; LONG right; LONG bottom; } RECT;
/// C: #define LF_FACESIZE 32
/// C: #define LF_FULLFACESIZE 64
/// C: typedef struct _FIXED { WORD fract; short value; } FIXED;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POINT
{
  pub x: LONG,
  pub y: LONG,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIZE
{
  pub cx: LONG,
  pub cy: LONG,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RECT
{
  pub left: LONG,
  pub top: LONG,
  pub right: LONG,
  pub bottom: LONG,
}
#[cfg (feature = "have_ntgui")]
pub const LF_FACESIZE: c_int = 32;
#[cfg (feature = "have_ntgui")]
pub const LF_FULLFACESIZE: c_int = 64;
/// C: typedef struct _FIXED { WORD fract; short value; } FIXED;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FIXED
{
  pub fract: WORD,
  pub value: c_short,
}

/// C 20's `<windows.h>`: `tagLOGFONTA` (`wingdi.h`), the record `fill_in_logfont` fills and
/// `CreateFontIndirect` takes.  `lfFaceName` is `CHAR[LF_FACESIZE]` (the A variant, which C 293's
/// `strncpy`/`sprintf` and `check_face_name`'s `xstrcasecmp` read as `char`).
/// C: typedef struct tagLOGFONTA { LONG lfHeight; LONG lfWidth; LONG lfEscapement;
/// C:   LONG lfOrientation; LONG lfWeight; BYTE lfItalic; BYTE lfUnderline; BYTE lfStrikeOut;
/// C:   BYTE lfCharSet; BYTE lfOutPrecision; BYTE lfClipPrecision; BYTE lfQuality;
/// C:   BYTE lfPitchAndFamily; CHAR lfFaceName[LF_FACESIZE]; } LOGFONTA;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct LOGFONT
{
  pub lfHeight: LONG,
  pub lfWidth: LONG,
  pub lfEscapement: LONG,
  pub lfOrientation: LONG,
  pub lfWeight: LONG,
  pub lfItalic: BYTE,
  pub lfUnderline: BYTE,
  pub lfStrikeOut: BYTE,
  pub lfCharSet: BYTE,
  pub lfOutPrecision: BYTE,
  pub lfClipPrecision: BYTE,
  pub lfQuality: BYTE,
  pub lfPitchAndFamily: BYTE,
  pub lfFaceName: [c_char; LF_FACESIZE as usize],
}

/// C 20's `<windows.h>`: the metrics records (`wingdi.h`).  `w32font_info.metrics` is a
/// `TEXTMETRICW` by value (the `include/w32font.rs` mirror) and C 1127's `metrics->otmTextMetrics`
/// is the first field of `OUTLINETEXTMETRICW`.
/// C: typedef struct tagTEXTMETRICW { LONG tmHeight; LONG tmAscent; LONG tmDescent;
/// C:   LONG tmInternalLeading; LONG tmExternalLeading; LONG tmAveCharWidth; LONG tmMaxCharWidth;
/// C:   LONG tmWeight; LONG tmOverhang; LONG tmDigitizedAspectX; LONG tmDigitizedAspectY;
/// C:   WCHAR tmFirstChar; WCHAR tmLastChar; WCHAR tmDefaultChar; WCHAR tmBreakChar;
/// C:   BYTE tmItalic; BYTE tmUnderlined; BYTE tmStruckOut; BYTE tmPitchAndFamily;
/// C:   BYTE tmCharSet; } TEXTMETRICW;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct TEXTMETRICW
{
  pub tmHeight: LONG,
  pub tmAscent: LONG,
  pub tmDescent: LONG,
  pub tmInternalLeading: LONG,
  pub tmExternalLeading: LONG,
  pub tmAveCharWidth: LONG,
  pub tmMaxCharWidth: LONG,
  pub tmWeight: LONG,
  pub tmOverhang: LONG,
  pub tmDigitizedAspectX: LONG,
  pub tmDigitizedAspectY: LONG,
  pub tmFirstChar: WCHAR,
  pub tmLastChar: WCHAR,
  pub tmDefaultChar: WCHAR,
  pub tmBreakChar: WCHAR,
  pub tmItalic: BYTE,
  pub tmUnderlined: BYTE,
  pub tmStruckOut: BYTE,
  pub tmPitchAndFamily: BYTE,
  pub tmCharSet: BYTE,
}
/// C 20's `<windows.h>`: `tagNEWTEXTMETRICW` and its `EX` wrapper (`wingdi.h`); C 1187-1200 and
/// C 1233-1788 read `ntmTm` and `ntmFontSig`.
/// `NEWTEXTMETRICW` is `TEXTMETRICW`'s twenty fields **flattened** plus three, not a nested
/// `TEXTMETRICW` (`wingdi.h` repeats them), which is why C's `ntmTm.tmCharSet` and friends resolve
/// here.
/// C: typedef struct tagNEWTEXTMETRICW { LONG tmHeight; LONG tmAscent; LONG tmDescent;
/// C:   LONG tmInternalLeading; LONG tmExternalLeading; LONG tmAveCharWidth; LONG tmMaxCharWidth;
/// C:   LONG tmWeight; LONG tmOverhang; LONG tmDigitizedAspectX; LONG tmDigitizedAspectY;
/// C:   WCHAR tmFirstChar; WCHAR tmLastChar; WCHAR tmDefaultChar; WCHAR tmBreakChar;
/// C:   BYTE tmItalic; BYTE tmUnderlined; BYTE tmStruckOut; BYTE tmPitchAndFamily;
/// C:   BYTE tmCharSet; DWORD ntmFlags; UINT ntmCellHeight; UINT ntmAvgWidth; } NEWTEXTMETRICW;
/// C: typedef struct tagFONTSIGNATURE { DWORD fsUsb[4]; DWORD fsCsb[2]; } FONTSIGNATURE;
/// C: typedef struct tagNEWTEXTMETRICEXW { NEWTEXTMETRICW ntmTm; FONTSIGNATURE ntmFontSig; }
/// C:   NEWTEXTMETRICEXW;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct NEWTEXTMETRICW
{
  pub tmHeight: LONG,
  pub tmAscent: LONG,
  pub tmDescent: LONG,
  pub tmInternalLeading: LONG,
  pub tmExternalLeading: LONG,
  pub tmAveCharWidth: LONG,
  pub tmMaxCharWidth: LONG,
  pub tmWeight: LONG,
  pub tmOverhang: LONG,
  pub tmDigitizedAspectX: LONG,
  pub tmDigitizedAspectY: LONG,
  pub tmFirstChar: WCHAR,
  pub tmLastChar: WCHAR,
  pub tmDefaultChar: WCHAR,
  pub tmBreakChar: WCHAR,
  pub tmItalic: BYTE,
  pub tmUnderlined: BYTE,
  pub tmStruckOut: BYTE,
  pub tmPitchAndFamily: BYTE,
  pub tmCharSet: BYTE,
  pub ntmFlags: DWORD,
  pub ntmCellHeight: UINT,
  pub ntmAvgWidth: UINT,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FONTSIGNATURE
{
  pub fsUsb: [DWORD; 4],
  pub fsCsb: [DWORD; 2],
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct NEWTEXTMETRICEX
{
  pub ntmTm: NEWTEXTMETRICW,
  pub ntmFontSig: FONTSIGNATURE,
}
/// C 20's `<windows.h>`: `tagENUMLOGFONTEXW` (`wingdi.h`), the callback's first parameter.
/// `elfLogFont` is a `LOGFONTA` in this translation unit's SDK (`ENUMLOGFONTEX` is the A variant
/// where `UNICODE` is not defined, the same variant `GetTextFace` below is compiled in).
/// C: typedef struct tagENUMLOGFONTEXA { LOGFONTA elfLogFont; CHAR elfFullName[LF_FULLFACESIZE];
/// C:   CHAR elfStyle[LF_FACESIZE]; CHAR elfScript[LF_FACESIZE]; } ENUMLOGFONTEXA;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct ENUMLOGFONTEX
{
  pub elfLogFont: LOGFONT,
  pub elfFullName: [c_char; LF_FULLFACESIZE as usize],
  pub elfStyle: [c_char; LF_FACESIZE as usize],
  pub elfScript: [c_char; LF_FACESIZE as usize],
}
/// C 20's `<windows.h>`: the glyph-metrics pair `compute_metrics` fills (`wingdi.h`).
/// C: typedef struct _GLYPHMETRICS { UINT gmBlackBoxX; UINT gmBlackBoxY;
/// C:   POINTL gmptGlyphOrigin; short gmCellIncX; short gmCellIncY; } GLYPHMETRICS;
/// C: typedef struct _MAT2 { FIXED eM11; FIXED eM12; FIXED eM21; FIXED eM22; } MAT2;
/// C: typedef struct tagPOINTL { LONG x; LONG y; } POINTL;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POINTL
{
  pub x: LONG,
  pub y: LONG,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GLYPHMETRICS
{
  pub gmBlackBoxX: UINT,
  pub gmBlackBoxY: UINT,
  pub gmptGlyphOrigin: POINTL,
  pub gmCellIncX: c_short,
  pub gmCellIncY: c_short,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct MAT2
{
  pub eM11: FIXED,
  pub eM12: FIXED,
  pub eM21: FIXED,
  pub eM22: FIXED,
}

/// C 20's `<windows.h>`: the charset/quality/precision/pitch/weight values `w32font.c` compares
/// against (all `wingdi.h`).  The four names C 42-56 guards are given C's own `#define` values
/// (the SDK's are the same numbers).
/// C: #define ANSI_CHARSET 0 / #define DEFAULT_CHARSET 1 / #define SYMBOL_CHARSET 2
/// C: #define MAC_CHARSET 77 / #define SHIFTJIS_CHARSET 128 / #define HANGEUL_CHARSET 129
/// C: #define JOHAB_CHARSET 130 / #define GB2312_CHARSET 134 / #define CHINESEBIG5_CHARSET 136
/// C: #define GREEK_CHARSET 161 / #define TURKISH_CHARSET 162 / #define VIETNAMESE_CHARSET 163
/// C: #define HEBREW_CHARSET 177 / #define ARABIC_CHARSET 178 / #define BALTIC_CHARSET 186
/// C: #define RUSSIAN_CHARSET 204 / #define THAI_CHARSET 222 / #define EASTEUROPE_CHARSET 238
/// C: #define OEM_CHARSET 255
#[cfg (feature = "have_ntgui")]
pub const ANSI_CHARSET: BYTE = 0;
#[cfg (feature = "have_ntgui")]
pub const DEFAULT_CHARSET: BYTE = 1;
#[cfg (feature = "have_ntgui")]
pub const SYMBOL_CHARSET: BYTE = 2;
#[cfg (feature = "have_ntgui")]
pub const MAC_CHARSET: BYTE = 77;
#[cfg (feature = "have_ntgui")]
pub const SHIFTJIS_CHARSET: BYTE = 128;
#[cfg (feature = "have_ntgui")]
pub const HANGEUL_CHARSET: BYTE = 129;
#[cfg (feature = "have_ntgui")]
pub const HANGUL_CHARSET: BYTE = 129;
#[cfg (all (feature = "have_ntgui", not (feature = "johab_charset")))]
pub const JOHAB_CHARSET: BYTE = 130;
#[cfg (all (feature = "have_ntgui", feature = "johab_charset"))]
pub const JOHAB_CHARSET: BYTE = 130;
#[cfg (feature = "have_ntgui")]
pub const GB2312_CHARSET: BYTE = 134;
#[cfg (feature = "have_ntgui")]
pub const CHINESEBIG5_CHARSET: BYTE = 136;
#[cfg (feature = "have_ntgui")]
pub const GREEK_CHARSET: BYTE = 161;
#[cfg (feature = "have_ntgui")]
pub const TURKISH_CHARSET: BYTE = 162;
#[cfg (all (feature = "have_ntgui", not (feature = "vietnamese_charset")))]
pub const VIETNAMESE_CHARSET: BYTE = 163;
#[cfg (all (feature = "have_ntgui", feature = "vietnamese_charset"))]
pub const VIETNAMESE_CHARSET: BYTE = 163;
#[cfg (feature = "have_ntgui")]
pub const HEBREW_CHARSET: BYTE = 177;
#[cfg (feature = "have_ntgui")]
pub const ARABIC_CHARSET: BYTE = 178;
#[cfg (feature = "have_ntgui")]
pub const BALTIC_CHARSET: BYTE = 186;
#[cfg (feature = "have_ntgui")]
pub const RUSSIAN_CHARSET: BYTE = 204;
#[cfg (feature = "have_ntgui")]
pub const THAI_CHARSET: BYTE = 222;
#[cfg (feature = "have_ntgui")]
pub const EASTEUROPE_CHARSET: BYTE = 238;
#[cfg (feature = "have_ntgui")]
pub const OEM_CHARSET: BYTE = 255;
/// C: #define DEFAULT_QUALITY 0 / #define NONANTIALIASED_QUALITY 3 / #define ANTIALIASED_QUALITY 4
/// C: #define CLEARTYPE_QUALITY 5 / #define CLEARTYPE_NATURAL_QUALITY 6
#[cfg (feature = "have_ntgui")]
pub const DEFAULT_QUALITY: BYTE = 0;
#[cfg (feature = "have_ntgui")]
pub const NONANTIALIASED_QUALITY: BYTE = 3;
#[cfg (feature = "have_ntgui")]
pub const ANTIALIASED_QUALITY: BYTE = 4;
#[cfg (all (feature = "have_ntgui", not (feature = "cleartype_quality")))]
pub const CLEARTYPE_QUALITY: BYTE = 5;
#[cfg (all (feature = "have_ntgui", feature = "cleartype_quality"))]
pub const CLEARTYPE_QUALITY: BYTE = 5;
#[cfg (all (feature = "have_ntgui", not (feature = "cleartype_natural_quality")))]
pub const CLEARTYPE_NATURAL_QUALITY: BYTE = 6;
#[cfg (all (feature = "have_ntgui", feature = "cleartype_natural_quality"))]
pub const CLEARTYPE_NATURAL_QUALITY: BYTE = 6;
/// C: #define OUT_DEFAULT_PRECIS 0 / #define OUT_STRING_PRECIS 1 / #define OUT_STROKE_PRECIS 3
/// C: #define OUT_TT_PRECIS 4 / #define OUT_OUTLINE_PRECIS 8
#[cfg (feature = "have_ntgui")]
pub const OUT_STRING_PRECIS: BYTE = 1;
#[cfg (feature = "have_ntgui")]
pub const OUT_STROKE_PRECIS: BYTE = 3;
#[cfg (feature = "have_ntgui")]
pub const OUT_TT_PRECIS: BYTE = 4;
#[cfg (feature = "have_ntgui")]
pub const OUT_OUTLINE_PRECIS: BYTE = 8;
/// C: #define FF_DONTCARE 0 / #define FF_ROMAN 16 / #define FF_SWISS 32 / #define FF_MODERN 48
/// C: #define FF_SCRIPT 64 / #define FF_DECORATIVE 80
/// C: #define DEFAULT_PITCH 0 / #define FIXED_PITCH 1 / #define VARIABLE_PITCH 2
#[cfg (feature = "have_ntgui")]
pub const FF_DONTCARE: BYTE = 0;
#[cfg (feature = "have_ntgui")]
pub const FF_ROMAN: BYTE = 16;
#[cfg (feature = "have_ntgui")]
pub const FF_SWISS: BYTE = 32;
#[cfg (feature = "have_ntgui")]
pub const FF_MODERN: BYTE = 48;
#[cfg (feature = "have_ntgui")]
pub const FF_SCRIPT: BYTE = 64;
#[cfg (feature = "have_ntgui")]
pub const FF_DECORATIVE: BYTE = 80;
#[cfg (feature = "have_ntgui")]
pub const DEFAULT_PITCH: BYTE = 0;
#[cfg (feature = "have_ntgui")]
pub const FIXED_PITCH: BYTE = 1;
#[cfg (feature = "have_ntgui")]
pub const VARIABLE_PITCH: BYTE = 2;
/// C: #define FW_THIN 100 / #define FW_EXTRALIGHT 200 / #define FW_LIGHT 300 / #define FW_NORMAL 400
/// C: #define FW_MEDIUM 500 / #define FW_SEMIBOLD 600 / #define FW_BOLD 700
/// C: #define FW_EXTRABOLD 800 / #define FW_HEAVY 900
#[cfg (feature = "have_ntgui")]
pub const FW_THIN: c_int = 100;
#[cfg (feature = "have_ntgui")]
pub const FW_EXTRALIGHT: c_int = 200;
#[cfg (feature = "have_ntgui")]
pub const FW_LIGHT: c_int = 300;
#[cfg (feature = "have_ntgui")]
pub const FW_NORMAL: c_int = 400;
#[cfg (feature = "have_ntgui")]
pub const FW_MEDIUM: c_int = 500;
#[cfg (feature = "have_ntgui")]
pub const FW_SEMIBOLD: c_int = 600;
#[cfg (feature = "have_ntgui")]
pub const FW_BOLD: c_int = 700;
#[cfg (feature = "have_ntgui")]
pub const FW_EXTRABOLD: c_int = 800;
#[cfg (feature = "have_ntgui")]
pub const FW_HEAVY: c_int = 900;
/// C: #define RASTER_FONTTYPE 0x0001 / #define DEVICE_FONTTYPE 0x0002 / #define TRUETYPE_FONTTYPE
/// C: 0x0004
#[cfg (feature = "have_ntgui")]
pub const RASTER_FONTTYPE: DWORD = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const DEVICE_FONTTYPE: DWORD = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const TRUETYPE_FONTTYPE: DWORD = 0x0004;
/// C: #define GDI_ERROR 0xFFFFFFFF
/// C: #define GGO_METRICS 0 / #define GGO_GLYPH_INDEX 0x0080
/// C: #define TRANSPARENT 1 / #define OPAQUE 2
/// C: #define RGN_OR 2
/// C: #define DT_CALCRECT 0x0400 / #define DT_NOPREFIX 0x0800 / #define DT_SINGLELINE 0x0020
#[cfg (feature = "have_ntgui")]
pub const GDI_ERROR: DWORD = 0xFFFFFFFF;
#[cfg (feature = "have_ntgui")]
pub const GGO_METRICS: UINT = 0;
#[cfg (feature = "have_ntgui")]
pub const GGO_GLYPH_INDEX: UINT = 0x0080;
#[cfg (feature = "have_ntgui")]
pub const TRANSPARENT: c_int = 1;
#[cfg (feature = "have_ntgui")]
pub const RGN_OR: c_int = 2;
#[cfg (feature = "have_ntgui")]
pub const DT_CALCRECT: UINT = 0x0400;
#[cfg (feature = "have_ntgui")]
pub const DT_NOPREFIX: UINT = 0x0800;
#[cfg (feature = "have_ntgui")]
pub const DT_SINGLELINE: UINT = 0x0020;

/// C 20's `<windows.h>`: the entry points this file calls.  Each is the SDK's A/W alias; C's text
/// writes the name and the port keeps it, so a Windows target supplies whichever entry point its
/// own `#define` picks (PORTING.md 0.1's rename convention).
/// C: HFONT WINAPI CreateFontIndirectA (const LOGFONTA *);
/// C: BOOL WINAPI DeleteObject (HGDIOBJ);
/// C: HGDIOBJ WINAPI SelectObject (HDC, HGDIOBJ);
/// C: BOOL WINAPI GetTextMetricsA (HDC, LPTEXTMETRICA);
/// C: UINT WINAPI GetOutlineTextMetricsW (HDC, UINT, LPOUTLINETEXTMETRICW);
/// C: DWORD WINAPI GetGlyphOutlineW (HDC, UINT, UINT, LPGLYPHMETRICS, DWORD, LPVOID, const MAT2 *);
/// C: BOOL WINAPI GetCharWidth32W (HDC, UINT, UINT, LPINT);
/// C: int WINAPI EnumFontFamiliesExW (HDC, LPLOGFONT, FONTENUMPROCW, LPARAM, DWORD);
/// C: HDC WINAPI GetDC (HWND);
/// C: int WINAPI ReleaseDC (HWND, HDC);
/// C: int WINAPI GetTextFaceA (HDC, int, LPSTR);
/// C: HRGN WINAPI CreateRectRgnIndirect (const RECT *);
/// C: HRGN WINAPI CreateRectRgn (int, int, int, int);
/// C: int WINAPI GetClipRgn (HDC, HRGN);
/// C: int WINAPI CombineRgn (HRGN, HRGN, HRGN, int);
/// C: int WINAPI SelectClipRgn (HDC, HRGN);
/// C: int WINAPI SetBkMode (HDC, int);
/// C: HBRUSH WINAPI CreateSolidBrush (COLORREF);
/// C: int WINAPI FillRect (HDC, const RECT *, HBRUSH);
/// C: COLORREF WINAPI GetTextColor (HDC);
/// C: BOOL WINAPI ExtTextOutW (HDC, int, int, UINT, const RECT *, const WCHAR *, UINT, const INT *);
/// C: BOOL WINAPI GetTextExtentPoint32W (HDC, const WCHAR *, int, LPSIZE);
/// C: int WINAPI DrawTextW (HDC, const WCHAR *, int, LPRECT, UINT);
/// C: BOOL WINAPI IsValidCodePage (UINT);
#[cfg (feature = "have_ntgui")]
pub type LPSIZE = *mut SIZE;
#[cfg (feature = "have_ntgui")]
pub type LPRECT = *mut RECT;
#[cfg (feature = "have_ntgui")]
pub type FONTENUMPROC = Option<unsafe extern "system" fn (*mut ENUMLOGFONTEX,
                                                       *mut NEWTEXTMETRICEX, DWORD, LPARAM)
                                                      -> c_int>;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn CreateFontIndirect (lplf: *const LOGFONT) -> HFONT;
  pub fn DeleteObject (ho: HGDIOBJ) -> BOOL;
  pub fn SelectObject (hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
  pub fn GetTextMetrics (hdc: HDC, lptm: *mut TEXTMETRIC) -> BOOL;
  pub fn GetTextMetricsW (hdc: HDC, lptm: LPTEXTMETRICW) -> BOOL;
  pub fn GetOutlineTextMetricsW (hdc: HDC, cbData: UINT, lpotm: LPOUTLINETEXTMETRICW) -> UINT;
  pub fn GetGlyphOutlineW (hdc: HDC, uChar: UINT, fuFormat: UINT, lpgm: LPGLYPHMETRICS,
                           cjBuffer: DWORD, pvBuffer: LPVOID, lpmat2: *const MAT2) -> DWORD;
  pub fn GetCharWidth32W (hdc: HDC, iFirst: UINT, iLast: UINT, lpBuffer: LPINT) -> BOOL;
  pub fn EnumFontFamiliesEx (hdc: HDC, lpLogfont: *mut LOGFONT, lpProc: FONTENUMPROC,
                             lParam: LPARAM, dwFlags: DWORD) -> c_int;
  pub fn GetDC (hWnd: *mut c_void) -> HDC;
  pub fn ReleaseDC (hWnd: *mut c_void, hDC: HDC) -> c_int;
  pub fn GetTextFace (hdc: HDC, c: c_int, lpName: *mut c_char) -> c_int;
  pub fn CreateRectRgnIndirect (lprect: *const RECT) -> HRGN;
  pub fn CreateRectRgn (x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> HRGN;
  pub fn GetClipRgn (hdc: HDC, hrgn: HRGN) -> c_int;
  pub fn CombineRgn (hrgnDst: HRGN, hrgnSrc1: HRGN, hrgnSrc2: HRGN, iMode: c_int) -> c_int;
  pub fn SelectClipRgn (hdc: HDC, hrgn: HRGN) -> c_int;
  pub fn SetBkMode (hdc: HDC, mode: c_int) -> c_int;
  pub fn CreateSolidBrush (color: COLORREF) -> HBRUSH;
  pub fn FillRect (hdc: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
  pub fn GetTextColor (hdc: HDC) -> COLORREF;
  pub fn ExtTextOutW (hdc: HDC, x: c_int, y: c_int, options: UINT, lprect: *const RECT,
                      lpString: *const WCHAR, c: UINT, lpDx: *const c_int) -> BOOL;
  pub fn GetTextExtentPoint32W (hdc: HDC, lpString: *const WCHAR, c: c_int, psizl: LPSIZE)
                                -> BOOL;
  pub fn DrawTextW (hdc: HDC, lpchText: *const WCHAR, cchText: c_int, lprc: LPRECT,
                    format: UINT) -> c_int;
  pub fn IsValidCodePage (CodePage: UINT) -> BOOL;
}
/// C 2759-2767's `GetTextMetrics`/`GetTextFace` pair takes C's `TEXTMETRIC` (the A variant, since
/// this translation unit's `ENUMLOGFONTEX`/`LOGFONT` are the A ones - C 1247's `strncmp` and
/// C 1590's `xstrcasecmp` read `lfFaceName` as `char`), and `get_text_metrics_w` reaches the W
/// variant through the SDK's own `GetTextMetricsW` (C 168).
/// C: typedef struct tagTEXTMETRICA { LONG tmHeight; LONG tmAscent; LONG tmDescent;
/// C:   LONG tmInternalLeading; LONG tmExternalLeading; LONG tmAveCharWidth; LONG tmMaxCharWidth;
/// C:   LONG tmWeight; LONG tmOverhang; LONG tmDigitizedAspectX; LONG tmDigitizedAspectY;
/// C:   CHAR tmFirstChar; CHAR tmLastChar; CHAR tmDefaultChar; CHAR tmBreakChar; BYTE tmItalic;
/// C:   BYTE tmUnderlined; BYTE tmStruckOut; BYTE tmPitchAndFamily; BYTE tmCharSet; } TEXTMETRICA;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct TEXTMETRIC
{
  pub tmHeight: LONG,
  pub tmAscent: LONG,
  pub tmDescent: LONG,
  pub tmInternalLeading: LONG,
  pub tmExternalLeading: LONG,
  pub tmAveCharWidth: LONG,
  pub tmMaxCharWidth: LONG,
  pub tmWeight: LONG,
  pub tmOverhang: LONG,
  pub tmDigitizedAspectX: LONG,
  pub tmDigitizedAspectY: LONG,
  pub tmFirstChar: c_char,
  pub tmLastChar: c_char,
  pub tmDefaultChar: c_char,
  pub tmBreakChar: c_char,
  pub tmItalic: BYTE,
  pub tmUnderlined: BYTE,
  pub tmStruckOut: BYTE,
  pub tmPitchAndFamily: BYTE,
  pub tmCharSet: BYTE,
}

#[cfg (feature = "have_ntgui")]
include!("../lib/include/stdio.rs");           // C 21: `#include <stdio.h>` (`sprintf`)
#[cfg (feature = "have_ntgui")]
include!("../lib/include/math.rs");            // C 22: `#include <math.h>` (`floor`)
#[cfg (feature = "have_ntgui")]
include!("../lib/include/ctype.rs");           // C 23: `#include <ctype.h>`

/* ----------------------------------------------------------------------------------------------
   C 24: `#include <commdlg.h>`.

   No file in the reference tree carries it either (the P-88 (1) shape), so the names this file
   takes from it stand here.  C 2757-2817's `Fx_select_font` is the only user: `CHOOSEFONT`,
   `ChooseFont`, the five `CF_*` flags and `CF_INITTOLOGFONTSTRUCT`.
   ---------------------------------------------------------------------------------------------- */

/// C 24's `<commdlg.h>` (`commdlg.h`/`dlgs.h`).
/// C: typedef struct tagCHOOSEFONTA { DWORD lStructSize; HWND hwndOwner; HDC hDC; LPLOGFONTA
/// C:   lpLogFont; INT iPointSize; DWORD Flags; COLORREF rgbColors; LPARAM lCustData;
/// C:   LPCCHOOKPROC lpfnHook; LPCSTR lpTemplateName; HINSTANCE hInstance; LPSTR lpszStyle;
/// C:   WORD nFontType; WORD ___MISSING_ALIGNMENT__; INT nSizeMin; INT nSizeMax; } CHOOSEFONTA;
/// C: #define CF_SCREENFONTS 0x00000001 / #define CF_INITTOLOGFONTSTRUCT 0x00000040
/// C: #define CF_FIXEDPITCHONLY 0x00004000 / #define CF_FORCEFONTEXIST 0x00010000
/// C: #define CF_NOVERTFONTS 0x01000000
/// C: BOOL WINAPI ChooseFontA (LPCHOOSEFONTA);
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct CHOOSEFONT
{
  pub lStructSize: DWORD,
  pub hwndOwner: *mut c_void,
  pub hDC: HDC,
  pub lpLogFont: *mut LOGFONT,
  pub iPointSize: c_int,
  pub Flags: DWORD,
  pub rgbColors: COLORREF,
  pub lCustData: LPARAM,
  pub lpfnHook: *mut c_void,
  pub lpTemplateName: *const c_char,
  pub hInstance: *mut c_void,
  pub lpszStyle: *mut c_char,
  pub nFontType: WORD,
  pub ___MISSING_ALIGNMENT__: WORD,
  pub nSizeMin: c_int,
  pub nSizeMax: c_int,
}
#[cfg (feature = "have_ntgui")]
pub const CF_SCREENFONTS: DWORD = 0x00000001;
#[cfg (feature = "have_ntgui")]
pub const CF_INITTOLOGFONTSTRUCT: DWORD = 0x00000040;
#[cfg (feature = "have_ntgui")]
pub const CF_FIXEDPITCHONLY: DWORD = 0x00004000;
#[cfg (feature = "have_ntgui")]
pub const CF_FORCEFONTEXIST: DWORD = 0x00010000;
#[cfg (feature = "have_ntgui")]
pub const CF_NOVERTFONTS: DWORD = 0x01000000;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn ChooseFont (lpcf: *mut CHOOSEFONT) -> BOOL;
}

/// C 21-24: the C standard-library names those three headers give that no mirror in this graph
/// carries.  `<stdlib.h>`'s `atoi` (`x_to_w32_charset` C 1806/2050 and `w32_to_x_charset` C 1957)
/// and `<string.h>`'s `strstr` (C 1601/1607/1104) and `strncpy` (C 1594/1601/2312/2334) are the
/// four; the rest (`memset`, `memcpy`, `strlen`, `strcmp`, `strncmp`, `strcpy`, `strchr`,
/// `sprintf`, `floor`) come from `lib/include/{string,stdio,math}.rs`.
/// C: int atoi (const char *);
/// C: char *strstr (const char *, const char *);
/// C: char *strncpy (char *dest, const char *src, size_t n);
/// C: int strnicmp (const char *, const char *, size_t);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn atoi (nptr: *const c_char) -> c_int;
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
  pub fn strncpy (dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
  pub fn strnicmp (s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

#[cfg (feature = "have_ntgui")]
include!("include/lisp.rs");                    // C 26: `#include "lisp.h"`
#[cfg (feature = "have_ntgui")]
include!("include/w32term.rs");                 // C 27: `#include "w32term.h"`
#[cfg (feature = "have_ntgui")]
include!("include/frame.rs");                   // C 28: `#include "frame.h"`
#[cfg (feature = "have_ntgui")]
include!("include/coding.rs");                  // C 29: `#include "coding.h"`	/* for ENCODE_SYSTEM, DECODE_SYSTEM */
#[cfg (feature = "have_ntgui")]
include!("include/w32font.rs");                 // C 30: `#include "w32font.h"`
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
include!("include/w32common.rs");               // C 32: `#include "w32common.h"`
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
include!("include/w32.rs");                     // C 33: `#include "w32.h"`
#[cfg (feature = "have_ntgui")]
include!("include/pdumper.rs");                 // C 36: `#include "pdumper.h"`

// C 38-40: "Cleartype available on Windows XP, cleartype_natural from XP SP1" - the design note
//    is carried in C's own words where it stands:
//      /* Cleartype available on Windows XP, cleartype_natural from XP SP1.
//         The latter does not try to fit cleartype smoothed fonts into the
//         same bounding box as the non-antialiased version of the font.
//       */
//    C 42-56's four guards (`#ifndef CLEARTYPE_QUALITY` / `CLEARTYPE_NATURAL_QUALITY` /
//    `VIETNAMESE_CHARSET` / `JOHAB_CHARSET`, each a `#define` of 5 / 6 / 163 / 130) are carried by
//    the four constant pairs of the `<windows.h>` section above, where the SDK's own values live
//    under the same names; the guard text is C's:

//      #ifndef CLEARTYPE_QUALITY
//      #define CLEARTYPE_QUALITY 5
//      #endif
//      #ifndef CLEARTYPE_NATURAL_QUALITY
//      #define CLEARTYPE_NATURAL_QUALITY 6
//      #endif
//      /* VIETNAMESE_CHARSET and JOHAB_CHARSET are not defined in some versions
//         of MSVC headers.  */
//      #ifndef VIETNAMESE_CHARSET
//      #define VIETNAMESE_CHARSET 163
//      #endif
//      #ifndef JOHAB_CHARSET
//      #define JOHAB_CHARSET 130
//      #endif
//

/* C 58-77: the file's ten forward declarations are dropped (PORTING.md 0.1-12: a declaration
   plus a definition is `E0428`, and Rust items are order-independent).  C's own text:

     static void fill_in_logfont (struct frame *, LOGFONT *, Lisp_Object);
     static BYTE w32_antialias_type (Lisp_Object);
     static Lisp_Object lispy_antialias_type (BYTE);
     static Lisp_Object font_supported_scripts (FONTSIGNATURE *);
     static int w32font_full_name (LOGFONT *, Lisp_Object, int, char *, int);
     static void compute_metrics (HDC, struct w32font_info *, unsigned int,
                                  struct w32_metric_cache *);
     static Lisp_Object w32_registry (LONG, DWORD);
     static int CALLBACK ALIGN_STACK add_font_entity_to_list (ENUMLOGFONTEX *, NEWTEXTMETRICEX *,
                                                              DWORD, LPARAM);
     static int CALLBACK ALIGN_STACK add_one_font_entity_to_list (ENUMLOGFONTEX *,
                                                                  NEWTEXTMETRICEX *, DWORD,
                                                                  LPARAM);
     static int CALLBACK ALIGN_STACK add_font_name_to_list (ENUMLOGFONTEX *, NEWTEXTMETRICEX *,
                                                            DWORD, LPARAM);
   */

/* C 79-100: the record passed as the LPARAM argument to EnumFontFamiliesEx.  Every field is a C
   record or a `Lisp_Object`; the size is **UNVERIFIED** (no `<windows.h>` here) and the layout
   assertion is owed.  */
/// C: struct font_callback_data { LOGFONT pattern; Lisp_Object orig_font_spec; Lisp_Object frame;
/// C:   Lisp_Object known_fonts; Lisp_Object list; bool opentype_only; };
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct font_callback_data
{
  pub pattern: LOGFONT,
  pub orig_font_spec: Lisp_Object,
  pub frame: Lisp_Object,
  pub known_fonts: Lisp_Object,
  pub list: Lisp_Object,
  pub opentype_only: bool,
}

/* C 103: `static void list_all_matching_fonts (struct font_callback_data *);` - dropped with the
   block above (the definition is C 2289-2330).  */

/* C 105-218: the `#ifdef WINDOWSNT` arm - four `BOOL` initialisation flags, the four wide-API
   function-pointer typedefs, `w32_load_unicows_or_gdi32` and the four wrappers that call the
   "wide" entry points through `get_proc_addr`.  The `#else` Cygwin arm (C 220-230) is the
   four `#define`s that call the SDK directly; both arms are carried (P-86 (1)).  */
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
static mut g_b_init_get_outline_metrics_w: BOOL = 0;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
static mut g_b_init_get_text_metrics_w: BOOL = 0;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
static mut g_b_init_get_glyph_outline_w: BOOL = 0;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
static mut g_b_init_get_char_width_32_w: BOOL = 0;

/// C 112-130: the four `WINAPI` function-pointer types (`extern "system"`; a null one is
/// `Option<…> = None`).
/// C: typedef UINT (WINAPI * GetOutlineTextMetricsW_Proc) (HDC hdc, UINT cbData,
/// C:   LPOUTLINETEXTMETRICW lpotmw);
/// C: typedef BOOL (WINAPI * GetTextMetricsW_Proc) (HDC hdc, LPTEXTMETRICW lptmw);
/// C: typedef DWORD (WINAPI * GetGlyphOutlineW_Proc) (HDC hdc, UINT uChar, UINT uFormat,
/// C:   LPGLYPHMETRICS lpgm, DWORD cbBuffer, LPVOID lpvBuffer, const MAT2 *lpmat2);
/// C: typedef BOOL (WINAPI * GetCharWidth32W_Proc) (HDC hdc, UINT uFirstChar, UINT uLastChar,
/// C:   LPINT lpBuffer);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type GetOutlineTextMetricsW_Proc = unsafe extern "system" fn (HDC, UINT, LPOUTLINETEXTMETRICW)
                                                                  -> UINT;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type GetTextMetricsW_Proc = unsafe extern "system" fn (HDC, LPTEXTMETRICW) -> BOOL;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type GetGlyphOutlineW_Proc = unsafe extern "system" fn (HDC, UINT, UINT, LPGLYPHMETRICS,
                                                             DWORD, LPVOID, *const MAT2) -> DWORD;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type GetCharWidth32W_Proc = unsafe extern "system" fn (HDC, UINT, UINT, LPINT) -> BOOL;

/// C 141-148: `w32_load_unicows_or_gdi32` - "attempts to load UNICOWS.DLL on Windows 9X, and
/// refuses to let Emacs start up if that library is not found.  On NT and later versions, it
/// simply loads GDI32.DLL".
/// C: static HMODULE w32_load_unicows_or_gdi32 (void) { return maybe_load_unicows_dll (); }
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe fn w32_load_unicows_or_gdi32 () -> HMODULE
{
  return maybe_load_unicows_dll ();
}

/// C 150-165: `get_outline_metrics_w`.  C's `static GetOutlineTextMetricsW_Proc s_pfn… = NULL;`
/// is the port's `static mut … : Option<…> = None;` (A null pointer and `None` are the same
/// value), and C's cast `(GetOutlineTextMetricsW_Proc) get_proc_addr (…)` is the `transmute` the
/// two pointer types' equal width makes the same operation (the `src/w32proc.rs` 1258 reading).
/// C: static UINT WINAPI get_outline_metrics_w (HDC hdc, UINT cbData, LPOUTLINETEXTMETRICW lpotmw)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "system" fn get_outline_metrics_w (hdc: HDC, cbData: UINT,
                                                 lpotmw: LPOUTLINETEXTMETRICW) -> UINT
{
  static mut s_pfn_Get_Outline_Text_MetricsW: Option<GetOutlineTextMetricsW_Proc> = None;
  let mut hm_unicows: HMODULE = std::ptr::null_mut ();
  if g_b_init_get_outline_metrics_w == 0
    {
      g_b_init_get_outline_metrics_w = 1;
      hm_unicows = w32_load_unicows_or_gdi32 ();
      if ! hm_unicows.is_null ()
	{
	  s_pfn_Get_Outline_Text_MetricsW
	    = Some (std::mem::transmute::<VOIDFNPTR, GetOutlineTextMetricsW_Proc>
		      (get_proc_addr (hm_unicows, c"GetOutlineTextMetricsW".as_ptr ())));
	}
    }
  eassert! (s_pfn_Get_Outline_Text_MetricsW.is_some ());
  return (s_pfn_Get_Outline_Text_MetricsW.unwrap ()) (hdc, cbData, lpotmw);
}

/// C 167-181: `get_text_metrics_w`.
/// C: static BOOL WINAPI get_text_metrics_w (HDC hdc, LPTEXTMETRICW lptmw)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "system" fn get_text_metrics_w (hdc: HDC, lptmw: LPTEXTMETRICW) -> BOOL
{
  static mut s_pfn_Get_Text_MetricsW: Option<GetTextMetricsW_Proc> = None;
  let mut hm_unicows: HMODULE = std::ptr::null_mut ();
  if g_b_init_get_text_metrics_w == 0
    {
      g_b_init_get_text_metrics_w = 1;
      hm_unicows = w32_load_unicows_or_gdi32 ();
      if ! hm_unicows.is_null ()
	{
	  s_pfn_Get_Text_MetricsW
	    = Some (std::mem::transmute::<VOIDFNPTR, GetTextMetricsW_Proc>
		      (get_proc_addr (hm_unicows, c"GetTextMetricsW".as_ptr ())));
	}
    }
  eassert! (s_pfn_Get_Text_MetricsW.is_some ());
  return (s_pfn_Get_Text_MetricsW.unwrap ()) (hdc, lptmw);
}

/// C 183-200: `get_glyph_outline_w`.
/// C: static DWORD WINAPI get_glyph_outline_w (HDC hdc, UINT uChar, UINT uFormat,
/// C:   LPGLYPHMETRICS lpgm, DWORD cbBuffer, LPVOID lpvBuffer, const MAT2 *lpmat2)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "system" fn get_glyph_outline_w (hdc: HDC, uChar: UINT, uFormat: UINT,
                                               lpgm: LPGLYPHMETRICS, cbBuffer: DWORD,
                                               lpvBuffer: LPVOID, lpmat2: *const MAT2) -> DWORD
{
  static mut s_pfn_Get_Glyph_OutlineW: Option<GetGlyphOutlineW_Proc> = None;
  let mut hm_unicows: HMODULE = std::ptr::null_mut ();
  if g_b_init_get_glyph_outline_w == 0
    {
      g_b_init_get_glyph_outline_w = 1;
      hm_unicows = w32_load_unicows_or_gdi32 ();
      if ! hm_unicows.is_null ()
	{
	  s_pfn_Get_Glyph_OutlineW
	    = Some (std::mem::transmute::<VOIDFNPTR, GetGlyphOutlineW_Proc>
		      (get_proc_addr (hm_unicows, c"GetGlyphOutlineW".as_ptr ())));
	}
    }
  eassert! (s_pfn_Get_Glyph_OutlineW.is_some ());
  return (s_pfn_Get_Glyph_OutlineW.unwrap ()) (hdc, uChar, uFormat, lpgm, cbBuffer, lpvBuffer,
					       lpmat2);
}

/// C 202-217: `get_char_width_32_w`.
/// C: static DWORD WINAPI get_char_width_32_w (HDC hdc, UINT uFirstChar, UINT uLastChar,
/// C:   LPINT lpBuffer)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "system" fn get_char_width_32_w (hdc: HDC, uFirstChar: UINT, uLastChar: UINT,
                                               lpBuffer: LPINT) -> BOOL
{
  static mut s_pfn_Get_Char_Width_32W: Option<GetCharWidth32W_Proc> = None;
  let mut hm_unicows: HMODULE = std::ptr::null_mut ();
  if g_b_init_get_char_width_32_w == 0
    {
      g_b_init_get_char_width_32_w = 1;
      hm_unicows = w32_load_unicows_or_gdi32 ();
      if ! hm_unicows.is_null ()
	{
	  s_pfn_Get_Char_Width_32W
	    = Some (std::mem::transmute::<VOIDFNPTR, GetCharWidth32W_Proc>
		      (get_proc_addr (hm_unicows, c"GetCharWidth32W".as_ptr ())));
	}
    }
  eassert! (s_pfn_Get_Char_Width_32W.is_some ());
  return (s_pfn_Get_Char_Width_32W.unwrap ()) (hdc, uFirstChar, uLastChar, lpBuffer);
}

/* C 220-230: the `#else` Cygwin arm - "Cygwin doesn't support Windows 9X, and links
   against GDI32.DLL, so it can just call these functions directly".  */
/// C: #define get_outline_metrics_w(h,d,o)   GetOutlineTextMetricsW(h,d,o)
/// C: #define get_text_metrics_w(h,t)        GetTextMetricsW(h,t)
/// C: #define get_glyph_outline_w(h,uc,f,gm,b,v,m) GetGlyphOutlineW(h,uc,f,gm,b,v,m)
/// C: #define get_char_width_32_w(h,fc,lc,b) GetCharWidth32W(h,fc,lc,b)
#[cfg (all (feature = "have_ntgui", not (feature = "windowsnt")))]
macro_rules! get_outline_metrics_w {
  ($h:expr, $d:expr, $o:expr) => { GetOutlineTextMetricsW ($h, $d, $o) };
}
#[cfg (all (feature = "have_ntgui", not (feature = "windowsnt")))]
macro_rules! get_text_metrics_w {
  ($h:expr, $t:expr) => { GetTextMetricsW ($h, $t) };
}
#[cfg (all (feature = "have_ntgui", not (feature = "windowsnt")))]
macro_rules! get_glyph_outline_w {
  ($h:expr, $uc:expr, $f:expr, $gm:expr, $b:expr, $v:expr, $m:expr)
    => { GetGlyphOutlineW ($h, $uc, $f, $gm, $b, $v, $m) };
}
#[cfg (all (feature = "have_ntgui", not (feature = "windowsnt")))]
macro_rules! get_char_width_32_w {
  ($h:expr, $fc:expr, $lc:expr, $b:expr) => { GetCharWidth32W ($h, $fc, $lc, $b) };
}

/* C 232-243: `intern_font_name` - "This code is similar to intern function from lread.c".  It is
   a non-`static` C definition (`w32font.h` 96 declares it), so it carries
   `#[unsafe(no_mangle)]`; that is the declaration this unit's definition collides with (the
   ledger's `## Owed`).  */
/// C: Lisp_Object intern_font_name (char * string)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn intern_font_name (string: *mut c_char) -> Lisp_Object
{
  let str_: Lisp_Object = DECODE_SYSTEM (build_string (string));
  let len: ptrdiff_t = SCHARS (str_);
  let obarray: Lisp_Object = check_obarray (Vobarray);
  let tem: Lisp_Object = oblookup (obarray, SSDATA (str_), len, len);
  /* This code is similar to intern function from lread.c.  */
  return if SYMBOLP (tem) { tem } else { intern_driver (str_, obarray, tem) };
}

/* C 245-253: `w32font_get_cache` - "Return a cache of font-entities on FRAME.  The cache must be
   a cons whose cdr part is the actual cache area".  `struct frame *` is `*mut c_void` in the
   mirror's `font_driver` record (P-72 (3)), the `src/androidfont.rs` 561 reading.  */
/// C: Lisp_Object w32font_get_cache (struct frame *f)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32font_get_cache (f: *mut c_void) -> Lisp_Object
{
  let f = f as *mut frame;
  let dpyinfo: *mut w32_display_info = FRAME_DISPLAY_INFO (f);
  return (*dpyinfo).name_list_element;
}

/* C 255-264: `w32font_list`.  */
/// C: static Lisp_Object w32font_list (struct frame *f, Lisp_Object font_spec)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn w32font_list (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{
  let f = f as *mut frame;
  let fonts: Lisp_Object = w32font_list_internal (f, font_spec, false);
  FONT_ADD_LOG! (c"w32font-list".as_ptr (), font_spec, fonts);
  return fonts;
}

/* C 266-276: `w32font_match`.  */
/// C: static Lisp_Object w32font_match (struct frame *f, Lisp_Object font_spec)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn w32font_match (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{
  let f = f as *mut frame;
  let entity: Lisp_Object = w32font_match_internal (f, font_spec, false);
  FONT_ADD_LOG! (c"w32font-match".as_ptr (), font_spec, entity);
  return entity;
}

/* C 278-308: `w32font_list_family`.  */
/// C: static Lisp_Object w32font_list_family (struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn w32font_list_family (f: *mut c_void) -> Lisp_Object
{
  let f = f as *mut frame;
  let mut list: Lisp_Object = Qnil ();
  let mut prev_quit: Lisp_Object = Vinhibit_quit;
  let mut font_match_pattern: LOGFONT = core::mem::zeroed ();
  let mut dc: HDC;

  memset (&raw mut font_match_pattern as *mut c_void, 0,
	  core::mem::size_of::<LOGFONT> ());				// C 295
  font_match_pattern.lfCharSet = DEFAULT_CHARSET;			// C 296

  /* Prevent quitting while EnumFontFamiliesEx runs and conses the
     list it will return.  That's because get_frame_dc acquires the
     critical section, so we cannot quit before we release it in
     release_frame_dc.  */
  Vinhibit_quit = Qt ();						// C 303
  dc = get_frame_dc (f);						// C 304

  EnumFontFamiliesEx (dc, &mut font_match_pattern,			// C 306
		      Some (add_font_name_to_list),
		      (&raw mut list) as LPARAM, 0);
  release_frame_dc (f, dc);						// C 309
  Vinhibit_quit = prev_quit;						// C 310

  return list;								// C 312
}

/* C 310-332: `w32font_open`.  */
/// C: static Lisp_Object w32font_open (struct frame *f, Lisp_Object font_entity, int pixel_size)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn w32font_open (f: *mut c_void, font_entity: Lisp_Object,
				   pixel_size: c_int) -> Lisp_Object
{
  let f = f as *mut frame;
  let font_object: Lisp_Object
    = font_make_object (VECSIZE (core::mem::size_of::<w32font_info> ()) as c_int,
			font_entity, pixel_size);
  let w32_font: *mut w32font_info = XFONT_OBJECT (font_object) as *mut w32font_info;

  ASET (font_object, FONT_TYPE_INDEX as isize, Qgdi ());		// C 324

  if w32font_open_internal (f, font_entity, pixel_size, font_object) == 0
    {
      return Qnil ();							// C 326
    }

  /* GDI backend does not use glyph indices.  */
  (*w32_font).glyph_idx = 0;						// C 330

  return font_object;							// C 332
}

/* C 334-361: `w32font_close`.  */
/// C: void w32font_close (struct font *font)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32font_close (font: *mut font)
{
  let w32_font: *mut w32font_info = font as *mut w32font_info;

  if ! (*w32_font).hfont.is_null ()
    {
      /* Delete the GDI font object.  */
      DeleteObject ((*w32_font).hfont);					// C 344
      (*w32_font).hfont = std::ptr::null_mut ();			// C 345

      /* Free all the cached metrics.  */
      if ! (*w32_font).cached_metrics.is_null ()			// C 348
	{
	  let mut i: c_int;

	  i = 0;							// C 352
	  while i < (*w32_font).n_cache_blocks
	    {
	      xfree (*(*w32_font).cached_metrics.offset (i as isize) as *mut c_void);
	      i += 1;
	    }
	  xfree ((*w32_font).cached_metrics as *mut c_void);		// C 354
	  (*w32_font).cached_metrics = std::ptr::null_mut ();		// C 355
	}
    }
}

/* C 363-405: `w32font_has_char` - "We can't be certain about which characters a font will
   support until we open it", so the frozen body is C 369's `return -1;` and C 371-401 is an
   `#if 0` region.  C's own preprocessor never compiles it, so the port keeps C's text inside a
   comment (the `cm.rs` 86 reading):
     #if 0
       Lisp_Object supported_scripts, extra, script;
       DWORD mask;
       extra = AREF (entity, FONT_EXTRA_INDEX);
       if (!CONSP (extra))
         return -1;
       supported_scripts = assq_no_quit (QCscript, extra);
       if (!CONSP (supported_scripts))
         return -1;
       supported_scripts = XCDR (supported_scripts);
       script = CHAR_TABLE_REF (Vchar_script_table, c);
       if (NILP (script) || !NILP (memq_no_quit (script, supported_scripts)))
         return -1;
       return -1;
     #endif
   */
/// C: int w32font_has_char (Lisp_Object entity, int c)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32font_has_char (entity: Lisp_Object, c: c_int) -> c_int
{
  /* We can't be certain about which characters a font will support until
     we open it.  Checking the scripts that the font supports turns out
     to not be reliable.  */
  let _ = (entity, c);
  return -1;								// C 369
}

/* C 407-427: `w32font_encode_char`.  */
/// C: static unsigned w32font_encode_char (struct font *font, int c)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn w32font_encode_char (font: *mut font, c: c_int) -> c_uint
{
  let w32_font: *mut w32font_info = font as *mut w32font_info;

  if c < (*w32_font).metrics.tmFirstChar as c_int
      || c > (*w32_font).metrics.tmLastChar as c_int
    { return FONT_INVALID_CODE; }					// C 419
  else
    { return c as c_uint; }						// C 421
}

/* C 429-620: `w32font_text_extents` - "Perform the size computation of glyphs of FONT and fill in
   members of METRICS".  C's `alloca` is the port's stack pool (PORTING.md P-05, the `xrdb.rs` 63
   reading), and C's `struct frame *f UNINIT;` is `UNINIT`'s `= {0,}` arm - the null pointer.  */
/// C: void w32font_text_extents (struct font *font, const unsigned *code, int nglyphs,
/// C:                             struct font_metrics *metrics)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32font_text_extents (font: *mut font, code: *const c_uint,
                                               mut nglyphs: c_int, metrics: *mut font_metrics)
{
  let mut i: c_int;
  let mut old_font: HFONT = std::ptr::null_mut ();			// C 436
  let mut dc: HDC = std::ptr::null_mut ();				// C 437
  let mut f: *mut frame = std::ptr::null_mut ();			// C 438 (`UNINIT`)
  let mut total_width: c_int = 0;					// C 439
  let mut wcode: *mut WORD;						// C 440
  let mut size: SIZE = core::mem::zeroed ();				// C 441
  let mut first: bool;							// C 442
  let mut prev_quit: Lisp_Object = Vinhibit_quit;			// C 443

  let w32_font: *mut w32font_info = font as *mut w32font_info;		// C 445

  memset (metrics as *mut c_void, 0, core::mem::size_of::<font_metrics> ());	// C 447

  if w32_use_direct_write (w32_font)					// C 449
      && w32_dwrite_text_extents (font, code, nglyphs, metrics)
    { return; }

  i = 0;								// C 453
  first = true;
  while i < nglyphs
    {
      let char_metric: *mut w32_metric_cache;
      let block: c_int = (*code.offset (i as isize) / CACHE_BLOCKSIZE as c_uint) as c_int;
      let pos_in_block: c_int
	= (*code.offset (i as isize) % CACHE_BLOCKSIZE as c_uint) as c_int;

      if block >= (*w32_font).n_cache_blocks				// C 461
	{
	  if (*w32_font).cached_metrics.is_null ()			// C 463
	    {
	      (*w32_font).cached_metrics
		= xmalloc (((block + 1) as usize)
			   * core::mem::size_of::<*mut w32_metric_cache> ())
		  as *mut *mut w32_metric_cache;
	    }
	  else
	    {
	      (*w32_font).cached_metrics
		= xrealloc ((*w32_font).cached_metrics as *mut c_void,
			    ((block + 1) as usize)
			    * core::mem::size_of::<*mut w32_metric_cache> ())
		  as *mut *mut w32_metric_cache;
	    }
	  memset ((*w32_font).cached_metrics.offset ((*w32_font).n_cache_blocks as isize)
		  as *mut c_void, 0,
		  ((block + 1 - (*w32_font).n_cache_blocks) as usize)
		  * core::mem::size_of::<*mut w32_metric_cache> ());
	  (*w32_font).n_cache_blocks = block + 1;			// C 470
	}

      if (*(*w32_font).cached_metrics.offset (block as isize)).is_null ()	// C 473
	{
	  *(*w32_font).cached_metrics.offset (block as isize)
	    = xzalloc (CACHE_BLOCKSIZE as usize * core::mem::size_of::<w32_metric_cache> ())
	      as *mut w32_metric_cache;
	}

      char_metric = (*(*w32_font).cached_metrics.offset (block as isize))
	.offset (pos_in_block as isize);				// C 479

      if (*char_metric).status == W32METRIC_NO_ATTEMPT			// C 481
	{
	  if dc.is_null ()						// C 483
	    {
	      /* TODO: Frames can come and go, and their fonts
		 outlive them. So we can't cache the frame in the
		 font structure.  Use selected_frame until the API
		 is updated to pass in a frame.  */
	      f = XFRAME (selected_frame);				// C 489

	      /* Prevent quitting while EnumFontFamiliesEx runs and
		 conses the list it will return.  That's because
		 get_frame_dc acquires the critical section, so we
		 cannot quit before we release it in release_frame_dc.  */
	      prev_quit = Vinhibit_quit;				// C 494
	      Vinhibit_quit = Qt ();					// C 495
	      dc = get_frame_dc (f);					// C 496
	      old_font = SelectObject (dc, (*w32_font).hfont);
	    }
	  compute_metrics (dc, w32_font, *code.offset (i as isize), char_metric);	// C 499
	}

      if (*char_metric).status == W32METRIC_SUCCESS			// C 502
	{
	  if first							// C 504
	    {
	      (*metrics).lbearing = (*char_metric).lbearing;
	      (*metrics).rbearing = (*char_metric).rbearing;
	      (*metrics).width    = 0;
	      (*metrics).ascent   = (*char_metric).ascent;
	      (*metrics).descent  = (*char_metric).descent;
	      first = false;
	    }
	  if (*metrics).lbearing > (*char_metric).lbearing
	    { (*metrics).lbearing = (*char_metric).lbearing; }
	  if (*metrics).rbearing < (*char_metric).rbearing
	    { (*metrics).rbearing = (*char_metric).rbearing; }
	  (*metrics).width += (*char_metric).width;
	  if (*metrics).ascent < (*char_metric).ascent
	    { (*metrics).ascent = (*char_metric).ascent; }
	  if (*metrics).descent < (*char_metric).descent
	    { (*metrics).descent = (*char_metric).descent; }
	}
      else
	/* If we couldn't get metrics for a char,
	   use alternative method.  */
	{ break; }

      i += 1;								// C 453's `i++`
    }
  /* If we got through everything, return.  */
  if i == nglyphs							// C 524
    {
      if ! dc.is_null ()						// C 526
	{
	  /* Restore state and release DC.  */
	  SelectObject (dc, old_font as HGDIOBJ);			// C 529
	  release_frame_dc (f, dc);					// C 530
	  Vinhibit_quit = prev_quit;					// C 531
	}
      return;
    }

  /* For non-truetype fonts, GetGlyphOutlineW is not supported, so
     fallback on other methods that will at least give some of the metric
     information.  */

  /* Make array big enough to hold surrogates.  */
  eassume! (0 <= nglyphs);	/* pacify GCC warning on next line */	// C 540
  wcode = sa_stack_alloc ((nglyphs as usize) * core::mem::size_of::<WORD> () * 2)
	  as *mut WORD;							// C 541
  i = 0;								// C 542
  while i < nglyphs
    {
      if *code.offset (i as isize) < 0x10000				// C 544
	{
	  *wcode.offset (i as isize) = *code.offset (i as isize) as WORD;
	}
      else
	{
	  let surrogate: DWORD = *code.offset (i as isize) - 0x10000;	// C 549

	  /* High surrogate: U+D800 - U+DBFF.  */
	  *wcode.offset (i as isize) = (0xD800 + ((surrogate >> 10) & 0x03FF)) as WORD;	// C 552
	  i += 1;
	  /* Low surrogate: U+DC00 - U+DFFF.  */
	  *wcode.offset (i as isize) = (0xDC00 + (surrogate & 0x03FF)) as WORD;		// C 554
	  /* An extra glyph. wcode is already double the size of code to
	     cope with this.  */
	  nglyphs += 1;							// C 557
	}
      i += 1;
    }

  if dc.is_null ()							// C 561
    {
      /* TODO: Frames can come and go, and their fonts outlive
	 them. So we can't cache the frame in the font structure.  Use
	 selected_frame until the API is updated to pass in a
	 frame.  */
      f = XFRAME (selected_frame);					// C 567

      /* Prevent quitting while EnumFontFamiliesEx runs and conses the
	 list it will return.  That's because get_frame_dc acquires
	 the critical section, so we cannot quit before we release it
	 in release_frame_dc.  */
      prev_quit = Vinhibit_quit;					// C 572
      Vinhibit_quit = Qt ();						// C 573
      dc = get_frame_dc (f);						// C 574
      old_font = SelectObject (dc, (*w32_font).hfont);			// C 575
    }

  if GetTextExtentPoint32W (dc, wcode, nglyphs, &mut size) != 0		// C 578
    {
      total_width = size.cx;						// C 580
    }

  /* On 95/98/ME, only some Unicode functions are available, so fallback
     on doing a dummy draw to find the total width.  */
  if total_width == 0							// C 585
    {
      let mut rect: RECT = core::mem::zeroed ();
      rect.top = 0; rect.bottom = (*font).height; rect.left = 0; rect.right = 1;	// C 588
      DrawTextW (dc, wcode, nglyphs, &mut rect,				// C 589
		 DT_CALCRECT | DT_NOPREFIX | DT_SINGLELINE);
      total_width = rect.right;						// C 591
    }

  /* Give our best estimate of the metrics, based on what we know.  */
  (*metrics).width = (total_width - (*w32_font).metrics.tmOverhang) as c_short;	// C 594
  (*metrics).lbearing = 0;						// C 595
  (*metrics).rbearing = total_width as c_short;				// C 596
  (*metrics).ascent = (*font).ascent as c_short;			// C 597
  (*metrics).descent = (*font).descent as c_short;			// C 598

  /* Restore state and release DC.  */
  SelectObject (dc, old_font as HGDIOBJ);				// C 601
  release_frame_dc (f, dc);						// C 602
  Vinhibit_quit = prev_quit;						// C 603
}

// C 606-625: the draw method's banner ("TODO: Currently this assumes that the colors and fonts are
//    already set in the DC. ...") is C's own text and is carried at its position:

//    /* w32 implementation of draw for font backend.
//       Optional.
//       Draw glyphs between FROM and TO of S->char2b at (X Y) pixel
//       position of frame F with S->FACE and S->GC.  If WITH_BACKGROUND,
//       fill the background in advance.  It is assured that WITH_BACKGROUND
//       is false when (FROM > 0 || TO < S->nchars).

//       TODO: Currently this assumes that the colors and fonts are already
//       set in the DC. This seems to be true now, but maybe only due to
//       the old font code setting it up. It may be safer to resolve faces
//       and fonts in here and set them explicitly
//

/* The C bitfield `glyph->u.glyphless.method` is the mirror's packed `bits` field; the port reads
   it through a macro, the `src/w32term.rs` 1733 / `src/xftfont.rs` 1015 spelling (P-55).  */
macro_rules! glyphless_bits {
  (method, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_METHOD_SHIFT) & 0x3) as c_int };
  (for_no_font, $g:expr) => { ((*$g).u.glyphless.bits & GLYPH_U_GLYPHLESS_FOR_NO_FONT) != 0 };
  (len, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_LEN_SHIFT) & 0xf) as c_int };
  (ch, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_CH_SHIFT) & 0x1ffffff) as c_int };
}

/* C 628-745: `w32font_draw`.  `struct glyph_string`'s `clip` is the mirror's `[c_int; 2]`
   because `NativeRectangle` is a *type* macro (`w32gui.rs` 168 gives `RECT`), so the two
   `CreateRectRgnIndirect` call sites cast to `*const RECT`; C's `alloca` is the port's stack
   pool.  */
/// C: int w32font_draw (struct glyph_string *s, int from, int to, int x, int y,
/// C:                    bool with_background)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32font_draw (s: *mut glyph_string, from: c_int, to: c_int,
				       x: c_int, y: c_int, with_background: bool) -> c_int
{
  let mut options: UINT;						// C 632
  let mut orig_clip: HRGN = std::ptr::null_mut ();			// C 633
  let len: c_int = to - from;						// C 634
  let w32font: *mut w32font_info = (*s).font as *mut w32font_info;	// C 635

  options = (*w32font).glyph_idx;					// C 637

  if (*s).num_clips > 0							// C 639
    {
      let new_clip: HRGN
	= CreateRectRgnIndirect (((*s).clip.as_mut_ptr ()) as *const RECT);	// C 641

      /* Save clip region for later restoration.  */
      orig_clip = CreateRectRgn (0, 0, 0, 0);				// C 644
      if GetClipRgn ((*s).hdc as HDC, orig_clip) == 0			// C 645
	{
	  DeleteObject (orig_clip);					// C 647
	  orig_clip = std::ptr::null_mut ();				// C 648
	}

      if (*s).num_clips > 1						// C 651
	{
	  let clip2: HRGN
	    = CreateRectRgnIndirect (((*s).clip.as_mut_ptr ()).offset (1) as *const RECT);	// C 653

	  CombineRgn (new_clip, new_clip, clip2, RGN_OR);		// C 655
	  DeleteObject (clip2);						// C 656
	}

      SelectClipRgn ((*s).hdc as HDC, new_clip);			// C 659
      DeleteObject (new_clip);						// C 660
    }

  /* Using OPAQUE background mode can clear more background than expected
     when Cleartype is used.  Draw the background manually to avoid this.  */
  SetBkMode ((*s).hdc as HDC, TRANSPARENT);				// C 664
  if with_background							// C 665
    {
      let mut brush: HBRUSH;
      let mut rect: RECT = core::mem::zeroed ();
      let font: *mut font = (*s).font;
      let mut ascent: c_int = (*font).ascent;
      let mut descent: c_int = (*font).descent;

      /* Font's global ascent and descent values might be
	 preposterously large for some fonts.  We fix here the case
	 when those fonts are used for display of glyphless
	 characters, because drawing background with font dimensions
	 in those cases makes the display illegible.  There's only one
	 more call to the draw method with with_background set to
	 true, and that's in w32_draw_glyph_string_foreground, when
	 drawing the cursor, where we have no such heuristics
	 available.  FIXME.  */
      if glyph_bits! (type, *(*s).first_glyph) == GLYPHLESS_GLYPH	// C 680
	  && (glyphless_bits! (method, (*s).first_glyph) == GLYPHLESS_DISPLAY_HEX_CODE
	      || glyphless_bits! (method, (*s).first_glyph) == GLYPHLESS_DISPLAY_ACRONYM)
	{
	  ascent =
	    (*(*s).first_glyph).slice.glyphless.lower_yoff as c_int
	    - (*(*s).first_glyph).slice.glyphless.upper_yoff as c_int;	// C 686
	  descent = 0;							// C 687
	}
      brush = CreateSolidBrush ((*(*s).gc).background as COLORREF);	// C 690
      rect.left = x;							// C 691
      rect.top = y - ascent;						// C 692
      rect.right = x + (*s).width;					// C 693
      rect.bottom = y + descent;					// C 694
      FillRect ((*s).hdc as HDC, &mut rect, brush);			// C 695
      DeleteObject (brush);						// C 696
    }

  if glyph_string_bits! (padding_p, *s)					// C 699
    {
      let mut i: c_int;

      i = 0;								// C 702
      while i < len
	{
	  if ! w32_use_direct_write (w32font)				// C 703
	      || ! w32_dwrite_draw ((*s).hdc as HDC, x, y, (*s).char2b.offset (from as isize),
				    1, GetTextColor ((*s).hdc as HDC), (*s).font)
	    {
	      let c: WCHAR = (*(*s).char2b.offset ((from + i) as isize) & 0xFFFF) as WCHAR;
	      ExtTextOutW ((*s).hdc as HDC, x + i, y, options, std::ptr::null (), &c, 1,
			   std::ptr::null ());				// C 707
	    }
	  i += 1;
	}
    }
  else
    {
      if ! w32_use_direct_write (w32font)				// C 713
	  || ! w32_dwrite_draw ((*s).hdc as HDC, x, y, (*s).char2b.offset (from as isize),
				len, GetTextColor ((*s).hdc as HDC), (*s).font)
	{
	  /* The number of glyphs in a glyph_string cannot be larger than
	     the maximum value of the 'used' member of a glyph_row, so we
	     are OK using alloca here.  */
	  eassert! (len <= SHRT_MAX);					// C 721
	  let chars: *mut WCHAR
	    = sa_stack_alloc ((len as usize) * core::mem::size_of::<WCHAR> ()) as *mut WCHAR;
	  let mut j: c_int;
	  j = 0;							// C 724
	  while j < len
	    {
	      *chars.offset (j as isize)
		= (*(*s).char2b.offset ((from + j) as isize) & 0xFFFF) as WCHAR;
	      j += 1;
	    }
	  ExtTextOutW ((*s).hdc as HDC, x, y, options, std::ptr::null (), chars, len as UINT,
		       std::ptr::null ());				// C 726
	}
    }

  /* Restore clip region.  */
  if (*s).num_clips > 0							// C 730
    { SelectClipRgn ((*s).hdc as HDC, orig_clip); }

  if ! orig_clip.is_null ()						// C 733
    { DeleteObject (orig_clip); }

  return len;								// C 736
}

// C 738-830: the six optional methods this backend does not define.  Each is a design comment
//    around a prototype, and C's own preprocessor never compiles the prototype (it is inside a
//    comment in C too).  Carried as C's own text:

//    /* w32 implementation of free_entity for font backend.
//       Optional.
//       Free FONT_EXTRA_INDEX field of FONT_ENTITY.
//    static void
//    w32font_free_entity (Lisp_Object entity);
//      */

//    /* w32 implementation of prepare_face for font backend.
//       Optional (if FACE->extra is not used).
//       Prepare FACE for displaying characters by FONT on frame F by
//       storing some data in FACE->extra.  If successful, return 0.
//       Otherwise, return -1.
//    static int
//    w32font_prepare_face (struct frame *f, struct face *face);
//      */
//    /* w32 implementation of done_face for font backend.
//       Optional.
//       Done FACE for displaying characters by FACE->font on frame F.
//    static void
//    w32font_done_face (struct frame *f, struct face *face);  */

//    /* w32 implementation of get_bitmap for font backend.
//       Optional.
//       Store bitmap data for glyph-code CODE of FONT in BITMAP.  It is
//       intended that this method is called from the other font-driver
//       for actual drawing.
//    static int
//    w32font_get_bitmap (struct font *font, unsigned code,
//                        struct font_bitmap *bitmap, int bits_per_pixel);
//      */
//    /* w32 implementation of free_bitmap for font backend.
//       Optional.
//       Free bitmap data in BITMAP.
//    static void
//    w32font_free_bitmap (struct font *font, struct font_bitmap *bitmap);
//      */
//    /* w32 implementation of anchor_point for font backend.
//       Optional.
//       Get coordinates of the INDEXth anchor point of the glyph whose
//       code is CODE.  Store the coordinates in *X and *Y.  Return 0 if
//       the operations was successful.  Otherwise return -1.
//    static int
//    w32font_anchor_point (struct font *font, unsigned code,
//                                     int index, int *x, int *y);
//      */
//    /* w32 implementation of otf_capability for font backend.
//       Optional.
//       Return a list describing which scripts/languages FONT
//       supports by which GSUB/GPOS features of OpenType tables.
//    static Lisp_Object
//    w32font_otf_capability (struct font *font);
//      */
//    /* w32 implementation of otf_drive for font backend.
//       Optional.
//       Apply FONT's OTF-FEATURES to the glyph string.

//       FEATURES specifies which OTF features to apply in this format:
//          (SCRIPT LANGSYS GSUB-FEATURE GPOS-FEATURE)
//       See the documentation of `font-drive-otf' for the detail.

//       This method applies the specified features to the codes in the
//       elements of GSTRING-IN (between FROMth and TOth).  The output
//       codes are stored in GSTRING-OUT at the IDXth element and the
//       following elements.

//       Return the number of output codes.  If none of the features are
//       applicable to the input data, return 0.  If GSTRING-OUT is too
//       short, return -1.
//    static int
//    w32font_otf_drive (struct font *font, Lisp_Object features,
//                       Lisp_Object gstring_in, int from, int to,
//                       Lisp_Object gstring_out, int idx,
//                       bool alternate_subst);
//      */
//

// C 808-902: "Notes about the way fonts are found on MS-Windows when we have a character
//    unsupported by the default font."  Carried as C's own text (an extract):

//    /* Notes about the way fonts are found on MS-Windows when we have a
//       character unsupported by the default font.

//       Since we don't use Fontconfig on MS-Windows, we cannot efficiently
//       search for fonts which support certain characters, because Windows
//       doesn't store this information anywhere, and we can only know whether
//       a font supports some character if we actually open the font, which is
//       expensive and slow.  Instead, we rely on font information Windows
//       exposes to the API we use to enumerate available fonts,
//       EnumFontFamiliesEx.  This information includes two bitmapped attributes:

//         USB (which stands for Unicode Subset Bitfields) -- this is an array
//             of 4 32-bit values, 128 bits in total, where each bit
//             corresponds to some block (sometimes several related blocks) of
//             Unicode codepoints which the font claims to support.
//         CSB (which stands for Codepage Bitfields) -- this is an array of 2
//             32-bit values (64 bits), where each bit corresponds to some
//             codepage whose characters the font claims to support.

//       When Emacs needs to find a font for a character, it enumerates the
//       available fonts, filtering the fonts by examining these bitmaps and a
//       few other font attributes.  The script of the character is converted
//       to the corresponding bits in USB, and a font that has any of these
//       bits set is deemed as a candidate; see font_supported_scripts, which
//       is called by font_matches_spec.  The problem with this strategy is
//       twofold:

//        - Some Unicode blocks have no USB bits.  For the scripts
//          corresponding to those blocks we use a small cache of fonts known
//          to support those script.  ...  See the function w32-find-non-USB-fonts.

//        - Some fonts claim support for a block, but don't support it well.
//          ...  We
//          consider those as candidates for CJK scripts ('han', 'kana',
//          etc.) if the CSB bits are set for the corresponding CJK
//          codepages.

//       Eventually, some characters could still appear as "tofu" (a box with
//       the character's hex codepoint), even though a font might be available
//       on the system which supports the character.  ...  The solution for these
//       issues is to customize the
//       default fontset using set-fontset-font, to force Emacs to use a font
//       known to support some characters.

//       One other Windows-specific issue is the fact that some Windows fonts
//       have hyphens in their names.  ...  See this page for the latest update:

//         https://learn.microsoft.com/en-us/typography/fonts/windows_11_font_list
//

/* C 904-986: `w32font_list_internal`.  */
/// C: Lisp_Object w32font_list_internal (struct frame *f, Lisp_Object font_spec,
/// C:                                    bool opentype_only)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32font_list_internal (f: *mut frame, font_spec: Lisp_Object,
						opentype_only: bool) -> Lisp_Object
{
  let mut match_data: font_callback_data = core::mem::zeroed ();	// C 910
  let mut dc: HDC;

  match_data.orig_font_spec = font_spec;				// C 913
  match_data.list = Qnil ();						// C 914
  XSETFRAME! (match_data.frame, f);					// C 915

  memset (&raw mut match_data.pattern as *mut c_void, 0,
	  core::mem::size_of::<LOGFONT> ());				// C 917
  fill_in_logfont (f, &mut match_data.pattern, font_spec);		// C 918

  /* If the charset is unrecognized, then we won't find a font, so don't
     waste time looking for one.  */
  if match_data.pattern.lfCharSet == DEFAULT_CHARSET			// C 922
    {
      let spec_charset: Lisp_Object = AREF (font_spec, FONT_REGISTRY_INDEX as isize);
      if ! NILP (spec_charset)						// C 925
	  && ! EQ (spec_charset, Qiso10646_1 ())
	  && ! EQ (spec_charset, Qunicode_bmp ())
	  && ! EQ (spec_charset, Qunicode_sip ())
	  && ! EQ (spec_charset, Qunknown ())
	  && ! EQ (spec_charset, Qascii_0 ())
	{ return Qnil (); }
    }

  match_data.opentype_only = opentype_only;				// C 932
  if opentype_only
    { match_data.pattern.lfOutPrecision = OUT_OUTLINE_PRECIS; }		// C 934
  match_data.known_fonts = Qnil ();					// C 935
  let vw32_non_USB_fonts: Lisp_Object = Fsymbol_value (Qw32_non_USB_fonts ());
  let mut val: Lisp_Object;						// C 936
  if CONSP (vw32_non_USB_fonts)						// C 937
    {
      let mut extra: Lisp_Object;
      extra = AREF (font_spec, FONT_EXTRA_INDEX as isize);		// C 941
      while CONSP (extra)
	{
	  let tem: Lisp_Object = XCAR (extra);				// C 944
	  if CONSP (tem)						// C 945
	      && EQ (XCAR (tem), QCscript ())
	      && SYMBOLP (XCDR (tem))
	      && { val = assq_no_quit (XCDR (tem), vw32_non_USB_fonts); ! NILP (val) }
	    {
	      match_data.known_fonts = XCDR (val);			// C 950
	      break;
	    }
	  extra = XCDR (extra);						// C 941's `extra = XCDR (extra)`
	}
    }

  if match_data.pattern.lfFaceName[0] == 0				// C 955
    {
      /* EnumFontFamiliesEx does not take other fields into account if
         font name is blank, so need to use two passes.  */
      list_all_matching_fonts (&mut match_data);			// C 959
    }
  else
    {
      let mut prev_quit: Lisp_Object = Vinhibit_quit;			// C 963

      /* Prevent quitting while EnumFontFamiliesEx runs and conses the
	 list it will return.  That's because get_frame_dc acquires
	 the critical section, so we cannot quit before we release it
	 in release_frame_dc.  */
      Vinhibit_quit = Qt ();						// C 970
      dc = get_frame_dc (f);						// C 971

      EnumFontFamiliesEx (dc, &mut match_data.pattern,			// C 973
			  Some (add_font_entity_to_list),
			  (&raw mut match_data) as LPARAM, 0);
      release_frame_dc (f, dc);						// C 976
      Vinhibit_quit = prev_quit;					// C 977
    }

  return match_data.list;						// C 980
}

/* C 988-1041: `w32font_match_internal`.  */
/// C: Lisp_Object w32font_match_internal (struct frame *f, Lisp_Object font_spec,
/// C:                                     bool opentype_only)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32font_match_internal (f: *mut frame, font_spec: Lisp_Object,
						 opentype_only: bool) -> Lisp_Object
{
  let mut match_data: font_callback_data = core::mem::zeroed ();	// C 994
  let mut dc: HDC;

  match_data.orig_font_spec = font_spec;				// C 997
  XSETFRAME! (match_data.frame, f);					// C 998
  match_data.list = Qnil ();						// C 999

  memset (&raw mut match_data.pattern as *mut c_void, 0,
	  core::mem::size_of::<LOGFONT> ());				// C 1001
  fill_in_logfont (f, &mut match_data.pattern, font_spec);		// C 1002

  match_data.opentype_only = opentype_only;				// C 1004
  if opentype_only
    { match_data.pattern.lfOutPrecision = OUT_OUTLINE_PRECIS; }		// C 1006

  match_data.known_fonts = Qnil ();					// C 1008
  let vw32_non_USB_fonts: Lisp_Object = Fsymbol_value (Qw32_non_USB_fonts ());
  let mut val: Lisp_Object;						// C 1009
  if CONSP (vw32_non_USB_fonts)						// C 1010
    {
      let mut extra: Lisp_Object;
      extra = AREF (font_spec, FONT_EXTRA_INDEX as isize);		// C 1013
      while CONSP (extra)
	{
	  let tem: Lisp_Object = XCAR (extra);				// C 1016
	  if CONSP (tem)
	      && EQ (XCAR (tem), QCscript ())
	      && SYMBOLP (XCDR (tem))
	      && { val = assq_no_quit (XCDR (tem), vw32_non_USB_fonts); ! NILP (val) }
	    {
	      match_data.known_fonts = XCDR (val);			// C 1022
	      break;
	    }
	  extra = XCDR (extra);
	}
    }

  /* Prevent quitting while EnumFontFamiliesEx runs and conses the
     list it will return.  That's because get_frame_dc acquires the
     critical section, so we cannot quit before we release it in
     release_frame_dc.  */
  let mut prev_quit: Lisp_Object = Vinhibit_quit;			// C 1032
  Vinhibit_quit = Qt ();						// C 1033
  dc = get_frame_dc (f);						// C 1034

  EnumFontFamiliesEx (dc, &mut match_data.pattern,			// C 1036
		      Some (add_one_font_entity_to_list),
		      (&raw mut match_data) as LPARAM, 0);
  release_frame_dc (f, dc);						// C 1039
  Vinhibit_quit = prev_quit;						// C 1040

  return if NILP (match_data.list) { Qnil () } else { XCAR (match_data.list) };	// C 1042
}

/* C 1043-1172: `w32font_open_internal`.  C's `alloca` is the port's stack pool (P-05).  */
/// C: int w32font_open_internal (struct frame *f, Lisp_Object font_entity, int pixel_size,
/// C:                             Lisp_Object font_object)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32font_open_internal (f: *mut frame, font_entity: Lisp_Object,
						pixel_size: c_int, font_object: Lisp_Object)
						-> c_int
{
  let mut len: c_int;							// C 1046
  let mut size: c_int;
  let mut logfont: LOGFONT = core::mem::zeroed ();			// C 1047
  let mut dc: HDC;							// C 1048
  let mut hfont: HFONT;							// C 1049
  let mut old_font: HFONT;
  let val: Lisp_Object;							// C 1050
  let w32_font: *mut w32font_info;					// C 1051
  let font: *mut font;							// C 1052
  let mut metrics: *mut OUTLINETEXTMETRICW = std::ptr::null_mut ();	// C 1053

  w32_font = XFONT_OBJECT (font_object) as *mut w32font_info;		// C 1055
  font = w32_font as *mut font;						// C 1056

  if font.is_null ()							// C 1058
    { return 0; }

  memset (&raw mut logfont as *mut c_void, 0, core::mem::size_of::<LOGFONT> ());	// C 1061
  fill_in_logfont (f, &mut logfont, font_entity);			// C 1062

  /* Prefer truetype fonts, to avoid known problems with type1 fonts, and
     limitations in bitmap fonts.  */
  val = AREF (font_entity, FONT_FOUNDRY_INDEX as isize);		// C 1066
  if ! EQ (val, Qraster ())
    { logfont.lfOutPrecision = OUT_TT_PRECIS; }				// C 1068

  size = XFIXNUM (AREF (font_entity, FONT_SIZE_INDEX as isize)) as c_int;	// C 1070
  if size == 0
    { size = pixel_size; }						// C 1072

  logfont.lfHeight = -size;						// C 1074
  hfont = CreateFontIndirect (&logfont);				// C 1075

  if hfont.is_null ()							// C 1077
    { return 0; }

  /* Get the metrics for this font.  */
  dc = get_frame_dc (f);						// C 1081
  old_font = SelectObject (dc, hfont);					// C 1082

  /* Try getting the outline metrics (only works for truetype fonts).  */
  len = get_outline_metrics_w (dc, 0, std::ptr::null_mut ()) as c_int;	// C 1085
  if len != 0								// C 1086
    {
      metrics = sa_stack_alloc (len as usize) as *mut OUTLINETEXTMETRICW;	// C 1088
      if get_outline_metrics_w (dc, len as UINT, metrics) != 0		// C 1089
	{
	  memcpy (&raw mut (*w32_font).metrics as *mut c_void,
		  &raw const (*metrics).otmTextMetrics as *const c_void,
		  core::mem::size_of::<TEXTMETRICW> ());		// C 1090
	}
      else
	{ metrics = std::ptr::null_mut (); }				// C 1093
    }

  if metrics.is_null ()							// C 1096
    { get_text_metrics_w (dc, &mut (*w32_font).metrics); }		// C 1097

  (*w32_font).cached_metrics = std::ptr::null_mut ();			// C 1099
  (*w32_font).n_cache_blocks = 0;					// C 1100

  SelectObject (dc, old_font);						// C 1102
  release_frame_dc (f, dc);						// C 1103

  (*w32_font).hfont = hfont;						// C 1105

  {
    let mut name: *mut c_char;						// C 1108

    /* We don't know how much space we need for the full name, so start with
       96 bytes and go up in steps of 32.  */
    len = 96;								// C 1112
    name = sa_stack_alloc (len as usize) as *mut c_char;		// C 1113
    while ! name.is_null ()						// C 1114
	&& w32font_full_name (&mut logfont, font_entity, pixel_size, name, len) < 0
      {
        len += 32;							// C 1117
        name = sa_stack_alloc (len as usize) as *mut c_char;		// C 1118
      }
    if ! name.is_null ()						// C 1120
      {
	(*font).props[FONT_FULLNAME_INDEX as usize]
	  = DECODE_SYSTEM (build_string (name));
      }
    else
      {
	(*font).props[FONT_FULLNAME_INDEX as usize]
	  = DECODE_SYSTEM (build_string (logfont.lfFaceName.as_ptr ()));	// C 1124
      }
  }

  (*font).max_width = (*w32_font).metrics.tmMaxCharWidth as c_int;	// C 1127
  /* Parts of Emacs display assume that height = ascent + descent...
     so height is defined later, after ascent and descent.
  font->height = w32_font->metrics.tmHeight
    + w32_font->metrics.tmExternalLeading;
  */

  (*font).space_width = (*w32_font).metrics.tmAveCharWidth as c_int;	// C 1135
  (*font).average_width = (*w32_font).metrics.tmAveCharWidth as c_int;

  (*font).vertical_centering = false;					// C 1137
  (*font).baseline_offset = 0;						// C 1138
  (*font).relative_compose = 0;						// C 1139
  (*font).default_ascent = (*w32_font).metrics.tmAscent as c_int;	// C 1140
  (*font).pixel_size = size;						// C 1141
  (*font).driver = &raw const w32font_driver as *const c_void;		// C 1142
  (*font).encoding_charset = -1;					// C 1143
  (*font).repertory_charset = -1;					// C 1144
  /* TODO: do we really want the minimum width here, which could be negative? */
  (*font).min_width = (*font).space_width;				// C 1146
  (*font).ascent = (*w32_font).metrics.tmAscent as c_int;		// C 1147
  (*font).descent = (*w32_font).metrics.tmDescent as c_int;		// C 1148
  (*font).height = (*font).ascent + (*font).descent;			// C 1149

  if ! metrics.is_null ()						// C 1151
    {
      (*font).underline_thickness = (*metrics).otmsUnderscoreSize;	// C 1153
      (*font).underline_position = -(*metrics).otmsUnderscorePosition;	// C 1154
    }
  else
    {
      (*font).underline_thickness = 0;					// C 1158
      (*font).underline_position = -1;					// C 1159
    }

  /* For temporary compatibility with legacy code that expects the
     name to be usable in x-list-fonts. Eventually we expect to change
     x-list-fonts and other places that use fonts so that this can be
     an fcname or similar.  */
  (*font).props[FONT_NAME_INDEX as usize]
    = Ffont_xlfd_name (font_object, Qnil (), Qt ());			// C 1166

  return 1;								// C 1168
}

/* The names the file's own text uses that no mirror in this graph carries (P-55's shape: they are
   declared here, at a `/// C:` line with the reference's own text, until the mirror that owns
   them lands - the ledger's `## Owed`).  */
/// C: lisp.h 4729: `extern Lisp_Object intern_driver (Lisp_Object, Lisp_Object, Lisp_Object);`
/// C: lisp.h 4286: `Lisp_Object memq_no_quit (Lisp_Object elt, Lisp_Object list);`
/// C: font.h 842: `extern Lisp_Object font_make_entity (void);`
/// C: font.h 846: `extern Lisp_Object font_make_object (int, Lisp_Object, int);`
/// C: font.h 885: `extern Lisp_Object font_intern_prop (const char *str, ptrdiff_t len, bool force);`
/// C: font.h 913: `extern Lisp_Object font_put_extra (Lisp_Object font, Lisp_Object prop,
/// C:                                            Lisp_Object val);`
/// C: font.h 927: `extern void font_filter_properties (Lisp_Object font, Lisp_Object alist,
/// C:                                          const char *const booleans[],
/// C:                                          const char *const non_booleans[]);`
/// C: font.h 896: `extern void register_font_driver (struct font_driver const *,
/// C:                                           struct frame *);`
/// C: frame.h: `struct frame *decode_window_system_frame (Lisp_Object);`
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {


  pub fn font_make_entity () -> Lisp_Object;
  pub fn font_make_object (size: c_int, entity: Lisp_Object, pixel_size: c_int) -> Lisp_Object;
  pub fn font_intern_prop (str_: *const c_char, len: ptrdiff_t, force: bool) -> Lisp_Object;
  pub fn font_put_extra (font: Lisp_Object, prop: Lisp_Object, val: Lisp_Object) -> Lisp_Object;
  pub fn font_filter_properties (font: Lisp_Object, alist: Lisp_Object,
				 booleans: *const *const c_char,
				 non_booleans: *const *const c_char);
  pub fn register_font_driver (driver: *const font_driver, f: *mut frame);
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
}

/* C 1174-1190: `add_font_name_to_list` - the `CALLBACK`/`ALIGN_STACK` convention is `extern
   "system"` plus the attribute the mirror records (`include/w32term.rs` 37-39's `ALIGN_STACK`).  */
/// C: static int CALLBACK ALIGN_STACK add_font_name_to_list (ENUMLOGFONTEX *logical_font,
/// C:   NEWTEXTMETRICEX *physical_font, DWORD font_type, LPARAM list_object)
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn add_font_name_to_list (logical_font: *mut ENUMLOGFONTEX,
						 physical_font: *mut NEWTEXTMETRICEX,
						 font_type: DWORD, list_object: LPARAM) -> c_int
{
  let list: *mut Lisp_Object = list_object as *mut Lisp_Object;		// C 1178
  let family: Lisp_Object;

  /* Skip vertical fonts (intended only for printing)  */
  if (*logical_font).elfLogFont.lfFaceName[0] == b'@' as c_char		// C 1182
    { return 1; }

  family = intern_font_name ((*logical_font).elfLogFont.lfFaceName.as_mut_ptr());	// C 1185
  if NILP (memq_no_quit (family, *list))				// C 1186
    { *list = Fcons (family, *list); }

  let _ = (physical_font, font_type);
  return 1;								// C 1189
}

/* C 1193-1194: `static int w32_decode_weight (int); static int w32_encode_weight (int);` -
   dropped (PORTING.md 0.1-12); the definitions are C 2094-2124.  */

/* C 1197-1306: `w32_enumfont_pattern_entity`.  */
/// C: static Lisp_Object w32_enumfont_pattern_entity (Lisp_Object frame, ENUMLOGFONTEX
/// C:   *logical_font, NEWTEXTMETRICEX *physical_font, DWORD font_type, LOGFONT *requested_font,
/// C:   Lisp_Object backend)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_enumfont_pattern_entity (frame: Lisp_Object, logical_font: *mut ENUMLOGFONTEX,
				       physical_font: *mut NEWTEXTMETRICEX, font_type: DWORD,
				       requested_font: *mut LOGFONT, backend: Lisp_Object)
				       -> Lisp_Object
{
  let mut entity: Lisp_Object;						// C 1204
  let mut tem: Lisp_Object;
  let lf: *mut LOGFONT = logical_font as *mut LOGFONT;			// C 1205
  let mut generic_type: BYTE;						// C 1206
  let full_type: DWORD = (*physical_font).ntmTm.ntmFlags;		// C 1207

  entity = font_make_entity ();						// C 1209

  ASET (entity, FONT_TYPE_INDEX as isize, backend);			// C 1211
  ASET (entity, FONT_REGISTRY_INDEX as isize,
	w32_registry ((*lf).lfCharSet as LONG, font_type));		// C 1212
  ASET (entity, FONT_OBJLIST_INDEX as isize, Qnil ());			// C 1213

  /* Foundry is difficult to get in readable form on Windows.
     But Emacs crashes if it is not set, so set it to something more
     generic.  These values make xlfds compatible with Emacs 22. */
  if (*lf).lfOutPrecision == OUT_STRING_PRECIS				// C 1219
    { tem = Qraster (); }
  else if (*lf).lfOutPrecision == OUT_STROKE_PRECIS
    { tem = Qoutline (); }
  else
    { tem = Qunknown (); }

  ASET (entity, FONT_FOUNDRY_INDEX as isize, tem);			// C 1227

  /* Save the generic family in the extra info, as it is likely to be
     useful to users looking for a close match.  */
  generic_type = (*physical_font).ntmTm.tmPitchAndFamily & 0xF0;	// C 1232
  if generic_type == FF_DECORATIVE						// C 1233
    { tem = Qdecorative (); }
  else if generic_type == FF_MODERN
    { tem = Qmono (); }
  else if generic_type == FF_ROMAN
    { tem = Qserif (); }
  else if generic_type == FF_SCRIPT
    { tem = Qscript (); }
  else if generic_type == FF_SWISS
    { tem = Qsans (); }
  else
    { tem = Qnil (); }

  ASET (entity, FONT_ADSTYLE_INDEX as isize, tem);			// C 1247

  if ((*physical_font).ntmTm.tmPitchAndFamily & 0x01) != 0		// C 1249
    { ASET (entity, FONT_SPACING_INDEX as isize,
	    make_fixnum (FONT_SPACING_PROPORTIONAL as EMACS_INT)); }
  else
    { ASET (entity, FONT_SPACING_INDEX as isize,
	    make_fixnum (FONT_SPACING_CHARCELL as EMACS_INT)); }

  if (*requested_font).lfQuality != DEFAULT_QUALITY			// C 1254
    {
      font_put_extra (entity, QCantialias (),			// C 1256
		      lispy_antialias_type ((*requested_font).lfQuality));
    }
  ASET (entity, FONT_FAMILY_INDEX as isize,				// C 1259
	intern_font_name ((*lf).lfFaceName.as_mut_ptr ()));

  FONT_SET_STYLE! (entity, FONT_WEIGHT_INDEX,				// C 1261
		   make_fixnum (w32_decode_weight ((*lf).lfWeight) as EMACS_INT));
  FONT_SET_STYLE! (entity, FONT_SLANT_INDEX,				// C 1263
		   make_fixnum (if (*lf).lfItalic != 0 { 200 } else { 100 }));
  /* TODO: PANOSE struct has this info, but need to call GetOutlineTextMetrics
     to get it.  */
  FONT_SET_STYLE! (entity, FONT_WIDTH_INDEX, make_fixnum (100));	// C 1266

  if (font_type & RASTER_FONTTYPE) != 0					// C 1268
    {
      ASET (entity, FONT_SIZE_INDEX as isize,
	    make_fixnum (((*physical_font).ntmTm.tmHeight
			  + (*physical_font).ntmTm.tmExternalLeading) as EMACS_INT));
    }
  else
    { ASET (entity, FONT_SIZE_INDEX as isize, make_fixnum (0)); }	// C 1272

  /* Cache Unicode codepoints covered by this font, as there is no other way
     of getting this information easily.  */
  if (font_type & TRUETYPE_FONTTYPE) != 0				// C 1276
    {
      tem = font_supported_scripts (&mut (*physical_font).ntmFontSig);	// C 1278
      if ! NILP (tem)
	{ font_put_extra (entity, QCscript (), tem); }			// C 1280
    }

  /* This information is not fully available when opening fonts, so
     save it here.  Only Windows 2000 and later return information
     about opentype and type1 fonts, so need a fallback for detecting
     truetype so that this information is not any worse than we could
     have obtained later.  */
  if EQ (backend, Quniscribe ()) && (full_type & NTMFLAGS_OPENTYPE) != 0	// C 1288
    { tem = Qopentype (); }
  else if (font_type & TRUETYPE_FONTTYPE) != 0
    { tem = Qtruetype (); }
  else if (full_type & NTM_PS_OPENTYPE) != 0
    { tem = Qpostscript (); }
  else if (full_type & NTM_TYPE1) != 0
    { tem = Qtype1 (); }
  else if (font_type & RASTER_FONTTYPE) != 0
    { tem = Qw32bitmap (); }
  else
    { tem = Qw32vector (); }

  font_put_extra (entity, QCformat (), tem);				// C 1301

  let _ = frame;
  return entity;							// C 1303
}

/* C 1308-1324: `w32_generic_family` - "Convert generic families to the family portion of
   lfPitchAndFamily".  */
/// C: static BYTE w32_generic_family (Lisp_Object name)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_generic_family (name: Lisp_Object) -> BYTE
{
  /* Generic families.  */
  if EQ (name, Qmonospace ()) || EQ (name, Qmono ())			// C 1312
    { return FF_MODERN; }
  else if EQ (name, Qsans ()) || EQ (name, Qsans_serif ()) || EQ (name, Qsansserif ())
    { return FF_SWISS; }						// C 1314
  else if EQ (name, Qserif ())
    { return FF_ROMAN; }						// C 1316
  else if EQ (name, Qdecorative ())
    { return FF_DECORATIVE; }						// C 1318
  else if EQ (name, Qscript ())
    { return FF_SCRIPT; }						// C 1320
  else
    { return FF_DONTCARE; }						// C 1322
}

/* C 1326-1344: `logfonts_match`.  */
/// C: static int logfonts_match (LOGFONT *font, LOGFONT *pattern)
#[cfg (feature = "have_ntgui")]
unsafe fn logfonts_match (font: *mut LOGFONT, pattern: *mut LOGFONT) -> c_int
{
  /* Only check height for raster fonts.  */
  if (*pattern).lfHeight != 0						// C 1331
      && (*font).lfOutPrecision == OUT_STRING_PRECIS
      && (*font).lfHeight != (*pattern).lfHeight
    { return 0; }

  /* Have some flexibility with weights.  */
  if (*pattern).lfWeight != 0						// C 1336
      && (((*font).lfWeight < ((*pattern).lfWeight - 150))
	  || (*font).lfWeight > ((*pattern).lfWeight + 150))
    { return 0; }

  /* Charset and face should be OK.  Italic has to be checked
     against the original spec, in case we don't have any preference.  */
  return 1;								// C 1342
}

/* C 1346-1348: "Codepage Bitfields in FONTSIGNATURE struct."  */
/// C: #define CSB_JAPANESE (1 << 17)
/// C: #define CSB_KOREAN ((1 << 19) | (1 << 21))
/// C: #define CSB_CHINESE ((1 << 18) | (1 << 20))
#[cfg (feature = "have_ntgui")]
pub const CSB_JAPANESE: DWORD = 1 << 17;
#[cfg (feature = "have_ntgui")]
pub const CSB_KOREAN: DWORD = (1 << 19) | (1 << 21);
#[cfg (feature = "have_ntgui")]
pub const CSB_CHINESE: DWORD = (1 << 18) | (1 << 20);

/* C 1350-1544: `font_matches_spec`.  */
/// C: static int font_matches_spec (DWORD type, NEWTEXTMETRICEX *font, Lisp_Object spec,
/// C:                               Lisp_Object backend, LOGFONT *logfont)
#[cfg (feature = "have_ntgui")]
unsafe fn font_matches_spec (r#type: DWORD, font: *mut NEWTEXTMETRICEX, spec: Lisp_Object,
			     backend: Lisp_Object, logfont: *mut LOGFONT) -> c_int
{
  let mut extra: Lisp_Object;						// C 1355
  let mut val: Lisp_Object;

  /* Check italic. Can't check logfonts, since it is a boolean field,
     so there is no difference between "non-italic" and "don't care".  */
  {
    let slant: EMACS_INT = FONT_SLANT_NUMERIC! (spec);			// C 1361

    if slant >= 0							// C 1363
	&& ((slant > 150 && (*font).ntmTm.tmItalic == 0)
	    || (slant <= 150 && (*font).ntmTm.tmItalic != 0))
      { return 0; }
  }

  /* Check adstyle against generic family.  */
  val = AREF (spec, FONT_ADSTYLE_INDEX as isize);			// C 1371
  if ! NILP (val)
    {
      let family: BYTE = w32_generic_family (val);
      if family != FF_DONTCARE
	  && family != ((*font).ntmTm.tmPitchAndFamily & 0xF0)
	{ return 0; }
    }

  /* Check spacing */
  val = AREF (spec, FONT_SPACING_INDEX as isize);			// C 1381
  if FIXNUMP (val)
    {
      let spacing: c_int = XFIXNUM (val) as c_int;
      let proportional: bool = spacing < FONT_SPACING_MONO;

      if (proportional && ((*font).ntmTm.tmPitchAndFamily & 0x01) == 0)	// C 1387
	  || (! proportional && ((*font).ntmTm.tmPitchAndFamily & 0x01) != 0)
	{ return 0; }
    }

  /* Check extra parameters.  */
  extra = AREF (spec, FONT_EXTRA_INDEX as isize);			// C 1393
  while CONSP (extra)
    {
      let extra_entry: Lisp_Object = XCAR (extra);
      if CONSP (extra_entry)
	{
	  let key: Lisp_Object = XCAR (extra_entry);

	  val = XCDR (extra_entry);
	  if EQ (key, QCscript ()) && SYMBOLP (val)			// C 1404
	    {
	      /* Only truetype fonts will have information about what
		 scripts they support.  This probably means the user
		 will have to force Emacs to use raster, PostScript
		 or ATM fonts for non-ASCII text.  */
	      if (r#type & TRUETYPE_FONTTYPE) != 0			// C 1410
		{
		  let support: Lisp_Object
		    = font_supported_scripts (&raw mut (*font).ntmFontSig);
		  if NILP (memq_no_quit (val, support))
		    { return 0; }

		  /* Avoid using non-Japanese fonts for Japanese, even
		     if they claim they are capable, due to known
		     breakage in Vista and Windows 7 fonts
		     (bug#6029).  */
		  if EQ (val, Qkana ())					// C 1423
		      && ((*font).ntmTm.tmCharSet != SHIFTJIS_CHARSET
			  || ((*font).ntmFontSig.fsCsb[0] & CSB_JAPANESE) == 0)
		    { return 0; }
		}
	      else
		{
		  /* Return specific matches, but play it safe. Fonts
		     that cover more than their charset would suggest
		     are likely to be truetype or opentype fonts,
		     covered above.  */
		  if EQ (val, Qlatin ())				// C 1436
		    {
		      /* Although every charset but symbol, thai and
			 arabic contains the basic ASCII set of latin
			 characters, Emacs expects much more.  */
		      if (*font).ntmTm.tmCharSet != ANSI_CHARSET
			{ return 0; }
		    }
		  else if EQ (val, Qsymbol ())				// C 1443
		    {
		      if (*font).ntmTm.tmCharSet != SYMBOL_CHARSET
			{ return 0; }
		    }
		  else if EQ (val, Qcyrillic ())			// C 1448
		    {
		      if (*font).ntmTm.tmCharSet != RUSSIAN_CHARSET
			{ return 0; }
		    }
		  else if EQ (val, Qgreek ())				// C 1453
		    {
		      if (*font).ntmTm.tmCharSet != GREEK_CHARSET
			{ return 0; }
		    }
		  else if EQ (val, Qarabic ())				// C 1458
		    {
		      if (*font).ntmTm.tmCharSet != ARABIC_CHARSET
			{ return 0; }
		    }
		  else if EQ (val, Qhebrew ())				// C 1463
		    {
		      if (*font).ntmTm.tmCharSet != HEBREW_CHARSET
			{ return 0; }
		    }
		  else if EQ (val, Qthai ())				// C 1468
		    {
		      if (*font).ntmTm.tmCharSet != THAI_CHARSET
			{ return 0; }
		    }
		  else if EQ (val, Qkana ())				// C 1473
		    {
		      if (*font).ntmTm.tmCharSet != SHIFTJIS_CHARSET
			{ return 0; }
		    }
		  else if EQ (val, Qbopomofo ())			// C 1478
		    {
		      if (*font).ntmTm.tmCharSet != CHINESEBIG5_CHARSET
			{ return 0; }
		    }
		  else if EQ (val, Qhangul ())				// C 1483
		    {
		      if (*font).ntmTm.tmCharSet != HANGUL_CHARSET
			  && (*font).ntmTm.tmCharSet != JOHAB_CHARSET
			{ return 0; }
		    }
		  else if EQ (val, Qhan ())				// C 1489
		    {
		      if (*font).ntmTm.tmCharSet != CHINESEBIG5_CHARSET
			  && (*font).ntmTm.tmCharSet != GB2312_CHARSET
			  && (*font).ntmTm.tmCharSet != HANGUL_CHARSET
			  && (*font).ntmTm.tmCharSet != JOHAB_CHARSET
			  && (*font).ntmTm.tmCharSet != SHIFTJIS_CHARSET
			{ return 0; }
		    }
		  else
		    /* Other scripts unlikely to be handled by non-truetype
		       fonts.  */
		    { return 0; }
		}
	    }
	  else if EQ (key, QClang ()) && SYMBOLP (val)			// C 1500
	    {
	      /* Just handle the CJK languages here, as the lang
		 parameter is used to select a font with appropriate
		 glyphs in the cjk unified ideographs block. Other fonts
	         support for a language can be solely determined by
	         its character coverage.  */
	      if EQ (val, Qja ())					// C 1508
		{
		  if ((*font).ntmFontSig.fsCsb[0] & CSB_JAPANESE) == 0
		    { return 0; }
		}
	      else if EQ (val, Qko ())					// C 1513
		{
		  if ((*font).ntmFontSig.fsCsb[0] & CSB_KOREAN) == 0
		    { return 0; }
		}
	      else if EQ (val, Qzh ())					// C 1518
		{
		  if ((*font).ntmFontSig.fsCsb[0] & CSB_CHINESE) == 0
		    { return 0; }
		}
	      else
		/* Any other language, we don't recognize it. Only the above
                   currently appear in fontset.el, so it isn't worth
                   creating a mapping table of codepages/scripts to languages
                   or opening the font to see if there are any language tags
                   in it that the Windows API does not expose. Fontset
		   spec should have a fallback, as some backends do
		   not recognize language at all.  */
		{ return 0; }
	    }
	  else if EQ (key, QCotf ()) && CONSP (val)			// C 1527
	    {
	      /* OTF features only supported by the uniscribe backend.  */
	      if EQ (backend, Quniscribe ())				// C 1530
		{
		  if uniscribe_check_otf (logfont, val) == 0
		    { return 0; }
		}
	      else
		{ return 0; }
	    }
	}
      extra = XCDR (extra);						// C 1393's `extra = XCDR (extra)`
    }
  return 1;								// C 1542
}

/* C 1546-1578: `w32font_coverage_ok` and its four masks.  */
/// C: static int w32font_coverage_ok (FONTSIGNATURE * coverage, BYTE charset)
#[cfg (feature = "have_ntgui")]
unsafe fn w32font_coverage_ok (coverage: *mut FONTSIGNATURE, charset: BYTE) -> c_int
{
  let subrange1: DWORD = (*coverage).fsUsb[1];				// C 1549
  let codepages0: DWORD = (*coverage).fsCsb[0];				// C 1550

  /* We consider the coverage to be OK if either (a) subrange1 has the
     bits set that correspond to CHARSET, or (b) subrange1 indicates SIP
     support and codepages0 has one or more bits set corresponding to
     CHARSET.  */
  if charset == GB2312_CHARSET || charset == CHINESEBIG5_CHARSET	// C 1563
    {
      return if ((subrange1 & SUBRANGE1_HAN_MASK) == SUBRANGE1_HAN_MASK
		 || (((subrange1 & SUBRANGE1_SIP_MASK) != 0)
		     && ((codepages0 & CSB_CHINESE) != 0)))
	       { 1 } else { 0 };
    }
  else if charset == SHIFTJIS_CHARSET					// C 1569
    {
      return if ((subrange1 & SUBRANGE1_JAPANESE_MASK) == SUBRANGE1_JAPANESE_MASK
		 || (((subrange1 & SUBRANGE1_SIP_MASK) != 0)
		     && ((codepages0 & CSB_JAPANESE) != 0)))
	       { 1 } else { 0 };
    }
  else if charset == HANGEUL_CHARSET					// C 1575
    {
      return if ((subrange1 & SUBRANGE1_HANGEUL_MASK) == SUBRANGE1_HANGEUL_MASK
		 || (((subrange1 & SUBRANGE1_SIP_MASK) != 0)
		     && ((codepages0 & CSB_KOREAN) != 0)))
	       { 1 } else { 0 };
    }

  return 1;								// C 1578
}
/// C: #define SUBRANGE1_HAN_MASK 0x08000000
/// C: #define SUBRANGE1_HANGEUL_MASK 0x01000000
/// C: #define SUBRANGE1_JAPANESE_MASK (0x00060000 | SUBRANGE1_HAN_MASK)
/// C: #define SUBRANGE1_SIP_MASK 0x02000000
#[cfg (feature = "have_ntgui")]
pub const SUBRANGE1_HAN_MASK: DWORD = 0x08000000;
#[cfg (feature = "have_ntgui")]
pub const SUBRANGE1_HANGEUL_MASK: DWORD = 0x01000000;
#[cfg (feature = "have_ntgui")]
pub const SUBRANGE1_JAPANESE_MASK: DWORD = 0x00060000 | SUBRANGE1_HAN_MASK;
#[cfg (feature = "have_ntgui")]
pub const SUBRANGE1_SIP_MASK: DWORD = 0x02000000;

/* C 1583-1585: `#ifndef WINDOWSNT` / `#define _strlwr strlwr` / `#endif` - "the name Cygwin's
   `<string.h>` carries".  The MinGW arm takes ms-w32.h's `_strlwr`.  */
/// C: #define _strlwr strlwr
#[cfg (all (feature = "have_ntgui", not (feature = "windowsnt")))]
macro_rules! _strlwr { ($($a:tt)*) => { strlwr ($($a)*) }; }
/// C: `<string.h>`: `char *strlwr (char *);` - the Cygwin entry the define above renames.
#[cfg (all (feature = "have_ntgui", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn strlwr (s: *mut c_char) -> *mut c_char;
}

/// C: `<string.h>` (the Windows SDK): `char *_strlwr (char *);` - the MinGW arm's spelling of
/// C 1594/1601/1607's `_strlwr` call, which ms-w32.h does not rename.
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "C" {
  pub fn _strlwr (s: *mut c_char) -> *mut c_char;
}
/// The call shape C's text writes (`_strlwr (full_iname)`); the macro body is the function
/// call, so both arms of the file's `#ifdef WINDOWSNT` reach the same name.
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
macro_rules! _strlwr { ($s:expr) => { _strlwr ($s) }; }

/* C 1587-1630: `check_face_name`.  C's `char full_iname[LF_FULLFACESIZE+1];` is the port's
   array; `strstr ("helvetica", full_iname) != NULL` is `! is_null ()`.  */
/// C: static int check_face_name (LOGFONT *font, char *full_name)
#[cfg (feature = "have_ntgui")]
unsafe fn check_face_name (font: *mut LOGFONT, full_name: *mut c_char) -> c_int
{
  let mut full_iname: [c_char; (LF_FULLFACESIZE + 1) as usize] = [0; (LF_FULLFACESIZE + 1) as usize];

  /* Just check for names known to cause problems, since the full name
     can contain expanded abbreviations, prefixed foundry, postfixed
     style, the latter of which sometimes differs from the style indicated
     in the shorter name (eg Lt becomes Light or even Extra Light)  */

  /* Helvetica is mapped to Arial in Windows, but if a Type-1 Helvetica is
     installed, we run into problems with the Uniscribe backend which tries
     to avoid non-truetype fonts, and ends up mixing the Type-1 Helvetica
     with Arial's characteristics, since that attempt to use TrueType works
     some places, but not others.  */
  if xstrcasecmp ((*font).lfFaceName.as_ptr (), c"helvetica".as_ptr ()) == 0	// C 1601
    {
      strncpy (full_iname.as_mut_ptr (), full_name, LF_FULLFACESIZE as usize);	// C 1603
      full_iname[LF_FULLFACESIZE as usize] = 0;				// C 1604
      _strlwr! (full_iname.as_mut_ptr ());				// C 1605
      return if ! strstr (c"helvetica".as_ptr (), full_iname.as_mut_ptr ()).is_null ()
	       { 1 } else { 0 };					// C 1606
    }
  /* Same for Helv.  */
  if xstrcasecmp ((*font).lfFaceName.as_ptr (), c"helv".as_ptr ()) == 0	// C 1609
    {
      strncpy (full_iname.as_mut_ptr (), full_name, LF_FULLFACESIZE as usize);	// C 1611
      full_iname[LF_FULLFACESIZE as usize] = 0;				// C 1612
      _strlwr! (full_iname.as_mut_ptr ());				// C 1613
      return if ! strstr (c"helv".as_ptr (), full_iname.as_mut_ptr ()).is_null ()
	       { 1 } else { 0 };					// C 1614
    }

  /* Since Times is mapped to Times New Roman, a substring
     match is not sufficient to filter out the bogus match.  */
  else if xstrcasecmp ((*font).lfFaceName.as_ptr (), c"times".as_ptr ()) == 0	// C 1618
    { return if xstrcasecmp (full_name, c"times".as_ptr ()) == 0 { 1 } else { 0 }; }
									// C 1619
  return 1;								// C 1621
}

/* C 1632-1773: `add_font_entity_to_list`.  */
/// C: static int CALLBACK ALIGN_STACK add_font_entity_to_list (ENUMLOGFONTEX *logical_font,
/// C:   NEWTEXTMETRICEX *physical_font, DWORD font_type, LPARAM lParam)
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn add_font_entity_to_list (logical_font: *mut ENUMLOGFONTEX,
						   physical_font: *mut NEWTEXTMETRICEX,
						   font_type: DWORD, lParam: LPARAM) -> c_int
{
  let match_data: *mut font_callback_data = lParam as *mut font_callback_data;	// C 1637
  let backend: Lisp_Object = if (*match_data).opentype_only { Quniscribe () } else { Qgdi () };
  let mut entity: Lisp_Object;						// C 1640

  let is_unicode: c_int = ((*physical_font).ntmFontSig.fsUsb[3] != 0	// C 1642
			   || (*physical_font).ntmFontSig.fsUsb[2] != 0
			   || (*physical_font).ntmFontSig.fsUsb[1] != 0
			   || ((*physical_font).ntmFontSig.fsUsb[0] & 0x3fffffff) != 0)
			  as c_int;

  /* Kludgey fix for Arial Unicode MS font that claims support for
     scripts it doesn't actually cover.  */
  if strncmp ((*logical_font).elfLogFont.lfFaceName.as_ptr (), c"Arial Unicode MS".as_ptr (),
	      16) == 0							// C 1649
    {
      /* Reset bits 4 (Phonetic), 12 (Vai), 14 (Nko), 27 (Balinese).  */
      (*physical_font).ntmFontSig.fsUsb[0] &= 0xf7ffafef;		// C 1652
      /* Reset bits 53 (Phags-pa) and 58 (Phoenician).  */
      (*physical_font).ntmFontSig.fsUsb[1] &= 0xfbdfffff;		// C 1654
      /* Set bit 70 (Tibetan).  */
      (*physical_font).ntmFontSig.fsUsb[2] |= 0x00000040;		// C 1656
    }

  /* Skip non matching fonts.  */

  /* For uniscribe backend, consider only truetype or opentype fonts
     that have some Unicode coverage.  */
  if (*match_data).opentype_only					// C 1662
      && ((((*physical_font).ntmTm.ntmFlags & NTMFLAGS_OPENTYPE) == 0
	   && (font_type & TRUETYPE_FONTTYPE) == 0)
	  || is_unicode == 0)
    { return 1; }

  /* Ensure a match.  */
  if logfonts_match (&mut (*logical_font).elfLogFont, &mut (*match_data).pattern) == 0	// C 1669
      || ! (font_matches_spec (font_type, physical_font,
			       (*match_data).orig_font_spec, backend,
			       &mut (*logical_font).elfLogFont) != 0
	    || (! NILP ((*match_data).known_fonts)
		&& ! NILP (memq_no_quit
			   (intern_font_name ((*logical_font).elfLogFont.lfFaceName.as_mut_ptr ()),
			    (*match_data).known_fonts))))
      || w32font_coverage_ok (&mut (*physical_font).ntmFontSig,
			      (*match_data).pattern.lfCharSet) == 0
    { return 1; }

  /* Avoid substitutions involving raster fonts (eg Helv -> MS Sans Serif)
     We limit this to raster fonts, because the test can catch some
     genuine fonts (eg the full name of DejaVu Sans Mono Light is actually
     DejaVu Sans Mono ExtraLight). Helvetica -> Arial substitution will
     therefore get through this test.  Since full names can be prefixed
     by a foundry, we accept raster fonts if the font name is found
     anywhere within the full name.  */
  if ((*logical_font).elfLogFont.lfOutPrecision == OUT_STRING_PRECIS	// C 1694
      && strstr ((*logical_font).elfFullName.as_mut_ptr (),
		 (*logical_font).elfLogFont.lfFaceName.as_ptr ()).is_null ()
      /* Check for well known substitutions that mess things up in the
	 presence of Type-1 fonts of the same name.  */
      || (check_face_name (&mut (*logical_font).elfLogFont,
			   (*logical_font).elfFullName.as_mut_ptr ()) == 0))
    { return 1; }

  /* Make a font entity for the font.  */
  entity = w32_enumfont_pattern_entity ((*match_data).frame, logical_font,	// C 1703
					physical_font, font_type,
					&mut (*match_data).pattern,
					backend);

  if ! NILP (entity)							// C 1708
    {
      let spec_charset: Lisp_Object = AREF ((*match_data).orig_font_spec,
					    FONT_REGISTRY_INDEX as isize);

      /* iso10646-1 fonts must contain Unicode mapping tables.  */
      if EQ (spec_charset, Qiso10646_1 ())				// C 1715
	{
	  if is_unicode == 0
	    { return 1; }
	}
      /* unicode-bmp fonts must contain characters from the BMP.  */
      else if EQ (spec_charset, Qunicode_bmp ())			// C 1720
	{
	  if (*physical_font).ntmFontSig.fsUsb[3] == 0
	      && ((*physical_font).ntmFontSig.fsUsb[2] & 0xFFFFFF9E) == 0
	      && ((*physical_font).ntmFontSig.fsUsb[1] & 0xE81FFFFF) == 0
	      && ((*physical_font).ntmFontSig.fsUsb[0] & 0x007F001F) == 0
	    { return 1; }
	}
      /* unicode-sip fonts must contain characters in Unicode plane 2.
	 so look for bit 57 (surrogates) in the Unicode subranges, plus
	 the bits for CJK ranges that include those characters or CJK
	 bits in code-page bit fields..  */
      else if EQ (spec_charset, Qunicode_sip ())			// C 1732
	{
	  if ! (((*physical_font).ntmFontSig.fsUsb[1] & 0x02000000) != 0
		&& ((((*physical_font).ntmFontSig.fsUsb[1] & 0x28000000) != 0)
		    /* Some CJK fonts with very good coverage of SIP
                       characters have only the 0x02000000 bit in USB
                       set, so we allow them if their code-page bits
                       indicate support for CJK character sets.  */
		    || (((*physical_font).ntmFontSig.fsCsb[0]
			 & (CSB_CHINESE | CSB_JAPANESE | CSB_KOREAN)) != 0)))
	    { return 1; }
	}

      /* This font matches.  */

      /* If registry was specified, ensure it is reported as the same.  */
      if ! NILP (spec_charset)						// C 1752
	{
	  /* Avoid using non-Japanese fonts for Japanese, even if they
	     claim they are capable, due to known breakage in Vista
	     and Windows 7 fonts (bug#6029).  */
	  if (*logical_font).elfLogFont.lfCharSet == SHIFTJIS_CHARSET	// C 1757
	      && ((*physical_font).ntmFontSig.fsCsb[0] & CSB_JAPANESE) == 0
	    { return 1; }
	  else
	    { ASET (entity, FONT_REGISTRY_INDEX as isize, spec_charset); }	// C 1760
	}
      /* Otherwise if using the uniscribe backend, report ANSI and DEFAULT
	 fonts as Unicode and skip other charsets.  */
      else if (*match_data).opentype_only				// C 1763
	{
	  if (*logical_font).elfLogFont.lfCharSet == ANSI_CHARSET
	      || (*logical_font).elfLogFont.lfCharSet == DEFAULT_CHARSET
	    { ASET (entity, FONT_REGISTRY_INDEX as isize, Qiso10646_1 ()); }	// C 1767
	  else
	    { return 1; }
	}

      /* Add this font to the list.  */
      (*match_data).list = Fcons (entity, (*match_data).list);		// C 1771
    }
  return 1;								// C 1773
}

/* C 1775-1787: `add_one_font_entity_to_list` - "Terminates the search once we have a match".  */
/// C: static int CALLBACK ALIGN_STACK add_one_font_entity_to_list (ENUMLOGFONTEX *logical_font,
/// C:   NEWTEXTMETRICEX *physical_font, DWORD font_type, LPARAM lParam)
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn add_one_font_entity_to_list (logical_font: *mut ENUMLOGFONTEX,
						       physical_font: *mut NEWTEXTMETRICEX,
						       font_type: DWORD, lParam: LPARAM) -> c_int
{
  let match_data: *mut font_callback_data = lParam as *mut font_callback_data;	// C 1780
  add_font_entity_to_list (logical_font, physical_font, font_type, lParam);	// C 1781

  /* If we have a font in the list, terminate the search.  */
  return NILP ((*match_data).list) as c_int;				// C 1784
}

/* C 1789-1869: `x_to_w32_charset` - "Old function to convert from x to w32 charset, from
   w32fns.c".  C's `alloca (len + 1)` is the port's stack pool (P-05).  */
/// C: static LONG x_to_w32_charset (char * lpcs)
#[cfg (feature = "have_ntgui")]
unsafe fn x_to_w32_charset (lpcs: *mut c_char) -> LONG
{
  let mut this_entry: Lisp_Object;					// C 1791
  let mut w32_charset: Lisp_Object;
  let charset: *mut c_char;						// C 1792
  let len: c_int = strlen (lpcs) as c_int;				// C 1793
  let mut lpcs = lpcs;							// C 1804 rebinds it

  /* Support "*-#nnn" format for unknown charsets.  */
  if strncmp (lpcs, c"*-#".as_ptr (), 3) == 0				// C 1796
    { return atoi (lpcs.offset (3)); }

  /* All Windows fonts qualify as Unicode.  */
  if strncmp (lpcs, c"iso10646".as_ptr (), 8) == 0			// C 1800
    { return DEFAULT_CHARSET as LONG; }

  /* Handle wildcards by ignoring them; eg. treat "big5*-*" as "big5".  */
  charset = sa_stack_alloc ((len + 1) as usize) as *mut c_char;		// C 1804
  strcpy (charset, lpcs);						// C 1805
  lpcs = strchr (charset, b'*' as c_int);				// C 1806
  if ! lpcs.is_null ()
    { *lpcs = 0; }							// C 1808

  /* Look through w32-charset-info-alist for the character set.
     Format of each entry is
       (CHARSET_NAME . (WINDOWS_CHARSET . CODEPAGE)).
  */
  this_entry = Fassoc (build_string (charset), Vw32_charset_info_alist, Qnil ());	// C 1815

  if NILP (this_entry)							// C 1817
    {
      /* At startup, we want iso8859-1 fonts to come up properly. */
      if xstrcasecmp (charset, c"iso8859-1".as_ptr ()) == 0		// C 1820
        { return ANSI_CHARSET as LONG; }
      else
        { return DEFAULT_CHARSET as LONG; }
    }

  w32_charset = Fcar (Fcdr (this_entry));				// C 1825

  /* Translate Lisp symbol to number.  */
  if EQ (w32_charset, Qw32_charset_ansi ())		{ return ANSI_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_symbol ())		{ return SYMBOL_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_shiftjis ())		{ return SHIFTJIS_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_hangeul ())		{ return HANGEUL_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_chinesebig5 ())	{ return CHINESEBIG5_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_gb2312 ())		{ return GB2312_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_oem ())		{ return OEM_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_johab ())		{ return JOHAB_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_easteurope ())	{ return EASTEUROPE_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_turkish ())		{ return TURKISH_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_baltic ())		{ return BALTIC_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_russian ())		{ return RUSSIAN_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_arabic ())		{ return ARABIC_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_greek ())		{ return GREEK_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_hebrew ())		{ return HEBREW_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_vietnamese ())	{ return VIETNAMESE_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_thai ())		{ return THAI_CHARSET as LONG; }
  if EQ (w32_charset, Qw32_charset_mac ())		{ return MAC_CHARSET as LONG; }

  return DEFAULT_CHARSET as LONG;					// C 1867
}

/* C 1871-1884: `registry_to_w32_charset` - "Convert a Lisp font registry (symbol) to a windows
   charset".  */
/// C: static LONG registry_to_w32_charset (Lisp_Object charset)
#[cfg (feature = "have_ntgui")]
unsafe fn registry_to_w32_charset (charset: Lisp_Object) -> LONG
{
  if EQ (charset, Qiso10646_1 ()) || EQ (charset, Qunicode_bmp ())	// C 1874
      || EQ (charset, Qunicode_sip ())
    { return DEFAULT_CHARSET as LONG; }	/* UNICODE_CHARSET not defined in MingW32 */	// C 1876
  else if EQ (charset, Qiso8859_1 ())
    { return ANSI_CHARSET as LONG; }					// C 1878
  else if SYMBOLP (charset)
    { return x_to_w32_charset (SSDATA (SYMBOL_NAME (charset))); }	// C 1880
  else
    { return DEFAULT_CHARSET as LONG; }					// C 1882
}

/* C 1886-2078: `w32_to_x_charset` - "Old function to convert from w32 to x charset, from
   w32fns.c".  C's `switch` on an `int` with `BYTE` case labels is the port's guarded `match`
   (a Rust pattern cannot name a `BYTE` constant against an `int` scrutinee; `coding.rs` 3790
   spells the same shape), and every arm's body and order is C's.  */
/// C: static char * w32_to_x_charset (int fncharset, char *matching)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_to_x_charset (fncharset: c_int, matching: *mut c_char) -> *mut c_char
{
  static mut buf: [c_char; 32] = [0; 32];				// C 1889
  let mut charset_type: Lisp_Object = Qnil ();				// C 1890
  let mut match_len: c_int = 0;						// C 1891

  if ! matching.is_null ()						// C 1893
    {
      /* If fully specified, accept it as it is.  Otherwise use a
	 substring match. */
      let wildcard: *mut c_char = strchr (matching, b'*' as c_int);	// C 1897
      if ! wildcard.is_null ()
	{ *wildcard = 0; }
      else if ! strchr (matching, b'-' as c_int).is_null ()
	{ return matching; }

      match_len = strlen (matching) as c_int;				// C 1902
    }

  match fncharset							// C 1905
    {
      _ if fncharset == ANSI_CHARSET as c_int =>			// C 1907
	{
	  /* Handle startup case of w32-charset-info-alist not
             being set up yet. */
	  if NILP (Vw32_charset_info_alist)
	    { return c"iso8859-1".as_ptr () as *mut c_char; }
	  charset_type = Qw32_charset_ansi ();
	}
      _ if fncharset == DEFAULT_CHARSET as c_int =>			// C 1912
	{ charset_type = Qw32_charset_default (); }
      _ if fncharset == SYMBOL_CHARSET as c_int =>			// C 1915
	{ charset_type = Qw32_charset_symbol (); }
      _ if fncharset == SHIFTJIS_CHARSET as c_int =>			// C 1918
	{ charset_type = Qw32_charset_shiftjis (); }
      _ if fncharset == HANGEUL_CHARSET as c_int =>			// C 1921
	{ charset_type = Qw32_charset_hangeul (); }
      _ if fncharset == GB2312_CHARSET as c_int =>			// C 1924
	{ charset_type = Qw32_charset_gb2312 (); }
      _ if fncharset == CHINESEBIG5_CHARSET as c_int =>			// C 1927
	{ charset_type = Qw32_charset_chinesebig5 (); }
      _ if fncharset == OEM_CHARSET as c_int =>				// C 1930
	{ charset_type = Qw32_charset_oem (); }
      _ if fncharset == EASTEUROPE_CHARSET as c_int =>			// C 1933
	{ charset_type = Qw32_charset_easteurope (); }
      _ if fncharset == TURKISH_CHARSET as c_int =>			// C 1936
	{ charset_type = Qw32_charset_turkish (); }
      _ if fncharset == BALTIC_CHARSET as c_int =>			// C 1939
	{ charset_type = Qw32_charset_baltic (); }
      _ if fncharset == RUSSIAN_CHARSET as c_int =>			// C 1942
	{ charset_type = Qw32_charset_russian (); }
      _ if fncharset == ARABIC_CHARSET as c_int =>			// C 1945
	{ charset_type = Qw32_charset_arabic (); }
      _ if fncharset == GREEK_CHARSET as c_int =>			// C 1948
	{ charset_type = Qw32_charset_greek (); }
      _ if fncharset == HEBREW_CHARSET as c_int =>			// C 1951
	{ charset_type = Qw32_charset_hebrew (); }
      _ if fncharset == VIETNAMESE_CHARSET as c_int =>			// C 1954
	{ charset_type = Qw32_charset_vietnamese (); }
      _ if fncharset == THAI_CHARSET as c_int =>			// C 1957
	{ charset_type = Qw32_charset_thai (); }
      _ if fncharset == MAC_CHARSET as c_int =>				// C 1960
	{ charset_type = Qw32_charset_mac (); }
      _ if fncharset == JOHAB_CHARSET as c_int =>			// C 1963
	{ charset_type = Qw32_charset_johab (); }

      _ =>								// C 1966
	{
	  /* Encode numerical value of unknown charset.  */
	  sprintf (buf.as_mut_ptr (), c"*-#%d".as_ptr (), fncharset);	// C 1968
	  return buf.as_mut_ptr ();
	}
    }

  {
    let mut rest: Lisp_Object;						// C 1973
    let mut best_match: *mut c_char = std::ptr::null_mut ();		// C 1974
    let mut matching_found: c_int = 0;					// C 1975

    /* Look through w32-charset-info-alist for the character set.
       Prefer ISO codepages, and prefer lower numbers in the ISO
       range. Only return charsets for codepages which are installed.

       Format of each entry is
         (CHARSET_NAME . (WINDOWS_CHARSET . CODEPAGE)).
    */
    rest = Vw32_charset_info_alist;					// C 1985
    while CONSP (rest)
      {
	let x_charset: *mut c_char;
	let w32_charset: Lisp_Object;
	let codepage: Lisp_Object;

	let this_entry: Lisp_Object = XCAR (rest);			// C 1991

	/* Skip invalid entries in alist. */
	if ! (CONSP (this_entry) && STRINGP (XCAR (this_entry))		// C 1994
	      && CONSP (XCDR (this_entry))
	      && SYMBOLP (XCAR (XCDR (this_entry))))
	  { rest = XCDR (rest); continue; }

	x_charset = SSDATA (XCAR (this_entry));				// C 2000
	w32_charset = XCAR (XCDR (this_entry));				// C 2001
	codepage = XCDR (XCDR (this_entry));				// C 2002

	/* Look for Same charset and a valid codepage (or non-int
           which means ignore).  */
	if EQ (w32_charset, charset_type)				// C 2006
	    && (! FIXNUMP (codepage) || XFIXNUM (codepage) == CP_DEFAULT as EMACS_INT
		|| IsValidCodePage (XFIXNUM (codepage) as UINT) != 0)
	  {
	    /* If we don't have a match already, then this is the
               best.  */
	    if best_match.is_null ()					// C 2013
	      {
		best_match = x_charset;
		if ! matching.is_null ()
		    && strnicmp (x_charset, matching, match_len as usize) == 0
		  { matching_found = 1; }
	      }
	    /* If we already found a match for MATCHING, then
	       only consider other matches.  */
	    else if matching_found != 0					// C 2020
		     && strnicmp (x_charset, matching, match_len as usize) != 0
	      { rest = XCDR (rest); continue; }
	    /* If this matches what we want, and the best so far doesn't,
	       then this is better.  */
	    else if matching_found == 0 && ! matching.is_null ()	// C 2026
		     && strnicmp (x_charset, matching, match_len as usize) == 0
	      {
		best_match = x_charset;
		matching_found = 1;
	      }
	    /* If this is fully specified, and the best so far isn't,
	       then this is better.  */
	    else if (strchr (best_match, b'-' as c_int).is_null ()
		     && ! strchr (x_charset, b'-' as c_int).is_null ())
	    /* If this is an ISO codepage, and the best so far isn't,
	       then this is better, but only if it fully specifies the
	       encoding.  */
		|| (strnicmp (best_match, c"iso".as_ptr (), 3) != 0
		    && strnicmp (x_charset, c"iso".as_ptr (), 3) == 0
		    && ! strchr (x_charset, b'-' as c_int).is_null ())
		{ best_match = x_charset; }
            /* If both are ISO8859 codepages, choose the one with the
               lowest number in the encoding field.  */
            else if strnicmp (best_match, c"iso8859-".as_ptr (), 8) == 0	// C 2040
                     && strnicmp (x_charset, c"iso8859-".as_ptr (), 8) == 0
              {
                let best_enc: c_int = atoi (best_match.offset (8));	// C 2042
                let this_enc: c_int = atoi (x_charset.offset (8));
                if this_enc > 0 && this_enc < best_enc
                  { best_match = x_charset; }
              }
	  }

	rest = XCDR (rest);						// C 1985's `rest = XCDR (rest)`
      }

    /* If no match, encode the numeric value. */
    if best_match.is_null ()						// C 2050
      {
        sprintf (buf.as_mut_ptr (), c"*-#%d".as_ptr (), fncharset);	// C 2052
        return buf.as_mut_ptr ();
      }

    strncpy (buf.as_mut_ptr (), best_match, 31);			// C 2056
    /* If the charset is not fully specified, put -0 on the end.  */
    if strchr (best_match, b'-' as c_int).is_null ()			// C 2058
      {
	let mut pos: c_int = strlen (best_match) as c_int;		// C 2060
	/* Charset specifiers shouldn't be very long.  If it is a made
	   up one, truncating it should not do any harm since it isn't
	   recognized anyway.  */
	if pos > 29
	  { pos = 29; }
	strcpy (buf.as_mut_ptr ().offset (pos as isize), c"-0".as_ptr ());	// C 2066
      }
    buf[31] = 0;							// C 2068
    return buf.as_mut_ptr ();						// C 2069
  }
}

/* C 2080-2092: `w32_registry`.  */
/// C: static Lisp_Object w32_registry (LONG w32_charset, DWORD font_type)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_registry (w32_charset: LONG, font_type: DWORD) -> Lisp_Object
{
  let charset: *mut c_char;						// C 2082

  /* If charset is defaulted, charset is Unicode or unknown, depending on
     font type.  */
  if w32_charset == DEFAULT_CHARSET as LONG				// C 2086
    {
      return if font_type == TRUETYPE_FONTTYPE { Qiso10646_1 () } else { Qunknown () };
    }

  charset = w32_to_x_charset (w32_charset, std::ptr::null_mut ());	// C 2089
  return font_intern_prop (charset, strlen (charset) as ptrdiff_t, true);	// C 2090
}

/* C 2094-2107: `w32_decode_weight`.  */
/// C: static int w32_decode_weight (int fnweight)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_decode_weight (fnweight: c_int) -> c_int
{
  if fnweight >= FW_HEAVY      { return 210; }
  if fnweight >= FW_EXTRABOLD  { return 205; }
  if fnweight >= FW_BOLD       { return 200; }
  if fnweight >= FW_SEMIBOLD   { return 180; }
  if fnweight >= FW_MEDIUM     { return 100; }
  if fnweight >= FW_NORMAL     { return  80; }
  if fnweight >= FW_LIGHT      { return  50; }
  if fnweight >= FW_EXTRALIGHT { return  40; }
  if fnweight >= FW_THIN       { return  20; }
  return 0;
}

/* C 2109-2124: `w32_encode_weight`.  */
/// C: static int w32_encode_weight (int n)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_encode_weight (n: c_int) -> c_int
{
  if n >= 210 { return FW_HEAVY; }
  if n >= 205 { return FW_EXTRABOLD; }
  if n >= 200 { return FW_BOLD; }
  if n >= 180 { return FW_SEMIBOLD; }
  if n >= 100 { return FW_MEDIUM; }
  if n >=  80 { return FW_NORMAL; }
  if n >=  50 { return FW_LIGHT; }
  if n >=  40 { return FW_EXTRALIGHT; }
  if n >=  20 { return FW_THIN; }
  return 0;
}

/* C 2126-2139: `w32_to_fc_weight` - "Convert a Windows font weight into one of the weights
   supported by fontconfig (see font.c:font_parse_fcname)".  */
/// C: static Lisp_Object w32_to_fc_weight (int n)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_to_fc_weight (n: c_int) -> Lisp_Object
{
  if n >= FW_HEAVY      { return Qblack (); }
  if n >= FW_EXTRABOLD  { return Qextra_bold (); }
  if n >= FW_BOLD       { return Qbold (); }
  if n >= FW_SEMIBOLD   { return Qsemi_bold (); }
  if n >= FW_MEDIUM     { return Qmedium (); }
  if n >= FW_NORMAL     { return Qnormal (); }
  if n >= FW_LIGHT      { return Qlight (); }
  if n >= FW_EXTRALIGHT { return Qextra_light (); }
  return Qthin ();
}

/* C 2141-2287: `fill_in_logfont` - "Fill in all the available details of LOGFONT from
   FONT_SPEC".  */
/// C: static void fill_in_logfont (struct frame *f, LOGFONT *logfont, Lisp_Object font_spec)
#[cfg (feature = "have_ntgui")]
unsafe fn fill_in_logfont (f: *mut frame, logfont: *mut LOGFONT, font_spec: Lisp_Object)
{
  let mut tmp: Lisp_Object;						// C 2143
  let mut extra: Lisp_Object;
  let mut dpi: c_int = FRAME_RES (f) as c_int;				// C 2144

  tmp = AREF (font_spec, FONT_DPI_INDEX as isize);			// C 2146
  if FIXNUMP (tmp)
    { dpi = XFIXNUM (tmp) as c_int; }
  else if FLOATP (tmp)
    { dpi = (XFLOAT_DATA (tmp) + 0.5) as c_int; }			// C 2150

  /* Height  */
  tmp = AREF (font_spec, FONT_SIZE_INDEX as isize);			// C 2154
  if FIXNUMP (tmp)
    { (*logfont).lfHeight = -1 * XFIXNUM (tmp) as LONG; }		// C 2156
  else if FLOATP (tmp)
    { (*logfont).lfHeight = (-1.0 * dpi as f64 * XFLOAT_DATA (tmp) / 72.27 + 0.5) as LONG; }

  /* Escapement  */

  /* Orientation  */

  /* Weight  */
  tmp = AREF (font_spec, FONT_WEIGHT_INDEX as isize);			// C 2166
  if FIXNUMP (tmp)
    { (*logfont).lfWeight = w32_encode_weight (FONT_WEIGHT_NUMERIC! (font_spec) as c_int) as LONG; }

  /* Italic  */
  tmp = AREF (font_spec, FONT_SLANT_INDEX as isize);			// C 2171
  if FIXNUMP (tmp)
    {
      let slant: EMACS_INT = FONT_SLANT_NUMERIC! (font_spec);		// C 2173
      (*logfont).lfItalic = if slant > 150 { 1 } else { 0 };
    }

  /* Underline  */

  /* Strikeout  */

  /* Charset  */
  tmp = AREF (font_spec, FONT_REGISTRY_INDEX as isize);			// C 2183
  if ! NILP (tmp)
    { (*logfont).lfCharSet = registry_to_w32_charset (tmp) as BYTE; }	// C 2185
  else
    { (*logfont).lfCharSet = DEFAULT_CHARSET; }				// C 2187

  /* Out Precision  */

  /* Clip Precision  */

  /* Quality */
  (*logfont).lfQuality = DEFAULT_QUALITY;				// C 2195

  /* Generic Family and Face Name  */
  (*logfont).lfPitchAndFamily = FF_DONTCARE | DEFAULT_PITCH;		// C 2198

  tmp = AREF (font_spec, FONT_FAMILY_INDEX as isize);			// C 2200
  if ! NILP (tmp)
    {
      (*logfont).lfPitchAndFamily = w32_generic_family (tmp) | DEFAULT_PITCH;	// C 2202
      if ((*logfont).lfPitchAndFamily & 0xF0) != FF_DONTCARE		// C 2203
        { } /* Font name was generic, don't fill in font name.  */
        /* Font families are interned, but allow for strings also in case of
           user input.  */
      else if SYMBOLP (tmp)
	{
	  strncpy ((*logfont).lfFaceName.as_mut_ptr (),			// C 2208
		   SSDATA (ENCODE_SYSTEM (SYMBOL_NAME (tmp))),
		   LF_FACESIZE as usize);
	  (*logfont).lfFaceName[(LF_FACESIZE - 1) as usize] = 0;	// C 2210
	}
    }

  tmp = AREF (font_spec, FONT_ADSTYLE_INDEX as isize);			// C 2214
  if ! NILP (tmp)
    {
      /* Override generic family.  */
      let family: BYTE = w32_generic_family (tmp);			// C 2217
      if family != FF_DONTCARE
        { (*logfont).lfPitchAndFamily = family | DEFAULT_PITCH; }	// C 2219
    }

  /* Set pitch based on the spacing property.  */
  tmp = AREF (font_spec, FONT_SPACING_INDEX as isize);			// C 2223
  if FIXNUMP (tmp)
    {
      let spacing: c_int = XFIXNUM (tmp) as c_int;			// C 2225
      if spacing < FONT_SPACING_MONO					// C 2226
	{
	  (*logfont).lfPitchAndFamily
	    = ((*logfont).lfPitchAndFamily & 0xF0) | VARIABLE_PITCH;
	}
      else
	{
	  (*logfont).lfPitchAndFamily
	    = ((*logfont).lfPitchAndFamily & 0xF0) | FIXED_PITCH;	// C 2230
	}
    }

  /* Process EXTRA info.  */
  extra = AREF (font_spec, FONT_EXTRA_INDEX as isize);			// C 2235
  while CONSP (extra)
    {
      tmp = XCAR (extra);						// C 2237
      if CONSP (tmp)
        {
          let key: Lisp_Object;
          let val: Lisp_Object;
          key = XCAR (tmp);						// C 2241
	  val = XCDR (tmp);
          /* Only use QCscript if charset is not provided, or is Unicode
             and a single script is specified.  This is rather crude,
             and is only used to narrow down the fonts returned where
             there is a definite match.  Some scripts, such as latin, han,
             cjk-misc match multiple lfCharSet values, so we can't pre-filter
             them.  */
	  if EQ (key, QCscript ())					// C 2250
                   && (*logfont).lfCharSet == DEFAULT_CHARSET
                   && SYMBOLP (val)
            {
              if EQ (val, Qgreek ())					// C 2256
                { (*logfont).lfCharSet = GREEK_CHARSET; }
              else if EQ (val, Qhangul ())				// C 2258
                { (*logfont).lfCharSet = HANGUL_CHARSET; }
              else if EQ (val, Qkana ())				// C 2260
                { (*logfont).lfCharSet = SHIFTJIS_CHARSET; }
              else if EQ (val, Qbopomofo ())				// C 2262
                { (*logfont).lfCharSet = CHINESEBIG5_CHARSET; }
              /* GB 18030 supports tibetan, yi, mongolian,
                 fonts that support it should show up if we ask for
                 GB2312 fonts. */
              else if EQ (val, Qtibetan ())				// C 2267
		       /* Font supporting Yi in Windows 10/11 doesn't
			  get listed if we ask for GB2312 fonts.
			  FIXME: Is this true also for Windows 7 and
			  later?  */
		       || (EQ (val, Qyi ()) && w32_major_version < 10)
                       || EQ (val, Qmongolian ())
                { (*logfont).lfCharSet = GB2312_CHARSET; }
              else if EQ (val, Qhebrew ())				// C 2276
                { (*logfont).lfCharSet = HEBREW_CHARSET; }
              else if EQ (val, Qarabic ())				// C 2278
                { (*logfont).lfCharSet = ARABIC_CHARSET; }
              else if EQ (val, Qthai ())				// C 2280
                { (*logfont).lfCharSet = THAI_CHARSET; }
            }
          else if EQ (key, QCantialias ()) && SYMBOLP (val)		// C 2282
            {
              (*logfont).lfQuality = w32_antialias_type (val);		// C 2284
            }
        }
      extra = XCDR (extra);						// C 2235's `extra = XCDR (extra)`
    }
}

/* C 2289-2330: `list_all_matching_fonts`.  */
/// C: static void list_all_matching_fonts (struct font_callback_data *match_data)
#[cfg (feature = "have_ntgui")]
unsafe fn list_all_matching_fonts (match_data: *mut font_callback_data)
{
  let mut dc: HDC;							// C 2291
  let mut families: Lisp_Object = w32font_list_family (XFRAME ((*match_data).frame) as *mut c_void);
									// C 2292
  let f: *mut frame = XFRAME ((*match_data).frame);			// C 2293

  /* Prevent quitting while EnumFontFamiliesEx runs and conses the
     list it will return.  That's because get_frame_dc acquires the
     critical section, so we cannot quit before we release it in
     release_frame_dc.  */
  let mut prev_quit: Lisp_Object = Vinhibit_quit;			// C 2298
  Vinhibit_quit = Qt ();						// C 2299
  dc = get_frame_dc (f);						// C 2300

  while ! NILP (families)						// C 2302
    {
      /* Only fonts from the current locale are given localized names
	 on Windows, so we can keep backwards compatibility with
	 Windows 9x/ME by using non-Unicode font enumeration without
	 sacrificing internationalization here.  */
      let name: *mut c_char;						// C 2310
      let family: Lisp_Object = CAR (families);				// C 2311
      families = CDR (families);					// C 2312
      if NILP (family)
        { continue; }
      else if SYMBOLP (family)
	{ name = SSDATA (ENCODE_SYSTEM (SYMBOL_NAME (family))); }	// C 2317
      else
	{ continue; }

      strncpy ((*match_data).pattern.lfFaceName.as_mut_ptr (), name,	// C 2320
	       LF_FACESIZE as usize);
      (*match_data).pattern.lfFaceName[(LF_FACESIZE - 1) as usize] = 0;	// C 2321

      EnumFontFamiliesEx (dc, &mut (*match_data).pattern,			// C 2323
			  Some (add_font_entity_to_list),
			  match_data as LPARAM, 0);
    }

  release_frame_dc (f, dc);						// C 2328
  Vinhibit_quit = prev_quit;						// C 2329
}

/* C 2332-2357: `lispy_antialias_type`.  */
/// C: static Lisp_Object lispy_antialias_type (BYTE type)
#[cfg (feature = "have_ntgui")]
unsafe fn lispy_antialias_type (r#type: BYTE) -> Lisp_Object
{
  let mut lispy: Lisp_Object;						// C 2334

  match r#type								// C 2336
    {
      NONANTIALIASED_QUALITY => { lispy = Qnone (); }			// C 2338
      ANTIALIASED_QUALITY => { lispy = Qstandard (); }			// C 2341
      CLEARTYPE_QUALITY => { lispy = Qsubpixel (); }			// C 2344
      CLEARTYPE_NATURAL_QUALITY => { lispy = Qnatural (); }		// C 2347
      _ => { lispy = Qnil (); }						// C 2350
    }
  return lispy;								// C 2354
}

/* C 2359-2373: `w32_antialias_type` - "Convert antialiasing symbols to lfQuality".  */
/// C: static BYTE w32_antialias_type (Lisp_Object type)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_antialias_type (r#type: Lisp_Object) -> BYTE
{
  if EQ (r#type, Qnone ())						// C 2362
    { return NONANTIALIASED_QUALITY; }
  else if EQ (r#type, Qstandard ())
    { return ANTIALIASED_QUALITY; }
  else if EQ (r#type, Qsubpixel ())
    { return CLEARTYPE_QUALITY; }
  else if EQ (r#type, Qnatural ())
    { return CLEARTYPE_NATURAL_QUALITY; }
  else
    { return DEFAULT_QUALITY; }
}
/* C 2375-2599: `font_supported_scripts` - "Return a list of all the scripts that the font
   supports".  C's two `#define`s (C 2382-2388) are the port's two `macro_rules!`s with the
   enclosing locals passed in (`subranges`, `supported`); C's `#undef`s at C 2592-2593 are the
   macro definitions' own scope.  */
/// C: static Lisp_Object font_supported_scripts (FONTSIGNATURE * sig)
#[cfg (feature = "have_ntgui")]
unsafe fn font_supported_scripts (sig: *mut FONTSIGNATURE) -> Lisp_Object
{
  let subranges: *mut DWORD = (*sig).fsUsb.as_mut_ptr ();		// C 2378
  let mut supported: Lisp_Object = Qnil ();				// C 2379

  /* Match a single subrange. SYM is set if bit N is set in subranges.  */
  macro_rules! SUBRANGE {
    ($subranges:expr, $supported:expr, $n:expr, $sym:expr) => {
      if (*$subranges.offset ((($n) / 32) as isize) & (1u32 << (($n) % 32))) != 0
	{ $supported = Fcons ($sym, $supported); }
    };
  }

  /* Match multiple subranges. SYM is set if any MASK bit is set in
     subranges[0 - 3].  */
  macro_rules! MASK_ANY {
    ($subranges:expr, $supported:expr, $m0:expr, $m1:expr, $m2:expr, $m3:expr, $sym:expr) => {
      if ((*$subranges.offset (0) & ($m0)) != 0) || ((*$subranges.offset (1) & ($m1)) != 0)
	  || ((*$subranges.offset (2) & ($m2)) != 0) || ((*$subranges.offset (3) & ($m3)) != 0)
	{ $supported = Fcons ($sym, $supported); }
    };
  }

  /* 0: ASCII (a.k.a. "Basic Latin"),
     1: Latin-1 supplement, 2: Latin Extended A, 3: Latin Extended B,
     29: Latin Extended Additional.  */
  /* Most fonts that support Latin will have good coverage of the
     Extended blocks, so in practice marking them below is not really
     needed, or useful: if a font claims support for, say, Latin
     Extended-B, but does not contain glyphs for some of the
     characters in the range, the user will have to augment her
     fontset to display those few characters.  But we mark these
     subranges here anyway, for the marginal use cases where they
     might make a difference.  */
  MASK_ANY! (subranges, supported, 0x2000000F, 0, 0, 0, Qlatin ());
  SUBRANGE! (subranges, supported, 4, Qphonetic ());
  /* 5: Spacing and tone modifiers, 6: Combining Diacritical Marks.  */
  /* 7: Greek and Coptic, 30: Greek Extended.  */
  MASK_ANY! (subranges, supported, 0x40000080, 0, 0, 0, Qgreek ());
  SUBRANGE! (subranges, supported, 8, Qcoptic ());
  SUBRANGE! (subranges, supported, 9, Qcyrillic ());
  SUBRANGE! (subranges, supported, 10, Qarmenian ());
  SUBRANGE! (subranges, supported, 11, Qhebrew ());
  /* Bit 12 is rather useless if the user has Hebrew fonts installed,
     because apparently at some point in the past bit 12 was "Hebrew
     Extended", and many Hebrew fonts still have this bit set.  The
     only workaround is to customize fontsets to use fonts like Ebrima
     or Quivira.  */
  SUBRANGE! (subranges, supported, 12, Qvai ());
  SUBRANGE! (subranges, supported, 13, Qarabic ());
  SUBRANGE! (subranges, supported, 14, Qnko ());
  SUBRANGE! (subranges, supported, 15, Qdevanagari ());
  SUBRANGE! (subranges, supported, 16, Qbengali ());
  SUBRANGE! (subranges, supported, 17, Qgurmukhi ());
  SUBRANGE! (subranges, supported, 18, Qgujarati ());
  SUBRANGE! (subranges, supported, 19, Qoriya ());	/* a.k.a. "Odia" */
  SUBRANGE! (subranges, supported, 20, Qtamil ());
  SUBRANGE! (subranges, supported, 21, Qtelugu ());
  SUBRANGE! (subranges, supported, 22, Qkannada ());
  SUBRANGE! (subranges, supported, 23, Qmalayalam ());
  SUBRANGE! (subranges, supported, 24, Qthai ());
  SUBRANGE! (subranges, supported, 25, Qlao ());
  SUBRANGE! (subranges, supported, 26, Qgeorgian ());
  SUBRANGE! (subranges, supported, 27, Qbalinese ());
  /* 28: Hangul Jamo -- covered by the default fontset.  */
  /* 29: Latin Extended, 30: Greek Extended -- covered above.  */
  /* 31: Supplemental Punctuation -- most probably be masked by
     Courier New, so fontset customization is needed.  */
  /* 31-47: Symbols (defined below).  */
  SUBRANGE! (subranges, supported, 48, Qcjk_misc ());
  /* Match either 49: katakana or 50: hiragana for kana.  */
  MASK_ANY! (subranges, supported, 0, 0x00060000, 0, 0, Qkana ());
  SUBRANGE! (subranges, supported, 51, Qbopomofo ());
  /* 52: Compatibility Jamo */
  SUBRANGE! (subranges, supported, 53, Qphags_pa ());
  /* 54: Enclosed CJK letters and months, 55: CJK Compatibility.  */
  SUBRANGE! (subranges, supported, 56, Qhangul ());
  /* 57: Non-BMP.  Processed specially: Several fonts that support CJK
     Ideographs Extensions and other extensions, set just this bit and
     Latin, and nothing else.  */
  if (*subranges.offset (57 / 32) & (1u32 << (57 % 32))) != 0
    {
      if ((*sig).fsCsb[0] & CSB_CHINESE) != 0
	{ supported = Fcons (Qhan (), supported); }
      if ((*sig).fsCsb[0] & CSB_JAPANESE) != 0
	{ supported = Fcons (Qkana (), supported); }
      if ((*sig).fsCsb[0] & CSB_KOREAN) != 0
	{ supported = Fcons (Qhangul (), supported); }
    }
  SUBRANGE! (subranges, supported, 58, Qphoenician ());
  SUBRANGE! (subranges, supported, 59, Qhan ()); /* There are others, but this is the main one.  */
  SUBRANGE! (subranges, supported, 59, Qideographic_description ()); /* Windows lumps this in.  */
  /* These are covered well either by the default Courier New or by
     CJK fonts that are set up specially in the default fontset.  So
     marking them here wouldn't be useful.  */
  /* 60: Private use, 61: CJK strokes and compatibility.  */
  /* 62: Alphabetic Presentation, 63: Arabic Presentation A.  */
  /* 64: Combining half marks, 65: Vertical and CJK compatibility.  */
  /* 66: Small forms, 67: Arabic Presentation B, 68: Half and Full width.  */
  /* 69: Specials.  */
  SUBRANGE! (subranges, supported, 70, Qtibetan ());
  SUBRANGE! (subranges, supported, 71, Qsyriac ());
  SUBRANGE! (subranges, supported, 72, Qthaana ());
  SUBRANGE! (subranges, supported, 73, Qsinhala ());
  SUBRANGE! (subranges, supported, 74, Qburmese ());	/* a.k.a. "Myanmar" */
  SUBRANGE! (subranges, supported, 75, Qethiopic ());
  SUBRANGE! (subranges, supported, 76, Qcherokee ());
  SUBRANGE! (subranges, supported, 77, Qcanadian_aboriginal ());
  SUBRANGE! (subranges, supported, 78, Qogham ());
  SUBRANGE! (subranges, supported, 79, Qrunic ());
  SUBRANGE! (subranges, supported, 80, Qkhmer ());
  SUBRANGE! (subranges, supported, 81, Qmongolian ());
  SUBRANGE! (subranges, supported, 82, Qbraille ());
  SUBRANGE! (subranges, supported, 83, Qyi ());
  SUBRANGE! (subranges, supported, 84, Qbuhid ());
  SUBRANGE! (subranges, supported, 84, Qhanunoo ());
  SUBRANGE! (subranges, supported, 84, Qtagalog ());
  SUBRANGE! (subranges, supported, 84, Qtagbanwa ());
  SUBRANGE! (subranges, supported, 85, Qold_italic ());
  SUBRANGE! (subranges, supported, 86, Qgothic ());
  SUBRANGE! (subranges, supported, 87, Qdeseret ());
  SUBRANGE! (subranges, supported, 88, Qbyzantine_musical_symbol ());
  SUBRANGE! (subranges, supported, 88, Qmusical_symbol ()); /* Windows doesn't distinguish these.  */
  SUBRANGE! (subranges, supported, 89, Qmathematical_bold ()); /* See fontset.el:setup-default-fontset.  */
  SUBRANGE! (subranges, supported, 89, Qmathematical_italic ());
  SUBRANGE! (subranges, supported, 89, Qmathematical_bold_italic ());
  SUBRANGE! (subranges, supported, 89, Qmathematical_script ());
  SUBRANGE! (subranges, supported, 89, Qmathematical_bold_script ());
  SUBRANGE! (subranges, supported, 89, Qmathematical_fraktur ());
  SUBRANGE! (subranges, supported, 89, Qmathematical_double_struck ());
  SUBRANGE! (subranges, supported, 89, Qmathematical_bold_fraktur ());
  SUBRANGE! (subranges, supported, 89, Qmathematical_sans_serif ());
  SUBRANGE! (subranges, supported, 89, Qmathematical_sans_serif_bold ());
  SUBRANGE! (subranges, supported, 89, Qmathematical_sans_serif_italic ());
  SUBRANGE! (subranges, supported, 89, Qmathematical_sans_serif_bold_italic ());
  SUBRANGE! (subranges, supported, 89, Qmathematical_monospace ());
  /* 90: Private use, 91: Variation selectors, 92: Tags.  */
  SUBRANGE! (subranges, supported, 93, Qlimbu ());
  SUBRANGE! (subranges, supported, 94, Qtai_le ());
  SUBRANGE! (subranges, supported, 95, Qtai_lue ());
  SUBRANGE! (subranges, supported, 96, Qbuginese ());
  SUBRANGE! (subranges, supported, 97, Qglagolitic ());
  SUBRANGE! (subranges, supported, 98, Qtifinagh ());
  /* 99: Yijing Hexagrams.  */
  SUBRANGE! (subranges, supported, 99, Qcjk_misc ());
  SUBRANGE! (subranges, supported, 100, Qsyloti_nagri ());
  SUBRANGE! (subranges, supported, 101, Qlinear_b ());
  SUBRANGE! (subranges, supported, 101, Qaegean_number ());
  SUBRANGE! (subranges, supported, 102, Qancient_greek_number ());
  SUBRANGE! (subranges, supported, 103, Qugaritic ());
  SUBRANGE! (subranges, supported, 104, Qold_persian ());
  SUBRANGE! (subranges, supported, 105, Qshavian ());
  SUBRANGE! (subranges, supported, 106, Qosmanya ());
  SUBRANGE! (subranges, supported, 107, Qcypriot_syllabary ());
  SUBRANGE! (subranges, supported, 108, Qkharoshthi ());
  SUBRANGE! (subranges, supported, 109, Qtai_xuan_jing_symbol ());
  SUBRANGE! (subranges, supported, 110, Qcuneiform ());
  SUBRANGE! (subranges, supported, 111, Qcounting_rod_numeral ());
  SUBRANGE! (subranges, supported, 112, Qsundanese ());
  SUBRANGE! (subranges, supported, 113, Qlepcha ());
  SUBRANGE! (subranges, supported, 114, Qol_chiki ());
  SUBRANGE! (subranges, supported, 115, Qsaurashtra ());
  SUBRANGE! (subranges, supported, 116, Qkayah_li ());
  SUBRANGE! (subranges, supported, 117, Qrejang ());
  SUBRANGE! (subranges, supported, 118, Qcham ());
  SUBRANGE! (subranges, supported, 119, Qancient_symbol ());
  SUBRANGE! (subranges, supported, 120, Qphaistos_disc ());
  SUBRANGE! (subranges, supported, 121, Qlycian ());
  SUBRANGE! (subranges, supported, 121, Qcarian ());
  SUBRANGE! (subranges, supported, 121, Qlydian ());
  SUBRANGE! (subranges, supported, 122, Qdomino_tile ());
  SUBRANGE! (subranges, supported, 122, Qmahjong_tile ());
  /* 123-127: Reserved.  */

  /* There isn't really a main symbol range, so include symbol if any
     relevant range is set.  */
  MASK_ANY! (subranges, supported, 0x80000000, 0x0000FFFF, 0x00000004, 0, Qsymbol ());

  /* Missing:
       Tai Viet
       Old Permic
       Palmyrene
       Nabatean
       Manichean
       Hanifi Rohingya
       Sogdian
       Elymaic
       Mahajani
       Khojki
       Khudawadi
       Grantha
       Newa
       Tirhuta
       Siddham
       Modi
       Takri
       Dogra
       Warang Citi
       Nandinagari
       Zanabazar Square
       Soyombo
       Pau Cin Hau
       Bhaiksuki
       Marchen
       Masaram Gondi
       Makasar
       Egyptian
       Mro
       Bassa-Vah
       Pahawh Hmong
       Medefaidrin
       Tangut
       Tangut Components
       Nushu
       Duployan Shorthand
       Ancient Greek Musical Notation
       Nyiakeng Puachue Hmong
       Wancho
       Mende Kikakui
       Adlam
       Indic Siyaq Number
       Ottoman Siyaq Number.  */
/* C 2592-2593: `#undef SUBRANGE` / `#undef MASK_ANY` - the two `macro_rules!`s above are scoped
   to this function body, which is what C's `#undef`s say.  */
  return supported;							// C 2598
}

/* C 2601-2672: `w32font_full_name` - "Generate a full name for a Windows font.  The full name is
   in fcname format, with weight, slant and antialiasing specified if they are not
   `normal`".  */
/// C: static int w32font_full_name (LOGFONT * font, Lisp_Object font_obj, int pixel_size,
/// C:                               char *name, int nbytes)
#[cfg (feature = "have_ntgui")]
unsafe fn w32font_full_name (font: *mut LOGFONT, font_obj: Lisp_Object, pixel_size: c_int,
			     name: *mut c_char, nbytes: c_int) -> c_int
{
  let mut len: c_int;							// C 2604
  let mut height: c_int;
  let outline: c_int;
  let mut p: *mut c_char;
  let antialiasing: Lisp_Object;					// C 2606
  let mut weight: Lisp_Object = Qnil ();

  len = strlen ((*font).lfFaceName.as_ptr ()) as c_int;			// C 2608

  outline = (EQ (AREF (font_obj, FONT_FOUNDRY_INDEX as isize),
		 Qoutline ())) as c_int;				// C 2610

  /* Represent size of scalable fonts by point size. But use pixelsize for
     raster fonts to indicate that they are exactly that size.  */
  if outline != 0							// C 2614
    { len += 11; } /* -SIZE */
  else
    { len += 21; }

  if (*font).lfItalic != 0						// C 2619
    { len += 7; } /* :italic */

  if (*font).lfWeight != 0 && (*font).lfWeight != FW_NORMAL		// C 2621
    {
      weight = w32_to_fc_weight ((*font).lfWeight as c_int);		// C 2623
      len += 1 + SBYTES (SYMBOL_NAME (weight)) as c_int; /* :WEIGHT */
    }

  antialiasing = lispy_antialias_type ((*font).lfQuality);		// C 2626
  if ! NILP (antialiasing)
    { len += 11 + SBYTES (SYMBOL_NAME (antialiasing)) as c_int; } /* :antialias=NAME */

  /* Check that the buffer is big enough  */
  if len > nbytes							// C 2631
    { return -1; }

  p = name;								// C 2634
  p = p.offset (sprintf (p, c"%s".as_ptr (), (*font).lfFaceName.as_ptr ()) as isize);	// C 2635

  height = if (*font).lfHeight != 0					// C 2637
	     { eabs! ((*font).lfHeight) as c_int } else { pixel_size };

  if height > 0								// C 2639
    {
      if outline != 0							// C 2641
        {
          let mut pointsize: c_double
	    = height as c_double * 72.0 / one_w32_display_info.resy;	// C 2643
          /* Round to nearest half point.  floor is used, since round is not
	     supported in MS library.  */
          pointsize = floor (pointsize * 2.0 + 0.5) / 2.0;		// C 2647
          p = p.offset (sprintf (p, c"-%1.1f".as_ptr (), pointsize) as isize);	// C 2648
        }
      else
	{
	  p = p.offset (sprintf (p, c":pixelsize=%d".as_ptr (), height) as isize);	// C 2651
	}
    }

  if SYMBOLP (weight) && ! NILP (weight)				// C 2655
    {
      p = p.offset (sprintf (p, c":%s".as_ptr (),
			     SDATA (SYMBOL_NAME (weight)) as *mut c_char) as isize);
    }

  if (*font).lfItalic != 0						// C 2658
    { p = p.offset (sprintf (p, c":italic".as_ptr ()) as isize); }

  if SYMBOLP (antialiasing) && ! NILP (antialiasing)			// C 2661
    {
      p = p.offset (sprintf (p, c":antialias=%s".as_ptr (),
			     SDATA (SYMBOL_NAME (antialiasing)) as *mut c_char) as isize);
    }

  return p.offset_from (name) as c_int;					// C 2664
}

/* C 2674-2711: `logfont_to_fcname` - "Convert a logfont and point size into a fontconfig style
   font name.  POINTSIZE is in tenths of points".  */
/// C: static int logfont_to_fcname (LOGFONT* font, int pointsize, char *fcname, int size)
#[cfg (feature = "have_ntgui")]
unsafe fn logfont_to_fcname (font: *mut LOGFONT, pointsize: c_int, fcname: *mut c_char,
			     size: c_int) -> c_int
{
  let mut len: c_int;							// C 2677
  let mut height: c_int;
  let mut p: *mut c_char = fcname;					// C 2679
  let mut weight: Lisp_Object = Qnil ();

  len = (strlen ((*font).lfFaceName.as_ptr ()) + 2) as c_int;		// C 2682
  height = pointsize / 10;						// C 2683
  height /= 10;								// C 2684
  while height != 0
    {
      len += 1;
      height /= 10;
    }

  if pointsize % 10 != 0						// C 2687
    { len += 2; }

  if (*font).lfItalic != 0						// C 2690
    { len += 7; } /* :italic */
  if (*font).lfWeight != 0 && (*font).lfWeight != FW_NORMAL		// C 2691
    {
      weight = w32_to_fc_weight ((*font).lfWeight as c_int);		// C 2693
      len += SBYTES (SYMBOL_NAME (weight)) as c_int + 1;
    }

  if len > size								// C 2696
    { return -1; }

  p = p.offset (sprintf (p, c"%s-%d".as_ptr (), (*font).lfFaceName.as_ptr (),
			 pointsize / 10) as isize);			// C 2698
  if pointsize % 10 != 0						// C 2699
    { p = p.offset (sprintf (p, c".%d".as_ptr (), pointsize % 10) as isize); }

  if SYMBOLP (weight) && ! NILP (weight)				// C 2702
    {
      p = p.offset (sprintf (p, c":%s".as_ptr (),
			     SDATA (SYMBOL_NAME (weight)) as *mut c_char) as isize);
    }

  if (*font).lfItalic != 0						// C 2705
    { p = p.offset (sprintf (p, c":italic".as_ptr ()) as isize); }

  return p.offset_from (fcname) as c_int;				// C 2707
}

/* C 2713-2750: `compute_metrics`.  C's `metrics->rbearing = gm.gmptGlyphOrigin.x +
   gm.gmBlackBoxX;` is a `LONG`+`UINT` mix assigned to a `short`: the port keeps C's
   value-preserving `wrapping_add`/`wrapping_sub` and the truncating cast (PORTING.md 2.2).  */
/// C: static void compute_metrics (HDC dc, struct w32font_info *w32_font, unsigned int code,
/// C:                               struct w32_metric_cache *metrics)
#[cfg (feature = "have_ntgui")]
unsafe fn compute_metrics (dc: HDC, w32_font: *mut w32font_info, code: c_uint,
			   metrics: *mut w32_metric_cache)
{
  let mut gm: GLYPHMETRICS = core::mem::zeroed ();			// C 2716
  let mut transform: MAT2 = core::mem::zeroed ();			// C 2717
  let mut options: c_uint = GGO_METRICS;				// C 2718
  let mut width: c_int = 0;	/* zero-initialised, PORTING.md 1.5 */	// C 2719

  if (*w32_font).glyph_idx != 0						// C 2721
    { options |= GGO_GLYPH_INDEX; }

  memset (&raw mut transform as *mut c_void, 0, core::mem::size_of::<MAT2> ());	// C 2723
  transform.eM11.value = 1;						// C 2724
  transform.eM22.value = 1;						// C 2725

  if get_glyph_outline_w (dc, code, options, &mut gm, 0, std::ptr::null_mut (),
			  &transform) != GDI_ERROR			// C 2727
    {
      (*metrics).lbearing = gm.gmptGlyphOrigin.x as c_short;		// C 2729
      (*metrics).rbearing
	= gm.gmptGlyphOrigin.x.wrapping_add (gm.gmBlackBoxX as LONG) as c_short;	// C 2730
      (*metrics).width = gm.gmCellIncX;					// C 2731
      (*metrics).ascent = gm.gmptGlyphOrigin.y as c_short;		// C 2732
      (*metrics).descent
	= gm.gmBlackBoxY.wrapping_sub (gm.gmptGlyphOrigin.y as UINT) as c_short;	// C 2733
      (*metrics).status = W32METRIC_SUCCESS;				// C 2734
    }
  else if get_char_width_32_w (dc, code, code, &mut width) != 0		// C 2736
    {
      (*metrics).lbearing = 0;						// C 2738
      (*metrics).rbearing = width as c_short;				// C 2739
      (*metrics).width = width as c_short;				// C 2740
      (*metrics).ascent = (*w32_font).font.ascent as c_short;		// C 2741
      (*metrics).descent = (*w32_font).font.descent as c_short;		// C 2742
      (*metrics).status = W32METRIC_SUCCESS;				// C 2743
    }
  else
    { (*metrics).status = W32METRIC_FAIL; }				// C 2746
}

/* C 2752-2817: the `x-select-font` subr.  C's docstring is the `doc:` literal.  */
/// C: DEFUN ("x-select-font", Fx_select_font, Sx_select_font, 0, 2, 0,
/// C:        doc: /* Read a font name using a W32 font selection dialog. ... */)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-select-font", Fx_select_font,
       Sx_select_font, 0, 2, 0,
       doc: "Read a font name using a W32 font selection dialog.\nReturn fontconfig style font string corresponding to the selection.\n\nIf FRAME is omitted or nil, it defaults to the selected frame.\nIf EXCLUDE-PROPORTIONAL is non-nil, exclude proportional fonts\nin the font selection dialog.",
       (frame: Lisp_Object, exclude_proportional: Lisp_Object),
       {
  let f: *mut frame = decode_window_system_frame (frame);		// C 2759
  let mut cf: CHOOSEFONT = core::mem::zeroed ();			// C 2760
  let mut lf: LOGFONT = core::mem::zeroed ();				// C 2761
  let mut tm: TEXTMETRIC = core::mem::zeroed ();			// C 2762
  let mut hdc: HDC;							// C 2763
  let mut oldobj: HANDLE;						// C 2764
  let mut buf: [c_char; 100] = [0; 100];				// C 2765

  memset (&raw mut cf as *mut c_void, 0, core::mem::size_of::<CHOOSEFONT> ());	// C 2767
  memset (&raw mut lf as *mut c_void, 0, core::mem::size_of::<LOGFONT> ());	// C 2768

  cf.lStructSize = core::mem::size_of::<CHOOSEFONT> () as DWORD;	// C 2770
  cf.hwndOwner = FRAME_W32_WINDOW! (f);					// C 2771
  cf.Flags = CF_FORCEFONTEXIST | CF_SCREENFONTS | CF_NOVERTFONTS;	// C 2772

  /* If exclude_proportional is non-nil, limit the selection to
     monospaced fonts.  */
  if ! NILP (exclude_proportional)					// C 2776
    { cf.Flags |= CF_FIXEDPITCHONLY; }

  cf.lpLogFont = &mut lf;						// C 2778

  /* Initialize as much of the font details as we can from the current
     default font.  */
  hdc = GetDC (FRAME_W32_WINDOW! (f));					// C 2782
  oldobj = SelectObject (hdc, FONT_HANDLE! (FRAME_FONT! (f)) as HGDIOBJ);	// C 2783
  GetTextFace (hdc, LF_FACESIZE, lf.lfFaceName.as_mut_ptr ());		// C 2784
  if GetTextMetrics (hdc, &mut tm) != 0					// C 2785
    {
      lf.lfHeight = tm.tmInternalLeading - tm.tmHeight;			// C 2787
      lf.lfWeight = tm.tmWeight;					// C 2788
      lf.lfItalic = tm.tmItalic;					// C 2789
      lf.lfUnderline = tm.tmUnderlined;					// C 2790
      lf.lfStrikeOut = tm.tmStruckOut;					// C 2791
      lf.lfCharSet = tm.tmCharSet;					// C 2792
      cf.Flags |= CF_INITTOLOGFONTSTRUCT;				// C 2793
    }
  SelectObject (hdc, oldobj as HGDIOBJ);				// C 2795
  ReleaseDC (FRAME_W32_WINDOW! (f), hdc);				// C 2796

  {
    let count: specpdl_ref = SPECPDL_INDEX ();				// C 2799
    let mut value: Lisp_Object = Qnil ();				// C 2800

    w32_dialog_in_progress (Qt ());					// C 2802
    specbind (Qinhibit_redisplay (), Qt ());				// C 2803
    record_unwind_protect (Some (w32_dialog_in_progress), Qnil ());	// C 2804

    if ChooseFont (&mut cf) != 0					// C 2806
	&& logfont_to_fcname (&mut lf, cf.iPointSize, buf.as_mut_ptr (), 100) >= 0
      { value = DECODE_SYSTEM (build_string (buf.as_mut_ptr ())); }	// C 2808

    unbind_to (count, Qnil ());						// C 2810

    return value;							// C 2812
  }
});

/* C 2819-2828: the two property-name arrays `font_filter_properties` reads.  */
/// C: static const char *const w32font_booleans [] = { NULL, };
/// C: static const char *const w32font_non_booleans [] = { ":script", ":antialias", ":style",
/// C:                                                    NULL, };
#[cfg (feature = "have_ntgui")]
static mut w32font_booleans: [*const c_char; 1] = [std::ptr::null ()];
#[cfg (feature = "have_ntgui")]
static mut w32font_non_booleans: [*const c_char; 4]
  = [c":script".as_ptr (), c":antialias".as_ptr (), c":style".as_ptr (), std::ptr::null ()];

/* C 2830-2834: `w32font_filter_properties`.  */
/// C: static void w32font_filter_properties (Lisp_Object font, Lisp_Object alist)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn w32font_filter_properties (font: Lisp_Object, alist: Lisp_Object)
{
  font_filter_properties (font, alist, w32font_booleans.as_ptr (),	// C 2833
			  w32font_non_booleans.as_ptr ());
}

/* C 2836-2869: the driver record.  `LISPSYM_INITIALLY` is not a `const fn` in the mirror, so
   the port writes the same constant expression over the generated `iQgdi` index (the
   `src/androidfont.rs` 1291 precedent); the members C's positional initialiser leaves implicit
   are written out as `None` because Rust has no designated initialisers, and `draw` plus the two
   bitmap members carry the mirror's `have_window_system` condition (font.h 682-698).  */
/// C: struct font_driver w32font_driver = { LISPSYM_INITIALLY (Qgdi), false, w32font_get_cache,
/// C:   w32font_list, w32font_match, w32font_list_family, NULL /* free_entity */, w32font_open,
/// C:   w32font_close, NULL /* prepare_face */, NULL /* done_face */, w32font_has_char,
/// C:   w32font_encode_char, w32font_text_extents, w32font_draw, NULL /* get_bitmap */,
/// C:   NULL /* free_bitmap */, NULL /* anchor_point */, NULL /* otf_capability */,
/// C:   NULL /* otf_drive */, NULL /* start_for_frame */, NULL /* end_for_frame */,
/// C:   NULL /* shape */, NULL /* check */, NULL /* get_variation_glyphs */,
/// C:   w32font_filter_properties, NULL /* cached_font_ok */ };
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32font_driver: font_driver = font_driver {
  r#type: (((iQgdi * core::mem::size_of::<Lisp_Symbol> ()) as EMACS_UINT
	    + Lisp_Type::Lisp_Symbol as EMACS_UINT) as Lisp_Object),	// C 2839
  case_sensitive: false,	/* case insensitive */			// C 2840
  get_cache: Some (w32font_get_cache),					// C 2841
  list: Some (w32font_list),						// C 2842
  r#match: Some (w32font_match),					// C 2843
  list_family: Some (w32font_list_family),				// C 2844
  free_entity: None,		/* NULL */				// C 2845
  open_font: Some (w32font_open),					// C 2846
  close_font: Some (w32font_close),					// C 2847
  prepare_face: None,		/* NULL */				// C 2848
  done_face: None,		/* NULL */				// C 2849
  has_char: Some (w32font_has_char),					// C 2850
  encode_char: Some (w32font_encode_char),				// C 2851
  text_extents: Some (w32font_text_extents),				// C 2852
  #[cfg (feature = "have_window_system")]
  draw: Some (w32font_draw),						// C 2853
  #[cfg (feature = "have_window_system")]
  get_bitmap: None,		/* NULL */				// C 2854
  #[cfg (feature = "have_window_system")]
  free_bitmap: None,		/* NULL */				// C 2855
  anchor_point: None,		/* NULL */				// C 2856
  otf_capability: None,		/* NULL */				// C 2857
  otf_drive: None,		/* NULL */				// C 2858
  start_for_frame: None,	/* NULL */				// C 2859
  end_for_frame: None,		/* NULL */				// C 2860
  shape: None,			/* NULL */				// C 2861
  check: None,			/* NULL */				// C 2862
  get_variation_glyphs: None,	/* NULL */				// C 2863
  filter_properties: Some (w32font_filter_properties),			// C 2864
  cached_font_ok: None,		/* NULL */				// C 2865
  combining_capability: None,	/* C's initialiser stops here */
  drop_xrender_surfaces: None,
};

/* C 2871: `static void syms_of_w32font_for_pdumper (void);` - dropped (PORTING.md 0.1-12); the
   definition is C 3064-3068.  */

/* C's one `globals.f_Vw32_charset_info_alist` field this unit's `syms_of_w32font` registers
   (its `DEFVAR_LISP` call at C 3014), defined here as the port's storage - C's
   `struct emacs_globals` is zero-initialised and the DEFVAR macros only take each field's
   address, so the initialiser is `lisp_h_Qnil ()` (docs/status.md 3.29, the `src/cmds.rs`
   row and the `src/alloc.rs` 8019-8025 note).  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut Vw32_charset_info_alist: Lisp_Object = lisp_h_Qnil (); // C 3014

/* C 2873-3015: `syms_of_w32font`.  */
/// C: void syms_of_w32font (void)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_w32font ()
{
  DEFSYM! (Qgdi, "gdi");
  DEFSYM! (Quniscribe, "uniscribe");
  DEFSYM! (Qharfbuzz, "harfbuzz");
  DEFSYM! (QCformat, ":format");

  /* Generic font families.  */
  DEFSYM! (Qmonospace, "monospace");
  DEFSYM! (Qserif, "serif");
  DEFSYM! (Qsansserif, "sansserif");
  DEFSYM! (Qscript, "script");
  DEFSYM! (Qdecorative, "decorative");
  /* Aliases.  */
  DEFSYM! (Qsans_serif, "sans_serif");
  DEFSYM! (Qsans, "sans");
  DEFSYM! (Qmono, "mono");

  /* Fake foundries.  */
  DEFSYM! (Qraster, "raster");
  DEFSYM! (Qoutline, "outline");
  DEFSYM! (Qunknown, "unknown");

  /* Antialiasing.  */
  DEFSYM! (Qstandard, "standard");
  DEFSYM! (Qsubpixel, "subpixel");
  DEFSYM! (Qnatural, "natural");

  /* Font formats.  */
  DEFSYM! (Qtruetype, "truetype");
  DEFSYM! (Qtype1, "type1");
  DEFSYM! (Qw32bitmap, "w32bitmap");
  DEFSYM! (Qw32vector, "w32vector");

  /* Languages  */
  DEFSYM! (Qzh, "zh");

  /* Scripts  */
  DEFSYM! (Qlatin, "latin");
  DEFSYM! (Qgreek, "greek");
  DEFSYM! (Qcoptic, "coptic");
  DEFSYM! (Qcyrillic, "cyrillic");
  DEFSYM! (Qarmenian, "armenian");
  DEFSYM! (Qhebrew, "hebrew");
  DEFSYM! (Qvai, "vai");
  DEFSYM! (Qarabic, "arabic");
  DEFSYM! (Qsyriac, "syriac");
  DEFSYM! (Qnko, "nko");
  DEFSYM! (Qthaana, "thaana");
  DEFSYM! (Qdevanagari, "devanagari");
  DEFSYM! (Qbengali, "bengali");
  DEFSYM! (Qgurmukhi, "gurmukhi");
  DEFSYM! (Qgujarati, "gujarati");
  DEFSYM! (Qoriya, "oriya");
  DEFSYM! (Qtamil, "tamil");
  DEFSYM! (Qtelugu, "telugu");
  DEFSYM! (Qkannada, "kannada");
  DEFSYM! (Qmalayalam, "malayalam");
  DEFSYM! (Qsinhala, "sinhala");
  DEFSYM! (Qthai, "thai");
  DEFSYM! (Qlao, "lao");
  DEFSYM! (Qtibetan, "tibetan");
  DEFSYM! (Qburmese, "burmese");
  DEFSYM! (Qgeorgian, "georgian");
  DEFSYM! (Qhangul, "hangul");
  DEFSYM! (Qethiopic, "ethiopic");
  DEFSYM! (Qcherokee, "cherokee");
  DEFSYM! (Qcanadian_aboriginal, "canadian-aboriginal");
  DEFSYM! (Qogham, "ogham");
  DEFSYM! (Qrunic, "runic");
  DEFSYM! (Qkhmer, "khmer");
  DEFSYM! (Qmongolian, "mongolian");
  DEFSYM! (Qbraille, "braille");
  DEFSYM! (Qhan, "han");
  DEFSYM! (Qideographic_description, "ideographic-description");
  DEFSYM! (Qcjk_misc, "cjk-misc");
  DEFSYM! (Qkana, "kana");
  DEFSYM! (Qbopomofo, "bopomofo");
  DEFSYM! (Qyi, "yi");
  DEFSYM! (Qbyzantine_musical_symbol, "byzantine-musical-symbol");
  DEFSYM! (Qmusical_symbol, "musical-symbol");
  DEFSYM! (Qmathematical_bold, "mathematical-bold");
  DEFSYM! (Qmathematical_italic, "mathematical-italic");
  DEFSYM! (Qmathematical_bold_italic, "mathematical-bold-italic");
  DEFSYM! (Qmathematical_script, "mathematical-script");
  DEFSYM! (Qmathematical_bold_script, "mathematical-bold-script");
  DEFSYM! (Qmathematical_fraktur, "mathematical-fraktur");
  DEFSYM! (Qmathematical_double_struck, "mathematical-double-struck");
  DEFSYM! (Qmathematical_bold_fraktur, "mathematical-bold-fraktur");
  DEFSYM! (Qmathematical_sans_serif, "mathematical-sans-serif");
  DEFSYM! (Qmathematical_sans_serif_bold, "mathematical-sans-serif-bold");
  DEFSYM! (Qmathematical_sans_serif_italic, "mathematical-sans-serif-italic");
  DEFSYM! (Qmathematical_sans_serif_bold_italic, "mathematical-sans-serif-bold-italic");
  DEFSYM! (Qmathematical_monospace, "mathematical-monospace");
  DEFSYM! (Qcham, "cham");
  DEFSYM! (Qphonetic, "phonetic");
  DEFSYM! (Qbalinese, "balinese");
  DEFSYM! (Qbuginese, "buginese");
  DEFSYM! (Qbuhid, "buhid");
  DEFSYM! (Qcuneiform, "cuneiform");
  DEFSYM! (Qcypriot_syllabary, "cypriot-syllabary");
  DEFSYM! (Qdeseret, "deseret");
  DEFSYM! (Qglagolitic, "glagolitic");
  DEFSYM! (Qgothic, "gothic");
  DEFSYM! (Qhanunoo, "hanunoo");
  DEFSYM! (Qkharoshthi, "kharoshthi");
  DEFSYM! (Qlimbu, "limbu");
  DEFSYM! (Qlinear_b, "linear-b");
  DEFSYM! (Qaegean_number, "aegean-number");
  DEFSYM! (Qold_italic, "old-italic");
  DEFSYM! (Qold_persian, "old-persian");
  DEFSYM! (Qosmanya, "osmanya");
  DEFSYM! (Qphags_pa, "phags-pa");
  DEFSYM! (Qphoenician, "phoenician");
  DEFSYM! (Qshavian, "shavian");
  DEFSYM! (Qsyloti_nagri, "syloti-nagri");
  DEFSYM! (Qtagalog, "tagalog");
  DEFSYM! (Qtagbanwa, "tagbanwa");
  DEFSYM! (Qtai_le, "tai-le");
  DEFSYM! (Qtai_lue, "tai-lue");
  DEFSYM! (Qtifinagh, "tifinagh");
  DEFSYM! (Qugaritic, "ugaritic");
  DEFSYM! (Qlycian, "lycian");
  DEFSYM! (Qcarian, "carian");
  DEFSYM! (Qlydian, "lydian");
  DEFSYM! (Qdomino_tile, "domino-tile");
  DEFSYM! (Qmahjong_tile, "mahjong-tile");
  DEFSYM! (Qtai_xuan_jing_symbol, "tai-xuan-jing-symbol");
  DEFSYM! (Qcounting_rod_numeral, "counting-rod-numeral");
  DEFSYM! (Qancient_symbol, "ancient-symbol");
  DEFSYM! (Qphaistos_disc, "phaistos-disc");
  DEFSYM! (Qancient_greek_number, "ancient-greek-number");
  DEFSYM! (Qsundanese, "sundanese");
  DEFSYM! (Qlepcha, "lepcha");
  DEFSYM! (Qol_chiki, "ol-chiki");
  DEFSYM! (Qsaurashtra, "saurashtra");
  DEFSYM! (Qkayah_li, "kayah-li");
  DEFSYM! (Qrejang, "rejang");
  /* W32 font encodings.  */
  DEFVAR_LISP! (c"w32-charset-info-alist",
               Vw32_charset_info_alist,
               "Alist linking Emacs character sets to Windows fonts and codepages.\nEach entry should be of the form:\n\n   (CHARSET_NAME . (WINDOWS_CHARSET . CODEPAGE))\n\nwhere CHARSET_NAME is a string used in font names to identify the charset,\nWINDOWS_CHARSET is a symbol that can be one of:\n\n  w32-charset-ansi, w32-charset-default, w32-charset-symbol,\n  w32-charset-shiftjis, w32-charset-hangeul, w32-charset-gb2312,\n  w32-charset-chinesebig5, w32-charset-johab, w32-charset-hebrew,\n  w32-charset-arabic, w32-charset-greek, w32-charset-turkish,\n  w32-charset-vietnamese, w32-charset-thai, w32-charset-easteurope,\n  w32-charset-russian, w32-charset-mac, w32-charset-baltic,\n  or w32-charset-oem.\n\nCODEPAGE should be an integer specifying the codepage that should be used\nto display the character set, t to do no translation and output as Unicode,\nor nil to do no translation and output as 8 bit (or multibyte on far-east\nversions of Windows) characters.");                                   // C 3014-3035
  Vw32_charset_info_alist = Qnil ();                            // C 3036

  DEFSYM! (Qw32_charset_ansi, "w32-charset-ansi");
  DEFSYM! (Qw32_charset_symbol, "w32-charset-symbol");
  DEFSYM! (Qw32_charset_default, "w32-charset-default");
  DEFSYM! (Qw32_charset_shiftjis, "w32-charset-shiftjis");
  DEFSYM! (Qw32_charset_hangeul, "w32-charset-hangeul");
  DEFSYM! (Qw32_charset_chinesebig5, "w32-charset-chinesebig5");
  DEFSYM! (Qw32_charset_gb2312, "w32-charset-gb2312");
  DEFSYM! (Qw32_charset_oem, "w32-charset-oem");
  DEFSYM! (Qw32_charset_johab, "w32-charset-johab");
  DEFSYM! (Qw32_charset_easteurope, "w32-charset-easteurope");
  DEFSYM! (Qw32_charset_turkish, "w32-charset-turkish");
  DEFSYM! (Qw32_charset_baltic, "w32-charset-baltic");
  DEFSYM! (Qw32_charset_russian, "w32-charset-russian");
  DEFSYM! (Qw32_charset_arabic, "w32-charset-arabic");
  DEFSYM! (Qw32_charset_greek, "w32-charset-greek");
  DEFSYM! (Qw32_charset_hebrew, "w32-charset-hebrew");
  DEFSYM! (Qw32_charset_vietnamese, "w32-charset-vietnamese");
  DEFSYM! (Qw32_charset_thai, "w32-charset-thai");
  DEFSYM! (Qw32_charset_mac, "w32-charset-mac");
  DEFSYM! (Qw32_non_USB_fonts, "w32-non-USB-fonts");

  defsubr (&raw mut Sx_select_font);                            // C 3059

  pdumper_do_now_and_after_load (Some (syms_of_w32font_for_pdumper));   // C 3061
}

/* C 3064-3068: "Initialize state that does not change between invocations. This is only
   called when Emacs is dumped."  */
/// C: static void syms_of_w32font_for_pdumper (void)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn syms_of_w32font_for_pdumper ()
{
  register_font_driver (&raw const w32font_driver, std::ptr::null_mut ());	// C 3067
}

/* C 3070-3079: `globals_of_w32font` - the four `g_b_init_*` flags, zeroed only on the
   `WINDOWSNT` arm (the Cygwin arm's text is the empty `#else`).  */
/// C: void globals_of_w32font (void)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn globals_of_w32font ()
{
  #[cfg (feature = "windowsnt")]
  {
    g_b_init_get_outline_metrics_w = 0;					// C 3074
    g_b_init_get_text_metrics_w = 0;					// C 3075
    g_b_init_get_glyph_outline_w = 0;					// C 3076
    g_b_init_get_char_width_32_w = 0;					// C 3077
  }
}
