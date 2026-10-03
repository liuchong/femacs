# emacs/src/sfntfont.c (4222 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 72 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 7
- [x] sfnt_font_desc: struct sfnt_font_desc* next, Lisp_Object family, Lisp_Object style, Lisp_Object designer, Lisp_Object adstyle, Lisp_Object languages, Lisp_Object registry, Lisp_Object instances, int width, int weight, int slant, int spacing, char* path, Lisp_Object char_cache, struct sfnt_cmap_encoding_subtable subtable, off_t offset, struct sfnt_font_tables* tables, int num_glyphs, int refcount, sfnt_fword underline_position, sfnt_fword underline_thickness, bool_bf underline_position_set : 1, bool cmap_invalid : 1
- [x] sfnt_font_info: struct font font, struct sfnt_font_info* next, struct sfnt_font_desc* desc, struct sfnt_cmap_table* cmap, struct sfnt_hhea_table* hhea, struct sfnt_maxp_table* maxp, struct sfnt_head_table* head, struct sfnt_hmtx_table* hmtx, struct sfnt_glyf_table* glyf, struct sfnt_loca_table_short* loca_short, struct sfnt_loca_table_long* loca_long, struct sfnt_prep_table* prep, struct sfnt_fpgm_table* fpgm, struct sfnt_cvt_table* cvt, struct sfnt_cmap_encoding_subtable_data* cmap_data, struct sfnt_cmap_encoding_subtable cmap_subtable, struct sfnt_uvs_context* uvs, struct sfnt_outline_cache outline_cache, int outline_cache_size, struct sfnt_raster_cache raster_cache, int raster_cache_size, struct sfnt_interpreter* interpreter, struct sfnt_graphics_state state, sfnt_fixed scale, struct sfnt_blend blend, int instance, bool glyf_table_mapped
- [x] sfnt_font_tables: struct sfnt_cmap_table* cmap, struct sfnt_hhea_table* hhea, struct sfnt_maxp_table* maxp, struct sfnt_head_table* head, struct sfnt_hmtx_table* hmtx, struct sfnt_glyf_table* glyf, struct sfnt_loca_table_short* loca_short, struct sfnt_loca_table_long* loca_long, struct sfnt_prep_table* prep, struct sfnt_fpgm_table* fpgm, struct sfnt_cvt_table* cvt, struct sfnt_fvar_table* fvar, struct sfnt_avar_table* avar, struct sfnt_gvar_table* gvar, struct sfnt_cvar_table* cvar, struct sfnt_cmap_encoding_subtable_data* cmap_data, struct sfnt_cmap_encoding_subtable cmap_subtable, struct sfnt_uvs_context* uvs, bool glyf_table_mapped
- [x] sfnt_outline_cache: struct sfnt_outline_cache* next, struct sfnt_outline_cache* last, struct sfnt_glyph_outline* outline, struct sfnt_glyph_metrics metrics, sfnt_glyph glyph
- [x] sfnt_raster_cache: struct sfnt_raster_cache* next, struct sfnt_raster_cache* last, struct sfnt_raster* raster, sfnt_glyph glyph
- [x] sfnt_style_desc: const char* c_string, int value
- [x] sfntfont_get_glyph_outline_dcontext: struct sfnt_loca_table_long* loca_long, struct sfnt_loca_table_short* loca_short, struct sfnt_glyf_table* glyf, struct sfnt_hmtx_table* hmtx, struct sfnt_hhea_table* hhea, struct sfnt_maxp_table* maxp, struct sfnt_blend* blend

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/sfntfont.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
