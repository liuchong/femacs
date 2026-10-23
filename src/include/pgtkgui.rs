// femacs: port of src/pgtkgui.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): typedefs, constants and macros only.  `pgtkgui.h` is the
// pure-GTK (pgtk) window system's `gui.h`-shaped header: C 19-121 is unconditional text with no
// `#if` of its own, and the guards that decide whether it is compiled are its *includers'* -
// `dispextern.h` 151-157 (`#ifdef HAVE_PGTK`) and `src/pgtkterm.h` 29's own `#ifdef HAVE_PGTK`
// (`include/pgtkterm.rs` records that this target compiles none of that: `HAVE_PGTK` and
// `USE_GTK` are `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, and no `src/pgtk*.c`
// is in `docs/baseline/frozen-build-sources.txt`).
//
// The two names this header repeats are what make that guard visible, and the frozen clang
// measures it: `clang -std=gnu23 -O0 -I emacs/src` on a TU that includes `dispextern.h` and then
// `pgtkgui.h` fails with `typedef redefinition with different types ('struct _GdkCursor *' vs
// 'void *')` (pgtkgui.h 46 against dispextern.h 175) and `'NativeRectangle' macro redefined`
// (101 against dispextern.h 179).  In a pgtk build `HAVE_PGTK` implies `HAVE_WINDOW_SYSTEM`, so
// both of those fallbacks are dead and C 46/101 win - which is why the port gives the *repeating*
// names a feature and leaves the rest live.
//
// The split follows `include/macfont.rs` (3.170), `include/w32font.rs` (3.176) and
// `include/ftfont.rs` (3.178).  Live text is what a plain target can compile as C compiles it:
// `_XCharStruct`/`XCharStruct`, `XRectangle`, `unichar`/`XChar2b`, the three `XCHAR2B` macros,
// `Color` and `XrmDatabase` - none of them collides with a mirror in this tree, and the two
// records are measured below.  Two groups carry `#[cfg (feature = "have_pgtk")]`, P-51's arm for
// the non-frozen side: the names that name a GDK record (`Emacs_Cursor`, `Display`) and the
// names another *live* mirror already declares (`include/w32gui.rs` 57-148 holds `Window`, the
// eleven gravity, eight value and ten position constants, `NativeRectangle` and the three
// rectangle conversions; 68 holds `WINDOW_HANDLE_UINTPTR`) - the `include/msdos.rs` 207-228
// reading of the same collision.  The feature is this header's own build macro rather than
// `USE_GTK`, because a pgtk build defines both (`configure.ac` 3826 sets `USE_GTK` for every GTK
// build) and only `HAVE_PGTK` names the side whose text this is.

// C 19-20: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const __PGTKGUI_H__: bool = true;

/* C 22-30: `typedef struct _XCharStruct { int rbearing, lbearing, width, ascent, descent; }
   XCharStruct;` - "Emulate XCharStruct".  The tag and the typedef name are C's own (support for
   a pgtk build cannot take them from <X11/Xlib.h>, which a pure-GTK build does not include), and
   the frozen clang can measure them, so every number below is asserted.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _XCharStruct
{
  pub rbearing: c_int,
  pub lbearing: c_int,
  pub width: c_int,
  pub ascent: c_int,
  pub descent: c_int,
}
pub type XCharStruct = _XCharStruct;

// Measured: `clang -std=gnu23 -O0 -I emacs/src` on a TU that includes this header and prints
// `sizeof`/`_Alignof`/`offsetof`: 20, 4, and 0/4/8/12/16.
const _: () = core::assert! (std::mem::size_of::<_XCharStruct> () == 20);
const _: () = core::assert! (std::mem::align_of::<_XCharStruct> () == 4);
const _: () = core::assert! (std::mem::offset_of! (_XCharStruct, rbearing) == 0);
const _: () = core::assert! (std::mem::offset_of! (_XCharStruct, lbearing) == 4);
const _: () = core::assert! (std::mem::offset_of! (_XCharStruct, width) == 8);
const _: () = core::assert! (std::mem::offset_of! (_XCharStruct, ascent) == 12);
const _: () = core::assert! (std::mem::offset_of! (_XCharStruct, descent) == 16);

/* C 32-34: "Fake structure from Xlib.h to represent two-byte characters."  `unsigned short` is
   PORTING.md 1.1's `c_ushort`; the frozen clang measures 2 bytes and alignment 2.  */
pub type unichar = c_ushort;
pub type XChar2b = unichar;

const _: () = core::assert! (std::mem::size_of::<XChar2b> () == 2);
const _: () = core::assert! (std::mem::align_of::<XChar2b> () == 2);

/* C 36-43: the three accessors.  C's `(XChar2b)` cast is the port's `as XChar2b`, and the
   bit masks keep C's spelling (`0x00ff`/`0xff00`).  */
/// C: #define STORE_XCHAR2B(chp, b1, b2) \
/// C:   (*(chp) = ((XChar2b)((((b1) & 0x00ff) << 8) | ((b2) & 0x00ff))))
macro_rules! STORE_XCHAR2B {
  ($chp:expr, $b1:expr, $b2:expr) =>
    { *($chp) = (((($b1) & 0x00ff) << 8) | (($b2) & 0x00ff)) as XChar2b };
}

/// C: #define XCHAR2B_BYTE1(chp) ((*(chp) & 0xff00) >> 8)
macro_rules! XCHAR2B_BYTE1 {
  ($chp:expr) => { (*(chp) & 0xff00) >> 8 };
}

/// C: #define XCHAR2B_BYTE2(chp) (*(chp) & 0x00ff)
macro_rules! XCHAR2B_BYTE2 {
  ($chp:expr) => { *(chp) & 0x00ff };
}

/// C 46: `typedef struct _GdkCursor *Emacs_Cursor;`.  The pointee is GDK's incomplete record, so
/// the pointer keeps P-55's ABI-exact stand-in with C's declaration here; the name is also
/// `dispextern.h` 175's (`dispextern.rs` 62) - C's `HAVE_PGTK`/`HAVE_WINDOW_SYSTEM` keep the two
/// apart, and `layout-check`'s single crate needs the feature for the same reason.
#[cfg (feature = "have_pgtk")]
pub type Emacs_Cursor = *mut c_void;

/// C 48: `typedef void *Color;` - C leaves the pointee untyped, so the port does too.  No other
/// mirror in this tree declares the name.
pub type Color = *mut c_void;

/// C 49: `typedef int Window;` - pgtk emulates X's window ID as `int`.  `include/w32gui.rs` 57
/// declares the same name for the Windows handle, so this copy carries the pgtk feature.
#[cfg (feature = "have_pgtk")]
pub type Window = c_int;

// C 50: `typedef struct _GdkDisplay Display;` - recorded, not mirrored.  The record is GDK's and
// incomplete in C too, and Rust cannot alias a name it does not have (measured: `pub type
// Display = _GdkDisplay;` alone is `error[E0425]: cannot find type`, /tmp probe).  The pgtk
// target supplies the name, the `include/emacsgtkfixed.rs` reading of `EmacsFixedPrivate` (a
// typedef of an incomplete record that no item here needs).
// C: typedef struct _GdkDisplay Display;

/// C 52: `#define WINDOW_HANDLE_UINTPTR(h) ((uintptr_t) (h))` - C's cast is the port's `as usize`
/// (`uintptr_t` is PORTING.md 1.1's `usize`, the `include/w32gui.rs` 66 reading).  The NAME is
/// declared by `include/w32gui.rs` 68 too, hence the feature.
/// C: #define WINDOW_HANDLE_UINTPTR(h) ((uintptr_t) (h))
#[cfg (feature = "have_pgtk")]
macro_rules! WINDOW_HANDLE_UINTPTR {
  ($h:expr) => { ($h) as usize };
}

/* C 54-55: "Xism" - X's resource database handle, which C leaves untyped here because a pgtk
   build has no Xlib.  The file note above says the *repeating* names get a feature; this one and
   `XRectangle` below were the two it did not, and a pgtk flip read them as `E0428` against
   `include/xlib.rs` (`XrmDatabase` 38, `XRectangle` 871 - measured on the `xsettings` arm B,
   status 3.532).  The gate is the "not a configuration" form the ACL mirror uses for the same
   situation (status 3.510): a build is either X11 or pgtk, and `pgtkgui.h`'s own copy is the
   pgtk side's.  */
#[cfg (all (feature = "have_pgtk", not (feature = "have_x_windows")))]
pub type XrmDatabase = *mut c_void;

/* C 58-64: "Some sort of attempt to normalize rectangle handling..".  The struct is an
   anonymous typedef in C; a Rust struct needs a name, so the port uses the typedef's name and
   records the deviation (PORTING.md P-67, the `dispextern.rs` `Emacs_Rectangle` reading).
   Measured: 16 bytes, alignment 4, offsets 0/4/8/12.  C's X11 arm has a *different* record under
   the same name (Xlib's, 8 bytes: four `short`s), which is exactly why this one is the pgtk
   side's and carries the same "not a configuration" gate as `XrmDatabase` above.  */
#[cfg (all (feature = "have_pgtk", not (feature = "have_x_windows")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XRectangle
{
  pub x: c_int,
  pub y: c_int,
  pub width: c_uint,
  pub height: c_uint,
}

const _: () = core::assert! (std::mem::size_of::<XRectangle> () == 16);
const _: () = core::assert! (std::mem::align_of::<XRectangle> () == 4);
const _: () = core::assert! (std::mem::offset_of! (XRectangle, x) == 0);
const _: () = core::assert! (std::mem::offset_of! (XRectangle, y) == 4);
const _: () = core::assert! (std::mem::offset_of! (XRectangle, width) == 8);
const _: () = core::assert! (std::mem::offset_of! (XRectangle, height) == 12);

/* Bit gravity (C 66-77).  C's implicit type is `int`; `frame.c` is the consumer C 66 names.  */
#[cfg (feature = "have_pgtk")]
pub const ForgetGravity: c_int = 0;
#[cfg (feature = "have_pgtk")]
pub const NorthWestGravity: c_int = 1;
#[cfg (feature = "have_pgtk")]
pub const NorthGravity: c_int = 2;
#[cfg (feature = "have_pgtk")]
pub const NorthEastGravity: c_int = 3;
#[cfg (feature = "have_pgtk")]
pub const WestGravity: c_int = 4;
#[cfg (feature = "have_pgtk")]
pub const CenterGravity: c_int = 5;
#[cfg (feature = "have_pgtk")]
pub const EastGravity: c_int = 6;
#[cfg (feature = "have_pgtk")]
pub const SouthWestGravity: c_int = 7;
#[cfg (feature = "have_pgtk")]
pub const SouthGravity: c_int = 8;
#[cfg (feature = "have_pgtk")]
pub const SouthEastGravity: c_int = 9;
#[cfg (feature = "have_pgtk")]
pub const StaticGravity: c_int = 10;

/* C 79-86: the value-mask constants, spelled as C's hex literals.  */
#[cfg (feature = "have_pgtk")]
pub const NoValue: c_int = 0x0000;
#[cfg (feature = "have_pgtk")]
pub const XValue: c_int = 0x0001;
#[cfg (feature = "have_pgtk")]
pub const YValue: c_int = 0x0002;
#[cfg (feature = "have_pgtk")]
pub const WidthValue: c_int = 0x0004;
#[cfg (feature = "have_pgtk")]
pub const HeightValue: c_int = 0x0008;
#[cfg (feature = "have_pgtk")]
pub const AllValues: c_int = 0x000F;
#[cfg (feature = "have_pgtk")]
pub const XNegative: c_int = 0x0010;
#[cfg (feature = "have_pgtk")]
pub const YNegative: c_int = 0x0020;

/* C 88-98: the ten position flags.  C's `1L` is `c_long` (PORTING.md 1.1), and its `(1L << n)`
   keeps C's token pair in the value the constant check compares.  */
#[cfg (feature = "have_pgtk")]
pub const USPosition: c_long = 1 << 0;        /* user specified x, y */
#[cfg (feature = "have_pgtk")]
pub const USSize: c_long = 1 << 1;            /* user specified width, height */
#[cfg (feature = "have_pgtk")]
pub const PPosition: c_long = 1 << 2;         /* program specified position */
#[cfg (feature = "have_pgtk")]
pub const PSize: c_long = 1 << 3;             /* program specified size */
#[cfg (feature = "have_pgtk")]
pub const PMinSize: c_long = 1 << 4;          /* program specified minimum size */
#[cfg (feature = "have_pgtk")]
pub const PMaxSize: c_long = 1 << 5;          /* program specified maximum size */
#[cfg (feature = "have_pgtk")]
pub const PResizeInc: c_long = 1 << 6;        /* program specified resize increments */
#[cfg (feature = "have_pgtk")]
pub const PAspect: c_long = 1 << 7;           /* program specified min, max aspect ratios */
#[cfg (feature = "have_pgtk")]
pub const PBaseSize: c_long = 1 << 8;         /* program specified base for incrementing */
#[cfg (feature = "have_pgtk")]
pub const PWinGravity: c_long = 1 << 9;       /* program specified window gravity */

/* C 101: `#define NativeRectangle XRectangle` - a *type* macro, so it is a `macro_rules!` with
   no metavariables (the `include/w32gui.rs` 148 / `include/haikugui.rs` 64 spelling): a `pub
   type` alias is not what the gate reads for a macro, and C uses the name in type position.  */
#[cfg (feature = "have_pgtk")]
macro_rules! NativeRectangle {
  () => { XRectangle };
}

/* C 103-119: the three rectangle conversions.  C's bodies are comma expressions; Rust has no
   comma operator, so each body is a block of the same assignments in C's order (the
   `include/w32gui.rs` 150-186 reading of the same block).  */
/// C: #define CONVERT_TO_EMACS_RECT(xr, nr) \
/// C:   ((xr).x = (nr).x, (xr).y = (nr).y, (xr).width = (nr).width, (xr).height = (nr).height)
#[cfg (feature = "have_pgtk")]
macro_rules! CONVERT_TO_EMACS_RECT {
  ($xr:expr, $nr:expr) =>
    { {
        ($xr).x = ($nr).x;
        ($xr).y = ($nr).y;
        ($xr).width = ($nr).width;
        ($xr).height = ($nr).height;
      } };
}

/// C: #define CONVERT_FROM_EMACS_RECT(xr, nr) \
/// C:   ((nr).x = (xr).x, (nr).y = (xr).y, (nr).width = (xr).width, (nr).height = (xr).height)
#[cfg (feature = "have_pgtk")]
macro_rules! CONVERT_FROM_EMACS_RECT {
  ($xr:expr, $nr:expr) =>
    { {
        ($nr).x = ($xr).x;
        ($nr).y = ($xr).y;
        ($nr).width = ($xr).width;
        ($nr).height = ($xr).height;
      } };
}

/// C: #define STORE_NATIVE_RECT(nr, px, py, pwidth, pheight) \
/// C:   ((nr).x = (px), (nr).y = (py), (nr).width = (pwidth), (nr).height = (pheight))
#[cfg (feature = "have_pgtk")]
macro_rules! STORE_NATIVE_RECT {
  ($nr:expr, $px:expr, $py:expr, $pwidth:expr, $pheight:expr) =>
    { {
        ($nr).x = ($px);
        ($nr).y = ($py);
        ($nr).width = ($pwidth);
        ($nr).height = ($pheight);
      } };
}
