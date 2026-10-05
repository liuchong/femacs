# emacs/src/dispextern.h (4025 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 9
- [x] xstrcasecmp  (line 108)
- [x] GLYPH_CODE_CHAR  (line 401)
- [x] GLYPH_CODE_FACE  (line 409)
- [x] face_tty_specified_color  (line 1946)
- [x] FACE_SUITABLE_FOR_ASCII_CHAR_P  (line 2016)
- [x] FACE_FOR_CHAR  (line 2031)
- [x] GLYPH_CHAR_VALID_P  (line 2043)
- [x] GLYPH_CODE_P  (line 2053)
- [x] reset_mouse_highlight  (line 3027)

## Macros defined — 108
- [x] BIDI_AT_BASE_LEVEL  (line 2207, body 971025605d73)
- [x] BIDI_MAXDEPTH  (line 2073, body 0ca9277f91e4)
- [x] BYTEPOS  (line 286, body f5e18d0fbc01)
- [x] CHARPOS  (line 287, body 73ec1c170ff8)
- [x] CHAR_GLYPH_PADDING_P  (line 729, body c0865b8ac24d)
- [x] CHAR_GLYPH_SPACE_P  (line 648, body e2dd0bbfb3cd)
- [x] CHECK_MATRIX  (line 869, body a29646b15239)
- [x] CLIP_TEXT_POS_FROM_MARKER  (line 328, body 002b35e3ba7f)
- [x] CURRENT_HEADER_LINE_ACTIVE_FACE_ID  (line 1607, body fabe4139f274)
- [x] CURRENT_HEADER_LINE_ACTIVE_FACE_ID_3  (line 1594, body 93df835e5c32)
- [x] CURRENT_HEADER_LINE_HEIGHT  (line 1616, body b0a21d9aff69)
- [x] CURRENT_MODE_LINE_ACTIVE_FACE_ID  (line 1566, body af19b7b09f94)
- [x] CURRENT_MODE_LINE_ACTIVE_FACE_ID_3  (line 1553, body 5312b10a70fc)
- [x] CURRENT_MODE_LINE_HEIGHT  (line 1575, body fba2902e5cc3)
- [x] CURRENT_TAB_LINE_ACTIVE_FACE_ID  (line 1641, body 925fa57920f2)
- [x] CURRENT_TAB_LINE_ACTIVE_FACE_ID_3  (line 1629, body aed5798bdffa)
- [x] CURRENT_TAB_LINE_HEIGHT  (line 1650, body 001407fb1bad)
- [x] DEC_TEXT_POS  (line 309, body ae74bb8bed91)
- [x] DEFAULT_TAB_BAR_BUTTON_MARGIN  (line 3435, body 356a192b7913)
- [x] DEFAULT_TAB_BAR_BUTTON_RELIEF  (line 3436, body 356a192b7913)
- [x] DEFAULT_TAB_BAR_IMAGE_HEIGHT  (line 3440, body 9e6a55b6b456)
- [x] DEFAULT_TOOL_BAR_BUTTON_MARGIN  (line 3514, body 1b6453892473)
- [x] DEFAULT_TOOL_BAR_BUTTON_RELIEF  (line 3515, body 356a192b7913)
- [x] DEFAULT_TOOL_BAR_IMAGE_HEIGHT  (line 3519, body 4d134bc07221)
- [x] DEFAULT_TOOL_BAR_LABEL_SIZE  (line 3510, body fa35e192121e)
- [x] DESIRED_HEADER_LINE_HEIGHT  (line 1666, body a8267bef2bfe)
- [x] DESIRED_MODE_LINE_HEIGHT  (line 1661, body 841e5d4036b1)
- [x] DESIRED_TAB_LINE_HEIGHT  (line 1671, body 11212daf96ab)
- [x] DISPEXTERN_H_INCLUDED  (line 23, body da39a3ee5e6b)
- [x] FACE_EXTENSIBLE_P  (line 2011, body cd0ec3c1d2ae)
- [x] FACE_ID_BITS  (line 562, body 91032ad7bbcb)
- [x] FACE_TTY_DEFAULT_BG_COLOR  (line 1940, body aec1a599270e)
- [x] FACE_TTY_DEFAULT_COLOR  (line 1932, body d129ea7e5b92)
- [x] FACE_TTY_DEFAULT_FG_COLOR  (line 1936, body ee7da968dd72)
- [x] FACE_UNIBYTE_P  (line 1953, body f9be50a1871d)
- [x] FONT_TOO_HIGH  (line 1712, body b1cfce490b0e)
- [x] FONT_TYPE_UNKNOWN  (line 644, body b6589fc6ab0d)
- [x] FRINGE_HEIGHT_BITS  (line 250, body fe5dbbcea5ce)
- [x] FRINGE_ID_BITS  (line 247, body 1574bddb75c7)
- [x] GCBackground  (line 83, body d6c81010612a)
- [x] GCForeground  (line 82, body 2113f7bfab26)
- [x] GLYPH_CHAR_AND_FACE_EQUAL_P  (line 685, body aa3134da4707)
- [x] GLYPH_EQUAL_P  (line 671, body cf3e48fcd372)
- [x] GLYPH_INVALID_P  (line 725, body 33ac13298a7b)
- [x] GLYPH_SLICE_EQUAL_P  (line 660, body 9c418a93c05b)
- [x] IF_DEBUG  (line 264, body a29646b15239)
- [x] INC_TEXT_POS  (line 296, body 6e78be0b5653)
- [x] ITERATOR_AT_END_OF_LINE_P  (line 2954, body 5fe0db6bf4f3)
- [x] ITERATOR_AT_END_P  (line 2948, body 41e3bb4fc207)
- [x] IT_BYTEPOS  (line 2941, body 0f75d85b7e29)
- [x] IT_CHARPOS  (line 2940, body bcd8f758e9a9)
- [x] IT_STACK_SIZE  (line 2383, body ac3478d69a3c)
- [x] IT_STRING_BYTEPOS  (line 2943, body f11b6d59f1ec)
- [x] IT_STRING_CHARPOS  (line 2942, body d8348a36bac4)
- [x] MATRIX_BOTTOM_TEXT_ROW  (line 1231, body aabcfcfcceb6)
- [x] MATRIX_FIRST_TEXT_ROW  (line 1189, body 4e76fb21e86e)
- [x] MATRIX_HEADER_LINE_HEIGHT  (line 1529, body 2932fb04bc39)
- [x] MATRIX_HEADER_LINE_ROW  (line 1184, body 17a50d307fc8)
- [x] MATRIX_MODE_LINE_HEIGHT  (line 1520, body 37d33c0cddf3)
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
- [x] MATRIX_TAB_LINE_HEIGHT  (line 1538, body a74e931726e2)
- [x] MATRIX_TAB_LINE_ROW  (line 1178, body c26c1bb176c4)
- [x] MAX_FACE_ID  (line 1986, body e89e9b1c9f28)
- [x] MAX_FRINGE_BITMAPS  (line 2235, body 737b674ea0ea)
- [x] MR_PARTIALLY_VISIBLE  (line 1257, body 3511fa7f7707)
- [x] MR_PARTIALLY_VISIBLE_AT_BOTTOM  (line 1263, body 4cdbd4949124)
- [x] MR_PARTIALLY_VISIBLE_AT_TOP  (line 1260, body 886ac09320dc)
- [ ] NativeRectangle  (line 179, body 46f8ab7c0cff)
- [x] No_Cursor  (line 58, body 48a28489fbe0)
- [x] OVERLAY_STRING_CHUNK_SIZE  (line 2572, body 1574bddb75c7)
- [x] PRODUCE_GLYPHS  (line 2962, body b381e080c20c)
- [x] SET_CHAR_GLYPH  (line 694, body 87cdee165f02)
- [x] SET_CHAR_GLYPH_FROM_GLYPH  (line 707, body 93df533422c7)
- [x] SET_GLYPH  (line 395, body 4660fe499b7f)
- [x] SET_GLYPH_CHAR  (line 393, body df53ce587fba)
- [x] SET_GLYPH_FACE  (line 394, body bda12f7fd668)
- [x] SET_GLYPH_FROM_CHAR_GLYPH  (line 715, body b2e7c6eecdd3)
- [x] SET_GLYPH_FROM_GLYPH_CODE  (line 414, body 6e5736c7b386)
- [x] SET_MARKER_FROM_TEXT_POS  (line 336, body 2a85f22416c9)
- [x] SET_MATRIX_ROW_ENABLED_P  (line 1243, body c92cc2f3ccd9)
- [x] SET_TEXT_POS  (line 291, body 191b91beaae4)
- [x] SET_TEXT_POS_FROM_MARKER  (line 322, body 4aac43f6e52f)
- [x] TEXT_POS_EQUAL_P  (line 342, body 4611f71b8e7c)
- [x] TRY_WINDOW_CHECK_MARGINS  (line 3699, body ee799cd75778)
- [x] TRY_WINDOW_IGNORE_FONTS_CHANGE  (line 3700, body 8ef58e36184c)
- [x] VCENTER_BASELINE_OFFSET  (line 1702, body f9470532af8f)

## Types and their fields — 19
- [x] bidi_it: ptrdiff_t bytepos, ptrdiff_t charpos, int ch, ptrdiff_t nchars, ptrdiff_t ch_len, bidi_type_t type, bidi_type_t type_after_wn, bidi_type_t orig_type, signed char resolved_level, signed char isolate_level, ptrdiff_t invalid_levels, ptrdiff_t invalid_isolates, struct bidi_saved_info prev, struct bidi_saved_info last_strong, struct bidi_saved_info next_for_neutral, struct bidi_saved_info prev_for_neutral, struct bidi_saved_info next_for_ws, ptrdiff_t bracket_pairing_pos, bidi_type_t bracket_enclosed_type, ptrdiff_t next_en_pos, bidi_type_t next_en_type, bidi_dir_t sos, int scan_dir, ptrdiff_t disp_pos, int disp_prop, int stack_idx, struct bidi_stack level_stack, struct bidi_string_data string, struct window* w, bidi_dir_t paragraph_dir, ptrdiff_t separator_limit, bool_bf first_elt : 1, bool_bf new_paragraph : 1, bool_bf frame_window_p : 1
- [x] bidi_saved_info: ptrdiff_t charpos, bidi_type_t type, bidi_type_t orig_type
- [x] bidi_stack: ptrdiff_t next_for_neutral_pos, unsigned next_for_neutral_type : 3, unsigned last_strong_type : 3, unsigned prev_for_neutral_type : 3, unsigned char level, unsigned char flags
- [x] bidi_string_data: Lisp_Object lstring, const unsigned char* s, ptrdiff_t schars, ptrdiff_t bufpos, bool_bf from_disp_str : 1, bool_bf unibyte : 1
- [x] composition_it: ptrdiff_t stop_pos, ptrdiff_t id, int ch, EMACS_INT rule_idx, ptrdiff_t lookback, int nglyphs, bool reversed_p, struct it* parent_it, ptrdiff_t charpos, int nchars, int nbytes, int from, int to, int width
- [x] display_pos: struct text_pos pos, ptrdiff_t overlay_string_index, struct text_pos string_pos, int dpvec_index
- [x] draw_fringe_bitmap_params: int which, unsigned short* bits, int wd, int h, int dh, int x, int y, int bx, int nx, int by, int ny, bool_bf cursor_p : 1, bool_bf overlay_p : 1, struct face* face
- [x] face: Lisp_Object lface, int id, unsigned long foreground, unsigned long background, unsigned long underline_color, unsigned long overline_color, unsigned long strike_through_color, unsigned long box_color, struct font* font, int fontset, int box_vertical_line_width, int box_horizontal_line_width, int underline_pixels_above_descent_line, ENUM_BF (face_box_type) box : 2, ENUM_BF (face_underline_type) underline : 3, bool_bf use_box_color_for_shadows_p : 1, bool_bf overline_p : 1, bool_bf strike_through_p : 1, bool_bf foreground_defaulted_p : 1, bool_bf background_defaulted_p : 1, bool_bf underline_defaulted_p : 1, bool_bf overline_color_defaulted_p : 1, bool_bf strike_through_color_defaulted_p : 1, bool_bf box_color_defaulted_p : 1, bool_bf underline_at_descent_line_p : 1, bool_bf tty_bold_p : 1, bool_bf tty_italic_p : 1, bool_bf tty_reverse_p : 1, bool_bf tty_strike_through_p : 1, bool_bf tty_overline_p : 1, bool_bf colors_copied_bitwise_p : 1, bool_bf overstrike : 1, uintptr_t hash, struct face* next, struct face* prev, struct face* ascii_face
- [x] face_cache: struct face** buckets, struct frame* f, struct face** faces_by_id, ptrdiff_t size, int used, bool_bf menu_face_changed_p : 1
- [x] glyph: ptrdiff_t charpos, Lisp_Object object, struct frame* frame, short pixel_width, short ascent, short descent, short voffset, unsigned type : 3, bool_bf multibyte_p : 1, bool_bf left_box_line_p : 1, bool_bf right_box_line_p : 1, bool_bf overlaps_vertically_p : 1, bool_bf padding_p : 1, bool_bf glyph_not_available_p : 1, bool_bf avoid_cursor_p : 1, unsigned resolved_level : 7, unsigned bidi_type : 3, unsigned face_id : FACE_ID_BITS, unsigned font_type : 3, struct glyph_slice img, short upper_xoff, short upper_yoff, short lower_xoff, short lower_yoff, <aggregate> glyphless, <aggregate> slice, unsigned ch, bool_bf automatic : 1, unsigned id : 31, <aggregate> cmp, int img_id, unsigned height : 16, unsigned ascent : 16, <aggregate> stretch, unsigned method : 2, bool_bf for_no_font : 1, unsigned len : 4, unsigned ch : 25, <aggregate> glyphless, unsigned val, <aggregate> u
- [x] glyph_matrix: struct glyph_pool* pool, struct glyph_row* rows, ptrdiff_t rows_allocated, int nrows, int matrix_x, int matrix_y, int matrix_w, int matrix_h, int window_pixel_left, int window_pixel_top, int window_height, int window_width, int window_vscroll, int left_margin_glyphs, int right_margin_glyphs, bool_bf no_scrolling_p : 1, bool_bf tab_line_p : 1, bool_bf header_line_p : 1, struct buffer* buffer, ptrdiff_t begv, ptrdiff_t zv
- [x] glyph_pool: struct glyph* glyphs, ptrdiff_t nglyphs, int nrows, int ncolumns
- [x] glyph_row: struct glyph* glyphs, short used, unsigned hash, int x, int y, int pixel_width, int ascent, int height, int phys_ascent, int phys_height, int visible_height, int extra_line_spacing, int extra_line_spacing_above, struct display_pos start, struct display_pos end, struct text_pos minpos, struct text_pos maxpos, int overlay_arrow_bitmap, unsigned left_user_fringe_bitmap : FRINGE_ID_BITS, unsigned right_user_fringe_bitmap : FRINGE_ID_BITS, unsigned left_fringe_bitmap : FRINGE_ID_BITS, unsigned right_fringe_bitmap : FRINGE_ID_BITS, unsigned left_user_fringe_face_id : FACE_ID_BITS, unsigned right_user_fringe_face_id : FACE_ID_BITS, unsigned left_fringe_face_id : FACE_ID_BITS, unsigned right_fringe_face_id : FACE_ID_BITS, signed left_fringe_offset : FRINGE_HEIGHT_BITS, signed right_fringe_offset : FRINGE_HEIGHT_BITS, bool_bf fringe_bitmap_periodic_p : 1, bool_bf redraw_fringe_bitmaps_p : 1, bool_bf enabled_p : 1, bool_bf truncated_on_left_p : 1, bool_bf truncated_on_right_p : 1, bool_bf continued_p : 1, bool_bf displays_text_p : 1, bool_bf ends_at_zv_p : 1, bool_bf fill_line_p : 1, bool_bf indicate_empty_line_p : 1, bool_bf contains_overlapping_glyphs_p : 1, bool_bf full_width_p : 1, bool_bf mode_line_p : 1, bool_bf tab_line_p : 1, bool_bf overlapped_p : 1, bool_bf ends_in_middle_of_char_p : 1, bool_bf starts_in_middle_of_char_p : 1, bool_bf overlapping_p : 1, bool_bf mouse_face_p : 1, bool_bf ends_in_newline_from_string_p : 1, bool_bf exact_window_width_line_p : 1, bool_bf cursor_in_fringe_p : 1, bool_bf ends_in_ellipsis_p : 1, bool_bf indicate_bob_p : 1, bool_bf indicate_top_line_p : 1, bool_bf indicate_eob_p : 1, bool_bf indicate_bottom_line_p : 1, bool_bf reversed_p : 1, bool_bf stipple_p : 1, int continuation_lines_width
- [x] glyph_slice: unsigned x : 16, unsigned y : 16, unsigned width : 16, unsigned height : 16
- [x] it: Lisp_Object window, struct window* w, struct frame* f, enum it_method method, ptrdiff_t stop_charpos, ptrdiff_t prev_stop, ptrdiff_t base_level_stop, ptrdiff_t end_charpos, ptrdiff_t medium_narrowing_begv, ptrdiff_t medium_narrowing_zv, ptrdiff_t large_narrowing_begv, ptrdiff_t large_narrowing_zv, const unsigned char* s, ptrdiff_t string_nchars, bool_bf multibyte_p : 1, bool_bf tab_line_p : 1, bool_bf header_line_p : 1, bool_bf string_from_display_prop_p : 1, bool_bf string_from_prefix_prop_p : 1, bool_bf from_disp_prop_p : 1, bool_bf ellipsis_p : 1, bool_bf avoid_cursor_p : 1, struct Lisp_Char_Table* dp, Lisp_Object* dpvec, Lisp_Object* dpend, int dpvec_char_len, int dpvec_face_id, int saved_face_id, Lisp_Object ctl_chars, struct display_pos start, struct display_pos current, ptrdiff_t n_overlay_strings, ptrdiff_t overlay_strings_charpos, Lisp_Object overlay_strings, Lisp_Object string_overlays, Lisp_Object string, Lisp_Object from_overlay, Lisp_Object string, int string_nchars, ptrdiff_t end_charpos, ptrdiff_t stop_charpos, ptrdiff_t prev_stop, ptrdiff_t base_level_stop, struct composition_it cmp_it, int face_id, Lisp_Object object, struct it_slice slice, ptrdiff_t image_id, <aggregate> image, Lisp_Object object, <aggregate> stretch, Lisp_Object object, <aggregate> xwidget, <aggregate> u, struct text_pos position, struct display_pos current, Lisp_Object from_overlay, enum glyph_row_area area, enum it_method method, bidi_dir_t paragraph_embedding, bool_bf multibyte_p : 1, bool_bf string_from_display_prop_p : 1, bool_bf string_from_prefix_prop_p : 1, bool_bf display_ellipsis_p : 1, bool_bf avoid_cursor_p : 1, bool_bf bidi_p : 1, bool_bf from_disp_prop_p : 1, enum line_wrap_method line_wrap, short voffset, Lisp_Object space_width, Lisp_Object font_height, <aggregate> stack, int sp, ptrdiff_t selective, enum display_element_type what, int face_id, bool_bf selective_display_ellipsis_p : 1, bool_bf ctl_arrow_p : 1, bool_bf face_box_p : 1, bool_bf start_of_box_run_p : 1, bool_bf end_of_box_run_p : 1, bool_bf overlay_strings_at_end_processed_p : 1, bool_bf ignore_overlay_strings_at_pos_p : 1, bool_bf glyph_not_available_p : 1, bool_bf starts_in_middle_of_char_p : 1, bool_bf face_before_selective_p : 1, bool_bf constrain_row_ascent_descent_p : 1, bool_bf line_number_produced_p : 1, bool_bf align_visually_p : 1, enum line_wrap_method line_wrap, int base_face_id, int c, int len, struct composition_it cmp_it, int char_to_display, enum glyphless_display_method glyphless_method, ptrdiff_t image_id, struct xwidget* xwidget, struct it_slice slice, Lisp_Object space_width, short voffset, short tab_width, Lisp_Object font_height, Lisp_Object object, struct text_pos position, short truncation_pixel_width, short continuation_pixel_width, int first_visible_x, int last_visible_x, int last_visible_y, int extra_line_spacing, int extra_line_spacing_above, int max_extra_line_spacing, int override_ascent, int override_descent, int override_boff, struct glyph_row* glyph_row, enum glyph_row_area area, int nglyphs, int pixel_width, int ascent, int descent, int max_ascent, int max_descent, int phys_ascent, int phys_descent, int max_phys_ascent, int max_phys_descent, int current_x, int wrap_prefix_width, int continuation_lines_width, struct text_pos eol_pos, int current_y, int first_vpos, int vpos, int hpos, ptrdiff_t lnum, ptrdiff_t lnum_bytepos, int lnum_width, int lnum_pixel_width, ptrdiff_t pt_lnum, int stretch_adjust, unsigned left_user_fringe_bitmap : FRINGE_ID_BITS, unsigned right_user_fringe_bitmap : FRINGE_ID_BITS, unsigned left_user_fringe_face_id : FACE_ID_BITS, unsigned right_user_fringe_face_id : FACE_ID_BITS, bool_bf bidi_p : 1, struct bidi_it bidi_it, bidi_dir_t paragraph_embedding, Lisp_Object min_width_property, int min_width_start
- [x] it_slice: Lisp_Object x, Lisp_Object y, Lisp_Object width, Lisp_Object height
- [x] redisplay_interface: frame_parm_handler* frame_parm_handlers, void (*) produce_glyphs, void (*) write_glyphs, void (*) insert_glyphs, void (*) clear_end_of_line, void (*) scroll_run_hook, void (*) after_update_window_line_hook, void (*) update_window_begin_hook, void (*) update_window_end_hook, void (*) flush_display, void (*) clear_window_mouse_face, void (*) get_glyph_overhangs, void (*) fix_overlapping_area
- [x] run: int desired_y, int current_y, int desired_vpos, int current_vpos, int height, int nrows
- [x] text_pos: ptrdiff_t charpos, ptrdiff_t bytepos

## File-scope objects — 16
- [x] extern struct glyph space_glyph  (line 1338)
- [x] extern bool face_change  (line 2068)
- [x] extern bool redisplaying_p  (line 3608)
- [x] extern bool display_working_on_window_p  (line 3610)
- [x] extern int dont_resize_frames  (line 3611)
- [x] extern bool help_echo_showing_p  (line 3613)
- [x] extern Lisp_Object help_echo_string  (line 3614)
- [x] extern Lisp_Object help_echo_window  (line 3614)
- [x] extern Lisp_Object help_echo_object  (line 3615)
- [x] extern Lisp_Object previous_help_echo_string  (line 3615)
- [x] extern ptrdiff_t help_echo_pos  (line 3616)
- [x] extern int last_tab_bar_item  (line 3617)
- [x] extern int last_tool_bar_item  (line 3618)
- [x] extern int max_used_fringe_bitmap  (line 3710)
- [x] extern char unspecified_fg  (line 3834)
- [x] extern char unspecified_bg  (line 3834)
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

Not mirrored yet: the rest of C's second `HAVE_WINDOW_SYSTEM` block (`struct image_type`,
`struct image_cache`), the fringe and X-widget sections, and the gated branches not already
carried (besides `struct glyph_string`, `struct image` and `struct face`'s window-system pair).
The extractor's *only* unchecked box is `NativeRectangle`, whose deferral is an
`excluded-macros.txt` row (3.224) rather than missing work; the remaining content is the
deferred-branch set, which `--check` leaves out (`OK emacs/src/dispextern.h ->
femacs/src/include/dispextern.rs`).  A struct lands when the unit that needs its *layout* does
(`xtask layout-asserts` must be able to assert it: a type joins `tools/gen-layout-fields.py`'s
`STRUCTS` together with its mirror, since a key whose type is missing counts as pending and the
ratchet only shrinks pending) - or, for a record that exists only in a window-system arm, with
the P-82 readings and a window-system probe crate (`struct image`, below).

### The X11 arm of `Emacs_Rectangle` (round: the X11 front's first mirror work)

C spells `Emacs_Rectangle` twice: `typedef XRectangle Emacs_Rectangle;` under
`#ifdef HAVE_X_WINDOWS` (C 44, reached through the `<X11/Xlib.h>` at C 29) and a four-short
struct in the `#else` (C 66-70).  The mirror now carries both arms: the X11 one aliases the
Xlib record from `include/xlib.rs` (`pub type Emacs_Rectangle = XRectangle;`, so the mirror
includes that file under the same feature) and the frozen one keeps the struct the frozen
configuration measures.  The two arms are 8 and 16 bytes, which is why `struct MonitorInfo`
(two of them) is 32 bytes in the x11 arm and 48 in `ns_cocoa`
(`spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h`, `docs/ports/xterm.md`
Decision 16).  `python3 tools/extract-abi.py --check emacs/src/dispextern.h
femacs/src/include/dispextern.rs` is OK with both arms present.

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
  `glyph_u.xwidget` (`feature = "have_xwidgets"`), `glyph_matrix.method` (`feature = "glyph_debug"`)
  and `glyph_row.clip` (`feature = "have_window_system"`); none of them is part of the frozen
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

### Integrator addition (2026-10-04, with window.h's geometry family)

- The twelve display-dimension macros land here, not in `include/window.rs`: window.h's
  `WINDOW_MODE_LINE_HEIGHT`/`WINDOW_HEADER_LINE_HEIGHT`/`WINDOW_TAB_LINE_HEIGHT` and the two
  `WINDOW_BOX_*` macros call `CURRENT_*_LINE_HEIGHT` (1575/1616/1650), whose bodies call the
  three `MATRIX_*_LINE_HEIGHT` readers (1513/1529/1538) and `estimate_mode_line_height` (3582).
  They were duplicated at four use sites - `window.rs`, `dispnew.rs`, `menu.rs` and the
  in-flight `xdisp.rs`; the mirror is the landing point now, the three committed copies are
  deleted, and `xdisp.rs`'s is its own lane's to drop (P-26, docs/ports/include-window.md
  records the change from window.h's side).  Their bodies use C's `XFRAME ((W)->frame)` text
  rather than `WINDOW_FRAME (W)`, so this mirror needs no window.h name and no unit gains one.
  `estimate_mode_line_height` stays a use-site declaration in `window.rs`/`dispnew.rs`/
  `menu.rs`/`xdisp.rs` (dispextern.h 3582; xdisp.c defines it), because a crate-root
  declaration would collide with a defining unit (PORTING.md 0.1-10) - the module + glob
  import pattern is available if the integrator wants one landing point for it.
- Evidence: `python3 tools/extract-abi.py --check emacs/src/dispextern.h
  femacs/src/include/dispextern.rs` reports 141 problems against 153 before the change
  (none of them these twelve), and `build-tu src/dispnew.c`, `src/menu.c`, `src/window.c`
  and their 41 siblings that include window.rs all report `built`.

## Decisions

### The window-system record `struct glyph_string` (dispextern.h 1358-1509), added 2026-10-04

- **It is dead text on this target, and it is carried anyway under P-51's feature.**  The
  record sits between C 1358's `#ifdef HAVE_WINDOW_SYSTEM` and C 1509's matching `#endif`;
  `HAVE_WINDOW_SYSTEM` is `undef` in the frozen configuration
  (`docs/baseline/config/darwin-arm64.macros.tsv`, and `tools/check-cond-eval.py src/dispextern.h`
  reports 0 disagreements with clang for this file), so the frozen build never compiled it and
  `--check`'s container list does not contain it either (it is not one of the seven types the
  extractor defers for this file).  The mirror carries the record under
  `#[cfg (feature = "have_window_system")]` - P-51's rule, the feature naming the *non*-frozen side -
  so the compiled build is exactly what clang compiles.
- **Who needs it (why it landed now).**  The window-system backends need the record *and its
  fields*, not only its name: `src/nsfont.m`'s GNUstep port reads `s->char2b[i]` (nsfont.m
  1176), `s->first_glyph->type` (1161) and `s->first_glyph[i].u.ch` (1720) - the Swift lane's
  `nsfont_draw` (1151) and `ns_dump_glyphstring` (1711) are the immediate consumer - and the X,
  W32 and pgtk backends read the same members.  The alternative (an opaque placeholder such as
  `femacs/src/font.rs` 739's `pub struct glyph_string;`) was rejected by the integrator, and
  this is the record's owner: `dispextern.h` is where C defines it.
- **Field types: named where an owner mirror exists *in this header's include graph*,
  `*mut c_void` + `/// C:` where not.**
  `frame`, `window` and `font` are named (`include/frame.rs`, `include/window.rs`,
  `include/font.rs`), because a unit that enables this feature has them by construction: C's
  backends include frame.h/window.h/font.h, and `src/nsfont.m` 27-39 includes `dispextern.h`,
  `frame.h`, `window.h` and `font.h`.  `composition` was named here until status 3.526: it does
  have an owner mirror (`include/composite.rs`), but C's dispextern.h does **not** include
  composite.h - it names the tag only as a pointer - and the measured counterexample is
  `src/fontset.c` 36 (2 x `E0425 composition` on the `fontset` arm), so
  `glyph_string.cmp` is `*mut c_void` with its `/// C:` line now.  Note the contrast
  with this mirror's *live* text, where the same types degrade (`glyph.frame`,
  dispextern.rs 293; `Mouse_HLInfo.mouse_face_mouse_frame`, 1591; `glyph.buffer`, 686): those
  fields are compiled by every unit that takes this mirror, this record only by a window-system
  unit.  Degraded here, each with the `/// C:` line P-55 asks for:
  `face` (C 1414; the record is C 1787 in this same header and is not mirrored yet),
  `xwidget` (C 1486; xwidget.h, still a todo header in
  `docs/port-inventory.md`), `GC gc` (C 1465; X11's `struct _XGC *` - the only `GC` this tree
  declares is msdos.h's `typedef int GC`, msdos.rs 217, a different type with the same name, so
  naming it would silently pick that one up), `struct android_gc *gc` (C 1467; not in
  androidgui.rs) and `HDC hdc` (C 1471; `<windows.h>`).  `Emacs_GC *gc` (C 1470) and
  `Emacs_GC xgcv` (C 1474) *are* named: this mirror declares `Emacs_GC` itself (dispextern.rs 51).
- **The four platform `gc` arms are carried under their own features**, because a window-system
  build picks exactly one: `have_x_windows` (C 1464-1465), `have_android` (1466-1467), `have_ntgui`
  (1469-1471), `have_pgtk` (1473-1474) - the spellings the tree's units use for those four C macros.
  `keyboard.rs` 4057/4470/4549 spells the *same* C macro `HAVE_X_WINDOWS` as
  `have_x_windows`, so the tree currently has two spellings for one condition; that split is
  recorded here for the integrator, because a unit that enabled only `have_x_windows` would
  silently leave this record's `gc` out (and if they standardise on `have_x_windows`, this arm
  changes with them).
- **The one bitfield run is P-80's shape, and only the run is measured.**  C 1431-1461 is four
  `bool_bf : 1` members, `unsigned for_overlaps : 3` and one more `bool_bf : 1`; the mirror
  stores one `bits: u32` and puts the six member names in `glyph_string_bits!`'s arms and in
  `GLYPH_STRING_*` masks.  The `u32` and the six positions come from a **reduced probe** - those
  six declarations alone, compiled by the frozen clang with no window-system type - which prints
  `sizeof = 4` and `ff 00 00 00` with all six set (`01 00 00 00` for the first member alone,
  `71 00 00 00` for it plus `for_overlaps`), i.e. one byte of members inside a four-byte unit,
  `extends_to_end_of_line_p` at bit 0 through `padding_p` at bit 7.  **The record's own layout is
  not measured and no assertion is written for it**: this target has no window-system
  configuration to measure it with, so `sizeof`/offsets of `glyph_string` - and of every record
  that embeds one - would be invented numbers (the task's rule; the port's own rule, P-80④, is
  that measured byte patterns are what licence a bitfield unit).  It is asserted nowhere and is
  `layout-check`-invisible by construction (the item is `cfg`'d out).
- **`NativeRectangle clip[2]` (C 1497) is carried as `[c_int; 2]` and the type macro is
  *not* added here.**  `NativeRectangle` is a type macro whose answer differs per window system:
  C 119's `#define NativeRectangle XRectangle` (X11), C 178-180's fallback
  `#ifndef NativeRectangle / #define NativeRectangle int` - *live* text in the frozen
  configuration - plus `RECT` in w32gui.h and `struct haiku_rect` in haikugui.h.  `int` is what a
  window-system build without X, Android or Haiku has (the NS target's answer), and the port's
  shape for the macro is a `macro_rules!` at the platform header (w32gui.rs 148, haikugui.rs 64);
  this mirror cannot also carry one, because two `macro_rules! NativeRectangle` in
  `layout-check`'s single crate is a macro-namespace collision with `w32gui.rs`'s.  The
  integrator owns that call; `[c_int; 2]` must become `[NativeRectangle; 2]`-equivalent text on
  the X/XRectangle and Haiku paths (the array is 16 bytes there, 8 here).
- **The four `OVERLAPS_*` macros (C 1445-1448) are carried under the same feature.**  They are
  written inside the record's body in C but are macros in its sense (visible to every unit that
  compiles the arm).  `femacs/src/dispnew.rs` 3646-3647 carries `OVERLAPS_PRED`/`OVERLAPS_SUCC`
  as *live* copies of its own, because the frozen configuration has none - that is a use-site
  copy to retire when the window-system target is frozen (the `struct font` precedent, status
  3.158), not a collision today (`dispnew.rs`'s copies are live, this pair is `cfg`'d out).
- **Two more use-site copies this record's landing creates a claim on** (reported, not edited -
  they live in other lanes' files): `femacs/src/font.rs` 739 declares
  `pub struct glyph_string;` as an opaque placeholder for `font.h` 693's
  `font_driver.draw` (that field is already `#[cfg (feature = "have_window_system")]`, font.rs 283),
  so a window-system build now needs that placeholder retired; and `include/nsgui.rs`,
  `include/haikuterm.rs` (489-494) and `include/nsxwidget.rs` (23-78) degrade
  `struct glyph_string *` to `*mut c_void` with the note that the record "has no mirror yet" -
  those notes are now stale, and each of those mirrors would upgrade to `*mut glyph_string` by
  adding this mirror to its own `include!` list when its target is frozen (P-72④).

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/dispextern.h femacs/src/include/dispextern.rs`
  -> **142 problem lines both before and after the addition, byte-identical lists** (`diff` of
  the two runs is empty).  The mirror is a WIP slice - its ledger still has 78 unchecked boxes -
  and the 142 lines are the not-yet-mirrored *live* text of this header: 43 missing macros, 76
  missing constants, 16 missing file-scope objects, 5 missing functions and the two
  `GLYPH_CHAR`/`GLYPH_FACE` token lines.  A full `--check OK` is therefore not achievable by
  adding a record the frozen evaluation does not even require; the count is the ledger's
  progress signal (the previous round recorded 141/153).  Adding the `cfg`'d record cannot
  change it in either direction, which the identical lists measure.
- `python3 tools/check-cond-eval.py src/dispextern.h` -> `71 conditions, 0 unknown,
  0 disagreements [VERIFIED]`.
- **Compile proof, feature on** (a throwaway probe under `/tmp`, not added to the tree: it
  declares stubs for the four owner records and for lisp.h's/character.h's names this mirror's
  live text uses - `EMACS_INT`, `intmax_t`, `MAX_CHAR`, `CHARACTERBITS`, `EMACS_INT_MAX`,
  `CONSP`, `XCAR`, `XCDR`, `XFIXNUM`, `RANGED_FIXNUMP`, `CHARACTERP`, `CHAR_VALID_P` - then
  `include!`s the real `femacs/src/include/dispextern.rs`):
  `rustc --edition 2024 --crate-type lib --emit metadata --cfg 'feature = "have_window_system"'
  /tmp/gstr_feature_probe.rs` -> exit 0, 0 warnings; the probe calls
  `glyph_string_bits! (padding_p, *s, true)` and `size_of::<glyph_string> ()`, so the record's
  field names and types and the bit accessors really resolve.  Negative control: the same probe
  against a `/tmp` copy of the mirror with `pub f: *mut frame` renamed to `*mut frame_typo`
  fails with `error[E0425]: cannot find type frame_typo` - the probe has teeth.
- **Bitfield run, reduced probe** (`clang -std=gnu23 -O0`, a file holding exactly C 1431-1461's
  six declarations): `sizeof 4`, `_Alignof 4`, and the byte dumps above.  The *record's* layout
  is not measured (no window-system configuration on this target) and is asserted nowhere.
- `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-disp cargo run -q -p xtask -- layout-check`
  -> `layout probe ok` / `layout-check passed (see build/femacs/layout_asserts.rs)`;
  `... -- probe ../spikes/sfnt_inc_probe.rs ...` -> exit 0, 0 warnings;
  `... -- build-tu src/font.c` -> `built src/font.rs -> ../build/femacs/font.o` (this is the
  unit that would hit `E0428` against `font.rs` 739's placeholder if the record were not
  `cfg`'d - it builds), `build-tu src/frame.c` -> `built src/frame.rs -> ../build/femacs/frame.o`.
- Gate readings on this worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed
  literals) / 0 failures; `check-globals` 283 units / 0; `check-mirror-dupes` 1 duplicate
  declaration / 1 ratchet row / 0 failures; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 595 `.rs` / 593 map targets / 0 findings.
- Same round, `src/sfnt.h`: the placeholder `macro_rules! PROTOTYPE` the previous round added is
  deleted now that `docs/ports/excluded-macros.txt` 152 carries the row; `--check
  emacs/src/sfnt.h femacs/src/include/sfnt.rs` prints `OK` and `(1 C macros have a recorded Rust
  counterpart: PROTOTYPE)`.

## Decisions (the two records `frame.h` was waiting for)

- **Landed: `struct face` (C 1787-1937) and `struct face_cache` (C 1991-2009)**, in C's order,
  `#[repr(C)]`, each with the measured layout asserted inline.  `face`: `sizeof` 280, align 8,
  `lface`@0, `id`@160, `foreground`@168, `background`@176, `underline_color`@184,
  `overline_color`@192, `strike_through_color`@200, `box_color`@208, `font`@216, `fontset`@224,
  `box_vertical_line_width`@228, `box_horizontal_line_width`@232,
  `underline_pixels_above_descent_line`@236, the `box : 2`/`underline : 3` + seventeen `bool_bf`
  bits C packs into the four-byte unit at 240 (stored as C's bytes and named through the masks,
  PORTING.md 1.3/P-80), `hash`@248, `next`@256, `prev`@264, `ascii_face`@272.
  `face_cache`: `sizeof` 40, `buckets`@0, `f`@8, `faces_by_id`@16, `size`@24, `used`@32,
  `menu_face_changed_p`@36 (a `bool_bf : 1` carried as the byte it occupies).
  The numbers are the frozen clang's and are the ones `src/xfaces.rs` 279-345 already asserts,
  so the copy that lands here is field-for-field the copy that unit verified.
- **Two members are the untyped pointer (P-72 rule ③), measured, not preferred**:
  `face.font` and `face_cache.f`.  C's dispextern.h knows both tags only as incomplete ones, and
  a unit that includes this mirror need not include `include/font.rs` or `include/frame.rs`:
  with `font: *mut font` `build-tu src/term.c` answered *"cannot find type `font`"*, and with
  `f: *mut frame` `build-tu src/bidi.c` and `src/lread.c` answered *"cannot find type `frame`"*.
  `src/xfaces.rs` 294 spells `*mut font` and `src/window.rs` 899 `*mut font` too, so the units
  that retire their copies cast the two members; that cast is part of the retirement, not of this
  mirror.
- **Seven units still carry a local copy of the two records** - the shape of the time when this
  mirror had neither - and, **by the ruling below, they keep them**.  File and line, both records:
  `src/alloc.rs` `face` 3442 / `face_cache` 3453; `src/dispnew.rs` 3610 / 3618;
  `src/font.rs` 456 / 519; `src/term.rs` 323 / 308; `src/window.rs` 895 / 906;
  `src/xdisp.rs` 1062 / 1108; `src/xfaces.rs` 288 / 337.  Every other unit that includes this
  mirror builds (measured: `src/character.c`, `src/bidi.c`, `src/lread.c`, `src/bytecode.c`,
  `src/syntax.c`, `src/marker.c`, `src/frame.c`, `src/keyboard.c`).
- **Ruling (status 3.220): the records stay behind the flip feature, and the units keep their
  live copies.**  Landing them unconditionally made all seven units red (`E0428`), and the
  `stdbit.in.rs` retirement of 3.181 was measured as the alternative: the smallest unit,
  `src/alloc.rs`, needs three edits after deleting its two records (one `E0308` and two
  `no field header on type c_void`), the whole set is roughly 40-60 edits, and it would have to
  touch two lanes' in-flight files (`src/xdisp.rs`, `src/font.rs`) and downgrade their *typed*
  accessors to casts.  The ruling is therefore the cheap arm that keeps C's text present: this
  copy is gated `#[cfg (feature = "dispextern_face_records")]` (a row in
  `femacs/xtask/Cargo.toml`, so `xtask dead-arms`/`gate-probes` compiles it when the feature is
  flipped), it is field-for-field the copy `src/xfaces.rs` 279-345 asserts, and the units' typed
  copies stay live.  The alternative and its measured cost are in the mirror's own comment too,
  so a later round can take it deliberately.  The two `*mut c_void` members (`face.font`,
  `face_cache.f`) are then the *flipped* arm's spelling - what a consumer of this mirror needs
  (measured: `build-tu src/term.c` / `src/bidi.c` / `src/lread.c` answered "cannot find type") -
  while the live typed copies spell them `*mut font` / `*mut frame`.
- **What is still open: the same 142 `- ` lines**, now classified (the two records were in the
  gate's *deferred* set, so closing them moved no line, only the deferral list):
  5 `missing function`, 43 `missing macro`, 76 `missing constant`, 17 `missing file-scope object`,
  and 2 body-token lines (`GLYPH_CHAR` does not mention `glyph_char`, `GLYPH_FACE` does not
  mention `glyph_face`).  The remaining deferred types are five: `bidi_it`, `composition_it`,
  `draw_fringe_bitmap_params`, `it`, `redisplay_interface`.  Each of those is its own slice.

## Evidence (the two records)

- **The judge's reading**: `python3 tools/check-cond-eval.py src/dispextern.h` ->
  `src/dispextern.h: 71 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the extractor
  resolves the arms against the frozen table and every item it lists is *live* text.
- **`--check`**: `python3 tools/extract-abi.py --check emacs/src/dispextern.h
  femacs/src/include/dispextern.rs` -> `FAIL`, **142** `- ` lines, unchanged by this slice, and
  `grep -c '^- '` before and after the two records is the same `142` with *no* line added or
  removed (`diff` of the two sorted lists is empty); the deferral line on stderr went from seven
  names to five (`bidi_it composition_it draw_fringe_bitmap_params it redisplay_interface`).
  The ledger's unchecked boxes went from 78 to 76 (the two type boxes).
- **Layout assertions**: the two records carry 15 + 6 `const _: () = assert! (…)` lines (the
  offsets above plus both sizes); they are the copy `src/xfaces.rs` 279-345 carries, whose
  numbers came from the frozen clang (`sizeof`/`offsetof` probe, recorded in
  docs/ports/include-frame.md's third slice for the same records).
- **`layout-check`**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-dx cargo run -q -p xtask --
  layout-check` -> the run **immediately after landing the two records** printed
  `layout probe ok` / `layout-check passed` (that is this mirror's compile evidence for them);
  at the end of the round the same command is **red, and every diagnostic is in another lane's
  in-flight `femacs/src/include/xterm.rs`** (`expected identifier, found keyword `use`` at 272,
  `expected `mut` or `const` keyword in raw pointer type` at 1263-1265) - no diagnostic names
  `dispextern.rs`, and I did not touch that file (the `ftfont.rs` precedent of status 3.181).
- **The units' `build-tu` after the last edit**: `src/character.c` -> `built src/character.rs ->
  ../build/femacs/character.o`; `src/bidi.c` -> `built src/bidi.rs -> …/bidi.o`; `src/lread.c` ->
  `built src/lread.rs -> …/lread.o`; `src/bytecode.c`, `src/syntax.c`, `src/marker.c`,
  `src/frame.c` (`built src/frame.rs -> …/frame.o`) and `src/keyboard.c` (`built
  src/keyboard.rs -> …/keyboard.o`) all the same; and the seven units of the bullet above answer
  `error[E0428]: the name `face` is defined multiple times` (and the same for `face_cache`) while
  the records were ungated; **with the 3.220 ruling in place they build again** (`src/dispnew.c`,
  `src/term.c`, `src/window.c`, `src/xfaces.c`, `src/alloc.c` all `built`), which is the ruling's
  own check.

## Decisions (the freely landable remainder, dispextern.h 105-3834)

- **H. The integrator repaired two defects of this lane's first append in place (status 3.221);
  both repairs stay.**  (i) The append landed `GLYPH_CHAR`/`GLYPH_FACE` a *second* time at the end
  of the file, which is `E0428` in every unit that includes this mirror (measured: `build-tu
  src/alloc.c`, `src/keyboard.c`, `src/dispnew.c`); the integrator moved the two
  `let glyph_char = glyph.ch; glyph_char` bodies - the spelling the two
  `function … does not mention …` token lines ask for - to C's position (right after
  `struct GLYPH`) and deleted the trailing pair.  (ii) C 104's `#include <c-strcase.h>` had no
  counterpart here, so C 110's `c_strcasecmp` had no declaration in scope; the integrator added the
  include of the mirror that owns that name (`lib/include/c-strcase.rs`), which is the "name the
  owner" rule for a name C takes from its includer's graph.  This lane's only follow-up is the
  module placement in the `xstrcasecmp` entry below.
- **The slice is defined by a measurement, not by taste: an item is landable here exactly when no
  unit already declares it.**  Matching every one of the round's 105 `- missing …` names against a
  declaration in `femacs/src/*.rs` + `femacs/lib/*.rs` (the `fn`/`const`/`static`/`type`/
  `macro_rules!`/`mod` forms, one match per name per file) splits them 102 / 2 / 1: **102 names are
  declared by a unit** (listed by owner in the Evidence), so landing them here is `E0428` in that
  unit - the collision status 3.220 ruled on for the two face records - and `NativeRectangle` is
  the one name no unit declares, deferred on its own grounds below.  (`IT_STACK_SIZE` is both a
  macro and a constant in C, so 105 `- missing` lines carry 103 distinct names.)
- **The 29 items that are free by that measurement are landed**, in C's order: `xstrcasecmp`
  (105-111, in a module - next entry), `TEXT_POS_EQUAL_P` (340-343), `DESIRED_MODE_LINE_HEIGHT`
  (1661-1662), `DESIRED_HEADER_LINE_HEIGHT` (1666-1667), `DESIRED_TAB_LINE_HEIGHT` (1671-1672),
  `VCENTER_BASELINE_OFFSET` (1702-1705), `FONT_TOO_HIGH` (1712-1713), `FACE_UNIBYTE_P` (1953),
  `FACE_EXTENSIBLE_P` (2011-2012), `MAX_FRINGE_BITMAPS` (2235), `IF_DEBUG` (262-265),
  `ITERATOR_AT_END_P` (2948), the three `DEFAULT_TAB_BAR_*` (3435/3436/3440) and four
  `DEFAULT_TOOL_BAR_*` (3510/3514/3515/3519) defaults, `enum tool_bar_item_image` with its four
  `TOOL_BAR_IMAGE_*` members (3502-3508), `TRY_WINDOW_CHECK_MARGINS`/`TRY_WINDOW_IGNORE_FONTS_CHANGE`
  (3699/3700) and the three objects `last_tab_bar_item` (3617), `last_tool_bar_item` (3618) and
  `max_used_fringe_bitmap` (3710).  The `DEFAULT_*` names and the `TOOL_BAR_IMAGE_*` members each
  count twice in the gate (macro + constant), which is why 29 items retire 37 `- ` lines.
- **`xstrcasecmp` (C 104-111) sits in `dispextern_externs`, a module the including unit does not
  glob-import.**  `src/font.rs` 561 carries its own definition of the same C name (the shape of the
  time this mirror had none) and includes this mirror, so a copy at the mirror's top level is
  `E0428` there (measured: `build-tu src/font.c` -> "the name `xstrcasecmp` is defined multiple
  times", with `src/font.rs` 561 named as the other definition).  The module is the `bignum.rs`
  `bignum_externs` / `cm.rs` `cm_globals` shape (P-21/P-39): C's text is present, the unit's
  definition stays the live one, and the two do not collide.  Both `build-tu src/font.c` and
  `layout-check` are green with it.
- **`NativeRectangle` (C 176-181) stays unspelled, and its box stays unchecked.**  It is a *type*
  macro whose answer is the window system's (`XRectangle` / `RECT` / `struct haiku_rect` / `int`);
  the platform mirrors own their `macro_rules!` (w32gui.rs 148, haikugui.rs 64, pgtkgui.rs 214,
  nsgui.rs 91, androidgui.rs 245) and a second one in `layout-check`'s single crate is a
  macro-namespace collision with theirs, while a `pub type` alias here would hand every
  window-system unit this target's 8-byte `int`.  An `excluded-macros.txt` row
  (`NativeRectangle`, `kind=config`) is requested so the inventory gate can report the deferral
  instead of a missing macro.
- **`IF_DEBUG` (C 262-265) is landed as the live arm's no-op.**  `GLYPH_DEBUG` is undef, so the
  evaluated body is `((void) 0)` and the port's expansion (`let _ = $x;`) has no `0` to name; a
  `macro-body-notes.tsv` row (`IF_DEBUG`, token `0`, the `FD_SET`/`FD_CLR` class) is requested, and
  this ledger's box stays unchecked until it lands.
- **What is left is not landable from this mirror alone.**  The C 2600-4026 windowed machinery
  (`struct image_cache`, the fringe/X-widget/cursor records) is *dead text* on this target
  (`HAVE_WINDOW_SYSTEM` is undef: C 3203-3395 and 3633-3686), so P-51/P-66 apply.
  `struct image` (C 3215) has since landed under that arm - see its own section below.  The five types the extractor still defers are live text but unit-owned exactly like
  the 102 names: `struct bidi_it` (`src/bidi.rs` 86), `composition_it` (`src/dispnew.rs` 6828 and
  `src/indent.rs` 180), `struct it` (`src/dispnew.rs` 6855), `redisplay_interface`
  (`src/dispnew.rs` 3569 and `src/minibuf.rs` 666), `draw_fringe_bitmap_params` (no unit); landing
  any of them needs the 3.220 shape (a flip feature row) or the unit retirement, both of which the
  integrator owns.

## Evidence (the freely landable remainder)

- **`--check`, measured against the committed mirror and the working tree**: `git show
  HEAD:femacs/src/include/dispextern.rs > /tmp/dxhead/dispextern.rs` then
  `python3 tools/extract-abi.py --check emacs/src/dispextern.h /tmp/dxhead/dispextern.rs` ->
  `FAIL`, `grep -c '^- '` **142** (140 `missing` + the two `GLYPH_CHAR`/`GLYPH_FACE` token lines);
  the same command on `femacs/src/include/dispextern.rs` -> `FAIL`, `grep -c '^- '` **106**
  (105 `missing` = 4 functions + 23 macros + 65 constants + 13 objects, plus one token line,
  `macro IF_DEBUG does not mention 0 (C body: ((void) 0))`).  `diff` of the two sorted lists: 37
  lines gone (the 29 landed items - the `DEFAULT_*`/`TOOL_BAR_IMAGE_*` ones twice each - plus
  `missing function xstrcasecmp` and the two `GLYPH_*` token lines) and 1 line new (the `IF_DEBUG`
  token line).
- **The free / unit-declared split** (the round's 105 `- missing` lines, 103 distinct names):

```
xdisp.rs     (63): BIDI_AT_BASE_LEVEL, BYTEPOS, CHARPOS, COMPOSITION_PROP_IDX, DEC_TEXT_POS,
  DISPLAY_PROP_IDX, FACE_FOR_CHAR, FACE_PROP_IDX, FONTIFIED_PROP_IDX, GET_FROM_BUFFER,
  GET_FROM_C_STRING, GET_FROM_DISPLAY_VECTOR, GET_FROM_IMAGE, GET_FROM_STRETCH, GET_FROM_STRING,
  GET_FROM_XWIDGET, GLYPHLESS_DISPLAY_ACRONYM, GLYPHLESS_DISPLAY_EMPTY_BOX,
  GLYPHLESS_DISPLAY_HEX_CODE, GLYPHLESS_DISPLAY_THIN_SPACE, INC_TEXT_POS, INVISIBLE_PROP_IDX,
  ITERATOR_AT_END_OF_LINE_P, IT_BYTEPOS, IT_CHARACTER, IT_CHARPOS, IT_COMPOSITION,
  IT_CONTINUATION, IT_EOB, IT_GLYPHLESS, IT_IMAGE, IT_STACK_SIZE, IT_STRETCH, IT_STRING_BYTEPOS,
  IT_STRING_CHARPOS, IT_TRUNCATION, IT_XWIDGET, LAST_PROP_IDX, MOVE_TO_POS, MOVE_TO_VPOS,
  MOVE_TO_X, MOVE_TO_Y, NUM_IT_METHODS, OVERLAY_PROP_IDX, OVERLAY_STRING_CHUNK_SIZE,
  PRODUCE_GLYPHS, SET_MARKER_FROM_TEXT_POS, SET_TEXT_POS, SET_TEXT_POS_FROM_MARKER, TRUNCATE,
  WINDOW_WRAP, WORD_WRAP, display_working_on_window_p, dont_resize_frames, face_change,
  help_echo_object, help_echo_pos, help_echo_showing_p, help_echo_string, help_echo_window,
  previous_help_echo_string, redisplay_counter, redisplaying_p
dispnew.rs   (30): BYTEPOS, CHARPOS, CHAR_GLYPH_SPACE_P, CLIP_TEXT_POS_FROM_MARKER,
  FACE_TTY_DEFAULT_BG_COLOR, FACE_TTY_DEFAULT_FG_COLOR, IT_CHARACTER, IT_CHARPOS, IT_COMPOSITION,
  IT_CONTINUATION, IT_EOB, IT_GLYPHLESS, IT_IMAGE, IT_STRETCH, IT_TRUNCATION, MOVE_TO_POS,
  MOVE_TO_VPOS, MOVE_TO_X, MOVE_TO_Y, SET_GLYPH_FROM_GLYPH_CODE, TRUNCATE, WINDOW_WRAP,
  WORD_WRAP, dont_resize_frames, help_echo_object, help_echo_pos, help_echo_string,
  help_echo_window, previous_help_echo_string, redisplaying_p
indent.rs    (25): BYTEPOS, CHARPOS, GET_FROM_BUFFER, GET_FROM_C_STRING, GET_FROM_DISPLAY_VECTOR,
  GET_FROM_IMAGE, GET_FROM_STRETCH, GET_FROM_STRING, GET_FROM_XWIDGET, IT_BYTEPOS, IT_CHARPOS,
  IT_STACK_SIZE, IT_STRING_BYTEPOS, MOVE_TO_POS, MOVE_TO_VPOS, MOVE_TO_X, MOVE_TO_Y,
  NUM_IT_METHODS, OVERLAY_STRING_CHUNK_SIZE, SET_TEXT_POS, SET_TEXT_POS_FROM_MARKER, TRUNCATE,
  WINDOW_WRAP, WORD_WRAP, display_working_on_window_p
window.rs    (25): BYTEPOS, CHARPOS, CLIP_TEXT_POS_FROM_MARKER, IT_BYTEPOS, IT_CHARACTER,
  IT_CHARPOS, IT_COMPOSITION, IT_CONTINUATION, IT_EOB, IT_GLYPHLESS, IT_IMAGE, IT_STACK_SIZE,
  IT_STRETCH, IT_TRUNCATION, IT_XWIDGET, MOVE_TO_POS, MOVE_TO_VPOS, MOVE_TO_X, MOVE_TO_Y,
  OVERLAY_STRING_CHUNK_SIZE, SET_TEXT_POS, SET_TEXT_POS_FROM_MARKER, display_working_on_window_p,
  face_change, reset_mouse_highlight
term.rs      (25): CHARPOS, FACE_TTY_DEFAULT_BG_COLOR, FACE_TTY_DEFAULT_COLOR,
  FACE_TTY_DEFAULT_FG_COLOR, GLYPHLESS_DISPLAY_ACRONYM, GLYPHLESS_DISPLAY_EMPTY_BOX,
  GLYPHLESS_DISPLAY_HEX_CODE, GLYPHLESS_DISPLAY_THIN_SPACE, IT_CHARACTER, IT_COMPOSITION,
  IT_CONTINUATION, IT_EOB, IT_GLYPHLESS, IT_IMAGE, IT_STRETCH, IT_TRUNCATION, TTY_CAP_BOLD,
  TTY_CAP_DIM, TTY_CAP_INVERSE, TTY_CAP_ITALIC, TTY_CAP_OVERLINE, TTY_CAP_STRIKE_THROUGH,
  TTY_CAP_UNDERLINE, TTY_CAP_UNDERLINE_STYLED, face_tty_specified_color
keyboard.rs  (21): CHARPOS, TAB_BAR_ITEM_BINDING, TAB_BAR_ITEM_CAPTION, TAB_BAR_ITEM_ENABLED_P,
  TAB_BAR_ITEM_HELP, TAB_BAR_ITEM_KEY, TAB_BAR_ITEM_NSLOTS, TAB_BAR_ITEM_SELECTED_P,
  TOOL_BAR_ITEM_BINDING, TOOL_BAR_ITEM_CAPTION, TOOL_BAR_ITEM_ENABLED_P, TOOL_BAR_ITEM_HELP,
  TOOL_BAR_ITEM_IMAGES, TOOL_BAR_ITEM_KEY, TOOL_BAR_ITEM_LABEL, TOOL_BAR_ITEM_NSLOTS,
  TOOL_BAR_ITEM_RTL_IMAGE, TOOL_BAR_ITEM_SELECTED_P, TOOL_BAR_ITEM_TYPE, TOOL_BAR_ITEM_VERT_ONLY,
  TOOL_BAR_ITEM_WRAP
xfaces.rs    (16): FACE_FOR_CHAR, FACE_SUITABLE_FOR_ASCII_CHAR_P, FACE_TTY_DEFAULT_BG_COLOR,
  FACE_TTY_DEFAULT_COLOR, FACE_TTY_DEFAULT_FG_COLOR, TTY_CAP_BOLD, TTY_CAP_DIM, TTY_CAP_INVERSE,
  TTY_CAP_ITALIC, TTY_CAP_OVERLINE, TTY_CAP_STRIKE_THROUGH, TTY_CAP_UNDERLINE,
  TTY_CAP_UNDERLINE_STYLED, face_change, unspecified_bg, unspecified_fg
frame.rs      (5): FACE_TTY_DEFAULT_BG_COLOR, FACE_TTY_DEFAULT_COLOR, FACE_TTY_DEFAULT_FG_COLOR,
  unspecified_bg, unspecified_fg
bidi.rs       (2): IT_STACK_SIZE, SET_TEXT_POS
eval.rs       (2): redisplay_counter, redisplaying_p
font.rs       (1): FACE_FOR_CHAR
free         (1): NativeRectangle
```

- **`layout-check`** (the mirror is in the seed set, so this is its compile evidence):
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-dx cargo run -q -p xtask -- layout-check` ->
  `layout probe ok` / `layout-check passed (see build/femacs/layout_asserts.rs)` - run after the
  module move *and* after the ledger edit.
- **The units' `build-tu`**: `src/character.c` -> `built src/character.rs ->
  ../build/femacs/character.o`; `src/bidi.c` -> `built src/bidi.rs -> …/bidi.o`; `src/lread.c` ->
  `built src/lread.rs -> …/lread.o`; `src/frame.c` -> `built src/frame.rs -> …/frame.o`;
  `src/dispnew.c` -> `built src/dispnew.rs -> …/dispnew.o`; `src/xfaces.c` -> `built
  src/xfaces.rs -> …/xfaces.o`; `src/font.c` -> `built src/font.rs -> …/font.o` (the module move's
  own check: the same command answered `error[E0428]: the name `xstrcasecmp` is defined multiple
  times … src/font.rs 561` before it).  `src/keyboard.c` answers `error[E0609]: no field `bits` on
  type `buffered_input_event`` at `keyboard.rs`'s own `input_event_bits!` macro (4089-4090) - a
  diagnostic that names neither this mirror nor any file of it, in another lane's in-flight unit.
- **The other text gates, in one run**: `check-declarations` -> `13 declaration sites (0
  forbidden, 13 allowed literals), 0 failures`; `check-globals` -> `283 units, 0 DEFVAR_*
  variables without a definition in their own unit`; `check-mirror-dupes` -> `1 duplicate
  declaration(s), 1 ratchet row(s), 0 failure(s)`; `check-verbatim` -> `6 trees, 4607 files, 0
  failures`; `check-target-names` -> `626 .rs files, 591 map targets, 0 finding(s)`.

## Decisions (the unit-owned names, dispextern.h 286-3834)

- **Shape: one non-glob-imported module, `dispextern_unit_names`, carrying C's names in C order.**
  The measurement below says 102 of the round's 104 `--check` items are names a unit that
  includes this mirror already declares, so a copy at the mirror's top level is `E0428` in that
  unit - the collision status 3.220 ruled on for the two face records.  Of the three shapes 3.220
  left open for names of this class, this is the one `xstrcasecmp` already uses (3.224), and it is
  preferred over a `#[cfg (feature = …)]` arm here because
  * it needs no shared row: status 3.187 requires a `--cfg` producer for a feature name, and
    `femacs/xtask/Cargo.toml` plus the FLIPS rows are the integrator's; one feature for the whole
    class would still leave 87 names invisible to every ordinary build;
  * a module's contents are *compiled* by every unit that includes this mirror and by
    `layout-check`, so C's constants, signatures and bodies are type-checked in the normal build,
    where a cfg'd-out arm is compiled only when `dead-arms`/`gate-probes` flips it.
  The module makes no unit file change of any kind (P-21/P-39's `bignum_externs`/`cm_globals`
  shape), and the units' own copies stay live by the 3.220 ruling.
- **The 87 names landed, in C order.**  The text-position accessors `BYTEPOS`, `CHARPOS`,
  `SET_TEXT_POS`, `SET_TEXT_POS_FROM_MARKER`, `CLIP_TEXT_POS_FROM_MARKER` (286-343);
  `SET_GLYPH_FROM_GLYPH_CODE` (414-420) and `CHAR_GLYPH_SPACE_P` (648-650); the three
  `FACE_TTY_DEFAULT_*` sentinels (1932/1936/1940) and `face_tty_specified_color` (1944-1949);
  `FACE_SUITABLE_FOR_ASCII_CHAR_P` (2015-2022) and `FACE_FOR_CHAR` (2030-2038); the object
  `face_change` (2068); the nine `IT_*` display-element members (2285-2323), the three
  `line_wrap_method` members (2338-2343), the four `glyphless_display_method` members
  (2347-2365), the eight `it_method` members (2371-2380), `IT_STACK_SIZE` (2383),
  `OVERLAY_STRING_CHUNK_SIZE` (2572), the three iterator accessors `IT_CHARPOS`/`IT_BYTEPOS`/
  `IT_STRING_BYTEPOS` (2937-2943), the four `move_operation_enum` flags (2977-2993),
  `reset_mouse_highlight` (3026-3041), the seven `tab_bar_item_idx` members (3407-3430), the
  thirteen `tool_bar_item_idx` members (3440-3495), the eight terminal-capability bits
  (3533-3540), and the ten objects of 3608-3616 plus `unspecified_fg`/`unspecified_bg` (3834).
  The C enumerator constants are `pub const`s of C's `int` (PORTING.md 1.3) with C's values, the
  objects are `extern` declarations with the types the units already use (`bool` for
  `redisplaying_p`/`dont_resize_frames`, `c_uint` for `redisplay_counter`, `Lisp_Object` for the
  help-echo group, `[c_char; 0]` for the two "possibly empty" arrays), and the macros keep C's
  bodies.
- **`FACE_FOR_CHAR` (2030-2038) is the one name of the 87 that does *not* sit in the module: it
  is carried in the 3.220 arm, with `struct face`, at its C position.**  C's frozen body is
  `return face->id;`, a read of a `struct face` member, and that record exists in this mirror only
  in that arm.  Its frame parameter is `*mut c_void` because the arm does not carry
  `struct frame` (P-72 ③).  `FACE_SUITABLE_FOR_ASCII_CHAR_P` (2015-2022) needs no member read (its
  frozen body is `return true;`), so it is ungated in the module and spells the same `*mut c_void`;
  both units' typed `*mut face` copies (`src/xfaces.rs` 484/489) stay live.
- **The arm's own compile coverage is a gap this round measured, and it is the integrator's to
  close.**  `xtask`'s `dead_arms()` flips exactly the six rows of its `FLIPS` array, and
  `dispextern_face_records` is not one of them; that feature name appears in the tree only in
  `femacs/xtask/Cargo.toml` 61 and in this mirror, no `spikes/*.sh` names it, and `dead_arms()`
  refuses a row whose config key is absent ("config.rs has no {key} to flip"), so closing it needs
  either a FLIPS row with a real config constant or a `key.is_empty()` skip in that loop.  Until
  then the two records and `FACE_FOR_CHAR` are text no compiler sees - the 86 ungated names of the
  module are compiled by every consumer.
- **`reset_mouse_highlight` (3026-3041) writes C's three `bool_bf : 1` members through the
  mirror's one `bits` byte.**  `struct Mouse_HLInfo` in this mirror packs
  `mouse_face_past_end`/`mouse_face_defer`/`mouse_face_hidden` (C 3019-3023) into `bits` with the
  `MOUSE_FACE_*` masks, so C's three `= false` writes are the mask clear; the three C names are
  bound to the `false`s C assigns them (the `GLYPH_CHAR` "name the value" shape, status 3.228) so
  the body-token rule sees them.  If a `macro-body-notes.tsv` row for those three names and
  `false` is preferred, the three `let`s can go - the mask clear is the whole body either way.
- **`SET_GLYPH_FROM_GLYPH_CODE` (414-420) is the one landed name whose `--check` line does not
  retire yet.**  C's body ends in `while (false)`, and the port's spelling is the `if` the loop
  encloses (the shape `src/dispnew.rs` 245-252 carries), so the gate asks for `false` - exactly
  the `FD_SET`/`FD_CLR` class of `macro-body-notes.tsv`.  The macro itself is landed; the row is
  requested in this round's report.
- **The 15 names still `missing` are `src/xdisp.rs`'s (and `src/font.rs`'s) alone**:
  `BIDI_AT_BASE_LEVEL`, `DEC_TEXT_POS`, `INC_TEXT_POS`, `ITERATOR_AT_END_OF_LINE_P`,
  `IT_STRING_CHARPOS`, `PRODUCE_GLYPHS`, `SET_MARKER_FROM_TEXT_POS`, `COMPOSITION_PROP_IDX`,
  `DISPLAY_PROP_IDX`, `FACE_PROP_IDX`, `FONTIFIED_PROP_IDX`, `INVISIBLE_PROP_IDX`,
  `LAST_PROP_IDX`, `OVERLAY_PROP_IDX`, `help_echo_showing_p`.  They are deferred by this round's
  lane boundary (two in-flight units); the module shape touches no unit file, so landing them
  needs only a go-ahead, not a unit edit.
- **The five records stay for the next round** (`bidi_it`, `composition_it`,
  `draw_fringe_bitmap_params`, `it`, `redisplay_interface`): status 3.220 assigned them the flip
  arm, and each arm needs a row in `femacs/xtask/Cargo.toml` (the integrator's file).  All five
  are unit-owned as well (`src/bidi.rs` 86, `src/dispnew.rs` 6828/6855/3569, `src/indent.rs` 180,
  `src/minibuf.rs` 666).
- **Nine stale boxes were ticked** - `GLYPH_CODE_P`, `CHECK_MATRIX`, `DISPEXTERN_H_INCLUDED`,
  `FACE_ID_BITS`, `IF_DEBUG`, `MAX_FACE_ID`, `SET_CHAR_GLYPH`, `SET_CHAR_GLYPH_FROM_GLYPH`,
  `SET_GLYPH_FROM_CHAR_GLYPH`: every one is in the mirror and `--check` has not asked about it for
  several rounds, so the box (which means "the C item has a Rust counterpart the gate accepts")
  was simply older than the item.
- **The ledger's checklist carries no box for the 57 enumerator constants** this round landed: the
  default inventory lists functions, macros, types and objects, not C's enumerator constants,
  while `--check` does compare them (value included).  They are named in the Evidence below.

## Evidence (the unit-owned names)

- **`--check` before and after**: `python3 tools/extract-abi.py --check emacs/src/dispextern.h
  femacs/src/include/dispextern.rs` -> `grep -c '^- '` went **104 -> 16**; the 16 are the 15
  `missing` names of the deferred list above plus one token line, `macro
  SET_GLYPH_FROM_GLYPH_CODE does not mention false (C body: do { if (CONSP (gc)) SET_GLYPH (glyph,
  XFIXNUM (XCAR (gc)), …)`.  `comm -23`/`comm -13` of the two sorted `- ` lists: **89 lines gone,
  1 line new** (the `missing macro SET_GLYPH_FROM_GLYPH_CODE` line became that token line).
- **The free / unit-declared split this round worked from** (the same scan as the previous
  slice's Evidence, run on the 104 `- missing` names): 87 names in scope, 15 with no owner except
  the two in-flight units.  Owner counts: `indent.rs` 25, `term.rs` 25, `window.rs` 25,
  `keyboard.rs` 21, `xfaces.rs` 16, `frame.rs` 5, `eval.rs` 2, `dispnew.rs` 30, `bidi.rs` 2, and
  the deferred 15 in `xdisp.rs` (48 names total there, 33 of them shared with an in-scope owner).
- **`layout-check`**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-dx cargo run -q -p xtask --
  layout-check` -> `layout probe ok` / `layout-check passed (see build/femacs/layout_asserts.rs)`.
  This is the module's compile evidence: the probe crate `include!`s this mirror, so all 87 items
  were compiled and type-checked in one crate.
- **`build-tu`, twelve consumers** (the first four are the module's new owners, the rest include
  this mirror for other reasons): `src/indent.c` -> `built src/indent.rs -> …/indent.o`;
  `src/term.c` -> `built src/term.rs -> …/term.o`; `src/window.c` -> `built src/window.rs ->
  …/window.o`; `src/keyboard.c` -> `built src/keyboard.rs -> …/keyboard.o`; `src/xfaces.c`,
  `src/frame.c`, `src/eval.c`, `src/dispnew.c`, `src/bidi.c`, `src/character.c`, `src/lread.c`,
  `src/alloc.c` - all `built …`.  (Last round's `src/keyboard.c` reading was against a stale
  `build/` object; the integrator re-measured it as built, and it is built here too.)
- **`xtask dead-arms`** (the flipped-configuration compile of every finished unit):
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-dx cargo run -q -p xtask -- dead-arms` -> exit 0,
  `337 translation unit(s) compiled with the non-frozen arms`, 0 `error` lines.  It does not name
  `dispextern_face_records` (see the FLIPS bullet above), so this is evidence that the module's 86
  ungated names compile in the units, not that the arm does.
- **The five text gates, one run**: `check-declarations` -> `13 declaration sites (0 forbidden, 13
  allowed literals), 0 failures`; `check-globals` -> `283 units, 0 DEFVAR_* variables without a
  definition in their own unit`; `check-mirror-dupes` -> `1 duplicate declaration(s), 1 ratchet
  row(s), 0 failure(s)` (the module adds no finding, which is what the non-glob-imported shape is
  for); `check-verbatim` -> `6 trees, 4607 files, 0 failures`; `check-target-names` -> `626 .rs
  files, 591 map targets, 0 finding(s)`.

## Decisions (the five records, dispextern.h 2159-3200)

- **Shape: the five records are carried in the module, each behind the 3.220 flip feature.**
  They are the same class as the two face records: every one is declared by a unit that includes
  this mirror (`bidi_it` - `src/bidi.rs` 86; `composition_it` - `src/dispnew.rs` 6828 and
  `src/indent.rs` 180; `it` - `src/dispnew.rs` 6855; `redisplay_interface` - `src/dispnew.rs` 3569
  and `src/minibuf.rs` 666; `draw_fringe_bitmap_params` - no unit yet), so an ungated copy is
  `E0428` in those units.  They sit in `dispextern_unit_names` at their C positions, with
  `iterator_stack_entry` hoisted to just before `it` because C defines that record *inside* `it`
  (C 2588-2658) and Rust has no nested record definitions.  The units' typed copies stay live.
- **Every number is the frozen clang's own measurement of the reference header**, and the
  assertions repeat it, so a moved member fails the build rather than the ledger:
  `bidi_it` sizeof 2328 / align 8 (32 member offsets, 3 bits in the byte at 2320);
  `draw_fringe_bitmap_params` 64 / 8 (13 offsets, 2 bits at 52);
  `composition_it` 88 / 8 (14 offsets);
  `iterator_stack_entry` 304 / 8 (20 offsets, 7 bits at 276);
  `it` 4968 / 8 (94 offsets; four bitfield runs: `bits` at 112, C 2477-2485's eight flags;
  `bits2` at 2216, C 2696-2707's thirteen; `bits3` at 2604, C 2915-2932's twelve bytes - the two
  16-bit fringe ids, the two 20-bit face ids and `bidi_p`, which shares the spare bit of their
  third storage unit, measured by setting it alone and reading `2614 = 10` back);
  `redisplay_interface` 104 / 8 (13 offsets);
  `iterator_stack_entry_u` 48 (its `image` arm 48, `slice` at 8 and `image_id` at 40).
  180 measured offsets in all.
- **Three port decisions inside the records.**  (a) A pointer to a record this arm does not carry
  is `*mut c_void` (P-72 ③): `it.w`/`it.f`/`it.dp`/`it.xwidget`, `bidi_it.w` and
  `redisplay_interface`'s twelve parameters, plus `frame_parm_handlers` (C's
  `frame_parm_handler *`); the records this arm *does* carry keep their names (`glyph_row`,
  `glyph`, `run`, `face`, `it_slice`, `text_pos`, `display_pos`, `bidi_saved_info`, `bidi_stack`,
  `bidi_string_data`, `composition_it`, `bidi_it`).  (b) `draw_fringe_bitmap_params`' bitfield
  byte is spelled `flags`, because C's pointer member in that record already owns the name `bits`.
  (c) C's `struct iterator_stack_entry stack[IT_STACK_SIZE]` is spelled `it_stack` - one named head
  entry plus the remaining four, the same 1520 bytes - because C defines the entry type inline, so
  its members are C fields of `it` itself, and the field-member gate follows *named* types only
  (`stack: [iterator_stack_entry; 5]` hides them).
- **`it_stack` and `iterator_stack_entry_u{,_image,_stretch,_xwidget}` are the port's names for
  C's anonymous types** (P-67, the `glyph_u`/`glyph_slice_union` shape): the union is named after
  its member and each arm after the union and its own member.
- **The arm is still compiled by no gate, and this round proves it compiles by hand.**  `xtask`'s
  `dead_arms()` flips exactly its six `FLIPS` rows and `dispextern_face_records` is not one of them;
  a row for it would also need a config constant, because that loop fails on a row whose key is
  absent from `config.rs`.  The probe is the one `xtask` itself uses (a first-reach-wins expansion
  of the include graph, so nested duplicates are include-guard no-ops) over `config.rs` +
  `lisp.rs` (with its own includes) + `character.rs` + `systime.rs` + `dispextern.rs`, compiled with
  `--cfg 'feature="dispextern_face_records"'`.  **It must be compiled from `femacs/`**, the cwd
  `layout_check` uses: from the repository root the same expanded root fails with `c_variadic`
  E0658s in `lisp.rs`/`jni.rs`, which is a probe-invocation artifact, not a defect of the arm.

## Evidence (the five records)

- **The clang measurement**: `clang -std=gnu23 -O0 -I build/emacs-ref/lib -idirafter spikes/shim -I
  build/emacs-ref/src -I emacs/src -I emacs/lib <probe>.c` -> the probe prints
  `sizeof`/`alignof`/`offsetof` for `bidi_it`, `composition_it`, `draw_fringe_bitmap_params`, `it`,
  `iterator_stack_entry` and `redisplay_interface` (180 `offsetof` lines), and a second probe sets
  one bit at a time and reads the byte back: `bidi_it.bits 2320`,
  `iterator_stack_entry.bits 276`, `draw_fringe_bitmap_params.bits 52`, `it.bits 112`,
  `it.bits2 2216`, `it.bits3 2604`, `nonzero byte 2614 = 10` for `bidi_p`, and the fringe run
  `2604:34 12 2606:78 56 2608:de bc 0a 00 2612:45 23 11 00`.
- **`--check`**: `python3 tools/extract-abi.py --check emacs/src/dispextern.h
  femacs/src/include/dispextern.rs` -> `OK emacs/src/dispextern.h -> femacs/src/include/dispextern.rs`,
  `grep -c '^- '` = **0** (from 15 at the start of the round; the three `false`/do-while rows in
  `macro-body-notes.tsv` 1243-1245, which the integrator wrote, retired the last three lines).
- **The arm's own compile** (the flip no gate performs): the `/tmp` probe above, compiled from
  `femacs/` with `--cfg 'feature="dispextern_face_records"'` -> exit 0, `records probe ok`; the
  compile is what evaluates the 180 `const _: () = assert! (…)` lines, so every measured offset
  holds in the port too.
- **`layout-check`** (the arm is off there, so it is the ungated text that is checked):
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-dx cargo run -q -p xtask -- layout-check` ->
  `layout probe ok` / `layout-check passed (see build/femacs/layout_asserts.rs)`.
- **`build-tu`, ten units**: `src/bidi.c` -> `built src/bidi.rs -> …/bidi.o`; `src/dispnew.c`,
  `src/indent.c`, `src/term.c`, `src/window.c`, `src/xfaces.c`, `src/keyboard.c`, `src/frame.c`,
  `src/alloc.c`, `src/character.c` - all `built …`.  (`src/xdisp.c` still fails inside
  `src/xdisp.rs`'s own `buffer_text_bits!` macro, another lane's in-flight work; no diagnostic
  names this mirror.)
- **The ledger**: unchecked boxes **14 -> 1**, and the one that stays is `NativeRectangle`, whose
  deferral is the `excluded-macros.txt` row (3.224) rather than a missing item.

## `struct face`'s window-system pair (status 3.276)

`struct face`'s window-system arm was not carried either: C declares the GC it may draw with and the
stipple id under `#ifdef HAVE_WINDOW_SYSTEM` (dispextern.h 1797-1811), with the GC itself having
three arms (`GC` for X11, `struct android_gc *` for Android, `Emacs_GC *` otherwise).  Both are
carried now under those gates, with the same untyped-pointer ruling the port already uses for
`struct glyph_string`'s GC.

`docs/baseline/face-layout.tsv` (`spikes/gated-layout-probe.py 'struct face' dispextern.h`, P-82)
is the reading: frozen 280/8, x11 296/8, ns_cocoa 296/8, with `gc` at 168 and `stipple` at 176 in
both window-system arms - the sixteen bytes are exactly this pair, and the flag run after them does
not move, so no numbering is gated for this struct.

Evidence: `build-tu src/xfaces.c` and `src/dispnew.c` exit 0; `xtask layout-check` passed.

## Decisions (`struct image`, dispextern.h 3203-3395)

- **The whole record is one gated arm.**  C's second `#ifdef HAVE_WINDOW_SYSTEM` block opens at
  C 3203 and closes at C 3395; `struct image` (C 3215-3362) is inside it, and
  `spikes/gated-layout-probe.py 'struct image' dispextern.h` reads `frozen` as `parse=no` (an
  *incomplete* type there - measured).  The mirror carries the record - and its one bitfield run
  - under `#[cfg (feature = "have_window_system")]`, P-51's feature for the non-frozen side, the
  same arm `struct glyph_string` and `struct face`'s window-system pair use.
- **Layout: measured on both window-system arms, and deliberately not asserted in the mirror.**
  The 3.271 tool's readings are `docs/baseline/image-layout.tsv` (new file, written by
  `--write`): `x11` `sizeof` 256 / `align` 8 and `ns_cocoa` 240 / 8, every member's byte offset.
  A `const _: () = assert! (…)` in this mirror would be an *invented* number: `layout-check`'s
  single crate compiles the frozen configuration, where this record does not exist, and its C
  baseline `docs/baseline/layout/c-layout-darwin-arm64.tsv` has no `image` keys (the recorder is
  `tools/gen-layout-fields.py`, whose `STRUCTS` this type is not in).  Asserting these offsets
  needs a window-system probe crate (the `spikes/ns_inc_probe.rs` / Android-probe shape); the
  readings above are what it would assert.  What replaces the in-tree assertion for now is the
  *reduced probe* in Evidence - the mirror's own `struct image` text compiled with the feature on
  and every measured offset asserted - the same substitute `glyph_string`'s six-member run used.
- **The bitfield run: the NS arm has three members too, and one set of shifts covers both arms.**
  The integrator's count found only two on the NS arm; the tool says otherwise - the NS arm's raw
  reading is `96:0-0`, `96:1-1`, `96:2-2` (`background_transparent`, `background_valid`,
  `background_transparent_valid`), the same three the X arm puts at 112.  It was an extraction
  slip, not a configuration difference, and the mirror spells all three.  `--bits` gives
  `unit/shift` = `1/0`, `1/1`, `1/2` on *both* arms, so one set of `IMAGE_*_SHIFT`/`_MASK`
  constants serves both: the unit's byte offset differs (112 vs 96) only because of the guarded
  members ahead of it, which the declaration reproduces, and the `frame-bits.tsv` lesson - a
  member can move *unit* between configurations - does not apply to this run.  The unit is
  `bits: u32` (one four-byte allocation unit of `bool_bf` bits).
- **Degradations (P-72 rule 3 / P-55).**  Each names C's prototype in a `/// C:` line:
  `Emacs_Pixmap` (C 3222; X11's `Pixmap`, nsgui.h's `id` - both one 8-byte scalar, and
  dispextern.rs declares no `Emacs_Pixmap`, the platform mirrors do), `XImage` (3232),
  `Picture` (3236), `struct android_image` (3244) and `struct image_type` (3345) - that last one
  is this header's own record, in the same gated block, still unmirrored.  `next`/`prev` name
  this record; `struct timespec` (3219) is the existing mirror.
- **One deviation is on record.**  C 3247's `XFORM xform` is a *by-value* member (six `FLOAT`s,
  24 bytes, `<windows.h>`'s `struct tagXFORM`, no mirror declares it).  P-72 rule 3's untyped
  pointer does **not** preserve those 24 bytes; it is used because `have_ntgui` (the MS-W32
  target) is not a target of this lane.  A Windows probe crate has to give the member
  `tagXFORM`'s six floats before it asserts this record's layout.
- **`struct timespec` arrives through `include!("../../lib/include/time.in.rs")`.**  P-72: a
  mirror may only name types it declares or includes.  The include is unconditional, not inside
  the `have_window_system` arm, because `xtask`'s include-graph expansion is textual (it dedupes
  a file's *second* reach to a comment and does not evaluate `cfg`): a cfg'd include could be
  stripped in the frozen build while its dedupe had already eaten another mirror's copy.
- **`glyph_string.img` (C 1483) now names the record** (`*mut image`) instead of the untyped
  pointer it used while this record had no owner mirror - P-72's rule that a degradation is only
  for a type with no mirror.  The two are in the same gated arm, so nothing changes for the
  frozen build.
- **What this landing cannot claim.**  The type does not exist in the frozen configuration, so
  nothing here is compiled or asserted by the frozen gates (`layout-check`, `build-tu`,
  `--check`): its evidence is the gated-arm readings plus the reduced probe.  Four guarded
  combinations (`use_cairo`, `have_x_windows`+`have_xrender`, `have_ntgui`, `have_haiku`) have
  no probe arm at all - the tool's `--define` arm builds on the *frozen* configuration, which
  leaves the record incomplete - so their member positions are unmeasured (the `have_android`
  arm's `ximg`/`mask_img` pair is the X arm's shape by C's own text).

## Evidence (`struct image`)

- **The P-82 readings** (`python3 spikes/gated-layout-probe.py 'struct image' dispextern.h
  --write docs/baseline/image-layout.tsv` -> `wrote docs/baseline/image-layout.tsv: 81 row(s)`,
  the new file): `frozen` `parse=no` (`invalid application of 'sizeof' to an incomplete type
  'struct image'`); `x11` `sizeof 256`, `align 8`, members `0/16/24/32/40/48/56/64/72/80/88/96/
  104/108/112/116/120/124/128/132/136/144/160/168/176/184/188/192/200/208/216/224/232/240/248`;
  `ns_cocoa` `sizeof 240`, `align 8`, members `0/16/24/32/40/48/56/64/72/80/88/92/96/100/104/
  108/112/116/120/128/144/152/160/168/172/176/184/192/200/208/216/224/232`.
- **The `--bits` derivation** (`… 'struct image' dispextern.h --bits`): `x11 1/0, 1/1, 1/2` and
  `ns_cocoa 1/0, 1/1, 1/2` for `background_transparent`, `background_valid`,
  `background_transparent_valid` - one set of shifts, and the NS arm's three rows are what
  answered the count question.
- **The reduced probe** (the mirror's own `struct image` text, cut from
  `femacs/src/include/dispextern.rs`, with `struct timespec` from `lib/include/time.in.rs` and
  the four scalar aliases; `rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib
  -C panic=abort --cfg 'feature="have_window_system"' …` -> **exit 0** with 35 `const _`
  assertions, `sizeof 240`/`align 8` and every `ns_cocoa` offset above; the same probe with
  `--cfg 'feature="have_x_windows"'` -> **exit 0** with 36 assertions, `sizeof 256` and every
  `x11` offset).  This is the only compile the record gets today; it is a `/tmp` probe, not a
  gate.
- **`xtask layout-check`** (the arm is off, so it checks that the ungated text is unchanged):
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-img cargo run -q -p xtask -- layout-check`
  -> `layout probe ok` / `layout-check passed (see build/femacs/layout_asserts.rs)`.
- **`build-tu`**: `src/xfaces.c` -> `built src/xfaces.rs -> ../build/femacs/xfaces.o`;
  `src/dispnew.c` -> `built src/dispnew.rs -> ../build/femacs/dispnew.o`.
- **The other gates**: `python3 tools/extract-abi.py --check emacs/src/dispextern.h
  femacs/src/include/dispextern.rs` -> `OK emacs/src/dispextern.h ->
  femacs/src/include/dispextern.rs` (the type is in the extractor's *deferred* set, so the
  verdict is unchanged) and `grep -c '^- '` still 0; `python3 tools/check-cond-eval.py
  src/dispextern.h` -> `71 conditions, 0 unknown, 0 disagreements [VERIFIED]`;
  `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures` / `read 630 files (623 .rs 7 .swift)`; `python3
  tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`
  (the pre-existing duplicate, not this record).

## Decisions (the window-system coverage pass, status 3.283)

- **`struct redisplay_interface`'s `#ifdef HAVE_WINDOW_SYSTEM` arm (C 3131-3195) lands as the
  fifteen display hooks** (`draw_fringe_bitmap` … `default_font_parameter`), each one pointer
  wide and each behind `#[cfg (feature = "have_window_system")]`, the same arm the record's
  other window-system users take.  The frozen arm's thirteen members, the measured 104/8 and
  every asserted offset are unchanged, because the fields do not exist without the feature.  The
  member types are the mirror's own: `text_cursor_kinds` and `Emacs_Cursor` are declared here,
  `glyph_row`/`glyph_string` are this file's records, and `struct window`/`struct frame`
  parameters keep the `*mut c_void` spelling the thirteen frozen members already use.
- **`struct image`'s `type` member is C's own name.**  The field was spelled `type_` (a Rust
  keyword workaround) and is now `r#type` - the spelling the tree's other `type` members use
  (`bidi_saved_info`, dispextern.rs 1811; `bidi_it`, 2608), and the one
  `spikes/mirror-struct-coverage.py` accepts (`pub (r#)?<name>:`).

## Evidence (the coverage pass)

- **`python3 spikes/mirror-struct-coverage.py`** -> `282 mirrored struct(s): 846 arm(s)
  attempted, 239 unreadable (the configuration does not define the type, or the platform headers
  are absent), 82 gap(s)` - from **114**, and the `## ns_cocoa` section is gone (that arm is at
  0 gaps): this file's `struct image` (1) and `struct redisplay_interface` (15) closed on *both*
  arms, 32 rows.  The 82 that remain are `struct x_display_info` (80) and `struct x_output` (2),
  both `xterm.rs`'s.
- **The gated arm's own compile** (no tree gate turns `have_window_system` on, and `frame.rs`
  fails by design at its first gated accessor - P-82's Rust-side wall): the `redisplay_interface`
  text cut from this mirror, compiled with stubs for the six referenced records,
  `rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib -C panic=abort
  --cfg 'feature="dispextern_face_records"' --cfg 'feature="have_window_system"'` -> **exit 0**,
  with `size_of::<redisplay_interface> () == 224` asserting (13 + 15) pointers.  This is the same
  reduced-probe substitute `struct image` used.
- **The frozen gates**: `xtask layout-check` -> `layout-check passed`; `build-tu src/xfaces.c`,
  `src/dispnew.c`, `src/window.c` -> `built …`; `python3 tools/extract-abi.py --check
  emacs/src/dispextern.h femacs/src/include/dispextern.rs` -> `OK …`.

## Decisions (the "whole type gated" sweep, status 3.278 ④)

**The candidate rule.**  Mirrored struct names were collected from *every* `femacs/**/*.rs` (not
just `src/include`+`lib/include`, which missed e.g. `lib/malloc/include/dynarray.rs`); the C side
is every `struct NAME {` definition that sits inside a `#if`/`#ifdef`/`#ifndef` region in
`emacs/{src,lib,nt,oldXMenu,lwlib,lib-src,exec}/**/*.h` whose name no mirror declares.  Each
survivor was then read with `spikes/gated-layout-probe.py <type> <header>` - the probe's
`incomplete type` *is* the "whole type is gated" reading (3.278 ①), and a type that reads
incomplete in all three arms is gated on a macro no arm of this machine turns on.

**13 unique candidates; three classes.**

*Landed here (this file owns them):*

| type | C | gate | frozen | x11 | ns_cocoa | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| `gui_box` | dispextern.h 205-212 | `HAVE_WINDOW_SYSTEM` (block 182-212) | incomplete | complete(4) | complete(4) | **landed** (`#[cfg (feature = "have_window_system")]`, 4 `int`s at 0/4/8/12, 16/4) |
| `image_cache` | dispextern.h 3368-3385 | `HAVE_WINDOW_SYSTEM` (block 3203-3395) | incomplete | complete(6) | complete(6) | **landed** (6 members at 0/8/16/24/32/40, 48/8) |
| `IMAGE_CACHE_BUCKETS_SIZE` | dispextern.h 3387 | same block | - | - | - | **landed** (`= 1009`, same gate) |

*Landable but another mirror owns the type* (handed over, not touched - the lane rule):

| type | C | gate | frozen | x11 | ns_cocoa | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| `MonitorInfo` | frame.h 1929-1937 | `HAVE_WINDOW_SYSTEM` (+`HAVE_PGTK` adds `scale_factor`) | incomplete | complete(13) | complete(13) | **to `frame.rs`**: `geom` 0, `work` 16, `mm_width` 32, `mm_height` 36, `name` 40, 48/8; consumers are `frame.c`'s `free_monitors`/`make_monitor_attribute_list` and the NS lane's `ns_make_monitor_attribute_list` |

*Gated on a macro none of the three arms sets - unreadable everywhere* (no reading is possible
until an arm exists; both headers are dead-by-construction on this target and both mirrors already
record that):

| type | C | gate | frozen | x11 | ns_cocoa | verdict |
| --- | --- | --- | --- | --- | --- | --- |
| `pgtk_bitmap_record` | pgtkterm.h 43-49 | `HAVE_PGTK` | incomplete | incomplete | incomplete | deferred (no PGTK arm) |
| `pgtk_device_t` | pgtkterm.h 51-64 | `HAVE_PGTK` | incomplete | incomplete | incomplete | deferred |
| `pgtk_touch_point` | pgtkterm.h 66-83 | `HAVE_PGTK` | incomplete | incomplete | incomplete | deferred |
| `pgtk_display_info` | pgtkterm.h 146-281 | `HAVE_PGTK` | incomplete | incomplete | incomplete | deferred |
| `pgtk_output` | pgtkterm.h 286-455 | `HAVE_PGTK` | incomplete | incomplete | incomplete | deferred |
| `Lisp_TS_Parser` | treesit.h 35 | `HAVE_TREE_SITTER` | incomplete | incomplete | incomplete | deferred |
| `Lisp_TS_Node` | treesit.h 128 | `HAVE_TREE_SITTER` | incomplete | incomplete | incomplete | deferred |

*Not candidates after inspection:*

- `int_array` / `str_array` (lib/dynarray.h 197/234) are **inside a `/* … */` documentation
  comment**, not definitions at all; the probe reads them incomplete for that reason.  (The
  scanner that produced the candidate list does not strip comments - a limit worth knowing.)
- `__time_t_must_be_integral` (lib/time.in.h 113) reads `complete(1)` in all three arms and is
  already carried *as a marker* by `lib/include/time.in.rs` (`GNULIB_defined_struct_time_t_must_be_integral`
  plus the comment: its only member is a one-bit bitfield, and `time_t = i64` satisfies it by
  construction).
- `Lisp_TS_Query` (treesit.h 155) is declared by a **unit**, `femacs/src/pdumper.rs` 2527, so the
  mirror owes nothing.

**`struct pgtk_output`'s verdict: defer, with a reason, not silently.**  Three facts together:
its three arms are *all* unreadable (so no reading exists to write a mirror against); its pointees
are GTK's (`GdkWindow`, `GdkCursor`, `cairo_pattern_t`, `GdkEvent*`, `GtkWidget`), which no mirror
in this tree declares, so the record cannot be typed from what the port has; and it has **no
consumer** - `src/pgtkterm.c` is not in the frozen build (`docs/port-map.md` 511-512) and no ported
unit names any PGTK type.  The same three facts hold for the other four pgtk records above, and
`docs/ports/include-pgtkterm.md` already records the whole `#ifdef HAVE_PGTK` body as
dead-by-construction.  **Deferred until a PGTK target is frozen** (then the mirror, the GTK
pointee mirrors and the arm all arrive together).  The two `treesit.h` records are the same case
with `include-treesit.md` recording it.

## Evidence (the whole-type-gated sweep)

- **The scan**: candidate rule above; 13 unique types survive (14 rows - lib/time.in.h lists the
  same `__time_t_must_be_integral` twice because the scanner walks direct and generated listings).
  Every row's three-arm reading is in the tables above; `frozen=incomplete` for 12 of the 13
  (`__time_t_must_be_integral` is the complete-everywhere exception).
- **`spikes/mirror-struct-coverage.py`** -> `284 mirrored struct(s): 1136 arm(s) attempted, 320
  unreadable …, 5 gap(s)`, and the 5 are `x_display_info`'s in the new **`x11_ext`** arm
  (`xsync_*`/`server_time_monotonic_p` - `xterm.rs`'s, not this file's).  The two records landed
  here are now *mirrored* rows (282 -> 284) with 0 gaps of their own, and the `ns_cocoa` section is
  absent (0 gaps).
- **The gated arm's own compile** (no tree gate turns `have_window_system` on): `struct gui_box`,
  `struct image_cache` and the constant cut from this mirror, plus the `struct image` text and
  `lib/include/time.in.rs`, compiled with `rustup run nightly-2026-10-01 rustc --edition 2024
  --crate-type lib -C panic=abort --cfg 'feature="have_window_system"'` -> **exit 0** with
  `size_of::<gui_box> () == 16`/`align 4`/offsets 0/4/8/12, `size_of::<image_cache> () == 48`/
  `align 8`/offsets 0/8/16/24/32/40 and `IMAGE_CACHE_BUCKETS_SIZE == 1009` asserted; the same probe
  with `--cfg 'feature="have_x_windows"'` -> exit 0.
- **The frozen gates**: `xtask layout-check` -> `layout-check passed (see
  build/femacs/layout_asserts.rs)`; `build-tu src/xfaces.c` / `src/dispnew.c` / `src/window.c` ->
  `built …`; `python3 tools/extract-abi.py --check emacs/src/dispextern.h
  femacs/src/include/dispextern.rs` -> `OK …` (the three names are in the extractor's deferred
  set); `check-cond-eval.py src/dispextern.h` -> `71 conditions, 0 unknown, 0 disagreements`;
  `check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed literals), 0
  failures`; `check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0
  failure(s)`.
- **One shared-table row needs the integrator**: `docs/baseline/citations-allowlist.tsv` line 37
  keys `femacs/src/include/dispextern.rs:544`; this batch's `gui_box` insertion moved that
  citation to line **567**, so `check-citations` now reports the same finding twice - a `drift` at
  `:567` and a `stale` allowlist row at `:544`.  Retargeting the row's line number (544 -> 567)
  is a one-field edit in a shared table, so it is reported rather than made here.

# ----------------------------------------------------------------------
## Decision and Evidence (`Emacs_Cursor` vs `androidgui.h` 42, 2026-10-05)

- **The arm that is dead under a window system carries C's own condition.**  C declares the name
  twice on this header's side: `typedef Cursor Emacs_Cursor;` at 39 under `HAVE_X_WINDOWS`
  (27-98), and `typedef void *Emacs_Cursor;` at 175 under `#ifndef HAVE_WINDOW_SYSTEM`
  (174-176).  `androidgui.h` 42 (`typedef android_handle android_cursor, Emacs_Cursor;`)
  declares it on the window-system side of the same `#if` pair, and `androidterm.h` 23/26
  includes both headers, so an Android unit's graph holds both mirrors and rustc reports
  `E0428: the name Emacs_Cursor is defined multiple times` (this mirror's 82 vs
  `androidgui.rs` 53) - the whole of 3.370's blocker for the arm-flip compile.  The arm at 175
  is the one C turns off, so the gate is here:
  `#[cfg (all (not (feature = "have_window_system"), not (feature = "have_android")))]`.
  The `have_android` term is not redundant: `HAVE_ANDROID` implies `HAVE_WINDOW_SYSTEM` in C,
  and the arm-flip compile sets only `have_android`, so without it the frozen arm would stay
  live next to `androidgui.rs`'s alias and the collision would survive the "fix".
- **The declaration keeps the two lines it had** (one comment line, one item, the C text as the
  item's trailing comment), so every citation-bearing line below it stays put -
  `docs/baseline/citations-allowlist.tsv` keys `dispextern.rs:587` by line, and the 3.283
  precedent (that file's own last Evidence bullet) shows what an insertion above such a row
  costs: a shared-table retarget for the integrator.  Nothing here needs one.
- **`struct redisplay_interface.define_frame_cursor` (3542, C 3150)** is under
  `have_window_system` and takes the name from the platform GUI mirror there, exactly as C does
  (dispextern.h 164 includes `androidgui.h` for that arm); no configuration in
  `docs/baseline/config/` enables the feature, and the Android side of the pair is what the
  arm-flip below compiles.

## Evidence (`Emacs_Cursor`)

- **Before** - `rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj
  -C panic=abort --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'
  -o /tmp/am.o build/femacs/units/androidmenu/_root.rs` -> exit 1, **11 `^error` lines** (10
  errors + `aborting due to 10 previous errors`), the `E0428` at this mirror's 82 plus 9 more,
  all in included mirror copies, none in the unit's own text.
- **After** - the same command -> **exit 0, 0 `^error` lines**, `/tmp/am.o` **89,808 bytes**
  (the unit's code is compiled, where the frozen object below is 336 bytes of nothing).
- **Frozen** - `cd femacs && CARGO_TARGET_DIR=/tmp/andfix cargo run -q -p xtask -- build-tu
  src/androidmenu.c` -> `built src/androidmenu.rs -> ../build/femacs/androidmenu.o`, 336 bytes;
  that compile *is* the no-`have_android` combination, and it is unchanged.
- `python3 tools/extract-abi.py --check emacs/src/dispextern.h
  femacs/src/include/dispextern.rs` -> `OK emacs/src/dispextern.h ->
  femacs/src/include/dispextern.rs`.
- `python3 tools/check-citations.py` -> `675 file(s) scanned, 0 drift, 0 range, 351 external,
  33 allowed, 0 stale`.

# ----------------------------------------------------------------------
## The window-system half of the typedefs (2026-10-05)

**What landed.**  `dispextern.h` 113-198's per-platform types, each with C's own condition (and
`have_window_system` on top of the platform's, so a unit that sets a platform feature without the
window system - `xtask`'s own `have_haiku` flip is exactly that shape - reaches no record it may
not have; P-87 3):

1. **C 143/150/158's `#include`s of the platform `gui.h` headers** (`nsgui.h`, `pgtkgui.h`,
   `haikugui.h`), carried as `include!`s under their platform feature **plus**
   `have_window_system`.  This is the repair for the 18 `E0425 cannot find type Emacs_Cursor` the
   NS configuration reported: C reaches `nsgui.h` 64's `typedef void *Emacs_Cursor` (and NS's
   `Window`, `Emacs_Pixmap`) *through this header*, so the mirror does too - no unit has to
   invent the names.  **Two of C 135/143/150/158/164's five edges are deliberately not carried**,
   each with a measurement behind it: `w32gui.h` (C 135) and `androidgui.h` (C 164).  Both bring
   more than themselves into `xtask`'s materializer, whose nested-`include!` dedupe is by *file*
   and ignores the `#[cfg]` on the edge that first reaches it:
   * `androidgui.rs` includes `lib/include/wchar.rs`; with the edge here, `build-tu src/sysdep.c`
     read `// [xtask dedupe] …/lib/include/wchar.rs already included` at the top level and the
     only `wchar.rs` copy left sat inside the `have_android` arm - `wint_t` vanished and
     `check-all-units.sh` read `83 unit(s), 1 failure(s)`.  (The Android units reach the file
     through C's own edge anyway: `androidterm.h` 23 → `include/androidterm.rs` 41.)
   * `w32gui.rs` is inside `layout-check`'s compiled set (not one of the ten platform-exclusive
     mirrors that probe skips), and the same dedupe moved its one inclusion into a `have_ntgui`
     arm, so `include/msdos.rs` 349-362 - which spells `Display`/`Window` ungated and was
     borrowing both from there - failed: six `E0425 cannot find type Display` + one `Window`,
     `error: layout probe failed to compile`.  At HEAD the same probe passes, so this was a
     regression, and the W32 units lose nothing: C's own edge `w32term.h` 21 is already in
     `include/w32term.rs` 49.
2. **The six `Display_Info` arms** (C 115/136/145/152/159/165): x11, w32, NS, haiku and android
   alias the term mirror's record (`xterm.rs` 355, `w32term.rs` 154, `nsterm.rs` 318,
   `haikuterm.rs` 89, `androidterm.rs` 86) - the same relation C has, where the complete record
   arrives from `TERM_HEADER` (`frame.h` 31-33).  `pgtk_display_info` is **not** here:
   `include/pgtkterm.rs` has no record yet and no arm on this machine can read one (no GTK
   headers - the same measurement that defers the five pgtk records under 3.278 4).
3. **`Emacs_Pix_Container`/`Emacs_Pix_Context`/`Emacs_Pixmap`**: C 122-131's `USE_CAIRO` arm
   (the anonymous record, named `emacs_pix_container`, with the four layout assertions the
   `image.md` clang reading gives: sizeof 24, align 8, `data` 8, `bytes_per_line` 16,
   `bits_per_pixel` 20), C 146's NS arm, C 160's haiku arm, C 166's android arm, and C 196-197's
   default `Emacs_Pix_Context`/`PIX_CONTAINER_TO_CONTEXT` pair.  **Deferred with a reason**: C
   116-118's X11 arm needs `XImage` and C 41's `Emacs_Pixmap` needs `Pixmap`, which
   `include/xlib.rs` does not carry yet (its two names are part of the X11 round the x+cairo flip
   measures - 230 error lines, 124 of them `Atom`); C 137-139's `HAVE_NTGUI` arm is the
   unmeasured Windows arm (see `docs/ports/include-frame.md`'s section of the same date).
4. **C 184-190's `WINDOW_HANDLE_UINTPTR` default is deliberately not spelled**: it is a macro and
   `haikugui.rs` 53 / `pgtkgui.rs` 116 already own the name with the same cast, so a second
   `macro_rules!` of it is `E0659` ambiguity in `layout-check`'s single crate - the reason this
   file already gives for `NativeRectangle` above.

**Not landed, and why (each a named item).**

- `check_x_display_info` (C 3993): `femacs/src/frame.rs` 6155-6156 declares it at its crate root,
  so a declaration here makes `tools/check-mirror-dupes.py` fail (`FAIL femacs/src/frame.rs:
  check_x_display_info is already declared by femacs/src/include/dispextern.rs (in the include
  graph)` - measured).  It lands with the commit that deletes that copy and adds the ratchet row;
  `docs/ports/w32fns.md` owed 9 stays owed.
- C 169-172's `#include "fontset.h"`: `femacs/src/font.rs` 362/448/450/5027 declares four of
  `fontset.rs`'s names at its own crate root (`face_for_char`, `fs_query_fontset`,
  `fontset_ascii`, `font_for_char`), so the edge would be four `E0428`s in that unit until they
  retire - and the units that need the names already carry the include themselves
  (`src/haikuterm.rs` 75 does exactly that, citing this line).  The edge lands with those
  copies' commit.

**Evidence.**

- `python3 tools/extract-abi.py --check emacs/src/dispextern.h femacs/src/include/dispextern.rs`
  -> `OK emacs/src/dispextern.h -> femacs/src/include/dispextern.rs`.
- `python3 tools/check-mirror-dupes.py` -> `18 duplicate declaration(s), 18 ratchet row(s),
  0 failure(s)`.
- `python3 tools/check-citations.py` -> `0 drift`, `0 stale` (`docs/baseline/
  citations-allowlist.tsv` 33's key at `dispextern.rs:587` is **unmoved**: the new block was
  inserted below it, so no shared-table retarget is owed for it).
- The flip: `src/image.rs` with `--cfg 'feature="have_window_system"' --cfg 'feature="have_ns"'`
  -> `55` `error` lines before, `11` after; the 18 `Emacs_Cursor` errors are gone and what remains
  is the unit's own text plus four `E0428`s from copies that must retire (listed in
  `docs/ports/include-frame.md`'s section).  It is a **name-resolution reading**: `E0428` stops
  rustc before type-checking, so `include/font.rs`'s eight `E0080`s are hidden behind it, not
  fixed (P-86 7).
- Frozen units (`build-tu`, `CARGO_TARGET_DIR=/tmp/mirror-lane/cargo`): `src/dispnew.c`,
  `src/window.c`, `src/xfaces.c`, `src/xdisp.c`, `src/editfns.c`, `src/lread.c` -> `built …`.
- `xtask layout-check` -> `layout-check passed (see build/femacs/layout_asserts.rs)` (this is also
  the gate that caught the `w32gui.rs` edge above; the first version of this block failed it).
- `build-tu src/sysdep.c`, `src/syntax.c`, `src/frame.c`, `src/window.c`, `src/term.c` ->
  `built …` (the first two are the `<wchar.h>` users the `androidgui.rs` edge broke).
- **`tools/check-all-units.sh` (P-87, both legs), the batch's final revision**:
  `checked 83 unit(s), 0 failure(s)` / `checking 83 unit(s), 0 unit failure(s)`.  One run covers
  every mirror this lane touched.  It is the *main working tree* reading, so it contains the other
  lanes' in-flight units (`xdisp.rs`, `w32term.rs`, `image.rs` are modified in the tree); the
  recorded HEAD reading is the same 83/0 (`docs/status.md` 3.402), and the batch's one intermediate
  run - `83 unit(s), 1 failure(s)` on `src/sysdep.c` - was this lane's own `androidgui.rs` edge,
  fixed and re-measured (the failure is named in the block above).

# ----------------------------------------------------------------------
## The bit-field accessors, and the four prototypes that stay owed (2026-10-05, `docs/ports/w32term.md` owed 5 and 10)

**The accessors.**  C stores two `bool_bf : 1` members in
`struct draw_fringe_bitmap_params` (C 2230-2231, `cursor_p` at bit 0 and `overlay_p` at bit 1 of
the byte at offset 52) and three in `Mouse_HLInfo` (C 3019-3023, `mouse_face_past_end`/
`mouse_face_defer`/`mouse_face_hidden`); the mirror records the run as one `flags`/`bits` byte
(PORTING.md 1.3) with the masks already beside it, and now carries the accessor pair C's member
access stands for: `get_cursor_p`/`set_cursor_p`, `get_overlay_p`/`set_overlay_p` (with the two
`DRAW_FRINGE_BITMAP_*` masks) and `get_mouse_face_defer`/`set_mouse_face_defer`.  The bit numbers
are the measured ones (`abi_shim.rs` 2185-2205's offset probe).

**Two of the five missing prototypes land.**  `get_glyph_string_clip_rect` (C 3663) and
`gui_intersect_rectangles` (C 3676) are now in `dispextern_late_decls`, gated
`have_window_system` - C's own guard for both (C 3660-3674) and the feature `struct glyph_string`
itself carries; `NativeRectangle` is a type macro this mirror deliberately does not spell, so the
out parameter is the untyped pointer its call sites pass (`row_containing_pos`'s spelling).
`gui_define_fringe_bitmap` (C 3711) needed nothing further: its definition already exists in this
tree (`abi_shim.rs` 2240).

**Four stay owed, measured.**  C 3688's `note_mouse_highlight`, C 3711's
`gui_define_fringe_bitmap`, C 3719's `buffer_flipping_blocked_p` and C 3813's
`lookup_basic_face` are each declared at the crate root of at least one ported unit, and
`tools/check-mirror-dupes.py` counts a declaration inside this glob-imported module as the same
construct: adding all four produced **eleven new `FAIL` lines** (`androidterm` ×
`buffer_flipping_blocked_p`/`note_mouse_highlight`, `dispnew` × `lookup_basic_face`/
`note_mouse_highlight`, `font` × `lookup_basic_face`, `frame` × `note_mouse_highlight`,
`haikuterm` × `buffer_flipping_blocked_p`, `w32term` × `buffer_flipping_blocked_p`/
`gui_define_fringe_bitmap`/`lookup_basic_face`, `xdisp` × `lookup_basic_face`).  They land in
the commit that deletes those copies and adds the ratchet rows
`docs/baseline/mirror-dupes.tsv` needs - both outside this lane's writable set.  Recorded in the
mirror at the four names' position.

**Evidence.**  `extract-abi --check emacs/src/dispextern.h femacs/src/include/dispextern.rs` ->
`OK`; `xtask layout-check` passed (the two new prototypes' `have_window_system` gate is what keeps
`struct glyph_string` from being named in a crate that has not got it - the ungated first draft
read `E0425: cannot find type glyph_string` in the `have_ntgui` probe); `check-citations` 0 drift;
`check-declarations` 13 sites / 0 forbidden; `check-mirror-dupes` 18 ratchet rows / 0 failure;
`check-all-units.sh` 83/0 both legs.  `check_x_display_info` (C 3993) stays **not** declared: the
note at its position records the same measurement for it (`FAIL femacs/src/frame.rs`).

### status 3.496 (the mirror round this file took part in)

- The edge/declaration changes are recorded in `docs/status.md` 3.496: `include/dispextern.rs`'s
  `include!("xlib.rs")` edge lost its `#[cfg]` (the `xtask` dedupe gives the first edge the slot),
  `include/xlib.rs` widened `XRectangle`/`Window`/`XrmDatabase`/`XID` to
  `any (have_x_windows, have_pgtk)`, `include/pgtkterm.rs` fixed a dangling `///` (a syntax error
  that made every `have_pgtk` TU unparsable), its `pgtk_im_set_cursor_location` prototype and its
  two bare `atimer` tags, `include/font.rs` gained the `harfbuzz` arm of the `font_driver` size
  assertion (clang-measured 208/224), `include/xterm.rs`'s three INLINE bodies became `unsafe fn`,
  `include/gtk.rs` gained `GtkWidgetClass.get_preferred_height`, and `include/emacsgtkfixed.rs`
  moved `emacs_fixed_new` into a `_decls` module.  The two arm flips that were blocked now read
  **exit 0**: `src/pgtkim.rs` (`have_pgtk` + `have_gtk3`, 14,083,208-byte rlib) and
  `src/hbfont.rs` (`harfbuzz`, 14,580,952-byte rlib); `layout-check` passes and
  `tools/check-all-units.sh` is **83/0 + 83/0**.

## 2026-10-05: the mirror's share of the window-system arm (3.505)

Two items the arms had been reading as *this* file's errors are fixed here, both measured on
`src/fringe.rs`'s flip (`--cfg feature="have_window_system" --cfg feature="have_x_windows"
--cfg feature="dispextern_face_records"`, the arm 3.501 used):

1. **The three globals left the top level** (`last_tab_bar_item`, `last_tool_bar_item`,
   `max_used_fringe_bitmap`, C 3617/3618/3710) and joined `dispextern_unit_names`.  A unit that
   includes this mirror *defines* them (`src/fringe.c` 497, `keyboard.c`), and a top-level
   `extern` copy is `E0428` in exactly that unit: 3.501 read `1 x E0428
   max_used_fringe_bitmap` in `src/fringe.rs`'s flip, and the same shape would hit `abi_shim.rs`,
   which also defines one.  The module already carried the 102 names of this class; these three
   were the ones the file's own comment at 2661 had left outside it.
2. **The `struct face` layout assertions are arm-aware.**  C 1797-1811's two
   `HAVE_WINDOW_SYSTEM` members (`gc`, `stipple`) sit between `id` and `foreground`, so every
   member from `foreground` on is sixteen bytes further along in a window-system build -
   `docs/baseline/face-layout.tsv` already holds both readings (`frozen` 280/8 with
   `foreground`@168; `x11`/`ns_cocoa`/`x11_ext` 296/8 with `foreground`@184).  The assertions now
   add `FACE_WS_SHIFT` (0 in the frozen configuration, 16 with `have_window_system`), so one set
   of measured numbers covers every arm, and the two members have assertions of their own
   (168/176, measured in both window-system arms).  The arm read `13 x E0080` on the frozen
   values before this.

Readings, same command, before -> after: `38 -> 13` errors overall, of which *this* file's share
falls `29 -> 4` (1 x `E0428` and 13 x `E0080` gone; the 3 `E0425` for `estimate_mode_line_height`
were never this file's - the decl belongs to the calling unit, see `docs/ports/fringe.md`).
`layout-check` still passes (the frozen arm reads the same numbers as before: the shift is 0
there), `check-all-units` is 83/0, `check-mirror-dupes` 17/17/0.

What is **left, and why** (the four `E0425` in the flip - `x_display_info`, `Emacs_Pix_Container`,
`Emacs_Cursor` x2): the X11 arms of C 39/116-118 are deferred in this mirror with a written
reason (see the comment above `Display_Info`), and their precondition has since been met
(`include/xlib.rs` now has `Cursor` at 862 and `XImage` around 1000).  What they need is a ruling
on where the *tag* `x_display_info` lives: at the mirror's top level it collides with
`include/xterm.rs`'s own record in a unit that includes both (`E0428`), and inside the module
`pub type Display_Info = x_display_info;` cannot see it.  That ruling is the next mirror round's
first item, not a defect of the arms.

## 2026-10-05: the X11 typedefs land and `redisplay_interface`'s arm is read (3.507)

Same flip (`src/fringe.rs`, `have_window_system` + `have_x_windows` + `dispextern_face_records`),
three items:

1. **The four `E0425` this file was carrying are gone.**  `Emacs_Cursor` (C 37/39, inside
   C 27-45's `#ifdef HAVE_X_WINDOWS`) is `Cursor` - `include/xlib.rs` 862 - and
   `Emacs_Pix_Container` (C 116-118, `#ifndef USE_CAIRO`) is `*mut XImage` (that mirror's
   356-390 record, measured 136/8).  The deferral comment above `Display_Info` had outlived its
   reason: it named `include/xlib.rs`'s missing `XImage`/`Cursor` and both have been there since
   3.500.
2. **The `x_display_info` tag lives in the module.**  C 115's `typedef struct x_display_info
   Display_Info;` declares the *tag* and C's record is `xterm.h`'s, so the port names an opaque
   type (`dispextern_unit_names::x_display_info`) and aliases it; a top-level tag would be
   `E0428` in every unit that also includes `include/xterm.rs` (the record's real definition),
   which is the same reason the 102 names of this class live there.  A unit that needs the
   complete record declares its own alias - `src/frame.rs` 6059 and `src/androidfns.rs` 418
   already do.
3. **`struct redisplay_interface`'s arm reading.**  C 3131-3195's fifteen `HAVE_WINDOW_SYSTEM`
   hooks (already carried behind `have_window_system`) are appended after `fix_overlapping_area`,
   so they add 120 bytes and shift nothing: `docs/baseline/redisplay_interface-layout.tsv`
   (`frozen` 104/8, `x11`/`ns_cocoa`/`x11_ext` 224/8, the fifteen at 104 … 216).  The size
   assertion adds `REDISPLAY_IFACE_WS_SHIFT` (0/120) and the fifteen members have arm-only
   assertions at their measured offsets.

Readings, same flip: `38 -> 24 -> 13 -> 5 -> 2 -> 1` errors over 3.505/3.506/3.507.  **The one
left is not this file's**: `include/systime.rs` 27 asserts that `have_x_windows` and
`config.rs`'s `HAVE_X_WINDOWS` constant move together, which is exactly what `xtask dead-arms`
does and a bare `--cfg` does not (status 3.434's reading) - the faithful arm for that class is
`dead-arms`, not a change to `systime.rs`.
