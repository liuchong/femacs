# emacs/src/sfnt.h (2151 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 8
- [x] PROTOTYPE  (line 2125, body 3149cd5fc72c)
- [x] SFNT_CEIL_FIXED  (line 1457, body a3a33da525da)
- [x] SFNT_ENDOF  (line 59, body e3aed58dce08)
- [x] SFNT_FLOOR_FIXED  (line 1459, body 95312374f3a5)
- [x] SFNT_ROUND_FIXED  (line 1458, body 8c27aa6f34a1)
- [x] _SFNT_H_  (line 21, body da39a3ee5e6b)
- [x] sfnt_coerce_fixed  (line 125, body 2828bb17c2ce)
- [x] sfnt_fixed_float  (line 126, body 85be771acf56)

## Types and their fields — 68
- [x] sfnt_OS_2_table: uint16_t version, int16_t x_avg_char_width, uint16_t us_weight_class, uint16_t us_width_class, int16_t fs_type, int16_t y_subscript_x_size, int16_t y_subscript_y_size, int16_t y_subscript_x_offset, int16_t y_subscript_y_offset, int16_t y_superscript_x_size, int16_t y_superscript_y_size, int16_t y_superscript_x_offset, int16_t y_superscript_y_offset, int16_t y_strikeout_size, int16_t y_strikeout_position, int16_t s_family_class, unsigned char panose, uint32_t ul_unicode_range, char ach_vendor_id, uint16_t fs_selection, uint16_t fs_first_char_index, uint16_t fs_last_char_index
- [x] sfnt_avar_table: sfnt_fixed version, int32_t axis_count, struct sfnt_short_frac_segment* segments
- [x] sfnt_blend: struct sfnt_fvar_table* fvar, struct sfnt_gvar_table* gvar, struct sfnt_avar_table* avar, struct sfnt_cvar_table* cvar, sfnt_fixed* coords, sfnt_fixed* norm_coords
- [x] sfnt_cmap_encoding_subtable: uint16_t platform_id, uint16_t platform_specific_id, uint32_t offset
- [x] sfnt_cmap_encoding_subtable_data: uint16_t format, uint16_t length
- [x] sfnt_cmap_format_0: uint16_t format, uint16_t length, uint16_t language, uint8_t glyph_index_array
- [x] sfnt_cmap_format_12: uint16_t format, uint16_t reserved, uint32_t length, uint32_t language, uint32_t num_groups, struct sfnt_cmap_format_8_or_12_group* groups
- [x] sfnt_cmap_format_14: uint16_t format, uint32_t length, uint16_t num_var_selector_records, off_t offset, struct sfnt_variation_selector_record* records
- [x] sfnt_cmap_format_2: uint16_t format, uint16_t length, uint16_t language, uint16_t sub_header_keys, struct sfnt_cmap_format_2_subheader* subheaders, uint16_t* glyph_index_array, uint16_t num_glyphs
- [x] sfnt_cmap_format_2_subheader: uint16_t first_code, uint16_t entry_count, int16_t id_delta, uint16_t id_range_offset
- [x] sfnt_cmap_format_4: uint16_t format, uint16_t length, uint16_t language, uint16_t seg_count_x2, uint16_t search_range, uint16_t entry_selector, uint16_t range_shift, uint16_t* end_code, uint16_t* reserved_pad, uint16_t* start_code, int16_t* id_delta, int16_t* id_range_offset, uint16_t* glyph_index_array, size_t glyph_index_size
- [x] sfnt_cmap_format_6: uint16_t format, uint16_t length, uint16_t language, uint16_t first_code, uint16_t entry_count, uint16_t* glyph_index_array
- [x] sfnt_cmap_format_8: uint16_t format, uint16_t reserved, uint32_t length, uint32_t language, uint8_t is32, uint32_t num_groups, struct sfnt_cmap_format_8_or_12_group* groups
- [x] sfnt_cmap_format_8_or_12_group: uint32_t start_char_code, uint32_t end_char_code, uint32_t start_glyph_code
- [x] sfnt_cmap_table: uint16_t version, uint16_t num_subtables
- [x] sfnt_compound_glyph: struct sfnt_compound_glyph_component* components, size_t num_components, uint8_t* instructions, uint16_t instruction_length
- [x] sfnt_compound_glyph_component: uint16_t flags, uint16_t glyph_index, uint8_t a, int8_t b, uint16_t c, int16_t d, <aggregate> argument1, uint8_t a, int8_t b, uint16_t c, int16_t d, <aggregate> argument2, int16_t scale, int16_t xscale, int16_t yscale, <aggregate> a, int16_t xscale, int16_t scale01, int16_t scale10, int16_t yscale, <aggregate> b, <aggregate> u
- [x] sfnt_cvar_table: sfnt_fixed version, uint16_t tuple_count, uint16_t data_offset, struct sfnt_tuple_variation* variation
- [x] sfnt_cvt_table: size_t num_elements, sfnt_fword* values
- [x] sfnt_default_uvs_table: uint32_t num_unicode_value_ranges, struct sfnt_unicode_value_range* ranges
- [x] sfnt_edge: struct sfnt_edge* next, int winding, sfnt_fixed x, sfnt_fixed top, sfnt_fixed bottom, sfnt_fixed step_x
- [x] sfnt_fpgm_table: size_t num_instructions, unsigned char* instructions
- [x] sfnt_fvar_table: uint16_t major_version, uint16_t minor_version, uint16_t offset_to_data, uint16_t count_size_pairs, uint16_t axis_count, uint16_t axis_size, uint16_t instance_count, uint16_t instance_size, struct sfnt_variation_axis* axis, struct sfnt_instance* instance
- [x] sfnt_glyf_table: size_t size, unsigned char* glyphs, unsigned char* start
- [x] sfnt_glyph: int16_t number_of_contours, sfnt_fword xmin, sfnt_fword ymin, sfnt_fword xmax, sfnt_fword ymax, sfnt_fword advance_distortion, sfnt_fword origin_distortion, struct sfnt_simple_glyph* simple, struct sfnt_compound_glyph* compound
- [x] sfnt_glyph_metrics: sfnt_fixed lbearing, sfnt_fixed advance
- [x] sfnt_glyph_outline: struct sfnt_glyph_outline_command* outline, size_t outline_size, size_t outline_used, sfnt_fixed xmin, sfnt_fixed ymin, sfnt_fixed xmax, sfnt_fixed ymax, sfnt_fixed origin, short refcount
- [x] sfnt_glyph_outline_command: int flags, sfnt_fixed x, sfnt_fixed y
- [x] sfnt_graphics_state: sfnt_f26dot6 (*) round, sfnt_f26dot6 (*) project, sfnt_f26dot6 (*) dual_project, void (*) move, sfnt_f2dot14 vector_dot_product, bool auto_flip, sfnt_f26dot6 cvt_cut_in, unsigned short delta_base, unsigned short delta_shift, struct sfnt_unit_vector dual_projection_vector, struct sfnt_unit_vector freedom_vector, unsigned char instruct_control, unsigned short loop, sfnt_f26dot6 minimum_distance, struct sfnt_unit_vector projection_vector, int round_state, uint16_t rp0, uint16_t rp1, uint16_t rp2, int scan_control, sfnt_f26dot6 sw_cut_in, sfnt_f26dot6 single_width_value, int zp0, int zp1, int zp2
- [x] sfnt_gvar_table: uint16_t version, uint16_t reserved, uint16_t axis_count, uint16_t shared_coord_count, uint32_t offset_to_coord, uint16_t glyph_count, uint16_t flags, uint32_t offset_to_data, size_t data_size, uint16_t* offset_word, uint32_t* offset_long, <aggregate> u, sfnt_f2dot14* global_coords, unsigned char* glyph_variation_data
- [x] sfnt_head_table: sfnt_fixed version, sfnt_fixed revision, uint32_t checksum_adjustment, uint32_t magic, uint16_t flags, uint16_t units_per_em, uint32_t created_high, uint32_t created_low, uint32_t modified_high, uint32_t modified_low, sfnt_fword xmin, sfnt_fword ymin, sfnt_fword xmax, sfnt_fword ymax, uint16_t mac_style, uint16_t lowest_rec_ppem, int16_t font_direction_hint, int16_t index_to_loc_format, int16_t glyph_data_format
- [x] sfnt_hhea_table: sfnt_fixed version, sfnt_fword ascent, sfnt_fword descent, sfnt_fword line_gap, sfnt_ufword advance_width_max, sfnt_fword min_left_side_bearing, sfnt_fword min_right_side_bearing, sfnt_fword x_max_extent, int16_t caret_slope_rise, int16_t caret_slope_run, sfnt_fword caret_offset, int16_t reserved1, int16_t reserved2, int16_t reserved3, int16_t reserved4, int16_t metric_data_format, uint16_t num_of_long_hor_metrics
- [x] sfnt_hmtx_table: struct sfnt_long_hor_metric* h_metrics, int16_t* left_side_bearing
- [x] sfnt_instance: uint16_t name_id, uint16_t flags, uint16_t ps_name_id, sfnt_fixed* coords
- [x] sfnt_instructed_outline: size_t num_points, size_t num_contours, size_t* contour_end_points, sfnt_f26dot6 *restrict x_points, sfnt_f26dot6 *restrict y_points, unsigned char* flags
- [x] sfnt_interpreter: uint16_t max_stack_elements, uint16_t num_instructions, uint16_t storage_size, uint16_t function_defs_size, uint16_t instruction_defs_size, uint16_t twilight_zone_size, int IP, sfnt_fixed scale, int ppem, int point_size, uint32_t* stack, uint32_t* SP, size_t cvt_size, unsigned char *restrict instructions, sfnt_f26dot6 *restrict twilight_x, sfnt_f26dot6 *restrict twilight_y, sfnt_f26dot6 *restrict twilight_original_x, sfnt_f26dot6 *restrict twilight_original_y, struct sfnt_interpreter_zone* glyph_zone, sfnt_f26dot6 advance_width, uint32_t* storage, sfnt_f26dot6* cvt, struct sfnt_interpreter_definition* function_defs, struct sfnt_interpreter_definition* instruction_defs, struct sfnt_graphics_state state, sfnt_f26dot6 period, sfnt_f26dot6 phase, sfnt_f26dot6 threshold, int call_depth, jmp_buf trap, const char* trap_reason, int n_axis, sfnt_fixed* norm_coords
- [x] sfnt_interpreter_definition: uint16_t opcode, uint16_t instruction_count, unsigned char* instructions
- [x] sfnt_interpreter_zone: size_t num_points, size_t num_contours, size_t* contour_end_points, sfnt_f26dot6 *restrict x_points, sfnt_f26dot6 *restrict x_current, sfnt_f26dot6 *restrict y_points, sfnt_f26dot6 *restrict y_current, unsigned char* flags, struct sfnt_simple_glyph* simple
- [x] sfnt_loca_table_long: uint32_t* offsets, size_t num_offsets
- [x] sfnt_loca_table_short: uint16_t* offsets, size_t num_offsets
- [x] sfnt_long_hor_metric: uint16_t advance_width, int16_t left_side_bearing
- [x] sfnt_mapped_table: void* data, void* mapping, size_t length, size_t size
- [x] sfnt_mapped_variation_selector_record: unsigned int selector, struct sfnt_default_uvs_table* default_uvs, struct sfnt_nondefault_uvs_table* nondefault_uvs
- [x] sfnt_maxp_table: sfnt_fixed version, uint16_t num_glyphs, uint16_t max_points, uint16_t max_contours, uint16_t max_composite_points, uint16_t max_composite_contours, uint16_t max_zones, uint16_t max_twilight_points, uint16_t max_storage, uint16_t max_function_defs, uint16_t max_instruction_defs, uint16_t max_stack_elements, uint16_t max_size_of_instructions, uint16_t max_component_elements, uint16_t max_component_depth
- [x] sfnt_meta_data_map: uint32_t tag, uint32_t data_offset, uint32_t data_length
- [x] sfnt_meta_table: uint32_t version, uint32_t flags, uint32_t data_offset, uint32_t num_data_maps, struct sfnt_meta_data_map* data_maps, unsigned char* data
- [x] sfnt_metrics_distortion: sfnt_fword origin, sfnt_fword advance
- [x] sfnt_name_record: uint16_t platform_id, uint16_t platform_specific_id, uint16_t language_id, uint16_t name_id, uint16_t length, uint16_t offset
- [x] sfnt_name_table: uint16_t format, uint16_t count, uint16_t string_offset, struct sfnt_name_record* name_records, unsigned char* data
- [x] sfnt_nondefault_uvs_table: uint32_t num_uvs_mappings, struct sfnt_uvs_mapping* mappings
- [x] sfnt_offset_subtable: uint32_t scaler_type, uint16_t num_tables, uint16_t search_range, uint16_t entry_selector, uint16_t range_shift, struct sfnt_table_directory* subtables
- [x] sfnt_point: sfnt_fixed x, sfnt_fixed y
- [x] sfnt_post_table: sfnt_fixed format, sfnt_fixed italic_angle, sfnt_fword underline_position, sfnt_fword underline_thickness, uint32_t is_fixed_pitch, uint32_t min_mem_type_42, uint32_t max_mem_type_42, uint32_t min_mem_type_1, uint32_t max_mem_type_1
- [x] sfnt_prep_table: size_t num_instructions, unsigned char* instructions
- [x] sfnt_raster: unsigned char* cells, unsigned short width, unsigned short height, short offx, short offy, unsigned short stride, unsigned short refcount
- [x] sfnt_short_frac_correspondence: sfnt_f2dot14 from_coord, sfnt_f2dot14 to_coord
- [x] sfnt_short_frac_segment: uint16_t pair_count, struct sfnt_short_frac_correspondence* correspondence
- [x] sfnt_simple_glyph: size_t number_of_points, uint16_t *restrict end_pts_of_contours, uint16_t instruction_length, uint8_t *restrict instructions, uint8_t *restrict flags, int16_t *restrict x_coordinates, int16_t *restrict y_coordinates, int16_t *restrict y_coordinates_end
- [x] sfnt_table_directory: uint32_t tag, uint32_t checksum, uint32_t offset, uint32_t length
- [x] sfnt_table_offset_rec: off_t offset, bool is_nondefault_table, void* table
- [x] sfnt_ttc_header: uint32_t ttctag, uint32_t version, uint32_t num_fonts, uint32_t* offset_table, uint32_t ul_dsig_tag, uint32_t ul_dsig_length, uint32_t ul_dsig_offset
- [x] sfnt_tuple_variation: uint16_t* points, sfnt_fword* deltas, sfnt_f2dot14* coordinates, sfnt_f2dot14 *restrict intermediate_start, sfnt_f2dot14 *restrict intermediate_end, uint16_t num_points
- [x] sfnt_unicode_value_range: unsigned int start_unicode_value, unsigned char additional_count
- [x] sfnt_unit_vector: sfnt_f2dot14 x, sfnt_f2dot14 y
- [x] sfnt_uvs_context: size_t num_records, size_t nmemb, struct sfnt_table_offset_rec* tables, struct sfnt_mapped_variation_selector_record* records
- [x] sfnt_uvs_mapping: unsigned int unicode_value, uint16_t base_character_value
- [x] sfnt_variation_axis: uint32_t axis_tag, sfnt_fixed min_value, sfnt_fixed default_value, sfnt_fixed max_value, uint16_t flags, uint16_t name_id
- [x] sfnt_variation_selector_record: unsigned int var_selector, uint32_t default_uvs_offset, uint32_t nondefault_uvs_offset

## File-scope objects — 0

## Decisions

- **The `#if` inventory: seven conditions, one dead arm** (`tools/check-cond-eval.py src/sfnt.h`
  -> `7 conditions, 0 unknown, 0 disagreements [VERIFIED]`; the file is in
  `docs/baseline/config/cond-eval-verified.txt`, so P-08's frozen answers are what the mirror
  carries).  Live: C 20-21 the guard; C 1068-1084 `#if defined HAVE_MMAP && !defined TEST`
  (`HAVE_MMAP` is 1 in `docs/baseline/config/darwin-arm64.macros.tsv`, `TEST` is undefined) -
  `sfnt_mapped_table`; C 1467-1647 `#ifndef TEST` with the two `HAVE_MMAP` pairs at C 1489-1492
  and C 1579-1585 - the first ~45 prototypes; C 2071-2147 `#ifndef TEST` - the hinting
  prototypes and `sfnt_vary_interpreter`.  Dead: **C 2026-2038** (`#ifdef TEST`), the three hook
  members of `struct sfnt_interpreter` - `void (*run_hook) (struct sfnt_interpreter *);` (C
  2029), `void (*push_hook) (struct sfnt_interpreter *, uint32_t);` (C 2033) and
  `void (*pop_hook) (struct sfnt_interpreter *, uint32_t);` (C 2037).  They are **not
  transcribed**: the header policy (PORTING.md P-66 rule 2 / P-08, the `dispextern.rs` shape) is
  that a mirror carries the frozen configuration's live text and marks the boundary, which the
  mirror does at that member.  `tools/extract-abi.py --check` agrees - its field list for
  `sfnt_interpreter` ends at `norm_coords` (verified by reading `ci.inventory` with the macro
  table installed: the three names are absent).
- **Nothing here degrades to an untyped pointer, and `restrict` is the only C qualifier that
  cannot come along.**  Every pointee the text names is a record *this* header declares (the two
  forward declarations C 698 and C 1095 are defined later in the same file), so `struct X *` is
  always `*mut X`; `void *`/`char *`/`unsigned char *` are C's own untyped pointers and map to
  `*mut c_void`/`*mut c_char`/`*mut u8` directly.  `restrict` (six members: `sfnt_simple_glyph`
  583-601, `sfnt_tuple_variation` 1212/1215, `sfnt_interpreter_zone` 1749-1758,
  `sfnt_interpreter` 1964-1977, `sfnt_graphics_state.move` 1814-1815, `sfnt_instructed_outline`
  2061) has no Rust counterpart and is dropped; each keeps the `/// C:` line P-55 requires with
  C's own spelling.
- **`sfnt_glyph` is one name in C and two types: the record keeps it, the scalar typedef is
  renamed (P-67 deviation, measured `E0428`).**  C 128's `typedef unsigned int sfnt_glyph;` and
  C 661's `struct sfnt_glyph` are legal side by side in C's two namespaces and impossible in
  Rust's single type namespace: `xtask layout-check` stopped the first compile of this mirror
  with `error[E0428]: the name `sfnt_glyph` is defined multiple times`.  The port keeps the
  **record** name (the extractor's item list demands it, and the record's nine fields therefore
  stay compared) and gives the scalar the name C's own declarators use for it,
  `sfnt_glyph_index`.  The rename is recorded here as the deviation P-67 asks for.  *Lead for
  the integrator*: the alternative shape is to keep the scalar's name (it is the more frequent
  spelling - `sfnt.c` has 51 scalar uses against 28 `struct sfnt_glyph` uses, `sfntfont.c` 14
  against 4) and put the record in a module (`sfnt_glyph_record::sfnt_glyph`), which the
  extractor's text scan still checks field by field; that is a call for the integrator, not for
  this lane.
- **`PROTOTYPE` is C's parameter-list trick and has no Rust counterpart; the exclusion row is
  in the table.**  C 1471, 1481, 1500, 1514, 1519, 1537, 1545, 1557, 1565, 1571, 1594, 1602,
  1633, 1641, 2073, 2081, 2092, 2100, 2109, 2115 and 2125 each `#define PROTOTYPE` one entry
  point's parameters and `#undef` it after, so one signature serves several prototypes.  Rust
  has no textual parameter-list macro, so every declaration in the mirror writes its
  parameters out and the name has no Rust item and no use site.  `--check` used to report it
  as `missing macro PROTOTYPE`; the interim answer was a `macro_rules! PROTOTYPE` that carried
  the name and nothing else (the `lisp_h_Qnil` class).  The integrator has since written the
  row into `docs/ports/excluded-macros.txt` (152, `kind=syntax`), and this round **deleted**
  the placeholder macro: with the row present the checker finds the name in the exclusion
  list, `--check` still prints `OK`, and the mirror no longer carries a construct it cannot
  use.
- **Two C-spelling traps are handled in the bodies, not by shared rows.**  (1) `sfnt_fixed_float`
  (C 126) multiplies by `65535.0f`, and the token comparison sees C's `f` as its own token while
  Rust's only float suffixes (`f32`/`f64`) are skipped - the mirror names the divisor `f`, so the
  token is in the body and no `macro-body-notes.tsv` row is needed.  (2) `SFNT_CEIL_FIXED`/
  `SFNT_ROUND_FIXED`/`SFNT_FLOOR_FIXED` (C 1457-1459) mask with `037777600000` = 0xFFFF0000 =
  4294901760, which does not fit a C `int` either: C's literal is `unsigned int`, so the port
  writes the unsigned arithmetic out (`fixed as u32`, `wrapping_add`, `as sfnt_fixed`) because
  an `i32` literal of that value is not a Rust literal (`error: literal out of range for i32`,
  measured).  The octal spellings are Rust's `0o` form of the same three values, which is what
  the token comparison folds.
- **The anonymous aggregates get P-67 names, and two field names are Rust keywords.**
  `sfnt_compound_glyph_component`'s two argument unions and its scaling union become
  `sfnt_compound_glyph_component_argument1`/`_argument2`/`_u` (with `_u_a`/`_u_b` for the two
  structs inside the scaling union), and `sfnt_gvar_table`'s offset union becomes
  `sfnt_gvar_table_u`; the member itself keeps C's name (`argument1`, `argument2`, `u`).
  `sfnt_graphics_state.move` (C 1814) and `sfnt_graphics_state.loop` (C 1884) are Rust keywords
  and are spelled `r#move`/`r#loop` (the extractor strips the `r#`, which is what
  `xtask`'s generated assertions do for `r#type`, status's `bidi_saved_info` note).
- **`jmp_buf` is the unit's, and the record's layout is C's anyway.**  C 25's `#include
  <setjmp.h>` has no mirror in the port; the name's owner is `lisp.rs` (1058, the frozen layout
  baseline's 192-byte `jmp_buf`), which a translation unit supplies through its own includes
  (PORTING.md 0.1-9), so `sfnt_interpreter.trap` (C 2015) names `jmp_buf` without an `include!`.
  `lisp.rs`'s alias is the opaque `[u8; 192]` (alignment 1) where C's type is `int[48]`
  (alignment 4, measured): the member's *offset* and every offset after it are still C's because
  the byte count is the same and the next member is 8-aligned, which is why `sizeof
  sfnt_interpreter` (480) and `alignof` (8) are assertable and are asserted.
- **The layout assertions are the mirror's own, because these records are not in the
  whitelist.**  `tools/gen-layout-fields.py`'s `STRUCTS` does not list any `sfnt_*` type, so
  `xtask layout-check`'s generated keys do not cover them; the mirror carries 76 handwritten
  `const _` assertions (21 `sizeof`/`alignof` pairs plus 34 field offsets) measured with the
  frozen clang - the `font.rs` shape.  `xtask layout-check` compiles and runs them.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/sfnt.h src/include/sfnt.rs`
  -> `OK emacs/src/sfnt.h -> src/include/sfnt.rs` (no `- ` problem lines); the run prints
  `(1 C macros have a recorded Rust counterpart: PROTOTYPE)` after the `macro_rules!`
  placeholder was deleted - the name is answered by `docs/ports/excluded-macros.txt` now.
- `python3 tools/check-cond-eval.py src/sfnt.h` -> `7 conditions, 0 unknown to the evaluator,
  0 disagreements`.
- This ledger: 0 subrs, 0 functions defined, 8 macros, 68 types, 0 file-scope objects.
- **Layout, measured with the frozen clang** (a probe that includes `sfnt.h`:
  `clang -std=gnu23 -O0 -I build/emacs-ref/lib -idirafter spikes/shim -I build/emacs-ref/src
  -I emacs/src -I emacs/lib <probe>.c`).  Selected readings, all asserted in the mirror:
  `sizeof jmp_buf` 192 / `_Alignof` 4; `struct sfnt_offset_subtable` 24/8 with `scaler_type@0`,
  `num_tables@4`, `subtables@16`; `struct sfnt_table_directory` 16/4 (`length@12`);
  `struct sfnt_cmap_format_0` 262/2 (`glyph_index_array@6`); `struct sfnt_cmap_format_8`
  65560/8 (`is32@12`, `num_groups@65548`, `groups@65552`); `struct sfnt_maxp_table` 32/4;
  `struct sfnt_post_table` 32/4; `struct sfnt_simple_glyph` 64/8 (`instruction_length@16`);
  `struct sfnt_compound_glyph_component` 16/2 (`argument1@4`, `argument2@6`, `u@8`);
  `struct sfnt_glyph` 32/8 (`simple@16`, `compound@24`); `struct sfnt_point` 8/4;
  `struct sfnt_glyph_outline` 48/8 (`refcount@44`); `struct sfnt_raster` 24/8 (`refcount@18`);
  `struct sfnt_gvar_table` 56/8 (`u@32`, `global_coords@40`, `glyph_variation_data@48`);
  `struct sfnt_blend` 48/8 (`norm_coords@40`); `struct sfnt_OS_2_table` 72/4 (`panose@32`,
  `ul_unicode_range@44`, `ach_vendor_id@60`, `fs_selection@64`, `fs_last_char_index@68`);
  `struct sfnt_mapped_table` 32/8 (`size@24`); `struct sfnt_table_offset_rec` 24/8 (`table@16`);
  `struct sfnt_uvs_context` 32/8; `struct sfnt_graphics_state` 104/8 (`rp0@68`, `zp0@88`);
  `struct sfnt_interpreter` 480/8 (`ppem@20`, `trap@264`, `trap_reason@456`,
  `norm_coords@472`); `struct sfnt_instructed_outline` 48/8 (`flags@40`).
- **Compile proof** (no unit includes this mirror, so the compiler is the single-crate probe
  plus the orphan probe): `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-sfnt cargo run -q -p
  xtask -- layout-check` -> `layout probe ok` / `layout-check passed (see
  build/femacs/layout_asserts.rs)`; `cargo run -q -p xtask -- probe
  ../spikes/sfnt_inc_probe.rs ../build/probes/sfnt_inc_probe.rmeta --crate-type=lib
  --emit=metadata` -> exit 0, 0 warnings (the probe is registered in `xtask`'s `PROBES`, so
  `gate-fast` re-measures it every round).
- **Consumer smoke**: `cargo run -q -p xtask -- build-tu src/sfnt.c` ->
  `built src/sfnt.rs -> ../build/femacs/sfnt.o`; `build-tu src/sfntfont.c` ->
  `built src/sfntfont.rs -> ../build/femacs/sfntfont.o` (both are the empty-shape units of
  docs/status.md 3.31, which is what the frozen configuration builds for these files).  The
  header-form `build-tu src/sfnt.h` cannot work for *any* mirror that names `c_int` - the crate
  root is the mirror, which carries no `use` - measured identically for `src/w32gui.h`,
  `src/menu.h`, `src/cygw32.h` and the landed `src/w32inevt.h`.
- Gate readings on this worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed
  literals) / 0 failures; `check-globals` 283 units / 0; `check-mirror-dupes` 1 duplicate
  declaration / 1 ratchet row / 0 failures; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 580 `.rs` / 592 map targets / 0 findings.
