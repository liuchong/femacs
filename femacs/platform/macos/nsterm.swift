// femacs: port of src/nsterm.m @ fd7afced403.
//
// **Giant unit** (`docs/ports/nsterm.md`'s `## Progress`): the reference is 11,811 lines and is
// transcribed in C order, one slice per round.  **Round 1 landed C 60-390** - the file's first
// statics (C 82-88), the dead `NSTRACE_ENABLED` block (C 96-149), the `NSColor (EmacsColor)`
// category (C 152-206), the `convert_ns_to_X_keysym` table (C 214-293), the file-scope objects
// (C 295-347), the seven modifier masks (C 349-356) and the first two functions
// (`mod_of_kind` C 358, `ev_modifiers_helper` C 369).  **Round 2 lands C 391-519**: five of the
// seven `EV_*` accessors (C 397-444), the forward declarations (C 473-475, recorded - Swift has
// none), `ns_init_events`/`ns_finish_events` (C 484-495, the unit's C interface) and
// `hold_event`/`append2` (C 498-519).  `EV_TRAILER`/`EV_TRAILER2` (C 448-470) are **held** to
// C 5013, where their two callees exist (see the note at the end of this file).  The next slice
// starts at C 522 (`ns_relocate`).
//
// Swift has no `#include`, so the unit carries the declarations of its C environment.  The three
// kinds are the lane's (`docs/ports/nsimage.md`'s `## ABI`, `docs/ports/nsselect.md`'s): imports
// of the ported core and of the lane's header constructs with `@_silgen_name`, this unit's own C
// interface exported with `@_cdecl`, and the C constructs that have no symbol at all spelled out
// here.  `src/nsterm.h` already has a Rust mirror (`femacs/src/include/nsterm.rs`, status 3.199);
// this unit does not repeat it, it declares only what its own slice reads, each line carrying the
// reference's own `/// C:` text.
//
// The reference's platform arms: `NS_IMPL_COCOA` is the arm this lane compiles (the frozen
// configuration has no `HAVE_NS`/`NS_IMPL_COCOA`, so the whole `.m` is dead there -
// `docs/ports/nsterm.md`'s banner), and `NSTRACE_ENABLED` is **0** in `src/nsterm.h` 142-144, so
// C 92-143's trace block is dead by construction and recorded in the ledger, not transcribed.

import Foundation
import AppKit

// ==========================================================================
// Lane ABI: C types.
// ==========================================================================

/// C: `typedef struct Lisp_X *Lisp_Word; typedef Lisp_Word Lisp_Object;` (src/lisp.h 311-323).
/// The pointee is incomplete in C too, and a Swift pointer to a Swift type cannot cross
/// `@_cdecl`, so the lane keeps the pointer alone (docs/ports/nsimage.md).
typealias Lisp_Object = UnsafeMutableRawPointer?

/// C: `typedef CGFloat EmacsCGFloat;` (src/nsterm.h 36, the `NS_IMPL_COCOA` arm).
typealias EmacsCGFloat = CGFloat

/// C: `union vectorlike_header { ptrdiff_t size; }` (src/lisp.h 923-953) - one member, needed by
/// the records the later slices carry.
struct vectorlike_header {
  var size: Int
}

/// C: `struct input_event` (src/termhooks.h 371-412) - the record `emacs_event` points at, and
/// the one the `EV_*` accessors and `hold_event` fill.  It belongs to termhooks.h and has a
/// Rust mirror (`femacs/src/include/termhooks.rs`); a Swift unit cannot see a Rust record, so
/// this unit carries its own view of the same members, under the mirror's names.  The layout is
/// measured, not assumed - clang on the frozen configuration says `sizeof 64`, `align 8`, and
/// `code` 4, `modifiers` 8, `x` 16, `y` 24, `timestamp` 32, `frame_or_window` 40, `arg` 48,
/// `device` 56, with the two bitfields sharing the unit at 0 (Evidence).
struct input_event {
  /// C: `ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH;` and `ENUM_BF (scroll_bar_part)
  /// part : 16;` (src/termhooks.h 374 and 377; the width is 360's `EVENT_KIND_WIDTH = 16`) -
  /// `kind` is the low half, `part` the high half.
  var bits: UInt32
  /// C: `unsigned code;` (src/termhooks.h 383)
  var code: UInt32
  /// C: `unsigned modifiers;` (src/termhooks.h 386)
  var modifiers: UInt32
  /// C: `Lisp_Object x, y;` (src/termhooks.h 390)
  var x: Lisp_Object
  var y: Lisp_Object
  /// C: `timestamp` (src/termhooks.h 395), whose type `Time` is systime.h's `unsigned long`.
  var timestamp: UInt
  /// C: `Lisp_Object frame_or_window;` (src/termhooks.h 399)
  var frame_or_window: Lisp_Object
  /// C: `Lisp_Object arg;` (src/termhooks.h 404)
  var arg: Lisp_Object
  /// C: `Lisp_Object device;` (src/termhooks.h 411)
  var device: Lisp_Object
}

// ==========================================================================
// Lane ABI: C symbols this file calls (`@_silgen_name`).
// ==========================================================================

// Header-only constructs (macros and INLINE functions of src/lisp.h): the core exports a C-ABI
// entry under the construct's own name (docs/ports/nsselect.md (2)b).
/// C: `INLINE bool SYMBOLP (Lisp_Object)` (src/lisp.h 1581).
@_silgen_name("SYMBOLP") func SYMBOLP(_ x: Lisp_Object) -> Bool
/// C: `INLINE bool EQ (Lisp_Object, Lisp_Object)` (src/lisp.h 390-460 family).
@_silgen_name("EQ") func EQ(_ x: Lisp_Object, _ y: Lisp_Object) -> Bool

// Symbols the ported core defines out of line.
/// C: `Lisp_Object plist_get (Lisp_Object, Lisp_Object)` (src/lisp.h 4280; defined src/fns.c).
@_silgen_name("plist_get") func plist_get(_ plist: Lisp_Object, _ prop: Lisp_Object) -> Lisp_Object
/// C: `int parse_solitary_modifier (Lisp_Object symbol)` (src/keyboard.h 463; defined
/// src/keyboard.c 7920).  The port's `femacs/src/keymap.rs` mirror declares this one as returning
/// `Lisp_Object` (src/keyboard.h 465's neighbour in the generated globals), which disagrees with
/// both C's own declaration and definition; this unit takes C's `int` (flagged in the ledger).
@_silgen_name("parse_solitary_modifier") func parse_solitary_modifier(_ symbol: Lisp_Object) -> Int32
/// C: `ns_use_srgb_colorspace` - src/nsterm.m 11716's `DEFVAR_BOOL ("ns-use-srgb-colorspace",
/// …)`, which the generated globals.h spells as a macro over the `globals` record's field.
/// The lane reaches a C variable another unit must supply as a C-ABI entry of its own name,
/// and `@_silgen_name` on a `var` is an *import*; the landed `nsimage.swift` imports this same
/// symbol that way (line 145), so this unit matches it (see the ledger's Decisions/ABI).
@_silgen_name("ns_use_srgb_colorspace") var ns_use_srgb_colorspace: Bool

// The generated globals (globals.h `#define Qfoo builtin_lisp_symbol (iQfoo)`): the core exports
// one C-ABI function per name (docs/ports/nsselect.md (2)b).
/// C: `Qnil` (globals.h; `iQnil` 0).
@_silgen_name("Qnil") func Qnil() -> Lisp_Object
/// C: `Qleft` (globals.h) - the value `ev_modifiers_helper` compares against.
@_silgen_name("Qleft") func Qleft() -> Lisp_Object

// ==========================================================================
// C 82-88: the file-scope objects the file opens with.
// ==========================================================================

/// C 82: `static EmacsMenu *dockMenu;` - `EmacsMenu` is src/nsterm.h 540's class, implemented
/// by this lane's src/nsmenu.m (`femacs/platform/macos/nsmenu.swift`, compiled as its own unit
/// and not joint with this one), so C's pointer keeps P-55's untyped stand-in and the class's
/// methods are reached as C-ABI entries with the receiver first (the reading
/// `docs/ports/nsselect.md` gives `EmacsWindow`).
fileprivate var dockMenu: UnsafeMutableRawPointer? = nil
/// C 84: `static EmacsMenu *mainMenu;` (inside `#ifdef NS_IMPL_COCOA`)
fileprivate var mainMenu: UnsafeMutableRawPointer? = nil
/// C 88: "The last known monitor attributes list."  `static Lisp_Object last_known_monitors;` -
/// zero-initialised in C, which is what `nil` is here; `ns_term_init` fills it in.
fileprivate var last_known_monitors: Lisp_Object = nil

// C 96-149: `#if NSTRACE_ENABLED`'s trace block (`nstrace_num`, `nstrace_depth`,
// `nstrace_enabled_global`, `nstrace_leave`, `nstrace_restore_global_trace_state`,
// `nstrace_fullscreen_type_name`).  `NSTRACE_ENABLED` is **0** (src/nsterm.h 142-143), so none of
// it is compiled in the reference either; the three functions stay on the ledger's checklist as
// recorded dead text (docs/ports/nsterm.md).

// ==========================================================================
// C 152-206: `@implementation NSColor (EmacsColor)` - an Objective-C category.
// The port keeps the category as a Swift `extension` on the same class with the same four
// methods (docs/ports/nsselect.md's `NSString (EmacsString)` reading).
// ==========================================================================

extension NSColor {
  /// C 154-173: `+ (NSColor *)colorForEmacsRed:(CGFloat)red green:(CGFloat)green
  /// blue:(CGFloat)blue alpha:(CGFloat)alpha`.  The `NS_IMPL_COCOA &&
  /// MAC_OS_X_VERSION_MAX_ALLOWED >= 1070` arm is the one the Cocoa target compiles: the
  /// `NSAppKitVersionNumber >= NSAppKitVersionNumber10_7` runtime test and the sRGB branch.
  /// C's two globals are renamed in Swift - AppKit.apinotes 8624-8626 maps the first to
  /// `NSAppKitVersion.current` and the vintage constants to `NSAppKitVersion.macOS10_x`; the
  /// `.rawValue` comparison is the spelling the landed `nsimage.swift` uses for the pair.
  class func colorForEmacsRed(_ red: EmacsCGFloat, green: EmacsCGFloat, blue: EmacsCGFloat,
                              alpha: EmacsCGFloat) -> NSColor {
    if ns_use_srgb_colorspace
      && NSAppKitVersion.current.rawValue >= NSAppKitVersion.macOS10_7.rawValue {
      return NSColor(srgbRed: red, green: green, blue: blue, alpha: alpha)
    }
    return NSColor(calibratedRed: red, green: green, blue: blue, alpha: alpha)
  }

  /// C 175-186: `- (NSColor *)colorUsingDefaultColorSpace`.
  func colorUsingDefaultColorSpace() -> NSColor {
    if ns_use_srgb_colorspace
      && NSAppKitVersion.current.rawValue >= NSAppKitVersion.macOS10_7.rawValue {
      return usingColorSpace(NSColorSpace.sRGB) ?? self
    }
    return usingColorSpace(NSColorSpace.genericRGB) ?? self
  }

  /// C 188-197: `+ (NSColor *)colorWithUnsignedLong:(unsigned long)c` - the ARGB byte order.
  class func colorWithUnsignedLong(_ c: UInt) -> NSColor {
    let a = EmacsCGFloat((c >> 24) & 0xff) / 255.0
    let r = EmacsCGFloat((c >> 16) & 0xff) / 255.0
    let g = EmacsCGFloat((c >> 8) & 0xff) / 255.0
    let b = EmacsCGFloat(c & 0xff) / 255.0

    return NSColor.colorForEmacsRed(r, green: g, blue: b, alpha: a)
  }

  /// C 199-213: `- (unsigned long)unsignedLong` - the four components back into ARGB.
  func unsignedLong() -> UInt {
    var r: EmacsCGFloat = 0, g: EmacsCGFloat = 0, b: EmacsCGFloat = 0, a: EmacsCGFloat = 0
    getRed(&r, green: &g, blue: &b, alpha: &a)

    return (UInt(a * 255) << 24) | (UInt(r * 255) << 16) | (UInt(g * 255) << 8) | UInt(b * 255)
  }

}

// ==========================================================================
// C 208-293: "Local declarations" - the NeXT key-code to X-keysym table.
// ==========================================================================

/// C 214-217: "Convert a symbol indexed with an NSxxx value to a value as defined in
/// keyboard.c (lispy_function_key).  I hope this is a correct way of doing things..."
/// C: `static unsigned convert_ns_to_X_keysym[] =`
///
/// C's flat pair layout is kept: its one reader, `ns_convert_key` (C 2648), walks the table
/// with `keysym += 2` and reads `convert_ns_to_X_keysym[keysym]` / `[keysym + 1]`, so a flat
/// array transcribes the reference literally.  The enumerators are AppKit's own (NSEvent.h's
/// function-key enum imports as `Int`, NSText.h's character enum likewise), and C's
/// `NSEventModifierFlagNumericPad` is `NSEvent.ModifierFlags.numericPad`.
fileprivate let convert_ns_to_X_keysym: [UInt32] = [
  UInt32(NSHomeFunctionKey), 0x50,
  UInt32(NSLeftArrowFunctionKey), 0x51,
  UInt32(NSUpArrowFunctionKey), 0x52,
  UInt32(NSRightArrowFunctionKey), 0x53,
  UInt32(NSDownArrowFunctionKey), 0x54,
  UInt32(NSPageUpFunctionKey), 0x55,
  UInt32(NSPageDownFunctionKey), 0x56,
  UInt32(NSEndFunctionKey), 0x57,
  UInt32(NSBeginFunctionKey), 0x58,
  UInt32(NSSelectFunctionKey), 0x60,
  UInt32(NSPrintFunctionKey), 0x61,
  UInt32(NSClearLineFunctionKey), 0x0B,
  UInt32(NSExecuteFunctionKey), 0x62,
  UInt32(NSInsertFunctionKey), 0x63,
  UInt32(NSUndoFunctionKey), 0x65,
  UInt32(NSRedoFunctionKey), 0x66,
  UInt32(NSMenuFunctionKey), 0x67,
  UInt32(NSFindFunctionKey), 0x68,
  UInt32(NSHelpFunctionKey), 0x6A,
  UInt32(NSBreakFunctionKey), 0x6B,

  UInt32(NSF1FunctionKey), 0xBE,
  UInt32(NSF2FunctionKey), 0xBF,
  UInt32(NSF3FunctionKey), 0xC0,
  UInt32(NSF4FunctionKey), 0xC1,
  UInt32(NSF5FunctionKey), 0xC2,
  UInt32(NSF6FunctionKey), 0xC3,
  UInt32(NSF7FunctionKey), 0xC4,
  UInt32(NSF8FunctionKey), 0xC5,
  UInt32(NSF9FunctionKey), 0xC6,
  UInt32(NSF10FunctionKey), 0xC7,
  UInt32(NSF11FunctionKey), 0xC8,
  UInt32(NSF12FunctionKey), 0xC9,
  UInt32(NSF13FunctionKey), 0xCA,
  UInt32(NSF14FunctionKey), 0xCB,
  UInt32(NSF15FunctionKey), 0xCC,
  UInt32(NSF16FunctionKey), 0xCD,
  UInt32(NSF17FunctionKey), 0xCE,
  UInt32(NSF18FunctionKey), 0xCF,
  UInt32(NSF19FunctionKey), 0xD0,
  UInt32(NSF20FunctionKey), 0xD1,
  UInt32(NSF21FunctionKey), 0xD2,
  UInt32(NSF22FunctionKey), 0xD3,
  UInt32(NSF23FunctionKey), 0xD4,
  UInt32(NSF24FunctionKey), 0xD5,

  UInt32(NSBackspaceCharacter), 0x08,        // C 265: /* 8: Not on some KBs.  */
  UInt32(NSDeleteCharacter), 0xFF,           // C 266: /* 127: Big 'delete' key upper right.  */
  UInt32(NSDeleteFunctionKey), 0x9F,         // C 267: /* 63272: Del forw key off main array.  */

  UInt32(NSTabCharacter), 0x09,
  0x19, 0x09,                                // C 270: /* left tab->regular since pass shift */
  UInt32(NSCarriageReturnCharacter), 0x0D,
  UInt32(NSNewlineCharacter), 0x0D,
  UInt32(NSEnterCharacter), 0x8D,

  0x41 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xAE,  // C 275: /* KP_Decimal */
  0x43 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xAA,  // C 276: /* KP_Multiply */
  0x45 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xAB,  // C 277: /* KP_Add */
  0x4B | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xAF,  // C 278: /* KP_Divide */
  0x4E | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xAD,  // C 279: /* KP_Subtract */
  0x51 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xBD,  // C 280: /* KP_Equal */
  0x52 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xB0,  // C 281: /* KP_0 */
  0x53 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xB1,  // C 282: /* KP_1 */
  0x54 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xB2,  // C 283: /* KP_2 */
  0x55 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xB3,  // C 284: /* KP_3 */
  0x56 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xB4,  // C 285: /* KP_4 */
  0x57 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xB5,  // C 286: /* KP_5 */
  0x58 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xB6,  // C 287: /* KP_6 */
  0x59 | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xB7,  // C 288: /* KP_7 */
  0x5B | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xB8,  // C 289: /* KP_8 */
  0x5C | UInt32(NSEvent.ModifierFlags.numericPad.rawValue), 0xB9,  // C 290: /* KP_9 */

  0x1B, 0x1B                                 // C 292: /* escape */
]

// ==========================================================================
// C 295-347: the file-scope objects the file's state lives in.
// ==========================================================================

/// C 298-299: "On macOS picks up the default NSGlobalDomain AppleAntiAliasingThreshold, the
/// maximum font size to NOT antialias.  On GNUstep there is currently no way to control this
/// behavior."  `float ns_antialias_threshold;`
var ns_antialias_threshold: Float = 0

/// C 300-301: `NSArray *ns_send_types = 0, *ns_return_types = 0; static NSArray *ns_drag_types = 0;`
/// The first two are C *data* symbols other files read (`src/nsterm.h` 774); Swift cannot export a
/// data symbol, so the lane reaches them as C-ABI **functions** of the same name
/// (docs/ports/nsselect.md (2)c, where `nsselect.swift` imports them), and this unit owns the
/// definitions.
fileprivate var ns_send_types_storage: NSArray? = nil
fileprivate var ns_return_types_storage: NSArray? = nil
fileprivate var ns_drag_types: NSArray? = nil

/// C 300: `NSArray *ns_send_types` - the C-ABI entry (see above).
@_cdecl("ns_send_types") func ns_send_types() -> NSArray? { ns_send_types_storage }
/// C 300: `NSArray *ns_return_types` - the C-ABI entry (see above).
@_cdecl("ns_return_types") func ns_return_types() -> NSArray? { ns_return_types_storage }

/// C 303: `NSString *ns_app_name = @"Emacs"; /* default changed later */` - exported (src/nsterm.h
/// 775), so it takes the same C-ABI-function shape.
fileprivate var ns_app_name_storage: NSString? = "Emacs"
@_cdecl("ns_app_name") func ns_app_name() -> NSString? { ns_app_name_storage }

/// C 305-306: "Display variables": `struct ns_display_info *x_display_list;` and
/// `long context_menu_value = 0;`.  Both are exported (`src/nsterm.h` 931-933); the record's
/// pointee belongs to this lane's header mirror, so the port keeps C's pointer depth with the
/// untyped stand-in (P-55) and the ledger names the owner.
fileprivate var x_display_list: UnsafeMutableRawPointer? = nil
@_cdecl("x_display_list") func x_display_list_get() -> UnsafeMutableRawPointer? { x_display_list }
fileprivate var context_menu_value_storage: Int = 0
@_cdecl("context_menu_value") func context_menu_value() -> Int { context_menu_value_storage }

/// C 308-313: "display update": `static struct frame *ns_updating_frame; static int ns_window_num
/// = 0; static BOOL gsaved = NO; static BOOL ns_menu_bar_is_hidden = NO;` (the last one inside
/// `#ifdef NS_IMPL_COCOA`).
fileprivate var ns_updating_frame: UnsafeMutableRawPointer? = nil
fileprivate var ns_window_num: Int32 = 0
fileprivate var gsaved: Bool = false
fileprivate var ns_menu_bar_is_hidden: Bool = false

/// C 315-342: "event loop".  `send_appdefined`, the `NO_APPDEFINED_DATA` sentinel, the two
/// timers, the `select` state, the self-pipe, the mutex, the autorelease pool, the event pointers
/// and the pending-file arrays - C's declarations with C's comments.
/// C: `static BOOL send_appdefined = YES;`
fileprivate var send_appdefined: Bool = true
/// C: `#define NO_APPDEFINED_DATA (-8)` / `static int last_appdefined_event_data =
/// NO_APPDEFINED_DATA;`
fileprivate let NO_APPDEFINED_DATA: Int32 = -8
fileprivate var last_appdefined_event_data: Int32 = NO_APPDEFINED_DATA
/// C: `static NSTimer *timed_entry = 0; static NSTimer *scroll_repeat_entry = nil;`
fileprivate var timed_entry: Timer? = nil
fileprivate var scroll_repeat_entry: Timer? = nil
/// C: `static fd_set select_readfds, select_writefds;`
fileprivate var select_readfds = fd_set()
fileprivate var select_writefds = fd_set()
/// C: `enum { SELECT_HAVE_READ = 1, SELECT_HAVE_WRITE = 2, SELECT_HAVE_TMO = 4 };` - the
/// enumerators are imported with `use …::*` in the Rust lane and stay bare names here.
/// C: `static int select_nfds = 0, select_valid = 0;`
fileprivate var select_nfds: Int32 = 0
fileprivate var select_valid: Int32 = 0
/// C: `static struct timespec select_timeout = { 0, 0 };`
fileprivate var select_timeout = timespec(tv_sec: 0, tv_nsec: 0)
/// C: `static int selfds[2] = { -1, -1 };`
fileprivate var selfds: (Int32, Int32) = (-1, -1)
/// C: `static pthread_mutex_t select_mutex;`
fileprivate var select_mutex = pthread_mutex_t()
/// C: `static NSAutoreleasePool *outerpool;` - ARC has no `NSAutoreleasePool` (measured: it is
/// "not available in automatic reference counting mode"), so the unit keeps the *handle*; the
/// slices that create and re-create it (C 560-568, 5154-5155, 5982-5983) reach the runtime's
/// `objc_autoreleasePoolPush`/`objc_autoreleasePoolPop` (the pair `@autoreleasepool` compiles
/// to; the symbols are in libobjc but not in the SDK's Swift-visible headers - measured).
fileprivate var outerpool: UnsafeMutableRawPointer? = nil
/// C: `static struct input_event *emacs_event = NULL; static struct input_event *q_event_ptr =
/// NULL;` - the record is this unit's view of termhooks.h's (see `input_event` above), so the
/// pointers are typed by it rather than by P-55's untyped stand-in.
fileprivate var emacs_event: UnsafeMutablePointer<input_event>? = nil
fileprivate var q_event_ptr: UnsafeMutablePointer<input_event>? = nil
/// C: `static int n_emacs_events_pending = 0;`
fileprivate var n_emacs_events_pending: Int32 = 0
/// C: `static NSMutableArray *ns_pending_files, *ns_pending_service_names,
/// *ns_pending_service_args;`
fileprivate var ns_pending_files: NSMutableArray? = nil
fileprivate var ns_pending_service_names: NSMutableArray? = nil
fileprivate var ns_pending_service_args: NSMutableArray? = nil
/// C: `static BOOL ns_do_open_file = NO; static BOOL ns_last_use_native_fullscreen;`
fileprivate var ns_do_open_file: Bool = false
fileprivate var ns_last_use_native_fullscreen: Bool = false

/// C 335-336: "Non-zero means that a HELP_EVENT has been generated since Emacs start."
/// C: `static BOOL any_help_event_p = NO;`
fileprivate var any_help_event_p: Bool = false

/// C 338-342: `static struct { struct input_event *q; int nr, cap; } hold_event_q = { NULL, 0, 0 };`
/// C's `q` is an array of `struct input_event` (`xrealloc`ed, indexed by `nr`), so it is a typed
/// pointer here.
fileprivate var hold_event_q_q: UnsafeMutablePointer<input_event>? = nil
fileprivate var hold_event_q_nr: Int32 = 0
fileprivate var hold_event_q_cap: Int32 = 0

/// C 349-355: the NeXTstep modifier masks, "Convert modifiers in a NeXTstep event to emacs style
/// modifiers".  C's `#define`s name AppKit's own flags, so they stay `let`s of the same shape.
/// C: `#define NS_FUNCTION_KEY_MASK 0x800000`
let NS_FUNCTION_KEY_MASK: UInt = 0x800000
/// C: `#define NSLeftControlKeyMask (0x000001 | NSEventModifierFlagControl)`
let NSLeftControlKeyMask: UInt = 0x000001 | NSEvent.ModifierFlags.control.rawValue
let NSRightControlKeyMask: UInt = 0x002000 | NSEvent.ModifierFlags.control.rawValue
let NSLeftCommandKeyMask: UInt = 0x000008 | NSEvent.ModifierFlags.command.rawValue
let NSRightCommandKeyMask: UInt = 0x000010 | NSEvent.ModifierFlags.command.rawValue
let NSLeftAlternateKeyMask: UInt = 0x000020 | NSEvent.ModifierFlags.option.rawValue
let NSRightAlternateKeyMask: UInt = 0x000040 | NSEvent.ModifierFlags.option.rawValue

// ==========================================================================
// C 358-389: the first two functions (the masks above are C 349-356).
// ==========================================================================

/// C 358-367: "MODIFIER if a symbol; otherwise its property KIND, if a symbol."
/// C: `static Lisp_Object mod_of_kind (Lisp_Object modifier, Lisp_Object kind)`
fileprivate func mod_of_kind(_ modifier: Lisp_Object, _ kind: Lisp_Object) -> Lisp_Object {
  if SYMBOLP(modifier) {
    return modifier
  } else {
    let val = plist_get(modifier, kind)
    return SYMBOLP(val) ? val : Qnil()
  }
}

/// C 369-389: the left/right modifier helper.  C's `unsigned int` returns are the modifier bits
/// `parse_solitary_modifier` produces (an `int` there, src/keyboard.c 7920).
/// C: `static unsigned int ev_modifiers_helper (unsigned int flags, unsigned int left_mask,
/// unsigned int right_mask, unsigned int either_mask, Lisp_Object left_modifier,
/// Lisp_Object right_modifier)`
fileprivate func ev_modifiers_helper(_ flags: UInt32, _ left_mask: UInt32, _ right_mask: UInt32,
                         _ either_mask: UInt32, _ left_modifier: Lisp_Object,
                         _ right_modifier: Lisp_Object) -> UInt32 {
  var modifiers: UInt32 = 0

  if (flags & either_mask) != 0 {
    let left_key = (flags & left_mask) == left_mask
    let right_key = (flags & right_mask) == right_mask && !EQ(right_modifier, Qleft())

    if right_key {
      modifiers |= UInt32(bitPattern: parse_solitary_modifier(right_modifier))
    }

    /* GNUstep (and possibly macOS in certain circumstances) doesn't
       differentiate between the left and right keys, so if we can't
       identify which key it is, we use the left key setting.  */
    if left_key || !right_key {
      modifiers |= UInt32(bitPattern: parse_solitary_modifier(left_modifier))
    }
  }

  return modifiers
}


// ==========================================================================
// C 391-519: the `EV_*` accessors, the forward declarations, and the event-loop utilities.
// ==========================================================================

// The seven `DEFVAR_LISP` variables `syms_of_nsterm` defines (C 11608-11670).  This block reads
// them about 11,000 lines earlier, and a fileprivate global of this unit's own would not exist
// yet at C 397, so the lane imports them as the C data symbols the core exports - the shape
// `docs/ports/macfont.swift` uses for `Votf_script_alist`, and the integrator's ruling for this
// slice (docs/ports/nsterm.md's Decisions).
@_silgen_name("ns_alternate_modifier") var ns_alternate_modifier: Lisp_Object
@_silgen_name("ns_right_alternate_modifier") var ns_right_alternate_modifier: Lisp_Object
@_silgen_name("ns_command_modifier") var ns_command_modifier: Lisp_Object
@_silgen_name("ns_right_command_modifier") var ns_right_command_modifier: Lisp_Object
@_silgen_name("ns_control_modifier") var ns_control_modifier: Lisp_Object
@_silgen_name("ns_right_control_modifier") var ns_right_control_modifier: Lisp_Object
@_silgen_name("ns_function_modifier") var ns_function_modifier: Lisp_Object

// `src/termhooks.h` 434-459's modifier bits are enumerators; the lane reaches an enumerator
// through a C-ABI entry carrying its own name (docs/ports/nsselect.md's `PVEC_SUBR` reading).
@_silgen_name("up_modifier") func up_modifier() -> Int32
@_silgen_name("down_modifier") func down_modifier() -> Int32
@_silgen_name("hyper_modifier") func hyper_modifier() -> Int32
@_silgen_name("shift_modifier") func shift_modifier() -> Int32

/// C: `DEFSYM (QCmouse, ":mouse")` (src/nsterm.m 11795) is **this file's own** `DEFSYM`, and it
/// is absent from `docs/baseline/globals/symbols.tsv` (whose table is generated from the frozen
/// build set, which never compiles this file); the core has to export it like the rest - the
/// reading `docs/ports/macfont.swift` gives `Qmac_ct`/`QCdestination`.
@_silgen_name("QCmouse") func QCmouse() -> Lisp_Object

/// C: `void *xrealloc (void *, size_t)` (src/lisp.h 5541) - the "x" allocator: it never returns
/// NULL (src/alloc.c 4130's `memory_full`), which is why the call below takes the result apart.
@_silgen_name("xrealloc") func xrealloc(_ p: UnsafeMutableRawPointer?,
                                        _ size: Int) -> UnsafeMutableRawPointer?
/// C: `Lisp_Object nconc2 (Lisp_Object, Lisp_Object)` (src/lisp.h 4271; defined src/fns.c).
@_silgen_name("nconc2") func nconc2(_ a: Lisp_Object, _ b: Lisp_Object) -> Lisp_Object
/// C: `list1` (src/lisp.h 4486; defined src/alloc.c) - the fixed-arity spelling of the
/// variadic `list` macro (src/lisp.h 4493), see `append2` below.
@_silgen_name("list1") func list1(_ a: Lisp_Object) -> Lisp_Object
/// C: `#define EVENT_INIT(event) ((event) = (struct input_event) {.device = Qt})` (src/termhooks.h
/// 414).  The core's entry takes `struct input_event *`, which this unit's view of that record
/// spells as a pointer to it (the same layout - see the type's comment).
@_silgen_name("EVENT_INIT") func EVENT_INIT(_ event: UnsafeMutablePointer<input_event>)

/// C 397-424: `#define EV_MODIFIERS2(flags, kind)` - "Convert modifiers in a NeXTstep event to
/// emacs style modifiers".  C's value is `unsigned int`, and its three helper calls take
/// `unsigned int` parameters, so the `NSUInteger` flags, masks and AppKit flags are converted
/// exactly where C's implicit conversions convert them (`truncatingIfNeeded:` is C's narrowing).
func EV_MODIFIERS2(_ flags: UInt, _ kind: Lisp_Object) -> UInt32 {
  var modifiers: UInt32 = 0

  modifiers |= (flags & NSEvent.ModifierFlags.help.rawValue) != 0
    ? UInt32(bitPattern: hyper_modifier()) : 0
  modifiers |= (flags & NSEvent.ModifierFlags.shift.rawValue) != 0
    ? UInt32(bitPattern: shift_modifier()) : 0
  modifiers |= (flags & NS_FUNCTION_KEY_MASK) != 0
    ? UInt32(bitPattern: parse_solitary_modifier(mod_of_kind(ns_function_modifier, kind))) : 0

  modifiers |= ev_modifiers_helper(UInt32(truncatingIfNeeded: flags),
                                   UInt32(truncatingIfNeeded: NSLeftControlKeyMask),
                                   UInt32(truncatingIfNeeded: NSRightControlKeyMask),
                                   UInt32(NSEvent.ModifierFlags.control.rawValue),
                                   mod_of_kind(ns_control_modifier, kind),
                                   mod_of_kind(ns_right_control_modifier, kind))
  modifiers |= ev_modifiers_helper(UInt32(truncatingIfNeeded: flags),
                                   UInt32(truncatingIfNeeded: NSLeftCommandKeyMask),
                                   UInt32(truncatingIfNeeded: NSRightCommandKeyMask),
                                   UInt32(NSEvent.ModifierFlags.command.rawValue),
                                   mod_of_kind(ns_command_modifier, kind),
                                   mod_of_kind(ns_right_command_modifier, kind))
  modifiers |= ev_modifiers_helper(UInt32(truncatingIfNeeded: flags),
                                   UInt32(truncatingIfNeeded: NSLeftAlternateKeyMask),
                                   UInt32(truncatingIfNeeded: NSRightAlternateKeyMask),
                                   UInt32(NSEvent.ModifierFlags.option.rawValue),
                                   mod_of_kind(ns_alternate_modifier, kind),
                                   mod_of_kind(ns_right_alternate_modifier, kind))

  return modifiers
}

/// C 425: `#define EV_MODIFIERS(e) EV_MODIFIERS2 ([e modifierFlags], QCmouse)`.
func EV_MODIFIERS(_ e: NSEvent) -> UInt32 {
  EV_MODIFIERS2(e.modifierFlags.rawValue, QCmouse())
}

/// C 427-437: `#define EV_UDMODIFIERS(e)` - the up/down bit of a mouse event, C's `int`.  The
/// ten comparisons are transcribed one for one (C ORs the same bit in more than once when more
/// than one holds, which is why this is not folded into a set membership test).
func EV_UDMODIFIERS(_ e: NSEvent) -> Int32 {
  ((e.type == .leftMouseDown) ? down_modifier() : 0)
    | ((e.type == .rightMouseDown) ? down_modifier() : 0)
    | ((e.type == .otherMouseDown) ? down_modifier() : 0)
    | ((e.type == .leftMouseDragged) ? down_modifier() : 0)
    | ((e.type == .rightMouseDragged) ? down_modifier() : 0)
    | ((e.type == .otherMouseDragged) ? down_modifier() : 0)
    | ((e.type == .leftMouseUp) ? up_modifier() : 0)
    | ((e.type == .rightMouseUp) ? up_modifier() : 0)
    | ((e.type == .otherMouseUp) ? up_modifier() : 0)
}

/// C 438-442: `#define EV_BUTTON(e)` - the wheel/button index C computes from `[e buttonNumber]`,
/// C's `NSInteger`.
func EV_BUTTON(_ e: NSEvent) -> Int {
  ((e.type == .leftMouseDown) || (e.type == .leftMouseUp)) ? 0
    : ((e.type == .rightMouseDown) || (e.type == .rightMouseUp)) ? 2
    : e.buttonNumber - 1
}

/// C 444: `#define EV_TIMESTAMP(e) ([e timestamp] * 1000)` - "Convert the time field to a
/// timestamp in milliseconds."  C's product of `NSTimeInterval` (double); the sites that store
/// it convert it the way C's assignment converts a double to `Time`.
func EV_TIMESTAMP(_ e: NSEvent) -> Double {
  e.timestamp * 1000
}

// C 448-470: `EV_TRAILER` and `EV_TRAILER2`, the two macros every event handler ends with, are
// **held**: Swift has no forward declaration, and their bodies call `hold_event` (this file's
// C 498) and `ns_send_appdefined` (this file's C 5013), whose Swift definitions do not exist
// yet.  The reference's own macro text sits before those definitions too, and is only *expanded*
// after them (the first expansion is C 6412) - so they land at C 5013, the point where both
// callees exist, and their checklist boxes stay open until then (docs/ports/nsterm.md).

// C 473-475: "TODO: Get rid of need for these forward declarations." - `static void
// ns_condemn_scroll_bars (struct frame *f);` and `static void ns_judge_scroll_bars (struct
// frame *f);`.  Swift has no forward declaration and needs none: each function lands once, at
// its own definition (C 5575 and C 5622).

/// C 484-489: `void ns_init_events (struct input_event *ev)`.  Declared src/nsterm.h 1257 and
/// called from src/nsfns.m 2492, so it is part of the unit's C interface (the `@_cdecl` half of
/// the lane's ABI).  C: `EVENT_INIT (*ev); emacs_event = ev;`
///
/// The exported parameter is untyped: `@_cdecl` refuses a pointer to a Swift record ("the type
/// of the parameter cannot be represented in Objective-C", measured), and at the ABI it is C's
/// `struct input_event *` either way - the caller passes its own view of the record.
@_cdecl("ns_init_events") func ns_init_events(_ ev: UnsafeMutableRawPointer) {
  let event = ev.assumingMemoryBound(to: input_event.self)
  EVENT_INIT(event)
  emacs_event = event
}

/// C 492-495: `void ns_finish_events (void)` (declared src/nsterm.h 1258, called src/nsfns.m
/// 2495).  C: `emacs_event = NULL;`
@_cdecl("ns_finish_events") func ns_finish_events() {
  emacs_event = nil
}

/// C 498-514: `static void hold_event (struct input_event *event)` (the record's line above is
/// termhooks.h 371) - the hold queue
/// `ns_read_socket` drains; `EV_TRAILER2` (C 467) and the scroller's
/// `sendScrollEventAtLoc:fromEvent:` (C 10886) are its callers.
fileprivate func hold_event(_ event: UnsafeMutablePointer<input_event>?) {
  if hold_event_q_nr == hold_event_q_cap {
    if hold_event_q_cap == 0 { hold_event_q_cap = 10 } else { hold_event_q_cap *= 2 }
    // C's `xrealloc` never returns NULL, so the pointer comes back to this lane's typed view
    // without an optional in between (src/alloc.c's `memory_full` on failure).
    hold_event_q_q = xrealloc(hold_event_q_q,
                              Int(hold_event_q_cap) * MemoryLayout<input_event>.stride)!
      .assumingMemoryBound(to: input_event.self)
  }

  // C's `hold_event_q.q[hold_event_q.nr++] = *event;` - the value copy of one record; a NULL
  // argument traps here as the reference's unguarded `*event` faults there.
  hold_event_q_q![Int(hold_event_q_nr)] = event!.pointee
  hold_event_q_nr += 1
  /* Make sure ns_read_socket is called, i.e. we have input.  */
  raise(SIGIO)
  send_appdefined = true
}

/// C 515-519: `static Lisp_Object append2 (Lisp_Object list, Lisp_Object item)` - "Utility to
/// append to a list".  C's `list` (src/lisp.h 4493) is the variadic macro
/// `listn (countof (…), …)`; with one argument it is `list1`'s call (src/lisp.h 4486), the
/// lane's reading for a fixed-arity spelling of a variadic macro (docs/ports/nsselect.md's
/// `calln`).
fileprivate func append2(_ list: Lisp_Object, _ item: Lisp_Object) -> Lisp_Object {
  nconc2(list, list1(item))
}

