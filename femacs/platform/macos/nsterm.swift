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
// `hold_event`/`append2` (C 498-519).  **Round 3 lands C 522-638**: `ns_relocate` (C 525-557,
// its `NS_SELF_CONTAINED` arm live per `configure.ac` 689-690/2885-2886), `ns_init_pool`
// (C 560-568), `ns_init_locale` (C 569-600), and the four retain/release/pool hooks "callable
// from C" (C 602-638).  `EV_TRAILER`/`EV_TRAILER2` (C 448-470) are **held** to C 5013, where
// their two callees exist (see the note in round 2's block).  **Round 4 lands C 641-991**: the
// menu-bar predicate (C 641-648), `struct EmacsMargins` (C 650-656), the screen margins and the
// hidden-dock variant (C 659-753), the menu-bar height (C 754-772), `NS_PARENT_WINDOW_*` and the
// parent-window rect (C 773-807), `ns_row_rect` (C 808-827), `ns_frame_scale_factor` (C 828-891)
// and `constrain_frame_rect` (C 892-991).  The next live definition is C 1001's
// `ns_constrain_all_frames`, which needs a ruling: it calls this unit's own `EmacsView`
// `-isFullscreen` (C 8957), a method of a class that lands much later in the same file: the
// entry/definition split the ruling settled (see `## ABI` (4) in the ledger).  **Round 5 lands
// C 1001-1211**: `ns_constrain_all_frames` (C 1001-1030), `ns_update_auto_hide_menu_bar`
// (C 1033-1073), `ns_update_begin` (C 1075-1087) and the whole macOS Accessibility Zoom block
// (C 1090-1211: its statics, `ns_ua_zoom_enabled_p`, `ns_cg_rect_flip_y` and
// `ns_UAZoomChangeFocus`).  **Round 6 lands C 1212-1492**: `ns_update_end` (C 1212-1234),
// `ns_focus` (C 1236-1273) and `ns_unfocus` (C 1282-1310), the visible bell's narrative
// (C 1313-1330), the file's **first class** `EmacsBell` (C 1333-1430, its `@interface` and all
// four methods), `bell_view` (C 1433) and `ns_ring_bell` (C 1436-1469); `hide_bell` (C 1471-1491)
// is dead in this build (guard at 1471) and is recorded, not transcribed.  **Round 7 lands
// C 1493-1743**: the focus hooks (`ns_get_focus_frame` C 1493-1508, `ns_focus_frame` C 1510-1525),
// the raise/lower pair and their dispatcher (C 1527-1578), `ns_frame_rehighlight` (C 1582-1620),
// and the visibility hooks `ns_make_frame_visible` (C 1624-1676, exported),
// `ns_make_frame_invisible` (C 1678-1691, exported), `ns_make_frame_visible_invisible`
// (C 1693-1702) and `ns_iconify_frame` (C 1705-1742, exported).  The next live definition is
// C 1744's `ns_free_frame_resources`.  **Round 8 lands C 1744-1876**: the resource teardown
// (`ns_free_frame_resources` C 1744-1780, exported), `ns_destroy_window` (C 1782-1810),
// `compute_offset` (C 1812-1852) and **`ns_set_offset` (C 1854-1875) - the definition the
// deferred entry `ns_set_offset_entry` waited for** - so `nm -u` no longer lists
// `_ns_set_offset`.  **Round 9 lands C 1877-1956**: `ns_set_window_size` (C 1877-1915) and
// `ns_set_window_size_and_position` (C 1920-1956).  The next live definition is C 1959's
// `ns_set_undecorated`.
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
import ApplicationServices

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

// ==========================================================================
// C 522-638: path relocation, the autorelease pool, the locale, and the
// retain/release hooks "callable from C".
// ==========================================================================

/// C: `void *objc_autoreleasePoolPush (void)` and `void objc_autoreleasePoolPop (void *)` - the
/// pair `@autoreleasepool` compiles to, and the runtime's own pool primitives.  The SDK's
/// Swift-visible headers do not carry them (measured) and `NSAutoreleasePool` is unavailable
/// under ARC (round 1's `outerpool` decision), so the pool functions below reach the runtime by
/// name; round 1 measured the two symbols in libobjc with a C probe.
@_silgen_name("objc_autoreleasePoolPush")
func objc_autoreleasePoolPush() -> UnsafeMutableRawPointer?
@_silgen_name("objc_autoreleasePoolPop")
func objc_autoreleasePoolPop(_ pool: UnsafeMutableRawPointer?)

/// C: `int fprintf (FILE *, const char *, ...)` (stdio.h) - Swift's own `fprintf` is marked
/// unavailable, and the lane reaches the C entry by name (docs/ports/nsfont.swift 1173-1177).
@_silgen_name("fprintf")
func fprintf(_ stream: UnsafeMutablePointer<FILE>?, _ format: UnsafePointer<CChar>,
             _ args: CVarArg...)

/// C 525-557: `const char *ns_relocate (const char *epath)` - "If we're running in a
/// self-contained app bundle some hard-coded paths are relative to the root of the bundle, so
/// work out the full path."  Declared src/nsterm.h 1195 and called from src/emacs.c 1059 and
/// 3298, so it is a `@_cdecl` export.
///
/// The `#ifdef NS_SELF_CONTAINED` arm is the live one: `configure.ac` 689-690 defaults the
/// option to `yes` and 2885-2886 `AC_DEFINE`s the macro, so the bundled build - the one this
/// ledger describes - has the block; the `--disable-ns-self-contained` build compiles the same
/// function without it (P-51: the dead arm is this paragraph, recorded, not transcribed).
@_cdecl("ns_relocate") func ns_relocate(_ epath: UnsafePointer<CChar>?)
  -> UnsafePointer<CChar>? {
  let root = Bundle.main.bundlePath
  let original: NSString? = epath.flatMap { NSString(utf8String: $0) }
  // C's `[NSArray arrayWithObjects: root, original, nil]` stops at the first nil, so a path
  // that is not UTF-8 leaves `root` alone in the array.
  let components = original == nil ? [root] : [root, original! as String]
  let fixedPath = NSString.path(withComponents: components) as NSString
  let fileManager = FileManager.default

  if !(original?.isAbsolutePath ?? false)
       && fileManager.fileExists(atPath: fixedPath as String, isDirectory: nil) {
    // C returns `[fixedPath UTF8String]`, and `fixedPath` is *autoreleased* there: the pointer
    // stays valid until the pool drains.  The unit autoreleases its `NSString` the same way -
    // `passRetained` + `autorelease` is exactly that pair - so the pointer the caller gets has
    // C's lifetime instead of one that ends when this scope does.
    _ = Unmanaged.passRetained(fixedPath).autorelease()
    return fixedPath.utf8String
  }

  /* If we reach here either the path is absolute and therefore we
     don't need to complete it, or we're unable to relocate the
     file/directory.  If it's the latter it may be because the user is
     trying to use a bundled app as though it's a Unix style install
     and we have no way to guess what was intended, so return the
     original string unaltered.  */

  return epath
}

/// C 560-568: `void ns_init_pool (void)` - "Initialize the 'outerpool' autorelease pool.  This
/// should be called from main before any Objective C code is run."  Declared src/nsterm.h 1171,
/// called from src/emacs.c 1426.  C: `outerpool = [[NSAutoreleasePool alloc] init];` - under ARC
/// that type is unavailable, so the pool is the runtime's push (round 1's `outerpool` decision).
@_cdecl("ns_init_pool") func ns_init_pool() {
  outerpool = objc_autoreleasePoolPush()
}

/// C 597: `const char *const default_lang = "en_US.UTF-8";` - a C literal has static storage, so
/// its bytes outlive the call; this file-scope `NSString` is the same storage class (a
/// `withCString` scope or a local Swift `String` would die before `setenv` below runs).
fileprivate let ns_init_locale_default_lang = NSString(string: "en_US.UTF-8")

/// C 569-600: `void ns_init_locale (void)` - "macOS doesn't set any environment variables for
/// the locale when run from the GUI.  Get the locale from the OS and set LANG."  Declared
/// src/nsterm.h 1172, called from src/emacs.c 1647.  `NSTRACE ("ns_init_locale")` is the empty
/// macro when `NSTRACE_ENABLED` is 0 (src/nsterm.h 297 and 302), so it carries no statement here
/// either - the same dead arm round 1 recorded for C 96-149.
@_cdecl("ns_init_locale") func ns_init_locale() {
  /* Either use LANG, if set, or try to construct LANG from
     NSLocale.  */
  var lang = UnsafePointer<CChar>(getenv("LANG"))
  if lang == nil || lang!.pointee == 0 {
    let localeID = NSString(format: "%@.UTF-8" as NSString,
                            Locale.current.identifier as NSString)
    // C's `[NSString stringWithFormat:…]` is autoreleased there, so the `const char *` it yields
    // is valid for the rest of the function; the unit autoreleases it the same way.
    _ = Unmanaged.passRetained(localeID).autorelease()
    lang = localeID.utf8String
  }

  /* Check if LANG can be used for initializing the locale.  If not,
     use a default setting.  Note that Emacs's main will undo the
     setlocale below, initializing the locale from the
     environment.  */
  if setlocale(LC_ALL, lang) == nil {
    fprintf(stderr, "LANG=%s cannot be used, using %s instead.\n",
            lang!, ns_init_locale_default_lang.utf8String!)
    lang = ns_init_locale_default_lang.utf8String
  }

  setenv("LANG", lang, 1)
}

/// C 602-608: `void ns_release_object (void *obj)` - "Release an object (callable from C)".
/// Declared src/nsterm.h 1166, and `nsselect.swift` (line 215) imports it as the
/// `record_unwind_protect_ptr` callback (src/nsselect.m 601), so this export's parameter type is
/// that file's: an optional untyped pointer.
@_cdecl("ns_release_object") func ns_release_object(_ obj: UnsafeMutableRawPointer?) {
  // C: `[(id)obj release];`.  ARC refuses `release`, so the unit consumes the +1 the reference
  // consumes through the runtime's own entry (`Unmanaged.release`); a NULL argument is C's
  // nil-receiver message, which does nothing.
  if obj == nil { return }
  Unmanaged<AnyObject>.fromOpaque(obj!).release()
}

/// C 610-616: `void ns_retain_object (void *obj)` - "Retain an object (callable from C)".
/// Declared src/nsterm.h 1167, called from src/image.c 4418.  C: `[(id)obj retain];` - the +1
/// belongs to the caller, so the result is discarded rather than balanced.
@_cdecl("ns_retain_object") func ns_retain_object(_ obj: UnsafeMutableRawPointer?) {
  if obj == nil { return }
  _ = Unmanaged<AnyObject>.fromOpaque(obj!).retain()
}

/// C 622-628: `void *ns_alloc_autorelease_pool (void)` - "Allocate a pool for temporary objects
/// (callable from C)".  Declared src/nsterm.h 1168, called from src/thread.c 786 (paired with
/// `ns_release_autorelease_pool` at 842).
@_cdecl("ns_alloc_autorelease_pool")
func ns_alloc_autorelease_pool() -> UnsafeMutableRawPointer? {
  objc_autoreleasePoolPush()
}

/// C 632-638: `void ns_release_autorelease_pool (void *pool)` - "Free a pool and temporary
/// objects it refers to (callable from C)".  C: `ns_release_object (pool);` - the same
/// `[(id)pool release]` that a pool's pop is, which is the runtime entry this unit pushes with;
/// C's nil receiver is a no-op, so a NULL pool returns here.
@_cdecl("ns_release_autorelease_pool")
func ns_release_autorelease_pool(_ pool: UnsafeMutableRawPointer?) {
  if pool == nil { return }
  objc_autoreleasePoolPop(pool)
}

// ==========================================================================
// C 641-991: the screen geometry, the menu bar, and the window constrainer.
// ==========================================================================

/// C: `bool ns_auto_hide_menu_bar` - src/nsterm.m 11697's `DEFVAR_LISP ("ns-auto-hide-menu-bar",
/// …)` (a `Lisp_Object`, set to `Qnil` at 11700).  Imported as the C data symbol the core
/// exports, the shape round 2 set for the seven `ns_*_modifier` variables and round 1 for
/// `ns_use_srgb_colorspace`; it is owed until `syms_of_nsterm`'s slice lands.
@_silgen_name("ns_auto_hide_menu_bar") var ns_auto_hide_menu_bar: Lisp_Object

/// C: `#define NILP(x) lisp_h_NILP (x)` (src/lisp.h 450) - the predicate the generated globals
/// use; the lane reaches it as a C-ABI entry of its own name (docs/ports/nsfns.swift 176).
@_silgen_name("NILP") func NILP(_ x: Lisp_Object) -> Bool

/// C: `struct frame *FRAME_PARENT_FRAME (struct frame *f)` (src/frame.h 1234, an INLINE).
@_silgen_name("FRAME_PARENT_FRAME")
func FRAME_PARENT_FRAME(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
/// C: `#define FRAME_NS_VIEW(f) ((f)->output_data.ns->view)` (src/nsterm.h 1045).
@_silgen_name("FRAME_NS_VIEW") func FRAME_NS_VIEW(_ f: UnsafeMutableRawPointer?) -> NSView?

/// C: `#define WINDOW_TO_FRAME_PIXEL_Y(W, Y) ((Y) + WINDOW_TOP_EDGE_Y (W))` (src/window.h 1078) -
/// "Convert window W relative pixel Y to frame pixel coordinates."
@_silgen_name("WINDOW_TO_FRAME_PIXEL_Y")
func WINDOW_TO_FRAME_PIXEL_Y(_ w: UnsafeMutableRawPointer?, _ y: Int32) -> Int32
/// C: `void window_box (struct window *, enum glyph_row_area, int *, int *, int *, int *)`
/// (src/dispextern.h 3574).
@_silgen_name("window_box")
func window_box(_ w: UnsafeMutableRawPointer?, _ area: Int32,
                _ x: UnsafeMutablePointer<Int32>?, _ y: UnsafeMutablePointer<Int32>?,
                _ width: UnsafeMutablePointer<Int32>?, _ height: UnsafeMutablePointer<Int32>?)

/// C: the record members this block reads directly and that no C macro or INLINE names
/// (`row->y`, `row->visible_height`): `struct glyph_row` belongs to the core, so each access is a
/// C-ABI entry named after the member - the rule `docs/ports/nsmenu.swift` 311-328 states for
/// `struct frame`'s members and `struct ns_output`'s.
@_silgen_name("glyph_row_y") func glyph_row_y(_ row: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_row_visible_height")
func glyph_row_visible_height(_ row: UnsafeMutableRawPointer?) -> Int32

/// C 641-648: `static BOOL ns_menu_bar_should_be_hidden (void)` - "True, if the menu bar should
/// be hidden."
fileprivate func ns_menu_bar_should_be_hidden() -> Bool {
  // C's `[NSApp respondsToSelector:@selector(setPresentationOptions:)]`; Swift exposes that
  // setter as the `presentationOptions` property, so the selector is the property's setter.
  !NILP(ns_auto_hide_menu_bar)
    && NSApp.responds(to: #selector(setter: NSApplication.presentationOptions))
}

/// C 650-656: `struct EmacsMargins { CGFloat top; CGFloat bottom; CGFloat left; CGFloat right; }`
/// - what the screen-margin functions pass around.  Measured rather than assumed: `sizeof 32`,
/// `align 8`, the four members at 0/8/16/24 (Evidence), and the Swift view agrees.
struct EmacsMargins {
  var top: CGFloat
  var bottom: CGFloat
  var left: CGFloat
  var right: CGFloat
}

/// C 659-709: `static struct EmacsMargins ns_screen_margins (NSScreen *screen)` - "The parts of
/// SCREEN used by the operating system."  C's parameter is an object pointer, and every caller
/// passes a real screen; the port keeps the pointer's optionality, because C's nil receiver
/// yields zeroed `NSRect`s rather than crashing (`?? NSRect.zero` below).
/// `NSTRACE`/`NSTRACE_MSG` are the empty macros at `NSTRACE_ENABLED` 0 (src/nsterm.h 297-307),
/// so neither carries a statement.
fileprivate func ns_screen_margins(_ screen: NSScreen?) -> EmacsMargins {
  var margins = EmacsMargins(top: 0, bottom: 0, left: 0, right: 0)

  let screenFrame = screen?.frame ?? NSRect.zero
  let screenVisibleFrame = screen?.visibleFrame ?? NSRect.zero

  /* Sometimes, visibleFrame isn't up-to-date with respect to a hidden
     menu bar, check this explicitly.  */
  if ns_menu_bar_should_be_hidden() {
    margins.top = 0
  } else {
    let frameTop = screenFrame.origin.y + screenFrame.size.height
    let visibleFrameTop = (screenVisibleFrame.origin.y
                           + screenVisibleFrame.size.height)

    margins.top = frameTop - visibleFrameTop
  }

  do {
    let frameRight = screenFrame.origin.x + screenFrame.size.width
    let visibleFrameRight = (screenVisibleFrame.origin.x
                             + screenVisibleFrame.size.width)
    margins.right = frameRight - visibleFrameRight
  }

  margins.bottom = screenVisibleFrame.origin.y - screenFrame.origin.y
  margins.left = screenVisibleFrame.origin.x - screenFrame.origin.x

  return margins
}

/* A screen margin between 1 and DOCK_IGNORE_LIMIT (inclusive) is
   assumed to contain a hidden dock.  macOS currently use 4 pixels for
   this, however, to be future compatible, a larger value is used.  */
/// C 708: `#define DOCK_IGNORE_LIMIT 6` - the `int` the comparisons below promote.
fileprivate let DOCK_IGNORE_LIMIT: CGFloat = 6

/// C 710-753: `static struct EmacsMargins ns_screen_margins_ignoring_hidden_dock (NSScreen
/// *screen)` - "The parts of SCREEN used by the operating system, excluding the parts reserved
/// for a hidden dock."
fileprivate func ns_screen_margins_ignoring_hidden_dock(_ screen: NSScreen?) -> EmacsMargins {
  var margins = ns_screen_margins(screen)

  /* macOS (currently) reserved 4 pixels along the edge where a hidden
     dock is located.  Unfortunately, it's not possible to find the
     location and information about if the dock is hidden.  Instead,
     it is assumed that if the margin of an edge is less than
     DOCK_IGNORE_LIMIT, it contains a hidden dock.  */
  if margins.left <= DOCK_IGNORE_LIMIT {
    margins.left = 0
  }
  if margins.right <= DOCK_IGNORE_LIMIT {
    margins.right = 0
  }
  if margins.top <= DOCK_IGNORE_LIMIT {
    margins.top = 0
  }
  /* Note: This doesn't occur in current versions of macOS, but
     included for completeness and future compatibility.  */
  if margins.bottom <= DOCK_IGNORE_LIMIT {
    margins.bottom = 0
  }

  return margins
}

/// C 754-772: `static CGFloat ns_menu_bar_height (NSScreen *screen)` - "The height of the menu
/// bar, if visible."  (The note that follows it in C - not to use this when fullscreen is
/// enabled - is C's own text and is kept with the function.)
fileprivate func ns_menu_bar_height(_ screen: NSScreen?) -> CGFloat {
  let margins = ns_screen_margins(screen)
  return margins.top
}

/// C 773-807: `static NSRect ns_parent_window_rect (struct frame *f)` - "Get the frame rect, in
/// system coordinates, of the parent window or, if there is no parent window, the main screen."
///
/// C's three arms are all `NS_IMPL_COCOA` variants of getting the rect into screen coordinates;
/// the live one is the last (`MAC_OS_X_VERSION_MIN_REQUIRED` is 260000 here, so the
/// `convertBaseToScreen:` fallbacks are dead text): `[[parentView window]
/// convertRectToScreen:parentRect]`.  A nil window yields a zeroed `NSRect` in C, which is what
/// `?? NSRect.zero` reads.
fileprivate func ns_parent_window_rect(_ f: UnsafeMutableRawPointer?) -> NSRect {
  var parentRect: NSRect

  if FRAME_PARENT_FRAME(f) != nil {
    let parentView = FRAME_NS_VIEW(FRAME_PARENT_FRAME(f))
    parentRect = parentView!.convert(parentView!.frame, to: nil)
    parentRect = parentView?.window?.convertToScreen(parentRect) ?? NSRect.zero
  } else {
    parentRect = NSScreen.screens[0].frame
  }

  return parentRect
}

/* Calculate system coordinates of the left and top of the parent
   window or, if there is no parent window, the main screen.  */
/// C 803-804: `#define NS_PARENT_WINDOW_LEFT_POS(f) NSMinX (ns_parent_window_rect (f))` and
/// `#define NS_PARENT_WINDOW_TOP_POS(f) NSMaxY (ns_parent_window_rect (f))`.
func NS_PARENT_WINDOW_LEFT_POS(_ f: UnsafeMutableRawPointer?) -> CGFloat {
  NSMinX(ns_parent_window_rect(f))
}
func NS_PARENT_WINDOW_TOP_POS(_ f: UnsafeMutableRawPointer?) -> CGFloat {
  NSMaxY(ns_parent_window_rect(f))
}

/// C 808-827: `static NSRect ns_row_rect (struct window *w, struct glyph_row *row, enum
/// glyph_row_area area)` - "Get the row as an NSRect."
fileprivate func ns_row_rect(_ w: UnsafeMutableRawPointer?, _ row: UnsafeMutableRawPointer?,
                             _ area: Int32) -> NSRect {
  var window_x: Int32 = 0, window_y: Int32 = 0, window_width: Int32 = 0

  window_box(w, area, &window_x, &window_y, &window_width, nil)

  var rect = NSRect.zero
  rect.origin.x = CGFloat(window_x)
  rect.origin.y = CGFloat(WINDOW_TO_FRAME_PIXEL_Y(w, max(0, glyph_row_y(row))))
  rect.origin.y = max(rect.origin.y, CGFloat(window_y))
  rect.size.width = CGFloat(window_width)
  rect.size.height = CGFloat(glyph_row_visible_height(row))

  return rect
}

/// C 828-832 (with the `#if` chain that follows): `double ns_frame_scale_factor (struct frame
/// *f)`.  Declared src/nsterm.h 1260 and called from src/nsfns.m, so it is a `@_cdecl` export.
/// The live arm is the last: `MAC_OS_X_VERSION_MIN_REQUIRED` is 260000, so
/// `[[FRAME_NS_VIEW (f) window] backingScaleFactor]`; the GNUstep `userSpaceScaleFactor` arm and
/// the `respondsToSelector:@selector(backingScaleFactor:)` arm are the dead text recorded here.
/// A nil view or window yields 0.0 through C's nil receiver, which `?? 0` reads.
@_cdecl("ns_frame_scale_factor")
func ns_frame_scale_factor(_ f: UnsafeMutableRawPointer?) -> Double {
  Double(FRAME_NS_VIEW(f)?.window?.backingScaleFactor ?? 0)
}

/* ==========================================================================

    Focus (clipping) and screen update

   ========================================================================== */

//
// Window constraining
// -------------------
//
// To ensure that the windows are not placed under the menu bar, they
// are typically moved by the call-back constrainFrameRect. However,
// by overriding it, it's possible to inhibit this, leaving the window
// in it's original position.
//
// It's possible to hide the menu bar. However, technically, it's only
// possible to hide it when the application is active. To ensure that
// this work properly, the menu bar and window constraining are
// deferred until the application becomes active.
//
// Even though it's not possible to manually move a window above the
// top of the screen, it is allowed if it's done programmatically,
// when the menu is hidden. This allows the editable area to cover the
// full screen height.
//
// Test cases
// ----------
//
// Use the following extra files:
//
//    init.el:
//       ;; Hide menu and place frame slightly above the top of the screen.
//       (setq ns-auto-hide-menu-bar t)
//       (set-frame-position (selected-frame) 0 -20)
//
// Test 1:
//
//    emacs -Q -l init.el
//
//    Result: No menu bar, and the title bar should be above the screen.
//
// Test 2:
//
//    emacs -Q
//
//    Result: Menu bar visible, frame placed immediately below the menu.
//

/// C 892-991: `static NSRect constrain_frame_rect(NSRect frameRect, bool isFullscreen)` - the
/// window constrainer the comment above documents.  C's `#if 0` block at 962-985 (the disabled
/// "don't stretch below the screens" restriction) is skipped by the extractor's own rule and is
/// recorded here, not transcribed.
fileprivate func constrain_frame_rect(_ frameRect: NSRect, _ isFullscreen: Bool) -> NSRect {
  var frameRect = frameRect

  // --------------------
  // Collect information about the screen the frame is covering.
  //

  let screens = NSScreen.screens
  let nr_screens = screens.count

  // The height of the menu bar, if present in any screen the frame is
  // displayed in.
  var menu_bar_height: Int32 = 0

  // A rectangle covering all the screen the frame is displayed in.
  var multiscreenRect = NSMakeRect(0, 0, 0, 0)
  for i in 0..<nr_screens {
    let s = screens[i]
    let scrRect = s.frame

    if NSIntersectionRect(frameRect, scrRect).size.height != 0 {
      multiscreenRect = NSUnionRect(multiscreenRect, scrRect)

      if !isFullscreen {
        let screen_menu_bar_height = ns_menu_bar_height(s)
        menu_bar_height = Int32(max(CGFloat(menu_bar_height), screen_menu_bar_height))
      }
    }
  }

  if multiscreenRect.size.width == 0
       || multiscreenRect.size.height == 0 {
    // Failed to find any monitor, give up.
    return frameRect
  }

  // --------------------
  // Find a suitable placement.
  //

  if ns_menu_bar_should_be_hidden() {
    // When the menu bar is hidden, the user may place part of the
    // frame above the top of the screen, for example to hide the
    // title bar.
    //
    // Hence, keep the original position.
  } else {
    // Ensure that the frame is below the menu bar, or below the top
    // of the screen.
    //
    // This assume that the menu bar is placed at the top in the
    // rectangle that covers the monitors.  (It doesn't have to be,
    // but if it's not it's hard to do anything useful.)
    let topOfWorkArea = (multiscreenRect.origin.y
                         + multiscreenRect.size.height
                         - CGFloat(menu_bar_height))

    let topOfFrame = frameRect.origin.y + frameRect.size.height
    if topOfFrame > topOfWorkArea {
      frameRect.origin.y -= topOfFrame - topOfWorkArea
    }
  }

  // C's `#if 0` block (C 962-985): "Include the following section to restrict frame to the
  // screens." - the disabled "ensure frame doesn't stretch below the screens" restriction.

  return frameRect
}

// ==========================================================================
// C 1001-1211: window constraining, the menu bar's deferred visibility, and
// the macOS Accessibility Zoom support.
// ==========================================================================

/// C: `void block_input (void)` / `void unblock_input (void)` (src/blockinput.h).
@_silgen_name("block_input") func block_input()
@_silgen_name("unblock_input") func unblock_input()
/// C: `struct frame *XFRAME (Lisp_Object)` (src/frame.h) - the lane's reading
/// (docs/ports/nsfns.swift 193).
@_silgen_name("XFRAME") func XFRAME(_ a: Lisp_Object) -> UnsafeMutableRawPointer?
/// C: `INLINE bool FRAME_NS_P (struct frame *f)` (src/nsterm.h).
@_silgen_name("FRAME_NS_P") func FRAME_NS_P(_ f: UnsafeMutableRawPointer?) -> Bool
/// C: `#define CONSP(x)` / `XCAR (c)` / `XCDR (c)` (src/lisp.h) - the list walkers
/// `FOR_EACH_FRAME` is spelled with (docs/ports/nsfns.swift 179/191/192).
@_silgen_name("CONSP") func CONSP(_ x: Lisp_Object) -> Bool
@_silgen_name("XCAR") func XCAR(_ c: Lisp_Object) -> Lisp_Object
@_silgen_name("XCDR") func XCDR(_ c: Lisp_Object) -> Lisp_Object
/// C: `Vframe_list` - the `DEFVAR_LISP ("frame-list", …)` variable (src/frame.c), read here as a
/// data symbol like the other core variables this unit imports.
@_silgen_name("Vframe_list") var Vframe_list: Lisp_Object

/// **A deferred entry** (`docs/ports/nsterm.md`'s `## ABI` (4)).  `ns_constrain_all_frames`
/// (C 1001) messages this unit's own `EmacsView` `-isFullscreen`, whose class lands at C 8957 -
/// so the call goes through a C-ABI entry of the selector's name, receiver first, under a Swift
/// name that is *not* the definition's; the `@_cdecl("isFullscreen")` definition lands beside
/// the class.  When it does, `nm -u` stops listing `_isFullscreen` - that is this entry's
/// acceptance test (the lane's `macfont_driver` precedent, docs/status.md 3.220/3.231).
@_silgen_name("isFullscreen") func isFullscreen_entry(_ view: NSView?) -> Bool

/// C 1001-1030: `static void ns_constrain_all_frames (void)` - "Ensure that the menu bar doesn't
/// cover any frames."
fileprivate func ns_constrain_all_frames() {
  block_input()

  /* C 1011: `FOR_EACH_FRAME (tail, frame)` (src/frame.h 1404-1408) - the macro's own spelling:
     walk `Vframe_list` while it is a cons (docs/ports/nsfns.swift 967).  */
  var tail: Lisp_Object = Vframe_list
  while CONSP(tail) {
    let frame = XCAR(tail)
    let f = XFRAME(frame)
    if FRAME_NS_P(f) {
      let view = FRAME_NS_VIEW(f)

      if !isFullscreen_entry(view) {
        view?.window?.setFrame(constrain_frame_rect(view!.window!.frame, false),
                               display: false)
      }
    }
    tail = XCDR(tail)
  }

  unblock_input()
}

/// C 1033-1073: `static void ns_update_auto_hide_menu_bar (void)` - "Show or hide the menu bar,
/// based on user setting."  The whole body is inside `#ifdef NS_IMPL_COCOA`, the arm this ledger
/// describes.
fileprivate func ns_update_auto_hide_menu_bar() {
  block_input()

  if NSApp != nil && NSApp.isActive {
    // Note, "setPresentationOptions" triggers an error unless the application is active.
    let menu_bar_should_be_hidden = ns_menu_bar_should_be_hidden()

    if menu_bar_should_be_hidden != ns_menu_bar_is_hidden {
      // C: `NSApplicationPresentationOptions options = NSApplicationPresentationDefault;` - the
      // empty option set, whose `default` Swift spelling is unavailable (use `[]`).
      var options: NSApplication.PresentationOptions = []

      if menu_bar_should_be_hidden {
        options = [options, .autoHideMenuBar, .autoHideDock]
      }

      NSApp.presentationOptions = options

      ns_menu_bar_is_hidden = menu_bar_should_be_hidden

      if !ns_menu_bar_is_hidden {
        ns_constrain_all_frames()
      }
    }
  }

  unblock_input()
}

/// C 1075-1087: `static void ns_update_begin (struct frame *f)` - "Prepare for a grouped
/// sequence of drawing calls".  `NSTRACE_WHEN (NSTRACE_GROUP_UPDATES, …)` is empty at
/// `NSTRACE_ENABLED` 0, and C's `[view lockFocus]` on a nil view does nothing.
fileprivate func ns_update_begin(_ f: UnsafeMutableRawPointer?) {
  let view = FRAME_NS_VIEW(f)

  ns_update_auto_hide_menu_bar()

  ns_updating_frame = f
  // C's `[view lockFocus]` is itself deprecated since macOS 10.14 (AppKit prefers a subclassed
  // `-drawRect:`), and the port keeps the call: `swiftc -c` reports the deprecation, which is
  // recorded in the ledger's Evidence rather than silenced.
  view?.lockFocus()
}

/* --------------------------------------------------------------------------
   macOS Accessibility Zoom Support
   -------------------------------------------------------------------------- */
// C 1090: `#ifdef NS_IMPL_COCOA` - the whole block below is the Cocoa arm.

/// C 1093: `static BOOL ns_is_UAZoomEnabled = NO;`
fileprivate var ns_is_UAZoomEnabled: Bool = false
/// C 1094: `static unsigned long ns_UAZoomEnabled_last_called_time_ns = 0;`
fileprivate var ns_UAZoomEnabled_last_called_time_ns: UInt64 = 0
/// C 1095-1096: `static const unsigned long NS_UAZOOMENABLED_CACHE_INTERVAL_NS =
/// (unsigned long)(500 * NSEC_PER_MSEC); /* 500ms.  */`
fileprivate let NS_UAZOOMENABLED_CACHE_INTERVAL_NS: UInt64 = 500 * NSEC_PER_MSEC
/// C 1097: `static NSTimeInterval NS_UAZOOMENABLED_DEFER_INTERVAL_SECS = 0.2; /* 200ms.  */`
fileprivate var NS_UAZOOMENABLED_DEFER_INTERVAL_SECS: TimeInterval = 0.2
/// C 1098: `static NSTimer *ns_deferred_UAZoomChangeFocus_timer = nil;`
fileprivate var ns_deferred_UAZoomChangeFocus_timer: Timer? = nil

/// C 1102-1140: `static BOOL ns_ua_zoom_enabled_p (void)` - "Return the cached result of
/// UAZoomEnabled.  Refresh the cache every NS_UAZOOMENABLED_CACHE_INTERVAL_NS nanoseconds."
/// (C's comment on why the result is cached, and on `clock_gettime_nsec_np` rather than
/// `CFAbsoluteTimeGetCurrent`, is C's own text and is kept with the function.)
fileprivate func ns_ua_zoom_enabled_p() -> Bool {
  /* User-space equivalent to mach_absolute_time.  */
  let now_ns = clock_gettime_nsec_np(CLOCK_UPTIME_RAW)
  if now_ns - ns_UAZoomEnabled_last_called_time_ns
       > NS_UAZOOMENABLED_CACHE_INTERVAL_NS {
    ns_is_UAZoomEnabled = UAZoomEnabled()
    ns_UAZoomEnabled_last_called_time_ns = now_ns
  }
  return ns_is_UAZoomEnabled
}

/// C 1142-1162: `static inline CGRect ns_cg_rect_flip_y (CGRect r)` - "Convert a CGRect from
/// Cocoa screen coordinates (origin at bottom-left of the primary display) to CoreGraphics
/// coordinates (origin at top-left of the primary display)."  The C parameter is by value and
/// mutated, hence the Swift copy.
fileprivate func ns_cg_rect_flip_y(_ rect: CGRect) -> CGRect {
  var r = rect
  let mainID = CGMainDisplayID()
  if mainID == kCGNullDirectDisplay {
    return r
  }
  let primaryH = CGDisplayBounds(mainID).size.height
  if primaryH <= 0 {
    return r
  }
  r.origin.y = primaryH - r.origin.y - r.size.height
  return r
}

/* Cache cursor rects to call UAZoomChangeFocus only when the cursor
   position has changed, not merely when the cursor is blinking.
   See ns_draw_window_cursor and ns_update_end.  */
/// C 1152-1153: `static NSRect ns_UAZoom_cursor_rect_new; static NSRect
/// ns_UAZoom_cursor_rect_old;`
fileprivate var ns_UAZoom_cursor_rect_new: NSRect = .zero
fileprivate var ns_UAZoom_cursor_rect_old: NSRect = .zero

/* Track Zoom state per display cycle.  Update the macOS Zoom cursor
   position when Zoom transitions to enabled.  */
/// C 1157: `static BOOL ns_update_was_UAZoomEnabled = NO;`
fileprivate var ns_update_was_UAZoomEnabled: Bool = false

/// C 1164-1211: `static void ns_UAZoomChangeFocus (EmacsView *view, BOOL force)` - "Advise macOS
/// Accessibility Zoom UAZoomChangeFocus of a potentially new cursor position."
///
/// The receiver is this unit's `EmacsView`, but only `NSView`/`NSWindow` messages are sent to
/// it, so the parameter is the lane's `NSView?`.  A nil view or window yields a zeroed `NSRect`
/// through C's nil receiver, which the `?? NSRect.zero` reads below make explicit.
fileprivate func ns_UAZoomChangeFocus(_ view: NSView?, _ force: Bool) {
  if ns_ua_zoom_enabled_p() {
    let force = force || !ns_update_was_UAZoomEnabled
    ns_update_was_UAZoomEnabled = true
    if NSIsEmptyRect(ns_UAZoom_cursor_rect_new) {
      return
    }
    if force || !NSEqualRects(ns_UAZoom_cursor_rect_new, ns_UAZoom_cursor_rect_old) {
      ns_UAZoom_cursor_rect_old = ns_UAZoom_cursor_rect_new
      let windowRect = view?.convert(ns_UAZoom_cursor_rect_new, to: nil) ?? .zero
      let screenRect = view?.window?.convertToScreen(windowRect) ?? .zero
      var cgRect = ns_cg_rect_flip_y(NSRectToCGRect(screenRect))
      /* Some versions of macOS can ignore tiny rects, so we
         slightly expand a tiny one.  Since we care mostly about its
         origin, this should be innocuous.  */
      cgRect.size.width = max(cgRect.size.width, 6)
      cgRect.size.height = max(cgRect.size.height, 10)
      if force {
        /* UAZoomChangeFocus needs old and new cursor positions to
           be different, and also it sometimes needs a kick.  In
           both cases, we fake a cursor move followed by the real
           cursor move.  */
        var cgRectJiggle = cgRect.offsetBy(dx: 1.0, dy: 1.0)
        if UAZoomChangeFocus(&cgRectJiggle, nil,
                             UAZoomChangeFocusType(kUAZoomFocusTypeInsertionPoint)) != 0 {
          NSLog("UAZoomChangeFocus jiggle failed")
        }
      }
      if UAZoomChangeFocus(&cgRect, nil,
                           UAZoomChangeFocusType(kUAZoomFocusTypeInsertionPoint)) != 0 {
        NSLog("UAZoomChangeFocus failed")
      }
      NSAccessibility.post(element: view as Any,
                           notification: .focusedUIElementChanged)
    }
  } else {
    ns_update_was_UAZoomEnabled = false
  }
}

// ==========================================================================
// C 1212-1492: the focus pair, the visual bell, and the file's first class.
// ==========================================================================

/// C: `void NSRectClip (NSRect rect)` and `void NSRectClipList (const NSRect *rects, NSInteger
/// count)` (AppKit's NSGraphics.h 210-211) - `APPKIT_EXTERN` C functions that Swift marks
/// unavailable, so the lane reaches them by name.
@_silgen_name("NSRectClip") func NSRectClip(_ rect: NSRect)
@_silgen_name("NSRectClipList")
func NSRectClipList(_ rects: UnsafePointer<NSRect>?, _ count: Int)

/// C: the `struct mouse_highlight` bitfield member `ns_update_end` writes
/// (`MOUSE_HL_INFO (f)->mouse_face_defer = 0`; the member is src/dispextern.h 3020's `bool_bf
/// mouse_face_defer : 1`).  A bitfield cannot be written through a Swift view of the record, so
/// the access is a C-ABI entry named after the member - the `<record>_set_<member>` naming
/// `docs/ports/nsmenu.swift` 311-328 uses (`ns_output_set_in_animation`), taking C's own
/// argument (the frame, like `frame_left_pos`).
@_silgen_name("mouse_highlight_set_mouse_face_defer")
func mouse_highlight_set_mouse_face_defer(_ f: UnsafeMutableRawPointer?, _ v: Int32)

/// C: `bool visible_bell` - the `DEFVAR_BOOL ("visible-bell", …)` variable (src/dispnew.c), read
/// by `ns_ring_bell`; imported as the C data symbol the core exports.
@_silgen_name("visible_bell") var visible_bell: Bool
/// C: `#define SELECTED_FRAME() …` (src/frame.h) - the selected frame, an entry of its own name.
@_silgen_name("SELECTED_FRAME") func SELECTED_FRAME() -> UnsafeMutableRawPointer?

/// C 1212-1234: `static void ns_update_end (struct frame *f)` - "Finished a grouped sequence of
/// drawing calls".  The `[[view window] flushWindow]` arm is guarded by
/// `#if defined (NS_IMPL_GNUSTEP) || MAC_OS_X_VERSION_MIN_REQUIRED < 101400` - dead at 260000 -
/// and `ns_UAZoomChangeFocus` is the `NS_IMPL_COCOA` arm, which is live.  `NSTRACE_WHEN` is
/// empty at `NSTRACE_ENABLED` 0.
fileprivate func ns_update_end(_ f: UnsafeMutableRawPointer?) {
  let view = FRAME_NS_VIEW(f)

  /*   if (f == MOUSE_HL_INFO (f)->mouse_face_mouse_frame) */
  mouse_highlight_set_mouse_face_defer(f, 0)

  block_input()

  view?.unlockFocus()

  ns_UAZoomChangeFocus(view, false)

  unblock_input()
  ns_updating_frame = nil
}

/// C 1236-1273: `static void ns_focus (struct frame *f, NSRect *r, int n)` - "Internal: Focus on
/// given frame."  (C's own explanation of why the accumulate-rectangle behaviour is GNUstep's
/// is kept above the function.)  `r` is C's `NSRect *`, called with a real rect or NULL, so it
/// is optional here; the `#ifdef NS_IMPL_COCOA` clipping arm is the live one and the
/// `GSRectClipList` else-arm is the GNUstep dead text.
fileprivate func ns_focus(_ f: UnsafeMutableRawPointer?, _ r: UnsafePointer<NSRect>?,
                          _ n: Int32) {
  if f != ns_updating_frame {
    let view = FRAME_NS_VIEW(f)
    view?.lockFocus()
  }

  /* clipping */
  if r != nil {
    let ctx = NSGraphicsContext.current
    ctx?.saveGraphicsState()

    if n == 2 {
      NSRectClipList(r, 2)
    } else {
      NSRectClip(r!.pointee)
    }
    gsaved = true
  }
}

/// C 1282-1310: `static void ns_unfocus (struct frame *f)` - "Internal: Remove focus on given
/// frame".  The `flushWindow` arm is the same dead GNUstep/pre-10.14 guard as `ns_update_end`'s.
fileprivate func ns_unfocus(_ f: UnsafeMutableRawPointer?) {
  if gsaved {
    NSGraphicsContext.current?.restoreGraphicsState()
    gsaved = false
  }

  if f != ns_updating_frame {
    let view = FRAME_NS_VIEW(f)
    view?.unlockFocus()
  }
}

/* ==========================================================================

    Visible bell and beep.

   ========================================================================== */

// This bell implementation shows the visual bell image asynchronously
// from the rest of Emacs. This is done by adding a NSView to the
// superview of the Emacs window and removing it using a timer.
//
// Unfortunately, some Emacs operations, like scrolling, is done using
// low-level primitives that copy the content of the window, including
// the bell image. To some extent, this is handled by removing the
// image prior to scrolling and marking that the window is in need for
// redisplay.
//
// To test this code, make sure that there is no artifacts of the bell
// image in the following situations. Use a non-empty buffer (like the
// tutorial) to ensure that a scroll is performed:
//
// * Single-window: C-g C-v
//
// * Side-by-windows: C-x 3 C-g C-v
//
// * Windows above each other: C-x 2 C-g C-v

/// C 1333-1343: `@interface EmacsBell : NSImageView` - the ivars `unsigned int nestCount;`
/// (1336), `NSView * mView;` (1337) and `bool isAttached;` (1338), and the three methods
/// (`show:` 1340, `hide` 1341, `remove` 1342).  **The file's first class**, and the first of the
/// eight the ledger's Objective-C section tracks.  Each method is `@objc`, which is what makes
/// its selector reachable (`#selector(hide)` below) and mirrors the reference's Objective-C
/// methods (the reading `docs/ports/nsmenu.swift` 849 uses); the ivars become stored properties
/// with C's values.
class EmacsBell: NSImageView {
  /// C 1336: `unsigned int nestCount;` - "Number of currently active bells."
  fileprivate var nestCount: UInt32 = 0
  /// C 1337: `NSView * mView;`
  fileprivate var mView: NSView? = nil
  /// C 1338: `bool isAttached;`
  fileprivate var isAttached: Bool = false

  /// C 1347-1371: `- (id)init`.  C's `init` on an `NSView` is AppKit's designated
  /// `-initWithFrame:`, hence the override's parameter; `required init(coder:)` below is Swift's
  /// requirement on a view subclass, and C has no unarchiving path for this class.
  ///
  /// The `NS_IMPL_GNUSTEP && !HAVE_DECL_NSIMAGENAMECAUTION` arm is the GNUstep-only fallback
  /// (dead here); the live arm names AppKit's caution image and scales it by five.  C's
  /// `self.image` can be nil there and the messages to it would do nothing, which is what the
  /// optional chaining reads.
  override init(frame frameRect: NSRect) {
    super.init(frame: frameRect)

    image = NSImage(named: NSImage.cautionName)
    image?.size = NSSize(width: (image?.size.width ?? 0) * 5,
                         height: (image?.size.height ?? 0) * 5)
  }

  /// NSImageView's own `-initWithCoder:`, inherited as-is (the class is only ever built in code
  /// by `ns_ring_bell`); the port's designated override above makes Swift want it spelled out,
  /// the way `docs/ports/nsmenu.swift` 843 does.
  required init?(coder: NSCoder) {
    super.init(coder: coder)
  }

  /// C 1373-1398: `- (void)show:(NSView *)view`.
  @objc func show(_ view: NSView) {
    // Show the image, unless it's already shown.
    if nestCount == 0 {
      let rect = view.bounds
      var pos = NSPoint.zero
      pos.x = rect.origin.x + (rect.size.width - (image?.size.width ?? 0)) / 2
      pos.y = rect.origin.y + (rect.size.height - (image?.size.height ?? 0)) / 2

      setFrameOrigin(pos)
      setFrameSize(image?.size ?? .zero)

      isAttached = true
      mView = view
      view.window?.contentView?.addSubview(self, positioned: .above, relativeTo: nil)
    }

    nestCount += 1

    perform(#selector(hide), with: self, afterDelay: 0.5)
  }

  /// C 1402-1415: `- (void)hide`.  (C's note that this method's trace output never shows is
  /// C's own text and stays with it.)
  @objc func hide() {
    if nestCount > 0 {
      nestCount -= 1
    }

    // Remove the image once the last bell became inactive.
    if nestCount == 0 {
      remove()
    }
  }

  /// C 1418-1429: `-(void)remove`.
  @objc func remove() {
    if isAttached {
      removeFromSuperview()
      mView?.needsDisplay = true
      isAttached = false
    }
  }
}

/// C 1433: `static EmacsBell * bell_view = nil;`
fileprivate var bell_view: EmacsBell? = nil

/// C 1436-1469: `static void ns_ring_bell (struct frame *f)` - the "Beep" routine.
fileprivate func ns_ring_bell(_ f: UnsafeMutableRawPointer?) {
  if visible_bell {
    let frame = SELECTED_FRAME()

    if bell_view == nil {
      // C's `bell_view = [[EmacsBell alloc] init]; [bell_view retain];` - the global owns the
      // object, which is what ARC's strong reference does; the explicit `retain` is C's manual
      // memory management for the same ownership.  `init` on an `NSView` is `initWithFrame:`,
      // and C's `[[NSView alloc] init]` yields a zero frame.
      bell_view = EmacsBell(frame: .zero)
    }

    block_input()

    let view = FRAME_NS_VIEW(frame)
    if view != nil {
      bell_view!.show(view!)
    }

    unblock_input()
  } else {
    // C: `NSBeep ();` - Swift's AppKit spells it `NSSound.beep ()` (measured: `NSBeep` is not in
    // scope in Swift).
    NSSound.beep()
  }
}

// C 1471-1491: `hide_bell` is **dead** in this build - its guard is
// `#if !defined (NS_IMPL_COCOA) || MAC_OS_X_VERSION_MIN_REQUIRED < 101400`, and this lane's
// target both defines `NS_IMPL_COCOA` and has a minimum of 260000.  Its one call site (C 9388,
// inside the same guard) is dead with it, and the ledger's box for the function is ticked with
// that reason rather than transcribed.

// ==========================================================================
// C 1493-1743: the focus/raise/lower hooks and the frame-visibility hooks.
// ==========================================================================

/// C: `struct ns_display_info *FRAME_DISPLAY_INFO (struct frame *f)` (src/nsterm.h 1032's macro) -
/// the display-info accessor, reached as an entry like the other frame macros; the members it
/// leads to are entries of their own (`<record>_<member>`, the naming
/// `docs/ports/nsmenu.swift` 311-328 sets and `nsfns.swift`'s
/// `ns_display_info_set_horizontal_scroll_bar_cursor` already uses).
@_silgen_name("FRAME_DISPLAY_INFO")
func FRAME_DISPLAY_INFO(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
/// C: the `struct ns_display_info` members this block reads or writes.
@_silgen_name("ns_display_info_ns_focus_frame")
func ns_display_info_ns_focus_frame(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("ns_display_info_highlight_frame")
func ns_display_info_highlight_frame(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("ns_display_info_set_highlight_frame")
func ns_display_info_set_highlight_frame(_ f: UnsafeMutableRawPointer?,
                                         _ v: UnsafeMutableRawPointer?)
/// C: `INLINE bool FRAMEP (Lisp_Object)` (src/lisp.h) and the frame-writing macro
/// `XSETFRAME (Lisp_Object, struct frame *)` (src/frame.h) - the shapes
/// `docs/ports/nsfns.swift` 182 and `docs/ports/nsselect.swift` 181 use.
@_silgen_name("FRAMEP") func FRAMEP(_ x: Lisp_Object) -> Bool
@_silgen_name("XSETFRAME")
func XSETFRAME(_ a: UnsafeMutablePointer<Lisp_Object>, _ b: UnsafeMutableRawPointer?)
/// C: `Lisp_Object FRAME_FOCUS_FRAME (struct frame *f)` (src/frame.h) and its setter
/// `INLINE void fset_focus_frame (struct frame *f, Lisp_Object val)` - the family
/// `docs/ports/nsmenu.swift` 307 already reaches (`fset_menu_bar_items`).
@_silgen_name("FRAME_FOCUS_FRAME")
func FRAME_FOCUS_FRAME(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("fset_focus_frame")
func fset_focus_frame(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object)
/// C: `INLINE bool FRAME_LIVE_P (struct frame *)` / `FRAME_VISIBLE_P` (src/frame.h, nsterm.h) and
/// the two `SET_FRAME_*` setters.
@_silgen_name("FRAME_LIVE_P") func FRAME_LIVE_P(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("FRAME_VISIBLE_P") func FRAME_VISIBLE_P(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("SET_FRAME_VISIBLE")
func SET_FRAME_VISIBLE(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("SET_FRAME_ICONIFIED")
func SET_FRAME_ICONIFIED(_ f: UnsafeMutableRawPointer?, _ v: Int32)
/// C: `bool FRAME_NO_FOCUS_ON_MAP (struct frame *)` (src/frame.h) and the enumerator
/// `FULLSCREEN_NONE` (src/nsterm.h; the entry `docs/ports/nsfns.swift` 513 uses).
@_silgen_name("FRAME_NO_FOCUS_ON_MAP")
func FRAME_NO_FOCUS_ON_MAP(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("FULLSCREEN_NONE") func FULLSCREEN_NONE() -> Int32
/// C: the `struct frame` members this block reads directly and that no C macro names
/// (`f->no_accept_focus`, `f->want_fullscreen`, `f->left_pos`, `f->top_pos`) - one entry per
/// member, the rule `docs/ports/nsmenu.swift` 311-328 states; `frame_left_pos`/`frame_top_pos`
/// are the names that file already uses.
@_silgen_name("frame_no_accept_focus")
func frame_no_accept_focus(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("frame_want_fullscreen")
func frame_want_fullscreen(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_left_pos") func frame_left_pos(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_top_pos") func frame_top_pos(_ f: UnsafeMutableRawPointer?) -> Int32
/// C: `void check_window_system (struct frame *)` (src/frame.h 1504) and
/// `void gui_update_cursor (struct frame *f, bool on_p)` (src/dispextern.h 3691).
@_silgen_name("check_window_system") func check_window_system(_ f: UnsafeMutableRawPointer?)
@_silgen_name("gui_update_cursor")
func gui_update_cursor(_ f: UnsafeMutableRawPointer?, _ on_p: Bool)

// **Deferred entries** (`## ABI` (4)): call sites that precede the definition, which is written
// at its own C line under the selector's/function's own Swift name with `@_cdecl`.  The first
// four wait for `EmacsView`/`EmacsWindow` (C 7039/9756); the last two are this unit's own
// *functions*, whose C forward declarations (`ns_set_frame_alpha` at C 1622) are the reference's
// own acknowledgement of the same forward use.
@_silgen_name("fsIsNative") func fsIsNative_entry(_ view: NSView?) -> Bool
@_silgen_name("handleFS") func handleFS_entry(_ view: NSView?)
@_silgen_name("setParentChildRelationships") func setParentChildRelationships_entry(_ window: NSWindow?)
@_silgen_name("ns_set_offset")
func ns_set_offset_entry(_ f: UnsafeMutableRawPointer?, _ xoff: Int32, _ yoff: Int32,
                         _ type: Int32)
@_silgen_name("ns_set_frame_alpha") func ns_set_frame_alpha_entry(_ f: UnsafeMutableRawPointer?)

/// C 1493-1508: `static Lisp_Object ns_get_focus_frame (struct frame *f)` - "External (hook)".
fileprivate func ns_get_focus_frame(_ f: UnsafeMutableRawPointer?) -> Lisp_Object {
  var lisp_focus: Lisp_Object = Qnil()

  let focus = ns_display_info_ns_focus_frame(f)

  if focus == nil {
    return Qnil()
  }

  XSETFRAME(&lisp_focus, focus)
  return lisp_focus
}

/// C 1510-1525: `static void ns_focus_frame (struct frame *f, bool noactivate)` - "External
/// (hook)".
fileprivate func ns_focus_frame(_ f: UnsafeMutableRawPointer?, _ noactivate: Bool) {
  if ns_display_info_ns_focus_frame(f) != f {
    let view = FRAME_NS_VIEW(f)
    block_input()
    NSApp.activate(ignoringOtherApps: true)
    view?.window?.makeKeyAndOrderFront(view)
    unblock_input()
  }
}

/// C 1527-1546: `static void ns_raise_frame (struct frame *f, BOOL make_key)` - "Bring window to
/// foreground and if make_key is YES, give it focus."
fileprivate func ns_raise_frame(_ f: UnsafeMutableRawPointer?, _ make_key: Bool) {
  check_window_system(f)
  let view = FRAME_NS_VIEW(f)
  block_input()
  if FRAME_VISIBLE_P(f) {
    if make_key && !frame_no_accept_focus(f) {
      view?.window?.makeKeyAndOrderFront(NSApp)
    } else {
      view?.window?.orderFront(NSApp)
    }
  }
  unblock_input()
}

/// C 1549-1563: `static void ns_lower_frame (struct frame *f)` - "Send window to back".
fileprivate func ns_lower_frame(_ f: UnsafeMutableRawPointer?) {
  check_window_system(f)
  let view = FRAME_NS_VIEW(f)
  block_input()
  view?.window?.orderBack(NSApp)
  unblock_input()
}

/// C 1566-1578: `static void ns_frame_raise_lower (struct frame *f, bool raise)` - "External
/// (hook)".  `NSTRACE` is empty.
fileprivate func ns_frame_raise_lower(_ f: UnsafeMutableRawPointer?, _ raise: Bool) {
  if raise {
    ns_raise_frame(f, true)
  } else {
    ns_lower_frame(f)
  }
}

/// C 1582-1620: `static void ns_frame_rehighlight (struct frame *frame)` - "External (hook):
/// called on things like window switching within frame".  `ns_set_frame_alpha` is this unit's own
/// function defined at C 8550, hence the deferred entry (C forward-declares it at C 1622).
fileprivate func ns_frame_rehighlight(_ frame: UnsafeMutableRawPointer?) {
  let old_highlight = ns_display_info_highlight_frame(frame)

  if ns_display_info_ns_focus_frame(frame) != nil {
    let focus = ns_display_info_ns_focus_frame(frame)
    ns_display_info_set_highlight_frame(frame,
      FRAMEP(FRAME_FOCUS_FRAME(focus)) ? XFRAME(FRAME_FOCUS_FRAME(focus)) : focus)
    if !FRAME_LIVE_P(ns_display_info_highlight_frame(frame)) {
      fset_focus_frame(focus, Qnil())
      ns_display_info_set_highlight_frame(frame, focus)
    }
  } else {
    ns_display_info_set_highlight_frame(frame, nil)
  }

  if ns_display_info_highlight_frame(frame) != nil
       && ns_display_info_highlight_frame(frame) != old_highlight {
    if old_highlight != nil {
      gui_update_cursor(old_highlight, true)
      ns_set_frame_alpha_entry(old_highlight)
    }
    if ns_display_info_highlight_frame(frame) != nil {
      gui_update_cursor(ns_display_info_highlight_frame(frame), true)
      ns_set_frame_alpha_entry(ns_display_info_highlight_frame(frame))
    }
  }
}

/// C 1624-1676: `void ns_make_frame_visible (struct frame *f)` - "External: Show the window
/// (X11 semantics)".  Declared src/nsterm.h 1226.
///
/// The calls into this unit's own later classes (`fsIsNative`, `isFullscreen`, `handleFS`,
/// `setParentChildRelationships`) are deferred entries, and so is `ns_set_offset` (defined at
/// C 1854).
@_cdecl("ns_make_frame_visible")
func ns_make_frame_visible(_ f: UnsafeMutableRawPointer?) {
  /* XXX: at some points in past this was not needed, as the only place that
     called this (frame.c:Fraise_frame ()) also called raise_lower;
     if this ends up the case again, comment this out again.  */
  if !FRAME_VISIBLE_P(f) {
    let view = FRAME_NS_VIEW(f)
    let window = view?.window

    SET_FRAME_VISIBLE(f, true)
    ns_raise_frame(f, !FRAME_NO_FOCUS_ON_MAP(f))

    /* Making a new frame from a fullscreen frame will make the new frame
       fullscreen also.  So skip handleFS as this will print an error.  */
    if fsIsNative_entry(view) && isFullscreen_entry(view) {
      return
    }

    if frame_want_fullscreen(f) != FULLSCREEN_NONE() {
      block_input()
      handleFS_entry(view)
      unblock_input()
    }

    /* A child window cannot remain attached while hidden.  ... (C's own note on
       `-orderOut:` and the parent->child relationship is kept with the call.)  */
    if window?.parent == nil && FRAME_PARENT_FRAME(f) != nil {
      block_input()
      setParentChildRelationships_entry(window)
      unblock_input()

      /* If the parent frame moved while the child frame was
         invisible, the child frame's position won't have been
         updated.  Make sure it's in the right place now.  */
      ns_set_offset_entry(f, frame_left_pos(f), frame_top_pos(f), 0)
    }
  }
}

/// C 1678-1691: `void ns_make_frame_invisible (struct frame *f)` - "Hide the window (X11
/// semantics)".  Declared src/nsterm.h 1227.
@_cdecl("ns_make_frame_invisible")
func ns_make_frame_invisible(_ f: UnsafeMutableRawPointer?) {
  check_window_system(f)
  let view = FRAME_NS_VIEW(f)
  view?.window?.orderOut(NSApp)
  SET_FRAME_VISIBLE(f, false)
  SET_FRAME_ICONIFIED(f, 0)
}

/// C 1693-1702: `static void ns_make_frame_visible_invisible (struct frame *f, bool visible)` -
/// "External (hook)".
fileprivate func ns_make_frame_visible_invisible(_ f: UnsafeMutableRawPointer?, _ visible: Bool) {
  if visible {
    ns_make_frame_visible(f)
  } else {
    ns_make_frame_invisible(f)
  }
}

/// C 1705-1742: `void ns_iconify_frame (struct frame *f)` - "External (hook): Iconify window".
/// Declared src/nsterm.h 1228.
@_cdecl("ns_iconify_frame")
func ns_iconify_frame(_ f: UnsafeMutableRawPointer?) {
  check_window_system(f)
  let view = FRAME_NS_VIEW(f)

  if ns_display_info_highlight_frame(f) == f {
    ns_display_info_set_highlight_frame(f, nil)
  }

  if (view?.window?.windowNumber ?? 0) <= 0 {
    /* The window is still deferred.  Make it very small, bring it
       on screen and order it out.  */
    let s = NSRect(x: 100, y: 100, width: 0, height: 0)
    let t = view?.window?.frame ?? .zero
    view?.window?.setFrame(s, display: false)
    view?.window?.orderBack(NSApp)
    view?.window?.orderOut(NSApp)
    view?.window?.setFrame(t, display: false)
  }

  /* Processing input while Emacs is being minimized can cause a
     crash, so block it for the duration.  */
  block_input()
  view?.window?.miniaturize(NSApp)
  unblock_input()
}

// ==========================================================================
// C 1744-1876: the frame-resource teardown and the window-placement pair.
// ==========================================================================

/// C: `Mouse_HLInfo *MOUSE_HL_INFO (struct frame *f)` (src/frame.h 1028's macro) and the
/// `struct mouse_highlight` member this block reads.  A member entry takes the handle C's own
/// expression starts from - here the `hlinfo` local, in round 6's write the frame - and its name
/// always spells the record and the member.
@_silgen_name("MOUSE_HL_INFO")
func MOUSE_HL_INFO(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("mouse_highlight_mouse_face_mouse_frame")
func mouse_highlight_mouse_face_mouse_frame(_ hlinfo: UnsafeMutableRawPointer?)
  -> UnsafeMutableRawPointer?
/// C: `void reset_mouse_highlight (Mouse_HLInfo *)` (src/dispextern.h).
@_silgen_name("reset_mouse_highlight")
func reset_mouse_highlight(_ hlinfo: UnsafeMutableRawPointer?)
/// C: the `struct ns_display_info` member write `dpyinfo->ns_focus_frame = 0` - the setter of
/// round 7's reader.
@_silgen_name("ns_display_info_set_ns_focus_frame")
func ns_display_info_set_ns_focus_frame(_ f: UnsafeMutableRawPointer?,
                                        _ v: UnsafeMutableRawPointer?)
/// C: `struct ns_output`'s member `f->output_data.ns->miniimage` - the `ns_output_<member>`
/// family `docs/ports/nsmenu.swift` 311-328 names (`ns_output_in_animation`), and the
/// `f->output_data.ns` member itself, whose write `nsfns.swift` 717 names
/// `frame_set_display_info` for the same kind of access.
@_silgen_name("ns_output_miniimage")
func ns_output_miniimage(_ f: UnsafeMutableRawPointer?) -> NSImage?
@_silgen_name("frame_set_output_data_ns")
func frame_set_output_data_ns(_ f: UnsafeMutableRawPointer?, _ v: UnsafeMutableRawPointer?)
/// C: the same member read (`xfree (f->output_data.ns);`, C 1776).
@_silgen_name("frame_output_data_ns")
func frame_output_data_ns(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
/// C: `void free_frame_menubar (struct frame *)` (src/frame.h 1883),
/// `void free_frame_faces (struct frame *)` (src/dispextern.h) and `void xfree (void *)`
/// (src/lisp.h).
@_silgen_name("free_frame_menubar") func free_frame_menubar(_ f: UnsafeMutableRawPointer?)
@_silgen_name("free_frame_faces") func free_frame_faces(_ f: UnsafeMutableRawPointer?)
@_silgen_name("xfree") func xfree(_ p: UnsafeMutableRawPointer?)
/// C: `AVOID emacs_abort (void)` (src/lisp.h) - it does not return.
@_silgen_name("emacs_abort") func emacs_abort() -> Never
/// C: the `struct frame` member `f->size_hint_flags` (read, and read-modify-written by
/// `ns_set_offset`) with the two enumerators it is tested against.
@_silgen_name("frame_size_hint_flags")
func frame_size_hint_flags(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_set_size_hint_flags")
func frame_set_size_hint_flags(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("XNegative") func XNegative() -> Int32
@_silgen_name("YNegative") func YNegative() -> Int32

/// **A deferred entry**: `[(EmacsView *)view setWindowClosing: YES]` (C 1757) waits for
/// `EmacsView -setWindowClosing:` (C 7047), so the call site takes the entry and the
/// `@_cdecl("setWindowClosing")` definition lands beside the class.
@_silgen_name("setWindowClosing")
func setWindowClosing_entry(_ view: NSView?, _ closing: Bool)

/// C 1744-1780: `void ns_free_frame_resources (struct frame *f)` - "Free resources of frame F."
/// Declared src/nsterm.h 1187, so it keeps C's name under `@_cdecl`.  C's `[x release]` calls
/// are the manual memory management ARC replaces; where the reference *drops* a reference it
/// owned (`[f->output_data.ns->miniimage release]`, `[view release]`), the port performs the
/// same decrement through the runtime (`Unmanaged.passUnretained (x).release ()`), the reading
/// round 3's `ns_release_object` uses.
@_cdecl("ns_free_frame_resources")
func ns_free_frame_resources(_ f: UnsafeMutableRawPointer?) {
  check_window_system(f)
  let view = FRAME_NS_VIEW(f)

  setWindowClosing_entry(view, true) /* may not have been informed */

  block_input()

  free_frame_menubar(f)
  free_frame_faces(f)

  if f == ns_display_info_ns_focus_frame(f) {
    ns_display_info_set_ns_focus_frame(f, nil)
  }
  if f == ns_display_info_highlight_frame(f) {
    ns_display_info_set_highlight_frame(f, nil)
  }
  let hlinfo = MOUSE_HL_INFO(f)
  if f == mouse_highlight_mouse_face_mouse_frame(hlinfo) {
    reset_mouse_highlight(hlinfo)
  }

  if ns_output_miniimage(f) != nil {
    Unmanaged.passUnretained(ns_output_miniimage(f)!).release()
  }

  view?.window?.close()
  view?.removeFromSuperview()
  Unmanaged.passUnretained(view!).release()

  xfree(frame_output_data_ns(f))
  frame_set_output_data_ns(f, nil)

  unblock_input()
}

/// C 1782-1810: `static void ns_destroy_window (struct frame *f)` - "External: Delete the
/// window".  `emacs_abort` does not return, and C's second `FRAME_PARENT_FRAME (f)` test is the
/// reference's own "pacify a incorrect GCC warning" branch.
fileprivate func ns_destroy_window(_ f: UnsafeMutableRawPointer?) {
  check_window_system(f)

  /* If this frame has a parent window, detach it as not doing so can
     cause a crash in GNUStep.  */
  if FRAME_PARENT_FRAME(f) != nil {
    let child = FRAME_NS_VIEW(f)?.window
    var parent: NSWindow?

    /* Pacify a incorrect GCC warning about FRAME_PARENT_FRAME (f)
       being NULL.  */
    if FRAME_PARENT_FRAME(f) != nil {
      parent = FRAME_NS_VIEW(FRAME_PARENT_FRAME(f))?.window
    } else {
      emacs_abort()
    }

    parent?.removeChildWindow(child!)
  }

  FRAME_NS_VIEW(f)?.window?.close()
  ns_free_frame_resources(f)
  ns_window_num -= 1
}

/// C 1812-1852: `static NSPoint compute_offset (struct frame *f, NSView *view, int xoff, int
/// yoff)`.  The `NS_IMPL_GNUSTEP` menu-overlap clamp is the GNUstep arm (dead here).
fileprivate func compute_offset(_ f: UnsafeMutableRawPointer?, _ view: NSView?,
                                _ xoff: Int32, _ yoff: Int32) -> NSPoint {
  /* If there is no parent frame then just convert to screen
     coordinates, UNLESS we have negative values, in which case I
     think it's best to position from the bottom and right of the
     current screen rather than the main screen or whole display.  */

  let parentRect = ns_parent_window_rect(f)
  let windowFrame = view?.window?.frame ?? .zero
  var topLeft = NSPoint.zero

  if frame_size_hint_flags(f) & XNegative() != 0 {
    topLeft.x = NSMaxX(parentRect) - NSWidth(windowFrame) + CGFloat(xoff)
  } else if FRAME_PARENT_FRAME(f) != nil {
    topLeft.x = NSMinX(parentRect) + CGFloat(xoff)
  } else {
    topLeft.x = CGFloat(xoff)
  }

  if frame_size_hint_flags(f) & YNegative() != 0 {
    topLeft.y = NSMinY(parentRect) + NSHeight(windowFrame) - CGFloat(yoff)
  } else if FRAME_PARENT_FRAME(f) != nil {
    topLeft.y = NSMaxY(parentRect) - CGFloat(yoff)
  } else {
    topLeft.y = NSMaxY(NSScreen.screens[0].frame) - CGFloat(yoff)
  }

  return topLeft
}

/// C 1854-1875: `void ns_set_offset (struct frame *f, int xoff, int yoff, int change_grav)` -
/// "External: Position the window".  Declared src/nsterm.h 1126 - **this is the definition the
/// deferred entry `ns_set_offset_entry` (first called at C 1691) waits for**, and its signature
/// is the one `nsfns.swift` 600 imports.
@_cdecl("ns_set_offset")
func ns_set_offset(_ f: UnsafeMutableRawPointer?, _ xoff: Int32, _ yoff: Int32,
                   _ change_grav: Int32) {
  let view = FRAME_NS_VIEW(f)

  if view == nil {
    return
  }

  block_input()

  let topLeft = compute_offset(f, view, xoff, yoff)
  view?.window?.setFrameTopLeftPoint(topLeft)
  frame_set_size_hint_flags(f, frame_size_hint_flags(f) & ~(XNegative() | YNegative()))

  unblock_input()
}

// ==========================================================================
// C 1877-1956: the window-sizing pair.
// ==========================================================================

/// C: `struct ns_output`'s member `f->output_data.ns->zooming` (read, and reset to 0) - the
/// `ns_output_<member>` family `docs/ports/nsmenu.swift` 311-328 names.
@_silgen_name("ns_output_zooming") func ns_output_zooming(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("ns_output_set_zooming")
func ns_output_set_zooming(_ f: UnsafeMutableRawPointer?, _ v: Bool)
/// C: `void change_frame_size (struct frame *, int, int, bool, bool, bool)` (src/dispnew.h) -
/// C's `NO` is `false`.
@_silgen_name("change_frame_size")
func change_frame_size(_ f: UnsafeMutableRawPointer?, _ new_width: Int32, _ new_height: Int32,
                       _ pretend: Bool, _ delay: Bool, _ safe: Bool)
/// C: `#define FRAME_NS_TITLEBAR_HEIGHT(f) …` (src/nsterm.h 1062-1068, "Compute pixel height of
/// the frame's titlebar") and `#define FRAME_TOOLBAR_HEIGHT(f) …` (src/nsterm.h 1071-1079).
/// **Neither has a symbol anywhere in the port** (measured: `nm -g` over
/// `build/emacs-ref/{src,lib}/*.o` finds none), so per the integrator's rule of status 3.251 -
/// a name with no real C symbol cannot be a C-ABI entry - this unit spells each macro's *body*
/// instead of importing its name.  The bodies need only AppKit and the existing `FRAME_NS_VIEW`
/// entry.
fileprivate func frame_ns_titlebar_height(_ f: UnsafeMutableRawPointer?) -> Int32 {
  let view = FRAME_NS_VIEW(f)
  if view == nil || NSHeight(view!.frame) == 0 {
    return 0
  }
  let window = view!.window
  return Int32(NSHeight(window!.frame)
               - NSHeight(NSWindow.contentRect(forFrameRect: window!.frame,
                                               styleMask: window!.styleMask)))
}

fileprivate func frame_toolbar_height(_ f: UnsafeMutableRawPointer?) -> Int32 {
  let window = FRAME_NS_VIEW(f)?.window
  let toolbar = window?.toolbar
  if toolbar == nil || !toolbar!.isVisible {
    return 0
  }
  return Int32(NSHeight(NSWindow.contentRect(forFrameRect: window!.frame,
                                            styleMask: window!.styleMask))
               - NSHeight(window!.contentView!.frame))
}

/// C 1877-1915: `static void ns_set_window_size (struct frame *f, bool change_gravity, int
/// width, int height)` - "Adjust window pixel size based on native sizes WIDTH and HEIGHT."
/// The `NSTRACE_*` calls are the empty macros at `NSTRACE_ENABLED` 0, so their arguments
/// (`FRAME_COLUMN_WIDTH`/`FRAME_LINE_HEIGHT`, `[window frame]`) are not evaluated here either -
/// the same dead-arm reading rounds 1 and 6 recorded.
fileprivate func ns_set_window_size(_ f: UnsafeMutableRawPointer?, _ change_gravity: Bool,
                                    _ width: Int32, _ height: Int32) {
  let view = FRAME_NS_VIEW(f)
  let window = view?.window

  if view == nil {
    return
  }

  block_input()

  var frameRect = window?.frameRect(forContentRect:
                                      NSMakeRect(0, 0, CGFloat(width), CGFloat(height))) ?? .zero

  /* Set the origin so the top left of the frame doesn't move.  */
  frameRect.origin = window?.frame.origin ?? .zero
  frameRect.origin.y += NSHeight(view!.frame) - CGFloat(height)

  if ns_output_zooming(f) {
    ns_output_set_zooming(f, false)
  }

  /* Usually it seems safe to delay changing the frame size, but when a
     series of actions are taken with no redisplay between them then we
     can end up using old values so don't delay here.  */
  change_frame_size(f, width, height, false, false, false)

  window?.setFrame(frameRect, display: false)

  unblock_input()
}

/// C 1920-1956: `static void ns_set_window_size_and_position (struct frame *f, int width, int
/// height)` - "Adjust window pixelwise size and position in one operation."
fileprivate func ns_set_window_size_and_position(_ f: UnsafeMutableRawPointer?, _ width: Int32,
                                                 _ height: Int32) {
  let view = FRAME_NS_VIEW(f)
  let window = view?.window

  if view == nil {
    return
  }

  block_input()

  /* Both window frame origin, and the rect that setFrame accepts are
     anchored to the bottom-left corner of the window.  */
  let topLeft = compute_offset(f, view, frame_left_pos(f), frame_top_pos(f))
  let bottomLeft = NSMakePoint(topLeft.x,
                               /* text-area pixels + decorations. */
                               topLeft.y - (CGFloat(height)
                                            + CGFloat(frame_ns_titlebar_height(f))
                                            + CGFloat(frame_toolbar_height(f))))
  let frameRect = window?.frameRect(forContentRect: NSMakeRect(bottomLeft.x, bottomLeft.y,
                                                               CGFloat(width),
                                                               CGFloat(height))) ?? .zero

  if ns_output_zooming(f) {
    ns_output_set_zooming(f, false)
  }

  /* Usually it seems safe to delay changing the frame size, but when a
     series of actions are taken with no redisplay between them then we
     can end up using old values so don't delay here.  */
  change_frame_size(f, width, height, false, false, false)
  window?.setFrame(frameRect, display: false)
  frame_set_size_hint_flags(f, frame_size_hint_flags(f) & ~(XNegative() | YNegative()))

  unblock_input()
}
