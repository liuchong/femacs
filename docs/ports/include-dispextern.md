# emacs/src/dispextern.h (4025 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 9
- [ ] xstrcasecmp  (line 108)
- [x] GLYPH_CODE_CHAR  (line 401)
- [x] GLYPH_CODE_FACE  (line 409)
- [ ] face_tty_specified_color  (line 1946)
- [ ] FACE_SUITABLE_FOR_ASCII_CHAR_P  (line 2016)
- [ ] FACE_FOR_CHAR  (line 2031)
- [x] GLYPH_CHAR_VALID_P  (line 2043)
- [ ] GLYPH_CODE_P  (line 2053)
- [ ] reset_mouse_highlight  (line 3027)

## Macros defined — 108
- [ ] BIDI_AT_BASE_LEVEL  (line 2207, body 971025605d73)
- [x] BIDI_MAXDEPTH  (line 2073, body 0ca9277f91e4)
- [ ] BYTEPOS  (line 286, body f5e18d0fbc01)
- [ ] CHARPOS  (line 287, body 73ec1c170ff8)
- [x] CHAR_GLYPH_PADDING_P  (line 729, body c0865b8ac24d)
- [ ] CHAR_GLYPH_SPACE_P  (line 648, body e2dd0bbfb3cd)
- [ ] CHECK_MATRIX  (line 869, body a29646b15239)
- [ ] CLIP_TEXT_POS_FROM_MARKER  (line 328, body 002b35e3ba7f)
- [ ] CURRENT_HEADER_LINE_ACTIVE_FACE_ID  (line 1607, body fabe4139f274)
- [ ] CURRENT_HEADER_LINE_ACTIVE_FACE_ID_3  (line 1594, body 93df835e5c32)
- [ ] CURRENT_HEADER_LINE_HEIGHT  (line 1616, body b0a21d9aff69)
- [ ] CURRENT_MODE_LINE_ACTIVE_FACE_ID  (line 1566, body af19b7b09f94)
- [ ] CURRENT_MODE_LINE_ACTIVE_FACE_ID_3  (line 1553, body 5312b10a70fc)
- [ ] CURRENT_MODE_LINE_HEIGHT  (line 1575, body fba2902e5cc3)
- [ ] CURRENT_TAB_LINE_ACTIVE_FACE_ID  (line 1641, body 925fa57920f2)
- [ ] CURRENT_TAB_LINE_ACTIVE_FACE_ID_3  (line 1629, body aed5798bdffa)
- [ ] CURRENT_TAB_LINE_HEIGHT  (line 1650, body 001407fb1bad)
- [ ] DEC_TEXT_POS  (line 309, body ae74bb8bed91)
- [ ] DEFAULT_TAB_BAR_BUTTON_MARGIN  (line 3435, body 356a192b7913)
- [ ] DEFAULT_TAB_BAR_BUTTON_RELIEF  (line 3436, body 356a192b7913)
- [ ] DEFAULT_TAB_BAR_IMAGE_HEIGHT  (line 3440, body 9e6a55b6b456)
- [ ] DEFAULT_TOOL_BAR_BUTTON_MARGIN  (line 3514, body 1b6453892473)
- [ ] DEFAULT_TOOL_BAR_BUTTON_RELIEF  (line 3515, body 356a192b7913)
- [ ] DEFAULT_TOOL_BAR_IMAGE_HEIGHT  (line 3519, body 4d134bc07221)
- [ ] DEFAULT_TOOL_BAR_LABEL_SIZE  (line 3510, body fa35e192121e)
- [ ] DESIRED_HEADER_LINE_HEIGHT  (line 1666, body a8267bef2bfe)
- [ ] DESIRED_MODE_LINE_HEIGHT  (line 1661, body 841e5d4036b1)
- [ ] DESIRED_TAB_LINE_HEIGHT  (line 1671, body 11212daf96ab)
- [ ] DISPEXTERN_H_INCLUDED  (line 23, body da39a3ee5e6b)
- [ ] FACE_EXTENSIBLE_P  (line 2011, body cd0ec3c1d2ae)
- [ ] FACE_ID_BITS  (line 562, body 91032ad7bbcb)
- [ ] FACE_TTY_DEFAULT_BG_COLOR  (line 1940, body aec1a599270e)
- [ ] FACE_TTY_DEFAULT_COLOR  (line 1932, body d129ea7e5b92)
- [ ] FACE_TTY_DEFAULT_FG_COLOR  (line 1936, body ee7da968dd72)
- [ ] FACE_UNIBYTE_P  (line 1953, body f9be50a1871d)
- [ ] FONT_TOO_HIGH  (line 1712, body b1cfce490b0e)
- [x] FONT_TYPE_UNKNOWN  (line 644, body b6589fc6ab0d)
- [x] FRINGE_HEIGHT_BITS  (line 250, body fe5dbbcea5ce)
- [x] FRINGE_ID_BITS  (line 247, body 1574bddb75c7)
- [x] GCBackground  (line 83, body d6c81010612a)
- [x] GCForeground  (line 82, body 2113f7bfab26)
- [x] GLYPH_CHAR_AND_FACE_EQUAL_P  (line 685, body aa3134da4707)
- [x] GLYPH_EQUAL_P  (line 671, body cf3e48fcd372)
- [x] GLYPH_INVALID_P  (line 725, body 33ac13298a7b)
- [x] GLYPH_SLICE_EQUAL_P  (line 660, body 9c418a93c05b)
- [ ] IF_DEBUG  (line 264, body a29646b15239)
- [ ] INC_TEXT_POS  (line 296, body 6e78be0b5653)
- [ ] ITERATOR_AT_END_OF_LINE_P  (line 2954, body 5fe0db6bf4f3)
- [ ] ITERATOR_AT_END_P  (line 2948, body 41e3bb4fc207)
- [ ] IT_BYTEPOS  (line 2941, body 0f75d85b7e29)
- [ ] IT_CHARPOS  (line 2940, body bcd8f758e9a9)
- [ ] IT_STACK_SIZE  (line 2383, body ac3478d69a3c)
- [ ] IT_STRING_BYTEPOS  (line 2943, body f11b6d59f1ec)
- [ ] IT_STRING_CHARPOS  (line 2942, body d8348a36bac4)
- [x] MATRIX_BOTTOM_TEXT_ROW  (line 1231, body aabcfcfcceb6)
- [x] MATRIX_FIRST_TEXT_ROW  (line 1189, body 4e76fb21e86e)
- [ ] MATRIX_HEADER_LINE_HEIGHT  (line 1529, body 2932fb04bc39)
- [x] MATRIX_HEADER_LINE_ROW  (line 1184, body 17a50d307fc8)
- [ ] MATRIX_MODE_LINE_HEIGHT  (line 1520, body 37d33c0cddf3)
- [x] MATRIX_MODE_LINE_ROW  (line 1171, body 819b62a04044)
- [x] MATRIX_ROW  (line 1165, body c6255477f955)
- [x] MATRIX_ROW_BOTTOM_Y  (line 1290, body e6875dfb0c96)
- [x] MATRIX_ROW_CONTINUATION_LINE_P  (line 1300, body 5ec3b17b5094)
- [x] MATRIX_ROW_DISPLAYS_TEXT_P  (line 1252, body 397bffe8650d)
- [x] MATRIX_ROW_ENABLED_P  (line 1246, body 8c5f0efd6ea9)
- [x] MATRIX_ROW_ENDS_IN_MIDDLE_OF_CHAR_P  (line 1307, body 0e14ae713706)
- [x] MATRIX_ROW_ENDS_IN_OVERLAY_STRING_P  (line 1314, body d22cdc3b928b)
- [x] MATRIX_ROW_END_BYTEPOS  (line 1221, body 5b243044fb29)
- [x] MATRIX_ROW_END_CHARPOS  (line 1220, body 8c65f84f8db2)
- [x] MATRIX_ROW_EXTENDS_FACE_P  (line 1239, body a50a1ec3cdc1)
- [x] MATRIX_ROW_GLYPH_START  (line 1198, body 0dc72eb35e5a)
- [x] MATRIX_ROW_LAST_VISIBLE_P  (line 1295, body 4ae0c6fdac52)
- [x] MATRIX_ROW_OVERLAPS_PRED_P  (line 1327, body ed3f451aedf8)
- [x] MATRIX_ROW_OVERLAPS_SUCC_P  (line 1332, body 0092bc3cfeec)
- [x] MATRIX_ROW_PARTIALLY_VISIBLE_AT_BOTTOM_P  (line 1284, body cd8ba990fa02)
- [x] MATRIX_ROW_PARTIALLY_VISIBLE_AT_TOP_P  (line 1278, body f398bea426a8)
- [x] MATRIX_ROW_PARTIALLY_VISIBLE_P  (line 1269, body 7f419cc85e38)
- [x] MATRIX_ROW_STARTS_IN_MIDDLE_OF_CHAR_P  (line 1319, body ad070ce61d45)
- [x] MATRIX_ROW_START_BYTEPOS  (line 1213, body ef9b1460bb27)
- [x] MATRIX_ROW_START_CHARPOS  (line 1212, body fafca7eb1d99)
- [x] MATRIX_ROW_USED  (line 1203, body 7f85c52fa373)
- [x] MATRIX_ROW_VPOS  (line 1225, body 5ed91f0f505c)
- [ ] MATRIX_TAB_LINE_HEIGHT  (line 1538, body a74e931726e2)
- [x] MATRIX_TAB_LINE_ROW  (line 1178, body c26c1bb176c4)
- [ ] MAX_FACE_ID  (line 1986, body e89e9b1c9f28)
- [ ] MAX_FRINGE_BITMAPS  (line 2235, body 737b674ea0ea)
- [x] MR_PARTIALLY_VISIBLE  (line 1257, body 3511fa7f7707)
- [x] MR_PARTIALLY_VISIBLE_AT_BOTTOM  (line 1263, body 4cdbd4949124)
- [x] MR_PARTIALLY_VISIBLE_AT_TOP  (line 1260, body 886ac09320dc)
- [ ] NativeRectangle  (line 179, body 46f8ab7c0cff)
- [x] No_Cursor  (line 58, body 48a28489fbe0)
- [ ] OVERLAY_STRING_CHUNK_SIZE  (line 2572, body 1574bddb75c7)
- [ ] PRODUCE_GLYPHS  (line 2962, body b381e080c20c)
- [ ] SET_CHAR_GLYPH  (line 694, body 87cdee165f02)
- [ ] SET_CHAR_GLYPH_FROM_GLYPH  (line 707, body 93df533422c7)
- [x] SET_GLYPH  (line 395, body 4660fe499b7f)
- [x] SET_GLYPH_CHAR  (line 393, body df53ce587fba)
- [x] SET_GLYPH_FACE  (line 394, body bda12f7fd668)
- [ ] SET_GLYPH_FROM_CHAR_GLYPH  (line 715, body b2e7c6eecdd3)
- [ ] SET_GLYPH_FROM_GLYPH_CODE  (line 414, body 6e5736c7b386)
- [ ] SET_MARKER_FROM_TEXT_POS  (line 336, body 2a85f22416c9)
- [x] SET_MATRIX_ROW_ENABLED_P  (line 1243, body c92cc2f3ccd9)
- [ ] SET_TEXT_POS  (line 291, body 191b91beaae4)
- [ ] SET_TEXT_POS_FROM_MARKER  (line 322, body 4aac43f6e52f)
- [ ] TEXT_POS_EQUAL_P  (line 342, body 4611f71b8e7c)
- [ ] TRY_WINDOW_CHECK_MARGINS  (line 3699, body ee799cd75778)
- [ ] TRY_WINDOW_IGNORE_FONTS_CHANGE  (line 3700, body 8ef58e36184c)
- [ ] VCENTER_BASELINE_OFFSET  (line 1702, body f9470532af8f)

## Types and their fields — 19
- [ ] bidi_it: ptrdiff_t bytepos, ptrdiff_t charpos, int ch, ptrdiff_t nchars, ptrdiff_t ch_len, bidi_type_t type, bidi_type_t type_after_wn, bidi_type_t orig_type, signed char resolved_level, signed char isolate_level, ptrdiff_t invalid_levels, ptrdiff_t invalid_isolates, struct bidi_saved_info prev, struct bidi_saved_info last_strong, struct bidi_saved_info next_for_neutral, struct bidi_saved_info prev_for_neutral, struct bidi_saved_info next_for_ws, ptrdiff_t bracket_pairing_pos, bidi_type_t bracket_enclosed_type, ptrdiff_t next_en_pos, bidi_type_t next_en_type, bidi_dir_t sos, int scan_dir, ptrdiff_t disp_pos, int disp_prop, int stack_idx, struct bidi_stack level_stack, struct bidi_string_data string, struct window* w, bidi_dir_t paragraph_dir, ptrdiff_t separator_limit, bool_bf first_elt : 1, bool_bf new_paragraph : 1, bool_bf frame_window_p : 1
- [x] bidi_saved_info: ptrdiff_t charpos, bidi_type_t type, bidi_type_t orig_type
- [x] bidi_stack: ptrdiff_t next_for_neutral_pos, unsigned next_for_neutral_type : 3, unsigned last_strong_type : 3, unsigned prev_for_neutral_type : 3, unsigned char level, unsigned char flags
- [x] bidi_string_data: Lisp_Object lstring, const unsigned char* s, ptrdiff_t schars, ptrdiff_t bufpos, bool_bf from_disp_str : 1, bool_bf unibyte : 1
- [ ] composition_it: ptrdiff_t stop_pos, ptrdiff_t id, int ch, EMACS_INT rule_idx, ptrdiff_t lookback, int nglyphs, bool reversed_p, struct it* parent_it, ptrdiff_t charpos, int nchars, int nbytes, int from, int to, int width
- [x] display_pos: struct text_pos pos, ptrdiff_t overlay_string_index, struct text_pos string_pos, int dpvec_index
- [ ] draw_fringe_bitmap_params: int which, unsigned short* bits, int wd, int h, int dh, int x, int y, int bx, int nx, int by, int ny, bool_bf cursor_p : 1, bool_bf overlay_p : 1, struct face* face
- [ ] face: Lisp_Object lface, int id, unsigned long foreground, unsigned long background, unsigned long underline_color, unsigned long overline_color, unsigned long strike_through_color, unsigned long box_color, struct font* font, int fontset, int box_vertical_line_width, int box_horizontal_line_width, int underline_pixels_above_descent_line, ENUM_BF (face_box_type) box : 2, ENUM_BF (face_underline_type) underline : 3, bool_bf use_box_color_for_shadows_p : 1, bool_bf overline_p : 1, bool_bf strike_through_p : 1, bool_bf foreground_defaulted_p : 1, bool_bf background_defaulted_p : 1, bool_bf underline_defaulted_p : 1, bool_bf overline_color_defaulted_p : 1, bool_bf strike_through_color_defaulted_p : 1, bool_bf box_color_defaulted_p : 1, bool_bf underline_at_descent_line_p : 1, bool_bf tty_bold_p : 1, bool_bf tty_italic_p : 1, bool_bf tty_reverse_p : 1, bool_bf tty_strike_through_p : 1, bool_bf tty_overline_p : 1, bool_bf colors_copied_bitwise_p : 1, bool_bf overstrike : 1, uintptr_t hash, struct face* next, struct face* prev, struct face* ascii_face
- [ ] face_cache: struct face** buckets, struct frame* f, struct face** faces_by_id, ptrdiff_t size, int used, bool_bf menu_face_changed_p : 1
- [x] glyph: ptrdiff_t charpos, Lisp_Object object, struct frame* frame, short pixel_width, short ascent, short descent, short voffset, unsigned type : 3, bool_bf multibyte_p : 1, bool_bf left_box_line_p : 1, bool_bf right_box_line_p : 1, bool_bf overlaps_vertically_p : 1, bool_bf padding_p : 1, bool_bf glyph_not_available_p : 1, bool_bf avoid_cursor_p : 1, unsigned resolved_level : 7, unsigned bidi_type : 3, unsigned face_id : FACE_ID_BITS, unsigned font_type : 3, struct glyph_slice img, short upper_xoff, short upper_yoff, short lower_xoff, short lower_yoff, <aggregate> glyphless, <aggregate> slice, unsigned ch, bool_bf automatic : 1, unsigned id : 31, <aggregate> cmp, int img_id, unsigned height : 16, unsigned ascent : 16, <aggregate> stretch, unsigned method : 2, bool_bf for_no_font : 1, unsigned len : 4, unsigned ch : 25, <aggregate> glyphless, unsigned val, <aggregate> u
- [x] glyph_matrix: struct glyph_pool* pool, struct glyph_row* rows, ptrdiff_t rows_allocated, int nrows, int matrix_x, int matrix_y, int matrix_w, int matrix_h, int window_pixel_left, int window_pixel_top, int window_height, int window_width, int window_vscroll, int left_margin_glyphs, int right_margin_glyphs, bool_bf no_scrolling_p : 1, bool_bf tab_line_p : 1, bool_bf header_line_p : 1, struct buffer* buffer, ptrdiff_t begv, ptrdiff_t zv
- [x] glyph_pool: struct glyph* glyphs, ptrdiff_t nglyphs, int nrows, int ncolumns
- [x] glyph_row: struct glyph* glyphs, short used, unsigned hash, int x, int y, int pixel_width, int ascent, int height, int phys_ascent, int phys_height, int visible_height, int extra_line_spacing, int extra_line_spacing_above, struct display_pos start, struct display_pos end, struct text_pos minpos, struct text_pos maxpos, int overlay_arrow_bitmap, unsigned left_user_fringe_bitmap : FRINGE_ID_BITS, unsigned right_user_fringe_bitmap : FRINGE_ID_BITS, unsigned left_fringe_bitmap : FRINGE_ID_BITS, unsigned right_fringe_bitmap : FRINGE_ID_BITS, unsigned left_user_fringe_face_id : FACE_ID_BITS, unsigned right_user_fringe_face_id : FACE_ID_BITS, unsigned left_fringe_face_id : FACE_ID_BITS, unsigned right_fringe_face_id : FACE_ID_BITS, signed left_fringe_offset : FRINGE_HEIGHT_BITS, signed right_fringe_offset : FRINGE_HEIGHT_BITS, bool_bf fringe_bitmap_periodic_p : 1, bool_bf redraw_fringe_bitmaps_p : 1, bool_bf enabled_p : 1, bool_bf truncated_on_left_p : 1, bool_bf truncated_on_right_p : 1, bool_bf continued_p : 1, bool_bf displays_text_p : 1, bool_bf ends_at_zv_p : 1, bool_bf fill_line_p : 1, bool_bf indicate_empty_line_p : 1, bool_bf contains_overlapping_glyphs_p : 1, bool_bf full_width_p : 1, bool_bf mode_line_p : 1, bool_bf tab_line_p : 1, bool_bf overlapped_p : 1, bool_bf ends_in_middle_of_char_p : 1, bool_bf starts_in_middle_of_char_p : 1, bool_bf overlapping_p : 1, bool_bf mouse_face_p : 1, bool_bf ends_in_newline_from_string_p : 1, bool_bf exact_window_width_line_p : 1, bool_bf cursor_in_fringe_p : 1, bool_bf ends_in_ellipsis_p : 1, bool_bf indicate_bob_p : 1, bool_bf indicate_top_line_p : 1, bool_bf indicate_eob_p : 1, bool_bf indicate_bottom_line_p : 1, bool_bf reversed_p : 1, bool_bf stipple_p : 1, int continuation_lines_width
- [x] glyph_slice: unsigned x : 16, unsigned y : 16, unsigned width : 16, unsigned height : 16
- [ ] it: Lisp_Object window, struct window* w, struct frame* f, enum it_method method, ptrdiff_t stop_charpos, ptrdiff_t prev_stop, ptrdiff_t base_level_stop, ptrdiff_t end_charpos, ptrdiff_t medium_narrowing_begv, ptrdiff_t medium_narrowing_zv, ptrdiff_t large_narrowing_begv, ptrdiff_t large_narrowing_zv, const unsigned char* s, ptrdiff_t string_nchars, bool_bf multibyte_p : 1, bool_bf tab_line_p : 1, bool_bf header_line_p : 1, bool_bf string_from_display_prop_p : 1, bool_bf string_from_prefix_prop_p : 1, bool_bf from_disp_prop_p : 1, bool_bf ellipsis_p : 1, bool_bf avoid_cursor_p : 1, struct Lisp_Char_Table* dp, Lisp_Object* dpvec, Lisp_Object* dpend, int dpvec_char_len, int dpvec_face_id, int saved_face_id, Lisp_Object ctl_chars, struct display_pos start, struct display_pos current, ptrdiff_t n_overlay_strings, ptrdiff_t overlay_strings_charpos, Lisp_Object overlay_strings, Lisp_Object string_overlays, Lisp_Object string, Lisp_Object from_overlay, Lisp_Object string, int string_nchars, ptrdiff_t end_charpos, ptrdiff_t stop_charpos, ptrdiff_t prev_stop, ptrdiff_t base_level_stop, struct composition_it cmp_it, int face_id, Lisp_Object object, struct it_slice slice, ptrdiff_t image_id, <aggregate> image, Lisp_Object object, <aggregate> stretch, Lisp_Object object, <aggregate> xwidget, <aggregate> u, struct text_pos position, struct display_pos current, Lisp_Object from_overlay, enum glyph_row_area area, enum it_method method, bidi_dir_t paragraph_embedding, bool_bf multibyte_p : 1, bool_bf string_from_display_prop_p : 1, bool_bf string_from_prefix_prop_p : 1, bool_bf display_ellipsis_p : 1, bool_bf avoid_cursor_p : 1, bool_bf bidi_p : 1, bool_bf from_disp_prop_p : 1, enum line_wrap_method line_wrap, short voffset, Lisp_Object space_width, Lisp_Object font_height, <aggregate> stack, int sp, ptrdiff_t selective, enum display_element_type what, int face_id, bool_bf selective_display_ellipsis_p : 1, bool_bf ctl_arrow_p : 1, bool_bf face_box_p : 1, bool_bf start_of_box_run_p : 1, bool_bf end_of_box_run_p : 1, bool_bf overlay_strings_at_end_processed_p : 1, bool_bf ignore_overlay_strings_at_pos_p : 1, bool_bf glyph_not_available_p : 1, bool_bf starts_in_middle_of_char_p : 1, bool_bf face_before_selective_p : 1, bool_bf constrain_row_ascent_descent_p : 1, bool_bf line_number_produced_p : 1, bool_bf align_visually_p : 1, enum line_wrap_method line_wrap, int base_face_id, int c, int len, struct composition_it cmp_it, int char_to_display, enum glyphless_display_method glyphless_method, ptrdiff_t image_id, struct xwidget* xwidget, struct it_slice slice, Lisp_Object space_width, short voffset, short tab_width, Lisp_Object font_height, Lisp_Object object, struct text_pos position, short truncation_pixel_width, short continuation_pixel_width, int first_visible_x, int last_visible_x, int last_visible_y, int extra_line_spacing, int extra_line_spacing_above, int max_extra_line_spacing, int override_ascent, int override_descent, int override_boff, struct glyph_row* glyph_row, enum glyph_row_area area, int nglyphs, int pixel_width, int ascent, int descent, int max_ascent, int max_descent, int phys_ascent, int phys_descent, int max_phys_ascent, int max_phys_descent, int current_x, int wrap_prefix_width, int continuation_lines_width, struct text_pos eol_pos, int current_y, int first_vpos, int vpos, int hpos, ptrdiff_t lnum, ptrdiff_t lnum_bytepos, int lnum_width, int lnum_pixel_width, ptrdiff_t pt_lnum, int stretch_adjust, unsigned left_user_fringe_bitmap : FRINGE_ID_BITS, unsigned right_user_fringe_bitmap : FRINGE_ID_BITS, unsigned left_user_fringe_face_id : FACE_ID_BITS, unsigned right_user_fringe_face_id : FACE_ID_BITS, bool_bf bidi_p : 1, struct bidi_it bidi_it, bidi_dir_t paragraph_embedding, Lisp_Object min_width_property, int min_width_start
- [x] it_slice: Lisp_Object x, Lisp_Object y, Lisp_Object width, Lisp_Object height
- [ ] redisplay_interface: frame_parm_handler* frame_parm_handlers, void (*) produce_glyphs, void (*) write_glyphs, void (*) insert_glyphs, void (*) clear_end_of_line, void (*) scroll_run_hook, void (*) after_update_window_line_hook, void (*) update_window_begin_hook, void (*) update_window_end_hook, void (*) flush_display, void (*) clear_window_mouse_face, void (*) get_glyph_overhangs, void (*) fix_overlapping_area
- [x] run: int desired_y, int current_y, int desired_vpos, int current_vpos, int height, int nrows
- [x] text_pos: ptrdiff_t charpos, ptrdiff_t bytepos

## File-scope objects — 16
- [x] extern struct glyph space_glyph  (line 1338)
- [ ] extern bool face_change  (line 2068)
- [ ] extern bool redisplaying_p  (line 3608)
- [ ] extern bool display_working_on_window_p  (line 3610)
- [ ] extern int dont_resize_frames  (line 3611)
- [ ] extern bool help_echo_showing_p  (line 3613)
- [ ] extern Lisp_Object help_echo_string  (line 3614)
- [ ] extern Lisp_Object help_echo_window  (line 3614)
- [ ] extern Lisp_Object help_echo_object  (line 3615)
- [ ] extern Lisp_Object previous_help_echo_string  (line 3615)
- [ ] extern ptrdiff_t help_echo_pos  (line 3616)
- [ ] extern int last_tab_bar_item  (line 3617)
- [ ] extern int last_tool_bar_item  (line 3618)
- [ ] extern int max_used_fringe_bitmap  (line 3710)
- [ ] extern char unspecified_fg  (line 3834)
- [ ] extern char unspecified_bg  (line 3834)
# ----------------------------------------------------------------------
## Notes

### What this mirror carries so far (WIP — PORTING.md P-66)

The platform-independent part of the display interface the frozen configuration leaves in
place: the non-X `Emacs_Color`/`Emacs_Rectangle`/`Emacs_GC`, `Emacs_Cursor`, `No_Cursor` and
the two GC mask macros; the `text_cursor_kinds`/`window_part`/`glyph_type`/`glyph_row_area`/
`draw_glyphs_face`/`lface_attribute_index`/`face_box_type`/`face_underline_type`/`face_id`
enumerations (each as a C type alias plus constants with C's names and values); the `GLYPH`
datatype with `GLYPH_CHAR`/`GLYPH_FACE`, `SET_GLYPH_CHAR`/`SET_GLYPH_FACE`/`SET_GLYPH` and
`GLYPH_CHAR_VALID_P`; `glyph_slice` and the whole glyph record (`glyph`, `glyph_pool`,
`glyph_matrix`, `glyph_row`, their nested unions, the `MATRIX_*`/`MR_*` accessor families and
the `GLYPH_SLICE_EQUAL_P`/`GLYPH_EQUAL_P`/`GLYPH_CHAR_AND_FACE_EQUAL_P` comparisons) with the
bit members named through `glyph_bits!`/`glyph_row_bits!`/`glyph_matrix_bits!`; the text
positions (`text_pos`, `display_pos`, `it_slice`, `run`) and the bidi state (`bidi_saved_info`,
`bidi_stack`, `bidi_string_data`, the `bidi_type_t`/`bidi_bracket_type_t`/`bidi_dir_t`
enumerations); `Mouse_HLInfo`, `CHECK_MATRIX`, and the `space_glyph`/`bitch_at_user` externs.

Not mirrored yet (the unchecked boxes): `face`, `face_cache`, `image`, `image_cache`, the
`struct it`/`composition_it`/`bidi_it` iterators, the fringe and X-widget sections, the
`CURRENT_*`/`DESIRED_*` face and line-height macros, and everything gated on
`HAVE_WINDOW_SYSTEM`/`HAVE_X_WINDOWS`/… .  A struct lands when the unit that needs its
*layout* does (`xtask layout-asserts` must be able to assert it: a type joins
`tools/gen-layout-fields.py`'s `STRUCTS` together with its mirror, since a key whose type is
missing counts as pending and the ratchet only shrinks pending).

### Deviations (recorded, not missing work)

- The include guard has no Rust counterpart: `pub const DISPEXTERN_H_INCLUDED: bool = true`.
- C spells `Emacs_Color`, `Emacs_Rectangle`, `Emacs_GC` and `GLYPH` as *anonymous* struct
  typedefs; a Rust struct needs a name, so the port names it after the typedef (the layout is
  what the ABI needs).  The generated layout keys cannot cover them yet for the same reason —
  `tools/gen-layout-fields.py` looks for `struct NAME`, and these have no tag, while the C
  probe's `offsetof (Emacs_Color, red)` would work (PORTING.md P-67).
- The bitfield structs (`glyph_slice` here, `glyph`/`glyph_row`/`face` when they land) keep
  C's storage unit with named masks, and the unit's width and the measured byte pattern are
  recorded in the mirror (PORTING.md 1.3/P-35): `glyph_slice` is one `u64` with `x` in bits
  0-15, `y` in 16-31, `width` in 32-47, `height` in 48-63, `sizeof` 8, `alignof` 4.
- `enum` types are `pub type NAME = c_int;` plus `pub const` members, which is how the C code
  uses them (as `int` comparisons and as declared variable types), matching `lisp.rs`'s
  `MANY`/`UNEVALLED` precedent.

### Round 55 — the display core: `glyph`, `glyph_pool`, `glyph_matrix`, `glyph_row`

- The four types are mirrored with the frozen configuration's measured layout, and `xtask
  layout-check` asserts all 58 of their keys (746 assertions from the baseline, up from 688;
  the 6 pending keys are still `handler`'s): `glyph` 56/8, `glyph_pool` 24/8, `glyph_matrix`
  104/8, `glyph_row` 256/8, with every non-bitfield member at its measured offset.
- The *bitfield* members have no Rust field (PORTING.md 1.3), so each is stored in its
  measured allocation unit — `glyph.bits: u64`, `glyph_row.bits: [u32; 6]` plus
  `right_fringe_offset: i8` and `bools: [u8; 4]`, `glyph_matrix.bits: u8` — and *named at each
  use* by an accessor macro whose first argument is the C member: `glyph_bits! (padding_p, g)`
  reads `g->padding_p`, `glyph_bits! (padding_p, g, v)` assigns it, and `glyph_row_bits!` /
  `glyph_matrix_bits!` / `glyph_slice_bits!` are the same shape.  **This construct has no
  PORTING.md entry yet**: the files landed so far pack a bitfield into `bits` and register a
  `token-aliases.tsv` family for the member's name, which cannot work here because one C name
  (`tab_line_p`, `padding_p`) is a member of several mirrored structs at once.  Naming the
  member in the call keeps the C name in the ported text (what `--check`'s macro-body token
  comparison asks for) and stays type-dispatched.  §11 needs the entry before a second file
  copies it.
- Bit positions are invisible to `offsetof`, so they were proved the way the earlier mirrors
  were: the C compiler's own `-Xclang -fdump-record-layouts` gives every member's `byte:bits`,
  and a probe that sets every member of `glyph`, `glyph_row` and `glyph_matrix` to a value it
  can be told apart by prints byte-identical images from C and from the mirror (`glyph` bytes
  32-39 `dd fe 0b 00 de bc 3a 00`; the row's fringe run at 220-248
  `34 12 78 56 bc 9a f0 de 11 11 01 00 22 22 02 00 33 33 03 00 44 44 d4 0f fb 75 19 91 08`).
- The two anonymous unions of `glyph` are named after their member (`glyph_slice_union`,
  `glyph_u`) and their own anonymous members `<union>_<member>` (`glyph_slice_cmp`,
  `glyph_slice_glyphless`, `glyph_u_cmp`, `glyph_u_stretch`, `glyph_u_glyphless`).  The C
  type `struct glyph_slice` keeps that name, which is why the union member `slice` carries the
  `_union` suffix (P-67).
- Dead `#ifdef` members stay behind their features, as elsewhere in the mirror (P-51):
  `glyph_u.xwidget` (`feature = "xwidgets"`), `glyph_matrix.method` (`feature = "glyph_debug"`)
  and `glyph_row.clip` (`feature = "window_system"`); none of them is part of the frozen
  layout the assertions lock.
- `glyph.frame` and `glyph_matrix.buffer` degrade to `*mut c_void` (P-72 rule ③: this mirror
  includes neither frame.rs nor buffer.rs) with the `/// C:` note carrying C's prototype.
- `space_glyph` is `unsafe extern "C" { pub static mut space_glyph: glyph; }` (C's
  `extern struct glyph space_glyph`), and `bitch_at_user` the extern function dispextern.h
  3900 declares (cmds.c's user).
- `SET_CHAR_GLYPH`, `SET_CHAR_GLYPH_FROM_GLYPH` and `SET_GLYPH_FROM_CHAR_GLYPH` (697-724) stay
  unmirrored: each C body ends in `while (false)`, the shape the port records in
  `docs/ports/macro-body-notes.tsv` (`CHECK_CODING_SYSTEM`/`SAFE_ALLOCA_LISP`); they land with
  that row.  `CHAR_GLYPH_SPACE_P` (648) likewise waits for `SPACEGLYPH`, which disptab.h
  declares (disptab.rs), rather than defining a second copy of it here.
- `MATRIX_BOTTOM_TEXT_ROW` and the three `MR_PARTIALLY_VISIBLE_AT_*` macros expand window.h's
  `WINDOW_TAB_LINE_HEIGHT`/`WINDOW_HEADER_LINE_HEIGHT`/`WINDOW_BOX_HEIGHT_NO_MODE_LINE` and
  call `window_wants_mode_line`; the mirror keeps C's text (P-72 rule ③), so they expand in
  exactly the units C's do — the ones that include window.rs.
- `GLYPH_CHAR_VALID_P` spells out the `int`→`intmax_t` widening C performs implicitly, because
  character.h's `CHAR_VALID_P` takes an `intmax_t`.

### Call-relationship differences (D7)

- None so far; the file follows C's order for what it carries.
