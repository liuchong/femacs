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

/// More `Q…` symbols (generated globals.h, PORTING.md P-17; called as `Qfoo ()`).
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

// ==========================================================================
// C 1188-1610: `x-create-frame` (Fx_create_frame) - the file's biggest function.
// ==========================================================================

/// C: `DEFUN`'s file-static record (src/lisp.h 3406-3440): the header word is
/// `PVEC_SUBR << PSEUDOVECTOR_AREA_BITS`, the function slot is C's `{.a ## maxargs = fnname}`,
/// and the initialiser stops at `command_modes`/`doc`.
fileprivate func make_aligned_subr(_ lname: StaticString, _ minargs: Int16, _ maxargs: Int16,
                                   _ function: UnsafeMutableRawPointer) -> Aligned_Lisp_Subr {
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
    ns_output_set_parent_desc(f, UInt(XFIXNAT(parent)))
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
                    unsafeBitCast(Fx_create_frame, to: UnsafeMutableRawPointer.self))

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
                    unsafeBitCast(Fns_frame_list_z_order, to: UnsafeMutableRawPointer.self))

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
                    unsafeBitCast(Fns_frame_restack, to: UnsafeMutableRawPointer.self))

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
                    unsafeBitCast(Fx_select_font, to: UnsafeMutableRawPointer.self))

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
                    unsafeBitCast(Fns_popup_color_panel, to: UnsafeMutableRawPointer.self))

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

/// C 1774: `fileDelegate = [EmacsFileDelegate new];` - a cross-file class-method call into
/// C 4048-4065's class, which the next batch ports; the ledger's `## ABI` (3) records it.
@_silgen_name("EmacsFileDelegate_new") func EmacsFileDelegate_new() -> AnyObject?

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
                    unsafeBitCast(Fns_read_file_name, to: UnsafeMutableRawPointer.self))
