// femacs: port of src/nsxwidget.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): declarations only.  `nsxwidget.h` is the Cocoa half of
// xwidget/webkit: the 21 prototypes `src/nsxwidget.m` implements.  Its two C includers are
// `src/xwidget.c` 51 (behind `NS_IMPL_COCOA`) and `src/xwidget.h` 41-44 (behind
// `NS_IMPL_COCOA && __OBJC__`).  (The header's own comment at C 23 says it "can be included
// from non-objc files through 'xwidget.h'"; in this revision `xwidget.h` reaches it only in
// that `__OBJC__` arm, and the non-Objective-C includer is `xwidget.c` 51.)  The frozen
// configuration has no `HAVE_NS`, no `HAVE_XWIDGETS` and no `NS_IMPL_COCOA`
// (`docs/baseline/config/darwin-arm64.macros.tsv`), so no translation unit of this target
// includes the header and `src/nsxwidget.m` is not built
// (`docs/baseline/frozen-build-sources.txt` lists no `.m`); the header's own text is
// unconditional, so the mirror carries it - the `src/w32select.h` reading, status 3.158.
//
// Dead text under the frozen configuration (PORTING.md P-08; `tools/check-cond-eval.py
// src/nsxwidget.h` -> `4 conditions, 0 unknown to the evaluator, 0 disagreements`; `__OBJC__`
// is undefined because this configuration compiles C, not Objective-C): C 24-26
// (`#import <AppKit/NSView.h>` and its guard), C 47-51 (`@interface XwWindow`) and C 60-65
// (`@interface XvWindow`) - Objective-C class declarations with no Rust counterpart.  C 20-21
// is the include guard; C 28-43, C 53-56 and C 67-81 are live text.
//
// Degraded names (PORTING.md P-55/P-72 rule 3): `struct xwidget`, `struct xwidget_view` and
// `struct glyph_string` have no mirror in the port (`docs/port-inventory.md` still has
// `src/xwidget.h` as `todo`, and `include/dispextern.rs` carries the glyph datatype but not
// `struct glyph_string`), so every pointer to one is `*mut c_void` and carries the C text in
// its `/// C:` line; C's pointer depth is kept.  `Lisp_Object` is named but not declared, the
// `w32inevt.rs` reading of PORTING.md 0.1-9: C 29 includes `lisp.h`, and the port's lisp.h
// mirror is the one header every translation unit reaches through its own includes (`spikes/
// ns_inc_probe.rs` supplies it the way a unit does).  `bool` is C's `bool`, `double` is
// `f64` and `int` is `c_int` (PORTING.md 1.1).

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const NSXWIDGET_H_INCLUDED: bool = true;

/* C 32-43: the webkit functions `src/nsxwidget.m` defines.  They stay at this mirror's top
   level (the `w32select.rs` shape rather than `haiku_support.rs`'s module): the unit that
   defines them is the Swift port of `src/nsxwidget.m` (`platform/macos/nsxwidget.swift`),
   which never `include!`s this mirror, so there is no same-crate definition for a declaration
   to collide with.  */
unsafe extern "C" {
  /// C: bool nsxwidget_is_web_view (struct xwidget *xw);
  pub fn nsxwidget_is_web_view (xw: *mut c_void) -> bool;
  /// C: Lisp_Object nsxwidget_webkit_uri (struct xwidget *xw);
  pub fn nsxwidget_webkit_uri (xw: *mut c_void) -> Lisp_Object;
  /// C: Lisp_Object nsxwidget_webkit_title (struct xwidget *xw);
  pub fn nsxwidget_webkit_title (xw: *mut c_void) -> Lisp_Object;
  /// C: void nsxwidget_webkit_goto_uri (struct xwidget *xw, const char *uri);
  pub fn nsxwidget_webkit_goto_uri (xw: *mut c_void, uri: *const c_char);
  /// C: void nsxwidget_webkit_goto_history (struct xwidget *xw, int rel_pos);
  pub fn nsxwidget_webkit_goto_history (xw: *mut c_void, rel_pos: c_int);
  /// C: double nsxwidget_webkit_estimated_load_progress(struct xwidget *xw);
  pub fn nsxwidget_webkit_estimated_load_progress (xw: *mut c_void) -> f64;
  /// C: void nsxwidget_webkit_stop_loading (struct xwidget *xw);
  pub fn nsxwidget_webkit_stop_loading (xw: *mut c_void);
  /// C: void nsxwidget_webkit_zoom (struct xwidget *xw, double zoom_change);
  pub fn nsxwidget_webkit_zoom (xw: *mut c_void, zoom_change: f64);
  /// C: void nsxwidget_webkit_execute_script (struct xwidget *xw, const char *script,
  ///                                          Lisp_Object fun);
  pub fn nsxwidget_webkit_execute_script (xw: *mut c_void, script: *const c_char,
                                          fun: Lisp_Object);
}

/* C 53-56: the model functions.  */
unsafe extern "C" {
  /// C: void nsxwidget_init (struct xwidget *xw);
  pub fn nsxwidget_init (xw: *mut c_void);
  /// C: void nsxwidget_kill (struct xwidget *xw);
  pub fn nsxwidget_kill (xw: *mut c_void);
  /// C: void nsxwidget_resize (struct xwidget *xw);
  pub fn nsxwidget_resize (xw: *mut c_void);
  /// C: Lisp_Object nsxwidget_get_size (struct xwidget *xw);
  pub fn nsxwidget_get_size (xw: *mut c_void) -> Lisp_Object;
}

/* C 67-81: the view functions.  */
unsafe extern "C" {
  /// C: void nsxwidget_init_view (struct xwidget_view *xv, struct xwidget *xww,
  ///                              struct glyph_string *s, int x, int y);
  pub fn nsxwidget_init_view (xv: *mut c_void, xww: *mut c_void, s: *mut c_void,
                              x: c_int, y: c_int);
  /// C: void nsxwidget_delete_view (struct xwidget_view *xv);
  pub fn nsxwidget_delete_view (xv: *mut c_void);
  /// C: void nsxwidget_show_view (struct xwidget_view *xv);
  pub fn nsxwidget_show_view (xv: *mut c_void);
  /// C: void nsxwidget_hide_view (struct xwidget_view *xv);
  pub fn nsxwidget_hide_view (xv: *mut c_void);
  /// C: void nsxwidget_resize_view (struct xwidget_view *xv, int widget, int height);
  pub fn nsxwidget_resize_view (xv: *mut c_void, widget: c_int, height: c_int);
  /// C: void nsxwidget_move_view (struct xwidget_view *xv, int x, int y);
  pub fn nsxwidget_move_view (xv: *mut c_void, x: c_int, y: c_int);
  /// C: void nsxwidget_move_widget_in_view (struct xwidget_view *xv, int x, int y);
  pub fn nsxwidget_move_widget_in_view (xv: *mut c_void, x: c_int, y: c_int);
  /// C: void nsxwidget_set_needsdisplay (struct xwidget_view *xv);
  pub fn nsxwidget_set_needsdisplay (xv: *mut c_void);
}
