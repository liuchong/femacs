// femacs: port of src/nsselect.m @ fd7afced403.
//
// Swift has no `#include`, so this unit carries the declarations of its C environment (the
// three kinds the lane uses are listed in docs/ports/nsimage.md's `## ABI` and repeated
// here): `@_silgen_name` imports of the ported core, header-only constructs (macros, INLINE
// functions, generated-globals entries) whose C-ABI entry the core exports under the C
// construct's own name, and this file's own C interface exported with `@_cdecl`.
//
// Two things are specific to this unit:
//
//   * src/nsselect.m 397-825 defines six subrs.  C's `DEFUN` builds a file-static
//     `union Aligned_Lisp_Subr` in `__DATA,subrs` plus the `Lisp_Object F…` function
//     (src/lisp.h 3406-3440); the port keeps both halves, and `syms_of_nsselect` registers
//     the records with `defsubr` exactly as the reference does.
//   * It reads constructs that other files of *this lane* own (src/nsterm.h's
//     `FRAME_NS_VIEW`, `ns_send_types`/`ns_return_types`, `ns_release_object`, the
//     `EmacsWindow` drag method, and src/nsfns.m's `NSString` category methods).  A Swift
//     translation unit has no forward declaration for another file's items, so each one is
//     reached through a C-ABI entry named after the C construct; the file that ports its
//     definition owns it, and the ledger's `## ABI` section says which file that is.
//
// `#if NS_USE_NSPasteboardTypeFileURL != 0` (src/nsselect.m 71-79, 141-146, 615-625) cannot
// be spelled in Swift, whose `#if` tests definedness only; the lane's target is the Cocoa
// arm, for which src/nsterm.h 44-49 defines it as 1, so those are the arms compiled here and
// the `NSFilenamesPboardType` arms (with `NSURL`) are recorded in the ledger, not
// transcribed.  `DEFSYM` is empty in the reference (src/lisp.h 2394).

import Foundation
import AppKit

// ==========================================================================
// Lane ABI: C types.
// ==========================================================================

/// C: `typedef struct Lisp_X *Lisp_Word; typedef Lisp_Word Lisp_Object;` (src/lisp.h
/// 311-323).  The pointee is incomplete in C too and a Swift pointer to a Swift type cannot
/// be a `@_cdecl` parameter, so the lane keeps the pointer alone (docs/ports/nsimage.md).
typealias Lisp_Object = UnsafeMutableRawPointer?

/// C: `union vectorlike_header { ptrdiff_t size; }` (src/lisp.h 923-953) - one member.
struct vectorlike_header {
  var size: Int
}

/// C: `union { Lisp_Object (*a0) (void); … a8 … aUNEVALLED … aMANY … } function;` inside
/// `struct Lisp_Subr` (src/lisp.h 2177-2189): eleven function pointers, one slot wide.
/// Swift has no unions, so the port keeps the slot and the record initialiser stores the
/// pointer C's `{.a ## maxargs = fnname}` stores (the core's `defsubr`/`XSUBR` read it back
/// through the C type).
struct Lisp_Subr_function {
  var ptr: UnsafeMutableRawPointer
}

/// C: `struct Lisp_Subr` (src/lisp.h 2174-2207).  The four `HAVE_NATIVE_COMP` fields
/// (2201-2206) are behind the reference's own `#ifdef`, which the lane mirrors as Swift
/// conditional compilation; without them C's initialiser stops at `doc` and the aggregate
/// rule zero-fills the rest.
struct Lisp_Subr {
  var header: vectorlike_header
  var function: Lisp_Subr_function
  var min_args: Int16
  var max_args: Int16
  var symbol_name: UnsafePointer<CChar>?
  /// C: `union { const char *string; Lisp_Object native; } intspec;` (2192-2195) - the
  /// initialiser sets `.string`.
  var intspec: UnsafePointer<CChar>?
  var command_modes: Lisp_Object
  var doc: Int
  #if HAVE_NATIVE_COMP
  // C's initialiser stops at `command_modes`/`doc`, so these four are zero-filled by the
  // aggregate rule (lisp.h 2201-2206); the port says so with default values, which is what
  // the Rust lane spells out in its `DEFUN!`.
  var native_comp_u: Lisp_Object = Qnil()
  var native_c_name: UnsafeMutablePointer<CChar>? = nil
  var lambda_list: Lisp_Object = Qnil()
  var type: Lisp_Object = Qnil()
  #endif
}

/// C: `union Aligned_Lisp_Subr { struct Lisp_Subr s; }` (src/lisp.h 2208-2212), `GCALIGNED`.
struct Aligned_Lisp_Subr {
  var s: Lisp_Subr
}

/// C: `typedef struct { ptrdiff_t bytes; } specpdl_ref;` (src/lisp.h 3646, the
/// `WRAP_SPECPDL_REF` arm the 64-bit target takes).
struct specpdl_ref {
  var bytes: Int
}

/// C: `struct Lisp_Fwd { ENUM_BF (Lisp_Fwd_Type) type : 8; union { … } u; }` (src/lisp.h
/// 3118-3133).  The bitfield is one byte and the union is one slot; the port keeps the
/// member this file sets (`objvar`) and names the field after C's.
struct Lisp_Fwd_u {
  var objvar: UnsafeMutablePointer<Lisp_Object>?
}
struct Lisp_Fwd {
  var type: UInt8
  var u: Lisp_Fwd_u
}

/// C: `enum ns_return_frame_mode { RETURN_FRAME_NEVER, RETURN_FRAME_EVENTUALLY,
/// RETURN_FRAME_NOW }` (src/nsterm.h 411-416).
enum ns_return_frame_mode: Int32 {
  case RETURN_FRAME_NEVER = 0
  case RETURN_FRAME_EVENTUALLY = 1
  case RETURN_FRAME_NOW = 2
}

// ==========================================================================
// Lane ABI: the C side.
// ==========================================================================

// Functions of the ported core.
@_silgen_name("defsubr") func defsubr(_ sname: UnsafeMutablePointer<Aligned_Lisp_Subr>?)
/// C: `void staticpro (Lisp_Object const *)` (src/lisp.h 3919).
@_silgen_name("staticpro") func staticpro(_ varaddress: UnsafePointer<Lisp_Object>?)
/// C: `extern void defvar_lisp (struct Lisp_Fwd const *, char const *)` (src/lisp.h 3473).
@_silgen_name("defvar_lisp") func defvar_lisp(_ o_fwd: UnsafePointer<Lisp_Fwd>?, _ namestring: UnsafePointer<CChar>?, docstring: StaticString)
@_silgen_name("Fcons") func Fcons(_ car: Lisp_Object, _ cdr: Lisp_Object) -> Lisp_Object
@_silgen_name("Fcar") func Fcar(_ list: Lisp_Object) -> Lisp_Object
@_silgen_name("Fcdr") func Fcdr(_ list: Lisp_Object) -> Lisp_Object
@_silgen_name("Fsetcdr") func Fsetcdr(_ cons: Lisp_Object, _ value: Lisp_Object) -> Lisp_Object
@_silgen_name("Fmake_vector") func Fmake_vector(_ length: Lisp_Object, _ init: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object Ffuncall (ptrdiff_t, Lisp_Object *)` (globals.h 4824, `EXFUN (Ffuncall, MANY)`).
@_silgen_name("Ffuncall") func Ffuncall(_ nargs: Int, _ args: UnsafeMutablePointer<Lisp_Object>?) -> Lisp_Object
/// C: `Lisp_Object make_string (const char *, ptrdiff_t)` (src/lisp.h 4541).
@_silgen_name("make_string") func make_string(_ str: UnsafePointer<CChar>?, _ length: Int) -> Lisp_Object
/// C: `extern Lisp_Object list2 (Lisp_Object, Lisp_Object)` (src/lisp.h 4487).
@_silgen_name("list2") func list2(_ a: Lisp_Object, _ b: Lisp_Object) -> Lisp_Object
/// C: `extern Lisp_Object assq_no_quit (Lisp_Object, Lisp_Object)` (src/lisp.h 4272).
@_silgen_name("assq_no_quit") func assq_no_quit(_ key: Lisp_Object, _ list: Lisp_Object) -> Lisp_Object
/// C: `extern AVOID error (const char *, ...)` (src/lisp.h 4835) - `AVOID` is `_Noreturn`
/// (src/lisp.h 153).  Its variadic arguments are Lisp_Objects, like
/// `add_to_log`'s (src/lisp.h 4378).
@_silgen_name("error") func error(_ format: UnsafePointer<CChar>, _ args: CVarArg...) -> Never
/// C: `extern AVOID signal_error (const char *, Lisp_Object)` (src/lisp.h 4796).
@_silgen_name("signal_error") func signal_error(_ s: UnsafePointer<CChar>, _ data: Lisp_Object) -> Never
/// C: `extern WINDOW_SYSTEM_RETURN void check_window_system (struct frame *)` (src/frame.h 1504).
@_silgen_name("check_window_system") func check_window_system(_ f: UnsafeMutableRawPointer?)
/// C: `extern struct frame *decode_window_system_frame (Lisp_Object)` (src/frame.h 1475).
@_silgen_name("decode_window_system_frame") func decode_window_system_frame(_ frame: Lisp_Object) -> UnsafeMutableRawPointer?
/// C: `extern void record_unwind_protect_ptr (void (*) (void *), void *)` (src/lisp.h 4820).
@_silgen_name("record_unwind_protect_ptr")
func record_unwind_protect_ptr(_ fn: @convention(c) (UnsafeMutableRawPointer?) -> Void, _ arg: UnsafeMutableRawPointer?)
/// C: `extern Lisp_Object unbind_to (specpdl_ref, Lisp_Object)` (src/lisp.h 4833).
@_silgen_name("unbind_to") func unbind_to(_ count: specpdl_ref, _ value: Lisp_Object) -> Lisp_Object

// Header-only constructs (macros, INLINE functions, enumerators, generated-globals entries);
// the C names are the constructs' own names, and the core side exports each as a C-ABI entry.
@_silgen_name("EQ") func EQ(_ x: Lisp_Object, _ y: Lisp_Object) -> Bool
@_silgen_name("NILP") func NILP(_ x: Lisp_Object) -> Bool
@_silgen_name("CONSP") func CONSP(_ x: Lisp_Object) -> Bool
@_silgen_name("FIXNUMP") func FIXNUMP(_ x: Lisp_Object) -> Bool
@_silgen_name("STRINGP") func STRINGP(_ x: Lisp_Object) -> Bool
@_silgen_name("SYMBOLP") func SYMBOLP(_ x: Lisp_Object) -> Bool
@_silgen_name("VECTORP") func VECTORP(_ x: Lisp_Object) -> Bool
@_silgen_name("XCAR") func XCAR(_ c: Lisp_Object) -> Lisp_Object
@_silgen_name("XCDR") func XCDR(_ c: Lisp_Object) -> Lisp_Object
@_silgen_name("XFIXNUM") func XFIXNUM(_ a: Lisp_Object) -> Int
@_silgen_name("ASIZE") func ASIZE(_ array: Lisp_Object) -> Int
@_silgen_name("AREF") func AREF(_ array: Lisp_Object, _ idx: Int) -> Lisp_Object
@_silgen_name("ASET") func ASET(_ array: Lisp_Object, _ idx: Int, _ val: Lisp_Object)
@_silgen_name("make_fixnum") func make_fixnum(_ n: Int) -> Lisp_Object
@_silgen_name("make_nil_vector") func make_nil_vector(_ size: Int) -> Lisp_Object
/// C: `INLINE Lisp_Object SYMBOL_NAME (Lisp_Object sym)` (src/lisp.h 2350).
@_silgen_name("SYMBOL_NAME") func SYMBOL_NAME(_ sym: Lisp_Object) -> Lisp_Object
@_silgen_name("CHECK_SYMBOL") func CHECK_SYMBOL(_ x: Lisp_Object)
@_silgen_name("CHECK_STRING") func CHECK_STRING(_ x: Lisp_Object)
@_silgen_name("CHECK_LIST") func CHECK_LIST(_ x: Lisp_Object)
@_silgen_name("CHECK_LIST_END") func CHECK_LIST_END(_ x: Lisp_Object, _ y: Lisp_Object)
/// C: `INLINE Lisp_Object intern (const char *str)` (src/lisp.h 4757).
@_silgen_name("intern") func intern(_ str: UnsafePointer<CChar>) -> Lisp_Object
@_silgen_name("SPECPDL_INDEX") func SPECPDL_INDEX() -> specpdl_ref
/// C: `INLINE bool window_system_available (struct frame *f)` (src/frame.h 1495).
@_silgen_name("window_system_available") func window_system_available(_ f: UnsafeMutableRawPointer?) -> Bool
/// C: `INLINE void maybe_quit (void)` (src/lisp.h 3896).
@_silgen_name("maybe_quit") func maybe_quit()
/// C: `#define XSETFRAME(a, b) (XSETPSEUDOVECTOR (a, b, PVEC_FRAME))` (src/frame.h) - the
/// macro assigns to its first argument, so the entry takes its address.
@_silgen_name("XSETFRAME") func XSETFRAME(_ a: UnsafeMutablePointer<Lisp_Object>, _ b: UnsafeMutableRawPointer?)
/// C: `PVEC_SUBR` (src/lisp.h 999) and
/// `PSEUDOVECTOR_AREA_BITS` (src/lisp.h 1039) - the two enumerators of the subr header word.
@_silgen_name("PVEC_SUBR") func PVEC_SUBR() -> Int
@_silgen_name("PSEUDOVECTOR_AREA_BITS") func PSEUDOVECTOR_AREA_BITS() -> Int
/// C: `Lisp_Fwd_Obj` (src/lisp.h 528-532), the `enum Lisp_Fwd_Type` member `DEFVAR_LISP` stores.
@_silgen_name("Lisp_Fwd_Obj") func Lisp_Fwd_Obj() -> UInt8
/// C: `#define Qfoo builtin_lisp_symbol (...)`, one per symbol, generated into globals.h
/// (PORTING.md P-17); called as `Qfoo ()`, the Rust mirror's shape.
@_silgen_name("Qnil") func Qnil() -> Lisp_Object
@_silgen_name("Qt") func Qt() -> Lisp_Object
@_silgen_name("Qunbound") func Qunbound() -> Lisp_Object
@_silgen_name("Qstring") func Qstring() -> Lisp_Object
@_silgen_name("Qfile") func Qfile() -> Lisp_Object
@_silgen_name("QCLIPBOARD") func QCLIPBOARD() -> Lisp_Object
@_silgen_name("QPRIMARY") func QPRIMARY() -> Lisp_Object
@_silgen_name("QSECONDARY") func QSECONDARY() -> Lisp_Object
@_silgen_name("QTEXT") func QTEXT() -> Lisp_Object
@_silgen_name("QFILE_NAME") func QFILE_NAME() -> Lisp_Object
@_silgen_name("QTARGETS") func QTARGETS() -> Lisp_Object
@_silgen_name("QXdndActionCopy") func QXdndActionCopy() -> Lisp_Object
@_silgen_name("QXdndActionMove") func QXdndActionMove() -> Lisp_Object
@_silgen_name("QXdndActionLink") func QXdndActionLink() -> Lisp_Object
@_silgen_name("QXdndActionPrivate") func QXdndActionPrivate() -> Lisp_Object
@_silgen_name("Qnow") func Qnow() -> Lisp_Object

// Constructs of this lane's own headers that other lane files define.  The definition owner
// is named on each; the ledger's `## ABI` lists them together.
/// C: `extern NSArray *ns_send_types, *ns_return_types;` (src/nsterm.h 774), defined in
/// src/nsterm.m 300.  Swift cannot export a C data symbol (only `@_cdecl` functions), so a
/// lane variable that another lane file reads is reached as a C-ABI function of its name.
@_silgen_name("ns_send_types") var ns_send_types: NSArray?
@_silgen_name("ns_return_types") var ns_return_types: NSArray?
/// C: `extern void ns_release_object (void *obj)` (src/nsterm.h 1166, defined src/nsterm.m 602).
@_silgen_name("ns_release_object") func ns_release_object(_ obj: UnsafeMutableRawPointer?)
/// C: `#define FRAME_NS_VIEW(f) ((f)->output_data.ns->view)` (src/nsterm.h 1045) - the
/// frame's `EmacsView *` (an `NSView`), read through the owning file's mirror of
/// `struct frame`.
@_silgen_name("FRAME_NS_VIEW") func FRAME_NS_VIEW(_ f: UnsafeMutableRawPointer?) -> NSView?
/// C: `-[EmacsWindow beginDrag:forPasteboard:withMode:returnFrameTo:prohibitSame:followTooltip:]`
/// (declared src/nsterm.h 446-450, defined src/nsterm.m 10528).  A cross-file Objective-C
/// method is a C-ABI entry with the receiver first; the port of nsterm.m owns the definition.
@_silgen_name("beginDrag")
func beginDrag(_ window: UnsafeMutableRawPointer?, _ op: NSDragOperation, _ pasteboard: NSPasteboard?,
               _ mode: Int32, _ returnFrameTo: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
               _ prohibitSameFrame: Bool, _ followTooltip: Bool) -> NSDragOperation
/// C: `+[NSString stringWithLispString:]` (declared src/nsterm.h 367, defined src/nsfns.m 4104).
@_silgen_name("stringWithLispString") func stringWithLispString(_ string: Lisp_Object) -> NSString?
/// C: `-[NSString lispString]` (declared src/nsterm.h 368, defined src/nsfns.m 4143).
@_silgen_name("lispString") func lispString(_ string: NSString) -> Lisp_Object

// ==========================================================================
// Lane ABI: the C constructs the port spells itself.
// ==========================================================================

/// C: `#define eassert(cond) ((void) (false && (cond)))` (src/lisp.h 181) - the argument
/// must compile and is never evaluated.
@inline(__always)
func eassert(_ cond: @autoclosure () -> Bool) {
  _ = false && cond()
}

/// C: `error`/`add_to_log` read their variadic arguments with `va_arg (ap, Lisp_Object)` -
/// a pointer-sized value, 0 for `Qnil` (src/xdisp.c 12288-12300).  Swift's `CVarArg` has no
/// conformance for an Optional pointer, so the port passes the same bits as a `UInt`.
@inline(__always)
func lispVariadicArg(_ value: Lisp_Object) -> UInt {
  return value.map { UInt(bitPattern: UnsafeRawPointer($0)) } ?? 0
}

/// C: `#define calln(...) CALLN (Ffuncall, __VA_ARGS__)` with
/// `#define CALLN(f, ...) CALLMANY (f, ((Lisp_Object []) {__VA_ARGS__}))` and
/// `#define CALLMANY(f, array) (f) (countof (array), array)` (src/lisp.h 3455-3463): the
/// arguments become a compound literal and `Ffuncall` gets its element count.  Swift has no
/// variadic macros and no compound literals, so the call site passes C's argument list as an
/// array and the port calls the same `Ffuncall` (a real symbol) with that count.
@inline(__always)
func calln(_ args: [Lisp_Object]) -> Lisp_Object {
  var array = args
  return Ffuncall(array.count, &array)
}

/// C's pasteboard type lists are `NSArray *` of `NSString *` (`ns_send_types`, the type
/// lookup's keys, `[pb types]`); the SDK's Swift API takes `[NSPasteboard.PasteboardType]`,
/// so the lane converts once here - the same pasteboard type names either way.
@inline(__always)
fileprivate func pasteboardTypes(_ array: [Any]) -> [NSPasteboard.PasteboardType] {
  return array.compactMap { ($0 as? NSString).map { NSPasteboard.PasteboardType($0 as String) } }
}

/// C: `DEFUN`'s file-static record (src/lisp.h 3406-3440): the header word is
/// `PVEC_SUBR << PSEUDOVECTOR_AREA_BITS`, the function slot is C's
/// `{.a ## maxargs = fnname}` (one slot whichever arity the union member had), and the
/// initialiser stops at `command_modes`/`doc`.
fileprivate func make_aligned_subr(_ lname: StaticString, _ minargs: Int16, _ maxargs: Int16,
                                  _ function: UnsafeMutableRawPointer,
                                   docstring: StaticString, args: StaticString) -> Aligned_Lisp_Subr {
  return Aligned_Lisp_Subr(s: Lisp_Subr(
    header: vectorlike_header(size: PVEC_SUBR() << PSEUDOVECTOR_AREA_BITS()),
    function: Lisp_Subr_function(ptr: function),
    min_args: minargs,
    max_args: maxargs,
    symbol_name: UnsafeRawPointer(lname.utf8Start).assumingMemoryBound(to: CChar.self),
    intspec: nil,
    command_modes: Qnil(),
    doc: 0
  ))
}

// The `DEFUN` names as static storage: C's record points at the literal's bytes
// (src/lisp.h 3435), and a Swift string literal is not stable storage (the Rust lane's
// `concat!($lname, "\0").as_ptr ()`).
fileprivate let lname_ns_own_selection_internal: StaticString = "ns-own-selection-internal"
fileprivate let lname_ns_disown_selection_internal: StaticString = "ns-disown-selection-internal"
fileprivate let lname_ns_selection_exists_p: StaticString = "ns-selection-exists-p"
fileprivate let lname_ns_selection_owner_p: StaticString = "ns-selection-owner-p"
fileprivate let lname_ns_get_selection: StaticString = "ns-get-selection"
fileprivate let lname_ns_begin_drag: StaticString = "ns-begin-drag"
fileprivate let lname_ns_sent_selection_hooks: StaticString = "ns-sent-selection-hooks"

// ==========================================================================
// File-scope objects (src/nsselect.m 35-42).
// ==========================================================================

fileprivate var Vselection_alist: Lisp_Object = Qnil()

/// C: `NSPasteboardNameGeneral is pretty much analogous to X11 CLIPBOARD.  */
fileprivate var NXPrimaryPboard: NSString? = nil
fileprivate var NXSecondaryPboard: NSString? = nil

fileprivate var pasteboard_changecount: NSMutableDictionary? = nil

/// C: `Vns_sent_selection_hooks` is a field of the generated `globals` record (the
/// `DEFVAR_LISP` macro's target, src/lisp.h 3495-3500); the port keeps the storage in the
/// unit that owns it (docs/status.md 3.29), here as a Swift global, because Swift cannot
/// emit a C-named data symbol.
@_silgen_name("Vns_sent_selection_hooks") var Vns_sent_selection_hooks: Lisp_Object

/// C: `DEFVAR_LISP`'s function-local `static struct Lisp_Fwd` (src/lisp.h 3500-3512); Swift
/// has no function-local statics, so the record is a fileprivate global, and `syms_of_nsselect`
/// fills it in and hands it to `defvar_lisp`.
@_silgen_name("femacs_fwd_Vns_sent_selection_hooks") var O_FWD_Vns_sent_selection_hooks: Lisp_Fwd

/* ==========================================================================

    Internal utility functions

   ========================================================================== */

fileprivate func symbol_to_nsstring(_ sym: Lisp_Object) -> NSString? {
  CHECK_SYMBOL(sym)
  if EQ(sym, QCLIPBOARD()) { return NSPasteboard.Name.general as NSString }
  if EQ(sym, QPRIMARY()) { return NXPrimaryPboard }
  if EQ(sym, QSECONDARY()) { return NXSecondaryPboard }
  if EQ(sym, QTEXT()) { return NSPasteboard.PasteboardType.string as NSString }
  return stringWithLispString(SYMBOL_NAME(sym))
}

fileprivate func ns_symbol_to_pb(_ symbol: Lisp_Object) -> NSPasteboard? {
  return symbol_to_nsstring(symbol).map { NSPasteboard(name: NSPasteboard.Name($0 as String)) }
}

fileprivate func ns_string_to_symbol(_ t: NSString) -> Lisp_Object {
  if t.isEqual(to: NSPasteboard.Name.general.rawValue) { return QCLIPBOARD() }
  if NXPrimaryPboard != nil && t.isEqual(to: NXPrimaryPboard! as String) { return QPRIMARY() }
  if NXSecondaryPboard != nil && t.isEqual(to: NXSecondaryPboard! as String) { return QSECONDARY() }
  if t.isEqual(to: NSPasteboard.PasteboardType.string.rawValue) { return QTEXT() }
  if t.isEqual(to: NSPasteboard.PasteboardType.fileURL.rawValue) { return QFILE_NAME() }
  if t.isEqual(to: NSPasteboard.PasteboardType.tabularText.rawValue) { return QTEXT() }
  return intern(t.utf8String!)
}

fileprivate func clean_local_selection_data(_ obj: Lisp_Object) -> Lisp_Object {
  var obj = obj

  if CONSP(obj)
      && FIXNUMP(XCAR(obj))
      && CONSP(XCDR(obj))
      && FIXNUMP(XCAR(XCDR(obj)))
      && NILP(XCDR(XCDR(obj))) {
    obj = Fcons(XCAR(obj), XCDR(obj))
  }

  if CONSP(obj)
      && FIXNUMP(XCAR(obj))
      && FIXNUMP(XCDR(obj)) {
    if XFIXNUM(XCAR(obj)) == 0 { return XCDR(obj) }
    if XFIXNUM(XCAR(obj)) == -1 { return make_fixnum(-XFIXNUM(XCDR(obj))) }
  }

  if VECTORP(obj) {
    var i: Int
    let size = ASIZE(obj)
    var copy: Lisp_Object

    if size == 1 { return clean_local_selection_data(AREF(obj, 0)) }
    copy = make_nil_vector(size)
    i = 0
    while i < size {
      ASET(copy, i, clean_local_selection_data(AREF(obj, i)))
      i += 1
    }
    return copy
  }

  return obj
}

fileprivate func ns_declare_pasteboard(_ pb: NSPasteboard?) {
  pb?.declareTypes(pasteboardTypes((ns_send_types as? [Any]) ?? []), owner: NSApp)
}

fileprivate func ns_undeclare_pasteboard(_ pb: NSPasteboard?) {
  pb?.declareTypes([], owner: nil)
}

fileprivate func ns_store_pb_change_count(_ pb: NSPasteboard?) {
  pasteboard_changecount?.setObject(NSNumber(value: pb!.changeCount),
                                    forKey: pb!.name.rawValue as NSString)
}

fileprivate func ns_get_pb_change_count(_ selection: Lisp_Object) -> Int {
  let pb = ns_symbol_to_pb(selection)
  return pb != nil ? pb!.changeCount : -1
}

fileprivate func ns_get_our_change_count_for(_ selection: Lisp_Object) -> Int {
  // `symbol_to_nsstring` returns the name of a symbol (a Lisp string) here, so the factory's
  // result is non-nil; C hands whatever it returns to `objectForKey:` unchecked.
  let num = pasteboard_changecount?.object(forKey: symbol_to_nsstring(selection)!) as? NSNumber
  return num != nil ? num!.intValue : -1
}

fileprivate func ns_string_to_pasteboard_internal(_ pb: NSPasteboard?, _ str: Lisp_Object,
                                                  _ gtype: NSString?) {
  if NILP(str) {
    pb?.declareTypes([], owner: nil)
  } else {
    let nsStr: NSString

    CHECK_STRING(str)

    nsStr = stringWithLispString(str)!
    // FIXME: Why those 2 different code paths?
    if gtype == nil {
      // Used for ns_string_to_pasteboard
      pb?.declareTypes(pasteboardTypes((ns_send_types as? [Any]) ?? []), owner: nil)
      // C enumerates `ns_send_types` with an NSEnumerator; the elements are pasteboard type
      // names, which the SDK's `setString (_:forType:)` takes as `NSPasteboard.PasteboardType`.
      for type in (ns_send_types as? [String]) ?? [] {
        pb?.setString(nsStr as String, forType: NSPasteboard.PasteboardType(type))
      }
    } else {
      // Used for ns-own-selection-internal.
      eassert(gtype == (NSPasteboard.PasteboardType.string as NSString))
      pb?.setString(nsStr as String, forType: NSPasteboard.PasteboardType(gtype! as String))
    }
    ns_store_pb_change_count(pb)
  }
}

@_cdecl("ns_get_local_selection")
func ns_get_local_selection(_ selection_name: Lisp_Object, _ target_type: Lisp_Object) -> Lisp_Object {
  var local_value: Lisp_Object
  /* `yank-media' uses this combination to figure out what the
     available types of the selection are.  Return nil here so that
     ns_get_foreign_selection ends up being called to do that
     (Bug#71377).  */
  if EQ(selection_name, QCLIPBOARD())
      && EQ(target_type, QTARGETS()) {
    return Qnil()
  }

  local_value = assq_no_quit(selection_name, Vselection_alist)
  return local_value
}

fileprivate func ns_get_foreign_selection(_ symbol: Lisp_Object, _ target: Lisp_Object) -> Lisp_Object {
  var typeLookup: NSDictionary
  var pb: NSPasteboard?

  pb = ns_symbol_to_pb(symbol)

  /* Dictionary for looking up NS types from MIME types, and vice versa.  */
  typeLookup = [
    NSPasteboard.PasteboardType.URL: "text/plain",
    NSPasteboard.PasteboardType.fileURL: "text/plain",
    NSPasteboard.PasteboardType.multipleTextSelection: "text/plain",
    NSPasteboard.PasteboardType.png: "image/png",
    NSPasteboard.PasteboardType.html: "text/html",
    NSPasteboard.PasteboardType.pdf: "application/pdf",
    NSPasteboard.PasteboardType.rtf: "application/rtf",
    NSPasteboard.PasteboardType.rtfd: "application/rtfd",
    NSPasteboard.PasteboardType.string: "STRING",
    NSPasteboard.PasteboardType.tabularText: "text/plain",
    NSPasteboard.PasteboardType.tiff: "image/tiff",
  ] as NSDictionary

  if EQ(target, QTARGETS()) {
    let types: NSMutableArray = NSMutableArray(capacity: 3)

    for type in pb?.types ?? [] {
      if let val = typeLookup.value(forKey: type.rawValue) as? NSString {
        if !types.contains(val) {
          types.add(val)
        }
      }
    }

    let v = Fmake_vector(make_fixnum(types.count + 1), Qnil())
    ASET(v, 0, QTARGETS())

    var i: Int = 0
    while i < types.count {
      ASET(v, i + 1, intern(((types.object(at: i)) as! NSString).utf8String!))
      i += 1
    }

    return v
  } else {
    var d: Data?
    var availableTypes: [NSPasteboard.PasteboardType]
    var result: NSString?
    var t: NSPasteboard.PasteboardType?

    if !NILP(target) {
      availableTypes = pasteboardTypes(typeLookup.allKeys(for: stringWithLispString(SYMBOL_NAME(target))!))
    } else {
      availableTypes = [NSPasteboard.PasteboardType.string]
    }

    t = pb?.availableType(from: availableTypes)

    result = t.flatMap { pb?.string(forType: $0) }.map { $0 as NSString }
    if result != nil {
      return lispString(result!)
    }

    d = t.flatMap { pb?.data(forType: $0) }
    return d.map { data in
      data.withUnsafeBytes { buf in
        make_string(buf.baseAddress?.assumingMemoryBound(to: CChar.self), buf.count)
      }
    } ?? make_string(nil, 0)
  }
}

/* ==========================================================================

    Functions used externally

   ========================================================================== */

@_cdecl("ns_string_from_pasteboard")
func ns_string_from_pasteboard(_ pb: NSPasteboard?) -> Lisp_Object {
  var type: NSPasteboard.PasteboardType?
  var str: NSString?

  type = pb?.availableType(from: pasteboardTypes((ns_return_types as? [Any]) ?? []))
  if type == nil {
    return Qnil()
  }

  /* get the string */
  str = pb?.string(forType: type!).map { $0 as NSString }
  if str == nil {
    let data = pb?.data(forType: type!)
    if data != nil {
      str = NSString(data: data!, encoding: String.Encoding.utf8.rawValue)
    }
    if str == nil {
      return Qnil()
    }
  }

  /* FIXME: Is the below EOL conversion even needed?  I've removed it
     for now so we can see if it causes problems.  */
  return lispString(str!)
}

@_cdecl("ns_string_to_pasteboard")
func ns_string_to_pasteboard(_ pb: NSPasteboard?, _ str: Lisp_Object) {
  ns_string_to_pasteboard_internal(pb, str, nil)
}

/* ==========================================================================

    Lisp Defuns

   ========================================================================== */

/* C: DEFUN ("ns-own-selection-internal", Fns_own_selection_internal,
       Sns_own_selection_internal, 2, 2, 0,
       doc: /* Assert an X selection of type SELECTION and value VALUE.
SELECTION is a symbol, typically `PRIMARY', `SECONDARY', or `CLIPBOARD'.
\(Those are literal upper-case symbol names, since that's what X expects.)
VALUE is typically a string, or a cons of two markers, but may be
anything that the functions on `selection-converter-alist' know about.  */) */
fileprivate var Sns_own_selection_internal: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_own_selection_internal, 2, 2,
                    unsafeBitCast(Fns_own_selection_internal as @convention(c) (Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Assert an X selection of type SELECTION and value VALUE.\nSELECTION is a symbol, typically `PRIMARY', `SECONDARY', or `CLIPBOARD'.\n(Those are literal upper-case symbol names, since that's what X expects.)\nVALUE is typically a string, or a cons of two markers, but may be\nanything that the functions on `selection-converter-alist' know about.  ", args: "selection value")

@_cdecl("Fns_own_selection_internal")
func Fns_own_selection_internal(_ selection: Lisp_Object, _ value: Lisp_Object) -> Lisp_Object {
  var pb: NSPasteboard?
  var type: NSString?
  var successful_p: Lisp_Object = Qnil()
  var rest: Lisp_Object
  var target_symbol: Lisp_Object

  check_window_system(nil)
  CHECK_SYMBOL(selection)
  if NILP(value) {
    error("Selection value may not be nil")
  }
  pb = ns_symbol_to_pb(selection)
  if pb == nil { return Qnil() }

  ns_declare_pasteboard(pb)
  do {
    let old_value = assq_no_quit(selection, Vselection_alist)
    let new_value = list2(selection, value)

    if NILP(old_value) {
      Vselection_alist = Fcons(new_value, Vselection_alist)
    } else {
      _ = Fsetcdr(old_value, Fcdr(new_value))
    }
  }

  /* We only support copy of text.  */
  type = NSPasteboard.PasteboardType.string as NSString
  target_symbol = ns_string_to_symbol(type!)
  if STRINGP(value) {
    ns_string_to_pasteboard_internal(pb, value, type)
    successful_p = Qt()
  }

  if !EQ(Vns_sent_selection_hooks, Qunbound()) {
    /* FIXME: Use run-hook-with-args!  */
    rest = Vns_sent_selection_hooks
    while CONSP(rest) {
      _ = calln([Fcar(rest), selection, target_symbol, successful_p])
      rest = Fcdr(rest)
    }
  }

  return value
}

/* C: DEFUN ("ns-disown-selection-internal", Fns_disown_selection_internal,
       Sns_disown_selection_internal, 1, 1, 0,
       doc: /* If we own the selection SELECTION, disown it.
Disowning it means there is no such selection.  */) */
fileprivate var Sns_disown_selection_internal: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_disown_selection_internal, 1, 1,
                    unsafeBitCast(Fns_disown_selection_internal as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " If we own the selection SELECTION, disown it.\nDisowning it means there is no such selection.  ", args: "selection")

@_cdecl("Fns_disown_selection_internal")
func Fns_disown_selection_internal(_ selection: Lisp_Object) -> Lisp_Object {
  var pb: NSPasteboard?
  check_window_system(nil)
  CHECK_SYMBOL(selection)

  if ns_get_pb_change_count(selection) != ns_get_our_change_count_for(selection) {
    return Qnil()
  }

  pb = ns_symbol_to_pb(selection)
  if pb != nil { ns_undeclare_pasteboard(pb) }
  return Qt()
}

/* C: DEFUN ("ns-selection-exists-p", Fns_selection_exists_p, Sns_selection_exists_p,
       0, 1, 0, doc: /* Whether there is an owner for the given X selection.
SELECTION should be the name of the selection in question, typically
one of the symbols `PRIMARY', `SECONDARY', or `CLIPBOARD'.  (X expects
these literal upper-case names.)  The symbol nil is the same as
`PRIMARY', and t is the same as `SECONDARY'.  */) */
fileprivate var Sns_selection_exists_p: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_selection_exists_p, 0, 1,
                    unsafeBitCast(Fns_selection_exists_p as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Whether there is an owner for the given X selection.\nSELECTION should be the name of the selection in question, typically\none of the symbols `PRIMARY', `SECONDARY', or `CLIPBOARD'.  (X expects\nthese literal upper-case names.)  The symbol nil is the same as\n`PRIMARY', and t is the same as `SECONDARY'.  ", args: "selection")

@_cdecl("Fns_selection_exists_p")
func Fns_selection_exists_p(_ selection: Lisp_Object) -> Lisp_Object {
  var selection = selection
  var pb: NSPasteboard?
  var types: NSArray

  if !window_system_available(nil) {
    return Qnil()
  }

  CHECK_SYMBOL(selection)
  if NILP(selection) { selection = QPRIMARY() }
  if EQ(selection, Qt()) { selection = QSECONDARY() }
  pb = ns_symbol_to_pb(selection)
  if pb == nil { return Qnil() }

  types = (pb?.types ?? []) as NSArray
  return (types.count == 0) ? Qnil() : Qt()
}

/* C: DEFUN ("ns-selection-owner-p", Fns_selection_owner_p, Sns_selection_owner_p,
       0, 1, 0,
       doc: /* Whether the current Emacs process owns the given X Selection.
The arg should be the name of the selection in question, typically one of
the symbols `PRIMARY', `SECONDARY', or `CLIPBOARD'.
\(Those are literal upper-case symbol names, since that's what X expects.)
For convenience, the symbol nil is the same as `PRIMARY',
and t is the same as `SECONDARY'.  */) */
fileprivate var Sns_selection_owner_p: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_selection_owner_p, 0, 1,
                    unsafeBitCast(Fns_selection_owner_p as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Whether the current Emacs process owns the given X Selection.\nThe arg should be the name of the selection in question, typically one of\nthe symbols `PRIMARY', `SECONDARY', or `CLIPBOARD'.\n(Those are literal upper-case symbol names, since that's what X expects.)\nFor convenience, the symbol nil is the same as `PRIMARY',\nand t is the same as `SECONDARY'.  ", args: "selection")

@_cdecl("Fns_selection_owner_p")
func Fns_selection_owner_p(_ selection: Lisp_Object) -> Lisp_Object {
  var selection = selection
  check_window_system(nil)
  CHECK_SYMBOL(selection)
  if NILP(selection) { selection = QPRIMARY() }
  if EQ(selection, Qt()) { selection = QSECONDARY() }
  return ns_get_pb_change_count(selection) == ns_get_our_change_count_for(selection) ? Qt() : Qnil()
}

/* C: DEFUN ("ns-get-selection", Fns_get_selection,
       Sns_get_selection, 2, 2, 0,
       doc: /* Return text selected from some X window.
SELECTION-SYMBOL is typically `PRIMARY', `SECONDARY', or `CLIPBOARD'.
\(Those are literal upper-case symbol names, since that's what X expects.)
TARGET-TYPE is the type of data desired, typically `STRING'.  */) */
fileprivate var Sns_get_selection: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_get_selection, 2, 2,
                    unsafeBitCast(Fns_get_selection as @convention(c) (Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Return text selected from some X window.\nSELECTION-SYMBOL is typically `PRIMARY', `SECONDARY', or `CLIPBOARD'.\n(Those are literal upper-case symbol names, since that's what X expects.)\nTARGET-TYPE is the type of data desired, typically `STRING'.  ", args: "selection_name target_type")

@_cdecl("Fns_get_selection")
func Fns_get_selection(_ selection_name: Lisp_Object, _ target_type: Lisp_Object) -> Lisp_Object {
  var val: Lisp_Object = Qnil()

  check_window_system(nil)
  CHECK_SYMBOL(selection_name)
  CHECK_SYMBOL(target_type)

  if ns_get_pb_change_count(selection_name) == ns_get_our_change_count_for(selection_name) {
    val = ns_get_local_selection(selection_name, target_type)
  }
  if NILP(val) {
    val = ns_get_foreign_selection(selection_name, target_type)
  }
  if CONSP(val) && SYMBOLP(Fcar(val)) {
    val = Fcdr(val)
    if CONSP(val) && NILP(Fcdr(val)) {
      val = Fcar(val)
    }
  }
  val = clean_local_selection_data(val)
  return val
}

/// C: `void nxatoms_of_nsselect (void)` (src/nsselect.m 545).
@_cdecl("nxatoms_of_nsselect")
func nxatoms_of_nsselect() {
  NXPrimaryPboard = "Selection"
  NXSecondaryPboard = "Secondary"

  // This is a memory loss, never released.
  pasteboard_changecount = NSMutableDictionary(dictionary: [
    NSPasteboard.Name.general.rawValue as NSString: NSNumber(value: 0),
    NXPrimaryPboard!: NSNumber(value: 0),
    NXSecondaryPboard!: NSNumber(value: 0),
    NSPasteboard.PasteboardType.string.rawValue as NSString: NSNumber(value: 0),
    NSPasteboard.PasteboardType.fileURL.rawValue as NSString: NSNumber(value: 0),
    NSPasteboard.PasteboardType.tabularText.rawValue as NSString: NSNumber(value: 0),
  ])
}

fileprivate func ns_decode_data_to_pasteboard(_ type: Lisp_Object, _ data: Lisp_Object,
                                              _ pasteboard: NSPasteboard?) {
  var types: [NSPasteboard.PasteboardType]
  var new: [NSPasteboard.PasteboardType]
  var temp: NSMutableArray?
  var tem: Lisp_Object
  let count: specpdl_ref

  types = pasteboard?.types ?? []
  count = SPECPDL_INDEX()

  CHECK_SYMBOL(type)

  if EQ(type, Qstring()) {
    CHECK_STRING(data)

    new = types + [NSPasteboard.PasteboardType.string]

    pasteboard?.declareTypes(new, owner: nil)
    // The value is a Lisp string here (checked above), so the factory's result is non-nil;
    // a nil would be what C's `[pb setString: nil forType:]` raises on.
    pasteboard?.setString(stringWithLispString(data)! as String, forType: NSPasteboard.PasteboardType.string)
  } else if EQ(type, Qfile()) {
    if CONSP(data) {
      new = types + [NSPasteboard.PasteboardType.URL]
    } else {
      new = types + [NSPasteboard.PasteboardType.fileURL]
    }

    pasteboard?.declareTypes(new, owner: nil)

    if STRINGP(data) {
      pasteboard?.setString(stringWithLispString(data)! as String,
                            forType: NSPasteboard.PasteboardType.fileURL)
    } else {
      CHECK_LIST(data)
      temp = NSMutableArray()
      record_unwind_protect_ptr(ns_release_object,
                               UnsafeMutableRawPointer(Unmanaged.passRetained(temp!).toOpaque()))

      tem = data
      while CONSP(tem) {
        CHECK_STRING(XCAR(tem))

        temp!.add(stringWithLispString(XCAR(tem))!)
        tem = XCDR(tem)
      }
      CHECK_LIST_END(tem, data)
      pasteboard?.setPropertyList(temp!, forType: NSPasteboard.PasteboardType("NSFilenamesPboardType"))
      _ = unbind_to(count, Qnil())
    }
  } else {
    signal_error("Unknown pasteboard type", type)
  }
}

fileprivate func ns_lisp_to_pasteboard(_ object: Lisp_Object, _ pasteboard: NSPasteboard?) {
  var tem: Lisp_Object
  var type: Lisp_Object
  var data: Lisp_Object

  pasteboard?.declareTypes([], owner: nil)

  CHECK_LIST(object)
  tem = object
  while CONSP(tem) {
    maybe_quit()

    type = Fcar(Fcar(tem))
    data = Fcdr(Fcar(tem))

    ns_decode_data_to_pasteboard(type, data, pasteboard)
    tem = XCDR(tem)
  }
  CHECK_LIST_END(tem, object)
}

fileprivate func ns_dnd_action_to_operation(_ action: Lisp_Object) -> NSDragOperation {
  if EQ(action, QXdndActionCopy()) {
    return NSDragOperation.copy
  }

  if EQ(action, QXdndActionMove()) {
    return NSDragOperation.move
  }

  if EQ(action, QXdndActionLink()) {
    return NSDragOperation.link
  }

  signal_error("Unsupported drag-and-drop action", action)
}

fileprivate func ns_dnd_action_from_operation(_ operation: NSDragOperation) -> Lisp_Object {
  switch operation {
  case NSDragOperation.copy:
    return QXdndActionCopy()

  case NSDragOperation.move:
    return QXdndActionMove()

  case NSDragOperation.link:
    return QXdndActionLink()

  case []:
    return Qnil()

  default:
    return QXdndActionPrivate()
  }
}

/* C: DEFUN ("ns-begin-drag", Fns_begin_drag, Sns_begin_drag, 3, 6, 0,
       doc: /* Begin a drag-and-drop operation on FRAME.

FRAME must be a window system frame.  PBOARD is an alist of (TYPE
. DATA), where TYPE is one of the following data types that determine
the meaning of DATA:

  - `string' means DATA should be a string describing text that will
    be dragged to another program.

  - `file' means DATA should be a file URL that will be dragged to
    another program.  DATA may also be a list of file names; that
    means each file in the list will be dragged to another program.

ACTION is the action that will be taken by the drop target towards the
data inside PBOARD.

Return the action that the drop target actually chose to perform, or
nil if no action was performed (either because there was no drop
target, or the drop was rejected).  If RETURN-FRAME is the symbol
`now', also return any frame that mouse moves into during the
drag-and-drop operation, whilst simultaneously canceling it.  Any
other non-nil value means to do the same, but to wait for the mouse to
leave FRAME first.

If ALLOW-SAME-FRAME is nil, dropping on FRAME will result in the drop
being ignored.

FOLLOW-TOOLTIP means the same thing it does in `x-begin-drag'.  */) */
fileprivate var Sns_begin_drag: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_begin_drag, 3, 6,
                    unsafeBitCast(Fns_begin_drag as @convention(c) (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Begin a drag-and-drop operation on FRAME.\n\nFRAME must be a window system frame.  PBOARD is an alist of (TYPE\n. DATA), where TYPE is one of the following data types that determine\nthe meaning of DATA:\n\n  - `string' means DATA should be a string describing text that will\n    be dragged to another program.\n\n  - `file' means DATA should be a file URL that will be dragged to\n    another program.  DATA may also be a list of file names; that\n    means each file in the list will be dragged to another program.\n\nACTION is the action that will be taken by the drop target towards the\ndata inside PBOARD.\n\nReturn the action that the drop target actually chose to perform, or\nnil if no action was performed (either because there was no drop\ntarget, or the drop was rejected).  If RETURN-FRAME is the symbol\n`now', also return any frame that mouse moves into during the\ndrag-and-drop operation, whilst simultaneously canceling it.  Any\nother non-nil value means to do the same, but to wait for the mouse to\nleave FRAME first.\n\nIf ALLOW-SAME-FRAME is nil, dropping on FRAME will result in the drop\nbeing ignored.\n\nFOLLOW-TOOLTIP means the same thing it does in `x-begin-drag'.  ", args: "frame pboard action return_frame allow_same_frame follow_tooltip")

@_cdecl("Fns_begin_drag")
func Fns_begin_drag(_ frame: Lisp_Object, _ pboard: Lisp_Object, _ action: Lisp_Object,
                    _ return_frame: Lisp_Object, _ allow_same_frame: Lisp_Object,
                    _ follow_tooltip: Lisp_Object) -> Lisp_Object {
  var f: UnsafeMutableRawPointer?
  var return_to: UnsafeMutableRawPointer?
  var pasteboard: NSPasteboard?
  var window: NSWindow?
  var operation: NSDragOperation
  var mode: ns_return_frame_mode
  var val: Lisp_Object = Qnil()

  if EQ(return_frame, Qnow()) {
    mode = .RETURN_FRAME_NOW
  } else if !NILP(return_frame) {
    mode = .RETURN_FRAME_EVENTUALLY
  } else {
    mode = .RETURN_FRAME_NEVER
  }

  if NILP(pboard) {
    signal_error("Empty pasteboard", pboard)
  }

  f = decode_window_system_frame(frame)
  pasteboard = NSPasteboard(name: .drag)
  // C: `window = (EmacsWindow *) [FRAME_NS_VIEW (f) window];`
  window = FRAME_NS_VIEW(f)?.window

  operation = ns_dnd_action_to_operation(action)
  ns_lisp_to_pasteboard(pboard, pasteboard)

  // C sends this message to `window`, which may be nil: a message to nil does nothing and
  // its reply is 0 (an empty `NSDragOperation`).  `return_to` is C's uninitialised local;
  // the port zero-initialises it (PORTING.md 0.1-13), so the branch below is only taken when
  // the method wrote it.
  if let window = window {
    operation = beginDrag(UnsafeMutableRawPointer(Unmanaged.passUnretained(window).toOpaque()),
                          operation,
                          pasteboard,
                          mode.rawValue,
                          &return_to,
                          Bool(NILP(allow_same_frame)),
                          Bool(!NILP(follow_tooltip)))
  } else {
    operation = []
  }

  if return_to != nil {
    XSETFRAME(&val, return_to)
    return val
  }

  return ns_dnd_action_from_operation(operation)
}

/// C: `void syms_of_nsselect (void)` (src/nsselect.m 788; declared src/nsterm.h 1199).
@_cdecl("syms_of_nsselect")
func syms_of_nsselect() {
  // C: `DEFSYM (QCLIPBOARD, "CLIPBOARD");` and the nine others that follow (lines 790-799):
  // `#define DEFSYM(sym, name) /* empty */` (src/lisp.h 2394) is only a marker for
  // make-docfile, so those call sites carry no statement; the symbols come from the
  // generated globals.

  defsubr(&Sns_disown_selection_internal)
  defsubr(&Sns_get_selection)
  defsubr(&Sns_own_selection_internal)
  defsubr(&Sns_selection_exists_p)
  defsubr(&Sns_selection_owner_p)
  defsubr(&Sns_begin_drag)

  Vselection_alist = Qnil()
  staticpro(&Vselection_alist)

  // C: `DEFVAR_LISP ("ns-sent-selection-hooks", Vns_sent_selection_hooks, "…")` (lines
  // 803-812): the macro builds its `struct Lisp_Fwd` with `Lisp_Fwd_Obj` and the variable's
  // address (src/lisp.h 3500-3512) and hands it to `defvar_lisp`; the docstring above is
  // the macro's third argument.
  O_FWD_Vns_sent_selection_hooks = Lisp_Fwd(
    type: Lisp_Fwd_Obj(), u: Lisp_Fwd_u(objvar: &Vns_sent_selection_hooks))
  defvar_lisp(&O_FWD_Vns_sent_selection_hooks, "ns-sent-selection-hooks", docstring: "A list of functions to be called when Emacs answers a selection request.\nThe functions are called with four arguments:\n  - the selection name (typically `PRIMARY', `SECONDARY', or `CLIPBOARD');\n  - the selection-type which Emacs was asked to convert the\n    selection into before sending (for example, `STRING' or `LENGTH');\n  - a flag indicating success or failure for responding to the request.\nWe might have failed (and declined the request) for any number of reasons,\nincluding being asked for a selection that we no longer own, or being asked\nto convert into a type that we don't know about or that is inappropriate.\nThis hook doesn't let you change the behavior of Emacs's selection replies,\nit merely informs you that they have happened.")
  Vns_sent_selection_hooks = Qnil()
}
