// femacs: port of src/nsimage.m @ fd7afced403.
//
// Swift has no `#include`, so this unit carries the declarations of the C environment it
// uses.  They are of three kinds, and docs/ports/nsimage.md's `## ABI` section lists every
// one of them with its reference location:
//
//   * `@_silgen_name` imports a symbol the ported core defines (`valid_image_p`, `Fcons`,
//     ...); their prototypes come from src/lisp.h and src/dispextern.h.
//   * Header-only constructs - the macros and INLINE functions of src/lisp.h, and the
//     entries the generated globals.h turns into macros (`Qnil`, `ns_use_srgb_colorspace`)
//     - have no symbol in the reference, so the lane names each by the C construct's own
//     name and the core lane exports the C-ABI entry.  They cannot be transcribed here:
//     EQ alone needs BASE_EQ plus symbols_with_pos_enabled and slow_eq (src/lisp.h 1314).
//   * `@_cdecl` exports this file's C interface with the C names and signatures unchanged
//     (src/nsterm.h 1203-1222).
//
// src/nsimage.m 36-43's GNUstep arm and 97-113's GNUstep spelling of the image types are
// not this lane's target (the lane is the Cocoa/AppKit one the reference's `NS_IMPL_COCOA`
// arms describe); the ledger records them.

import Foundation
import AppKit
import CoreGraphics

// ==========================================================================
// Lane ABI: C types.
// ==========================================================================

/// C: `typedef struct Lisp_X *Lisp_Word; typedef Lisp_Word Lisp_Object;` (src/lisp.h
/// 311-323, the pointer form the frozen configuration uses; PORTING.md P-01/1.2).  The
/// pointee is incomplete in C too, and a Swift pointer to a Swift-declared type cannot be a
/// `@_cdecl` parameter, so the lane keeps the pointer alone - still a pointer-sized value
/// with no wrapper (PORTING.md 1.2), and `Qnil` is its nil.
typealias Lisp_Object = UnsafeMutableRawPointer?

/// C: `#ifdef __OBJC__ typedef id Emacs_Pixmap; #else typedef void *Emacs_Pixmap;`
/// (src/nsgui.h 49-55) - an object pointer either way, so a pointer here.
typealias Emacs_Pixmap = UnsafeMutableRawPointer?

/// C: `double m[3][3]` as a parameter type, i.e. `double (*)[3]`; a struct of three
/// doubles has an element's layout, so `m[i][j]` stays `m[i].vj` (src/nsimage.m 627).
struct double3 {
  var v0: Double
  var v1: Double
  var v2: Double
}

/// C: `int corners[4];` (src/dispextern.h, the `corners` member of `struct image`) - four
/// consecutive ints.
struct corners4 {
  var v0: Int32
  var v1: Int32
  var v2: Int32
  var v3: Int32
}

/// C: `struct image` (src/dispextern.h 3215-3300).  Its platform arms (USE_CAIRO,
/// HAVE_X_WINDOWS, HAVE_ANDROID, HAVE_NTGUI, HAVE_HAIKU) are not defined for this lane's
/// target, which is the text the frozen darwin-arm64 configuration compiles;
/// docs/ports/nsimage.md's Evidence records the `offsetof`/`sizeof` probe.
struct image {
  var timestamp: timespec
  var pixmap: Emacs_Pixmap
  var mask: Emacs_Pixmap
  var colors: UnsafeMutablePointer<UInt>?
  var ncolors: Int32
  var background: UInt
  var face_foreground: UInt
  var face_background: UInt
  var face_font_size: Int32
  var face_font_family: UnsafeMutablePointer<CChar>?
  var face_font_height: Int32
  var face_font_width: Int32
  /// C: `bool_bf background_transparent : 1; bool_bf background_valid : 1,
  /// background_transparent_valid : 1;` - one byte with three bits used (PORTING.md 1.3).
  var background_bits: UInt8
  var refresh: UInt32
  var original_width: Int32
  var original_height: Int32
  var width: Int32
  var height: Int32
  var scale: Double
  var corners: corners4
  var ascent: Int32
  var spec: Lisp_Object
  var dependencies: Lisp_Object
  var relief: Int32
  var hmargin: Int32
  var vmargin: Int32
  var type: OpaquePointer?
  var load_failed_p: Bool
  var lisp_data: Lisp_Object
  var hash: UInt
  var id: Int
  var next: UnsafeMutablePointer<image>?
  var prev: UnsafeMutablePointer<image>?
}

// ==========================================================================
// Lane ABI: the C side.
// ==========================================================================

// Functions of the ported core (declared in src/lisp.h 4280/4378 and src/dispextern.h
// 3748/3755; defined in src/image.c, src/xdisp.c, src/alloc.c).
@_silgen_name("valid_image_p") func valid_image_p(_ object: Lisp_Object) -> Bool
@_silgen_name("image_find_image_file") func image_find_image_file(_ file: Lisp_Object) -> Lisp_Object
@_silgen_name("add_to_log") func add_to_log(_ format: UnsafePointer<CChar>, _ args: CVarArg...)
@_silgen_name("plist_get") func plist_get(_ plist: Lisp_Object, _ prop: Lisp_Object) -> Lisp_Object
@_silgen_name("Fcons") func Fcons(_ car: Lisp_Object, _ cdr: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object make_float (double)` (declared src/lisp.h 4652, defined
/// src/alloc.c 2507); the reference passes a `float`, which the prototype widens.
@_silgen_name("make_float") func make_float(_ float_value: Double) -> Lisp_Object

// Header-only constructs (macros, INLINE functions, generated globals.h entries); the C
// names are the constructs' own names.  See the ledger's ABI section for each one's
// reference location.
@_silgen_name("EQ") func EQ(_ x: Lisp_Object, _ y: Lisp_Object) -> Bool
@_silgen_name("STRINGP") func STRINGP(_ x: Lisp_Object) -> Bool
@_silgen_name("FIXNUMP") func FIXNUMP(_ x: Lisp_Object) -> Bool
@_silgen_name("XFIXNAT") func XFIXNAT(_ a: Lisp_Object) -> Int
@_silgen_name("XCDR") func XCDR(_ c: Lisp_Object) -> Lisp_Object
/// C: `INLINE char *SSDATA (Lisp_Object string)` (src/lisp.h 1658) - never null for a
/// string, which is the only way this file calls it.
@_silgen_name("SSDATA") func SSDATA(_ string: Lisp_Object) -> UnsafeMutablePointer<CChar>
/// C: `INLINE ptrdiff_t SBYTES (Lisp_Object string)` (src/lisp.h 1697).
@_silgen_name("SBYTES") func SBYTES(_ string: Lisp_Object) -> Int
@_silgen_name("make_fixnum") func make_fixnum(_ n: Int) -> Lisp_Object
/// C: `#define Qfoo builtin_lisp_symbol (...)` - one per symbol, generated into globals.h
/// from the DEFSYM set (PORTING.md P-17); called as `Qfoo ()` here, as the Rust mirror's
/// `pub fn Qfoo ()` does.
@_silgen_name("Qnil") func Qnil() -> Lisp_Object
@_silgen_name("Qnative_image") func Qnative_image() -> Lisp_Object
@_silgen_name("Qjpeg") func Qjpeg() -> Lisp_Object
@_silgen_name("Qpng") func Qpng() -> Lisp_Object
@_silgen_name("Qgif") func Qgif() -> Lisp_Object
@_silgen_name("Qtiff") func Qtiff() -> Lisp_Object
@_silgen_name("Qsvg") func Qsvg() -> Lisp_Object
@_silgen_name("Qwebp") func Qwebp() -> Lisp_Object
@_silgen_name("Qheic") func Qheic() -> Lisp_Object
@_silgen_name("QCindex") func QCindex() -> Lisp_Object
@_silgen_name("Qcount") func Qcount() -> Lisp_Object
@_silgen_name("Qdelay") func Qdelay() -> Lisp_Object
/// C: `ns_use_srgb_colorspace` (src/nsterm.m 11716's `DEFVAR_BOOL`), which the generated
/// globals.h spells as a macro over the `globals` record's field.
@_silgen_name("ns_use_srgb_colorspace") var ns_use_srgb_colorspace: Bool
/// C: `+[NSString stringWithLispString:]` (declared src/nsterm.h 367, defined src/nsfns.m
/// 4104).  A stand-alone translation unit has no forward declaration for a method another
/// unit defines, so the lane spells this cross-file category method as a C-ABI entry.
@_silgen_name("stringWithLispString") func stringWithLispString(_ string: Lisp_Object) -> NSString?

/// C: `CGDataProviderCreateWithData` (CoreGraphics; the SDK's API notes rename it to
/// `CGDataProvider.init(dataInfo:data:size:releaseData:)`, whose callback is not optional).
/// The C entry point is declared so the reference's NULL release callback stays NULL.
@_silgen_name("CGDataProviderCreateWithData")
func CGDataProviderCreateWithData(_ info: UnsafeMutableRawPointer?, _ data: UnsafeRawPointer?,
                                  _ size: Int,
                                  _ releaseData: (@convention(c) (UnsafeMutableRawPointer?, UnsafeRawPointer, Int) -> Void)?) -> CGDataProvider?
/// C: `CGImageMaskCreate` (renamed to `CGImage.init(maskWidth:height:…)` by the API notes).
@_silgen_name("CGImageMaskCreate")
func CGImageMaskCreate(_ width: Int, _ height: Int, _ bitsPerComponent: Int,
                       _ bitsPerPixel: Int, _ bytesPerRow: Int, _ provider: CGDataProvider?,
                       _ decode: UnsafePointer<CGFloat>?, _ shouldInterpolate: Bool) -> CGImage?
/// C: `CGImageCreateCopy` (renamed to `CGImage.copy()` by the API notes); its argument is
/// nullable, which the renamed method cannot express.
@_silgen_name("CGImageCreateCopy") func CGImageCreateCopy(_ image: CGImage?) -> CGImage?

/// C: `#define eassert(cond) ((void) (false && (cond)))` (src/lisp.h 181): the disabled
/// form still requires COND to compile but never evaluates it, and `&&`'s short circuit
/// keeps that here.
@inline(__always)
func eassert(_ cond: @autoclosure () -> Bool) {
  _ = false && cond()
}

/// C: `NSTRACE_ENABLED` is 0 in the frozen configuration (src/nsterm.h 143), so `NSTRACE
/// (…)` has no expansion and its arguments are not compiled at all (src/nsterm.h 276-330).
/// The five call sites (src/nsimage.m 65, 130, 139, 147, 160) therefore carry no statement
/// and are recorded in the ledger, not transcribed: a Swift no-op function would add a call
/// the C text does not have.

/// C: `add_to_log`'s variadic arguments are read back with `va_arg (ap, Lisp_Object)`
/// (src/xdisp.c 12288-12300) - a pointer-sized value, 0 for `Qnil`.  Swift's `CVarArg` has
/// no conformance for an Optional pointer, so the port passes the same bits as a `UInt`.
@inline(__always)
func lispVariadicArg(_ value: Lisp_Object) -> UInt {
  return value.map { UInt(bitPattern: UnsafeRawPointer($0)) } ?? 0
}

/// C: `NSCalibratedRGBColorSpace` in the macro's pre-10.7 arm (src/nsimage.m 36-43); the
/// lane's target takes the arm at 45-49, whose modern Swift spelling is the two
/// `NSColorSpaceName` constants below.
fileprivate var COLORSPACE_NAME: NSColorSpaceName {
  return (ns_use_srgb_colorspace
            && NSAppKitVersion.current.rawValue >= NSAppKitVersion.macOS10_7.rawValue)
    ? NSColorSpaceName.deviceRGB : NSColorSpaceName.calibratedRGB
}

/* ==========================================================================

   C interface.  This allows easy calling from C files.  We could just
   compile everything as Objective-C, but that might mean slower
   compilation and possible difficulties on some platforms.

   ========================================================================== */

@_cdecl("ns_can_use_native_image_api")
func ns_can_use_native_image_api(_ type: Lisp_Object) -> Bool {
  var imageType: NSString = "unknown"
  var types: NSArray

  // (NSTRACE ("ns_can_use_native_image_api") - no expansion, recorded in the ledger.)

  if EQ(type, Qnative_image()) { return true }

  /* Work out the UTI of the image type.  */
  if EQ(type, Qjpeg()) { imageType = "public.jpeg" }
  else if EQ(type, Qpng()) { imageType = "public.png" }
  else if EQ(type, Qgif()) { imageType = "com.compuserve.gif" }
  else if EQ(type, Qtiff()) { imageType = "public.tiff" }
  // src/nsimage.m 105-113's `#ifndef HAVE_RSVG` / `#ifndef HAVE_WEBP` arms.  Swift's
  // conditional compilation cannot split an `if`/`else` chain, so the two configuration
  // arms stand as separate `if` statements in C's own order; the conditions are mutually
  // exclusive symbol comparisons, so the value is the reference's.
  #if !HAVE_RSVG
  if EQ(type, Qsvg()) { imageType = "public.svg-image" }
  #endif
  #if !HAVE_WEBP
  if EQ(type, Qwebp()) { imageType = "org.webmproject.webp" }
  #endif
  if EQ(type, Qheic()) { imageType = "public.heic" }

  /* NSImage also supports a host of other types such as PDF and BMP,
     but we don't yet support these in image.c.  */

  types = NSImage.imageTypes as NSArray

  /* Check if the type is supported on this system.  */
  if types.index(of: imageType) != NSNotFound { return true }
  else { return false }
}

@_cdecl("ns_image_from_XBM")
func ns_image_from_XBM(_ bits: UnsafeMutablePointer<CChar>?, _ width: Int32, _ height: Int32,
                       _ fg: UInt, _ bg: UInt) -> UnsafeMutableRawPointer? {
  // (NSTRACE ("ns_image_from_XBM") - no expansion, recorded in the ledger.)
  // C's `[[EmacsImage alloc] initFromXBM: (unsigned char *) bits …]`: the cast is the
  // bit cast, and the returned reference is the callee's +1.
  let image = EmacsImage(fromXBM: unsafeBitCast(bits, to: UnsafeMutablePointer<UInt8>.self),
                         width: width, height: height, fg: fg, bg: bg, reverseBytes: true)
  return UnsafeMutableRawPointer(Unmanaged.passRetained(image).toOpaque())
}

@_cdecl("ns_image_for_XPM")
func ns_image_for_XPM(_ width: Int32, _ height: Int32, _ depth: Int32) -> UnsafeMutableRawPointer? {
  // (NSTRACE ("ns_image_for_XPM") - no expansion, recorded in the ledger.)
  let image = EmacsImage(forXPMWithDepth: depth, width: width, height: height)
  return UnsafeMutableRawPointer(Unmanaged.passRetained(image).toOpaque())
}

@_cdecl("ns_image_from_file")
func ns_image_from_file(_ file: Lisp_Object) -> UnsafeMutableRawPointer? {
  // (NSTRACE ("ns_image_from_file") - no expansion, recorded in the ledger.)
  guard let image = EmacsImage.allocInitFromFile(file) else { return nil }
  return UnsafeMutableRawPointer(Unmanaged.passRetained(image).toOpaque())
}

@_cdecl("ns_load_image")
func ns_load_image(_ f: UnsafeMutableRawPointer?, _ img: UnsafeMutableRawPointer?,
                   _ spec_file: Lisp_Object, _ spec_data: Lisp_Object) -> Bool {
  // C: `struct frame *f` is not read here, and `struct image *img` is what the reference
  // dereferences; `void *` is what crosses the ABI, so this is the reference's own cast.
  let img = img!.assumingMemoryBound(to: image.self)
  var eImg: EmacsImage? = nil
  var size: NSSize
  var lisp_index: Lisp_Object
  var index: UInt32

  _ = f
  // (NSTRACE ("ns_load_image") - no expansion, recorded in the ledger.)

  eassert(valid_image_p(img.pointee.spec))

  lisp_index = plist_get(XCDR(img.pointee.spec), QCindex())
  // C's `unsigned int` assignment truncates; `truncatingIfNeeded` is that conversion.
  index = FIXNUMP(lisp_index) ? UInt32(truncatingIfNeeded: XFIXNAT(lisp_index)) : 0

  if STRINGP(spec_file) {
    eImg = EmacsImage.allocInitFromFile(spec_file)
  } else if STRINGP(spec_data) {
    let data: Data

    data = Data(bytes: UnsafeRawPointer(SSDATA(spec_data)), count: SBYTES(spec_data))
    eImg = EmacsImage(data: data)
    eImg?.setPixmapData()
  }

  if eImg == nil {
    add_to_log("Unable to load image %s", lispVariadicArg(img.pointee.spec))
    return false
  }

  if !(eImg?.setFrame(index) ?? false) {
    add_to_log("Unable to set index %d for image %s",
               lispVariadicArg(make_fixnum(Int(index))), lispVariadicArg(img.pointee.spec))
    return false
  }

  img.pointee.lisp_data = eImg!.getMetadata()

  size = eImg!.size
  img.pointee.width = Int32(size.width)
  img.pointee.height = Int32(size.height)

  /* 4) set img->pixmap = emacsimage */
  // The reference's `img->pixmap = eImg` hands the object's +1 to the slot (image.c 5823
  // releases it again through ns_image_reset), so the port passes a retained reference.
  img.pointee.pixmap = UnsafeMutableRawPointer(Unmanaged.passRetained(eImg!).toOpaque())

  return true
}

@_cdecl("ns_image_width")
func ns_image_width(_ img: UnsafeMutableRawPointer?) -> Int32 {
  // C: `return [(id)img size].width;` - a null pointer is ObjC's nil, whose message send
  // yields zero.
  let image = img.map { Unmanaged<EmacsImage>.fromOpaque($0).takeUnretainedValue() }
  return Int32(image?.size.width ?? 0)
}

@_cdecl("ns_image_height")
func ns_image_height(_ img: UnsafeMutableRawPointer?) -> Int32 {
  let image = img.map { Unmanaged<EmacsImage>.fromOpaque($0).takeUnretainedValue() }
  return Int32(image?.size.height ?? 0)
}

@_cdecl("ns_image_set_size")
func ns_image_set_size(_ img: UnsafeMutableRawPointer?, _ width: Int32, _ height: Int32) {
  let image = img.map { Unmanaged<EmacsImage>.fromOpaque($0).takeUnretainedValue() }
  image?.size = NSSize(width: CGFloat(width), height: CGFloat(height))
}

@_cdecl("ns_image_set_transform")
func ns_image_set_transform(_ img: UnsafeMutableRawPointer?, _ m: UnsafeMutableRawPointer?) {
  let image = img.map { Unmanaged<EmacsImage>.fromOpaque($0).takeUnretainedValue() }
  image?.setTransform(m!.assumingMemoryBound(to: double3.self))
}

@_cdecl("ns_image_set_smoothing")
func ns_image_set_smoothing(_ img: UnsafeMutableRawPointer?, _ smooth: Bool) {
  let image = img.map { Unmanaged<EmacsImage>.fromOpaque($0).takeUnretainedValue() }
  image?.setSmoothing(smooth)
}

@_cdecl("ns_get_pixel")
func ns_get_pixel(_ img: UnsafeMutableRawPointer?, _ x: Int32, _ y: Int32) -> UInt {
  let image = img.map { Unmanaged<EmacsImage>.fromOpaque($0).takeUnretainedValue() }
  return image?.getPixelAtX(x, Y: y) ?? 0
}

@_cdecl("ns_put_pixel")
func ns_put_pixel(_ img: UnsafeMutableRawPointer?, _ x: Int32, _ y: Int32, _ argb: UInt) {
  let image = img.map { Unmanaged<EmacsImage>.fromOpaque($0).takeUnretainedValue() }
  var alpha = UInt8((argb >> 24) & 0xFF)
  if alpha == 0 {
    alpha = 0xFF
  }
  image?.setPixelAtX(x, Y: y, toRed: UInt8((argb >> 16) & 0xFF),
                     green: UInt8((argb >> 8) & 0xFF), blue: UInt8(argb & 0xFF),
                     alpha: alpha)
}

@_cdecl("ns_set_alpha")
func ns_set_alpha(_ img: UnsafeMutableRawPointer?, _ x: Int32, _ y: Int32, _ a: UInt8) {
  let image = img.map { Unmanaged<EmacsImage>.fromOpaque($0).takeUnretainedValue() }
  image?.setAlphaAtX(x, Y: y, to: a)
}

@_cdecl("ns_image_reset")
func ns_image_reset(_ img: UnsafeMutableRawPointer?, _ width: Int32, _ height: Int32) -> UnsafeMutableRawPointer? {
  // C's `[(EmacsImage *)img release]`: the parameter is the reference the callers hand
  // over (src/image.c 5823 stores the result over the same slot), so the port consumes it.
  if let img = img { Unmanaged<EmacsImage>.fromOpaque(img).release() }
  return ns_image_for_XPM(width, height, 32)
}

@_cdecl("ns_image_size_in_bytes")
func ns_image_size_in_bytes(_ img: UnsafeMutableRawPointer?) -> Int {
  let image = img.map { Unmanaged<EmacsImage>.fromOpaque($0).takeUnretainedValue() }
  return image?.sizeInBytes() ?? 0
}

/* ==========================================================================

   Class supporting bitmaps and images of various sorts.

   ========================================================================== */

/// C: `@interface EmacsImage : NSImage` (src/nsterm.h 670-703); the ivars are src/nsterm.h
/// 672-681.  The two `@public` ivars stay module-visible because src/nsterm.m reads them
/// (4342, 4349, 4359).
class EmacsImage: NSImage {

  /// C: `NSBitmapImageRep *bmRep;` used for accessing pixel data.
  fileprivate var bmRep: NSBitmapImageRep? = nil
  /// C: `unsigned char *pixmapData[5];` shortcut to access pixel data.  Swift has no
  /// fixed-size array, so the five slots are an array; only this file reads them.
  fileprivate var pixmapData: [UnsafeMutablePointer<UInt8>?] = [nil, nil, nil, nil, nil]
  /// C: `CGImageRef stippleMask;` (the Cocoa arm of src/nsterm.h 674-678).  The C ivar and
  /// the `stippleMask` method below share their name; Swift rejects that collision, so the
  /// method keeps the C name (src/nsterm.m 4120/4141/4505/4523 calls it) and the cache is
  /// spelled here.
  fileprivate var cachedStippleMask: CGImage? = nil
  /// C: `@public NSAffineTransform *transform;`
  var transform: NSAffineTransform? = nil
  /// C: `@public BOOL smoothing;`
  var smoothing: Bool = false

  /* Search bitmap-file-path for the file, if appropriate and hand it over.  */
  class func allocInitFromFile(_ file: Lisp_Object) -> EmacsImage? {
    var imgRep: NSImageRep?
    var found: Lisp_Object
    var image: EmacsImage?
    var filename: NSString?

    /* Search bitmap-file-path for the file, if appropriate.  */
    found = image_find_image_file(file)
    if !STRINGP(found) { return nil }
    filename = stringWithLispString(found)

    // C: `image = [[EmacsImage alloc] initByReferencingFile:filename];` - ObjC hands the
    // possibly-nil NSString on; the Swift initializer takes a String, so the same message
    // send is spelled with the value bound first (nsfns.m's factory returns nil only for a
    // non-string, which STRINGP above excludes).
    image = filename.flatMap { EmacsImage(byReferencingFile: $0 as String) }

    // C stores into the raw ivar (`image->bmRep = nil`), which faults on a nil image; the
    // port keeps that one unchecked dereference and leaves the message sends tolerant.
    image!.bmRep = nil
    if !(image?.isValid ?? false) {
      return nil
    }
    imgRep = image?.representations.first

    if let imgRep = imgRep {
      image?.size = NSSize(width: CGFloat(imgRep.pixelsWide),
                           height: CGFloat(imgRep.pixelsHigh))
    }
    if let filename = filename { _ = image?.setName(filename as String) }

    return image
  }

  // C: `- (void)dealloc` (src/nsimage.m 310-321) releases its three references and calls
  // `[super dealloc]`.  Swift is ARC-only, so those releases are implicit and there is no
  // `deinit` to write; recorded in the ledger.

  override func copy(with zone: NSZone?) -> Any {
    let copy = unsafeDowncast(super.copy(with: zone) as AnyObject, to: EmacsImage.self)

    copy.cachedStippleMask = CGImageCreateCopy(cachedStippleMask)
    copy.bmRep = bmRep.map { unsafeDowncast($0.copy(with: zone) as AnyObject, to: NSBitmapImageRep.self) }
    copy.transform = transform.map { unsafeDowncast($0.copy(with: zone) as AnyObject, to: NSAffineTransform.self) }

    return copy
  }

  /* Create image from monochrome bitmap. If both FG and BG are 0
     (black), set the background to white and make it transparent.  */
  convenience init(fromXBM bits: UnsafeMutablePointer<UInt8>, width w: Int32, height h: Int32,
                   fg: UInt, bg: UInt, reverseBytes reverse: Bool) {
    var planes: [UnsafeMutablePointer<UInt8>?] = [nil, nil, nil, nil, nil]
    var bg_alpha: UInt8 = 0xff
    var bg = bg

    self.init(size: NSSize(width: CGFloat(w), height: CGFloat(h)))

    bmRep = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: Int(w), pixelsHigh: Int(h),
                             bitsPerSample: 8, samplesPerPixel: 4, hasAlpha: true,
                             isPlanar: true, colorSpaceName: COLORSPACE_NAME,
                             bytesPerRow: Int(w), bitsPerPixel: 0)

    bmRep?.getBitmapDataPlanes(&planes)

    if fg == 0 && bg == 0 {
      bg = 0xffffff
      bg_alpha = 0
    }

    do {
      /* Pull bits out to set the (bytewise) alpha mask.  */
      let swt: [UInt8] = [0, 8, 4, 12, 2, 10, 6, 14,
                          1, 9, 5, 13, 3, 11, 7, 15]
      var i: Int32
      var j: Int32
      var k: Int32
      var s = bits
      var rr = planes[0]!
      var gg = planes[1]!
      var bb = planes[2]!
      var alpha = planes[3]!
      let fgr = UInt8((fg >> 16) & 0xff)
      let fgg = UInt8((fg >> 8) & 0xff)
      let fgb = UInt8(fg & 0xff)
      let bgr = UInt8((bg >> 16) & 0xff)
      let bgg = UInt8((bg >> 8) & 0xff)
      let bgb = UInt8(bg & 0xff)
      var c: UInt8

      j = 0
      while j < h {
        i = 0
        while i < w {
          c = s.pointee
          s += 1

          /* XBM files have the bits in reverse order within each byte
             as compared to our fringe bitmaps.  This function deals
             with both so has to be able to handle the bytes in either
             order.  */
          if reverse {
            c = swt[Int(c >> 4)] | (swt[Int(c & 0xf)] << 4)
          }

          k = 0
          while i < w && k < 8 {
            if c & 0x80 != 0 {
              rr.pointee = fgr
              gg.pointee = fgg
              bb.pointee = fgb
              alpha.pointee = 0xff
            } else {
              rr.pointee = bgr
              gg.pointee = bgg
              bb.pointee = bgb
              alpha.pointee = bg_alpha
            }
            rr += 1
            gg += 1
            bb += 1
            alpha += 1
            c <<= 1
            k += 1
            i += 1
          }
        }
        j += 1
      }
    }

    addRepresentation(bmRep!)
  }

  convenience init(forXPMWithDepth depth: Int32, width: Int32, height: Int32) {
    let s = NSSize(width: CGFloat(width), height: CGFloat(height))
    var i: Int

    self.init(size: s)

    bmRep = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: Int(width),
                             pixelsHigh: Int(height),
                             /* keep things simple for now */
                             bitsPerSample: 8, samplesPerPixel: 4 /*RGB+A*/,
                             hasAlpha: true, isPlanar: true,
                             colorSpaceName: COLORSPACE_NAME,
                             bytesPerRow: Int(width), bitsPerPixel: 0)

    bmRep?.getBitmapDataPlanes(&pixmapData)
    i = 0
    while i < 4 {
      memset(pixmapData[i].map { UnsafeMutableRawPointer($0) }, 0, Int(width) * Int(height))
      i += 1
    }
    addRepresentation(bmRep!)
  }

  /* Attempt to pull out pixmap data from a BitmapImageRep; returns NO if fails.  */
  func setPixmapData() {
    for rep in representations {
      if rep.responds(to: #selector(NSBitmapImageRep.getBitmapDataPlanes(_:))) {
        let bmr = unsafeDowncast(rep, to: NSBitmapImageRep.self)

        if bmr.numberOfPlanes >= 3 {
          bmr.getBitmapDataPlanes(&pixmapData)
        }

        size = NSSize(width: CGFloat(bmr.pixelsWide), height: CGFloat(bmr.pixelsHigh))

        break
      }
    }
  }

  /* Note: this and next work only for image created with initForXPMWithDepth,
           initFromSkipXBM, or where setPixmapData was called successfully.  */
  /* return ARGB */
  func getPixelAtX(_ x: Int32, Y y: Int32) -> UInt {
    if bmRep == nil {
      return 0
    }

    /* This method is faster but won't work for bitmaps.  */
    if pixmapData[0] != nil {
      // C's `int loc = x + y * [self size].width;` computes in CGFloat and truncates last.
      let loc = Int(CGFloat(x) + CGFloat(y) * size.width)
      return (UInt(pixmapData[3]![loc]) << 24) /* alpha */
        | (UInt(pixmapData[0]![loc]) << 16)
        | (UInt(pixmapData[1]![loc]) << 8)
        | UInt(pixmapData[2]![loc])
    } else {
      if let color = bmRep?.colorAt(x: Int(x), y: Int(y)) {
        var r: CGFloat = 0, g: CGFloat = 0, b: CGFloat = 0, a: CGFloat = 0
        color.getRed(&r, green: &g, blue: &b, alpha: &a)
        // The C expression is an `int` that the return converts to `unsigned long`, which
        // sign-extends; the port keeps that conversion.
        let argb: Int32 = (Int32(a * 255.0) << 24)
          | (Int32(r * 255.0) << 16) | (Int32(g * 255.0) << 8)
          | Int32(b * 255.0)
        return UInt(bitPattern: Int(argb))
      }
      return 0
    }
  }

  func setPixelAtX(_ x: Int32, Y y: Int32, toRed r: UInt8, green g: UInt8, blue b: UInt8,
                   alpha a: UInt8) {
    if bmRep == nil {
      return
    }

    if pixmapData[0] != nil {
      let loc = Int(CGFloat(x) + CGFloat(y) * size.width)
      pixmapData[0]![loc] = r
      pixmapData[1]![loc] = g
      pixmapData[2]![loc] = b
      pixmapData[3]![loc] = a
    } else {
      bmRep?.setColor(NSColor(calibratedRed: CGFloat(r) / 255.0, green: CGFloat(g) / 255.0,
                              blue: CGFloat(b) / 255.0, alpha: CGFloat(a) / 255.0),
                      atX: Int(x), y: Int(y))
    }
  }

  func setAlphaAtX(_ x: Int32, Y y: Int32, to a: UInt8) {
    if bmRep == nil {
      return
    }

    if pixmapData[0] != nil {
      let loc = Int(CGFloat(x) + CGFloat(y) * size.width)

      pixmapData[3]![loc] = a
    } else {
      if let color = bmRep?.colorAt(x: Int(x), y: Int(y)) {
        let color = color.withAlphaComponent(CGFloat(a) / 255.0)
        bmRep?.setColor(color, atX: Int(x), y: Int(y))
      }
    }
  }

  /* Returns a cached CGImageMask of the stipple pattern */
  func stippleMask() -> CGImage? {
    if cachedStippleMask == nil {
      let provider = CGDataProviderCreateWithData(nil, bmRep?.bitmapData.map { UnsafeRawPointer($0) },
                                                  sizeInBytes(), nil)
      let mask = CGImageMaskCreate(Int(size.width), Int(size.height), 8, 8, Int(size.width),
                                   provider, nil, false)

      // C caches with `stippleMask = CGImageRetain (mask)` and releases it in dealloc;
      // Swift's ARC owns Core Foundation objects, so the assignment is the whole of it.
      cachedStippleMask = mask
    }
    return cachedStippleMask
  }

  /* Find the first NSBitmapImageRep which has multiple frames.  */
  func getAnimatedBitmapImageRep() -> NSBitmapImageRep? {
    for r in representations {
      if r is NSBitmapImageRep {
        let bm = unsafeDowncast(r, to: NSBitmapImageRep.self)
        if (bm.value(forProperty: .frameCount) as? NSNumber)?.intValue ?? 0 > 0 {
          return bm
        }
      }
    }
    return nil
  }

  /* If the image has multiple frames, get a count of them and the
     animation delay, if available.  */
  func getMetadata() -> Lisp_Object {
    var metadata: Lisp_Object = Qnil()

    let bm = getAnimatedBitmapImageRep()

    if bm != nil {
      let frames = Int32((bm!.value(forProperty: .frameCount) as? NSNumber)?.intValue ?? 0)
      let delay = Float((bm!.value(forProperty: .currentFrameDuration) as? NSNumber)?.floatValue ?? 0)

      if frames > 1 {
        metadata = Fcons(Qcount(), Fcons(make_fixnum(Int(frames)), metadata))
      }
      if delay > 0 {
        metadata = Fcons(Qdelay(), Fcons(make_float(Double(delay)), metadata))
      }
    }
    return metadata
  }

  /* Attempt to set the animation frame to be displayed.  */
  func setFrame(_ index: UInt32) -> Bool {
    let bm = getAnimatedBitmapImageRep()

    if bm != nil {
      let frames = Int32((bm!.value(forProperty: .frameCount) as? NSNumber)?.intValue ?? 0)

      /* If index is invalid, give up.  */
      // C compares an `unsigned int` against a nonnegative `int`, so `index < 0` is dead
      // text there and `frames` converts to unsigned here; both spellings are the C ones.
      if index < 0 || index > UInt32(truncatingIfNeeded: frames) {
        return false
      }

      bm!.setProperty(.currentFrame, withValue: NSNumber(value: index))
    }

    /* Setting the frame has succeeded, or the image doesn't have
       multiple frames.  */
    return true
  }

  func setTransform(_ m: UnsafeMutablePointer<double3>) {
    transform = NSAffineTransform()
    let tm = NSAffineTransformStruct(m11: m[0].v0, m12: m[0].v1, m21: m[1].v0,
                                     m22: m[1].v1, tX: m[2].v0, tY: m[2].v1)
    transform?.transformStruct = tm
  }

  func setSmoothing(_ s: Bool) {
    smoothing = s
  }

  /* Approximate allocated size of image in bytes.  */
  func sizeInBytes() -> Int {
    var bytes = 0
    for rep in representations {
      if rep.responds(to: #selector(getter: NSBitmapImageRep.bytesPerRow)) {
        let bmr = unsafeDowncast(rep, to: NSBitmapImageRep.self)
        bytes += bmr.bytesPerRow * bmr.numberOfPlanes * bmr.pixelsHigh
      }
    }
    return bytes
  }
}

@_silgen_name("EmacsImage_allocInitFromFile")
func ns_image_allocInitFromFile(_ file: Lisp_Object) -> AnyObject? { EmacsImage.allocInitFromFile(file) }
@_cdecl("is_emacs_image")
func ns_image_is_emacs_image(_ image: AnyObject?) -> Bool { image is EmacsImage }

@_silgen_name("EmacsImage_size")
func export_EmacsImage_size(_ img: AnyObject?) -> NSSize { (img as? EmacsImage)?.size ?? .zero }
@_cdecl("EmacsImage_smoothing")
func export_EmacsImage_smoothing(_ img: AnyObject?) -> Bool { (img as? EmacsImage)?.smoothing ?? false }
@_silgen_name("EmacsImage_transform")
func export_EmacsImage_transform(_ img: AnyObject?) -> NSAffineTransform? { (img as? EmacsImage)?.transform }
@_silgen_name("EmacsImage_stippleMask")
func export_EmacsImage_stippleMask(_ img: AnyObject?) -> CGImage? { (img as? EmacsImage)?.stippleMask() }
@_silgen_name("EmacsImage_drawInRect")
func export_EmacsImage_drawInRect(_ img: AnyObject?, _ dstRect: NSRect, _ srcRect: NSRect,
                                _ operation: NSCompositingOperation, _ fraction: CGFloat,
                                _ respectFlipped: Bool, _ hints: NSDictionary?) {
  (img as? EmacsImage)?.draw(in: dstRect, from: srcRect, operation: operation, fraction: fraction,
                            respectFlipped: respectFlipped, hints: hints as? [NSImageRep.HintKey: Any])
}
