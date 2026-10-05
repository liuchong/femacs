# emacs/src/xdisp.c (39805 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 13
- [ ] set-buffer-redisplay  (Fset_buffer_redisplay, 4/4, doc 5c281f24209d, line 922)
- [ ] line-pixel-height  (Fline_pixel_height, 0/0, doc a1c4fe180f27, line 1545)
- [ ] get-display-property  (Fget_display_property, 2/4, doc ebee6e28fe94, line 5877)
- [x] window-text-pixel-size  (Fwindow_text_pixel_size, 0/7, doc 24dfcb0ed7db, line 12078)
- [x] buffer-text-pixel-size  (Fbuffer_text_pixel_size, 0/6, doc 8ad2cd85d9f9, line 12158)
- [x] display--line-is-continued-p  (Fdisplay__line_is_continued_p, 0/0, doc 198c6180dfb2, line 12216)
- [x] long-line-optimizations-p  (Flong_line_optimizations_p, 0/0, doc 9778916a82c2, line 20414)
- [x] current-bidi-paragraph-direction  (Fcurrent_bidi_paragraph_direction, 0/1, doc 2adae0649377, line 26937)
- [x] bidi-find-overridden-directionality  (Fbidi_find_overridden_directionality, 3/4, doc 6e06604b8c32, line 27030)
- [x] move-point-visually  (Fmove_point_visually, 1/1, doc 3a8d3658283b, line 27181)
- [x] bidi-resolved-levels  (Fbidi_resolved_levels, 0/1, doc e680f6eaa573, line 27637)
- [ ] format-mode-line  (Fformat_mode_line, 1/4, doc 74f1a4c3b665, line 29063)
- [ ] invisible-p  (Finvisible_p, 1/1, doc babc2e0a077e, line 30349)

## Functions defined — 302
- [x] it_char_has_category  (line 557, static)
- [x] char_can_wrap_before  (line 577, static)
- [x] char_can_wrap_after  (line 599, static)
- [x] fill_column_indicator_column  (line 628, static)
- [x] redisplay_other_windows  (line 863)
- [x] wset_redisplay  (line 870)
- [x] fset_redisplay  (line 879)
- [x] bset_redisplay  (line 886)
- [x] bset_update_mode_line  (line 902)
- [x] wset_update_mode_line  (line 910)
- [x] redisplay_trace  (line 944, static)
- [x] move_trace  (line 962, static)
- [x] window_text_bottom_y  (line 1260)
- [x] window_box_width  (line 1279)
- [x] window_box_height  (line 1307)
- [x] window_box_left_offset  (line 1385)
- [x] window_box_right_offset  (line 1418, static)
- [x] window_box_left  (line 1430)
- [x] window_box_right  (line 1450)
- [x] window_box  (line 1463)
- [x] line_bottom_y  (line 1511)
- [x] default_line_pixel_height  (line 1595)
- [x] string_from_display_spec  (line 1629, static)
- [x] window_hscroll_limited  (line 1651, static)
- [x] reset_box_start_end_flags  (line 1668, static)
- [x] pos_visible_p  (line 1693)
- [x] check_char_and_length  (line 2159, static)
- [x] string_pos_nchars_ahead  (line 2177, static)
- [x] string_pos  (line 2204, static)
- [x] c_string_pos  (line 2219, static)
- [x] number_of_chars  (line 2248, static)
- [x] compute_string_pos  (line 2275, static)
- [x] estimate_mode_line_height  (line 2292)
- [x] pixel_to_glyph_coords  (line 2326)
- [x] x_y_to_hpos_vpos  (line 2380, static)
- [x] frame_to_window_pixel_xy  (line 2471, static)
- [x] adjust_window_ends  (line 3064, static)
- [x] hscrolling_current_line_p  (line 3074, static)
- [x] dsafe_eval_handler  (line 3099, static)
- [x] dsafe__call  (line 3113, static)
- [x] funcall_with_backtraces  (line 3145, static)
- [x] dsafe_call1  (line 3164, static)
- [x] dsafe_eval  (line 3170, static)
- [x] CHECK_IT  (line 3184, static)
- [x] CHECK_WINDOW_END  (line 3214, static)
- [x] init_iterator  (line 3253)
- [x] get_narrowed_width  (line 3717, static)
- [x] get_narrowed_len  (line 3732, static)
- [x] get_medium_narrowing_begv  (line 3739, static)
- [x] get_medium_narrowing_zv  (line 3746, static)
- [x] get_nearby_bol_pos  (line 3758, static)
- [x] get_small_narrowing_begv  (line 3786)
- [x] get_large_narrowing_begv  (line 3794)
- [x] get_large_narrowing_zv  (line 3812)
- [x] unwind_narrowed_begv  (line 3821, static)
- [x] start_display  (line 3846)
- [x] in_ellipses_for_invisible_text_p  (line 3951, static)
- [x] init_from_display_pos  (line 3985, static)
- [x] init_to_row_start  (line 4132, static)
- [x] init_to_row_end  (line 4148, static)
- [x] handle_stop  (line 4189, static)
- [x] compute_stop_pos  (line 4330, static)
- [x] compute_display_string_pos  (line 4508)
- [x] compute_display_string_end  (line 4605)
- [x] handle_fontified_prop  (line 4667, static)
- [x] face_at_pos  (line 4799, static)
- [x] handle_face_prop  (line 4910, static)
- [x] underlying_face_id  (line 4969, static)
- [x] face_before_or_after_it_pos  (line 4989, static)
- [x] handle_invisible_prop  (line 5206, static)
- [x] setup_for_ellipsis  (line 5654, static)
- [x] find_display_property  (line 5696, static)
- [x] get_display_property  (line 5761, static)
- [x] display_min_width  (line 5773, static)
- [x] handle_display_prop  (line 5911, static)
- [x] handle_display_spec  (line 5999, static)
- [x] display_prop_end  (line 6078, static)
- [x] handle_single_display_spec  (line 6121, static)
- [x] display_prop_intangible_p  (line 6658)
- [x] single_display_spec_string_p  (line 6680, static)
- [x] display_prop_string_p  (line 6724, static)
- [x] string_buffer_position_lim  (line 6763, static)
- [x] string_buffer_position  (line 6811, static)
- [x] handle_composition_prop  (line 6837, static)
- [x] handle_overlay_change  (line 6918, static)
- [x] next_overlay_string  (line 6935, static)
- [x] compare_overlay_entries  (line 7048, static)
- [x] load_overlay_strings  (line 7104, static)
- [x] get_overlay_strings_1  (line 7212, static)
- [x] get_overlay_strings  (line 7301, static)
- [x] push_it  (line 7327, static)
- [x] iterate_out_of_display_property  (line 7389, static)
- [x] restore_face_box_flags  (line 7427, static)
- [x] pop_it  (line 7450, static)
- [x] back_to_previous_line_start  (line 7573, static)
- [x] strings_with_newlines  (line 7588, static)
- [x] forward_to_next_line_start  (line 7662, static)
- [x] back_to_previous_visible_line_start  (line 7808, static)
- [x] reseat_at_previous_visible_line_start  (line 7897)
- [x] reseat_at_next_visible_line_start  (line 7913, static)
- [x] reseat  (line 7994, static)
- [x] reseat_1  (line 8068, static)
- [x] reseat_to_string  (line 8138, static)
- [x] lookup_glyphless_char_display  (line 8318)
- [x] merge_escape_glyph_face  (line 8377, static)
- [x] merge_glyphless_glyph_face  (line 8402)
- [x] forget_escape_and_glyphless_faces  (line 8424)
- [x] get_next_display_element  (line 8437, static)
- [x] set_iterator_to_next  (line 8896, static)
- [x] next_element_from_display_vector  (line 9258, static)
- [x] get_visually_first_element  (line 9351, static)
- [x] next_element_from_string  (line 9463, static)
- [x] next_element_from_c_string  (line 9629, static)
- [x] next_element_from_ellipsis  (line 9676, static)
- [x] next_element_from_image  (line 9703, static)
- [x] next_element_from_xwidget  (line 9710, static)
- [x] next_element_from_stretch  (line 9722, static)
- [x] compute_stop_pos_backwards  (line 9734, static)
- [x] handle_stop_backwards  (line 9781, static)
- [x] next_element_from_buffer  (line 9830, static)
- [x] next_element_from_composition  (line 10019, static)
- [x] move_it_in_display_line_to  (line 10109, static)
- [x] move_it_in_display_line  (line 10881)
- [x] move_it_to  (line 10928)
- [x] move_it_vertically_backward  (line 11273)
- [x] move_it_vertically  (line 11460)
- [x] move_it_past_eol  (line 11484)
- [x] move_it_by_lines  (line 11503)
- [x] partial_line_height  (line 11648)
- [x] fast_move_it_horizontally  (line 11694, static)
- [x] in_display_vector_p  (line 11730)
- [x] window_text_pixel_size  (line 11742, static)
- [x] format_nargs  (line 12264, static)
- [x] add_to_log  (line 12279)
- [x] vadd_to_log  (line 12288)
- [x] message_log_maybe_newline  (line 12314)
- [x] reset_message_log_need_newline  (line 12325)
- [x] message_dolog  (line 12340)
- [x] message_log_check_duplicate  (line 12535, static)
- [x] log_message  (line 12565, static)
- [x] message3  (line 12592)
- [x] message3_frame  (line 12603)
- [x] message_to_stderr  (line 12613, static)
- [x] message3_nolog  (line 12644)
- [x] message3_frame_nolog  (line 12650)
- [x] message1  (line 12723)
- [x] message1_nolog  (line 12732)
- [x] message_with_string  (line 12741)
- [x] vmessage  (line 12802, static)
- [x] message  (line 12869)
- [x] update_echo_area  (line 12883)
- [x] ensure_echo_area_buffers  (line 12898, static)
- [x] with_echo_area_buffer  (line 12938, static)
- [x] with_echo_area_buffer_unwind_data  (line 13028, static)
- [x] unwind_with_echo_area_buffer  (line 13072, static)
- [x] setup_echo_area_for_printing  (line 13106)
- [x] display_echo_area  (line 13188, static)
- [x] display_echo_area_1  (line 13228, static)
- [x] resize_echo_area_exactly  (line 13263)
- [x] resize_mini_window_1  (line 13289, static)
- [x] resize_mini_window  (line 13307)
- [x] current_message  (line 13454)
- [x] current_message_1  (line 13472, static)
- [x] push_message  (line 13490)
- [x] restore_message  (line 13501)
- [x] pop_message_unwind  (line 13511)
- [x] check_message_stack  (line 13524)
- [x] clear_message_stack  (line 13531)
- [x] truncate_echo_area  (line 13540)
- [x] truncate_message_1  (line 13563, static)
- [x] set_message  (line 13576, static)
- [x] set_message_1  (line 13621, static)
- [x] clear_message  (line 13657)
- [x] clear_garbaged_frames  (line 13699, static)
- [x] echo_area_display  (line 13748, static)
- [x] window_buffer_changed  (line 13854, static)
- [x] mode_line_update_needed  (line 13866, static)
- [x] window_frozen_p  (line 13877, static)
- [x] format_mode_line_unwind_data  (line 13943, static)
- [x] unwind_format_mode_line  (line 13997, static)
- [x] store_mode_line_noprop_char  (line 14070, static)
- [x] store_mode_line_noprop  (line 14097, static)
- [x] needs_no_redisplay  (line 14242, static)
- [x] prepare_menu_bars  (line 14259, static)
- [x] update_menu_bar  (line 14421, static)
- [x] restore_selected_window  (line 14521, static)
- [x] restore_frame_selected_window  (line 14563, static)
- [x] update_tab_bar  (line 14584, static)
- [x] display_tab_bar  (line 14693, static)
- [x] tty_get_tab_bar_item  (line 15586, static)
- [ ] tty_handle_tab_bar_click  (line 15616)
- [x] hscroll_window_tree  (line 16625, static)
- [x] hscroll_windows  (line 16923, static)
- [x] text_outside_line_unchanged_p  (line 17006, static)
- [x] redisplay  (line 17073)
- [x] overlay_arrow_string_or_property  (line 17080, static)
- [x] overlay_arrow_in_current_buffer_p  (line 17092, static)
- [x] overlay_arrows_changed_p  (line 17120, static)
- [x] update_overlay_arrows  (line 17162, static)
- [x] overlay_arrow_at_row  (line 17199, static)
- [x] check_point_in_composition  (line 17255, static)
- [x] reconsider_clip_changes  (line 17290, static)
- [x] propagate_buffer_redisplay  (line 17318, static)
- [x] redisplay_internal  (line 17351, static)
- [x] unwind_redisplay_preserve_echo_area  (line 18238, static)
- [x] redisplay_preserve_echo_area  (line 18255)
- [x] unwind_redisplay  (line 18284, static)
- [x] unwind_display_working_on_window  (line 18293)
- [x] mark_window_display_accurate_1  (line 18304, static)
- [x] mark_window_display_accurate  (line 18407)
- [x] disp_char_vector  (line 18436)
- [x] block_buffer_flips  (line 18461, static)
- [x] unblock_buffer_flips  (line 18468, static)
- [x] buffer_flipping_blocked_p  (line 18486)
- [x] redisplay_windows  (line 18499, static)
- [x] redisplay_window_error  (line 18522, static)
- [x] redisplay_window_0  (line 18540, static)
- [x] redisplay_window_1  (line 18548, static)
- [x] update_redisplay_ticks  (line 18574)
- [x] set_cursor_from_row  (line 18631, static)
- [x] run_window_scroll_functions  (line 19264, static)
- [x] cursor_row_fully_visible_p  (line 19305, static)
- [x] try_scrolling  (line 19402, static)
- [x] compute_window_start_on_continuation_line  (line 19745, static)
- [x] try_cursor_movement  (line 19865, static)
- [x] set_vertical_scroll_bar  (line 20268)
- [x] set_horizontal_scroll_bar  (line 20325)
- [x] window_start_acceptable_p  (line 20389, static)
- [x] redisplay_window  (line 20475, static)
- [x] try_window  (line 21684)
- [x] try_window_reusing_current_matrix  (line 21793, static)
- [x] find_last_row_displaying_text  (line 22261, static)
- [x] find_last_unchanged_at_beg_row  (line 22295, static)
- [x] find_first_unchanged_at_end_row  (line 22352, static)
- [x] sync_frame_with_window_matrix_rows  (line 22428, static)
- [x] row_containing_pos  (line 22472)
- [x] try_window_id  (line 22588, static)
- [x] get_overlay_arrow_glyph_row  (line 23822, static)
- [x] insert_left_trunc_glyphs  (line 23893, static)
- [x] row_hash  (line 24048)
- [x] compute_line_metrics  (line 24082, static)
- [x] clear_position  (line 24157, static)
- [x] append_space_for_newline  (line 24176, static)
- [x] extend_face_to_end_of_line  (line 24384, static)
- [x] trailing_whitespace_p  (line 24869, static)
- [x] highlight_trailing_whitespace  (line 24891, static)
- [x] row_for_charpos_p  (line 24975, static)
- [x] cursor_row_p  (line 25062, static)
- [x] push_prefix_prop  (line 25077, static)
- [x] get_it_property  (line 25175, static)
- [x] get_line_prefix_it_property  (line 25196, static)
- [x] handle_line_prefix  (line 25217, static)
- [x] unproduce_glyphs  (line 25258, static)
- [x] find_row_edges  (line 25278, static)
- [x] display_count_lines_logically  (line 25431, static)
- [x] display_count_lines_visually  (line 25450, static)
- [x] maybe_produce_line_number  (line 25499, static)
- [x] should_produce_line_number  (line 25800, static)
- [x] row_text_area_empty  (line 25835, static)
- [x] display_line  (line 25865, static)
- [x] display_menu_bar  (line 27761, static)
- [x] deep_copy_glyph_row  (line 27924, static)
- [x] display_tty_menu_separator_char  (line 27954, static)
- [x] display_tty_menu_separator  (line 27973, static)
- [x] display_tty_menu_item  (line 28008)
- [x] display_mode_lines  (line 28112, static)
- [x] display_mode_line  (line 28205, static)
- [x] move_elt_to_front  (line 28394, static)
- [x] safe_set_text_properties  (line 28432, static)
- [ ] display_mode_element  (line 28463, static)
- [ ] store_mode_line_string  (line 28977, static)
- [ ] pint2str  (line 29193, static)
- [ ] pint2hrstr  (line 29239, static)
- [ ] decode_mode_spec_coding  (line 29339, static)
- [ ] percent99  (line 29417, static)
- [ ] decode_mode_spec  (line 29434, static)
- [ ] count_lines  (line 29897)
- [ ] display_count_lines  (line 29914)
- [ ] display_string  (line 30062, static)
- [ ] invisible_prop  (line 30314)
- [ ] calc_pixel_width_or_height  (line 30470, static)
- [ ] get_font_ascent_descent  (line 30702)
- [ ] produce_stretch_glyph  (line 32972)
- [ ] produce_special_glyphs  (line 33192, static)
- [ ] pad_mode_line  (line 33406, static)
- [ ] get_specified_cursor_type  (line 34731, static)
- [ ] set_frame_cursor_types  (line 34779)
- [ ] draw_row_with_mouse_face  (line 35477, static)
- [ ] show_mouse_face  (line 35497, static)
- [ ] clear_mouse_face  (line 35662)
- [ ] coords_in_mouse_face_p  (line 35678, static)
- [ ] cursor_in_mouse_face_p  (line 35727)
- [ ] rows_from_pos_range  (line 35759, static)
- [ ] mouse_face_from_buffer_pos  (line 35915, static)
- [ ] mouse_face_from_string_pos  (line 36327, static)
- [ ] define_frame_cursor1  (line 36586, static)
- [ ] note_line_or_margin_highlight  (line 36633, static)
- [ ] note_mouse_highlight  (line 37056)
- [ ] gui_clear_window_mouse_face  (line 37706)
- [ ] cancel_mouse_face  (line 37724)
- [ ] syms_of_xdisp  (line 38410)
- [ ] init_xdisp  (line 39542)

## Macros defined — 38
- [x] AINC  (line 17604, body 27cb0e150c56)
- [x] BASE_LINE_NUMBER_VALID_P  (line 19396, body 89582bb92958)
- [x] BUFFER_POS_REACHED_P  (line 10165, body 27a2bd2c9b82)
- [x] CHAR_COMPOSED_P  (line 8297, body 9fd5f9641609)
- [x] CLEAR_FACE_CACHE_COUNT  (line 1054, body f83a383c0fa8)
- [x] DEFAULT_HOURGLASS_DELAY  (line 1115, body 356a192b7913)
- [x] DISP_INFINITY  (line 508, body 9dfdd483be2b)
- [x] FRAME_OUTPUT_DATA  (line 505, body 48a28489fbe0)
- [x] GET_NEXT_DISPLAY_ELEMENT  (line 8286, body 174c658220fb)
- [x] GIVE_UP  (line 22628, body 06d3d9b2060d)
- [x] IT_DISPLAYING_WHITESPACE  (line 538, body 56e7f5473230)
- [x] IT_OVERFLOW_NEWLINE_INTO_FRINGE  (line 531, body 7cb6efb98ba5)
- [x] IT_POS_VALID_AFTER_MOVE_P  (line 10071, body 63b253cef7dc)
- [x] IT_RESET_X_ASCENT_DESCENT  (line 10215, body f1100fbd8546)
- [x] LINE_BREAKABLE  (line 554, body 9a67501b22a9)
- [ ] LNUM_ONCE  (line 30485, body a34f80a0f2f1)
- [x] MAX_DISP_SCAN  (line 4493, body ba30fd97b412)
- [x] MAX_SCRATCH_GLYPHS  (line 813, body 310b86e0b62b)
- [x] MODE_LINE_NOPROP_LEN  (line 13913, body 72c39ab23422)
- [x] NOT_AT_BOL  (line 553, body f192a1aff587)
- [x] NOT_AT_EOL  (line 552, body 79632480908d)
- [ ] OK_ALIGN_TO  (line 30482, body fe44bbad711c)
- [ ] OK_PIXELS  (line 30481, body 1d72993912be)
- [x] RECORD_MAX_MIN_POS  (line 26024, body 9ee0416c4092)
- [x] RECORD_OVERLAY_STRING  (line 7121, body 16715b981720)
- [x] REDISPLAY_SOME_P  (line 14235, body c6fb49f84210)
- [x] RESTORE_IT  (line 848, body 4985d364c618)
- [x] RESUME_POLLING  (line 17343, body 2a5d0cb2a6a4)
- [x] ROW_GLYPH_NEWLINE_P  (line 27196, body 9d6d5b96f828)
- [x] SAFE_CALLMANY  (line 3155, body 67a378c1bce8)
- [x] SAVE_IT  (line 840, body cf8ea527efc2)
- [x] SCROLL_LIMIT  (line 19391, body 310b86e0b62b)
- [x] SET_WITH_NARROWED_BEGV  (line 3829, body 53ab6f455be2)
- [x] STOP_POLLING  (line 17339, body c86b380dd52a)
- [x] TEXT_PROP_DISTANCE_LIMIT  (line 831, body 310b86e0b62b)
- [x] dsafe_calln  (line 3157, body a9ee6403caf7)
- [x] face_after_it_pos  (line 1213, body 27264910dde1)
- [x] face_before_it_pos  (line 1212, body ed8f34f9cee6)

## Types and their fields — 2
- [x] overlay_entry: Lisp_Object overlay, Lisp_Object string, EMACS_INT priority, bool after_string_p
- [x] props: short name, enum prop_idx idx, enum prop_handled (*) handler

## File-scope objects — 60
- [x] static Lisp_Object list_of_error  (line 511, static)
- [x] bool noninteractive_need_newline  (line 654)
- [x] static bool message_log_need_newline  (line 658, static)
- [x] static Lisp_Object message_dolog_marker1  (line 663, static)
- [x] static Lisp_Object message_dolog_marker2  (line 664, static)
- [x] static Lisp_Object message_dolog_marker3  (line 665, static)
- [x] static struct text_pos this_line_start_pos  (line 672, static)
- [x] static struct text_pos this_line_end_pos  (line 677, static)
- [x] static int this_line_vpos  (line 681, static)
- [x] static int this_line_y  (line 682, static)
- [x] static int this_line_pixel_height  (line 683, static)
- [x] static int this_line_start_x  (line 688, static)
- [x] static struct text_pos this_line_min_pos  (line 694, static)
- [x] static struct buffer this_line_buffer  (line 698, static)
- [x] static bool overlay_arrow_seen  (line 702, static)
- [x] static Lisp_Object default_invis_vector  (line 706, static)
- [x] Lisp_Object echo_area_window  (line 712)
- [x] static Lisp_Object Vmessage_stack  (line 717, static)
- [x] static bool message_enable_multibyte  (line 722, static)
- [x] int windows_or_buffers_changed  (line 754)
- [x] int update_mode_lines  (line 775)
- [x] static bool line_number_displayed  (line 780, static)
- [x] Lisp_Object echo_area_buffer  (line 785)
- [x] static Lisp_Object echo_buffer  (line 789, static)
- [x] static Lisp_Object Vwith_echo_area_save_vector  (line 793, static)
- [x] static bool display_last_displayed_message_p  (line 798, static)
- [x] static bool message_buf_print  (line 803, static)
- [x] static bool message_cleared_p  (line 808, static)
- [x] static struct glyph_row scratch_glyph_row  (line 814, static)
- [x] static struct glyph scratch_glyphs  (line 815, static)
- [x] static int last_height  (line 819, static)
- [x] bool help_echo_showing_p  (line 823)
- [x] static struct buffer displayed_buffer  (line 975, static)
- [x] static int clear_face_cache_count  (line 1055, static)
- [x] bool redisplaying_p  (line 1069)
- [x] bool display_working_on_window_p  (line 1078)
- [x] int dont_resize_frames  (line 1086)
- [x] Lisp_Object help_echo_string  (line 1091)
- [x] Lisp_Object help_echo_window  (line 1092)
- [x] Lisp_Object help_echo_object  (line 1093)
- [x] ptrdiff_t help_echo_pos  (line 1094)
- [x] Lisp_Object previous_help_echo_string  (line 1098)
- [x] unsigned int redisplay_counter  (line 3094)
- [x] static struct frame last_escape_glyph_frame  (line 8372, static)
- [x] static int last_escape_glyph_face_id  (line 8373, static)
- [x] static int last_escape_glyph_merged_face_id  (line 8374, static)
- [x] static struct frame last_glyphless_glyph_frame  (line 8397, static)
- [x] static int last_glyphless_glyph_face_id  (line 8398, static)
- [x] static int last_glyphless_glyph_merged_face_id  (line 8399, static)
- [x] static char mode_line_noprop_buf  (line 13906, static)
- [x] static char mode_line_noprop_buf_end  (line 13910, static)
- [x] static char mode_line_noprop_ptr  (line 13911, static)
- [x] static Lisp_Object mode_line_proptrans_alist  (line 13925, static)
- [x] static Lisp_Object mode_line_string_list  (line 13928, static)
- [x] static ptrdiff_t mode_line_elt_no  (line 13931, static)
- [x] static Lisp_Object mode_line_string_face  (line 13934, static)
- [x] static Lisp_Object mode_line_string_face_prop  (line 13935, static)
- [x] static Lisp_Object Vmode_line_unwind_vector  (line 13940, static)
- [x] static int buffer_flip_blocked_depth  (line 18458, static)
- [ ] invalid_eol_type  (line 29336, static)
- [ ] static char lots_of_dashes  (line 29431, static)

# ----------------------------------------------------------------------
## Decisions

### Progress (this lane is incremental)

**This round adds C 27637-28462** — the bidi group's third `DEFUN` and the menu-bar/mode-line
opener.  In C's order: `bidi-resolved-levels` (C 27637-27754, the `DEFUN`), `display_menu_bar`
(C 27761-27915), `deep_copy_glyph_row` (C 27924-27946), the three TTY menu helpers
(`display_tty_menu_separator_char` C 27954-27964, `display_tty_menu_separator` C 27973-27984,
`display_tty_menu_item` C 28008-28098), `display_mode_lines` (C 28112-28194),
`display_mode_line` (C 28205-28388), `move_elt_to_front` (C 28394-28429) and
`safe_set_text_properties` (C 28432-28436).  The covered range is now **C 468-28436**; the next
live definition is **C 28463's `display_mode_element`** (the round after it opens there, in C
order — the earlier hole at C 15616's `tty_handle_tab_bar_click` is *not* this round's, and
`--check` still reports it).  `src/xdisp.rs` is 27,160 lines; the readings are in the Evidence
section at the end of this ledger, the round's own deviations are 266-279, and what is left is
enumerated in the Owed section.


`src/xdisp.c` is 39,805 lines and is being transcribed in C order.  **This round adds
C 25865-26935** — `display_line` (C 25864-26935, `static bool`, 1,072 C lines), the whole of
the round: the prologue's measured locals (the eight `UNINIT` scalars, `hscroll_this_line`,
the `MATRIX_ROW_VPOS`/`nrows` guard, `prepare_desired_row`'s clear-and-enable, the two
`SAVE_IT` snapshots' `wrap_*`/`prev_*` registers), the hscrolled-entry walk
(`fast_move_it_horizontally` behind `long_line_optimizations_p` and the
`large_hscroll_threshold` test, else `move_it_in_display_line_to`, then the
`MOVE_NEWLINE_OR_CR`/`MOVE_POS_MATCH_OR_ZV` pretend-reached fixup and
`this_line_min_pos`'s `min_pos`/`min_bpos`), the three line-number/line-prefix entry arms
(`maybe_produce_line_number`/`handle_line_prefix`/
`pending_handle_line_prefix`), the in-function `RECORD_MAX_MIN_POS` macro, the glyph loop
with its `WORD_WRAP` wrap snapshots (`char_can_wrap_after`/`char_can_wrap_before`,
`SAVE_IT`/`RESTORE_IT` over `wrap_it`/`prev_it`), `PRODUCE_GLYPHS` and the marginal-area arm
with its TTY clip fixup, the fit test and the whole `else` branch — the per-glyph `for` with
the four "End of a continued line" arms (`back_to_wrap`'s two `goto`s at C 26331/26368, the
padding-glyph continuation fill, the `wrap_row_*` restore, the TAB arm, the generic
restore) — then the `at_end_of_line` label with its backward `goto` (C 26702), the `TRUNCATE`
block's truncation-glyph loops (`produce_special_glyphs (IT_TRUNCATION)`), the
`margin_face_id` extend and the `BEG_BYTE`/`ZV_BYTE`/`FETCH_BYTE` end-at-ZV tail, the
`bidi_unshelve_cache` pair, the hscrolled-left `insert_left_trunc_glyphs`, `row->end` and
`find_row_edges`, the overlay-arrow copy/padding sweep (`overlay_arrow_at_row`,
`get_overlay_arrow_glyph_row`), `highlight_trailing_whitespace` and `compute_line_metrics`,
the ellipsis record (`ends_in_ellipsis_p`), the four `*_user_fringe_*` copies,
`set_cursor_from_row`'s test and the next-row epilogue.  The covered range is now
**C 468-26935**.  The next live definition is **C 26937's `current-bidi-paragraph-direction`**
(`Fcurrent_bidi_paragraph_direction`/`Scurrent_bidi_paragraph_direction`, a `DEFUN`); the
round after it opens there.  **That round is now in (C 26937-27179): the covered range is
C 468-27179 and the next live definition is C 27181's `move-point-visually`** - the Progress
bullet below carries its readings.  **This round adds C 27181-27636 — `move-point-visually`
(`Fmove_point_visually`/`Smove_point_visually`, a `DEFUN`, 456 C lines, with the in-function
`ROW_GLYPH_NEWLINE_P` macro at C 27196-27202): the covered range is now C 468-27636 and the
next live definition is C 27637's `bidi-resolved-levels`** - the readings are in the Gate
evidence section below.

`src/xdisp.c` is 39,805 lines and is being transcribed in C order.  **The round before this one added C
25077-25863** — the "line numbers and line prefixes" group, the eleven definitions between the
previous round's `cursor_row_p` and `display_line` (the two `^L` page breaks at C 25067 and
C 25251 are dropped: neither opens a section banner, and the port keeps a `^L` only where the
line after it starts one — it carries exactly four, at C 18491/18555/21782/22242).  In C's order: `push_prefix_prop` (C 25077, the `line-prefix`/
`wrap-prefix` pusher — the `text_pos` save through `push_it`, the `phony_display_string` reset
of `string_from_display_prop_p`, the `SCHARS (prop) == 0` bail-out, the string arm's full
iterator reset (`IT_STRING_CHARPOS`/`IT_STRING_BYTEPOS`, `stop_charpos`/`prev_stop`/
`base_level_stop`, `cmp_it.id`, `paragraph_embedding` and the `bidi_init_it` setup) and the
live `(space ...)` stretch arm); `get_it_property` (C 25175, the `Fget_char_property` read of
`IT_STRING_CHARPOS`/`IT_CHARPOS` through `it->window`); `get_line_prefix_it_property` (C 25196,
the `FROM_BUFFER` out-parameter and the overlay/display-string fallback to
`it->w->contents`); `handle_line_prefix` (C 25217, the `continuation_lines_width > 0` choice of
`Qwrap_prefix`/`Vwrap_prefix` against `Qline_prefix`/`Vline_prefix`, and the
`line_wrap = TRUNCATE`/`avoid_cursor_p`/`align_visually_p` fixups); `unproduce_glyphs`
(C 25258, the four `eassert`s and the backward `glyph[-n] = *glyph` compaction);
`find_row_edges` (C 25278, the `minpos`/`maxpos` choice with its `seen_this_string` L2R/R2L
walks over the previous row, the use-case table's six arms and the `emacs_abort` default);
`display_count_lines_logically` (C 25431, the `display_line_numbers_widen`/`BEGV == BEG` fast
path, the `save_restriction_save`/`labeled_restrictions_remove_in_current_buffer`/`Fwiden`
sandwich and the `unbind_to`); `display_count_lines_visually` (C 25450, the `lnum_bytepos`
cache, the `specbind (Qdisplay_line_numbers, Qrelative)` recursion guard and the
`start_display`/`move_it_to` walk); `maybe_produce_line_number` (C 25499, the round's largest:
the `it->lnum`/`lnum_bytepos` cache reuse from `base_line_number`, the two
`display_count_lines_logically` counts, the `lnum_width`/`max_lnum`/`log10` width computation,
the `pint2str`/`strcat` buffer, the whole glyph-production loop over `scratch_glyph_row` with
its face choices (`Qline_number`, `Qline_number_current_line`, the major/minor tick faces) and
the `lnum_pixel_width`/`max_ascent`-family copy-out); `should_produce_line_number` (C 25800,
the `MINI_WINDOW_P` and `Qdisplay_line_numbers_disable` tests plus
`disable_line_numbers_overlay_at_eob`) and `row_text_area_empty` (C 25835, the two
`glyphs[TEXT_AREA]` walks).  The covered range became **C 468-25863**, and the round that
followed transcribed **C 25864-26935**'s `display_line` (the opening paragraph above).
Ticked in the ledger above, bucket by bucket — **measured** with `grep -c '^- \[x\]'` over each
section (`Subrs` 4/13, `Functions` 249/302, `Macros` 31/38, `Types and their fields` 2/2,
`File-scope objects` 59/60) — **345 of the 415 boxes** (249+4+31+2+59); the remaining 70 are
untouched.  **The scaffold shrinks 12 -> 11** (measured with
`git show HEAD:femacs/src/xdisp.rs | awk '/TEMPORARY SCAFFOLD/{f=1} f' | grep -c '^  fn '` = 12):
`display_line`'s entry is deleted and none is added — every name the round's body reaches is
already defined in the unit, defined before use in C, the include graph's, or one of the eleven
entries that remain.  (The burned entry's citation was wrong: it said `dispextern.h 3595`, but
dispextern.h has no `display_line`; C's own declaration is `static bool display_line (struct
it *, int);` at C 1167 — see to-do item 78.)
**Two `docs/ports/macro-body-notes.tsv` rows are owed** (the integrator appends them; item 4 of
the to-do list): `display_line` / `goto,uninit` — C's two `goto back_to_wrap` (C 26331, C 26368)
are the port's `back_to_wrap` flag and `break 'end_of_continued_line`, and the eight `UNINIT`
initialisers (C 25870-25883) are the port's 0 (PORTING.md 1.5/2.2, deviations 236/238/245) —
and `RECORD_MAX_MIN_POS` / `false` — C's in-function `do { … } while (false)` statement macro
(C 26105-26134) is the port's function-scope `macro_rules!`, whose closing brace cannot spell
the trailing test (the `RECORD_OVERLAY_STRING` row's shape; deviation 236).  So `--check`'s
`does not mention` set goes **3 -> 5**; every other token of the body is transcribed.
`--check`'s `^- ` count is **77 -> 77** (both revisions measured): the round retires
`missing function display_line` (44 -> 43) and `missing macro RECORD_MAX_MIN_POS` (5 -> 4), and
adds the two `does not mention` rows above (3 -> 5).  With the two notes rows appended the
`does not mention` set returns to 3 and the reading drops to **75**.  `src/xdisp.rs` is 25,692
lines.

Unticked and reported, not ticked here (the two rounds before last): 10 function boxes whose
definitions are in place and which `--check` does not report (`handle_fontified_prop` C 4667,
`face_at_pos` C 4799, `handle_face_prop` C 4910, `underlying_face_id` C 4969,
`face_before_or_after_it_pos` C 4989, `handle_invisible_prop` C 5206, `setup_for_ellipsis`
C 5654, `find_display_property` C 5696, `get_display_property` C 5761, `display_min_width`
C 5773).

* C 20-467 — the reference's "New redisplay" essay, kept verbatim as a block comment (the
  port's `#![allow (…)]` prelude has to precede it).
* C 468-506 — the `#include`s and `FRAME_OUTPUT_DATA`'s NULL fallback.
* C 508-622 — `DISP_INFINITY`, `list_of_error`, the two `IT_*` word-wrap macros, the three
  category constants, `it_char_has_category`, `char_can_wrap_before`, `char_can_wrap_after`.
* C 624-649 — `fill_column_indicator_column`.
* C 652-931 — the message/redisplay file-scope objects, `SAVE_IT`/`RESTORE_IT`,
  `REDISPLAY_SOME`, and the six `*set_*redisplay`/`*set_update_mode_line` functions.
* C 922-931 — the `set-buffer-redisplay` `DEFUN`.
* C 933-1098 — `redisplay_trace`/`move_trace`, `displayed_buffer`, the `prop_handled` /
  `prop_idx` / `props` types, `it_props[]`, `move_it_result`, and the remaining
  redisplay/echo/help-echo objects.
* C 1212-1684 — the `face_before_it_pos`/`face_after_it_pos` macros, the window display
  dimensions (`window_text_bottom_y`, `window_box_width`, `window_box_height`,
  `window_box_left_offset`, `window_box_right_offset`, `window_box_left`, `window_box_right`,
  `window_box`), `line_bottom_y`, the `line-pixel-height` `DEFUN`,
  `default_line_pixel_height`, `string_from_display_spec`, `window_hscroll_limited`,
  `reset_box_start_end_flags`.
* C 1686-2486 — `pos_visible_p` (with its `#if false` debug block dropped, dead arm), the
  string positions (`check_char_and_length`, `string_pos_nchars_ahead`, `string_pos`,
  `c_string_pos`, `number_of_chars`, `compute_string_pos`), `estimate_mode_line_height`,
  `pixel_to_glyph_coords`, `x_y_to_hpos_vpos`, `frame_to_window_pixel_xy`.
* C 3063-3104 — `adjust_window_ends`, `hscrolling_current_line_p`, the `redisplay_counter`
  global (C 3094, a non-`static` object the ledger's list had missed), the "Lisp form
  evaluation" banner and `dsafe_eval_handler`.
* C 3106-3173 — `dsafe__call` (the `inhibit_eval_during_redisplay` guard, the
  `SPECPDL_INDEX`/`specbind`/`Qinhibit_redisplay`/`Qinhibit_quit` prefix, the nested-redisplay
  counter check with `Ftop_level` and the closing `unbind_to`), `funcall_with_backtraces`,
  the `SAFE_CALLMANY`/`dsafe_calln` macros, `dsafe_call1` and `dsafe_eval`.
* C 3180-3230 — the "Debugging" banner, `CHECK_IT` and `CHECK_WINDOW_END` (both bodies dead;
  see the dead-arm list and deviation 19) and the "Iterator initialization" banner.
* C 3253-3628 — `init_iterator` (`eassert`'s precondition pair, the `inhibit_free_realized_
  faces`/`face_change` flush of the realized faces, `lookup_basic_face`'s remap, the mode
  line/tab line/header line row choice out of the desired matrix, the `memclear` of the
  iterator, `XSETWINDOW`/`it->w`/`it->f`, the `cmp_it` reset, the buffer-local
  `extra-line-spacing` setup, the basic-face recomputation, `ctl_arrow_p`/`selective`/
  `selective_display_ellipsis_p`/`dp`/`multibyte_p`/`tab_width`, the line-wrap choice, the
  truncation/continuation glyph dimensions through `produce_special_glyphs`, the display-area
  geometry, the mode-line box fixup and the buffer-position tail with the bidi setup and
  `reseat`).
* C 3630-3715 — the reference's "How Emacs deals with long lines" essay, kept verbatim as a
  block comment.
* C 3716-3825 — `get_narrowed_width`, `get_narrowed_len`, `get_medium_narrowing_begv`,
  `get_medium_narrowing_zv`, `get_nearby_bol_pos` (its gradually enlarged
  `find_newline1` search), `get_small_narrowing_begv`, `get_large_narrowing_begv`,
  `get_large_narrowing_zv` and `unwind_narrowed_begv`.
* C 3827-3845 — the `SET_WITH_NARROWED_BEGV` macro (the `SPECPDL_INDEX`/
  `record_unwind_protect`/`SET_BUF_BEGV`/`unbind_to` sandwich around `EXPR`).
* C 3846-3946 — `start_display` (the desired-matrix row, `init_iterator`, and the
  `SAVE_IT`/`reseat_at_previous_visible_line_start`/`move_it_to` continuation-line width
  computation).
* C 3951-3983 — `in_ellipses_for_invisible_text_p`.
* C 3985-4323 — the `init_to_row` family and the text-properties entry point:
  `init_from_display_pos` (the overlay-string scan, the `GET_FROM_IMAGE` `pop_it`, the
  chunked `load_overlay_strings` loop, the overlay-string/bidi re-seat and the
  `dpvec_index` restoration), `init_to_row_start`, `init_to_row_end` (the `continued_p`
  continuation width and the composition punt), and `handle_stop` (the `it_props[]`
  handler loop with its `HANDLED_*` dispatch, the `get_overlay_strings_1` re-check, the
  `handle_overlay_change` step and `compute_stop_pos`).
* C 4326-4484 — `compute_stop_pos` (the string/buffer split, the `next_overlay_change` and
  composition-break-at-point stop positions, the `TEXT_PROP_DISTANCE_LIMIT`/`10 *` composable
  run scan through `fetch_char_advance_no_check`, the `validate_interval_range` walk over
  `it_props`'s `values_here` array, and the `composition_compute_stop_pos` call with the
  `bidi_level_start`-derived backward-scan stop).
* C 4490-4493 — the `MAX_DISP_SCAN` macro.
* C 4495-4582 — `compute_display_string_pos` (the object/`string_p` prologue, the
  `get_char_property_and_overlay` test at CHARPOS, and the
  `Fnext_single_char_property_change` forward scan that calls `handle_display_spec` with a
  null iterator).
* C 4594-4641 — `compute_display_string_end` (the `WINDOWP`/`OVERLAYP` object choice, the
  "the display property was removed" guard, and the `OVERLAY_END`/next-property-change
  end).
* C 4658-4790 — the "Fontification" banner and `handle_fontified_prop` (the
  `Vmemory_full` bail-out, the `STRINGP`/`it->s`/`Vfontification_functions`/`Vrun_hooks`
  guard whose last conjunct assigns `pos`/`prop` and tests `NILP (prop)` below `Z`, the
  `specpdl_ref`/`specbind` `Qfontification_functions` sandwich, the `long_line_
  optimizations_p` narrowing through `labeled_narrow_to_region`, the
  `inhibit_clear_image_cache` fig leaf, the `t`-means-run-the-global-binding `dsafe_call1`
  walk, and the `clip_changed` reset with its `set_buffer_internal_1` fallback).
* C 4794-4980 — the "Faces" banner, `face_at_pos` (the `STRINGP` split between
  `face_at_buffer_position` and the overlay-string/`stack` walk that ends in
  `face_at_string_position`, with the `MARGIN_FACE_ID` margin fixup), `handle_face_prop`
  (the `Qinhibit_quit` sandwich around `face_at_pos`, the `face_before_it_pos` look-back and
  the `start_of_box_run_p`/`face_box_p` box-run bookkeeping) and `underlying_face_id`.
* C 4983-5194 — `face_before_or_after_it_pos` (the string/buffer split, the non-bidi
  `DEC_TEXT_POS`/`INC_TEXT_POS` walks and the composition look-ahead, the bidi re-scan from
  the string start through `bidi_move_to_visually_next`, the
  `move_it_vertically_backward`/`move_it_in_display_line` look-back, and the `FACE_FOR_CHAR`
  charset fixup on both sides).
* C 5198-5647 — the "Invisible text" banner and `handle_invisible_prop` (the text-property
  / char-property read with its `overlay` out-parameter, `TEXT_PROP_MEANS_INVISIBLE`, the
  `BIDI_AT_BASE_LEVEL` fast/slow split, both bidi-iteration loops out of invisible text, the
  `skip_invisible` loop with its newline/`ZV` exits, the `reseat_1` newline shortcut, the
  `display_ellipsis_p` position fixup and the `get_overlay_strings` before-string handling).
* C 5650-5692 — `setup_for_ellipsis` (the `DISP_INVIS_VECTOR`/`default_invis_vector` choice,
  the `dpvec`/`dpend`/`dpvec_char_len`/`dpvec_face_id` reset, the `saved_face_id` restore and
  the switch to `GET_FROM_DISPLAY_VECTOR`).
* C 5695-5896 — `find_display_property` (the vector / list / single-spec cases around C's
  `goto found`, whose tail is the port's labelled block), `get_display_property`,
  `display_min_width` (the two `min-width` bound tests, the `calc_pixel_width_or_height`
  measurement and `produce_stretch_glyph` insertion, and the end-of-run `width_spec` record)
  and the `get-display-property` `DEFUN`.
* C 5897-6762 — the `'display' property` banner, `handle_display_prop` (the string/buffer
  position split, the iterator reset of `slice`/`space_width`/`font_height`/`voffset`, the
  `min-width` end handling and the `handle_display_spec` call), `handle_display_spec` (the
  `disable-eval` unwrap and the list/vector/single-spec walk with C's `replacing`
  short-circuit), `display_prop_end`, `handle_single_display_spec` (the `when` form with its
  `object`/`position`/`buffer-position` bindings and the bidi-cache sandwich, the `height`,
  `space-width`, `min-width`, `slice` and `raise` sub-specs, the overlay-end /
  `display_prop_end` text skip, the fringe arm's bidi synchronisation, the margin prefix, and
  the string / `(space ...)` / xwidget / image dispositions), `display_prop_intangible_p`,
  `single_display_spec_string_p` and `display_prop_string_p`.
* C 6763-7562 — `string_buffer_position_lim`/`string_buffer_position` (the two
  `Fget_char_property` walks and the `Fnext_single_char_property_change` /
  `Fprevious_single_char_property_change` steps), the `overlay_entry` record,
  `handle_composition_prop` (the string/buffer character read, `find_composition`,
  `composition_valid_p`, `get_composition_id` and the `cmp_it` setup), `handle_overlay_change`,
  `next_overlay_string` (the exhausted/exhausted-empty paths, the
  `ignore_overlay_strings_at_pos_p` and `overlay_strings_at_end_processed_p` flags, and the
  next-chunk string setup with its bidi state), `compare_overlay_entries` (the qsort
  comparator), `load_overlay_strings` (the `RECORD_OVERLAY_STRING` macro, the `USE_SAFE_ALLOCA`
  buffer growth, the `ITREE_FOREACH … DESCENDING` overlay walk, the `qsort` and the
  `OVERLAY_STRING_CHUNK_SIZE` copy-out), `get_overlay_strings_1`, `get_overlay_strings`,
  `push_it`/`pop_it` (the whole `iterator_stack_entry` save/restore, both method switches,
  `bidi_push_it`/`bidi_pop_it` and the empty-argument `eassert`),
  `iterate_out_of_display_property` and `restore_face_box_flags`.

* C 7573-8430 — `back_to_previous_line_start` (the `dec_both` + `SET_WITH_NARROWED_BEGV`
  step through `find_newline_no_quit`), `strings_with_newlines` (the overlay walk with its
  `memchr` tests and the `Fnext_single_property_change` comma-expression loop),
  `forward_to_next_line_start` (the newline short-cut, the `next_overlay_change`/
  `strings_with_newlines` probe, the bidi `disp_pos` walk and the `ITERATOR_AT_END_OF_LINE_P`
  slow path), `back_to_previous_visible_line_start` (the `SAVE_IT`/`RESTORE_IT` snapshot, the
  composition and display-property `replaced:` label), `reseat_at_previous_visible_line_start`,
  `reseat_at_next_visible_line_start`, `reseat` (the medium/large narrowing refit and the
  `prev_stop`/`base_level_stop` priming), `reseat_1` (the full iterator reset and the bidi
  re-init), `reseat_to_string` (both the Lisp-string and C-string paths, the
  precision/field-width clamp, the standard display table and the composition stop position),
  the `next_element_function` typedef with `get_next_element[]`, the `GET_NEXT_DISPLAY_ELEMENT`
  and `CHAR_COMPOSED_P` macros, `lookup_glyphless_char_display` (the
  `Vglyphless_char_display` lookup with its `retry:` loop), the six escape/glyphless face
  caches, `merge_escape_glyph_face`, `merge_glyphless_glyph_face` and
  `forget_escape_and_glyphless_faces`.
* C 8437-9246 — `get_next_display_element` (the `GET_NEXT_DISPLAY_ELEMENT` dispatch, the
  UAX#9 mirrored-glyph fixup, the `CHARSET_FROM_ID`/`DECODE_CHAR`/`BYTE8_TO_CHAR` unibyte
  mapping, the `DISP_CHAR_VECTOR` display-table branch, the glyphless lookup, the
  control-character translation with `DISPLAY_CTRL_GLYPH`/`DISPLAY_ESCAPE_GLYPH`,
  `merge_faces`/`merge_escape_glyph_face` and the `sprintf` formatter, and the `done:`
  box-run bookkeeping with `face_after_it_pos`/`face_at_buffer_position`/
  `face_at_string_position`) and `set_iterator_to_next` (all six method arms, the
  composition grapheme-cluster advance, the bidi walk with its stop-position re-computation,
  and the shared `consider_string_end` label).
* C 9258-10060 — `next_element_from_display_vector` (the `dpend`/`dpvec` glyph read, the
  unibyte-to-multibyte promotion, `dpvec_face_id`/`merge_faces`, and the box-run flags),
  `get_visually_first_element` (the `SET_WITH_NARROWED_BEGV` bob, the bidi priming walk and
  the composition stop position), `next_element_from_string` (the invisible-text/stop-position
  checks with `handle_stop_backwards`, and both the overlay-string and padded-string paths),
  `next_element_from_c_string`, `next_element_from_ellipsis`, `next_element_from_image`,
  `next_element_from_xwidget`, `next_element_from_stretch`, `compute_stop_pos_backwards`,
  `handle_stop_backwards` (the two backward scans that save and restore the iterator),
  `next_element_from_buffer` (the end-of-buffer/overlay-string handling, the bidi stop
  bookkeeping, the `BYTE_POS_ADDR` character read and the selective-display ellipsis) and
  `next_element_from_composition`.
* C 10063-10877 — the "Moving an iterator without producing glyphs" banner,
  `IT_POS_VALID_AFTER_MOVE_P`, and `move_it_in_display_line_to`: the line-number priming
  (`should_produce_line_number`/`maybe_produce_line_number`/`handle_line_prefix` with the
  `line_number_pending` deferral), the `TRUNCATE` vs `WORD_WRAP` stop tests, the
  per-glyph `for` loop with its `IT_RESET_X_ASCENT_DESCENT` bookkeeping and the
  `atpos_it`/`atx_it`/`wrap_it`/`ppos_it` snapshot chain, the box-run `PRODUCE_GLYPHS` pass,
  the bidi `closest_pos` look-ahead with its recursive re-scan, the
  `ITERATOR_AT_END_OF_LINE_P` newline handling, the truncation `MOVE_LINE_TRUNCATED` arm, the
  `bidi_unshelve_cache` unwind of `done:`, and the start-of-box-run/glyph-row restore.
* C 10879-11645 — `move_it_in_display_line` (the `WORD_WRAP` + `MOVE_TO_X` backtrack over
  `SAVE_IT`/`RESTORE_IT`), `move_it_to` (the `MOVE_TO_VPOS`/`MOVE_TO_Y`/position arms, the
  `skip` switch with its `MOVE_LINE_TRUNCATED` EOB test and the `MOVE_LINE_CONTINUED` TAB
  tab-stop reproduction through `FACE_FROM_ID`/`font->space_width`, the `out:` epilogue with
  the text-terminal multi-glyph fixup), `move_it_vertically_backward` (the `move_further_back:`
  loop, the `nlines--` estimate, the `it2`/`it3` snapshots, the `dy == 0` line-start path with
  `find_newline_no_quit`, and the `dy > 0` pixel-target path with `line_bottom_y`/
  `window_box_height`), `move_it_vertically`, `move_it_past_eol`, and `move_it_by_lines` (the
  `MOVE_TO_VPOS` path, the backward line-walk with `hit_pos_limit`, the display-string retreat
  loop, the `it2` re-scan and the three-way restore).
* C 11647-12525 — `partial_line_height` (the `long_line_optimizations_p` short-cut, the
  `IT_EOB` branch and the `move_it_by_lines` tail), `fast_move_it_horizontally` (the
  `find_newline_no_quit` look-ahead and the `MOVE_NEWLINE_OR_CR` adjustment),
  `in_display_vector_p`, `window_text_pixel_size` (the `from`/`to` `t`-and-cons trimming
  loops, the `decode_live_window`/`start_display` setup, the vertical-offset walk, both
  `move_it_to` passes with their `display property at START/END` retries, the x/y clamping
  and the three `display_mode_line` additions), the three `DEFUN!`s (`window-text-pixel-size`,
  `buffer-text-pixel-size` with its `record_unwind_protect`/`wset_buffer`/`set_marker_both`
  window swap, and `display--line-is-continued-p`), and the message-log cluster
  (`format_nargs`'s `strchr` loop, the variadic `add_to_log` forwarding through `VaList`,
  `vadd_to_log`'s `AUTO_STRING`/`Fformat_message`/`SAFE_ALLOCA` buffer, and `message_dolog`'s
  buffer swap, the three `message_dolog_marker*` saves, the multibyte conversions, the
  duplicate-line collapse through `message_log_check_duplicate`
  (`sprintf`/`PRIdMAX`/`del_range_both`) and the restoration of `BEGV`/`ZV`/point/markers).
* C 12526-13255 — `message_log_check_duplicate` (the byte-by-byte `seen_dots` walk with its
  post-increment `*p1++` chain, `strtoimax`/`INTMAX_MAX`/`strncmp`), `log_message` (the
  `SAFE_ALLOCA_STRING` copy into `message_dolog`), the `message3*` family with
  `message_to_stderr`'s `errputc`/`errwrite` and `message3_frame_nolog`'s frame swap
  (`record_unwind_protect`, `XSETFRAME`, `frame_up_to_date_hook`), `message1`/`message1_nolog`,
  `message_with_string`'s `AUTO_STRING`+`CALLN (Fformat_message)` path, `vmessage` (the
  `doprnt` formatter and the multibyte head truncation) with its variadic `message`,
  `update_echo_area`, `ensure_echo_area_buffers` (the `" *Echo Area %d*"` name buffer),
  `with_echo_area_buffer` (the which/this_one/the_other choice and the
  `record_unwind_protect` pair), `with_echo_area_buffer_unwind_data`/
  `unwind_with_echo_area_buffer` (the 11-element save vector),
  `setup_echo_area_for_printing` (the `del_range`/`Fset_buffer_multibyte`/`Fraise_frame`
  path), and `display_echo_area`/`display_echo_area_1` (the `with_echo_area_buffer` callback,
  `resize_mini_window`, `clear_glyph_matrix` and `try_window`).
* C 13256-13994 — `resize_echo_area_exactly` (the `with_echo_area_buffer` call with
  `minibuf_level` choosing the `resize_exactly` argument), `resize_mini_window_1` and
  `resize_mini_window` (the `WINDOW_BOX_TEXT_HEIGHT` bookkeeping, the `FRAME_INNER_HEIGHT`
  against `Vmax_mini_window_height`, the `init_iterator`/`move_it_to` height scan with its
  EOB and line-spacing corrections, and the `grow_mini_window`/`shrink_mini_window` pair),
  `current_message`/`current_message_1` (the callback that fills a `Lisp_Object` through
  `void *`), the `Vmessage_stack` family (`push_message`, `restore_message`,
  `pop_message_unwind`, `check_message_stack`, `clear_message_stack`),
  `truncate_echo_area`/`truncate_message_1` (C's `(void *) (intptr_t) {nchars}` packing),
  `set_message`/`set_message_1`/`clear_message` (the `Vset_message_function`/
  `Vclear_message_function` hooks with `dsafe_call1`/`dsafe_calln`, `message_enable_multibyte`,
  `Fset_buffer_multibyte`, `bset_bidi_paragraph_direction`, `insert_from_string`),
  `clear_garbaged_frames` (the `FOR_EACH_FRAME` walk with `frame_redisplay_p`/
  `FRAME_GARBAGED_P`/`is_tty_frame`), `echo_area_display` (the `update_frame_p` update path
  through `update_single_window`/`flush_frame`/`update_frame`/`combine_updates_for_frame`),
  `window_buffer_changed`, `mode_line_update_needed`, `window_frozen_p`, the mode-line
  file-scope objects and enumeration, `MODE_LINE_NOPROP_LEN`, and
  `format_mode_line_unwind_data` (the 12-element save vector with `FRAME_TTY`'s top frame).
* C 13995-14783 — `unwind_format_mode_line` (`AREF`-restore of the mode-line save vector, the
  `WINDOW_LIVE_P` window/frame/top-frame reselection and the `set_point_from_marker` point
  restore), `store_mode_line_noprop_char` (the `mode_line_noprop_ptr ==
  mode_line_noprop_buf_end` `xpalloc` growth) with `store_mode_line_noprop` (the
  `strlen`/`c_string_width` copy and the `nbytes--` post-decrement loop),
  `REDISPLAY_SOME_P` with `needs_no_redisplay` (the four `bool_bf` reads),
  `prepare_menu_bars` (the `Vpre_redisplay_function` call through `dsafe_calln`, the
  `window_list` filtering and the per-frame `update_menu_bar`/`update_tab_bar` walk),
  `update_menu_bar` (the live `FRAME_MENU_BAR_LINES` ternary, the `Qinhibit_menubar_update`
  binding, the two `safe_run_hooks` calls, `menu_bar_items` and the
  `w->update_mode_line` store), `restore_selected_window`/`restore_frame_selected_window`,
  `update_tab_bar` (the `FRAME_TAB_BAR_LINES` test, the `selected_frame`/`selected_window`
  swap under `record_unwind_protect`, and `tab_bar_items`/`Fequal_including_properties`), and
  `display_tab_bar` (the TTY `init_iterator` into the frame matrix rows, the row-clearing loop
  through `glyph_row_bits!`, and the `display_string`/`compute_line_metrics` tail).
* C 15584-16929 — `tty_get_tab_bar_item` (the `SCHARS` walk over `f->tab_bar_items` and the
  `Fget_text_property`/`Qclose_tab` hit test) and `hscroll_window_tree`/`hscroll_windows` (the
  `FLOATP`/`TYPE_RANGED_FIXNUMP` step computation, the recursive window-tree walk, the
  line-number `x_offset` scan over `cursor_row->glyphs`, the `suspend_auto_hscroll` reset with
  `SET_FRAME_GARBAGED`, the `old_pointm` marker update, the long
  `!NILP (Fbuffer_local_value (Qauto_hscroll_mode, …))` margin test, and the
  `init_to_row_start` + `DISP_INFINITY` `move_it_in_display_line_to` /
  `fast_move_it_horizontally` scan that ends in the `hscroll` clamp and the
  `prevent_redisplay_optimizations_p` store).  C 15615-15654's `tty_handle_tab_bar_click` and
  the whole Tool-bars section (C 15656-16610) are recorded, not transcribed (see the Progress
  paragraph).
* C 17005-17348 — `text_outside_line_unchanged_p` (the `GPT`/`Z` gap test, the two
  `BEG_UNCHANGED`/`END_UNCHANGED` bounds, the `overlay_touches_p` pair and the bidi
  `bidi_display_reordering`/`bidi_paragraph_direction` give-up), `redisplay`,
  `overlay_arrow_string_or_property` (the `if (val = Fget (...), STRINGP (val))` comma
  expression), the four `Voverlay_arrow_variable_list` walks
  (`overlay_arrow_in_current_buffer_p`, `overlay_arrows_changed_p` with its `Fmarker_position` /
  `Qlast_arrow_position` / `Qlast_arrow_string` comparison and `bset_redisplay`,
  `update_overlay_arrows` with its `Fput` pairs and `overlay_arrow_at_row` with the
  `MATRIX_ROW_START_CHARPOS`/`MATRIX_ROW_END_CHARPOS` window and the dead fringe arm),
  `check_point_in_composition` (the `find_composition` + `composition_valid_p` pair),
  `reconsider_clip_changes` (the `current_matrix->{buffer,zv,begv}` test and the `check_point_in_composition`
  call) and `propagate_buffer_redisplay` (the `window_list` copy of `text->redisplay` up to
  `w->redisplay`), plus the `STOP_POLLING`/`RESUME_POLLING` macros.  `redisplay_internal`
  (C 17350-18235) is *not* here: it is one 886-line function and is the whole of the next
  round.
* C 17350-18235 — `redisplay_internal`: the early-out tests (`redisplaying_p`,
  `FRAME_INITIAL_P`/`Vinhibit_redisplay`, `glyphs_initialized_p`), the unwind-protect and
  `specbind` prelude, the tty `previous_frame`/`set_tty_color_mode` swap, the visible-frame
  count and `adjust_frame_glyphs`/`SET_FRAME_GARBAGED` repair, `clear_garbaged_frames`,
  `prepare_menu_bars`, `reconsider_clip_changes`, the echo-area/`resize_mini_window` branch,
  the `AINC` tallies, the one-line optimization chain (`text_outside_line_unchanged_p` gate,
  `start_display`/`display_line`, the `increment_matrix_positions` charstart fixup, and the
  `set_cursor_from_row` variant), the `consider_all_windows_p` frame walk with
  `redisplay_windows`, `hscroll_windows`, `update_frame` and the
  `condemn`/`judge_scroll_bars_hook` pair, the single-window path through
  `internal_condition_case_1 (redisplay_window_1, …, redisplay_window_error)`, the `update:`
  block's `update_frame`/`combine_updates_for_frame` pair, `mark_window_display_accurate_1`,
  `update_overlay_arrows`, `RESUME_POLLING` and the tick reset.  C's dead arms inside it
  (C 17411-17419, C 17760-17766, C 17842-17845, C 17862-17864, C 17997-18001, C 18213-18219
  and C 18222-18224) are recorded, not transcribed (deviation 149).
* C 18237-18619 — the redisplay tail: `unwind_redisplay_preserve_echo_area` (the
  `unblock_buffer_flips` unwind), `redisplay_preserve_echo_area` (the `block_input` /
  `record_unwind_protect_void` / echo-area `display_last_displayed_message_p` pair around
  `redisplay_internal`, then `flush_frame` and `unbind_to`), `unwind_redisplay` (clearing
  `redisplaying_p`), `unwind_display_working_on_window`, `mark_window_display_accurate_1`
  (the `BUF_*` bookkeeping, the `current_matrix` `buffer`/`begv`/`zv` and its two matrix bits,
  the `last_point`/`last_mark` update and the `MATRIX_ROW` window-end test),
  `mark_window_display_accurate` (the recursive walk plus the `update_overlay_arrows`
  pair), `disp_char_vector` (the `ASCII_CHAR_P` fast path through `dp->ascii` and its
  `XSUB_CHAR_TABLE`, else `char_table_ref`, with `defalt` as the fallback),
  `buffer_flip_blocked_depth`/`block_buffer_flips`/`unblock_buffer_flips`/
  `buffer_flipping_blocked_p` (the `FOR_EACH_FRAME` `buffer_flipping_unblocked_hook` fan-out),
  `redisplay_windows` (the recursive window walk through
  `internal_condition_case_1 (redisplay_window_0, …)`), `redisplay_window_error` (the
  `display_error_modiff` store and the `max_redisplay_ticks` delayed-warning path),
  `redisplay_window_0`/`redisplay_window_1` and `update_redisplay_ticks` (the tick
  accumulators and the "takes too long" `error`).  C 18307-18309 and C 18339-18383
  (`HAVE_TEXT_CONVERSION`) are recorded, not transcribed (deviation 157).
* C 18630-19255 — `set_cursor_from_row`: the two edge-pointer scans that skip the glyphs with no
  object at the row's start and end (with the reversed-row mirror image and the
  `LAST_AREA` terminal-border guard), the two `Step 1` scans (forward and reversed) that hunt
  for the glyph at point, record `pos_before`/`pos_after` alternatives, accumulate
  `bpos_max`/`bpos_min`/`bpos_covered` and consult the `cursor` text property through
  `Fget_char_property` and `string_buffer_position_lim`, the `Step 2` fallbacks
  (`ends_in_ellipsis_p`'s backward scan, `match_with_avoid_cursor`, the `string_seen`
  interval search with its `start`/`stop` ordering and the `compute_x` jump, the truncated
  left/right and zero-width cases), the `compute_x:` block itself, and the final candidate
  comparison against `w->cursor` (the `g1` glyph, `eabs` distance, the `cursor`-property
  preference and the row-span tie-break) before `w->cursor.{hpos,x,vpos,y}` and the
  `this_line_*` records are written.  No C `#if` inside the function is dead.
* C 19256-19862 — `run_window_scroll_functions` (`safe_run_hooks_2` over
  `Vwindow_scroll_functions`, the `SET_TEXT_POS_FROM_MARKER` re-read of `w->start`, and the
  `set_buffer_internal` restore), `cursor_row_fully_visible_p` (the
  `buffer_local_value (Qmake_cursor_line_fully_visible)`/`Qunbound` fallback to the global, the
  `FUNCTIONP` preference with its "a signal means do not scroll" note, the desired-vs-current
  matrix choice, the `MATRIX_ROW_PARTIALLY_VISIBLE_AT_TOP_P` row and the `force_p` tail),
  the `SCROLLING_*` enum as `const`s and `SCROLL_LIMIT`,
  `try_scrolling` (the `arg_scroll_conservatively` clamp to `scroll_limit + 1`, the
  `max`-of-three `scroll_lines` bound through `clip_to_bounds`, the scroll-margin pixel probe
  with its `slack`/`y_to_move` search, the before-string recovery `do … while`, the
  `too_near_end:` labelled loop with its `continue`, the conservative/`scroll_step`/aggressive
  `amount_to_scroll` chain, the `move_it_vertically` short path and the
  `SAVE_IT`/`RESTORE_IT` `it1` loop around `line_bottom_y`, the top-margin mirror image ending
  in `move_it_vertically_backward`, and the `run_window_scroll_functions` → `try_window` →
  cursor-visibility tail that loops back through `continue 'too_near_end`),
  `compute_window_start_on_continuation_line` (the `BEGV`/`ZV` start clamp, the desired-matrix
  row offset by `window_wants_tab_line`/`window_wants_header_line`,
  `reseat_at_previous_visible_line_start`, the `WINDOW_TOTAL_LINES`×`WINDOW_TOTAL_COLS` guard,
  the closest-below-point walk with its `WORD_WRAP` two-call X precision, and the
  `pos_before_pt` fallback before `SET_MARKER_FROM_TEXT_POS`), and the `CURSOR_MOVEMENT_*` enum
  as `const`s.
* C 19863-20422 — `try_cursor_movement` (the `is_tty_root_frame_with_visible_child` bail-out, the
  two `eassert`s on `last_point`/`window_end_vpos`, the seventeen-clause outer test —
  `PT >= CHARPOS (startp)`, `clip_changed`, `update_mode_lines`, `windows_or_buffers_changed`,
  `cursor_type_changed`, `show-trailing-whitespace`, three `display-line-numbers` clauses,
  `minibuf_window` and `overlay_arrow_in_current_buffer_p` — then the last-cursor-row
  validation, the forward walk with its "next row owns PT" loop, the backward walk with the
  `row0->continued_p` bidi second pass, the BEGV/START and overlay-newline fixups, the two
  scroll tests, the `must_scroll`/partially-visible/`scroll_p` chain, the bidi candidate loop
  with its `at_zv_p`/`exact_match_p` test on `candidate->glyphs[TEXT_AREA] + w->cursor.hpos`,
  and the plain `set_cursor_from_row` walk), `set_vertical_scroll_bar` (`BUF_ZV`/`BUF_BEGV`/
  `BUF_Z` arithmetic, the `window_end_valid`/`long_line_optimizations_p` recomputation through
  `start_display`/`move_it_to`, and `set_vertical_scroll_bar_hook`), `set_horizontal_scroll_bar`
  (`last_visible_x = INT_MAX`, the `move_it_to` width, `hscroll * FRAME_COLUMN_WIDTH`, the
  `it.bidi_p` left-to-right flip through `Fcurrent_bidi_paragraph_direction`, `hscroll_whole`,
  and `set_horizontal_scroll_bar_hook`), `window_start_acceptable_p`,
  `Flong_line_optimizations_p`'s `DEFUN` body.  (`BUF_BEGV`/`BUF_ZV` are C's `INLINE`
  functions at buffer.h 917/951, not `#define`s; the unit names the mirror's `pub fn`s at
  `include/buffer.rs:923/957`.)
  C 19879-19882 and C 19934-19936 (`#ifdef GLYPH_DEBUG`) are the round's two dead arms.
* C 20424-21662 — `redisplay_window`: the doc comment (C 20424-20472) and the whole body.
  The prologue (`XWINDOW`/`XFRAME`/`XBUFFER` and the reference's twenty-two locals, the three
  `text_pos` ones fully initialised — deviation 180), the dead `GLYPH_DEBUG` arm of C
  20502-20504, `SET_TEXT_POS`/`opoint = lpoint`, the `needs_no_redisplay` early-out, the two
  marker `eassert`s, `reconsider_clip_changes`, `default_line_pixel_height`,
  `window_scroll_margin (w, MARGIN_IN_LINES)`, the `update_mode_line` fold over
  `w->update_mode_line`/`update_mode_lines`/`buffer->clip_changed`/
  `buffer->prevent_redisplay_optimizations_p`, the `must_be_updated_p` store, the
  `MINI_WINDOW_P` shortcuts (the echo-area window's `finish_menu_bars`/`finish_scroll_bars`
  jumps, the inactive mini-window's `blank_row` loop over `w->desired_matrix->rows` with its
  `y += row->height, ++row` increment, the `resize_mini_window` call and the trailing
  `clear_glyph_matrix`), `set_buffer_internal_1`, the six-clause
  `current_matrix_up_to_date_p` test, `BEG_UNCHANGED`/`END_UNCHANGED`, `specbind
  (Qinhibit_point_motion_hooks, Qt)`, the `windows_or_buffers_changed` repair
  (`compute_window_start_on_continuation_line` plus `window_end_valid`/the `false` flag),
  `CHECK_WINDOW_END` and the two `Z == Z_BYTE`/`BYTEPOS < CHARPOS` `emacs_abort`s,
  `mode_line_update_needed`, the point-marker fixup for a non-selected window
  (`marker_position`/`marker_byte_position`/`set_marker_both`/`TEMP_SET_PT_BOTH` over the
  `BEGV`/`ZV` clamps), the width-run-cache invalidation (`buffer_display_table`,
  `disptab_matches_widthtab` against `XVECTOR (BVAR (current_buffer, width_table))`,
  `invalidate_region_cache` and `recompute_width_table`) and the long-line threshold scan
  (`XFIXNUM (Vlong_line_threshold)`, the `find_newline1` walk with its `&found` out-parameter
  and the `long_line_optimizations_p` store);
  the `optional_new_start` probe (`window_frozen_p`, `start_display`/`move_it_to` with
  `MOVE_TO_POS|MOVE_TO_X|MOVE_TO_Y`, `line_bottom_y` and the two `force_start` stores);
  `force_start`: the vscroll/mode-line repair (`preserve_vscroll_p`, `window_end_valid`,
  `run_window_scroll_functions`), the `BEGV`/`ZV` `startp` clamps, the
  `window_start_acceptable_p` rejection, `clear_glyph_matrix` + `try_window`, the
  `cursor.vpos < 0` invisible-text fixup (`get_char_property_and_overlay`,
  `Fnext_single_char_property_change`, `row_containing_pos`, `MATRIX_ROW_BOTTOM_Y`), the
  `cursor_row_fully_visible_p` retries with `pixel_margin`/`tab_line`/`header_line` and
  `CURRENT_TAB_LINE_HEIGHT`/`CURRENT_HEADER_LINE_HEIGHT`, the `new_vpos` walk over
  `MATRIX_FIRST_TEXT_ROW`/`MATRIX_ROW_BOTTOM_Y`/`ends_at_zv_p` with
  `MATRIX_ROW_START_CHARPOS`/`_BYTEPOS`, `set_cursor_from_row`, the `w->redisplay`/
  `text->redisplay` clears with `dsafe_calln (Vpre_redisplay_function, …)` and the
  `display-line-numbers` re-run, and the closing `cursor_row_fully_visible_p` test;
  `ignore_start`: the `try_cursor_movement` call with its `switch (rc)` over
  `CURSOR_MOVEMENT_SUCCESS`/`_MUST_SCROLL`/`default`, the `start_at_line_beg` test that
  recomputes the window start (`pos_visible_p` with its eight out-parameters and the
  `(rtop != 0) == (rbot != 0)` acceptance), the `try_window_id` alternative with
  `f->fonts_changed`, and the continuation-line/invisible-start/`same window start` block
  (`BEG + beg_unchanged`, `Z - end_unchanged`, `try_window_reusing_current_matrix`,
  `TRY_WINDOW_CHECK_MARGINS` and the dead `IF_DEBUG` of C 21077);
  `try_to_scroll`: the mode-line re-raise and the `try_scrolling` call with its `switch (ss)`
  over `SCROLLING_SUCCESS`/`SCROLLING_NEED_LARGER_MATRICES`/`SCROLLING_FAILED` (the
  `SCROLL_LIMIT + 1` override and the `SCROLL_LIMIT` clamps on `scroll_conservatively`);
  `recenter`: `init_iterator` at PT, the `centering_position < 0` measurement
  (`SAVE_IT`/`RESTORE_IT` around `start_display` + `move_it_vertically`, the
  `scroll_up_aggressively`/`scroll_down_aggressively` choice via `XFLOATINT` and
  `WINDOW_BOX_TEXT_HEIGHT`, the `pt_offset` clamp and the `margin * frame_line_height`
  floor), the truncated-long-line shortcut (`back_to_previous_visible_line_start` with the
  `nlines--` loop and `reseat_1`) against `move_it_vertically_backward`, the
  `it.current_x`/`wrap_prefix_width`/`hpos` reset, `set_marker_both (w->start, …)`,
  `run_window_scroll_functions`, `bidi_shelve_cache`/`bidi_unshelve_cache`, the
  `try_window`/`try_window_reusing_current_matrix` pass, the `cursor.vpos < 0` retries
  (`move_it_by_lines` ±1, the `before-string` `do … while`, `overlay_touches_p`), the
  `row_containing_pos` fallback chain with `use_desired_matrix`, `set_cursor_from_row`, and
  the `cursor_row_fully_visible_p` tail (`vscroll` reset, the `WINDOW_TOTAL_LINES`/
  `FRAME_LINE_HEIGHT` `window_total_lines`, `move_it_by_lines` and the
  `TRY_WINDOW_CHECK_MARGINS` re-try, the `centering_position = 0` path);
  `done`: `SET_TEXT_POS_FROM_MARKER`, the `start_at_line_beg` recomputation, the mode-line/
  header-line/tab-line display (`display_mode_lines` under `specbind (Qinhibit_quit, Qt)`,
  `current_column`, and the three height fixups through `MATRIX_MODE_LINE_ROW`/
  `MATRIX_TAB_LINE_ROW`/`MATRIX_HEADER_LINE_ROW` and `f->fonts_changed`), and the
  `base_line_pos`/`base_line_number` reset;
  `finish_menu_bars`: the frame's menu bar for the selected window (`FRAME_SELECTED_WINDOW`,
  `FRAME_MENU_BAR_LINES`, the `root_frame` child-frame arm with
  `Qtty_menu_open_use_tmm`/`Fdefault_value`, and the live `#else` `display_tab_bar` arm),
  and the `need_larger_matrices`/`finish_scroll_bars` tail (the two setters, the
  `FRAME_TERMINAL (f)->redeem_scroll_bar_hook` call, the `opoint`/`lpoint` restores with
  their `CHAR_TO_BYTE` fallbacks, `set_buffer_internal_1 (old)` and `unbind_to (count,
  Qnil)`).  C 20502-20504, C 20726-20734, C 20939-20941, C 20977-20979, C 20988-20990,
  C 21045-21047, C 21056-21058, C 21062-21064, C 21157-21159, C 21077's `IF_DEBUG`,
  C 21527-21533, C 21551-21581 and C 21584-21605 are recorded, not transcribed
  (the dead-arm list above).
* C 21664-21779 — `try_window`: the doc comment (C 21665-21682) and the whole body.  The
  `set_marker_both ((*w).start, Qnil, CHARPOS (pos), BYTEPOS (pos))` window-start store, the
  `cursor.vpos = -1`/`overlay_arrow_seen = false` reset, `start_display` and
  `it.glyph_row->reversed_p = false`; the display loop with `display_line` and the
  `last_text_row = it.glyph_row - 1` capture, the two per-iteration `last_row_scale`/
  `last_col_scale` snapshots and the `f->fonts_changed` + `TRY_WINDOW_IGNORE_FONTS_CHANGE` test
  that returns 0 when the matrix dimensions moved; `IT_CHARPOS`; the `TRY_WINDOW_CHECK_MARGINS`
  block (`window_scroll_margin (w, MARGIN_IN_PIXELS)`, the `CURRENT_HEADER_LINE_HEIGHT`/
  `CURRENT_TAB_LINE_HEIGHT` top-margin adds, `bot_scroll_margin`, the second `start_display` and
  the `partial_line_height` bottom test that returns -1 after `clear_glyph_matrix`); the
  `window_end_pos <= 0 && Z != it_charpos` mode-line fixup; the `MATRIX_ROW_DISPLAYS_TEXT_P`
  `eassert` pair around `adjust_window_ends`; the `window_end_bytepos`/`window_end_pos`/
  `window_end_vpos` fallback and `window_end_valid = false`.  The function is non-`static`
  (dispextern.h 3573), so the definition carries `#[unsafe(no_mangle)] pub unsafe extern "C"`;
  it opens with C's own doc comment, verbatim.  **No `#if`** — the live-conditional list above is
  unchanged this round.
* C 21780-22542 — the "reusing the current matrix when the buffer has not changed" group
  (banner C 21783-21786): `try_window_reusing_current_matrix` (C 21793-22239, its doc comment
  opening at C 21787, both arms), `find_last_row_displaying_text` (C 22261),
  `find_last_unchanged_at_beg_row` (C 22295), `find_first_unchanged_at_end_row` (C 22352),
  `sync_frame_with_window_matrix_rows` (C 22428) and `row_containing_pos` (C 22472-22542).  The
  three forward declarations at C 22247-22253 are dropped (0.1-12); the round's six dead `#if`
  blocks are in the dead-arm list below, and its two `does not mention` rows
  (`try_window_reusing_current_matrix`, `row_containing_pos`) are deviation 197's.
* C 23817-25075 — the "Building Desired Matrix Rows" group's first eleven definitions (the
  C 23813-23816 banner introduces them): `get_overlay_arrow_glyph_row` (C 23821, `static struct
  glyph_row *`), `insert_left_trunc_glyphs` (C 23893), `row_hash` (C 24048, non-`static`), the
  already-scaffolded `compute_line_metrics` (C 24082), `clear_position` (C 24157),
  `append_space_for_newline` (C 24176), the already-forwarded `extend_face_to_end_of_line`
  (C 24384), `trailing_whitespace_p` (C 24869), `highlight_trailing_whitespace` (C 24891) and
  the two scaffold entries `row_for_charpos_p` (C 24975) and `cursor_row_p` (C 25062).  The
  round's five dead `#if` blocks are in the dead-arm list below; it retires three scaffold
  entries and adds none, and it owes no `macro-body-notes.tsv` row (the Progress section).
* C 25077-25863 — the "line numbers and line prefixes" group's eleven definitions:
  `push_prefix_prop` (C 25077, static), `get_it_property` (C 25175), the new
  `get_line_prefix_it_property` (C 25196), `handle_line_prefix` (C 25217, one of the three
  scaffold entries this round retires), `unproduce_glyphs` (C 25258), `find_row_edges`
  (C 25278), `display_count_lines_logically` (C 25431), `display_count_lines_visually`
  (C 25450), `maybe_produce_line_number` (C 25499, scaffold entry retired) and
  `should_produce_line_number` (C 25800, scaffold entry retired), the round's two dead `#if`
  blocks are in the dead-arm list below, and it owes the one `macro-body-notes.tsv` row named
  in the Progress section.  `display_line` (C 25864-26935) was the round after it.
* C 25864-26935 — `display_line` (the opening paragraph names every arm); its two `goto
  back_to_wrap` sites and its `at_end_of_line` label are the round's only control-flow work,
  and its `#undef RECORD_MAX_MIN_POS` (C 26934) has no Rust site (the dead-arm list below).
* C 26937-27179 — the bidi paragraph group's first two DEFUNs.
  `current-bidi-paragraph-direction` (C 26937-27028, `Fcurrent_bidi_paragraph_direction`/
  `Scurrent_bidi_paragraph_direction`, 0/1): C's `switch (itb.paragraph_dir)` becomes the port's
  `match` over `NEUTRAL_DIR`/`L2R`/`R2L` (deviation 246), the uninitialised `struct bidi_it itb`
  is `core::mem::zeroed ()` (1.5), and `AUTO_STRING (trailing_white_space, "[\f\t ]*\n")` is the
  `AUTO_STRING!` spelling of the same literal.  It needs one use-site declaration
  (`fast_looking_at`, item 17) and one `macro-body-notes.tsv` row (`case,default,switch`), and it
  retires the subr and defsubr the scaffold had carried.
  `bidi-find-overridden-directionality` (C 27030-27179, `Fbidi_find_overridden_directionality`/
  `Sbidi_find_overridden_directionality`, 3/4) follows the same C order: the object dispatch
  (`BUFFERP`/`WINDOWP`/`CHECK_STRING`), the string arm's `validate_subarray` and the buffer arm's
  `validate_region`, the iterator's setup in both arms, then C's `do { … } while (…)` as a `loop`
  with the condition negated at the end (deviation 251).  It needs two more use-site declarations
  (item 18) and owes no `macro-body-notes.tsv` row.  The covered range is now **C 468-27179**; the
  **next live definition is C 27181's `move-point-visually`** (`DEFUN`, the Subrs list's tenth
  unchecked item).
* C 27181-27636 — `move-point-visually` (the Subrs list's tenth box; `Fmove_point_visually`/
  `Smove_point_visually`, 1/1): the whole DEFUN in C's order — the two paths.  The glyph path
  is the `w->window_end_valid`/`windows_or_buffers_changed`/`b->clip_changed`/
  `prevent_redisplay_optimizations_p`/`window_outdated`/`w->last_point == BUF_PT (b)` guard
  with its `w->cursor.vpos`/`nrows`/`enabled_p` row test, the `for (g = gpt + dir; …)` walk
  with its `BUFFERP` arm, its overlay-string arm (`gpt->resolved_level - row->reversed_p`,
  `clip_to_bounds` and the BEGV/ZV error break) and its `ROW_GLYPH_NEWLINE_P` arm, then the
  `g == e || NILP (g->object)` row step and the two direction-specific glyph scans (the
  `dir > 0` scan with its `ends_at_zv_p` arm and the `dir <= 0` one with its `STRETCH_GLYPH`
  arm); the `simulate_display:` fallback is `Fcurrent_bidi_paragraph_direction`'s paragraph
  direction, the `xsignal0` pair at BEGV/ZV, and the `struct it` arena (`SET_TEXT_POS`/
  `start_display`, the `reseat:` loop, `PRODUCE_GLYPHS`'s `pixel_width`, the overshoot/bidi
  fixup, the `dir < 0` TTY re-seat and its `WORD_WRAP` `SAVE_IT`/`RESTORE_IT` probe, the
  `at_eol_p`/`last_visible_x` line step, the GUI grapheme walk with its `IT_COMPOSITION`
  positions, the `IT_CHARPOS (it) == PT` skip and the `SET_PT_BOTH`/`bidi_unshelve_cache`
  tail).  Its two `goto simulate_display`, its `goto reseat` and its in-function
  `ROW_GLYPH_NEWLINE_P` macro are deviations 255-257 (the rest of its zero-initialised locals,
  chained stores, `for`-loop increments and the mirror's untyped `current_matrix` are
  258-264).  It retires the `missing subr` and the `missing macro` lines and owes the one
  `macro-body-notes.tsv` row item 4 names; the `missing defsubr` line stays, because
  `defsubr (&Smove_point_visually)` is `syms_of_xdisp`'s (C 38469), outside the covered range.

### The transcription that is in place

The ledger above is the checklist.  Its tick boxes read 248 of the C file's 302 functions
(the sentence here previously carried the stale figures 219 functions / 20,897 lines) plus 4 of
its 13 subrs — the three the frozen configuration keeps live and `long-line-optimizations-p` —
defined in C's order, with C's names, C's statements and C's comments; `src/xdisp.rs` is 24,435
lines.  Every function that C declares `static` is a
plain `unsafe fn` here (and `unsafe extern "C" fn` where it is variadic or its address is
handed out — `redisplay_trace`, `move_trace`, `dsafe_eval_handler`,
`funcall_with_backtraces`, `handle_display_prop` (whose address is `it_props[]`'s handler
field) and `unwind_narrowed_begv`, whose address `SET_WITH_NARROWED_BEGV` hands to
`record_unwind_protect`); every non-`static` definition carries
`#[unsafe(no_mangle)] pub unsafe extern "C"` (PORTING.md 0.2/P-52).

The 24 `DEFUN`s of C are the unit's subrs; the four that are live on the frozen target
(`set-buffer-redisplay`, `line-pixel-height`, `get-display-property`,
`long-line-optimizations-p`) use `DEFUN!` (PORTING.md 5.1).

### Dead arms (P-08: recorded here, not transcribed)

`src/xdisp.c` *is* in `docs/baseline/config/cond-eval-verified.txt` (line 420;
`python3 tools/check-cond-eval.py src/xdisp.c` = `196 conditions, 0 unknown, 0 disagreements
[VERIFIED]`), so `tools/extract-abi.py` decides every `#if` and the arms the frozen
configuration does not compile are recorded, not carried.  The ones inside this round's range (C 468-1684):

* C 517-530 — the `HAVE_WINDOW_SYSTEM` arm of `IT_OVERFLOW_NEWLINE_INTO_FRINGE` (the live
  `#else` arm, C 531's `false`, is what the port defines).
* C 937-942 — `#ifdef GLYPH_DEBUG`'s `extern bool trace_redisplay_p` + its definition; the
  live `#else` arm is `enum { trace_redisplay_p = false };` (the port's `const`).
* C 953-960 — `#ifdef DEBUG_TRACE_MOVE`'s `trace_move` (same shape; live arm is `false`).
* C 1064-1071 — `CLEAR_IMAGE_CACHE_COUNT`, `clear_image_cache_count`, `null_glyph_slice`.
* C 1110-1112, C 1117-1122 — `hourglass_shown_p`, `hourglass_atimer`, `THIN_SPACE_WIDTH`.
* C 1215-1238 — the whole `HAVE_WINDOW_SYSTEM` block of prototypes.
* C 1480-1502 — `window_box_edges`.
* C 3027-3031 — the `#if false && defined HAVE_X_WINDOWS` feedback rectangle (inside
  `remember_mouse_glyph`, past this round's end; listed for the next round).
* C 2135-2142 — `pos_visible_p`'s `#if false` "Debugging code" `fprintf` block (`w->vscroll`
  is named nowhere else); the port keeps a pointer comment at the site (the `window.rs`
  `unshow_buffer` shape).
* C 2294-2315 — the `HAVE_WINDOW_SYSTEM` arm of `estimate_mode_line_height` (the
  `FRAME_WINDOW_P (f)` `FONT_HEIGHT`/`FACE_FROM_ID_OR_NULL` computation); the live body is
  C 2317's `return 1;`.
* C 2330-2366 — the `HAVE_WINDOW_SYSTEM` arm of `pixel_to_glyph_coords` (the
  `FRAME_PIXEL_X_TO_COL`/`FRAME_LINE_TO_PIXEL_Y` conversion, `STORE_NATIVE_RECT` and the
  `noclip` clamp); the live body is C 2368-2369's `*x = pix_x; *y = pix_y;`.
* C 2452-2465 — `x_y_to_column_row` under
  `#if defined HAVE_WINDOW_SYSTEM && !defined HAVE_EXT_MENU_BAR`.
* C 2488-3062 — the whole `HAVE_WINDOW_SYSTEM` block between `frame_to_window_pixel_xy` and
  `adjust_window_ends` (the GUI glyph-string clip/mouse-face machinery); this round's range
  ends at 2486 and the block is dead, so it is recorded, not transcribed.
* C 3185-3211 — `CHECK_IT`'s whole body is one `#if false` block (dead in every configuration,
  the `cond-eval-verified.txt` line 420 verdict), and C 3222-3229 is `CHECK_WINDOW_END`'s
  `#if defined GLYPH_DEBUG && defined ENABLE_CHECKING` (neither macro is defined in
  `docs/baseline/config/darwin-arm64.macros.tsv`).  Both functions stay as named no-ops
  (deviation 19).

The third round's range (C 3253-3979: `init_iterator`, the long-line narrowing helpers,
`start_display` and `in_ellipses_for_invisible_text_p`) and this round's (C 3985-4323: the
`init_to_row` family and `handle_stop`) contain no `#if` at all, so nothing new is recorded
for them here.

The fifth round's range (C 5897-6762) holds five `#if` blocks, all dead on this target:

* C 6016-6018 — the `HAVE_XWIDGETS` disjunct of `handle_display_spec`'s "simple
  specifications" test (`&& !EQ (XCAR (spec), Qxwidget)`).
* C 6320-6330 — the `HAVE_WINDOW_SYSTEM` arm of the `(raise FACTOR)` handler (the
  `FACE_FROM_ID`/`normal_char_height`/`FRAME_LINE_HEIGHT` computation of `it->voffset`;
  `normal_char_height` and `FRAME_LINE_HEIGHT` are named nowhere else in the unit).
* C 6407-6473 — the `HAVE_WINDOW_SYSTEM` arm of the `(left-fringe …)`/`(right-fringe …)`
  handler (`lookup_fringe_bitmap`, `lookup_derived_face`, `push_it`, and the
  `IT_IMAGE`/`GET_FROM_IMAGE` setup with the two `*_user_fringe_*` members).
* C 6513-6516 — the `HAVE_WINDOW_SYSTEM` disjunct of `valid_p` (`valid_image_p`).
* C 6620-6641 — the `HAVE_WINDOW_SYSTEM` `else` arm that would set up an image
  (`lookup_image`, `IT_IMAGE`, `GET_FROM_IMAGE`).

Also dropped (not dead arms, but text with no Rust counterpart): C 514-515, 863-864, 1003-1008
and 1124-1209's forward declarations (PORTING.md 0.1-12), and C 619-622's four `#undef`s
(Rust has no `#undef`; the four names stay defined in the unit and nothing else references
them).

The twenty-third round's range (C 19256-19862) holds one `#if` block, dead on this target:

* C 19418-19420 — `try_scrolling`'s `#ifdef GLYPH_DEBUG` `debug_method_add (w, "try_scrolling")`
  (the macro is not defined in `docs/baseline/config/darwin-arm64.macros.tsv`); the port keeps a
  pointer comment at the site (deviation 170).

The twenty-fourth round's range (C 19863-20422) holds two `#if` blocks, both dead on this target:

* C 19879-19882 — `try_cursor_movement`'s `#ifdef GLYPH_DEBUG` `if (inhibit_try_cursor_movement)
  return rc;` (the macro is not defined in `docs/baseline/config/darwin-arm64.macros.tsv`).
* C 19934-19936 — the same function's `#ifdef GLYPH_DEBUG` `debug_method_add (w, "cursor
  movement")`.  Both keep a pointer comment at the site (deviation 178).

The twenty-fifth round's range (C 20424-21662, `redisplay_window`) holds fourteen dead `#if`
blocks.  The eleven `#ifdef GLYPH_DEBUG` ones are the reference's own debugging text (the
macro is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`): C 20502-20504
(`*w->desired_matrix->method = 0`), C 20726-20734 (the `w->force_start` test around the two
`debug_method_add` calls of the `optional_new_start` probe), C 20939-20941 ("forced window
start"), C 20977-20979 ("recenter 1"), C 20988-20990 (`try_window_id %d`), C 21045-21047
("recomputed window start in continuation line"), C 21056-21058 ("recenter 2"),
C 21062-21064 ("same window start") and C 21157-21159 ("recenter").  The tenth is C 21077's
`IF_DEBUG (debug_method_add (w, "1"));`: `IF_DEBUG`'s live arm is C's `((void) 0)` (its
`#else`), so the *argument is never evaluated* — and `debug_method_add` is itself a
`GLYPH_DEBUG`-only declaration, so evaluating it would not link.  The last three are
`#ifdef`s of the GUI half: C 21527-21533 (`HAVE_EXT_MENU_BAR`'s
`FRAME_EXTERNAL_MENU_BAR` arm; the live `#else` arm is the `FRAME_MENU_BAR_LINES` test that
stays), C 21551-21581 (`HAVE_WINDOW_SYSTEM`'s tab-bar/tool-bar block with its inner
`HAVE_EXT_TOOL_BAR` arm, whose live `#else` is the `FRAME_TAB_BAR_LINES`/`display_tab_bar`
pair that stays) and C 21584-21605 (`HAVE_WINDOW_SYSTEM`'s `update_window_fringes`/
`draw_window_fringes`/`gui_draw_*_divider` block).  All fourteen keep a pointer comment at
the site (the standing shape, deviations 170/178).


The twenty-sixth round's range (C 21780-22542) holds six dead `#if` blocks, all `undef` in
`docs/baseline/config/darwin-arm64.macros.tsv`:

* C 21813-21816 — `try_window_reusing_current_matrix`'s `#ifdef GLYPH_DEBUG`
  `if (inhibit_try_window_reusing) return false;`.
* C 21863-21866 — the same function's `#ifdef HAVE_NTGUI` `if (w32_use_visible_system_caret)
  return false;`.
* C 21971-21978 and C 22139-22146 — its two `HAVE_WINDOW_SYSTEM` scroll-hook blocks
  (`update_begin`/`gui_update_window_begin`/`FRAME_RIF (f)->clear_window_mouse_face (w)`/
  `FRAME_RIF (f)->scroll_run_hook (w, &run)`/`gui_update_window_end`/`update_end`).
* C 22043-22045 and C 22232-22234 — the two `#ifdef GLYPH_DEBUG` `debug_method_add` blocks
  (`"try_window_reusing_current_matrix 1"` and `" 2"`), plus C 21883's live `IF_DEBUG
  (debug_method_add (w, "twu1"));`, whose `IF_DEBUG` arm is C's `((void) 0)`.

All six keep a pointer comment at the site (the standing shape, deviations 170/178).  Also
dropped (text with no Rust counterpart, not dead arms): C 22247-22253's three forward
declarations (`find_last_unchanged_at_beg_row`, `find_first_unchanged_at_end_row`,
`find_last_row_displaying_text`) per PORTING.md 0.1-12.

The twenty-seventh round's range (C 22545-23381, `try_window_id`) holds five dead `#if`
blocks, all `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`:

* C 22615-22618 — the function's `#ifdef GLYPH_DEBUG` `if (inhibit_try_window_id) return 0;`.
* C 22621-22627 — the `#if false` arm of the debugging `GIVE_UP`
  (`redisplay_trace ("try_window_id give up %d\n", X); return 0;`); C's live `#else` arm (C
  22628) is the port's `macro_rules! GIVE_UP`, and C's `#undef GIVE_UP` (C 23380) has no Rust
  site (the macro's scope is the function's block).
* C 22872-22875 — the `#ifdef HAVE_NTGUI` `if (FRAME_W32_P (f) && w32_use_visible_system_caret)
  GIVE_UP (25);` (the one `GIVE_UP` the frozen configuration does not compile).
* C 22998-23012 — the `#ifdef GLYPH_DEBUG` `first_unchanged_at_end_row` eassert and the two
  `debug_last_unchanged_at_beg_vpos`/`debug_first_unchanged_at_end_vpos` stores.
* C 23153-23158 — the `HAVE_WINDOW_SYSTEM` scroll-hook arm (`gui_update_window_begin (w)`;
  `FRAME_RIF (f)->clear_window_mouse_face (w)`; `FRAME_RIF (f)->scroll_run_hook (w, &run)`;
  `gui_update_window_end (w, false, false)`), the one `#if` of the round whose `update_begin`/
  `update_end` siblings are live.
* C 23388-23809 — the *next* region's whole `#ifdef GLYPH_DEBUG` block (the `dump-glyph-matrix`/
  `dump-frame-glyph-matrix`/`dump-glyph-row`/`dump-tab-bar-row`/`dump-tool-bar-row`/
  `trace-redisplay`/`trace-to-stderr` subrs and the three `EXTERNALLY_VISIBLE` prototypes).
  It is outside this round's transcribed range (which ends at C 23381) and is recorded here so
  the round's boundary is unambiguous; the next round opens at C 23821.

All five in-range blocks keep a pointer comment at the site (the standing shape, deviations
170/178).  Also dropped (text with no Rust counterpart, not dead arms): C's `#undef GIVE_UP`
(C 23380, above), and the seven live `IF_DEBUG (...)` statements (C 22957-22958, 23076, 23324,
23330, 23367 and 23372-23373), whose `IF_DEBUG` arm is C's `((void) 0)` — recorded at the site,
and the reason `--check` owes the `try_window_id` row (see the to-do list).

The twenty-eighth round's range (C 23817-25075, the desired-matrix-row builders) holds five
dead `#if` blocks, all `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`:

* C 24205-24207 — `append_space_for_newline`'s `#ifdef HAVE_WINDOW_SYSTEM` disjunct
  (`|| FRAME_WINDOW_P (it->f)`) of its `if (default_face_p …)` test.
* C 24213-24225 — the same function's `HAVE_WINDOW_SYSTEM` arm (`FACE_FROM_ID`, `FRAME_FONT`,
  `default_face->font` and the `font->average_width`/`font->space_width` `char_width`
  computation).
* C 24271-24356 — the same function's `HAVE_WINDOW_SYSTEM` arm (the appended space glyph's
  ascent/descent setup: `normal_char_ascent_descent`, `get_it_property` and
  `calc_line_height_property` with `Qline_height`/`Qline_spacing`, `VCENTER_BASELINE_OFFSET`,
  and the `it->max_ascent`/`max_descent` stores).
* C 24449-24451 — `extend_face_to_end_of_line`'s `#ifdef HAVE_WINDOW_SYSTEM` disjunct
  (`&& !face->stipple`) of its skip test.
* C 24474-24732 — the same function's whole `HAVE_WINDOW_SYSTEM` arm (the `FRAME_WINDOW_P (f)`
  GUI block: `space_glyph`, the left/right margin stretch fills through
  `append_stretch_glyph`/`FONT_BASE`/`FONT_HEIGHT`, `WINDOW_*_MARGIN_COLS`, the fill-column
  indicator glyph, and the R2L stretch prepend), with its nested `#ifndef HAVE_EXT_TOOL_BAR` at
  C 24491-24493.

All five keep a pointer comment at the site (the standing shape, deviations 170/178).  The
`#ifdef GLYPH_DEBUG` block the previous round recorded as C 23388-23809 is the "More debugging
support" banner's own block (banner C 23385-23387, `#ifdef` C 23389, `#endif` C 23809); this
round opens at C 23817 and only reconciles the boundary.

The twenty-ninth round's range (C 25077-25863, the line-number and line-prefix group) holds two
dead `#if` blocks, both `HAVE_WINDOW_SYSTEM` and both `undef` in
`docs/baseline/config/darwin-arm64.macros.tsv`:

* C 25154-25162 — `push_prefix_prop`'s `#ifdef HAVE_WINDOW_SYSTEM` arm (`else if (IMAGEP
  (prop))`: `it->what = IT_IMAGE`, `it->image_id = lookup_image (it->f, prop, it->face_id)`,
  `it->method = GET_FROM_IMAGE` and `it->string_from_prefix_prop_p = true`); the live `else`
  arm (C 25163's bogus-property `pop_it`/`return false`) is what the port keeps.
* C 25809-25813 — `should_produce_line_number`'s `#ifdef HAVE_WINDOW_SYSTEM` arm
  (`FRAME_TOOLTIP_P (XFRAME (WINDOW_FRAME (it->w)))`); the live body goes straight from the
  `MINI_WINDOW_P` test (C 25806) to the `Qdisplay_line_numbers_disable` read.

Both keep a pointer comment at the site (the standing shape, deviations 170/178), which is why
`--check` owes no `does not mention` row for `IMAGEP`/`lookup_image`/`FRAME_TOOLTIP_P`/
`XFRAME`/`WINDOW_FRAME`.

The thirtieth round's range (C 25864-26935, `display_line`) holds **no `#if` at all** (the
function is one straight body in every configuration), so nothing new is recorded here for it.
Its one preprocessor line is C's `#undef RECORD_MAX_MIN_POS` (C 26934), which has no Rust site:
the in-function `#define` becomes a function-scope `macro_rules!` whose scope is its block (the
`IT_RESET_X_ASCENT_DESCENT`/`GIVE_UP` class), and the note owed for its un-spellable
`while (false)` tail is named in the Progress section.

### Use-site declarations (headers without a mirror, or mirror gaps; each keeps C's prototype in a `/// C:` line)

1. **`window.h`'s accessor macros — retired to the mirror.**  `include/window.rs` was WIP and
   did not carry `WINDOW_XFRAME`/`WINDOW_FRAME`, the divider widths, the frame edges, the
   scroll-bar geometry, `WINDOW_MODE_LINE_HEIGHT`/`_HEADER_LINE_HEIGHT`/`_TAB_LINE_HEIGHT` or
   the `CURRENT_*_LINE_HEIGHT` family, so this unit defined them under C's own name (PORTING.md
   0.1-2/P-72 rule 3).  The mirror finished that work in the interim, and the
   duplicate-declaration gate — which learned the macro namespace on 2026-10-05 — then found
   the unit's **44** copies shadowing the mirror's.  **All 44 are deleted this round** and the
   unit names the mirror's copies; the two bodies that were not token-identical are the same
   construct, and the mirror's spelling is the C-faithful one (see deviation 179).  The
   `window.h` prototypes whose *definitions* live in window.c stay declared in the unit's
   `unsafe extern "C"` block: `window_body_width` and `window_body_height` (window.h
   1223-1224); frame.h 1191's l-value `FRAME_WINDOW_CHANGE` is the two-argument macro next to
   `frame_bits!` (deviation 25).
2. **`frame_bits!`/`buffer_bits!`/`it_bits!`.**  `struct frame`'s and `struct buffer`'s
   `bool_bf` members are bit runs in the mirror; the writers are here with C's member name as
   the macro's first argument (PORTING.md 1.3/P-80; `f->redisplay` is bit 7 of `bits1`,
   `b->prevent_redisplay_optimizations_p` is bit 0 of `buffer.bits`).
3. **`struct it`.**  `dispextern.h` is mirrored only to its batch-II prefix, so the display
   iterator, `struct bidi_it` and `struct iterator_stack_entry` are declared here as this
   unit's own measured copy (the `window.rs`/`indent.rs` shape).  Every offset was measured
   with clang on the frozen target using `spikes/run.sh`'s flags; the `const _` assertions
   below each record pin sizeof, align and each named member.  The three `bool_bf` runs are
   byte runs the port names through `it_bits!` (byte 112, bytes 2216-2219, and byte 2614 for
   `bidi_p`).  This round names more of that prefix's members (deviation 22) and adds
   `struct composition_it` (deviation 23); `struct bidi_it` names `bytepos`/`charpos`/`w`
   and, this round, its `string` member (deviation 34).
4. **`redisplay_interface`.**  `termhooks.rs` degrades `terminal.rif` to `*mut c_void`
   (P-72 rule 3), so `PRODUCE_GLYPHS` names the two members it needs (`frame_parm_handlers`,
   `produce_glyphs` at measured offset 8) itself — the `dispnew.rs` shape.  The fallback
   `produce_glyphs` (term.c 1640) is declared with C's prototype.
5. **`enum display_element_type`, `it_method`, `line_wrap_method`, `move_operation_enum`,
   `enum prop_handled`, `enum prop_idx`, `IT_*`, `OVERLAY_STRING_CHUNK_SIZE`,
   `IT_STACK_SIZE`** — all in `dispextern.h`'s un-mirrored part, declared here with C's
   numbers (PORTING.md 1.3: C's enum ABI is `c_int`).
6. **The face cache, the bidi state and the long-line search.**  `face_change`
   (dispextern.h 2068), `free_all_realized_faces` (3833), `lookup_basic_face` (3813),
   `init_frame_faces` (3821), `recompute_basic_faces` (3823), `bidi_init_it` (3550),
   `bidi_move_to_visually_next` (3551) and, in this round, `bidi_level_start` (3559) are
   declared in the unit's `unsafe extern "C"` block, as is `find_newline1` (lisp.h 5033; the mirror chain carries only `find_newline`).  `struct face`, `struct face_cache`, `FRAME_FACE_CACHE`,
   `FACE_FROM_ID_OR_NULL`, `FACE_BOX_SHIFT`/`FACE_BOX_MASK` and `face_bits!` are this unit's
   own measured copies next to `struct it` (deviation 26).

7. **The `xwidget.h` inline pair.**  `xwidget.h` has no mirror at all, and C's xdisp.c
   reaches it through `#include "xwidget.h"` (C 499); with `HAVE_XWIDGETS` undef C compiles
   the header's `#else` arm, whose two entry points are `INLINE`s with constant bodies
   (`valid_xwidget_spec_p` returns `false`; `lookup_xwidget` is `eassume (0)`).  This unit
   spells both at their use site with C's header lines in `/// C:` notes (PORTING.md P-72
   rule 3, the `FACE_FOR_CHAR` shape): the `false` one is what makes the `IT_XWIDGET` arm of
   `handle_single_display_spec` unreachable text, and `lookup_xwidget`'s pointer stays
   `*mut c_void` because `struct xwidget` is incomplete on this target.
8. **Three more cross-unit entry points** join the unit's `unsafe extern "C"` block with C's
   header lines: `smaller_face` (dispextern.h 3814) and `face_with_height` (dispextern.h
   3815), both defined by xfaces.c, and `FUNCTIONP` (lisp.h 4799, defined by bytecode.c —
   `include/lisp.rs` does not carry it; `keyboard.rs`/`xfaces.rs` declare their own copies).
11. **One more use-site declaration this round**: `bidi_mirror_char` (dispextern.h 3553,
   bidi.c) joins `bidi_push_it`/`bidi_pop_it` in the unit's `unsafe extern "C"` block — the
   mirror chain carries `bidi_init_it`/`bidi_move_to_visually_next`/`bidi_paragraph_init`/
   `bidi_shelve_cache`/`bidi_unshelve_cache`/`bidi_level_start` but not these three.
   `CHARSET_FROM_ID!`/`DECODE_CHAR!` (charset.rs), `CHAR_*`/`BYTE8_TO_CHAR` (character.rs),
   `DISP_CHAR_VECTOR`/`DISP_CTRL_GLYPH`/`DISP_ESCAPE_GLYPH` (disptab.rs),
   `GLYPH_CODE_P`/`GLYPH_CODE_CHAR`/`GLYPH_CODE_FACE`, `glyph_row_bits!`, `XSETINT!`,
   `XVECTOR`, `sprintf`, `blankp`, `SOFT_HYPHEN`/`HYPHEN`/`NON_BREAKING_HYPHEN` and the
   `bidi_type_t` constants (`STRONG_R`) are already in this unit's include graph.
12. **One more use-site declaration this round**: `rotate_matrix` (dispextern.h 3911,
   dispnew.c) joins the unit's `unsafe extern "C"` block with C's prototype in a `/// C:` line —
   the mirror chain does not carry it, and `try_window_reusing_current_matrix` calls it twice
   (C 21983, C 22167; deviation 201).  The two matrix pointers this round passes to it are the
   mirror's `*mut c_void` with the `as *mut glyph_matrix` cast (§"Deviations" 206).
10. **Four more use-site declarations this round** (same block, C's prototypes in `/// C:`
   lines): `find_newline_no_quit` (lisp.h 5040-5041, search.c — declared per-unit by
   `bidi.rs`/`indent.rs`/`insdel.rs` too), `indented_beyond_p` (lisp.h 5113, indent.c),
   `merge_faces` (dispextern.h 3831, xfaces.c — the `dispnew.rs` spelling) and `window_point`
   (window.h 1233, window.c).  `next_overlay_change`, `get_property_and_range`,
   `buf_charpos_to_bytepos`, `memchr`/`strlen`/`memset`, `dec_both`, `DISP_TABLE_P`,
   `CHAR_TABLE_P`/`XCHAR_TABLE`/`CHAR_TABLE_REF`/`CHAR_TABLE_EXTRA_SLOTS` and
   `FAM_ELEM!` are already in this unit's include graph.
9. **Three more use-site declarations this round** (same block, C's prototypes in `/// C:`
   lines): `bidi_push_it`/`bidi_pop_it` (dispextern.h 3554-3555, defined by bidi.c — the
   mirror chain carries the other four `bidi_*` entry points but not these two) and `qsort`
   (`<stdlib.h>` 489, which `stdlib.rs` does not carry; `buffer.rs`/`font.rs` declare their
   own copies).  `enum itree_order`'s members are imported with `use itree_order::*;` next to
   the include list (the `buffer.rs` shape), and the stack entry's three anonymous `u` records
   are the unit's own named copies (deviation 55).

13. **Six more use-site declarations this round** (the same `unsafe extern "C"` block, C's
    prototypes in `/// C:` lines): `update_begin`/`update_end`/`set_terminal_window`/
    `ins_del_lines` (dispextern.h 3934/3935/3936/3945; terminal.c defines all four and the mirror
    chain carries none of them) and `clear_glyph_matrix_rows`/`shift_glyph_matrix`
    (dispextern.h 3915/3909-3910; dispnew.c defines them).  C's `gui_clear_window_mouse_face`
    (dispextern.h 3689) is **not** among them: `xdisp.c` itself defines it (C 37705, non-`static`),
    so it is a temporary scaffold entry until that definition lands (to-do item 67).

14. **Two more use-site declarations this round**: `lookup_named_face` (dispextern.h 3812) and
    `compute_char_face` (dispextern.h 3832), both defined by xfaces.c, join the unit's `unsafe
    extern "C"` block next to `lookup_basic_face`/`merge_faces` (the mirror chain does not carry
    them; `get_overlay_arrow_glyph_row` at C 23866/23868 and `highlight_trailing_whitespace` at
    C 24932 are this unit's first uses).
15. **Five more use-site declarations this round** (the same `unsafe extern "C"` block, C's
    prototypes in `/// C:` lines): `save_restriction_save`/`save_restriction_restore` (lisp.h
    4924-4925), `labeled_restrictions_remove_in_current_buffer` (lisp.h 4931) and
    `disable_line_numbers_overlay_at_eob` (lisp.h 4937, defined by buffer.c — the mirror's
    "Defined in buffer.c" block stops at `mouse_face_overlay_overlaps`) — editfns.c/buffer.c
    define all four and the lisp.h mirror carries none of them — plus `strcat` (`<string.h>`,
    which `lib/include/string.rs` does not carry; `strcpy`/`stpcpy`/`memset` are already in the
    unit's include graph).  This round's two *scaffold* entries are `pint2str` (xdisp.c 1178,
    defined at C 29193) and `display_count_lines` (lisp.h 4404-4407, defined at C 29914) — C's
    own forward declarations, dropped in the port because a declaration plus the definition is
    E0428 (PORTING.md 0.1-12).
16. **One more use-site declaration this round**: `prepare_desired_row` (dispextern.h 3917,
    defined by dispnew.c) joins the unit's `unsafe extern "C"` block next to
    `clear_glyph_matrix_rows`/`shift_glyph_matrix`, C's prototype in a `/// C:` line; the
    mirror chain carries none of the three, and `display_line` (C 25906) is this unit's first
    use.  `nm build/femacs/xdisp.o` reads `U _prepare_desired_row`.

17. **One more use-site declaration this round**: `fast_looking_at` (lisp.h 5031-5032, defined by
    search.c) joins the unit's `unsafe extern "C"` block next to the `bidi_*` group, C's
    prototype in a `/// C:` line; the lisp.h mirror the chain carries does not have it (bidi.rs
    declares the same prototype for its own first use), and `current-bidi-paragraph-direction`
    (C 27007) is this unit's first use.
18. **Two more use-site declarations, from the round's second DEFUN**: `bidi_find_first_overridden`
    (dispextern.h 3558, defined by bidi.c) and `validate_subarray` (lisp.h 4261-4262, defined by
    fns.c) join the same block, C's prototypes in `/// C:` lines; the mirror chain carries neither
    for this unit, and `bidi-find-overridden-directionality` (C 27143 and C 27107) is the first
    use of each.  (`CHECK_STRING`/`validate_region`/`decode_live_window`/`find_newline_no_quit`
    were already visible: the first two come from the included `buffer.rs` mirror, the other two
    from `window.rs`/`search.c`-side declarations this unit already had.)

### Deviations (each one also named in the report)

1. **Zero-initialised locals** (PORTING.md 1.5): `Fline_pixel_height`'s `it` and `pt` (C
   leaves both to `start_display`), `window_box_left_offset`'s `x` (C's branches always assign
   it, but Rust cannot prove that), `window_box_left`'s `x`.
2. **C's comma/chain expressions became statements in C's order**:
   `it.vpos = it.current_y = 0` is `it.current_y = 0; it.vpos = 0;`, the
   `x = (WINDOW_LEFT_EDGE_X (w) + window_box_left_offset (w, area));` grouping is C's own
   parentheses.
3. **`window_hscroll_limited`'s narrowing return.**  C declares `int` and returns a
   `ptrdiff_t`; the port writes `return window_hscroll as c_int;` — the conversion C performs
   implicitly (`extract-abi.py` reports the cast).
4. **`ckd_mul`/`ckd_add`** (`fill_column_indicator_column`) are the `overflowing_mul`/
   `overflowing_add` pair with C's store-then-test order (P-15, the
   `docs/ports/token-aliases.tsv` family; the `casefiddle.rs` shape).
5. **`trace_redisplay_p`/`trace_move`** are `const ... : c_int = 0` (C's live `enum { ... =
   false }` arm); the `GLYPH_DEBUG`/`DEBUG_TRACE_MOVE` definitions are the recorded dead arms
   above.
6. **`it_props[]`'s `SYMBOL_INDEX (Qx)`** is the generated constant `iQx` (P-17; a static
   initialiser cannot call the port's `SYMBOL_INDEX` function — the `font.rs`/`frame.rs`
   precedent).
7. **`xdisp_defined_globals`**: `include/window.rs` declares `echo_area_window`,
   `update_mode_lines` and `windows_or_buffers_changed` at the mirror's *own top level*, so a
   second top-level definition in this unit is `E0428`.  Those three are therefore defined
   inside `pub mod xdisp_defined_globals` + `use xdisp_defined_globals::*;` (PORTING.md
   0.1-10/P-21; the `window.rs` `window_defined_decls` shape).  The symbol is the one C
   defines.
8. **`start_display`, `move_it_by_lines`, `estimate_mode_line_height`, `move_it_to`,
   `set_iterator_to_next`, `display_mode_line`, `get_next_display_element`,
   `move_it_in_display_line_to`, `handle_display_spec` and `add_to_log`** are *this unit's
   own* later definitions (C 3846, 11503, 2292, 10928, 1126, 28205, 8437, 10109, 5999,
   12279), so they sit in the `TEMPORARY SCAFFOLD` block, not among the cross-unit
   declarations; `estimate_mode_line_height`'s line was deleted when its definition landed
   in the first round, `start_display`'s when its definition landed in the third, and
   `move_it_in_display_line_to`'s in this one (its C prototype is at C 1199-1201, not the
   C 1204-1206 the entry's `/// C:` line said — the burned entry's stale comment is recorded
   in deviation 83).

### Deviations, continued (the round's second pass)

9. **The three char constants** (`NOT_AT_EOL`, `NOT_AT_BOL`, `LINE_BREAKABLE`): C's `#define`
   bodies are char literals, and `extract-abi.py` compares a literal constant by its spelling,
   so the port declares them as `char` (`coding.rs`'s `CODING_INHIBIT_CHARACTER_SUBSTITUTION`
   shape) and the two call sites take the `c_int` that `it_char_has_category` wants with an
   `as c_int` C does with its implicit conversion.
10. **`buffer_bits!`/`buffer_text_bits!`/`frame_bits!`/`it_bits!` keep C's member name as the
    macro's first argument** (PORTING.md 1.3/P-80) so that the `= true`/`= false` of C's bit
    assignments stays in the port's text (`--check`'s "does not mention" rule): C's
    `b->text->redisplay = true` is `buffer_text_bits! (text.redisplay, b, true)` (the arm
    spells C's whole member path, so `text`, `redisplay` and `true` are all still counted),
    `b->prevent_redisplay_optimizations_p = true` is
    `buffer_bits! (prevent_redisplay_optimizations_p, current_buffer, true)`, and
    `f->redisplay = true` is `frame_bits! (redisplay, f, true)`.  No
    `docs/ports/macro-body-notes.tsv` row is needed for any of the three.

11. **`SAVE_IT`'s `if (CACHE)` is pointer truthiness.**  C's `CACHE` parameter is a `void *`
    tested directly; the port's macro arm is `if !$CACHE.is_null () { … }` (Rust has no
    pointer-as-bool).  This is a `docs/ports/macro-body-notes.tsv` row (see §"Integrator
    to-do list" 4); `RESTORE_IT`'s `$pITORIG != $pITCOPY` compares two raw pointers, which
    Rust allows as written.
12. **`struct it` now names `object` and `position`.**  `pos_visible_p` reaches `it->object`
    (C 2793), which the unit's measured `struct it` had inside the `_pad_e` run; the run is
    split at the offsets `term.rs`/`dispnew.rs` already assert (`object` 2408, `position`
    2416) and both members carry the same `offset_of!` assertions here.
13. **Zero-initialised locals** (PORTING.md 1.5): `pos_visible_p`'s `it`, `top`, `it2`,
    `it2_prev`, `it2data`, `save_it`, `save_it_data`, `it3`, `bottom_y`; `x_y_to_hpos_vpos`'s
    `x0`/`i` (C's `int x0, i;` is uninitialised) and its `glyph`/`end`/`row` pointers (C
    initialises `row` to `NULL` and leaves the other two to the loop).
14. **`iterators`' post-decrement loops.**  C's `while (nchars--)` is
    `while nchars != 0 { nchars -= 1; … }` (`string_pos_nchars_ahead`, `c_string_pos`), the
    `indent.rs`/`window.rs` shape; C's `for (nchars = 0; rest > 0; ++nchars)` is a `while rest
    > 0` with C's `nchars += 1` at the loop foot (`number_of_chars`).
15. **`it_bits!` gained the `line_number_produced_p` read arm** (byte 2217, bit 3); C spells
    the member directly and the port names it through the macro (PORTING.md 1.3/P-80), the
    `term.rs` arm verbatim.
16. **Three use-site declarations were needed** beyond the first round's list: `window.c`'s
    `window_parameter` (window.h 1209; the mirror carries the field, not the accessor),
    `SET_TEXT_POS_FROM_MARKER` (dispextern.h 322) and `ITERATOR_AT_END_OF_LINE_P`
    (dispextern.h 2954-2957) — each with C's header line in a `/// C:`/`/* C: … */` note.
17. **`adjust_window_ends`'s casts.**  C's `w->window_end_vpos = MATRIX_ROW_VPOS (…)` narrows
    the macro's `ptrdiff_t` to `int` (`as c_int`), and the ternary's two `*mut c_void`
    matrices take `as *mut glyph_matrix` for the macro (the mirror's degraded matrix pointer,
    P-72 rule 3).  `redisplay_counter` is an ordinary non-`static` global: this unit defines
    it `#[unsafe(no_mangle)]` and `eval.rs` declares it `extern` (the P-21 shape).
18. **`dsafe_eval_handler` and `funcall_with_backtraces` are `unsafe extern "C" fn`.**  Both
    are `static` helpers in C, but each one's address is handed out (`dsafe__call` passes the
    former to `internal_condition_case_n`; the `dsafe_calln` macro picks the latter as one arm
    of its ternary), so the port uses the `extern "C"` ABI for them — the existing
    `redisplay_trace`/`move_trace` rule.  The previous window declared `dsafe_eval_handler`
    `unsafe fn` because its caller did not exist yet; this round corrects the signature.
19. **`CHECK_IT`/`CHECK_WINDOW_END` are named no-ops.**  Both reference bodies are entirely
    dead text, so the live Rust body is empty; `extract-abi.py`'s no-stub rule rejects an
    empty Rust body outright (its `body_is_empty` still sees the `#if` directive line as live),
    so each keeps a `let _ = param;` under a comment naming the dead range — the `xfaces.rs`
    `clear_face_cache` / `dynlib.rs` `dynlib_addr` shape, and this file's own
    `IT_OVERFLOW_NEWLINE_INTO_FRINGE` treatment of a dead arm.
20. **`SAFE_CALLMANY`/`dsafe_calln`'s spelling.**  C's `countof (array)` is
    `countof! ($array) as ptrdiff_t` (C's implicit `size_t`→`ptrdiff_t` conversion) and the
    array argument is `$array.as_ptr () as *mut Lisp_Object` (C's array-to-pointer decay,
    which Rust does not do implicitly); C's compound literal
    `((Lisp_Object []) {__VA_ARGS__})` is a local array (the `lisp.rs` `CALLN!`/`safe_calln!`
    shape) and C's ternary becomes an `if`/`else` whose two arms both cast to C's
    function-pointer type.  `dsafe_calln` needs a `docs/ports/macro-body-notes.tsv` row for
    `__VA_ARGS__` (see the integrator list).
21. **Three new use-site declarations** for `internal_condition_case_n` (lisp.h 4808),
    `pop_handler` (lisp.h 4814) and `push_handler_bind` (lisp.h 4815).  eval.c defines them and
    lisp.h declares them, but the mirror chain this unit includes does not carry them
    (`keyboard.rs` declares its own copies for keyboard.c), so they keep C's prototypes in
    `/// C:` lines (PORTING.md 0.1-2/P-72 rule 3).  `internal_condition_case_n`'s two function
    parameters are the bare pointers C's prototype names — the handler is never NULL at these
    call sites.

### Deviations, continued (the round's third pass: `init_iterator` and the narrowing helpers)

22. **`struct it`'s pads were split into the members this round reaches.**  `stop_charpos`,
    `prev_stop`, `base_level_stop`, `end_charpos` and the four narrowing positions come out
    of the first pad (C 2446-2478), `dp` and `start` out of the second (C 2519-2551;
    `dpvec`, `dpend`, `dpvec_char_len`, `dpvec_face_id`, `saved_face_id` and `ctl_chars[16]`
    stay a pad), `cmp_it` is the measured `composition_it` record declared above `struct it`,
    `tab_width` (C 2795-2796, a `short` at 2394) comes out of `_pad_e`, and this round's
    `truncation_pixel_width`/`continuation_pixel_width` (C 2797-2799) and
    `paragraph_embedding` (C 2938-2941) out of their own pads.  Every new member carries the
    same `offset_of!` assertion as the first round's, and the bytes left over stay pads whose
    comments name the C range they cover.  `struct bidi_it` gets the same treatment for `w`
    (C 2296).
23. **`composition_it` is a measured partial record.**  C 2387-2428's `struct composition_it`
    is in `dispextern.h`'s un-mirrored part, so the unit declares it with `id` (8) and
    `parent_it` (48) named, sizeof 88 and the rest inside measured pads.
24. **`it_bits!` gained the write arms this round needs**: `multibyte_p`, `tab_line_p`,
    `header_line_p`, `face_box_p`, `selective_display_ellipsis_p`, `ctl_arrow_p` and
    `bidi_p`.  C's `it->multibyte_p = …` is `it_bits! (multibyte_p, it, …)` so the member
    name stays in the port's text (PORTING.md 1.3/P-80, deviation 10's rule);
    `IT_CTL_ARROW_P` (byte 2216, bit 1) is the new constant.
25. **`FRAME_WINDOW_CHANGE` and the frame `bool_bf` runs.**  C's
    `FRAME_WINDOW_CHANGE (it->f) = true` is an l-value; `include/frame.rs` carries only the
    read-only accessor of that name, so the unit defines the two-argument
    `FRAME_WINDOW_CHANGE!` macro (the `window.rs`/`frame.rs` copy) plus
    `set_frame_flag1!`/`set_frame_flag2!`, and extends its own `frame_bits!` with the
    `no_special_glyphs` and `face_change` arms.  C's `XFRAME (w->frame)->face_change = 0` is
    `frame_bits! (face_change, XFRAME ((*w).frame), 0)`, and C's `it->f->no_special_glyphs`
    is `frame_bits! (no_special_glyphs, (*it).f)`.
26. **`struct face`, `struct face_cache`, `FRAME_FACE_CACHE`, `FACE_FROM_ID_OR_NULL` and the
    `FACE_BOX_*` pair are this unit's own measured copies** (the `window.rs`/`font.rs`
    shape): no mirror in this unit's include graph carries them, and `include/dispextern.rs`
    has `FACE_FROM_ID` but not the NULL-returning sibling.  `face_bits! (box, face)` names
    C's member (deviation 10's rule) and its value is compared after an `as face_box_type`
    (C's enum conversion).
27. **Nine new use-site declarations** (§"Use-site declarations" 1 and 6): `face_change`,
    `free_all_realized_faces`, `lookup_basic_face`, `init_frame_faces`,
    `recompute_basic_faces`, `bidi_init_it`, `window_body_width`, `window_body_height` and
    `find_newline1`.  All are defined by other units and declared by headers whose mirrors do
    not carry them (PORTING.md 0.1-2/P-72 rule 3).
28. **C's implicit conversions.**  C's `int`/`short` stores and C's mixed-width comparisons
    became explicit `as c_int`, `as c_short`, `as ptrdiff_t`, `as EMACS_INT`, `as c_double`,
    `as usize` and `as face_box_type` casts: `it->extra_line_spacing` and
    `it->extra_line_spacing_above` (`int`, and the `XFLOAT_DATA (... ) *
    FRAME_LINE_HEIGHT (...)` products C truncates), `it->tab_width` (`short`),
    `it->truncation_pixel_width`/`continuation_pixel_width` (`short`, from C's
    `max (width, it->pixel_width)` — the port's `max` takes `isize`),
    `get_narrowed_len`'s `int` into `ptrdiff_t`, `dist` into `pos - dist`,
    `long_line_optimizations_bol_search_limit` into `ptrdiff_t`, the `XFIXNUM
    (Vtruncate_partial_width_windows) <= WINDOW_TOTAL_COLS (it->w)` comparison (C promotes
    the `int` to `EMACS_INT`), `first_vpos` into the `rows` pointer sum, and
    `window_wants_tab_line (w) + window_wants_header_line (w)` (C adds two `bool`s; Rust
    needs the two `as c_int`).
29. **`unwind_narrowed_begv` is `unsafe extern "C" fn`** (deviation 18's rule): it is
    `static` in C, but `SET_WITH_NARROWED_BEGV` hands its address to
    `record_unwind_protect`.
30. **Zero-initialised locals** (PORTING.md 1.5, deviations 1/13): `get_nearby_bol_pos`'s
    `found` (C passes `&found` to `find_newline1` without initialising it, and Rust will not
    take `&raw mut` of an uninitialised place) and `start_display`'s `it2`
    (`core::mem::zeroed ()`, the `pos_visible_p` shape).
31. **`in_ellipses_for_invisible_text_p`'s comma expression** inside the `if` condition is a
    Rust block expression (`&& { XSETWINDOW! (…); prop = …; TEXT_PROP_MEANS_INVISIBLE! (prop)
    == 0 }`), the `callproc.rs`/`terminal.rs` shape for C's comma operator.
32. **`SET_WITH_NARROWED_BEGV`'s `DST` assignment.**  C's macro stores through its `DST`
    parameter; the Rust `macro_rules!` keeps C's two statements with `$DST:expr` (Rust
    permits assignment to an `expr` fragment that is a place expression at the call site).
    `unwind_narrowed_begv` and `Fpoint_min ()` are the `record_unwind_protect` pair, and C's
    `IT->medium_narrowing_begv` truth test is `(*$IT).medium_narrowing_begv != 0` (Rust has
    no integer-as-bool).
33. **The scaffold block this round**: `start_display`'s entry is deleted (its definition
    landed) and four entries are added — `reseat` (C 1187), `produce_special_glyphs`
    (C 1240), `reseat_at_previous_visible_line_start` (dispextern.h 3619) and
    `update_redisplay_ticks` (dispextern.h 3721; the definition is at C 18574).  The last two
    are this unit's own later definitions whose C prototypes come from the un-mirrored
    `dispextern.h` — the reason `extract-abi.py` still prints its "is not static in C but has
    no `#[unsafe(no_mangle)]`" line for them (with `move_it_to`, `move_it_by_lines` and
    `add_to_log`).

34. **`struct bidi_it` now names `bytepos`/`charpos`/`string`, and `struct it` names
    `dpvec`, `n_overlay_strings`, `overlay_strings_charpos` and `saved_face_id`** (the
    "members this unit reaches are named" rule of §"Use-site declarations" 3): the two
    offsets measured for `string` (2256, after the 128-entry `level_stack`) and for the four
    `struct it` members (128, 384, 392 and 152) carry their own `offset_of!` assertions.
    `struct bidi_string_data`'s two `bool_bf : 1` members are the new
    `bidi_string_bits!` macro over the mirror's `bits: u32` (the `bidi.rs` copy).
35. **`it_bits!` gained the `selective_display_ellipsis_p`, `ignore_overlay_strings_at_pos_p`,
    `ellipsis_p`, `from_disp_prop_p` and `string_from_display_prop_p` write arms and the
    `selective_display_ellipsis_p`/`ignore_overlay_strings_at_pos_p` read arms** (deviation
    10's rule; `IT_IGNORE_OVERLAY_STRINGS_AT_POS_P` is byte 2216, bit 6).  C's
    `it->ignore_overlay_strings_at_pos_p` is read once, `it->ellipsis_p` read and written,
    and the other three written.
36. **`while (n--)` in `init_from_display_pos`** is `while n != 0 { n -= 1; … }` (deviation
    14's rule).
37. **C's implicit `int`/`ptrdiff_t` mixing in this round** became explicit casts:
    `init_from_display_pos`'s `i as ptrdiff_t` (C compares `int i` against the `ptrdiff_t`
    `n_overlay_strings`) and `relative_index as c_int` (the `%` yields `ptrdiff_t`, C stores
    `int`), and `init_to_row_end`'s `(*row).continuation_lines_width + (*row).pixel_width`
    (both `int`).
38. **Bare `lvalue` pointer casts and no-op parameters.**  `init_from_display_pos`'s
    `SSDATA (...)` is cast `as *const c_char` for C's `const char *s`
    (`SDATA`/`SSDATA` return `char *`), and `init_to_row_start`'s void call keeps C's
    `init_from_display_pos (it, w, &row->start);` spelling (the discarded `bool`).
39. **C's uninitialised locals stay uninitialised** where every path assigns before use
    (PORTING.md 1.5's exception; the `start_display` `new_x` shape): `init_from_display_pos`'s
    `i` (assigned by the `for` init), and `handle_stop`'s `handled`, `handle_overlay_change_p`
    and `p` (assigned at the head of the `do` body / the `for` init).  None needed
    `std::mem::zeroed ()`.
40. **The scaffold block this round**: twenty-five entries now — the seven added are
    `handle_overlay_change` (C 1007), `setup_for_ellipsis` (C 1126), `pop_it` (C 1148),
    `get_overlay_strings_1` (C 1185), `load_overlay_strings` (C 1197),
    `get_visually_first_element` (C 1202) and `compute_stop_pos` (C 1203), all this unit's
    own later definitions with C's forward-declaration prototypes.  `bidi_move_to_visually_next`
    (dispextern.h 3551, bidi.c) is *not* scaffold: it joins the unit's `unsafe extern "C"`
    block (deviation 27's rule).

### Deviations, continued (the round's fourth pass: the text-property stop and the display-string scan)

41. **`struct composition_it` now names `stop_pos` and `struct bidi_it` now names
    `resolved_level`** (the "members this unit reaches are named" rule of deviations 22/34):
    the `_pad_a` runs are split at the measured offsets 0 and 52, each new member carries its
    own `offset_of!` assertion, and the bytes left over stay pads whose comments name the C
    range they cover.  `compute_stop_pos` reads C's `it->cmp_it.stop_pos` (C 2389, the first
    member of `struct composition_it`) and `it->bidi_it.resolved_level` (C 2176, a
    `signed char`).
42. **The `MAX_DISP_SCAN` macro is a `c_int` constant** (`pub const MAX_DISP_SCAN: c_int =
    250;`, the `TEXT_PROP_DISTANCE_LIMIT`/`MAX_SCRATCH_GLYPHS` shape), and both of its uses
    take an explicit `as ptrdiff_t` for C's implicit `int`-to-`ptrdiff_t` promotion, as do
    `compute_stop_pos`'s three `TEXT_PROP_DISTANCE_LIMIT` sums (including the `10 *` product),
    per deviation 28.
43. **C's `values_here` array.**  C leaves `Lisp_Object values_here[LAST_PROP_IDX]`
    uninitialised; the port writes `[lisp_h_Qnil (); LAST_PROP_IDX as usize]` (the
    `keyboard.rs` `keybuf` shape).  Only the five `it_props` entries that carry a handler are
    ever stored to or read from; C's own two loops never touch the sixth slot
    (`OVERLAY_PROP_IDX`) either.
44. **The `do { … } while (NILP (spec) || !(rv = handle_display_spec (...)))` loop in
    `compute_display_string_pos`** is the port's `loop { …; if !(NILP (spec) || { rv = …;
    rv == 0 }) { break; } }` (deviation 36's `while` shape and deviation 31's
    comma-expression shape), and C's `&& (rv = handle_display_spec (...))` truth test is
    `&& { rv = handle_display_spec (...); rv != 0 }`.  `handle_display_spec` is called with
    C's `NULL` iterator (`core::ptr::null_mut ()`), and C's `spec = …` inside the `if`
    condition is the port's `{ spec = …; spec }` block (the `handle_stop` shape).
45. **C's `'\f'`** is spelled `b'\x0c'` (the `print.rs` precedent; Rust has no `\f` byte
    escape).
46. **The `handle_display_spec` scaffold prototype's parameter names were corrected** to C's
    definition's (C 5999: `spec`, `object`, `overlay`, `position`, `bufpos`,
    `frame_window_p`); the third pass's entry had shifted the four names after `spec` one
    place.  The types and the order were already right, so no call site changed and the
    scaffold entry count is unaffected by this correction.
47. **The scaffold block this round**: `compute_stop_pos`'s entry is deleted (its definition
    landed), leaving twenty-four entries.  `bidi_level_start` (dispextern.h 3559, bidi.c) is
    *not* scaffold: it joins the unit's `unsafe extern "C"` block (deviation 27's rule) and is
    named in the integrator to-do list below.
48. **C's `register` storage class is dropped** where it appears in the transcribed range
    (`compute_stop_pos`'s `register INTERVAL iv, next_iv;`, C 4332; C 1178's is inside a
    forward declaration the port drops per PORTING.md 0.1-12).  The keyword has no Rust
    counterpart and no semantic effect; the rest of the unit's ranges carry it too, and this
    is the first round to write the convention down.

### Deviations, continued (the round's fifth pass: the `'display' property`)

49. **`struct it` now names `xwidget`, `slice`, `space_width`, `voffset` and `font_height`**
    (the "members this unit reaches are named" rule of deviations 22/34/41): the 50-byte
    `_pad_e` and the 12-byte `_pad_e1` runs are split at the offsets clang measured on the
    frozen target (2344/2352/2384/2392/2394/2400/2408), each new member carries its own
    `offset_of!` assertion, and the bytes left over stay pads.  `xwidget` is C's
    `struct xwidget *`; `HAVE_XWIDGETS` is undef, so that record is incomplete and the port
    keeps the pointer as `*mut c_void` (the P-72 rule 3 shape that `terminal.rif` and
    `struct font`'s `driver` already use); the conversion disappears once the record has a
    mirror.
50. **C's narrowing assignments became explicit Rust casts** (deviation 3's rule): the
    `(height …)` arm's `int steps = XFIXNUM (…)` and its three `new_height = XFLOATINT (…)` /
    `(XFLOATINT (…) * XFIXNUM (…))` stores are `as c_int` (the product puts `as f64` on the
    `EMACS_INT` factor, the `window.rs` `POINT_TO_PIXEL` shape) — `--check` reports the casts.
51. **C's chain assignments became one statement per place** (deviation 2's rule, in C's
    textual order as `init_iterator`'s C 3464/3936/3941 already do): `it->slice.x = it->slice.y
    = it->slice.width = it->slice.height = Qnil`, `IT_STRING_CHARPOS (*it) = IT_STRING_BYTEPOS
    (*it) = 0`, `it->end_charpos = it->string_nchars = SCHARS (it->string)` and `*position =
    it->position = start_pos` each became their members' statements.
52. **Two `text_pos` locals are zero-initialised** (PORTING.md 1.5, deviation 1's rule):
    `display_prop_end`'s `end_pos` (C's `CHARPOS`/`BYTEPOS` writes and `compute_string_pos`
    fill it) and `display_prop_intangible_p`'s `position` (C's `SET_TEXT_POS`); Rust has no
    partial initialisation, which `build-tu` rejected on the first attempt.
53. **`handle_single_display_spec`'s `object` parameter is `mut`** because C's code assigns it
    (`XSETBUFFER (object, current_buffer)` when the object is nil — the `c_string_pos` /
    `pixel_to_glyph_coords` shape), and `lookup_xwidget`'s `eassume (0)` is `eassume! (false)`
    (PORTING.md 5.2's int-literal-to-bool rule).  C's `if (CONSP (prop)) if (EQ (…)) …` in
    `single_display_spec_string_p` keeps an explicit block for the inner `if` (Rust's `if`
    bodies are blocks, not arbitrary expressions).

### Deviations, continued (the round's sixth pass: the overlay strings and the iterator stack)

54. **`struct it`, `struct composition_it` and `struct iterator_stack_entry` name the members
    this round reaches** (deviations 22/34/41/49's rule): the `struct it` bit
    `overlay_strings_at_end_processed_p` (byte 2216 bit 5, through a new `it_bits!` arm),
    `composition_it`'s `ch` (16) and `nglyphs` (40), and the stack entry's
    `string_nchars`/`end_charpos`/`prev_stop`/`base_level_stop`/`cmp_it`/`u`/`position`/
    `area`/`paragraph_embedding`/`line_wrap`/`voffset`/`space_width`/`font_height`, whose
    50-byte and 12-byte pads are split at the clang-measured offsets.  Every new member and
    the two new record types carry `offset_of!`/`size_of!` assertions.
55. **The stack entry's anonymous `u` union is three named `#[repr(C)]` records
    (`iterator_stack_entry_u_image`/`_stretch`/`_xwidget`) inside a `#[repr(C)] union`
    (P-67's rule for anonymous records; the `Lisp_Symbol_u` family in `lisp.rs` is the
    precedent).**  `push_it`/`pop_it` reach `p->u.image.object`, `p->u.image.slice`,
    `p->u.image.image_id`, `p->u.stretch.object` and `p->u.xwidget.object`, all spelled as in
    C; `it->xwidget`'s `*mut c_void` (deviation 49) is only ever stored through the `it`
    member, never through the union.
56. **`RECORD_OVERLAY_STRING` is a function-scoped `macro_rules!`** (PORTING.md P-37's
    `do { … } while (false)` rule).  Rust's macro hygiene does not let a macro body see the
    caller's locals, so the three places it mutates (`entries`, `n`, `size`) are passed in as
    `:ident` parameters — the same reason `SAFE_NALLOCA!` takes `sa_avail` — while the C
    macro's three parameters (`OVERLAY`, `STRING`, `AFTER_P`) stay `:expr`.  C's `#define`
    (C 7121) and `#undef` (C 7182) have no Rust counterpart and are dropped (like C's forward
    declarations, PORTING.md 0.1-12).
57. **`switch (method)`became `match`** in `push_it` and `pop_it`, with C's `default:
    emacs_abort ();` as `_ => { emacs_abort (); }` and C's fall-through arms as `|` patterns
    (`GET_FROM_BUFFER | GET_FROM_DISPLAY_VECTOR | GET_FROM_STRING | GET_FROM_C_STRING =>
    { }`); `--check` reports `switch`, `case`, `default` and `break` for both, and the four
    rows are listed in the Progress section for the integrator (PORTING.md 2.2).
58. **`ITREE_FOREACH (…, DESCENDING)` is expanded at its use site** (P-33, the status 3.7
    decision): C's `if (!t) { } else for (struct itree_iterator …)` becomes the same
    `if … .is_null ()`/`while { node = itree_iterator_next (…); … }` block `buffer.rs`'s
    `overlay_touches_p` writes, and `enum itree_order`'s members are imported with
    `use itree_order::*;` because C spells them bare (the `buffer.rs` shape).  `--check`
    reports `descending` and `itree_foreach` for `load_overlay_strings` (row listed above).
59. **C's narrowing conversions in this range became explicit casts** (deviation 3/50's
    rule): `it->string_nchars` (a `ptrdiff_t`) into the stack entry's `int string_nchars` and
    back (`as c_int`/`as ptrdiff_t`), `COMPOSITION_LENGTH (prop)` (an `EMACS_INT`) into
    `cmp_it.nchars` (`as c_int`), `*s` (an `unsigned char`) into `it->c` (`as c_int`), and
    the chunk index `it->current.overlay_string_index % OVERLAY_STRING_CHUNK_SIZE`
    (`as ptrdiff_t` on the constant, then `as c_int`).
60. **Rust-side initialisers Rust needs where C has none:** `entriesbuf` (C leaves the
    20-element array uninitialised) is `core::mem::zeroed ()` (deviation 43's `values_here`
    shape), and in `handle_composition_prop` the three out-parameters `prop`, `start` and
    `end` — which C leaves to `find_composition` and then reads in the `&&` chain — are
    zero/`Qnil`-initialised because Rust cannot prove the call wrote them (deviation 1's
    rule).  `string_buffer_position`'s `const int MAX_DISTANCE = 1000;` is the local
    `let MAX_DISTANCE: c_int = 1000;`, and `compare_overlay_entries` (C's `static` whose
    address goes to `qsort`) is `unsafe extern "C" fn` without `#[unsafe(no_mangle)]`
    (deviation 18's rule).

### Deviations, continued (the round's seventh pass: moving over lines and the reseats)

61. **Two `goto` destinations became a named loop plus a named block.**  In
    `back_to_previous_visible_line_start` C's `break` (C 7871) exits the `while` while its two
    `goto replaced` (C 7848, 7866) jump to the `replaced:` label at the end of the same
    block, so the port names the loop `'next_line` and the block `'replaced` and writes
    `break 'next_line` / `break 'replaced` (PORTING.md 2.2).  In
    `lookup_glyphless_char_display` the backward `goto retry` is the port's labelled
    `'retry: loop` with `continue 'retry`.  `--check` reports `goto` for both functions and
    the two rows are listed in the Progress section.
62. **`ITREE_FOREACH (…, DESCENDING)` in `strings_with_newlines`** is the same use-site
    expansion as `load_overlay_strings` (P-33, deviation 58); `--check` reports
    `descending`/`itree_foreach` and the row is listed above.
63. **`struct it`'s `glyphless_method` and `struct bidi_it`'s `disp_pos`/`disp_prop` are
    named** (deviations 22/34/41/49/54's rule): `enum glyphless_display_method` is spelled at
    the use site (the `term.rs` copy, C 2347-2358) because the mirror chain does not carry it,
    and the new `it` bit `face_before_selective_p` (byte 2217 bit 1) gets its own `it_bits!`
    arm.  clang against the frozen target measured `glyphless_method` 2332, `disp_pos` 192 and
    `disp_prop` 200 (the last two sit behind the four-byte hole after `scan_dir`), and each
    new member carries its `offset_of!` assertion.
64. **The `next_element_function` typedef and its `get_next_element[]` table.**  C's
    `typedef bool (*next_element_function) (struct it *);` is
    `Option<unsafe extern "C" fn (it: *mut it) -> bool>` (the `props.handler` /
    `redisplay_interface.produce_glyphs` shape) and the `static ... const` table is a plain
    `static` array of `Some (…)`; `GET_NEXT_DISPLAY_ELEMENT` unwraps at the call (C's `(*)`
    dereference).  `tools/cinventory` tracks neither the typedef nor the table, so no ledger
    box covers them — they are transcribed as text (reported here).
65. **C's implicit conversions in this range** (deviation 3/50's rule): `n += !STRINGP
    (it->string)` is `n += (! STRINGP ((*it).string)) as c_int`, `strlen (s)` is
    `strlen (s) as ptrdiff_t`, and `field_width > it->end_charpos - charpos` (an `int`
    compared with a `ptrdiff_t`) takes `field_width as ptrdiff_t`.  `buf_charpos_to_bytepos`
    is called with `current_buffer as *mut c_void` (the mirror's opaque pointer),
    `composition_compute_stop_pos` takes `&raw mut (*it).cmp_it as *mut c_void` (the
    existing call site's spelling), and `XCHAR_TABLE (...)->extras[0]` is the mirror's
    flexible-array accessor `FAM_ELEM! (…, 0)` (the `buffer.rs`/`casefiddle.rs` shape).
66. **Rust-side initialisers Rust needs where C has none** (deviations 52/60's rule):
    `forward_to_next_line_start`'s `bytepos` (an out-parameter of `find_newline_no_quit`),
    `reseat_at_next_visible_line_start`'s `bidi_it_prev`, and the six locals of
    `back_to_previous_visible_line_start`'s inner block (`it2`, which `SAVE_IT` fills in,
    plus `it2data`/`pos`/`beg`/`end`/`val`/`overlay`).  `it2.sp = 0` is written on the value
    (C's `it2.sp = 0`), and C's `pos = --IT_CHARPOS (it2)` keeps one statement as
    `pos = { IT_CHARPOS! (it2) -= 1; IT_CHARPOS! (it2) };`.  `forward_to_next_line_start`'s
    `for (n = 0; …; n += …)` is the port's `while` with C's increment at the loop foot, and
    C's `*skipped_p = newline_found_p = true` is two statements in C's textual order
    (deviation 51).

### Deviations, continued (the round's eighth pass: the display element and the method switch)

67. **Three C labels became labelled loops and blocks.**  In `get_next_display_element`, C's
    `get_next:` label with three backward `goto get_next` is the labelled `'get_next: loop`
    (`continue 'get_next`), its two `goto done` leave that loop for the `done:` block below
    it, and the `display_control:` label (the epilogue of the control-character block, C 8680)
    is a labelled block whose four forward `goto display_control` sites `break` to it.  In
    `set_iterator_to_next`, C's `switch` becomes a `match` inside a labelled block (the arms'
    `break`s are `break 'consider_string_end`), and the shared `consider_string_end:` label
    is the loop that follows the match: the arms that `goto` it fall out of the match into
    it, and its own backward `goto consider_string_end` (C 9220) is a `continue`; the
    GET_FROM_STRING arm's two early `goto`s skip the rest of the arm through an inner
    labelled block (`'string_arm`).  `--check` reports `goto`/`done` for the first and
    `switch,case,default,goto` for the second, and both rows are listed above.  **This is
    also where item 22's list is corrected**: it said `ctl_chars[16]` stays a pad; this round
    names it, at the clang-measured offset **160** (the older comment's "156" was wrong —
    the four bytes before it are the alignment hole after `saved_face_id`).
68. **`struct bidi_it`'s `type` is `r#type`** in the port: C's member name is a Rust
    keyword, so the port uses the raw identifier (`alloc.rs`'s `mem_node.r#type` is the
    precedent); `struct composition_it`'s `reversed_p` (44), `from` (72) and `to` (76) come
    out of their pads at the clang-measured offsets (deviations 22/34/41/49/54/63's rule).
    `it_bits!` gains the `ctl_arrow_p` read arm and `bidi_it_bits!` the `new_paragraph` one.
69. **C's implicit conversions in this range are written out** (deviations 3/50/59/65's
    rule): `it->dpvec_char_len = it->len` and `it->len = it->dpvec_char_len` narrow/expand
    between `int` and `ptrdiff_t` (`as c_int`/`as ptrdiff_t`), the six `XSETINT (it->ctl_chars
    […], x)` sites whose `x` is an `int` take `as EMACS_INT` (C's promotion into the two-argument
    macro), `IT_CHARPOS (*it) += it->cmp_it.nchars` (and the three siblings) take
    `as ptrdiff_t`, and `IT_STRING_CHARPOS (*it) + it->cmp_it.nchars` in the box-run test
    takes the same cast.
70. **The `sprintf` formatter is the port's C-string literal + variadic call** (the
    `ascii.rs`/`buffer.rs` shape): `char str[10]` is `let mut str: [c_char; 10] = [0; 10];`,
    C's two format spellings are `c"x%02x".as_ptr ()`/`c"%03o".as_ptr ()`, and the argument
    C writes as `c + 0u` is `c as c_uint` (the variadic promotion).  `str[i]` (a `char`) is
    `str[i as usize] as c_int` — C's own two-step promotion into the `XSETINT` macro.

### Deviations, continued (the round's ninth pass: the element producers)

71. **`struct it`'s `len` is an `int`, and the unit's copy had it as `ptrdiff_t`.**  C
    declares `int c, len;` (dispextern.h 2733) and clang measures `c` at 2228 and `len` at
    2232, both four bytes, with the four-byte alignment hole 2236-2240 before `cmp_it`; the
    copy's eight-byte `len` happened to keep every later offset, which is why the defect
    survived five rounds — but it also kept `&it->len` from meeting
    `check_char_and_length`'s `int *`.  This round writes `len: c_int` plus `_pad_d0: [u8; 4]`
    (the offsets and the 4,968-byte size are unchanged), drops the two `as c_int` casts and
    the `as ptrdiff_t` cast the earlier rounds had added around `len`, and adds the
    `as ptrdiff_t` casts the C `ptrdiff_t += int` sites now need.  **The same correction
    belongs in `include/dispextern.rs` when `struct it` gets a mirror there.**  (The
    `ctl_chars` offset that deviation 67 corrected was the neighbouring instance of this
    class: the numbers must come from clang, not from the inherited prose.)
72. **The seven `next_element_from_*` that `get_next_element[]` names carry the C ABI.**
    C's are `static`, but their addresses go into the table, so the port writes them
    `unsafe extern "C" fn` without `#[unsafe(no_mangle)]` (deviation 18's rule, the
    `it_props[]` handler shape); `next_element_from_ellipsis` and
    `next_element_from_composition`, which no table names, stay plain `unsafe fn`.
    The `CHAR_COMPOSED_P` macro's `composition_reseat_it` call takes the mirror's narrowed
    parameter types (`*mut c_void` for the two pointers, `c_char` for the bidi level) with
    C's conversions written out.
73. **`FETCH_BYTE` yields a `u8` and `SREF` takes a `usize`** (the `include/buffer.rs` and
    `include/lisp.rs` mirrors), so this round's comparisons are `FETCH_BYTE! (…) == b'\n'`
    and its two byte reads are `SREF (string, idx as usize) as c_int` (C's `char` to `int`
    promotion); two older `eassert` sites still spelled `== b'\n' as c_int` (they are
    compiled only under `enable_checking`, which is why the mismatch had never been
    reported) and were brought to the same spelling.  `(*it).dpend.offset_from ((*it).dpvec)`
    is C's pointer difference; its cast to `isize` keeps a leading `<` off the start of a
    line, which Rust would read as generics.
74. **Two `text_pos` locals are zero-initialised** (`compute_stop_pos_backwards`'s `pos` and
    `handle_stop_backwards`'s `pos1`, both filled by `SET_TEXT_POS`) — PORTING.md 1.5,
    deviation 1's rule.

### Deviations, continued (the round's tenth pass: `move_it_in_display_line_to`)

75. **C 10187 spells `(op && MOVE_TO_X)`, a *logical* AND, not the bit test `op & MOVE_TO_X`
    that every other site in the function uses.**  `MOVE_TO_X` is `0x01` (dispextern.h 2980),
    so C's operand evaluates to `op != 0`: with `op == MOVE_TO_POS` alone (0x08) and
    `to_x == it->first_visible_x`, C *does* consider producing a line number, while the bit
    test would not.  Rust's `&&` takes `bool`, so the C spelling cannot be written at all;
    the port writes `op != 0 && MOVE_TO_X != 0 && to_x == (*it).first_visible_x`, which keeps
    C's truth value with both operands named (noted at the site).  This is the only
    `op && …` in the file — C 10394, 10601, 10665, 10886 and 11001-11002 all use
    `op & MOVE_TO_X`.
76. **The two C labels become a labelled block and a labelled loop** (PORTING.md 2.2).  C's
    `done:` (C 10855), entered by `goto done` at C 10308 and C 10314 — whose whole point is to
    skip the wrap/atpos/atx restore chain C 10829-10853 — is the port's `'done: { … }` with
    `break 'done`.  C's `buffer_pos_reached:` (C 10660) is a leaf of the `if (it->nglyphs)`
    chain that four forward `goto`s enter (C 10399, 10627, 10698, 10702) and that its own
    `else if (BUFFER_POS_REACHED_P ())` arm falls into (C 10658); the port wraps the loop
    body in `'buffer_pos_reached: { … continue 'scan; }` and puts the label's three
    statements (C 10661-10663) after the block, so a `goto` becomes `break 'buffer_pos_reached`
    and the fall-through arm becomes a block that only `break 'buffer_pos_reached`s.
    Rust 2024 rejects an unlabelled `break`/`continue` that would diverge through a labelled
    block (E0695): the eleven statements inside the block that C's plain `break` sends to the
    `while (true)` were therefore spelled `break 'scan` / `continue 'scan` (C 10245, 10266,
    10283, 10359, 10656, 10673, 10725, 10794, 10799, 10814, 10817 — the label body's own
    `break` at C 10663 was already labelled) — same target, C's own loop.
77. **The three macros are `macro_rules!`** (PORTING.md P-17).  `BUFFER_POS_REACHED_P`
    (C 10165) and `IT_RESET_X_ASCENT_DESCENT` (C 10215) are defined *inside* the function, at
    C's own positions, and read the enclosing `it`/`op`/`to_charpos` and `x`/`ascent`/`descent`
    from the environment — a Rust macro is hygienic, so C's `(IT)` parameter cannot be
    mirrored the way C passes `&atpos_it`.  `IT_POS_VALID_AFTER_MOVE_P` (C 10071) is
    unit-level, next to `ITERATOR_AT_END_OF_LINE_P`, because C uses it outside this function
    (C 11332, 11532, 11581, 11587 — all past this round's boundary).
78. **The four `struct it` snapshots and `closest_pos` are zero-initialised.**  C leaves
    `wrap_it`/`atpos_it`/`atx_it`/`ppos_it` to `SAVE_IT` and declares
    `ptrdiff_t closest_pos UNINIT` (C 10120, `UNINIT` = `= {0,}` in the frozen
    configuration); Rust has no partially-initialised locals, so the port spells them
    `core::mem::zeroed ()` / `0` (PORTING.md 1.5, deviation 1's rule).  Their `sp` fields are
    set to `-1` immediately after, exactly as C does.
79. **`constrain_row_ascent_descent_p` reaches the unit's `it_bits!` for the first time.**
    C 10724 writes `it->constrain_row_ascent_descent_p = false;` — a `bool_bf : 1` member
    (dispextern.h 2697-2699) that lives in the port's measured byte 2217 at bit 2, between
    `face_before_selective_p` (bit 1) and `line_number_produced_p` (bit 3).  The mask
    constant `IT_CONSTRAIN_ROW_ASCENT_DESCENT_P` had been in the block since the layout work;
    this round adds its two `it_bits!` arms (read and write), the first body to need either.
80. **`struct composition_it` gains `charpos`.**  C 2419-2420 declares
    `ptrdiff_t charpos;` and C 10174-10176 reads `it->cmp_it.charpos`; the unit's measured
    copy had it inside the `_pad_c0: [u8; 8]` hole at offset 56, so this round names the
    member and pins it with a new `offset_of! == 56` assertion (the following `nchars` at 64
    and the 88-byte size are unchanged).
81. **The ten `#ifdef GLYPH_DEBUG` blocks in this range are dead** (P-08; `GLYPH_DEBUG` is off
    in `docs/baseline/config/darwin-arm64.macros.tsv`): C 10128-10132 (the
    `atx_flag`/`atpos_flag`/`wrap_flag` declarations), 10253-10255, 10318-10320, 10404-10406,
    10421-10423, 10519-10521, 10631-10633, 10835-10837, 10842-10844 and 10849-10851 — each
    recorded at the site with its C line range and not transcribed.
82. **`move_trace`'s first call in this unit** (C 10618) — a non-`static`-looking C `static`
    function whose variadic body the earlier rounds already landed (C 962); the call passes
    C's format literal as `c"…".as_ptr ()`.  C 10671's `eassert (it->nglyphs == 0)` uses the
    unit's `eassert!`.
83. **The burned scaffold entry carried a stale C line number.**  Its comment said
    `C: xdisp.c 1204-1206`; C's prototype for `move_it_in_display_line_to` is at C 1199-1201
    (C 1204-1206 is `face_before_or_after_it_pos`).  The entry is gone, but the neighbouring
    `/// C:` lines should be spot-checked against the file rather than trusted (the
    3.153/3.156/3.157/3.169 class).
84. **C 10655-10656's guard is a `for`-body tail, and the port keeps it where it is.**  Its
    `break` leaves the `for` (C 10389-10657), not the scan loop; every assignment to `result`
    inside that body (C 10414, 10510, 10549, 10557, 10568, 10577, 10607) is immediately
    followed by its own `break`, so the guard is never taken.  The port leaves the statement
    inside the `if (nglyphs)` arm and notes at the site why breaking the scan loop there is
    the same unreachable outcome (Rust has no unlabelled `for`-break inside the labelled
    block that C's `break` would be).

### Deviations, continued (the round's eleventh pass: the `move_it_*` cluster)

85. **C's `out:` label is the same target as its loop `break`.**  C 11227's `out:` is the
    first statement after the `for (;;)` body (closed at C 11225), so `move_it_to`'s two
    `goto out` (C 11130, 11163) and its plain `break`s land in the same place: the port writes
    `break` for both and needs no labelled block.  The `switch (skip)` (C 11125) becomes a
    `match` whose two `goto out` cases `break` the scan loop, while C's per-case `break`
    (which leaves the *switch*) is simply the end of the Rust arm — the distinction the
    `--check` `does not mention case, default, switch` row records.
86. **`while (nlines-- && …)` keeps C's post-decrement.**  In `move_it_vertically_backward`
    (C 11298) the decrement runs on *every* test, including the one that ends the loop, so the
    port spells the condition out as an inner `loop` with `let old = nlines; nlines -= 1;`
    rather than a Rust `while` (same value of `nlines` after the loop as C leaves).
87. **`move_further_back:` is a labelled loop.**  C 11283's label is jumped to once from the
    `else` arm (C 11416), so the port writes `'move_further_back: loop { … }` with
    `continue 'move_further_back` and a `break` where the body ends (PORTING.md 2.2, the
    `lookup_glyphless_char_display` `'retry` shape); the function's `dy == 0`/`dy > 0` arms
    are the loop body's tail.
88. **C's chained assignments are separate statements, in C's own order.**  `it->current_x =
    it->hpos = it->max_ascent = it->max_descent = 0` (C 11247) and the two
    `it->current_x = it->hpos = it->wrap_prefix_width = 0` (C 11596, C 11605) are written
    left-to-right as C spells them; every target ends at the same value either way.
89. **`struct face` gains `font` at offset 216.**  C 1828 declares `struct font *font;`
    between the color members and `fontset`; the unit's copy had 164-228 as one 64-byte pad,
    so the round splits it (52 + the pointer + the 4-byte `fontset` hole) and pins 216 with a
    new `offset_of!` assertion.  Measured with clang on the frozen target
    (`clang -std=gnu23 -O0 -I build/emacs-ref/lib -idirafter spikes/shim …`): sizeof 280,
    `id`@160, `font`@216, `box_vertical_line_width`@228 — the same 280 the existing assertion
    already required.
90. **`it_bits!` gains the `line_number_produced_p` write arm** for C 11220's
    `it->line_number_produced_p = false;` (byte 2217 bit 3, between
    `constrain_row_ascent_descent_p` and `align_visually_p`); only the read arm existed, from
    the ninth round.
91. **`int`/`ptrdiff_t` mixes are spelled out.**  `move_it_by_lines`'s
    `it->vpos + dvpos` (C 11531, an `int` plus a `ptrdiff_t` passed as `int`) is
    `((*it).vpos as ptrdiff_t + dvpos) as c_int`, and the comparisons
    `it2.vpos > -dvpos` (C 11608), `it->vpos - it2.vpos > delta` (C 11617) and
    `it2.vpos < -dvpos` (C 11623) promote to `ptrdiff_t` the same way; the two
    `max (…)`/`min (…)` calls go through `include/lisp.rs`'s `isize` helpers with C's
    conversions written out (deviation 73's rule).
92. **The `move_it_*` prototypes the scaffold carried are now definitions**, so the block's
    four entries are burned and the four bodies carry `#[unsafe(no_mangle)] pub unsafe
    extern "C" fn` (C's are non-`static`: C 10880, 10927, 11272, 11459, 11483, 11502).

### Deviations, continued (the round's twelfth pass: `window_text_pixel_size` and the log)

93. **C 12458's `char dupstr[sizeof " [ times]" + INT_STRLEN_BOUND (dups)]` is 30 bytes.**
    Measured with clang on the frozen target (a probe including `lisp.h`, which is where
    `INT_STRLEN_BOUND` becomes visible — it is not in any of the headers the tree mirrors):
    `sizeof " [ times]"` = 10, `INT_STRLEN_BOUND (intmax_t)` = 20, so the port writes
    `let mut dupstr: [c_char; 30] = [0; 30];` with the arithmetic in a comment.
94. **`sprintf`'s `"%"PRIdMAX" times]"` is the literal `" [jd times]"`.**  The frozen target's
    `PRIdMAX` is `"jd"` (measured by the same probe; the `filelock.rs` `const PRIdMAX` row has
    the same value), and C's token paste is a compile-time string concatenation, so the port
    writes the joined literal with the measurement noted at the site.
95. **`add_to_log`'s `va_list`/`va_start`/`va_end` are the port's `...` parameter and `VaList`**
    (PORTING.md P-04, the `move_trace`/`redisplay_trace` shape): C's three macro calls have no
    Rust counterpart, which is what the `--check` `does not mention va_end,va_list,va_start`
    row records.
96. **`Rust has no uninitialised locals`, so two `Qnil`s appear where C writes none**: DEFUN
    `buffer-text-pixel-size`'s `Lisp_Object buffer` (C assigns it through `XSETBUFFER` before
    any read) and `vadd_to_log`'s `Lisp_Object args[10]` (C fills every element it reads) are
    zero-initialised here (PORTING.md 1.5, deviation 1's rule).  That is the whole of the
    identifier-count difference for this round (`Qnil` C 12 : Rust 14; every other name in the
    range matches exactly).
97. **`INT_MAX` is a `c_int` constant, not a macro**, and `BEG`/`BEG_BYTE` are `pub const`s in
    `include/buffer.rs` (not the `BEGV!`/`ZV!`-style macros), so C's `int` uses become
    `INT_MAX` and `BEG as ptrdiff_t` / `BEG_BYTE as ptrdiff_t` where a `ptrdiff_t` is wanted.
98. **The `buf`-state writes keep C's own l-values**: `BEGV = BEG;`, `ZV = Z;`,
    `TEMP_SET_PT_BOTH (Z, Z_BYTE);` are the port's `BEGV! () = BEG as ptrdiff_t;`,
    `ZV! () = Z! ();` and `TEMP_SET_PT_BOTH (Z! (), Z_BYTE! ());` — the read macros expand to
    `(*current_buffer).field`, so they are places (the tenth round's `it_bits!` note's
    counterpart).
99. **`unchain_marker` takes `*mut c_void` in the mirror** (lisp.h 4119's declaration is
    `struct Lisp_Marker *`), so the three calls cast `XMARKER (oldX) as *mut c_void`
    (`editfns.rs`'s spelling).
100. **`window_end_valid` is a packed bit**: DEFUN `buffer-text-pixel-size`'s
     `int end_valid = w->window_end_valid;` read is `if window_bits! (w, window_end_valid)
     { 1 } else { 0 }` and its write is `window_bits! (w, window_end_valid, end_valid != 0)`
     (byte-2217-family writer the unit already had).  `wset_buffer` is a real extern
     (window.h 1231), unlike the `bset_*` inline writers the mirror carries.
101. **`in_display_vector_p`'s pointer sum is `offset`**: C compares
     `it->dpvec + it->current.dpvec_index != it->dpend`, the port
     `(*it).dpvec.offset ((*it).current.dpvec_index as isize) != (*it).dpend` (the
     `dpvec_char_len` site's shape from the tenth round).
102. **The three `DEFUN`s keep C's `doc:` text verbatim** (as one Rust string literal with
     `\n` escapes, generated from C's comment by the round's own script — the subr-box
     `doc <hash>` column is `extract-abi.py`'s digest of exactly that text).

103. **`lookup_xwidget`'s unreachable tail aborts rather than panicking.**  C's `INLINE struct
     xwidget *lookup_xwidget (Lisp_Object obj) { eassume (0); }` (xwidget.h 237) has no
     `return`, and the port had spelled the tail `unreachable! ()` — which
     `tools/extract-abi.py`'s `STUB_RE` (`unimplemented!|todo!|unreachable!\s*\(\s*\)\s*$`)
     reads as a *stub* and would have reported once the surrounding function stopped being
     skipped.  It is now `emacs_abort ()`, the `include/frame.rs` 240 shape the rest of the
     tree uses for "cannot happen" (the `eassume! (false)` above already states the C
     assumption).  **This is the class of finding the platform-header lane caught**; no other
     `unimplemented!`/`todo!`/`unreachable!` token remains in the file (the four remaining
     `unreachable` strings are the `#![allow (unreachable_code)]` lint name and prose).
### Deviations, continued (the round's thirteenth pass: the echo area)

104. **C's `if (*p1++ == ' ' && *p1++ == '[')` keeps both post-increments.**
     `message_log_check_duplicate` (C 12553) advances `p1` whether or not the comparison that
     reads it succeeds, while `&&` still short-circuits the second test; the port writes each
     test as a statement block that returns the comparison and then steps the pointer, so the
     pointer ends where C leaves it.
105. **C 12906's `char name[sizeof name_fmt + INT_STRLEN_BOUND (int)]` is 27 bytes.**  Measured
     with clang on the frozen target (the same probe shape as deviation 93): `sizeof name_fmt`
     = 16, `INT_STRLEN_BOUND (int)` = 11.  The `INT_STRLEN_BOUND` token itself cannot appear in
     the port, which is the `--check` row item 30 asks for.
106. **`vmessage`'s `doprnt` and the two `err*` streams are use-site declarations.**  C reaches
     `doprnt` through lisp.h. 4715 and `errputc`/`errwrite` through `sysstdio.h`; the port's
     mirror chain has neither in this unit's scope (`include/sysstdio.rs` mirrors the header,
     but C 477's `#include "sysstdio.h"` maps to `lib/include/stdio.rs`), so all three are
     declared here with C's prototypes.
107. **`frame_bits!` gained the `glyphs_initialized_p` arm** (frame.h 331, bit 0 of `bits1`).
     The mirror carries the shift constant but no reader for this member, and C reads it in
     three places in this round (`message3_frame_nolog`, `message_with_string`, `vmessage`) —
     the same gap family as the `ITERATOR_AT_END_OF_LINE_P` item.  The round's first attempt
     *redefined* the whole macro; the build caught the duplicate (the later definition shadows
     the earlier one), and the arm now lives in the existing definition.
108. **`INTERACTIVE` is a macro here** (`include/commands.rs` 22-23), so C's `else if
     (INTERACTIVE)` is `else if INTERACTIVE! ()`; `FRAME_TERMINAL` is likewise the macro
     `FRAME_TERMINAL!`.
109. **`unwind_with_echo_area_buffer` and `display_echo_area_1` are `unsafe extern "C" fn`**:
     C's are `static`, but their addresses go to `record_unwind_protect` and to
     `with_echo_area_buffer` respectively (deviation 18's rule, the
     `next_element_from_*`/`handle_display_prop` shape).
110. **`display_echo_area`'s `bool i` indexes `echo_area_buffer[]`**; C leans on `bool` being
     0/1 ("if oddly typed", C 13204-13206).  The port writes `echo_area_buffer[i as usize]`
     and `with_echo_area_buffer (…, if display_last_displayed_message_p { 1 } else { 0 }, …)`
     for C's implicit `bool`-to-`int` argument.
111. **`FRAME_MESSAGE_BUF_SIZE` returns `c_int` in the mirror and the array length is
     `isize`**, so the `ptrdiff_t maxsize` and the `AUTO_STRING_WITH_LEN` length both carry
     C's conversions (`as ptrdiff_t`, `as isize`); `w->desired_matrix` is `*mut c_void` in the
     mirror, so `clear_glyph_matrix` takes it `as *mut glyph_matrix`.

### Deviations, continued (the round's fourteenth pass: the echo area and mode lines)

112. **Where this round's range really starts.**  The previous round's Progress paragraph said
     "C 13258's `resize_mini_window` is the next live definition": C 13258-13260 is that
     function's *preceding comment*, and the first definition of this round's range is
     `resize_echo_area_exactly` (C 13263); `resize_mini_window` itself is at C 13307.  The
     ledger box `resize_mini_window (line 13307)` was right, the prose was not (the
     3.153/3.156/3.157/3.169/3.183 class — the numbers must come from the tool).
113. **C's `(void *) (intptr_t) {nchars}` is `nchars as *mut c_void`.**  `truncate_echo_area`
     packs a `ptrdiff_t` into the `void *` that `with_echo_area_buffer` hands to
     `truncate_message_1`, which unpacks it with `intptr_t nchars = (intptr_t) a1;` — the port
     writes both halves as the same `as` casts (C's compound literal is a scalar cast here).
114. **`current_message`'s `msg` is zero-initialised** although C leaves it uninitialised (the
     callback stores through a pointer to it): Rust has no partially-initialised locals
     (PORTING.md 1.5, deviation 1's rule), so the port writes `Qnil ()` and notes why.
115. **`FOR_EACH_FRAME (tail, frame)` is `FOR_EACH_FRAME! (tail, frame, { … })`** — the mirror's
     macro takes C's body as a third (block) argument, so the loop's braces are the macro's.
116. **Two `bool_bf` readers this unit spells itself.**  `frame.h 336`'s `resized_p` and
     `garbaged` join `glyphs_initialized_p` in the unit's `frame_bits!` (`FRAME_GARBAGED_P`
     exists in the mirror for the read; the *writer* and `resized_p`'s whole pair do not).
     The `--check` `does not mention garbaged` row that the first attempt produced is exactly
     what the arm removes: C's `f->garbaged = false;` has to be spelled with C's member name.
117. **`INLINE void flush_frame (struct frame *f)` is defined in this unit** (frame.h
     1913-1920): `INLINE` is `static inline`, so there is no external symbol to declare
     against — each unit defines its own copy (`minibuf.rs` is the precedent).  That copy
     reaches `struct redisplay_interface`'s `flush_display`, which the unit's measured record
     did not yet name: clang puts it at **72** (sizeof 104), so the record gains the field
     between two pads and the two new `const _` assertions.
118. **`SET_MARKER_FROM_TEXT_POS` is a mirror gap** (dispextern.h 336-337, the inverse of the
     `SET_TEXT_POS_FROM_MARKER` the unit already spells): the macro is defined here under C's
     name, with C's `set_marker_both (MARKER, Qnil, CHARPOS (POS), BYTEPOS (POS))` body.
119. **The mode-line enumerators are constants.**  C 13916-13921's anonymous
     `enum { MODE_LINE_DISPLAY = 0, … } mode_line_target;` becomes four `pub const`s plus the
     `mode_line_target` static (the `--check` `missing constant MODE_LINE_*` rows disappear).
120. **`BUF_MODIFF`/`BUF_SAVE_MODIFF`/`BVAR!`/`FRAME_OUTPUT_DATA` are macros here** (the
     underscore-suffixed field name for `BVAR!`, and `FRAME_OUTPUT_DATA!` whose frozen
     definition is the NULL fallback, so its use needs an explicit `as *mut c_void` for
     `.is_null ()`); `garbage_collection_inhibited` is an `intptr_t`, so C's `!` is `== 0`.

### Deviations, continued (the round's fifteenth pass: titles, menus and tab bars)

121. **`restore_selected_window`'s `Lisp_Object frame UNINIT` is `Qnil ()`.**  The frozen
     config's `UNINIT` is `= {0,}`; Rust has no partially-initialised locals (PORTING.md 1.5,
     deviation 1's rule).  This is the round's one `macro-body-notes.tsv` request (item 36).
122. **`FRAME_SELECTED_WINDOW` is used as an *l-value* in two of this round's functions** (C
     14529, 14572) while the mirror carries only the read-only accessor of that name: the port
     defines a same-named `macro_rules!` for the store, the `FRAME_WINDOW_CHANGE` precedent a
     few lines above it in this file (macros and functions live in different namespaces, so
     both spellings coexist).  That keeps C's own token in the body instead of a
     `macro-body-notes.tsv` row.
123. **`display_tab_bar`'s `#if` chain is almost entirely dead.**  C 14701-14735 tries
     `HAVE_NTGUI`, `USE_X_TOOLKIT || USE_GTK`, `HAVE_NS` and `HAVE_X_WINDOWS` before the plain
     TTY `else`; on this target only that trailing body is live, so the port transcribes it
     (with the dead chain recorded at the site).  The same applies to
     `gui_consider_frame_title` (C 14124-14227, entirely inside `HAVE_WINDOW_SYSTEM`, with its
     `record_unwind_protect (unwind_format_mode_line, …)` call) and to the three
     `update_tool_bar`/`gui_consider_frame_title` arms inside `prepare_menu_bars`, and to
     `#ifdef HAVE_EXT_MENU_BAR`'s two arms in `update_menu_bar` (whose live `#else` tests
     `FRAME_MENU_BAR_LINES` and stores `w->update_mode_line`).
124. **`FOR_EACH_FRAME!`'s body cannot `continue`.**  The mirror's macro appends
     `$list_var = XCDR ($list_var);` after the body, so a Rust `continue` would skip it and
     spin: the two C `continue`s in `prepare_menu_bars` are `tail = XCDR (tail); continue;`.
125. **`rf = root_frame (sf)` inside an argument** (C 14335/14385) cannot be an assignment in
     Rust, so the port uses the statement-block form `FRAME_MENU_BAR_LINES ({ rf = root_frame
     (sf); rf })`; `strlen` returns `usize` (cast to `ptrdiff_t`), `display_string`'s
     `field_width`/`multibyte` take `SCHARS`/`STRING_MULTIBYTE` with C's conversions, and
     `w->enabled_p`-style row flags go through the mirror's `glyph_row_bits!`.
126. **`buffer_text_bits!` gained the `text.redisplay` read arm** (the write arm was already
     there from the eleventh round; C's `buffer->text->redisplay` is read in
     `needs_no_redisplay` and `prepare_menu_bars`).

### Deviations, continued (the round's sixteenth pass: the tab-bar hit test and hscroll)

127. **`tty_handle_tab_bar_click` cannot be defined in this unit.**  `include/termchar.rs`
     already declares it (termchar.h 238-239), and Rust rejects an `extern "C"` declaration
     plus a definition of the same name in one scope (E0428) — the same family as the ledger's
     integrator item 6 (`set_buffer_internal_1`), so the unit leaves C 15615-15654 untranslated
     and reports it (item 38).  `--check` keeps listing it as a missing function.
128. **`TYPE_RANGED_FIXNUMP` is a function here, not C's type-parameterised macro**: C's
     `TYPE_RANGED_FIXNUMP (int, Vhscroll_step)` is
     `TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, Vhscroll_step, INT_MAX as EMACS_INT)` (the
     `ccl.rs`/`frame.rs` spelling).
129. **C 16630's `double hscroll_step_rel` is `f64`**, so the token `double` does not appear in
     the port — this round's one `macro-body-notes.tsv` request (item 40).  Its uses carry
     C's conversions: `XFLOAT_DATA` yields a double, and the two `wanted_x` computations that
     mix it with `int` columns write the casts out.
130. **`w->hscroll`/`min_hscroll` are `ptrdiff_t`** in the window mirror, so `hscroll` stays a
     `ptrdiff_t` through the clamp (`max (0, …) / FRAME_COLUMN_WIDTH`), while
     `cursor.x`/`text_area_width`/`h_margin` are `int` — the comparisons are written with C's
     own promotions.
131. **`include/dispextern.rs`'s new `pub struct it` is behind `#[cfg (feature =
     "dispextern_face_records")]`, and `xtask build-tu` passes no `--cfg` at all** (its rustc
     invocation has only `--edition/--crate-type/--emit/--crate-name` and three `-C` flags), so
     the mirror's record is invisible to every unit build; this unit's own measured copy is
     still the definition that compiles (verified by a clean `build-tu`).  The record does
     carry `len: c_int` at 2232 as item 31 required.  Item 39 reports the gate.
132. **`MATRIX_BOTTOM_TEXT_ROW`/`MATRIX_ROW` take the matrix pointer**, so the frame/window
     matrices (which the mirrors type as `*mut c_void`) are cast at the call:
     `MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix, …)`; the `rows` pointer is
     `(*((*w).desired_matrix as *mut glyph_matrix)).rows` (the `as`-before-`.` precedence the
     previous round also hit).

### Deviations, continued (the round's seventeenth pass: the redisplay family)

133. **The four overlay-arrow `for` loops carry `continue`, and C's increment clause must still
     run.**  `for (vlist = Voverlay_arrow_variable_list; CONSP (vlist); vlist = XCDR (vlist))`
     with `continue` in the body becomes a `while` whose increment sits at the end of the body
     and a `'body:` labelled block whose `break 'body` is C's `continue` (PORTING.md 2.2; the
     `ccl.rs`/`bytecode.rs` `'tail_step` shape, and the integrity of every path is why the
     increment may not be skipped).  `--check` therefore reports `does not mention continue` for
     the four functions — the tsv rows are items 41-44.
134. **`redisplay` is C's non-`static` `void redisplay (void)`**, so the port is
     `#[unsafe(no_mangle)] pub unsafe extern "C" fn redisplay ()` with the one call C has.
135. **`if (val = Fget (var, Qoverlay_arrow_string), STRINGP (val))`** is the port's
     `if { val = Fget (var, Qoverlay_arrow_string ()); STRINGP (val) }` (the C 8879 shape): the
     comma expression's assignment stays inside the condition.
136. **`Voverlay_arrow_variable_list` / `Voverlay_arrow_string`** are `globals::variables`'
     `pub static mut` Lisp_Objects and are read unqualified, exactly as the unit already reads
     `Vhscroll_step`.
137. **`XMARKER (val)->buffer` is the mirror's `*mut c_void`**, so the identity test is
     `(*XMARKER (val)).buffer as *mut buffer == current_buffer` (the buffer.rs 806 spelling), and
     the same cast is what `bset_redisplay` receives.
138. **The packed bits this round reaches.**  `b->clip_changed` is `buffer_bits! (clip_changed,
     b)` (the unit's own macro, 2432); `w->window_end_valid` is `window_bits! (w,
     window_end_valid)`; `w->current_matrix->{buffer,zv,begv}` are
     `(*((*w).current_matrix as *mut glyph_matrix)).…` with the `buffer` field cast back to
     `*mut buffer` (the row/frame casts of the previous round).
139. **`check_point_in_composition`'s `start`, `end` and `prop` are filled in through `&raw mut`
     by `find_composition`.**  Rust has no partial initialisation, so the port zero-initialises
     them (PORTING.md 1.5, deviation 1's rule) with the same comment C 8849 carries.
140. **`STOP_POLLING` / `RESUME_POLLING` assign a local of their caller**, so they are
     `macro_rules!`s that take that local as `$polling_stopped_here:ident` (the coding.rs
     `ONE_MORE_BYTE` shape) rather than file-scope `fn`s.  C's `do { … } while (false)` has no
     Rust counterpart at all, so the `false` token cannot reappear — row 45.
141. **`BEG` is an enum constant in this Emacs** (`buffer.h`'s `enum { BEG = 1, BEG_BYTE = 1 }`),
     so C's `BEG + BEG_UNCHANGED == start` is `BEG as ptrdiff_t + BEG_UNCHANGED! () == start`
     (the `BEG as ptrdiff_t` spelling the unit already uses).
142. **C 17233-17240's `#ifdef HAVE_WINDOW_SYSTEM` arm inside `overlay_arrow_at_row` is dead**
     (P-08): the `Fget (var, Qoverlay_arrow_bitmap)` test, the `lookup_fringe_bitmap` call and
     the `fringe_bitmap != 0` early return are recorded in the file's own comment and here, not
     transcribed.  The live `FRAME_WINDOW_P (it->f)` (a function in the mirror, `frame.rs` 282)
     and `WINDOW_LEFT_FRINGE_WIDTH! ((*it).w)` (the unit's macro) remain, and both are false on
     a TTY, so the arm is unreachable at run time as well.

### Deviations, continued (the round's eighteenth pass: `redisplay_internal`)

143. **The five labels of `redisplay_internal` are Rust control flow, and each keeps C's name
     where it can carry one.**
     * `retry:` is the enclosing `'retry: loop`; C's six backward jumps (`goto retry`, C 17787,
       17995, 18088, 18108, 18140, 18199) are `continue 'retry`.
     * `cancel:` is a labelled block `'cancel` inside the optimization `if`, and the six
       `goto cancel` (C 17658, 17685, 17692, 17770, 17849, 17852) are `break 'cancel`; the
       label's own statement (`SET_MATRIX_ROW_ENABLED_P`) follows the block, exactly where C
       falls into it.
     * `update:` is the tail of the `'update` block, so that both C's `goto update` (C 17767,
       17794, 17846 → `break 'update`) and C's fall-through out of the `else if
       (frame_redisplay_p (sf))` prelude reach the same code; the `consider_all_windows_p` arm
       keeps its own `break 'frame_pass` because C falls out of the whole chain there and never
       runs `update:`.
     * `retry_frame:` (C 17898, the only backward label inside the frame walk) is
       `'retry_frame: loop`; C 17956/17967/17983/18021 are `continue 'retry_frame` and the
       body's end is `break 'retry_frame`.
     * `end_of_redisplay:`'s code is after the `'retry` loop: C's `goto end_of_redisplay`
       (C 17792) and the loop's own fall-through are both its `break 'retry`.
     * C's `continue` inside a `FOR_EACH_FRAME` body (C 17890, 17912, 17938) cannot be a Rust
       `continue` (deviation 124's rule), so the body is a `'next_frame:` labelled block and
       they are `break 'next_frame`.
144. **`AINC` (C 17604-17609) is a `macro_rules!`** — it declares a local `entry`, so it takes
     its two arguments (the coding.rs `ONE_MORE_BYTE` shape); `windows_or_buffers_changed` and
     `update_mode_lines` enter it as `as EMACS_INT` because `make_fixnum` does.
145. **A post-increment inside a condition.**  C 17994's `garbaged_frame_retries++ <
     MAX_GARBAGED_FRAME_RETRIES` is `{ let old: c_int = garbaged_frame_retries;
     garbaged_frame_retries += 1; old } < MAX_GARBAGED_FRAME_RETRIES`, so the comparison sees
     the old value and the increment still happens.
146. **The packed members this function reaches.**  Five new `frame_bits!` arms
     (`already_hscrolled_p` 3, `updated_p` 4, `fonts_changed` 5, `cursor_type_changed` 6,
     `after_make_frame` 29 — all in `bits1`) and one new `window_bits_shift!` arm
     (`last_cursor_off_p`, shift 9); `buffer_bits!` gains the *reader* for
     `prevent_redisplay_optimizations_p` (only its writer existed, C 17624 needs the read).
147. **`f->root_window` is written the way C writes it** — `(*f).root_window` (C 17964, 18045)
     rather than through `FRAME_ROOT_WINDOW`, which is also what keeps the C body's token
     spelled.  `SAVE_MODIFF`/`MODIFF` (C 17530) keep the mirror's `BUF_SAVE_MODIFF!`/
     `BUF_MODIFF!` instead, which is why they are in this round's tsv row (item 46).
148. **`redisplay_trace ("redisplay_internal %d\n", redisplaying_p)`** passes
     `redisplaying_p as c_int`: the mirror types it `bool`, and Rust refuses to hand a `bool`
     to a variadic C function (the same reason the unit's own `redisplay_trace` takes its
     arguments through `VaList`).
149. **The dead arms inside `redisplay_internal`** (all recorded, not transcribed): C
     17411-17419 (the toolkit and Haiku `popup_activated` tests), C 17760-17763 (GLYPH_DEBUG's
     `*w->desired_matrix->method = 0` / `debug_method_add`), C 17764-17766
     (`update_window_fringes`), C 17842-17845 (GLYPH_DEBUG's "optimization 3"), C 17862-17864
     (`++clear_image_cache_count`), C 17997-18001 (`FRAME_RIF (f)->clear_under_internal_border`),
     C 18213-18219 (`clear_image_caches`), C 18222-18224 (`ns_set_doc_edited`).
150. **`mark_window_display_accurate` is now declared in the unit's scaffold** (its definition
     lands at C 18407, past this round).  The checker then adds a *fourth*
     "`is not static` … `no #[unsafe(no_mangle)]`" row for it — the class the scaffold's other
     three already carry, not a new defect (the row disappears when C 18407 lands).

### Deviations, continued (the round's nineteenth pass: the redisplay tail)

151. **Fifteen definitions land between C 18238 and C 18618** — the four unwind-protect
     callbacks, the two `mark_window_display_accurate*`, `disp_char_vector`, the four
     buffer-flip functions and their file-scope counter, `redisplay_windows`, the three
     `redisplay_window_*` condition-case functions and `update_redisplay_ticks`.  Nine of them
     were the previous round's scaffold entries; the entries are deleted in the same round the
     definitions arrive.
152. **Five of them are C statics whose *address* the port hands out** —
     `unwind_redisplay_preserve_echo_area` and `unwind_redisplay` (to
     `record_unwind_protect_void`), `redisplay_window_error`, `redisplay_window_0` and
     `redisplay_window_1` (to `internal_condition_case_1`) — so they are
     `unsafe extern "C" fn`, not plain `unsafe fn` (the unit's standing rule).
153. **`disp_char_vector`'s definition lives in `mod disp_char_vector_decl` and is
     glob-imported.**  `include/disptab.rs:67` declares the name at its mirror's top level, so
     a plain definition beside it is E0428 (the `tty_handle_tab_bar_click` family); the
     definition is unchanged and the symbol it emits is the same.  Item 49 asks the mirror to
     move its declaration into a module, the shape `include/window.rs` already uses for the
     names a unit defines (`window_redisplay_decls`).
154. **`update_redisplay_ticks`'s function-local statics** (`cwindow`, `window_ticks`) are an
     inline `mod update_redisplay_ticks_caches` **inside the function body**, exactly where C
     declares them, glob-imported one line later (the coding.rs `get_char_bytes_caches`
     shape).  Keeping the module inside the body also keeps the C body's own `struct window *`
     token spelled, which `--check` requires.
155. **Two parameters are `mut`** because C's `for` steps reassign them:
     `mark_window_display_accurate`'s `window` and `redisplay_windows`'s `window`.
156. **`ASCII_CHAR_P` takes `isize`** in the mirror, so `disp_char_vector` passes `c as isize`.
157. **The dead arms of this round** (recorded, not transcribed): C 18307-18309 (the
     `HAVE_TEXT_CONVERSION` `prev_point`/`prev_mark` declarations) and C 18339-18383 (that
     arm's `ephemeral_last_point` copy and `report_point_change` call, comment included).
158. **`buffer_flip_blocked_depth` is the unit's own file-scope object** (C 18458 declares it
     `static int`), so the port keeps it a plain `static mut` with no `#[unsafe(no_mangle)]`;
     that retires the ledger's last-but-two file-scope-object box and one `--check` row
     (`missing file-scope object` 3 -> 2).

### Deviations, continued (the round's twentieth pass: `set_cursor_from_row`)

159. **`compute_x:` is a labelled block.**  C's `goto compute_x` (C 19077) is the port's
     `break 'compute_x`, and the labelled block that carries the name opens at the
     `if (!((…)) && !(…))` whose body holds the jump; the block's end is C's `compute_x:` site,
     so the fall-through reaches the same code.  C's `return false` statements inside stay
     `return false` (they leave the function, not the block).
160. **`max`/`min` over glyph pointers are `std::cmp::max`/`min`** (C 18994/18995/18999/19000) —
     the mirror's `max`/`min` take `isize`, and the dispnew.rs precedent writes the pointer
     folds this way; it is also what keeps the two C tokens spelled (deviation 161 is only
     `goto`).
161. **The five `if`/`while` bodies that C leaves brace-less** are braced in the port
     (C 18692/18708/18718's single-statement `while` bodies, and the two `if`s at C 18743 and
     C 18835 whose body is a `while`), because Rust requires a block.  Two of them (C 18698,
     C 19002) are `for` loops whose increment moves into the body's end — the standing shape.
162. **Type spellings this round**: `MATRIX_ROW_USED!` yields `c_short`, so the `hpos`
     comparison casts it to `c_int`; `MATRIX_ROW_VPOS!` yields `isize` (it is a pointer
     difference), so `w->cursor.vpos` casts it; `eabs` is the mirror's macro, so both uses are
     `eabs! (…)`; and the C ternary at C 19128-19130 (which glyph's `object` to test) is a
     dereferencing `if`-expression: `STRINGP ((*(if … { A } else { B })).object)`.
163. **`cursor_row_p` is a new scaffold entry** (C 1130 declares it, C 25062 defines it) and
     `set_cursor_from_row`'s own forward declaration retires in the same round its definition
     lands — measured: the block stays at fifteen entries.

### Deviations, continued (the round's twenty-first pass: `try_scrolling` and the continuation start)

164. **`too_near_end:` is a labelled `loop`.**  C's `goto too_near_end` (C 19725) is the port's
     `continue 'too_near_end`; the label opens on C 19452's line as `'too_near_end: loop` and its
     body spans the rest of the function, so C's fall-through past its own label is the loop's
     fall-through.  The `return`s inside stay `return` (they leave the function, not the loop).
165. **The two `do … while` loops** (C 19509-19517 and C 19590-19596) are
     `loop { … if !(cond) { break; } }`, the standing shape; C's `break` (C 19514) and the
     test-at-the-bottom order of the second loop are preserved.
166. **`while ((distance = eabs (…)), (distance < min_distance))`** (C 19796-19797) is the
     assignment hoisted before a plain `while distance < min_distance`, the comma-expression
     shape earlier rounds used.
167. **The `if`/`else if`/`else` bodies C leaves brace-less are braced** (C 19432, 19440-19443,
     19448, 19450, 19540-19543, 19563-19564, 19571-19572, 19599-19600, 19657-19660, 19671-19672,
     19677-19678, 19683-19684, 19696-19697, 19800-19801, 19820-19821, 19828-19829),
     because Rust requires a block.
168. **Type spellings this round**: the mirror's `max`/`min` take `isize`, so the `intmax_t`
     folds (C 19440, 19476, 19641-19642) and the `c_int` folds (C 19542-19543, 19659-19660) cast
     their operands `as isize` and the result back; the `text_pos` locals C leaves uninitialised
     (`try_scrolling`'s `pos`/`startp`, `compute_window_start_on_continuation_line`'s
     `pos`/`start_pos`/`pos_before_pt`) are `core::mem::zeroed ()` (the `start_display` shape —
     a plain `let mut p: text_pos;` cannot be initialised through the `SET_TEXT_POS_*` macros,
     measured); `arg_scroll_conservatively` is `mut` because C 19432 assigns the parameter; and
     `WINDOW_TOTAL_LINES (w) * WINDOW_TOTAL_COLS (w)` (C 19784) casts the product to `ptrdiff_t`
     before the compare with C's pointer difference.
169. **`window_scroll_margin` is a use-site declaration.**  `window.h 1226` declares it and
     `window.c` defines it, but the `include/window.rs` mirror chain does not carry it, so the
     unit declares it with C's prototype in its `unsafe extern "C"` block (PORTING.md P-72 rule 3;
     item 55).
170. **The dead `#ifdef GLYPH_DEBUG` arm** C 19418-19420 (`debug_method_add (w, "try_scrolling")`)
     is recorded, not transcribed (the dead-arm list above).
171. **Two C doc comments the range boundaries had left out are restored.**  Round 22's range
     started at C 18630, so `set_cursor_from_row`'s own comment (C 18623-18628, "Set cursor
     position of W. …") never reached the port; this round's range therefore starts at C 19256
     so that `run_window_scroll_functions`' comment (C 19258-19261) does, and the earlier one is
     put back at its site in the same step.  **Measured**: both texts are in `src/xdisp.rs`
     (lines 17848 and 18494) and neither is a paraphrase.

### Deviations, continued (the round's twenty-second pass: cursor movement and the scroll bars)

172. **The two `do … while` loops** (C 20176-20233's bidi candidate loop and C 20247-20258's plain
     loop) are `loop { … if !(cond) { break; } }`, the standing shape; the `break`s inside them
     (C 20222, 20226, 20252) leave the loop exactly as C's do.  C's `do`/`}` pair becomes the
     Rust `loop`/`}` pair, and the trailing `while (…)` becomes the tail `if !(…) { break; }`.
173. **`if (must_scroll) ;`** (C 20123-20124) is the port's `if must_scroll { }` — C's empty
     statement; the `else if` chain that follows is unchanged.
174. **The two comma/walrus conditions**: C 20401-20402's
     `if ((invprop = Fget_char_property (…)), TEXT_PROP_MEANS_INVISIBLE (invprop) != 0)` hoists
     the assignment to its own statement, and C 20406's `if (!NILP (disp_spec = …) && …)` uses
     the unit's `{ X = …; X }` block (the `handle_single_display_spec` shape).  Both keep the
     exact evaluation order.
175. **No `goto` and no new label this round**, so `--check` gains no `does not mention` line.
176. **Type spellings this round**: `!update_mode_lines`/`!windows_or_buffers_changed` (C 19904/
     19905) are `== 0` (the two are `c_int`, not `bool`); `w->current_matrix` is the mirror's
     `*mut c_void`, so its fields are read as `(*((*w).current_matrix as *mut glyph_matrix)).nrows`
     / `.rows` (the `redisplay_window` precedent — the parens are required because Rust's `as`
     binds looser than unary `*`); the two `set_*_scroll_bar_hook`s take
     `(*mut c_void, c_int, c_int, c_int)`, so the vertical site writes
     `(w as *mut c_void, (end - start) as c_int, whole as c_int, start as c_int)`; `hscroll` is
     `ptrdiff_t`, so `w->hscroll * FRAME_COLUMN_WIDTH (…)` is `((…) as isize) as c_int` and
     `w->hscroll_whole = whole` casts to `ptrdiff_t`; and `scroll_step` is passed as `*mut bool`,
     which is what `*scroll_step = true` (C 20143) writes through.
177. **`BUF_BEGV` and `BUF_ZV` are the mirror's functions, not macros.**  This round first
     wrote them as unit-local `macro_rules!`; the integrator's 8958c83 removed them, and the
     finding is worth keeping: C's `BUF_BEGV`/`BUF_ZV` are `INLINE` **functions** (buffer.h
     917/951, with the `current_buffer` fast path and the `begv_marker`/`zv_marker` branches),
     not `#define`s, and `include/buffer.rs:923/957` already mirrors them as `pub fn`.  The
     macros were therefore not a mirror gap but an invented construct — and because Rust keeps
     macros and functions in separate namespaces, the shadowing was silent; the four call sites
     (C 20285-20287 and C 20305) had been reading the bare fields instead of calling the mirror.
     Item 56 is closed as "the unit had one copy too many", not "the mirror is short".
178. **The two dead `#ifdef GLYPH_DEBUG` arms** (C 19879-19882, C 19934-19936) are recorded, not
     transcribed (the dead-arm list above).

### Deviations, continued (the round's twenty-third pass: the mirror-dupes sweep)

179. **Forty-four unit-local macros deleted as duplicates of the mirror's.**  The
     `window.h`/`dispextern.h` families this unit had declared for itself (the `WINDOW_*`
     accessors, the divider widths and frame edges, the scroll-bar geometry, the three
     `*_LINE_HEIGHT` triples, and the `CURRENT_*_LINE_HEIGHT`/`MATRIX_*_LINE_HEIGHT` family) are
     carried by `include/window.rs` and `include/dispextern.rs` now; `macro_rules!` textual
     shadowing meant the unit's copies won at every call site.  Both sides were compared
     against C before deleting: 37 bodies are token-identical, and the remaining 7 differ only
     in ways that leave the C reading intact — three `MATRIX_*_LINE_HEIGHT` bodies differ by a
     trailing `;`, the three `CURRENT_*_LINE_HEIGHT` bodies call
     `estimate_mode_line_height (XFRAME ((*($w)).frame), …)` (C's `XFRAME ((W)->frame)`) where
     the unit had routed through `WINDOW_FRAME!`, and `WINDOW_BOTTOM_DIVIDER_WIDTH` reads
     `WINDOW_PSEUDO_P ($w)` where the unit had `window_bits! ($w, pseudo_window_p)` — the same
     bit (`WINDOW_PSEUDO_WINDOW_P_SHIFT`).  The mirror's spelling therefore wins in every case;
     **no unit-side or mirror-side defect was found**, so nothing is handed back for the mirror.
     `window_bits`/`window_bits_shift` stay: the mirror carries `window_bit` (the reader) and
     the shift constants, not those two names.

10. **This round's six use-site declarations.**  `blank_row` (dispextern.h 3914, defined by
    `dispnew.c`) and five of this unit's own later definitions — `row_containing_pos`
    (dispextern.h 3563-3565, defined at C 22472), `try_window_reusing_current_matrix`
    (xdisp.c 1163, defined at C 21793), `try_window_id` (xdisp.c 1164, defined at C 22588),
    `display_mode_lines` (xdisp.c 1168, defined at C 28112) and `display_menu_bar`
    (xdisp.c 1175, defined at C 27761) — are declared in the unit's scaffold with C's
    prototype in a `/// C:` line (PORTING.md 0.1-12/P-72 rule 3: the four that C declares
    `static` cannot come from a mirror anyway; `row_containing_pos`'s dispextern.h prototype
    *could*, and item 57 asks for it).  The four `static` ones leave the ledger's
    `missing function` rows in place and the two non-`static` ones each cost an `is not
    static in C but has no #[unsafe(no_mangle)]` row (deviation 184).

### Deviations, continued (the round's twenty-fourth pass: `redisplay_window`)

180. **The eight C labels are the port's labelled blocks and loops** (PORTING.md 2.2).  The five
     forward targets — `finish_scroll_bars` (C 21613), `need_larger_matrices` (C 21611),
     `finish_menu_bars` (C 21519), `done` (C 21445) and `recenter` (C 21155) — open as labelled
     blocks at the top of the function body and *end at the C label's own site*, so each
     `break 'X` lands on the label's body and the C fall-through reaches the same statement (the
     `compute_x:` shape, deviation 159); the blocks nest in the C labels' order, which is what
     makes C's jump spans proper (a `break` needs the label to enclose it).
     * `recenter` carries **two** Rust labels because C jumps to it from both sides: the
       `'recenter` block (whose end is C 21154) for the forward jumps (C 20693, C 20980,
       C 21059) and the `'recenter_body` loop, whose first iteration is C 21155, for the
       backward ones (C 21414, C 21442 → `continue 'recenter_body`); the loop's trailing
       `break 'recenter_body` is C's fall-through into `done:`.
     * `force_start` (C 20743) is a `'force_start: loop`: C's one backward `goto force_start`
       (C 21048) is `continue 'force_start`, and the loop's trailing `break 'force_start`
       stands where C's `recenter:` label is — without it the fall-through would re-run the
       loop (the `'too_near_end`/`'retry` shape, deviations 143/164).
     * `try_to_scroll` (C 21104) and `ignore_start` (C 20945) are entered from *inside* C's
       `if (w->force_start)` block (C 20848, 20868, 20882, 20936, 20961, 21083 and C 20786), so
       their blocks open *before* that `if` — a Rust labelled block cannot open inside a block
       and close outside it — and end at C 21103 and C 20944 respectively.
     * `need_larger_matrices`'s own statement is C's empty `;` (C 21612), so the port needs no
       statement for it: the labelled block closes at C 21610 and both the `break` and C's
       fall-through landing on the empty statement continue into the `finish_scroll_bars` tail
       (the `if (must_scroll) ;` shape, deviation 173).
     * C's eight `goto`s appear as 24 `break`/`continue` statements; `--check` reports the
       `goto` token in this function's one `does not mention` line.
181. **The four loops.**  C 20556-20559's `for (y = 0, row = …; y < yb; y += row->height, ++row)`
     is a `while` with the two init clauses at their declarations and the increment at the
     body's end (the standing shape), and C 21080-21085's `for (cur = BEGV; cur < ZV; cur =
     next)` likewise with C's `break` before the increment; C 21338-21340's
     `do { move_it_by_lines (&it, 1); } while (IT_CHARPOS (it) == pos0)` is
     `loop { …; if !(cond) { break; } }`; C 21245's `while (nlines-- && …)` is
     `while ({ let old: c_int = nlines; nlines -= 1; old }) != 0 && …` (the post-decrement shape
     of deviation 145, parenthesised so the `{` cannot be read as the loop's own body).
182. **C 21266's chained assignment is three statements.**  `it.current_x = it.wrap_prefix_width =
     it.hpos = 0;` cannot be one Rust expression (an assignment's value is `()`), so the port
     writes the three in C's right-to-left evaluation order (`it.hpos = 0;
     it.wrap_prefix_width = 0; it.current_x = 0;`), which the unit already does for the same
     C idiom at C 12194.
183. **Type spellings this round.**  `scroll_conservatively` is `EMACS_INT` (`isize`) and
     `SCROLL_LIMIT` is `c_int`, so the three `scroll_conservatively > SCROLL_LIMIT` tests read
     `> SCROLL_LIMIT as ptrdiff_t`; `emacs_scroll_step` is `EMACS_INT` where `try_scrolling`
     takes `intmax_t`, so the call casts it; `scroll_minibuffer_conservatively` is a `bool` in
     `globals.rs` (C's `int`), so C's `!= 0` tests are the bare name; `float_amount` is
     `c_double` and `pt_offset = float_amount * WINDOW_BOX_TEXT_HEIGHT (w)` casts both ways;
     `BEG + beg_unchanged` is `BEG as ptrdiff_t + beg_unchanged` (deviation 141);
     `move_it_by_lines`'s argument is `if move_down { margin + 1 } else { -(margin + 1) }` cast
     to `ptrdiff_t`; `XFIXNAT`/`XFIXNUM` results are `as ptrdiff_t`; `centering_position`,
     `new_vpos`, `pixel_margin`, `window_height`, `window_total_lines`, `pt_offset`, `nlines`,
     `ss`, `rc`, `tem` and the `pos_visible_p` out-parameters are `c_int`; the packed members
     are `window_bits!` (`force_start`, `optional_new_start`, `preserve_vscroll_p`,
     `start_at_line_beg`, `window_end_valid`, `update_mode_line`, `redisplay`,
     `must_be_updated_p`), `buffer_bits!` (`clip_changed`,
     `prevent_redisplay_optimizations_p`, and — **new this round** — the
     `long_line_optimizations_p` *writer*, C 20688 being its first write in the unit),
     `buffer_text_bits! (text.redisplay, …)` (both the clear at C 20915 and the read at
     C 20918) and `frame_bits!` (`fonts_changed`, `cursor_type_changed`); `(*w).cursor.y`/
     `vpos`, `window_end_pos`, `base_line_pos`, `column_number_displayed`,
     `mode_line_height`, `tab_line_height`, `header_line_height` and `vscroll` are plain
     fields.
184. **`row_containing_pos` is a use-site declaration, and it is this round's one new `--check`
     row.**  The mirror chain does not carry dispextern.h 3563-3565, so the unit declares it in
     its scaffold (item 10); because C defines it **in this file** (C 22472) and not `static`,
     that declaration trips `--check`'s "`is not static` … `no #[unsafe(no_mangle)]`" rule — the
     class `try_window` and `produce_stretch_glyph` already carry (two of the previous round's
     107).  **Measured**: 107 -> 108, and this is the round's only growth; the suggested
     C 20424-20693 boundary measures 109 (see the Progress paragraph).  The row retires when
     C 22472 lands, or sooner if the mirror takes the prototype (item 57).
185. **C's `DESIRED_MODE_LINE_HEIGHT (w)` is spelled as its own expansion.**  The mirror's three
     `DESIRED_*_LINE_HEIGHT` macros call their inner `MATRIX_*_LINE_HEIGHT` names without the
     `!` a Rust macro call needs, so `DESIRED_MODE_LINE_HEIGHT! (w)` is `E0423` (measured);
     the six sites (C 21479, 21484, 21490, 21495, 21501, 21506) therefore spell
     dispextern.h 1661-1662's own expansion, `MATRIX_MODE_LINE_HEIGHT! ((*w).desired_matrix)`
     and its two siblings, with a note at the site.  The mirror defect is item 58; `--check`
     then names the three `desired_*_line_height` tokens in this function's one
     `does not mention` line.
186. **Three mirror macros that C calls as functions** are called with the `!` they need:
     `TEXT_PROP_MEANS_INVISIBLE!` (C 20813, C 21378), and `BUF_Z!` (C 20545 — `BUF_BEG` next to
     it is a mirror *function*, so only the one name carries the `!`).  `if (!try_window (…))`
     (C 20794, C 20928) is `if try_window (…) == 0`, the mirror's `try_window` returning `c_int`.
187. **C's two `switch`es are `match`es** (the standing shape): `switch (rc)` over
     `CURSOR_MOVEMENT_SUCCESS`/`_MUST_SCROLL` with C's `default: emacs_abort ()` as `_` (C
     20954-20965) and `switch (ss)` over `SCROLLING_SUCCESS`/
     `SCROLLING_NEED_LARGER_MATRICES`/`SCROLLING_FAILED` (C 21136-21149), where
     `SCROLLING_FAILED`'s `break` leaves the switch and the port's arm is `{ }`.
188. **The `{ X = …; X }` block** carries C's assignment-in-expression sites: C 20951's
     `(rc = try_cursor_movement (…), rc != …)`, C 20986's `(tem = try_window_id (w)) != 0`,
     C 21074's and C 21291's `!(used_current_matrix_p = try_window_reusing_current_matrix (w))`
     and C 21541's `FRAME_MENU_BAR_LINES (rf = root_frame (f))` (deviations 135/174's shape).
189. **`pos_visible_p`'s six out-parameters are zero-initialised** (`d1`/`d2`/`d5`/`d6`/`rtop`/
     `rbot`, C 21008-21009) and so is `found` in the long-line scan (C 20678): `&raw mut` needs a
     value and Rust has no partial initialisation (deviation 139's rule, PORTING.md 1.5).
190. **The three `text_pos` locals are `core::mem::zeroed ()`** (C 20481).  `SET_TEXT_POS`
     assigns the two fields one at a time, and Rust's definite-initialisation analysis rejects
     that for a `let` without a value (`E0381`, measured), so `lpoint`/`opoint`/`startp` start
     zeroed (the `try_scrolling` shape, deviation 168); the `struct it` local is zeroed the
     same way (C 20484), and C's other uninitialised locals (`tem`, `rc`, `beg_unchanged`,
     `end_unchanged`, `frame_line_height`, `update_mode_line`, `use_desired_matrix`,
     `it_charpos`, `new_pt`, `alt_pt`, `aggressive`, `scrolling_up`, `margin_pos`) are declared
     without a value where every path assigns before the read (the `redisplay_internal` shape).
191. **Pointer and matrix spellings.**  `w->desired_matrix`/`w->current_matrix` are the mirror's
     `*mut c_void`, so the rows are `(*((*w).desired_matrix as *mut glyph_matrix)).rows` (the
     paren rule of deviation 176) and the two row walks advance with `row.offset (1)`;
     `MATRIX_FIRST_TEXT_ROW!`/`MATRIX_MODE_LINE_ROW!`/`MATRIX_TAB_LINE_ROW!`/
     `MATRIX_HEADER_LINE_ROW!`/`MATRIX_ROW_BOTTOM_Y!`/`MATRIX_ROW_START_CHARPOS!`/
     `MATRIX_ROW_START_BYTEPOS!` and `glyph_row_bits! (ends_at_zv_p|tab_line_p|mode_line_p,
     row)` come from the mirrors; the `redeem_scroll_bar_hook` call passes `w as *mut c_void`
     (the hook is typed `*mut c_void`, C 21628); `FRAME_SELECTED_WINDOW!` is the unit's own
     macro (frame.h 1387's l-value form, deviation 146's sibling) and `FRAME_WINDOW_P`/`WINDOWP`
     are the mirrors'; `specpdl_ref count1` keeps C's name for the mode-line block's
     `SPECPDL_INDEX`.

### Deviations, continued (the round's twenty-fifth pass: `try_window`)

192. **The scaffold loses `try_window`'s declaration.**  The standing rule (deviation 150's
     class) is that a use-site declaration is deleted in the same round its definition lands;
     `try_window` (C dispextern.h 3573) is this round's only such entry, so the block goes
     17 -> 16.  There is no `row_containing_pos` sibling to delete: the round-25 commit
     (d90609a/3.357) added the mirror's `dispextern_late_decls` module (dispextern.rs 3686-3706)
     for `row_containing_pos` and `blank_row` and glob-imports it (3706), so the unit declares
     neither (item 57, CLOSED) — and `--check`'s `is not static … no #[unsafe(no_mangle)]` row
     for `row_containing_pos` (deviation 184) retired with the unit's own copy, which is why this
     round's measured baseline is 107, not the round-25 text's 108.
193. **The three `DESIRED_*_LINE_HEIGHT` macros are C's now** (item 58, CLOSED): the mirror's
     bodies carry the `!` their inner `MATRIX_*_LINE_HEIGHT` calls need, so `redisplay_window`'s
     six sites (C 21479-21506) read `DESIRED_MODE_LINE_HEIGHT! (w)` and its two siblings again —
     deviation 185 is superseded and the three `desired_*_line_height` tokens have left
     `redisplay_window`'s `does not mention` line.  `try_window` does not call them.
194. **`CURRENT_HEADER_LINE_HEIGHT!`/`CURRENT_TAB_LINE_HEIGHT!` are the mirror's macros** (the
     `include/window.rs` 751/758 spellings the unit already used at C 18949-18951); C 21725/21727
     are their new call sites and keep the same shape.
195. **Type spellings this round.**  `flags` is `c_int` and the two `TRY_WINDOW_*` masks are
     `c_int` constants (`include/dispextern.rs` 2499/2502), so the two tests read
     `(flags & TRY_WINDOW_… ) != 0`; `cursor_vpos`, `last_row_scale`/`last_col_scale`,
     `top_scroll_margin`/`bot_scroll_margin` are `c_int` and the `IT_CHARPOS` snapshot
     `it_charpos` is `ptrdiff_t`; `last_text_row` is `*mut glyph_row` and C's `it.glyph_row - 1`
     is `.offset (-1)`; `it.glyph_row->reversed_p = false` is `glyph_row_bits! (reversed_p,
     it.glyph_row, false)` (the bitfield writer, PORTING.md 1.3); `w->update_mode_line = true`
     and `w->window_end_valid = false` are `window_bits! (w, update_mode_line, true)`/
     `window_bits! (w, window_end_valid, false)`; `f->fonts_changed` is `frame_bits!
     (fonts_changed, f)`; and `w->desired_matrix` is the mirror's `*mut c_void`, so
     `clear_glyph_matrix` and `MATRIX_ROW` take the `as *mut glyph_matrix` cast (deviation 191's
     rule).
196. **`struct it it;` is `core::mem::zeroed ()`** (C 21687), the standing shape (deviations
     139/168/190): `start_display` fills it in and Rust has no partial initialisation.  The loop's
     `while` is C's own (not `while true`), so the build's warning set is unchanged.

### Deviations, continued (the round's twenty-sixth pass: `try_window_reusing_current_matrix` and the reusing-current-matrix helpers)

197. **The tail `while (true)` is `loop`.**  `row_containing_pos`'s display loop (C 22491-22541)
     is C's `while (true)`, and its only exits are the five `return`s inside.  Rust's `while
     true` is **not** a diverging expression, so a `while true` loop that is the function's
     last statement does not type-check in a `-> *mut glyph_row` function; the port writes
     `loop`, which is the construct for exactly this shape (the `handle_stop`/`move_it_to`
     `loop` sites are the same idea for `do {} while`).  C's `true` token is therefore absent,
     which is why `--check` grows a `does not mention true` row (listed in the Progress section
     and the to-do list); the two earlier `while (true)` sites (C 10210, C 26051) are *not* the
     function's tail, so they stay `while true`.
198. **`row_containing_pos`'s own definition takes `*mut window`.**  The mirror declares the name
     at dispextern.h 3563 with the P-72-rule-3 degradation `*mut c_void` (six units include
     `dispextern.rs` without `window.rs`); this unit includes `window.rs`, so its **definition**
     spells C's `struct window *` as `*mut window`, and the definition shadows the glob-imported
     declaration (PORTING.md 0.1-10).  The three sites an earlier round wrote against the
     mirror's spelling — C 19877/20447/20472's calls — are updated in the same round to drop
     their `as *mut c_void` casts (P-87's rule that a mirror-shape change is validated against
     every includer; here the direction is the reverse).
199. **`BUF_ZV` is a mirror *function*, not a macro.**  `include/buffer.rs` 965 spells it
     `pub fn BUF_ZV (buf: *mut buffer) -> ptrdiff_t` (the `BUF_BEGV`/`BUF_ZV` family cannot be
     l-values the way `BEGV`/`Z` can, buffer.rs 926), so C 22477's `BUF_ZV (XBUFFER (w->contents))`
     is `BUF_ZV (XBUFFER ((*w).contents))` with no `!`.
200. **`BVAR (buf, bidi_display_reordering)` is `BVAR! (buf, bidi_display_reordering_)`.**  C's
     `BVAR` pastes an underscore onto the member name (`BVAR (BUF, x)` is `(BUF)->x_`), and the
     port's mirror `BVAR!` takes the **C struct member's own name**, so the C source token
     `bidi_display_reordering` cannot appear — the six earlier sites in this unit (C 4317, 9016,
     14627, 16018, 19113, 19177) spell it the same way.
201. **`rotate_matrix` is a use-site declaration.**  `dispnew.c` defines it and no mirror in this
     unit's include graph carries it (dispextern.h 3911), so it joins the unit's own
     `unsafe extern "C"` block with C's prototype in a `/// C:` line (the `clear_glyph_matrix`/
     `increment_matrix_positions` shape; §"Use-site declarations" 12).  C 21983/22167 pass
     `MATRIX_ROW_VPOS (bottom_row, w->current_matrix)`, which the mirror states as an `isize`
     (`offset_from`), so the argument is `… as c_int` (C's `int` parameter).
202. **Zero-initialised locals** (PORTING.md 1.5), this round's set: `try_window_reusing_current_matrix`'s
     `it`, `run`, `start`, `new_start` (`core::mem::zeroed ()`), `bottom_row`,
     `last_text_row`, `last_reused_text_row`, `start_row` (`core::ptr::null_mut ()`),
     `nrows_scrolled`, `i`, `start_vpos`, `min_y`, `max_y`, and the two nested blocks'
     `row`, `dy`, `pt_row`, `first_reusable_row`, `first_row_to_display`, `yb`;
     `find_last_row_displaying_text`'s `row`, `row_found`; `find_last_unchanged_at_beg_row`'s
     `row`, `row_found`, `first_changed_pos` (the `ptrdiff_t` sum) and `yb`;
     `find_first_unchanged_at_end_row`'s `row`, `row_found`, `Z_old`, `Z_BYTE_old`,
     `last_unchanged_pos`, `last_unchanged_pos_old`, `first_text_row`;
     `sync_frame_with_window_matrix_rows`'s `window_row`, `window_row_end`, `frame_row`;
     `row_containing_pos`'s `best_row`, `mindif`, `last_y`, `g`.  C declares each without an
     initializer and assigns before the first read, except the three that C's branches only
     sometimes assign (`start`/`new_start`, `last_unchanged_pos*`, `g`).
203. **C's chained assignments became consecutive statements** with the C chain in a comment
     (the `init_iterator` shape, deviation 2): `last_text_row = last_reused_text_row = NULL`
     (C 21891), `*delta = *delta_bytes = 0` (C 22358) and
     `row->enabled_p = row->mode_line_p = row->tab_line_p = false` (C 22020, three
     `glyph_row_bits!` writers).
204. **C's `for` loops became `while` loops with the increment at the body's end** — the
     `for (i = 0; i < nrows_scrolled; ++i)` (C 21989), `for (row = start_row + nrows_scrolled;
     row < bottom_row; ++row)` (C 21995), `for (++row; row < bottom_row; ++row)` (C 22019,
     written as a leading `row = row.offset (1);` plus the loop), `for (row = first_reusable_row;
     row < first_row_to_display; ++row)` (C 22153), `for (row -= nrows_scrolled; …)` (C 22173),
     `for (row = MATRIX_ROW (w->current_matrix, w->cursor.vpos); …; row++)` (C 22180),
     `for (first_row_to_display = first_reusable_row; …; ++first_row_to_display)` (C 22077),
     `for (; row > first_text_row; --row)` (C 22403) and `for (g = …; g < …; g++)` (C 22521).
     The increment sits at the end of the body so a C `break` skips it exactly as C does.
205. **Bitfield/field spellings this round**: `f->cursor_type_changed` -> `frame_bits!
     (cursor_type_changed, f)`; `f->fonts_changed` -> `frame_bits! (fonts_changed, f)`;
     `w->current_matrix->header_line_p` -> `glyph_matrix_bits! (header_line_p, …)`;
     `w->desired_matrix->no_scrolling_p` -> `glyph_matrix_bits! (no_scrolling_p, …)`;
     `w->window_end_valid` -> `window_bits! (w, window_end_valid[, false])`;
     `row->{enabled,mode_line,tab_line,continued,ends_at_zv,exact_window_width_line,fringe_bitmap_periodic,redraw_fringe_bitmaps,reversed}_p`
     -> `glyph_row_bits! (…)`; and `glyph->pixel_width` (the mirror's `c_short`) is
     `(*glyph).pixel_width as c_int` where C's `int` arithmetic needs it (the `set_cursor_from_row`
     shape).
206. **Pointer arithmetic and matrix spellings**: `w->current_matrix`/`w->desired_matrix`/
     `f->current_matrix` are the mirror's `*mut c_void`, so every `MATRIX_ROW`/
     `MATRIX_FIRST_TEXT_ROW`/`MATRIX_MODE_LINE_ROW`/`MATRIX_TAB_LINE_ROW`/`MATRIX_HEADER_LINE_ROW`/
     `MATRIX_BOTTOM_TEXT_ROW`/`MATRIX_ROW_VPOS` call and the three member reads in
     `sync_frame_with_window_matrix_rows` carry the `as *mut glyph_matrix` cast;
     `MATRIX_ROW_VPOS` is an `isize` (`offset_from`), so the two assignments through it are
     `… as c_int`/`as isize`; `start_row + i` is `.offset (i as isize)`;
     `row -= nrows_scrolled` is `.offset (-(nrows_scrolled as isize))`; `row + 1` / `row - 1`
     are `.offset (1)` / `.offset (-1)`; and `start_row - MATRIX_FIRST_TEXT_ROW (w->current_matrix)`
     is `start_row.offset_from (…) as c_int` (C 21940).
207. **`it.glyph_row - 1` is `.offset (-1)`** (C 21928/22111) and `it.glyph_row->reversed_p = false`
     is `glyph_row_bits! (reversed_p, it.glyph_row, false)` (C 21926).
208. **`eassert!` is the mirror's release arm** (an empty `#[cfg (enable_checking)]` block,
     PORTING.md 2.5), so the four assertions this round adds — `row->enabled_p` (C 22273),
     `nrows_scrolled > 0` (C 22166), `w->window_end_valid` (C 22362) and `!row_found ||
     MATRIX_ROW_DISPLAYS_TEXT_P (row_found)` (C 22415) — carry their conditions unevaluated in
     the frozen build.  **The same rule hid a real type error in an earlier round's text**: the
     `(*(*w).current_matrix).nrows` inside `try_cursor_movement`'s `eassert!` (xdisp.rs 18899,
     C 19892's `w->current_matrix->nrows`) does not compile under `--cfg enable_checking`
     (`E0609: no field nrows on type c_void`, measured; the fix is
     `(*((*w).current_matrix as *mut glyph_matrix)).nrows`).  It is reported, not touched (it
     is outside this round's C range); see the to-do list.

### Deviations, continued (the round's twenty-seventh pass: `try_window_id`)

209. **The brief's C range is a line count, not a boundary.**  `try_window_id` opens at C 22587
     (`static int`) / 22588 (its name) and closes at C 23381; the "to C 23821, 1,234 lines" in
     the brief and the previous round's Progress text is the distance from the signature to the
     *next* definition's `static struct glyph_row *` (C 23821, `get_overlay_arrow_glyph_row`),
     which includes the dead `#ifdef GLYPH_DEBUG` block C 23388-23809.  This round stops at the
     function's own closing brace (`794` lines with the doc comment from C 22545); the dead block
     is recorded in the dead-arm list so the boundary stays unambiguous.
210. **`GIVE_UP` is a function-local `macro_rules!`.**  C 22621-22629 defines it inside the
     function under `#if false` / `#else` and `#undef`s it at C 23380.  The port writes C's live
     arm as `macro_rules! GIVE_UP { ($x:expr) => { return 0; }; }` at the same point; the dead
     arm's `redisplay_trace` body is recorded, so the argument has no site (the
     `IT_OVERFLOW_NEWLINE_INTO_FRINGE` shape, P-08), and the `#undef` needs no site because a
     Rust macro's scope ends with the enclosing block.  `--check`'s `missing macro GIVE_UP` row
     retires with it.
211. **The seven live `IF_DEBUG (...)` statements are recorded, not transcribed** (the
     `redisplay_window` class): C 22957-22958 (`debug_delta`/`debug_delta_bytes`), C 23076
     (`debug_dvpos`/`debug_dy`), C 23324/23330/23367 (`debug_method_add (w, "A"/"B"/"C")`) and
     C 23372-23373 (`debug_end_pos`/`debug_end_vpos`).  The mirror's `IF_DEBUG` expands its
     argument (`let _ = $x;`), and every symbol those arguments name is a `GLYPH_DEBUG`-only
     global the frozen configuration does not define, so each site keeps a pointer comment.
     The eight tokens are the row this round owes (Progress section; to-do item 66).
212. **The fringe/line-prefix give-up keeps C's structure.**  C 22903-22920's
     `it.string_from_prefix_prop_p` read is the unit's
     `it_bits! (string_from_prefix_prop_p, &raw mut it)` (the `&raw mut` is the spelling for a
     local `it` value, the C 3099 shape), `it.w->contents` is `(*it.w).contents`, and
     `Qline_prefix`/`Qwrap_prefix` are the `globals.rs` mirrors' `Qline_prefix ()`/
     `Qwrap_prefix ()` (this is the unit's first use of either).  The four-part `||` chain is
     C's own grouping.
213. **Pointer arithmetic and matrix spellings.**  `w->current_matrix`/`w->desired_matrix` are
     the mirror's `*mut c_void`, so the port's two locals are
     `(*w).current_matrix as *mut glyph_matrix` / `(*w).desired_matrix as *mut glyph_matrix`;
     C's `w->current_matrix->rows` (the `while` bound) is
     `(*((*w).current_matrix as *mut glyph_matrix)).rows` (the C 19892 spelling);
     `MATRIX_ROW_VPOS` returns an `isize` (`offset_from`), so each of its seven assignments here
     carries `as c_int`; `first_unchanged_at_end_row += dvpos` is `.offset (dvpos as isize)`;
     `vpos + 1`'s row walk is `current_row.offset (-1)` / `desired_row.offset (-1)`; and
     C 23348-23349's `current_matrix->rows + vpos` is `(*current_matrix).rows.offset (vpos as isize)`.
214. **C's chained assignments became consecutive statements** (deviation 203's shape), each with
     the C chain in a comment: C 23072-23073's
     `delta = delta_bytes = dvpos = dy = run.current_y = run.desired_y = run.height = 0` and
     C 23295's `it.hpos = it.current_x = it.wrap_prefix_width = 0`.
215. **`max` takes `isize`.**  C 23068's `max (run.current_y, run.desired_y)` is
     `max (run.current_y as isize, run.desired_y as isize) as c_int` (the `include/lisp.rs`
     mirror's `pub fn max (a: isize, b: isize) -> isize`, the unit's standing spelling for `int`
     arguments).
216. **Type spellings this round**: `it.sp`/`it.method`/`it.image_id`/`it.vpos`/`it.hpos`/
     `it.current_x`/`it.wrap_prefix_width`/`it.continuation_lines_width`/`it.current_y`/
     `it.last_visible_y`/`it.first_vpos` are the unit's own `struct it` members (C 1389-1660);
     `it.glyph_row - 1` is `.offset (-1)`; `glyph_row->{reversed,displays_text,enabled,
     ends_in_newline_from_string}_p` are the `glyph_row_bits!` writers/readers; `window_end_valid`
     is `window_bits! (w, window_end_valid[, false])`; `desired_matrix->no_scrolling_p` is
     `glyph_matrix_bits! (no_scrolling_p, …)`; `f->cursor_type_changed`/`f->fonts_changed` are
     `frame_bits!`; `current_buffer->clip_changed`/
     `prevent_redisplay_optimizations_p` are `buffer_bits!`; the four `BVAR` reads are
     `BVAR! (XBUFFER ((*w).contents), word_wrap_)` and its three siblings; `OVERLAY_MODIFF` is
     the mirror's `OVERLAY_MODIFF! ()`; `BEG` is the `pub const BEG: c_int` (`BEG as ptrdiff_t`,
     parenthesised where the `as` met a following `<`); `window_wants_tab_line (w)`/
     `window_wants_header_line (w)` need `as c_int` inside C 23167-23170's integer sum; and
     `FRAME_SCROLL_REGION_OK`/`FRAME_LINE_INS_DEL_OK`/`WINDOW_FULL_WIDTH_P` are the mirrors'
     `macro_rules!` (the `!` form; the last two were the two `E0423`s of this round's first
     build).
217. **`eassert!`/`eassume!` are the mirror's release arms.**  This round's eleven assertions
     (C 22931, 22968, 22991, 23273, 23318, 23321, 23323, 23329, 23338, 23353 and 23366) carry
     their conditions unevaluated in the frozen build, exactly as deviation 208 records for the
     last round's four; C 23003's is inside the dead `GLYPH_DEBUG` block and stays recorded.
     Because `build-tu`'s single-unit path compiles one rustc pass, these bodies are visible to
     the `--cfg enable_checking` probe only (see the Gate evidence section, which runs it).

### Deviations, continued (the round's twenty-eighth pass: the desired-matrix-row builders)

218. **The round's C range is C 23817-25075; the gap above it is C 23385-23816.**  The previous
     round ended at `try_window_id`'s closing brace (C 23381); C 23385-23816 is the "More
     debugging support" banner and its whole `#ifdef GLYPH_DEBUG` block (already recorded as
     C 23388-23809), so this round's first transcribed line is C 23817's doc comment.  The next
     live definition is C 25077's `push_prefix_prop`.
219. **`struct face` names four more members** (the unit's own measured copy, deviation 26's
     record): `background` (offset 176, carved out of the copy's old `_pad0`) and the
     `face_bits!` read arms `underline` (dispextern.h 1893-1935's `ENUM_BF (face_underline_type)
     underline : 3` at bit 2) and the `overline_p`/`strike_through_p` `bool_bf` bits (bits 6/7),
     measured in `docs/baseline/face-layout.tsv` and spelled identically to `xfaces.rs`'s copy of
     the record.  `extend_face_to_end_of_line`'s C 24442-24458 skip test and its two margin fills
     (C 24753, C 24830) are the readers.
220. **`glyph_bits!`/`glyph_row_bits!`/`it_bits!` stand in for C's bitfields** (PORTING.md 1.3):
     `row->glyphs[area][k].type` is `glyph_bits! (type, *ptr)` and `padding_p` is
     `glyph_bits! (padding_p, *ptr)` (C 24058 names the member directly, so the port does too
     rather than through `CHAR_GLYPH_PADDING_P`), `glyph->face_id = face_id` is
     `glyph_bits! (face_id, *glyph, face_id)`, the row flags (`reversed_p`, `continued_p`,
     `truncated_on_right_p`, `overlapping_p`, `mode_line_p`, `ends_at_zv_p`,
     `ends_in_ellipsis_p`, `fill_line_p`) are `glyph_row_bits!` readers/writers, `it`'s
     `end_of_box_run_p`/`constrain_row_ascent_descent_p`/`string_from_prefix_prop_p`/
     `face_before_selective_p` are `it_bits!`, and C's `int saved_end_of_box_run =
     it->end_of_box_run_p;` is `it_bits! (…) as c_int`, restored as `saved_end_of_box_run != 0`.
221. **Pointer arithmetic, `memmove` and the `short` accumulators.**  Every `to++`/`from--`
     becomes a `.offset (±1)` step with C's order kept; `it->glyph_row->glyphs[TEXT_AREA][k]` is
     `.glyphs[TEXT_AREA as usize].offset (k as isize)`; `(toend - g) * sizeof(*g)` is
     `toend.offset_from (g) as usize * core::mem::size_of::<glyph> ()`; `g - to - tused` is
     `g.offset_from (to) - tused as isize`; `from - end + 1` is `from.offset_from (end) as c_int
     + 1`; and C's `short += int`/`short -= int` (`insert_left_trunc_glyphs`,
     `compute_line_metrics`) is written in the widened form
     `((*gp).pixel_width as c_int + extra) as c_short`, because Rust's `+=` on a `c_short` would
     trap where C truncates.
222. **`as c_int` before a `<` is parenthesised.**  C's `row->used[LEFT_MARGIN_AREA] <
     WINDOW_LEFT_MARGIN_WIDTH (it->w)` compares a `short` with an `int`, so the port writes
     `((… as c_int) < WINDOW_LEFT_MARGIN_WIDTH! (it->w))` — the `(BEG as ptrdiff_t)` shape
     deviation 216 records; the four sites were the round's first build's only type errors.
223. **The whole `HAVE_WINDOW_SYSTEM` arms are recorded, not transcribed** (P-08, the unit's
     standing rule).  They leave their C tokens out of `c_body_text`, which is why `--check` owes
     no new `does not mention` row even though `append_space_for_newline` and
     `extend_face_to_end_of_line` name `get_it_property`, `calc_line_height_property`,
     `normal_char_ascent_descent` and `append_stretch_glyph` in their dead halves.
224. **Type spellings this round**: `SDATA`/`SBYTES` are `lisp.rs`'s fns; `WINDOW_FRAME! (w)` is
     the window.rs mirror's macro (`XFRAME (WINDOW_FRAME! (w))`); `it->glyph_row->used[TEXT_AREA]`
     is `(*(*it).glyph_row).used[TEXT_AREA as usize]` and the local-`it` form is
     `(*(it.glyph_row)).used[…]`; `MATRIX_FIRST_TEXT_ROW (it->w->desired_matrix)` needs the
     `as *mut glyph_matrix` cast (the window mirror's `*mut c_void`, deviation 213);
     `WINDOW_TAB_LINE_HEIGHT!`/`WINDOW_HEADER_LINE_HEIGHT!`/`WINDOW_BOX_HEIGHT_NO_MODE_LINE!` and
     `WINDOW_LEFT_MARGIN_WIDTH!`/`WINDOW_RIGHT_MARGIN_WIDTH!`/
     `WINDOW_{LEFT,RIGHT}_FRINGE_WIDTH!`/`MATRIX_FIRST_TEXT_ROW!`/`MATRIX_ROW_OVERLAPS_*!`/
     `MATRIX_ROW_{END_CHARPOS,ENDS_IN_MIDDLE_OF_CHAR_P}!`/`MATRIX_ROW_DISPLAYS_TEXT_P!`/`BVAR!`/
     `SET_TEXT_POS!`/`CHARPOS!`/`BYTEPOS!`/`PRODUCE_GLYPHS!`/`CHAR_GLYPH_PADDING_P!` are the
     mirrors' `macro_rules!` (the `!` form); `FRAME_WINDOW_P`/`FRAME_LINE_HEIGHT`/
     `FRAME_COLUMN_WIDTH`/`FRAME_BACKGROUND_PIXEL`/`FACE_FROM_ID`/`FACE_FOR_CHAR`/
     `FACE_FROM_ID_OR_NULL`/`CHAR_TO_BYTE` are fns; `FACE_FROM_ID (f,
     margin_fill_face_id)->background` is `(*FACE_FROM_ID (f, margin_fill_face_id)).background`;
     and `glyph->frame = it->f` is `(*glyph).frame = (*it).f as *mut c_void` (the glyph mirror's
     untyped pointer).
225. **Two `eassert!`s, the mirror's release arms** (deviations 208/217's shape): C 23898-23902's
     four-part fringe test and C 24109-24110's `pixel_width >= 0` /
     `ascent >= 0 && height > 0` pair carry their conditions unevaluated in the frozen build and
     are visible to the `--cfg enable_checking` probe only (the Gate evidence section runs it).

### Deviations, continued (the round's twenty-ninth pass: the line-number and line-prefix group)

226. **C's chained assignments became consecutive statements** (deviation 203/214's shape), each
     with C's chain in a comment: C 25092's `IT_STRING_CHARPOS (*it) = IT_STRING_BYTEPOS (*it) =
     0`, C 25094's `it->end_charpos = it->string_nchars = SCHARS (it->string)` and C 25719/C 25721's
     `tem_it.c = tem_it.char_to_display = ' '` / `= *p`.
227. **`struct it`'s `eol_pos` is named.**  The copy carried it as the `_pad_f: [u8; 16]` measured
     pad (offset 2536); `find_row_edges` reads `CHARPOS (it->eol_pos)`/`BYTEPOS (it->eol_pos)`
     (C 25387-25388), so the round names the member and adds the matching `offset_of!` assertion
     (the layout is unchanged — `text_pos` is the same 16 bytes).
228. **Four more zero-initialised locals** (deviation 1's rule): `bytepos` and `ignored` in
     `maybe_produce_line_number` (C leaves both to `display_count_lines_logically`, called through
     a `&raw mut`), `from` in `display_count_lines_visually` (C writes it with `SET_TEXT_POS`,
     whose two member writes Rust cannot see as an initialisation) and `from_buffer` in
     `handle_line_prefix` (C leaves it to `get_line_prefix_it_property`).  Each carries the note.
229. **C's bare `alloca` is the `SAFE_ALLOCA` family** (PORTING.md P-05; the port has no `alloca`).
     `char *lnum_buf = alloca (max (it->lnum_width, INT_STRLEN_BOUND (ptrdiff_t)) + 2 + 1)`
     (C 25642-25643) becomes `USE_SAFE_ALLOCA!`/`SAFE_ALLOCA!`, with gnulib's
     `INT_STRLEN_BOUND (ptrdiff_t)` measured 20 on the frozen target (the fn takes `isize`, so the
     fold is `max ((*it).lnum_width as isize, 20) + 2 + 1`).  C's early `return` inside the glyph
     loop (C 25735) needs the port's pool release, so it runs `SAFE_FREE! (sa_count)` before
     returning — C's `alloca` bytes go with the frame — and the normal exit (C 25795) runs it
     too.  The two tokens (`alloca`, `int_strlen_bound`) are the row this round owes to
     `docs/ports/macro-body-notes.tsv` (to-do items 4/73).
230. **`max`/`clip_to_bounds` take `isize`.**  C 25620's `clip_to_bounds (1, lnum_width,
     lnum_width_limit)` mixes `EMACS_INT` and `int`, so the port writes
     `clip_to_bounds (1, lnum_width, lnum_width_limit as isize) as c_int`; C 25638's
     `max (it->lnum_width, log10 (max_lnum) + 1)` mixes `int` with `double`, so the port writes
     the `isize` fold `max ((*it).lnum_width as isize, (log10 (max_lnum as c_double) + 1.0) as
     isize) as c_int` with C's expression in a comment (deviation 215's family).
231. **`desired_matrix` is the mirror's `*mut c_void`** (deviation 213's class), so
     `it->w->desired_matrix->nrows`/`->rows` is `(*((*(*it).w).desired_matrix as *mut
     glyph_matrix)).nrows`/`.rows` — the `(*((*w).current_matrix as *mut glyph_matrix)).rows`
     spelling the unit already uses (deviation 213's C 19892 note).
232. **`short`/`int` widenings.**  `it->glyph_row->used[TEXT_AREA]` is a `c_short`, so every
     comparison and assignment of `n` against it carries `as c_int` (C 25264-25271); C 25762's
     `(*u)++` is `*u = (*u as c_int + 1) as c_short` (deviation 221's amplification rule), and
     C 25753's `it->current_x += g->pixel_width` is `+= (*g).pixel_width as c_int`.
233. **Type spellings this round**: `it->what != IT_EOB` is `(*it).what != IT_EOB` (the unit's own
     `display_element_type`); `tem_it.bidi_it.type = WEAK_EN` is `tem_it.bidi_it.r#type = WEAK_EN`
     (the raw identifier the record's copy already uses) and `tem_it.bidi_it.resolved_level = 2`
     is a `c_char` store; `it->bidi_it.string.s = NULL` is `core::ptr::null ()`; `tem_it.len = 1`
     is a `c_int`; `it->line_wrap = TRUNCATE` is the unit's `line_wrap_method` constant.
234. **The round's bitfield spellings**: `it->string_from_display_prop_p`/
     `string_from_prefix_prop_p`/`multibyte_p`/`avoid_cursor_p`/`align_visually_p`/`bidi_p`/
     `line_number_produced_p` are the `it_bits!` readers/writers, `it->bidi_it.first_elt` is
     `bidi_it_bits! (first_elt, …)`, `it->bidi_it.string.from_disp_str`/`unibyte` are
     `bidi_string_bits! (…)`, `current_buffer->clip_changed` is
     `buffer_bits! (clip_changed, current_buffer)`, and the row flags (`reversed_p`,
     `continued_p`, `truncated_on_right_p`, `mode_line_p`, `ends_at_zv_p`,
     `ends_in_newline_from_string_p`) are `glyph_row_bits!` readers/writers (deviation 220's
     family).
235. **The round's `eassert!`s** (C 25082-25085's four-arm method test, C 25262-25265's four
     `unproduce_glyphs` assertions, C 25570-25571's two line-number invariants, C 25571's
     `bytepos` check, C 25639's `lnum_width > 0` and C 25749's empty-row check) live in the
     release build's `#[cfg (enable_checking)]` arm only, so `build-tu` does not compile them;
     the Gate evidence section's probe is what reads them.

### Deviations, continued (the round's thirtieth pass: `display_line`)

236. **C's `#define RECORD_MAX_MIN_POS` is a function-scope `macro_rules!`** (PORTING.md 2.1; the
     `BUFFER_POS_REACHED_P`/`IT_RESET_X_ASCENT_DESCENT` shape).  C's `#define` text is dropped and
     its trailing `#undef` (C 26934) has no Rust site; the macro's body reads the enclosing `it`
     and the four position locals, exactly as C's does, and the `macro-body-notes.tsv` row it owes
     (`RECORD_MAX_MIN_POS` / `false`) is named in the Progress section.
237. **Three zero-initialised locals** (deviation 1's rule / PORTING.md 1.5): the eight `UNINIT`
     scalars C declares at C 25870-25883 (`wrap_x`, `wrap_row_ascent`/`height`/`phys_ascent`/
     `phys_height`/`extra_line_spacing`, `wrap_row_min_pos`/`min_bpos`/`max_pos`/`max_bpos`,
     `wrap_face_id`, `min_bpos`, `max_bpos`), plus `prev_face_id` (C reads it only after the wrap
     block writes it) and `overlay_arrow_string` (C's comma assignment at C 26790) — Rust cannot
     prove either read is reached after the write, so each starts at 0/`Qnil ()`, marked
     `/* UNINIT */` where C's macro is what supplies it.
238. **C's two `goto back_to_wrap` (C 26331, C 26368) are a flag**, not a label: the port sets
     `back_to_wrap` and leaves C's `if`-chain through `break 'end_of_continued_line`, and C's
     `back_to_wrap:` body (C 26415-26441) runs once, after the chain, guarded by that flag.  C's
     `else if (wrap_row_used > 0)` arm is the flag set.  C's `at_end_of_line:` label (C 26569)
     with its backward `goto` (C 26702) is the port's `'at_end_of_line: loop`, whose `continue` is
     the `goto` and whose trailing `break` is the next iteration of the outer loop; C's unlabelled
     `while (true)` is named `'row` so that the two `break`s inside that inner loop leave it the
     way C's `break`s leave the `while` (PORTING.md 2.2 — the `buffer_pos_reached`/
     `move_further_back` shapes).  `--check` therefore owes the `display_line` / `goto,uninit` row
     (deviation 245).
239. **Type spellings and cast points this round** (deviation 213/230/232's classes):
     `MATRIX_ROW_VPOS`'s `isize` against `nrows`'s `c_int` (`as c_int`); C's implicit `int` ->
     `ptrdiff_t` promotion in `x_incr`'s product (`as ptrdiff_t` … `as c_int`); `min_hscroll`'s
     and `large_hscroll_threshold`'s `ptrdiff_t` (the latter `EMACS_INT`, so `as ptrdiff_t`);
     `max`'s `isize` arguments; `glyph->pixel_width`'s `short` (C widens it in
     `x + glyph->pixel_width` and truncates `it->last_visible_x - x` into it); `XFIXNUM`'s
     `ptrdiff_t` into `overlay_arrow_bitmap`'s `int`; `window_wants_tab_line`/`_header_line`'s
     `bool` into C's `int`; and `BVAR`'s buffer member, which the mirror spells
     `indicate_empty_lines_`.
240. **`(*(*it).w).desired_matrix as *mut glyph_matrix` needs its parentheses**:
     `(*((*(*it).w).desired_matrix as *mut glyph_matrix))`.  Rust's unary `*` binds tighter than
     `as`, so the bare form dereferences the `*mut c_void` before the cast (`E0606 casting c_void
     as *mut glyph_matrix`) — the `redisplay_window`/`MATRIX_ROW_VPOS` precedent (deviation 231).
241. **The round's bitfield spellings** (PORTING.md 1.3/P-80): `struct it`'s four `unsigned` fringe
     members (C 2908-2929) are 16- and 20-bit fields of the measured 96-bit `bits3` run, so the
     round adds `IT_STARTS_IN_MIDDLE_OF_CHAR_P` (byte 2217 bit 0) and the `it_bits3_get`/
     `it_bits3_set` pair plus eight `it_bits!` arms (the two bitmap numbers at bytes 0/2, the two
     face ids at bytes 4/8); the four `row->*_user_fringe_*` copies use `glyph_row_bits!`'s
     existing 16/20-bit arms, `glyph->type` is `glyph_bits! (type, …)`, and every other flag is an
     existing arm (`displays_text_p`, `continued_p`, `ends_at_zv_p`, `reversed_p`, …).
242. **The five call sites of `display_line` in this unit lose their `!= 0`.**  C's
     `if (display_line (&it, …))` is truthiness (C 21708, C 21927, C 22110, C 23031, C 23306) and
     C's definition returns `bool`; earlier rounds wrote `!= 0` because the burned scaffold entry
     declared the function `c_int` (the same stale typing to-do item 78 names).  C 17699's
     statement keeps discarding the value.  They are `E0308`s the definition landing exposed, and
     they are the port's only edits outside the new function.
243. **C's comma/chain expressions in the round** (deviation 2/226's shape): the overlay-arrow
     test keeps its assignment inside the condition as
     `&& { overlay_arrow_string = overlay_arrow_at_row (it, row); !NILP (overlay_arrow_string) }`
     (deviation 135/174's shape), C's three-way chain `it->wrap_prefix_width = it->current_x =
     it->hpos = 0;` becomes the three stores in C's order, `p = where = row->glyphs[area]` becomes
     `where_` then `p`, and the two `*p++ = *x++` walks become the store followed by the two
     increments (C's post-increment order).
244. **The round's `eassert!`s** (C 25883's `hpos`/`current_x` pair, C 26421's `wrap_face_id >= 0`,
     C 26537's `FRAME_WINDOW_P`, C 26548's `first_visible_x <= last_visible_x` and C 26848's
     `FIXNUMP`) live in the release build's `#[cfg (enable_checking)]` arm only, so `build-tu` does
     not compile them; the Gate evidence section's probe is what reads them.
245. **C's `if (!get_next_display_element (it))`/`if (!wrapped)` arms and the `IT_IMAGE` disjunct
     are transcribed as C writes them**, so the round's two `goto`s are the only tokens
     `--check` misses (deviation 238); the `uninit` token is the second one, and it is the
     `/* UNINIT */` markers' job to say where the C initialisers went (deviation 237), not a
     Rust token.

### Deviations, continued (the round's thirty-first pass: `current-bidi-paragraph-direction`)

246. **`switch` -> `match`** (the family of deviations 244/245 and the `cmgoto` row): C
     27016-27027's `switch (itb.paragraph_dir)` with its `case L2R`/`case R2L` (each a `return`
     plus an unreachable `break`) and its `default:` (`emacs_abort ()`) becomes a `match` over the
     three `bidi_dir_t` constants, the two arms `return`ing and the `_` arm calling
     `emacs_abort ()`; the `break` token survives in the port because the body's own scan loop
     uses one (the `macro-body-notes.tsv` row says so).
247. **Zero-initialised `struct bidi_it itb`** (PORTING.md 1.5, the `UNINIT` family): C declares it
     uninitialised and `bidi_init_it` fills it; Rust cannot leave it uninitialised, so it is
     `core::mem::zeroed ()` and every field C sets afterwards is set in C's order.
248. **C's assignment inside a condition**: `while ((c = FETCH_BYTE (bytepos)) == '\n' || …)` is
     the port's `while { c = FETCH_BYTE! (bytepos) as c_int; … }` block expression (the
     `trailing_whitespace_p` shape at C 11764), with `int c` as `c_int` and C's `'\f'` as `b'\x0c'`
     (`'\n'`/`' '`/`'\t'` keep their spellings).
249. **`AUTO_STRING (name, "[\f\t ]*\n")`** -> `AUTO_STRING! (name, c"[\x0c\t ]*\n".as_ptr ())`
     (the macro is the tree's; C's escaped literal is spelled with Rust's escapes).
250. **`BVAR (buf, field)` -> `BVAR! (buf, field_)`** (buffer.rs 399's macro pastes C's `## _`
     suffix), and `itb.string.from_disp_str = false` / `unibyte = false` go through this unit's
     `bidi_string_bits!` arms (the bit-run spelling, P-80) rather than field writes.
251. **C's `do { … } while (…)`** in `bidi-find-overridden-directionality` is the port's `loop`
     with the condition negated at the end (`if !(found == ZV! () && … ) { break; }`), and C's
     inner empty-bodied `while ((found = bidi_find_first_overridden (&itb)) < from_pos) ;` is the
     same `while` with an assignment block expression and an empty `{ }` body.
252. **Definite assignment (PORTING.md 1.5) in the same DEFUN**: C's `ptrdiff_t from_pos, to_pos,
     from_bpos; void *itb_data;` and the later `ptrdiff_t found;` are zero-initialised in the port
     because the two arms write `from_pos`/`to_pos` through pointer arguments
     (`validate_subarray`/`validate_region`) and Rust cannot see those writes; C's assignments and
     their order are unchanged.
253. **The DEFUN's own parameters are taken mutable**: C passes `&from`/`&to` to
     `validate_region`, which may replace them, so the port opens with `let mut from = from;` /
     `let mut to = to;` (the `composite.rs` 1989 shape) and passes `&raw mut`.
254. **`EQ (base_dir, Qright_to_left) ? R2L : L2R`** is the port's `if … { R2L } else { L2R }` with
     the result bound to a `bidi_dir_t` local, and the final
     `(from_pos <= found && found < to_pos) ? make_fixnum (found) : Qnil` is the port's `return
     if … { … } else { … };`.

### Deviations, continued (the round's thirty-second pass: `move-point-visually`)

255. **`goto simulate_display` -> a labelled block** (C 27285, C 27292; the label is C 27373):
     the two forward jumps leave the glyph path's `if` for the `simulate_display:` body that
     follows it, which falling out of the `if` also reaches.  The port wraps the `if` in the
     labelled block `'simulate_display: { … }`, so each jump is `break 'simulate_display` and
     the fall-through continues at the label's body (PORTING.md 2.2, the `'done`/`cancel:`
     labelled-block shapes this file already carries).
256. **`goto reseat` -> a labelled loop** (C 27437; the label is C 27421): the backward jump
     re-runs the reseat from the label, so the port starts `'reseat: loop` at the label's own
     statement and the jump is `continue 'reseat`; the `break` at the loop body's end is C's
     fall-through out of the label (the `retry:` shape at `get_next_display_element`).
257. **The in-function macro `ROW_GLYPH_NEWLINE_P`** (C 27196, `#undef`ed at C 27634) becomes a
     function-scoped `macro_rules!` in the same place, with C's `ROW`/`GLYPH` as `$row`/`$glyph`
     (PORTING.md 2.1, the `RECORD_OVERLAY_STRING` shape).  `row->continued_p` is the mirror's
     `glyph_row_bits! (continued_p, row)` and the glyph's four members are the `glyph_bits!`
     pair; C's `#undef` is a preprocessor directive with no Rust site, and the macro's scope
     ends with the function's block.
258. **C's three `for` loops** — `for (g = gpt + dir; (dir > 0 ? g < e : g > e); g += dir)`
     (C 27232), `for ( ; g < e; g++)` (C 27304) and `for ( ; g >= e; g--)` (C 27342) — are the
     port's `while` loops with C's init before them and C's increment at the end of the body.
     The two `continue`s (C 27326, C 27364) would skip that increment in Rust, so the port
     writes the increment before each one (`{ g = g.offset (1); continue; }` /
     `{ g = g.offset (-1); continue; }`), which is what C's own `for` increment does on
     `continue`.
259. **C's assignment inside the `if` condition** `(row = MATRIX_ROW (w->current_matrix,
     w->cursor.vpos))->enabled_p` (C 27226) is the port's last `&&` operand as a block
     expression that assigns `row` and then reads `glyph_row_bits! (enabled_p, row)`; `row` is
     declared `*mut glyph_row = core::ptr::null_mut ()` because Rust cannot leave it unwritten
     (PORTING.md 1.5, the `UNINIT` class).
260. **Zero-initialised C locals** (PORTING.md 1.5): `struct text_pos pt` and `struct it it`
     (C 27390-27391) and `struct it it2` (C 27529) are `core::mem::zeroed ()`, and so is
     `struct text_pos new_pos` (C 27563) because its composition arm writes its two members one
     at a time (C's `new_pos.charpos = …; new_pos.bytepos = -1;`), which Rust cannot do to an
     uninitialised aggregate.  C's `void *it_data = NULL` (C 27528) is C's own NULL, so it
     stays `core::ptr::null_mut ()`.
261. **C's chained stores** `it.current_x = it.hpos = it.current_y = it.vpos = 0` (C 27423) and
     `it.current_x = it.current_y = it.hpos = 0` (C 27506) are the port's separate stores in
     C's right-to-left order (`current_x`, `hpos`, `current_y`, `vpos`; and `current_x`,
     `current_y`, `hpos`) — `=` is not an expression in Rust (the `row->enabled_p = …` chain's
     class).
262. **`gpt->resolved_level - row->reversed_p`** (C 27247): `resolved_level` is the glyph's
     7-bit member (`glyph_bits! (resolved_level, *gpt)`) and `reversed_p` is a `bool_bf : 1`
     row member, so the port writes C's conversion to `int` explicitly as
     `(glyph_row_bits! (reversed_p, row) as c_int)`.
263. **`w->current_matrix` is `*mut c_void` in the mirror** (`include/window.rs` has not typed
     the pointer yet), so the port casts at its use sites: `(*w).current_matrix as *mut
     glyph_matrix` for the `nrows` read and for `MATRIX_ROW!`, `MATRIX_FIRST_TEXT_ROW!` and
     `MATRIX_BOTTOM_TEXT_ROW!` (the `set_cursor_from_row`/`display_line` precedent).
264. **The DEFUN's own `row` is shadowed** by C's block-local `struct glyph_row *row =
     it.glyph_row;` (C 27447), so the port's inner binding is `let row: *mut glyph_row =
     it.glyph_row;` and C's `it.glyph_row = NULL` / `it.glyph_row = row` pair restores the
     outer iterator's row exactly as in C.
265. **One `macro-body-notes.tsv` row is owed** (the integrator appends it; item 4 of the
     to-do list): `Fmove_point_visually` / `goto` — C's two `goto simulate_display` and its
     `goto reseat` are the labelled block and the labelled loop above, so the `goto` token has
     no Rust spelling and `--check` reports `subr Fmove_point_visually does not mention goto`
     (its only such row).  With that row appended the `does not mention` set returns to 0.

### The thirteen stale boxes of this round's sweep

The ten function boxes of C 4667-5790 (`handle_fontified_prop` through `display_min_width`) and
the three macro boxes of `AINC` (C 17604), `GIVE_UP` (C 22628) and `RECORD_MAX_MIN_POS`
(C 26024) were landed by earlier rounds but left unticked (that round's Decisions sentence says
so: "Unticked and reported, not ticked here").  This round checks the artifact rather than the
sentence: each of the ten names has its definition in `femacs/src/xdisp.rs` and each of the three
macros has its `macro_rules!`, and `python3 tools/extract-abi.py --check` reports none of the
thirteen ("missing") — so the boxes are ticked now and `tools/check-ledger-ticks.py` binds them
(0 failures).  The `--check` line count is unaffected either way: those thirteen were never in it.

### Deviations, continued (the round's thirty-third pass: the bidi levels, the menu bar and the mode lines)

266. **The round's two section banners are dropped**, the `Bidi` banner's shape: C 27756-27759's
     `Menu Bar` banner and C 28103-28105's `Mode Line` banner are pure decoration, and the port
     keeps C's *function* comments verbatim while dropping the banner boxes (the previous round's
     omission, recorded there as well).
267. **The round's `#if` arms are dead on the frozen target and recorded, not transcribed**
     (`HAVE_WINDOW_SYSTEM`, `HAVE_NS`, `HAVE_X_WINDOWS`, `HAVE_NTGUI`, `USE_X_TOOLKIT` and
     `USE_GTK` are all undef in `docs/baseline/config/darwin-arm64.macros.tsv`): C 27768-27791's
     five "don't do all this for graphical frames" early returns; C 27797-27815's
     `USE_X_TOOLKIT`/`USE_GTK` and `HAVE_X_WINDOWS || HAVE_ANDROID` iterator setups (the live
     text is C's trailing `else` body at C 27817-27824, the TTY case); and C 27876-27914's
     `HAVE_WINDOW_SYSTEM && !HAVE_EXT_MENU_BAR` right-end shadow plus the menu-bar height
     adjustment.  C 27920's `#ifndef HAVE_ANDROID` is live (`HAVE_ANDROID` is undef), so the
     three TTY menu helpers are carried; the `menu_window`/`face` locals of the dead
     `display_menu_bar` arm are named nowhere in the port's live text (the same rule that keeps
     `#undef`s and forward declarations out).
268. **`display_tty_menu_item`'s `WIDTH` is a `mut` parameter.**  C's body writes it
     (`width--` at C 28073, `width -= item_len` at C 28080, `width -= 1`), and a Rust parameter
     binding is immutable by default, so the signature is
     `(item_text: *const c_char, mut width: c_int, …)` — the same value, written in place.
269. **`deep_copy_glyph_row`'s three `memcpy`s and the `min`** are stated in the port's
     spellings: `sizeof to->glyphs` is `core::mem::size_of_val (&(*to).glyphs)`, the copy length
     is `min ((*from).used[TEXT_AREA] as isize, to_used as isize) as usize
     * core::mem::size_of::<glyph> ()` (C's `min` is the `lisp.h` helper, whose port takes
     `isize`), and `*to = *from` is the port's `glyph_row` structure assignment (`Copy`, the
     mirror's own derive).  `pointers` is `[core::ptr::null_mut (); (1 + LAST_AREA) as usize]`.
270. **`bidi-resolved-levels`'s guard keeps C's assignment inside the test**: C 27702's
     `(row = MATRIX_ROW (w->current_matrix, nrow))->enabled_p` is the port's
     `{ row = MATRIX_ROW! (…); glyph_row_bits! (enabled_p, row) }`, and its two
     `for (i = 0; g1 < g; i++, g1++)` loops (C 27714, C 27726) are `while` loops with both
     increments at the body's end (the file's established shape).  Its five `int d1 … d5;`
     and `nrow`/`row`/`g`/`e`/`g1`/`nglyphs`/`i` are zero-initialised (PORTING.md 1.5).
271. **`display_mode_line`'s two `do { … } while (…)` loops** (C 28270-28279's `while
     (!NILP (end))` and C 28292-28351's `while (XFIXNUM (end) < SCHARS (mode_string))`) are the
     port's `loop` with the trailing test as `if … { break; }` at the body's end (PORTING.md 2.2,
     deviation 251's shape — the loop's body, not a `break` that skips it).
272. **`BVAR`'s trailing underscore is spelled at the call site** (P-37's note in
     `include/buffer.rs`): C 28160/28172/28184's `BVAR (current_buffer, mode_line_format)` and
     its two siblings are `BVAR! (current_buffer, mode_line_format_)`,
     `BVAR! (current_buffer, tab_line_format_)` and `BVAR! (current_buffer,
     header_line_format_)` — the mirror's `buffer` fields carry the `_` C's macro appends.
273. **`safe_set_text_properties` is `unsafe extern "C" fn`.**  C hands its address to
     `internal_condition_case_n` (C 28550), so the port's definition must have C's ABI
     (PORTING.md 1.4/§"the address is handed out", the `dsafe_eval_handler` shape); C's
     `args[0] … args[3]` on a `Lisp_Object *` are `*args.offset (0) … *args.offset (3)` (the
     `alloc.rs` spelling — a raw pointer does not index).
274. **Zero-initialised locals of this round** (PORTING.md 1.5): `display_mode_lines`'s `n`;
     `display_mode_line`'s `face`, `end`, `elt`, `i`, `i_byte`, `prev`;
     `display_tty_menu_item`'s `rf`; `display_mode_bar`'s `i`; and the exception locals of 270.
275. **`display_mode_line`'s `mode_line_target = MODE_LINE_DISPLAY;` and
     `display_mode_element (&it, 0, 0, 0, format, Qnil, false)`** keep C's argument order and
     C's `Qnil` (the port's `Qnil ()`), and C's `Qnil` in `record_unwind_protect`'s data
     argument is spelled the same way; `FRAME_KBOARD (it.f)` is `frame.rs`'s `*mut c_void`
     (P-72 rule 3), so the call casts to `*mut kboard` (the mirror chain's `keyboard.rs` type).
276. **The round's four use-site declarations** keep C's prototypes in `/// C:` lines
     (PORTING.md 0.1-2/P-72 rule 3): `menu_separator_name_p` (keyboard.h 473), `push_kboard`
     (keyboard.h 480), `pop_kboard` (keyboard.h 481) and `fill_up_frame_row_with_spaces`
     (dispextern.h 3929).  They are integrator items 80 below.
277. **`display_menu_bar`'s `items` is not `const`** — C's `Lisp_Object items;` is assigned once
     (C 27842) and read by the `AREF`/`ASET` loop, so the port declares `let items:
     Lisp_Object;` and assigns it at C's own line (the file's `let window: Lisp_Object;` shape);
     `it.f` is C's field read, `FRAME_MENU_BAR_ITEMS (it.f)` the mirror's macro.
278. **`display_tty_menu_item`'s `saved_used` is `c_short`** — `glyph_row.used` is
     `[c_short; 4]` in the mirror (C's `short used[1 + LAST_AREA]`), so C's
     `max (saved_used, row->used[TEXT_AREA])` is
     `max (saved_used as isize, (*row).used[TEXT_AREA as usize] as isize) as c_short` and the
     stores cast too.
279. **`move_elt_to_front`'s `maybe_quit ()` (C 28422)** is carried at C's own position inside
     the loop, and C's `else prev = tail;` / `tail = XCDR (tail);` keep C's two-branch shape
     (the `if` returns, so the port writes the `else` C writes rather than falling through).

### Integrator to-do list (the mirror gaps this unit reports and does not touch)

1. `include/window.rs`: the accessor macros of §"Use-site declarations" 1 should move into the
   mirror when the display chain's `CURRENT_*_LINE_HEIGHT` prerequisites land (P-26: one
   spelling per construct); `window.rs`'s copies are then to be deleted too.
2. `include/window.rs`: `echo_area_window`, `update_mode_lines` and
   `windows_or_buffers_changed` are declared at the mirror's top level (window.h
   1146/1154/1161).  A unit that *defines* one cannot also define it at its own top level
   (E0428), so those three declarations should move into a module the mirror glob-imports
   (the `window_redisplay_decls` shape); this unit works around it (deviation 7) and will drop
   the workaround once the mirror changes.
3. `include/dispextern.rs`: `struct it`, `struct bidi_it`, `struct iterator_stack_entry`,
   `enum display_element_type`, `enum it_method`, `enum line_wrap_method`,
   `enum move_operation_enum`, `enum prop_handled`, `enum prop_idx`, `OVERLAY_STRING_CHUNK_SIZE`,
   `IT_STACK_SIZE`, the `IT_*`/`BYTEPOS`/`CHARPOS`/`SET_TEXT_POS` macros, `PRODUCE_GLYPHS`,
   `struct redisplay_interface` and `FACE_FROM_ID`'s family are the mirror gaps this unit
   declares for itself (the `indent.rs`/`dispnew.rs` precedent).
4. `docs/ports/macro-body-notes.tsv`: append these rows (the complete list this unit leaves;
   everything else `--check` reports is a `missing …` line for the not-yet-transcribed
   remainder).  The first four are still open from the previous round (only the
   `SAVE_IT if (CACHE)` row is in the table today), the fifth is the
   `maybe_produce_line_number` round's, and the sixth (the bottom row below) is this
   thirty-second round's:

   ```
   redisplay_trace	va_list,va_start,va_end	P-04: C's `va_list ap; va_start (ap, fmt); vfprintf (stderr, fmt, ap); va_end (ap);` is the port's `...` parameter bound once as `VaList` (the dump_trace row's shape)
   move_trace	va_list,va_start,va_end	same as redisplay_trace
   SAVE_IT	false	C's `do { … } while (false)` has no Rust counterpart (PORTING.md 2.2, the CHECK_CODING_SYSTEM/SAFE_ALLOCA_LISP class): the port's block runs the guard once and never mentions `false`
   SAVE_IT	if (CACHE)	C's `void *CACHE` tested as a boolean has no Rust counterpart: the port's arm is `if !$CACHE.is_null ()`
   maybe_produce_line_number	alloca,int_strlen_bound	the twenty-ninth round's row: C's bare `char *lnum_buf = alloca (max (it->lnum_width, INT_STRLEN_BOUND (ptrdiff_t)) + 2 + 1)` is the port's `SAFE_ALLOCA` family (PORTING.md P-05, the `AVAIL_ALLOCA` row) with gnulib's `INT_STRLEN_BOUND (ptrdiff_t)` measured 20 on the frozen target (deviation 229; the `message_dolog`/`ensure_echo_area_buffers` rows' shape)
   Fmove_point_visually	goto	the thirty-second round's row: C's two `goto simulate_display` (C 27285, C 27292) and its `goto reseat` (C 27437) are the port's labelled block and labelled loop (deviations 255/256), so the `goto` token cannot reappear; it is the only `does not mention` line this round's body leaves
   ```
5. `docs/baseline/config/cond-eval-verified.txt`: **nothing to add** —
   `src/xdisp.c` is already line 420 of that file (the window.c trap does not apply here).
6. `set_buffer_internal_1` and `buffer_window_count` are already declared (and, for the
   latter, defined) by `include/buffer.rs`; this unit declares neither (the integrator's
   2026-10-04 07:3x mirror-dupes finding).
7. `include/window.rs` now carries `window_wants_mode_line`/`_header_line`/`_tab_line` (its
   `window_wants_decls` module, window.h 1213-1215), so this unit's local copies were deleted
   (the 2026-10-04 mirror-dupes FAIL, 3 rows) and `check-mirror-dupes.py` is back to its one
   `insdel.rs` ratchet row.  `SET_TEXT_POS_FROM_MARKER` (dispextern.h 322) is *not* in any
   mirror this unit includes, so the unit still spells it itself (deviation 16).
8. `include/window.rs`: `window_parameter` (window.h 1209) is a mirror gap — only
   `window_parameters` the field is mirrored; `include/dispextern.rs`: `ITERATOR_AT_END_OF_LINE_P`
   (dispextern.h 2954-2957) is a gap too.  Both are declared at the use site for now
   (deviation 16).
9. `include/lisp.rs`: `internal_condition_case_n` (lisp.h 4808), `pop_handler` (lisp.h 4814)
   and `push_handler_bind` (lisp.h 4815) are declared by lisp.h and defined by eval.c, but the
   mirror does not carry them (only `keyboard.rs`'s own extern block has copies, and it is the
   keyboard.c unit, not a mirror).  This unit declares them at the use site for now
   (deviation 21); they belong in the mirror (the `other_units` block) once another unit needs
   them too.

10. `include/dispextern.rs`: the members `struct it` now names beyond its batch-II prefix
    (`stop_charpos`, `prev_stop`, `base_level_stop`, `end_charpos`, the four narrowing
    positions, `dp`, `dpvec`, `n_overlay_strings`, `overlay_strings_charpos`,
    `saved_face_id`, `start`, `tab_width`, `truncation_pixel_width`/
    `continuation_pixel_width`, `paragraph_embedding`, and — from the twenty-ninth round — the
    `eol_pos` that used to be the copy's `_pad_f`), `struct composition_it`,
    `struct bidi_it`'s `bytepos`/`charpos`/`string`/`w` (and `struct bidi_string_data`'s two
    `bool_bf : 1` members through the unit's own `bidi_string_bits!`), the fourth pass's
    `struct composition_it` `stop_pos` and `struct bidi_it` `resolved_level`, `struct face`,
    `struct face_cache`, `FRAME_FACE_CACHE`, `FACE_FROM_ID_OR_NULL`, `face_bits!` with
    `FACE_BOX_SHIFT`/`FACE_BOX_MASK`, `face_change` (2068), `bidi_init_it` (3550),
    `bidi_move_to_visually_next` (3551), `reseat_at_previous_visible_line_start` (3619),
    `lookup_basic_face` (3813), `init_frame_faces` (3821), `recompute_basic_faces` (3823),
    `free_all_realized_faces` (3833), `bidi_level_start` (3559) and `update_redisplay_ticks`
    (3721) are the mirror gaps
    this unit declares for itself (P-66/§"Use-site declarations" 3 and 6); they should move
    into the mirror when the display chain's prerequisites land (P-26), at which point this
    unit's local copies are deleted.
11. `include/window.rs`: `window_body_width`/`window_body_height` (window.h 1223-1224) are
    declared at the use site (the `indent.rs` copies are the same gap).
12. `include/lisp.rs`: `find_newline1` (lisp.h 5033-5034) is declared at the use site; the
    mirror chain carries only `find_newline` (lisp.h 5035-5036, which `indent.rs` also
    declares).  `frame.rs` too: `FRAME_WINDOW_CHANGE`'s l-value form (frame.h 1191) is a
    *macro* in this unit because the mirror's same-named accessor is read-only.
13. `include/xwidget.h`: the header has no mirror; this unit inlines its `#else` arm's two
    `INLINE`s at the use site (§"Use-site declarations" 7).  A mirror would give
    `struct xwidget` a home, at which point `struct it`'s `xwidget` member can stop being
    `*mut c_void` and `lookup_xwidget` can return the named pointer (deviation 49).
14. `include/dispextern.rs` / `include/lisp.rs`: `smaller_face` (dispextern.h 3814),
    `face_with_height` (3815) and `FUNCTIONP` (lisp.h 4799) are declared at the use site
    (§"Use-site declarations" 8); they belong in the mirrors once another unit needs them.
15. `include/dispextern.rs`: `struct it`'s five new members (`xwidget` 2344, `slice` 2352,
    `space_width` 2384, `voffset` 2392, `font_height` 2400) join item 10's list of members
    this unit names for itself.
16. `include/dispextern.rs`: this round adds `struct it`'s `overlay_strings_at_end_processed_p`
    bit (2216/5), `struct composition_it`'s `ch` (16) and `nglyphs` (40), the whole
    `struct iterator_stack_entry` reached-member set with its `u` union, and the
    `overlay_entry` record (which is xdisp.c's own, not a header's — it has no mirror to move
    to).  They join item 10/15's list.
17. `include/dispextern.rs`: `bidi_push_it`/`bidi_pop_it` (3554-3555) are declared at the use
    site (§"Use-site declarations" 9) and belong next to the other `bidi_*` declarations in
    the mirror.
18. `docs/ports/macro-body-notes.tsv`: the four rows the fifth round owed (`load_overlay_strings`,
    `push_it`, `pop_it`, `RECORD_OVERLAY_STRING`) are in the table now; this round's three
    (`strings_with_newlines`, `back_to_previous_visible_line_start`,
    `lookup_glyphless_char_display`) are listed in the Progress section.
19. `include/dispextern.rs`: this round names `struct it`'s `glyphless_method` (2748),
    `struct bidi_it`'s `disp_pos` (2187)/`disp_prop` (2188), and spells
    `enum glyphless_display_method` (2347-2358) at the use site (§"Use-site declarations"
    10).  They join items 10/15/16's list.
20. `include/dispextern.rs` / `include/window.rs` / `include/lisp.rs`: the four declarations
    of §"Use-site declarations" 10 belong in the mirrors next to their siblings
    (`bidi_push_it`/`bidi_pop_it` from item 17 and `bidi_mirror_char` from item 11 are the
    same gap).
21. `include/dispextern.rs`: this round names `struct it`'s `ctl_chars` (160),
    `struct bidi_it`'s `type` (40, `r#type` in the port) and `struct composition_it`'s
    `reversed_p` (44)/`from` (72)/`to` (76).  They join items 10/15/16/19's list.
22. `docs/ports/macro-body-notes.tsv`: the eighth round's two rows
    (`get_next_display_element`, `set_iterator_to_next`) are in the table now; **this round
    owes none**.
23. `docs/ports/macro-body-notes.tsv`: the row the tenth round owed —
    `move_it_in_display_line_to` / `goto,uninit` / `xdisp.c 10109-10877: C's six `goto`s
    (C 10308, 10314 to `done:`; C 10399, 10627, 10698, 10702 to `buffer_pos_reached:`) become
    labelled `break`s, and `closest_pos`'s C initialiser is the `UNINIT` macro (C 10120),
    which the port spells `0` (PORTING.md 1.5)` — is still held by the integrator until this
    unit closes; this round adds two more of the same kind (item 27).
24. `include/dispextern.rs`: the tenth round named `struct composition_it`'s `charpos` (2419-2420,
    offset 56) — it had been inside the copy's `_pad_c0` — and `struct it`'s
    `constrain_row_ascent_descent_p` bit (2697-2699, byte 2217 bit 2).  They join items
    10/15/16/19/21's list.
25. `include/dispextern.rs` / `include/lisp.rs`: the three forward declarations the tenth round
    added to the scaffold (`handle_line_prefix` C 1132, `maybe_produce_line_number` C 1165,
    `should_produce_line_number` C 1166) are this unit's own later definitions, not mirror
    items (the item-23 rule for the seven `next_element_from_*`).
26. `docs/ports/xdisp.md`'s own §"Use-site declarations" 1 lists the scaffold members by name;
    its `move_it_in_display_line_to` is now defined (updated above).  The `/// C:` line of
    that burned entry said C 1204-1206 where C's prototype is C 1199-1201 (deviation 83) —
    the other entries in that block were re-checked this round and are correct.
27. `docs/ports/macro-body-notes.tsv`: **two rows are owed by the round before this one** —
    `move_it_to` / `case,default,goto,out,switch` / `xdisp.c 10928-11260: C's `switch (skip)`
    (C 11125) with its `case`/`default` arms becomes a `match`, `goto out` (C 11130, 11163)
    becomes `break` because `out:` is the loop's own exit (deviation 85)`, and
    `move_it_vertically_backward` / `goto` / `xdisp.c 11273-11452: C's `move_further_back:`
    label with its backward `goto move_further_back` (C 11416) becomes a labelled `loop` with
    `continue 'move_further_back` (PORTING.md 2.2)`.
28. `docs/ports/macro-body-notes.tsv`: **two more rows are owed by this round** —
    `add_to_log` / `va_end,va_list,va_start` / `xdisp.c 12278-12285: C's `va_list ap; va_start
    (ap, format); vadd_to_log (format, ap); va_end (ap);` is the port's `...` parameter bound
    once as `VaList` (the `move_trace`/`redisplay_trace` row's shape)`, and `message_dolog` /
    `int_strlen_bound,pridmax` / `xdisp.c 12458-12463: C's `INT_STRLEN_BOUND (dups)` sizes the
    local array (measured 20 + 10 = 30, deviation 93) and `PRIdMAX` (measured `"jd"`,
    deviation 94) is pasted into the `sprintf` format, so neither token survives`.
29. `include/window.rs` / `include/lisp.rs`: this round declares five use-site externs that
    belong in the mirrors next to their siblings — `decode_live_window` (window.h 1210),
    `wset_buffer` (window.h 1231), `insert_1_both` (lisp.h 4331-4332), `del_range_both`
    (lisp.h 4348) and `chars_in_text` (lisp.h 4226).  They join items 10/15/16/19/21/25/26's
    list.
30. `include/dispextern.rs`: this round names `struct face`'s `font` (C 1828, offset 216) —
    it had been inside the copy's `_pad0` — and (with the tenth round's items) `struct it`'s
    `line_number_produced_p` write arm, which stays in this unit's `it_bits!` until the
    mirror's own accessor arrives.  They join items 10/15/16/19/21/26's list.
31. `include/dispextern.rs`: when `struct it` lands in the mirror, its `len` member must be
    `c_int` (with the four-byte hole before `cmp_it`), not the `ptrdiff_t` this unit carried
    until this round (deviation 71) — and the seven `next_element_from_*` definitions are the
    unit's own (`unsafe extern "C" fn`, no `no_mangle`), not mirror items (deviation 72).
32. `docs/ports/macro-body-notes.tsv`: **this round owes one row** —
    `ensure_echo_area_buffers` / `int_strlen_bound` / `xdisp.c 12906: C's `char name[sizeof
    name_fmt + INT_STRLEN_BOUND (int)];` sizes the local array from the C macro (measured
    16 + 11 = 27, deviation 105); `INT_STRLEN_BOUND` has no Rust token`.  With the five rows
    items 23/27/28 hold, this unit's `does not mention` count returns to 0 when they land.
33. `include/window.rs` / `include/dispextern.rs` / `include/lisp.rs` / `include/character.rs`:
    this round's seven use-site externs (`cancel_echoing` lisp.h 5082,
    `do_pending_window_change` dispextern.h 3923, `clear_glyph_matrix` dispextern.h 3906,
    `errputc`/`errwrite` sysstdio.h 32-33, `strtoimax` inttypes.h, `doprnt` lisp.h 4715-4716)
    belong in the mirrors next to their siblings; `include/frame.rs` also lacks a reader for
    frame.h 331's `glyphs_initialized_p` bit, which this unit spells itself (the same gap
    family as item 8's `ITERATOR_AT_END_OF_LINE_P`).
34. `include/frame.rs` / `include/window.rs` / `include/dispextern.rs` / `include/lisp.rs`:
    this round's eleven use-site externs (`frame_redisplay_p`, `is_tty_frame`,
    `window_outdated`, `call_debugger`, `clear_current_matrices`, `redraw_frame`,
    `update_single_window`, `combine_updates_for_frame`, `grow_mini_window`,
    `shrink_mini_window`, `insert_from_string`) belong in the mirrors next to their siblings;
    `include/frame.rs`'s `struct redisplay_interface` sits in this unit, not the mirror, so its
    new `flush_display` member (offset 72) is recorded here for whoever mirrors the record.
35. `docs/ports/macro-body-notes.tsv`: **this round owes none** — the six rows items 23/27/28/32
    hold are unchanged, and `--check`'s `does not mention` count stayed at six.
36. `docs/ports/macro-body-notes.tsv`: **this round owes one row** —
    `restore_selected_window` / `uninit` / `xdisp.c 14541: C's `Lisp_Object frame UNINIT;`
    (`UNINIT` is `= {0,}` in the frozen configuration) has no Rust counterpart, so the port
    initialises it to `Qnil ()` (PORTING.md 1.5)`.  With the integrator's six rows already in
    the table, this is the only `does not mention` line `--check` reports for this unit.
37. `include/dispextern.rs` / `include/lisp.rs` / `include/frame.rs` / `include/window.rs`:
    this round's seven use-site externs (`clear_glyph_row`, `menu_bar_items`, `tab_bar_items`,
    `record_unwind_save_match_data`, `root_frame`, `safe_run_hooks`, `window_list`) belong in
    the mirrors next to their siblings; `TAB_BAR_ITEM_CAPTION`/`TAB_BAR_ITEM_NSLOTS` (and the
    four enumerators before them) are spelled in this unit because the mirror does not carry
    `enum tab_bar_item_idx`.
38. `include/termchar.rs`: **`tty_handle_tab_bar_click` needs a home.**  The mirror declares it
    (termchar.h 238-239) but nothing defines it, and this unit cannot (E0428, deviation 127).
    Either the mirror's declaration moves somewhere a defining unit can coexist with it (the
    `window_redisplay_decls` shape item 2 suggests), or the C 15615-15654 body lands in a unit
    that does not include termchar.h's declaration.  Until then `--check` lists
    `missing function tty_handle_tab_bar_click` for this unit and the ledger box stays unticked.
39. `include/dispextern.rs` / `xtask`: the mirror's `pub struct it` (and its
    `bidi_it`/`composition_it`/`iterator_stack_entry`/`it_stack` neighbours) is gated by
    `#[cfg (feature = "dispextern_face_records")]`, which `build-tu` never sets — so the record
    is dead code for every unit.  Either drop the gate or have `build-tu` pass the flag (the
    3.183 ruling that cfg axes are bare `--cfg` names argues for the former).  When it becomes
    visible, this unit's own copy and its `composition_it`/`bidi_it`/`iterator_stack_entry`
    copies must be deleted in the same round (their layouts would clash).
40. `docs/ports/macro-body-notes.tsv`: **this round owes one row** —
    `hscroll_window_tree` / `double` / `xdisp.c 16630: C's `double hscroll_step_rel;` is the
    port's `f64`, so the C token cannot appear (deviation 129)`.

41. `docs/ports/macro-body-notes.tsv`: **this round owes** `overlay_arrow_in_current_buffer_p` /
    `continue` / `xdisp.c 17104: C's `for (…; CONSP (vlist); vlist = XCDR (vlist))` loop carries
    a `continue`, and the port's `'body:` labelled block holds C's increment at the end of the
    body so every path runs it (PORTING.md 2.2, deviation 133)`.

42. `docs/ports/macro-body-notes.tsv`: **this round owes** `overlay_arrows_changed_p` /
    `continue` / `xdisp.c 17133 and 17136: the same `'body:` shape (deviation 133)`.
43. `docs/ports/macro-body-notes.tsv`: **this round owes** `update_overlay_arrows` / `continue` /
    `xdisp.c 17173 and 17179: the same `'body:` shape (deviation 133)`.
44. `docs/ports/macro-body-notes.tsv`: **this round owes** `overlay_arrow_at_row` / `continue` /
    `xdisp.c 17211: the same `'body:` shape (deviation 133)`.
45. `docs/ports/macro-body-notes.tsv`: **this round owes** `STOP_POLLING` / `false` /
    `xdisp.c 17341: C's `do { … } while (false)` runs its block once; Rust has no do-while, so
    the port writes the block and the `false` has no site (deviation 140)`.  `RESUME_POLLING`
    needs **no** row (its body still mentions `false`).
    (Items 41-44 may also be written as one glob row `overlay_arrow*` plus
    `update_overlay_arrows` if the integrator prefers.)
46. `docs/ports/macro-body-notes.tsv`: **this round owes one row** — `redisplay_internal` /
    `end_of_redisplay,goto,modiff,save_modiff` / `xdisp.c 18221/17792: C's `end_of_redisplay:`
    label and its `goto end_of_redisplay` are the port's `break 'retry` (the label has no Rust
    site), and `goto` is C's jumping itself; `xdisp.c 17530: C's `SAVE_MODIFF`/`MODIFF` are the
    mirror's `BUF_SAVE_MODIFF!`/`BUF_MODIFF!` macros (the buffer.rs 1610 spelling), so the C
    member tokens cannot appear (deviation 147)`.
47. The scaffold's shape changed: `redisplay_internal` is deleted and eleven forward
    declarations are added (`unwind_redisplay`, `redisplay_window_error`, `redisplay_window_1`,
    `mark_window_display_accurate_1`, `block_buffer_flips`, `unblock_buffer_flips`,
    `mark_window_display_accurate`, `redisplay_windows`, `set_cursor_from_row`,
    `cursor_row_fully_visible_p`, `display_line`) — 13 -> 23 entries.  Four of them
    (`unwind_redisplay`, `redisplay_window_error`, `redisplay_window_1`,
    `set_cursor_from_row`) are C statics whose *address* the port hands out, so they must be
    `unsafe extern "C" fn` when they land.
48. `xtask`/the ledger: `--check` now reports one extra non-missing row —
    `function mark_window_display_accurate is not static in C but has no
    #[unsafe(no_mangle)] in the port` (deviation 150).  It is not a `missing` regression; it
    appears because the unit declares the name one round before it defines it.
49. `include/disptab.rs:67`: **move `disp_char_vector`'s declaration into a module** and
    glob-import it (the `include/window.rs` `window_redisplay_decls` shape).  Left at its
    mirror's top level, every unit that *defines* it — `xdisp.c` is the C owner — is E0428;
    this round worked around it locally (deviation 153).
50. **The 3.251 link-gate method pointed at xdisp's object found four macro redirects that
    are declared as symbols.**  `thread.h` defines all of
    `buffer.h`/`lisp.h`'s `current_buffer` (thread.h 147) and `specpdl` / `specpdl_end` /
    `specpdl_ptr` (thread.h 131/135/139) as **macros** over `current_thread`, and
    `include/thread.rs:156/158` already carries the right `macro_rules!` shape — but
    `include/buffer.rs:27` and `include/lisp.rs:3783-3785` declare them as `extern "C"`
    statics, which nothing defines, so the port's objects import `_current_buffer`,
    `_specpdl`, `_specpdl_end`, `_specpdl_ptr` and the final link cannot resolve them
    (`nm -u build/emacs-ref/src/xdisp.o` has none of the four; `build/femacs/xdisp.o` has
    all four).  These are the same class as the integrator's `fflush_unlocked`/`VFORK`
    findings.
51. **`putc`** (C 12809/12813 in `message_to_stderr`): on this target `putc` is the stdio
    macro, and the reference object imports `putc_unlocked` instead; the port imports `putc`.
    It links on macOS (libSystem exports both), but it is not C's expansion — for whichever
    lane owns that call site.
52. **Scaffold shape**: nine entries retire and `redisplay_window` (C 1154/20475) is added —
    23 -> 15 entries.
53. `docs/ports/macro-body-notes.tsv`: **this round owes one row** — `set_cursor_from_row` /
    `goto` / `xdisp.c 19077: C's `goto compute_x` is the port's `break 'compute_x` out of the
    `'compute_x` labelled block (a label has no Rust site; deviations 159-160)`.
54. `docs/ports/macro-body-notes.tsv`: **this round owes one row** — `try_scrolling` / `goto` /
    `xdisp.c 19725: C's `goto too_near_end` is the port's `continue 'too_near_end` out of the
    `'too_near_end: loop` that spans the rest of the function (deviations 164-165)`.
55. `include/window.rs`: `window_scroll_margin` (window.h 1226; window.c defines it) is declared
    use-site in `src/xdisp.rs` because the mirror chain does not carry it — the §"Use-site
    declarations" 1 block should gain it (deviation 169).
56. **`include/buffer.rs`: `BUF_BEGV`/`BUF_ZV` — CLOSED, and not a mirror gap.**  Reported by
    this lane as a missing mirror macro, but C's `BUF_BEGV`/`BUF_ZV` are `INLINE` functions
    (buffer.h 917/951) and `include/buffer.rs:923/957` already carries them as `pub fn`; the
    unit's two `macro_rules!` copies were an invented construct and are deleted (integrator
    8958c83; deviation 177).  No mirror work is owed.
57. `include/dispextern.rs`: **CLOSED for both names** (d90609a/3.357).  `row_containing_pos`
    (dispextern.h 3563-3565) and `blank_row` (dispextern.h 3914) now live in the mirror's
    `dispextern_late_decls` module (dispextern.rs 3686-3706, glob-imported at 3706).  The unit
    declares neither, and `row_containing_pos`'s `--check` row (deviation 184) retired with the
    unit's own copy — so the round-25 text's 108 is really 107.  `blank_row` is `dispnew.c`'s
    definition; a unit that defines it needs the module shape, not a local copy.

58. `include/dispextern.rs`: **CLOSED** (d90609a/3.357).  The three
    `DESIRED_MODE_LINE_HEIGHT`/`DESIRED_HEADER_LINE_HEIGHT`/`DESIRED_TAB_LINE_HEIGHT` macros now
    carry the `!` on their inner `MATRIX_*_LINE_HEIGHT` calls (dispextern.rs 2420-2432), and this
    unit's six `redisplay_window` sites (C 21479-21506) read C's own names again — deviation 185
    is superseded.

59. `femacs/src/xdisp.rs`'s `clear_glyph_row` citation: **CLOSED** (d90609a/3.357 fixed it to
    dispextern.h 3916, `clear_glyph_row`'s real line; 3914 is `blank_row`'s).

60. **Scaffold shape**: `try_window`'s declaration retires as its definition lands —
    17 -> 16 entries (deviation 192).  The block's remaining sixteen are other rounds' work.

61. `include/dispextern.rs`: **`rotate_matrix` (dispextern.h 3911) has no mirror home.**  This
    round declares it use-site in `src/xdisp.rs` (deviation 201 / §"Use-site declarations" 12);
    it belongs in the mirror next to `clear_glyph_matrix`/`increment_matrix_positions`/
    `clear_glyph_row`.
62. `docs/ports/macro-body-notes.tsv`: **two rows are owed by this round** —
    `try_window_reusing_current_matrix` / `debug_method_add,if_debug` / `xdisp.c
    21813-21816, 22043-22045, 22232-22234 and 21883: the three `#ifdef GLYPH_DEBUG` blocks and
    the live `IF_DEBUG (debug_method_add (w, "twu1"));` site are recorded, not transcribed
    (`GLYPH_DEBUG` is undef in the frozen macros table), so neither token survives (the
    `redisplay_window` row's class)`, and `row_containing_pos` / `true` / `xdisp.c 22491: C's
    tail `while (true)` is the port's `loop` (Rust's `while true` is not diverging), so the
    `true` token has no site (deviation 197)`.
63. **CLOSED (3.385, the follow-up fix).**  `femacs/src/xdisp.rs` 18899 (C 19892,
    `try_cursor_movement`) read `(*(*w).current_matrix).nrows` — a type error that `eassert!`'s
    empty frozen arm kept out of `build-tu`'s sight.  It is now
    `(*((*w).current_matrix as *mut glyph_matrix)).nrows`, the unit's existing spelling for the
    same C field (six live sites; the identical C 18387 expression at xdisp.rs 17339 is the
    sibling the fix names), with a three-line note at the site giving C 19892.  **The whole-unit
    audit is closed with it**: all 115 `eassert!`/`eassume!`/`clobbered_eassert` statements were
    enumerated and their `…->member` derefs extracted (49 of them carry one); the only site whose
    pointee is a mirror `*mut c_void` was this one, and the two remaining `as *mut` sites
    (`(*XMARKER ((*w).start)).buffer as *mut buffer` and its `pointm` sibling, C 20494-20495) are
    the correct P-72-rule-3 conversion.  Readings: the `--cfg enable_checking` probe goes from
    **exit 1 / `E0609`** to **exit 0 / 0 errors**, with **no error from any other unit or
    mirror** (the mirrors' own `#[cfg (enable_checking)]` blocks compile clean); the frozen
    `build-tu` stays **exit 0** with the same six warnings, and the object keeps its size
    (2,136,144 bytes) while its md5 changes **only** in `__DATA,__const`'s line-number table
    (every differing word is +3, the note's lines): `__text`, `__cstring`, `__const`, `__data`,
    `subrs`, `__compact_unwind` and the symbol table are byte-identical (measured, see the Gate
    evidence section).
64. `include/dispextern.rs`: **the mirror's `row_containing_pos` declaration is now shadowed in
    this unit.**  The unit defines the name with C's own `struct window *` (`*mut window`,
    deviation 198), so the mirror's `*mut c_void` declaration (dispextern.rs 3703) is only there
    for the six units that cannot name `window`; no other unit's call sites change (this unit's
    three did, in this round).
65. `docs/ports/xdisp.md`'s own §"Dead arms" and §"Use-site declarations" are updated in place
    this round (the six `#if` blocks of C 21780-22542 and `rotate_matrix`); the scaffold's
    `row_for_charpos_p` entry retires when its definition lands at C 24975.

66. `docs/ports/macro-body-notes.tsv`: **this round owes one row** — `try_window_id` /
    `debug_delta,debug_delta_bytes,debug_dvpos,debug_dy,debug_end_pos,debug_end_vpos,debug_method_add,if_debug` /
    `xdisp.c 22957-22958, 23076, 23324, 23330, 23367 and 23372-23373: the seven live
    `IF_DEBUG (...)` statements are recorded, not transcribed (`GLYPH_DEBUG` is undef in the
    frozen macros table), and every symbol they name is a GLYPH_DEBUG-only global, so neither
    `if_debug` nor those seven names survives (the `redisplay_window` row's class)`.  With the
    integrator's three existing rows in the table, this is the only `does not mention` line
    `--check` reports for this unit (`--check`'s set grows 3 -> 4 with it).
67. **Scaffold shape**: `try_window_id`'s declaration retires as its definition lands and
    `gui_clear_window_mouse_face` is added — 16 -> 17 entries.  Its `--check` row (`function
    gui_clear_window_mouse_face is not static in C but has no #[unsafe(no_mangle)] in the port
    (no C symbol to link)`) is the class to-do item 48 recorded for
    `mark_window_display_accurate`, and it retires when the C 37705 definition lands.
68. `include/dispextern.rs`: **six use-site declarations this round** belong in the mirror next
    to their siblings — `update_begin` (dispextern.h 3934), `update_end` (3935),
    `set_terminal_window` (3936), `ins_del_lines` (3945), `clear_glyph_matrix_rows` (3915) and
    `shift_glyph_matrix` (3909-3910).  They join items 29/33/34/37's list.
69. **The four `try_window_id`-only helpers now carry machine code.**  `find_last_row_displaying_text`,
    `find_last_unchanged_at_beg_row`, `find_first_unchanged_at_end_row` and
    `sync_frame_with_window_matrix_rows` had no symbol in the object while nothing reachable
    called them (round 26's text-vs-machine gap); with this round's `try_window_id` they are
    emitted (`t __RNv…5xdisp29find_last_row_displaying_text` and its three siblings, measured).
    `try_window_id` itself is `t` (C `static`), and `_gui_clear_window_mouse_face` is `U` until
    its definition lands (item 67).
70. `docs/ports/xdisp.md`'s own §"Dead arms" and §"Use-site declarations" are updated in place
    this round (the five in-range `#if` blocks of C 22545-23381, the dead C 23388-23809 block
    that sits between this round and the next, and the six new use-site names).
71. `include/dispextern.rs`: the round's two use-site externs — `lookup_named_face`
    (dispextern.h 3812) and `compute_char_face` (dispextern.h 3832), both xfaces.c's — belong in
    the mirror next to `lookup_basic_face`/`merge_faces` (item 8's family).  `struct face`'s
    `background` (offset 176) and the three `face_bits!` read arms (`underline` bit 2,
    `overline_p` bit 6, `strike_through_p` bit 7) the round names in this unit's own copy join
    items 10/24/30's list for whoever pushes the record into the mirror.
72. **Text-vs-machine gap (item 69's class), this round's share.**  Seven of the round's eleven
    definitions are unreachable from the transcribed text, so `nm build/femacs/xdisp.o` emits no
    symbol for them: `get_overlay_arrow_glyph_row`, `insert_left_trunc_glyphs`, `clear_position`,
    `append_space_for_newline`, `extend_face_to_end_of_line`, `trailing_whitespace_p` and
    `highlight_trailing_whitespace` (their callers are at C 26761-26815, C 30293 and C 26068-26487
    — all past this unit's transcribed range).  `cursor_row_p`/`row_for_charpos_p`/
    `compute_line_metrics` do get `t` symbols (already-transcribed callers) and `row_hash` a `T`
    one.  The gap closes as the callers land.
73. `docs/ports/macro-body-notes.tsv`: **this round owes one row** (also listed in item 4) —
    `maybe_produce_line_number` / `alloca,int_strlen_bound`, C 25642-25643's
    `char *lnum_buf = alloca (max (it->lnum_width, INT_STRLEN_BOUND (ptrdiff_t)) + 2 + 1)`.
    It is the only new `does not mention` row the round leaves (`--check` 3 -> 4); every other
    token of the round's eleven bodies is transcribed.
74. **One more scaffold-declaration row** (item 67's class): `--check` now also reports
    `function display_count_lines is not static in C but has no #[unsafe(no_mangle)] in the
    port`, because the scaffold declares it (lisp.h 4404-4407) but C 29914's definition has not
    landed; `produce_stretch_glyph` and `gui_clear_window_mouse_face` are the same shape.  The
    row retires in the same commit as the definition.
75. **Text-vs-machine gap (item 69/72's class), this round's share.**  Three of the round's
    eleven definitions are unreachable from the transcribed text, so `nm build/femacs/xdisp.o`
    emits no symbol for them: `find_row_edges`, `unproduce_glyphs` and `row_text_area_empty`
    (their callers are `display_line`, C 25865-26935, and the code past it).  The other eight
    (`push_prefix_prop`, `get_it_property`, `get_line_prefix_it_property`, `handle_line_prefix`,
    `display_count_lines_logically`, `display_count_lines_visually`,
    `maybe_produce_line_number`, `should_produce_line_number`) do get `t` symbols — their
    callers are `handle_line_prefix`/`move_it_in_display_line_to` (C 11778-11856), already
    transcribed.
76. `include/window.rs` / `include/lisp.rs`: the round's five use-site externs —
    `save_restriction_save`/`save_restriction_restore` (lisp.h 4924-4925),
    `labeled_restrictions_remove_in_current_buffer` (lisp.h 4931) and
    `disable_line_numbers_overlay_at_eob` (lisp.h 4937) and `strcat` (`<string.h>`) — belong in
    the mirror chain next to their siblings (the `labeled_narrow_to_region` family, item 29).
77. `docs/ports/macro-body-notes.tsv`: **this round owes two rows** (also listed in item 4 and
    the Progress section) — `display_line` / `goto,uninit` (C 25865-26935's two `goto
    back_to_wrap` and its eight `UNINIT` initialisers) and `RECORD_MAX_MIN_POS` / `false` (C
    26105-26134's in-function `do { … } while (false)` macro, which the port writes as a
    function-scope `macro_rules!`).  They are the round's only new `does not mention` rows
    (`--check` 3 -> 5), and with them appended the reading is 75.  Every other token of the body
    is transcribed.
78. **The burned scaffold entry's citation was wrong** (deviation 83's class): the unit's
    `display_line` prototype said `/// C: dispextern.h 3595: int display_line (struct it *, int);`
    — dispextern.h has no `display_line` (its 3595 line is inside the `move_it_*` batch), C's own
    forward declaration is `static bool display_line (struct it *, int);` at C 1167, and the
    definition's return type is `bool`.  The entry is deleted with the definition, and the five
    in-unit call sites that had been typed against it dropped their `!= 0` (deviation 242).
79. **Text-vs-machine gap (item 69/72/75's class), this round's share.**  `display_line` is called
    from five already-transcribed sites, so `nm build/femacs/xdisp.o` now gives it a local `t`
    symbol; the three helpers that were unreachable before it landed (`find_row_edges`,
    `unproduce_glyphs`, `row_text_area_empty` — item 75) get theirs too, so the round adds four `t`
    symbols and one undefined reference (`U _prepare_desired_row`, item 76's class: the new
    use-site declaration belongs in the mirror chain next to `clear_glyph_matrix_rows`/
    `shift_glyph_matrix`).

80. **Four new use-site declarations** (deviation 276) belong in the mirror chain next:
    `include/keyboard.rs`'s extern block should carry `menu_separator_name_p` (keyboard.h 473),
    `push_kboard` (keyboard.h 480) and `pop_kboard` (keyboard.h 481) — `femacs/src/keyboard.rs`
    is the C unit that defines `push_kboard`/`pop_kboard`, and the mirror does not carry the
    prototypes today; `include/dispextern.rs` should carry `fill_up_frame_row_with_spaces`
    (dispextern.h 3929) next to `clear_glyph_matrix_rows`/`shift_glyph_matrix`, since
    `femacs/src/dispnew.rs` defines it non-`static` with `#[unsafe(no_mangle)]`.  Until then the
    unit declares all four itself and the mirror-dupes gate is at its 17-row ratchet.
81. **The scaffold shrinks 11 -> 8** (measured with
    `awk '/TEMPORARY SCAFFOLD/{f=1} f && /^  fn /{c++} END{print c}' femacs/src/xdisp.rs` = 8):
    `display_menu_bar`, `display_mode_lines` and `display_mode_line` are retired with their
    definitions and none is added — the round's bodies reach `display_string`,
    `display_mode_element`, `display_count_lines`, `pint2str`, `produce_special_glyphs`,
    `calc_pixel_width_or_height`, `produce_stretch_glyph` and `gui_clear_window_mouse_face`,
    which are already in the block (the last two are the two the round does *not* land).
82. **`docs/ports/macro-body-notes.tsv`: nothing owed this round.**  The landed bodies carry no
    `goto`, no `switch` (the two `do { … } while (…)`s of `display_mode_line` and the two `for`s
    of `bidi-resolved-levels` are structural, deviations 270/271), no `va_list` and no `alloca`,
    so `--check`'s `does not mention` set stays 5 for this unit and item 4's list is unchanged.

### Gate evidence for this WIP state

See the final report accompanying this ledger, which quotes every gate's last line.  Every
reading below is from this round's own run, against `femacs/src/xdisp.rs` md5
`25a1231b6aa5e72965c02096b8c3ce02` (25,692 lines = HEAD's 24,435 plus the round's 1,257 net
inserted lines) and the mirrors it includes: `include/dispextern.rs`, `include/window.rs`,
`include/buffer.rs`, `include/lisp.rs` (the last three are other lanes' and mutable).  The
`^- ` baseline is the same file's **HEAD revision** measured with this round's extractor
(`git show HEAD:femacs/src/xdisp.rs` into a scratch path, 77 lines: 44 `missing function`,
5 `missing macro`, 6 `missing subr`, 13 `missing defsubr`, 2 `missing file-scope object`,
3 `does not mention`, 3 `is not static`, 1 `line count drifted`).

* `cd femacs && cargo run -q -p xtask -- build-tu src/xdisp.c` = `built src/xdisp.rs ->
  ../build/femacs/xdisp.o`, **exit 0**, object `build/femacs/xdisp.o` **2,661,656 bytes**, md5
  `2a2e940f0960a293ff33b8c234787b7b` (HEAD's object, for the delta: 2,358,272 bytes, md5
  `75bd6b392b9cb28a75368916a49642ae`).  The build emits **the same six warnings as every previous
  round** (three `unused doc comment`s in mirrors, the two `while true` lint notes in
  `produce_stretch_glyph`/`move_it_to`, one `unused attribute`); this round adds none, and the
  clean build took **the third** of this round's `build-tu` invocations (the brief's budget is
  three; the second's seven errors are in the next bullet).  **Cross-lane note**: the object
  above is the reading at the mirror state this round compiled against; `git status` shows
  `include/dispextern.rs` and `include/frame.rs` modified **after** that run by the mirror lane
  (`femacs/src/include/dispextern.rs` is the header this unit includes at C 482 and carries the
  measured `struct it` the round reads), so a later re-run may read differently — the standing
  "the mirrors are other lanes' and mutable" caveat, and the reason the ledger pins the md5s it
  measured against;
* the first invocation's 33 errors and the second's 7 were diagnosed against the materialised
  `build/femacs/units/xdisp/_root.rs` with `rustc` invocations of the same flags (not `build-tu`
  calls, and the sanctioned `--cfg enable_checking` probe below is one of them): the C `#define`
  text left in the file (`unknown start of token: \`), the `*ptr as T` parenthesisation
  (`E0606`/`E0609`), the `c_int`/`ptrdiff_t` promotions (`E0308`/`E0277`), the buffer member's
  mirror spelling (`E0609 indicate_empty_lines`), `glyph->pixel_width`'s `short` (`E0308`), the
  `XFIXNUM` store into `overlay_arrow_bitmap` (`E0308`) and the five `display_line` call sites
  the scaffold's `c_int` typing had given `!= 0` (`E0308`);
* `python3 tools/extract-abi.py --check emacs/src/xdisp.c femacs/src/xdisp.rs` = `FAIL`; its last
  two lines are `- line count drifted 35.5%: C 39805 vs Rust 25692` and
  `FAIL emacs/src/xdisp.c -> femacs/src/xdisp.rs`.  The `^- ` count is **77 -> 77**: the round
  retires `missing function display_line` (44 -> 43) and `missing macro RECORD_MAX_MIN_POS`
  (5 -> 4) and adds the two `does not mention` rows the Progress section names
  (`does not mention` 3 -> 5, of which one is the `macro … does not mention false` family).
  `missing subr` 6, `missing defsubr` 13, `missing file-scope object` 2 and `is not static` 3
  are unchanged, and there is still no `stub or empty body` line.  With the two owed
  `macro-body-notes.tsv` rows appended the `does not mention` set returns to 3 and the reading
  drops to **75**;
* `python3 tools/check-citations.py` = `675 file(s) scanned, 0 drift, 0 range, 499 external,
  33 allowed, 0 stale`; **`femacs/src/xdisp.rs` has no finding** (this round adds one `/// C:`
  line for the new use-site declaration);
* `python3 tools/check-declarations.py` = `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`; its final line is `walked past 0 file(s) of an unscanned kind: none
  (1 of them read above)`.  No finding from this unit;
* `python3 tools/check-carried.py` = `11 carried file(s), 11 baseline row(s), 0 failure(s)`;
* `python3 tools/check-dead-file-coverage.py` = `17 ledger(s) bound (a C range table), 2
  failure(s)`; both failures name `docs/ports/android.md` (`4 reference line(s) with text are not
  in any range (141, 386, 464, 669)` and `7 reference line(s) with text are not in any range (21,
  141, 272, 309, 313, 334, 340)`), which is another lane's in-flight ledger — `git status` shows
  `docs/ports/android.md`, `femacs/src/android.rs` and `femacs/src/image.rs` modified but
  uncommitted, and this unit touches none of them.  This unit is not a P-86 file; the gate is run
  because the brief's reading list names it;
* `python3 tools/check-ledger-ticks.py` = `235 ledger(s) bound (a function tick to compare), 0
  failure(s), 0 carried by another file, 355 row(s) with no function tick - a unit with no
  function at all, or one whose body the frozen configuration leaves under a false condition
  (check-port-lines.py is that class's gate), 0 row(s) without a ledger` — the round's one new
  tick (`display_line`) is carried.

**Symbol-table reading** (`nm`, run by hand): `build/femacs/xdisp.o` gains four `t` symbols this
round — `display_line` itself (five transcribed callers: C 17699, 21708, 21927, 22110, 23031,
23306's sites) and the three helpers that were unreachable before it landed, `find_row_edges`,
`unproduce_glyphs` and `row_text_area_empty` (item 75's gap closes for those three) — and one new
undefined reference, `U _prepare_desired_row` (the round's new use-site declaration).

**The `enable_checking` probe** (not one of the four gates): `cd femacs && rustup run
nightly-2026-10-01 rustc --edition 2024 --crate-type=lib -C panic=abort -C codegen-units=1
-C opt-level=0 --cfg enable_checking --crate-name xdisp --emit=metadata -o
/tmp/dline-port/probe/xdisp.rmeta ../build/femacs/units/xdisp/_root.rs` = **exit 0, 0 errors**,
the same six warnings.  It matters here because `build-tu` does **not** compile the
`#[cfg (enable_checking)]` arm (`xtask`'s single-unit `build_tu` runs one rustc pass), so this
round's five `eassert!`s (deviation 244) were checked only by this probe.  The probe is run from
`femacs/` with `rustup run nightly-2026-10-01`, over the materialised `_root.rs` `build-tu` has
just written and with the same flags `tools/check-all-units.sh`'s checking leg uses (with
`--emit=metadata` instead of `--emit=obj`, into a scratch directory, so no `.rlib` is left
behind); a bare `rustc` from the repository root falls through to stable and reports spurious
`E0658 c_variadic` errors.

`xtask layout-check` was **not** re-run this round: the unit adds no `struct` and no new layout
family — the round's only type-level work is the `it_bits!`/`it_bits3_*` accessors over the
already-measured 96-bit `bits3` run (`struct it`'s byte 2604, whose `const _` assertions are
unchanged) — and the task's reading list is `build-tu` plus the text gates above.

**This round (C 26937-27028).**  `femacs/src/xdisp.rs` md5 `7d0539d6438f7806f23a006d2c994cc4`
(25,779 lines; HEAD's revision is 25,692 lines, md5 `25a1231b6aa5e72965c02096b8c3ce02`).

* `cd femacs && CARGO_TARGET_DIR=/tmp/xdisp32 cargo run -q -p xtask -- build-tu src/xdisp.c` =
  `built src/xdisp.rs -> ../build/femacs/xdisp.o`, **exit 0**, one invocation, object
  **2,674,432 bytes**, md5 `5dd3b2cc6f03149295d5d8083afcc034` (HEAD's object: 2,661,656 bytes, md5
  `2a2e940f0960a293ff33b8c234787b7b`), with the same six warnings as every previous round and
  none added.  `nm build/femacs/xdisp.o` reads `T _Fcurrent_bidi_paragraph_direction` and
  `s __RNvCs3P4vcM1aRw9_5xdisp33Scurrent_bidi_paragraph_direction`.
* `python3 tools/extract-abi.py --check emacs/src/xdisp.c femacs/src/xdisp.rs` = **71 `- ` lines**
  (the round's HEAD reading is 72 with this extractor: it retires `missing subr
  Fcurrent_bidi_paragraph_direction` and `missing defsubr (&raw const
  Scurrent_bidi_paragraph_direction)`, and adds `subr Fcurrent_bidi_paragraph_direction does not
  mention case, default, switch`, which the row above answers).  No `does not mention` line
  remains; the rest are the unported remainder and the three scaffold declarations.
* `python3 tools/check-ledger-ticks.py` binds the new subr's tick; `check-dead-file-coverage.py`
  and `check-declarations.py` are unchanged (this unit is not a P-86 range-table file and adds no
  product-name site).
* `xtask layout-check` was not re-run: the round adds no `struct`, no field and no layout family
  (it uses `bidi_it` as bidi.rs declares it).
`bidi-find-overridden-directionality` (C 27030-27179) was added in the same round, after the
readings above: `build-tu` again exit 0 (one invocation, the same six warnings), object
**2,684,688 bytes**, md5 `7aaf0b43ac76352217bee6bbdd799c9e`; `nm` reads
`T _Fbidi_find_overridden_directionality` and its `Sbidi_find_overridden_directionality` static;
`extract-abi --check` **70 `- ` lines** (the round's HEAD baseline is 72) with no `does not
mention` line and `line count drifted 34.9%`; `femacs/src/xdisp.rs` is now 25,909 lines.

**One correction after the commit (3.422).**  The first version declared `validate_subarray`
locally and `check-mirror-dupes` reported `FAIL femacs/src/xdisp.rs: validate_subarray is already
declared by femacs/src/include/lisp.rs (in the include graph)` - the lisp.rs mirror carried it all
along (3719-3721), so the local copy is gone and a comment records why; the gate is back to
`19 duplicate declaration(s), 19 ratchet row(s), 0 failure(s)`.  The re-build after that is exit 0
with the same six warnings and object md5 `e8f95d661dc6f5033c3543544f3bbe8b`; `extract-abi --check`
stays at 70 with no `does not mention`.

**This round (C 27181-27636).**  `femacs/src/xdisp.rs` md5 `e8d54bdc37c401f09759e8e6932b1cc9`
(26,406 lines; the round's HEAD revision is 25,908 lines, md5 `70de58ef2241dd900f9971e2cb86978d`).
**Cross-lane note**: while this round was in flight another lane rewrote the unit's own
`const _: () = assert! (…)` layout assertions to `core::assert! (…)` (the eight `struct`s at lines
953-1975, 157 lines), moving the unit to md5 `74469b8dd224c0d13083ab080285f5a4` without touching
this round's text; this round's own last edit (three C-line citations inside its comments) then
moved it to the md5 above.  Both re-runs read the **identical** object (same size, same md5 - the
assertions are compile-time only and comments are not code).

* `cd femacs && CARGO_TARGET_DIR=/tmp/xdisp33-lane/cargo cargo run -q -p xtask -- build-tu
  src/xdisp.c` = `built src/xdisp.rs -> ../build/femacs/xdisp.o`, **exit 0**, with the same six
  warnings as every previous round and none added; the clean build took **the second** of this
  round's **four** `build-tu` invocations (the brief's budget is three, so this round is **one
  over** and says so: the third re-ran after the cross-lane rewrite above and the fourth after
  this round's own comment-citation fix, and all three clean builds read the same object).  The first invocation's
  **56** errors are all one class - `E0614 type `it` cannot be dereferenced`, the 56
  `(*it).FIELD` reads of the transcribed body, where C's `struct it it` local is a Rust *value*
  and its members are `it.FIELD` (the `pos_visible_p` local-`it` convention).  Object
  `build/femacs/xdisp.o` **2,738,320 bytes**, md5 `64f75ce3d5b57ac105b506dbe79e80da` (HEAD's
  object, for the delta: 2,684,688 bytes, md5 `7aaf0b43ac76352217bee6bbdd799c9e`).
* `nm build/femacs/xdisp.o | grep -i move_point_visually` reads
  `0000000000004104 T _Fmove_point_visually` and
  `000000000016d7e8 s __RNvCs3P4vcM1aRw9_5xdisp20Smove_point_visually` (the subr record's own
  static).  No new undefined symbol: every name the body reaches was already defined in the
  unit, in the include graph, or carried by one of the eleven `TEMPORARY SCAFFOLD` entries the
  round leaves untouched.
* `python3 tools/extract-abi.py --check emacs/src/xdisp.c femacs/src/xdisp.rs` = **69 `- `
  lines** (the HEAD revision measured with this extractor is **70**): it retires `missing subr
  Fmove_point_visually` and `missing macro ROW_GLYPH_NEWLINE_P` and adds the round's one
  `does not mention` row, `subr Fmove_point_visually does not mention goto` (deviation 265;
  the owed `macro-body-notes.tsv` row clears it).  **`missing defsubr (&raw const
  Smove_point_visually)` stays**, and so do the other twelve: C's
  `defsubr (&Smove_point_visually)` is `syms_of_xdisp`'s (C 38469), outside the covered range,
  and the port carries no `defsubr` call site at all yet.  `line count drifted 33.7%: C 39805
  vs Rust 26406`.  With the owed row appended the reading drops to **68** and the
  `does not mention` set returns to 0.
* `python3 tools/check-ledger-ticks.py` = `235 ledger(s) bound (a function tick to compare), 0
  failure(s), …, 355 row(s) with no function tick …, 0 row(s) without a ledger` (the "carried by
  another file" count moved 1 -> 0 between this round's two runs as another lane's commit
  landed; 0 failure(s) both times).  The round's `move-point-visually` tick sits under `## Subrs (DEFUN)` and the
  `ROW_GLYPH_NEWLINE_P` tick under `## Macros defined` - headings this gate does not bind (it
  reads `## Functions defined`-style sections), so its reading is unchanged and 0 failures.
* `python3 tools/check-declarations.py` = `14 declaration sites (0 forbidden, 14 allowed
  literals), 0 failures`; no finding from this unit (the round adds no `/// C:` declaration).
* `python3 tools/check-dead-file-coverage.py` = `23 ledger(s) bound (a C range table), 0
  failure(s)` when this round measured it; the end-of-round re-run read `25 ledger(s) bound …, 2
  failure(s)`, both naming `docs/ports/treesit.md` (another lane's in-flight ledger - the same
  file `git status` shows modified alongside its unit) and neither naming this unit; that lane's
  next commit landed before this round closed and the gate reads `25 ledger(s) bound …, 0
  failure(s)` again.  This unit is not a P-86 range-table file (its ledger carries no
  range-table heading).
* `xtask layout-check` was **not** re-run: the round adds no `struct`, no field and no layout
  family (it uses `struct it`, `struct text_pos`, `glyph` and `glyph_row` as the mirrors
  declare them).


## Evidence (this round: C 27637-28436; all commands from `cwd=/Users/liu/projects/src/github.com/liuchong/femacs`)

**Before the round.**  The working tree's `femacs/src/xdisp.rs` was `HEAD`'s
(`git show HEAD:femacs/src/xdisp.rs | md5` = `e8d54bdc37c401f09759e8e6932b1cc9`, 26,406 lines).

* `python3 tools/extract-abi.py --check emacs/src/xdisp.c femacs/src/xdisp.rs` (saved to
  `/tmp`): **68** `- ` lines, last line `- line count drifted 33.7%: C 39805 vs Rust 26406`,
  `FAIL emacs/src/xdisp.c -> femacs/src/xdisp.rs`.
* `python3 tools/check-cond-eval.py src/xdisp.c`: `196 conditions, 0 unknown, 0 disagreements
  [VERIFIED]` — the ledger's `#if` verdicts stand, so the round transcribed the live arms only.

**After the round.**

* `python3 tools/extract-abi.py --check emacs/src/xdisp.c femacs/src/xdisp.rs`: **58** `- `
  lines (`68 -> 58`; the ten retired are the `missing subr Fbidi_resolved_levels` row and the
  nine `missing function` rows of the nine functions this round defines), last line
  `- line count drifted 31.8%: C 39805 vs Rust 27160` (`FAIL` stays: the remainder of C
  30702-39805 is not transcribed, so the 5% tolerance cannot close yet).
* `femacs/src/xdisp.rs`: 27,160 lines, md5 `5486db2473f4b33139a244cadeab2d2a`
  (`26,406 -> 27,160`, +754).
* `docs/ports/xdisp.md`: 43 unchecked boxes (`66 -> 43`: 10 ticked for this round's own
  definitions, 13 stale boxes ticked after checking the artifact — see the section above — and
  1 box added for `invalid_eol_type`, which the tool's 61-object list requires and the ledger's
  60-row list had missed).
* `cd femacs && CARGO_TARGET_DIR=/tmp/xdisp-lane/target cargo run -q -p xtask -- build-tu
  src/xdisp.c` — three runs, the budget's cap: (1) `error[E0609] no field
  header_line_format` + 3 `error[E0384]` (the mirror's `buffer` fields carry C's `BVAR`
  trailing `_`; `width` needs `mut`; the inner `display` is reassigned) -> fixed in the text;
  (2) **exit 0**, `built src/xdisp.rs -> ../build/femacs/xdisp.o` (6 warnings, all pre-existing
  style ones); (3) `error[E0608] cannot index into a value of type *mut *mut Lisp_X` (the
  `args[0]` spelling of `safe_set_text_properties`) -> fixed to `*args.offset (0)`.
* `cd femacs && CARGO_TARGET_DIR=/tmp/xdisp-lane/target cargo run -q -p xtask -- probe
  src/xdisp.rs /tmp/xdisp-lane/x.rlib --crate-type=lib -C panic=abort` (the budget's first
  probe run, after the third fix): **exit 0**, `/tmp/xdisp-lane/x.rlib` written
  (22,263,512 bytes), 6 warnings and no errors — so the tree as handed off compiles.
* `/opt/homebrew/opt/llvm/bin/llvm-nm /tmp/xdisp-lane/x.rlib | grep …`: `T _Fbidi_resolved_levels`
  (the `DEFUN!`'s C symbol), `T _display_tty_menu_item` (C's non-`static` definition), and
  `t __RNv…_5__root16display_menu_bar` / `t __RNv…_5__root17display_mode_line` (C's `static`
  helpers keep local Rust symbols and no C symbol, PORTING.md 0.2/P-52); the four new
  declarations are `U _menu_separator_name_p`, `U _push_kboard`, `U _pop_kboard`,
  `U _fill_up_frame_row_with_spaces` (item 80's mirror gap).
* `python3 tools/check-ledger-ticks.py`: `238 ledger(s) bound …, 0 failure(s), 0 carried by
  another file, 352 row(s) with no function tick …, 0 row(s) without a ledger` — the round's 23
  ticks (10 + 13) all bind to definitions.
* `python3 tools/check-declarations.py`: `17 declaration sites (0 forbidden, 17 allowed
  literals), 0 failures`; `read 683 files …`, `read 46 own prose/manifest file(s) …`.
* `python3 tools/check-citations.py`: `676 file(s) scanned, 0 drift, 0 range, 1814 external,
  36 allowed, 0 stale` (the round's four new `/// C:` lines are among the `external` set).
* `python3 tools/check-mirror-dupes.py`: `17 duplicate declaration(s), 17 ratchet row(s),
  0 failure(s)` — unchanged (the four new declarations are not in any mirror yet, item 80).
* `python3 tools/check-dead-file-coverage.py`: `143 ledger(s) bound (a C range table), 0
  failure(s)` — this unit's ledger is a checkbox ledger, not a range table.
* `python3 tools/check-cond-eval.py src/xdisp.c`: unchanged (`196 conditions, 0 unknown,
  0 disagreements [VERIFIED]`).
* Machine artifacts (`build/femacs/units/xdisp`, `build/probes/xdisp`, `build/femacs/xdisp.o`)
  were deleted after each of the four compile runs (measured: `ls build/femacs | grep -c xdisp`
  = 0, `ls build/probes` = `w32term xterm`, neither this unit's).

## Owed

The 43 unchecked boxes, in C order (and the four rows that are not boxes).

**Functions (34).**

| C | box | why not yet |
| --- | --- | --- |
| 15616 | `tty_handle_tab_bar_click` | the earlier hole inside the covered range (C 15616-16624, ~1008 C lines, the TTY tab-bar click handler); `--check` reports it |
| 28463-28976 | `display_mode_element` | the next live definition; the round's largest single item (514 C lines, the `tail_recurse:`/`invalid:` labels and the `:eval`/`:propertize` arms) |
| 28977-29062 | `store_mode_line_string` | after it, in C order |
| 29193-29238 | `pint2str` | `maybe_produce_line_number`'s already-transcribed caller reaches it through the scaffold |
| 29239-29338 | `pint2hrstr` | |
| 29339-29416 | `decode_mode_spec_coding` | `invalid_eol_type` (C 29336) is its neighbour |
| 29417-29433 | `percent99` | |
| 29434-29896 | `decode_mode_spec` | 463 C lines |
| 29897-29913 | `count_lines` | |
| 29914-30061 | `display_count_lines` | C's is non-`static`; the port must carry `#[unsafe(no_mangle)]` (`--check`'s third row class) |
| 30062-30313 | `display_string` | 252 C lines, reaches `invisible_prop` |
| 30314-30348 | `invisible_prop` | |
| 30470-30701 | `calc_pixel_width_or_height` | carries `OK_PIXELS`/`OK_ALIGN_TO`/`LNUM_ONCE` (C 30481-30485) and `lots_of_dashes` table use |
| 30702-32971 | `get_font_ascent_descent` | |
| 32972 | `produce_stretch_glyph` | C's is non-`static` (the `--check` row) |
| 33192 | `produce_special_glyphs` | in the scaffold; `display_line` and `PRODUCE_GLYPHS` already call it |
| 33406 | `pad_mode_line` | |
| 34731 | `get_specified_cursor_type` | |
| 34779 | `set_frame_cursor_types` | |
| 35477 | `draw_row_with_mouse_face` | |
| 35497 | `show_mouse_face` | |
| 35662 | `clear_mouse_face` | |
| 35678 | `coords_in_mouse_face_p` | |
| 35727 | `cursor_in_mouse_face_p` | |
| 35759 | `rows_from_pos_range` | |
| 35915 | `mouse_face_from_buffer_pos` | |
| 36327 | `mouse_face_from_string_pos` | |
| 36586 | `define_frame_cursor1` | |
| 36633 | `note_line_or_margin_highlight` | |
| 37056 | `note_mouse_highlight` | |
| 37706 | `gui_clear_window_mouse_face` | C's is non-`static` (the `--check` row); in the scaffold |
| 37724 | `cancel_mouse_face` | |
| 38410-39541 | `syms_of_xdisp` | the single item that retires the **eight** `missing defsubr` rows for subrs already defined (`Sbidi_resolved_levels`, `Sbidi_find_overridden_directionality`, `Sbuffer_text_pixel_size`, `Scurrent_bidi_paragraph_direction`, `Sdisplay__line_is_continued_p`, `Sget_display_property`, `Sline_pixel_height`, `Slong_line_optimizations_p`, `Smove_point_visually`, `Swindow_text_pixel_size`, `Sset_buffer_redisplay` — eleven rows, of which three wait on their `DEFUN`) plus the five of the two missing subrs |
| 39542-39805 | `init_xdisp` | the unit's last definition |

**Subrs (5).**  `set-buffer-redisplay` (C 922), `line-pixel-height` (C 1545) and
`get-display-property` (C 5877) have their `F…` definitions already and await `syms_of_xdisp`'s
`defsubr`; `format-mode-line` (C 29063, `Fformat_mode_line`) and `invisible-p` (C 30349,
`Finvisible_p`) have no definition yet — `--check`'s two `missing subr` rows.

**Macros (3).**  `OK_PIXELS` (C 30481, body `1d72993912be`), `OK_ALIGN_TO` (C 30482, body
`fe44bbad711c`) and `LNUM_ONCE` (C 30485, body `a34f80a0f2f1`) — all three are in-function
machinery of `calc_pixel_width_or_height`, so they land with it.  The round owes **no**
`macro-body-notes.tsv` row (item 82).

**File-scope objects (2).**  `invalid_eol_type` (C 29336, the box added this round) and
`lots_of_dashes` (C 29431) — both are `decode_mode_spec`'s neighbours.

**Rows that are not boxes.**  The unit's line count must reach **37,815** lines (C 39,805 minus
the 5% tolerance, `tools/extract-abi.py`'s `line count drifted` row): at 27,160 today that is
**10,655 lines** away, which is the rest of C 15616-16624 and C 28463-39805 minus their dead
arms; and the eleven `missing defsubr` rows named under `syms_of_xdisp` are the same item seen
from the other side.
