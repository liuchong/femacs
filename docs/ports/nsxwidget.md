# emacs/src/nsxwidget.m (637 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 22
- [x] nsxwidget_is_web_view  (line 330)
- [x] nsxwidget_webkit_uri  (line 337)
- [x] nsxwidget_webkit_title  (line 344)
- [x] nsxwidget_webkit_goto_uri  (line 353)
- [x] nsxwidget_webkit_goto_history  (line 363)
- [x] nsxwidget_webkit_estimated_load_progress  (line 374)
- [x] nsxwidget_webkit_stop_loading  (line 381)
- [x] nsxwidget_webkit_zoom  (line 388)
- [x] js_to_lisp  (line 398, static)
- [x] nsxwidget_webkit_execute_script  (line 454)
- [x] nsxwidget_init  (line 490)
- [x] nsxwidget_kill  (line 507)
- [x] nsxwidget_resize  (line 535)
- [x] nsxwidget_get_size  (line 545)
- [x] nsxwidget_init_view  (line 558)
- [x] nsxwidget_delete_view  (line 586)
- [x] nsxwidget_show_view  (line 599)
- [x] nsxwidget_hide_view  (line 607)
- [x] nsxwidget_resize_view  (line 614)
- [x] nsxwidget_move_view  (line 620)
- [x] nsxwidget_move_widget_in_view  (line 627)
- [x] nsxwidget_set_needsdisplay  (line 634)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 1
- [x] static NSString xwScript  (line 287, static)

## Decisions (2026-10-17, before the port)

- **Target**: `femacs/platform/macos/nsxwidget.swift`, the lane `docs/port-map.md` names
  (`src/nsxwidget.m` -> Swift).  The unit is **not compiled by the frozen configuration**
  (`--with-ns=no`), like the other seven `.m` lanes; it exists for 1:1 coverage, and
  `tools/check-swift-units.py` is what will check its function set against this list.
- **Structure** follows the other Swift lanes (`nsimage.swift` is the closest): a Lane-ABI
  section that imports what the C core exports (`@_silgen_name`) and declares the C types the
  bodies touch (`struct xwidget`, `struct xwidget_view`, `glyph_string`'s `f`, and the
  `FRAME_NS_VIEW` accessor), then the ObjC classes as `@objc` Swift classes with the same
  selectors, then the 22 C functions as `@_cdecl`.
- **What is carried verbatim**: the `#if 0` plugin arm (C 73-79) and the `decisionHandler`
  TODO comment (C 141) stay as recorded absences with their C line numbers, as in the other
  lanes (PORTING.md 0.1/P-08).
- **What needs a decision at port time**: `xwScript` is a `static NSString *` initialised in
  `+initialize` (C 271-303); Swift has no `+initialize`, so the initialisation becomes a
  `let` computed once (same string, same content) and the file records the difference.
- **Owner of the gap**: PORTING.md P-197 (the platform-inventory gate runs `--warn` until this
  file exists).
