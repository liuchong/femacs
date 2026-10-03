# emacs/src/androidfont.c (1102 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 35 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 7
- [x] android_emacs_font_driver: jclass class, jmethodID list, jmethodID match, jmethodID list_families, jmethodID open_font, jmethodID has_char, jmethodID text_extents, jmethodID encode_char, jmethodID draw, jmethodID create_font_driver
- [x] android_emacs_font_metrics: jclass class, jfieldID lbearing, jfieldID rbearing, jfieldID width, jfieldID ascent, jfieldID descent
- [x] android_emacs_font_object: jclass class, jfieldID min_width, jfieldID max_width, jfieldID pixel_size, jfieldID height, jfieldID space_width, jfieldID average_width, jfieldID ascent, jfieldID descent, jfieldID underline_thickness, jfieldID underline_position, jfieldID baseline_offset, jfieldID relative_compose, jfieldID default_ascent, jfieldID encoding_charset, jfieldID repertory_charset
- [x] android_emacs_font_spec: jclass class, jfieldID foundry, jfieldID family, jfieldID adstyle, jfieldID registry, jfieldID width, jfieldID weight, jfieldID slant, jfieldID size, jfieldID spacing, jfieldID avgwidth, jfieldID dpi
- [x] android_integer: jclass class, jmethodID constructor, jmethodID int_value
- [x] androidfont_entity: struct font_entity font, jobject object
- [x] androidfont_info: struct font font, jobject object, struct font_metrics** metrics

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/androidfont.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
