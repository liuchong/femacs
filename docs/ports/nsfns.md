# emacs/src/nsfns.m (4348 lines)

> **This file is not compiled by the frozen configuration** (`docs/baseline/frozen-build-sources.txt`,
> `docs/status.md` 3.31): the frozen target is darwin-arm64 with no window system, so `HAVE_NS`
> and `NS_IMPL_COCOA` are undefined and src/nsfns.m belongs to the 77 src files `make -n` never
> reaches.  Batch F ports the Objective-C front end to Swift anyway (`docs/port-map.md`'s Swift
> table, `src/*.m` -> `femacs/platform/macos/*.swift`), so this ledger is the checklist of the
> **Cocoa/AppKit build** the file's arms describe, and the unit is
> `femacs/platform/macos/nsfns.swift`.  Line numbers are the raw reference lines.
>
> **State: second batch (C 40-1884).** The file is 4348 lines and the largest remaining Swift
> unit after src/nsterm.m, so it is landing in batches the way `macfont.swift` did.  Batch 1
> carried C 40-1190 (the file-scope objects, the internal utilities, every frame-parameter setter
> the file defines, the AppKit version/icon helpers, the `ns_frame_parm_handlers` table and the
> unwind/geometry support).  Batch 2 carries C 1188-1884: **`x-create-frame` (C 1192-1610, the
> file's biggest function)**, the four frame-operation `DEFUN`s that follow it
> (`ns-frame-list-z-order`, `ns-frame-restack`, `x-select-font`, `ns-popup-color-panel`),
> `ns_window_is_ancestor`, the file-dialog record and `ns_run_file_dialog`, and
> `ns-read-file-name` (C 1762-1884).  The remaining 19 C functions and 46 `DEFUN`s are the next
> batch and are named in `docs/baseline/swift-units.tsv` (19 rows; the `DEFUN`s are not tracked
> by that gate).

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 52
- [x] x-create-frame  (Fx_create_frame, 1/1, doc f3e009aa43c7, line 1192) - `@_cdecl` + `Sx_create_frame` - next batch
- [x] ns-frame-list-z-order  (Fns_frame_list_z_order, 0/1, doc 9f50e5d349c0, line 1626) - `@_cdecl` + `Sns_frame_list_z_order`
- [x] ns-frame-restack  (Fns_frame_restack, 2/3, doc eac4febee7e9, line 1657) - `@_cdecl` + `Sns_frame_restack`
- [x] x-select-font  (Fx_select_font, 0/2, doc ad00028ff8bf, line 1688) - `@_cdecl` + `Sx_select_font`
- [x] ns-popup-color-panel  (Fns_popup_color_panel, 0/1, doc 0c21686be8bc, line 1705) - `@_cdecl` + `Sns_popup_color_panel`
- [x] ns-read-file-name  (Fns_read_file_name, 1/5, doc e98e737bd5ec, line 1758) - `@_cdecl` + `Sns_read_file_name`
- [ ] ns-get-resource  (Fns_get_resource, 2/2, doc 9304e7bd5dcc, line 1897)
- [ ] ns-set-resource  (Fns_set_resource, 3/3, doc 12f0cd058463, line 1917)
- [ ] x-server-max-request-size  (Fx_server_max_request_size, 0/1, doc f3e009aa43c7, line 1944)
- [ ] x-server-vendor  (Fx_server_vendor, 0/1, doc f3e009aa43c7, line 1957)
- [ ] x-server-version  (Fx_server_version, 0/1, doc f3e009aa43c7, line 1970)
- [ ] x-display-screens  (Fx_display_screens, 0/1, doc f3e009aa43c7, line 1985)
- [ ] x-display-mm-height  (Fx_display_mm_height, 0/1, doc f3e009aa43c7, line 1994)
- [ ] x-display-mm-width  (Fx_display_mm_width, 0/1, doc f3e009aa43c7, line 2015)
- [ ] x-display-backing-store  (Fx_display_backing_store, 0/1, doc f3e009aa43c7, line 2036)
- [ ] x-display-visual-class  (Fx_display_visual_class, 0/1, doc f3e009aa43c7, line 2060)
- [ ] x-display-save-under  (Fx_display_save_under, 0/1, doc f3e009aa43c7, line 2087)
- [ ] x-open-connection  (Fx_open_connection, 1/3, doc f3e009aa43c7, line 2111)
- [ ] x-close-connection  (Fx_close_connection, 1/1, doc f3e009aa43c7, line 2136)
- [ ] x-display-list  (Fx_display_list, 0/0, doc f3e009aa43c7, line 2147)
- [ ] ns-hide-others  (Fns_hide_others, 0/0, doc e0c4b6a8dba0, line 2161)
- [ ] ns-hide-emacs  (Fns_hide_emacs, 1/1, doc 7a15fe88b5a3, line 2171)
- [ ] ns-emacs-info-panel  (Fns_emacs_info_panel, 0/0, doc 9037ec6f5a75, line 2204)
- [ ] ns-font-name  (Fns_font_name, 1/1, doc cf7fb802cc5d, line 2215)
- [ ] ns-list-colors  (Fns_list_colors, 0/1, doc 04694a2cc15f, line 2291)
- [ ] ns-list-services  (Fns_list_services, 0/0, doc 50081da2e47c, line 2331)
- [ ] ns-perform-service  (Fns_perform_service, 2/2, doc 1c52a8aeaf8f, line 2356)
- [ ] ns-do-applescript  (Fns_do_applescript, 1/1, doc 20491d02fe35, line 2454)
- [ ] system-move-file-to-trash  (Fsystem_move_file_to_trash, 1/1, doc 4e8f353b9231, line 2574)
- [ ] xw-color-defined-p  (Fxw_color_defined_p, 1/2, doc f3e009aa43c7, line 2612)
- [ ] xw-color-values  (Fxw_color_values, 1/2, doc f3e009aa43c7, line 2622)
- [ ] xw-display-color-p  (Fxw_display_color_p, 0/1, doc f3e009aa43c7, line 2647)
- [ ] x-display-grayscale-p  (Fx_display_grayscale_p, 0/1, doc f3e009aa43c7, line 2664)
- [ ] x-display-pixel-width  (Fx_display_pixel_width, 0/1, doc f3e009aa43c7, line 2678)
- [ ] x-display-pixel-height  (Fx_display_pixel_height, 0/1, doc f3e009aa43c7, line 2689)
- [ ] ns-display-monitor-attributes-list  (Fns_display_monitor_attributes_list, 0/1, doc fe0f18f2b85b, line 2736)
- [ ] x-display-planes  (Fx_display_planes, 0/1, doc f3e009aa43c7, line 2848)
- [ ] x-display-color-cells  (Fx_display_color_cells, 0/1, doc f3e009aa43c7, line 2859)
- [ ] x-show-tip  (Fx_show_tip, 1/6, doc f3e009aa43c7, line 3229)
- [ ] x-hide-tip  (Fx_hide_tip, 0/0, doc f3e009aa43c7, line 3541)
- [ ] ns-frame-geometry  (Fns_frame_geometry, 0/1, doc ed7876cab3f5, line 3616)
- [ ] ns-frame-edges  (Fns_frame_edges, 0/2, doc 64d4478f2b7a, line 3660)
- [ ] ns-set-mouse-absolute-pixel-position  (Fns_set_mouse_absolute_pixel_position, 2/2, doc 084be46aa072, line 3684)
- [ ] ns-mouse-absolute-pixel-position  (Fns_mouse_absolute_pixel_position, 0/0, doc cdeb37cbd73d, line 3726)
- [ ] ns-show-character-palette  (Fns_show_character_palette, 0/0, doc a6af2b7c9688, line 3745)
- [ ] ns-process-is-accessibility-trusted  (Fns_process_is_accessibility_trusted, 0/0, doc 33940a79fea0, line 3759)
- [ ] ns-badge  (Fns_badge, 1/1, doc 43c87b05e806, line 3779)
- [ ] ns-request-user-attention  (Fns_request_user_attention, 1/1, doc b7dd254b2e0a, line 3802)
- [ ] ns-progress-indicator  (Fns_progress_indicator, 1/1, doc 8aad684a609a, line 3832)
- [ ] ns-block-system-sleep  (Fns_block_system_sleep, 2/2, doc a61a3188a360, line 3912)
- [ ] ns-unblock-system-sleep  (Fns_unblock_system_sleep, 1/1, doc dd3e82a40673, line 3960)
- [ ] ns-send-items  (Fns_send_items, 1/1, doc 1d7aaf406677, line 3993)

## Functions defined — 56
- [x] check_ns_display_info  (line 100, static)
- [x] ns_get_window  (line 137, static)
- [x] ns_display_info_for_name  (line 155, static)
- [x] ns_filename_from_panel  (line 177, static, the `NS_IMPL_COCOA` arm)
- [x] ns_directory_from_panel  (line 189, static, the `NS_IMPL_COCOA` arm)
- [ ] interpret_services_menu  (line 202, static; `#ifndef NS_IMPL_COCOA` - **manifest row**, dead arm)
- [x] ns_set_foreground_color  (line 260, static)
- [x] ns_set_background_color  (line 293, static)
- [x] ns_set_cursor_color  (line 345, static)
- [x] ns_set_icon_name  (line 371, static)
- [x] ns_set_name_internal  (line 415, static)
- [x] ns_set_name  (line 434, static)
- [x] ns_set_represented_filename  (line 471, static)
- [x] ns_explicitly_set_name  (line 525, static)
- [x] ns_implicitly_set_name  (line 536, non-static - exported with `@_cdecl`)
- [x] ns_set_title  (line 551, static)
- [x] ns_set_doc_edited  (line 571, non-static - `@_cdecl`)
- [x] ns_set_menu_bar_lines  (line 599, static)
- [x] ns_change_tab_bar_height  (line 627, non-static - `@_cdecl`)
- [x] ns_make_frame_key_window  (line 690, non-static, the `NS_IMPL_COCOA` arm - `@_cdecl`)
- [x] ns_set_tab_bar_lines  (line 699, static)
- [x] ns_set_tool_bar_lines  (line 723, static)
- [x] ns_set_tool_bar_position  (line 774, static)
- [x] ns_set_child_frame_border_width  (line 782, static)
- [x] ns_set_inhibit_double_buffering  (line 806, static, the `NS_IMPL_COCOA` + 10.14 arm)
- [x] ns_set_internal_border_width  (line 826, static)
- [x] ns_implicitly_set_icon_type  (line 843, static)
- [x] ns_set_icon_type  (line 929, static)
- [x] ns_set_cursor_type  (line 968, static)
- [x] ns_set_mouse_color  (line 976, static)
- [x] ns_appkit_version_str  (line 986, static)
- [x] ns_appkit_version_int  (line 1013, static)
- [x] ns_icon  (line 1025, static)
- [x] unwind_create_frame  (line 1122, static)
- [x] do_unwind_create_frame  (line 1145, static)
- [x] get_geometry_from_preferences  (line 1156, static)
- [x] ns_window_is_ancestor  (line 1615, static)
- [x] ns_run_file_dialog  (line 1726, non-static - `@_cdecl`)
- [ ] ns_get_defaults_value  (line 1886) - next batch (manifest row)
- [ ] ns_init_colors  (line 2237) - next batch (manifest row)
- [ ] ns_do_applescript  (line 2392, static) - next batch (manifest row)
- [ ] ns_run_ascript  (line 2447) - next batch (manifest row)
- [ ] check_x_display_info  (line 2520) - next batch (manifest row)
- [ ] ns_set_scroll_bar_default_width  (line 2527) - next batch (manifest row)
- [ ] ns_set_scroll_bar_default_height  (line 2536) - next batch (manifest row)
- [ ] ns_get_string_resource  (line 2546) - next batch (manifest row)
- [ ] ns_make_monitor_attribute_list  (line 2700, static) - next batch (manifest row)
- [ ] compute_tip_xy  (line 2870, static) - next batch (manifest row)
- [ ] unwind_create_tip_frame  (line 2942, static) - next batch (manifest row)
- [ ] ns_create_tip_frame  (line 2961, static) - next batch (manifest row)
- [ ] x_hide_tip  (line 3165, static) - next batch (manifest row)
- [ ] frame_geometry  (line 3554, static) - next batch (manifest row)
- [ ] all_nonzero_ascii  (line 4072, static) - next batch (manifest row)
- [ ] count_utf8_chars  (line 4083, static) - next batch (manifest row)
- [ ] ns_move_tooltip_to_mouse_location  (line 4154) - next batch (manifest row)
- [ ] syms_of_nsfns  (line 4202) - next batch (manifest row)

## Objective-C classes — 2
- [ ] EmacsFileDelegate  (`@implementation` 4048-4065, declared src/nsterm.h 654-661) - next batch
- [x] NSString (EmacsString)  (`@implementation` 4101-4151, declared src/nsterm.h 366-369) ->
      the two `@_cdecl` definitions `stringWithLispString` (4104) and `lispString` (4143).
      These are the two names `docs/ports/nsmenu.swift`, `nsfont.swift`, `nsimage.swift` and
      `nsselect.swift` import, so this file is where their definitions live.

## Macros defined — 4
- [ ] IOMasterPort  (line 54, body 08ecf8189f84) - `#define IOMasterPort IOMainPort` under
      `MAC_OS_X_VERSION_MAX_ALLOWED >= 120000` + `MIN_REQUIRED >= 120000`; on this SDK the
      `IOGraphicsLib.h` port call is `IOMainPort`, so the alias is dead text (recorded)
- [x] MODAL_OK_RESPONSE  (line 1755, body a4b3cc6adce4) - the `MAC_OS_X_VERSION_MAX_ALLOWED
      > 1090` arm is `NSModalResponseOK`, which the port spells
      `NSApplication.ModalResponse.OK` at its one use site
- [ ] Str  (line 982, body f1d202006571) and
- [ ] Xstr  (line 983, body 36e288827bc0) - the token-pasting pair the GNUstep arm of
      `ns_appkit_version_str` uses; dead text on this target (recorded)

## Types and their fields — 1
- [x] `<anon@1159>` (`const char *val, const char *cls, Lisp_Object tem`) -> the Swift tuple
      `(val: String, cls: String, tem: Lisp_Object)` inside `get_geometry_from_preferences`
      (C's `const char *` fields are string literals; the port passes their bytes per call)

## File-scope objects — 14
- [x] `static EmacsTooltip *ns_tooltip`  (line 61) -> `fileprivate var ns_tooltip: AnyObject?`
- [x] `static Lisp_Object tip_frame`  (line 64) ; `tip_dx, tip_dy` (67) ; `tip_timer` (71) ;
      `tip_last_string` (74) ; `tip_last_frame` (77) ; `tip_last_parms` (80)
- [x] `static Lisp_Object as_script, *as_result`  (line 83) ; `static int as_status` (84)
- [ ] `static NSInteger ns_request_user_attention_id`  (line 3800) - next batch
- [ ] `static unsigned int sleep_block_id`  (line 3909) ; `static NSMutableDictionary
      sleep_block_map` (3910) - next batch (they are `ns_request_user_attention_id`'s section)

## Decisions

**Where this batch stops, and why.**  The file is 4348 lines; `x_create_frame` (C 1192-1610,
418 lines) and its 51 sibling `DEFUN`s pull in the whole frame-construction surface
(`make_frame*`, `gui_default_parameter`, `Vx_resource_name`, the `struct kboard` accessors), so
the batch boundary is drawn at the last function *before* the Lisp definitions: C 40-1190.
Everything above that line is landed; the 21 remaining C functions and the 52 `DEFUN`s are
named in `docs/baseline/swift-units.tsv` so the gap is visible to the gate, not hidden in prose
(3.158's "WIP is not FAIL" reading).

**Which arms are live.**  The lane's target is the Cocoa/AppKit build (`NS_IMPL_COCOA`):
- the `NS_IMPL_COCOA` arms are carried: `ns_filename_from_panel`/`ns_directory_from_panel`
  (176-198 loop over `[panel URL]`), the Mach-port-leak workaround (494-508), the
  `TYPE_RANGED_FIXNUMP` variants, `ns_make_frame_key_window` (690-697), the font-name default
  (`userFixedPitchFontOfSize:`) at 1319-1331, and `setActivationPolicy:` at 1503;
- the `NS_IMPL_GNUSTEP` arms are recorded, not carried: `interpret_services_menu` (200-258, a
  whole function - a manifest row), `[panel filename]`/`[panel directory]` (185, 196),
  `[font screenFont]`, `GNUSTEP_GUI_*` in `ns_appkit_version_*` (1006, 1018), and the
  `Str`/`Xstr` macro pair (982-983) that only the GNUstep arm of `ns_appkit_version_str` uses;
- `check-cond-eval.py src/nsfns.m` -> **`64 conditions, 0 unknown, 0 disagreements [VERIFIED]`**,
  so every arm above was decided by measurement rather than by reading (Evidence).

**The parameter setters are the file's spine.**  Every one of them keeps C's shape: the
`block_input ()`/`unblock_input ()` bracket, the "Unknown color" `error` after
`store_frame_param (f, …, oldval)`, the `update_face_from_frame_parameter` call, ARC in place of
the `release`/`retain` pairs (`[col retain]; [f->output_data.ns->foreground_color release];
… = col;` becomes one assignment), and `NSTRACE` call sites kept as comments because the
reference's own `NSTRACE_ENABLED` is 0 (src/nsterm.h 128, 143).

**Objective-C to Swift.**  `[obj method: x]` -> `obj?.method(x)`; `id` -> `AnyObject?`;
`[f->output_data.ns->X release]; …->X = v;` -> the accessor pair `ns_output_X(f)` /
`ns_output_set_X(f, v)`; `f->name`/`f->title`/`f->icon_name`/`f->explicit_name`/`f->tab_bar_*`
-> `frame_*` accessors (the `struct frame` record belongs to the core).  `NSAppKitVersionNumber`
is `NSAppKitVersion.current.rawValue` in Swift 6.4 and `NSImage.name` is a *method*
(`name()`), both measured with a scratch compile.  `FOR_EACH_FRAME` (src/frame.h 1404-1408) is
spelled as C's own macro is: walk `Vframe_list` while `CONSP`.

**The `ns_frame_parm_handlers` table.**  C 1063-1120 defines a 53-entry array of
`frame_parm_handler` function pointers, including ten explicit nulls.  Swift cannot emit a
C-named *data* symbol, so the port carries the array (`let ns_frame_parm_handlers:
[frame_parm_handler?]`) with the same entries in the same order - the imported handlers as
non-capturing closures over the `@_silgen_name` functions, this file's own setters as closures
over its Swift functions, and `nil` for C's `0` - and the ledger's `## ABI` (3) records that the
C symbol itself still needs a port-side definition.  (`unsafeBitCast` of a Swift function to a
`@convention(c)` pointer is a warning in Swift 6.4 - "changes @convention and is undefined" -
which is why the entries are closures.)

**The two category methods are this file's own definitions.**  `stringWithLispString` and
`lispString` (C 4104, 4143) are what `nsmenu.swift`, `nsfont.swift`, `nsimage.swift` and
`nsselect.swift` import as C-ABI entries; they are `@_cdecl` here, so the name and the selector
shape survive.  The port keeps C's `<!-- invalid string -->` fallback and the two encodings
(UTF-8 when `STRING_MULTIBYTE`, ASCII otherwise).

**Batch 2: `x-create-frame` (C 1192-1610).**  The port keeps C's construction order exactly:
parms copy -> `Qterminal` display lookup -> `check_ns_display_info` -> `kb` -> name ->
`Qparent_id` -> the minibuffer-frame choice -> `XSETFRAME` -> `frame_set_id_from_params` ->
terminal / output_method / the `ns_output` allocation -> fontset -> icon-name -> display_info ->
`record_unwind_protect` -> window/parent descriptors -> name + explicit_name ->
`mac_register_font_driver` -> the ~35 `gui_default_parameter` defaults (with the Cocoa
`userFixedPitchFontOfSize:` font-name default and its `" Regular"` trim) -> `init_frame_faces`
-> min width/height -> `adjust_frame_size` -> undecorated / ns-appearance / transparent-titlebar
-> parent-frame -> z-group / no-focus / no-accept -> menu/tab/tool bar lines ->
buffer-predicate / title -> inhibit-double-buffering -> `get_geometry_from_preferences` ->
`gui_figure_window_size` -> unsplittable -> the fifteen cursors -> visibility -> minibuffer-frame
bookkeeping -> the leftover-parms alist loop -> the cascade/center block -> `unbind_to`.

- **`f->output_data.ns = xzalloc (sizeof *f->output_data.ns)`** (C 1278) is a core-side
  allocation, so it is `frame_alloc_ns_output (f)` - a port-named C-ABI entry (the record
  belongs to the core, as in the batch-1 `ns_output_*` group).
- **The fifteen `Emacs_Cursor` slots** (C 1478-1492) and the two `ns_display_info` scroll-bar
  cursors (C 1494-1497) get one accessor per C member name.  Batch 1 carried an unused
  string-dispatched `ns_output_set_cursor_kind`; that is the wrong shape and was **removed** in
  this batch in favour of the member names.
- **`record_unwind_protect (do_unwind_create_frame, frame)`** (C 1287): `do_unwind_create_frame`
  is this file's own `static` function, so the port passes a non-capturing closure over it.  (A
  `unsafeBitCast` of a Swift function to `@convention(c)` is a *warning* in Swift 6.4 -
  "changes @convention and is undefined" - which the `ns_frame_parm_handlers` table already
  taught this lane.)
- **The 6 `DEFUN`s of this batch** keep the `docs/ports/nsselect.md` shape: the function is
  `@_cdecl("F…")` with C's arity, and the file-static record is a `fileprivate var S…` built by
  `make_aligned_subr` (which batch 1 did not need and this batch adds, together with the
  `PVEC_SUBR << PSEUDOVECTOR_AREA_BITS` header word).  The docstrings stay in the comments that
  introduce each pair - the DOC-scanner gap of (3).

**Batch 2: the file dialog (C 1715-1884).**
- C's file-scope anonymous `static struct { id panel; BOOL ret; … } ns_fd_data` becomes the
  Swift record `ns_fd_data_t` plus one `fileprivate var ns_fd_data`; its `NS_IMPL_GNUSTEP`-only
  members (`dirS`, `initS`, `no_types`) are recorded, not carried, like the `#else` arm of
  `ns_run_file_dialog` (`runModalForDirectory:file:[types:]`).
- `MODAL_OK_RESPONSE`'s live arm is `NSModalResponseOK`, which Swift spells
  `NSApplication.ModalResponse.OK`.
- C's `id panel` is `NSSavePanel?`; the two `NSOpenPanel`-only setters
  (`setCanChooseDirectories:`, `setCanChooseFiles:`) are reached through `as? NSOpenPanel`,
  which is exactly the dynamic type C's `id` has at that point.
- `[NSSavePanel savePanel]` / `[NSOpenPanel openPanel]` are the SDK's shared-panel
  constructors; Swift spells them `NSSavePanel()` / `NSOpenPanel()`.
- `[panel setAllowedContentTypes: [NSArray array]]` (C 1810, the live macOS-12 arm) is
  `panel?.allowedContentTypes = []`.
- `[[NSApp orderedWindows] reverseObjectEnumerator]` (C 1639) is
  `NSApp.orderedWindows.reversed()`, and `((EmacsView *)[win delegate])->emacsframe` (C 1648) is
  the two port-named entries `is_emacs_view` / `emacs_view_frame` - the `EmacsView` record and
  its `emacsframe` ivar belong to src/nsterm.m's port.
- `[EmacsFileDelegate new]` (C 1774) is `EmacsFileDelegate_new ()`: the class itself is
  C 4048-4065, i.e. the next batch (see (3)).

## ABI

### (1) C symbols this batch defines and exports (`@_cdecl`)

| C name | C prototype (reference) | Swift signature |
| --- | --- | --- |
| `ns_implicitly_set_name` | `void (struct frame *, Lisp_Object, Lisp_Object)` (src/nsterm.h 1236) | `(UnsafeMutableRawPointer?, Lisp_Object, Lisp_Object) -> Void` |
| `ns_set_doc_edited` | `void (void)` (src/nsterm.h 1139) | `() -> Void` |
| `ns_change_tab_bar_height` | `void (struct frame *, int)` (src/nsterm.h 1233) | `(UnsafeMutableRawPointer?, Int32) -> Void` |
| `ns_make_frame_key_window` | `void (struct frame *)` (src/nsterm.h 1156) | `(UnsafeMutableRawPointer?) -> Void` |
| `stringWithLispString` | `+ (NSString *)stringWithLispString:(Lisp_Object)` (src/nsterm.h 367) | `(Lisp_Object) -> NSString?` |
| `lispString` | `- (Lisp_Object)lispString` (src/nsterm.h 368) | `(NSString) -> Lisp_Object` |

`nm -g` on the object shows exactly these six (batch 1, Evidence).  Batch 2 adds seven more, and
the object now exports **thirteen** C names (Evidence):

| C name | C prototype (reference) | Swift signature |
| --- | --- | --- |
| `Fx_create_frame` | `Lisp_Object (Lisp_Object)` (DEFUN, src/lisp.h 3406) | `(Lisp_Object) -> Lisp_Object` |
| `Fns_frame_list_z_order` | `Lisp_Object (Lisp_Object)` (0/1) | `(Lisp_Object) -> Lisp_Object` |
| `Fns_frame_restack` | `Lisp_Object (Lisp_Object ×3)` (2/3) | `(Lisp_Object, Lisp_Object, Lisp_Object) -> Lisp_Object` |
| `Fx_select_font` | `Lisp_Object (Lisp_Object ×2)` (0/2) | `(Lisp_Object, Lisp_Object) -> Lisp_Object` |
| `Fns_popup_color_panel` | `Lisp_Object (Lisp_Object)` (0/1) | `(Lisp_Object) -> Lisp_Object` |
| `ns_run_file_dialog` | `void (void)` (src/nsterm.h 1172) | `() -> Void` |
| `Fns_read_file_name` | `Lisp_Object (Lisp_Object ×5)` (1/5) | `(Lisp_Object ×5) -> Lisp_Object` |

The seven `S…` records of the new `DEFUN`s are `fileprivate` here, as C's `DEFUN` makes them
file-static.  The other non-static name in
this range, `ns_frame_parm_handlers` (C 1063), is a **data** symbol: carried as a Swift array,
flagged in (3).  The 52 `DEFUN`s' `F…` names and their `S…` records arrive with the next batch.

### (2) C symbols this file calls (all `@_silgen_name`)

**a. Symbols the reference defines out of line** (the ported core defines them) - 30 + 60 names:

`error`, `build_string`, `store_frame_param`, `get_frame_param`, `update_face_from_frame_parameter`,
`ns_lisp_to_color`, `gui_update_cursor`, `set_frame_cursor_types`, `adjust_frame_size`,
`adjust_frame_glyphs`, `clear_frame`, `clear_current_matrices`, `clear_glyph_matrix`,
`check_int_nonnegative`, `fast_string_match`, `assq_no_quit`, `Fcons`, `Fcar`, `Fcdr`, `Freverse`,
`Fstring_equal`, `Fbuffer_modified_p`, `Fbuffer_file_name`, `defsubr`, `block_input`,
`unblock_input`, `free_frame_menubar`, `update_frame_tool_bar`, `free_frame_tool_bar`,
`ns_clear_frame`

`gui_display_get_arg`, `RES_TYPE_NUMBER`, `RES_TYPE_STRING`, `RES_TYPE_SYMBOL`, `RES_TYPE_BOOLEAN`,
`CHECK_FIXNUM`, `Fassq`, `Fmemq`, `ns_free_frame_resources`, `free_glyphs`, `gui_set_autoraise`,
`gui_set_autolower`, `gui_set_font`, `gui_set_right_divider_width`, `gui_set_bottom_divider_width`,
`gui_set_scroll_bar_width`, `gui_set_scroll_bar_height`, `gui_set_unsplittable`,
`gui_set_vertical_scroll_bars`, `gui_set_horizontal_scroll_bars`, `gui_set_visibility`,
`gui_set_screen_gamma`, `gui_set_line_spacing`, `gui_set_left_fringe`, `gui_set_right_fringe`,
`gui_set_fullscreen`, `gui_set_font_backend`, `gui_set_alpha`, `gui_set_no_special_glyphs`,
`gui_set_alpha_background`, `gui_set_borders_respect_alpha_background`, `ns_set_undecorated`,
`ns_set_parent_frame`, `ns_set_no_focus_on_map`, `ns_set_no_accept_focus`, `ns_set_z_group`,
`ns_set_appearance`, `ns_set_transparent_titlebar`, `selected_frame`, `CHECK_STRING`, `signal_error`,
`NS_FACE_BACKGROUND`, `face_set_background`, `NSColor_colorWithUnsignedLong`, `unsignedLong`,
`frame_set_menu_bar_lines`, `frame_set_external_menu_bar`, `frame_set_tab_bar_height`,
`frame_set_tab_bar_lines`, `frame_set_external_tool_bar`, `frame_set_double_buffered`,
`frame_selected_window`, `window_contents`, `window_current_matrix`, `buffer_filename`,
`setDoubleBuffered`, `FULLSCREEN_NONE`, `FULLSCREEN_WIDTH`, `FULLSCREEN_HEIGHT`,
`FULLSCREEN_MAXIMIZED`

**b. Header-only constructs** - macros, INLINE functions, enumerators, generated-globals
entries; the reference has no symbol for any of them, so the core side exports the C-ABI entry
named here (53 names):

`NILP`, `EQ`, `BASE_EQ`, `CONSP`, `STRINGP`, `SYMBOLP`, `FRAMEP`, `WINDOWP`, `MINI_WINDOW_P`,
`VECTORP`, `FIXNUMP`, `RANGED_FIXNUMP`, `TYPE_RANGED_FIXNUMP`, `XFIXNUM`, `XFIXNAT`, `XCAR`, `XCDR`,
`XFRAME`, `XWINDOW`, `XBUFFER`, `SYMBOL_NAME`, `SSDATA`, `SBYTES`, `STRING_MULTIBYTE`, `SDATA`,
`make_fixnum`, `FRAME_NS_P`, `FRAME_LIVE_P`, `FRAME_VISIBLE_P`, `FRAME_MINIBUF_ONLY_P`,
`FRAME_DISPLAY_INFO`, `FRAME_NS_VIEW`, `FRAME_NS_WINDOW`, `FRAME_NATIVE_WINDOW`,
`FRAME_DEFAULT_FACE`, `FRAME_DOUBLE_BUFFERED`, `FRAME_MENU_BAR_LINES`, `FRAME_EXTERNAL_MENU_BAR`,
`FRAME_EXTERNAL_TOOL_BAR`, `FRAME_LINE_HEIGHT`, `FRAME_TAB_BAR_HEIGHT`, `FRAME_TAB_BAR_LINES`,
`FRAME_TEXT_WIDTH`, `FRAME_TEXT_HEIGHT`, `FRAME_CHILD_FRAME_BORDER_WIDTH`,
`FRAME_INTERNAL_BORDER_WIDTH`, `FRAME_SELECTED_WINDOW`, `SET_FRAME_GARBAGED`, `fset_redisplay`,
`fset_name`, `fset_icon_name`, `fset_title`, `fset_tab_bar_window`

**c. `Q…` symbols** (generated globals.h, PORTING.md P-17; called as `Qfoo ()`) -
19 names:

`Qnil`, `Qt`, `Qtop`, `Qfullscreen`, `Qfullwidth`, `Qicon_type`, `Qforeground_color`,
`Qbackground_color`, `Qcursor_color`, `Qtab_bar_lines`, `Qtool_bar_lines`,
`Qchild_frame_border_width`, `Qinternal_border_width`, `Qwidth`, `Qheight`, `Qleft`, `Qicon_left`,
`Qicon_top`, `Qunbound`

**d. The port's names for the core's records** - each is a C-ABI entry named after the C
expression, because `struct frame`, `struct ns_output`, `struct face`, `struct window`,
`struct buffer` and `struct terminal` belong to the core (44 + 7
names):

`frame_explicit_name`, `frame_set_explicit_name`, `frame_name`, `frame_title`, `frame_icon_name`,
`frame_param_alist`, `frame_tab_bar_window`, `frame_tab_bar_resized`, `frame_set_tab_bar_resized`,
`frame_tab_bar_redisplayed`, `frame_set_child_frame_border_width`, `frame_set_internal_border_width`,
`frame_set_output_method`, `frame_set_foreground_pixel`, `frame_set_background_pixel`,
`frame_set_wants_modeline`, `frame_set_border_width`, `ns_output_view`, `ns_output_set_view`,
`ns_output_miniimage`, `ns_output_set_miniimage`, `ns_output_cursor_color`,
`ns_output_set_cursor_color`, `ns_output_foreground_color`, `ns_output_set_foreground_color`,
`ns_output_background_color`, `ns_output_set_background_color`, `ns_output_icon_top`,
`ns_output_set_icon_top`, `ns_output_icon_left`, `ns_output_set_icon_left`, `ns_output_window_desc`,
`ns_output_set_window_desc`, `ns_output_parent_desc`, `ns_output_set_parent_desc`,
`ns_output_explicit_parent`, `ns_output_set_explicit_parent`, `ns_output_text_cursor`,
`ns_output_set_text_cursor`, `ns_output_set_cursor_kind`, `ns_output_set_current_pointer`,
`ns_output_in_animation`, `ns_output_set_in_animation`, `ns_output_display_info`

**e. The core's variable storage this file reads or writes** (a `@_silgen_name` `var` is an
import of the symbol's storage; C's `extern`):

| C name | reference |
| --- | --- |
| `selected_frame` | `extern Lisp_Object selected_frame` (src/frame.h 1459) |
| `Vframe_list` | `DEFVAR_LISP ("frame-list", …)` (src/frame.c 284) |
| `x_display_list` | `extern struct ns_display_info *x_display_list` (src/nsterm.h 931) |
| `ns_app_name` | `extern NSString *ns_app_name` (src/nsterm.h 775, src/nsterm.m's) |
| `ns_use_proxy_icon` | `DEFVAR_BOOL` (src/nsfns.m 4280) |
| `Vns_icon_type_alist` | `DEFVAR_LISP` (src/nsfns.m 4268) |
| `update_mode_lines` | `extern int update_mode_lines` (src/window.h 1154) |
| `windows_or_buffers_changed` | `extern int windows_or_buffers_changed` (src/window.h 1161) |

**f. Constructs of this lane's headers that other lane files define** (4
names): `free_frame_menubar` / `update_frame_tool_bar` / `free_frame_tool_bar` and
`ns_clear_frame` are src/nsmenu.m's and src/nsterm.m's (this file's port of the setters calls
them exactly as C does); `EmacsImage_allocInitFromFile` is src/nsimage.m's class method;
`setMiniwindowImage`/`fullscreenState`/`setFSValue`/`setDoubleBuffered` are `EmacsView`/
`EmacsLayer` methods in src/nsterm.m; the seven `ns_set_*` handlers the table takes
(`ns_set_undecorated`, `ns_set_parent_frame`, `ns_set_no_focus_on_map`, `ns_set_no_accept_focus`,
`ns_set_z_group`, `ns_set_appearance`, `ns_set_transparent_titlebar`) are src/nsterm.m 1961-2160.

**g. Spellings the port provides itself** (no ABI entry): `eassert`, the `frame_parm_handler`
typealias, the `err_*`/`empty` C strings, and the `ns_color_with_unsigned_long` spelling of
`[NSColor colorWithUnsignedLong:]`.

### (3) What this batch (and the lane) still has to settle

1. **`ns_frame_parm_handlers` is a data symbol.**  Swift cannot emit one; the array is carried,
   the C name needs a definition on the port's C-ABI side (a small C shim in this directory, or
   a getter the Rust side calls).  Same class as `docs/ports/nsselect.md` (3) item 2/3.
2. **`EmacsImage_allocInitFromFile`**: a cross-file *class* method.  The port spells the entry
   with the class in its name; `nsimage.swift`'s port has no such export yet, so
   `ns_implicitly_set_icon_type`'s "special case: t means go by file type" call has no
   definition in the tree today.
3. **`ns_tooltip`** is an `EmacsTooltip *` - src/nsmenu.m's class.  The port keeps it as
   `AnyObject?`; the tooltip section (next batch) is where it gets used.
4. The remaining 21 functions and 52 `DEFUN`s (next batch), listed in the checklist above and
   in `docs/baseline/swift-units.tsv`.
5. **`EmacsFileDelegate`** (C 4048-4065) is a class the next batch ports; `[EmacsFileDelegate
   new]` (C 1774) is imported as `EmacsFileDelegate_new ()` and has no definition in the tree
   yet.  The same shape as `EmacsImage_allocInitFromFile` in item 2.
6. Batch 2's new cross-file entries: `ns_set_appearance_1` (src/nsterm.m 2120),
   `mac_register_font_driver` (src/macfont.m, docs/ports/macfont.md),
   `EmacsView_initFrameFromEmacs`, `showFontPanel`, `restackWindow`, `emacs_view_frame`,
   `is_emacs_view` (src/nsterm.m's `EmacsView`/`EmacsWindow`), `ns_make_frame_visible` /
   `ns_iconify_frame` / `ns_set_offset` (src/nsterm.m 1126-1228) and `ns_clear_frame` - all of
   them are the ported nsterm/macfont side's to define.
7. `tools/extract-abi.py` has no `.m -> .swift` pair, so `--check` cannot compare this unit.

## Evidence

- `swiftc --version` ->
  `swift-driver version: 1.168.6 Apple Swift version 6.4 (swiftlang-6.4.0.34.1 clang-2100.3.34.1)`,
  `Target: arm64-apple-macosx26.0`.
- `cd /Users/liu/projects/src/github.com/liuchong/femacs && swiftc -parse-as-library -c -o
  /tmp/nsfns/nsfns.o femacs/platform/macos/nsfns.swift` -> **exit 0**, 0 errors, **0 warnings**.
- `nm -g /tmp/nsfns/nsfns.o | grep ' T _'`, minus Swift's own `__swift*` symbols -> the six
  exported C names, in address order: `_stringWithLispString`, `_lispString`,
  `_ns_implicitly_set_name`, `_ns_set_doc_edited`, `_ns_change_tab_bar_height`,
  `_ns_make_frame_key_window`.
- `python3 tools/check-swift-units.py` ->
  `femacs/platform/macos/nsfns.swift: 56 C definition(s), 21 unported, 21 row(s); 216 C symbol
  import(s)`; overall last line `140 new, 0 stale, 7 Swift unit(s) landed` (the 140 `new` rows
  are `nsterm.swift`, another lane's in-flight file).  The manifest was refreshed with the
  writer, then reduced to this unit's rows only: `cp docs/baseline/swift-units.tsv
  /tmp/nsfns/swift-units.tsv.bak && python3 tools/check-swift-units.py --write-manifest` ->
  `diff` shows the writer would add **21 nsfns rows *and* 140 nsterm rows**; the backup was
  restored and only the 21 nsfns rows were inserted, so the final `diff` vs the backup is
  **21 added / 0 removed / 0 other files**.
- `python3 tools/check-cond-eval.py src/nsfns.m` ->
  `src/nsfns.m: 64 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- `python3 tools/check-citations.py` -> the last line moves with the other lanes' in-flight
  edits; **zero rows name `femacs/platform/macos/nsfns.swift`** (six did before the pass below
  and were corrected: `NS_IMPL_COCOA`'s citation pointed at nsterm.h 70-76 instead of 36,
  `BVAR` at buffer.h 318 instead of 308, and `src/dispnew.h` - which does not exist in this
  tree - for `adjust_frame_glyphs`/`free_glyphs`/`clear_glyph_matrix`/
  `clear_current_matrices`, all four of which are src/dispextern.h 3901-3907).
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures` (none of them this unit).
- `python3 tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`.
- `python3 tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s),
  0 failure(s)` (neither is this unit).
- Two scratch compiles measured the Swift-spelled SDK names the port needs:
  `NSAppKitVersion.current.rawValue` (the bare `NSAppKitVersionNumber` is an error in Swift
  6.4: "has been renamed to 'NSAppKitVersion.current'") and `NSImage.name()` (a method, not a
  property, on this SDK).
- `/tmp/nsfns` (this lane's probe directory) is deleted after these readings.

### Batch 2 evidence

- `swiftc -parse-as-library -c -o /tmp/nsfns.o femacs/platform/macos/nsfns.swift` -> **exit 0,
  0 errors, 0 warnings** (the batch's only warning - `main_window` never mutated - was fixed).
- `nm -g /tmp/nsfns.o | grep ' T _'` minus Swift's own `__swift*` symbols -> **thirteen**
  exported C names: batch 1's six plus `_Fx_create_frame`, `_Fns_frame_list_z_order`,
  `_Fns_frame_restack`, `_Fx_select_font`, `_Fns_popup_color_panel`, `_ns_run_file_dialog`,
  `_Fns_read_file_name`.
- `python3 tools/check-swift-units.py` ->
  `femacs/platform/macos/nsfns.swift: 56 C definition(s), 19 unported, 19 row(s); 346 C symbol
  import(s)`; last line `0 new, 0 stale, 7 Swift unit(s) landed`.  Two rows left the manifest -
  `ns_run_file_dialog` and `ns_window_is_ancestor`, the two C definitions this batch landed -
  and the `diff` vs the backup is **2 removed / 0 added / 0 other files** (the gate reported
  them as `stale` before the removal).
- `python3 tools/check-cond-eval.py src/nsfns.m` -> `64 conditions, 0 unknown, 0 disagreements
  [VERIFIED]` (unchanged: this batch takes the `NS_IMPL_COCOA` arms the file already had).
- `python3 tools/check-citations.py` -> **zero rows name
  `femacs/platform/macos/nsfns.swift`**; two were corrected in this batch: `ENCODE_FILE`/
  `DECODE_FILE` cited coding.h 800/803 (the file has 790 lines) -> **648/652**, and the
  `PVEC_SUBR`/`PSEUDOVECTOR_AREA_BITS` pair on one citation line (which the gate paired with the
  wrong snippet) -> one citation per line.
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures` (none of them this unit).
