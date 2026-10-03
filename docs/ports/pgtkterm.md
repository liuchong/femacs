# emacs/src/pgtkterm.c (7780 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 238 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 2
- [x] event_queue_t: union buffered_input_event* q, int nr, int cap
- [x] pgtk_window_is_of_frame_recursive_t: GdkWindow* window, bool result, GtkWidget* emacs_gtk_fixed

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/pgtkterm.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
