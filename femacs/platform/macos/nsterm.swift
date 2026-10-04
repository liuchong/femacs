// femacs: port of src/nsterm.m @ fd7afced403.
//
// **Giant unit** (`docs/ports/nsterm.md`'s `## Progress`): the reference is 11,811 lines and is
// transcribed in C order, one slice per round.  **This round lands C 60-390** - the file's first
// statics (C 82-88), the dead `NSTRACE_ENABLED` block (C 96-149), the `NSColor (EmacsColor)`
// category (C 152-206), the `convert_ns_to_X_keysym` table (C 214-293), the file-scope objects
// (C 295-347), the seven modifier masks (C 349-356) and the first two functions
// (`mod_of_kind` C 358, `ev_modifiers_helper` C 369).  The next slice starts at C 391 (the
// `EV_*` macro block).
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
/// NULL;` - `struct input_event` is termhooks.h's (`femacs/src/include/termhooks.rs`), so the
/// pointer keeps P-55's untyped stand-in.
fileprivate var emacs_event: UnsafeMutableRawPointer? = nil
fileprivate var q_event_ptr: UnsafeMutableRawPointer? = nil
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
fileprivate var hold_event_q_q: UnsafeMutableRawPointer? = nil
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

// C 379-470: the `EV_*` macro block (`EV_MODIFIERS2`, `EV_MODIFIERS`, `EV_UDMODIFIERS`,
// `EV_BUTTON`, `EV_TIMESTAMP`, `EV_TRAILER`, `EV_TRAILER2`) is the **next slice** (it reads
// `emacs_event`, `ns_send_appdefined`, the `ns_*_modifier` globals and AppKit events); the two
// forward declarations at C 473-475 (`ns_condemn_scroll_bars`, `ns_judge_scroll_bars`) are the
// slice after that.
