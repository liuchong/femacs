// femacs: port of src/nsxwidget.m @ fd7afced403.
//
// The NS Cocoa half of xwidget and WebKit: one `WKWebView` per model (the reference's
// comment at src/nsxwidget.m 30-46 explains why WebKit2 rules out the GTK lane's offscreen
// sharing), its delegate methods, and the C entry points src/xwidget.h declares.
//
// Swift has no `#include`, so this unit carries the C environment it uses, the way the other
// Swift lanes do (nsimage.swift's header describes the same three kinds):
//
//   * `@_silgen_name` imports a symbol the ported core defines (`Fbuffer_live_p`, `Fcons`,
//     `store_xwidget_event_string`, ...);
//   * header-only constructs - the `Qnil`/`Qt`/`Qunbound` entries the generated globals.h
//     turns into names, and `FRAME_NS_VIEW` - are named after the C construct and the core
//     lane exports the C-ABI entry;
//   * `@_cdecl` exports this file's C interface under the C names and signatures.
//
// Recorded absences (PORTING.md 0.1/P-08):
//   * src/nsxwidget.m 73-79's `#if 0` arm (plugins are not supported by macOS any more) is
//     dead in every configuration and stays C text;
//   * the `/* decisionHandler (…Cancel); */` comment at src/nsxwidget.m 141 is a comment in
//     the reference too - the live call is the `Allow` below it;
//   * `xwScript` (src/nsxwidget.m 271-303) is a `static NSString *` filled in `+initialize`.
//     Swift has no `+initialize`, so it is a lazily computed `let` with the same content;
//     docs/ports/nsxwidget.md records that as the unit's one language difference.
//
// The unit is not compiled by the frozen configuration (`--with-ns=no`), exactly like the
// other seven `.m` lanes; it exists for 1:1 coverage of the reference's sources.

import Foundation
import AppKit
import WebKit

// ==========================================================================
// Lane ABI: the C environment this unit uses.
// ==========================================================================

/// C: `typedef struct Lisp_X *Lisp_Word; typedef Lisp_Word Lisp_Object;` (src/lisp.h
/// 311-323, the pointer form the frozen configuration uses; PORTING.md P-01/1.2).
typealias Lisp_Object = UnsafeMutableRawPointer?

/// C: `union vectorlike_header` (src/lisp.h) - the first member of every pseudovector; its
/// size and alignment are what puts the fields below at the offsets the C side asserts.
struct vectorlike_header {
  var size: Int
}

/// C: `struct Lisp_Vector` (src/lisp.h) - the flexible `contents` array is the only member
/// `js_to_lisp` writes, and it is written by index after `allocate_nil_vector` sized it.
struct Lisp_Vector {
  var header: vectorlike_header
  var contents: Lisp_Object
}

/// C: `struct xwidget` (src/xwidget.h 55-99), the `NS_IMPL_COCOA` arm: the Lisp part first
/// (`plist`, `type`, `buffer`, `title`, `script_callbacks`), then `height`, `width`,
/// `xwidget_id`, `find_text`, then the Cocoa members `xwWidget`, `xwWindow`, `xv`, and the
/// `kill_without_query` bitfield.  Field order is load-bearing: the C side's `offsetof`
/// asserts read these, so a field may not be moved or dropped.
struct xwidget {
  var header: vectorlike_header
  var plist: Lisp_Object
  var type: Lisp_Object
  var buffer: Lisp_Object
  var title: Lisp_Object
  var script_callbacks: Lisp_Object
  var height: Int32
  var width: Int32
  var xwidget_id: UInt32
  var find_text: UnsafeMutablePointer<CChar>?
  /// C: `NSView *xwWidget;` (xwidget.h 86) - an Objective-C object pointer.
  var xwWidget: UnsafeMutableRawPointer?
  /// C: `XwWindow *xwWindow;` (xwidget.h 87).
  var xwWindow: UnsafeMutableRawPointer?
  /// C: `struct xwidget_view *xv;` (xwidget.h 90) - the 1:1 relationship WebKit2 forces.
  var xv: UnsafeMutablePointer<xwidget_view>?
  /// C: `bool_bf kill_without_query : 1;` (xwidget.h 98).
  var kill_without_query: UInt32
}

/// C: `struct xwidget_view` (src/xwidget.h 102-155), the `NS_IMPL_COCOA` arm.  The Lisp
/// part ends at `w`, then `redisplayed`, `hidden`, `area`, the Cocoa members `xvWindow` and
/// `emacswindow`, and the geometry `x`, `y`, `clip_right`, `clip_bottom`, `clip_top`,
/// `clip_left`, `handler_id`.
struct xwidget_view {
  var header: vectorlike_header
  var model: Lisp_Object
  var w: Lisp_Object
  var redisplayed: Bool
  var hidden: Bool
  var area: Int32
  /// C: `XvWindow *xvWindow;` (xwidget.h 143).
  var xvWindow: UnsafeMutableRawPointer?
  /// C: `NSView *emacswindow;` (xwidget.h 144).
  var emacswindow: UnsafeMutableRawPointer?
  var x: Int32
  var y: Int32
  var clip_right: Int32
  var clip_bottom: Int32
  var clip_top: Int32
  var clip_left: Int32
  var handler_id: Int
}

/// C: `struct glyph_string` - only its `f` member is read here (src/nsxwidget.m 578:
/// `FRAME_NS_VIEW (s->f)`); the rest of the struct belongs to the redisplay lane.
struct glyph_string {
  var f: UnsafeMutableRawPointer?
}

/// C: `#define NILP(x) EQ (x, Qnil)` (src/lisp.h) - `Qnil` is the nil pointer in the
/// pointer-tagging form this configuration uses.
@inline(__always) func NILP(_ x: Lisp_Object) -> Bool { return x == nil }

/// C: `#define EQ(x, y) ((x) == (y))` for the same reason (src/lisp.h 1314's fast arm).
@inline(__always) func EQ(_ x: Lisp_Object, _ y: Lisp_Object) -> Bool { return x == y }

// --- names the core exports (globals.h turns the first three into macros) ----------------

@_silgen_name("Qnil") func Qnil() -> Lisp_Object
@_silgen_name("Qt") func Qt() -> Lisp_Object
@_silgen_name("Qunbound") func Qunbound() -> Lisp_Object

// --- C functions the bodies call ---------------------------------------------------------

@_silgen_name("Fbuffer_live_p") func Fbuffer_live_p(_ buffer: Lisp_Object) -> Lisp_Object
@_silgen_name("Fcurrent_buffer") func Fcurrent_buffer() -> Lisp_Object
@_silgen_name("buffer_local_value") func buffer_local_value(_ variable: Lisp_Object,
                                                           _ buffer: Lisp_Object) -> Lisp_Object
@_silgen_name("build_string") func build_string(_ s: UnsafePointer<CChar>) -> Lisp_Object
@_silgen_name("Fintern") func Fintern(_ name: Lisp_Object, _ obarray: Lisp_Object) -> Lisp_Object
@_silgen_name("Fcons") func Fcons(_ car: Lisp_Object, _ cdr: Lisp_Object) -> Lisp_Object
@_silgen_name("make_int") func make_int(_ n: Int) -> Lisp_Object
@_silgen_name("make_float") func make_float(_ d: Double) -> Lisp_Object
@_silgen_name("allocate_nil_vector") func allocate_nil_vector(_ n: Int) -> UnsafeMutablePointer<Lisp_Vector>
@_silgen_name("list2i") func list2i(_ a: Int, _ b: Int) -> Lisp_Object
@_silgen_name("make_multibyte_string") func make_multibyte_string(_ data: UnsafePointer<UInt8>?,
                                                                  _ chars: Int, _ bytes: Int) -> Lisp_Object
@_silgen_name("block_input") func block_input()
@_silgen_name("unblock_input") func unblock_input()
@_silgen_name("message") func message(_ format: UnsafePointer<CChar>, _ args: CVarArg...)
@_silgen_name("store_xwidget_event_string") func store_xwidget_event_string(
  _ xw: UnsafeMutablePointer<xwidget>, _ event: UnsafePointer<CChar>, _ name: UnsafePointer<CChar>)
@_silgen_name("store_xwidget_download_callback_event") func store_xwidget_download_callback_event(
  _ xw: UnsafeMutablePointer<xwidget>, _ url: UnsafePointer<CChar>?,
  _ mime_type: UnsafePointer<CChar>?, _ filename: UnsafePointer<CChar>?)
@_silgen_name("store_xwidget_js_callback_event") func store_xwidget_js_callback_event(
  _ xw: UnsafeMutablePointer<xwidget>, _ fun: Lisp_Object, _ value: Lisp_Object)

/// C: `@interface NSString (EmacsString) - (Lisp_Object) lispString;` (src/nsterm.m), the
/// conversion the four text-returning functions use; the lane spells it as a function.
func lispString(_ string: NSString) -> Lisp_Object {
  let utf8 = string.utf8String
  let bytes = string.lengthOfBytes(using: String.Encoding.utf8.rawValue)
  let data = utf8.map { UnsafeRawPointer($0).assumingMemoryBound(to: UInt8.self) }
  return make_multibyte_string(data, count_utf8_chars(utf8, bytes), bytes)
}

/// C 4153-4200's `count_utf8_chars` helper, local to the Cocoa lanes: the C function is not
/// exported, so nsfns.swift carries the same body (its line 4902) and so does this file.
func count_utf8_chars(_ strIn: UnsafePointer<CChar>?, _ nbytes: Int) -> Int {
  var str = UnsafeRawPointer(strIn!).assumingMemoryBound(to: UInt8.self)
  let end = str.advanced(by: nbytes)
  var nc = 0
  while str < end {
    nc += 1
    let c = str.pointee
    if c < 0x80 {
      str = str.advanced(by: 1)
    } else if c < 0xE0 {
      str = str.advanced(by: 2)
    } else if c < 0xF0 {
      str = str.advanced(by: 3)
    } else {
      str = str.advanced(by: 4)
    }
  }
  return nc
}

/// C: `#define FRAME_NS_VIEW(f) ((f)->output_data.ns->view)` (src/nsterm.h) - the frame's
/// NSView, which `nsxwidget_init_view` inserts the view into.
@_silgen_name("FRAME_NS_VIEW") func FRAME_NS_VIEW(_ f: UnsafeMutableRawPointer?) -> NSView?

// ==========================================================================
// The reference's source text follows, in its own order.
// ==========================================================================

/* Thoughts on NS Cocoa xwidget and webkit2:

   Webkit2 process architecture seems to be very hostile for offscreen
   rendering techniques, which is used by GTK xwidget implementation;
   Specifically NSView level view sharing / copying is not working.

   *** So only one view can be associated with a model. ***

   With this decision, implementation is plain and can expect best out
   of webkit2's rationale.  But process and session structures will
   diverge from GTK xwidget.  Though, cosmetically similar usages can
   be presented and will be preferred, if agreeable.

   For other widget types, OSR seems possible, but will not care for a
   while.  */

/* Xwidget webkit.  */

@objc(XwWebView) class XwWebView: WKWebView, WKNavigationDelegate, WKUIDelegate, WKScriptMessageHandler {
  /// C: `@property struct xwidget *xw;` (src/nsxwidget.m 53).
  var xw: UnsafeMutablePointer<xwidget>?
  /// C: `@property(retain) NSMutableDictionary *urlScriptBlocked;` (54-56) - whether
  /// javascript is blocked by a `Content-Security-Policy` sandbox without allow-scripts.
  var urlScriptBlocked: NSMutableDictionary = [:]

  /// C: `- (id) initWithFrame:(CGRect)frame configuration:… xwidget:…` (60-101).
  init(frame: CGRect, configuration: WKWebViewConfiguration, xwidget: UnsafeMutablePointer<xwidget>) {
    /* Script controller to add script message handler and user script.  */
    let scriptor = WKUserContentController()
    configuration.userContentController = scriptor

    /* Enable inspect element context menu item for debugging.  */
    configuration.preferences.setValue(true, forKey: "developerExtrasEnabled")

    super.init(frame: frame, configuration: configuration)
    self.xw = xwidget
    self.urlScriptBlocked = NSMutableDictionary()
    self.navigationDelegate = self
    self.uiDelegate = self
    self.customUserAgent =
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_6)"
      + " AppleWebKit/603.3.8 (KHTML, like Gecko)"
      + " Version/11.0.1 Safari/603.3.8"
    scriptor.add(self, name: "keyDown")
    scriptor.addUserScript(WKUserScript(source: XwWebView.xwScript,
                                        injectionTime: .atDocumentStart,
                                        forMainFrameOnly: false))
  }

  required init?(coder: NSCoder) { fatalError("not used: the reference has no nib path") }

  /* These 4 functions emulate the behavior of webkit_view_load_changed_cb
     in the GTK implementation*/

  /// C 104-110.
  @objc func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
    if !NILP(Fbuffer_live_p(xw!.pointee.buffer)) {
      store_xwidget_event_string(xw!, "load-changed", "load-finished")
    }
  }

  /// C 112-118.
  @objc func webView(_ webView: WKWebView, didStartProvisionalNavigation navigation: WKNavigation!) {
    if !NILP(Fbuffer_live_p(xw!.pointee.buffer)) {
      store_xwidget_event_string(xw!, "load-changed", "load-started")
    }
  }

  /// C 120-127.
  @objc func webView(_ webView: WKWebView,
                     didReceiveServerRedirectForProvisionalNavigation navigation: WKNavigation!) {
    if !NILP(Fbuffer_live_p(xw!.pointee.buffer)) {
      store_xwidget_event_string(xw!, "load-changed", "load-redirected")
    }
  }

  /* Start loading WKWebView */
  /// C 129-134.
  @objc func webView(_ webView: WKWebView, didCommit navigation: WKNavigation!) {
    if !NILP(Fbuffer_live_p(xw!.pointee.buffer)) {
      store_xwidget_event_string(xw!, "load-changed", "load-committed")
    }
  }

  /// C 136-149.
  @objc func webView(_ webView: WKWebView,
                     decidePolicyFor navigationAction: WKNavigationAction,
                     decisionHandler: @escaping (WKNavigationActionPolicy) -> Void) {
    switch navigationAction.navigationType {
    case .linkActivated:
      decisionHandler(.allow)
    default:
      /* decisionHandler (WKNavigationActionPolicyCancel); */
      decisionHandler(.allow)
    }
  }

  /// C 151-194.
  @objc func webView(_ webView: WKWebView,
                     decidePolicyFor navigationResponse: WKNavigationResponse,
                     decisionHandler: @escaping (WKNavigationResponsePolicy) -> Void) {
    if !navigationResponse.canShowMIMEType {
      let url = navigationResponse.response.url?.absoluteString
      let mimetype = navigationResponse.response.mimeType
      let filename = navigationResponse.response.suggestedFilename
      decisionHandler(.cancel)
      store_xwidget_download_callback_event(xw!, url, mimetype, filename)
      return
    }
    decisionHandler(.allow)

    let responseURL = navigationResponse.response.url
    if let responseURL = responseURL {
      urlScriptBlocked[responseURL] = NSNumber(value: false)
    }
    if let http = navigationResponse.response as? HTTPURLResponse {
      let headers = http.allHeaderFields
      if let value = headers["Content-Security-Policy"] as? String {
        /* TODO: Sloppy parsing of 'Content-Security-Policy' value.  */
        let s = value as NSString
        let sandbox = s.range(of: "sandbox")
        if sandbox.location != NSNotFound
            && (sandbox.location == 0
                || s.character(at: sandbox.location - 1) == 0x20
                || s.character(at: sandbox.location - 1) == 0x3b) {
          let allowScripts = s.range(of: "allow-scripts")
          if allowScripts.location == NSNotFound || allowScripts.location < sandbox.location {
            if let responseURL = responseURL {
              urlScriptBlocked[responseURL] = NSNumber(value: true)
            }
          }
        }
      }
    }
  }

  /* No additional new webview or emacs window will be created
     for <a ... target="_blank">.  */
  /// C 197-206.
  @objc func webView(_ webView: WKWebView,
                     createWebViewWith configuration: WKWebViewConfiguration,
                     for navigationAction: WKNavigationAction,
                     windowFeatures: WKWindowFeatures) -> WKWebView? {
    if let target = navigationAction.targetFrame, !target.isMainFrame {
      webView.load(navigationAction.request)
    }
    return nil
  }

  /* Open panel for file upload.  */
  /// C 208-219.
  @objc func webView(_ webView: WKWebView,
                     runOpenPanelWith parameters: WKOpenPanelParameters,
                     initiatedByFrame frame: WKFrameInfo,
                     completionHandler: @escaping ([URL]?) -> Void) {
    let openPanel = NSOpenPanel()
    openPanel.canChooseFiles = true
    openPanel.canChooseDirectories = false
    openPanel.allowsMultipleSelection = parameters.allowsMultipleSelection
    if openPanel.runModal() == .OK {
      completionHandler(openPanel.urls)
    } else {
      completionHandler(nil)
    }
  }

  /* By forwarding mouse events to emacs view (frame)
     - Mouse click in webview selects the window contains the webview.
     - Correct mouse hand/arrow/I-beam is displayed (TODO: not perfect yet).  */

  /// C 224-232.
  override func mouseDown(with event: NSEvent) {
    (xw!.pointee.xv!.pointee.emacswindow as? NSView)?.mouseDown(with: event)
    super.mouseDown(with: event)
  }

  /// C 234-238.
  override func mouseUp(with event: NSEvent) {
    (xw!.pointee.xv!.pointee.emacswindow as? NSView)?.mouseUp(with: event)
    super.mouseUp(with: event)
  }

  /* Basically we want keyboard events handled by emacs unless an input
     element has focus.  Especially, while incremental search, we set
     emacs as first responder to avoid focus held in an input element
     with matching text.  */

  /// C 244-267.
  override func keyDown(with event: NSEvent) {
    let variable = Fintern(build_string("isearch-mode"), Qnil())
    let value = buffer_local_value(variable, Fcurrent_buffer())
    if !EQ(value, Qunbound()) && !EQ(value, Qnil()) {
      window?.makeFirstResponder(xw!.pointee.xv!.pointee.emacswindow as? NSView)
      (xw!.pointee.xv!.pointee.emacswindow as? NSView)?.keyDown(with: event)
      return
    }

    /* Emacs handles keyboard events when javascript is blocked.  */
    if let blocked = urlScriptBlocked[url] as? NSNumber, blocked.boolValue {
      (xw!.pointee.xv!.pointee.emacswindow as? NSView)?.keyDown(with: event)
      return
    }

    evaluateJavaScript("xwHasFocus()") { result, error in
      if error != nil {
        NSLog("xwHasFocus: %@", String(describing: error))
        (self.xw!.pointee.xv!.pointee.emacswindow as? NSView)?.keyDown(with: event)
      } else if let result = result as? NSNumber {
        /* __NSCFBoolean */
        if !result.boolValue {
          (self.xw!.pointee.xv!.pointee.emacswindow as? NSView)?.keyDown(with: event)
        } else {
          super.keyDown(with: event)
        }
      }
    }
  }

  /// C 269-274.
  override func interpretKeyEvents(_ eventArray: [NSEvent]) {
    /* We should do nothing and do not forward (default implementation
       if we not override here) to let emacs collect key events and ask
       interpretKeyEvents to its superclass.  */
  }

  /// C 271-303: `static NSString *xwScript;` filled in `+initialize` - a lazily computed
  /// `let` here, with the same concatenated source.
  static let xwScript: String = {
    /* Find out if an input element has focus.
       Message to script message handler when 'C-g' key down.  */
    return "function xwHasFocus() {"
      + "  var ae = document.activeElement;"
      + "  if (ae) {"
      + "    var name = ae.nodeName;"
      + "    return name == 'INPUT' || name == 'TEXTAREA';"
      + "  } else {"
      + "    return false;"
      + "  }"
      + "}"
      + "function xwKeyDown(event) {"
      + "  if (event.ctrlKey && event.key == 'g') {"
      + "    window.webkit.messageHandlers.keyDown.postMessage('C-g');"
      + "  }"
      + "}"
      + "document.addEventListener('keydown', xwKeyDown);"
  }()

  /* Confirming to WKScriptMessageHandler, listens concerning keyDown in
     webkit. Currently 'C-g'.  */
  /// C 305-314.
  func userContentController(_ userContentController: WKUserContentController,
                             didReceive message: WKScriptMessage) {
    if let body = message.body as? String, body == "C-g" {
      /* Just give up focus, no relay "C-g" to emacs, another "C-g"
         follows will be handled by emacs.  */
      window?.makeFirstResponder(xw!.pointee.xv!.pointee.emacswindow as? NSView)
    }
  }
}

/* Xwidget webkit commands.  */

/// C 328-334.
func nsxwidget_is_web_view(_ xw: UnsafeMutablePointer<xwidget>?) -> Bool {
  guard let xw = xw, let widget = xw.pointee.xwWidget else { return false }
  return (Unmanaged<AnyObject>.fromOpaque(widget).takeUnretainedValue() as? WKWebView) != nil
}

/// C 336-341.
func nsxwidget_webkit_uri(_ xw: UnsafeMutablePointer<xwidget>?) -> Lisp_Object {
  let webView = xwWebView(xw)
  return lispString((webView?.url?.absoluteString ?? "") as NSString)
}

/// C 343-348.
func nsxwidget_webkit_title(_ xw: UnsafeMutablePointer<xwidget>?) -> Lisp_Object {
  let webView = xwWebView(xw)
  return lispString((webView?.title ?? "") as NSString)
}

/* @Note ATS - Need application transport security in 'Info.plist' or
   remote pages will not loaded.  */
/// C 350-361.
func nsxwidget_webkit_goto_uri(_ xw: UnsafeMutablePointer<xwidget>?, _ uri: UnsafePointer<CChar>?) {
  guard let webView = xwWebView(xw) else { return }
  let urlString = uri.map { String(cString: $0) }
  if let urlString = urlString, let url = URL(string: urlString) {
    webView.load(URLRequest(url: url))
  }
}

/// C 362-371.
func nsxwidget_webkit_goto_history(_ xw: UnsafeMutablePointer<xwidget>?, _ rel_pos: Int32) {
  guard let webView = xwWebView(xw) else { return }
  switch rel_pos {
  case -1: webView.goBack()
  case 0: webView.reload()
  case 1: webView.goForward()
  default: break
  }
}

/// C 373-378.
func nsxwidget_webkit_estimated_load_progress(_ xw: UnsafeMutablePointer<xwidget>?) -> Double {
  return xwWebView(xw)?.estimatedProgress ?? 0
}

/// C 380-386.
func nsxwidget_webkit_stop_loading(_ xw: UnsafeMutablePointer<xwidget>?) {
  xwWebView(xw)?.stopLoading()
}

/// C 387-396.
func nsxwidget_webkit_zoom(_ xw: UnsafeMutablePointer<xwidget>?, _ zoom_change: Double) {
  guard let webView = xwWebView(xw) else { return }
  webView.magnification += zoom_change
  /* TODO: setMagnification:centeredAtPoint.  */
}

/// C 398-452: recursively convert an objc native type JavaScript value to a Lisp value.
/// Mostly copied from GTK xwidget's `webkit_js_to_lisp`.
func js_to_lisp(_ value: Any?) -> Lisp_Object {
  guard let value = value else { return Qnil() }
  if value is NSNull { return Qnil() }
  if let string = value as? NSString { return lispString(string) }
  if let number = value as? NSNumber {
    let type = String(cString: number.objCType)
    if type == "c" { /* __NSCFBoolean has type character 'c'.  */
      return number.boolValue ? Qt() : Qnil()
    } else if type == "i" || type == "l" {
      return make_int(number.intValue)
    } else if type == "f" || type == "d" {
      return make_float(number.doubleValue)
    }
    /* else fall through.  */
  }
  if let array = value as? NSArray {
    let n = array.count
    let p = allocate_nil_vector(n)
    let contents = UnsafeMutableRawPointer(p).advanced(by: MemoryLayout<vectorlike_header>.size)
                        .assumingMemoryBound(to: Lisp_Object.self)
    for i in 0..<n {
      contents[i] = js_to_lisp(array[i])
    }
    /* XSETVECTOR (obj, p): the tag for a vectorlike object is the pointer itself in the
       pointer-tagging form, so the C macro is the identity here (PORTING.md 1.2).  */
    return UnsafeMutableRawPointer(p)
  }
  if let dict = value as? NSDictionary {
    let keys = dict.allKeys
    let n = keys.count
    let p = allocate_nil_vector(n)
    let contents = UnsafeMutableRawPointer(p).advanced(by: MemoryLayout<vectorlike_header>.size)
                        .assumingMemoryBound(to: Lisp_Object.self)
    for i in 0..<n {
      let key = keys[i] as! NSString
      contents[i] = Fcons(lispString(key), js_to_lisp(dict[key]))
    }
    return UnsafeMutableRawPointer(p)
  }
  NSLog("Unhandled type in javascript result")
  return Qnil()
}

/// C 454-481.
func nsxwidget_webkit_execute_script(_ xw: UnsafeMutablePointer<xwidget>?,
                                            _ script: UnsafePointer<CChar>?,
                                            _ fun: Lisp_Object) {
  guard let webView = xwWebView(xw), let xw = xw else { return }
  if let blocked = webView.urlScriptBlocked[webView.url] as? NSNumber, blocked.boolValue {
    message("Javascript is blocked by 'CSP: sandbox'.")
    return
  }

  let javascriptString = script.map { String(cString: $0) } ?? ""
  webView.evaluateJavaScript(javascriptString) { result, error in
    if error != nil {
      NSLog("evaluateJavaScript error : %@", (error! as NSError).localizedDescription)
      NSLog("error script=%@", javascriptString)
    } else if let result = result {
      /* NSLog (@"result=%@, type=%@", result, [result class]); */
      let lispValue = js_to_lisp(result)
      store_xwidget_js_callback_event(xw, fun, lispValue)
    }
  }
}

/* Window containing an xwidget.  */

@objc(XwWindow) class XwWindow: NSView {
  /// C 483-486.
  override var isFlipped: Bool { return true }
}

/* Xwidget model, macOS Cocoa part.  */

/// C 488-502.
func nsxwidget_init(_ xw: UnsafeMutablePointer<xwidget>?) {
  guard let xw = xw else { return }
  block_input()
  let rect = NSRect(x: 0, y: 0, width: Int(xw.pointee.width), height: Int(xw.pointee.height))
  let configuration = WKWebViewConfiguration()
  let view = XwWebView(frame: rect, configuration: configuration, xwidget: xw)
  xw.pointee.xwWidget = Unmanaged.passRetained(view).toOpaque()
  let window = XwWindow(frame: rect)
  xw.pointee.xwWindow = Unmanaged.passRetained(window).toOpaque()
  window.addSubview(view)
  xw.pointee.xv = nil /* for 1 to 1 relationship of webkit2.  */
  unblock_input()
}

/// C 504-530.
func nsxwidget_kill(_ xw: UnsafeMutablePointer<xwidget>?) {
  guard let xw = xw else { return }
  let webView = xwWebView(xw)
  if let scriptor = webView?.configuration.userContentController {
    scriptor.removeAllUserScripts()
    scriptor.removeScriptMessageHandler(forName: "keyDown")
  }

  if let xv = xw.pointee.xv {
    xv.pointee.model = Qnil() /* Make sure related view stale.  */
  }

  /* This stops playing audio when a xwidget-webkit buffer is
     killed.  I could not find other solution.
     TODO: improve this */
  nsxwidget_webkit_goto_uri(xw, "about:blank")

  webView?.urlScriptBlocked.removeAllObjects()
  (xw.pointee.xwWidget.flatMap { Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() } as? NSView)?
    .removeFromSuperviewWithoutNeedingDisplay()
  if let widget = xw.pointee.xwWidget {
    Unmanaged<AnyObject>.fromOpaque(widget).release()
  }
  (xw.pointee.xwWindow.flatMap { Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() } as? NSView)?
    .removeFromSuperviewWithoutNeedingDisplay()
  if let window = xw.pointee.xwWindow {
    Unmanaged<AnyObject>.fromOpaque(window).release()
  }
  xw.pointee.xwWidget = nil
}

/// C 532-540.
func nsxwidget_resize(_ xw: UnsafeMutablePointer<xwidget>?) {
  guard let xw = xw, let widget = xw.pointee.xwWidget else { return }
  let size = NSSize(width: Int(xw.pointee.width), height: Int(xw.pointee.height))
  (xw.pointee.xwWindow.flatMap { Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() } as? NSView)?
    .setFrameSize(size)
  (Unmanaged<AnyObject>.fromOpaque(widget).takeUnretainedValue() as? NSView)?.setFrameSize(size)
}

/// C 542-550.
func nsxwidget_get_size(_ xw: UnsafeMutablePointer<xwidget>?) -> Lisp_Object {
  guard let xw = xw, let widget = xw.pointee.xwWidget,
        let view = Unmanaged<AnyObject>.fromOpaque(widget).takeUnretainedValue() as? NSView
  else { return Qnil() }
  return list2i(Int(view.frame.size.width), Int(view.frame.size.height))
}

/* Xwidget view, macOS Cocoa part.  */

@objc(XvWindow) class XvWindow: NSView {
  /// C 553-557.
  override var isFlipped: Bool { return true }
  /// C 585/556: the back pointers `nsxwidget_init_view` sets.
  var xw: UnsafeMutablePointer<xwidget>?
  var xv: UnsafeMutablePointer<xwidget_view>?
}

/// C 556-584.
func nsxwidget_init_view(_ xv: UnsafeMutablePointer<xwidget_view>?,
                                _ xw: UnsafeMutablePointer<xwidget>?,
                                _ s: UnsafeMutablePointer<glyph_string>?,
                                _ x: Int32, _ y: Int32) {
  guard let xv = xv, let xw = xw else { return }
  /* 'x_draw_xwidget_glyph_string' will calculate correct position and
     size of clip to draw in emacs buffer window.  Thus, just begin at
     origin with no crop.  */
  xv.pointee.x = x
  xv.pointee.y = y
  xv.pointee.clip_left = 0
  xv.pointee.clip_right = xw.pointee.width
  xv.pointee.clip_top = 0
  xv.pointee.clip_bottom = xw.pointee.height

  let window = XvWindow(frame: NSRect(x: Int(x), y: Int(y),
                                      width: Int(xw.pointee.width), height: Int(xw.pointee.height)))
  window.xw = xw
  window.xv = xv
  xv.pointee.xvWindow = Unmanaged.passRetained(window).toOpaque()

  xw.pointee.xv = xv /* For 1 to 1 relationship of webkit2.  */
  if let modelWindow = xw.pointee.xwWindow.flatMap({
    Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() as? NSView
  }) {
    window.addSubview(modelWindow)
  }

  let emacsView = FRAME_NS_VIEW(s?.pointee.f)
  xv.pointee.emacswindow = emacsView.flatMap { Unmanaged.passUnretained($0).toOpaque() }
  emacsView?.addSubview(window)
}

/// C 586-597.
func nsxwidget_delete_view(_ xv: UnsafeMutablePointer<xwidget_view>?) {
  guard let xv = xv else { return }
  if !NILP(xv.pointee.model) {
    /* struct xwidget *xw = XXWIDGET (xv->model); - the vectorlike tag is the pointer.  */
    if let xw = xv.pointee.model?.assumingMemoryBound(to: xwidget.self) {
      (xw.pointee.xwWindow.flatMap {
        Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() as? NSView
      })?.removeFromSuperviewWithoutNeedingDisplay()
      xw.pointee.xv = nil /* Now model has no view.  */
    }
  }
  (xv.pointee.xvWindow.flatMap {
    Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() as? NSView
  })?.removeFromSuperviewWithoutNeedingDisplay()
  if let window = xv.pointee.xvWindow {
    Unmanaged<AnyObject>.fromOpaque(window).release()
  }
}

/// C 599-605.
func nsxwidget_show_view(_ xv: UnsafeMutablePointer<xwidget_view>?) {
  guard let xv = xv else { return }
  xv.pointee.hidden = false
  (xv.pointee.xvWindow.flatMap {
    Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() as? NSView
  })?.setFrameOrigin(NSPoint(x: Int(xv.pointee.x + xv.pointee.clip_left),
                             y: Int(xv.pointee.y + xv.pointee.clip_top)))
}

/// C 607-612.
func nsxwidget_hide_view(_ xv: UnsafeMutablePointer<xwidget_view>?) {
  guard let xv = xv else { return }
  xv.pointee.hidden = true
  (xv.pointee.xvWindow.flatMap {
    Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() as? NSView
  })?.setFrameOrigin(NSPoint(x: 10000, y: 10000))
}

/// C 614-618.
func nsxwidget_resize_view(_ xv: UnsafeMutablePointer<xwidget_view>?, _ width: Int32, _ height: Int32) {
  (xv?.pointee.xvWindow.flatMap {
    Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() as? NSView
  })?.setFrameSize(NSSize(width: Int(width), height: Int(height)))
}

/// C 620-625.
func nsxwidget_move_view(_ xv: UnsafeMutablePointer<xwidget_view>?, _ x: Int32, _ y: Int32) {
  (xv?.pointee.xvWindow.flatMap {
    Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() as? NSView
  })?.setFrameOrigin(NSPoint(x: Int(x), y: Int(y)))
}

/* Move model window in container (view window).  */
/// C 627-632.
func nsxwidget_move_widget_in_view(_ xv: UnsafeMutablePointer<xwidget_view>?, _ x: Int32, _ y: Int32) {
  guard let xv = xv, let window = xv.pointee.xvWindow.flatMap({
    Unmanaged<XvWindow>.fromOpaque($0).takeUnretainedValue() as? XvWindow
  }), let xww = window.xw else { return }
  (xww.pointee.xwWindow.flatMap {
    Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() as? NSView
  })?.setFrameOrigin(NSPoint(x: Int(x), y: Int(y)))
}

/// C 634-637.
func nsxwidget_set_needsdisplay(_ xv: UnsafeMutablePointer<xwidget_view>?) {
  (xv?.pointee.xvWindow.flatMap {
    Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() as? NSView
  })?.needsDisplay = true
}

// ==========================================================================
// Lane helper (not a C definition): the model's `WKWebView`, if it has one.
// ==========================================================================

func xwWebView(_ xw: UnsafeMutablePointer<xwidget>?) -> XwWebView? {
  guard let widget = xw?.pointee.xwWidget else { return nil }
  return Unmanaged<AnyObject>.fromOpaque(widget).takeUnretainedValue() as? XwWebView
}

// ==========================================================================
// The C interface (src/xwidget.h 1203-1222's counterparts for this lane).
//
// The bodies above take Swift-declared structs (`struct xwidget`, `struct xwidget_view`),
// which `@_cdecl` cannot represent, so the exported entry points are thin wrappers that bind
// the C pointer to the struct the way every other lane does.  The C names and signatures are
// unchanged; the C side sees exactly the symbols src/xwidget.h declares.
// ==========================================================================

@_cdecl("nsxwidget_is_web_view")
func nsxwidget_is_web_view_c(_ xw: UnsafeMutableRawPointer?) -> Bool {
  return nsxwidget_is_web_view(xw?.assumingMemoryBound(to: xwidget.self))
}

@_cdecl("nsxwidget_webkit_uri")
func nsxwidget_webkit_uri_c(_ xw: UnsafeMutableRawPointer?) -> Lisp_Object {
  return nsxwidget_webkit_uri(xw?.assumingMemoryBound(to: xwidget.self))
}

@_cdecl("nsxwidget_webkit_title")
func nsxwidget_webkit_title_c(_ xw: UnsafeMutableRawPointer?) -> Lisp_Object {
  return nsxwidget_webkit_title(xw?.assumingMemoryBound(to: xwidget.self))
}

@_cdecl("nsxwidget_webkit_goto_uri")
func nsxwidget_webkit_goto_uri_c(_ xw: UnsafeMutableRawPointer?, _ uri: UnsafePointer<CChar>?) {
  nsxwidget_webkit_goto_uri(xw?.assumingMemoryBound(to: xwidget.self), uri)
}

@_cdecl("nsxwidget_webkit_goto_history")
func nsxwidget_webkit_goto_history_c(_ xw: UnsafeMutableRawPointer?, _ rel_pos: Int32) {
  nsxwidget_webkit_goto_history(xw?.assumingMemoryBound(to: xwidget.self), rel_pos)
}

@_cdecl("nsxwidget_webkit_estimated_load_progress")
func nsxwidget_webkit_estimated_load_progress_c(_ xw: UnsafeMutableRawPointer?) -> Double {
  return nsxwidget_webkit_estimated_load_progress(xw?.assumingMemoryBound(to: xwidget.self))
}

@_cdecl("nsxwidget_webkit_stop_loading")
func nsxwidget_webkit_stop_loading_c(_ xw: UnsafeMutableRawPointer?) {
  nsxwidget_webkit_stop_loading(xw?.assumingMemoryBound(to: xwidget.self))
}

@_cdecl("nsxwidget_webkit_zoom")
func nsxwidget_webkit_zoom_c(_ xw: UnsafeMutableRawPointer?, _ zoom_change: Double) {
  nsxwidget_webkit_zoom(xw?.assumingMemoryBound(to: xwidget.self), zoom_change)
}

@_cdecl("nsxwidget_webkit_execute_script")
func nsxwidget_webkit_execute_script_c(_ xw: UnsafeMutableRawPointer?, _ script: UnsafePointer<CChar>?,
                                       _ fun: Lisp_Object) {
  nsxwidget_webkit_execute_script(xw?.assumingMemoryBound(to: xwidget.self), script, fun)
}

@_cdecl("nsxwidget_init")
func nsxwidget_init_c(_ xw: UnsafeMutableRawPointer?) {
  nsxwidget_init(xw?.assumingMemoryBound(to: xwidget.self))
}

@_cdecl("nsxwidget_kill")
func nsxwidget_kill_c(_ xw: UnsafeMutableRawPointer?) {
  nsxwidget_kill(xw?.assumingMemoryBound(to: xwidget.self))
}

@_cdecl("nsxwidget_resize")
func nsxwidget_resize_c(_ xw: UnsafeMutableRawPointer?) {
  nsxwidget_resize(xw?.assumingMemoryBound(to: xwidget.self))
}

@_cdecl("nsxwidget_get_size")
func nsxwidget_get_size_c(_ xw: UnsafeMutableRawPointer?) -> Lisp_Object {
  return nsxwidget_get_size(xw?.assumingMemoryBound(to: xwidget.self))
}

@_cdecl("nsxwidget_init_view")
func nsxwidget_init_view_c(_ xv: UnsafeMutableRawPointer?, _ xw: UnsafeMutableRawPointer?,
                           _ s: UnsafeMutableRawPointer?, _ x: Int32, _ y: Int32) {
  nsxwidget_init_view(xv?.assumingMemoryBound(to: xwidget_view.self),
                      xw?.assumingMemoryBound(to: xwidget.self),
                      s?.assumingMemoryBound(to: glyph_string.self), x, y)
}

@_cdecl("nsxwidget_delete_view")
func nsxwidget_delete_view_c(_ xv: UnsafeMutableRawPointer?) {
  nsxwidget_delete_view(xv?.assumingMemoryBound(to: xwidget_view.self))
}

@_cdecl("nsxwidget_show_view")
func nsxwidget_show_view_c(_ xv: UnsafeMutableRawPointer?) {
  nsxwidget_show_view(xv?.assumingMemoryBound(to: xwidget_view.self))
}

@_cdecl("nsxwidget_hide_view")
func nsxwidget_hide_view_c(_ xv: UnsafeMutableRawPointer?) {
  nsxwidget_hide_view(xv?.assumingMemoryBound(to: xwidget_view.self))
}

@_cdecl("nsxwidget_resize_view")
func nsxwidget_resize_view_c(_ xv: UnsafeMutableRawPointer?, _ width: Int32, _ height: Int32) {
  nsxwidget_resize_view(xv?.assumingMemoryBound(to: xwidget_view.self), width, height)
}

@_cdecl("nsxwidget_move_view")
func nsxwidget_move_view_c(_ xv: UnsafeMutableRawPointer?, _ x: Int32, _ y: Int32) {
  nsxwidget_move_view(xv?.assumingMemoryBound(to: xwidget_view.self), x, y)
}

@_cdecl("nsxwidget_move_widget_in_view")
func nsxwidget_move_widget_in_view_c(_ xv: UnsafeMutableRawPointer?, _ x: Int32, _ y: Int32) {
  nsxwidget_move_widget_in_view(xv?.assumingMemoryBound(to: xwidget_view.self), x, y)
}

@_cdecl("nsxwidget_set_needsdisplay")
func nsxwidget_set_needsdisplay_c(_ xv: UnsafeMutableRawPointer?) {
  nsxwidget_set_needsdisplay(xv?.assumingMemoryBound(to: xwidget_view.self))
}
