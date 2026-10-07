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

## Functions defined — 150
- [x] nstrace_leave  (line 113)  — dead: `NSTRACE_ENABLED` is 0 (src/nsterm.h 142-143), so the reference does not compile it either - recorded, not transcribed
- [x] nstrace_restore_global_trace_state  (line 122)  — dead, like the box above
- [x] nstrace_fullscreen_type_name  (line 129)  — dead, like the box above
- [x] mod_of_kind  (line 360, static)  — C 358-367, landed as `fileprivate func mod_of_kind`
- [x] ev_modifiers_helper  (line 372, static)  — C 369-389, landed as `fileprivate func ev_modifiers_helper`
- [x] ns_init_events  (line 485) — landed, `@_cdecl("ns_init_events")` (the unit's C interface, src/nsterm.h 1257)
- [x] ns_finish_events  (line 492) — landed, `@_cdecl("ns_finish_events")` (src/nsterm.h 1258)
- [x] hold_event  (line 498, static) — landed as `fileprivate func hold_event` (the hold queue's `input_event` copy is `MemoryLayout<input_event>.stride`, measured 64)
- [x] append2  (line 515, static) — landed as `fileprivate func append2`; C's `list (item)` is `list1` for one argument
- [x] ns_relocate  (line 525) — landed as `@_cdecl("ns_relocate")` (src/nsterm.h 1195; called src/emacs.c 1059/3298); the `NS_SELF_CONTAINED` arm is the live one (`configure.ac` 689-690 default `yes`, 2885-2886 `AC_DEFINE`)
- [x] ns_init_pool  (line 560) — landed as `@_cdecl("ns_init_pool")`; C's `[[NSAutoreleasePool alloc] init]` is the runtime's `objc_autoreleasePoolPush` (ARC has no `NSAutoreleasePool`)
- [x] ns_init_locale  (line 569) — landed as `@_cdecl("ns_init_locale")` (src/nsterm.h 1172; src/emacs.c 1647); `NSTRACE (…)` is the empty macro at `NSTRACE_ENABLED` 0, so it carries no statement
- [x] ns_release_object  (line 602) — landed as `@_cdecl("ns_release_object")`; `[(id)obj release]` is `Unmanaged.release` under ARC, and `nsselect.swift` 215 already imports that signature
- [x] ns_retain_object  (line 612) — landed as `@_cdecl("ns_retain_object")` (src/nsterm.h 1167; src/image.c 4418); `[(id)obj retain]` is `Unmanaged.retain`, its +1 belonging to the caller
- [x] ns_alloc_autorelease_pool  (line 622) — landed as `@_cdecl("ns_alloc_autorelease_pool")` (src/nsterm.h 1168; src/thread.c 786); the runtime's pool push
- [x] ns_release_autorelease_pool  (line 632) — landed as `@_cdecl("ns_release_autorelease_pool")` (src/thread.c 842); C's `ns_release_object (pool)` is a pool pop
- [x] ns_menu_bar_should_be_hidden  (line 642, static) — landed as `fileprivate func ns_menu_bar_should_be_hidden () -> Bool`; `ns_auto_hide_menu_bar` (C 11697) is imported as the data symbol the core exports
- [x] ns_screen_margins  (line 660, static) — landed; `NSTRACE`/`NSTRACE_MSG` are the empty macros, and C's nil receiver for a screen reads as `?? NSRect.zero`
- [x] ns_screen_margins_ignoring_hidden_dock  (line 711, static) — landed; `DOCK_IGNORE_LIMIT` (C 708) is a file-scope `let`
- [x] ns_menu_bar_height  (line 754, static) — landed
- [x] ns_parent_window_rect  (line 773, static) — landed; the live `convertRectToScreen:` arm (SDK 26), the two `convertBaseToScreen:` fallbacks recorded dead
- [x] ns_row_rect  (line 808, static) — landed; `window_box` + `WINDOW_TO_FRAME_PIXEL_Y` are entries, and `row->y`/`row->visible_height` are member entries per nsmenu.swift's rule
- [x] ns_frame_scale_factor  (line 828) — landed as `@_cdecl("ns_frame_scale_factor")` (src/nsterm.h 1260; called src/nsfns.m 3598); the `backingScaleFactor` arm is live, the GNUstep/`respondsToSelector:` arms recorded dead
- [x] constrain_frame_rect  (line 892, static) — landed; C's `#if 0` restriction (C 962-985) recorded, not transcribed
- [x] ns_constrain_all_frames  (line 1001, static) — landed; `FOR_EACH_FRAME` is spelled out per `docs/ports/nsfns.swift` 967, and `[view isFullscreen]` is the first **deferred entry** (`## ABI` (4))
- [x] ns_update_auto_hide_menu_bar  (line 1033, static) — landed; the whole body is the `NS_IMPL_COCOA` arm, and `NSApplicationPresentationDefault` is Swift's empty option set (`[]`, since `.default` is unavailable)
- [x] ns_update_begin  (line 1075, static) — landed; `swiftc -c` reports C's own `lockFocus` deprecation (Evidence), and the port keeps the call
- [x] ns_UAZoomChangeFocus  (line 1162, static) — landed; the receiver is `NSView?` because only `NSView`/`NSWindow` messages are sent to it, and `UAZoomChangeFocus`'s third argument needs `UAZoomChangeFocusType(…)` (the SDK's enum imports as `Int`)
- [x] ns_focus  (line 1241, static) — landed; **the comment-stop gap of status 3.260** (this box's
      old number, 1236, was the hand count of a generator-missed function and pointed at the
      previous function's last statement): `NSRectClip`/`NSRectClipList` are AppKit's own C functions,
      reached by name because Swift marks them unavailable, and C's NULL-able `NSRect *r` is the
      optional `UnsafePointer<NSRect>?`
- [x] ns_update_end  (line 1212, static) — landed; the dead `flushWindow` arm (GNUstep/pre-10.14) recorded, and `MOUSE_HL_INFO (f)->mouse_face_defer = 0` is the first member-entry **write** (`mouse_highlight_set_mouse_face_defer`)
- [x] ns_unfocus  (line 1282, static) — landed; `NSGraphicsContext` restore + C's `gsaved` flag
- [x] ns_ring_bell  (line 1436, static) — landed; `visible_bell` and `SELECTED_FRAME()` are entries, `NSBeep ()` is Swift's `NSSound.beep ()`, and `[[EmacsBell alloc] init]` is `EmacsBell (frame: .zero)`
- [x] hide_bell  (line 1471, static) — **dead**: the guard at C 1471 is `#if !defined (NS_IMPL_COCOA) || MAC_OS_X_VERSION_MIN_REQUIRED < 101400`, and this lane's target both defines `NS_IMPL_COCOA` and has a minimum of 260000; its one call site (C 9388) is inside the same guard. Recorded, not transcribed
- [x] ns_get_focus_frame  (line 1493, static) — landed as `fileprivate func` (C's `static Lisp_Object`); the display-info member is the entry `ns_display_info_ns_focus_frame`
- [x] ns_focus_frame  (line 1510, static) — landed as `fileprivate func`; `[NSApp activateIgnoringOtherApps:YES]` is Swift's `NSApp.activate(ignoringOtherApps: true)`
- [x] ns_raise_frame  (line 1528, static) — landed; `f->no_accept_focus` is the member entry `frame_no_accept_focus`
- [x] ns_lower_frame  (line 1550, static) — landed
- [x] ns_frame_raise_lower  (line 1566, static) — landed; `NSTRACE` is empty
- [x] ns_frame_rehighlight  (line 1582, static) — landed; `ns_set_frame_alpha` is a **deferred entry** (this unit's own function, defined at C 2366; C forward-declares it at C 1622)
- [x] ns_make_frame_visible  (line 1624) — landed as `@_cdecl("ns_make_frame_visible")` (src/nsterm.h 1226); four deferred entries (`fsIsNative`, `isFullscreen`, `handleFS`, `setParentChildRelationships`) and `ns_set_offset` (defined at C 1854)
- [x] ns_make_frame_invisible  (line 1678) — landed as `@_cdecl("ns_make_frame_invisible")` (src/nsterm.h 1227)
- [x] ns_make_frame_visible_invisible  (line 1693, static) — landed
- [x] ns_iconify_frame  (line 1705) — landed as `@_cdecl("ns_iconify_frame")` (src/nsterm.h 1228)
- [x] ns_free_frame_resources  (line 1744) — landed as `@_cdecl("ns_free_frame_resources")` (src/nsterm.h 1187); C's `[x release]` calls are the runtime decrement (`Unmanaged.passUnretained (x).release ()`, round 3's reading), and the three display-info/mouse-highlight member writes are entries
- [x] ns_destroy_window  (line 1784, static) — landed as `fileprivate func` (C's `static void`); `emacs_abort ()` is the `Never`-returning entry
- [x] compute_offset  (line 1816, static) — landed as `fileprivate func`; `f->size_hint_flags` is the member entry `frame_size_hint_flags` with `XNegative`/`YNegative` as enumerator entries, and the GNUstep clamp is dead
- [x] ns_set_offset  (line 1854) — landed as **`@_cdecl("ns_set_offset")`** - the definition the `ns_set_offset_entry` row of (4) waited for (first called at C 1691); its signature matches `nsfns.swift` 600, and `nm -u` no longer lists `_ns_set_offset` (Evidence)
- [x] ns_set_window_size  (line 1877, static) — landed as `fileprivate func` (C's `static void`); the `NSTRACE_MSG` arguments are not evaluated (empty macro), and `f->output_data.ns->zooming` is the member pair `ns_output_zooming`/`ns_output_set_zooming`
- [x] ns_set_window_size_and_position  (line 1920, static) — landed; `FRAME_NS_TITLEBAR_HEIGHT`/`FRAME_TOOLBAR_HEIGHT` are entries, and `compute_offset`/`frame_left_pos`/`frame_top_pos` were already in place
- [x] ns_set_undecorated  (line 1961) — **the status 3.260 generator gap**, added by hand in round
      10 (the reader stopped at the doc comment before C 1970's `{`; the round-10 Decisions call to
      it an 8-line lookahead, corrected below); landed
      as `@_cdecl("ns_set_undecorated")` (src/nsterm.h 1229), and `[[EmacsWindow alloc]
      initWithEmacsFrame:]` is the deferred entry `EmacsWindow_initWithEmacsFrame` (`## ABI` (4))
- [x] ns_set_parent_frame  (line 1999) — **the same 3.260 gap** (a 21-line comment before the
      body); landed as `@_cdecl("ns_set_parent_frame")` (src/nsterm.h 1230), and C's
      `p = XFRAME (new_value)` inside the condition is spelled as three steps (Decisions)
- [x] ns_set_no_focus_on_map  (line 2043) — landed as `@_cdecl("ns_set_no_focus_on_map")`
      (src/nsterm.h 1231); the write half of `FRAME_NO_FOCUS_ON_MAP` is the member entry
      `frame_set_no_focus_on_map`
- [x] ns_set_no_accept_focus  (line 2061) — **the same 3.260 gap** (a comment before the body);
      landed as `@_cdecl("ns_set_no_accept_focus")` (src/nsterm.h 1232)
- [x] ns_set_z_group  (line 2078) — **the same 3.260 gap** (a comment before the body); landed as `@_cdecl("ns_set_z_group")` (src/nsterm.h 1233); `NSNormalWindowLevel` is Swift's
      `NSWindow.Level.normal`, and the four `z_group_*` are enumerator entries
- [x] ns_set_appearance_1  (line 2120) — landed as `@_cdecl("ns_set_appearance_1")` (src/nsterm.h
      1240; imported by `nsfns.swift` 604); the three `ns_appearance_*` are enumerator entries
- [x] ns_set_appearance  (line 2131) — landed as `@_cdecl("ns_set_appearance")` (src/nsterm.h
      1241); `NSAppKitVersionNumber10_10` is spelled as Swift's `NSAppKitVersion.macOS10_10`, and
      `[window setAppearance]` is the deferred entry `setAppearance`
- [x] ns_set_transparent_titlebar  (line 2149) — landed as
      `@_cdecl("ns_set_transparent_titlebar")` (src/nsterm.h 1242-1245); the `respondsToSelector:`
      test is `#selector(getter: NSWindow.titlebarAppearsTransparent)`
- [x] ns_fullscreen_hook  (line 2169, static) — landed as `fileprivate func` (C's `static`; the
      reference registers it as `terminal->fullscreen_hook`, C 5936), and `[view handleFS]` is
      round 7's entry
- [x] ns_get_color  (line 2191, static) — landed as `fileprivate func`; C reassigns its `name`
      parameter twice, which the port reads off `nsname` (Decisions), and `parse_color_spec` is the
      core's own export (`femacs/src/xfaces.rs` 760)
- [x] ns_lisp_to_color  (line 2301) — landed as `@_cdecl("ns_lisp_to_color")` (src/nsterm.h 1148 -
      `nsfns.swift` 118-119 imports this signature)
- [x] ns_query_color  (line 2315, static) — landed as `fileprivate func`; C's `void *col` is typed
      as `NSColor?` (its two call sites both pass one), and `-unsignedLong` is the unit's own
      category method
- [x] ns_defined_color  (line 2332) — landed as `@_cdecl("ns_defined_color")` (src/nsterm.h 1142);
      `Emacs_Color *` crosses as `UnsafeMutableRawPointer` because `@_cdecl` refuses a pointer to a
      Swift record (measured)
- [x] ns_query_frame_background_color  (line 2357, static) — landed as `fileprivate func`;
      `FRAME_BACKGROUND_COLOR` is the entry `macfont.swift` 756 already imports
- [x] ns_set_frame_alpha  (line 2366, static) — landed as **`@_cdecl("ns_set_frame_alpha")`** - the
      definition the `ns_set_frame_alpha_entry` row of (4) waited for (C forward-declares it at
      C 1579 and calls it first at C 1612/1617), and `nm -u` no longer lists `_ns_set_frame_alpha`
      (Evidence); `f->alpha[0]`/`[1]` is the member entry `frame_alpha`
- [x] frame_set_mouse_pixel_position  (line 2409) — landed as `@_cdecl("frame_set_mouse_pixel_position")` (src/frame.h 1882, the redisplay interface's `set_mouse_pixel_position_hook`); the two `FRAME_NS_*_HEIGHT` macros are round 9's expanded helpers and `CGWarpMouseCursorPosition` is CoreGraphics' own call
- [x] ns_note_mouse_movement  (line 2433, static) — landed as `fileprivate func` (C's `static int`); `r = &dpyinfo->last_mouse_glyph` is the *pointer* member entry `ns_display_info_last_mouse_glyph`, so every `r->field` access stays in Swift, and the two `CGFloat` arguments land in `int` parameters (C's implicit truncation)
- [x] ns_mouse_position  (line 2476, static) — landed as `fileprivate func` (C's `static void`, the `mouse_position_hook`); `FOR_EACH_FRAME` is spelled out, `track_mouse` is a symbol-less generated global, and the `EmacsView`/`EmacsTooltip` kind tests are the receiver-first entries `is_emacs_view`/`is_emacs_tooltip`
- [x] ns_frame_up_to_date  (line 2599, static) — landed as `fileprivate func` (C's `static void`, the `frame_up_to_date_hook`); the three `hlinfo->…` reads are `Mouse_HLInfo` member entries and `note_mouse_highlight` is a real symbol of the core (`T` in `build/emacs-ref/src/xdisp.o`)
- [x] ns_define_frame_cursor  (line 2625, static) — landed as `fileprivate func` (C's `static void`, the `define_frame_cursor_hook`); `FRAME_POINTER_TYPE` is the `struct ns_output` member `current_pointer`, read through the shim family's `ns_output_current_pointer` (its new read half) and written through its existing `ns_output_set_current_pointer` - the very C line that entry cites
- [x] ns_convert_key  (line 2649, static) — landed as `fileprivate func`; `countof (convert_ns_to_X_keysym)` is the round-1 table's `count`, and C's `for (keysym = 0; …; keysym += 2)` keeps its shape as the same loop
- [x] get_keysym_name  (line 2667) — landed as **`@_cdecl("get_keysym_name")`** (declared src/lisp.h 5436) - **this unit is the definition** the other side's dead arm (`keyboard.rs` 8553, under `feature = "have_window_system"`) names; C's `static char value[16]` is a file-scope buffer and `snprintf` is the `String(format:)` substitution the lane already records
- [x] right_mod  (line 2681, static) — landed as `fileprivate func` (the `NS_IMPL_COCOA` arm at C 2679); `EQ`/`Qleft` are the existing entries
- [x] nil_or_none  (line 2687, static) — landed as `fileprivate func`; `NILP` is the existing entry and `Qnone` is a new one (`DEFSYM (Qnone, "none")`, src/frame.c 7241; `_Qnone` is `S` in `build/emacs-ref/src/emacs.o`)
- [x] ns_get_shifted_character  (line 2693, static) — landed as `fileprivate func` (C's
      `static UniChar`); it is the name status 3.260's reader fix surfaced (its manifest row landed
      in `aa3e13a`), the Carbon dependency is C 72-73's `#include <Carbon/Carbon.h>` - the file now
      carries `import Carbon` - `dead_key_state` is the function-scope static at C 2703, and
      `QCordinary` is this file's own `DEFSYM` (C 11793)
- [x] ns_redraw_scroll_bars  (line 2773, static) — **dead in this build**: C opens `#ifdef NS_IMPL_GNUSTEP` at C 2771 and closes it at C 2786, so the whole definition is the arm this target does not take.  Recorded, not transcribed (the `ns_update_window_end` / `-updateMonitors:` reading), in round 49's block with its body described
- [x] ns_clear_frame  (line 2790) — landed as **`@_cdecl("ns_clear_frame")`** (src/nsterm.h 1124, the `clear_frame_hook`); the `NS_IMPL_GNUSTEP` arm at C 2816-2818 is dead (`ns_redraw_scroll_bars` is GNUstep-only, C 2771-2786)
- [x] ns_clear_frame_area  (line 2824, static) — landed as `fileprivate func` (C's `static void`); `NSIntersectionRect` is AppKit's own Swift-visible function while `NSRectFill` is reached by name (the `NSRectClip` reading)
- [x] ns_scroll_run  (line 2850, static) — landed as `fileprivate func`; `w->frame` is the `window_frame` member entry, the three `run->…` reads are the new `struct run` member entries, and `[view copyRect:to:]` is the deferred entry `copyRect` (this unit's own `EmacsView` method, C 9340) - the SDK-101400 arm at C 2910-2912 is dead here
- [x] ns_clear_under_internal_border  (line 2920, static) — landed as `fileprivate func`; the face-id expression is the three-way `FRAME_PARENT_FRAME`/`Vface_remapping_alist` conditional of C 2931-2938 over `lookup_basic_face` (which returns the **id**, not a face) and the three face-id enumerators
- [x] ns_after_update_window_line  (line 2964, static) — landed as `fileprivate func` (C's `static void`); `eassert` is the no-op of an unset `ENABLE_CHECKING` and C's two comma-expression conditions become nested `if`s (same short-circuit), with `max (0, …)` expanded as `Swift.max` (class B: arithmetic)
- [x] ns_shift_glyphs_for_insert  (line 3023, static) — landed as `fileprivate func`; the copy is round 13's `copyRect` entry, so the function adds no new name at all
- [x] ns_compute_glyph_string_overhangs  (line 3048, static) — landed as `fileprivate func`; it **holds** a `struct font_metrics` (measured, Evidence), reaches C's `font->driver->text_extents (…)` as the two-hop pair `font_driver`/`font_driver_text_extents`, and starts the `glyph_string_*` family with the overhang **writes** only (the reads belong to their own slice)
- [x] ns_define_fringe_bitmap  (line 3092, static) — landed as `fileprivate func` (C's `static void`); C's `alloca` is `withUnsafeTemporaryAllocation` (the reading `macfont.swift` 1684/2978 gives) and the `points`/`cur` pointer walk keeps its shape; the trace state machine's `char` letters are `CChar`s
- [x] ns_destroy_fringe_bitmap  (line 3199, static) — landed as `fileprivate func`; one `removeObject(forKey:)` on the `fringe_bmp` table below
- [x] ns_draw_fringe_bitmap  (line 3206, static) — landed as `fileprivate func`; `struct draw_fringe_bitmap_params` is read member by member (13 entries), and C's `bmp = [bmp copy]` … `[bmp release]` pair is ARC's own copy (no release call - the round-3 reading), with `[bmp transformUsingAffineTransform:]` taking the transform `as AffineTransform`
- [x] ns_draw_window_cursor  (line 3295, static) — landed as `fileprivate func` (C's `static void`, the RIF cursor hook); `get_phys_cursor_glyph` is provided by `femacs/src/window.rs` 8078, while `get_phys_cursor_geometry`/`draw_phys_cursor_glyph` (xdisp.c's `HAVE_WINDOW_SYSTEM` arms) and `draw_fringe_bitmap` (fringe.c) are the round's three **gated/dead** owed entries - the 3.262 shape
- [x] ns_draw_vertical_window_border  (line 3430, static) — landed as `fileprivate func`; one `FACE_FROM_ID_OR_NULL` on `VERTICAL_BORDER_FACE_ID` and one `face_foreground` read (round 15's entry)
- [x] ns_draw_window_divider  (line 3453, static) — landed as `fileprivate func`; the three divider face ids and `FRAME_FOREGROUND_PIXEL` are new entries, `face_foreground`/`NSRectFill`/`NSFocus`-family are existing ones
- [x] ns_show_hourglass  (line 3512, static) — landed as `fileprivate func`; the RIF `show_hourglass_hook`, and C's whole body is its TODO comment (the port keeps the comment, so `f` is unused here exactly as it is in C)
- [x] ns_hide_hourglass  (line 3518, static) — landed as `fileprivate func`; the mirror hook, its body is C's TODO comment alone
- [x] ns_get_glyph_string_clip_rect  (line 3530, static) — landed as `fileprivate func`; `NativeRectangle` is `NSRect` in this build (src/nsgui.h 80, not the X11 arm's `XRectangle`), and its one call reaches `get_glyph_string_clip_rects` (src/xdisp.c 2495), **gated** inside that file's `HAVE_WINDOW_SYSTEM` block
- [x] ns_draw_underwave  (line 3553, static) — landed as `fileprivate func`; `NSBezierPath.strokeLine(from:to:)` is AppKit's own Swift rename of `+strokeLineFromPoint:toPoint:`, and the loop's `odd = !odd` is C's `int` toggle
- [x] ns_draw_dash  (line 3598, static) — landed as `fileprivate func`; C's `alloc`/`init`/`release` `NSBezierPath` is `NSBezierPath()` under ARC (the `[path release]` is dropped, round 3's reading)
- [x] ns_fill_underline  (line 3620, static) — landed as `fileprivate func`; the six `enum face_underline_type` enumerators are new class-A entries, and C's `FACE_NO_UNDERLINE`/`FACE_UNDERLINE_WAVE`/`default` trio becomes one Swift `default` (the two named cases fall through into it, so both stay entries)
- [x] ns_draw_text_decoration  (line 3655, static) — landed as `fileprivate func`; `WINDOW_BUFFER_LOCAL_VALUE` stays class A (its body needs `BUFFERP` + `buffer_local_value`), `font_for_underline_metrics` is **gated** (src/xdisp.c 32256, `HAVE_WINDOW_SYSTEM`), and C's `unsigned long` locals narrow back with `truncatingIfNeeded:`
- [x] ns_draw_box  (line 3810, static) — landed as `fileprivate func`; C's two `char` flags are
      `Bool` (the file's own `ns_draw_image_relief` spells the same flags `bool`, C 4183), `[col set]`
      reads through the optional receiver, and `NSRect s = r` is Swift's by-value `var s = r`
- [x] ns_setup_relief_colors  (line 3844, static) — landed as `fileprivate func`; `[NSColor grayColor]`
      is `NSColor.gray`, `highlightWithLevel:`/`shadowWithLevel:` are `highlight (withLevel :)`/
      `shadow (withLevel :)`, and the three `[old release]; field = [new retain];` pairs are the six
      `ns_output_*_relief_color` entries with the runtime's release half and the nil-receiver reading
      `color.flatMap { … }` (Decisions)
- [x] ns_draw_relief  (line 3872, static) — landed as `fileprivate func`; the reference's own
      duplicated `vthickness > 1 && left_p` block (C 3980-3986 and 3988-3994) is kept verbatim, and
      the `NSBezierPath` spellings are `NSBezierPath ()`/`close ()`/`fill ()`/`stroke ()`/
      `removeAllPoints ()`/`NSBezierPath.fill (_:)`
- [x] ns_dumpglyphs_box_or_relief  (line 4027, static) — landed as `fileprivate func`; the composite
      walk keeps C's pointer shape through the new `glyph_advance` entry (the member family's first
      *operation* row), and `WINDOW_RIGHT_EDGE_X`/`window_box_right`/`glyph_row_glyphs`/
      `glyph_row_used`/`glyph_string_row`/`glyph_string_area` are its row reads
- [x] ns_maybe_dumpglyphs_background  (line 4092, static) — landed as `fileprivate func`; `char
      force_p` is `Bool`, the `NS_IMPL_COCOA` stipple arm is the live one (its `#else`'s
      `goto fill;` and the `fill:` label are recorded, not transcribed), and the arm's five
      CoreGraphics calls plus `EmacsImage -stippleMask` (`ns_display_info_bitmaps_img`) are entries
- [x] ns_draw_image_relief  (line 4180, static) — landed as `fileprivate func`; the four
      `s->slice` reads are member entries and `&s->slice` is the family's second *operation* row
      (`glyph_string_slice_ptr`), the `struct image` members and `image_ascent` are entries, and
      the two `DEFAULT_*_BUTTON_RELIEF` literals plus `eabs` are *spelled* (class B of 3.255)
- [x] ns_dumpglyphs_image  (line 4267, static) — landed as `fileprivate func` (C's `NSRect r`
      parameter is unused in the reference's own body and is kept); its four `EmacsImage` accesses
      are cross-lane entries, and `-appendTransform:` is the compiler's rename `append(_:)`
- [x] ns_draw_stretch_glyph_string  (line 4397, static) — C 4396-4545, landed as `fileprivate func ns_draw_stretch_glyph_string`
- [x] ns_draw_glyph_string_foreground  (line 4548, static) — C 4547-4564, landed as `fileprivate func ns_draw_glyph_string_foreground` (the driver's `draw` slot is a two-hop read)
- [x] ns_draw_composite_glyph_string_foreground  (line 4568, static) — C 4567-4655, landed as `fileprivate func ns_draw_composite_glyph_string_foreground`
- [x] ns_draw_glyphless_glyph_string_foreground  (line 4659, static) — C 4658-4741, landed as `fileprivate func ns_draw_glyphless_glyph_string_foreground` (the `char2b`/`buf` lifetimes are two `withUnsafeMutableBufferPointer` closures)
- [x] ns_set_glyph_string_gc  (line 4748, static) — C 4747-4780, landed as `fileprivate func ns_set_glyph_string_gc`
- [x] ns_draw_glyph_string  (line 4783, static) — C 4782-5001, landed as `fileprivate func ns_draw_glyph_string` (the RIF's main draw-text call)
- [x] ns_send_appdefined  (line 5013, static) — landed as `fileprivate func ns_send_appdefined (_ value: Int32)`: the non-main-thread arm reaches `nextappdefined` and `-sendFromMainThread:` through `NSApp as! EmacsApp` (round 21's class), `send_appdefined`/`timed_entry` are C 317/320's own entries, and `[NSApp context]` is kept as C writes it (that call carries a new deprecation warning, recorded in Evidence)
- [x] check_native_fs  (line 5088, static) — landed in round 25 as `fileprivate func check_native_fs ()`: the body's `FOR_EACH_FRAME` keeps the spelling C 1011's reading above gives it, and C 5103's `[view updateCollectionBehavior]` is the method that landed with this record - so the two callers C 5127/5218 were waiting for are both here now
- [x] ns_read_socket_1  (line 5111, static) — landed as `fileprivate func ns_read_socket_1 (_:_:_:) -> Int32`: the three callees it was waiting for are `check_native_fs` (round 25), `ns_send_appdefined` (round 22) and `EmacsApp`'s `-openFile:`/`-fulfillService:withArg:` (round 28), and C 5154/5155's `[outerpool release]` + `[[NSAutoreleasePool alloc] init]` pair is this unit's pop/push
- [x] ns_read_socket  (line 5196, static) — landed as `fileprivate func ns_read_socket (_:_:) -> Int32`: C's body is the `ns_read_socket_1` call with `NO`
- [x] ns_select_1  (line 5203, static) — landed as `fileprivate func ns_select_1 (_:_:_:_:_:_:_:)`: C 5235/5236's `FD_ISSET` is Darwin's `__darwin_fd_isset` (the macro's own expansion, imported in the `fputs` shape), `thread_select` is imported with the `select_func` `@convention(c)` alias its C typedef names, and C 5250-5265's commented-out outerpool drain is carried as that comment rather than restated
- [x] ns_select  (line 5366) — landed as `func ns_select (_:_:_:_:_:_:)`: C's body is the `ns_select_1` call with `NO`
- [x] ns_run_loop_break  (line 5376) — landed as `func ns_run_loop_break ()` (the `HAVE_PTHREAD` arm): C's two lines are the `NSApp` guard and the `ns_send_appdefined (-1)` it wraps
- [x] ns_set_vertical_scroll_bar  (line 5396, static) — C 5396-5486, landed as `fileprivate func ns_set_vertical_scroll_bar (_:_:_:_:)`: the `EmacsScroller` it builds and drives landed in rounds 42-46, C's `XNS_SCROLL_BAR` cast is `as? EmacsScroller`, `NSEqualRects (a, b)` is `a == b`, the possibly-nil view's reads are C's nil-receiver answers (`?? .zero` for the rect), and C 5466's `make_mint_ptr (bar)` is the object's own opaque pointer
- [x] ns_set_horizontal_scroll_bar  (line 5490, static) — C 5490-5571, landed as `fileprivate func ns_set_horizontal_scroll_bar (_:_:_:_:)`: the vertical hook's twin with the three differences C's own text shows - C 5525 takes the *x* pair out of `window_box`, C 5528-5529 read the horizontal bar's own height and y (the round's two new macro entries), and C 5548-5558's update branch sets `update_p = YES` without the vertical hook's C 5474 width copy; C 5561-5566's shared-corner clear keeps C's own `window->vertical_scroll_bar` test (the frame's read nothing else in this batch needs)
- [x] ns_condemn_scroll_bars  (line 5575, static) — C 5575-5593, landed as `fileprivate func ns_condemn_scroll_bars (_:)`: C's descending `for` is the `while`, the `isKindOfClass:` test is the port's own class test, and `condemn`'s `instancetype` result is discarded as C's statement discards it
- [x] ns_redeem_scroll_bar  (line 5597, static) — C 5597-5618, landed as `fileprivate func ns_redeem_scroll_bar (_:)`: C's `id bar` is `AnyObject?`, each `XNS_SCROLL_BAR` cast is `as? EmacsScroller`, and the two guards need this round's `WINDOW_HAS_VERTICAL_SCROLL_BAR`/`WINDOW_HAS_HORIZONTAL_SCROLL_BAR` macro entries
- [x] ns_judge_scroll_bars  (line 5622, static) — C 5622-5640, landed as `fileprivate func ns_judge_scroll_bars (_:)`: C's `continue` is written as such, which in a `while` needs the decrement on both sides of it (C's `for` does that itself), and `judge`'s `bool` result is discarded as C's statement discards it
- [x] ns_free_pixmap  (line 5649, static) — C 5648-5652, landed as `fileprivate func ns_free_pixmap (_:_:)`: C's whole body is the `ns_release_object` call (C 5651), and the pixmap parameter is spelled the way that export spells its own (an optional untyped pointer, C 805-811), so C's NULL arm needs no code here (round 35's Evidence)
- [x] ns_display_pixel_height  (line 5661) — C 5660-5673, landed as `func ns_display_pixel_height (_:) -> Int32`: C's enumerator over the screen list, with `NSUnionRect` accumulating each `-frame`, is the Swift `for` loop over `NSScreen.screens`; `dpyinfo` is unused in C too
- [x] ns_display_pixel_width  (line 5676) — C 5675-5688, landed as `func ns_display_pixel_width (_:) -> Int32`: the same union read as a width, in C's own two-call shape (round 35's Evidence)
- [x] ns_string_to_lispmod  (line 5691, static) — C 5691-5710, landed as `fileprivate func ns_string_to_lispmod (_:) -> Lisp_Object`: C's six `strncmp (SSDATA (SYMBOL_NAME (Q…)), s, 10)` tests keep their order and their length 10, and the five modifier symbols are read through this file's own `DEFSYM` names (C 11799's `syms_of_nsterm`)
- [x] ns_default  (line 5714, static) — C 5713-5738, landed as `fileprivate func ns_default (_:_:_:_:_:_:)`: C's two `BOOL`s are `Bool`, C 5731's comma expression is one local function (`strtod_and_test`) whose body is C's own two steps so the chain keeps its single `else if` and its short-circuit, and the two pointer uses carry `value!` behind C's own `if (value)` guard (the `Fns_get_resource` shape)
- [x] ns_initialize_display_info  (line 5742, static) — C 5741-5764, landed as `fileprivate func ns_initialize_display_info (_:)`: C 5758's chained assignment is written in C's own order (`a = b = NULL` assigns `b` first), C 5752-5755's two `isEqualToString:` reads are the eight member entries this round declares (`ns_display_info_set_resx` through `ns_display_info_set_smallest_char_width`, plus `ns_display_info_mouse_highlight` for C's `&dpyinfo->mouse_highlight`), and the AppKit spellings are the compiler's own renames (`NSColorSpaceName.deviceWhite`, `depth.colorSpaceName`, `depth.bitsPerPixel` - class B)
- [x] ns_default_font_parameter  (line 5770, static) — C 5769-5773, landed as `fileprivate func ns_default_font_parameter (_:_:)`: C's body is empty and the comment above the definition (C 5766-5768) says why, so both parameters stay unread (the `applicationWillBecomeActive` reading)
- [x] ns_update_window_end  (line 5776, static) — **dead in this build**: C opens `#ifdef NS_IMPL_GNUSTEP` at C 5774 and closes it at C 5783, so the whole definition is the arm this target does not take.  Recorded, not transcribed (the `-updateMonitors:` reading); its two statements, named in the round's note, are `NSTRACE` (empty) and `ns_redraw_scroll_bars (WINDOW_XFRAME (w))`
- [x] ns_flush_display  (line 5786, static) — C 5785-5792, landed as `fileprivate func ns_flush_display (_:)`: C's body is `EVENT_INIT` plus one `ns_read_socket_1` call whose value C discards; it waited on `FRAME_TERMINAL (f)`, which this round declares as a macro entry under the construct's own name (the `FRAME_DISPLAY_INFO` shape, and the other direction of `nsfns.swift` 639's `frame_set_terminal`)
- [x] ns_displays_reconfigured  (line 5839, static) — C 5837-5868 (the `NS_IMPL_COCOA` arm), landed as `fileprivate func ns_displays_reconfigured (_:_:_:)`: it needed this unit's view of `union buffered_input_event` (src/keyboard.h 228-235), the kbd buffer's three data entries (`kbd_buffer`/`kbd_fetch_ptr`/`kbd_store_ptr`, the core's `#[unsafe(no_mangle)]` statics), `Fequal`, the nsfns lane's `Fns_display_monitor_attributes_list`, `XSETTERMINAL` (a macro entry) and the member reader `ns_display_info_terminal` this round adds; the two enumerators it reads are mirror-valued constants (`KBD_BUFFER_SIZE` 4096, `MONITORS_CHANGED_EVENT` 35 - the values `femacs/src/include/keyboard.rs` 43 and `termhooks.rs` 62 fix)
- [x] ns_delete_display  (line 5872, static) — C 5871-5875, landed as `fileprivate func ns_delete_display (_:)`: C's whole body is the comment `/* TODO...  */`, so the body is empty here too and the parameter unread (the `ns_default_font_parameter`/`applicationWillBecomeActive` reading: a body C leaves empty is transcribed empty rather than stubbed)
- [x] ns_delete_terminal  (line 5880, static) — C 5877-5902, landed as `fileprivate func ns_delete_terminal (_:)`: it needed `terminal_display_info_ns` (already the core's own shim, `femacs/src/abi_shim.rs` 412), `image_destroy_all_bitmaps` (the core's, `femacs/src/image.rs` 1841) and one new member entry, `terminal_name`, for C 5888's guard; `ns_delete_display` (C 5900) landed in round 38, and C 5896's `setActivationPolicy:` is `NSApp.setActivationPolicy (.prohibited)`
- [x] ns_create_terminal  (line 5908, static) — C 5906-5961, landed as `fileprivate func ns_create_terminal (_:) -> UnsafeMutableRawPointer?`: round 50 landed the entry layer (create_terminal/output_ns/ns_display_info_set_terminal + the 36 setters) and round 51 the last hook it needed (`ns_new_font`), so round 52 is the body - C 5917's create call with the round-39 table's address, C 5919's `terminal_set_display_info_ns` write, C 5920's setter, the 36 hook writes as non-capturing `@convention(c)` closures over the ported functions (seven of them the nsfns/nsmenu lanes' `@_cdecl` entries), and C 5960's return
- [ ] ns_term_init  (line 5965)
- [ ] ns_term_shutdown  (line 6198)
- [x] runAlertPanel  (line 6634, static) — landed as `fileprivate func runAlertPanel (_:_:_:_:) -> Bool`: the `NS_IMPL_COCOA` arm builds an `NSAlert` (the GNUstep `NSRunAlertPanel` at C 6640 is dead), and C 6649's `[alert release]` is the reference being dropped under ARC
- [x] not_in_argv  (line 6672, static) — landed as `fileprivate func not_in_argv (_:) -> Int32`: C's `[arg UTF8String]` is the `withCString` view, and `initial_argc`/`initial_argv`/`strcmp` are imports in the lane's established shapes
- [ ] ns_font_desc_to_font_spec  (line 6915, static)
- [ ] ns_create_font_panel_buttons  (line 6992, static)
- [x] ns_in_echo_area_1  (line 7624, static) — landed as `fileprivate func ns_in_echo_area_1 (_:) -> Lisp_Object`: C 7628's `safe_calln (Qns_in_echo_area)` is one `safe_funcall` call (the macro's own expansion at lisp.h 4846-4847), and `Qns_in_echo_area` is this file's own `DEFSYM` (C 11799) imported as a data symbol
- [x] ns_in_echo_area_2  (line 7633, static) — landed as `fileprivate func ns_in_echo_area_2 (_:_:) -> Lisp_Object`: C's whole body is `return Qnil;`
- [x] ns_in_echo_area  (line 7639, static) — landed as `fileprivate func ns_in_echo_area () -> Bool`: the `internal_catch_all` call takes C 7644-7645's two helpers as `@convention(c)` closures and its `NULL`, and C 7647's `!NILP` is the predicate the entry returns
- [ ] cancel_ns_deferred_UAZoomChangeFocus_timer  (line 8460, static)
- [x] nswindow_orderedIndex_sort  (line 10085, static) — landed as `fileprivate func nswindow_orderedIndex_sort (_:_:_:) -> Int`: C sends `orderedIndex` as a **message**, and on Cocoa that selector is not declared on `NSWindow` at all (C's own definition of it is the GNUstep arm at C 10074-10077), so the port sends the same dynamic message instead of inventing a property; the three `NSOrdered*` results are C 10090-10095's three returns
- [x] ns_new_font  (line 11418, static) — C 11410-11476, landed as `fileprivate func ns_new_font (_:_:_:)` (round 51 took it ahead of `ns_create_terminal`'s body because C 5942 hands it to `terminal->set_new_font_hook`, the dependency order P-91 ① allows): C's `font` reads are the `font_<member>` family, the `struct ns_output` writes are `ns_output_set_*` and the `struct frame` writes `frame_set_*` (this round's entry rule), C's parameter reassignment is the shadow spelling, and C 11470's `! [view isFullscreen]` on a possibly-nil view is C's nil-receiver answer
- [ ] ns_xlfd_to_fontname  (line 11484)
- [x] mark_nsterm  (line 11529) — C 11528-11547, landed as `@_cdecl("mark_nsterm")` (C's own non-static entry): it walks every frame's scrollers and sends each the `mark` method round 43 landed; the `FOR_EACH_FRAME` spelling, `FRAME_NS_P`, `FRAME_NS_VIEW` and the `isKindOfClass:` test are the unit's own already, so it needed **no new entry**
- [ ] syms_of_nsterm  (line 11550)

Two definitions the pre-3.260 reader missed for the same comment-stop reason as `ns_focus` above
(the documented declarator is followed by a comment before its `{`); they were added by hand so the
checklist was complete, and since `tools/cinventory.py`'s fix they are ordinary generator rows -
the numbers here are the reader's own (1103 and 1131; the boxes' first numbers were hand counts):
- [x] ns_ua_zoom_enabled_p  (line 1103, static) — landed as `fileprivate func ns_ua_zoom_enabled_p () -> Bool`
- [x] ns_cg_rect_flip_y  (line 1131, static inline) — landed as `fileprivate func ns_cg_rect_flip_y (_ rect: CGRect) -> CGRect`


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
- [x] EmacsBell  (`@interface` 1333, `@end` 1343 - a file-local declaration; its three
      methods are the `@implementation`'s below)
  - [x] `- (void)show:(NSView *)view`  (line 1340) -> the class's `@objc func show(_ view: NSView)`
  - [x] `- (void)hide`  (line 1341) -> `@objc func hide()`
  - [x] `- (void)remove`  (line 1342) -> `@objc func remove()`
- [x] EmacsBell  (`@implementation` 1345, `@end` 1430) - **the file's first class**, a Swift
      `class EmacsBell: NSImageView`; the ivars `nestCount`/`mView`/`isAttached` (1336-1338)
      become stored properties, and each method is `@objc` so its selector is reachable
      (`#selector(hide)` in `show:`)
  - [x] `- (id)init`  (line 1347) -> `override init(frame:)` + `required init?(coder:)` (AppKit's designated initializer is `-initWithFrame:`, which is what C's `[[EmacsBell alloc] init]` is); the GNUstep no-named-images arm is dead
  - [x] `- (void)show:(NSView *)view`  (line 1373) -> `@objc func show(_ view: NSView)`
  - [x] `- (void)hide`  (line 1402) -> `@objc func hide()`
  - [x] `-(void)remove`  (line 1418) -> `@objc func remove()`
- [ ] EmacsApp  (`@implementation` 6225, `@end` 6912) - round 21 landed **C 6225-6400**: the class record with its `NS_IMPL_COCOA` ivars (`src/nsterm.h` 378-389) and the five methods below; the twenty-five that remain (C 6402-6912) are what keeps this box open
  - [x] `- (id)init`  (line 6227) — landed as `override init ()`: C 6238's `self->isFirst = YES;` is the one live statement (the `NS_IMPL_GNUSTEP` ivar at 6241 and the notification-centre observer at 6245-6251 are the arm this target does not take)
  - [x] `- (void)run`  (line 6257) — landed as `override func run ()`: C 6261-6263's `#ifndef NSAppKitVersionNumber10_9` fallback is `NSAppKitVersion.macOS10_9` (the `NSAppKitVersionNumber10_10` reading above, class B) and `NSAutoreleasePool` is this unit's own `objc_autoreleasePoolPush`/`Pop` (C 560)
  - [x] `- (void)stop: (id)sender`  (line 6295) — landed as `override func stop (_ sender: Any?)` (the `NS_IMPL_COCOA` arm; C 6304 closes it)
  - [x] `- (void)logNotification: (NSNotification *)notification`  (line 6306) — landed as `@objc func logNotification (_:)`: C's `[[notification name] UTF8String]` is `withCString` over `notification.name.rawValue`, so the three `strstr` calls read the same bytes (the new declaration is the `fputs` shape)
  - [x] `- (void)sendEvent: (NSEvent *)theEvent`  (line 6317) — landed as `override func sendEvent (_:)`: C's `NSEventTypeXxx` constants are `NSEvent.EventType` cases (C 6323's `int type` becomes the enum), the `NS_IMPL_GNUSTEP` keyboard arm (6329-6337) is dead, C 6344/6349's two `data2` values are `src/nsterm.h` 1189/1192's own `let`s, and C 6389's `di->next` walk is the new `ns_display_info_next` shim
  - [x] `- (void)showPreferencesWindow: (id)sender`  (line 6402) — landed as `@objc func showPreferencesWindow (_:)`: the four statements are C 6409-6412, and the `kind` write is the value round 28 settled (`NS_NONKEY_EVENT`, the unit's UNVERIFIED constant)
  - [x] `- (void)newFrame: (id)sender`  (line 6416) — landed as `@objc func newFrame (_:)`, the row above's twin with `KEY_NS_NEW_FRAME` (`src/nsterm.h` 804)
  - [x] `- (BOOL) openFile: (NSString *)fileName`  (line 6433) — landed as `@objc func openFile (_:) -> Bool`, the event constructor the pending open-file queue drains into; `ns_input_file`/`ns_input_line` are this file's own file-scope objects imported as data symbols, and `lispString` is the Cocoa lane's exported helper
  - [x] `- (void) updateMonitors: (NSNotification *) notification`  (line 6454) — **dead in this build**: C opens `#ifdef NS_IMPL_GNUSTEP` at C 6453 and closes it at C 6489, so the body is the arm this target does not take.  Recorded, not transcribed (the `nstrace_*` reading), which closes `EmacsApp`'s method list
  - [x] `- (void)applicationDidFinishLaunching: (NSNotification *)notification`  (line 6490) — landed as `@objc func applicationDidFinishLaunching (_:)`: the GNUstep arm (C 6497-6499) is dead, the three dummy CoreFoundation/Foundation calls (C 6531-6533) precede the `RLIMIT_NOFILE` clamp to `FD_SETSIZE`, and the antialias notification's Swift spelling is `NSFont.antialiasThresholdChangedNotification` (the compiler's own rename reading, class B)
  - [x] `- (void)antialiasThresholdDidChange:(NSNotification *)notification`  (line 6556) — landed as `@objc func antialiasThresholdDidChange (_:)`: its one statement calls the Mac font lane's exported entry (`macfont.swift` 1376 over `src/macfont.m` 747)
  - [x] `- (void) systemWillSleep:(NSNotification *)notification`  (line 6565) — landed as `@objc func systemWillSleep (_:)`: C 6569's `struct input_event ie` is the state C 6570's `EVENT_INIT` writes, `SLEEP_EVENT`'s value comes from the port's own mirror (`src/include/termhooks.rs` 55), and the `kind` write is a bitfield setter that keeps `part`
  - [x] `- (void) systemDidWake:(NSNotification *)notification`  (line 6577) — landed as `@objc func systemDidWake (_:)`, `Qpre_sleep`'s twin with `Qpost_wake` (C 6584)
  - [x] `- (BOOL) applicationSupportsSecureRestorableState: (NSApplication *)app`  (line 6610) — landed as `@objc func applicationSupportsSecureRestorableState (_:) -> Bool`: C's `return YES;` and nothing else
  - [x] `- (void) terminate: (id)sender`  (line 6615) — landed as `@objc override func terminate (_:)` (AppKit's own method, hence the override): it was the box that waited for `NS_NONKEY_EVENT`'s value, which this round takes from the enum's declaration order under Cocoa's configuration - `fileprivate let NS_NONKEY_EVENT: UInt32 = 30`, the unit's first **UNVERIFIED** constant (no Cocoa build exists here to check it against C)
  - [x] `- (NSApplicationTerminateReply)applicationShouldTerminate: (id)sender`  (line 6655) — landed as `@objc func applicationShouldTerminate (_:) -> NSApplication.TerminateReply`: C's two returns are the enum's `.terminateNow`/`.terminateCancel`, and `ns_confirm_quit` is imported as this file's own `DEFVAR_BOOL`'s data symbol
  - [x] `- (BOOL)application: sender openFile: (NSString *)file`  (line 6682) — landed as `@objc func application (_:openFile:) -> Bool`: C's `[ns_pending_files addObject: file]` is the array's `add`, and the `not_in_argv` guard is the one C 6684 names
  - [x] `- (BOOL)application: sender openTempFile: (NSString *)file`  (line 6691) — landed as `@objc func application (_:openTempFile:) -> Bool`, the same three lines as the row above
  - [x] `- (BOOL)application: sender openFileWithoutUI: (NSString *)file`  (line 6700) — landed as `@objc func application (_:openFileWithoutUI:) -> Bool`, the same three lines again
  - [x] `- (void)application: sender openFiles: (NSArray *)fileList`  (line 6708) — landed as `@objc func application (_:openFiles:)`: C's `NSEnumerator` walk is the loop, and C 6719's `replyToOpenOrPrint:` is Swift's `reply (toOpenOrPrint:)`
  - [x] `- (NSMenu *)applicationDockMenu: (NSApplication *) sender`  (line 6725) — landed as `@objc func applicationDockMenu (_:) -> NSMenu?`: C's `return dockMenu;` hands back the untyped stand-in C 205 holds for the `EmacsMenu` class that has not landed
  - [x] `- (void)applicationWillBecomeActive: (NSNotification *)notification`  (line 6732) — landed as an empty `@objc func applicationWillBecomeActive (_:)`: C's body is the commented-out `ns_app_active` line and nothing else
  - [x] `- (void)applicationDidBecomeActive: (NSNotification *)notification`  (line 6738) — landed as `@objc func applicationDidBecomeActive (_:)`: the GNUstep arm (C 6742-6745) is dead, and C 6748/6750's two calls are this unit's own entries
  - [x] `- (void)applicationDidResignActive: (NSNotification *)notification`  (line 6752) — landed as `@objc func applicationDidResignActive (_:)`: C's one live statement is the `ns_send_appdefined (-1)` call
  - [x] `- (void)timeout_handler: (NSTimer *)timedEntry`  (line 6769) — landed as `@objc func timeout_handler (_ timedEntry: Timer)`: its one statement calls `ns_send_appdefined (-2)`, so it lands with round 22
  - [x] `- (void)sendFromMainThread:(id)unused`  (line 6778) — landed as `@objc func sendFromMainThread (_ unused: Any?)`: the main-thread half of `ns_send_appdefined`'s non-main-thread arm (C 5027 posts this very selector), which is why the two land together
  - [x] `- (void)fd_handler:(id)unused`  (line 6783) — landed as `@objc func fd_handler (_:)`: the `while true` is C 6798's `for (;;)`, `FD_ZERO`/`FD_SET`/`FD_ISSET` are the three Darwin expansions this unit imports (`__darwin_fd_zero`/`__darwin_fd_set`/`__darwin_fd_isset`), and C 6825/6832's two `&local` assignments became the four call shapes C's `wfds`/`tmo` select (Swift cannot assign `&`, so the choice happens at the call)
  - [x] `- (void)requestService: (NSPasteboard *)pboard userData: (NSString *)userData error: (NSString **)error`  (line 6881) — landed as `@objc func requestService (_:userData:error:)`: the queue `ns_read_socket_1` drains is filled here, and C 6886's factory is the selection lane's `stringWithLispString` over `ns_string_from_pasteboard`
  - [x] `- (BOOL)fulfillService: (NSString *)name withArg: (NSString *)arg`  (line 6891) — landed as `@objc func fulfillService (_:withArg:) -> Bool`, the service-request twin of `openFile:` (C 6905's `EV_MODIFIERS` is this unit's own entry)
- [ ] EmacsView  (`@implementation` 7039, `@end` 9746) - round 25 landed **the class record** (the fifteen ivars of `src/nsterm.h` 470-488, `@public emacsframe` included) and three methods (`fsIsNative` C 8952, `isFullscreen` C 8957, `updateCollectionBehavior` C 8981); the protocol list (nsterm.h 466) is the named deferral - Swift enforces the required methods of `NSTextInput`/`NSTextInputClient`/`NSWindowDelegate` where the conformance is declared, so it lands with that method batch
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
  - [x] `- (void) insertText: (id) string replacementRange: (NSRange) replacementRange`  (line 7435) — landed as `@objc func insertText (_:replacementRange:)`: C unmarks and hands the string to the older method (C 7440-7441), the `NSAttributedString` arm being C 7438-7439
  - [x] `- (void) setMarkedText: (id) string selectedRange: (NSRange) selectedRange replacementRange: (NSRange) replacementRange`  (line 7444) — landed as `@objc func setMarkedText (_:selectedRange:replacementRange:)`, whose body is C 7448's two-argument call
  - [x] `- (nullable NSAttributedString *) attributedSubstringForProposedRange: (NSRange) range actualRange: (nullable NSRangePointer) actualRange`  (line 7451) — landed as `@objc func attributedSubstring (forProposedRange:actualRange:) -> NSAttributedString?`: C's whole body is `return nil;`
  - [ ] `- (NSRect) firstRectForCharacterRange: (NSRange) range actualRange: (nullable NSRangePointer) actualRange`  (line 7458)
  - [ ] `- (NSRect)accessibilityFrame`  (line 7464)
  - [x] `- (void)insertText: (id)aString`  (line 7481) — landed as `@objc override func insertText (_:)` (the override is for `NSResponder`'s own method, measured): C 7515-7523's surrogate pair is this unit's two predicates and `surrogates_to_codepoint` (written out from their `src/coding.h` bodies, since the port's mirror spells them as plain functions), C 7524-7526's tilde substitution and the 32/0 modifier rule are kept, and the `kind` write uses the three enum constants the port's mirror fixes
  - [x] `- (void)setMarkedText: (id)aString selectedRange: (NSRange)selRange`  (line 7538) — landed as `@objc func setMarkedText (_:selectedRange:)`: C 7561-7562's `[workingText release]` + `[str copy]` is the ARC assignment, `ns_working_text` is this file's own `DEFVAR_LISP` data symbol, and C 7578-7579's `NS_KEYLOG` block is dead
  - [x] `- (void)deleteWorkingText`  (line 7572) — landed as `@objc func deleteWorkingText ()`: C 7580's `[workingText release]` is the reference being dropped, and the two `EMACS_EVENT` writes are C 7587-7588
  - [x] `- (BOOL)hasMarkedText`  (line 7593) — landed as `@objc func hasMarkedText () -> Bool`; C's whole body is the `workingText` test
  - [x] `- (NSRange)markedRange`  (line 7601) — landed as `@objc func markedRange () -> NSRange`: C 7605-7606's `NSMakeRange (NSNotFound, 0)` is the no-marked-text answer, and C 7607-7608's `NS_KEYLOG` block is dead (`NS_KEYLOG` is 0 at C 7241)
  - [x] `- (void)unmarkText`  (line 7613) — landed as `@objc func unmarkText ()`: C's two statements are the `deleteWorkingText` call and the `processingCompose` clear
  - [ ] `- (NSRect)firstRectForCharacterRange: (NSRange)theRange`  (line 7651)
  - [x] `- (NSInteger)conversationIdentifier`  (line 7699) — landed as `@objc func conversationIdentifier () -> Int`: C returns the receiver's own address as an integer, which Swift spells through `Unmanaged`
  - [x] `- (void)doCommandBySelector: (SEL)aSelector`  (line 7705) — landed as `@objc func doCommandBySelector (_:)`: C 7709-7710's `NS_KEYLOG` block is dead, the `deleteBackward:` arm writes `NON_ASCII_KEYSTROKE_EVENT` (the enum value the port's mirror fixes) and code `0xFF08`, and C's `EV_TRAILER ((id)nil)` passes the same `emacsframe` this unit's other callers do
  - [x] `- (NSArray *)validAttributesForMarkedText`  (line 7725) — landed as `@objc func validAttributesForMarkedText () -> [NSAttributedString.Key]`: C's lazy `static NSArray *arr` starts empty and is never filled (its one `arrayWithObject:` is commented out at C 7729), so the body is the empty array
  - [ ] `- (NSRange)selectedRange`  (line 7733)
  - [x] `- (NSUInteger)characterIndexForPoint: (NSPoint)thePoint`  (line 7759) — landed as `@objc func characterIndexForPoint (_:) -> UInt`, C's whole body being `return 0;`
  - [x] `- (unsigned int)characterIndexForPoint: (NSPoint)thePoint`  (line 7761; the `#if defined (NS_IMPL_COCOA) || GNUSTEP_GUI…` arm of the box above) — the same selector and the same body; this target takes C 7757's first conjunct, so the row above is the live spelling and the two share one Swift `func`
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
  - [x] `- (BOOL)fsIsNative`  (line 8952) — landed as `func fsIsNative () -> Bool`; C's whole body is the ivar read
  - [x] `- (BOOL)isFullscreen`  (line 8957) — landed as `func isFullscreen () -> Bool`; C 8967-8971's `#else` is dead on this target, and C 8968's `styleMask & NSWindowStyleMaskFullScreen` is the option-set's `contains`
  - [x] `- (void)updateCollectionBehavior`  (line 8981) — landed as `func updateCollectionBehavior ()`; the three `b &=`/`b |=` lines are the collection-behavior option set's insert/remove, `ns_use_native_fullscreen` is imported as the C 11702 `DEFVAR_BOOL`'s data symbol (the `ns_auto_hide_menu_bar` shape), and `FRAME_PARENT_FRAME`/`FRAME_TOOLTIP_P` are this unit's existing entries
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
- [ ] EmacsWindow  (`@implementation` 9756, `@end` 10610) - round 24 landed **the class record** (the four Cocoa ivars, `src/nsterm.h` 421-424) and five methods; the four drag-and-drop ivars (nsterm.h 426-429, whose `dnd_mode` is `nsselect.swift` 104's enum) and `dealloc` are the two named deferrals, and the remaining methods keep this box open
  - [ ] `- (instancetype) initWithEmacsFrame: (struct frame *) f`  (line 9759)
  - [ ] `- (instancetype) initWithEmacsFrame: (struct frame *) f fullscreen: (BOOL) fullscreen screen: (NSScreen *) screen`  (line 9765)
  - [ ] `- (void)createToolbar: (struct frame *)f`  (line 9878)
  - [ ] `- (void)dealloc`  (line 9912) — **deferred by name**: Swift forbids overriding `dealloc` (measured: `error: 'init(coder:)' is unavailable`'s family - ARC owns the releases C writes), so this box records the reference's three statements (`toolbar` release, `setToolbar: nil`, `last_drag_event` release) as the ARC equivalent they already read as
  - [x] `- (NSInteger) borderWidth`  (line 9928) — landed as `func borderWidth () -> Int`: C's `NSWidth` is the rect's own `width`, and the `contentView` read is optional-chained because C's own message to nil does nothing
  - [ ] `- (void)setParentChildRelationships`  (line 9934)
  - [x] `- (void)orderFront:(id)sender`  (line 10043) — landed as `@objc override func orderFront (_:)`: the parent arm removes and re-adds the receiver above it, the `else` is the `super` call, and C 10047's `[self parentWindow]` is Swift's renamed `parent`
  - [x] `- (void)makeKeyAndOrderFront:(id)sender`  (line 10057) — landed as `@objc override func makeKeyAndOrderFront (_:)`: C 10064's `[self makeKeyWindow]` is Swift's `makeKey ()`
  - [ ] `- (NSInteger) orderedIndex`  (line 10074)
  - [x] `- (void)orderBack:(id)sender`  (line 10098) — landed as `@objc override func orderBack (_:)`: C sorts the parent's children with the comparator above (`sorted` over `childWindows`), re-adds the receiver above them, then walks the sorted list; C's `NSArray` is the optional array with its own empty default
  - [ ] `- (BOOL)restackWindow:(NSWindow *)win above:(BOOL)above`  (line 10124)
  - [ ] `- (id)accessibilityAttributeValue:(NSString *)attribute`  (line 10158)
  - [ ] `- (NSRect)constrainFrameRect:(NSRect)frameRect toScreen:(NSScreen *)screen`  (line 10230)
  - [ ] `- (void)performZoom:(id)sender`  (line 10299)
  - [ ] `- (void)zoom:(id)sender`  (line 10306)
  - [ ] `- (void)setAppearance`  (line 10398)
  - [ ] `- (void)setFrame:(NSRect)windowFrame display:(BOOL)displayViews`  (line 10424)
  - [ ] `- (void)setFrame:(NSRect)windowFrame display:(BOOL)displayViews animate:(BOOL)performAnimation`  (line 10433)
  - [x] `- (void)setFrameTopLeftPoint:(NSPoint)point`  (line 10444) — landed as `override func setFrameTopLeftPoint (_:)`; C's body is the `super` call and nothing else
  - [x] `- (BOOL)canBecomeKeyWindow`  (line 10452) — landed as `override var canBecomeKey: Bool` (the AppKit apinotes' rename, class B: no symbol is invented); the frame comes through this unit's `emacs_view_frame` shim and the predicate is the `frame_no_accept_focus` entry
  - [x] `- (BOOL)canBecomeMainWindow`  (line 10457) — landed as `override var canBecomeMain: Bool` (the same rename): C's whole body is `return YES;`
  - [x] `- (void) setLastDragEvent: (NSEvent *) event`  (line 10463) — landed as `func setLastDragEvent (_:)`: C's `[event copy]` is `copy()` with the `NSCopying` result cast back to the record type
  - [ ] `- (NSDragOperation) draggingSourceOperationMaskForLocal: (BOOL) is_local`  (line 10470)
  - [ ] `- (void) draggedImage: (NSImage *) image endedAt: (NSPoint) screen_point operation: (NSDragOperation) operation`  (line 10475)
  - [ ] `- (void) draggedImage: (NSImage *) dragged_image movedTo: (NSPoint) screen_point`  (line 10482)
  - [ ] `- (BOOL) mustNotDropOn: (NSView *) receiver`  (line 10522)
  - [ ] `- (NSDragOperation) beginDrag: (NSDragOperation) op forPasteboard: (NSPasteboard *) pasteboard withMode: (enum ns_return_frame_mode) mode returnFrameTo: (struct frame **) frame_return prohibitSame: (BOOL) prohibit_same_frame followTooltip: (BOOL) follow_tooltip`  (line 10528)
- [x] EmacsScroller  (`@implementation` 10620, `@end` 11117) - **closed in round 46**: every method is landed except the one Swift cannot declare; round 42 landed **the class record** (the eleven ivars of `src/nsterm.h` 716-733, two of whose C names `NSView` already owns: `window`/`frame`, carried as `emacs_window`/`emacs_frame` - a declared deviation) and two methods (`initFrame:window:` C 10650 and, in round 43, `setFrame:` C 10703, `dealloc` C 10719, `condemn` C 10734, `reprieve` C 10742, `judge` C 10750, `mark` C 10777 and `resetCursorRects` C 10788); `+scrollerWidth` (C 10626) is **omitted** because Swift refuses both spellings of it (the `EmacsFileDelegate` class of refusal), and rounds 44-46 the rest: `checkSamePosition:portion:whole:` (C 10806), `setPosition:portion:whole:` (C 10814), `sendScrollEventAtLoc:fromEvent:` (C 10850), `repeatScroll:` (C 10894), `mouseDown:` (C 10929), `mouseDragged:` (C 11051), `mouseUp:` (C 11094) and `scrollWheel:` (C 11109)
  - [x] `+ (CGFloat) scrollerWidth`  (line 10626) — **omitted, recorded**: Swift refuses the override (`cannot override 'scrollerWidth' which has been marked unavailable`) *and* the `@objc(scrollerWidth)` renamed spelling (`conflicts with method 'scrollerWidth()' from superclass 'NSScroller'`), and nothing in the reference calls it, so omitting it changes no caller's behaviour (the `EmacsFileDelegate` shape; the ledger's `## ABI` (3) carries the gap)
  - [x] `- (instancetype)initFrame: (NSRect )r window: (Lisp_Object)nwin`  (line 10650) — landed as `@objc(initFrame:window:) init(frame:window:)`: C's descending `for` is the `while`, C 10679's `20 / pixel_length` keeps C's integer division, C 10666-10667's GNUstep arm is dead here, and the `isKindOfClass: [EmacsScroller class]` count is the `is EmacsScroller` test
  - [x] `- (void)setFrame: (NSRect)newRect`  (line 10703) — landed as the `frame` property's **setter** (`override var frame: NSRect { get { super.frame } set(newRect) { … } }`): AppKit's `-setFrame:` is imported as the property (measured: a plain `override func setFrame(_:)` is refused, and `super.setFrame` has no member of that name), and C overrides only the setter, so the getter is the superclass's; C 10713's `20 / pixel_length` keeps C's integer division
  - [x] `- (void)dealloc`  (line 10719) — landed as `deinit`: a Swift class cannot name `dealloc`, and C 10730's `[super dealloc]` is implicit in the runtime's own teardown (the `ns_free_frame_resources` reading); the two `wset_*_scroll_bar` writes are imported setters
  - [x] `- (instancetype)condemn`  (line 10734) — landed as `func condemn () -> EmacsScroller` (C's `instancetype` is this class's own type); C 10736's `NSTRACE` is the empty macro
  - [x] `- (instancetype)reprieve`  (line 10742) — landed as `func reprieve () -> EmacsScroller`, `condemn`'s sibling
  - [x] `-(bool)judge`  (line 10750) — landed as `func judge () -> Bool`: C's `block_input ()`/`unblock_input ()` pair wraps the teardown, C 10759's cast is the `as? EmacsView` the unit already uses, and C 10771's `[self release]` is the reference the method drops (`Unmanaged.passUnretained (self).release ()`)
  - [x] `- (void) mark`  (line 10777) — landed as `func mark ()` (C's own spacing in the selector): it needs `XSETWINDOW` (a macro entry of the `XSETFRAME` family) and the collector's `mark_object` (already the core's, `femacs/src/alloc.rs` 7308)
  - [x] `- (void)resetCursorRects`  (line 10788) — landed as `override func resetCursorRects ()`: the `NS_IMPL_GNUSTEP || MAC_OS_X_VERSION_MIN_REQUIRED < 101300` block at C 10796-10802 (its `setOnMouseEntered:` message included) is dead on this target (260000), so it is recorded, not transcribed
  - [x] `- (int) checkSamePosition: (int) position portion: (int) portion whole: (int) whole`  (line 10806) — landed as `func checkSamePosition (_:portion:whole:) -> Int32`: C's whole body is the four-way conjunct and it returns an `int`, so the port spells C's integral conversion of it
  - [x] `- (instancetype)setPosition: (int)position portion: (int)portion whole: (int)whole`  (line 10814) — landed as `func setPosition (_:portion:whole:) -> EmacsScroller`: the `NS_IMPL_COCOA` arms are the live ones (C 10825-10826, C 10839-10840), the two setters are Swift's `knobProportion`/`doubleValue`, C 10835's `max ((float)whole*min_portion/pixel_length, portion)` is `Swift.max` over C's promoted `float` arithmetic with C's truncation back into the `int`, and C's reassignment of the `portion` parameter is the shadow the lane already uses for that shape
  - [x] `- (void) sendScrollEventAtLoc: (float)loc fromEvent: (NSEvent *)e`  (line 10850) — landed as `func sendScrollEventAtLoc (_:fromEvent:)`: `part` is written through the new bitfield entry `input_event_set_part` (the `input_event_set_kind` counterpart), C's `float` expressions are converted where C's `make_number` converts them (`XSETINT` takes C's `int`), C's mixed `unsigned int`/`int` modifier OR is `truncatingIfNeeded:`, and C's `EV_MODIFIERS (e)`/`EV_TIMESTAMP (e)` on a **possibly-nil** event are written out (nil's `modifierFlags`/`timestamp` are zero) because both macros take a live `NSEvent`
  - [x] `- (instancetype)repeatScroll: (NSTimer *)scrollEntry`  (line 10894) — landed as `@objc func repeatScroll (_:) -> EmacsScroller`: C 10896-10897's `[self window]` is AppKit's own message (not the class's `window` ivar, which is why that ivar is `emacs_window` here) and its nil receiver answers the zero-valued point, C 10906's `release` is the reference the method drops, C 10912-10919's `retain` is ARC's strong reference to the timer `scheduledTimer` returns (the `SCROLL_BAR_CONTINUOUS_DELAY` macro is a file-scope `let`), and C 10916's `@selector (repeatScroll:)` is this method's own `#selector`
  - [x] `- (void)mouseDown: (NSEvent *)e`  (line 10929) — landed as `@objc override func mouseDown (with:)` (Swift's own rename reading; the Objective-C selector stays `mouseDown:`): C 10945-10950's two `case`s are dead on this target, C 10934's `UNINIT` is conf_post.h 369's `= {0,}` here (`GCC_LINT` is 1 in the frozen macros, line 1162), C 10956's `%ld` prints `part.rawValue`, the rect reads are the rect's own accessors, and C 11029's `[super mouseUp: fake_event]` is the superclass's renamed `mouseUp (with:)` with C's nil no-op kept
  - [x] `- (void)mouseDragged: (NSEvent *)e`  (line 11051) — landed as `@objc override func mouseDragged (with:)`: C's two locals are `double` (not `CGFloat`), C 11077-11079's bound is `length + last_mouse_offset` with C's own promotions, and C 11090 hands the `double` to the `float` parameter of `sendScrollEventAtLoc:`, which the port converts where C's call does
  - [x] `- (void)mouseUp: (NSEvent *)e`  (line 11094) — landed as `@objc override func mouseUp (with:)`: C 11101's `release` is the reference the method drops (the `repeatScroll:` reading) and C 11104's enumerator is one this unit already imports
  - [x] `- (void) scrollWheel: (NSEvent *)theEvent`  (line 11109) — landed as `@objc override func scrollWheel (with:)`: C 11113's cast is the unit's `as? EmacsView` and C 11114's message to a nil view is the optional chain's no-op
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
- [x] DOCK_IGNORE_LIMIT  (line 708, body c1dfd96eea8c) — landed as a file-scope `let` (CGFloat, C's `int` promoted at the comparisons)
- [x] EV_BUTTON  (line 438, body 02ec0a9e0dff) — landed as `func EV_BUTTON (_ e: NSEvent) -> Int`
- [x] EV_MODIFIERS  (line 425, body 0a951d59f9e1) — landed as `func EV_MODIFIERS (_ e: NSEvent) -> UInt32`
- [x] EV_MODIFIERS2  (line 397, body 8155b8d11eb3) — landed as `func EV_MODIFIERS2 (_ flags: UInt, _ kind: Lisp_Object) -> UInt32`; C's `NSUInteger`→`unsigned int` conversions are `truncatingIfNeeded:`
- [x] EV_TIMESTAMP  (line 444, body 8e139f589481) — landed as `func EV_TIMESTAMP (_ e: NSEvent) -> Double`
- [x] EV_TRAILER  (line 448, body 0dd430c310a9) — landed as `fileprivate func EV_TRAILER (_ e: NSEvent?, _ emacsframe: UnsafeMutableRawPointer?)`: the body reads two names from the caller's scope, so the port passes both (the `EV_MODIFIERS2` shape), and `emacsframe` is the local every handler declares (C 6404 is the first expansion's)
- [x] EV_TRAILER2  (line 455, body bf5ab84e8621) — landed as `fileprivate func EV_TRAILER2 (_ e: NSEvent?)`: C's `Vinhibit_quit` save/restore is kept as written (the core global is imported as a data symbol), and C 457's `if (e)` guard is `if e != nil`
- [x] EV_UDMODIFIERS  (line 427, body 89301cbc268c) — landed as `func EV_UDMODIFIERS (_ e: NSEvent) -> Int32`, C's ten comparisons transcribed one for one
- [x] NO_APPDEFINED_DATA  (line 318, body 9d95217d51d9)  — landed as a file-scope `let` of the same name
- [x] NSAppKitVersionNumber10_10  (line 10407, body ed9f6e93628b) — the *name* is used at
      C 2139 (`ns_set_appearance`) and spelled there as Swift's `NSAppKitVersion.macOS10_10`
      (class B of the 3.255 ruling: the AppKit apinotes rename the vintage constants, and no
      symbol is invented); C's own `#define … 1343` fallback at C 10406-10408 is inside the later
      `-[EmacsWindow setAppearance]` slice, whose box stays where it is
- [x] NSAppKitVersionNumber10_9  (line 6262, body 74b10c057ded) — landed with round 21: C 6261-6263's `#ifndef` fallback names the vintage constant, which `EmacsApp run` spells `NSAppKitVersion.macOS10_9` (the `NSAppKitVersionNumber10_10` reading above, class B: the apinotes rename it and no symbol is invented)
- [x] NSLeftAlternateKeyMask  (line 355, body a08e8dccf9c4)  — landed as a file-scope `let` of the same name
- [x] NSLeftCommandKeyMask  (line 353, body 6f54877ddc32)  — landed as a file-scope `let` of the same name
- [x] NSLeftControlKeyMask  (line 351, body 5cc979ee22b5)  — landed as a file-scope `let` of the same name
- [x] NSRightAlternateKeyMask  (line 356, body 2e125f92fda6)  — landed as a file-scope `let` of the same name
- [x] NSRightCommandKeyMask  (line 354, body 9d1bbbd0282a)  — landed as a file-scope `let` of the same name
- [x] NSRightControlKeyMask  (line 352, body 5a769eb9b670)  — landed as a file-scope `let` of the same name
- [x] NS_FUNCTION_KEY_MASK  (line 350, body 91eddf5a25af)  — landed as a file-scope `let` of the same name
- [ ] NS_KEYLOG  (line 7241, body b6589fc6ab0d)
- [x] NS_PARENT_WINDOW_LEFT_POS  (line 803, body 74769de1ca23) — landed as `func NS_PARENT_WINDOW_LEFT_POS (_ f:) -> CGFloat` = `NSMinX (ns_parent_window_rect (f))`
- [x] NS_PARENT_WINDOW_TOP_POS  (line 804, body 7f398f29c646) — landed as `func NS_PARENT_WINDOW_TOP_POS (_ f:) -> CGFloat` = `NSMaxY (ns_parent_window_rect (f))`
- [ ] SCROLL_BAR_CONTINUOUS_DELAY  (line 10624, body cea8ad3417bc)
- [ ] SCROLL_BAR_FIRST_DELAY  (line 10623, body 1b390cd54a0c)

## Types and their fields — 1
- [x] EmacsMargins: CGFloat top, CGFloat bottom, CGFloat left, CGFloat right — landed as a Swift
      `struct EmacsMargins`; measured `sizeof 32`, `align 8`, members at 0/8/16/24 on both sides
      (Evidence)

(Not counted here, the way `docs/ports/nsselect.md` does not count `Lisp_Subr`/`specpdl_ref`: the
unit also declares Swift **views** of `struct input_event` (src/termhooks.h 371-412) - which this
file's `emacs_event` points at - and, since round 14, of `struct font_metrics` (src/font.h
414-417), which `ns_compute_glyph_string_overhangs` holds as a local and hands to the core's
driver slot through a pointer (the same view `macfont.swift` 49-60 and `femacs/src/font.rs` 196
already carry; `spikes/gated-layout-probe.py` reads all three arms as `sizeof=10, align=2`, members
0/2/4/6/8 - Evidence).  The record is termhooks.h's and already has a Rust mirror
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
- [x] static BOOL ns_is_UAZoomEnabled  (line 1095, static) — landed as `fileprivate var ns_is_UAZoomEnabled: Bool = false`
- [x] static unsigned long ns_UAZoomEnabled_last_called_time_ns  (line 1096, static) — landed (`UInt64`)
- [x] static NSTimer ns_deferred_UAZoomChangeFocus_timer  (line 1100, static) — landed as `Timer?`
- [x] static NSRect ns_UAZoom_cursor_rect_new  (line 1154, static) — landed as `NSRect.zero`
- [x] static NSRect ns_UAZoom_cursor_rect_old  (line 1155, static) — landed as `NSRect.zero`
- [x] static BOOL ns_update_was_UAZoomEnabled  (line 1159, static) — landed as `Bool = false`
- [x] static EmacsBell bell_view  (line 1433, static) — landed as `fileprivate var bell_view: EmacsBell? = nil`; C's `[[EmacsBell alloc] init]` + `[bell_view retain]` is ARC's strong reference to the same ownership
- [x] static NSMutableDictionary fringe_bmp  (line 3089, static) — landed as
      `fileprivate var fringe_bmp: NSMutableDictionary?` (C zero-initialises it to NULL, i.e. `nil`);
      round 15's two writers are `ns_define_fringe_bitmap`/`ns_destroy_fringe_bitmap`


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
- [x] static const unsigned long NS_UAZOOMENABLED_CACHE_INTERVAL_NS  (line 1095) — landed as a
      file-scope `let` (`500 * NSEC_PER_MSEC`); a generator gap of the `static const` kind
- [x] static NSTimeInterval NS_UAZOOMENABLED_DEFER_INTERVAL_SECS  (line 1097) — landed as a
      file-scope `var` (0.2); the same gap family
- [x] struct redisplay_interface ns_redisplay_interface  (lines 5801-5835, static) — **a second
      generator gap of this hand-added family, found by round 38 and named here rather than left
      invisible**: `tools/extract-abi.py` lists neither the table nor a row for it, so no gate sees
      its absence (rounds 37/38 walked past the block C 5796-5835).  It is not landed: the table's
      28 members are function pointers (`ns_frame_parm_handlers`, the six `gui_*` entries of the
      xdisp lane, `ns_scroll_run`, `ns_after_update_window_line`, the two `NULL`s this arm takes
      for `update_window_begin`/`update_window_end`, `ns_flush_display`, …), so landing it needs
      the `struct redisplay_interface` spelling plus those imports settled first
      (`docs/baseline/redisplay_interface-layout.tsv` is the record's measured layout).
      **Landed in round 39** as a file-scope `var ns_redisplay_interface`, with this unit's own view
      of the record in the arm it ports (`NS_IMPL_COCOA`: the `ns_cocoa` rows, 28 members at
      0…216, `sizeof 224`, all optional `@convention(c)` types) and C's own order kept entry by
      entry.  The C symbol itself still needs a port-side definition, the state `nsfns.swift`
      2070's array is in (Swift cannot emit a C-named data symbol), and that is the ledger's
      `## ABI` note below.

# ----------------------------------------------------------------------
## Decisions

### Progress (this lane is incremental)

`src/nsterm.m` is 11,811 lines - the largest `.m` of batch F - and is transcribed in C order,
one slice per round (`docs/ports/xdisp.md` is the same shape for `src/xdisp.c`).  **Round 1 landed
C 60-390**: the `#include`/`#import` prologue (C 60-81) recorded rather than transcribed, the
first three statics (C 82-88), the dead `NSTRACE_ENABLED` block (C 96-149), the
`NSColor (EmacsColor)` category (C 152-206), the `convert_ns_to_X_keysym` table (C 214-293), the
file-scope objects (C 295-347), the seven modifier masks (C 349-356) and the file's first two
functions `mod_of_kind`/`ev_modifiers_helper` (C 358-389).  **Round 2 landed C 391-519** - the
`EV_*` accessors' five self-contained macros (C 397-444), the forward declarations recorded
(C 473-475), `ns_init_events`/`ns_finish_events` (C 484-495) and `hold_event`/`append2`
(C 498-519).  **Round 3 landed C 522-638** - `ns_relocate` (C 525-557), `ns_init_pool`
(C 560-568), `ns_init_locale` (C 569-600) and the four retain/release/pool hooks
`ns_release_object`/`ns_retain_object`/`ns_alloc_autorelease_pool`/`ns_release_autorelease_pool`
(C 602-638).  **Round 4 lands C 641-991** - the menu-bar predicate (C 641-648), `struct
EmacsMargins` (C 650-656), `ns_screen_margins` and its hidden-dock variant (C 659-753),
`ns_menu_bar_height` (C 754-772), `ns_parent_window_rect` with the two `NS_PARENT_WINDOW_*`
macros (C 773-807), `ns_row_rect` (C 808-827), `ns_frame_scale_factor` (C 828-891) and
`constrain_frame_rect` (C 892-991) - so the covered range is now **C 60-991**, with
`EV_TRAILER`/`EV_TRAILER2` (C 448-470) held to C 5013 (below).

**Round 5 lands C 1001-1211** under the integrator's ruling for the forward reference (see
`## ABI` (4)): `ns_constrain_all_frames` (C 1001-1030), `ns_update_auto_hide_menu_bar`
(C 1033-1073), `ns_update_begin` (C 1075-1087) and the whole macOS Accessibility Zoom block
(C 1090-1211 - its six statics, two interval constants, `ns_ua_zoom_enabled_p`,
`ns_cg_rect_flip_y` and `ns_UAZoomChangeFocus`) - so the covered range is now **C 60-1211**.
**Round 6 lands C 1212-1492** - `ns_update_end` (C 1212-1234), `ns_focus` (C 1236-1273, which
the generator misses) and `ns_unfocus` (C 1282-1310), the visible bell's narrative (C 1313-1330),
**the file's first class** `EmacsBell` (C 1333-1430, its `@interface` and all four methods),
`bell_view` (C 1433) and `ns_ring_bell` (C 1436-1469); `hide_bell` (C 1471-1491) is dead in this
build and is recorded.  **Round 9 lands C 1877-1956**: `ns_set_window_size` (C 1877-1915) and
`ns_set_window_size_and_position` (C 1920-1956).  **Round 8 landed C 1744-1876**: the frame-resource teardown (`ns_free_frame_resources`
C 1744-1780, exported), `ns_destroy_window` (C 1782-1810), `compute_offset` (C 1812-1852) and
**`ns_set_offset` (C 1854-1875) - the definition the (4) table's first function row waited for**,
after which `nm -u` no longer lists `_ns_set_offset`.  **Round 7 landed C 1493-1743** - the focus hooks (C 1493-1525), the raise/lower pair and their
dispatcher (C 1527-1578), `ns_frame_rehighlight` (C 1582-1620) and the visibility hooks
(C 1624-1742, three of them `@_cdecl` exports) - and round 8 lands C 1744-1876, so the covered range is
now **C 60-1956**.  **Round 10 lands C 1959-2398** - the seven frame-parameter handlers
`ns_set_undecorated` (C 1961-1996), `ns_set_parent_frame` (C 1999-2040),
`ns_set_no_focus_on_map` (C 2043-2058), `ns_set_no_accept_focus` (C 2061-2075) and
`ns_set_z_group` (C 2078-2116), the `NS_IMPL_COCOA` trio `ns_set_appearance_1` (C 2120-2128),
`ns_set_appearance` (C 2131-2146) and `ns_set_transparent_titlebar` (C 2149-2165),
`ns_fullscreen_hook` (C 2169-2181), the whole color block `ns_get_color` (C 2191-2297),
`ns_lisp_to_color` (C 2300-2312), `ns_query_color` (C 2315-2329), `ns_defined_color`
(C 2332-2354) and `ns_query_frame_background_color` (C 2357-2363), and **`ns_set_frame_alpha`
(C 2366-2398) - the definition the (4) table's second function row waited for** - so the covered
range is now **C 60-2398**.  The next live definition is C 2409's `frame_set_mouse_pixel_position`
- and **round 11 lands C 2401-2621**, the whole Mouse handling section: `frame_set_mouse_pixel_position`
(C 2408-2430, the `NS_IMPL_COCOA` arm; the GNUstep arm is dead), `ns_note_mouse_movement`
(C 2432-2472), `ns_mouse_position` (C 2475-2595) and `ns_frame_up_to_date` (C 2598-2621) - so the
covered range is now **C 60-2621**.  **Round 12 lands C 2624-2761**: `ns_define_frame_cursor`
(C 2624-2637, the RIF `define_frame_cursor_hook`), the Keyboard handling banner (C 2641-2645),
`ns_convert_key` (C 2648-2663), **`get_keysym_name` (C 2666-2677) - exported, since src/lisp.h
5436 declares it and this unit is where its definition lives**, `right_mod` (C 2680-2684),
`nil_or_none` (C 2686-2690) and `ns_get_shifted_character` (C 2692-2761, the name the 3.260
reader fix surfaced, with `import Carbon` as C 72-73's header) - so the covered range is now
**C 60-2761**.  **Round 13 lands C 2789-2960** - the Block drawing operations section's first four
definitions: `ns_clear_frame` (C 2788-2820, exported - src/nsterm.h 1124), `ns_clear_frame_area`
(C 2823-2846), `ns_scroll_run` (C 2849-2916) and `ns_clear_under_internal_border` (C 2919-2960);
C 2771-2786's `ns_redraw_scroll_bars` is `NS_IMPL_GNUSTEP`-only, i.e. dead for this lane and
recorded - so the covered range is now **C 60-2960**.  **Round 14 lands C 2963-3079**: the RIF's
fringe-update preparation `ns_after_update_window_line` (C 2963-3019), `ns_shift_glyphs_for_insert`
(C 3022-3036), the "Character encoding and metrics" banner (C 3040-3044) and
`ns_compute_glyph_string_overhangs` (C 3047-3079) - **no new C export this round** (all three are
C `static`s) - so the covered range is now **C 60-3079**.  The next live definition is C 3091's
**Round 15 lands C 3083-3291**: the Fringe and cursor drawing banner (C 3083-3087), the file's
last file-scope object `fringe_bmp` (C 3089) and the section's three definitions
`ns_define_fringe_bitmap` (C 3091-3195), `ns_destroy_fringe_bitmap` (C 3198-3202) and
`ns_draw_fringe_bitmap` (C 3205-3291) - **no new C export** (all three are C `static`s) - so the
covered range is now **C 60-3291**.  **Round 16 lands C 3294-3508**: `ns_draw_window_cursor`
(C 3295-3426, the RIF cursor hook), `ns_draw_vertical_window_border` (C 3430-3449) and
`ns_draw_window_divider` (C 3453-3508) - **no new C export** again - so the covered range is now
**C 60-3508**.  **Round 17 lands C 3511-3807**: the Glyph drawing operations banner (C 3523-3527),
the RIF hourglass pair `ns_show_hourglass` (C 3511-3515) and `ns_hide_hourglass` (C 3517-3521) -
whose whole bodies are their TODO comments - `ns_get_glyph_string_clip_rect` (C 3529-3539),
`ns_draw_underwave` (C 3553-3591), `ns_draw_dash` (C 3598-3613), `ns_fill_underline`
(C 3620-3652) and the text-decoration block's `ns_draw_text_decoration` (C 3655-3807) - **no new
C export** (all seven are C `static`s) - so the covered range is now **C 60-3807**.  **Round 18
lands C 3809-4177**: `ns_draw_box` (C 3809-3839), `ns_setup_relief_colors` (C 3841-3869),
`ns_draw_relief` (C 3871-4023), `ns_dumpglyphs_box_or_relief` (C 4026-4089) and
`ns_maybe_dumpglyphs_background` (C 4091-4177) - **no new C export** (all five are C `static`s) -
so the covered range is now **C 60-4177**.  **Round 19 lands C 4179-4393**, the image sub-block:
`ns_draw_image_relief` (C 4179-4264) and `ns_dumpglyphs_image` (C 4266-4393) - **no new C export**
again (both are C `static`s) - so the covered range is now **C 60-4393**.  **Round 20 lands
C 4396-5001**: the stretch glyph (`ns_draw_stretch_glyph_string`, C 4396-4545), the three
foregrounds (`ns_draw_glyph_string_foreground` C 4547-4564,
`ns_draw_composite_glyph_string_foreground` C 4567-4655 and
`ns_draw_glyphless_glyph_string_foreground` C 4658-4741), the GC setter (`ns_set_glyph_string_gc`,
C 4747-4780) and the RIF's `ns_draw_glyph_string` (C 4782-5001) - **no new C export** (all six are
C `static`s) - so the covered range is now **C 60-5001**.  The boundary is the `Event loop` banner
at C 5005-5009: the dispatcher is the Glyph drawing operations section's last definition (the
section the ledger has walked since round 17), and the next live definition - C 5013's
`ns_send_appdefined` - is the callee the two held `EV_TRAILER` macro boxes already wait for.  **Round 21 lands C 6225-6400, the `EmacsApp` class's first five methods**, ahead of C 5005-6224: `ns_send_appdefined` (C 5013) reaches `app->nextappdefined` and `-sendFromMainThread:` through an `EmacsApp *` cast, so the class is what the event loop's own round needs (`## ABI` (4)'s entry/definition split).  The remaining twenty-five methods stay open.  **Round 22 lands C 5012-5084's `ns_send_appdefined` with C 448-470's `EV_TRAILER`/`EV_TRAILER2`** - the two macro boxes round 2 held open now close - plus `EmacsApp`'s `timeout_handler:` (C 6769) and `sendFromMainThread:` (C 6778), the two methods whose bodies call it; `check_native_fs` (C 5088) stays held on an `EmacsView` method.  The covered range is **C 60-5001 plus C 5012-5084 and C 6769-6781**.  **Round 23 lands `EmacsApp`'s four notification methods** (C 6556-6613: `antialiasThresholdDidChange:` C 6556, `systemWillSleep:` C 6565, `systemDidWake:` C 6577 and `applicationSupportsSecureRestorableState:` C 6610), so the range becomes **C 60-5001, C 5012-5084, C 6556-6613 and C 6769-6781**.  C 5109-5395's `ns_read_socket_1`/`ns_select_1` pair stays held on `check_native_fs`, i.e. on `EmacsView`'s class - the two `#if defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MAX_ALLOWED >= 1070` guards at C 5126 and C 5217 are the live arm, so the dependency is hard.  **Round 24 lands `EmacsWindow`'s record** (`src/nsterm.h` 419-430's four Cocoa ivars) with `borderWidth` (C 9928), `setFrameTopLeftPoint:` (C 10444), `canBecomeKeyWindow` (C 10452), `canBecomeMainWindow` (C 10457) and `setLastDragEvent:` (C 10463) - the type `EmacsView`'s own `nonfs_window` ivar needs, so that class's record is the next round's.  **Round 25 lands `EmacsView`'s record** (the fifteen ivars of `src/nsterm.h` 470-488) with `fsIsNative` (C 8952), `isFullscreen` (C 8957), `updateCollectionBehavior` (C 8981) and the `check_native_fs` (C 5086-5107) they were blocking - so C 5109-5395's `ns_read_socket_1`/`ns_select_1` pair is unblocked for the next round.  The class's protocol list (nsterm.h 466) is that round's named deferral: Swift enforces those protocols' required methods where the conformance is declared, so it lands with that batch.  **Round 26 lands C 5203-5385** - `ns_select_1` (C 5203), `ns_select` (C 5366) and `ns_run_loop_break` (C 5376), the three the unblocked dependency was for - so the covered range becomes **C 60-5001, C 5012-5084, C 5203-5385, C 6556-6613 and C 6769-6781**, and C 5109-5199's `ns_read_socket_1`/`ns_read_socket` is the pair that remains (it calls `-openFile:`/`-fulfillService:withArg:`, whose `NS_NONKEY_EVENT` value is the item that round must settle first).

Ticked above: **109 of the 150 function boxes**.  The count is the generator's own again: the
pre-3.260 `tools/cinventory.py` could not see **eight** of this file's definitions (the walk from a
declarator to its `{` stopped at the documentation comment in between - status 3.260), the port had
already written seven of them (round 5's two, round 6's `ns_focus`, round 10's four), and the
eighth - `ns_get_shifted_character`, C 2693 - landed in round 12.  Also
**17 of the 23 macro boxes** (round 18's four macro entries - `WINDOW_RIGHT_EDGE_X`,
`NS_FACE_FOREGROUND`, `FONT_HEIGHT`, `FONT_TOO_HIGH` - are other files' headers, so nothing here
moves; the eight masks,
`NO_APPDEFINED_DATA`, the five `EV_*` accessors, `DOCK_IGNORE_LIMIT`, the two
`NS_PARENT_WINDOW_*` and round 10's `NSAppKitVersionNumber10_10`; the two held `EV_TRAILER*` and
the four later ones stay open), **49 of the 49 file-scope-object boxes**
(the generator's 39 minus its gaps, the 8 hand-added round 1 rows, round 5's two interval
constants, round 6's `bell_view` and round 15's `fringe_bmp` - **the file's last open object
box**), **1 of the 1 type box** (`EmacsMargins`,
C 650) and **2 of the 8 Objective-C classes** (`NSColor (EmacsColor)` with its four methods, and
**`EmacsBell`** - the file's first *class*, with its `@interface` declaration and all four
methods).  The other 41 function boxes, 6 macro boxes, 6 class boxes and 185 method boxes are
later slices; round 7 added five rows to the `## ABI` (4) table - three class methods and, for the
first time, two of this unit's own *functions* - and round 10 added two more, both class methods
(`EmacsWindow_initWithEmacsFrame`, `setAppearance`).

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
  are the Swift spelling of the platform half, and round 12's `import Carbon` is C 72-73's
  `#include <Carbon/Carbon.h>` (the unit's first non-AppKit framework, Decisions).
* C's **function-scope** statics become file-scope Swift storage (round 12's `static char value[16]`
  at C 2673 and `static UInt32 dead_key_state` at C 2703 - the file's only two, and the reason is
  that Swift has no function-scope storage; the reference's storage is stable for the process's life
  and never freed either).  They land as `get_keysym_name_value` (the C name `value` cannot carry
  file scope) and `dead_key_state` (C's name).  The generator's `## File-scope objects` section does
  not list function-scope statics - they are not file-scope objects in C - so the checklist carries
  them in the two functions' boxes and here, never as object boxes.

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

### Round 3: the startup hooks, and the two `const char *` lifetimes

**`ns_relocate`'s arm.**  The `#ifdef NS_SELF_CONTAINED` block is **live**: `configure.ac`
689-690 gives the option the default `yes` and 2885-2886 `AC_DEFINE`s the macro, so the bundled
build this ledger describes has it; the `--disable-ns-self-contained` build compiles the same
function with the block removed.  That dead arm is recorded here, not transcribed (P-51), and
`nsterm.m`'s `check-cond-eval` state (`[NOT VERIFIED]`, round 1) is not an obstacle: the ruling
rests on the build system's own lines, which do not need the macro table.

**`const char *` out of autoreleased objects, twice.**  C hands out pointers into an
`NSString`/`NSString(format:)` that is *autoreleased* - valid until the enclosing pool drains -
in `ns_relocate` (returns `[fixedPath UTF8String]`) and in `ns_init_locale` (uses
`[localeID UTF8String]` for the rest of the function).  ARC releases a local at scope end, so the
unit keeps C's lifetime explicitly with `Unmanaged.passRetained(x).autorelease()`: the object is
retained and immediately autoreleased, which is exactly the pair C's `+pathWithComponents:` /
`+stringWithFormat:` perform, and the returned pointer stays valid as long as C's does.  The
generated code calls `objc_autorelease` (visible in `nm -u`, Evidence), so this is the real
runtime path and not a naming accident.  `ns_init_locale`'s *other* source of `lang` is
`getenv`, whose storage is static; C's `default_lang` literal is static too, and the unit's
file-scope `NSString` is its storage of the same class (a local Swift `String` or a
`withCString` scope would die before the `setenv` below runs).

**`NSAutoreleasePool` becomes the runtime pair.**  `ns_init_pool` (C 560-568),
`ns_alloc_autorelease_pool` (C 622-628) and `ns_release_autorelease_pool` (C 632-638) are C's
pool lifecycle; ARC has no `NSAutoreleasePool` (round 1's `outerpool` decision), so they are
`objc_autoreleasePoolPush`/`objc_autoreleasePoolPop` - the two entries `@autoreleasepool`
compiles to, which round 1 measured in libobjc.  The push/pop pair is the pool's own primitive,
and C's `ns_release_autorelease_pool`'s `ns_release_object (pool)` is the same `[(id)pool
release]` a pop is.

**`retain`/`release` under ARC.**  C's `[(id)obj retain]` / `[(id)obj release]` (C 602-616) are
unavailable to Swift ARC as written; the port performs the same reference-count operations
through the runtime's own entries - `Unmanaged.fromOpaque(obj).release()` consumes the +1 the
reference consumes, and `.retain()` performs the +1 `ns_retain_object`'s caller owns.  Both keep
C's nil-receiver behaviour (a NULL argument does nothing).  `nsselect.swift` 215 already imports
this export's exact signature (`(_ obj: UnsafeMutableRawPointer?)`), so the two units agree.

**No new owed symbols.**  Round 3's imports are all library entries - `objc_autoreleasePoolPush`,
`objc_autoreleasePoolPop`, `objc_autorelease`, `objc_retain`, `objc_release` (libobjc) and
`fprintf`, `getenv`, `setenv`, `setlocale` (libSystem) - so `## ABI` (3)'s list does not grow.

**`NSTRACE` is empty.**  `ns_init_locale`'s `NSTRACE ("ns_init_locale")` expands to nothing when
`NSTRACE_ENABLED` is 0 (src/nsterm.h 297's `NSTRACE` → 302's empty `NSTRACE_WHEN`), so it carries
no statement in the port either - the same dead arm round 1 recorded for C 96-149.

### Round 4: the screen geometry, and the forward reference the rest of the file needs

**`EmacsMargins` is measured, not assumed.**  The C side (`clang`, CoreGraphics'
`CGFloat` = 8 bytes) gives `sizeof 32`, `align 8` and the four members at 0/8/16/24; the Swift
side's `MemoryLayout` gives the same four numbers, so the view is the record's layout and not a
guess (`## Evidence`).

**`ns_auto_hide_menu_bar` is a `DEFVAR_LISP`, not a `DEFVAR_BOOL`.**  C 11697 is
`DEFVAR_LISP ("ns-auto-hide-menu-bar", ns_auto_hide_menu_bar, …)` with `ns_auto_hide_menu_bar =
Qnil` at 11700, so the import is a `Lisp_Object` data symbol and `ns_menu_bar_should_be_hidden`
reads it with `NILP` - the same data-symbol shape the briefing asked for, under the type the
reference actually gives it.

**Two selector/type readings, both measured.**  `[NSApp respondsToSelector:@selector
(setPresentationOptions:)]`: Swift exposes that setter as a property, so the port tests
`#selector(setter: NSApplication.presentationOptions)` - the method-reference spelling does not
compile (`error: type 'NSApplication' has no member 'setPresentationOptions'`).  And
`ns_frame_scale_factor` returns C's `double` built from `CGFloat` by C's implicit conversion,
which Swift needs spelled (`Double(…)`, measured error otherwise).

**`row->y`/`row->visible_height` are member entries.**  `ns_row_rect` is this unit's first read of
a *record member* no macro names, and the lane's rule for that is already written down in
`docs/ports/nsmenu.swift` 311-328: "the frame members this file reads or writes directly and that
no C macro or INLINE function names … each access is a C-ABI entry named after the member".  So
`glyph_row_y`/`glyph_row_visible_height` join that family (owed by the core), and the record
needs no Swift view - only those two members are read.

**The forward reference, which decides how the rest of the file is transcribed.**
C 1001's `ns_constrain_all_frames` calls `[view isFullscreen]` - a method of this unit's own
`EmacsView` (C 8957), a class that lands 8,000 lines later in the same file.  Swift resolves the
*name* file-wide, but the class has to exist in the compiled file, and this lane's rule is that
the mirror compiles at every slice, so a free function cannot land before the class it messages.
Roughly 71 such call sites sit in C 1001-6000.  The two candidate shapes:

* **(a) the classes land early.**  Transcribe `EmacsApp`/`EmacsView`/`EmacsWindow`
  (C 6225-10610) as whole Swift classes before the free functions that message them.  That
  abandons C order, and the classes' own bodies call the free functions back, so the order
  becomes a judgement call at every boundary.
* **(b) selector-named, receiver-first entries** - the shape *other units already use for this
  unit's methods*: `nsfns.swift` 390-393 imports `setMiniwindowImage`, `fullscreenState` and
  `setFSValue`, and `nsselect.swift` 223 imports `beginDrag`; all four are
  `EmacsView`/`EmacsWindow` methods of *this* file, so this unit is already committed to
  defining selector-named entries with the receiver first.  Under (b) the free functions call
  those entries, each entry's body lands beside its class, and C order survives.

**This lane recommends (b)**, and this round's report asks for the ruling before C 1001.

### Round 5: the ruling's entry/definition split, and the Accessibility Zoom block

**The forward reference is settled by an entry/definition split.**  The integrator's ruling:
where a free function messages a class that lands later in this same file, the call site declares
a C-ABI entry of the selector's name, receiver first, under a **different Swift name**
(`isFullscreen_entry`), and the definition - `@_cdecl("isFullscreen")` under the selector's own
Swift name - lands beside the class.  Two Swift functions, one C symbol, so the file compiles at
every slice; the alternative (the same name declared and later defined) is `invalid
redeclaration`, measured in round 1.  `## ABI` (4) is the table the ruling asks for, with the
first call site, the class each entry waits for, and its state; the acceptance for a row is that
`nm -u` stops listing its name (this round's `nm -u` still shows `_isFullscreen`, which is
correct - `EmacsView` lands at C 7039).

**(a) was rejected for the reason the integrator gives**: landing `EmacsApp`/`EmacsView`/
`EmacsWindow` (C 6225-10610) early abandons C order, and their bodies call the free functions
back, so every boundary becomes a judgement call.

**`FOR_EACH_FRAME` is spelled, not imported.**  C 1011's loop macro becomes `var tail =
Vframe_list; while CONSP (tail) { … tail = XCDR (tail) }`, the spelling
`docs/ports/nsfns.swift` 967 already uses for the same macro - a loop cannot be a C-ABI entry, so
the lane writes the macro's own text.

**The UAZoom block's library reaches.**  `UAZoomEnabled`/`UAZoomChangeFocus` need
`import ApplicationServices` (HIServices' `UniversalAccess.h`), and the SDK's
`kUAZoomFocusTypeInsertionPoint` imports as `Int` while the parameter is
`UAZoomChangeFocusType` (`UInt32`), so the call spells the conversion.  `CGMainDisplayID`/
`CGDisplayBounds` come from CoreGraphics, `clock_gettime_nsec_np (CLOCK_UPTIME_RAW)` from
libSystem, and `NSAccessibilityPostNotification` is Swift's
`NSAccessibility.post(element:notification:)` (the C name is replaced, measured).

**One warning, recorded.**  `swiftc -c` reports `'lockFocus()' was deprecated in macOS 10.14` for
C's own `[view lockFocus]` (C 1085); the port keeps the call, and the warning is quoted in
`## Evidence`.  It is not silence-worthy: the lane's landed `nsimage.swift` compiles with four
warnings of its own (measured).

### Round 6: the file's first class, and the first member write

**`EmacsBell` is the first class of the file** (C 1333-1430), and it lands *inside* its C order
with no deferred entries: its methods are called only from `ns_ring_bell` (C 1436) and from its
own body, both after the class.  The three `@interface` ivars become stored properties; the
methods are `@objc` so their selectors are reachable (`show:` schedules `hide` with
`#selector(hide)`), the reading `docs/ports/nsmenu.swift` 849 already uses; and C's `- (id)init`
becomes `override init(frame:)` with `required init?(coder:)` - AppKit's designated initializer
is `-initWithFrame:`, and `[[EmacsBell alloc] init]` is a zero frame (the reading
`docs/ports/nsmenu.swift` 843 uses for the same Swift requirement).

**`MOUSE_HL_INFO (f)->mouse_face_defer = 0` is the first member *write*.**  The member is
src/dispextern.h 3020's `bool_bf mouse_face_defer : 1` - a **bitfield**, which no Swift view can
write - so it takes the member-entry route, named `<record>_set_<member>` exactly as
`docs/ports/nsmenu.swift` 311-328 names its writes (`ns_output_set_in_animation`), and taking
C's own argument (the frame, like `frame_left_pos`).

**`NSRectClip`/`NSRectClipList` are AppKit's own C symbols that Swift hides.**  They are
`APPKIT_EXTERN` in NSGraphics.h 210-211, so the lane reaches them by name - the same treatment
`docs/ports/nsfont.swift` gives `fprintf`.  (The citation is an `external` row in
`check-citations.py`: counted, never a failure.)

**Three Swift-name readings, all measured.**  `NSBeep ()` is `NSSound.beep ()`;
`NSImageNameCaution` is `NSImage.cautionName`; and `[[EmacsBell alloc] init]` needs the frame
parameter, because a view subclass's designated initializer is `init(frame:)`.

**Eight deprecation warnings, recorded.**  `swiftc -c` prints four `lockFocus()` and four
`unlockFocus()` deprecations for C's own calls (C 1085, 1219, 1220, 1249, 1300, 1302 and the two
in the bell's neighbours); the port keeps every call, and the lane's landed `nsimage.swift`
compiles with four warnings of its own (measured) - so a recorded deprecation is ordinary here.

**`hide_bell` is dead in this build.**  Its guard (C 1471) is `#if !defined (NS_IMPL_COCOA) ||
MAC_OS_X_VERSION_MIN_REQUIRED < 101400`, and this lane's target defines `NS_IMPL_COCOA` and has a
minimum of 260000; its single call site (C 9388) sits inside the same guard.  The box is ticked
with that reason rather than transcribed (P-08).

### Round 7: the entries table grows, and the ruling reaches functions

**Every frame/display-info member this block touches is a member entry**, named per
`docs/ports/nsmenu.swift` 311-328 (and the `ns_display_info_set_*` family `nsfns.swift` already
uses): `ns_display_info_ns_focus_frame`, `ns_display_info_highlight_frame`,
`ns_display_info_set_highlight_frame`, and the frame members `frame_no_accept_focus`,
`frame_want_fullscreen`, `frame_left_pos`/`frame_top_pos` (the last two under the names
`nsmenu.swift` already uses).  Macros and INLINEs keep their own names as entries
(`FRAME_DISPLAY_INFO`, `FRAME_FOCUS_FRAME`, `fset_focus_frame`, `FRAME_LIVE_P`, `FRAME_VISIBLE_P`,
`SET_FRAME_VISIBLE`, `SET_FRAME_ICONIFIED`, `FRAME_NO_FOCUS_ON_MAP`, `FULLSCREEN_NONE`, `FRAMEP`,
`XSETFRAME`, `check_window_system`, `gui_update_cursor`).

**The ruling's shape now covers functions too.**  `ns_set_offset` (called at C 1691, defined at
C 1854) and `ns_set_frame_alpha` (called at C 1616/1619, defined at C 2366) are this unit's own
functions whose call sites precede their definitions - and C **itself** forward-declares the
latter at C 1622.  They take the entry/definition split (`ns_set_offset_entry` /
`@_cdecl("ns_set_offset")` at C 1854), and `nsfns.swift` 600 already imports `ns_set_offset` with
that signature, so the two units agree.

**Three new exports.**  `ns_make_frame_visible`, `ns_make_frame_invisible` and `ns_iconify_frame`
(src/nsterm.h 1226-1228) are the frame hooks the redisplay interface calls, so they keep C's names
under `@_cdecl`.

### Round 10: the frame-parameter handlers, the color block, and the second function row closed

**The generator loses four of this slice's fifteen definitions.**  *(Corrected in round 11: this
paragraph first read the loss as an eight-line lookahead window; status 3.260 measured the real
rule.)*  `tools/cinventory.py`'s `function_spans` walks from the end of a parameter list to the
line that carries `{`, and it stopped at the first line that was neither dead, nor blank, nor a
preprocessor line - which for a documented function is **its comment**.  Emacs writes
`ns_set_undecorated (…)` / `/* … */` / `{`, so the definition was dropped whole, and the
eight-line window around the walk made that look like a length limit (the longer the comment, the
more reliably it was lost: `ns_set_no_focus_on_map`'s 7-line comment stayed inside the window,
`ns_set_undecorated`'s 8-line one did not).  The fix (status 3.260) skips comment lines and raises
the bound to 64, and this file's generator count goes 142 -> **150**: the four boxes above are
ordinary generator rows now, and so are the other four names that stop hid - `ns_focus`,
`ns_ua_zoom_enabled_p`, `ns_cg_rect_flip_y` and C 2693's still-open `ns_get_shifted_character`.
Because the stop hid the four, `docs/baseline/swift-units.tsv` never held rows for them, so this
round's regeneration request to the integrator asked for **deletions only** (the eleven names that
did have rows and now match a Swift `func`); 3.260's one residue, `ns_get_shifted_character`, was
the row to *add*, and the integrator wrote it in `aa3e13a`.

**C's assignment-inside-a-condition has no Swift spelling, and the port keeps the
short-circuit.**  Three sites: `ns_set_parent_frame`'s `!FRAMEP (new_value) || !FRAME_LIVE_P (p =
XFRAME (new_value)) || !FRAME_NS_P (p)` (C 2026-2029) becomes three steps, so `XFRAME` still runs
only when `FRAMEP` failed to reject and `FRAME_NS_P` only when `FRAME_LIVE_P` did; and
`ns_get_color`'s two `(new = [NSColor selectedText…]) != nil` (C 2215/2230) become the assignment
plus an `if`.  Swift has no assignment expression, and hoisting any of them would evaluate a
macro C does not evaluate on that path - PORTING.md 0.3's "语句与顺序" (one C statement is one
statement, and the definition/`#if` order does not change) is the rule the shape preserves.

**`ns_get_color` reassigns its `const char *name` parameter twice, and its colour-list block
shadows `name` with an `NSString *`.**  Swift's `String` holds `[nsname UTF8String]`'s bytes, so
the port keeps `nsname` alone and reads the pointer's two uses (C 2243's `parse_color_spec`
through `withCString`, C 2252's `name[0]` through `utf8.first`) from it; C's inner `NSString *name`
(C 2270) is the `for name in clist.allKeys` binding, the same shadow.  `NS_SELECTION_BG_COLOR_DEFAULT`
and `NS_SELECTION_FG_COLOR_DEFAULT` (src/nsterm.h 1278-1279) are **not** entries: their bodies are
string literals, so they are spelled in place - class B of the 3.255 ruling, the reading
`FRAME_NS_TITLEBAR_HEIGHT`/`FRAME_TOOLBAR_HEIGHT` already carry.  `NSNormalWindowLevel` (C
2095/2100/2106/2111) is the AppKit rename `NSWindow.Level.normal`, and C's `+ 1`/`- 1` become that
level's `rawValue` arithmetic; `NSAppKitVersionNumber10_10` (C 2139) is
`NSAppKitVersion.macOS10_10`, the rename the `NSColor (EmacsColor)` category above already uses for
the 10_7 pair.

**The slice's entries are the 3.255 classes, and the member writes take the shim's own naming.**
The six `frame_set_<member>` writes follow `femacs/src/abi_shim.md`'s existing export names
(`frame_set_border_width`, `frame_set_tab_bar_resized`, …), and `nsfns.swift` 573/577 already
import `frame_set_undecorated`/`frame_set_ns_transparent_titlebar`, so the two units agree
name for name.  The enumerator families, the seven globals and the header-only accessors stay
class A: their values are the core's facts (an enumerator's number, a generated global's storage),
so `femacs/src/abi_shim.rs` exports each name.

**`Emacs_Color` is a Swift `struct` of this unit, and its layout is measured.**  C's
`typedef struct { unsigned long pixel; unsigned short red, green, blue; } Emacs_Color`
(src/dispextern.h 49-54) crosses into `ns_defined_color` and
`ns_query_frame_background_color`; clang reads `sizeof 16, align 8, pixel 0, red 8, green 10,
blue 12`, and the Swift side reads `size 14, stride 16, align 8` with the same four offsets -
Swift does not count the tail padding in `size`, and `stride` (16) is the storage unit C's
`sizeof` names (Evidence).  `@_cdecl` refuses `UnsafeMutablePointer<Emacs_Color>` ("Swift structs
cannot be represented in Objective-C", measured this round), so the exported
`ns_defined_color` takes `UnsafeMutableRawPointer?` and re-binds it - the shape round 2's
`ns_init_events` set for the same refusal.

**`ns_set_frame_alpha` (C 2366) is the definition the (4) table's second function row waited
for.**  The call sites C 1612/1617 could only take an entry because C itself forward-declares the
function at C 1579; with the definition landed under `@_cdecl("ns_set_frame_alpha")` the symbol is
real and `nm -u` no longer lists `_ns_set_frame_alpha`.  C's `dpyinfo->highlight_frame == f` is
read through round 7's `ns_display_info_highlight_frame` entry (which takes the *frame* the
`FRAME_DISPLAY_INFO` macro starts from), so the local `dpyinfo` has no separate reading, and
`f->alpha[0]`/`[1]` is the member entry `frame_alpha`.

**Two new deferred rows.**  `ns_set_undecorated` constructs an `EmacsWindow` (C 1983) and
`ns_set_appearance` sends it `setAppearance` (C 2144); the class lands at C 9756, so both call
sites take entries of the `handleFS` shape - `EmacsWindow_initWithEmacsFrame` (the name
`nsfns.swift` 695's `EmacsView_initFrameFromEmacs` sets) and `setAppearance`.

**One new warning, of C's own kind.**  `swiftc -c` reports `'scanFloat' was deprecated in macOS
10.15` three times for `ns_get_color`'s three `[scanner scanFloat: &…]` calls (C 2257-2259) - the
same reading as the `lockFocus`/`unlockFocus` pair the ledger's Evidence already records, so the
port keeps the calls.

### Round 11: the whole Mouse handling section, and the `FRAME_NS_VIEW` question the integrator raised

**Scope and shapes.**  Four definitions, C 2409-2621, all of them this file's hooks for the
redisplay interface: `frame_set_mouse_pixel_position` (C 2408-2430, exported since src/frame.h 1882
declares it), and the three `static`s `ns_note_mouse_movement` (C 2432-2472), `ns_mouse_position`
(C 2475-2595, C's `mouse_position_hook`) and `ns_frame_up_to_date` (C 2598-2621, the
`frame_up_to_date_hook`).  The entries they need are the families rounds 5-10 settled - per-member
entries, same-name entries for globals/enumerators/header-only accessors, receiver-first entries
for the two classes' messages - and this round adds two *shapes* to those families:
`ns_display_info_last_mouse_glyph` is a **pointer** member entry (C takes the member's address at
C 2448, so every `r->field` access stays in Swift, the way an `NSRect`-typed record member should),
and `is_emacs_tooltip` is a class-kind test that belongs to `src/nsmenu.m`'s port (C 2534's
`isKindOfClass: [EmacsTooltip class]`), the mirror of `nsfns.swift` 691's `is_emacs_view`.

**`FRAME_NS_VIEW` stays an entry this round** - the integrator's note offered the class-B route
for the `ns_output_*` names (declare this file's `struct ns_output` layout prefix, expand
`f->output_data.ns->view` in Swift, and change the two expanded macro bodies' call sites with it).
The slice uses `FRAME_NS_VIEW` exactly once (C 2576), and the unit already reads it as an entry in
twenty-seven places, two of them the `frame_ns_titlebar_height`/`frame_toolbar_height` bodies
rounds 9-10 recorded; taking the route for one call site would leave two shapes for one macro, and
the whole-file conversion is a slice of its own (the frame's own path to `output_data` is the part
the lane has kept behind entries - `frame_output_data_ns`, C 1732's site - so the B route needs
*both* that prefix and the `ns_output` one).  The ruling's criterion is unchanged and the entry is
class A by it: the expansion needs a layout fact the core owns.  If the integrator wants the
conversion, it is the 27 call sites plus the prefix declaration in one go, and this paragraph is
the record that it was offered and deferred rather than missed.

**`remember_mouse_glyph` is the gated entry of this slice.**  Its declaration (src/dispextern.h
3586) is live, but its *definition* (src/xdisp.c 2779) sits inside `#ifdef HAVE_WINDOW_SYSTEM`
(opened at src/xdisp.c 2488), which the frozen configuration leaves undefined: the objects have no
`_remember_mouse_glyph` (measured), and the same guard explains the other non-static xdisp.c names
missing from `build/emacs-ref/src/xdisp.o` (`x_y_to_column_row`, `get_glyph_string_clip_rect*`,
`get_phys_cursor_geometry`).  This is exactly the status 3.255 ③ shape: the ratchet keeps the row
with the reason that only a window-system build can satisfy it, and the shim exports it under the
same gate.  `note_mouse_highlight` is the opposite case: the reference defines it (`T` in
`build/emacs-ref/src/xdisp.o`), so its row is an ordinary core debt that `xdisp.rs`'s landing
clears.

**`track_mouse` has no symbol in the reference, and the core already carries one.**  C
2532/2539/2542/2554/2565 compare against it, and the generated `globals.h` (852-853) spells it
`globals.f_track_mouse`: `nm` over `build/emacs-ref/src/*.o` finds no `_track_mouse` (the only hit
is `_Finternal_track_mouse`), so the import has to be a data symbol of its own name - the reading
`Vframe_list`/`Vframe_alpha_lower_limit` already take - and **the ported core already defines it**
(`nm -g build/femacs/*.o` shows `S _track_mouse`, from `keyboard.rs`'s own writes through the
flattened `globals.rs`).  So this one needs no shim work at all; the ledger records it because the
*name* has no reference-side symbol to point at.

**C's implicit conversions, spelled.**  Three of them are in this slice and each keeps C's own
truncation: `ns_note_mouse_movement`'s `CGFloat x`/`y` land in `note_mouse_highlight`'s and
`remember_mouse_glyph`'s `int` parameters (`Int32(x)`), and `ns_mouse_position`'s
`lrint (view_position.x)` is the **double** overload (`CGFloat` is `double` on this target), so the
port writes `lrint (Double(view_position.x))`.  `XSETINT` is an *lvalue* macro
(`((a) = make_fixnum (b))`, src/lisp.h 1341), so its entry takes the assignable `Lisp_Object` first -
`withUnsafeMutablePointer`-free here because C's `*x`/`*y` are already pointers - the `XSETFRAME`
shape `nsfns.swift` 711 uses.  Two smaller spellings: C's `id view;` at the top of
`ns_mouse_position` becomes a `let` inside the one block that uses it (C declares it early and
assigns at C 2576), and `do { … } while (…)` is Swift's `repeat { … } while (…)` (whose
`continue` re-checks the condition, as C's does).

**`fputs`/`stderr`.**  C's null-`*fp` warning is `fputs (…, stderr)`; `fputs` takes the lane's
`@_silgen_name` shape (Swift marks the stdio family unavailable - `docs/ports/nsfont.swift` 1178's
comment), while `stderr` is the C library overlay's own declaration, which this unit has read since
round 3 (C 569-600).

**No new warning.**  `swiftc -parse-as-library -c` is exit 0 with the same seven warnings the
previous round carried (four `lockFocus`/`unlockFocus`, three `scanFloat`) - this slice adds none.

### Round 12: the cursor hook and the Keyboard handling prologue, and where the `ns_output` family stands

**Scope.**  Six definitions, C 2624-2761: `ns_define_frame_cursor` (C 2624-2637, the RIF
`define_frame_cursor_hook`), then the Keyboard handling banner (C 2641-2645) and its prologue -
`ns_convert_key` (C 2648-2663), **`get_keysym_name` (C 2666-2677), the slice's one export**
(src/lisp.h 5436 declares it; every platform's `*term.c` carries the definition, and for this lane
it lives here), `right_mod` (C 2680-2684), `nil_or_none` (C 2686-2690) and
`ns_get_shifted_character` (C 2692-2761, the name the 3.260 reader fix surfaced).

**`current_pointer` takes the family's member names, and `FRAME_NS_VIEW` still waits for its own
slice.**  `FRAME_POINTER_TYPE (f)` is `f->output_data.ns->current_pointer`, and the shim's batch 6
had already fixed this member's **write** entry to the family name - `abi_shim.md` 96 cites
`ns_output_set_current_pointer` with the C text `FRAME_POINTER_TYPE (f) = cursor;` *nsterm.m 2634*,
this slice's own line - so the read half is `ns_output_current_pointer` (the sibling of the
fourteen reads beside it) and the pair is what `ns_output_text_cursor`/`ns_output_set_text_cursor`
already look like.  A macro-name entry for `FRAME_POINTER_TYPE` would have been the other option;
the family wins because the shim has already spoken for this member, and mixing the two shapes for
one member is exactly what the ledger tries not to do.  That same reasoning is *why* the round-11
deferral of `FRAME_NS_VIEW` is now recorded more sharply: the shim's family already holds
`ns_output_view` (round 11's report described the conversion as needing an `ns_output` layout
prefix; it does not - it is a rename of this unit's 27 call sites to the member entry, plus the four
other units that import the macro name).  It stays a slice of its own because a rename inside one
unit would leave the tree with two shapes for one member; this is the second round that has looked
at it, and the next round that needs an `ns_output` member should schedule it before writing that
member's call sites.

**`import Carbon` is a new dependency of this unit.**  C 70-80's `NS_IMPL_COCOA` arm is
`#include <Carbon/Carbon.h>` (C 72-73) plus C's own note about ApplicationServices and the
UniversalAccess pair (C 74-79, which round 5 already obeys for the UAZoom block).
`ns_get_shifted_character` is the first function in this file that needs the Carbon half -
`TISCopyCurrentKeyboardLayoutInputSource`/`TISGetInputSourceProperty`/`UCKeyTranslate`/
`LMGetKbdType` and the four key masks - and ApplicationServices alone does not bring them
(measured: the same probe fails to resolve `TISCopyCurrentKeyboardLayoutInputSource` without the
module).  The port therefore adds `import Carbon` at the top of the unit, which makes the eventual
Cocoa link line carry `-framework Carbon` - the mirror of C's own include, recorded because it is
the first non-AppKit framework this unit names.

**`get_keysym_name`'s two substitutions.**  C's `static char value[16]` is a **function-scope**
static; Swift has no function-scope storage, so it is a file-scope buffer of the same size,
allocated once and never freed (C's address is stable for the process's life and C never frees it
either), zero-initialised to match C's static storage.  `snprintf (value, 16, "%d", keysym)` is
variadic and unavailable in Swift (the reading `docs/ports/nsfns.swift` 3698 gives), so the port
builds the same string with `String(format:)` and copies it into the same buffer, keeping C's
16-byte bound through `min (bytes.count, 16)`.

**Two ownership readings.**  `TISCopyCurrentKeyboardLayoutInputSource` follows the Create rule:
C takes the +1 and never releases it, while Swift's `takeRetainedValue()` gives ARC the release -
same observable behaviour, and the lane's standard spelling for a Create-rule function.
`TISGetInputSourceProperty`'s result is a **+0** `CFTypeRef`, so the port *borrows* it
(`Unmanaged<CFData>.fromOpaque(…).takeUnretainedValue()`, the reading `macfont.swift` 1541 uses) -
and that borrowing is also what removed the round's one compiler warning: `layout_ref as! CFData`
on the property's implicitly-unwrapped pointer draws `treating a forced downcast to 'CFData' as
optional will never produce 'nil'`, which the explicit `fromOpaque` does not.

**The small spellings.**  `countof (convert_ns_to_X_keysym)` is the round-1 table's `count` (and
the two reads index it with `Int(keysym)` because C's `unsigned` index becomes a Swift `Int`);
C's `for (keysym = 0; …; keysym += 2)` keeps its shape as the same `while` loop; `dead_key_state`
(C 2703) is the second function-scope static of the slice and lands the same way as `value`;
`UniChar` is `UInt16` and Swift's SDK has no `UniCharCount`, so C's `UniCharCount` variables are
`Int` (which is also what `UCKeyTranslate`'s Swift signature wants); the Carbon key masks are
anonymous-enum constants, so each is `UInt32 (…)` exactly where C's arithmetic would convert it;
`FRAME_POINTER_TYPE (f) != cursor` compares **pointers**, so the port writes `!==` (identity) rather
than letting `NSObject`'s `isEqual:` in; and `[event modifierFlags]`'s `NSUInteger` truncates into
C's `UInt32 flags`, the reading the round-1 `EV_*` accessors already carry.

**No new warning.**  `swiftc -parse-as-library -c` is exit 0 with the same seven the previous round
left (four `lockFocus`/`unlockFocus`, three `scanFloat`), after the one `CFData` cast warning above
was removed by the borrow.

### Round 13: block drawing (frame clearing and scrolling), and the record families it starts

**Scope.**  Four definitions, C 2789-2960: `ns_clear_frame` (C 2788-2820, exported - src/nsterm.h
1124 declares it), `ns_clear_frame_area` (C 2823-2846), `ns_scroll_run` (C 2849-2916) and
`ns_clear_under_internal_border` (C 2919-2960).  C 2771-2786's `ns_redraw_scroll_bars` sits inside
`#ifdef NS_IMPL_GNUSTEP` and is dead for this lane (recorded in the section banner, not
transcribed); the `#if defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MAX_ALLOWED < 101400` arm inside
`ns_scroll_run` (C 2910-2912) is dead for the same reason (the SDK's value is 260000).

**Three record families start here: `struct face`, `struct window` and `struct run`.**  The unit
already read `struct frame` members through `frame_<member>`/`FRAME_*` entries and
`struct ns_display_info`/`Mouse_HLInfo`/`struct glyph_row` the same way; this round adds
`NS_FACE_BACKGROUND` (src/nsterm.h 1041's `((f)->background)`, the macro's own name as the entry),
`window_frame` (the `window_<member>` family, read at C 2855) and the three `struct run` members
(`run_current_y`/`run_desired_y`/`run_height`, read at C 2865-2874), plus the two face-id lookups
(`FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL`) and the three ids (`DEFAULT_FACE_ID`,
`CHILD_FRAME_BORDER_FACE_ID`, `INTERNAL_BORDER_FACE_ID`).  **The status-3.271 gated-layout probe
was not needed for any of them**: every access is a member entry over a pointer this unit gets
from an entry, which is the lane's rule - a *view* of a gated record would be the other route (the
`input_event`/`Emacs_Color` cases, where the unit itself holds the record), and `struct face` only
exists under `HAVE_WINDOW_SYSTEM` besides.  The probe's reading is nevertheless recorded in
Evidence, since the integrator asked for it: it is the lane's route for the gated members the
`FRAME_*` entries stand for.

**`FRAME_TOP_MARGIN_HEIGHT`/`FRAME_BOTTOM_MARGIN_HEIGHT` keep their names.**  Their bodies
(src/frame.h 1139/1151) sum `FRAME_MENU_BAR_HEIGHT`/`FRAME_TAB_BAR_HEIGHT`/
`FRAME_TOOL_BAR_TOP_HEIGHT` and `FRAME_TOOL_BAR_BOTTOM_HEIGHT`, which are themselves macros over
`struct frame` members and `Qtop`/`Qbottom` comparisons.  Expanding them here would owe four member
entries (`menu_bar_height`, `tab_bar_height`, `tool_bar_height`, `tool_bar_position`) where one
name is owed, and the ruling's class-A criterion applies either way - the expansion needs core
layout facts.  This is the case `FRAME_NS_TITLEBAR_HEIGHT`/`FRAME_TOOLBAR_HEIGHT` is *not*: those
bodies are pure AppKit arithmetic on the view, which is why they are the expanded sample.

**`gui_clear_cursor` is the round's gated entry.**  Declared in src/dispextern.h, defined at
src/xdisp.c 35466 **inside `#ifdef HAVE_WINDOW_SYSTEM`** (opened at src/xdisp.c 35045): the frozen
objects have no `_gui_clear_cursor` (measured, and the same guard is what makes the whole
`gui_*`/`dump_glyph*` family absent from `xdisp.o`).  Same shape as `remember_mouse_glyph`'s row
(status 3.255 ③): the shim exports it under the same gate.  `Vface_remapping_alist` is the
opposite reading - a generated global (no symbol of its own in the *reference*), yet the core's
flattened table already exports it, so the row is not open at all.

**`NSRectFill` is reached by name, `NSIntersectionRect` is not.**  Swift marks parts of AppKit's C
graphics API unavailable and leaves others alone: `NSIntersectionRect` (C 2838) is called directly,
while `NSRectFill` (C 2813, 2842, 2952-2957) needs the `@_silgen_name` shape round 6 used for
`NSRectClip`/`NSRectClipList` - a per-function SDK property, so each is a reading of its own.

**`copyRect:to:` is an `EmacsView` method, so C 2909 is a deferred entry.**  The selector is this
file's own (C 9340, inside `@implementation EmacsView`), and it is the first *multi-segment*
selector this lane has had to reach: the entry takes the **first selector segment** as its name
with the receiver first and every argument after it, the shape `docs/ports/nsselect.swift` 220-223
gives `beginDrag:forPasteboard:withMode:returnFrameTo:prohibitSame:followTooltip:`.

**No new warning.**  `swiftc -parse-as-library -c` is exit 0 with the same seven warnings the
previous rounds carry (four `lockFocus`/`unlockFocus`, three `scanFloat`).

### Round 14: the fringe update, the shift, and the first held record view

**Scope.**  Three definitions, C 2963-3079, and **no new C export**: all three are C `static`s
(`ns_after_update_window_line` C 2963-3019, `ns_shift_glyphs_for_insert` C 3022-3036,
`ns_compute_glyph_string_overhangs` C 3047-3079 - with the "Character encoding and metrics" banner
at C 3040-3044).  `ns_shift_glyphs_for_insert` adds *no* new name at all: its copy is round 13's
`copyRect` entry.

**`font_metrics` is the unit's second held record, and the status-3.271 probe read it.**  C declares
`struct font_metrics metrics;` as a *local* and hands `&metrics` to the core's driver slot, which
fills it through the pointer; C then reads `rbearing`/`width`/`lbearing`.  That is the
`Emacs_Color`/`input_event` situation (the unit owns the storage), so the unit declares the view -
the same five `short`s `macfont.swift` 49-60 and `femacs/src/font.rs` 196 already carry.  Its layout
is *measured*, and this round is the first to use the gated-arm probe the integrator landed in
3.271: `spikes/gated-layout-probe.py 'struct font_metrics' font.h` reads **frozen**, x11 and
ns_cocoa alike as `sizeof=10, align=2` with `lbearing 0, rbearing 2, width 4, ascent 6, descent 8`
- this record parses in the frozen arm too (`parse=no` is the NS-only types' row), which is why the
reading did not need an arm at all.  The probe was run read-only; its `--write` target is a
`docs/baseline/**` table, which this lane does not touch.

**`font->driver->text_extents (…)` is two entries, not a view.**  C walks a `struct font` member
(`driver`) and then a `struct font_driver` slot (`text_extents`, a function pointer at ~offset 104
of that table).  The lane's rule for a member of a *core* record is an entry, so the port reads the
two hops as `font_driver` and `font_driver_text_extents` and calls what the second returns - the
second's type is the `@convention(c)` closure `macfont.swift` 11160-11162 declares for the same
slot, so the two units agree on the ABI.  (The alternative, declaring `struct font`'s and
`struct font_driver`'s prefixes here, would repeat ~40 members of a table this file never builds.)
C calls the slot unconditionally; the port's `if let` is the nil check that C's driver contract
makes unnecessary, and it is the same shape as the file's optional chaining elsewhere.

**C's comma expressions and `eassert`.**  C's two conditions are
`(f = XFRAME (w->frame), width = FRAME_INTERNAL_BORDER_WIDTH (f), width != 0)` and
`(height = desired_row->visible_height, height > 0)`; Swift has neither the comma operator nor an
assignment expression, so the three pure reads move into the body (the same short-circuit - C does
not evaluate them either when the earlier conjuncts fail), and the two locals keep C's names.
`eassert (w)` is the empty macro unless `ENABLE_CHECKING` is defined, which this configuration
leaves off (the `NSTRACE_ENABLED` reading), and `max (0, desired_row->y)` is class B of the 3.255
ruling: its body is arithmetic, so it becomes `Swift.max` rather than an entry.

**A declared-but-uncalled entry never reaches the object.**  This round's first written form
declared both the `glyph_string_*` overhang **reads** and their writers; the object's `nm -u` then
listed only the writers, because nothing called the readers and the compiler drops unused
`@_silgen_name` declarations whole.  The port therefore declares only what its slice calls (the
unit's own rule: "it declares only what its own slice reads"), and the reads will arrive with the
slice that reads them - the finding is recorded because it is also the mechanism that keeps an
unused entry from inflating the import ratchet.

### Round 15: the fringe bitmaps, and one entry per C spelling

**Scope.**  C 3083-3291: the Fringe and cursor drawing banner, the file's last file-scope object
`fringe_bmp` (C 3089) and three definitions - `ns_define_fringe_bitmap` (C 3091-3195),
`ns_destroy_fringe_bitmap` (C 3198-3202), `ns_draw_fringe_bitmap` (C 3205-3291).  **No new C
export**: all three are C `static`s, so this round moves the export count not at all.

**One entry per C spelling, and this round is where the two-spelling cases land.**  C reads the
same storage through a macro in one place and through the bare member in another, and the entries
follow C's own text rather than collapsing the two: `WINDOW_FRAME (w)` (src/frame.h 929's
`((w)->frame)`) is the macro's entry while the bare `w->frame` stays round 13's `window_frame`;
`face_foreground`/`face_background` are the `struct face` member entries for C 3254/3279/3281's
bare reads, while `NS_FACE_BACKGROUND` (src/nsterm.h 1041's own macro) stays the macro's entry used
where C uses *it*; and `f->output_data.ns->cursor_color` (C 3283) takes the family name
`ns_output_cursor_color` that the shim already exports.  PORTING.md 0.2 asks for names verbatim, so
each of C's spellings keeps a nameable entry and the port's text stays greppable against the
reference - the alternative (calling `NS_FACE_BACKGROUND` where C wrote `face->background`) would
have been a *text* deviation in the other direction, with the same number of owed names either way
once both spellings appear in the file.

**`alloca`, the copy/release pair, and the trace's `char`s.**  C 3099's
`alloca ((h + 1) * (w + 1) * 4)` becomes `withUnsafeTemporaryAllocation(of: UInt8.self,
capacity:)` - the lane's reading for `alloca` (`macfont.swift` 1684/2978) - so the `points`/`cur`
walk keeps C's shape over the temporary buffer.  C 3274/3288's `bmp = [bmp copy]; … [bmp release];`
is ARC's own copy (no release call: the reading rounds 3/6 record for `[x release]`), and
`[bmp transformUsingAffineTransform: transform]` needs the Swift spelling
`transform(using: transform as AffineTransform)` - the one compiler-named correction this round's
probe found before the compile.  `ns_define_fringe_bitmap`'s state machine carries C's `char`
letters (`'n'`/`'e'`/`'s'`/`'w'`) as `CChar`s, with C's truthiness tests written out
(`last != 0`, `nw == 0 && ne != 0`).

**Two names from a file no configuration compiles.**  `gui_define_fringe_bitmap` (src/fringe.c
1853) and `max_used_fringe_bitmap` (src/fringe.c 497) have **no symbol in the reference objects**,
because the frozen configuration compiles no `fringe.c` at all - `femacs/src/fringe.rs` is the
empty unit status 3.31 records, quoting the build's empty platform object variables.  Both are
therefore owed entries (the function's row carries the gate the file itself has, the
`remember_mouse_glyph`/`gui_clear_cursor` family), and the global is a data symbol of the core's.

### Round 16: the cursor hooks, and the first batch whose members are gated *in the header*

**Scope.**  C 3294-3508: `ns_draw_window_cursor` (C 3295-3426, the RIF cursor hook),
`ns_draw_vertical_window_border` (C 3430-3449) and `ns_draw_window_divider` (C 3453-3508).  **No
new C export** - all three are C `static`s - so the round moves the export count not at all.

**The `struct window` cursor members are gated in `window.h` itself.**  `phys_cursor_type`,
`phys_cursor_on_p`, `phys_cursor_width` and the nested `phys_cursor.hpos` live inside src/window.h
340-350's `#ifdef HAVE_WINDOW_SYSTEM` block, so the frozen `struct window` has no such fields and
no built object can answer them: the five entries are owed **under that gate**, which is the first
time this lane's gated debt comes from a *header* block rather than a `.c` file.  The gate does not
change the shape (member entries, `<record>_<member>` with `_set_` for the three writes), only the
row's reason - status 3.262's "the core does export it, but only behind `feature = …`: no
configuration this repository builds turns that arm on" wording, plus what would re-open it (a build
that defines `HAVE_WINDOW_SYSTEM`, i.e. the Cocoa target this whole lane is written for).

**Three gated/dead functions, each with its own reason.**  The hook reaches
`get_phys_cursor_geometry` (src/xdisp.c 2707) and `draw_phys_cursor_glyph` (src/xdisp.c 35099) -
both inside `HAVE_WINDOW_SYSTEM` (the `#ifdef`s at 2488 and 35045), measured absent from
`xdisp.o` - and `draw_fringe_bitmap` (src/fringe.c 838), whose *file* no built configuration
compiles at all (`femacs/src/fringe.rs` is the empty unit status 3.31 records, the same one round
15's two names came from).  `get_phys_cursor_glyph` is the contrast: src/window.c 8228, no gate,
`T` in `window.o`, and the port's `femacs/src/window.rs` 8078 already exports it - a plain import.

**Three spellings re-shaped, each recorded.**  C's `(phys_cursor_glyph = get_phys_cursor_glyph (w))
== NULL` is the assignment-in-condition shape again (hoisted, the same short-circuit); the
parameter `cursor_width` is assigned twice inside the body (`max (FRAME_CURSOR_WIDTH (f), 1)` and
the R2L adjustment), so the port shadows it with a local of the same name because Swift's
parameters are constants; and C's `switch` has no `default:` (it covers `enum text_cursor_kinds`
exactly), so the port's `default: break` is the arm Swift needs for an `Int32` subject.  `max` is
round 14's class-B expansion (arithmetic), and `lrint (0.25 * h)` is the
`lrint (0.25 * Double(h))` the earlier `lrint` uses needed.

**The Objective-C side's readings.**  C's `NSGraphicsContext *ctx = [NSGraphicsContext
currentContext];` is `NSGraphicsContext.current` (an optional, so the save/restore pair is
`ctx?.…` - the nil receiver C relies on); the `NS_IMPL_GNUSTEP` arm (`GSRectClipList`) is dead and
the live one is the existing `NSRectClip` entry; `[NSBezierPath strokeRect:…]` is Swift's
`NSBezierPath.stroke(_:)`; and the `NS_IMPL_COCOA` block at C 3378-3386 writes the global round 5
already landed as `ns_UAZoom_cursor_rect_new`.

### Round 17: the text-decoration block, and a macro kept class A *because of a predicate*

**Scope.**  C 3511-3807: the hourglass pair (C 3511-3521), the Glyph drawing operations banner
(C 3523-3527), `ns_get_glyph_string_clip_rect` (C 3529-3539), `ns_draw_underwave` (C 3553-3591),
`ns_draw_dash` (C 3598-3613), `ns_fill_underline` (C 3620-3652) and `ns_draw_text_decoration`
(C 3655-3807).  **No new C export** - all seven are C `static`s - so `## ABI` (1) is unchanged.

**`NativeRectangle` is `NSRect` for this lane, and the comment says so.**  src/dispextern.h 119's
`#define NativeRectangle XRectangle` is inside the `HAVE_X_WINDOWS` arm; the Cocoa build falls to
src/nsgui.h 80's `#define NativeRectangle NSRect`, which is exactly what
`ns_get_glyph_string_clip_rect`'s own comment means by "nr should be pointer to two successive
NSRects".  The C signature's `int` return keeps C's `int`, and the body's
`get_glyph_string_clip_rects (s, nr, 2)` passes both arguments through unchanged.

**Two more gated imports, both from the same class of debt.**  `get_glyph_string_clip_rects`
(src/xdisp.c 2495) and `font_for_underline_metrics` (src/xdisp.c 32256) sit inside xdisp.c's
`#ifdef HAVE_WINDOW_SYSTEM` blocks (opened at 2488 and 30717), so the frozen configuration leaves
both undefined (measured: **0** symbols each in `build/emacs-ref/**/*.o`).  Their rows carry the
gate and the condition that would re-open them - a build that defines `HAVE_WINDOW_SYSTEM`, i.e.
the Cocoa target this whole lane is written for (status 3.262's shape, round 16's
`get_phys_cursor_geometry` reading).

**`WINDOW_BUFFER_LOCAL_VALUE` is the first macro this lane keeps as class A *because of a
predicate*.**  Its body - `(BUFFERP ((W)->contents) ? buffer_local_value(V, (W)->contents) :
Qnil)` (src/window.h 633-636) - needs the `BUFFERP` predicate, the `buffer_local_value` function
and `Qnil`: the 3.255 A criterion ("only the core knows it") applies in full, and unlike the
layout-family macros there is no way to spell the read from a record this file declares (the
`struct window` contents slot and the buffer-local machinery are the core's).  So the name stays a
`@_silgen_name` entry and the shim exports it, with the entry taking the window pointer C's `W`
is - the shape `WINDOW_FRAME` already has.

**The four `Q*` symbols split in half.**  `Qunbound` and `Qunderline_minimum_offset` are **real
symbols** of the reference (`S` in `build/emacs-ref/src/emacs.o`), i.e. provided;
`Qx_underline_at_descent_line` and `Qx_use_underline_position_properties` are this file's own
`DEFSYM`s (C 11770/11748) and so are **open**, the `QCmouse` reading of round 2.

**Five spellings re-shaped, each recorded.**  C's `unsigned long` locals (`thickness`, `position`,
`descent`, `minimum_offset`, `h`, `dy`) stay `UInt` and every narrowing back to the `int`
parameters is `truncatingIfNeeded:` - C's implicit conversion, the lane's rule; `lround
(font->descent / 2)` keeps C's **integer** division before the `lround` (the reason the port does
not write `lround(font->descent / 2.0)`); the strike-through's `lrint ((glyph_height - h) / 2)`
likewise keeps the unsigned integer division `h` forces; `ns_fill_underline`'s `switch` lists the
two abort cases and the `default` as one Swift `default` (Swift cannot name `default` beside a
case), with both enumerators still called; and `[defaultCol set]` reads through the optional
receiver C relies on (`defaultCol?.set()`), as does `NSGraphicsContext.current` in
`ns_draw_underwave`.  On the ObjC side, `+strokeLineFromPoint:toPoint:` is Swift's
`NSBezierPath.strokeLine(from:to:)`, `-setLineDash:count:phase:` is `setLineDash(_:count:phase:)`
(AppKit's own renames, checked against the compiler), and `[[NSBezierPath alloc] init]` /
`[path release]` is `NSBezierPath()` under ARC.  **One recorded caveat**: the arithmetic on those
`UInt` locals is Swift's (`position - thickness - 1`, `descent - thickness - …`), and Swift
*traps* on unsigned underflow where C wraps - the same "not executed, so not exercised" class of
divergence every pixel-arithmetic site in this lane carries, kept rather than papered over with
`&-` (no macOS Swift unit in this tree uses the wrapping operators).

**The two hourglass hooks have no body to port.**  C's `ns_show_hourglass`/`ns_hide_hourglass` are
`TODO` comments inside empty functions; the port keeps the comments and the same empty bodies, so
the boxes are ticked as *recorded*, not converted.

**One stale sentence in `## Progress` is corrected.**  The round-5 generation note still said
`ns_get_shifted_character` (C 2693) was "an open box here until its slice lands"; round 12 landed
it, so the sentence now reads "landed in round 12".

### Round 18: the box/relief sub-block, the background helper, and the family's first *operation* row

**Scope.**  C 3809-4177: `ns_draw_box` (C 3809-3839), `ns_setup_relief_colors` (C 3841-3869),
`ns_draw_relief` (C 3871-4023), `ns_dumpglyphs_box_or_relief` (C 4026-4089) and
`ns_maybe_dumpglyphs_background` (C 4091-4177).  **No new C export** - all five are C `static`s -
so `## ABI` (1) is unchanged; the slice declares **44** `@_silgen_name` names and all 44 are
referenced in the built object (Evidence).

**C's `char` boolean flags are `Bool`, and the reference itself is the reason.**  `ns_draw_box`,
`ns_draw_relief` and `ns_maybe_dumpglyphs_background` spell their flags `char`, while the sibling
`ns_draw_image_relief` (C 4183) spells the identical set `bool raised_p, top_p, bot_p, left_p,
right_p`; every `char` here is only tested with C truthiness or copied to another flag, and Swift
has no truthiness, so the port types them `Bool` and records it.  The call sites follow: C's
`ns_draw_relief (r, abs (hthickness), abs (vthickness), s->face->box == FACE_RAISED_BOX, 1, 1,
left_p, right_p, s)` reads `… FACE_RAISED_BOX (), true, true, left_p, right_p, s`.

**`glyph_advance` is the member family's first *operation* row.**  `ns_dumpglyphs_box_or_relief`
walks `struct glyph` pointers (`s->first_glyph + s->nchars - 1` at C 4062, the row walk's
`…glyphs[s->area] + …used[s->area]` at 4053, `g++` at 4055/4058 and `g < end` at 4056), and
`struct glyph` is a core record with no Swift view in this lane: the stride is the core's fact, so
the advance cannot be spelled at the use site and the class-B half of the 3.255 ruling does not
apply.  The entry takes the pointer and the count C's expression has and returns C's pointer type;
`g < end` is Swift's `<` on `UnsafeMutableRawPointer` (its `Strideable` conformance supplies
`Comparable`), with the two nil guards the optional pointer type needs - C's own comparison against
a NULL `end` is false for the non-NULL `g` the loop holds, which is what the guards reproduce, and
a `g` walking off a NULL base is a case C does not survive either.

**The stipple arm reaches outside this unit three times.**  `ns_maybe_dumpglyphs_background`'s
`NS_IMPL_COCOA` arm is the live one (its `#else`'s `goto fill;` and the `fill:` label are the
GNUstep text, recorded not transcribed - a `goto` has no Swift form and no live reader).
①`dpyinfo->bitmaps[face->stipple - 1].img` is a two-level member entry whose second argument is
C's own subscript (the `glyph_row_used` shape); ②`EmacsImage` is another lane's class
(`nsimage.swift`'s - the `@interface` at src/nsterm.h 670-703), so `- (CGImageRef)stippleMask`
crosses as a `<Class>_<selector>` entry, the `EmacsImage_allocInitFromFile` shape `nsfns.swift`
402 declares - and **no provider exists yet**, so the (3) table registers it as owed; ③the five
CoreGraphics calls keep their C names as entries (the round-5 `CGMainDisplayID`/`CGDisplayBounds`
shape) because `CoreGraphics.apinotes` renames all five in Swift.  `CGRectMake` is the one
expression of the arm that does **not** become an entry: it is a `static inline` of CGGeometry.h
with no symbol, so the port spells the rect with Swift's own initializer (class B).

**The reference's own duplicated block is kept.**  `ns_draw_relief` tests
`vthickness > 1 && left_p` twice (C 3980-3986 and C 3988-3994); the second is what a `right_p`
test would mean, but "照录不转写" is the lane's rule for a live statement, so both blocks are
transcribed verbatim - the C text is the reference's, not the port's to repair.

**The slice's four macro names are audited, and two of them are already ratcheted.**  The
macro-name audit's rule applies to `WINDOW_RIGHT_EDGE_X`, `NS_FACE_FOREGROUND`, `FONT_HEIGHT` and
`FONT_TOO_HIGH`: each has **0** symbols in `build/emacs-ref/{src,lib}/*.o` (measured this round),
so each is a header-only name; `NS_FACE_FOREGROUND` and `FONT_HEIGHT` are already imported by
`macfont.swift`, so only `WINDOW_RIGHT_EDGE_X` and `FONT_TOO_HIGH` are new class-A rows, and
`window_box_right` is the slice's one name with a reference-side symbol (`T` in
`build/emacs-ref/src/xdisp.o`) - which is why its row reads *provided*.

### Round 19: the image sub-block, the family's first *address-of* row, and macros kept class B

**Scope.**  C 4179-4393: `ns_draw_image_relief` (C 4179-4264) and `ns_dumpglyphs_image`
(C 4266-4393).  **No new C export** - both are C `static`s - so `## ABI` (1) is unchanged, and the
two function boxes close (103 of 150).  The boundary is C 4396's `ns_draw_stretch_glyph_string`: it
opens the stretch-glyph block (the stretch's own background and its `char2b` foreground), a
different subject from the image's drawing, and the two boxes here are the image *pair* - the
"coherent sub-block" rule of round 18.

**`struct image` is read through seven member entries, and its two pointer members are `id`.**
`pixmap`/`mask` are `Emacs_Pixmap`, which the NS arm `typedef`s to `id` (src/nsgui.h 55-59), so
those two reads come back `AnyObject?` - the `ns_display_info_bitmaps_img` reading of round 18.
`width`, `height`, `relief`, `hmargin` and `vmargin` are `int` (src/dispextern.h 3304-3342) and
stay `Int32`.  The whole record sits inside that header's `HAVE_WINDOW_SYSTEM` block, which the
frozen configuration does not define, so the rows are owed under the same gate the round-13/16
member rows carry.

**`s->slice` needs four reads *and* an address-of row.**  Its four `unsigned : 16` members
(src/dispextern.h 455-460) promote to `int` in C's expressions, so
`glyph_string_slice_x`/`_y`/`_width`/`_height` keep the promoted `Int32` value (the
`glyph_u_cmp_id` reading of round 18).  `&s->slice` is a fifth name, `glyph_string_slice_ptr`:
`image_ascent`'s third parameter is a `struct glyph_slice *` and the record's layout is the core's
fact, so the address cannot be spelled at the use site - the family's second *operation* row after
`glyph_advance`, for the same reason.

**`image_ascent` is defined in a file no configuration here builds.**  It is declared at
src/dispextern.h 3774, inside the header's `HAVE_WINDOW_SYSTEM` block (opened at 3727), and defined
at src/image.c 1911; src/image.c is not in `docs/baseline/frozen-build-sources.txt`, and its port
(`femacs/src/image.rs`) is an empty unit.  No object holds the symbol, so the row is a shim work
order rather than a "provided" reading - the class round 17's two gated xdisp.c functions record.

**Two macros are *spelled*, not imported - the (B)-first half of the 3.255 ruling.**
`#define DEFAULT_TAB_BAR_BUTTON_RELIEF 1` at src/dispextern.h 3436 and
`#define DEFAULT_TOOL_BAR_BUTTON_RELIEF 1` at 3515 expand to a literal, and src/lisp.h 5597's
expression macro `eabs` to arithmetic; both are expressible, so the use site spells them
(file-scope `let`s for the two names, `eabs` inline) and no shim is owed - the exception
`FRAME_NS_TITLEBAR_HEIGHT`/`FRAME_TOOLBAR_HEIGHT` is.  (2) records them as **not entries**.  One
sharp edge is recorded with the spelling: the inline `eabs` negates an `Int32`, so it traps on
`Int32.min` where C's negation is undefined - the class round 17's unsigned-arithmetic note already
carries, and nothing in this tree exercises it.

**C's `intmax_t` globals narrow at the assignment.**  The two bar thicknesses are generated
`globals` fields (`intmax_t f_tab_bar_button_relief;`, globals.h 942-943, the `DEFVAR_INT` at
src/xdisp.c 38973; `intmax_t f_tool_bar_button_relief;`, 944-945 / 38979), so the reads are `Int`
and `min (…, 1000000)`, the `< 0` tests and the `extra_x`/`extra_y` arithmetic narrow C's way with
`Int32(truncatingIfNeeded: …)` - the lane's narrowing rule (round 17's six `unsigned long` locals).
The two margins are `Lisp_Object`s (src/xdisp.c 38965/38985).  None of the four has a reference
symbol (they are `globals` fields), so all four are class-A rows of the core's shim - the
`windows_or_buffers_changed` shape.

**C's comma statements and chained assignments are spelled as the statements they are.**  C's
`x -= thick + extra_x, left_p = true;` and its three siblings (C 4253-4260) are the two statements
each; C's `extra_x = extra_y = XFIXNUM (…) - thick;` (C 4235/4248) is two assignments; and C 4251's
`top_p = bot_p = left_p = right_p = false;` is the four assignments Swift needs (the flags are
initialized at their declaration, so the statement is kept for C's text).  C's uninitialized locals
(`int x1, y1, thick;`, `int bg_x, bg_y, bg_height;`, `NSColor *tdCol;`) take the round-18 zero
defaults.

**Four readings of the image body, three of them the compiler's own.**
① C's `NSRect r` parameter (C 4267) is **unused in the reference's own body** - the port keeps it,
and Swift warns about neither an unread parameter nor an unread `let`.
② `[NSAffineTransform transform]` is Swift's `NSAffineTransform()` (round 15) and
`translateXBy:yBy:` is `translateX(by:yBy:)`, but C 4343's `-appendTransform:` is **renamed**
`append(_:)` - this round's first compile reported
`error: 'appendTransform' has been renamed to 'append(_:)'`, and the fix is the compiler's own
name; the value crosses the Foundation `SwiftBridge` into the `AffineTransform` parameter with the
`as AffineTransform` cast round 15 uses for `transform(using:)`.
③ The two interpolation settings are the Swift property: `setImageInterpolation:` with
`NSImageInterpolationNone` is `imageInterpolation = .none`, `NSImageInterpolationDefault` is
`.default`, and the compositing operation's case is the prefix-stripped
`NSCompositingOperation.sourceOver`.
④ `[img size].width`/`.height` stay two reads of the `EmacsImage_size` entry, exactly the two
messages C 4324 sends.

**Four cross-lane `EmacsImage` names, all owed to `nsimage.swift`.**  The class's `@public` ivars
`transform`/`smoothing` (src/nsterm.h 680-681) are not C symbols and this unit has no Swift view of
the class, so they cross as `EmacsImage_transform`/`EmacsImage_smoothing`; NSImage's own `-size` and
`-drawInRect:fromRect:operation:fraction:respectFlipped:hints:` (NSImage.h 133) cross as
`EmacsImage_size`/`EmacsImage_drawInRect` - receiver first, the `EmacsImage_stippleMask` shape of
round 18 and the `EmacsTooltip_*` family (`nsfns.swift` 945-953).  The gate files all four as
`unclassified` (Evidence), and (3) carries the work order.

### The macro-name audit (status 3.251's rule)

`nm -g build/emacs-ref/{src,lib}/*.o` was searched for every macro/INLINE name this unit imports
as an entry: **`FRAME_NS_VIEW`, `FRAME_DISPLAY_INFO`, `MOUSE_HL_INFO`, `SELECTED_FRAME`,
`WINDOW_TO_FRAME_PIXEL_Y`, `FRAME_NS_TITLEBAR_HEIGHT`, `FRAME_TOOLBAR_HEIGHT`, `NILP`, `CONSP`,
`XCAR`, `XCDR`, `EQ`, `SYMBOLP`, `FRAME_PARENT_FRAME`, `SET_FRAME_VISIBLE`, `FRAME_VISIBLE_P`,
`FRAME_LIVE_P`, `FRAME_FOCUS_FRAME`, `fset_focus_frame`, `FRAME_NO_FOCUS_ON_MAP`, `XSETFRAME`,
`FRAMEP`, `reset_mouse_highlight`, `free_frame_menubar` have zero symbols**, while the true
functions have them (`change_frame_size` 4, `xfree` 49, `emacs_abort` 37, `free_frame_faces` 3,
`window_box` 3).  So every macro-name row in `## ABI` (2)-(4) is a name the **core** has to
export as a shim (the lane's rule for header-only constructs, and what
`docs/baseline/swift-import-pending.txt` tracks); none of them can come from C or a library.

**The two computed macros are the exception that proves it**: `FRAME_NS_TITLEBAR_HEIGHT` and
`FRAME_TOOLBAR_HEIGHT` expand to AppKit arithmetic (nsterm.h 1062-1079), so they are spelled as
Swift bodies (`frame_ns_titlebar_height`/`frame_toolbar_height`) rather than imported, and no
shim is owed for them - the reading status 3.251 asks for (`fflush_unlocked`'s lesson: a macro
whose expansion is expressible must not become a symbol).

### Round 20: the stretch glyph, the three foregrounds, and the dispatcher that closes the section

**Scope.**  C 4396-5001: `ns_draw_stretch_glyph_string` (C 4396-4545), the three foregrounds -
`ns_draw_glyph_string_foreground` (C 4547-4564), `ns_draw_composite_glyph_string_foreground`
(C 4567-4655) and `ns_draw_glyphless_glyph_string_foreground` (C 4658-4741) - `ns_set_glyph_string_gc`
(C 4747-4780) and the RIF's `ns_draw_glyph_string` (C 4782-5001).  **No new C export** - all six are
C `static`s - so `## ABI` (1) is unchanged, and the six function boxes close (**109 of 150**).  The
boundary is the `Event loop` banner at C 5005-5009: the dispatcher is the last definition of the
Glyph drawing operations section the ledger has walked since round 17, and the next live definition
- C 5013's `ns_send_appdefined` - is the callee the two held `EV_TRAILER` macro boxes already wait
for.

**The slice declares 60 new `@_silgen_name` names** (this unit's C symbol imports go from 347 to
405), which the gate files as **30 `reference-source`, 27 `unclassified` and 1 `reference`**
(`cursor_in_mouse_face_p`), with `window_box_left`/`window_box_left_offset` **provided** by the
core's `xdisp.rs`; `FRAME_COLUMN_WIDTH`, `LGSTRING_GLYPH` and `XCHAR_TABLE` already carry ratchet
rows, so **55 new `swift-import-pending` rows are owed** (Evidence).  The slice adds **no**
cross-lane name: the stretch block re-uses round 18's `EmacsImage_stippleMask` and nothing else of
that class, so the five outstanding `EmacsImage_*` spellings stay five.

**The stretch block's five readings.**  ①`x_stretch_cursor_p` is a generated `globals` field
(`bool f_x_stretch_cursor_p;`, globals.h 1230-1231, the `DEFVAR_BOOL` at src/xdisp.c 38638), so it
is a class-A entry and the core owes it - the `tab_bar_button_relief` row of round 19.  ②C 4434
re-tests `s->hl == DRAW_CURSOR` inside an arm C 4402 has already established: the reference's own
duplication, transcribed as it stands.  ③The row's three new flags are this slice's members
(`reversed_p`, read three times; `mouse_face_p`, with `cursor_in_mouse_face_p`; and the
`stipple_p` read/write pair).  ④`MOUSE_HL_INFO (s->f)->mouse_face_face_id` is a two-level read
(`ns_display_info_bitmaps_img`'s shape) and `MOUSE_FACE_ID` is the fallback's face id.  ⑤The
`NS_IMPL_COCOA` stipple arm is the live one and is round 18's three lines again - `CGRectMake` is
the spelled initializer (class B, round 19), and the `#else`'s single `[[…stippleMask] set];` (C
4523) is recorded, not transcribed.

**The driver gains two slots, and both are two-hop reads.**  `draw`
(`int (*draw) (struct glyph_string *, int, int, int, int, bool)`, src/font.h 683-684) and
`encode_char` (`unsigned (*encode_char) (struct font *, int)`, src/font.h 667) are the second
`@convention(c)` slot view beside round 14's `font_text_extents_fn`, reached through the same
`font_driver` entry.  C calls `draw` unconditionally and drops its `int` at all eight call sites;
the port's `if let` is the nil check C's driver contract makes unnecessary, and each call is `_ =`
(`_ = composition_gstring_width`'s shape, round 14).

**The composite body's four new reads.**  `s->cmp->offsets[j * 2]` and `… [j * 2 + 1]` (C 4605-4606)
are the `short *offsets` member (src/composite.h 177) with C's own subscript, so it is one entry
returning the promoted `Int32` (`glyph_row_used`'s shape); `COMPOSITION_GLYPH`, `LGSTRING_GLYPH`
and the four `LGLYPH_*` abbreviations keep C's macro names as class-A entries; `s->face->overstrike`
(four reads) is the fourth.  C 4603's `'\t'` is spelled `9`, the character constant's value, and
the four `XFIXNUM`-derived `lglyph` reads are C's `ptrdiff_t`, narrowed at the `int` locals with
`Int32(truncatingIfNeeded:)` - round 17's narrowing rule.

**The glyphless body is the slice's one lifetime reading, and it changes the shape of the text.**
C's `NSGlyph char2b[8]` (`unsigned int` in this SDK, AppKit's NSFont.h 147) *is* the storage the
driver reads through `s->char2b`, so that pointer has to live from C 4673's `s->char2b = char2b;` to
C 4740's `s->char2b = NULL;`; Swift's array storage is only valid inside
`withUnsafeMutableBufferPointer`, so those two assignments and the loop between them sit in that
closure.  C's `char buf[8 + PACIFY_GCC_BUG_81401]` needs the same treatment for the hex-code arm,
because `str` points into it: the per-iteration body that reads `str[j]` sits in `buf`'s closure.
Three spellings are recorded with it - `snprintf (buf, 8, "%0*X", …)` (C 4708) is variadic and has
no Swift spelling (round 12's `get_keysym_name` reading), so the port writes the same uppercase
zero-padded hexadecimal conversion of width 4 or 6 into the same eight-byte buffer; C 4682's
`#ifdef GCC_LINT` arm (its 1) is a compiler this build does not use, and the live arm's 0 - eight
bytes - is what is transcribed; and `eassume (ch <= MAX_CHAR);` (C 4707) is src/lisp.h 182/193's
macro, `assume (cond)`, a compiler hint with no statement when `ENABLE_CHECKING` is off, which this
configuration leaves undefined (round 13's `eassert (w)` reading) - so it is recorded and `MAX_CHAR`
(src/character.h 49) is not imported with it.

**The char-table reads get a member entry instead of a struct view.**  `nsfont.swift` 50 and
531-532 declare that record's view and its own `XCHAR_TABLE`; this unit has no view of that record
(it is the core's), so `XCHAR_TABLE (Vglyphless_char_display)->extras[0]` crosses as
`char_table_extras (XCHAR_TABLE (…), 0)` - a member entry whose second argument is C's own
subscript 0, the `ns_display_info_bitmaps_img` shape of round 18.  `Vglyphless_char_display` is the
other generated `globals` field this slice reads (globals.h 296-297, the `DEFVAR_LISP` at
src/xdisp.c 39354).

**`ns_draw_glyph_string` is transcribed whole, and four of its shapes are worth naming.**  ①`NSRect
r[2]` is a two-element `[NSRect]` - `get_glyph_string_clip_rects` documents "pointer to two
successive NSRects", so the two slots are the reference's - and the entries that take it want the
element pointer: `&r` for the mutable one, the array itself for `ns_focus`.  ②`NSRect r` inside the
two overhang loops (C 4933 and C 4974) shadows the array's name exactly as C's does, and the
`NSRect rc;` + `get_glyph_string_clip_rect (s, &rc);` pair is the zero default plus the call; the
first loop guards its second clip with `if (n)` (C 4938) and the second does not (C 4979) - the
reference's own asymmetry, transcribed.  ③C's right-overhang loop mixes `next` and `s->next` in one
body (C 4805-4816); the port keeps both readings, because on the loop's first turn they are the same
pointer and the reference's own text tests `s->next` in the rest of the body.  ④The four
`NS_IMPL_GNUSTEP` arms (`DPSgsave` with two `DPSrectclip`s at C 4941-4945 and C 4981-4985, and
`DPSgrestore` at C 4952 and C 4992) are dead and recorded; the live
`saveGraphicsState`/`restoreGraphicsState` pair stays.  The two extra C block scopes in the
`CHAR_GLYPH`/`COMPOSITE_GLYPH` arm (C 4860 and C 4875) are the plain declarations here - no name
collides - and `case XWIDGET_GLYPH:` (C 4850-4851) reaches `x_draw_xwidget_glyph_string`:
src/xwidget.h 188's declaration, src/xwidget.c 2793's definition, and *no* object in the frozen
build (src/xwidget.c is not among `docs/baseline/frozen-build-sources.txt`'s files), so the (3)
table carries it as a shim work order.

**The overhang reads round 14 deferred land here.**  `glyph_string_left_overhang` and
`glyph_string_right_overhang` are declared by this slice (round 14 wrote the pair at C 3070-3077 and
recorded that their reads "get their own entries when that slice arrives"); the same is true of
`glyph_string_set_char2b` and `glyph_string_set_stippled_p`, which complete two read-only rows.

**No Swift API is renamed this round** - unlike round 19's `-appendTransform:` - so the ledger has
no rename to record.  The two platform spellings it re-uses are round 18's `ctx?.cgContext` and
round 19's `CGRect (x:y:width:height:)`; its one new stdlib spelling is `String (_:radix:uppercase:)`,
which replaces the recorded `snprintf`.

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
| `ns_relocate` | `extern const char *ns_relocate (const char *)` (src/nsterm.h 1195) | `func ns_relocate(_ epath: UnsafePointer<CChar>?) -> UnsafePointer<CChar>?` |
| `ns_init_pool` | `extern void ns_init_pool (void)` (src/nsterm.h 1171) | `func ns_init_pool()` |
| `ns_init_locale` | `extern void ns_init_locale (void)` (src/nsterm.h 1172) | `func ns_init_locale()` |
| `ns_release_object` | `extern void ns_release_object (void *)` (src/nsterm.h 1166) | `func ns_release_object(_ obj: UnsafeMutableRawPointer?)` |
| `ns_retain_object` | `extern void ns_retain_object (void *)` (src/nsterm.h 1167) | `func ns_retain_object(_ obj: UnsafeMutableRawPointer?)` |
| `ns_alloc_autorelease_pool` | `extern void *ns_alloc_autorelease_pool (void)` (1168) | `func ns_alloc_autorelease_pool() -> UnsafeMutableRawPointer?` |
| `ns_release_autorelease_pool` | `extern void ns_release_autorelease_pool (void *)` (1169) | `func ns_release_autorelease_pool(_ pool: UnsafeMutableRawPointer?)` |
| `ns_frame_scale_factor` | `extern double ns_frame_scale_factor (struct frame *)` (src/nsterm.h 1260) | `func ns_frame_scale_factor(_ f: UnsafeMutableRawPointer?) -> Double` |
| `ns_set_undecorated` | `extern void ns_set_undecorated (struct frame *, Lisp_Object, Lisp_Object)` (src/nsterm.h 1229) | `func ns_set_undecorated(_ f: UnsafeMutableRawPointer?, _ new_value: Lisp_Object, _ old_value: Lisp_Object)` |
| `ns_set_parent_frame` | the same shape (src/nsterm.h 1230) | `func ns_set_parent_frame(…)` |
| `ns_set_no_focus_on_map` | the same shape (src/nsterm.h 1231) | `func ns_set_no_focus_on_map(…)` |
| `ns_set_no_accept_focus` | the same shape (src/nsterm.h 1232) | `func ns_set_no_accept_focus(…)` |
| `ns_set_z_group` | the same shape (src/nsterm.h 1233) | `func ns_set_z_group(…)` |
| `ns_set_appearance_1` | `extern void ns_set_appearance_1 (struct frame *, Lisp_Object)` (src/nsterm.h 1240, `NS_IMPL_COCOA`) | `func ns_set_appearance_1(_ f: UnsafeMutableRawPointer?, _ new_value: Lisp_Object)` |
| `ns_set_appearance` | `extern void ns_set_appearance (struct frame *, Lisp_Object, Lisp_Object)` (src/nsterm.h 1241) | `func ns_set_appearance(…)` |
| `ns_set_transparent_titlebar` | `extern void ns_set_transparent_titlebar (struct frame *, Lisp_Object, Lisp_Object)` (src/nsterm.h 1242-1245) | `func ns_set_transparent_titlebar(…)` |
| `ns_lisp_to_color` | `extern int ns_lisp_to_color (Lisp_Object, NSColor **)` (src/nsterm.h 1148, `__OBJC__`) | `func ns_lisp_to_color(_ color: Lisp_Object, _ col: UnsafeMutablePointer<NSColor?>) -> Int32` |
| `ns_defined_color` | `extern bool ns_defined_color (struct frame *, const char *, Emacs_Color *, bool, bool)` (src/nsterm.h 1142) | `func ns_defined_color(_ f:, _ name: UnsafePointer<CChar>?, _ color_def: UnsafeMutableRawPointer?, _ alloc: Bool, _ _makeIndex: Bool) -> Bool` |
| `ns_set_frame_alpha` | `static void ns_set_frame_alpha (struct frame *)` (C 1579's forward declaration; the `set_frame_alpha_hook` at C 5941) | `func ns_set_frame_alpha(_ f: UnsafeMutableRawPointer?)` - **the (4) table's second function row, closed** |
| `frame_set_mouse_pixel_position` | `extern void frame_set_mouse_pixel_position (struct frame *, int, int)` (src/frame.h 1882) | `func frame_set_mouse_pixel_position(_ f: UnsafeMutableRawPointer?, _ pix_x: Int32, _ pix_y: Int32)` |
| `get_keysym_name` | `extern char *get_keysym_name (int)` (src/lisp.h 5436) | `func get_keysym_name(_ keysym: Int32) -> UnsafeMutablePointer<CChar>?` - the definition other platforms keep in their own `*term.c`; for the Cocoa build it is this unit's |
| `ns_clear_frame` | `extern void ns_clear_frame (struct frame *)` (src/nsterm.h 1124) | `func ns_clear_frame(_ f: UnsafeMutableRawPointer?)` |

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
| `objc_autoreleasePoolPush` | `void *objc_autoreleasePoolPush (void)` | objc/runtime (not Swift-visible) | libobjc |
| `objc_autoreleasePoolPop` | `void objc_autoreleasePoolPop (void *)` | objc/runtime (not Swift-visible) | libobjc |
| `fprintf` | `int fprintf (FILE *, const char *, ...)` | stdio.h | libSystem (`docs/ports/nsfont.swift` 1173-1177's reading) |
| `NILP` | `#define NILP(x) lisp_h_NILP (x)` | src/lisp.h 450 | header-only (`docs/ports/nsfns.swift` 176) |
| `FRAME_PARENT_FRAME` | `INLINE struct frame *FRAME_PARENT_FRAME (struct frame *)` | src/frame.h 1234 | header-only |
| `FRAME_NS_VIEW` | `#define FRAME_NS_VIEW(f) ((f)->output_data.ns->view)` | src/nsterm.h 1045 | header-only |
| `WINDOW_TO_FRAME_PIXEL_Y` | `#define WINDOW_TO_FRAME_PIXEL_Y(W, Y) ((Y) + WINDOW_TOP_EDGE_Y (W))` | src/window.h 1078 | header-only |
| `window_box` | `void window_box (struct window *, enum glyph_row_area, int *, int *, int *, int *)` | src/dispextern.h 3574 | src/window.c |
| `glyph_row_y`, `glyph_row_visible_height` | the record members `row->y`, `row->visible_height` | src/dispextern.h (the `struct glyph_row` members) | **open**: member entries, per `docs/ports/nsmenu.swift` 311-328 |
| `block_input`, `unblock_input` | `void block_input (void)` / `void unblock_input (void)` | src/blockinput.h | src/keyboard.c |
| `CONSP`, `XCAR`, `XCDR` | the list walkers `FOR_EACH_FRAME` is spelled with | src/lisp.h | header-only (`docs/ports/nsfns.swift` 179/191/192) |
| `Vframe_list` | `DEFVAR_LISP ("frame-list", …)` variable | src/frame.c | **open**: data symbol, the core's to export |
| `UAZoomEnabled`, `UAZoomChangeFocus` | `Boolean UAZoomEnabled (void)` / `OSStatus UAZoomChangeFocus (const CGRect *, const CGRect *, UAZoomChangeFocusType)` | ApplicationServices' `UniversalAccess.h` | ApplicationServices |
| `CGMainDisplayID`, `CGDisplayBounds` | CoreGraphics | `CGDirectDisplay.h` | CoreGraphics |
| `clock_gettime_nsec_np` | `uint64_t clock_gettime_nsec_np (clockid_t)` | time.h | libSystem |
| `NSRectClip`, `NSRectClipList` | `void NSRectClip (NSRect)` / `void NSRectClipList (const NSRect *, NSInteger)` | AppKit's NSGraphics.h 210-211 | AppKit (Swift marks them unavailable) |
| `mouse_highlight_set_mouse_face_defer` | the bitfield member `MOUSE_HL_INFO (f)->mouse_face_defer` (src/dispextern.h 3020) | src/dispextern.h | **open**: member-write entry, `<record>_set_<member>` per `docs/ports/nsmenu.swift` 311-328 |
| `visible_bell` | `DEFVAR_BOOL ("visible-bell", …)` variable | src/dispnew.c | **open**: data symbol, the core's to export |
| `SELECTED_FRAME` | `#define SELECTED_FRAME() …` | src/frame.h | header-only |
| `MOUSE_HL_INFO` | `Mouse_HLInfo *MOUSE_HL_INFO (struct frame *)` | src/frame.h 1028 | header-only |
| member entries (round 8) | `mouse_highlight_mouse_face_mouse_frame`, `ns_display_info_set_ns_focus_frame`, `ns_output_miniimage`, `frame_output_data_ns`/`frame_set_output_data_ns`, `frame_size_hint_flags`/`frame_set_size_hint_flags`, `XNegative`, `YNegative` | the records' members, per `docs/ports/nsmenu.swift` 311-328; a member entry takes the handle C's own expression starts from | **open**: the core's to export |
| `free_frame_menubar`, `free_frame_faces`, `reset_mouse_highlight`, `xfree`, `emacs_abort`, `change_frame_size` | core functions | src/frame.h 1883 / src/dispextern.h / src/lisp.h / src/dispnew.h | the core |
| `ns_output_zooming`, `ns_output_set_zooming` | the `struct ns_output` member `f->output_data.ns->zooming` | src/nsterm.h's `struct ns_output` | **open**: member entries |
| `FRAME_NS_TITLEBAR_HEIGHT`, `FRAME_TOOLBAR_HEIGHT` | **not entries** - the two computed macros (src/nsterm.h 1062-1079) are spelled as Swift bodies instead, per the rule of status 3.251 (see Decisions) | src/nsterm.h 1062-1079 | spelled in place |

| member writes (round 10) | `frame_set_undecorated` (C 1981), `frame_set_no_focus_on_map` (C 2056), `frame_set_no_accept_focus` (C 2074), `frame_set_z_group` (C 2096/2101/2107/2112), `frame_set_ns_appearance` (C 2123/2125/2127), `frame_set_ns_transparent_titlebar` (C 2162) | the `FRAME_*` macros' write halves, per `docs/ports/nsmenu.swift` 311-328 (`<record>_set_<member>`); the first two names are the ones `nsfns.swift` 573/577 already imports | **open**: the core's to export |
| enumerators (round 10) | `z_group_none`, `z_group_above`, `z_group_below`, `z_group_above_suspended` (src/frame.h 61-67) and `ns_appearance_system_default`, `ns_appearance_aqua`, `ns_appearance_vibrant_dark` (src/frame.h 70-75) | `enum z_group` / `enum ns_appearance_type` | **open**: the core's to export (class A of 3.255 - an enumerator's value is the core's fact) |
| globals (round 10) | `Qparent_frame`, `Qabove`, `Qabove_suspended`, `Qbelow`, `Qdark`, `Qlight`, `Vframe_alpha_lower_limit` | the `DEFSYM`/`DEFVAR_LISP` carriers (`build/emacs-ref/src/emacs.o` holds `_Qdark` and friends as `S`; `Vframe_alpha_lower_limit` is `globals.f_…`, globals.h 280-281, so it has no symbol until the core exports one) | **open**: the core's to export |
| `fset_parent_frame` | `INLINE void fset_parent_frame (struct frame *, Lisp_Object)` (src/frame.h 774) | src/frame.h 774 | **open**: `nsfns.swift` 641-642 imports it under this name and signature |
| member entry `frame_alpha` | the `double alpha[2]` member `f->alpha[0]`/`[1]` (src/frame.h 707-711) | src/frame.h | **open**: the subscript is the entry's second argument |
| `store_frame_param`, `parse_color_spec`, `error` | `void store_frame_param (struct frame *, Lisp_Object, Lisp_Object)` (src/lisp.h 5117); `bool parse_color_spec (const char *, unsigned short *, unsigned short *, unsigned short *)` (src/dispextern.h 3798); `AVOID error (const char *, ...)` (src/lisp.h 4835) | defined src/frame.c 3872 / src/xfaces.c 987 / src/eval.c (`_error` is `T` there) | **provided**: `femacs/src/frame.rs` 3842 and `femacs/src/xfaces.rs` 760, and the core's `error` the other three Swift units already import |
| `STRINGP`, `SSDATA`, `SYMBOL_NAME` | `INLINE bool STRINGP` (src/lisp.h 1582), `INLINE char *SSDATA` (src/lisp.h 1658), `INLINE Lisp_Object SYMBOL_NAME` (src/lisp.h 2350) | header-only | **open**: the core's to export (`nsfns.swift` 184/200-201 imports the same three) |
| `FLOATP`, `XFLOAT_DATA` | `#define FLOATP(x) lisp_h_FLOATP (x)` (src/lisp.h 448), `INLINE double XFLOAT_DATA (Lisp_Object)` (src/lisp.h 3186) | header-only | **open**: the core's to export |
| `FRAME_TOOLTIP_P` | `#define FRAME_TOOLTIP_P(f) ((f)->tooltip)` (src/frame.h 1256) | header-only | **open**: the core's to export |
| member write `frame_set_mouse_moved` | the `struct frame` member `f->mouse_moved` (src/frame.h 434), written at C 2464/2513 | src/frame.h | **open**: `<record>_set_<member>` |
| `struct ns_display_info` members (round 11) | `ns_display_info_set_last_mouse_motion_frame` (C 2447), `ns_display_info_last_mouse_glyph` (C 2448's `&dpyinfo->last_mouse_glyph` - a **pointer** entry, `UnsafeMutablePointer<NSRect>?`), `ns_display_info_set_last_mouse_scroll_bar` (C 2515), `ns_display_info_last_mouse_frame` (C 2561/2567/2570), `ns_display_info_last_mouse_movement_time` (C 2590) | src/nsterm.h's `struct ns_display_info` | **open**: the core's to export (each takes the *frame*, the round-7 shape) |
| `Mouse_HLInfo` members (round 11) | `mouse_highlight_mouse_face_mouse_x`/`_y` (src/dispextern.h 3013) | src/dispextern.h | **open**: they take the `MOUSE_HL_INFO` handle |
| `track_mouse` | the generated global `globals.h` 852-853 spells `globals.f_track_mouse` | src/keyboard.c's `DEFVAR_LISP` | **provided** by the core's flattened globals: `nm -g build/femacs/*.o` shows `S _track_mouse` (the reference itself has **no `_track_mouse` symbol**, measured, which is why the import has to be a data symbol of its own name rather than a function) |
| `Qdrag_source`, `Qdropping`, `scroll_bar_above_handle` | two `DEFSYM`s (`S` in `build/emacs-ref/src/emacs.o`) and one `enum scroll_bar_part` enumerator (src/termhooks.h 37) | src/emacs.c / src/termhooks.h | **open**: the core's to export |
| `XSETINT` | `#define XSETINT(a, b) ((a) = make_fixnum (b))` (src/lisp.h 1341) | header-only | **open**: the lvalue shape `XSETFRAME` has |
| `fputs` | `int fputs (const char *, FILE *)` | stdio.h | libSystem (`docs/ports/nsfont.swift` 1178; `stderr` is the overlay's) |
| `note_mouse_highlight` | `void note_mouse_highlight (struct frame *, int, int)` (src/dispextern.h 3688) | defined src/xdisp.c | **open**: the reference defines it (`T` in `build/emacs-ref/src/xdisp.o`) but the ported core only *declares* it (`femacs/src/dispnew.rs` 281) - the definition is `xdisp.rs`'s, still in flight, so the gate reports the name as imported-and-unprovided (Evidence) |
| `remember_mouse_glyph` | `void remember_mouse_glyph (struct frame *, int, int, NativeRectangle *)` (src/dispextern.h 3586; `NativeRectangle` is `NSRect`, src/nsgui.h 80) | defined xdisp.c 2779, **inside `HAVE_WINDOW_SYSTEM`** (the `#ifdef` at xdisp.c 2488) | **open, gated**: the frozen configuration leaves it undefined, so the object has no symbol and the shim exports it under the same gate - the status 3.255 ③ `frame_set_double_buffered` reading (only a window-system build can satisfy the name) |
| `emacs_view_frame`, `is_emacs_view` | the `EmacsView` messages C 2537-2538 sends | this unit's own class, C 7039 | **open**: receiver-first entries, defined when the class lands; `nsfns.swift` 689-691 imports the same two names |
| `is_emacs_tooltip` | `[[w delegate] isKindOfClass: [EmacsTooltip class]]` (C 2534) | `nsmenu.swift`'s `EmacsTooltip` (`src/nsmenu.m` 1421) | **open**: that class's own unit defines it here |
| `ns_output_current_pointer`, `ns_output_set_current_pointer` | the `struct ns_output` member `f->output_data.ns->current_pointer` - `#define FRAME_POINTER_TYPE(f)` (src/nsterm.h 977/1047), read at C 2631 and written at C 2634 | src/nsterm.h | **open**: the family's member names (round 12 takes them rather than a macro-name entry - see Decisions); both sit behind the shim's `have_ns` gate, the **write** half already exists for this very C line (`docs/ports/abi_shim.md` 96), and the read is the one name this slice adds to the ratchet |
| `QCordinary` | src/nsterm.m 11793's own `DEFSYM (QCordinary, ":ordinary")` | **this file** | **open**: class A - a symbol of this file, absent from `docs/baseline/globals/symbols.tsv` (the `QCmouse` reading of round 2) |
| `Qnone` | `DEFSYM (Qnone, "none")` (src/frame.c 7241) | src/frame.c | **open**: the core's to export; `_Qnone` is `S` in `build/emacs-ref/src/emacs.o` |
| the face/window/run records (round 13) | `NS_FACE_BACKGROUND` (src/nsterm.h 1041), `FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL`, `DEFAULT_FACE_ID`/`CHILD_FRAME_BORDER_FACE_ID`/`INTERNAL_BORDER_FACE_ID` (src/dispextern.h), `ANY_AREA` (src/dispextern.h 883), `window_frame` (the `window_<member>` family), `run_current_y`/`run_desired_y`/`run_height` (`struct run`, src/dispextern.h), `FRAME_TOP_MARGIN_HEIGHT`/`FRAME_BOTTOM_MARGIN_HEIGHT` (src/frame.h 1139/1151) | src/nsterm.h / src/dispextern.h / src/frame.h | **open**: the member/enumerator/macro names of a record the unit reaches through entries; `FRAME_TOP_MARGIN_HEIGHT`'s body sums three other `FRAME_*` macros over `struct frame` members, so the expansion would owe four member entries where one name is owed (Decisions) |
| `gui_clear_cursor` | `void gui_clear_cursor (struct window *)` (src/dispextern.h; defined src/xdisp.c 35466 **inside `HAVE_WINDOW_SYSTEM`**, the `#ifdef` at xdisp.c 35045) | src/xdisp.c | **open, gated**: the frozen configuration leaves it undefined, so the object has no symbol (measured) - the status 3.255 ③ shape again |
| `Vface_remapping_alist` | src/xfaces.c's `DEFVAR_LISP ("face-remapping-alist", …)` | src/xfaces.c | **provided**: the core's flattened global exports `_Vface_remapping_alist` (measured) |
| `FRAME_DEFAULT_FACE`, `FRAME_ROOT_WINDOW`, `XWINDOW`, `NS_FACE_BACKGROUND`, `FRAME_INTERNAL_BORDER_WIDTH`, `FRAME_PIXEL_WIDTH`, `FRAME_PIXEL_HEIGHT` | the frame/window/face accessors (src/nsterm.h 1043, src/frame.h 1201/1066, src/window.h) | the reference's headers | **open**: already ratcheted from the other Swift units' imports; round 13 is where this unit starts reading them |
| `mark_window_cursors_off`, `lookup_basic_face`, `NSRectFill` | `void mark_window_cursors_off (struct window *)` (src/dispextern.h), `int lookup_basic_face (struct window *, struct frame *, int)` (src/xfaces.c), `void NSRectFill (NSRect)` (AppKit) | src/window.c / src/xfaces.c / AppKit's NSGraphics.h | **provided**: the core's own (`femacs/src/window.rs` 5788, `femacs/src/xfaces.rs` 3753) and the SDK's (`system` in the gate's classes) |
| `copyRect` (`copyRect_entry`) | C 2909: `[view copyRect:srcRect to:dest];` - `EmacsView -copyRect:to:` (C 9340) | **this unit's own class** | **deferred** (`## ABI` (4)): the first selector segment as the entry name, the `beginDrag` reading |
| the glyph_row/window/glyph-string records (round 14) | `window_pseudo_window_p` (src/window.h), `glyph_row_mode_line_p`/`glyph_row_full_width_p`/`glyph_row_set_redraw_fringe_bitmaps_p` (the `glyph_row_*` family round 4 opened with `glyph_row_y`/`glyph_row_visible_height`), `glyph_string_set_left_overhang`/`glyph_string_set_right_overhang`/`glyph_string_char2b`/`glyph_string_nchars`/`glyph_string_font`/`glyph_string_cmp`/`glyph_string_cmp_id`/`glyph_string_cmp_from`/`glyph_string_cmp_to`/`glyph_string_first_glyph` (the new family), `glyph_type` (the `unsigned type : 3` bitfield) and the enumerators `CHAR_GLYPH`/`COMPOSITE_GLYPH` (src/dispextern.h 431-438) | src/dispextern.h / src/window.h | **open**: member/enumerator entries, `<record>_<member>` with `_set_` for the two writes |
| `composition_pixel_width`, `composition_lbearing`, `composition_rbearing` | `struct composition`'s `short pixel_width, ascent, descent; short lbearing, rbearing;` (src/composite.h 148) | src/composite.h | **open**: the record belongs to the core (`femacs/src/composite.rs`), so the three reads are member entries |
| `windows_or_buffers_changed` | `int windows_or_buffers_changed` (src/xdisp.c) | src/xdisp.c | **open**: a plain global the objects hold (`C`, common, in `build/emacs-ref/src/xdisp.o`), so the core exports a data symbol of its own name |
| `font_driver`, `font_driver_text_extents` | `struct font`'s member `driver` and the driver slot `void (*text_extents) (struct font *, const unsigned *, int, struct font_metrics *)` (src/font.h 672-673) | src/font.h | **open**: C's `font->driver->text_extents (…)` as two member entries, the second returning the C function pointer (`@convention(c)`, the slot `macfont.swift` 11162 declares) - no `struct font`/`struct font_driver` view in this unit |
| `composition_gstring_from_id`, `composition_gstring_width` | `Lisp_Object composition_gstring_from_id (ptrdiff_t)` and `int composition_gstring_width (Lisp_Object, ptrdiff_t, ptrdiff_t, struct font_metrics *)` (src/composite.h) | defined src/composite.c | **provided**: `femacs/src/composite.rs` 758 and 829 |
| the fringe records (round 15) | `WINDOW_FRAME` (src/frame.h 929, the macro - the bare `w->frame` read stays round 13's `window_frame`, one entry per C spelling), `face_foreground`/`face_background` (the **bare** `struct face` reads C 3254/3279/3281 - `NS_FACE_BACKGROUND` is the macro's entry), the thirteen `draw_fringe_bitmap_params_*` members (src/dispextern.h) and `ns_output_cursor_color` (the `struct ns_output` member read at C 3283, already ratcheted from `nsfns.swift` 282) | src/frame.h / src/dispextern.h / src/nsterm.h | **open**: member/macro entries under `<record>_<member>`, the family `macfont`/`nsmenu` established |
| `gui_define_fringe_bitmap`, `max_used_fringe_bitmap` | `void gui_define_fringe_bitmap (struct frame *, int)` (src/fringe.c 1853) and `int max_used_fringe_bitmap = MAX_STANDARD_FRINGE_BITMAPS;` (src/fringe.c 497) | src/fringe.c | **open, gated**: the frozen configuration compiles no `fringe.c` at all (`femacs/src/fringe.rs` is the empty unit status 3.31 records), so neither has a reference-side symbol (measured) - the function's row carries the gate the file has, and the global is a data symbol |
| the cursor records (round 16) | `WINDOW_XFRAME` (src/window.h 620's macro), the five `struct window` cursor members `window_set_phys_cursor_type`/`window_set_phys_cursor_on_p`/`window_phys_cursor_width`/`window_set_phys_cursor_width`/`window_phys_cursor_hpos` (**gated**: src/window.h 340-350's `HAVE_WINDOW_SYSTEM` block), the four `struct glyph_row` members `glyph_row_exact_window_width_line_p`/`glyph_row_used` (the `short used[1 + LAST_AREA]` member with C's `TEXT_AREA` subscript as the argument)/`glyph_row_set_cursor_in_fringe_p`/`glyph_row_height`, the two `struct glyph` members `glyph_resolved_level`/`glyph_pixel_width`, the macros `FRAME_CURSOR_WIDTH`/`FRAME_FOREGROUND_PIXEL`, the enumerators `TEXT_AREA`/`DRAW_CURSOR`/six `text_cursor_kinds`/four face ids | src/window.h / src/dispextern.h / src/frame.h | **open**: member/macro/enumerator entries; the five window members keep the gate their C declaration has |
| `get_phys_cursor_geometry`, `draw_phys_cursor_glyph`, `draw_fringe_bitmap` | `void get_phys_cursor_geometry (struct window *, struct glyph_row *, struct glyph *, int *, int *, int *)` (src/xdisp.c 2707), `void draw_phys_cursor_glyph (struct window *, struct glyph_row *, enum draw_glyphs_face)` (src/xdisp.c 35099), `void draw_fringe_bitmap (struct window *, struct glyph_row *, int)` (src/fringe.c 838) | src/xdisp.c (both inside `HAVE_WINDOW_SYSTEM`, opened at 2488 and 35045) / src/fringe.c (a file no built configuration compiles - `femacs/src/fringe.rs`) | **open, gated**: none has a reference-side symbol (measured), so each row carries its gate - the 3.255 ③ / 3.262 shape |
| `get_phys_cursor_glyph` | `struct glyph *get_phys_cursor_glyph (struct window *)` (src/window.h 1186; defined src/window.c 8228, no gate) | src/window.c | **provided**: `femacs/src/window.rs` 8078, and `_get_phys_cursor_glyph` is `T` in `build/emacs-ref/src/window.o` |
| Carbon and CoreFoundation (round 12) | `TISCopyCurrentKeyboardLayoutInputSource`, `TISGetInputSourceProperty`, `UCKeyTranslate`, `LMGetKbdType`, `kUCKeyActionDown`, `kUCKeyTranslateNoDeadKeysBit` and the four key masks (`shiftKey`/`optionKey`/`rightOptionKey`/`cmdKey`), plus `CFDataGetBytePtr` | Carbon.HIToolbox / CoreFoundation (C 72-73's `#include <Carbon/Carbon.h>`, C 70-80's arm) | **system**: these are *module* calls, not `@_silgen_name` entries - `import Carbon` at the top of the unit - which is a new kind of dependency for this file: the eventual Cocoa link line needs `-framework Carbon` |
| `FRAME_BACKGROUND_COLOR` | `#define FRAME_BACKGROUND_COLOR(f) ((f)->output_data.ns->background_color)` (src/nsterm.h 1038) | header-only | **open**: `macfont.swift` 756 imports the same name and type |
| `get_glyph_string_clip_rects`, `font_for_underline_metrics` | `int get_glyph_string_clip_rects (struct glyph_string *, NativeRectangle *, int)` (src/dispextern.h 3661) and `struct font *font_for_underline_metrics (struct glyph_string *)` (src/dispextern.h 3638) | src/xdisp.c 2495 and 32256, both **inside that file's `HAVE_WINDOW_SYSTEM` block** (opened at 2488 and 30717) | **open, gated**: neither has a reference-side symbol (measured this round), so each row carries its gate - the 3.255 ③ / 3.262 shape |
| `WINDOW_BUFFER_LOCAL_VALUE` | `#define WINDOW_BUFFER_LOCAL_VALUE(V, W) (BUFFERP ((W)->contents) ? buffer_local_value(V, (W)->contents) : Qnil)` (src/window.h 633-636) | header-only | **open**: class A kept *because of a predicate* - the body needs `BUFFERP` and the `buffer_local_value` function, both core facts, so the shim exports the macro's own name (the entry takes the window pointer C's `W` is) |
| `XFIXNAT` | `INLINE unsigned long XFIXNAT (Lisp_Object)` (src/lisp.h's fixnum accessor family) | header-only | **open**: class A, the `FIXNUMP` reading (round 10); already ratcheted, so it is **not** among this round's `FAIL` names |
| `Qunbound`, `Qunderline_minimum_offset` | the two generated `Lisp_Object` symbols (src/globals.h's emitter) | src/emacs.c | **provided**: `_Qunbound` and `_Qunderline_minimum_offset` are `S` in `build/emacs-ref/src/emacs.o` (measured); `Qunbound` is already ratcheted, `Qunderline_minimum_offset` is this round's one `reference`-class `FAIL` row |
| `Qx_underline_at_descent_line`, `Qx_use_underline_position_properties` | this file's own `DEFSYM`s, C 11770 and 11748 | **this file** | **open**: class A - symbols of this file, absent from `docs/baseline/globals/symbols.tsv` (the `QCmouse` reading of round 2) |
| the text-decoration records (round 17) | the fifteen `struct glyph_string` members (`glyph_string_ybase`/`_x`/`_y`/`_width`/`_height`/`_f`/`_face`/`_prev`/`_w`/`_hl`/`_for_overlaps`, the `_underline_thickness` and `_underline_position` reads plus their two `_set_` writes), the two `struct glyph` members `glyph_ascent`/`glyph_descent` (src/dispextern.h 506), the eleven `struct face` members (`face_underline`, `_underline_defaulted_p`, `_underline_color`, `_underline_at_descent_line_p`, `_underline_pixels_above_descent_line`, `_overline_p`, `_overline_color`, `_overline_color_defaulted_p`, `_strike_through_p`, `_strike_through_color`, `_strike_through_color_defaulted_p`, src/dispextern.h 1823-1887), the three `struct font` members `font_descent`/`font_underline_thickness`/`font_underline_position` (src/font.h 319-332) and the six `enum face_underline_type` enumerators (src/dispextern.h 1767-1780) | src/dispextern.h / src/font.h | **open**: member/enumerator entries under `<record>_<member>`, the family round 14 opened; `for_overlaps` is an `unsigned : 3` bitfield, so its entry keeps C's raw value rather than a `Bool` |

| `face_box`, `face_box_color`, `face_box_vertical_line_width`, `face_box_horizontal_line_width`, `face_use_box_color_for_shadows_p`, `face_stipple` | the `struct face` members C 3849-3850/4040-4041/4077-4086/4120 read - `ENUM_BF (face_box_type) box : 2`, `unsigned long box_color`, `int box_vertical_line_width`, `int box_horizontal_line_width`, `bool_bf use_box_color_for_shadows_p : 1` and `ptrdiff_t stipple` (src/dispextern.h 1858/1826/1844/1845/1865/1810) | src/dispextern.h | **open**: member entries under `<record>_<member>`; `box` is a bitfield, so its entry keeps C's raw value |
| `FACE_SIMPLE_BOX`, `FACE_RAISED_BOX`, `DRAW_MOUSE_FACE` | the two `enum face_box_type` enumerators C 4077-4087 tests (src/dispextern.h 1757/1761) and the `enum draw_glyphs_face` one C 4068/4071 tests (1353) | src/dispextern.h | **open**: enumerator entries (class A of 3.255 - an enumerator's value is the core's fact) |
| `ns_output_relief_background_color`, `ns_output_light_relief_color`, `ns_output_dark_relief_color` and their three `_set_` writes | `struct ns_output`'s `NSColor *relief_background_color` / `light_relief_color` / `dark_relief_color` (src/nsterm.h 943-945; the frozen arm's pointer spellings are 953-955) | src/nsterm.h | **open**: three member pairs of the `have_ns` family, the `ns_output_cursor_color` shape; the setter's C text is `x = [y retain];` |
| `glyph_string_row`, `glyph_string_area`, `glyph_string_img`, `glyph_string_next`, `glyph_string_background_width`, `glyph_string_extends_to_end_of_line_p`, `glyph_string_background_filled_p` + `glyph_string_set_background_filled_p`, `glyph_string_font_not_found_p`, `glyph_string_stippled_p` | the `struct glyph_string` members C 4045-4174 reads and the one it writes (src/dispextern.h 1374, 1399, 1402, 1431, 1434, 1439, 1443, 1483, 1506) | src/dispextern.h | **open**: member entries (`_set_` for `background_filled_p`, the one write) |
| `glyph_u_cmp_automatic`, `glyph_u_cmp_id`, `glyph_slice_cmp_to`, `glyph_left_box_line_p`, `glyph_right_box_line_p` | the `struct glyph` members the composite walk and the two box-line tests read - `bool_bf automatic : 1`, `unsigned id : 31`, the other union's `struct { int from, to; } cmp`, `bool_bf left_box_line_p`/`right_box_line_p` (src/dispextern.h 599/601/581/524-525) | src/dispextern.h | **open**: member entries under `<record>_<member>`; `id` is 31 bits, so it is `Int32` and widens to the `ptrdiff_t` `s->cmp_id` (C's own promotion) |
| `glyph_row_glyphs` | `struct glyph *glyphs[1 + LAST_AREA]` (src/dispextern.h 934) with C's `s->area` subscript as the entry's second argument | src/dispextern.h | **open**: member entry, the `glyph_row_used` shape |
| `glyph_advance` | C's `struct glyph *` arithmetic - `g + n` (C 4053), `g++` (C 4055/4058), `s->first_glyph + s->nchars - 1` (C 4062) and the `g < end` comparison (C 4056) | src/dispextern.h | **open**: the family's first *operation* row - the record's stride is the core's fact (Decisions) |
| `WINDOW_RIGHT_EDGE_X` | `#define WINDOW_RIGHT_EDGE_X(W) (FRAME_INTERNAL_BORDER_WIDTH (WINDOW_XFRAME (W)) + WINDOW_RIGHT_PIXEL_EDGE (W))` (src/window.h 764-766) | header-only | **open**: class A (a macro over three other macros), the `WINDOW_FRAME` shape |
| `window_box_right` | `int window_box_right (struct window *, enum glyph_row_area)` (src/dispextern.h 3581) | src/xdisp.c 1450 | **provided**: `T` in `build/emacs-ref/src/xdisp.o`, and xdisp.c is a ported unit |
| `NS_FACE_FOREGROUND` | `#define NS_FACE_FOREGROUND(f) ((f)->foreground)` (src/nsterm.h 1040) | header-only | **open**: the other half of the `NS_FACE_BACKGROUND` pair round 13 imports; already ratcheted (`macfont.swift`) |
| `FONT_HEIGHT`, `FONT_TOO_HIGH` | `#define FONT_HEIGHT(f) ((f)->height)` (src/font.h 244) and `#define FONT_TOO_HIGH(ft) ((ft)->pixel_size > 0 && (ft)->ascent + (ft)->descent > 3*(ft)->pixel_size)` (src/dispextern.h 1712-1713) | header-only | **open**: class A (both bodies are `struct font` member arithmetic); `FONT_HEIGHT` is already ratcheted |
| `CGContextClipToRect`, `CGContextScaleCTM`, `CGContextDrawTiledImage`, `CGImageGetWidth`, `CGImageGetHeight` | the five CoreGraphics calls of C 4130-4137 | CoreGraphics' `CGContext.h`/`CGImage.h` | **system**: the SDK's own symbols (the round-5 `CGMainDisplayID` row); Swift's API notes rename all five, which is why they are entries |
| `ns_display_info_bitmaps_img` | `struct ns_bitmap_record *bitmaps` (src/nsterm.h 873, the record at 810-819) with C's `face->stipple - 1` subscript as the argument, reaching the record's `EmacsImage *img` (813 in the `__OBJC__` arm, `void *` 816 in the frozen one) - C 4119-4120 | src/nsterm.h | **open**: a two-level member entry whose second argument is C's own subscript (the `glyph_row_used` shape) |
| `EmacsImage_stippleMask` | `- (CGImageRef)stippleMask` (declared src/nsterm.h 696, defined src/nsimage.m 542) | another lane's class (`nsimage.swift`'s `EmacsImage`) | **open**: the `<Class>_<selector>` shape (`EmacsImage_allocInitFromFile`, `nsfns.swift` 402) - that class landed; its C-ABI entry has not |

| `image_pixmap`, `image_mask` | `Emacs_Pixmap pixmap, mask;` (src/dispextern.h 3222) - C 4272's `s->img->pixmap` and C 4302's `s->img->mask` | src/dispextern.h | **open**: the NS arm `typedef`s `Emacs_Pixmap` to `id` (src/nsgui.h 56), so both are `AnyObject?`, the `ns_display_info_bitmaps_img` reading |
| `image_width`, `image_height`, `image_relief`, `image_hmargin`, `image_vmargin` | `int width, height;` (src/dispextern.h 3304), `int relief;` (3339) and `int hmargin, vmargin;` (3342) - C 4198-4200, 4217-4218, 4257-4259, 4301-4302 | src/dispextern.h | **open**: member entries of the `have_window_system` record |
| `glyph_string_slice_x`, `glyph_string_slice_y`, `glyph_string_slice_width`, `glyph_string_slice_height` | `struct glyph_slice`'s four `unsigned : 16` members (`unsigned x` at src/dispextern.h 457; the record 455-460), read at C 4192/4197/4199/4221-4222/4253-4260/4283/4287/4292-4293 | src/dispextern.h | **open**: member reads; the fields promote to `int`, so each keeps the promoted `Int32` value |
| `glyph_string_slice_ptr` | the `&s->slice` of C 4186/4274 - the address of the `struct glyph_slice slice;` member (src/dispextern.h 1489), `image_ascent`'s third argument | src/dispextern.h | **open**: the family's second *operation* row after `glyph_advance` - the record's layout is the core's fact |
| `image_ascent` | `int image_ascent (struct image *, struct face *, struct glyph_slice *)` (src/dispextern.h 3774, inside the header's `HAVE_WINDOW_SYSTEM` block; defined src/image.c 1911) | src/dispextern.h | **open**: src/image.c is not compiled by the frozen build and its port is an empty unit, so no object holds the symbol |
| `face_id` | `struct face`'s `int id;` (src/dispextern.h 1795; ungated) - C 4205/4225/4238 | src/dispextern.h | **open**: member entry, the `face_box_*` family's shape |
| `FACE_NO_BOX`, `TOOL_BAR_FACE_ID`, `TAB_BAR_FACE_ID`, `DRAW_IMAGE_RAISED`, `DRAW_IMAGE_SUNKEN` | the `enum face_box_type` member at src/dispextern.h 1753, the two `enum face_id` members at 1964/1979 and the two `enum draw_glyphs_face` members at 1354/1355 - C 4190-4213 and 4282-4380 test them | src/dispextern.h | **open**: enumerator entries (class A of 3.255) |
| `tab_bar_button_relief`, `tool_bar_button_relief` | the generated globals `intmax_t f_tab_bar_button_relief;` / `intmax_t f_tool_bar_button_relief;` (globals.h 942-945; the `DEFVAR_INT`s at src/xdisp.c 38973/38979) - C 4206-4212 | generated `globals.h` | **open**: data symbols the core exports (`windows_or_buffers_changed`'s row); `Int`, C's `intmax_t` |
| `Vtab_bar_button_margin`, `Vtool_bar_button_margin` | the `DEFVAR_LISP` variables at src/xdisp.c 38965/38985 - C 4227-4235 and 4240-4248 | src/xdisp.c | **open**: data symbols (`docs/baseline/globals/variables.tsv` 395/413) |
| `EmacsImage_transform`, `EmacsImage_smoothing` | `EmacsImage`'s `@public` ivars `NSAffineTransform *transform;` / `BOOL smoothing;` (src/nsterm.h 680-681) - C 4342-4343 and 4349-4359 | another lane's class (`nsimage.swift`'s `EmacsImage`) | **open**: receiver-first C-ABI entries; the class has no C symbol of its own |
| `EmacsImage_size`, `EmacsImage_drawInRect` | NSImage's `- (NSSize)size` and `- (void)drawInRect:(NSRect)rect fromRect:(NSRect)fromRect operation:(NSCompositingOperation)op fraction:(CGFloat)delta respectFlipped:(BOOL)respectContextIsFlipped hints:(nullable NSDictionary<NSImageHintKey, id> *)hints` (NSImage.h 132-133) - C 4324 and 4353-4355 | another lane's class, on NSImage's own methods | **open**: receiver first, with C's six arguments and `NSCompositingOperation` kept |
| `DEFAULT_TAB_BAR_BUTTON_RELIEF`, `DEFAULT_TOOL_BAR_BUTTON_RELIEF`, `eabs` | **not entries** - the two literal `#define`s at src/dispextern.h 3436/3515 and the expression macro at src/lisp.h 5597 are spelled at the use site instead, per the (B)-first rule of status 3.251 (see Decisions) | src/dispextern.h / src/lisp.h | spelled in place |

`Qnil`/`Qleft`, the two `INLINE` predicates and `EVENT_INIT` are header-only constructs, so the
core exports a C-ABI entry under the construct's own name (`docs/ports/nsselect.md` (2)b).  The
five `var` imports (`ns_use_srgb_colorspace`, the seven `ns_*_modifier`s, `ns_auto_hide_menu_bar`,
`Vframe_list`, `visible_bell` and round 10's `Vframe_alpha_lower_limit`) are data symbols, which
is why `tools/check-swift-units.py` counts only the `func` imports; `nm -u` on the built object
lists the whole set (Evidence).

### (4) Deferred entries (the round-5 ruling's contract)

| entry name (`@_silgen_name`, Swift name) | first call site | waits for | state |
| --- | --- | --- | --- |
| `isFullscreen` (`isFullscreen_entry`) | src/nsterm.m 1001 (`ns_constrain_all_frames`) | `EmacsView -isFullscreen`, C 8957 | **deferred** |
| `setMiniwindowImage` | `nsfns.swift` 390 | `EmacsView -setMiniwindowImage:`, C 9721 | **deferred** |
| `fullscreenState` | `nsfns.swift` 392 | `EmacsView -fullscreenState`, C 9741 | **deferred** |
| `setFSValue` | `nsfns.swift` 393 | `EmacsView -setFSValue:`, C 9177 | **deferred** |
| `beginDrag` | `nsselect.swift` 223 | `EmacsWindow -beginDrag:…`, C 10528 | **deferred** |

Each row's definition is `@_cdecl("<selector>")` with the receiver first, landing beside its
class; the acceptance is that `nm -u` no longer lists the entry's name.  Round 5 adds the first
row; the other four are the cross-unit contract the same table records, and the round's report
repeats them until they land.

### (4) Deferred entries (continued)

| entry name (`@_silgen_name`, Swift name) | first call site | waits for | state |
| --- | --- | --- | --- |
| `fsIsNative` (`fsIsNative_entry`) | src/nsterm.m 1655 (`ns_make_frame_visible`) | `EmacsView -fsIsNative`, C 8952 | **deferred** |
| `handleFS` (`handleFS_entry`) | src/nsterm.m 1663 | `EmacsView -handleFS`, C 9126 | **deferred** |
| `setParentChildRelationships` (`setParentChildRelationships_entry`) | src/nsterm.m 1687 | `EmacsWindow -setParentChildRelationships`, C 9934 | **deferred** |
| `ns_set_offset` (`ns_set_offset_entry`) | src/nsterm.m 1691 | this unit's own `ns_set_offset`, C 1854 (the signature `nsfns.swift` 600 already imports) | **landed (round 8, C 1854)** - `_ns_set_offset` left `nm -u` |
| `setWindowClosing` (`setWindowClosing_entry`) | src/nsterm.m 1757 (`ns_free_frame_resources`) | `EmacsView -setWindowClosing:`, C 7047 | **deferred** |
| `ns_set_frame_alpha` (`ns_set_frame_alpha_entry`) | src/nsterm.m 1612 and 1617 (its two call sites in `ns_frame_rehighlight`) | this unit's own `ns_set_frame_alpha`, C 2366; C forward-declares it at C 1579 | **landed (round 10, C 2366)** - `_ns_set_frame_alpha` left `nm -u`.  (The rows written in round 7 cited C 1616/C 1622; the reference's own lines are 1612/1617 and 1579 - corrected here, the calls and the prototype are the same two.) |
| `EmacsWindow_initWithEmacsFrame` | src/nsterm.m 1983 (`ns_set_undecorated`) | `EmacsWindow -initWithEmacsFrame:`, C 9759 | **deferred** - the name follows `EmacsView_initFrameFromEmacs` (`nsfns.swift` 695) |
| `setAppearance` (`setAppearance_entry`) | src/nsterm.m 2144 (`ns_set_appearance`) | `EmacsWindow -setAppearance`, C 10398 | **deferred** |
| `copyRect` (`copyRect_entry`) | src/nsterm.m 2909 (`ns_scroll_run`) | `EmacsView -copyRect:to:`, C 9340 | **deferred** - the first emitter of a multi-segment selector (the `beginDrag` shape) |

The `ns_set_offset` and `ns_set_frame_alpha` rows are the ruling's shape applied to **functions**:
C itself forward-declares them (the reference's own acknowledgement of the forward use), so the
call site takes an entry and the definition lands at its own C line - and **both are closed now**
(round 8's C 1854 and round 10's C 2366).  Round 10 adds two more rows of the class family
(`EmacsWindow_initWithEmacsFrame`, `setAppearance`), the shape `handleFS` /
`setParentChildRelationships` already have.

### (3) What other units must supply

* the core: (2)'s ten call entries - `SYMBOLP`, `EQ`, `plist_get`,
  `parse_solitary_modifier`, `Qnil`, `Qleft`, `xrealloc`, `nconc2`, `list1`, `EVENT_INIT` - and
  its **thirteen** data symbols `ns_use_srgb_colorspace`, `QCmouse`, `up_modifier`, `down_modifier`,
  `hyper_modifier`, `shift_modifier` and the seven `ns_*_modifier` variables.  **Round 10 adds the
  names its (2) rows carry**: the six `frame_set_*` member writes, the seven enumerators
  (`z_group_*`, `ns_appearance_*`), the seven globals (`Qparent_frame`, `Qabove`,
  `Qabove_suspended`, `Qbelow`, `Qdark`, `Qlight`, `Vframe_alpha_lower_limit`), `fset_parent_frame`,
  the member entry `frame_alpha`, and `STRINGP`/`SSDATA`/`SYMBOL_NAME`/`FLOATP`/`XFLOAT_DATA`/
  `FRAME_BACKGROUND_COLOR`; `store_frame_param`, `parse_color_spec` and `error` are already the
  core's (frame.rs 3842, xfaces.rs 760, eval.c's `_error`).  **Round 11 adds the mouse section's**:
  `FRAME_TOOLTIP_P`, `frame_set_mouse_moved`, the five `ns_display_info_*` last-mouse members, the
  two `Mouse_HLInfo` coordinates, `Qdrag_source`/`Qdropping`, `scroll_bar_above_handle`, `XSETINT`,
  `emacs_view_frame`/`is_emacs_view` and `is_emacs_tooltip`; `note_mouse_highlight` is the one
  whose provider is *in flight* (`xdisp.rs` owns its definition), `track_mouse` and `fputs` are
  already provided (the core's flattened global and libSystem), and `remember_mouse_glyph` is the
  gated one.  **Round 12 adds two**: `QCordinary` (class A - this file's own `DEFSYM`, so the core
  exports a symbol of that name) and `ns_output_current_pointer` (the `have_ns` family's missing
  read half, next to the fourteen its siblings already have) (its C text is inside
  `HAVE_WINDOW_SYSTEM`, so nothing to link against in the frozen build - export it under the same
  gate, the 3.255 ③ `frame_set_double_buffered` reading).  `QCmouse` and the
  seven modifiers are this file's own `DEFSYM`/`DEFVAR_LISP`, so the core has to export them the
  way it exports the rest; `QCmouse` is also absent from `docs/baseline/globals/symbols.tsv`,
  whose table comes from a build that never compiles this file;
* `femacs/platform/macos/nsmenu.swift` (another lane's unit): the `EmacsMenu` class whose
  pointer this unit carries untyped;
* other lanes read this unit's exports: `nsselect.swift` calls `ns_send_types()`,
  `ns_return_types()` and `ns_release_object` (as a callback, 215/793); `emacs.c`'s port will call
  `ns_relocate`/`ns_init_pool`/`ns_init_locale`, `image.c`'s `ns_retain_object`, and `thread.c`'s
  the two pool hooks;
* nothing new to supply from round 3: its imports are libobjc/libSystem entries.  Round 4 adds
  two owed **member entries** (`glyph_row_y`, `glyph_row_visible_height`) to the list above;
* the core, **round 13's names**: `NS_FACE_BACKGROUND`, `FACE_FROM_ID`, `FACE_FROM_ID_OR_NULL`,
  `DEFAULT_FACE_ID`, `CHILD_FRAME_BORDER_FACE_ID`, `INTERNAL_BORDER_FACE_ID`, `ANY_AREA`,
  `window_frame`, `run_current_y`/`run_desired_y`/`run_height`, `FRAME_TOP_MARGIN_HEIGHT`,
  `FRAME_BOTTOM_MARGIN_HEIGHT` and - gated, like `remember_mouse_glyph` - `gui_clear_cursor`.
  Provided already: `mark_window_cursors_off` (window.rs 5788), `lookup_basic_face`
  (xfaces.rs 3753), `Vface_remapping_alist` (the core's flattened global) and `NSRectFill`
  (AppKit's `system` class in the gate's reading); the seven accessors round 13 starts reading
  (`FRAME_DEFAULT_FACE`, `FRAME_ROOT_WINDOW`, `XWINDOW`, `NS_FACE_BACKGROUND`,
  `FRAME_INTERNAL_BORDER_WIDTH`, `FRAME_PIXEL_WIDTH`, `FRAME_PIXEL_HEIGHT`) were already ratcheted
  by the other Swift units' imports;
* the core, **round 14's names**: the `glyph_row_*` trio (`mode_line_p`, `full_width_p`, the
  `set_redraw_fringe_bitmaps_p` write), `window_pseudo_window_p`, the ten new `glyph_string_*`
  entries (the family's first slice; the two overhang **writes** only - their reads land with the
  slice that reads them), `glyph_type`, the `CHAR_GLYPH`/`COMPOSITE_GLYPH` enumerators, the three
  `composition_*` member reads, `font_driver`, `font_driver_text_extents` and the
  `windows_or_buffers_changed` data symbol.  Provided already: `composition_gstring_from_id`/
  `composition_gstring_width` (`femacs/src/composite.rs` 758/829) and the `struct glyph_row`
  family's first two rows (`glyph_row_y`/`glyph_row_visible_height`);
* the core, **round 15's names**: `WINDOW_FRAME`, `face_foreground`, `face_background`, the thirteen
  `draw_fringe_bitmap_params_*` members and - the two from a file no built configuration compiles -
  `gui_define_fringe_bitmap` and the `max_used_fringe_bitmap` data symbol.  `ns_output_cursor_color`
  was already ratcheted (its siblings' round-11 batch);
* the core, **round 16's names**: `WINDOW_XFRAME`, the five gated window-cursor members, the four
  `glyph_row_*`, the two `glyph_*`, `FRAME_CURSOR_WIDTH`, `FRAME_FOREGROUND_PIXEL`, the twelve
  enumerators (`TEXT_AREA`, `DRAW_CURSOR`, the six `text_cursor_kinds`, the four face ids) and the
  three gated/dead functions `get_phys_cursor_geometry`/`draw_phys_cursor_glyph`/
  `draw_fringe_bitmap`.  `get_phys_cursor_glyph` is provided (`window.rs` 8078) and
  `FRAME_CURSOR_COLOR` was already ratcheted (`macfont.swift`'s batch);
* the core, **round 17's names**: the two gated xdisp.c functions `get_glyph_string_clip_rects`
  and `font_for_underline_metrics`; `WINDOW_BUFFER_LOCAL_VALUE` (class A: `BUFFERP` +
  `buffer_local_value`); `XFIXNAT`; the four `Q*` symbols (`Qunbound` and
  `Qunderline_minimum_offset` provided, the two `Qx_*` this file's own `DEFSYM`s); the fifteen
  `glyph_string_*` member entries (thirteen reads and the two `_set_` writes); the two `glyph_*`
  (src/dispextern.h 506); the eleven `face_*`; the three `font_*`; and the six
  `FACE_UNDERLINE_*`/`FACE_NO_UNDERLINE` enumerators.  `face_underline` is the one the gate files
  as `rust-side` - the name already appears in the ported Rust text (`femacs/src/xfaces.rs`).
  Provided already: `NSRectFill`/`NSRectClip` (AppKit), `DRAW_CURSOR`, `glyph_string_first_glyph`,
  `glyph_string_char2b`, `glyph_string_font` and `enum glyph_type`'s pair; `XFIXNAT` and
  `Qunbound` already carry ratchet rows (they are not among the 43 `FAIL` names);
* the core, **round 18's names**: the six `struct face` members (`face_box`, `face_box_color`,
  `face_box_vertical_line_width`, `face_box_horizontal_line_width`,
  `face_use_box_color_for_shadows_p`, `face_stipple`); the three enumerators (`FACE_SIMPLE_BOX`,
  `FACE_RAISED_BOX`, `DRAW_MOUSE_FACE`); the three `ns_output_*_relief_color` read/write pairs (the
  `have_ns` family's next three, `ns_output_cursor_color`'s shape); the ten `struct glyph_string`
  member entries (`glyph_string_row`/`_area`/`_img`/`_next`/`_background_width`/
  `_extends_to_end_of_line_p`/`_background_filled_p` plus its `_set_`/`_font_not_found_p`/
  `_stippled_p`); the five `struct glyph` members (`glyph_u_cmp_automatic`, `glyph_u_cmp_id`,
  `glyph_slice_cmp_to`, `glyph_left_box_line_p`, `glyph_right_box_line_p`); `glyph_row_glyphs`;
  **`glyph_advance`** - the member family's first *operation* row, whose provider owes the
  `struct glyph` stride; `WINDOW_RIGHT_EDGE_X` and `FONT_TOO_HIGH` (class A); `NS_FACE_FOREGROUND`
  (the `NS_FACE_BACKGROUND` pair's other half) and `FONT_HEIGHT`, both already ratcheted; and
  `ns_display_info_bitmaps_img`.  `window_box_right` is **provided** (xdisp.c's port,
  `T` in `build/emacs-ref/src/xdisp.o`) and the five CoreGraphics calls are the SDK's own symbols.
  The gate files **36 of the 44 as `FAIL`** - five `reference-source` and thirty-one
  `unclassified`, the rounds-13/14 shape - and round 17's forty-three are gone from that list;
* `femacs/platform/macos/nsimage.swift` (another lane's unit), round 18's addition: the
  `EmacsImage -stippleMask` entry of (2) - C 4120's message - is that side's to define (receiver
  first, the `EmacsImage_allocInitFromFile` shape).  **Round 19 adds four more of that class's**:
  the two `@public` ivars `transform`/`smoothing` (src/nsterm.h 680-681) as
  `EmacsImage_transform`/`EmacsImage_smoothing`, and NSImage's own `-size` /
  `-drawInRect:fromRect:operation:fraction:respectFlipped:hints:` (NSImage.h 132-133) as
  `EmacsImage_size`/`EmacsImage_drawInRect` - all five are that side's to define, receiver first,
  and this unit imports them and nothing else;
* the core, **round 19's names**: the seven `struct image` reads (`image_pixmap`, `image_mask`,
  `image_width`, `image_height`, `image_relief`, `image_hmargin`, `image_vmargin`); the five
  `struct glyph_slice` rows (`glyph_string_slice_x`/`_y`/`_width`/`_height` and the operation row
  `glyph_string_slice_ptr`); `image_ascent` (src/image.c's - no configuration here compiles that
  file, so this one is the shim's and not "provided"); `face_id`; the five enumerators
  (`FACE_NO_BOX`, `TOOL_BAR_FACE_ID`, `TAB_BAR_FACE_ID`, `DRAW_IMAGE_RAISED`,
  `DRAW_IMAGE_SUNKEN`); and the four generated globals (`tab_bar_button_relief`,
  `tool_bar_button_relief`, `Vtab_bar_button_margin`, `Vtool_bar_button_margin`, all under the
  same gate the `windows_or_buffers_changed` row carries).  `image_pixmap` is the one name the
  **ratchet already carries** (`docs/baseline/swift-import-pending.txt` 583), which is why it is a
  review row and not a `FAIL`; the gate files **26 of the 27 as `FAIL`** - thirteen
  `reference-source` and thirteen `unclassified` - and round 18's thirty-six are gone from that
  list (Evidence).  **Round 20 adds sixty**: the driver's two new slots (`font_driver_draw`,
  `font_driver_encode_char`, with the `font_draw_fn`/`font_encode_char_fn` slot views); the four
  `struct glyph_row` flags (`glyph_row_reversed_p`, `glyph_row_stipple_p`,
  `glyph_row_set_stipple_p`, `glyph_row_mouse_face_p`); the two window queries the core already
  provides (`window_box_left`, `window_box_left_offset`) and `cursor_in_mouse_face_p` (the one
  `reference`-class row: the frozen `xdisp.o` holds the symbol and the core does not export it
  yet); `FRAME_COLUMN_WIDTH`; `mouse_highlight_mouse_face_face_id` and `MOUSE_FACE_ID`;
  `prepare_face_for_display` (src/xfaces.c's - no object in the frozen build holds it); the two
  generated `globals` fields (`x_stretch_cursor_p`, `Vglyphless_char_display`); `char_table_extras`,
  `CHAR_TABLE_P`, `XCHAR_TABLE`, `CHAR_TABLE_EXTRA_SLOTS` and `CHAR_TABLE_REF`; the composite and
  `lglyph` family (`composition_offsets`, `COMPOSITION_GLYPH`, `LGSTRING_GLYPH`,
  `LGLYPH_ADJUSTMENT`, `LGLYPH_WIDTH`, `LGLYPH_XOFF`, `LGLYPH_YOFF`, `LGLYPH_WADJUST`);
  `face_overstrike`; the glyphless members (`glyph_u_glyphless_method`/`_for_no_font`/`_len`/`_ch`,
  the four `glyph_slice_glyphless_*` offsets and the three `GLYPHLESS_DISPLAY_*` enumerators);
  `glyph_string_set_char2b` and `glyph_string_set_stippled_p`; the dispatcher's reads and writes
  (`glyph_string_left_overhang`, `glyph_string_right_overhang`, `glyph_string_clip_head`,
  `glyph_string_clip_tail`, `glyph_string_set_clip_head`, `glyph_string_set_hl`,
  `glyph_string_set_num_clips`); `face_font`, `FRAME_FONT` and `FRAME_FOREGROUND_COLOR`; the six
  `enum glyph_type`/`enum draw_glyphs_face` members (`IMAGE_GLYPH`, `STRETCH_GLYPH`,
  `XWIDGET_GLYPH`, `GLYPHLESS_GLYPH`, `DRAW_NORMAL_TEXT`, `DRAW_INVERSE_VIDEO`);
  `get_glyph_string_clip_rect` (the gated one-rect wrapper beside round 16's plural); and
  `x_draw_xwidget_glyph_string` (src/xwidget.c's, which the frozen build does not compile - the
  `image_ascent` shape).  The gate files **55 of those 58 as `FAIL`** - thirty `reference-source`,
  twenty-seven `unclassified` and one `reference` - because `FRAME_COLUMN_WIDTH`
  (`docs/baseline/swift-import-pending.txt` 94), `LGSTRING_GLYPH` (161) and `XCHAR_TABLE` (412)
  already carry rows and the two window queries are provided (Evidence);
* `femacs/platform/macos/nsmenu.swift` (another lane's unit), round 11's addition: the
  `EmacsTooltip` class is landed there, so the `is_emacs_tooltip` entry of (2) - C 2534's
  `isKindOfClass: [EmacsTooltip class]` - is that side's to define (receiver first, the
  `is_emacs_view` shape).  This unit imports it and nothing else;
* **the reverse direction - what other units already import from this one** (a contract this
  unit has to satisfy when its classes land): `nsfns.swift` 390-393 imports
  `setMiniwindowImage`, `fullscreenState` and `setFSValue` (all `EmacsView` methods) and
  `nsselect.swift` 223 imports `beginDrag` (an `EmacsWindow` method).  The first three are also
  the evidence for shape (b) in Decisions.  **Round 11 names three more**: `nsfns.swift` 689-691
  already imports `emacs_view_frame` and `is_emacs_view`, which this unit's own class has to
  define when it lands (C 7039's `@implementation EmacsView`), and C 2537-2538 in this round are
  the same two entries read from this side.  **Round 12 adds the other direction**: `get_keysym_name`
  is declared by src/lisp.h 5436 and *defined here*, and `keyboard.rs` 8553's
  `#[cfg (feature = "have_window_system")]` arm is the Rust side's carrier of the call (its own
  comment records that the name is not declared there, so nothing links against it today - the
  entry exists for the Cocoa build the way this whole lane does).
* **the core and the xdisp lane, round 39's names**: the table's first entry is C's
  `ns_frame_parm_handlers` (src/nsterm.h 1262, defined in src/nsfns.m 1058) - the `nsfns` lane
  carries it as a Swift array and its own ledger's `## ABI` (3) records that **the C data symbol
  still needs a port-side definition**; this unit now needs the same thing for the same reason, so
  the requirement is owed once and used twice.  The seven `gui_*` entries C hands the table -
  `gui_produce_glyphs`, `gui_write_glyphs`, `gui_insert_glyphs`, `gui_clear_end_of_line`,
  `gui_clear_window_mouse_face`, `gui_get_glyph_overhangs`, `gui_fix_overlapping_area` - are
  `src/xdisp.c`'s (the xdisp lane's), imported by name with C's own signatures.  And the table's own
  C symbol, `ns_redisplay_interface`, needs a port-side definition for the same reason the handlers
  array does: Swift cannot emit a C-named data symbol, so the port carries it as a file-scope value.
* **the core, round 40's names**: the kbd buffer's three data symbols are already the core's
  (`femacs/src/keyboard.rs` 178-186: `kbd_buffer`, `kbd_fetch_ptr`, `kbd_store_ptr`, all
  `#[unsafe(no_mangle)] static mut`), as are `kbd_buffer_store_event` (keyboard.rs 4118) and
  `Fequal` (the globals mirror, `femacs/src/include/globals.rs` 7577); the nsfns lane already
  exports `Fns_display_monitor_attributes_list` (`nsfns.swift` 3669).  What is *not* supplied
  anywhere and is therefore this round's own addition: the member reader
  `ns_display_info_terminal`, the macro entry `XSETTERMINAL` (src/lisp.h 1384 - the core owes the
  C-ABI entry the way it owes `XSETFRAME`), and the two enumerator values, which this unit takes
  from its own mirror instead (`KBD_BUFFER_SIZE` 4096 at `femacs/src/include/keyboard.rs` 43,
  `MONITORS_CHANGED_EVENT` 35 at `femacs/src/include/termhooks.rs` 62) because neither the frozen
  macro table nor the core's exports carry them.
* **round 42's omission**: `+[EmacsScroller scrollerWidth]` (C 10626-10648) cannot be declared in Swift at
  all - the SDK marks `NSScroller`'s `+scrollerWidth` unavailable, so the plain override and the
  `@objc(scrollerWidth)`-renamed spelling are both refused (measured; `nsfns.swift` 4867-4874 records the same
  refusal class for `EmacsFileDelegate`'s three delegate methods).  C's method *overrides* the superclass's, its
  live body is one message (`NSScroller.scrollerWidth (for: .regular, scrollerStyle: .legacy)`), and no reference
  caller reaches it (`[EmacsScroller scrollerWidth]` occurs nowhere; C 10645's call is on `NSScroller`, in a dead
  arm), so the class is declared with the method omitted - the gap named here rather than hidden.
* **round 43's names**: `mark_object` is already the core's (`femacs/src/alloc.rs` 7308), while
  `wset_horizontal_scroll_bar`/`wset_vertical_scroll_bar` (src/window.h's INLINE setters - the port
  mirrors them as Rust functions at `femacs/src/include/window.rs` 233/236) and the macro entry
  `XSETWINDOW` (src/window.h, the `XSETFRAME` family) are C-ABI entries the core still owes.
* **round 44's names**: the bitfield writer `input_event_set_part` (src/termhooks.h 377's
  `scroll_bar_part part : 16`, the `input_event_set_kind` counterpart) is the one C-ABI entry this
  round adds and the core still owes; the two enumerators it writes
  (`SCROLL_BAR_CLICK_EVENT` 8, `HORIZONTAL_SCROLL_BAR_CLICK_EVENT` 9) are mirror-valued constants
  (`femacs/src/include/termhooks.rs` 35/36), not exports.
* **round 45's names**: the six `enum scroll_bar_part` enumerators C 10942-10954 assigns to
  `last_hit_part` (`scroll_bar_before_handle`, `scroll_bar_after_handle`, `scroll_bar_below_handle`,
  `scroll_bar_handle`, `scroll_bar_horizontal_handle`, `scroll_bar_move_ratio`) are enumerator
  entries of the `scroll_bar_above_handle` family this unit already imports, so the core owes them
  the same way.
* **round 47's names**: `XNS_SCROLL_BAR` (src/nsterm.h 1056's macro - the `xmint_pointer` inside it
  is the mirror's, `femacs/src/include/lisp.rs` 1679), `make_mint_ptr` (src/lisp.h's INLINE, mirrored
  at lisp.rs 1664), the member reader `window_vertical_scroll_bar` (the `window_frame` family) and
  the three macros `WINDOW_SCROLL_BAR_AREA_WIDTH`/`WINDOW_SCROLL_BAR_AREA_X`/`WINDOW_TOTAL_LINES`
  are the C-ABI entries this round adds and the core still owes.
* **round 48's names**: the member reader `window_horizontal_scroll_bar` (the vertical reader's
  twin) and the two macros `WINDOW_SCROLL_BAR_AREA_HEIGHT`/`WINDOW_SCROLL_BAR_AREA_Y` are the three
  entries the horizontal hook adds; the rest of its reads (`XNS_SCROLL_BAR`, `make_mint_ptr`,
  `window_box`, `WINDOW_FRAME`, `WINDOW_TOTAL_LINES`, `WINDOW_SCROLL_BAR_AREA_X`/`_WIDTH`) landed
  with round 47.
* **round 49's names**: the two predicate macros `WINDOW_HAS_VERTICAL_SCROLL_BAR` (src/window.h 894)
  and `WINDOW_HAS_HORIZONTAL_SCROLL_BAR` (src/window.h 909) - declared as `Bool` the way this unit
  declares `FRAME_LIVE_P` - are what round 49 adds and the core owes.  The scroll-bar hook batch is
  closed with this round: five hooks landed (rounds 47-49) and `ns_redraw_scroll_bars` recorded as a
  dead arm.
* **round 50's names**: the `ns_create_terminal` entry layer - `create_terminal` (the core's own),
  `output_ns` (mirror-valued), `ns_display_info_set_terminal`, and the 36 `terminal_set_<member>`
  setters (`terminal_set_clear_frame_hook` … `terminal_set_change_tab_bar_height_hook`), each with
  C's prototype from src/termhooks.h - are what this round adds and the core owes.  The setters'
  function-pointer types are `@convention(c)`, and C 5926's `struct input_event *` is **untyped**
  there because a `@convention(c)` type cannot name this unit's Swift view of that record
  (measured), the limitation round 2 recorded for `@_cdecl`.
* **round 51's names**: the frame and font members `ns_new_font` reads and writes - `XFONT_OBJECT`,
  `fontset_from_font`, `ns_output_set_fontset`/`_set_font`/`_set_baseline_offset`,
  `frame_set_column_width`/`_set_line_height`/`_set_config_scroll_bar_cols`/`_set_config_scroll_bar_lines`,
  the readers `FRAME_LINE_HEIGHT`, `FRAME_CONFIG_SCROLL_BAR_WIDTH`/`_HEIGHT`, `FRAME_COLS`, `FRAME_LINES`,
  `ns_output_window_desc`, `font_baseline_offset`/`font_average_width`, plus `get_font_ascent_descent`,
  `adjust_frame_size` and `Qfont` (the last already the core's, `src/textprop.c` 2439) - are what that
  round adds and the core owes.  The rule it settles: **reads keep the accessor macro's name, writes go
  through the setter of the family the record belongs to** (`ns_output_set_*` vs `frame_set_*`).

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
* Round 4's compile, same command: **exit 0, no output**, after two fix-ups the compiler named:
  `#selector(NSApplication.setPresentationOptions(_:))` does not exist (Swift exposes the setter
  as a property) and `CGFloat?` does not convert to `Double?` implicitly.
* `nm -g /tmp/s188r4/nsterm.o | grep ' T _'` -> one new export, `_ns_frame_scale_factor`, plus
  `___swift_memcpy32_8` - the helper Swift emits for `EmacsMargins`'s 32-byte value copies, which
  is the record's measured size again.
* `nm -u /tmp/s188r4/nsterm.o` -> round 4's imports: `_NILP`, `_FRAME_PARENT_FRAME`,
  `_FRAME_NS_VIEW`, `_WINDOW_TO_FRAME_PIXEL_Y`, `_window_box`, `_glyph_row_y`,
  `_glyph_row_visible_height`, and the data symbol `_ns_auto_hide_menu_bar`.
* `EmacsMargins`' layout, C side (`clang -std=gnu23 -I <SDK>/CoreGraphics.framework/Headers …
  -I <SDK>/Foundation.framework/Headers` on a probe that declares the four `CGFloat` members) ->
  `sizeof 32 align 8 top 0 bottom 8 left 16 right 24 (CGFloat 8)`.
* `EmacsMargins`' layout, Swift side (`swiftc -o /tmp/s188r4/layoutprobe
  femacs/platform/macos/nsterm.swift /tmp/s188r4/main.swift /tmp/s188r4/stubs.c -Xlinker
  -undefined -Xlinker dynamic_lookup`, then the program) -> `EmacsMargins size 32 align 8 top 0
  bottom 8 left 16 right 24` (the stub C file defines the data symbols the core still owes, so
  that dyld can start the program - the same note as the `input_event` probe).
* Round 5's compile, same command: **exit 0 with one warning**, `'lockFocus()' was deprecated in
  macOS 10.14: To draw, subclass NSView and implement -drawRect: …` at the port's
  `ns_update_begin` - C's own `[view lockFocus]` (C 1085).  For scale: the lane's landed
  `nsimage.swift` prints **4** warnings under the same command (measured), so a recorded
  deprecation is ordinary here, not a defect.  Two errors were fixed on the way, both named by
  the compiler: `let cgRectJiggle` cannot be passed as `&` (`var` now, matching C's mutable
  local), and the `UAZoomChangeFocusType(…)` conversion above.
* `nm -g /tmp/s188r5/nsterm.o | grep ' T _'` -> **no new C export this round** (the block adds
  only fileprivate functions and statics).
* `nm -u /tmp/s188r5/nsterm.o` -> round 5's imports: `_block_input`, `_unblock_input`, `_XFRAME`,
  `_FRAME_NS_P`, `_CONSP`, `_XCAR`, `_XCDR`, `_Vframe_list`, `_UAZoomEnabled`,
  `_UAZoomChangeFocus`, `_CGMainDisplayID`, `_CGDisplayBounds`, `_clock_gettime_nsec_np`, `_NSLog`
  - and **`_isFullscreen`**, the deferred entry of `## ABI` (4), which stays undefined until
  `EmacsView` lands (its absence from this list is that row's acceptance test).
* Round 6's compile, same command: **exit 0 with eight deprecation warnings** - four
  `'lockFocus()' was deprecated in macOS 10.14` and four `'unlockFocus()' was deprecated in macOS
  10.14`, for C's own calls.  Three errors were fixed on the way, each named by the compiler:
  `required init?(coder:)` is mandatory on an `NSImageView` subclass and `init()` is not a
  designated initializer to override (hence `init(frame:)`), and `NSBeep` is not in scope in
  Swift.
* `nm -g /tmp/s188r6/nsterm.o | grep ' T _'` -> **no new C export this round** (the bell class's
  methods are Objective-C methods, not C symbols).
* `nm -u /tmp/s188r6/nsterm.o` -> round 6's imports: `_NSRectClip`, `_NSRectClipList`,
  `_SELECTED_FRAME`, `_visible_bell` and `_mouse_highlight_set_mouse_face_defer`.
* `struct input_event`'s layout, C side (`clang -std=gnu23 -I build/emacs-ref/src -I
  build/emacs-ref/lib -I emacs/src -I emacs/lib` on a probe including `<config.h>`, `lisp.h`,
  `systime.h` and `termhooks.h`) -> `sizeof 64 align 8`, `code 4 modifiers 8 x 16 y 24
  timestamp 32 frame_or_window 40 arg 48 device 56` (the two bitfields share the unit at 0,
  width 16); the same probe prints the modifier bits `up 1 down 2 drag 4 click 8 double 16
  triple 32 alt 4194304 super 8388608 hyper 16777216 shift 33554432 ctrl 67108864
  meta 134217728`.
* Round 3's compile, same command: **exit 0, no output**.
* `nm -g /tmp/s188r3/nsterm.o | grep ' T _'` -> the exports now carry `_ns_relocate`,
  `_ns_init_pool`, `_ns_init_locale`, `_ns_release_object`, `_ns_retain_object`,
  `_ns_alloc_autorelease_pool` and `_ns_release_autorelease_pool` beside the earlier seven.
* `nm -u /tmp/s188r3/nsterm.o` -> round 3's imports are `_fprintf`, `_getenv`, `_setenv`,
  `_setlocale`, `_objc_autoreleasePoolPush`, `_objc_autoreleasePoolPop` - and **`_objc_autorelease`**,
  which is the object-time proof that `Unmanaged.passRetained(x).autorelease()` is C's
  autorelease and not a no-op; `_objc_retain`/`_objc_release` are the two hooks' runtime entries.
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

* Round 10's compile, same command: **exit 0**, with **seven** warnings.  Four are C's own
  `lockFocus`/`unlockFocus` deprecations the lane already carries (C 1217-1219, 1368-1370,
  1385-1387, 1412-1414 - `'lockFocus()' was deprecated in macOS 10.14` twice and `'unlockFocus()'`
  twice), and **three are this slice's**: `'scanFloat' was deprecated in macOS 10.15` for
  `ns_get_color`'s three `[scanner scanFloat: &…]` calls (C 2257-2259).  The two errors fixed on
  the way were each named by the compiler: `value of type 'NSString' has no member 'withCString'`
  (the receiver has to be the bridged `String`) and `initialization of immutable value 'dpyinfo'
  was never used` (the display-info entry takes the frame, not the local).
* `nm -g /tmp/s188r10/nsterm.o | grep -E ' T _(ns_set_|ns_lisp_to_color|ns_defined_color)'` ->
  round 10's eleven new C exports: `_ns_set_undecorated`, `_ns_set_parent_frame`,
  `_ns_set_no_focus_on_map`, `_ns_set_no_accept_focus`, `_ns_set_z_group`, `_ns_set_appearance_1`,
  `_ns_set_appearance`, `_ns_set_transparent_titlebar`, `_ns_lisp_to_color`, `_ns_defined_color`
  and **`_ns_set_frame_alpha`**.
* `nm -u /tmp/s188r10/nsterm.o` -> the slice's imports are `_EmacsWindow_initWithEmacsFrame` and
  `_setAppearance` (the two new deferred rows), `_frame_set_undecorated`,
  `_frame_set_no_focus_on_map`, `_frame_set_no_accept_focus`, `_frame_set_z_group`,
  `_frame_set_ns_appearance`, `_frame_set_ns_transparent_titlebar`, `_frame_alpha`, `_z_group_*`
  (four), `_ns_appearance_*` (three), `_Qparent_frame`, `_Qabove`, `_Qabove_suspended`, `_Qbelow`,
  `_Qdark`, `_Qlight`, `_Vframe_alpha_lower_limit`, `_fset_parent_frame`, `_STRINGP`, `_SSDATA`,
  `_SYMBOL_NAME`, `_FLOATP`, `_XFLOAT_DATA`, `_FIXNUMP`, `_XFIXNUM`, `_FRAME_BACKGROUND_COLOR`,
  `_store_frame_param`, `_parse_color_spec`, `_error` - and **`_ns_set_frame_alpha` is gone from
  the list**, which is the (4) table's second function row closing (`grep -c '_ns_set_frame_alpha$'`
  is 0).
* The macro/symbol audit for round 10's names (`nm -g build/emacs-ref/src/*.o
  build/emacs-ref/lib/*.o`, one `grep -E "[ _]NAME$"` per name) -> **0 symbols** for every
  macro/enumerator/member-entry name the slice imports (`FRAME_UNDECORATED`,
  `FRAME_NS_TRANSPARENT_TITLEBAR`, `FRAME_NO_FOCUS_ON_MAP`, `FRAME_NO_ACCEPT_FOCUS`,
  `FRAME_Z_GROUP`, `FRAME_NS_APPEARANCE`, `FRAME_BACKGROUND_COLOR`, `z_group_*` four,
  `ns_appearance_*` three, `Vframe_alpha_lower_limit`, `FLOATP`, `XFLOAT_DATA`,
  `EmacsWindow`/`initWithEmacsFrame`, `setAppearance`, `ns_set_frame_alpha`), and **1** for the
  data/function symbols the core already has: `_Qabove`/`_Qbelow`/`_Qdark`/`_Qlight`/
  `_Qparent_frame` (all `S` in `build/emacs-ref/src/emacs.o`), `_store_frame_param` (`T` in
  `frame.o`), `_parse_color_spec` (`T` in `xfaces.o`) and `_error` (`T` in `eval.o`) - so every
  name of the first group is class A of the 3.255 ruling.
* `Emacs_Color`'s layout, C side (`clang -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib
  -I emacs/src -I emacs/lib` on a probe including `<config.h>` and `<dispextern.h>`) ->
  `Emacs_Color sizeof 16 align 8 pixel 0 red 8 green 10 blue 12`; Swift side
  (`swiftc -o /tmp/s188r10/layoutprobe femacs/platform/macos/nsterm.swift /tmp/s188r10/main.swift
  /tmp/s188r10/stubs.c -Xlinker -undefined -Xlinker dynamic_lookup`, then the program) ->
  `Emacs_Color size 14 stride 16 align 8` and the same four offsets (`pixel 0 red 8 green 10
  blue 12`).  Swift's `size` omits the tail padding C's `sizeof` rounds up; `stride` (16) is the
  storage unit and matches C, and the member offsets and the alignment agree member for member.
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 142 C
  definition(s), 87 unported, 98 row(s); 115 C symbol import(s)` and `0 new, 17 stale, 7 Swift
  unit(s) landed`.  Eleven of the seventeen stale rows are this unit's - the eleven landed names
  that had rows; the other four this slice landed (the generator gap's) have no row to go stale -
  and the other six are the in-flight `nsfns.swift` batch.  The
  manifest is the integrator's table: this unit asks for the deletions, it does not write them.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures` and `read 630 files (623 .rs 7 .swift)` - unchanged by this slice (the
  new comments and literals carry no forbidden class).
* `python3 tools/check-swift-imports.py` -> before this slice `838 import(s) in 7 Swift file(s),
  5629 core export(s), 616 unprovided, 549 ratchet row(s), 89 failure(s)`; after it `860 import(s)
  in 7 Swift file(s), 5629 core export(s), 627 unprovided, 538 ratchet row(s), 109 failure(s)`.
  **The two readings are not from one tool revision**: `tools/check-swift-imports.py` is itself
  modified in this working tree while this round runs (another lane's or the integrator's edit),
  and the tree also carries four other lanes' in-flight units.  The comparable part is the
  `FAIL` **name** diff, and it is this slice's names only: `EmacsWindow_initWithEmacsFrame`,
  `FLOATP`, `XFLOAT_DATA`, `frame_alpha`, `frame_set_no_accept_focus`, `frame_set_no_focus_on_map`,
  `frame_set_ns_appearance`, `frame_set_z_group`, `ns_appearance_aqua`,
  `ns_appearance_system_default`, `ns_appearance_vibrant_dark`, `setAppearance`, `Qabove`,
  `Qabove_suspended`, `Qbelow`, `Qdark`, `Vframe_alpha_lower_limit`, `z_group_above`,
  `z_group_above_suspended`, `z_group_below`, `z_group_none` - **21 rows, all additions**.  The
  slice's other imports were already ratchet rows from the other Swift units
  (`frame_set_undecorated`, `frame_set_ns_transparent_titlebar`, `FRAME_BACKGROUND_COLOR`,
  `Qparent_frame`, `Qlight`, `FIXNUMP`, `XFIXNUM`, `STRINGP`, `SSDATA`, `SYMBOL_NAME`,
  `fset_parent_frame`), or the core already provides them (`store_frame_param`,
  `parse_color_spec`, `error`).  This unit does not write the ratchet: the integrator's
  `--write-pending` does.

* Round 11's compile, same command: **exit 0**, the same **seven** warnings round 10 left (four
  C's own `lockFocus`/`unlockFocus` at C 1217-1219/1368-1370/1385-1387/1412-1414 and three
  `'scanFloat' was deprecated in macOS 10.15` for C 2257-2259), and no new one - the slice's only
  new AppKit/stdlib calls (`CGWarpMouseCursorPosition`, `NSEvent.mouseLocation`,
  `windowNumberAtPoint:belowWindowWithWindowNumber:`, `NSApp windowWithWindowNumber:`,
  `fputs`) are not deprecated.
* `nm -g /tmp/s188r11/nsterm.o | grep ' T _frame_set_mouse_pixel_position'` -> the one new C
  export, `_frame_set_mouse_pixel_position`; the slice's other three definitions are C `static`s and
  stay Swift-mangled (`grep -cE ' T _(ns_note_mouse_movement|ns_mouse_position|ns_frame_up_to_date)$'`
  is 0).
* `nm -u /tmp/s188r11/nsterm.o` -> the slice's own imports: `_FRAME_TOOLTIP_P`,
  `_frame_set_mouse_moved`, `_ns_display_info_set_last_mouse_motion_frame`,
  `_ns_display_info_last_mouse_glyph`, `_ns_display_info_set_last_mouse_scroll_bar`,
  `_ns_display_info_last_mouse_frame`, `_ns_display_info_last_mouse_movement_time`,
  `_mouse_highlight_mouse_face_mouse_x`/`_y`, `_Qdrag_source`, `_Qdropping`,
  `_scroll_bar_above_handle`, `_XSETINT`, `_note_mouse_highlight`, `_remember_mouse_glyph`,
  `_fputs`, `_emacs_view_frame`, `_is_emacs_view`, `_is_emacs_tooltip`, and the data symbol
  `_track_mouse` (461 undefined names in all, the object's whole set).
* The symbol audit for this slice's names (`nm -g -A` over `build/emacs-ref/src/*.o` and
  `build/emacs-ref/lib/*.o`, one `grep -E "[ _]NAME$"` per name): **0** for `FRAME_TOOLTIP_P`,
  `frame_set_mouse_moved`, all five `ns_display_info_*` last-mouse members,
  `mouse_highlight_mouse_face_mouse_x`/`_y`, `scroll_bar_above_handle`, `XSETINT`,
  `track_mouse`, `remember_mouse_glyph` and the three port-named entries
  (`emacs_view_frame`, `is_emacs_view`, `is_emacs_tooltip`); **1** for `_Qdrag_source`/`_Qdropping`
  (`S` in `build/emacs-ref/src/emacs.o`) and `_note_mouse_highlight` (`T` in
  `build/emacs-ref/src/xdisp.o`).  The `track_mouse` and `remember_mouse_glyph` zeros are the two
  readings the Decisions explain: a generated global with no carrier symbol, and a definition the
  frozen configuration does not compile.
* The guard behind `remember_mouse_glyph`: tracking the preprocessor nesting of `src/xdisp.c` to
  line 2779 gives `#ifdef HAVE_WINDOW_SYSTEM` at **2488**, and `nm
  build/emacs-ref/src/xdisp.o | grep -i remember` finds nothing at all (`grep -i` over the whole
  symbol table, not just `-g`); the four non-static xdisp.c names the object is also missing
  (`x_y_to_column_row`, `get_glyph_string_clip_rects`, `get_glyph_string_clip_rect`,
  `get_phys_cursor_geometry`) are the same region's.
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 150 C
  definition(s), 84 unported, 87 row(s); 134 C symbol import(s)` (the C count and the import count
  are the fixed reader's).  The run that opened this round read `1 new, 10 stale, 7 Swift unit(s)
  landed`, and the `new` was `ns_get_shifted_character` (C 2693 - the eighth name status 3.260's
  fix uncovered, with the other seven already landed); the integrator wrote its row in `aa3e13a`,
  after which the same command reads `0 new, 10 stale, 7 Swift unit(s) landed`, whose ten are this
  unit's four (the slice's four definitions) and the in-flight `nsfns.swift` batch.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`, `read 630 files (623 .rs 7 .swift)` - this slice's comments and literals
  add no forbidden class.
* `python3 tools/check-swift-imports.py` -> after the slice's entries: `877 import(s) in 7 Swift
  file(s), 5640 core export(s), 632 unprovided, 549 ratchet row(s), 105 failure(s)` (94 `FAIL`
  lines plus the stale-ratchet count).  The `FAIL` names that belong to this slice are **sixteen**,
  each with the tool's own class: `FRAME_TOOLTIP_P`, `XSETINT` and `remember_mouse_glyph`
  (`reference-source`), `Qdrag_source`/`Qdropping` (`reference`), `note_mouse_highlight`
  (`reference` - the reference defines it, `femacs/src/dispnew.rs` 281 only declares it, and the
  definition is the in-flight `xdisp.rs`'s), `is_emacs_tooltip`, `frame_set_mouse_moved`, the five
  `ns_display_info_*` last-mouse members and the two `Mouse_HLInfo` coordinates
  (`unclassified`).  Four of the slice's imports are **not** in the list because something already
  provides them: `track_mouse` (`S` in `build/femacs/*.o`), `fputs` (libSystem),
  `emacs_view_frame`/`is_emacs_view` (ratchet rows `nsfns.swift` already asked for).  The count's
  movement is again not this unit's alone - four lanes' in-flight units sit in the same tree - so
  the name-level `FAIL` diff is the reading that belongs to the slice.

* Round 12's compile, same command: **exit 0**, the same **seven** warnings round 11 left (four
  C's own `lockFocus`/`unlockFocus`, three `'scanFloat' was deprecated in macOS 10.15`).  The
  first run of the slice printed an eighth - `treating a forced downcast to 'CFData' as optional
  will never produce 'nil'` on `layout_ref as! CFData` - and the explicit
  `Unmanaged<CFData>.fromOpaque(…).takeUnretainedValue()` that replaced it is both the warning's
  fix and the correct ownership reading (Evidence, Decisions).
* `nm -g /tmp/s188r12/nsterm.o | grep ' T _get_keysym_name'` -> the slice's one new C export; its
  other five definitions are C `static`s and stay Swift-mangled (`nm -g … | grep -cE '
  T _(ns_define_frame_cursor|ns_convert_key|right_mod|nil_or_none|ns_get_shifted_character)$'` is 0).
* `nm -u /tmp/s188r12/nsterm.o` -> the slice's own imports: `_ns_output_current_pointer`,
  `_ns_output_set_current_pointer`, `_QCordinary`, `_Qnone` (with `_Qleft` already there) and the
  Carbon/CoreFoundation calls `_TISCopyCurrentKeyboardLayoutInputSource`,
  `_TISGetInputSourceProperty`, `_UCKeyTranslate`, `_LMGetKbdType`, `_CFDataGetBytePtr` - 495
  undefined names in all (461 after round 11's slice, so Carbon and `NSLog` are most of the
  difference).
* The symbol audit for this slice's names (`nm -g -A` over `build/emacs-ref/src/*.o` and
  `build/emacs-ref/lib/*.o`): **0** for `ns_output_current_pointer`,
  `ns_output_set_current_pointer`, `QCordinary`, `FRAME_POINTER_TYPE`, `get_keysym_name` and the
  slice's five own definitions; **1** for `_Qnone` (`S` in `build/emacs-ref/src/emacs.o`).  The
  `get_keysym_name` zero is the one that matters: the frozen build compiles no platform `*term.c`
  and not this `.m`, so the name has no reference-side symbol to point at - *this unit* is its
  definition, and `check-swift-imports` counts it as provided because the providers include the
  Swift units' own `@_cdecl` names.  Carbon's and CoreFoundation's calls are SDK symbols (nothing
  in the reference objects, and the gate classes them as system).
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 150 C
  definition(s), 78 unported, 84 row(s); 138 C symbol import(s)` and `0 new, 10 stale, 7 Swift
  unit(s) landed`: six of the stale rows are this slice's (`get_keysym_name`, `nil_or_none`,
  `ns_convert_key`, `ns_define_frame_cursor`, `ns_get_shifted_character`, `right_mod`) and the
  other four the in-flight `nsfns.swift` batch.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`, `read 630 files (623 .rs 7 .swift)`.
* `python3 tools/check-swift-imports.py` -> `889 import(s) in 7 Swift file(s), 5656 core export(s),
  623 unprovided, 616 ratchet row(s), 15 failure(s)`.  Two of the eleven `FAIL` names are this
  slice's - `QCordinary` (`reference-source`) and `ns_output_current_pointer` (`unclassified`) -
  and the other nine are the in-flight `nsfns.swift` batch; `Qnone`,
  `ns_output_set_current_pointer` and `get_keysym_name` are deliberately absent from the list
  (ratcheted, ratcheted, and provided by this unit).

* Round 13's compile, same command: **exit 0**, the same **seven** warnings round 12 left (four
  C's own `lockFocus`/`unlockFocus`, three `'scanFloat' was deprecated in macOS 10.15`) and no new
  one - `NSRectFill` and the `struct face`/`run` entries add no deprecated call.
* `nm -g /tmp/s188r13/nsterm.o | grep ' T _ns_clear_frame$'` -> the slice's one new C export; the
  other three definitions are C `static`s (`grep -cE ' T _(ns_clear_frame_area|ns_scroll_run|
  ns_clear_under_internal_border)$'` is 0).
* `nm -u /tmp/s188r13/nsterm.o` -> the slice's imports, all 25 of them: `_NS_FACE_BACKGROUND`,
  `_FACE_FROM_ID`, `_FACE_FROM_ID_OR_NULL`, `_DEFAULT_FACE_ID`, `_CHILD_FRAME_BORDER_FACE_ID`,
  `_INTERNAL_BORDER_FACE_ID`, `_ANY_AREA`, `_window_frame`, `_run_current_y`, `_run_desired_y`,
  `_run_height`, `_FRAME_TOP_MARGIN_HEIGHT`, `_FRAME_BOTTOM_MARGIN_HEIGHT`, `_gui_clear_cursor`,
  `_NSRectFill`, `_copyRect` (new) and `_FRAME_DEFAULT_FACE`, `_FRAME_ROOT_WINDOW`, `_XWINDOW`,
  `_FRAME_INTERNAL_BORDER_WIDTH`, `_FRAME_PIXEL_WIDTH`, `_FRAME_PIXEL_HEIGHT`,
  `_mark_window_cursors_off`, `_lookup_basic_face`, `_Vface_remapping_alist` (already ratcheted or
  provided).  521 undefined names in all (495 after round 12).
* The symbol audit for round 13's names (`nm -g -A` over `build/emacs-ref/src/*.o` and
  `build/emacs-ref/lib/*.o`): **1** for `_mark_window_cursors_off` (`T` in `window.o`) and
  `_lookup_basic_face` (`T` in `xfaces.o`); **0** for `NS_FACE_BACKGROUND`, `FACE_FROM_ID`,
  `FACE_FROM_ID_OR_NULL`, the three face ids, `ANY_AREA`, `window_frame`, the three `struct run`
  members, `FRAME_TOP_MARGIN_HEIGHT`, `FRAME_BOTTOM_MARGIN_HEIGHT`, `gui_clear_cursor`,
  `Vface_remapping_alist` and `NSRectFill`.  The `gui_clear_cursor` zero is the gated one (its guard
  is measured below); `mark_window_cursors_off`/`lookup_basic_face`/`Vface_remapping_alist` are
  provided by the ported core (`femacs/src/window.rs` 5788, `femacs/src/xfaces.rs` 3753, the
  flattened globals), and `NSRectFill` is AppKit's, which the gate's own classifier calls `system`
  (`SDK_SYMBOLS`, the AppKit `.tbd`).
* `gui_clear_cursor`'s guard, measured the way `remember_mouse_glyph`'s was: tracking the
  preprocessor nesting of `src/xdisp.c` to line 35466 gives the `#ifdef HAVE_WINDOW_SYSTEM` at
  **35045**, and `nm build/emacs-ref/src/xdisp.o | grep -c gui_clear_cursor` is **0**.
* The gated-layout probe (status 3.271) - the route for a *gated record's* offsets, which this
  round did not need (every access above is a member entry).  Its existing readings, for the
  record: `docs/baseline/ns_display_info-layout.tsv` -> ns_cocoa `sizeof 280 / align 8` (the frozen
  arm is `parse=no`, since the type is only complete in the NS arm), and
  `docs/baseline/frame-layout.tsv` -> frozen `sizeof 568 / align 8`, ns_cocoa `608 / align 8`, with
  the per-member rows this lane's `FRAME_*` entries stand for - among them `ns_appearance` at 592 and
  `ns_transparent_titlebar` at 596 (both `NS_IMPL_COCOA`), the two members round 10's
  `frame_set_ns_appearance`/`frame_set_ns_transparent_titlebar` write, and the `HAVE_WINDOW_SYSTEM`
  bit-block rows (`tooltip` 275:6, `no_focus_on_map` 277:1, `z_group` 277:2-3).  Both tables were
  read, not regenerated, by this round: `spikes/gated-layout-probe.py` remains the integrator's to
  re-run.
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 150 C
  definition(s), 74 unported, 84 row(s); 162 C symbol import(s)` and `0 new, 14 stale, 7 Swift
  unit(s) landed`: **ten of the stale rows are this lane's** - round 13's four and round 12's six,
  which the manifest has not absorbed yet (the round-12 delivery is still uncommitted in this
  working tree; only up to round 11 is at HEAD) - and the other four are the in-flight `nsfns.swift`
  batch.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`, `read 630 files (623 .rs 7 .swift)`.
* `python3 tools/check-swift-imports.py` -> `907 import(s) in 7 Swift file(s), 5661 core export(s),
  636 unprovided, 616 ratchet row(s), 30 failure(s)`.  The `FAIL` names that are this lane's are
  **sixteen**, the same split as the stale set: round 13's fourteen (`ANY_AREA`,
  `CHILD_FRAME_BORDER_FACE_ID`, `DEFAULT_FACE_ID`, `FACE_FROM_ID`, `FACE_FROM_ID_OR_NULL`,
  `FRAME_BOTTOM_MARGIN_HEIGHT`, `FRAME_TOP_MARGIN_HEIGHT`, `INTERNAL_BORDER_FACE_ID`, `copyRect`,
  `gui_clear_cursor`, `run_current_y`, `run_desired_y`, `run_height`, `window_frame`) and round
  12's two (`QCordinary`, `ns_output_current_pointer`), with the remaining fourteen the in-flight
  `nsfns.swift` batch.

* Round 14's compile, same command: **exit 0 on the first pass**, the same **seven** warnings the
  previous rounds carry (four C's own `lockFocus`/`unlockFocus`, three `'scanFloat' was deprecated
  in macOS 10.15`) and no new one.
* `nm -g /tmp/s188r14/nsterm.o` -> **no new C export this round** (`grep -cE ' T
  _(ns_after_update_window_line|ns_shift_glyphs_for_insert|ns_compute_glyph_string_overhangs)$'` is
  0: all three are C `static`s).
* `nm -u /tmp/s188r14/nsterm.o` -> the slice's 22 new imports (`_window_pseudo_window_p`, the three
  `_glyph_row_*`, `_windows_or_buffers_changed`, the ten `_glyph_string_*` this slice calls,
  `_glyph_type`, `_CHAR_GLYPH`, `_COMPOSITE_GLYPH`, the three `_composition_*` reads,
  `_font_driver`, `_font_driver_text_extents`) plus the two provided core functions
  (`_composition_gstring_from_id`, `_composition_gstring_width`); 547 undefined names in all (521
  after round 13).  The `glyph_string_left_overhang`/`_right_overhang` reads were declared in the
  round's first written form and are **absent from both the object and this list** - the mechanism
  the Decisions record.
* The symbol audit for round 14's names (`nm -g -A` over `build/emacs-ref/src/*.o` and
  `build/emacs-ref/lib/*.o`): **1** for `_windows_or_buffers_changed` (`C`, common, in
  `build/emacs-ref/src/xdisp.o`), `_composition_gstring_from_id` (`T` in `composite.o`) and
  `_composition_gstring_width` (`T` in `composite.o`); **0** for `window_pseudo_window_p`, the
  three `glyph_row_*`, `glyph_type`, `CHAR_GLYPH`, `COMPOSITE_GLYPH`, the three `composition_*`
  members, `font_driver`, `font_driver_text_extents` and the ten `glyph_string_*` entries.
* `spikes/gated-layout-probe.py 'struct font_metrics' font.h` (status 3.271's probe, run
  read-only) -> **frozen**, `x11` and `ns_cocoa` all read `[sizeof=10, align=2]` with
  `short lbearing 0, rbearing 2, width 4, ascent 6, descent 8`.  This record parses in the frozen
  arm as well (`parse=no` is the NS-only types' row), so the view's layout needs no arm at all;
  the Swift side's five `Int16` members give the same 10 bytes at the same offsets.
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 150 C
  definition(s), 71 unported, 74 row(s); 186 C symbol import(s)` and `0 new, 3 stale, 7 Swift
  unit(s) landed` - the three stale rows are exactly this slice's three definitions.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`, `read 652 files (623 .rs 22 .kt 7 .swift)` (the Kotlin files are another
  lane's).
* `python3 tools/check-swift-imports.py` -> `931 import(s) in 7 Swift file(s), 5687 core export(s),
  632 unprovided, 636 ratchet row(s), 48 failure(s)`.  The `FAIL` names that are this lane's are
  **22** - `font_driver`, `glyph_type`, `CHAR_GLYPH`, `COMPOSITE_GLYPH` (`reference-source`) and the
  eighteen member entries (`unclassified`) - with the remaining twenty-six the other lanes'
  in-flight units; `windows_or_buffers_changed` and the two `composition_gstring_*` names are not
  in the list (the core provides them).

* Round 15's compile, same command: **exit 0 on the first pass**, the same **seven** warnings the
  previous rounds carry (four C's own `lockFocus`/`unlockFocus`, three `'scanFloat' was deprecated
  in macOS 10.15`) and no new one; the compiler's last output line is its
  `[#DeprecatedDeclaration]: <https://docs.swift.org/compiler/documentation/diagnostics/deprecated-declaration>`
  note and the exit status is 0.  Object: `/tmp/s188r15/nsterm.o`, **md5
  `f96258c7c3401695638606c93d108888`** (the pre-round-15 revision's was round 14's object).
* `nm -g /tmp/s188r15/nsterm.o` -> **no new C export** (`grep -cE ' T
  _(ns_define_fringe_bitmap|ns_destroy_fringe_bitmap|ns_draw_fringe_bitmap)$'` is 0: all three are
  C `static`s).
* `nm -u /tmp/s188r15/nsterm.o` -> the slice's 19 imports (`_WINDOW_FRAME`, `_face_foreground`,
  `_face_background`, the thirteen `_draw_fringe_bitmap_params_*`, `_ns_output_cursor_color`,
  `_gui_define_fringe_bitmap`, `_max_used_fringe_bitmap`); 594 undefined names in all (547 after
  round 14).
* The symbol audit for round 15's names (`nm -g -A` over `build/emacs-ref/src/*.o` and
  `build/emacs-ref/lib/*.o`, all 19 imports plus the three definitions): **0 for every one of
  them** - the macros and member entries have no symbol, and `gui_define_fringe_bitmap` /
  `max_used_fringe_bitmap` do not either because the frozen configuration compiles no `fringe.c`
  (its note in `femacs/src/fringe.rs` quotes the build's empty `WINDOW_SYSTEM_OBJ=` and the rest).
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 150 C
  definition(s), 68 unported, 71 row(s); 204 C symbol import(s)` and `0 new, 3 stale, 7 Swift
  unit(s) landed` - the three stale rows are exactly this slice's three definitions.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`, `read 666 files (623 .rs 25 .kt 7 .swift 6 .s 5 .cc)` (the Kotlin, `.s`
  and `.cc` files are other lanes').
* `python3 tools/check-swift-imports.py` -> `949 import(s) in 7 Swift file(s), 5687 core export(s),
  650 unprovided, 632 ratchet row(s), 18 failure(s)`.  **All eighteen `FAIL` names are this
  lane's** - `WINDOW_FRAME`, `face_foreground`, `face_background`, `gui_define_fringe_bitmap`,
  `max_used_fringe_bitmap` (`reference-source`) and the thirteen `draw_fringe_bitmap_params_*`
  (`unclassified`) - with no stale rows and no other lane's names in the list;
  `ns_output_cursor_color` is already ratcheted (its family's batch), so it is not among them.

* Round 16's compile, same command: **exit 0 on the first pass**, the same **seven** warnings the
  previous rounds carry (four C's own `lockFocus`/`unlockFocus`, three `'scanFloat' was deprecated
  in macOS 10.15`) and no new one; the compiler's last output line is its
  `[#DeprecatedDeclaration]: <https://docs.swift.org/compiler/documentation/diagnostics/deprecated-declaration>`
  note, the exit status is 0, and the object `/tmp/s188r16/nsterm.o` has **md5
  `64c1e95f1c228e9dccb2e9dc6887e351`**.
* `nm -g /tmp/s188r16/nsterm.o` -> **no new C export** (`grep -cE ' T
  _(ns_draw_window_cursor|ns_draw_vertical_window_border|ns_draw_window_divider)$'` is 0: all three
  are C `static`s).
* `nm -u /tmp/s188r16/nsterm.o` -> the slice's **31** declared names, every one of them referenced
  (checked name by name): `_WINDOW_XFRAME`, the five window-cursor members, the four `_glyph_row_*`,
  the two `_glyph_*`, `_FRAME_CURSOR_WIDTH`, `_FRAME_FOREGROUND_PIXEL`, `_FRAME_CURSOR_COLOR`, the
  twelve enumerators, `_get_phys_cursor_glyph`, `_get_phys_cursor_geometry`,
  `_draw_phys_cursor_glyph` and `_draw_fringe_bitmap`; 628 undefined names in all (594 after round
  15).
* The symbol audit for round 16's names (`nm -g -A` over `build/emacs-ref/src/*.o` and
  `build/emacs-ref/lib/*.o`): **0** for all twenty-nine new owed names (the macros, the members -
  the five window ones are gated fields the frozen record does not have - and the twelve
  enumerators), and **1** for `_get_phys_cursor_glyph` (`T` in `build/emacs-ref/src/window.o`, the
  provided one).  `get_phys_cursor_geometry` (xdisp.c 2707, inside the `#ifdef` at 2488),
  `draw_phys_cursor_glyph` (xdisp.c 35099, inside the one at 35045) and `draw_fringe_bitmap`
  (fringe.c 838, a file no build compiles) are the three gated/dead zeros.
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 150 C
  definition(s), 65 unported, 68 row(s); 235 C symbol import(s)` and `0 new, 3 stale, 7 Swift
  unit(s) landed` - the three stale rows are exactly this slice's three definitions.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`, `read 670 files (624 .rs 28 .kt 7 .swift 6 .s 5 .cc)` (the Kotlin, `.s`
  and `.cc` files are other lanes').
* `python3 tools/check-swift-imports.py` -> `977 import(s) in 7 Swift file(s), 5705 core export(s),
  663 unprovided, 650 ratchet row(s), 41 failure(s)`.  **All twenty-seven `FAIL` names are this
  lane's** (thirteen `reference-source`: `WINDOW_XFRAME`, `FRAME_CURSOR_WIDTH`,
  `FRAME_FOREGROUND_PIXEL`, the six `*_CURSOR` enumerators, the four face ids,
  `draw_fringe_bitmap`, `draw_phys_cursor_glyph`, `get_phys_cursor_geometry`; fourteen
  `unclassified`: the nine member entries); `TEXT_AREA` and `DRAW_CURSOR` are not among them
  because other units' ratchet rows already carry them, and `FRAME_CURSOR_COLOR` /
  `get_phys_cursor_glyph` are a ratcheted row and a provided symbol.  The gate's other fourteen
  rows are `STALE (satisfied)` - **rounds 14 and 15's names, now provided** (`CHAR_GLYPH`,
  `COMPOSITE_GLYPH`, `WINDOW_FRAME`, the three `composition_*`, `font_driver`,
  `font_driver_text_extents`, the three `glyph_row_*`, `glyph_type`, `max_used_fringe_bitmap`,
  `window_pseudo_window_p`) - i.e. the shim lane's batches ⑪/⑫ have landed their export halves;
  those rows are the integrator's regeneration to drop.

* Round 17's compile, same command: **exit 0 on the first pass**, the same **seven** warnings the
  previous rounds carry (four C's own `lockFocus`/`unlockFocus`, three `'scanFloat' was deprecated
  in macOS 10.15`) and **no new one** (the new lines are all above 4,274 and none of them is in
  the warning list); the compiler's last output line is its
  `[#DeprecatedDeclaration]: <https://docs.swift.org/compiler/documentation/diagnostics/deprecated-declaration>`
  note and the exit status is 0.  Object `/tmp/s188r17/nsterm.o` **md5
  `f119b6a67745dd27e65803c602dd2d69`**.
* `nm -g /tmp/s188r17/nsterm.o` -> **no new C export** (`grep -cE ' [Tt]
  _(ns_show_hourglass|ns_hide_hourglass|ns_get_glyph_string_clip_rect|ns_draw_underwave|ns_draw_dash|ns_fill_underline|ns_draw_text_decoration)$'`
  is 0: all seven are C `static`s, so `## ABI` (1) gains no row - the unit's 418 defined
  `T`/`t`/`D`/`d` symbols are the same set the previous rounds' exports produce).
* `nm -u /tmp/s188r17/nsterm.o` -> the slice's **45** declared names, **every one of them
  referenced** (checked name by name, 0 anomalies): the two gated xdisp.c functions,
  `_WINDOW_BUFFER_LOCAL_VALUE`, `_XFIXNAT`, the four `_Q*`, the fifteen `_glyph_string_*`
  (thirteen reads plus the two `_set_` writes), `_glyph_ascent`/`_glyph_descent`, the eleven
  `_face_*`, the three `_font_*` and the six enumerators; **679 undefined names in all** (628 after
  round 16).
* The symbol audit for round 17's 45 names (`nm -g -A` over `build/emacs-ref/src/*.o` and
  `build/emacs-ref/lib/*.o`): **1** for `_Qunbound` and **1** for `_Qunderline_minimum_offset` (both
  `S` in `build/emacs-ref/src/emacs.o` - the provided pair) and **0 for the other forty-three** -
  the member entries, the enumerators, `WINDOW_BUFFER_LOCAL_VALUE`, `XFIXNAT` and the two gated
  functions have no reference-side symbol.
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 150 C
  definition(s), 58 unported, 65 row(s); 280 C symbol import(s)` and `0 new, 7 stale, 7 Swift
  unit(s) landed`.  The seven stale rows are exactly this slice's seven definitions, i.e. the
  rows `docs/baseline/swift-units.tsv` still carries for them; the tool's `58 unported` is the
  ledger's 54 open boxes **plus** the four `recorded, not transcribed` definitions (the three dead
  `NSTRACE` names and `hide_bell`), which is why it reads four above the ledger's unticked count.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`, `read 675 files (624 .rs 33 .kt 7 .swift 6 .s 5 .cc)` (the Kotlin, `.s`
  and `.cc` files are other lanes').
* `python3 tools/check-swift-imports.py` -> `1020 import(s) in 7 Swift file(s), 5726 core
  export(s), 685 unprovided, 642 ratchet row(s), 43 failure(s)`, with
  `reference=133 reference-source=390 rust-side=1 rust-side-gated=73 system=0 unclassified=88`.
  **All forty-three `FAIL` names are this lane's** and there are **no `STALE` rows**: thirteen
  `reference-source` (the six `FACE_*` enumerators, the two `Qx_*`, `WINDOW_BUFFER_LOCAL_VALUE`,
  `get_glyph_string_clip_rects`, `font_for_underline_metrics`, `glyph_ascent`, `font_descent`),
  one `reference` (`Qunderline_minimum_offset`), one `rust-side` (`face_underline`) and
  twenty-eight `unclassified` (the member entries).  `XFIXNAT` and `Qunbound` are the two names of
  the forty-five that are **not** among them - both already carry ratchet rows - and
  `Qunbound`/`Qunderline_minimum_offset` are the two the audit found provided.
* `python3 tools/check-mirror-dupes.py` -> `20 duplicate declaration(s), 111 ratchet row(s), 91
  failure(s)` on one run and `56 / 111 / 55` on another: the failures are all `STALE` rows naming
  **other units** (`femacs/src/dispnew.rs`, `femacs/src/xdisp.rs`, `femacs/src/xfaces.rs` -
  `AUTO_FRAME_ARG`, `CHECK_FRAME`, `SET_FRAME_GARBAGED`, `WINDOW_TAB_LINE_HEIGHT`,
  `WINDOW_TOP_EDGE_Y`, `WINDOW_XFRAME` and siblings, "from femacs/src/include/frame.rs is gone"),
  i.e. an in-flight lane regenerating that mirror while this round ran - and
  `grep -c nsterm docs/baseline/mirror-dupes.tsv` is **0**, so no row of the new duplicate-name
  ratchet names this unit and the gate does not touch this lane.

* Round 18's compile, same command: **exit 0 on the first pass**, the same **seven** warnings the
  previous rounds carry (four C's own `lockFocus`/`unlockFocus`, three `'scanFloat' was deprecated
  in macOS 10.15`) and **no new one**; the compiler's last output line is its
  `[#DeprecatedDeclaration]` note and the exit status is 0.  Object `/tmp/nsterm18.o` **md5
  `8357eb1d9e1e0d40a013917bb4c970b`**.  The three citation fixes below are `///`-only edits made
  after this compile (`git diff` shows the appended block plus the header's `//` lines and nothing
  else), so the reading stands for the code as delivered.
* `nm -g /tmp/nsterm18.o` -> **no new C export** (a count of ` [Tt] _(ns_draw_box|
  ns_setup_relief_colors|ns_draw_relief|ns_dumpglyphs_box_or_relief|ns_maybe_dumpglyphs_background)$`
  is 0: all five are C `static`s, so `## ABI` (1) gains no row).
* `nm -u /tmp/nsterm18.o` -> the slice's **44** declared names, **every one of them referenced**
  (checked name by name, 0 anomalies); **732 undefined names in all** (679 after round 17): the 44
  plus the five AppKit selectors whose first use in this unit is this slice (`grayColor`,
  `highlightWithLevel:`, `shadowWithLevel:`, `removeAllPoints`, `fillRect:`) and a few
  Swift-runtime helpers (`Optional.flatMap`'s specialization among them).
* The symbol audit for round 18's 44 names (`nm -g -A` over `build/emacs-ref/src/*.o` and
  `build/emacs-ref/lib/*.o`): **1** for `window_box_right` (`T` in `build/emacs-ref/src/xdisp.o` -
  the provided row) and **0 for the other forty-three** - the member/operation entries, the three
  enumerators, the four macro names and the SDK's five CoreGraphics calls have no reference-side
  symbol.
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 150 C definition(s),
  53 unported, 58 row(s); 324 C symbol import(s)` and `0 new, 5 stale, 7 Swift unit(s) landed`.
  The five stale rows are exactly this slice's five definitions - the rows
  `docs/baseline/swift-units.tsv` still carries for them, the integrator's to drop - and the tool's
  `53 unported` is the ledger's 49 open boxes **plus** the four `recorded, not transcribed`
  definitions (the three dead `NSTRACE` names and `hide_bell`).
* `python3 tools/check-swift-imports.py` -> `1062 import(s) in 7 Swift file(s), 5733 core
  export(s), 721 unprovided, 685 ratchet row(s), 36 failure(s)`, with
  `reference=133 reference-source=395 rust-side=1 rust-side-gated=73 system=0 unclassified=119`.
  **All thirty-six `FAIL` names are this lane's** - five `reference-source` (`DRAW_MOUSE_FACE`,
  `FACE_SIMPLE_BOX`, `FACE_RAISED_BOX`, `FONT_TOO_HIGH`, `WINDOW_RIGHT_EDGE_X`) and thirty-one
  `unclassified` (the member/operation entries) - and not one of round 17's forty-three is left in
  that list.
* `python3 tools/check-citations.py` -> the **first** reading was `672 file(s) scanned, 3 drift,
  0 range, 349 external, 33 allowed, 0 stale`, with all three rows this unit's new comments (a
  cited line whose first following snippet lived elsewhere: `void *` against src/nsterm.h 943-945,
  `HAVE_WINDOW_SYSTEM` against src/xdisp.c 1450, `enum draw_glyphs_face` against
  src/dispextern.h 1353).  Each was re-worded so the snippet after the number is the name that
  lives on the cited line (`relief_background_color`, `window_box_right`, `DRAW_MOUSE_FACE`), and
  the tool's last line is `674 file(s) scanned, 0 drift, 0 range, 349 external, 33 allowed,
  0 stale`.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`, `read 681 files (624 .rs 39 .kt 7 .swift 6 .s 5 .cc)` (the Kotlin, `.s`
  and `.cc` files are other lanes', and the two counts moved under this round).
* `python3 tools/check-mirror-dupes.py` -> `18 duplicate declaration(s), 18 ratchet row(s),
  0 failure(s)` - the in-flight lane's stale rows the round-17 block records are gone this round -
  and `grep -c nsterm docs/baseline/mirror-dupes.tsv` is **0**, so no row of that ratchet names
  this unit.

* Round 19's compile, same command: **exit 0**, the same **seven** warnings the previous rounds
  carry and no new one (**14** `warning:` lines: the two `lockFocus`/two `unlockFocus`
  deprecations and the three `'scanFloat' was deprecated in macOS 10.15`); object
  `/tmp/nsterm19.o`, **md5 `ac18810766ccc2fdce3e02bddd606100`**.  **This round needed two
  compiles**: the first one's single error was
  `error: 'appendTransform' has been renamed to 'append(_:)'` on C 4343's message, and the fix
  touched one `///` line and one code line - the second run is the reading quoted above, and the
  rename is the only one this slice met.
* `nm -g /tmp/nsterm19.o` -> **no new C export**: `grep -cE ' T _(ns_draw_image_relief|
  ns_dumpglyphs_image)$'` is 0 (both are C `static`s); the object's `T _` count is 351, so
  `## ABI` (1) gains no row.
* `nm -u /tmp/nsterm19.o` -> the slice's **27** declared names, **every one of them referenced**
  (checked name by name, 0 anomalies - the first pass of that check used a pattern that required a
  leading space and reported all 27 missing, a bug in the check, not in the object); **762
  undefined names in all** (732 after round 18).
* The symbol audit for round 19's 27 names (`nm -g -A` over `build/emacs-ref/src/*.o` and
  `build/emacs-ref/lib/*.o`): **0 symbols for all twenty-seven** - the member/operation entries,
  the five enumerators, the four globals, `face_id`, `image_ascent` and the four `EmacsImage`
  cross-lane names have none (src/image.c is not compiled, and the globals are `globals` fields).
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 150 C definition(s),
  51 unported, 53 row(s); 347 C symbol import(s)` and `0 new, 2 stale, 7 Swift unit(s) landed`.
  The two stale rows are exactly this slice's two definitions - the rows
  `docs/baseline/swift-units.tsv` still carries, the integrator's to drop - and `51 unported` is
  the ledger's 47 open boxes **plus** the four `recorded, not transcribed` definitions.  (Before
  this slice: `53 unported`, `324` imports.)
* `python3 tools/check-swift-imports.py` -> `1088 import(s) in 7 Swift file(s), 5734 core
  export(s), 746 unprovided, 721 ratchet row(s), 27 failure(s)`, with
  `reference=132 reference-source=408 rust-side=1 rust-side-gated=73 system=0 unclassified=132`.
  Twenty-six of the twenty-seven failures are this slice's (thirteen `reference-source`, thirteen
  `unclassified`); the twenty-seventh is another lane's stale `try_window` row, and
  `image_pixmap` is not among them because the ratchet already carries a row for it
  (`docs/baseline/swift-import-pending.txt` 583).  The reading before this slice, same tool and
  tree, was `1062 / 5734 / 720 / 721 / 1` with `reference=132 reference-source=395 rust-side=1
  rust-side-gated=73 system=0 unclassified=119`, so the slice's delta is +26 imports, +13
  `reference-source` and +13 `unclassified` - and the ratchet itself is the integrator's to write.
* `python3 tools/check-citations.py` -> `675 file(s) scanned, 0 drift, 0 range, 350 external,
  33 allowed, 0 stale`.  The one new `external` is NSImage.h 133 (the SDK header the
  `EmacsImage_drawInRect` row cites), and two first-draft citations were re-worded after the
  first run (`src/dispextern.h 1964` + `TAB_BAR_FACE_ID`, `src/xdisp.c 38965` +
  `Vtool_bar_button_margin`), so each cited line now ends the sentence it belongs to - the
  gate wants the snippet that *follows* a cited number to live on the cited line.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures` - unchanged by this slice.
* `python3 tools/check-mirror-dupes.py` -> `23 duplicate declaration(s), 23 ratchet row(s),
  0 failure(s)`, and `grep -c nsterm docs/baseline/mirror-dupes.tsv` is **0**, so no row of that
  ratchet names this unit.  (The same tree read `26 / 18 / 10` before this slice - the drop is
  other lanes' rows being regenerated while this round ran, not this slice's doing.)

* Round 20's compile, same command: **exit 0**, the same **seven** warnings the previous rounds
  carry and no new one (`grep -cE '^[^ ]+:[0-9]+:[0-9]+: warning:'` is 7: the two
  `lockFocus`/two `unlockFocus` deprecations, all four before C 1500, and the three
  `'scanFloat' was deprecated in macOS 10.15` at C 2594-2596; the tool's rendering repeats the word
  in seven caret lines, which is why `grep -c 'warning:'` reads 14).  Object `/tmp/nsterm20.o`,
  **md5 `c5e4e2256caee24abde2bd3818eb624b`**.  **This round needed three compiles**: the first's
  five errors were all one missing declaration (`Vglyphless_char_display` - the gate printed
  `cannot find 'Vglyphless_char_display' in scope`), which the second run fixed; the third
  recompiled **comment-only** changes (six function ranges and the section banner were off by the
  blank lines around each `static void`), and it produced a **byte-identical object** to the second
  run - the md5 above is both - so the three runs are one reading.
* `nm -g /tmp/nsterm20.o` -> **no new C export**: `grep -cE ' T _(ns_draw_stretch_glyph_string|
  ns_draw_glyph_string_foreground|ns_draw_composite_glyph_string_foreground|
  ns_draw_glyphless_glyph_string_foreground|ns_set_glyph_string_gc|ns_draw_glyph_string)$'` is 0
  (all six are C `static`s); the object's `T _` count is 375, which counts the Swift-mangled
  globals too, so `## ABI` (1) gains no row.
* `nm -u /tmp/nsterm20.o` -> the slice's **60** declared names, **every one of them referenced**
  (checked name by name, 0 anomalies).  The check has a trap worth recording: this toolchain's
  `nm -u` prints the bare names with **no leading `U`**, so a comparison that expects one reports
  all sixty missing - round 19's Evidence records the same shape with a leading space.  The object
  has **835 undefined names in all** (762 after round 19).
* The symbol audit for round 20's 60 names (`nm -g` over `build/emacs-ref/src/*.o` and
  `build/emacs-ref/lib/*.o`, 5,018 names): exactly **three** of the sixty hold a symbol -
  `window_box_left`, `window_box_left_offset` and `cursor_in_mouse_face_p`, all `T` in
  `xdisp.o` - and **57 have none** (the member and operation entries, the macro/enumerator names,
  the two `globals` fields, `prepare_face_for_display`, the gated `get_glyph_string_clip_rect`, and
  `x_draw_xwidget_glyph_string`, whose file this build does not compile).
* `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsterm.swift: 150 C definition(s),
  45 unported, 51 row(s); 405 C symbol import(s)` and `0 new, 6 stale, 7 Swift unit(s) landed`.
  The six stale rows are exactly this slice's six definitions - `docs/baseline/swift-units.tsv`'s
  rows for them, the integrator's to drop - and `45 unported` is the ledger's 41 open boxes **plus**
  the four `recorded, not transcribed` definitions.  (Before this slice, same tool: `51 unported`,
  `347` imports.)
* `python3 tools/check-swift-imports.py` -> `1145 import(s) in 7 Swift file(s), 5734 core
  export(s), 801 unprovided, 746 ratchet row(s), 55 failure(s)`, with
  `reference=133 reference-source=435 rust-side=1 rust-side-gated=73 system=0 unclassified=159`.
  The slice's delta is +57 imports and +55 unprovided: of its sixty names, three
  (`FRAME_COLUMN_WIDTH`, `LGSTRING_GLYPH`, `XCHAR_TABLE`) already carry ratchet rows and two
  (`window_box_left`, `window_box_left_offset`) are provided by the core, which is why the class
  deltas are `reference-source` +27, `unclassified` +27 and `reference` +1.  Before this slice, same
  tool and tree: `1088 / 5734 / 746 / 746 / 0` with `reference=132 reference-source=408 rust-side=1
  rust-side-gated=73 system=0 unclassified=132`.
* `python3 tools/check-citations.py` -> `675 file(s) scanned, 0 drift, 0 range, 395 external,
  33 allowed, 0 stale`.  This slice's one new `external` is NSFont.h 147 (the `NSGlyph` typedef its
  buffer's width comes from); the other 44 new `external` rows are other lanes'.  Four first-draft
  citations were re-worded before the compile (`src/font.h 683-684`, `src/font.h 667`,
  `src/lisp.h 2081`, `src/nsterm.h 1049`), so the snippet that follows each cited number now lives
  on the cited line - the gate's drift rule.  Mid-round the same tree read `5 drift / 4 stale` from
  two other lanes' in-flight mirror files (their rows named only
  `femacs/src/include/coding.rs` and `femacs/src/include/dispextern.rs`); those lanes cleared them,
  and the reading above is the tree at rest.
* `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures` - unchanged by this slice.
* `python3 tools/check-ledger-ticks.py` -> `235 ledger(s) bound (a function tick to compare),
  0 failure(s), 0 carried by another file, 355 row(s) with no function tick ..., 0 row(s) without a
  ledger` - unchanged by this slice (the six ticks it adds bind to the six landed functions).
* `python3 tools/check-mirror-dupes.py` -> `18 duplicate declaration(s), 18 ratchet row(s),
  0 failure(s)`, and `grep -c nsterm docs/baseline/mirror-dupes.tsv` is **0**, so no row of that
  ratchet names this unit.

- **Ruling (3.255, integrator) on this file's macro-name entries.**  Two halves, and the criterion
  is whether the macro's body can be spelled in Swift: **(B) first** - a body that is a pure AppKit
  expression, arithmetic, or a member read of a record whose layout this file already declares must
  be *expanded at the use site* (this round's `FRAME_NS_TITLEBAR_HEIGHT`/`FRAME_TOOLBAR_HEIGHT` are
  the worked example), because importing a macro name puts a symbol in the frozen record that C
  does not have (P-08).  **(A) only when the expansion needs core-internal facts** - the
  `Lisp_Object` predicates/accessors, `globals.h` entries, enumerators, cross-record INLINEs: there
  the core exports a same-name C-ABI entry from `femacs/src/abi_shim.rs` and the ratchet row stays
  until it does.  The audit recorded above (24 macro names, 0 symbols in `build/emacs-ref/**/*.o`)
  is what puts this file's remaining macro entries in class A.

### Round 21: the `EmacsApp` class's first five methods (C 6225-6400)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`** (this lane's compile gate,
  the form `docs/ports/nsselect.md` uses): **exit 0**, **0 errors**, the same **seven** warnings
  every round since C 1500 carries (four `lockFocus`/`unlockFocus` deprecations, three
  `'scanFloat' was deprecated in macOS 10.15`), and **no new one**.  Object `/tmp/nsterm21.o`,
  **446,512 bytes**, md5 `6820bba82f832c7708019df040cf6b5f`.  **The first compile failed with one
  error** - `value of optional type 'NSEvent?' must be unwrapped to a value of type 'NSEvent'` at
  C 6288's `sendEvent` call - because Swift's `nextEvent(matching:until:inMode:dequeue:)` returns
  an optional where C's `NSEvent *event` is a bare pointer; the port unwraps it, and the comment
  beside the call records why the unwrap cannot fire (`untilDate` is `distantFuture`).
* **`python3 tools/check-swift-units.py`**: **`0 new, 0 stale, 7 Swift unit(s) landed`**, with
  `femacs/platform/macos/nsterm.swift: 150 C definition(s), 45 unported, 45 row(s); 409 C symbol
  import(s)` - every number unchanged from round 20, which is the expected reading: an
  Objective-C *method* is not one of C's functions, so this round's five methods sit outside the
  reader's subject, and the two `@_silgen_name` imports it adds (`ns_run_ascript` and
  `ns_run_file_dialog`, both defined in `nsfns.swift` over `src/nsfns.m` 2447 and 1726) are imports
  and never coverage.  The ledger's Objective-C boxes are what this round's coverage rests on.
* **No `nm` audit and no new C export**: the five additions are Objective-C selectors, so the
  unit's exported surface is unchanged and the symbol audit that rounds 2-20 ran has nothing new
  to read.

### Round 22: `ns_send_appdefined`, `EV_TRAILER`/`EV_TRAILER2`, and the two `EmacsApp` methods that call it

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  and **eight** warnings against the previous rounds' seven.  The new one is
  `'context' was deprecated in macOS 10.12: This method always returns nil` at C 5074's
  `context: NSApp.context` - C's own `[NSApp context]`, kept as the reference writes it (the
  `lockFocus` precedent: the port keeps the call and records the deprecation instead of silencing
  it).  Object `/tmp/nsterm22.o`, **472328 bytes**, md5 `738b92bd18467f2da42e06ee24ecddbb`.
* **`python3 tools/check-swift-units.py`**: `femacs/platform/macos/nsterm.swift: 150 C
  definition(s), **44** unported, 44 row(s); 411 C symbol import(s)` - one fewer unported than
  round 21's 45, and **one stale row was cleared**: `docs/baseline/swift-units.tsv` named
  `ns_send_appdefined`, and the gate reported exactly that row as `stale` the moment the function
  landed, which is the gate working as designed.  Final reading: `0 new, 0 stale, 7 Swift unit(s)
  landed`.
* **What this round unlocks**: the two `Macros defined` boxes round 2 opened (`EV_TRAILER` C 448
  and `EV_TRAILER2` C 455) close here - the first boxes this unit has closed for a *macro* rather
  than for a function.  Both were held "to C 5013, where their two callees exist", which is where
  they land, and the three entries they needed (`Qt`, the `Vinhibit_quit` data symbol and
  `kbd_buffer_store_event_hold`) are imports in the lane's three established shapes.
* **`check_native_fs` (C 5088) is the row that stays**: its body calls
  `[view updateCollectionBehavior]` (C 5103), a method of `EmacsView`, which lands at C 7039 - so
  that function is held on a class exactly the way `ns_send_appdefined` was held on `EmacsApp`.

### Round 23: `EmacsApp`'s four notification methods (C 6556-6613)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight round 22 ended with, so **no new warning** this round.
  Object `/tmp/nsterm23.o`, **477184 bytes**, md5 `3f25b82186a480e057d595e645bb4dca`.
* **`python3 tools/check-swift-units.py`**: `femacs/platform/macos/nsterm.swift: 150 C
  definition(s), 44 unported, 44 row(s); **415** C symbol import(s)` - the unported count is
  unchanged by construction (these four are Objective-C methods, outside that reader's subject),
  and the three imports it adds - `Qpre_sleep`, `Qpost_wake` and `kbd_buffer_store_event` - plus
  the cross-unit `macfont_update_antialias_threshold` account for the 411 -> 415.  Reading: `0 new,
  0 stale, 7 Swift unit(s) landed`.
* **The bitfield writer**: `input_event_set_kind` is this unit's second reader/writer of that
  field (`input_event_kind` above is the reader).  Both exist because the port's view of `struct
  input_event` packs C's `kind`/`part` bitfields into one `UInt32` (`src/termhooks.h` 374/377), and
  C's two uses here are a *write* (C 6571/6583's `ie.kind = SLEEP_EVENT`) whose `part` half must
  survive - which is what the mask keeps.
* **What stays held**: `ns_read_socket_1` (C 5110) and `ns_select_1` (C 5203) both call
  `check_native_fs ()` from their live `NS_IMPL_COCOA` arm (C 5126 and C 5217), and that function
  calls `[view updateCollectionBehavior]`, a method of `EmacsView` (C 7039) - which needs
  `EmacsWindow`'s class for its own `nonfs_window` ivar (`src/nsterm.h` 482) and declares protocol
  conformances (`NSTextInput`, `NSTextInputClient`, `NSWindowDelegate`, `src/nsterm.h` 466) whose
  required methods Swift enforces.  Those two facts are why the event-loop pair is the next round's
  subject and not this one's: no `EmacsView` shell can compile without deciding that class's
  conformance question first.

### Round 24: `EmacsWindow`'s class record and five of its methods (C 9756)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22 and 23 carry, so **no new warning**.  Object
  `/tmp/nsterm24.o`, **504,048 bytes**, md5 `af13a351891674e7608337aa8c3b18e0` (the
  citation repair below moved it from the first compile's 503,392 / `3794318feaaccaea07816cee1dab332f`: a
  `///` comment reaches the object, so the reading is taken after the repair).
* **The first compile failed with four errors**, all of them Swift-vs-AppKit, and all four are
  recorded in the source rather than worked around: `'canBecomeKeyWindow' has been renamed to
  'canBecomeKey'` and `'canBecomeMainWindow' has been renamed to 'canBecomeMain'` (the apinotes
  rename this unit already reads for `NSAppKitVersionNumber10_10`, class B - no symbol is
  invented), plus `'init(coder:)' is unavailable` with `must call a designated initializer of the
  superclass 'NSWindow'`, which the class answers by spelling `NSWindow`'s designated initializer
  out.
* **`python3 tools/check-swift-units.py`**: `0 new, 0 stale, 7 Swift unit(s) landed`, with
  nsterm.swift at 150 C definitions / 44 unported / 415 C symbol imports - every number unchanged,
  which is what an Objective-C-method round reads as through that gate.
* **One citation drift, repaired**: this round's first write cited `nsterm.h 421-424` on the same
  line as `borderWidth`, and `tools/check-citations.py` reported it as `drift` (the name belongs to
  `nsterm.h` 442, the declaration the reference carries at that line).  Splitting the sentence and
  giving `borderWidth` its own citation clears it: `676 file(s) scanned, 0 drift, 0 range, 2414
  external, 36 allowed, 0 stale`.
* **Two deferrals, by name**: the four drag-and-drop ivars (`src/nsterm.h` 426-429; `dnd_mode`'s
  type is `ns_return_frame_mode`, which `nsselect.swift` 104 already spells, so they land with the
  drag methods that read them) and `dealloc` (C 9912-9926, which Swift forbids overriding - ARC
  owns the three releases C writes).

### Round 25: `EmacsView`'s class record, `isFullscreen`, `updateCollectionBehavior` and `check_native_fs`

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-24 carry, so **no new warning**.  Object
  `/tmp/nsterm25.o`, **563696 bytes**, md5 `fc27aa476b41e28d3c7b551b1d48ab62`.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **43** unported,
  43 row(s); 415 C symbol import(s)` - one fewer than round 24's 44, and **one stale row cleared
  again**: `docs/baseline/swift-units.tsv` named `check_native_fs`, and the gate reported exactly
  that row as `stale` the moment the function landed (the same reading round 22 got for
  `ns_send_appdefined`).  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this unblocks**: `ns_read_socket_1` (C 5110) and `ns_select_1` (C 5203) call
  `check_native_fs ()` from their live arm (C 5127 and C 5218), and that call now has a target -
  the C 5109-5395 pair is the next round's subject.  Four ledger boxes are ticked (210 -> 214).
* **The named deferral**: `EmacsView`'s protocol list (`src/nsterm.h` 466's `<NSTextInput,
  NSTextInputClient, NSWindowDelegate>`).  Swift enforces every required method of a conformed
  protocol at the declaration site, so the conformance cannot land with the record; it lands with
  the method batch that implements those requirements.  Until then the class's own text is what
  the reference has - the record and the three methods above - and no method below reads a
  protocol requirement.

### Round 26: `ns_select_1`, `ns_select` and `ns_run_loop_break` (C 5203-5385)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - back to the level rounds 22-25 carry, so **no new warning** once the
  intermediate ones were cleared, and `tools/check-own-voice.py`'s one reading of this round is repaired in the same commit (a paraphrase that used the bare product name became "the reference", with C's own sentence carried as a citation).  Object `/tmp/nsterm26.o`, **575,784 bytes**, md5 `e8d700136a0e6e5ea7de10d44d832b66` - taken after that repair, a comment reaching the object the way round 24's did.
* **Five errors on the first compile and one new warning after them**, all repaired in place:
  `SELECT_HAVE_READ`/`_WRITE`/`_TMO` (C 323's enum, now this unit's own entries beside the statics
  they feed) were missing; the `NSTimer` factory's Swift label is `timeInterval:`; `retain` has no
  Swift counterpart (`Unmanaged.passRetained` returns a wrapper, and the run loop already holds the
  timer, which is what C's `retain` buys); and passing `NSApp` (an implicitly-unwrapped optional)
  to the factory's `Any` target earned `coercion of implicitly unwrappable value … does not
  unwrap`, cleared by unwrapping it - C's argument is a bare pointer.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **40** unported,
  40 row(s); 418 C symbol import(s)` - three fewer unported than round 25's 43, and **three stale
  rows cleared**: the manifest named `ns_select_1`, `ns_select` and `ns_run_loop_break`, and the
  gate named exactly those three the moment they landed.  Final: `0 new, 0 stale, 7 Swift unit(s)
  landed`.
* **What remains in the event loop's neighbourhood**: `ns_read_socket_1` (C 5110) and
  `ns_read_socket` (C 5195).  They call `(EmacsApp *) NSApp`'s `-openFile:` (C 6433) and
  `-fulfillService:withArg:` (C 6891), both of which write `emacs_event->kind = NS_NONKEY_EVENT` -
  an `enum event_kind` member whose value this unit must take from a source that can decide it
  (the frozen macro table cannot; the Rust mirror carries the enum under `HAVE_NS`).  **Round 28
  settles that value and lands all five**: `NS_NONKEY_EVENT` is counted out of the C enum's
  declaration order under Cocoa's configuration and recorded as the unit's first **UNVERIFIED**
  constant, and `-terminate:` (C 6615), `-openFile:` (C 6433), `-fulfillService:withArg:` (C 6891),
  `ns_read_socket_1` (C 5110) and `ns_read_socket` (C 5195) land with it.  The covered range is now
  **C 60-5001, C 5012-5084, C 5109-5199, C 5203-5385, C 6433-6451, C 6556-6613, C 6615-6631,
  C 6769-6781 and C 6891-6909**.

### Round 27: `EmacsApp`'s termination and file-open handlers (C 6633-6758)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-26 carry, so **no new warning**.  Object
  `/tmp/nsterm27.o`, **604,976 bytes**, md5 `1e71802d7b4e842a599cfd7a07f50ca0`.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **38** unported,
  38 row(s); **419** C symbol import(s)` - two fewer unported than round 26's 40, and **two stale
  rows cleared** (`runAlertPanel` and `not_in_argv`, the two C *functions* this round lands; the
  nine Objective-C selectors sit outside that gate's subject).  Final: `0 new, 0 stale, 7 Swift
  unit(s) landed`.  `tools/check-own-voice.py` reads 0 failure(s).
* **Four imports, each in a shape the lane already uses**: `ns_confirm_quit` (this file's own
  `DEFVAR_BOOL` data symbol, the `ns_auto_hide_menu_bar` reading), `initial_argc`/`initial_argv`
  (`src/emacs.c`'s command line) and `strcmp` (the `fputs` story).  `dockMenu` is the existing
  untyped stand-in (C 205) for the `EmacsMenu` class that has not landed, so the dock-menu getter
  hands back the menu that pointer points at.
* **The one deferral, by name**: `-terminate:` (C 6615-6631) writes
  `ie.kind = NS_NONKEY_EVENT` (C 6625) - the same `enum event_kind` member the `ns_read_socket_1`
  pair needs.  The frozen macro table cannot decide that value because C gates it on `HAVE_NS`
  (`src/termhooks.h` 273-279), so `terminate:` and the read-socket pair land together once it has
  a home; the ledger's box for `terminate:` records exactly that.




### Round 28: `NS_NONKEY_EVENT`'s value, `ns_read_socket_1` and `ns_read_socket` (C 5109-5199, C 6433-6451, C 6615-6631, C 6891-6909)

* **The value this round had to settle first**: `NS_NONKEY_EVENT`.  C gates the member on `HAVE_NS`
  (`src/termhooks.h` 273-279), the frozen macro table has neither `HAVE_NS` nor the `MAC_OS_X_*`
  values, and neither mirror defines it (the Rust side carries it only inside
  `#[cfg (feature = "have_ns")]` arms, which the frozen configuration never compiles).  C's enum
  gives no initialisers, so the value is the declaration order counted under the Cocoa build's
  configuration: `NS_TEXT_EVENT` 29 and `NS_NONKEY_EVENT` **30**.  The count is calibrated against
  the one value that is machine-checked - `SLEEP_EVENT = 28`
  (`femacs/src/include/termhooks.rs` 55) - which is what the same count yields for the frozen
  configuration.  **This is the unit's first UNVERIFIED constant**: there is no Cocoa build here to
  check it against the reference, and the entry above records that rather than hiding it.
* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-27 carry, so **no new warning**.  Object
  `/tmp/nsterm28.o`, **620,952 bytes**, md5 `51e13f5dd66d43807294200ea901e25b`.  The first compile failed with one error,
  `overriding declaration requires an 'override' keyword`, because `-terminate:` overrides
  `NSApplication`'s own method; the override is spelled and recorded in the source.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **36** unported,
  36 row(s); **420** C symbol import(s)` - two fewer unported than round 27's 38, and **two stale
  rows cleared** (`ns_read_socket_1` and `ns_read_socket`, the two C *functions* this round lands).
  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this closes**: the event-loop entry pair (`ns_read_socket_1`/`ns_read_socket`) and the
  three `EmacsApp` members that were bound to it (`-terminate:`, `-openFile:`,
  `-fulfillService:withArg:`).  Five ledger boxes are ticked (228 -> 233), and the covered range
  now spans C 60-5001, C 5012-5084, C 5109-5199, C 5203-5385, C 6433-6451, C 6556-6613,
  C 6615-6631, C 6769-6781 and C 6891-6909.



### Round 29: `showPreferencesWindow:`, `newFrame:` and `fd_handler:` (C 6402-6429, C 6783-6870)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-28 carry, so **no new warning**.  Object
  `/tmp/nsterm29.o`, **631,608 bytes**, md5 `ba37f082a5702630d97eb6a20854bbb8`.
* **One Swift-versus-C shape, recorded in the source**: C 6825/6832 hold the *addresses* of two
  locals in `wfds`/`tmo` for `pselect`'s optional arguments.  Swift cannot assign `&x`
  (`error: '&' may only be used to pass an argument to inout parameter`, measured twice), so the
  two flags now select one of C's four call shapes at the call itself - same mutex, same values,
  same order.  `selfds[0]` likewise became the local `self_fd` at C 6839/6840/6848/6850, where C
  indexes the array.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), 36 unported,
  36 row(s); **422** C symbol import(s)` - the unported count is unchanged because these three are
  Objective-C selectors, and the two new imports are the `fd_set` family members above.  Final:
  `0 new, 0 stale, 7 Swift unit(s) landed`; `tools/check-own-voice.py` reads 0 failure(s).
* **Three boxes are ticked** (233 -> 236).  `fd_handler:` is the run loop's own worker - the thread
  `ns_term_init` starts - so the event loop now has both its entry pair (round 28) and its select
  side (round 26) in the port.



### Round 30: `applicationDidFinishLaunching:` and `requestService:` (C 6490-6554, C 6881-6887)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-29 carry, so **no new warning**.  Object
  `/tmp/nsterm30.o`, **643,776 bytes**, md5 `0fb82a0b8c9224c5241f16b3bf27081f`.
* **Four errors on the first compiles, all of them Swift's own naming, all recorded in the source**:
  `NSApplication` has no `antialiasThresholdChangedNotification`, and the compiler then names the
  spelling it does have (`'NSAntialiasThresholdChangedNotification' has been renamed to
  'NSFont.antialiasThresholdChangedNotification'`) - the apinotes reading this unit already uses for
  `canBecomeKey` and `NSAppKitVersionNumber10_10`; `'NSFileHandle' has been renamed to
  'FileHandle'`; and `UInt (FD_SETSIZE)` does not assign to `rlim_t` (`UInt64`), which the cast
  spells.
* **Four imports**: `CFSocketGetTypeID`/`CFFileDescriptorGetTypeID` (CoreFoundation's two `CFTypeID`
  entries), `build_string` (the port's `femacs/src/alloc.rs`), `EmacsImage_allocInitFromFile` (the
  `<Class>_<selector>` C-ABI entry round 18 set for that class) and the selection lane's
  `stringWithLispString`/`ns_string_from_pasteboard` pair.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), 36 unported,
  36 row(s); **428** C symbol import(s)` - the unported count is unchanged (both are selectors),
  and the six new symbols (`CFSocketGetTypeID`, `CFFileDescriptorGetTypeID`, `build_string`,
  `EmacsImage_allocInitFromFile`, `stringWithLispString`, `ns_string_from_pasteboard`) account for
  422 -> 428.  Final: `0 new, 0 stale, 7 Swift unit(s)
  landed`; `tools/check-own-voice.py` reads 0 failure(s).  Two boxes are ticked (236 -> 238).



### Round 31: `EmacsWindow`'s ordering methods (C 10043-10122) and `EmacsApp`'s last box

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-30 carry, so **no new warning**.  Object
  `/tmp/nsterm31.o`, **653,152 bytes**, md5 `75b0f294cee40095ea1996af91ed28c8`.
* **Three errors on the first compiles, all of them this round's own doing and all recorded**: the
  three methods were first inserted inside `EmacsApp` (the anchor was that class's
  `requestService:`), which the compiler read as `method does not override any method from its
  superclass` plus `cannot find 'parentWindow' in scope`; they moved to `EmacsWindow`, where they
  belong.  Then `'parentWindow' has been renamed to 'parent'` (the apinotes reading again) and
  `value of optional type '[NSWindow]?' must be unwrapped`, which is C's `NSArray` defaulting.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **35** unported,
  36 row(s); 428 C symbol import(s)` - one fewer unported than round 30's 36 (the C function
  `nswindow_orderedIndex_sort`), and its manifest row was cleared as `stale`.  Final: `0 new,
  0 stale, 7 Swift unit(s) landed`; `tools/check-own-voice.py` reads 0 failure(s).
* **`EmacsApp` is closed**: `-updateMonitors:` (C 6454-6489) is dead in this build - C opens
  `#ifdef NS_IMPL_GNUSTEP` at C 6453 and closes it at C 6489 - so its box is ticked as *recorded,
  not transcribed*, the reading this unit uses for its three `nstrace_*` rows.  Five boxes are
  ticked (238 -> 243).



### Round 32: `EmacsView`'s leaf input methods (C 7593-7767)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-31 carry, so **no new warning**.  Object
  `/tmp/nsterm32.o`, **669,824 bytes**, md5 `fc8052696523d4a1d0c8bff1f055cc9d`.
* **Six methods and the two enum constants they need**: `hasMarkedText` (C 7593),
  `markedRange` (C 7601), `validAttributesForMarkedText` (C 7725), `characterIndexForPoint:`
  (C 7759, the live `NS_IMPL_COCOA` arm of C 7757-7762), `conversationIdentifier` (C 7699) and
  `doCommandBySelector:` (C 7705).  These are the leaves of the protocol batch: each one's body
  closes over members the class already has, which is why they went first - and round 33 landed the
  ones this paragraph named as waiting: `insertText:` (C 7481), `setMarkedText:selectedRange:`
  (C 7538), `deleteWorkingText` (C 7572) and `unmarkText` (C 7613), plus the three light
  `NSTextInputClient` methods of the Cocoa arm (C 7435, C 7444, C 7451).
* **`NS_KEYLOG` is 0** (C 7241), so the three `if (NS_KEYLOG)` blocks in these bodies are the dead
  arm this unit records rather than transcribes - the `NSTRACE` reading.
* **`python3 tools/check-swift-units.py`**: `0 new, 0 stale, 7 Swift unit(s) landed`, with
  nsterm.swift at 150 C definitions / 35 unported / 428 imports - every number unchanged, which is
  what an Objective-C-method round reads as through that gate.  `tools/check-own-voice.py` reads
  0 failure(s).
* **Seven ledger boxes are ticked** (243 -> 250; the two `characterIndexForPoint:` arms are one
  selector and share the one Swift function).



### Round 33: `EmacsView`'s composing pair and three light Cocoa-arm methods (C 7435-7456, C 7481-7568, C 7572-7621)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-32 carry, so **no new warning**.  Object
  `/tmp/nsterm33.o`, **685,976 bytes**, md5 `322c7bbe4a128c5353a0212a5506dd19`.  Two errors on the first compile, both
  recorded: `overriding declaration requires an 'override' keyword` for the one-argument
  `insertText:` (it overrides `NSResponder`'s own method), and `MULTIBYTE_CHAR_KEYSTROKE_EVENT`
  missing, which is now the third of the enum constants the port's mirror fixes.
* **Seven boxes are ticked** (250 -> 257): the composing pair (`insertText:` C 7481,
  `setMarkedText:selectedRange:` C 7538), its helper (`deleteWorkingText` C 7572) and `unmarkText`
  (C 7613), plus the three light `NSTextInputClient` methods of the Cocoa arm -
  `insertText:replacementRange:` (C 7435), `setMarkedText:selectedRange:replacementRange:`
  (C 7444) and `attributedSubstringForProposedRange:actualRange:` (C 7451).
* **What this round wrote out instead of importing**: `UTF_16_HIGH_SURROGATE_P`,
  `UTF_16_LOW_SURROGATE_P` and `surrogates_to_codepoint`.  Their C bodies are `src/coding.h`
  674/680/760-768, and the port's mirror spells them as plain Rust functions
  (`femacs/src/include/coding.rs` 304/308/317) rather than C-ABI entries, so they cannot be
  reached by `@_silgen_name`; each Swift body is the macro's own expression, with the two dead
  `eassert`s of an unset `ENABLE_CHECKING` carried as the comment they are.
* **And one more UNVERIFIED constant**: `NS_TEXT_EVENT = 29`, the member beside round 28's
  `NS_NONKEY_EVENT` - same enum, same `HAVE_NS` gate, same counted-inference mark.
* **`python3 tools/check-swift-units.py`**: `0 new, 0 stale, 7 Swift unit(s) landed`, with
  nsterm.swift at 150 C definitions / 35 unported / 428 imports - unchanged, an
  Objective-C-method round's reading.  `tools/check-own-voice.py` reads 0 failure(s).
* **Still waiting**: `firstRectForCharacterRange:` (C 7651) and the Cocoa arm's two callers
  (C 7458, C 7464).  Its body reaches five macros and `ns_in_echo_area`, which is why it is a
  round of its own rather than a tail on this one.



### Round 34: `ns_in_echo_area` (C 7623-7648)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-33 carry, so **no new warning**.  Object
  `/tmp/nsterm34.o`, **691,024 bytes**, md5 `8a76d3d2b4e0775e9d6fca03735223f6`.
* **Three C functions and the imports they need**: `ns_in_echo_area_1` (C 7623),
  `ns_in_echo_area_2` (C 7632) and `ns_in_echo_area` (C 7638).  The imports are `frame_selected_window`,
  `echo_area_window`, `Qns_in_echo_area` (this file's own `DEFSYM` at C 11799, as a data symbol),
  `internal_catch_all` (whose two callbacks are `@convention(c)` types here), `safe_funcall`, and
  the four entries `nsfns.swift`/`nsselect.swift` already declare for `WINDOWP`, `SPECPDL_INDEX`,
  `specbind` and `unbind_to`, plus this unit's own view of `specpdl_ref` (the `nsselect.swift` 87 /
  `nsfns.swift` 97 reading: each unit spells what it reads).
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **32** unported,
  32 row(s); **436** C symbol import(s)` - three fewer unported than round 33's 35 (the three C
  functions above), **three stale rows cleared** by name, and the eight new symbols
  (`frame_selected_window`, `echo_area_window`, `Qns_in_echo_area`, `internal_catch_all`,
  `safe_funcall`, `WINDOWP`, `SPECPDL_INDEX`, `specbind`, `unbind_to`, `Qinhibit_quit` minus the two
  this unit already had) are the round's imports.  Final: `0 new, 0 stale, 7 Swift unit(s) landed`;
  `tools/check-own-voice.py` reads 0 failure(s).  Three boxes are ticked (257 -> 260).
* **What moved out of this round, by name**: `firstRectForCharacterRange:` (C 7651) and the Cocoa
  arm's two callers (C 7458, C 7464).  Its body reaches four macros (`FRAME_COLUMN_WIDTH`,
  `FRAME_LINE_HEIGHT`, `WINDOW_TEXT_TO_FRAME_PIXEL_X`, `WINDOW_TO_FRAME_PIXEL_Y`) whose Swift
  spellings this unit has not yet settled plus `window_box_left`/`TEXT_AREA`, and the first compile
  showed exactly that (`invalid redeclaration of 'TEXT_AREA'`, `cannot find 'FRAME_LINE_HEIGHT'`,
  `cannot find 'emacsframe' in scope` - the last because the method was written as a file-scope
  function instead of a member).  They get their own round, with those four spellings settled
  first.

### Round 35: `ns_free_pixmap`, the two display metrics, and `ns_string_to_lispmod` (C 5648-5710)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-34 carry, so **no new warning**.  Object
  `/tmp/nsterm35.o`, **695,696 bytes**, md5 `18df1c23bd60508bcd24783a908ffc78` (taken after the
  fix below, per the round 24 note about object digests following comment edits).  The unit is
  **8,334 lines**.
* **Four C definitions and the imports they need**: `ns_free_pixmap` (C 5648, the image hook's
  free entry), `ns_display_pixel_height` (C 5660), `ns_display_pixel_width` (C 5675) and
  `ns_string_to_lispmod` (C 5691).  The imports are the five modifier symbols this file's own
  `syms_of_nsterm` defines (`Qmeta`, `Qsuper`, `Qcontrol`, `Qalt`, `Qhyper`, read as data symbols
  the way `Qnone` already is) plus `strncmp` in the `fputs` shape, and `SSDATA`/`SYMBOL_NAME`,
  which C 2254/2255 already declare in this unit.
* **The one compile failure and its fix**: `cannot convert value of type 'AnyObject?' to expected
  argument type 'UnsafeMutableRawPointer?'` at the `ns_release_object (pixmap)` call (C 5651).
  The pixmap parameter was written `AnyObject?` (the round 18 `image_pixmap` reading); the call's
  callee is this unit's own `@_cdecl` entry, whose parameter is documented at C 805-811 as an
  optional untyped pointer (and which carries C's nil-receiver no-op itself).  The fix follows the
  callee rather than forcing a bridge: the parameter is `UnsafeMutableRawPointer?`, so C's body
  stays the single call C writes, and a NULL pixmap needs no arm of its own.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **28** unported,
  28 row(s); **442** C symbol import(s)` - four fewer unported than round 34's 32 (the four C
  functions above), **four stale rows cleared** by name, and the round's imports are the new
  symbol count (436 -> 442).  Final: `0 new, 0 stale, 7 Swift unit(s) landed`;
  `tools/check-own-voice.py` reads 0 failure(s).  Four boxes are ticked (260 -> 264).
* **What this round does not claim**: no run evidence (no pixmap freed, no screen list walked, no
  modifier string converted); the compile gate reaches parsing and type checking only, and
  `tools/check-swift-units.py` compares name sets, not signatures, bodies, layout or ABI.

### Round 36: `ns_default`, `ns_default_font_parameter`, and the dead GNUstep `ns_update_window_end` (C 5713-5783)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-35 carry, so **no new warning**.  Object
  `/tmp/nsterm36.o`, **700,568 bytes**, md5 `3ed96154faa125e1f9add0675271f72b`.  The unit is
  **8,419 lines**.
* **Three C definitions and the imports they need**: `ns_default` (C 5713-5738, the preferences
  reader), `ns_default_font_parameter` (C 5769-5773, empty in C) and `ns_update_window_end`
  (C 5775-5783, dead).  The imports are `ns_get_defaults_value` (src/nsterm.h 1170; defined in
  src/nsfns.m 1886, which this lane's `nsfns.swift` 2858-2865 already exports), `c_strcasecmp`
  (gnulib; this tree's own `femacs/lib/c-strcasecmp.rs` 22) and `make_float` (src/alloc.c;
  `femacs/src/alloc.rs` 2789).  `fprintf`/`stderr` were already in the unit (round 3).
* **Two compile failures and their fixes** (both named in the source): ⑴ every pointer use needed
  C's own guard made explicit - `c_strcasecmp (value!, …)`, `strtod (value!, &pos)` and
  `fprintf (…, value!)` - because a Swift `Optional` pointer does not convert where these entries
  want a plain one (measured: three `error:` lines, two of them "must be unwrapped"); the guard is
  C's `if (value)` itself, so the unwrapping is C's own control flow, not a new one.  ⑵ a local
  `@_silgen_name("strtod")` made the call **ambiguous** against Darwin's import of the same C
  symbol (`error: ambiguous use of 'strtod'`), so the unit does what it already does for
  `setlocale`/`getenv`/`setenv` (C 785-805): the available entry is called by name with no
  declaration of its own.  Unlike `fprintf`, whose Swift name is marked unavailable, `strtod`
  needs no local spelling.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **26** unported,
  26 row(s); **445** C symbol import(s)` - two fewer unported than round 35's 28 (the two
  functions that land code), **two stale rows cleared** by name, and the round's imports are the
  new symbol count (442 -> 445).  `ns_update_window_end` keeps its manifest row on purpose: no
  Swift item carries that name because the arm does not exist in this build.  Final:
  `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What moved out of this round, by name**: `ns_initialize_display_info` (C 5741-5764) needs
  eight member setters this unit does not carry yet (`resx`/`resy`/`color_p`/`n_planes`/
  `root_window`/`n_fonts`/`smallest_font_height`/`smallest_char_width`, plus the `mouse_highlight`
  member `reset_mouse_highlight` takes by address), and `ns_flush_display` (C 5785-5792) needs a
  reader for `FRAME_TERMINAL (f)`, which exists nowhere in the lane yet (`frame_set_terminal` is
  the only `terminal` accessor; `frame.rs` carries the member as a raw pointer).  Each gets its own
  round, with those spellings settled first.
* **What this round does not claim**: no run evidence (no defaults key read, no float parsed, no
  `ns_redraw_scroll_bars` reached); the compile gate reaches parsing and type checking only, and
  `tools/check-swift-units.py` compares name sets, not signatures, bodies, layout or ABI.

### Round 37: `ns_initialize_display_info` (C 5741-5764)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-36 carry, so **no new warning**.  Object
  `/tmp/nsterm37.o`, **712,856 bytes**, md5 `e5d6726aecabdf8838370b7fab226415`.  The unit is
  **8,491 lines**.
* **One C definition and the nine member entries it needs**: the definition writes five `int`
  members, two `double`s and `Window`, so this round declares `ns_display_info_set_resx`,
  `_set_resy`, `_set_color_p`, `_set_n_planes`, `_set_root_window`, `_set_n_fonts`,
  `_set_smallest_font_height` and `_set_smallest_char_width`, plus `ns_display_info_mouse_highlight`
  for C 5763's `&dpyinfo->mouse_highlight` (the member's *address*, which is what
  `reset_mouse_highlight` takes).  The two frame members reuse round 7's
  `ns_display_info_set_ns_focus_frame` and `ns_display_info_set_highlight_frame`.  `Window` is
  `typedef int Window` on this arm (src/nsgui.h 67), and the frozen layout agrees
  (`docs/baseline/ns_display_info-layout.tsv` 27: `root_window` at 92, `int color_p` at 88), so the
  root-window entry takes an `Int32` like its neighbours.
* **Four compile failures and their fixes** (all named in the source): ⑴ four AppKit names are
  renames Swift performs itself - `NSDeviceWhiteColorSpace` is `NSColorSpaceName.deviceWhite`,
  `NSCalibratedWhiteColorSpace` is `.calibratedWhite`, `NSColorSpaceFromDepth (depth)` is
  `depth.colorSpaceName` and `NSBitsPerPixelFromDepth (depth)` is `depth.bitsPerPixel` (class B);
  C's two calls to `NSColorSpaceFromDepth` stay two reads.  ⑵ C 5747-5748's nil receiver is the
  round 2 reading again, and it needed one measurement: `NSScreen.depth` is `NSWindow.Depth` in
  Swift (the `NSWindowDepth` typedef) whose raw-value initializer is **failable**, so C's own zero
  is `NSWindow.Depth (rawValue: 0)!` - a probe on this machine answers `rawValue0_is_nil: false`
  (so the unwrap cannot trap for 0), and the same probe is why the form is not written as
  `?? 0`.  ⑶ C 5758's chained assignment keeps C's order: the inner `ns_focus_frame` write comes
  before the outer `highlight_frame` one.  ⑷ the `int color_p` member takes C's integral
  conversion of a `bool` explicitly (`color_p ? 1 : 0`), because Swift has no such implicit one.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **25** unported,
  25 row(s); **454** C symbol import(s)` - one fewer unported than round 36's 26 (the definition
  above), **one stale row cleared** by name, and the round's import count rises 445 -> 454 (the
  nine member entries).  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What is left in this block, by name**: `ns_flush_display` (C 5785-5792) still needs a reader for
  `FRAME_TERMINAL (f)`; no lane has spelled it yet (`frame_set_terminal` is the only `terminal`
  accessor, and `frame.rs` carries the member as a raw pointer), so it gets its own round with
  that spelling settled first.
* **What this round does not claim**: no run evidence (no display info initialised, no screen
  queried, no mouse-highlight state reset); the compile gate reaches parsing and type checking
  only, and `tools/check-swift-units.py` compares name sets, not signatures, bodies, layout or ABI.
  The nine member entries are *declarations*: the core side that exports them is the
  `src/nsterm.h` unit's, which is not landed yet - the same standing state round 7's member
  entries are in.

### Round 38: `ns_flush_display`, the empty `ns_delete_display`, and the object nobody tracked (C 5785-5875)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-37 carry, so **no new warning**, and this round
  needed **no fix round**: it compiled on the first attempt.  Object `/tmp/nsterm38.o`,
  **713,480 bytes**, md5 `6819aed85a22bdec8a6b17db5990c2d7`.  The unit is **8,519 lines**.
* **Two C definitions and the one entry they need**: `ns_flush_display` (C 5785-5792, whose body is
  `EVENT_INIT` plus one discarded `ns_read_socket_1` result) and `ns_delete_display` (C 5871-5875,
  whose body is C's `/* TODO...  */` alone, so the Swift body is empty and the parameter unread -
  the `ns_default_font_parameter` reading).  `ns_flush_display` waited on `FRAME_TERMINAL (f)`,
  which this round declares as a macro entry under the construct's own name (the
  `FRAME_DISPLAY_INFO` shape round 7 settled; `nsfns.swift` 639's `frame_set_terminal` is the same
  member's other direction).
* **The finding that changed the next action**: C 5796-5835 is the `static struct
  redisplay_interface ns_redisplay_interface` table, and **neither `tools/extract-abi.py` nor this
  ledger had a row for it** - rounds 37 and 38 walked past that block without anything to compare
  against.  It is now a hand row in `## File-scope objects` (the second member of that
  already-documented family), **unticked**, with the reason it cannot land yet: its 28 members are
  function pointers over this lane and the xdisp lane (`ns_frame_parm_handlers`, `gui_*` ×6,
  `ns_scroll_run`, `ns_after_update_window_line`, the two `NULL`s this arm takes, …), so the
  `struct redisplay_interface` spelling and those imports come first.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **23** unported,
  23 row(s); **455** C symbol import(s)` - two fewer unported than round 37's 25 (the two
  definitions above), **two stale rows cleared** by name, and the round's import count rises
  454 -> 455 (`FRAME_TERMINAL`).  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no socket read, no event record initialised
  in a live loop, nothing deleted); the compile gate reaches parsing and type checking only, and
  `tools/check-swift-units.py` compares name sets, not signatures, bodies, layout or ABI.
  `FRAME_TERMINAL` is a declaration: the core side that exports it lands with the `src/frame.h`
  unit, the standing state of round 7's macro entries.

### Round 39: the redisplay interface table (C 5801-5835)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-38 carry, so **no new warning**, and like round 38
  this one **needed no fix round**.  Object `/tmp/nsterm39.o`, **754,584 bytes**, md5
  `93927fa69f456a94ce4ec8f5217f6a11`.  The unit is **8,673 lines**.
* **The record, in the arm this unit ports.**  `struct redisplay_interface` is measured four times
  in `docs/baseline/redisplay_interface-layout.tsv`: the *frozen* configuration is **13 members /
  sizeof 104** (it leaves `HAVE_WINDOW_SYSTEM` false, and the Rust mirror
  `femacs/src/include/dispextern.rs` 3794 is that shape), while `x11`, `x11_ext` and **`ns_cocoa`**
  are **28 members / sizeof 224**.  C 5801-5835 initialises 28 members, so this unit carries its
  own view of the `ns_cocoa` shape (members at 0, 8, 16, … 216) rather than the mirror's - the same
  relationship `input_event` has to `termhooks.rs`.  Every member is a function pointer, spelled as
  an optional `@convention(c)` type (C's `NULL` is the `nil` the table's two nulls use), with
  `frame_parm_handler` as this unit's own view of src/dispextern.h 3063's typedef.
* **The table itself.**  C's order is kept entry by entry (C 5803-5834), including C's own
  comments where they say something the code does not (`generic OK`) and the two `NULL`s: C 5810's
  `update_window_begin` and C 5812's `update_window_end`, the second because C 5811 tests
  `#ifndef NS_IMPL_GNUSTEP` and this target takes the `NULL` side.  The entries are non-capturing
  closures over this unit's own functions and over seven imported `gui_*` entries of the xdisp lane
  (`docs/ports/nsfns.md` 234-236 is the same shape and the same reason: a `unsafeBitCast` of a
  Swift function to a `@convention(c)` pointer is a Swift 6.4 warning).
* **The one reshaping, named.**  C's first entry is the *address* of `ns_frame_parm_handlers`
  (src/nsterm.h 1262, defined in src/nsfns.m 1058).  A unit cannot see another unit's Swift array
  (`nsfns.swift` 2070 carries it) and Swift cannot emit the C data symbol, so the address is taken
  from the imported symbol itself, `withUnsafeMutablePointer (to: &ns_frame_parm_handlers_entry)`.
  C's value is a link-time constant; the port-side definition that would give this entry the same
  standing is the gap `docs/ports/nsfns.md`'s `## ABI` (3) already records, and it is now owed by
  two units instead of one.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **23** unported,
  23 row(s); **462** C symbol import(s)` - unported is unchanged (the table is a variable, not a C
  *function*, which is also why `extract-abi.py` never had a row for it), and the round's imports
  are the seven `gui_*` entries (455 -> 462).  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (nothing has read the table, no glyph written
  through it, no frame parameter set); the compile gate reaches parsing and type checking only; the
  record's layout is *documented* from the probe's `ns_cocoa` rows, not asserted at run time, and
  the seven `gui_*` entries plus `ns_frame_parm_handlers` are declarations whose definitions live in
  the xdisp and nsfns lanes.

### Round 40: `ns_displays_reconfigured` (C 5837-5868)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-39 carry, so **no new warning**.  Object
  `/tmp/nsterm40.o`, **760,632 bytes**, md5 `2267c28a739f4b03af29f98a05638e0a` (**taken after the
  citation fix below**).  The unit is **8,780 lines**.
* **What the definition needed** (C 5837-5868, live: C 5837 opens the `NS_IMPL_COCOA` guard and
  C 5869 closes it): ⑴ this unit's own view of `union buffered_input_event` (src/keyboard.h
  228-235) - the union's members all start at offset 0 and this arm has only the `kind` bitfield and
  `struct input_event ie` (`sie` is X11/PGTK's), so the view carries `ie` and C's `ev->ie.kind` is
  read as `input_event_kind (&ev!.pointee.ie)`, i.e. the same low 16 bits the record's own view
  documents; ⑵ the kbd buffer's three data entries - `kbd_buffer` (declared with the element type,
  because C's use here is the array's address), `kbd_fetch_ptr`, `kbd_store_ptr`, all
  `#[unsafe(no_mangle)] static mut` in the core (`femacs/src/keyboard.rs` 178-186); ⑶ `Fequal`
  (the core's globals mirror, `femacs/src/include/globals.rs` 7577); ⑷ the nsfns lane's
  `Fns_display_monitor_attributes_list` (`nsfns.swift` 3669); ⑸ `XSETTERMINAL` as a macro entry
  (src/lisp.h 1384, the `XSETFRAME` shape); ⑹ a new member reader `ns_display_info_terminal` for
  C 5865's `x_display_list->terminal` (the `ns_display_info_next` family, which the mouse-move
  block at C 6389 already walks).
* **The two enumerators, and why they are mirror-valued constants.**  `KBD_BUFFER_SIZE` (4096) and
  `MONITORS_CHANGED_EVENT` (35) are *not* in `docs/baseline/config/*.tsv` nor exported by the core,
  and C's `enum event_kind` has guarded members before `MONITORS_CHANGED_EVENT` (`HAVE_DBUS`,
  `THREADS_ENABLED`, …), so neither the frozen macro table nor counting alone decides them.  Both
  values are taken from this tree's own mirror (`femacs/src/include/keyboard.rs` 43,
  `femacs/src/include/termhooks.rs` 62) - the reading round 33's
  `MULTIBYTE_CHAR_KEYSTROKE_EVENT` established for exactly this class.
* **One compile failure and its fix**: `kbd_buffer_store_event` was already this unit's entry (the
  C 1373 block's declaration), so C 5867's call reuses it rather than redeclaring it - the fix
  removed the duplicate declaration and left a note where it stood.
* **A citation drift, and its fix** (the round 24 lesson): the first version of the
  `MONITORS_CHANGED_EVENT` note named `enum event_kind` on the line that cites `src/termhooks.h
  342`, and `tools/check-citations.py` read it as a claim that the *enum* is declared at 342
  (`drift ... enum event_kind is at termhooks.h:72`, exit 1; the second attempt hit the same rule
  with `NO_EVENT`, which is at termhooks.h:74).  The note now names only the enumerator the citing
  line is about, and the same command reads **0 drift** - the object digest above is the one taken
  after that fix.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **22** unported,
  22 row(s); **466** C symbol import(s)` - one fewer unported than round 39's 23 (the definition
  above), **one stale row cleared** by name, and the round's imports are 462 -> 466.  Final:
  `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no monitor change has fired the callback, no
  event stored in the buffer, no terminal set); the compile gate reaches parsing and type checking
  only; the two enumerator values are mirror-derived rather than measured on this machine, and
  `kbd_buffer`'s address is taken the way round 39 takes the handlers array's.

### Round 41: `ns_delete_terminal` (C 5877-5902)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-40 carry, so **no new warning**, and like rounds
  38/39 it **needed no fix round**.  Object `/tmp/nsterm41.o`, **761,520 bytes**, md5
  `4575f252497a81e0467d977f679b14f4`.  The unit is **8,828 lines**.
* **What the definition needed**: `terminal->display_info.ns` (C 5882) is already the core's own
  shim (`femacs/src/abi_shim.rs` 412, `terminal_display_info_ns`) - the first member entry of this
  lane that was provided before its Swift side landed; `image_destroy_all_bitmaps` (C 5899) is the
  core's (`femacs/src/image.rs` 1841); `block_input`/`unblock_input` (C 5891/5901) and the
  `NSTRACE` empty macro are the unit's existing shapes; `ns_delete_display` (C 5900) landed in
  round 38; and C 5896's `[NSApp setActivationPolicy: NSApplicationActivationPolicyProhibited]` is
  `NSApp.setActivationPolicy (.prohibited)`.  **One new name**: `terminal_name`, the member entry
  C 5888's guard reads.
* **The Swift-imports gate, run once for this round** (`tools/check-swift-imports.py`, the gate
  that asks whether the core provides every `@_silgen_name` the Swift units import): it reads
  `1209 import(s) in 7 Swift file(s), 5037 core export(s), 965 unprovided, 801 ratchet row(s),
  182 failure(s)`, of which nine are ratchet rows the tool now calls `STALE (satisfied)` and 173
  names it calls unclassified.  **This gate was already red before this round and is not one of the
  lane's per-round set** (`docs/baseline/swift-import-pending.txt`'s own rule: the ratchet is
  rewritten by `--write-pending`, "appended only by the integrator", and the tool reports that 159
  ratcheted names "need a ruling"), so this round does **not** rewrite it - it records the reading
  and names the debt: the member entries this lane declares for the core (`terminal_name`, the nine
  `ns_display_info_set_*` of round 37, `ns_display_info_terminal` and `XSETTERMINAL` of round 40,
  the seven `gui_*` and `ns_frame_parm_handlers` of round 39) are the class the core still owes.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **21** unported,
  21 row(s); **469** C symbol import(s)` - one fewer unported than round 40's 22 (the definition
  above), **one stale row cleared** by name, and the round's imports are 466 -> 469.  Final:
  `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no terminal deleted, no activation policy
  changed, no bitmaps destroyed); the compile gate reaches parsing and type checking only;
  `terminal_name` is a declaration whose provider the core still owes.

### Round 42: the `EmacsScroller` record and `initFrame:window:` (C 10620-10701)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-41 carry, so **no new warning**.  Object
  `/tmp/nsterm42.o`, **804,112 bytes**, md5 `1ecb1c3e047774ffac1c2a22ba7aaf78`.  The unit is
  **8,956 lines**.
* **The record** (src/nsterm.h 714-754): eleven ivars, and **two of C's names are `NSView`'s own** -
  `window` (the `NSWindow` a view is in) and `frame` (its rect), which a Swift subclass cannot
  redeclare.  They are carried as `emacs_window`/`emacs_frame`, the `emacs*` spelling this file
  already uses for C pointers of that kind (`EmacsView`'s `emacsframe`); the other nine keep C's
  names.  This is this lane's first *name* deviation of the record kind and it is declared here.
* **Three compile failures, all measured, and the decision they force**: ⑴ `override class var
  scrollerWidth: CGFloat` - `error: property does not override any property from its superclass`
  (Swift imports the superclass entry as a class *method*); ⑵ `override class func scrollerWidth()`
  - `error: cannot override 'scrollerWidth' which has been marked unavailable`; ⑶
  `@objc(scrollerWidth) class func ns_scrollerWidth()` - `error: method … with Objective-C selector
  'scrollerWidth' conflicts with method 'scrollerWidth()' from superclass 'NSScroller' with the same
  Objective-C selector`.  So the method is **omitted and recorded** - the shape `nsfns.swift`
  4867-4874 documents for `EmacsFileDelegate`'s three delegate methods, and the reason the
  omission is harmless is measured too: no reference caller reaches it.
* **`initFrame:window:` (C 10650-10701) lands whole**: C 10652's `NSTRACE` is the empty macro, C's
  three-arm `scrollerWidth` test is not part of it, the `NS_IMPL_GNUSTEP` arm at C 10666-10667 is
  dead here (this target takes C 10669's `NSViewMinXMargin | NSViewHeightSizable`,
  `[.minXMargin, .height]`), C 10679's `20 / pixel_length` keeps C's *integer* division before the
  `float` store, and C 10691's descending `for` is the `while` (the `sendEvent` block's reading of
  that shape).
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **21** unported,
  21 row(s); **469** C symbol import(s)` - unchanged from round 41, which is the expected reading
  for an Objective-C method round (`cinventory` counts C definitions, not `@implementation`
  methods).  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no scroller created, no frame read, no
  subview added); the compile gate reaches parsing and type checking only; the omitted
  `+scrollerWidth` is a gap named in `## ABI` (3), not a transcribed method.

### Round 43: `EmacsScroller`'s record-keeping methods (C 10703-10804)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-42 carry, so **no new warning**.  Object
  `/tmp/nsterm43.o`, **813,304 bytes**, md5 `bb3129da32d8656740cc4cfe6a7fb3fb`.  The unit is
  **9,086 lines**.
* **Seven methods land**: `setFrame:` (C 10703), `dealloc` (C 10719), `condemn` (C 10734),
  `reprieve` (C 10742), `judge` (C 10750), `mark` (C 10777) and `resetCursorRects` (C 10788).
* **Two compile failures and the reshape they force** (both in the source's doc comment): Swift
  imports AppKit's `-setFrame:` as the `frame` *property*, so ⑴ `override func setFrame(_:)` is
  refused (`error: method does not override any method from its superclass`) and ⑵
  `super.setFrame(newRect)` has no such member (`error: value of type 'NSScroller' has no member
  'setFrame'`).  C's body is therefore the property's **setter** (`set(newRect)`, keeping C's
  parameter name) and the getter is the superclass's, because C overrides only the setter.
* **The two other spellings this round fixes**: C's `dealloc` is Swift's `deinit` (a class cannot
  name `dealloc`; C 10730's `[super dealloc]` is implicit), and C's `NSTRACE` lines are the empty
  macro (recorded where they stand).  C 10713's `20 / pixel_length` keeps C's *integer* division,
  the same expression `initFrame:window:` has at C 10679.
* **The dead arm**: C 10796-10802's `NS_IMPL_GNUSTEP || MAC_OS_X_VERSION_MIN_REQUIRED < 101300`
  block (with its `setOnMouseEntered:` message) is not transcribed - this target is 260000.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **21** unported,
  21 row(s); **473** C symbol import(s)` - unported unchanged again (Objective-C methods are not C
  definitions), and the round's imports are 469 -> 473 (`wset_horizontal_scroll_bar`,
  `wset_vertical_scroll_bar`, `XSETWINDOW`, `mark_object`).  Final:
  `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no scroll bar resized, freed or judged, no
  cursor rect added, no object marked); the compile gate reaches parsing and type checking only;
  the three names the core still owes (`wset_*`, `XSETWINDOW`) are declarations, recorded in
  `## ABI` (3).

### Round 44: `EmacsScroller`'s position and event methods (C 10806-10927)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-43 carry, so **no new warning**.  Object
  `/tmp/nsterm44.o`, **831,912 bytes**, md5 `d12150a92426ec8d2ba3a0f5a47162e0`.  The unit is
  **9,231 lines**.
* **Four methods land**: `checkSamePosition:portion:whole:` (C 10806), `setPosition:portion:whole:`
  (C 10814), `sendScrollEventAtLoc:fromEvent:` (C 10850) and `repeatScroll:` (C 10894), plus the two
  `#define`s at the `@implementation`'s head (C 10623-10624) as file-scope `let`s.
* **One compile failure and its fix**: C's `EV_MODIFIERS (e) | down_modifier` mixes an
  `unsigned int` with the `int` enumerator, and Swift refuses the bare `|` (measured:
  `cannot convert value of type 'Int32' to expected argument type 'UInt32'`); the port spells C's
  own promotion (`UInt32 (truncatingIfNeeded: down_modifier ())`), the shape `EV_MODIFIERS2`'s
  reading already uses.
* **The one place this round writes C's nil-receiver semantics out**: `sendScrollEventAtLoc:`'s
  event may be nil in C (its `EV_MODIFIERS (e)`/`EV_TIMESTAMP (e)` then read a nil receiver's zero
  `modifierFlags`/`timestamp`), while both ported macros take a live `NSEvent`; the method's two
  reads therefore carry C's own nil branch (`EV_MODIFIERS2 (0, QCmouse ())` is exactly what
  `EV_MODIFIERS` computes from zero flags).
* **`repeatScroll:`'s two `window`s**: C 10896-10897's `[self window]` is AppKit's message to the
  view's window, *not* the class's `window` ivar - the round-42 rename (`emacs_window`) is what
  makes them distinguishable in Swift; and the timer C allocates and `retain`s is
  `Timer.scheduledTimer (…)` with this method's own `#selector`.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **21** unported,
  21 row(s); **474** C symbol import(s)` - unported unchanged (Objective-C methods are not C
  definitions), and the round's import is 473 -> 474 (`input_event_set_part`).  Final:
  `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no scroll event stored or sent, no timer
  fired, no knob moved); the compile gate reaches parsing and type checking only; `input_event_set_part`
  is a declaration the core still owes.

### Round 45: `EmacsScroller`'s `mouseDown:` (C 10927-11047)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-44 carry, so **no new warning**.  Object
  `/tmp/nsterm45.o`, **838,456 bytes**, md5 `dc438486d790b23e0e567360cb4d6b97`.  The unit is
  **9,370 lines**.
* **The method, and the two renames Swift forces** (measured, and recorded in the source's doc
  comment): `@objc override func mouseDown(_:)` is refused (`error: 'mouseDown' has been renamed to
  'mouseDown(with:)'`) and `super.mouseUp(_:)` likewise (`mouseUp(with:)`), so the port uses the
  renamed spellings - class B, the compiler's own reading - while the Objective-C selectors stay
  C's (`mouseDown:`, `mouseUp:`).
* **What C's own text decides here**: the `switch` keeps C's order and its five live cases
  (C 10945-10950's `NSScrollerDecrementLine`/`NSScrollerIncrementLine` pair is dead on this
  target - `NS_IMPL_GNUSTEP || MAC_OS_X_VERSION_MIN_REQUIRED < 1070` - and is recorded, not
  transcribed); C 10934's `UNINIT` is `conf_post.h` 369's `= {0,}` on this configuration
  (`GCC_LINT` is 1 at `docs/baseline/config/darwin-arm64.macros.tsv` 1162), so the three locals
  start at zero as C's own arm does; C 10956's `%ld` prints `part`'s `NSInteger`; and
  `NSWidth`/`NSHeight`/`NSMinX`/`NSMinY` are the rect's own accessors.
* **Two nil readings kept**: C 11029's `[super mouseUp: fake_event]` is C's message, whose nil case
  does nothing - the port keeps that as the `if let` (rather than a force-unwrap); and the fake
  event itself is `NSEvent.mouseEvent (with:…)`, the `otherEvent (with:…)` spelling this unit
  already uses.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **21** unported,
  21 row(s); **480** C symbol import(s)` - unported unchanged (Objective-C methods are not C
  definitions), and the round's imports are 474 -> 480 (the six `scroll_bar_*` enumerators).
  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no mouse-down reached the scroller, no fake
  event sent, no timer armed); the compile gate reaches parsing and type checking only; the six
  enumerator entries are declarations the core still owes.

### Round 46: `EmacsScroller` closed (C 11050-11115)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-45 carry, so **no new warning**, and the round
  **compiled first try** (the three renamed spellings below were pre-applied from round 45's
  measurements).  Object `/tmp/nsterm46.o`, **843,232 bytes**, md5
  `d2d6317ef94c46554a3d7db61126e3f3`.  The unit is **9,434 lines**.
* **Three methods land, and the class closes**: `mouseDragged:` (C 11050-11090), `mouseUp:`
  (C 11094-11105) and `scrollWheel:` (C 11108-11115).  All three take the compiler's own rename
  readings (`mouseDragged`/`mouseUp`/`scrollWheel` -> `… (with:)`), the spellings round 45 measured.
* **What C's own text decides here**: `mouseDragged:`'s two locals are `double`, not `CGFloat` (so
  the port uses `Double` and converts at the `float` parameter of `sendScrollEventAtLoc:` exactly
  where C's call converts), its bound is `length + last_mouse_offset` with C's promotions, and its
  `pos * pixel_length / length` keeps C's per-operator promotion; `mouseUp:`'s `release` is the
  reference the method drops (the `repeatScroll:` reading); `scrollWheel:`'s
  `(EmacsView *) FRAME_NS_VIEW (frame)` is the unit's `as? EmacsView`, whose optional chain is C's
  nil-receiver no-op.
* **The class box is now ticked**: 14 of the 15 methods are landed, and the fifteenth
  (`+scrollerWidth`, C 10626) is the one Swift cannot declare at all - refused both as an override
  and with `@objc(scrollerWidth)`, measured in round 42, recorded there and in `## ABI` (3), and
  unreachable by any reference caller.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **21** unported,
  21 row(s); **480** C symbol import(s)` - unchanged from round 45 (Objective-C methods are not C
  definitions, and this round adds no import).  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no drag tracked, no button released, no
  wheel event forwarded); the compile gate reaches parsing and type checking only; the omitted
  `+scrollerWidth` remains a named gap.

### Round 47: `ns_set_vertical_scroll_bar` (C 5396-5486)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-46 carry, so **no new warning**.  Object
  `/tmp/nsterm47.o`, **849,096 bytes**, md5 `ea9a9a0d055d36c26b107cc24795f0b2`.  The unit is
  **9,561 lines**.
* **A structural slip this round found and corrected**: the four entries round 43 added
  (`wset_horizontal_scroll_bar`, `wset_vertical_scroll_bar`, `XSETWINDOW`, `mark_object`) had landed
  *inside* `class EmacsScroller` - C keeps those macros and INLINEs at file scope, and the first
  compile of this round is what showed it (three `error: cannot find … in scope` lines from
  `ns_set_vertical_scroll_bar`, which C writes outside the class).  They are file-scope now, with a
  note saying why; their call sites inside the class were unaffected.
* **The hook itself lands whole**: C 5412-5425's optimization (with the `EmacsView` counter and the
  `windows_or_buffers_changed` early return), C 5431's `window_box` call with its two NULL
  out-parameters, C 5437-5440's flipped-y rect arithmetic, C 5446-5458's "at least 5 lines" teardown
  (C 5453's `[bar release]` is the reference C drops, and `wset_vertical_scroll_bar (window, Qnil ())`
  is the round-43 entry), C 5460-5481's create-vs-update split (C 5466's `make_mint_ptr (bar)` is the
  object's own opaque pointer), and C 5483-5485's conditional `setPosition:` plus `unblock_input`.
* **The `EmacsScroller` it drives is fully landed now** (rounds 42-46), which is why this box could
  finally open: `initFrame:window:` (C 5465), `checkSamePosition:portion:whole:` (C 5415),
  `setPosition:portion:whole:` (C 5484), `frame`'s setter (C 5473/5479) and `removeFromSuperview`
  (C 5451) are all the port's own methods by now.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **20** unported,
  20 row(s); **486** C symbol import(s)` - one fewer unported than round 46's 21 (this definition),
  one stale row cleared by name, and the round's imports are 480 -> 486 (the six entries above).
  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no scroll bar created, resized or removed);
  the compile gate reaches parsing and type checking only; the six entries are declarations the core
  still owes.

### Round 48: `ns_set_horizontal_scroll_bar` (C 5490-5571)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-47 carry, so **no new warning**, and the round
  **compiled first try**.  Object `/tmp/nsterm48.o`, **854,424 bytes**, md5
  `844f126505fa3884d27ba6b84259b98a`.  The unit is **9,661 lines**.
* **The twin, and the three differences C's own text shows** (no two hooks are copy-paste here):
  ⑴ C 5525 takes the *x* pair out of `window_box` (`&window_x`, `0`, `&window_width`, `0`) where the
  vertical hook takes the *y* pair; ⑵ C 5528-5529 read the horizontal bar's own height and y
  (`WINDOW_SCROLL_BAR_AREA_HEIGHT`/`_Y`, this round's two new entries) instead of the width and x the
  vertical hook uses; ⑶ C 5548-5558's update branch has **no** `oldRect`-into-`r.size.width` copy
  (the vertical hook's C 5474) and sets `update_p = YES` after the frame write.  C 5561-5566's
  shared-corner clear is its own step, kept with C's `window->vertical_scroll_bar` test.
* **The two `EmacsScroller` calls are the landed methods**: `initFrame:window:` (C 5544, the round-42
  initializer) and `setPosition:portion:whole:` (C 5569, round 44), with `checkSamePosition:`
  (C 5509) and `frame`'s setter (C 5552/5556) from rounds 44 and 43.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **19** unported,
  19 row(s); **489** C symbol import(s)` - one fewer unported than round 47's 20 (this definition),
  one stale row cleared by name, and the round's imports are 486 -> 489 (the three entries above).
  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no horizontal bar created, resized or
  corner-cleared); the compile gate reaches parsing and type checking only; the three entries are
  declarations the core still owes.

### Round 49: the scroll-bar hook batch closes (C 5575-5640), and the batch's dead arm (C 2771-2786)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-48 carry, so **no new warning**, and the round
  **compiled first try**.  Object `/tmp/nsterm49.o`, **860,064 bytes**, md5
  `53c692b9cd3ec35e7ea610c27e833003`.  The unit is **9,746 lines**.
* **Three definitions land**: `ns_condemn_scroll_bars` (C 5575-5593), `ns_redeem_scroll_bar`
  (C 5597-5618) and `ns_judge_scroll_bars` (C 5622-5640).  Each is short, and each is a *consumer*
  of things earlier rounds built: `condemn`/`reprieve`/`judge` are round 43's `EmacsScroller`
  methods, `XNS_SCROLL_BAR` is round 47's entry, and the two member readers are rounds 47-48's.
  The new names are just the two predicate macros.
* **The dead arm, recorded**: `ns_redraw_scroll_bars` (C 2771-2786) is wholly inside
  `#ifdef NS_IMPL_GNUSTEP`, so it is not transcribed - the round's block describes what its body
  would do (walk the superview's subviews backwards, skip everything that is not an
  `EmacsScroller`, send each the rest `display`) and says why it sits after round 4's block, not at
  its C position.
* **Two shapes C's text forces, both written out**: `ns_judge_scroll_bars`'s `continue` (in a
  `while` it needs the decrement on both sides, which C's `for` does itself) and `ns_redeem_scroll_bar`'s
  `id bar` (the port's `AnyObject?`, with the `XNS_SCROLL_BAR` cast at the use site).
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **16** unported,
  16 row(s); **491** C symbol import(s)` - three fewer unported than round 48's 19 (the three
  definitions), three stale rows cleared by name, and the round's imports are 489 -> 491 (the two
  macros).  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **The batch is closed**: all five scroll-bar hooks are landed (rounds 47-49) and the one dead
  member of the family is recorded - which is what `ns_create_terminal` (C 5908) was waiting for.
* **What this round does not claim**: no run evidence (nothing condemned, redeemed, judged or
  redrawn); the compile gate reaches parsing and type checking only; the two macro entries are
  declarations the core still owes.

### Round 50: the `ns_create_terminal` entry layer (C 5906-5961)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-49 carry, so **no new warning**.  Object
  `/tmp/nsterm50.o`, **860,304 bytes**, md5 `ce0bc8cd21cc37fa3f4bbf490c6c81c1` (**identical before and after the citation fix below** - a
  comment-only edit does not change the object).  The unit is
  **9,919 lines**.
* **Why this round is the entry layer rather than the body**: C 5922-5957 assigns 36 distinct
  hook functions, and every assignment needs a `struct terminal` member write whose parameter type
  is that hook's own C prototype.  The prototypes are 36 different function-pointer types, so the
  entries come first; the body (the create call, the two display-info writes and the 36
  assignments with their non-capturing `@convention(c)` closures, the round-39 shape) is the
  remainder, and the box's text says so.
* **What landed**: `create_terminal` (the core's own entry, `femacs/src/terminal.rs` 366),
  `output_ns` (src/termhooks.h 64's enumerator - a *type tag*, not a record - mirror-valued 5 at
  `femacs/src/include/termhooks.rs` 159), `ns_display_info_set_terminal` (the writer C 5920 needs,
  of the family round 37 established and round 40 read), and the **36 `terminal_set_<member>`
  setters**, each declared with C's prototype transcribed from src/termhooks.h and the C line
  `ns_create_terminal` writes it at.
* **One compile failure and its fix**: `terminal_set_read_socket_hook`'s `(…, *mut input_event) ->
  int` is refused as a `@convention(c)` type - `'(UnsafeMutableRawPointer?, UnsafeMutablePointer<input_event>?) -> Int32'
  is not representable in Objective-C` - so C 5926's record pointer is **untyped** in that entry, the
  same limitation round 2 recorded for `@_cdecl` (a Swift record cannot cross that boundary).
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **16** unported,
  16 row(s); **529** C symbol import(s)` - unported unchanged, which is the expected reading for an
  entry-layer round (no C definition lands yet); the imports are 491 -> 529 (the 38 new entries).
  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **Two citation drifts, both fixed** (the round 24 lesson, and the second time this lane meets it):
  the `output_ns` note first named the enumeration on the line citing src/termhooks.h 64, then named
  `create_terminal` there (the header declares that function at 964); the note now names on each
  citing line only what lives at the cited line, and `tools/check-citations.py` reads **0 drift** -
  the digest above is the same one before and after that fix (comments do not reach the object).
* **What this round does not claim**: no run evidence (no terminal created, no hook installed); the
  compile gate reaches parsing and type checking only; all 38 entries are declarations the core
  still owes, and `ns_create_terminal`'s body is not written yet.

### Round 51: `ns_new_font` (C 11410-11476)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-50 carry, so **no new warning**, and the round
  **compiled first try**.  Object `/tmp/nsterm51.o`, **864,336 bytes**, md5
  `a95602d825bbf424d56c6ec1661a26d7`.  The unit is **10,037 lines**.
* **Why here, why now**: `ns_create_terminal`'s body still needed one hook function - C 5942's
  `ns_new_font` - and it was the only one of the 36 that no lane had landed.  Landing it first is
  the dependency order P-91 ① allows (the class rounds already land out of C order for that reason).
* **The entry rule this round settles** (recorded in the source and in `## ABI` (3)): **reads keep
  the accessor macro's own name** (`FRAME_LINE_HEIGHT`, `FRAME_CONFIG_SCROLL_BAR_WIDTH`/`_HEIGHT`,
  `FRAME_COLS`, `FRAME_LINES` - the practice `FRAME_FONT`/`FRAME_COLUMN_WIDTH` already follow),
  while **writes go through the setter of the family the record belongs to**: `ns_output_set_fontset`
  / `_set_font` / `_set_baseline_offset` for the `struct ns_output` members (src/nsterm.h 1049-1051,
  the family `ns_output_set_current_pointer` belongs to) and `frame_set_column_width` /
  `_set_line_height` / `_set_config_scroll_bar_cols` / `_set_config_scroll_bar_lines` for the
  `struct frame` members (src/frame.h, the family `frame_set_terminal` belongs to).
* **Two shapes C's text forces, both written out**: C 11427-11429 reassigns the `fontset` parameter
  (the shadow spelling this lane uses for that) and C 11470's `! [view isFullscreen]` reads a
  possibly-nil view (C's nil-receiver answer, `?? false` inside the negation).
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **15** unported,
  15 row(s); **549** C symbol import(s)` - one fewer unported than round 50's 16 (this definition),
  one stale row cleared by name, and the round's imports are 529 -> 549 (the 20 entries above).
  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no font set, no frame resized); the compile
  gate reaches parsing and type checking only; the entries are declarations (several of the core
  items are mirrored Rust functions rather than C-ABI exports, so the export side is owed).

### Round 52: `ns_create_terminal` (C 5906-5961)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-51 carry, so **no new warning**, and the round
  **compiled first try** (the 36-entry table included).  Object `/tmp/nsterm52.o`, **883,472 bytes**,
  md5 `2cbf5e8968af8fc18aabcd1e542ba8eb`.  The unit is **10,151 lines**.
* **The body, and what each piece is**: C 5917's `create_terminal (output_ns,
  &ns_redisplay_interface)` is round 50's entry plus the round-39 table's address (taken the way
  that round takes it); C 5919's `terminal->display_info.ns = dpyinfo` is this round's
  `terminal_set_display_info_ns` (the writer of round 40's reader); C 5920 is round 50's
  `ns_display_info_set_terminal`; C 5922-5957's 36 writes are non-capturing `@convention(c)`
  closures over the ported hooks, 7 of them reached through the nsfns/nsmenu lanes' `@_cdecl`
  entries (this round declares those C-ABI signatures); and C 5960 returns the terminal.
* **Three crossings the closures spell out** (each because a `@convention(c)` type cannot name this
  unit's Swift records): C 5926's `input_event *`
  (`hold_quit?.assumingMemoryBound (to: input_event.self)`), C 5929's `Emacs_Color *`
  (`bgcolor!.assumingMemoryBound (to: Emacs_Color.self)`) and C 5953's `char *` return
  (`UnsafePointer (ns_get_string_resource (…))`).
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **14** unported,
  14 row(s); **557** C symbol import(s)` - one fewer unported than round 51's 15 (this definition),
  one stale row cleared by name, and the round's imports are 549 -> 557 (`terminal_set_display_info_ns`
  and the 7 cross-lane entries).  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (no terminal created, no hook ever called
  through the table); the compile gate reaches parsing and type checking only; the entries remain
  declarations the core owes, and the seven cross-lane hooks are reached through symbols those lanes
  export.

### Round 53: `mark_nsterm` (C 11528-11547)

* **`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift`**: **exit 0**, **0 errors**,
  **eight** warnings - the same eight rounds 22-52 carry, so **no new warning**, and the round
  **compiled first try**.  Object `/tmp/nsterm53.o`, **885,736 bytes**, md5
  `90e4176a5263d8fd0dd94d706306adf4`.  The unit is **10,189 lines**.
* **`mark_nsterm` needed no new entry at all**: its whole body is `FOR_EACH_FRAME` over
  `Vframe_list`, `FRAME_NS_P`, `FRAME_NS_VIEW (f)?.superview?.subviews`, the `isKindOfClass:` test
  and round 43's `mark` method - every one of those landed in earlier rounds.  It is `@_cdecl` because
  C's own definition is non-static (declared src/nsterm.h 1276).
* **A re-check rather than a discovery**: this round also looked at `hide_bell` (C 1469-1483), which
  is the file's *third* dead arm - and found it **already recorded** as such (this ledger's line 62,
  an earlier round's tick, with its one call site C 9388 named).  The round adds the same reading to
  the source at its C position; the ledger's existing row stands.
* **`python3 tools/check-swift-units.py`**: `nsterm.swift: 150 C definition(s), **13** unported,
  13 row(s); **557** C symbol import(s)` - one fewer unported than round 52's 14 (this definition),
  one stale row cleared by name, and the imports are unchanged, the expected reading for a round that
  adds no entry.  Final: `0 new, 0 stale, 7 Swift unit(s) landed`.
* **What this round does not claim**: no run evidence (nothing marked, no bell hidden); the compile
  gate reaches parsing and type checking only.
