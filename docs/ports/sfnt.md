# emacs/src/sfnt.c (21215 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 385 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 7
- [x] sfnt_build_glyph_outline_context: struct sfnt_glyph_outline* outline, sfnt_fixed factor, sfnt_fixed x, sfnt_fixed y
- [x] sfnt_compound_glyph_context: sfnt_fixed* x_coordinates, sfnt_fixed* y_coordinates, unsigned char* flags, size_t num_points, size_t points_size, size_t* contour_end_points, size_t num_end_points, size_t end_points_size, sfnt_f26dot6 phantom_point_1_x, sfnt_f26dot6 phantom_point_2_x, sfnt_f26dot6 phantom_point_1_y, sfnt_f26dot6 phantom_point_2_y, sfnt_f26dot6 phantom_point_1_s, sfnt_f26dot6 phantom_point_2_s
- [x] sfnt_fedge: struct sfnt_fedge* next, int winding, float x, float top, float bottom, float step_x, float step_y
- [x] sfnt_gvar_glyph_header: uint16_t tuple_count, uint16_t data_offset
- [x] sfnt_step: struct sfnt_step* next, int x, float coverage
- [x] sfnt_step_chunk: struct sfnt_step_chunk* next, size_t nused, struct sfnt_step steps
- [x] sfnt_step_raster: size_t scanlines, struct sfnt_step** steps, struct sfnt_step_chunk* chunks

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/sfnt.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
