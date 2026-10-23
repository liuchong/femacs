// femacs: port of src/w32dwrite.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  The target's
// platform object variables are empty (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`,
// `CYGWIN_OBJ=`, docs/baseline/frozen-build-sources.txt), so `w32dwrite.o` is in no source list and
// `extract-abi.py`'s checklist for the file is empty (P-86 (2)); the ledger
// `docs/ports/w32dwrite.md` carries that clause's substitute - the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "have_ntgui"`, on every item *and on the include graph*
//     (P-86 (8)), which is why the frozen configuration compiles nothing here;
//   * C 40-42's `#if !defined MINGW_W64 && !defined CYGWIN` is
//     `all (feature = "have_ntgui", not (feature = "mingw_w64"), not (feature = "cygwin"))`;
//   * C 52's `#ifndef MINGW_W64` opens the file's own DirectWrite/D2D stand-in declarations (C
//     53-544); its `#else` arm is C 546-547's two SDK includes.  The two arms are
//     `not (feature = "mingw_w64")` and `feature = "mingw_w64"`;
//   * C 1136-1146 is the `#ifdef MINGW_W64` / `#else` pair around the three IUnknown vtable slots;
//   * C 1332-1339 is `#ifdef HAVE_HARFBUZZ` / `#else` / `#endif` in `w32_use_direct_write`.
//
// **Acquisition note - this piece is `text-only` (P-86 (3)(ii)).**  The machine has no Windows and
// no mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty
// and `find` finds no `windows.h`/`dwrite.h`/`dwrite_3.h`/`d2d1.h`), so there is no `<windows.h>`
// and no `<dwrite_3.h>`/`<d2d1.h>` to compile or measure against: **every record and every value
// of the stand-in sections below is `UNVERIFIED`** (the documented SDK text, not a measurement) and
// the layout assertions are owed.  What the round does show is that the `have_ntgui` + `windowsnt`
// + `have_window_system` + `harfbuzz` arm type-checks under this machine's `rustc` - the ledger's
// `## Evidence` carries that reading and names the one error that belongs to the include graph
// rather than to this unit.
//
// `docs/ports/w32dwrite.md` carries the C line ranges, the acceptance readings, every deviation
// (the dropped static forward declarations, the SDK stand-in sections, the `void *` conversions,
// the in-unit declarations no mirror carries) and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, unused_must_use,
         let_underscore_drop, clippy::all)]

/* C 1-18 is the file's opening ownership/title block, dropped (PORTING.md 0.1-19); the port's
   first line is the revision line above.  C 19 is blank.  C 20-34 is the file's own design note,
   carried as C's words (it names no product and recites nothing):  */

/* This requires the HarfBuzz font backend to be available.

   It works by modifying the HarfBuzz backend to use DirectWrite at
   some points, if it is available:

   - When encoding characters: w32hb_encode_char
   - When measuring text: w32font_text_extents
   - When drawing text: w32font_draw

   DirectWrite is setup by calling w32_initialize_direct_write.  From
   that point, the function w32_use_direct_write will return true if
   DirectWrite is to be used.

   DirectWrite is available since Windows 7, but we don't activate it on
   versions before 8.1, because color fonts are only available since that.  */

include!("include/config.rs");                  // C 36: `#include <config.h>`

// C 37: `#include <math.h>` (`round`, C 859/864/868/873/905/906).  `lib/include/math.rs` mirrors
// that header but carries no `round` row yet, so this unit declares it in place (P-88 (1); the
// ledger's `## Owed` asks for the mirror row).  C's `float` argument and result are upgraded to
// `double` by the prototype and converted back at each assignment; the port spells both
// conversions.
// C: double round (double);
#[cfg (feature = "have_ntgui")]
unsafe extern "C"
{
  pub fn round (x: c_double) -> c_double;
}

/// C 37's `<math.h>` chain: `<float.h>`'s `FLT_MAX`, the largest finite `float` (C 884-885's two
/// sentinels for the geometry sink's box).  C's macro is `3.402823466e+38F`; the port spells the
/// same value as Rust's own bound for the 32-bit IEEE type.
/// C: <float.h>: #define FLT_MAX 3.402823466e+38F
#[cfg (feature = "have_ntgui")]
pub const FLT_MAX: c_float = c_float::MAX;

/* ----------------------------------------------------------------------------------------------
   C 38: `#include <windows.h>`.

   No file in the reference tree carries `<windows.h>` (P-88 (1): the declaration site is C's own
   `#include` line, and no file is added under `src/include/`), so the names *this file* and
   the four mirrors it includes take from it stand here, each with the SDK's own text above it.
   The spellings repeat `src/w32term.rs`'s and `src/w32font.rs`'s sections (`DWORD` is the 32-bit
   Windows unsigned whatever this target's `c_long` is, `HANDLE`/`HDC`/`HFONT`/`HMODULE` are
   `void *`, `BOOL`/`WINBOOL` are C's `int`, `WINAPI`/`STDMETHODCALLTYPE` are `extern "system"`), so
   that a Windows translation unit sees one set of names.

   **No measurement was possible**: this machine has no `<windows.h>` (see the file note), so every
   scalar is spelled from PORTING.md 1.1's fixed Windows widths and every record below is
   **UNVERIFIED** - its size, offsets and values are the documented ones and the layout assertions
   are owed (the ledger's `## Owed`).
   ---------------------------------------------------------------------------------------------- */

/// C 38's `<windows.h>`: the scalar types (`windef.h`/`winnt.h`).  C's `<winnt.h>` declares
/// `INT16`/`UINT16`/`INT32`/`UINT32` by their widths and `FLOAT` is the 32-bit IEEE type;
/// `WINBOOL` is `windef.h`'s `int` spelling that this file's own `DWRITE_GLYPH_RUN` uses (C 184).
/// C: typedef unsigned char BYTE; typedef unsigned short WORD; typedef unsigned short WCHAR;
/// C: typedef unsigned long DWORD; typedef long LONG; typedef unsigned long ULONG;
/// C: typedef unsigned int UINT; typedef int BOOL; typedef int WINBOOL; typedef long HRESULT;
/// C: typedef signed short INT16; typedef unsigned short UINT16; typedef signed int INT32;
/// C: typedef unsigned int UINT32; typedef float FLOAT;
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
#[cfg (feature = "have_ntgui")]
pub type WINBOOL = c_int;
/// `HRESULT` is `LONG` in the SDK (`winerror.h`'s `typedef long HRESULT;`), which is the width
/// every `IDWrite*`/`ID2D1*` entry point in this file returns and that `SUCCEEDED`/`FAILED` below
/// test.  C's `VOID` (`winnt.h`'s `typedef void VOID;`) has no Rust item: a `void`-returning
/// function pointer is `unsafe extern "system" fn (…)` with `()` for its result, the
/// `include/w32term.rs` reading, so `VOID` appears only in the `/// C:` lines below.
#[cfg (feature = "have_ntgui")]
pub type HRESULT = LONG;
#[cfg (feature = "have_ntgui")]
pub type INT16 = i16;
#[cfg (feature = "have_ntgui")]
pub type UINT16 = u16;
#[cfg (feature = "have_ntgui")]
pub type INT32 = i32;
#[cfg (feature = "have_ntgui")]
pub type UINT32 = u32;
#[cfg (feature = "have_ntgui")]
pub type FLOAT = c_float;
#[cfg (feature = "have_ntgui")]
pub type VOID = core::ffi::c_void;

/// C 38's `<windows.h>`: the handle names.  Every one is `void *` at the ABI (`HGDIOBJ` is the GDI
/// object handle `SelectObject` returns and takes, so `HFONT` is an alias of it and C's implicit
/// conversions at C 747/752 need no cast), and C's `HWND`/`HCURSOR`/`HPALETTE` are the pointer
/// spellings `include/w32term.rs` gives them.
/// C: typedef void *HANDLE; typedef void *HDC; typedef void *HGDIOBJ; typedef HGDIOBJ HFONT;
/// C: typedef void *HMODULE; typedef void *LPVOID; typedef BYTE *LPBYTE; typedef void *HWND;
/// C: typedef void *HCURSOR; typedef void *HPALETTE; typedef void *HINSTANCE;
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
pub type LPBYTE = *mut u8;
#[cfg (feature = "have_ntgui")]
pub type HWND = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HCURSOR = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HPALETTE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HINSTANCE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type COLORREF = DWORD;
#[cfg (feature = "have_ntgui")]
pub type LPARAM = isize;

/// C 38's `<windows.h>`: `windef.h`'s `FALSE`/`TRUE`.
/// C: #define FALSE 0
/// C: #define TRUE 1
#[cfg (feature = "have_ntgui")]
pub const FALSE: BOOL = 0;
#[cfg (feature = "have_ntgui")]
pub const TRUE: BOOL = 1;

/// C 38's `<windows.h>`: the return-value pair the `verify_hr` helper (C 697) tests every
/// DirectWrite `HRESULT` with, and `winerror.h`'s four statuses this file compares against
/// (C 599's `E_NOINTERFACE`, C 683's `S_OK`, C 881's `E_INVALIDARG`).
/// C: #define SUCCEEDED(hr) (((HRESULT) (hr)) >= 0)
/// C: #define FAILED(hr) (((HRESULT) (hr)) < 0)
/// C: #define S_OK ((HRESULT) 0L)
/// C: #define E_NOINTERFACE _HRESULT_TYPEDEF_ (0x80004002L)
/// C: #define E_INVALIDARG _HRESULT_TYPEDEF_ (0x80070057L)
/// C: #define _HRESULT_TYPEDEF_(hr) ((HRESULT) (hr))
macro_rules! SUCCEEDED { ($hr:expr) => { (($hr) as HRESULT) >= 0 }; }
macro_rules! FAILED { ($hr:expr) => { (($hr) as HRESULT) < 0 }; }
/// C's `_HRESULT_TYPEDEF_` is the SDK's cast helper this file's own `DWRITE_E_NOCOLOR` (C 61)
/// expands; the port keeps C's token so the macro's body compares (`HRESULT` is the frozen
/// `LONG`, PORTING.md 1.1).
#[cfg (feature = "have_ntgui")]
macro_rules! _HRESULT_TYPEDEF_ { ($hr:expr) => { $hr as HRESULT }; }
#[cfg (feature = "have_ntgui")]
pub const S_OK: HRESULT = 0;
#[cfg (feature = "have_ntgui")]
pub const E_NOINTERFACE: HRESULT = 0x80004002u32 as HRESULT;
#[cfg (feature = "have_ntgui")]
pub const E_INVALIDARG: HRESULT = 0x80070057u32 as HRESULT;

/// C 38's `<windows.h>`: `wingdi.h`'s `RGB` macro, used once (C 1302-1304) to turn the three
/// colour channels of a colour-glyph layer into a `COLORREF`.  C's `RGB(r,g,b)` is
/// `((COLORREF)(((BYTE)(r))|((WORD)((BYTE)(g))<<8)|(((DWORD)(BYTE)(b))<<16)))`; the port keeps the
/// three tokens and the same arithmetic.
/// C: #define RGB(r,g,b) ((COLORREF)(((BYTE)(r))|((WORD)((BYTE)(g))<<8)|(((DWORD)(BYTE)(b))<<16)))
#[cfg (feature = "have_ntgui")]
macro_rules! RGB {
  ($r:expr, $g:expr, $b:expr) => {
    ((($r) as BYTE as u32) | ((($g) as BYTE as WORD as u32) << 8)
     | ((($b) as BYTE as u32) << 16)) as COLORREF
  };
}

/// C 38's `<windows.h>`: `windef.h`'s two geometry records.  `SIZE` is the pair
/// `get_bitmap_render_target` (C 969) keeps as its static cache; `RECT` is the out parameter of
/// `DrawGlyphRun` (C 331/1305).
/// C: typedef struct tagSIZE { LONG cx; LONG cy; } SIZE;
/// C: typedef struct tagRECT { LONG left; LONG top; LONG right; LONG bottom; } RECT;
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

/// C 38's `<windows.h>`: `tagLOGFONTW` (`wingdi.h`), the record `get_font_face` (C 727) fills with
/// `GetObjectW` and reads `lfHeight` from (C 785).  `LOGFONTA`/`LOGFONT` (the narrow variant) is
/// what `include/w32font.rs` 176 spells for `uniscribe_check_otf`; this file's own `GetObjectW`
/// call is the wide one, so the name here is `LOGFONTW`.
/// C: typedef struct tagLOGFONTW { LONG lfHeight; LONG lfWidth; LONG lfEscapement;
/// C:   LONG lfOrientation; LONG lfWeight; BYTE lfItalic; BYTE lfUnderline; BYTE lfStrikeOut;
/// C:   BYTE lfCharSet; BYTE lfOutPrecision; BYTE lfClipPrecision; BYTE lfQuality;
/// C:   BYTE lfPitchAndFamily; WCHAR lfFaceName[LF_FACESIZE]; } LOGFONTW;
#[cfg (feature = "have_ntgui")]
pub const LF_FACESIZE: c_int = 32;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct LOGFONTW
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
  pub lfFaceName: [WCHAR; LF_FACESIZE as usize],
}

/// C 38's `<windows.h>`: `tagLOGFONTA`, the narrow `LOGFONT` the mirrors name
/// (`include/w32font.rs` 176's `uniscribe_check_otf (LOGFONT *font, …)`, and `w32font.c` 958's
/// `lfFaceName[0] == '\0'` / 1126's `build_string (logfont.lfFaceName)` - C's `UNICODE` is not
/// defined for these translation units, so `LOGFONT` is the `A` variant, the `src/w32uniscribe.rs`
/// reading).  This file's own `GetObjectW` call is the wide one and uses `LOGFONTW` above.
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

/// C 38's `<windows.h>`: the four entry points this file calls.  `GetObjectW` fills the
/// `LOGFONTW`, `SelectObject` swaps the font in and out of the frame's DC, `GetFontData` reads the
/// font's `maxp` table (C 761) and `GetProcAddress`-style loading is `w32common.h`'s
/// (`include/w32common.rs`); `BitBlt` copies the background in and the rendered text out
/// (C 1226/1312).  `HGDIOBJ` is what `SelectObject` returns, so the `HFONT` C stores it in needs no
/// cast.
/// C: int GetObjectW (HANDLE h, int c, LPVOID pv);
/// C: HGDIOBJ SelectObject (HDC hdc, HGDIOBJ h);
/// C: DWORD GetFontData (HDC hdc, DWORD dwTable, DWORD dwOffset, LPVOID pvBuffer, DWORD cbData);
/// C: BOOL BitBlt (HDC hdcDest, int x, int y, int cx, int cy, HDC hdcSrc, int x1, int y1,
/// C:             DWORD rop);
#[cfg (feature = "have_ntgui")]
pub const SRCCOPY: DWORD = 0x00CC0020;
unsafe extern "C"
{
  #[cfg (feature = "have_ntgui")]
  pub fn GetObjectW (h: HANDLE, c: c_int, pv: LPVOID) -> c_int;
  #[cfg (feature = "have_ntgui")]
  pub fn SelectObject (hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
  #[cfg (feature = "have_ntgui")]
  pub fn GetFontData (hdc: HDC, dwTable: DWORD, dwOffset: DWORD, pvBuffer: LPVOID,
                      cbData: DWORD) -> DWORD;
  #[cfg (feature = "have_ntgui")]
  pub fn BitBlt (hdcDest: HDC, x: c_int, y: c_int, cx: c_int, cy: c_int, hdcSrc: HDC,
                 x1: c_int, y1: c_int, rop: DWORD) -> BOOL;
  /// C 1026/1040's pair: `w32_initialize_direct_write` loads `dwrite.dll` through `LoadLibrary`
  /// and drops it with `FreeLibrary`.  Both are `libloaderapi.h`'s (`<windows.h>`'s own chain).
  /// C: HMODULE LoadLibrary (LPCSTR lpLibFileName);
  /// C: BOOL FreeLibrary (HMODULE hLibModule);
  #[cfg (feature = "have_ntgui")]
  pub fn LoadLibrary (lpLibFileName: *const c_char) -> HMODULE;
  #[cfg (feature = "have_ntgui")]
  pub fn FreeLibrary (hLibModule: HMODULE) -> BOOL;
}

/* C 40-42: `#if !defined MINGW_W64 && !defined CYGWIN` / `# define INITGUID` / `#endif`.  Both
   arms are carried; the feature names the non-frozen side (`mingw_w64`/`cygwin`, PORTING.md P-51's
   convention, the `src/w32fns.rs` 99-106 shape).  C's `INITGUID` is a marker with no value, so the
   port's answer is a unit-typed constant (the `include/w32gui.rs` include-guard spelling).  */
/// C: #if !defined MINGW_W64 && !defined CYGWIN
/// C: # define INITGUID
/// C: #endif
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64"), not (feature = "cygwin")))]
pub const INITGUID: () = ();

/* C 43-45: the COM/GUID headers.  None of them has a file in the reference tree, so the names this
   file takes from them stand here (P-88 (1)).  `DEFINE_GUID` is C 165-170's expansion - the
   `EXTERN_C const GUID DECLSPEC_SELECTANY name = { … }` the SDK's `<guiddef.h>` writes - and the
   three records it names are below; `interface` is `<objbase.h>`'s `struct`, `CONST_VTBL` its
   `const`, and `BEGIN_INTERFACE`/`END_INTERFACE` are empty markers, so nothing of the three
   appears in the port's items.  */

/// C 43-44's `<initguid.h>`/`<ole2.h>`: the GUID record and the interface markers.
/// C: <guiddef.h>: typedef struct _GUID { unsigned long Data1; unsigned short Data2;
/// C:   unsigned short Data3; unsigned char Data4[8]; } GUID; typedef GUID IID; typedef const IID *REFIID;
/// C: <objbase.h>: #define interface struct
/// C: <objbase.h>: #define BEGIN_INTERFACE /* nothing */
/// C: <objbase.h>: #define END_INTERFACE /* nothing */
/// C: <objbase.h>: #ifdef __cplusplus / #define CONST_VTBL const / #else / #define CONST_VTBL const / #endif
/// C: <guiddef.h>: EXTERN_C const GUID DECLSPEC_SELECTANY name = { … }
/// C: #define DEFINE_GUID(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8) … (C 165-170 expands it)
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID
{
  pub Data1: c_ulong,
  pub Data2: c_ushort,
  pub Data3: c_ushort,
  pub Data4: [u8; 8],
}
#[cfg (feature = "have_ntgui")]
pub type IID = GUID;
#[cfg (feature = "have_ntgui")]
pub type REFIID = *const IID;

/// C 45's `<unknwn.h>`: the base interface the three vtable slots of C 567 and the file's own
/// `IUnknownVtbl`-less sink share.  C's `STDMETHODCALLTYPE` is `__stdcall`, which Rust spells
/// `extern "system"`; the three signatures are the SDK's own.
/// C: <unknwn.h>: struct IUnknown { CONST_VTBL IUnknownVtbl *lpVtbl; };
/// C: <unknwn.h>: HRESULT (STDMETHODCALLTYPE *QueryInterface) (IUnknown *This, REFIID riid,
/// C:   void **ppvObject); ULONG (STDMETHODCALLTYPE *AddRef) (IUnknown *This);
/// C: ULONG (STDMETHODCALLTYPE *Release) (IUnknown *This);
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct IUnknown
{
  pub lpVtbl: *const IUnknownVtbl,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct IUnknownVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IUnknown, riid: REFIID,
                                                        ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IUnknown) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IUnknown) -> ULONG>,
}

/* C 47-50: the reference tree's own headers, each its mirror; all inside the file gate (P-86 (8)).
   `include/lisp.rs` is not named by C 47-50 directly but is the chain every one of the four needs
   (`Lisp_Object`, `eassert`, `SAFE_ALLOCA`, `defsubr`), the way `src/w32font.rs` 694 includes it;
   `include/conf_post.rs` is config.h's AH_BOTTOM (`DebPrint`, C 701).  */
#[cfg (feature = "have_ntgui")]
include!("../lib/include/stdlib.rs");          // C 43-45's chain: `<stdlib.h>` (none used here)
#[cfg (feature = "have_ntgui")]
include!("include/conf_post.rs");              // config.h's AH_BOTTOM: `DebPrint` (C 701)
#[cfg (feature = "have_ntgui")]
include!("include/lisp.rs");                   // C 47's chain: `Lisp_Object`, `eassert`, `SAFE_ALLOCA`
#[cfg (feature = "have_ntgui")]
include!("include/frame.rs");                  // C 47: `#include "frame.h"` (`SELECTED_FRAME`, `FRAME_LINE_HEIGHT`)
#[cfg (feature = "have_ntgui")]
include!("include/w32font.rs");                // C 48: `#include "w32font.h"` (`uniscribe_font_info`)
#[cfg (feature = "have_ntgui")]
include!("include/w32common.rs");              // C 49: `#include "w32common.h"` (`get_proc_addr`)
#[cfg (feature = "have_ntgui")]
include!("include/w32term.rs");                // C 50: `#include "w32term.h"` (`get_frame_dc`, `w32_inhibit_dwrite`)

/* C 52-548: `#ifndef MINGW_W64` … `#else` … `#endif`.  The first arm (C 53-544) is the file's
   own set of DirectWrite/D2D declarations, written out because mingw.org's MinGW has no
   `<dwrite_3.h>`; the second (C 546-547) is the two SDK headers the MINGW_W64 build gets them
   from.  Both arms are carried (`not (feature = "mingw_w64")` / `feature = "mingw_w64"`), and the
   second arm's two files have no mirror in this tree, so C's own lines stand as the section.  */
/// C: #ifndef MINGW_W64

/// C 54-57: "The following definitions would be included from dwrite_3.h, but it is not available
/// when building with mingw.org's MinGW.  Methods that we don't use are declared with the
/// EMACS_DWRITE_UNUSED macro, to avoid bringing in more types that would need to be declared."
// C 54-57 is that note; carried here as C's own words.
/// C 59: `#define EMACS_DWRITE_UNUSED(name) void (STDMETHODCALLTYPE *name) (void)`
/// C's `STDMETHODCALLTYPE` is `__stdcall`, which Rust spells `extern "system"`; the expansion is a
/// *struct field*, and Rust's `macro_rules!` cannot expand to one (`macros cannot expand to struct
/// fields`), so every site below writes the field out and this macro stands as the C body's answer
/// (`docs/ports/macro-body-notes.tsv` carries the one token, `stdmethodcalltype`, the Rust body
/// cannot name - the ledger's Deviations 5).  The parameter list `(void)` is the nullary
/// `unsafe extern "system" fn ()` the `src/w32home`-era sections use.
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
macro_rules! EMACS_DWRITE_UNUSED { ($name:ident) => { Option<unsafe extern "system" fn ()> }; }

/// C 61: `#define DWRITE_E_NOCOLOR _HRESULT_TYPEDEF_(0x8898500CL)`.  C's `L` suffix is part of the
/// literal, not the value, so the port spells the same number with `u32` (the `include/w32term.rs`
/// `E_OUTOFMEMORY` reading).  Matched at C 1283.
/// C: #define DWRITE_E_NOCOLOR _HRESULT_TYPEDEF_(0x8898500CL)
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
macro_rules! DWRITE_E_NOCOLOR { () => { _HRESULT_TYPEDEF_ (0x8898500Cu32) }; }

/// C 63-67: `typedef enum DWRITE_PIXEL_GEOMETRY { … } DWRITE_PIXEL_GEOMETRY;`.  C's enum ABI is
/// `int`, so the port keeps `#[repr(C)]` (PORTING.md 1.3) and C's explicit values.
/// C: DWRITE_PIXEL_GEOMETRY_FLAT = 0, DWRITE_PIXEL_GEOMETRY_RGB = 1, DWRITE_PIXEL_GEOMETRY_BGR = 2
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DWRITE_PIXEL_GEOMETRY
{
  DWRITE_PIXEL_GEOMETRY_FLAT = 0,
  DWRITE_PIXEL_GEOMETRY_RGB = 1,
  DWRITE_PIXEL_GEOMETRY_BGR = 2,
}

/// C 69-77: `DWRITE_RENDERING_MODE`.
/// C: DWRITE_RENDERING_MODE_DEFAULT = 0, DWRITE_RENDERING_MODE_ALIASED = 1,
/// C: DWRITE_RENDERING_MODE_GDI_CLASSIC = 2, DWRITE_RENDERING_MODE_GDI_NATURAL = 3,
/// C: DWRITE_RENDERING_MODE_NATURAL = 4, DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC = 5,
/// C: DWRITE_RENDERING_MODE_OUTLINE = 6
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DWRITE_RENDERING_MODE
{
  DWRITE_RENDERING_MODE_DEFAULT = 0,
  DWRITE_RENDERING_MODE_ALIASED = 1,
  DWRITE_RENDERING_MODE_GDI_CLASSIC = 2,
  DWRITE_RENDERING_MODE_GDI_NATURAL = 3,
  DWRITE_RENDERING_MODE_NATURAL = 4,
  DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC = 5,
  DWRITE_RENDERING_MODE_OUTLINE = 6,
}

/// C 79-83: `DWRITE_MEASURING_MODE`.  C 558's `MEASURING_MODE` is `DWRITE_MEASURING_MODE_NATURAL`.
/// C: DWRITE_MEASURING_MODE_NATURAL = 0, DWRITE_MEASURING_MODE_GDI_CLASSIC = 1,
/// C: DWRITE_MEASURING_MODE_GDI_NATURAL = 2
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DWRITE_MEASURING_MODE
{
  DWRITE_MEASURING_MODE_NATURAL = 0,
  DWRITE_MEASURING_MODE_GDI_CLASSIC = 1,
  DWRITE_MEASURING_MODE_GDI_NATURAL = 2,
}

/// C 85-88: `DWRITE_TEXT_ANTIALIAS_MODE`.  C 560's `ANTIALIAS_MODE` is the CLEARTYPE member.
/// C: DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE = 0, DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE = 1
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DWRITE_TEXT_ANTIALIAS_MODE
{
  DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE = 0,
  DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE = 1,
}

/// C 90-93: `DWRITE_FACTORY_TYPE`, the argument `DWriteCreateFactory` (C 1075) takes.
/// C: DWRITE_FACTORY_TYPE_SHARED = 0, DWRITE_FACTORY_TYPE_ISOLATED = 1
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DWRITE_FACTORY_TYPE
{
  DWRITE_FACTORY_TYPE_SHARED = 0,
  DWRITE_FACTORY_TYPE_ISOLATED = 1,
}

/// C 95-106: `DWRITE_FONT_METRICS`, the record `GetMetrics` (C 818) fills and
/// `designUnitsPerEm` (C 851) is read from.
/// C: typedef struct DWRITE_FONT_METRICS { UINT16 designUnitsPerEm; UINT16 ascent;
/// C:   UINT16 descent; INT16 lineGap; UINT16 capHeight; UINT16 xHeight;
/// C:   INT16 underlinePosition; UINT16 underlineThickness; INT16 strikethroughPosition;
/// C:   UINT16 strikethroughThickness; } DWRITE_FONT_METRICS;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct DWRITE_FONT_METRICS
{
  pub designUnitsPerEm: UINT16,
  pub ascent: UINT16,
  pub descent: UINT16,
  pub lineGap: INT16,
  pub capHeight: UINT16,
  pub xHeight: UINT16,
  pub underlinePosition: INT16,
  pub underlineThickness: UINT16,
  pub strikethroughPosition: INT16,
  pub strikethroughThickness: UINT16,
}

/// C 108-116: `DWRITE_GLYPH_METRICS`, the per-glyph record
/// `GetGdiCompatibleGlyphMetrics` (C 827) writes and C 855-871 read.
/// C: typedef struct DWRITE_GLYPH_METRICS { INT32 leftSideBearing; UINT32 advanceWidth;
/// C:   INT32 rightSideBearing; INT32 topSideBearing; UINT32 advanceHeight;
/// C:   INT32 bottomSideBearing; INT32 verticalOriginY; } DWRITE_GLYPH_METRICS;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct DWRITE_GLYPH_METRICS
{
  pub leftSideBearing: INT32,
  pub advanceWidth: UINT32,
  pub rightSideBearing: INT32,
  pub topSideBearing: INT32,
  pub advanceHeight: UINT32,
  pub bottomSideBearing: INT32,
  pub verticalOriginY: INT32,
}

/// C 118-121: `D2D1_POINT_2F`, the point `BeginFigure`/`AddLines` (C 638/652) receive.
/// C: typedef struct D2D1_POINT_2F { float x; float y; } D2D1_POINT_2F;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D1_POINT_2F
{
  pub x: f32,
  pub y: f32,
}

/// C 123-127: `D2D1_BEZIER_SEGMENT`, the three points `AddBeziers` (C 616) walks.
/// C: typedef struct D2D1_BEZIER_SEGMENT { D2D1_POINT_2F point1; D2D1_POINT_2F point2;
/// C:   D2D1_POINT_2F point3; } D2D1_BEZIER_SEGMENT;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D1_BEZIER_SEGMENT
{
  pub point1: D2D1_POINT_2F,
  pub point2: D2D1_POINT_2F,
  pub point3: D2D1_POINT_2F,
}

/// C 129-133: `D2D1_FILL_MODE`, the argument `SetFillMode` (C 676) takes.
/// C: D2D1_FILL_MODE_ALTERNATE = 0, D2D1_FILL_MODE_WINDING = 1, D2D1_FILL_MODE_FORCE_DWORD = 0xffffffff
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum D2D1_FILL_MODE
{
  D2D1_FILL_MODE_ALTERNATE = 0,
  D2D1_FILL_MODE_WINDING = 1,
  D2D1_FILL_MODE_FORCE_DWORD = 0xffffffff,
}

/// C 135-140: `D2D1_PATH_SEGMENT`, the argument `SetSegmentFlags` (C 682) takes.
/// C: D2D1_PATH_SEGMENT_NONE = 0x00000000, D2D1_PATH_SEGMENT_FORCE_UNSTROKED = 0x00000001,
/// C: D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN = 0x00000002, D2D1_PATH_SEGMENT_FORCE_DWORD = 0xffffffff
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum D2D1_PATH_SEGMENT
{
  D2D1_PATH_SEGMENT_NONE = 0x00000000,
  D2D1_PATH_SEGMENT_FORCE_UNSTROKED = 0x00000001,
  D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN = 0x00000002,
  D2D1_PATH_SEGMENT_FORCE_DWORD = 0xffffffff,
}

/// C 142-146: `D2D1_FIGURE_BEGIN`, the argument `BeginFigure` (C 652) takes.
/// C: D2D1_FIGURE_BEGIN_FILLED = 0, D2D1_FIGURE_BEGIN_HOLLOW = 1, D2D1_FIGURE_BEGIN_FORCE_DWORD = 0xffffffff
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum D2D1_FIGURE_BEGIN
{
  D2D1_FIGURE_BEGIN_FILLED = 0,
  D2D1_FIGURE_BEGIN_HOLLOW = 1,
  D2D1_FIGURE_BEGIN_FORCE_DWORD = 0xffffffff,
}

/// C 148-152: `D2D1_FIGURE_END`, the argument `EndFigure` (C 665) takes.
/// C: D2D1_FIGURE_END_OPEN = 0, D2D1_FIGURE_END_CLOSED = 1, D2D1_FIGURE_END_FORCE_DWORD = 0xffffffff
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum D2D1_FIGURE_END
{
  D2D1_FIGURE_END_OPEN = 0,
  D2D1_FIGURE_END_CLOSED = 1,
  D2D1_FIGURE_END_FORCE_DWORD = 0xffffffff,
}

/* C 154-163: the ten `typedef interface X X;` forward declarations.  `interface` is
   `<objbase.h>`'s `struct`, so each line is C's `typedef struct X X;` - a tag declaration, which
   Rust has no item for: the ten names are the records C 258-543 define, and the port's definitions
   below are the one item per name (Rust resolves them wherever they are written, so C's forward
   declarations have no counterpart).  C's lines are kept here so a reader sees the order.  */
/// C: typedef interface IDWriteRenderingParams IDWriteRenderingParams;
/// C: typedef interface IDWriteFont IDWriteFont;
/// C: typedef interface IDWriteGdiInterop IDWriteGdiInterop;
/// C: typedef interface IDWriteFactory IDWriteFactory;
/// C: typedef interface IDWriteFactory2 IDWriteFactory2;
/// C: typedef interface IDWriteFontFace IDWriteFontFace;
/// C: typedef interface IDWriteBitmapRenderTarget IDWriteBitmapRenderTarget;
/// C: typedef interface IDWriteBitmapRenderTarget1 IDWriteBitmapRenderTarget1;
/// C: typedef interface IDWriteColorGlyphRunEnumerator IDWriteColorGlyphRunEnumerator;
/// C: typedef interface ID2D1SimplifiedGeometrySink ID2D1SimplifiedGeometrySink;

/* C 165-170: the three `DEFINE_GUID` definitions.  C's `<initguid.h>` makes them definitions
   (`EXTERN_C const GUID DECLSPEC_SELECTANY name = { … }`), so each is a non-`static` object and
   carries `#[unsafe(no_mangle)]` (PORTING.md 0.2).  They are declared for the same reason C's are:
   `QueryInterface` (C 1081/1091) passes their addresses.  */
/// C: DEFINE_GUID (IID_IDWriteBitmapRenderTarget1, 0x791e8298, 0x3ef3, 0x4230, 0x98, 0x80, 0xc9,
/// C:   0xbd, 0xec, 0xc4, 0x20, 0x64);
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[unsafe(no_mangle)]
pub static IID_IDWriteBitmapRenderTarget1: GUID = GUID {
  Data1: 0x791e8298, Data2: 0x3ef3, Data3: 0x4230,
  Data4: [0x98, 0x80, 0xc9, 0xbd, 0xec, 0xc4, 0x20, 0x64],
};
/// C: DEFINE_GUID (IID_IDWriteFactory2, 0x0439fc60, 0xca44, 0x4994, 0x8d, 0xee, 0x3a, 0x9a,
/// C:   0xf7, 0xb7, 0x32, 0xec);
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[unsafe(no_mangle)]
pub static IID_IDWriteFactory2: GUID = GUID {
  Data1: 0x0439fc60, Data2: 0xca44, Data3: 0x4994,
  Data4: [0x8d, 0xee, 0x3a, 0x9a, 0xf7, 0xb7, 0x32, 0xec],
};
/// C: DEFINE_GUID (IID_IDWriteFactory, 0xb859ee5a, 0xd838, 0x4b5b, 0xa2, 0xe8, 0x1a, 0xdc,
/// C:   0x7d, 0x93, 0xdb, 0x48);
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[unsafe(no_mangle)]
pub static IID_IDWriteFactory: GUID = GUID {
  Data1: 0xb859ee5a, Data2: 0xd838, Data3: 0x4b5b,
  Data4: [0xa2, 0xe8, 0x1a, 0xdc, 0x7d, 0x93, 0xdb, 0x48],
};

/// C 172-175: `DWRITE_GLYPH_OFFSET`, the optional per-glyph offset array `GetGlyphRunOutline`
/// (C 235) takes (`DWRITE_GLYPH_RUN.glyphOffsets`, C 183).
/// C: typedef struct DWRITE_GLYPH_OFFSET { FLOAT advanceOffset; FLOAT ascenderOffset; }
/// C: DWRITE_GLYPH_OFFSET;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DWRITE_GLYPH_OFFSET
{
  pub advanceOffset: FLOAT,
  pub ascenderOffset: FLOAT,
}

/// C 177-186: `DWRITE_GLYPH_RUN`, the run `TranslateColorGlyphRun`/`DrawGlyphRun` (C 1240/1272)
/// take and that `w32_dwrite_draw` fills member by member (C 1262-1270).
/// C: typedef struct DWRITE_GLYPH_RUN { IDWriteFontFace *fontFace; FLOAT fontEmSize;
/// C:   UINT32 glyphCount; const UINT16 *glyphIndices; const FLOAT *glyphAdvances;
/// C:   const DWRITE_GLYPH_OFFSET *glyphOffsets; WINBOOL isSideways; UINT32 bidiLevel; }
/// C: DWRITE_GLYPH_RUN;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct DWRITE_GLYPH_RUN
{
  pub fontFace: *mut IDWriteFontFace,
  pub fontEmSize: FLOAT,
  pub glyphCount: UINT32,
  pub glyphIndices: *const UINT16,
  pub glyphAdvances: *const FLOAT,
  pub glyphOffsets: *const DWRITE_GLYPH_OFFSET,
  pub isSideways: WINBOOL,
  pub bidiLevel: UINT32,
}

/// C 188-193: `_D3DCOLORVALUE`, the colour record C 195's `DWRITE_COLOR_F` aliases.  C's tag is
/// `_D3DCOLORVALUE`, so the port keeps it and adds C's own typedef as the alias below.
/// C: typedef struct _D3DCOLORVALUE { float r; float g; float b; float a; } D3DCOLORVALUE;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _D3DCOLORVALUE
{
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}
/// C 195: `typedef D3DCOLORVALUE DWRITE_COLOR_F;`
/// C: typedef D3DCOLORVALUE DWRITE_COLOR_F;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
pub type DWRITE_COLOR_F = _D3DCOLORVALUE;

/// C 197-204: `DWRITE_COLOR_GLYPH_RUN`, the per-layer record `GetCurrentRun` (C 1300) hands back
/// and C 1301-1307 draw.
/// C: typedef struct DWRITE_COLOR_GLYPH_RUN { DWRITE_GLYPH_RUN glyphRun;
/// C:   void *glyphRunDescription; FLOAT baselineOriginX; FLOAT baselineOriginY;
/// C:   DWRITE_COLOR_F runColor; UINT16 paletteIndex; } DWRITE_COLOR_GLYPH_RUN;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct DWRITE_COLOR_GLYPH_RUN
{
  pub glyphRun: DWRITE_GLYPH_RUN,
  pub glyphRunDescription: *mut c_void,
  pub baselineOriginX: FLOAT,
  pub baselineOriginY: FLOAT,
  pub runColor: DWRITE_COLOR_F,
  pub paletteIndex: UINT16,
}

/* C 206-543: the ten vtable records and their ten interface records.  C's `BEGIN_INTERFACE` and
   `END_INTERFACE` are empty markers (`<objbase.h>`), so only the methods between them become
   fields; `CONST_VTBL` is `const`, so the `lpVtbl` member is `*const`; and every
   `EMACS_DWRITE_UNUSED (name);` site is the `Option<unsafe extern "system" fn ()>` field the macro
   above answers for.  The parameter names are C's.  */

/// C 206-256: `IDWriteFontFaceVtbl`.  `GetMetrics`, `GetGlyphCount`, `GetGlyphIndices` and
/// `GetGdiCompatibleGlyphMetrics` are the four the port calls (C 818/827/931/1248).
/// C: typedef struct IDWriteFontFaceVtbl { BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IDWriteFontFace *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IDWriteFontFace *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IDWriteFontFace *This);
/// C:   EMACS_DWRITE_UNUSED (GetType); EMACS_DWRITE_UNUSED (GetFiles); EMACS_DWRITE_UNUSED (GetIndex);
/// C:   EMACS_DWRITE_UNUSED (GetSimulations); EMACS_DWRITE_UNUSED (IsSymbolFont);
/// C:   void (STDMETHODCALLTYPE *GetMetrics) (IDWriteFontFace *This, DWRITE_FONT_METRICS *metrics);
/// C:   UINT16 (STDMETHODCALLTYPE *GetGlyphCount) (IDWriteFontFace *This);
/// C:   EMACS_DWRITE_UNUSED (GetDesignGlyphMetrics);
/// C:   HRESULT (STDMETHODCALLTYPE *GetGlyphIndices) (IDWriteFontFace *This, const UINT32 *codepoints,
/// C:     UINT32 count, UINT16 *glyph_indices);
/// C:   EMACS_DWRITE_UNUSED (TryGetFontTable); EMACS_DWRITE_UNUSED (ReleaseFontTable);
/// C:   HRESULT (STDMETHODCALLTYPE *GetGlyphRunOutline) (IDWriteFontFace *This, FLOAT emSize,
/// C:     const UINT16 *glyph_indices, const DWRITE_GLYPH_OFFSET *glyph_offsets,
/// C:     const FLOAT *glyph_advances, UINT32 glyph_count, WINBOOL is_sideways,
/// C:     WINBOOL is_right_to_left, ID2D1SimplifiedGeometrySink *geometry_sink);
/// C:   EMACS_DWRITE_UNUSED (GetRecommendedRenderingMode); EMACS_DWRITE_UNUSED (GetGdiCompatibleMetrics);
/// C:   HRESULT (STDMETHODCALLTYPE *GetGdiCompatibleGlyphMetrics) (IDWriteFontFace *This, FLOAT emSize,
/// C:     FLOAT pixels_per_dip, void *transform, WINBOOL use_gdi_natural, const UINT16 *glyph_indices,
/// C:     UINT32 glyph_count, DWRITE_GLYPH_METRICS *metrics, WINBOOL is_sideways);
/// C:   END_INTERFACE } IDWriteFontFaceVtbl;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteFontFaceVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IDWriteFontFace, riid: REFIID,
                                                        ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IDWriteFontFace) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IDWriteFontFace) -> ULONG>,
  pub GetType: Option<unsafe extern "system" fn ()>,
  pub GetFiles: Option<unsafe extern "system" fn ()>,
  pub GetIndex: Option<unsafe extern "system" fn ()>,
  pub GetSimulations: Option<unsafe extern "system" fn ()>,
  pub IsSymbolFont: Option<unsafe extern "system" fn ()>,
  pub GetMetrics: Option<unsafe extern "system" fn (This: *mut IDWriteFontFace,
                                                     metrics: *mut DWRITE_FONT_METRICS)>,
  pub GetGlyphCount: Option<unsafe extern "system" fn (This: *mut IDWriteFontFace) -> UINT16>,
  pub GetDesignGlyphMetrics: Option<unsafe extern "system" fn ()>,
  pub GetGlyphIndices: Option<unsafe extern "system" fn (This: *mut IDWriteFontFace,
                                                         codepoints: *const UINT32,
                                                         count: UINT32,
                                                         glyph_indices: *mut UINT16) -> HRESULT>,
  pub TryGetFontTable: Option<unsafe extern "system" fn ()>,
  pub ReleaseFontTable: Option<unsafe extern "system" fn ()>,
  pub GetGlyphRunOutline: Option<unsafe extern "system" fn (This: *mut IDWriteFontFace,
                                                            emSize: FLOAT,
                                                            glyph_indices: *const UINT16,
                                                            glyph_offsets: *const DWRITE_GLYPH_OFFSET,
                                                            glyph_advances: *const FLOAT,
                                                            glyph_count: UINT32,
                                                            is_sideways: WINBOOL,
                                                            is_right_to_left: WINBOOL,
                                                            geometry_sink: *mut ID2D1SimplifiedGeometrySink) -> HRESULT>,
  pub GetRecommendedRenderingMode: Option<unsafe extern "system" fn ()>,
  pub GetGdiCompatibleMetrics: Option<unsafe extern "system" fn ()>,
  pub GetGdiCompatibleGlyphMetrics: Option<unsafe extern "system" fn (This: *mut IDWriteFontFace,
                                                                     emSize: FLOAT,
                                                                     pixels_per_dip: FLOAT,
                                                                     transform: *mut c_void,
                                                                     use_gdi_natural: WINBOOL,
                                                                     glyph_indices: *const UINT16,
                                                                     glyph_count: UINT32,
                                                                     metrics: *mut DWRITE_GLYPH_METRICS,
                                                                     is_sideways: WINBOOL) -> HRESULT>,
}

/// C 258-260: `interface IDWriteFontFace { CONST_VTBL IDWriteFontFaceVtbl *lpVtbl; };`
/// C: interface IDWriteFontFace { CONST_VTBL IDWriteFontFaceVtbl *lpVtbl; };
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteFontFace
{
  pub lpVtbl: *const IDWriteFontFaceVtbl,
}

/// C 262-279: `IDWriteRenderingParamsVtbl`.  The four accessors `w32_initialize_direct_write`
/// reads (C 1103/1106/1117/1122) are here.
/// C: typedef struct IDWriteRenderingParamsVtbl { BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IDWriteRenderingParams *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IDWriteRenderingParams *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IDWriteRenderingParams *This);
/// C:   FLOAT (STDMETHODCALLTYPE *GetGamma) (IDWriteRenderingParams *This);
/// C:   FLOAT (STDMETHODCALLTYPE *GetEnhancedContrast) (IDWriteRenderingParams *This);
/// C:   FLOAT (STDMETHODCALLTYPE *GetClearTypeLevel) (IDWriteRenderingParams *This);
/// C:   int (STDMETHODCALLTYPE *GetPixelGeometry) (IDWriteRenderingParams *This);
/// C:   END_INTERFACE } IDWriteRenderingParamsVtbl;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteRenderingParamsVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IDWriteRenderingParams,
                                                        riid: REFIID,
                                                        ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IDWriteRenderingParams) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IDWriteRenderingParams) -> ULONG>,
  pub GetGamma: Option<unsafe extern "system" fn (This: *mut IDWriteRenderingParams) -> FLOAT>,
  pub GetEnhancedContrast: Option<unsafe extern "system" fn (This: *mut IDWriteRenderingParams) -> FLOAT>,
  pub GetClearTypeLevel: Option<unsafe extern "system" fn (This: *mut IDWriteRenderingParams) -> FLOAT>,
  pub GetPixelGeometry: Option<unsafe extern "system" fn (This: *mut IDWriteRenderingParams) -> c_int>,
}

/// C 281-283: `interface IDWriteRenderingParams`.
/// C: interface IDWriteRenderingParams { CONST_VTBL IDWriteRenderingParamsVtbl *lpVtbl; };
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteRenderingParams
{
  pub lpVtbl: *const IDWriteRenderingParamsVtbl,
}

/// C 285-311: `IDWriteFontVtbl`.  Only `GetMetrics` and `CreateFontFace` are declared in full; the
/// rest are the `EMACS_DWRITE_UNUSED` slots.  `CreateFontFaceFromHdc` (C 396) is what this file
/// actually uses, so neither `CreateFontFace` nor `GetMetrics` is called here - they are kept
/// because C declares them.
/// C: typedef struct IDWriteFontVtbl { BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IDWriteFont *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IDWriteFont *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IDWriteFont *This);
/// C:   EMACS_DWRITE_UNUSED (GetFontFamily); EMACS_DWRITE_UNUSED (GetWeight);
/// C:   EMACS_DWRITE_UNUSED (GetStretch); EMACS_DWRITE_UNUSED (GetStyle);
/// C:   EMACS_DWRITE_UNUSED (IsSymbolFont); EMACS_DWRITE_UNUSED (GetFaceNames);
/// C:   EMACS_DWRITE_UNUSED (GetInformationalStrings); EMACS_DWRITE_UNUSED (GetSimulations);
/// C:   void (STDMETHODCALLTYPE *GetMetrics) (IDWriteFont *This, DWRITE_FONT_METRICS *metrics);
/// C:   EMACS_DWRITE_UNUSED (HasCharacter);
/// C:   HRESULT (STDMETHODCALLTYPE *CreateFontFace) (IDWriteFont *This, IDWriteFontFace **face);
/// C:   END_INTERFACE } IDWriteFontVtbl;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteFontVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IDWriteFont, riid: REFIID,
                                                        ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IDWriteFont) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IDWriteFont) -> ULONG>,
  pub GetFontFamily: Option<unsafe extern "system" fn ()>,
  pub GetWeight: Option<unsafe extern "system" fn ()>,
  pub GetStretch: Option<unsafe extern "system" fn ()>,
  pub GetStyle: Option<unsafe extern "system" fn ()>,
  pub IsSymbolFont: Option<unsafe extern "system" fn ()>,
  pub GetFaceNames: Option<unsafe extern "system" fn ()>,
  pub GetInformationalStrings: Option<unsafe extern "system" fn ()>,
  pub GetSimulations: Option<unsafe extern "system" fn ()>,
  pub GetMetrics: Option<unsafe extern "system" fn (This: *mut IDWriteFont,
                                                    metrics: *mut DWRITE_FONT_METRICS)>,
  pub HasCharacter: Option<unsafe extern "system" fn ()>,
  pub CreateFontFace: Option<unsafe extern "system" fn (This: *mut IDWriteFont,
                                                        face: *mut *mut IDWriteFontFace) -> HRESULT>,
}

/// C 313-315: `interface IDWriteFont`.
/// C: interface IDWriteFont { CONST_VTBL IDWriteFontVtbl *lpVtbl; };
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteFont
{
  pub lpVtbl: *const IDWriteFontVtbl,
}

/// C 317-347: `IDWriteBitmapRenderTargetVtbl`.  `DrawGlyphRun`, `GetMemoryDC` and
/// `SetPixelsPerDip` are the three the port calls (C 992/1225/1256).
/// C: typedef struct IDWriteBitmapRenderTargetVtbl { BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IDWriteBitmapRenderTarget *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IDWriteBitmapRenderTarget *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IDWriteBitmapRenderTarget *This);
/// C:   HRESULT (STDMETHODCALLTYPE *DrawGlyphRun) (IDWriteBitmapRenderTarget *This, FLOAT baselineOriginX,
/// C:     FLOAT baselineOriginY, DWRITE_MEASURING_MODE measuring_mode, const DWRITE_GLYPH_RUN *glyph_run,
/// C:     IDWriteRenderingParams *params, COLORREF textColor, RECT *blackbox_rect);
/// C:   HDC (STDMETHODCALLTYPE *GetMemoryDC) (IDWriteBitmapRenderTarget *This);
/// C:   EMACS_DWRITE_UNUSED (GetPixelsPerDip);
/// C:   HRESULT (STDMETHODCALLTYPE *SetPixelsPerDip) (IDWriteBitmapRenderTarget *This, FLOAT pixels_per_dip);
/// C:   EMACS_DWRITE_UNUSED (GetCurrentTransform); EMACS_DWRITE_UNUSED (SetCurrentTransform);
/// C:   EMACS_DWRITE_UNUSED (GetSize); EMACS_DWRITE_UNUSED (Resize);
/// C:   END_INTERFACE } IDWriteBitmapRenderTargetVtbl;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteBitmapRenderTargetVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IDWriteBitmapRenderTarget,
                                                        riid: REFIID,
                                                        ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IDWriteBitmapRenderTarget) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IDWriteBitmapRenderTarget) -> ULONG>,
  pub DrawGlyphRun: Option<unsafe extern "system" fn (This: *mut IDWriteBitmapRenderTarget,
                                                      baselineOriginX: FLOAT,
                                                      baselineOriginY: FLOAT,
                                                      measuring_mode: DWRITE_MEASURING_MODE,
                                                      glyph_run: *const DWRITE_GLYPH_RUN,
                                                      params: *mut IDWriteRenderingParams,
                                                      textColor: COLORREF,
                                                      blackbox_rect: *mut RECT) -> HRESULT>,
  pub GetMemoryDC: Option<unsafe extern "system" fn (This: *mut IDWriteBitmapRenderTarget) -> HDC>,
  pub GetPixelsPerDip: Option<unsafe extern "system" fn ()>,
  pub SetPixelsPerDip: Option<unsafe extern "system" fn (This: *mut IDWriteBitmapRenderTarget,
                                                         pixels_per_dip: FLOAT) -> HRESULT>,
  pub GetCurrentTransform: Option<unsafe extern "system" fn ()>,
  pub SetCurrentTransform: Option<unsafe extern "system" fn ()>,
  pub GetSize: Option<unsafe extern "system" fn ()>,
  pub Resize: Option<unsafe extern "system" fn ()>,
}

/// C 349-351: `interface IDWriteBitmapRenderTarget`.
/// C: interface IDWriteBitmapRenderTarget { CONST_VTBL IDWriteBitmapRenderTargetVtbl *lpVtbl; };
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteBitmapRenderTarget
{
  pub lpVtbl: *const IDWriteBitmapRenderTargetVtbl,
}

/// C 353-375: `IDWriteBitmapRenderTarget1Vtbl`.  `SetTextAntialiasMode` is the one member C 1002
/// calls; every other one is the `EMACS_DWRITE_UNUSED` slot, including the eight the base record
/// declares (C's own comment: the derived record repeats them).
/// C: typedef struct IDWriteBitmapRenderTarget1Vtbl { BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IDWriteBitmapRenderTarget1 *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IDWriteBitmapRenderTarget1 *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IDWriteBitmapRenderTarget1 *This);
/// C:   EMACS_DWRITE_UNUSED (DrawGlyphRun); EMACS_DWRITE_UNUSED (GetMemoryDC);
/// C:   EMACS_DWRITE_UNUSED (GetPixelsPerDip); EMACS_DWRITE_UNUSED (SetPixelsPerDip);
/// C:   EMACS_DWRITE_UNUSED (GetCurrentTransform); EMACS_DWRITE_UNUSED (SetCurrentTransform);
/// C:   EMACS_DWRITE_UNUSED (GetSize); EMACS_DWRITE_UNUSED (Resize);
/// C:   EMACS_DWRITE_UNUSED (GetTextAntialiasMode);
/// C:   HRESULT (STDMETHODCALLTYPE *SetTextAntialiasMode) (IDWriteBitmapRenderTarget1 *This,
/// C:     DWRITE_TEXT_ANTIALIAS_MODE mode);
/// C:   END_INTERFACE } IDWriteBitmapRenderTarget1Vtbl;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteBitmapRenderTarget1Vtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IDWriteBitmapRenderTarget1,
                                                        riid: REFIID,
                                                        ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IDWriteBitmapRenderTarget1) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IDWriteBitmapRenderTarget1) -> ULONG>,
  pub DrawGlyphRun: Option<unsafe extern "system" fn ()>,
  pub GetMemoryDC: Option<unsafe extern "system" fn ()>,
  pub GetPixelsPerDip: Option<unsafe extern "system" fn ()>,
  pub SetPixelsPerDip: Option<unsafe extern "system" fn ()>,
  pub GetCurrentTransform: Option<unsafe extern "system" fn ()>,
  pub SetCurrentTransform: Option<unsafe extern "system" fn ()>,
  pub GetSize: Option<unsafe extern "system" fn ()>,
  pub Resize: Option<unsafe extern "system" fn ()>,
  pub GetTextAntialiasMode: Option<unsafe extern "system" fn ()>,
  pub SetTextAntialiasMode: Option<unsafe extern "system" fn (This: *mut IDWriteBitmapRenderTarget1,
                                                              mode: DWRITE_TEXT_ANTIALIAS_MODE) -> HRESULT>,
}

/// C 377-379: `interface IDWriteBitmapRenderTarget1`.
/// C: interface IDWriteBitmapRenderTarget1 { CONST_VTBL IDWriteBitmapRenderTarget1Vtbl *lpVtbl; };
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteBitmapRenderTarget1
{
  pub lpVtbl: *const IDWriteBitmapRenderTarget1Vtbl,
}

/// C 381-402: `IDWriteGdiInteropVtbl`.  `CreateFontFaceFromHdc` and `CreateBitmapRenderTarget` are
/// the two the port calls (C 1030/985); the two `EMACS_DWRITE_UNUSED` slots sit between them in
/// C's order.
/// C: typedef struct IDWriteGdiInteropVtbl { BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IDWriteGdiInterop *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IDWriteGdiInterop *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IDWriteGdiInterop *This);
/// C:   HRESULT (STDMETHODCALLTYPE *CreateFontFromLOGFONT) (IDWriteGdiInterop *This,
/// C:     const LOGFONTW *logfont, IDWriteFont **font);
/// C:   EMACS_DWRITE_UNUSED (ConvertFontToLOGFONT); EMACS_DWRITE_UNUSED (ConvertFontFaceToLOGFONT);
/// C:   HRESULT (STDMETHODCALLTYPE *CreateFontFaceFromHdc) (IDWriteGdiInterop *This, HDC hdc,
/// C:     IDWriteFontFace **font_face);
/// C:   HRESULT (STDMETHODCALLTYPE *CreateBitmapRenderTarget) (IDWriteGdiInterop *This, HDC hdc,
/// C:     UINT32 width, UINT32 height, IDWriteBitmapRenderTarget **target);
/// C:   END_INTERFACE } IDWriteGdiInteropVtbl;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteGdiInteropVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IDWriteGdiInterop, riid: REFIID,
                                                        ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IDWriteGdiInterop) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IDWriteGdiInterop) -> ULONG>,
  pub CreateFontFromLOGFONT: Option<unsafe extern "system" fn (This: *mut IDWriteGdiInterop,
                                                                logfont: *const LOGFONTW,
                                                                font: *mut *mut IDWriteFont) -> HRESULT>,
  pub ConvertFontToLOGFONT: Option<unsafe extern "system" fn ()>,
  pub ConvertFontFaceToLOGFONT: Option<unsafe extern "system" fn ()>,
  pub CreateFontFaceFromHdc: Option<unsafe extern "system" fn (This: *mut IDWriteGdiInterop,
                                                               hdc: HDC,
                                                               font_face: *mut *mut IDWriteFontFace) -> HRESULT>,
  pub CreateBitmapRenderTarget: Option<unsafe extern "system" fn (This: *mut IDWriteGdiInterop,
                                                                  hdc: HDC,
                                                                  width: UINT32,
                                                                  height: UINT32,
                                                                  target: *mut *mut IDWriteBitmapRenderTarget) -> HRESULT>,
}

/// C 404-406: `interface IDWriteGdiInterop`.
/// C: interface IDWriteGdiInterop { CONST_VTBL IDWriteGdiInteropVtbl *lpVtbl; };
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteGdiInterop
{
  pub lpVtbl: *const IDWriteGdiInteropVtbl,
}

/// C 408-443: `IDWriteFactoryVtbl`.  `CreateRenderingParams`, `CreateCustomRenderingParams` and
/// `GetGdiInterop` are the three the port calls (C 1051/1093/1119); C's comment for the unused
/// slots is why they are `EMACS_DWRITE_UNUSED` and not spelled out.
/// C: typedef struct IDWriteFactoryVtbl { BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IDWriteFactory *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IDWriteFactory *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IDWriteFactory *This);
/// C:   EMACS_DWRITE_UNUSED (GetSystemFontCollection); EMACS_DWRITE_UNUSED (CreateCustomFontCollection);
/// C:   EMACS_DWRITE_UNUSED (RegisterFontCollectionLoader); EMACS_DWRITE_UNUSED (UnregisterFontCollectionLoader);
/// C:   EMACS_DWRITE_UNUSED (CreateFontFileReference); EMACS_DWRITE_UNUSED (CreateCustomFontFileReference);
/// C:   EMACS_DWRITE_UNUSED (CreateFontFace);
/// C:   HRESULT (STDMETHODCALLTYPE *CreateRenderingParams) (IDWriteFactory *This, IDWriteRenderingParams **params);
/// C:   EMACS_DWRITE_UNUSED (CreateMonitorRenderingParams);
/// C:   HRESULT (STDMETHODCALLTYPE *CreateCustomRenderingParams) (IDWriteFactory *This, FLOAT gamma,
/// C:     FLOAT enhancedContrast, FLOAT cleartype_level, DWRITE_PIXEL_GEOMETRY geometry,
/// C:     DWRITE_RENDERING_MODE mode, IDWriteRenderingParams **params);
/// C:   EMACS_DWRITE_UNUSED (RegisterFontFileLoader); EMACS_DWRITE_UNUSED (UnregisterFontFileLoader);
/// C:   EMACS_DWRITE_UNUSED (CreateTextFormat); EMACS_DWRITE_UNUSED (CreateTypography);
/// C:   HRESULT (STDMETHODCALLTYPE *GetGdiInterop) (IDWriteFactory *This, IDWriteGdiInterop **gdi_interop);
/// C:   EMACS_DWRITE_UNUSED (CreateTextLayout); EMACS_DWRITE_UNUSED (CreateGdiCompatibleTextLayout);
/// C:   EMACS_DWRITE_UNUSED (CreateEllipsisTrimmingSign); EMACS_DWRITE_UNUSED (CreateTextAnalyzer);
/// C:   EMACS_DWRITE_UNUSED (CreateNumberSubstitution); EMACS_DWRITE_UNUSED (CreateGlyphRunAnalysis);
/// C:   END_INTERFACE } IDWriteFactoryVtbl;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteFactoryVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IDWriteFactory, riid: REFIID,
                                                        ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IDWriteFactory) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IDWriteFactory) -> ULONG>,
  pub GetSystemFontCollection: Option<unsafe extern "system" fn ()>,
  pub CreateCustomFontCollection: Option<unsafe extern "system" fn ()>,
  pub RegisterFontCollectionLoader: Option<unsafe extern "system" fn ()>,
  pub UnregisterFontCollectionLoader: Option<unsafe extern "system" fn ()>,
  pub CreateFontFileReference: Option<unsafe extern "system" fn ()>,
  pub CreateCustomFontFileReference: Option<unsafe extern "system" fn ()>,
  pub CreateFontFace: Option<unsafe extern "system" fn ()>,
  pub CreateRenderingParams: Option<unsafe extern "system" fn (This: *mut IDWriteFactory,
                                                               params: *mut *mut IDWriteRenderingParams) -> HRESULT>,
  pub CreateMonitorRenderingParams: Option<unsafe extern "system" fn ()>,
  pub CreateCustomRenderingParams: Option<unsafe extern "system" fn (This: *mut IDWriteFactory,
                                                                     gamma: FLOAT,
                                                                     enhancedContrast: FLOAT,
                                                                     cleartype_level: FLOAT,
                                                                     geometry: DWRITE_PIXEL_GEOMETRY,
                                                                     mode: DWRITE_RENDERING_MODE,
                                                                     params: *mut *mut IDWriteRenderingParams) -> HRESULT>,
  pub RegisterFontFileLoader: Option<unsafe extern "system" fn ()>,
  pub UnregisterFontFileLoader: Option<unsafe extern "system" fn ()>,
  pub CreateTextFormat: Option<unsafe extern "system" fn ()>,
  pub CreateTypography: Option<unsafe extern "system" fn ()>,
  pub GetGdiInterop: Option<unsafe extern "system" fn (This: *mut IDWriteFactory,
                                                       gdi_interop: *mut *mut IDWriteGdiInterop) -> HRESULT>,
  pub CreateTextLayout: Option<unsafe extern "system" fn ()>,
  pub CreateGdiCompatibleTextLayout: Option<unsafe extern "system" fn ()>,
  pub CreateEllipsisTrimmingSign: Option<unsafe extern "system" fn ()>,
  pub CreateTextAnalyzer: Option<unsafe extern "system" fn ()>,
  pub CreateNumberSubstitution: Option<unsafe extern "system" fn ()>,
  pub CreateGlyphRunAnalysis: Option<unsafe extern "system" fn ()>,
}

/// C 445: `interface IDWriteFactory { CONST_VTBL IDWriteFactoryVtbl *lpVtbl; };` (C's one-line
/// spelling).
/// C: interface IDWriteFactory { CONST_VTBL IDWriteFactoryVtbl *lpVtbl; };
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteFactory
{
  pub lpVtbl: *const IDWriteFactoryVtbl,
}

/// C 447-462: `IDWriteColorGlyphRunEnumeratorVtbl`.  `MoveNext` and `GetCurrentRun` are the two the
/// colour-layer loop calls (C 1289/1297); the base slots are C's.
/// C: typedef struct IDWriteColorGlyphRunEnumeratorVtbl { BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IDWriteColorGlyphRunEnumerator *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IDWriteColorGlyphRunEnumerator *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IDWriteColorGlyphRunEnumerator *This);
/// C:   HRESULT (STDMETHODCALLTYPE *MoveNext) (IDWriteColorGlyphRunEnumerator *This, WINBOOL *hasRun);
/// C:   HRESULT (STDMETHODCALLTYPE *GetCurrentRun) (IDWriteColorGlyphRunEnumerator *This,
/// C:     const DWRITE_COLOR_GLYPH_RUN **run);
/// C:   END_INTERFACE } IDWriteColorGlyphRunEnumeratorVtbl;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteColorGlyphRunEnumeratorVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IDWriteColorGlyphRunEnumerator,
                                                        riid: REFIID,
                                                        ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IDWriteColorGlyphRunEnumerator) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IDWriteColorGlyphRunEnumerator) -> ULONG>,
  pub MoveNext: Option<unsafe extern "system" fn (This: *mut IDWriteColorGlyphRunEnumerator,
                                                  hasRun: *mut WINBOOL) -> HRESULT>,
  pub GetCurrentRun: Option<unsafe extern "system" fn (This: *mut IDWriteColorGlyphRunEnumerator,
                                                       run: *mut *const DWRITE_COLOR_GLYPH_RUN) -> HRESULT>,
}

/// C 464-466: `interface IDWriteColorGlyphRunEnumerator`.
/// C: interface IDWriteColorGlyphRunEnumerator { CONST_VTBL IDWriteColorGlyphRunEnumeratorVtbl *lpVtbl; };
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteColorGlyphRunEnumerator
{
  pub lpVtbl: *const IDWriteColorGlyphRunEnumeratorVtbl,
}

/// C 468-515: `IDWriteFactory2Vtbl`.  `TranslateColorGlyphRun` is the one member the port calls
/// (C 1243); every other slot is `EMACS_DWRITE_UNUSED`, in C's order, including the two slots whose
/// C names carry the `<dwrite_3.h>` interface prefixes (`IDWriteFactory1_…`,
/// `IDWriteFactory2_…`).
/// C: typedef struct IDWriteFactory2Vtbl { BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IDWriteFactory2 *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IDWriteFactory2 *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IDWriteFactory2 *This);
/// C:   EMACS_DWRITE_UNUSED (GetSystemFontCollection); EMACS_DWRITE_UNUSED (CreateCustomFontCollection);
/// C:   EMACS_DWRITE_UNUSED (RegisterFontCollectionLoader); EMACS_DWRITE_UNUSED (UnregisterFontCollectionLoader);
/// C:   EMACS_DWRITE_UNUSED (CreateFontFileReference); EMACS_DWRITE_UNUSED (CreateCustomFontFileReference);
/// C:   EMACS_DWRITE_UNUSED (CreateFontFace); EMACS_DWRITE_UNUSED (CreateRenderingParams);
/// C:   EMACS_DWRITE_UNUSED (CreateMonitorRenderingParams); EMACS_DWRITE_UNUSED (CreateCustomRenderingParams);
/// C:   EMACS_DWRITE_UNUSED (RegisterFontFileLoader); EMACS_DWRITE_UNUSED (UnregisterFontFileLoader);
/// C:   EMACS_DWRITE_UNUSED (CreateTextFormat); EMACS_DWRITE_UNUSED (CreateTypography);
/// C:   EMACS_DWRITE_UNUSED (GetGdiInterop); EMACS_DWRITE_UNUSED (CreateTextLayout);
/// C:   EMACS_DWRITE_UNUSED (CreateGdiCompatibleTextLayout); EMACS_DWRITE_UNUSED (CreateEllipsisTrimmingSign);
/// C:   EMACS_DWRITE_UNUSED (CreateTextAnalyzer); EMACS_DWRITE_UNUSED (CreateNumberSubstitution);
/// C:   EMACS_DWRITE_UNUSED (CreateGlyphRunAnalysis);
/// C:   EMACS_DWRITE_UNUSED (GetEudcFontCollection); EMACS_DWRITE_UNUSED (IDWriteFactory1_CreateCustomRenderingParams);
/// C:   EMACS_DWRITE_UNUSED (GetSystemFontFallback); EMACS_DWRITE_UNUSED (CreateFontFallbackBuilder);
/// C:   HRESULT (STDMETHODCALLTYPE *TranslateColorGlyphRun) (IDWriteFactory2 *This, FLOAT originX,
/// C:     FLOAT originY, const DWRITE_GLYPH_RUN *run, void *rundescr, DWRITE_MEASURING_MODE mode,
/// C:     void *transform, UINT32 palette_index, IDWriteColorGlyphRunEnumerator **colorlayers);
/// C:   EMACS_DWRITE_UNUSED (IDWriteFactory2_CreateCustomRenderingParams);
/// C:   EMACS_DWRITE_UNUSED (IDWriteFactory2_CreateGlyphRunAnalysis);
/// C:   END_INTERFACE } IDWriteFactory2Vtbl;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteFactory2Vtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IDWriteFactory2, riid: REFIID,
                                                        ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IDWriteFactory2) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IDWriteFactory2) -> ULONG>,
  pub GetSystemFontCollection: Option<unsafe extern "system" fn ()>,
  pub CreateCustomFontCollection: Option<unsafe extern "system" fn ()>,
  pub RegisterFontCollectionLoader: Option<unsafe extern "system" fn ()>,
  pub UnregisterFontCollectionLoader: Option<unsafe extern "system" fn ()>,
  pub CreateFontFileReference: Option<unsafe extern "system" fn ()>,
  pub CreateCustomFontFileReference: Option<unsafe extern "system" fn ()>,
  pub CreateFontFace: Option<unsafe extern "system" fn ()>,
  pub CreateRenderingParams: Option<unsafe extern "system" fn ()>,
  pub CreateMonitorRenderingParams: Option<unsafe extern "system" fn ()>,
  pub CreateCustomRenderingParams: Option<unsafe extern "system" fn ()>,
  pub RegisterFontFileLoader: Option<unsafe extern "system" fn ()>,
  pub UnregisterFontFileLoader: Option<unsafe extern "system" fn ()>,
  pub CreateTextFormat: Option<unsafe extern "system" fn ()>,
  pub CreateTypography: Option<unsafe extern "system" fn ()>,
  pub GetGdiInterop: Option<unsafe extern "system" fn ()>,
  pub CreateTextLayout: Option<unsafe extern "system" fn ()>,
  pub CreateGdiCompatibleTextLayout: Option<unsafe extern "system" fn ()>,
  pub CreateEllipsisTrimmingSign: Option<unsafe extern "system" fn ()>,
  pub CreateTextAnalyzer: Option<unsafe extern "system" fn ()>,
  pub CreateNumberSubstitution: Option<unsafe extern "system" fn ()>,
  pub CreateGlyphRunAnalysis: Option<unsafe extern "system" fn ()>,
  pub GetEudcFontCollection: Option<unsafe extern "system" fn ()>,
  pub IDWriteFactory1_CreateCustomRenderingParams: Option<unsafe extern "system" fn ()>,
  pub GetSystemFontFallback: Option<unsafe extern "system" fn ()>,
  pub CreateFontFallbackBuilder: Option<unsafe extern "system" fn ()>,
  pub TranslateColorGlyphRun: Option<unsafe extern "system" fn (This: *mut IDWriteFactory2,
                                                                originX: FLOAT,
                                                                originY: FLOAT,
                                                                run: *const DWRITE_GLYPH_RUN,
                                                                rundescr: *mut c_void,
                                                                mode: DWRITE_MEASURING_MODE,
                                                                transform: *mut c_void,
                                                                palette_index: UINT32,
                                                                colorlayers: *mut *mut IDWriteColorGlyphRunEnumerator) -> HRESULT>,
  pub IDWriteFactory2_CreateCustomRenderingParams: Option<unsafe extern "system" fn ()>,
  pub IDWriteFactory2_CreateGlyphRunAnalysis: Option<unsafe extern "system" fn ()>,
}

/// C 517-519: `interface IDWriteFactory2`.
/// C: interface IDWriteFactory2 { CONST_VTBL IDWriteFactory2Vtbl *lpVtbl; };
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct IDWriteFactory2
{
  pub lpVtbl: *const IDWriteFactory2Vtbl,
}

/// C 521-537: `ID2D1SimplifiedGeometrySinkVtbl`, the vtable the `geometry_sink` record's first
/// member points at (C 591).  The seven methods after the three base slots are the ones the sink
/// implements (C 616-685).  C 541's alias makes `IDWriteGeometrySink` the same record.
/// C: typedef struct ID2D1SimplifiedGeometrySinkVtbl { BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (ID2D1SimplifiedGeometrySink *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (ID2D1SimplifiedGeometrySink *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (ID2D1SimplifiedGeometrySink *This);
/// C:   VOID (STDMETHODCALLTYPE *SetFillMode) (ID2D1SimplifiedGeometrySink *This, D2D1_FILL_MODE fillMode);
/// C:   VOID (STDMETHODCALLTYPE *SetSegmentFlags) (ID2D1SimplifiedGeometrySink *This, D2D1_PATH_SEGMENT vertexFlags);
/// C:   VOID (STDMETHODCALLTYPE *BeginFigure) (ID2D1SimplifiedGeometrySink *This, D2D1_POINT_2F startPoint, D2D1_FIGURE_BEGIN figureBegin);
/// C:   VOID (STDMETHODCALLTYPE *AddLines) (ID2D1SimplifiedGeometrySink *This, const D2D1_POINT_2F *points, UINT pointsCount);
/// C:   VOID (STDMETHODCALLTYPE *AddBeziers) (ID2D1SimplifiedGeometrySink *This, const D2D1_BEZIER_SEGMENT *beziers, UINT beziersCount);
/// C:   VOID (STDMETHODCALLTYPE *EndFigure) (ID2D1SimplifiedGeometrySink *This, D2D1_FIGURE_END figureEnd);
/// C:   HRESULT (STDMETHODCALLTYPE *Close) (ID2D1SimplifiedGeometrySink *This);
/// C:   END_INTERFACE } ID2D1SimplifiedGeometrySinkVtbl;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct ID2D1SimplifiedGeometrySinkVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut ID2D1SimplifiedGeometrySink,
                                                        riid: REFIID,
                                                        ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut ID2D1SimplifiedGeometrySink) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut ID2D1SimplifiedGeometrySink) -> ULONG>,
  pub SetFillMode: Option<unsafe extern "system" fn (This: *mut ID2D1SimplifiedGeometrySink,
                                                     fillMode: D2D1_FILL_MODE)>,
  pub SetSegmentFlags: Option<unsafe extern "system" fn (This: *mut ID2D1SimplifiedGeometrySink,
                                                         vertexFlags: D2D1_PATH_SEGMENT)>,
  pub BeginFigure: Option<unsafe extern "system" fn (This: *mut ID2D1SimplifiedGeometrySink,
                                                     startPoint: D2D1_POINT_2F,
                                                     figureBegin: D2D1_FIGURE_BEGIN)>,
  pub AddLines: Option<unsafe extern "system" fn (This: *mut ID2D1SimplifiedGeometrySink,
                                                  points: *const D2D1_POINT_2F,
                                                  pointsCount: UINT)>,
  pub AddBeziers: Option<unsafe extern "system" fn (This: *mut ID2D1SimplifiedGeometrySink,
                                                    beziers: *const D2D1_BEZIER_SEGMENT,
                                                    beziersCount: UINT)>,
  pub EndFigure: Option<unsafe extern "system" fn (This: *mut ID2D1SimplifiedGeometrySink,
                                                   figureEnd: D2D1_FIGURE_END)>,
  pub Close: Option<unsafe extern "system" fn (This: *mut ID2D1SimplifiedGeometrySink) -> HRESULT>,
}

/// C 539-541: `interface ID2D1SimplifiedGeometrySink { const ID2D1SimplifiedGeometrySinkVtbl
/// *lpVtbl; };` (C writes `const` here, not `CONST_VTBL`).
/// C: interface ID2D1SimplifiedGeometrySink { const ID2D1SimplifiedGeometrySinkVtbl *lpVtbl; };
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct ID2D1SimplifiedGeometrySink
{
  pub lpVtbl: *const ID2D1SimplifiedGeometrySinkVtbl,
}

/// C 543: `typedef ID2D1SimplifiedGeometrySink IDWriteGeometrySink;` - the sink the two
/// `geometry_sink`-shaped functions (C 599-685) cast their `This` to.
/// C: typedef ID2D1SimplifiedGeometrySink IDWriteGeometrySink;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
pub type IDWriteGeometrySink = ID2D1SimplifiedGeometrySink;

/* C 545-547: `#else /* MINGW_W64 */` / `# include <dwrite_3.h>` / `# include <d2d1.h>`.  This is
   the other arm: with MINGW_W64 the declarations above come from the SDK's own headers instead.
   Neither header has a file in this tree and neither has a consumer here (the arm selects them
   only), so C's two lines stand as the section (P-88 (1); the ledger's Deviations 2).  */
// C 545: #else /* MINGW_W64 */
// C 546: # include <dwrite_3.h>
// C 547: # include <d2d1.h>
// C 548: #endif

/* ----------------------------------------------------------------------------------------------
   C 550-1399: the file's own program.  From here on every item carries the file gate only - C's
   own guards are the `MINGW_W64` pair (C 40-42, C 52-548, C 1136-1146) and `HAVE_HARFBUZZ`
   (C 1332-1339), and the rest of the text is unconditional, exactly as C's is.
   ---------------------------------------------------------------------------------------------- */

/* C 550-552: "User configurable variables.  If they are smaller than 0, use DirectWrite's defaults,
   or our defaults.  To set them, the user calls 'w32-dwrite-reinit'".  */
/// C: static float config_enhanced_contrast = -1.0f;
/// C: static float config_clear_type_level = -1.0f;
/// C: static float config_gamma = -1.0f;
#[cfg (feature = "have_ntgui")]
static mut config_enhanced_contrast: c_float = -1.0f32;
#[cfg (feature = "have_ntgui")]
static mut config_clear_type_level: c_float = -1.0f32;
#[cfg (feature = "have_ntgui")]
static mut config_gamma: c_float = -1.0f32;

/* C 557: "Values to use for DirectWrite rendering."  The three object-like macros keep C's names
   and each body is the enum member it expands to (PORTING.md 2.1).  */
/// C: #define MEASURING_MODE DWRITE_MEASURING_MODE_NATURAL
#[cfg (feature = "have_ntgui")]
macro_rules! MEASURING_MODE { () => { DWRITE_MEASURING_MODE_NATURAL }; }
/// C: #define RENDERING_MODE DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC
#[cfg (feature = "have_ntgui")]
macro_rules! RENDERING_MODE { () => { DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC }; }
/// C: #define ANTIALIAS_MODE DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE
#[cfg (feature = "have_ntgui")]
macro_rules! ANTIALIAS_MODE { () => { DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE }; }

/// C 562-570: `release_com` - "Release an interface pointer and clear it".  C's
/// `(*i)->lpVtbl->Release (*i)` is the port's two dereferences of C's two arrows, `Release` being
/// the `Option<unsafe extern "system" fn …>` field the mirror convention gives a C function
/// pointer.  C's `NULL` is `core::ptr::null_mut ()` (PORTING.md's `null` token family).
/// C: static void release_com (IUnknown **i) { if (*i) { (*i)->lpVtbl->Release (*i); *i = NULL; } }
#[cfg (feature = "have_ntgui")]
unsafe fn release_com (i: *mut *mut IUnknown)
{
  if !(*i).is_null ()
    {
      (*(**i).lpVtbl).Release.unwrap () (*i);
      *i = core::ptr::null_mut ();
    }
}

/// C 572: `#define RELEASE_COM(i) release_com ((IUnknown **) &i)`.  C's cast is the port's
/// `as *mut *mut IUnknown`, and C's `&i` - the address of the *place* the macro parameter names -
/// is `&mut $i` (each call site passes a mutable place).
/// C: #define RELEASE_COM(i) release_com ((IUnknown **) &i)
#[cfg (feature = "have_ntgui")]
macro_rules! RELEASE_COM { ($i:expr) => { release_com (&raw mut $i as *mut *mut IUnknown) }; }

/* C 574-587: "Implementation of IDWriteGeometrySink, used to the get bounding vertical coordinates
   of glyphs (ascent/descent). …" - carried as C's own words:  */

/* Implementation of IDWriteGeometrySink, used to the get bounding
   vertical coordinates of glyphs (ascent/descent).  The methods that
   affect the bounding box are BeginFigure (which gives a start point),
   AddBeziers and AddLines.

   Normal procedures to get text extents fail to give correct
   ascent/descent metrics for individual glyphs, using a default value
   for an entire font.  That is not acceptable, specially for fonts like
   "Sans Serif Collection", which include glyphs for many different
   scripts and have a huge default value.

   Because of that, we need to use the GetGlyphRunOutline and examine
   the glyph's geometry.
*/

/// C 589-594: `struct geometry_sink`.  C's first member is the whole `IDWriteGeometrySink`, so
/// `(struct geometry_sink *) This` (C 618 and its seven siblings) is valid and the port's
/// `#[repr(C)]` record keeps that layout.
/// C: struct geometry_sink { IDWriteGeometrySink sink; int empty; float min_y, max_y; };
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct geometry_sink
{
  pub sink: IDWriteGeometrySink,
  pub empty: c_int,
  pub min_y: c_float,
  pub max_y: c_float,
}

// C 596-600: `geometry_sink_QueryInterface` - the sink has nothing to hand out.
// C: static HRESULT STDMETHODCALLTYPE geometry_sink_QueryInterface (IUnknown *This, REFIID ri,
// C:   void **r) { return E_NOINTERFACE; }
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn geometry_sink_QueryInterface (This: *mut IUnknown, ri: REFIID,
                                                        r: *mut *mut c_void) -> HRESULT
{
  return E_NOINTERFACE;
}

// C 602-607: `geometry_sink_AddRef` - "There is nothing to allocate of free heres, so we can
// safely skip ref counting."
// C: static ULONG STDMETHODCALLTYPE geometry_sink_AddRef (IUnknown *This) { return 1; }
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn geometry_sink_AddRef (This: *mut IUnknown) -> ULONG
{
  return 1;
}

// C 609-613: `geometry_sink_Release`, the same reading.
// C: static ULONG STDMETHODCALLTYPE geometry_sink_Release (IUnknown *This) { return 1; }
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn geometry_sink_Release (This: *mut IUnknown) -> ULONG
{
  return 1;
}

// C 615-635: `geometry_sink_AddBeziers` - the first of the three methods that move the bounding
// box.  C's `beziers[i]` is the port's `*beziers.offset (i as isize)`; `UINT32` and the `UINT` the
// vtable field declares are the same 32-bit unsigned type (`as UINT32` at C 618's cast is the
// `*mut geometry_sink` conversion below).
// C: static void STDMETHODCALLTYPE geometry_sink_AddBeziers (IDWriteGeometrySink *This,
// C:   const D2D1_BEZIER_SEGMENT *beziers, UINT32 count) { struct geometry_sink *sink =
// C:   (struct geometry_sink *) This; for (UINT32 i = 0; i < count; i++) { … point1/2/3 y … } }
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn geometry_sink_AddBeziers (This: *mut IDWriteGeometrySink,
                                                   beziers: *const D2D1_BEZIER_SEGMENT,
                                                   count: UINT32)
{
  let sink: *mut geometry_sink = This as *mut geometry_sink;
  let mut i: UINT32 = 0;
  while i < count
    {
      if (*sink).min_y > (*beziers.offset (i as isize)).point1.y
	{ (*sink).min_y = (*beziers.offset (i as isize)).point1.y; }
      if (*sink).max_y < (*beziers.offset (i as isize)).point1.y
	{ (*sink).max_y = (*beziers.offset (i as isize)).point1.y; }
      if (*sink).min_y > (*beziers.offset (i as isize)).point2.y
	{ (*sink).min_y = (*beziers.offset (i as isize)).point2.y; }
      if (*sink).max_y < (*beziers.offset (i as isize)).point2.y
	{ (*sink).max_y = (*beziers.offset (i as isize)).point2.y; }
      if (*sink).min_y > (*beziers.offset (i as isize)).point3.y
	{ (*sink).min_y = (*beziers.offset (i as isize)).point3.y; }
      if (*sink).max_y < (*beziers.offset (i as isize)).point3.y
	{ (*sink).max_y = (*beziers.offset (i as isize)).point3.y; }
      i += 1;
    }
}

// C 637-649: `geometry_sink_AddLines`, the same reading with one point per iteration.
// C: static void STDMETHODCALLTYPE geometry_sink_AddLines (IDWriteGeometrySink *This,
// C:   const D2D1_POINT_2F *points, UINT32 count) { … points[i].y … }
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn geometry_sink_AddLines (This: *mut IDWriteGeometrySink,
                                                 points: *const D2D1_POINT_2F,
                                                 count: UINT32)
{
  let sink: *mut geometry_sink = This as *mut geometry_sink;
  let mut i: UINT32 = 0;
  while i < count
    {
      if (*sink).min_y > (*points.offset (i as isize)).y
	{ (*sink).min_y = (*points.offset (i as isize)).y; }
      if (*sink).max_y < (*points.offset (i as isize)).y
	{ (*sink).max_y = (*points.offset (i as isize)).y; }
      i += 1;
    }
}

// C 651-662: `geometry_sink_BeginFigure` - the start point joins the box and the figure is no
// longer empty (`empty = 0`).
// C: static void STDMETHODCALLTYPE geometry_sink_BeginFigure (IDWriteGeometrySink *This,
// C:   D2D1_POINT_2F startPoint, D2D1_FIGURE_BEGIN figureBegin) { … sink->empty = 0; }
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn geometry_sink_BeginFigure (This: *mut IDWriteGeometrySink,
                                                    startPoint: D2D1_POINT_2F,
                                                    figureBegin: D2D1_FIGURE_BEGIN)
{
  let sink: *mut geometry_sink = This as *mut geometry_sink;
  if (*sink).min_y > startPoint.y
    { (*sink).min_y = startPoint.y; }
  if (*sink).max_y < startPoint.y
    { (*sink).max_y = startPoint.y; }
  (*sink).empty = 0;
}

// C 664-667: `geometry_sink_EndFigure` - C's body is empty, and this one is too (PORTING.md
// 0.3: a C body that does nothing stays a body that does nothing).
// C: static void STDMETHODCALLTYPE geometry_sink_EndFigure (IDWriteGeometrySink *This,
// C:   D2D1_FIGURE_END figureEnd) { }
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn geometry_sink_EndFigure (This: *mut IDWriteGeometrySink,
                                                  figureEnd: D2D1_FIGURE_END)
{
}

// C 669-673: `geometry_sink_Close` - "return S_OK" is C's whole body.
// C: static HRESULT STDMETHODCALLTYPE geometry_sink_Close (IDWriteGeometrySink *This)
// C: { return S_OK; }
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn geometry_sink_Close (This: *mut IDWriteGeometrySink) -> HRESULT
{
  return S_OK;
}

// C 675-679: `geometry_sink_SetFillMode` - empty (the sink only reads the box).
// C: static void STDMETHODCALLTYPE geometry_sink_SetFillMode (IDWriteGeometrySink *This,
// C:   D2D1_FILL_MODE fillMode) { }
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn geometry_sink_SetFillMode (This: *mut IDWriteGeometrySink,
                                                     fillMode: D2D1_FILL_MODE)
{
}

// C 681-685: `geometry_sink_SetSegmentFlags` - empty.
// C: static void STDMETHODCALLTYPE geometry_sink_SetSegmentFlags (IDWriteGeometrySink *This,
// C:   D2D1_PATH_SEGMENT vertexFlags) { }
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn geometry_sink_SetSegmentFlags (This: *mut IDWriteGeometrySink,
                                                         vertexFlags: D2D1_PATH_SEGMENT)
{
}

/* C 687: "Global variables for DirectWrite."  */
/// C 688: `static bool direct_write_available = false;`
#[cfg (feature = "have_ntgui")]
static mut direct_write_available: bool = false;
/// C 689-692: the four COM pointers the module keeps; C's `NULL` is the port's null pointer.
/// C: static IDWriteFactory *dwrite_factory = NULL; static IDWriteFactory2 *dwrite_factory2 = NULL;
/// C: static IDWriteGdiInterop *gdi_interop = NULL; static IDWriteRenderingParams *rendering_params = NULL;
#[cfg (feature = "have_ntgui")]
static mut dwrite_factory: *mut IDWriteFactory = core::ptr::null_mut ();
#[cfg (feature = "have_ntgui")]
static mut dwrite_factory2: *mut IDWriteFactory2 = core::ptr::null_mut ();
#[cfg (feature = "have_ntgui")]
static mut gdi_interop: *mut IDWriteGdiInterop = core::ptr::null_mut ();
#[cfg (feature = "have_ntgui")]
static mut rendering_params: *mut IDWriteRenderingParams = core::ptr::null_mut ();
/// C 693-694: the sink record and its vtable.  Both are C `static`s with static storage duration,
/// so their zero value is `core::mem::zeroed ()` (the `src/w32font.rs` reading); the vtable's
/// eleven slots are filled in at C 1136-1154.
/// C: static struct geometry_sink dwrite_geometry_sink;
/// C: static ID2D1SimplifiedGeometrySinkVtbl dwrite_geometry_sink_vtbl;
#[cfg (feature = "have_ntgui")]
static mut dwrite_geometry_sink: geometry_sink = unsafe { core::mem::zeroed () };
#[cfg (feature = "have_ntgui")]
static mut dwrite_geometry_sink_vtbl: ID2D1SimplifiedGeometrySinkVtbl =
  unsafe { core::mem::zeroed () };

/// C 696-706: `verify_hr` - every DirectWrite call's result goes through it, and C's `DebPrint`
/// is the mirror's macro of the same name (C 701).
/// C: static bool verify_hr (HRESULT hr, const char *msg) { if (FAILED (hr)) { DebPrint ((…));
/// C:   eassert (SUCCEEDED (hr)); return false; } return true; }
#[cfg (feature = "have_ntgui")]
unsafe fn verify_hr (hr: HRESULT, msg: *const c_char) -> bool
{
  if FAILED! (hr)
    {
      DebPrint! (("DirectWrite HRESULT failed: (%d) %s\n", hr, msg));
      eassert! (SUCCEEDED! (hr));
      return false;
    }
  return true;
}

/* C 708-722: `get_font_face`'s design note - "Gets a IDWriteFontFace from a struct font (its
   HFONT). … Never call Release on the result, as it is cached for reuse on the struct font."  */

/// C 723-789: `get_font_face`.  The `uniscribe_font_info` members are `include/w32font.rs`'s
/// (`dwrite_cache` is C's `void *`, so both readings of it carry the `*mut c_void` ⇄
/// `*mut IDWriteFontFace` conversion C gets from its implicit `void *` conversions); `LOGFONTW`
/// and `GetObjectW` are the stand-in section's; `sizeof (LOGFONTW)` is `size_of::<LOGFONTW> ()`
/// and the `unsigned` C compares is the port's `c_uint`.
/// C: `*face = uniscribe_font->dwrite_cache; if (*face) return uniscribe_font->dwrite_font_size;`
/// C: `GetObjectW (FONT_HANDLE (infont), sizeof (LOGFONTW), &logfont);`
/// C: `hr = gdi_interop->lpVtbl->CreateFontFaceFromHdc (gdi_interop, hdc, face);`
/// C: `DWORD nbytes = GetFontData (hdc, ('p' << 24) | ('x' << 16) | ('a' << 8) | 'm', 4, num_glyphs, 2);`
/// C: `if (gdi_glyph_count != (*face)->lpVtbl->GetGlyphCount (*face)) …`
/// C: `uniscribe_font->dwrite_font_size = eabs (logfont.lfHeight);`
#[cfg (feature = "have_ntgui")]
unsafe fn get_font_face (infont: *mut font, face: *mut *mut IDWriteFontFace) -> c_float
{
  let mut hr: HRESULT = 0;					// C 726
  let mut logfont: LOGFONTW = core::mem::zeroed ();		// C 727

  let uniscribe_font: *mut uniscribe_font_info
    = infont as *mut uniscribe_font_info;			// C 729-730

  /* Check the cache.  */
  *face = (*uniscribe_font).dwrite_cache as *mut IDWriteFontFace;	// C 733
  if !(*face).is_null ()					// C 734
    { return (*uniscribe_font).dwrite_font_size; }

  GetObjectW (FONT_HANDLE! (infont) as *mut c_void, size_of::<LOGFONTW> () as c_int,
	      &raw mut logfont as *mut c_void);			// C 737

  /* Select the font into a DC, so that DirectWrite can give us the face
     for the font GDI realizes for it.  */
  let f: *mut frame = SELECTED_FRAME ();			// C 741
  let hdc: HDC = get_frame_dc (f);				// C 742
  let old_font: HFONT = SelectObject (hdc, FONT_HANDLE! (infont) as *mut c_void) as HFONT;	// C 743

  hr = (*(*gdi_interop).lpVtbl).CreateFontFaceFromHdc.unwrap () (gdi_interop, hdc, face);	// C 745

  /* The number of glyphs of the font GDI selected, from its 'maxp'
     table.  The tag is pushed into a DWORD backwards, to cope with
     endianness, as GetFontData expects it.  */
  let mut num_glyphs: [BYTE; 2] = [0, 0];			// C 750
  let nbytes: DWORD = GetFontData (hdc,
				   (('p' as DWORD) << 24) | (('x' as DWORD) << 16)
				   | (('a' as DWORD) << 8) | ('m' as DWORD),
				   4, &raw mut num_glyphs as *mut c_void, 2);	// C 751-752

  SelectObject (hdc, old_font as *mut c_void);			// C 754
  release_frame_dc (f, hdc);					// C 755

  if !verify_hr (hr, c"Failed to CreateFontFaceFromHdc".as_ptr ())	// C 757
    {
      (*uniscribe_font).dwrite_skip_font = true;		// C 759
      *face = core::ptr::null_mut ();				// C 760
      return 0.0;						// C 761
    }

  /* CreateFontFaceFromHdc is documented to return the face of the font
     that GDI uses, but if that ever fails to hold, we would be drawing
     glyph indices produced from the GDI font with a different font,
     and display garbage.  Verify it, and let GDI do the drawing for
     this font if the two disagree.  If 'maxp' could not be read,
     skip the check rather than lose DirectWrite for this font.  */
  if nbytes == 2						// C 771
    {
      let gdi_glyph_count: c_uint = ((num_glyphs[0] as c_uint) << 8)
	+ (num_glyphs[1] as c_uint);				// C 773

      if gdi_glyph_count != (*(**face).lpVtbl).GetGlyphCount.unwrap () (*face) as c_uint	// C 775
	{
	  DebPrint! (("DirectWrite and GDI disagree about the font\n"));	// C 777
	  RELEASE_COM! (*face);					// C 778
	  (*uniscribe_font).dwrite_skip_font = true;		// C 779
	  *face = core::ptr::null_mut ();			// C 780
	  return 0.0;						// C 781
	}
    }

  /* Cache this FontFace.  */
  (*uniscribe_font).dwrite_font_size = eabs (logfont.lfHeight) as c_float;	// C 786
  (*uniscribe_font).dwrite_cache = *face as *mut c_void;	// C 787

  return eabs (logfont.lfHeight) as c_float;			// C 789
}

/// C 791-796: `w32_dwrite_free_cached_face` - the one non-`static` entry the font driver's
/// `close_font` calls (declared in `w32font.h`, so it carries `#[unsafe(no_mangle)]`).
/// C: void w32_dwrite_free_cached_face (void *cache) { if (cache) RELEASE_COM (cache); }
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_dwrite_free_cached_face (mut cache: *mut c_void)
{
  if !cache.is_null ()						// C 794
    { RELEASE_COM! (cache); }					// C 795
}

/// C 798-802: `convert_metrics_sz` - "Convert a size in font design units to pixels".  C's
/// `(float) sz` is the port's `as c_float`, and the division is C's own `int`→`float` promotion of
/// `units_per_em`.
/// C: static float convert_metrics_sz (int sz, float font_size, int units_per_em)
/// C: { return (float) sz * font_size / units_per_em; }
#[cfg (feature = "have_ntgui")]
unsafe fn convert_metrics_sz (sz: c_int, font_size: c_float, units_per_em: c_int) -> c_float
{
  return (sz as c_float) * font_size / units_per_em as c_float;
}

/* C 805-807: "If the caller does not need ascent/descent information, it should pass
   NEED_ASCENT_DESCENT = false.  This is used to avoid the overhead of calling GetGlyphRunOutline." */

/// C 809-916: `text_extents_internal`.  C's two `SAFE_ALLOCA`s are the mirror's two-argument macro
/// (the `<sa_avail>`/`<sa_count>` the call site names, the one deviation `PORTING.md` records for
/// it), its five `sizeof`s are `size_of::<…> ()`, and C's `round` (and `FLT_MAX`) come from
/// `<math.h>`/`<float.h>` (C 37's chain).  The three `E_INVALIDARG`/`verify_hr` early exits and
/// their `SAFE_FREE ()` are C's own control flow, and the metric assignments are C's implicit
/// narrowings (`float`→`short`, `double`→`int`→`short`).
/// C: `dwrite_font_face->lpVtbl->GetMetrics (dwrite_font_face, &dwrite_font_metrics);`
/// C: `UINT16 *indices = SAFE_ALLOCA (nglyphs * sizeof (UINT16));`
/// C: `DWRITE_GLYPH_METRICS *gmetrics = SAFE_ALLOCA (nglyphs * sizeof (DWRITE_GLYPH_METRICS));`
/// C: `hr = dwrite_font_face->lpVtbl->GetGdiCompatibleGlyphMetrics (…, 1.0, NULL, TRUE, …);`
/// C: `if (hr == E_INVALIDARG) { SAFE_FREE (); return false; }`
/// C: `dwrite_geometry_sink.min_y = FLT_MAX; dwrite_geometry_sink.max_y = -FLT_MAX; …empty = 1;`
/// C: `metrics->ascent = (int) round (-dwrite_geometry_sink.min_y);`
#[cfg (feature = "have_ntgui")]
unsafe fn text_extents_internal (dwrite_font_face: *mut IDWriteFontFace,
                                 need_ascent_descent: bool,
                                 font_size: c_float, code: *const c_uint,
                                 nglyphs: c_int, metrics: *mut font_metrics) -> bool
{
  let mut hr: HRESULT = 0;					// C 815

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);			// C 817

  let mut dwrite_font_metrics: DWRITE_FONT_METRICS = core::mem::zeroed ();	// C 819
  (*(*dwrite_font_face).lpVtbl).GetMetrics.unwrap () (dwrite_font_face,
						      &raw mut dwrite_font_metrics);	// C 820-821

  let indices: *mut UINT16 = SAFE_ALLOCA! (sa_avail, nglyphs * size_of::<UINT16> () as c_int)
    as *mut UINT16;						// C 823
  let mut i: c_int = 0;						// C 824
  while i < nglyphs
    {
      *indices.offset (i as isize) = *code.offset (i as isize) as UINT16;	// C 825
      i += 1;
    }

  let gmetrics: *mut DWRITE_GLYPH_METRICS
    = SAFE_ALLOCA! (sa_avail, nglyphs * size_of::<DWRITE_GLYPH_METRICS> () as c_int)
      as *mut DWRITE_GLYPH_METRICS;				// C 827-828

  hr = (*(*dwrite_font_face).lpVtbl).GetGdiCompatibleGlyphMetrics.unwrap ()
    (dwrite_font_face,
     font_size,
     1.0,
     core::ptr::null_mut (),
     TRUE,
     indices,
     nglyphs as UINT32,
     gmetrics,
     false as WINBOOL);						// C 830-838

  /* E_INVALIDARG means some of the glyphs index is out of bounds for the font.  */
  if hr == E_INVALIDARG						// C 841
    {
      SAFE_FREE! (sa_count, sa_mark);					// C 843
      return false;						// C 844
    }

  if !verify_hr (hr, c"Failed to GetGdiCompatibleGlyphMetrics".as_ptr ())	// C 847
    {
      SAFE_FREE! (sa_count, sa_mark);					// C 849
      return false;						// C 850
    }

  let mut width: c_float = 0.0;					// C 853
  let du_per_em: c_int = dwrite_font_metrics.designUnitsPerEm as c_int;	// C 854

  let mut i: c_int = 0;						// C 856
  while i < nglyphs
    {
      let advance: c_float
	= convert_metrics_sz ((*gmetrics.offset (i as isize)).advanceWidth as c_int,
			      font_size, du_per_em);		// C 858-859

      width += advance;						// C 861

      let lbearing: c_float
	= round (convert_metrics_sz ((*gmetrics.offset (i as isize)).leftSideBearing,
				     font_size, du_per_em) as c_double) as c_float;	// C 861-863
      let rbearing: c_float
	= round ((advance -
		  convert_metrics_sz ((*gmetrics.offset (i as isize)).rightSideBearing,
				      font_size, du_per_em)) as c_double) as c_float;	// C 866-869
      if i == 0							// C 870
	{
	  (*metrics).lbearing = lbearing as c_short;		// C 872
	  (*metrics).rbearing = rbearing as c_short;		// C 873
	}
      if ((*metrics).lbearing as c_float) > lbearing		// C 875
	{ (*metrics).lbearing = lbearing as c_short; }
      if ((*metrics).rbearing as c_float) < rbearing		// C 877
	{ (*metrics).rbearing = rbearing as c_short; }
      i += 1;
    }
  (*metrics).width = round (width as c_double) as c_short;	// C 880

  if need_ascent_descent					// C 882
    {
      dwrite_geometry_sink.min_y = FLT_MAX;			// C 884
      dwrite_geometry_sink.max_y = -FLT_MAX;			// C 885
      dwrite_geometry_sink.empty = 1;				// C 886

      hr = (*(*dwrite_font_face).lpVtbl).GetGlyphRunOutline.unwrap ()
	(dwrite_font_face,
	 font_size,
	 indices,
	 core::ptr::null (),
	 core::ptr::null (),
	 nglyphs as UINT32,
	 FALSE,
	 FALSE,
	 &raw mut dwrite_geometry_sink.sink);			// C 888-896

      if !verify_hr (hr, c"Failed to GetGlyhRunOutline".as_ptr ())	// C 898
	{
	  SAFE_FREE! (sa_count, sa_mark);				// C 900
	  return false;						// C 901
	}

      if dwrite_geometry_sink.empty != 0			// C 904
	{
	  dwrite_geometry_sink.min_y = 0.0;			// C 906
	  dwrite_geometry_sink.max_y = 0.0;			// C 907
	}

      (*metrics).ascent = round (-dwrite_geometry_sink.min_y as c_double) as c_int as c_short;	// C 910
      (*metrics).descent = round (dwrite_geometry_sink.max_y as c_double) as c_int as c_short;	// C 911
    }

  SAFE_FREE! (sa_count, sa_mark);					// C 914
  return true;							// C 915
}

/// C 918-938: `w32_dwrite_encode_char` - the HarfBuzz driver's `encode_char`.  C's
/// `(UINT32 *) &c` is the port's `&raw mut c as *const UINT32` (the field's parameter is
/// `const UINT32 *`), `index` returns as C's `unsigned`, and the final `dwrite_skip_font` mark is
/// C's "this font is not ours" latch.
/// C: unsigned w32_dwrite_encode_char (struct font *font, int c) { … get_font_face (font, &dwrite_font_face); … }
/// C: `hr = dwrite_font_face->lpVtbl->GetGlyphIndices (dwrite_font_face, (UINT32 *) &c, 1, &index);`
/// C: `if (verify_hr (hr, "Failed to GetGlyphIndices")) { if (index == 0) return FONT_INVALID_CODE; return index; }`
/// C: `((struct uniscribe_font_info *) font)->dwrite_skip_font = true;`
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_dwrite_encode_char (font: *mut font, c: c_int) -> c_uint
{
  let mut hr: HRESULT = 0;					// C 921
  let mut dwrite_font_face: *mut IDWriteFontFace = core::ptr::null_mut ();	// C 922
  let mut index: UINT16 = 0;					// C 923

  get_font_face (font, &raw mut dwrite_font_face);		// C 925
  if dwrite_font_face.is_null ()				// C 926
    { return FONT_INVALID_CODE; }				// C 927
  hr = (*(*dwrite_font_face).lpVtbl).GetGlyphIndices.unwrap ()
    (dwrite_font_face, &raw mut c as *const UINT32, 1, &raw mut index);	// C 928-929
  if verify_hr (hr, c"Failed to GetGlyphIndices".as_ptr ())	// C 930
    {
      if index == 0						// C 932
	{ return FONT_INVALID_CODE; }				// C 933
      return index as c_uint;					// C 934
    }
  (*((font) as *mut uniscribe_font_info)).dwrite_skip_font = true;	// C 936
  return FONT_INVALID_CODE;					// C 937
}

/// C 940-962: `w32_dwrite_text_extents`.  C's `font_size <= 0.0f` and the
/// `FRAME_LINE_HEIGHT (SELECTED_FRAME ())` fallback are the port's, and `metrics` starts from the
/// font's own ascent/descent before `text_extents_internal` refines them.
/// C: bool w32_dwrite_text_extents (struct font *font, const unsigned *code, int nglyphs,
/// C:   struct font_metrics *metrics) { … if (font_size <= 0.0f) font_size = FRAME_LINE_HEIGHT
/// C:   (SELECTED_FRAME ()); metrics->ascent = font->ascent; metrics->descent = font->descent; … }
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_dwrite_text_extents (font: *mut font, code: *const c_uint,
                                                  nglyphs: c_int,
                                                  metrics: *mut font_metrics) -> bool
{
  let mut dwrite_font_face: *mut IDWriteFontFace = core::ptr::null_mut ();	// C 944

  let mut font_size: c_float = get_font_face (font, &raw mut dwrite_font_face);	// C 946

  if dwrite_font_face.is_null ()				// C 948
    { return false; }

  /* We can get fonts with a size of 0.  GDI handles this by using a default
     size.  We do the same.  */
  if font_size <= 0.0						// C 953
    { font_size = FRAME_LINE_HEIGHT (SELECTED_FRAME ()) as c_float; }	// C 954

  (*metrics).ascent = (*font).ascent as c_short;		// C 956
  (*metrics).descent = (*font).descent as c_short;		// C 957

  return text_extents_internal (dwrite_font_face, true,			// C 959
				font_size, code, nglyphs,
				metrics);
}

/* C 964-965: "Never call Release on the value returned by this function, as it is reused."  */

/// C 966-1011: `get_bitmap_render_target`.  The two C `static`s are the function's own storage
/// (`brt` and `size`), `SetPixelsPerDip (brt, 1.0)` is C 996, and the `QueryInterface` for
/// `IDWriteBitmapRenderTarget1` (C 1000-1009) is the one place this file passes one of its own
/// `IID_*` records.
/// C: static IDWriteBitmapRenderTarget *get_bitmap_render_target (HDC hdc, int width, int height)
/// C: { … static IDWriteBitmapRenderTarget *brt = NULL; static SIZE size = {0, 0}; …
/// C:   hr = gdi_interop->lpVtbl->CreateBitmapRenderTarget (gdi_interop, hdc, size.cx, size.cy, &brt);
/// C:   brt->lpVtbl->SetPixelsPerDip (brt, 1.0);
/// C:   hr = brt->lpVtbl->QueryInterface (brt, &IID_IDWriteBitmapRenderTarget1, (void **) &brt1);
/// C:   brt1->lpVtbl->SetTextAntialiasMode (brt1, ANTIALIAS_MODE); RELEASE_COM (brt1); }
#[cfg (feature = "have_ntgui")]
unsafe fn get_bitmap_render_target (hdc: HDC, width: c_int, height: c_int)
  -> *mut IDWriteBitmapRenderTarget
{
  let mut hr: HRESULT = 0;					// C 969
  static mut brt: *mut IDWriteBitmapRenderTarget = core::ptr::null_mut ();	// C 970
  static mut size: SIZE = SIZE { cx: 0, cy: 0 };		// C 971

  if !brt.is_null ()						// C 973
    {
      /* Check if we need to make a bigger one.  */
      if width <= size.cx && height <= size.cy			// C 976
	{ return brt; }
      RELEASE_COM! (brt);					// C 978
    }

  if width > size.cx						// C 981
    { size.cx = width; }
  if height > size.cy						// C 983
    { size.cy = height; }

  hr = (*(*gdi_interop).lpVtbl).CreateBitmapRenderTarget.unwrap ()
    (gdi_interop,
     hdc,
     size.cx as UINT32, size.cy as UINT32,
     &raw mut brt);						// C 986-989
  if !verify_hr (hr, c"Failed to CreateBitmapRenderTarget".as_ptr ())	// C 990
    { return core::ptr::null_mut (); }				// C 991

  /* We handle high dpi displays by increasing font size, so override
     PixelsPerDip.  */
  (*(*brt).lpVtbl).SetPixelsPerDip.unwrap () (brt, 1.0);	// C 995

  /* The SetTextAntialiasMode method is only available in
     IDWriteBitmapRenderTarget1.  */
  let mut brt1: *mut IDWriteBitmapRenderTarget1 = core::ptr::null_mut ();	// C 999
  hr = (*(*brt).lpVtbl).QueryInterface.unwrap () (brt,
						  &raw const IID_IDWriteBitmapRenderTarget1,
						  &raw mut brt1 as *mut *mut c_void);	// C 1000-1002
  /* This error should not happen, but is not catastrofic  */
  if verify_hr (hr, c"Failed to QueryInterface for IDWriteBitmapRenderTarget1".as_ptr ())	// C 1004
    {
      (*(*brt1).lpVtbl).SetTextAntialiasMode.unwrap () (brt1, ANTIALIAS_MODE! ());	// C 1006
      RELEASE_COM! (brt1);					// C 1007
    }

  return brt;							// C 1010
}

/// C 1013-1158: `w32_initialize_direct_write`.  The loader half is `include/w32common.rs`'s
/// `get_proc_addr` (C 1069) and `LoadLibrary`/`FreeLibrary` (the stand-in section); C's function-local
/// `typedef ... DWCreateFactory` is the port's local type alias, and C's three
/// `(IUnknown **) &obj` / `(void **) &obj` casts are the port's `as` casts at the same sites.
/// C: static HRESULT (WINAPI *DWCreateFactory) (DWRITE_FACTORY_TYPE, REFIID, IUnknown **);
/// C: `HMODULE direct_write = LoadLibrary ("dwrite.dll"); if (!direct_write) return;`
/// C: `hr = dw_create_factory (DWRITE_FACTORY_TYPE_SHARED, &IID_IDWriteFactory, (IUnknown **) &dwrite_factory);`
/// C: `hr = dwrite_factory->lpVtbl->QueryInterface (dwrite_factory, &IID_IDWriteFactory2, (void **) &dwrite_factory2);`
/// C: `hr = dwrite_factory->lpVtbl->CreateCustomRenderingParams (dwrite_factory, config_gamma, …);`
/// C: `dwrite_geometry_sink.sink.lpVtbl = &dwrite_geometry_sink_vtbl;`
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_initialize_direct_write ()
{
  direct_write_available = false;				// C 1016

  if !dwrite_factory.is_null ()					// C 1018
    {
      RELEASE_COM! (dwrite_factory);				// C 1020
      RELEASE_COM! (dwrite_factory2);				// C 1021
      RELEASE_COM! (gdi_interop);				// C 1022
      RELEASE_COM! (rendering_params);				// C 1023
    }

  let direct_write: HMODULE = LoadLibrary (c"dwrite.dll".as_ptr ());	// C 1026
  if direct_write.is_null ()					// C 1027
    { return; }

  /* This is only used here, no need to define it globally.  */
  type DWCreateFactory = unsafe extern "system" fn (DWRITE_FACTORY_TYPE,
						    REFIID, *mut *mut IUnknown) -> HRESULT;	// C 1031-1032

  let dw_create_factory: DWCreateFactory
    = core::mem::transmute::<VOIDFNPTR, DWCreateFactory> (
	get_proc_addr (direct_write,
		       c"DWriteCreateFactory".as_ptr ()));	// C 1034-1036

  if dw_create_factory == core::mem::transmute::<*const c_void, DWCreateFactory> (core::ptr::null ())	// C 1038
    {
      FreeLibrary (direct_write);				// C 1040
      return;							// C 1041
    }

  let mut hr: HRESULT = dw_create_factory (DWRITE_FACTORY_TYPE_SHARED,	// C 1044-1046
					   &raw const IID_IDWriteFactory,
					   &raw mut dwrite_factory as *mut *mut IUnknown);
  if FAILED! (hr)						// C 1047
    {
      DebPrint! (("DirectWrite HRESULT failed: (%d) CreateFactory\n", hr));	// C 1049
      FreeLibrary (direct_write);				// C 1050
      eassert! (SUCCEEDED! (hr));				// C 1051
      return;							// C 1052
    }

  /* IDWriteFactory2 is only available on Windows 8.1 and later.
     Without this, we can't use color fonts.  So we disable DirectWrite
     if it is not available.  */
  hr = (*(*dwrite_factory).lpVtbl).QueryInterface.unwrap () (dwrite_factory,	// C 1058-1060
							    &raw const IID_IDWriteFactory2,
							    &raw mut dwrite_factory2 as *mut *mut c_void);

  if FAILED! (hr)						// C 1062
    {
      DebPrint! (("DirectWrite HRESULT failed: (%d) QueryInterface IDWriteFactory2\n", hr));	// C 1064
      RELEASE_COM! (dwrite_factory);				// C 1065
      FreeLibrary (direct_write);				// C 1066
      return;							// C 1067
    }

  hr = (*(*dwrite_factory).lpVtbl).GetGdiInterop.unwrap () (dwrite_factory,	// C 1070-1071
							   &raw mut gdi_interop);
  if FAILED! (hr)						// C 1072
    {
      DebPrint! (("DirectWrite HRESULT failed: (%d) GetGdiInterop\n", hr));	// C 1074
      RELEASE_COM! (dwrite_factory);				// C 1075
      RELEASE_COM! (dwrite_factory2);				// C 1076
      FreeLibrary (direct_write);				// C 1077
      eassert! (SUCCEEDED! (hr));				// C 1078
      return;							// C 1079
    }

  let mut def: *mut IDWriteRenderingParams = core::ptr::null_mut ();	// C 1082

  hr = (*(*dwrite_factory).lpVtbl).CreateRenderingParams.unwrap () (dwrite_factory,	// C 1084-1085
								   &raw mut def);
  if FAILED! (hr)						// C 1086
    {
      DebPrint! (("DirectWrite HRESULT failed: (%d) CreateRenderingParams\n", hr));	// C 1088
      RELEASE_COM! (dwrite_factory);				// C 1089
      RELEASE_COM! (dwrite_factory2);				// C 1090
      RELEASE_COM! (gdi_interop);				// C 1091
      FreeLibrary (direct_write);				// C 1092
      eassert! (SUCCEEDED! (hr));				// C 1093
      return;							// C 1094
    }

  /* range: [0.0, 1.0] */
  if config_enhanced_contrast < 0.0 || config_enhanced_contrast > 1.0	// C 1098
    { config_enhanced_contrast = (*(*def).lpVtbl).GetEnhancedContrast.unwrap () (def); }	// C 1099

  /* range: [0.0, 1.0]  */
  if config_clear_type_level < 0.0 || config_clear_type_level > 1.0	// C 1102
     { config_clear_type_level = (*(*def).lpVtbl).GetClearTypeLevel.unwrap () (def); }	// C 1103

  /* range: (0.0, 256.0] */
  /* We change the default value of 2.2 for gamma to 1.4, that looks
     very similar to GDI.  The default looks too dim for emacs,
     subjectively.  */
  if config_gamma <= 0.0 || config_gamma > 256.0		// C 1109
    { config_gamma = 1.4; /* def->lpVtbl->GetGamma (def);  */ }	// C 1110

  hr = (*(*dwrite_factory).lpVtbl).CreateCustomRenderingParams.unwrap () (dwrite_factory,	// C 1112-1118
									 config_gamma,
									 config_enhanced_contrast,
									 config_clear_type_level,
									 core::mem::transmute::<c_int, DWRITE_PIXEL_GEOMETRY> ((*(*def).lpVtbl).GetPixelGeometry.unwrap () (def)),
									 RENDERING_MODE! (),
									 &raw mut rendering_params);

  RELEASE_COM! (def);						// C 1120

  if FAILED! (hr)						// C 1122
    {
      DebPrint! (("DirectWrite HRESULT failed: (%d)"
		  " CreateCustomRenderingParams\n", hr));	// C 1124-1125
      RELEASE_COM! (dwrite_factory);				// C 1126
      RELEASE_COM! (dwrite_factory2);				// C 1127
      RELEASE_COM! (gdi_interop);				// C 1128
      FreeLibrary (direct_write);				// C 1129
      eassert! (SUCCEEDED! (hr));				// C 1130
      return;							// C 1131
    }

  dwrite_geometry_sink.sink.lpVtbl = &raw const dwrite_geometry_sink_vtbl;	// C 1134

  /* C 1136-1146: `#ifdef MINGW_W64` … `#else` … `#endif`.  The `MINGW_W64` arm reaches the three
     base slots through `<d2d1.h>`'s embedded `IUnknownVtbl Base` member, which no file in this tree
     mirrors (`dwrite_3.h`/`d2d1.h` have no consumer here), so C's three `Base.`-qualified lines
     stand as this section's text; the `#else` arm is the one below, and it is the arm the port can
     spell (C's `(void *)` cast of a function pointer has no Rust counterpart - a direct `as` between
     two function-pointer types is `E0605` - so the three slots take `core::mem::transmute`).  */
  #[cfg (feature = "mingw_w64")]
  {
    // C 1137: dwrite_geometry_sink_vtbl.Base.AddRef = geometry_sink_AddRef;
    // C 1138: dwrite_geometry_sink_vtbl.Base.Release = geometry_sink_Release;
    // C 1139-1140: dwrite_geometry_sink_vtbl.Base.QueryInterface = geometry_sink_QueryInterface;
  }
  #[cfg (not (feature = "mingw_w64"))]
  {
    dwrite_geometry_sink_vtbl.AddRef = Some (core::mem::transmute (geometry_sink_AddRef));	// C 1142
    dwrite_geometry_sink_vtbl.Release = Some (core::mem::transmute (geometry_sink_Release));	// C 1143
    dwrite_geometry_sink_vtbl.QueryInterface
      = Some (core::mem::transmute (geometry_sink_QueryInterface));	// C 1144-1145
  }

  dwrite_geometry_sink_vtbl.AddBeziers = Some (geometry_sink_AddBeziers);	// C 1148
  dwrite_geometry_sink_vtbl.AddLines = Some (geometry_sink_AddLines);	// C 1149
  dwrite_geometry_sink_vtbl.BeginFigure = Some (geometry_sink_BeginFigure);	// C 1150
  dwrite_geometry_sink_vtbl.EndFigure = Some (geometry_sink_EndFigure);	// C 1151
  dwrite_geometry_sink_vtbl.Close = Some (geometry_sink_Close);	// C 1152
  dwrite_geometry_sink_vtbl.SetFillMode = Some (geometry_sink_SetFillMode);	// C 1153
  dwrite_geometry_sink_vtbl.SetSegmentFlags = Some (geometry_sink_SetSegmentFlags);	// C 1154

  direct_write_available = true;				// C 1156
  w32_inhibit_dwrite = false;					// C 1157
}

/// C 1160-1321: `w32_dwrite_draw`.  C's two `SAFE_ALLOCA`s, the four `SAFE_FREE` exits, the
/// `DWRITE_GLYPH_RUN` member-by-member fill, the `DWRITE_E_NOCOLOR` arm and the colour-layer
/// `for (;;)` loop are all here; the loop is PORTING.md 2.2's `loop { … break; }` shape (C's
/// `break` and `return`s keep their positions), and the `RGB` of C 1302-1304 is the stand-in
/// section's macro of the same name.
/// C: `y -= font->ascent;` … `int left_margin = metrics.lbearing < 0 ? -metrics.lbearing : 0;`
/// C: `BitBlt (text_dc, 0, 0, bitmap_width, bitmap_height, hdc, x - left_margin, y, SRCCOPY);`
/// C: `hr = dwrite_factory2->lpVtbl->TranslateColorGlyphRun (dwrite_factory2, left_margin, …);`
/// C: `if (hr == DWRITE_E_NOCOLOR) bitmap_render_target->lpVtbl->DrawGlyphRun (…);`
/// C: `hr = layers->lpVtbl->MoveNext (layers, &more_layers); if (!more_layers) break;`
/// C: `RGB (layer->runColor.r * 255, layer->runColor.g * 255, layer->runColor.b * 255)`
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_dwrite_draw (hdc: HDC, x: c_int, mut y: c_int,
					  glyphs: *mut c_uint, len: c_int,
					  color: COLORREF, font: *mut font) -> bool
{
  let mut hr: HRESULT = 0;					// C 1164
  let mut dwrite_font_face: *mut IDWriteFontFace = core::ptr::null_mut ();	// C 1165

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);			// C 1167

  let uniscribe_font: *mut uniscribe_font_info
    = font as *mut uniscribe_font_info;				// C 1169-1170

  /* What we get as y is the baseline position.  */
  y -= (*font).ascent;						// C 1173

  let font_size: c_float = get_font_face (font, &raw mut dwrite_font_face);	// C 1175
  if dwrite_font_face.is_null ()				// C 1176
    { return false; }

  let mut metrics: font_metrics = core::mem::zeroed ();		// C 1179
  if !text_extents_internal (dwrite_font_face, false, font_size, glyphs, len,	// C 1180-1181
			     &raw mut metrics)
    {
      (*uniscribe_font).dwrite_skip_font = true;		// C 1183
      return false;						// C 1184
    }

  let left_margin: c_int = if metrics.lbearing < 0		// C 1187
    { -(metrics.lbearing as c_int) } else { 0 };

  let bitmap_width: c_int = left_margin + metrics.width as c_int
    + metrics.rbearing as c_int;				// C 1189
  let bitmap_height: c_int = (*font).ascent + (*font).descent;	// C 1190

  /* We never release this, get_bitmap_render_target reuses it.  */
  let bitmap_render_target: *mut IDWriteBitmapRenderTarget
    = get_bitmap_render_target (hdc, bitmap_width, bitmap_height);	// C 1193-1194

  /* If this fails, completely disable DirectWrite.  */
  if bitmap_render_target.is_null ()				// C 1197
    {
      direct_write_available = false;				// C 1199
      return false;						// C 1200
    }

  /* This DC can't be released.  */
  let text_dc: HDC
    = (*(*bitmap_render_target).lpVtbl).GetMemoryDC.unwrap () (bitmap_render_target);	// C 1204-1205

  /* Copy the background pixel to the render target bitmap.  */
  BitBlt (text_dc, 0, 0, bitmap_width, bitmap_height, hdc, x - left_margin, y, SRCCOPY);	// C 1208

  let indices: *mut UINT16 = SAFE_ALLOCA! (sa_avail, len * size_of::<UINT16> () as c_int)
    as *mut UINT16;						// C 1210

  let mut i: c_int = 0;						// C 1212
  while i < len
    {
      *indices.offset (i as isize) = *glyphs.offset (i as isize) as UINT16;	// C 1213
      i += 1;
    }

  let advances: *mut FLOAT = SAFE_ALLOCA! (sa_avail, len * size_of::<FLOAT> () as c_int)
    as *mut FLOAT;						// C 1215

  let mut i: c_int = 0;						// C 1217
  while i < len
    {
      if !text_extents_internal (dwrite_font_face, false, font_size, glyphs.offset (i as isize), 1,	// C 1219-1220
				 &raw mut metrics)
	{
	  (*uniscribe_font).dwrite_skip_font = true;		// C 1222
	  SAFE_FREE! (sa_count, sa_mark);				// C 1223
	  return false;						// C 1224
	}
      *advances.offset (i as isize) = metrics.width as c_float;	// C 1226
      i += 1;
    }

  let mut glyph_run: DWRITE_GLYPH_RUN = core::mem::zeroed ();	// C 1229
  glyph_run.fontFace = dwrite_font_face;			// C 1230
  glyph_run.fontEmSize = font_size;				// C 1231
  glyph_run.glyphIndices = indices;				// C 1232
  glyph_run.glyphCount = len as UINT32;				// C 1233
  glyph_run.isSideways = false as WINBOOL;			// C 1234
  glyph_run.bidiLevel = 0;	/* we reorder bidi text ourselves */	// C 1235
  glyph_run.glyphOffsets = core::ptr::null ();			// C 1236
  glyph_run.glyphAdvances = advances;				// C 1237

  let mut layers: *mut IDWriteColorGlyphRunEnumerator = core::ptr::null_mut ();	// C 1239
  /* This call will tell us if we have to handle any color glyphs.  */
  hr = (*(*dwrite_factory2).lpVtbl).TranslateColorGlyphRun.unwrap ()	// C 1241-1248
    (dwrite_factory2,
     left_margin as FLOAT, (*font).ascent as FLOAT,
     &raw const glyph_run,
     core::ptr::null_mut (),
     MEASURING_MODE! (),
     core::ptr::null_mut (),
     0,
     &raw mut layers);

  /* No color.  Just draw the GlyphRun.  */
  if hr == DWRITE_E_NOCOLOR! ()					// C 1251
    {
      (*(*bitmap_render_target).lpVtbl).DrawGlyphRun.unwrap () (bitmap_render_target,	// C 1252-1258
							       left_margin as FLOAT,
							       (*font).ascent as FLOAT,
							       MEASURING_MODE! (),
							       &raw const glyph_run,
							       rendering_params,
							       color,
							       core::ptr::null_mut ());
    }
  else
    {
      /* If there were color glyphs, 'layers' contains a list of
	 GlyphRun with a color and a position for each.  We draw them
	 individually.  */
      if !verify_hr (hr, c"Failed at TranslateColorGlyphRun".as_ptr ())	// C 1264
	{
	  (*uniscribe_font).dwrite_skip_font = true;		// C 1266
	  RELEASE_COM! (layers);				// C 1267
	  SAFE_FREE! (sa_count, sa_mark);				// C 1268
	  return false;						// C 1269
	}
      loop							// C 1271
	{
	  let mut hr: HRESULT = 0;				// C 1273
	  let mut more_layers: BOOL = 0;			// C 1274
	  let mut layer: *const DWRITE_COLOR_GLYPH_RUN = core::ptr::null ();	// C 1275

	  hr = (*(*layers).lpVtbl).MoveNext.unwrap () (layers, &raw mut more_layers);	// C 1277
	  if !verify_hr (hr, c"Failed at MoveNext".as_ptr ())	// C 1278
	    {
	      (*uniscribe_font).dwrite_skip_font = true;	// C 1280
	      RELEASE_COM! (layers);				// C 1281
	      SAFE_FREE! (sa_count, sa_mark);				// C 1282
	      return false;					// C 1283
	    }
	  if more_layers == 0					// C 1285
	    { break; }						// C 1286
	  hr = (*(*layers).lpVtbl).GetCurrentRun.unwrap () (layers, &raw mut layer);	// C 1287
	  if !verify_hr (hr, c"Failed at GetCurrentRun".as_ptr ())	// C 1288
	    {
	      (*uniscribe_font).dwrite_skip_font = true;	// C 1290
	      RELEASE_COM! (layers);				// C 1291
	      SAFE_FREE! (sa_count, sa_mark);				// C 1292
	      return false;					// C 1293
	    }
	  hr = (*(*bitmap_render_target).lpVtbl).DrawGlyphRun.unwrap ()	// C 1295-1305
	    (bitmap_render_target,
	     (*layer).baselineOriginX,
	     (*layer).baselineOriginY,
	     MEASURING_MODE! (),
	     &raw const (*layer).glyphRun,
	     rendering_params,
	     RGB! ((*layer).runColor.r * 255.0,			// C 1302
		   (*layer).runColor.g * 255.0,
		   (*layer).runColor.b * 255.0),
	     core::ptr::null_mut ());
	  if !verify_hr (hr, c"Failed at GetCurrentRun".as_ptr ())	// C 1306
	    {
	      (*uniscribe_font).dwrite_skip_font = true;	// C 1308
	      RELEASE_COM! (layers);				// C 1309
	      SAFE_FREE! (sa_count, sa_mark);				// C 1310
	      return false;					// C 1311
	    }
	}
      RELEASE_COM! (layers);					// C 1314
    }

  /* Finally, copy the rendered text back to the original DC.  */
  BitBlt (hdc, x - left_margin, y, bitmap_width, bitmap_height, text_dc, 0, 0, SRCCOPY);	// C 1317-1318
  SAFE_FREE! (sa_count, sa_mark);					// C 1319
  return true;							// C 1320
}

/* C 1323-1328: "Returns true if DirectWrite is to be used: - It is available. - The font is
   handled by HarfBuzz. - w32-inhibit-dwrite is false. - The font has not been marked after a
   failed DirectWrite operation."  */

// C 1334's `harfbuzz_font_driver` is `font.h` 980's
// `extern struct font_driver harfbuzz_font_driver;`, defined in `src/w32uniscribe.rs` (C:
// w32uniscribe.c 56).  `include/font.rs` carries no row for it yet, so this unit declares it in
// place (P-88 (1); the ledger's `## Owed` names the mirror row).  C 1332's `#ifdef HAVE_HARFBUZZ`
// is the only predicate that reaches it, so the declaration carries that feature too.
// C: font.h 980: extern struct font_driver harfbuzz_font_driver;
#[cfg (all (feature = "have_ntgui", feature = "harfbuzz"))]
unsafe extern "C"
{
  pub static mut harfbuzz_font_driver: font_driver;
}

/// C 1329-1340: `w32_use_direct_write`.  C's four-way test is kept in C's order and C's
/// `#ifdef HAVE_HARFBUZZ` pair is carried (`feature = "harfbuzz"` / its negation); the driver
/// pointer comparison spells C's `&harfbuzz_font_driver` as `&raw const`.
/// C: bool w32_use_direct_write (struct w32font_info *w32font) { #ifdef HAVE_HARFBUZZ
/// C:   return (direct_write_available && w32font->font.driver == &harfbuzz_font_driver
/// C:   && !w32_inhibit_dwrite && !((struct uniscribe_font_info *) w32font)->dwrite_skip_font);
/// C:   #else return false; #endif }
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_use_direct_write (w32font: *mut w32font_info) -> bool
{
  #[cfg (feature = "harfbuzz")]
  { return (direct_write_available					// C 1333
	    && (*w32font).font.driver == &raw const harfbuzz_font_driver as *const c_void	// C 1334
	    && !w32_inhibit_dwrite					// C 1335
	    && !(*((w32font) as *mut uniscribe_font_info)).dwrite_skip_font); }	// C 1336
  #[cfg (not (feature = "harfbuzz"))]
  { return false; }							// C 1338
}

// C 1342-1348: the first subr.  C's `direct_write_available ? Qt : Qnil` is the port's `if`; the
// docstring is C's comment text without the delimiters (the trailing spaces C's `*/` follows are
// not part of the documented string).
// C: DEFUN ("w32-dwrite-available", Fw32_dwrite_available, Sw32_dwrite_available, 0, 0, 0, doc:
// C:   /* Returns t if DirectWrite is available.  DirectWrite will be used if it is available and
// C:   'w32-inhibit-dwrite' is nil.  */) (void) { return direct_write_available ? Qt : Qnil; }
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-dwrite-available", Fw32_dwrite_available, Sw32_dwrite_available, 0, 0, 0,
       doc: "Returns t if DirectWrite is available.\nDirectWrite will be used if it is available and 'w32-inhibit-dwrite' is nil.",
       (),
       {
  return if direct_write_available { Qt () } else { Qnil () };	// C 1346
});

// C 1350-1385: the second subr, `w32-dwrite-reinit`.  Each parameter's `FLOATP`/`FIXNUMP` pair
// assigns through the config variable (C's `XFLOAT_DATA` is the port's `f64`, so the assignment
// carries the `c_float` narrowing C gets implicitly), then C 1382 reinitialises DirectWrite.
// C: DEFUN ("w32-dwrite-reinit", Fw32_dwrite_reinit, Sw32_dwrite_reinit, 0, 3, 0, doc: /* … */)
// C:   (Lisp_Object enhanced_contrast, Lisp_Object clear_type_level, Lisp_Object gamma)
// C: { config_enhanced_contrast = -1.0f; if (FLOATP (enhanced_contrast)) … ; w32_initialize_direct_write (); }
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-dwrite-reinit", Fw32_dwrite_reinit, Sw32_dwrite_reinit, 0, 3, 0,
       doc: "Reinitialize DirectWrite with the given parameters.\nIf a parameter is not specified, or is out of range, it will take a default\nvalue.\n\nReturn value is nil.\n\nENHANCED_CONTRAST is in the range [0.0, 1.0], and defaults to 0.5.\nCLEAR_TYPE_LEVEL is in the range [0.0, 1.0], and defaults to 1.0.\nGAMMA is in the range (0.0, 256.0], and defaults to a system-dependent value\n      around 2.0 (sometimes 1.8, sometimes 2.2).",
       (enhanced_contrast: Lisp_Object, clear_type_level: Lisp_Object, gamma: Lisp_Object),
       {
  config_enhanced_contrast = -1.0f32;				// C 1361
  if FLOATP (enhanced_contrast)					// C 1362
    { config_enhanced_contrast = XFLOAT_DATA (enhanced_contrast) as c_float; }	// C 1363
  if FIXNUMP (enhanced_contrast)				// C 1364
    { config_enhanced_contrast = XFIXNUM (enhanced_contrast) as c_float; }	// C 1365

  config_clear_type_level = -1.0f32;				// C 1367
  if FLOATP (clear_type_level)					// C 1368
    { config_clear_type_level = XFLOAT_DATA (clear_type_level) as c_float; }	// C 1369
  if FIXNUMP (clear_type_level)					// C 1370
    { config_clear_type_level = XFIXNUM (clear_type_level) as c_float; }	// C 1371

  config_gamma = -1.0f32;					// C 1373
  if FLOATP (gamma)						// C 1374
    { config_gamma = XFLOAT_DATA (gamma) as c_float; }		// C 1375
  if FIXNUMP (gamma)						// C 1376
    { config_gamma = XFIXNUM (gamma) as c_float; }		// C 1377

  w32_initialize_direct_write ();				// C 1379

  return Qnil ();						// C 1381
});

/// C 1390's `DEFVAR_BOOL` storage.  C's own definition is the field
/// `globals.f_w32_inhibit_dwrite` of the single `struct emacs_globals globals` (`src/alloc.c`), so
/// C's text has no definition line for it; the port keeps the variable in its own unit, the
/// `check-globals.py` rule every `DEFVAR_*` unit follows (`src/buffer.rs` 4949's shape), and the
/// name is the mirror's `include/globals.rs` 7156 declaration.  C also reads it at C 1335 and
/// clears it at C 1157.
/// C: DEFVAR_BOOL ("w32-inhibit-dwrite", w32_inhibit_dwrite, doc: /* If t, don't use DirectWrite.  */);
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_inhibit_dwrite: bool = false;

/// C 1387-1399: `syms_of_w32dwrite`.  C's `DEFVAR_BOOL` is the mirror's macro (its `doc:` is the
/// C string literal), the two `defsubr` calls take the two records the `DEFUN!`s above define, and
/// the `w32_inhibit_dwrite = false` between them is C's comment's subject: "The actual value is
/// determined at startup in w32_initialize_direct_write, which is called from
/// syms_of_w32uniscribe_for_pdumper."
/// C: void syms_of_w32dwrite (void) { DEFVAR_BOOL ("w32-inhibit-dwrite", w32_inhibit_dwrite, …);
/// C:   w32_inhibit_dwrite = false; defsubr (&Sw32_dwrite_reinit); defsubr (&Sw32_dwrite_available); }
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_w32dwrite ()
{
  DEFVAR_BOOL! (c"w32-inhibit-dwrite", w32_inhibit_dwrite,	// C 1390
		c"If t, don't use DirectWrite.  ");
  /* The actual value is determined at startup in
     w32_initialize_direct_write, which is called from
     syms_of_w32uniscribe_for_pdumper.  */
  w32_inhibit_dwrite = false;					// C 1395

  defsubr (&raw mut Sw32_dwrite_reinit);			// C 1397
  defsubr (&raw mut Sw32_dwrite_available);			// C 1398
}
