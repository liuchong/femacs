// femacs: port of src/w32gui.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header is the Windows GUI layer's X-compatibility
// surface: C 19-20 is the include guard, C 21 and C 23 the two includes, C 26-28 the menu
// allocator macros, C 30-36 the handle typedefs and the handle-to-integer macro, C 39-44 the
// `XImage` record, C 46-57 the image entry points and the application instance, and C 59-112
// the X compatibility block (the eleven gravity, the eight value and the ten position
// constants, `NativeRectangle` and the three rectangle conversions).  No `#if` anywhere
// (`tools/check-cond-eval.py src/w32gui.h` -> `1 conditions, 0 unknown, 0 disagreements
// [VERIFIED]`, the guard), so every line below is live text.
//
// Nothing on this target compiles it: the frozen configuration's `WINDOW_SYSTEM_OBJ` is empty
// (docs/baseline/frozen-build-sources.txt; docs/status.md 3.31), so no unit `include!`s this
// mirror and the declarations are deferred to the Windows target with their definitions.
//
// **The names C takes from `<windows.h>` (C 21) are spelled the way the port's other platform
// mirrors spell them, and one of them carries a `cfg`.**  Every handle here is `void *` at the ABI
// (`HBITMAP`, `HWND`, `HDC`, `HCURSOR`, `HINSTANCE`), so the four typedefs and C 57's object are
// `*mut c_void` - the `haikugui.rs`/`androidgui.rs` reading of their own handle typedefs.
// **`Emacs_Cursor` is behind `#[cfg (feature = "have_window_system")]`**: `dispextern.h` 175 declares
// the same name on the other side of `#ifndef HAVE_WINDOW_SYSTEM` (`dispextern.rs` 62 carries
// that arm), so a crate that includes both mirrors sees two items and reports `E0428` - measured
// when `xtask layout-check` compiles every mirror in one crate.  C's own guards keep the two
// apart, and the port uses the feature that names the window-system build (PORTING.md P-51's
// convention: the feature is the non-frozen side).  The `androidterm.rs` 35-37 note records the
// same interaction for Android's alias, from the other end.
//
// The two `<windows.h>` entry points the macros below call (`GetProcessHeap`, `HeapAlloc`,
// `HeapFree` and the `HEAP_ZERO_MEMORY` flag, C 26-28) have no owner mirror either; they are
// *named* in the macro bodies and not declared, exactly as C's text takes the declaration from
// its own include - a `macro_rules!` body is not resolved until it is expanded, so the mirror
// needs no invented declaration, and a Windows unit supplies them from that target (PORTING.md
// 0.1-9, P-72; the `nt/inc/include/ms-w32.rs` convention for the platform's own names).

// C 19-20: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const EMACS_W32GUI_H: bool = true;

/* FIXME: old local memory management for menus.  */
// C 26-28: C's object-like `local_heap` is a nullary `macro_rules!` here; the three system
// names are `<windows.h>`'s (see the note above) and C's tokens are kept in the bodies.
/// C: #define local_heap (GetProcessHeap ())
macro_rules! local_heap { () => { GetProcessHeap () }; }
/// C: #define local_alloc(n) (HeapAlloc (local_heap, HEAP_ZERO_MEMORY, (n)))
macro_rules! local_alloc { ($n:expr) => { HeapAlloc (local_heap (), HEAP_ZERO_MEMORY, ($n)) }; }
/// C: #define local_free(p) (HeapFree (local_heap, 0, ((LPVOID) (p))))
/// C's `(LPVOID)` cast is kept as `as LPVOID`, the name the Windows target supplies together
/// with the three `Heap*` entries (a macro body is not resolved until it is expanded).
macro_rules! local_free { ($p:expr) => { HeapFree (local_heap (), 0, ($p) as LPVOID) }; }

/// C 30-34: the platform's handle typedefs, each `void *` at the ABI.  C 33's comment (the
// `HDC` spelling) is why this header's `Display` is not the X11 one.
/// C: typedef HBITMAP Emacs_Pixmap;
/// C: typedef HWND Window;
/// C: typedef HDC Display;  /* HDC so it doesn't conflict with xpm lib.  */
/// C: typedef HCURSOR Emacs_Cursor;
pub type Emacs_Pixmap = *mut c_void;
pub type Window = *mut c_void;
pub type Display = *mut c_void;
/// C 34.  The alias is the window-system side of `dispextern.h` 175's pair, so it carries the
/// feature (see the file note above); without it the single-crate layout probe reports two
/// `Emacs_Cursor` items.
/// C: typedef HCURSOR Emacs_Cursor;
#[cfg (feature = "have_window_system")]
pub type Emacs_Cursor = *mut c_void;

/// C 36: `#define WINDOW_HANDLE_UINTPTR(h) ((uintptr_t) (h))` - C's cast is the port's `as usize`
/// (`uintptr_t` is PORTING.md 1.1's `usize`), the `haikugui.rs` 39 spelling.
macro_rules! WINDOW_HANDLE_UINTPTR { ($h:expr) => { ($h) as usize }; }

/// C 39-44: `typedef struct _XImage { unsigned char *data; BITMAPINFO info; } XImage;` -
/// **deferred**.  `BITMAPINFO` is a `<wingdi.h>` record C embeds **by value**, the port has no
/// mirror of that header, and no compiler on this target could measure it, so the mirror carries
/// no invented layout and the record's checklist box stays unchecked (its `data` member would be
/// `*mut u8`, C 41).  The extractor names the deferral on stderr ("missing type deferred") rather
/// than failing a check, which is what the unchecked box is for.

/// C 46-51: C's C 46 `struct image;` is a forward declaration of `dispextern.h` 3215's record,
/// which this header does not include - so it has **no item of its own** here (the `haikuterm.rs`
/// reading of `struct haiku_font_change_event;`) and both record pointers stay `*mut c_void`
/// (PORTING.md P-55/P-72 rule 3).  `Lisp_Object` stays named (lisp.h is every unit's include,
/// PORTING.md 0.1-9).
/// C: extern int w32_load_image (struct frame *f, struct image *img,
/// C:                            Lisp_Object spec_file, Lisp_Object spec_data);
/// C: extern bool w32_can_use_native_image_api (Lisp_Object);
/// C: extern bool w32_gdiplus_startup (void);
/// C: extern void w32_gdiplus_shutdown (void);
// The six names are definitions in the Windows GUI units (`w32image.c` implements the image
// entry points; `w32fns.c` defines C 57's `hinst`), so a crate-root declaration here collides
// with the unit's own item as `E0428` - measured for `hinst` by
// `xtask probe src/w32fns.rs … --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'`
// (`docs/ports/w32fns.md` owed 5).  The declarations therefore sit in modules this mirror
// glob-imports, which a crate-root definition shadows (the `window.rs` 302-367 /
// `androidterm.rs` 513-537 / `w32term.rs` `w32term_fn_decls` shape; PORTING.md 0.1-10 / P-21).
pub mod w32gui_fn_decls {
  use super::*;

  unsafe extern "C" {
    pub fn w32_load_image (f: *mut c_void, img: *mut c_void,
                           spec_file: Lisp_Object, spec_data: Lisp_Object) -> c_int;
    pub fn w32_can_use_native_image_api (spec: Lisp_Object) -> bool;
    pub fn w32_gdiplus_startup () -> bool;
    pub fn w32_gdiplus_shutdown ();

    /// C 53: `extern size_t w32_image_size (Emacs_Pixmap);`
    pub fn w32_image_size (pixmap: Emacs_Pixmap) -> usize;
  }
}
use w32gui_fn_decls::*;

/// C 57: `extern HINSTANCE hinst;` - the application instance, a handle, which `w32fns.c`
/// defines.
pub mod w32gui_objs {
  use super::*;

  unsafe extern "C" {
    pub static mut hinst: *mut c_void;
  }
}
use w32gui_objs::*;

/// C 55: `#define FACE_DEFAULT (~0)` - C's `~0` on an `int`, which is `!0` in Rust.
pub const FACE_DEFAULT: c_int = !0;

/* Bit Gravity */

/// C 61-71: the eleven gravity constants (their order is X's).
pub const ForgetGravity: c_int = 0;
pub const NorthWestGravity: c_int = 1;
pub const NorthGravity: c_int = 2;
pub const NorthEastGravity: c_int = 3;
pub const WestGravity: c_int = 4;
pub const CenterGravity: c_int = 5;
pub const EastGravity: c_int = 6;
pub const SouthWestGravity: c_int = 7;
pub const SouthGravity: c_int = 8;
pub const SouthEastGravity: c_int = 9;
pub const StaticGravity: c_int = 10;

/// C 73-80: the eight value-mask constants (C spells them with `0x`).
pub const NoValue: c_int = 0x0000;
pub const XValue: c_int = 0x0001;
pub const YValue: c_int = 0x0002;
pub const WidthValue: c_int = 0x0004;
pub const HeightValue: c_int = 0x0008;
pub const AllValues: c_int = 0x000F;
pub const XNegative: c_int = 0x0010;
pub const YNegative: c_int = 0x0020;

/// C 82-92: the ten position flags.  C's `1L` is `c_long` (PORTING.md 1.1), and its `(1L << n)`
/// keeps C's token pair in the value the constant check compares (`haikugui.rs` 126-134 spells
/// the same block the same way).
pub const USPosition: c_long = 1 << 0; /* user specified x, y */
pub const USSize: c_long = 1 << 1; /* user specified width, height */

pub const PPosition: c_long = 1 << 2; /* program specified position */
pub const PSize: c_long = 1 << 3; /* program specified size */
pub const PMinSize: c_long = 1 << 4; /* program specified minimum size */
pub const PMaxSize: c_long = 1 << 5; /* program specified maximum size */
pub const PResizeInc: c_long = 1 << 6; /* program specified resize increments */
pub const PAspect: c_long = 1 << 7; /* program specified min and max aspect ratios */
pub const PBaseSize: c_long = 1 << 8; /* program specified base for incrementing */
pub const PWinGravity: c_long = 1 << 9; /* program specified window gravity */

/// C 94: `#define NativeRectangle RECT` - a *type* macro, so it is a `macro_rules!` with no
/// metavariables (the `haikugui.rs` 63 / `androidgui.rs` 241 spelling): a `pub type` alias is not
/// what the gate reads for a macro, and C uses the name in type position.  `RECT` is
/// `<windef.h>`'s record, named here and declared nowhere in the port (see the file note).
macro_rules! NativeRectangle { () => { RECT }; }

/// C 96-100 / C 102-106 / C 108-112: the three rectangle conversions.  C's bodies are comma
/// expressions; Rust has no comma operator, so each body is a block of the same assignments in
/// C's order (the `haikugui.rs` 70-95 reading of the same block).
///
/// **Two of the three carry C's implicit integer conversions as written-out casts** (the
/// `w32term_fn_decls` block's note and `docs/ports/w32term.md`'s owed 8): this header's
/// `NativeRectangle` is `RECT` (`LONG`, the unit's `i32`) while `Emacs_Rectangle`'s non-X arm is
/// `int x, y; unsigned width, height` (C 66-70), so C's `(xr).width = (nr).right - (nr).left`
/// is an `int`-to-`unsigned` conversion and its `(nr).right = (xr).x + (xr).width` is
/// `int + unsigned` in unsigned arithmetic, then a conversion to `LONG`.  Rust refuses both
/// implicit forms (`E0308`/`E0277`, measured by `xtask probe src/w32term.rs … --cfg
/// 'feature="have_ntgui"' --cfg 'feature="windowsnt"'), so each is spelled with C's own
/// conversions: the `as c_uint` / `as c_int` the C assignment performs.
/// C: #define CONVERT_TO_EMACS_RECT(xr,nr)            \
/// C:   ((xr).x = (nr).left,				\
/// C:    (xr).y = (nr).top,				\
/// C:    (xr).width = ((nr).right - (nr).left),	\
/// C:    (xr).height = ((nr).bottom - (nr).top))
macro_rules! CONVERT_TO_EMACS_RECT {
  ($xr:expr, $nr:expr) =>
    { {
        ($xr).x = ($nr).left;
        ($xr).y = ($nr).top;
        ($xr).width = (($nr).right - ($nr).left) as c_uint;
        ($xr).height = (($nr).bottom - ($nr).top) as c_uint;
      } };
}
/// C 102-106.
macro_rules! CONVERT_FROM_EMACS_RECT {
  ($xr:expr, $nr:expr) =>
    { {
        ($nr).left = ($xr).x;
        ($nr).top = ($xr).y;
        ($nr).right = (($xr).x as c_uint).wrapping_add (($xr).width) as c_int;
        ($nr).bottom = (($xr).y as c_uint).wrapping_add (($xr).height) as c_int;
      } };
}
/// C 108-112.
macro_rules! STORE_NATIVE_RECT {
  ($nr:expr, $x:expr, $y:expr, $width:expr, $height:expr) =>
    { {
        ($nr).left = ($x);
        ($nr).top = ($y);
        ($nr).right = ($nr).left + ($width);
        ($nr).bottom = ($nr).top + ($height);
      } };
}

/* src/w32gui.h 38-44: "Windows equivalent of XImage",
   `typedef struct _XImage { unsigned char *data; BITMAPINFO info; } XImage;`.
   Dead platform text on the frozen target, carried under the feature that makes the file
   reachable at all: its only two includers are `dispextern.h` 135 and `w32term.h` 21, and both
   sit inside a `HAVE_NTGUI` guard, so `have_ntgui` is the gate - the `windows` feature it used
   to carry (`WINDOWS`, an unrelated macro) answered a question this header never asks, and the
   Windows GUI build is the one configuration that needs the record (docs/ports/w32fns.md's
   consumption of `XImage` is what reads it).  `BITMAPINFO` is C's spelling, which nothing
   compiles here.  */
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct _XImage
{
  /* unsigned char *data;  */
  pub data: *mut c_uchar,
  /* BITMAPINFO info;  */
  pub info: *mut c_void,
}
#[cfg (feature = "have_ntgui")]
pub type XImage = _XImage;
