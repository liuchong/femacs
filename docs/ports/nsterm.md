# emacs/src/nsterm.m (11811 lines)

> **Giant unit.**  `src/nsterm.m` is 11,811 lines and is transcribed in C order, one slice per
> round; the unit is `femacs/platform/macos/nsterm.swift`, and `## Progress` below names the
> covered range.  Like every `.m` of batch F the file is **not compiled by the frozen
> configuration** (`docs/baseline/frozen-build-sources.txt`, `docs/status.md` 3.31): the frozen
> target is darwin-arm64 with no window system, so `HAVE_NS` and `NS_IMPL_COCOA` are undefined
> and src/nsterm.m is one of the 77 src files `make -n` never reaches.  Batch F ports the
> Objective-C front end to Swift anyway (`docs/port-map.md`'s Swift table), so this ledger is the
> checklist of the **Cocoa/AppKit build** the file's `NS_IMPL_COCOA` arms describe, and the line
> numbers are the raw reference lines.
>
> `python3 tools/extract-abi.py emacs/src/nsterm.m` generates the five sections below; the
> `Objective-C classes and categories` section is hand-written (its cause is in `## Decisions`),
> as are the eight `File-scope objects` rows the generator misses.  **`extract-abi.py --check`
> does not apply to a Swift unit** (the tool has no `.m -> .swift` pair, `docs/status.md` 3.164),
> so this unit's coverage gate is `tools/check-swift-units.py` and its compile gate is
> `swiftc -parse-as-library -c`; `## Evidence` quotes both.



# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 142
- [x] nstrace_leave  (line 113)  — dead: `NSTRACE_ENABLED` is 0 (src/nsterm.h 142-143), so the reference does not compile it either - recorded, not transcribed
- [x] nstrace_restore_global_trace_state  (line 122)  — dead, like the box above
- [x] nstrace_fullscreen_type_name  (line 129)  — dead, like the box above
- [x] mod_of_kind  (line 360, static)  — C 358-367, landed as `fileprivate func mod_of_kind`
- [x] ev_modifiers_helper  (line 372, static)  — C 369-389, landed as `fileprivate func ev_modifiers_helper`
- [x] ns_init_events  (line 485) — landed, `@_cdecl("ns_init_events")` (the unit's C interface, src/nsterm.h 1257)
- [x] ns_finish_events  (line 492) — landed, `@_cdecl("ns_finish_events")` (src/nsterm.h 1258)
- [x] hold_event  (line 498, static) — landed as `fileprivate func hold_event` (the hold queue's `input_event` copy is `MemoryLayout<input_event>.stride`, measured 64)
- [x] append2  (line 515, static) — landed as `fileprivate func append2`; C's `list (item)` is `list1` for one argument
- [ ] ns_relocate  (line 525)
- [ ] ns_init_pool  (line 560)
- [ ] ns_init_locale  (line 569)
- [ ] ns_release_object  (line 602)
- [ ] ns_retain_object  (line 612)
- [ ] ns_alloc_autorelease_pool  (line 622)
- [ ] ns_release_autorelease_pool  (line 632)
- [ ] ns_menu_bar_should_be_hidden  (line 642, static)
- [ ] ns_screen_margins  (line 660, static)
- [ ] ns_screen_margins_ignoring_hidden_dock  (line 711, static)
- [ ] ns_menu_bar_height  (line 754, static)
- [ ] ns_parent_window_rect  (line 773, static)
- [ ] ns_row_rect  (line 808, static)
- [ ] ns_frame_scale_factor  (line 828)
- [ ] constrain_frame_rect  (line 892, static)
- [ ] ns_constrain_all_frames  (line 1001, static)
- [ ] ns_update_auto_hide_menu_bar  (line 1033, static)
- [ ] ns_update_begin  (line 1075, static)
- [ ] ns_UAZoomChangeFocus  (line 1162, static)
- [ ] ns_update_end  (line 1212, static)
- [ ] ns_unfocus  (line 1282, static)
- [ ] ns_ring_bell  (line 1436, static)
- [ ] hide_bell  (line 1471, static)
- [ ] ns_get_focus_frame  (line 1493, static)
- [ ] ns_focus_frame  (line 1510, static)
- [ ] ns_raise_frame  (line 1528, static)
- [ ] ns_lower_frame  (line 1550, static)
- [ ] ns_frame_raise_lower  (line 1566, static)
- [ ] ns_frame_rehighlight  (line 1582, static)
- [ ] ns_make_frame_visible  (line 1624)
- [ ] ns_make_frame_invisible  (line 1678)
- [ ] ns_make_frame_visible_invisible  (line 1693, static)
- [ ] ns_iconify_frame  (line 1705)
- [ ] ns_free_frame_resources  (line 1744)
- [ ] ns_destroy_window  (line 1784, static)
- [ ] compute_offset  (line 1816, static)
- [ ] ns_set_offset  (line 1854)
- [ ] ns_set_window_size  (line 1877, static)
- [ ] ns_set_window_size_and_position  (line 1920, static)
- [ ] ns_set_no_focus_on_map  (line 2043)
- [ ] ns_set_appearance_1  (line 2120)
- [ ] ns_set_appearance  (line 2131)
- [ ] ns_set_transparent_titlebar  (line 2149)
- [ ] ns_fullscreen_hook  (line 2169, static)
- [ ] ns_get_color  (line 2191, static)
- [ ] ns_lisp_to_color  (line 2301)
- [ ] ns_query_color  (line 2315, static)
- [ ] ns_defined_color  (line 2332)
- [ ] ns_query_frame_background_color  (line 2357, static)
- [ ] ns_set_frame_alpha  (line 2366, static)
- [ ] frame_set_mouse_pixel_position  (line 2409)
- [ ] ns_note_mouse_movement  (line 2433, static)
- [ ] ns_mouse_position  (line 2476, static)
- [ ] ns_frame_up_to_date  (line 2599, static)
- [ ] ns_define_frame_cursor  (line 2625, static)
- [ ] ns_convert_key  (line 2649, static)
- [ ] get_keysym_name  (line 2667)
- [ ] right_mod  (line 2681, static)
- [ ] nil_or_none  (line 2687, static)
- [ ] ns_redraw_scroll_bars  (line 2773, static)
- [ ] ns_clear_frame  (line 2790)
- [ ] ns_clear_frame_area  (line 2824, static)
- [ ] ns_scroll_run  (line 2850, static)
- [ ] ns_clear_under_internal_border  (line 2920, static)
- [ ] ns_after_update_window_line  (line 2964, static)
- [ ] ns_shift_glyphs_for_insert  (line 3023, static)
- [ ] ns_compute_glyph_string_overhangs  (line 3048, static)
- [ ] ns_define_fringe_bitmap  (line 3092, static)
- [ ] ns_destroy_fringe_bitmap  (line 3199, static)
- [ ] ns_draw_fringe_bitmap  (line 3206, static)
- [ ] ns_draw_window_cursor  (line 3295, static)
- [ ] ns_draw_vertical_window_border  (line 3430, static)
- [ ] ns_draw_window_divider  (line 3453, static)
- [ ] ns_show_hourglass  (line 3512, static)
- [ ] ns_hide_hourglass  (line 3518, static)
- [ ] ns_get_glyph_string_clip_rect  (line 3530, static)
- [ ] ns_draw_underwave  (line 3553, static)
- [ ] ns_draw_dash  (line 3598, static)
- [ ] ns_fill_underline  (line 3620, static)
- [ ] ns_draw_text_decoration  (line 3655, static)
- [ ] ns_draw_box  (line 3810, static)
- [ ] ns_setup_relief_colors  (line 3844, static)
- [ ] ns_draw_relief  (line 3872, static)
- [ ] ns_dumpglyphs_box_or_relief  (line 4027, static)
- [ ] ns_maybe_dumpglyphs_background  (line 4092, static)
- [ ] ns_draw_image_relief  (line 4180, static)
- [ ] ns_dumpglyphs_image  (line 4267, static)
- [ ] ns_draw_stretch_glyph_string  (line 4397, static)
- [ ] ns_draw_glyph_string_foreground  (line 4548, static)
- [ ] ns_draw_composite_glyph_string_foreground  (line 4568, static)
- [ ] ns_draw_glyphless_glyph_string_foreground  (line 4659, static)
- [ ] ns_set_glyph_string_gc  (line 4748, static)
- [ ] ns_draw_glyph_string  (line 4783, static)
- [ ] ns_send_appdefined  (line 5013, static)
- [ ] check_native_fs  (line 5088, static)
- [ ] ns_read_socket_1  (line 5111, static)
- [ ] ns_read_socket  (line 5196, static)
- [ ] ns_select_1  (line 5203, static)
- [ ] ns_select  (line 5366)
- [ ] ns_run_loop_break  (line 5376)
- [ ] ns_set_vertical_scroll_bar  (line 5396, static)
- [ ] ns_set_horizontal_scroll_bar  (line 5490, static)
- [ ] ns_condemn_scroll_bars  (line 5575, static)
- [ ] ns_redeem_scroll_bar  (line 5597, static)
- [ ] ns_judge_scroll_bars  (line 5622, static)
- [ ] ns_free_pixmap  (line 5649, static)
- [ ] ns_display_pixel_height  (line 5661)
- [ ] ns_display_pixel_width  (line 5676)
- [ ] ns_string_to_lispmod  (line 5691, static)
- [ ] ns_default  (line 5714, static)
- [ ] ns_initialize_display_info  (line 5742, static)
- [ ] ns_default_font_parameter  (line 5770, static)
- [ ] ns_update_window_end  (line 5776, static)
- [ ] ns_flush_display  (line 5786, static)
- [ ] ns_displays_reconfigured  (line 5839, static)
- [ ] ns_delete_display  (line 5872, static)
- [ ] ns_delete_terminal  (line 5880, static)
- [ ] ns_create_terminal  (line 5908, static)
- [ ] ns_term_init  (line 5965)
- [ ] ns_term_shutdown  (line 6198)
- [ ] runAlertPanel  (line 6634, static)
- [ ] not_in_argv  (line 6672, static)
- [ ] ns_font_desc_to_font_spec  (line 6915, static)
- [ ] ns_create_font_panel_buttons  (line 6992, static)
- [ ] ns_in_echo_area_1  (line 7624, static)
- [ ] ns_in_echo_area_2  (line 7633, static)
- [ ] ns_in_echo_area  (line 7639, static)
- [ ] cancel_ns_deferred_UAZoomChangeFocus_timer  (line 8460, static)
- [ ] nswindow_orderedIndex_sort  (line 10085, static)
- [ ] ns_new_font  (line 11418, static)
- [ ] ns_xlfd_to_fontname  (line 11484)
- [ ] mark_nsterm  (line 11529)
- [ ] syms_of_nsterm  (line 11550)


**8 `@implementation`s, 1 file-local `@interface`, 193 method start lines.**

## Objective-C classes and categories — 8

(Hand-written, the way `docs/ports/nsimage.md` writes its `EmacsImage` section:
`extract-abi.py` reads C *functions*, and an Objective-C method is not one, so none of
these boxes comes from the generated checklist above.  A pair of `#if`/`#else` arms
spelling the same selector is one box and is named where it occurs.)

- [x] NSColor (EmacsColor)  (`@implementation` 157, `@end` 206) - landed as a Swift
      `extension NSColor` with the same four methods (C 152-206); a *category*, so it
      declares no ivars and the `NSColor` class itself is AppKit's
  - [x] `+ (NSColor *)colorForEmacsRed:(CGFloat)red green:(CGFloat)green blue:(CGFloat)blue alpha:(CGFloat)alpha`  (line 158) -> `class func colorForEmacsRed(_:green:blue:alpha:)`; the `NS_IMPL_COCOA && MAC_OS_X_VERSION_MAX_ALLOWED >= 1070` arm with the `NSAppKitVersion.current >= NSAppKitVersion.macOS10_7` test
  - [x] `- (NSColor *)colorUsingDefaultColorSpace`  (line 175) -> `func colorUsingDefaultColorSpace()`
  - [x] `+ (NSColor *)colorWithUnsignedLong:(unsigned long)c`  (line 185) -> `class func colorWithUnsignedLong(_:)`
  - [x] `- (unsigned long)unsignedLong`  (line 195) -> `func unsignedLong() -> UInt`
- [ ] EmacsBell  (`@interface` 1333, `@end` 1343 - a file-local declaration; its three
      methods are the `@implementation`'s below)
  - [ ] `- (void)show:(NSView *)view`  (line 1340)
  - [ ] `- (void)hide`  (line 1341)
  - [ ] `- (void)remove`  (line 1342)
- [ ] EmacsBell  (`@implementation` 1345, `@end` 1430)
  - [ ] `- (id)init`  (line 1347)
  - [ ] `- (void)show:(NSView *)view`  (line 1373)
  - [ ] `- (void)hide`  (line 1402)
  - [ ] `-(void)remove`  (line 1418)
- [ ] EmacsApp  (`@implementation` 6225, `@end` 6912)
  - [ ] `- (id)init`  (line 6227)
  - [ ] `- (void)run`  (line 6257)
  - [ ] `- (void)stop: (id)sender`  (line 6295)
  - [ ] `- (void)logNotification: (NSNotification *)notification`  (line 6306)
  - [ ] `- (void)sendEvent: (NSEvent *)theEvent`  (line 6317)
  - [ ] `- (void)showPreferencesWindow: (id)sender`  (line 6402)
  - [ ] `- (void)newFrame: (id)sender`  (line 6416)
  - [ ] `- (BOOL) openFile: (NSString *)fileName`  (line 6433)
  - [ ] `- (void) updateMonitors: (NSNotification *) notification`  (line 6454)
  - [ ] `- (void)applicationDidFinishLaunching: (NSNotification *)notification`  (line 6490)
  - [ ] `- (void)antialiasThresholdDidChange:(NSNotification *)notification`  (line 6556)
  - [ ] `- (void) systemWillSleep:(NSNotification *)notification`  (line 6565)
  - [ ] `- (void) systemDidWake:(NSNotification *)notification`  (line 6577)
  - [ ] `- (BOOL) applicationSupportsSecureRestorableState: (NSApplication *)app`  (line 6610)
  - [ ] `- (void) terminate: (id)sender`  (line 6615)
  - [ ] `- (NSApplicationTerminateReply)applicationShouldTerminate: (id)sender`  (line 6655)
  - [ ] `- (BOOL)application: sender openFile: (NSString *)file`  (line 6682)
  - [ ] `- (BOOL)application: sender openTempFile: (NSString *)file`  (line 6691)
  - [ ] `- (BOOL)application: sender openFileWithoutUI: (NSString *)file`  (line 6700)
  - [ ] `- (void)application: sender openFiles: (NSArray *)fileList`  (line 6708)
  - [ ] `- (NSMenu *)applicationDockMenu: (NSApplication *) sender`  (line 6725)
  - [ ] `- (void)applicationWillBecomeActive: (NSNotification *)notification`  (line 6732)
  - [ ] `- (void)applicationDidBecomeActive: (NSNotification *)notification`  (line 6738)
  - [ ] `- (void)applicationDidResignActive: (NSNotification *)notification`  (line 6752)
  - [ ] `- (void)timeout_handler: (NSTimer *)timedEntry`  (line 6769)
  - [ ] `- (void)sendFromMainThread:(id)unused`  (line 6778)
  - [ ] `- (void)fd_handler:(id)unused`  (line 6783)
  - [ ] `- (void)requestService: (NSPasteboard *)pboard userData: (NSString *)userData error: (NSString **)error`  (line 6881)
  - [ ] `- (BOOL)fulfillService: (NSString *)name withArg: (NSString *)arg`  (line 6891)
- [ ] EmacsView  (`@implementation` 7039, `@end` 9746)
  - [ ] `- (void)windowDidEndLiveResize:(NSNotification *)notification`  (line 7041)
  - [ ] `- (void) setWindowClosing: (BOOL)closing`  (line 7047)
  - [ ] `- (void)dealloc`  (line 7055)
  - [ ] `- (void) changeFont: (id) sender`  (line 7080)
  - [ ] `- (void) noteUserSelectedFont`  (line 7109)
  - [ ] `- (void) noteUserCancelledSelection`  (line 7128)
  - [ ] `- (Lisp_Object) showFontPanel`  (line 7140)
  - [ ] `- (BOOL)acceptsFirstResponder`  (line 7204)
  - [ ] `- (BOOL)acceptsFirstMouse:(NSEvent *)theEvent`  (line 7211)
  - [ ] `- (void)resetCursorRects`  (line 7217)
  - [ ] `- (void)keyDown: (NSEvent *)theEvent`  (line 7243)
  - [ ] `- (void) insertText: (id) string replacementRange: (NSRange) replacementRange`  (line 7435)
  - [ ] `- (void) setMarkedText: (id) string selectedRange: (NSRange) selectedRange replacementRange: (NSRange) replacementRange`  (line 7444)
  - [ ] `- (nullable NSAttributedString *) attributedSubstringForProposedRange: (NSRange) range actualRange: (nullable NSRangePointer) actualRange`  (line 7451)
  - [ ] `- (NSRect) firstRectForCharacterRange: (NSRange) range actualRange: (nullable NSRangePointer) actualRange`  (line 7458)
  - [ ] `- (NSRect)accessibilityFrame`  (line 7464)
  - [ ] `- (void)insertText: (id)aString`  (line 7481)
  - [ ] `- (void)setMarkedText: (id)aString selectedRange: (NSRange)selRange`  (line 7538)
  - [ ] `- (void)deleteWorkingText`  (line 7572)
  - [ ] `- (BOOL)hasMarkedText`  (line 7593)
  - [ ] `- (NSRange)markedRange`  (line 7601)
  - [ ] `- (void)unmarkText`  (line 7613)
  - [ ] `- (NSRect)firstRectForCharacterRange: (NSRange)theRange`  (line 7651)
  - [ ] `- (NSInteger)conversationIdentifier`  (line 7699)
  - [ ] `- (void)doCommandBySelector: (SEL)aSelector`  (line 7705)
  - [ ] `- (NSArray *)validAttributesForMarkedText`  (line 7725)
  - [ ] `- (NSRange)selectedRange`  (line 7733)
  - [ ] `- (NSUInteger)characterIndexForPoint: (NSPoint)thePoint`  (line 7759)
  - [ ] `- (unsigned int)characterIndexForPoint: (NSPoint)thePoint`  (line 7761; the `#if defined (NS_IMPL_COCOA) || GNUSTEP_GUI…` arm of the box above)
  - [ ] `- (NSAttributedString *)attributedSubstringFromRange: (NSRange)theRange`  (line 7769)
  - [ ] `- (void)mouseDown: (NSEvent *)theEvent`  (line 7783)
  - [ ] `- (void)rightMouseDown: (NSEvent *)theEvent`  (line 8022)
  - [ ] `- (void)otherMouseDown: (NSEvent *)theEvent`  (line 8029)
  - [ ] `- (void)mouseUp: (NSEvent *)theEvent`  (line 8036)
  - [ ] `- (void)rightMouseUp: (NSEvent *)theEvent`  (line 8043)
  - [ ] `- (void)otherMouseUp: (NSEvent *)theEvent`  (line 8050)
  - [ ] `- (void) scrollWheel: (NSEvent *)theEvent`  (line 8057)
  - [ ] `- (void)mouseMoved: (NSEvent *)e`  (line 8065)
  - [ ] `- (void)mouseDragged: (NSEvent *)e`  (line 8138)
  - [ ] `- (void)rightMouseDragged: (NSEvent *)e`  (line 8145)
  - [ ] `- (void)otherMouseDragged: (NSEvent *)e`  (line 8152)
  - [ ] `- (void) magnifyWithEvent: (NSEvent *) event`  (line 8159)
  - [ ] `- (BOOL)windowShouldClose: (id)sender`  (line 8195)
  - [ ] `- (void)updateFramePosition`  (line 8212)
  - [ ] `- (NSSize)windowWillResize: (NSWindow *)sender toSize: (NSSize)frameSize`  (line 8243)
  - [ ] `- (void)viewDidEndLiveResize`  (line 8357)
  - [ ] `- (void)resizeWithOldSuperviewSize: (NSSize)oldSize`  (line 8373)
  - [ ] `- (void)windowDidBecomeKey: (NSNotification *)notification`  (line 8404)
  - [ ] `- (void)windowDidBecomeKey`  (line 8411)
  - [ ] `- (void)windowWillClose: (NSNotification *)notification`  (line 8472)
  - [ ] `- (void)deferred_UAZoomChangeFocus_handler: (NSTimer *)timer`  (line 8482)
  - [ ] `- (void)windowDidResignKey: (NSNotification *)notification`  (line 8494)
  - [ ] `- (void)windowWillMiniaturize: sender`  (line 8532)
  - [ ] `- (void)setFrame:(NSRect)frameRect`  (line 8538)
  - [ ] `- (BOOL)isFlipped`  (line 8547)
  - [ ] `- (BOOL)isOpaque`  (line 8553)
  - [ ] `- (instancetype) initFrameFromEmacs: (struct frame *)f`  (line 8559)
  - [ ] `- (void)windowDidMove: sender`  (line 8645)
  - [ ] `- (BOOL)windowShouldZoom: (NSWindow *)sender toFrame: (NSRect)newFrame`  (line 8656)
  - [ ] `- (NSRect)windowWillUseStandardFrame:(NSWindow *)sender defaultFrame:(NSRect)defaultFrame`  (line 8670)
  - [ ] `- (void)windowDidDeminiaturize: sender`  (line 8788)
  - [ ] `- (void)windowDidExpose: sender`  (line 8806)
  - [ ] `- (void)windowDidMiniaturize: sender`  (line 8820)
  - [ ] `- (NSApplicationPresentationOptions)window:(NSWindow *)window willUseFullScreenPresentationOptions: (NSApplicationPresentationOptions)proposedOptions`  (line 8837)
  - [ ] `- (void)windowWillEnterFullScreen:(NSNotification *)notification`  (line 8845)
  - [ ] `- (void)windowWillEnterFullScreen`  (line 8850)
  - [ ] `- (void)windowDidEnterFullScreen:(NSNotification *)notification`  (line 8856)
  - [ ] `- (void)adjustEmacsFrameRect`  (line 8862)
  - [ ] `- (void)windowDidEnterFullScreen`  (line 8871)
  - [ ] `- (void)windowWillExitFullScreen:(NSNotification *)notification`  (line 8906)
  - [ ] `- (void)windowWillExitFullScreen`  (line 8912)
  - [ ] `- (void)windowDidExitFullScreen:(NSNotification *)notification`  (line 8924)
  - [ ] `- (void)windowDidExitFullScreen`  (line 8930)
  - [ ] `- (BOOL)fsIsNative`  (line 8952)
  - [ ] `- (BOOL)isFullscreen`  (line 8957)
  - [ ] `- (void)updateCollectionBehavior`  (line 8981)
  - [ ] `- (void)toggleFullScreen: (id)sender`  (line 9017)
  - [ ] `- (void)handleFS`  (line 9126)
  - [ ] `- (void) setFSValue: (int)value`  (line 9177)
  - [ ] `- (void)mouseEntered: (NSEvent *)theEvent`  (line 9202)
  - [ ] `- (void)mouseExited: (NSEvent *)theEvent`  (line 9211)
  - [ ] `- (instancetype)menuDown: sender`  (line 9231)
  - [ ] `- (instancetype)toolbarClicked: (id)item`  (line 9250)
  - [ ] `- (BOOL) validateToolbarItem: (NSToolbarItem *) toolbarItem`  (line 9270)
  - [ ] `- (instancetype)toggleToolbar: (id)sender`  (line 9275)
  - [ ] `- (CALayer *)makeBackingLayer`  (line 9290)
  - [ ] `- (void)lockFocus`  (line 9301)
  - [ ] `- (void)unlockFocus`  (line 9314)
  - [ ] `- (void)windowDidChangeBackingProperties:(NSNotification *)notification`  (line 9323)
  - [ ] `- (void)copyRect:(NSRect)srcRect to:(NSPoint)dest`  (line 9340)
  - [ ] `- (void)layoutSublayersOfLayer:(CALayer *)layer`  (line 9410)
  - [ ] `- (void)drawRect: (NSRect)rect`  (line 9431)
  - [ ] `-(NSDragOperation) draggingEntered: (id <NSDraggingInfo>) sender`  (line 9452)
  - [ ] `-(BOOL) prepareForDragOperation: (id <NSDraggingInfo>) sender`  (line 9468)
  - [ ] `- (BOOL) wantsPeriodicDraggingUpdates`  (line 9481)
  - [ ] `- (NSDragOperation) draggingUpdated: (id <NSDraggingInfo>) sender`  (line 9486)
  - [ ] `- (BOOL) performDragOperation: (id <NSDraggingInfo>) sender`  (line 9538)
  - [ ] `- (id) validRequestorForSendType: (NSString *)typeSent returnType: (NSString *)typeReturned`  (line 9657)
  - [ ] `- (BOOL) readSelectionFromPasteboard: (NSPasteboard *)pb`  (line 9680)
  - [ ] `- (BOOL) writeSelectionToPasteboard: (NSPasteboard *)pb types: (NSArray *)types`  (line 9688)
  - [ ] `- (instancetype)setMiniwindowImage: (BOOL) setMini`  (line 9721)
  - [ ] `- (int) fullscreenState`  (line 9741)
- [ ] EmacsWindow  (`@implementation` 9756, `@end` 10610)
  - [ ] `- (instancetype) initWithEmacsFrame: (struct frame *) f`  (line 9759)
  - [ ] `- (instancetype) initWithEmacsFrame: (struct frame *) f fullscreen: (BOOL) fullscreen screen: (NSScreen *) screen`  (line 9765)
  - [ ] `- (void)createToolbar: (struct frame *)f`  (line 9878)
  - [ ] `- (void)dealloc`  (line 9912)
  - [ ] `- (NSInteger) borderWidth`  (line 9928)
  - [ ] `- (void)setParentChildRelationships`  (line 9934)
  - [ ] `- (void)orderFront:(id)sender`  (line 10043)
  - [ ] `- (void)makeKeyAndOrderFront:(id)sender`  (line 10057)
  - [ ] `- (NSInteger) orderedIndex`  (line 10074)
  - [ ] `- (void)orderBack:(id)sender`  (line 10098)
  - [ ] `- (BOOL)restackWindow:(NSWindow *)win above:(BOOL)above`  (line 10124)
  - [ ] `- (id)accessibilityAttributeValue:(NSString *)attribute`  (line 10158)
  - [ ] `- (NSRect)constrainFrameRect:(NSRect)frameRect toScreen:(NSScreen *)screen`  (line 10230)
  - [ ] `- (void)performZoom:(id)sender`  (line 10299)
  - [ ] `- (void)zoom:(id)sender`  (line 10306)
  - [ ] `- (void)setAppearance`  (line 10398)
  - [ ] `- (void)setFrame:(NSRect)windowFrame display:(BOOL)displayViews`  (line 10424)
  - [ ] `- (void)setFrame:(NSRect)windowFrame display:(BOOL)displayViews animate:(BOOL)performAnimation`  (line 10433)
  - [ ] `- (void)setFrameTopLeftPoint:(NSPoint)point`  (line 10444)
  - [ ] `- (BOOL)canBecomeKeyWindow`  (line 10452)
  - [ ] `- (BOOL)canBecomeMainWindow`  (line 10457)
  - [ ] `- (void) setLastDragEvent: (NSEvent *) event`  (line 10463)
  - [ ] `- (NSDragOperation) draggingSourceOperationMaskForLocal: (BOOL) is_local`  (line 10470)
  - [ ] `- (void) draggedImage: (NSImage *) image endedAt: (NSPoint) screen_point operation: (NSDragOperation) operation`  (line 10475)
  - [ ] `- (void) draggedImage: (NSImage *) dragged_image movedTo: (NSPoint) screen_point`  (line 10482)
  - [ ] `- (BOOL) mustNotDropOn: (NSView *) receiver`  (line 10522)
  - [ ] `- (NSDragOperation) beginDrag: (NSDragOperation) op forPasteboard: (NSPasteboard *) pasteboard withMode: (enum ns_return_frame_mode) mode returnFrameTo: (struct frame **) frame_return prohibitSame: (BOOL) prohibit_same_frame followTooltip: (BOOL) follow_tooltip`  (line 10528)
- [ ] EmacsScroller  (`@implementation` 10620, `@end` 11117)
  - [ ] `+ (CGFloat) scrollerWidth`  (line 10626)
  - [ ] `- (instancetype)initFrame: (NSRect )r window: (Lisp_Object)nwin`  (line 10650)
  - [ ] `- (void)setFrame: (NSRect)newRect`  (line 10703)
  - [ ] `- (void)dealloc`  (line 10719)
  - [ ] `- (instancetype)condemn`  (line 10734)
  - [ ] `- (instancetype)reprieve`  (line 10742)
  - [ ] `-(bool)judge`  (line 10750)
  - [ ] `- (void) mark`  (line 10777)
  - [ ] `- (void)resetCursorRects`  (line 10788)
  - [ ] `- (int) checkSamePosition: (int) position portion: (int) portion whole: (int) whole`  (line 10806)
  - [ ] `- (instancetype)setPosition: (int)position portion: (int)portion whole: (int)whole`  (line 10814)
  - [ ] `- (void) sendScrollEventAtLoc: (float)loc fromEvent: (NSEvent *)e`  (line 10850)
  - [ ] `- (instancetype)repeatScroll: (NSTimer *)scrollEntry`  (line 10894)
  - [ ] `- (void)mouseDown: (NSEvent *)e`  (line 10929)
  - [ ] `- (void)mouseDragged: (NSEvent *)e`  (line 11051)
  - [ ] `- (void)mouseUp: (NSEvent *)e`  (line 11094)
  - [ ] `- (void) scrollWheel: (NSEvent *)theEvent`  (line 11109)
- [ ] EmacsLayer  (`@implementation` 11128, `@end` 11396)
  - [ ] `- (id) initWithDoubleBuffered: (bool)db`  (line 11152)
  - [ ] `- (void) setColorSpace: (CGColorSpaceRef)cs`  (line 11170)
  - [ ] `- (void) setDoubleBuffered: (bool)db`  (line 11181)
  - [ ] `- (void) dealloc`  (line 11190)
  - [ ] `- (void) releaseSurfaces`  (line 11199)
  - [ ] `- (BOOL) checkDimensions`  (line 11222)
  - [ ] `- (CGContextRef) getContext`  (line 11235)
  - [ ] `- (void) releaseContext`  (line 11324)
  - [ ] `- (void) display`  (line 11341)
  - [ ] `- (void) copyContentsTo: (IOSurfaceRef) destination`  (line 11364)
- [ ] EmacsDocument  (`@implementation` 11404, `@end` 11406)

## Macros defined — 23
- [ ] DOCK_IGNORE_LIMIT  (line 708, body c1dfd96eea8c)
- [x] EV_BUTTON  (line 438, body 02ec0a9e0dff) — landed as `func EV_BUTTON (_ e: NSEvent) -> Int`
- [x] EV_MODIFIERS  (line 425, body 0a951d59f9e1) — landed as `func EV_MODIFIERS (_ e: NSEvent) -> UInt32`
- [x] EV_MODIFIERS2  (line 397, body 8155b8d11eb3) — landed as `func EV_MODIFIERS2 (_ flags: UInt, _ kind: Lisp_Object) -> UInt32`; C's `NSUInteger`→`unsigned int` conversions are `truncatingIfNeeded:`
- [x] EV_TIMESTAMP  (line 444, body 8e139f589481) — landed as `func EV_TIMESTAMP (_ e: NSEvent) -> Double`
- [ ] EV_TRAILER  (line 448, body 0dd430c310a9) — **held** to C 5013: its body calls the hold queue and `EV_TRAILER2`, and `ns_send_appdefined` (C 5013) is this file's own function, so no Swift text can exist before it does (see Decisions)
- [ ] EV_TRAILER2  (line 455, body bf5ab84e8621) — **held** to C 5013, like `EV_TRAILER` (its callees are C 498's `hold_event` and C 5013's `ns_send_appdefined`)
- [x] EV_UDMODIFIERS  (line 427, body 89301cbc268c) — landed as `func EV_UDMODIFIERS (_ e: NSEvent) -> Int32`, C's ten comparisons transcribed one for one
- [x] NO_APPDEFINED_DATA  (line 318, body 9d95217d51d9)  — landed as a file-scope `let` of the same name
- [ ] NSAppKitVersionNumber10_10  (line 10407, body ed9f6e93628b)
- [ ] NSAppKitVersionNumber10_9  (line 6262, body 74b10c057ded)
- [x] NSLeftAlternateKeyMask  (line 355, body a08e8dccf9c4)  — landed as a file-scope `let` of the same name
- [x] NSLeftCommandKeyMask  (line 353, body 6f54877ddc32)  — landed as a file-scope `let` of the same name
- [x] NSLeftControlKeyMask  (line 351, body 5cc979ee22b5)  — landed as a file-scope `let` of the same name
- [x] NSRightAlternateKeyMask  (line 356, body 2e125f92fda6)  — landed as a file-scope `let` of the same name
- [x] NSRightCommandKeyMask  (line 354, body 9d1bbbd0282a)  — landed as a file-scope `let` of the same name
- [x] NSRightControlKeyMask  (line 352, body 5a769eb9b670)  — landed as a file-scope `let` of the same name
- [x] NS_FUNCTION_KEY_MASK  (line 350, body 91eddf5a25af)  — landed as a file-scope `let` of the same name
- [ ] NS_KEYLOG  (line 7241, body b6589fc6ab0d)
- [ ] NS_PARENT_WINDOW_LEFT_POS  (line 803, body 74769de1ca23)
- [ ] NS_PARENT_WINDOW_TOP_POS  (line 804, body 7f398f29c646)
- [ ] SCROLL_BAR_CONTINUOUS_DELAY  (line 10624, body cea8ad3417bc)
- [ ] SCROLL_BAR_FIRST_DELAY  (line 10623, body 1b390cd54a0c)

## Types and their fields — 1
- [ ] EmacsMargins: CGFloat top, CGFloat bottom, CGFloat left, CGFloat right

(Not counted here, the way `docs/ports/nsselect.md` does not count `Lisp_Subr`/`specpdl_ref`: the
unit also declares a Swift **view** of `struct input_event` (src/termhooks.h 371-412), which this
file's `emacs_event` points at.  The record is termhooks.h's and already has a Rust mirror
(`femacs/src/include/termhooks.rs`); a Swift unit cannot see a Rust record, so the view repeats
the mirror's member names, and its layout is measured rather than assumed - clang `sizeof 64`,
`align 8`, and the offsets in `## Evidence`, which the Swift side matches exactly.)

## File-scope objects — 39
- [x] static EmacsMenu dockMenu  (line 82, static)  — landed
- [x] static EmacsMenu mainMenu  (line 84, static)  — landed
- [x] static Lisp_Object last_known_monitors  (line 88, static)  — landed
- [x] volatile int nstrace_num  (line 100)  — dead (inside `#if NSTRACE_ENABLED`, C 96-149) - recorded, not transcribed
- [x] volatile int nstrace_depth  (line 101)  — dead, like the box above
- [x] volatile int nstrace_enabled_global  (line 109)  — dead, like the box above
- [x] float ns_antialias_threshold  (line 298)  — landed
- [x] NSArray ns_send_types  (line 300)  — landed
- [x] static NSArray ns_drag_types  (line 301, static)  — landed
- [x] long context_menu_value  (line 306)  — landed
- [x] static struct frame ns_updating_frame  (line 309, static)  — landed
- [x] static int ns_window_num  (line 310, static)  — landed
- [x] static BOOL gsaved  (line 311, static)  — landed
- [x] static BOOL ns_menu_bar_is_hidden  (line 313, static)  — landed
- [x] static BOOL send_appdefined  (line 317, static)  — landed
- [x] static int last_appdefined_event_data  (line 319, static)  — landed
- [x] static NSTimer timed_entry  (line 320, static)  — landed
- [x] static NSTimer scroll_repeat_entry  (line 321, static)  — landed
- [x] static fd_set select_readfds  (line 322, static)  — landed
- [x] static fd_set select_writefds  (line 322, static)  — landed
- [x] static int select_nfds  (line 324, static)  — landed
- [x] static struct timespec select_timeout  (line 325, static)  — landed
- [x] static int selfds  (line 326, static)  — landed
- [x] static pthread_mutex_t select_mutex  (line 327, static)  — landed
- [x] static NSAutoreleasePool outerpool  (line 328, static)  — landed
- [x] static struct input_event emacs_event  (line 329, static)  — landed
- [x] static struct input_event q_event_ptr  (line 330, static)  — landed
- [x] static int n_emacs_events_pending  (line 331, static)  — landed
- [x] static BOOL ns_do_open_file  (line 334, static)  — landed
- [x] static BOOL ns_last_use_native_fullscreen  (line 335, static)  — landed
- [x] static BOOL any_help_event_p  (line 340, static)  — landed
- [ ] static BOOL ns_is_UAZoomEnabled  (line 1095, static)
- [ ] static unsigned long ns_UAZoomEnabled_last_called_time_ns  (line 1096, static)
- [ ] static NSTimer ns_deferred_UAZoomChangeFocus_timer  (line 1100, static)
- [ ] static NSRect ns_UAZoom_cursor_rect_new  (line 1154, static)
- [ ] static NSRect ns_UAZoom_cursor_rect_old  (line 1155, static)
- [ ] static BOOL ns_update_was_UAZoomEnabled  (line 1159, static)
- [ ] static EmacsBell bell_view  (line 1433, static)
- [ ] static NSMutableDictionary fringe_bmp  (line 3089, static)


The eight names below are `src/nsterm.m` file-scope objects that `extract-abi.py`
does not list (a second declarator on one line, an anonymous `struct`, or a name the reader
otherwise skips); they are added by hand so the checklist is complete - see Decisions.

- [x] NSArray ns_return_types  (line 300) — landed; same shape as `ns_send_types`
- [x] NSString ns_app_name  (line 302) — landed (exported, `@_cdecl` form)
- [x] struct ns_display_info x_display_list  (line 305) — landed (P-55 untyped pointer; exported)
- [x] static int select_valid  (line 324) — landed (the second declarator on `select_nfds`'s line)
- [x] static NSMutableArray ns_pending_files  (line 332) — landed
- [x] static NSMutableArray ns_pending_service_names  (line 332) — landed
- [x] static NSMutableArray ns_pending_service_args  (line 333) — landed
- [x] static struct { … } hold_event_q  (lines 342-347) — landed as three fields (`hold_event_q_q`/`_nr`/`_cap`)

# ----------------------------------------------------------------------
## Decisions

### Progress (this lane is incremental)

`src/nsterm.m` is 11,811 lines - the largest `.m` of batch F - and is transcribed in C order,
one slice per round (`docs/ports/xdisp.md` is the same shape for `src/xdisp.c`).  **Round 1 landed
C 60-390**: the `#include`/`#import` prologue (C 60-81) recorded rather than transcribed, the
first three statics (C 82-88), the dead `NSTRACE_ENABLED` block (C 96-149), the
`NSColor (EmacsColor)` category (C 152-206), the `convert_ns_to_X_keysym` table (C 214-293), the
file-scope objects (C 295-347), the seven modifier masks (C 349-356) and the file's first two
functions `mod_of_kind`/`ev_modifiers_helper` (C 358-389).  **Round 2 lands C 391-519** - the
`EV_*` accessors' five self-contained macros (C 397-444), the forward declarations recorded
(C 473-475), `ns_init_events`/`ns_finish_events` (C 484-495) and `hold_event`/`append2`
(C 498-519) - so the covered range is now **C 60-519**, with `EV_TRAILER`/`EV_TRAILER2`
(C 448-470) held to C 5013 (below).

**The next slice starts at C 522** (`ns_relocate`, whose `NS_SELF_CONTAINED` arm needs a ruling:
the macro belongs to the build, not to `config.h`, and the Cocoa app bundle defines it).

Ticked above: **9 of the 142 function boxes** (the six landed - `mod_of_kind`,
`ev_modifiers_helper`, `ns_init_events`, `ns_finish_events`, `hold_event`, `append2` - and the
three dead `nstrace_*`), **13 of the 23 macro boxes** (the eight masks and `NO_APPDEFINED_DATA`,
plus the five `EV_*` accessors; the two held ones stay open), **39 of the 47 file-scope-object
boxes** (31 of the generator's 39 plus the 8 hand-added ones), and **1 of the 8 Objective-C
classes** (`NSColor (EmacsColor)`, all four of its methods).  The other 133 function boxes, 10
macro boxes, 7 class boxes, 189 method boxes and the one type box (`struct EmacsMargins`, C 650)
are later slices.

**The slice's text had been landed before this round but measured with `swiftc -parse`, which
resolves no names.** This lane's compile gate is `swiftc -parse-as-library -c` (the form
`docs/ports/nsselect.md` uses), and on that revision it fails with twelve errors; all twelve are
fixed in this round:

1. `NSAppKitVersionNumber` / `NSAppKitVersionNumber10_7` are renamed in Swift - AppKit.apinotes
   8624-8626 maps the first to `NSAppKitVersion.current` and each vintage to
   `NSAppKitVersion.macOS10_x`; the port compares `.rawValue`, the spelling the landed
   `nsimage.swift` uses for the same pair.
2. `NSEventModifierFlagControl`/`…Command`/`…Option` are `NSEvent.ModifierFlags.control`/
   `.command`/`.option` in Swift (six uses); C's own spellings stay in the `/// C:` lines.
3. `NSAutoreleasePool` is "not available in automatic reference counting mode" - see the
   `outerpool` decision below.
4. `nsUseSrgbColorspace` was an *instance* property of the `NSColor` extension and
   `+colorForEmacsRed:` is a *class* method ("instance member 'nsUseSrgbColorspace' cannot be
   used on type 'NSColor'"); the port reads the imported C variable directly instead.
5. `EmacsMenu` is another unit's class - see the P-55 decision below.

### The three kinds of declaration this unit carries

Swift has no `#include`, so the unit carries its C environment itself, the way
`docs/ports/nsimage.md`'s `## ABI` describes for the lane: `@_silgen_name` imports of the ported
core and of the lane's own C constructs, this unit's own C interface exported with `@_cdecl`, and
the constructs that have no symbol at all spelled out here.  `src/nsterm.h` already has a Rust
mirror (`femacs/src/include/nsterm.rs`, status 3.199) - this unit does not repeat it; it declares
only what its own slice reads.

### Round 2: the `EV_*` accessors, the held pair, and the event-loop utilities

**The seven `ns_*_modifier` variables are imported, not declared here.**  C 11608-11670 defines
them with `DEFVAR_LISP` (i.e. as fields of `globals`), and the macro block reads them ~11,000
lines earlier: a fileprivate Swift global of this unit's own would not exist yet at C 397.  The
lane's shape for a C variable the core exports is `@_silgen_name("…") var …: Lisp_Object` - the
reading `docs/ports/macfont.swift` gives `Votf_script_alist`/`Vscript_representative_chars`, and
the integrator's ruling for this slice.  The same holds for `QCmouse` (C 11795's `DEFSYM`, **this
file's own**, so it is absent from `docs/baseline/globals/symbols.tsv`, whose table is generated
from a frozen build that never compiles this file), for the four `termhooks.h` enumerators the
accessors read (`up_modifier`/`down_modifier`/`hyper_modifier`/`shift_modifier`, reached as
C-ABI entries of their own names - `docs/ports/nsselect.md`'s `PVEC_SUBR` reading), and for
`xrealloc`/`nconc2`/`list1`/`EVENT_INIT`.  **Each is an owed symbol** until the core (or the
slice that owns its definition) exports it; `## ABI` (3) lists them, and the round's report
hands the list to the integrator's `docs/baseline/undef-pending.txt` ratchet.

**`EV_TRAILER`/`EV_TRAILER2` are held to C 5013.**  Swift has no forward declaration and no
"declare now, define later" pair (`@_silgen_name` on a declaration plus a later definition of the
same name is an "invalid redeclaration", measured), and the two macros' bodies call `hold_event`
(this file's C 498) and `ns_send_appdefined` (this file's C 5013) - both *this* unit's functions,
so neither an import nor a placeholder exists.  C's own macro text has the same shape: it is
written at C 448-470 and only *expanded* after those definitions (the first expansion is
C 6412), so holding them to C 5013 matches the reference's own order, and nothing before C 6412
needs them.  The slice that lands C 5013 lands them.

**`struct input_event` gets a Swift view.**  `emacs_event` points at termhooks.h 371-412's
record and `hold_event` copies one value (`hold_event_q.q[nr++] = *event`).  Swift cannot see the
Rust mirror, so the unit declares the same members under the mirror's names and **measures** the
layout rather than assuming it - clang on the frozen configuration and the Swift side agree
member for member (`## Evidence`).  The carried `Lisp_Subr`/`specpdl_ref`/`Lisp_Fwd` of
`nsselect.swift` are the same situation, and like those this view is not counted a second time in
the checklist.

**`ns_init_events`/`ns_finish_events` are the unit's C interface.**  Both are declared in
src/nsterm.h (1257-1258) and called from src/nsfns.m (2492/2495), so they are `@_cdecl` exports
keeping C's names.  `@_cdecl` refuses `UnsafeMutablePointer<input_event>` ("the type of the
parameter cannot be represented in Objective-C", measured), so the exported parameter is
`UnsafeMutableRawPointer` and the body re-binds it to the view; at the ABI it is C's
`struct input_event *` either way.

**`hold_event`'s copy and `append2`'s `list`.**  The queue's element size is
`MemoryLayout<input_event>.stride` (64, measured), so C's `xrealloc (q, cap * sizeof *q)` and its
value copy transcribe literally, and `xrealloc`'s result is taken apart without an optional in
between (it never returns NULL - src/alloc.c's `memory_full`); a NULL `event` traps where the
reference's unguarded `*event` faults.  C's `list (item)` is the variadic macro
`listn (countof (…), …)` (src/lisp.h 4493), which for one argument is `list1`'s call - the lane's
reading for a fixed-arity spelling of a variadic macro (`docs/ports/nsselect.md`'s `calln`).

**The forward declarations carry no Swift text.**  C 473-475 declares
`ns_condemn_scroll_bars`/`ns_judge_scroll_bars` (defined at C 5575 and C 5622) to get around C's
one-pass compilation; Swift resolves names file-wide, so there is nothing to write, and each
function lands once at its own definition.

### C data symbols are exported as C-ABI *functions*

`ns_send_types`, `ns_return_types`, `ns_app_name` (src/nsterm.h 774-775) and `x_display_list`,
`context_menu_value` (931-933) are C *data* symbols other units read, and `@_cdecl` refuses a
`var` while `@_silgen_name` on a `var` is an *import* (both measured in this lane, nsselect.md).
Each is therefore spelled as a C-ABI function of its own name returning the value, and `nsselect.swift`
already imports `ns_send_types()`/`ns_return_types()` that way.  `x_display_list` returns C's
pointer depth with P-55's untyped stand-in (`struct ns_display_info` is this lane's header, and
its mirror's record is not this unit's); `context_menu_value` is C's `long` = `Int`.

### `EmacsMenu *` is another unit's class (P-55)

`EmacsMenu` is src/nsterm.h 540's class, implemented by src/nsmenu.m - this lane's
`femacs/platform/macos/nsmenu.swift`, which is compiled as its own unit and is not joint with
this one.  C's `EmacsMenu *dockMenu`/`mainMenu` therefore keep P-55's untyped pointer, and the
class's methods are reached as C-ABI entries with the receiver first (the reading
`docs/ports/nsselect.md` gives `EmacsWindow`).

### `outerpool` is a pool handle under ARC

C's `static NSAutoreleasePool *outerpool` (C 328) cannot be spelled as that type: Swift's ARC
marks `NSAutoreleasePool` unavailable (error, measured).  The unit keeps the *handle* as an
untyped pointer; the slices that create and re-create it (C 560-568 `ns_init_pool`, and the
release/re-create pairs at C 5154-5155 and 5982-5983) will push and pop the runtime's
`objc_autoreleasePoolPush`/`objc_autoreleasePoolPop` - the pair `@autoreleasepool` itself
compiles to.  The two symbols are in libobjc but not in the SDK's Swift-visible headers
(measured: `@_silgen_name` or a C prototype reaches them; neither name is in scope in Swift), so
that slice will reach them by name.  Pending, and named here so the later slice does not invent a
different shape.

### `ns_use_srgb_colorspace` is a `DEFVAR_BOOL` of *this* file

C 11716 (`syms_of_nsterm`) defines `ns_use_srgb_colorspace` with `DEFVAR_BOOL`, and the generated
globals.h spells it as a macro over the `globals` record's field; this file reads it at C 162 and
C 178.  The landed `nsimage.swift` imports the symbol as a C-ABI *variable*
(`@_silgen_name("ns_use_srgb_colorspace") var …: Bool`, its line 145), so this unit matches that
shape rather than the function-returning-a-pointer form a first reading suggests; the two units
must agree because each is compiled alone and the linker sees one symbol.  **Open item:** nothing
in `femacs/src` exports that data symbol today (`rg ns_use_srgb_colorspace femacs/` finds the
Swift import only), so the port of `syms_of_nsterm` - or the core's globals record - has to
supply it.

### `parse_solitary_modifier` returns `int`, not `Lisp_Object`

C's own declaration (src/keyboard.h 463) and definition (src/keyboard.c 7920) both return `int`,
and that is what `ev_modifiers_helper` stores into its `unsigned int modifiers`.  The port's
`femacs/src/keymap.rs` mirror declares it returning `Lisp_Object` (a mismatch carried from the
previous round's reading and flagged, not fixed here: `keymap.rs` is not this lane's file).
The Swift side takes C's `Int32` and converts with `UInt32(bitPattern:)`.

### `convert_ns_to_X_keysym` keeps C's flat pair layout

C's table (C 217-293) is a flat array of `(key, keysym)` pairs whose one reader, `ns_convert_key`
(C 2648), walks it with `keysym += 2` and reads `[keysym]`/`[keysym + 1]`; the port keeps the flat
`[UInt32]` so that the reader transcribes literally, and C's inline annotations
(`/* 8: Not on some KBs. */` …) become trailing `// C <line>:` comments.  The enumerators are
AppKit's own (`NSEvent.h`'s function-key enum, `NSText.h`'s character enum), which Swift imports
as `Int`; `NSEventModifierFlagNumericPad` is `NSEvent.ModifierFlags.numericPad`.

### Dead arms (P-08: recorded, not transcribed)

* C 96-149, the whole `#if NSTRACE_ENABLED` block (`nstrace_num`, `nstrace_depth`,
  `nstrace_enabled_global`, `nstrace_leave`, `nstrace_restore_global_trace_state`,
  `nstrace_fullscreen_type_name`): `NSTRACE_ENABLED` is **0** (src/nsterm.h 142-143, and the
  `#define … 1` at nsterm.h 65 is commented out), so the reference does not compile it either.
  The three functions keep their ticked boxes with that reason.
* C 65-68's GNUstep includes (`process.h`, `<GNUstepGUI/GSDisplayServer.h>`) and C 70-81's
  `NS_IMPL_COCOA` includes: Swift's `import Foundation`/`import AppKit` replace the whole
  prologue, which is recorded here rather than transcribed.
* `ns_menu_bar_is_hidden` (C 313) keeps its `#ifdef NS_IMPL_COCOA` guard as an ordinary
  declaration: this lane's target *is* the Cocoa arm.

### The generated checklist's eight gaps, added by hand

`extract-abi.py` lists 39 file-scope objects for this file and misses eight that are there in C:
`ns_return_types` (a second declarator on C 300's line), `ns_app_name` (C 302),
`x_display_list` (C 305), `select_valid` (a second declarator on C 324's line),
`ns_pending_files`/`ns_pending_service_names` (C 332) and `ns_pending_service_args` (C 333)
(three declarators on two lines), and `hold_event_q` (C 342-347, an anonymous `struct`).  They are
boxes of their own in the section above so the checklist is complete; **a regeneration would drop
them**, the way `docs/ports/data.md` records its 120-vs-116 box difference.

### Deviations

* `hold_event_q`'s anonymous struct (`struct { struct input_event *q; int nr, cap; }`) has no
  Swift spelling, so its three members are three file-scope globals of the C names
  (`hold_event_q_q`/`_nr`/`_cap`).
* The 26 C statics of this slice land as `fileprivate` (the lane's reading of C's `static`;
  `docs/ports/nsselect.md` does the same), so no exported Swift symbol appears for
  `mod_of_kind`/`ev_modifiers_helper`.
* C's includes and `#import`s (C 33-81) are not transcribed; `import Foundation`/`import AppKit`
  are the Swift spelling of the platform half.

### The 14 `check-cond-eval.py` rows, and the two readings

`python3 tools/check-cond-eval.py src/nsterm.m` reports `14 disagreements [NOT VERIFIED]`, and
**the tool's clang side is broken for this file**: its own probe's marker never appears in clang's
`-dM` dump, not even for an appended `#if 1` or a bare `#define` (Evidence), while the same text
is live in `-E -P` output.  Read against the frozen `config.h` instead, the evaluator's 14 answers
are the *frozen configuration's* own: `NS_IMPL_COCOA` and `NS_IMPL_GNUSTEP` are undefined there
(`/* #undef … */`), so every `!defined (NS_IMPL_COCOA)` / `#ifndef NS_IMPL_COCOA` arm is **live**
in the frozen build and every `#ifdef NS_IMPL_COCOA` arm dead.

This lane ports the **Cocoa build** (the banner), where that flips, so the ledger's arm choices
are the column marked "Cocoa".  **None of the 14 is inside this round's C 60-390** - the slice's
only conditions are C 65/70/83 (`NS_IMPL_GNUSTEP` dead, `NS_IMPL_COCOA` live, which is why the
sRGB arm at C 162/175 is compiled here and `ns_menu_bar_is_hidden` at C 313 is an ordinary
declaration) and C 96 (`NSTRACE_ENABLED`, dead).

| C line | condition | frozen config | Cocoa build (this ledger) |
| --- | --- | --- | --- |
| 1469 | `!defined (NS_IMPL_COCOA) \|\| MAC_OS_X_VERSION_MIN_REQUIRED < 101400` | live | **dead** (Cocoa defines it; min-required is >= 101400) |
| 4167 | `#ifndef NS_IMPL_COCOA` | live | **dead** |
| 4677 | `#ifdef GCC_LINT` | live | live (`GCC_LINT` is 1 in the frozen `config.h`) |
| 5374 | `#ifdef HAVE_PTHREAD` | live | live (`HAVE_PTHREAD` is 1) |
| 5811 | `#ifndef NS_IMPL_GNUSTEP` | live | live (GNUstep is undefined under Cocoa too) |
| 7102 | `#ifndef NS_IMPL_COCOA` | live | **dead** |
| 7352 | `#ifndef NS_IMPL_GNUSTEP` | live | live |
| 7675 | `#if MAC_OS_X_VERSION_MAX_ALLOWED >= 1070` | live | live (SDK 26) |
| 8571 | `#if MAC_OS_X_VERSION_MAX_ALLOWED >= 1070` | live | live |
| 8630 | `!defined (NS_IMPL_COCOA) \|\| MAC_OS_X_VERSION_MIN_REQUIRED <= 1090` | live | **dead** |
| 8632 | `#if MAC_OS_X_VERSION_MAX_ALLOWED > 1090` | live | **dead** (nested in 8630) |
| 8967 | `#if MAC_OS_X_VERSION_MAX_ALLOWED >= 1070` | live | live |
| 9812 | `!defined (NS_IMPL_COCOA) \|\| MAC_OS_X_VERSION_MIN_REQUIRED <= 1090` | live | **dead** |
| 9813 | `#if MAC_OS_X_VERSION_MAX_ALLOWED > 1090` | live | **dead** (nested in 9812) |

## ABI

### (1) C symbols this file defines and exports (`@_cdecl`)

| C name | C declaration | Swift |
| --- | --- | --- |
| `ns_send_types` | `extern NSArray *ns_send_types` (src/nsterm.h 774) | `func ns_send_types() -> NSArray?` |
| `ns_return_types` | `extern NSArray *ns_return_types` (src/nsterm.h 774) | `func ns_return_types() -> NSArray?` |
| `ns_app_name` | `extern NSString *ns_app_name` (src/nsterm.h 775) | `func ns_app_name() -> NSString?` |
| `x_display_list` | `extern struct ns_display_info *x_display_list` (931) | `func x_display_list_get() -> UnsafeMutableRawPointer?` |
| `context_menu_value` | `extern long context_menu_value` (933) | `func context_menu_value() -> Int` |
| `ns_init_events` | `extern void ns_init_events (struct input_event *)` (src/nsterm.h 1257) | `func ns_init_events(_ ev: UnsafeMutableRawPointer)` |
| `ns_finish_events` | `extern void ns_finish_events (void)` (src/nsterm.h 1258) | `func ns_finish_events()` |

`mod_of_kind`, `ev_modifiers_helper`, `hold_event` and `append2` are C `static`s - no C symbol exists in the reference
either - so they land as `fileprivate func`s of their own names.  The exported set is exactly
what `nm -g` shows (Evidence).

### (2) C symbols this file calls (`@_silgen_name`)

| C name | prototype | declared | defined |
| --- | --- | --- | --- |
| `SYMBOLP` | `INLINE bool SYMBOLP (Lisp_Object)` | src/lisp.h 1075 | header-only |
| `EQ` | `INLINE bool EQ (Lisp_Object, Lisp_Object)` | src/lisp.h (the `lisp_h_EQ` family) | header-only |
| `plist_get` | `Lisp_Object plist_get (Lisp_Object, Lisp_Object)` | src/lisp.h 4280 | src/fns.c |
| `parse_solitary_modifier` | `int parse_solitary_modifier (Lisp_Object)` | src/keyboard.h 463 | src/keyboard.c 7920 |
| `Qnil` | generated global (`globals.h`) | - | core |
| `Qleft` | generated global (`globals.h`) | - | core |
| `ns_use_srgb_colorspace` | `bool ns_use_srgb_colorspace` (a `DEFVAR_BOOL` variable) | src/nsterm.m 11716 | **open** - see Decisions |
| `QCmouse` | `Lisp_Object` symbol (a `DEFSYM`) | src/nsterm.m 11795 | **open**: this file's own `DEFSYM`, not in `docs/baseline/globals/symbols.tsv` |
| `up_modifier`, `down_modifier`, `hyper_modifier`, `shift_modifier` | `int` enumerators | src/termhooks.h 434-459 | **open**: the mirror's `pub const`s carry no C symbol yet |
| `ns_alternate_modifier`, `ns_right_alternate_modifier`, `ns_command_modifier`, `ns_right_command_modifier`, `ns_control_modifier`, `ns_right_control_modifier`, `ns_function_modifier` | `Lisp_Object` `DEFVAR_LISP` variables | src/nsterm.m 11608-11670 | **open**: imported as data symbols (Decisions) |
| `xrealloc` | `void *xrealloc (void *, size_t)` | src/lisp.h 5541 | src/alloc.c |
| `nconc2` | `Lisp_Object nconc2 (Lisp_Object, Lisp_Object)` | src/lisp.h 4271 | src/fns.c |
| `list1` | `Lisp_Object list1 (Lisp_Object)` | src/lisp.h (the `list` family) | src/alloc.c |
| `EVENT_INIT` | `INLINE void EVENT_INIT (struct input_event *)` | src/termhooks.h 414 | header-only |

`Qnil`/`Qleft`, the two `INLINE` predicates and `EVENT_INIT` are header-only constructs, so the
core exports a C-ABI entry under the construct's own name (`docs/ports/nsselect.md` (2)b).  The
two `var` imports (`ns_use_srgb_colorspace`, the seven `ns_*_modifier`s) are data symbols, which
is why `tools/check-swift-units.py` counts only the `func` imports; `nm -u` on the built object
lists the whole set (Evidence).

### (3) What other units must supply

* the core: (2)'s ten call entries - `SYMBOLP`, `EQ`, `plist_get`,
  `parse_solitary_modifier`, `Qnil`, `Qleft`, `xrealloc`, `nconc2`, `list1`, `EVENT_INIT` - and
  its **thirteen** data symbols `ns_use_srgb_colorspace`, `QCmouse`, `up_modifier`, `down_modifier`,
  `hyper_modifier`, `shift_modifier` and the seven `ns_*_modifier` variables.  `QCmouse` and the
  seven modifiers are this file's own `DEFSYM`/`DEFVAR_LISP`, so the core has to export them the
  way it exports the rest; `QCmouse` is also absent from `docs/baseline/globals/symbols.tsv`,
  whose table comes from a build that never compiles this file;
* `femacs/platform/macos/nsmenu.swift` (another lane's unit): the `EmacsMenu` class whose
  pointer this unit carries untyped;
* other lanes read this unit's exports: `nsselect.swift` calls `ns_send_types()` and
  `ns_return_types()`.

## Evidence

* `swiftc --version` -> `swift-driver version: 1.168.6 Apple Swift version 6.4
  (swiftlang-6.4.0.34.1 clang-2100.3.34.1)`, `Target: arm64-apple-macosx26.0`.
* `cd /Users/liu/projects/src/github.com/liuchong/femacs && swiftc -parse-as-library -c -o
  /tmp/s188/nsterm.o femacs/platform/macos/nsterm.swift` -> **exit 0, no output** (no warnings).
  The same command on the revision this round received printed twelve errors - the list is in
  `## Decisions`; `swiftc -parse` alone prints nothing for either revision, which is why the
  ledger's gate is the `-c` form.
* `nm -g /tmp/s188/nsterm.o | grep ' T _'` -> the five exported C names `_ns_send_types`,
  `_ns_return_types`, `_ns_app_name`, `_x_display_list`, `_context_menu_value` (plus
  Swift-mangled symbols; no C symbol for the two `static`s).
* `nm -u /tmp/s188/nsterm.o` -> the unit's own imports, `_EQ`, `_Qleft`, `_Qnil`, `_SYMBOLP`,
  `_ns_use_srgb_colorspace`, `_parse_solitary_modifier`, `_plist_get`, alongside the Swift
  runtime and AppKit entries.
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 142 C
  definition(s), 140 unported, 0 row(s); 6 C symbol import(s)` and the tool's last line
  `140 new, 0 stale, 6 Swift unit(s) landed`.  The 140 `new` are this file's un-ported definitions
  (this is a giant unit in flight, so they are expected here the way `nsmenu.swift`'s three
  `new` rows are Boyle's); the two landed functions are the difference between 142 and 140, and
  the three dead `nstrace_*` will need `docs/baseline/swift-units.tsv` rows when the ledger is
  closed - **the manifest is the integrator's table, so this unit asks rather than writes**.
* `python3 tools/check-cond-eval.py src/nsterm.m` -> `src/nsterm.m: 182 conditions, 0 unknown,
  14 disagreements [NOT VERIFIED]` and `182 conditions, 0 unknown to the evaluator, 14
  disagreements`.  **The clang side of the probe is broken for this file**: the tool's own probe
  (`/tmp/femacs-cond-eval/probe_nsterm_1468.c`) keeps its marker undefined, and even
  `#if 1` / a bare `#define` appended after the file's 1468-line prefix fails to appear in
  clang's `-dM` dump (9598 macros), while the same text is *live* in `-E -P` output; a control
  run on `#include <config.h>` plus the same `-I` flags does show the marker.  The evaluator's
  answers for the 14 agree with the frozen `config.h` read directly (`#define GCC_LINT 1`,
  `#define HAVE_PTHREAD 1`, `/* #undef NS_IMPL_COCOA */`, `/* #undef NS_IMPL_GNUSTEP */`,
  `MAC_OS_X_VERSION_MAX_ALLOWED` = `__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__` = 260000), so
  the 14 rows are the tool's artifact, not 14 wrong branches; the file stays out of
  `docs/baseline/config/cond-eval-verified.txt` either way, which is why `extract-abi.py` lists
  the dead `nstrace_*` definitions too.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures` (no site from this unit).
* Round 2's compile, same command: **exit 0, no output**.  Two measurements came out of it:
  `@_cdecl` refuses a pointer to a Swift record - `error: global function cannot be marked
  '@_cdecl' because the type of the parameter cannot be represented in Objective-C` on
  `ns_init_events(_ ev: UnsafeMutablePointer<input_event>)`, hence the untyped parameter - and a
  bodyless `@_silgen_name("foo") func foo` followed by `func foo` in the same file is
  `error: invalid redeclaration of 'foo'`, which is why `EV_TRAILER`/`EV_TRAILER2` are held.
* `nm -g /tmp/s188r2/nsterm.o | grep ' T _'` -> the exports now include `_ns_init_events` and
  `_ns_finish_events` beside round 1's five, plus `___swift_memcpy64_8` - the helper Swift emits
  for `hold_event`'s 64-byte value copy, which is the record's measured size.
* `nm -u /tmp/s188r2/nsterm.o` -> the own imports of (2): `_EQ`, `_EVENT_INIT`, `_QCmouse`,
  `_Qleft`, `_Qnil`, `_SYMBOLP`, `_down_modifier`, `_hyper_modifier`, `_list1`,
  `_nconc2`, `_ns_alternate_modifier`, `_ns_command_modifier`, `_ns_control_modifier`,
  `_ns_function_modifier`, `_ns_right_alternate_modifier`, `_ns_right_command_modifier`,
  `_ns_right_control_modifier`, `_ns_use_srgb_colorspace`, `_parse_solitary_modifier`,
  `_plist_get`, `_shift_modifier`, `_up_modifier`, `_xrealloc` (with the Swift runtime and
  AppKit entries; the `var` imports are the non-lazy ones).
* `struct input_event`'s layout, C side (`clang -std=gnu23 -I build/emacs-ref/src -I
  build/emacs-ref/lib -I emacs/src -I emacs/lib` on a probe including `<config.h>`, `lisp.h`,
  `systime.h` and `termhooks.h`) -> `sizeof 64 align 8`, `code 4 modifiers 8 x 16 y 24
  timestamp 32 frame_or_window 40 arg 48 device 56` (the two bitfields share the unit at 0,
  width 16); the same probe prints the modifier bits `up 1 down 2 drag 4 click 8 double 16
  triple 32 alt 4194304 super 8388608 hyper 16777216 shift 33554432 ctrl 67108864
  meta 134217728`.
* `struct input_event`'s layout, Swift side (`swiftc -o /tmp/s188r2/layoutprobe
  femacs/platform/macos/nsterm.swift /tmp/s188r2/main.swift /tmp/s188r2/stubs.c -Xlinker
  -undefined -Xlinker dynamic_lookup`, then the program) -> `input_event size 64 stride 64
  align 8`, `bits 0`, `code 4`, `modifiers 8`, `x 16`, `y 24`, `timestamp 32`,
  `frame_or_window 40`, `arg 48`, `device 56` - member for member the same as the C side.
  (The probe links a two-line C file that only *defines* the data symbols the core still owes,
  because dyld resolves data imports at launch; `nsselect.md`'s probe ran without it only
  because every import it had was a lazily bound function.)
* `python3 tools/extract-abi.py emacs/src/nsterm.m` -> the checklist above (0 subrs, 142
  functions, 23 macros, 1 type, 39 file-scope objects); the Objective-C section is hand-written
  from the same reference.
* `clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I
  emacs/lib /tmp/s188/cfgprobe.c` (a file that is just `#include <config.h>`) -> the
  `NS_IMPL_*`/`GCC_LINT`/`HAVE_PTHREAD`/`MAC_OS_X_VERSION_*` lines quoted above.
* `clang -o /tmp/s188/crt /tmp/s188/crt.c -framework Foundation && /tmp/s188/crt` -> the two
  autorelease-pool runtime symbols exist (`0x18a833dc8 0x18a83403c`), which is the measurement
  behind the `outerpool` decision.
