# emacs/src/pgtkselect.c (1975 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 50 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 3
- [x] pgtk_selection_request: struct pgtk_selection_request* last, struct pgtk_display_info* dpyinfo, struct selection_input_event* request, struct selection_data* converted_selections, GdkAtom conversion_fail_tag, bool converted
- [x] prop_location: int identifier, GdkDisplay* display, GdkWindow* window, GdkAtom property, int desired_state, bool arrived, struct prop_location* next
- [x] selection_data: unsigned char* data, ptrdiff_t size, int format, GdkAtom type, bool nofree, GdkAtom property, struct prop_location* wait_object, struct selection_data* next

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/pgtkselect.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
