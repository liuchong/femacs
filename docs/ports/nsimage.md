# emacs/src/nsimage.m (658 lines)

> **This file is not compiled by the frozen configuration** (`docs/baseline/frozen-build-sources.txt`,
> `docs/status.md` 3.31): the frozen target is darwin-arm64 with no window system, so `HAVE_NS`
> and `NS_IMPL_COCOA` are undefined and src/nsimage.m belongs to the 77 src files `make -n`
> never reaches.  Batch F ports the Objective-C front end to Swift anyway
> (`docs/port-map.md`'s Swift table, `src/*.m` -> `platform/macos/*.swift`), so this
> ledger is the checklist of the **Cocoa/AppKit build** that the file's `NS_IMPL_COCOA` arms
> describe, and the unit is `platform/macos/nsimage.swift`.  Line numbers are the raw
> reference lines.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 15
- [x] ns_can_use_native_image_api  (line 60)
- [x] ns_image_from_XBM  (line 127)
- [x] ns_image_for_XPM  (line 137)
- [x] ns_image_from_file  (line 145)
- [x] ns_load_image  (line 152)
- [x] ns_image_width  (line 208)
- [x] ns_image_height  (line 214)
- [x] ns_image_set_size  (line 220)
- [x] ns_image_set_transform  (line 226)
- [x] ns_image_set_smoothing  (line 232)
- [x] ns_get_pixel  (line 238)
- [x] ns_put_pixel  (line 244)
- [x] ns_set_alpha  (line 254)
- [x] ns_image_reset  (line 260)
- [x] ns_image_size_in_bytes  (line 267)

## Objective-C classes — 1
- [x] EmacsImage  (`@implementation` line 278, `@end` line 658; declared src/nsterm.h
      670-703) - Swift `class EmacsImage: NSImage`.  Its ivars are declared in src/nsterm.h:
  - [x] `NSBitmapImageRep *bmRep;`  (nsterm.h 672) -> `fileprivate var bmRep:
        NSBitmapImageRep?`
  - [x] `unsigned char *pixmapData[5];`  (nsterm.h 673) -> `fileprivate var pixmapData:
        [UnsafeMutablePointer<UInt8>?]` (five slots; Swift has no fixed-size array)
  - [x] `CGImageRef stippleMask;`  (nsterm.h 675, the `NS_IMPL_COCOA` arm) ->
        `fileprivate var cachedStippleMask: CGImage?` (renamed: the C ivar and the
        `stippleMask` method share a name, which Swift rejects; see Decisions)
  - [x] `NSAffineTransform *transform;`  (`@public`, nsterm.h 679) -> `var transform:
        NSAffineTransform?` (module-visible: src/nsterm.m 4342 reads it)
  - [x] `BOOL smoothing;`  (`@public`, nsterm.h 680) -> `var smoothing: Bool` (src/nsterm.m
        4349/4359)

## Methods — 17
- [x] `+ (instancetype)allocInitFromFile: (Lisp_Object)file`  (line 280) ->
      `class func allocInitFromFile(_ file: Lisp_Object) -> EmacsImage?`
- [x] `- (void)dealloc`  (line 310) - not transcribed: every statement it has is a manual
      release, which ARC performs; see Decisions
- [x] `- (id)copyWithZone: (NSZone *)zone`  (line 323) -> `override func copy(with zone:
      NSZone?) -> Any`
- [x] `- (instancetype)initFromXBM: (unsigned char *)bits width: (int)w height: (int)h
      fg: (unsigned long)fg bg: (unsigned long)bg reverseBytes: (BOOL)reverse`  (line 341) ->
      `convenience init(fromXBM:width:height:fg:bg:reverseBytes:)`
- [x] `- (instancetype)initForXPMWithDepth: (int)depth width: (int)width height: (int)height`
      (line 421) -> `convenience init(forXPMWithDepth:width:height:)`
- [x] `- (void)setPixmapData`  (line 445) -> `func setPixmapData()`
- [x] `- (unsigned long)getPixelAtX: (int)x Y: (int)y`  (line 471) -> `func getPixelAtX(_:Y:)
      -> UInt`
- [x] `- (void)setPixelAtX: (int)x Y: (int)y toRed: (unsigned char)r green: (unsigned char)g
      blue: (unsigned char)b alpha:(unsigned char)a`  (line 497) -> `func setPixelAtX(_:Y:toRed:
      green:blue:alpha:)`
- [x] `- (void)setAlphaAtX: (int)x Y: (int)y to: (unsigned char)a`  (line 521) ->
      `func setAlphaAtX(_:Y:to:)`
- [x] `- (CGImageRef)stippleMask`  (line 542, the `NS_IMPL_COCOA` arm) -> `func
      stippleMask() -> CGImage?`
- [x] `- (NSColor *)stippleMask`  (line 560, the GNUstep `#else` arm) - not this lane's
      target; recorded, not transcribed
- [x] `- (NSBitmapImageRep *)getAnimatedBitmapImageRep`  (line 569) ->
      `func getAnimatedBitmapImageRep() -> NSBitmapImageRep?`
- [x] `- (Lisp_Object)getMetadata`  (line 585) -> `func getMetadata() -> Lisp_Object`
- [x] `- (BOOL)setFrame: (unsigned int)index`  (line 606) -> `func setFrame(_ index: UInt32)
      -> Bool`
- [x] `- (void)setTransform: (double[3][3])m`  (line 627) -> `func setTransform(_ m:
      UnsafeMutablePointer<double3>)`
- [x] `- (void)setSmoothing: (BOOL)s`  (line 635) -> `func setSmoothing(_ s: Bool)`
- [x] `- (size_t)sizeInBytes`  (line 641) -> `func sizeInBytes() -> Int`

## File-scope objects — 0

## Macros defined — 1
- [x] COLORSPACE_NAME  (line 45, body dd5321bd07d6)

## Types and their fields — 0
(The unit declares `struct image`, `double3` and `corners4` because it dereferences
`struct image *` and takes `double m[3][3]`; those types belong to src/dispextern.h and
src/lisp.h, and their layouts are asserted below, so they are not counted a second time.)

## Decisions

**Layout of the C types this unit dereferences.** src/nsimage.m is not compiled by the frozen
configuration, and `struct image` (src/dispextern.h 3215) sits inside `#ifdef
HAVE_WINDOW_SYSTEM` (3203), which that configuration leaves undefined.  The unit therefore
carries the mirror of `struct image` for the **Cocoa** target (none of its own platform arms -
`USE_CAIRO`, `HAVE_X_WINDOWS`, `HAVE_ANDROID`, `HAVE_NTGUI`, `HAVE_HAIKU` - is defined there,
so the field list is the same on both 64-bit darwin builds) and proves it twice:

1. a clang probe against the frozen headers plus the three NS arms
   (`-DHAVE_WINDOW_SYSTEM=1 -DHAVE_NS=1 -DNS_IMPL_COCOA=1`) prints `sizeof image 240` and the
   `offsetof` of all 32 members (Evidence);
2. a Swift program compiled **together with this unit** prints `MemoryLayout<image>` for the
   same members; every number equals the C probe's (Evidence).

`Darwin.timespec` is the C struct, so it is used directly.  `int corners[4]` is `corners4`
(four `Int32`), `double m[3][3]` is `UnsafeMutablePointer<double3>`, `bool_bf ... : 1` triples
are one `UInt8` (`background_bits`, at offset 96; `refresh` then sits at 100, which is where
the C bitfield storage unit and its padding put it), `Emacs_Pixmap` (`id` under `__OBJC__`,
`void *` otherwise, src/nsgui.h 49-55) is a raw pointer, and `EMACS_UINT`/`ptrdiff_t`/`size_t`
are `UInt`/`Int`/`Int` on this target.

**`Lisp_Object` is the pointer alone.** PORTING.md 1.2 keeps C's pointer form; Swift's
`@_cdecl` refuses a parameter whose pointee is a Swift-declared type, so the unit writes
`typealias Lisp_Object = UnsafeMutableRawPointer?`.  It is still a pointer-sized value with no
wrapper, `Qnil` is its `nil`, and nothing in this file dereferences it - the same property the
C has, where `struct Lisp_X` is incomplete.

**Header-only constructs come from the core, not from this file.** `EQ`, `STRINGP`, `FIXNUMP`,
`XFIXNAT`, `XCDR`, `SSDATA`, `SBYTES`, `make_fixnum` and the twelve `Q*` are macros or INLINE
functions in src/lisp.h and the generated globals.h: they have no symbol in the reference, and
their bodies need the interpreter's tag bits, `lispsym` and (for `EQ`) `BASE_EQ` plus
`symbols_with_pos_enabled`/`slow_eq` (src/lisp.h 1314-1325).  Transcribing them into a platform
file would duplicate the core's internals, so the lane names each by the C construct's own
name and the core lane exports the C-ABI entry; the `## ABI` section is the list.  `Qfoo` is
called as `Qfoo ()`, which is the call shape the Rust mirror's `pub fn Qfoo ()` already has
(PORTING.md P-17).

**`eassert` is kept, `NSTRACE` is not.** src/lisp.h 181's disabled form is
`((void) (false && (cond)))`: the argument must still compile, and it is never evaluated.  The
unit reproduces that exactly (`@autoclosure`, `_ = false && cond()`).  `NSTRACE (…)` is the
opposite case: with `NSTRACE_ENABLED` 0 (src/nsterm.h 143) the macro expands to nothing and
discards its arguments unread (src/nsterm.h 276-330), so the five call sites (lines 65, 130,
139, 147, 160) carry no statement; a Swift no-op function would add a call the C text does not
have, so they are recorded here.

**`#if` arms.** The lane's target is the Cocoa arm, so:
- `#ifdef NS_IMPL_COCOA` (lines 33-35, 58, 71-96, 268-287, 299-307, 536-543, 549-558) is the
  live arm throughout; the GNUstep `#else` arms (97-113, 552-556, 619-625) are recorded, not
  transcribed.  `CGImageRef`/`CGImageCreateCopy`/`CGImageRelease` are the Cocoa spellings.
- `COLORSPACE_NAME` (36-49): the `NS_IMPL_GNUSTEP || MAC_OS_X_VERSION_MAX_ALLOWED < 1070` arm
  (36-43) is out of scope, so the port is a fileprivate computed property taking 45-49's live
  arm, with the SDK's modern names: `NSAppKitVersion.current` (the global the API notes rename
  from `NSAppKitVersionNumber`), `NSAppKitVersion.macOS10_7`,
  `NSColorSpaceName.deviceRGB`/`.calibratedRGB`.
- `#ifndef HAVE_RSVG` / `#ifndef HAVE_WEBP` (105-113): kept as Swift conditional compilation
  with the same macro names (`-D HAVE_RSVG` is the counterpart of the configure result).
  Swift's conditional-compilation blocks cannot split an `if`/`else` chain, so those two arms
  stand as separate `if` statements in C's order; the conditions are mutually exclusive symbol
  comparisons, so the value is the reference's.  This is the one place where the port's control
  flow is not C's, and it is a language limit, not a semantic choice.

**Objective-C constructs and their Swift counterparts.**
- `@implementation EmacsImage` -> `class EmacsImage: NSImage`; `static`-like file scope -> none
  here (the ivars are the class's).
- The two `[self initWithSize: …]` initializers are Swift `convenience` initializers, which is
  what delegating to the superclass' designated initializer means in Swift; they are also the
  reason the class keeps NSImage's `init(coder:)` and friends inherited instead of having to
  write unavailable stubs.
- ObjC nil-messaging (`[nil m]` is a no-op, and a nil-returning message yields 0/nil) is
  Swift's optional chaining with the zero default: `bmRep?.getBitmapDataPlanes(&planes)`,
  `eImg?.setPixmapData()`, `image?.size.width ?? 0`, `eImg?.setFrame(index) ?? false`.
- C's direct ivar stores are the port's force-unwraps at the same point: `image->bmRep = nil`
  (line 292) dereferences the failable initializer's result without a nil test, which faults in
  C and traps here, while every other nil case in the file is a message send and stays
  optional-chained.  `addRepresentation (bmRep)` (lines 418, 440) passes that same result
  unchecked; Swift's parameter is not optional, so the port force-unwraps (`bmRep!`) where C
  would pass nil and AppKit would raise.
- ObjC's unchecked casts are `unsafeDowncast` (`(NSBitmapImageRep *) rep`, `[super
  copyWithZone:]`, `[bmRep copyWithZone:]`); `(EmacsImage *) img` is
  `Unmanaged<EmacsImage>.fromOpaque(_).takeUnretainedValue()`; `(unsigned char *) bits` is
  `unsafeBitCast`; `struct image *img` is `assumingMemoryBound`.
- `respondsToSelector:@selector(…)` is `responds(to: #selector(…))` (no selector strings are
  built); `NSEnumerator` + `nextObject` loops are `for … in` with the same `break`.
- `dealloc` (310-321) is not transcribed: `CGImageRelease(stippleMask)`, `[bmRep release]`,
  `[transform release]` and `[super dealloc]` are exactly what ARC does for the three stored
  properties, and a `deinit` with no statements would be an empty body.  For the same reason
  `CGImageRetain`/`CGImageRelease`/`CGDataProviderRelease` are not declared: `stippleMask =
  CGImageRetain (mask)` is the property assignment (the extra reference the C code leaves
  behind is not reproduced), and the provider's release is ARC's.
- The container slots keep the reference's manual-retain semantics: `img->pixmap = eImg` hands
  the callee's +1 to the slot (`Unmanaged.passRetained`), and `ns_image_reset`'s
  `[(EmacsImage *)img release]` consumes the reference its callers transfer
  (`Unmanaged.fromOpaque(_).release()`), because src/image.c 5823 stores the result over the
  same slot.  Every `void *img` entry point is a **borrowed** reference.
- A name Swift rejects is renamed, and the name the other units call keeps its spelling: the
  C ivar `stippleMask` becomes `cachedStippleMask` because the `stippleMask()` method (called
  by src/nsterm.m 4120/4141/4505/4523) must keep the C name.

**Swift/Foundation spellings of the C calls** (semantics unchanged): `[NSData
dataWithBytes:length:]` is `Data(bytes:count:)`; `[NSBitmapImageRep
initWithBitmapDataPlanes:…]` keeps the pointer-formed call with `nil` planes; `getPixelAtX:Y:`
uses `bmRep?.colorAt(x:y:)` / `color.getRed(_:green:blue:alpha:)`; `setPixelAtX:Y:toRed:…`
uses `NSColor(calibratedRed:…)`; `NSImage.imageTypes` and `NSImage.setName(_:)` are the
imported spellings of `+imageTypes`/`-setName:`; `setTransformStruct:` is the
`transformStruct` property; `NSAffineTransform.transform` is `NSAffineTransform()`;
`NSImageFrameCount`/`NSImageCurrentFrame`/`NSImageCurrentFrameDuration` are
`NSBitmapImageRep.PropertyKey.frameCount`/`.currentFrame`/`.currentFrameDuration`.

**CoreGraphics.** The SDK's API notes rename `CGDataProviderCreateWithData` to
`CGDataProvider.init(dataInfo:data:size:releaseData:)`, `CGImageMaskCreate` to
`CGImage.init(maskWidth:…)` and `CGImageCreateCopy` to `CGImage.copy()`.  The renamed
initializer takes a non-optional release callback, so that call site would have to invent a
no-op callback the reference does not have; the unit declares the three C entry points with
`@_silgen_name` instead and keeps the reference's `NULL` arguments (`releaseData: nil`,
`decode: nil`, `provider` nullable).

**Integer and float conversions stay C's.** `unsigned long`/`size_t` -> `UInt`/`Int`, `int` ->
`Int32`, `unsigned int` -> `UInt32`, `unsigned char` -> `UInt8`, `BOOL`/`bool` -> `Bool`,
`CGFloat` -> `CGFloat`, `double` -> `Double`.  C's implicit narrowing is spelled with the
truncating conversion (`UInt32(truncatingIfNeeded: XFIXNAT(…))`, `UInt32(truncatingIfNeeded:
frames)`), so the reference's truncation is kept where Swift's checked conversion would trap;
`CGFloat` -> `int` is `Int32(x)`; `make_float(delay)` widens the C `float` to `Double`, which
is the same value the prototype's `double` parameter receives.  The pixel index keeps C's own
order of operations: `int loc = x + y * [self size].width;` computes in `CGFloat` and
truncates only at the assignment, so the port writes `Int(CGFloat(x) + CGFloat(y) *
size.width)` rather than truncating the width first.

**Locals C declares and assigns later.** Swift's definite initialization covers them, so the
port declares them without a value exactly where C does (`var size: NSSize`, `var lisp_index:
Lisp_Object`, the `int i, j, k` / `unsigned char c` of `initFromXBM:`'s block, `int i` of
`initForXPMWithDepth:`) and assigns them on C's assignments; no zero is invented that C does
not have.  `var eImg: EmacsImage? = nil` is the C `EmacsImage *eImg = nil;` itself.

**Variadic arguments to `add_to_log`.** src/xdisp.c 12288-12300 reads every argument with
`va_arg (ap, Lisp_Object)` - a pointer-sized value, 0 for `Qnil`.  Swift's `CVarArg` has no
conformance for an Optional pointer, so the unit passes the same bits as a `UInt`
(`lispVariadicArg`): `add_to_log("Unable to load image %s", lispVariadicArg(img->spec))`.

**A cross-file Objective-C method.** `[NSString stringWithLispString:found]` is declared in
src/nsterm.h 367 and defined in src/nsfns.m 4101-4140; Swift has no forward declaration for
another file's method, and this file is compiled on its own (`swiftc -c … nsimage.swift`), so
the call is spelled as the lane's C-ABI entry for that category method
(`stringWithLispString (found)`), which the port of nsfns.m owns.  When the lane is compiled
as one module this is the only place that needs a decision; see the ABI section.

**Findings while reading the reference (recorded, not changed).**
1. `getPixelAtX:Y:` (471-495): the `pixmapData[0] != NULL` branch builds the ARGB value in
   `unsigned long`, but the `colorAtX:y:` branch builds it in `int` and lets the return convert
   to `unsigned long`, which sign-extends on LP64 (`a == 1.0` yields `0xFFFFFFFFFF000000`, not
   `0xFF000000`).  The two branches disagree for an opaque pixel; the port keeps the
   conversion.  The only caller, src/image.c 373-378, documents itself as "Use with images
   created by ns_image_for_XPM", whose branch is the first one, so the second is latent.
2. `setFrame:` (606-625) tests `index < 0` on an `unsigned int`, which is always false; the
   comparison is kept and the `frames` conversion spelled out.
3. `initForXPMWithDepth:width:height:`'s `depth` parameter is never used, and `planes[4]` in
   `initFromXBM:` is beyond the four planes the initializer fills (the C code only reads
   0..3).  Both are kept as they are.
4. `allocInitFromFile:` (280-308) dereferences the failable `-initByReferencingFile:` result
   (`image->bmRep = nil;`) *before* testing `[image isValid]`, so an unreadable file - which
   makes that initializer return nil - faults before the intended `return nil`.  The port keeps
   the order and the fault.

## ABI

### (1) C symbols this file defines and exports (`@_cdecl`, the C names and signatures of
src/nsterm.h 1203-1222)

| C name | C prototype (src/nsterm.h) | Swift signature |
| --- | --- | --- |
| `ns_can_use_native_image_api` | `bool (Lisp_Object)` (1203) | `(Lisp_Object) -> Bool` |
| `ns_image_from_XBM` | `void *(char *, int, int, unsigned long, unsigned long)` (1204) | `(UnsafeMutablePointer<CChar>?, Int32, Int32, UInt, UInt) -> UnsafeMutableRawPointer?` |
| `ns_image_for_XPM` | `void *(int, int, int)` (1206) | `(Int32, Int32, Int32) -> UnsafeMutableRawPointer?` |
| `ns_image_from_file` | `void *(Lisp_Object)` (1207) | `(Lisp_Object) -> UnsafeMutableRawPointer?` |
| `ns_load_image` | `bool (struct frame *, struct image *, Lisp_Object, Lisp_Object)` (1208) | `(UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, Lisp_Object, Lisp_Object) -> Bool` |
| `ns_image_width` | `int (void *)` (1210) | `(UnsafeMutableRawPointer?) -> Int32` |
| `ns_image_height` | `int (void *)` (1211) | `(UnsafeMutableRawPointer?) -> Int32` |
| `ns_image_set_size` | `void (void *, int, int)` (1212) | `(UnsafeMutableRawPointer?, Int32, Int32) -> Void` |
| `ns_image_set_transform` | `void (void *, double[3][3])` (1213) | `(UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void` |
| `ns_image_set_smoothing` | `void (void *, bool)` (1214) | `(UnsafeMutableRawPointer?, Bool) -> Void` |
| `ns_get_pixel` | `unsigned long (void *, int, int)` (1215) | `(UnsafeMutableRawPointer?, Int32, Int32) -> UInt` |
| `ns_put_pixel` | `void (void *, int, int, unsigned long)` (1216) | `(UnsafeMutableRawPointer?, Int32, Int32, UInt) -> Void` |
| `ns_set_alpha` | `void (void *, int, int, unsigned char)` (1217) | `(UnsafeMutableRawPointer?, Int32, Int32, UInt8) -> Void` |
| `ns_image_reset` | `void *(void *, int, int)` (1218) | `(UnsafeMutableRawPointer?, Int32, Int32) -> UnsafeMutableRawPointer?` |
| `ns_image_size_in_bytes` | `size_t (void *)` (1222) | `(UnsafeMutableRawPointer?) -> Int` |

Ownership at this boundary is the reference's: `ns_image_from_XBM`/`ns_image_for_XPM`/
`ns_image_from_file`/`ns_image_reset`/`ns_load_image`'s `img->pixmap` all hand the caller a
**+1** reference (the C `[[EmacsImage alloc] init…]`), and every `void *img` parameter is
borrowed - `ns_image_reset` consumes the reference it is given.  `nm -g` on the compiled
object shows these 15 names (Evidence).

### (2) C symbols this file calls (all `@_silgen_name`)

**a. Symbols the reference defines out of line** (the ported core defines them):

| C name | prototype | declared | defined |
| --- | --- | --- | --- |
| `valid_image_p` | `bool valid_image_p (Lisp_Object)` | src/dispextern.h 3755 | src/image.c 1395 |
| `image_find_image_file` | `Lisp_Object image_find_image_file (Lisp_Object)` | src/dispextern.h 3748 | src/image.c 4508 |
| `add_to_log` | `void add_to_log (char const *, ...)` | src/lisp.h 4378 | src/xdisp.c 12279 |
| `plist_get` | `Lisp_Object plist_get (Lisp_Object, Lisp_Object)` | src/lisp.h 4280 | src/fns.c 2639 |
| `Fcons` | `Lisp_Object Fcons (Lisp_Object, Lisp_Object)` | globals.h 4560 (`EXFUN`) | src/alloc.c 2625 (`DEFUN "cons"`) |
| `make_float` | `Lisp_Object make_float (double)` | src/floatfns.c 4652 | src/alloc.c 2507 |
| `stringWithLispString` | `+ (NSString *)stringWithLispString:(Lisp_Object)` | src/nsterm.h 367 | src/nsfns.m 4104 |

**b. Header-only constructs** - macros, INLINE functions and generated-globals entries.  The
reference has no symbol for any of these, so the lane's core side (the Rust units that mirror
src/lisp.h and the generated globals.h) has to export the C-ABI entry named here:

| C name | reference form | where | the port's C-ABI entry |
| --- | --- | --- | --- |
| `EQ` | `INLINE bool EQ (Lisp_Object, Lisp_Object)` (needs `BASE_EQ`, `symbols_with_pos_enabled`, `slow_eq`) | src/lisp.h 1314-1325 | `#[unsafe(no_mangle)] pub extern "C" fn EQ (Lisp_Object, Lisp_Object) -> bool` (mirror `lisp.rs` 814) |
| `STRINGP` | `INLINE bool STRINGP (Lisp_Object)` | src/lisp.h 1581 | mirror `lisp.rs`'s `STRINGP` |
| `FIXNUMP` | `#define FIXNUMP(x) lisp_h_FIXNUMP (x)` / `INLINE bool (FIXNUMP) (Lisp_Object)` | src/lisp.h 395, 449, 1262 | mirror `lisp.rs`'s `FIXNUMP` |
| `XFIXNAT` | `INLINE EMACS_INT XFIXNAT (Lisp_Object)` | src/lisp.h 3246 | mirror `lisp.rs`'s `XFIXNAT` |
| `XCDR` | `#define XCDR(c) lisp_h_XCDR (c)` / `INLINE Lisp_Object (XCDR) (Lisp_Object)` | src/lisp.h 406, 456, 1500 | mirror `lisp.rs` 879 |
| `SSDATA` | `INLINE char *SSDATA (Lisp_Object)` | src/lisp.h 1658 | mirror `lisp.rs`'s `SSDATA` |
| `SBYTES` | `INLINE ptrdiff_t SBYTES (Lisp_Object)` | src/lisp.h 1697 | mirror `lisp.rs`'s `SBYTES` |
| `make_fixnum` | `#define make_fixnum(n) lisp_h_make_fixnum (n)` / `INLINE Lisp_Object (make_fixnum) (EMACS_INT)` | src/lisp.h 412-417, 459, 1184 | mirror `lisp.rs` 719 |
| `Qnil`, `Qnative_image`, `Qjpeg`, `Qpng`, `Qgif`, `Qtiff`, `Qsvg`, `Qwebp`, `Qheic`, `QCindex`, `Qcount`, `Qdelay` | generated globals.h `#define Qfoo builtin_lisp_symbol (iQfoo)`, indices 0/1378/1123/1497/934/1817/1761/2032/1006/114/596/663 | `docs/baseline/globals/symbols.tsv` 1, 1380, 1125, 1499, 936, 1819, 1763, 2034, 1008, 116, 598, 665 | mirror `globals.rs`'s `pub fn Qfoo ()` (e.g. `Qnil` 10, `Qcount` 1796) |
| `ns_use_srgb_colorspace` | `DEFVAR_BOOL ("ns-use-srgb-colorspace", …)` -> globals.h macro over `globals.f_ns_use_srgb_colorspace` | src/nsterm.m 11716 | a `bool` variable of that name |
| `CGDataProviderCreateWithData` | `CGDataProviderRef CGDataProviderCreateWithData (void *, const void *, size_t, CGDataProviderReleaseDataCallback)` | CoreGraphics (API notes rename it to `CGDataProvider.init(dataInfo:data:size:releaseData:)`, whose callback is not optional) | the SDK symbol, kept so `NULL` stays `NULL` |
| `CGImageMaskCreate` | `CGImageRef CGImageMaskCreate (size_t, size_t, size_t, size_t, size_t, CGDataProviderRef, const CGFloat *, bool)` | CoreGraphics (renamed to `CGImage.init(maskWidth:…)`) | the SDK symbol |
| `CGImageCreateCopy` | `CGImageRef CGImageCreateCopy (CGImageRef)` | CoreGraphics (renamed to `CGImage.copy()`) | the SDK symbol, because the renamed method cannot take its nullable argument |

Two further C constructs have no ABI entry because the unit spells them itself, exactly as the
C macro does: `eassert` (src/lisp.h 181, the disabled form) and `COLORSPACE_NAME` (src/nsimage.m
45-49).

### (3) What the lane still has to settle (flagged for the integrator)

1. The core must export (2.b)'s entries; `src/include/lisp.rs` and
   `globals.rs` define them as Rust items today (`#[inline(always)] pub fn`), and
   `#[unsafe(no_mangle)] pub extern "C"` on those items is the whole change.  The alternative -
   transcribing the constructs in Swift - is rejected: `EQ` alone needs the core's tag bits and
   `slow_eq`, and `make_fixnum` needs `INTTYPEBITS`/`Lisp_Int0` and the fixnum overflow test.
2. `stringWithLispString (…)` is this file's only cross-file Objective-C call, so it is the
   only proof that a stand-alone `swiftc -c` of a lane file needs such an entry; the port of
   src/nsfns.m owns its definition.  The rest of the lane's ObjC cross-file calls (nsmenu.m,
   nsselect.m and nsterm.m call the same categories, and nsterm.h declares
   `NSColor (EmacsColor)` and `NSString (EmacsString)`) need the same treatment.
3. `stringWithLispString` and the other `.m`-side C symbols of src/nsterm.h 1203-1222 have no
   `.m -> .swift` pair support in `tools/extract-abi.py` (it only generates the C-side
   checklist), so `extract-abi.py --check` cannot compare this unit; the mirror-dupes,
   declarations and verbatim gates do cover it.

## Evidence

- `swiftc --version` ->
  `swift-driver version: 1.168.6 Apple Swift version 6.4 (swiftlang-6.4.0.34.1 clang-2100.3.34.1)`,
  `Target: arm64-apple-macosx26.0` (SDK `MacOSX.sdk`, macOS 26.6.2).
- `cd /Users/liu/projects/src/github.com/liuchong/femacs && swiftc -parse-as-library -c -o
  /tmp/nsimage.o platform/macos/nsimage.swift` -> **exit 0**, in the default language
  mode (Swift 5 semantics; no `-swift-version` flag was needed and none was used), with two
  warnings: `class EmacsImage: NSImage {` is a non-final class and `var bmRep` is a mutable
  stored property, so the class cannot conform to `Sendable` - "this is an error in the Swift 6
  language mode", a warning here.  No global-mutable-state or strict-concurrency error arose.
- `nm -g /tmp/nsimage.o | grep ' T _ns_'` -> the 15 exported C names:
  `_ns_can_use_native_image_api`, `_ns_get_pixel`, `_ns_image_for_XPM`, `_ns_image_from_XBM`,
  `_ns_image_from_file`, `_ns_image_height`, `_ns_image_reset`, `_ns_image_set_size`,
  `_ns_image_set_smoothing`, `_ns_image_set_transform`, `_ns_image_size_in_bytes`,
  `_ns_image_width`, `_ns_load_image`, `_ns_put_pixel`, `_ns_set_alpha` (15 of 15).
- `struct image` layout, C side (`clang -std=gnu23 -DHAVE_WINDOW_SYSTEM=1 -DHAVE_NS=1
  -DNS_IMPL_COCOA=1 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib`
  on a probe that includes `<config.h>`, `"lisp.h"`, `"dispextern.h"` and prints
  `sizeof`/`offsetof`) -> `sizeof image 240`; `timestamp 0, pixmap 16, mask 24, colors 32,
  ncolors 40, background 48, face_foreground 56, face_background 64, face_font_size 72,
  face_font_family 80, face_font_height 88, face_font_width 92, refresh 100, original_width
  104, original_height 108, width 112, height 116, scale 120, corners 128, ascent 144, spec
  152, dependencies 160, relief 168, hmargin 172, vmargin 176, type 184, load_failed_p 192,
  lisp_data 200, hash 208, id 216, next 224, prev 232`; `sizeof timespec 16` (`tv_sec` 0,
  `tv_nsec` 8).  (`refresh` at 100 puts the three `bool_bf` bits in the byte at 96.)
- The same layout, Swift side (`swiftc -o /tmp/run platform/macos/nsimage.swift
  /tmp/nsimage-layout-check/main.swift <a one-line C stub that defines the unit's C globals>
  -Xlinker -undefined -Xlinker dynamic_lookup`, then the program, which prints
  `MemoryLayout<image>` of the unit's own struct) -> `sizeof image 240; pixmap 16, mask 24,
  ncolors 40, background 48, face_font_family 80, background_bits 96, refresh 100, scale 120,
  corners 128, corners.v3 140, spec 152, load_failed_p 192, lisp_data 200, hash 208, id 216,
  next 224, prev 232, width 112, height 116; sizeof timespec 16, double3 24, corners4 16` -
  every number equals the C probe's.
- `python3 tools/extract-abi.py emacs/src/nsimage.m` -> the checklist above (15 functions,
  0 subrs, 1 macro, 0 types, 0 file-scope objects); the tool has no `.m -> .swift` pair, so
  `--check` cannot be run on this unit.
- `python3 tools/check-cond-eval.py src/nsimage.m` ->
  `src/nsimage.m: 10 conditions, 0 unknown, 2 disagreements [NOT VERIFIED]`, both at
  `ifndef HAVE_RSVG`/`ifndef HAVE_WEBP` (105, 109): clang says the branch is dead because the
  frozen configuration never enters the enclosing `#ifdef NS_IMPL_COCOA` block, while the
  evaluator keeps `#ifndef` true.  The disagreement is the not-built-on-target situation
  §3.31 describes, not a mis-evaluated arm.
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures` (none of them this unit).
- `python3 tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`.
- `python3 tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s),
  0 failure(s)` (neither the duplicate nor the ratchet row is this unit).
- `python3 tools/check-globals.py` -> `check-globals: 283 units, 0 DEFVAR_* variables without
  a definition in their own unit`.
- `python3 tools/check-symbols.py` -> `86 units compared, 2 mismatched, 0 still pending`; both
  mismatches are `lib/mempcpy` and `lib/regex` (pre-existing, not this unit - the unit has no
  `.m` pair for that gate).
- `python3 tools/check-linkage.py` -> `3 unit object(s) compared against the non-static objects
  of the C sources the frozen build links, 0 with 0 missing symbol(s)`.
- `/tmp/nsimage.o` and the probe files were deleted after these readings.
