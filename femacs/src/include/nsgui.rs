// femacs: port of src/nsgui.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): declarations only.  `nsgui.h` is the NeXT/Open/GNUstep
// window system's `gui.h`-shaped header: the X-compatibility names a plain (non-Objective-C)
// translation unit in this backend needs - `XCharStruct`, `Emacs_Pixmap`, `Emacs_Cursor`,
// `Window`, `CGFloat`, `NSPoint`/`NSSize`/`NSRect`, `NativeRectangle`, the rectangle
// conversion macros and the gravity/value/position constants.  Its includers are
// `src/dispextern.h` 142-148 (behind `HAVE_NS`, which is why `Emacs_Cursor` and
// `NativeRectangle` have to exist twice in C - dispextern.h 175-177 and 178-180 are the
// `#ifndef` fallbacks that both platforms define over) and `src/macfont.m` 34.  The frozen
// configuration has no `HAVE_NS` and no
// `NS_IMPL_COCOA` (`docs/baseline/config/darwin-arm64.macros.tsv`), so no translation unit of
// this target includes the header; the header's own text is (almost) unconditional, so the
// mirror carries it - the `src/w32select.h` reading, status 3.158.
//
// Dead text under the frozen configuration (PORTING.md P-08; `tools/check-cond-eval.py
// src/nsgui.h` -> `10 conditions, 0 unknown to the evaluator, 0 disagreements`):
//   * C 23-43, the `#ifdef __OBJC__` head: the `Cursor` -> `FooFoo` rename (C 29), its
//     `#undef` (C 35) and the three framework imports (`<AppKit/AppKit.h>` C 32,
//     `<Foundation/NSDistantObject.h>` C 37, `<AvailabilityMacros.h>` C 40).  `__OBJC__` is
//     undefined because this configuration compiles C.
//   * C 56 (`typedef id Emacs_Pixmap;`) and C 62 (`typedef NSCursor *Emacs_Cursor;`), the
//     Objective-C halves of the `#ifdef __OBJC__`/`#else` pairs at C 55-59 and C 61-65; the
//     live halves are C 58 and C 64.
//   * C 72-73 (`typedef float CGFloat;`), the `#else` of C 70's
//     `#if defined (__LP64__) && __LP64__`; the live arm is C 71's `double`.
// C 19-20 is the include guard; C 137 is the guard's `#endif`.
//
// Nothing in the live text is degraded (PORTING.md P-55/P-72 do not apply): every pointer is
// `void *`, which C itself leaves untyped.  `NSPoint`/`NSSize`/`NSRect` are Objective-C
// runtime types (`struct _NSPoint { CGFloat x, y; }` etc.) that C re-declares in this
// non-Objective-C arm, so the mirror declares C's tags and typedef names rather than taking
// them from a framework header, exactly as C does.

// C 19-20: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const __NSGUI_H__: bool = true;

/* C 46-53: `typedef struct _XCharStruct { … } XCharStruct;`.  Declared as C's own tag plus
   the typedef alias, PORTING.md 1.3's shape for a tagged struct that also has a typedef name
   (the `haikugui.rs` `haiku_char_struct`/`XCharStruct` pair is the same construction).  */
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

// C 58: the non-Objective-C arm of C 55-59.
pub type Emacs_Pixmap = *mut c_void;

// C 64: the non-Objective-C arm of C 61-65.
pub type Emacs_Cursor = *mut c_void;

// C 67.
pub type Window = c_int;

// C 71: the `__LP64__` arm of C 69-74.  C's implicit type here is `double`, and
// PORTING.md 1.1 maps `double` to `f64`.
pub type CGFloat = f64;

// C 75-78: `typedef struct _NSPoint { CGFloat x, y; } NSPoint;` and its two siblings.  The
// tags are C's own (`_NSPoint`, `_NSSize`, `_NSRect`) and the typedef names alias them, as in
// the `_XCharStruct` pair above; `NSRect` embeds the other two by value, which is C's layout.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _NSPoint
{
  pub x: CGFloat,
  pub y: CGFloat,
}
pub type NSPoint = _NSPoint;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct _NSSize
{
  pub width: CGFloat,
  pub height: CGFloat,
}
pub type NSSize = _NSSize;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct _NSRect
{
  pub origin: NSPoint,
  pub size: NSSize,
}
pub type NSRect = _NSRect;

// C 80: a macro because C uses it in type position (`NativeRectangle *nr`).
macro_rules! NativeRectangle {
  () => { NSRect };
}

/* C 82-98: C's bodies are comma expressions; Rust has no comma operator, so each macro keeps
   the four assignments as a block (the `haikugui.rs`/`dispextern.rs` shape).  The macros are
   used as statements; the value of C's comma expression is the last assignment, which no
   caller reads.  */
macro_rules! CONVERT_TO_EMACS_RECT {
  ($xr:expr, $nr:expr) =>
    { {
        ($xr).x = ($nr).origin.x as _;
        ($xr).y = ($nr).origin.y as _;
        ($xr).width = ($nr).size.width as _;
        ($xr).height = ($nr).size.height as _;
      } };
}
macro_rules! CONVERT_FROM_EMACS_RECT {
  ($xr:expr, $nr:expr) =>
    { {
        ($nr).origin.x = ($xr).x as _;
        ($nr).origin.y = ($xr).y as _;
        ($nr).size.width = ($xr).width as _;
        ($nr).size.height = ($xr).height as _;
      } };
}
macro_rules! STORE_NATIVE_RECT {
  ($nr:expr, $px:expr, $py:expr, $pwidth:expr, $pheight:expr) =>
    { {
        ($nr).origin.x = ($px) as CGFloat;
        ($nr).origin.y = ($py) as CGFloat;
        ($nr).size.width = ($pwidth) as CGFloat;
        ($nr).size.height = ($pheight) as CGFloat;
      } };
}

/* C 103-114: the ten `*Gravity` constants `frame.c` needs.  C's implicit type is `int`.  */
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

// C 116-123: the value-mask constants, spelled as C's hex literals.
pub const NoValue: c_int = 0x0000;
pub const XValue: c_int = 0x0001;
pub const YValue: c_int = 0x0002;
pub const WidthValue: c_int = 0x0004;
pub const HeightValue: c_int = 0x0008;
pub const AllValues: c_int = 0x000F;
pub const XNegative: c_int = 0x0010;
pub const YNegative: c_int = 0x0020;

// C 125-135: C writes `(1L << n)`, so these are `c_long`, not `c_int` (PORTING.md 1.1).
pub const USPosition: c_long = 1 << 0;        /* user specified x, y */
pub const USSize: c_long = 1 << 1;            /* user specified width, height */
pub const PPosition: c_long = 1 << 2;         /* program specified position */
pub const PSize: c_long = 1 << 3;             /* program specified size */
pub const PMinSize: c_long = 1 << 4;          /* program specified minimum size */
pub const PMaxSize: c_long = 1 << 5;          /* program specified maximum size */
pub const PResizeInc: c_long = 1 << 6;        /* program specified resize increments */
pub const PAspect: c_long = 1 << 7;           /* program specified min, max aspect ratios */
pub const PBaseSize: c_long = 1 << 8;         /* program specified base for incrementing */
pub const PWinGravity: c_long = 1 << 9;       /* program specified window gravity */
