// femacs: port of src/w32image.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `w32image.o`
// joins `W32_OBJ` in `configure.ac` 3051 - the non-Cygwin arm of the split inside the `HAVE_W32`
// test (3019-3030) whose 3024 `AC_DEFINE`s `HAVE_NTGUI` - and `docs/baseline/frozen-build-sources.txt`
// lists no `w32image.o` for this target, so `extract-abi.py`'s checklist for the file is empty
// (P-86 (2)) and the ledger `docs/ports/w32image.md` carries that clause's substitute: the **C
// line-range table**, the `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "windowsnt"`, on every item *and on the include graph*
//     (P-86 (8)), which is why the frozen configuration compiles nothing here.  It is `WINDOWSNT`,
//     not `HAVE_NTGUI`: `HAVE_NTGUI` is on for *both* the Cygwin and the native Windows GUI build
//     (3024), while `w32image.o` joins `W32_OBJ` only in the non-Cygwin arm (3051) - the build
//     whose `nt/inc/ms-w32.h` 23-25 `#define`s `WINDOWSNT`.  The units of that same arm
//     (`src/w32.rs`, `src/w32heap.rs`, `src/w32proc.rs`, `src/w32console.rs`, `src/w32inevt.rs`)
//     already landed with this gate, so this one does too;
//   * C 26-29's `#ifdef MINGW_W64` around `#include <objidl.h>` is the file's only `MINGW_W64`
//     condition, and it names no name this unit spells (the COM stream comes from the SDK headers
//     in either arm), so the three C lines are kept and the names it supplies stand in the
//     `<objidl.h>` section below under the file gate;
//   * C 35-37's `#ifdef WINDOWSNT` around `#include "w32.h"` is the file's own arm, so the edge
//     needs no feature of its own (the file gate *is* that arm - the `src/w32console.rs` reading of
//     its own C 42-44);
//   * C 42-207's `#ifdef WINDOWSNT` is that arm too: the 28 loaded entry-point objects and
//     `gdiplus_init` are live text here and C 207 is its `#endif`;
//   * C 224-229's `#ifdef WINDOWSNT` is the same macro once more, so the `#ifdef` arm is code and
//     C 227-229's `#else` arm is carried as C's own lines (the file gate already selects the arm).
//
// **Acquisition note - this piece is `text-only` (P-86 (3)(ii)).**  The machine has no Windows and
// no mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty,
// no Homebrew `mingw-w64`, and `find` finds no `windows.h`/`gdiplus.h`/`shlwapi.h`), so there is
// **no `<windows.h>` and no GDI+ header to compile or measure against**, no arm flip on a Windows
// target and no layout measurement: every record, every value and every prototype of the
// `<windows.h>`/`<gdiplus.h>` stand-in sections below is `UNVERIFIED` (the documented SDK text, not
// a measurement) and the layout assertions are owed (the ledger's `## Owed`).  What the round does
// carry is the frozen `build-tu` reading, the machine-checked coverage table, the `#if` verdicts, a
// per-item C citation and the per-file error census of the `windowsnt` + `have_ntgui` +
// `have_window_system` arm.
//
// `docs/ports/w32image.md` carries the C line ranges, the acceptance readings, every deviation
// (the dropped `extern` declarations, the two SDK stand-in sections, the enum spelling, the
// `switch` -> `match` body, the `IStream_Release` and `MAKEINTRESOURCEA` macros, and the two
// declaration rows the mirrors leave) and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, unused_must_use,
         let_underscore_drop, clippy::all)]

/* The block C 1-18 closes is the file's opening ownership/title comment, dropped (PORTING.md
   0.1-19); the port's first line is the revision line above.

   C 20 is the file's own second comment - its one author.  C's words are carried:  */

/* Written by Juan Jose Garcia-Ripoll <juanjose.garciaripoll@gmail.com>.  */

/* C 22-41: the include graph.  `w32image.o` is built only by the native Windows GUI configuration,
   so the whole graph sits under the file gate (P-86 (8): with the includes outside it the frozen
   compile immediately eats `E0428`s).  C 26-32's `<objidl.h>`/`<wtypes.h>`/`<gdiplus.h>`/
   `<shlwapi.h>` have no mirror in this tree (P-88 (1): the declaration site is C's own `#include`
   line, and no file is added under `femacs/src/include/`), so the names this file takes from them
   stand in the sections below; the other edges are the mirrors, whose own nested `include!`s
   `xtask` expands and dedupes (PORTING.md 0.1-4).  */
#[cfg (feature = "windowsnt")]
include!("include/config.rs");                  // C 22: `#include <config.h>`
/* C 22's AH_BOTTOM: `#include <conf_post.h>` (`eassert`, the `NDEBUG` guards).  */
#[cfg (feature = "windowsnt")]
include!("include/conf_post.rs");
#[cfg (feature = "windowsnt")]
include!("../lib/include/string.rs");           // lisp.h 30: `#include <string.h>` (`memset ()`)
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdlib.rs");           // ms-w32.h 410: `#include <stdlib.h>` (`malloc`, `free`)
#[cfg (feature = "windowsnt")]
include!("include/lisp.rs");                    // C 23: `#include "lisp.h"`
#[cfg (feature = "windowsnt")]
include!("include/dispextern.rs");              // C 24: `#include "dispextern.h"`
/* C 26-29: `#ifdef MINGW_W64` / the FIXME line / `# include <objidl.h>` / `#endif`.  `objidl.h` is
   the Windows SDK's and has no mirror, so the edge carries no file; C's four lines are kept and
   the names the header supplies this file (`IStream`, C 432/436/437) stand in the `<objidl.h>`
   section below.  Both arms need them - the macro only decides which SDK header spells them - and
   `MINGW_W64` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 3914, so C's `#else`
   side is the frozen one.  */
// C: #ifdef MINGW_W64
// C: /* FIXME: Do we need to include objidl.h?  */
// C: #include <objidl.h>
// C: #endif
#[cfg (feature = "windowsnt")]
include!("include/w32common.rs");               // C 33: `#include "w32common.h"`
#[cfg (feature = "windowsnt")]
include!("include/w32term.rs");                 // C 34: `#include "w32term.h"`
#[cfg (feature = "windowsnt")]
include!("include/w32.rs");                     // C 36: `#include "w32.h"` (`map_w32_filename`, `filename_to_utf16`)
#[cfg (feature = "windowsnt")]
include!("include/frame.rs");                   // C 38: `#include "frame.h"`
#[cfg (feature = "windowsnt")]
include!("include/coding.rs");                  // C 39: `#include "coding.h"` (`ENCODE_FILE`)
#[cfg (feature = "windowsnt")]
include!("include/w32gdiplus.rs");              // C 41: `#include "w32gdiplus.h"` (the `Gdip*` entry-point macros)

/* ----------------------------------------------------------------------------------------------
   C 30-32: `#include <wtypes.h>`, `<gdiplus.h>`, `<shlwapi.h>` (with C 26-29's `<objidl.h>`).

   No file in the reference tree carries any of the four (P-88 (1)), so the names *this file* and
   the two mirrors it includes (`include/w32gdiplus.rs`, whose own head note leaves its GDI+ and
   `<windows.h>` names to the includer, and `include/w32term.rs`) take from them stand here, each
   with the SDK's own text above it.  The spellings repeat `src/w32select.rs`'s section so that a
   Windows translation unit sees one set of names: `DWORD` is the 32-bit Windows unsigned whatever
   this target's `c_long` is, the handles are all `void *` at the ABI, `BOOL` is C's `int`, and
   `WINAPI`/`WINGDIPAPI` is the `extern "system"` ABI Rust spells `__stdcall` with.

   **No measurement was possible**: this machine has no `<windows.h>` (see the file note), so every
   scalar, record, value and prototype below is **UNVERIFIED** - its size, offsets and values are
   the documented ones and the layout assertions are owed (the ledger's `## Owed`).
   ---------------------------------------------------------------------------------------------- */

/// C 30's `<wtypes.h>`: the scalar types (`windef.h`/`winnt.h`).  C's `INT` is `windef.h`'s
/// `int` spelling and `wchar_t` is the Windows **16-bit** wide character (`include/w32gdiplus.rs`
/// spells it `WCHAR`); the frozen target's own `<wchar.h>` mirror, `lib/include/wchar.rs`, is
/// glibc's 32-bit one and is not on this file's graph (the `src/w32inevt.rs` 102-104 reading).
/// C: typedef unsigned char BYTE; typedef unsigned short WORD; typedef unsigned short WCHAR;
/// C: typedef unsigned long DWORD; typedef long LONG; typedef unsigned long ULONG;
/// C: typedef unsigned int UINT; typedef int BOOL; typedef int INT; typedef void VOID;
#[cfg (feature = "windowsnt")]
pub type BYTE = u8;
#[cfg (feature = "windowsnt")]
pub type WORD = c_ushort;
#[cfg (feature = "windowsnt")]
pub type WCHAR = c_ushort;
#[cfg (feature = "windowsnt")]
pub type wchar_t = c_ushort;
#[cfg (feature = "windowsnt")]
pub type DWORD = u32;
#[cfg (feature = "windowsnt")]
pub type LONG = i32;
#[cfg (feature = "windowsnt")]
pub type ULONG = u32;
#[cfg (feature = "windowsnt")]
pub type UINT = c_uint;
#[cfg (feature = "windowsnt")]
pub type BOOL = c_int;
#[cfg (feature = "windowsnt")]
pub type INT = c_int;
#[cfg (feature = "windowsnt")]
pub type VOID = c_void;
/// C 30's `<wtypes.h>`: the two truth values.  C's `FALSE`/`TRUE` are `<windef.h>`'s two names.
/// C: #define FALSE 0
/// C: #define TRUE 1
#[cfg (feature = "windowsnt")]
pub const FALSE: BOOL = 0;
#[cfg (feature = "windowsnt")]
pub const TRUE: BOOL = 1;
/// C 30's `<wtypes.h>`: the handle names this file and the GDI+ mirror spell.  Each is `void *`
/// at the ABI (the `include/w32gui.rs`/`src/w32.rs` reading), so the implicit conversions at the
/// call sites need no cast.
/// C: typedef void *HANDLE; typedef void *HWND; typedef void *HDC; typedef void *HBITMAP;
/// C: typedef void *HGDIOBJ; typedef void *HPALETTE; typedef void *LPVOID;
/// C: typedef const char *LPCSTR; typedef ULONG_PTR *PULONG_PTR;
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HWND = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HDC = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HBITMAP = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HGDIOBJ = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HPALETTE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPCSTR = *const c_char;
#[cfg (feature = "windowsnt")]
pub type ULONG_PTR = usize;
/// C 30's `<wtypes.h>`: `MAX_PATH` (`windef.h`'s path limit, the `src/w32select.rs` 229 and
/// `src/sound.rs` 439 spelling).
/// C: #define MAX_PATH 260
#[cfg (feature = "windowsnt")]
pub const MAX_PATH: usize = 260;

/// C 26-30's `<objidl.h>`/`<wtypes.h>`: `GUID` and its `CLSID`/`IID` aliases (`guiddef.h`'s
/// record), the identity C 358/371/490/515/558/559/686/689 spell.  **UNVERIFIED**.
/// C: typedef struct _GUID { unsigned long Data1; unsigned short Data2; unsigned short Data3;
/// C:   unsigned char Data4[8]; } GUID; typedef GUID CLSID; typedef GUID IID;
/// C: typedef long HRESULT;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID
{
  pub Data1: c_ulong,
  pub Data2: c_ushort,
  pub Data3: c_ushort,
  pub Data4: [c_uchar; 8],
}
#[cfg (feature = "windowsnt")]
pub type CLSID = GUID;
#[cfg (feature = "windowsnt")]
pub type IID = GUID;
#[cfg (feature = "windowsnt")]
pub type HRESULT = c_long;

/// C 26-29's `<objidl.h>`: the COM stream `SHCreateMemStream` hands back and C 437 releases.
/// C 25's `COBJMACROS` is what asks for the `IStream_*` C spellings, and the one this file uses is
/// the macro below; the record and its vtable are **UNVERIFIED** (no COM header on this machine).
/// C: <objidl.h>: typedef struct IStream IStream;
/// C: <unknwn.h>: struct IUnknown { CONST_VTBL IUnknownVtbl *lpVtbl; };
/// C: ULONG (STDMETHODCALLTYPE *AddRef) (IUnknown *This);
/// C: ULONG (STDMETHODCALLTYPE *Release) (IUnknown *This);
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct IStream
{
  pub lpVtbl: *const IStreamVtbl,
}
/// The vtable's first three slots are `IUnknown`'s (`QueryInterface`, `AddRef`, `Release`) and
/// only `Release` is reached here (C 437), so the rest of `<objidl.h>`'s `IStreamVtbl` is not
/// written - a reader arrives at a slot by offset, and the three below keep C's order.
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct IStreamVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (*mut IStream, *const IID,
                                                        *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (*mut IStream) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (*mut IStream) -> ULONG>,
}
/// C 25's `#define COBJMACROS` asks the COM headers for the C spellings; C 437's
/// `IStream_Release (pStream)` is `<objidl.h>`'s `#define IStream_Release(This)
/// (This)->lpVtbl->Release(This)`, so the port spells that one macro's body here.
/// C: #define IStream_Release(This) (This)->lpVtbl->Release(This)
#[cfg (feature = "windowsnt")]
macro_rules! IStream_Release {
  ($this:expr) => { ((*((*($this)).lpVtbl)).Release.unwrap ()) ($this) };
}

/// C 31's `<gdiplus.h>` (and C 41's `w32gdiplus.h`, whose mirror leaves the GDI+ types to its
/// includer - its own head note says so).  `GpStatus` is the SDK's `Status` enum, which
/// `src/w32select.rs` 412/483 models the same way (`type` + constants) because the values C
/// compares against are data; the handle records are incomplete types C only ever points at.
/// C: typedef INT GpStatus; typedef enum Status { Ok = 0, GenericError, InvalidParameter, ... } Status;
/// C: typedef struct GpImage GpImage; typedef struct GpGraphics GpGraphics;
/// C: typedef struct GpBitmap GpBitmap; typedef struct GpImageAttributes GpImageAttributes;
/// C: typedef UINT32 ARGB; typedef UINT32 PROPID; typedef INT PixelFormat; typedef INT InterpolationMode;
/// C: typedef INT GpUnit; typedef INT RotateFlipType;
/// C: typedef VOID (*DrawImageAbort) (VOID *); typedef BOOL (*GetThumbnailImageAbort) (VOID *);
#[cfg (feature = "windowsnt")]
pub type GpStatus = c_int;
#[cfg (feature = "windowsnt")]
pub type GpImage = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type GpGraphics = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type GpBitmap = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type GpImageAttributes = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type ARGB = u32;
#[cfg (feature = "windowsnt")]
pub type PROPID = u32;
#[cfg (feature = "windowsnt")]
pub type PixelFormat = c_int;
#[cfg (feature = "windowsnt")]
pub type InterpolationMode = c_int;
#[cfg (feature = "windowsnt")]
pub type GpUnit = c_int;
#[cfg (feature = "windowsnt")]
pub type RotateFlipType = c_int;
#[cfg (feature = "windowsnt")]
pub type DrawImageAbort = Option<unsafe extern "system" fn (*mut c_void) -> BOOL>;
#[cfg (feature = "windowsnt")]
pub type GetThumbnailImageAbort = Option<unsafe extern "system" fn (*mut c_void) -> BOOL>;
/// C 31's `Status` members this file names (C 238/240/330/373/415/442/450/459/465/624/636/678/687/
/// 702/705): the three `status == Ok` tests, `GenericError` (C 415) and `InvalidParameter` (C 703).
/// C: <gdiplus.h>: Ok = 0, GenericError = 1, InvalidParameter = 2
#[cfg (feature = "windowsnt")]
pub const Ok: GpStatus = 0;
#[cfg (feature = "windowsnt")]
pub const GenericError: GpStatus = 1;
#[cfg (feature = "windowsnt")]
pub const InvalidParameter: GpStatus = 2;
/// C 31's `<gdiplus.h>`: `PropertyTagFrameDelay` (`GdiplusImaging.h`'s `#define`) and the frame
/// dimension the SDK's `FrameDimensionTime` names, which C 218/327/337 read.
/// C: #define PropertyTagFrameDelay ((PROPID) 0x5100)
#[cfg (feature = "windowsnt")]
pub const PropertyTagFrameDelay: PROPID = 0x5100;

/// C 31's `<gdiplus.h>`: the records this file reads, writes or embeds **by value**.
/// **UNVERIFIED** (no `gdiplus.h` on this machine; the layout assertions are owed).
/// C: typedef VOID (CALLBACK *DebugEventProc) (DebugEventLevel, CHAR *);
/// C: typedef struct GdiplusStartupInput { UINT32 GdiplusVersion; DebugEventProc DebugEventCallback;
/// C:   BOOL SuppressBackgroundThread; BOOL SuppressExternalCodecs; } GdiplusStartupInput;
#[cfg (feature = "windowsnt")]
pub type DebugEventLevel = c_int;
#[cfg (feature = "windowsnt")]
pub type DebugEventProc = Option<unsafe extern "system" fn (DebugEventLevel, *const c_char)>;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GdiplusStartupInput
{
  pub GdiplusVersion: u32,
  pub DebugEventCallback: DebugEventProc,
  pub SuppressBackgroundThread: BOOL,
  pub SuppressExternalCodecs: BOOL,
}
/// C: typedef VOID (CALLBACK *NotificationHookProc) (ULONG_PTR *);
/// C: typedef VOID (CALLBACK *NotificationUnhookProc) (ULONG_PTR);
/// C: typedef struct GdiplusStartupOutput { NotificationHookProc NotificationHook;
/// C:   NotificationUnhookProc NotificationUnhook; } GdiplusStartupOutput;
#[cfg (feature = "windowsnt")]
pub type NotificationHookProc = Option<unsafe extern "system" fn (*mut ULONG_PTR) -> GpStatus>;
#[cfg (feature = "windowsnt")]
pub type NotificationUnhookProc = Option<unsafe extern "system" fn (ULONG_PTR)>;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GdiplusStartupOutput
{
  pub NotificationHook: NotificationHookProc,
  pub NotificationUnhook: NotificationUnhookProc,
}
/// C: typedef struct PropertyItem { PROPID id; ULONG length; WORD type; VOID *value; } PropertyItem;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PropertyItem
{
  pub id: PROPID,
  pub length: ULONG,
  pub r#type: WORD,
  pub value: *mut c_void,
}
/// C: typedef struct ImageCodecInfo { CLSID Clsid; GUID FormatID; const WCHAR *CodecName;
/// C:   const WCHAR *DllName; const WCHAR *FormatDescription; const WCHAR *FilenameExtension;
/// C:   const WCHAR *MimeType; DWORD Flags; DWORD Version; DWORD SigCount; DWORD SigSize;
/// C:   const BYTE *SigPattern; const BYTE *SigMask; } ImageCodecInfo;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageCodecInfo
{
  pub Clsid: CLSID,
  pub FormatID: GUID,
  pub CodecName: *const WCHAR,
  pub DllName: *const WCHAR,
  pub FormatDescription: *const WCHAR,
  pub FilenameExtension: *const WCHAR,
  pub MimeType: *const WCHAR,
  pub Flags: DWORD,
  pub Version: DWORD,
  pub SigCount: DWORD,
  pub SigSize: DWORD,
  pub SigPattern: *const BYTE,
  pub SigMask: *const BYTE,
}
/// C: typedef struct EncoderParameters { UINT Count; EncoderParameter Parameter[1]; } EncoderParameters;
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct EncoderParameters
{
  _private: [u8; 0],
}

/// C 30-32's `<windows.h>` chain: the Win32 entry points `w32_gdip_export_frame` calls, one
/// `unsafe extern "system"` block per header group (`WINAPI` is that block's ABI).  `CALLBACK`
/// and `WINGDIPAPI` are the same `__stdcall` spelling Rust names `extern "system"`, so the GDI+
/// pointers of `include/w32gdiplus.rs` and these have one convention.
/// C: BOOL WINAPI GetWindowRect (HWND, LPRECT); HDC WINAPI GetWindowDC (HWND);
/// C: int WINAPI GetDeviceCaps (HDC, int); HDC WINAPI CreateCompatibleDC (HDC);
/// C: int WINAPI SaveDC (HDC); HGDIOBJ WINAPI SelectObject (HDC, HGDIOBJ);
/// C: BOOL WINAPI BitBlt (HDC, int, int, int, int, HDC, int, int, DWORD);
/// C: BOOL WINAPI RestoreDC (HDC, int); BOOL WINAPI DeleteDC (HDC);
/// C: HBITMAP WINAPI CreateDIBSection (HDC, const BITMAPINFO *, UINT, VOID **, HANDLE, DWORD);
/// C: BOOL WINAPI DeleteObject (HGDIOBJ);
#[cfg (feature = "windowsnt")]
unsafe extern "system"
{
  pub fn GetWindowRect (hwnd: HWND, lprect: *mut RECT) -> BOOL;
  pub fn GetWindowDC (hwnd: HWND) -> HDC;
  pub fn GetDeviceCaps (hdc: HDC, index: c_int) -> c_int;
  pub fn CreateCompatibleDC (hdc: HDC) -> HDC;
  pub fn SaveDC (hdc: HDC) -> c_int;
  pub fn SelectObject (hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
  pub fn BitBlt (hdcDest: HDC, x: c_int, y: c_int, cx: c_int, cy: c_int, hdcSrc: HDC,
                 x1: c_int, y1: c_int, rop: DWORD) -> BOOL;
  pub fn RestoreDC (hdc: HDC, nSavedDC: c_int) -> BOOL;
  pub fn DeleteDC (hdc: HDC) -> BOOL;
  pub fn CreateDIBSection (hdc: HDC, pbmi: *const BITMAPINFO, usage: UINT,
                           ppvBits: *mut *mut c_void, hSection: HANDLE,
                           offset: DWORD) -> HBITMAP;
  pub fn DeleteObject (ho: HGDIOBJ) -> BOOL;
}

/// C 30-32's `<windows.h>` chain: the DIB records and the codes C 595-612 reads, plus the `RECT`
/// C 571 reads and the `BITSPIXEL` index C 591 asks `GetDeviceCaps` for.  **UNVERIFIED**.
/// C: typedef struct tagBITMAPINFOHEADER { DWORD biSize; LONG biWidth; LONG biHeight; WORD biPlanes;
/// C:   WORD biBitCount; DWORD biCompression; DWORD biSizeImage; LONG biXPelsPerMeter;
/// C:   LONG biYPelsPerMeter; DWORD biClrUsed; DWORD biClrImportant; } BITMAPINFOHEADER;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BITMAPINFOHEADER
{
  pub biSize: DWORD,
  pub biWidth: LONG,
  pub biHeight: LONG,
  pub biPlanes: WORD,
  pub biBitCount: WORD,
  pub biCompression: DWORD,
  pub biSizeImage: DWORD,
  pub biXPelsPerMeter: LONG,
  pub biYPelsPerMeter: LONG,
  pub biClrUsed: DWORD,
  pub biClrImportant: DWORD,
}
/// C: typedef struct tagRGBQUAD { BYTE rgbBlue; BYTE rgbGreen; BYTE rgbRed;
/// C:   BYTE rgbReserved; } RGBQUAD;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RGBQUAD
{
  pub rgbBlue: BYTE,
  pub rgbGreen: BYTE,
  pub rgbRed: BYTE,
  pub rgbReserved: BYTE,
}
/// C: typedef struct tagBITMAPINFO { BITMAPINFOHEADER bmiHeader; RGBQUAD bmiColors[1]; } BITMAPINFO;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BITMAPINFO
{
  pub bmiHeader: BITMAPINFOHEADER,
  pub bmiColors: [RGBQUAD; 1],
}
/// C: typedef struct tagRECT { LONG left; LONG top; LONG right; LONG bottom; } RECT;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RECT
{
  pub left: LONG,
  pub top: LONG,
  pub right: LONG,
  pub bottom: LONG,
}
/// C: #define BI_RGB 0
/// C: #define DIB_PAL_COLORS 1
/// C: #define SRCCOPY (DWORD)0x00CC0020
/// C: #define BITSPIXEL 12
#[cfg (feature = "windowsnt")]
pub const BI_RGB: DWORD = 0;
#[cfg (feature = "windowsnt")]
pub const DIB_PAL_COLORS: UINT = 1;
#[cfg (feature = "windowsnt")]
pub const SRCCOPY: DWORD = 0x00CC0020;
#[cfg (feature = "windowsnt")]
pub const BITSPIXEL: c_int = 12;

/// C 32's `<shlwapi.h>`: `SHCreateMemStream`'s ordinal form.  The header documents
/// `MAKEINTRESOURCEA` (`winuser.h`'s macro) as the way to name it on Windows before Vista, and
/// C 171 spells that call - so the macro's body is written here rather than a bare cast.
/// C: #define MAKEINTRESOURCEA(i) ((LPSTR) (ULONG_PTR) ((WORD) (i)))
#[cfg (feature = "windowsnt")]
macro_rules! MAKEINTRESOURCEA {
  ($i:expr) => { ($i as ULONG_PTR) as *const c_char };
}

/* C 25: `#define COBJMACROS` - the COM headers' C spellings (the `src/w32fns.rs` 95-97 reading:
   the macro selects them, and the port has no COM mirror to select from).  */
/// C: #define COBJMACROS
#[cfg (feature = "windowsnt")]
pub const COBJMACROS: () = ();

/* C 30-32's system headers: the two string comparisons and the image format C 519/529/551 uses,
   neither of which any mirror on this graph declares (`lib/include/string.rs` has `strcmp` and
   `memset`; the tree's `<wchar.h>` mirror, `lib/include/wchar.rs`, is not reachable from here and
   carries no `wcscmp`), so both stand where C's include chain puts them (P-88 (1)).  */
/// C 519/529's `stricmp`: MinGW's `<string.h>` case-insensitive compare (`_stricmp`'s alias).
/// C: int stricmp (const char *, const char *);
#[cfg (feature = "windowsnt")]
unsafe extern "C"
{
  pub fn stricmp (s1: *const c_char, s2: *const c_char) -> c_int;
}
/// C 551's `wcscmp`: the wide-character compare of the same header pair.
/// C: int wcscmp (const wchar_t *, const wchar_t *);
#[cfg (feature = "windowsnt")]
unsafe extern "C"
{
  pub fn wcscmp (s1: *const wchar_t, s2: *const wchar_t) -> c_int;
}

/* C 42-207: `#ifdef WINDOWSNT`.  The file gate *is* that arm (see the file note), so the 28
   loaded entry-point objects and `gdiplus_init` are live text and C 207 is the `#endif`.  C
   declares the objects once each; C 70 repeats C 58's `fn_GdipCreateBitmapFromHBITMAP` word for
   word, so the port writes it once, at C 58's place in the order, and records the repeat here
   (`include/w32gdiplus.rs` 114-115 makes the same pair-of-lines reading for the typedef).  */
/// C 43: `GdiplusStartup_Proc fn_GdiplusStartup;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdiplusStartup: GdiplusStartup_Proc = None;
/// C 44: `GdiplusShutdown_Proc fn_GdiplusShutdown;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdiplusShutdown: GdiplusShutdown_Proc = None;
/// C 45: `GdipCreateFromHDC_Proc fn_GdipCreateFromHDC;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipCreateFromHDC: GdipCreateFromHDC_Proc = None;
/// C 46: `GdipDeleteGraphics_Proc fn_GdipDeleteGraphics;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipDeleteGraphics: GdipDeleteGraphics_Proc = None;
/// C 47: `GdipGetPropertyItemSize_Proc fn_GdipGetPropertyItemSize;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipGetPropertyItemSize: GdipGetPropertyItemSize_Proc = None;
/// C 48: `GdipGetPropertyItem_Proc fn_GdipGetPropertyItem;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipGetPropertyItem: GdipGetPropertyItem_Proc = None;
/// C 49: `GdipImageGetFrameDimensionsCount_Proc fn_GdipImageGetFrameDimensionsCount;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipImageGetFrameDimensionsCount: GdipImageGetFrameDimensionsCount_Proc = None;
/// C 50: `GdipImageGetFrameDimensionsList_Proc fn_GdipImageGetFrameDimensionsList;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipImageGetFrameDimensionsList: GdipImageGetFrameDimensionsList_Proc = None;
/// C 51: `GdipImageGetFrameCount_Proc fn_GdipImageGetFrameCount;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipImageGetFrameCount: GdipImageGetFrameCount_Proc = None;
/// C 52: `GdipImageSelectActiveFrame_Proc fn_GdipImageSelectActiveFrame;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipImageSelectActiveFrame: GdipImageSelectActiveFrame_Proc = None;
/// C 53: `GdipCreateBitmapFromFile_Proc fn_GdipCreateBitmapFromFile;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipCreateBitmapFromFile: GdipCreateBitmapFromFile_Proc = None;
/// C 54: `GdipCreateBitmapFromStream_Proc fn_GdipCreateBitmapFromStream;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipCreateBitmapFromStream: GdipCreateBitmapFromStream_Proc = None;
/// C 55: `GdipCreateBitmapFromScan0_Proc fn_GdipCreateBitmapFromScan0;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipCreateBitmapFromScan0: GdipCreateBitmapFromScan0_Proc = None;
/// C 56: `SHCreateMemStream_Proc fn_SHCreateMemStream;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_SHCreateMemStream: SHCreateMemStream_Proc = None;
/// C 57: `GdipCreateHBITMAPFromBitmap_Proc fn_GdipCreateHBITMAPFromBitmap;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipCreateHBITMAPFromBitmap: GdipCreateHBITMAPFromBitmap_Proc = None;
/// C 58: `GdipCreateBitmapFromHBITMAP_Proc fn_GdipCreateBitmapFromHBITMAP;` (C 70 repeats it)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipCreateBitmapFromHBITMAP: GdipCreateBitmapFromHBITMAP_Proc = None;
/// C 59: `GdipDrawImageRectRectI_Proc fn_GdipDrawImageRectRectI;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipDrawImageRectRectI: GdipDrawImageRectRectI_Proc = None;
/// C 60: `GdipSetInterpolationMode_Proc fn_GdipSetInterpolationMode;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipSetInterpolationMode: GdipSetInterpolationMode_Proc = None;
/// C 61: `GdipDisposeImage_Proc fn_GdipDisposeImage;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipDisposeImage: GdipDisposeImage_Proc = None;
/// C 62: `GdipGetImageHeight_Proc fn_GdipGetImageHeight;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipGetImageHeight: GdipGetImageHeight_Proc = None;
/// C 63: `GdipGetImageWidth_Proc fn_GdipGetImageWidth;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipGetImageWidth: GdipGetImageWidth_Proc = None;
/// C 64: `GdipGetImageEncodersSize_Proc fn_GdipGetImageEncodersSize;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipGetImageEncodersSize: GdipGetImageEncodersSize_Proc = None;
/// C 65: `GdipGetImageEncoders_Proc fn_GdipGetImageEncoders;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipGetImageEncoders: GdipGetImageEncoders_Proc = None;
/// C 66: `GdipLoadImageFromFile_Proc fn_GdipLoadImageFromFile;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipLoadImageFromFile: GdipLoadImageFromFile_Proc = None;
/// C 67: `GdipGetImageThumbnail_Proc fn_GdipGetImageThumbnail;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipGetImageThumbnail: GdipGetImageThumbnail_Proc = None;
/// C 68: `GdipSaveImageToFile_Proc fn_GdipSaveImageToFile;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipSaveImageToFile: GdipSaveImageToFile_Proc = None;
/// C 69: `GdipImageRotateFlip_Proc fn_GdipImageRotateFlip;`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fn_GdipImageRotateFlip: GdipImageRotateFlip_Proc = None;

/* C 72-205: `gdiplus_init`.  C's loader loads each entry point by hand - it reads from two
   libraries (`gdiplus.dll` and `shlwapi.dll`) and `SHCreateMemStream` needs the ordinal fallback
   below, so the `LOAD_DLL_FN` macro `include/w32common.rs` 72-97 defers is not what this body
   uses.  C's cast `(X_Proc) get_proc_addr (…)` is the port's `transmute` between two
   `Option<unsafe extern "system" fn …>` types of the same width (the `src/comp.rs` 137 reading),
   and C's `if (!fn_X)` is `fn_X.is_none ()`.  */

/// C 72-205: `static bool gdiplus_init (void)`.
#[cfg (feature = "windowsnt")]
unsafe fn gdiplus_init () -> bool
{
  let mut gdiplus_lib: HANDLE = core::ptr::null_mut ();                         /* C 75 */
  let mut shlwapi_lib: HANDLE = core::ptr::null_mut ();

  gdiplus_lib = w32_delayed_load (Qgdiplus ());                                 /* C 77 */
  shlwapi_lib = w32_delayed_load (Qshlwapi ());                                 /* C 78 */
  if !(!gdiplus_lib.is_null () && !shlwapi_lib.is_null ())                      /* C 77-78 */
    { return false; }                                                           /* C 79 */

  fn_GdiplusStartup                                                             /* C 81-82 */
    = std::mem::transmute::<VOIDFNPTR, GdiplusStartup_Proc>
        (get_proc_addr (gdiplus_lib, c"GdiplusStartup".as_ptr ()));
  if fn_GdiplusStartup.is_none ()                                               /* C 83 */
    { return false; }                                                           /* C 84 */
  fn_GdiplusShutdown                                                            /* C 85-86 */
    = std::mem::transmute::<VOIDFNPTR, GdiplusShutdown_Proc>
        (get_proc_addr (gdiplus_lib, c"GdiplusShutdown".as_ptr ()));
  if fn_GdiplusShutdown.is_none ()                                              /* C 87 */
    { return false; }                                                           /* C 88 */
  fn_GdipCreateFromHDC                                                          /* C 89-90 */
    = std::mem::transmute::<VOIDFNPTR, GdipCreateFromHDC_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipCreateFromHDC".as_ptr ()));
  if fn_GdipCreateFromHDC.is_none ()                                            /* C 91 */
    { return false; }                                                           /* C 92 */
  fn_GdipDeleteGraphics                                                         /* C 93-94 */
    = std::mem::transmute::<VOIDFNPTR, GdipDeleteGraphics_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipDeleteGraphics".as_ptr ()));
  if fn_GdipDeleteGraphics.is_none ()                                           /* C 95 */
    { return false; }                                                           /* C 96 */
  fn_GdipGetPropertyItemSize                                                    /* C 97-98 */
    = std::mem::transmute::<VOIDFNPTR, GdipGetPropertyItemSize_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipGetPropertyItemSize".as_ptr ()));
  if fn_GdipGetPropertyItemSize.is_none ()                                      /* C 99 */
    { return false; }                                                           /* C 100 */
  fn_GdipGetPropertyItem                                                        /* C 101-102 */
    = std::mem::transmute::<VOIDFNPTR, GdipGetPropertyItem_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipGetPropertyItem".as_ptr ()));
  if fn_GdipGetPropertyItem.is_none ()                                          /* C 103 */
    { return false; }                                                           /* C 104 */
  fn_GdipImageGetFrameDimensionsCount                                           /* C 105-106 */
    = std::mem::transmute::<VOIDFNPTR, GdipImageGetFrameDimensionsCount_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipImageGetFrameDimensionsCount".as_ptr ()));
  if fn_GdipImageGetFrameDimensionsCount.is_none ()                             /* C 107 */
    { return false; }                                                           /* C 108 */
  fn_GdipImageGetFrameDimensionsList                                            /* C 109-110 */
    = std::mem::transmute::<VOIDFNPTR, GdipImageGetFrameDimensionsList_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipImageGetFrameDimensionsList".as_ptr ()));
  if fn_GdipImageGetFrameDimensionsList.is_none ()                              /* C 111 */
    { return false; }                                                           /* C 112 */
  fn_GdipImageGetFrameCount                                                     /* C 113-114 */
    = std::mem::transmute::<VOIDFNPTR, GdipImageGetFrameCount_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipImageGetFrameCount".as_ptr ()));
  if fn_GdipImageGetFrameCount.is_none ()                                       /* C 115 */
    { return false; }                                                           /* C 116 */
  fn_GdipImageSelectActiveFrame                                                 /* C 117-118 */
    = std::mem::transmute::<VOIDFNPTR, GdipImageSelectActiveFrame_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipImageSelectActiveFrame".as_ptr ()));
  if fn_GdipImageSelectActiveFrame.is_none ()                                   /* C 119 */
    { return false; }                                                           /* C 120 */
  fn_GdipCreateBitmapFromFile                                                   /* C 121-122 */
    = std::mem::transmute::<VOIDFNPTR, GdipCreateBitmapFromFile_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipCreateBitmapFromFile".as_ptr ()));
  if fn_GdipCreateBitmapFromFile.is_none ()                                     /* C 123 */
    { return false; }                                                           /* C 124 */
  fn_GdipCreateBitmapFromStream                                                 /* C 125-126 */
    = std::mem::transmute::<VOIDFNPTR, GdipCreateBitmapFromStream_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipCreateBitmapFromStream".as_ptr ()));
  if fn_GdipCreateBitmapFromStream.is_none ()                                   /* C 127 */
    { return false; }                                                           /* C 128 */
  fn_GdipCreateBitmapFromScan0                                                  /* C 129-130 */
    = std::mem::transmute::<VOIDFNPTR, GdipCreateBitmapFromScan0_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipCreateBitmapFromScan0".as_ptr ()));
  if fn_GdipCreateBitmapFromScan0.is_none ()                                    /* C 131 */
    { return false; }                                                           /* C 132 */
  fn_GdipCreateHBITMAPFromBitmap                                                /* C 133-134 */
    = std::mem::transmute::<VOIDFNPTR, GdipCreateHBITMAPFromBitmap_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipCreateHBITMAPFromBitmap".as_ptr ()));
  if fn_GdipCreateHBITMAPFromBitmap.is_none ()                                  /* C 135 */
    { return false; }                                                           /* C 136 */
  fn_GdipCreateBitmapFromHBITMAP                                                /* C 137-138 */
    = std::mem::transmute::<VOIDFNPTR, GdipCreateBitmapFromHBITMAP_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipCreateBitmapFromHBITMAP".as_ptr ()));
  if fn_GdipCreateBitmapFromHBITMAP.is_none ()                                  /* C 139 */
    { return false; }                                                           /* C 140 */
  fn_GdipDrawImageRectRectI                                                     /* C 141-142 */
    = std::mem::transmute::<VOIDFNPTR, GdipDrawImageRectRectI_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipDrawImageRectRectI".as_ptr ()));
  if fn_GdipDrawImageRectRectI.is_none ()                                       /* C 143 */
    { return false; }                                                           /* C 144 */
  fn_GdipSetInterpolationMode                                                   /* C 145-146 */
    = std::mem::transmute::<VOIDFNPTR, GdipSetInterpolationMode_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipSetInterpolationMode".as_ptr ()));
  if fn_GdipSetInterpolationMode.is_none ()                                     /* C 147 */
    { return false; }                                                           /* C 148 */
  fn_GdipDisposeImage                                                           /* C 149-150 */
    = std::mem::transmute::<VOIDFNPTR, GdipDisposeImage_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipDisposeImage".as_ptr ()));
  if fn_GdipDisposeImage.is_none ()                                             /* C 151 */
    { return false; }                                                           /* C 152 */
  fn_GdipGetImageHeight                                                         /* C 153-154 */
    = std::mem::transmute::<VOIDFNPTR, GdipGetImageHeight_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipGetImageHeight".as_ptr ()));
  if fn_GdipGetImageHeight.is_none ()                                           /* C 155 */
    { return false; }                                                           /* C 156 */
  fn_GdipGetImageWidth                                                          /* C 157-158 */
    = std::mem::transmute::<VOIDFNPTR, GdipGetImageWidth_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipGetImageWidth".as_ptr ()));
  if fn_GdipGetImageWidth.is_none ()                                            /* C 159 */
    { return false; }                                                           /* C 160 */
  /* LOAD_DLL_FN (shlwapi_lib, SHCreateMemStream); */                           /* C 161 */

  /* The following terrible kludge is required to use native image API
     on Windows before Vista, because SHCreateMemStream was not
     exported by name in those versions, only by ordinal number.  */
  fn_SHCreateMemStream                                                          /* C 166-167 */
    = std::mem::transmute::<VOIDFNPTR, SHCreateMemStream_Proc>
        (get_proc_addr (shlwapi_lib, c"SHCreateMemStream".as_ptr ()));
  if fn_SHCreateMemStream.is_none ()                                            /* C 168 */
    {
      fn_SHCreateMemStream                                                      /* C 170-171 */
	= std::mem::transmute::<VOIDFNPTR, SHCreateMemStream_Proc>
	    (get_proc_addr (shlwapi_lib, MAKEINTRESOURCEA! (12)));
      if fn_SHCreateMemStream.is_none ()                                        /* C 172 */
	{ return false; }                                                       /* C 173 */
    }
  fn_GdipGetImageEncodersSize                                                   /* C 175-176 */
    = std::mem::transmute::<VOIDFNPTR, GdipGetImageEncodersSize_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipGetImageEncodersSize".as_ptr ()));
  if fn_GdipGetImageEncodersSize.is_none ()                                     /* C 177 */
    { return false; }                                                           /* C 178 */
  fn_GdipGetImageEncoders                                                        /* C 179-180 */
    = std::mem::transmute::<VOIDFNPTR, GdipGetImageEncoders_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipGetImageEncoders".as_ptr ()));
  if fn_GdipGetImageEncoders.is_none ()                                         /* C 181 */
    { return false; }                                                           /* C 182 */
  fn_GdipLoadImageFromFile                                                      /* C 183-184 */
    = std::mem::transmute::<VOIDFNPTR, GdipLoadImageFromFile_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipLoadImageFromFile".as_ptr ()));
  if fn_GdipLoadImageFromFile.is_none ()                                        /* C 185 */
    { return false; }                                                           /* C 186 */
  fn_GdipGetImageThumbnail                                                      /* C 187-188 */
    = std::mem::transmute::<VOIDFNPTR, GdipGetImageThumbnail_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipGetImageThumbnail".as_ptr ()));
  if fn_GdipGetImageThumbnail.is_none ()                                        /* C 189 */
    { return false; }                                                           /* C 190 */
  fn_GdipSaveImageToFile                                                        /* C 191-192 */
    = std::mem::transmute::<VOIDFNPTR, GdipSaveImageToFile_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipSaveImageToFile".as_ptr ()));
  if fn_GdipSaveImageToFile.is_none ()                                          /* C 193 */
    { return false; }                                                           /* C 194 */
  fn_GdipImageRotateFlip                                                        /* C 195-196 */
    = std::mem::transmute::<VOIDFNPTR, GdipImageRotateFlip_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipImageRotateFlip".as_ptr ()));
  if fn_GdipImageRotateFlip.is_none ()                                          /* C 197 */
    { return false; }                                                           /* C 198 */
  fn_GdipCreateBitmapFromHBITMAP                                                /* C 199-200 */
    = std::mem::transmute::<VOIDFNPTR, GdipCreateBitmapFromHBITMAP_Proc>
        (get_proc_addr (gdiplus_lib, c"GdipCreateBitmapFromHBITMAP".as_ptr ()));
  if fn_GdipCreateBitmapFromHBITMAP.is_none ()                                  /* C 201 */
    { return false; }                                                           /* C 202 */

  return true;                                                                  /* C 204 */
}

/* C 209-213: the five file-scope objects.  C's are tentative definitions, i.e. zeroed
   (PORTING.md 0.1-13); the two record-valued ones spell their zero field by field, because a
   Rust `static` initialiser is a constant expression.  */
/// C 209: `static int gdip_initialized;`
#[cfg (feature = "windowsnt")]
static mut gdip_initialized: c_int = 0;
/// C 210: `static bool gdiplus_started;`
#[cfg (feature = "windowsnt")]
static mut gdiplus_started: bool = false;
/// C 211: `static ULONG_PTR token;`
#[cfg (feature = "windowsnt")]
static mut token: ULONG_PTR = 0;
/* C 212-213's two records live in the module below rather than at this crate root: a `static`
   named `output` here makes every `let output` in the mirrors on this graph illegal, and Rust has
   no `#[allow]` for that (`E0530`, measured: include/character.rs 448/474/503 and
   include/buffer.rs 1919/1947).  C's statements keep C's names and read
   `gdiplus_state::input.GdiplusVersion = 1;` - the path is the port's, the name is C's.  */
#[cfg (feature = "windowsnt")]
mod gdiplus_state
{
  use super::*;

  /// C 212: `static GdiplusStartupInput input;`
  pub static mut input: GdiplusStartupInput =
    GdiplusStartupInput { GdiplusVersion: 0, DebugEventCallback: None,
			  SuppressBackgroundThread: 0, SuppressExternalCodecs: 0 };
  /// C 213: `static GdiplusStartupOutput output;`
  pub static mut output: GdiplusStartupOutput =
    GdiplusStartupOutput { NotificationHook: None, NotificationUnhook: None };
}

/* C 23-24's `lisp.h`/`dispextern.h` declarations the mirrors on this graph do not carry.  C 310's
   `add_to_log`, C 392/445's `plist_get` (defined by `fns.c`) and C 419's `valid_image_p`
   (defined by `image.c`) are declared here instead - the
   P-88 (1) in-place shape the `src/image.rs` 3748/2078 and `src/charset.rs` 95 readings use, kept
   next to C's own include lines rather than at each call site so that one row covers them.  */
/// C 310/481's `add_to_log`: `lisp.h` 4378's `extern void add_to_log (char const *, ...);`.
/// C: lisp.h 4378: extern void add_to_log (char const *, ...);
#[cfg (feature = "windowsnt")]
unsafe extern "C"
{
  pub fn add_to_log (format: *const c_char, ...);
}
/// C 392/445's `plist_get`.
/// C: lisp.h 4280: extern Lisp_Object plist_get (Lisp_Object, Lisp_Object);
#[cfg (feature = "windowsnt")]
unsafe extern "C"
{
  pub fn plist_get (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object;
}
/// C 419's `valid_image_p`.
/// C: dispextern.h: bool valid_image_p (Lisp_Object);
#[cfg (feature = "windowsnt")]
unsafe extern "C"
{
  pub fn valid_image_p (object: Lisp_Object) -> bool;
}

/* Initialize GDI+, return true if successful.  */
/// C 216-243: `bool w32_gdiplus_startup (void)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_gdiplus_startup () -> bool
{
  let mut status: GpStatus = 0;                                    /* C 220 (zero-init, PORTING.md 1.5) */

  if gdiplus_started                                               /* C 222 */
    { return true; }                                               /* C 223 */
  /* C 224-229: `#ifdef WINDOWSNT` / `#else` / `#endif`.  The file gate *is* that macro, so
     C 225-226 are this arm's code and C 227-229 is the arm C's own `#else` spells.  */
  if gdip_initialized == 0                                         /* C 225 */
    { gdip_initialized = if gdiplus_init () { 1 } else { -1 }; }    /* C 226 */
  // C 227: `#else`
  // C 228: `  gdip_initialized = 1;`
  // C 229: `#endif`
  if gdip_initialized > 0                                          /* C 230 */
    {
      gdiplus_state::input.GdiplusVersion = 1;                     /* C 232 */
      gdiplus_state::input.DebugEventCallback = None;              /* C 233 `NULL` */
      gdiplus_state::input.SuppressBackgroundThread = FALSE;       /* C 234 */
      gdiplus_state::input.SuppressExternalCodecs = FALSE;         /* C 235 */

      status = (GdiplusStartup! ().unwrap ()) (&raw mut token,
					      &raw mut gdiplus_state::input,
					      &raw mut gdiplus_state::output);  /* C 237 */
      if status == Ok                                              /* C 238 */
	{ gdiplus_started = true; }                                /* C 239 */
      return status == Ok;                                         /* C 240 */
    }
  return false;                                                    /* C 242 */
}

/* This is called from term_ntproc.  */
/// C 245-252: `void w32_gdiplus_shutdown (void)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_gdiplus_shutdown ()
{
  if gdiplus_started                                               /* C 249 */
    { (GdiplusShutdown! ().unwrap ()) (token); }                   /* C 250 */
  gdiplus_started = false;                                         /* C 251 */
}

/// C 254-271: `bool w32_can_use_native_image_api (Lisp_Object type)`.  C's parameter is named
/// `type`; Rust spells that `r#type` (`src/alloc.rs` 1087's reading of the same C name).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_can_use_native_image_api (r#type: Lisp_Object) -> bool
{
  if !w32_use_native_image_api                                    /* C 257 */
    { return false; }                                             /* C 258 */
  if !(EQ (r#type, Qjpeg ())                                      /* C 259-264 */
	|| EQ (r#type, Qpng ())
	|| EQ (r#type, Qgif ())
	|| EQ (r#type, Qtiff ())
	|| EQ (r#type, Qbmp ())
	|| EQ (r#type, Qnative_image ()))
    {
      /* GDI+ can also display Exif, ICON, WMF, and EMF images.
	 But we don't yet support these in image.c.  */
      return false;                                               /* C 268 */
    }
  return w32_gdiplus_startup ();                                  /* C 270 */
}

/* C 273-282: `enum PropertyItem_type`.  The values C 287 reads are a GDI+ `WORD` and C's own
   `switch` has a `default` arm for the values the enum does not name, so the port spells the
   members the way this tree spells an SDK enum whose values are data - a `type` plus constants
   (`src/w32select.rs` 412/483's `GpStatus`/`Status`) - rather than a `#[repr(C)] enum`, which
   could not hold an out-of-range value without `transmute`.  */
/// C: enum PropertyItem_type { PI_BYTE = 1, PI_ASCIIZ = 2, PI_USHORT = 3, PI_ULONG = 4,
/// C:   PI_ULONG_PAIR = 5, PI_BYTE_ANY = 6, PI_LONG = 7, PI_LONG_PAIR = 10 };
#[cfg (feature = "windowsnt")]
pub type PropertyItem_type = c_int;
#[cfg (feature = "windowsnt")]
pub const PI_BYTE: PropertyItem_type = 1;
#[cfg (feature = "windowsnt")]
pub const PI_ASCIIZ: PropertyItem_type = 2;
#[cfg (feature = "windowsnt")]
pub const PI_USHORT: PropertyItem_type = 3;
#[cfg (feature = "windowsnt")]
pub const PI_ULONG: PropertyItem_type = 4;
#[cfg (feature = "windowsnt")]
pub const PI_ULONG_PAIR: PropertyItem_type = 5;
#[cfg (feature = "windowsnt")]
pub const PI_BYTE_ANY: PropertyItem_type = 6;
#[cfg (feature = "windowsnt")]
pub const PI_LONG: PropertyItem_type = 7;
#[cfg (feature = "windowsnt")]
pub const PI_LONG_PAIR: PropertyItem_type = 10;

/// C 284-315: `static double decode_delay (PropertyItem *propertyItem, int frame)`.  C 291-312's
/// `switch` is the `match` below (PORTING.md 2.2), so its `case`/`break`/`default` tokens have no
/// Rust counterpart (the `docs/ports/macro-body-notes.tsv` row).
#[cfg (feature = "windowsnt")]
unsafe fn decode_delay (propertyItem: *mut PropertyItem, frame: c_int) -> c_double
{
  let mut r#type: PropertyItem_type
    = (*propertyItem.add (0)).r#type as PropertyItem_type;         /* C 287 */
  let mut udelay: c_ulong = 0;                                     /* C 288 */
  let mut retval: c_double = 0.0;                                  /* C 289 */

  match r#type                                                      /* C 291 */
    {
      PI_BYTE                                                      /* C 293 */
      | PI_BYTE_ANY =>                                             /* C 294 */
	{
	  udelay = (*((*propertyItem.add (0)).value as *mut u8)
		    .offset (frame as isize)) as c_ulong;           /* C 295 */
	  retval = udelay as c_double;                             /* C 296 */
	}
      PI_USHORT =>                                                 /* C 298 */
	{
	  udelay = (*((*propertyItem.add (0)).value as *mut c_ushort)
		    .offset (frame as isize)) as c_ulong;           /* C 299 */
	  retval = udelay as c_double;                             /* C 300 */
	}
      PI_ULONG                                                     /* C 302 */
      | PI_LONG =>                                                 /* C 303 delay should always be positive */
	{
	  udelay = (*((*propertyItem.add (0)).value as *mut c_ulong)
		    .offset (frame as isize)) as c_ulong;           /* C 304 */
	  retval = udelay as c_double;                             /* C 305 */
	}
      _ =>                                                         /* C 307 */
	{
	  /* This negative value will cause the caller to disregard the
	     delay if we cannot determine it reliably.  */
	  add_to_log (c"Invalid or unknown propertyItem type in w32image.c".as_ptr ());  /* C 310 */
	  retval = -1.0;                                           /* C 311 */
	}
    }

  return retval;                                                   /* C 314 */
}

/// C 317-351: `static double w32_frame_delay (GpBitmap *pBitmap, int frame)`.
#[cfg (feature = "windowsnt")]
unsafe fn w32_frame_delay (pBitmap: *mut GpBitmap, frame: c_int) -> c_double
{
  let mut size: UINT = 0;                                          /* C 320 */
  let mut propertyItem: *mut PropertyItem = core::ptr::null_mut (); /* C 321 */
  let mut delay: c_double = -1.0;                                  /* C 322 */

  /* Assume that the image has a property item of type PropertyItemEquipMake.
     Get the size of that property item.  This can fail for multi-frame TIFF
     images.  */
  let mut status: GpStatus = (GdipGetPropertyItemSize! ().unwrap ())  /* C 327-328 */
    (pBitmap, PropertyTagFrameDelay, &raw mut size);

  if status == Ok                                                  /* C 330 */
    {
      /* Allocate a buffer to receive the property item.  */
      propertyItem = malloc (size as usize) as *mut PropertyItem;  /* C 333 */
      if !propertyItem.is_null ()                                  /* C 334 */
	{
	  /* Get the property item.  */
	  (GdipGetPropertyItem! ().unwrap ())                          /* C 337-338 */
	    (pBitmap, PropertyTagFrameDelay, size, propertyItem);
	  delay = decode_delay (propertyItem, frame);                  /* C 339 */
	  if delay <= 0.0                                              /* C 340 */
	    {
	      /* In GIF files, unfortunately, delay is only specified
		 for the first frame.  */
	      delay = decode_delay (propertyItem, 0);                  /* C 344 */
	    }
	  delay /= 100.0;                                              /* C 346 */
	  free (propertyItem as *mut c_void);                          /* C 347 */
	}
    }
  return delay;                                                    /* C 350 */
}

/// C 353-387: `static GpStatus w32_select_active_frame (GpBitmap *pBitmap, int frame,
/// int *nframes, double *delay)`.
#[cfg (feature = "windowsnt")]
unsafe fn w32_select_active_frame (pBitmap: *mut GpBitmap, frame: c_int,
				   nframes: *mut c_int, delay: *mut c_double) -> GpStatus
{
  let mut count: UINT = 0;                                         /* C 357 */
  let mut frameCount: UINT = 0;
  let mut pDimensionIDs: [GUID; 1] = core::mem::zeroed ();         /* C 358 */
  let mut status: GpStatus = Ok;                                   /* C 359 */

  status = (GdipImageGetFrameDimensionsCount! ().unwrap ()) (pBitmap, &raw mut count);  /* C 361 */
  frameCount = 0;                                                  /* C 362 */
  *nframes = 0;
  *delay = -1.0;                                                   /* C 363 */
  if count != 0                                                    /* C 364 */
    {
      /* The following call will fill pDimensionIDs[0] with the
	 FrameDimensionTime GUID for GIF images, and
	 FrameDimensionPage GUID for other image types.  Multi-page
	 GIF and TIFF images expect these values in the
	 GdipImageSelectActiveFrame call below.  */
      status = (GdipImageGetFrameDimensionsList! ().unwrap ())       /* C 371 */
	(pBitmap, pDimensionIDs.as_mut_ptr (), 1);
      status = (GdipImageGetFrameCount! ().unwrap ())                /* C 372 */
	(pBitmap, &raw mut pDimensionIDs[0], &raw mut frameCount);
      if status == Ok && frameCount > 1                              /* C 373 */
	{
	  if frame < 0 || frame >= frameCount as c_int               /* C 375 */
	    { status = GenericError; }                               /* C 376 */
	  else
	    {
	      status = (GdipImageSelectActiveFrame! ().unwrap ())     /* C 379-380 */
		(pBitmap, &raw mut pDimensionIDs[0], frame as UINT);
	      *delay = w32_frame_delay (pBitmap, frame);              /* C 381 */
	      *nframes = frameCount as c_int;                         /* C 382 */
	    }
	}
    }
  return status;                                                   /* C 386 */
}

/// C 389-409: `static ARGB w32_image_bg_color (struct frame *f, struct image *img)`.
#[cfg (feature = "windowsnt")]
unsafe fn w32_image_bg_color (f: *mut frame, img: *mut image) -> ARGB
{
  let specified_bg: Lisp_Object = plist_get (XCDR ((*img).spec), QCbackground ());  /* C 392 */
  let mut color: Emacs_Color = core::mem::zeroed ();               /* C 393 */

  /* If the user specified a color, try to use it; if not, use the
     current frame background, ignoring any default background
     color set by the image.  */
  if (if STRINGP (specified_bg)                                    /* C 398-400 */
	{ w32_defined_color (f, SSDATA (specified_bg), &raw mut color, false, false) }
      else
	{ w32_query_frame_background_color (f, &raw mut color); true })
    /* The user specified ':background', use that.  */
    {
      let red: DWORD = ((color.red as DWORD) & 0xff00) << 8;     /* C 403 */
      let green: DWORD = (color.green as DWORD) & 0xff00;        /* C 404 */
      let blue: DWORD = (color.blue as DWORD) >> 8;              /* C 405 */
      return (red | green | blue) as ARGB;                         /* C 406 */
    }
  return 0xff000000;                                               /* C 408 */
}

/// C 411-485: `int w32_load_image (struct frame *f, struct image *img, Lisp_Object spec_file,
/// Lisp_Object spec_data)`.  C 425's local is named `fn`, which Rust cannot spell, so the port
/// names it `filename` (the token check's two-character rule skips `fn` anyway).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_load_image (f: *mut frame, img: *mut image,
					spec_file: Lisp_Object, spec_data: Lisp_Object) -> c_int
{
  let mut status: GpStatus = GenericError;                         /* C 415 */
  let mut pBitmap: *mut GpBitmap = core::ptr::null_mut ();         /* C 416 */
  let mut metadata: Lisp_Object = Qnil ();                         /* C 417 */

  eassert! (valid_image_p ((*img).spec));                          /* C 419 */

  /* This function only gets called if w32_gdiplus_startup was invoked
     and succeeded.  We have a valid token and GDI+ is active.  */
  if STRINGP (spec_file)                                           /* C 423 */
    {
      let filename: *const c_char
	= map_w32_filename (SSDATA (spec_file), core::ptr::null_mut ());  /* C 425 */
      let mut filename_w: [wchar_t; MAX_PATH] = [0; MAX_PATH];      /* C 426 */
      filename_to_utf16 (filename, filename_w.as_mut_ptr ());       /* C 427 */
      status = (GdipCreateBitmapFromFile! ().unwrap ())             /* C 428 */
	(filename_w.as_mut_ptr (), &raw mut pBitmap);
    }
  else if STRINGP (spec_data)                                      /* C 430 */
    {
      let pStream: *mut IStream = (SHCreateMemStream! ().unwrap ())  /* C 432-433 */
	(SDATA (spec_data) as *mut BYTE, SBYTES (spec_data) as UINT);
      if !pStream.is_null ()                                        /* C 434 */
	{
	  status = (GdipCreateBitmapFromStream! ().unwrap ())          /* C 436 */
	    (pStream, &raw mut pBitmap);
	  IStream_Release! (pStream);                                  /* C 437 */
	}
    }

  metadata = Qnil ();                                              /* C 441 */
  if status == Ok                                                  /* C 442 */
    {
      /* In multiframe pictures, select the first frame.  */
      let lisp_index: Lisp_Object = plist_get (XCDR ((*img).spec), QCindex ());  /* C 445 */
      let index: c_int = if FIXNATP (lisp_index)                    /* C 446 */
	{ XFIXNAT (lisp_index) as c_int } else { 0 };
      let mut nframes: c_int = 0;                                  /* C 447 */
      let mut delay: c_double = 0.0;                               /* C 448 */
      status = w32_select_active_frame (pBitmap, index, &raw mut nframes,
					&raw mut delay);                /* C 449 */
      if status == Ok                                              /* C 450 */
	{
	  if nframes > 1                                           /* C 452 */
	    { metadata = Fcons (Qcount (),                            /* C 453 */
				Fcons (make_fixnum (nframes as EMACS_INT), metadata)); }
	  if delay >= 0.0                                          /* C 454 */
	    { metadata = Fcons (Qdelay (), Fcons (make_float (delay), metadata)); }  /* C 455 */
	}
    }

  if status == Ok                                                  /* C 459 */
    {
      let bg_color: ARGB = w32_image_bg_color (f, img);            /* C 461 */
      let mut pixmap: Emacs_Pixmap = core::ptr::null_mut ();       /* C 462 */

      status = (GdipCreateHBITMAPFromBitmap! ().unwrap ())          /* C 464 */
	(pBitmap, &raw mut pixmap, bg_color);
      if status == Ok                                              /* C 465 */
	{
	  let mut width: UINT = 0;                                 /* C 467 */
	  let mut height: UINT = 0;
	  (GdipGetImageWidth! ().unwrap ()) (pBitmap, &raw mut width);   /* C 468 */
	  (GdipGetImageHeight! ().unwrap ()) (pBitmap, &raw mut height); /* C 469 */
	  (*img).width = width as c_int;                            /* C 470 */
	  (*img).height = height as c_int;                          /* C 471 */
	  (*img).pixmap = pixmap;                                   /* C 472 */
	  (*img).lisp_data = metadata;                              /* C 473 */
	}

      (GdipDisposeImage! ().unwrap ()) (pBitmap);                   /* C 476 */
    }

  if status != Ok                                                   /* C 479 */
    {
      add_to_log (c"Unable to load image %s".as_ptr (), (*img).spec);  /* C 481 */
      return 0;                                                     /* C 482 */
    }
  return 1;                                                         /* C 484 */
}

/* C 487-491: `struct cached_encoder`.  */
/// C: struct cached_encoder { int num; char *type; CLSID clsid; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct cached_encoder
{
  pub num: c_int,
  pub r#type: *mut c_char,
  pub clsid: CLSID,
}
/// C 493: `static struct cached_encoder last_encoder;`
#[cfg (feature = "windowsnt")]
static mut last_encoder: cached_encoder =
  cached_encoder { num: 0, r#type: core::ptr::null_mut (), clsid: GUID {
    Data1: 0, Data2: 0, Data3: 0, Data4: [0; 8] } };

/* C 495-498: `struct thumb_type_data`.  */
/// C: struct thumb_type_data { const char *ext; const wchar_t *mime; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct thumb_type_data
{
  pub ext: *const c_char,
  pub mime: *const wchar_t,
}

/* C 500-511: `static struct thumb_type_data thumb_types []`.  C's six `L"…"` literals are wide
   strings, which Rust has no syntax for, so each is spelled as the same UTF-16 code units plus
   C's terminator (the `src/w32fns.rs` 385-393 reading of `L"DarkMode_Explorer"`).  */
/// C 504: `{"jpg", L"image/jpeg"}`
#[cfg (feature = "windowsnt")]
static THUMB_MIME_JPEG: [u16; 11] =
  [0x0069, 0x006D, 0x0061, 0x0067, 0x0065, 0x002F, 0x006A, 0x0070, 0x0065, 0x0067, 0x0000];
/// C 505: `{"png", L"image/png"}`
#[cfg (feature = "windowsnt")]
static THUMB_MIME_PNG: [u16; 10] =
  [0x0069, 0x006D, 0x0061, 0x0067, 0x0065, 0x002F, 0x0070, 0x006E, 0x0067, 0x0000];
/// C 506: `{"bmp", L"image/bmp"}`
#[cfg (feature = "windowsnt")]
static THUMB_MIME_BMP: [u16; 10] =
  [0x0069, 0x006D, 0x0061, 0x0067, 0x0065, 0x002F, 0x0062, 0x006D, 0x0070, 0x0000];
/// C 508: `{"gif", L"image/gif"}`
#[cfg (feature = "windowsnt")]
static THUMB_MIME_GIF: [u16; 10] =
  [0x0069, 0x006D, 0x0061, 0x0067, 0x0065, 0x002F, 0x0067, 0x0069, 0x0066, 0x0000];
/// C 509: `{"tiff", L"image/tiff"}`
#[cfg (feature = "windowsnt")]
static THUMB_MIME_TIFF: [u16; 11] =
  [0x0069, 0x006D, 0x0061, 0x0067, 0x0065, 0x002F, 0x0074, 0x0069, 0x0066, 0x0066, 0x0000];
/// C 500-511: `static struct thumb_type_data thumb_types [] = { … {NULL, NULL} };`
/// C: static struct thumb_type_data thumb_types [] =
/// C:   { {"jpg", L"image/jpeg"}, {"png", L"image/png"}, {"bmp", L"image/bmp"},
/// C:     {"jpeg", L"image/jpeg"}, {"gif", L"image/gif"}, {"tiff", L"image/tiff"},
/// C:     {NULL, NULL} };
#[cfg (feature = "windowsnt")]
static mut thumb_types: [thumb_type_data; 7] =
  [ thumb_type_data { ext: c"jpg".as_ptr (),
		      mime: core::ptr::addr_of! (THUMB_MIME_JPEG) as *const wchar_t },   /* C 504 */
    thumb_type_data { ext: c"png".as_ptr (),
		      mime: core::ptr::addr_of! (THUMB_MIME_PNG) as *const wchar_t },    /* C 505 */
    thumb_type_data { ext: c"bmp".as_ptr (),
		      mime: core::ptr::addr_of! (THUMB_MIME_BMP) as *const wchar_t },    /* C 506 */
    thumb_type_data { ext: c"jpeg".as_ptr (),
		      mime: core::ptr::addr_of! (THUMB_MIME_JPEG) as *const wchar_t },   /* C 507 */
    thumb_type_data { ext: c"gif".as_ptr (),
		      mime: core::ptr::addr_of! (THUMB_MIME_GIF) as *const wchar_t },    /* C 508 */
    thumb_type_data { ext: c"tiff".as_ptr (),
		      mime: core::ptr::addr_of! (THUMB_MIME_TIFF) as *const wchar_t },   /* C 509 */
    thumb_type_data { ext: core::ptr::null (),
		      mime: core::ptr::null () } ];                                      /* C 510 */

/// C 514-566: `int w32_gdip_get_encoder_clsid (const char *type, CLSID *clsid)`.  C 549's
/// `for (int j = 0; j < num; ++j)` is the `while` below with C's increment at the end of the
/// body (the `src/w32select.rs` 1034 reading of the same C loop).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_gdip_get_encoder_clsid (r#type: *const c_char,
						    clsid: *mut CLSID) -> c_int
{
  /* A simple cache based on the assumptions that many thumbnails will
     be generated using the same TYPE.  */
  if !last_encoder.r#type.is_null ()                               /* C 519 */
    && stricmp (r#type, last_encoder.r#type) == 0
    {
      *clsid = last_encoder.clsid;                                 /* C 521 */
      return last_encoder.num;                                     /* C 522 */
    }

  let mut format: *const wchar_t = core::ptr::null ();             /* C 525 */
  let mut tp: *mut thumb_type_data
    = core::ptr::addr_of_mut! (thumb_types) as *mut thumb_type_data;  /* C 526 */
  while !(*tp).ext.is_null ()                                      /* C 527 */
    {
      if stricmp (r#type, (*tp).ext) == 0                          /* C 529 */
	{
	  format = (*tp).mime;                                     /* C 531 */
	  break;                                                   /* C 532 */
	}
      tp = tp.add (1);                                             /* C 527's increment */
    }
  if format.is_null ()                                             /* C 535 */
    { return -1; }                                                 /* C 536 */

  let mut num: c_uint = 0;                                         /* C 538 */
  let mut size: c_uint = 0;                                        /* C 539 */
  let mut image_codec_info: *mut ImageCodecInfo = core::ptr::null_mut ();  /* C 540 */

  (GdipGetImageEncodersSize! ().unwrap ()) (&raw mut num, &raw mut size);  /* C 542 */
  if size == 0                                                     /* C 543 */
    { return -1; }                                                 /* C 544 */

  image_codec_info = xmalloc (size as usize) as *mut ImageCodecInfo;  /* C 546 */
  (GdipGetImageEncoders! ().unwrap ()) (num, size, image_codec_info);  /* C 547 */

  let mut j: c_int = 0;                                            /* C 549 */
  while j < num as c_int                                           /* C 549 */
    {
      if wcscmp ((*image_codec_info.add (j as usize)).MimeType, format) == 0  /* C 551 */
	{
	  if !last_encoder.r#type.is_null ()                       /* C 553 */
	    { xfree (last_encoder.r#type as *mut c_void); }        /* C 554 */
	  last_encoder.r#type = xstrdup ((*tp).ext);               /* C 555 */
	  last_encoder.clsid = (*image_codec_info.add (j as usize)).Clsid;  /* C 556 */
	  last_encoder.num = j;                                    /* C 557 */
	  *clsid = (*image_codec_info.add (j as usize)).Clsid;      /* C 558 */
	  xfree (image_codec_info as *mut c_void);                  /* C 559 */
	  return j;                                                /* C 560 */
	}
      j += 1;                                                      /* C 549's increment */
    }

  xfree (image_codec_info as *mut c_void);                         /* C 564 */
  return -1;                                                       /* C 565 */
}

/// C 568-641: `int w32_gdip_export_frame (HWND hwnd, Lisp_Object file, CLSID *clsid)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_gdip_export_frame (hwnd: HWND, mut file: Lisp_Object,
					       clsid: *mut CLSID) -> c_int
{
  let mut frame_rect: RECT = core::mem::zeroed ();                 /* C 571 */
  let mut frame_width: c_int = 0;                                  /* C 572 */
  let mut frame_height: c_int = 0;
  let mut hdc: HDC = core::ptr::null_mut ();                       /* C 573 */
  let mut capture_hdc: HDC = core::ptr::null_mut ();
  let mut bits_per_pixel: c_int = 0;                               /* C 574 */

  /* FIXME (maybe): GetWindowRect returns the rectangle that includes
     the "resize borders", which are by default invisible on Windows 10
     and later.  It is unclear whether we should strive to exclude those
     resize borders: it is not trivial, and OTOH these borders can be
     made visible, in which case users might want them in the image.
     The downside is that the image has some of the desktop around the
     frame included in it.  So what?  */
  if GetWindowRect (hwnd, &raw mut frame_rect) == 0                /* C 584 */
    { return -1; }                                                 /* C 585 */
  frame_width = frame_rect.right - frame_rect.left;                /* C 586 */
  frame_height = frame_rect.bottom - frame_rect.top;               /* C 587 */
  if !(frame_width > 0 && frame_height > 0)                        /* C 587-588 */
    { return -1; }                                                 /* C 588 */

  hdc = GetWindowDC (hwnd);                                        /* C 590 */
  bits_per_pixel = GetDeviceCaps (hdc, BITSPIXEL);                 /* C 591 */
  capture_hdc = CreateCompatibleDC (hdc);                          /* C 592 */

  /* Capture the image.  */
  let mut capture_bmi: BITMAPINFO = core::mem::zeroed ();          /* C 595 */
  memset (&raw mut capture_bmi as *mut c_void, 0,
	  core::mem::size_of::<BITMAPINFO> ());                    /* C 596 */
  let header: *mut BITMAPINFOHEADER = &raw mut capture_bmi.bmiHeader;  /* C 597 */
  (*header).biSize = core::mem::size_of::<BITMAPINFOHEADER> () as DWORD;  /* C 598 */
  (*header).biWidth = frame_width;                                 /* C 599 */
  (*header).biHeight = -frame_height;                              /* C 600 */
  (*header).biPlanes = 1;                                          /* C 601 */
  (*header).biBitCount = bits_per_pixel as WORD;                   /* C 602 */
  (*header).biCompression = BI_RGB;                                /* C 603 */
  let mut dib_bits: *mut DWORD = core::ptr::null_mut ();           /* C 604 */
  let capture_bitmap: HBITMAP = CreateDIBSection (hdc, &raw const capture_bmi,
						  DIB_PAL_COLORS,
						  &raw mut dib_bits as *mut *mut c_void,
						  core::ptr::null_mut (), 0);  /* C 605-607 */
  if !capture_bitmap.is_null ()                                    /* C 608 */
    {
      SaveDC (capture_hdc);                                        /* C 610 */
      SelectObject (capture_hdc, capture_bitmap as HGDIOBJ);       /* C 611 */
      BitBlt (capture_hdc, 0, 0, frame_width, frame_height, hdc, 0, 0, SRCCOPY);  /* C 612 */
      RestoreDC (capture_hdc, -1);                                 /* C 613 */
    }
  DeleteDC (capture_hdc);                                          /* C 615 */
  DeleteDC (hdc);                                                  /* C 616 */

  /* Save the captured image to file in requested format.  */
  let mut bitmap: *mut GpBitmap = core::ptr::null_mut ();          /* C 619 */
  if !capture_bitmap.is_null ()                                    /* C 620 */
    {
      let mut status: GpStatus                                       /* C 622-623 */
	= (GdipCreateBitmapFromHBITMAP! ().unwrap ()) (capture_bitmap,
						      core::ptr::null_mut (),
						      &raw mut bitmap);
      if status == Ok                                              /* C 624 */
	{
	  let mut file_w: [wchar_t; MAX_PATH] = [0; MAX_PATH];       /* C 626 */
	  file = ENCODE_FILE (Fexpand_file_name (Fcopy_sequence (file), Qnil ()));  /* C 627-628 */
	  unixtodos_filename (SSDATA (file));                        /* C 629 */
	  filename_to_utf16 (SSDATA (file), file_w.as_mut_ptr ());   /* C 630 */
	  status = (GdipSaveImageToFile! ().unwrap ())               /* C 631 */
	    (bitmap, file_w.as_ptr (), clsid as *const CLSID, core::ptr::null ());
	}
      (GdipDisposeImage! ().unwrap ()) (bitmap);                   /* C 633 */
      DeleteObject (capture_bitmap as HGDIOBJ);                    /* C 634 */

      if status == Ok                                              /* C 636 */
	{ return 0; }                                              /* C 637 */
    }

  return -1;                                                       /* C 640 */
}

/* C 643-706: `DEFUN ("w32image-create-thumbnail", Fw32image_create_thumbnail,
   Sw32image_create_thumbnail, 5, 5, 0, …)`.  C reassigns two of its five parameters (C 671,
   C 693), which the macro's `$arg : $ty` pattern cannot mark `mut`, so the body shadows each
   with a local of C's own name (`let mut input_file = input_file;`).  */
#[cfg (feature = "windowsnt")]
DEFUN! ("w32image-create-thumbnail", Fw32image_create_thumbnail, Sw32image_create_thumbnail, 5, 5, 0,
       doc: "Create a HEIGHT by WIDTH thumbnail file THUMB-FILE for image INPUT-FILE.\nTYPE is the image type to use for the thumbnail file, a string.  It is\nusually identical to the file-name extension of THUMB-FILE, but without\nthe leading period, and both \"jpeg\" and \"jpg\" can be used for JPEG.\nTYPE is matched case-insensitively against supported types.  Currently,\nthe supported TYPEs are BMP, JPEG, GIF, TIFF, and PNG; any other type\nwill cause the function to fail.\nReturn non-nil if thumbnail creation succeeds, nil otherwise.  ",
       (input_file: Lisp_Object, thumb_file: Lisp_Object, r#type: Lisp_Object,
	height: Lisp_Object, width: Lisp_Object),
       {
	 /* Sanity checks.  */
	 CHECK_STRING (input_file);                                    /* C 657 */
	 CHECK_STRING (thumb_file);                                    /* C 658 */
	 CHECK_STRING (r#type);                                        /* C 659 */
	 CHECK_FIXNAT (height);                                        /* C 660 */
	 CHECK_FIXNAT (width);                                         /* C 661 */

	 if !gdiplus_started                                          /* C 663 */
	   {
	     if !w32_gdiplus_startup ()                               /* C 665 */
	       { return Qnil (); }                                    /* C 666 */
	   }

	 /* Create an image by reading from INPUT_FILE.  */
	 let mut input_file_w: [wchar_t; MAX_PATH] = [0; MAX_PATH];    /* C 670 */
	 let mut input_file = input_file;                              /* C 671 */
	 input_file
	   = ENCODE_FILE (Fexpand_file_name (Fcopy_sequence (input_file), Qnil ()));  /* C 671-672 */
	 unixtodos_filename (SSDATA (input_file));                     /* C 673 */
	 filename_to_utf16 (SSDATA (input_file), input_file_w.as_mut_ptr ());  /* C 674 */
	 let mut file_image: *mut GpImage = core::ptr::null_mut ();    /* C 675 */
	 let mut status: GpStatus                                       /* C 676 */
	   = (GdipLoadImageFromFile! ().unwrap ()) (input_file_w.as_mut_ptr (),
						   &raw mut file_image);

	 if status == Ok                                               /* C 678 */
	   {
	     /* Create a thumbnail for the image.  */
	     let mut thumb_image: *mut GpImage = core::ptr::null_mut ();  /* C 681 */
	     status = (GdipGetImageThumbnail! ().unwrap ())            /* C 682-684 */
	       (file_image, XFIXNAT (width) as UINT, XFIXNAT (height) as UINT,
		&raw mut thumb_image, None, core::ptr::null_mut ());
	     (GdipDisposeImage! ().unwrap ()) (file_image);            /* C 685 */
	     let mut thumb_clsid: CLSID = core::mem::zeroed ();        /* C 686 */
	     if status == Ok                                           /* C 687 */
	       /* Get the GUID of the TYPE's encoder. */
	       && w32_gdip_get_encoder_clsid (SSDATA (r#type), &raw mut thumb_clsid) >= 0  /* C 689 */
	       {
		 /* Save the thumbnail image to a file of specified TYPE.  */
		 let mut thumb_file_w: [wchar_t; MAX_PATH] = [0; MAX_PATH];  /* C 692 */
		 let mut thumb_file = thumb_file;                      /* C 693 */
		 thumb_file
		   = ENCODE_FILE (Fexpand_file_name (Fcopy_sequence (thumb_file),
						     Qnil ()));       /* C 693-695 */
		 unixtodos_filename (SSDATA (thumb_file));             /* C 696 */
		 filename_to_utf16 (SSDATA (thumb_file), thumb_file_w.as_mut_ptr ());  /* C 697 */
		 status = (GdipSaveImageToFile! ().unwrap ())          /* C 698-699 */
		   (thumb_image, thumb_file_w.as_ptr (), &raw const thumb_clsid, core::ptr::null ());
		 (GdipDisposeImage! ().unwrap ()) (thumb_image);       /* C 700 */
	       }
	     else if status == Ok                                      /* C 702 */
	       { status = InvalidParameter; }                          /* C 703 */
	   }
	 return if status == Ok { Qt () } else { Qnil () };            /* C 705 */
       });

/// C 708-712: `void syms_of_w32image (void)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_w32image ()
{
  defsubr (&raw mut Sw32image_create_thumbnail);                   /* C 711 */
}

/// C 714-717: `void globals_of_w32image (void)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn globals_of_w32image ()
{
}
