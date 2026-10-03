# emacs/src/androidmenu.c (860 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 19 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 4
- [x] android_dismiss_menu_data: jobject menu, jobject window
- [x] android_emacs_context_menu: jclass class, jmethodID create_context_menu, jmethodID add_item, jmethodID add_submenu, jmethodID add_pane, jmethodID parent, jmethodID display, jmethodID dismiss
- [x] android_emacs_dialog: jclass class, jmethodID create_dialog, jmethodID add_button, jmethodID display
- [x] android_menu_subprefix: struct android_menu_subprefix* last, Lisp_Object subprefix

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/androidmenu.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
