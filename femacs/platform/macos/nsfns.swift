// femacs: port of src/nsfns.m @ fd7afced403.
//
// Swift has no `#include`, so this unit carries the declarations of its C environment (the three
// kinds the lane uses are listed in docs/ports/nsimage.md's `## ABI`): `@_silgen_name` imports of
// the ported core, header-only constructs (macros, INLINE functions, enumerators,
// generated-globals entries) whose C-ABI entry the core exports under the C construct's own
// name, and this file's own C interface exported with `@_cdecl`.
//
// What is specific to this unit: it is the frame-parameter / frame-operation half of the Cocoa
// front end.  It defines 52 `DEFUN`s, 56 C functions, the `EmacsFileDelegate` class, the
// `NSString (EmacsString)` category (`stringWithLispString` / `lispString`, which
// `femacs/platform/macos/nsmenu.swift` and src/nsselect.m's port already import), and
// `syms_of_nsfns`.  `src/nsterm.m` (Poincare's lane) owns the classes this file sends messages
// to; the ledger's `## ABI` records each cross-file name and its owner.

import Foundation
import AppKit
// C 36-47: the Cocoa build's `IOKit/graphics/IOGraphicsLib.h` and the AppleScript constants
// (`kAENullEvent`, `typeUTF8Text`, …) come from the SDK's Carbon umbrella (src/nsfns.m 30-47).
import Carbon

// ==========================================================================
// Lane ABI: C types.
// ==========================================================================

/// C: `typedef struct Lisp_X *Lisp_Word; typedef Lisp_Word Lisp_Object;` (src/lisp.h
/// 311-323).  The pointee is incomplete in C too and a Swift pointer to a Swift type cannot
/// be a `@_cdecl` parameter, so the lane keeps the pointer alone (docs/ports/nsimage.md).
typealias Lisp_Object = UnsafeMutableRawPointer?

/// C: `typedef CGFloat EmacsCGFloat;` (src/nsterm.h 36, the `NS_IMPL_COCOA` arm).
typealias EmacsCGFloat = CGFloat

/// C: `struct ns_display_info`'s prefix (src/nsterm.h 853-928).  This file reads `next`,
/// `terminal`, `name_list_element`, `resx`/`resy`, `n_planes` and `root_window`, so the mirror
/// runs to `root_window` and stops there.  The same mirror is in nsfont.swift and macfont.swift;
/// the three are one spelling once the ~9 files are one module (PORTING.md 0.1-5).  The member
/// offsets are the gated `ns_cocoa` reading of `spikes/gated-layout-probe.py 'struct
/// ns_display_info' nsterm.h objc` (PORTING.md P-82, `docs/baseline/ns_display_info-layout.tsv`):
/// `next 0, terminal 8, name_list_element 16, n_fonts 24, smallest_char_width 28,
/// smallest_font_height 32, bitmaps 40, bitmaps_size 48, bitmaps_last 56, resx 64, resy 72,
/// grabbed 80, n_planes 84, color_p 88, root_window 92` - the order spelled below.
struct ns_display_info {
  /// Chain of all ns_display_info structures.
  var next: UnsafeMutablePointer<ns_display_info>?
  /// The generic display parameters corresponding to this NS display.
  var terminal: UnsafeMutableRawPointer?
  /// This is a cons cell of the form (NAME . FONT-LIST-CACHE).
  var name_list_element: Lisp_Object
  /// The number of fonts loaded.
  var n_fonts: Int32
  var smallest_char_width: Int32
  var smallest_font_height: Int32
  var bitmaps: UnsafeMutableRawPointer?
  var bitmaps_size: Int
  var bitmaps_last: Int
  /// DPI resolution of this screen.
  var resx: Double
  var resy: Double
  /// Mask of things that cause the mouse to be grabbed.
  var grabbed: Int32
  var n_planes: Int32
  var color_p: Int32
  var root_window: Int32
}

/// C: `union vectorlike_header { ptrdiff_t size; }` (src/lisp.h 923-953) - one member.
struct vectorlike_header {
  var size: Int
}

/// C: `union { Lisp_Object (*a0) (void); … } function;` inside `struct Lisp_Subr`
/// (src/lisp.h 2177-2189): eleven function pointers, one slot wide; Swift has no unions.
struct Lisp_Subr_function {
  var ptr: UnsafeMutableRawPointer
}

/// C: `struct Lisp_Subr` (src/lisp.h 2174-2207); the `HAVE_NATIVE_COMP` fields (2201-2206) are
/// behind the reference's own `#ifdef`, mirrored here as Swift conditional compilation.
struct Lisp_Subr {
  var header: vectorlike_header
  var function: Lisp_Subr_function
  var min_args: Int16
  var max_args: Int16
  var symbol_name: UnsafePointer<CChar>?
  var intspec: UnsafePointer<CChar>?
  var command_modes: Lisp_Object
  var doc: Int
}

/// C: `union Aligned_Lisp_Subr { struct Lisp_Subr s; }` (src/lisp.h 2208-2212), `GCALIGNED`.
struct Aligned_Lisp_Subr {
  var s: Lisp_Subr
}

/// C: `typedef struct { ptrdiff_t bytes; } specpdl_ref;` (src/lisp.h 3646).
struct specpdl_ref {
  var bytes: Int
}

// ==========================================================================
// Lane ABI: C symbols this file calls (`@_silgen_name`).
// ==========================================================================

// Functions of the ported core.
/// C: `void error (const char *m, ...)` (src/lisp.h 4835) - `AVOID` is `_Noreturn`.
@_silgen_name("error") func error(_ format: UnsafePointer<CChar>, _ args: CVarArg...) -> Never
/// C: `Lisp_Object build_string (const char *)` (src/lisp.h 4574, `INLINE`).
@_silgen_name("build_string") func build_string(_ str: UnsafePointer<CChar>) -> Lisp_Object
/// C: `void store_frame_param (struct frame *, Lisp_Object, Lisp_Object)` (src/frame.h 1556).
@_silgen_name("store_frame_param")
func store_frame_param(_ f: UnsafeMutableRawPointer?, _ prop: Lisp_Object, _ val: Lisp_Object)
/// C: `Lisp_Object get_frame_param (struct frame *, Lisp_Object)` (src/frame.h 1555).
@_silgen_name("get_frame_param")
func get_frame_param(_ f: UnsafeMutableRawPointer?, _ prop: Lisp_Object) -> Lisp_Object
/// C: `void update_face_from_frame_parameter (struct frame *, Lisp_Object, Lisp_Object)`
/// (src/xfaces.c 6008).
@_silgen_name("update_face_from_frame_parameter")
func update_face_from_frame_parameter(_ f: UnsafeMutableRawPointer?, _ prop: Lisp_Object,
                                      _ newval: Lisp_Object)
/// C: `int ns_lisp_to_color (Lisp_Object color, NSColor **col)` (src/nsterm.h 1148).
@_silgen_name("ns_lisp_to_color")
func ns_lisp_to_color(_ color: Lisp_Object, _ col: UnsafeMutablePointer<NSColor?>) -> Int32
/// C: `void gui_update_cursor (struct frame *f, bool on_p)` (src/dispextern.h 3691).
@_silgen_name("gui_update_cursor")
func gui_update_cursor(_ f: UnsafeMutableRawPointer?, _ on_p: Bool)
/// C: `void set_frame_cursor_types (struct frame *, Lisp_Object)` (src/frame.h 1517).
@_silgen_name("set_frame_cursor_types")
func set_frame_cursor_types(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object)
/// C: `void adjust_frame_size (struct frame *, int, int, int, bool, Lisp_Object)`
/// (src/frame.h 1826).
@_silgen_name("adjust_frame_size")
func adjust_frame_size(_ f: UnsafeMutableRawPointer?, _ new_width: Int32, _ new_height: Int32,
                       _ inhibit: Int32, _ pretend: Bool, _ parameter: Lisp_Object)
/// C: `extern void adjust_frame_glyphs (struct frame *)` (src/dispextern.h 3901).
@_silgen_name("adjust_frame_glyphs") func adjust_frame_glyphs(_ f: UnsafeMutableRawPointer?)
/// C: `void clear_frame (struct frame *)` (src/dispextern.h 3667).
@_silgen_name("clear_frame") func clear_frame(_ f: UnsafeMutableRawPointer?)
/// C: `void clear_current_matrices (struct frame *f)` (src/dispextern.h 3907).
@_silgen_name("clear_current_matrices") func clear_current_matrices(_ f: UnsafeMutableRawPointer?)
/// C: `void clear_glyph_matrix (struct glyph_matrix *)` (src/dispextern.h 3906).
@_silgen_name("clear_glyph_matrix")
func clear_glyph_matrix(_ matrix: UnsafeMutableRawPointer?)
/// C: `int check_int_nonnegative (Lisp_Object)` (src/lisp.h 4795).
@_silgen_name("check_int_nonnegative") func check_int_nonnegative(_ x: Lisp_Object) -> Int32
/// C: `int fast_string_match (Lisp_Object regexp, Lisp_Object string)` (src/search.c 3113).
@_silgen_name("fast_string_match")
func fast_string_match(_ regexp: Lisp_Object, _ string: Lisp_Object) -> Int32
/// C: `Lisp_Object assq_no_quit (Lisp_Object key, Lisp_Object list)` (src/lisp.h 4272).
@_silgen_name("assq_no_quit")
func assq_no_quit(_ key: Lisp_Object, _ list: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object Fcons (Lisp_Object, Lisp_Object)` / `Fcar` / `Fcdr` (subrs, globals.h).
@_silgen_name("Fcons") func Fcons(_ car: Lisp_Object, _ cdr: Lisp_Object) -> Lisp_Object
@_silgen_name("list1") func list1(_ a: Lisp_Object) -> Lisp_Object
@_silgen_name("Fcar") func Fcar(_ list: Lisp_Object) -> Lisp_Object
@_silgen_name("Fcdr") func Fcdr(_ list: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object Freverse (Lisp_Object)` (subr, globals.h).
@_silgen_name("Freverse") func Freverse(_ list: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object Fstring_equal (Lisp_Object, Lisp_Object)` (subr, globals.h).
@_silgen_name("Fstring_equal")
func Fstring_equal(_ s1: Lisp_Object, _ s2: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object Fbuffer_modified_p (Lisp_Object)` (subr, globals.h).
@_silgen_name("Fbuffer_modified_p") func Fbuffer_modified_p(_ buffer: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object Fbuffer_file_name (Lisp_Object)` (subr, globals.h).
@_silgen_name("Fbuffer_file_name") func Fbuffer_file_name(_ buffer: Lisp_Object) -> Lisp_Object
/// C: `void defsubr (union Aligned_Lisp_Subr *)` (src/lisp.h 3444).
@_silgen_name("defsubr") func defsubr(_ sname: UnsafeMutablePointer<Aligned_Lisp_Subr>?)
/// C: `void block_input (void)` (src/blockinput.h 52, `INLINE`) and `unblock_input` (58).
@_silgen_name("block_input") func block_input()
@_silgen_name("unblock_input") func unblock_input()
/// C: `void free_frame_menubar (struct frame *)` (src/frame.h 1883) and
/// `void update_frame_tool_bar (struct frame *)` / `void free_frame_tool_bar (struct frame *)`
/// (src/nsterm.h 1175, 1180) - this file's cross-file calls into src/nsmenu.m's port, which
/// exports them with `@_cdecl`.
@_silgen_name("free_frame_menubar") func free_frame_menubar(_ f: UnsafeMutableRawPointer?)
@_silgen_name("update_frame_tool_bar") func update_frame_tool_bar(_ f: UnsafeMutableRawPointer?)
@_silgen_name("free_frame_tool_bar") func free_frame_tool_bar(_ f: UnsafeMutableRawPointer?)
/// C: `void ns_clear_frame (struct frame *f)` (src/nsterm.h 1124) - src/nsterm.m's.
@_silgen_name("ns_clear_frame") func ns_clear_frame(_ f: UnsafeMutableRawPointer?)

// Header-only constructs (macros, INLINE functions, enumerators, generated-globals entries);
// the C names are the constructs' own names, and the core side exports each as a C-ABI entry.
/// C: `#define NILP(x) lisp_h_NILP (x)` / `INLINE bool NILP (Lisp_Object)` (src/lisp.h 393, 1183).
@_silgen_name("NILP") func NILP(_ x: Lisp_Object) -> Bool
@_silgen_name("EQ") func EQ(_ x: Lisp_Object, _ y: Lisp_Object) -> Bool
@_silgen_name("BASE_EQ") func BASE_EQ(_ x: Lisp_Object, _ y: Lisp_Object) -> Bool
@_silgen_name("CONSP") func CONSP(_ x: Lisp_Object) -> Bool
@_silgen_name("STRINGP") func STRINGP(_ x: Lisp_Object) -> Bool
@_silgen_name("SYMBOLP") func SYMBOLP(_ x: Lisp_Object) -> Bool
@_silgen_name("FRAMEP") func FRAMEP(_ x: Lisp_Object) -> Bool
@_silgen_name("WINDOWP") func WINDOWP(_ x: Lisp_Object) -> Bool
@_silgen_name("MINI_WINDOW_P") func MINI_WINDOW_P(_ w: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("VECTORP") func VECTORP(_ x: Lisp_Object) -> Bool
@_silgen_name("FIXNUMP") func FIXNUMP(_ x: Lisp_Object) -> Bool
@_silgen_name("RANGED_FIXNUMP") func RANGED_FIXNUMP(_ lo: Int, _ x: Lisp_Object, _ hi: Int) -> Bool
@_silgen_name("TYPE_RANGED_FIXNUMP") func TYPE_RANGED_FIXNUMP(_ x: Lisp_Object) -> Bool
@_silgen_name("XFIXNUM") func XFIXNUM(_ x: Lisp_Object) -> Int
@_silgen_name("XFIXNAT") func XFIXNAT(_ x: Lisp_Object) -> Int
@_silgen_name("XCAR") func XCAR(_ c: Lisp_Object) -> Lisp_Object
@_silgen_name("XCDR") func XCDR(_ c: Lisp_Object) -> Lisp_Object
@_silgen_name("XFRAME") func XFRAME(_ a: Lisp_Object) -> UnsafeMutableRawPointer?
@_silgen_name("XWINDOW") func XWINDOW(_ a: Lisp_Object) -> UnsafeMutableRawPointer?
@_silgen_name("XBUFFER") func XBUFFER(_ a: Lisp_Object) -> UnsafeMutableRawPointer?
@_silgen_name("SYMBOL_NAME") func SYMBOL_NAME(_ sym: Lisp_Object) -> Lisp_Object
@_silgen_name("SSDATA") func SSDATA(_ string: Lisp_Object) -> UnsafeMutablePointer<CChar>
@_silgen_name("SBYTES") func SBYTES(_ string: Lisp_Object) -> Int
@_silgen_name("STRING_MULTIBYTE") func STRING_MULTIBYTE(_ str: Lisp_Object) -> Bool
@_silgen_name("SDATA") func SDATA(_ string: Lisp_Object) -> UnsafePointer<CChar>
@_silgen_name("make_fixnum") func make_fixnum(_ n: Int) -> Lisp_Object
@_silgen_name("FRAME_NS_P") func FRAME_NS_P(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("FRAME_LIVE_P") func FRAME_LIVE_P(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("FRAME_VISIBLE_P") func FRAME_VISIBLE_P(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("FRAME_MINIBUF_ONLY_P")
func FRAME_MINIBUF_ONLY_P(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("FRAME_DISPLAY_INFO")
func FRAME_DISPLAY_INFO(_ f: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<ns_display_info>
@_silgen_name("FRAME_NS_VIEW")
func FRAME_NS_VIEW_borrowed_raw(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
func FRAME_NS_VIEW(_ f: UnsafeMutableRawPointer?) -> NSView? {
  guard let pointer = FRAME_NS_VIEW_borrowed_raw(f) else { return nil }
  return Unmanaged<NSView>.fromOpaque(pointer).takeUnretainedValue()
}
@_silgen_name("FRAME_NS_WINDOW") func FRAME_NS_WINDOW(_ f: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("FRAME_NATIVE_WINDOW") func FRAME_NATIVE_WINDOW(_ f: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("FRAME_DEFAULT_FACE")
func FRAME_DEFAULT_FACE(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("FRAME_DOUBLE_BUFFERED")
func FRAME_DOUBLE_BUFFERED(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("FRAME_MENU_BAR_LINES") func FRAME_MENU_BAR_LINES(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_EXTERNAL_MENU_BAR")
func FRAME_EXTERNAL_MENU_BAR(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_EXTERNAL_TOOL_BAR")
func FRAME_EXTERNAL_TOOL_BAR(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_LINE_HEIGHT") func FRAME_LINE_HEIGHT(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_TAB_BAR_HEIGHT") func FRAME_TAB_BAR_HEIGHT(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_TAB_BAR_LINES") func FRAME_TAB_BAR_LINES(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_TEXT_WIDTH") func FRAME_TEXT_WIDTH(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_TEXT_HEIGHT") func FRAME_TEXT_HEIGHT(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_CHILD_FRAME_BORDER_WIDTH")
func FRAME_CHILD_FRAME_BORDER_WIDTH(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_INTERNAL_BORDER_WIDTH")
func FRAME_INTERNAL_BORDER_WIDTH(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_SELECTED_WINDOW")
func FRAME_SELECTED_WINDOW(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("SET_FRAME_GARBAGED") func SET_FRAME_GARBAGED(_ f: UnsafeMutableRawPointer?)
@_silgen_name("fset_redisplay") func fset_redisplay(_ f: UnsafeMutableRawPointer?)
@_silgen_name("fset_name") func fset_name(_ f: UnsafeMutableRawPointer?, _ name: Lisp_Object)
@_silgen_name("fset_icon_name")
func fset_icon_name(_ f: UnsafeMutableRawPointer?, _ name: Lisp_Object)
@_silgen_name("fset_title") func fset_title(_ f: UnsafeMutableRawPointer?, _ title: Lisp_Object)
/// C: `INLINE void fset_tab_bar_window (struct frame *f, Lisp_Object val)` (src/frame.h 762).
@_silgen_name("fset_tab_bar_window")
func fset_tab_bar_window(_ f: UnsafeMutableRawPointer?, _ val: Lisp_Object)

// The port's names for the `struct frame` / `struct ns_output` members this file reads or
// writes.  The records belong to the core, so each access is a C-ABI entry named after it
// (docs/ports/nsmenu.swift's `frame_left_pos`/`ns_output_in_animation` shape).
@_silgen_name("frame_explicit_name") func frame_explicit_name(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("frame_set_explicit_name")
func frame_set_explicit_name(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("frame_name") func frame_name(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("frame_title") func frame_title(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("frame_icon_name") func frame_icon_name(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("frame_param_alist") func frame_param_alist(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("frame_tab_bar_window") func frame_tab_bar_window(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("frame_tab_bar_resized") func frame_tab_bar_resized(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("frame_set_tab_bar_resized")
func frame_set_tab_bar_resized(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("frame_tab_bar_redisplayed")
func frame_tab_bar_redisplayed(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("frame_set_child_frame_border_width")
func frame_set_child_frame_border_width(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("frame_set_internal_border_width")
func frame_set_internal_border_width(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("frame_set_output_method") func frame_set_output_method(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("frame_set_foreground_pixel")
func frame_set_foreground_pixel(_ f: UnsafeMutableRawPointer?, _ v: UInt)
@_silgen_name("frame_set_background_pixel")
func frame_set_background_pixel(_ f: UnsafeMutableRawPointer?, _ v: UInt)
@_silgen_name("frame_set_wants_modeline")
func frame_set_wants_modeline(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("frame_set_border_width")
func frame_set_border_width(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_output_view")
func ns_output_view_borrowed_raw(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
func ns_output_view(_ f: UnsafeMutableRawPointer?) -> NSView? {
  guard let pointer = ns_output_view_borrowed_raw(f) else { return nil }
  return Unmanaged<NSView>.fromOpaque(pointer).takeUnretainedValue()
}
@_silgen_name("ns_output_set_view")
func ns_output_set_view(_ f: UnsafeMutableRawPointer?, _ v: NSView?)
@_silgen_name("ns_output_miniimage")
func ns_output_miniimage_borrowed_raw(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
func ns_output_miniimage(_ f: UnsafeMutableRawPointer?) -> AnyObject? {
  guard let pointer = ns_output_miniimage_borrowed_raw(f) else { return nil }
  return Unmanaged<AnyObject>.fromOpaque(pointer).takeUnretainedValue()
}
@_silgen_name("ns_output_set_miniimage")
func ns_output_set_miniimage(_ f: UnsafeMutableRawPointer?, _ v: AnyObject?)
@_silgen_name("ns_output_cursor_color")
func ns_output_cursor_color_borrowed_raw(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
func ns_output_cursor_color(_ f: UnsafeMutableRawPointer?) -> NSColor? {
  guard let pointer = ns_output_cursor_color_borrowed_raw(f) else { return nil }
  return Unmanaged<NSColor>.fromOpaque(pointer).takeUnretainedValue()
}
@_silgen_name("ns_output_set_cursor_color")
func ns_output_set_cursor_color(_ f: UnsafeMutableRawPointer?, _ v: NSColor?)
@_silgen_name("ns_output_foreground_color")
func ns_output_foreground_color_borrowed_raw(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
func ns_output_foreground_color(_ f: UnsafeMutableRawPointer?) -> NSColor? {
  guard let pointer = ns_output_foreground_color_borrowed_raw(f) else { return nil }
  return Unmanaged<NSColor>.fromOpaque(pointer).takeUnretainedValue()
}
@_silgen_name("ns_output_set_foreground_color")
func ns_output_set_foreground_color(_ f: UnsafeMutableRawPointer?, _ v: NSColor?)
@_silgen_name("ns_output_background_color")
func ns_output_background_color_borrowed_raw(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
func ns_output_background_color(_ f: UnsafeMutableRawPointer?) -> NSColor? {
  guard let pointer = ns_output_background_color_borrowed_raw(f) else { return nil }
  return Unmanaged<NSColor>.fromOpaque(pointer).takeUnretainedValue()
}
@_silgen_name("ns_output_set_background_color")
func ns_output_set_background_color(_ f: UnsafeMutableRawPointer?, _ v: NSColor?)
@_silgen_name("ns_output_icon_top") func ns_output_icon_top(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("ns_output_set_icon_top")
func ns_output_set_icon_top(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_output_icon_left") func ns_output_icon_left(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("ns_output_set_icon_left")
func ns_output_set_icon_left(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_output_window_desc") func ns_output_window_desc(_ f: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("ns_output_set_window_desc")
func ns_output_set_window_desc(_ f: UnsafeMutableRawPointer?, _ v: UInt)
@_silgen_name("ns_output_parent_desc") func ns_output_parent_desc(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("ns_output_set_parent_desc")
func ns_output_set_parent_desc(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_output_explicit_parent")
func ns_output_explicit_parent(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("ns_output_set_explicit_parent")
func ns_output_set_explicit_parent(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("ns_output_text_cursor")
func ns_output_text_cursor_borrowed_raw(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
func ns_output_text_cursor(_ f: UnsafeMutableRawPointer?) -> NSObject? {
  guard let pointer = ns_output_text_cursor_borrowed_raw(f) else { return nil }
  return Unmanaged<NSObject>.fromOpaque(pointer).takeUnretainedValue()
}
@_silgen_name("ns_output_set_current_pointer")
func ns_output_set_current_pointer(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_in_animation")
func ns_output_in_animation(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("ns_output_set_in_animation")
func ns_output_set_in_animation(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_output_display_info")
func ns_output_display_info(_ f: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<ns_display_info>

/// C: `#define Qfoo builtin_lisp_symbol (iQfoo)`, one per symbol, generated into globals.h
/// (PORTING.md P-17); called as `Qfoo ()`, the Rust mirror's shape.
@_silgen_name("Qnil") func Qnil() -> Lisp_Object
@_silgen_name("Qt") func Qt() -> Lisp_Object
@_silgen_name("Qtop") func Qtop() -> Lisp_Object
@_silgen_name("Qbuffered") func Qbuffered() -> Lisp_Object
@_silgen_name("Qstatic_gray") func Qstatic_gray() -> Lisp_Object
@_silgen_name("Qgray_scale") func Qgray_scale() -> Lisp_Object
@_silgen_name("Qpseudo_color") func Qpseudo_color() -> Lisp_Object
@_silgen_name("Qtrue_color") func Qtrue_color() -> Lisp_Object
@_silgen_name("Qdirect_color") func Qdirect_color() -> Lisp_Object
@_silgen_name("Qactivate") func Qactivate() -> Lisp_Object
@_silgen_name("Qquit") func Qquit() -> Lisp_Object
@_silgen_name("Qdelete_file") func Qdelete_file() -> Lisp_Object
@_silgen_name("Qdelete_directory") func Qdelete_directory() -> Lisp_Object
@_silgen_name("Qfullscreen") func Qfullscreen() -> Lisp_Object
@_silgen_name("Qfullwidth") func Qfullwidth() -> Lisp_Object
@_silgen_name("Qicon_type") func Qicon_type() -> Lisp_Object
@_silgen_name("Qforeground_color") func Qforeground_color() -> Lisp_Object
@_silgen_name("Qbackground_color") func Qbackground_color() -> Lisp_Object
@_silgen_name("Qcursor_color") func Qcursor_color() -> Lisp_Object
@_silgen_name("Qtab_bar_lines") func Qtab_bar_lines() -> Lisp_Object
@_silgen_name("Qtool_bar_lines") func Qtool_bar_lines() -> Lisp_Object
@_silgen_name("Qchild_frame_border_width") func Qchild_frame_border_width() -> Lisp_Object
@_silgen_name("Qinternal_border_width") func Qinternal_border_width() -> Lisp_Object
@_silgen_name("Qwidth") func Qwidth() -> Lisp_Object
@_silgen_name("Qheight") func Qheight() -> Lisp_Object
@_silgen_name("Qicon_left") func Qicon_left() -> Lisp_Object
@_silgen_name("Qicon_top") func Qicon_top() -> Lisp_Object
@_silgen_name("Qunbound") func Qunbound() -> Lisp_Object

// The core's variable storage this file reads or writes.
/// C 4280-4286: `DEFVAR_BOOL ("ns-use-proxy-icon", ns_use_proxy_icon, …)` - the variable is
/// *this* unit's (C's `DEFVAR_BOOL` defines it here), so the storage lives here with the C
/// name's spelling; Swift cannot emit a C-named data symbol (the
/// docs/ports/nsselect.md `Vns_sent_selection_hooks` shape).
@_silgen_name("ns_use_proxy_icon") var ns_use_proxy_icon: Bool
/// C 4268-4279: `DEFVAR_LISP ("ns-icon-type-alist", Vns_icon_type_alist, …)` - this unit's
/// storage, for the same reason.
@_silgen_name("Vns_icon_type_alist") var Vns_icon_type_alist: Lisp_Object
/// C: `extern Lisp_Object Vframe_list;` (`DEFVAR_LISP ("frame-list", …)`, src/frame.c 284).
@_silgen_name("Vframe_list") var Vframe_list: Lisp_Object
/// C: `extern NSString *ns_app_name;` (src/nsterm.h 775, src/nsterm.m's).
@_silgen_name("ns_app_name") var ns_app_name: NSString?
/// C: `extern int x_display_list`-adjacent `struct ns_display_info *x_display_list` (src/nsterm.h 931).
@_silgen_name("x_display_list")
var x_display_list: UnsafeMutablePointer<ns_display_info>?
/// C: `extern int update_mode_lines` (src/window.h 1154) and
/// `extern int windows_or_buffers_changed` (src/window.h 1161) - the two redisplay flags this
/// file bumps.
@_silgen_name("update_mode_lines") var update_mode_lines: Int32
@_silgen_name("windows_or_buffers_changed") var windows_or_buffers_changed: Int32
/// C: `struct ns_display_info`'s `terminal` member's `output_ns` enumerator, and the two
/// `terminal`-record fields `check_ns_display_info` reads (src/termhooks.h's `struct terminal`):
/// the record belongs to the core, so each read is a C-ABI entry.
@_silgen_name("terminal_type") func terminal_type(_ t: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("terminal_id") func terminal_id(_ t: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("terminal_display_info_ns")
func terminal_display_info_ns(_ t: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<ns_display_info>?
/// C: `#define TERMINALP(x)` / `decode_live_terminal (Lisp_Object)` (src/termhooks.h 823,
/// src/terminal.c 508).
@_silgen_name("TERMINALP") func TERMINALP(_ x: Lisp_Object) -> Bool
@_silgen_name("decode_live_terminal")
func decode_live_terminal(_ t: Lisp_Object) -> UnsafeMutableRawPointer?
/// C: `struct frame *decode_window_system_frame (Lisp_Object)` (src/frame.h 1475).
@_silgen_name("decode_window_system_frame")
func decode_window_system_frame(_ frame: Lisp_Object) -> UnsafeMutableRawPointer?
/// C: `#define OUTPUT_NS ...`'s `output_ns` value (src/termhooks.h's `enum output_method`).
@_silgen_name("output_ns") func output_ns() -> Int32
/// C: `bool ns_use_proxy_icon`'s neighbour `ns_use_srgb_colorspace` is src/nsterm.m's; this
/// file's `ns_set_represented_filename` also reads `Vns_icon_type_alist`'s list.

// Constructs of this lane's own headers that other lane files define.
/// C: `+ (EmacsImage *)allocInitFromFile: (Lisp_Object)file` (src/nsterm.h 683, defined
/// src/nsimage.m 280) - a *class* method of src/nsimage.m's port.  A cross-file class method
/// needs a C-ABI entry whose name carries the class; the ledger's `## ABI` (3) records that
/// this one has no definition yet.
@_silgen_name("EmacsImage_allocInitFromFile")
func EmacsImage_allocInitFromFile(_ file: Lisp_Object) -> AnyObject?
/// C: `- (void) setMiniwindowImage: (BOOL)setMini` and
/// `- (void) setFSValue: (int)value` / `- (int) fullscreenState` on `EmacsView`
/// (src/nsterm.h 466-531, defined src/nsterm.m): the port of src/nsterm.m owns them, and a
/// cross-file Objective-C method is a C-ABI entry with the receiver first.
@_silgen_name("setMiniwindowImage")
func setMiniwindowImage(_ view: NSView?, _ mini: Bool)
@_silgen_name("fullscreenState") func fullscreenState(_ view: NSView?) -> Int32
@_silgen_name("setFSValue") func setFSValue(_ view: NSView?, _ value: Int32)

/// C: `typedef void (*frame_parm_handler) (struct frame *, Lisp_Object, Lisp_Object);`
/// (src/frame.h 1867).
typealias frame_parm_handler =
  @convention(c) (UnsafeMutableRawPointer?, Lisp_Object, Lisp_Object) -> Void

// More core constructs the rest of this section uses.
/// C: `Lisp_Object gui_display_get_arg (struct ns_display_info *, Lisp_Object, Lisp_Object,
/// const char *, const char *, enum resource_type)` (src/frame.h 1888).
@_silgen_name("gui_display_get_arg")
func gui_display_get_arg(_ dpyinfo: UnsafeMutablePointer<ns_display_info>?, _ alist: Lisp_Object,
                         _ prop: Lisp_Object, _ val: UnsafePointer<CChar>?,
                         _ cls: UnsafePointer<CChar>?, _ type: Int32) -> Lisp_Object
/// C: `enum resource_type { RES_TYPE_NUMBER, RES_TYPE_STRING, RES_TYPE_SYMBOL,
/// RES_TYPE_BOOLEAN }` (src/frame.h 1870-1876).
@_silgen_name("RES_TYPE_NUMBER") func RES_TYPE_NUMBER() -> Int32
@_silgen_name("RES_TYPE_STRING") func RES_TYPE_STRING() -> Int32
@_silgen_name("RES_TYPE_SYMBOL") func RES_TYPE_SYMBOL() -> Int32
@_silgen_name("RES_TYPE_BOOLEAN") func RES_TYPE_BOOLEAN() -> Int32
/// C: `INLINE void CHECK_FIXNUM (Lisp_Object)` (src/lisp.h 3320) and
/// `Lisp_Object Fassq (Lisp_Object, Lisp_Object)` / `Fmemq` (subrs, globals.h).
@_silgen_name("CHECK_FIXNUM") func CHECK_FIXNUM(_ x: Lisp_Object)
@_silgen_name("Fassq") func Fassq(_ key: Lisp_Object, _ list: Lisp_Object) -> Lisp_Object
@_silgen_name("Fmemq") func Fmemq(_ elt: Lisp_Object, _ list: Lisp_Object) -> Lisp_Object
/// C: `void ns_free_frame_resources (struct frame *)` (src/nsterm.h 1151) and
/// `void free_glyphs (struct frame *)` (src/dispextern.h 3902).
@_silgen_name("ns_free_frame_resources") func ns_free_frame_resources(_ f: UnsafeMutableRawPointer?)
@_silgen_name("free_glyphs") func free_glyphs(_ f: UnsafeMutableRawPointer?)
/// C: the generic frame-parameter handlers src/frame.c (and src/xfaces.c) define, taken as
/// `frame_parm_handler` values by the table above (src/frame.h 1888-1905's declarations).
@_silgen_name("gui_set_autoraise") func gui_set_autoraise(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_autolower") func gui_set_autolower(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_font") func gui_set_font(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_right_divider_width") func gui_set_right_divider_width(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_bottom_divider_width") func gui_set_bottom_divider_width(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_scroll_bar_width") func gui_set_scroll_bar_width(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_scroll_bar_height") func gui_set_scroll_bar_height(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_unsplittable") func gui_set_unsplittable(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_vertical_scroll_bars") func gui_set_vertical_scroll_bars(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_horizontal_scroll_bars") func gui_set_horizontal_scroll_bars(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_visibility") func gui_set_visibility(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_screen_gamma") func gui_set_screen_gamma(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_line_spacing") func gui_set_line_spacing(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_left_fringe") func gui_set_left_fringe(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_right_fringe") func gui_set_right_fringe(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_fullscreen") func gui_set_fullscreen(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_font_backend") func gui_set_font_backend(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_alpha") func gui_set_alpha(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_no_special_glyphs") func gui_set_no_special_glyphs(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_alpha_background") func gui_set_alpha_background(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("gui_set_borders_respect_alpha_background") func gui_set_borders_respect_alpha_background(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
/// C: the seven `ns_set_*` handlers src/nsterm.m defines and src/nsterm.h 1229-1245 declares -
/// the port of src/nsterm.m owns those definitions.
@_silgen_name("ns_set_undecorated") func ns_set_undecorated(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("ns_set_parent_frame") func ns_set_parent_frame(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("ns_set_no_focus_on_map") func ns_set_no_focus_on_map(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("ns_set_no_accept_focus") func ns_set_no_accept_focus(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("ns_set_z_group") func ns_set_z_group(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("ns_set_appearance") func ns_set_appearance(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)
@_silgen_name("ns_set_transparent_titlebar") func ns_set_transparent_titlebar(_ f: UnsafeMutableRawPointer?, _ v: Lisp_Object, _ o: Lisp_Object)

/// C: `extern Lisp_Object selected_frame;` (src/frame.h 1459).
@_silgen_name("selected_frame") var selected_frame: Lisp_Object
/// C: `INLINE void CHECK_STRING (Lisp_Object)` (src/lisp.h 1588) and
/// `AVOID signal_error (const char *, Lisp_Object)` (src/lisp.h 4796) - `AVOID` is `_Noreturn`.
@_silgen_name("CHECK_STRING") func CHECK_STRING(_ x: Lisp_Object)
@_silgen_name("signal_error") func signal_error(_ s: UnsafePointer<CChar>, _ arg: Lisp_Object) -> Never
/// C: `#define NS_FACE_BACKGROUND(f) ((f)->background)` (src/nsterm.h 1071).
@_silgen_name("NS_FACE_BACKGROUND") func NS_FACE_BACKGROUND(_ face: UnsafeMutableRawPointer?) -> UInt
/// C: `struct face`'s `unsigned long background;` (src/dispextern.h 1819): the record is the
/// core's, so the write `face->background = …` (src/nsfns.m 325-327) is a C-ABI entry.
@_silgen_name("face_set_background")
func face_set_background(_ face: UnsafeMutableRawPointer?, _ v: UInt)
/// C: `+[NSColor colorWithUnsignedLong:]` and `-[NSColor unsignedLong]` (src/nsterm.h 359-361,
/// defined src/nsterm.m): cross-file Objective-C methods, so each is a C-ABI entry (the class
/// method carries the class in its name).
@_silgen_name("NSColor_colorWithUnsignedLong")
func NSColor_colorWithUnsignedLong(_ c: UInt) -> NSColor
@_silgen_name("unsignedLong") func unsignedLong(_ col: NSColor) -> UInt
/// C: `struct frame` and `struct window` members this section reads or writes, as C-ABI
/// entries named after them (the `struct frame`/`struct window` records belong to the core).
@_silgen_name("frame_set_menu_bar_lines")
func frame_set_menu_bar_lines(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("frame_set_external_menu_bar")
func frame_set_external_menu_bar(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("frame_set_tab_bar_height")
func frame_set_tab_bar_height(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("frame_set_tab_bar_lines")
func frame_set_tab_bar_lines(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("frame_set_external_tool_bar")
func frame_set_external_tool_bar(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("frame_set_double_buffered")
func frame_set_double_buffered(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("frame_selected_window")
func frame_selected_window(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("window_contents") func window_contents(_ w: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("window_current_matrix")
func window_current_matrix(_ w: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
/// C: `BVAR (XBUFFER (buf), filename)` (src/buffer.h 308's `BVAR`):
/// the `struct buffer` record is the core's, so the read is a C-ABI entry.
@_silgen_name("buffer_filename")
func buffer_filename(_ buf: UnsafeMutableRawPointer?) -> Lisp_Object
/// C: `- (void)setDoubleBuffered:(BOOL)flag` on `EmacsLayer` (src/nsterm.h 752-764, defined
/// src/nsterm.m): src/nsterm.m's port owns it, receiver first.
@_silgen_name("setDoubleBuffered")
func setDoubleBuffered(_ layer: CALayer?, _ flag: Bool)
/// C: `enum fullscreen_type` members (src/frame.h 49-56) - the values `handleFS` and
/// `setFSValue:` use.
@_silgen_name("FULLSCREEN_NONE") func FULLSCREEN_NONE() -> Int32
@_silgen_name("FULLSCREEN_WIDTH") func FULLSCREEN_WIDTH() -> Int32
@_silgen_name("FULLSCREEN_HEIGHT") func FULLSCREEN_HEIGHT() -> Int32
@_silgen_name("FULLSCREEN_MAXIMIZED") func FULLSCREEN_MAXIMIZED() -> Int32

// ==========================================================================
// Lane ABI (batch 2): the constructs C 1192-1884 reads.
// ==========================================================================

/// C: `void record_unwind_protect (void (*) (Lisp_Object), Lisp_Object)` (src/lisp.h 4818)
/// and `Lisp_Object unbind_to (specpdl_ref, Lisp_Object)` (src/lisp.h 4833).
@_silgen_name("record_unwind_protect")
func record_unwind_protect(_ fn: @convention(c) (Lisp_Object) -> Void, _ arg: Lisp_Object)
@_silgen_name("unbind_to")
func unbind_to(_ count: specpdl_ref, _ value: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object gui_default_parameter (struct frame *, Lisp_Object, Lisp_Object,
/// Lisp_Object, const char *, const char *, enum resource_type)` (src/dispextern.h 4001).
@_silgen_name("gui_default_parameter")
func gui_default_parameter(_ f: UnsafeMutableRawPointer?, _ alist: Lisp_Object,
                           _ prop: Lisp_Object, _ defval: Lisp_Object,
                           _ xprop: UnsafePointer<CChar>?, _ xclass: UnsafePointer<CChar>?,
                           _ type: Int32) -> Lisp_Object
/// C: `struct frame *make_frame_without_minibuffer (Lisp_Object, struct kboard *, Lisp_Object)`
/// (src/frame.h 1491), `struct frame *make_minibuffer_frame (void)` (src/frame.h 1489) and
/// `struct frame *make_frame (bool)` (src/frame.c 581).
@_silgen_name("make_frame_without_minibuffer")
func make_frame_without_minibuffer(_ mini_window: Lisp_Object, _ kb: UnsafeMutableRawPointer?,
                                   _ display: Lisp_Object) -> UnsafeMutableRawPointer?
@_silgen_name("make_minibuffer_frame") func make_minibuffer_frame() -> UnsafeMutableRawPointer?
@_silgen_name("make_frame") func make_frame(_ mini: Bool) -> UnsafeMutableRawPointer?
/// C: `EMACS_UINT frame_set_id_from_params (struct frame *, Lisp_Object)` (src/frame.h 1430).
@_silgen_name("frame_set_id_from_params")
func frame_set_id_from_params(_ f: UnsafeMutableRawPointer?, _ params: Lisp_Object) -> UInt
/// C: `void *xzalloc (size_t)` (src/lisp.h 5537) - `xmalloc` + zero fill.
@_silgen_name("xzalloc") func xzalloc(_ size: Int) -> UnsafeMutableRawPointer?
/// C: `void init_frame_faces (struct frame *)` (src/dispextern.h 3821).
@_silgen_name("init_frame_faces") func init_frame_faces(_ f: UnsafeMutableRawPointer?)
/// C: `long gui_figure_window_size (struct frame *, Lisp_Object, bool, bool)`
/// (src/frame.h 1865).
@_silgen_name("gui_figure_window_size")
func gui_figure_window_size(_ f: UnsafeMutableRawPointer?, _ parms: Lisp_Object,
                            _ toolbar_p: Bool, _ menubar_p: Bool) -> Int
/// C: `void check_window_system (struct frame *)` (src/frame.h 1504) and
/// `Lisp_Object quit (void)` (src/lisp.h 4786).
@_silgen_name("check_window_system") func check_window_system(_ f: UnsafeMutableRawPointer?)
@_silgen_name("quit") func quit() -> Lisp_Object
/// C: `struct frame *decode_live_frame (Lisp_Object)` (src/frame.h 1471).
@_silgen_name("decode_live_frame")
func decode_live_frame(_ frame: Lisp_Object) -> UnsafeMutableRawPointer?
/// C: `Lisp_Object Fcopy_alist (Lisp_Object)` and `Ffile_directory_p` (subrs, globals.h).
@_silgen_name("Fcopy_alist") func Fcopy_alist(_ alist: Lisp_Object) -> Lisp_Object
@_silgen_name("Ffile_directory_p") func Ffile_directory_p(_ file: Lisp_Object) -> Lisp_Object
/// C: `#define FRAME_HAS_MINIBUF_P(f)` (src/frame.h 1051) and
/// `#define FRAME_PARENT_FRAME(f) ((f)->parent_frame)` (src/frame.h 1103).
@_silgen_name("FRAME_HAS_MINIBUF_P")
func FRAME_HAS_MINIBUF_P(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("FRAME_PARENT_FRAME")
func FRAME_PARENT_FRAME(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
/// C: `#define FRAME_UNDECORATED(f)` / `FRAME_NS_TRANSPARENT_TITLEBAR(f)` (src/frame.h 1239,
/// 1259) - read *and* written by `x-create-frame`.
@_silgen_name("FRAME_UNDECORATED") func FRAME_UNDECORATED(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("frame_set_undecorated")
func frame_set_undecorated(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("FRAME_NS_TRANSPARENT_TITLEBAR")
func FRAME_NS_TRANSPARENT_TITLEBAR(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("frame_set_ns_transparent_titlebar")
func frame_set_ns_transparent_titlebar(_ f: UnsafeMutableRawPointer?, _ v: Bool)
/// C: `#define USPosition (1L << 0)` / `#define PPosition (1L << 2)` (src/nsgui.h 125, 128).
/// C: `#define SELECTED_FRAME()` (src/frame.h 1526-1528).
@_silgen_name("SELECTED_FRAME") func SELECTED_FRAME() -> UnsafeMutableRawPointer?
@_silgen_name("USPosition") func USPosition() -> Int
@_silgen_name("PPosition") func PPosition() -> Int
/// C: `enum` member of `enum ns_app_delegate_data_2` (src/nsterm.h 779):
/// `NSAPP_DATA2_RUNFILEDIALOG`.
@_silgen_name("NSAPP_DATA2_RUNFILEDIALOG") func NSAPP_DATA2_RUNFILEDIALOG() -> Int
@_silgen_name("NSAPP_DATA2_RUNASSCRIPT") func NSAPP_DATA2_RUNASSCRIPT() -> Int
/// C: `#define KVAR(kboard, field) ((kboard)->field ## _)` (src/keyboard.h 38) and
/// `INLINE void kset_default_minibuffer_frame (struct kboard *, Lisp_Object)`
/// (src/keyboard.h 188).
@_silgen_name("kboard_default_minibuffer_frame")
func kboard_default_minibuffer_frame(_ kb: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("kset_default_minibuffer_frame")
func kset_default_minibuffer_frame(_ kb: UnsafeMutableRawPointer?, _ val: Lisp_Object)
/// C: `void ns_set_offset (struct frame *, int, int, int)` (src/nsterm.h 1126),
/// `void ns_make_frame_visible (struct frame *)` (1226) and
/// `void ns_iconify_frame (struct frame *)` (1228) - src/nsterm.m's.
@_silgen_name("ns_set_offset")
func ns_set_offset(_ f: UnsafeMutableRawPointer?, _ xoff: Int32, _ yoff: Int32, _ type: Int32)
@_silgen_name("ns_make_frame_visible") func ns_make_frame_visible(_ f: UnsafeMutableRawPointer?)
@_silgen_name("ns_iconify_frame") func ns_iconify_frame(_ f: UnsafeMutableRawPointer?)
/// C: `void ns_set_appearance_1 (struct frame *, Lisp_Object)` (src/nsterm.h 1240) -
/// src/nsterm.m's.
@_silgen_name("ns_set_appearance_1")
func ns_set_appearance_1(_ f: UnsafeMutableRawPointer?, _ value: Lisp_Object)
/// C: `void mac_register_font_driver (struct frame *)` (src/macfont.h 27) - src/macfont.m's
/// port exports it (docs/ports/macfont.md).
@_silgen_name("mac_register_font_driver")
func mac_register_font_driver(_ f: UnsafeMutableRawPointer?)
/// C: `char *xstrdup (char const *)` (src/lisp.h 5551) and `void xfree (void *)` (5543).
@_silgen_name("xstrdup") func xstrdup(_ s: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?
@_silgen_name("xfree") func xfree(_ p: UnsafeMutableRawPointer?)
/// C: `ENCODE_FILE (Lisp_Object)` / `DECODE_FILE (Lisp_Object)` (src/coding.h 648, 652) and
/// `INLINE Lisp_Object Ffile_directory_p`'s neighbour `current_buffer`
/// (`#define current_buffer (current_thread->m_current_buffer)`, src/thread.h 147) with
/// `BVAR (current_buffer, directory)` (src/buffer.h 308).
@_silgen_name("ENCODE_FILE") func ENCODE_FILE(_ file: Lisp_Object) -> Lisp_Object
@_silgen_name("DECODE_FILE") func DECODE_FILE(_ file: Lisp_Object) -> Lisp_Object
@_silgen_name("current_buffer") func current_buffer() -> UnsafeMutableRawPointer?
@_silgen_name("buffer_directory")
func buffer_directory(_ buf: UnsafeMutableRawPointer?) -> Lisp_Object
/// C: `struct terminal`'s `kboard`, `name` and `reference_count` members (src/termhooks.h):
/// the record is the core's, so each read/write is a C-ABI entry.
@_silgen_name("terminal_kboard")
func terminal_kboard(_ t: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("terminal_name") func terminal_name(_ t: UnsafeMutableRawPointer?) -> UnsafePointer<CChar>?
@_silgen_name("terminal_increment_reference_count")
func terminal_increment_reference_count(_ t: UnsafeMutableRawPointer?)
/// C: the `struct frame` / `struct ns_output` members C 1192-1884 writes, as C-ABI entries
/// (the records belong to the core; the same shape as the batch-1 `frame_set_*` group).
@_silgen_name("frame_set_terminal")
func frame_set_terminal(_ f: UnsafeMutableRawPointer?, _ t: UnsafeMutableRawPointer?)
@_silgen_name("frame_alloc_ns_output") func frame_alloc_ns_output(_ f: UnsafeMutableRawPointer?)
@_silgen_name("frame_set_fontset") func frame_set_fontset(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("frame_set_no_split") func frame_set_no_split(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("frame_set_was_invisible") func frame_set_was_invisible(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("frame_set_can_set_window_size")
func frame_set_can_set_window_size(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("fset_param_alist")
func fset_param_alist(_ f: UnsafeMutableRawPointer?, _ alist: Lisp_Object)
@_silgen_name("fset_parent_frame")
func fset_parent_frame(_ f: UnsafeMutableRawPointer?, _ parent: Lisp_Object)
@_silgen_name("frame_left_pos") func frame_left_pos(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_top_pos") func frame_top_pos(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_COLS") func FRAME_COLS(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_COLUMN_WIDTH") func FRAME_COLUMN_WIDTH(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_LINES") func FRAME_LINES(_ f: UnsafeMutableRawPointer?) -> Int32
/// C: `struct ns_output`'s fifteen `Emacs_Cursor` members and the two on `struct
/// ns_display_info` (src/nsterm.h 878-913): each is set by `x-create-frame`, so each is an
/// accessor of the C member's own name.
@_silgen_name("ns_output_set_text_cursor")
func ns_output_set_text_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_nontext_cursor")
func ns_output_set_nontext_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_modeline_cursor")
func ns_output_set_modeline_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_hand_cursor")
func ns_output_set_hand_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_hourglass_cursor")
func ns_output_set_hourglass_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_horizontal_drag_cursor")
func ns_output_set_horizontal_drag_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_vertical_drag_cursor")
func ns_output_set_vertical_drag_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_left_edge_cursor")
func ns_output_set_left_edge_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_top_left_corner_cursor")
func ns_output_set_top_left_corner_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_top_edge_cursor")
func ns_output_set_top_edge_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_top_right_corner_cursor")
func ns_output_set_top_right_corner_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_right_edge_cursor")
func ns_output_set_right_edge_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_bottom_right_corner_cursor")
func ns_output_set_bottom_right_corner_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_bottom_edge_cursor")
func ns_output_set_bottom_edge_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_bottom_left_corner_cursor")
func ns_output_set_bottom_left_corner_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_display_info_set_vertical_scroll_bar_cursor")
func ns_display_info_set_vertical_scroll_bar_cursor(
  _ dpyinfo: UnsafeMutablePointer<ns_display_info>?, _ v: NSObject?)
@_silgen_name("ns_display_info_set_horizontal_scroll_bar_cursor")
func ns_display_info_set_horizontal_scroll_bar_cursor(
  _ dpyinfo: UnsafeMutablePointer<ns_display_info>?, _ v: NSObject?)
/// C: the `EmacsView`/`EmacsWindow` messages C 1614-1884 sends across files - each is a C-ABI
/// entry with the receiver first, and the port of src/nsterm.m owns the definitions.
@_silgen_name("emacs_view_frame")
func emacs_view_frame(_ view: AnyObject?) -> UnsafeMutableRawPointer?
@_silgen_name("is_emacs_view") func is_emacs_view(_ obj: AnyObject?) -> Bool
@_silgen_name("showFontPanel") func showFontPanel(_ view: NSView?) -> Lisp_Object
@_silgen_name("restackWindow")
func restackWindow(_ window: AnyObject?, _ window2: NSWindow?, _ above: Bool) -> Bool
@_silgen_name("EmacsView_initFrameFromEmacs")
func EmacsView_initFrameFromEmacs(_ f: UnsafeMutableRawPointer?) -> AnyObject?
/// C: `id` values C 1614-1884 passes as the `EmacsView`/`EmacsWindow` receivers:
/// `[NSApp orderedWindows]`' delegates and `[FRAME_NS_VIEW (f) window]` - the latter is the
/// SDK's, the former an `AnyObject?` the port keeps untyped.

/// C: `PVEC_SUBR` (src/lisp.h 999).
@_silgen_name("PVEC_SUBR") func PVEC_SUBR() -> Int
/// C: `PSEUDOVECTOR_AREA_BITS` (src/lisp.h 1039) - the second of the two enumerators the
/// DEFUN record's header word above is built from.
@_silgen_name("PSEUDOVECTOR_AREA_BITS") func PSEUDOVECTOR_AREA_BITS() -> Int
/// C: `SPECPDL_INDEX ()` (src/lisp.h 3544-3660) - the macro over the specpdl stack that
/// returns a `specpdl_ref`.
@_silgen_name("SPECPDL_INDEX") func SPECPDL_INDEX() -> specpdl_ref
/// C: `#define XSETFRAME(a, b) (XSETPSEUDOVECTOR (a, b, PVEC_FRAME))` (src/frame.h 1309) - the
/// macro assigns to its first argument, so the entry takes its address.
@_silgen_name("XSETFRAME") func XSETFRAME(_ a: UnsafeMutablePointer<Lisp_Object>, _ b: UnsafeMutableRawPointer?)
/// C: `void specbind (Lisp_Object symbol, Lisp_Object value)` (src/lisp.h 4817).
@_silgen_name("specbind") func specbind(_ symbol: Lisp_Object, _ value: Lisp_Object)
/// C: `FRAME_DISPLAY_INFO (f) = dpyinfo` (src/nsterm.h 1032's macro, written at src/nsfns.m
/// 1283): the `struct frame` record belongs to the core, so the write is a C-ABI entry.
@_silgen_name("frame_set_display_info")
func frame_set_display_info(_ f: UnsafeMutableRawPointer?, _ dpyinfo: UnsafeMutablePointer<ns_display_info>?)

/// ==========================================================================
// Lane ABI (batch 6): the constructs C 3802-4348 reads.
// ==========================================================================

/// C: `struct Lisp_Fwd { ENUM_BF (Lisp_Fwd_Type) type : 8; union { … } u; }` (src/lisp.h
/// 3118-3133).  The bitfield is one byte and the union one slot; the port keeps the union's
/// single slot raw and passes each member's address (`objvar`, `boolvar`), exactly as C's
/// `DEFVAR_LISP`/`DEFVAR_BOOL` do (src/lisp.h 3495-3512).  The shape is
/// docs/ports/nsselect.swift's, extended to the `bool` member.
struct Lisp_Fwd_u {
  var ptr: UnsafeMutableRawPointer?
}
struct Lisp_Fwd {
  var type: UInt8
  var u: Lisp_Fwd_u
}
/// C: `Lisp_Fwd_Bool` (src/lisp.h 531) and `Lisp_Fwd_Obj` (src/lisp.h 532), the two
/// `enum Lisp_Fwd_Type` members this file uses; `defvar_lisp` (src/lisp.h 3472) and
/// `defvar_bool` (src/lisp.h 3474) are the registering calls.
/// C: `void staticpro (Lisp_Object const *)` (src/lisp.h 3919).
@_silgen_name("Lisp_Fwd_Obj") func Lisp_Fwd_Obj() -> UInt8
@_silgen_name("Lisp_Fwd_Bool") func Lisp_Fwd_Bool() -> UInt8
@_silgen_name("defvar_lisp")
func defvar_lisp(_ o_fwd: UnsafePointer<Lisp_Fwd>?, _ namestring: UnsafePointer<CChar>?, docstring: StaticString)
@_silgen_name("defvar_bool")
func defvar_bool(_ b_fwd: UnsafePointer<Lisp_Fwd>?, _ namestring: UnsafePointer<CChar>?, docstring: StaticString)
@_silgen_name("staticpro") func staticpro(_ varaddress: UnsafePointer<Lisp_Object>?)
/// C: `INLINE bool FLOATP (Lisp_Object)` (src/lisp.h 1596) and
/// `INLINE double XFLOAT_DATA (Lisp_Object)` (src/lisp.h 3176).
@_silgen_name("FLOATP") func FLOATP(_ x: Lisp_Object) -> Bool
@_silgen_name("XFLOAT_DATA") func XFLOAT_DATA(_ x: Lisp_Object) -> Double
/// C: `INLINE void CHECK_LIST (Lisp_Object)` (src/lisp.h 3326).
@_silgen_name("CHECK_LIST") func CHECK_LIST(_ x: Lisp_Object)
/// C: `void window_box (struct window *, enum glyph_row_area, int *, int *, int *, int *)`
/// (src/dispextern.h 4009), whose area argument's enumerator is:
/// C: `TEXT_AREA` (src/dispextern.h 885).
@_silgen_name("window_box")
func window_box(_ w: UnsafeMutableRawPointer?, _ area: Int32, _ box_x: UnsafeMutablePointer<Int32>,
                _ box_y: UnsafeMutablePointer<Int32>, _ box_width: UnsafeMutablePointer<Int32>,
                _ box_height: UnsafeMutablePointer<Int32>)
@_silgen_name("TEXT_AREA") func TEXT_AREA() -> Int32
/// C: `struct window`'s `phys_cursor` (src/window.h 332), whose record is `struct cursor_pos`
/// (src/window.h 91).  Its two members are read, the record belonging to the core.
@_silgen_name("window_phys_cursor_x") func window_phys_cursor_x(_ w: UnsafeMutableRawPointer?) -> Int
@_silgen_name("window_phys_cursor_y") func window_phys_cursor_y(_ w: UnsafeMutableRawPointer?) -> Int
/// C: `extern Lisp_Object selected_window;` (src/window.h 1141).
@_silgen_name("selected_window") var selected_window: Lisp_Object
/// C: `Lisp_Object string_to_multibyte (Lisp_Object)` (src/character.c 448),
/// `INLINE ptrdiff_t SCHARS (Lisp_Object)` (src/lisp.h 1870),
/// `int string_char_advance (const unsigned char **)` (src/character.h 346) and
/// `Lisp_Object make_multibyte_string (const char *, ptrdiff_t, ptrdiff_t)` (src/lisp.h 4562).
@_silgen_name("string_to_multibyte") func string_to_multibyte(_ string: Lisp_Object) -> Lisp_Object
@_silgen_name("SCHARS") func SCHARS(_ str: Lisp_Object) -> Int
@_silgen_name("string_char_advance")
func string_char_advance(_ p: UnsafeMutablePointer<UnsafePointer<UInt8>?>) -> Int32
@_silgen_name("make_multibyte_string")
func make_multibyte_string(_ str: UnsafePointer<CChar>?, _ nchars: Int, _ nbytes: Int) -> Lisp_Object
/// More `Q…` symbols (generated globals.h, PORTING.md P-17; called as `Qfoo ()`).
@_silgen_name("Qinformational") func Qinformational() -> Lisp_Object
@_silgen_name("Qcritical") func Qcritical() -> Lisp_Object

// ==========================================================================
// Lane ABI (batch 5): the constructs C 3229-3800 reads.
// ==========================================================================

/// C: `INLINE void CHECK_FIXNAT (Lisp_Object)` (src/lisp.h 3364) and
/// `intmax_t check_integer_range (Lisp_Object, intmax_t, intmax_t)` (src/lisp.h 608).
@_silgen_name("CHECK_FIXNAT") func CHECK_FIXNAT(_ x: Lisp_Object)
@_silgen_name("check_integer_range")
func check_integer_range(_ x: Lisp_Object, _ lo: Int, _ hi: Int) -> Int
/// C: `INLINE Lisp_Object list4i (intmax_t, intmax_t, intmax_t, intmax_t)` (src/lisp.h 4532)
/// and `#define list(...)` over `listn` (src/lisp.h 4493-4494).  `listn` is variadic, which
/// Swift cannot call, so the port folds the array with `Fcons` - the same list value (the
/// `calln` shape of docs/ports/nsselect.swift).
@_silgen_name("list4i")
func list4i(_ a: Int, _ b: Int, _ c: Int, _ d: Int) -> Lisp_Object
@inline(__always)
fileprivate func list(_ args: [Lisp_Object]) -> Lisp_Object {
  var out: Lisp_Object = Qnil()
  for a in args.reversed() { out = Fcons(a, out) }
  return out
}
/// C: `Lisp_Object Fequal (Lisp_Object, Lisp_Object)` / `Fequal_including_properties`
/// (subrs, globals.h), and `Lisp_Object Fget_buffer_create (Lisp_Object, Lisp_Object)` /
/// `Ferase_buffer` / `Finsert` / `Fwindow_text_pixel_size` (subrs, globals.h).
@_silgen_name("Fequal") func Fequal(_ a: Lisp_Object, _ b: Lisp_Object) -> Lisp_Object
@_silgen_name("Fequal_including_properties")
func Fequal_including_properties(_ a: Lisp_Object, _ b: Lisp_Object) -> Lisp_Object
@_silgen_name("Fget_buffer_create")
func Fget_buffer_create(_ name: Lisp_Object, _ base_buffer: Lisp_Object) -> Lisp_Object
@_silgen_name("Ferase_buffer") func Ferase_buffer() -> Lisp_Object
@_silgen_name("Finsert") func Finsert(_ nargs: Int, _ args: UnsafeMutablePointer<Lisp_Object>) -> Lisp_Object
@_silgen_name("Fwindow_text_pixel_size")
func Fwindow_text_pixel_size(_ window: Lisp_Object, _ from: Lisp_Object, _ to: Lisp_Object,
                             _ x_limit: Lisp_Object, _ y_limit: Lisp_Object,
                             _ mode_lines: Lisp_Object, _ pixelwise: Lisp_Object) -> Lisp_Object
/// C: `void set_window_buffer (Lisp_Object, Lisp_Object, bool, bool)` (src/window.h 965) and
/// `Lisp_Object Fset_window_parameter (Lisp_Object, Lisp_Object, Lisp_Object)` (subr).
@_silgen_name("set_window_buffer")
func set_window_buffer(_ window: Lisp_Object, _ buffer: Lisp_Object, _ run_hooks: Bool,
                       _ keep_margins: Bool)
@_silgen_name("Fset_window_parameter")
func Fset_window_parameter(_ window: Lisp_Object, _ alist: Lisp_Object, _ value: Lisp_Object) -> Lisp_Object
/// C: `int try_window (Lisp_Object, struct text_pos, int)` (src/dispextern.h 3573),
/// `void update_single_window (struct window *)` (src/dispextern.h 3918) and
/// `INLINE void flush_frame (struct frame *f)` (src/frame.h 1914).
@_silgen_name("try_window")
func try_window(_ window: Lisp_Object, _ pos: text_pos, _ flags: Int32) -> Bool
@_silgen_name("update_single_window")
func update_single_window(_ w: UnsafeMutableRawPointer?)
@_silgen_name("flush_frame") func flush_frame(_ f: UnsafeMutableRawPointer?)
/// C: `INLINE void bset_left_margin_cols (struct buffer *, Lisp_Object)` and
/// `bset_right_margin_cols` / `bset_truncate_lines` (src/buffer.h 779-782, the `bset` family):
/// `struct buffer`'s fields belong to the core.
/// C: `void set_buffer_internal_1 (struct buffer *b)` (src/buffer.h 1113).
@_silgen_name("set_buffer_internal_1")
func set_buffer_internal_1(_ b: UnsafeMutableRawPointer?)
@_silgen_name("bset_left_margin_cols")
func bset_left_margin_cols(_ b: UnsafeMutableRawPointer?, _ v: Lisp_Object)
@_silgen_name("bset_right_margin_cols")
func bset_right_margin_cols(_ b: UnsafeMutableRawPointer?, _ v: Lisp_Object)
@_silgen_name("bset_truncate_lines")
func bset_truncate_lines(_ b: UnsafeMutableRawPointer?, _ v: Lisp_Object)
/// C: `struct text_pos` (src/lisp.h 1103-1108): `ptrdiff_t charpos, bytepos`, and C's
/// `SET_TEXT_POS (pos, charpos, bytepos)` (src/dispextern.h 291-297).
struct text_pos {
  var charpos: Int
  var bytepos: Int
}
/// C: `INLINE void SET_FRAME_VISIBLE (struct frame *f, bool v)` (src/frame.h 1537),
/// `FRAME_INITIAL_P (f)`
/// (src/frame.h 932), `FRAME_ROOT_WINDOW (f)` (src/frame.h 1201),
/// `FRAME_TOTAL_COLS (f)` (src/frame.h 1068), `FRAME_PIXEL_WIDTH/HEIGHT (f)`
/// (src/frame.h 1066-1067), `FRAME_TOOL_BAR_POSITION (f)` (src/frame.h 1104),
/// `FRAME_NS_TITLEBAR_HEIGHT (f)` / `FRAME_TOOLBAR_HEIGHT (f)` (src/nsterm.h 1062, 1071) and
/// the two position macros `BEGV`/`BEGV_BYTE` (src/buffer.h 41-42).
@_silgen_name("SET_FRAME_VISIBLE")
func SET_FRAME_VISIBLE(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("FRAME_INITIAL_P") func FRAME_INITIAL_P(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("FRAME_ROOT_WINDOW") func FRAME_ROOT_WINDOW(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("FRAME_TOTAL_COLS") func FRAME_TOTAL_COLS(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_set_total_cols")
func frame_set_total_cols(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("FRAME_PIXEL_WIDTH") func FRAME_PIXEL_WIDTH(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_set_pixel_width")
func frame_set_pixel_width(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("FRAME_PIXEL_HEIGHT") func FRAME_PIXEL_HEIGHT(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_set_pixel_height")
func frame_set_pixel_height(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("FRAME_TOOL_BAR_POSITION")
func FRAME_TOOL_BAR_POSITION(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
/// C: `#define frame_ns_titlebar_height(f) …` (src/nsterm.h 1062-1068, "Compute pixel height of
/// the frame's titlebar") and `#define frame_toolbar_height(f) …` (src/nsterm.h 1071-1079).
/// **Neither has a symbol anywhere in the port** (measured: `nm -g` over
/// `build/emacs-ref/{src,lib}/*.o` finds none), so per 3.255's ruling - a name with no real C
/// symbol cannot be a C-ABI entry (P-08) - this unit spells each macro's *body* instead of
/// importing its name.  The bodies need only AppKit and the existing `FRAME_NS_VIEW` entry
/// (the same shape docs/ports/nsterm.md gave the same two macros).
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
/// `struct frame`'s `border_width` field (`f->border_width`), which C has no `FRAME_BORDER_WIDTH`
/// macro for: a port-named accessor, like the other lowercase `frame_*` entries.
@_silgen_name("frame_border_width") func frame_border_width(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("current_buffer_begv") func current_buffer_begv() -> Int
@_silgen_name("current_buffer_begv_byte") func current_buffer_begv_byte() -> Int
/// C: `enum ... { TRY_WINDOW_IGNORE_FONTS_CHANGE = (1 << 1) }` (src/dispextern.h 3700).
@_silgen_name("TRY_WINDOW_IGNORE_FONTS_CHANGE") func TRY_WINDOW_IGNORE_FONTS_CHANGE() -> Int32
/// C: `struct window`'s members C 3430-3520 reads or writes (src/window.h): the record belongs
/// to the core, so each access is a C-ABI entry named after the member.
@_silgen_name("window_set_pseudo_window_p")
func window_set_pseudo_window_p(_ w: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("window_set_left_col")
func window_set_left_col(_ w: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("window_set_top_line")
func window_set_top_line(_ w: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("window_set_pixel_left")
func window_set_pixel_left(_ w: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("window_set_pixel_top")
func window_set_pixel_top(_ w: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("window_total_cols") func window_total_cols(_ w: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("window_set_total_cols")
func window_set_total_cols(_ w: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("window_total_lines") func window_total_lines(_ w: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("window_set_total_lines")
func window_set_total_lines(_ w: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("window_pixel_width") func window_pixel_width(_ w: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("window_set_pixel_width")
func window_set_pixel_width(_ w: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("window_pixel_height") func window_pixel_height(_ w: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("window_set_pixel_height")
func window_set_pixel_height(_ w: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("window_desired_matrix")
func window_desired_matrix(_ w: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("window_set_must_be_updated_p")
func window_set_must_be_updated_p(_ w: UnsafeMutableRawPointer?, _ v: Bool)
/// C: `- (instancetype)init` (src/nsmenu.m's `EmacsTooltip`, docs/ports/nsmenu.md),
/// `- (void)setBackgroundColor:`, `- (void)setForegroundColor:`, `- (void)setText:`,
/// `- (NSRect)frame`, `- (void)showAtX:(int)x Y:(int)y for:(int)seconds`: cross-file methods
/// of that Swift class, receiver first.
@_silgen_name("EmacsTooltip_new") func EmacsTooltip_new() -> AnyObject?
@_silgen_name("EmacsTooltip_setBackgroundColor")
func EmacsTooltip_setBackgroundColor(_ tip: AnyObject?, _ col: NSColor?)
@_silgen_name("EmacsTooltip_setForegroundColor")
func EmacsTooltip_setForegroundColor(_ tip: AnyObject?, _ col: NSColor?)
@_silgen_name("EmacsTooltip_setText")
func EmacsTooltip_setText(_ tip: AnyObject?, _ text: UnsafePointer<CChar>?)
@_silgen_name("EmacsTooltip_frame") func EmacsTooltip_frame(_ tip: AnyObject?) -> NSRect
@_silgen_name("EmacsTooltip_showAtX_Y_for")
func EmacsTooltip_showAtX_Y_for(_ tip: AnyObject?, _ x: Int32, _ y: Int32, _ seconds: Int32)
/// The core's tooltip-related storage: `DEFVAR_LISP ("x-show-tooltip-timeout", …)`,
/// `DEFVAR_LISP ("x-max-tooltip-size", …)` and the `bool tooltip_reuse_hidden_frame`
/// (`DEFVAR_BOOL ("tooltip-reuse-hidden-frame", …)`) - all in src/xfns.c's family, mirrored by
/// the port's core.
@_silgen_name("Vx_show_tooltip_timeout") var Vx_show_tooltip_timeout: Lisp_Object
/// C 4235-4237: `DEFVAR_LISP ("x-max-tooltip-size", Vx_max_tooltip_size, …)` - this unit's
/// storage (`xfns.c` has the same DEFVAR, but only one of the two files is ever compiled).
@_silgen_name("Vx_max_tooltip_size") var Vx_max_tooltip_size: Lisp_Object
@_silgen_name("tooltip_reuse_hidden_frame") var tooltip_reuse_hidden_frame: Bool
/// More `Q…` symbols (generated globals.h, PORTING.md P-17; called as `Qfoo ()`).
@_silgen_name("Qassq_delete_all") func Qassq_delete_all() -> Lisp_Object
@_silgen_name("Qno_other_window") func Qno_other_window() -> Lisp_Object
@_silgen_name("Qinhibit_read_only") func Qinhibit_read_only() -> Lisp_Object
@_silgen_name("Qinhibit_modification_hooks") func Qinhibit_modification_hooks() -> Lisp_Object
@_silgen_name("Qinhibit_point_motion_hooks") func Qinhibit_point_motion_hooks() -> Lisp_Object
@_silgen_name("Qrun_at_time") func Qrun_at_time() -> Lisp_Object
@_silgen_name("Qx_hide_tip") func Qx_hide_tip() -> Lisp_Object
@_silgen_name("Qfullboth") func Qfullboth() -> Lisp_Object
@_silgen_name("Qouter_edges") func Qouter_edges() -> Lisp_Object
@_silgen_name("Qnative_edges") func Qnative_edges() -> Lisp_Object
@_silgen_name("Qinner_edges") func Qinner_edges() -> Lisp_Object
@_silgen_name("Qouter_position") func Qouter_position() -> Lisp_Object
@_silgen_name("Qouter_size") func Qouter_size() -> Lisp_Object
@_silgen_name("Qexternal_border_size") func Qexternal_border_size() -> Lisp_Object
@_silgen_name("Qtitle_bar_size") func Qtitle_bar_size() -> Lisp_Object
@_silgen_name("Qmenu_bar_external") func Qmenu_bar_external() -> Lisp_Object
@_silgen_name("Qmenu_bar_size") func Qmenu_bar_size() -> Lisp_Object
@_silgen_name("Qtool_bar_external") func Qtool_bar_external() -> Lisp_Object
@_silgen_name("Qtool_bar_position") func Qtool_bar_position() -> Lisp_Object
@_silgen_name("Qtool_bar_size") func Qtool_bar_size() -> Lisp_Object
@_silgen_name("Qleft") func Qleft() -> Lisp_Object

// ==========================================================================
// Lane ABI (batch 4): the constructs C 2612-3227 reads.
// ==========================================================================

/// C: `typedef struct { int x, y; unsigned width, height; } Emacs_Rectangle;`
/// (src/dispextern.h 63-69, the non-X arm).
struct Emacs_Rectangle {
  var x: Int32
  var y: Int32
  var width: UInt32
  var height: UInt32
}

/// C: `struct MonitorInfo` (src/frame.h 1928-1935): `Emacs_Rectangle geom, work; int mm_width,
/// mm_height; char *name;`.  `make_monitor_attribute_list`/`free_monitors` (the core) read
/// this record, so the mirror's layout is asserted in the ledger's Evidence.
struct MonitorInfo {
  var geom: Emacs_Rectangle
  var work: Emacs_Rectangle
  var mm_width: Int32
  var mm_height: Int32
  var name: UnsafeMutablePointer<CChar>?
}

/// C: `NSColorSpaceFromDepth (NSWindowDepth)` and `NSBitsPerPixelFromDepth (NSWindowDepth)`
/// (the SDK's NSGraphics.h 166, 168) - like `NSBestDepth`, AppKit's Swift interface does not
/// vend them, so the port binds the C symbols (system functions, not the core's).
@_silgen_name("NSColorSpaceFromDepth")
func NSColorSpaceFromDepth(_ depth: NSWindow.Depth) -> NSColorSpaceName?
@_silgen_name("NSBitsPerPixelFromDepth")
func NSBitsPerPixelFromDepth(_ depth: NSWindow.Depth) -> Int
/// C: `- (NSColor *)colorUsingDefaultColorSpace` (src/nsterm.h 360, src/nsterm.m's
/// `NSColor (EmacsColor)` category): a cross-file method, receiver first.
@_silgen_name("colorUsingDefaultColorSpace")
func colorUsingDefaultColorSpace(_ col: NSColor) -> NSColor
/// C: `Lisp_Object make_nil_vector (ptrdiff_t)` (src/lisp.h 4621),
/// `void *xcalloc (size_t, size_t)` (src/lisp.h 5539) and
/// `void free_monitors (struct MonitorInfo *, int)` / `Lisp_Object
/// make_monitor_attribute_list (struct MonitorInfo *, int, int, Lisp_Object, const char *)`
/// (src/frame.h 1937-1942).
@_silgen_name("make_nil_vector") func make_nil_vector(_ size: Int) -> Lisp_Object
@_silgen_name("xcalloc") func xcalloc(_ n: Int, _ size: Int) -> UnsafeMutableRawPointer?
@_silgen_name("xmalloc") func xmalloc(_ size: Int) -> UnsafeMutableRawPointer?
@_silgen_name("free_monitors")
func free_monitors(_ monitors: UnsafeMutablePointer<MonitorInfo>?, _ n_monitors: Int32)
@_silgen_name("make_monitor_attribute_list")
func make_monitor_attribute_list(_ monitors: UnsafeMutablePointer<MonitorInfo>?,
                                 _ n_monitors: Int32, _ primary_monitor: Int32,
                                 _ monitor_frames: Lisp_Object,
                                 _ source: UnsafePointer<CChar>?) -> Lisp_Object
/// C: `INLINE Lisp_Object AREF (Lisp_Object, ptrdiff_t)` / `INLINE void ASET (…)`
/// (src/lisp.h 1964, 1978).
@_silgen_name("AREF") func AREF(_ array: Lisp_Object, _ idx: Int) -> Lisp_Object
@_silgen_name("ASET") func ASET(_ array: Lisp_Object, _ idx: Int, _ val: Lisp_Object)
/// C: `Lisp_Object delete_frame (Lisp_Object, Lisp_Object)` (src/frame.h 1509) and
/// `void ns_make_frame_invisible (struct frame *)` (src/nsterm.h 1227, src/nsterm.m's).
@_silgen_name("delete_frame")
func delete_frame(_ frame: Lisp_Object, _ force: Lisp_Object) -> Lisp_Object
@_silgen_name("ns_make_frame_invisible")
func ns_make_frame_invisible(_ f: UnsafeMutableRawPointer?)
/// C: `Lisp_Object Fframe_parameter (Lisp_Object, Lisp_Object)` /
/// `Fmodify_frame_parameters` (subrs, globals.h).
@_silgen_name("Fframe_parameter")
func Fframe_parameter(_ frame: Lisp_Object, _ prop: Lisp_Object) -> Lisp_Object
@_silgen_name("Fmodify_frame_parameters")
func Fmodify_frame_parameters(_ frame: Lisp_Object, _ alist: Lisp_Object) -> Lisp_Object
/// C: the `struct frame` members the tip machinery writes (`f->tooltip`, `f->wants_modeline`).
@_silgen_name("frame_set_tooltip")
func frame_set_tooltip(_ f: UnsafeMutableRawPointer?, _ v: Bool)
/// C: `#define AUTO_FRAME_ARG(name, parameter, value)` (src/frame.h 1425-1426) with
/// `AUTO_CONS_EXPR`/`AUTO_LIST1` (src/lisp.h 5763-5773): `name = list1 (Fcons (parameter,
/// value))`.  C's `USE_STACK_CONS` fast path is a GC optimisation the core owns; the value is
/// the same.
@inline(__always)
fileprivate func AUTO_FRAME_ARG(_ parameter: Lisp_Object, _ value: Lisp_Object) -> Lisp_Object {
  return list1(Fcons(parameter, value))
}
/// C: `- (void)hide` and `- (BOOL)isActive` on `EmacsTooltip` (src/nsmenu.m's class, now
/// docs/ports/nsmenu.md's `EmacsTooltip`): cross-file methods, receiver first.
@_silgen_name("EmacsTooltip_hide") func EmacsTooltip_hide(_ tip: AnyObject?)
@_silgen_name("EmacsTooltip_isActive") func EmacsTooltip_isActive(_ tip: AnyObject?) -> Bool
/// The core's two tooltip-related variables: `extern bool use_system_tooltips`
/// (`DEFVAR_BOOL ("use-system-tooltips", …)`, src/frame.c 7727) and `extern bool face_change`
/// (src/xfaces.c 349).
@_silgen_name("use_system_tooltips") var use_system_tooltips: Bool
@_silgen_name("face_change") var face_change: Bool
/// More `Q…` symbols (generated globals.h, PORTING.md P-17; called as `Qfoo ()`).
@_silgen_name("Qbottom") func Qbottom() -> Lisp_Object
@_silgen_name("Qmouse_color") func Qmouse_color() -> Lisp_Object
@_silgen_name("Qborder_color") func Qborder_color() -> Lisp_Object
@_silgen_name("Qtooltip") func Qtooltip() -> Lisp_Object
@_silgen_name("Qdisplay_type") func Qdisplay_type() -> Lisp_Object
@_silgen_name("Qcolor") func Qcolor() -> Lisp_Object
@_silgen_name("Qface_set_after_frame_default") func Qface_set_after_frame_default() -> Lisp_Object
@_silgen_name("Qtip_frame") func Qtip_frame() -> Lisp_Object
@_silgen_name("Qcancel_timer") func Qcancel_timer() -> Lisp_Object
@_silgen_name("Qinhibit_redisplay") func Qinhibit_redisplay() -> Lisp_Object
@_silgen_name("Qinhibit_quit") func Qinhibit_quit() -> Lisp_Object

// ==========================================================================
// Lane ABI (batch 3): the constructs C 1886-2610 reads.
// ==========================================================================

/// C: `INLINE Lisp_Object list3i (intmax_t a, intmax_t b, intmax_t c)` (src/lisp.h 4526).
@_silgen_name("list3i")
func list3i(_ a: Int, _ b: Int, _ c: Int) -> Lisp_Object
/// C: `extern int ns_display_pixel_height (struct ns_display_info *)` /
/// `ns_display_pixel_width` (src/nsterm.h 1220-1221) - src/nsterm.m's.
@_silgen_name("ns_display_pixel_height")
func ns_display_pixel_height(_ dpyinfo: UnsafeMutablePointer<ns_display_info>?) -> Int32
@_silgen_name("ns_display_pixel_width")
func ns_display_pixel_width(_ dpyinfo: UnsafeMutablePointer<ns_display_info>?) -> Int32
/// C: `extern const char *ns_xlfd_to_fontname (const char *)` (src/nsterm.h 1129).
@_silgen_name("ns_xlfd_to_fontname")
func ns_xlfd_to_fontname(_ xlfd: UnsafePointer<CChar>) -> UnsafePointer<CChar>
/// C: `void nxatoms_of_nsselect (void)` (src/nsterm.h 1138, src/nsselect.m's) and
/// `struct ns_display_info *ns_term_init (Lisp_Object)` (src/nsterm.h 1108, src/nsterm.m's).
@_silgen_name("nxatoms_of_nsselect") func nxatoms_of_nsselect()
@_silgen_name("ns_term_init")
func ns_term_init(_ display_name: Lisp_Object) -> UnsafeMutablePointer<ns_display_info>?
/// C: `AVOID fatal (const char *, ...)` (src/lisp.h 4840); AVOID is _Noreturn (src/lisp.h 153).
@_silgen_name("fatal") func fatal(_ format: UnsafePointer<CChar>, _ args: CVarArg...) -> Never
/// C: `AVOID Fsignal (Lisp_Object, Lisp_Object)` (src/lisp.h 4786's declaration, globals.h's
/// `EXFUN`) and `AVOID report_file_error (const char *, Lisp_Object)` (src/lisp.h 4982).
@_silgen_name("Fsignal") func Fsignal(_ symbol: Lisp_Object, _ data: Lisp_Object) -> Never
@_silgen_name("report_file_error")
func report_file_error(_ str: UnsafePointer<CChar>, _ data: Lisp_Object) -> Never
/// C: `extern int c_strncasecmp (const char *, const char *, size_t)` (lib/c-strcase.h 52).
@_silgen_name("c_strncasecmp")
func c_strncasecmp(_ s1: UnsafePointer<CChar>?, _ s2: UnsafePointer<CChar>?, _ n: Int) -> Int32
/// C: `Lisp_Object Fexpand_file_name (Lisp_Object, Lisp_Object)` /
/// `Fdirectory_file_name` / `Fsymbol_value` / `Ffind_file_name_handler` /
/// `Ffile_symlink_p` / `Fx_load_color_file` (subrs, globals.h) and
/// `Lisp_Object intern (const char *)` (src/lisp.h 4757).
@_silgen_name("Fexpand_file_name")
func Fexpand_file_name(_ name: Lisp_Object, _ default_directory: Lisp_Object) -> Lisp_Object
@_silgen_name("Fdirectory_file_name") func Fdirectory_file_name(_ dir: Lisp_Object) -> Lisp_Object
@_silgen_name("Fsymbol_value") func Fsymbol_value(_ symbol: Lisp_Object) -> Lisp_Object
@_silgen_name("Ffind_file_name_handler")
func Ffind_file_name_handler(_ filename: Lisp_Object, _ operation: Lisp_Object) -> Lisp_Object
@_silgen_name("Ffile_symlink_p") func Ffile_symlink_p(_ file: Lisp_Object) -> Lisp_Object
@_silgen_name("Fx_load_color_file") func Fx_load_color_file(_ file: Lisp_Object) -> Lisp_Object
@_silgen_name("intern") func intern(_ str: UnsafePointer<CChar>) -> Lisp_Object
/// C: `Lisp_Object Ffuncall (ptrdiff_t, Lisp_Object *)` (globals.h 4824) - the target of
/// `#define calln(…) CALLN (Ffuncall, __VA_ARGS__)` (src/lisp.h 3462); the port spells `calln`
/// as the variadic-argument array, exactly as docs/ports/nsselect.swift does.
@_silgen_name("Ffuncall")
func Ffuncall(_ nargs: Int, _ args: UnsafeMutablePointer<Lisp_Object>) -> Lisp_Object
/// C: `void ns_string_to_pasteboard (id pb, Lisp_Object str)` /
/// `Lisp_Object ns_string_from_pasteboard (id pb)` (src/nsterm.h 1133-1134, src/nsselect.m's).
@_silgen_name("ns_string_to_pasteboard")
func ns_string_to_pasteboard(_ pb: AnyObject?, _ str: Lisp_Object)
@_silgen_name("ns_string_from_pasteboard")
func ns_string_from_pasteboard(_ pb: AnyObject?) -> Lisp_Object
/// C: `void ns_init_events (struct input_event *)` and `void ns_finish_events (void)`
/// (src/nsterm.m 353, 362) - src/nsterm.m's event-loop bracketing helpers.
@_silgen_name("ns_init_events") func ns_init_events(_ ev: UnsafeMutableRawPointer?)
@_silgen_name("ns_finish_events") func ns_finish_events()
/// C: `NSBestDepth (NSColorSpaceName, NSInteger, NSInteger, BOOL, BOOL *)` (the SDK's
/// NSGraphics.h 164).  Swift's AppKit interface does not vend it (AppKit.apinotes 3857), so the
/// port binds the C symbol directly - it is a system function, not the core's.
@_silgen_name("NSBestDepth")
func NSBestDepth(_ colorSpace: NSColorSpaceName, _ bps: Int, _ bpp: Int, _ planar: Bool,
                 _ exactMatch: UnsafeMutablePointer<ObjCBool>?) -> NSWindow.Depth
/// C: `#define FRAME_CONFIG_SCROLL_BAR_WIDTH(f)` / `_HEIGHT` / `_COLS` / `_LINES` (src/frame.h
/// 1286-1301) - read *and* written by the two `ns_set_scroll_bar_default_*` functions.
@_silgen_name("FRAME_CONFIG_SCROLL_BAR_WIDTH")
func FRAME_CONFIG_SCROLL_BAR_WIDTH(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_set_config_scroll_bar_width")
func frame_set_config_scroll_bar_width(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("FRAME_CONFIG_SCROLL_BAR_COLS")
func FRAME_CONFIG_SCROLL_BAR_COLS(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_set_config_scroll_bar_cols")
func frame_set_config_scroll_bar_cols(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("FRAME_CONFIG_SCROLL_BAR_HEIGHT")
func FRAME_CONFIG_SCROLL_BAR_HEIGHT(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_set_config_scroll_bar_height")
func frame_set_config_scroll_bar_height(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("FRAME_CONFIG_SCROLL_BAR_LINES")
func FRAME_CONFIG_SCROLL_BAR_LINES(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_set_config_scroll_bar_lines")
func frame_set_config_scroll_bar_lines(_ f: UnsafeMutableRawPointer?, _ v: Int32)
/// C: `#define CHECK_FRAME(x)` (src/frame.h 1387).
@_silgen_name("CHECK_FRAME") func CHECK_FRAME(_ x: Lisp_Object)
/// C: `extern bool inhibit_x_resources` (`DEFVAR_BOOL ("inhibit-x-resources", …)`,
/// src/emacs.c 3596).
@_silgen_name("inhibit_x_resources") var inhibit_x_resources: Bool
/// The port's spelling for the one call C protects with `@try/@catch` (src/nsfns.m 2290-2310):
/// Swift cannot catch an `NSException`, so the guarded `[cl writeToURL:error:]` is a C-ABI
/// entry (the ledger's `## ABI` (3)).  It returns C's `false` on failure and logs like the
/// `@catch` clause does.
@_silgen_name("ns_color_list_write_or_log")
func ns_color_list_write_or_log(_ cl: NSColorList?) -> Bool

// More `Q…` symbols (generated globals.h, PORTING.md P-17; called as `Qfoo ()`).
@_silgen_name("Qterminal") func Qterminal() -> Lisp_Object
@_silgen_name("Qname") func Qname() -> Lisp_Object
@_silgen_name("Qparent_id") func Qparent_id() -> Lisp_Object
@_silgen_name("Qminibuffer") func Qminibuffer() -> Lisp_Object
@_silgen_name("Qnone") func Qnone() -> Lisp_Object
@_silgen_name("Qonly") func Qonly() -> Lisp_Object
@_silgen_name("Qicon_name") func Qicon_name() -> Lisp_Object
@_silgen_name("Qfont_backend") func Qfont_backend() -> Lisp_Object
@_silgen_name("Qfontsize") func Qfontsize() -> Lisp_Object
@_silgen_name("Qfont") func Qfont() -> Lisp_Object
@_silgen_name("Qborder_width") func Qborder_width() -> Lisp_Object
@_silgen_name("Qright_divider_width") func Qright_divider_width() -> Lisp_Object
@_silgen_name("Qbottom_divider_width") func Qbottom_divider_width() -> Lisp_Object
@_silgen_name("Qvertical_scroll_bars") func Qvertical_scroll_bars() -> Lisp_Object
@_silgen_name("Qhorizontal_scroll_bars") func Qhorizontal_scroll_bars() -> Lisp_Object
@_silgen_name("Qline_spacing") func Qline_spacing() -> Lisp_Object
@_silgen_name("Qleft_fringe") func Qleft_fringe() -> Lisp_Object
@_silgen_name("Qright_fringe") func Qright_fringe() -> Lisp_Object
@_silgen_name("Qno_special_glyphs") func Qno_special_glyphs() -> Lisp_Object
@_silgen_name("Qmin_width") func Qmin_width() -> Lisp_Object
@_silgen_name("Qmin_height") func Qmin_height() -> Lisp_Object
@_silgen_name("Qx_create_frame_1") func Qx_create_frame_1() -> Lisp_Object
@_silgen_name("Qx_create_frame_2") func Qx_create_frame_2() -> Lisp_Object
@_silgen_name("Qundecorated") func Qundecorated() -> Lisp_Object
@_silgen_name("Qns_appearance") func Qns_appearance() -> Lisp_Object
@_silgen_name("Qns_transparent_titlebar") func Qns_transparent_titlebar() -> Lisp_Object
@_silgen_name("Qparent_frame") func Qparent_frame() -> Lisp_Object
@_silgen_name("Qz_group") func Qz_group() -> Lisp_Object
@_silgen_name("Qno_focus_on_map") func Qno_focus_on_map() -> Lisp_Object
@_silgen_name("Qno_accept_focus") func Qno_accept_focus() -> Lisp_Object
@_silgen_name("Qmenu_bar_lines") func Qmenu_bar_lines() -> Lisp_Object
@_silgen_name("Qbuffer_predicate") func Qbuffer_predicate() -> Lisp_Object
@_silgen_name("Qtitle") func Qtitle() -> Lisp_Object
@_silgen_name("Qinhibit_double_buffering") func Qinhibit_double_buffering() -> Lisp_Object
@_silgen_name("Qunsplittable") func Qunsplittable() -> Lisp_Object
@_silgen_name("Qauto_raise") func Qauto_raise() -> Lisp_Object
@_silgen_name("Qauto_lower") func Qauto_lower() -> Lisp_Object
@_silgen_name("Qcursor_type") func Qcursor_type() -> Lisp_Object
@_silgen_name("Qbox") func Qbox() -> Lisp_Object
@_silgen_name("Qscroll_bar_width") func Qscroll_bar_width() -> Lisp_Object
@_silgen_name("Qscroll_bar_height") func Qscroll_bar_height() -> Lisp_Object
@_silgen_name("Qalpha") func Qalpha() -> Lisp_Object
@_silgen_name("Qalpha_background") func Qalpha_background() -> Lisp_Object
@_silgen_name("Qborders_respect_alpha_background")
func Qborders_respect_alpha_background() -> Lisp_Object
@_silgen_name("Qvisibility") func Qvisibility() -> Lisp_Object
@_silgen_name("Qicon") func Qicon() -> Lisp_Object
@_silgen_name("Qright") func Qright() -> Lisp_Object
@_silgen_name("Qx_resource_name") func Qx_resource_name() -> Lisp_Object
@_silgen_name("Qframep") func Qframep() -> Lisp_Object

/// More of the core's variable storage (a `@_silgen_name` `var` imports the storage).
/// C: `DEFVAR_LISP ("x-resource-name", …)` / `("invocation-name", …)` (src/xfns.c, src/emacs.c);
/// `DEFVAR_LISP ("window-list", …)` (src/window.c) and the three mode flags
/// (src/xdisp.c 27333-27343).
@_silgen_name("Vx_resource_name") var Vx_resource_name: Lisp_Object
@_silgen_name("Vinvocation_name") var Vinvocation_name: Lisp_Object
@_silgen_name("Vwindow_list") var Vwindow_list: Lisp_Object
@_silgen_name("Vmenu_bar_mode") var Vmenu_bar_mode: Lisp_Object
@_silgen_name("Vtab_bar_mode") var Vtab_bar_mode: Lisp_Object
@_silgen_name("Vtool_bar_mode") var Vtool_bar_mode: Lisp_Object

// ==========================================================================
// C 86-84: the file-scope objects.
// ==========================================================================

/// C 61: `static EmacsTooltip *ns_tooltip = nil;` - `EmacsTooltip` is src/nsmenu.m's class, so
/// the port keeps the object alone rather than a Swift class of the same name (the
/// docs/ports/nsselect.swift `EmacsWindow` shape); the ledger's `## ABI` (3) records it.
fileprivate var ns_tooltip: AnyObject? = nil

/// C 64: `static Lisp_Object tip_frame;` - the frame of the currently visible tooltip.
fileprivate var tip_frame: Lisp_Object = nil
/// C 67: `static Lisp_Object tip_dx, tip_dy;` - the deltas of the last `x-show-tip` call.
fileprivate var tip_dx: Lisp_Object = nil
fileprivate var tip_dy: Lisp_Object = nil
/// C 71: `static Lisp_Object tip_timer;` - a timer that hides or deletes the tooltip.
fileprivate var tip_timer: Lisp_Object = nil
/// C 74: `static Lisp_Object tip_last_string;`
fileprivate var tip_last_string: Lisp_Object = nil
/// C 77: `static Lisp_Object tip_last_frame;`
fileprivate var tip_last_frame: Lisp_Object = nil
/// C 80: `static Lisp_Object tip_last_parms;`
fileprivate var tip_last_parms: Lisp_Object = nil
/// C 83: `static Lisp_Object as_script, *as_result;` and C 84: `static int as_status;`.
fileprivate var as_script: Lisp_Object = nil
fileprivate var as_result: UnsafeMutablePointer<Lisp_Object>? = nil
fileprivate var as_status: Int32 = 0

// ==========================================================================
// C 88-260: Internal utility functions.
// ==========================================================================

/// C 91-127.  `nil` stands for the selected frame - or, if that is not a Nextstep frame, the
/// first Nextstep display on the list.
func check_ns_display_info(_ object: Lisp_Object) -> UnsafeMutablePointer<ns_display_info>? {
  var dpyinfo: UnsafeMutablePointer<ns_display_info>? = nil

  if NILP(object) {
    let sf = XFRAME(selected_frame)
    if FRAME_NS_P(sf) && FRAME_LIVE_P(sf) {
      dpyinfo = FRAME_DISPLAY_INFO(sf)
    } else if x_display_list != nil {
      dpyinfo = x_display_list
    } else {
      error("Nextstep windows are not in use or not initialized")
    }
  } else if TERMINALP(object) {
    let t = decode_live_terminal(object)
    if terminal_type(t) != output_ns() {
      error("Terminal %d is not a Nextstep display", terminal_id(t))
    }
    dpyinfo = terminal_display_info_ns(t)
  } else if STRINGP(object) {
    dpyinfo = ns_display_info_for_name(object)
  } else {
    let f = decode_window_system_frame(object)
    dpyinfo = FRAME_DISPLAY_INFO(f)
  }

  return dpyinfo
}

/// C 134-149.
func ns_get_window(_ maybeFrame: Lisp_Object) -> AnyObject? {
  var maybeFrame = maybeFrame
  var view: NSView? = nil
  var window: NSWindow? = nil

  if !FRAMEP(maybeFrame) || !FRAME_NS_P(XFRAME(maybeFrame)) {
    maybeFrame = selected_frame;  /* wrong_type_argument (Qframep, maybeFrame); */
  }

  if !NILP(maybeFrame) {
    view = FRAME_NS_VIEW(XFRAME(maybeFrame))
  }
  if view != nil { window = view?.window }

  return window
}

/// C 153-172.  Return the display structure for the display named NAME.
func ns_display_info_for_name(_ name: Lisp_Object) -> UnsafeMutablePointer<ns_display_info>? {
  var dpyinfo: UnsafeMutablePointer<ns_display_info>? = nil

  CHECK_STRING(name)

  dpyinfo = x_display_list
  while dpyinfo != nil {
    if !NILP(Fstring_equal(XCAR(dpyinfo!.pointee.name_list_element), name)) {
      return dpyinfo
    }
    dpyinfo = dpyinfo!.pointee.next
  }

  error("Emacs for Nextstep does not yet support multi-display")

  /* C 168-171 is unreachable in C too (`error` is `_Noreturn`, src/lisp.h 153): the reference
     keeps it compiled, so it is recorded rather than dropped:

       Fx_open_connection (name, Qnil, Qnil);
       dpyinfo = x_display_list;
       if (dpyinfo == 0)
         error ("Display on %s not responding.\n", SDATA (name));  */
}

/// C 175-186: the `NS_IMPL_COCOA` arm (the `#else` arm is GNUstep's `[panel filename]`).
func ns_filename_from_panel(_ panel: NSSavePanel) -> NSString? {
  let url = panel.url
  let str = url?.path
  return str as NSString?
}

/// C 188-197: the `NS_IMPL_COCOA` arm (GNUstep's is `[panel directory]`).
func ns_directory_from_panel(_ panel: NSSavePanel) -> NSString? {
  let url = panel.directoryURL
  let str = url?.path
  return str as NSString?
}

/* C 200-258: `#ifndef NS_IMPL_COCOA` - `interpret_services_menu` is the GNUstep arm only, so
   the port records it instead of carrying it (the ledger's Decisions; a `swift-units.tsv` row
   names it so the gate can tell the two apart):

     static Lisp_Object
     interpret_services_menu (NSMenu *menu, Lisp_Object prefix, Lisp_Object old)
     { … count = [menu numberOfItems]; … }  */

// ==========================================================================
// C 260-1000: Frame parameter setters.
// ==========================================================================

/// C 259-290.
@_cdecl("ns_set_foreground_color")
func ns_set_foreground_color(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                             _ oldval: Lisp_Object) {
  var col: NSColor? = nil

  /* Must block_input, because ns_lisp_to_color does block/unblock_input
     which means that col may be deallocated in its unblock_input if there
     is user input, unless we also block_input.  */
  block_input()
  if ns_lisp_to_color(arg, &col) != 0 {
    store_frame_param(f, Qforeground_color(), oldval)
    unblock_input()
    error("Unknown color")
  }

  ns_output_set_foreground_color(f, col)

  frame_set_foreground_pixel(f, unsignedLong(col!))

  if FRAME_NS_VIEW(f) != nil {
    update_face_from_frame_parameter(f, Qforeground_color(), arg)
    /* recompute_basic_faces (f); */
    if FRAME_VISIBLE_P(f) {
      SET_FRAME_GARBAGED(f)
    }
  }
  unblock_input()
}

/// C 292-341.
@_cdecl("ns_set_background_color")
func ns_set_background_color(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                             _ oldval: Lisp_Object) {
  var face: UnsafeMutableRawPointer?
  var col: NSColor? = nil
  let view = FRAME_NS_VIEW(f)
  var alpha: EmacsCGFloat

  block_input()
  if ns_lisp_to_color(arg, &col) != 0 {
    store_frame_param(f, Qbackground_color(), oldval)
    unblock_input()
    error("Unknown color")
  }

  ns_output_set_background_color(f, col)

  frame_set_background_pixel(f, unsignedLong(col!))
  alpha = col!.alphaComponent

  if view != nil {
    view?.window?.backgroundColor = col

    if alpha != EmacsCGFloat(1.0) {
      view?.window?.isOpaque = false
    } else {
      view?.window?.isOpaque = true
    }

    face = FRAME_DEFAULT_FACE(f)
    if face != nil {
      col = NSColor_colorWithUnsignedLong(NS_FACE_BACKGROUND(face!))
      /* C 324-327 assigns `face->background`; the record is the core's, so the write is a
         C-ABI entry (see the ledger's `## ABI` (3)).  */
      face_set_background(f, unsignedLong(col!.withAlphaComponent(alpha)))

      update_face_from_frame_parameter(f, Qbackground_color(), arg)
    }

    if FRAME_VISIBLE_P(f) {
      SET_FRAME_GARBAGED(f)
      ns_clear_frame(f)
    }
  }
  unblock_input()
}

/// C 344-368.
@_cdecl("ns_set_cursor_color")
func ns_set_cursor_color(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                         _ oldval: Lisp_Object) {
  var col: NSColor? = nil

  block_input()
  if ns_lisp_to_color(arg, &col) != 0 {
    store_frame_param(f, Qcursor_color(), oldval)
    unblock_input()
    error("Unknown color")
  }

  ns_output_set_cursor_color(f, col)

  if FRAME_VISIBLE_P(f) {
    gui_update_cursor(f, false)
    gui_update_cursor(f, true)
  }
  update_face_from_frame_parameter(f, Qcursor_color(), arg)
  unblock_input()
}

/// C 370-412.
@_cdecl("ns_set_icon_name")
func ns_set_icon_name(_ f: UnsafeMutableRawPointer?, _ argIn: Lisp_Object,
                      _ oldval: Lisp_Object) {
  var arg = argIn
  let view = FRAME_NS_VIEW(f)
  /* C 372: `NSTRACE ("ns_set_icon_name");` - the reference's no-op trace.  */

  /* See if it's changed.  */
  if STRINGP(arg) {
    if STRINGP(oldval) && BASE_EQ(Fstring_equal(oldval, arg), Qt()) {
      return
    }
  } else if !STRINGP(oldval) && NILP(oldval) == NILP(arg) {
    return
  }

  fset_icon_name(f, arg)

  if NILP(arg) {
    if !NILP(frame_title(f)) {
      arg = frame_title(f)
    } else if frame_explicit_name(f) {
      /* Explicit name and no icon-name -> explicit_name.  */
      arg = frame_name(f)
    } else {
      /* No explicit name and no icon-name ->
         name has to be rebuild from icon_title_format.  */
      windows_or_buffers_changed = 62
      return
    }
  }

  /* Don't change the name if it's already NAME.  */
  if view?.window?.miniwindowTitle != nil
      && (view?.window?.miniwindowTitle == stringWithLispString(arg) as String?) {
    return
  }

  view?.window?.miniwindowTitle = stringWithLispString(arg) as String?
}

/// C 414-431.
func ns_set_name_internal(_ f: UnsafeMutableRawPointer?, _ name: Lisp_Object) {
  let view = FRAME_NS_VIEW(f)
  var str = stringWithLispString(name)

  /* Don't change the name if it's already NAME.  */
  if view?.window?.title != str as String? {
    view?.window?.title = str as String? ?? ""
  }

  if STRINGP(frame_icon_name(f)) {
    str = stringWithLispString(frame_icon_name(f))
  }

  if view?.window?.miniwindowTitle != nil
      && view?.window?.miniwindowTitle != str as String? {
    view?.window?.miniwindowTitle = str as String? ?? ""
  }
}

/// C 433-468.
func ns_set_name(_ f: UnsafeMutableRawPointer?, _ nameIn: Lisp_Object, _ explicit: Int32) {
  var name = nameIn
  /* C 435: `NSTRACE ("ns_set_name");` */

  /* Make sure that requests from lisp code override requests from
     Emacs redisplay code.  */
  if explicit != 0 {
    /* If we're switching from explicit to implicit, we had better
       update the mode lines and thereby update the title.  */
    if frame_explicit_name(f) && NILP(name) {
      update_mode_lines = 21
    }

    frame_set_explicit_name(f, !NILP(name))
  } else if frame_explicit_name(f) {
    return
  }

  if NILP(name) {
    name = lispString(ns_app_name!)
  } else {
    CHECK_STRING(name)
  }

  /* Don't change the name if it's already NAME.  */
  if !NILP(Fstring_equal(name, frame_name(f))) {
    return
  }

  fset_name(f, name)

  /* Title overrides explicit name.  */
  if !NILP(frame_title(f)) {
    name = frame_title(f)
  }

  ns_set_name_internal(f, name)
}

/// C 470-522.
func ns_set_represented_filename(_ f: UnsafeMutableRawPointer?) {
  var filename: Lisp_Object
  var fstr: NSString?
  let view = FRAME_NS_VIEW(f)

  /* C 478: `NSTRACE ("ns_set_represented_filename");` */

  if frame_explicit_name(f) || !NILP(frame_title(f)) {
    return
  }

  block_input()
  let buf = window_contents(XWINDOW(frame_selected_window(f)!))
  filename = buffer_filename(XBUFFER(buf))

  if !NILP(filename) {
    fstr = stringWithLispString(filename)
    if fstr == nil { fstr = "" }
  } else {
    fstr = ""
  }

  /* Work around for Mach port leaks on macOS 10.15 (bug#38618).  */
  let fileURL = URL(fileURLWithPath: fstr! as String, isDirectory: false)
  var isUbiquitousItem: AnyObject? = NSNumber(value: true)
  /* C 502-506: `[fileURL getResourceValue:(id *)&isUbiquitousItem
     forKey:NSURLIsUbiquitousItemKey error:nil];` - the one place this file asks a URL for a
     resource value; the call is kept as a call on the URL.  */
  _ = try? (fileURL as NSURL).getResourceValue(&isUbiquitousItem,
                                               forKey: URLResourceKey.isUbiquitousItemKey)
  if (isUbiquitousItem as? NSNumber)?.boolValue == true {
    fstr = ""
  }

  /* Work around a bug observed on 10.3 and later where
     setTitleWithRepresentedFilename does not clear out previous state
     if given filename does not exist.  */
  if !FileManager.default.fileExists(atPath: fstr! as String) {
    view?.window?.representedFilename = ""
  }
  view?.window?.representedFilename = fstr! as String

  unblock_input()
}

/// C 524-534.  Called when the user's lisp code has specified a name for the frame.
@_cdecl("ns_explicitly_set_name")
func ns_explicitly_set_name(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                            _ oldval: Lisp_Object) {
  /* C 527: `NSTRACE ("ns_explicitly_set_name");` */
  ns_set_name(f, arg, 1)
}

/// C 536-549.  Called by the reference's redisplay code; names set this way never
/// override the user's.
@_cdecl("ns_implicitly_set_name")
func ns_implicitly_set_name(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                            _ oldval: Lisp_Object) {
  /* C 538: `NSTRACE ("ns_implicitly_set_name");` */

  if ns_use_proxy_icon {
    ns_set_represented_filename(f)
  }

  ns_set_name(f, arg, 0)
}

/// C 551-569.
@_cdecl("ns_set_title")
func ns_set_title(_ f: UnsafeMutableRawPointer?, _ nameIn: Lisp_Object,
                  _ old_name: Lisp_Object) {
  var name = nameIn
  /* C 553: `NSTRACE ("ns_set_title");` */
  /* Don't change the title if it's already NAME.  */
  if EQ(name, frame_title(f)) {
    return
  }

  update_mode_lines = 22

  fset_title(f, name)

  if NILP(name) {
    name = frame_name(f)
  } else {
    CHECK_STRING(name)
  }

  ns_set_name_internal(f, name)
}

/// C 571-597.
@_cdecl("ns_set_doc_edited")
func ns_set_doc_edited() {
  block_input()
  /* C 579: `FOR_EACH_FRAME (tail, frame)` (src/frame.h 1404-1408) - the macro's own spelling:
     walk `Vframe_list` while it is a cons.  */
  var tail: Lisp_Object = Vframe_list
  while CONSP(tail) {
    let frame = XCAR(tail)
    var edited = false
    let f = XFRAME(frame)
    var w: UnsafeMutableRawPointer?
    var view: NSView?

    if !FRAME_NS_P(f) { tail = XCDR(tail); continue }
    w = XWINDOW(FRAME_SELECTED_WINDOW(f))
    view = FRAME_NS_VIEW(f)
    if !MINI_WINDOW_P(w) {
      edited = !NILP(Fbuffer_modified_p(window_contents(w)))
        && !NILP(Fbuffer_file_name(window_contents(w)))
    }
    view?.window?.isDocumentEdited = edited
    tail = XCDR(tail)
  }

  unblock_input()
}

/// C 599-625.
@_cdecl("ns_set_menu_bar_lines")
func ns_set_menu_bar_lines(_ f: UnsafeMutableRawPointer?, _ value: Lisp_Object,
                           _ oldval: Lisp_Object) {
  var nlines: Int32
  if FRAME_MINIBUF_ONLY_P(f) {
    return
  }

  if TYPE_RANGED_FIXNUMP(value) {
    nlines = Int32(XFIXNUM(value))
  } else {
    nlines = 0
  }

  frame_set_menu_bar_lines(f, 0)
  if nlines != 0 {
    frame_set_external_menu_bar(f, 1)
    /* Does for all frames, whereas we just want for one frame
       [NSMenu setMenuBarVisible: YES]; */
  } else {
    if FRAME_EXTERNAL_MENU_BAR(f) == 1 {
      free_frame_menubar(f)
    }
    /* [NSMenu setMenuBarVisible: NO]; */
    frame_set_external_menu_bar(f, 0)
  }
}

/// C 627-688.
@_cdecl("ns_change_tab_bar_height")
func ns_change_tab_bar_height(_ f: UnsafeMutableRawPointer?, _ height: Int32) {
  let unit = FRAME_LINE_HEIGHT(f)
  let old_height = FRAME_TAB_BAR_HEIGHT(f)

  /* This differs from the tool bar code in that the tab bar height is
     not rounded up.  Otherwise, if redisplay_tab_bar decides to grow
     the tab bar by even 1 pixel, FRAME_TAB_BAR_LINES will be changed,
     leading to the tab bar height being incorrectly set upon the next
     call to x_set_font.  (bug#59285) */
  var lines = height / unit

  /* Even so, HEIGHT might be less than unit if the tab bar face is
     not so tall as the frame's font height; which if true lines will
     be set to 0 and the tab bar will thus vanish.  */

  if lines == 0 && height != 0 {
    lines = 1
  }

  /* Make sure we redisplay all windows in this frame.  */
  fset_redisplay(f)

  /* Recalculate tab bar and frame text sizes.  */
  frame_set_tab_bar_height(f, height)
  frame_set_tab_bar_lines(f, lines)
  store_frame_param(f, Qtab_bar_lines(), make_fixnum(Int(lines)))

  if FRAME_NS_WINDOW(f) != 0 && FRAME_TAB_BAR_HEIGHT(f) == 0 {
    clear_frame(f)
    clear_current_matrices(f)
  }

  if height < old_height && WINDOWP(frame_tab_bar_window(f)) {
    clear_glyph_matrix(window_current_matrix(XWINDOW(frame_tab_bar_window(f))))
  }

  if !frame_tab_bar_resized(f) {
    let fullscreen = get_frame_param(f, Qfullscreen())

    /* As long as tab_bar_resized is false, effectively try to change
       F's native height.  */
    if NILP(fullscreen) || EQ(fullscreen, Qfullwidth()) {
      adjust_frame_size(f, FRAME_TEXT_WIDTH(f), FRAME_TEXT_HEIGHT(f), 1, false,
                        Qtab_bar_lines())
    } else {
      adjust_frame_size(f, -1, -1, 4, false, Qtab_bar_lines())
    }

    frame_set_tab_bar_resized(f, frame_tab_bar_redisplayed(f))
  } else {
    /* Any other change may leave the native size of F alone.  */
    adjust_frame_size(f, -1, -1, 3, false, Qtab_bar_lines())
  }

  /* adjust_frame_size might not have done anything, garbage frame
     here.  */
  adjust_frame_glyphs(f)
  SET_FRAME_GARBAGED(f)
}

/// C 690-697 (the `NS_IMPL_COCOA` arm).
@_cdecl("ns_make_frame_key_window")
func ns_make_frame_key_window(_ f: UnsafeMutableRawPointer?) {
  FRAME_NS_VIEW(f)?.window?.makeKey()
}

/// C 699-721.
@_cdecl("ns_set_tab_bar_lines")
func ns_set_tab_bar_lines(_ f: UnsafeMutableRawPointer?, _ value: Lisp_Object,
                          _ oldval: Lisp_Object) {
  let olines = FRAME_TAB_BAR_LINES(f)
  var nlines: Int32

  /* Treat tab bars like menu bars.  */
  if FRAME_MINIBUF_ONLY_P(f) {
    return
  }

  /* Use VALUE only if an int >= 0.  */
  if RANGED_FIXNUMP(0, value, Int(Int32.max)) {
    nlines = Int32(XFIXNAT(value))
  } else {
    nlines = 0
  }

  if nlines != olines && (olines == 0 || nlines == 0) {
    ns_change_tab_bar_height(f, nlines * FRAME_LINE_HEIGHT(f))
  }
}

/// C 723-772.
@_cdecl("ns_set_tool_bar_lines")
func ns_set_tool_bar_lines(_ f: UnsafeMutableRawPointer?, _ value: Lisp_Object,
                           _ oldval: Lisp_Object) {
  /* Currently, when the tool bar changes state, the frame is resized.

     TODO: It would be better if this didn't occur when 1) the frame
     is full height or maximized or 2) when specified by
     `frame-inhibit-implied-resize'.  */
  var nlines: Int32

  /* C 733: `NSTRACE ("ns_set_tool_bar_lines");` */

  if FRAME_MINIBUF_ONLY_P(f) {
    return
  }

  if RANGED_FIXNUMP(0, value, Int(Int32.max)) {
    nlines = Int32(XFIXNAT(value))
  } else {
    nlines = 0
  }

  if nlines != 0 {
    frame_set_external_tool_bar(f, 1)
    update_frame_tool_bar(f)
  } else {
    if FRAME_EXTERNAL_TOOL_BAR(f) != 0 {
      free_frame_tool_bar(f)
      frame_set_external_tool_bar(f, 0)

      let view = FRAME_NS_VIEW(f)
      let fs_state = view.map { fullscreenState($0) } ?? 0

      if fs_state == FULLSCREEN_MAXIMIZED() {
        setFSValue(view, FULLSCREEN_WIDTH())
      } else if fs_state == FULLSCREEN_HEIGHT() {
        setFSValue(view, FULLSCREEN_NONE())
      }
    }
  }

  adjust_frame_size(f, -1, -1, 2, false, Qtool_bar_lines())
}

/// C 774-780.
@_cdecl("ns_set_tool_bar_position")
func ns_set_tool_bar_position(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                              _ oldval: Lisp_Object) {
  if !EQ(arg, Qtop()) {
    error("Tool bar position must be `top'")
  }
}

/// C 782-803.
@_cdecl("ns_set_child_frame_border_width")
func ns_set_child_frame_border_width(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                                     _ oldval: Lisp_Object) {
  var border: Int32

  if NILP(arg) {
    border = -1
  } else if RANGED_FIXNUMP(0, arg, Int(Int32.max)) {
    border = Int32(XFIXNAT(arg))
  } else {
    signal_error("Invalid child frame border width", arg)
  }

  if border != FRAME_CHILD_FRAME_BORDER_WIDTH(f) {
    frame_set_child_frame_border_width(f, border)

    if FRAME_NATIVE_WINDOW(f) != 0 {
      adjust_frame_size(f, -1, -1, 3, false, Qchild_frame_border_width())
    }

    SET_FRAME_GARBAGED(f)
  }
}

/// C 805-824 (the `NS_IMPL_COCOA` + macOS 10.14 arm; the whole body is inside the reference's
/// own `#if`, which this target takes).
@_cdecl("ns_set_inhibit_double_buffering")
func ns_set_inhibit_double_buffering(_ f: UnsafeMutableRawPointer?, _ new_value: Lisp_Object,
                                     _ old_value: Lisp_Object) {
  if !EQ(new_value, old_value) {
    frame_set_double_buffered(f, NILP(new_value))

    /* If the view or layer haven't been created yet this will be a
       noop.  */
    setDoubleBuffered(FRAME_NS_VIEW(f)?.layer, FRAME_DOUBLE_BUFFERED(f))

    SET_FRAME_GARBAGED(f)
  }
}

/// C 826-840.
@_cdecl("ns_set_internal_border_width")
func ns_set_internal_border_width(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                                  _ oldval: Lisp_Object) {
  let old_width = FRAME_INTERNAL_BORDER_WIDTH(f)
  let new_width = check_int_nonnegative(arg)

  if new_width == old_width {
    return
  }
  frame_set_internal_border_width(f, new_width)

  if FRAME_NATIVE_WINDOW(f) != 0 {
    adjust_frame_size(f, -1, -1, 3, false, Qinternal_border_width())
  }

  SET_FRAME_GARBAGED(f)
}

/// C 842-927.
func ns_implicitly_set_icon_type(_ f: UnsafeMutableRawPointer?) {
  var tem: Lisp_Object
  let view = FRAME_NS_VIEW(f)
  var image: AnyObject? = nil
  var chain: Lisp_Object
  var elt: Lisp_Object
  var setMini = true
  let workspace = NSWorkspace.shared

  /* C 850: `NSTRACE ("ns_implicitly_set_icon_type");` */

  block_input()
  let mini = ns_output_miniimage(f) as? NSImage
  if mini != nil && (mini!.name() as String?) == (stringWithLispString(frame_name(f)) as String?) {
    unblock_input()
    return
  }

  tem = assq_no_quit(Qicon_type(), frame_param_alist(f))
  if CONSP(tem) && !NILP(XCDR(tem)) {
    unblock_input()
    return
  }

  chain = Vns_icon_type_alist
  while image == nil && CONSP(chain) {
    elt = XCAR(chain)
    /* Special case: t means go by file type.  */
    if SYMBOLP(elt) && EQ(elt, Qt()) && SSDATA(frame_name(f))[0] == UInt8(ascii: "/") {
      let str = stringWithLispString(frame_name(f))
      if FileManager.default.fileExists(atPath: str! as String) {
        image = NSWorkspace.shared.icon(forFile: str! as String)
      }
    } else if CONSP(elt) && STRINGP(XCAR(elt)) && STRINGP(XCDR(elt))
                && fast_string_match(XCAR(elt), frame_name(f)) >= 0 {
      image = EmacsImage_allocInitFromFile(XCDR(elt))
      if image == nil {
        image = NSImage(named: stringWithLispString(XCDR(elt))! as String)
      }
    }
    chain = XCDR(chain)
  }

  if image == nil {
    image = workspace.icon(for: .text) as AnyObject
    setMini = false
  }

  ns_output_set_miniimage(f, image)
  setMiniwindowImage(view, setMini)
  unblock_input()
}

/// C 929-966.
@_cdecl("ns_set_icon_type")
func ns_set_icon_type(_ f: UnsafeMutableRawPointer?, _ argIn: Lisp_Object,
                      _ oldval: Lisp_Object) {
  var arg = argIn
  let view = FRAME_NS_VIEW(f)
  var image: AnyObject? = nil
  var setMini = true

  /* C 935: `NSTRACE ("ns_set_icon_type");` */

  if !NILP(arg) && SYMBOLP(arg) {
    arg = build_string(SSDATA(SYMBOL_NAME(arg)))
    store_frame_param(f, Qicon_type(), arg)
  }

  /* Do it the implicit way.  */
  if NILP(arg) {
    ns_implicitly_set_icon_type(f)
    return
  }

  CHECK_STRING(arg)

  image = EmacsImage_allocInitFromFile(arg)
  if image == nil {
    image = NSImage(named: stringWithLispString(arg)! as String)
  }

  if image == nil {
    image = NSImage(named: "text")
    setMini = false
  }

  ns_output_set_miniimage(f, image)
  setMiniwindowImage(view, setMini)
}

/// C 968-973: the same as the xfns.c definition.
@_cdecl("ns_set_cursor_type")
func ns_set_cursor_type(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                        _ oldval: Lisp_Object) {
  set_frame_cursor_types(f, arg)
}

/// C 975-983: called to set mouse pointer color, but all other terms use it to initialize
/// pointer types (and don't set the color ;) - "Don't think we can do this on Nextstep."
@_cdecl("ns_set_mouse_color")
func ns_set_mouse_color(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                        _ oldval: Lisp_Object) {
}

/* C 982-983: `#define Str(x) #x` / `#define Xstr(x) Str(x)` - the token-pasting pair the
   GNUstep arm of `ns_appkit_version_str` uses (`Xstr (GNUSTEP_GUI_VERSION)`); the Cocoa arm
   below does not, so they are dead text here.  */

/// C 985-1011, the `NS_IMPL_COCOA` arm.
func ns_appkit_version_str() -> Lisp_Object {
  var tmp: NSString?
  var string: Lisp_Object

  tmp = NSString(format: "appkit-%.2f %@", NSAppKitVersion.current.rawValue,
                 ProcessInfo.processInfo.operatingSystemVersionString) as NSString
  string = build_string(tmp!.utf8String!)
  return string
}

/// C 1013-1022: the GNUstep arm collapses `GNUSTEP_GUI_MAJOR_VERSION`/`_MINOR_VERSION`; the
/// Cocoa arm is `(int) NSAppKitVersionNumber`.
func ns_appkit_version_int() -> Int32 {
  return Int32(NSAppKitVersion.current.rawValue)
}

/// C 1024-1048.  Strangely-named function to set icon position parameters in frame; this is
/// not a standard frame parameter-setter - it is called directly from `x-create-frame`.
func ns_icon(_ f: UnsafeMutableRawPointer?, _ parms: Lisp_Object) {
  var icon_x: Lisp_Object
  var icon_y: Lisp_Object
  let dpyinfo = check_ns_display_info(Qnil())

  ns_output_set_icon_top(f, -1)
  ns_output_set_icon_left(f, -1)

  /* Set the position of the icon.  */
  icon_x = gui_display_get_arg(dpyinfo, parms, Qicon_left(), nil, nil, RES_TYPE_NUMBER())
  icon_y = gui_display_get_arg(dpyinfo, parms, Qicon_top(), nil, nil, RES_TYPE_NUMBER())
  if !EQ(icon_x, Qunbound()) && !EQ(icon_y, Qunbound()) {
    CHECK_FIXNUM(icon_x)
    CHECK_FIXNUM(icon_y)
    ns_output_set_icon_top(f, Int32(XFIXNUM(icon_y)))
    ns_output_set_icon_left(f, Int32(XFIXNUM(icon_x)))
  } else if !EQ(icon_x, Qunbound()) || !EQ(icon_y, Qunbound()) {
    error("Both left and top icon corners of icon must be specified")
  }
}

/// C 1058-1120: `frame_parm_handler ns_frame_parm_handlers[]` - the table src/frame.c reads by
/// index.  Swift cannot emit a C-named *data* symbol, so the port carries the array (the same
/// entries, in the same order, with the same explicit nulls) and the ledger's `## ABI` (3)
/// records that the C symbol itself needs a port-side definition.
let ns_frame_parm_handlers: [frame_parm_handler?] = [
  { f, a, b in gui_set_autoraise(f, a, b) },     /* generic OK */
  { f, a, b in gui_set_autolower(f, a, b) },     /* generic OK */
  { f, a, b in ns_set_background_color(f, a, b) },
  nil,  /* x_set_border_color,  may be impossible under Nextstep */
  nil,  /* x_set_border_width,  may be impossible under Nextstep */
  { f, a, b in ns_set_cursor_color(f, a, b) },
  { f, a, b in ns_set_cursor_type(f, a, b) },
  { f, a, b in gui_set_font(f, a, b) },          /* generic OK */
  { f, a, b in ns_set_foreground_color(f, a, b) },
  { f, a, b in ns_set_icon_name(f, a, b) },
  { f, a, b in ns_set_icon_type(f, a, b) },
  { f, a, b in ns_set_child_frame_border_width(f, a, b) },
  { f, a, b in ns_set_internal_border_width(f, a, b) },
  { f, a, b in gui_set_right_divider_width(f, a, b) },   /* generic OK */
  { f, a, b in gui_set_bottom_divider_width(f, a, b) },  /* generic OK */
  { f, a, b in ns_set_menu_bar_lines(f, a, b) },
  { f, a, b in ns_set_mouse_color(f, a, b) },
  { f, a, b in ns_explicitly_set_name(f, a, b) },
  { f, a, b in gui_set_scroll_bar_width(f, a, b) },  /* generic OK */
  { f, a, b in gui_set_scroll_bar_height(f, a, b) }, /* generic OK */
  { f, a, b in ns_set_title(f, a, b) },
  { f, a, b in gui_set_unsplittable(f, a, b) },      /* generic OK */
  { f, a, b in gui_set_vertical_scroll_bars(f, a, b) },   /* generic OK */
  { f, a, b in gui_set_horizontal_scroll_bars(f, a, b) }, /* generic OK */
  { f, a, b in gui_set_visibility(f, a, b) },        /* generic OK */
  { f, a, b in ns_set_tab_bar_lines(f, a, b) },
  { f, a, b in ns_set_tool_bar_lines(f, a, b) },
  nil,  /* x_set_scroll_bar_foreground, will ignore (not possible on NS) */
  nil,  /* x_set_scroll_bar_background,  will ignore (not possible on NS) */
  { f, a, b in gui_set_screen_gamma(f, a, b) },      /* generic OK */
  { f, a, b in gui_set_line_spacing(f, a, b) },      /* generic OK, sets f->extra_line_spacing to int */
  { f, a, b in gui_set_left_fringe(f, a, b) },       /* generic OK */
  { f, a, b in gui_set_right_fringe(f, a, b) },      /* generic OK */
  nil,  /* x_set_wait_for_wm, will ignore */
  { f, a, b in gui_set_fullscreen(f, a, b) },        /* generic OK */
  { f, a, b in gui_set_font_backend(f, a, b) },      /* generic OK */
  { f, a, b in gui_set_alpha(f, a, b) },
  nil,  /* x_set_sticky */
  { f, a, b in ns_set_tool_bar_position(f, a, b) },
  { f, a, b in ns_set_inhibit_double_buffering(f, a, b) },
  { f, a, b in ns_set_undecorated(f, a, b) },
  { f, a, b in ns_set_parent_frame(f, a, b) },
  nil,  /* x_set_skip_taskbar */
  { f, a, b in ns_set_no_focus_on_map(f, a, b) },
  { f, a, b in ns_set_no_accept_focus(f, a, b) },
  { f, a, b in ns_set_z_group(f, a, b) },
  nil,  /* x_set_override_redirect */
  { f, a, b in gui_set_no_special_glyphs(f, a, b) },
  { f, a, b in gui_set_alpha_background(f, a, b) },
  { f, a, b in gui_set_borders_respect_alpha_background(f, a, b) },
  nil,
  /* C 1111-1114, the `NS_IMPL_COCOA` tail of the initialiser.  */
  { f, a, b in ns_set_appearance(f, a, b) },
  { f, a, b in ns_set_transparent_titlebar(f, a, b) },
]

/// C 1122-1140.  Handler for signals raised during `x_create_frame`.
func unwind_create_frame(_ frame: Lisp_Object) -> Lisp_Object {
  let f = XFRAME(frame)

  /* If frame is already dead, nothing to do.  This can happen if the
     display is disconnected after the frame has become official, but
     before x_create_frame removes the unwind protect.  */
  if !FRAME_LIVE_P(f) {
    return Qnil()
  }

  /* If frame is ``official'', nothing to do.  */
  if NILP(Fmemq(frame, Vframe_list)) {
    ns_free_frame_resources(f)
    free_glyphs(f)
    return Qt()
  }

  return Qnil()
}

/// C 1144-1147.
func do_unwind_create_frame(_ frame: Lisp_Object) {
  _ = unwind_create_frame(frame)
}

/// C 1155-1186: read geometry related parameters from preferences if not in PARMS; returns the
/// union of parms and any preferences read.
func get_geometry_from_preferences(_ dpyinfo: UnsafeMutablePointer<ns_display_info>?,
                                   _ parmsIn: Lisp_Object) -> Lisp_Object {
  var parms = parmsIn
  /* C 1158-1163's local record: `const char *val, *cls; Lisp_Object tem;`.  C's `const char *`
     fields are string literals, so the port keeps them as Swift strings and passes their bytes
     for the duration of the call.  */
  let r: [(val: String, cls: String, tem: Lisp_Object)] = [
    ("width", "Width", Qwidth()),
    ("height", "Height", Qheight()),
    ("left", "Left", Qleft()),
    ("top", "Top", Qtop()),
  ]

  for i in 0 ..< r.count {
    if NILP(Fassq(r[i].tem, parms)) {
      let value = r[i].val.withCString { valp in
        r[i].cls.withCString { clsp in
          gui_display_get_arg(dpyinfo, parms, r[i].tem, valp, clsp, RES_TYPE_NUMBER())
        }
      }
      if !EQ(value, Qunbound()) {
        parms = Fcons(Fcons(r[i].tem, value), parms)
      }
    }
  }

  return parms
}

// ==========================================================================
// C 1188-1610: `x-create-frame` (Fx_create_frame) - the file's biggest function.
// ==========================================================================

/// C: `DEFUN`'s file-static record (src/lisp.h 3406-3440): the header word is
/// `PVEC_SUBR << PSEUDOVECTOR_AREA_BITS`, the function slot is C's `{.a ## maxargs = fnname}`,
/// and the initialiser stops at `command_modes`/`doc`.
fileprivate func make_aligned_subr(_ lname: StaticString, _ minargs: Int16, _ maxargs: Int16,
                                   _ function: UnsafeMutableRawPointer,
                                   docstring: StaticString, args: StaticString, interactive: StaticString? = nil) -> Aligned_Lisp_Subr {
  return Aligned_Lisp_Subr(s: Lisp_Subr(
    header: vectorlike_header(size: PVEC_SUBR() << PSEUDOVECTOR_AREA_BITS()),
    function: Lisp_Subr_function(ptr: function),
    min_args: minargs,
    max_args: maxargs,
    symbol_name: UnsafeRawPointer(lname.utf8Start).assumingMemoryBound(to: CChar.self),
    intspec: interactive.map { UnsafeRawPointer($0.utf8Start).assumingMemoryBound(to: CChar.self) },
    command_modes: Qnil(),
    doc: 0
  ))
}

/// C 1207: `static int desc_ctr = 1;` - `x-create-frame`'s window-descriptor counter (a
/// function-static in C, so a fileprivate global here).
fileprivate var desc_ctr: Int32 = 1

/// C 1192-1610.  `C 1199: NSWindow *main_window = [NSApp mainWindow];` is read before
/// anything else happens, as C does.
@_cdecl("Fx_create_frame")
func Fx_create_frame(_ parmsIn: Lisp_Object) -> Lisp_Object {
  var parms = parmsIn
  var f: UnsafeMutableRawPointer?
  var frame: Lisp_Object = Qnil()
  var tem: Lisp_Object
  var name: Lisp_Object
  var minibuffer_only = false
  var window_prompting = 0
  let count = SPECPDL_INDEX()
  var display: Lisp_Object
  var dpyinfo: UnsafeMutablePointer<ns_display_info>? = nil
  var parent: Lisp_Object
  var parent_frame: Lisp_Object
  var kb: UnsafeMutableRawPointer?
  let main_window = NSApp.mainWindow

  /* gui_display_get_arg modifies parms.  */
  parms = Fcopy_alist(parms)

  /* Use this general default value to start with
     until we know if this frame has a specified name.  */
  Vx_resource_name = Vinvocation_name

  display = gui_display_get_arg(dpyinfo, parms, Qterminal(), nil, nil, RES_TYPE_STRING())
  if EQ(display, Qunbound()) {
    display = Qnil()
  }
  dpyinfo = check_ns_display_info(display)
  kb = terminal_kboard(dpyinfo!.pointee.terminal)

  if terminal_name(dpyinfo!.pointee.terminal) == nil {
    error("Terminal is not live, can't create new frames on it")
  }

  name = gui_display_get_arg(dpyinfo, parms, Qname(), nil, nil, RES_TYPE_STRING())
  if !STRINGP(name) && !EQ(name, Qunbound()) && !NILP(name) {
    error("Invalid frame name--not a string or nil")
  }

  if STRINGP(name) {
    Vx_resource_name = name
  }

  parent = gui_display_get_arg(dpyinfo, parms, Qparent_id(), nil, nil, RES_TYPE_NUMBER())
  if EQ(parent, Qunbound()) {
    parent = Qnil()
  }
  if !NILP(parent) {
    CHECK_FIXNUM(parent)
  }

  /* make_frame_without_minibuffer can run Lisp code and garbage collect.  */
  /* No need to protect DISPLAY because that's not used after passing
     it to make_frame_without_minibuffer.  */
  frame = Qnil()
  tem = gui_display_get_arg(dpyinfo, parms, Qminibuffer(), "minibuffer", "Minibuffer",
                            RES_TYPE_SYMBOL())
  if EQ(tem, Qnone()) || NILP(tem) {
    f = make_frame_without_minibuffer(Qnil(), kb, display)
  } else if EQ(tem, Qonly()) {
    f = make_minibuffer_frame()
    minibuffer_only = true
  } else if WINDOWP(tem) {
    f = make_frame_without_minibuffer(tem, kb, display)
  } else {
    f = make_frame(true)
  }

  frame = nil
  withUnsafeMutablePointer(to: &frame) { XSETFRAME($0, f) }

  _ = frame_set_id_from_params(f, parms)

  frame_set_terminal(f, dpyinfo!.pointee.terminal)

  frame_set_output_method(f, output_ns())
  frame_alloc_ns_output(f)

  frame_set_fontset(f, -1)

  fset_icon_name(f, gui_display_get_arg(dpyinfo, parms, Qicon_name(), "iconName", "Title",
                                        RES_TYPE_STRING()))
  if !STRINGP(frame_icon_name(f)) {
    fset_icon_name(f, Qnil())
  }

  frame_set_display_info(f, dpyinfo)

  /* With FRAME_DISPLAY_INFO set up, this unwind-protect is safe.  */
  record_unwind_protect({ frame in do_unwind_create_frame(frame) }, frame)

  ns_output_set_window_desc(f, UInt(desc_ctr))
  desc_ctr += 1
  if TYPE_RANGED_FIXNUMP(parent) {
    ns_output_set_parent_desc(f, Int32(truncatingIfNeeded: XFIXNAT(parent)))
    ns_output_set_explicit_parent(f, true)
  } else {
    ns_output_set_parent_desc(f, FRAME_DISPLAY_INFO(f).pointee.root_window)
    ns_output_set_explicit_parent(f, false)
  }

  /* Set the name; the functions to which we pass f expect the name to
     be set.  */
  if EQ(name, Qunbound()) || NILP(name) || !STRINGP(name) {
    fset_name(f, lispString(ns_app_name!))
    frame_set_explicit_name(f, false)
  } else {
    fset_name(f, name)
    frame_set_explicit_name(f, true)
    specbind(Qx_resource_name(), name)
  }

  block_input()

  /* C 1310-1314: the `NS_IMPL_COCOA` arm registers the macfont driver; the `#else` arm is
     `register_font_driver (&nsfont_driver, f)`, another target's.  */
  mac_register_font_driver(f)

  _ = gui_default_parameter(f, parms, Qfont_backend(), Qnil(), "fontBackend", "FontBackend",
                            RES_TYPE_STRING())

  do {
    /* use for default font name */
    let font = NSFont.userFixedPitchFont(ofSize: -1.0)!  /* default */
    _ = gui_default_parameter(f, parms, Qfontsize(),
                              make_fixnum(0 /* (int)[font pointSize] */),
                              "fontSize", "FontSize", RES_TYPE_NUMBER())
    /* Remove ' Regular', not handled by backends.  */
    let fontname = xstrdup(font.displayName!)
    let len = Int(strlen(fontname!))
    if len > 8 && strcmp(fontname! + len - 8, " Regular") == 0 {
      fontname![len - 8] = 0
    }
    _ = gui_default_parameter(f, parms, Qfont(), build_string(fontname!),
                              "font", "Font", RES_TYPE_STRING())
    xfree(fontname)
  }
  unblock_input()

  _ = gui_default_parameter(f, parms, Qborder_width(), make_fixnum(0),
                            "borderwidth", "BorderWidth", RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qinternal_border_width(), make_fixnum(2),
                            "internalBorderWidth", "InternalBorderWidth", RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qchild_frame_border_width(), Qnil(),
                            "childFrameBorderWidth", "childFrameBorderWidth",
                            RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qright_divider_width(), make_fixnum(0),
                            nil, nil, RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qbottom_divider_width(), make_fixnum(0),
                            nil, nil, RES_TYPE_NUMBER())

  /* default vertical scrollbars on right on Mac */
  do {
    let spos = Qright()  /* C 1344-1348: `#ifdef NS_IMPL_GNUSTEP … = Qt; #else … = Qright;` */
    _ = gui_default_parameter(f, parms, Qvertical_scroll_bars(), spos,
                              "verticalScrollBars", "VerticalScrollBars", RES_TYPE_SYMBOL())
  }
  _ = gui_default_parameter(f, parms, Qhorizontal_scroll_bars(), Qnil(),
                            "horizontalScrollBars", "HorizontalScrollBars", RES_TYPE_SYMBOL())
  _ = gui_default_parameter(f, parms, Qforeground_color(), build_string("Black"),
                            "foreground", "Foreground", RES_TYPE_STRING())
  _ = gui_default_parameter(f, parms, Qbackground_color(), build_string("White"),
                            "background", "Background", RES_TYPE_STRING())
  // Establish a valid cursor color before the first native redisplay. Explicit
  // frame/default-frame parameters still take precedence over the default.
  _ = gui_default_parameter(f, parms, Qcursor_color(), build_string("Black"),
                            "cursorColor", "Foreground", RES_TYPE_STRING())
  _ = gui_default_parameter(f, parms, Qline_spacing(), Qnil(),
                            "lineSpacing", "LineSpacing", RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qleft_fringe(), Qnil(),
                            "leftFringe", "LeftFringe", RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qright_fringe(), Qnil(),
                            "rightFringe", "RightFringe", RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qno_special_glyphs(), Qnil(), nil, nil,
                            RES_TYPE_BOOLEAN())

  init_frame_faces(f)

  /* Read comment about this code in corresponding place in xfns.c.  */
  tem = gui_display_get_arg(dpyinfo, parms, Qmin_width(), nil, nil, RES_TYPE_NUMBER())
  if FIXNUMP(tem) {
    store_frame_param(f, Qmin_width(), tem)
  }
  tem = gui_display_get_arg(dpyinfo, parms, Qmin_height(), nil, nil, RES_TYPE_NUMBER())
  if FIXNUMP(tem) {
    store_frame_param(f, Qmin_height(), tem)
  }
  adjust_frame_size(f, FRAME_COLS(f) * FRAME_COLUMN_WIDTH(f),
                    FRAME_LINES(f) * FRAME_LINE_HEIGHT(f), 5, true,
                    Qx_create_frame_1())

  tem = gui_display_get_arg(dpyinfo, parms, Qundecorated(), nil, nil, RES_TYPE_BOOLEAN())
  frame_set_undecorated(f, !NILP(tem) && !EQ(tem, Qunbound()))
  store_frame_param(f, Qundecorated(), FRAME_UNDECORATED(f) ? Qt() : Qnil())

  /* C 1405-1418, the `NS_IMPL_COCOA` arm.  */
  tem = gui_display_get_arg(dpyinfo, parms, Qns_appearance(), nil, nil, RES_TYPE_SYMBOL())
  ns_set_appearance_1(f, tem)
  store_frame_param(f, Qns_appearance(),
                    (!NILP(tem) && !EQ(tem, Qunbound())) ? tem : Qnil())

  tem = gui_display_get_arg(dpyinfo, parms, Qns_transparent_titlebar(), nil, nil,
                            RES_TYPE_BOOLEAN())
  frame_set_ns_transparent_titlebar(f, !NILP(tem) && !EQ(tem, Qunbound()))
  store_frame_param(f, Qns_transparent_titlebar(),
                    FRAME_NS_TRANSPARENT_TITLEBAR(f) ? Qt() : Qnil())

  parent_frame = gui_display_get_arg(dpyinfo, parms, Qparent_frame(), nil, nil,
                                     RES_TYPE_SYMBOL())
  /* Accept parent-frame iff parent-id was not specified.  */
  if !NILP(parent)
      || EQ(parent_frame, Qunbound())
      || NILP(parent_frame)
      || !FRAMEP(parent_frame)
      || !FRAME_LIVE_P(XFRAME(parent_frame)) {
    parent_frame = Qnil()
  }

  fset_parent_frame(f, parent_frame)
  store_frame_param(f, Qparent_frame(), parent_frame)

  _ = gui_default_parameter(f, parms, Qz_group(), Qnil(), nil, nil, RES_TYPE_SYMBOL())
  _ = gui_default_parameter(f, parms, Qno_focus_on_map(), Qnil(), nil, nil, RES_TYPE_BOOLEAN())
  _ = gui_default_parameter(f, parms, Qno_accept_focus(), Qnil(), nil, nil, RES_TYPE_BOOLEAN())

  /* The resources controlling the menu-bar and tool-bar are
     processed specially at startup, and reflected in the mode
     variables; ignore them here.  */
  _ = gui_default_parameter(f, parms, Qmenu_bar_lines(),
                            NILP(Vmenu_bar_mode) ? make_fixnum(0) : make_fixnum(1),
                            nil, nil, RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qtab_bar_lines(),
                            NILP(Vtab_bar_mode) ? make_fixnum(0) : make_fixnum(1),
                            nil, nil, RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qtool_bar_lines(),
                            NILP(Vtool_bar_mode) ? make_fixnum(0) : make_fixnum(1),
                            nil, nil, RES_TYPE_NUMBER())

  _ = gui_default_parameter(f, parms, Qbuffer_predicate(), Qnil(), "bufferPredicate",
                            "BufferPredicate", RES_TYPE_SYMBOL())
  _ = gui_default_parameter(f, parms, Qtitle(), Qnil(), "title", "Title", RES_TYPE_STRING())

  /* C 1455-1461, the `NS_IMPL_COCOA` + macOS 10.14 arm.  */
  tem = gui_display_get_arg(dpyinfo, parms, Qinhibit_double_buffering(), nil, nil,
                            RES_TYPE_BOOLEAN())
  frame_set_double_buffered(f, NILP(tem) || EQ(tem, Qunbound()))
  store_frame_param(f, Qinhibit_double_buffering(),
                    FRAME_DOUBLE_BUFFERED(f) ? Qnil() : Qt())

  parms = get_geometry_from_preferences(dpyinfo, parms)
  window_prompting = gui_figure_window_size(f, parms, false, true)

  tem = gui_display_get_arg(dpyinfo, parms, Qunsplittable(), nil, nil, RES_TYPE_BOOLEAN())
  frame_set_no_split(f, minibuffer_only || (!EQ(tem, Qunbound()) && !NILP(tem)))

  /* NOTE: on other terms, this is done in set_mouse_color, however this
     was not getting called under Nextstep.  */
  ns_output_set_text_cursor(f, NSCursor.iBeam)
  ns_output_set_nontext_cursor(f, NSCursor.arrow)
  ns_output_set_modeline_cursor(f, NSCursor.pointingHand)
  ns_output_set_hand_cursor(f, NSCursor.pointingHand)
  ns_output_set_hourglass_cursor(f, NSCursor.disappearingItem)
  ns_output_set_horizontal_drag_cursor(f, NSCursor.resizeLeftRight)
  ns_output_set_vertical_drag_cursor(f, NSCursor.resizeUpDown)
  ns_output_set_left_edge_cursor(f, NSCursor.resizeLeftRight)
  ns_output_set_top_left_corner_cursor(f, NSCursor.arrow)
  ns_output_set_top_edge_cursor(f, NSCursor.resizeUpDown)
  ns_output_set_top_right_corner_cursor(f, NSCursor.arrow)
  ns_output_set_right_edge_cursor(f, NSCursor.resizeLeftRight)
  ns_output_set_bottom_right_corner_cursor(f, NSCursor.arrow)
  ns_output_set_bottom_edge_cursor(f, NSCursor.resizeUpDown)
  ns_output_set_bottom_left_corner_cursor(f, NSCursor.arrow)

  ns_display_info_set_vertical_scroll_bar_cursor(FRAME_DISPLAY_INFO(f), NSCursor.arrow)
  ns_display_info_set_horizontal_scroll_bar_cursor(FRAME_DISPLAY_INFO(f), NSCursor.arrow)
  ns_output_set_current_pointer(f, ns_output_text_cursor(f))

  ns_output_set_in_animation(f, 0)

  /* If the app has previously been disabled, start it up again.  */
  NSApp.setActivationPolicy(.regular)

  _ = EmacsView_initFrameFromEmacs(f)

  ns_icon(f, parms)

  /* ns_display_info does not have a reference_count.  */
  terminal_increment_reference_count(dpyinfo!.pointee.terminal)

  /* It is now ok to make the frame official even if we get an error
     below.  The frame needs to be on Vframe_list or making it visible
     won't work.  */
  Vframe_list = Fcons(frame, Vframe_list)

  _ = gui_default_parameter(f, parms, Qicon_type(), Qnil(),
                            "bitmapIcon", "BitmapIcon", RES_TYPE_SYMBOL())

  _ = gui_default_parameter(f, parms, Qauto_raise(), Qnil(),
                            "autoRaise", "AutoRaiseLower", RES_TYPE_BOOLEAN())
  _ = gui_default_parameter(f, parms, Qauto_lower(), Qnil(),
                            "autoLower", "AutoLower", RES_TYPE_BOOLEAN())
  _ = gui_default_parameter(f, parms, Qcursor_type(), Qbox(),
                            "cursorType", "CursorType", RES_TYPE_SYMBOL())
  _ = gui_default_parameter(f, parms, Qscroll_bar_width(), Qnil(),
                            "scrollBarWidth", "ScrollBarWidth", RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qscroll_bar_height(), Qnil(),
                            "scrollBarHeight", "ScrollBarHeight", RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qalpha(), Qnil(),
                            "alpha", "Alpha", RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qalpha_background(), Qnil(),
                            "alphaBackground", "AlphaBackground", RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qborders_respect_alpha_background(), Qnil(),
                            "bordersRespectAlphaBackground",
                            "BordersRespectAlphaBackground", RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qfullscreen(), Qnil(),
                            "fullscreen", "Fullscreen", RES_TYPE_SYMBOL())

  /* Allow set_window_size_hook, now.  */
  frame_set_can_set_window_size(f, true)

  adjust_frame_size(f, FRAME_TEXT_WIDTH(f), FRAME_TEXT_HEIGHT(f),
                    0, true, Qx_create_frame_2())

  if !ns_output_explicit_parent(f) {
    var visibility: Lisp_Object

    visibility = gui_display_get_arg(dpyinfo, parms, Qvisibility(), nil, nil,
                                     RES_TYPE_SYMBOL())
    if EQ(visibility, Qunbound()) {
      visibility = Qt()
    }

    if EQ(visibility, Qicon()) {
      ns_iconify_frame(f)
    } else if !NILP(visibility) {
      ns_make_frame_visible(f)
      FRAME_NS_VIEW(f)?.window?.makeKey()
    } else {
      /* Must have been Qnil.  */
      frame_set_was_invisible(f, true)
    }
  }

  if FRAME_HAS_MINIBUF_P(f)
      && (!FRAMEP(kboard_default_minibuffer_frame(kb))
          || !FRAME_LIVE_P(XFRAME(kboard_default_minibuffer_frame(kb)))) {
    kset_default_minibuffer_frame(kb, frame)
  }

  /* All remaining specified parameters, which have not been "used" by
     gui_display_get_arg and friends, now go in the misc. alist of the
     frame.  */
  tem = parms
  while CONSP(tem) {
    if CONSP(XCAR(tem)) && !NILP(XCAR(XCAR(tem))) {
      fset_param_alist(f, Fcons(XCAR(tem), frame_param_alist(f)))
    }
    tem = XCDR(tem)
  }

  /* This cascading behavior (which is the job of the window manager
     on X-based systems) is something NS applications are expected to
     implement themselves.  At least one person tells me he used
     Carbon Emacs solely for this behavior.  */
  if window_prompting & (USPosition() | PPosition()) != 0 || !NILP(FRAME_PARENT_FRAME(f)) {
    ns_set_offset(f, frame_left_pos(f), frame_top_pos(f), 1)
  } else {
    let frame_window = FRAME_NS_VIEW(f)?.window

    if main_window != nil {
      var top_left = NSPoint(x: main_window!.frame.minX, y: main_window!.frame.maxY)
      top_left = frame_window!.cascadeTopLeft(from: top_left)
      _ = frame_window!.cascadeTopLeft(from: top_left)
    } else {
      frame_window?.center()
    }
  }

  /* Make sure windows on this frame appear in calls to next-window
     and similar functions.  */
  Vwindow_list = Qnil()

  return unbind_to(count, frame)
}
fileprivate let lname_x_create_frame: StaticString = "x-create-frame"
fileprivate var Sx_create_frame: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_create_frame, 1, 1,
                    unsafeBitCast(Fx_create_frame as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "parms")

// ==========================================================================
// C 1612-1884: the frame z-order / restack / font / color-panel DEFUNs, the file dialog, and
// `ns_read_file_name`.
// ==========================================================================

/// C 1614-1622: test whether CANDIDATE is an ancestor window of WIN.
func ns_window_is_ancestor(_ win: NSWindow?, _ candidate: NSWindow?) -> Bool {
  if candidate == nil {
    return false
  } else if win == candidate {
    return true
  } else {
    return ns_window_is_ancestor(win, candidate?.parent)
  }
}

/// C 1624-1651: `DEFUN ("ns-frame-list-z-order", …)`.
@_cdecl("Fns_frame_list_z_order")
func Fns_frame_list_z_order(_ terminal: Lisp_Object) -> Lisp_Object {
  var frames: Lisp_Object = Qnil()
  var parent: NSWindow? = nil

  if FRAMEP(terminal) && FRAME_LIVE_P(XFRAME(terminal)) {
    parent = FRAME_NS_VIEW(XFRAME(terminal))?.window
  }

  for win in NSApp.orderedWindows.reversed() {
    var frame: Lisp_Object = Qnil()

    /* Check against [win parentWindow] so that it doesn't match itself. */
    if is_emacs_view(win.delegate) && (parent == nil || ns_window_is_ancestor(parent, win.parent)) {
      withUnsafeMutablePointer(to: &frame) { XSETFRAME($0, emacs_view_frame(win.delegate)) }
      frames = Fcons(frame, frames)
    }
  }

  return frames
}
fileprivate let lname_ns_frame_list_z_order: StaticString = "ns-frame-list-z-order"
fileprivate var Sns_frame_list_z_order: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_frame_list_z_order, 0, 1,
                    unsafeBitCast(Fns_frame_list_z_order as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Return list of Emacs's frames, in Z (stacking) order.\nIf TERMINAL is non-nil and specifies a live frame, return the child\nframes of that frame in Z (stacking) order.\n\nFrames are listed from topmost (first) to bottommost (last).  ", args: "terminal")

/// C 1653-1680: `DEFUN ("ns-frame-restack", …)`.
@_cdecl("Fns_frame_restack")
func Fns_frame_restack(_ frame1: Lisp_Object, _ frame2: Lisp_Object,
                       _ above: Lisp_Object) -> Lisp_Object {
  let f1 = decode_live_frame(frame1)
  let f2 = decode_live_frame(frame2)

  if FRAME_NS_VIEW(f1) != nil && FRAME_NS_VIEW(f2) != nil {
    let window = FRAME_NS_VIEW(f1)?.window
    let window2 = FRAME_NS_VIEW(f2)?.window

    if restackWindow(window, window2, !NILP(above)) {
      return Qt()
    } else {
      return Qnil()
    }
  } else {
    error("Cannot restack frames")
    /* C 1678-1679's `return Qnil` is unreachable (`error` is `_Noreturn`), recorded above.  */
  }
}
fileprivate let lname_ns_frame_restack: StaticString = "ns-frame-restack"
fileprivate var Sns_frame_restack: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_frame_restack, 2, 3,
                    unsafeBitCast(Fns_frame_restack as @convention(c) (Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Restack FRAME1 below FRAME2.\nThis means that if both frames are visible and the display areas of\nthese frames overlap, FRAME2 (partially) obscures FRAME1.  If optional\nthird argument ABOVE is non-nil, restack FRAME1 above FRAME2.  This\nmeans that if both frames are visible and the display areas of these\nframes overlap, FRAME1 (partially) obscures FRAME2.\n\nSome window managers may refuse to restack windows.  ", args: "frame1 frame2 above")

/// C 1682-1699: `DEFUN ("x-select-font", …)`.
@_cdecl("Fx_select_font")
func Fx_select_font(_ frame: Lisp_Object, _ ignored: Lisp_Object) -> Lisp_Object {
  let f = decode_window_system_frame(frame)
  let font = showFontPanel(FRAME_NS_VIEW(f))

  if NILP(font) {
    _ = quit()
  }

  return font
}
fileprivate let lname_x_select_font: StaticString = "x-select-font"
fileprivate var Sx_select_font: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_select_font, 0, 2,
                    unsafeBitCast(Fx_select_font as @convention(c) (Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Read a font using a Nextstep dialog.\nReturn a font specification describing the selected font.\n\nFRAME is the frame on which to pop up the font chooser.  If omitted or\nnil, it defaults to the selected frame. ", args: "frame ignored")

/// C 1701-1712: `DEFUN ("ns-popup-color-panel", …)` - C's `""` interactively is a null
/// interactive spec, which the port spells `nil`.
@_cdecl("Fns_popup_color_panel")
func Fns_popup_color_panel(_ frame: Lisp_Object) -> Lisp_Object {
  check_window_system(nil)
  NSApp.orderFrontColorPanel(NSApp)
  return Qnil()
}
fileprivate let lname_ns_popup_color_panel: StaticString = "ns-popup-color-panel"
fileprivate var Sns_popup_color_panel: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_popup_color_panel, 0, 1,
                    unsafeBitCast(Fns_popup_color_panel as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Pop up the color panel.  ", args: "frame", interactive: "")

/// C 1715-1724: the file dialog's `static struct { id panel; BOOL ret; … } ns_fd_data;`.  The
/// `NS_IMPL_GNUSTEP`-only `dirS`/`initS`/`no_types` members are recorded, not carried.
struct ns_fd_data_t {
  var panel: AnyObject? = nil
  var ret: Bool = false
}
fileprivate var ns_fd_data = ns_fd_data_t()

/// C 1768: `static id fileDelegate = nil;` - the panel's delegate, created once and kept
/// (the port of C 4048-4065's `EmacsFileDelegate` class owns the class itself).
fileprivate var file_delegate: AnyObject? = nil

/// C 1774: `fileDelegate = [EmacsFileDelegate new];` - the call site precedes the class
/// (C 4048-4065) *in this same file*, so per 3.239 the declaration is a distinct Swift name
/// into the same C symbol, and the `@_cdecl("EmacsFileDelegate_new")` at the class is the
/// definition (the `macfont_driver` precedent).

/// C 1726-1749 (non-static).  The `NS_IMPL_COCOA` arm is `[panel runModal]`; the `#else` arm
/// is GNUstep's `runModalForDirectory:file:[types:]` pair.
@_cdecl("ns_run_file_dialog")
func ns_run_file_dialog() {
  if ns_fd_data.panel == nil { return }
  ns_fd_data.ret = (ns_fd_data.panel as? NSSavePanel)?.runModal() == .OK
  ns_fd_data.panel = nil
}

/* C 1752-1760: `#define MODAL_OK_RESPONSE NSModalResponseOK` (the `MAC_OS_X_VERSION_MAX_ALLOWED
   > 1090` arm) with a GNUstep `NSOKButton` fallback.  The port uses the SDK's spelling,
   `NSApplication.ModalResponse.OK`, at its one use site.  */

/// C 1762-1884: `DEFUN ("ns-read-file-name", …)`.
@_cdecl("Fns_read_file_name")
func Fns_read_file_name(_ prompt: Lisp_Object, _ dir: Lisp_Object, _ mustmatch: Lisp_Object,
                        _ init_ : Lisp_Object, _ dir_only_p: Lisp_Object) -> Lisp_Object {
  let isSave = NILP(mustmatch) && NILP(dir_only_p)
  var panel: NSSavePanel?
  var fname: Lisp_Object = Qnil()
  var promptS: NSString?
  var dirS: NSString?
  var initS: NSString?
  var str: NSString?

  promptS = (NILP(prompt) || !STRINGP(prompt)) ? nil : stringWithLispString(prompt)
  if NILP(dir) || !STRINGP(dir) {
    dirS = stringWithLispString(ENCODE_FILE(buffer_directory(current_buffer())))
  } else {
    dirS = stringWithLispString(ENCODE_FILE(dir))
  }
  initS = (NILP(init_) || !STRINGP(init_)) ? nil : stringWithLispString(init_)

  check_window_system(nil)

  /* C 1770: `static id fileDelegate = nil;` - the delegate is kept for the panel's lifetime. */
  if file_delegate == nil {
    file_delegate = EmacsFileDelegate_new()
  }

  NSCursor.setHiddenUntilMouseMoves(false)

  if dirS!.character(at: 0) == UInt16(UInt8(ascii: "~")) {
    dirS = dirS!.expandingTildeInPath as NSString
  }

  panel = isSave ? NSSavePanel() : NSOpenPanel()

  panel?.title = promptS as String?  /* C 1781: `[panel setTitle: promptS]` */

  panel?.allowsOtherFileTypes = true
  panel?.treatsFilePackagesAsDirectories = true
  panel?.delegate = file_delegate as? any NSOpenSavePanelDelegate

  if !NILP(dir_only_p) {
    (panel as? NSOpenPanel)?.canChooseDirectories = true
    (panel as? NSOpenPanel)?.canChooseFiles = false
  } else if !isSave {
    /* This is not quite what the documentation says, but it is compatible
       with the Gtk+ code.  Also, the menu entry says "Open File...".  */
    (panel as? NSOpenPanel)?.canChooseDirectories = false
    (panel as? NSOpenPanel)?.canChooseFiles = true
  }

  block_input()
  ns_fd_data.panel = panel
  ns_fd_data.ret = false
  if !NILP(mustmatch) || !NILP(dir_only_p) {
    /* C 1802-1816, the `NS_IMPL_COCOA` + macOS 12 arm.  */
    panel?.allowedContentTypes = []
  }
  if dirS != nil { panel?.directoryURL = URL(fileURLWithPath: dirS! as String) }
  if initS != nil && NILP(Ffile_directory_p(init_)) {
    panel?.nameFieldStringValue = (initS! as String as NSString).lastPathComponent
  } else {
    panel?.nameFieldStringValue = ""
  }

  /* runModalForDirectory/runModal restarts the main event loop when done,
     so we must start an event loop and then pop up the file dialog.
     The file dialog may pop up a confirm dialog after Ok has been pressed,
     so we can not simply pop down on the Ok/Cancel press.
   */
  let nxev = NSEvent.otherEvent(with: .applicationDefined,
                                location: NSPoint(x: 0, y: 0),
                                modifierFlags: [],
                                timestamp: 0,
                                windowNumber: NSApp.mainWindow?.windowNumber ?? 0,
                                context: nil,
                                subtype: 0,
                                data1: 0,
                                data2: NSAPP_DATA2_RUNFILEDIALOG())

  NSApp.postEvent(nxev!, atStart: false)
  while ns_fd_data.panel != nil {
    NSApp.run()
  }

  if ns_fd_data.ret {
    str = ns_filename_from_panel(panel!)

    if str == nil {
      str = ns_directory_from_panel(panel!)
    }
    if str != nil {
      fname = lispString(str!)
    }

    if !NILP(fname) {
      fname = DECODE_FILE(fname)
    }
  }

  FRAME_NS_VIEW(SELECTED_FRAME())?.window?.makeKey()
  unblock_input()

  return fname
}
fileprivate let lname_ns_read_file_name: StaticString = "ns-read-file-name"
fileprivate var Sns_read_file_name: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_read_file_name, 1, 5,
                    unsafeBitCast(Fns_read_file_name as @convention(c) (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Use a graphical panel to read a file name, using prompt PROMPT.\nOptional arg DIR, if non-nil, supplies a default directory.\nOptional arg MUSTMATCH, if non-nil, means the returned file or\ndirectory must exist.\nOptional arg INIT, if non-nil, provides a default file name to use.\nOptional arg DIR-ONLY-P, if non-nil, means choose only directories.  ", args: "prompt dir mustmatch init dir_only_p")

// ==========================================================================
// C 1886-2610: the defaults/resources group, the x-server-*/x-display-* DEFUNs, the
// application-level DEFUNs, AppleScript, the scroll-bar defaults and the trash handler.
// ==========================================================================

/// C: `#define calln(…) CALLN (Ffuncall, __VA_ARGS__)` (src/lisp.h 3462), the
/// `CALLN`-array convention docs/ports/nsselect.swift already spells this way.
@inline(__always)
fileprivate func calln(_ args: [Lisp_Object]) -> Lisp_Object {
  var a = args
  return a.withUnsafeMutableBufferPointer { buf in
    Ffuncall(buf.count, buf.baseAddress!)
  }
}

/// C 1893 returns a pointer into an autoreleased `NSString`; C's callers use it before the pool
/// drains.  A Swift bridged temporary would be released at the end of this call, so the port
/// keeps the last string alive (a one-slot cache) and returns the same kind of pointer.
fileprivate var ns_defaults_last_string: NSString? = nil

/// C 1885-1895 (non-static - `@_cdecl`: src/nsterm.h 1196 declares it for the other units).
/// C 1885-1895: `const char *ns_get_defaults_value (const char *key)`.  `@_cdecl` refuses an
/// `UnsafePointer<T>?` result in Swift 6.4 (measured: "its result type cannot be represented in
/// Objective-C"), so the port spells the result as the mutable pointer - the same C ABI.
@_cdecl("ns_get_defaults_value")
func ns_get_defaults_value(_ key: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>? {
  let obj = UserDefaults.standard.object(forKey: String(cString: key))

  if obj == nil { return nil }

  /* C 1893: `[[NSString stringWithFormat: @"%@", obj] UTF8String]` - `%@` on an `id` asks for
     its `description`, which `String (describing:)` is.  */
  ns_defaults_last_string = String(describing: obj!) as NSString
  return UnsafeMutablePointer(mutating: ns_defaults_last_string!.utf8String)
}

/// C 1897-1915: `DEFUN ("ns-get-resource", …)`.
@_cdecl("Fns_get_resource")
func Fns_get_resource(_ ownerIn: Lisp_Object, _ name: Lisp_Object) -> Lisp_Object {
  var owner = ownerIn
  var value: UnsafeMutablePointer<CChar>?

  check_window_system(nil)
  if NILP(owner) {
    owner = lispString(ns_app_name!)
  }
  CHECK_STRING(name)

  value = ns_get_defaults_value(SSDATA(name))

  if value != nil {
    return build_string(value!)
  }
  return Qnil()
}
fileprivate let lname_ns_get_resource: StaticString = "ns-get-resource"
fileprivate var Sns_get_resource: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_get_resource, 2, 2,
                    unsafeBitCast(Fns_get_resource as @convention(c) (Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Return the value of the property NAME of OWNER from the defaults database.\nIf OWNER is nil, Emacs is assumed.  ", args: "owner name")

/// C 1917-1942: `DEFUN ("ns-set-resource", …)`.
@_cdecl("Fns_set_resource")
func Fns_set_resource(_ ownerIn: Lisp_Object, _ name: Lisp_Object,
                      _ value: Lisp_Object) -> Lisp_Object {
  var owner = ownerIn

  check_window_system(nil)
  if NILP(owner) {
    owner = lispString(ns_app_name!)
  }
  CHECK_STRING(name)
  if NILP(value) {
    UserDefaults.standard.removeObject(forKey: stringWithLispString(name)! as String)
  } else {
    CHECK_STRING(value)
    UserDefaults.standard.set(stringWithLispString(value)! as String,
                              forKey: stringWithLispString(name)! as String)
  }

  return Qnil()
}
fileprivate let lname_ns_set_resource: StaticString = "ns-set-resource"
fileprivate var Sns_set_resource: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_set_resource, 3, 3,
                    unsafeBitCast(Fns_set_resource as @convention(c) (Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Set property NAME of OWNER to VALUE, from the defaults database.\nIf OWNER is nil, Emacs is assumed.\nIf VALUE is nil, the default is removed.  ", args: "owner name value")

/// C 1944-1955: `DEFUN ("x-server-max-request-size", …)` - no real equivalent under Nextstep.
@_cdecl("Fx_server_max_request_size")
func Fx_server_max_request_size(_ terminal: Lisp_Object) -> Lisp_Object {
  _ = check_ns_display_info(terminal)
  return Qnil()
}
fileprivate let lname_x_server_max_request_size: StaticString = "x-server-max-request-size"
fileprivate var Sx_server_max_request_size: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_server_max_request_size, 0, 1,
                    unsafeBitCast(Fx_server_max_request_size as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 1957-1968: `DEFUN ("x-server-vendor", …)` - the `NS_IMPL_GNUSTEP` arm returns "GNU".
@_cdecl("Fx_server_vendor")
func Fx_server_vendor(_ terminal: Lisp_Object) -> Lisp_Object {
  _ = check_ns_display_info(terminal)
  return build_string("Apple")
}
fileprivate let lname_x_server_vendor: StaticString = "x-server-vendor"
fileprivate var Sx_server_vendor: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_server_vendor, 0, 1,
                    unsafeBitCast(Fx_server_vendor as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 1970-1983: `DEFUN ("x-server-version", …)`.
@_cdecl("Fx_server_version")
func Fx_server_version(_ terminal: Lisp_Object) -> Lisp_Object {
  _ = check_ns_display_info(terminal)
  /* NOTE: it is unclear what would best correspond with "protocol";
           we return 10.3, meaning Panther, since this is roughly the
           level that GNUstep's APIs correspond to.  The last number
           is where we distinguish between the Apple and GNUstep
           implementations ("distributor-specific release number") and
           give int'ized versions of major.minor.  */
  return list3i(10, 3, Int(ns_appkit_version_int()))
}
fileprivate let lname_x_server_version: StaticString = "x-server-version"
fileprivate var Sx_server_version: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_server_version, 0, 1,
                    unsafeBitCast(Fx_server_version as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 1985-1992: `DEFUN ("x-display-screens", …)`.
@_cdecl("Fx_display_screens")
func Fx_display_screens(_ terminal: Lisp_Object) -> Lisp_Object {
  _ = check_ns_display_info(terminal)
  return make_fixnum(1)
}
fileprivate let lname_x_display_screens: StaticString = "x-display-screens"
fileprivate var Sx_display_screens: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_screens, 0, 1,
                    unsafeBitCast(Fx_display_screens as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 1994-2013: `DEFUN ("x-display-mm-height", …)` - the `NS_IMPL_COCOA` arm measures the main
/// display; the `#else` arm is `25.4 / dpyinfo->resx`.
@_cdecl("Fx_display_mm_height")
func Fx_display_mm_height(_ terminal: Lisp_Object) -> Lisp_Object {
  var px_to_mm: Double
  let dpyinfo = check_ns_display_info(terminal)

  let did = CGMainDisplayID()
  let size_mm = CGDisplayScreenSize(did)
  let bounds = CGDisplayBounds(did)

  px_to_mm = size_mm.height / bounds.size.height

  return make_fixnum(Int(Double(ns_display_pixel_height(dpyinfo)) * px_to_mm))
}
fileprivate let lname_x_display_mm_height: StaticString = "x-display-mm-height"
fileprivate var Sx_display_mm_height: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_mm_height, 0, 1,
                    unsafeBitCast(Fx_display_mm_height as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2015-2034: `DEFUN ("x-display-mm-width", …)`.
@_cdecl("Fx_display_mm_width")
func Fx_display_mm_width(_ terminal: Lisp_Object) -> Lisp_Object {
  var px_to_mm: Double
  let dpyinfo = check_ns_display_info(terminal)

  let did = CGMainDisplayID()
  let size_mm = CGDisplayScreenSize(did)
  let bounds = CGDisplayBounds(did)

  px_to_mm = size_mm.width / bounds.size.width

  return make_fixnum(Int(Double(ns_display_pixel_width(dpyinfo)) * px_to_mm))
}
fileprivate let lname_x_display_mm_width: StaticString = "x-display-mm-width"
fileprivate var Sx_display_mm_width: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_mm_width, 0, 1,
                    unsafeBitCast(Fx_display_mm_width as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2036-2058: `DEFUN ("x-display-backing-store", …)`.  The `NSBackingStoreRetained` /
/// `Nonretained` arms are the `defined (NS_IMPL_GNUSTEP) || MAC_OS_X_VERSION_MIN_REQUIRED <
/// 101300` arm - dead text on this target, recorded.
@_cdecl("Fx_display_backing_store")
func Fx_display_backing_store(_ terminal: Lisp_Object) -> Lisp_Object {
  _ = check_ns_display_info(terminal)
  /* Note that the xfns.c version has different return values.  */
  switch (ns_get_window(terminal) as? NSWindow)?.backingType ?? .buffered {
  case .buffered:
    return Qbuffered()
  default:
    error("Strange value for backingType parameter of frame")
  }
}
fileprivate let lname_x_display_backing_store: StaticString = "x-display-backing-store"
fileprivate var Sx_display_backing_store: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_backing_store, 0, 1,
                    unsafeBitCast(Fx_display_backing_store as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2060-2085: `DEFUN ("x-display-visual-class", …)`.
@_cdecl("Fx_display_visual_class")
func Fx_display_visual_class(_ terminal: Lisp_Object) -> Lisp_Object {
  var depth: NSWindow.Depth

  _ = check_ns_display_info(terminal)
  depth = NSScreen.screens[0].depth

  if depth == NSBestDepth(NSColorSpaceName.calibratedWhite, 2, 2, true, nil) {
    return Qstatic_gray()
  } else if depth == NSBestDepth(NSColorSpaceName.calibratedWhite, 8, 8, true, nil) {
    return Qgray_scale()
  } else if depth == NSBestDepth(NSColorSpaceName.calibratedRGB, 8, 8, true, nil) {
    return Qpseudo_color()
  } else if depth == NSBestDepth(NSColorSpaceName.calibratedRGB, 4, 12, false, nil) {
    return Qtrue_color()
  } else if depth == NSBestDepth(NSColorSpaceName.calibratedRGB, 8, 24, false, nil) {
    return Qdirect_color()
  } else {
    /* Color management as far as we do it is really handled by
       Nextstep itself anyway.  */
    return Qdirect_color()
  }
}
fileprivate let lname_x_display_visual_class: StaticString = "x-display-visual-class"
fileprivate var Sx_display_visual_class: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_visual_class, 0, 1,
                    unsafeBitCast(Fx_display_visual_class as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2087-2109: `DEFUN ("x-display-save-under", …)`.
@_cdecl("Fx_display_save_under")
func Fx_display_save_under(_ terminal: Lisp_Object) -> Lisp_Object {
  _ = check_ns_display_info(terminal)
  switch (ns_get_window(terminal) as? NSWindow)?.backingType ?? .buffered {
  case .buffered:
    return Qt()
  default:
    error("Strange value for backingType parameter of frame")
  }
}
fileprivate let lname_x_display_save_under: StaticString = "x-display-save-under"
fileprivate var Sx_display_save_under: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_save_under, 0, 1,
                    unsafeBitCast(Fx_display_save_under as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2111-2134: `DEFUN ("x-open-connection", …)`.
@_cdecl("Fx_open_connection")
func Fx_open_connection(_ display: Lisp_Object, _ resource_string: Lisp_Object,
                        _ must_succeed: Lisp_Object) -> Lisp_Object {
  var dpyinfo: UnsafeMutablePointer<ns_display_info>?

  CHECK_STRING(display)

  nxatoms_of_nsselect()
  dpyinfo = ns_term_init(display)
  if dpyinfo == nil {
    if !NILP(must_succeed) {
      fatal("Display on %s not responding.\n", SDATA(display))
    } else {
      error("Display on %s not responding.\n", SDATA(display))
    }
  }

  return Qnil()
}
fileprivate let lname_x_open_connection: StaticString = "x-open-connection"
fileprivate var Sx_open_connection: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_open_connection, 1, 3,
                    unsafeBitCast(Fx_open_connection as @convention(c) (Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "display resource_string must_succeed")

/// C 2136-2145: `DEFUN ("x-close-connection", …)`.
@_cdecl("Fx_close_connection")
func Fx_close_connection(_ terminal: Lisp_Object) -> Lisp_Object {
  _ = check_ns_display_info(terminal)
  NSApp.terminate(NSApp)
  return Qnil()
}
fileprivate let lname_x_close_connection: StaticString = "x-close-connection"
fileprivate var Sx_close_connection: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_close_connection, 1, 1,
                    unsafeBitCast(Fx_close_connection as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2147-2159: `DEFUN ("x-display-list", …)`.
@_cdecl("Fx_display_list")
func Fx_display_list() -> Lisp_Object {
  var result: Lisp_Object = Qnil()
  var ndi = x_display_list

  while ndi != nil {
    result = Fcons(XCAR(ndi!.pointee.name_list_element), result)
    ndi = ndi!.pointee.next
  }

  return result
}
fileprivate let lname_x_display_list: StaticString = "x-display-list"
fileprivate var Sx_display_list: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_list, 0, 0,
                    unsafeBitCast(Fx_display_list as @convention(c) () -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "void")

/// C 2161-2169: `DEFUN ("ns-hide-others", …)`.
@_cdecl("Fns_hide_others")
func Fns_hide_others() -> Lisp_Object {
  check_window_system(nil)
  NSApp.hideOtherApplications(NSApp)
  return Qnil()
}
fileprivate let lname_ns_hide_others: StaticString = "ns-hide-others"
fileprivate var Sns_hide_others: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_hide_others, 0, 0,
                    unsafeBitCast(Fns_hide_others as @convention(c) () -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Hides all applications other than Emacs.  ", args: "void")

/// C 2171-2202: `DEFUN ("ns-hide-emacs", …)`.  The `activate-front` arm is the
/// `GNUSTEP_GUI_MAJOR_VERSION`-guarded one, dead text on this target (recorded).
@_cdecl("Fns_hide_emacs")
func Fns_hide_emacs(_ on: Lisp_Object) -> Lisp_Object {
  check_window_system(nil)
  if EQ(on, Qactivate()) {
    NSApp.unhide(NSApp)
    NSApp.activate(ignoringOtherApps: true)
  } else if NILP(on) {
    NSApp.unhide(NSApp)
  } else {
    NSApp.hide(NSApp)
  }
  return Qnil()
}
fileprivate let lname_ns_hide_emacs: StaticString = "ns-hide-emacs"
fileprivate var Sns_hide_emacs: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_hide_emacs, 1, 1,
                    unsafeBitCast(Fns_hide_emacs as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " If ON is non-nil, the entire Emacs application is hidden.\nOtherwise if Emacs is hidden, it is unhidden.\nIf ON is equal to `activate', Emacs is unhidden and becomes\nthe active application.\nIf ON is equal to `activate-front', Emacs is unhidden and\nbecomes the active application, but only the selected frame\nis layered in front of the windows of other applications.  ", args: "on")

/// C 2204-2213: `DEFUN ("ns-emacs-info-panel", …)`.
@_cdecl("Fns_emacs_info_panel")
func Fns_emacs_info_panel() -> Lisp_Object {
  check_window_system(nil)
  NSApp.orderFrontStandardAboutPanel(nil)
  return Qnil()
}
fileprivate let lname_ns_emacs_info_panel: StaticString = "ns-emacs-info-panel"
fileprivate var Sns_emacs_info_panel: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_emacs_info_panel, 0, 0,
                    unsafeBitCast(Fns_emacs_info_panel as @convention(c) () -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Shows the `Info' or `About' panel for Emacs.  ", args: "void")

/// C 2215-2235: `DEFUN ("ns-font-name", …)`.
@_cdecl("Fns_font_name")
func Fns_font_name(_ name: Lisp_Object) -> Lisp_Object {
  var nm: UnsafeMutablePointer<CChar>
  CHECK_STRING(name)
  nm = SSDATA(name)

  if nm[0] != UInt8(ascii: "-") {
    return name
  }
  if strstr(nm, "fontset") != nil && strstr(nm, "fontset-startup") == nil {
    return name
  }

  return build_string(ns_xlfd_to_fontname(SSDATA(name)))
}
fileprivate let lname_ns_font_name: StaticString = "ns-font-name"
fileprivate var Sns_font_name: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_font_name, 1, 1,
                    unsafeBitCast(Fns_font_name as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Determine font PostScript or family name for font NAME.\nNAME should be a string containing either the font name or an XLFD\nfont descriptor.  If string contains `fontset' and not\n`fontset-startup', it is left alone.  ", args: "name")

/// C 2237-2289 (non-static).  Called in emacs.c to support early calls to `ns_lisp_to_color`
/// or `Fns_list_colors`.
@_cdecl("ns_init_colors")
func ns_init_colors() {
  /* C 2240: `NSTRACE ("ns_init_colors");` */

  var cl = NSColorList(named: "Emacs")

  /* There are 752 colors defined in rgb.txt.  */
  if cl == nil || (cl!.allKeys.count) < 752 {
    var color_file: Lisp_Object
    var color_map: Lisp_Object
    var color: Lisp_Object
    var name: Lisp_Object
    var c: UInt

    color_file = Fexpand_file_name(build_string("rgb.txt"),
                                   Fsymbol_value(intern("data-directory")))

    color_map = Fx_load_color_file(color_file)
    if NILP(color_map) {
      fatal("Could not read %s.\n", SDATA(color_file))
    }

    cl = NSColorList(name: "Emacs")
    while CONSP(color_map) {
      color = XCAR(color_map)
      name = XCAR(color)
      c = UInt(XFIXNUM(XCDR(color)))
      c |= 0xFF000000
      cl!.setColor(NSColor_colorWithUnsignedLong(c),
                   forKey: stringWithLispString(name)! as String)
      color_map = XCDR(color_map)
    }
    /* C 2290-2310: C wraps `[cl writeToURL:nil error:nil]` in `@try`/`@catch (NSException *)`
       and logs a `fprintf (stderr, …)`/`NSLog` line on failure.  Swift cannot catch an
       `NSException`, so the guarded write is the C-ABI entry `ns_color_list_write_or_log`
       (see the ledger's `## ABI` (3)); it returns `false` on failure, like C's `== false`.  */
    if !ns_color_list_write_or_log(cl) {
      FileHandle.standardError.write(Data("ns_init_colors: could not write Emacs.clr\n".utf8))
    }
  }
}

/// C 2291-2329: `DEFUN ("ns-list-colors", …)`.
@_cdecl("Fns_list_colors")
func Fns_list_colors(_ frame: Lisp_Object) -> Lisp_Object {
  var list: Lisp_Object = Qnil()

  if !NILP(frame) {
    CHECK_FRAME(frame)
    if !FRAME_NS_P(XFRAME(frame)) {
      error("non-Nextstep frame used in `ns-list-colors'")
    }
  }

  block_input()
  /* This can be called during dumping, so we need to set up a
     temporary autorelease pool.  */
  autoreleasepool {
    /* C 2316-2322: `[[NSColorList availableColorLists] objectEnumerator]` and
       `[[clist allKeys] reverseObjectEnumerator]` - Swift vends both as arrays.  */
    for cl in NSColorList.availableColorLists {
      /* C 2316-2319: `[[clist name] length]` / `[[clist name] rangeOfString: @"PANTONE"]` -
         `NSColorList.name` is optional in Swift.  */
      let cname = cl.name ?? ""
      if cname.count < 7 || (cname as NSString).range(of: "PANTONE").location == 0 {
        for cname in cl.allKeys.reversed() {
          list = Fcons(lispString(cname as NSString), list)
        }
      }
    }
  }
  unblock_input()

  return list
}
fileprivate let lname_ns_list_colors: StaticString = "ns-list-colors"
fileprivate var Sns_list_colors: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_list_colors, 0, 1,
                    unsafeBitCast(Fns_list_colors as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Return a list of all available colors.\nThe optional argument FRAME is currently ignored.  ", args: "frame")

/// C 2331-2354: `DEFUN ("ns-list-services", …)`.  The `NS_IMPL_COCOA` arm is
/// `/* You can't get services like this in 10.6+.  */ return Qnil;` - the `#else` arm is the
/// GNUstep one, which calls the manifest-rowed `interpret_services_menu`.
@_cdecl("Fns_list_services")
func Fns_list_services() -> Lisp_Object {
  return Qnil()
}
fileprivate let lname_ns_list_services: StaticString = "ns-list-services"
fileprivate var Sns_list_services: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_list_services, 0, 0,
                    unsafeBitCast(Fns_list_services as @convention(c) () -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " List available Nextstep services by querying NSApp.  ", args: "void")

/// C 2356-2374: `DEFUN ("ns-perform-service", …)`.
@_cdecl("Fns_perform_service")
func Fns_perform_service(_ service: Lisp_Object, _ send: Lisp_Object) -> Lisp_Object {
  var pb: AnyObject?
  var svcName: NSString?

  CHECK_STRING(service)
  check_window_system(nil)

  svcName = stringWithLispString(service)

  pb = NSPasteboard.withUniqueName()
  ns_string_to_pasteboard(pb, send)

  if !NSPerformService(svcName! as String, pb as? NSPasteboard) {
    _ = Fsignal(Qquit(), list1(build_string("service not available")))
  }

  if ((pb as? NSPasteboard)?.types?.count ?? 0) == 0 {
    return build_string("")
  }
  return ns_string_from_pasteboard(pb)
}
fileprivate let lname_ns_perform_service: StaticString = "ns-perform-service"
fileprivate var Sns_perform_service: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_perform_service, 2, 2,
                    unsafeBitCast(Fns_perform_service as @convention(c) (Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Perform Nextstep SERVICE on SEND.\nSEND should be either a string or nil.\nThe return value is the result of the service, as string, or nil if\nthere was no result.  ", args: "service send")

// C 2378-2445 (the `NS_IMPL_COCOA` arm): compile and execute SCRIPT and return the error status
// as function value - 0 on success (with `*result` set), 1 otherwise.
func ns_do_applescript(_ script: Lisp_Object,
                       _ result: UnsafeMutablePointer<Lisp_Object>) -> Int32 {
  var desc: NSAppleEventDescriptor?
  var errorDict: NSDictionary?
  var returnDescriptor: NSAppleEventDescriptor? = nil

  let scriptObject = NSAppleScript(source: stringWithLispString(script)! as String)

  returnDescriptor = scriptObject?.executeAndReturnError(&errorDict)
  result.pointee = Qnil()

  if returnDescriptor != nil {
    // successful execution
    if kAENullEvent != returnDescriptor!.descriptorType {
      result.pointee = Qt()
      // script returned an AppleScript result
      if (typeUnicodeText == returnDescriptor!.descriptorType)
          || (typeUTF16ExternalRepresentation == returnDescriptor!.descriptorType)
          || (typeUTF8Text == returnDescriptor!.descriptorType)
          || (typeCString == returnDescriptor!.descriptorType) {
        desc = returnDescriptor!.coerce(toDescriptorType: typeUTF8Text)
        if desc != nil {
          result.pointee = lispString(desc!.stringValue! as NSString)
        }
      } else {
        /* use typeUTF16ExternalRepresentation? */
        // coerce the result to the appropriate ObjC type
        desc = returnDescriptor!.coerce(toDescriptorType: typeUTF8Text)
        if desc != nil {
          result.pointee = make_fixnum(Int(desc!.int32Value))
        }
      }
    }
  } else {
    // no script result, return error
    return 1
  }
  return 0
}

/// C 2447-2452 (non-static - src/nsterm.m's event loop calls it): helper called from sendEvent
/// to run AppleScript from within the main event loop.
@_cdecl("ns_run_ascript")
func ns_run_ascript() {
  if !NILP(as_script) {
    as_status = ns_do_applescript(as_script, as_result!)
  }
  as_script = Qnil()
}

/// C 2454-2518: `DEFUN ("ns-do-applescript", …)`.
@_cdecl("Fns_do_applescript")
func Fns_do_applescript(_ script: Lisp_Object) -> Lisp_Object {
  var result: Lisp_Object = Qnil()
  var status: Int32

  CHECK_STRING(script)
  check_window_system(nil)

  block_input()

  as_script = script
  as_result = withUnsafeMutablePointer(to: &result) { $0 }

  /* Executing AppleScript requires the event loop to run, otherwise
     errors aren't returned and executeAndReturnError hangs forever.
     Post an event that runs AppleScript and then start the event
     loop.  The event loop is exited when the script is done.  */
  let nxev = NSEvent.otherEvent(with: .applicationDefined,
                                location: NSPoint(x: 0, y: 0),
                                modifierFlags: [],
                                timestamp: 0,
                                windowNumber: NSApp.mainWindow?.windowNumber ?? 0,
                                context: nil,
                                subtype: 0,
                                data1: 0,
                                data2: NSAPP_DATA2_RUNASSCRIPT())

  NSApp.postEvent(nxev!, atStart: false)

  /* If there are other events, the event loop may exit.  Keep running
     until the script has been handled.  */
  /* C 2482: `struct input_event ev;` - the scratch record `ns_init_events` fills; the record is
     src/termhooks.h's (64 bytes, measured in the ledger's Evidence), and only its address
     crosses the boundary.  */
  let ev = UnsafeMutableRawPointer.allocate(byteCount: 64, alignment: 8)
  ns_init_events(ev)
  while !NILP(as_script) {
    NSApp.run()
  }
  ns_finish_events()

  status = as_status
  as_status = 0
  as_result = nil
  ev.deallocate()
  unblock_input()
  if status == 0 {
    return result
  } else if !STRINGP(result) {
    error("AppleScript error %d", status)
  } else {
    error("%s", SSDATA(result))
  }
}
fileprivate let lname_ns_do_applescript: StaticString = "ns-do-applescript"
fileprivate var Sns_do_applescript: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_do_applescript, 1, 1,
                    unsafeBitCast(Fns_do_applescript as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Execute AppleScript SCRIPT and return the result.\nIf compilation and execution are successful, the resulting script value\nis returned as a string, a number or, in the case of other constructs, t.\nIn case the execution fails, an error is signaled.  ", args: "script")

/// C 2520-2524 (non-static): `struct ns_display_info *check_x_display_info (Lisp_Object)`,
/// called from frame.c.  `@_cdecl` refuses a pointer to a *Swift* type, so the result is the
/// opaque pointer C's prototype is (`struct ns_display_info` is incomplete outside this file).
@_cdecl("check_x_display_info")
func check_x_display_info(_ frame: Lisp_Object) -> UnsafeMutableRawPointer? {
  return UnsafeMutableRawPointer(check_ns_display_info(frame))
}

/// C 2527-2534 (non-static).
@_cdecl("ns_set_scroll_bar_default_width")
func ns_set_scroll_bar_default_width(_ f: UnsafeMutableRawPointer?) {
  let wid = FRAME_COLUMN_WIDTH(f)
  frame_set_config_scroll_bar_width(f, NS_scroll_bar_width_default())
  frame_set_config_scroll_bar_cols(f, (FRAME_CONFIG_SCROLL_BAR_WIDTH(f) + wid - 1) / wid)
}

/// C 2536-2543 (non-static).
@_cdecl("ns_set_scroll_bar_default_height")
func ns_set_scroll_bar_default_height(_ f: UnsafeMutableRawPointer?) {
  let height = FRAME_LINE_HEIGHT(f)
  frame_set_config_scroll_bar_height(f, NS_scroll_bar_width_default())
  frame_set_config_scroll_bar_lines(f,
                                    (FRAME_CONFIG_SCROLL_BAR_HEIGHT(f) + height - 1) / height)
}

/// C: `#define NS_SCROLL_BAR_WIDTH_DEFAULT [EmacsScroller scrollerWidth]` (src/nsterm.h 1275).
@_silgen_name("NS_SCROLL_BAR_WIDTH_DEFAULT") func NS_scroll_bar_width_default() -> Int32

/// C 2546-2563 (non-static): terms implement this instead of `x-get-resource` directly.
@_cdecl("ns_get_string_resource")
func ns_get_string_resource(_ rdb: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>,
                            _ class_: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>? {
  /* remove appname prefix; TODO: allow for !="Emacs" */
  var res: UnsafeMutablePointer<CChar>?
  let toCheck = class_.advanced(by: strncmp(class_, "Emacs.", 6) == 0 ? 6 : 0)

  check_window_system(nil)

  if inhibit_x_resources {
    /* --quick was passed, so this is a no-op.  */
    return nil
  }

  res = ns_get_defaults_value(toCheck)
  /* C 2560-2562: `!res ? NULL : !c_strncasecmp (res, "YES", 3) ? "true" :
     !c_strncasecmp (res, "NO", 2) ? "false" : res` - the two literals C returns live in static
     storage, so the port keeps one of each.  */
  if res == nil { return nil }
  if c_strncasecmp(res, "YES", 3) == 0 { return c_true_string }
  if c_strncasecmp(res, "NO", 2) == 0 { return c_false_string }
  return res
}

/// C 2560-2561's `"true"`/`"false"` literals, kept in static storage so the returned pointer
/// stays valid (the `empty_c_string` shape of docs/ports/nsmenu.swift).
fileprivate let c_true_string: UnsafeMutablePointer<CChar> = strdup("true")!
fileprivate let c_false_string: UnsafeMutablePointer<CChar> = strdup("false")!

/// C 2574-2608: `DEFUN ("system-move-file-to-trash", …)` (the `NS_IMPL_COCOA` +
/// `MAC_OS_X_VERSION_MAX_ALLOWED >= 1080` arm).
@_cdecl("Fsystem_move_file_to_trash")
func Fsystem_move_file_to_trash(_ filenameIn: Lisp_Object) -> Lisp_Object {
  var filename = filenameIn
  var handler: Lisp_Object
  var operation: Lisp_Object

  operation = Qdelete_file()
  if !NILP(Ffile_directory_p(filename)) && NILP(Ffile_symlink_p(filename)) {
    operation = Qdelete_directory()
    filename = Fdirectory_file_name(filename)
  }

  /* Must have fully qualified file names for moving files to Trash. */
  filename = Fexpand_file_name(filename, Qnil())

  handler = Ffind_file_name_handler(filename, operation)
  if !NILP(handler) {
    return calln([handler, operation, filename])
  } else {
    let fm = FileManager.default
    var result = false
    let fileURL = URL(fileURLWithPath: stringWithLispString(filename)! as String,
                      isDirectory: !NILP(Ffile_directory_p(filename)))
    if fm.responds(to: #selector(FileManager.trashItem(at:resultingItemURL:))) {
      result = (try? fm.trashItem(at: fileURL, resultingItemURL: nil)) != nil
    }

    if !result {
      report_file_error("Removing old name", list1(filename))
    }
  }
  return Qnil()
}
fileprivate let lname_system_move_file_to_trash: StaticString = "system-move-file-to-trash"
fileprivate var Ssystem_move_file_to_trash: Aligned_Lisp_Subr =
  make_aligned_subr(lname_system_move_file_to_trash, 1, 1,
                    unsafeBitCast(Fsystem_move_file_to_trash as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Move file or directory named FILENAME to the recycle bin.  ", args: "filename")

// ==========================================================================
// C 2612-3227: the color/display group, the monitor attribute list, `compute_tip_xy`, the tip
// frame's creation and `x_hide_tip`.
// ==========================================================================

/// C 2612-2620: `DEFUN ("xw-color-defined-p", …)`.
@_cdecl("Fxw_color_defined_p")
func Fxw_color_defined_p(_ color: Lisp_Object, _ frame: Lisp_Object) -> Lisp_Object {
  var col: NSColor? = nil
  check_window_system(nil)
  return ns_lisp_to_color(color, &col) != 0 ? Qnil() : Qt()
}
fileprivate let lname_xw_color_defined_p: StaticString = "xw-color-defined-p"
fileprivate var Sxw_color_defined_p: Aligned_Lisp_Subr =
  make_aligned_subr(lname_xw_color_defined_p, 1, 2,
                    unsafeBitCast(Fxw_color_defined_p as @convention(c) (Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "color frame")

/// C 2622-2645: `DEFUN ("xw-color-values", …)`.
@_cdecl("Fxw_color_values")
func Fxw_color_values(_ color: Lisp_Object, _ frame: Lisp_Object) -> Lisp_Object {
  var col: NSColor? = nil
  var red: EmacsCGFloat = 0, green: EmacsCGFloat = 0
  var blue: EmacsCGFloat = 0, alpha: EmacsCGFloat = 0

  check_window_system(nil)
  CHECK_STRING(color)

  block_input()
  if ns_lisp_to_color(color, &col) != 0 {
    unblock_input()
    return Qnil()
  }

  colorUsingDefaultColorSpace(col!).getRed(&red, green: &green, blue: &blue, alpha: &alpha)
  unblock_input()
  return list3i(lrint(Double(red * 65535)), lrint(Double(green * 65535)),
                lrint(Double(blue * 65535)))
}
fileprivate let lname_xw_color_values: StaticString = "xw-color-values"
fileprivate var Sxw_color_values: Aligned_Lisp_Subr =
  make_aligned_subr(lname_xw_color_values, 1, 2,
                    unsafeBitCast(Fxw_color_values as @convention(c) (Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "color frame")

/// C 2647-2662: `DEFUN ("xw-display-color-p", …)`.
@_cdecl("Fxw_display_color_p")
func Fxw_display_color_p(_ terminal: Lisp_Object) -> Lisp_Object {
  var depth: NSWindow.Depth
  var colorSpace: NSColorSpaceName?

  _ = check_ns_display_info(terminal)
  depth = NSScreen.screens[0].depth
  colorSpace = NSColorSpaceFromDepth(depth)

  return (colorSpace == NSColorSpaceName.deviceWhite
          || colorSpace == NSColorSpaceName.calibratedWhite) ? Qnil() : Qt()
}
fileprivate let lname_xw_display_color_p: StaticString = "xw-display-color-p"
fileprivate var Sxw_display_color_p: Aligned_Lisp_Subr =
  make_aligned_subr(lname_xw_display_color_p, 0, 1,
                    unsafeBitCast(Fxw_display_color_p as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2664-2676: `DEFUN ("x-display-grayscale-p", …)`.
@_cdecl("Fx_display_grayscale_p")
func Fx_display_grayscale_p(_ terminal: Lisp_Object) -> Lisp_Object {
  var depth: NSWindow.Depth

  _ = check_ns_display_info(terminal)
  depth = NSScreen.screens[0].depth

  return NSBitsPerPixelFromDepth(depth) > 1 ? Qt() : Qnil()
}
fileprivate let lname_x_display_grayscale_p: StaticString = "x-display-grayscale-p"
fileprivate var Sx_display_grayscale_p: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_grayscale_p, 0, 1,
                    unsafeBitCast(Fx_display_grayscale_p as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2678-2687: `DEFUN ("x-display-pixel-width", …)`.
@_cdecl("Fx_display_pixel_width")
func Fx_display_pixel_width(_ terminal: Lisp_Object) -> Lisp_Object {
  let dpyinfo = check_ns_display_info(terminal)

  return make_fixnum(Int(ns_display_pixel_width(dpyinfo)))
}
fileprivate let lname_x_display_pixel_width: StaticString = "x-display-pixel-width"
fileprivate var Sx_display_pixel_width: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_pixel_width, 0, 1,
                    unsafeBitCast(Fx_display_pixel_width as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2689-2698: `DEFUN ("x-display-pixel-height", …)`.
@_cdecl("Fx_display_pixel_height")
func Fx_display_pixel_height(_ terminal: Lisp_Object) -> Lisp_Object {
  let dpyinfo = check_ns_display_info(terminal)

  return make_fixnum(Int(ns_display_pixel_height(dpyinfo)))
}
fileprivate let lname_x_display_pixel_height: StaticString = "x-display-pixel-height"
fileprivate var Sx_display_pixel_height: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_pixel_height, 0, 1,
                    unsafeBitCast(Fx_display_pixel_height as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2700-2734 (static).
func ns_make_monitor_attribute_list(_ monitors: UnsafeMutablePointer<MonitorInfo>?,
                                    _ n_monitors: Int32, _ primary_monitor: Int32,
                                    _ source: UnsafePointer<CChar>?) -> Lisp_Object {
  let monitor_frames = make_nil_vector(Int(n_monitors))
  var frame: Lisp_Object = Qnil()
  let screens = NSScreen.screens
  var i: Int32

  /* C 2709: `FOR_EACH_FRAME (rest, frame)` - the macro's own spelling (src/frame.h
     1404-1408).  */
  var rest: Lisp_Object = Vframe_list
  while CONSP(rest) {
    frame = XCAR(rest)
    let f = XFRAME(frame)

    if FRAME_NS_P(f) {
      let view = FRAME_NS_VIEW(f)
      let screen = view?.window?.screen

      i = -1
      var k = 0
      while i == -1 && k < screens.count {
        if screens[k] == screen {
          i = Int32(k)
        }
        k += 1
      }

      if i > -1 {
        ASET(monitor_frames, Int(i), Fcons(frame, AREF(monitor_frames, Int(i))))
      }
    }
    rest = XCDR(rest)
  }

  return make_monitor_attribute_list(monitors, n_monitors, primary_monitor,
                                      monitor_frames, source)
}

/// C 2736-2846: `DEFUN ("ns-display-monitor-attributes-list", …)`.
@_cdecl("Fns_display_monitor_attributes_list")
func Fns_display_monitor_attributes_list(_ terminal: Lisp_Object) -> Lisp_Object {
  let term = decode_live_terminal(terminal)
  var screens: [NSScreen]
  var n_monitors: Int
  var monitors: UnsafeMutablePointer<MonitorInfo>?
  var attributes_list: Lisp_Object = Qnil()
  var primary_display_height: CGFloat = 0

  if terminal_type(term) != output_ns() {
    return Qnil()
  }

  screens = NSScreen.screens
  n_monitors = screens.count
  if n_monitors == 0 {
    return Qnil()
  }

  monitors = xcalloc(n_monitors, MemoryLayout<MonitorInfo>.stride)?
    .assumingMemoryBound(to: MonitorInfo.self)

  for ii in 0 ..< screens.count {
    let s = screens[ii]
    let m = monitors!.advanced(by: ii)
    let fr = s.frame
    let vfr = s.visibleFrame

    /* The `NS_IMPL_COCOA` arm reads the screen's device description for the display id.  */
    let dict = s.deviceDescription
    let nid = dict[NSDeviceDescriptionKey("NSScreenNumber")] as? NSNumber
    let did = nid?.uint32Value ?? 0

    /* The primary display is always the first in the array.  */
    if ii == 0 {
      primary_display_height = fr.size.height
    }

    /* Flip y coordinate as NS screen coordinates originate from
       the bottom.  */

    m.pointee.geom.x = Int32(fr.origin.x)
    m.pointee.geom.y = Int32(primary_display_height - fr.maxY)
    m.pointee.geom.width = UInt32(fr.size.width)
    m.pointee.geom.height = UInt32(fr.size.height)

    /* The work area excludes the menu bar and the dock.  */
    m.pointee.work.x = Int32(vfr.origin.x)
    m.pointee.work.y = Int32(primary_display_height - vfr.maxY)
    m.pointee.work.width = UInt32(vfr.size.width)
    m.pointee.work.height = UInt32(vfr.size.height)

    m.pointee.name = nil
    if s.responds(to: #selector(getter: NSScreen.localizedName)) {
      let name: NSString? = s.localizedName as NSString?
      if name != nil {
        m.pointee.name = xmalloc(name!.lengthOfBytes(using: String.Encoding.utf8.rawValue) + 1)?
          .assumingMemoryBound(to: CChar.self)
        strcpy(m.pointee.name, name!.utf8String)
      }
    }
    /* If necessary, synthesize a name of the following form:
        %dx%d@%d,%d width height x y. */
    if m.pointee.name == nil {
      /* C 2802-2807 fills a 25-byte buffer ("sufficient for 12345x78901@34567,90123") with
         `snprintf` and copies it with `xstrdup`; `snprintf` is variadic and unavailable in
         Swift, so the port builds the same formatted string and hands it to `xstrdup`.  */
      let synthesized = String(format: "%ux%u@%d,%d",
                               m.pointee.work.width, m.pointee.work.height,
                               m.pointee.work.x, m.pointee.work.y)
      m.pointee.name = xstrdup(synthesized)
    }

    do {
      let mms = CGDisplayScreenSize(did)
      m.pointee.mm_width = Int32(mms.width)
      m.pointee.mm_height = Int32(mms.height)
    }
  }

  /* Primary monitor is always ordered first for NS.  */
  attributes_list = ns_make_monitor_attribute_list(monitors, Int32(n_monitors), 0, "NS")

  free_monitors(monitors, Int32(n_monitors))
  return attributes_list
}
fileprivate let lname_ns_display_monitor_attributes_list: StaticString =
  "ns-display-monitor-attributes-list"
fileprivate var Sns_display_monitor_attributes_list: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_display_monitor_attributes_list, 0, 1,
                    unsafeBitCast(Fns_display_monitor_attributes_list as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Return a list of physical monitor attributes on the X display TERMINAL.\n\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n\nIn addition to the standard attribute keys listed in\n`display-monitor-attributes-list', the following keys are contained in\nthe attributes:\n\n source -- String describing the source from which multi-monitor\n\t   information is obtained, \"NS\" is always the source.\"\n\nInternal use only, use `display-monitor-attributes-list' instead.  ", args: "terminal")

/// C 2848-2857: `DEFUN ("x-display-planes", …)`.
@_cdecl("Fx_display_planes")
func Fx_display_planes(_ terminal: Lisp_Object) -> Lisp_Object {
  _ = check_ns_display_info(terminal)
  return make_fixnum(Int(NSBitsPerPixelFromDepth(NSScreen.screens[0].depth)))
}
fileprivate let lname_x_display_planes: StaticString = "x-display-planes"
fileprivate var Sx_display_planes: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_planes, 0, 1,
                    unsafeBitCast(Fx_display_planes as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2859-2867: `DEFUN ("x-display-color-cells", …)`.
@_cdecl("Fx_display_color_cells")
func Fx_display_color_cells(_ terminal: Lisp_Object) -> Lisp_Object {
  let dpyinfo = check_ns_display_info(terminal)
  /* We force 24+ bit depths to 24-bit to prevent an overflow.  */
  return make_fixnum(1 << Int(min(dpyinfo!.pointee.n_planes, 24)))
}
fileprivate let lname_x_display_color_cells: StaticString = "x-display-color-cells"
fileprivate var Sx_display_color_cells: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_display_color_cells, 0, 1,
                    unsafeBitCast(Fx_display_color_cells as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "terminal")

/// C 2869-2940 (static).
func compute_tip_xy(_ f: UnsafeMutableRawPointer?, _ parms: Lisp_Object, _ dx: Lisp_Object,
                    _ dy: Lisp_Object, _ width: Int32, _ height: Int32,
                    _ root_x: UnsafeMutablePointer<Int32>, _ root_y: UnsafeMutablePointer<Int32>) {
  var left: Lisp_Object
  var top: Lisp_Object
  var right: Lisp_Object
  var bottom: Lisp_Object
  var pt = NSPoint()
  /* C 2878: `NSScreen *screen;` - the loop assigns it every iteration, so C's value when no
     screen matches is the *last* one; the port starts from that same element.  */
  var screen = NSScreen.screens[NSScreen.screens.count - 1]

  /* Start with user-specified or mouse position.  */
  left = Fcdr(Fassq(Qleft(), parms))
  top = Fcdr(Fassq(Qtop(), parms))
  right = Fcdr(Fassq(Qright(), parms))
  bottom = Fcdr(Fassq(Qbottom(), parms))

  if (!FIXNUMP(left) && !FIXNUMP(right)) || (!FIXNUMP(top) && !FIXNUMP(bottom)) {
    pt = NSEvent.mouseLocation
  } else {
    /* Absolute coordinates.  */
    pt.x = CGFloat(FIXNUMP(left) ? XFIXNUM(left) : XFIXNUM(right))
    pt.y = CGFloat(Int(ns_display_pixel_height(FRAME_DISPLAY_INFO(f)))
                   - (FIXNUMP(top) ? XFIXNUM(top) : XFIXNUM(bottom))
                   - Int(height))
  }

  /* Find the screen that pt is on.  */
  for s in NSScreen.screens {
    if pt.x >= s.frame.origin.x
        && pt.x < s.frame.origin.x + s.frame.size.width
        && pt.y >= s.frame.origin.y
        && pt.y < s.frame.origin.y + s.frame.size.height {
      screen = s
      break
    }
  }

  /* We could use this instead of the if above:

         if (CGRectContainsPoint ([screen frame], pt))

     which would be neater, but it causes problems building on old
     versions of macOS and in GNUstep.  */

  /* Ensure in bounds.  (Note, screen origin = lower left.) */
  if FIXNUMP(left) || FIXNUMP(right) {
    root_x.pointee = Int32(pt.x)
  } else if pt.x + CGFloat(XFIXNUM(dx)) <= screen.frame.origin.x {
    root_x.pointee = Int32(screen.frame.origin.x)
  } else if pt.x + CGFloat(XFIXNUM(dx)) + CGFloat(width)
              <= screen.frame.origin.x + screen.frame.size.width {
    /* It fits to the right of the pointer.  */
    root_x.pointee = Int32(pt.x + CGFloat(XFIXNUM(dx)))
  } else if CGFloat(width) + CGFloat(XFIXNUM(dx)) <= pt.x {
    /* It fits to the left of the pointer.  */
    root_x.pointee = Int32(pt.x - CGFloat(width) - CGFloat(XFIXNUM(dx)))
  } else {
    /* Put it left justified on the screen -- it ought to fit that way.  */
    root_x.pointee = Int32(screen.frame.origin.x)
  }

  if FIXNUMP(top) || FIXNUMP(bottom) {
    root_y.pointee = Int32(pt.y)
  } else if pt.y - CGFloat(XFIXNUM(dy)) - CGFloat(height) >= screen.frame.origin.y {
    /* It fits below the pointer.  */
    root_y.pointee = Int32(pt.y - CGFloat(height) - CGFloat(XFIXNUM(dy)))
  } else if pt.y + CGFloat(XFIXNUM(dy)) + CGFloat(height)
              <= screen.frame.origin.y + screen.frame.size.height {
    /* It fits above the pointer.  */
    root_y.pointee = Int32(pt.y + CGFloat(XFIXNUM(dy)))
  } else {
    /* Put it on the top.  */
    root_y.pointee = Int32(screen.frame.origin.y + screen.frame.size.height - CGFloat(height))
  }
}

/// C 2942-2948 (static).
func unwind_create_tip_frame(_ frame: Lisp_Object) {
  var deleted: Lisp_Object

  deleted = unwind_create_frame(frame)
  if EQ(deleted, Qt()) {
    tip_frame = Qnil()
  }
}

/// C 2950-3163 (static).  Create a frame for a tooltip on the display described by DPYINFO.
func ns_create_tip_frame(_ dpyinfo: UnsafeMutablePointer<ns_display_info>?,
                         _ parmsIn: Lisp_Object) -> Lisp_Object {
  var parms = parmsIn
  var f: UnsafeMutableRawPointer?
  var frame: Lisp_Object = Qnil()
  var name: Lisp_Object
  let count = SPECPDL_INDEX()
  let face_change_before = face_change

  if terminal_name(dpyinfo!.pointee.terminal) == nil {
    error("Terminal is not live, can't create new frames on it")
  }

  parms = Fcopy_alist(parms)

  /* Get the name of the frame to use for resource lookup.  */
  name = gui_display_get_arg(dpyinfo, parms, Qname(), "name", "Name", RES_TYPE_STRING())
  if !STRINGP(name) && !EQ(name, Qunbound()) && !NILP(name) {
    error("Invalid frame name--not a string or nil")
  }

  frame = Qnil()
  f = make_frame(false)
  frame_set_wants_modeline(f, false)
  withUnsafeMutablePointer(to: &frame) { XSETFRAME($0, f) }
  record_unwind_protect({ frame in unwind_create_tip_frame(frame) }, frame)

  frame_set_terminal(f, dpyinfo!.pointee.terminal)

  frame_set_output_method(f, output_ns())
  frame_alloc_ns_output(f)
  frame_set_tooltip(f, true)

  frame_set_fontset(f, -1)
  frame_set_display_info(f, dpyinfo)

  block_input()
  /* C 3001-3005: the `NS_IMPL_COCOA` arm; the `#else` arm registers the nsfont driver.  */
  mac_register_font_driver(f)
  unblock_input()

  _ = gui_default_parameter(f, parms, Qfont_backend(), Qnil(), "fontBackend", "FontBackend",
                            RES_TYPE_STRING())

  do {
    /* use for default font name */
    let font = NSFont.userFixedPitchFont(ofSize: -1.0)!  /* default */
    _ = gui_default_parameter(f, parms, Qfontsize(),
                              make_fixnum(0 /* (int)[font pointSize] */),
                              "fontSize", "FontSize", RES_TYPE_NUMBER())
    /* Remove ' Regular', not handled by backends.  */
    let fontname = xstrdup(font.displayName!)
    let len = Int(strlen(fontname!))
    if len > 8 && strcmp(fontname! + len - 8, " Regular") == 0 {
      fontname![len - 8] = 0
    }
    _ = gui_default_parameter(f, parms, Qfont(), build_string(fontname!),
                              "font", "Font", RES_TYPE_STRING())
    xfree(fontname)
  }

  _ = gui_default_parameter(f, parms, Qborder_width(), make_fixnum(0),
                            "borderWidth", "BorderWidth", RES_TYPE_NUMBER())

  /* This defaults to 1 in order to match xterm.  We recognize either
     internalBorderWidth or internalBorder (which is what xterm calls
     it).  */
  if NILP(Fassq(Qinternal_border_width(), parms)) {
    let value = gui_display_get_arg(dpyinfo, parms, Qinternal_border_width(),
                                    "internalBorder", "internalBorder", RES_TYPE_NUMBER())
    if !EQ(value, Qunbound()) {
      parms = Fcons(Fcons(Qinternal_border_width(), value), parms)
    }
  }

  _ = gui_default_parameter(f, parms, Qinternal_border_width(), make_fixnum(1),
                            "internalBorderWidth", "internalBorderWidth", RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qright_divider_width(), make_fixnum(0),
                            nil, nil, RES_TYPE_NUMBER())
  _ = gui_default_parameter(f, parms, Qbottom_divider_width(), make_fixnum(0),
                            nil, nil, RES_TYPE_NUMBER())

  /* Also do the stuff which must be set before the window exists.  */
  _ = gui_default_parameter(f, parms, Qforeground_color(), build_string("black"),
                            "foreground", "Foreground", RES_TYPE_STRING())
  _ = gui_default_parameter(f, parms, Qbackground_color(), build_string("white"),
                            "background", "Background", RES_TYPE_STRING())
  _ = gui_default_parameter(f, parms, Qmouse_color(), build_string("black"),
                            "pointerColor", "Foreground", RES_TYPE_STRING())
  _ = gui_default_parameter(f, parms, Qcursor_color(), build_string("black"),
                            "cursorColor", "Foreground", RES_TYPE_STRING())
  _ = gui_default_parameter(f, parms, Qborder_color(), build_string("black"),
                            "borderColor", "BorderColor", RES_TYPE_STRING())
  _ = gui_default_parameter(f, parms, Qno_special_glyphs(), Qnil(), nil, nil,
                            RES_TYPE_BOOLEAN())

  /* Init faces before gui_default_parameter is called for the
     scroll-bar-width parameter because otherwise we end up in
     init_iterator with a null face cache, which should not happen.  */
  init_frame_faces(f)

  ns_output_set_parent_desc(f, FRAME_DISPLAY_INFO(f).pointee.root_window)

  _ = gui_default_parameter(f, parms, Qinhibit_double_buffering(), Qnil(),
                            "inhibitDoubleBuffering", "InhibitDoubleBuffering",
                            RES_TYPE_BOOLEAN())

  _ = gui_figure_window_size(f, parms, false, false)

  block_input()
  _ = EmacsView_initFrameFromEmacs(f)
  ns_icon(f, parms)
  unblock_input()

  _ = gui_default_parameter(f, parms, Qauto_raise(), Qnil(),
                            "autoRaise", "AutoRaiseLower", RES_TYPE_BOOLEAN())
  _ = gui_default_parameter(f, parms, Qauto_lower(), Qnil(),
                            "autoLower", "AutoRaiseLower", RES_TYPE_BOOLEAN())
  _ = gui_default_parameter(f, parms, Qcursor_type(), Qbox(),
                            "cursorType", "CursorType", RES_TYPE_SYMBOL())
  _ = gui_default_parameter(f, parms, Qalpha(), Qnil(),
                            "alpha", "Alpha", RES_TYPE_NUMBER())

  /* Add `tooltip' frame parameter's default value. */
  if NILP(Fframe_parameter(frame, Qtooltip())) {
    let arg = AUTO_FRAME_ARG(Qtooltip(), Qt())
    _ = Fmodify_frame_parameters(frame, arg)
  }

  /* FIXME - can this be done in a similar way to normal frames?
     https://lists.gnu.org/r/emacs-devel/2007-10/msg00641.html */

  /* Set the `display-type' frame parameter before setting up faces. */
  do {
    let disptype = Qcolor()

    if NILP(Fframe_parameter(frame, Qdisplay_type())) {
      let arg = AUTO_FRAME_ARG(Qdisplay_type(), disptype)
      _ = Fmodify_frame_parameters(frame, arg)
    }
  }

  /* Set up faces after all frame parameters are known.  This call
     also merges in face attributes specified for new frames.

     Frame parameters may be changed if .Xdefaults contains
     specifications for the default font.  For example, if there is an
     `Emacs.default.attributeBackground: pink', the `background-color'
     attribute of the frame gets set, which let's the internal border
     of the tooltip frame appear in pink.  Prevent this.  */
  do {
    let bg = Fframe_parameter(frame, Qbackground_color())

    _ = calln([Qface_set_after_frame_default(), frame, Qnil()])

    if !EQ(bg, Fframe_parameter(frame, Qbackground_color())) {
      let arg = AUTO_FRAME_ARG(Qbackground_color(), bg)
      _ = Fmodify_frame_parameters(frame, arg)
    }
  }

  frame_set_no_split(f, true)

  /* Now that the frame will be official, it counts as a reference to
     its display and terminal.  */
  terminal_increment_reference_count(dpyinfo!.pointee.terminal)

  /* It is now ok to make the frame official even if we get an error
     below.  And the frame needs to be on Vframe_list or making it
     visible won't work.  */
  Vframe_list = Fcons(frame, Vframe_list)
  frame_set_can_set_window_size(f, true)
  adjust_frame_size(f, FRAME_TEXT_WIDTH(f), FRAME_TEXT_HEIGHT(f), 0, true, Qtip_frame())

  /* Setting attributes of faces of the tooltip frame from resources
     and similar will set face_change, which leads to the clearing of
     all current matrices.  Since this isn't necessary here, avoid it
     by resetting face_change to the value it had before we created
     the tip frame.  */
  face_change = face_change_before

  /* Discard the unwind_protect.  */
  return unbind_to(count, frame)
}

/// C 3165-3227 (static).
func x_hide_tip(_ delete: Bool) -> Lisp_Object {
  if !NILP(tip_timer) {
    _ = calln([Qcancel_timer(), tip_timer])
    tip_timer = Qnil()
  }

  if !(ns_tooltip == nil || !EmacsTooltip_isActive(ns_tooltip)) {
    EmacsTooltip_hide(ns_tooltip)
    tip_last_frame = Qnil()
    return Qt()
  }

  if (NILP(tip_last_frame) && NILP(tip_frame))
      || (!use_system_tooltips
          && !delete
          && !NILP(tip_frame)
          && FRAME_LIVE_P(XFRAME(tip_frame))
          && !FRAME_VISIBLE_P(XFRAME(tip_frame))) {
    /* Either there's no tooltip to hide or it's an already invisible
       Emacs tooltip and we don't want to change its type.  Return
       quickly.  */
    return Qnil()
  } else {
    var count: specpdl_ref
    var was_open: Lisp_Object = Qnil()

    count = SPECPDL_INDEX()
    specbind(Qinhibit_redisplay(), Qt())
    specbind(Qinhibit_quit(), Qt())

    /* Now look whether there's an Emacs tip around.  */
    if !NILP(tip_frame) {
      let f = XFRAME(tip_frame)

      if FRAME_LIVE_P(f) {
        if delete || use_system_tooltips {
          /* Delete the Emacs tooltip frame when DELETE is true
             or we change the tooltip type from an Emacs one to
             a GTK+ system one.  */
          _ = delete_frame(tip_frame, Qnil())
          tip_frame = Qnil()
        } else {
          ns_make_frame_invisible(f)
        }

        was_open = Qt()
      } else {
        tip_frame = Qnil()
      }
    } else {
      tip_frame = Qnil()
    }

    return unbind_to(count, was_open)
  }
}

// ==========================================================================
// C 3229-3800: `x-show-tip`/`x-hide-tip`, `frame_geometry`, the geometry/edge DEFUNs, the
// absolute mouse position pair, the character palette, the accessibility query and `ns-badge`.
// ==========================================================================

/// C 3229-3540: `DEFUN ("x-show-tip", …)`.  C's `goto start_timer` (C 3475, 3534) is the
/// flag on the shared tail, since Swift has no `goto`.
@_cdecl("Fx_show_tip")
func Fx_show_tip(_ string: Lisp_Object, _ frame: Lisp_Object, _ parmsIn: Lisp_Object,
                 _ timeoutIn: Lisp_Object, _ dxIn: Lisp_Object,
                 _ dyIn: Lisp_Object) -> Lisp_Object {
  var parms = parmsIn
  var timeout = timeoutIn
  var dx = dxIn
  var dy = dyIn
  var root_x: Int32 = 0
  var root_y: Int32 = 0
  let count = SPECPDL_INDEX()
  var f: UnsafeMutableRawPointer?
  var tip_f: UnsafeMutableRawPointer?
  var w: UnsafeMutableRawPointer?
  var old_buffer: UnsafeMutableRawPointer?
  var pos = text_pos(charpos: 0, bytepos: 0)
  var width: Int32 = 0
  var height: Int32 = 0
  let old_windows_or_buffers_changed = windows_or_buffers_changed
  var count_1: specpdl_ref
  var window: Lisp_Object
  var size: Lisp_Object
  var tip_buf: Lisp_Object
  var str: UnsafeMutablePointer<CChar>
  var nswindow: NSWindow?
  var start_timer = false

  /* C 3247: `AUTO_STRING (tip, " *tip*")` (src/lisp.h 5760-5766) - the macro makes a Lisp
     string out of the literal; the port's value is the same (`build_string`).  */
  let tip = build_string(" *tip*")

  specbind(Qinhibit_redisplay(), Qt())

  CHECK_STRING(string)
  str = SSDATA(string)
  f = decode_window_system_frame(frame)
  if NILP(timeout) {
    timeout = Vx_show_tooltip_timeout
  }
  CHECK_FIXNAT(timeout)

  if NILP(dx) {
    dx = make_fixnum(5)
  } else {
    CHECK_FIXNUM(dx)
  }

  if NILP(dy) {
    dy = make_fixnum(-10)
  } else {
    CHECK_FIXNUM(dy)
  }

  tip_dx = dx
  tip_dy = dy

  if use_system_tooltips {
    var size: NSSize
    var color: NSColor? = nil
    var t: Lisp_Object

    block_input()
    if ns_tooltip == nil {
      ns_tooltip = EmacsTooltip_new()
    } else {
      _ = Fx_hide_tip()
    }

    t = gui_display_get_arg(nil, parms, Qbackground_color(), nil, nil, RES_TYPE_STRING())
    if ns_lisp_to_color(t, &color) == 0 {
      EmacsTooltip_setBackgroundColor(ns_tooltip, color)
    }

    t = gui_display_get_arg(nil, parms, Qforeground_color(), nil, nil, RES_TYPE_STRING())
    if ns_lisp_to_color(t, &color) == 0 {
      EmacsTooltip_setForegroundColor(ns_tooltip, color)
    }

    EmacsTooltip_setText(ns_tooltip, str)
    size = EmacsTooltip_frame(ns_tooltip).size

    /* Move the tooltip window where the mouse pointer is.  Resize and
       show it.  */
    compute_tip_xy(f, parms, dx, dy, Int32(size.width), Int32(size.height), &root_x, &root_y)

    EmacsTooltip_showAtX_Y_for(ns_tooltip, root_x, root_y, Int32(XFIXNUM(timeout)))
    unblock_input()
  } else {
    if !NILP(tip_frame) && FRAME_LIVE_P(XFRAME(tip_frame)) {
      if FRAME_VISIBLE_P(XFRAME(tip_frame))
          && !NILP(Fequal_including_properties(tip_last_string, string))
          && !NILP(Fequal(tip_last_parms, parms)) {
        /* Only DX and DY have changed.  */
        tip_f = XFRAME(tip_frame)
        if !NILP(tip_timer) {
          _ = calln([Qcancel_timer(), tip_timer])
          tip_timer = Qnil()
        }

        nswindow = FRAME_NS_VIEW(tip_f)?.window

        block_input()
        compute_tip_xy(tip_f, parms, dx, dy, FRAME_PIXEL_WIDTH(tip_f),
                       FRAME_PIXEL_HEIGHT(tip_f), &root_x, &root_y)
        nswindow?.setFrame(NSRect(x: CGFloat(root_x), y: CGFloat(root_y),
                                  width: CGFloat(FRAME_PIXEL_WIDTH(tip_f)),
                                  height: CGFloat(FRAME_PIXEL_HEIGHT(tip_f))),
                           display: true)
        nswindow?.level = .popUpMenu
        nswindow?.orderFront(NSApp)
        nswindow?.display()

        SET_FRAME_VISIBLE(tip_f, true)
        unblock_input()

        start_timer = true
      } else if tooltip_reuse_hidden_frame && EQ(frame, tip_last_frame) {
        var delete = false
        var elt: Lisp_Object
        var parm: Lisp_Object
        var last: Lisp_Object

        /* Check if every parameter in PARMS has the same value in
           tip_last_parms.  This may destruct tip_last_parms which,
           however, will be recreated below.  */
        var tail: Lisp_Object = parms
        while CONSP(tail) {
          elt = XCAR(tail)
          parm = Fcar(elt)
          /* The left, top, right and bottom parameters are handled
             by compute_tip_xy so they can be ignored here.  */
          if !EQ(parm, Qleft()) && !EQ(parm, Qtop())
              && !EQ(parm, Qright()) && !EQ(parm, Qbottom()) {
            last = Fassq(parm, tip_last_parms)
            if NILP(Fequal(Fcdr(elt), Fcdr(last))) {
              /* We lost, delete the old tooltip.  */
              delete = true
              break
            } else {
              tip_last_parms = calln([Qassq_delete_all(), parm, tip_last_parms])
            }
          } else {
            tip_last_parms = calln([Qassq_delete_all(), parm, tip_last_parms])
          }
          tail = XCDR(tail)
        }

        /* Now check if every parameter in what is left of
           tip_last_parms with a non-nil value has an association in
           PARMS.  */
        var tail2: Lisp_Object = tip_last_parms
        while CONSP(tail2) {
          elt = XCAR(tail2)
          parm = Fcar(elt)
          if !EQ(parm, Qleft()) && !EQ(parm, Qtop()) && !EQ(parm, Qright())
              && !EQ(parm, Qbottom()) && !NILP(Fcdr(elt)) {
            /* We lost, delete the old tooltip.  */
            delete = true
            break
          }
          tail2 = XCDR(tail2)
        }

        _ = x_hide_tip(delete)
      } else {
        _ = x_hide_tip(true)
      }
    } else {
      _ = x_hide_tip(true)
    }

    tip_last_frame = frame
    tip_last_string = string
    tip_last_parms = parms

    if !start_timer {
      if NILP(tip_frame) || !FRAME_LIVE_P(XFRAME(tip_frame)) {
        /* Add default values to frame parameters.  */
        if NILP(Fassq(Qname(), parms)) {
          parms = Fcons(Fcons(Qname(), build_string("tooltip")), parms)
        }
        if NILP(Fassq(Qinternal_border_width(), parms)) {
          parms = Fcons(Fcons(Qinternal_border_width(), make_fixnum(3)), parms)
        }
        if NILP(Fassq(Qborder_width(), parms)) {
          parms = Fcons(Fcons(Qborder_width(), make_fixnum(1)), parms)
        }
        if NILP(Fassq(Qborder_color(), parms)) {
          parms = Fcons(Fcons(Qborder_color(), build_string("lightyellow")), parms)
        }
        if NILP(Fassq(Qbackground_color(), parms)) {
          parms = Fcons(Fcons(Qbackground_color(), build_string("lightyellow")), parms)
        }

        /* Create a frame for the tooltip, and record it in the global
           variable tip_frame.  */
        tip_frame = ns_create_tip_frame(FRAME_DISPLAY_INFO(f), parms)
        if NILP(tip_frame) {
          /* Creating the tip frame failed.  */
          return unbind_to(count, Qnil())
        }
      }

      tip_f = XFRAME(tip_frame)
      window = FRAME_ROOT_WINDOW(tip_f)
      tip_buf = Fget_buffer_create(tip, Qnil())
      /* We will mark the tip window a "pseudo-window" below, and such
         windows cannot have display margins.  */
      bset_left_margin_cols(XBUFFER(tip_buf), make_fixnum(0))
      bset_right_margin_cols(XBUFFER(tip_buf), make_fixnum(0))
      set_window_buffer(window, tip_buf, false, false)
      w = XWINDOW(window)
      window_set_pseudo_window_p(w, true)
      /* Try to avoid that `other-window' select us (Bug#47207).  */
      _ = Fset_window_parameter(window, Qno_other_window(), Qt())

      /* Set up the frame's root window.  Note: The following code does not
         try to size the window or its frame correctly.  Its only purpose is
         to make the subsequent text size calculations work.  The right
         sizes should get installed when the toolkit gets back to us.  */
      window_set_left_col(w, 0)
      window_set_top_line(w, 0)
      window_set_pixel_left(w, 0)
      window_set_pixel_top(w, 0)

      if CONSP(Vx_max_tooltip_size)
          && RANGED_FIXNUMP(1, XCAR(Vx_max_tooltip_size), Int(Int32.max))
          && RANGED_FIXNUMP(1, XCDR(Vx_max_tooltip_size), Int(Int32.max)) {
        window_set_total_cols(w, Int32(XFIXNAT(XCAR(Vx_max_tooltip_size))))
        window_set_total_lines(w, Int32(XFIXNAT(XCDR(Vx_max_tooltip_size))))
      } else {
        window_set_total_cols(w, 80)
        window_set_total_lines(w, 40)
      }

      window_set_pixel_width(w, window_total_cols(w) * FRAME_COLUMN_WIDTH(tip_f))
      window_set_pixel_height(w, window_total_lines(w) * FRAME_LINE_HEIGHT(tip_f))
      frame_set_total_cols(tip_f, window_total_cols(w))
      adjust_frame_glyphs(tip_f)

      /* Insert STRING into root window's buffer and fit the frame to the
         buffer.  */
      count_1 = SPECPDL_INDEX()
      old_buffer = current_buffer()
      set_buffer_internal_1(XBUFFER(window_contents(w)))
      bset_truncate_lines(current_buffer(), Qnil())
      specbind(Qinhibit_read_only(), Qt())
      specbind(Qinhibit_modification_hooks(), Qt())
      specbind(Qinhibit_point_motion_hooks(), Qt())
      _ = Ferase_buffer()
      var s = string
      withUnsafeMutablePointer(to: &s) { _ = Finsert(1, $0) }
      clear_glyph_matrix(window_desired_matrix(w))
      clear_glyph_matrix(window_current_matrix(w))
      pos = text_pos(charpos: current_buffer_begv(), bytepos: current_buffer_begv_byte())
      /* C 3468: `displayed = try_window (window, pos, TRY_WINDOW_IGNORE_FONTS_CHANGE);` - the
         value is read only by the reference's `#ifdef ENABLE_CHECKING` block below, which is
         dead text on this target.  */
      _ = try_window(window, pos, TRY_WINDOW_IGNORE_FONTS_CHANGE())

      /* C 3470-3490's `#ifdef ENABLE_CHECKING` block (the `row`/`end` walk and its `eassert`)
         is the reference's own checking build - dead text here, recorded.  */

      /* Calculate size of tooltip window.  */
      size = Fwindow_text_pixel_size(window, Qnil(), Qnil(), Qnil(),
                                     make_fixnum(Int(window_pixel_height(w))), Qnil(), Qnil())
      /* Add the frame's internal border to calculated size.  */
      width = Int32(XFIXNUM(Fcar(size))) + 2 * FRAME_INTERNAL_BORDER_WIDTH(tip_f)
      height = Int32(XFIXNUM(Fcdr(size))) + 2 * FRAME_INTERNAL_BORDER_WIDTH(tip_f)

      /* Calculate position of tooltip frame.  */
      compute_tip_xy(tip_f, parms, dx, dy, width, height, &root_x, &root_y)

      block_input()
      nswindow = FRAME_NS_VIEW(tip_f)?.window
      nswindow?.setFrame(NSRect(x: CGFloat(root_x), y: CGFloat(root_y),
                                width: CGFloat(width), height: CGFloat(height)),
                         display: true)
      nswindow?.level = .popUpMenu
      nswindow?.orderFront(NSApp)
      nswindow?.display()

      SET_FRAME_VISIBLE(tip_f, true)
      frame_set_pixel_width(tip_f, width)
      frame_set_pixel_height(tip_f, height)
      unblock_input()

      window_set_must_be_updated_p(w, true)
      update_single_window(w)
      flush_frame(tip_f)
      set_buffer_internal_1(old_buffer)
      _ = unbind_to(count_1, Qnil())
      windows_or_buffers_changed = old_windows_or_buffers_changed
    }

    /* C 3534-3536: `start_timer: tip_timer = calln (Qrun_at_time, timeout, Qnil, Qx_hide_tip);`
       - let the tip disappear after timeout seconds.  */
    tip_timer = calln([Qrun_at_time(), timeout, Qnil(), Qx_hide_tip()])
  }

  return unbind_to(count, Qnil())
}
fileprivate let lname_x_show_tip: StaticString = "x-show-tip"
fileprivate var Sx_show_tip: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_show_tip, 1, 6,
                    unsafeBitCast(Fx_show_tip as @convention(c) (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "string frame parms timeout dx dy")

/// C 3541-3549: `DEFUN ("x-hide-tip", …)`.
@_cdecl("Fx_hide_tip")
func Fx_hide_tip() -> Lisp_Object {
  return x_hide_tip(!tooltip_reuse_hidden_frame)
}
fileprivate let lname_x_hide_tip: StaticString = "x-hide-tip"
fileprivate var Sx_hide_tip: Aligned_Lisp_Subr =
  make_aligned_subr(lname_x_hide_tip, 0, 0,
                    unsafeBitCast(Fx_hide_tip as @convention(c) () -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " SKIP: real doc in xfns.c.  ", args: "void")

/// C 3551-3614 (static).  Return geometric attributes of FRAME according to ATTRIBUTES.
func frame_geometry(_ frame: Lisp_Object, _ attribute: Lisp_Object) -> Lisp_Object {
  let f = decode_live_frame(frame)
  let fullscreen_symbol = Fframe_parameter(frame, Qfullscreen())
  let fullscreen = (EQ(fullscreen_symbol, Qfullboth()) || EQ(fullscreen_symbol, Qfullscreen()))
  let border = fullscreen ? 0 : frame_border_width(f)
  let title_height = fullscreen ? 0 : frame_ns_titlebar_height(f)
  let tool_bar_height = frame_toolbar_height(f)
  let native_width = FRAME_PIXEL_WIDTH(f)
  let native_height = FRAME_PIXEL_HEIGHT(f)
  let outer_width = native_width + 2 * border
  let outer_height = native_height + 2 * border + title_height + tool_bar_height
  let native_left = frame_left_pos(f) + border
  let native_top = frame_top_pos(f) + border + title_height
  let native_right = frame_left_pos(f) + outer_width - border
  let native_bottom = frame_top_pos(f) + outer_height - border
  let internal_border_width = FRAME_INTERNAL_BORDER_WIDTH(f)
  let tool_bar_width = tool_bar_height != 0 ? outer_width - 2 * internal_border_width : 0

  /* Construct list.  */
  if EQ(attribute, Qouter_edges()) {
    return list4i(Int(frame_left_pos(f)), Int(frame_top_pos(f)),
                  Int(frame_left_pos(f) + outer_width),
                  Int(frame_top_pos(f) + outer_height))
  } else if EQ(attribute, Qnative_edges()) {
    return list4i(Int(native_left), Int(native_top), Int(native_right), Int(native_bottom))
  } else if EQ(attribute, Qinner_edges()) {
    return list4i(Int(native_left + internal_border_width),
                  Int(native_top + tool_bar_height + internal_border_width),
                  Int(native_right - internal_border_width),
                  Int(native_bottom - internal_border_width))
  } else {
    return
      list([Fcons(Qouter_position(),
                  Fcons(make_fixnum(Int(frame_left_pos(f))), make_fixnum(Int(frame_top_pos(f))))),
            Fcons(Qouter_size(),
                  Fcons(make_fixnum(Int(outer_width)), make_fixnum(Int(outer_height)))),
            Fcons(Qexternal_border_size(),
                  (fullscreen
                   ? Fcons(make_fixnum(0), make_fixnum(0))
                   : Fcons(make_fixnum(Int(border)), make_fixnum(Int(border))))),
            Fcons(Qtitle_bar_size(),
                  Fcons(make_fixnum(0), make_fixnum(Int(title_height)))),
            Fcons(Qmenu_bar_external(), Qnil()),
            Fcons(Qmenu_bar_size(), Fcons(make_fixnum(0), make_fixnum(0))),
            Fcons(Qtool_bar_external(), FRAME_EXTERNAL_TOOL_BAR(f) != 0 ? Qt() : Qnil()),
            Fcons(Qtool_bar_position(), FRAME_TOOL_BAR_POSITION(f)),
            Fcons(Qtool_bar_size(),
                  Fcons(make_fixnum(Int(tool_bar_width)), make_fixnum(Int(tool_bar_height)))),
            Fcons(Qinternal_border_width(), make_fixnum(Int(internal_border_width)))])
  }
}

/// C 3616-3658: `DEFUN ("ns-frame-geometry", …)`.
@_cdecl("Fns_frame_geometry")
func Fns_frame_geometry(_ frame: Lisp_Object) -> Lisp_Object {
  return frame_geometry(frame, Qnil())
}
fileprivate let lname_ns_frame_geometry: StaticString = "ns-frame-geometry"
fileprivate var Sns_frame_geometry: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_frame_geometry, 0, 1,
                    unsafeBitCast(Fns_frame_geometry as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Return geometric attributes of FRAME.\nFRAME must be a live frame and defaults to the selected one.  The return\nvalue is an association list of the attributes listed below.  All height\nand width values are in pixels.\n\n`outer-position' is a cons of the outer left and top edges of FRAME\n  relative to the origin - the position (0, 0) - of FRAME's display.\n\n`outer-size' is a cons of the outer width and height of FRAME.  The\n  outer size includes the title bar and the external borders as well as\n  any menu and/or tool bar of frame.\n\n`external-border-size' is a cons of the horizontal and vertical width of\n  FRAME's external borders as supplied by the window manager.\n\n`title-bar-size' is a cons of the width and height of the title bar of\n  FRAME as supplied by the window manager.  If both of them are zero,\n  FRAME has no title bar.  If only the width is zero, Emacs was not\n  able to retrieve the width information.\n\n`menu-bar-external', if non-nil, means the menu bar is external (never\n  included in the inner edges of FRAME).\n\n`menu-bar-size' is a cons of the width and height of the menu bar of\n  FRAME.\n\n`tool-bar-external', if non-nil, means the tool bar is external (never\n  included in the inner edges of FRAME).\n\n`tool-bar-position' tells on which side the tool bar on FRAME is and can\n  be one of `left', `top', `right' or `bottom'.  If this is nil, FRAME\n  has no tool bar.\n\n`tool-bar-size' is a cons of the width and height of the tool bar of\n  FRAME.\n\n`internal-border-width' is the width of the internal border of\n  FRAME.  ", args: "frame")

/// C 3660-3682: `DEFUN ("ns-frame-edges", …)`.
@_cdecl("Fns_frame_edges")
func Fns_frame_edges(_ frame: Lisp_Object, _ type: Lisp_Object) -> Lisp_Object {
  return frame_geometry(frame, ((EQ(type, Qouter_edges()) || EQ(type, Qinner_edges()))
                                ? type : Qnative_edges()))
}
fileprivate let lname_ns_frame_edges: StaticString = "ns-frame-edges"
fileprivate var Sns_frame_edges: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_frame_edges, 0, 2,
                    unsafeBitCast(Fns_frame_edges as @convention(c) (Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Return edge coordinates of FRAME.\nFRAME must be a live frame and defaults to the selected one.  The return\nvalue is a list of the form (LEFT, TOP, RIGHT, BOTTOM).  All values are\nin pixels relative to the origin - the position (0, 0) - of FRAME's\ndisplay.\n\nIf optional argument TYPE is the symbol `outer-edges', return the outer\nedges of FRAME.  The outer edges comprise the decorations of the window\nmanager (like the title bar or external borders) as well as any external\nmenu or tool bar of FRAME.  If optional argument TYPE is the symbol\n`native-edges' or nil, return the native edges of FRAME.  The native\nedges exclude the decorations of the window manager and any external\nmenu or tool bar of FRAME.  If TYPE is the symbol `inner-edges', return\nthe inner edges of FRAME.  These edges exclude title bar, any borders,\nmenu bar or tool bar of FRAME.  ", args: "frame type")

/// C 3684-3723: `DEFUN ("ns-set-mouse-absolute-pixel-position", …)` (the `NS_IMPL_COCOA` arm;
/// the GNUstep arm is empty text).
@_cdecl("Fns_set_mouse_absolute_pixel_position")
func Fns_set_mouse_absolute_pixel_position(_ x: Lisp_Object, _ y: Lisp_Object) -> Lisp_Object {
  /* GNUstep doesn't support CGWarpMouseCursorPosition, so none of
     this will work.  */
  let f = SELECTED_FRAME()
  let view = FRAME_NS_VIEW(f)
  let screen = view?.window?.screen
  let screen_frame = screen?.frame ?? NSRect()
  var mouse_x: Int32
  var mouse_y: Int32

  let primary_screen = NSScreen.screens[0]
  let primary_screen_frame = primary_screen.frame
  let primary_screen_height = primary_screen_frame.size.height

  if FRAME_INITIAL_P(f) || !FRAME_NS_P(f) {
    return Qnil()
  }

  let xval = Int32(check_integer_range(x, Int(Int32.min), Int(Int32.max)))
  let yval = Int32(check_integer_range(y, Int(Int32.min), Int(Int32.max)))

  mouse_x = Int32(screen_frame.origin.x) + xval

  if screen == primary_screen {
    mouse_y = Int32(screen_frame.origin.y) + yval
  } else {
    mouse_y = Int32(primary_screen_height - screen_frame.size.height
                    - screen_frame.origin.y) + yval
  }

  CGWarpMouseCursorPosition(CGPoint(x: CGFloat(mouse_x), y: CGFloat(mouse_y)))

  return Qnil()
}
fileprivate let lname_ns_set_mouse_absolute_pixel_position: StaticString =
  "ns-set-mouse-absolute-pixel-position"
fileprivate var Sns_set_mouse_absolute_pixel_position: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_set_mouse_absolute_pixel_position, 2, 2,
                    unsafeBitCast(Fns_set_mouse_absolute_pixel_position as @convention(c) (Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Move mouse pointer to absolute pixel position (X, Y).\nThe coordinates X and Y are interpreted in pixels relative to a position\n(0, 0) of the selected frame's display.  ", args: "x y")

/// C 3726-3743: `DEFUN ("ns-mouse-absolute-pixel-position", …)`.
@_cdecl("Fns_mouse_absolute_pixel_position")
func Fns_mouse_absolute_pixel_position() -> Lisp_Object {
  let f = SELECTED_FRAME()
  let view = FRAME_NS_VIEW(f)
  let screen = view?.window?.screen ?? NSScreen.screens[0]
  let pt = NSEvent.mouseLocation

  return Fcons(make_fixnum(Int(pt.x - screen.frame.origin.x)),
               make_fixnum(Int(screen.frame.size.height - (pt.y - screen.frame.origin.y))))
}
fileprivate let lname_ns_mouse_absolute_pixel_position: StaticString =
  "ns-mouse-absolute-pixel-position"
fileprivate var Sns_mouse_absolute_pixel_position: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_mouse_absolute_pixel_position, 0, 0,
                    unsafeBitCast(Fns_mouse_absolute_pixel_position as @convention(c) () -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Return absolute position of mouse cursor in pixels.\nThe position is returned as a cons cell (X . Y) of the\ncoordinates of the mouse cursor position in pixels relative to a\nposition (0, 0) of the selected frame's terminal.  ", args: "void")

/// C 3745-3757: `DEFUN ("ns-show-character-palette", …)`.
@_cdecl("Fns_show_character_palette")
func Fns_show_character_palette() -> Lisp_Object {
  let f = SELECTED_FRAME()
  let view = FRAME_NS_VIEW(f)
  NSApp.orderFrontCharacterPalette(view)

  return Qnil()
}
fileprivate let lname_ns_show_character_palette: StaticString = "ns-show-character-palette"
fileprivate var Sns_show_character_palette: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_show_character_palette, 0, 0,
                    unsafeBitCast(Fns_show_character_palette as @convention(c) () -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Show the macOS character palette.  ", args: "void")

/// C 3759-3777: `DEFUN ("ns-process-is-accessibility-trusted", …)`.
@_cdecl("Fns_process_is_accessibility_trusted")
func Fns_process_is_accessibility_trusted() -> Lisp_Object {
  if AXIsProcessTrusted() {
    return Qt()
  }
  return Qnil()
}
fileprivate let lname_ns_process_is_accessibility_trusted: StaticString =
  "ns-process-is-accessibility-trusted"
fileprivate var Sns_process_is_accessibility_trusted: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_process_is_accessibility_trusted, 0, 0,
                    unsafeBitCast(Fns_process_is_accessibility_trusted as @convention(c) () -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Return non-nil if Emacs is trusted by macOS Accessibility.\nReturn nil otherwise or if the OS is not macOS.\nThis is necessary for Emacs to support Zoom and related accessibility\nfeatures.  Authorize Emacs for accessibility via System\nSettings... Privacy & Security... Accessibility... and add the Emacs.app\ninstalled directory to the enabled application list.  ", args: "void")

/// C 3779-3800: `DEFUN ("ns-badge", …)`.
@_cdecl("Fns_badge")
func Fns_badge(_ badge: Lisp_Object) -> Lisp_Object {
  block_input()
  if NILP(badge) {
    NSApp.dockTile.badgeLabel = nil
  } else {
    CHECK_STRING(badge)
    NSApp.dockTile.badgeLabel = NSString(utf8String: SSDATA(badge)) as String?
  }
  unblock_input()
  return Qnil()
}
fileprivate let lname_ns_badge: StaticString = "ns-badge"
fileprivate var Sns_badge: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_badge, 1, 1,
                    unsafeBitCast(Fns_badge as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Set the app icon badge to BADGE.\nBADGE should be a string short enough to display nicely in the short\nspace intended for badges.\nIf BADGE is nil, clear the app badge.  ", args: "badge")

// ==========================================================================
// C 3802-4348: the closing batch - the dock-icon DEFUNs, the sleep block pair, `ns-send-items`,
// `EmacsFileDelegate`, the two string helpers, the `NSString (EmacsString)` category,
// `ns_move_tooltip_to_mouse_location` and `syms_of_nsfns`.
// ==========================================================================

/// C 3908-3910: `static NSInteger ns_request_user_attention_id = -1;` (C 3799) and the sleep
/// block pair: "A unique integer sleep block id and a hash map of its id to opaque NSObject
/// sleep block activity tokens."
/// C 4229-4233's variable - this unit's storage, like the two above.
@_silgen_name("Vns_version_string") var Vns_version_string: Lisp_Object

/// C's `DEFVAR_LISP`/`DEFVAR_BOOL` build a function-local `static struct Lisp_Fwd`
/// (src/lisp.h 3495-3512); Swift has no function-local statics, so each record is a
/// fileprivate global that `syms_of_nsfns` fills in and hands to `defvar_lisp`/`defvar_bool`
/// (the docs/ports/nsselect.md `O_FWD_…` shape).
@_silgen_name("femacs_fwd_Vns_icon_type_alist") var O_FWD_Vns_icon_type_alist: Lisp_Fwd
@_silgen_name("femacs_fwd_Vns_version_string") var O_FWD_Vns_version_string: Lisp_Fwd
@_silgen_name("femacs_fwd_ns_use_proxy_icon") var O_FWD_ns_use_proxy_icon: Lisp_Fwd
@_silgen_name("femacs_fwd_Vx_max_tooltip_size") var O_FWD_Vx_max_tooltip_size: Lisp_Fwd

fileprivate var ns_request_user_attention_id: Int = -1
fileprivate var sleep_block_id: UInt32 = 0
fileprivate var sleep_block_map: NSMutableDictionary? = nil

/// C 3802-3830: `DEFUN ("ns-request-user-attention", …)`.
@_cdecl("Fns_request_user_attention")
func Fns_request_user_attention(_ urgency: Lisp_Object) -> Lisp_Object {
  block_input()
  if ns_request_user_attention_id != -1 {
    NSApp.cancelUserAttentionRequest(ns_request_user_attention_id)
    ns_request_user_attention_id = -1
  }
  if !NILP(urgency) && SYMBOLP(urgency) {
    if EQ(urgency, Qinformational()) {
      ns_request_user_attention_id =
        NSApp.requestUserAttention(.informationalRequest)
    } else if EQ(urgency, Qcritical()) {
      ns_request_user_attention_id =
        NSApp.requestUserAttention(.criticalRequest)
    }
  }
  unblock_input()
  return Qnil()
}
fileprivate let lname_ns_request_user_attention: StaticString = "ns-request-user-attention"
fileprivate var Sns_request_user_attention: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_request_user_attention, 1, 1,
                    unsafeBitCast(Fns_request_user_attention as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Bounce the app dock icon to request user attention.\nIf URGENCY nil, cancel the outstanding request, if any.\nIf URGENCY is the symbol `informational', bouncing lasts a few seconds.\nIf URGENCY is the symbol `critical', bouncing lasts until Emacs is\nfocused.  ", args: "urgency")

/// C 3832-3907: `DEFUN ("ns-progress-indicator", …)`.
@_cdecl("Fns_progress_indicator")
func Fns_progress_indicator(_ progress: Lisp_Object) -> Lisp_Object {
  block_input()
  let dock_tile = NSApp.dockTile
  /* Use NSLevelIndicator with reliable redraws, not NSProgressIndicator.  */
  var level_indicator: NSLevelIndicator
  /* Reuse the indicator subview or create one. */
  if dock_tile.contentView != nil
      && (dock_tile.contentView!.subviews.count) > 0
      && ((dock_tile.contentView!.subviews.last)!.isKind(of: NSLevelIndicator.self)) {
    level_indicator = dock_tile.contentView!.subviews.last as! NSLevelIndicator
  } else {
    if dock_tile.contentView == nil {
      let image_view = NSImageView()
      image_view.image = NSApp.applicationIconImage
      dock_tile.contentView = image_view
    }
    /* Set width to the width of the application icon, and height to
       % of the icon height to respect scaled icons.  */
    let width = Float(NSApp.applicationIconImage.size.width)
    let height = Float(0.10 * NSApp.applicationIconImage.size.height)
    level_indicator = NSLevelIndicator(frame: NSRect(x: 0.0, y: 0.0,
                                                     width: CGFloat(width),
                                                     height: CGFloat(height)))
    level_indicator.wantsLayer = true  /* Performance.  */
    level_indicator.isEnabled = false  /* Ignore mouse input.  */
    /* C 3876-3882: the `NS_IMPL_GNUSTEP` arm sets `NSContinuousCapacityLevelIndicatorStyle`;
       this target takes `NSLevelIndicatorStyleContinuousCapacity` = Swift's
       `NSLevelIndicator.Style.continuousCapacity`.  */
    level_indicator.levelIndicatorStyle = .continuousCapacity
    /* Match NSProgressIndicator color.  */
    level_indicator.fillColor = NSColor.controlAccentColor
    level_indicator.minValue = 0.0
    level_indicator.maxValue = 1.0
    /* The contentView takes ownership.  */
    dock_tile.contentView!.addSubview(level_indicator)
  }
  var progress_value: Double = 0
  var hide = (NILP(progress) || (!NILP(progress) && !FLOATP(progress)))
  if !hide {
    progress_value = XFLOAT_DATA(progress)
    hide = (progress_value < 0.0 || progress_value > 1.0)
  }
  if hide {
    level_indicator.doubleValue = 0.0
    level_indicator.isHidden = true
  } else {
    level_indicator.doubleValue = progress_value
    level_indicator.isHidden = false
  }
  dock_tile.display()
  unblock_input()
  return Qnil()
}
fileprivate let lname_ns_progress_indicator: StaticString = "ns-progress-indicator"
fileprivate var Sns_progress_indicator: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_progress_indicator, 1, 1,
                    unsafeBitCast(Fns_progress_indicator as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Bounce the app dock icon to request user attention.\nPROGRESS is a float between 0.0 and 1.0.\nIf PROGRESS is nil, remove the progress indicator.  ", args: "progress")

/// C 3912-3958: `DEFUN ("ns-block-system-sleep", …)`.
@_cdecl("Fns_block_system_sleep")
func Fns_block_system_sleep(_ why: Lisp_Object, _ allow_display_sleep: Lisp_Object) -> Lisp_Object {
  block_input()

  var reason: NSString = "Emacs"
  if !NILP(why) {
    CHECK_STRING(why)
    reason = stringWithLispString(why)!
  }

  var activity_options: ProcessInfo.ActivityOptions =
    [.userInitiated, .idleSystemSleepDisabled]
  if NILP(allow_display_sleep) {
    activity_options.insert(.idleDisplaySleepDisabled)
  }

  let processInfo = ProcessInfo.processInfo
  var activity_id: (any NSObjectProtocol)? = nil
  if processInfo.responds(to: #selector(ProcessInfo.beginActivity(options:reason:))) {
    activity_id = ProcessInfo.processInfo.beginActivity(options: activity_options,
                                                        reason: reason as String)
  }
  if sleep_block_map == nil {
    sleep_block_map = NSMutableDictionary(capacity: 25)
  }

  if activity_id != nil {
    sleep_block_id += 1
    sleep_block_map!.setObject(activity_id!,
                               forKey: NSNumber(value: sleep_block_id))
    unblock_input()
    return make_fixnum(Int(sleep_block_id))
  } else {
    unblock_input()
    return Qnil()
  }
}
fileprivate let lname_ns_block_system_sleep: StaticString = "ns-block-system-sleep"
fileprivate var Sns_block_system_sleep: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_block_system_sleep, 2, 2,
                    unsafeBitCast(Fns_block_system_sleep as @convention(c) (Lisp_Object, Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Block system idle sleep.\nWHY is a string reason for the block.\nIf ALLOW-DISPLAY-SLEEP is non-nil, block the screen from sleeping.\nReturn a token to unblock this block using `ns-unblock-system-sleep',\nor nil if the block fails.  ", args: "why allow_display_sleep")

/// C 3960-3991: `DEFUN ("ns-unblock-system-sleep", …)`.
@_cdecl("Fns_unblock_system_sleep")
func Fns_unblock_system_sleep(_ token: Lisp_Object) -> Lisp_Object {
  block_input()
  var res: Lisp_Object = Qnil()
  CHECK_FIXNAT(token)
  if sleep_block_map != nil {
    let key = NSNumber(value: XFIXNAT(token))
    let activity_id = sleep_block_map!.object(forKey: key) as? NSObjectProtocol
    if activity_id != nil {
      let processInfo = ProcessInfo.processInfo
      if processInfo.responds(to: #selector(ProcessInfo.endActivity(_:))) {
        ProcessInfo.processInfo.endActivity(activity_id!)
        res = Qt()
      }
      sleep_block_map!.removeObject(forKey: key)
    }
  }
  unblock_input()
  return res
}
fileprivate let lname_ns_unblock_system_sleep: StaticString = "ns-unblock-system-sleep"
fileprivate var Sns_unblock_system_sleep: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_unblock_system_sleep, 1, 1,
                    unsafeBitCast(Fns_unblock_system_sleep as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Unblock system idle sleep.\nTOKEN is an object returned by `ns-block-system-sleep'.\nReturn non-nil if the TOKEN block was unblocked.  ", args: "token")

/// C 3993-4045: `DEFUN ("ns-send-items", …)` (the `NS_IMPL_COCOA` arm).
@_cdecl("Fns_send_items")
func Fns_send_items(_ items: Lisp_Object) -> Lisp_Object {
  CHECK_LIST(items)

  let sent = NSMutableArray()
  var tail: Lisp_Object = items
  while CONSP(tail) {
    let elt = XCAR(tail)
    if !STRINGP(elt) {
      signal_error("Element not a string", elt)
    }
    var isDir: ObjCBool = false
    if FileManager.default.fileExists(atPath: String(cString: SSDATA(elt)), isDirectory: &isDir) {
      sent.add(URL(fileURLWithPath: String(cString: SSDATA(elt))))
      tail = XCDR(tail)
      continue
    }
    sent.add(String(cString: SSDATA(elt)))
    tail = XCDR(tail)
  }

  let frame = SELECTED_FRAME()
  let window = XWINDOW(selected_window)

  var text_area_x: Int32 = 0, text_area_y: Int32 = 0
  var text_area_width: Int32 = 0, text_area_height: Int32 = 0
  window_box(window, TEXT_AREA(), &text_area_x, &text_area_y,
             &text_area_width, &text_area_height)

  let x = Int32(text_area_x) + Int32(window_phys_cursor_x(window))
  let y = Int32(text_area_y) + Int32(window_phys_cursor_y(window))

  let point = NSPoint(x: CGFloat(x + FRAME_COLUMN_WIDTH(frame) / 2),
                      y: CGFloat(y + FRAME_LINE_HEIGHT(frame) + 10))

  let view = FRAME_NS_VIEW(frame)
  let picker = NSSharingServicePicker(items: sent as! [Any])
  picker.show(relativeTo: NSRect(x: point.x - 1, y: point.y - 1, width: 2, height: 2),
              of: view!,
              preferredEdge: .maxY)

  return Qnil()
}
fileprivate let lname_ns_send_items: StaticString = "ns-send-items"
fileprivate var Sns_send_items: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_send_items, 1, 1,
                    unsafeBitCast(Fns_send_items as @convention(c) (Lisp_Object) -> Lisp_Object, to: UnsafeMutableRawPointer.self), docstring: " Send a list of items to macOS applications or services. ", args: "items")

// ==========================================================================
// C 4048-4065: `@implementation EmacsFileDelegate` - delegate methods for Open/Save panels.
// `Fns_read_file_name` (C 1774) instantiates it, and the call site precedes this definition in
// C too, so the C-ABI entry is `@_silgen_name` there and `@_cdecl` here (3.239's shape).
// ==========================================================================

class EmacsFileDelegate: NSObject, NSOpenSavePanelDelegate {
  /* C 4053-4069 implements three delegate methods, and each body is a *constant* return:

       - (BOOL)panel: (id)sender isValidFilename: (NSString *)filename  { return YES; }
       - (BOOL)panel: (id)sender shouldShowFilename: (NSString *)filename { return YES; }
       - (NSString *)panel: (id)sender userEnteredFilename: (NSString *)filename
                 confirmed: (BOOL)okFlag                                { return filename; }

     Swift cannot spell any of the three: the SDK declares the same selectors on `NSObject`
     (`NSOpenPanel.h`'s deprecated `NSObject (NSOpenSavePanelDelegate)` category) *and* marks
     them `NS_SWIFT_UNAVAILABLE`, so both a plain declaration ("conflicts with method … from
     superclass 'NSObject'") and an `@objc(selector)`-renamed one are refused (measured with a
     scratch compile).  Because every body is the delegate's neutral answer, the class with the
     methods omitted behaves identically; the ledger's `## ABI` (3) records the gap and the
     C-shim option.  */
}
/// C 1774's `[EmacsFileDelegate new]` - the class is a Swift one, so another unit's `alloc`
/// cannot reach it and the port exposes the same message as a C-ABI entry.
@_silgen_name("EmacsFileDelegate_new")
func EmacsFileDelegate_new() -> AnyObject? {
  return EmacsFileDelegate()
}

/// C 4071-4080: whether N bytes at STR are in the [1,127] range.
func all_nonzero_ascii(_ str: UnsafeMutablePointer<UInt8>?, _ n: Int) -> Bool {
  var i = 0
  while i < n {
    if str![i] < 1 || str![i] > 127 {
      return false
    }
    i += 1
  }
  return true
}

/// C 4082-4100: count the number of characters in STR, NBYTES long.  The string must be valid
/// UTF-8 - "This is faster than parse_str_as_multibyte, and much faster than
/// `[NSString lengthOfBytesUsingEncoding: NSUTF32StringEncoding]`."
func count_utf8_chars(_ strIn: UnsafePointer<CChar>?, _ nbytes: Int) -> Int {
  var str = UnsafeRawPointer(strIn!).assumingMemoryBound(to: UInt8.self)
  let end = str.advanced(by: nbytes)
  var nc = 0
  while str < end {
    nc += 1
    let c = str.pointee
    str = str.advanced(by: c <= 0x7f ? 1    // 0xxxxxxx
                          : c <= 0xdf ? 2   // 110xxxxx 10xxxxxx
                          : c <= 0xef ? 3   // 1110xxxx 10xxxxxx 10xxxxxx
                          : 4)              // 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
  }
  return nc
}

// ==========================================================================
// C 4101-4151: `@implementation NSString (EmacsString)` - this file's own definitions, called
// by the setters above and by nsmenu/nsfont/nsimage/nsselect's ports.
// ==========================================================================

/// C 4104-4141: `+ (NSString *)stringWithLispString: (Lisp_Object)string`.  STRING must not be
/// in an encoded form (e.g. UTF-8).
@_silgen_name("stringWithLispString")
func stringWithLispString(_ stringIn: Lisp_Object) -> NSString? {
  var string = stringIn
  if !STRINGP(string) {
    return nil
  }

  /* Shortcut for the common case.  */
  if all_nonzero_ascii(UnsafeMutableRawPointer(SSDATA(string))
                        .assumingMemoryBound(to: UInt8.self), SBYTES(string)) {
    return NSString(cString: SSDATA(string), encoding: String.Encoding.ascii.rawValue)
  }
  string = string_to_multibyte(string)

  /* Now the string is multibyte; convert to UTF-16.  */
  let chars = xmalloc(4 * SCHARS(string))!.assumingMemoryBound(to: UInt16.self)
  var d = chars
  var s = UnsafeRawPointer(SDATA(string)).assumingMemoryBound(to: UInt8.self)
  let end = s.advanced(by: SBYTES(string))
  while s < end {
    var p: UnsafePointer<UInt8>? = s
    let c = string_char_advance(&p)
    s = p!
    /* We pass unpaired surrogates through, because they are typically
       handled fairly well by the NS libraries (displayed with distinct
       glyphs etc).  */
    if c <= 0xffff {
      d.pointee = UInt16(c)
      d = d.advanced(by: 1)
    } else if c <= 0x10ffff {
      d.pointee = UInt16(0xd800 + ((c - 0x10000) >> 10))
      d = d.advanced(by: 1)
      d.pointee = UInt16(0xdc00 + (c & 0x3ff))
      d = d.advanced(by: 1)
    } else {
      d.pointee = UInt16(0xfffd)  /* Not valid for UTF-16.  */
      d = d.advanced(by: 1)
    }
  }
  let str = NSString(characters: chars, length: d - chars)
  xfree(chars)
  return str
}

/// C 4143-4149: `- (Lisp_Object)lispString`.
@_cdecl("lispString")
func lispString(_ string: NSString) -> Lisp_Object {
  /* If the input string includes unpaired surrogates, then the result
     will be an empty string.  */
  let utf8 = string.utf8String
  let bytes = string.lengthOfBytes(using: String.Encoding.utf8.rawValue)
  return make_multibyte_string(utf8, count_utf8_chars(utf8, bytes), bytes)
}

/// C 4153-4200 (non-static).
@_cdecl("ns_move_tooltip_to_mouse_location")
func ns_move_tooltip_to_mouse_location(_ screen_point: NSPoint) {
  var root_x: Int32 = 0, root_y: Int32 = 0
  var size = NSSize()
  var window: NSWindow? = nil

  window = nil

  if !FIXNUMP(tip_dx) || !FIXNUMP(tip_dy) {
    return
  }

  if ns_tooltip != nil {
    size = EmacsTooltip_frame(ns_tooltip).size
  } else if !FRAMEP(tip_frame)
      || !FRAME_LIVE_P(XFRAME(tip_frame))
      || !FRAME_VISIBLE_P(XFRAME(tip_frame)) {
    return
  } else {
    let tip_f = XFRAME(tip_frame)
    window = FRAME_NS_VIEW(tip_f)?.window
    size = window!.frame.size
  }

  root_x = Int32(screen_point.x)
  root_y = Int32(screen_point.y)

  /* We can directly use `compute_tip_xy' here, since it doesn't cons
     nearly as much as it does on X.  */
  compute_tip_xy(nil, Qnil(), tip_dx, tip_dy, Int32(size.width), Int32(size.height),
                 &root_x, &root_y)

  if ns_tooltip != nil {
    EmacsTooltip_moveTo(ns_tooltip, NSPoint(x: CGFloat(root_x), y: CGFloat(root_y)))
  } else {
    window?.setFrame(NSRect(x: CGFloat(root_x), y: CGFloat(root_y),
                            width: size.width, height: size.height),
                     display: true)
  }
}

// ==========================================================================
// C 4202-4348: `syms_of_nsfns`.
// ==========================================================================

/// C: `- (void)moveTo: (NSPoint)screen_point` on `EmacsTooltip` (src/nsmenu.m's class).
@_silgen_name("EmacsTooltip_moveTo")
func EmacsTooltip_moveTo(_ tip: AnyObject?, _ screen_point: NSPoint)

/// C 4202-4348.  `DEFSYM` (src/lisp.h 2394) is `/* empty */` - a make-docfile marker, not a
/// statement - so those call sites are recorded, not called; the symbols come from the
/// generated globals.
@_cdecl("syms_of_nsfns")
func syms_of_nsfns() {
  // C 4204-4208: `DEFSYM (Qfontsize, "fontsize"); … DEEPSYM (Qlight, "light");`

  // C 4210-4227: `DEFVAR_LISP ("ns-icon-type-alist", Vns_icon_type_alist, …)` - the macro
  // builds its `struct Lisp_Fwd` with `Lisp_Fwd_Obj` and the variable's address
  // (src/lisp.h 3495-3500) and hands it to `defvar_lisp`; the docstring is the macro's third
  // argument.
  O_FWD_Vns_icon_type_alist = Lisp_Fwd(
    type: Lisp_Fwd_Obj(), u: Lisp_Fwd_u(ptr: withUnsafeMutablePointer(to: &Vns_icon_type_alist) { UnsafeMutableRawPointer($0) }))
  defvar_lisp(&O_FWD_Vns_icon_type_alist, "ns-icon-type-alist", docstring: " Alist of elements (REGEXP . IMAGE) for images of icons associated to frames.\nIf the title of a frame matches REGEXP, then IMAGE.tiff is\nselected as the image of the icon representing the frame when it's\nminiaturized.  If an element is t, then Emacs tries to select an icon\nbased on the filetype of the visited file.\n\nThe images have to be installed in a folder called English.lproj in the\nEmacs folder.  You have to restart Emacs after installing new icons.\n\nExample: Install an icon Gnus.tiff and execute the following code\n\n  (setq ns-icon-type-alist\n        (append ns-icon-type-alist\n                \\='((\"^\\\\*\\\\(Group\\\\*$\\\\|Summary \\\\|Article\\\\*$\\\\)\"\n                   . \"Gnus\"))))\n\nWhen you miniaturize a Group, Summary or Article frame, Gnus.tiff will\nbe used as the image of the icon representing the frame.  ")
  Vns_icon_type_alist = list1(Qt())

  // C 4229-4233: `DEFVAR_LISP ("ns-version-string", Vns_version_string, …)`.
  O_FWD_Vns_version_string = Lisp_Fwd(
    type: Lisp_Fwd_Obj(), u: Lisp_Fwd_u(ptr: withUnsafeMutablePointer(to: &Vns_version_string) { UnsafeMutableRawPointer($0) }))
  defvar_lisp(&O_FWD_Vns_version_string, "ns-version-string", docstring: " Toolkit version for NS Windowing.  ")
  Vns_version_string = ns_appkit_version_str()

  // C 4280-4286: `DEFVAR_BOOL ("ns-use-proxy-icon", ns_use_proxy_icon, …)`.
  O_FWD_ns_use_proxy_icon = Lisp_Fwd(
    type: Lisp_Fwd_Bool(), u: Lisp_Fwd_u(ptr: withUnsafeMutablePointer(to: &ns_use_proxy_icon) { UnsafeMutableRawPointer($0) }))
  defvar_bool(&O_FWD_ns_use_proxy_icon, "ns-use-proxy-icon", docstring: " When non-nil display a proxy icon in the titlebar.\nThe proxy icon can be used to drag the file associated with the\ncurrent buffer to other applications, a printer, the desktop, etc., in\nthe same way you can from Finder.  Note that you might have to disable\n`tool-bar-mode' to see the proxy icon.\n\nThe default value is t.  ")
  ns_use_proxy_icon = true

  // C 4235-4241: `DEFVAR_LISP ("x-max-tooltip-size", Vx_max_tooltip_size, "…")`.
  O_FWD_Vx_max_tooltip_size = Lisp_Fwd(
    type: Lisp_Fwd_Obj(), u: Lisp_Fwd_u(ptr: withUnsafeMutablePointer(to: &Vx_max_tooltip_size) { UnsafeMutableRawPointer($0) }))
  defvar_lisp(&O_FWD_Vx_max_tooltip_size, "x-max-tooltip-size", docstring: " SKIP: real doc in xfns.c.  ")
  Vx_max_tooltip_size = Qnil()

  defsubr(&Sns_read_file_name)
  defsubr(&Sns_get_resource)
  defsubr(&Sns_set_resource)
  defsubr(&Sxw_display_color_p)  /* this and next called directly by C code */
  defsubr(&Sx_display_grayscale_p)
  defsubr(&Sns_font_name)
  defsubr(&Sns_list_colors)
  defsubr(&Sns_do_applescript)
  defsubr(&Sxw_color_defined_p)
  defsubr(&Sxw_color_values)
  defsubr(&Sx_server_max_request_size)
  defsubr(&Sx_server_vendor)
  defsubr(&Sx_server_version)
  defsubr(&Sx_display_pixel_width)
  defsubr(&Sx_display_pixel_height)
  defsubr(&Sns_display_monitor_attributes_list)
  defsubr(&Sns_frame_geometry)
  defsubr(&Sns_frame_edges)
  defsubr(&Sns_frame_list_z_order)
  defsubr(&Sns_frame_restack)
  defsubr(&Sns_set_mouse_absolute_pixel_position)
  defsubr(&Sns_mouse_absolute_pixel_position)
  defsubr(&Sns_show_character_palette)
  defsubr(&Sns_process_is_accessibility_trusted)
  defsubr(&Sns_badge)
  defsubr(&Sns_request_user_attention)
  defsubr(&Sns_progress_indicator)
  defsubr(&Sns_block_system_sleep)
  defsubr(&Sns_unblock_system_sleep)
  defsubr(&Sns_send_items)
  defsubr(&Sx_display_mm_width)
  defsubr(&Sx_display_mm_height)
  defsubr(&Sx_display_screens)
  defsubr(&Sx_display_planes)
  defsubr(&Sx_display_color_cells)
  defsubr(&Sx_display_visual_class)
  defsubr(&Sx_display_backing_store)
  defsubr(&Sx_display_save_under)
  defsubr(&Sx_create_frame)
  defsubr(&Sx_open_connection)
  defsubr(&Sx_close_connection)
  defsubr(&Sx_display_list)

  defsubr(&Sns_hide_others)
  defsubr(&Sns_hide_emacs)
  defsubr(&Sns_emacs_info_panel)
  defsubr(&Sns_list_services)
  defsubr(&Sns_perform_service)
  defsubr(&Sx_select_font)
  defsubr(&Sns_popup_color_panel)

  defsubr(&Sx_show_tip)
  defsubr(&Sx_hide_tip)

  tip_timer = Qnil()
  staticpro(&tip_timer)
  tip_frame = Qnil()
  staticpro(&tip_frame)
  tip_last_frame = Qnil()
  staticpro(&tip_last_frame)
  tip_last_string = Qnil()
  staticpro(&tip_last_string)
  tip_last_parms = Qnil()
  staticpro(&tip_last_parms)
  tip_dx = Qnil()
  staticpro(&tip_dx)
  tip_dy = Qnil()
  staticpro(&tip_dy)

  /* C 4326-4328, the `NS_IMPL_COCOA` + macOS 10.8 arm.  */
  defsubr(&Ssystem_move_file_to_trash)

  as_status = 0
  as_script = Qnil()
  staticpro(&as_script)
  as_result = nil

  // C 4337-4352: the thirteen `DEFSYM` calls that close the function - recorded, not called
  // (`DEFSYM` is empty in the reference).
}
