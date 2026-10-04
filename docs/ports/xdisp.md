# emacs/src/xdisp.c (39805 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 13
- [ ] set-buffer-redisplay  (Fset_buffer_redisplay, 4/4, doc 5c281f24209d, line 922)
- [ ] line-pixel-height  (Fline_pixel_height, 0/0, doc a1c4fe180f27, line 1545)
- [ ] get-display-property  (Fget_display_property, 2/4, doc ebee6e28fe94, line 5877)
- [x] window-text-pixel-size  (Fwindow_text_pixel_size, 0/7, doc 24dfcb0ed7db, line 12078)
- [x] buffer-text-pixel-size  (Fbuffer_text_pixel_size, 0/6, doc 8ad2cd85d9f9, line 12158)
- [x] display--line-is-continued-p  (Fdisplay__line_is_continued_p, 0/0, doc 198c6180dfb2, line 12216)
- [ ] long-line-optimizations-p  (Flong_line_optimizations_p, 0/0, doc 9778916a82c2, line 20414)
- [ ] current-bidi-paragraph-direction  (Fcurrent_bidi_paragraph_direction, 0/1, doc 2adae0649377, line 26937)
- [ ] bidi-find-overridden-directionality  (Fbidi_find_overridden_directionality, 3/4, doc 6e06604b8c32, line 27030)
- [ ] move-point-visually  (Fmove_point_visually, 1/1, doc 3a8d3658283b, line 27181)
- [ ] bidi-resolved-levels  (Fbidi_resolved_levels, 0/1, doc e680f6eaa573, line 27637)
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
- [ ] handle_fontified_prop  (line 4667, static)
- [ ] face_at_pos  (line 4799, static)
- [ ] handle_face_prop  (line 4910, static)
- [ ] underlying_face_id  (line 4969, static)
- [ ] face_before_or_after_it_pos  (line 4989, static)
- [ ] handle_invisible_prop  (line 5206, static)
- [ ] setup_for_ellipsis  (line 5654, static)
- [ ] find_display_property  (line 5696, static)
- [ ] get_display_property  (line 5761, static)
- [ ] display_min_width  (line 5773, static)
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
- [ ] text_outside_line_unchanged_p  (line 17006, static)
- [ ] redisplay  (line 17073)
- [ ] overlay_arrow_string_or_property  (line 17080, static)
- [ ] overlay_arrow_in_current_buffer_p  (line 17092, static)
- [ ] overlay_arrows_changed_p  (line 17120, static)
- [ ] update_overlay_arrows  (line 17162, static)
- [ ] overlay_arrow_at_row  (line 17199, static)
- [ ] check_point_in_composition  (line 17255, static)
- [ ] reconsider_clip_changes  (line 17290, static)
- [ ] propagate_buffer_redisplay  (line 17318, static)
- [ ] redisplay_internal  (line 17351, static)
- [ ] unwind_redisplay_preserve_echo_area  (line 18238, static)
- [ ] redisplay_preserve_echo_area  (line 18255)
- [ ] unwind_redisplay  (line 18284, static)
- [ ] unwind_display_working_on_window  (line 18293)
- [ ] mark_window_display_accurate_1  (line 18304, static)
- [ ] mark_window_display_accurate  (line 18407)
- [ ] disp_char_vector  (line 18436)
- [ ] block_buffer_flips  (line 18461, static)
- [ ] unblock_buffer_flips  (line 18468, static)
- [ ] buffer_flipping_blocked_p  (line 18486)
- [ ] redisplay_windows  (line 18499, static)
- [ ] redisplay_window_error  (line 18522, static)
- [ ] redisplay_window_0  (line 18540, static)
- [ ] redisplay_window_1  (line 18548, static)
- [ ] update_redisplay_ticks  (line 18574)
- [ ] set_cursor_from_row  (line 18631, static)
- [ ] run_window_scroll_functions  (line 19264, static)
- [ ] cursor_row_fully_visible_p  (line 19305, static)
- [ ] try_scrolling  (line 19402, static)
- [ ] compute_window_start_on_continuation_line  (line 19745, static)
- [ ] try_cursor_movement  (line 19865, static)
- [ ] set_vertical_scroll_bar  (line 20268)
- [ ] set_horizontal_scroll_bar  (line 20325)
- [ ] window_start_acceptable_p  (line 20389, static)
- [ ] redisplay_window  (line 20475, static)
- [ ] try_window  (line 21684)
- [ ] try_window_reusing_current_matrix  (line 21793, static)
- [ ] find_last_row_displaying_text  (line 22261, static)
- [ ] find_last_unchanged_at_beg_row  (line 22295, static)
- [ ] find_first_unchanged_at_end_row  (line 22352, static)
- [ ] sync_frame_with_window_matrix_rows  (line 22428, static)
- [ ] row_containing_pos  (line 22472)
- [ ] try_window_id  (line 22588, static)
- [ ] get_overlay_arrow_glyph_row  (line 23822, static)
- [ ] insert_left_trunc_glyphs  (line 23893, static)
- [ ] row_hash  (line 24048)
- [ ] compute_line_metrics  (line 24082, static)
- [ ] clear_position  (line 24157, static)
- [ ] append_space_for_newline  (line 24176, static)
- [ ] extend_face_to_end_of_line  (line 24384, static)
- [ ] trailing_whitespace_p  (line 24869, static)
- [ ] highlight_trailing_whitespace  (line 24891, static)
- [ ] row_for_charpos_p  (line 24975, static)
- [ ] cursor_row_p  (line 25062, static)
- [ ] push_prefix_prop  (line 25077, static)
- [ ] get_it_property  (line 25175, static)
- [ ] get_line_prefix_it_property  (line 25196, static)
- [ ] handle_line_prefix  (line 25217, static)
- [ ] unproduce_glyphs  (line 25258, static)
- [ ] find_row_edges  (line 25278, static)
- [ ] display_count_lines_logically  (line 25431, static)
- [ ] display_count_lines_visually  (line 25450, static)
- [ ] maybe_produce_line_number  (line 25499, static)
- [ ] should_produce_line_number  (line 25800, static)
- [ ] row_text_area_empty  (line 25835, static)
- [ ] display_line  (line 25865, static)
- [ ] display_menu_bar  (line 27761, static)
- [ ] deep_copy_glyph_row  (line 27924, static)
- [ ] display_tty_menu_separator_char  (line 27954, static)
- [ ] display_tty_menu_separator  (line 27973, static)
- [ ] display_tty_menu_item  (line 28008)
- [ ] display_mode_lines  (line 28112, static)
- [ ] display_mode_line  (line 28205, static)
- [ ] move_elt_to_front  (line 28394, static)
- [ ] safe_set_text_properties  (line 28432, static)
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
- [ ] AINC  (line 17604, body 27cb0e150c56)
- [ ] BASE_LINE_NUMBER_VALID_P  (line 19396, body 89582bb92958)
- [x] BUFFER_POS_REACHED_P  (line 10165, body 27a2bd2c9b82)
- [x] CHAR_COMPOSED_P  (line 8297, body 9fd5f9641609)
- [x] CLEAR_FACE_CACHE_COUNT  (line 1054, body f83a383c0fa8)
- [x] DEFAULT_HOURGLASS_DELAY  (line 1115, body 356a192b7913)
- [x] DISP_INFINITY  (line 508, body 9dfdd483be2b)
- [x] FRAME_OUTPUT_DATA  (line 505, body 48a28489fbe0)
- [x] GET_NEXT_DISPLAY_ELEMENT  (line 8286, body 174c658220fb)
- [ ] GIVE_UP  (line 22628, body 06d3d9b2060d)
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
- [ ] RECORD_MAX_MIN_POS  (line 26024, body 9ee0416c4092)
- [x] RECORD_OVERLAY_STRING  (line 7121, body 16715b981720)
- [x] REDISPLAY_SOME_P  (line 14235, body c6fb49f84210)
- [x] RESTORE_IT  (line 848, body 4985d364c618)
- [ ] RESUME_POLLING  (line 17343, body 2a5d0cb2a6a4)
- [ ] ROW_GLYPH_NEWLINE_P  (line 27196, body 9d6d5b96f828)
- [x] SAFE_CALLMANY  (line 3155, body 67a378c1bce8)
- [x] SAVE_IT  (line 840, body cf8ea527efc2)
- [ ] SCROLL_LIMIT  (line 19391, body 310b86e0b62b)
- [x] SET_WITH_NARROWED_BEGV  (line 3829, body 53ab6f455be2)
- [ ] STOP_POLLING  (line 17339, body c86b380dd52a)
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
- [ ] static int buffer_flip_blocked_depth  (line 18458, static)
- [ ] static char lots_of_dashes  (line 29431, static)

# ----------------------------------------------------------------------
## Decisions

### Progress (this lane is incremental)

`src/xdisp.c` is 39,805 lines and is being transcribed in C order.  **This round adds C
15584-16929** — but **most of that range is one more dead arm**: C 15662-16610 is the whole
Tool-bars section, which sits inside `#ifdef HAVE_WINDOW_SYSTEM` (the frozen configuration has
no window system), so the round transcribes only its three live definitions —
`tty_get_tab_bar_item`, `hscroll_window_tree` (with the `hscroll_step_abs`/`hscroll_step_rel`
margin arithmetic, the auto-hscroll suspension, the `MATRIX_BOTTOM_TEXT_ROW`/`MATRIX_ROW` row
choice, the `init_to_row_start` + infinite-width `move_it_in_display_line_to` scan and the
`hscroll` clamp) and `hscroll_windows`.  `tty_handle_tab_bar_click` (C 15615-15654) is **not**
defined here: `include/termchar.rs` already carries termchar.h 238-239's `extern "C"`
declaration of that name, and a Rust definition beside it is E0428 (the
`set_buffer_internal_1` family — the ledger's integrator item 6); the box stays unticked and
the gap is reported (item 38).  The covered range is now **C 468-16929** (the earlier rounds'
C 468-4323, and the C 4326-4641 / C 4667-4980 / C 4983-5647 / C 5650-5896 / C 5897-6762 /
C 6763-7562 / C 7573-8430 / C 8437-9246 / C 9258-10060 / C 10063-10877 / C 10879-11645 /
C 11647-12525 / C 12526-13255 / C 13256-13994 / C 13995-14783 rounds).  C 17006's
`text_outside_line_unchanged_p` is the next live definition, reached after the dead
`#ifdef GLYPH_DEBUG` block (C 16940-16997, `debug_method_add`).  Ticked in the ledger above, bucket by bucket — **measured** with
`grep -c '^- \[x\]'` over each section (`Subrs` 3/13, `Functions` 182/302, `Macros` 27/38,
`Types and their fields` 2/2, `File-scope objects` 58/60) — **272 of the 415 boxes**
(182+3+27+2+58); the remaining 143 are untouched.  The three live subrs
`set-buffer-redisplay`, `line-pixel-height` and `get-display-property` keep their `Subrs`
boxes unticked as before, so the `Subrs` trio is the thirteen-round-earlier one.  (Earlier
drafts of this sentence listed only the function total plus three round-specific increments
and still claimed 272, an addition that does not close; the bucket list above is the one that
does.)
**No scaffold entry is burned or added this round** — **measured**: the block stays at thirteen
`fn` entries.  `include/dispextern.rs` now **has** `pub struct it` (C 2815, with
`len: c_int`@2232 as the ledger's item 31 required) — but the record is inside
`#[cfg (feature = "dispextern_face_records")]`, and `xtask build-tu` passes **no `--cfg`
flags at all**, so that record is invisible to every unit build and this unit's own measured
copy is still what compiles (verified by `rg` + a clean build; the finding is reported as
item 39).  One new use-site extern is declared (`clear_desired_matrices`, dispextern.h 3908).
**This round owes one `docs/ports/macro-body-notes.tsv` row** —
`hscroll_window_tree` / `double` (C 16630's `double hscroll_step_rel`).

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

### The transcription that is in place

The ledger above is the checklist.  182 of the C file's 302 functions (plus 3 of its 13
subrs — the three the frozen configuration keeps live and this round's three) are defined, in
C's order, with C's names, C's statements and
C's comments; `src/xdisp.rs` is 16,194 lines.  Every function that C declares `static` is a
plain `unsafe fn` here (and `unsafe extern "C" fn` where it is variadic or its address is
handed out — `redisplay_trace`, `move_trace`, `dsafe_eval_handler`,
`funcall_with_backtraces`, `handle_display_prop` (whose address is `it_props[]`'s handler
field) and `unwind_narrowed_begv`, whose address `SET_WITH_NARROWED_BEGV` hands to
`record_unwind_protect`); every non-`static` definition carries
`#[unsafe(no_mangle)] pub unsafe extern "C"` (PORTING.md 0.2/P-52).

The 24 `DEFUN`s of C are the unit's subrs; the three that are live on the frozen target
(`set-buffer-redisplay`, `line-pixel-height`, `get-display-property`) use `DEFUN!`
(PORTING.md 5.1).

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

### Use-site declarations (headers without a mirror, or mirror gaps; each keeps C's prototype in a `/// C:` line)

1. **`window.h`'s accessor macros.**  `include/window.rs` is WIP: it carries the record, the
   nineteen packed-flag shifts, the read-only accessors, `wset_redisplay`/`fset_redisplay`/
   `bset_redisplay`'s declarations (module + glob import) and the fringe/margin readers, but
   not `WINDOW_XFRAME`/`WINDOW_FRAME`, the divider widths, the scroll-bar geometry,
   `WINDOW_MODE_LINE_HEIGHT`/`_HEADER_LINE_HEIGHT`/`_TAB_LINE_HEIGHT` or the
   `CURRENT_*_LINE_HEIGHT` family.  Those are defined here under C's own name (PORTING.md
   0.1-2/P-72 rule 3), the `window.rs` copy verbatim where the text is the same.  The
   `window.h` prototypes whose *definitions* live in window.c join them: `window_body_width`
   and `window_body_height` (window.h 1223-1224), declared in the unit's `unsafe extern "C"`
   block; frame.h 1191's l-value `FRAME_WINDOW_CHANGE` is the two-argument macro next to
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
   `SAVE_IT if (CACHE)` row is in the table today), and the fifth is this round's:

   ```
   redisplay_trace	va_list,va_start,va_end	P-04: C's `va_list ap; va_start (ap, fmt); vfprintf (stderr, fmt, ap); va_end (ap);` is the port's `...` parameter bound once as `VaList` (the dump_trace row's shape)
   move_trace	va_list,va_start,va_end	same as redisplay_trace
   SAVE_IT	false	C's `do { … } while (false)` has no Rust counterpart (PORTING.md 2.2, the CHECK_CODING_SYSTEM/SAFE_ALLOCA_LISP class): the port's block runs the guard once and never mentions `false`
   SAVE_IT	if (CACHE)	C's `void *CACHE` tested as a boolean has no Rust counterpart: the port's arm is `if !$CACHE.is_null ()`
   dsafe_calln	__VA_ARGS__	the variadic arm is a Rust `$($args:expr),*` metavariable over a local array (the lisp.h safe_calln row's shape); C's `__VA_ARGS__` has no Rust token
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
    `continuation_pixel_width`, `paragraph_embedding`), `struct composition_it`,
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

### Gate evidence for this WIP state

See the final report accompanying this ledger, which quotes every gate's last line.  The
figures below were taken at the 16,194-line revision (through `hscroll_windows`, C 16929):

* `build-tu src/xdisp.c` = `built src/xdisp.rs -> ../build/femacs/xdisp.o`, taken **in place at
  the current workspace state**, after a three-step fix-up loop (the compiler caught, in order:
  the E0428 clash with termchar.rs's `tty_handle_tab_bar_click` declaration plus the
  `TYPE_RANGED_FIXNUMP`/`SET_FRAME_GARBAGED`/`glyph_matrix`-deref/`pixel_width` spellings; then
  `hscroll`/`min_hscroll`-as-`ptrdiff_t` and the by-value `it`; then the missing
  `clear_desired_matrices`).  This revision's `xdisp.rs` md5 is
  `b9950fbb6cc2494d20b86fda8669ff86` (16,194 lines) and the mirrors it was built against are
  `include/dispextern.rs a1d71da831bba153fe9bd26e70b24cbd` (its `struct it` is gated, see item
  39), `include/lisp.rs 44f7b6ab6ee488c8796c5f405f174952`, `include/window.rs
  d163253b3b643d79b39187e77411a4d4` and `lib/include/stdbit.in.rs bd5e18f5cf4b99d4d0d32412d275bf50`
  (the last two are other lanes' and mutable; every md5 above was re-read in the same shell
  step as the closing `build-tu`, so the pairing is exact).  The
  build prints 6 warnings: 4 pre-existing mirror ones and 2 `while_true` from this unit's two
  `'scan: while true` loops;
* `python3 tools/extract-abi.py --check emacs/src/xdisp.c femacs/src/xdisp.rs` = `FAIL`; its
  last two lines are `- line count drifted 59.3%: C 39805 vs Rust 16194` and
  `FAIL emacs/src/xdisp.c -> femacs/src/xdisp.rs`.  The count stays at 159 lines (the three
  new definitions retire three `missing function` rows, from 113 to 110, and this round's one
  `double` row replaces the `uninit` row the integrator landed).  Every remaining `missing …`
  line names a definition after C 16929 (the earliest is C 17006
  `text_outside_line_unchanged_p`, after the dead `#ifdef GLYPH_DEBUG` block C 16940-16997) and there is still no `stub or empty body` line;
* `python3 tools/check-cond-eval.py src/xdisp.c` = `196 conditions, 0 unknown to the evaluator,
  0 disagreements` (this round's dead arms: C 15615-15654's caller-side twin is live but the
  function is not defined here, and C 15662-16610 with its nested `HAVE_EXT_TOOL_BAR`/`GLYPH_DEBUG`
  arms is one recorded dead block);
* `python3 tools/check-citations.py` = `628 file(s) scanned, 9 drift, 1 range, 284 external,
  38 allowed, 0 stale` — `femacs/src/xdisp.rs` still has **no** finding;
* `python3 tools/check-mirror-dupes.py` = `1 duplicate declaration(s), 1 ratchet row(s), 0
  failure(s)`; the surviving row is the `insdel.rs`/`window.rs` `bset_redisplay` one, not this
  unit's;
* `python3 tools/check-declarations.py` = `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures` (none from this unit);
* `python3 tools/check-verbatim.py` = `6 trees, 4607 files, 0 failures`;
* `python3 tools/check-globals.py` = `283 units, 0 DEFVAR_* variables without a definition in
  their own unit` (the unit count moves as the other lanes add units; this run's value);
* `python3 tools/check-target-names.py` = `626 .rs files, 591 map targets, 0 finding(s)`.
  (the counts move with the other lanes).
  (the file count moves with the other lanes).

Note: the `--check` header line now says `424 files` in
`docs/baseline/config/cond-eval-verified.txt` where the previous round's said `431` — another
lane's edit to that manifest, not this unit's.
  (the file count moves with the other lanes).
  (the file count moves with the other lanes).
  (the file count moves with the other lanes).
