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
// `ns_set_window_size_and_position` (C 1920-1956).  **Round 10 lands C 1959-2398**: the seven
// frame-parameter handlers (C 1961-2116), the `NS_IMPL_COCOA` trio `ns_set_appearance_1`/
// `ns_set_appearance`/`ns_set_transparent_titlebar` (C 2120-2165), `ns_fullscreen_hook`
// (C 2169-2181), the color block (`ns_get_color` C 2191-2297, `ns_lisp_to_color` C 2300-2312,
// `ns_query_color` C 2315-2329, `ns_defined_color` C 2332-2354,
// `ns_query_frame_background_color` C 2357-2363) and **`ns_set_frame_alpha` (C 2366-2398) - the
// definition the deferred entry `ns_set_frame_alpha_entry` waited for** - so `nm -u` no longer
// lists `_ns_set_frame_alpha` either.  **Round 11 lands C 2401-2621**, the whole Mouse handling
// section: `frame_set_mouse_pixel_position` (C 2408-2430), `ns_note_mouse_movement`
// (C 2432-2472), `ns_mouse_position` (C 2475-2595) and `ns_frame_up_to_date` (C 2598-2621).
// **Round 12 lands C 2624-2761**: `ns_define_frame_cursor` (C 2624-2637), the Keyboard handling
// banner (C 2641-2645), `ns_convert_key` (C 2648-2663), `get_keysym_name` (C 2666-2677),
// `right_mod` (C 2680-2684), `nil_or_none` (C 2686-2690) and `ns_get_shifted_character`
// (C 2692-2761).  **Round 13 lands C 2789-2960**: `ns_clear_frame` (C 2788-2820, exported),
// `ns_clear_frame_area` (C 2823-2846), `ns_scroll_run` (C 2849-2916) and
// `ns_clear_under_internal_border` (C 2919-2960).  **Round 14 lands C 2963-3079**:
// `ns_after_update_window_line` (C 2963-3019), `ns_shift_glyphs_for_insert` (C 3022-3036), the
// "Character encoding and metrics" banner (C 3040-3044) and `ns_compute_glyph_string_overhangs`
// (C 3047-3079).  **Round 15 lands C 3083-3291**: the Fringe and cursor drawing banner
// (C 3083-3087), its `fringe_bmp` table (C 3089) and the section's three definitions
// `ns_define_fringe_bitmap` (C 3091-3195), `ns_destroy_fringe_bitmap` (C 3198-3202) and
// `ns_draw_fringe_bitmap` (C 3205-3291) - **no new C export** (all three are C `static`s).
// **Round 16 lands C 3294-3508**: `ns_draw_window_cursor` (C 3295-3426),
// `ns_draw_vertical_window_border` (C 3430-3449) and `ns_draw_window_divider` (C 3453-3508) -
// again no new export - so the covered range is **C 60-3508**.  **Round 17 lands C 3511-3807**
// (the hourglass pair, the Glyph drawing operations banner and the text-decoration block) and
// **round 18 lands C 3809-4177** (the box/relief sub-block and the `dumpglyphs` background
// helper) and **round 19 lands C 4179-4393** (the image sub-block: `ns_draw_image_relief` and
// `ns_dumpglyphs_image`), so the covered range is **C 60-4393**; the next live definition is
// C 4396's `ns_draw_stretch_glyph_string`.
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
// C 70-80: the `NS_IMPL_COCOA` arm's own includes - `#include <Carbon/Carbon.h>` (C 72-73) is
// what `ns_get_shifted_character` needs (TIS/UCKeyTranslate), and C 74-79's comment about
// ApplicationServices' UniversalAccess pair is the text round 5's UAZoom block already records.
import Carbon

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

/// C: `typedef struct { unsigned long pixel; unsigned short red, green, blue; }
/// Emacs_Color;` (src/dispextern.h 49-54, the `!HAVE_X_WINDOWS` arm that this target selects) -
/// "XColor-like struct used by non-X code".  It crosses into `ns_defined_color` and
/// `ns_query_frame_background_color`, whose pointee it is; the layout is measured (Evidence).
struct Emacs_Color {
  /// C: `unsigned long pixel;`
  var pixel: UInt
  /// C: `unsigned short red, green, blue;` - the declaration order, which is the layout.
  var red: UInt16
  var green: UInt16
  var blue: UInt16
}

/// C: `struct font_metrics` (src/font.h 414-417) - `short lbearing, rbearing, width, ascent,
/// descent;`.  `ns_compute_glyph_string_overhangs` **holds** one: the core's driver slot fills it
/// through a pointer and C then reads the fields, so the unit declares the same five members (the
/// view `macfont.swift` 49-60 and `femacs/src/font.rs` 196 already carry).  Measured with
/// `spikes/gated-layout-probe.py 'struct font_metrics' font.h` (status 3.271's probe, read-only
/// here): **all three arms read `sizeof=10, align=2`** with `lbearing 0, rbearing 2, width 4,
/// ascent 6, descent 8` - the frozen arm parses for this record (Evidence).
struct font_metrics {
  var lbearing: Int16
  var rbearing: Int16
  var width: Int16
  var ascent: Int16
  var descent: Int16
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

// ==========================================================================
// C 1959-2398: the frame-parameter handlers, the color-management block, and
// the alpha hook's definition (the mouse-handling banner at C 2401-2405 is the next slice).
//
// This slice's C constructs that the unit reads or writes and that have no symbol of their own
// in the reference are entries (the integrator's ruling of status 3.255; the audit quoted in
// `## Evidence` is the measurement behind the class): the `frame_set_*` member writes, the
// two enumerator families (`z_group`, `ns_appearance_type`), the `Q*` globals the handlers
// compare against, and the header-only `Lisp_Object` accessors.  Two call sites take the
// round-5 ruling's deferred shape - an `EmacsWindow` constructor (C 1983) and one of its
// methods (C 2144), whose class lands at C 9756 (`## ABI` (4)).
// ==========================================================================

/// C: `INLINE void fset_parent_frame (struct frame *f, Lisp_Object val)` (src/frame.h 774) -
/// the setter family `docs/ports/nsmenu.swift` 307 reaches; `nsfns.swift` 641-642 imports the
/// same name and the same signature.
@_silgen_name("fset_parent_frame")
func fset_parent_frame(_ f: UnsafeMutableRawPointer?, _ parent: Lisp_Object)
/// C: src/frame.h 1239's `FRAME_UNDECORATED` (`#define FRAME_UNDECORATED(f) ((f)->undecorated)`) -
/// the member is src/frame.h 449's `undecorated` (`bool_bf undecorated : 1;`), written at C 1981.
/// Only the write half is this slice's, and the name is the one `nsfns.swift` 573-574 already
/// imports.
@_silgen_name("frame_set_undecorated")
func frame_set_undecorated(_ f: UnsafeMutableRawPointer?, _ v: Bool)
/// C: `#define FRAME_NO_FOCUS_ON_MAP(f) ((f)->no_focus_on_map)` (src/frame.h 1248) - written at
/// C 2056 (`FRAME_NO_FOCUS_ON_MAP (f) = !NILP (new_value);`).  The macro's read half is the
/// entry `ns_raise_frame` already carries (`FRAME_NO_FOCUS_ON_MAP`), so the write takes the
/// member-write name (`<record>_set_<member>`, `docs/ports/nsmenu.swift` 311-328).
@_silgen_name("frame_set_no_focus_on_map")
func frame_set_no_focus_on_map(_ f: UnsafeMutableRawPointer?, _ v: Bool)
/// C: `#define FRAME_NO_ACCEPT_FOCUS(f) ((f)->no_accept_focus)` (src/frame.h 1249) - written at
/// C 2074; this slice does not read it.
@_silgen_name("frame_set_no_accept_focus")
func frame_set_no_accept_focus(_ f: UnsafeMutableRawPointer?, _ v: Bool)
/// C: `#define FRAME_Z_GROUP(f) ((f)->z_group)` (src/frame.h 1250) - the `ENUM_BF (z_group)
/// z_group : 2` member (src/frame.h 482), written at C 2096/2101/2107/2112, and the four
/// enumerators of `enum z_group` (src/frame.h 61-67) it is written with.
@_silgen_name("frame_set_z_group")
func frame_set_z_group(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("z_group_none") func z_group_none() -> Int32
@_silgen_name("z_group_above") func z_group_above() -> Int32
@_silgen_name("z_group_below") func z_group_below() -> Int32
@_silgen_name("z_group_above_suspended") func z_group_above_suspended() -> Int32
/// C: `#define FRAME_NS_APPEARANCE(f) ((f)->ns_appearance)` (src/frame.h 1258, the
/// `NS_IMPL_COCOA` arm) - the `enum ns_appearance_type` member (src/frame.h 70-75 and 737),
/// written at C 2123/2125/2127 - and the three enumerators of that enum.
@_silgen_name("frame_set_ns_appearance")
func frame_set_ns_appearance(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_appearance_system_default") func ns_appearance_system_default() -> Int32
@_silgen_name("ns_appearance_aqua") func ns_appearance_aqua() -> Int32
@_silgen_name("ns_appearance_vibrant_dark") func ns_appearance_vibrant_dark() -> Int32
/// C: `#define FRAME_NS_TRANSPARENT_TITLEBAR(f) ((f)->ns_transparent_titlebar)` (src/frame.h
/// 1259) - written at C 2162; the write name is the one `nsfns.swift` 577-578 already imports.
@_silgen_name("frame_set_ns_transparent_titlebar")
func frame_set_ns_transparent_titlebar(_ f: UnsafeMutableRawPointer?, _ v: Bool)
/// C: the globals this slice compares against - `Qparent_frame` (src/frame.c 7232),
/// `Qbelow`/`Qabove_suspended` (src/frame.c 7367-7368), `Qabove` (src/window.c 9251) and
/// `Qdark`/`Qlight` (src/nsfns.m 4207-4208, and src/frame.c 7904-7905).  Each is a C data
/// symbol in the reference objects (`_Qdark` and friends are `S` in `build/emacs-ref/src/`),
/// so the core exports one of its own name, the shape `docs/ports/nsfns.swift` 1135-1138 uses.
@_silgen_name("Qparent_frame") func Qparent_frame() -> Lisp_Object
@_silgen_name("Qabove") func Qabove() -> Lisp_Object
@_silgen_name("Qabove_suspended") func Qabove_suspended() -> Lisp_Object
@_silgen_name("Qbelow") func Qbelow() -> Lisp_Object
@_silgen_name("Qdark") func Qdark() -> Lisp_Object
@_silgen_name("Qlight") func Qlight() -> Lisp_Object
/// C: `void store_frame_param (struct frame *, Lisp_Object, Lisp_Object)` (src/lisp.h 5117,
/// defined src/frame.c 3872) - the core's `femacs/src/frame.rs` 3842 defines it.
@_silgen_name("store_frame_param")
func store_frame_param(_ f: UnsafeMutableRawPointer?, _ prop: Lisp_Object, _ val: Lisp_Object)
/// C: src/lisp.h 4835's `error` (`AVOID error (const char *, ...)`) - the lane's variadic shape
/// (`nsfns.swift` 103); `_error` is `T` in `build/emacs-ref/src/eval.o`.
@_silgen_name("error") func error(_ format: UnsafePointer<CChar>, _ args: CVarArg...) -> Never
/// C: `bool parse_color_spec (const char *, unsigned short *, unsigned short *,
/// unsigned short *)` (src/dispextern.h 3798, defined src/xfaces.c 987) - the core's
/// `femacs/src/xfaces.rs` 760 exports the same name and the same `*mut c_ushort` arguments.
@_silgen_name("parse_color_spec")
func parse_color_spec(_ spec: UnsafePointer<CChar>?, _ r: UnsafeMutablePointer<UInt16>,
                      _ g: UnsafeMutablePointer<UInt16>, _ b: UnsafeMutablePointer<UInt16>) -> Bool
/// C: `INLINE bool STRINGP (Lisp_Object)` (src/lisp.h 1582), `INLINE char *SSDATA (Lisp_Object)`
/// (src/lisp.h 1658) and `INLINE Lisp_Object SYMBOL_NAME (Lisp_Object)` (src/lisp.h 2350) - the
/// header-only accessors `ns_lisp_to_color` reads (`docs/ports/nsfns.swift` 184/200-201).
@_silgen_name("STRINGP") func STRINGP(_ x: Lisp_Object) -> Bool
@_silgen_name("SSDATA") func SSDATA(_ string: Lisp_Object) -> UnsafeMutablePointer<CChar>
@_silgen_name("SYMBOL_NAME") func SYMBOL_NAME(_ sym: Lisp_Object) -> Lisp_Object
/// C: `#define FRAME_BACKGROUND_COLOR(f) ((f)->output_data.ns->background_color)` (src/nsterm.h
/// 1038) - the entry `macfont.swift` 756 already imports under this name and this type.
@_silgen_name("FRAME_BACKGROUND_COLOR")
func FRAME_BACKGROUND_COLOR(_ f: UnsafeMutableRawPointer?) -> NSColor?
/// C: `DEFVAR_LISP ("frame-alpha-lower-limit", Vframe_alpha_lower_limit, …)` (src/frame.c
/// 7421-7425) - a generated global (`build/emacs-ref/src/globals.h` 280-281 spells it
/// `globals.f_…`), so the core exports a data symbol of its own name, the shape `Vframe_list`
/// takes above.
@_silgen_name("Vframe_alpha_lower_limit") var Vframe_alpha_lower_limit: Lisp_Object
/// C: `#define FLOATP(x) lisp_h_FLOATP (x)` / `#define FIXNUMP(x) lisp_h_FIXNUMP (x)`
/// (src/lisp.h 448-449) and `INLINE double XFLOAT_DATA (Lisp_Object)` (src/lisp.h 3186) /
/// `INLINE EMACS_INT XFIXNUM (Lisp_Object)` (src/lisp.h 1268) - header-only, so the core
/// exports each under its own name.
@_silgen_name("FLOATP") func FLOATP(_ x: Lisp_Object) -> Bool
@_silgen_name("XFLOAT_DATA") func XFLOAT_DATA(_ x: Lisp_Object) -> Double
@_silgen_name("FIXNUMP") func FIXNUMP(_ x: Lisp_Object) -> Bool
@_silgen_name("XFIXNUM") func XFIXNUM(_ x: Lisp_Object) -> Int
/// C: the `double alpha[2]` member of `struct frame` (src/frame.h 707-711: "`alpha[0]`: alpha
/// transparency of the active frame, `alpha[1]`: alpha transparency of inactive frames") - read
/// at C 2378/2380 with C's own subscript, which is the entry's second argument.
@_silgen_name("frame_alpha")
func frame_alpha(_ f: UnsafeMutableRawPointer?, _ index: Int32) -> Double

// **Deferred entries** (`## ABI` (4)): call sites that precede the definition's C line.
@_silgen_name("EmacsWindow_initWithEmacsFrame")
func EmacsWindow_initWithEmacsFrame(_ f: UnsafeMutableRawPointer?) -> AnyObject?
/// C 2144: `[window setAppearance];` - `EmacsWindow -setAppearance` (C 10398).
@_silgen_name("setAppearance") func setAppearance_entry(_ window: NSWindow?)

// ==========================================================================
// C 1960-1956: the seven frame-parameter handlers src/nsterm.h 1229-1245 declares.
// ==========================================================================

/// C 1960-1996: `void ns_set_undecorated (struct frame *f, Lisp_Object new_value, Lisp_Object
/// old_value)` - `/* Set frame F's `undecorated' parameter.  If non-nil, F's window-system
/// window is drawn without decorations, title, minimize/maximize boxes and external borders.
/// This usually means that the window cannot be dragged, resized, iconified, maximized or
/// deleted with the mouse.  If nil, draw the frame with all the elements listed above unless
/// these have been suspended via window manager settings.  */`.  Declared src/nsterm.h 1229, so
/// it keeps C's name under `@_cdecl`; `nsfns.swift` 457 imports it.
@_cdecl("ns_set_undecorated")
func ns_set_undecorated(_ f: UnsafeMutableRawPointer?, _ new_value: Lisp_Object,
                        _ old_value: Lisp_Object) {
  // C 1971: `NSTRACE ("ns_set_undecorated");` - `NSTRACE` is the empty macro at
  // `NSTRACE_ENABLED` 0 (src/nsterm.h 142-143/297-302), so it carries no statement here.

  if !EQ(new_value, old_value) {
    let view = FRAME_NS_VIEW(f)
    let oldWindow = view?.window
    var newWindow: NSWindow?

    block_input()

    frame_set_undecorated(f, !NILP(new_value))

    // C 1983: `newWindow = [[EmacsWindow alloc] initWithEmacsFrame:f];` - `EmacsWindow` is
    // this file's own class and lands at C 9756 (`-initWithEmacsFrame:` C 9759), so the
    // constructor is the deferred entry above.
    newWindow = EmacsWindow_initWithEmacsFrame(f) as? NSWindow

    if oldWindow?.isKeyWindow ?? false {
      newWindow?.makeKeyAndOrderFront(NSApp)
    }

    newWindow?.setIsVisible(oldWindow?.isVisible ?? false)
    if oldWindow?.isMiniaturized ?? false {
      newWindow?.miniaturize(NSApp)
    }

    oldWindow?.close()

    unblock_input()
  }
}

/// C 1998-2040: `void ns_set_parent_frame (struct frame *f, Lisp_Object new_value, Lisp_Object
/// old_value)` - "Set frame F's `parent-frame' parameter. … On macOS moving a parent frame
/// moves all its child frames too, keeping their position relative to the parent unaltered.
/// …".  Declared src/nsterm.h 1230 (`nsfns.swift` 458).
@_cdecl("ns_set_parent_frame")
func ns_set_parent_frame(_ f: UnsafeMutableRawPointer?, _ new_value: Lisp_Object,
                         _ old_value: Lisp_Object) {
  var p: UnsafeMutableRawPointer? = nil

  // C 2024: `NSTRACE ("ns_set_parent_frame");` - empty, as above.

  // C 2026-2029: `if (!NILP (new_value) && (!FRAMEP (new_value) || !FRAME_LIVE_P (p =
  // XFRAME (new_value)) || !FRAME_NS_P (p)))`.  C's `p = XFRAME (new_value)` sits inside the
  // condition, where Swift has no assignment expression, so the port spells the same
  // short-circuit as three steps: `XFRAME` still runs only when `FRAMEP` failed to reject, and
  // `FRAME_NS_P` only when `FRAME_LIVE_P` did.
  if !NILP(new_value) {
    var bad = !FRAMEP(new_value)

    if !bad {
      p = XFRAME(new_value)
      bad = !FRAME_LIVE_P(p)
      if !bad {
        bad = !FRAME_NS_P(p)
      }
    }

    if bad {
      store_frame_param(f, Qparent_frame(), old_value)
      error("Invalid specification of `parent-frame'")
    }
  }

  fset_parent_frame(f, new_value)

  block_input()
  // C 2038: `[(EmacsWindow *)[FRAME_NS_VIEW (f) window] setParentChildRelationships];` - the
  // `EmacsWindow` method whose entry round 7 already carries.
  setParentChildRelationships_entry(FRAME_NS_VIEW(f)?.window)
  unblock_input()
}

/// C 2042-2058: `void ns_set_no_focus_on_map (struct frame *f, Lisp_Object new_value,
/// Lisp_Object old_value)` - "Set frame F's `no-focus-on-map' parameter which, if non-nil,
/// means that F's window-system window does not want to receive input focus when it is
/// mapped. …".  Declared src/nsterm.h 1231 (`nsfns.swift` 459).
@_cdecl("ns_set_no_focus_on_map")
func ns_set_no_focus_on_map(_ f: UnsafeMutableRawPointer?, _ new_value: Lisp_Object,
                            _ old_value: Lisp_Object) {
  // C 2052: `NSTRACE ("ns_set_no_focus_on_map");` - empty, as above.

  if !EQ(new_value, old_value) {
    frame_set_no_focus_on_map(f, !NILP(new_value))
  }
}

/// C 2060-2075: `void ns_set_no_accept_focus (struct frame *f, Lisp_Object new_value,
/// Lisp_Object old_value)` - "Set frame F's `no-accept-focus' parameter which, if non-nil,
/// hints that F's window-system window does not want to receive input focus via mouse clicks or
/// by moving the mouse into it. …".  Declared src/nsterm.h 1232 (`nsfns.swift` 460).
@_cdecl("ns_set_no_accept_focus")
func ns_set_no_accept_focus(_ f: UnsafeMutableRawPointer?, _ new_value: Lisp_Object,
                            _ old_value: Lisp_Object) {
  // C 2071: `NSTRACE ("ns_set_no_accept_focus");` - empty, as above.

  if !EQ(new_value, old_value) {
    // C 2074 is a single unbraced statement; Swift requires the braces.
    frame_set_no_accept_focus(f, !NILP(new_value))
  }
}

/// C 2077-2116: `void ns_set_z_group (struct frame *f, Lisp_Object new_value, Lisp_Object
/// old_value)` - "Set frame F's `z-group' parameter.  If `above', F's window-system window is
/// displayed above all windows that do not have the `above' property set. …".  Declared
/// src/nsterm.h 1233 (`nsfns.swift` 461).
@_cdecl("ns_set_z_group")
func ns_set_z_group(_ f: UnsafeMutableRawPointer?, _ new_value: Lisp_Object,
                    _ old_value: Lisp_Object) {
  let view = FRAME_NS_VIEW(f)
  let window = view?.window

  // C 2091: `NSTRACE ("ns_set_z_group");` - empty, as above.

  // C 2095/2100/2106/2111: `window.level = NSNormalWindowLevel [+|-] 1;` - Swift renames the
  // AppKit constant to `NSWindow.Level.normal`, and C's `+ 1`/`- 1` become the same level's
  // `rawValue` arithmetic.
  if NILP(new_value) {
    window?.level = NSWindow.Level.normal
    frame_set_z_group(f, z_group_none())
  } else if EQ(new_value, Qabove()) {
    window?.level = NSWindow.Level(rawValue: NSWindow.Level.normal.rawValue + 1)
    frame_set_z_group(f, z_group_above())
  } else if EQ(new_value, Qabove_suspended()) {
    /* Not sure what level this should be.  */
    window?.level = NSWindow.Level(rawValue: NSWindow.Level.normal.rawValue + 1)
    frame_set_z_group(f, z_group_above_suspended())
  } else if EQ(new_value, Qbelow()) {
    window?.level = NSWindow.Level(rawValue: NSWindow.Level.normal.rawValue - 1)
    frame_set_z_group(f, z_group_below())
  } else {
    error("Invalid z-group specification")
  }
}

/// C 2118-2128: `void ns_set_appearance_1 (struct frame *f, Lisp_Object new_value)` - the
/// helper of the `NS_IMPL_COCOA` arm (the `#ifdef` at C 2118 is the live one).  Declared
/// src/nsterm.h 1240; `nsfns.swift` 604 imports it.
@_cdecl("ns_set_appearance_1")
func ns_set_appearance_1(_ f: UnsafeMutableRawPointer?, _ new_value: Lisp_Object) {
  if EQ(new_value, Qdark()) {
    frame_set_ns_appearance(f, ns_appearance_vibrant_dark())
  } else if EQ(new_value, Qlight()) {
    frame_set_ns_appearance(f, ns_appearance_aqua())
  } else {
    frame_set_ns_appearance(f, ns_appearance_system_default())
  }
}

/// C 2130-2146: `void ns_set_appearance (struct frame *f, Lisp_Object new_value, Lisp_Object
/// old_value)`.  The body is C's `MAC_OS_X_VERSION_MAX_ALLOWED >= 101000` arm, which is live
/// for this lane's target (the frozen configuration reads 260000).  Declared src/nsterm.h 1241
/// (`nsfns.swift` 462).
@_cdecl("ns_set_appearance")
func ns_set_appearance(_ f: UnsafeMutableRawPointer?, _ new_value: Lisp_Object,
                       _ old_value: Lisp_Object) {
  let view = FRAME_NS_VIEW(f)
  let window = view?.window

  // C 2137: `NSTRACE ("ns_set_appearance");` - empty, as above.

  // C 2139: `if (NSAppKitVersionNumber < NSAppKitVersionNumber10_10) return;` - the two AppKit
  // globals are renamed in Swift (AppKit.apinotes 8624-8626, the reading the `colorForEmacsRed`
  // category method above records); the fallback `#define NSAppKitVersionNumber10_10 1343` is
  // C 10406-10408, inside a later slice's `-[EmacsWindow setAppearance]`.
  if NSAppKitVersion.current.rawValue < NSAppKitVersion.macOS10_10.rawValue {
    return
  }

  ns_set_appearance_1(f, new_value)

  // C 2144: `[window setAppearance];` - the deferred entry above.
  setAppearance_entry(window)
}

/// C 2148-2165: `void ns_set_transparent_titlebar (struct frame *f, Lisp_Object new_value,
/// Lisp_Object old_value)`.  C's `MAC_OS_X_VERSION_MAX_ALLOWED >= 101000` arm is the live one.
/// Declared src/nsterm.h 1242-1245 (`nsfns.swift` 463).
@_cdecl("ns_set_transparent_titlebar")
func ns_set_transparent_titlebar(_ f: UnsafeMutableRawPointer?, _ new_value: Lisp_Object,
                                 _ old_value: Lisp_Object) {
  let view = FRAME_NS_VIEW(f)
  let window = view?.window

  // C 2156: `NSTRACE ("ns_set_transparent_titlebar");` - empty, as above.

  // C 2158: `[window respondsToSelector: @selector(titlebarAppearsTransparent)]` - Swift
  // reaches the getter's selector through `#selector`, the reading
  // `ns_update_auto_hide_menu_bar` above uses for `setPresentationOptions:`.
  if (window?.responds(to: #selector(getter: NSWindow.titlebarAppearsTransparent)) ?? false)
       && !EQ(new_value, old_value) {
    window?.titlebarAppearsTransparent = !NILP(new_value)
    frame_set_ns_transparent_titlebar(f, !NILP(new_value))
  }
}

/// C 2168-2181: `static void ns_fullscreen_hook (struct frame *f)`.  It is a C `static` (the
/// reference registers it as `terminal->fullscreen_hook` at C 5936), so it lands as a
/// `fileprivate func` of its own name.
fileprivate func ns_fullscreen_hook(_ f: UnsafeMutableRawPointer?) {
  let view = FRAME_NS_VIEW(f)

  // C 2173: `NSTRACE ("ns_fullscreen_hook");` - empty, as above.

  if !FRAME_VISIBLE_P(f) {
    return
  }

  block_input()
  // C 2179: `[view handleFS];` - the `EmacsView` method whose entry round 7 already carries.
  handleFS_entry(view)
  unblock_input()
}

/* ==========================================================================

    Color management

   ========================================================================== */

/// C 2190-2297: `static int ns_get_color (const char *name, NSColor **col)` - "Parse a color
/// name", plus C's own note that it "attempt[s] to mimic the X11 platform here, down to
/// installing an X11 rgb.txt-compatible color list in Emacs.clr".
///
/// C reassigns its `const char *name` parameter twice (C 2224/2238) from `[nsname UTF8String]`;
/// Swift's `String` holds those same bytes, so the port keeps `nsname` alone and reads the two
/// uses of the pointer (C 2243's `parse_color_spec` and C 2252's `name[0]`) from it.
fileprivate func ns_get_color(_ name: UnsafePointer<CChar>?,
                              _ col: UnsafeMutablePointer<NSColor?>) -> Int32 {
  var new: NSColor? = nil
  var nsname: NSString = String(cString: name!) as NSString

  // C 2202: `NSTRACE ("ns_get_color(%s, **)", name);` - empty, so its arguments are not
  // evaluated here either.

  block_input()

  if nsname.isEqual(to: "ns_selection_bg_color") {
    /* C 2208-2214: the `NS_IMPL_COCOA` arm reads the system's own highlight colour; C's
       `#else` arm (the other implementation's spelling) is dead in this build.  */
    let defname = UserDefaults.standard.string(forKey: "AppleHighlightColor")
    if defname != nil {
      nsname = defname! as NSString
    } else {
      // C 2215: `(new = [NSColor selectedTextBackgroundColor]) != nil` - C's assignment
      // inside the condition becomes a statement of its own (Swift has no assignment
      // expression).
      new = NSColor.selectedTextBackgroundColor
      if new != nil {
        col.pointee = new!.colorUsingDefaultColorSpace()
        unblock_input()
        return 0
      } else {
        /* C 2222: `nsname = NS_SELECTION_BG_COLOR_DEFAULT;` - the macro's body is the string
           literal at src/nsterm.h 1278, spelled in place (status 3.255's class B: no symbol
           is invented for a macro whose expansion is a literal).  */
        nsname = "LightGoldenrod2"
      }
    }

    // C 2224: `name = [nsname UTF8String];` - see the note above.
  } else if nsname.isEqual(to: "ns_selection_fg_color") {
    /* NOTE: macOS applications normally don't set foreground
       selection, but text may be unreadable if we don't.  */
    // C 2230: `if ((new = [NSColor selectedTextColor]) != nil)` - as at C 2215.
    new = NSColor.selectedTextColor
    if new != nil {
      col.pointee = new!.colorUsingDefaultColorSpace()
      unblock_input()
      return 0
    }

    /* C 2237: `nsname = NS_SELECTION_FG_COLOR_DEFAULT;` - src/nsterm.h 1279's `@"Black"`.  */
    nsname = "Black"
    // C 2238: `name = [nsname UTF8String];` - see the note above.
  }

  /* First, check for some sort of numeric specification.  */
  var r16: UInt16 = 0, g16: UInt16 = 0, b16: UInt16 = 0
  // C 2243: `parse_color_spec (name, &r16, &g16, &b16)` - `name` is `[nsname UTF8String]`, so
  // the port hands the entry the same bytes through `withCString`.
  if (nsname as String).withCString({ parse_color_spec($0, &r16, &g16, &b16) }) {
    col.pointee = NSColor.colorForEmacsRed(EmacsCGFloat(r16) / 65535.0,
                                           green: EmacsCGFloat(g16) / 65535.0,
                                           blue: EmacsCGFloat(b16) / 65535.0,
                                           alpha: 1.0)
    unblock_input()
    return 0
  } else if (nsname as String).utf8.first == UInt8(ascii: "0")
              || (nsname as String).utf8.first == UInt8(ascii: "1")
              || (nsname as String).utf8.first == UInt8(ascii: ".") {
    /* RGB decimal */
    let scanner = Scanner(string: nsname as String)
    var r: Float = 0, g: Float = 0, b: Float = 0
    if scanner.scanFloat(&r) && r >= 0 && r <= 1
        && scanner.scanFloat(&g) && g >= 0 && g <= 1
        && scanner.scanFloat(&b) && b >= 0 && b <= 1 {
      col.pointee = NSColor.colorForEmacsRed(EmacsCGFloat(r), green: EmacsCGFloat(g),
                                             blue: EmacsCGFloat(b), alpha: 1.0)
      unblock_input()
      return 0
    }
  }

  /* Otherwise, color is expected to be from a list */
  do {
    /* C 2268-2271 declares `NSEnumerator *lenum, *cenum; NSString *name; NSColorList *clist;`
       - C's inner `NSString *name` shadows the parameter, and this loop's `name` is the same
       shadow.  C 2281/2284's `&& new == nil` is the loop's early exit, and the
       `objectEnumerator` reads of C 2280/2283 are the arrays Swift vends (the reading
       `nsfns.swift` 3220 records).  */
    for clist in NSColorList.availableColorLists {
      if new != nil { break }
      for name in clist.allKeys {
        if new != nil { break }
        // C 2286-2288: `[name compare: nsname options: NSCaseInsensitiveSearch] ==
        // NSOrderedSame`.
        if name.compare(nsname as String, options: .caseInsensitive) == .orderedSame {
          new = clist.color(withKey: name)
        }
      }
    }

    /* C 2273-2278: the `NS_IMPL_GNUSTEP` arm's /* XXX: who is wrong, the requestor or the
       implementation?  */ rewrite of `nsname` - dead in this build, recorded not
       transcribed.  (Swift's block comments nest, so C's inner comment survives verbatim.)  */
  }

  if new != nil {
    col.pointee = new!.colorUsingDefaultColorSpace()
  }
  unblock_input()
  return new != nil ? 0 : 1
}

/// C 2299-2312: `int ns_lisp_to_color (Lisp_Object color, NSColor **col)` - "Convert a Lisp
/// string object to a NS color."  Declared src/nsterm.h 1148 (the `__OBJC__` arm), so it keeps
/// C's name under `@_cdecl`; `nsfns.swift` 118-119 imports this signature, and its three call
/// sites pass `&col` for an `NSColor?`.
@_cdecl("ns_lisp_to_color")
func ns_lisp_to_color(_ color: Lisp_Object, _ col: UnsafeMutablePointer<NSColor?>) -> Int32 {
  // C 2306: `NSTRACE ("ns_lisp_to_color");` - empty, as above.

  if STRINGP(color) {
    return ns_get_color(SSDATA(color), col)
  } else if SYMBOLP(color) {
    return ns_get_color(SSDATA(SYMBOL_NAME(color)), col)
  }
  return 1
}

/// C 2314-2329: `static void ns_query_color (void *col, Emacs_Color *color_def)` - "Get ARGB
/// values out of NSColor col and put them into color_def and set color_def pixel to the ARGB
/// color."  C's `void *` parameter is the `NSColor *` its two call sites pass (C 2351/2362),
/// so the port types it; the ARGB extraction is the `NSColor (EmacsColor)` category's
/// `-unsignedLong` above.
fileprivate func ns_query_color(_ col: NSColor?, _ color_def: UnsafeMutablePointer<Emacs_Color>) {
  var r: EmacsCGFloat = 0, g: EmacsCGFloat = 0, b: EmacsCGFloat = 0, a: EmacsCGFloat = 0

  col?.getRed(&r, green: &g, blue: &b, alpha: &a)
  color_def.pointee.red = UInt16(r * 65535)
  color_def.pointee.green = UInt16(g * 65535)
  color_def.pointee.blue = UInt16(b * 65535)

  color_def.pointee.pixel = col!.unsignedLong()
}

/// C 2331-2354: `bool ns_defined_color (struct frame *f, const char *name, Emacs_Color
/// *color_def, bool alloc, bool _makeIndex)` - "Return true if named color found, and set
/// color_def rgb accordingly.  Return false if not found."  Declared src/nsterm.h 1142, so it
/// keeps C's name under `@_cdecl`.  `@_cdecl` refuses a pointer to a Swift record (measured),
/// so the `Emacs_Color *` crosses as `UnsafeMutableRawPointer` and is re-bound inside - the
/// reading round 2's `ns_init_events` records - while the ABI is C's pointer either way.
@_cdecl("ns_defined_color")
func ns_defined_color(_ f: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?,
                      _ color_def: UnsafeMutableRawPointer?, _ alloc: Bool,
                      _ _makeIndex: Bool) -> Bool {
  var col: NSColor? = nil
  // C 2343: `NSTRACE_WHEN (NSTRACE_GROUP_COLOR, "ns_defined_color");` - empty at
  // `NSTRACE_ENABLED` 0, so neither its group nor its format string is evaluated.

  block_input()
  if ns_get_color(name, &col) != 0 { /* Color not found  */
    unblock_input()
    return false
  }
  ns_query_color(col, color_def!.assumingMemoryBound(to: Emacs_Color.self))
  unblock_input()
  return true
}

/// C 2356-2363: `static void ns_query_frame_background_color (struct frame *f, Emacs_Color
/// *bgcolor)` - "External (hook): Store F's background color into *BGCOLOR".  The reference
/// registers it as `terminal->query_frame_background_color` (C 5929).
fileprivate func ns_query_frame_background_color(_ f: UnsafeMutableRawPointer?,
                                                 _ bgcolor: UnsafeMutablePointer<Emacs_Color>) {
  ns_query_color(FRAME_BACKGROUND_COLOR(f), bgcolor)
}

/// C 2365-2398: `static void ns_set_frame_alpha (struct frame *f)` - "change the entire-frame
/// transparency".  C forward-declares it at C 1579 (`static void ns_set_frame_alpha (struct
/// frame *f);`) and calls it first at C 1612/1617, which is what put those call sites on the
/// deferred-entry footing (`ns_set_frame_alpha_entry`, `## ABI` (4)); **this is the definition
/// that entry waited for**, and the reference also registers it as
/// `terminal->set_frame_alpha_hook` (C 5941).
@_cdecl("ns_set_frame_alpha")
func ns_set_frame_alpha(_ f: UnsafeMutableRawPointer?) {
  var alpha: Double = 1.0
  var alpha_min: Double = 1.0

  // C 2375: `NSTRACE ("ns_set_frame_alpha");` - empty, as above.

  // C 2371: `struct ns_display_info *dpyinfo = FRAME_DISPLAY_INFO (f);` - the display-info
  // member entry round 7 lands takes the *frame* C's `FRAME_DISPLAY_INFO` starts from, the
  // shape `ns_frame_rehighlight` above already uses, so the local itself has no separate
  // reading here.
  // C 2377-2380: `dpyinfo->highlight_frame == f` is that entry; `f->alpha[0]`/`[1]` is the
  // `frame_alpha` entry above.
  if ns_display_info_highlight_frame(f) == f {
    alpha = frame_alpha(f, 0)
  } else {
    alpha = frame_alpha(f, 1)
  }

  // C 2382-2385: `FLOATP`/`XFLOAT_DATA` and `FIXNUMP`/`XFIXNUM` on the generated global
  // `Vframe_alpha_lower_limit`.
  if FLOATP(Vframe_alpha_lower_limit) {
    alpha_min = XFLOAT_DATA(Vframe_alpha_lower_limit)
  } else if FIXNUMP(Vframe_alpha_lower_limit) {
    alpha_min = Double(XFIXNUM(Vframe_alpha_lower_limit)) / 100.0
  }

  if alpha < 0.0 {
    return
  } else if 1.0 < alpha {
    alpha = 1.0
  } else if 0.0 <= alpha && alpha < alpha_min && alpha_min <= 1.0 {
    alpha = alpha_min
  }

  do {
    let view = FRAME_NS_VIEW(f)
    view?.window?.alphaValue = EmacsCGFloat(alpha)
  }
}

/* ==========================================================================

    Mouse handling

   ========================================================================== */

// C 2408-2621: the section's four functions.  Its entries are the shapes rounds 5-10 settled:
// per-member entries for `struct frame`/`struct ns_display_info`/`Mouse_HLInfo`, same-name
// entries for the globals, enumerators and header-only accessors, and receiver-first entries for
// the messages this unit's own classes (`EmacsView`) and `src/nsmenu.m`'s (`EmacsTooltip`)
// answer.

/// C: src/frame.h 1256's `FRAME_TOOLTIP_P` (`#define FRAME_TOOLTIP_P(f) ((f)->tooltip)`), inside the
/// `HAVE_WINDOW_SYSTEM` arm opened at src/frame.h 1241.
/// The member is src/frame.h 457's `tooltip` (`bool_bf tooltip : 1;`), read at C 2544/2560.
@_silgen_name("FRAME_TOOLTIP_P") func FRAME_TOOLTIP_P(_ f: UnsafeMutableRawPointer?) -> Bool
/// C: the `struct frame` member `f->mouse_moved` (src/frame.h 434's `bool_bf mouse_moved : 1`),
/// written at C 2464 (`= 1`) and C 2513 (`= 0`) - a member-write entry
/// (`<record>_set_<member>`, `docs/ports/nsmenu.swift` 311-328).
@_silgen_name("frame_set_mouse_moved")
func frame_set_mouse_moved(_ f: UnsafeMutableRawPointer?, _ v: Bool)
/// C: the `struct ns_display_info` members this section reads or writes.  Each takes the *frame*
/// C's `FRAME_DISPLAY_INFO` starts from - the shape rounds 7/8's `ns_display_info_*` entries use -
/// and the rect member is a *pointer* entry because C takes its address
/// (`r = &dpyinfo->last_mouse_glyph`), which leaves every `r->field` access in Swift.
@_silgen_name("ns_display_info_set_last_mouse_motion_frame")
func ns_display_info_set_last_mouse_motion_frame(_ f: UnsafeMutableRawPointer?,
                                                 _ v: UnsafeMutableRawPointer?)
@_silgen_name("ns_display_info_last_mouse_glyph")
func ns_display_info_last_mouse_glyph(_ f: UnsafeMutableRawPointer?)
  -> UnsafeMutablePointer<NSRect>?
@_silgen_name("ns_display_info_set_last_mouse_scroll_bar")
func ns_display_info_set_last_mouse_scroll_bar(_ f: UnsafeMutableRawPointer?,
                                               _ v: UnsafeMutableRawPointer?)
@_silgen_name("ns_display_info_last_mouse_frame")
func ns_display_info_last_mouse_frame(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("ns_display_info_last_mouse_movement_time")
func ns_display_info_last_mouse_movement_time(_ f: UnsafeMutableRawPointer?) -> UInt
/// C: the `Mouse_HLInfo` members `mouse_face_mouse_x`/`mouse_face_mouse_y` (src/dispextern.h
/// 3013's `int mouse_face_mouse_x, mouse_face_mouse_y;`), read at C 2615-2616 through the
/// `MOUSE_HL_INFO` **handle** - the reading `mouse_highlight_mouse_face_mouse_frame` above uses.
@_silgen_name("mouse_highlight_mouse_face_mouse_x")
func mouse_highlight_mouse_face_mouse_x(_ hlinfo: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("mouse_highlight_mouse_face_mouse_y")
func mouse_highlight_mouse_face_mouse_y(_ hlinfo: UnsafeMutableRawPointer?) -> Int32
/// C: `track_mouse` - src/keyboard.c's `DEFVAR_LISP ("track-mouse", …)`, which the reference's
/// generated `globals.h` (852-853) spells `globals.f_track_mouse`: the object has **no
/// `_track_mouse` symbol** (measured), so the core exports a data symbol of its own name, the
/// reading `Vframe_list`/`Vframe_alpha_lower_limit` take.
@_silgen_name("track_mouse") var track_mouse: Lisp_Object
/// C: `Qdrag_source`/`Qdropping` (`DEFSYM`s; `_Qdrag_source`/`_Qdropping` are `S` in
/// `build/emacs-ref/src/emacs.o`) and the `enum scroll_bar_part` enumerator
/// `scroll_bar_above_handle` (src/termhooks.h 37), which has no symbol.
@_silgen_name("Qdrag_source") func Qdrag_source() -> Lisp_Object
@_silgen_name("Qdropping") func Qdropping() -> Lisp_Object
@_silgen_name("scroll_bar_above_handle") func scroll_bar_above_handle() -> Int32
/// C: `#define XSETINT(a, b) ((a) = make_fixnum (b))` (src/lisp.h 1341) - an *lvalue* macro, so the
/// entry takes the assignable `Lisp_Object` first, the shape `XSETFRAME` has
/// (`docs/ports/nsfns.swift` 709-711).
@_silgen_name("XSETINT")
func XSETINT(_ a: UnsafeMutablePointer<Lisp_Object>, _ b: Int)
/// C: `void note_mouse_highlight (struct frame *, int, int)` (src/dispextern.h 3688) - a real
/// symbol of the ported core (`_note_mouse_highlight` is `T` in `build/emacs-ref/src/xdisp.o`).
@_silgen_name("note_mouse_highlight")
func note_mouse_highlight(_ f: UnsafeMutableRawPointer?, _ x: Int32, _ y: Int32)
/// C: `void remember_mouse_glyph (struct frame *, int, int, NativeRectangle *)`
/// (src/dispextern.h 3586; `NativeRectangle` is `NSRect` for this target, src/nsgui.h 80).  The
/// definition is src/xdisp.c 2779's `remember_mouse_glyph`, inside the `HAVE_WINDOW_SYSTEM` block
/// opened at src/xdisp.c 2488, which the frozen configuration leaves undefined - so the object
/// has no
/// `_remember_mouse_glyph` (measured), and this is the owed-entry class status 3.255 ③ gives
/// `frame_set_double_buffered`: the shim exports it under the same gate.
@_silgen_name("remember_mouse_glyph")
func remember_mouse_glyph(_ f: UnsafeMutableRawPointer?, _ gx: Int32, _ gy: Int32,
                          _ rect: UnsafeMutablePointer<NSRect>?)
/// C: `int fputs (const char *, FILE *)` (stdio.h) - the lane's `@_silgen_name` shape
/// (`docs/ports/nsfont.swift` 1178; Swift marks the stdio family unavailable).  `stderr` itself
/// is the C library overlay's own declaration, which this unit already reads at C 569-600.
@_silgen_name("fputs") func fputs(_ s: UnsafePointer<CChar>, _ stream: UnsafeMutablePointer<FILE>?)

/// C: `[[w delegate] isKindOfClass: [EmacsView class]]` (C 2537) and
/// `((EmacsView *) [w delegate])->emacsframe` (C 2538) - `EmacsView` is this unit's own class
/// (C 7039) and lands later, so both are receiver-first entries; the names and signatures are the
/// ones `nsfns.swift` 689-691 already imports, and **this unit defines them when the class
/// lands** (the ledger's (3), the reverse direction).
@_silgen_name("emacs_view_frame")
func emacs_view_frame(_ view: AnyObject?) -> UnsafeMutableRawPointer?
@_silgen_name("is_emacs_view") func is_emacs_view(_ obj: AnyObject?) -> Bool
/// C: `[[w delegate] isKindOfClass: [EmacsTooltip class]]` (C 2534) - `EmacsTooltip` is
/// `src/nsmenu.m`'s class (`nsmenu.swift`'s `EmacsTooltip`), so this entry's definition belongs
/// on that side, the way `is_emacs_view`'s belongs here.
@_silgen_name("is_emacs_tooltip") func is_emacs_tooltip(_ obj: AnyObject?) -> Bool

/// C 2408-2430: `void frame_set_mouse_pixel_position (struct frame *f, int pix_x, int pix_y)` -
/// "Programmatically reposition mouse pointer in pixel coordinates".  Declared src/frame.h 1882
/// (the redisplay interface's `set_mouse_pixel_position_hook`), so it keeps C's name under
/// `@_cdecl`.  C's `#ifdef NS_IMPL_COCOA` arm is the live one; the GNUstep arm (C 2422-2429,
/// `GSServerForWindow`/`setMouseLocation:onScreen:`) is recorded, not transcribed.
@_cdecl("frame_set_mouse_pixel_position")
func frame_set_mouse_pixel_position(_ f: UnsafeMutableRawPointer?, _ pix_x: Int32, _ pix_y: Int32) {
  // C 2414: `NSTRACE ("frame_set_mouse_pixel_position");` - empty at `NSTRACE_ENABLED` 0
  // (src/nsterm.h 142-143), so it carries no statement here.

  // C 2417-2421: `CGPointMake (f->left_pos + pix_x, f->top_pos + pix_y +
  // FRAME_NS_TITLEBAR_HEIGHT (f) + FRAME_TOOLBAR_HEIGHT (f))` then
  // `CGWarpMouseCursorPosition (mouse_pos)`.  The two macros are this unit's *expanded*
  // `frame_ns_titlebar_height`/`frame_toolbar_height` (round 9), and `CGWarpMouseCursorPosition`
  // is CoreGraphics' own call, the reading `CGMainDisplayID`/`CGDisplayBounds` above take.
  let mouse_pos = CGPoint(x: CGFloat(frame_left_pos(f) + pix_x),
                          y: CGFloat(frame_top_pos(f) + pix_y
                                     + frame_ns_titlebar_height(f) + frame_toolbar_height(f)))
  CGWarpMouseCursorPosition(mouse_pos)
}

/// C 2432-2472: `static int ns_note_mouse_movement (struct frame *frame, CGFloat x, CGFloat y,
/// BOOL dragging)` - "Called by EmacsView on mouseMovement events.  Passes on to emacs
/// mainstream code if we moved off of a rect of interest known as last_mouse_glyph."  It is a C
/// `static` (an `EmacsView` helper), so it lands as a `fileprivate func`; C's `BOOL` is Swift's
/// `Bool`.
fileprivate func ns_note_mouse_movement(_ frame: UnsafeMutableRawPointer?, _ x: CGFloat,
                                        _ y: CGFloat, _ dragging: Bool) -> Int32 {
  // C 2441: `struct ns_display_info *dpyinfo = FRAME_DISPLAY_INFO (frame);` - the member entries
  // below take the frame that macro starts from, so the pointer has no separate reading (round
  // 10's `ns_set_frame_alpha` records the same shape).
  var force_update = false

  // C 2445: `// NSTRACE ("note_mouse_movement");` - commented out in C itself, so it stays a
  // comment here too.

  // C 2447-2448: `dpyinfo->last_mouse_motion_frame = frame;` and
  // `r = &dpyinfo->last_mouse_glyph;`.
  ns_display_info_set_last_mouse_motion_frame(frame, frame)
  let r = ns_display_info_last_mouse_glyph(frame)

  /* If the last rect is too large (ex, xwidget webkit), update at
     every move, or resizing by dragging modeline or vertical split is
     very hard to make its way.  */
  if dragging && (r!.pointee.size.width > 32 || r!.pointee.size.height > 32) {
    force_update = true
  }

  /* Note, this doesn't get called for enter/leave, since we don't have a
     position.  Those are taken care of in the corresponding NSView methods.  */

  /* Has movement gone beyond last rect we were tracking?  */
  if force_update || x < r!.pointee.origin.x || x >= r!.pointee.origin.x + r!.pointee.size.width
       || y < r!.pointee.origin.y || y >= r!.pointee.origin.y + r!.pointee.size.height {
    ns_update_begin(frame)
    frame_set_mouse_moved(frame, true)
    // C 2465-2466: C hands `note_mouse_highlight`/`remember_mouse_glyph` the `int` parameters
    // their own prototypes declare, converting `x`/`y` implicitly; the port spells the same
    // truncation.
    note_mouse_highlight(frame, Int32(x), Int32(y))
    remember_mouse_glyph(frame, Int32(x), Int32(y), r)
    ns_update_end(frame)
    return 1
  }

  return 0
}

/// C 2475-2595: `static void ns_mouse_position (struct frame **fp, int insist, Lisp_Object
/// *bar_window, enum scroll_bar_part *part, Lisp_Object *x, Lisp_Object *y, Time *time)` -
/// "External (hook): inform emacs about mouse position and hit parts. …".  It is C's
/// `mouse_position_hook`; the body never reads `insist` (C's own signature keeps it).  The
/// `NS_IMPL_COCOA` arm is the live one - the GNUstep compatibility note inside it (C 2519-2521)
/// is C's own text - and `view`'s C declaration at the top of the function lives in the one
/// block that uses it here.
fileprivate func ns_mouse_position(_ fp: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
                                  _ insist: Int32,
                                  _ bar_window: UnsafeMutablePointer<Lisp_Object>?,
                                  _ part: UnsafeMutablePointer<Int32>?,
                                  _ x: UnsafeMutablePointer<Lisp_Object>?,
                                  _ y: UnsafeMutablePointer<Lisp_Object>?,
                                  _ time: UnsafeMutablePointer<UInt>?) {
  var view_position = NSPoint.zero
  var frame: Lisp_Object = nil
  var tail: Lisp_Object = nil
  var f: UnsafeMutableRawPointer? = nil
  var return_no_frame_flag = false
  // C 2492-2496: the `NS_IMPL_COCOA` arm's locals (`NSPoint screen_position; NSInteger
  // window_number; NSWindow *w;`).
  var screen_position = NSPoint.zero
  var window_number = 0
  var w: NSWindow? = nil

  // C 2498: `NSTRACE ("ns_mouse_position");` - empty, as above.

  if fp!.pointee == nil {
    fputs("Warning: ns_mouse_position () called with null *fp.\n", stderr)
    return
  }

  // C 2506: `dpyinfo = FRAME_DISPLAY_INFO (*fp);` - the member entries below take the frame.

  block_input()

  /* Clear the mouse-moved flag for every frame on this display.  */
  // C 2511-2513: `FOR_EACH_FRAME (tail, frame) if (FRAME_NS_P (XFRAME (frame))) XFRAME
  // (frame)->mouse_moved = 0;` - the macro is spelled out, the reading
  // `docs/ports/nsfns.swift` 967 gives and round 5's `ns_constrain_all_frames` already uses.
  tail = Vframe_list
  while CONSP(tail) {
    frame = XCAR(tail)
    if FRAME_NS_P(XFRAME(frame)) {
      frame_set_mouse_moved(XFRAME(frame), false)
    }
    tail = XCDR(tail)
  }

  // C 2515: `dpyinfo->last_mouse_scroll_bar = nil;` - `EmacsScroller` is this file's own class
  // (a later slice), so the member-write entry carries it untyped (P-55).
  ns_display_info_set_last_mouse_scroll_bar(fp!.pointee, nil)

  // C 2517-2548: the `NS_IMPL_COCOA` arm - "Find the uppermost Emacs frame under the mouse
  // pointer."  C's `do { … } while (window_number > 0 && !f);` is Swift's `repeat { … } while`.
  screen_position = NSEvent.mouseLocation
  window_number = 0

  repeat {
    window_number = NSWindow.windowNumber(at: screen_position,
                                          belowWindowWithWindowNumber: window_number)
    w = NSApp.window(withWindowNumber: window_number)

    if (EQ(track_mouse, Qdrag_source()) || EQ(track_mouse, Qdropping()))
         && w != nil && is_emacs_tooltip(w!.delegate) {
      continue
    }

    if w != nil && is_emacs_view(w!.delegate) {
      f = emacs_view_frame(w!.delegate)
    } else if EQ(track_mouse, Qdrag_source()) {
      break
    }

    if f != nil && (EQ(track_mouse, Qdrag_source()) || EQ(track_mouse, Qdropping()))
         && FRAME_TOOLTIP_P(f) {
      continue
    }
  } while window_number > 0 && f == nil

  if f == nil {
    // C 2552-2554: `f = (dpyinfo->ns_focus_frame ? dpyinfo->ns_focus_frame : SELECTED_FRAME ());
    // return_no_frame_flag = EQ (track_mouse, Qdrag_source);`
    let focus = ns_display_info_ns_focus_frame(fp!.pointee)
    f = (focus != nil ? focus : SELECTED_FRAME())
    return_no_frame_flag = EQ(track_mouse, Qdrag_source())
  }

  if !FRAME_NS_P(f) {
    f = nil
  }

  if f != nil && FRAME_TOOLTIP_P(f) {
    f = ns_display_info_last_mouse_frame(fp!.pointee)
  }

  /* While dropping, use the last mouse frame only if there is no
     currently focused frame.  */
  // C 2565-2572 reads `dpyinfo->last_mouse_frame` three times, and the entry is read the same
  // number of times here.
  if f == nil && (EQ(track_mouse, Qdropping()) || EQ(track_mouse, Qdrag_source()))
       && ns_display_info_last_mouse_frame(fp!.pointee) != nil
       && FRAME_LIVE_P(ns_display_info_last_mouse_frame(fp!.pointee)) {
    f = ns_display_info_last_mouse_frame(fp!.pointee)
    return_no_frame_flag = EQ(track_mouse, Qdrag_source())
  }

  if f != nil && FRAME_NS_P(f) {
    // C 2486: `id view;` - C declares it at the top and assigns it here, the one block that
    // uses it.
    let view = FRAME_NS_VIEW(f)

    // C 2578-2579: `view_position = [[view window] mouseLocationOutsideOfEventStream];
    // view_position = [view convertPoint: view_position fromView: nil];`
    view_position = view?.window?.mouseLocationOutsideOfEventStream ?? .zero
    view_position = view?.convert(view_position, from: nil) ?? .zero
    // C 2580-2581: the two `CGFloat` arguments land in `remember_mouse_glyph`'s `int`
    // parameters, C's implicit conversion again; the rect is the member's own address.
    remember_mouse_glyph(f, Int32(view_position.x), Int32(view_position.y),
                         ns_display_info_last_mouse_glyph(fp!.pointee))
    // C 2582: `NSTRACE_POINT ("view_position", view_position);` - empty, as above.

    if bar_window != nil { bar_window!.pointee = Qnil() }
    if part != nil { part!.pointee = scroll_bar_above_handle() }

    // C 2587-2588: `XSETINT (*x, lrint (view_position.x)); XSETINT (*y, lrint (view_position.y));`
    // - C's `lrint` is the double overload (`CGFloat` is `double` on this target), so the port
    // spells it with `Double`.
    if x != nil { XSETINT(x!, lrint(Double(view_position.x))) }
    if y != nil { XSETINT(y!, lrint(Double(view_position.y))) }
    if time != nil { time!.pointee = ns_display_info_last_mouse_movement_time(fp!.pointee) }
    // C 2591: `*fp = return_no_frame_flag ? NULL : f;`
    fp!.pointee = return_no_frame_flag ? nil : f
  }

  unblock_input()
}

/// C 2598-2621: `static void ns_frame_up_to_date (struct frame *f)` - "External (hook): Fix up
/// mouse highlighting right after a full update.  Can't use FRAME_MOUSE_UPDATE due to
/// ns_frame_begin and ns_frame_end calls."  It is C's `frame_up_to_date_hook`.
fileprivate func ns_frame_up_to_date(_ f: UnsafeMutableRawPointer?) {
  // C 2605: `NSTRACE_WHEN (NSTRACE_GROUP_UPDATES, "ns_frame_up_to_date");` - empty at
  // `NSTRACE_ENABLED` 0, so neither its group nor its format string is evaluated.

  if FRAME_NS_P(f) {
    let hlinfo = MOUSE_HL_INFO(f)
    if f == mouse_highlight_mouse_face_mouse_frame(hlinfo) {
      block_input()
      ns_update_begin(f)
      // C 2614-2616: the three `hlinfo->…` reads feed `note_mouse_highlight (struct frame *,
      // int, int)`.
      note_mouse_highlight(mouse_highlight_mouse_face_mouse_frame(hlinfo),
                           mouse_highlight_mouse_face_mouse_x(hlinfo),
                           mouse_highlight_mouse_face_mouse_y(hlinfo))
      ns_update_end(f)
      unblock_input()
    }
  }
}

// ==========================================================================
// C 2624-2761: the RIF cursor hook, and the Keyboard handling prologue.
// ==========================================================================

/// C: `#define FRAME_POINTER_TYPE(f) ((f)->output_data.ns->current_pointer)` (src/nsterm.h 1047) -
/// the `struct ns_output` member `current_pointer` (src/nsterm.h 977; `Emacs_Cursor` is
/// `NSCursor *`, src/nsgui.h 62).  The **write** entry already exists for this very C line - the
/// shim's `ns_output_set_current_pointer` cites nsterm.m 2634 (`docs/ports/abi_shim.md` 96) - so
/// the port takes the family's member names (`ns_output_text_cursor`/`ns_output_set_text_cursor`
/// are the pair's own shape) rather than a macro-name entry for the read half; both live behind
/// the shim's `have_ns` gate, the ratchet's reason for the whole `ns_output_*` family
/// (status 3.262/3.264).  `nsfns.swift` 312-313 imports the write with this signature.
@_silgen_name("ns_output_current_pointer")
func ns_output_current_pointer(_ f: UnsafeMutableRawPointer?) -> NSObject?
@_silgen_name("ns_output_set_current_pointer")
func ns_output_set_current_pointer(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
/// C: `QCordinary` - src/nsterm.m 11793's own `DEFSYM (QCordinary, ":ordinary")`.  It is **this
/// file's** symbol, so it is absent from `docs/baseline/globals/symbols.tsv` (a table built from
/// a configuration that never compiles this file) and the core has to export the name - the
/// `QCmouse` reading round 2 recorded.
@_silgen_name("QCordinary") func QCordinary() -> Lisp_Object
/// C: `Qnone` - `DEFSYM (Qnone, "none")` (src/frame.c 7241); `_Qnone` and `_Qleft` are `S` in
/// `build/emacs-ref/src/emacs.o`, and the core's globals table already carries it (another unit's
/// import put it in the ratchet).
@_silgen_name("Qnone") func Qnone() -> Lisp_Object

/// C 2624-2637: `static void ns_define_frame_cursor (struct frame *f, Emacs_Cursor cursor)` -
/// "External (RIF): set frame mouse pointer type."  It is C's `define_frame_cursor_hook`, so it
/// lands as a `fileprivate func`.  C compares and assigns the pointer, so the port keeps identity
/// comparison (`!==`) rather than an `isEqual:` call.
fileprivate func ns_define_frame_cursor(_ f: UnsafeMutableRawPointer?, _ cursor: NSObject?) {
  // C 2630: `NSTRACE ("ns_define_frame_cursor");` - empty at `NSTRACE_ENABLED` 0 (src/nsterm.h
  // 142-143), so it carries no statement here.

  // C 2631/2634: `FRAME_POINTER_TYPE (f)` read and written.
  if ns_output_current_pointer(f) !== cursor {
    let view = FRAME_NS_VIEW(f)
    ns_output_set_current_pointer(f, cursor)
    // C 2635: `[[view window] invalidateCursorRectsForView: view];`
    view?.window?.invalidateCursorRects(for: view!)
  }
}

/* ==========================================================================

    Keyboard handling

   ========================================================================== */

/// C 2647-2663: `static unsigned ns_convert_key (unsigned code)` - "Internal call used by
/// NSView-keyDown."  `countof (convert_ns_to_X_keysym)` is the table's element count; the table
/// itself is round 1's `fileprivate let convert_ns_to_X_keysym: [UInt32]` (flat `(key, keysym)`
/// pairs), which is why the two reads carry `Int(keysym)` subscripts.
fileprivate func ns_convert_key(_ code: UInt32) -> UInt32 {
  let last_keysym = UInt32(convert_ns_to_X_keysym.count)
  /* An array would be faster, but less easy to read.  */
  var keysym: UInt32 = 0
  while keysym < last_keysym {
    if code == convert_ns_to_X_keysym[Int(keysym)] {
      return 0xFF00 | convert_ns_to_X_keysym[Int(keysym) + 1]
    }
    keysym += 2
  }
  return 0
  /* if decide to use keyCode and Carbon table, use this line:
       return code > 0xff ? 0 : 0xFF00 | ns_keycode_to_xkeysym_table[code]; */
}

/// C 2673: `static char value[16];` - C's **function-scope** static.  Swift has no function-scope
/// storage, so it is a file-scope buffer of the same size, allocated once: the reference's address
/// is stable for the process's life and it is never freed there either.  C's static storage is
/// zero-initialised, hence the `initialize`.
fileprivate let get_keysym_name_value: UnsafeMutablePointer<CChar> = {
  let p = UnsafeMutablePointer<CChar>.allocate(capacity: 16)
  p.initialize(repeating: 0, count: 16)
  return p
}()

/// C 2666-2677: `char *get_keysym_name (int keysym)` - "Called by keyboard.c.  Not sure if the
/// return val is important, except that it be unique."  Declared src/lisp.h 5436, so it keeps C's
/// name under `@_cdecl`: this unit is where the definition lives, and `keyboard.rs` 8553's
/// `#[cfg (feature = "have_window_system")]` arm is the Rust side's dead-text carrier of the same
/// call (its own comment records that it is not declared there).
@_cdecl("get_keysym_name")
func get_keysym_name(_ keysym: Int32) -> UnsafeMutablePointer<CChar>? {
  // C 2674: `NSTRACE ("get_keysym_name");` - empty, as above.

  // C 2675: `snprintf (value, 16, "%d", keysym);` - `snprintf` is variadic and unavailable in
  // Swift (the reading `docs/ports/nsfns.swift` 3698 gives), so the port builds the same string
  // and copies it into the same 16-byte buffer, keeping C's bound.
  let bytes = Array(String(format: "%d", keysym).utf8CString)
  let n = min(bytes.count, 16)
  for i in 0..<n {
    get_keysym_name_value[i] = bytes[i]
  }
  return get_keysym_name_value
}

/// C 2679-2684: `static Lisp_Object right_mod (Lisp_Object left, Lisp_Object right)` - the
/// `NS_IMPL_COCOA` arm's helper (the `#ifdef` at C 2679 is the live one).
fileprivate func right_mod(_ left: Lisp_Object, _ right: Lisp_Object) -> Lisp_Object {
  return EQ(right, Qleft()) ? left : right
}

/// C 2686-2690: `static bool nil_or_none (Lisp_Object val)` - the same arm.
fileprivate func nil_or_none(_ val: Lisp_Object) -> Bool {
  return NILP(val) || EQ(val, Qnone())
}

/// C 2703: `static UInt32 dead_key_state;` - `ns_get_shifted_character`'s function-scope static,
/// a file-scope `var` here for the reason `get_keysym_name_value` above records.
fileprivate var dead_key_state: UInt32 = 0

/// C 2692-2761: `static UniChar ns_get_shifted_character (NSEvent *event)` - "Look up the
/// character corresponding to the key pressed on the current keyboard layout and the currently
/// configured shift-like modifiers. …".  The `NS_IMPL_COCOA` arm is the live one, and its Carbon
/// dependency is C 72-73's `#include <Carbon/Carbon.h>` (`import Carbon` at the top of this file).
///
/// `UniChar` is `UInt16` (`UniCharCount` is `Int` in Swift - the SDK has no `UniCharCount` type),
/// and `[event modifierFlags]`'s `NSUInteger` truncates to C's `UInt32 flags` exactly as the
/// round-1 `EV_*` accessors do.
fileprivate func ns_get_shifted_character(_ event: NSEvent) -> UInt16 {
  /* UCKeyTranslate may return up to 255 characters.  If the buffer
     isn't large enough then it produces an error.  What kind of
     keyboard inputs 255 characters in a single keypress?  */
  var buf = [UniChar](repeating: 0, count: 255)
  let max_string_length = 255
  var actual_string_length = 0
  var result: OSStatus = 0

  // C 2713-2715: `CFDataRef layout_ref = (CFDataRef) TISGetInputSourceProperty
  // (TISCopyCurrentKeyboardLayoutInputSource (), kTISPropertyUnicodeKeyLayoutData);` then
  // `UCKeyboardLayout *layout = (UCKeyboardLayout *) CFDataGetBytePtr (layout_ref);`.
  // `TISCopyCurrentKeyboardLayoutInputSource` follows the Create rule, so Swift takes the +1
  // (`takeRetainedValue`, and ARC releases it at the end) where C simply leaks it.
  let layout_ref = TISGetInputSourceProperty(
    TISCopyCurrentKeyboardLayoutInputSource()!.takeRetainedValue(),
    kTISPropertyUnicodeKeyLayoutData)
  // ...and the property hands back a **+0** `CFTypeRef` (`CFDataRef layout_ref = (CFDataRef)
  // TISGetInputSourceProperty (…)`), so the port borrows it instead of taking a reference it does
  // not own - `Unmanaged.fromOpaque(…).takeUnretainedValue()`, the reading `macfont.swift` 1541
  // uses for the same CFTypeRef-to-Swift-CF conversion.
  let layout_data = Unmanaged<CFData>.fromOpaque(layout_ref!).takeUnretainedValue()
  let layout = UnsafeRawPointer(CFDataGetBytePtr(layout_data)!)
    .assumingMemoryBound(to: UCKeyboardLayout.self)

  // C 2717-2718: `UInt32 flags = [event modifierFlags];` and
  // `UInt32 modifiers = (flags & NSEventModifierFlagShift) ? shiftKey : 0;` - the four Carbon key
  // masks are `UInt32(…)` conversions of the anonymous-enum constants, as C's implicit
  // enum-to-`UInt32` arithmetic does (the file's `EV_MODIFIERS2` reads the `UInt32` masks the same
  // way).
  let flags = UInt32(truncatingIfNeeded: event.modifierFlags.rawValue)
  var modifiers: UInt32 = (flags & UInt32(truncatingIfNeeded: NSEvent.ModifierFlags.shift.rawValue)) != 0
    ? UInt32(shiftKey)
    : 0

  // C 2720: `NSTRACE ("ns_get_shifted_character");` - empty, as above.

  if (flags & UInt32(truncatingIfNeeded: NSRightAlternateKeyMask))
       == UInt32(truncatingIfNeeded: NSRightAlternateKeyMask)
       && nil_or_none(mod_of_kind(right_mod(ns_alternate_modifier, ns_right_alternate_modifier),
                                  QCordinary())) {
    modifiers |= UInt32(rightOptionKey)
  }

  if (flags & UInt32(truncatingIfNeeded: NSLeftAlternateKeyMask))
       == UInt32(truncatingIfNeeded: NSLeftAlternateKeyMask)
       && nil_or_none(mod_of_kind(ns_alternate_modifier, QCordinary())) {
    modifiers |= UInt32(optionKey)
  }

  if (flags & UInt32(truncatingIfNeeded: NSRightCommandKeyMask))
       == UInt32(truncatingIfNeeded: NSRightCommandKeyMask)
       && nil_or_none(mod_of_kind(right_mod(ns_command_modifier, ns_right_command_modifier),
                                  QCordinary())) {
    /* Carbon doesn't differentiate between left and right command
       keys.  */
    modifiers |= UInt32(cmdKey)
  }

  if (flags & UInt32(truncatingIfNeeded: NSLeftCommandKeyMask))
       == UInt32(truncatingIfNeeded: NSLeftCommandKeyMask)
       && nil_or_none(mod_of_kind(ns_command_modifier, QCordinary())) {
    modifiers |= UInt32(cmdKey)
  }

  result = UCKeyTranslate(layout, event.keyCode, UInt16(kUCKeyActionDown),
                          (modifiers >> 8) & 0xFF, UInt32(LMGetKbdType()),
                          OptionBits(kUCKeyTranslateNoDeadKeysBit), &dead_key_state,
                          max_string_length, &actual_string_length, &buf)

  if result != 0 {
    // C 2751-2752: `NSLog (@"Failed to translate character '%@' with modifiers %x",
    // [event characters], modifiers);` - `%@` bridges the Swift `String`.
    NSLog("Failed to translate character '%@' with modifiers %x", event.characters ?? "", modifiers)
    return 0
  }

  /* FIXME: What do we do if more than one code unit is returned?  */
  if actual_string_length > 0 {
    return buf[0]
  }

  return 0
}

/* ==========================================================================

    Block drawing operations

   ========================================================================== */

// C 2771-2786's `ns_redraw_scroll_bars` is inside `#ifdef NS_IMPL_GNUSTEP` - dead for this lane,
// recorded in the ledger, not transcribed.  The section's live definitions start at C 2789.

/// C: `#define FRAME_ROOT_WINDOW(f) f->root_window` (src/frame.h 1201) - read at C 2805.
@_silgen_name("FRAME_ROOT_WINDOW")
func FRAME_ROOT_WINDOW(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
/// C: `struct window *XWINDOW (Lisp_Object)` (src/window.h) - the lane's reading is
/// `docs/ports/nsfns.swift` 195.
@_silgen_name("XWINDOW") func XWINDOW(_ a: Lisp_Object) -> UnsafeMutableRawPointer?
/// C: `void mark_window_cursors_off (struct window *)` (src/dispextern.h) - the core's own
/// `femacs/src/window.rs` 5788 defines it.
@_silgen_name("mark_window_cursors_off")
func mark_window_cursors_off(_ w: UnsafeMutableRawPointer?)
/// C: `#define FRAME_DEFAULT_FACE(f) FACE_FROM_ID_OR_NULL (f, DEFAULT_FACE_ID)` (src/nsterm.h 1043) -
/// the macro's own name is the entry, the reading `FRAME_TOOLTIP_P`/`FRAME_BACKGROUND_COLOR` take.
@_silgen_name("FRAME_DEFAULT_FACE")
func FRAME_DEFAULT_FACE(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
/// C: the `struct face` accessors this section reads - `#define NS_FACE_BACKGROUND(f) ((f)->background)`
/// (src/nsterm.h 1041), the `FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL` pair (src/dispextern.h) and the
/// three face-id enumerators (`DEFAULT_FACE_ID`, `CHILD_FRAME_BORDER_FACE_ID`,
/// `INTERNAL_BORDER_FACE_ID`).  `struct face` exists only under `HAVE_WINDOW_SYSTEM`, so these are
/// its gated-arm entries; the status-3.271 layout probe is the route for a *view* of that record,
/// which this unit does not need - every access here is a member entry.
@_silgen_name("NS_FACE_BACKGROUND")
func NS_FACE_BACKGROUND(_ face: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("FACE_FROM_ID")
func FACE_FROM_ID(_ f: UnsafeMutableRawPointer?, _ id: Int32) -> UnsafeMutableRawPointer?
@_silgen_name("FACE_FROM_ID_OR_NULL")
func FACE_FROM_ID_OR_NULL(_ f: UnsafeMutableRawPointer?, _ id: Int32) -> UnsafeMutableRawPointer?
@_silgen_name("DEFAULT_FACE_ID") func DEFAULT_FACE_ID() -> Int32
@_silgen_name("CHILD_FRAME_BORDER_FACE_ID") func CHILD_FRAME_BORDER_FACE_ID() -> Int32
@_silgen_name("INTERNAL_BORDER_FACE_ID") func INTERNAL_BORDER_FACE_ID() -> Int32
/// C: `Vface_remapping_alist` - src/xfaces.c's `DEFVAR_LISP ("face-remapping-alist", …)`, a
/// generated global (the objects hold no symbol of its own), so the core exports a data symbol of
/// its own name - the `Vframe_list` reading.
@_silgen_name("Vface_remapping_alist") var Vface_remapping_alist: Lisp_Object
/// C: `int lookup_basic_face (struct window *, struct frame *, int)` (src/xfaces.c) - the core's
/// `femacs/src/xfaces.rs` 3753; its return is the **face id** the calls above hand to
/// `FACE_FROM_ID_OR_NULL`.
@_silgen_name("lookup_basic_face")
func lookup_basic_face(_ w: UnsafeMutableRawPointer?, _ f: UnsafeMutableRawPointer?,
                       _ face_id: Int32) -> Int32
/// C: src/dispextern.h 883's `ANY_AREA` (`enum glyph_row_area`'s `ANY_AREA = -1`) - the area
/// argument `window_box` takes (src/dispextern.h 3574).
@_silgen_name("ANY_AREA") func ANY_AREA() -> Int32
/// C: `struct run`'s members (src/dispextern.h: `int desired_y, current_y; … int height, nrows;`),
/// read at C 2865-2874 through the `<record>_<member>` family.
@_silgen_name("run_current_y") func run_current_y(_ run: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("run_desired_y") func run_desired_y(_ run: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("run_height") func run_height(_ run: UnsafeMutableRawPointer?) -> Int32
/// C: `struct window`'s member `w->frame` (a `Lisp_Object`), read at C 2855 - the
/// `window_<member>` family.
@_silgen_name("window_frame") func window_frame(_ w: UnsafeMutableRawPointer?) -> Lisp_Object
/// C: `#define FRAME_INTERNAL_BORDER_WIDTH(f)` / `FRAME_PIXEL_WIDTH(f)` / `FRAME_PIXEL_HEIGHT(f)`
/// (src/frame.h) and the two margin heights at C 2929-2930 - `FRAME_TOP_MARGIN_HEIGHT (F)`
/// (src/frame.h 1139's sum of `FRAME_MENU_BAR_HEIGHT`/`FRAME_TAB_BAR_HEIGHT`/
/// `FRAME_TOOL_BAR_TOP_HEIGHT`) and `FRAME_BOTTOM_MARGIN_HEIGHT (f)` (1151's
/// `FRAME_TOOL_BAR_BOTTOM_HEIGHT`).  Their bodies read `struct frame` members and other `FRAME_*`
/// macros, so each keeps its own name as an entry (class A) instead of expanding into `(f)->…`:
/// the expansion would owe four member entries where one name is owed.  (The AppKit-only body of
/// `FRAME_NS_TITLEBAR_HEIGHT` is the case that *is* expanded.)
@_silgen_name("FRAME_INTERNAL_BORDER_WIDTH")
func FRAME_INTERNAL_BORDER_WIDTH(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_PIXEL_WIDTH") func FRAME_PIXEL_WIDTH(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_PIXEL_HEIGHT") func FRAME_PIXEL_HEIGHT(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_TOP_MARGIN_HEIGHT")
func FRAME_TOP_MARGIN_HEIGHT(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_BOTTOM_MARGIN_HEIGHT")
func FRAME_BOTTOM_MARGIN_HEIGHT(_ f: UnsafeMutableRawPointer?) -> Int32
/// C: `void gui_clear_cursor (struct window *)` (src/dispextern.h; defined src/xdisp.c 35466
/// **inside `HAVE_WINDOW_SYSTEM`** - the `#ifdef` at src/xdisp.c 35045 - so the frozen objects have
/// no
/// `_gui_clear_cursor`, measured): the owed-entry class of status 3.255 ③, like
/// `remember_mouse_glyph`.
@_silgen_name("gui_clear_cursor") func gui_clear_cursor(_ w: UnsafeMutableRawPointer?)
/// C: `void NSRectFill (NSRect)` (AppKit's NSGraphics.h) - Swift marks the `NSRect*` family
/// unavailable, so it is reached by name, the reading the `NSRectClip`/`NSRectClipList` pair above
/// already records.
@_silgen_name("NSRectFill") func NSRectFill(_ rect: NSRect)
/// C 2909: `[view copyRect:srcRect to:dest];` - `EmacsView -copyRect:to:`, **this file's own
/// class** (C 9340), so the call site takes the round-5 ruling's deferred shape with the first
/// selector segment as the entry name (the `beginDrag` reading, `docs/ports/nsselect.swift` 220-223).
@_silgen_name("copyRect")
func copyRect_entry(_ view: NSView?, _ srcRect: NSRect, _ dest: NSPoint)

/// C 2788-2820: `void ns_clear_frame (struct frame *f)` - "External (hook): Erase the entire
/// frame".  Declared src/nsterm.h 1124 (the `clear_frame_hook`), so it keeps C's name under
/// `@_cdecl`.  C's `NS_IMPL_GNUSTEP` arm at C 2816-2818 (`ns_redraw_scroll_bars (f)`) is dead here:
/// that function is GNUstep-only (C 2771-2786).
@_cdecl("ns_clear_frame")
func ns_clear_frame(_ f: UnsafeMutableRawPointer?) {
  let view = FRAME_NS_VIEW(f)
  var r = NSRect.zero

  // C 2798: `NSTRACE_WHEN (NSTRACE_GROUP_UPDATES, "ns_clear_frame");` - empty at
  // `NSTRACE_ENABLED` 0, so neither its group nor its format string is evaluated.

  /* comes on initial frame because we have
     after-make-frame-functions = select-frame */
  if FRAME_DEFAULT_FACE(f) == nil {
    return
  }

  // C 2805: `mark_window_cursors_off (XWINDOW (FRAME_ROOT_WINDOW (f)));`
  mark_window_cursors_off(XWINDOW(FRAME_ROOT_WINDOW(f)))

  // C 2807: `r = [view bounds];`
  r = view?.bounds ?? .zero

  block_input()
  ns_focus(f, &r, 1)
  // C 2811-2812: `[[NSColor colorWithUnsignedLong:NS_FACE_BACKGROUND (FACE_FROM_ID (f,
  // DEFAULT_FACE_ID))] set];` - the category's `+colorWithUnsignedLong:` above.
  NSColor.colorWithUnsignedLong(NS_FACE_BACKGROUND(FACE_FROM_ID(f, DEFAULT_FACE_ID()))).set()
  NSRectFill(r)
  ns_unfocus(f)

  unblock_input()
}

/// C 2823-2846: `static void ns_clear_frame_area (struct frame *f, int x, int y, int width,
/// int height)` - "External (RIF):  Clear section of frame".  C's `static`, so it lands as a
/// `fileprivate func`.
fileprivate func ns_clear_frame_area(_ f: UnsafeMutableRawPointer?, _ x: Int32, _ y: Int32,
                                     _ width: Int32, _ height: Int32) {
  var r = NSMakeRect(CGFloat(x), CGFloat(y), CGFloat(width), CGFloat(height))
  let view = FRAME_NS_VIEW(f)
  let face = FRAME_DEFAULT_FACE(f)

  if view == nil || face == nil {
    return
  }

  // C 2836: `NSTRACE_WHEN (NSTRACE_GROUP_UPDATES, "ns_clear_frame_area");` - empty, as above.

  // C 2838: `r = NSIntersectionRect (r, [view frame]);`
  r = NSIntersectionRect(r, view!.frame)
  ns_focus(f, &r, 1)
  NSColor.colorWithUnsignedLong(NS_FACE_BACKGROUND(face)).set()

  NSRectFill(r)

  ns_unfocus(f)
  return
}

/// C 2849-2916: `static void ns_scroll_run (struct window *w, struct run *run)` - "External
/// (RIF):  Insert or delete n lines at line vpos."  The `run->…` reads are `struct run` member
/// entries, and `[view copyRect:to:]` is this unit's own `EmacsView` method (C 9340), i.e. the
/// deferred entry above.  The `#if defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MAX_ALLOWED <
/// 101400` arm at C 2910-2912 is dead for this lane (the SDK's value is 260000) and is recorded,
/// not transcribed.
fileprivate func ns_scroll_run(_ w: UnsafeMutableRawPointer?, _ run: UnsafeMutableRawPointer?) {
  let f = XFRAME(window_frame(w))
  var x: Int32 = 0, y: Int32 = 0, width: Int32 = 0, height: Int32 = 0
  var from_y: Int32 = 0, to_y: Int32 = 0, bottom_y: Int32 = 0

  // C 2858: `NSTRACE ("ns_scroll_run");` - empty, as above.

  /* Get frame-relative bounding box of the text display area of W,
     without mode lines.  Include in this box the left and right
     fringe of W.  */
  window_box(w, ANY_AREA(), &x, &y, &width, &height)

  from_y = WINDOW_TO_FRAME_PIXEL_Y(w, run_current_y(run))
  to_y = WINDOW_TO_FRAME_PIXEL_Y(w, run_desired_y(run))
  bottom_y = y + height

  if to_y < from_y {
    /* Scrolling up.  Make sure we don't copy part of the mode
       line at the bottom.  */
    if from_y + run_height(run) > bottom_y {
      height = bottom_y - from_y
    } else {
      height = run_height(run)
    }

    /* If the destination is off the top of the current window, don't
       draw over the window above.  */
    if to_y < y {
      let d = y - to_y
      height -= d
      to_y += d
      from_y += d
    }
  } else {
    /* Scrolling down.  Make sure we don't copy over the mode line.
       at the bottom.  */
    if to_y + run_height(run) > bottom_y {
      height = bottom_y - to_y
    } else {
      height = run_height(run)
    }
  }

  if height == 0 {
    return
  }

  block_input()

  gui_clear_cursor(w)

  do {
    let srcRect = NSMakeRect(CGFloat(x), CGFloat(from_y), CGFloat(width), CGFloat(height))
    let dest = NSMakePoint(CGFloat(x), CGFloat(to_y))
    let view = FRAME_NS_VIEW(f)

    copyRect_entry(view, srcRect, dest)
  }

  unblock_input()
}

/// C 2919-2960: `static void ns_clear_under_internal_border (struct frame *f)` - the RIF's
/// internal-border clearing, `fileprivate` because C's is a `static`.  C's `int face_id = (…)` is
/// the three-way conditional over `FRAME_PARENT_FRAME`/`Vface_remapping_alist`, and the id it
/// settles on goes to `FACE_FROM_ID_OR_NULL` (with `FRAME_DEFAULT_FACE` as the fallback).
fileprivate func ns_clear_under_internal_border(_ f: UnsafeMutableRawPointer?) {
  // C 2922: `NSTRACE ("ns_clear_under_internal_border");` - empty, as above.

  if FRAME_LIVE_P(f) && FRAME_INTERNAL_BORDER_WIDTH(f) > 0 {
    let border = FRAME_INTERNAL_BORDER_WIDTH(f)
    let width = FRAME_PIXEL_WIDTH(f)
    let height = FRAME_PIXEL_HEIGHT(f)
    let margin = FRAME_TOP_MARGIN_HEIGHT(f)
    let bottom_margin = FRAME_BOTTOM_MARGIN_HEIGHT(f)
    let face_id =
      (FRAME_PARENT_FRAME(f) != nil
       ? (!NILP(Vface_remapping_alist)
          ? lookup_basic_face(nil, f, CHILD_FRAME_BORDER_FACE_ID())
          : CHILD_FRAME_BORDER_FACE_ID())
       : (!NILP(Vface_remapping_alist)
          ? lookup_basic_face(nil, f, INTERNAL_BORDER_FACE_ID())
          : INTERNAL_BORDER_FACE_ID()))
    var face = FACE_FROM_ID_OR_NULL(f, face_id)

    if face == nil {
      face = FRAME_DEFAULT_FACE(f)
    }

    /* Sometimes with new frames we reach this point and have no
       face.  I'm not sure why we have a live frame but no face, so
       just give up.  */
    if face == nil {
      return
    }

    ns_focus(f, nil, 1)
    NSColor.colorWithUnsignedLong(NS_FACE_BACKGROUND(face)).set()
    NSRectFill(NSMakeRect(0, CGFloat(margin), CGFloat(width), CGFloat(border)))
    NSRectFill(NSMakeRect(0, 0, CGFloat(border), CGFloat(height)))
    NSRectFill(NSMakeRect(0, CGFloat(margin), CGFloat(width), CGFloat(border)))
    NSRectFill(NSMakeRect(CGFloat(width - border), 0, CGFloat(border), CGFloat(height)))
    NSRectFill(NSMakeRect(0, CGFloat(height - bottom_margin - border),
                          CGFloat(width), CGFloat(border)))
    ns_unfocus(f)
  }
}

// ==========================================================================
// C 2963-3079: the RIF's fringe-update preparation, the glyph shift, and the
// overhang computation (with the "Character encoding and metrics" banner).
// ==========================================================================

/// C: `struct window`'s member `w->pseudo_window_p` (src/window.h) - read at C 2977, the
/// `window_<member>` family round 13 opened.
@_silgen_name("window_pseudo_window_p")
func window_pseudo_window_p(_ w: UnsafeMutableRawPointer?) -> Bool
/// C: `struct glyph_row`'s members this slice reads and the one it writes -
/// `desired_row->mode_line_p` (C 2977), `desired_row->full_width_p` (C 2983) and
/// `desired_row->redraw_fringe_bitmaps_p = 1` (C 2978, a **bitfield write**, the
/// `mouse_highlight_set_mouse_face_defer` shape).  `glyph_row_y`/`glyph_row_visible_height`
/// above are the family's first two rows.
@_silgen_name("glyph_row_mode_line_p")
func glyph_row_mode_line_p(_ row: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_row_full_width_p")
func glyph_row_full_width_p(_ row: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_row_set_redraw_fringe_bitmaps_p")
func glyph_row_set_redraw_fringe_bitmaps_p(_ row: UnsafeMutableRawPointer?, _ v: Bool)
/// C: `int windows_or_buffers_changed` (src/xdisp.c) - a plain global, and the reference objects
/// hold it (`C`, common, in `build/emacs-ref/src/xdisp.o`), so the core exports a data symbol of
/// its own name - the `Vframe_list` reading.
@_silgen_name("windows_or_buffers_changed") var windows_or_buffers_changed: Int32
/// C: `struct glyph_string`'s members (src/dispextern.h): the two overhangs this slice **writes**
/// (C 3070-3077 - their *reads* are elsewhere in the file and get their own entries when that
/// slice arrives, so only the `_set_` pair is declared here; a declared-but-uncalled entry is
/// dropped from the object and never reaches `nm -u`, measured this round),
/// `char2b`/`nchars`/`font` (C 3062), `cmp`/`cmp_id`/`cmp_from`/`cmp_to` (C 3053/3066/3068) and
/// `first_glyph` (C 3054).  This is the family's first slice - the record belongs to the core
/// (`femacs/src/dispextern.rs`), so each access is a member entry under `<record>_<member>`.
@_silgen_name("glyph_string_set_left_overhang")
func glyph_string_set_left_overhang(_ s: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("glyph_string_set_right_overhang")
func glyph_string_set_right_overhang(_ s: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("glyph_string_char2b")
func glyph_string_char2b(_ s: UnsafeMutableRawPointer?) -> UnsafePointer<UInt32>?
@_silgen_name("glyph_string_nchars") func glyph_string_nchars(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_font")
func glyph_string_font(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_cmp")
func glyph_string_cmp(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_cmp_id")
func glyph_string_cmp_id(_ s: UnsafeMutableRawPointer?) -> Int
@_silgen_name("glyph_string_cmp_from")
func glyph_string_cmp_from(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_cmp_to") func glyph_string_cmp_to(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_first_glyph")
func glyph_string_first_glyph(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
/// C: `struct glyph`'s `unsigned type : 3;` (src/dispextern.h 514) - the bitfield read at
/// C 3054/3055/3059 - and the two `enum glyph_type` enumerators it is compared with
/// (src/dispextern.h 431-438).
@_silgen_name("glyph_type") func glyph_type(_ g: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("CHAR_GLYPH") func CHAR_GLYPH() -> Int32
@_silgen_name("COMPOSITE_GLYPH") func COMPOSITE_GLYPH() -> Int32
/// C: `struct composition`'s members (src/composite.h 148; `short pixel_width, ascent, descent;
/// short lbearing, rbearing;`), read at C 3076-3077.
@_silgen_name("composition_pixel_width")
func composition_pixel_width(_ cmp: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("composition_lbearing")
func composition_lbearing(_ cmp: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("composition_rbearing")
func composition_rbearing(_ cmp: UnsafeMutableRawPointer?) -> Int32
/// C: `struct font`'s member `driver` (`struct font_driver *`, src/font.h) and the driver slot
/// `void (*text_extents) (struct font *, const unsigned *, int, struct font_metrics *)`
/// (src/font.h 672-673, ~offset 104 in that table - the slot `macfont.swift` 11160-11162
/// declares).  C reaches the call as `font->driver->text_extents (…)`; the port reads the two
/// hops as entries and calls what the second returns, so no `struct font`/`struct font_driver`
/// view is needed here.
typealias font_text_extents_fn = @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<UInt32>?,
                                                Int32, UnsafeMutableRawPointer?) -> Void
@_silgen_name("font_driver") func font_driver(_ font: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("font_driver_text_extents")
func font_driver_text_extents(_ driver: UnsafeMutableRawPointer?) -> font_text_extents_fn?
/// C: the two core functions of C 3062/3068 - `femacs/src/composite.rs` 758 and 829 define them,
/// so they are provided, not owed.  `composition_gstring_width`'s `metrics` is a
/// `struct font_metrics *`, i.e. the pointer the local view above is handed through.
@_silgen_name("composition_gstring_from_id")
func composition_gstring_from_id(_ id: Int) -> Lisp_Object
@_silgen_name("composition_gstring_width")
func composition_gstring_width(_ gstring: Lisp_Object, _ from: Int, _ to: Int,
                               _ metrics: UnsafeMutableRawPointer?) -> Int32

/// C 2963-3019: `static void ns_after_update_window_line (struct window *w, struct glyph_row
/// *desired_row)` - "External (RIF): preparatory to fringe update after text was updated".
/// Two of C's spellings are re-shaped: `eassert (w)` is the no-op it is when `ENABLE_CHECKING`
/// is off (recorded, as the `NSTRACE` family), and the condition's comma expressions
/// (`(f = XFRAME (w->frame), width = …, width != 0)`) become the nested `if`s below - the same
/// short-circuit, with the three pure reads where C evaluates them.
fileprivate func ns_after_update_window_line(_ w: UnsafeMutableRawPointer?,
                                             _ desired_row: UnsafeMutableRawPointer?) {
  var f: UnsafeMutableRawPointer? = nil
  var width: Int32 = 0
  var height: Int32 = 0

  // C 2972: `NSTRACE_WHEN (NSTRACE_GROUP_UPDATES, "ns_after_update_window_line");` - empty at
  // `NSTRACE_ENABLED` 0, so neither its group nor its format string is evaluated.

  /* begin copy from other terms */
  // C 2975: `eassert (w);` - the empty macro unless `ENABLE_CHECKING`, which this configuration
  // leaves undefined (the reading the dead `NSTRACE_ENABLED` block above already carries).

  if !glyph_row_mode_line_p(desired_row) && !window_pseudo_window_p(w) {
    glyph_row_set_redraw_fringe_bitmaps_p(desired_row, true)
  }

  /* When a window has disappeared, make sure that no rest of
     full-width rows stays visible in the internal border.  */
  if windows_or_buffers_changed != 0
       && glyph_row_full_width_p(desired_row) {
    f = XFRAME(window_frame(w))
    width = FRAME_INTERNAL_BORDER_WIDTH(f)

    if width != 0 {
      height = glyph_row_visible_height(desired_row)

      if height > 0 {
        // C 2990: `int y = WINDOW_TO_FRAME_PIXEL_Y (w, max (0, desired_row->y));` - C's `max`
        // is the two-argument macro (`lisp.h`/`sysdep`'s), expressible in Swift as
        // `Swift.max` (class B of the 3.255 ruling: the body is arithmetic, no core fact).
        let y = WINDOW_TO_FRAME_PIXEL_Y(w, max(0, glyph_row_y(desired_row)))
        let face_id =
          !NILP(Vface_remapping_alist)
          ? lookup_basic_face(nil, f, INTERNAL_BORDER_FACE_ID())
          : INTERNAL_BORDER_FACE_ID()
        let face = FACE_FROM_ID_OR_NULL(f, face_id)

        block_input()
        if face != nil {
          var r = NSMakeRect(0, CGFloat(y), CGFloat(FRAME_PIXEL_WIDTH(f)), CGFloat(height))
          ns_focus(f, &r, 1)

          NSColor.colorWithUnsignedLong(NS_FACE_BACKGROUND(face)).set()
          NSRectFill(NSMakeRect(0, CGFloat(y), CGFloat(width), CGFloat(height)))
          NSRectFill(NSMakeRect(CGFloat(FRAME_PIXEL_WIDTH(f) - width),
                                CGFloat(y), CGFloat(width), CGFloat(height)))

          ns_unfocus(f)
        } else {
          ns_clear_frame_area(f, 0, y, width, height)
          ns_clear_frame_area(f,
                              FRAME_PIXEL_WIDTH(f) - width,
                              y, width, height)
        }
        unblock_input()
      }
    }
  }
}

/// C 3022-3036: `static void ns_shift_glyphs_for_insert (struct frame *f, int x, int y, int
/// width, int height, int shift_by)` - "External (RIF): copy an area horizontally, don't worry
/// about clearing src".  The copy is the `EmacsView -copyRect:to:` entry round 13 added.
fileprivate func ns_shift_glyphs_for_insert(_ f: UnsafeMutableRawPointer?, _ x: Int32, _ y: Int32,
                                            _ width: Int32, _ height: Int32, _ shift_by: Int32) {
  let srcRect = NSMakeRect(CGFloat(x), CGFloat(y), CGFloat(width), CGFloat(height))
  let dest = NSMakePoint(CGFloat(x + shift_by), CGFloat(y))

  // C 3033: `NSTRACE ("ns_shift_glyphs_for_insert");` - empty, as above.

  copyRect_entry(FRAME_NS_VIEW(f), srcRect, dest)
}

/* ==========================================================================

    Character encoding and metrics

   ========================================================================== */

/// C 3047-3079: `static void ns_compute_glyph_string_overhangs (struct glyph_string *s)` -
/// "External (RIF); compute left/right overhang of whole string and set in s".  C's
/// `struct font_metrics metrics;` is a local the driver slot fills through a pointer, which is
/// why the unit declares the view above; `font->driver->text_extents (…)` is the two-hop read the
/// entries `font_driver`/`font_driver_text_extents` spell (C calls the slot unconditionally, and
/// the port's `if let` is the nil check C's driver contract makes unnecessary).
fileprivate func ns_compute_glyph_string_overhangs(_ s: UnsafeMutableRawPointer?) {
  if glyph_string_cmp(s) == nil
       && (glyph_type(glyph_string_first_glyph(s)) == CHAR_GLYPH()
           || glyph_type(glyph_string_first_glyph(s)) == COMPOSITE_GLYPH()) {
    var metrics = font_metrics(lbearing: 0, rbearing: 0, width: 0, ascent: 0, descent: 0)

    if glyph_type(glyph_string_first_glyph(s)) == CHAR_GLYPH() {
      let font = glyph_string_font(s)
      if let text_extents = font_driver_text_extents(font_driver(font)) {
        text_extents(font, glyph_string_char2b(s), glyph_string_nchars(s), &metrics)
      }
    } else {
      let gstring = composition_gstring_from_id(glyph_string_cmp_id(s))

      _ = composition_gstring_width(gstring, Int(glyph_string_cmp_from(s)),
                                    Int(glyph_string_cmp_to(s)), &metrics)
    }
    // C 3070-3072: `metrics.rbearing > metrics.width ? metrics.rbearing - metrics.width : 0`
    // (the `short` fields promote to `int` in C's arithmetic, hence the `Int32` conversions) and
    // `metrics.lbearing < 0 ? - metrics.lbearing : 0`.
    glyph_string_set_right_overhang(s, metrics.rbearing > metrics.width
                                       ? Int32(metrics.rbearing) - Int32(metrics.width) : 0)
    glyph_string_set_left_overhang(s, metrics.lbearing < 0 ? -Int32(metrics.lbearing) : 0)
  } else if glyph_string_cmp(s) != nil {
    glyph_string_set_right_overhang(s, composition_rbearing(glyph_string_cmp(s))
                                       - composition_pixel_width(glyph_string_cmp(s)))
    glyph_string_set_left_overhang(s, -composition_lbearing(glyph_string_cmp(s)))
  }
}

/* ==========================================================================

    Fringe and cursor drawing

   ========================================================================== */

/// C 3089: `static NSMutableDictionary *fringe_bmp;` - this file's own file-scope table, a
/// `fileprivate var` of the same name (C zero-initialises it to NULL, which is `nil` here).
fileprivate var fringe_bmp: NSMutableDictionary? = nil

/// C: `#define WINDOW_FRAME(w) ((w)->frame)` (src/frame.h 929) - the macro's own name, read at
/// C 3228.  The bare member read (`w->frame`, round 13's `window_frame`) and this macro spelling
/// keep one entry each: PORTING.md 0.2's names-verbatim rule, the way `NS_FACE_BACKGROUND` and
/// `face_background` are two entries below.
@_silgen_name("WINDOW_FRAME") func WINDOW_FRAME(_ w: UnsafeMutableRawPointer?) -> Lisp_Object
/// C: `struct face`'s members `foreground`/`background` (src/dispextern.h), read **bare** at
/// C 3254/3279/3281 - C does not go through `NS_FACE_BACKGROUND` there, so each spelling gets its
/// own `<record>_<member>` entry (round 13's `NS_FACE_BACKGROUND` is the macro's).
@_silgen_name("face_foreground") func face_foreground(_ face: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("face_background") func face_background(_ face: UnsafeMutableRawPointer?) -> UInt
/// C: `struct draw_fringe_bitmap_params` (src/dispextern.h) - the record `ns_draw_fringe_bitmap`
/// reads member by member (`face` C 3229, `which` C 3235, `cursor_p`/`overlay_p` C 3235,
/// `wd`/`h`/`dh` C 3235-3238, `x`/`y` C 3238, `bx`/`by`/`nx`/`ny` C 3240-3241), plus the two
/// `bool_bf` bitfields.  The record belongs to the core, so each is a member entry.
@_silgen_name("draw_fringe_bitmap_params_face")
func draw_fringe_bitmap_params_face(_ p: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("draw_fringe_bitmap_params_which")
func draw_fringe_bitmap_params_which(_ p: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("draw_fringe_bitmap_params_wd")
func draw_fringe_bitmap_params_wd(_ p: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("draw_fringe_bitmap_params_h")
func draw_fringe_bitmap_params_h(_ p: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("draw_fringe_bitmap_params_dh")
func draw_fringe_bitmap_params_dh(_ p: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("draw_fringe_bitmap_params_x")
func draw_fringe_bitmap_params_x(_ p: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("draw_fringe_bitmap_params_y")
func draw_fringe_bitmap_params_y(_ p: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("draw_fringe_bitmap_params_bx")
func draw_fringe_bitmap_params_bx(_ p: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("draw_fringe_bitmap_params_by")
func draw_fringe_bitmap_params_by(_ p: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("draw_fringe_bitmap_params_nx")
func draw_fringe_bitmap_params_nx(_ p: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("draw_fringe_bitmap_params_ny")
func draw_fringe_bitmap_params_ny(_ p: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("draw_fringe_bitmap_params_cursor_p")
func draw_fringe_bitmap_params_cursor_p(_ p: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("draw_fringe_bitmap_params_overlay_p")
func draw_fringe_bitmap_params_overlay_p(_ p: UnsafeMutableRawPointer?) -> Bool
/// C: `f->output_data.ns->cursor_color` (C 3283) - the `struct ns_output` member read, under the
/// family name the shim already exports (`docs/ports/abi_shim.md` 74; `nsfns.swift` 282 imports
/// it with this signature).
@_silgen_name("ns_output_cursor_color")
func ns_output_cursor_color(_ f: UnsafeMutableRawPointer?) -> NSColor?
/// C: `void gui_define_fringe_bitmap (struct frame *, int)` (defined src/fringe.c 1853) and the
/// global `int max_used_fringe_bitmap = MAX_STANDARD_FRINGE_BITMAPS;` (src/fringe.c 497).
/// **Neither has a reference-side symbol** (measured): the frozen configuration compiles no
/// `fringe.c` at all - `femacs/src/fringe.rs` is the empty unit status 3.31 records, its note
/// quoting the build's empty platform object variables - so both are owed entries, and the
/// function's row carries the gate the file itself has.
@_silgen_name("gui_define_fringe_bitmap")
func gui_define_fringe_bitmap(_ f: UnsafeMutableRawPointer?, _ n: Int32)
@_silgen_name("max_used_fringe_bitmap") var max_used_fringe_bitmap: Int32

/// C 3091-3195: `static void ns_define_fringe_bitmap (int which, unsigned short *bits, int h,
/// int w)` - the tracer that turns the bitmap's bit rows into an `NSBezierPath` and caches it in
/// `fringe_bmp`.  C's `alloca` is `withUnsafeTemporaryAllocation` (the reading `macfont.swift`
/// 1684/2978 gives), so the `points`/`cur` pointer walk keeps its shape over the temporary
/// buffer; the C `char` state variables hold the ASCII letters `'n'`/`'e'`/`'s'`/`'w'` and are
/// `CChar` here.
fileprivate func ns_define_fringe_bitmap(_ which: Int32, _ bits: UnsafePointer<UInt16>?, _ h: Int32,
                                         _ w: Int32) {
  let p = NSBezierPath()

  if fringe_bmp == nil {
    fringe_bmp = NSMutableDictionary(capacity: 25)
  }

  withUnsafeTemporaryAllocation(of: UInt8.self, capacity: Int((h + 1) * (w + 1) * 4)) { points in
    var cur = points.baseAddress!

    /* Find all the outgoing edges in a clockwise path.  That is, we only
       want to list the edges leaving a point, not the ones entering a
       point, so we don't double count them.  */
    for y in 0..<(h + 1) {
      for x in 0..<(w + 1) {
        var nw: Int32 = 0, ne: Int32 = 0, se: Int32 = 0, sw: Int32 = 0
        if x != 0 && y != 0 {
          nw = Int32(bits![Int(y - 1)]) & (1 << (w - x))
        }

        if x != 0 && y < h {
          sw = Int32(bits![Int(y)]) & (1 << (w - x))
        }

        if x < w && y < h {
          se = Int32(bits![Int(y)]) & (1 << (w - x - 1))
        }

        if x < w && y != 0 {
          ne = Int32(bits![Int(y - 1)]) & (1 << (w - x - 1))
        }

        cur[0] = (nw == 0 && ne != 0) ? 1 : 0 /* North.  */
        cur[1] = (ne == 0 && se != 0) ? 1 : 0 /* East.  */
        cur[2] = (se == 0 && sw != 0) ? 1 : 0 /* South.  */
        cur[3] = (sw == 0 && nw != 0) ? 1 : 0 /* West.  */
        cur += 4
      }
    }

    /* Find all the points with edges and trace them out.  */
    var v: Int32 = 0
    var last: CChar = 0
    while v < (h + 1) * (w + 1) * 4 {
      var this: CChar = 0
      let x = (v / 4) % (w + 1)
      let y = (v / 4) / (w + 1)

      if points[Int(v + 3)] != 0 {
        /* West.  */
        points[Int(v + 3)] = 0
        v = v - 4
        this = CChar(UInt8(ascii: "w"))
      } else if points[Int(v + 1)] != 0 {
        /* East.  */
        points[Int(v + 1)] = 0
        v = v + 4
        this = CChar(UInt8(ascii: "e"))
      } else if points[Int(v + 2)] != 0 {
        /* South.  */
        points[Int(v + 2)] = 0
        v = ((y + 1) * (w + 1) + x) * 4
        this = CChar(UInt8(ascii: "s"))
      } else if points[Int(v)] != 0 {
        /* North.  */
        points[Int(v)] = 0
        v = ((y - 1) * (w + 1) + x) * 4
        this = CChar(UInt8(ascii: "n"))
      } else {
        /* No edge.  */
        v = v + 4

        if last != 0 {
          /* If we reach here we were tracing a shape but have run
             out of edges, so we must be back to the start (or
             something's gone wrong).  */
          p.close()
          last = 0
        }
      }

      if this != 0 {
        /* If we've found an edge we now need to either move to that
           point (if it's the start of a shape) or draw a line from
           the last corner to this point, but only if it's a
           corner.  */
        if last == 0 {
          p.move(to: NSMakePoint(CGFloat(x), CGFloat(y)))
        } else if last != this {
          p.line(to: NSMakePoint(CGFloat(x), CGFloat(y)))
        }
        last = this
      }
    }
  }

  fringe_bmp!.setObject(p, forKey: NSNumber(value: which))
}

/// C 3198-3202: `static void ns_destroy_fringe_bitmap (int which)`.
fileprivate func ns_destroy_fringe_bitmap(_ which: Int32) {
  fringe_bmp?.removeObject(forKey: NSNumber(value: which))
}

/// C 3205-3291: `static void ns_draw_fringe_bitmap (struct window *w, struct glyph_row *row,
/// struct draw_fringe_bitmap_params *p)` - "External (RIF); fringe-related".  The record's
/// members are the entries above; C's `bmp = [bmp copy]` … `[bmp release]` pair is ARC's own
/// copy here (no release call - the reading rounds 3/6 record for `[x release]`), and the
/// `NSTRACE_*` family is empty (`NSTRACE_ENABLED` 0), so C's arguments to it are not evaluated.
fileprivate func ns_draw_fringe_bitmap(_ w: UnsafeMutableRawPointer?, _ row: UnsafeMutableRawPointer?,
                                       _ p: UnsafeMutableRawPointer?) {
  let f = XFRAME(WINDOW_FRAME(w))
  let face = draw_fringe_bitmap_params_face(p)
  var bmpRect = NSZeroRect
  let rowRect = ns_row_rect(w, row, ANY_AREA())

  // C 3233-3235: `NSTRACE_WHEN (NSTRACE_GROUP_FRINGE, "ns_draw_fringe_bitmap");` and
  // `NSTRACE_MSG ("which:%d cursor:%d …", p->which, p->cursor_p, …);` - empty, as above.

  /* Work out the rectangle we will need to clear.  */
  bmpRect = NSMakeRect(CGFloat(draw_fringe_bitmap_params_x(p)),
                       CGFloat(draw_fringe_bitmap_params_y(p)),
                       CGFloat(draw_fringe_bitmap_params_wd(p)),
                       CGFloat(draw_fringe_bitmap_params_h(p)))

  if draw_fringe_bitmap_params_bx(p) >= 0 {
    bmpRect = NSUnionRect(bmpRect,
                          NSMakeRect(CGFloat(draw_fringe_bitmap_params_bx(p)),
                                     CGFloat(draw_fringe_bitmap_params_by(p)),
                                     CGFloat(draw_fringe_bitmap_params_nx(p)),
                                     CGFloat(draw_fringe_bitmap_params_ny(p))))
  }

  /* Handle partially visible rows.  */
  bmpRect = NSIntersectionRect(bmpRect, rowRect)

  /* Clip to the bitmap's area.  */
  ns_focus(f, &bmpRect, 1)

  /* Clear screen unless overlay.  */
  if !draw_fringe_bitmap_params_overlay_p(p) && !NSIsEmptyRect(bmpRect) {
    // C 3252: `NSTRACE_RECT ("clearRect", bmpRect);` - empty, as above.

    NSColor.colorWithUnsignedLong(face_background(face)).set()
    NSRectFill(bmpRect)
  }

  var bmp = fringe_bmp?.object(forKey: NSNumber(value: draw_fringe_bitmap_params_which(p)))
    as? NSBezierPath

  if bmp == nil
       && draw_fringe_bitmap_params_which(p) < max_used_fringe_bitmap {
    gui_define_fringe_bitmap(f, draw_fringe_bitmap_params_which(p))
    bmp = fringe_bmp?.object(forKey: NSNumber(value: draw_fringe_bitmap_params_which(p)))
      as? NSBezierPath
  }

  if bmp != nil {
    let transform = NSAffineTransform()
    var bm_color: NSColor? = nil

    /* Because the image is defined at (0, 0) we need to take a copy
       and then transform that copy to the new origin.  */
    bmp = bmp!.copy() as? NSBezierPath
    transform.translateX(by: CGFloat(draw_fringe_bitmap_params_x(p)),
                         yBy: CGFloat(draw_fringe_bitmap_params_y(p)
                                      - draw_fringe_bitmap_params_dh(p)))
    bmp!.transform(using: transform as AffineTransform)

    if !draw_fringe_bitmap_params_cursor_p(p) {
      bm_color = NSColor.colorWithUnsignedLong(face_foreground(face))
    } else if draw_fringe_bitmap_params_overlay_p(p) {
      bm_color = NSColor.colorWithUnsignedLong(face_background(face))
    } else {
      bm_color = ns_output_cursor_color(f)
    }

    bm_color?.set()
    bmp!.fill()
  }
  ns_unfocus(f)
}

/// C: `#define WINDOW_XFRAME(W) XFRAME (WINDOW_FRAME (W))` (src/window.h 620) - the macro of
/// C 3305/3435/3458, the `WINDOW_FRAME` sibling above.
@_silgen_name("WINDOW_XFRAME")
func WINDOW_XFRAME(_ w: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
/// C: `struct window`'s cursor members, which live in src/window.h's **`HAVE_WINDOW_SYSTEM`**
/// block (window.h 340-350): `phys_cursor_type` (written at C 3319), `phys_cursor_on_p` (3319),
/// `phys_cursor_width` (read at C 3351/3358/3373 and written at 3324/3352) and the nested
/// `phys_cursor.hpos` (3333).  They are **gated** members - the frozen `struct window` has no
/// such fields - so the core exports them under the same gate (status 3.262's shape), and the
/// nested read follows the `window_phys_cursor_x`/`_y` names `nsfns.swift` already imports.
@_silgen_name("window_set_phys_cursor_type")
func window_set_phys_cursor_type(_ w: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("window_set_phys_cursor_on_p")
func window_set_phys_cursor_on_p(_ w: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("window_phys_cursor_width")
func window_phys_cursor_width(_ w: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("window_set_phys_cursor_width")
func window_set_phys_cursor_width(_ w: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("window_phys_cursor_hpos")
func window_phys_cursor_hpos(_ w: UnsafeMutableRawPointer?) -> Int32
/// C: `struct glyph_row`'s members `exact_window_width_line_p` (src/dispextern.h 1115, read at
/// C 3332), `used[1 + LAST_AREA]` (938, read at 3333 through C's own `TEXT_AREA` subscript, which
/// is the entry's second argument the way `frame_alpha`'s index is), `cursor_in_fringe_p` (1118,
/// written at 3335) and `height` (read at 3364-3365).
@_silgen_name("glyph_row_exact_window_width_line_p")
func glyph_row_exact_window_width_line_p(_ row: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_row_used")
func glyph_row_used(_ row: UnsafeMutableRawPointer?, _ area: Int32) -> Int16
@_silgen_name("glyph_row_set_cursor_in_fringe_p")
func glyph_row_set_cursor_in_fringe_p(_ row: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("glyph_row_height") func glyph_row_height(_ row: UnsafeMutableRawPointer?) -> Int32
/// C: `struct glyph`'s members `resolved_level` (`unsigned … : 7`, read at C 3357) and
/// `pixel_width` (`short`, read at 3358) - the `glyph_type` family's next two.
@_silgen_name("glyph_resolved_level") func glyph_resolved_level(_ g: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_pixel_width") func glyph_pixel_width(_ g: UnsafeMutableRawPointer?) -> Int16
/// C: `#define FRAME_CURSOR_WIDTH(f) ((f)->cursor_width)` and
/// `#define FRAME_FOREGROUND_PIXEL(f) ((f)->foreground_pixel)` (src/frame.h 1372/1375), and the
/// existing `FRAME_CURSOR_COLOR` entry (`nsterm.h` 1037's macro, `macfont.swift` 1157 imports it).
@_silgen_name("FRAME_CURSOR_WIDTH") func FRAME_CURSOR_WIDTH(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_FOREGROUND_PIXEL")
func FRAME_FOREGROUND_PIXEL(_ f: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("FRAME_CURSOR_COLOR") func FRAME_CURSOR_COLOR(_ f: UnsafeMutableRawPointer?) -> NSColor?
/// C: the enumerators this slice compares against - src/dispextern.h 885's `TEXT_AREA`
/// (`enum glyph_row_area`), src/dispextern.h 1352's `DRAW_CURSOR` (`enum draw_glyphs_face`), the six
/// `enum text_cursor_kinds` (src/dispextern.h 216's `DEFAULT_CURSOR` -2, NO_CURSOR -1,
/// FILLED_BOX_CURSOR, HOLLOW_BOX_CURSOR, BAR_CURSOR, HBAR_CURSOR) and the four face ids C
/// 3441/3459-3463 read.
@_silgen_name("TEXT_AREA") func TEXT_AREA() -> Int32
@_silgen_name("DRAW_CURSOR") func DRAW_CURSOR() -> Int32
@_silgen_name("DEFAULT_CURSOR") func DEFAULT_CURSOR() -> Int32
@_silgen_name("NO_CURSOR") func NO_CURSOR() -> Int32
@_silgen_name("FILLED_BOX_CURSOR") func FILLED_BOX_CURSOR() -> Int32
@_silgen_name("HOLLOW_BOX_CURSOR") func HOLLOW_BOX_CURSOR() -> Int32
@_silgen_name("BAR_CURSOR") func BAR_CURSOR() -> Int32
@_silgen_name("HBAR_CURSOR") func HBAR_CURSOR() -> Int32
@_silgen_name("VERTICAL_BORDER_FACE_ID") func VERTICAL_BORDER_FACE_ID() -> Int32
@_silgen_name("WINDOW_DIVIDER_FACE_ID") func WINDOW_DIVIDER_FACE_ID() -> Int32
@_silgen_name("WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID")
func WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID() -> Int32
@_silgen_name("WINDOW_DIVIDER_LAST_PIXEL_FACE_ID")
func WINDOW_DIVIDER_LAST_PIXEL_FACE_ID() -> Int32
/// C: `struct glyph *get_phys_cursor_glyph (struct window *)` (src/window.h 1186; defined
/// src/window.c 8228, no gate) - **provided** by the core's `femacs/src/window.rs` 8078.
@_silgen_name("get_phys_cursor_glyph")
func get_phys_cursor_glyph(_ w: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
/// C: the three cursor/fringe helpers no built configuration has a symbol for, all measured:
/// src/xdisp.c 2707's `get_phys_cursor_geometry` and src/xdisp.c 35099's `draw_phys_cursor_glyph`
/// are the two in `HAVE_WINDOW_SYSTEM` arms (`#ifdef`s at src/xdisp.c 2488 and 35045), and
/// src/fringe.c 838's `draw_fringe_bitmap` belongs to the file `femacs/src/fringe.rs` records as
/// not compiled at all.
/// Each is an owed entry whose row carries that gate (status 3.255 ③ / 3.262's shape).
@_silgen_name("get_phys_cursor_geometry")
func get_phys_cursor_geometry(_ w: UnsafeMutableRawPointer?, _ row: UnsafeMutableRawPointer?,
                              _ glyph: UnsafeMutableRawPointer?, _ x: UnsafeMutablePointer<Int32>?,
                              _ y: UnsafeMutablePointer<Int32>?,
                              _ height: UnsafeMutablePointer<Int32>?)
@_silgen_name("draw_phys_cursor_glyph")
func draw_phys_cursor_glyph(_ w: UnsafeMutableRawPointer?, _ row: UnsafeMutableRawPointer?,
                            _ draw: Int32)
@_silgen_name("draw_fringe_bitmap")
func draw_fringe_bitmap(_ w: UnsafeMutableRawPointer?, _ row: UnsafeMutableRawPointer?,
                        _ left_p: Int32)

/// C 3294-3426: `static void ns_draw_window_cursor (struct window *w, struct glyph_row *glyph_row,
/// int x, int y, enum text_cursor_kinds cursor_type, int cursor_width, bool on_p, bool active_p)` -
/// "External call (RIF): draw cursor."  Three of C's spellings are re-shaped and recorded: the
/// assignment inside the condition `(phys_cursor_glyph = get_phys_cursor_glyph (w)) == NULL`
/// becomes the assignment plus the test; the parameter `cursor_width`, which C assigns twice, is
/// shadowed by a local of the same name (Swift parameters are constants); and C's switch has no
/// `default:` because it covers the enum, so the port's `default: break` is the arm Swift needs
/// for an `Int32` switch.  `NSGraphicsContext`'s save/restore pair is C's own and the
/// `NS_IMPL_GNUSTEP` arm (`GSRectClipList`) is dead for this lane.
fileprivate func ns_draw_window_cursor(_ w: UnsafeMutableRawPointer?,
                                       _ glyph_row: UnsafeMutableRawPointer?,
                                       _ x: Int32, _ y: Int32, _ cursor_type: Int32,
                                       _ cursor_width: Int32, _ on_p: Bool, _ active_p: Bool) {
  var r = NSRect.zero
  var fx: Int32 = 0, fy: Int32 = 0, h: Int32 = 0
  let f = WINDOW_XFRAME(w)

  /* If cursor is out of bounds, don't draw garbage.  This can happen
     in mini-buffer windows when switching between echo area glyphs
     and mini-buffer.  */

  // C 3313-3314: `NSTRACE ("ns_draw_window_cursor (on = %d, cursor_type = %d)", on_p,
  // cursor_type);` - empty, as above.

  if !on_p {
    return
  }

  window_set_phys_cursor_type(w, cursor_type)
  window_set_phys_cursor_on_p(w, on_p)

  if cursor_type == NO_CURSOR() {
    window_set_phys_cursor_width(w, 0)
    return
  }

  let phys_cursor_glyph = get_phys_cursor_glyph(w)
  if phys_cursor_glyph == nil {
    // C 3330: `NSTRACE_MSG ("No phys cursor glyph was found!");` - empty, as above.

    if glyph_row_exact_window_width_line_p(glyph_row)
         && window_phys_cursor_hpos(w) >= Int32(glyph_row_used(glyph_row, TEXT_AREA())) {
      glyph_row_set_cursor_in_fringe_p(glyph_row, true)
      draw_fringe_bitmap(w, glyph_row, 0)
    }
    return
  }

  get_phys_cursor_geometry(w, glyph_row, phys_cursor_glyph, &fx, &fy, &h)

  /* The above get_phys_cursor_geometry call set w->phys_cursor_width
     to the glyph width; replace with CURSOR_WIDTH for (V)BAR cursors.  */
  // C 3345-3359: the parameter is C's to reassign; Swift shadows it, so the three later uses
  // below (`max`, the comparison and the branch) read this local.
  var cursor_width = cursor_width
  if cursor_type == BAR_CURSOR() {
    if cursor_width < 1 {
      cursor_width = max(FRAME_CURSOR_WIDTH(f), 1)
    }

    /* The bar cursor should never be wider than the glyph.  */
    if cursor_width < window_phys_cursor_width(w) {
      window_set_phys_cursor_width(w, cursor_width)
    }

    /* If the character under cursor is R2L, draw the bar cursor
       on the right of its glyph, rather than on the left.  */
    let cursor_glyph = get_phys_cursor_glyph(w)
    if (glyph_resolved_level(cursor_glyph) & 1) != 0 {
      fx += Int32(glyph_pixel_width(cursor_glyph)) - window_phys_cursor_width(w)
    }
  }
  /* If we have an HBAR, "cursor_width" MAY specify height.  */
  else if cursor_type == HBAR_CURSOR() {
    var cursor_height = (cursor_width < 1) ? Int32(lrint(0.25 * Double(h))) : cursor_width
    if cursor_height > glyph_row_height(glyph_row) {
      cursor_height = glyph_row_height(glyph_row)
    }
    if h > cursor_height /* Cursor smaller than line height, move down */ {
      fy += h - cursor_height
    }
    h = cursor_height
  }

  r.origin.x = CGFloat(fx)
  r.origin.y = CGFloat(fy)
  r.size.height = CGFloat(h)
  r.size.width = CGFloat(window_phys_cursor_width(w))

  /* Prevent the cursor from being drawn outside the text area.  */
  r = NSIntersectionRect(r, ns_row_rect(w, glyph_row, TEXT_AREA()))

  // C 3378-3386: the `NS_IMPL_COCOA` arm caches the rect for the macOS Accessibility Zoom
  // integration, the global round 5 landed as `ns_UAZoom_cursor_rect_new`.
  if active_p {
    ns_UAZoom_cursor_rect_new = r
  }

  ns_focus(f, nil, 0)

  let ctx = NSGraphicsContext.current
  ctx?.saveGraphicsState()
  // C 3392-3396: the `NS_IMPL_GNUSTEP` arm (`GSRectClipList (ctx, &r, 1)`) is dead; the live arm
  // is `NSRectClip (r)` - this unit's existing entry.
  NSRectClip(r)

  FRAME_CURSOR_COLOR(f)?.set()

  switch cursor_type {
  case DEFAULT_CURSOR(), NO_CURSOR():
    break
  case FILLED_BOX_CURSOR():
    /* The call to draw_phys_cursor_glyph can end up undoing the
       ns_focus, so unfocus here and regain focus later.  */
    ctx?.restoreGraphicsState()
    ns_unfocus(f)
    draw_phys_cursor_glyph(w, glyph_row, DRAW_CURSOR())
    ns_focus(f, &r, 1)
  case HOLLOW_BOX_CURSOR():
    /* This works like it does in PostScript, not X Windows.  */
    NSBezierPath.stroke(NSInsetRect(r, 0.5, 0.5))
    ctx?.restoreGraphicsState()
  case HBAR_CURSOR(), BAR_CURSOR():
    NSRectFill(r)
    ctx?.restoreGraphicsState()
  default:
    // C's switch covers every `enum text_cursor_kinds`; Swift needs the arm for an `Int32` value.
    break
  }

  ns_unfocus(f)
}

/// C 3429-3449: `static void ns_draw_vertical_window_border (struct window *w, int x, int y0,
/// int y1)` - "External (RIF): Draw a vertical line."
fileprivate func ns_draw_vertical_window_border(_ w: UnsafeMutableRawPointer?, _ x: Int32,
                                                _ y0: Int32, _ y1: Int32) {
  let f = XFRAME(WINDOW_FRAME(w))
  var r = NSMakeRect(CGFloat(x), CGFloat(y0), 1, CGFloat(y1 - y0))

  // C 3439: `NSTRACE ("ns_draw_vertical_window_border");` - empty, as above.

  let face = FACE_FROM_ID_OR_NULL(f, VERTICAL_BORDER_FACE_ID())

  ns_focus(f, &r, 1)
  if face != nil {
    NSColor.colorWithUnsignedLong(face_foreground(face)).set()
  }

  NSRectFill(r)
  ns_unfocus(f)
}

/// C 3452-3508: `static void ns_draw_window_divider (struct window *w, int x0, int x1, int y0,
/// int y1)` - "External (RIF): Draw a window divider."
fileprivate func ns_draw_window_divider(_ w: UnsafeMutableRawPointer?, _ x0: Int32, _ x1: Int32,
                                        _ y0: Int32, _ y1: Int32) {
  let f = XFRAME(WINDOW_FRAME(w))
  let face = FACE_FROM_ID_OR_NULL(f, WINDOW_DIVIDER_FACE_ID())
  let face_first = FACE_FROM_ID_OR_NULL(f, WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID())
  let face_last = FACE_FROM_ID_OR_NULL(f, WINDOW_DIVIDER_LAST_PIXEL_FACE_ID())
  let color = face != nil ? face_foreground(face) : FRAME_FOREGROUND_PIXEL(f)
  let color_first = (face_first != nil
                     ? face_foreground(face_first)
                     : FRAME_FOREGROUND_PIXEL(f))
  let color_last = (face_last != nil
                    ? face_foreground(face_last)
                    : FRAME_FOREGROUND_PIXEL(f))
  var divider = NSMakeRect(CGFloat(x0), CGFloat(y0), CGFloat(x1 - x0), CGFloat(y1 - y0))

  // C 3473: `NSTRACE ("ns_draw_window_divider");` - empty, as above.

  ns_focus(f, &divider, 1)

  if (y1 - y0 > x1 - x0) && (x1 - x0 >= 3) {
    /* A vertical divider, at least three pixels wide: Draw first and
       last pixels differently.  */
    NSColor.colorWithUnsignedLong(color_first).set()
    NSRectFill(NSMakeRect(CGFloat(x0), CGFloat(y0), 1, CGFloat(y1 - y0)))
    NSColor.colorWithUnsignedLong(color).set()
    NSRectFill(NSMakeRect(CGFloat(x0 + 1), CGFloat(y0), CGFloat(x1 - x0 - 2),
                          CGFloat(y1 - y0)))
    NSColor.colorWithUnsignedLong(color_last).set()
    NSRectFill(NSMakeRect(CGFloat(x1 - 1), CGFloat(y0), 1, CGFloat(y1 - y0)))
  } else if (x1 - x0 > y1 - y0) && (y1 - y0 >= 3) {
    /* A horizontal divider, at least three pixels high: Draw first and
       last pixels differently.  */
    NSColor.colorWithUnsignedLong(color_first).set()
    NSRectFill(NSMakeRect(CGFloat(x0), CGFloat(y0), CGFloat(x1 - x0), 1))
    NSColor.colorWithUnsignedLong(color).set()
    NSRectFill(NSMakeRect(CGFloat(x0), CGFloat(y0 + 1), CGFloat(x1 - x0),
                          CGFloat(y1 - y0 - 2)))
    NSColor.colorWithUnsignedLong(color_last).set()
    NSRectFill(NSMakeRect(CGFloat(x0), CGFloat(y1 - 1), CGFloat(x1 - x0), 1))
  } else {
    /* In any other case do not draw the first and last pixels
       differently.  */
    NSColor.colorWithUnsignedLong(color).set()
    NSRectFill(divider)
  }

  ns_unfocus(f)
}

/// C 3511-3515: `static void ns_show_hourglass (struct frame *f)` - the RIF
/// `show_hourglass_hook`.  C's whole body is the TODO comment below, so the port keeps the
/// comment and nothing else; the hook's `f` is unused in C too (the RIF table takes the
/// signature, not the argument).
fileprivate func ns_show_hourglass(_ f: UnsafeMutableRawPointer?) {
  /* TODO: add NSProgressIndicator to all frames.  */
}

/// C 3517-3521: `static void ns_hide_hourglass (struct frame *f)` - the RIF
/// `hide_hourglass_hook`, the mirror of the box above; C's body is its TODO comment alone.
fileprivate func ns_hide_hourglass(_ f: UnsafeMutableRawPointer?) {
  /* TODO: remove NSProgressIndicator from all frames.  */
}

/* ==========================================================================

    Glyph drawing operations

   ========================================================================== */

/// C: `int get_glyph_string_clip_rects (struct glyph_string *, NativeRectangle *, int)`
/// (src/dispextern.h 3661) - defined at src/xdisp.c 2495, **inside that file's
/// `#ifdef HAVE_WINDOW_SYSTEM` block (opened at 2488)**, so the frozen configuration has no
/// symbol for it (measured this round): the row carries that gate - status 3.262's shape, the
/// `get_phys_cursor_geometry` reading of round 16.
@_silgen_name("get_glyph_string_clip_rects")
func get_glyph_string_clip_rects(_ s: UnsafeMutableRawPointer?,
                                 _ rects: UnsafeMutablePointer<NSRect>?, _ n: Int32) -> Int32

/// C 3529-3539: `static int ns_get_glyph_string_clip_rect (struct glyph_string *s,
/// NativeRectangle *nr)` - "Wrapper utility to account for internal border width on full-width
/// lines, and allow top full-width rows to hit the frame top.  nr should be pointer to two
/// successive NSRects.  Number of rects actually used is returned."  `NativeRectangle` is
/// `NSRect` in this build (src/nsgui.h 80, not the `HAVE_X_WINDOWS` arm's `XRectangle`), so
/// `nr` is a pointer to two successive `NSRect`s - the shape C's comment names - and both
/// arguments pass straight through to the core's two-rect call.
fileprivate func ns_get_glyph_string_clip_rect(_ s: UnsafeMutableRawPointer?,
                                               _ nr: UnsafeMutablePointer<NSRect>?) -> Int32 {
  let n = get_glyph_string_clip_rects(s, nr, 2)
  return n
}

/// C: `struct glyph_string`'s members (src/dispextern.h 1336-1508) this slice reads - `ybase`
/// (C 3567/3604/3625), `x` (C 3603/3605/3625), `width` (C 3634), `y`/`height` (C 3699), `f`
/// (C 3665), `face` (C 3672), `prev` (C 3684), `w` (C 3703), `hl` (C 3664) and `for_overlaps`
/// (C 3661) - plus the two it **writes** (C 3749-3750, `_set_`).  The record belongs to the core
/// (`femacs/src/dispextern.rs`), so each access is a member entry under `<record>_<member>`,
/// the family round 14 opened; `for_overlaps` is an `unsigned : 3` bitfield, so its entry keeps
/// C's raw value rather than a `Bool`.
@_silgen_name("glyph_string_ybase")
func glyph_string_ybase(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_x") func glyph_string_x(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_y") func glyph_string_y(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_width")
func glyph_string_width(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_height")
func glyph_string_height(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_f")
func glyph_string_f(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_face")
func glyph_string_face(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_prev")
func glyph_string_prev(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_w")
func glyph_string_w(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_hl") func glyph_string_hl(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_for_overlaps")
func glyph_string_for_overlaps(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_underline_thickness")
func glyph_string_underline_thickness(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_set_underline_thickness")
func glyph_string_set_underline_thickness(_ s: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("glyph_string_underline_position")
func glyph_string_underline_position(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_set_underline_position")
func glyph_string_set_underline_position(_ s: UnsafeMutableRawPointer?, _ v: Int32)

/// C 3553-3591: `static void ns_draw_underwave (struct glyph_string *s, EmacsCGFloat width,
/// EmacsCGFloat x)` - "Draw a wavy line under glyph string s.  The wave fills wave_height pixels
/// from y."  C's `int` locals stay `Int32`, `NSPoint a, b` is two `NSPoint`s and the loop's
/// `odd = !odd` is C's `int` toggle spelled as a conditional; `[NSBezierPath
/// strokeLineFromPoint:toPoint:]` is Swift's `NSBezierPath.strokeLine(from:to:)` (AppKit's own
/// rename), and `[[NSGraphicsContext currentContext] saveGraphicsState]` is
/// `NSGraphicsContext.current?.saveGraphicsState()` (the optional receiver C relies on).
fileprivate func ns_draw_underwave(_ s: UnsafeMutableRawPointer?, _ width: CGFloat,
                                   _ x: CGFloat) {
  let wave_height: Int32 = 3, wave_length: Int32 = 2
  var y: Int32 = 0, dx: Int32 = 0, dy: Int32 = 0, odd: Int32 = 0, xmax: Int32 = 0
  var a = NSPoint(), b = NSPoint()
  var waveClip = NSRect.zero

  dx = wave_length
  dy = wave_height - 1
  y = glyph_string_ybase(s) - wave_height + 3
  xmax = Int32(x + width)

  /* Find and set clipping rectangle */
  waveClip = NSMakeRect(x, CGFloat(y), width, CGFloat(wave_height))
  NSGraphicsContext.current?.saveGraphicsState()
  NSRectClip(waveClip)

  /* Draw the waves */
  a.x = x - CGFloat(Int32(x) % dx) + 0.5
  b.x = a.x + CGFloat(dx)
  odd = Int32(a.x / CGFloat(dx)) % 2
  a.y = CGFloat(y) + 0.5
  b.y = a.y

  if odd != 0 {
    a.y += CGFloat(dy)
  } else {
    b.y += CGFloat(dy)
  }

  while a.x <= CGFloat(xmax) {
    NSBezierPath.strokeLine(from: a, to: b)
    a.x = b.x
    a.y = b.y
    b.x += CGFloat(dx)
    b.y = CGFloat(y) + 0.5 + CGFloat(odd * dy)
    odd = odd == 0 ? 1 : 0
  }

  /* Restore previous clipping rectangle(s) */
  NSGraphicsContext.current?.restoreGraphicsState()
}

/// C 3598-3613: `static void ns_draw_dash (struct glyph_string *s, int width, int segment, int
/// offset, int thickness)` - "Draw a dashed underline of thickness THICKNESS and width WIDTH
/// onto the focused frame at a vertical offset of OFFSET from the position of the glyph string
/// S, with each segment SEGMENT pixels in length."  C's `alloc`/`init`/`release`
/// `NSBezierPath` is Swift's `NSBezierPath()` (ARC owns the local, so the `[path release]` is
/// dropped), the `CGFloat pattern[2]` is a local array, and `y_center` keeps C's
/// `s->ybase + offset + thickness / 2.0` grouping (the last term is the division C writes).
fileprivate func ns_draw_dash(_ s: UnsafeMutableRawPointer?, _ width: Int32, _ segment: Int32,
                              _ offset: Int32, _ thickness: Int32) {
  let y_center = CGFloat(glyph_string_ybase(s) + offset) + CGFloat(thickness) / 2.0
  let path = NSBezierPath()
  let pattern: [CGFloat] = [CGFloat(segment), CGFloat(segment)]

  path.setLineDash(pattern, count: 2, phase: CGFloat(glyph_string_x(s)))
  path.lineWidth = CGFloat(thickness)
  path.move(to: NSMakePoint(CGFloat(glyph_string_x(s)), y_center))
  path.line(to: NSMakePoint(CGFloat(glyph_string_x(s) + width), y_center))
  path.stroke()
}

/// C: `struct glyph`'s `short ascent, descent;` (src/dispextern.h 506, the two reads at
/// C 3771-3772) and the `struct face`/`struct font` members the underline/overline/
/// strike-through block reads: `underline`/`underline_defaulted_p`/`underline_color`/
/// `underline_at_descent_line_p`/`underline_pixels_above_descent_line` (src/dispextern.h
/// 1848-1887), the `overline_*` and `strike_through_*` trios (1823-1883) and `struct font`'s
/// `descent`/`underline_thickness`/`underline_position` (src/font.h 319-332).  Same
/// member-entry shape as the `glyph_string` family above.
@_silgen_name("glyph_ascent") func glyph_ascent(_ g: UnsafeMutableRawPointer?) -> Int16
@_silgen_name("glyph_descent") func glyph_descent(_ g: UnsafeMutableRawPointer?) -> Int16
@_silgen_name("face_underline") func face_underline(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("face_underline_defaulted_p")
func face_underline_defaulted_p(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("face_underline_color")
func face_underline_color(_ f: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("face_underline_at_descent_line_p")
func face_underline_at_descent_line_p(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("face_underline_pixels_above_descent_line")
func face_underline_pixels_above_descent_line(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("face_overline_p") func face_overline_p(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("face_overline_color_defaulted_p")
func face_overline_color_defaulted_p(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("face_overline_color")
func face_overline_color(_ f: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("face_strike_through_p")
func face_strike_through_p(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("face_strike_through_color_defaulted_p")
func face_strike_through_color_defaulted_p(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("face_strike_through_color")
func face_strike_through_color(_ f: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("font_underline_thickness")
func font_underline_thickness(_ font: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("font_underline_position")
func font_underline_position(_ font: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("font_descent") func font_descent(_ font: UnsafeMutableRawPointer?) -> Int32
/// C: `enum face_underline_type` (src/dispextern.h 1767-1780) - the six enumerators the fill
/// switch and the block's comparisons name.  Class A of the 3.255 ruling, like round 16's
/// `text_cursor_kinds` batch: an enumerator's number is the core's fact.
@_silgen_name("FACE_NO_UNDERLINE") func FACE_NO_UNDERLINE() -> Int32
@_silgen_name("FACE_UNDERLINE_SINGLE") func FACE_UNDERLINE_SINGLE() -> Int32
@_silgen_name("FACE_UNDERLINE_DOUBLE_LINE") func FACE_UNDERLINE_DOUBLE_LINE() -> Int32
@_silgen_name("FACE_UNDERLINE_WAVE") func FACE_UNDERLINE_WAVE() -> Int32
@_silgen_name("FACE_UNDERLINE_DOTS") func FACE_UNDERLINE_DOTS() -> Int32
@_silgen_name("FACE_UNDERLINE_DASHES") func FACE_UNDERLINE_DASHES() -> Int32

/// C 3620-3652: `static void ns_fill_underline (struct glyph_string *s, enum face_underline_type
/// style, int position, int thickness)` - "Draw an underline of STYLE onto the focused frame at
/// an offset of POSITION from the baseline of the glyph string S, S->WIDTH in length, and
/// THICKNESS in height."  C's `switch` lists `FACE_NO_UNDERLINE` and `FACE_UNDERLINE_WAVE` with
/// the `default`'s `emacs_abort ()`; Swift has no way to name `default` beside two cases, so
/// the port keeps the two named cases falling through into the `default` - the same statement,
/// and the two enumerators stay entries.
fileprivate func ns_fill_underline(_ s: UnsafeMutableRawPointer?, _ style: Int32,
                                   _ position: Int32, _ thickness: Int32) {
  var segment: Int32 = 0
  var rect = NSRect.zero

  segment = thickness * 3

  switch style {
  /* FACE_UNDERLINE_DOUBLE_LINE is treated identically to SINGLE, as
     the second line will be filled by another invocation of this
     function.  */
  case FACE_UNDERLINE_SINGLE(), FACE_UNDERLINE_DOUBLE_LINE():
    rect = NSMakeRect(CGFloat(glyph_string_x(s)), CGFloat(glyph_string_ybase(s) + position),
                      CGFloat(glyph_string_width(s)), CGFloat(thickness))
    NSRectFill(rect)

  case FACE_UNDERLINE_DOTS():
    segment = thickness
    fallthrough

  case FACE_UNDERLINE_DASHES():
    ns_draw_dash(s, glyph_string_width(s), segment, position, thickness)

  case FACE_NO_UNDERLINE(), FACE_UNDERLINE_WAVE():
    fallthrough

  default:
    emacs_abort()
  }
}

/// C: `struct font *font_for_underline_metrics (struct glyph_string *s)` (src/dispextern.h
/// 3638) - defined at src/xdisp.c 32256, **inside that file's `#ifdef HAVE_WINDOW_SYSTEM`
/// block (opened at 30717)**, so the frozen build has no symbol for it (measured): the row
/// carries that gate, the `get_glyph_string_clip_rects` shape above.
@_silgen_name("font_for_underline_metrics")
func font_for_underline_metrics(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?

/// C: `#define WINDOW_BUFFER_LOCAL_VALUE(V, W) (BUFFERP ((W)->contents) ?
/// buffer_local_value(V, (W)->contents) : Qnil)` (src/window.h 633-636).  Its body needs the
/// `BUFFERP` predicate and the `buffer_local_value` function - both core facts - so it keeps
/// its own name as a class-A entry (the 3.255 A exception, the `WINDOW_FRAME` shape) and the
/// shim exports it; the entry takes the window pointer C's `W` is.
@_silgen_name("WINDOW_BUFFER_LOCAL_VALUE")
func WINDOW_BUFFER_LOCAL_VALUE(_ v: Lisp_Object, _ w: UnsafeMutableRawPointer?) -> Lisp_Object

/// C: `INLINE unsigned long XFIXNAT (Lisp_Object)` (src/lisp.h's fixnum accessor family) -
/// header-only, so class A; `FIXNUMP` was already declared above.
@_silgen_name("XFIXNAT") func XFIXNAT(_ x: Lisp_Object) -> UInt

/// C: the four `Lisp_Object` symbols the C 3702-3718 block reads.  `Qunbound` and
/// `Qunderline_minimum_offset` are **provided** - they are `S` data symbols in
/// `build/emacs-ref/src/emacs.o` (measured this round) - while
/// `Qx_underline_at_descent_line` and `Qx_use_underline_position_properties` are this file's
/// own `DEFSYM`s (C 11748/11770) with no reference-side symbol, i.e. **open** in the shim's
/// sense, the `QCmouse` reading.
@_silgen_name("Qunbound") func Qunbound() -> Lisp_Object
@_silgen_name("Qunderline_minimum_offset") func Qunderline_minimum_offset() -> Lisp_Object
@_silgen_name("Qx_underline_at_descent_line") func Qx_underline_at_descent_line() -> Lisp_Object
@_silgen_name("Qx_use_underline_position_properties")
func Qx_use_underline_position_properties() -> Lisp_Object

/// C 3655-3807: `static void ns_draw_text_decoration (struct glyph_string *s, struct face *face,
/// NSColor *defaultCol, CGFloat width, CGFloat x)` - "Draw underline, overline, and
/// strike-through on glyph string s."  Four spellings are re-shaped and here recorded:
/// the assignment-free `val`-family is C's own, but C's `unsigned long` locals (`thickness`,
/// `position`, `descent`, `minimum_offset`, `h`, `dy`) stay `UInt` and every narrowing back to
/// the `int` parameters is `truncatingIfNeeded:` (C's implicit conversion, the lane's rule);
/// `lround (font->descent / 2)` keeps C's **integer** division before the `lround`; the
/// strike-through's `lrint ((glyph_height - h) / 2)` likewise keeps the unsigned integer
/// division `h` forces; and `[defaultCol set]` reads through the optional receiver C relies on
/// (`defaultCol?.set()`).
fileprivate func ns_draw_text_decoration(_ s: UnsafeMutableRawPointer?,
                                         _ face: UnsafeMutableRawPointer?,
                                         _ defaultCol: NSColor?, _ width: CGFloat, _ x: CGFloat) {
  if glyph_string_for_overlaps(s) != 0 {
    return
  }

  if glyph_string_hl(s) == DRAW_CURSOR() {
    FRAME_BACKGROUND_COLOR(glyph_string_f(s))?.set()
  } else {
    defaultCol?.set()
  }

  /* Do underline.  */
  if face_underline(face) != 0 {
    if face_underline(glyph_string_face(s)) == FACE_UNDERLINE_WAVE() {
      if !face_underline_defaulted_p(face) {
        NSColor.colorWithUnsignedLong(face_underline_color(face)).set()
      }

      ns_draw_underwave(s, width, x)
    } else if face_underline(face) >= FACE_UNDERLINE_SINGLE() {
      var thickness: UInt = 0
      var position: UInt = 0

      /* If the prev was underlined, match its appearance.  */
      if glyph_string_prev(s) != nil
           && (face_underline(glyph_string_face(glyph_string_prev(s))) != FACE_UNDERLINE_WAVE()
               && face_underline(glyph_string_face(glyph_string_prev(s)))
                    >= FACE_UNDERLINE_SINGLE())
           && glyph_string_underline_thickness(glyph_string_prev(s)) > 0
           && (face_underline_at_descent_line_p(glyph_string_face(glyph_string_prev(s)))
               == face_underline_at_descent_line_p(glyph_string_face(s)))
           && (face_underline_pixels_above_descent_line(glyph_string_face(glyph_string_prev(s)))
               == face_underline_pixels_above_descent_line(glyph_string_face(s))) {
        thickness = UInt(truncatingIfNeeded:
                           glyph_string_underline_thickness(glyph_string_prev(s)))
        position = UInt(truncatingIfNeeded:
                          glyph_string_underline_position(glyph_string_prev(s)))
      } else {
        let font = font_for_underline_metrics(s)
        let descent = UInt(truncatingIfNeeded:
                             glyph_string_y(s) + glyph_string_height(s) - glyph_string_ybase(s))
        var minimum_offset: UInt = 0
        var underline_at_descent_line = false
        var use_underline_position_properties = false
        var val = WINDOW_BUFFER_LOCAL_VALUE(Qunderline_minimum_offset(), glyph_string_w(s))

        if FIXNUMP(val) {
          minimum_offset = XFIXNAT(val)
        } else {
          minimum_offset = 1
        }

        val = WINDOW_BUFFER_LOCAL_VALUE(Qx_underline_at_descent_line(), glyph_string_w(s))
        underline_at_descent_line = (!(NILP(val) || EQ(val, Qunbound()))
                                     || face_underline_at_descent_line_p(
                                          glyph_string_face(s)))

        val = WINDOW_BUFFER_LOCAL_VALUE(Qx_use_underline_position_properties(),
                                        glyph_string_w(s))
        use_underline_position_properties = !(NILP(val) || EQ(val, Qunbound()))

        /* Use underline thickness of font, defaulting to 1.  */
        thickness = (font != nil && font_underline_thickness(font) > 0)
          ? UInt(truncatingIfNeeded: font_underline_thickness(font)) : 1

        /* Determine the offset of underlining from the baseline.  */
        if underline_at_descent_line {
          position = (descent - thickness
                      - UInt(truncatingIfNeeded:
                               face_underline_pixels_above_descent_line(glyph_string_face(s))))
        } else if use_underline_position_properties
                    && font != nil && font_underline_position(font) >= 0 {
          position = UInt(truncatingIfNeeded: font_underline_position(font))
        } else if font != nil {
          position = UInt(truncatingIfNeeded: lround(Double(font_descent(font) / 2)))
        } else {
          position = minimum_offset
        }

        if face_underline_pixels_above_descent_line(glyph_string_face(s)) == 0 {
          position = max(position, minimum_offset)
        }

        /* Ensure underlining is not cropped.  */
        if descent <= position {
          position = descent - 1
          thickness = 1
        } else if descent < position + thickness {
          thickness = 1
        }
      }

      glyph_string_set_underline_thickness(s, Int32(truncatingIfNeeded: thickness))
      glyph_string_set_underline_position(s, Int32(truncatingIfNeeded: position))

      if !face_underline_defaulted_p(face) {
        NSColor.colorWithUnsignedLong(face_underline_color(face)).set()
      }

      ns_fill_underline(s, face_underline(glyph_string_face(s)),
                        Int32(truncatingIfNeeded: position),
                        Int32(truncatingIfNeeded: thickness))

      /* Place a second underline above the first if this was
         requested in the face specification.  */

      if face_underline(glyph_string_face(s)) == FACE_UNDERLINE_DOUBLE_LINE() {
        /* Compute the position of the second underline.  */
        position = position - thickness - 1
        ns_fill_underline(s, face_underline(glyph_string_face(s)),
                          Int32(truncatingIfNeeded: position),
                          Int32(truncatingIfNeeded: thickness))
      }
    }
  }
  /* Do overline.  We follow other terms in using a thickness of 1
     and ignoring overline_margin.  */
  if face_overline_p(face) {
    var r = NSRect.zero
    r = NSMakeRect(x, CGFloat(glyph_string_y(s)), width, 1)

    if !face_overline_color_defaulted_p(face) {
      NSColor.colorWithUnsignedLong(face_overline_color(face)).set()
    }

    NSRectFill(r)
  }

  /* Do strike-through.  We follow other terms for thickness and
     vertical position.  */
  if face_strike_through_p(face) {
    var r = NSRect.zero
    /* Y-coordinate and height of the glyph string's first glyph.
       We cannot use s->y and s->height because those could be
       larger if there are taller display elements (e.g., characters
       displayed with a larger font) in the same glyph row.  */
    let glyph_y = glyph_string_ybase(s) - Int32(glyph_ascent(glyph_string_first_glyph(s)))
    let glyph_height = (Int32(glyph_ascent(glyph_string_first_glyph(s)))
                        + Int32(glyph_descent(glyph_string_first_glyph(s))))
    /* Strike-through width and offset from the glyph string's
       top edge.  */
    let h: UInt = 1
    var dy: UInt = 0

    dy = UInt(truncatingIfNeeded:
                lrint(Double((UInt(truncatingIfNeeded: glyph_height) - h) / 2)))
    r = NSMakeRect(x, CGFloat(glyph_y) + CGFloat(dy), width, 1)

    if !face_strike_through_color_defaulted_p(face) {
      NSColor.colorWithUnsignedLong(face_strike_through_color(face)).set()
    }

    NSRectFill(r)
  }
}

// ==========================================================================
// C 3809-4177: the box/relief sub-block and the `dumpglyphs` background helper.
// ==========================================================================

/// C 3809-3839: `static void ns_draw_box (NSRect r, CGFloat hthickness, CGFloat vthickness,
/// NSColor *col, char left_p, char right_p)` - "Draw an unfilled rect inside r, optionally leaving
/// left and/or right open.  Note we can't just use an NSDrawRect command, because of the
/// possibility of some sides not being drawn, and because the rect will be filled."  C's two
/// `char` flags are `Bool` here and the reason is in the reference itself: the identical flag set
/// is spelled `bool raised_p, top_p, bot_p, left_p, right_p` in the sibling `ns_draw_image_relief`
/// (C 4183), and both `char`s are only ever tested with C truthiness or copied.  `NSRect s = r` is
/// C's by-value copy, which `var s = r` is; `[col set]` reads through the optional receiver C
/// relies on (`col?.set ()`), as `ns_draw_text_decoration`'s `defaultCol` does.
fileprivate func ns_draw_box(_ r: NSRect, _ hthickness: CGFloat, _ vthickness: CGFloat,
                             _ col: NSColor?, _ left_p: Bool, _ right_p: Bool) {
  var s = r
  col?.set()

  /* top, bottom */
  s.size.height = hthickness
  NSRectFill(s)
  s.origin.y += r.size.height - hthickness
  NSRectFill(s)

  s.size.height = r.size.height
  s.origin.y = r.origin.y

  /* left, right (optional) */
  s.size.width = vthickness
  if left_p {
    NSRectFill(s)
  }
  if right_p {
    s.origin.x += r.size.width - vthickness
    NSRectFill(s)
  }
}

/// C: `struct face`'s box members - `enum face_box_type box` (a `: 2` bitfield, src/dispextern.h
/// 1858, so its entry keeps C's raw value, the `for_overlaps` reading), `unsigned long box_color`
/// (1826), `int box_vertical_line_width` / `int box_horizontal_line_width` (1844-1845),
/// `bool_bf use_box_color_for_shadows_p : 1` (1865) and `ptrdiff_t stipple` (1810, the array index
/// `ns_maybe_dumpglyphs_background` subtracts one from) - plus the two `enum face_box_type`
/// enumerators C 4078-4087 tests (`FACE_SIMPLE_BOX`, `FACE_RAISED_BOX`, src/dispextern.h 1757/1761).
/// The record belongs to the core (`femacs/src/dispextern.rs`), so each read is a member entry
/// under `face_<member>`, the family round 17's eleven already use.
@_silgen_name("face_box") func face_box(_ face: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("face_box_color") func face_box_color(_ face: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("face_box_vertical_line_width")
func face_box_vertical_line_width(_ face: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("face_box_horizontal_line_width")
func face_box_horizontal_line_width(_ face: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("face_use_box_color_for_shadows_p")
func face_use_box_color_for_shadows_p(_ face: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("face_stipple") func face_stipple(_ face: UnsafeMutableRawPointer?) -> Int
@_silgen_name("FACE_SIMPLE_BOX") func FACE_SIMPLE_BOX() -> Int32
@_silgen_name("FACE_RAISED_BOX") func FACE_RAISED_BOX() -> Int32

/// C: `struct ns_output`'s three relief colors - src/nsterm.h 943-945's `relief_background_color`,
/// `light_relief_color` and `dark_relief_color`; the frozen arm's pointer spellings are 953-955 -
/// read and written by `ns_setup_relief_colors` (C 3861-3867) and read by
/// `ns_draw_relief` (C 3912, 3934, 3996).  The family takes the *frame* C's `FRAME_OUTPUT_DATA`
/// starts from (`ns_output_cursor_color`'s shape), and the writes' C text is `x = [y retain];` -
/// the reading `ns_output_set_cursor_color`'s own row records (`FRAME_CURSOR_COLOR (f) =
/// [col retain];`, nsfns.m 358), so the `NSColor?` setter is the `retain` half and the port's
/// `release` half is the runtime's (`Unmanaged.passUnretained (…).release ()`, the
/// `ns_free_frame_resources` reading).
@_silgen_name("ns_output_relief_background_color")
func ns_output_relief_background_color(_ f: UnsafeMutableRawPointer?) -> NSColor?
@_silgen_name("ns_output_set_relief_background_color")
func ns_output_set_relief_background_color(_ f: UnsafeMutableRawPointer?, _ v: NSColor?)
@_silgen_name("ns_output_light_relief_color")
func ns_output_light_relief_color(_ f: UnsafeMutableRawPointer?) -> NSColor?
@_silgen_name("ns_output_set_light_relief_color")
func ns_output_set_light_relief_color(_ f: UnsafeMutableRawPointer?, _ v: NSColor?)
@_silgen_name("ns_output_dark_relief_color")
func ns_output_dark_relief_color(_ f: UnsafeMutableRawPointer?) -> NSColor?
@_silgen_name("ns_output_set_dark_relief_color")
func ns_output_set_dark_relief_color(_ f: UnsafeMutableRawPointer?, _ v: NSColor?)

/// C 3841-3869: `static void ns_setup_relief_colors (struct glyph_string *s)` - "Set up colors for
/// the relief lines around glyph string S."  Four spellings are re-shaped and here recorded: C's
/// `[NSColor grayColor]` is Swift's `NSColor.gray` and `highlightWithLevel:`/`shadowWithLevel:` are
/// `highlight (withLevel :)`/`shadow (withLevel :)` (AppKit's own renames - the two selectors have
/// no `AppKit.apinotes` entry, so the importer's default rule applies); the `color == nil` test and
/// the `color != di->relief_background_color` comparison are Swift's (`color == nil`, `!==` on the
/// two optionals - C compares the pointers); and the three `[old release]; field = [new retain];`
/// pairs keep both halves (see the `ns_output_*_relief_color` block above), with
/// `[color highlightWithLevel: 0.4]`'s nil-receiver reading as `color.flatMap { … }` - C's message
/// to a nil receiver yields nil.  `FRAME_OUTPUT_DATA (s->f)`'s local `di` is not carried: the
/// member entries take the frame, the reading round 11's mouse section uses.
fileprivate func ns_setup_relief_colors(_ s: UnsafeMutableRawPointer?) {
  var color: NSColor?

  if face_use_box_color_for_shadows_p(glyph_string_face(s)) {
    color = NSColor.colorWithUnsignedLong(face_box_color(glyph_string_face(s)))
  } else {
    color = NSColor.colorWithUnsignedLong(face_background(glyph_string_face(s)))
  }

  if glyph_string_hl(s) == DRAW_CURSOR() {
    color = FRAME_CURSOR_COLOR(glyph_string_f(s))
  }

  if color == nil {
    color = NSColor.gray
  }

  if color !== ns_output_relief_background_color(glyph_string_f(s)) {
    if let old = ns_output_relief_background_color(glyph_string_f(s)) {
      Unmanaged.passUnretained(old).release()
    }
    ns_output_set_relief_background_color(glyph_string_f(s), color)

    if let old = ns_output_light_relief_color(glyph_string_f(s)) {
      Unmanaged.passUnretained(old).release()
    }
    ns_output_set_light_relief_color(glyph_string_f(s),
                                     color.flatMap { $0.highlight(withLevel: 0.4) })

    if let old = ns_output_dark_relief_color(glyph_string_f(s)) {
      Unmanaged.passUnretained(old).release()
    }
    ns_output_set_dark_relief_color(glyph_string_f(s),
                                    color.flatMap { $0.shadow(withLevel: 0.4) })
  }
}

/// C 3871-4023: `static void ns_draw_relief (NSRect outer, int hthickness, int vthickness, char
/// raised_p, char top_p, char bottom_p, char left_p, char right_p, struct glyph_string *s)` -
/// "Draw a relief rect inside r, optionally leaving some sides open.  Note we can't just use an
/// NSDrawBezel command, because of the possibility of some sides not being drawn, and because the
/// rect will be filled."  The five `char` flags are `Bool` (the `ns_draw_box` reading above), and
/// C's `1, 1` literals at the one call site become `true, true`.  The two `NSBezierPath *p = nil`
/// blocks are `NSBezierPath ()` under ARC (the reading round 15 records for `fringe_bmp`'s path),
/// `[p closePath]` is `close ()`, `[p fill]`/`[p stroke]` are `fill ()`/`stroke ()`,
/// `[p removeAllPoints]` is `removeAllPoints ()` and `[NSBezierPath fillRect: …]` is the class
/// method `NSBezierPath.fill (_:)` - the rename the already-landed `NSBezierPath.stroke (_:)`
/// (C 3470) proves for the pair.  `inner`/`p` are C's uninitialized locals
/// (`var inner = NSRect.zero`, `var p: NSBezierPath? = nil`), and the three reads of
/// `di->light_relief_color`/`di->dark_relief_color` go through the member entries with the frame
/// C's `di` was `FRAME_OUTPUT_DATA` of.  **One transcription note**: the reference's own
/// `vthickness > 1 && left_p` block appears **twice** (C 3980-3986 and C 3988-3994 - the second
/// is what a `right_p` test would mean); the port keeps both, verbatim, as the reference's text.
fileprivate func ns_draw_relief(_ outer: NSRect, _ hthickness: Int32, _ vthickness: Int32,
                                _ raised_p: Bool, _ top_p: Bool, _ bottom_p: Bool,
                                _ left_p: Bool, _ right_p: Bool,
                                _ s: UnsafeMutableRawPointer?) {
  var inner = NSRect.zero
  var p: NSBezierPath? = nil

  // C 3884: `NSTRACE ("ns_draw_relief");` - empty at `NSTRACE_ENABLED` 0, like the rest.

  /* set up colors */
  ns_setup_relief_colors(s)

  /* Calculate the inner rectangle.  */
  inner = outer

  if left_p {
    inner.origin.x += CGFloat(vthickness)
    inner.size.width -= CGFloat(vthickness)
  }

  if right_p {
    inner.size.width -= CGFloat(vthickness)
  }

  if top_p {
    inner.origin.y += CGFloat(hthickness)
    inner.size.height -= CGFloat(hthickness)
  }

  if bottom_p {
    inner.size.height -= CGFloat(hthickness)
  }

  // C 3910: `struct ns_output *di = FRAME_OUTPUT_DATA (s->f);` - the entries below take the frame
  // C's macro starts from.

  (raised_p ? ns_output_light_relief_color(glyph_string_f(s))
            : ns_output_dark_relief_color(glyph_string_f(s)))?.set()

  if top_p || left_p {
    p = NSBezierPath()

    p!.move(to: NSMakePoint(NSMinX(outer), NSMinY(outer)))
    if top_p {
      p!.line(to: NSMakePoint(NSMaxX(outer), NSMinY(outer)))
      p!.line(to: NSMakePoint(NSMaxX(inner), NSMinY(inner)))
    }
    p!.line(to: NSMakePoint(NSMinX(inner), NSMinY(inner)))
    if left_p {
      p!.line(to: NSMakePoint(NSMinX(inner), NSMaxY(inner)))
      p!.line(to: NSMakePoint(NSMinX(outer), NSMaxY(outer)))
    }
    p!.close()
    p!.fill()
  }

  (raised_p ? ns_output_dark_relief_color(glyph_string_f(s))
            : ns_output_light_relief_color(glyph_string_f(s)))?.set()

  if bottom_p || right_p {
    p = NSBezierPath()

    p!.move(to: NSMakePoint(NSMaxX(outer), NSMaxY(outer)))
    if right_p {
      p!.line(to: NSMakePoint(NSMaxX(outer), NSMinY(outer)))
      p!.line(to: NSMakePoint(NSMaxX(inner), NSMinY(inner)))
    }
    p!.line(to: NSMakePoint(NSMaxX(inner), NSMaxY(inner)))
    if bottom_p {
      p!.line(to: NSMakePoint(NSMinX(inner), NSMaxY(inner)))
      p!.line(to: NSMakePoint(NSMinX(outer), NSMaxY(outer)))
    }
    p!.close()
    p!.fill()
  }

  /* If one of h/vthickness are more than 1, draw the outermost line
     on the respective sides in the black relief color.  */

  if p != nil {
    p!.removeAllPoints()
  } else {
    p = NSBezierPath()
  }

  if hthickness > 1 && top_p {
    p!.move(to: NSMakePoint(NSMinX(outer), NSMinY(outer) + 0.5))
    p!.line(to: NSMakePoint(NSMaxX(outer), NSMinY(outer) + 0.5))
  }

  if hthickness > 1 && bottom_p {
    p!.move(to: NSMakePoint(NSMinX(outer), NSMaxY(outer) - 0.5))
    p!.line(to: NSMakePoint(NSMaxX(outer), NSMaxY(outer) - 0.5))
  }

  if vthickness > 1 && left_p {
    p!.move(to: NSMakePoint(NSMinX(outer) + 0.5, NSMinY(outer) + 0.5))
    p!.line(to: NSMakePoint(NSMinX(outer) + 0.5, NSMaxY(outer) - 0.5))
  }

  if vthickness > 1 && left_p {
    p!.move(to: NSMakePoint(NSMinX(outer) + 0.5, NSMinY(outer) + 0.5))
    p!.line(to: NSMakePoint(NSMinX(outer) + 0.5, NSMaxY(outer) - 0.5))
  }

  ns_output_dark_relief_color(glyph_string_f(s))?.set()
  p!.stroke()

  if vthickness > 1 && hthickness > 1 {
    FRAME_BACKGROUND_COLOR(glyph_string_f(s))?.set()

    if left_p && top_p {
      NSBezierPath.fill(NSMakeRect(NSMinX(outer), NSMinY(outer), 1, 1))
    }

    if right_p && top_p {
      NSBezierPath.fill(NSMakeRect(NSMaxX(outer) - 1, NSMinY(outer), 1, 1))
    }

    if right_p && bottom_p {
      NSBezierPath.fill(NSMakeRect(NSMaxX(outer) - 1, NSMaxY(outer) - 1, 1, 1))
    }

    if left_p && bottom_p {
      NSBezierPath.fill(NSMakeRect(NSMinX(outer), NSMaxY(outer) - 1, 1, 1))
    }
  }
}

/// C: `struct glyph_string`'s members this slice adds to the family round 17 started -
/// `struct glyph_row *row` (src/dispextern.h 1399, read at C 4045/4053/4064), `enum glyph_row_area
/// area` (1402, read at 4047/4053), `struct image *img` (1483, the `s->img` test at 4048),
/// `struct glyph_string *next` (1506, the `s->next` read at 4072), `int background_width`
/// (1374, 4065/4115/4171), `bool_bf extends_to_end_of_line_p : 1` (1431, 4064/4153),
/// `bool_bf background_filled_p : 1` (1434, the read at 4103 and the **writes** at 4118/4174),
/// `bool_bf font_not_found_p : 1` (1439, 4152) and `bool_bf stippled_p : 1` (1443, 4107).  The
/// record belongs to the core (`femacs/src/dispextern.rs`), so each is a member entry under
/// `<record>_<member>` with `_set_` for the write; `prev` (round 17's) completes the pair.
@_silgen_name("glyph_string_row")
func glyph_string_row(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_area") func glyph_string_area(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_img")
func glyph_string_img(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_next")
func glyph_string_next(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_background_width")
func glyph_string_background_width(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_extends_to_end_of_line_p")
func glyph_string_extends_to_end_of_line_p(_ s: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_string_background_filled_p")
func glyph_string_background_filled_p(_ s: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_string_set_background_filled_p")
func glyph_string_set_background_filled_p(_ s: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("glyph_string_font_not_found_p")
func glyph_string_font_not_found_p(_ s: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_string_stippled_p")
func glyph_string_stippled_p(_ s: UnsafeMutableRawPointer?) -> Bool

/// C: `struct glyph`'s members the composite walk reads - `u.cmp.automatic` (the `bool_bf
/// automatic : 1` of the union `u`'s `cmp` sub-struct, src/dispextern.h 599, read at C 4051/4056),
/// `u.cmp.id` (`unsigned id : 31`, 601, read at 4056) and the other union's `slice.cmp.to`
/// (`struct { int from, to; } cmp;`, 581, read at 4057) - plus the two box-line flags
/// `left_box_line_p`/`right_box_line_p` (`bool_bf … : 1`, 524-525, read at 4067/4070).  Names keep
/// C's member path (the `glyph_type` family's spelling); `u.cmp.id` is a 31-bit field, so its entry
/// is `Int32` and the comparison against the `ptrdiff_t` `s->cmp_id` widens it (`Int (…)`), which
/// is C's own conversion (the field's `unsigned` promotes to the long).
@_silgen_name("glyph_u_cmp_automatic")
func glyph_u_cmp_automatic(_ g: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_u_cmp_id") func glyph_u_cmp_id(_ g: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_slice_cmp_to") func glyph_slice_cmp_to(_ g: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_left_box_line_p")
func glyph_left_box_line_p(_ g: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_right_box_line_p")
func glyph_right_box_line_p(_ g: UnsafeMutableRawPointer?) -> Bool

/// C: `struct glyph_row`'s `struct glyph *glyphs[1 + LAST_AREA]` (src/dispextern.h 934, read at
/// C 4053 with C's own `s->area` subscript, which is the entry's second argument the way
/// `glyph_row_used`'s `TEXT_AREA` is) - the base of the composite walk.
@_silgen_name("glyph_row_glyphs")
func glyph_row_glyphs(_ row: UnsafeMutableRawPointer?, _ area: Int32) -> UnsafeMutableRawPointer?

/// C: `struct glyph *` arithmetic - the reference's `s->first_glyph + s->nchars - 1` (C 4062), the
/// walk's `… .glyphs[s->area] + s->row->used[s->area]` (C 4053), `g++` (C 4055/4058) and the
/// `g < end` comparison (C 4056).  `struct glyph` is the core's record and this unit has no Swift
/// view of it, so the **stride** is the core's fact and the advance is an entry of its own - the
/// family's first *operation* row, next to round 14's member rows; `glyph_advance (g, n)` is C's
/// `g + n` and returns the same pointer type C's expression has.
@_silgen_name("glyph_advance")
func glyph_advance(_ g: UnsafeMutableRawPointer?, _ n: Int32) -> UnsafeMutableRawPointer?

/// C: `#define WINDOW_RIGHT_EDGE_X(W) (FRAME_INTERNAL_BORDER_WIDTH (WINDOW_XFRAME (W)) +
/// WINDOW_RIGHT_PIXEL_EDGE (W))` (src/window.h 764-766), read at C 4046, and
/// `int window_box_right (struct window *, enum glyph_row_area)` (src/dispextern.h 3581) - defined
/// at src/xdisp.c 1450's `window_box_right`, **before** that file's HAVE_WINDOW_SYSTEM block
/// (opened at 500), so it is a real symbol of the frozen build (measured `T` in
/// `build/emacs-ref/src/xdisp.o`); C 4047 reads it.  The macro keeps its own name as a class-A
/// entry (`WINDOW_FRAME`'s shape); the function is the core's provider, since xdisp.c is a ported
/// unit.
@_silgen_name("WINDOW_RIGHT_EDGE_X")
func WINDOW_RIGHT_EDGE_X(_ w: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("window_box_right")
func window_box_right(_ w: UnsafeMutableRawPointer?, _ area: Int32) -> Int32
/// C: src/dispextern.h 1353's `DRAW_MOUSE_FACE` - the third enumerator of `enum draw_glyphs_face`
/// (src/dispextern.h 1348-1353), which `DRAW_CURSOR` 1352 precedes - the face-override the two
/// box-line tests compare against.
@_silgen_name("DRAW_MOUSE_FACE") func DRAW_MOUSE_FACE() -> Int32

/// C 4026-4089: `static void ns_dumpglyphs_box_or_relief (struct glyph_string *s)` - "Function
/// modeled after x_draw_glyph_string_box ().  Sets up parameters for drawing."  Three readings:
/// C's `char` locals are `Bool` and its `int`s stay `Int32` (`last_glyph` is the untyped
/// `struct glyph *`), the `s->cmp || s->img` test and the two `DRAW_MOUSE_FACE` dereferences read
/// through the member
/// entries above, and **the composite walk keeps C's pointer shape** through `glyph_advance` with
/// the nil guards Swift needs for `g < end` (C's comparison against a NULL `end` is false for the
/// non-NULL `g` the loop holds, which is what the guards reproduce) - a `g` that walks off a NULL
/// base is not a case C survives either.  `abs (hthickness)` keeps C's `int` `abs` and the box
/// call's two `CGFloat` parameters (C's implicit conversion), while the relief call's are the
/// `int`s C declares.
fileprivate func ns_dumpglyphs_box_or_relief(_ s: UnsafeMutableRawPointer?) {
  var right_x: Int32 = 0, last_x: Int32 = 0
  var left_p = false, right_p = false
  var last_glyph: UnsafeMutableRawPointer?
  var r = NSRect.zero
  var hthickness: Int32 = 0, vthickness: Int32 = 0
  let face = glyph_string_face(s)

  vthickness = face_box_vertical_line_width(face)
  hthickness = face_box_horizontal_line_width(face)

  // C 4043: `NSTRACE ("ns_dumpglyphs_box_or_relief");` - empty at `NSTRACE_ENABLED` 0.

  last_x = (glyph_row_full_width_p(glyph_string_row(s))
            && !window_pseudo_window_p(glyph_string_w(s))
            ? WINDOW_RIGHT_EDGE_X(glyph_string_w(s))
            : window_box_right(glyph_string_w(s), glyph_string_area(s)))
  if glyph_string_cmp(s) != nil || glyph_string_img(s) != nil {
    last_glyph = glyph_string_first_glyph(s)
  } else if glyph_type(glyph_string_first_glyph(s)) == COMPOSITE_GLYPH()
              && glyph_u_cmp_automatic(glyph_string_first_glyph(s)) {
    let end = glyph_advance(glyph_row_glyphs(glyph_string_row(s), glyph_string_area(s)),
                            Int32(glyph_row_used(glyph_string_row(s), glyph_string_area(s))))
    var g = glyph_string_first_glyph(s)
    /* C 4055-4059: `for (last_glyph = g++; cond; last_glyph = g++) ;` - the empty body's
       increment is this loop's tail.  */
    last_glyph = g
    g = glyph_advance(g, 1)
    while g != nil && end != nil && g! < end!
            && glyph_u_cmp_automatic(g)
            && Int(glyph_u_cmp_id(g)) == glyph_string_cmp_id(s)
            && glyph_slice_cmp_to(g) < glyph_string_cmp_to(s) {
      last_glyph = g
      g = glyph_advance(g, 1)
    }
  } else {
    last_glyph = glyph_advance(glyph_string_first_glyph(s), glyph_string_nchars(s) - 1)
  }

  right_x = (glyph_row_full_width_p(glyph_string_row(s))
             && glyph_string_extends_to_end_of_line_p(s)
             ? last_x - 1
             : min(last_x, glyph_string_x(s) + glyph_string_background_width(s)) - 1)

  left_p = (glyph_left_box_line_p(glyph_string_first_glyph(s))
            || (glyph_string_hl(s) == DRAW_MOUSE_FACE()
                && (glyph_string_prev(s) == nil
                    || glyph_string_hl(glyph_string_prev(s)) != glyph_string_hl(s))))
  right_p = (glyph_right_box_line_p(last_glyph)
             || (glyph_string_hl(s) == DRAW_MOUSE_FACE()
                 && (glyph_string_next(s) == nil
                     || glyph_string_hl(glyph_string_next(s)) != glyph_string_hl(s))))

  r = NSMakeRect(CGFloat(glyph_string_x(s)), CGFloat(glyph_string_y(s)),
                 CGFloat(right_x - glyph_string_x(s) + 1), CGFloat(glyph_string_height(s)))

  /* TODO: Sometimes box_color is 0 and this seems wrong; should investigate.  */
  if face_box(face) == FACE_SIMPLE_BOX() && face_box_color(face) != 0 {
    ns_draw_box(r, CGFloat(abs(hthickness)), CGFloat(abs(vthickness)),
                NSColor.colorWithUnsignedLong(face_box_color(face)), left_p, right_p)
  } else {
    ns_draw_relief(r, abs(hthickness), abs(vthickness),
                   face_box(face) == FACE_RAISED_BOX(), true, true, left_p, right_p, s)
  }
}

/// C: `#define NS_FACE_FOREGROUND(f) ((f)->foreground)` (src/nsterm.h 1040) - the write half of the
/// `NS_FACE_BACKGROUND` pair round 13 imports (src/nsterm.h 1041); C 4166 reads it.
@_silgen_name("NS_FACE_FOREGROUND")
func NS_FACE_FOREGROUND(_ face: UnsafeMutableRawPointer?) -> UInt
/// C: `#define FONT_HEIGHT(f) ((f)->height)` (src/font.h 244) and `#define FONT_TOO_HIGH(ft)
/// ((ft)->pixel_size > 0 && (ft)->ascent + (ft)->descent > 3*(ft)->pixel_size)`
/// (src/dispextern.h 1712-1713) - the two font predicates of C 4157-4163.  Both bodies need
/// `struct font` members the core owns, so each keeps its own name as a class-A entry (the 3.255 A
/// criterion), the `FONT_HEIGHT` reading of the reference's own `FONT_TOO_HIGH` comment.
@_silgen_name("FONT_HEIGHT") func FONT_HEIGHT(_ font: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FONT_TOO_HIGH") func FONT_TOO_HIGH(_ font: UnsafeMutableRawPointer?) -> Bool

/// C: the five CoreGraphics calls of the stipple arm (C 4130-4137) - `void CGContextClipToRect
/// (CGContextRef, CGRect)`, `void CGContextScaleCTM (CGContextRef, CGFloat, CGFloat)`,
/// `void CGContextDrawTiledImage (CGContextRef, CGRect, CGImageRef)`, `size_t CGImageGetWidth
/// (CGImageRef)` and `size_t CGImageGetHeight (CGImageRef)`.  Each keeps its C name as an
/// `@_silgen_name` entry (the round-5 `CGMainDisplayID`/`CGDisplayBounds` shape) because the API
/// notes rename all five in Swift (`CoreGraphics.apinotes` 194-195, 236-237, 361-364 and 508-511
/// map them to `CGContext.scaleBy (x:y:)`, `CGContext.clip (to:)`,
/// `CGContext.__draw (_:in:byTiling:)` and the `CGImage.width`/`CGImage.height` getters), and each
/// entry's provider is the SDK's own symbol - the `CGImageMaskCreate` reading `nsimage.swift` 159
/// records.
@_silgen_name("CGContextClipToRect")
func CGContextClipToRect(_ c: CGContext?, _ rect: CGRect)
@_silgen_name("CGContextScaleCTM")
func CGContextScaleCTM(_ c: CGContext?, _ sx: CGFloat, _ sy: CGFloat)
@_silgen_name("CGContextDrawTiledImage")
func CGContextDrawTiledImage(_ c: CGContext?, _ rect: CGRect, _ image: CGImage?)
@_silgen_name("CGImageGetWidth") func CGImageGetWidth(_ image: CGImage?) -> Int
@_silgen_name("CGImageGetHeight") func CGImageGetHeight(_ image: CGImage?) -> Int

/// C: `struct ns_display_info`'s `struct ns_bitmap_record *bitmaps` (src/nsterm.h 873; the record
/// itself is 810-819) with C's own subscript `face->stipple - 1` as the second argument (the
/// `glyph_row_used` shape), reaching that record's `EmacsImage *img` (src/nsterm.h 813 in the
/// `__OBJC__` arm; the frozen arm's `void *img` is 816) - C 4119-4120.
/// `EmacsImage` is another lane's class (`nsimage.swift`'s, the `@interface` at src/nsterm.h
/// 670-703), so its `- (CGImageRef)stippleMask` (declared src/nsterm.h 696, defined src/nsimage.m
/// 542) crosses as a C-ABI entry under `<Class>_<selector>` - the `EmacsImage_allocInitFromFile`
/// shape `nsfns.swift` 402 declares.  **No provider exists yet**: the ledger's (3) registers it as
/// owed, the P-55 reading.
@_silgen_name("ns_display_info_bitmaps_img")
func ns_display_info_bitmaps_img(_ dpyinfo: UnsafeMutableRawPointer?,
                                 _ i: Int) -> AnyObject?
@_silgen_name("EmacsImage_stippleMask")
func EmacsImage_stippleMask(_ img: AnyObject?) -> CGImage?

/// C 4091-4177: `static void ns_maybe_dumpglyphs_background (struct glyph_string *s, char force_p)` -
/// "Modeled after x_draw_glyph_string_background, which draws BG in certain cases.  Others are left
/// to the text rendering routine."  `char force_p` is `Bool` (the `ns_draw_box` reading), and the
/// `NS_IMPL_COCOA` stipple arm is **the live one**: the `#else` arm's
/// `[[dpyinfo->bitmaps[…].img stippleMask] set]; goto fill;` and the `fill:` label the `!NS_IMPL_COCOA`
/// guard carries at C 4167-4169 are the GNUstep text and are recorded, not transcribed (the `goto`
/// has no Swift form and no live reader).  Four more readings: `FRAME_DISPLAY_INFO (s->f)`'s local
/// `dpyinfo` is not carried (the entry takes the frame, as in `ns_setup_relief_colors`); the colour
/// ternary and the `else if`'s comparison keep C's statement shape with an explicit `NSColor?`
/// binding, whose `?.set ()` is C's message to a possibly-nil receiver; `[nil unsignedLong]`'s
/// zero is `?? 0` (the category method above is this unit's own); and `CGRectMake (0, 0, …)`
/// - a `static inline` of CGGeometry.h with no symbol - is Swift's `CGRect (x:y:width:height:)`,
/// a class-B expansion, with C's two `size_t` arguments converted to `CGFloat`.
fileprivate func ns_maybe_dumpglyphs_background(_ s: UnsafeMutableRawPointer?,
                                               _ force_p: Bool) {
  let face = glyph_string_face(s)
  var r = NSRect.zero

  // C 4101: `NSTRACE ("ns_maybe_dumpglyphs_background");` - empty at `NSTRACE_ENABLED` 0.

  if !glyph_string_background_filled_p(s) {
    let box_line_width = max(face_box_horizontal_line_width(face), 0)

    if glyph_string_stippled_p(s) {
      /* On cocoa emacs the stipple is stored as a mask CGImage.
         First we want to clear the background with the bg color.  */
      NSColor.colorWithUnsignedLong(face_background(face)).set()
      r = NSMakeRect(CGFloat(glyph_string_x(s)), CGFloat(glyph_string_y(s) + box_line_width),
                     CGFloat(glyph_string_background_width(s)),
                     CGFloat(glyph_string_height(s) - 2 * box_line_width))
      NSRectFill(r)
      glyph_string_set_background_filled_p(s, true)
      let mask = EmacsImage_stippleMask(
        ns_display_info_bitmaps_img(FRAME_DISPLAY_INFO(glyph_string_f(s)),
                                    face_stipple(face) - 1))

      /* This part could possibly be improved, the author is
         unfamiliar with NS/CoreGraphics and isn't sure if it's
         possible to do this with NSImage */
      let ctx = NSGraphicsContext.current
      ctx?.saveGraphicsState()
      /* Checkpoint the graphics state and then focus in on the area
         we're going to fill */
      let context = ctx?.cgContext
      CGContextClipToRect(context, r)
      CGContextScaleCTM(context, 1, -1)

      /* Stamp the foreground color using the stipple mask */
      NSColor.colorWithUnsignedLong(face_foreground(face)).set()
      let imageSize = CGRect(x: 0, y: 0, width: CGFloat(CGImageGetWidth(mask)),
                             height: CGFloat(CGImageGetHeight(mask)))
      CGContextDrawTiledImage(context, imageSize, mask)

      NSGraphicsContext.current?.restoreGraphicsState()

    } else if FONT_HEIGHT(glyph_string_font(s)) < glyph_string_height(s) - 2 * box_line_width
                /* When xdisp.c ignores FONT_HEIGHT, we cannot trust font
                   dimensions, since the actual glyphs might be much
                   smaller.  So in that case we always clear the
                   rectangle with background color.  */
                || FONT_TOO_HIGH(glyph_string_font(s))
                || glyph_string_font_not_found_p(s)
                || glyph_string_extends_to_end_of_line_p(s)
                || force_p {
      if glyph_string_hl(s) != DRAW_CURSOR() {
        let bg: NSColor? = NS_FACE_BACKGROUND(face) != 0
          ? NSColor.colorWithUnsignedLong(NS_FACE_BACKGROUND(face))
          : FRAME_BACKGROUND_COLOR(glyph_string_f(s))
        bg?.set()
      } else if face != nil
                  && (NS_FACE_BACKGROUND(face)
                      == (FRAME_CURSOR_COLOR(glyph_string_f(s))?.unsignedLong() ?? 0)) {
        NSColor.colorWithUnsignedLong(NS_FACE_FOREGROUND(face)).set()
      } else {
        FRAME_CURSOR_COLOR(glyph_string_f(s))?.set()
      }

      r = NSMakeRect(CGFloat(glyph_string_x(s)), CGFloat(glyph_string_y(s) + box_line_width),
                     CGFloat(glyph_string_background_width(s)),
                     CGFloat(glyph_string_height(s) - 2 * box_line_width))
      NSRectFill(r)
      glyph_string_set_background_filled_p(s, true)
    }
  }
}

// ==========================================================================
// C 4179-4393: the image sub-block - the relief drawn around an image, and the
// image glyph itself with its background, transform and cursor box.
// ==========================================================================

/// C: the `struct image` members this slice reads.
///
/// `Emacs_Pixmap pixmap, mask;` is src/dispextern.h 3222
/// (`typedef id Emacs_Pixmap;`, the NS arm of src/nsgui.h 55-59, so both come
/// back untyped - `AnyObject?`, the `ns_display_info_bitmaps_img` reading of
/// round 18).  `int width, height;` is src/dispextern.h 3304, `int relief;` is
/// src/dispextern.h 3339 and `int hmargin, vmargin;` is src/dispextern.h 3342.
///
/// The record is the core's (`femacs/src/include/dispextern.rs` 2147) and the
/// whole of it sits inside that header's `HAVE_WINDOW_SYSTEM` block, so every
/// read is a member entry under `image_<member>`, the `face_<member>` family's
/// shape.
@_silgen_name("image_pixmap") func image_pixmap(_ img: UnsafeMutableRawPointer?) -> AnyObject?
@_silgen_name("image_mask") func image_mask(_ img: UnsafeMutableRawPointer?) -> AnyObject?
@_silgen_name("image_width") func image_width(_ img: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("image_height") func image_height(_ img: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("image_relief") func image_relief(_ img: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("image_hmargin") func image_hmargin(_ img: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("image_vmargin") func image_vmargin(_ img: UnsafeMutableRawPointer?) -> Int32

/// C: `struct glyph_slice`'s four `unsigned : 16` bit members, read as
/// `s->slice.x` and its siblings at C 4192, 4197, 4199-4200, 4221-4222,
/// 4253-4260 and 4283, 4287, 4292-4293.
///
/// The record is src/dispextern.h 455-460
/// (`unsigned x` is its first member, 457), and its one use here is the
/// `struct glyph_slice slice;` member at src/dispextern.h 1489.
/// It belongs to the core and has no Swift view in this lane, so the four reads
/// are member entries under `glyph_string_slice_<member>`; C's expressions
/// promote each 16-bit field to `int`, so the entries keep the promoted `Int32`
/// value - round 18's `glyph_u_cmp_id` reading.
@_silgen_name("glyph_string_slice_x")
func glyph_string_slice_x(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_slice_y")
func glyph_string_slice_y(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_slice_width")
func glyph_string_slice_width(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_slice_height")
func glyph_string_slice_height(_ s: UnsafeMutableRawPointer?) -> Int32

/// C: `&s->slice` (C 4186 and 4274) - the *address* of the slice member, which
/// is what `image_ascent`'s third parameter wants.
///
/// It is the family's second *operation* row, next to round 18's
/// `glyph_advance`, and for the same reason: the record's layout is the core's
/// fact, so a pointer into it cannot be spelled at the use site.
@_silgen_name("glyph_string_slice_ptr")
func glyph_string_slice_ptr(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?

/// C: `int image_ascent (struct image *, struct face *, struct glyph_slice *)`
/// - declared at src/dispextern.h 3774
/// (inside the header's `HAVE_WINDOW_SYSTEM` block, opened at src/dispextern.h 3727)
/// and defined at src/image.c 1911-1912.
///
/// src/image.c is one of the files the frozen configuration does not compile
/// (`docs/baseline/frozen-build-sources.txt`; its port is an empty unit,
/// `docs/ports/image.md`), so no object holds the symbol and the shim lane owes
/// the export - the gate files the name (Evidence).  C 4186 and C 4274 are its
/// two call sites here.
@_silgen_name("image_ascent")
func image_ascent(_ img: UnsafeMutableRawPointer?, _ face: UnsafeMutableRawPointer?,
                  _ slice: UnsafeMutableRawPointer?) -> Int32

/// C: `struct face`'s `int id;` at src/dispextern.h 1795
/// - the face's index in its cache, read at C 4205, 4225 and 4238.  It sits
/// *above* the record's own `HAVE_WINDOW_SYSTEM` block, so the member is
/// ungated; the entry is the member family's shape, like round 18's
/// `face_box_*`.
@_silgen_name("face_id") func face_id(_ face: UnsafeMutableRawPointer?) -> Int32

/// C: the three enumerators the box and bar tests name.
///
/// `FACE_NO_BOX` is `enum face_box_type`'s first member (the
/// `s->face->box != FACE_NO_BOX` test at C 4190 and 4282), declared at
/// src/dispextern.h 1753
/// - Class A of the 3.255 ruling, like round 18's
/// `FACE_SIMPLE_BOX`/`FACE_RAISED_BOX`, because an enumerator's number is the
/// core's fact.  `TOOL_BAR_FACE_ID` and `TAB_BAR_FACE_ID` are the two
/// `enum face_id` members the bar branches compare against at C 4205, 4225 and
/// 4238, declared at src/dispextern.h 1964 and src/dispextern.h 1979
@_silgen_name("FACE_NO_BOX") func FACE_NO_BOX() -> Int32
@_silgen_name("TOOL_BAR_FACE_ID") func TOOL_BAR_FACE_ID() -> Int32
@_silgen_name("TAB_BAR_FACE_ID") func TAB_BAR_FACE_ID() -> Int32

/// C: `enum draw_glyphs_face`'s last two enumerators, `DRAW_IMAGE_RAISED` at
/// src/dispextern.h 1354 and `DRAW_IMAGE_SUNKEN` at src/dispextern.h 1355
/// (the enum is src/dispextern.h 1348-1356) - the `s->hl` tests at C 4202-4203,
/// 4213 and 4378-4380; `DRAW_CURSOR` (src/dispextern.h 1352) is round 16's row.
@_silgen_name("DRAW_IMAGE_RAISED") func DRAW_IMAGE_RAISED() -> Int32
@_silgen_name("DRAW_IMAGE_SUNKEN") func DRAW_IMAGE_SUNKEN() -> Int32

/// C: the four generated globals the bar geometry reads.
///
/// The two thicknesses are `intmax_t` members of the generated `globals` struct
/// - `intmax_t f_tab_bar_button_relief;` and `intmax_t f_tool_bar_button_relief;`
/// - so C's `min (…, 1000000)` and `< 0` tests are `Int` here, and the
/// assignment back to C's `int thick` is the lane's `truncatingIfNeeded:`
/// narrowing.  The first is declared at globals.h 942-943
/// and its `DEFVAR_INT` is at src/xdisp.c 38973; the second is declared at
/// globals.h 944-945 and its own is at src/xdisp.c 38979.
/// The two margins are `Lisp_Object`s; `Vtab_bar_button_margin` is the
/// `DEFVAR_LISP` at src/xdisp.c 38965
/// and `Vtool_bar_button_margin` is the one at src/xdisp.c 38985.
///
/// No reference object holds a symbol for any of the four (they are `globals`
/// fields; `docs/baseline/globals/variables.tsv` 753, 756, 395 and 413 are the
/// names' source), so the core exports them - the `windows_or_buffers_changed`
/// row of round 14.
@_silgen_name("tab_bar_button_relief") var tab_bar_button_relief: Int
@_silgen_name("tool_bar_button_relief") var tool_bar_button_relief: Int
@_silgen_name("Vtab_bar_button_margin") var Vtab_bar_button_margin: Lisp_Object
@_silgen_name("Vtool_bar_button_margin") var Vtool_bar_button_margin: Lisp_Object

/// C: `#define DEFAULT_TAB_BAR_BUTTON_RELIEF 1` at src/dispextern.h 3436 and
/// `#define DEFAULT_TOOL_BAR_BUTTON_RELIEF 1` at src/dispextern.h 3515
/// - the two literals the thickness ternaries fall back to.
///
/// **Spelled, not imported** (class B of the 3.255 ruling): the expansion is a
/// literal, so the `DOCK_IGNORE_LIMIT` reading of round 4 applies and no shim is
/// owed - the exception the `FRAME_NS_TITLEBAR_HEIGHT` pair records.  Both are
/// `Int` because the ternary's other arm is the `intmax_t` global above, and C's
/// narrowing to `int thick` happens at the assignment.
fileprivate let DEFAULT_TAB_BAR_BUTTON_RELIEF: Int = 1
fileprivate let DEFAULT_TOOL_BAR_BUTTON_RELIEF: Int = 1

/// C: `EmacsImage`'s two `@public` ivars and the two NSImage messages
/// C 4272-4355 sends.
///
/// `NSAffineTransform *transform;` and `BOOL smoothing;` are src/nsterm.h 680-681,
/// inside the `@interface EmacsImage : NSImage` at src/nsterm.h 670.
/// The class is another lane's (`nsimage.swift`'s `class EmacsImage: NSImage`)
/// and this unit has no Swift view of it, so each access crosses as a
/// receiver-first C-ABI entry - the `EmacsImage_stippleMask` shape of round 18
/// and the `EmacsTooltip_*` family (`nsfns.swift` 945-953).  `NSAffineTransform`
/// is the ivar's own type (round 15's `NSAffineTransform()`); the
/// `drawInRect:fromRect:operation:fraction:respectFlipped:hints:` method is
/// NSImage's own, declared at NSImage.h 133, so the entry keeps C's six arguments
/// and their types - `NSCompositingOperation` is what the `beginDrag` row
/// (`nsselect.swift` 224) does with `NSDragOperation`.
@_silgen_name("EmacsImage_transform")
func EmacsImage_transform(_ img: AnyObject?) -> NSAffineTransform?
@_silgen_name("EmacsImage_smoothing") func EmacsImage_smoothing(_ img: AnyObject?) -> Bool
@_silgen_name("EmacsImage_size") func EmacsImage_size(_ img: AnyObject?) -> NSSize
@_silgen_name("EmacsImage_drawInRect")
func EmacsImage_drawInRect(_ img: AnyObject?, _ dstRect: NSRect, _ srcRect: NSRect,
                           _ operation: NSCompositingOperation, _ fraction: CGFloat,
                           _ respectFlipped: Bool, _ hints: NSDictionary?)

/// C 4179-4264: `static void ns_draw_image_relief (struct glyph_string *s)` - the
/// image's relief, drawn from the slice, the image's margins and the bar
/// geometry.
///
/// Readings this body makes:
///
/// * C's `int x1, y1, thick;` and its five `bool` flags are uninitialized
///   locals, so they take a zero default here (`var x1: Int32 = 0`,
///   `var raised_p = false`) - the `ns_draw_relief` reading of round 18; C 4251's
///   chained `top_p = bot_p = left_p = right_p = false;` is kept as its four
///   assignments.
/// * C's comma statements - `x -= thick + extra_x, left_p = true;` (C 4253-4254)
///   and its three siblings at C 4255-4260 - are the two statements they are.
/// * `eabs (s->img->relief)` (C 4217) is the macro at src/lisp.h 5597,
///   whose expansion is `((x) < 0 ? -(x) : (x))`; it is spelled (class B): the
///   read is the entry and the macro's three evaluations of the same expression
///   have no side effect.  Sharp edge, recorded: the negated `Int32` traps on
///   `Int32.min` where C's is undefined - the class round 17's unsigned-arithmetic
///   note already carries, not exercised here.
/// * The two `min (…, 1000000)` fallbacks and the `extra_x`/`extra_y` arithmetic
///   narrow C's `intmax_t`/`EMACS_INT` back to `int`, so each is
///   `Int32(truncatingIfNeeded: …)` - the lane's narrowing rule.
/// * C's `extra_x = extra_y = XFIXNUM (…) - thick;` (C 4235 and 4248) is spelled
///   as its two assignments, as C 4251's chain is.
/// * C's `int x = s->x;` and `int y = s->ybase - image_ascent (…);` are the
///   entries above; `x`/`y` are `var` because C 4193-4200 and 4254-4256 mutate
///   them.
fileprivate func ns_draw_image_relief(_ s: UnsafeMutableRawPointer?) {
  var x1: Int32 = 0, y1: Int32 = 0, thick: Int32 = 0
  var raised_p = false, top_p = false, bot_p = false, left_p = false, right_p = false
  var extra_x: Int32 = 0, extra_y: Int32 = 0
  var x = glyph_string_x(s)
  var y = glyph_string_ybase(s) - image_ascent(glyph_string_img(s), glyph_string_face(s),
                                               glyph_string_slice_ptr(s))

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */
  if face_box(glyph_string_face(s)) != FACE_NO_BOX()
       && glyph_left_box_line_p(glyph_string_first_glyph(s))
       && glyph_string_slice_x(s) == 0 {
    x += max(face_box_vertical_line_width(glyph_string_face(s)), 0)
  }

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */
  if glyph_string_slice_x(s) == 0 {
    x += image_hmargin(glyph_string_img(s))
  }
  if glyph_string_slice_y(s) == 0 {
    y += image_vmargin(glyph_string_img(s))
  }

  if glyph_string_hl(s) == DRAW_IMAGE_SUNKEN()
       || glyph_string_hl(s) == DRAW_IMAGE_RAISED() {
    if face_id(glyph_string_face(s)) == TAB_BAR_FACE_ID() {
      thick = Int32(truncatingIfNeeded: tab_bar_button_relief < 0
                    ? DEFAULT_TAB_BAR_BUTTON_RELIEF
                    : min(tab_bar_button_relief, 1000000))
    } else {
      thick = Int32(truncatingIfNeeded: tool_bar_button_relief < 0
                    ? DEFAULT_TOOL_BAR_BUTTON_RELIEF
                    : min(tool_bar_button_relief, 1000000))
    }
    raised_p = glyph_string_hl(s) == DRAW_IMAGE_RAISED()
  } else {
    thick = image_relief(glyph_string_img(s)) < 0
      ? -image_relief(glyph_string_img(s)) : image_relief(glyph_string_img(s))
    raised_p = image_relief(glyph_string_img(s)) > 0
  }

  x1 = x + glyph_string_slice_width(s) - 1
  y1 = y + glyph_string_slice_height(s) - 1

  extra_x = 0
  extra_y = 0
  if face_id(glyph_string_face(s)) == TAB_BAR_FACE_ID() {
    if CONSP(Vtab_bar_button_margin)
         && FIXNUMP(XCAR(Vtab_bar_button_margin))
         && FIXNUMP(XCDR(Vtab_bar_button_margin)) {
      extra_x = Int32(truncatingIfNeeded: XFIXNUM(XCAR(Vtab_bar_button_margin)) - Int(thick))
      extra_y = Int32(truncatingIfNeeded: XFIXNUM(XCDR(Vtab_bar_button_margin)) - Int(thick))
    } else if FIXNUMP(Vtab_bar_button_margin) {
      extra_x = Int32(truncatingIfNeeded: XFIXNUM(Vtab_bar_button_margin) - Int(thick))
      extra_y = Int32(truncatingIfNeeded: XFIXNUM(Vtab_bar_button_margin) - Int(thick))
    }
  }

  if face_id(glyph_string_face(s)) == TOOL_BAR_FACE_ID() {
    if CONSP(Vtool_bar_button_margin)
         && FIXNUMP(XCAR(Vtool_bar_button_margin))
         && FIXNUMP(XCDR(Vtool_bar_button_margin)) {
      extra_x = Int32(truncatingIfNeeded: XFIXNUM(XCAR(Vtool_bar_button_margin)))
      extra_y = Int32(truncatingIfNeeded: XFIXNUM(XCDR(Vtool_bar_button_margin)))
    } else if FIXNUMP(Vtool_bar_button_margin) {
      extra_x = Int32(truncatingIfNeeded: XFIXNUM(Vtool_bar_button_margin))
      extra_y = Int32(truncatingIfNeeded: XFIXNUM(Vtool_bar_button_margin))
    }
  }

  top_p = false
  bot_p = false
  left_p = false
  right_p = false

  if glyph_string_slice_x(s) == 0 {
    x -= thick + extra_x
    left_p = true
  }
  if glyph_string_slice_y(s) == 0 {
    y -= thick + extra_y
    top_p = true
  }
  if glyph_string_slice_x(s) + glyph_string_slice_width(s) == image_width(glyph_string_img(s)) {
    x1 += thick + extra_x
    right_p = true
  }
  if glyph_string_slice_y(s) + glyph_string_slice_height(s) == image_height(glyph_string_img(s)) {
    y1 += thick + extra_y
    bot_p = true
  }

  ns_draw_relief(NSMakeRect(CGFloat(x), CGFloat(y), CGFloat(x1 - x + 1), CGFloat(y1 - y + 1)),
                 thick, thick, raised_p, top_p, bot_p, left_p, right_p, s)
}

/// C 4266-4393: `static void ns_dumpglyphs_image (struct glyph_string *s, NSRect r)` -
/// "Renders an image and associated borders."
///
/// Readings this body makes:
///
/// * `NSRect r` is **unused in the body** (C 4267's declaration carries it, C 4847
///   is the caller); it is kept, and Swift warns about neither unused parameters
///   nor an unread `let`.
/// * `EmacsImage *img = s->img->pixmap;` (C 4272) is the entry pair, and `img`
///   stays untyped (`AnyObject?`): the class is `nsimage.swift`'s.  C's
///   `if (img != nil)` is the same test.
/// * C's uninitialized locals take a zero default (`var bg_x: Int32 = 0`,
///   `var br = NSRect.zero`, `var tdCol: NSColor? = nil`) - the round-18 reading.
/// * C's truthiness tests on `int` members become `!= 0` (the `bg_height >
///   s->slice.height` head of C 4301-4302 and its five `||` arms), and the two
///   pointer members are `nil` tests (`s->img->mask`, `s->img->pixmap == 0`) -
///   Swift has no truthiness.
/// * `NSColor colorWithUnsignedLong:` is this unit's own category (round 1);
///   `[col set]` is `?.set()` through the optional receiver C relies on.
/// * `[NSGraphicsContext currentContext]` is `NSGraphicsContext.current?` (the
///   round-17 reading), `saveGraphicsState`/`restoreGraphicsState` are its
///   methods, and `setImageInterpolation:NSImageInterpolationNone` is the Swift
///   property assignment `imageInterpolation = .none`
///   (`NSImageInterpolationDefault` is `.default`).
/// * `[NSAffineTransform transform]` is Swift's `NSAffineTransform()` (round 15),
///   `translateXBy:yBy:` is `translateX(by:yBy:)` and `concat` is `concat()`.
///   `img->transform`'s non-nil test binds once (`if let`) instead of C's two
///   reads, and the value crosses the Foundation `SwiftBridge` into
///   `append(_:)`'s `AffineTransform` parameter with the `as AffineTransform`
///   cast round 15 uses for `transform(using:)` - the compiler proved the
///   rename (`'appendTransform' has been renamed to 'append(_:)'`).
/// * `[img size].width` is the `EmacsImage_size` entry read twice, exactly the
///   two messages C 4324 sends;
///   `[img drawInRect:fromRect:operation:fraction:respectFlipped:hints:]` is the
///   `EmacsImage_drawInRect` entry with C's own six arguments.
/// * `abs (s->img->relief)` (C 4388) and `max (…, 0)` (C 4273) keep C's calls -
///   Swift's `abs`/`max` on `Int32`; `ns_draw_box`'s two `char` flags are the
///   `Bool` round 18 established, and its `int thickness` arguments convert as
///   `CGFloat`.
fileprivate func ns_dumpglyphs_image(_ s: UnsafeMutableRawPointer?, _ r: NSRect) {
  let img = image_pixmap(glyph_string_img(s))
  let box_line_vwidth = max(face_box_horizontal_line_width(glyph_string_face(s)), 0)
  var x = glyph_string_x(s)
  var y = glyph_string_ybase(s) - image_ascent(glyph_string_img(s), glyph_string_face(s),
                                               glyph_string_slice_ptr(s))
  var bg_x: Int32 = 0, bg_y: Int32 = 0, bg_height: Int32 = 0
  var br = NSRect.zero
  let face = glyph_string_face(s)
  var tdCol: NSColor? = nil

  // C 4280: `NSTRACE ("ns_dumpglyphs_image");` - empty at `NSTRACE_ENABLED` 0, like the rest.

  if face_box(face) != FACE_NO_BOX()
       && glyph_left_box_line_p(glyph_string_first_glyph(s))
       && glyph_string_slice_x(s) == 0 {
    x += max(face_box_vertical_line_width(face), 0)
  }

  bg_x = x
  bg_y = glyph_string_slice_y(s) == 0 ? glyph_string_y(s) : glyph_string_y(s) + box_line_vwidth
  bg_height = glyph_string_height(s)
  /* other terms have this, but was causing problems w/tabbar mode */
  /* - 2 * box_line_vwidth; */

  if glyph_string_slice_x(s) == 0 { x += image_hmargin(glyph_string_img(s)) }
  if glyph_string_slice_y(s) == 0 { y += image_vmargin(glyph_string_img(s)) }

  /* Draw BG: if we need larger area than image itself cleared, do that,
     otherwise, since we composite the image under NS (instead of mucking
     with its background color), we must clear just the image area.  */

  NSColor.colorWithUnsignedLong(NS_FACE_BACKGROUND(face)).set()

  if bg_height > glyph_string_slice_height(s)
       || image_hmargin(glyph_string_img(s)) != 0
       || image_vmargin(glyph_string_img(s)) != 0
       || image_mask(glyph_string_img(s)) != nil
       || image_pixmap(glyph_string_img(s)) == nil
       || glyph_string_width(s) != glyph_string_background_width(s) {
    br = NSMakeRect(CGFloat(bg_x), CGFloat(bg_y), CGFloat(glyph_string_background_width(s)),
                    CGFloat(bg_height))
    glyph_string_set_background_filled_p(s, true)
  } else {
    br = NSMakeRect(CGFloat(x), CGFloat(y), CGFloat(glyph_string_slice_width(s)),
                    CGFloat(glyph_string_slice_height(s)))
  }

  NSRectFill(br)

  /* Draw the image... do we need to draw placeholder if img == nil?  */
  if img != nil {
    /* The idea here is that the clipped area is set in the normal
       view coordinate system, then we transform the coordinate
       system so that when we draw the image it is rotated, resized
       or whatever as required.  This is kind of backwards, but
       there's no way to apply the transform to the image without
       creating a whole new bitmap.  */
    let dr = NSMakeRect(CGFloat(x), CGFloat(y), CGFloat(glyph_string_slice_width(s)),
                        CGFloat(glyph_string_slice_height(s)))
    let ir = NSMakeRect(0, 0, EmacsImage_size(img).width, EmacsImage_size(img).height)

    let setOrigin = NSAffineTransform()

    NSGraphicsContext.current?.saveGraphicsState()

    /* Because of the transforms it's difficult to work out what
       portion of the original, untransformed, image will be drawn,
       so the clipping area will ensure we draw only the correct
       bit.  */
    NSRectClip(dr)

    setOrigin.translateX(by: CGFloat(x - glyph_string_slice_x(s)),
                         yBy: CGFloat(y - glyph_string_slice_y(s)))
    setOrigin.concat()

    let doTransform = NSAffineTransform()

    /* ImageMagick images don't have transforms.  */
    if let transform = EmacsImage_transform(img) {
      doTransform.append(transform as AffineTransform)
    }

    doTransform.concat()

    /* Smoothing is the default, so if we don't want smoothing we
       have to turn it off.  */
    if !EmacsImage_smoothing(img) {
      NSGraphicsContext.current?.imageInterpolation = .none
    }

    EmacsImage_drawInRect(img, ir, ir, NSCompositingOperation.sourceOver, 1.0, true, nil)

    /* Apparently image interpolation is not reset with
       restoreGraphicsState, so we have to manually reset it.  */
    if !EmacsImage_smoothing(img) {
      NSGraphicsContext.current?.imageInterpolation = .default
    }

    NSGraphicsContext.current?.restoreGraphicsState()
  }

  if glyph_string_hl(s) == DRAW_CURSOR() {
    FRAME_CURSOR_COLOR(glyph_string_f(s))?.set()
    tdCol = NSColor.colorWithUnsignedLong(NS_FACE_BACKGROUND(face))
  } else {
    tdCol = NSColor.colorWithUnsignedLong(NS_FACE_FOREGROUND(face))
  }

  /* Draw underline, overline, strike-through.  */
  ns_draw_text_decoration(s, face, tdCol, br.size.width, br.origin.x)

  /* If we must draw a relief around the image, do it.  */
  if image_relief(glyph_string_img(s)) != 0
       || glyph_string_hl(s) == DRAW_IMAGE_RAISED()
       || glyph_string_hl(s) == DRAW_IMAGE_SUNKEN() {
    ns_draw_image_relief(s)
  }

  /* If there is no mask, the background won't be seen, so draw a
     rectangle on the image for the cursor.  Do this for all images,
     getting transparency right is not reliable.  */
  if glyph_string_hl(s) == DRAW_CURSOR() {
    var thickness = abs(image_relief(glyph_string_img(s)))
    if thickness == 0 { thickness = 1 }
    ns_draw_box(br, CGFloat(thickness), CGFloat(thickness),
                FRAME_CURSOR_COLOR(glyph_string_f(s)), true, true)
  }
}

// ==========================================================================
// C 4396-5001: the stretch glyph, the three foregrounds, the GC setter and the
// glyph-string dispatcher - the Glyph drawing operations section closes.
// ==========================================================================

/// C: `bool x_stretch_cursor_p;` - the `x-stretch-cursor` variable, read at C 4402.
///
/// It is a generated `globals` field (`bool f_x_stretch_cursor_p;`, globals.h 1230-1231,
/// from the `DEFVAR_BOOL` at src/xdisp.c 38638), so no object holds a symbol of this
/// name and the core owes the export - the `tab_bar_button_relief` shape of round 19.
@_silgen_name("x_stretch_cursor_p") var x_stretch_cursor_p: Bool

/// C: `struct glyph_row`'s `bool_bf reversed_p : 1` (src/dispextern.h 1141, read at
/// C 4409, 4431 and 4447 - the stretch block's R2L tests), `bool_bf stipple_p : 1`
/// (1144, the read at C 4483 and the **write** at C 4484, so the `_set_` pair) and
/// `bool_bf mouse_face_p : 1` (1108, read at C 4452-4453) - three more members of the row
/// family round 13 started.
@_silgen_name("glyph_row_reversed_p")
func glyph_row_reversed_p(_ row: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_row_stipple_p")
func glyph_row_stipple_p(_ row: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_row_set_stipple_p")
func glyph_row_set_stipple_p(_ row: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("glyph_row_mouse_face_p")
func glyph_row_mouse_face_p(_ row: UnsafeMutableRawPointer?) -> Bool

/// C: `int window_box_left_offset (struct window *, enum glyph_row_area)` (declared
/// src/dispextern.h 3580) and `int window_box_left (…)` (declared 3579) - defined at
/// src/xdisp.c 1385 and 1430, **before** that file's glyph-clipping block, so both are
/// real symbols of the frozen build (measured `T` in `build/emacs-ref/src/xdisp.o`) and
/// the core's `xdisp.rs` provides them (`window_box_right`'s reading of round 18); C 4411
/// and C 4473 read them.
@_silgen_name("window_box_left_offset")
func window_box_left_offset(_ w: UnsafeMutableRawPointer?, _ area: Int32) -> Int32
@_silgen_name("window_box_left")
func window_box_left(_ w: UnsafeMutableRawPointer?, _ area: Int32) -> Int32

/// C: `#define FRAME_COLUMN_WIDTH(F) ((F)->column_width)` (src/frame.h 1565) - the
/// `min (…)` partner of C 4430.  Class A of the 3.255 ruling (`FRAME_CURSOR_WIDTH`'s
/// shape), and its member is `int column_width;` (src/frame.h 641), so the entry is `Int32`.
@_silgen_name("FRAME_COLUMN_WIDTH")
func FRAME_COLUMN_WIDTH(_ f: UnsafeMutableRawPointer?) -> Int32

/// C: `bool cursor_in_mouse_face_p (struct window *w)` (declared src/dispextern.h 3692,
/// defined src/xdisp.c 35727) - C 4453's second test.  The frozen objects hold the symbol
/// (`T` in `build/emacs-ref/src/xdisp.o`); the core does not export it yet (measured), so
/// the row is the class the gate calls `reference` and the core owes its provider.
@_silgen_name("cursor_in_mouse_face_p")
func cursor_in_mouse_face_p(_ w: UnsafeMutableRawPointer?) -> Bool

/// C: `struct mouse_highlight`'s `int mouse_face_face_id;` (src/dispextern.h 3007) - the
/// member `MOUSE_HL_INFO (s->f)->mouse_face_face_id` reads at C 4456
/// (`mouse_highlight_mouse_face_mouse_frame`'s family, round 7).
@_silgen_name("mouse_highlight_mouse_face_face_id")
func mouse_highlight_mouse_face_face_id(_ hlinfo: UnsafeMutableRawPointer?) -> Int32
/// C: src/dispextern.h 1971's `MOUSE_FACE_ID` - the `enum face_id` member C 4459 falls
/// back to (`DEFAULT_FACE_ID`'s family, round 18).
@_silgen_name("MOUSE_FACE_ID") func MOUSE_FACE_ID() -> Int32
/// C: `void prepare_face_for_display (struct frame *, struct face *)` (declared
/// src/dispextern.h 3810, defined src/xfaces.c 4764) - C 4461's call, in the cursor
/// fallback.  Neither the frozen objects nor the core holds the symbol (measured), so the
/// row is a shim work order rather than a "provided" reading.
@_silgen_name("prepare_face_for_display")
func prepare_face_for_display(_ f: UnsafeMutableRawPointer?, _ face: UnsafeMutableRawPointer?)

/// C 4396-4545: `static void ns_draw_stretch_glyph_string (struct glyph_string *s)` -
/// the stretch glyph's cursor background, its stipple or plain background, and its text
/// decoration.
///
/// Readings this body makes:
///
/// * C's `int width, background_width = s->background_width;` and `int x = s->x;` are the
///   three locals below; `width` is uninitialized in C, so it takes the zero default
///   (`ns_draw_image_relief`'s reading of round 19).
/// * C 4434 re-tests `s->hl == DRAW_CURSOR` inside an arm that already established it
///   (C 4402) - the reference's own duplication, transcribed as it stands.
/// * `[[NSColor colorWithUnsignedLong: …] set]` is this unit's own `NSColor` category
///   method, and `[FRAME_CURSOR_COLOR (s->f) set]` is `FRAME_CURSOR_COLOR(…)?.set()`
///   (round 19's optional receiver).
/// * `CGRectMake` is the spelled initializer (class B, round 19) and the `NS_IMPL_COCOA`
///   stipple arm is the live one; the `#else`'s single `[[…stippleMask] set];` is the
///   GNUstep text, recorded.
fileprivate func ns_draw_stretch_glyph_string(_ s: UnsafeMutableRawPointer?) {
  var face: UnsafeMutableRawPointer? = nil
  var fg_color: NSColor? = nil

  if glyph_string_hl(s) == DRAW_CURSOR() && !x_stretch_cursor_p {
    /* If `x-stretch-cursor' is nil, don't draw a block cursor as
       wide as the stretch glyph.  */
    var width: Int32 = 0
    var background_width = glyph_string_background_width(s)
    var x = glyph_string_x(s)

    if !glyph_row_reversed_p(glyph_string_row(s)) {
      let left_x = window_box_left_offset(glyph_string_w(s), TEXT_AREA())

      if x < left_x {
        background_width -= left_x - x
        x = left_x
      }
    } else {
      /* In R2L rows, draw the cursor on the right edge of the
         stretch glyph.  */
      let right_x = window_box_right(glyph_string_w(s), TEXT_AREA())

      if x + background_width > right_x {
        background_width -= x - right_x
      }
      x += background_width
    }

    width = min(FRAME_COLUMN_WIDTH(glyph_string_f(s)), background_width)
    if glyph_row_reversed_p(glyph_string_row(s)) {
      x -= width
    }

    if glyph_string_hl(s) == DRAW_CURSOR() {
      FRAME_CURSOR_COLOR(glyph_string_f(s))?.set()
    } else {
      NSColor.colorWithUnsignedLong(face_foreground(glyph_string_face(s))).set()
    }

    NSRectFill(NSMakeRect(CGFloat(x), CGFloat(glyph_string_y(s)), CGFloat(width),
                          CGFloat(glyph_string_height(s))))

    /* Clear rest using the GC of the original non-cursor face.  */
    if width < background_width {
      let y = glyph_string_y(s)
      let w = background_width - width
      let h = glyph_string_height(s)

      if !glyph_row_reversed_p(glyph_string_row(s)) {
        x += width
      } else {
        x = glyph_string_x(s)
      }

      if glyph_row_mouse_face_p(glyph_string_row(s))
           && cursor_in_mouse_face_p(glyph_string_w(s)) {
        face = FACE_FROM_ID_OR_NULL(glyph_string_f(s),
                                    mouse_highlight_mouse_face_face_id(
                                      MOUSE_HL_INFO(glyph_string_f(s))))

        if face == nil {
          face = FACE_FROM_ID(glyph_string_f(s), MOUSE_FACE_ID())
        }
        prepare_face_for_display(glyph_string_f(s), face)

        NSColor.colorWithUnsignedLong(face_background(face)).set()
      } else {
        NSColor.colorWithUnsignedLong(face_background(glyph_string_face(s))).set()
      }
      NSRectFill(NSMakeRect(CGFloat(x), CGFloat(y), CGFloat(w), CGFloat(h)))
    }
  } else if !glyph_string_background_filled_p(s) {
    var background_width = glyph_string_background_width(s)
    var x = glyph_string_x(s)
    let text_left_x = window_box_left(glyph_string_w(s), TEXT_AREA())

    /* Don't draw into left fringe or scrollbar area except for
       header line and mode line.  */
    if glyph_string_area(s) == TEXT_AREA()
         && x < text_left_x && !glyph_row_mode_line_p(glyph_string_row(s)) {
      background_width -= text_left_x - x
      x = text_left_x
    }

    if !glyph_row_stipple_p(glyph_string_row(s)) {
      glyph_row_set_stipple_p(glyph_string_row(s), glyph_string_stippled_p(s))
    }

    if background_width > 0 {
      let dpyinfo = FRAME_DISPLAY_INFO(glyph_string_f(s))
      if glyph_string_hl(s) == DRAW_CURSOR() {
        FRAME_CURSOR_COLOR(glyph_string_f(s))?.set()
      } else if glyph_string_stippled_p(s) {
        /* On cocoa emacs the stipple is stored as a mask CGImage.
           First we want to clear the background with the bg color.  */
        NSColor.colorWithUnsignedLong(face_background(glyph_string_face(s))).set()
        NSRectFill(NSMakeRect(CGFloat(x), CGFloat(glyph_string_y(s)),
                              CGFloat(background_width), CGFloat(glyph_string_height(s))))

        /* This part could possibly be improved, the author is unfamiliar with
           NS/CoreGraphics and isn't sure if it's possible to do this with NSImage.  */
        let mask = EmacsImage_stippleMask(
          ns_display_info_bitmaps_img(dpyinfo, face_stipple(glyph_string_face(s)) - 1))
        let bounds = CGRect(x: CGFloat(glyph_string_x(s)), y: CGFloat(glyph_string_y(s)),
                            width: CGFloat(glyph_string_background_width(s)),
                            height: CGFloat(glyph_string_height(s)))

        /* Checkpoint the graphics state and then focus in on the
           area we're going to fill.  */
        let ctx = NSGraphicsContext.current
        ctx?.saveGraphicsState()
        let context = ctx?.cgContext
        CGContextClipToRect(context, bounds)
        CGContextScaleCTM(context, 1, -1)

        /* Stamp the foreground color using the stipple mask.  */
        NSColor.colorWithUnsignedLong(face_foreground(glyph_string_face(s))).set()
        let imageSize = CGRect(x: 0, y: 0, width: CGFloat(CGImageGetWidth(mask)),
                               height: CGFloat(CGImageGetHeight(mask)))
        CGContextDrawTiledImage(context, imageSize, mask)

        NSGraphicsContext.current?.restoreGraphicsState()
        // C 4523: the `#else` arm's `[[dpyinfo->bitmaps[…].img stippleMask] set];` is the
        // GNUstep text behind `#ifdef NS_IMPL_COCOA` - recorded, not transcribed.
      } else {
        NSColor.colorWithUnsignedLong(face_background(glyph_string_face(s))).set()
      }

      NSRectFill(NSMakeRect(CGFloat(x), CGFloat(glyph_string_y(s)),
                            CGFloat(background_width), CGFloat(glyph_string_height(s))))
    }
  }

  /* Draw overlining, etc. on the stretch glyph (or the part of the
     stretch glyph after the cursor).  If the glyph has a box, then
     decorations will be drawn after drawing the box in
     ns_draw_glyph_string, in order to prevent them from being
     overwritten by the box.  */
  if face_box(glyph_string_face(s)) == FACE_NO_BOX() {
    fg_color = NSColor.colorWithUnsignedLong(NS_FACE_FOREGROUND(glyph_string_face(s)))
    ns_draw_text_decoration(s, glyph_string_face(s), fg_color,
                            CGFloat(glyph_string_background_width(s)), CGFloat(glyph_string_x(s)))
  }
}

/// C: `struct font_driver`'s `draw` slot
/// `int (*draw) (struct glyph_string *s, int from, int to, int x, int y,
/// bool with_background)` (src/font.h 683-684) - the second slot this lane reaches through
/// round 14's two-hop `font_driver` read (`font_driver_text_extents`'s shape) - and the
/// driver's `encode_char` slot `unsigned (*encode_char) (struct font *font, int c)`
/// (src/font.h 667's `encode_char`), the third.  `draw` returns C's `int`, which C drops at
/// every one of this slice's eight call sites.
typealias font_draw_fn = @convention(c) (UnsafeMutableRawPointer?, Int32, Int32, Int32, Int32,
                                         Bool) -> Int32
typealias font_encode_char_fn = @convention(c) (UnsafeMutableRawPointer?, Int32) -> UInt32
@_silgen_name("font_driver_draw")
func font_driver_draw(_ driver: UnsafeMutableRawPointer?) -> font_draw_fn?
@_silgen_name("font_driver_encode_char")
func font_driver_encode_char(_ driver: UnsafeMutableRawPointer?) -> font_encode_char_fn?

/// C 4547-4564: `static void ns_draw_glyph_string_foreground (struct glyph_string *s)` -
/// the plain text foreground: one driver call and no background.
///
/// `font->driver->draw (…)` is the two-hop read above; C calls the slot unconditionally,
/// so the port's `if let` is the nil check C's driver contract makes unnecessary, and `_ =`
/// is C's dropped `int` return (`_ = composition_gstring_width`'s shape, round 14).
/// `!s->for_overlaps` on the bitfield entry is `== 0` (round 18's reading).
fileprivate func ns_draw_glyph_string_foreground(_ s: UnsafeMutableRawPointer?) {
  var x: Int32 = 0
  let font = glyph_string_font(s)

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if glyph_string_face(s) != nil && face_box(glyph_string_face(s)) != FACE_NO_BOX()
       && glyph_left_box_line_p(glyph_string_first_glyph(s)) {
    x = glyph_string_x(s) + max(face_box_vertical_line_width(glyph_string_face(s)), 0)
  } else {
    x = glyph_string_x(s)
  }

  if let draw = font_driver_draw(font_driver(font)) {
    _ = draw(s, glyph_string_cmp_from(s), glyph_string_nchars(s), x,
             glyph_string_ybase(s),
             glyph_string_for_overlaps(s) == 0 && !glyph_string_background_filled_p(s))
  }
}

/// C: `struct composition`'s `short *offsets;` (src/composite.h 177) with C's own
/// subscript - `s->cmp->offsets[j * 2]` (C 4605) and `… [j * 2 + 1]` (C 4606) - so the row
/// is a member entry whose second argument is that subscript (`glyph_row_used`'s shape),
/// and the `short` promotes to `int` in C's arithmetic, hence `Int32`.
@_silgen_name("composition_offsets")
func composition_offsets(_ cmp: UnsafeMutableRawPointer?, _ i: Int32) -> Int32
/// C: `#define COMPOSITION_GLYPH(cmp, n)` (src/composite.h 91-93, `XFIXNUM (AREF (…))`) -
/// the abbreviation table read at C 4603.
@_silgen_name("COMPOSITION_GLYPH")
func COMPOSITION_GLYPH(_ cmp: UnsafeMutableRawPointer?, _ n: Int32) -> Int
/// C: `struct face`'s `bool_bf overstrike : 1` (src/dispextern.h 1903) - the
/// `s->face->overstrike` read at C 4609, 4632, 4640 and 4651.
@_silgen_name("face_overstrike") func face_overstrike(_ face: UnsafeMutableRawPointer?) -> Bool

/// C: the `lglyph` family the compound branch of C 4613-4654 reads - `LGSTRING_GLYPH` and
/// `LGLYPH_ADJUSTMENT` (src/composite.h 273 and 312) plus the three adjustment members
/// `LGLYPH_WIDTH`/`LGLYPH_XOFF`/`LGLYPH_YOFF`/`LGLYPH_WADJUST` (307, 328, 330 and 332).
/// Each expands to `XFIXNUM`-of-`AREF` arithmetic over `Lisp_Object`s, so the three offsets
/// are C's `ptrdiff_t` (Swift `Int`) and the port narrows them at the `int` locals with
/// `Int32(truncatingIfNeeded:)` (round 17's narrowing rule).
@_silgen_name("LGSTRING_GLYPH")
func LGSTRING_GLYPH(_ lgs: Lisp_Object, _ idx: Int) -> Lisp_Object
@_silgen_name("LGLYPH_ADJUSTMENT")
func LGLYPH_ADJUSTMENT(_ g: Lisp_Object) -> Lisp_Object
@_silgen_name("LGLYPH_WIDTH") func LGLYPH_WIDTH(_ g: Lisp_Object) -> Int
@_silgen_name("LGLYPH_XOFF") func LGLYPH_XOFF(_ g: Lisp_Object) -> Int
@_silgen_name("LGLYPH_YOFF") func LGLYPH_YOFF(_ g: Lisp_Object) -> Int
@_silgen_name("LGLYPH_WADJUST") func LGLYPH_WADJUST(_ g: Lisp_Object) -> Int

/// C 4567-4655: `static void ns_draw_composite_glyph_string_foreground
/// (struct glyph_string *s)` - the two composition shapes: the rule-based walk over
/// `s->cmp->offsets` and the automatic one over the `lglyph` vector.
///
/// Two more readings: C's `'\t'` comparison is the character constant, spelled `9` here,
/// and the `font_not_found_p` arm's `NSRect r` is a local of the same name as nothing else
/// in this function (the dispatcher's array is elsewhere).
fileprivate func ns_draw_composite_glyph_string_foreground(_ s: UnsafeMutableRawPointer?) {
  var i: Int32 = 0, j: Int32 = 0, x: Int32 = 0
  let font = glyph_string_font(s)

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if glyph_string_face(s) != nil && face_box(glyph_string_face(s)) != FACE_NO_BOX()
       && glyph_left_box_line_p(glyph_string_first_glyph(s)) {
    x = glyph_string_x(s) + max(face_box_vertical_line_width(glyph_string_face(s)), 0)
  } else {
    x = glyph_string_x(s)
  }

  /* S is a glyph string for a composition.  S->cmp_from is the index
     of the first character drawn for glyphs of this composition.
     S->cmp_from == 0 means we are drawing the very first character of
     this composition.  */

  /* Draw a rectangle for the composition if the font for the very
     first character of the composition could not be loaded.  */
  if glyph_string_font_not_found_p(s) {
    if glyph_string_cmp_from(s) == 0 {
      let r = NSMakeRect(CGFloat(glyph_string_x(s)), CGFloat(glyph_string_y(s)),
                         CGFloat(glyph_string_width(s) - 1),
                         CGFloat(glyph_string_height(s) - 1))
      ns_draw_box(r, 1, 1, FRAME_CURSOR_COLOR(glyph_string_f(s)), true, true)
    }
  } else if !glyph_u_cmp_automatic(glyph_string_first_glyph(s)) {
    let y = glyph_string_ybase(s)

    i = 0
    j = glyph_string_cmp_from(s)
    while i < glyph_string_nchars(s) {
      /* TAB in a composition means display glyphs with padding
         space on the left or right.  */
      if COMPOSITION_GLYPH(glyph_string_cmp(s), j) != 9 {
        let xx = x + composition_offsets(glyph_string_cmp(s), j * 2)
        let yy = y - composition_offsets(glyph_string_cmp(s), j * 2 + 1)

        if let draw = font_driver_draw(font_driver(font)) {
          _ = draw(s, j, j + 1, xx, yy, false)
          if face_overstrike(glyph_string_face(s)) {
            _ = draw(s, j, j + 1, xx + 1, yy, false)
          }
        }
      }
      i += 1
      j += 1
    }
  } else {
    let gstring = composition_gstring_from_id(glyph_string_cmp_id(s))
    var glyph: Lisp_Object = nil
    let y = glyph_string_ybase(s)
    var width: Int32 = 0

    i = glyph_string_cmp_from(s)
    j = i
    while i < glyph_string_cmp_to(s) {
      glyph = LGSTRING_GLYPH(gstring, Int(i))
      if NILP(LGLYPH_ADJUSTMENT(glyph)) {
        width += Int32(truncatingIfNeeded: LGLYPH_WIDTH(glyph))
      } else {
        var xoff: Int32 = 0, yoff: Int32 = 0, wadjust: Int32 = 0

        if j < i {
          if let draw = font_driver_draw(font_driver(font)) {
            _ = draw(s, j, i, x, y, false)
            if face_overstrike(glyph_string_face(s)) {
              _ = draw(s, j, i, x + 1, y, false)
            }
          }
          x += width
        }
        xoff = Int32(truncatingIfNeeded: LGLYPH_XOFF(glyph))
        yoff = Int32(truncatingIfNeeded: LGLYPH_YOFF(glyph))
        wadjust = Int32(truncatingIfNeeded: LGLYPH_WADJUST(glyph))
        if let draw = font_driver_draw(font_driver(font)) {
          _ = draw(s, i, i + 1, x + xoff, y + yoff, false)
          if face_overstrike(glyph_string_face(s)) {
            _ = draw(s, i, i + 1, x + xoff + 1, y + yoff, false)
          }
        }
        x += wadjust
        j = i + 1
        width = 0
      }
      i += 1
    }
    if j < i {
      if let draw = font_driver_draw(font_driver(font)) {
        _ = draw(s, j, i, x, y, false)
        if face_overstrike(glyph_string_face(s)) {
          _ = draw(s, j, i, x + 1, y, false)
        }
      }
    }
  }
}

/// C: `struct glyph`'s `u.glyphless` sub-struct (src/dispextern.h 624-635): the
/// `unsigned method : 2` (628, read at C 4686, 4704 and 4728), the
/// `bool_bf for_no_font : 1` (630, C 4694) and the two widths `unsigned len : 4` (632,
/// C 4684) and `unsigned ch : 25` (634, C 4696 and 4706).  Each field promotes to `int` in
/// C's expressions, so the three numeric entries are `Int32` (round 18's `glyph_u_cmp_id`).
@_silgen_name("glyph_u_glyphless_method")
func glyph_u_glyphless_method(_ g: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_u_glyphless_for_no_font")
func glyph_u_glyphless_for_no_font(_ g: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_u_glyphless_len")
func glyph_u_glyphless_len(_ g: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_u_glyphless_ch")
func glyph_u_glyphless_ch(_ g: UnsafeMutableRawPointer?) -> Int32
/// C: the three `enum glyphless_display_method` members C 4670 and 4684-4704 test
/// (src/dispextern.h 2351, 2355 and 2357).
@_silgen_name("GLYPHLESS_DISPLAY_THIN_SPACE")
func GLYPHLESS_DISPLAY_THIN_SPACE() -> Int32
@_silgen_name("GLYPHLESS_DISPLAY_ACRONYM")
func GLYPHLESS_DISPLAY_ACRONYM() -> Int32
@_silgen_name("GLYPHLESS_DISPLAY_HEX_CODE")
func GLYPHLESS_DISPLAY_HEX_CODE() -> Int32
/// C: `Lisp_Object Vglyphless_char_display;` - the `glyphless-char-display` variable the
/// acronym branch reads at C 4689, 4690, 4695 and 4697.
///
/// A generated `globals` field too (like `x_stretch_cursor_p` above): `Lisp_Object
/// f_Vglyphless_char_display;`, globals.h 296-297, the `DEFVAR_LISP` at src/xdisp.c 39354,
/// so no object holds a symbol of this name and the core owes the export.
@_silgen_name("Vglyphless_char_display") var Vglyphless_char_display: Lisp_Object

/// C: the char-table reads of C 4685-4690 - `CHAR_TABLE_P` and `XCHAR_TABLE`
/// (src/lisp.h 2085 and 2091, both `INLINE`s, class A), `CHAR_TABLE_EXTRA_SLOTS`
/// (2263, also an `INLINE`), `CHAR_TABLE_REF` (2150) and the `->extras[…]` member read
/// (src/lisp.h 2081's `extras`, the record's flexible array).  `nsfont.swift` 531-532
/// declares `XCHAR_TABLE` with a `struct Lisp_Char_Table` view of its own; this unit has no
/// such view (the record is the core's), so the member read is an entry whose second
/// argument is C's own subscript 0 - the `ns_display_info_bitmaps_img` shape of round 18.
@_silgen_name("CHAR_TABLE_P") func CHAR_TABLE_P(_ a: Lisp_Object) -> Bool
@_silgen_name("XCHAR_TABLE")
func XCHAR_TABLE(_ a: Lisp_Object) -> UnsafeMutableRawPointer?
@_silgen_name("CHAR_TABLE_EXTRA_SLOTS")
func CHAR_TABLE_EXTRA_SLOTS(_ ct: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("CHAR_TABLE_REF")
func CHAR_TABLE_REF(_ ct: Lisp_Object, _ idx: Int32) -> Lisp_Object
@_silgen_name("char_table_extras")
func char_table_extras(_ ct: UnsafeMutableRawPointer?, _ i: Int32) -> Lisp_Object
/// C: `struct glyph`'s `slice.glyphless` sub-struct - `short upper_xoff, upper_yoff;` and
/// `short lower_xoff, lower_yoff;` (src/dispextern.h 583-586), read at C 4720-4725; each
/// `short` promotes to `int` in C's `x + …`, so the entries are `Int16` and the use site
/// widens (the `glyph_ascent` reading of round 16).
@_silgen_name("glyph_slice_glyphless_upper_xoff")
func glyph_slice_glyphless_upper_xoff(_ g: UnsafeMutableRawPointer?) -> Int16
@_silgen_name("glyph_slice_glyphless_upper_yoff")
func glyph_slice_glyphless_upper_yoff(_ g: UnsafeMutableRawPointer?) -> Int16
@_silgen_name("glyph_slice_glyphless_lower_xoff")
func glyph_slice_glyphless_lower_xoff(_ g: UnsafeMutableRawPointer?) -> Int16
@_silgen_name("glyph_slice_glyphless_lower_yoff")
func glyph_slice_glyphless_lower_yoff(_ g: UnsafeMutableRawPointer?) -> Int16

/// C: `struct glyph_string`'s `unsigned *char2b;` (src/dispextern.h 1407) - the member
/// C 4695 writes the local buffer's address into and C 4745 clears.  Round 14 declared the
/// *read* (`glyph_string_char2b`); this slice adds the two writes.
@_silgen_name("glyph_string_set_char2b")
func glyph_string_set_char2b(_ s: UnsafeMutableRawPointer?, _ v: UnsafeMutablePointer<UInt32>?)

/// C 4658-4741: `static void ns_draw_glyphless_glyph_string_foreground
/// (struct glyph_string *s)` - "Draw the foreground of glyph string S for glyphless
/// characters."
///
/// Four readings, beyond the entries above:
///
/// * `NSGlyph char2b[8];` (C 4662) is `unsigned int` in this SDK (AppKit's NSFont.h 147),
///   so the buffer is `[UInt32]` of eight and C's two assignments, C 4673's
///   `s->char2b = char2b;` and C 4740's `s->char2b = NULL;`, are the `_set_` entry's.  C's array *is* the storage the driver reads through that pointer, so the
///   pointer has to live across the whole walk - Swift's array storage is only valid inside
///   `withUnsafeMutableBufferPointer`, which is why the pair and the loop sit in that
///   closure (the `withUnsafeBufferPointer` reading `macfont.swift` 2207 uses).
/// * `char buf[8 + PACIFY_GCC_BUG_81401];` (C 4682) is eight bytes here: the `#ifdef
///   GCC_LINT` arm's 1 (`enum { PACIFY_GCC_BUG_81401 = 1 };`, C 4678) is for a compiler this
///   build does not use, and the live arm's 0 (C 4680) is the one transcribed - a dead arm,
///   recorded.
/// * `snprintf (buf, 8, "%0*X", ch < 0x10000 ? 4 : 6, ch);` (C 4708) is variadic and has no Swift
///   spelling (round 12's `get_keysym_name` reading), so the port writes the same
///   conversion into the same buffer: an uppercase zero-padded hexadecimal number of width
///   4 or 6, and `ch` is at most `MAX_CHAR` (0x3FFFFF, six digits), so C's eight-byte bound
///   is kept.  `str` therefore points into `buf` only inside `buf`'s closure.
/// * `eassume (ch <= MAX_CHAR);` (C 4707) is the src/lisp.h 182/193 macro - `assume (cond)`,
///   a compiler hint with no statement when `ENABLE_CHECKING` is off, which this
///   configuration leaves undefined (round 13's `eassert (w)` reading); it is recorded, and
///   `MAX_CHAR` (src/character.h 49) is not imported with it.
fileprivate func ns_draw_glyphless_glyph_string_foreground(_ s: UnsafeMutableRawPointer?) {
  var glyph = glyph_string_first_glyph(s)
  var char2b = [UInt32](repeating: 0, count: 8)
  var x: Int32 = 0
  var i: Int32 = 0
  var j: Int32 = 0

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if glyph_string_face(s) != nil && face_box(glyph_string_face(s)) != FACE_NO_BOX()
       && glyph_left_box_line_p(glyph_string_first_glyph(s)) {
    x = glyph_string_x(s) + max(face_box_vertical_line_width(glyph_string_face(s)), 0)
  } else {
    x = glyph_string_x(s)
  }

  char2b.withUnsafeMutableBufferPointer { char2b_buf in
    glyph_string_set_char2b(s, char2b_buf.baseAddress)

    i = 0
    while i < glyph_string_nchars(s) {
      var buf = [CChar](repeating: 0, count: 8)
      let len = glyph_u_glyphless_len(glyph)

      buf.withUnsafeMutableBufferPointer { bufPtr in
        var str: UnsafeMutablePointer<CChar>? = nil

        if glyph_u_glyphless_method(glyph) == GLYPHLESS_DISPLAY_ACRONYM() {
          if len > 0
               && CHAR_TABLE_P(Vglyphless_char_display)
               && (CHAR_TABLE_EXTRA_SLOTS(XCHAR_TABLE(Vglyphless_char_display)) >= 1) {
            var acronym = !glyph_u_glyphless_for_no_font(glyph)
              ? CHAR_TABLE_REF(Vglyphless_char_display, glyph_u_glyphless_ch(glyph))
              : char_table_extras(XCHAR_TABLE(Vglyphless_char_display), 0)
            if CONSP(acronym) {
              acronym = XCAR(acronym)
            }
            if STRINGP(acronym) {
              str = SSDATA(acronym)
            }
          }
        } else if glyph_u_glyphless_method(glyph) == GLYPHLESS_DISPLAY_HEX_CODE() {
          let ch = UInt32(truncatingIfNeeded: glyph_u_glyphless_ch(glyph))
          let digits = Array(String(ch, radix: 16, uppercase: true).utf8)
          let width = ch < 0x10000 ? 4 : 6
          /* C's `%0*X` pads to WIDTH but never truncates a longer number, and the 25-bit
             field can hold seven digits; the buffer's eight bytes still bound the write.  */
          let pad = max(width - digits.count, 0)

          for (idx, d) in digits.enumerated() {
            bufPtr[pad + idx] = CChar(bitPattern: d)
          }
          str = bufPtr.baseAddress
        }

        if str != nil {
          let upper_len = (len + 1) / 2

          /* It is assured that all LEN characters in STR is ASCII.  */
          j = 0
          while j < len {
            let font = glyph_string_font(s)
            if let encode_char = font_driver_encode_char(font_driver(font)) {
              let code = encode_char(font, Int32(str![Int(j)]))
              char2b_buf[Int(j)] = code & 0xFFFF
            }
            j += 1
          }

          let font = glyph_string_font(s)
          if let draw = font_driver_draw(font_driver(font)) {
            _ = draw(s, 0, upper_len,
                     x + Int32(glyph_slice_glyphless_upper_xoff(glyph)),
                     glyph_string_ybase(s) + Int32(glyph_slice_glyphless_upper_yoff(glyph)),
                     false)
            _ = draw(s, upper_len, len,
                     x + Int32(glyph_slice_glyphless_lower_xoff(glyph)),
                     glyph_string_ybase(s) + Int32(glyph_slice_glyphless_lower_yoff(glyph)),
                     false)
          }
        }
      }

      if glyph_u_glyphless_method(glyph) != GLYPHLESS_DISPLAY_THIN_SPACE() {
        let ascent = Int32(glyph_ascent(glyph))
        let descent = Int32(glyph_descent(glyph))
        ns_draw_box(NSMakeRect(CGFloat(x), CGFloat(glyph_string_ybase(s) - ascent),
                               CGFloat(Int32(glyph_pixel_width(glyph)) - 1),
                               CGFloat(ascent + descent - 1)),
                    1, 1,
                    NSColor.colorWithUnsignedLong(NS_FACE_FOREGROUND(glyph_string_face(s))),
                    true, true)
      }
      x += Int32(glyph_pixel_width(glyph))

      i += 1
      glyph = glyph_advance(glyph, 1)
    }

    /* GCC 12 complains even though nothing ever uses s->char2b after
       this function returns.  */
    glyph_string_set_char2b(s, nil)
  }
}

/// C: the two `enum draw_glyphs_face` members C 4756 and 4761 test that round 16's
/// `DRAW_CURSOR` and round 18's `DRAW_MOUSE_FACE` do not already carry
/// (src/dispextern.h 1350-1351).
@_silgen_name("DRAW_NORMAL_TEXT") func DRAW_NORMAL_TEXT() -> Int32
@_silgen_name("DRAW_INVERSE_VIDEO") func DRAW_INVERSE_VIDEO() -> Int32
/// C: `struct glyph_string`'s `bool_bf stippled_p : 1` (src/dispextern.h 1443) - round 18
/// declared the read; the five-arm write of C 4758-4779 is this `_set_` row.
@_silgen_name("glyph_string_set_stippled_p")
func glyph_string_set_stippled_p(_ s: UnsafeMutableRawPointer?, _ v: Bool)

/// C 4747-4780: `static void ns_set_glyph_string_gc (struct glyph_string *s)` -
/// "Transfer glyph string parameters from S's face to S itself.  Set S->stipple_p as
/// appropriate, taking the draw type into account."
///
/// The five comment-only lines (C 4754 `s->gc = s->face->gc;`, C 4759
/// `x_set_mode_line_face_gc (s);`, C 4764 `x_set_cursor_gc (s);`, C 4769
/// `x_set_mouse_face_gc (s);` and C 4775 `s->gc = s->face->gc;`) are the X port's leftovers
/// and carry no statement in this arm - recorded.  C's final `emacs_abort ()` (C 4779) is
/// the core's own symbol (round 3's row).
fileprivate func ns_set_glyph_string_gc(_ s: UnsafeMutableRawPointer?) {
  prepare_face_for_display(glyph_string_f(s), glyph_string_face(s))

  if glyph_string_hl(s) == DRAW_NORMAL_TEXT() {
    /* s->gc = s->face->gc; */
    glyph_string_set_stippled_p(s, face_stipple(glyph_string_face(s)) != 0)
  } else if glyph_string_hl(s) == DRAW_INVERSE_VIDEO() {
    /* x_set_mode_line_face_gc (s); */
    glyph_string_set_stippled_p(s, face_stipple(glyph_string_face(s)) != 0)
  } else if glyph_string_hl(s) == DRAW_CURSOR() {
    /* x_set_cursor_gc (s); */
    glyph_string_set_stippled_p(s, false)
  } else if glyph_string_hl(s) == DRAW_MOUSE_FACE() {
    /* x_set_mouse_face_gc (s); */
    glyph_string_set_stippled_p(s, face_stipple(glyph_string_face(s)) != 0)
  } else if glyph_string_hl(s) == DRAW_IMAGE_RAISED()
              || glyph_string_hl(s) == DRAW_IMAGE_SUNKEN() {
    /* s->gc = s->face->gc; */
    glyph_string_set_stippled_p(s, face_stipple(glyph_string_face(s)) != 0)
  } else {
    emacs_abort()
  }
}

/// C: `struct face`'s `struct font *font;` (src/dispextern.h 1828's `font` member) - the
/// `s->face->font` read at C 4786 - and the `FRAME_NS_VIEW` macro's shape,
/// `#define FRAME_FONT(f) ((f)->output_data.ns->font)` (src/nsterm.h 1049) - the fallback
/// C 4787 reads - plus `#define FRAME_FOREGROUND_COLOR(f)
/// ((f)->output_data.ns->foreground_color)` (src/nsterm.h 1037) - the non-zero-foreground
/// arm's value at C 4849.
@_silgen_name("face_font")
func face_font(_ face: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("FRAME_FONT")
func FRAME_FONT(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("FRAME_FOREGROUND_COLOR")
func FRAME_FOREGROUND_COLOR(_ f: UnsafeMutableRawPointer?) -> NSColor?

/// C: `struct glyph_string`'s `int left_overhang` (src/dispextern.h 1384) and
/// `int right_overhang` (1389) - the two **reads** round 14's note left open (that slice
/// wrote them at C 3070-3077 and declared only the `_set_` pair); C 4837 and C 4967 read
/// the left one here, and C 4797, 4803, 4838 and 4926 the right one.
@_silgen_name("glyph_string_left_overhang")
func glyph_string_left_overhang(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_right_overhang")
func glyph_string_right_overhang(_ s: UnsafeMutableRawPointer?) -> Int32
/// C: `enum glyph_type`'s remaining four members - `GLYPHLESS_GLYPH` (src/dispextern.h
/// 440), `IMAGE_GLYPH` (443), `STRETCH_GLYPH` (446) and `XWIDGET_GLYPH` (449) - the
/// dispatcher's `switch` at C 4843 tests all four beside round 14's `CHAR_GLYPH` and
/// `COMPOSITE_GLYPH`.
@_silgen_name("IMAGE_GLYPH") func IMAGE_GLYPH() -> Int32
@_silgen_name("STRETCH_GLYPH") func STRETCH_GLYPH() -> Int32
@_silgen_name("XWIDGET_GLYPH") func XWIDGET_GLYPH() -> Int32
@_silgen_name("GLYPHLESS_GLYPH") func GLYPHLESS_GLYPH() -> Int32
/// C: `struct glyph_string`'s `struct glyph_string *clip_head, *clip_tail;`
/// (src/dispextern.h 1494), `int num_clips;` (1500) and `enum draw_glyphs_face hl;` (1411).
/// Round 14 declared the `hl` *read* (`glyph_string_hl`); the five writes below - C 4932's
/// `prev->hl = s->hl`, C 4955's `prev->hl = save`, C 4973's `next->hl = s->hl`, C 4995's
/// `next->hl = save` and C 4996's `next->clip_head = s->next` - and the two `num_clips`
/// writes (C 4815 and C 5000) are the `_set_` rows.
@_silgen_name("glyph_string_clip_head")
func glyph_string_clip_head(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_set_clip_head")
func glyph_string_set_clip_head(_ s: UnsafeMutableRawPointer?,
                                _ v: UnsafeMutableRawPointer?)
@_silgen_name("glyph_string_clip_tail")
func glyph_string_clip_tail(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_set_num_clips")
func glyph_string_set_num_clips(_ s: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("glyph_string_set_hl")
func glyph_string_set_hl(_ s: UnsafeMutableRawPointer?, _ v: Int32)

/// C: `void get_glyph_string_clip_rect (struct glyph_string *, NativeRectangle *nr)`
/// (declared src/dispextern.h 3663, right after round 16's `get_glyph_string_clip_rects`)
/// - "Wrapper utility …"; defined at src/xdisp.c 2695, inside that file's glyph-clipping
/// block.  The frozen configuration has no symbol for it (measured), so it carries the same
/// gate as its plural sibling: this unit's `ns_get_glyph_string_clip_rect` (round 17) is the
/// two-rect wrapper and this is the one-rect core call C 4935 and C 4976 make.
@_silgen_name("get_glyph_string_clip_rect")
func get_glyph_string_clip_rect(_ s: UnsafeMutableRawPointer?,
                                _ nr: UnsafeMutablePointer<NSRect>?)
/// C: `void x_draw_xwidget_glyph_string (struct glyph_string *)` - declared src/xwidget.h
/// 188 (the `HAVE_XWIDGETS` arm) and defined src/xwidget.c 2793; the header's `#else` arm
/// (236) is the `INLINE { eassume (0); }` stub.  C 4850-4851's `case XWIDGET_GLYPH` calls it,
/// and neither object exists in the frozen build (`src/xwidget.c` is not among
/// `docs/baseline/frozen-build-sources.txt`'s files, measured), so this is a shim work
/// order - the `image_ascent` row of round 19.
@_silgen_name("x_draw_xwidget_glyph_string")
func x_draw_xwidget_glyph_string(_ s: UnsafeMutableRawPointer?)

/// C 4782-5001: `static void ns_draw_glyph_string (struct glyph_string *s)` - the RIF's
/// "Main draw-text call" and the Glyph drawing operations section's last definition.
///
/// Readings this body makes:
///
/// * `NSRect r[2]` is a two-element `[NSRect]` - `get_glyph_string_clip_rects` "should be
///   pointer to two successive NSRects", so the two slots are the reference's - and both
///   entries that take it want the element pointer, which is `&r` (the `&unichars` reading
///   `macfont.swift` 2261-2262 uses).
/// * C's `char box_drawn_p = 0;` (C 4791) is `Bool`, and C 4830's `box_drawn_p = 1` is
///   `true`.
/// * `struct font *font = s->face->font;` (C 4792) with `if (! font) font = FRAME_FONT
///   (s->f);` (C 4793) is the member entry plus the macro entry with C's nil fallback.
/// * the right-overhang loop's comma-headed `for (width = 0, next = s->next; …;
///   width += next->width, next = next->next)` (C 4802-4804) is the `while` below, C's two
///   clauses as the statements they are; C's body mixes `next` and `s->next` (C 4805-4816)
///   and the port keeps both readings - on the first turn they are the same pointer, and
///   the reference's own text uses `s->next` in the rest of the body.
/// * `!s->clip_head` and `!s->clip_tail` are `== nil` on the pointer getters, and
///   `NSIntersectionRect` is Foundation's own (round 13's spelling of it).
/// * the two overhang loops' `NSRect r = NSMakeRect (…)` (C 4933 and C 4974) shadows the
///   outer array's name exactly as C's does, and their `NSRect rc;` with
///   `get_glyph_string_clip_rect (s, &rc);` (C 4934-4935 and C 4975-4976) is the zero
///   default plus the call.  The first loop guards its second clip with `if (n)` (C 4938),
///   the second does not (C 4979) - the reference's own asymmetry, transcribed.
/// * the four `NS_IMPL_GNUSTEP` arms (`DPSgsave` with two `DPSrectclip`s at C 4941-4945 and
///   C 4981-4985, `DPSgrestore` at C 4952 and C 4992) are dead - recorded; the live
///   `saveGraphicsState`/`restoreGraphicsState` pair stays.
/// * the `CHAR_GLYPH`/`COMPOSITE_GLYPH` arm's two extra C block scopes (the case's own at
///   C 4860 and the `NSColor *col` one at C 4875) are the plain declarations below: C's
///   names do not collide there, so no scope is needed, and C's `break`s are Swift's arm
///   ends (round 16's `switch` reading).
/// * `case XWIDGET_GLYPH:` (C 4850-4851) calls the entry above.
fileprivate func ns_draw_glyph_string(_ s: UnsafeMutableRawPointer?) {
  /* TODO (optimize): focus for box and contents draw */
  var r = [NSRect](repeating: NSRect.zero, count: 2)
  var n: Int32 = 0
  var box_drawn_p = false
  var font = face_font(glyph_string_face(s))
  if font == nil { font = FRAME_FONT(glyph_string_f(s)) }

  // C 4795: `NSTRACE ("ns_draw_glyph_string (hl = %u)", s->hl);` - empty at `NSTRACE_ENABLED`
  // 0, so neither its group nor its format argument is evaluated.

  if glyph_string_next(s) != nil && glyph_string_right_overhang(s) != 0
       && glyph_string_for_overlaps(s) == 0 {
    var width: Int32 = 0
    var next = glyph_string_next(s)

    while next != nil && width < glyph_string_right_overhang(s) {
      if glyph_type(glyph_string_first_glyph(next)) != IMAGE_GLYPH() {
        ns_set_glyph_string_gc(next)
        n = ns_get_glyph_string_clip_rect(glyph_string_next(s), &r)
        ns_focus(glyph_string_f(s), r, n)
        if glyph_type(glyph_string_first_glyph(next)) != STRETCH_GLYPH() {
          ns_maybe_dumpglyphs_background(glyph_string_next(s), true)
        } else {
          ns_draw_stretch_glyph_string(glyph_string_next(s))
        }
        ns_unfocus(glyph_string_f(s))
        glyph_string_set_num_clips(next, 0)
      }
      width += glyph_string_width(next)
      next = glyph_string_next(next)
    }
  }

  ns_set_glyph_string_gc(s)

  if glyph_string_for_overlaps(s) == 0 && face_box(glyph_string_face(s)) != FACE_NO_BOX()
       && (glyph_type(glyph_string_first_glyph(s)) == CHAR_GLYPH()
           || glyph_type(glyph_string_first_glyph(s)) == COMPOSITE_GLYPH()) {
    n = ns_get_glyph_string_clip_rect(s, &r)
    ns_focus(glyph_string_f(s), r, n)
    ns_maybe_dumpglyphs_background(s, true)
    ns_dumpglyphs_box_or_relief(s)
    ns_unfocus(glyph_string_f(s))
    box_drawn_p = true
  }

  n = ns_get_glyph_string_clip_rect(s, &r)

  if glyph_string_clip_head(s) == nil /* draw_glyphs didn't specify a clip mask. */
       && glyph_string_clip_tail(s) == nil
       && ((glyph_string_prev(s) != nil
            && glyph_string_hl(glyph_string_prev(s)) != glyph_string_hl(s)
            && glyph_string_left_overhang(s) != 0)
           || (glyph_string_next(s) != nil
               && glyph_string_hl(glyph_string_next(s)) != glyph_string_hl(s)
               && glyph_string_right_overhang(s) != 0)) {
    r[0] = NSIntersectionRect(r[0],
                              NSMakeRect(CGFloat(glyph_string_x(s)), CGFloat(glyph_string_y(s)),
                                         CGFloat(glyph_string_width(s)),
                                         CGFloat(glyph_string_height(s))))
  }

  ns_focus(glyph_string_f(s), r, n)

  switch glyph_type(glyph_string_first_glyph(s)) {
  case IMAGE_GLYPH():
    ns_dumpglyphs_image(s, r[0])
  case XWIDGET_GLYPH():
    x_draw_xwidget_glyph_string(s)
  case STRETCH_GLYPH():
    ns_draw_stretch_glyph_string(s)
  case CHAR_GLYPH(), COMPOSITE_GLYPH():
    let isComposite = glyph_type(glyph_string_first_glyph(s)) == COMPOSITE_GLYPH()
    if glyph_string_for_overlaps(s) != 0
         || (isComposite
             && (glyph_string_cmp_from(s) > 0
                 && !glyph_u_cmp_automatic(glyph_string_first_glyph(s)))) {
      glyph_string_set_background_filled_p(s, true)
    } else {
      ns_maybe_dumpglyphs_background(s,
                                     glyph_type(glyph_string_first_glyph(s)) == COMPOSITE_GLYPH())
    }

    if isComposite {
      ns_draw_composite_glyph_string_foreground(s)
    } else {
      ns_draw_glyph_string_foreground(s)
    }

    let col: NSColor? = (NS_FACE_FOREGROUND(glyph_string_face(s)) != 0
      ? NSColor.colorWithUnsignedLong(NS_FACE_FOREGROUND(glyph_string_face(s)))
      : FRAME_FOREGROUND_COLOR(glyph_string_f(s)))

    /* Draw underline, overline, strike-through.  */
    ns_draw_text_decoration(s, glyph_string_face(s), col, CGFloat(glyph_string_width(s)),
                            CGFloat(glyph_string_x(s)))
  case GLYPHLESS_GLYPH():
    if glyph_string_for_overlaps(s) != 0
         || (glyph_string_cmp_from(s) > 0
             && !glyph_u_cmp_automatic(glyph_string_first_glyph(s))) {
      glyph_string_set_background_filled_p(s, true)
    } else {
      ns_maybe_dumpglyphs_background(s,
                                     glyph_type(glyph_string_first_glyph(s)) == COMPOSITE_GLYPH())
    }
    ns_draw_glyphless_glyph_string_foreground(s)
  default:
    // C's switch covers every `enum glyph_type`; Swift needs the arm for an `Int32` value.
    emacs_abort()
  }

  /* Draw box if not done already.  */
  if glyph_string_for_overlaps(s) == 0 && !box_drawn_p
       && face_box(glyph_string_face(s)) != FACE_NO_BOX() {
    ns_dumpglyphs_box_or_relief(s)
  }

  if face_box(glyph_string_face(s)) != FACE_NO_BOX()
       && glyph_type(glyph_string_first_glyph(s)) == STRETCH_GLYPH() {
    let fg_color = NSColor.colorWithUnsignedLong(NS_FACE_FOREGROUND(glyph_string_face(s)))

    ns_draw_text_decoration(s, glyph_string_face(s), fg_color,
                            CGFloat(glyph_string_background_width(s)),
                            CGFloat(glyph_string_x(s)))
  }

  ns_unfocus(glyph_string_f(s))

  /* Draw surrounding overhangs.  */
  if glyph_string_prev(s) != nil {
    ns_focus(glyph_string_f(s), nil, 0)
    var prev = glyph_string_prev(s)

    while prev != nil {
      if glyph_string_hl(prev) != glyph_string_hl(s)
           && glyph_string_x(prev) + glyph_string_width(prev)
                + glyph_string_right_overhang(prev) > glyph_string_x(s) {
        /* As prev was drawn while clipped to its own area, we
           must draw the right_overhang part using s->hl now.  */
        let save = glyph_string_hl(prev)

        glyph_string_set_hl(prev, glyph_string_hl(s))
        let r = NSMakeRect(CGFloat(glyph_string_x(s)), CGFloat(glyph_string_y(s)),
                           CGFloat(glyph_string_width(s)), CGFloat(glyph_string_height(s)))
        var rc = NSRect.zero
        get_glyph_string_clip_rect(s, &rc)
        NSGraphicsContext.current?.saveGraphicsState()
        NSRectClip(r)
        if n != 0 {
          NSRectClip(rc)
        }
        if glyph_type(glyph_string_first_glyph(prev)) == CHAR_GLYPH() {
          ns_draw_glyph_string_foreground(prev)
        } else {
          ns_draw_composite_glyph_string_foreground(prev)
        }
        NSGraphicsContext.current?.restoreGraphicsState()
        glyph_string_set_hl(prev, save)
      }
      prev = glyph_string_prev(prev)
    }
    ns_unfocus(glyph_string_f(s))
  }

  if glyph_string_next(s) != nil {
    ns_focus(glyph_string_f(s), nil, 0)
    var next = glyph_string_next(s)

    while next != nil {
      if glyph_string_hl(next) != glyph_string_hl(s)
           && glyph_string_x(next) - glyph_string_left_overhang(next)
                < glyph_string_x(s) + glyph_string_width(s) {
        /* As next will be drawn while clipped to its own area,
           we must draw the left_overhang part using s->hl now.  */
        let save = glyph_string_hl(next)

        glyph_string_set_hl(next, glyph_string_hl(s))
        let r = NSMakeRect(CGFloat(glyph_string_x(s)), CGFloat(glyph_string_y(s)),
                           CGFloat(glyph_string_width(s)), CGFloat(glyph_string_height(s)))
        var rc = NSRect.zero
        get_glyph_string_clip_rect(s, &rc)
        NSGraphicsContext.current?.saveGraphicsState()
        NSRectClip(r)
        NSRectClip(rc)
        if glyph_type(glyph_string_first_glyph(next)) == CHAR_GLYPH() {
          ns_draw_glyph_string_foreground(next)
        } else {
          ns_draw_composite_glyph_string_foreground(next)
        }
        NSGraphicsContext.current?.restoreGraphicsState()
        glyph_string_set_hl(next, save)
        glyph_string_set_clip_head(next, glyph_string_next(s))
      }
      next = glyph_string_next(next)
    }
    ns_unfocus(glyph_string_f(s))
  }
  glyph_string_set_num_clips(s, 0)
}

// ---------------------------------------------------------------------------
// Round 21: `EmacsApp` (C 6225-6400) - the class C 5013's `ns_send_appdefined` and the two
// held `EV_TRAILER` macros wait for
// ---------------------------------------------------------------------------

/// `src/nsterm.h` 1189/1192: `#define NSAPP_DATA2_RUNASSCRIPT 10` and
/// `#define NSAPP_DATA2_RUNFILEDIALOG 11` - the two `data2` values `EmacsApp`'s `sendEvent:`
/// switches on (C 6344 and C 6349).  `NSEvent`'s `data2` is `Int` in Swift, so these are `let`s
/// of that type (the `NS_FUNCTION_KEY_MASK` shape the round-1 block records).
let NSAPP_DATA2_RUNASSCRIPT = 10
let NSAPP_DATA2_RUNFILEDIALOG = 11

/// C: `char *strstr (const char *, const char *)` (string.h) - the `fputs` shape above; Swift
/// marks the C string family unavailable.  C 6311-6312 is the one call site.
@_silgen_name("strstr")
func strstr(_ haystack: UnsafePointer<CChar>,
            _ needle: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

/// The `struct ns_display_info` walk C 6389 does
/// (`for (di = x_display_list; ! has_focus && di; di = di->next)`).  The record's definition is
/// Rust's `src/include/nsterm.rs` 326, so the Swift side keeps C's pointer depth with the
/// untyped stand-in (P-55) and reaches the fields through shims - `ns_display_info_ns_focus_frame`
/// above is the read half this one pairs with.
@_silgen_name("ns_display_info_next")
func ns_display_info_next(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?

/// C: `extern void ns_run_ascript (void);` (`src/nsterm.h` 1190) and
/// `extern void ns_run_file_dialog (void);` (1193) - the two cross-unit entries C 6345/6350
/// call.  Both are defined in `femacs/platform/macos/nsfns.swift` (3362 and 2716, `@_cdecl`)
/// over `src/nsfns.m` 2447 and 1726, so this unit declares the import it reads
/// (`emacs_view_frame`'s cross-unit shape above) and links the C symbol the other unit exports.
@_silgen_name("ns_run_ascript") func ns_run_ascript()
@_silgen_name("ns_run_file_dialog") func ns_run_file_dialog()

/// C 6225-6912: `@implementation EmacsApp` - the eight-class section's third entry, the object
/// `NSApp` is, and the class the held `EV_TRAILER`/`EV_TRAILER2` macros and C 5013's
/// `ns_send_appdefined` both need (`app->nextappdefined` at C 5026, `-sendFromMainThread:` at
/// C 6778).
///
/// **Round 21 lands C 6225-6400**: the record with its `NS_IMPL_COCOA` ivars (`src/nsterm.h`
/// 378-389) and the five methods that close over names this unit already has - `init`
/// (C 6227-6254), `run` (C 6256-6293), `stop:` (C 6295-6303), `logNotification:` (C 6306-6314)
/// and `sendEvent:` (C 6317-6399).  Its remaining twenty-five methods (C 6402-6912) stay open in
/// the ledger; C 5005-6224's event-loop functions land after this class because they call it (the
/// entry/definition split the ledger's `## ABI` (4) records).
class EmacsApp: NSApplication {
  /// `src/nsterm.h` 381-382, the `NS_IMPL_COCOA` arm: `BOOL shouldKeepRunning; BOOL isFirst;`.
  var shouldKeepRunning: Bool = false
  var isFirst: Bool = false
  /// `src/nsterm.h` 388: `@public int nextappdefined;` - `@public` because C 5026 and C 6780 reach
  /// it through an `EmacsApp *` cast, so the port keeps it a plain property.
  var nextappdefined: Int32 = 0

  /// C 6227-6254: `- (id)init`.  The `NS_IMPL_GNUSTEP` half - the
  /// `applicationDidFinishLaunchingCalled` ivar (C 6241) and the notification-center observer
  /// (C 6245-6251) - is the arm this target does not take, so C 6238's `self->isFirst = YES;` is
  /// what remains; C's `if ((self = [super init]))` is Swift's `super.init ()`.
  override init() {
    super.init()
    isFirst = true
  }

  /// Swift's requirement on an `NSApplication` subclass; C has no unarchiving path for this class
  /// and never builds one (the `required` override `EmacsBell` records above).
  required init?(coder: NSCoder) {
    super.init(coder: coder)
  }

  /// C 6256-6293: `- (void)run`.  C 6261-6263's `#ifndef NSAppKitVersionNumber10_9` fallback
  /// names the vintage constant, which Swift spells `NSAppKitVersion.macOS10_9`
  /// (`NSAppKitVersionNumber10_10`'s reading above; C 6265's `(int) NSAppKitVersionNumber` is the
  /// `rawValue` comparison), and `NSAutoreleasePool` is the `objc_autoreleasePoolPush`/`Pop` pair
  /// this unit declares at C 560.
  override func run() {
    if NSAppKitVersion.current.rawValue != NSAppKitVersion.macOS10_9.rawValue {
      super.run()
      return
    }

    var pool = objc_autoreleasePoolPush()

    if isFirst { finishLaunching() }                  // C 6273
    isFirst = false                                   // C 6274

    shouldKeepRunning = true                          // C 6276
    repeat {
      objc_autoreleasePoolPop(pool)                   // C 6279
      pool = objc_autoreleasePoolPush()               // C 6280

      let event = nextEvent(matching: .any,           // C 6282-6286
                            until: Date.distantFuture,
                            inMode: .default,
                            dequeue: true)

      /* C hands the pointer on unchanged: `nextEventMatchingMask:` returns nil only when
         `untilDate` expires, and C 6284 passes `distantFuture`, so the unwrap cannot fire.  */
      sendEvent(event!)                               // C 6288
      updateWindows()                                 // C 6289
    } while shouldKeepRunning                         // C 6290

    objc_autoreleasePoolPop(pool)                     // C 6292
  }

  /// C 6295-6303: `- (void)stop: (id)sender` (the `NS_IMPL_COCOA` arm; C 6304 closes it).
  override func stop(_ sender: Any?) {
    shouldKeepRunning = false                         // C 6299
    super.stop(sender)                                // C 6302
  }

  /// C 6306-6314: `- (void)logNotification: (NSNotification *)notification`.  C's
  /// `const char *name = [[notification name] UTF8String];` is a UTF-8 view over the
  /// notification's name, which Swift spells with `withCString` so the three `strstr` calls read
  /// the same bytes.  `@objc` is what makes the selector reachable (the GNUstep observer that
  /// names it at C 6248 is the arm this target does not take).
  @objc func logNotification(_ notification: Notification) {
    let name = notification.name.rawValue
    name.withCString { p in
      if strstr(p, "Update") == nil && strstr(p, "NSMenu") == nil
           && strstr(p, "WindowNumber") == nil {
        NSLog("notification: '%@'", name)
      }
    }
  }

  /// C 6317-6399: `- (void)sendEvent: (NSEvent *)theEvent`.  C's `[theEvent type]` is Swift's
  /// `NSEvent.EventType` and C's `NSEventTypeXxx` constants are that enum's cases; the
  /// `NS_IMPL_GNUSTEP` keyboard-forwarding arm (C 6329-6337) is dead here.
  override func sendEvent(_ theEvent: NSEvent) {
    let type = theEvent.type                          // C 6323
    let window = theEvent.window                      // C 6324

    if type == .applicationDefined {                  // C 6339
      switch theEvent.data2 {                         // C 6341
      case NSAPP_DATA2_RUNASSCRIPT:                   // C 6344 (the `NS_IMPL_COCOA` case)
        ns_run_ascript()
        stop(self)                                    // C 6346
        return
      case NSAPP_DATA2_RUNFILEDIALOG:                 // C 6349
        ns_run_file_dialog()
        stop(self)                                    // C 6351
        return
      default:
        break
      }
    }

    if type == .cursorUpdate && window == nil {       // C 6356
      fputs("Dropping external cursor update event.\n", stderr)   // C 6358
      return
    }

    if type == .applicationDefined {                  // C 6362
      /* C 6364-6367: "Events posted by ns_send_appdefined interrupt the run loop here.  But, if
         a modal window is up, an appdefined can still come through ... Just defer it until
         later."  */
      if NSApp.modalWindow == nil {                   // C 6368
        last_appdefined_event_data = Int32(theEvent.data1)        // C 6370
        stop(self)                                    // C 6371
      } else {
        send_appdefined = true                        // C 6375
      }
    }

    /* C 6380-6394, the `NS_IMPL_COCOA` arm: skip a mouse move that belongs to an auxiliary menu
       (the top-right one) when none of our frames has focus.  C's `for` loop is the `while`
       below, which keeps C's two conjuncts and its short-circuit.  */
    if type == .mouseMoved && NSApp.modalWindow == nil {          // C 6385
      var has_focus = false                           // C 6388
      var di = x_display_list                         // C 6389
      while !has_focus && di != nil {
        has_focus = ns_display_info_ns_focus_frame(di) != nil
        di = ns_display_info_next(di)
      }
      if !has_focus { return }                        // C 6391
    }

    super.sendEvent(theEvent)                         // C 6398
  }

  /// C 6769-6776: `- (void)timeout_handler: (NSTimer *)timedEntry` - "The timeout specified to
  /// ns_select has passed."  It lands here with round 22 because its one statement calls the
  /// function this round adds (the class's other methods stay open).
  @objc func timeout_handler(_ timedEntry: Timer) {
    ns_send_appdefined(-2)                            // C 6775
  }

  /// C 6778-6781: `- (void)sendFromMainThread:(id)unused` - the main-thread half of
  /// `ns_send_appdefined`'s non-main-thread arm (C 5027 posts this very selector), which is why
  /// the two land together.
  @objc func sendFromMainThread(_ unused: Any?) {
    ns_send_appdefined(nextappdefined)                // C 6780
  }

  /// C 6556-6561: `- (void)antialiasThresholdDidChange: (NSNotification *)notification`.  Its
  /// one statement calls the Mac font lane's entry, which that unit already exports
  /// (`macfont.swift` 1376 over `src/macfont.m` 747).
  @objc func antialiasThresholdDidChange(_ notification: Notification) {
    macfont_update_antialias_threshold()              // C 6559
  }

  /// C 6565-6575: `- (void)systemWillSleep: (NSNotification *)notification`.  C 6569's
  /// uninitialized `struct input_event ie` is the state C 6570's `EVENT_INIT` writes (its compound
  /// literal sets `device` and zeroes the rest), so the all-zero initializer below is that same
  /// state, not an invented one.
  @objc func systemWillSleep(_ notification: Notification) {
    var ie = input_event(bits: 0, code: 0, modifiers: 0, x: nil, y: nil, timestamp: 0,
                         frame_or_window: nil, arg: nil, device: nil)      // C 6569
    EVENT_INIT(&ie)                                   // C 6570
    input_event_set_kind(&ie, SLEEP_EVENT)            // C 6571
    ie.arg = list1(Qpre_sleep())                      // C 6572
    kbd_buffer_store_event(&ie)                       // C 6573
  }

  /// C 6577-6587: `- (void)systemDidWake: (NSNotification *)notification` - `Qpre_sleep`'s twin
  /// with `Qpost_wake` (C 6584).
  @objc func systemDidWake(_ notification: Notification) {
    var ie = input_event(bits: 0, code: 0, modifiers: 0, x: nil, y: nil, timestamp: 0,
                         frame_or_window: nil, arg: nil, device: nil)      // C 6581
    EVENT_INIT(&ie)                                   // C 6582
    input_event_set_kind(&ie, SLEEP_EVENT)            // C 6583
    ie.arg = list1(Qpost_wake())                      // C 6584
    kbd_buffer_store_event(&ie)                       // C 6585
  }

  /// C 6610-6613: `- (BOOL)applicationSupportsSecureRestorableState: (NSApplication *)app` -
  /// C's `return YES;` and nothing else.
  @objc func applicationSupportsSecureRestorableState(_ app: NSApplication) -> Bool {
    true                                              // C 6612
  }

  /// C 6655-6669: `- (NSApplicationTerminateReply)applicationShouldTerminate: (id)sender`.
  @objc func applicationShouldTerminate(_ sender: Any?) -> NSApplication.TerminateReply {
    if NILP(ns_confirm_quit) { return .terminateNow } // C 6661-6662
    let ret = runAlertPanel(ns_app_name()! as String, // C 6664
                            "Exit requested.  Would you like to Save Buffers and Exit, or Cancel the request?",
                            "Save Buffers and Exit", "Cancel")  // C 6665-6666
    return ret ? .terminateNow : .terminateCancel     // C 6668
  }

  /// C 6682-6687: `- (BOOL)application: sender openFile: (NSString *)file` - "Notification from
  /// the Workspace to open a file."
  @objc func application(_ sender: NSApplication, openFile file: String) -> Bool {
    if ns_do_open_file || not_in_argv(file) != 0 {    // C 6684
      ns_pending_files?.add(file)                     // C 6685
    }
    return true                                       // C 6686
  }

  /// C 6691-6696: `- (BOOL)application: sender openTempFile: (NSString *)file` - "Open a file as a
  /// temporary file."
  @objc func application(_ sender: NSApplication, openTempFile file: String) -> Bool {
    if ns_do_open_file || not_in_argv(file) != 0 {    // C 6693
      ns_pending_files?.add(file)                     // C 6694
    }
    return true                                       // C 6695
  }

  /// C 6700-6705: `- (BOOL)application: sender openFileWithoutUI: (NSString *)file`.
  @objc func application(_ sender: NSApplication, openFileWithoutUI file: String) -> Bool {
    if ns_do_open_file || not_in_argv(file) != 0 {    // C 6702
      ns_pending_files?.add(file)                     // C 6703
    }
    return true                                       // C 6704
  }

  /// C 6708-6721: `- (void)application: sender openFiles: (NSArray *)fileList`.  C's
  /// `NSEnumerator` walk is the loop below, and its comment about Cocoa's command-line parsing
  /// (C 6712-6714) is why `not_in_argv` gates every element.
  @objc func application(_ sender: NSApplication, openFiles fileList: [String]) {
    for file in fileList {                            // C 6710-6717
      if ns_do_open_file || not_in_argv(file) != 0 {
        ns_pending_files?.add(file)
      }
    }
    reply(toOpenOrPrint: .success)                    // C 6719
  }

  /// C 6725-6728: `- (NSMenu *)applicationDockMenu: (NSApplication *) sender` - C's body is
  /// `return dockMenu;`, and this unit keeps that record as the untyped stand-in above (the
  /// `EmacsMenu` class has not landed), so the pointer is handed back as the menu it points at.
  @objc func applicationDockMenu(_ sender: NSApplication) -> NSMenu? {
    dockMenu.map { Unmanaged<NSMenu>.fromOpaque($0).takeUnretainedValue() }
  }

  /// C 6732-6736: `- (void)applicationWillBecomeActive: (NSNotification *)notification` - C's body
  /// is the commented-out `ns_app_active` line and nothing else.
  @objc func applicationWillBecomeActive(_ notification: Notification) {
  }

  /// C 6738-6751: `- (void)applicationDidBecomeActive: (NSNotification *)notification`.  C's
  /// `NS_IMPL_GNUSTEP` arm (C 6742-6745) is dead here, and its `// ns_app_active=YES;` is the
  /// comment C 6746 carries.
  @objc func applicationDidBecomeActive(_ notification: Notification) {
    ns_update_auto_hide_menu_bar()                    // C 6748
    /* C 6749: "No constraining takes place when the application is not active."  */
    ns_constrain_all_frames()                         // C 6750
  }

  /// C 6752-6758: `- (void)applicationDidResignActive: (NSNotification *)notification`.
  @objc func applicationDidResignActive(_ notification: Notification) {
    ns_send_appdefined(-1)                            // C 6757
  }

  /// C 6881-6887: `- (void)requestService: (NSPasteboard *)pboard userData: (NSString *)userData
  /// error: (NSString **)error` - "Called from system: queue for next pass through event loop."
  /// The queue `ns_read_socket_1` drains is filled here, and C 6886's
  /// `[NSString stringWithLispString: …]` is the selection lane's exported factory.
  @objc func requestService(_ pboard: NSPasteboard, userData: String,
                            error: AutoreleasingUnsafeMutablePointer<NSString?>) {
    ns_pending_service_names?.add(userData)                                 // C 6885
    ns_pending_service_args?.add(stringWithLispString(ns_string_from_pasteboard(pboard))!)  // C 6886
  }

  /// C 6490-6554: `- (void)applicationDidFinishLaunching: (NSNotification *)notification` -
  /// "When application is loaded, terminate event loop in ns_term_init."  The GNUstep arm
  /// (C 6497-6499) is dead here; the three dummy CoreFoundation/Foundation calls (C 6531-6533)
  /// raise the process's open-file limit before C 6535-6539 lowers it to `FD_SETSIZE`, "since
  /// pselect cannot handle file descriptors that are greater than or equal to FD_SETSIZE".
  @objc func applicationDidFinishLaunching(_ notification: Notification) {
    NSApp.servicesProvider = NSApp                    // C 6500
    antialiasThresholdDidChange(notification)         // C 6502

    /* C 6504-6508: observe the antialias-threshold notification.  C's constant is
       `NSAntialiasThresholdChangedNotification`; the compiler names Swift's spelling
       (`'NSAntialiasThresholdChangedNotification' has been renamed to
       'NSFont.antialiasThresholdChangedNotification'`, measured - class B, no symbol invented).  */
    NotificationCenter.default.addObserver(
      self,
      selector: #selector(EmacsApp.antialiasThresholdDidChange(_:)),
      name: NSFont.antialiasThresholdChangedNotification,
      object: nil)

    /* C 6511-6523: "Sleep event notification."  */
    NSWorkspace.shared.notificationCenter.addObserver(
      self, selector: #selector(EmacsApp.systemWillSleep(_:)),
      name: NSWorkspace.willSleepNotification, object: nil)
    NSWorkspace.shared.notificationCenter.addObserver(
      self, selector: #selector(EmacsApp.systemDidWake(_:)),
      name: NSWorkspace.didWakeNotification, object: nil)

    /* C 6526-6530: the dummy calls, then the limit itself.  */
    _ = CFSocketGetTypeID()                           // C 6531
    _ = CFFileDescriptorGetTypeID()                   // C 6532
    _ = FileHandle()                                  // C 6533 (Swift's name for NSFileHandle)
    var rlim = rlimit()                               // C 6534
    if getrlimit(RLIMIT_NOFILE, &rlim) == 0 && rlim.rlim_cur > UInt(FD_SETSIZE) {  // C 6535-6536
      rlim.rlim_cur = rlim_t(FD_SETSIZE)              // C 6538
      _ = setrlimit(RLIMIT_NOFILE, &rlim)             // C 6539
    }
    if NSApp.activationPolicy() == .prohibited {      // C 6541
      /* C 6542-6544: "Set the app's activation policy to regular when we run outside of a
         bundle.  This is already done for us by Info.plist when we run inside a bundle."  */
      NSApp.setActivationPolicy(.regular)             // C 6545
      NSApp.applicationIconImage =                    // C 6546-6549
        EmacsImage_allocInitFromFile(build_string("icons/hicolor/128x128/apps/emacs.png")) as? NSImage
    }

    ns_send_appdefined(-2)                            // C 6553
  }

  /// C 6402-6413: `- (void)showPreferencesWindow: (id)sender`.
  @objc func showPreferencesWindow(_ sender: Any?) {
    let emacsframe = SELECTED_FRAME()                 // C 6404
    let theEvent = NSApp.currentEvent                 // C 6405

    if emacs_event == nil {                           // C 6407
      return                                          // C 6408
    }
    input_event_set_kind(emacs_event!, NS_NONKEY_EVENT)          // C 6409
    emacs_event!.pointee.code = UInt32(KEY_NS_SHOW_PREFS)        // C 6410
    emacs_event!.pointee.modifiers = 0                           // C 6411
    EV_TRAILER(theEvent, emacsframe)                             // C 6412
  }

  /// C 6416-6429: `- (void)newFrame: (id)sender` - `showPreferencesWindow:`'s twin with
  /// `KEY_NS_NEW_FRAME`.
  @objc func newFrame(_ sender: Any?) {
    let emacsframe = SELECTED_FRAME()                 // C 6420
    let theEvent = NSApp.currentEvent                 // C 6421

    if emacs_event == nil {                           // C 6423
      return                                          // C 6424
    }
    input_event_set_kind(emacs_event!, NS_NONKEY_EVENT)          // C 6425
    emacs_event!.pointee.code = UInt32(KEY_NS_NEW_FRAME)         // C 6426
    emacs_event!.pointee.modifiers = 0                           // C 6427
    EV_TRAILER(theEvent, emacsframe)                             // C 6428
  }

  /// C 6783-6870: `- (void)fd_handler:(id)unused` - "Check data waiting on file descriptors and
  /// terminate if so."  `waiting` alternates between a blocking probe of the self-pipe and a
  /// `pselect` over the descriptors `ns_select_1` recorded (C 5291-5295's mutex protocol is the
  /// same one), and the loop never returns - it is the thread `ns_term_init` starts.
  @objc func fd_handler(_ unused: Any?) {
    var result: Int32
    var waiting: Int32 = 1                            // C 6789
    var nfds: Int32
    var c: CChar = 0                                  // C 6790
    var readfds = fd_set()                            // C 6792
    var writefds = fd_set()
    var timeout = timespec()
    var pool: UnsafeMutableRawPointer? = nil          // C 6794

    while true {                                      // C 6798
      objc_autoreleasePoolPop(pool)                   // C 6800
      pool = objc_autoreleasePoolPush()               // C 6801

      if waiting != 0 {                               // C 6803
        var fds = fd_set()                            // C 6805
        __darwin_fd_zero(&fds)                        // C 6806
        __darwin_fd_set(selfds.0, &fds)               // C 6807
        result = pselect(selfds.0 + 1, &fds, nil, nil, nil, nil)   // C 6808
        if result > 0 && read(selfds.0, &c, 1) == 1 && c == CChar(UInt8(ascii: "g")) {
          waiting = 0                                 // C 6810
        }
      } else {
        pthread_mutex_lock(&select_mutex)             // C 6814
        nfds = select_nfds                            // C 6815

        if select_valid & Int32(SELECT_HAVE_READ) != 0 {   // C 6817
          readfds = select_readfds                    // C 6818
        } else {
          __darwin_fd_zero(&readfds)                  // C 6820
        }

        if select_valid & Int32(SELECT_HAVE_WRITE) != 0 {  // C 6822
          writefds = select_writefds                  // C 6824
        }
        if select_valid & Int32(SELECT_HAVE_TMO) != 0 {    // C 6829
          timeout = select_timeout                    // C 6831
        }

        pthread_mutex_unlock(&select_mutex)           // C 6837

        let self_fd = selfds.0                        // C 6839's and C 6840's `selfds[0]`
        __darwin_fd_set(self_fd, &readfds)            // C 6839
        if self_fd >= nfds { nfds = self_fd + 1 }     // C 6840

        /* C 6825/6832 put the *addresses* of those two locals into `wfds`/`tmo`, which Swift
           cannot assign (`&` is valid only at a call site).  The two flags therefore select one of
           C's four call shapes here, at the call itself; the mutex, the values and the order are
           C's.  */
        let haveW = select_valid & Int32(SELECT_HAVE_WRITE) != 0
        let haveT = select_valid & Int32(SELECT_HAVE_TMO) != 0
        if haveW {
          if haveT { result = pselect(nfds, &readfds, &writefds, nil, &timeout, nil) }
          else     { result = pselect(nfds, &readfds, &writefds, nil, nil, nil) }
        } else {
          if haveT { result = pselect(nfds, &readfds, nil, nil, &timeout, nil) }
          else     { result = pselect(nfds, &readfds, nil, nil, nil, nil) }
        }

        if result == 0 {                              // C 6844
          ns_send_appdefined(-2)                      // C 6845
        } else if result > 0 {                        // C 6846
          if __darwin_fd_isset(self_fd, &readfds) != 0 {   // C 6848
            if read(self_fd, &c, 1) == 1 && c == CChar(UInt8(ascii: "s")) {
              waiting = 1                             // C 6851
            }
          } else {
            pthread_mutex_lock(&select_mutex)         // C 6855
            if select_valid & Int32(SELECT_HAVE_READ) != 0 { select_readfds = readfds }    // C 6857
            if select_valid & Int32(SELECT_HAVE_WRITE) != 0 { select_writefds = writefds }  // C 6859
            if select_valid & Int32(SELECT_HAVE_TMO) != 0 { select_timeout = timeout }      // C 6861
            pthread_mutex_unlock(&select_mutex)       // C 6862

            ns_send_appdefined(result)                // C 6864
          }
        }
        waiting = 1                                   // C 6867
      }
    }
  }

  /// C 6615-6631: `- (void) terminate: (id)sender`.  It waited for `NS_NONKEY_EVENT`'s value (the
  /// entry above records how this unit takes it); C 6617's uninitialized record is the state
  /// C 6623's `EVENT_INIT` writes, as in `systemWillSleep:`.
  @objc override func terminate(_ sender: Any?) {
    let f = SELECTED_FRAME()                          // C 6622
    var ie = input_event(bits: 0, code: 0, modifiers: 0, x: nil, y: nil, timestamp: 0,
                         frame_or_window: nil, arg: nil, device: nil)    // C 6617
    EVENT_INIT(&ie)                                   // C 6623
    input_event_set_kind(&ie, NS_NONKEY_EVENT)        // C 6625
    ie.code = UInt32(KEY_NS_POWER_OFF)                // C 6626
    ie.arg = Qt() /* mark as non-key event */         // C 6627
    XSETFRAME(&ie.frame_or_window, f)                 // C 6628
    kbd_buffer_store_event(&ie)                       // C 6630
  }

  /// C 6433-6451: `- (BOOL) openFile: (NSString *)fileName` - the event constructor the pending
  /// open-file queue drains into (`ns_read_socket_1` above is its caller).
  @objc func openFile(_ fileName: String) -> Bool {
    let emacsframe = SELECTED_FRAME()                 // C 6437
    let theEvent = NSApp.currentEvent                 // C 6438

    if emacs_event == nil {                           // C 6440
      return false                                    // C 6441
    }

    input_event_set_kind(emacs_event!, NS_NONKEY_EVENT)                 // C 6443
    emacs_event!.pointee.code = UInt32(KEY_NS_OPEN_FILE_LINE)           // C 6444
    ns_input_file = append2(ns_input_file, lispString(fileName as NSString))  // C 6445
    ns_input_line = Qnil() /* can be start or cons start,end */         // C 6446
    emacs_event!.pointee.modifiers = 0                                  // C 6447
    EV_TRAILER(theEvent, emacsframe)                                    // C 6448

    return true                                       // C 6450
  }

  /// C 6891-6909: `- (BOOL)fulfillService: (NSString *)name withArg: (NSString *)arg` - the
  /// service-request twin of `openFile:`, and the second method `ns_read_socket_1` drains into.
  @objc func fulfillService(_ name: String, withArg arg: String) -> Bool {
    let emacsframe = SELECTED_FRAME()                 // C 6893
    let theEvent = NSApp.currentEvent                 // C 6894

    if emacs_event == nil {                           // C 6898
      return false                                    // C 6899
    }

    input_event_set_kind(emacs_event!, NS_NONKEY_EVENT)                 // C 6901
    emacs_event!.pointee.code = UInt32(KEY_NS_SPI_SERVICE_CALL)         // C 6902
    ns_input_spi_name = lispString(name as NSString)                    // C 6903
    ns_input_spi_arg = lispString(arg as NSString)                      // C 6904
    emacs_event!.pointee.modifiers = EV_MODIFIERS(theEvent!)            // C 6905
    EV_TRAILER(theEvent, emacsframe)                                    // C 6906

    return true                                       // C 6908
  }
}

// ==========================================================================
// Round 22: C 448-470's event-trailer pair and C 5012-5084's `ns_send_appdefined`
// ==========================================================================

/// C: `Qt` (src/globals.h) - the `Qnil` import above is the same shape; the two held
/// `EV_TRAILER` macros are this unit's first readers.
@_silgen_name("Qt") func Qt() -> Lisp_Object

/// C: `Lisp_Object Vinhibit_quit` (src/globals.h; the port defines it at `femacs/src/eval.rs`
/// 4275) - imported as the C data symbol the core exports (the `ns_auto_hide_menu_bar` shape
/// above), and the only core global this round writes.
@_silgen_name("Vinhibit_quit") var Vinhibit_quit: Lisp_Object

/// C: `void kbd_buffer_store_event_hold (struct input_event *, struct input_event *)`
/// (src/keyboard.h) - the port defines it at `femacs/src/include/keyboard.rs` 149; the import
/// shape is `fputs`'s above.
@_silgen_name("kbd_buffer_store_event_hold")
func kbd_buffer_store_event_hold(_ event: UnsafeMutablePointer<input_event>?,
                                 _ hold_quit: UnsafeMutablePointer<input_event>?)

/// C: `NO_EVENT` (`src/termhooks.h` 74, the first enumerator of `enum event_kind`).  C reads it
/// through the record's `kind` bitfield; this unit's view of `input_event` keeps the two
/// bitfields packed in one `UInt32` (the struct's own reading above says so), so the field is
/// masked out by the reader below.
fileprivate let NO_EVENT: UInt32 = 0

/// C: `ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH;` - the low 16 bits of `input_event.bits`
/// (`EVENT_KIND_WIDTH` is `src/termhooks.h` 360's 16, the struct's reading above).
fileprivate func input_event_kind(_ e: UnsafeMutablePointer<input_event>) -> UInt32 {
  e.pointee.bits & 0xffff
}

/// C 448-453: `#define EV_TRAILER(e)`.  Its body reads two names from the caller's scope:
/// `emacs_event` (this unit's static) and `emacsframe`, which every handler declares as a local
/// (C 6404 is the first expansion's).  The port passes both - the `EV_MODIFIERS2` shape above.
fileprivate func EV_TRAILER(_ e: NSEvent?, _ emacsframe: UnsafeMutableRawPointer?) {
  if input_event_kind(emacs_event!) != NO_EVENT {                 // C 449
    XSETFRAME(&emacs_event!.pointee.frame_or_window, emacsframe)  // C 451
    EV_TRAILER2(e)                                                // C 452
  }
}

/// C 455-470: `#define EV_TRAILER2(e)`.  C's save/restore of `Vinhibit_quit` around the store
/// is kept as written; `EVENT_INIT` (C 468) and `ns_send_appdefined` (C 469) are this unit's
/// own - the function below is the one the two held macro boxes waited for.
fileprivate func EV_TRAILER2(_ e: NSEvent?) {
  if e != nil { emacs_event!.pointee.timestamp = UInt(EV_TIMESTAMP(e!)) }  // C 457
  if let q = q_event_ptr {                                      // C 458
    let tem = Vinhibit_quit                                     // C 460
    Vinhibit_quit = Qt()                                        // C 461
    n_emacs_events_pending += 1                                 // C 462
    kbd_buffer_store_event_hold(emacs_event, q)                 // C 463
    Vinhibit_quit = tem                                         // C 464
  } else {
    hold_event(emacs_event)                                     // C 467
  }
  EVENT_INIT(emacs_event!)                                      // C 468
  ns_send_appdefined(-1)                                        // C 469
}

/// C 5012-5084: `static void ns_send_appdefined (int value)` - "post an appdefined event which
/// EmacsApp-sendEvent will recognize and take as a command to halt the event loop".  The
/// non-main-thread arm (C 5023-5031) is why `EmacsApp` had to land first: it reaches
/// `nextappdefined` and `-sendFromMainThread:` through an `EmacsApp *` cast, which is the
/// entry/definition split the ledger's `## ABI` (4) records.
fileprivate func ns_send_appdefined(_ value: Int32) {
  // C 5021-5023: GNUstep needs `postEvent` on the main thread, Cocoa needs
  // `nextEventMatchingMask` there too.
  if !Thread.current.isMainThread {
    let app = NSApp as! EmacsApp                                 // C 5025
    app.nextappdefined = value                                   // C 5026
    app.performSelector(onMainThread: #selector(EmacsApp.sendFromMainThread(_:)),
                        with: nil, waitUntilDone: false)         // C 5027-5029
    return
  }

  /* C 5033-5035: "Only post this event if we haven't already posted one."  */
  if send_appdefined {                                           // C 5036
    send_appdefined = false                                      // C 5042

    if timed_entry != nil {                                      // C 5045
      timed_entry!.invalidate()                                  // C 5047
      timed_entry = nil                                          // C 5049 (ARC: C's `release` is the
    }                                                            //         reference being dropped)

    /* C 5052-5057: address the event to a window that actually exists.  */
    var dest = NSApp.mainWindow                                  // C 5058
    if dest == nil { dest = NSApp.keyWindow }                    // C 5059-5060
    if dest == nil {                                             // C 5061
      for cand in NSApp.windows where cand.windowNumber > 0 {    // C 5062-5063
        dest = cand
        break                                                    // C 5066
      }
    }

    let nxev = NSEvent.otherEvent(with: .applicationDefined,     // C 5069-5077
                                  location: NSMakePoint(0, 0),
                                  modifierFlags: [],
                                  timestamp: 0,
                                  windowNumber: dest?.windowNumber ?? 0,
                                  context: NSApp.context,
                                  subtype: 0,
                                  data1: Int(value),
                                  data2: 0)

    /* C 5079-5081: post it so `[NXApp run]` returns after the queued events.  */
    NSApp.postEvent(nxev!, atStart: false)                       // C 5082
  }
}

// ==========================================================================
// Round 23: `EmacsApp`'s four notification methods (C 6556-6613)
// ==========================================================================

/// C: `Lisp_Object Qpre_sleep, Qpost_wake;` (src/globals.h; the port's builtin symbols are
/// `femacs/src/include/globals.rs` 4561 and 4549).  The `Qnil`/`Qt` shape above.
@_silgen_name("Qpre_sleep") func Qpre_sleep() -> Lisp_Object
@_silgen_name("Qpost_wake") func Qpost_wake() -> Lisp_Object

/// C: `void kbd_buffer_store_event (struct input_event *)` (src/keyboard.h) - the port defines it
/// at `femacs/src/include/keyboard.rs` 310; `kbd_buffer_store_event_hold`'s shape above.
@_silgen_name("kbd_buffer_store_event")
func kbd_buffer_store_event(_ event: UnsafeMutablePointer<input_event>?)

/// C: `void macfont_update_antialias_threshold (void)` (src/macfont.h 85, defined src/macfont.m
/// 747) - the Mac font lane exports it (`macfont.swift` 1376), so this unit imports the symbol
/// (`ns_run_ascript`'s cross-unit shape above).
@_silgen_name("macfont_update_antialias_threshold") func macfont_update_antialias_threshold()

/// C: `SLEEP_EVENT`, an `enum event_kind` member (the enum at `src/termhooks.h` 74, its `HAVE_NS`
/// arm); the port's mirror fixes the value at `femacs/src/include/termhooks.rs` 55.  Like
/// `NO_EVENT` above, C writes it into the record's `kind` bitfield.
fileprivate let SLEEP_EVENT: UInt32 = 28

/// C: `ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH;` as a **writer** - the low 16 bits of
/// `input_event.bits`, with the `part` half preserved, which is what C's bitfield assignment does
/// (C 6571's `ie.kind = SLEEP_EVENT`).  The reader is `input_event_kind` above.
fileprivate func input_event_set_kind(_ e: UnsafeMutablePointer<input_event>, _ kind: UInt32) {
  e.pointee.bits = (e.pointee.bits & 0xffff0000) | (kind & 0xffff)
}

// ==========================================================================
// Round 24: `EmacsWindow`'s class record and five of its methods (C 9756)
// ==========================================================================

/// C `src/nsterm.h` 419-430: `@interface EmacsWindow : NSWindow` - the drag-and-drop window whose
/// type `EmacsView` needs for its `nonfs_window` ivar (nsterm.h 482), which is why this record
/// lands before that class's.  C declares **no** protocol here (nsterm.h 419 has no `<...>`
/// list), so the class can land as a record plus the methods below.
///
/// **Round 24 lands** the four Cocoa ivars (nsterm.h 421-424) and five methods:
/// `borderWidth` (declared nsterm.h 442, defined C 9928-9931), `setFrameTopLeftPoint:`
/// (C 10444-10450), `canBecomeKeyWindow` (C 10452-10455),
/// `canBecomeMainWindow` (C 10457-10461) and `setLastDragEvent:` (C 10463-10468).  Two things are
/// deferred by name: the four drag-and-drop ivars (nsterm.h 426-429 - `dnd_mode` is the
/// `ns_return_frame_mode` enum `nsselect.swift` 104 spells, so they land with the drag methods
/// that read them) and `dealloc` (C 9912-9926, whose `release` calls ARC forbids overriding; the
/// reference being dropped is the same statement this lane reads elsewhere).
class EmacsWindow: NSWindow {
  /// C nsterm.h 421: `NSPoint grabOffset;`
  var grabOffset: NSPoint = .zero
  /// C nsterm.h 422: `NSEvent *last_drag_event;`
  var last_drag_event: NSEvent? = nil
  /// C nsterm.h 423-424: `NSDragOperation drag_op; NSDragOperation selected_op;`
  var drag_op: NSDragOperation = []
  var selected_op: NSDragOperation = []

  /// `NSWindow`'s designated initializer, spelled out because a Swift subclass must reach one and
  /// Swift marks `NSWindow`'s `init (coder:)` unavailable (id: `error: 'init(coder:)' is
  /// unavailable`, measured).  C's own creators are `initWithEmacsFrame:` (C 9759/9765), which
  /// land with `origin:`/`styleMask:` below, and they chain to this one.
  override init(contentRect: NSRect, styleMask style: NSWindow.StyleMask,
                backing backingStoreType: NSWindow.BackingStoreType, defer flag: Bool) {
    super.init(contentRect: contentRect, styleMask: style, backing: backingStoreType, defer: flag)
  }

  /// C 10444-10450: `- (void)setFrameTopLeftPoint:(NSPoint)point` - C's body is the `super` call
  /// and nothing else.
  override func setFrameTopLeftPoint(_ point: NSPoint) {
    super.setFrameTopLeftPoint(point)                 // C 10449
  }

  /// C 9928-9931: `- (NSInteger) borderWidth` - C's `NSWidth ([self frame]) - NSWidth ([[self
  /// contentView] frame])`, where `NSWidth` is the rect's own width.
  func borderWidth() -> Int {
    Int(frame.width - (contentView?.frame.width ?? 0))// C 9930
  }

  /// C 10452-10455: `- (BOOL)canBecomeKeyWindow`.  Swift's AppKit apinotes rename the pair below
  /// (`canBecomeKeyWindow` -> `canBecomeKey`, `canBecomeMainWindow` -> `canBecomeMain`), the class
  /// B reading this unit uses for `NSAppKitVersionNumber10_10`; neither renames the reference's
  /// own text, so the C line numbers stay the anchor.  C reaches the view's frame through the
  /// delegate cast (`((EmacsView *)[self delegate])->emacsframe`), which this unit already spells
  /// as the `emacs_view_frame` shim (C 2831), and the predicate is `FRAME_NO_ACCEPT_FOCUS`, whose
  /// entry this unit declares at C 1660.
  override var canBecomeKey: Bool {
    let f = emacs_view_frame(delegate)
    return !frame_no_accept_focus(f)                  // C 10454
  }

  /// C 10457-10461: `- (BOOL)canBecomeMainWindow` - "Required for fullscreen and undecorated
  /// windows"; C's whole body is `return YES;`.  The renamed Swift spelling is `canBecomeMain`
  /// (the note above).
  override var canBecomeMain: Bool {
    true                                              // C 10460
  }

  /// C 10463-10468: `- (void) setLastDragEvent: (NSEvent *) event`.  C's `[event copy]` is
  /// Swift's `copy()`, whose `NSCopying` result is cast back to the record type.
  func setLastDragEvent(_ event: NSEvent) {
    last_drag_event = event.copy() as? NSEvent         // C 10467
  }

  /// C 10043-10055: `- (void)orderFront:(id)sender` - C's parent-window arm removes and re-adds
  /// the receiver above it, and the `else` is the `super` call.
  @objc override func orderFront(_ sender: Any?) {
    let parent = parent                                   // C 10047 (the renamed spelling)
    if parent != nil {                                    // C 10048
      parent!.removeChildWindow(self)                     // C 10050
      parent!.addChildWindow(self, ordered: .above)       // C 10051
    } else {
      super.orderFront(sender)                            // C 10054
    }
  }

  /// C 10057-10068: `- (void)makeKeyAndOrderFront:(id)sender` - the parent arm orders the receiver
  /// front and makes it key; the `else` is the `super` call.
  @objc override func makeKeyAndOrderFront(_ sender: Any?) {
    if parent != nil {                                    // C 10061 (the renamed spelling)
      orderFront(sender)                                  // C 10063
      makeKey()                                           // C 10064 (Swift's name for makeKeyWindow)
    } else {
      super.makeKeyAndOrderFront(sender)                  // C 10067
    }
  }

  /// C 10098-10122: `- (void)orderBack:(id)sender`.  C sorts the parent's children with the
  /// comparator above, re-adds the receiver above them all, then walks the sorted list re-adding
  /// every other window in turn; the `else` is the `super` call.
  @objc override func orderBack(_ sender: Any?) {
    let parent = parent                                   // C 10102 (the renamed spelling)
    if parent != nil {                                    // C 10103
      let children = (parent!.childWindows ?? [])        // C 10105-10107
        .sorted { nswindow_orderedIndex_sort($0 as AnyObject, $1 as AnyObject, nil) < 0 }
      parent!.removeChildWindow(self)                     // C 10108
      parent!.addChildWindow(self, ordered: .above)       // C 10109

      for win in children {                               // C 10111
        if win != self {                                  // C 10113
          parent!.removeChildWindow(win)                  // C 10115
          parent!.addChildWindow(win, ordered: .above)    // C 10116
        }
      }
    } else {
      super.orderBack(sender)                             // C 10121
    }
  }
}

// ==========================================================================
// Round 25: `EmacsView`'s class record with `fsIsNative`, `isFullscreen`,
// `updateCollectionBehavior`, and the `check_native_fs` they unblock
// ==========================================================================

/// C: `bool ns_use_native_fullscreen` - src/nsterm.m 11702's `DEFVAR_BOOL
/// ("ns-use-native-fullscreen", …)`, this file's own variable, imported as the C data symbol the
/// core exports (the `ns_auto_hide_menu_bar` shape above: it is owed until this unit's own
/// `syms_of_nsterm` slice lands).  `ns_last_use_native_fullscreen` (C 335) is the static beside it.
@_silgen_name("ns_use_native_fullscreen") var ns_use_native_fullscreen: Bool

/// C `src/nsterm.h` 466-488: `@interface EmacsView : NSView <NSTextInput, NSTextInputClient,
/// NSWindowDelegate>`.  **Round 25 lands the record** - the fifteen ivars of nsterm.h 470-488,
/// `@public emacsframe` (485) included - and three of its methods.  The protocol list is the one
/// named deferral: Swift enforces every required method of those protocols at the point the
/// conformance is declared, so the `<…>` list lands in the round that carries that method batch,
/// not with the record (the ledger's box for this class says so).  The reference's own text is
/// unchanged by this split: no method below reads a protocol requirement it does not implement.
class EmacsView: NSView {
  /// C nsterm.h 472-474, the `NS_IMPL_COCOA` arm: `char *old_title; BOOL maximizing_resize;`
  var old_title: UnsafeMutablePointer<CChar>? = nil
  var maximizing_resize: Bool = false
  /// C nsterm.h 475-479.
  var font_panel_active: Bool = false
  var font_panel_result: NSFont? = nil
  var windowClosing: Bool = false
  var workingText: String? = nil
  var processingCompose: Bool = false
  /// C nsterm.h 480-483.
  var fs_state: Int32 = 0
  var fs_before_fs: Int32 = 0
  var next_maximized: Int32 = 0
  var maximized_width: Int32 = 0
  var maximized_height: Int32 = 0
  var nonfs_window: EmacsWindow? = nil
  var fs_is_native: Bool = false
  /// C nsterm.h 485-487, the `@public` half: C 2538 reaches `emacsframe` through a cast, so the
  /// property is the same stored slot.
  var emacsframe: UnsafeMutableRawPointer? = nil
  var scrollbarsNeedingUpdate: Int32 = 0
  var ns_userRect: NSRect = .zero

  /// `NSView`'s designated initializer, spelled out for the reason `EmacsWindow`'s is (Swift
  /// marks `init (coder:)` unavailable on these AppKit classes; measured last round).
  override init(frame frameRect: NSRect) {
    super.init(frame: frameRect)
  }

  required init?(coder: NSCoder) {
    super.init(coder: coder)
  }

  /// C 8952-8955: `- (BOOL)fsIsNative` - C's whole body is the ivar read.
  func fsIsNative() -> Bool {
    fs_is_native                                      // C 8954
  }

  /// C 8957-8978: `- (BOOL)isFullscreen`.  The `MAC_OS_X_VERSION_MAX_ALLOWED >= 1070` arm is the
  /// live one (C 8967-8971's `#else` is dead on this target), and C's
  /// `([[self window] styleMask] & NSWindowStyleMaskFullScreen) != 0` is the style-mask contains.
  func isFullscreen() -> Bool {
    var res: Bool
    if !fs_is_native {                                // C 8961
      res = (nonfs_window != nil)                     // C 8963
    } else {
      res = (window?.styleMask.contains(.fullScreen) ?? false)   // C 8968
    }
    return res                                        // C 8977
  }

  /// C 8981-9014: `- (void)updateCollectionBehavior` (the `NS_IMPL_COCOA && MAX_ALLOWED >= 1070`
  /// arm; C 9015 closes it).  C's three mask operations are the `NSWindowCollectionBehavior`
  /// option-set's insert/remove pair, which is what the three `b &=`/`b |=` lines say.
  func updateCollectionBehavior() {
    if !isFullscreen() {                              // C 8985
      let win = window                                // C 8987
      var b = win?.collectionBehavior ?? []           // C 8988
      if ns_use_native_fullscreen {                   // C 8989
        if FRAME_PARENT_FRAME(emacsframe) != nil || FRAME_TOOLTIP_P(emacsframe) {
          b.remove(.fullScreenPrimary)                // C 8994
          b.insert(.fullScreenAuxiliary)              // C 8995
        } else {
          b.insert(.fullScreenPrimary)                // C 8999
          b.remove(.fullScreenAuxiliary)              // C 9000
        }
      } else {
        b.remove(.fullScreenPrimary)                  // C 9005
      }
      win?.collectionBehavior = b                     // C 9008
      fs_is_native = ns_use_native_fullscreen         // C 9012
    }
  }

  /// C 7535-7568: `- (void)setMarkedText: (id)aString selectedRange: (NSRange)selRange` -
  /// "&lt;NSTextInput&gt;: inserts display of composing characters."  C's `[aString
  /// respondsToSelector: @selector (string)]` is the optional cast, C 7561-7562's
  /// `[workingText release]` + `[str copy]` is the ARC assignment, and C 7578-7579's `NS_KEYLOG`
  /// block is dead (`NS_KEYLOG` is 0 at C 7241).
  @objc func setMarkedText(_ aString: Any?, selectedRange selRange: NSRange) {
    let str: NSString = (aString as? NSAttributedString)?.string as NSString? ?? aString as? NSString ?? ""   // C 7540-7541
    if str.length == 0 {                              // C 7551
      deleteWorkingText()                             // C 7553
      return                                          // C 7554
    }
    if emacs_event == nil {                           // C 7557
      return                                          // C 7558
    }

    processingCompose = true                          // C 7560
    workingText = str as String                       // C 7562 (C's `[str copy]`)
    ns_working_text = lispString(workingText! as NSString)   // C 7563

    input_event_set_kind(emacs_event!, NS_TEXT_EVENT) // C 7565
    emacs_event!.pointee.code = UInt32(KEY_NS_PUT_WORKING_TEXT)   // C 7566
    EV_TRAILER(nil, emacsframe)                       // C 7567
  }

  /// C 7571-7590: `- (void)deleteWorkingText` - "Delete display of composing characters [not in
  /// &lt;NSTextInput&gt;]."  C 7580's `[workingText release]` is the reference being dropped.
  @objc func deleteWorkingText() {
    if workingText == nil {                           // C 7576
      return                                          // C 7577
    }
    workingText = nil                                 // C 7581
    processingCompose = false                         // C 7582

    if emacs_event == nil {                           // C 7584
      return                                          // C 7585
    }

    input_event_set_kind(emacs_event!, NS_TEXT_EVENT) // C 7587
    emacs_event!.pointee.code = UInt32(KEY_NS_UNPUT_WORKING_TEXT)  // C 7588
    EV_TRAILER(nil, emacsframe)                       // C 7589
  }

  /// C 7613-7621: `- (void)unmarkText`.
  @objc func unmarkText() {
    deleteWorkingText()                               // C 7619
    processingCompose = false                         // C 7620
  }

  /// C 7481-7534: `- (void)insertText: (id)aString` - "&lt;NSTextInput&gt;: called when done
  /// composing".  C 7515-7523's surrogate pair is the two predicates and the helper above;
  /// C 7496's and C 7525-7526's `NS_KEYLOG`/`TODO` lines are C's own comments.
  @objc override func insertText(_ aString: Any?) {
    let s: NSString = (aString as? NSAttributedString)?.string as NSString? ?? aString as? NSString ?? ""   // C 7488-7491
    let len = s.length                                // C 7493
    processingCompose = false                         // C 7497

    if emacs_event == nil {                           // C 7499
      return                                          // C 7500
    }

    /* C 7502-7504: "First, clear any working text."  */
    if workingText != nil {                           // C 7503
      deleteWorkingText()                             // C 7504
    }

    /* C 7511-7512: "Now insert the string as keystrokes."  */
    var i = 0
    while i < len {
      var code = UInt32(s.character(at: i))           // C 7514
      if UTF_16_HIGH_SURROGATE_P(code) && i < len - 1 {   // C 7515
        let low = UInt32(s.character(at: i + 1))      // C 7517
        if UTF_16_LOW_SURROGATE_P(low) {              // C 7518
          code = surrogates_to_codepoint(low, code)   // C 7520
          i += 1                                      // C 7521
        }
      }
      /* C 7524-7526: "TODO: still need this?" - the 0xB2DC code unit becomes a tilde.  */
      if code == 0x2DC { code = 0x7E }
      if code != 32 {                                 // C 7527
        emacs_event!.pointee.modifiers = 0            // C 7528
      }
      input_event_set_kind(emacs_event!,
                           code > 0xFF ? MULTIBYTE_CHAR_KEYSTROKE_EVENT : ASCII_KEYSTROKE_EVENT)  // C 7529-7530
      emacs_event!.pointee.code = code                // C 7531
      EV_TRAILER(nil, emacsframe)                     // C 7532
      i += 1
    }
  }

  /// C 7435-7442: `- (void) insertText: (id) string replacementRange: (NSRange) replacementRange` -
  /// the `NSTextInputClient` spelling, which unmarks and hands the string to the older method.
  @objc func insertText(_ string: Any?, replacementRange: NSRange) {
    var s = string
    if let attributed = string as? NSAttributedString {      // C 7438
      s = attributed.string                                 // C 7439
    }
    unmarkText()                                            // C 7440
    insertText(s)                                           // C 7441
  }

  /// C 7444-7449: `- (void) setMarkedText: (id) string selectedRange: (NSRange) selectedRange
  /// replacementRange: (NSRange) replacementRange` - C's body is the two-argument call.
  @objc func setMarkedText(_ string: Any?, selectedRange: NSRange, replacementRange: NSRange) {
    setMarkedText(string, selectedRange: selectedRange)      // C 7448
  }

  /// C 7451-7456: `- (nullable NSAttributedString *) attributedSubstringForProposedRange:
  /// (NSRange) range actualRange: (nullable NSRangePointer) actualRange` - C's whole body is
  /// `return nil;`.
  @objc func attributedSubstring(forProposedRange range: NSRange,
                                 actualRange: NSRangePointer?) -> NSAttributedString? {
    nil                                              // C 7455
  }

  /// C 7593-7598: `- (BOOL)hasMarkedText` - C's whole body is the `workingText` test.
  @objc func hasMarkedText() -> Bool {
    workingText != nil                                // C 7597
  }

  /// C 7601-7610: `- (NSRange)markedRange`.  C's `NSMakeRange (NSNotFound, 0)` is the "no marked
  /// text" answer, and C 7607-7608's `NS_KEYLOG` block is dead (`NS_KEYLOG` is 0 at C 7241).
  @objc func markedRange() -> NSRange {
    let rng = (workingText != nil)
      ? NSRange(location: 0, length: (workingText! as NSString).length)
      : NSRange(location: NSNotFound, length: 0)      // C 7605-7606
    return rng                                        // C 7609
  }

  /// C 7725-7731: `- (NSArray *)validAttributesForMarkedText` - C's lazy `static NSArray *arr`
  /// starts empty and is never filled (its one `arrayWithObject:` is commented out at C 7729), so
  /// the body is the empty array; C 7730's `return arr` is this return.
  @objc func validAttributesForMarkedText() -> [NSAttributedString.Key] {
    []                                                // C 7730
  }

  /// C 7757-7767: `- (NSUInteger)characterIndexForPoint: (NSPoint)thePoint` - the
  /// `NS_IMPL_COCOA` arm of C 7757-7762, whose whole body is `return 0;`.
  @objc func characterIndexForPoint(_ thePoint: NSPoint) -> UInt {
    0                                                 // C 7766
  }

  /// C 7699-7702: `- (NSInteger)conversationIdentifier` - C returns the receiver's own address as
  /// an integer, which Swift spells through `Unmanaged`.
  @objc func conversationIdentifier() -> Int {
    Int(bitPattern: Unmanaged.passUnretained(self).toOpaque())   // C 7701
  }

  /// C 7705-7723: `- (void)doCommandBySelector: (SEL)aSelector`.  C 7709-7710's `NS_KEYLOG` block
  /// is dead (`NS_KEYLOG` is 0 at C 7241); the `deleteBackward:` arm is the live one, and C's
  /// `EV_TRAILER ((id)nil)` reaches the same `emacsframe` this unit's other callers pass.
  @objc func doCommandBySelector(_ aSelector: Selector) {
    processingCompose = false                         // C 7712
    if aSelector == #selector(NSResponder.deleteBackward(_:)) {   // C 7713
      /* C 7715-7716: "Happens when user backspaces over an ongoing composition: throw a
         'delete' into the event queue."  */
      if emacs_event == nil {                         // C 7717
        return                                        // C 7718
      }
      input_event_set_kind(emacs_event!, NON_ASCII_KEYSTROKE_EVENT)   // C 7719
      emacs_event!.pointee.code = 0xFF08              // C 7720
      EV_TRAILER(nil, emacsframe)                     // C 7721
    }
  }
}

/// C 5086-5107: `static void check_native_fs (void)` - the function C 5127 and C 5218 call from
/// their live `NS_IMPL_COCOA` arm, and the reason the event-loop pair was held: its body calls
/// `[view updateCollectionBehavior]` (C 5103), which is the method above.  `FOR_EACH_FRAME` keeps
/// the spelling `ns_constrain_all_frames` uses (C 1011's reading above).
fileprivate func check_native_fs() {
  if ns_last_use_native_fullscreen == ns_use_native_fullscreen {
    return                                            // C 5093
  }
  ns_last_use_native_fullscreen = ns_use_native_fullscreen   // C 5095

  var tail: Lisp_Object = Vframe_list                 // C 5097
  while CONSP(tail) {
    let frame = XCAR(tail)
    let f = XFRAME(frame)
    if FRAME_NS_P(f) {                                // C 5100
      let view = FRAME_NS_VIEW(f)                     // C 5102
      (view as? EmacsView)?.updateCollectionBehavior()   // C 5103
    }
    tail = XCDR(tail)
  }
}

// ==========================================================================
// Round 26: C 5203-5385 - `ns_select_1`, `ns_select` and `ns_run_loop_break`
// ==========================================================================

/// C 323: `enum { SELECT_HAVE_READ = 1, SELECT_HAVE_WRITE = 2, SELECT_HAVE_TMO = 4 };` - the three
/// bits `ns_select_1` accumulates into `select_valid` (C 5277/5282/5288); C 322's statics are this
/// unit's own entries above.
let SELECT_HAVE_READ = 1
let SELECT_HAVE_WRITE = 2
let SELECT_HAVE_TMO = 4

/// C: `typedef int select_func (int, fd_set *, fd_set *, fd_set *, const struct timespec *,
/// const sigset_t *);` (src/thread.h 337-338) - the function-pointer type `thread_select` takes,
/// spelled as the `@convention(c)` type the port's own `select_func` alias names
/// (`femacs/src/include/thread.rs` 300, P-27's `atimer_callback` shape).
typealias select_func = @convention(c) (Int32, UnsafeMutablePointer<fd_set>?,
                                       UnsafeMutablePointer<fd_set>?,
                                       UnsafeMutablePointer<fd_set>?,
                                       UnsafePointer<timespec>?,
                                       UnsafePointer<sigset_t>?) -> Int32

/// C: `int thread_select (select_func *func, int max_fds, fd_set *rfds, fd_set *wfds,
/// fd_set *efds, struct timespec *timeout, sigset_t *sigmask);` (src/thread.h 338) - the port
/// defines it at `femacs/src/thread.rs` 666 as an `extern "C"` entry, so this unit imports the
/// symbol and passes `pselect` (C's own argument at C 5242/5247).
@_silgen_name("thread_select")
func thread_select(_ func: select_func?, _ max_fds: Int32,
                   _ rfds: UnsafeMutablePointer<fd_set>?, _ wfds: UnsafeMutablePointer<fd_set>?,
                   _ efds: UnsafeMutablePointer<fd_set>?,
                   _ timeout: UnsafeMutablePointer<timespec>?,
                   _ sigmask: UnsafeMutablePointer<sigset_t>?) -> Int32

/// C: `ptrdiff_t emacs_write_sig (int, const void *, ptrdiff_t)` (src/sysdep.h) - the port
/// defines it at `femacs/src/sysdep.rs` 4022; the `fputs` import shape.
@_silgen_name("emacs_write_sig")
func emacs_write_sig(_ fd: Int32, _ buf: UnsafeRawPointer?, _ nbyte: Int) -> Int

/// C: `FD_ISSET (fd, set)` (sys/select.h) - the macro's Darwin expansion, `__darwin_fd_isset`
/// (the `fputs` story again: Swift marks the fd_set family unavailable).
@_silgen_name("__darwin_fd_isset")
func __darwin_fd_isset(_ fd: Int32, _ set: UnsafePointer<fd_set>?) -> Int32

/// C: `timespectod (t)` (src/systime.h) - "Convert a struct timespec to a double"; the two
/// members and their units, as C's own `(t).tv_sec + (t).tv_nsec / 1e9` states them.
fileprivate func timespectod(_ t: timespec) -> Double {
  Double(t.tv_sec) + Double(t.tv_nsec) / 1e9
}

/// C 5203-5363: `static int ns_select_1 (int nfds, fd_set *readfds, fd_set *writefds,
/// fd_set *exceptfds, struct timespec *timeout, sigset_t *sigmask, BOOL run_loop_only)` - C's
/// replacement for `select`, "checking for events".  It is the second caller of `check_native_fs`
/// (C 5218) that round 25 unblocked.
fileprivate func ns_select_1(_ nfds: Int32, _ readfds: UnsafeMutablePointer<fd_set>?,
                             _ writefds: UnsafeMutablePointer<fd_set>?,
                             _ exceptfds: UnsafeMutablePointer<fd_set>?,
                             _ timeout: UnsafeMutablePointer<timespec>?,
                             _ sigmask: UnsafeMutablePointer<sigset_t>?,
                             _ run_loop_only: Bool) -> Int32 {
  var result: Int32
  var t: Int32
  var k: Int32
  var nr: Int32 = 0
  var event = input_event(bits: 0, code: 0, modifiers: 0, x: nil, y: nil, timestamp: 0,
                          frame_or_window: nil, arg: nil, device: nil)
  var c: CChar

  check_native_fs()                                   // C 5218

  /* C 5221-5224: store the held events so the reference can recognize C-g (C's own comment
     says "so that Emacs can recognize C-g"; the citation is what the reading above counts).  */
  if hold_event_q_nr > 0 {                            // C 5225
    var i: Int32 = 0
    while i < hold_event_q_nr {
      kbd_buffer_store_event_hold(&(hold_event_q_q![Int(i)]), nil)   // C 5228
      i += 1
    }
    hold_event_q_nr = 0                               // C 5229
  }

  /* C 5232's `eassert` is the no-op of an unset `ENABLE_CHECKING` (this unit's reading).  */
  k = 0
  while k < nfds {                                    // C 5233
    if readfds != nil && __darwin_fd_isset(k, readfds) != 0 { nr += 1 }   // C 5235
    if writefds != nil && __darwin_fd_isset(k, writefds) != 0 { nr += 1 } // C 5236
    k += 1
  }

  if NSApp == nil                                     // C 5239
       || !Thread.current.isMainThread                // C 5240
       || (timeout != nil && timeout!.pointee.tv_sec == 0 && timeout!.pointee.tv_nsec == 0) {
    return thread_select(pselect, nfds, readfds, writefds, exceptfds, timeout, sigmask)   // C 5242
  } else {
    var t2 = timespec(tv_sec: 0, tv_nsec: 1)          // C 5246
    _ = thread_select(pselect, 0, nil, nil, nil, &t2, sigmask)   // C 5247
  }

  /* C 5250-5265: the `outerpool` drain is commented out in the reference itself, and the comment
     is carried at C 5250-5265 rather than restated here.  */

  send_appdefined = true                              // C 5268
  if nr > 0 {                                         // C 5269
    pthread_mutex_lock(&select_mutex)                 // C 5271
    select_nfds = nfds                                // C 5272
    select_valid = 0                                  // C 5273
    if readfds != nil {                               // C 5274
      select_readfds = readfds!.pointee               // C 5276
      select_valid += Int32(SELECT_HAVE_READ)         // C 5277
    }
    if writefds != nil {                              // C 5279
      select_writefds = writefds!.pointee             // C 5281
      select_valid += Int32(SELECT_HAVE_WRITE)        // C 5282
    }
    if timeout != nil {                               // C 5285
      select_timeout = timeout!.pointee               // C 5287
      select_valid += Int32(SELECT_HAVE_TMO)          // C 5288
    }
    pthread_mutex_unlock(&select_mutex)               // C 5291

    /* C 5293-5295: inform fd_handler that select should be called.  */
    c = CChar(UInt8(ascii: "g"))
    _ = emacs_write_sig(selfds.1, &c, 1)
  } else if nr == 0 && timeout != nil {               // C 5297
    /* C 5299-5300: no file descriptor, just a timeout.  */
    let time = timespectod(timeout!.pointee)
    /* C 5301-5307's `[[NSTimer scheduledTimerWithTimeInterval:…] retain]`: Swift's factory is
       `timeInterval:` and the run loop already holds the timer, which is what C's `retain` buys
       (the ARC reading this lane uses for `release`).  */
    timed_entry = Timer.scheduledTimer(timeInterval: time,
                                       target: NSApp!,
                                       selector: #selector(EmacsApp.timeout_handler(_:)),
                                       userInfo: nil,
                                       repeats: false)
  } else {
    /* C 5309-5312: no timeout and no file descriptors - "can this happen?"  */
    ns_send_appdefined(-1)                            // C 5312
  }

  block_input()                                       // C 5315
  ns_init_events(&event)                              // C 5316
  NSApp.run()                                         // C 5318
  ns_finish_events()                                  // C 5320
  if nr > 0 && readfds != nil {                       // C 5321
    c = CChar(UInt8(ascii: "s"))
    _ = emacs_write_sig(selfds.1, &c, 1)              // C 5324
  }
  unblock_input()                                     // C 5326

  t = last_appdefined_event_data                      // C 5328
  if t != NO_APPDEFINED_DATA {                        // C 5330
    last_appdefined_event_data = NO_APPDEFINED_DATA   // C 5332
    if t == -2 {                                      // C 5334
      result = 0                                      // C 5337 (a timeout)
    } else if t == -1 {                               // C 5339
      errno = EINTR                                   // C 5343
      result = -1                                     // C 5344
    } else {
      /* C 5348: received back from `select ()` in fd_handler - copy the results.  */
      pthread_mutex_lock(&select_mutex)               // C 5349
      if readfds != nil { readfds!.pointee = select_readfds }    // C 5350
      if writefds != nil { writefds!.pointee = select_writefds } // C 5351
      pthread_mutex_unlock(&select_mutex)             // C 5352
      result = t                                        // C 5353
    }
  } else {
    errno = EINTR                                     // C 5358
    result = -1                                       // C 5359
  }

  return result                                       // C 5362
}

/// C 5365-5372: `int ns_select (int nfds, fd_set *readfds, fd_set *writefds, fd_set *exceptfds,
/// struct timespec *timeout, sigset_t *sigmask)` - C's body is the `ns_select_1` call with
/// `NO`.
func ns_select(_ nfds: Int32, _ readfds: UnsafeMutablePointer<fd_set>?,
               _ writefds: UnsafeMutablePointer<fd_set>?,
               _ exceptfds: UnsafeMutablePointer<fd_set>?,
               _ timeout: UnsafeMutablePointer<timespec>?,
               _ sigmask: UnsafeMutablePointer<sigset_t>?) -> Int32 {
  ns_select_1(nfds, readfds, writefds, exceptfds, timeout, sigmask, false)   // C 5370-5371
}

/// C 5374-5385: `void ns_run_loop_break (void)` (the `HAVE_PTHREAD` arm) - "Break out of the NS
/// run loop in ns_select or ns_read_socket."
func ns_run_loop_break() {
  /* C 5381-5383: "If we don't have a GUI, don't send the event."  */
  if NSApp != nil {                                   // C 5382
    ns_send_appdefined(-1)                            // C 5383
  }
}

// ==========================================================================
// Round 27: `EmacsApp`'s termination and file-open handlers (C 6633-6758)
// ==========================================================================

/// C: `Lisp_Object ns_confirm_quit` - this file's own `DEFVAR_BOOL` (`ns-confirm-quit`), imported
/// as the data symbol the core exports (the `ns_auto_hide_menu_bar` shape above).
@_silgen_name("ns_confirm_quit") var ns_confirm_quit: Lisp_Object

/// C: `extern int initial_argc;` and `extern char **initial_argv;` (src/emacs.c) - the command line
/// `not_in_argv` compares against; two more imports of the core's data symbols.
@_silgen_name("initial_argc") var initial_argc: Int32
@_silgen_name("initial_argv")
var initial_argv: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?

/// C: `int strcmp (const char *, const char *)` (string.h) - the `fputs` shape again.
@_silgen_name("strcmp") func strcmp(_ a: UnsafePointer<CChar>, _ b: UnsafePointer<CChar>) -> Int32

/// C 6633-6652: `static bool runAlertPanel (NSString *title, NSString *msgFormat, NSString
/// *defaultButton, NSString *alternateButton)`.  The `NS_IMPL_COCOA` arm builds an `NSAlert`
/// (`NS_IMPL_GNUSTEP`'s `NSRunAlertPanel` at C 6640-6641 is dead here); C's `[alert release]`
/// (C 6649) is the reference being dropped under ARC, the reading this lane already uses.
fileprivate func runAlertPanel(_ title: String, _ msgFormat: String,
                               _ defaultButton: String, _ alternateButton: String) -> Bool {
  let alert = NSAlert()                              // C 6643
  alert.alertStyle = .critical                       // C 6644
  alert.messageText = msgFormat                      // C 6645
  alert.addButton(withTitle: defaultButton)          // C 6646
  alert.addButton(withTitle: alternateButton)        // C 6647
  let ret = alert.runModal()                         // C 6648
  return ret == .alertFirstButtonReturn              // C 6650
}

/// C 6671-6679: `static int not_in_argv (NSString *arg)`.  C's `[arg UTF8String]` is the
/// `withCString` view the two notification methods use, and the walk keeps C's `k` and its two
/// returns.
fileprivate func not_in_argv(_ arg: String) -> Int32 {
  var k: Int32 = 1                                   // C 6674
  return arg.withCString { p in                      // C 6675
    while k < initial_argc {                         // C 6676
      if strcmp(p, initial_argv![Int(k)]!) == 0 { return 0 }   // C 6677
      k += 1
    }
    return 1                                         // C 6678
  }
}

// ==========================================================================
// Round 28: the `NS_NONKEY_EVENT` value, `ns_read_socket_1` and `ns_read_socket`
// ==========================================================================

/// C: `NS_NONKEY_EVENT` - an `enum event_kind` member C gates on `HAVE_NS`
/// (`src/termhooks.h` 273-279, between `CONFIG_CHANGED_EVENT` and `XWIDGET_EVENT`).  Its value is
/// the declaration order of that enum (C's members carry no initialiser), and this unit's number
/// comes from counting it under the Cocoa build's configuration: the frozen table cannot decide it
/// (it has no `HAVE_NS` and no `MAC_OS_X_*`), and neither mirror defines it (the Rust side carries
/// the member only inside `#[cfg (feature = "have_ns")]` arms, which the frozen configuration
/// never compiles).
///
/// **Calibration**: the port's own mirror fixes `SLEEP_EVENT` at 28
/// (`femacs/src/include/termhooks.rs` 55, a value the constant gate compares), and that is what the
/// same count gives for the frozen configuration (`THREADS_ENABLED`, `USE_FILE_NOTIFY` and
/// `HAVE_XWIDGETS` all off there).  Under Cocoa, `THREADS_ENABLED` and `USE_FILE_NOTIFY` are on
/// and `HAVE_XWIDGETS`/`HAVE_DBUS`/`HAVE_NTGUI` off, which puts `NS_TEXT_EVENT` at 29 and this
/// member at **30** - the first value this unit takes as **UNVERIFIED** (no Cocoa build exists here
/// to check it against C; the row is recorded rather than hidden, the `XmVERSION_STRING` shape
/// round 47 of the X11 lane used).
fileprivate let NS_NONKEY_EVENT: UInt32 = 30

/// C `src/nsterm.h` 796/800/803: `#define KEY_NS_POWER_OFF ((1<<28)|(0<<16)|1)`,
/// `KEY_NS_OPEN_FILE_LINE` (`|8`) and `KEY_NS_SPI_SERVICE_CALL` (`|11`) - the three `code` values
/// the functions below write.
let KEY_NS_POWER_OFF = (1 << 28) | (0 << 16) | 1
let KEY_NS_OPEN_FILE_LINE = (1 << 28) | (0 << 16) | 8
let KEY_NS_SPI_SERVICE_CALL = (1 << 28) | (0 << 16) | 11

/// C `void *inputs`: `static Lisp_Object ns_input_file, ns_input_line, ns_input_spi_name,
/// ns_input_spi_arg;` (src/nsterm.m 3395-3400 area) - this file's own file-scope objects, imported
/// as the data symbols the core exports (the `ns_auto_hide_menu_bar` reading).
@_silgen_name("ns_input_file") var ns_input_file: Lisp_Object
@_silgen_name("ns_input_line") var ns_input_line: Lisp_Object
@_silgen_name("ns_input_spi_name") var ns_input_spi_name: Lisp_Object
@_silgen_name("ns_input_spi_arg") var ns_input_spi_arg: Lisp_Object

/// C: `Lisp_Object lispString (NSString *)` - a helper of the Cocoa lane, exported by
/// `nsfns.swift` 4968-4969 (`@_cdecl`) and imported here in the shape `nsselect.swift` 230 and
/// `nsmenu.swift` 429 already use.
@_silgen_name("lispString") func lispString(_ string: NSString) -> Lisp_Object

/// C 5110-5193: `static int ns_read_socket_1 (struct terminal *terminal, struct input_event
/// *hold_quit, BOOL no_release)` - "Post an event to ourself and keep reading events until we read
/// it back again."  Its three callees are the ones rounds 22-25 landed (`check_native_fs`,
/// `ns_send_appdefined`, `EmacsApp -openFile:`/`-fulfillService:withArg:`), which is why it waited.
fileprivate func ns_read_socket_1(_ terminal: UnsafeMutableRawPointer?,
                                  _ hold_quit: UnsafeMutablePointer<input_event>?,
                                  _ no_release: Bool) -> Int32 {
  var ev = input_event(bits: 0, code: 0, modifiers: 0, x: nil, y: nil, timestamp: 0,
                       frame_or_window: nil, arg: nil, device: nil)          // C 5121
  var nevents: Int32

  check_native_fs()                                  // C 5127

  if NSApp.modalWindow != nil {                      // C 5130
    return -1                                        // C 5131
  }

  if hold_event_q_nr > 0 {                           // C 5133
    var i: Int32 = 0                                 // C 5135
    while i < hold_event_q_nr {                      // C 5136
      kbd_buffer_store_event_hold(&(hold_event_q_q![Int(i)]), hold_quit)   // C 5137
      i += 1
    }
    hold_event_q_nr = 0                              // C 5138
    return i                                         // C 5139
  }

  if Thread.current.isMainThread {                   // C 5142
    block_input()                                    // C 5144
    n_emacs_events_pending = 0                       // C 5145
    ns_init_events(&ev)                              // C 5146
    q_event_ptr = hold_quit                          // C 5147

    if !no_release {                                 // C 5149
      /* C 5151-5153: the pool is allocated/reallocated around the loop; C's
         `[outerpool release]` + `[[NSAutoreleasePool alloc] init]` pair is the pop/push below
         (this unit's pool reading).  */
      objc_autoreleasePoolPop(outerpool)              // C 5154
      outerpool = objc_autoreleasePoolPush()          // C 5155
    }

    /* C 5158: "If have pending open-file requests, attend to the next one of those."  */
    if ns_pending_files != nil && ns_pending_files!.count != 0
         && (NSApp as! EmacsApp).openFile(ns_pending_files![0] as! String) { // C 5159-5160
      ns_pending_files!.removeObject(at: 0)          // C 5162
    } else if ns_pending_service_names != nil && ns_pending_service_names!.count != 0
         && (NSApp as! EmacsApp).fulfillService(ns_pending_service_names![0] as! String,
                                                withArg: ns_pending_service_args![0] as! String) {  // C 5165-5168
      ns_pending_service_names!.removeObject(at: 0)  // C 5170
      ns_pending_service_args!.removeObject(at: 0)   // C 5171
    } else {
      /* C 5175-5176: "Run and wait for events.  We must always send one NX_APPDEFINED event to
         ourself, otherwise [NXApp run] will never exit."  */
      send_appdefined = true                         // C 5177
      ns_send_appdefined(-1)                         // C 5178
      NSApp.run()                                    // C 5180
    }

    nevents = n_emacs_events_pending                 // C 5183
    n_emacs_events_pending = 0                       // C 5184
    ns_finish_events()                               // C 5185
    q_event_ptr = nil                                // C 5186
    unblock_input()                                  // C 5187
  } else {
    return -1                                        // C 5190
  }

  return nevents                                     // C 5192
}

/// C 5195-5199: `static int ns_read_socket (struct terminal *terminal, struct input_event
/// *hold_quit)` - C's body is the `ns_read_socket_1` call with `NO`.
fileprivate func ns_read_socket(_ terminal: UnsafeMutableRawPointer?,
                                _ hold_quit: UnsafeMutablePointer<input_event>?) -> Int32 {
  ns_read_socket_1(terminal, hold_quit, false)       // C 5198
}

// ==========================================================================
// Round 29: `showPreferencesWindow:`, `newFrame:` and `fd_handler:`
// ==========================================================================

/// C `src/nsterm.h` 804 and 806: `#define KEY_NS_NEW_FRAME ((1<<28)|(0<<16)|12)` and
/// `#define KEY_NS_SHOW_PREFS ((1<<28)|(0<<16)|14)`.
let KEY_NS_NEW_FRAME = (1 << 28) | (0 << 16) | 12
let KEY_NS_SHOW_PREFS = (1 << 28) | (0 << 16) | 14

/// C: `void FD_ZERO (fd_set *)` and `void FD_SET (int, fd_set *)` (sys/select.h) - the other two
/// Darwin expansions of that family (`__darwin_fd_isset` above is the third), imported in the same
/// shape because Swift marks the `fd_set` macros unavailable.
@_silgen_name("__darwin_fd_zero") func __darwin_fd_zero(_ set: UnsafeMutablePointer<fd_set>?)
@_silgen_name("__darwin_fd_set")
func __darwin_fd_set(_ fd: Int32, _ set: UnsafeMutablePointer<fd_set>?)

// ==========================================================================
// Round 30: `applicationDidFinishLaunching:` and `requestService:`
// ==========================================================================

/// C: `CFTypeID CFSocketGetTypeID (void)` and `CFTypeID CFFileDescriptorGetTypeID (void)`
/// (CoreFoundation) - the two dummy calls C 6531/6532 makes so that the framework raises the
/// process's open-file limit before C 6535-6539 lowers it again.
@_silgen_name("CFSocketGetTypeID") func CFSocketGetTypeID() -> UInt
@_silgen_name("CFFileDescriptorGetTypeID") func CFFileDescriptorGetTypeID() -> UInt

/// C: `Lisp_Object build_string (const char *)` (src/lisp.h) - the port defines it in
/// `femacs/src/alloc.rs`; the `fputs` import shape.
@_silgen_name("build_string") func build_string(_ s: UnsafePointer<CChar>) -> Lisp_Object

/// C `+[EmacsImage allocInitFromFile:]` - the class is `nsimage.swift`'s, so the call crosses as a
/// C-ABI entry under `<Class>_<selector>` (the `EmacsImage_stippleMask` shape round 18 set).
@_silgen_name("EmacsImage_allocInitFromFile")
func EmacsImage_allocInitFromFile(_ file: Lisp_Object) -> AnyObject?

/// C: `NSPasteboard`-side helpers of the selection lane, both defined in `nsselect.swift`
/// (`stringWithLispString` at 228, `ns_string_from_pasteboard` at 533); `requestService:` below is
/// their reader in this unit.
@_silgen_name("stringWithLispString") func stringWithLispString(_ string: Lisp_Object) -> NSString?
@_silgen_name("ns_string_from_pasteboard")
func ns_string_from_pasteboard(_ pb: NSPasteboard?) -> Lisp_Object

// ==========================================================================
// Round 31: `EmacsWindow`'s ordering methods (C 10043-10122)
// ==========================================================================

/// C 10084-10096: `static NSInteger nswindow_orderedIndex_sort (id w1, id w2, void *c)` - the
/// comparator `orderBack:` sorts its parent's children with.  C sends `orderedIndex` as a
/// **message**, and on Cocoa that selector is not declared on `NSWindow` at all (C's own
/// definition of it sits in the `NS_IMPL_GNUSTEP` arm at C 10074-10077), so the port sends the
/// same dynamic message rather than inventing a property.
fileprivate func nswindow_orderedIndex_sort(_ w1: AnyObject, _ w2: AnyObject,
                                           _ c: UnsafeMutableRawPointer?) -> Int {
  let i1 = Int(bitPattern: (w1 as AnyObject).perform(Selector(("orderedIndex")))?
                 .takeUnretainedValue() as? UInt ?? 0)                       // C 10087
  let i2 = Int(bitPattern: (w2 as AnyObject).perform(Selector(("orderedIndex")))?
                 .takeUnretainedValue() as? UInt ?? 0)                       // C 10088

  if i1 > i2 { return -1 } /* NSOrderedAscending */                         // C 10090-10091
  if i1 < i2 { return 1 }  /* NSOrderedDescending */                        // C 10092-10093
  return 0                 /* NSOrderedSame */                              // C 10095
}



/// C: `NON_ASCII_KEYSTROKE_EVENT` and `ASCII_KEYSTROKE_EVENT` - two `enum event_kind` members the
/// input methods write into `kind`; the port's mirror fixes both values
/// (`femacs/src/include/termhooks.rs` 30 and 28).  They go through the bitfield setter, like
/// `SLEEP_EVENT` above.
fileprivate let NON_ASCII_KEYSTROKE_EVENT: UInt32 = 3
fileprivate let ASCII_KEYSTROKE_EVENT: UInt32 = 1
/// C: `MULTIBYTE_CHAR_KEYSTROKE_EVENT` - the third member `insertText:` writes into `kind`
/// (`femacs/src/include/termhooks.rs` 29).
fileprivate let MULTIBYTE_CHAR_KEYSTROKE_EVENT: UInt32 = 2

// ==========================================================================
// Round 33: `EmacsView`'s composing pair and its three light Cocoa-arm methods
// ==========================================================================

/// C: `NS_TEXT_EVENT` - the `enum event_kind` member beside the `NS_NONKEY_EVENT` round 28
/// counted (C's `src/termhooks.h` 273-279 gates both on `HAVE_NS`).  It is the same count, one
/// step earlier, and carries the same **UNVERIFIED** mark for the same reason.
fileprivate let NS_TEXT_EVENT: UInt32 = 29

/// C `src/nsterm.h` 801/802: `KEY_NS_PUT_WORKING_TEXT ((1<<28)|(0<<16)|9)` and
/// `KEY_NS_UNPUT_WORKING_TEXT (…|10)` - the two `code` values the composing methods write.
let KEY_NS_PUT_WORKING_TEXT = (1 << 28) | (0 << 16) | 9
let KEY_NS_UNPUT_WORKING_TEXT = (1 << 28) | (0 << 16) | 10

/// C: `Lisp_Object ns_working_text` - this file's own `DEFVAR_LISP` (`ns-working-text`, C 11604),
/// imported as the data symbol the core exports (the `ns_confirm_quit` shape).
@_silgen_name("ns_working_text") var ns_working_text: Lisp_Object

/// C `src/coding.h` 674/680: `#define UTF_16_HIGH_SURROGATE_P(val) (((val) & 0xFC00) == 0xD800)`
/// and the low-surrogate twin.  The port's mirror spells them as plain functions
/// (`femacs/src/include/coding.rs` 304/308) rather than C-ABI entries, so this unit writes them
/// out as the macro bodies state them.
fileprivate func UTF_16_HIGH_SURROGATE_P(_ val: UInt32) -> Bool {
  (val & 0xFC00) == 0xD800                             // C coding.h 675
}
fileprivate func UTF_16_LOW_SURROGATE_P(_ val: UInt32) -> Bool {
  (val & 0xFC00) == 0xDC00                             // C coding.h 681
}

/// C `src/coding.h` 760-768: `INLINE int surrogates_to_codepoint (int low, int high)` - "Return
/// the Unicode code point for the given UTF-16 surrogates."  C's four `eassert`s are the no-ops of
/// an unset `ENABLE_CHECKING`, and the expression is C 767's.
fileprivate func surrogates_to_codepoint(_ low: UInt32, _ high: UInt32) -> UInt32 {
  0x10000 + (low - 0xDC00) + ((high - 0xD800) * 0x400) // C coding.h 767
}

// ==========================================================================
// Round 34: `ns_in_echo_area` and `firstRectForCharacterRange:`
// ==========================================================================

/// C: `typedef struct { ptrdiff_t bytes; } specpdl_ref;` (src/lisp.h 3646) - this unit's own view
/// of it, the way `nsselect.swift` 87 and `nsfns.swift` 97 each carry theirs (a Swift unit
/// compiles alone, so each spells what it reads).
struct specpdl_ref {
  var bytes: Int
}

/// C: `Lisp_Object FRAME_SELECTED_WINDOW (struct frame *f)` (src/frame.h 1204, `f->selected_window`)
/// - the entry `nsfns.swift` 505 already declares.
@_silgen_name("frame_selected_window")
func frame_selected_window(_ f: UnsafeMutableRawPointer?) -> Lisp_Object

/// C: `extern Lisp_Object echo_area_window;` (src/window.h; the port's mirror declares it at
/// `femacs/src/include/window.rs` 280 as an `extern "C"` static).
@_silgen_name("echo_area_window") var echo_area_window: Lisp_Object

/// C: `Lisp_Object Qns_in_echo_area` - this file's own `DEFSYM ("ns-in-echo-area")` at C 11799,
/// imported as the data symbol the core exports (the `ns_working_text` shape).
@_silgen_name("Qns_in_echo_area") var Qns_in_echo_area: Lisp_Object

/// C: `Lisp_Object internal_catch_all (Lisp_Object (*) (void *), void *,
/// Lisp_Object (*) (enum nonlocal_exit, Lisp_Object))` (src/lisp.h) - the port defines it in
/// `femacs/src/eval.rs`; the two callbacks are `@convention(c)` pointers and the exit enum is the
/// integer it is.
typealias catch_all_func = @convention(c) (UnsafeMutableRawPointer?) -> Lisp_Object
typealias catch_all_handler = @convention(c) (Int32, Lisp_Object) -> Lisp_Object
@_silgen_name("internal_catch_all")
func internal_catch_all(_ func: catch_all_func?, _ arg: UnsafeMutableRawPointer?,
                        _ handler: catch_all_handler?) -> Lisp_Object

/// C: `Lisp_Object safe_funcall (ptrdiff_t, Lisp_Object *)` (src/lisp.h 4845) - what C's
/// `safe_calln (Qns_in_echo_area)` macro (lisp.h 4846-4847) expands to for one argument.
@_silgen_name("safe_funcall")
func safe_funcall(_ nargs: Int, _ args: UnsafeMutablePointer<Lisp_Object>?) -> Lisp_Object

/// C: the `WINDOWP`, `SPECPDL_INDEX`, `specbind` and `unbind_to` entries the two functions below
/// read; each is the shape this unit's neighbours already use (`nsfns.swift` 191/716/721,
/// `nsselect.swift` 147/174).
@_silgen_name("WINDOWP") func WINDOWP(_ x: Lisp_Object) -> Bool
@_silgen_name("SPECPDL_INDEX") func SPECPDL_INDEX() -> specpdl_ref
@_silgen_name("specbind") func specbind(_ symbol: Lisp_Object, _ value: Lisp_Object)
@_silgen_name("unbind_to") func unbind_to(_ count: specpdl_ref, _ value: Lisp_Object) -> Lisp_Object
@_silgen_name("Qinhibit_quit") func Qinhibit_quit() -> Lisp_Object

/// C 7623-7630: `static Lisp_Object ns_in_echo_area_1 (void *ptr)`.
fileprivate func ns_in_echo_area_1(_ ptr: UnsafeMutableRawPointer?) -> Lisp_Object {
  let count = SPECPDL_INDEX()                         // C 7626
  specbind(Qinhibit_quit(), Qt())                     // C 7627
  var args: [Lisp_Object] = [Qns_in_echo_area]        // C 7628 (C's `safe_calln`, one argument)
  let in_echo_area = safe_funcall(1, &args)
  return unbind_to(count, in_echo_area)               // C 7629
}

/// C 7632-7636: `static Lisp_Object ns_in_echo_area_2 (enum nonlocal_exit exit, Lisp_Object
/// error)` - C's whole body is `return Qnil;`.
fileprivate func ns_in_echo_area_2(_ exit: Int32, _ error: Lisp_Object) -> Lisp_Object {
  Qnil()                                              // C 7635
}

/// C 7638-7648: `static bool ns_in_echo_area (void)` - the `internal_catch_all` wrapper, whose
/// three arguments are C 7644-7645's two helpers and its `NULL`.
fileprivate func ns_in_echo_area() -> Bool {
  let in_echo_area = internal_catch_all(
    { ptr in ns_in_echo_area_1(ptr) },
    nil,
    { exit, error in ns_in_echo_area_2(exit, error) })   // C 7643-7645
  return !NILP(in_echo_area)                           // C 7647
}

// ==========================================================================
// Round 35: `ns_free_pixmap`, the two display metrics, and `ns_string_to_lispmod`
// ==========================================================================

/// C: the five modifier symbols `ns_string_to_lispmod` compares against, all `DEFSYM`s of this
/// file's own `syms_of_nsterm` and imported in the `Qnone` shape the unit already uses.
@_silgen_name("Qmeta") func Qmeta() -> Lisp_Object
@_silgen_name("Qsuper") func Qsuper() -> Lisp_Object
@_silgen_name("Qcontrol") func Qcontrol() -> Lisp_Object
@_silgen_name("Qalt") func Qalt() -> Lisp_Object
@_silgen_name("Qhyper") func Qhyper() -> Lisp_Object

/// C: `int strncmp (const char *, const char *, size_t)` (string.h) - the `fputs` shape.
@_silgen_name("strncmp")
func strncmp(_ a: UnsafePointer<CChar>?, _ b: UnsafePointer<CChar>?, _ n: Int) -> Int32

/// C 5648-5652: `static void ns_free_pixmap (struct frame *_f, Emacs_Pixmap pixmap)` - C's whole
/// body is the `ns_release_object` call, which is this unit's own `@_cdecl` entry (C 602-638).
/// The pixmap parameter is therefore spelled the way that export spells its own (C 805-811): an
/// optional untyped pointer (src/nsgui.h 55-59's `typedef id` arm, read untyped as the round 18
/// `ns_display_info_bitmaps_img` note reads it), and because the export is what carries C's
/// nil-receiver no-op (C 808-810), a NULL pixmap needs no arm here.
fileprivate func ns_free_pixmap(_ _f: UnsafeMutableRawPointer?, _ pixmap: UnsafeMutableRawPointer?) {
  ns_release_object(pixmap)                          // C 5651
}

/// C 5660-5673: `int ns_display_pixel_height (struct ns_display_info *dpyinfo)` - the union of
/// every screen's frame, which C builds with an enumerator; `NSScreen.screens` is the array and
/// `NSHeight` is the rect's own height.
func ns_display_pixel_height(_ dpyinfo: UnsafeMutableRawPointer?) -> Int32 {
  let screens = NSScreen.screens                     // C 5663
  var frame = NSRect.zero                            // C 5668
  for screen in screens {                            // C 5669
    frame = NSUnionRect(frame, screen.frame)         // C 5670
  }
  return Int32(frame.height)                         // C 5672
}

/// C 5675-5688: `int ns_display_pixel_width (struct ns_display_info *dpyinfo)` - the same union,
/// read as a width.
func ns_display_pixel_width(_ dpyinfo: UnsafeMutableRawPointer?) -> Int32 {
  let screens = NSScreen.screens                     // C 5678
  var frame = NSRect.zero                            // C 5683
  for screen in screens {                            // C 5684
    frame = NSUnionRect(frame, screen.frame)         // C 5685
  }
  return Int32(frame.width)                          // C 5687
}

/// C 5691-5710: `static Lisp_Object ns_string_to_lispmod (const char *s)` - "Convert modifier name
/// to lisp symbol."  C's six `strncmp (SSDATA (SYMBOL_NAME (Q…)), s, 10)` tests keep their order
/// and their length 10; `SYMBOL_NAME`/`SSDATA` are entries this unit already declares (C 2255 and
/// 2254) and `Qnone` is its own (C 3092).
fileprivate func ns_string_to_lispmod(_ s: UnsafePointer<CChar>) -> Lisp_Object {
  if strncmp(SSDATA(SYMBOL_NAME(Qmeta())), s, 10) == 0 { return Qmeta() }        // C 5696-5697
  if strncmp(SSDATA(SYMBOL_NAME(Qsuper())), s, 10) == 0 { return Qsuper() }      // C 5698-5699
  if strncmp(SSDATA(SYMBOL_NAME(Qcontrol())), s, 10) == 0 { return Qcontrol() }  // C 5700-5701
  if strncmp(SSDATA(SYMBOL_NAME(Qalt())), s, 10) == 0 { return Qalt() }          // C 5702-5703
  if strncmp(SSDATA(SYMBOL_NAME(Qhyper())), s, 10) == 0 { return Qhyper() }      // C 5704-5705
  if strncmp(SSDATA(SYMBOL_NAME(Qnone())), s, 10) == 0 { return Qnone() }        // C 5706-5707
  return Qnil()                                      // C 5709
}


// ==========================================================================
// Round 36: `ns_default`, `ns_default_font_parameter`, and the GNUstep-only
// `ns_update_window_end` (C 5713-5783)
// ==========================================================================

/// C: `extern const char *ns_get_defaults_value (const char *key)` (src/nsterm.h 1170) - the
/// definition is src/nsfns.m 1886's, which this lane's `nsfns.swift` (2858-2865) already exports
/// under the same name, and this unit reaches it the way that export spells it (a mutable pointer,
/// the shape `@_cdecl` accepts in Swift 6.4).
@_silgen_name("ns_get_defaults_value")
func ns_get_defaults_value(_ key: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

/// C: `int c_strcasecmp (const char *, const char *)` - gnulib's entry, ported in this tree as
/// `femacs/lib/c-strcasecmp.rs` 22 (`pub unsafe extern "C" fn c_strcasecmp`); the `fputs` shape.
@_silgen_name("c_strcasecmp")
func c_strcasecmp(_ a: UnsafePointer<CChar>, _ b: UnsafePointer<CChar>) -> Int32

/// C: `double strtod (const char *nptr, char **endptr)` (stdlib.h).  Unlike `fprintf`, this entry
/// is *available* in Swift (Darwin imports it, and it is the same C symbol), so the unit calls it
/// by name with no declaration of its own - the `setlocale`/`getenv`/`setenv` treatment at
/// C 785-805 above.  A local `@_silgen_name("strtod")` next to that import made the call
/// ambiguous (measured: `error: ambiguous use of 'strtod'`).

/// C: `Lisp_Object make_float (double)` (src/alloc.c) - the mirror's own entry
/// (`femacs/src/alloc.rs` 2789), reached by name like this unit's other core imports.
@_silgen_name("make_float") func make_float(_ f: Double) -> Lisp_Object

/// C 5713-5738: `static void ns_default (const char *parameter, Lisp_Object *result, Lisp_Object
/// yesval, Lisp_Object noval, BOOL is_float, BOOL is_modstring)` - "Check a parameter value in
/// user's preferences."  C's two `BOOL`s are this unit's `Bool` (the `ns_is_UAZoomEnabled` 1274
/// reading), and the nil test followed by `value!` is the `Fns_get_resource` shape
/// (`nsfns.swift` 2883-2887).
fileprivate func ns_default(_ parameter: UnsafePointer<CChar>,
                            _ result: UnsafeMutablePointer<Lisp_Object>,
                            _ yesval: Lisp_Object, _ noval: Lisp_Object,
                            _ is_float: Bool, _ is_modstring: Bool) {
  let value = ns_get_defaults_value (parameter)      // C 5721

  if value != nil {                                  // C 5723
    var f: Double = 0                                // C 5725
    var pos: UnsafeMutablePointer<CChar>? = nil      // C 5726

    /// C 5731's comma expression `(f = strtod (value, &pos), pos != value)`: Swift has no comma
    /// operator, so its two halves are one local function whose body is C's own two steps in C's
    /// order - the `ns_after_update_window_line` note's reshaping, narrowed to the one expression
    /// that cannot become a nested `if` without breaking the chain's fall-through.  The call sits
    /// in the condition below, so C's short-circuit is kept: `is_float` false never runs the
    /// assignment.
    func strtod_and_test () -> Bool {
      f = strtod (value!, &pos)                      // C 5731 (first half)
      return pos != value                            // C 5731 (second half)
    }

    if c_strcasecmp (value!, "YES") == 0 {           // C 5727
      result.pointee = yesval                        // C 5728
    } else if c_strcasecmp (value!, "NO") == 0 {     // C 5729
      result.pointee = noval                         // C 5730
    } else if is_float && strtod_and_test () {       // C 5731
      result.pointee = make_float (f)                // C 5732
    } else if is_modstring && value != nil {         // C 5733
      result.pointee = ns_string_to_lispmod (value!) // C 5734
    } else {
      fprintf (stderr, "Bad value for default \"%s\": \"%s\"\n",
               parameter, value!)                    // C 5735-5736
    }
  }
}

/// C 5769-5773: `static void ns_default_font_parameter (struct frame *f, Lisp_Object parms)` -
/// C's body is empty, and the comment above it (C 5766-5768) is why: the hook "currently does
/// nothing, since it's only really needed when changing the font-backend, but macOS currently
/// only has one possible backend."  Both parameters are unread in C, so neither is read here
/// (the `applicationWillBecomeActive` reading: a body C leaves empty is transcribed as empty).
fileprivate func ns_default_font_parameter(_ f: UnsafeMutableRawPointer?, _ parms: Lisp_Object) {
  // C 5771-5773: nothing - see the note above the definition.
}

/// C 5775-5783: `static void ns_update_window_end (struct window *w, bool cursor_on_p, bool
/// mouse_face_overwritten_p)` - **dead in this build**: C opens `#ifdef NS_IMPL_GNUSTEP` at C 5774
/// and closes it at C 5783 (src/nsterm.h's `NS_IMPL_COCOA` is the arm this target takes), so the
/// whole definition is the arm that does not exist here.  Recorded, not transcribed - no Swift
/// item carries the name, and the ledger's box says the same (the `-updateMonitors:` reading of
/// round 31).  The definition's two statements would be `NSTRACE`, empty at `NSTRACE_ENABLED` 0,
/// and `ns_redraw_scroll_bars (WINDOW_XFRAME (w))`.

// ==========================================================================
// Round 37: `ns_initialize_display_info` (C 5741-5764)
// ==========================================================================

/// C: the `struct ns_display_info` members this definition writes, as member entries of the
/// `<record>_<member>` family round 7 settled (`docs/ports/nsmenu.swift` 311-328): five `int`
/// members, two `double`s and `Window`.  src/nsgui.h 67 is `typedef int Window;` on this arm
/// (measured in the frozen layout: `docs/baseline/ns_display_info-layout.tsv` 27 puts
/// `root_window` at offset 92 with `int color_p` at 88), so the root-window entry takes an `Int32`
/// like its neighbours.
@_silgen_name("ns_display_info_set_resx")
func ns_display_info_set_resx(_ f: UnsafeMutableRawPointer?, _ v: Double)
@_silgen_name("ns_display_info_set_resy")
func ns_display_info_set_resy(_ f: UnsafeMutableRawPointer?, _ v: Double)
@_silgen_name("ns_display_info_set_color_p")
func ns_display_info_set_color_p(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_display_info_set_n_planes")
func ns_display_info_set_n_planes(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_display_info_set_root_window")
func ns_display_info_set_root_window(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_display_info_set_n_fonts")
func ns_display_info_set_n_fonts(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_display_info_set_smallest_font_height")
func ns_display_info_set_smallest_font_height(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_display_info_set_smallest_char_width")
func ns_display_info_set_smallest_char_width(_ f: UnsafeMutableRawPointer?, _ v: Int32)

/// C: `&dpyinfo->mouse_highlight` (C 5763) - the `mouse_highlight` member's *address*, which is
/// what `reset_mouse_highlight` takes.  It is the same member entry as the readers above; C's `&`
/// is this record's because the entry returns the member's own address rather than a copy of the
/// `Mouse_HLInfo` struct.
@_silgen_name("ns_display_info_mouse_highlight")
func ns_display_info_mouse_highlight(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?

/// C 5741-5764: `static void ns_initialize_display_info (struct ns_display_info *dpyinfo)` -
/// "Initialize global info and storage for display."  C 5758's chained assignment is written in
/// C's own order (`a = b = NULL` assigns `b` first), and C 5752-5755's two `isEqualToString:`
/// messages are the `==` the unit's other AppKit string comparisons use, kept as two calls to
/// `NSColorSpaceFromDepth` because C makes two.
fileprivate func ns_initialize_display_info(_ dpyinfo: UnsafeMutableRawPointer?) {
  let screen = NSScreen.main                             // C 5747
  // C 5748: `[screen depth]` - a message to a nil screen answers 0, the reading round 2's
  // `?? NSRect.zero` gives the same receiver.  Swift spells the result `NSWindow.Depth` (the
  // `NSWindowDepth` typedef) and its raw-value initializer is *failable*, so C's own zero is
  // written as `rawValue: 0` plus the unwrap that follows from the initializer's type; that 0 is
  // a representable depth was measured on this machine (`NSWindow.Depth (rawValue: 0) != nil`).
  let depth = screen?.depth ?? NSWindow.Depth (rawValue: 0)!

  ns_display_info_set_resx (dpyinfo, 72.27)              // C 5750
  ns_display_info_set_resy (dpyinfo, 72.27)              // C 5751

  /* C 5752-5755: `NSDeviceWhiteColorSpace` and `NSCalibratedWhiteColorSpace` are
     `NSColorSpaceName.deviceWhite`/`.calibratedWhite` in Swift, `NSColorSpaceFromDepth (depth)`
     is `depth.colorSpaceName` (the compiler's own rename readings - class B), and the member is
     an `int` while the expression is a `bool`, so C's integral conversion (false -> 0, true -> 1)
     is spelled out.  Two reads of `colorSpaceName` stay two, as C's two calls are two.  */
  let color_p = !(NSColorSpaceName.deviceWhite == depth.colorSpaceName)
    && !(NSColorSpaceName.calibratedWhite == depth.colorSpaceName)
  ns_display_info_set_color_p (dpyinfo, color_p ? 1 : 0)

  // C 5756: `NSBitsPerPixelFromDepth (depth)` is `depth.bitsPerPixel`.
  ns_display_info_set_n_planes (dpyinfo, Int32 (depth.bitsPerPixel))
  ns_display_info_set_root_window (dpyinfo, 42)          // C 5757 (C's placeholder)
  ns_display_info_set_ns_focus_frame (dpyinfo, nil)      // C 5758 (the inner assignment)
  ns_display_info_set_highlight_frame (dpyinfo, nil)     // C 5758 (the outer one, C's order)
  ns_display_info_set_n_fonts (dpyinfo, 0)               // C 5759
  ns_display_info_set_smallest_font_height (dpyinfo, 1)  // C 5760
  ns_display_info_set_smallest_char_width (dpyinfo, 1)   // C 5761

  reset_mouse_highlight (ns_display_info_mouse_highlight (dpyinfo))  // C 5763
}

// ==========================================================================
// Round 38: `ns_flush_display` and the empty `ns_delete_display` (C 5785-5875)
// ==========================================================================

/// C: `struct terminal *FRAME_TERMINAL (struct frame *f)` (src/frame.h) - a macro entry under the
/// construct's own name, the `FRAME_DISPLAY_INFO` shape above (the `frame_set_terminal` writer
/// `nsfns.swift` 639 carries is the same member's other direction).
@_silgen_name("FRAME_TERMINAL")
func FRAME_TERMINAL(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?

/// C 5785-5792: `static void ns_flush_display (struct frame *f)` - C's body is `EVENT_INIT` and one
/// `ns_read_socket_1` call whose value C discards.  The record's own initializer is round 3's
/// (`input_event` at C 5121, the same nine members), and the third argument is C's `YES`.
fileprivate func ns_flush_display(_ f: UnsafeMutableRawPointer?) {
  var ie = input_event(bits: 0, code: 0, modifiers: 0, x: nil, y: nil, timestamp: 0,
                       frame_or_window: nil, arg: nil, device: nil)      // C 5788
  EVENT_INIT(&ie)                                                        // C 5790
  _ = ns_read_socket_1(FRAME_TERMINAL(f), &ie, true)                     // C 5791
}

/// C 5871-5875: `static void ns_delete_display (struct ns_display_info *dpyinfo)` - C's whole body
/// is the comment `/* TODO...  */`, so the body is empty here too and the parameter is unread
/// (the `ns_default_font_parameter` reading of round 36: a body C leaves empty is transcribed as
/// empty rather than stubbed).
fileprivate func ns_delete_display(_ dpyinfo: UnsafeMutableRawPointer?) {
  // C 5874: nothing - C's body is the TODO comment above the definition.
}

// ==========================================================================
// Round 39: the redisplay interface table (C 5801-5835)
// ==========================================================================

/// C: `typedef void (*frame_parm_handler) (struct frame *, Lisp_Object, Lisp_Object);`
/// (src/dispextern.h 3063) - this unit's own view of the typedef, which `nsfns.swift` 415 spells
/// for its own unit (a Swift typealias is not visible across the lane's single-file compiles).
typealias frame_parm_handler =
  @convention(c) (UnsafeMutableRawPointer?, Lisp_Object, Lisp_Object) -> Void

/// C: `struct redisplay_interface` (src/dispextern.h) - **this unit's view, in the arm it ports**
/// (the `NS_IMPL_COCOA` one).  The Rust mirror `femacs/src/include/dispextern.rs` 3794 is the
/// *frozen* configuration's shape, which is 13 members wide because that configuration leaves
/// `HAVE_WINDOW_SYSTEM` false; the Cocoa arm C 5801-5835 initialises **28** members, and the
/// measured layout for it is `docs/baseline/redisplay_interface-layout.tsv`'s `ns_cocoa` rows:
/// `sizeof 224`, `align 8`, members at 0, 8, 16, … 216, i.e. one pointer each, in the order below.
/// Every member is a function pointer, so each is an *optional* `@convention(c)` type - C's
/// `NULL` is the `nil` the table's two `NULL`s use - and `frame_parm_handlers` is the
/// `frame_parm_handler *` C declares (offset 0).
struct redisplay_interface {
  var frame_parm_handlers: UnsafeMutablePointer<frame_parm_handler>?
  var produce_glyphs: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
  var write_glyphs: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?,
                                     UnsafeMutableRawPointer?, Int32, Int32) -> Void)?
  var insert_glyphs: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?,
                                      UnsafeMutableRawPointer?, Int32, Int32) -> Void)?
  var clear_end_of_line: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?,
                                          Int32, Int32) -> Void)?
  var scroll_run_hook: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void)?
  var after_update_window_line_hook: (@convention(c) (UnsafeMutableRawPointer?,
                                                      UnsafeMutableRawPointer?) -> Void)?
  var update_window_begin_hook: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
  var update_window_end_hook: (@convention(c) (UnsafeMutableRawPointer?, Bool, Bool) -> Void)?
  var flush_display: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
  var clear_window_mouse_face: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
  var get_glyph_overhangs: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?,
                                            UnsafeMutablePointer<Int32>?,
                                            UnsafeMutablePointer<Int32>?) -> Void)?
  var fix_overlapping_area: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?,
                                             Int32, Int32) -> Void)?
  var draw_fringe_bitmap: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?,
                                           UnsafeMutableRawPointer?) -> Void)?
  var define_fringe_bitmap: (@convention(c) (Int32, UnsafePointer<UInt16>?, Int32,
                                             Int32) -> Void)?
  var destroy_fringe_bitmap: (@convention(c) (Int32) -> Void)?
  var compute_glyph_string_overhangs: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
  var draw_glyph_string: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
  var define_frame_cursor: (@convention(c) (UnsafeMutableRawPointer?, AnyObject?) -> Void)?
  var clear_frame_area: (@convention(c) (UnsafeMutableRawPointer?, Int32, Int32, Int32,
                                         Int32) -> Void)?
  var clear_under_internal_border: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
  var draw_window_cursor: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?,
                                           Int32, Int32, Int32, Int32, Bool, Bool) -> Void)?
  var draw_vertical_window_border: (@convention(c) (UnsafeMutableRawPointer?, Int32, Int32,
                                                    Int32) -> Void)?
  var draw_window_divider: (@convention(c) (UnsafeMutableRawPointer?, Int32, Int32, Int32,
                                            Int32) -> Void)?
  var shift_glyphs_for_insert: (@convention(c) (UnsafeMutableRawPointer?, Int32, Int32, Int32,
                                                Int32, Int32) -> Void)?
  var show_hourglass: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
  var hide_hourglass: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
  var default_font_parameter: (@convention(c) (UnsafeMutableRawPointer?, Lisp_Object) -> Void)?
}

/// C: the six `gui_*` entries C 5804-5819 hands the table, which are `src/xdisp.c`'s
/// (the xdisp lane's) - imported by name with C's own signatures, as the unit imports the other
/// core entries (`src/xdisp.c` 33768, 34548, 34592, 34649, 35052, 31409, 37706).
@_silgen_name("gui_produce_glyphs")
func gui_produce_glyphs(_ it: UnsafeMutableRawPointer?)
@_silgen_name("gui_write_glyphs")
func gui_write_glyphs(_ w: UnsafeMutableRawPointer?, _ updated_row: UnsafeMutableRawPointer?,
                      _ string: UnsafeMutableRawPointer?, _ area: Int32, _ len: Int32)
@_silgen_name("gui_insert_glyphs")
func gui_insert_glyphs(_ w: UnsafeMutableRawPointer?, _ updated_row: UnsafeMutableRawPointer?,
                       _ start: UnsafeMutableRawPointer?, _ area: Int32, _ len: Int32)
@_silgen_name("gui_clear_end_of_line")
func gui_clear_end_of_line(_ w: UnsafeMutableRawPointer?, _ updated_row: UnsafeMutableRawPointer?,
                           _ area: Int32, _ x: Int32)
@_silgen_name("gui_clear_window_mouse_face")
func gui_clear_window_mouse_face(_ w: UnsafeMutableRawPointer?)
@_silgen_name("gui_get_glyph_overhangs")
func gui_get_glyph_overhangs(_ glyph: UnsafeMutableRawPointer?, _ f: UnsafeMutableRawPointer?,
                             _ left: UnsafeMutablePointer<Int32>?,
                             _ right: UnsafeMutablePointer<Int32>?)
@_silgen_name("gui_fix_overlapping_area")
func gui_fix_overlapping_area(_ w: UnsafeMutableRawPointer?, _ row: UnsafeMutableRawPointer?,
                              _ area: Int32, _ x: Int32)

/// C: `extern frame_parm_handler ns_frame_parm_handlers[]` (src/nsterm.h 1262) - defined in
/// src/nsfns.m 1058, which this lane's `nsfns.swift` 2070 carries as a Swift array and whose
/// ledger records that **the C symbol itself still needs a port-side definition**
/// (`docs/ports/nsfns.md`'s `## ABI` (3); Swift cannot emit a C-named data symbol).  C's table
/// entry is the array's address - what the array name decays to - so this unit takes the imported
/// symbol's own address.  A unit cannot see another unit's Swift array; this entry is the C
/// symbol's side, and it is the reason the member below is written the way it is
/// (see the round's note on that one reshaping).
@_silgen_name("ns_frame_parm_handlers") var ns_frame_parm_handlers_entry: frame_parm_handler

/// C 5801-5835: `static struct redisplay_interface ns_redisplay_interface = { … }` - C's own
/// order, one line per entry, with C's comments kept where they say something C's code does not
/// (`generic OK`, the two `NULL`s and their arm).  Swift cannot emit a C-named *data* symbol
/// either, so the port carries the table as a file-scope value and the ledger's `## ABI` records
/// that the C symbol needs a port-side definition - the same standing state `nsfns.swift` 2070's
/// array has.
///
/// Two entries are the arm's `NULL`s (C 5810's `update_window_begin` and C 5812's
/// `update_window_end`, the latter because C 5811 tests `#ifndef NS_IMPL_GNUSTEP` and this target
/// takes the `NULL` side), so they are `nil`.
fileprivate var ns_redisplay_interface = redisplay_interface(
  frame_parm_handlers:
    withUnsafeMutablePointer(to: &ns_frame_parm_handlers_entry) { $0 },     // C 5803
  produce_glyphs: { it in gui_produce_glyphs(it) },                        // C 5804
  write_glyphs: { w, row, string, area, len in                             // C 5805
    gui_write_glyphs(w, row, string, area, len) },
  insert_glyphs: { w, row, start, area, len in                             // C 5806
    gui_insert_glyphs(w, row, start, area, len) },
  clear_end_of_line: { w, row, area, x in                                  // C 5807
    gui_clear_end_of_line(w, row, area, x) },
  scroll_run_hook: { w, run in ns_scroll_run(w, run) },                    // C 5808
  after_update_window_line_hook: { w, row in                               // C 5809
    ns_after_update_window_line(w, row) },
  update_window_begin_hook: nil,                                           // C 5810 (C's NULL)
  update_window_end_hook: nil,                                             // C 5812 (C's NULL)
  flush_display: { f in ns_flush_display(f) },                             // C 5816
  clear_window_mouse_face: { w in gui_clear_window_mouse_face(w) },        // C 5817
  get_glyph_overhangs: { glyph, f, left, right in                          // C 5818
    gui_get_glyph_overhangs(glyph, f, left, right) },
  fix_overlapping_area: { w, row, area, x in                               // C 5819
    gui_fix_overlapping_area(w, row, area, x) },
  draw_fringe_bitmap: { w, row, p in ns_draw_fringe_bitmap(w, row, p) },    // C 5820
  define_fringe_bitmap: { which, bits, h, wd in                            // C 5821
    ns_define_fringe_bitmap(which, bits, h, wd) },
  destroy_fringe_bitmap: { which in ns_destroy_fringe_bitmap(which) },      // C 5822
  compute_glyph_string_overhangs: { s in                                    // C 5823
    ns_compute_glyph_string_overhangs(s) },
  draw_glyph_string: { s in ns_draw_glyph_string(s) },                      // C 5824
  define_frame_cursor: { f, cursor in                                       // C 5825
    ns_define_frame_cursor(f, cursor as? NSObject) },
  clear_frame_area: { f, x, y, width, height in                             // C 5826
    ns_clear_frame_area(f, x, y, width, height) },
  clear_under_internal_border: { f in ns_clear_under_internal_border(f) },  // C 5827
  draw_window_cursor: { w, row, x, y, kind, width, on_p, active_p in        // C 5828
    ns_draw_window_cursor(w, row, x, y, kind, width, on_p, active_p) },
  draw_vertical_window_border: { w, x, y0, y1 in                            // C 5829
    ns_draw_vertical_window_border(w, x, y0, y1) },
  draw_window_divider: { w, x0, x1, y0, y1 in                               // C 5830
    ns_draw_window_divider(w, x0, x1, y0, y1) },
  shift_glyphs_for_insert: { f, x, y, width, height, shift_by in            // C 5831
    ns_shift_glyphs_for_insert(f, x, y, width, height, shift_by) },
  show_hourglass: { f in ns_show_hourglass(f) },                            // C 5832
  hide_hourglass: { f in ns_hide_hourglass(f) },                            // C 5833
  default_font_parameter: { f, parms in                                     // C 5834
    ns_default_font_parameter(f, parms) })

// ==========================================================================
// Round 40: `ns_displays_reconfigured` (C 5837-5868)
// ==========================================================================

/// C: `union buffered_input_event` (src/keyboard.h 228-235) - the kbd buffer's element.  Its
/// members all begin at offset 0, and this target's arm has only two of them: the `kind` bitfield
/// and `struct input_event ie` (the third, `sie`, is `HAVE_X11`/`HAVE_PGTK`'s).  C's
/// `ev->ie.kind` therefore reads the same low 16 bits the record above already reads, so this
/// unit's view carries `ie` and reads the field through `input_event_kind` - the reader the
/// record's own view documents (`input_event`'s note says the two bitfields share one `UInt32`).
struct buffered_input_event {
  /// C: `struct input_event ie;` (src/keyboard.h 231)
  var ie: input_event
}

/// C: `enum { … KBD_BUFFER_SIZE = 4096 };` (src/keyboard.h 380-383).  An enumerator the frozen
/// macro table cannot decide, so the value is this tree's own mirror's
/// (`femacs/src/include/keyboard.rs` 43, `pub const KBD_BUFFER_SIZE: c_int = 4096`) - the reading
/// round 33 used for `MULTIBYTE_CHAR_KEYSTROKE_EVENT`.
fileprivate let KBD_BUFFER_SIZE: Int32 = 4096

/// C: `MONITORS_CHANGED_EVENT` (src/termhooks.h 342) - the same reading as the constant above,
/// and for the same reason: the value comes from the mirror
/// (`femacs/src/include/termhooks.rs` 62,
/// `pub const MONITORS_CHANGED_EVENT: event_kind = 35`), because C's enum has guarded members
/// before it (`HAVE_DBUS`, `THREADS_ENABLED`, …), which is why the frozen macro table cannot
/// decide it either.
fileprivate let MONITORS_CHANGED_EVENT: UInt32 = 35

/// C: the kbd buffer and its two pointers - `extern union buffered_input_event
/// kbd_buffer[KBD_BUFFER_SIZE];`, `*kbd_fetch_ptr`, `*kbd_store_ptr` (src/keyboard.h 455-457).
/// The core owns all three as `#[unsafe(no_mangle)] static mut` (`femacs/src/keyboard.rs`
/// 178-186), so they are data entries; `kbd_buffer` is declared with the *element* type, because
/// C's own use of the array name here is its address (`kbd_buffer + KBD_BUFFER_SIZE - 1`), and
/// the address is taken the way round 39 takes the handlers array's (`withUnsafeMutablePointer
/// (to:)`; Swift cannot emit or address another unit's Swift array).
@_silgen_name("kbd_buffer") var kbd_buffer_entry: buffered_input_event
@_silgen_name("kbd_fetch_ptr")
var kbd_fetch_ptr_entry: UnsafeMutablePointer<buffered_input_event>?
@_silgen_name("kbd_store_ptr")
var kbd_store_ptr_entry: UnsafeMutablePointer<buffered_input_event>?

/// C: `void kbd_buffer_store_event (struct input_event *)` - already this unit's own entry (the
/// block at C 1373's line, declared for that round's `ns_send_appdefined` path); C 5867's call
/// reuses it rather than redeclaring it.

/// C: `Lisp_Object Fequal (Lisp_Object, Lisp_Object)` (src/fns.c) - declared by the core's globals
/// mirror (`femacs/src/include/globals.rs` 7577); the `Fns_get_resource` import shape.
@_silgen_name("Fequal") func Fequal(_ o1: Lisp_Object, _ o2: Lisp_Object) -> Lisp_Object

/// C: `Lisp_Object Fns_display_monitor_attributes_list (Lisp_Object)` (a `DEFUN` of src/nsfns.m,
/// C 3135) - the `nsfns` lane's export (`nsfns.swift` 3669), reached the `Fns_get_resource` way.
@_silgen_name("Fns_display_monitor_attributes_list")
func Fns_display_monitor_attributes_list(_ terminal: Lisp_Object) -> Lisp_Object

/// C: `XSETTERMINAL (a, b)` = `XSETPSEUDOVECTOR (a, b, PVEC_TERMINAL)` (src/lisp.h 1384) - a macro
/// entry under the construct's own name, the `XSETFRAME` shape above.
@_silgen_name("XSETTERMINAL")
func XSETTERMINAL(_ a: UnsafeMutablePointer<Lisp_Object>, _ b: UnsafeMutableRawPointer?)

/// C: the `struct ns_display_info` member `x_display_list->terminal` (this file's static at
/// C 305 is the record's head) - a reader entry of the `ns_display_info_<member>` family round 7
/// settled and round 37 added setters to.
@_silgen_name("ns_display_info_terminal")
func ns_display_info_terminal(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?

/// C 5837-5868: `static void ns_displays_reconfigured (CGDirectDisplayID display,
/// CGDisplayChangeSummaryFlags flags, void *user_info)` - the `NS_IMPL_COCOA` arm (C 5837 opens
/// the guard, C 5869 closes it).  C's three parameters are unread in its body, as are the two
/// `CGDirectDisplayID`/flags types' uses elsewhere; the buffer walk keeps C's own arithmetic and
/// its two early returns.
fileprivate func ns_displays_reconfigured(_ display: CGDirectDisplayID,
                                          _ flags: CGDisplayChangeSummaryFlags,
                                          _ user_info: UnsafeMutableRawPointer?) {
  var ie = input_event(bits: 0, code: 0, modifiers: 0, x: nil, y: nil, timestamp: 0,
                       frame_or_window: nil, arg: nil, device: nil)   // C 5843
  var ev: UnsafeMutablePointer<buffered_input_event>? = nil           // C 5844
  var new_monitors: Lisp_Object = nil                                 // C 5845

  EVENT_INIT(&ie)                                                     // C 5847

  new_monitors = Fns_display_monitor_attributes_list(Qnil())          // C 5849

  if !NILP(Fequal(new_monitors, last_known_monitors)) {               // C 5851
    return
  }

  last_known_monitors = new_monitors                                  // C 5854

  // C 5856-5858: the buffer's address is C's array decay, and the two branches are C's own
  // (a wrap to the last slot when the store pointer is at the array's start).
  let kbd_buffer_ptr = withUnsafeMutablePointer(to: &kbd_buffer_entry) { $0 }
  ev = (kbd_store_ptr_entry == kbd_buffer_ptr
        ? kbd_buffer_ptr + Int(KBD_BUFFER_SIZE) - 1
        : kbd_store_ptr_entry! - 1)

  if kbd_store_ptr_entry != kbd_fetch_ptr_entry                      // C 5860
       && input_event_kind(&ev!.pointee.ie) == MONITORS_CHANGED_EVENT {  // C 5860-5861
    return
  }

  input_event_set_kind(&ie, MONITORS_CHANGED_EVENT)                  // C 5864
  XSETTERMINAL(&ie.arg, ns_display_info_terminal(x_display_list))    // C 5865

  kbd_buffer_store_event(&ie)                                        // C 5867
}

// ==========================================================================
// Round 41: `ns_delete_terminal` (C 5877-5902)
// ==========================================================================

/// C: `union display_info { … struct ns_display_info *ns; … }` (src/termhooks.h 533) - the
/// `terminal->display_info.ns` read, which the core already spells as its own shim
/// (`femacs/src/abi_shim.rs` 412, `pub unsafe extern "C" fn terminal_display_info_ns`).
@_silgen_name("terminal_display_info_ns")
func terminal_display_info_ns(_ t: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?

/// C: the `struct terminal` member C 5888's guard tests, `terminal->name` (src/termhooks.h) - a
/// member entry of the `<record>_<member>` family; the core owes this export the way it owes the
/// `ns_display_info_*` readers rounds 37 and 40 added (the shim's own table,
/// `docs/ports/abi_shim.md`, is where such an export is recorded).
@_silgen_name("terminal_name")
func terminal_name(_ t: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>?

/// C: `void image_destroy_all_bitmaps (struct ns_display_info *)` (src/image.c) - the core's own
/// entry (`femacs/src/image.rs` 1841, `pub unsafe extern "C" fn`); the `fputs` shape.
@_silgen_name("image_destroy_all_bitmaps")
func image_destroy_all_bitmaps(_ dpyinfo: UnsafeMutableRawPointer?)

/// C 5877-5902: `static void ns_delete_terminal (struct terminal *terminal)` - "This function is
/// called when the last frame on a display is deleted."  C 5884's `NSTRACE` is the empty macro at
/// `NSTRACE_ENABLED` 0, `block_input`/`unblock_input` are the pair the unit already imports
/// (C 1172-1174), and the `NS_IMPL_COCOA` arm C 5893-5897 is the one this target takes.
fileprivate func ns_delete_terminal(_ terminal: UnsafeMutableRawPointer?) {
  let dpyinfo = terminal_display_info_ns(terminal)          // C 5882

  // C 5884: `NSTRACE ("ns_delete_terminal");` - empty at `NSTRACE_ENABLED` 0.

  /* Protect against recursive calls.  delete_frame in
     delete_terminal calls us back when it deletes our last frame.  */
  if terminal_name(terminal) == nil {                      // C 5888
    return
  }

  block_input()                                            // C 5891

  /* Rather than try to clean up the NS environment we can just
     disable the app and leave it waiting for any new frames.  */
  NSApp.setActivationPolicy(.prohibited)                   // C 5896

  image_destroy_all_bitmaps(dpyinfo)                       // C 5899
  ns_delete_display(dpyinfo)                               // C 5900
  unblock_input()                                          // C 5901
}

// ==========================================================================
// Round 42: the `EmacsScroller` record, `+scrollerWidth` and `initFrame:window:`
// (C 10620-10701)
// ==========================================================================

/// C: `@interface EmacsScroller : NSScroller` (src/nsterm.h 714-754) - the scroll-bar view the
/// hooks of C 5396/5490 build and the methods of C 10620-11117 are the class's.  This round lands
/// the record and its first two methods; the rest follow in their own rounds (the
/// `EmacsView`/`EmacsWindow` pattern).  **Two of C's ivar names are `NSView`'s own**: `window`
/// (the `NSWindow` a view is in) and `frame` (its rect), which a Swift subclass cannot redeclare -
/// a declared deviation, spelled in the `emacs*` form this file already uses for C pointers of
/// that kind (`EmacsView`'s `emacsframe`, src/nsterm.h 485-487).
class EmacsScroller: NSScroller {
  /// C nsterm.h 716: `struct window *window;` - see the record's note (the name belongs to
  /// `NSView`).
  var emacs_window: UnsafeMutableRawPointer? = nil
  /// C nsterm.h 717: `struct frame *frame;` - the same note.
  var emacs_frame: UnsafeMutableRawPointer? = nil
  /// C nsterm.h 718: `NSResponder *prevResponder;`
  var prevResponder: NSResponder? = nil
  /// C nsterm.h 720-721: `/* Offset to the bottom of knob of last mouse down.  */ CGFloat
  /// last_mouse_offset;`
  var last_mouse_offset: CGFloat = 0
  /// C nsterm.h 722: `float min_portion;`
  var min_portion: Float = 0
  /// C nsterm.h 723: `int pixel_length;`
  var pixel_length: Int32 = 0
  /// C nsterm.h 724: `enum scroll_bar_part last_hit_part;` - C's enum flows as a number in this
  /// unit's other views of it (the `scroll_bar_above_handle` family are imported enumerators), so
  /// the slot is an `Int32` (the `draw_window_cursor` `cursor_type` reading).
  var last_hit_part: Int32 = 0
  /// C nsterm.h 726: `BOOL condemned;`
  var condemned: Bool = false
  /// C nsterm.h 728: `BOOL horizontal;`
  var horizontal: Bool = false
  /// C nsterm.h 730-733: `/* Optimize against excessive positioning calls generated by Emacs.  */
  /// int em_position; int em_portion; int em_whole;`
  var em_position: Int32 = 0
  var em_portion: Int32 = 0
  var em_whole: Int32 = 0

  /// `NSScroller`'s designated initializer, spelled out for the reason `EmacsView`'s and
  /// `EmacsWindow`'s are (Swift marks `init (coder:)` unavailable on these AppKit classes).
  override init(frame frameRect: NSRect) {
    super.init(frame: frameRect)
  }

  required init?(coder: NSCoder) {
    super.init(coder: coder)
  }

  /* C 10626-10648: `+ (CGFloat) scrollerWidth`.  Two of C's three arms are dead on this target
     (`MAC_OS_X_VERSION_MIN_REQUIRED < 1070` is false here - the frozen configuration's 260000):
     C 10633-10636's `respondsToSelector:` test and its `else` (C 10639-10641), and the whole
     `#if` at C 10643-10646 with C 10645's deprecated `[NSScroller scrollerWidth]`.  The live body
     is C 10637-10638's one message - `NSScroller.scrollerWidth (for: .regular, scrollerStyle:
     .legacy)` - and **the method itself cannot be declared in Swift**: the SDK marks
     `NSScroller`'s `+scrollerWidth` unavailable, so both the plain override (`error: cannot
     override 'scrollerWidth' which has been marked unavailable`) and the `@objc(scrollerWidth)`
     renamed spelling (`error: method … with Objective-C selector 'scrollerWidth' conflicts with
     method 'scrollerWidth()' from superclass 'NSScroller' with the same Objective-C selector`)
     are refused - measured, and the same class of refusal `nsfns.swift` 4867-4874 records for
     `EmacsFileDelegate`'s three delegate methods.

     The method is therefore **omitted**, and the gap is named (the ledger's `## ABI` (3)); nothing
     in the reference calls it, so no caller's behaviour changes: `[EmacsScroller scrollerWidth]`
     occurs nowhere, and C 10645's `[NSScroller scrollerWidth]` is the superclass's, in a dead
     arm.  */


  /// C 10650-10701: `- (instancetype)initFrame: (NSRect)r window: (Lisp_Object)nwin`.  C 10652's
  /// `NSTRACE` is the empty macro, C 10666-10667's `NS_IMPL_GNUSTEP` arm is dead here (this target
  /// takes C 10669's `NSViewMinXMargin | NSViewHeightSizable`), C 10679's `20 / pixel_length` is
  /// C's own *integer* division before the `float` store (kept as integer division), and C's
  /// descending `for` at C 10691 is the `while` below (the `sendEvent` block's reading of the same
  /// shape).
  @objc(initFrame:window:) init(frame r: NSRect, window nwin: Lisp_Object) {
    // C 10652: `NSTRACE ("[EmacsScroller initFrame: window:]");` - empty at `NSTRACE_ENABLED` 0.

    if r.size.width > r.size.height {                  // C 10654
      horizontal = true
    } else {
      horizontal = false
    }

    super.init(frame: r /*NSMakeRect (0, 0, 0, 0)*/)    // C 10659

    isContinuous = true                                // C 10660
    isEnabled = true                                   // C 10661

    /* Ensure auto resizing of scrollbars occurs within the emacs frame's view
       locked against the top and bottom edges, and right edge on macOS, where
       scrollers are on right.  */
    autoresizingMask = [.minXMargin, .height]          // C 10669

    emacs_window = XWINDOW(nwin)                       // C 10672
    condemned = false                                  // C 10673
    if horizontal {                                    // C 10674
      pixel_length = Int32(NSWidth(r))                 // C 10675
    } else {
      pixel_length = Int32(NSHeight(r))                // C 10677
    }
    if pixel_length == 0 { pixel_length = 1 }          // C 10678
    min_portion = Float(20 / Int(pixel_length))        // C 10679 (C's integer division)

    emacs_frame = XFRAME(window_frame(emacs_window))   // C 10681
    if FRAME_LIVE_P(emacs_frame) {                     // C 10682
      var i: Int32 = 0                                 // C 10684
      let view = FRAME_NS_VIEW(emacs_frame) as? EmacsView   // C 10685
      let sview = view?.window?.contentView            // C 10686
      let subs = sview?.subviews                       // C 10687

      /* Disable optimization stopping redraw of other scrollbars.  */
      view?.scrollbarsNeedingUpdate = 0                // C 10690
      i = Int32(subs?.count ?? 0) - 1                  // C 10691
      while i >= 0 {                                   // C 10691 (C's descending `for`)
        if subs?[Int(i)] is EmacsScroller {            // C 10692
          view?.scrollbarsNeedingUpdate += 1           // C 10693
        }
        i -= 1
      }
      sview?.addSubview(self)                          // C 10694
    }

    // C 10697: `/* [self setFrame: r]; */` - C's commented-out line, carried as the comment.
  }

  /// C 10703-10717: `- (void)setFrame: (NSRect)newRect`.  **C's method is Swift's `frame` property
  /// setter**: AppKit's `-setFrame:` is imported as `var frame: NSRect` (measured: a plain
  /// `override func setFrame(_:)` is refused - `error: method does not override any method from its
  /// superclass`, and `super.setFrame` read as `error: value of type 'NSScroller' has no member
  /// 'setFrame'`), so C's body is the setter below and the getter is the superclass's own (C
  /// overrides only the setter).  The setter's parameter keeps C's name.  C's `NSTRACE` is the
  /// empty macro, C 10707 and C 10715 are C's own commented-out `block_input ()`/`unblock_input ()`
  /// pair (carried as the comments), and C 10713's `20 / pixel_length` keeps C's integer division
  /// before the `float` store - the same expression `initFrame:window:`'s C 10679 has.
  override var frame: NSRect {
    get { super.frame }
    set(newRect) {
      // C 10705: `NSTRACE ("[EmacsScroller setFrame:]");` - empty at `NSTRACE_ENABLED` 0.
      // C 10707: `/* block_input (); */`

      if horizontal {                                  // C 10708
        pixel_length = Int32(NSWidth(newRect))         // C 10709
      } else {
        pixel_length = Int32(NSHeight(newRect))        // C 10711
      }
      if pixel_length == 0 { pixel_length = 1 }        // C 10712
      min_portion = Float(20 / Int(pixel_length))      // C 10713 (C's integer division)

      super.frame = newRect                            // C 10714
      // C 10715: `/* unblock_input (); */`
    }
  }

  /// C 10719-10732: `- (void)dealloc` - Swift's `deinit`, because a class cannot name `dealloc`;
  /// C 10730's `[super dealloc]` is implicit in the runtime's own teardown (the
  /// `ns_free_frame_resources` reading of the reference's manual release under ARC).
  deinit {
    // C 10721: `NSTRACE ("[EmacsScroller dealloc]");` - empty.

    if emacs_window != nil {                           // C 10722
      if horizontal {                                  // C 10724
        wset_horizontal_scroll_bar(emacs_window, Qnil())   // C 10725
      } else {
        wset_vertical_scroll_bar(emacs_window, Qnil())     // C 10727
      }
    }
    emacs_window = nil                                 // C 10729
    // C 10730: `[super dealloc];` - implicit in `deinit`.
  }

  /// C 10734-10740: `- (instancetype)condemn` - C's `instancetype` is this class's own type.
  func condemn() -> EmacsScroller {
    // C 10736: `NSTRACE ("[EmacsScroller condemn]");` - empty.
    condemned = true                                   // C 10737
    return self                                        // C 10738
  }

  /// C 10742-10748: `- (instancetype)reprieve`.
  func reprieve() -> EmacsScroller {
    // C 10744: `NSTRACE ("[EmacsScroller reprieve]");` - empty.
    condemned = false                                  // C 10745
    return self                                        // C 10746
  }

  /// C 10750-10775: `-(bool)judge`.  C's `block_input ()`/`unblock_input ()` pair wraps the
  /// teardown, C 10759's cast is the `as? EmacsView` the unit already uses, C 10771's
  /// `[self release]` is the reference the method drops (the `Unmanaged` reading), and C's `bool`
  /// return is Swift's `Bool`.
  func judge() -> Bool {
    // C 10752: `NSTRACE ("[EmacsScroller judge]");` - empty.
    let ret = condemned                                // C 10753
    if condemned {                                     // C 10754
      block_input()                                    // C 10757
      /* Ensure other scrollbar updates after deletion.  */
      let view = FRAME_NS_VIEW(emacs_frame) as? EmacsView   // C 10759
      if view != nil {                                 // C 10760
        view?.scrollbarsNeedingUpdate += 1             // C 10761
      }
      if emacs_window != nil {                         // C 10762
        if horizontal {                                // C 10764
          wset_horizontal_scroll_bar(emacs_window, Qnil())   // C 10765
        } else {
          wset_vertical_scroll_bar(emacs_window, Qnil())     // C 10767
        }
      }
      emacs_window = nil                               // C 10769
      removeFromSuperview()                            // C 10770
      Unmanaged.passUnretained(self).release()         // C 10771 (C's `[self release]`)
      unblock_input()                                  // C 10772
    }
    return ret                                         // C 10774
  }

  /// C 10777-10786: `- (void) mark` - C's own spacing in the selector, kept.
  func mark() {
    if emacs_window != nil {                           // C 10779
      var win: Lisp_Object = nil                       // C 10781
      XSETWINDOW(&win, emacs_window)                   // C 10782
      mark_object(win)                                 // C 10783
    }
  }

  /// C 10788-10804: `- (void)resetCursorRects`.  The
  /// `NS_IMPL_GNUSTEP || MAC_OS_X_VERSION_MIN_REQUIRED < 101300` block at C 10796-10802 (its
  /// `setOnMouseEntered:` message included) is dead on this target - the frozen configuration's
  /// 260000 - so it is recorded, not transcribed.
  override func resetCursorRects() {
    let visible = self.visibleRect                     // C 10790
    // C 10791: `NSTRACE ("[EmacsScroller resetCursorRects]");` - empty.

    if !NSIsEmptyRect(visible) {                       // C 10793
      self.addCursorRect(visible, cursor: NSCursor.arrow)   // C 10794
    }

    /* C 10796-10802: the pre-10.13 / GNUstep arm, dead here (recorded, not transcribed).  */
  }

  /// C 10806-10812: `- (int) checkSamePosition: (int) position portion: (int) portion
  /// whole: (int) whole`.  C's whole body is the four-way conjunct and it returns an `int`, so the
  /// port spells C's own integral conversion of it (`? 1 : 0`).
  func checkSamePosition(_ position: Int32, portion: Int32, whole: Int32) -> Int32 {
    return (em_position == position && em_portion == portion && em_whole == whole
            && portion != whole) /* Needed for resizing empty buffer.  */
           ? 1 : 0
  }

  /// C 10814-10847: `- (instancetype)setPosition: (int)position portion: (int)portion whole:
  /// (int)whole`.  The `NS_IMPL_COCOA` arms are the live ones (C 10825-10826 and C 10839-10840;
  /// C 10828 and C 10842 are dead here), and the two AppKit setters are Swift's `knobProportion`
  /// and `doubleValue` properties.  C 10835's `max ((float)whole*min_portion/pixel_length, portion)`
  /// is `Swift.max` over C's own promoted `float` arithmetic with C's truncation back into the
  /// `int` it assigns, and C 10836-10837 keep C's two divisions (a `float` one and a `CGFloat` one).
  func setPosition(_ position: Int32, portion portionIn: Int32, whole: Int32) -> EmacsScroller {
    // C 10816: `NSTRACE ("[EmacsScroller setPosition:portion:whole:]");` - empty.

    em_position = position                             // C 10818
    em_portion = portionIn                             // C 10819
    em_whole = whole                                   // C 10820

    if portionIn >= whole {                            // C 10822
      knobProportion = 1.0                             // C 10825 (the `NS_IMPL_COCOA` arm)
      doubleValue = 1.0                                // C 10826
    } else {
      var pos: Float = 0                               // C 10833
      var por: CGFloat = 0                             // C 10834
      /* C's `portion` is reassigned here, so the port shadows the parameter with C's own name.  */
      let portion = Int32(Swift.max(Float(whole) * min_portion / Float(pixel_length),
                                    Float(portionIn))) // C 10835
      pos = Float(position) / Float(whole - portion)   // C 10836
      por = CGFloat(portion) / CGFloat(whole)          // C 10837
      knobProportion = por                             // C 10839 (the `NS_IMPL_COCOA` arm)
      doubleValue = Double(pos)                        // C 10840
    }

    return self                                        // C 10846
  }

  /// C 10849-10890: `- (void) sendScrollEventAtLoc: (float)loc fromEvent: (NSEvent *)e` - "Set up
  /// emacs_event."  C 10854's `NSTRACE` is empty; the member writes are this unit's own view of the
  /// record; `part` is the high bitfield, which the core spells with `input_event_set_part` (the
  /// `input_event_set_kind` counterpart); `XSETINT` takes C's `int`, so C's `float` expressions are
  /// converted where C's `make_number` converts them; and **C's `EV_MODIFIERS (e)`/`EV_TIMESTAMP
  /// (e)` read a possibly-nil event** (nil's `modifierFlags`/`timestamp` are zero), which the port
  /// writes out because both macros take a live `NSEvent`.
  func sendScrollEventAtLoc(_ loc: Float, fromEvent e: NSEvent?) {
    var win: Lisp_Object = nil                         // C 10852

    // C 10854: `NSTRACE ("[EmacsScroller sendScrollEventAtLoc:fromEvent:]");` - empty.

    if emacs_event == nil {                            // C 10856
      return
    }

    input_event_set_part(emacs_event!, last_hit_part)  // C 10859
    emacs_event!.pointee.code = 0                      // C 10860
    /* C's `EV_MODIFIERS (e) | down_modifier` mixes an `unsigned int` with the `int` enumerator,
       which C's own promotion converts; the port spells that conversion (the `EV_MODIFIERS2`
       reading's `truncatingIfNeeded:` shape).  */
    emacs_event!.pointee.modifiers =
      (e == nil ? EV_MODIFIERS2(0, QCmouse()) : EV_MODIFIERS(e!))
        | UInt32(truncatingIfNeeded: down_modifier())   // C 10861
    XSETWINDOW(&win, emacs_window)                     // C 10862
    emacs_event!.pointee.frame_or_window = win         // C 10863
    emacs_event!.pointee.timestamp =
      UInt(e == nil ? 0 : EV_TIMESTAMP(e!))            // C 10864 (C stores the double into `Time`)
    emacs_event!.pointee.arg = Qnil()                  // C 10865

    if horizontal {                                    // C 10867
      input_event_set_kind(emacs_event!, HORIZONTAL_SCROLL_BAR_CLICK_EVENT)   // C 10869
      XSETINT(&emacs_event!.pointee.x,
              Int(Float(em_whole) * loc / Float(pixel_length)))               // C 10870
      XSETINT(&emacs_event!.pointee.y, Int(em_whole))                         // C 10871
    } else {
      input_event_set_kind(emacs_event!, SCROLL_BAR_CLICK_EVENT)              // C 10875
      XSETINT(&emacs_event!.pointee.x, Int(loc))                              // C 10876
      XSETINT(&emacs_event!.pointee.y, Int(pixel_length - 20))                // C 10877
    }

    if q_event_ptr != nil {                            // C 10880
      n_emacs_events_pending += 1                      // C 10882
      kbd_buffer_store_event_hold(emacs_event, q_event_ptr)   // C 10883
    } else {
      hold_event(emacs_event)                          // C 10886
    }
    EVENT_INIT(emacs_event!)                           // C 10887
    ns_send_appdefined(-1)                             // C 10888
  }

  /// C 10892-10924: `- (instancetype)repeatScroll: (NSTimer *)scrollEntry` - "Called manually
  /// through timer to implement repeated button action with hold-down."  C 10896-10897's `[self
  /// window]` is AppKit's own message, not the class's `window` ivar (which is why that ivar is
  /// `emacs_window` here) and its nil receiver answers the zero-valued point, C 10906's `release` is
  /// the reference the method drops, C 10912-10919's `retain` is ARC's strong reference to the timer
  /// `scheduledTimer` already returns, and C 10916's `@selector (repeatScroll:)` is the `#selector`
  /// this method's own `@objc` name provides.
  @objc func repeatScroll(_ scrollEntry: Timer) -> EmacsScroller {
    let e = self.window?.currentEvent                  // C 10896
    let p = self.window?.mouseLocationOutsideOfEventStream ?? .zero   // C 10897
    let inKnob = self.testPart(p) == .knob             // C 10898

    // C 10900: `NSTRACE ("[EmacsScroller repeatScroll:]");` - empty.

    /* Clear timer if need be.  */
    if inKnob || scroll_repeat_entry?.timeInterval == SCROLL_BAR_FIRST_DELAY {   // C 10903
      scroll_repeat_entry?.invalidate()                // C 10905
      scroll_repeat_entry = nil                        // C 10906-10907 (C's `release`, then `nil`)

      if inKnob {                                      // C 10909
        return self                                    // C 10910
      }

      scroll_repeat_entry =
        Timer.scheduledTimer(timeInterval: SCROLL_BAR_CONTINUOUS_DELAY,     // C 10912-10919
                             target: self,
                             selector: #selector(EmacsScroller.repeatScroll(_:)),
                             userInfo: nil,
                             repeats: true)
    }

    self.sendScrollEventAtLoc(0, fromEvent: e)         // C 10922
    return self                                        // C 10923
  }

  /// C 10927-11047: `- (void)mouseDown: (NSEvent *)e` - "Asynchronous mouse tracking for scroller.
  /// This allows us to dispatch mouseDragged events without going into a modal loop."  C 10945-10950's
  /// two `case`s (`NSScrollerDecrementLine`/`IncrementLine`) are dead on this target
  /// (`NS_IMPL_GNUSTEP || MAC_OS_X_VERSION_MIN_REQUIRED < 1070`), C 10934's `UNINIT` is
  /// conf_post.h 369's `= {0,}` on this configuration (`GCC_LINT` is 1 in the frozen macros), so
  /// the three locals start at zero as C's own arm does, C 10956's `%ld` prints `part`'s own
  /// `NSInteger` (`part.rawValue`), and C's `NSWidth`/`NSHeight`/`NSMinX`/`NSMinY` are the rect's
  /// own accessors.  Swift's own renames (`mouseDown` -> `mouseDown (with:)`, `mouseUp` ->
  /// `mouseUp (with:)`) are the compiler's readings (class B); the Objective-C selectors are C's.
  @objc override func mouseDown(with e: NSEvent) {
    var sr = NSRect.zero                               // C 10931
    var kr = NSRect.zero
    /* hitPart is only updated AFTER event is passed on.  */
    let part = self.testPart(e.locationInWindow)       // C 10933
    var loc: CGFloat = 0                               // C 10934 (C's `UNINIT` = `= {0,}`)
    var kloc: CGFloat = 0
    var pos: CGFloat = 0
    var edge: Int32 = 0                                // C 10935

    // C 10937: `NSTRACE ("[EmacsScroller mouseDown:]");` - empty at `NSTRACE_ENABLED` 0.

    switch part {                                      // C 10939
    case .decrementPage:                               // C 10941
      last_hit_part =
        horizontal ? scroll_bar_before_handle() : scroll_bar_above_handle()      // C 10942
    case .incrementPage:                               // C 10943
      last_hit_part =
        horizontal ? scroll_bar_after_handle() : scroll_bar_below_handle()       // C 10944
    /* C 10945-10950: `NSScrollerDecrementLine`/`NSScrollerIncrementLine` and their two
       `last_hit_part` arms, dead on this target (recorded, not transcribed).  */
    case .knob:                                       // C 10951
      last_hit_part =
        horizontal ? scroll_bar_horizontal_handle() : scroll_bar_handle()        // C 10952
    case .knobSlot:  /* GNUstep-only */               // C 10953
      last_hit_part = scroll_bar_move_ratio()         // C 10954
    default:  /* NSScrollerNoPart? */                 // C 10955
      fprintf(stderr, "EmacsScroller-mouseDown: unexpected part %ld\n",
              part.rawValue)                          // C 10956-10957
      return                                          // C 10958
    }

    if part == .knob || part == .knobSlot {            // C 10961
      /* handle, or on GNUstep possibly slot */
      var fake_event: NSEvent? = nil                   // C 10964
      var length: Int32 = 0                            // C 10965

      /* compute float loc in slot and mouse offset on knob */
      sr = self.convert(self.rect(for: .knobSlot), to: nil)          // C 10968-10969
      if horizontal {                                  // C 10970
        length = Int32(NSWidth(sr))                    // C 10972
        loc = (e.locationInWindow.x - NSMinX(sr))      // C 10973
      } else {
        length = Int32(NSHeight(sr))                   // C 10977
        loc = CGFloat(length) - (e.locationInWindow.y - NSMinY(sr))  // C 10978
      }

      if loc <= 0.0 {                                  // C 10981
        loc = 0.0
        edge = -1                                      // C 10984
      } else if loc >= CGFloat(length) {               // C 10986
        loc = CGFloat(length)                          // C 10988
        edge = 1                                       // C 10989
      }

      if edge != 0 {                                   // C 10992
        kloc = 0.5 * CGFloat(edge)                     // C 10993
      } else {
        kr = self.convert(self.rect(for: .knob), to: nil)            // C 10996-10997
        if horizontal {                                // C 10998
          kloc = (e.locationInWindow.x - NSMinX(kr))   // C 10999
        } else {
          kloc = NSHeight(kr) - (e.locationInWindow.y - NSMinY(kr))  // C 11001
        }
      }
      last_mouse_offset = kloc                         // C 11003

      /* if knob, tell emacs a location offset by knob pos
         (to indicate top of handle) */
      if part == .knob {                               // C 11007
        pos = (loc - last_mouse_offset)                // C 11008
      } else {
        /* else this is a slot click on GNUstep: go straight there */
        pos = loc                                      // C 11011
      }

      /* If there are buttons in the scroller area, we need to
         recalculate pos as emacs expects the scroller slot to take up
         the entire available length.  */
      if length != pixel_length {                      // C 11016
        pos = pos * CGFloat(pixel_length) / CGFloat(length)   // C 11017
      }

      /* send a fake mouse-up to super to preempt modal -trackKnob: mode */
      fake_event = NSEvent.mouseEvent(with: .leftMouseUp,       // C 11020-11028
                                      location: e.locationInWindow,
                                      modifierFlags: e.modifierFlags,
                                      timestamp: e.timestamp,
                                      windowNumber: e.windowNumber,
                                      context: nil,
                                      eventNumber: e.eventNumber,
                                      clickCount: e.clickCount,
                                      pressure: e.pressure)
      /* C 11029's `[super mouseUp: fake_event]` - a message to a nil event is C's own no-op.  */
      if let fake_event = fake_event {
        super.mouseUp(with: fake_event)
      }
    } else {
      pos = 0; /* ignored */                           // C 11033

      /* Set a timer to repeat, as we can't let superclass do this modally.  */
      scroll_repeat_entry =
        Timer.scheduledTimer(timeInterval: SCROLL_BAR_FIRST_DELAY,    // C 11036-11042
                             target: self,
                             selector: #selector(EmacsScroller.repeatScroll(_:)),
                             userInfo: nil,
                             repeats: true)
    }

    if part != .knob {                                 // C 11045
      self.sendScrollEventAtLoc(Float(pos), fromEvent: e)   // C 11046
    }
  }

  /// C 11050-11090: `- (void)mouseDragged: (NSEvent *)e` - "Called as we manually track scroller
  /// drags, rather than superclass."  C's two locals are `double` (not `CGFloat`), C 11077-11079's
  /// bound is `length + last_mouse_offset` with C's own promotions, and C 11087's
  /// `pos * pixel_length / length` keeps C's per-operator promotion; C 11090 hands the `double` to
  /// the `float` parameter of the method above, so the port converts where C's call does.
  @objc override func mouseDragged(with e: NSEvent) {
    var sr = NSRect.zero                               // C 11053
    var loc: Double = 0                                // C 11054
    var pos: Double = 0
    var length: Int32 = 0                              // C 11055

    // C 11057: `NSTRACE ("[EmacsScroller mouseDragged:]");` - empty.

    sr = self.convert(self.rect(for: .knobSlot), to: nil)          // C 11059-11060
    if horizontal {                                    // C 11062
      length = Int32(NSWidth(sr))                      // C 11064
      loc = Double(e.locationInWindow.x - NSMinX(sr))  // C 11065
    } else {
      length = Int32(NSHeight(sr))                     // C 11069
      loc = Double(length) - Double(e.locationInWindow.y - NSMinY(sr))   // C 11070
    }

    if loc <= 0.0 {                                    // C 11073
      loc = 0.0                                        // C 11075
    } else if loc >= Double(length) + Double(last_mouse_offset) {        // C 11077
      loc = Double(length) + Double(last_mouse_offset) // C 11079
    }

    pos = (loc - Double(last_mouse_offset))            // C 11082

    /* If there are buttons in the scroller area, we need to
       recalculate pos as emacs expects the scroller slot to take up
       the entire available length.  */
    if length != pixel_length {                        // C 11087
      pos = pos * Double(pixel_length) / Double(length)   // C 11088
    }

    self.sendScrollEventAtLoc(Float(pos), fromEvent: e)      // C 11090
  }

  /// C 11094-11105: `- (void)mouseUp: (NSEvent *)e`.  C 11101's `release` is the reference the
  /// method drops (the `repeatScroll:` reading), and C 11104's enumerator is the entry this unit
  /// already imports.
  @objc override func mouseUp(with e: NSEvent) {
    // C 11096: `NSTRACE ("[EmacsScroller mouseUp:]");` - empty.

    if scroll_repeat_entry != nil {                    // C 11098
      scroll_repeat_entry?.invalidate()                // C 11100
      scroll_repeat_entry = nil                        // C 11101-11102 (C's `release`, then `nil`)
    }
    last_hit_part = scroll_bar_above_handle()          // C 11104
  }

  /// C 11108-11115: `- (void) scrollWheel: (NSEvent *)theEvent` - "Treat scrollwheel events in
  /// the bar as though they were in the main window."  C's `EmacsView *view = (EmacsView *)
  /// FRAME_NS_VIEW (frame);` is the unit's `as? EmacsView` cast, and C's message to a nil view is
  /// the optional chain's no-op.
  @objc override func scrollWheel(with theEvent: NSEvent) {
    // C 11111: `NSTRACE ("[EmacsScroller scrollWheel:]");` - empty.

    let view = FRAME_NS_VIEW(emacs_frame) as? EmacsView     // C 11113
    view?.mouseDown(with: theEvent)                    // C 11114
  }
}

/// C: the names below are **file-scope** C constructs (macros and INLINEs of src/window.h and
/// src/alloc.c) - they sat inside the class between rounds 43 and 46, which was a structural slip
/// this round corrects (C keeps them outside `@implementation`), and round 47's
/// `ns_set_vertical_scroll_bar` needs them at file scope.
/// C: `INLINE void wset_horizontal_scroll_bar (struct window *w, Lisp_Object val)` and its
/// vertical sibling (src/window.h) - the two setters C 10725/10727 and C 10765/10767 call.  The
/// port's mirror carries them as Rust functions (`femacs/src/include/window.rs` 233/236), so the
/// C-ABI entry they are imported by is one the core still owes (the `ns_display_info_*` class).
@_silgen_name("wset_horizontal_scroll_bar")
func wset_horizontal_scroll_bar(_ w: UnsafeMutableRawPointer?, _ val: Lisp_Object)
@_silgen_name("wset_vertical_scroll_bar")
func wset_vertical_scroll_bar(_ w: UnsafeMutableRawPointer?, _ val: Lisp_Object)

/// C: `XSETWINDOW (a, b)` (src/window.h) - a macro entry of the `XSETFRAME`/`XSETTERMINAL`
/// family, which C 10782 uses to build the `Lisp_Object` `mark_object` takes.
@_silgen_name("XSETWINDOW")
func XSETWINDOW(_ a: UnsafeMutablePointer<Lisp_Object>, _ b: UnsafeMutableRawPointer?)

/// C: `void mark_object (Lisp_Object)` (src/alloc.c) - the collector's own entry, which the core
/// provides (`femacs/src/alloc.rs` 7308, `pub unsafe extern "C" fn`).
@_silgen_name("mark_object") func mark_object(_ obj: Lisp_Object)

/// C 10623-10624: the two `#define`s the class's repeat timer uses, spelled as this file's other
/// numeric macros are (`DOCK_IGNORE_LIMIT`): `#define SCROLL_BAR_FIRST_DELAY 0.5` and
/// `#define SCROLL_BAR_CONTINUOUS_DELAY (1.0 / 15)`.
fileprivate let SCROLL_BAR_FIRST_DELAY: TimeInterval = 0.5
fileprivate let SCROLL_BAR_CONTINUOUS_DELAY: TimeInterval = 1.0 / 15

/// C: `ENUM_BF (scroll_bar_part) part : 16;` (src/termhooks.h 377) - the high half of
/// `input_event.bits`, whose writer is this entry (the `input_event_set_kind` counterpart C 10859
/// is the first use of).
@_silgen_name("input_event_set_part")
func input_event_set_part(_ e: UnsafeMutablePointer<input_event>, _ v: Int32)

/// C: the `struct window` member `window->horizontal_scroll_bar` (src/window.h) - the vertical
/// reader's twin, and the two `WINDOW_SCROLL_BAR_AREA_*` macros C 5528-5529 takes the horizontal
/// bar's own height and y from (`WINDOW_SCROLL_BAR_AREA_WIDTH`/`_X` landed with round 47).
@_silgen_name("window_horizontal_scroll_bar")
func window_horizontal_scroll_bar(_ w: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("WINDOW_SCROLL_BAR_AREA_HEIGHT")
func WINDOW_SCROLL_BAR_AREA_HEIGHT(_ w: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("WINDOW_SCROLL_BAR_AREA_Y")
func WINDOW_SCROLL_BAR_AREA_Y(_ w: UnsafeMutableRawPointer?) -> Int32

/// C 5490-5571: `static void ns_set_horizontal_scroll_bar (struct window *window, int portion, int
/// whole, int position)` - "External (hook): Update or add scrollbar."  It is the vertical hook's
/// twin with three differences C's own text shows: C 5525 takes the *x* pair out of `window_box`,
/// C 5528-5529 read the horizontal bar's own height and y, and C 5548-5558's update branch has no
/// inner `oldRect`/`r` width copy (the vertical hook's C 5474 line has no counterpart here) while
/// setting `update_p = YES`.  C 5561-5566's shared-corner clear is kept with C's own
/// `window->vertical_scroll_bar` test.
fileprivate func ns_set_horizontal_scroll_bar(_ window: UnsafeMutableRawPointer?,
                                              _ portion: Int32, _ whole: Int32,
                                              _ position: Int32) {
  var win: Lisp_Object = nil                         // C 5496
  var r = NSRect.zero                                // C 5497
  var v = NSRect.zero
  let f = XFRAME(WINDOW_FRAME(window))               // C 5498
  let view = FRAME_NS_VIEW(f) as? EmacsView          // C 5499
  var bar: EmacsScroller? = nil                      // C 5500
  var top: Int32 = 0                                 // C 5501
  var height: Int32 = 0
  var left: Int32 = 0
  var width: Int32 = 0
  var window_x: Int32 = 0                            // C 5502
  var window_width: Int32 = 0
  var update_p = true                                // C 5503

  /* Optimization; display engine sends WAY too many of these.  */
  if !NILP(window_horizontal_scroll_bar(window)) {   // C 5506
    bar = XNS_SCROLL_BAR(window_horizontal_scroll_bar(window)) as? EmacsScroller  // C 5508
    if bar!.checkSamePosition(position, portion: portion, whole: whole) != 0 {    // C 5509
      if view?.scrollbarsNeedingUpdate == 0 {        // C 5511
        if windows_or_buffers_changed == 0 {         // C 5513
          return                                     // C 5514
        }
      } else {
        view?.scrollbarsNeedingUpdate -= 1           // C 5517
      }
      update_p = false                               // C 5518
    }
  }

  // C 5522: `NSTRACE ("ns_set_horizontal_scroll_bar");` - empty at `NSTRACE_ENABLED` 0.

  /* Get dimensions.  */
  window_box(window, ANY_AREA(), &window_x, nil, &window_width, nil)   // C 5525
  left = window_x                                    // C 5526
  width = window_width                               // C 5527
  height = WINDOW_SCROLL_BAR_AREA_HEIGHT(window)     // C 5528
  top = WINDOW_SCROLL_BAR_AREA_Y(window)             // C 5529

  r = NSMakeRect(CGFloat(left), CGFloat(top), CGFloat(width), CGFloat(height))   // C 5531
  /* The parent view is flipped, so we need to flip y value.  */
  v = view?.frame ?? .zero                           // C 5533 (C's nil receiver answers a zero rect)
  r.origin.y = (v.size.height - r.size.height - r.origin.y)      // C 5534

  XSETWINDOW(&win, window)                           // C 5536
  block_input()                                      // C 5537

  if NILP(window_horizontal_scroll_bar(window)) {    // C 5539
    if width > 0 && height > 0 {                     // C 5541
      ns_clear_frame_area(f, left, top, width, height)   // C 5542
    }

    bar = EmacsScroller(frame: r, window: win)       // C 5544
    wset_horizontal_scroll_bar(window,
                               make_mint_ptr(Unmanaged.passUnretained(bar!).toOpaque()))   // C 5545
    update_p = true                                  // C 5546
  } else {
    var oldRect = NSRect.zero                        // C 5550
    bar = XNS_SCROLL_BAR(window_horizontal_scroll_bar(window)) as? EmacsScroller   // C 5551
    oldRect = bar!.frame                             // C 5552
    if FRAME_LIVE_P(f) && !(oldRect == r) {          // C 5553 (C's `NSEqualRects` is `==`)
      ns_clear_frame_area(f, left, top, width, height)   // C 5555
      bar!.frame = r                                 // C 5556
      update_p = true                                // C 5557
    }
  }

  /* If there are both horizontal and vertical scroll-bars they leave
     a square that belongs to neither. We need to clear it otherwise
     it fills with junk.  */
  if !NILP(window_vertical_scroll_bar(window)) {     // C 5564
    ns_clear_frame_area(f, WINDOW_SCROLL_BAR_AREA_X(window), top,      // C 5565
                        WINDOW_SCROLL_BAR_AREA_WIDTH(window), height)  // C 5566
  }

  if update_p {                                      // C 5568
    _ = bar!.setPosition(position, portion: portion, whole: whole)   // C 5569
  }
  unblock_input()                                    // C 5570
}

/// C: `#define XNS_SCROLL_BAR(vec) ((id) xmint_pointer (vec))` (src/nsterm.h 1056, the
/// `NS_IMPL_COCOA` arm) - a macro entry under the construct's own name; the `xmint_pointer` inside
/// it is the INLINE the tree mirrors (`femacs/src/include/lisp.rs` 1679), and the `(id)` cast is
/// what the call sites' `as? EmacsScroller` reads.
@_silgen_name("XNS_SCROLL_BAR")
func XNS_SCROLL_BAR(_ vec: Lisp_Object) -> AnyObject?

/// C: `INLINE Lisp_Object make_mint_ptr (void *a)` (src/lisp.h) and its reader `xmint_pointer` -
/// macros/INLINEs of that family, mirrored at `femacs/src/include/lisp.rs` 1664/1679, so the C-ABI
/// entries are the core's to provide.
@_silgen_name("make_mint_ptr")
func make_mint_ptr(_ a: UnsafeMutableRawPointer?) -> Lisp_Object

/// C: the `struct window` member `window->vertical_scroll_bar` (src/window.h) - a member reader of
/// the `window_frame` family this unit already carries.
@_silgen_name("window_vertical_scroll_bar")
func window_vertical_scroll_bar(_ w: UnsafeMutableRawPointer?) -> Lisp_Object

/// C: `WINDOW_SCROLL_BAR_AREA_WIDTH (w)`, `WINDOW_SCROLL_BAR_AREA_X (w)` and
/// `WINDOW_TOTAL_LINES (w)` (src/window.h) - macro entries of the `WINDOW_FRAME` family.
@_silgen_name("WINDOW_SCROLL_BAR_AREA_WIDTH")
func WINDOW_SCROLL_BAR_AREA_WIDTH(_ w: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("WINDOW_SCROLL_BAR_AREA_X")
func WINDOW_SCROLL_BAR_AREA_X(_ w: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("WINDOW_TOTAL_LINES")
func WINDOW_TOTAL_LINES(_ w: UnsafeMutableRawPointer?) -> Int32

/// C 5396-5486: `static void ns_set_vertical_scroll_bar (struct window *window, int portion, int
/// whole, int position)` - "External (hook): Update or add scrollbar".  C's `XNS_SCROLL_BAR` cast is
/// the `as? EmacsScroller` here, `NSEqualRects (a, b)` is `a == b`, the two `[view …]` reads on a
/// possibly-nil view are C's nil-receiver answers (`?? .zero` for the rect), and C 5417-5423's
/// counter writes are `EmacsView`'s own member; `bar` keeps C's own assumption that the value it
/// casts is live.
fileprivate func ns_set_vertical_scroll_bar(_ window: UnsafeMutableRawPointer?,
                                            _ portion: Int32, _ whole: Int32,
                                            _ position: Int32) {
  var win: Lisp_Object = nil                         // C 5402
  var r = NSRect.zero                                // C 5403
  var v = NSRect.zero
  let f = XFRAME(WINDOW_FRAME(window))               // C 5404
  let view = FRAME_NS_VIEW(f) as? EmacsView          // C 5405
  var bar: EmacsScroller? = nil                      // C 5406
  var window_y: Int32 = 0                            // C 5407
  var window_height: Int32 = 0
  var top: Int32 = 0                                 // C 5408
  var left: Int32 = 0
  var height: Int32 = 0
  var width: Int32 = 0
  var update_p = true                                // C 5409

  /* Optimization; display engine sends WAY too many of these.  */
  if !NILP(window_vertical_scroll_bar(window)) {     // C 5412
    bar = XNS_SCROLL_BAR(window_vertical_scroll_bar(window)) as? EmacsScroller    // C 5414
    if bar!.checkSamePosition(position, portion: portion, whole: whole) != 0 {    // C 5415
      if view?.scrollbarsNeedingUpdate == 0 {        // C 5417
        if windows_or_buffers_changed == 0 {         // C 5419
          return                                     // C 5420
        }
      } else {
        view?.scrollbarsNeedingUpdate -= 1           // C 5423
      }
      update_p = false                               // C 5424
    }
  }

  // C 5428: `NSTRACE ("ns_set_vertical_scroll_bar");` - empty at `NSTRACE_ENABLED` 0.

  /* Get dimensions.  */
  window_box(window, ANY_AREA(), nil, &window_y, nil, &window_height)   // C 5431
  top = window_y                                     // C 5432
  height = window_height                             // C 5433
  width = WINDOW_SCROLL_BAR_AREA_WIDTH(window)       // C 5434
  left = WINDOW_SCROLL_BAR_AREA_X(window)            // C 5435

  r = NSMakeRect(CGFloat(left), CGFloat(top), CGFloat(width), CGFloat(height))   // C 5437
  /* The parent view is flipped, so we need to flip y value.  */
  v = view?.frame ?? .zero                           // C 5439 (C's nil receiver answers a zero rect)
  r.origin.y = (v.size.height - r.size.height - r.origin.y)      // C 5440

  XSETWINDOW(&win, window)                           // C 5442
  block_input()                                      // C 5443

  /* We want at least 5 lines to display a scrollbar.  */
  if WINDOW_TOTAL_LINES(window) < 5 {                // C 5446
    if !NILP(window_vertical_scroll_bar(window)) {   // C 5448
      bar = XNS_SCROLL_BAR(window_vertical_scroll_bar(window)) as? EmacsScroller   // C 5450
      bar!.removeFromSuperview()                     // C 5451
      wset_vertical_scroll_bar(window, Qnil())       // C 5452
      Unmanaged.passUnretained(bar!).release()       // C 5453 (C's `[bar release]`)
      ns_clear_frame_area(f, left, top, width, height)   // C 5454
    }
    unblock_input()                                  // C 5456
    return                                           // C 5457
  }

  if NILP(window_vertical_scroll_bar(window)) {      // C 5460
    if width > 0 && height > 0 {                     // C 5462
      ns_clear_frame_area(f, left, top, width, height)   // C 5463
    }

    bar = EmacsScroller(frame: r, window: win)       // C 5465
    wset_vertical_scroll_bar(window,
                             make_mint_ptr(Unmanaged.passUnretained(bar!).toOpaque()))   // C 5466
    update_p = true                                  // C 5467
  } else {
    var oldRect = NSRect.zero                        // C 5471
    bar = XNS_SCROLL_BAR(window_vertical_scroll_bar(window)) as? EmacsScroller   // C 5472
    oldRect = bar!.frame                             // C 5473
    r.size.width = oldRect.size.width                // C 5474
    if FRAME_LIVE_P(f) && !(oldRect == r) {          // C 5475 (C's `NSEqualRects` is `==`)
      if !(oldRect == r) {                           // C 5477
        ns_clear_frame_area(f, left, top, width, height)   // C 5478
      }
      bar!.frame = r                                 // C 5479
    }
  }

  if update_p {                                      // C 5483
    _ = bar!.setPosition(position, portion: portion, whole: whole)   // C 5484
  }
  unblock_input()                                    // C 5485
}

/// C: the `enum scroll_bar_part` enumerators C 10942-10954 assigns to `last_hit_part` (src/termhooks.h
/// 37's family, whose `scroll_bar_above_handle` this unit already imports) - enumerator entries, one
/// per name.
@_silgen_name("scroll_bar_before_handle")
func scroll_bar_before_handle() -> Int32
@_silgen_name("scroll_bar_after_handle")
func scroll_bar_after_handle() -> Int32
@_silgen_name("scroll_bar_below_handle")
func scroll_bar_below_handle() -> Int32
@_silgen_name("scroll_bar_handle")
func scroll_bar_handle() -> Int32
@_silgen_name("scroll_bar_horizontal_handle")
func scroll_bar_horizontal_handle() -> Int32
@_silgen_name("scroll_bar_move_ratio")
func scroll_bar_move_ratio() -> Int32

/// C: `SCROLL_BAR_CLICK_EVENT` and `HORIZONTAL_SCROLL_BAR_CLICK_EVENT` (members of `event_kind`,
/// src/termhooks.h) - mirror-valued constants, the `MONITORS_CHANGED_EVENT` reading: the frozen
/// macro table cannot decide that enum's members, and the tree's own mirror fixes both
/// (`femacs/src/include/termhooks.rs` 35 and 36 give 8 and 9).
fileprivate let SCROLL_BAR_CLICK_EVENT: UInt32 = 8
fileprivate let HORIZONTAL_SCROLL_BAR_CLICK_EVENT: UInt32 = 9

// ==========================================================================
// Round 49: the rest of the scroll-bar hook batch (C 5575-5640) and the dead
// `ns_redraw_scroll_bars` (C 2771-2786)
// ==========================================================================

/// C: `WINDOW_HAS_VERTICAL_SCROLL_BAR (W)` (src/window.h 894) and
/// `WINDOW_HAS_HORIZONTAL_SCROLL_BAR (W)` (src/window.h 909) - predicate macros, declared as `Bool`
/// the way this unit declares `FRAME_LIVE_P` (C's `||` yields an `int`, which the `if` tests).
@_silgen_name("WINDOW_HAS_VERTICAL_SCROLL_BAR")
func WINDOW_HAS_VERTICAL_SCROLL_BAR(_ w: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("WINDOW_HAS_HORIZONTAL_SCROLL_BAR")
func WINDOW_HAS_HORIZONTAL_SCROLL_BAR(_ w: UnsafeMutableRawPointer?) -> Bool

/// C 5575-5593: `static void ns_condemn_scroll_bars (struct frame *f)` - "External (hook): arrange
/// for all frame's scrollbars to be removed at next call to judge_scroll_bars, except for those
/// redeemed."  C's descending `for` is the `while` (the `initFrame:window:` reading), its
/// `isKindOfClass: [EmacsScroller class]` is the port's own class test, and `condemn`'s
/// `instancetype` result is discarded as C's statement discards it.
fileprivate func ns_condemn_scroll_bars(_ f: UnsafeMutableRawPointer?) {
  var i: Int32 = 0                                   // C 5581
  let subviews = FRAME_NS_VIEW(f)?.superview?.subviews   // C 5582

  // C 5585: `NSTRACE ("ns_condemn_scroll_bars");` - empty at `NSTRACE_ENABLED` 0.

  i = Int32(subviews?.count ?? 0) - 1                // C 5587
  while i >= 0 {                                     // C 5587 (C's descending `for`)
    let view = subviews?[Int(i)]                     // C 5589
    if view is EmacsScroller {                       // C 5590
      _ = (view as? EmacsScroller)?.condemn()        // C 5591
    }
    i -= 1
  }
}

/// C 5597-5618: `static void ns_redeem_scroll_bar (struct window *window)` - "External (hook):
/// arrange to spare this window's scrollbar at next call to judge_scroll_bars."  C's `id bar` is
/// `AnyObject?` here, C's `XNS_SCROLL_BAR` cast is the `as? EmacsScroller`, and each `[bar
/// reprieve]` discards the `instancetype` result as C's statement does.
fileprivate func ns_redeem_scroll_bar(_ window: UnsafeMutableRawPointer?) {
  var bar: AnyObject? = nil                          // C 5603
  // C 5604: `NSTRACE ("ns_redeem_scroll_bar");` - empty.

  if !NILP(window_vertical_scroll_bar(window))       // C 5605
       && WINDOW_HAS_VERTICAL_SCROLL_BAR(window) {   // C 5606
    bar = XNS_SCROLL_BAR(window_vertical_scroll_bar(window))     // C 5608
    _ = (bar as? EmacsScroller)?.reprieve()          // C 5609
  }

  if !NILP(window_horizontal_scroll_bar(window))     // C 5612
       && WINDOW_HAS_HORIZONTAL_SCROLL_BAR(window) { // C 5613
    bar = XNS_SCROLL_BAR(window_horizontal_scroll_bar(window))   // C 5615
    _ = (bar as? EmacsScroller)?.reprieve()          // C 5616
  }
}

/// C 5622-5640: `static void ns_judge_scroll_bars (struct frame *f)` - "External (hook): destroy all
/// scrollbars on frame that weren't redeemed after call to condemn_scroll_bars."  C's `continue`
/// is written as such here, which in a `while` needs the decrement both sides of it (C's `for`
/// does that itself); `judge`'s `bool` result is discarded as C's statement discards it.
fileprivate func ns_judge_scroll_bars(_ f: UnsafeMutableRawPointer?) {
  var i: Int32 = 0                                   // C 5628
  let eview = FRAME_NS_VIEW(f) as? EmacsView         // C 5630
  let subviews = eview?.superview?.subviews          // C 5631

  // C 5633: `NSTRACE ("ns_judge_scroll_bars");` - empty.

  i = Int32(subviews?.count ?? 0) - 1                // C 5634
  while i >= 0 {                                     // C 5634 (C's descending `for`)
    let view = subviews?[Int(i)]                     // C 5636
    if !(view is EmacsScroller) {                    // C 5637
      i -= 1                                         // (C's `for` increment, before the `continue`)
      continue
    }
    _ = (view as? EmacsScroller)?.judge()            // C 5638
    i -= 1
  }
}

/* C 2771-2786: `static void ns_redraw_scroll_bars (struct frame *f)` - **dead in this build**: C
   opens `#ifdef NS_IMPL_GNUSTEP` at C 2771 and closes it at C 2786, so the whole definition is the
   arm this target does not take.  Recorded, not transcribed (the `ns_update_window_end` and
   `-updateMonitors:` reading); its body would walk `[[FRAME_NS_VIEW (f) superview] subviews]`
   backwards, skipping everything that is not an `EmacsScroller`, and send each the rest `display`.
   It sits after round 4's C 641-991 block, which is why it is recorded here.  */
