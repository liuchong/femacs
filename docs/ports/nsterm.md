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
- [ ] ns_redraw_scroll_bars  (line 2773, static)
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
- [x] DOCK_IGNORE_LIMIT  (line 708, body c1dfd96eea8c) — landed as a file-scope `let` (CGFloat, C's `int` promoted at the comparisons)
- [x] EV_BUTTON  (line 438, body 02ec0a9e0dff) — landed as `func EV_BUTTON (_ e: NSEvent) -> Int`
- [x] EV_MODIFIERS  (line 425, body 0a951d59f9e1) — landed as `func EV_MODIFIERS (_ e: NSEvent) -> UInt32`
- [x] EV_MODIFIERS2  (line 397, body 8155b8d11eb3) — landed as `func EV_MODIFIERS2 (_ flags: UInt, _ kind: Lisp_Object) -> UInt32`; C's `NSUInteger`→`unsigned int` conversions are `truncatingIfNeeded:`
- [x] EV_TIMESTAMP  (line 444, body 8e139f589481) — landed as `func EV_TIMESTAMP (_ e: NSEvent) -> Double`
- [ ] EV_TRAILER  (line 448, body 0dd430c310a9) — **held** to C 5013: its body calls the hold queue and `EV_TRAILER2`, and `ns_send_appdefined` (C 5013) is this file's own function, so no Swift text can exist before it does (see Decisions)
- [ ] EV_TRAILER2  (line 455, body bf5ab84e8621) — **held** to C 5013, like `EV_TRAILER` (its callees are C 498's `hold_event` and C 5013's `ns_send_appdefined`)
- [x] EV_UDMODIFIERS  (line 427, body 89301cbc268c) — landed as `func EV_UDMODIFIERS (_ e: NSEvent) -> Int32`, C's ten comparisons transcribed one for one
- [x] NO_APPDEFINED_DATA  (line 318, body 9d95217d51d9)  — landed as a file-scope `let` of the same name
- [x] NSAppKitVersionNumber10_10  (line 10407, body ed9f6e93628b) — the *name* is used at
      C 2139 (`ns_set_appearance`) and spelled there as Swift's `NSAppKitVersion.macOS10_10`
      (class B of the 3.255 ruling: the AppKit apinotes rename the vintage constants, and no
      symbol is invented); C's own `#define … 1343` fallback at C 10406-10408 is inside the later
      `-[EmacsWindow setAppearance]` slice, whose box stays where it is
- [ ] NSAppKitVersionNumber10_9  (line 6262, body 74b10c057ded)
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
**C 60-3508**.  The next live definition is C 3512's `ns_show_hourglass` (three lines; the
`ns_hide_hourglass` and `ns_get_glyph_string_clip_rect` that follow are as short).

Ticked above: **89 of the 150 function boxes**.  The count is the generator's own again: the
pre-3.260 `tools/cinventory.py` could not see **eight** of this file's definitions (the walk from a
declarator to its `{` stopped at the documentation comment in between - status 3.260), the port had
already written seven of them (round 5's two, round 6's `ns_focus`, round 10's four), and the
eighth - `ns_get_shifted_character`, C 2693 - is an open box here until its slice lands.  Also
**17 of the 23 macro boxes** (the eight masks,
`NO_APPDEFINED_DATA`, the five `EV_*` accessors, `DOCK_IGNORE_LIMIT`, the two
`NS_PARENT_WINDOW_*` and round 10's `NSAppKitVersionNumber10_10`; the two held `EV_TRAILER*` and
the four later ones stay open), **49 of the 49 file-scope-object boxes**
(the generator's 39 minus its gaps, the 8 hand-added round 1 rows, round 5's two interval
constants, round 6's `bell_view` and round 15's `fringe_bmp` - **the file's last open object
box**), **1 of the 1 type box** (`EmacsMargins`,
C 650) and **2 of the 8 Objective-C classes** (`NSColor (EmacsColor)` with its four methods, and
**`EmacsBell`** - the file's first *class*, with its `@interface` declaration and all four
methods).  The other 83 function boxes, 6 macro boxes, 6 class boxes and 185 method boxes are
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
(opened at src/xdisp.c 2490), which the frozen configuration leaves undefined: the objects have no
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
src/xdisp.c 35466 **inside `#ifdef HAVE_WINDOW_SYSTEM`** (opened at xdisp.c 35086): the frozen
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
both inside `HAVE_WINDOW_SYSTEM` (the `#ifdef`s at 2490 and 35086), measured absent from
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
| `remember_mouse_glyph` | `void remember_mouse_glyph (struct frame *, int, int, NativeRectangle *)` (src/dispextern.h 3586; `NativeRectangle` is `NSRect`, src/nsgui.h 80) | defined xdisp.c 2779, **inside `HAVE_WINDOW_SYSTEM`** (the `#ifdef` at xdisp.c 2490) | **open, gated**: the frozen configuration leaves it undefined, so the object has no symbol and the shim exports it under the same gate - the status 3.255 ③ `frame_set_double_buffered` reading (only a window-system build can satisfy the name) |
| `emacs_view_frame`, `is_emacs_view` | the `EmacsView` messages C 2537-2538 sends | this unit's own class, C 7039 | **open**: receiver-first entries, defined when the class lands; `nsfns.swift` 689-691 imports the same two names |
| `is_emacs_tooltip` | `[[w delegate] isKindOfClass: [EmacsTooltip class]]` (C 2534) | `nsmenu.swift`'s `EmacsTooltip` (`src/nsmenu.m` 1421) | **open**: that class's own unit defines it here |
| `ns_output_current_pointer`, `ns_output_set_current_pointer` | the `struct ns_output` member `f->output_data.ns->current_pointer` - `#define FRAME_POINTER_TYPE(f)` (src/nsterm.h 977/1047), read at C 2631 and written at C 2634 | src/nsterm.h | **open**: the family's member names (round 12 takes them rather than a macro-name entry - see Decisions); both sit behind the shim's `have_ns` gate, the **write** half already exists for this very C line (`docs/ports/abi_shim.md` 96), and the read is the one name this slice adds to the ratchet |
| `QCordinary` | src/nsterm.m 11793's own `DEFSYM (QCordinary, ":ordinary")` | **this file** | **open**: class A - a symbol of this file, absent from `docs/baseline/globals/symbols.tsv` (the `QCmouse` reading of round 2) |
| `Qnone` | `DEFSYM (Qnone, "none")` (src/frame.c 7241) | src/frame.c | **open**: the core's to export; `_Qnone` is `S` in `build/emacs-ref/src/emacs.o` |
| the face/window/run records (round 13) | `NS_FACE_BACKGROUND` (src/nsterm.h 1041), `FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL`, `DEFAULT_FACE_ID`/`CHILD_FRAME_BORDER_FACE_ID`/`INTERNAL_BORDER_FACE_ID` (src/dispextern.h), `ANY_AREA` (src/dispextern.h 883), `window_frame` (the `window_<member>` family), `run_current_y`/`run_desired_y`/`run_height` (`struct run`, src/dispextern.h), `FRAME_TOP_MARGIN_HEIGHT`/`FRAME_BOTTOM_MARGIN_HEIGHT` (src/frame.h 1139/1151) | src/nsterm.h / src/dispextern.h / src/frame.h | **open**: the member/enumerator/macro names of a record the unit reaches through entries; `FRAME_TOP_MARGIN_HEIGHT`'s body sums three other `FRAME_*` macros over `struct frame` members, so the expansion would owe four member entries where one name is owed (Decisions) |
| `gui_clear_cursor` | `void gui_clear_cursor (struct window *)` (src/dispextern.h; defined src/xdisp.c 35466 **inside `HAVE_WINDOW_SYSTEM`**, the `#ifdef` at xdisp.c 35086) | src/xdisp.c | **open, gated**: the frozen configuration leaves it undefined, so the object has no symbol (measured) - the status 3.255 ③ shape again |
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
| `get_phys_cursor_geometry`, `draw_phys_cursor_glyph`, `draw_fringe_bitmap` | `void get_phys_cursor_geometry (struct window *, struct glyph_row *, struct glyph *, int *, int *, int *)` (src/xdisp.c 2707), `void draw_phys_cursor_glyph (struct window *, struct glyph_row *, enum draw_glyphs_face)` (src/xdisp.c 35099), `void draw_fringe_bitmap (struct window *, struct glyph_row *, int)` (src/fringe.c 838) | src/xdisp.c (both inside `HAVE_WINDOW_SYSTEM`, opened at 2490 and 35086) / src/fringe.c (a file no built configuration compiles - `femacs/src/fringe.rs`) | **open, gated**: none has a reference-side symbol (measured), so each row carries its gate - the 3.255 ③ / 3.262 shape |
| `get_phys_cursor_glyph` | `struct glyph *get_phys_cursor_glyph (struct window *)` (src/window.h 1186; defined src/window.c 8228, no gate) | src/window.c | **provided**: `femacs/src/window.rs` 8078, and `_get_phys_cursor_glyph` is `T` in `build/emacs-ref/src/window.o` |
| Carbon and CoreFoundation (round 12) | `TISCopyCurrentKeyboardLayoutInputSource`, `TISGetInputSourceProperty`, `UCKeyTranslate`, `LMGetKbdType`, `kUCKeyActionDown`, `kUCKeyTranslateNoDeadKeysBit` and the four key masks (`shiftKey`/`optionKey`/`rightOptionKey`/`cmdKey`), plus `CFDataGetBytePtr` | Carbon.HIToolbox / CoreFoundation (C 72-73's `#include <Carbon/Carbon.h>`, C 70-80's arm) | **system**: these are *module* calls, not `@_silgen_name` entries - `import Carbon` at the top of the unit - which is a new kind of dependency for this file: the eventual Cocoa link line needs `-framework Carbon` |
| `FRAME_BACKGROUND_COLOR` | `#define FRAME_BACKGROUND_COLOR(f) ((f)->output_data.ns->background_color)` (src/nsterm.h 1038) | header-only | **open**: `macfont.swift` 756 imports the same name and type |

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
  line 2779 gives `#ifdef HAVE_WINDOW_SYSTEM` at **2490**, and `nm
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
  **35086**, and `nm build/emacs-ref/src/xdisp.o | grep -c gui_clear_cursor` is **0**.
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
  provided one).  `get_phys_cursor_geometry` (xdisp.c 2707, inside the `#ifdef` at 2490),
  `draw_phys_cursor_glyph` (xdisp.c 35099, inside the one at 35086) and `draw_fringe_bitmap`
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
