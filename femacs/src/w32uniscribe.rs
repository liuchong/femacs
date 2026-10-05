// femacs: port of src/w32uniscribe.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `w32uniscribe.o`
// sits in `W32_OBJ` (`emacs/src/Makefile.in` 317-319, `W32_OBJ=@W32_OBJ@`, filled by
// `configure.ac` 3031 under the `HAVE_W32` test whose 3024 `AC_DEFINE`s `HAVE_NTGUI`), and
// `docs/baseline/frozen-build-sources.txt` lists no `w32uniscribe.o` for this target - so
// `extract-abi.py`'s checklist for the file is empty (P-86 (2)) and the ledger
// `docs/ports/w32uniscribe.md` carries that clause's substitute: the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "have_ntgui"`, on every item *and on the include graph*
//     (P-86 (8)), which is why the frozen configuration compiles nothing here.  `HAVE_NTGUI` is
//     the macro the Windows GUI build defines; it is *not* `WINDOWSNT` (the Cygwin GUI build
//     compiles this file with `HAVE_NTGUI` and without `WINDOWSNT`, and C 1541/1599 gate on the
//     latter), so the file's two `#ifdef WINDOWSNT`/`#else /* Cygwin */` pairs are carried as
//     `feature = "windowsnt"` inside the file gate;
//   * C 33-36 and C 54-88 are `#ifdef HAVE_HARFBUZZ`: the arm that loads HarfBuzz through DLL
//     pointers is `feature = "harfbuzz"` (the registered name, `femacs/xtask/Cargo.toml` 134),
//     and the `#else` arm is the empty one C's preprocessor leaves;
//   * C 1060-1065 is an `#if 0` region: C's own preprocessor never compiles it, so the port keeps
//     C's text inside a comment (the `cm.rs` 86 reading);
//   * C 1288-1292 is `#if GNUC_PREREQ (4, 3, 0)` / `#else` / `#endif` around one initialiser:
//     both arms are carried (the `src/xgselect.rs` shape) and the condition is a `macro_rules!` of
//     `include/conf_post.rs`.
//
// **Acquisition note - this piece is `text-only` (P-86 (3)(ii)).**  The machine has no Windows and
// no mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty,
// no Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`/`usp10.h`), so there is no
// `<windows.h>` and no `<usp10.h>` to compile or measure against: **every record and every value
// of the three stand-in sections below is `UNVERIFIED`** (the documented SDK text, not a
// measurement) and the layout assertions are owed.  What the round does show is that the
// `have_ntgui` + `windowsnt` + `have_window_system` arm type-checks under this machine's `rustc` -
// the ledger's `## Evidence` carries that reading and names the errors that belong to the include
// graph rather than to this unit.
//
// `docs/ports/w32uniscribe.md` carries the C line ranges, the acceptance readings, every deviation
// (the dropped forward declarations, the three stand-in sections, the in-unit declarations no
// mirror carries, the `#if 0` text, the `goto`s) and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, unused_must_use,
         let_underscore_drop, clippy::all)]

/* C 1-18 is the file's opening ownership/title block, dropped (PORTING.md 0.1-19); the port's
   first line is the revision line above.  C 19-36 is the include graph, C 38-45 the mirrors, and
   C 29-30 the SDK version override.  */

include!("include/config.rs");                  // C 21: `#include <config.h>`

/* C 22-30: "Override API version - Uniscribe is only available as standard since Windows 2000,
   though most users of older systems will have it since it installs with Internet Explorer 5.0 and
   other software.  Also, MinGW64 w32api headers by default define OPENTYPE_TAG typedef only if
   _WIN32_WINNT >= 0x0600.  We only use the affected APIs if they are available, so there is no
   chance of calling non-existent functions."

   C 29-30 override that *SDK* macro for this translation unit only.  The port cannot redefine the
   crate-root name: `include/conf_post.rs` 191 carries C's own `#define _WIN32_WINNT 0x500` and
   `include!` is textual, so two same-named `const`s in one scope would be `E0428`.  The override
   is therefore the module-scoped constant below, which the unit's `use` brings into scope (the
   P-39/P-21 shadowing mechanism); nothing in the port reads the name - it stands for the SDK
   version the `<windows.h>`/`<usp10.h>` stand-in sections are written against.  */
#[cfg (feature = "have_ntgui")]
mod win32_winnt_override
{
  /// C: #undef _WIN32_WINNT
  /// C: #define _WIN32_WINNT 0x0600
  pub const _WIN32_WINNT: core::ffi::c_uint = 0x0600;
}
#[cfg (feature = "have_ntgui")]
use win32_winnt_override::*;

/* ----------------------------------------------------------------------------------------------
   C 31: `#include <windows.h>`.

   No file in the reference tree carries `<windows.h>` (P-88 (1): the declaration site is C's own
   `#include` line, and no file is added under `femacs/src/include/`), so the names *this file*
   takes from it stand here, each with the SDK's own text above it.  The spellings repeat
   `src/w32font.rs`'s and `src/w32select.rs`'s sections (`DWORD` is the 32-bit Windows unsigned
   whatever this target's `c_long` is, `HANDLE`/`HDC`/`HFONT`/`HMODULE` are `void *`, `BOOL` is
   C's `int`, `WINAPI`/`CALLBACK` are `extern "system"`), so that a Windows translation unit sees
   one set of names.

   **No measurement was possible**: this machine has no `<windows.h>` (see the file note), so every
   scalar is spelled from PORTING.md 1.1's fixed Windows widths and every record below is
   **UNVERIFIED** - its size, offsets and values are the documented ones and the layout assertions
   are owed (the ledger's `## Owed`).
   ---------------------------------------------------------------------------------------------- */

/// C 31's `<windows.h>`: the scalar types (`windef.h`/`winnt.h`).
/// C: typedef unsigned char BYTE; typedef unsigned short WORD; typedef unsigned short WCHAR;
/// C: typedef unsigned long DWORD; typedef long LONG; typedef unsigned long ULONG;
/// C: typedef unsigned int UINT; typedef int BOOL; typedef long HRESULT;
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
/// `HRESULT` is `LONG` in the SDK (`winerror.h`'s `typedef long HRESULT;`), which is the width the
/// `Script*` entry points return and the `SUCCEEDED`/`FAILED` pair below tests.
#[cfg (feature = "have_ntgui")]
pub type HRESULT = LONG;
/// C's handle names are all `void *` at the ABI (the `include/w32gui.rs` reading); `HGDIOBJ` is
/// the GDI object handle `SelectObject` returns and `DeleteObject` takes, so `HFONT` is an alias
/// of it and C's implicit conversions at the assignment sites need no cast.
/// C: typedef void *HANDLE; typedef void *HDC; typedef void *HFONT; typedef void *HGDIOBJ;
/// C: typedef void *HMODULE; typedef void *LPVOID;
/// C: typedef DWORD COLORREF; typedef LONG_PTR LPARAM;
#[cfg (feature = "have_ntgui")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HDC = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HGDIOBJ = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HFONT = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HMODULE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type COLORREF = DWORD;
#[cfg (feature = "have_ntgui")]
pub type LPARAM = isize;
/// C's `WINAPI` is `__stdcall`, which Rust spells `extern "system"` (the `src/w32fns.rs`
/// convention); the note stays on the declarations below.  `CALLBACK` is the same attribute for
/// the `EnumFontFamiliesEx` callback C 737 declares, and `FALSE`/`TRUE` are `<windef.h>`'s.
/// C: #define WINAPI __stdcall
/// C: #define CALLBACK __stdcall
/// C: #define FALSE 0
/// C: #define TRUE 1
#[cfg (feature = "have_ntgui")]
pub const FALSE: BOOL = 0;
#[cfg (feature = "have_ntgui")]
pub const TRUE: BOOL = 1;

/// C 31's `<windows.h>`: the return-value pair the six `Script*` calls are tested with
/// (`winerror.h`) and the two statuses the shaping code compares against.
/// C: #define SUCCEEDED(hr) (((HRESULT) (hr)) >= 0)
/// C: #define FAILED(hr) (((HRESULT) (hr)) < 0)
/// C: #define E_OUTOFMEMORY _HRESULT_TYPEDEF_ (0x8007000EL)
/// C: #define E_PENDING _HRESULT_TYPEDEF_ (0x8007000AL)
macro_rules! SUCCEEDED { ($hr:expr) => { (($hr) as HRESULT) >= 0 }; }
macro_rules! FAILED { ($hr:expr) => { (($hr) as HRESULT) < 0 }; }
#[cfg (feature = "have_ntgui")]
pub const E_OUTOFMEMORY: HRESULT = 0x8007000Eu32 as HRESULT;
#[cfg (feature = "have_ntgui")]
pub const E_PENDING: HRESULT = 0x8007000Au32 as HRESULT;

/// C 31's `<windows.h>`: the three `windef.h` geometry records.  This file itself does not name
/// them, but `include/w32term.rs`'s two entry points (`w32_fullscreen_rect`, `w32_fill_rect`) do
/// and C's own translation unit has them from the same header (the `src/w32font.rs` 143-181 set).
/// C: typedef struct tagPOINT { LONG x; LONG y; } POINT;
/// C: typedef struct tagSIZE { LONG cx; LONG cy; } SIZE;
/// C: typedef struct tagRECT { LONG left; LONG top; LONG right; LONG bottom; } RECT;
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

/// C 31's `<windows.h>`: `tagLOGFONTA` (`wingdi.h`), the record C 140 zeroes and C 145 sets
/// `lfOutPrecision` on, and whose `lfCharSet`/`lfFaceName` the family enumeration reads.
/// C: typedef struct tagLOGFONTA { LONG lfHeight; LONG lfWidth; LONG lfEscapement;
/// C:   LONG lfOrientation; LONG lfWeight; BYTE lfItalic; BYTE lfUnderline; BYTE lfStrikeOut;
/// C:   BYTE lfCharSet; BYTE lfOutPrecision; BYTE lfClipPrecision; BYTE lfQuality;
/// C:   BYTE lfPitchAndFamily; CHAR lfFaceName[LF_FACESIZE]; } LOGFONTA;
#[cfg (feature = "have_ntgui")]
pub const LF_FACESIZE: c_int = 32;
#[cfg (feature = "have_ntgui")]
pub const LF_FULLFACESIZE: c_int = 64;
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

/// C 31's `<windows.h>`: the metrics records (`wingdi.h`).  `TEXTMETRICW` is the record
/// `include/w32font.rs`'s `w32font_info` embeds **by value** (so this unit must carry it for that
/// mirror to resolve), and `NEWTEXTMETRICW`/`FONTSIGNATURE` are the `NEWTEXTMETRICEX` C 739
/// receives - `ntmTm.ntmFlags` and `ntmFontSig.fsUsb` are the two members C 752/757 read.
/// C: typedef struct tagTEXTMETRICW { LONG tmHeight; LONG tmAscent; LONG tmDescent;
/// C:   LONG tmInternalLeading; LONG tmExternalLeading; LONG tmAveCharWidth; LONG tmMaxCharWidth;
/// C:   LONG tmWeight; LONG tmOverhang; LONG tmDigitizedAspectX; LONG tmDigitizedAspectY;
/// C:   WCHAR tmFirstChar; WCHAR tmLastChar; WCHAR tmDefaultChar; WCHAR tmBreakChar;
/// C:   BYTE tmItalic; BYTE tmUnderlined; BYTE tmStruckOut; BYTE tmPitchAndFamily;
/// C:   BYTE tmCharSet; } TEXTMETRICW;
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

/// C 31's `<windows.h>`: `tagENUMLOGFONTEXA` (`wingdi.h`), the callback's first parameter.
/// `elfLogFont` is a `LOGFONTA` in this translation unit's SDK (`ENUMLOGFONTEX` is the A variant
/// where `UNICODE` is not defined - the same variant C's `EnumFontFamiliesEx` is compiled in).
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

/// C 31's `<windows.h>`: the `EnumFontFamiliesEx` callback type (`CALLBACK` = `extern "system"`)
/// and the two `lfOutPrecision`/`font_type` values C 145 and C 752-753 use.
/// C: typedef int (CALLBACK *FONTENUMPROCA) (const LOGFONTA *, const TEXTMETRICA *,
/// C:                                        DWORD, LPARAM);
/// C: #define OUT_OUTLINE_PRECIS 8
/// C: #define TRUETYPE_FONTTYPE 0x0004
/// C: #define ETO_GLYPH_INDEX 0x0010
#[cfg (feature = "have_ntgui")]
pub type FONTENUMPROC = Option<unsafe extern "system" fn (*mut ENUMLOGFONTEX,
                                                       *mut NEWTEXTMETRICEX, DWORD, LPARAM)
                                                      -> c_int>;
#[cfg (feature = "have_ntgui")]
pub const OUT_OUTLINE_PRECIS: BYTE = 8;
#[cfg (feature = "have_ntgui")]
pub const TRUETYPE_FONTTYPE: DWORD = 0x0004;
/// C: #define ETO_GLYPH_INDEX 0x0010 (`wingdi.h`); `include/w32font.rs`'s `w32font_info.glyph_idx`
/// is `unsigned int` and C 189 assigns this flag to it.
#[cfg (feature = "have_ntgui")]
pub const ETO_GLYPH_INDEX: c_uint = 0x0010;

/// C 31's `<windows.h>`: the entry points this file calls.  Each is the SDK's own name; a Windows
/// target supplies whichever variant its `#define`s pick.
/// C: DWORD WINAPI GetFontData (HDC, DWORD, DWORD, LPVOID, DWORD);
/// C: int WINAPI EnumFontFamiliesExA (HDC, LPLOGFONTA, FONTENUMPROCA, LPARAM, DWORD);
/// C: HFONT WINAPI CreateFontIndirectA (const LOGFONTA *);
/// C: HGDIOBJ WINAPI SelectObject (HDC, HGDIOBJ);
/// C: BOOL WINAPI DeleteObject (HGDIOBJ);
/// C: HMODULE WINAPI LoadLibraryA (LPCSTR);
/// C: BOOL WINAPI FreeLibrary (HMODULE);
/// C: HMODULE WINAPI GetModuleHandleA (LPCSTR);
/// **UNVERIFIED** (no `<windows.h>` on this machine; ledger `## Owed`).
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetFontData (hdc: HDC, dwTable: DWORD, dwOffset: DWORD, pvBuffer: LPVOID,
                      cjBuffer: DWORD) -> DWORD;
  pub fn EnumFontFamiliesEx (hdc: HDC, lpLogfont: *mut LOGFONT, lpProc: FONTENUMPROC,
                             lParam: LPARAM, dwFlags: DWORD) -> c_int;
  pub fn CreateFontIndirect (lplf: *const LOGFONT) -> HFONT;
  pub fn SelectObject (hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
  pub fn DeleteObject (ho: HGDIOBJ) -> BOOL;
  pub fn LoadLibrary (lpLibFileName: *const c_char) -> HMODULE;
  pub fn FreeLibrary (hLibModule: HMODULE) -> BOOL;
  pub fn GetModuleHandle (lpModuleName: *const c_char) -> HMODULE;
}

/* ----------------------------------------------------------------------------------------------
   C 32: `#include <usp10.h>`.

   No file in the reference tree carries `<usp10.h>` either (P-88 (1)), so the Uniscribe surface
   this file names stands here: the six `Script*` entry points C 95-116 takes the addresses of and
   calls through pointers, and the seven records/aliases those signatures use.  Every record is
   **UNVERIFIED** (there is no `usp10.h` on this machine to compile against or measure), and the
   bitfield runs are the P-80 spelling (one storage unit plus named accessors), not a layout
   reading.
   ---------------------------------------------------------------------------------------------- */

/// C 32's `<usp10.h>`: the two aliases the signatures are written with.
/// C: typedef void *SCRIPT_CACHE;
/// C: typedef ULONG OPENTYPE_TAG;
/// `SCRIPT_CACHE` is the opaque cache handle C 216/C 366 passes as `(SCRIPT_CACHE)
/// &(uniscribe_font->cache)` - that member is `void *` (`include/w32font.rs`), so the port's
/// `*mut SCRIPT_CACHE` parameter and `&raw mut …cache` agree with C's cast without one.
#[cfg (feature = "have_ntgui")]
pub type SCRIPT_CACHE = *mut c_void;
/// **UNVERIFIED**: the SDK spells the tag a 32-bit unsigned (`ULONG`/`DWORD` in the two header
/// generations); both are `u32`, which is what C's `OTF_TAG` value fits in.
#[cfg (feature = "have_ntgui")]
pub type OPENTYPE_TAG = u32;

/// C 32's `<usp10.h>`: `tagSCRIPT_CONTROL`, the record C 334/644 pass as `NULL` only (so the
/// port carries the pointer-only form C's own use has; the field run is documented in C's text).
/// C: typedef struct tagSCRIPT_CONTROL { DWORD uDefaultLanguage : 16; DWORD fContextDigits : 1;
/// C:   DWORD fInvertPreBoundDir : 1; DWORD fInvertPostBoundDir : 1; DWORD fLinkStringBefore : 1;
/// C:   DWORD fLinkStringAfter : 1; DWORD fNeutralOverride : 1; DWORD fNumericOverride : 1;
/// C:   DWORD fLegacyBidiClass : 1; DWORD fMergeNeutralItems : 1; DWORD fUseStandardBidi : 1;
/// C:   DWORD fReserved : 6; } SCRIPT_CONTROL;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct SCRIPT_CONTROL
{
  _private: [u8; 0],
}

/// C 32's `<usp10.h>`: `tagSCRIPT_STATE`, embedded **by value** in `SCRIPT_ANALYSIS` below, so its
/// two bytes are part of that record's size (C never reads a member of it).
/// C: typedef struct tagSCRIPT_STATE { WORD uBidiLevel : 5; WORD fOverrideDirection : 1;
/// C:   WORD fInhibitSymSwap : 1; WORD fCharShape : 1; WORD fDigitSubstitute : 1;
/// C:   WORD fInhibitLigate : 1; WORD fDisplayZWG : 1; WORD fArabicNumContext : 1;
/// C:   WORD fGcpClusters : 1; WORD fReserved : 1; WORD fEngineReserved : 2; } SCRIPT_STATE;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct SCRIPT_STATE
{
  /// C 32's eleven `WORD` bitfields, one 16-bit storage unit (P-80).
  pub bits: WORD,
}

/// C 32's `<usp10.h>`: `tagSCRIPT_ANALYSIS`, the record C 361 writes `fLogicalOrder` into and C
/// 368/C 399/C 410/C 490/C 548/C 552 read `fNoGlyphIndex`/`fRTL` from.  C 365-369 and C 408-423
/// pass `&(items[i].a)` as the `SCRIPT_ANALYSIS *` argument, which is why the members are reached
/// through the accessors below rather than through a field.
/// C: typedef struct tagSCRIPT_ANALYSIS { WORD eScript : 10; WORD fRTL : 1; WORD fLayoutRTL : 1;
/// C:   WORD fLinkBefore : 1; WORD fLinkAfter : 1; WORD fLogicalOrder : 1;
/// C:   WORD fNoGlyphIndex : 1; SCRIPT_STATE s; } SCRIPT_ANALYSIS;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct SCRIPT_ANALYSIS
{
  /// C 32's six `WORD` bitfields, one 16-bit storage unit (P-80).
  pub bits: WORD,
  pub s: SCRIPT_STATE,
}

/// C 32's `<usp10.h>`: `tagSCRIPT_ITEM`, the run-record array C 332/C 643 allocate.  C reads
/// `items[i].a` and `items[i+1].iCharPos - items[i].iCharPos` (C 357).
/// C: typedef struct tagSCRIPT_ITEM { int iCharPos; SCRIPT_ANALYSIS a; } SCRIPT_ITEM;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct SCRIPT_ITEM
{
  pub iCharPos: c_int,
  pub a: SCRIPT_ANALYSIS,
}

/// C 32's `<usp10.h>`: `tagSCRIPT_VISATTR`, the per-glyph attribute array C 350 allocates and C
/// 453 reads `fClusterStart` from (also C 495/C 548).
/// C: typedef struct tagSCRIPT_VISATTR { WORD uJustification : 4; WORD fClusterStart : 1;
/// C:   WORD fDiacritic : 1; WORD fZeroWidth : 1; WORD fReserved : 1; WORD fShapeReserved : 8; }
/// C:   SCRIPT_VISATTR;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct SCRIPT_VISATTR
{
  /// C 32's seven `WORD` bitfields, one 16-bit storage unit (P-80).
  pub bits: WORD,
}

/// C 32's `<usp10.h>`: the two output records C 305/C 518 declares - `ABC` for the per-glyph
/// metrics C 532-533 reads (`abcA`, `abcB`) and `GOFFSET` for the placement deltas C 544/C 560/C
/// 567/C 572 reads (`du`, `dv`).
/// C: typedef struct _ABC { int abcA; UINT abcB; int abcC; } ABC;   (wingdi.h)
/// C: typedef struct tagGOFFSET { LONG du; LONG dv; } GOFFSET;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct ABC
{
  pub abcA: c_int,
  pub abcB: UINT,
  pub abcC: c_int,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct GOFFSET
{
  pub du: LONG,
  pub dv: LONG,
}

/* `SCRIPT_ANALYSIS.bits` and `SCRIPT_VISATTR.bits`, the members C names, in the order the SDK's
   declarations put them and at the positions their widths give (P-80; **no layout reading per
   record is possible here** - the bits are the documented run, and only the members this file
   touches have names).  */
#[cfg (feature = "have_ntgui")]
pub const SCRIPT_ANALYSIS_FRTL: WORD = 1 << 10;                 // C 32's `fRTL : 1`
#[cfg (feature = "have_ntgui")]
pub const SCRIPT_ANALYSIS_FLOGICALORDER: WORD = 1 << 14;        // C 32's `fLogicalOrder : 1`
#[cfg (feature = "have_ntgui")]
pub const SCRIPT_ANALYSIS_FNOGLYPHINDEX: WORD = 1 << 15;        // C 32's `fNoGlyphIndex : 1`
#[cfg (feature = "have_ntgui")]
pub const SCRIPT_VISATTR_FCLUSTERSTART: WORD = 1 << 4;          // C 32's `fClusterStart : 1`

/// Read one `WORD : 1` member of `SCRIPT_ANALYSIS.bits` - C's `(A).fRTL` (zero-extended, as an
/// unsigned C bitfield of that width is).
#[cfg (feature = "have_ntgui")]
#[inline(always)]
pub fn script_analysis_bool_p (a: *const SCRIPT_ANALYSIS, mask: WORD) -> bool
{
  unsafe { ((*a).bits & mask) != 0 }
}

/// Assign one `WORD : 1` member of `SCRIPT_ANALYSIS.bits` - C's `(A).fLogicalOrder = VALUE`;
/// only the member's bit changes.
#[cfg (feature = "have_ntgui")]
#[inline(always)]
pub fn script_analysis_bool_set (a: *mut SCRIPT_ANALYSIS, mask: WORD, value: bool)
{
  unsafe {
    let bits = &mut (*a).bits;
    *bits = if value { *bits | mask } else { *bits & !mask };
  }
}

/// The member names C spells on `SCRIPT_ANALYSIS`, so every use site reads like C's text
/// (`script_analysis_bits! (fRTL, (*items.offset (i as isize)).a)`).
/// C: `(items[i].a).fRTL`, `(items[i].a).fLogicalOrder = 1`, `(items[i].a).fNoGlyphIndex`
macro_rules! script_analysis_bits {
  (fRTL, $a:expr) => { script_analysis_bool_p (&($a), SCRIPT_ANALYSIS_FRTL) };
  (fLogicalOrder, $a:expr) => { script_analysis_bool_p (&($a), SCRIPT_ANALYSIS_FLOGICALORDER) };
  (fLogicalOrder, $a:expr, $value:expr) =>
    { script_analysis_bool_set (&mut ($a), SCRIPT_ANALYSIS_FLOGICALORDER, $value) };
  (fNoGlyphIndex, $a:expr) => { script_analysis_bool_p (&($a), SCRIPT_ANALYSIS_FNOGLYPHINDEX) };
}

/// Read one `WORD : 1` member of `SCRIPT_VISATTR.bits` - C's `attributes[j].fClusterStart`.
#[cfg (feature = "have_ntgui")]
#[inline(always)]
pub fn script_visattr_bool_p (v: *const SCRIPT_VISATTR, mask: WORD) -> bool
{
  unsafe { ((*v).bits & mask) != 0 }
}
#[cfg (feature = "have_ntgui")]
macro_rules! script_visattr_bits {
  (fClusterStart, $v:expr) => { script_visattr_bool_p (&($v), SCRIPT_VISATTR_FCLUSTERSTART) };
}

/// C 32's `<usp10.h>`: the six entry points C 95-116 declares pointers to, C 1546-1554 resolves by
/// name, and the `#else /* Cygwin */` arm C 1567-1572 takes the addresses of.  `WINAPI` is
/// `extern "system"`.
/// C: HRESULT WINAPI ScriptItemize (const WCHAR *, int, int, const SCRIPT_CONTROL *,
/// C:                               const SCRIPT_STATE *, SCRIPT_ITEM *, int *);
/// C: HRESULT WINAPI ScriptShape (HDC, SCRIPT_CACHE *, const WCHAR *, int, int,
/// C:                              SCRIPT_ANALYSIS *, WORD *, WORD *, SCRIPT_VISATTR *, int *);
/// C: HRESULT WINAPI ScriptPlace (HDC, SCRIPT_CACHE *, const WORD *, int, const SCRIPT_VISATTR *,
/// C:                              SCRIPT_ANALYSIS *, int *, GOFFSET *, ABC *);
/// C: HRESULT WINAPI ScriptGetGlyphABCWidth (HDC, SCRIPT_CACHE *, WORD, ABC *);
/// C: HRESULT WINAPI ScriptFreeCache (SCRIPT_CACHE *);
/// C: HRESULT WINAPI ScriptGetCMap (HDC, SCRIPT_CACHE *, const WCHAR *, int, DWORD, WORD *);
/// **UNVERIFIED** (no `usp10.h` here; ledger `## Owed`).
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn ScriptItemize (pwcInChars: *const WCHAR, cInChars: c_int, cMaxItems: c_int,
                        psControl: *const SCRIPT_CONTROL, psState: *const SCRIPT_STATE,
                        pItems: *mut SCRIPT_ITEM, pcItems: *mut c_int) -> HRESULT;
  pub fn ScriptShape (hdc: HDC, psc: *mut SCRIPT_CACHE, pwcChars: *const WCHAR,
                      cChars: c_int, cMaxGlyphs: c_int, psa: *mut SCRIPT_ANALYSIS,
                      pwOutGlyphs: *mut WORD, pwLogClust: *mut WORD,
                      psva: *mut SCRIPT_VISATTR, pcGlyphs: *mut c_int) -> HRESULT;
  pub fn ScriptPlace (hdc: HDC, psc: *mut SCRIPT_CACHE, pwGlyphs: *const WORD, cGlyphs: c_int,
                      psva: *const SCRIPT_VISATTR, psa: *mut SCRIPT_ANALYSIS, piAdvance: *mut c_int,
                      pGoffset: *mut GOFFSET, pABC: *mut ABC) -> HRESULT;
  pub fn ScriptGetGlyphABCWidth (hdc: HDC, psc: *mut SCRIPT_CACHE, wGlyph: WORD,
                                 pABC: *mut ABC) -> HRESULT;
  pub fn ScriptFreeCache (psc: *mut SCRIPT_CACHE) -> HRESULT;
  pub fn ScriptGetCMap (hdc: HDC, psc: *mut SCRIPT_CACHE, pwcInChars: *const WCHAR,
                        cChars: c_int, dwFlags: DWORD, pwOutGlyphs: *mut WORD) -> HRESULT;
}

/* ----------------------------------------------------------------------------------------------
   C 33-36: `#ifdef HAVE_HARFBUZZ` / `# include <hb.h>` / `# include <hb-ot.h>` / `#endif`.

   HarfBuzz has more than one consumer in the reference (`src/ftfont.c`, `src/sfntfont.c` and this
   file all `#include <hb.h>`), so P-90 (1) says that surface belongs in a mirror of its own,
   `femacs/src/include/hb.rs`; this lane may not create it, so the names *this file* reads stand
   here for now and the ledger's `## Owed` names the move (the `src/hbfont.rs` 30-36 shape, whose
   stand-in section is this one's sibling).  The `#else` arm is the empty text C's preprocessor
   leaves.  The values below are `src/hbfont.rs`'s clang readings of this machine's
   `/opt/homebrew/include/harfbuzz` (14.4.0, that ledger's Evidence); nothing here is a new
   measurement.
   ---------------------------------------------------------------------------------------------- */

/// C 34's `<hb.h>` (`hb-common.h`): the three scalar aliases this file's signatures name.
/// C: typedef int hb_bool_t; typedef uint32_t hb_codepoint_t; typedef uint32_t hb_tag_t;
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
pub type hb_bool_t = c_int;
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
pub type hb_codepoint_t = u32;
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
pub type hb_tag_t = u32;

/// C 34's `<hb.h>`: the two opaque records this file handles (`hb_blob_t` is C 62/C 1280's result,
/// `hb_face_t` C 65/C 1281's parameter, `hb_font_t` C 1321's result) and the two function-pointer
/// aliases `hb_blob_create`/`hb_face_create_for_tables` take.
/// C: typedef struct hb_blob_t hb_blob_t; typedef struct hb_face_t hb_face_t;
/// C: typedef struct hb_font_t hb_font_t;
/// C: typedef void (*hb_destroy_func_t) (void *user_data);
/// C: typedef hb_blob_t *(*hb_reference_table_func_t) (hb_face_t *face, hb_tag_t tag,
/// C:                                                 void *user_data);
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
#[repr(C)]
pub struct hb_blob_t
{
  _private: [u8; 0],
}
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
#[repr(C)]
pub struct hb_face_t
{
  _private: [u8; 0],
}
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
#[repr(C)]
pub struct hb_font_t
{
  _private: [u8; 0],
}
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
pub type hb_destroy_func_t = Option<unsafe extern "C" fn (user_data: *mut c_void)>;
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
pub type hb_reference_table_func_t
  = Option<unsafe extern "C" fn (face: *mut hb_face_t, tag: hb_tag_t, user_data: *mut c_void)
                                 -> *mut hb_blob_t>;

/// C 34's `<hb.h>`: `hb_memory_mode_t` and the one member C 1307 passes
/// (`HB_MEMORY_MODE_READONLY`).
/// C: typedef enum { HB_MEMORY_MODE_DUPLICATE, HB_MEMORY_MODE_READONLY, HB_MEMORY_MODE_WRITABLE,
/// C:   HB_MEMORY_MODE_READONLY_MAY_MAKE_WRITABLE } hb_memory_mode_t;
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
pub type hb_memory_mode_t = c_int;
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
pub const HB_MEMORY_MODE_READONLY: hb_memory_mode_t = 1;

/// C 34-35's `<hb.h>`/`<hb-ot.h>`: the ten entry points C 62-76 declares as DLL functions.  The
/// bodies reach them through the `fn_hb_*` pointers (C 78-87's renaming `#define`s), so these
/// declarations are what C's own `#include` gives and no call site names them directly.
/// C: hb_blob_t *hb_blob_create (const char *, unsigned int, hb_memory_mode_t, void *,
/// C:                             hb_destroy_func_t);
/// C: hb_face_t *hb_face_create_for_tables (hb_reference_table_func_t, void *, hb_destroy_func_t);
/// C: unsigned hb_face_get_glyph_count (const hb_face_t *);
/// C: hb_font_t *hb_font_create (hb_face_t *);
/// C: void hb_font_destroy (hb_font_t *); void hb_face_destroy (hb_face_t *);
/// C: unsigned int hb_face_get_upem (hb_face_t *);
/// C: hb_bool_t hb_font_get_nominal_glyph (hb_font_t *, hb_codepoint_t, hb_codepoint_t *);
/// C: hb_bool_t hb_font_get_variation_glyph (hb_font_t *, hb_codepoint_t, hb_codepoint_t,
/// C:                                        hb_codepoint_t *);
/// C: void hb_ot_font_set_funcs (hb_font_t *);
/// **UNVERIFIED** for the ABI surface: the declarations are this machine's header's (read by
/// `src/hbfont.rs`), not a Windows HarfBuzz build's.
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
unsafe extern "C" {
  pub fn hb_blob_create (data: *const c_char, length: c_uint, mode: hb_memory_mode_t,
                         user_data: *mut c_void, destroy: hb_destroy_func_t) -> *mut hb_blob_t;
  pub fn hb_face_create_for_tables (reference_table_func: hb_reference_table_func_t,
                                    user_data: *mut c_void, destroy: hb_destroy_func_t)
                                   -> *mut hb_face_t;
  pub fn hb_face_get_glyph_count (face: *const hb_face_t) -> c_uint;
  pub fn hb_font_create (face: *mut hb_face_t) -> *mut hb_font_t;
  pub fn hb_font_destroy (font: *mut hb_font_t);
  pub fn hb_face_destroy (face: *mut hb_face_t);
  pub fn hb_face_get_upem (face: *mut hb_face_t) -> c_uint;
  pub fn hb_font_get_nominal_glyph (font: *mut hb_font_t, unicode: hb_codepoint_t,
                                    glyph: *mut hb_codepoint_t) -> hb_bool_t;
  pub fn hb_font_get_variation_glyph (font: *mut hb_font_t, unicode: hb_codepoint_t,
                                      variation_selector: hb_codepoint_t,
                                      glyph: *mut hb_codepoint_t) -> hb_bool_t;
  pub fn hb_ot_font_set_funcs (font: *mut hb_font_t);
}

/* C 38-45: the reference tree's own headers, each its mirror; all inside the file gate (P-86 (8)).
   C 28-30's `<stdcountof.h>` and `<string.h>` edges are spelled where C's chain needs them (C
   885's `countof`, and `memset`/`memcpy` C 143/C 786, which `include/lisp.rs` 27 already carries);
   C's `malloc`/`free` (C 1275/C 1301) come from the SDK's own `<stdlib.h>`, whose mirror is
   `lib/include/stdlib.rs`.  `include/conf_post.rs` is C's `GNUC_PREREQ`/`DebPrint` (C 1288/C
   895-947); inside the gate, because the whole translation unit is (Deviation 4).  */
#[cfg (feature = "have_ntgui")]
include!("../lib/include/stdcountof.rs");      // lisp.h 28: `#include <stdcountof.h>` (`countof`, C 885)
#[cfg (feature = "have_ntgui")]
include!("../lib/include/stdlib.rs");          // C 19-31's chain: `<stdlib.h>` (`malloc`, `free`)
#[cfg (feature = "have_ntgui")]
include!("../lib/include/byteswap.in.rs");     // gnulib's `bswap_32` (C 1291's `#else` arm)
#[cfg (feature = "have_ntgui")]
include!("include/conf_post.rs");              // config.h's AH_BOTTOM: `GNUC_PREREQ`, `DebPrint`
#[cfg (feature = "have_ntgui")]
include!("include/lisp.rs");                   // C 38: `#include "lisp.h"`
#[cfg (feature = "have_ntgui")]
include!("include/w32term.rs");                // C 39: `#include "w32term.h"`
#[cfg (feature = "have_ntgui")]
include!("include/frame.rs");                  // C 40: `#include "frame.h"`
#[cfg (feature = "have_ntgui")]
include!("include/composite.rs");              // C 41: `#include "composite.h"` (LGSTRING/LGLYPH)
#[cfg (feature = "have_ntgui")]
include!("include/font.rs");                   // C 42: `#include "font.h"`
#[cfg (feature = "have_ntgui")]
include!("include/w32font.rs");                // C 43: `#include "w32font.h"` (`uniscribe_font_info`)
#[cfg (feature = "have_ntgui")]
include!("include/pdumper.rs");                // C 44: `#include "pdumper.h"`
#[cfg (feature = "have_ntgui")]
include!("include/w32common.rs");              // C 45: `#include "w32common.h"` (`get_proc_addr`)

/* C 47: `extern int uniscribe_available;` - dropped (PORTING.md 0.1-12); the definition is C 48.
   The name is non-`static` in C (`src/w32fns.c` 307 spells the same `extern int` there), so the
   definition carries `#[unsafe(no_mangle)]`; the declaration C 47 writes lives in the caller's own
   unit until a mirror owns it (ledger `## Owed`).  */
/// C: extern int uniscribe_available; int uniscribe_available = 0;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut uniscribe_available: c_int = 0;

/* C 50-53: the `EnumFontFamiliesEx` callback's forward declaration - dropped (PORTING.md 0.1-12);
   the definition is C 737-768.  `CALLBACK`/`ALIGN_STACK` are `extern "system"` plus the attribute
   `include/w32term.rs` 37-39 records (the `src/w32font.rs` 1925 reading).  */
/// C: static int CALLBACK ALIGN_STACK add_opentype_font_name_to_list (ENUMLOGFONTEX *,
/// C:   NEWTEXTMETRICEX *, DWORD, LPARAM);

/* C 54-88: `#ifdef HAVE_HARFBUZZ`.  The DLL half: the driver's own storage, the counter
   `src/w32fns.c` 308 reads, the ten `DEF_DLL_FN` declarations and the ten `#define`s that rename
   each entry point to its loaded pointer.  */
/// C: struct font_driver harfbuzz_font_driver;   (a tentative definition: zero-initialised)
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
#[unsafe(no_mangle)]
pub static mut harfbuzz_font_driver: font_driver = unsafe { core::mem::zeroed () };

/* C 57: `extern int harfbuzz_available;` - dropped (0.1-12); the definition is C 58.  */
/// C: extern int harfbuzz_available; int harfbuzz_available = 0;
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
#[unsafe(no_mangle)]
pub static mut harfbuzz_available: c_int = 0;

/* C 60-87: "Typedefs for HarfBuzz functions which we call through function pointers initialized
   after we load the HarfBuzz DLL."  C's `DEF_DLL_FN` / `LOAD_DLL_FN` are `w32common.h` 77-89's
   macros, which `include/w32common.rs` keeps `deferred` (P-22: identifier pasting has no
   `macro_rules!` counterpart), so this unit - its fourth consumer after `src/decompress.rs`,
   `src/lcms.rs` and `src/gnutls.rs`, the `src/hbfont.rs` 634-660 shape - repeats the long form
   (P-86 (11)).  C pastes `W32_PFN_##func`/`fn_##func` together, so both generated names are
   parameters; C's `__VA_ARGS__` (the parameter list) is the `$args` fragment and C's `CDECL` is
   `__cdecl`, empty on this target's ABI.  C 78-87's ten object-like `#define`s (`hb_blob_create` →
   `fn_hb_blob_create`, …) have no Rust counterpart, so every call site in the bodies below is
   spelled the way C's preprocessor leaves it - `(fn_hb_<name>.unwrap ()) (…)` - and the ten
   `#define` lines are carried as C's own text (Deviation 8):

     C 78  #define hb_blob_create fn_hb_blob_create
     C 79  #define hb_face_create_for_tables fn_hb_face_create_for_tables
     C 80  #define hb_face_get_glyph_count fn_hb_face_get_glyph_count
     C 81  #define hb_font_create fn_hb_font_create
     C 82  #define hb_font_destroy fn_hb_font_destroy
     C 83  #define hb_face_destroy fn_hb_face_destroy
     C 84  #define hb_face_get_upem fn_hb_face_get_upem
     C 85  #define hb_font_get_nominal_glyph fn_hb_font_get_nominal_glyph
     C 86  #define hb_font_get_variation_glyph fn_hb_font_get_variation_glyph
     C 87  #define hb_ot_font_set_funcs fn_hb_ot_font_set_funcs
  */
/// C: w32common.h 77-79: `#define DEF_DLL_FN(type, func, ...) typedef type (CDECL *W32_PFN_##func)
/// C:   __VA_ARGS__; static W32_PFN_##func fn_##func`
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
macro_rules! DEF_DLL_FN {
  ($type:ty, $func:ident, $pfn:ident, $fnvar:ident, ($($arg:ty),* $(,)?)) => {
    pub type $pfn = Option<unsafe extern "C" fn ($($arg),*) -> $type>;
    static mut $fnvar: $pfn = None;
  };
}
/// C: w32common.h 82-89: `#define LOAD_DLL_FN(lib, func) do { fn_##func = (W32_PFN_##func)
/// C:   get_proc_addr (lib, #func); if (!fn_##func) return false; } while (false)`
/// (`do { … } while (false)` is a block expression, C's `#func` is the stringified name - a
/// parameter here - and the conversion between the two function-pointer types is the `transmute`
/// C spells as a cast, the `src/hbfont.rs` 655 reading.)
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
macro_rules! LOAD_DLL_FN {
  ($lib:expr, $func:ident, $pfn:ident, $fnvar:ident, $name:expr) => {
    {
      $fnvar = std::mem::transmute::<VOIDFNPTR, $pfn> (get_proc_addr ($lib, $name.as_ptr ()));
      if $fnvar.is_none () { return false; }
    }
  };
}
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
DEF_DLL_FN! (*mut hb_blob_t, hb_blob_create, W32_PFN_hb_blob_create, fn_hb_blob_create,
             (*const c_char, c_uint, hb_memory_mode_t, *mut c_void, hb_destroy_func_t));
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
DEF_DLL_FN! (*mut hb_face_t, hb_face_create_for_tables, W32_PFN_hb_face_create_for_tables,
             fn_hb_face_create_for_tables, (hb_reference_table_func_t, *mut c_void,
                                            hb_destroy_func_t));
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
DEF_DLL_FN! (c_uint, hb_face_get_glyph_count, W32_PFN_hb_face_get_glyph_count,
             fn_hb_face_get_glyph_count, (*const hb_face_t));
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
DEF_DLL_FN! (*mut hb_font_t, hb_font_create, W32_PFN_hb_font_create, fn_hb_font_create,
             (*mut hb_face_t));
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
DEF_DLL_FN! ((), hb_font_destroy, W32_PFN_hb_font_destroy, fn_hb_font_destroy, (*mut hb_font_t));
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
DEF_DLL_FN! ((), hb_face_destroy, W32_PFN_hb_face_destroy, fn_hb_face_destroy, (*mut hb_face_t));
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
DEF_DLL_FN! (c_uint, hb_face_get_upem, W32_PFN_hb_face_get_upem, fn_hb_face_get_upem,
             (*mut hb_face_t));
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
DEF_DLL_FN! (hb_bool_t, hb_font_get_nominal_glyph, W32_PFN_hb_font_get_nominal_glyph,
             fn_hb_font_get_nominal_glyph, (*mut hb_font_t, hb_codepoint_t, *mut hb_codepoint_t));
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
DEF_DLL_FN! (hb_bool_t, hb_font_get_variation_glyph, W32_PFN_hb_font_get_variation_glyph,
             fn_hb_font_get_variation_glyph, (*mut hb_font_t, hb_codepoint_t, hb_codepoint_t,
                                              *mut hb_codepoint_t));
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
DEF_DLL_FN! ((), hb_ot_font_set_funcs, W32_PFN_hb_ot_font_set_funcs, fn_hb_ot_font_set_funcs,
             (*mut hb_font_t));

/* C 90-91: `static Lisp_Object otf_features (HDC context, const char *table);` - dropped
   (PORTING.md 0.1-12); the definition is C 1154-1239.  */
/// C: static Lisp_Object otf_features (HDC context, const char *table);

/* C 93-116: "Uniscribe function pointers."  Every one is NULL until C 1531's
   `syms_of_w32uniscribe_for_pdumper` resolves it, so each global is an `Option` of an
   `extern "system"` function (`WINAPI`; PORTING.md P-27's nullable function-pointer spelling).
   The signatures are C's own prototypes, character for character.  */
/// C: static HRESULT (WINAPI * pfnScriptItemize) (const WCHAR *, int, int,
/// C:   const SCRIPT_CONTROL *, const SCRIPT_STATE *, SCRIPT_ITEM *, int *);
#[cfg (feature = "have_ntgui")]
static mut pfnScriptItemize: Option<unsafe extern "system" fn (*const WCHAR, c_int, c_int,
                                                              *const SCRIPT_CONTROL,
                                                              *const SCRIPT_STATE,
                                                              *mut SCRIPT_ITEM, *mut c_int)
                                                             -> HRESULT> = None;
/// C: static HRESULT (WINAPI * pfnScriptShape) (HDC, SCRIPT_CACHE *, const WCHAR *, int, int,
/// C:   SCRIPT_ANALYSIS *, WORD *, WORD *, SCRIPT_VISATTR *, int *);
#[cfg (feature = "have_ntgui")]
static mut pfnScriptShape: Option<unsafe extern "system" fn (HDC, *mut SCRIPT_CACHE,
                                                             *const WCHAR, c_int, c_int,
                                                             *mut SCRIPT_ANALYSIS, *mut WORD,
                                                             *mut WORD, *mut SCRIPT_VISATTR,
                                                             *mut c_int) -> HRESULT> = None;
/// C: static HRESULT (WINAPI * pfnScriptPlace) (HDC, SCRIPT_CACHE *, const WORD *, int,
/// C:   const SCRIPT_VISATTR *, SCRIPT_ANALYSIS *, int *, GOFFSET *, ABC *);
#[cfg (feature = "have_ntgui")]
static mut pfnScriptPlace: Option<unsafe extern "system" fn (HDC, *mut SCRIPT_CACHE,
                                                             *const WORD, c_int,
                                                             *const SCRIPT_VISATTR,
                                                             *mut SCRIPT_ANALYSIS, *mut c_int,
                                                             *mut GOFFSET, *mut ABC) -> HRESULT>
  = None;
/// C: static HRESULT (WINAPI * pfnScriptGetGlyphABCWidth) (HDC, SCRIPT_CACHE *, WORD, ABC *);
#[cfg (feature = "have_ntgui")]
static mut pfnScriptGetGlyphABCWidth: Option<unsafe extern "system" fn (HDC, *mut SCRIPT_CACHE,
                                                                        WORD, *mut ABC)
                                                                       -> HRESULT> = None;
/// C: static HRESULT (WINAPI * pfnScriptFreeCache) (SCRIPT_CACHE *);
#[cfg (feature = "have_ntgui")]
static mut pfnScriptFreeCache: Option<unsafe extern "system" fn (*mut SCRIPT_CACHE) -> HRESULT>
  = None;
/// C: static HRESULT (WINAPI * pfnScriptGetCMap) (HDC, SCRIPT_CACHE *, const WCHAR *, int, DWORD,
/// C:   WORD *);
#[cfg (feature = "have_ntgui")]
static mut pfnScriptGetCMap: Option<unsafe extern "system" fn (HDC, *mut SCRIPT_CACHE,
                                                               *const WCHAR, c_int, DWORD,
                                                               *mut WORD) -> HRESULT> = None;

/* C 118-119: "Font backend interface implementation."  C 120-126 `uniscribe_list`.  The
   `font_driver` slots C 1475-1494 fills are P-72 (3)'s untyped `*mut c_void` for `struct frame *`,
   so the definition takes that spelling and casts at the top (the `src/w32font.rs` 975 reading);
   C's own prototype is the `/// C:` line.  */
/// C: static Lisp_Object uniscribe_list (struct frame *f, Lisp_Object font_spec)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn uniscribe_list (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{
  let f = f as *mut frame;						// C 121
  let fonts: Lisp_Object = w32font_list_internal (f, font_spec, true);	// C 123
  FONT_ADD_LOG! (c"uniscribe-list".as_ptr (), font_spec, fonts);	// C 124
  return fonts;								// C 125
}

/* C 128-134: `uniscribe_match`.  */
/// C: static Lisp_Object uniscribe_match (struct frame *f, Lisp_Object font_spec)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn uniscribe_match (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{
  let f = f as *mut frame;						// C 129
  let entity: Lisp_Object = w32font_match_internal (f, font_spec, true);	// C 131
  FONT_ADD_LOG! (c"uniscribe-match".as_ptr (), font_spec, entity);	// C 132
  return entity;							// C 133
}

/* C 136-162: `uniscribe_list_family`.  `dc` is declared then assigned (C 141/C 153) - the port
   zero-initialises it (PORTING.md 0.1-13) - and C 143's `memset` is kept as itself (PORTING.md
   3.3).  */
/// C: static Lisp_Object uniscribe_list_family (struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn uniscribe_list_family (f: *mut c_void) -> Lisp_Object
{
  let f = f as *mut frame;						// C 137
  let mut list: Lisp_Object = Qnil ();					// C 139
  let mut font_match_pattern: LOGFONT = core::mem::zeroed ();		// C 140
  let mut dc: HDC = core::ptr::null_mut ();				// C 141

  memset (&raw mut font_match_pattern as *mut c_void, 0,
	  core::mem::size_of::<LOGFONT> ());				// C 143
  /* Limit enumerated fonts to outline fonts to save time.  */
  font_match_pattern.lfOutPrecision = OUT_OUTLINE_PRECIS;		// C 145

  /* Prevent quitting while EnumFontFamiliesEx runs and conses the
     list it will return.  That's because get_frame_dc acquires the
     critical section, so we cannot quit before we release it in
     release_frame_dc.  */
  let prev_quit: Lisp_Object = Vinhibit_quit;				// C 151
  Vinhibit_quit = Qt ();						// C 152
  dc = get_frame_dc (f);						// C 153

  EnumFontFamiliesEx (dc, &raw mut font_match_pattern,		// C 155-157
                      Some (add_opentype_font_name_to_list),
                      (&raw mut list) as LPARAM, 0);
  release_frame_dc (f, dc);						// C 158
  Vinhibit_quit = prev_quit;						// C 159

  return list;								// C 161
}

/* C 164-199: `uniscribe_open`.  `VECSIZE (struct uniscribe_font_info)` is C 168's size argument
   and `FONT_TYPE_INDEX` is the `font_property_index` the mirror carries.  */
/// C: static Lisp_Object uniscribe_open (struct frame *f, Lisp_Object font_entity, int pixel_size)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn uniscribe_open (f: *mut c_void, font_entity: Lisp_Object, pixel_size: c_int)
				     -> Lisp_Object
{
  let f = f as *mut frame;						// C 165
  let font_object: Lisp_Object						// C 167-169
    = font_make_object (VECSIZE (core::mem::size_of::<uniscribe_font_info> ()) as c_int,
			font_entity, pixel_size);
  let uniscribe_font: *mut uniscribe_font_info				// C 170-171
    = XFONT_OBJECT (font_object) as *mut uniscribe_font_info;

  if ! NILP (AREF (font_entity, FONT_TYPE_INDEX as isize))		// C 173
    {
      ASET (font_object, FONT_TYPE_INDEX as isize,
	    AREF (font_entity, FONT_TYPE_INDEX as isize));		// C 174
    }
  else	/* paranoia: this should never happen */			// C 175
    {
      ASET (font_object, FONT_TYPE_INDEX as isize, Quniscribe ());	// C 176
    }

  if w32font_open_internal (f, font_entity, pixel_size, font_object) == 0	// C 178
    {
      return Qnil ();							// C 180
    }

  /* Initialize the cache for this font.  */
  (*uniscribe_font).cache = core::ptr::null_mut ();			// C 184
  (*uniscribe_font).dwrite_cache = core::ptr::null_mut ();		// C 185
  (*uniscribe_font).dwrite_skip_font = false;				// C 186

  /* Uniscribe and HarfBuzz backends use glyph indices.  */
  (*uniscribe_font).w32_font.glyph_idx = ETO_GLYPH_INDEX;		// C 189

  /* C 191-195: `#ifdef HAVE_HARFBUZZ` … `#endif` in front of C's `else` - the `if`/`else` pair is
     one arm of the condition (`feature = "harfbuzz"`), and the statement C's `else` falls through
     to (C 196) is the other arm's only statement (P-86 (1); the `src/alloc.rs` 5683 shape).  */
  #[cfg (feature = "harfbuzz")]
  {
    if EQ (AREF (font_object, FONT_TYPE_INDEX as isize), Qharfbuzz ())	// C 192
      {
	(*uniscribe_font).w32_font.font.driver
	  = (&raw const harfbuzz_font_driver) as *const c_void;		// C 193
      }
    else
      {
	(*uniscribe_font).w32_font.font.driver
	  = (&raw const uniscribe_font_driver) as *const c_void;	// C 196
      }
  }
  #[cfg (not (feature = "harfbuzz"))]
  {
    (*uniscribe_font).w32_font.font.driver
      = (&raw const uniscribe_font_driver) as *const c_void;		// C 196
  }

  return font_object;							// C 198
}

/* C 201-221: `uniscribe_close`.  C 207-214's `#ifdef HAVE_HARFBUZZ` arm ends at the `#endif`
   before C 215's statement, which is again the other arm's text.  */
/// C: static void uniscribe_close (struct font *font)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn uniscribe_close (font: *mut font)
{
  let uniscribe_font: *mut uniscribe_font_info				// C 204-205
    = font as *mut uniscribe_font_info;

  #[cfg (feature = "harfbuzz")]
  {
    w32_dwrite_free_cached_face ((*uniscribe_font).dwrite_cache);	// C 208
    (*uniscribe_font).dwrite_cache = core::ptr::null_mut ();		// C 209
    if (*uniscribe_font).w32_font.font.driver
	 == (&raw const harfbuzz_font_driver) as *const c_void		// C 210
      && !(*uniscribe_font).cache.is_null ()				// C 211
      {
	(fn_hb_font_destroy.unwrap ()) ((*uniscribe_font).cache as *mut hb_font_t);	// C 212
      }
  }
  /* C 213's `else` belongs to the `#ifdef` arm above; C 215-216's statement is what both arms
     reach (the non-harfbuzz arm reaches it alone).  */
  if ! (*uniscribe_font).cache.is_null ()				// C 215
    {
      (pfnScriptFreeCache.unwrap ()) ((&raw mut (*uniscribe_font).cache) as *mut SCRIPT_CACHE);	// C 216
    }

  (*uniscribe_font).cache = core::ptr::null_mut ();			// C 218

  w32font_close (font);							// C 220
}

/* C 223-243: the design note on `uniscribe_otf_capability` ("Return a list describing which
   scripts/languages FONT supports by which GSUB/GPOS features of OpenType tables. …"), carried in
   C's own words where it stands.  */
/* C 244-272: `uniscribe_otf_capability`.  */
/// C: static Lisp_Object uniscribe_otf_capability (struct font *font)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn uniscribe_otf_capability (font: *mut font) -> Lisp_Object
{
  let mut context: HDC;							// C 247
  let mut old_font: HFONT;						// C 248
  let f: *mut frame;							// C 249
  let mut capability: Lisp_Object = Fcons (Qnil (), Qnil ());		// C 250
  let mut features: Lisp_Object;					// C 251

  f = XFRAME (selected_frame);						// C 253
  /* Prevent quitting while we cons the lists in otf_features.
     That's because get_frame_dc acquires the critical section, so we
     cannot quit before we release it in release_frame_dc.  */
  let prev_quit: Lisp_Object = Vinhibit_quit;				// C 257
  Vinhibit_quit = Qt ();						// C 258
  context = get_frame_dc (f);						// C 259
  old_font = SelectObject (context, FONT_HANDLE (font)) as HFONT;	// C 260

  features = otf_features (context, c"GSUB".as_ptr ());			// C 262
  XSETCAR (capability, features);					// C 263
  features = otf_features (context, c"GPOS".as_ptr ());			// C 264
  XSETCDR (capability, features);					// C 265

  SelectObject (context, old_font);					// C 267
  release_frame_dc (f, context);					// C 268
  Vinhibit_quit = prev_quit;						// C 269

  return capability;							// C 271
}

/* C 274-290: the `uniscribe_shape` design note ("Uniscribe implementation of shape for font
   backend.  Shape text in LGSTRING.  See the docstring of `composition-get-gstring' for the
   format of LGSTRING. …"), carried in C's own words where it stands.  */
/* C 291-603: `uniscribe_shape`.  C's five `alloca` requests are the port's stack pool (P-05,
   `sa_stack_alloc`, the `src/msdos.rs` 1746 reading); the `wchar_t` locals are MinGW's 16-bit
   `wchar_t`, which this file's SDK types spell `WCHAR` (the `src/sound.rs` 1649 reading).  */
/// C: static Lisp_Object uniscribe_shape (Lisp_Object lgstring, Lisp_Object direction)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn uniscribe_shape (mut lgstring: Lisp_Object, direction: Lisp_Object)
				      -> Lisp_Object
{
  let font: *mut font = CHECK_FONT_GET_OBJECT (LGSTRING_FONT (lgstring));	// C 294
  let uniscribe_font: *mut uniscribe_font_info				// C 295-296
    = font as *mut uniscribe_font_info;
  let mut nchars: EMACS_UINT = 0;					// C 297
  let mut nitems: c_int = 0;						// C 298
  let mut max_items: c_int = 0;
  let mut i: c_int = 0;
  let mut max_glyphs: c_int = 0;
  let mut done_glyphs: c_int = 0;
  /// C: `wchar_t *chars;` - MinGW's `wchar_t` is the 16-bit unsigned `WCHAR`, which the port
  /// spells `c_ushort` (the `src/sound.rs` 1649 reading; docs/ports/token-aliases.tsv's
  /// `wchar_t` row); C's implicit conversions at C 326/C 507 stay as the casts below.
  let mut chars: *mut c_ushort = core::ptr::null_mut ();		// C 299
  let mut glyphs: *mut WORD = core::ptr::null_mut ();			// C 300
  let mut clusters: *mut WORD = core::ptr::null_mut ();
  let mut items: *mut SCRIPT_ITEM = core::ptr::null_mut ();		// C 301
  let mut attributes: *mut SCRIPT_VISATTR = core::ptr::null_mut ();	// C 302
  let mut advances: *mut c_int = core::ptr::null_mut ();		// C 303
  let mut offsets: *mut GOFFSET = core::ptr::null_mut ();		// C 304
  let mut overall_metrics: ABC = core::mem::zeroed ();			// C 305
  let mut result: HRESULT = 0;						// C 306
  let mut f: *mut frame = core::ptr::null_mut ();			// C 307
  let mut context: HDC = core::ptr::null_mut ();			// C 308
  let mut old_font: HFONT = core::ptr::null_mut ();			// C 309

  /* Get the chars from lgstring in a form we can use with uniscribe.  */
  /* C 312 is one chained assignment (`max_glyphs = nchars = …`); Rust's assignment yields `()`,
     so the two stores are two statements (Deviation 10).  */
  nchars = LGSTRING_GLYPH_LEN (lgstring) as EMACS_UINT;			// C 312
  max_glyphs = nchars as c_int;						// C 312
  done_glyphs = 0;							// C 313
  chars = sa_stack_alloc (nchars * core::mem::size_of::<c_ushort> ()) as *mut c_ushort;	// C 314
  /* FIXME: This loop assumes that characters in the input LGSTRING
     are all inside the BMP.  Need to encode characters beyond the BMP
     as UTF-16.  */
  i = 0;								// C 318
  while (i as EMACS_UINT) < nchars
    {
      /* lgstring can be bigger than the number of characters in it, in
	 the case where more glyphs are required to display those characters.
         If that is the case, note the real number of characters.  */
      if NILP (LGSTRING_GLYPH (lgstring, i as ptrdiff_t))		// C 323
	{ nchars = i as EMACS_UINT; }					// C 324
      else
	{
	  *chars.offset (i as isize)
	    = LGLYPH_CHAR (LGSTRING_GLYPH (lgstring, i as ptrdiff_t)) as c_ushort;	// C 326
	}
      i += 1;
    }

  /* First we need to break up the glyph string into runs of glyphs that
     can be treated together.  First try a single run.  */
  max_items = 2;							// C 331
  items = xmalloc (core::mem::size_of::<SCRIPT_ITEM> () * max_items as usize + 1)	// C 332
	  as *mut SCRIPT_ITEM;

  /* C 334-340: C's `while ((result = (*pfnScriptItemize) (…)) == E_OUTOFMEMORY)`; Rust has no
     assignment in a condition, so the call and its test are the loop head and the body is C's
     (Deviation 11).  C's `+ 1` outside the multiply (C 332/C 339) is kept character for
     character.  */
  loop
    {
      result = (pfnScriptItemize.unwrap ()) (chars, nchars as c_int, max_items,	// C 334
					     core::ptr::null (), core::ptr::null (),
					     items, &raw mut nitems);
      if result != E_OUTOFMEMORY
	{ break; }
      /* If that wasn't enough, keep trying with one more run.  */
      max_items += 1;							// C 338
      items = xrealloc (items as *mut c_void,				// C 339
			core::mem::size_of::<SCRIPT_ITEM> () * max_items as usize + 1)
	      as *mut SCRIPT_ITEM;
    }

  if FAILED (result)							// C 342
    {
      xfree (items as *mut c_void);					// C 344
      return Qnil ();							// C 345
    }

  glyphs = sa_stack_alloc (max_glyphs as usize * core::mem::size_of::<WORD> ())	// C 348
	   as *mut WORD;
  clusters = sa_stack_alloc (nchars * core::mem::size_of::<WORD> ()) as *mut WORD;	// C 349
  attributes = sa_stack_alloc (max_glyphs as usize			// C 350
			       * core::mem::size_of::<SCRIPT_VISATTR> ())
	       as *mut SCRIPT_VISATTR;
  advances = sa_stack_alloc (max_glyphs as usize * core::mem::size_of::<c_int> ())	// C 351
	     as *mut c_int;
  offsets = sa_stack_alloc (max_glyphs as usize * core::mem::size_of::<GOFFSET> ())	// C 352
	    as *mut GOFFSET;

  i = 0;								// C 354
  while i < nitems
    {
      let mut nglyphs: c_int;						// C 356
      let mut nchars_in_run: c_int;
      nchars_in_run = (*items.offset ((i + 1) as isize)).iCharPos	// C 357
	- (*items.offset (i as isize)).iCharPos;
      /* Force (*pfnScriptShape) to generate glyphs in the same order as
	 they are in the input LGSTRING, which is in the logical
	 order.  */
      script_analysis_bits! (fLogicalOrder, (*items.offset (i as isize)).a, true);	// C 361

      /* Context may be NULL here, in which case the cache should be
         used without needing to select the font.  */
      result								// C 365-369
	= (pfnScriptShape.unwrap ()) (context, (&raw mut (*uniscribe_font).cache) as *mut SCRIPT_CACHE,
				      chars.offset ((*items.offset (i as isize)).iCharPos as isize),
				      nchars_in_run, max_glyphs - done_glyphs,
				      &raw mut (*items.offset (i as isize)).a,
				      glyphs, clusters, attributes, &raw mut nglyphs);

      if result == E_PENDING && context.is_null ()			// C 371
	{
	  /* This assumes the selected frame is on the same display as the
	     one we are drawing.  It would be better for the frame to be
	     passed in.  */
	  f = XFRAME (selected_frame);					// C 376
	  context = get_frame_dc (f);					// C 377
	  old_font = SelectObject (context, FONT_HANDLE (font)) as HFONT;	// C 378

	  result							// C 380-385
	    = (pfnScriptShape.unwrap ()) (context, (&raw mut (*uniscribe_font).cache) as *mut SCRIPT_CACHE,
					  chars.offset ((*items.offset (i as isize)).iCharPos
							as isize),
					  nchars_in_run, max_glyphs - done_glyphs,
					  &raw mut (*items.offset (i as isize)).a,
					  glyphs, clusters, attributes, &raw mut nglyphs);
	}

      if result == E_OUTOFMEMORY						// C 388
	{
	  /* Need a bigger lgstring.  */
	  lgstring = Qnil ();						// C 391
	  break;							// C 392
	}
      else if FAILED (result)						// C 394
	{
	  /* Can't shape this run - return results so far if any.  */
	  break;							// C 397
	}
      else if script_analysis_bits! (fNoGlyphIndex, (*items.offset (i as isize)).a)	// C 399
	{
	  /* Glyph indices not supported by this font (or OS), means we
	     can't really do any meaningful shaping.  */
	  break;							// C 403
	}
      else
	{
	  result							// C 407-411
	    = (pfnScriptPlace.unwrap ()) (context, (&raw mut (*uniscribe_font).cache) as *mut SCRIPT_CACHE,
					  glyphs, nglyphs, attributes,
					  &raw mut (*items.offset (i as isize)).a,
					  advances, offsets, &raw mut overall_metrics);
	  if result == E_PENDING && context.is_null ()			// C 412
	    {
	      /* Cache not complete...  */
	      f = XFRAME (selected_frame);				// C 415
	      context = get_frame_dc (f);				// C 416
	      old_font = SelectObject (context, FONT_HANDLE (font)) as HFONT;	// C 417

	      result							// C 419-423
		= (pfnScriptPlace.unwrap ()) (context, (&raw mut (*uniscribe_font).cache) as *mut SCRIPT_CACHE,
					      glyphs, nglyphs, attributes,
					      &raw mut (*items.offset (i as isize)).a,
					      advances, offsets, &raw mut overall_metrics);
	    }
          if SUCCEEDED (result)						// C 425
	    {
	      let mut j: c_int;						// C 427
	      let mut from: c_int;
	      let mut to: c_int;
	      let mut adj_offset: c_int = 0;
	      let mut cluster_offset: c_int = 0;			// C 428

	      from = 0;							// C 430
	      to = from;						// C 431

	      j = 0;							// C 433
	      while j < nglyphs
		{
		  let lglyph_index: c_int = j + done_glyphs;		// C 435
		  let mut lglyph: Lisp_Object				// C 436
		    = LGSTRING_GLYPH (lgstring, lglyph_index as ptrdiff_t);
		  let mut char_metric: ABC = core::mem::zeroed ();	// C 437
		  let gl: c_uint;					// C 438

		  if NILP (lglyph)					// C 440
		    {
		      lglyph = LGLYPH_NEW ();				// C 442
		      LGSTRING_SET_GLYPH (lgstring, lglyph_index as ptrdiff_t, lglyph);	// C 443
		    }
		  /* Copy to a 32-bit data type to shut up the
		     compiler warning in LGLYPH_SET_CODE about
		     comparison being always false.  */
		  gl = *glyphs.offset (j as isize) as c_uint;		// C 448
		  LGLYPH_SET_CODE (lglyph, gl);				// C 449

		  /* Detect clusters, for linking codes back to
		     characters.  */
		  if script_visattr_bits! (fClusterStart, *attributes.offset (j as isize))	// C 453
		    {
		      while from < nchars_in_run
			    && ((*clusters.offset (from as isize)) as c_int) < j	// C 455
			{ from += 1; }
		      if from >= nchars_in_run				// C 457
			{
			  to = nchars_in_run - 1;			// C 458
			  from = to;
			}
		      else
			{
			  let mut k: c_int;				// C 461
			  to = nchars_in_run - 1;			// C 462
			  k = from + 1;					// C 463
			  while k < nchars_in_run
			    {
			      if ((*clusters.offset (k as isize)) as c_int) > j	// C 465
				{
				  to = k - 1;				// C 467
				  break;				// C 468
				}
			      k += 1;
			    }
			}
		      cluster_offset = 0;				// C 472

		      /* For RTL text, the Uniscribe shaper prepares
			 the values in ADVANCES array for layout in
			 reverse order, whereby "advance width" is
			 applied to move the pen in reverse direction
			 and _before_ drawing the glyph.  Since we
			 draw glyphs in their normal left-to-right
			 order, we need to adjust the coordinates of
			 each non-base glyph in a grapheme cluster via
			 X-OFF component of the gstring's ADJUSTMENT
			 sub-vector.  This loop computes, for each
			 grapheme cluster, the initial value of the
			 adjustment for the base character, which is
			 then updated for each successive glyph in the
			 grapheme cluster.  */
		      /* FIXME: Should we use DIRECTION here instead
			 of what (*pfnScriptItemize) guessed?  */
		      if script_analysis_bits! (fRTL, (*items.offset (i as isize)).a)	// C 490
			{
			  let mut j1: c_int = j;			// C 492

			  adj_offset = 0;				// C 494
			  while j1 < nglyphs
				&& ! script_visattr_bits! (fClusterStart,
							   *attributes.offset (j1 as isize))	// C 495
			    {
			      adj_offset += *advances.offset (j1 as isize);	// C 497
			      j1 += 1;
			    }
			}
		    }

		  let mut char_idx: c_int				// C 503
		    = (*items.offset (i as isize)).iCharPos + from + cluster_offset;
		  if from + cluster_offset > to				// C 504
		    { char_idx = (*items.offset (i as isize)).iCharPos + to; }	// C 505
		  cluster_offset += 1;					// C 506
		  LGLYPH_SET_CHAR (lglyph, *chars.offset (char_idx as isize) as EMACS_INT);	// C 507
		  LGLYPH_SET_FROM (lglyph,
				   ((*items.offset (i as isize)).iCharPos + from) as EMACS_INT);	// C 508
		  LGLYPH_SET_TO (lglyph,
				 ((*items.offset (i as isize)).iCharPos + to) as EMACS_INT);	// C 509

		  /* Metrics.  */
		  LGLYPH_SET_WIDTH (lglyph, *advances.offset (j as isize) as EMACS_INT);	// C 512
		  LGLYPH_SET_ASCENT (lglyph, (*font).ascent as EMACS_INT);	// C 513
		  LGLYPH_SET_DESCENT (lglyph, (*font).descent as EMACS_INT);	// C 514

		  result						// C 516-518
		    = (pfnScriptGetGlyphABCWidth.unwrap ())
		      (context, (&raw mut (*uniscribe_font).cache) as *mut SCRIPT_CACHE,
		       *glyphs.offset (j as isize), &raw mut char_metric);
		  if result == E_PENDING && context.is_null ()		// C 519
		    {
		      /* Cache incomplete... */
		      f = XFRAME (selected_frame);			// C 522
		      context = get_frame_dc (f);			// C 523
		      old_font = SelectObject (context, FONT_HANDLE (font)) as HFONT;	// C 524
		      result						// C 525-527
			= (pfnScriptGetGlyphABCWidth.unwrap ())
			  (context, (&raw mut (*uniscribe_font).cache) as *mut SCRIPT_CACHE,
			   *glyphs.offset (j as isize), &raw mut char_metric);
		    }

		  if SUCCEEDED (result)					// C 530
		    {
		      let lbearing: c_int = char_metric.abcA;		// C 532
		      /* C 533 adds C's `UINT` to a signed `int`: the sum is computed unsigned (as
			 C's usual arithmetic conversions do) and converted back.  */
		      let rbearing: c_int				// C 533
			= (char_metric.abcA as c_uint).wrapping_add (char_metric.abcB) as c_int;

		      LGLYPH_SET_LBEARING (lglyph, lbearing as EMACS_INT);	// C 535
		      LGLYPH_SET_RBEARING (lglyph, rbearing as EMACS_INT);	// C 536
		    }
		  else
		    {
		      LGLYPH_SET_LBEARING (lglyph, 0);			// C 540
		      LGLYPH_SET_RBEARING (lglyph,
					   *advances.offset (j as isize) as EMACS_INT);	// C 541
		    }

		  if (*offsets.offset (j as isize)).du != 0			// C 544
		      || (*offsets.offset (j as isize)).dv != 0
		      /* For non-base glyphs of RTL grapheme clusters,
			 adjust the X offset even if both DU and DV
			 are zero.  */
		      || (! script_visattr_bits! (fClusterStart, *attributes.offset (j as isize))
			  && script_analysis_bits! (fRTL, (*items.offset (i as isize)).a))	// C 548
		    {
		      let vec: Lisp_Object = make_uninit_vector (3);	// C 550

		      if script_analysis_bits! (fRTL, (*items.offset (i as isize)).a)	// C 552
			{
			  /* Empirically, it looks like Uniscribe
			     interprets DU in reverse direction for
			     RTL clusters.  E.g., if we don't reverse
			     the direction, the Hebrew point HOLAM is
			     drawn above the right edge of the base
			     consonant, instead of above the left edge.  */
			  ASET (vec, 0, make_fixnum ((-(*offsets.offset (j as isize)).du	// C 560
						      + adj_offset) as EMACS_INT));
			  /* Update the adjustment value for the width
			     advance of the glyph we just emitted.  */
			  adj_offset -= 2 * *advances.offset (j as isize);	// C 564
			}
		      else
			{
			  ASET (vec, 0, make_fixnum (((*offsets.offset (j as isize)).du	// C 567
						      + adj_offset) as EMACS_INT));
			}
		      /* In the font definition coordinate system, the
			 Y coordinate points up, while in our screen
			 coordinates Y grows downwards.  So we need to
			 reverse the sign of Y-OFFSET here.  */
		      ASET (vec, 1, make_fixnum ((-(*offsets.offset (j as isize)).dv)	// C 572
						 as EMACS_INT));
		      /* Based on what ftfont.c does... */
		      ASET (vec, 2, make_fixnum (*advances.offset (j as isize) as EMACS_INT));	// C 574
		      LGLYPH_SET_ADJUSTMENT (lglyph, vec);		// C 575
		    }
		  else
		    {
		      LGLYPH_SET_ADJUSTMENT (lglyph, Qnil ());		// C 579
		      /* Update the adjustment value to compensate for
			 the width of the base character.  */
		      if script_analysis_bits! (fRTL, (*items.offset (i as isize)).a)	// C 582
			{ adj_offset -= *advances.offset (j as isize); }	// C 583
		    }
		  j += 1;
		}
	    }
	}
      done_glyphs += nglyphs;						// C 588
      i += 1;
    }

  xfree (items as *mut c_void);						// C 591

  if !context.is_null ()						// C 593
    {
      SelectObject (context, old_font);					// C 595
      release_frame_dc (f, context);					// C 596
    }

  if NILP (lgstring)							// C 599
    { return Qnil (); }							// C 600
  else
    { return make_fixnum (done_glyphs as EMACS_INT); }			// C 602
}

/* C 605-607: "Uniscribe implementation of encode_char for font backend.  Return a glyph code of
   FONT for character C (Unicode code point).  If FONT doesn't have such a glyph, return
   FONT_INVALID_CODE."  */
/* C 608-708: `uniscribe_encode_char`.  C 643's `alloca` is the stack pool (P-05) and C 615's
   `wchar_t ch[2]` is MinGW's 16-bit `wchar_t` spelled `WCHAR` (the file note's reading).  */
/// C: static unsigned uniscribe_encode_char (struct font *font, int c)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn uniscribe_encode_char (font: *mut font, c: c_int) -> c_uint
{
  let mut context: HDC = core::ptr::null_mut ();			// C 611
  let mut f: *mut frame = core::ptr::null_mut ();			// C 612
  let mut old_font: HFONT = core::ptr::null_mut ();			// C 613
  let mut code: c_uint = FONT_INVALID_CODE;				// C 614
  /// C: `wchar_t ch[2];` - the same 16-bit `wchar_t` as C 299's `chars` (see the note there).
  let mut ch: [c_ushort; 2] = [0; 2];					// C 615
  let mut len: c_int;							// C 616
  let mut items: *mut SCRIPT_ITEM;					// C 617
  let mut nitems: c_int;						// C 618
  let uniscribe_font: *mut uniscribe_font_info				// C 619-620
    = font as *mut uniscribe_font_info;

  if c < 0x10000							// C 622
    {
      ch[0] = c as c_ushort;						// C 624
      len = 1;								// C 625
    }
  else
    {
      let surrogate: DWORD = (c - 0x10000) as DWORD;			// C 629

      /* High surrogate: U+D800 - U+DBFF.  */
      ch[0] = (0xD800 + ((surrogate >> 10) & 0x03FF)) as c_ushort;	// C 632
      /* Low surrogate: U+DC00 - U+DFFF.  */
      ch[1] = (0xDC00 + (surrogate & 0x03FF)) as c_ushort;		// C 634
      len = 2;								// C 635
    }

  /* Non BMP characters must be handled by the uniscribe shaping
     engine as GDI functions (except blindly displaying lines of
     Unicode text) and the promising looking ScriptGetCMap do not
     convert surrogate pairs to glyph indexes correctly.  */
    {
      items = sa_stack_alloc (core::mem::size_of::<SCRIPT_ITEM> () * 2 + 1)	// C 643
	      as *mut SCRIPT_ITEM;
      if SUCCEEDED ((pfnScriptItemize.unwrap ()) (ch.as_ptr (), len, 2,	// C 644
						  core::ptr::null (),
						  core::ptr::null (), items,
						  &raw mut nitems))
	{
	  let mut result: HRESULT;					// C 647
          /* Surrogates seem to need 2 here, even though only one glyph is
	     returned.  Indic characters can also produce 2 or more glyphs for
	     a single code point, but they need to use uniscribe_shape
	     above for correct display.  */
          let mut glyphs: [WORD; 2] = [0; 2];				// C 652
	  let mut clusters: [WORD; 2] = [0; 2];
          let mut attrs: [SCRIPT_VISATTR; 2]				// C 653
	    = [SCRIPT_VISATTR { bits: 0 }, SCRIPT_VISATTR { bits: 0 }];
          let mut nglyphs: c_int;					// C 654

	  /* Force (*pfnScriptShape) to generate glyphs in the logical
	     order.  */
	  script_analysis_bits! (fLogicalOrder, (*items).a, true);	// C 658

          result = (pfnScriptShape.unwrap ()) (context,			// C 660-663
					      (&raw mut (*uniscribe_font).cache) as *mut SCRIPT_CACHE,
					      ch.as_ptr (), len, 2, &raw mut (*items).a,
					      glyphs.as_mut_ptr (), clusters.as_mut_ptr (),
					      attrs.as_mut_ptr (), &raw mut nglyphs);

          if result == E_PENDING					// C 665
            {
              /* Use selected frame until API is updated to pass
                 the frame.  */
              f = XFRAME (selected_frame);				// C 669
              context = get_frame_dc (f);				// C 670
              old_font = SelectObject (context, FONT_HANDLE (font)) as HFONT;	// C 671
              result							// C 672-676
		= (pfnScriptShape.unwrap ()) (context,
					     (&raw mut (*uniscribe_font).cache) as *mut SCRIPT_CACHE,
					     ch.as_ptr (), len, 2, &raw mut (*items).a,
					     glyphs.as_mut_ptr (), clusters.as_mut_ptr (),
					     attrs.as_mut_ptr (), &raw mut nglyphs);
            }

          if SUCCEEDED (result) && nglyphs == 1			// C 679
            {
	      /* Some fonts return .notdef glyphs instead of failing.
	         (TrueType spec reserves glyph code 0 for .notdef)  */
	      if glyphs[0] != 0						// C 683
		{ code = glyphs[0] as c_uint; }				// C 684
            }
          else if SUCCEEDED (result) || result == E_OUTOFMEMORY		// C 686
            {
              /* This character produces zero or more than one glyph
                 when shaped. But we still need the return from here
                 to be valid for the shaping engine to be invoked
                 later.  */
              result							// C 692-695
		= (pfnScriptGetCMap.unwrap ()) (context,
					       (&raw mut (*uniscribe_font).cache) as *mut SCRIPT_CACHE,
					       ch.as_ptr (), len, 0, glyphs.as_mut_ptr ());
              if SUCCEEDED (result) && glyphs[0] != 0			// C 696
                { code = glyphs[0] as c_uint; }				// C 697
            }
	}
    }
    if !context.is_null ()						// C 701
      {
	SelectObject (context, old_font);				// C 703
	release_frame_dc (f, context);					// C 704
      }

    return code;							// C 707
}

/* C 710-731: the note listing which of the driver's methods are shared with `w32font` and which
   are unused, carried in C's own words where it stands:

   / *
      Shared with w32font:
      Lisp_Object uniscribe_get_cache (Lisp_Object frame);
      void uniscribe_free_entity (Lisp_Object font_entity);
      int uniscribe_has_char (Lisp_Object entity, int c);
      void uniscribe_text_extents (struct font *font, unsigned *code,
                                   int nglyphs, struct font_metrics *metrics);
      int uniscribe_draw (struct glyph_string *s, int from, int to,
                          int x, int y, int with_background);

      Unused:
      int uniscribe_prepare_face (struct frame *f, struct face *face);
      void uniscribe_done_face (struct frame *f, struct face *face);
      int uniscribe_get_bitmap (struct font *font, unsigned code,
                                struct font_bitmap *bitmap, int bits_per_pixel);
      void uniscribe_free_bitmap (struct font *font, struct font_bitmap *bitmap);
      int uniscribe_anchor_point (struct font *font, unsigned code,
                                  int index, int *x, int *y);
      int uniscribe_start_for_frame (struct frame *f);
      int uniscribe_end_for_frame (struct frame *f);

   * /  */

/* C 733-768: `add_opentype_font_name_to_list`, the `EnumFontFamiliesEx` callback.  `CALLBACK` and
   `ALIGN_STACK` are `extern "system"` plus the attribute `include/w32term.rs` 37-39 records (the
   `src/w32font.rs` 1925 note).  */
/// C: static int CALLBACK ALIGN_STACK add_opentype_font_name_to_list (ENUMLOGFONTEX *logical_font,
/// C:   NEWTEXTMETRICEX *physical_font, DWORD font_type, LPARAM list_object)
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn add_opentype_font_name_to_list (logical_font: *mut ENUMLOGFONTEX,
							  physical_font: *mut NEWTEXTMETRICEX,
							  font_type: DWORD,
							  list_object: LPARAM) -> c_int
{
  let list: *mut Lisp_Object = list_object as *mut Lisp_Object;		// C 742
  let family: Lisp_Object;						// C 743

  /* Skip vertical fonts (intended only for printing)  */
  if (*logical_font).elfLogFont.lfFaceName[0] == b'@' as c_char		// C 746
    { return 1; }							// C 747

  /* Skip non opentype fonts.  Count old truetype fonts as opentype,
     as some of them do contain GPOS and GSUB data that Uniscribe
     can make use of.  */
  if ((*physical_font).ntmTm.ntmFlags & NTMFLAGS_OPENTYPE) == 0		// C 752
      && font_type != TRUETYPE_FONTTYPE
    { return 1; }							// C 754

  /* Skip fonts that have no Unicode coverage.  */
  if (*physical_font).ntmFontSig.fsUsb[3] == 0				// C 757
      && (*physical_font).ntmFontSig.fsUsb[2] == 0			// C 758
      && (*physical_font).ntmFontSig.fsUsb[1] == 0			// C 759
      && ((*physical_font).ntmFontSig.fsUsb[0] & 0x3fffffff) == 0	// C 760
    { return 1; }							// C 761

  family = intern_font_name ((*logical_font).elfLogFont.lfFaceName.as_mut_ptr ());	// C 763
  if NILP (memq_no_quit (family, *list))				// C 764
    { *list = Fcons (family, *list); }					// C 765

  return 1;								// C 767
}

/* C 770-775: ":otf property handling.  Since the necessary Uniscribe APIs for getting font tag
   information are only available in Vista, we may need to parse the font data directly according
   to the OpenType Specification."  */
/* C 776-802: the four `:otf` byte-reading macros.  C's `do { … } while (0)` bodies are block
   expressions and C's `goto font_table_error` is the labelled block both callers put around their
   bodies (Deviation 6), so the label comes in as a parameter of the `break`; C's `STR[3]` on a
   `char *` is the port's `offset (3)` and the `memcpy` is `include/string.rs`'s (PORTING.md 3.3).  */
/// C: #define OTF_TAG(STR) ((STR[3] << 24) | (STR[2] << 16) | (STR[1] << 8) | STR[0])
macro_rules! OTF_TAG {
  ($str:expr) => {
    (((*($str).offset (3) as c_int) << 24) | ((*($str).offset (2) as c_int) << 16)
     | ((*($str).offset (1) as c_int) << 8) | (*($str).offset (0) as c_int))
  };
}
/// C: #define OTF_INT16_VAL(TABLE, OFFSET, PTR) do { BYTE temp, data[2];
/// C:   if (GetFontData (context, TABLE, OFFSET, data, 2) != 2) goto font_table_error;
/// C:   temp = data[0], data[0] = data[1], data[1] = temp; memcpy (PTR, data, 2); } while (0)
macro_rules! OTF_INT16_VAL {
  ($lbl:lifetime, $table:expr, $offset:expr, $ptr:expr) => {
    {
      let mut temp: BYTE;
      let mut data: [BYTE; 2] = [0; 2];
      if GetFontData (context, $table, $offset as DWORD, data.as_mut_ptr () as LPVOID, 2) != 2
	{ break $lbl; }
      temp = data[0];
      data[0] = data[1];
      data[1] = temp;
      memcpy ($ptr as *mut c_void, data.as_ptr () as *const c_void, 2);
    }
  };
}
/// C: #define OTF_DWORDTAG_VAL(TABLE, OFFSET, PTR) do {
/// C:   if (GetFontData (context, TABLE, OFFSET, PTR, 4) != 4) goto font_table_error; } while (0)
/// ("Do not reverse the bytes, because we will compare with a OTF_TAG value that has them
/// reversed already.")
macro_rules! OTF_DWORDTAG_VAL {
  ($lbl:lifetime, $table:expr, $offset:expr, $ptr:expr) => {
    {
      if GetFontData (context, $table, $offset as DWORD, $ptr as LPVOID, 4) != 4
	{ break $lbl; }
    }
  };
}
/// C: #define OTF_TAG_VAL(TABLE, OFFSET, STR) do {
/// C:   if (GetFontData (context, TABLE, OFFSET, STR, 4) != 4) goto font_table_error;
/// C:   STR[4] = '\0'; } while (0)
macro_rules! OTF_TAG_VAL {
  ($lbl:lifetime, $table:expr, $offset:expr, $str:expr) => {
    {
      if GetFontData (context, $table, $offset as DWORD, $str as LPVOID, 4) != 4
	{ break $lbl; }
      *($str).offset (4) = 0;
    }
  };
}
/// C: #define SNAME(VAL) SSDATA (SYMBOL_NAME (VAL))
macro_rules! SNAME { ($val:expr) => { SSDATA (SYMBOL_NAME ($val)) }; }

/* C 806-820: the three Vista entry points' typedefs, their three `static` pointers (all NULL until
   C 1579-1584 fills them) and the flag C 1009 tests.  */
/// C: typedef HRESULT (WINAPI *ScriptGetFontScriptTags_Proc)
/// C:   (HDC, SCRIPT_CACHE *, SCRIPT_ANALYSIS *, int, OPENTYPE_TAG *, int *);
/// C: typedef HRESULT (WINAPI *ScriptGetFontLanguageTags_Proc)
/// C:   (HDC, SCRIPT_CACHE *, SCRIPT_ANALYSIS *, OPENTYPE_TAG, int, OPENTYPE_TAG *, int *);
/// C: typedef HRESULT (WINAPI *ScriptGetFontFeatureTags_Proc)
/// C:   (HDC, SCRIPT_CACHE *, SCRIPT_ANALYSIS *, OPENTYPE_TAG, OPENTYPE_TAG, int, OPENTYPE_TAG *,
/// C:    int *);
#[cfg (feature = "have_ntgui")]
pub type ScriptGetFontScriptTags_Proc = Option<unsafe extern "system" fn (HDC, *mut SCRIPT_CACHE,
                                                                          *mut SCRIPT_ANALYSIS,
                                                                          c_int, *mut OPENTYPE_TAG,
                                                                          *mut c_int) -> HRESULT>;
#[cfg (feature = "have_ntgui")]
pub type ScriptGetFontLanguageTags_Proc = Option<unsafe extern "system" fn (HDC, *mut SCRIPT_CACHE,
                                                                            *mut SCRIPT_ANALYSIS,
                                                                            OPENTYPE_TAG, c_int,
                                                                            *mut OPENTYPE_TAG,
                                                                            *mut c_int)
                                                                           -> HRESULT>;
#[cfg (feature = "have_ntgui")]
pub type ScriptGetFontFeatureTags_Proc = Option<unsafe extern "system" fn (HDC, *mut SCRIPT_CACHE,
                                                                           *mut SCRIPT_ANALYSIS,
                                                                           OPENTYPE_TAG,
                                                                           OPENTYPE_TAG, c_int,
                                                                           *mut OPENTYPE_TAG,
                                                                           *mut c_int)
                                                                          -> HRESULT>;
/// C: static ScriptGetFontScriptTags_Proc script_get_font_scripts_fn;
/// C: static ScriptGetFontLanguageTags_Proc script_get_font_languages_fn;
/// C: static ScriptGetFontFeatureTags_Proc script_get_font_features_fn;
/// C: static bool uniscribe_new_apis;
#[cfg (feature = "have_ntgui")]
static mut script_get_font_scripts_fn: ScriptGetFontScriptTags_Proc = None;
#[cfg (feature = "have_ntgui")]
static mut script_get_font_languages_fn: ScriptGetFontLanguageTags_Proc = None;
#[cfg (feature = "have_ntgui")]
static mut script_get_font_features_fn: ScriptGetFontFeatureTags_Proc = None;
#[cfg (feature = "have_ntgui")]
static mut uniscribe_new_apis: bool = false;

/* C 822-826: "Verify that all the required features in FEATURES, each of whose elements is a list
   or nil, can be found among the N feature tags in FTAGS.  …"  */
/* C 827-871: `uniscribe_check_features` (C's `Lisp_Object features[2]` parameter decays to a
   pointer, which is what the port spells).  */
/// C: static bool uniscribe_check_features (Lisp_Object features[2], OPENTYPE_TAG *ftags, int n)
#[cfg (feature = "have_ntgui")]
unsafe fn uniscribe_check_features (features: *mut Lisp_Object, ftags: *mut OPENTYPE_TAG, n: c_int)
				    -> bool
{
  let mut j: c_int;							// C 830

  j = 0;								// C 832
  while j < 2
    {
      let mut negative: bool = false;					// C 834
      let mut rest: Lisp_Object;					// C 835

      rest = *features.offset (j as isize);				// C 837
      while CONSP (rest)
	{
	  let feature: Lisp_Object = XCAR (rest);			// C 839

	  /* The font must NOT have any of the features after nil.
	     See the doc string of 'font-spec', under ':otf'.  */
	  if NILP (feature)						// C 843
	    { negative = true; }					// C 844
	  else
	    {
	      let feature_tag: OPENTYPE_TAG				// C 847
		= OTF_TAG! (SNAME! (feature)) as OPENTYPE_TAG;
	      let mut i: c_int;						// C 848

	      i = 0;							// C 850
	      while i < n
		{
		  if *ftags.offset (i as isize) == feature_tag		// C 852
		    {
		      /* Test fails if we find a feature that the font
			 must NOT have.  */
		      if negative					// C 856
			{ return false; }				// C 857
		      break;						// C 858
		    }
		  i += 1;
		}

	      /* Test fails if we do NOT find a feature that the font
		 should have.  */
	      if i >= n && !negative					// C 864
		{ return false; }					// C 865
	    }
	  rest = XCDR (rest);						// C 837
	}
      j += 1;
    }

  return true;								// C 870
}

/* C 873-878: "Check if font supports the required OTF script/language/features using the Unsicribe
   APIs available since Windows Vista.  We prefer these APIs as a kind of future-proofing
   Emacs: …"  */
/* C 879-967: `uniscribe_check_otf_1`.  C's three `goto no_support` jumps become `break
   'no_support` out of the labelled block that carries C 889-961; the cleanup C 963-966 stands
   after it, which is where C's label is (Deviation 6).  */
/// C: static int uniscribe_check_otf_1 (HDC context, Lisp_Object script, Lisp_Object lang,
/// C:   Lisp_Object features[2], int *retval)
#[cfg (feature = "have_ntgui")]
unsafe fn uniscribe_check_otf_1 (context: HDC, script: Lisp_Object, lang: Lisp_Object,
				 features: *mut Lisp_Object, retval: *mut c_int) -> c_int
{
  let mut cache: SCRIPT_CACHE = core::ptr::null_mut ();			// C 883
  let mut tags: [OPENTYPE_TAG; 128] = [0; 128];				// C 884
  let mut script_tag: OPENTYPE_TAG = 0;
  let mut lang_tag: OPENTYPE_TAG = 0;
  let max_tags: c_int = countof! (tags) as c_int;			// C 885
  let mut ntags: c_int = 0;						// C 886
  let mut i: c_int = 0;
  let mut ret: c_int = 0;
  let mut rslt: HRESULT;						// C 887

  *retval = 0;								// C 889

  'no_support: {							// C 963's label
  rslt = (script_get_font_scripts_fn.unwrap ()) (context, &raw mut cache,	// C 891-892
						 core::ptr::null_mut (), max_tags,
						 tags.as_mut_ptr (), &raw mut ntags);
  if FAILED (rslt)							// C 893
    {
      DebPrint! (("ScriptGetFontScriptTags failed with 0x%x\n", rslt));	// C 895
      ret = -1;								// C 896
      break 'no_support;						// C 897
    }
  if NILP (script)							// C 899
    { script_tag = OTF_TAG! (c"DFLT".as_ptr ()) as OPENTYPE_TAG; }	// C 900
  else
    { script_tag = OTF_TAG! (SNAME! (script)) as OPENTYPE_TAG; }		// C 902
  i = 0;								// C 903
  while i < ntags
    {
      if tags[i as usize] == script_tag					// C 904
	{ break; }
      i += 1;
    }

  if i >= ntags								// C 907
    { break 'no_support; }						// C 908

  if NILP (lang)							// C 910
    { lang_tag = OTF_TAG! (c"dflt".as_ptr ()) as OPENTYPE_TAG; }		// C 911
  else
    {
      rslt = (script_get_font_languages_fn.unwrap ()) (context, &raw mut cache,	// C 914-915
						      core::ptr::null_mut (), script_tag,
						      max_tags, tags.as_mut_ptr (),
						      &raw mut ntags);
      if FAILED (rslt)							// C 916
	{
	  DebPrint! (("ScriptGetFontLanguageTags failed with 0x%x\n", rslt));	// C 918
	  ret = -1;							// C 919
	  break 'no_support;						// C 920
	}
      if ntags == 0							// C 922
	{ lang_tag = OTF_TAG! (c"dflt".as_ptr ()) as OPENTYPE_TAG; }	// C 923
      else
	{
	  lang_tag = OTF_TAG! (SNAME! (lang)) as OPENTYPE_TAG;		// C 926
	  i = 0;							// C 927
	  while i < ntags
	    {
	      if tags[i as usize] == lang_tag				// C 928
		{ break; }
	      i += 1;
	    }

	  if i >= ntags							// C 931
	    { break 'no_support; }					// C 932
	}
    }

  if ! NILP (*features.offset (0))					// C 936
    {
      /* Are the 2 feature lists valid?  */
      if ! CONSP (*features.offset (0))					// C 939
	  || (! NILP (*features.offset (1)) && ! CONSP (*features.offset (1)))	// C 940
	{ break 'no_support; }						// C 941
      rslt = (script_get_font_features_fn.unwrap ()) (context, &raw mut cache,	// C 942-944
						     core::ptr::null_mut (),
						     script_tag, lang_tag, max_tags,
						     tags.as_mut_ptr (), &raw mut ntags);
      if FAILED (rslt)							// C 945
	{
	  DebPrint! (("ScriptGetFontFeatureTags failed with 0x%x\n", rslt));	// C 947
	  ret = -1;							// C 948
	  break 'no_support;						// C 949
	}

      /* ScriptGetFontFeatureTags doesn't let us query features
	 separately for GSUB and GPOS, so we check them all together.
	 It doesn't really matter, since the features in GSUB and GPOS
	 are disjoint, i.e. no feature can appear in both tables.  */
      if ! uniscribe_check_features (features, tags.as_mut_ptr (), ntags)	// C 956
	{ break 'no_support; }						// C 957
    }

  ret = 1;								// C 960
  *retval = 1;								// C 961
  }

  /* C 963-966: `no_support:` and its cleanup.  */
  if !cache.is_null ()							// C 964
    { (pfnScriptFreeCache.unwrap ()) (&raw mut cache); }		// C 965
  return ret;								// C 966
}

/* C 969-971: "Check if font supports the otf script/language/features specified.  OTF_SPEC is in
   the format (script lang [(gsub_feature ...)|nil] [(gpos_feature ...)]?)".  */
/* C 972-1152: `uniscribe_check_otf` - the one non-`static` function of this file besides the two
   `syms_of_*` (so it carries `#[unsafe(no_mangle)]`, and `include/w32font.rs` 176 declares it in
   the glob-imported module C's own header mirror keeps).  C 1143-1145's three adjacent labels
   (`done`/`no_support`/`font_table_error`) all reach one cleanup, so the port has one labelled
   block and every `goto` is a `break 'font_table_error` (Deviation 6); the OTF macros take the
   label because their C bodies jump to it.  C 1060-1065 is `#if 0` - dead text in every
   configuration, carried as C's own words.  */
/// C: int uniscribe_check_otf (LOGFONT *font, Lisp_Object otf_spec)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uniscribe_check_otf (font: *mut LOGFONT, otf_spec: Lisp_Object) -> c_int
{
  let mut script: Lisp_Object;						// C 975
  let mut lang: Lisp_Object;
  let mut rest: Lisp_Object;
  let mut features: [Lisp_Object; 2] = [Qnil (); 2];			// C 976
  let mut feature_tables: [DWORD; 2] = [0; 2];				// C 977
  let mut script_tag: DWORD = 0;					// C 978
  let mut default_script: DWORD = 0;
  let mut lang_tag: DWORD = 0;
  let f: *mut frame;							// C 979
  let mut context: HDC = core::ptr::null_mut ();			// C 980
  let mut check_font: HFONT = core::ptr::null_mut ();			// C 981
  let mut old_font: HFONT = core::ptr::null_mut ();
  let mut i: c_int = 0;							// C 982
  let mut retval: c_int = 0;

  /* Check the spec is in the right format.  */
  if ! CONSP (otf_spec) || XFIXNUM (Flength (otf_spec)) < 3		// C 985
    { return 0; }							// C 986

  /* Break otf_spec into its components.  */
  script = XCAR (otf_spec);						// C 989
  rest = XCDR (otf_spec);						// C 990

  lang = XCAR (rest);							// C 992
  rest = XCDR (rest);							// C 993

  features[0] = XCAR (rest);						// C 995
  rest = XCDR (rest);							// C 996
  if NILP (rest)							// C 997
    { features[1] = Qnil (); }						// C 998
  else
    { features[1] = XCAR (rest); }					// C 1000

  /* Set up graphics context so we can use the font.  */
  f = XFRAME (selected_frame);						// C 1003
  context = get_frame_dc (f);						// C 1004
  check_font = CreateFontIndirect (font);				// C 1005
  old_font = SelectObject (context, check_font) as HFONT;		// C 1006

  'font_table_error: {							// C 1143-1145's three labels
  /* If we are on Vista or later, use the new APIs.  */
  if uniscribe_new_apis						// C 1009
      && ! w32_disable_new_uniscribe_apis				// C 1010
      && uniscribe_check_otf_1 (context, script, lang, features.as_mut_ptr (), &raw mut retval)
	 != -1								// C 1011
    { break 'font_table_error; }					// C 1012 (`goto done`)

  /* Set up tags we will use in the search.  */
  feature_tables[0] = OTF_TAG! (c"GSUB".as_ptr ()) as DWORD;		// C 1015
  feature_tables[1] = OTF_TAG! (c"GPOS".as_ptr ()) as DWORD;		// C 1016
  default_script = OTF_TAG! (c"DFLT".as_ptr ()) as DWORD;		// C 1017
  if NILP (script)							// C 1018
    { script_tag = default_script; }					// C 1019
  else
    { script_tag = OTF_TAG! (SNAME! (script)) as DWORD; }		// C 1021
  if ! NILP (lang)							// C 1022
    { lang_tag = OTF_TAG! (SNAME! (lang)) as DWORD; }			// C 1023

  /* Scan GSUB and GPOS tables.  */
  i = 0;								// C 1026
  while i < 2
    {
      let mut j: c_int;							// C 1028
      let mut n_match_features: c_int;
      let mut scriptlist_table: c_ushort = 0;				// C 1029
      let mut feature_table: c_ushort = 0;
      let mut n_scripts: c_ushort = 0;
      let mut script_table: c_ushort = 0;				// C 1030
      let mut langsys_table: c_ushort = 0;
      let mut n_langs: c_ushort = 0;
      let mut feature_index: c_ushort = 0;				// C 1031
      let mut n_features: c_ushort = 0;
      let tbl: DWORD = feature_tables[i as usize];			// C 1032
      let mut feature_id: DWORD = 0;					// C 1033
      let mut ftags: *mut DWORD = core::ptr::null_mut ();
      let mut farray: [Lisp_Object; 2] = [Qnil (); 2];			// C 1034

      /* Skip if no features requested from this table.  */
      if NILP (features[i as usize])					// C 1037
	{ i += 1; continue; }						// C 1038

      /* If features is not a cons, this font spec is messed up.  */
      if ! CONSP (features[i as usize])				// C 1041
	{ break 'font_table_error; }					// C 1042

      /* Read GPOS/GSUB header.  */
      OTF_INT16_VAL! ('font_table_error, tbl, 4, &raw mut scriptlist_table);	// C 1045
      OTF_INT16_VAL! ('font_table_error, tbl, 6, &raw mut feature_table);	// C 1046
      OTF_INT16_VAL! ('font_table_error, tbl, scriptlist_table as c_int,
		      &raw mut n_scripts);				// C 1047

      /* Find the appropriate script table.  */
      script_table = 0;							// C 1050
      j = 0;								// C 1051
      while j < n_scripts as c_int
	{
	  let mut script_id: DWORD;					// C 1053
	  OTF_DWORDTAG_VAL! ('font_table_error, tbl,
			     (scriptlist_table as c_int) + 2 + j * 6,
			     &raw mut script_id);			// C 1054
	  if script_id == script_tag					// C 1055
	    {
	      OTF_INT16_VAL! ('font_table_error, tbl,
			      (scriptlist_table as c_int) + 6 + j * 6,
			      &raw mut script_table);			// C 1057
	      break;							// C 1058
	    }
	  /* C 1060-1065: `#if 0` - "Causes false positives."  The dead arm is C's own text, with
	     C's comment delimiters written `/ *` and `* /` because Rust's block comments nest
	     (PORTING.md 0.3):

	       #if 0	  / * Causes false positives.  * /
			  / * If there is a DFLT script defined in the font, use it
			     if the specified script is not found.  * /
			  else if (script_id == default_script)
			    OTF_INT16_VAL (tbl, scriptlist_table + 6 + j * 6, &script_table);
	       #endif
	   */
	  j += 1;
	}
      /* If no specific or default script table was found, then this font
	 does not support the script.  */
      if script_table == 0						// C 1069
	{ break 'font_table_error; }					// C 1070

      /* Offset is from beginning of scriptlist_table.  */
      script_table = script_table.wrapping_add (scriptlist_table);	// C 1073

      /* Get default langsys table.  */
      OTF_INT16_VAL! ('font_table_error, tbl, script_table as c_int,
		      &raw mut langsys_table);				// C 1076

      /* If lang was specified, see if font contains a specific entry.  */
      if ! NILP (lang)							// C 1079
	{
	  OTF_INT16_VAL! ('font_table_error, tbl, (script_table as c_int) + 2,
			  &raw mut n_langs);				// C 1081

	  j = 0;							// C 1083
	  while j < n_langs as c_int
	    {
	      let mut lang_id: DWORD;					// C 1085
	      OTF_DWORDTAG_VAL! ('font_table_error, tbl,
				 (script_table as c_int) + 4 + j * 6,
				 &raw mut lang_id);			// C 1086
	      if lang_id == lang_tag					// C 1087
		{
		  OTF_INT16_VAL! ('font_table_error, tbl,
				  (script_table as c_int) + 8 + j * 6,
				  &raw mut langsys_table);		// C 1089
		  break;						// C 1090
		}
	      j += 1;
	    }
	}

      if langsys_table == 0						// C 1095
	{ break 'font_table_error; }					// C 1096

      /* Offset is from beginning of script table.  */
      langsys_table = langsys_table.wrapping_add (script_table);	// C 1099

      /* If there are no features to check, skip checking.  */
      if NILP (features[i as usize])					// C 1102
	{ i += 1; continue; }						// C 1103
      if ! CONSP (features[i as usize])				// C 1104
	{ break 'font_table_error; }					// C 1105

      n_match_features = 0;						// C 1107

      /* First get required feature (if any).  */
      OTF_INT16_VAL! ('font_table_error, tbl, (langsys_table as c_int) + 2,
		      &raw mut feature_index);				// C 1110
      if feature_index != 0xFFFF					// C 1111
	{ n_match_features = 1; }					// C 1112
      OTF_INT16_VAL! ('font_table_error, tbl, (langsys_table as c_int) + 4,
		      &raw mut n_features);				// C 1113
      n_match_features += n_features as c_int;				// C 1114
      USE_SAFE_ALLOCA! (sa_avail, sa_count);				// C 1115
      SAFE_NALLOCA! (sa_avail, ftags, 1, n_match_features);		// C 1116
      let mut k: c_int = 0;						// C 1117
      if feature_index != 0xFFFF					// C 1118
	{
	  OTF_DWORDTAG_VAL! ('font_table_error, tbl,
			     (feature_table as c_int) + 2 + (feature_index as c_int) * 6,
			     &raw mut feature_id);			// C 1120-1121
	  *ftags.offset (k as isize) = feature_id;			// C 1122
	  k += 1;
	}
      /* Now get all the other features.  */
      j = 0;								// C 1125
      while j < n_features as c_int
	{
	  OTF_INT16_VAL! ('font_table_error, tbl,
			  (langsys_table as c_int) + 6 + j * 2, &raw mut feature_index);	// C 1127
	  OTF_DWORDTAG_VAL! ('font_table_error, tbl,
			     (feature_table as c_int) + 2 + (feature_index as c_int) * 6,
			     &raw mut feature_id);			// C 1128-1129
	  *ftags.offset (k as isize) = feature_id;			// C 1130
	  k += 1;
	  j += 1;
	}

      /* Check the features for this table.  */
      farray[0] = features[i as usize];					// C 1134
      farray[1] = Qnil ();						// C 1135
      if ! uniscribe_check_features (farray.as_mut_ptr (), ftags, n_match_features)	// C 1136
	{ break 'font_table_error; }					// C 1137
      SAFE_FREE! (sa_count);						// C 1138
      i += 1;
    }

  retval = 1;								// C 1141
  }

  /* C 1143-1150: `done:` / `no_support:` / `font_table_error:` and the one cleanup they share.  */
  SelectObject (context, old_font);					// C 1147
  DeleteObject (check_font);						// C 1148
  release_frame_dc (f, context);					// C 1149

  return retval;							// C 1151
}

/* C 1154-1239: `otf_features`.  C's `goto font_table_error` (in the OTF macros) and the label at
   C 1237 are the labelled block below; C 1167's `for (i = n_scripts - 1; i >= 0; i--)` counts an
   `unsigned short` down through zero into -1, so the counter is the port's `c_int` (Deviation 12:
   C's promotion of the loop bound is spelled with `as c_int`).  */
/// C: static Lisp_Object otf_features (HDC context, const char *table)
#[cfg (feature = "have_ntgui")]
unsafe fn otf_features (context: HDC, table: *const c_char) -> Lisp_Object
{
  let mut script_list: Lisp_Object = Qnil ();				// C 1157
  let mut scriptlist_table: c_ushort = 0;				// C 1158
  let mut n_scripts: c_ushort = 0;
  let mut feature_table: c_ushort = 0;
  let tbl: DWORD = OTF_TAG! (table) as DWORD;				// C 1159
  let mut i: c_int;							// C 1160
  let mut j: c_int;
  let mut k: c_int;

  'font_table_error: {							// C 1237's label
  /* Look for scripts in the table.  */
  OTF_INT16_VAL! ('font_table_error, tbl, 4, &raw mut scriptlist_table);	// C 1163
  OTF_INT16_VAL! ('font_table_error, tbl, 6, &raw mut feature_table);	// C 1164
  OTF_INT16_VAL! ('font_table_error, tbl, scriptlist_table as c_int,
		  &raw mut n_scripts);					// C 1165

  i = n_scripts as c_int - 1;						// C 1167
  while i >= 0
    {
      let mut script: [c_char; 5] = [0; 5];				// C 1169
      let mut lang: [c_char; 5] = [0; 5];
      let mut script_table: c_ushort = 0;				// C 1170
      let mut lang_count: c_ushort = 0;
      let mut langsys_table: c_ushort = 0;
      let mut feature_count: c_ushort = 0;
      let mut script_tag: Lisp_Object;					// C 1171
      let mut langsys_list: Lisp_Object;
      let mut langsys_tag: Lisp_Object;
      let mut feature_list: Lisp_Object;
      let mut record_offset: c_ushort				// C 1172
	= ((scriptlist_table as c_int) + 2 + i * 6) as c_ushort;
      OTF_TAG_VAL! ('font_table_error, tbl, record_offset as c_int,
		    script.as_mut_ptr ());				// C 1173
      OTF_INT16_VAL! ('font_table_error, tbl, (record_offset as c_int) + 4,
		      &raw mut script_table);				// C 1174

      /* Offset is from beginning of script table.  */
      script_table = script_table.wrapping_add (scriptlist_table);	// C 1177

      script_tag = intern (script.as_ptr ());				// C 1179
      langsys_list = Qnil ();						// C 1180

      /* Optional default lang.  */
      OTF_INT16_VAL! ('font_table_error, tbl, script_table as c_int,
		      &raw mut langsys_table);				// C 1183
      if langsys_table != 0						// C 1184
	{
	  /* Offset is from beginning of script table.  */
	  langsys_table = langsys_table.wrapping_add (script_table);	// C 1187

	  langsys_tag = Qnil ();					// C 1189
	  feature_list = Qnil ();					// C 1190
	  OTF_INT16_VAL! ('font_table_error, tbl, (langsys_table as c_int) + 4,
			  &raw mut feature_count);			// C 1191
	  k = feature_count as c_int - 1;				// C 1192
	  while k >= 0
	    {
	      let mut feature: [c_char; 5] = [0; 5];			// C 1194
	      let mut index: c_ushort = 0;				// C 1195
	      OTF_INT16_VAL! ('font_table_error, tbl,
			      (langsys_table as c_int) + 6 + k * 2, &raw mut index);	// C 1196
	      OTF_TAG_VAL! ('font_table_error, tbl,
			    (feature_table as c_int) + 2 + (index as c_int) * 6,
			    feature.as_mut_ptr ());			// C 1197
	      feature_list = Fcons (intern (feature.as_ptr ()), feature_list);	// C 1198
	      k -= 1;
	    }
	  langsys_list = Fcons (Fcons (langsys_tag, feature_list), langsys_list);	// C 1200-1201
	}

      /* List of supported languages.  */
      OTF_INT16_VAL! ('font_table_error, tbl, (script_table as c_int) + 2,
		      &raw mut lang_count);				// C 1205

      j = lang_count as c_int - 1;					// C 1207
      while j >= 0
	{
	  record_offset = ((script_table as c_int) + 4 + j * 6) as c_ushort;	// C 1209
	  OTF_TAG_VAL! ('font_table_error, tbl, record_offset as c_int,
			lang.as_mut_ptr ());				// C 1210
	  OTF_INT16_VAL! ('font_table_error, tbl, (record_offset as c_int) + 4,
			  &raw mut langsys_table);			// C 1211

	  /* Offset is from beginning of script table.  */
	  langsys_table = langsys_table.wrapping_add (script_table);	// C 1214

	  langsys_tag = intern (lang.as_ptr ());			// C 1216
	  feature_list = Qnil ();					// C 1217
	  OTF_INT16_VAL! ('font_table_error, tbl, (langsys_table as c_int) + 4,
			  &raw mut feature_count);			// C 1218
	  k = feature_count as c_int - 1;				// C 1219
	  while k >= 0
	    {
	      let mut feature: [c_char; 5] = [0; 5];			// C 1221
	      let mut index: c_ushort = 0;				// C 1222
	      OTF_INT16_VAL! ('font_table_error, tbl,
			      (langsys_table as c_int) + 6 + k * 2, &raw mut index);	// C 1223
	      OTF_TAG_VAL! ('font_table_error, tbl,
			    (feature_table as c_int) + 2 + (index as c_int) * 6,
			    feature.as_mut_ptr ());			// C 1224
	      feature_list = Fcons (intern (feature.as_ptr ()), feature_list);	// C 1225
	      k -= 1;
	    }
	  langsys_list = Fcons (Fcons (langsys_tag, feature_list), langsys_list);	// C 1227-1228

	  j -= 1;
	}

      script_list = Fcons (Fcons (script_tag, langsys_list), script_list);	// C 1232
      i -= 1;
    }

  return script_list;							// C 1235
  }

  /* C 1237-1238: `font_table_error:` and its `return Qnil`.  */
  return Qnil ();
}

/* C 1241: `#ifdef HAVE_HARFBUZZ` (the file's second arm; the first is C 54-88).  */
/* C 1243-1254: `w32hb_list` - "W32 implementation of the 'list' method for HarfBuzz backend".  */
/// C: static Lisp_Object w32hb_list (struct frame *f, Lisp_Object font_spec)
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
unsafe extern "C" fn w32hb_list (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{
  let f = f as *mut frame;						// C 1245
  let fonts: Lisp_Object = w32font_list_internal (f, font_spec, true);	// C 1247
  FONT_ADD_LOG! (c"harfbuzz-list".as_ptr (), font_spec, fonts);		// C 1248

  let mut tail: Lisp_Object = fonts;					// C 1250
  while CONSP (tail)
    {
      ASET (XCAR (tail), FONT_TYPE_INDEX as isize, Qharfbuzz ());	// C 1251
      tail = XCDR (tail);
    }

  return fonts;								// C 1253
}

/* C 1256-1266: `w32hb_match`.  */
/// C: static Lisp_Object w32hb_match (struct frame *f, Lisp_Object font_spec)
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
unsafe extern "C" fn w32hb_match (f: *mut c_void, font_spec: Lisp_Object) -> Lisp_Object
{
  let f = f as *mut frame;						// C 1258
  let entity: Lisp_Object = w32font_match_internal (f, font_spec, true);	// C 1260
  FONT_ADD_LOG! (c"harfbuzz-match".as_ptr (), font_spec, entity);	// C 1261

  if ! NILP (entity)							// C 1263
    { ASET (entity, FONT_TYPE_INDEX as isize, Qharfbuzz ()); }		// C 1264
  return entity;							// C 1265
}

/* C 1268-1276: `free_cb` - "Callback function to free memory.  We need this so we could pass it to
   HarfBuzz as the function to call to destroy objects for which we allocated data by calling our
   'malloc' (as opposed to 'malloc' from the MS CRT, against which HarfBuzz was linked)."  */
/// C: static void free_cb (void *ptr)
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
unsafe extern "C" fn free_cb (ptr: *mut c_void)
{
  free (ptr);								// C 1275
}

/* C 1278-1316: `w32hb_get_font_table` - "A function used as reference_table_func for HarfBuzz.  It
   returns the data of a specified table of a font as a blob."  C 1288-1292's `#if
   GNUC_PREREQ (4, 3, 0)` / `#else` pair around one initialiser is carried with both arms and the
   `GNUC_PREREQ!` macro of `include/conf_post.rs` (the `src/xgselect.rs` 92 shape); C's
   `__builtin_bswap32` is Rust's `swap_bytes` (PORTING.md 2.6) and the `#else` arm is gnulib's
   `bswap_32`.  */
/// C: static hb_blob_t *w32hb_get_font_table (hb_face_t *face, hb_tag_t tag, void *data)
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
unsafe extern "C" fn w32hb_get_font_table (face: *mut hb_face_t, tag: hb_tag_t, data: *mut c_void)
					    -> *mut hb_blob_t
{
  let f: *mut frame = XFRAME (selected_frame);				// C 1283
  let context: HDC = get_frame_dc (f);					// C 1284
  let old_font: HFONT = SelectObject (context, data as HFONT) as HFONT;	// C 1285
  let mut font_data: *mut c_char = core::ptr::null_mut ();		// C 1286
  let mut font_data_size: DWORD = 0;					// C 1287
  let mut val: DWORD = 0;
  /* C 1288-1292: `#if GNUC_PREREQ (4, 3, 0)` / `#else` / `#endif` - both arms (Deviation 10).  */
  #[allow(unused_mut)]
  let mut table: DWORD;
  if GNUC_PREREQ! (4, 3, 0)
    { table = tag.swap_bytes (); }					// C 1289
  else
    { table = bswap_32 (tag); }						// C 1291
  let mut blob: *mut hb_blob_t = core::ptr::null_mut ();		// C 1293

  val = GetFontData (context, table, 0, font_data as LPVOID, font_data_size);	// C 1295
  if val != GDI_ERROR							// C 1296
    {
      font_data_size = val;						// C 1298
      /* Don't call xmalloc, because it can signal an error, while
	 we are inside a critical section established by get_frame_dc.  */
      font_data = malloc (font_data_size as usize) as *mut c_char;	// C 1301
      if ! font_data.is_null ()						// C 1302
	{
	  val = GetFontData (context, table, 0, font_data as LPVOID, font_data_size);	// C 1304
	  if val != GDI_ERROR						// C 1305
	    {
	      blob = (fn_hb_blob_create.unwrap ()) (font_data, font_data_size,	// C 1306-1307
						    HB_MEMORY_MODE_READONLY,
						    font_data as *mut c_void, Some (free_cb));
	    }
	}
    }

  /* Restore graphics context.  */
  SelectObject (context, old_font);					// C 1312
  release_frame_dc (f, context);					// C 1313

  return blob;								// C 1315
}

/* C 1318-1352: `w32hb_get_font` - "Helper function used by the HarfBuzz implementations of the
   encode_char, has_char, and begin_hb_font methods.  It creates an hb_font_t object for a given
   Emacs font."  */
/// C: static hb_font_t *w32hb_get_font (struct font *font, double *scale)
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
unsafe extern "C" fn w32hb_get_font (font: *mut font, scale: *mut c_double) -> *mut hb_font_t
{
  let mut hb_font: *mut hb_font_t = core::ptr::null_mut ();		// C 1324
  let font_handle: HFONT = FONT_HANDLE (font);				// C 1325
  let hb_face: *mut hb_face_t						// C 1326-1327
    = (fn_hb_face_create_for_tables.unwrap ()) (Some (w32hb_get_font_table), font_handle, None);
  if (fn_hb_face_get_glyph_count.unwrap ()) (hb_face) > 0		// C 1328
    {
      hb_font = (fn_hb_font_create.unwrap ()) (hb_face);		// C 1330
      /* This is needed for HarfBuzz before 2.0.0; it is the default
	 in later versions.  */
      (fn_hb_ot_font_set_funcs.unwrap ()) (hb_font);			// C 1333
    }

  let uniscribe_font: *mut uniscribe_font_info				// C 1336-1337
    = font as *mut uniscribe_font_info;
  let upem: c_uint = (fn_hb_face_get_upem.unwrap ()) (hb_face);		// C 1338
  eassert! (upem > 0);							// C 1339
  /* https://support.microsoft.com/en-sg/help/74299/info-calculating-the-logical-height-and-point-size-of-a-font.  */
  let font_point_size: LONG						// C 1341-1343
    = (*uniscribe_font).w32_font.metrics.tmHeight
    - (*uniscribe_font).w32_font.metrics.tmInternalLeading;
  /* https://docs.microsoft.com/en-us/typography/opentype/spec/ttch01,
     under "Converting FUnits to pixels".  */
  *scale = font_point_size as c_double * 1.0 / upem as c_double;	// C 1346

  (fn_hb_face_destroy.unwrap ()) (hb_face);				// C 1348

  /* FIXME: Can hb_font be non-NULL and yet invalid?  Compare to get_empty?  */
  return hb_font;							// C 1351
}

/* C 1354-1391: "W32 implementation of encode_char method for HarfBuzz backend."  */
/// C: static unsigned w32hb_encode_char (struct font *font, int c)
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
unsafe extern "C" fn w32hb_encode_char (font: *mut font, c: c_int) -> c_uint
{
  let uniscribe_font: *mut uniscribe_font_info				// C 1358-1359
    = font as *mut uniscribe_font_info;
  eassert! ((*uniscribe_font).w32_font.font.driver			// C 1360
	    == (&raw const harfbuzz_font_driver) as *const c_void);

  if w32_use_direct_write (&raw mut (*uniscribe_font).w32_font)		// C 1362
    {
      let encoded: c_uint = w32_dwrite_encode_char (font, c);		// C 1364

      /* The call to w32_dwrite_encode_char may fail, disabling
	 DirectWrite for this font.  So check again.  */
      if w32_use_direct_write (&raw mut (*uniscribe_font).w32_font)	// C 1368
	{ return encoded; }						// C 1369
    }

  let mut hb_font: *mut hb_font_t					// C 1372
    = (*uniscribe_font).cache as *mut hb_font_t;

  /* First time we use this font with HarfBuzz, create the hb_font_t
     object and cache it.  */
  if hb_font.is_null ()							// C 1376
    {
      let mut scale: c_double = 0.0;					// C 1378
      hb_font = w32hb_get_font (font, &raw mut scale);			// C 1379
      if hb_font.is_null ()						// C 1380
	{ return FONT_INVALID_CODE; }					// C 1381

      (*uniscribe_font).cache = hb_font as *mut c_void;			// C 1383
      eassert! (scale > 0.0);						// C 1384
      (*uniscribe_font).scale = scale;					// C 1385
    }
  let mut glyph: hb_codepoint_t = 0;					// C 1387
  if (fn_hb_font_get_nominal_glyph.unwrap ()) (hb_font, c as hb_codepoint_t, &raw mut glyph) != 0
    { return glyph; }							// C 1389
  return FONT_INVALID_CODE;						// C 1390
}

/* C 1393-1399: "W32 implementation of HarfBuzz begin_hb_font and end_hb_font methods.  Return a
   HarfBuzz font object for FONT and store in POSITION_UNIT the scale factor to convert a
   hb_position_t value to the number of pixels.  Return NULL if HarfBuzz font object is not
   available for FONT."  */
/// C: static hb_font_t *w32hb_begin_font (struct font *font, double *position_unit)
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
unsafe extern "C" fn w32hb_begin_font (font: *mut font, position_unit: *mut c_double)
				       -> *mut hb_font_t
{
  let uniscribe_font: *mut uniscribe_font_info				// C 1403-1404
    = font as *mut uniscribe_font_info;
  eassert! ((*uniscribe_font).w32_font.font.driver			// C 1405
	    == (&raw const harfbuzz_font_driver) as *const c_void);

  /* First time we use this font with HarfBuzz, create the hb_font_t
     object and cache it.  */
  if (*uniscribe_font).cache.is_null ()					// C 1409
    {
      let mut scale: c_double = 0.0;					// C 1411
      (*uniscribe_font).cache = w32hb_get_font (font, &raw mut scale) as *mut c_void;	// C 1412
      eassert! (scale > 0.0);						// C 1413
      (*uniscribe_font).scale = scale;					// C 1414
    }
  *position_unit = (*uniscribe_font).scale;				// C 1416
  return (*uniscribe_font).cache as *mut hb_font_t;			// C 1417
}

/* C 1420-1425: "W32 implementation of get_variation_glyphs method for HarfBuzz.  Return the number
   of variation glyphs of character C supported by FONT.  VARIATIONS is an array of 256 elements.
   If the variation selector N (1..256) defines a glyph, that glyph code is stored in the (N-1)th
   element of VARIATIONS."  */
/// C: static int w32hb_get_variation_glyphs (struct font *font, int c,
/// C:   unsigned variations[256])
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
unsafe extern "C" fn w32hb_get_variation_glyphs (font: *mut font, c: c_int,
						 variations: *mut c_uint) -> c_int
{
  let uniscribe_font: *mut uniscribe_font_info				// C 1429-1430
    = font as *mut uniscribe_font_info;
  eassert! ((*uniscribe_font).w32_font.font.driver			// C 1431
	    == (&raw const harfbuzz_font_driver) as *const c_void);

  /* First time we use this font with HarfBuzz, create the hb_font_t
     object and cache it.  */
  if (*uniscribe_font).cache.is_null ()					// C 1435
    {
      let mut scale: c_double = 0.0;					// C 1437
      (*uniscribe_font).cache = w32hb_get_font (font, &raw mut scale) as *mut c_void;	// C 1438
      eassert! (scale > 0.0);						// C 1439
      (*uniscribe_font).scale = scale;					// C 1440
    }

  let mut i: c_int;							// C 1443
  let mut n: c_int = 0;
  let hb_font: *mut hb_font_t = (*uniscribe_font).cache as *mut hb_font_t;	// C 1444
  i = 0;								// C 1445
  while i < 16
    {
      if (fn_hb_font_get_variation_glyph.unwrap ()) (hb_font, c as hb_codepoint_t,
						     (0xFE00 + i) as hb_codepoint_t,
						     variations.offset (i as isize)
						     as *mut hb_codepoint_t) != 0	// C 1447
	{ n += 1; }							// C 1448
      else
	{ *variations.offset (i as isize) = 0; }			// C 1450
      i += 1;
    }
  while i < 256								// C 1452
    {
      if (fn_hb_font_get_variation_glyph.unwrap ()) (hb_font, c as hb_codepoint_t,
						     (0xE0100 + (i - 16)) as hb_codepoint_t,
						     variations.offset (i as isize)
						     as *mut hb_codepoint_t) != 0	// C 1454-1455
	{ n += 1; }							// C 1456
      else
	{ *variations.offset (i as isize) = 0; }			// C 1458
      i += 1;
    }

  return n;								// C 1461
}

/* C 1465-1467: `#undef OTF_INT16_VAL` / `#undef OTF_TAG_VAL` / `#undef OTF_TAG`.  Rust has no
   `#undef`; the three `macro_rules!` stop being used at C 1239 and C's lines are kept as C's own
   text (Deviation 13).

     C 1465  #undef OTF_INT16_VAL
     C 1466  #undef OTF_TAG_VAL
     C 1467  #undef OTF_TAG
  */

/* C 1470-1499: `uniscribe_font_driver`, the file's `font_driver` instance.  `LISPSYM_INITIALLY
   (Quniscribe)` is the compile-time symbol form the `src/w32font.rs` 3731 initialiser spells with
   `iQuniscribe`, and the `NULL`s are `None` in the port's `Option` fields.  Two slots take a
   `transmute` because the mirror's declarations and the record's members spell one pointer
   differently: `w32font_get_cache`'s `struct frame *` against the slot's `*mut c_void`, and
   `w32font_draw`'s `*mut c_void` stand-in against the slot's `*mut glyph_string` (the
   `include/w32font.rs` 173 note; Deviation 14).  */
/// C: struct font_driver uniscribe_font_driver = { LISPSYM_INITIALLY (Quniscribe), 0 /* case
/// C:   insensitive */, w32font_get_cache, uniscribe_list, uniscribe_match, uniscribe_list_family,
/// C:   NULL /* free_entity */, uniscribe_open, uniscribe_close, NULL /* prepare_face */,
/// C:   NULL /* done_face */, w32font_has_char, uniscribe_encode_char, w32font_text_extents,
/// C:   w32font_draw, NULL /* get_bitmap */, NULL /* free_bitmap */, NULL /* anchor_point */,
/// C:   uniscribe_otf_capability /* Defined so (font-get FONTOBJ :otf) works.  */,
/// C:   NULL /* otf_drive - use shape instead.  */, NULL /* start_for_frame */,
/// C:   NULL /* end_for_frame */, uniscribe_shape, NULL /* check */,
/// C:   NULL /* get_variation_glyphs */, NULL /* filter_properties */,
/// C:   NULL /* cached_font_ok */ };
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut uniscribe_font_driver: font_driver = font_driver {
  r#type: (((iQuniscribe * core::mem::size_of::<Lisp_Symbol> ()) as EMACS_UINT
	    + Lisp_Type::Lisp_Symbol as EMACS_UINT) as Lisp_Object),	// C 1472
  case_sensitive: false,	/* case insensitive */			// C 1473
  get_cache: Some (unsafe {
    core::mem::transmute::<unsafe extern "C" fn (*mut frame) -> Lisp_Object, _> (w32font_get_cache)
  }),									// C 1474
  list: Some (uniscribe_list),						// C 1475
  r#match: Some (uniscribe_match),					// C 1476
  list_family: Some (uniscribe_list_family),				// C 1477
  free_entity: None,		/* NULL */				// C 1478
  open_font: Some (uniscribe_open),					// C 1479
  close_font: Some (uniscribe_close),					// C 1480
  prepare_face: None,		/* NULL */				// C 1481
  done_face: None,		/* NULL */				// C 1482
  has_char: Some (w32font_has_char),					// C 1483
  encode_char: Some (uniscribe_encode_char),				// C 1484
  text_extents: Some (w32font_text_extents),				// C 1485
  #[cfg (feature = "have_window_system")]
  draw: Some (unsafe {
    core::mem::transmute::<unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int, c_int, bool)
						  -> c_int, _> (w32font_draw)
  }),									// C 1486
  #[cfg (feature = "have_window_system")]
  get_bitmap: None,		/* NULL */				// C 1487
  #[cfg (feature = "have_window_system")]
  free_bitmap: None,		/* NULL */				// C 1488
  anchor_point: None,		/* NULL */				// C 1489
  otf_capability: Some (uniscribe_otf_capability),			// C 1490
  otf_drive: None,		/* NULL */				// C 1491
  start_for_frame: None,	/* NULL */				// C 1492
  end_for_frame: None,		/* NULL */				// C 1493
  shape: Some (uniscribe_shape),					// C 1494
  check: None,			/* NULL */				// C 1495
  get_variation_glyphs: None,	/* NULL */				// C 1496
  filter_properties: None,	/* NULL */				// C 1497
  cached_font_ok: None,		/* NULL */				// C 1498
  combining_capability: None,	/* C's initialiser stops here */
  drop_xrender_surfaces: None,
  #[cfg (feature = "harfbuzz")]
  begin_hb_font: None,
  #[cfg (feature = "harfbuzz")]
  end_hb_font: None,
};

/* C 1501-1503: "Note that this should be called at every startup, not just when dumping, as it
   needs to test for the existence of the Uniscribe library."  C 1505's `static void
   syms_of_w32uniscribe_for_pdumper (void);` is dropped (0.1-12); the definition is C 1531.  */
/// C: void syms_of_w32uniscribe (void);
/// C: static void syms_of_w32uniscribe_for_pdumper (void);
/* C 1507-1511: `syms_of_w32uniscribe`.  */
/// C: void syms_of_w32uniscribe (void)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_w32uniscribe ()
{
  pdumper_do_now_and_after_load (Some (syms_of_w32uniscribe_for_pdumper));	// C 1510
}

/* C 1513-1529: `load_harfbuzz_funcs`.  The ten `LOAD_DLL_FN` invocations are the macro this unit
   defines (C 1517-1526), and C's `return hbfont_init_w32_funcs (library);` is the mirror's
   declaration.  */
/// C: static bool load_harfbuzz_funcs (HMODULE library)
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
unsafe fn load_harfbuzz_funcs (library: HMODULE) -> bool
{
  LOAD_DLL_FN! (library, hb_blob_create, W32_PFN_hb_blob_create, fn_hb_blob_create,
		c"hb_blob_create");					// C 1517
  LOAD_DLL_FN! (library, hb_face_create_for_tables, W32_PFN_hb_face_create_for_tables,
		fn_hb_face_create_for_tables, c"hb_face_create_for_tables");	// C 1518
  LOAD_DLL_FN! (library, hb_face_get_glyph_count, W32_PFN_hb_face_get_glyph_count,
		fn_hb_face_get_glyph_count, c"hb_face_get_glyph_count");	// C 1519
  LOAD_DLL_FN! (library, hb_font_create, W32_PFN_hb_font_create, fn_hb_font_create,
		c"hb_font_create");					// C 1520
  LOAD_DLL_FN! (library, hb_font_destroy, W32_PFN_hb_font_destroy, fn_hb_font_destroy,
		c"hb_font_destroy");					// C 1521
  LOAD_DLL_FN! (library, hb_face_get_upem, W32_PFN_hb_face_get_upem, fn_hb_face_get_upem,
		c"hb_face_get_upem");					// C 1522
  LOAD_DLL_FN! (library, hb_face_destroy, W32_PFN_hb_face_destroy, fn_hb_face_destroy,
		c"hb_face_destroy");					// C 1523
  LOAD_DLL_FN! (library, hb_font_get_nominal_glyph, W32_PFN_hb_font_get_nominal_glyph,
		fn_hb_font_get_nominal_glyph, c"hb_font_get_nominal_glyph");	// C 1524
  LOAD_DLL_FN! (library, hb_font_get_variation_glyph, W32_PFN_hb_font_get_variation_glyph,
		fn_hb_font_get_variation_glyph, c"hb_font_get_variation_glyph");	// C 1525
  LOAD_DLL_FN! (library, hb_ot_font_set_funcs, W32_PFN_hb_ot_font_set_funcs,
		fn_hb_ot_font_set_funcs, c"hb_ot_font_set_funcs");	// C 1526
  return hbfont_init_w32_funcs (library);				// C 1527
}

/* C 1531-1628: `syms_of_w32uniscribe_for_pdumper`.  The two `#ifdef WINDOWSNT`/`#else /* Cygwin
   */` pairs are C 1541-1573 and C 1599-1603; both arms are carried (P-86 (1)).  The nine
   function-pointer stores go through `transmute`, which is how the port spells C's `(void *)`
   cast followed by an implicit conversion to the pointer type (the `DEF_DLL_FN`/`LOAD_DLL_FN`
   macros' own spelling), and `harfbuzz_font_driver = uniscribe_font_driver;` is C's struct copy,
   which Rust spells `std::ptr::read` (Deviation 15).  */
/// C: static void syms_of_w32uniscribe_for_pdumper (void)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn syms_of_w32uniscribe_for_pdumper ()
{
  /* Don't init Uniscribe and HarfBuzz when dumping */
  if ! initialized							// C 1535
    { return; }								// C 1536

  /* Don't register if Uniscribe is not available.  */
  let mut uniscribe: HMODULE;						// C 1539

  #[cfg (feature = "windowsnt")]
  {
  uniscribe = LoadLibrary (c"usp10.dll".as_ptr ());			// C 1542
  if uniscribe.is_null ()						// C 1543
    { return; }								// C 1544

  pfnScriptItemize = core::mem::transmute::<VOIDFNPTR, _>			// C 1546
    (get_proc_addr (uniscribe, c"ScriptItemize".as_ptr ()));
  pfnScriptShape = core::mem::transmute::<VOIDFNPTR, _>			// C 1547
    (get_proc_addr (uniscribe, c"ScriptShape".as_ptr ()));
  pfnScriptPlace = core::mem::transmute::<VOIDFNPTR, _>			// C 1548
    (get_proc_addr (uniscribe, c"ScriptPlace".as_ptr ()));
  pfnScriptGetGlyphABCWidth = core::mem::transmute::<VOIDFNPTR, _>	// C 1549-1550
    (get_proc_addr (uniscribe, c"ScriptGetGlyphABCWidth".as_ptr ()));
  pfnScriptFreeCache = core::mem::transmute::<VOIDFNPTR, _>		// C 1551-1552
    (get_proc_addr (uniscribe, c"ScriptFreeCache".as_ptr ()));
  pfnScriptGetCMap = core::mem::transmute::<VOIDFNPTR, _>		// C 1553-1554
    (get_proc_addr (uniscribe, c"ScriptGetCMap".as_ptr ()));
  if pfnScriptItemize.is_none () || pfnScriptShape.is_none ()		// C 1555
      || pfnScriptPlace.is_none ()					// C 1556
      || pfnScriptGetGlyphABCWidth.is_none () || pfnScriptFreeCache.is_none ()	// C 1556
      || pfnScriptGetCMap.is_none ()					// C 1557
    {
      FreeLibrary (uniscribe);						// C 1559
      return;								// C 1560
    }
  }
  #[cfg (not (feature = "windowsnt"))]
  {
  uniscribe = GetModuleHandle (c"usp10.dll".as_ptr ());			// C 1563
  if uniscribe.is_null ()						// C 1564
    { return; }								// C 1565

  pfnScriptItemize = Some (ScriptItemize);				// C 1567
  pfnScriptShape = Some (ScriptShape);					// C 1568
  pfnScriptPlace = Some (ScriptPlace);					// C 1569
  pfnScriptGetGlyphABCWidth = Some (ScriptGetGlyphABCWidth);		// C 1570
  pfnScriptFreeCache = Some (ScriptFreeCache);				// C 1571
  pfnScriptGetCMap = Some (ScriptGetCMap);				// C 1572
  }

  uniscribe_available = 1;						// C 1575

  register_font_driver (&raw const uniscribe_font_driver, core::ptr::null_mut ());	// C 1577

  script_get_font_scripts_fn						// C 1579-1580
    = core::mem::transmute::<VOIDFNPTR, ScriptGetFontScriptTags_Proc>
      (get_proc_addr (uniscribe, c"ScriptGetFontScriptTags".as_ptr ()));
  script_get_font_languages_fn						// C 1581-1582
    = core::mem::transmute::<VOIDFNPTR, ScriptGetFontLanguageTags_Proc>
      (get_proc_addr (uniscribe, c"ScriptGetFontLanguageTags".as_ptr ()));
  script_get_font_features_fn						// C 1583-1584
    = core::mem::transmute::<VOIDFNPTR, ScriptGetFontFeatureTags_Proc>
      (get_proc_addr (uniscribe, c"ScriptGetFontFeatureTags".as_ptr ()));
  if script_get_font_scripts_fn.is_some ()				// C 1585
      && script_get_font_languages_fn.is_some ()			// C 1586
      && script_get_font_features_fn.is_some ()				// C 1587
    { uniscribe_new_apis = true; }					// C 1588
  else
    { uniscribe_new_apis = false; }					// C 1590

  #[cfg (feature = "harfbuzz")]
  {
  /* Currently, HarfBuzz DLLs are always named libharfbuzz-0.dll on
     MS-Windows and cygharfbuzz-0.dll on Cygwin, as
     the project keeps the ABI backward-compatible.  So we can
     hard-code the name of the library here, for now.  If they ever
     break ABI compatibility, we may need to load the DLL that
     corresponds to the HarfBuzz version for which Emacs was built.  */
  #[cfg (feature = "windowsnt")]
  let harfbuzz: HMODULE = LoadLibrary (c"libharfbuzz-0.dll".as_ptr ());	// C 1600
  #[cfg (not (feature = "windowsnt"))]
  let harfbuzz: HMODULE = LoadLibrary (c"cygharfbuzz-0.dll".as_ptr ());	// C 1602
  /* Don't register if HarfBuzz is not available.  */
  if harfbuzz.is_null ()						// C 1605
    { return; }								// C 1606

  if ! load_harfbuzz_funcs (harfbuzz)					// C 1608
    { return; }								// C 1609

  Fput (Quniscribe (), Qfont_driver_superseded_by (), Qharfbuzz ());	// C 1611
  harfbuzz_available = 1;						// C 1612
  harfbuzz_font_driver = std::ptr::read (&raw const uniscribe_font_driver);	// C 1613
  harfbuzz_font_driver.r#type = Qharfbuzz ();				// C 1614
  harfbuzz_font_driver.list = Some (w32hb_list);			// C 1615
  harfbuzz_font_driver.r#match = Some (w32hb_match);			// C 1616
  harfbuzz_font_driver.encode_char = Some (w32hb_encode_char);		// C 1617
  harfbuzz_font_driver.otf_capability = Some (hbfont_otf_capability);	// C 1618
  harfbuzz_font_driver.shape = Some (hbfont_shape);			// C 1619
  harfbuzz_font_driver.get_variation_glyphs = Some (w32hb_get_variation_glyphs);	// C 1620
  harfbuzz_font_driver.combining_capability = Some (hbfont_combining_capability);	// C 1621
  /* C's member is `void (*begin_hb_font) (struct font *, double *)` (font.h 813) and the port's
     is the mirror's `*mut c_void` result, so the assignment is a `transmute` (Deviation 14).  */
  harfbuzz_font_driver.begin_hb_font = Some (core::mem::transmute::<
    unsafe extern "C" fn (*mut font, *mut c_double) -> *mut hb_font_t, _> (w32hb_begin_font));	// C 1622
  register_font_driver (&raw const harfbuzz_font_driver, core::ptr::null_mut ());	// C 1623

  w32_initialize_direct_write ();					// C 1625
  }
}

/* The names the file's own text uses that no mirror in this graph carries (P-55's shape: each is
   declared here at a `/// C:` line with the reference's own text, until the mirror that owns it
   lands - the ledger's `## Owed`; PORTING.md P-90 (7): a unit may declare a name no mirror has).  */
/// C: lisp.h 4286: `Lisp_Object memq_no_quit (Lisp_Object elt, Lisp_Object list);` (C 764)
/// C: font.h 846: `extern Lisp_Object font_make_object (int, Lisp_Object, int);` (C 168)
/// C: font.h 896: `extern void register_font_driver (struct font_driver const *,
/// C:                                          struct frame *);` (C 1577/C 1623)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn memq_no_quit (elt: Lisp_Object, list: Lisp_Object) -> Lisp_Object;
  pub fn font_make_object (size: c_int, entity: Lisp_Object, pixel_size: c_int) -> Lisp_Object;
  pub fn register_font_driver (driver: *const font_driver, f: *mut frame);
}
