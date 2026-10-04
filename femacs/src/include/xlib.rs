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
// would need a second first line and a second ledger for eight typedefs.
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
const _: () = assert! (core::mem::size_of::<XRectangle> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = assert! (core::mem::align_of::<XRectangle> () == 2);

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
const _: () = assert! (core::mem::size_of::<XrmValue> () == 16);
#[cfg (feature = "have_x_windows")]
const _: () = assert! (core::mem::align_of::<XrmValue> () == 8);
#[cfg (feature = "have_x_windows")]
const _: () = assert! (core::mem::offset_of! (XrmValue, size) == 0);
#[cfg (feature = "have_x_windows")]
const _: () = assert! (core::mem::offset_of! (XrmValue, addr) == 8);

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
