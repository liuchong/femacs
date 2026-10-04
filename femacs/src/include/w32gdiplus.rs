// femacs: port of src/w32gdiplus.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): function-pointer typedefs, the loaded-entry-point objects and
// the `#undef`/`#define` name pairs, plus two entry points at the end.  The header has **no
// include guard of its own**: C 1 opens `#ifdef WINDOWSNT`, C 142 closes it, and the two
// declarations after it (C 144-145) are live text on every platform.  Measured: the first line is
// `#ifdef WINDOWSNT` and `rg -n 'ifndef .*_H' emacs/src/w32gdiplus.h` finds nothing.
//
// `WINDOWSNT` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so C 1-142 is dead
// text on this target (`tools/extract-abi.py` inventories the file as 0 macros, 0 types, 0
// objects and 0 functions - everything it declares is inside that guard), and its C readers are
// Windows units (`src/w32image.c` and the `w32fns.c` image paths), none of which is in
// `docs/baseline/frozen-build-sources.txt`.  The whole region therefore carries
// `#[cfg (feature = "windowsnt")]` per item - P-51's arm for the non-frozen side, the
// `include/w32font.rs` (3.176) and `include/macfont.rs` (3.170) shapes.
//
// C's own names are kept inside that arm (`GpStatus`, `GpImage`, `GpGraphics`, `GpBitmap`,
// `IStream`, `HDC`, `HBITMAP`, `HPALETTE`, `WCHAR`, `UINT`, `PROPID`, `GUID`, `CLSID`,
// `PropertyItem`, `ImageCodecInfo`, `EncoderParameters`, `PixelFormat`, `InterpolationMode`,
// `GpUnit`, `RotateFlipType`, `ARGB`, `ULONG_PTR`, `DrawImageAbort`, `GetThumbnailImageAbort`):
// they come from `<windows.h>` and GDI+, the tree has no mirror of either, and the Windows target
// supplies them - the `include/ftfont.rs` 9-18 reading of `Display`/`XftFont`/`FcPattern`.  C's
// `WINGDIPAPI`/`WINAPI` is the `__stdcall` calling convention, which the port spells
// `extern "system"` (the `include/w32.rs` 317-329 reading of `WINAPI`), and a C function pointer
// is nullable, so each typedef is an `Option<…>` (same reading, `include/w32.rs` 323-329).
// `GDIPCONST` is C's `const`, so those parameters are `*const T`.  **No layout assertion is
// written**: nothing here is a record the frozen clang could compile (the `include/macfont.rs`
// 26-27 rule), and the inventory above asks for nothing.
//
// C repeats one typedef, one object, one `#undef` and one `#define`; Rust has one name per scope,
// so each is written at its first C line and the repeat is recorded there (C 53-54, 83, 112, 141).

/// C 2-3: `typedef GpStatus (WINGDIPAPI *GdiplusStartup_Proc) (ULONG_PTR *,
/// GdiplusStartupInput *, GdiplusStartupOutput *);`
#[cfg (feature = "windowsnt")]
pub type GdiplusStartup_Proc
  = Option<unsafe extern "system" fn (*mut ULONG_PTR, *mut GdiplusStartupInput,
                                      *mut GdiplusStartupOutput) -> GpStatus>;

/// C 4: `typedef VOID (WINGDIPAPI *GdiplusShutdown_Proc) (ULONG_PTR);` - `VOID` is C's `void`,
/// which Rust spells `()`.
#[cfg (feature = "windowsnt")]
pub type GdiplusShutdown_Proc = Option<unsafe extern "system" fn (ULONG_PTR)>;

/// C 5-6: `typedef GpStatus (WINGDIPAPI *GdipCreateFromHDC_Proc) (HDC hdc, GpGraphics **graphics);`
#[cfg (feature = "windowsnt")]
pub type GdipCreateFromHDC_Proc
  = Option<unsafe extern "system" fn (HDC, *mut *mut GpGraphics) -> GpStatus>;

/// C 7: `typedef GpStatus (WINGDIPAPI *GdipDeleteGraphics_Proc) (GpGraphics *graphics);`
#[cfg (feature = "windowsnt")]
pub type GdipDeleteGraphics_Proc
  = Option<unsafe extern "system" fn (*mut GpGraphics) -> GpStatus>;

/// C 8-9: `typedef GpStatus (WINGDIPAPI *GdipGetPropertyItemSize_Proc) (GpImage *, PROPID,
/// UINT *);`
#[cfg (feature = "windowsnt")]
pub type GdipGetPropertyItemSize_Proc
  = Option<unsafe extern "system" fn (*mut GpImage, PROPID, *mut UINT) -> GpStatus>;

/// C 10-11: `typedef GpStatus (WINGDIPAPI *GdipGetPropertyItem_Proc) (GpImage *, PROPID, UINT,
/// PropertyItem *);`
#[cfg (feature = "windowsnt")]
pub type GdipGetPropertyItem_Proc
  = Option<unsafe extern "system" fn (*mut GpImage, PROPID, UINT,
                                      *mut PropertyItem) -> GpStatus>;

/// C 12-13: `typedef GpStatus (WINGDIPAPI *GdipImageGetFrameDimensionsCount_Proc) (GpImage *,
/// UINT *);`
#[cfg (feature = "windowsnt")]
pub type GdipImageGetFrameDimensionsCount_Proc
  = Option<unsafe extern "system" fn (*mut GpImage, *mut UINT) -> GpStatus>;

/// C 14-15: `typedef GpStatus (WINGDIPAPI *GdipImageGetFrameDimensionsList_Proc) (GpImage *,
/// GUID *, UINT);`
#[cfg (feature = "windowsnt")]
pub type GdipImageGetFrameDimensionsList_Proc
  = Option<unsafe extern "system" fn (*mut GpImage, *mut GUID, UINT) -> GpStatus>;

/// C 16-17: `typedef GpStatus (WINGDIPAPI *GdipImageGetFrameCount_Proc) (GpImage *,
/// GDIPCONST GUID *, UINT *);`
#[cfg (feature = "windowsnt")]
pub type GdipImageGetFrameCount_Proc
  = Option<unsafe extern "system" fn (*mut GpImage, *const GUID, *mut UINT) -> GpStatus>;

/// C 18-19: `typedef GpStatus (WINGDIPAPI *GdipImageSelectActiveFrame_Proc) (GpImage*,
/// GDIPCONST GUID *, UINT);`
#[cfg (feature = "windowsnt")]
pub type GdipImageSelectActiveFrame_Proc
  = Option<unsafe extern "system" fn (*mut GpImage, *const GUID, UINT) -> GpStatus>;

/// C 20-21: `typedef GpStatus (WINGDIPAPI *GdipCreateBitmapFromFile_Proc) (WCHAR *,
/// GpBitmap **);`
#[cfg (feature = "windowsnt")]
pub type GdipCreateBitmapFromFile_Proc
  = Option<unsafe extern "system" fn (*mut WCHAR, *mut *mut GpBitmap) -> GpStatus>;

/// C 22-23: `typedef GpStatus (WINGDIPAPI *GdipCreateBitmapFromStream_Proc) (IStream *,
/// GpBitmap **);`
#[cfg (feature = "windowsnt")]
pub type GdipCreateBitmapFromStream_Proc
  = Option<unsafe extern "system" fn (*mut IStream, *mut *mut GpBitmap) -> GpStatus>;

/// C 24-25: `typedef GpStatus (WINGDIPAPI *GdipCreateBitmapFromScan0_Proc) (INT, INT, INT,
/// PixelFormat, BYTE*, GpBitmap**);`
#[cfg (feature = "windowsnt")]
pub type GdipCreateBitmapFromScan0_Proc
  = Option<unsafe extern "system" fn (INT, INT, INT, PixelFormat, *mut BYTE,
                                      *mut *mut GpBitmap) -> GpStatus>;

/// C 26-27: `typedef GpStatus (WINGDIPAPI *GdipCreateBitmapFromHBITMAP_Proc) (HBITMAP hbm,
/// HPALETTE hpal, GpBitmap** bitmap);`  C 53-54 repeats this typedef word for word; Rust has one
/// name per scope, so the repeat is recorded here rather than written twice.
#[cfg (feature = "windowsnt")]
pub type GdipCreateBitmapFromHBITMAP_Proc
  = Option<unsafe extern "system" fn (HBITMAP, HPALETTE, *mut *mut GpBitmap) -> GpStatus>;

/// C 28-29: `typedef GpStatus (WINGDIPAPI *GdipSetInterpolationMode_Proc) (GpGraphics *graphics,
/// InterpolationMode interpolationMode);`
#[cfg (feature = "windowsnt")]
pub type GdipSetInterpolationMode_Proc
  = Option<unsafe extern "system" fn (*mut GpGraphics, InterpolationMode) -> GpStatus>;

/// C 30-34: `typedef GpStatus (WINGDIPAPI *GdipDrawImageRectRectI_Proc) (GpGraphics *graphics,
/// GpImage *image, INT dstx, INT dsty, INT dstwidth, INT dstheight, INT srcx, INT srcy,
/// INT srcwidth, INT srcheight, GpUnit srcUnit, GDIPCONST GpImageAttributes* imageAttributes,
/// DrawImageAbort callback, VOID * callbackData);`
#[cfg (feature = "windowsnt")]
pub type GdipDrawImageRectRectI_Proc
  = Option<unsafe extern "system" fn (*mut GpGraphics, *mut GpImage, INT, INT, INT, INT, INT,
                                      INT, INT, INT, GpUnit, *const GpImageAttributes,
                                      DrawImageAbort, *mut VOID) -> GpStatus>;

/// C 35: `typedef IStream * (WINAPI *SHCreateMemStream_Proc) (const BYTE *, UINT);`
#[cfg (feature = "windowsnt")]
pub type SHCreateMemStream_Proc
  = Option<unsafe extern "system" fn (*const BYTE, UINT) -> *mut IStream>;

/// C 36-37: `typedef GpStatus (WINGDIPAPI *GdipCreateHBITMAPFromBitmap_Proc) (GpBitmap *,
/// HBITMAP *, ARGB);`
#[cfg (feature = "windowsnt")]
pub type GdipCreateHBITMAPFromBitmap_Proc
  = Option<unsafe extern "system" fn (*mut GpBitmap, *mut HBITMAP, ARGB) -> GpStatus>;

/// C 38: `typedef GpStatus (WINGDIPAPI *GdipDisposeImage_Proc) (GpImage *);`
#[cfg (feature = "windowsnt")]
pub type GdipDisposeImage_Proc
  = Option<unsafe extern "system" fn (*mut GpImage) -> GpStatus>;

/// C 39: `typedef GpStatus (WINGDIPAPI *GdipGetImageHeight_Proc) (GpImage *, UINT *);`
#[cfg (feature = "windowsnt")]
pub type GdipGetImageHeight_Proc
  = Option<unsafe extern "system" fn (*mut GpImage, *mut UINT) -> GpStatus>;

/// C 40: `typedef GpStatus (WINGDIPAPI *GdipGetImageWidth_Proc) (GpImage *, UINT *);`
#[cfg (feature = "windowsnt")]
pub type GdipGetImageWidth_Proc
  = Option<unsafe extern "system" fn (*mut GpImage, *mut UINT) -> GpStatus>;

/// C 41: `typedef GpStatus (WINGDIPAPI *GdipGetImageEncodersSize_Proc) (UINT *, UINT *);`
#[cfg (feature = "windowsnt")]
pub type GdipGetImageEncodersSize_Proc
  = Option<unsafe extern "system" fn (*mut UINT, *mut UINT) -> GpStatus>;

/// C 42-43: `typedef GpStatus (WINGDIPAPI *GdipGetImageEncoders_Proc) (UINT, UINT,
/// ImageCodecInfo *);`
#[cfg (feature = "windowsnt")]
pub type GdipGetImageEncoders_Proc
  = Option<unsafe extern "system" fn (UINT, UINT, *mut ImageCodecInfo) -> GpStatus>;

/// C 44-45: `typedef GpStatus (WINGDIPAPI *GdipLoadImageFromFile_Proc) (GDIPCONST WCHAR *,
/// GpImage **);`
#[cfg (feature = "windowsnt")]
pub type GdipLoadImageFromFile_Proc
  = Option<unsafe extern "system" fn (*const WCHAR, *mut *mut GpImage) -> GpStatus>;

/// C 46-47: `typedef GpStatus (WINGDIPAPI *GdipGetImageThumbnail_Proc) (GpImage *, UINT, UINT,
/// GpImage**, GetThumbnailImageAbort, VOID *);`
#[cfg (feature = "windowsnt")]
pub type GdipGetImageThumbnail_Proc
  = Option<unsafe extern "system" fn (*mut GpImage, UINT, UINT, *mut *mut GpImage,
                                      GetThumbnailImageAbort, *mut VOID) -> GpStatus>;

/// C 48-50: `typedef GpStatus (WINGDIPAPI *GdipSaveImageToFile_Proc) (GpImage *,
/// GDIPCONST WCHAR *, GDIPCONST CLSID *, GDIPCONST EncoderParameters *);`
#[cfg (feature = "windowsnt")]
pub type GdipSaveImageToFile_Proc
  = Option<unsafe extern "system" fn (*mut GpImage, *const WCHAR, *const CLSID,
                                      *const EncoderParameters) -> GpStatus>;

/// C 51-52: `typedef GpStatus (WINGDIPAPI *GdipImageRotateFlip_Proc) (GpImage *image,
/// RotateFlipType rfType);`
#[cfg (feature = "windowsnt")]
pub type GdipImageRotateFlip_Proc
  = Option<unsafe extern "system" fn (*mut GpImage, RotateFlipType) -> GpStatus>;

/// C 56-83: the loaded entry points.  C declares them in one `extern` list; the port keeps that
/// shape in one `unsafe extern "C"` block - these are data symbols, so no calling convention is
/// involved (the `include/w32.rs` 331-336 reading of its own `_Proc` objects).  C 68 and C 83 both
/// declare `fn_GdipCreateBitmapFromHBITMAP`; it is written once, at C 68's place in the order.
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut fn_GdiplusStartup: GdiplusStartup_Proc;                          // C 56
  pub static mut fn_GdiplusShutdown: GdiplusShutdown_Proc;                        // C 57
  pub static mut fn_GdipCreateFromHDC: GdipCreateFromHDC_Proc;                    // C 58
  pub static mut fn_GdipDeleteGraphics: GdipDeleteGraphics_Proc;                  // C 59
  pub static mut fn_GdipGetPropertyItemSize: GdipGetPropertyItemSize_Proc;        // C 60
  pub static mut fn_GdipGetPropertyItem: GdipGetPropertyItem_Proc;                // C 61
  pub static mut fn_GdipImageGetFrameDimensionsCount:
    GdipImageGetFrameDimensionsCount_Proc;                                        // C 62
  pub static mut fn_GdipImageGetFrameDimensionsList:
    GdipImageGetFrameDimensionsList_Proc;                                         // C 63
  pub static mut fn_GdipImageGetFrameCount: GdipImageGetFrameCount_Proc;          // C 64
  pub static mut fn_GdipImageSelectActiveFrame:
    GdipImageSelectActiveFrame_Proc;                                              // C 65
  pub static mut fn_GdipCreateBitmapFromFile: GdipCreateBitmapFromFile_Proc;      // C 66
  pub static mut fn_GdipCreateBitmapFromStream: GdipCreateBitmapFromStream_Proc;  // C 67
  pub static mut fn_GdipCreateBitmapFromHBITMAP:
    GdipCreateBitmapFromHBITMAP_Proc;                                             // C 68
  pub static mut fn_GdipDrawImageRectRectI: GdipDrawImageRectRectI_Proc;          // C 69
  pub static mut fn_GdipSetInterpolationMode: GdipSetInterpolationMode_Proc;      // C 70
  pub static mut fn_GdipCreateBitmapFromScan0: GdipCreateBitmapFromScan0_Proc;    // C 71
  pub static mut fn_SHCreateMemStream: SHCreateMemStream_Proc;                    // C 72
  pub static mut fn_GdipCreateHBITMAPFromBitmap:
    GdipCreateHBITMAPFromBitmap_Proc;                                             // C 73
  pub static mut fn_GdipDisposeImage: GdipDisposeImage_Proc;                      // C 74
  pub static mut fn_GdipGetImageHeight: GdipGetImageHeight_Proc;                  // C 75
  pub static mut fn_GdipGetImageWidth: GdipGetImageWidth_Proc;                    // C 76
  pub static mut fn_GdipGetImageEncodersSize: GdipGetImageEncodersSize_Proc;      // C 77
  pub static mut fn_GdipGetImageEncoders: GdipGetImageEncoders_Proc;              // C 78
  pub static mut fn_GdipLoadImageFromFile: GdipLoadImageFromFile_Proc;            // C 79
  pub static mut fn_GdipGetImageThumbnail: GdipGetImageThumbnail_Proc;            // C 80
  pub static mut fn_GdipSaveImageToFile: GdipSaveImageToFile_Proc;                // C 81
  pub static mut fn_GdipImageRotateFlip: GdipImageRotateFlip_Proc;                // C 82
}

/* C 85-112: the `#undef`s of the SDK's own macros.  C 111 spells `GdipSaveImageRotateFlip`,
   a name the file never defines (C 140 defines `GdipImageRotateFlip`); the port keeps C's text
   here rather than silently correcting it.  Rust has no `#undef` counterpart, so the 27 undefs
   are recorded in the 27 `/// C:` lines below, one per name, next to the `#define` that
   replaces them.  C 96 and C 112 both `#undef GdipCreateBitmapFromHBITMAP`, C 125 and C 141 both
   `#define` it; the pair is written once.  */

/// C 85/114: `# undef GdiplusStartup` then `# define GdiplusStartup fn_GdiplusStartup`.  C renames
/// each GDI+ entry point to its loaded pointer; the port keeps one name per object and spells the
/// alias as a nullary macro (the `include/macfont.rs` 62-66 reading of an object-like `#define`).
#[cfg (feature = "windowsnt")]
macro_rules! GdiplusStartup { () => { fn_GdiplusStartup }; }

/// C 86/115: `# undef GdiplusShutdown` / `# define GdiplusShutdown fn_GdiplusShutdown`.
#[cfg (feature = "windowsnt")]
macro_rules! GdiplusShutdown { () => { fn_GdiplusShutdown }; }

/// C 87/116: `# undef GdipGetPropertyItemSize` / `# define GdipGetPropertyItemSize
/// fn_GdipGetPropertyItemSize`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipGetPropertyItemSize { () => { fn_GdipGetPropertyItemSize }; }

/// C 88/117: `# undef GdipGetPropertyItem` / `# define GdipGetPropertyItem
/// fn_GdipGetPropertyItem`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipGetPropertyItem { () => { fn_GdipGetPropertyItem }; }

/// C 89/118: `# undef GdipImageGetFrameDimensionsCount` / `# define
/// GdipImageGetFrameDimensionsCount fn_GdipImageGetFrameDimensionsCount`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipImageGetFrameDimensionsCount {
  () => { fn_GdipImageGetFrameDimensionsCount };
}

/// C 90/119: `# undef GdipImageGetFrameDimensionsList` / `# define
/// GdipImageGetFrameDimensionsList fn_GdipImageGetFrameDimensionsList`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipImageGetFrameDimensionsList {
  () => { fn_GdipImageGetFrameDimensionsList };
}

/// C 91/120: `# undef GdipImageGetFrameCount` / `# define GdipImageGetFrameCount
/// fn_GdipImageGetFrameCount`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipImageGetFrameCount { () => { fn_GdipImageGetFrameCount }; }

/// C 92/121: `# undef GdipImageSelectActiveFrame` / `# define GdipImageSelectActiveFrame
/// fn_GdipImageSelectActiveFrame`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipImageSelectActiveFrame { () => { fn_GdipImageSelectActiveFrame }; }

/// C 93/122: `# undef GdipCreateBitmapFromFile` / `# define GdipCreateBitmapFromFile
/// fn_GdipCreateBitmapFromFile`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipCreateBitmapFromFile { () => { fn_GdipCreateBitmapFromFile }; }

/// C 94/123: `# undef GdipCreateBitmapFromStream` / `# define GdipCreateBitmapFromStream
/// fn_GdipCreateBitmapFromStream`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipCreateBitmapFromStream { () => { fn_GdipCreateBitmapFromStream }; }

/// C 95/124: `# undef GdipCreateBitmapFromScan0` / `# define GdipCreateBitmapFromScan0
/// fn_GdipCreateBitmapFromScan0`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipCreateBitmapFromScan0 { () => { fn_GdipCreateBitmapFromScan0 }; }

/// C 96+112/125+141: `# undef GdipCreateBitmapFromHBITMAP` / `# define
/// GdipCreateBitmapFromHBITMAP fn_GdipCreateBitmapFromHBITMAP` - each twice in C (see the
/// typedef note), written once here.
#[cfg (feature = "windowsnt")]
macro_rules! GdipCreateBitmapFromHBITMAP { () => { fn_GdipCreateBitmapFromHBITMAP }; }

/// C 97/126: `# undef GdipCreateFromHDC` / `# define GdipCreateFromHDC fn_GdipCreateFromHDC`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipCreateFromHDC { () => { fn_GdipCreateFromHDC }; }

/// C 98/127: `# undef GdipDrawImageRectRectI` / `# define GdipDrawImageRectRectI
/// fn_GdipDrawImageRectRectI`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipDrawImageRectRectI { () => { fn_GdipDrawImageRectRectI }; }

/// C 99/128: `# undef GdipSetInterpolationMode` / `# define GdipSetInterpolationMode
/// fn_GdipSetInterpolationMode`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipSetInterpolationMode { () => { fn_GdipSetInterpolationMode }; }

/// C 100/129: `# undef GdipDeleteGraphics` / `# define GdipDeleteGraphics
/// fn_GdipDeleteGraphics` (C writes two spaces after the name in the `#define`).
#[cfg (feature = "windowsnt")]
macro_rules! GdipDeleteGraphics { () => { fn_GdipDeleteGraphics }; }

/// C 101/130: `# undef SHCreateMemStream` / `# define SHCreateMemStream fn_SHCreateMemStream`.
#[cfg (feature = "windowsnt")]
macro_rules! SHCreateMemStream { () => { fn_SHCreateMemStream }; }

/// C 102/131: `# undef GdipCreateHBITMAPFromBitmap` / `# define GdipCreateHBITMAPFromBitmap
/// fn_GdipCreateHBITMAPFromBitmap`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipCreateHBITMAPFromBitmap { () => { fn_GdipCreateHBITMAPFromBitmap }; }

/// C 103/132: `# undef GdipDisposeImage` / `# define GdipDisposeImage fn_GdipDisposeImage`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipDisposeImage { () => { fn_GdipDisposeImage }; }

/// C 104/133: `# undef GdipGetImageHeight` / `# define GdipGetImageHeight
/// fn_GdipGetImageHeight`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipGetImageHeight { () => { fn_GdipGetImageHeight }; }

/// C 105/134: `# undef GdipGetImageWidth` / `# define GdipGetImageWidth fn_GdipGetImageWidth`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipGetImageWidth { () => { fn_GdipGetImageWidth }; }

/// C 106/135: `# undef GdipGetImageEncodersSize` / `# define GdipGetImageEncodersSize
/// fn_GdipGetImageEncodersSize`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipGetImageEncodersSize { () => { fn_GdipGetImageEncodersSize }; }

/// C 107/136: `# undef GdipGetImageEncoders` / `# define GdipGetImageEncoders
/// fn_GdipGetImageEncoders`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipGetImageEncoders { () => { fn_GdipGetImageEncoders }; }

/// C 108/137: `# undef GdipLoadImageFromFile` / `# define GdipLoadImageFromFile
/// fn_GdipLoadImageFromFile`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipLoadImageFromFile { () => { fn_GdipLoadImageFromFile }; }

/// C 109/138: `# undef GdipGetImageThumbnail` / `# define GdipGetImageThumbnail
/// fn_GdipGetImageThumbnail`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipGetImageThumbnail { () => { fn_GdipGetImageThumbnail }; }

/// C 110/139: `# undef GdipSaveImageToFile` / `# define GdipSaveImageToFile
/// fn_GdipSaveImageToFile`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipSaveImageToFile { () => { fn_GdipSaveImageToFile }; }

/// C 111: `# undef GdipSaveImageRotateFlip` - C's own spelling; no typedef or object of that name
/// exists in the file (C 140 renames `GdipImageRotateFlip`).  Recorded as C writes it.
// C: # undef GdipSaveImageRotateFlip   (C 111; the name C 140 defines is GdipImageRotateFlip)

/// C 140/111: `# define GdipImageRotateFlip fn_GdipImageRotateFlip` / `# undef
/// GdipImageRotateFlip`.
#[cfg (feature = "windowsnt")]
macro_rules! GdipImageRotateFlip { () => { fn_GdipImageRotateFlip }; }

/* C 144-145: the two entry points `src/w32image.c` defines - **outside** the guard (C 142
   already closed it), so they are live text on this target.  `CLSID` is `<windows.h>`'s and
   `HWND` is a handle, so both keep P-55's untyped pointer with C's prototype above; `Lisp_Object`
   is the enclosing unit's, the `include/nsxwidget.rs` 26-28 reading of PORTING.md 0.1-9 (this
   header includes no `lisp.h`, and neither does that one).  */
/// C: extern int w32_gdip_get_encoder_clsid (const char *, CLSID *);
/// C: extern int w32_gdip_export_frame (HWND, Lisp_Object, CLSID *);
unsafe extern "C" {
  pub fn w32_gdip_get_encoder_clsid (mime_type: *const c_char, clsid: *mut c_void) -> c_int;
  pub fn w32_gdip_export_frame (hwnd: *mut c_void, lisp_object: Lisp_Object,
                               clsid: *mut c_void) -> c_int;
}
