// femacs: mirror of the X11 SDK's <X11/Xlib.h> (system header of another target's toolchain, PORTING.md P-26/P-49).

// `<X11/Xlib.h>` is the X11 toolchain's own header: it is not part of the reference tree, and the
// frozen target has no such header, so it has no C file to be paired with.  This mirror is the
// port's declaration of the Xlib surface the *gated* arms of `src/*.c` and `src/*.h` name, the way
// `include/jni.rs` mirrors the Android NDK's `<jni.h>` and `lib/include/haiku_support.rs` the Haiku
// SDK's - `tools/check-target-names.py` accepts the first line above as that declaration.
//
// It carries **two** headers: `<X11/Xlib.h>` for the display/screen procedures and the
// `XPointer`/`Bool` scalars, and `<X11/Xresource.h>` for the resource-manager ("Xrm") types and
// procedures.  They live in one file because they are one surface to the port - C's `src/xrdb.c`,
// the first consumer, reaches both through `xterm.h` -> this mirror - and because a second mirror
// would need a second first line and a second ledger for eight typedefs.  (`src/xrdb.c` 37-41
// lists five headers for this one mirror - the pair above plus `<X11/X.h>`, `<X11/Xatom.h>` and
// `<X11/Xutil.h>` - because the *macros* those three declare have no mirror of their own: they
// live in the `x11_constants` module below, each row citing its own header 3.336 established the
// file with; `<xcb/*>` joins the same way at the end of the file, under `use_xcb`.)
//
// Every value and every record layout here is **measured**, from the X11 headers this machine has
// (`/opt/homebrew/include`: `xorgproto` 2025.1 and `libX11`), with the command that produced it in
// this ledger's Evidence: the macros are X11 protocol constants (fixed since X11R1), and the two
// records are `XRectangle` (Xlib.h 426-429) and `XrmValue` (Xresource.h 160-163), both asserted
// where they are declared.  `spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h` reads the
// whole `MonitorInfo` record as `[sizeof=32, align=8]` in its x11 arm - its two `Emacs_Rectangle`
// members are this `XRectangle`, so `include/dispextern.rs` aliases them to it under
// `have_x_windows`.
//
// The macros live in a module rather than at this file's top level: a unit that includes the
// mirror wants C's `None`, but a top-level `None` would shadow Rust's prelude `Option::None` for
// the whole unit, so a consumer imports the few names it needs where it needs them.

/// Xlib.h 426-429: `typedef struct { short x, y; unsigned short width, height; } XRectangle;`
/// - its consumers are the X11 arms *and* the pgtk arms (`src/pgtkim.c` passes one to
/// `pgtk_im_set_cursor_location`), so the gate is the union (status 3.496).
/* X11-only, *not* `any (have_x_windows, have_pgtk)`: C's pgtk builds have no Xlib and get these
   two names from `pgtkgui.h`'s own definitions (a 16-byte `{ int x, y; unsigned width, height; }`
   and a `void *`), which `include/pgtkgui.rs` carries under `all (have_pgtk, not
   (have_x_windows))` (status 3.533).  With the `any` gate both were live in a pgtk arm and it read
   `E0428` for `XRectangle` and `XrmDatabase` (measured on the `xsettings` pgtk-only arm, status
   3.542).  This file's own note at `XrmDatabase` below already says the name "belongs to the PGTK
   line to move, not to this one to duplicate".  */
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XRectangle
{
  pub x: c_short,                        // C: short x, y;
  pub y: c_short,
  pub width: c_ushort,                   // C: unsigned short width, height;
  pub height: c_ushort,
}

/// Xlib.h 426-429's record is eight bytes with align 2 (the reading quoted above; `-DHAVE_X_WINDOWS`
/// clang dump of `struct MonitorInfo`'s two members).
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XRectangle> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XRectangle> () == 2);

#[cfg (feature = "have_x_windows")]
pub mod x11_constants {
  // A nested module does not see the including unit's imports, so the three scalar aliases this
  // module's constants are typed with are named here rather than inherited from the crate root
  // (PORTING.md 0.1-9 forbids a `crate::` path; `core::ffi` is the absolute one).  Measured:
  // without this line all twenty-one constants are `E0425: cannot find type c_long` in the
  // x11-arm probe, which buries the arm's real gaps among twenty-one scoping errors.
  use core::ffi::{c_int, c_long, c_ulong};

  /// X11/X.h 115: `#define None 0L` - "universal null resource or null atom".
  pub const None: c_long = 0;
  /// X11/X.h 566-567: `#define GCForeground (1L<<2)` / `#define GCBackground (1L<<3)` - the two
  /// value masks an X11 build ORs into `XCreateGC`'s valuemask (measured with clang against the
  /// real header: `GCForeground=4 GCBackground=8`).  `include/dispextern.rs`'s pair of the same
  /// names is the *fallback* arm's (C's `ANDROID_GC_*` values, `1`/`2`) and carries
  /// `not (feature = "have_x_windows")` so the two arms cannot both be live (status 3.535).
  #[cfg (feature = "have_x_windows")]
  pub const GCForeground: c_int = 1 << 2;
  #[cfg (feature = "have_x_windows")]
  pub const GCBackground: c_int = 1 << 3;

  /// X11/X.h 350: `#define Success 0` - "everything's okay".
  pub const Success: c_int = 0;
  /// X11/X.h 121: `#define CopyFromParent 0L` - the CreateWindow border/colormap value.
  pub const CopyFromParent: c_long = 0;
  /// X11/X.h 131: `#define AnyPropertyType 0L` - the special Atom `XGetWindowProperty` accepts
  /// for "any type".
  pub const AnyPropertyType: c_long = 0;
  /// X11/X.h 150: `#define NoEventMask 0L`.
  pub const NoEventMask: c_long = 0;
  /// X11/X.h 168: `#define StructureNotifyMask (1L<<17)`.
  pub const StructureNotifyMask: c_long = 1 << 17;
  /// X11/X.h 173: `#define PropertyChangeMask (1L<<22)`.
  pub const PropertyChangeMask: c_long = 1 << 22;
  /// X11/X.h 212: `#define ClientMessage 33` - the `XEvent` union member `xclient` selects.
  pub const ClientMessage: c_int = 33;
  /// X11/X.h 388: `#define InputOnly 2` - the `XCreateWindow` class.
  pub const InputOnly: c_int = 2;
  /// X11/X.h 401: `#define CWOverrideRedirect (1L<<9)`.
  pub const CWOverrideRedirect: c_long = 1 << 9;
  /// X11/X.h 445: `#define IsUnmapped 0` - an `XWindowAttributes.map_state`.
  pub const IsUnmapped: c_int = 0;
  /// X11/X.h 456: `#define DestroyAll 0` - `XSetCloseDownMode`'s first mode.
  pub const DestroyAll: c_int = 0;
  /// X11/X.h 457: `#define RetainPermanent 1` - the mode that keeps the client slot.
  pub const RetainPermanent: c_int = 1;
  /// X11/X.h 475: `#define PropModeReplace 0`.
  pub const PropModeReplace: c_int = 0;
  /// X11/X.h 477: `#define PropModeAppend 2`.
  pub const PropModeAppend: c_int = 2;
  /// X11/Xatom.h 12: `#define XA_ATOM ((Atom) 4)`; C's `Atom` is `unsigned long`.
  pub const XA_ATOM: c_ulong = 4;
  /// X11/Xatom.h 14: `#define XA_CARDINAL ((Atom) 6)`.
  pub const XA_CARDINAL: c_ulong = 6;
  /// X11/Xatom.h 41: `#define XA_WINDOW ((Atom) 33)`.
  pub const XA_WINDOW: c_ulong = 33;
  /// X11/extensions/shapeconst.h 46: `#define ShapeBounding 0`.
  pub const ShapeBounding: c_int = 0;
  /// X11/extensions/shapeconst.h 48: `#define ShapeInput 2`.
  pub const ShapeInput: c_int = 2;
  /// X11/extensions/shapeconst.h 50: `#define ShapeNotifyMask (1L << 0)`.
  pub const ShapeNotifyMask: c_long = 1;
  /* The seven names `src/xterm.c`'s slices 6 and 7 name and this module did not carry: five from
     `<X11/X.h>` (one `map_state` value, two event *names* and their two masks), one from
     `<X11/Xatom.h>`, and one from `<X11/Xutil.h>`.  Each value below is
     measured on this machine's `xorgproto` headers (`grep -n` on `/opt/homebrew/include/X11`);
     the one `<X11/Xutil.h>` row is here rather than in a mirror of its own because this file is
     already the port's one mirror for that header - `src/xrdb.c` 37-41 lists `<X11/Xutil.h>` with
     `<X11/Xlib.h>`, `<X11/Xatom.h>`, `<X11/X.h>` and `<X11/Xresource.h>` as the five it includes,
     and the three `shapeconst.h` rows above are the same practice for a third-party extension
     header (P-72 rule 1 owns a *declaration* to the header that declares it; a macro whose header
     has no mirror of its own lands in this module, and no second home is created for it).  */
  /// X11/X.h 447: `#define IsViewable 2` - a mapped window's `XWindowAttributes.map_state`
  /// (its neighbours are `IsUnmapped 0` at 445 and `IsUnviewable 1` at 446).
  pub const IsViewable: c_int = 2;
  /// X11/Xatom.h 9: `#define XA_PRIMARY ((Atom) 1)` - C's `Atom` is `unsigned long`.
  pub const XA_PRIMARY: c_ulong = 1;
  /// X11/X.h 183: `#define ButtonPress 4` - an `XEvent` type, not the mask below.
  pub const ButtonPress: c_int = 4;
  /// X11/X.h 184: `#define ButtonRelease 5`.
  pub const ButtonRelease: c_int = 5;
  /// X11/X.h 153: `#define ButtonPressMask (1L<<2)` - a `XGrabButton`/event-mask bit.
  pub const ButtonPressMask: c_long = 1 << 2;
  /// X11/X.h 154: `#define ButtonReleaseMask (1L<<3)`.
  pub const ButtonReleaseMask: c_long = 1 << 3;
  /// X11/Xutil.h 147: `#define NormalState 1` - "most applications want to start this way"; the
  /// `WM_STATE` property's first-long value (`src/xterm.c` 3734 compares against it).
  pub const NormalState: c_int = 1;

  /* The eleven input-event masks `include/xterm.rs` 144-155's `STANDARD_EVENT_SET` body names
     (two of them were already here), plus the image format and visual-class values the X arms
     pass.  Each value is read on this machine's `xorgproto` headers; `STANDARD_EVENT_SET` is the
     one consumer, and it imports them with a targeted `use` (`include/xterm.rs` 152) rather than
     a glob, because this module's `None`/`Success` must not shadow the prelude's `None` for a
     whole unit - the file note above.  */
  /// X11/X.h 151: `#define KeyPressMask (1L<<0)`.
  pub const KeyPressMask: c_long = 1 << 0;
  /// X11/X.h 155: `#define EnterWindowMask (1L<<4)`.
  pub const EnterWindowMask: c_long = 1 << 4;
  /// X11/X.h 156: `#define LeaveWindowMask (1L<<5)`.
  pub const LeaveWindowMask: c_long = 1 << 5;
  /// X11/X.h 157: `#define PointerMotionMask (1L<<6)`.
  pub const PointerMotionMask: c_long = 1 << 6;
  /// X11/X.h 166: `#define ExposureMask (1L<<15)`.
  pub const ExposureMask: c_long = 1 << 15;
  /// X11/X.h 167: `#define VisibilityChangeMask (1L<<16)`.
  pub const VisibilityChangeMask: c_long = 1 << 16;
  /// X11/X.h 172: `#define FocusChangeMask (1L<<21)`.
  pub const FocusChangeMask: c_long = 1 << 21;
  /// X11/X.h 608: `#define ZPixmap 2` - "depth == drawable depth" (`XGetImage`'s `format`).
  pub const ZPixmap: c_int = 2;
  /// X11/X.h 704: `#define StaticGray 0`; 705 `GrayScale 1`; 706 `StaticColor 2`; 707
  /// `PseudoColor 3`; 708 `TrueColor 4`; 709 `DirectColor 5` - the `Visual`/`XVisualInfo`
  /// `class` values.  `include/xterm.rs` 2231-2241's `visual_class` run reads them.
  pub const StaticGray: c_int = 0;
  pub const GrayScale: c_int = 1;
  pub const StaticColor: c_int = 2;
  pub const PseudoColor: c_int = 3;
  pub const TrueColor: c_int = 4;
  pub const DirectColor: c_int = 5;
  /// X11/Xutil.h 339-342: `#define BitmapSuccess 0` / `BitmapOpenFailed 1` /
  /// `BitmapFileInvalid 2` / `BitmapNoMemory 3` - `XReadBitmapFile`'s result values.
  pub const BitmapSuccess: c_int = 0;
  pub const BitmapOpenFailed: c_int = 1;
  pub const BitmapFileInvalid: c_int = 2;
  pub const BitmapNoMemory: c_int = 3;
}

/* The Xlib scalar typedefs the gated arms name.  `Bool` is Xlib's own alias for `int` - C spells
   it as a macro, Xlib.h 78, so the port writes the alias C's body names, and carries it because
   `XrmQGetResource` below returns it.  `XPointer` is `char *` (Xlib.h 76).  `Display` and `Screen`
   are records this mirror does **not** define - `Display` is incomplete even in C (Xlib.h 483),
   and `Screen`'s definition is Xlib's own 19-member record (Xlib.h 253-271) that nothing in this
   tree reads - so where a prototype takes one, it degrades to `*mut c_void` with C's text in a
   `/// C:` line above it (PORTING.md P-72 rule 3).  */

/// Xlib.h 76: `typedef char *XPointer;`
#[cfg (feature = "have_x_windows")]
pub type XPointer = *mut c_char;

/// Xlib.h 78: `#define Bool int`
#[cfg (feature = "have_x_windows")]
pub type Bool = c_int;

/* `<X11/Xresource.h>`: the resource-manager surface `src/xrdb.c` is written against.  The scalars
   are one-line typedefs and are declared; the two records that appear only as pointers here
   (`XrmDatabase` is `struct _XrmHashBucketRec *`, Xresource.h 175) keep C's text in a `/// C:`
   line and degrade to `*mut c_void` (P-72 rule 3).  */

/// Xresource.h 175: `typedef struct _XrmHashBucketRec *XrmDatabase;` - the name `src/xrdb.c`'s
/// whole surface is written with.  **Its home is this mirror** (P-72 rule 1: the header that
/// declares it is `<X11/Xresource.h>`, which is this section); the identical line in
/// `include/pgtkgui.rs` 122 is unreachable - nothing in the tree `include!`s that mirror (measured
/// 3.334) - and it belongs to the PGTK line to move, not to this one to duplicate.  C's pointee is
/// an incomplete tag, so the port keeps C's text in this comment and the untyped pointer (rule 3).
#[cfg (feature = "have_x_windows")]
pub type XrmDatabase = *mut c_void;

/// Xresource.h 84: `typedef int XrmQuark, *XrmQuarkList;`
#[cfg (feature = "have_x_windows")]
pub type XrmQuark = c_int;
#[cfg (feature = "have_x_windows")]
pub type XrmQuarkList = *mut XrmQuark;

/// Xresource.h 87: `typedef char *XrmString;`
#[cfg (feature = "have_x_windows")]
pub type XrmString = *mut c_char;

/// Xresource.h 136: `typedef XrmQuark XrmName;`
#[cfg (feature = "have_x_windows")]
pub type XrmName = XrmQuark;
/// Xresource.h 137: `typedef XrmQuarkList XrmNameList;`
#[cfg (feature = "have_x_windows")]
pub type XrmNameList = XrmQuarkList;

/// Xresource.h 142: `typedef XrmQuark XrmClass;`
#[cfg (feature = "have_x_windows")]
pub type XrmClass = XrmQuark;
/// Xresource.h 143: `typedef XrmQuarkList XrmClassList;`
#[cfg (feature = "have_x_windows")]
pub type XrmClassList = XrmQuarkList;

/// Xresource.h 156: `typedef XrmQuark XrmRepresentation;`
#[cfg (feature = "have_x_windows")]
pub type XrmRepresentation = XrmQuark;

/// Xresource.h 160-163: `typedef struct { unsigned int size; XPointer addr; } XrmValue,
/// *XrmValuePtr;` - C passes this record **by value** (`XrmValue value;`, `src/xrdb.c` 478), so
/// the record itself is carried.  C's typedef has no tag, so the Rust struct takes the typedef's
/// name and records the deviation (PORTING.md P-67, the `XRectangle` reading above).
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XrmValue
{
  pub size: c_uint,   // C: unsigned int size;
  pub addr: XPointer,   // C: XPointer addr;
}

/// Xresource.h 160-163, second typedef.
#[cfg (feature = "have_x_windows")]
pub type XrmValuePtr = *mut XrmValue;

/// Xresource.h 160-163's record is sixteen bytes with align 8 and offsets 0/8 - a clang probe
/// against the installed header (this ledger's Evidence), asserted here because a wrong `addr`
/// width would move every consumer's `<XrmValue>`.
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XrmValue> () == 16);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XrmValue> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XrmValue, size) == 0);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XrmValue, addr) == 8);

/* The procedures.  `XDefaultScreenOfDisplay` is not named by C's `src/xrdb.c` directly - it is
   what the `DefaultScreenOfDisplay` macro below expands to here; the rest are C 386-486's own
   calls, and C 372's `XrmStringType` closes the list.  */

#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: Xlib.h 1832: `extern Screen *XDefaultScreenOfDisplay(Display*);`
  pub fn XDefaultScreenOfDisplay (display: *mut c_void) -> *mut c_void;
  /// C: Xlib.h 1715: `extern char *XResourceManagerString(Display*);`
  pub fn XResourceManagerString (display: *mut c_void) -> *mut c_char;
  /// C: Xlib.h 1718: `extern char *XScreenResourceString(Screen*);`
  pub fn XScreenResourceString (screen: *mut c_void) -> *mut c_char;
  /// C: Xlib.h 1483: `extern void XrmInitialize(void);`
  pub fn XrmInitialize ();
  /// C: Xresource.h 266: `extern XrmDatabase XrmGetFileDatabase(_Xconst char*);`
  pub fn XrmGetFileDatabase (filename: *const c_char) -> *mut c_void;
  /// C: Xresource.h 276: `extern XrmDatabase XrmGetStringDatabase(_Xconst char*);`
  pub fn XrmGetStringDatabase (data: *const c_char) -> *mut c_void;
  /// C: Xresource.h 285: `extern void XrmMergeDatabases(XrmDatabase, XrmDatabase*);`
  pub fn XrmMergeDatabases (source_db: *mut c_void, target_db: *mut *mut c_void);
  /// C: Xresource.h 210: `extern void XrmPutLineResource(XrmDatabase*, _Xconst char*);`
  pub fn XrmPutLineResource (database: *mut *mut c_void, line: *const c_char);
  /// C: Xresource.h 215-223: `extern Bool XrmQGetResource(XrmDatabase, XrmNameList, XrmClassList,
  /// XrmRepresentation*, XrmValue*);`
  pub fn XrmQGetResource (database: *mut c_void, quark_name: XrmNameList,
                          quark_class: XrmClassList,
                          quark_type_return: *mut XrmRepresentation,
                          value_return: *mut XrmValue) -> Bool;
  /// C: Xresource.h 91-93: `extern XrmQuark XrmStringToQuark(_Xconst char*);`
  pub fn XrmStringToQuark (string: *const c_char) -> XrmQuark;
  /// C: Xresource.h 119-122: `extern void XrmStringToQuarkList(_Xconst char*, XrmQuarkList);`
  pub fn XrmStringToQuarkList (string: *const c_char, quarks_return: XrmQuarkList);
}

/* The three Xlib/Xrm macros the X11 arms name.  C's `XrmStringTo*` pair are plain aliases of the
   real procedure, so the port keeps C's macro name and C's body.  `DefaultScreenOfDisplay` is the
   one **degradation** in this file: C's body (`Xlib.h` 120) reads `ScreenOfDisplay`/`DefaultScreen`,
   which dereference `_XPrivDisplay` - Xlib's complete-inline copy of its private display record
   (`Xlib.h` 489-546) that no mirror here carries (measured: the C macro expands inline and names
   **no** symbol, while `XDefaultScreenOfDisplay` is a symbol and returns the same `Screen *`).  */

/// Xlib.h 120: `#define DefaultScreenOfDisplay(dpy) ScreenOfDisplay(dpy,DefaultScreen(dpy))`
#[cfg (feature = "have_x_windows")]
macro_rules! DefaultScreenOfDisplay {
  ($dpy:expr) => { XDefaultScreenOfDisplay ($dpy) };
}

/// Xresource.h 140: `#define XrmStringToNameList(str, name) XrmStringToQuarkList(str, name)`
#[cfg (feature = "have_x_windows")]
macro_rules! XrmStringToNameList {
  ($str:expr, $name:expr) => { XrmStringToQuarkList ($str, $name) };
}

/// Xresource.h 146: `#define XrmStringToClassList(str,c_class) XrmStringToQuarkList(str, c_class)`
#[cfg (feature = "have_x_windows")]
macro_rules! XrmStringToClassList {
  ($str:expr, $c_class:expr) => { XrmStringToQuarkList ($str, $c_class) };
}

/// `src/xrdb.c` 372 defines this one in the unit rather than in a header (`#define XrmStringType
/// "String"`), and it is kept here with the rest of the Xrm surface so the name has one home:
/// `src/xrdb.rs` will take it from this mirror instead of declaring a second copy (PORTING.md
/// 0.1-3).  C's literal carries a terminating NUL and C 386 hands it to a C function
/// (`XrmStringToQuark (XrmStringType)`), so the port carries it as a `&CStr` and the use site
/// spells `XrmStringType.as_ptr ()` - the shape `src/emacs.rs` gives every `<config.h>` string
/// read by C (status 3.64), not the `&str` shape `pI`/`pD` use for Rust-side format strings.
/// Measured: the extractor compares this constant's *bytes*, and both `&str`/`"String"` and
/// `&CStr`/`c"String"` pass it (a wrong value fails either way; the scratch pair is in the
/// ledger's Evidence).
#[cfg (feature = "have_x_windows")]
pub const XrmStringType: &core::ffi::CStr = c"String";

/* `xcb/xcb.h`, `xcb/xproto.h` and `xcb/shape.h`: the xcb surface `src/xterm.c`'s `USE_XCB` arms
   name, and the home of the `xcb_connection_t`/`xcb_visualtype_t` pointers `xterm.h` 842-843
   declares.  C reaches the first two under `#ifdef USE_XCB` (`src/xterm.c` 643-646 includes
   `<xcb/xproto.h>` at 644 and `<xcb/xcb.h>` at 645) and again through `xterm.h` 117-119's
   `<X11/Xlib-xcb.h>`; the third under `#ifdef HAVE_XCB_SHAPE` (`src/xterm.c` 681-683).  They join
   the two headers above in this one file for the reason those two share it - a second mirror
   would need a second first line and a second ledger - and because what a consumer needs from them
   is not macros at all but the members of two *named* enums, which C declares beside the records
   below.

   Every record is **measured** against the XCB headers this machine has
   (`/opt/homebrew/include/xcb`, the same `libxcb` tree the `<X11>` values above were measured
   against): the sizes, alignments and offsets in the assertions below come from a C probe
   compiled and run against those headers, and each field list from the header named on the
   record's own `///` line.  C's names are kept (`xcb_visualtype_t`'s `_class` field is `_class`
   in C too), the one deviation being the raw identifier `r#type` for C's `type` member
   (PORTING.md 0.2, the rule the xterm.h mirror's `r#use` followed); `xcb_generic_error_t` is C's
   *anonymous* struct typedef, so the Rust record takes the typedef's name (P-67, the `XrmValue`
   reading).  A named C enum becomes a Rust enum (`xm_drag_style`'s shape) and an anonymous one
   the constants alone (PORTING.md 1.3).

   The gating is C's own reachability: the `xcb.h`/`xproto.h` half is `use_xcb` (C 643-646), the
   `shape.h` half is `use_xcb` **and** `have_xcb_shape` (C 681-683, which sits inside the
   `USE_XCB` block's own shape arms), and both sit under `have_x_windows` like every other
   declaration in this file.  Nothing here is a `stub`: a declaration whose C companion this tree
   cannot compile on the frozen target is still carried with C's text and a measured layout, and
   the two records the arms read fields of (`xcb_get_property_reply_t`,
   `xcb_get_window_attributes_reply_t`) assert those fields' offsets.  */

/// xproto.h 38: `typedef uint32_t xcb_window_t;` - an X window id.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub type xcb_window_t = u32;

/// xproto.h 104: `typedef uint32_t xcb_atom_t;` - the `xcb_*` spelling of Xlib's `Atom`.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub type xcb_atom_t = u32;

/// xproto.h 93: `typedef uint32_t xcb_colormap_t;`.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub type xcb_colormap_t = u32;

/// xproto.h 115: `typedef uint32_t xcb_drawable_t;` - `xcb_get_geometry`'s argument.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub type xcb_drawable_t = u32;

/// xproto.h 148: `typedef uint32_t xcb_visualid_t;`.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub type xcb_visualid_t = u32;

/// shape.h 38: `typedef uint8_t xcb_shape_kind_t;` - the `source_kind` argument of
/// `xcb_shape_get_rectangles` (the `XCB_SHAPE_SK_*` values below are a *different* C enum, whose
/// members C converts into this type implicitly; the cast belongs at the use site).
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
pub type xcb_shape_kind_t = u8;

/// xcb.h 108: `typedef struct xcb_connection_t xcb_connection_t;` - "Opaque structure containing
/// all data that XCB needs to communicate with an X server."  C's typedef names an incomplete
/// struct that no file of the port reads through, so the port keeps C's name on an opaque record
/// (P-70's stand-in, the `JNIInvokeInterface` shape in `include/jni.rs`).
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub struct xcb_connection_t;

/// xproto.h 1970-1972: `typedef struct xcb_get_property_cookie_t { unsigned int sequence; }
/// xcb_get_property_cookie_t;` - C hands a cookie **by value** to `xcb_get_property_reply`, so
/// the record itself is carried, not a pointer to it.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_get_property_cookie_t
{
  pub sequence: c_uint,   // C: unsigned int sequence;
}

/// xproto.h 1994-2003: `typedef struct xcb_get_property_reply_t { uint8_t response_type;
/// uint8_t format; uint16_t sequence; uint32_t length; xcb_atom_t type; uint32_t bytes_after;
/// uint32_t value_len; uint8_t pad0[12]; } xcb_get_property_reply_t;` - the reply `xterm.c`
/// reads `format` and `type` from.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_get_property_reply_t
{
  pub response_type: u8,   // C: uint8_t response_type;
  pub format: u8,   // C: uint8_t format;
  pub sequence: u16,   // C: uint16_t sequence;
  pub length: u32,   // C: uint32_t length;
  pub r#type: xcb_atom_t,   // C: xcb_atom_t type;
  pub bytes_after: u32,   // C: uint32_t bytes_after;
  pub value_len: u32,   // C: uint32_t value_len;
  pub pad0: [u8; 12],   // C: uint8_t pad0[12];
}

/// xproto.h 1562-1564: `typedef struct xcb_get_window_attributes_cookie_t { unsigned int
/// sequence; } xcb_get_window_attributes_cookie_t;`.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_get_window_attributes_cookie_t
{
  pub sequence: c_uint,   // C: unsigned int sequence;
}

/// xproto.h 1582-1602: the `xcb_get_window_attributes` reply, whose `map_state` and
/// `your_event_mask` the `USE_XCB` arm reads (C 3250-3256's `attrs->map_state`/`your_event_mask`).
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_get_window_attributes_reply_t
{
  pub response_type: u8,   // C: uint8_t response_type;
  pub backing_store: u8,   // C: uint8_t backing_store;
  pub sequence: u16,   // C: uint16_t sequence;
  pub length: u32,   // C: uint32_t length;
  pub visual: xcb_visualid_t,   // C: xcb_visualid_t visual;
  pub _class: u16,   // C: uint16_t _class;
  pub bit_gravity: u8,   // C: uint8_t bit_gravity;
  pub win_gravity: u8,   // C: uint8_t win_gravity;
  pub backing_planes: u32,   // C: uint32_t backing_planes;
  pub backing_pixel: u32,   // C: uint32_t backing_pixel;
  pub save_under: u8,   // C: uint8_t save_under;
  pub map_is_installed: u8,   // C: uint8_t map_is_installed;
  pub map_state: u8,   // C: uint8_t map_state;
  pub override_redirect: u8,   // C: uint8_t override_redirect;
  pub colormap: xcb_colormap_t,   // C: xcb_colormap_t colormap;
  pub all_event_masks: u32,   // C: uint32_t all_event_masks;
  pub your_event_mask: u32,   // C: uint32_t your_event_mask;
  pub do_not_propagate_mask: u16,   // C: uint16_t do_not_propagate_mask;
  pub pad0: [u8; 2],   // C: uint8_t pad0[2];
}

/// xproto.h 8210-8212: `typedef struct xcb_translate_coordinates_cookie_t { unsigned int
/// sequence; } xcb_translate_coordinates_cookie_t;`.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_translate_coordinates_cookie_t
{
  pub sequence: c_uint,   // C: unsigned int sequence;
}

/// xproto.h 8236-8244: `typedef struct xcb_translate_coordinates_reply_t { uint8_t
/// response_type; uint8_t same_screen; uint16_t sequence; uint32_t length; xcb_window_t child;
/// int16_t dst_x; int16_t dst_y; } xcb_translate_coordinates_reply_t;`.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_translate_coordinates_reply_t
{
  pub response_type: u8,   // C: uint8_t response_type;
  pub same_screen: u8,   // C: uint8_t same_screen;
  pub sequence: u16,   // C: uint16_t sequence;
  pub length: u32,   // C: uint32_t length;
  pub child: xcb_window_t,   // C: xcb_window_t child;
  pub dst_x: i16,   // C: int16_t dst_x;
  pub dst_y: i16,   // C: int16_t dst_y;
}

/// xproto.h 6534-6536: `typedef struct xcb_get_geometry_cookie_t { unsigned int sequence; }
/// xcb_get_geometry_cookie_t;`.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_get_geometry_cookie_t
{
  pub sequence: c_uint,   // C: unsigned int sequence;
}

/// xproto.h 6562-6574: `typedef struct xcb_get_geometry_reply_t { uint8_t response_type;
/// uint8_t depth; uint16_t sequence; uint32_t length; xcb_window_t root; int16_t x; int16_t y;
/// uint16_t width; uint16_t height; uint16_t border_width; uint8_t pad0[2]; }
/// xcb_get_geometry_reply_t;`.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_get_geometry_reply_t
{
  pub response_type: u8,   // C: uint8_t response_type;
  pub depth: u8,   // C: uint8_t depth;
  pub sequence: u16,   // C: uint16_t sequence;
  pub length: u32,   // C: uint32_t length;
  pub root: xcb_window_t,   // C: xcb_window_t root;
  pub x: i16,   // C: int16_t x;
  pub y: i16,   // C: int16_t y;
  pub width: u16,   // C: uint16_t width;
  pub height: u16,   // C: uint16_t height;
  pub border_width: u16,   // C: uint16_t border_width;
  pub pad0: [u8; 2],   // C: uint8_t pad0[2];
}

/// xcb.h 170-196: `typedef struct { uint8_t response_type; uint8_t error_code; uint16_t
/// sequence; uint32_t resource_id; uint16_t minor_code; uint8_t major_code; uint8_t pad0;
/// uint32_t pad[5]; uint32_t full_sequence; } xcb_generic_error_t;` - C's record has no tag, so
/// the Rust record takes the typedef's name (P-67).
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_generic_error_t
{
  pub response_type: u8,   // C: uint8_t response_type;
  pub error_code: u8,   // C: uint8_t error_code;
  pub sequence: u16,   // C: uint16_t sequence;
  pub resource_id: u32,   // C: uint32_t resource_id;
  pub minor_code: u16,   // C: uint16_t minor_code;
  pub major_code: u8,   // C: uint8_t major_code;
  pub pad0: u8,   // C: uint8_t pad0;
  pub pad: [u32; 5],   // C: uint32_t pad[5];
  pub full_sequence: u32,   // C: uint32_t full_sequence;
}

/// xproto.h 302-311: `typedef struct xcb_visualtype_t { xcb_visualid_t visual_id; uint8_t
/// _class; uint8_t bits_per_rgb_value; uint16_t colormap_entries; uint32_t red_mask; uint32_t
/// green_mask; uint32_t blue_mask; uint8_t pad0[4]; } xcb_visualtype_t;` - the pointee of
/// `x_display_info.xcb_visual`, which nothing in the port reads through yet; carried whole
/// because the record is complete in C and its layout is measurable.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_visualtype_t
{
  pub visual_id: xcb_visualid_t,   // C: xcb_visualid_t visual_id;
  pub _class: u8,   // C: uint8_t _class;
  pub bits_per_rgb_value: u8,   // C: uint8_t bits_per_rgb_value;
  pub colormap_entries: u16,   // C: uint16_t colormap_entries;
  pub red_mask: u32,   // C: uint32_t red_mask;
  pub green_mask: u32,   // C: uint32_t green_mask;
  pub blue_mask: u32,   // C: uint32_t blue_mask;
  pub pad0: [u8; 4],   // C: uint8_t pad0[4];
}

/// xproto.h 1553-1557: `typedef enum xcb_map_state_t { XCB_MAP_STATE_UNMAPPED = 0,
/// XCB_MAP_STATE_UNVIEWABLE = 1, XCB_MAP_STATE_VIEWABLE = 2 } xcb_map_state_t;` - the values
/// `xcb_get_window_attributes_reply_t.map_state` holds (the `USE_XCB` arm compares against
/// `XCB_MAP_STATE_UNMAPPED`; C converts the enum into that `uint8_t` field implicitly, so the
/// cast belongs at the use site - the `xm_drag_style` shape).
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
pub enum xcb_map_state_t
{
  XCB_MAP_STATE_UNMAPPED = 0,
  XCB_MAP_STATE_UNVIEWABLE = 1,
  XCB_MAP_STATE_VIEWABLE = 2,
}

/// xproto.h 234-239: `typedef struct xcb_rectangle_t { int16_t x; int16_t y; uint16_t width;
/// uint16_t height; } xcb_rectangle_t;` - a shape rectangle, the same eight bytes as
/// `XRectangle` above.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_rectangle_t
{
  pub x: i16,   // C: int16_t x;
  pub y: i16,   // C: int16_t y;
  pub width: u16,   // C: uint16_t width;
  pub height: u16,   // C: uint16_t height;
}

/// xproto.h 244-248: `typedef struct xcb_rectangle_iterator_t { xcb_rectangle_t *data; int rem;
/// int index; } xcb_rectangle_iterator_t;` - `xcb_shape_get_rectangles_rectangles_iterator`'s
/// return, walked by `xcb_rectangle_next` (both below).
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_rectangle_iterator_t
{
  pub data: *mut xcb_rectangle_t,   // C: xcb_rectangle_t *data;
  pub rem: c_int,   // C: int rem;
  pub index: c_int,   // C: int index;
}

/// shape.h 278-280: `typedef struct xcb_shape_get_rectangles_cookie_t { unsigned int sequence; }
/// xcb_shape_get_rectangles_cookie_t;`.
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_shape_get_rectangles_cookie_t
{
  pub sequence: c_uint,   // C: unsigned int sequence;
}

/// shape.h 300-307: `typedef struct xcb_shape_get_rectangles_reply_t { uint8_t response_type;
/// uint8_t ordering; uint16_t sequence; uint32_t length; uint32_t rectangles_len; uint8_t
/// pad0[20]; } xcb_shape_get_rectangles_reply_t;` - the reply the iterator is built from
/// (`rectangles_len` is the count; the arm reads the iterator's own `rem`).
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xcb_shape_get_rectangles_reply_t
{
  pub response_type: u8,   // C: uint8_t response_type;
  pub ordering: u8,   // C: uint8_t ordering;
  pub sequence: u16,   // C: uint16_t sequence;
  pub length: u32,   // C: uint32_t length;
  pub rectangles_len: u32,   // C: uint32_t rectangles_len;
  pub pad0: [u8; 20],   // C: uint8_t pad0[20];
}

/// shape.h 57-61: `typedef enum xcb_shape_sk_t { XCB_SHAPE_SK_BOUNDING = 0, XCB_SHAPE_SK_CLIP =
/// 1, XCB_SHAPE_SK_INPUT = 2 } xcb_shape_sk_t;` - the two values the shape arm passes as
/// `xcb_shape_kind_t` (the conversion C performs implicitly).
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
pub enum xcb_shape_sk_t
{
  XCB_SHAPE_SK_BOUNDING = 0,
  XCB_SHAPE_SK_CLIP = 1,
  XCB_SHAPE_SK_INPUT = 2,
}

/* The measured ABI of every record above (the C probe in this ledger's Evidence):
   `sizeof`/`alignof` for all of them, and the offsets of the members the `USE_XCB` arm reads.  */

#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::size_of::<xcb_get_property_cookie_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::align_of::<xcb_get_property_cookie_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::size_of::<xcb_get_property_reply_t> () == 32);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::align_of::<xcb_get_property_reply_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_get_property_reply_t, format) == 1);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_get_property_reply_t, r#type) == 8);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_get_property_reply_t, value_len) == 16);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::size_of::<xcb_get_window_attributes_cookie_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::size_of::<xcb_get_window_attributes_reply_t> () == 44);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::align_of::<xcb_get_window_attributes_reply_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_get_window_attributes_reply_t, map_state) == 26);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_get_window_attributes_reply_t, colormap) == 28);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_get_window_attributes_reply_t, your_event_mask) == 36);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::size_of::<xcb_translate_coordinates_cookie_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::size_of::<xcb_translate_coordinates_reply_t> () == 16);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::align_of::<xcb_translate_coordinates_reply_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_translate_coordinates_reply_t, dst_x) == 12);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_translate_coordinates_reply_t, dst_y) == 14);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::size_of::<xcb_get_geometry_cookie_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::size_of::<xcb_get_geometry_reply_t> () == 24);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::align_of::<xcb_get_geometry_reply_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_get_geometry_reply_t, width) == 16);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_get_geometry_reply_t, height) == 18);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_get_geometry_reply_t, border_width) == 20);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::size_of::<xcb_generic_error_t> () == 36);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::align_of::<xcb_generic_error_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_generic_error_t, error_code) == 1);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::size_of::<xcb_visualtype_t> () == 24);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
const _: () = core::assert! (core::mem::align_of::<xcb_visualtype_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
const _: () = core::assert! (core::mem::size_of::<xcb_rectangle_t> () == 8);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
const _: () = core::assert! (core::mem::align_of::<xcb_rectangle_t> () == 2);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_rectangle_t, width) == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
const _: () = core::assert! (core::mem::size_of::<xcb_rectangle_iterator_t> () == 16);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
const _: () = core::assert! (core::mem::align_of::<xcb_rectangle_iterator_t> () == 8);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_rectangle_iterator_t, rem) == 8);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_rectangle_iterator_t, index) == 12);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
const _: () = core::assert! (core::mem::size_of::<xcb_shape_get_rectangles_cookie_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
const _: () = core::assert! (core::mem::size_of::<xcb_shape_get_rectangles_reply_t> () == 32);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
const _: () = core::assert! (core::mem::align_of::<xcb_shape_get_rectangles_reply_t> () == 4);
#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
const _: () = core::assert! (core::mem::offset_of! (xcb_shape_get_rectangles_reply_t, rectangles_len) == 8);

/* The procedures, in C's own signature: every cookie is passed **by value** (that is what the
   records above are for) and every reply pointer is freed by the caller with `free` (C's own
   documentation on `xcb_get_property_reply`).  */

#[cfg (all (feature = "have_x_windows", feature = "use_xcb"))]
unsafe extern "C" {
  /// C: xproto.h 6924-6931: `xcb_get_property_cookie_t xcb_get_property (xcb_connection_t *c,
  /// uint8_t _delete, xcb_window_t window, xcb_atom_t property, xcb_atom_t type, uint32_t
  /// long_offset, uint32_t long_length);`
  pub fn xcb_get_property (c: *mut xcb_connection_t, _delete: u8, window: xcb_window_t,
                           property: xcb_atom_t, r#type: xcb_atom_t, long_offset: u32,
                           long_length: u32) -> xcb_get_property_cookie_t;
  /// C: xproto.h 6996-6998: `xcb_get_property_reply_t *xcb_get_property_reply
  /// (xcb_connection_t *c, xcb_get_property_cookie_t cookie, xcb_generic_error_t **e);`
  pub fn xcb_get_property_reply (c: *mut xcb_connection_t, cookie: xcb_get_property_cookie_t,
                                 e: *mut *mut xcb_generic_error_t) -> *mut xcb_get_property_reply_t;
  /// C: xproto.h 6972-6973: `void *xcb_get_property_value (const xcb_get_property_reply_t *R);`
  pub fn xcb_get_property_value (r: *const xcb_get_property_reply_t) -> *mut c_void;
  /// C: xproto.h 6975-6976: `int xcb_get_property_value_length (const
  /// xcb_get_property_reply_t *R);`
  pub fn xcb_get_property_value_length (r: *const xcb_get_property_reply_t) -> c_int;
  /// C: xproto.h 6030-6032: `xcb_get_window_attributes_cookie_t xcb_get_window_attributes
  /// (xcb_connection_t *c, xcb_window_t window);`
  pub fn xcb_get_window_attributes (c: *mut xcb_connection_t, window: xcb_window_t)
                                    -> xcb_get_window_attributes_cookie_t;
  /// C: xproto.h 6065-6068: `xcb_get_window_attributes_reply_t *xcb_get_window_attributes_reply
  /// (xcb_connection_t *c, xcb_get_window_attributes_cookie_t cookie, xcb_generic_error_t **e);`
  pub fn xcb_get_window_attributes_reply (c: *mut xcb_connection_t,
                                          cookie: xcb_get_window_attributes_cookie_t,
                                          e: *mut *mut xcb_generic_error_t)
                                          -> *mut xcb_get_window_attributes_reply_t;
  /// C: xproto.h 8220-8225: `xcb_translate_coordinates_cookie_t xcb_translate_coordinates
  /// (xcb_connection_t *c, xcb_window_t src_window, xcb_window_t dst_window, int16_t src_x,
  /// int16_t src_y);`
  pub fn xcb_translate_coordinates (c: *mut xcb_connection_t, src_window: xcb_window_t,
                                    dst_window: xcb_window_t, src_x: i16, src_y: i16)
                                    -> xcb_translate_coordinates_cookie_t;
  /// C: xproto.h 8259-8262: `xcb_translate_coordinates_reply_t *xcb_translate_coordinates_reply
  /// (xcb_connection_t *c, xcb_translate_coordinates_cookie_t cookie, xcb_generic_error_t **e);`
  pub fn xcb_translate_coordinates_reply (c: *mut xcb_connection_t,
                                          cookie: xcb_translate_coordinates_cookie_t,
                                          e: *mut *mut xcb_generic_error_t)
                                          -> *mut xcb_translate_coordinates_reply_t;
  /// C: xproto.h 6551-6553: `xcb_get_geometry_cookie_t xcb_get_geometry (xcb_connection_t *c,
  /// xcb_drawable_t drawable);`
  pub fn xcb_get_geometry (c: *mut xcb_connection_t, drawable: xcb_drawable_t)
                           -> xcb_get_geometry_cookie_t;
  /// C: xproto.h 6586-6589: `xcb_get_geometry_reply_t *xcb_get_geometry_reply
  /// (xcb_connection_t *c, xcb_get_geometry_cookie_t cookie, xcb_generic_error_t **e);`
  pub fn xcb_get_geometry_reply (c: *mut xcb_connection_t, cookie: xcb_get_geometry_cookie_t,
                                 e: *mut *mut xcb_generic_error_t) -> *mut xcb_get_geometry_reply_t;
}

#[cfg (all (feature = "have_x_windows", feature = "use_xcb", feature = "have_xcb_shape"))]
unsafe extern "C" {
  /// C: xproto.h 5314-5315: `void xcb_rectangle_next (xcb_rectangle_iterator_t *i);` - C's
  /// iterator advances *in place*, which is how the shape arm's `while` walks the reply.
  pub fn xcb_rectangle_next (i: *mut xcb_rectangle_iterator_t);
  /// C: shape.h 694-697: `xcb_shape_get_rectangles_cookie_t xcb_shape_get_rectangles
  /// (xcb_connection_t *c, xcb_window_t window, xcb_shape_kind_t source_kind);`
  pub fn xcb_shape_get_rectangles (c: *mut xcb_connection_t, window: xcb_window_t,
                                   source_kind: xcb_shape_kind_t)
                                   -> xcb_shape_get_rectangles_cookie_t;
  /// C: shape.h 738-741: `xcb_shape_get_rectangles_reply_t *xcb_shape_get_rectangles_reply
  /// (xcb_connection_t *c, xcb_shape_get_rectangles_cookie_t cookie, xcb_generic_error_t **e);`
  pub fn xcb_shape_get_rectangles_reply (c: *mut xcb_connection_t,
                                         cookie: xcb_shape_get_rectangles_cookie_t,
                                         e: *mut *mut xcb_generic_error_t)
                                         -> *mut xcb_shape_get_rectangles_reply_t;
  /// C: shape.h 721-722: `xcb_rectangle_iterator_t
  /// xcb_shape_get_rectangles_rectangles_iterator (const xcb_shape_get_rectangles_reply_t *R);`
  pub fn xcb_shape_get_rectangles_rectangles_iterator
    (r: *const xcb_shape_get_rectangles_reply_t) -> xcb_rectangle_iterator_t;
}

// ----------------------------------------------------------------------
// The Xlib surface the port's X arms name (2026-10-05).

/* **Why this section is a module.**  A ported arm declares some of these names itself -
   `src/image.rs` 232's `pub type Pixmap = c_ulong;` is the measured one, and the x+cairo
   probe reports `E0428` against a crate-root declaration here.  A glob import is
   *shadowed* by a crate-root item (PORTING.md 0.1-10 / P-21), so the section sits in a
   module this file glob-imports: the unit's own spelling stays the live one and every
   name no arm declares (`StaticColor`, `ZPixmap`, `BitmapSuccess`, `XGetPixel`, ...)
   resolves through the glob at the crate root.  The `X11/Xlib.h` surface the file's own
   top level already carried (`XRectangle`, `XPointer`, `Bool`, the Xrm types and the Xrm
   entry points) stays where it was; this is the same surface, extended.  */
pub mod xlib_surface {
  use super::*;

// ----------------------------------------------------------------------
// The Xlib surface the port's X arms name (2026-10-05).

/* `<X11/Xlib.h>` 66-483's scalar typedefs and the records behind them, plus the ten procedures
   `docs/ports/image.md`'s x+cairo configuration sweep read as `E0425` (its subject list:
   `Atom` 124, `Cursor` 19, `Window` 13, `GC` 7 - all of them member types of
   `include/xterm.rs`'s `x_display_info`/`x_output`, which is why one missing typedef produces
   dozens of errors).  Each layout below is **measured** on this machine's headers
   (`/opt/homebrew/include/X11`, `libx11` 1.8.13 + `xorgproto` 2025.1) with

     clang -I/opt/homebrew/include -o probe probe.c && ./probe    # sizeof/alignof/offsetof

   and asserted where it is declared, the `XRectangle`/`XrmValue` practice at the top of this
   file.  `Display`, `GC` and `XEvent` are the three C itself leaves opaque or does not read
   here; each says so where it is declared.

   What is deliberately **not** here: `xcb`'s section above is the only other one, and the
   request/host/font entry points this tree's arms do not call are absent - this mirror grows by
   the names a ported arm reads (the file note's rule).  `XEvent` is the one gap this round
   leaves: 33 member records, none of which any ported arm reads a field of (its one use is
   `include/xterm.rs` 789's `*mut XEvent`), so the type keeps C's measured size and the members
   are owed (see the ledger).  */

/// X11/X.h 66: `typedef unsigned long XID;` - its consumer is `Window` below, which the pgtk
/// arm needs too, so the gate is the union (status 3.496).
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
pub type XID = c_ulong;
/// X11/X.h 70: `typedef unsigned long Mask;`
#[cfg (feature = "have_x_windows")]
pub type Mask = c_ulong;
/// X11/X.h 74: `typedef unsigned long Atom;` - "Also in Xdefs.h".  Xlib.h 100's
/// `typedef XID Atom;` is the same width.
#[cfg (feature = "have_x_windows")]
pub type Atom = XID;
/// X11/X.h 76: `typedef unsigned long VisualID;`
#[cfg (feature = "have_x_windows")]
pub type VisualID = XID;
/// X11/X.h 96: `typedef XID Window;`
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
pub type Window = XID;
/// X11/X.h 97: `typedef XID Drawable;`
#[cfg (feature = "have_x_windows")]
pub type Drawable = XID;
/// X11/X.h 100: `typedef XID Font;`
#[cfg (feature = "have_x_windows")]
pub type Font = XID;
/// X11/X.h 102: `typedef XID Pixmap;`
#[cfg (feature = "have_x_windows")]
pub type Pixmap = XID;
/// X11/X.h 103: `typedef XID Cursor;`
#[cfg (feature = "have_x_windows")]
pub type Cursor = XID;
/// X11/X.h 104: `typedef XID Colormap;`
#[cfg (feature = "have_x_windows")]
pub type Colormap = XID;
/// X11/X.h 105: `typedef XID GContext;`
#[cfg (feature = "have_x_windows")]
pub type GContext = XID;
/// X11/X.h 106: `typedef XID KeySym;`
#[cfg (feature = "have_x_windows")]
pub type KeySym = XID;
/// X11/X.h 108: `typedef unsigned char KeyCode;` - measured `sizeof (KeyCode) == 1`.
#[cfg (feature = "have_x_windows")]
pub type KeyCode = c_uchar;

/// Xlib.h 479-483: "The contents of this structure are implementation dependent.  A Display
/// should be treated as opaque by application code."  `typedef struct _XDisplay Display;` is
/// Xlib.h 483; the body (Xlib.h 485-…) is behind `#ifdef XLIB_ILLEGAL_ACCESS`, so the port
/// declares the tag and the alias and never its members - C's own opacity.
#[cfg (feature = "have_x_windows")]
pub struct _XDisplay { _private: [u8; 0] }
#[cfg (feature = "have_x_windows")]
pub type Display = _XDisplay;

/// Xlib.h 210-222: "Graphics context.  … A GC should be treated as opaque by application
/// code."  `typedef struct _XGC … *GC;` - a pointer to a record whose body is behind
/// `XLIB_ILLEGAL_ACCESS`, so the port declares the tag and the pointer alias.  Measured
/// `sizeof (GC) == 8`.
#[cfg (feature = "have_x_windows")]
pub struct _XGC { _private: [u8; 0] }
#[cfg (feature = "have_x_windows")]
pub type GC = *mut _XGC;

/// Xlib.h 144-151: `typedef struct _XExtData { … } XExtData;` - the extension hook every other
/// record starts with.  Measured `[sizeof=32, align=8]`.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _XExtData
{
  pub number: c_int,                                        // Xlib.h 146
  pub next: *mut _XExtData,                                 // Xlib.h 147
  /// Xlib.h 148-150: `int (*free_private)(struct _XExtData *extension);`
  pub free_private: Option<unsafe extern "C" fn (*mut _XExtData) -> c_int>,
  pub private_data: XPointer,                               // Xlib.h 151
}
#[cfg (feature = "have_x_windows")]
pub type XExtData = _XExtData;
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XExtData> () == 32);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XExtData> () == 8);

/// Xlib.h 223-234: `typedef struct { … } Visual;`.  Measured `[sizeof=56, align=8]`.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Visual
{
  pub ext_data: *mut XExtData,                              // Xlib.h 225
  pub visualid: VisualID,                                   // Xlib.h 226
  /// Xlib.h 229: `int class;` - "class of screen (monochrome, etc.)" (`c_class` under C++).
  pub r#class: c_int,
  pub red_mask: c_ulong,                                    // Xlib.h 231
  pub green_mask: c_ulong,
  pub blue_mask: c_ulong,
  pub bits_per_rgb: c_int,                                  // Xlib.h 232
  pub map_entries: c_int,                                   // Xlib.h 233
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<Visual> () == 56);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<Visual> () == 8);

/// Xlib.h 239-252: `typedef struct { int depth; int nvisuals; Visual *visuals; } Depth;`.
/// Measured `[sizeof=16, align=8]`.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Depth
{
  pub depth: c_int,                                         // Xlib.h 249
  pub nvisuals: c_int,                                      // Xlib.h 250
  pub visuals: *mut Visual,                                 // Xlib.h 251
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<Depth> () == 16);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<Depth> () == 8);

/// Xlib.h 253-271: `typedef struct { … } Screen;` - "Information about the screen.  The contents
/// of this structure are implementation dependent."  Measured `[sizeof=128, align=8]`, with the
/// four quoted offsets.  Only pointers to it are used by this tree (`xterm.rs` 375's
/// `*mut Screen`), but the record is C's public one, so it is carried whole.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Screen
{
  pub ext_data: *mut XExtData,                              // Xlib.h 256
  pub display: *mut _XDisplay,                              // Xlib.h 257
  pub root: Window,                                         // Xlib.h 258   (offset 16)
  pub width: c_int,                                         // Xlib.h 259
  pub height: c_int,
  pub mwidth: c_int,                                        // Xlib.h 260
  pub mheight: c_int,
  pub ndepths: c_int,                                       // Xlib.h 261
  pub depths: *mut Depth,                                   // Xlib.h 262
  pub root_depth: c_int,                                    // Xlib.h 263
  pub root_visual: *mut Visual,                             // Xlib.h 264
  pub default_gc: GC,                                       // Xlib.h 265   (offset 72)
  pub cmap: Colormap,                                       // Xlib.h 266   (offset 80)
  pub white_pixel: c_ulong,                                 // Xlib.h 267
  pub black_pixel: c_ulong,
  pub max_maps: c_int,                                      // Xlib.h 268
  pub min_maps: c_int,
  pub backing_store: c_int,                                 // Xlib.h 269
  pub save_unders: Bool,                                    // Xlib.h 270
  pub root_input_mask: c_long,                              // Xlib.h 271  (offset 120)
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<Screen> () == 128);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<Screen> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (Screen, root) == 16);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (Screen, default_gc) == 72);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (Screen, cmap) == 80);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (Screen, root_input_mask) == 120);

/// Xlib.h 356-390: `typedef struct _XImage { … } XImage;`.  Measured `[sizeof=136, align=8]`;
/// the sixteen scalar members up to `obdata` sit at 0/4/8/12/16/24/28/32/36/40/44/48/56/64/72/80
/// and the nested `struct funcs f` at 88.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _XImage_funcs
{
  /// Xlib.h 371-381: `struct _XImage *(*create_image)(struct _XDisplay *, Visual *,
  /// unsigned int, int, int, char *, unsigned int, unsigned int, int, int);`
  pub create_image: Option<unsafe extern "C" fn (*mut _XDisplay, *mut Visual, c_uint, c_int,
                                                 c_int, *mut c_char, c_uint, c_uint, c_int,
                                                 c_int) -> *mut XImage>,
  pub destroy_image: Option<unsafe extern "C" fn (*mut XImage) -> c_int>,          // Xlib.h 382
  pub get_pixel: Option<unsafe extern "C" fn (*mut XImage, c_int, c_int) -> c_ulong>, // 383
  pub put_pixel: Option<unsafe extern "C" fn (*mut XImage, c_int, c_int, c_ulong) -> c_int>, // 384
  pub sub_image: Option<unsafe extern "C" fn (*mut XImage, c_int, c_int, c_uint, c_uint)
                                             -> *mut XImage>,                      // Xlib.h 385
  pub add_pixel: Option<unsafe extern "C" fn (*mut XImage, c_long) -> c_int>,      // Xlib.h 386
}
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _XImage
{
  pub width: c_int,                                         // Xlib.h 357
  pub height: c_int,
  pub xoffset: c_int,                                       // Xlib.h 358
  pub format: c_int,                                        // Xlib.h 359
  pub data: *mut c_char,                                    // Xlib.h 360
  pub byte_order: c_int,                                    // Xlib.h 361
  pub bitmap_unit: c_int,                                   // Xlib.h 362
  pub bitmap_bit_order: c_int,                              // Xlib.h 363
  pub bitmap_pad: c_int,                                    // Xlib.h 364
  pub depth: c_int,                                         // Xlib.h 365
  pub bytes_per_line: c_int,                                // Xlib.h 366
  pub bits_per_pixel: c_int,                                // Xlib.h 367
  pub red_mask: c_ulong,                                    // Xlib.h 368
  pub green_mask: c_ulong,                                  // Xlib.h 369
  pub blue_mask: c_ulong,                                   // Xlib.h 370
  pub obdata: XPointer,                                     // Xlib.h 371
  pub f: _XImage_funcs,                                     // Xlib.h 372-388 (offset 88)
}
#[cfg (feature = "have_x_windows")]
pub type XImage = _XImage;
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XImage> () == 136);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XImage> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XImage, data) == 16);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XImage, red_mask) == 56);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XImage, obdata) == 80);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XImage, f) == 88);

/// Xlib.h 404-411: `typedef struct { … } XColor;`.  Measured `[sizeof=16, align=8]` with
/// `pixel` 0, `red` 8, `green` 10, `blue` 12, `flags` 14, `pad` 15.  `include/xterm.rs` 218
/// holds one by value (`color_name_cache_entry.rgb`).
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XColor
{
  pub pixel: c_ulong,                                       // Xlib.h 407
  pub red: c_ushort,                                        // Xlib.h 408
  pub green: c_ushort,
  pub blue: c_ushort,
  pub flags: c_char,                                        // Xlib.h 409 ("do_red, do_green, …")
  pub pad: c_char,                                          // Xlib.h 410
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XColor> () == 16);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XColor> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XColor, red) == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XColor, flags) == 14);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XColor, pad) == 15);

/// Xlib.h 469-474: `typedef struct { int max_keypermod; KeyCode *modifiermap; }
/// XModifierKeymap;`.  Measured `[sizeof=16, align=8]`; `include/xterm.rs` 609 holds a pointer
/// to it.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XModifierKeymap
{
  pub max_keypermod: c_int,                                 // Xlib.h 472
  pub modifiermap: *mut KeyCode,                            // Xlib.h 473
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XModifierKeymap> () == 16);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XModifierKeymap> () == 8);

/// Xutil.h 119-130: `typedef struct { … } XWMHints;`.  Measured `[sizeof=56, align=8]` with
/// `flags` 0, `input` 8, `initial_state` 12, `icon_pixmap` 16, `icon_window` 24, `icon_x` 32,
/// `icon_y` 36, `icon_mask` 40, `window_group` 48.  `include/xterm.rs` 787 holds one by value.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XWMHints
{
  pub flags: c_long,                                        // Xutil.h 121
  pub input: Bool,                                          // Xutil.h 122
  pub initial_state: c_int,                                 // Xutil.h 124
  pub icon_pixmap: Pixmap,                                  // Xutil.h 125
  pub icon_window: Window,                                  // Xutil.h 126
  pub icon_x: c_int,                                        // Xutil.h 127
  pub icon_y: c_int,
  pub icon_mask: Pixmap,                                    // Xutil.h 128
  pub window_group: XID,                                    // Xutil.h 129
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XWMHints> () == 56);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XWMHints> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XWMHints, input) == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XWMHints, icon_window) == 24);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XWMHints, window_group) == 48);

/// Xutil.h 287-302: `typedef struct { … } XVisualInfo;`.  Measured `[sizeof=64, align=8]` with
/// `visual` 0, `visualid` 8, `screen` 16, `depth` 20, `class` 24, `red_mask` 32, `green_mask`
/// 40, `blue_mask` 48, `colormap_size` 56, `bits_per_rgb` 60.  `include/xterm.rs` 379 holds one
/// by value.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XVisualInfo
{
  pub visual: *mut Visual,                                  // Xutil.h 289
  pub visualid: VisualID,                                   // Xutil.h 290
  pub screen: c_int,                                        // Xutil.h 291
  pub depth: c_int,                                         // Xutil.h 292
  pub r#class: c_int,                                       // Xutil.h 296 (`c_class` under C++)
  pub red_mask: c_ulong,                                    // Xutil.h 298
  pub green_mask: c_ulong,
  pub blue_mask: c_ulong,
  pub colormap_size: c_int,                                 // Xutil.h 300
  pub bits_per_rgb: c_int,                                  // Xutil.h 301
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XVisualInfo> () == 64);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XVisualInfo> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XVisualInfo, screen) == 16);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XVisualInfo, r#class) == 24);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XVisualInfo, red_mask) == 32);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XVisualInfo, bits_per_rgb) == 60);

/// Xlib.h 969-1005: `typedef union _XEvent { … } XEvent;` - the measured size is 192 bytes with
/// align 8 (the `long pad[24]` member).  **The 33 member records are not transcribed this
/// round**: no ported arm reads a member of it (its one use is `include/xterm.rs` 789's
/// `*mut XEvent`), and the union's own members are 33 nested records of their own.  The type
/// keeps C's measured shape so a pointer to it is the right size; the members are owed and
/// recorded in the ledger `docs/ports/include-xlib.md`.
#[cfg (feature = "have_x_windows")]
#[repr(C, align(8))]
pub struct XEvent { _private: [u8; 192] }
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XEvent> () == 192);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::align_of::<XEvent> () == 8);

/* Xlib.h's ten procedures the X arms call.  Each keeps C's prototype in its `/// C:` line; the
   three macros among them (`XGetPixel`, `XPutPixel`, `XDestroyImage`) are *macros* in C, not
   entry points, and are `macro_rules!` here with C's own bodies - declaring them as functions
   would name symbols libX11 does not export.  */

#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: Xlib.h 1600-1606: `extern Pixmap XCreateBitmapFromData (Display *, Drawable,
  /// _Xconst char *, unsigned int, unsigned int);`
  pub fn XCreateBitmapFromData (display: *mut Display, d: Drawable, data: *const c_char,
                                width: c_uint, height: c_uint) -> Pixmap;
  /// C: Xlib.h 3041-3050: `extern int XReadBitmapFile (Display *, Drawable, _Xconst char *,
  /// unsigned int *, unsigned int *, Pixmap *, int *, int *);`
  pub fn XReadBitmapFile (display: *mut Display, d: Drawable, filename: *const c_char,
                          width_return: *mut c_uint, height_return: *mut c_uint,
                          bitmap_return: *mut Pixmap, x_hot_return: *mut c_int,
                          y_hot_return: *mut c_int) -> c_int;
  /// C: Xlib.h 2580-2583: `extern int XFreePixmap (Display *, Pixmap);`
  pub fn XFreePixmap (display: *mut Display, pixmap: Pixmap) -> c_int;
  /// C: Xlib.h 1452-1461: `extern XImage *XGetImage (Display *, Drawable, int, int,
  /// unsigned int, unsigned int, unsigned long, int);`
  pub fn XGetImage (display: *mut Display, d: Drawable, x: c_int, y: c_int, width: c_uint,
                    height: c_uint, plane_mask: c_ulong, format: c_int) -> *mut XImage;
  /// C: Xlib.h 2913-2923: `extern int XPutImage (Display *, Drawable, GC, XImage *, int, int,
  /// int, int, unsigned int, unsigned int);`
  pub fn XPutImage (display: *mut Display, d: Drawable, gc: GC, image: *mut XImage,
                    src_x: c_int, src_y: c_int, dest_x: c_int, dest_y: c_int,
                    width: c_uint, height: c_uint) -> c_int;
  /// C: Xlib.h 1580-1585: `extern GC XCreateGC (Display *, Drawable, unsigned long,
  /// XGCValues *);` - `XGCValues` (Xlib.h 177-203) is a record no ported arm reads, so the
  /// fourth parameter is the untyped pointer its call sites pass (P-72 rule 3).
  pub fn XCreateGC (display: *mut Display, d: Drawable, valuemask: c_ulong,
                    values: *mut c_void) -> GC;
  /// C: Xlib.h 2571-2574: `extern int XFreeGC (Display *, GC);`
  pub fn XFreeGC (display: *mut Display, gc: GC) -> c_int;
}

/// C: Xlib.h 4170's macro form: `#define XDestroyImage(ximg) ((ximg)->f.destroy_image (ximg))`
/// - three of the "ten procedures" are **macros in C, not entry points** (libX11 exports no such
/// symbols), so they are the port's inline functions with C's own bodies.  A `macro_rules!` was
/// the first spelling and the x+cairo probe rejected it: a ported arm calls them with C's call
/// shape (`XGetPixel (ximg, x, y)`, no `!`), which is a *function* call in Rust
/// (`E0423: cannot find function … a macro named `XGetPixel` exists in another namespace`).
/// `f`'s members are `Option`s (PORTING.md P-27: a C function pointer is nullable) and C's body
/// calls the pointer, so the body is `unwrap_unchecked`.
#[cfg (feature = "have_x_windows")]
#[inline(always)]
pub unsafe fn XDestroyImage (ximg: *mut XImage) -> c_int
{ ((*ximg).f.destroy_image.unwrap_unchecked ()) (ximg) }
/// C: Xlib.h 4171's macro form: `#define XGetPixel(ximage, x, y) ((*(ximage)->f.get_pixel)
/// ((ximage), (x), (y)))`.
#[cfg (feature = "have_x_windows")]
#[inline(always)]
pub unsafe fn XGetPixel (ximage: *mut XImage, x: c_int, y: c_int) -> c_ulong
{ ((*ximage).f.get_pixel.unwrap_unchecked ()) (ximage, x, y) }
/// C: Xlib.h 4172's macro form: `#define XPutPixel(ximage, x, y, pixel)
/// ((*(ximage)->f.put_pixel)((ximage), (x), (y), (pixel)))`.
#[cfg (feature = "have_x_windows")]
#[inline(always)]
pub unsafe fn XPutPixel (ximage: *mut XImage, x: c_int, y: c_int, pixel: c_ulong) -> c_int
{ ((*ximage).f.put_pixel.unwrap_unchecked ()) (ximage, x, y, pixel) }
}
use xlib_surface::*;
