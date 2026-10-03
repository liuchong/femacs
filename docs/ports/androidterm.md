# emacs/src/androidterm.c (7117 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 144 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 6
- [x] android_conversion_query_context: struct textconv_callback_struct query, android_window window, bool success
- [x] android_extracted_text_class: jclass class, jmethodID constructor, jfieldID flags, jfieldID partial_start_offset, jfieldID partial_end_offset, jfieldID selection_start, jfieldID selection_end, jfieldID start_offset, jfieldID text
- [x] android_extracted_text_request_class: bool initialized, jfieldID hint_max_chars, jfieldID token
- [x] android_get_extracted_text_context: int hint_max_chars, int token, int flags, char* text, ptrdiff_t length, ptrdiff_t bytes, ptrdiff_t start, ptrdiff_t start_offset, ptrdiff_t end_offset, android_window window, bool mark_active
- [x] android_get_selection_context: android_window window, ptrdiff_t point, ptrdiff_t mark
- [x] android_get_surrounding_text_context: int before_length, int after_length, char* text, ptrdiff_t length, ptrdiff_t bytes, ptrdiff_t offset, ptrdiff_t start, ptrdiff_t end, ptrdiff_t conversion_start, ptrdiff_t conversion_end, android_window window

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/androidterm.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
