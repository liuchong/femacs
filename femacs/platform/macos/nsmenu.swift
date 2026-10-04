// femacs: port of src/nsmenu.m @ fd7afced403.
//
// Swift has no `#include`, so this unit carries the declarations of its C environment (the three
// kinds the lane uses are listed in docs/ports/nsimage.md's `## ABI` and repeated here):
// `@_silgen_name` imports of the ported core, header-only constructs (macros, INLINE functions,
// enumerators, generated-globals entries) whose C-ABI entry the core exports under the C
// construct's own name, and this file's own C interface exported with `@_cdecl`.
//
// What is specific to this unit:
//
//   * It defines four Objective-C classes (`EmacsMenu`, `EmacsToolbar`, `EmacsTooltip`,
//     `EmacsDialogPanel`) and one NSView subclass (`FlippedView`), plus two subrs.  C's classes
//     become Swift `class`es that subclass the same AppKit class with the same ivars and
//     methods (src/nsterm.h 540-651 declares them); C's `self` is Swift's `self`, and ObjC
//     nil-messaging is optional chaining.
//   * Two classes are *instantiated outside* this file (`[[EmacsMenu alloc] initWithTitle: …]`
//     at src/nsterm.m 6088-6093, `[[EmacsToolbar alloc] initForView: …]` at 9895, and
//     `[EmacsTooltip class]` at 2534); a stand-alone Swift file cannot be reached by another
//     file's `alloc`/`init` or its class object, so that is flagged in the ledger's `## ABI`
//     (3), like nsselect.swift's `S…` records.
//   * `free_frame_menubar`/`popup_activated`/`set_frame_menubar`/`ns_menu_show`/
//     `free_frame_tool_bar`/`update_frame_tool_bar`/`update_frame_tool_bar_1`/`ns_popup_dialog`/
//     `syms_of_nsmenu` and the two `DEFUN`s are the C interface this file exports.
//
// `#if NSMENUPROFILE` (src/nsmenu.m 44-47, 133-136, 165-168, 421-425) is 0 in the reference
// itself, so those blocks are recorded, not transcribed, as are the other implementation's arms:
// the `#ifdef NS_IMPL_GNUSTEP` blocks (113-120, 142-145, 295-297, 384-386, 409-413, 431-433,
// 487-515, 728-731, 821-872, 1347-1349) and the `#else` halves of the `NS_IMPL_COCOA` tests
// (553-554, 586-592, 1130-1140).  The lane's target is the Cocoa/AppKit build, and its arms
// (395-398, 548-552, 584-585, 598-655, 662-702, 770-773, 1128-1129, 1211-1233, 1560-1562) are
// what the port carries.  `NSTRACE` is a no-op in the reference too
// (`NSTRACE_ENABLED` 0 discards its arguments unread, src/nsterm.h 143), so the call sites keep
// C's text in comments instead of a call the C text does not have.
//
// `HAVE_MULTILINGUAL_MENU` is never defined (src/xmenu.c 74 even `#undef`s it), so C's
// `#ifndef HAVE_MULTILINGUAL_MENU` arms are live: `ENCODE_MENU_STRING` (src/keyboard.h 393,
// the `HAVE_NS` arm) is `ENCODE_UTF_8`, and both are C-ABI entries.  `DEFSYM` is empty in the
// reference (src/lisp.h 2394).

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
  #if HAVE_NATIVE_COMP
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

/// C: `typedef struct { ptrdiff_t bytes; } specpdl_ref;` (src/lisp.h 3646).
struct specpdl_ref {
  var bytes: Int
}

/// C: `struct timespec { time_t tv_sec; long tv_nsec; }` (the SDK's `<sys/_types/_timespec.h>`,
/// `time_t` = `long`): 16 bytes, `tv_sec` at 0 and `tv_nsec` at 8, which the ledger's probe
/// measures on both sides.
struct timespec {
  var tv_sec: Int
  var tv_nsec: Int
}

/// C: `enum button_type { BUTTON_TYPE_NONE, BUTTON_TYPE_TOGGLE, BUTTON_TYPE_RADIO }`
/// (lwlib/lwlib-widget.h 36-41): the reference's enumerator values 0/1/2, measured.
enum button_type: Int32 {
  case BUTTON_TYPE_NONE = 0
  case BUTTON_TYPE_TOGGLE = 1
  case BUTTON_TYPE_RADIO = 2
}

/// C: `typedef struct _widget_value { … } widget_value` (lwlib/lwlib-widget.h 43-103) - the menu
/// tree this file builds and hands to the core.  The `HAVE_NTGUI`/`USE_X_TOOLKIT` members
/// (70-73, 87-101) are other targets' arms; the layout probe in the ledger's Evidence measures
/// this struct on both sides (80 bytes measured, so the field list below is the whole record).
struct widget_value {
  var lname: Lisp_Object
  var name: UnsafeMutablePointer<CChar>?
  var value: UnsafeMutablePointer<CChar>?
  var lkey: Lisp_Object
  var key: UnsafeMutablePointer<CChar>?
  var help: Lisp_Object
  var enabled: Bool
  var selected: Bool
  var edited: Bool
  var button_type: Int32
  var contents: UnsafeMutablePointer<widget_value>?
  var call_data: UnsafeMutableRawPointer?
  var next: UnsafeMutablePointer<widget_value>?
}

// ==========================================================================
// Lane ABI: the C side.
// ==========================================================================

// Functions of the ported core.
/// C: `extern Lisp_Object menu_bar_items (Lisp_Object keymap)` (src/lisp.h 5087).
@_silgen_name("menu_bar_items") func menu_bar_items(_ keymap: Lisp_Object) -> Lisp_Object
/// C: `extern bool parse_single_submenu (Lisp_Object key, Lisp_Object string, Lisp_Object maps)`
/// (src/menu.h 36).
@_silgen_name("parse_single_submenu")
func parse_single_submenu(_ key: Lisp_Object, _ string: Lisp_Object, _ maps: Lisp_Object) -> Bool
/// C: `extern widget_value *digest_single_submenu (int start, int end, bool top_level_items)`
/// (src/menu.h 44).
@_silgen_name("digest_single_submenu")
func digest_single_submenu(_ start: Int32, _ end: Int32, _ top_level_items: Bool)
  -> UnsafeMutablePointer<widget_value>?
/// C: `extern widget_value *make_widget_value (const char *name, char *value, bool enabled,
/// Lisp_Object help)` (src/menu.h 43).
@_silgen_name("make_widget_value")
func make_widget_value(_ name: UnsafePointer<CChar>?, _ value: UnsafeMutablePointer<CChar>?,
                       _ enabled: Bool, _ help: Lisp_Object)
  -> UnsafeMutablePointer<widget_value>?
/// C: `extern void free_menubar_widget_value_tree (widget_value *)` (src/menu.h 39).
@_silgen_name("free_menubar_widget_value_tree")
func free_menubar_widget_value_tree(_ wv: UnsafeMutablePointer<widget_value>?)
/// C: `extern void update_submenu_strings (widget_value *)` (src/menu.h 40).
@_silgen_name("update_submenu_strings")
func update_submenu_strings(_ wv: UnsafeMutablePointer<widget_value>?)
/// C: `extern void init_menu_items (void)` / `finish_menu_items` / `discard_menu_items` /
/// `save_menu_items` (src/menu.h 32-35).
@_silgen_name("init_menu_items") func init_menu_items()
@_silgen_name("finish_menu_items") func finish_menu_items()
@_silgen_name("discard_menu_items") func discard_menu_items()
@_silgen_name("save_menu_items") func save_menu_items()
/// C: `extern void list_of_panes (Lisp_Object menu)` (src/menu.h 37).
@_silgen_name("list_of_panes") func list_of_panes(_ menu: Lisp_Object)
/// C: `extern specpdl_ref inhibit_garbage_collection (void)` (src/lisp.h 4654).
@_silgen_name("inhibit_garbage_collection") func inhibit_garbage_collection() -> specpdl_ref
/// C: `extern void record_unwind_save_match_data (void)` (src/lisp.h 4998).
@_silgen_name("record_unwind_save_match_data") func record_unwind_save_match_data()
/// C: `extern void specbind (Lisp_Object symbol, Lisp_Object value)` (src/lisp.h 4817).
@_silgen_name("specbind") func specbind(_ symbol: Lisp_Object, _ value: Lisp_Object)
/// C: `extern Lisp_Object unbind_to (specpdl_ref count, Lisp_Object value)` (src/lisp.h 4833).
@_silgen_name("unbind_to") func unbind_to(_ count: specpdl_ref, _ value: Lisp_Object) -> Lisp_Object
/// C: `extern void record_unwind_protect_void (void (*) (void))` (src/lisp.h 4825).
@_silgen_name("record_unwind_protect_void")
func record_unwind_protect_void(_ fn: @convention(c) () -> Void)
/// C: `extern void record_unwind_protect_ptr (void (*) (void *), void *)` (src/lisp.h 4820).
@_silgen_name("record_unwind_protect_ptr")
func record_unwind_protect_ptr(_ fn: @convention(c) (UnsafeMutableRawPointer?) -> Void,
                               _ arg: UnsafeMutableRawPointer?)
/// C: `extern void safe_run_hooks (Lisp_Object hook)` (src/lisp.h 5097).
@_silgen_name("safe_run_hooks") func safe_run_hooks(_ hook: Lisp_Object)
/// C: `extern void set_buffer_internal_1 (struct buffer *b)` (src/buffer.h 1113).
@_silgen_name("set_buffer_internal_1")
func set_buffer_internal_1(_ b: UnsafeMutableRawPointer?)
/// C: `INLINE void block_input (void)` (src/blockinput.h 52-56) and
/// `extern void unblock_input (void)` (src/blockinput.h 58).
@_silgen_name("block_input") func block_input()
@_silgen_name("unblock_input") func unblock_input()
/// C: `extern Lisp_Object Fcar (Lisp_Object)` / `Fcdr` / `Fcons` (subrs, globals.h) and
/// `CHECK_STRING (Lisp_Object)` (src/lisp.h 1588).
@_silgen_name("Fcar") func Fcar(_ list: Lisp_Object) -> Lisp_Object
@_silgen_name("Fcdr") func Fcdr(_ list: Lisp_Object) -> Lisp_Object
@_silgen_name("Fcons") func Fcons(_ car: Lisp_Object, _ cdr: Lisp_Object) -> Lisp_Object
@_silgen_name("CHECK_STRING") func CHECK_STRING(_ x: Lisp_Object)
/// C: `extern void check_window_system (struct frame *)` (src/frame.h 1504).
@_silgen_name("check_window_system") func check_window_system(_ f: UnsafeMutableRawPointer?)
/// C: `extern void defsubr (union Aligned_Lisp_Subr *)` (src/lisp.h 3444).
@_silgen_name("defsubr") func defsubr(_ sname: UnsafeMutablePointer<Aligned_Lisp_Subr>?)
/// C: `#define FRAME_LINE_HEIGHT(F) ((F)->line_height)` (src/frame.h 1561) and
/// `#define FRAME_COLUMN_WIDTH(F) ((F)->column_width)` (src/frame.h 1565).
@_silgen_name("FRAME_LINE_HEIGHT") func FRAME_LINE_HEIGHT(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_COLUMN_WIDTH") func FRAME_COLUMN_WIDTH(_ f: UnsafeMutableRawPointer?) -> Int32
/// C: `INLINE Lisp_Object build_string (const char *str)` (src/lisp.h 4574).
@_silgen_name("build_string") func build_string(_ str: UnsafePointer<CChar>) -> Lisp_Object
/// C: `extern Lisp_Object list1 (Lisp_Object)` / `list2` (src/lisp.h 4486, 4487).
@_silgen_name("list1") func list1(_ a: Lisp_Object) -> Lisp_Object
@_silgen_name("list2") func list2(_ a: Lisp_Object, _ b: Lisp_Object) -> Lisp_Object
/// C: `extern void *xmalloc (size_t)` / `extern void xfree (void *)` (src/lisp.h 5535, 5543).
@_silgen_name("xmalloc") func xmalloc(_ size: Int) -> UnsafeMutableRawPointer?
@_silgen_name("xfree") func xfree(_ ptr: UnsafeMutableRawPointer?)
/// C: `extern AVOID error (const char *, ...)` (src/lisp.h 4835) - `AVOID` is `_Noreturn`.
@_silgen_name("error") func error(_ format: UnsafePointer<CChar>, _ args: CVarArg...) -> Never
/// C: `extern AVOID emacs_abort (void) NO_INLINE` (src/lisp.h 5302).
@_silgen_name("emacs_abort") func emacs_abort() -> Never
/// C: `extern Lisp_Object quit (void)` (src/lisp.h 4786).
@_silgen_name("quit") func quit() -> Lisp_Object
/// C: `extern void show_help_echo (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object)`
/// (src/keyboard.h 511).
@_silgen_name("show_help_echo")
func show_help_echo(_ help: Lisp_Object, _ window: Lisp_Object, _ pos: Lisp_Object,
                    _ frame: Lisp_Object)
/// C: `extern bool menu_separator_name_p (const char *)` (src/keyboard.h 473).
@_silgen_name("menu_separator_name_p")
func menu_separator_name_p(_ name: UnsafePointer<CChar>?) -> Bool
/// C: `extern void unuse_menu_items (void)` (src/keyboard.h 385).
@_silgen_name("unuse_menu_items") func unuse_menu_items()
/// C: `extern struct timespec timer_check (void)` (src/keyboard.h 521).
@_silgen_name("timer_check") func timer_check() -> timespec
/// C: `extern Lisp_Object find_and_return_menu_selection (struct frame *f, bool keymaps,
/// void *client_data)` (src/nsterm.h 1181).
@_silgen_name("find_and_return_menu_selection")
func find_and_return_menu_selection(_ f: UnsafeMutableRawPointer?, _ keymaps: Bool,
                                    _ client_data: UnsafeMutableRawPointer?) -> Lisp_Object
/// C: `extern bool valid_image_p (Lisp_Object)` (src/dispextern.h 3755).
@_silgen_name("valid_image_p") func valid_image_p(_ spec: Lisp_Object) -> Bool
/// C: `extern void prepare_image_for_display (struct frame *, struct image *)`
/// (src/dispextern.h 3756).
@_silgen_name("prepare_image_for_display")
func prepare_image_for_display(_ f: UnsafeMutableRawPointer?, _ img: UnsafeMutableRawPointer?)
/// C: `extern ptrdiff_t lookup_image (struct frame *, Lisp_Object, int)` (src/dispextern.h 3757).
@_silgen_name("lookup_image")
func lookup_image(_ f: UnsafeMutableRawPointer?, _ spec: Lisp_Object, _ face_id: Int32) -> Int
/* C: `extern int popup_activated (void)` (src/dispextern.h 3879),
   `extern void set_frame_menubar (struct frame *, bool)` (src/frame.h 1881),
   `extern void free_frame_menubar (struct frame *)` (src/frame.h 1883) and
   `extern void update_frame_tool_bar_1 (struct frame *f, EmacsToolbar *toolbar)`
   (src/nsterm.h 1177) are *this file's own* definitions: nothing to import, they are the
   `@_cdecl` entries the `## ABI` section lists as exports.  */

// Header-only constructs (macros, INLINE functions, enumerators, generated-globals entries);
// the C names are the constructs' own names, and the core side exports each as a C-ABI entry.
@_silgen_name("EQ") func EQ(_ x: Lisp_Object, _ y: Lisp_Object) -> Bool
@_silgen_name("NILP") func NILP(_ x: Lisp_Object) -> Bool
@_silgen_name("STRINGP") func STRINGP(_ x: Lisp_Object) -> Bool
@_silgen_name("VECTORP") func VECTORP(_ x: Lisp_Object) -> Bool
@_silgen_name("AREF") func AREF(_ array: Lisp_Object, _ idx: Int) -> Lisp_Object
@_silgen_name("ASET") func ASET(_ array: Lisp_Object, _ idx: Int, _ val: Lisp_Object)
@_silgen_name("ASIZE") func ASIZE(_ array: Lisp_Object) -> Int
/// C: `INLINE Lisp_Object *aref_addr (Lisp_Object array, ptrdiff_t idx)` (src/lisp.h 1971).
@_silgen_name("aref_addr")
func aref_addr(_ array: Lisp_Object, _ idx: Int) -> UnsafeMutablePointer<Lisp_Object>?
/// C: `INLINE void *xvector_contents (Lisp_Object a)` (src/lisp.h 3957).
@_silgen_name("xvector_contents") func xvector_contents(_ a: Lisp_Object) -> UnsafeMutableRawPointer?
/// C: `enum { header_size = …, bool_header_size = …, word_size = sizeof (Lisp_Object) }`
/// (src/lisp.h 1830-1835) - three enum constants.
@_silgen_name("word_size") func word_size() -> Int
/// C: `INLINE bool STRING_MULTIBYTE (Lisp_Object str)` (src/lisp.h 1601) and
/// `#define ENCODE_MENU_STRING(str) ENCODE_UTF_8 (str)` (src/keyboard.h 393, the `HAVE_NS` arm).
@_silgen_name("STRING_MULTIBYTE") func STRING_MULTIBYTE(_ str: Lisp_Object) -> Bool
@_silgen_name("ENCODE_MENU_STRING") func ENCODE_MENU_STRING(_ str: Lisp_Object) -> Lisp_Object
/// C: `#define XSETFRAME(a, b) (XSETPSEUDOVECTOR (a, b, PVEC_FRAME))` (src/frame.h) - the macro
/// assigns to its first argument, so the entry takes its address.
@_silgen_name("XSETFRAME") func XSETFRAME(_ a: UnsafeMutablePointer<Lisp_Object>, _ b: UnsafeMutableRawPointer?)
/// C: `INLINE struct buffer *XBUFFER (Lisp_Object a)` (src/buffer.h 1421) and
/// `INLINE struct window *XWINDOW (Lisp_Object a)` (src/window.h 1103).
@_silgen_name("XBUFFER") func XBUFFER(_ a: Lisp_Object) -> UnsafeMutableRawPointer?
@_silgen_name("XWINDOW") func XWINDOW(_ a: Lisp_Object) -> UnsafeMutableRawPointer?
/// C: `INLINE Lisp_Object image_spec_value (…)`-family accessors this file reads through
/// `struct image`: `IMAGE_FROM_ID (f, id)` (src/frame.h 1668) and the two `struct image`
/// members C reaches directly (`img->load_failed_p`, `img->pixmap`); the core owns the record,
/// so each read is a C-ABI entry named after the expression (the ledger's (3) lists them).
@_silgen_name("IMAGE_FROM_ID")
func IMAGE_FROM_ID(_ f: UnsafeMutableRawPointer?, _ id: Int) -> UnsafeMutableRawPointer?
@_silgen_name("image_load_failed_p") func image_load_failed_p(_ img: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("image_pixmap") func image_pixmap(_ img: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
/// C: `#define FRAME_NS_VIEW(f) ((f)->output_data.ns->view)` (src/nsterm.h 1045).
@_silgen_name("FRAME_NS_VIEW") func FRAME_NS_VIEW(_ f: UnsafeMutableRawPointer?) -> NSView?
/// C: `#define SELECTED_FRAME()` (src/frame.h 1526-1528).
@_silgen_name("SELECTED_FRAME") func SELECTED_FRAME() -> UnsafeMutableRawPointer?
/// C: `#define FRAME_LIVE_P(f)` (src/frame.h 1043), `FRAME_NS_P(f)` (src/frame.h 946-948),
/// `FRAME_EXTERNAL_MENU_BAR(f)` (src/frame.h 1080-1082), `FRAME_SELECTED_WINDOW(f)`
/// (src/frame.h 1204), `FRAME_MENU_BAR_ITEMS(f)` (src/frame.h 1367), `FRAME_COLS(f)`
/// (src/frame.h 1075) and `FRAME_LINES(f)` (src/frame.h 1076).
@_silgen_name("FRAME_LIVE_P") func FRAME_LIVE_P(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("FRAME_NS_P") func FRAME_NS_P(_ f: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("FRAME_EXTERNAL_MENU_BAR")
func FRAME_EXTERNAL_MENU_BAR(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_SELECTED_WINDOW")
func FRAME_SELECTED_WINDOW(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("FRAME_MENU_BAR_ITEMS")
func FRAME_MENU_BAR_ITEMS(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("FRAME_COLS") func FRAME_COLS(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FRAME_LINES") func FRAME_LINES(_ f: UnsafeMutableRawPointer?) -> Int32
/// C: `INLINE void fset_menu_bar_items (struct frame *f, Lisp_Object val)` (src/frame.h 790)
/// and `fset_menu_bar_vector` (src/frame.h 797).
@_silgen_name("fset_menu_bar_items")
func fset_menu_bar_items(_ f: UnsafeMutableRawPointer?, _ val: Lisp_Object)
@_silgen_name("fset_menu_bar_vector")
func fset_menu_bar_vector(_ f: UnsafeMutableRawPointer?, _ val: Lisp_Object)
/// C: the frame members this file reads or writes directly and that no C macro or INLINE
/// function names (`f->menu_bar_items_used`, `f->menu_bar_vector`, `f->n_tool_bar_items`,
/// `f->tool_bar_items`, `f->left_pos`, `f->top_pos`, `f->output_data.ns->in_animation`):
/// `struct frame`/`struct ns_output` belong to the core, so each access is a C-ABI entry named
/// after the member.  The ledger's `## ABI` (3) lists them.
@_silgen_name("frame_menu_bar_items_used")
func frame_menu_bar_items_used(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_set_menu_bar_items_used")
func frame_set_menu_bar_items_used(_ f: UnsafeMutableRawPointer?, _ v: Int32)
@_silgen_name("frame_menu_bar_vector")
func frame_menu_bar_vector(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("frame_n_tool_bar_items")
func frame_n_tool_bar_items(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_tool_bar_items")
func frame_tool_bar_items(_ f: UnsafeMutableRawPointer?) -> Lisp_Object
@_silgen_name("frame_left_pos") func frame_left_pos(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("frame_top_pos") func frame_top_pos(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("ns_output_in_animation")
func ns_output_in_animation(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("ns_output_set_in_animation")
func ns_output_set_in_animation(_ f: UnsafeMutableRawPointer?, _ v: Int32)
/// C: `INLINE bool timespec_valid_p (struct timespec t)` (src/systime.h 68) and
/// `timespectod` (lib/timespec.h 86, `_GL_TIMESPEC_INLINE double timespectod (struct timespec a)`).
@_silgen_name("timespec_valid_p") func timespec_valid_p(_ t: timespec) -> Bool
@_silgen_name("timespectod") func timespectod(_ a: timespec) -> Double
@_silgen_name("SPECPDL_INDEX") func SPECPDL_INDEX() -> specpdl_ref
/// C: enumerators of `enum menu_item_idx` / `menu_item_pane_idx` (src/keyboard.h 360-378),
/// `enum tool_bar_item_idx` / `tool_bar_item_image` (src/dispextern.h 3451-3508),
/// `MENU_KEYMAPS`/`MENU_FOR_CLICK` (src/menu.h 26-30) and `BUTTON_TYPE_*`
/// (lwlib/lwlib-widget.h 36-41).
@_silgen_name("MENU_ITEMS_PANE_NAME") func MENU_ITEMS_PANE_NAME() -> Int
@_silgen_name("MENU_ITEMS_PANE_PREFIX") func MENU_ITEMS_PANE_PREFIX() -> Int
@_silgen_name("MENU_ITEMS_PANE_LENGTH") func MENU_ITEMS_PANE_LENGTH() -> Int
@_silgen_name("MENU_ITEMS_ITEM_NAME") func MENU_ITEMS_ITEM_NAME() -> Int
@_silgen_name("MENU_ITEMS_ITEM_ENABLE") func MENU_ITEMS_ITEM_ENABLE() -> Int
@_silgen_name("MENU_ITEMS_ITEM_VALUE") func MENU_ITEMS_ITEM_VALUE() -> Int
@_silgen_name("MENU_ITEMS_ITEM_EQUIV_KEY") func MENU_ITEMS_ITEM_EQUIV_KEY() -> Int
@_silgen_name("MENU_ITEMS_ITEM_DEFINITION") func MENU_ITEMS_ITEM_DEFINITION() -> Int
@_silgen_name("MENU_ITEMS_ITEM_TYPE") func MENU_ITEMS_ITEM_TYPE() -> Int
@_silgen_name("MENU_ITEMS_ITEM_SELECTED") func MENU_ITEMS_ITEM_SELECTED() -> Int
@_silgen_name("MENU_ITEMS_ITEM_HELP") func MENU_ITEMS_ITEM_HELP() -> Int
@_silgen_name("MENU_ITEMS_ITEM_LENGTH") func MENU_ITEMS_ITEM_LENGTH() -> Int
@_silgen_name("MENU_KEYMAPS") func MENU_KEYMAPS() -> Int
@_silgen_name("TOOL_BAR_ITEM_ENABLED_P") func TOOL_BAR_ITEM_ENABLED_P() -> Int
@_silgen_name("TOOL_BAR_ITEM_CAPTION") func TOOL_BAR_ITEM_CAPTION() -> Int
@_silgen_name("TOOL_BAR_ITEM_IMAGES") func TOOL_BAR_ITEM_IMAGES() -> Int
@_silgen_name("TOOL_BAR_ITEM_TYPE") func TOOL_BAR_ITEM_TYPE() -> Int
@_silgen_name("TOOL_BAR_ITEM_HELP") func TOOL_BAR_ITEM_HELP() -> Int
@_silgen_name("TOOL_BAR_ITEM_LABEL") func TOOL_BAR_ITEM_LABEL() -> Int
@_silgen_name("TOOL_BAR_ITEM_NSLOTS") func TOOL_BAR_ITEM_NSLOTS() -> Int
@_silgen_name("TOOL_BAR_IMAGE_ENABLED_SELECTED")
func TOOL_BAR_IMAGE_ENABLED_SELECTED() -> Int
/// C: `PVEC_SUBR` (src/lisp.h 999).
/// C: `PSEUDOVECTOR_AREA_BITS` (src/lisp.h 1039).
@_silgen_name("PVEC_SUBR") func PVEC_SUBR() -> Int
@_silgen_name("PSEUDOVECTOR_AREA_BITS") func PSEUDOVECTOR_AREA_BITS() -> Int
/// C: `#define Qfoo builtin_lisp_symbol (iQfoo)`, one per symbol, generated into globals.h
/// (PORTING.md P-17); called as `Qfoo ()`, the Rust mirror's shape.
@_silgen_name("Qnil") func Qnil() -> Lisp_Object
@_silgen_name("Qt") func Qt() -> Lisp_Object
@_silgen_name("Qinhibit_quit") func Qinhibit_quit() -> Lisp_Object
@_silgen_name("Qdebug_on_next_call") func Qdebug_on_next_call() -> Lisp_Object
@_silgen_name("Qoverriding_terminal_local_map")
func Qoverriding_terminal_local_map() -> Lisp_Object
@_silgen_name("Qoverriding_local_map") func Qoverriding_local_map() -> Lisp_Object
@_silgen_name("Qactivate_menubar_hook") func Qactivate_menubar_hook() -> Lisp_Object
@_silgen_name("Qmenu_bar_update_hook") func Qmenu_bar_update_hook() -> Lisp_Object
@_silgen_name("Qlambda") func Qlambda() -> Lisp_Object
@_silgen_name("Qquote") func Qquote() -> Lisp_Object
@_silgen_name("QCtoggle") func QCtoggle() -> Lisp_Object
@_silgen_name("QCradio") func QCradio() -> Lisp_Object
@_silgen_name("Qundefined") func Qundefined() -> Lisp_Object

// The core's variable storage this file reads or writes: C's `menu_items` and its three
// counters (src/keyboard.h 344-358) and the two lane globals src/nsterm.m defines
// (`ns_app_name`, `context_menu_value`, src/nsterm.h 775, 933).  A `@_silgen_name` `var` is an
// import of the symbol's storage, which is what C's `extern` is.
@_silgen_name("menu_items") var menu_items: Lisp_Object
@_silgen_name("menu_items_allocated") var menu_items_allocated: Int32
@_silgen_name("menu_items_used") var menu_items_used: Int32
@_silgen_name("menu_items_n_panes") var menu_items_n_panes: Int32
@_silgen_name("ns_app_name") var ns_app_name: NSString?
@_silgen_name("context_menu_value") var context_menu_value: Int
/// C: `extern Lisp_Object Vmenu_updating_frame;` (`DEFVAR_LISP ("menu-updating-frame", …)`,
/// src/xdisp.c) - the globals entry this file sets with `XSETFRAME`.
@_silgen_name("Vmenu_updating_frame") var Vmenu_updating_frame: Lisp_Object
/// C: `extern Lisp_Object Voverriding_local_map_menu_flag;` (`DEFVAR_LISP
/// ("overriding-local-map-menu-flag", …)`, src/keyboard.c 14147) - the variable
/// `ns_update_menubar` tests; C's `NILP (Voverriding_local_map_menu_flag)` at src/nsmenu.m 189.
@_silgen_name("Voverriding_local_map_menu_flag")
var Voverriding_local_map_menu_flag: Lisp_Object
/// C: `#define current_buffer (current_thread->m_current_buffer)` (src/thread.h 147) and
/// `#define waiting_for_input (current_thread->m_waiting_for_input)` (thread.h 185) - two
/// thread-field macros this unit reads, the second also writes (`waiting_for_input = 0` at
/// src/nsmenu.m 202).  The core owns the thread record, so each access is a C-ABI entry; the
/// write's entry is the port's name (the ledger's `## ABI` (3) lists both).
@_silgen_name("current_buffer") func current_buffer() -> UnsafeMutableRawPointer?
@_silgen_name("waiting_for_input") func waiting_for_input_get() -> Bool
@_silgen_name("set_waiting_for_input_flag") func waiting_for_input_set(_ v: Bool)
/// C: `INLINE char *SSDATA (Lisp_Object string)` (src/lisp.h 1658).
@_silgen_name("SSDATA") func SSDATA(_ string: Lisp_Object) -> UnsafeMutablePointer<CChar>?
/// C: `XWINDOW (w)->contents` (src/window.h's `struct window`) and C's `struct image` members
/// `img->load_failed_p`/`img->pixmap` (src/dispextern.h's `struct image`): the records belong to
/// the core, so each read is a C-ABI entry named after the member (the ledger's (3)).
@_silgen_name("window_contents")
func window_contents(_ w: UnsafeMutableRawPointer?) -> Lisp_Object
/// C: `wv->next`/`wv->contents` are this file's own `widget_value` storage, so no entries are
/// needed for them; `strlen`/`memcpy`/`memmove`/`memmem`/`strcmp` are the SDK's.

// Constructs of this lane's own headers that other lane files define.  The definition owner is
// named on each; the ledger's `## ABI` lists them together.
/// C: `- (void)createToolbar: (struct frame *)f` (src/nsterm.h 440), defined src/nsterm.m
/// 9891-9910.  A cross-file Objective-C method is a C-ABI entry with the receiver first; the
/// port of src/nsterm.m owns the definition.
@_silgen_name("createToolbar")
func createToolbar(_ window: UnsafeMutableRawPointer?, _ f: UnsafeMutableRawPointer?)
/// C: `+[NSString stringWithLispString:]` (declared src/nsterm.h 367, defined src/nsfns.m
/// 4104) and `-[NSString lispString]` (src/nsterm.h 368, defined src/nsfns.m 4143).
@_silgen_name("stringWithLispString")
func stringWithLispString(_ string: Lisp_Object) -> NSString?
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
/// a pointer-sized value, 0 for `Qnil`.  Swift's `CVarArg` has no conformance for an Optional
/// pointer, so the port passes the same bits as a `UInt` (docs/ports/nsimage.md).
@inline(__always)
func lispVariadicArg(_ value: Lisp_Object) -> UInt {
  return value.map { UInt(bitPattern: UnsafeRawPointer($0)) } ?? 0
}

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

/// C: `MAX (a, b)` (src/lisp.h) - Swift's `max` over the same two `CGFloat`s.
@inline(__always)
fileprivate func MAX(_ a: CGFloat, _ b: CGFloat) -> CGFloat { return a > b ? a : b }

/// C's `""` in `((NILP (pane_name) ? "" : SSDATA (pane_name)))` (src/nsmenu.m 963-964) and in
/// `pane_string = ""` (968): a Swift string literal's pointer only lives for the duration of a
/// call, so the port keeps one stable empty C string for the places C reuses the literal.
fileprivate let empty_c_string: UnsafePointer<CChar> = {
  let p = UnsafeMutablePointer<CChar>.allocate(capacity: 1)
  p[0] = 0
  return UnsafePointer(p)
}()

/// C's `menu_items` is a Lisp vector this file indexes by slot; the reads go through the core's
/// `AREF`, and this keeps C's `int` subscripts spelled as C spells them.
@inline(__always)
fileprivate func menu_items_at(_ i: Int) -> Lisp_Object { return AREF(menu_items, i) }

// ==========================================================================
// C 50-56: the file-scope objects.
// ==========================================================================

/// C 50: `EmacsMenu *svcsMenu;` (src/nsterm.h 776) - this file *defines* it, and src/nsterm.m
/// 6088-6138 (`ns_initialize_display_info`) is its only user.  Swift cannot emit a C-named data
/// symbol (`@_cdecl` refuses `var`; `@_silgen_name` on a `var` only imports), so the name is
/// imported here like `ns_app_name`/`context_menu_value`; the storage itself has to come from
/// the port's C-ABI side, which the ledger's `## ABI` (3) records as an open item.
@_silgen_name("svcsMenu") var svcsMenu: UnsafeMutableRawPointer?

/// C 52: `static int popup_activated_flag;` - the re-entrancy flag `popup_activated` returns
/// and the popup/dialog paths set.
fileprivate var popup_activated_flag: Int32 = 0

/// C 56: `static struct frame *last_menubar_frame;` - the frame whose menubar is on screen.
fileprivate var last_menubar_frame: UnsafeMutableRawPointer? = nil

// ==========================================================================
// C 62-100: Menu: externally-called functions.
// ==========================================================================

/// C 73-93: discard the menubar's items (the menubar itself is shared among frames and is not
/// discarded).
@_cdecl("free_frame_menubar")
func free_frame_menubar(_ f: UnsafeMutableRawPointer?) {
  let menu = NSApp.mainMenu

  if f != last_menubar_frame { return }
  last_menubar_frame = nil

  var i = (menu?.numberOfItems ?? 0) - 1
  while i >= 0 {
    let item = menu?.item(at: i)
    let title = item?.title

    if (ns_app_name as String?) == title { i -= 1; continue }

    menu?.removeItem(at: i)
    i -= 1
  }
}

/// C 96-100.
@_cdecl("popup_activated")
func popup_activated() -> Int32 {
  return popup_activated_flag
}

/* C 103-109's prose: three cases - (1) ! deep_p, submenu = nil: fresh switch onto a frame;
   (2) deep_p, submenu = nil: recompute all submenus; (3) deep_p, submenu != nil: update one
   submenu's contents.  */

/// C 110-437: update the NS menubar.  `NSTRACE ("ns_update_menubar")` (138) is the reference's
/// no-op trace (src/nsterm.h 143: `NSTRACE_ENABLED` 0), so the call is recorded here, not
/// called.  The `NS_IMPL_GNUSTEP` re-entrancy guard (113-120, 142-145, 295-297, 431-433) is
/// another implementation's arm.
fileprivate func ns_update_menubar(_ f: UnsafeMutableRawPointer?, _ deep_p: Bool) {
  var needsSet = false
  var menu = NSApp.mainMenu
  var owfi = false

  var items: Lisp_Object = Qnil()
  var wv: UnsafeMutablePointer<widget_value>? = nil
  var first_wv: UnsafeMutablePointer<widget_value>? = nil
  var prev_wv: UnsafeMutablePointer<widget_value>? = nil
  var i: Int = 0
  var submenu_start: UnsafeMutablePointer<Int32>? = nil
  var submenu_end: UnsafeMutablePointer<Int32>? = nil
  var submenu_top_level_items: UnsafeMutablePointer<Bool>? = nil
  var submenu_n_panes: UnsafeMutablePointer<Int32>? = nil

  // C 138: `NSTRACE ("ns_update_menubar");`

  if f != SELECTED_FRAME() || FRAME_EXTERNAL_MENU_BAR(f) == 0 {
    return
  }

  withUnsafeMutablePointer(to: &Vmenu_updating_frame) { XSETFRAME($0, f) }
  last_menubar_frame = f
  block_input()

  /* Menu may have been created automatically; if so, discard it.  */
  if (menu?.isKind(of: EmacsMenu.self) ?? false) == false {
    menu = nil
  }

  if menu == nil {
    menu = EmacsMenu(title: ns_app_name as String? ?? "")
    needsSet = true
  }

  if deep_p {
    /* Make a widget-value tree representing the entire menu trees.  */

    let prev = current_buffer()
    let specpdl_count = SPECPDL_INDEX()
    let previous_menu_items_used = frame_menu_bar_items_used(f)
    let previous_items = UnsafeMutablePointer<Lisp_Object>.allocate(
      capacity: Int(previous_menu_items_used))
    let subitems = Int32(ASIZE(items) / 4)

    items = XWINDOW(FRAME_SELECTED_WINDOW(f)).map { window_contents($0) } ?? Qnil()
    specbind(Qinhibit_quit(), Qt())
    /* Don't let the debugger step into this code
       because it is not reentrant.  */
    specbind(Qdebug_on_next_call(), Qnil())

    record_unwind_save_match_data()
    if NILP(Voverriding_local_map_menu_flag) {
      specbind(Qoverriding_terminal_local_map(), Qnil())
      specbind(Qoverriding_local_map(), Qnil())
    }

    set_buffer_internal_1(XBUFFER(items))

    /* TODO: for some reason this is not needed in other terms, but
       some menu updates call Info-extract-pointer which causes
       abort-on-error if waiting-for-input.  Needs further
       investigation.  */
    owfi = waiting_for_input_get()
    waiting_for_input_set(false)

    /* Run the Lucid hook.  */
    safe_run_hooks(Qactivate_menubar_hook())

    /* If it has changed current-menubar from previous value,
       really recompute the menubar from the value.  */
    safe_run_hooks(Qmenu_bar_update_hook())
    fset_menu_bar_items(f, menu_bar_items(FRAME_MENU_BAR_ITEMS(f)))

    items = FRAME_MENU_BAR_ITEMS(f)

    /* Save the frame's previous menu bar contents data.  */
    if previous_menu_items_used != 0 {
      _ = memcpy(previous_items, xvector_contents(frame_menu_bar_vector(f)),
                 Int(previous_menu_items_used) * word_size())
    }

    /* Fill in menu_items with the current menu bar contents.
       This can evaluate Lisp code.  */
    save_menu_items()

    menu_items = frame_menu_bar_vector(f)
    menu_items_allocated = VECTORP(menu_items) ? Int32(ASIZE(menu_items)) : 0
    submenu_start = UnsafeMutablePointer<Int32>.allocate(capacity: Int(subitems) + 1)
    submenu_end = UnsafeMutablePointer<Int32>.allocate(capacity: Int(subitems))
    submenu_n_panes = UnsafeMutablePointer<Int32>.allocate(capacity: Int(subitems))
    submenu_top_level_items = UnsafeMutablePointer<Bool>.allocate(capacity: Int(subitems))
    init_menu_items()
    i = 0
    while i < Int(subitems) {
      let key = AREF(items, 4 * i)
      let string = AREF(items, 4 * i + 1)
      let maps = AREF(items, 4 * i + 2)
      if NILP(string) { break }

      submenu_start![i] = menu_items_used

      menu_items_n_panes = 0
      submenu_top_level_items![i] = parse_single_submenu(key, string, maps)
      submenu_n_panes![i] = menu_items_n_panes

      submenu_end![i] = menu_items_used
      i += 1
    }

    submenu_start![i] = -1
    finish_menu_items()
    waiting_for_input_set(owfi)

    /* Convert menu_items into widget_value trees
       to display the menu.  This cannot evaluate Lisp code.  */

    wv = make_widget_value("menubar", nil, true, Qnil())
    wv!.pointee.button_type = button_type.BUTTON_TYPE_NONE.rawValue
    first_wv = wv

    i = 0
    while submenu_start![i] >= 0 {
      menu_items_n_panes = submenu_n_panes![i]
      wv = digest_single_submenu(submenu_start![i], submenu_end![i],
                                 submenu_top_level_items![i])
      if prev_wv != nil {
        prev_wv!.pointee.next = wv
      } else {
        first_wv!.pointee.contents = wv
      }
      /* Don't set wv->name here; GC during the loop might relocate it.  */
      wv!.pointee.enabled = true
      wv!.pointee.button_type = button_type.BUTTON_TYPE_NONE.rawValue
      prev_wv = wv
      i += 1
    }

    set_buffer_internal_1(prev)

    /* If there has been no change in the Lisp-level contents
       of the menu bar, skip redisplaying it.  Just exit.  */

    /* Compare the new menu items with the ones computed last time.  */
    i = 0
    while i < Int(previous_menu_items_used) {
      if menu_items_used == Int32(i)
        || !EQ(previous_items[i], AREF(menu_items, i)) { break }
      i += 1
    }
    if i == Int(menu_items_used) && i == Int(previous_menu_items_used) && i != 0 {
      /* The menu items have not changed.  Don't bother updating
         the menus in any form, since it would be a no-op.  */
      free_menubar_widget_value_tree(first_wv)
      discard_menu_items()
      _ = unbind_to(specpdl_count, Qnil())
      previous_items.deallocate()
      submenu_start?.deallocate()
      submenu_end?.deallocate()
      submenu_n_panes?.deallocate()
      submenu_top_level_items?.deallocate()
      return
    }

    /* The menu items are different, so store them in the frame.  */
    fset_menu_bar_vector(f, menu_items)
    frame_set_menu_bar_items_used(f, menu_items_used)

    /* This undoes save_menu_items.  */
    _ = unbind_to(specpdl_count, Qnil())

    /* Now GC cannot happen during the lifetime of the widget_value,
       so it's safe to store data from a Lisp_String.  */
    wv = first_wv!.pointee.contents
    i = 0
    while i < ASIZE(items) {
      let string = AREF(items, i + 1)
      if NILP(string) { break }
      wv!.pointee.name = SSDATA(string)
      update_submenu_strings(wv!.pointee.contents)
      wv = wv!.pointee.next
      i += 4
    }

    previous_items.deallocate()
    submenu_start?.deallocate()
    submenu_end?.deallocate()
    submenu_n_panes?.deallocate()
    submenu_top_level_items?.deallocate()
  } else {
    /* Make a widget-value tree containing
       just the top level menu bar strings.  */

    wv = make_widget_value("menubar", nil, true, Qnil())
    wv!.pointee.button_type = button_type.BUTTON_TYPE_NONE.rawValue
    first_wv = wv

    items = FRAME_MENU_BAR_ITEMS(f)
    i = 0
    while i < ASIZE(items) {
      let string = AREF(items, i + 1)
      if NILP(string) { break }

      wv = make_widget_value(SSDATA(string), nil, true, Qnil())
      wv!.pointee.button_type = button_type.BUTTON_TYPE_NONE.rawValue
      /* This prevents lwlib from assuming this
         menu item is really supposed to be empty.  */
      /* The intptr_t cast avoids a warning.
         This value just has to be different from small integers.  */
      wv!.pointee.call_data = UnsafeMutableRawPointer(bitPattern: -1)

      if prev_wv != nil {
        prev_wv!.pointee.next = wv
      } else {
        first_wv!.pointee.contents = wv
      }
      prev_wv = wv
      i += 4
    }

    /* Forget what we thought we knew about what is in the
       detailed contents of the menu bar menus.
       Changing the top level always destroys the contents.  */
    frame_set_menu_bar_items_used(f, 0)
  }

  /* Now, update the NS menu.  */
  i = 0

  /* Make sure we skip the "application" menu, which is always the
     first entry in our top-level menu.  */
  if i < (menu?.numberOfItems ?? 0) {
    let title = menu?.item(at: i)?.title
    if (ns_app_name as String?) == title { i += 1 }
  }

  wv = first_wv!.pointee.contents
  while wv != nil {
    var submenu: EmacsMenu?

    if i < (menu?.numberOfItems ?? 0) {
      let titleStr = String(cString: wv!.pointee.name!)
      let item = menu!.item(at: i)
      submenu = item?.submenu.map { unsafeDowncast($0, to: EmacsMenu.self) }

      item?.title = titleStr
      submenu?.title = titleStr
      submenu?.removeAllItems()
    } else {
      submenu = unsafeDowncast(menu!, to: EmacsMenu.self).addSubmenuWithTitle(wv!.pointee.name!)
    }

    if submenu?.title == "Help" { NSApp.helpMenu = submenu }

    if deep_p { submenu!.fillWithWidgetValue(wv!.pointee.contents) }

    i += 1
    wv = wv!.pointee.next
  }

  while i < (menu?.numberOfItems ?? 0) {
    /* Remove any extra items.  */
    menu?.removeItem(at: i)
    i += 1
  }

  free_menubar_widget_value_tree(first_wv)

  /* set main menu */
  if needsSet { NSApp.mainMenu = menu }

  unblock_input()
}

/// C 444-447: the main emacs core entry point for menubar menus (src/frame.h 1881 declares the
/// prototype other files use; the definition is this file's).
@_cdecl("set_frame_menubar")
func set_frame_menubar(_ f: UnsafeMutableRawPointer?, _ deep_p: Bool) {
  ns_update_menubar(f, deep_p)
}

// ==========================================================================
// C 450-874: Menu: class implementation (EmacsMenu).
// ==========================================================================

/// C 457-459's prose: a menu that can define itself from Emacs "widget_value"s and lazily
/// update itself when clicked.
class EmacsMenu: NSMenu, NSMenuDelegate {
  /// C 542: `BOOL needsUpdate;` (src/nsterm.h 540-543).
  var needsUpdate: Bool = false

  /// C 462-472: C overrides `-initWithTitle:` (an NSMenu designated initializer).
  override init(title: String) {
    super.init(title: title)
    self.autoenablesItems = false
    self.delegate = self
    needsUpdate = true
  }

  /// NSMenu's own `-initWithCoder:`, inherited as-is: the port's override above is a designated
  /// initializer, so Swift wants this one spelled out rather than synthesised.
  required init(coder: NSCoder) {
    super.init(coder: coder)
  }

  /// C 475-516: the delegate method.  The `NS_IMPL_GNUSTEP` re-entrancy guard (487-503,
  /// 507-515) is another implementation's arm.
  @objc func menuNeedsUpdate(_ menu: NSMenu) {
    /* The context menu is built and then displayed, as opposed to the
       top-menu, which is partially built and then updated and filled in
       when it's time to display it.  Therefore, we don't call
       ns_update_menubar if a context menu is active. */
    if context_menu_value != 0 { return }

    if !FRAME_LIVE_P(SELECTED_FRAME()) { return }

    if needsUpdate {
      ns_update_menubar(SELECTED_FRAME(), true)
    }
  }

  /// C 519-525.
  override func performKeyEquivalent(with theEvent: NSEvent) -> Bool {
    if SELECTED_FRAME() != nil && FRAME_NS_P(SELECTED_FRAME())
        && FRAME_NS_VIEW(SELECTED_FRAME()) != nil {
      FRAME_NS_VIEW(SELECTED_FRAME())?.keyDown(with: theEvent)
    }
    return true
  }

  /// C 528-578.
  @objc func addItemWithWidgetValue(_ wvptr: UnsafeMutableRawPointer?,
                                    attributes: NSDictionary?) -> NSMenuItem {
    var item: NSMenuItem
    let wv = wvptr!.assumingMemoryBound(to: widget_value.self)

    if menu_separator_name_p(wv.pointee.name) {
      item = NSMenuItem.separator()
    } else {
      var title = String(cString: wv.pointee.name!)
      if wv.pointee.key != nil {
        let key = String(cString: wv.pointee.key!)
        /* Cocoa only permits a single key (with modifiers) as
           keyEquivalent, so we put them in the title string
           in a tab-separated column. */
        title = title + "\t" + key
      }

      let atitle = NSAttributedString(string: title, attributes: attributes as? [NSAttributedString.Key: Any])
      item = NSMenuItem()
      item.action = NSSelectorFromString("menuDown:")
      item.attributedTitle = atitle
      item.isEnabled = wv.pointee.enabled

      /* Draw radio buttons and tickboxes.  */
      if wv.pointee.selected && (wv.pointee.button_type == button_type.BUTTON_TYPE_TOGGLE.rawValue
                                 || wv.pointee.button_type == button_type.BUTTON_TYPE_RADIO.rawValue) {
        item.state = .on
      } else {
        item.state = .off
      }

      item.tag = Int(bitPattern: wv.pointee.call_data)
    }

    self.addItem(item)
    return item
  }

  /// C 581-596: C's `-removeAllItems` overrides NSMenu's, whose Cocoa arm calls `super` and
  /// then sets `needsUpdate`; the GNUstep arm (586-593) rebuilds the removal by hand.
  override func removeAllItems() {
    super.removeAllItems()
    needsUpdate = true
  }

  /* C 598-655: the Cocoa-only `key_symbols` table and `prettify_key`, which are what turn the
     menu key descriptions into the symbols macOS shows.  The table is a Swift array of the same
     pairs (C's `subst_t { const char *from, *to; }`), and the loop keeps C's algorithm: scan
     each pair over the buffer, replace in place and shift the tail.  */

  /// C 599-601: `typedef struct { const char *from, *to; } subst_t;`
  struct subst_t {
    var from: UnsafePointer<CChar>
    var to: UnsafePointer<CChar>
  }

  /// C 604-621: `static const subst_t key_symbols[] = { … };`.  C's strings are static storage
  /// that lives as long as the program; a Swift string literal's `CChar` pointer would only
  /// live for the initializer call, so each entry is `strdup`ed once, here - the table is
  /// built lazily and then holds the same stable pointers C's literals are.
  fileprivate static let key_symbols: [subst_t] = {
    func s(_ str: String) -> UnsafePointer<CChar> { return UnsafePointer(strdup(str)!) }
    return [
      subst_t(from: s("<backspace>"), to: s("⌫")),
      subst_t(from: s("DEL"), to: s("⌫")),
      subst_t(from: s("<deletechar>"), to: s("⌦")),
      subst_t(from: s("<return>"), to: s("↩")),
      subst_t(from: s("RET"), to: s("↩")),
      subst_t(from: s("<left>"), to: s("←")),
      subst_t(from: s("<right>"), to: s("→")),
      subst_t(from: s("<up>"), to: s("↑")),
      subst_t(from: s("<down>"), to: s("↓")),
      subst_t(from: s("<prior>"), to: s("⇞")),
      subst_t(from: s("<next>"), to: s("⇟")),
      subst_t(from: s("<home>"), to: s("↖")),
      subst_t(from: s("<end>"), to: s("↘")),
      subst_t(from: s("<tab>"), to: s("⇥")),
      subst_t(from: s("TAB"), to: s("⇥")),
      subst_t(from: s("<backtab>"), to: s("⇤")),
    ]
  }()

  /// C 623-654.  C returns a pointer into the Lisp string `build_string` made; the port keeps
  /// the same shape (`SSDATA` of the result), and the buffer C gets from `xmalloc` is a heap
  /// allocation here so it can be freed the way `xfree` frees it.
  fileprivate static func prettify_key(_ key: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>? {
    var k = key
    while k.pointee == 0x20 { k += 1 }

    let len0 = strlen(k)
    let buf = xmalloc(len0 + 1)!.assumingMemoryBound(to: CChar.self)
    _ = memcpy(buf, k, len0 + 1)
    var len = len0
    for sym in key_symbols {
      let fromlen = strlen(sym.from)
      var p: UnsafeMutablePointer<CChar> = buf
      while p < buf + len {
        guard let match = memmem(buf, len, sym.from, fromlen) else { break }
        let m = match.assumingMemoryBound(to: CChar.self)
        let tolen = strlen(sym.to)
        eassert(tolen <= fromlen)
        _ = memcpy(m, sym.to, tolen)
        _ = memmove(m + tolen, m + fromlen, len - (m + fromlen - buf) + 1)
        len -= fromlen - tolen
        p = m + tolen
      }
    }
    let result = build_string(buf)
    xfree(buf)
    return SSDATA(result)
  }

  /// C 657-732: fill the menu from a `widget_value` chain.  C's `NSDictionary` of text
  /// attributes is built in the Cocoa arm (662-702) with the tab stop at the widest name plus
  /// key; the GNUstep `#else` has no attributes.
  @objc func fillWithWidgetValue(_ wvptr: UnsafeMutableRawPointer?) {
    let first_wv = wvptr!.assumingMemoryBound(to: widget_value.self)
    var attributes: NSDictionary? = nil

    /* Cocoa doesn't allow multi-key sequences in its menu display, so
       work around it by using tabs to split the title into two
       columns.  */
    let menuFont = NSFont.menuFont(ofSize: 0)
    let font_attribs = NSDictionary(objects: [menuFont], forKeys: [NSAttributedString.Key.font as NSString])
    var maxNameWidth: CGFloat = 0
    var maxKeyWidth: CGFloat = 0

    /* Determine the maximum width of all menu items. */
    var wv: UnsafeMutablePointer<widget_value>? = UnsafeMutablePointer(mutating: first_wv)
    while wv != nil {
      if !menu_separator_name_p(wv!.pointee.name) {
        let name = String(cString: wv!.pointee.name!)
        let nameSize = (name as NSString).size(withAttributes: font_attribs as? [NSAttributedString.Key: Any])
        maxNameWidth = MAX(maxNameWidth, nameSize.width)
        if wv!.pointee.key != nil {
          wv!.pointee.key = EmacsMenu.prettify_key(wv!.pointee.key!)
          let key = String(cString: wv!.pointee.key!)
          let keySize = (key as NSString).size(withAttributes: font_attribs as? [NSAttributedString.Key: Any])
          maxKeyWidth = MAX(maxKeyWidth, keySize.width)
        }
      }
      wv = wv!.pointee.next
    }

    /* Put some space between the names and keys. */
    let maxWidth = maxNameWidth + maxKeyWidth + 40

    /* Set a right-aligned tab stop at the maximum width, so that the
       key will appear immediately to the left of it. */
    let tab = NSTextTab(textAlignment: .right, location: maxWidth,
                        options: [NSTextTab.OptionKey: Any]())
    let pstyle = NSMutableParagraphStyle()
    pstyle.tabStops = [tab]
    attributes = NSDictionary(objects: [pstyle],
                              forKeys: [NSAttributedString.Key.paragraphStyle as NSString])

    /* clear existing contents */
    self.removeAllItems()

    /* add new contents */
    var wv2: UnsafeMutablePointer<widget_value>? = UnsafeMutablePointer(mutating: first_wv)
    while wv2 != nil {
      let item = self.addItemWithWidgetValue(UnsafeMutableRawPointer(wv2), attributes: attributes)

      if wv2!.pointee.contents != nil {
        let submenu = EmacsMenu(title: item.title)

        self.setSubmenu(submenu, for: item)
        submenu.fillWithWidgetValue(UnsafeMutableRawPointer(wv2!.pointee.contents))
        item.action = nil
      }
      wv2 = wv2!.pointee.next
    }

    needsUpdate = false
  }

  /// C 735-746: adds an empty submenu and returns it.
  @objc func addSubmenuWithTitle(_ title: UnsafePointer<CChar>) -> EmacsMenu {
    let titleStr = String(cString: title)
    let item = self.addItem(withTitle: titleStr, action: nil, keyEquivalent: "")
    let submenu = EmacsMenu(title: titleStr)
    self.setSubmenu(submenu, for: item)
    return submenu
  }

  /// C 748-780: run a menu in popup mode.  The `NS_IMPL_COCOA` arm sets
  /// `allowsContextMenuPlugIns`.
  @objc func runMenuAt(_ p0: NSPoint, forFrame f: UnsafeMutableRawPointer?,
                       keymaps: Bool) -> Lisp_Object {
    var p = p0
    let view = FRAME_NS_VIEW(f)
    var retVal: Int

    /* p = [view convertPoint:p fromView: nil]; */
    p.y = (view?.frame.height ?? 0) - p.y
    let e = view?.window?.currentEvent
    let event = NSEvent.mouseEvent(
      with: .rightMouseDown,
      location: p,
      modifierFlags: [],
      timestamp: e?.timestamp ?? 0,
      windowNumber: view?.window?.windowNumber ?? 0,
      context: nil,
      eventNumber: 0 /* [e eventNumber] */,
      clickCount: 1,
      pressure: 0)

    context_menu_value = -1
    /* Don't let the system add a Services menu here.  */
    self.allowsContextMenuPlugIns = false
    NSMenu.popUpContextMenu(self, with: event!, for: view!)
    retVal = context_menu_value
    context_menu_value = 0
    return retVal > 0
      ? find_and_return_menu_selection(f, keymaps, UnsafeMutableRawPointer(bitPattern: retVal))
      : Qnil()
  }

  /// C 782-819: the delegate method that fills in the help echo.
  @objc func menu(_ menu: NSMenu, willHighlight item: NSMenuItem?) {
    let idx = item?.tag ?? 0
    let f = SELECTED_FRAME()
    let vec = frame_menu_bar_vector(f)
    var help: Lisp_Object

    var frame: Lisp_Object = Qnil()
    withUnsafeMutablePointer(to: &frame) { XSETFRAME($0, f) }

    /* This menu isn't a menubar, so use the pointer to the popup menu
       data.  */
    if context_menu_value != 0 {
      let client_data = UnsafeMutablePointer<Lisp_Object>(bitPattern: idx)

      if client_data != nil {
        help = client_data![Int(MENU_ITEMS_ITEM_HELP())]
      } else {
        help = Qnil()
      }
    }
    /* Just dismiss any help-echo that might already be in progress if
       no menu item will be highlighted.  */
    else if item == nil || idx <= 0 {
      help = Qnil()
    } else {
      if idx >= ASIZE(vec) { return }

      /* Otherwise, get the help data from the menu bar vector.  */
      help = AREF(vec, idx + Int(MENU_ITEMS_ITEM_HELP()))
    }

    popup_activated_flag += 1
    if STRINGP(help) || NILP(help) {
      show_help_echo(help, Qnil(), Qnil(), Qnil())
    }
    popup_activated_flag -= 1
  }
}

// ==========================================================================
// C 878-1086: Context Menu: implementing functions.
// ==========================================================================

/// C 884-1086.  C's `USE_SAFE_ALLOCA` plus `SAFE_ALLOCA (menu_items_used * sizeof
/// *submenu_stack)` is the port's one heap allocation of the file (Swift has no `alloca`); C's
/// `SAFE_FREE ()` at 1084 is the matching `deallocate`, and the block's `NSTRACE
/// ("ns_menu_show")` (902) is the reference's no-op trace.
@_cdecl("ns_menu_show")
func ns_menu_show(_ f: UnsafeMutableRawPointer?, _ x: Int32, _ y: Int32, _ menuflags: Int32,
                  _ title: Lisp_Object, _ error: UnsafeMutablePointer<UnsafePointer<CChar>?>?)
  -> Lisp_Object {
  var p = NSPoint()
  var tem: Lisp_Object = Qnil()
  var specpdl_count: specpdl_ref
  var wv: UnsafeMutablePointer<widget_value>? = nil
  var first_wv: UnsafeMutablePointer<widget_value>? = nil
  var save_wv: UnsafeMutablePointer<widget_value>? = nil
  var prev_wv: UnsafeMutablePointer<widget_value>? = nil
  var submenu_depth = 0
  var first_pane = 1
  var i: Int
  let keymaps = (menuflags & Int32(MENU_KEYMAPS())) != 0

  // C 900: `USE_SAFE_ALLOCA;`
  // C 902: `NSTRACE ("ns_menu_show");`

  block_input()

  p.x = CGFloat(x); p.y = CGFloat(y)

  /* now parse stage 2 as in ns_update_menubar */
  wv = make_widget_value("contextmenu", nil, true, Qnil())
  wv!.pointee.button_type = button_type.BUTTON_TYPE_NONE.rawValue
  first_wv = wv

  let submenu_stack = UnsafeMutablePointer<UnsafeMutablePointer<widget_value>?>.allocate(
    capacity: Int(menu_items_used))

  specpdl_count = SPECPDL_INDEX()

  /* Don't GC due to a mysterious bug.  */
  _ = inhibit_garbage_collection()

  /* Loop over all panes and items, filling in the tree.  */
  i = 0
  while i < Int(menu_items_used) {
    if NILP(menu_items_at(i)) {
      submenu_stack[submenu_depth] = save_wv
      submenu_depth += 1
      save_wv = prev_wv
      prev_wv = nil
      first_pane = 1
      i += 1
    } else if EQ(menu_items_at(i), Qlambda()) {
      prev_wv = save_wv
      submenu_depth -= 1
      save_wv = submenu_stack[submenu_depth]
      first_pane = 0
      i += 1
    } else if EQ(menu_items_at(i), Qt()) && submenu_depth != 0 {
      i += Int(MENU_ITEMS_PANE_LENGTH())
    }
    /* Ignore a nil in the item list.
       It's meaningful only for dialog boxes.  */
    else if EQ(menu_items_at(i), Qquote()) {
      i += 1
    } else if EQ(menu_items_at(i), Qt()) {
      /* Create a new pane.  */
      var pane_name = AREF(menu_items, i + Int(MENU_ITEMS_PANE_NAME()))
      let prefix = AREF(menu_items, i + Int(MENU_ITEMS_PANE_PREFIX()))
      var pane_string: UnsafePointer<CChar>

      if STRINGP(pane_name) && STRING_MULTIBYTE(pane_name) {
        pane_name = ENCODE_MENU_STRING(pane_name)
        ASET(menu_items, i + Int(MENU_ITEMS_PANE_NAME()), pane_name)
      }
      pane_string = NILP(pane_name) ? empty_c_string : UnsafePointer(SSDATA(pane_name)!)
      /* If there is just one top-level pane, put all its items directly
         under the top-level menu.  */
      if menu_items_n_panes == 1 {
        pane_string = empty_c_string
      }

      /* If the pane has a meaningful name,
         make the pane a top-level menu item
         with its items as a submenu beneath it.  */
      if !keymaps && strcmp(pane_string, "") != 0 {
        wv = make_widget_value(pane_string, nil, true, Qnil())
        if save_wv != nil {
          save_wv!.pointee.next = wv
        } else {
          first_wv!.pointee.contents = wv
        }
        if keymaps && !NILP(prefix) {
          wv!.pointee.name = wv!.pointee.name! + 1
        }
        wv!.pointee.button_type = button_type.BUTTON_TYPE_NONE.rawValue
        save_wv = wv
        prev_wv = nil
      } else if first_pane != 0 {
        save_wv = wv
        prev_wv = nil
      }
      first_pane = 0
      i += Int(MENU_ITEMS_PANE_LENGTH())
    } else {
      /* Create a new item within current pane.  */
      var item_name = AREF(menu_items, i + Int(MENU_ITEMS_ITEM_NAME()))
      let enable = AREF(menu_items, i + Int(MENU_ITEMS_ITEM_ENABLE()))
      var descrip = AREF(menu_items, i + Int(MENU_ITEMS_ITEM_EQUIV_KEY()))
      let def = AREF(menu_items, i + Int(MENU_ITEMS_ITEM_DEFINITION()))
      let type = AREF(menu_items, i + Int(MENU_ITEMS_ITEM_TYPE()))
      let selected = AREF(menu_items, i + Int(MENU_ITEMS_ITEM_SELECTED()))
      let help = AREF(menu_items, i + Int(MENU_ITEMS_ITEM_HELP()))

      if STRINGP(item_name) && STRING_MULTIBYTE(item_name) {
        item_name = ENCODE_MENU_STRING(item_name)
        ASET(menu_items, i + Int(MENU_ITEMS_ITEM_NAME()), item_name)
      }

      if STRINGP(descrip) && STRING_MULTIBYTE(descrip) {
        descrip = ENCODE_MENU_STRING(descrip)
        ASET(menu_items, i + Int(MENU_ITEMS_ITEM_EQUIV_KEY()), descrip)
      }

      wv = make_widget_value(SSDATA(item_name), nil, !NILP(enable),
                             STRINGP(help) ? help : Qnil())
      if prev_wv != nil {
        prev_wv!.pointee.next = wv
      } else {
        save_wv!.pointee.contents = wv
      }
      if !NILP(descrip) {
        wv!.pointee.key = SSDATA(descrip)
      }
      /* If this item has a null value,
         make the call_data null so that it won't display a box
         when the mouse is on it.  */
      wv!.pointee.call_data = !NILP(def) ? UnsafeMutableRawPointer(aref_addr(menu_items, i)) : nil

      if NILP(type) {
        wv!.pointee.button_type = button_type.BUTTON_TYPE_NONE.rawValue
      } else if EQ(type, QCtoggle()) {
        wv!.pointee.button_type = button_type.BUTTON_TYPE_TOGGLE.rawValue
      } else if EQ(type, QCradio()) {
        wv!.pointee.button_type = button_type.BUTTON_TYPE_RADIO.rawValue
      } else {
        emacs_abort()
      }

      wv!.pointee.selected = !NILP(selected)

      prev_wv = wv

      i += Int(MENU_ITEMS_ITEM_LENGTH())
    }
  }

  if !NILP(title) {
    let wv_sep = make_widget_value("--", nil, false, Qnil())

    /* Maybe replace this separator with a bitmap or owner-draw item
       so that it looks better.  Having two separators looks odd.  */
    wv_sep!.pointee.next = first_wv!.pointee.contents

    var title_ = title
    if STRING_MULTIBYTE(title_) {
      title_ = ENCODE_MENU_STRING(title_)
    }
    let wv_title = make_widget_value(SSDATA(title_), nil, false, Qnil())
    wv_title!.pointee.button_type = button_type.BUTTON_TYPE_NONE.rawValue
    wv_title!.pointee.next = wv_sep
    first_wv!.pointee.contents = wv_title
  }

  let pmenu = EmacsMenu(title: NILP(title) ? "" : (stringWithLispString(title) as String? ?? ""))
  /* On GNUstep, this call makes menu_items nil for whatever reason
     when displaying a context menu from `context-menu-mode'.  */
  let items = menu_items
  pmenu.fillWithWidgetValue(UnsafeMutableRawPointer(first_wv!.pointee.contents))
  menu_items = items
  free_menubar_widget_value_tree(first_wv)
  popup_activated_flag = 1
  tem = pmenu.runMenuAt(p, forFrame: f, keymaps: keymaps)
  popup_activated_flag = 0
  FRAME_NS_VIEW(SELECTED_FRAME())?.window?.makeKey()
  _ = unbind_to(specpdl_count, Qnil())
  unblock_input()

  submenu_stack.deallocate()
  return tem
}

// ==========================================================================
// C 1089-1254: Toolbar: externally-called functions.
// ==========================================================================

/// C 1095-1114.  The comment at 1107-1108 is C's: the visibility change triggers an animation,
/// which calls `windowDidResize` repeatedly, so `in_animation` brackets it.  `NSTRACE
/// ("free_frame_tool_bar")` (1103) is the reference's no-op trace.
@_cdecl("free_frame_tool_bar")
func free_frame_tool_bar(_ f: UnsafeMutableRawPointer?) {
  let view = FRAME_NS_VIEW(f)

  block_input()

  /* Note: This triggers an animation, which calls windowDidResize
     repeatedly.  */
  ns_output_set_in_animation(f, 1)
  view?.window?.toolbar?.isVisible = false
  ns_output_set_in_animation(f, 0)

  unblock_input()
}

/// C 1116-1237: update the toolbar's contents.  `NSTRACE ("update_frame_tool_bar")` (1124) is the
/// reference's no-op trace; the `#ifdef NS_IMPL_GNUSTEP` arm (1130-1141) is another
/// implementation's (`toolbar clearAll` plus the three `tool_bar_adjusted` nudges).
@_cdecl("update_frame_tool_bar_1")
func update_frame_tool_bar_1(_ f: UnsafeMutableRawPointer?, _ toolbarPtr: UnsafeMutableRawPointer?) {
  let toolbar = Unmanaged<EmacsToolbar>.fromOpaque(toolbarPtr!).takeUnretainedValue()
  var k = 0

  block_input()

  toolbar.clearActive()

  /* Update EmacsToolbar as in GtkUtils, build items list.  */
  var i = 0
  while i < Int(frame_n_tool_bar_items(f)) {
    let toolBarItems = frame_tool_bar_items(f)
    /* C 1146-1147: `#define TOOLPROP(IDX) AREF (f->tool_bar_items,
       i * TOOL_BAR_ITEM_NSLOTS + (IDX))` - the port keeps C's index expression.  */
    func TOOLPROP(_ idx: Int32) -> Lisp_Object {
      return AREF(toolBarItems, i * Int(TOOL_BAR_ITEM_NSLOTS()) + Int(idx))
    }

    let enabled_p = !NILP(TOOLPROP(Int32(TOOL_BAR_ITEM_ENABLED_P())))
    var idx: Int32
    var img_id: Int
    var img: UnsafeMutableRawPointer?
    var image: Lisp_Object
    var labelObj: Lisp_Object
    var helpObj: Lisp_Object

    /* Check if this is a separator.  */
    if EQ(TOOLPROP(Int32(TOOL_BAR_ITEM_TYPE())), Qt()) {
      /* Skip separators.  Newer macOS don't show them, and on
         GNUstep they are wide as a button, thus overflowing the
         toolbar most of the time.  */
      i += 1
      continue
    }

    /* If image is a vector, choose the image according to the
       button state.  */
    image = TOOLPROP(Int32(TOOL_BAR_ITEM_IMAGES()))
    if VECTORP(image) {
      /* NS toolbar auto-computes disabled and selected images.  */
      idx = Int32(TOOL_BAR_IMAGE_ENABLED_SELECTED())
      eassert(ASIZE(image) >= Int(idx))
      image = AREF(image, Int(idx))
    } else {
      idx = -1
    }
    labelObj = TOOLPROP(Int32(TOOL_BAR_ITEM_LABEL()))
    helpObj = TOOLPROP(Int32(TOOL_BAR_ITEM_HELP()))
    if NILP(helpObj) {
      helpObj = TOOLPROP(Int32(TOOL_BAR_ITEM_CAPTION()))
    }

    /* Ignore invalid image specifications.  */
    if !valid_image_p(image) {
      /* Don't log anything, GNUS makes invalid images all the time.  */
      i += 1
      continue
    }

    img_id = lookup_image(f, image, -1)
    img = IMAGE_FROM_ID(f, img_id)
    prepare_image_for_display(f, img)

    if image_load_failed_p(img) || image_pixmap(img) == nil {
      NSLog("Could not prepare toolbar image for display.")
      i += 1
      continue
    }

    toolbar.addDisplayItem(withImage: image_pixmap(img), idx: k, tag: i,
                           labelText: stringWithLispString(labelObj),
                           helpText: stringWithLispString(helpObj),
                           enabled: enabled_p)
    k += 1
    i += 1
  }

  if toolbar.changed() {
    /* Inform app that toolbar has changed.  */
    let dict = toolbar.configuration
    var newDict = dict
    for key in dict.keys {
      let val = dict[key]
      if val is NSArray {
        newDict[key] = toolbar.toolbarDefaultItemIdentifiers(toolbar).map { $0.rawValue }
        break
      }
    }
    toolbar.setConfiguration(newDict)
  }

  toolbar.isVisible = true
  unblock_input()
}

/// C 1239-1254.
@_cdecl("update_frame_tool_bar")
func update_frame_tool_bar(_ f: UnsafeMutableRawPointer?) {
  let window = FRAME_NS_VIEW(f)?.window
  let toolbar = window?.toolbar

  if toolbar == nil {
    createToolbar(Unmanaged.passUnretained(window!).toOpaque(), f)
    return
  }

  if window == nil || toolbar == nil { return }

  update_frame_tool_bar_1(f, Unmanaged.passUnretained(toolbar!).toOpaque())
}

// ==========================================================================
// C 1257-1409: Toolbar: class implementation (EmacsToolbar).
// ==========================================================================

/// C 1263-1409 with the ivars of src/nsterm.h 565-575.
class EmacsToolbar: NSToolbar, NSToolbarDelegate {
  /// C: `EmacsView *emacsView;` (src/nsterm.h 570) - another lane file's class, so the port
  /// keeps it as the object C sends `setTarget:`.
  var emacsView: AnyObject?
  var identifierToItem: NSMutableDictionary?
  var activeIdentifiers: NSMutableArray?
  var prevIdentifiers: NSArray?
  var enablement: UInt = 0
  var prevEnablement: UInt = 0

  /// C 1265-1279: C's `-initForView:withIdentifier:` sends `[super initWithIdentifier:]`.
  init(forView view: AnyObject?, withIdentifier identifier: String) {
    emacsView = view
    super.init(identifier: identifier)
    self.displayMode = .iconOnly
    self.sizeMode = .small
    self.delegate = self
    identifierToItem = NSMutableDictionary(capacity: 10)
    activeIdentifiers = NSMutableArray(capacity: 8)
    prevIdentifiers = nil
    prevEnablement = 0
    enablement = 0
  }

  /// C 1281-1289: C's `-dealloc` releases the three containers and calls `[super dealloc]`,
  /// which is what ARC does for them; the `NSTRACE ("[EmacsToolbar dealloc]")` (1283) is the
  /// reference's no-op trace.
  deinit {
  }

  /// C 1291-1300.
  @objc func clearActive() {
    /* C: `NSTRACE ("[EmacsToolbar clearActive]")` (1293).  */
    prevIdentifiers = activeIdentifiers?.copy() as? NSArray
    activeIdentifiers?.removeAllObjects()
    prevEnablement = enablement
    enablement = 0
  }

  /// C 1302-1309.
  @objc func clearAll() {
    /* C: `NSTRACE ("[EmacsToolbar clearAll]")` (1304).  */
    self.clearActive()
    while (self.items.count) > 0 {
      self.removeItem(at: 0)
    }
  }

  /// C 1311-1317.
  @objc func changed() -> Bool {
    /* C: `NSTRACE ("[EmacsToolbar changed]")` (1313).  */
    return (activeIdentifiers?.isEqual(to: prevIdentifiers ?? [])) == true
      && enablement == prevEnablement ? false : true
  }

  /// C 1319-1356.  The `NS_IMPL_GNUSTEP` insertion arm (1347-1349) is another implementation's.
  @objc func addDisplayItem(withImage img: UnsafeMutableRawPointer?, idx: Int, tag: Int,
                            labelText label: NSString?, helpText help: NSString?,
                            enabled: Bool) {
    /* C: `NSTRACE ("[EmacsToolbar addDisplayItemWithImage: ...]")` (1326).  */

    /* 1) come up w/identifier */
    let imageObj = Unmanaged<AnyObject>.fromOpaque(img!).takeUnretainedValue()
    let identifier = "\(UInt(bitPattern: imageObj.hash))\(label ?? "")"
    activeIdentifiers?.add(identifier)

    /* 2) create / reuse item */
    var item = identifierToItem?.object(forKey: identifier) as? NSToolbarItem
    if item == nil {
      item = NSToolbarItem(itemIdentifier: NSToolbarItem.Identifier(identifier))
      item!.image = imageObj as? NSImage
      item!.label = label as String? ?? ""
      item!.toolTip = help as String?
      item!.target = emacsView
      item!.action = NSSelectorFromString("toolbarClicked:")
      identifierToItem?.setObject(item!, forKey: identifier as NSString)
    }

    item!.tag = tag
    item!.isEnabled = enabled

    /* 3) update state */
    enablement = (enablement << 1) | (enabled ? 1 : 0)
  }

  /// C 1358-1363: C overrides the superclass' implementation, which sets every item enabled.
  override func validateVisibleItems() {
    /* C: `NSTRACE ("[EmacsToolbar validateVisibleItems]")` (1362).  */
  }

  /* delegate methods */

  /// C 1368-1376.
  @objc func toolbar(_ toolbar: NSToolbar, itemForItemIdentifier itemIdentifier: NSToolbarItem.Identifier,
                     willBeInsertedIntoToolbar flag: Bool) -> NSToolbarItem? {
    /* C: `NSTRACE ("[EmacsToolbar toolbar: ...]")` (1372).  */
    return identifierToItem?.object(forKey: itemIdentifier.rawValue) as? NSToolbarItem
  }

  /// C 1378-1384.
  @objc func toolbarDefaultItemIdentifiers(_ toolbar: NSToolbar) -> [NSToolbarItem.Identifier] {
    /* C: `NSTRACE ("[EmacsToolbar toolbarDefaultItemIdentifiers:]")` (1380).  */
    return (activeIdentifiers as? [String] ?? []).map { NSToolbarItem.Identifier($0) }
  }

  /// C 1386-1394: for the configuration palette (not yet supported).  C's commented-out
  /// `return [identifierToItem allKeys];` (1393) stays a comment.
  @objc func toolbarAllowedItemIdentifiers(_ toolbar: NSToolbar) -> [NSToolbarItem.Identifier] {
    /* C: `NSTRACE ("[EmacsToolbar toolbarAllowedItemIdentifiers:]")` (1389).  */
    return (activeIdentifiers as? [String] ?? []).map { NSToolbarItem.Identifier($0) }
  }

  /* C 1396-1401: `- (void)setVisible:(BOOL)shown { NSTRACE ("[EmacsToolbar
     setVisible:%d]", shown); [super setVisible:shown]; }`.  The override only forwards to the
     superclass, and Swift spells `setVisible:` as NSToolbar's `isVisible` property, so the
     port records the override instead of writing a method that would not be called.  */

  /* C 1404-1407: C's commented-out optional delegate methods (`toolbarWillAddItem:`,
     `toolbarDidRemoveItem:`, `toolbarSelectableItemIdentifiers:`) stay comments.  */
}

// ==========================================================================
// C 1413-1538: Tooltip: class implementation (EmacsTooltip).
// ==========================================================================

/// C 1419-1421's prose: NeXTstep does not provide enough control over tooltip display.
/// The class is the "Message / question windows" section's tooltip (src/nsterm.h 632-651), with
/// the Cocoa protocol conformance.
class EmacsTooltip: NSObject, NSWindowDelegate {
  var win: NSWindow?
  var textField: NSTextField?
  var timer: Timer?

  /// C 1423-1456.  C's `-init` never sends `[super init]` (NSObject's is a no-op); Swift
  /// requires the call, so it stands first and changes nothing.
  override init() {
    super.init()
    let col = NSColor(calibratedRed: 1.0, green: 1.0, blue: 0.792, alpha: 0.95)
    let font = NSFont.toolTipsFont(ofSize: 0)
    /* C: `[font screenFont]`, the deprecated spelling of
       `screenFontWithRenderingMode: NSFontDefaultRenderingMode` (measured: the SDK renames it).  */
    let sfont = font.screenFont(with: .defaultRenderingMode)
    let height = sfont.ascender - sfont.descender
    /* [font boundingRectForFont].size.height; */
    let r = NSRect(x: 0, y: 0, width: 100, height: CGFloat(Int(height) + 6))

    textField = NSTextField(frame: r)
    textField?.font = font
    textField?.backgroundColor = col

    textField?.isEditable = false
    textField?.isSelectable = false
    textField?.isBordered = false
    textField?.isBezeled = false
    textField?.drawsBackground = true

    win = NSWindow(contentRect: textField!.frame, styleMask: [], backing: .buffered, defer: true)
    win?.hasShadow = true
    win?.isReleasedWhenClosed = false
    win?.delegate = self
    win?.contentView?.addSubview(textField!)
    /* [win setBackgroundColor: col]; */
    win?.isOpaque = false
  }

  /// C 1458-1464: `[win close]` is a side effect ARC does not imply, so the port keeps it and
  /// drops the three `release` sends ARC performs.
  deinit {
    win?.close()
  }

  /// C 1466-1478.
  @objc func setText(_ text: UnsafeMutablePointer<CChar>?) {
    let str = String(cString: text!)
    var r = textField!.frame

    textField?.stringValue = str
    let tooltipDims = textField!.cell!.cellSize

    r.size.width = tooltipDims.width
    r.size.height = tooltipDims.height
    textField?.frame = r
  }

  /// C 1480-1483.
  @objc func setBackgroundColor(_ col: NSColor?) {
    textField?.backgroundColor = col
  }

  /// C 1485-1488.
  @objc func setForegroundColor(_ col: NSColor?) {
    textField?.textColor = col
  }

  /// C 1490-1505: `[timer retain]` (1504) is ARC's own reference here.
  @objc func showAtX(_ x: Int32, Y y: Int32, for seconds: Int32) {
    var wr = win!.frame

    wr.origin = NSPoint(x: CGFloat(x), y: CGFloat(y))
    wr.size = textField!.frame.size

    win?.setFrame(wr, display: true)
    win?.level = .popUpMenu
    win?.orderFront(self)
    win?.display()
    timer = Timer.scheduledTimer(timeInterval: TimeInterval(seconds), target: self,
                                 selector: #selector(hide), userInfo: nil, repeats: false)
  }

  /// C 1507-1514.
  @objc func moveTo(_ screen_point: NSPoint) {
    win?.setFrame(NSRect(x: screen_point.x, y: screen_point.y,
                         width: self.frame().size.width, height: self.frame().size.height),
                  display: true)
  }

  /// C 1516-1526.
  @objc func hide() {
    win?.close()
    if let t = timer {
      if t.isValid {
        t.invalidate()
      }
      timer = nil
    }
  }

  /// C 1528-1531.
  @objc func isActive() -> Bool {
    return timer != nil
  }

  /// C 1533-1536.
  @objc func frame() -> NSRect {
    return textField!.frame
  }
}

// ==========================================================================
// C 1542-1624: Popup Dialog: implementing functions.
// ==========================================================================

/// C 1548-1564.  C's `static void pop_down_menu (void *arg)` is the unwind-protect function
/// `ns_popup_dialog` installs; Swift passes a non-capturing function of the same shape to
/// `record_unwind_protect_ptr`, and the `NS_IMPL_COCOA` arm's `makeKeyWindow` is the
/// `NSWindow.makeKey` spelling.
fileprivate func pop_down_menu(_ arg: UnsafeMutableRawPointer?) {
  let panel = Unmanaged<EmacsDialogPanel>.fromOpaque(arg!).takeUnretainedValue()

  if popup_activated_flag != 0 {
    popup_activated_flag = 0
    panel.close()
    /* For some reason this is required on macOS, or the selected
       frame gets the keyboard focus but doesn't become
       highlighted.  */
    FRAME_NS_VIEW(SELECTED_FRAME())?.window?.makeKey()
  }
}

/// C 1566-1624.  `NSTRACE ("ns_popup_dialog")` (1576) is the reference's no-op trace.
@_cdecl("ns_popup_dialog")
func ns_popup_dialog(_ f: UnsafeMutableRawPointer?, _ header: Lisp_Object, _ contentsIn: Lisp_Object)
  -> Lisp_Object {
  var contents = contentsIn
  var tem: Lisp_Object = Qnil()
  var title: Lisp_Object
  var p = NSPoint()
  var is_question: Bool
  var error_name: UnsafePointer<CChar>? = nil
  var specpdl_count: specpdl_ref

  specpdl_count = SPECPDL_INDEX()

  is_question = NILP(header)
  check_window_system(f)

  p.x = CGFloat(Int32(frame_left_pos(f))
                + (FRAME_COLUMN_WIDTH(f) * FRAME_COLS(f)) / 2)
  p.y = CGFloat(Int32(frame_top_pos(f))
                + (FRAME_LINE_HEIGHT(f) * FRAME_LINES(f)) / 2)

  title = Fcar(contents)
  CHECK_STRING(title)

  if NILP(Fcar(Fcdr(contents))) {
    /* No buttons specified, add an "Ok" button so users can pop down
       the dialog.  */
    contents = list2(title, Fcons(build_string("Ok"), Qt()))
  }

  record_unwind_protect_void(unuse_menu_items)
  list_of_panes(list1(contents))

  block_input()
  let dialog = EmacsDialogPanel(title: SSDATA(title), isQuestion: is_question)

  dialog.processMenuItems(menu_items, used: Int(menu_items_used), withErrorOutput: &error_name)
  dialog.resizeBoundsPriorToDisplay()
  unblock_input()

  if error_name != nil {
    _ = unbind_to(specpdl_count, Qnil())
    discard_menu_items()
    dialog.close()
    error("%s", error_name!)
  }

  record_unwind_protect_ptr(pop_down_menu, Unmanaged.passUnretained(dialog).toOpaque())
  popup_activated_flag = 1
  tem = dialog.runDialogAt(p)
  _ = unbind_to(specpdl_count, Qnil())

  /* This must come *after* unuse_menu_items.  */
  discard_menu_items()
  return tem
}

// ==========================================================================
// C 1627-1999: Popup Dialog: class implementation (FlippedView, EmacsDialogPanel).
// ==========================================================================

/// C 1633-1643: `@interface FlippedView : NSView` with just `-isFlipped`.
class FlippedView: NSView {
  override var isFlipped: Bool { return true }
}

/* C 1647-1650: the four constants C defines inside the `@implementation EmacsDialogPanel`:

      #define SPACER 8.0
      #define ICONSIZE 64.0
      #define TEXTHEIGHT 20.0
      #define MINCELLWIDTH 90.0

   They are fileprivate here, next to the class that uses them.  */
fileprivate let SPACER: CGFloat = 8.0
fileprivate let ICONSIZE: CGFloat = 64.0
fileprivate let TEXTHEIGHT: CGFloat = 20.0
fileprivate let MINCELLWIDTH: CGFloat = 90.0

/// C 1645-1999 with the ivars of src/nsterm.h 602-610.
class EmacsDialogPanel: NSPanel {
  var command: NSTextField?
  /// C: (src/nsterm.h 605) the ivar `title` (`NSTextField *title;`) - renamed `title_`
  /// because Swift's
  /// `NSWindow.title` is a `String` property of the same name (the nsimage.swift
  /// `prev_buffer_` shape); C's `[self setTitle: @""]` (1689) is `self.title` below and its
  /// ivar reads (`title setStringValue:`, `[title sizeToFit]`) are `title_`'s.
  var title_: NSTextField?
  var matrix: NSMatrix?
  var rows: Int32 = 0
  var cols: Int32 = 1
  var timer_fired: Bool = false
  var window_closed: Bool = false
  var dialog_return: Lisp_Object = Qundefined()

  /// C 1652-1743: C overrides NSWindow's designated initializer.  The `NSTRACE`s inside the
  /// other methods are the reference's no-op traces.
  override init(contentRect: NSRect, styleMask: NSWindow.StyleMask,
                backing backingType: NSWindow.BackingStoreType, defer flag: Bool) {
    var area = NSRect()
    let spacing = NSSize(width: SPACER, height: SPACER)

    dialog_return = Qundefined()
    area.origin.x = 3 * SPACER
    area.origin.y = 2 * SPACER
    area.size.width = ICONSIZE
    area.size.height = ICONSIZE
    let img = NSImage(named: "NSApplicationIcon")!.copy() as! NSImage
    img.size = NSSize(width: ICONSIZE, height: ICONSIZE)
    let imgView = NSImageView(frame: area)
    imgView.image = img
    imgView.isEditable = false

    let aStyle: NSWindow.StyleMask = [.titled, .closable, .utilityWindow]
    let flag_ = true
    rows = 0
    cols = 1
    super.init(contentRect: contentRect, styleMask: aStyle, backing: backingType, defer: flag_)
    let contentView = FlippedView(frame: self.contentView!.frame)

    self.contentView = contentView

    self.contentView?.autoresizesSubviews = true

    self.contentView?.addSubview(imgView)
    self.title = ""

    area.origin.x += ICONSIZE + 2 * SPACER
    /* area.origin.y   = TEXTHEIGHT; ICONSIZE/2-10+SPACER; */
    area.size.width = 400
    area.size.height = TEXTHEIGHT
    command = NSTextField(frame: area)
    self.contentView?.addSubview(command!)
    command?.stringValue = ns_app_name as String? ?? ""
    command?.drawsBackground = false
    command?.isBezeled = false
    command?.isSelectable = false
    command?.font = NSFont.boldSystemFont(ofSize: 13.0)

    /* C 1703-1710's commented-out separator box stays a comment:

       area.origin.x = ICONSIZE+2*SPACER;
       area.origin.y   = TEXTHEIGHT + 2*SPACER;
       area.size.width = 400;
       area.size.height= 2;
       tem = [[[NSBox alloc] initWithFrame: area] autorelease];
       [[self contentView] addSubview: tem];
       [tem setTitlePosition: NSNoTitle];
       [tem setAutoresizingMask: NSViewWidthSizable]; */

    /* area.origin.x = ICONSIZE+2*SPACER; */
    area.origin.y += TEXTHEIGHT + SPACER
    area.size.width = 400
    area.size.height = TEXTHEIGHT
    title_ = NSTextField(frame: area)
    self.contentView?.addSubview(title_!)
    title_?.drawsBackground = false
    title_?.isBezeled = false
    title_?.isSelectable = false
    title_?.font = NSFont.systemFont(ofSize: 11.0)

    let cell = NSButtonCell(textCell: "")
    cell.isBordered = false
    cell.isEnabled = false
    cell.setCellAttribute(.cellIsInsetButton, to: 8)
    cell.bezelStyle = .rounded

    matrix = NSMatrix(frame: contentRect, mode: .highlightModeMatrix, prototype: cell,
                      numberOfRows: 0, numberOfColumns: 1)
    matrix?.setFrameOrigin(NSPoint(x: area.origin.x, y: area.origin.y + (TEXTHEIGHT + 3 * SPACER)))
    matrix?.intercellSpacing = spacing

    self.contentView?.addSubview(matrix!)
    self.isReleasedWhenClosed = true
    self.hidesOnDeactivate = true
  }

  /// C 1746-1751.
  @objc func windowShouldClose(_ sender: Any?) -> Bool {
    window_closed = true
    NSApp.stop(self)
    return false
  }

  /// C 1753-1756: C's `-dealloc` only sends `[super dealloc]`, which ARC does; nothing to
  /// transcribe.
  deinit {
  }

  /// C 1758-1800: fill the panel from the Lisp menu items.
  @objc func processMenuItems(_ menu_items_: Lisp_Object, used menu_items_used_: Int,
                              withErrorOutput error_name: UnsafeMutablePointer<UnsafePointer<CChar>?>?) {
    var nb_buttons: Int32 = 0
    var row: Int32 = 0

    var i: Int = Int(MENU_ITEMS_PANE_LENGTH())
    error_name!.pointee = nil

    /* Loop over all panes and items, filling in the tree.  */
    while i < menu_items_used_ {
      let item_name = AREF(menu_items_, i + Int(MENU_ITEMS_ITEM_NAME()))
      let enable = AREF(menu_items_, i + Int(MENU_ITEMS_ITEM_ENABLE()))

      if NILP(item_name) {
        error_name!.pointee = err_submenu_in_dialog_items
        return
      }

      if EQ(item_name, Qquote()) {
        /* This is the boundary between elements on the left and those
           on the right, but that boundary is currently not handled on
           NS.  */
        i += Int(MENU_ITEMS_ITEM_LENGTH())
        continue
      }

      if nb_buttons > 9 {
        error_name!.pointee = err_too_many_dialog_items
        return
      }

      self.addButton(SSDATA(item_name), value: Int(bitPattern: aref_addr(menu_items_, i)),
                     row: row, enable: !NILP(enable))
      row += 1

      i += Int(MENU_ITEMS_ITEM_LENGTH())
      nb_buttons += 1
    }
  }

  /// C 1803-1821.
  @objc func addButton(_ str: UnsafeMutablePointer<CChar>?, value tag: Int, row: Int32,
                       enable: Bool) {
    if row >= rows {
      matrix?.addRow()
      rows += 1
    }

    let cell = matrix?.cell(atRow: Int(row), column: Int(cols) - 1)
    cell?.target = self
    cell?.action = #selector(clicked(_:))
    cell?.title = String(cString: str!)
    cell?.tag = tag
    cell?.isBordered = true
    cell?.isEnabled = true
  }

  /// C 1824-1837.
  @objc func addString(_ str: UnsafeMutablePointer<CChar>?, row: Int32) {
    if row >= rows {
      matrix?.addRow()
      rows += 1
    }
    let cell = matrix?.cell(atRow: Int(row), column: Int(cols) - 1)
    cell?.title = String(cString: str!)
    cell?.isBordered = true
    cell?.isEnabled = false
  }

  /// C 1840-1844.
  @objc func addSplit() {
    matrix?.addColumn()
    cols += 1
  }

  /// C 1847-1862.
  @objc func clicked(_ sender: Any?) {
    let sellist = (sender as? NSMatrix)?.selectedCells

    if (sellist?.count ?? 0) < 1 { return }

    let seltag = sellist![0].tag
    let selarray = UnsafeMutablePointer<Lisp_Object>(bitPattern: seltag)
    dialog_return = selarray![Int(MENU_ITEMS_ITEM_VALUE())]
    NSApp.stop(self)
  }

  /// C 1865-1880.  C sends `[super init]`, whose NSWindow implementation dispatches to this
  /// class's overridden `initWithContentRect:…`; Swift has no such dispatch through `super`, so
  /// the port delegates to its own designated initializer with the same rectangle and style
  /// mask C's override ends up using, then runs C's body - which is what C's dynamic dispatch
  /// amounts to.
  convenience init(title title_string: UnsafeMutablePointer<CChar>?, isQuestion is_question: Bool) {
    self.init(contentRect: NSRect.zero, styleMask: [.titled, .closable, .utilityWindow],
              backing: .buffered, defer: true)

    if title_string != nil {
      title_?.stringValue = String(cString: title_string!)
    }

    if is_question {
      command?.stringValue = "Question"
    } else {
      command?.stringValue = "Information"
    }
  }

  /// C 1882-1935.
  @objc func resizeBoundsPriorToDisplay() {
    if cols == 1 && rows > 1 {
      matrix?.addColumn()
      var i: Int32 = 0
      while i < rows / 2 {
        matrix?.putCell(matrix!.cell(atRow: Int((rows + 1) / 2), column: 0)!,
                        atRow: Int(i), column: 1)
        matrix?.removeRow(Int((rows + 1) / 2))
        i += 1
      }
    }

    matrix?.sizeToFit()

    var csize = matrix!.cellSize
    if csize.width < MINCELLWIDTH {
      csize.width = MINCELLWIDTH
      matrix?.cellSize = csize
      matrix?.sizeToCells()
    }

    title_?.sizeToFit()
    command?.sizeToFit()

    var t = matrix!.frame
    var r = title_!.frame
    if r.size.width + r.origin.x > t.size.width + t.origin.x {
      t.origin.x = r.origin.x
      t.size.width = r.size.width
    }

    r = command!.frame
    if r.size.width + r.origin.x > t.size.width + t.origin.x {
      t.origin.x = r.origin.x
      t.size.width = r.size.width
    }

    r = self.frame
    let s = self.contentView!.frame
    r.size.width += (t.origin.x + t.size.width + 2 * SPACER - s.size.width)
    r.size.height += (t.origin.y + t.size.height + SPACER - s.size.height)
    self.setFrame(r, display: false)
  }

  /// C 1937-1955: the timer callback that posts a real event so the modal loop's `stop` is
  /// seen.  C's `[NSApp context]` is the deprecated `NSApplication.context`, which the SDK's
  /// Swift interface does not offer any more; the port passes `nil`, the same "no graphics
  /// context" the event carries on a modern macOS (recorded in the ledger).
  @objc func timeout_handler(_ timedEntry: Timer) {
    let nxev = NSEvent.otherEvent(with: .applicationDefined,
                                  location: NSPoint(x: 0, y: 0),
                                  modifierFlags: [],
                                  timestamp: 0,
                                  windowNumber: NSApp.mainWindow?.windowNumber ?? 0,
                                  context: nil,
                                  subtype: 0,
                                  data1: 0,
                                  data2: 0)

    timer_fired = true
    /* We use stop because stopModal/abortModal out of the main loop
       does not seem to work in 10.6.  But as we use stop we must send a
       real event so the stop is seen and acted upon.  */
    NSApp.stop(self)
    NSApp.postEvent(nxev!, atStart: false)
  }

  /// C 1957-1997.
  @objc func runDialogAt(_ p: NSPoint) -> Lisp_Object {
    var ret: Lisp_Object = Qundefined()

    while popup_activated_flag != 0 {
      var tmo: Timer? = nil
      let next_time = timer_check()

      if timespec_valid_p(next_time) {
        let time = timespectod(next_time)
        tmo = Timer(timeInterval: time, target: self,
                    selector: #selector(timeout_handler(_:)), userInfo: 0, repeats: false)
        RunLoop.current.add(tmo!, forMode: .modalPanel)
      }

      timer_fired = false
      dialog_return = Qundefined()
      NSApp.runModal(for: self)
      ret = dialog_return

      if !timer_fired {
        if tmo != nil {
          tmo!.invalidate() /* Cancels timer.  */
        }

        break
      }
    }

    if EQ(ret, Qundefined()) && window_closed {
      /* Make close button pressed equivalent to C-g.  */
      _ = quit()
    }

    return ret
  }
}

// ==========================================================================
// C 2002-2037: Lisp definitions.
// ==========================================================================

/// C 2008-2014: `DEFUN ("ns-reset-menu", Fns_reset_menu, Sns_reset_menu, 0, 0, 0,
/// doc: /* Cause the NS menu to be re-calculated.  */)`.
@_cdecl("Fns_reset_menu")
func Fns_reset_menu() -> Lisp_Object {
  set_frame_menubar(SELECTED_FRAME(), false)
  return Qnil()
}
fileprivate let lname_ns_reset_menu: StaticString = "ns-reset-menu"
fileprivate var Sns_reset_menu: Aligned_Lisp_Subr =
  make_aligned_subr(lname_ns_reset_menu, 0, 0,
                    unsafeBitCast(Fns_reset_menu, to: UnsafeMutableRawPointer.self))

/// C 2017-2022: `DEFUN ("menu-or-popup-active-p", Fmenu_or_popup_active_p,
/// Smenu_or_popup_active_p, 0, 0, 0, doc: /* SKIP: real doc in xmenu.c.  */)`.
@_cdecl("Fmenu_or_popup_active_p")
func Fmenu_or_popup_active_p() -> Lisp_Object {
  return popup_activated() != 0 ? Qt() : Qnil()
}
fileprivate let lname_menu_or_popup_active_p: StaticString = "menu-or-popup-active-p"
fileprivate var Smenu_or_popup_active_p: Aligned_Lisp_Subr =
  make_aligned_subr(lname_menu_or_popup_active_p, 0, 0,
                    unsafeBitCast(Fmenu_or_popup_active_p, to: UnsafeMutableRawPointer.self))

/// C 1776/1788: `*error_name = "Submenu in dialog items"` / `"Too many dialog items"` - the
/// pointer C stores is a static literal, so the port keeps one stable C string for each (the
/// `empty_c_string` shape); `*error_name = NULL` (1766) is `nil` in the same pointee.
fileprivate let err_submenu_in_dialog_items: UnsafePointer<CChar> =
  UnsafePointer(strdup("Submenu in dialog items")!)
fileprivate let err_too_many_dialog_items: UnsafePointer<CChar> =
  UnsafePointer(strdup("Too many dialog items")!)

/// C 2030-2037.  `DEFSYM (Qdebug_on_next_call, "debug-on-next-call")` is empty in the
/// reference (src/lisp.h 2394), so the call site is recorded, not called.
@_cdecl("syms_of_nsmenu")
func syms_of_nsmenu() {
  defsubr(&Sns_reset_menu)
  defsubr(&Smenu_or_popup_active_p)

  // C 2036: `DEFSYM (Qdebug_on_next_call, "debug-on-next-call");`
}
