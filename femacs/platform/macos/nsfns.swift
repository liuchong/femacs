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
/// the three are one spelling once the ~9 files are one module (PORTING.md 0.1-5), and the
/// ledger's Evidence measures this struct on both sides.
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
  var root_window: UInt
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
@_silgen_name("FRAME_NS_VIEW") func FRAME_NS_VIEW(_ f: UnsafeMutableRawPointer?) -> NSView?
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
func frame_set_wants_modeline(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("frame_set_border_width")
func frame_set_border_width(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("ns_output_view") func ns_output_view(_ f: UnsafeMutableRawPointer?) -> NSView?
@_silgen_name("ns_output_set_view")
func ns_output_set_view(_ f: UnsafeMutableRawPointer?, _ v: NSView?)
@_silgen_name("ns_output_miniimage")
func ns_output_miniimage(_ f: UnsafeMutableRawPointer?) -> AnyObject?
@_silgen_name("ns_output_set_miniimage")
func ns_output_set_miniimage(_ f: UnsafeMutableRawPointer?, _ v: AnyObject?)
@_silgen_name("ns_output_cursor_color")
func ns_output_cursor_color(_ f: UnsafeMutableRawPointer?) -> NSColor?
@_silgen_name("ns_output_set_cursor_color")
func ns_output_set_cursor_color(_ f: UnsafeMutableRawPointer?, _ v: NSColor?)
@_silgen_name("ns_output_foreground_color")
func ns_output_foreground_color(_ f: UnsafeMutableRawPointer?) -> NSColor?
@_silgen_name("ns_output_set_foreground_color")
func ns_output_set_foreground_color(_ f: UnsafeMutableRawPointer?, _ v: NSColor?)
@_silgen_name("ns_output_background_color")
func ns_output_background_color(_ f: UnsafeMutableRawPointer?) -> NSColor?
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
@_silgen_name("ns_output_parent_desc") func ns_output_parent_desc(_ f: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("ns_output_set_parent_desc")
func ns_output_set_parent_desc(_ f: UnsafeMutableRawPointer?, _ v: UInt)
@_silgen_name("ns_output_explicit_parent")
func ns_output_explicit_parent(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("ns_output_set_explicit_parent")
func ns_output_set_explicit_parent(_ f: UnsafeMutableRawPointer?, _ v: Bool)
@_silgen_name("ns_output_text_cursor")
func ns_output_text_cursor(_ f: UnsafeMutableRawPointer?) -> NSObject?
@_silgen_name("ns_output_set_text_cursor")
func ns_output_set_text_cursor(_ f: UnsafeMutableRawPointer?, _ v: NSObject?)
@_silgen_name("ns_output_set_cursor_kind") func ns_output_set_cursor_kind(_ f: UnsafeMutableRawPointer?, _ kind: UnsafePointer<CChar>)
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
@_silgen_name("Qleft") func Qleft() -> Lisp_Object
@_silgen_name("Qicon_left") func Qicon_left() -> Lisp_Object
@_silgen_name("Qicon_top") func Qicon_top() -> Lisp_Object
@_silgen_name("Qunbound") func Qunbound() -> Lisp_Object

// The core's variable storage this file reads or writes.
/// C: `extern bool ns_use_proxy_icon;` (`DEFVAR_BOOL`, src/nsfns.m 4280).
@_silgen_name("ns_use_proxy_icon") var ns_use_proxy_icon: Bool
/// C: `extern Lisp_Object Vns_icon_type_alist;` (`DEFVAR_LISP`, src/nsfns.m 4268).
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
// C 4101-4151: the `NSString (EmacsString)` category - this file's own definitions, called by
// the setters above (`[NSString stringWithLispString: …]`, `[ns_app_name lispString]`) exactly
// as C's `@implementation` is reached through the category declared in src/nsterm.h 366-369.
// ==========================================================================

/// C 4104-4141: `+[NSString stringWithLispString:]`, the `NS_IMPL_COCOA` arm.
@_cdecl("stringWithLispString")
func stringWithLispString(_ string: Lisp_Object) -> NSString? {
  let str = string
  if !STRINGP(str) {
    return "<!-- invalid string -->"
  }

  let cString = SDATA(str)
  let length = SBYTES(str)

  /* C 4120-4124: `NSString *result = [[NSString alloc] initWithBytes: … encoding:
     NSUTF8StringEncoding];` - the multibyte arm; the unibyte arm (4126-4130) is
     `initWithBytes:length:cString encoding:NSASCIIStringEncoding`.  */
  if STRING_MULTIBYTE(str) {
    if let result = NSString(bytes: cString, length: length,
                             encoding: String.Encoding.utf8.rawValue) {
      return result
    }
    return "<!-- invalid string -->"
  }
  return NSString(bytes: cString, length: length,
                  encoding: String.Encoding.ascii.rawValue)
}

/// C 4143-4149: `-[NSString lispString]`, the `NS_IMPL_COCOA` arm:
/// `build_string ([self UTF8String])`.
@_cdecl("lispString")
func lispString(_ string: NSString) -> Lisp_Object {
  return build_string(string.utf8String!)
}

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
func ns_explicitly_set_name(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                            _ oldval: Lisp_Object) {
  /* C 527: `NSTRACE ("ns_explicitly_set_name");` */
  ns_set_name(f, arg, 1)
}

/// C 536-549.  Called by Emacs redisplay code; names set this way never override the user's.
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
func ns_set_tool_bar_position(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                              _ oldval: Lisp_Object) {
  if !EQ(arg, Qtop()) {
    error("Tool bar position must be `top'")
  }
}

/// C 782-803.
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
func ns_set_cursor_type(_ f: UnsafeMutableRawPointer?, _ arg: Lisp_Object,
                        _ oldval: Lisp_Object) {
  set_frame_cursor_types(f, arg)
}

/// C 975-983: called to set mouse pointer color, but all other terms use it to initialize
/// pointer types (and don't set the color ;) - "Don't think we can do this on Nextstep."
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
