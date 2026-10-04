# emacs/src/window.c (9740 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 121
- [x] windowp  (Fwindowp, 1/1, doc c5be1fd35be2, line 316)
- [x] window-valid-p  (Fwindow_valid_p, 1/1, doc 78772a12eec3, line 323)
- [x] window-live-p  (Fwindow_live_p, 1/1, doc 6dc5d4d93306, line 332)
- [x] window-frame  (Fwindow_frame, 0/1, doc a77b37a0501f, line 342)
- [x] frame-root-window  (Fframe_root_window, 0/1, doc d02553c8ac77, line 350)
- [x] minibuffer-window  (Fminibuffer_window, 0/1, doc 56040b12ee90, line 372)
- [x] window-minibuffer-p  (Fwindow_minibuffer_p, 0/1, doc 483ed403b3bf, line 380)
- [x] frame-first-window  (Fframe_first_window, 0/1, doc 53f4f1f98f00, line 390)
- [x] frame-selected-window  (Fframe_selected_window, 0/1, doc f63f674c2b9d, line 416)
- [x] frame-old-selected-window  (Fframe_old_selected_window, 0/1, doc 4319eb28ca35, line 440)
- [x] set-frame-selected-window  (Fset_frame_selected_window, 2/3, doc 3ebac06329f7, line 456)
- [x] selected-window  (Fselected_window, 0/0, doc d710c692507e, line 485)
- [x] old-selected-window  (Fold_selected_window, 0/0, doc 4365f6ef7d3e, line 496)
- [x] select-window  (Fselect_window, 1/2, doc 7872657af726, line 616)
- [x] window-buffer  (Fwindow_buffer, 0/1, doc ae00f97ad969, line 645)
- [x] window-old-buffer  (Fwindow_old_buffer, 0/1, doc 41f1f1a8a37f, line 656)
- [x] window-parent  (Fwindow_parent, 0/1, doc e900577ff84f, line 680)
- [x] window-top-child  (Fwindow_top_child, 0/1, doc 07202935a2cd, line 689)
- [x] window-left-child  (Fwindow_left_child, 0/1, doc ed5a29cbf27a, line 701)
- [x] window-next-sibling  (Fwindow_next_sibling, 0/1, doc 9fcb1e092cf6, line 713)
- [x] window-prev-sibling  (Fwindow_prev_sibling, 0/1, doc 185fbadeee5c, line 722)
- [x] window-combination-limit  (Fwindow_combination_limit, 1/1, doc 843459534ea4, line 731)
- [x] set-window-combination-limit  (Fset_window_combination_limit, 2/2, doc 1a30b352b71b, line 748)
- [x] window-use-time  (Fwindow_use_time, 0/1, doc 356d8d9e8bb2, line 767)
- [x] window-bump-use-time  (Fwindow_bump_use_time, 0/1, doc 7e99d30b4dd7, line 783)
- [x] window-pixel-width  (Fwindow_pixel_width, 0/1, doc 1c331cacade2, line 808)
- [x] window-pixel-height  (Fwindow_pixel_height, 0/1, doc 59b8fae9a382, line 821)
- [x] window-old-pixel-width  (Fwindow_old_pixel_width, 0/1, doc 6045d2ce918d, line 833)
- [x] window-old-pixel-height  (Fwindow_old_pixel_height, 0/1, doc bd2625325405, line 847)
- [x] window-total-height  (Fwindow_total_height, 0/2, doc c6109eb338c0, line 861)
- [x] window-total-width  (Fwindow_total_width, 0/2, doc 1741b695a44a, line 897)
- [x] window-new-total  (Fwindow_new_total, 0/1, doc c3b81ce7f25e, line 934)
- [x] window-normal-size  (Fwindow_normal_size, 0/2, doc 31178b8163fb, line 947)
- [x] window-new-normal  (Fwindow_new_normal, 0/1, doc 4f18109e4657, line 976)
- [x] window-new-pixel  (Fwindow_new_pixel, 0/1, doc 3f62f4bb98e0, line 988)
- [x] window-pixel-left  (Fwindow_pixel_left, 0/1, doc f005e4515cfe, line 1001)
- [x] window-pixel-top  (Fwindow_pixel_top, 0/1, doc e578736e8ff6, line 1009)
- [x] window-left-column  (Fwindow_left_column, 0/1, doc a9367bad60f2, line 1017)
- [x] window-top-line  (Fwindow_top_line, 0/1, doc b3f5ad137832, line 1029)
- [x] window-body-width  (Fwindow_body_width, 0/2, doc 921fe9ce263b, line 1140)
- [x] window-body-height  (Fwindow_body_height, 0/2, doc f17d5935703c, line 1165)
- [x] window-old-body-pixel-width  (Fwindow_old_body_pixel_width, 0/1, doc eef26cd5e206, line 1185)
- [x] window-old-body-pixel-height  (Fwindow_old_body_pixel_height, 0/1, doc f8eae0301fae, line 1200)
- [x] window-mode-line-height  (Fwindow_mode_line_height, 0/1, doc 63d02409c976, line 1215)
- [x] window-header-line-height  (Fwindow_header_line_height, 0/1, doc 4d1fdd31b713, line 1224)
- [x] window-tab-line-height  (Fwindow_tab_line_height, 0/1, doc 5f988e8b7be6, line 1233)
- [x] window-right-divider-width  (Fwindow_right_divider_width, 0/1, doc d2c4c51b317e, line 1242)
- [x] window-bottom-divider-width  (Fwindow_bottom_divider_width, 0/1, doc b11a250da311, line 1251)
- [x] window-scroll-bar-width  (Fwindow_scroll_bar_width, 0/1, doc 830b493afb08, line 1260)
- [x] window-scroll-bar-height  (Fwindow_scroll_bar_height, 0/1, doc 1c95ade61829, line 1269)
- [x] window-hscroll  (Fwindow_hscroll, 0/1, doc 398661cdb6a2, line 1278)
- [x] set-window-hscroll  (Fset_window_hscroll, 2/2, doc 09a348384731, line 1313)
- [x] coordinates-in-window-p  (Fcoordinates_in_window_p, 2/2, doc 49e698b75566, line 1540)
- [x] window-at  (Fwindow_at, 2/3, doc ad225008751d, line 1749)
- [x] window-point  (Fwindow_point, 0/1, doc d601c3a5e7dd, line 1787)
- [x] window-old-point  (Fwindow_old_point, 0/1, doc 8deb1f7a03ec, line 1803)
- [x] window-start  (Fwindow_start, 0/1, doc 82115ed6b22e, line 1811)
- [x] window-end  (Fwindow_end, 0/2, doc fa14c0781be1, line 1831)
- [x] set-window-point  (Fset_window_point, 2/2, doc 7acd012b45c7, line 1903)
- [x] set-window-start  (Fset_window_start, 2/3, doc 27ce172c4f24, line 1939)
- [x] pos-visible-in-window-p  (Fpos_visible_in_window_p, 0/3, doc 7e9ae353a62b, line 1977)
- [x] window-line-height  (Fwindow_line_height, 0/2, doc d80c84746bce, line 2048)
- [x] window-lines-pixel-dimensions  (Fwindow_lines_pixel_dimensions, 0/6, doc 875f40566858, line 2159)
- [x] window-dedicated-p  (Fwindow_dedicated_p, 0/1, doc 003bb3eb2796, line 2274)
- [x] set-window-dedicated-p  (Fset_window_dedicated_p, 2/2, doc d8bbc1d4e4a9, line 2297)
- [x] window-prev-buffers  (Fwindow_prev_buffers, 0/1, doc 9bccc8e9c86d, line 2322)
- [x] set-window-prev-buffers  (Fset_window_prev_buffers, 2/2, doc f08542c04328, line 2339)
- [x] window-next-buffers  (Fwindow_next_buffers, 0/1, doc 9c7fd6fb2f9c, line 2353)
- [x] set-window-next-buffers  (Fset_window_next_buffers, 2/2, doc a0b32df2d3a2, line 2362)
- [x] window-parameters  (Fwindow_parameters, 0/1, doc 80d3da860c3f, line 2373)
- [x] window-parameter  (Fwindow_parameter, 2/2, doc 4d7c4f556bb1, line 2392)
- [x] set-window-parameter  (Fset_window_parameter, 3/3, doc 9cff1ec6916e, line 2403)
- [x] window-display-table  (Fwindow_display_table, 0/1, doc 6c7aeccd76ff, line 2455)
- [x] set-window-display-table  (Fset_window_display_table, 2/2, doc 29ca918f4df5, line 2489)
- [x] next-window  (Fnext_window, 0/3, doc 5ceaee58babe, line 2941)
- [x] previous-window  (Fprevious_window, 0/3, doc 772170ad3479, line 2980)
- [x] window-list  (Fwindow_list, 0/3, doc b655b9fcdd97, line 3056)
- [x] window-list-1  (Fwindow_list_1, 0/3, doc 5440be221025, line 3079)
- [x] get-buffer-window  (Fget_buffer_window, 0/2, doc 2c4a478c09f5, line 3309)
- [x] window-discard-buffer-from-window  (Fwindow_discard_buffer, 2/3, doc d6c8478b6450, line 3445)
- [x] delete-other-windows-internal  (Fdelete_other_windows_internal, 0/2, doc 7803c2423cfd, line 3467)
- [x] run-window-configuration-change-hook  (Frun_window_configuration_change_hook, 0/1, doc 00ff026ef41e, line 3839)
- [x] run-window-scroll-functions  (Frun_window_scroll_functions, 0/1, doc 841351c875c3, line 3852)
- [x] set-window-buffer  (Fset_window_buffer, 2/3, doc 600269f81805, line 4432)
- [x] force-window-update  (Fforce_window_update, 0/1, doc 7950a12c54d1, line 4492)
- [x] set-window-new-pixel  (Fset_window_new_pixel, 2/3, doc 73499fc063c0, line 4639)
- [x] set-window-new-total  (Fset_window_new_total, 2/3, doc 58b2c0ed3d6e, line 4667)
- [x] set-window-new-normal  (Fset_window_new_normal, 1/2, doc f2f9d88fdefe, line 4693)
- [x] window-resize-apply  (Fwindow_resize_apply, 0/2, doc 83cbfb3bcd60, line 4961)
- [x] window-resize-apply-total  (Fwindow_resize_apply_total, 0/2, doc e943ad8eb047, line 5003)
- [x] combine-windows  (Fcombine_windows, 2/2, doc ed740c308acd, line 5213)
- [x] uncombine-window  (Funcombine_window, 1/1, doc 07acf8877564, line 5324)
- [x] split-window-internal  (Fsplit_window_internal, 4/5, doc 1234bb88c4e3, line 5378)
- [x] delete-window-internal  (Fdelete_window_internal, 1/1, doc 8b65e1d0ab37, line 5688)
- [x] resize-mini-window-internal  (Fresize_mini_window_internal, 1/1, doc 4c3341c95a9a, line 5971)
- [x] scroll-up  (Fscroll_up, 0/1, doc 9b7b0100bfae, line 7026)
- [x] scroll-down  (Fscroll_down, 0/1, doc 565795aa7a39, line 7039)
- [x] other-window-for-scrolling  (Fother_window_for_scrolling, 0/0, doc 84bcfb67494c, line 7052)
- [x] scroll-left  (Fscroll_left, 0/2, doc 421cca508f84, line 7101)
- [x] scroll-right  (Fscroll_right, 0/2, doc 0cdc0f952798, line 7127)
- [x] minibuffer-selected-window  (Fminibuffer_selected_window, 0/0, doc 69a7b47150c0, line 7153)
- [x] recenter  (Frecenter, 0/2, doc c8e845a8c29e, line 7219)
- [x] window-text-width  (Fwindow_text_width, 0/2, doc 1e27f77a3fb8, line 7459)
- [x] window-text-height  (Fwindow_text_height, 0/2, doc ff83cd36e498, line 7481)
- [x] move-to-window-line  (Fmove_to_window_line, 1/1, doc 4d2b3bf3da69, line 7502)
- [x] window-configuration-p  (Fwindow_configuration_p, 1/1, doc f8f708e9472b, line 7640)
- [x] window-configuration-frame  (Fwindow_configuration_frame, 1/1, doc ae13916e5eae, line 7647)
- [x] set-window-configuration  (Fset_window_configuration, 1/3, doc d9ce973de1c0, line 7661)
- [x] current-window-configuration  (Fcurrent_window_configuration, 0/1, doc d11bb0107d03, line 8395)
- [x] set-window-margins  (Fset_window_margins, 2/3, doc 927a837da413, line 8495)
- [x] window-margins  (Fwindow_margins, 0/1, doc e50404901cc8, line 8520)
- [x] set-window-fringes  (Fset_window_fringes, 2/5, doc 8ed627f9c04d, line 8605)
- [x] window-fringes  (Fwindow_fringes, 0/1, doc 941a663984cf, line 8639)
- [x] set-window-cursor-type  (Fset_window_cursor_type, 2/2, doc 5fee1876606f, line 8656)
- [x] window-cursor-type  (Fwindow_cursor_type, 0/1, doc 247260200c46, line 8694)
- [x] window-cursor-info  (Fwindow_cursor_info, 0/1, doc 40a1e184415b, line 8703)
- [x] set-window-scroll-bars  (Fset_window_scroll_bars, 1/6, doc 3432b78b8d5f, line 8857)
- [x] window-scroll-bars  (Fwindow_scroll_bars, 0/1, doc efc93e45b619, line 8898)
- [x] window-vscroll  (Fwindow_vscroll, 0/2, doc dca0c3e02603, line 8937)
- [x] set-window-vscroll  (Fset_window_vscroll, 2/4, doc 602fb9fc9df0, line 8960)
- [x] window-configuration-equal-p  (Fwindow_configuration_equal_p, 2/2, doc 8f778d88b63e, line 9135)

## Functions defined — 113
- [x] CHECK_WINDOW_CONFIGURATION  (line 124, static)
- [x] wset_combination_limit  (line 131, static)
- [x] wset_dedicated  (line 137, static)
- [x] wset_display_table  (line 143, static)
- [x] wset_new_normal  (line 149, static)
- [x] wset_new_total  (line 155, static)
- [x] wset_normal_cols  (line 161, static)
- [x] wset_normal_lines  (line 167, static)
- [x] wset_parent  (line 173, static)
- [x] wset_pointm  (line 179, static)
- [x] wset_old_pointm  (line 185, static)
- [x] wset_start  (line 191, static)
- [x] wset_temslot  (line 197, static)
- [x] wset_vertical_scroll_bar_type  (line 203, static)
- [x] wset_window_parameters  (line 209, static)
- [x] wset_combination  (line 215, static)
- [x] window_outdated  (line 231)
- [x] decode_live_window  (line 239)
- [x] decode_any_window  (line 249)
- [x] decode_valid_window  (line 262, static)
- [x] adjust_window_count  (line 278, static)
- [x] wset_buffer  (line 299)
- [x] wset_old_buffer  (line 311, static)
- [x] select_window  (line 514, static)
- [x] select_window_1  (line 592, static)
- [x] window_body_unit_from_symbol  (line 1042, static)
- [x] window_body_height  (line 1056)
- [x] window_body_width  (line 1096)
- [x] set_window_hscroll  (line 1289, static)
- [x] coordinates_in_window  (line 1348, static)
- [x] window_relative_x_coord  (line 1494)
- [x] check_window_containing  (line 1651, static)
- [x] window_from_coordinates  (line 1686)
- [x] window_point  (line 1780)
- [x] window_parameter  (line 2384)
- [x] window_display_table  (line 2470)
- [x] unshow_buffer  (line 2501, static)
- [x] replace_window  (line 2551, static)
- [x] recombine_windows  (line 2612, static)
- [x] delete_deletable_window  (line 2680, static)
- [x] add_window_to_list  (line 2695, static)
- [x] window_list  (line 2710)
- [x] candidate_window_p  (line 2758, static)
- [x] decode_next_window_args  (line 2827, static)
- [x] next_window  (line 2870, static)
- [x] window_list_1  (line 3024, static)
- [x] mru_rooted_frame  (line 3119)
- [x] window_loop  (line 3165, static)
- [x] check_all_windows  (line 3304)
- [x] resize_root_window  (line 3344, static)
- [x] window_pixel_to_total  (line 3354, static)
- [x] window_discard_buffer_from_alist  (line 3363, static)
- [x] window_discard_buffer_from_list  (line 3385, static)
- [x] window_discard_buffer_from_window  (line 3402, static)
- [x] window_discard_buffer_from_dead_windows  (line 3437)
- [x] replace_buffer_in_windows  (line 3722)
- [x] replace_buffer_in_windows_safely  (line 3742)
- [x] run_funs  (line 3759, static)
- [x] select_window_norecord  (line 3767, static)
- [x] select_frame_norecord  (line 3774, static)
- [x] run_window_configuration_change_hook  (line 3790, static)
- [x] window_sub_list  (line 3884, static)
- [x] window_change_record_windows  (line 3919, static)
- [x] window_change_record  (line 3959, static)
- [x] run_window_change_functions_locally  (line 4003, static)
- [x] run_window_change_functions_globally  (line 4029, static)
- [x] run_window_change_functions  (line 4111)
- [x] set_window_buffer  (line 4336)
- [x] display_buffer  (line 4487, static)
- [x] temp_output_buffer_show  (line 4537)
- [x] allocate_window  (line 4591, static)
- [x] make_window  (line 4599)
- [x] window_resize_check  (line 4716, static)
- [x] window_resize_apply  (line 4815, static)
- [x] window_resize_apply_total  (line 4905, static)
- [x] resize_frame_windows  (line 5046)
- [x] make_parent_window  (line 5142, static)
- [x] window_set_parent_and_normal_sizes  (line 5161, static)
- [x] resize_mini_window_apply  (line 5861, static)
- [x] grow_mini_window  (line 5901)
- [x] shrink_mini_window  (line 5943)
- [x] mark_window_cursors_off  (line 6010)
- [x] window_wants_mode_line  (line 6037)
- [x] window_wants_header_line  (line 6067)
- [x] window_wants_tab_line  (line 6101)
- [x] window_internal_height  (line 6124)
- [x] window_scroll  (line 6153, static)
- [x] window_scroll_margin  (line 6207)
- [x] sanitize_next_screen_context_lines  (line 6233, static)
- [x] window_scroll_for_long_lines  (line 6242, static)
- [x] window_scroll_pixel_based  (line 6313, static)
- [x] window_scroll_line_based  (line 6789, static)
- [x] scroll_command  (line 6975, static)
- [x] displayed_window_lines  (line 7170, static)
- [x] restore_window_configuration  (line 8119)
- [x] restore_focus_frame  (line 8130)
- [x] delete_all_child_windows  (line 8155)
- [x] count_windows  (line 8194, static)
- [x] get_leaf_windows  (line 8208, static)
- [x] get_phys_cursor_glyph  (line 8228)
- [x] save_window_save  (line 8263, static)
- [x] apply_window_adjustment  (line 8444, static)
- [x] extract_dimension  (line 8459, static)
- [x] set_window_margins  (line 8467, static)
- [x] set_window_fringes  (line 8544, static)
- [x] set_window_scroll_bars  (line 8770, static)
- [x] foreach_window  (line 9018, static)
- [x] foreach_window_1  (line 9033, static)
- [x] compare_window_configurations  (line 9065, static)
- [x] init_window_once  (line 9151)
- [x] init_window_once_for_pdumper  (line 9172, static)
- [x] init_window  (line 9212)
- [x] syms_of_window  (line 9218)

## Macros defined — 1
- [x] SAVED_WINDOW_N  (line 7637, body 010fc440c3df)

## Types and their fields — 3
- [x] check_window_data: Lisp_Object* window, int x, int y, enum window_part* part
- [x] save_window_data: union vectorlike_header header, Lisp_Object selected_frame, Lisp_Object current_window, Lisp_Object f_current_buffer, Lisp_Object minibuf_scroll_window, Lisp_Object minibuf_selected_window, Lisp_Object root_window, Lisp_Object focus_frame, Lisp_Object saved_windows, int change_stamp, int frame_cols, int frame_lines, int frame_menu_bar_lines, int frame_tab_bar_lines, int frame_tool_bar_lines, int frame_text_width, int frame_text_height, int frame_menu_bar_height, int frame_tab_bar_height, int frame_tool_bar_height
- [x] saved_window: union vectorlike_header header, Lisp_Object window, Lisp_Object buffer, Lisp_Object old_buffer, Lisp_Object start, Lisp_Object pointm, Lisp_Object old_pointm, Lisp_Object pixel_left, Lisp_Object pixel_top, Lisp_Object pixel_height, Lisp_Object pixel_width, Lisp_Object left_col, Lisp_Object top_line, Lisp_Object total_cols, Lisp_Object total_lines, Lisp_Object normal_cols, Lisp_Object normal_lines, Lisp_Object hscroll, Lisp_Object min_hscroll, Lisp_Object hscroll_whole, Lisp_Object suspend_auto_hscroll, Lisp_Object vscroll, Lisp_Object parent, Lisp_Object prev, Lisp_Object start_at_line_beg, Lisp_Object display_table, Lisp_Object left_margin_cols, Lisp_Object right_margin_cols, Lisp_Object left_fringe_width, Lisp_Object right_fringe_width, Lisp_Object fringes_outside_margins, Lisp_Object fringes_persistent, Lisp_Object scroll_bar_width, Lisp_Object vertical_scroll_bar_type, Lisp_Object scroll_bar_height, Lisp_Object horizontal_scroll_bar_type, Lisp_Object scroll_bars_persistent, Lisp_Object dedicated, Lisp_Object combination_limit, Lisp_Object window_parameters

## File-scope objects — 13
- [x] Lisp_Object selected_window  (line 84)
- [x] static Lisp_Object old_selected_window  (line 89, static)
- [x] Lisp_Object Vwindow_list  (line 94)
- [x] static bool window_change_record_frames  (line 97, static)
- [x] Lisp_Object minibuf_window  (line 103)
- [x] Lisp_Object minibuf_selected_window  (line 107)
- [x] Lisp_Object last_mouse_window  (line 110)
- [x] static EMACS_INT sequence_number  (line 113, static)
- [x] static int window_scroll_pixel_based_preserve_x  (line 116, static)
- [x] static int window_scroll_pixel_based_preserve_y  (line 117, static)
- [x] static EMACS_INT window_scroll_preserve_hpos  (line 120, static)
- [x] static EMACS_INT window_scroll_preserve_vpos  (line 121, static)
- [x] EMACS_INT window_select_count  (line 506)

# ----------------------------------------------------------------------
## Decisions

**Port status: DONE — the unit is transcribed end to end.**  `femacs/src/window.rs`
transcribes **`emacs/src/window.c` lines 1-9740**: the 121 subrs, the 113 functions, the one
macro, the three records and the thirteen file-scope objects, plus the file's `#include`
prelude.  The second lane finished C 4905-9740 (the window-configuration save/restore block,
the scroll machinery, `foreach_window`, `init_window_once`/`init_window` and `syms_of_window`)
and the C 1540-1938 gap the first lane had left inside 1-4905 (`Fcoordinates_in_window_p`,
`window_from_coordinates`, `Fwindow_at`, `window_point`, `Fwindow_point`, `Fwindow_old_point`,
`Fwindow_start`, `Fwindow_end`, `Fset_window_point` and their helpers).

`cargo run -p xtask -- build-tu src/window.c` prints `built src/window.rs ->
../build/femacs/window.o`; `--cfg enable_checking` compiles too; the object's exported and
undefined symbols are compared with the reference object in the report.  Everything the
ledger's boxes record is ticked (251/251).

### The transcription that is in place

* Same statements in the same order, same names (function, struct, struct field, macro,
  file-scope object, local), same control flow, same error model (PORTING.md 0.1/0.2/0.3).
* `DEFUN!` for C's `DEFUN` (81 subrs in this range), `#[unsafe(no_mangle)] pub unsafe extern "C"
  fn` for the non-`static` C definitions, plain `unsafe fn` for the C `static` helpers.
* The nineteen `bool_bf` members of `struct window` share `bits` (window.h 392-410), so C's
  reads/writes go through `window_bits!(<member>, w[, v])` with the member name kept as the
  macro's first argument (PORTING.md 1.3/P-80; the mirror carries the shift constants and the
  read-only `window_bit`).
* The three `#if false` regions (C 2508-2513, 7518-7520 and 7556-7567) are recorded, not
  transcribed (P-08).

### Dead arms (P-08: recorded here, not transcribed)

**Correction (lane 2).**  The first lane's premise — "`emacs/src/window.c` is in
`docs/baseline/config/cond-eval-verified.txt`" — is **false**: only `src/window.h` is listed
(line 418 of that file).  `python3 tools/check-cond-eval.py src/window.c` does report
`17 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the file *could* be listed; until
the integrator adds it (`tools/cinventory.CONDITION_MACROS_PATHS` consults the table only for
listed files), `extract-abi.py` keeps both arms of every `#if` in this file and its token check
must be satisfied for the dead text too.  The route taken here is: record the arms (as below)
and exempt their tokens in `docs/ports/macro-body-notes.tsv` (the rows are in the report's
"追加行" section) — except `Fwindow_cursor_info`'s arm, which is *carried* under
`#[cfg (feature = "window_system")]` because `token-aliases.tsv`'s `type` family gets an
*occurrence* check that the notes cannot relax.  The frozen configuration's dead text is
otherwise not repeated:

* C 24-26 — `#if 10 <= __GNUC__` (clang reports `__GNUC__` 4; the diagnostic pragma is dead).
* C 42-44, 1716-1729, 2580-2583, 4624-4627, 7265-7267, 8732-8753 — the `HAVE_WINDOW_SYSTEM`
  arms (`#include TERM_HEADER`, the tab-bar window probe, the `phys_cursor_type`/
  `phys_cursor_width` reset, `minimize_tab_bar_window_p`, the `CURRENT_TAB_LINE_HEIGHT` write,
  and the GUI body of `Fwindow_cursor_info`).
* C 45-47 — the `MSDOS` arm (`#include "msdos.h"`).
* C 1701-1714, 1731-1744 — `HAVE_EXT_MENU_BAR`/`HAVE_EXT_TOOL_BAR` (both defined for this
  target, so these toolkit-menu/tool-bar probes are dead).
* C 2508-2513 — `#if false` in `unshow_buffer`.
* C 2792-2801 — the `HAVE_X_WINDOWS` half of `candidate_window_p`'s visibility test.
* C 4282-4292 — the `HAVE_TEXT_CONVERSION` arm of `run_window_change_functions`.
* C 7268-7270 — the `! defined (HAVE_EXT_TOOL_BAR)` half of `Frecenter`.
* C 8809-8839 — `#if USE_HORIZONTAL_SCROLL_BARS`; the object-like macro is `false` here, so
  C's live text is the `#else` arm's `wset_horizontal_scroll_bar_type (w, Qnil)` (the mirror
  keeps `USE_HORIZONTAL_SCROLL_BARS` as a `const`).

### Use-site declarations (headers without a mirror, or mirror gaps; each keeps C's prototype in a `/// C:` line)

`include/window.rs` is a WIP mirror: it carries the two records, the flag shifts, the
read-only accessors and the redisplay entry points, but **not** the accessors that dereference
`struct frame`, the `window_wants_*`/`CURRENT_*_LINE_HEIGHT` family, `WINDOW_FRAME`/
`WINDOW_XFRAME` or the mode/header/tab-line heights.  Those are defined in `window.rs` under
C's own name, with C's body, as `macro_rules!`: `WINDOW_FRAME`, `WINDOW_XFRAME`,
`WINDOW_FRAME_COLUMN_WIDTH`/`_LINE_HEIGHT`, `MIN_SAFE_WINDOW_PIXEL_WIDTH`/`_HEIGHT`,
`WINDOW_RIGHTMOST_P`/`_BOTTOMMOST_P`/`_FULL_WIDTH_P`, `WINDOW_RIGHT_DIVIDER_WIDTH`/
`_BOTTOM_DIVIDER_WIDTH`, `WINDOW_LEFT_EDGE_X`/`_RIGHT_EDGE_X`/`_TOP_EDGE_Y`/`_BOTTOM_EDGE_Y`,
`WINDOW_BOX_{LEFT,RIGHT}_{PIXEL_,}EDGE_*`, `WINDOW_*_MARGIN_WIDTH`, `WINDOW_LEFT_FRINGE_WIDTH`/
`_RIGHT_FRINGE_WIDTH`/`_FRINGES_WIDTH`, `WINDOW_VERTICAL_SCROLL_BAR_TYPE` and the
`WINDOW_HAS_VERTICAL_SCROLL_BAR*` family, the `WINDOW_*_SCROLL_BAR_*` / `WINDOW_CONFIG_*`
family, `WINDOW_{MODE,HEADER,TAB}_LINE_HEIGHT`, `WINDOW_BOX_HEIGHT_NO_MODE_LINE`/
`WINDOW_BOX_TEXT_HEIGHT`, `WINDOW_CONFIGURATIONP`, `MATRIX_{MODE,HEADER,TAB}_LINE_HEIGHT`,
`CURRENT_{MODE,HEADER,TAB}_LINE_HEIGHT` and the `CURRENT_*_ACTIVE_FACE_ID` chain,
`FRAME_PIXEL_{X,Y}_FROM_CANON_*`/`FRAME_CANON_{X,Y}_FROM_PIXEL_*`, `CHECK_FRAME`/
`CHECK_LIVE_FRAME`, `BYTEPOS`/`CHARPOS`/`SET_TEXT_POS_FROM_MARKER`, `FRAME_FACE_CACHE`,
`set_frame_flag1`/`set_frame_flag2` plus the pack/unpack helpers for the `struct frame` flags
this file reads and writes (`select_mini_window_flag`, `can_set_window_size`,
`after_make_frame`, `face_change`, `FRAME_WINDOW_CHANGE`, `FRAME_WINDOW_STATE_CHANGE`).

Declarations of definitions that live in **other** units and whose headers' mirrors do not
carry the prototype (PORTING.md 0.1-2/P-72 rule 3) — `decode_live_frame`, `decode_any_frame`,
`redisplay_other_windows`, `lookup_named_face`, `window_box_width`/`_left`/`_box`/`_height`/
`_left_offset`/`_right_offset`, `pos_visible_p`, `window_text_bottom_y`,
`wset_update_mode_line`, `estimate_mode_line_height`, `nconc2`, `root_frame`, `safe_run_hooks`,
`is_minibuffer`, `this_minibuffer_depth`, `mark_window_display_accurate`,
`reset_mouse_highlight`, `free_window_matrices`, `vmotion`, `MOUSE_HL_INFO`,
`record_unwind_protect_void`, `other_buffer_safely`, `run_hook_with_args_2`,
`adjust_frame_glyphs` — plus the records those need (`struct font` with `height`/
`space_width`/`average_width`, offsets measured with the frozen clang: 156/160/164, sizeof
184; `struct face` with `font`@216, sizeof 280; `struct face_cache`).

### Deviations (each one also named in the report)

1. `Fset_window_buffer`'s C local `prev_buffer` is renamed `prev_buffer_` in the port: the
   `keyboard.rs` mirror declares a `prev_buffer` **static** at the crate root, and a Rust local
   may not shadow a static (`E0530`).
2. `set_window_buffer`'s two `XMARKER (…)->insertion_type` writes go through the mirror's
   `MARKER_INSERTION_TYPE` mask (lisp.h 313-314 packs the two `bool_bf` members into
   `Lisp_Marker.bits`); the C token is kept in a local named `insertion_type`, which is what
   `docs/ports/token-aliases.tsv`'s `insertion_type` family counts.
3. C's `UNINIT` (`conf_post.h` 371: `/* empty */` on this target) is transcribed as a
   zero-initialised local and named in this ledger (PORTING.md 1.5):
   `Fdelete_other_windows_internal`'s `swindow`/`startpos`/`startbyte`/`top`.
4. `wset_buffer`/`adjust_window_count` etc. keep C's `int`→`EMACS_INT` conversions explicit
   (`as EMACS_INT`); `make_fixnum` is typed `EMACS_INT` while C's implicit conversions are
   tacit, so the cast is where C's promotion is.
5. C's `!int` / `||`-on-`int` conditions (`!FRAME_WINDOW_P (f)`,
   `windows_or_buffers_changed || …`) are spelled as the equivalent Rust comparisons
   (`== 0`, `!= 0`); the C operand tokens are unchanged.

### Lane-2 increments (deviations, fixes and records added when the unit was finished)

1. **Use-site records this unit needed for C 4905-9740** (each with a `/// C:` line above it,
   PORTING.md 0.1-2/P-72 rule 3, all of them mirror gaps the report lists): `struct it`
   (dispextern.h 2430-2936, sizeof 4,968, every offset measured with clang and pinned by
   `const _` asserts — the `indent.rs` shape; `#[derive (Clone, Copy)]` because C copies the
   iterator by value in `window_scroll_pixel_based`), `enum display_element_type` +
   `IT_*`/`IT_STACK_SIZE`/`OVERLAY_STRING_CHUNK_SIZE`, `enum move_operation_enum` +
   `MOVE_TO_*`, `margin_unit` + `MARGIN_IN_*`, `SET_TEXT_POS`, `IT_BYTEPOS`, `WINDOW_BUFFER`,
   `FUNCTIONP`, `KVAR` (one arm: `Vlast_command`), `VECSIZE`, `SET_FRAME_GARBAGED`,
   `FRAME_WINDOWS_FROZEN` (writer), `redisplay`/`can_set_window_size` (writers for the two
   `bool_bf` members the mirror only reads), and the prototypes of `bidi_shelve_cache`,
   `bidi_unshelve_cache`, `start_display`, `move_it_vertically`, `move_it_past_eol`,
   `move_it_by_lines`, `move_it_to`, `move_it_vertically_backward`, `init_iterator`,
   `line_bottom_y`, `in_display_vector_p`, `partial_line_height`, `clear_glyph_matrix`,
   `default_line_pixel_height`, `display_working_on_window_p`,
   `unwind_display_working_on_window`, `record_unwind_protect_excursion`, `adjust_frame_size`,
   `do_switch_frame`, `make_initial_frame`, and the `xwidget.h` `INLINE` no-op
   `xwidget_view_delete_all_in_window` (the `buffer.rs` `kill_buffer_xwidgets` shape).
2. **The `syms_of_window` storage.**  C's `DEFVAR_LISP`/`DEFVAR_BOOL`/`DEFVAR_INT` name fields
   of the single `globals` object; the port defines the 25 objects in this unit with
   `#[unsafe(no_mangle)] pub static mut`, initialised as C zero-initialises `globals`
   (`lisp_h_Qnil ()`/`false`/`0`) — `window.rs`'s definitions shadow
   `include/globals.rs`'s `variables` glob import (P-21).
3. **Zero-initialised locals** (PORTING.md 1.5, C leaves them uninitialised and this unit only
   names them here): `window_from_coordinates`'s `dummy` and `cw` (a struct cannot be
   partially initialised), `Fwindow_end`'s `startp`/`it`, `window_scroll_pixel_based`'s `it`,
   `Frecenter`'s `iarg` (C's `UNINIT`), its three iterators and `pt`, `displayed_window_lines`'
   `it`/`start`, `window_scroll_for_long_lines`' `pos`/`rtop`/`rbot`/`dummy_*`,
   `init_window_once_for_pdumper`'s `old_mode_line_in_non_selected_windows`.
4. **C comma expressions became statements**: `cw.window = &raw mut window; cw.x = x; cw.y = y;
   cw.part = part;` (C's single comma expression), the two `x = … ; (`apply_window_adjustment
   (w), Qt)` returns became `if !w.is_null () { apply_window_adjustment (w); Qt () } else
   { Qnil () }`, and `w->start_at_line_beg = w->suspend_auto_hscroll = …`-style chains
   (`old_selected_frame = Vterminal_frame = selected_frame`, `hare = tortoise = …`) became
   single assignments in C's order.
5. **Four latent defects in the C 1-4905 text that only a full build could show** (each fixed
   here, no C semantics changed): the first lane used `XSETCDR! (…)`, `WINDOW_BUFFER (…)` and
   `call0 (…)` with spellings the mirrors do not define (`XSETCDR` is a function, `WINDOW_BUFFER`
   a macro this file now declares, `call0` a function), and `add_window_to_list` was spelled
   `unsafe fn` although its address is handed to `foreach_window` (PORTING.md 1.4: a helper
   passed as a C function pointer must be `unsafe extern "C" fn`).
6. **Two `--cfg enable_checking` defects** in the same text: `unshow_buffer`'s
   `eassert (b == XMARKER (w->pointm)->buffer)` needed `b as *mut c_void` (the mirror's
   `Lisp_Marker.buffer` is untyped) and `scroll_command`'s `eabs` is a macro (`eabs!`).
7. **`vmotion`'s declaration** carried `vtarget: c_int` while the C prototype (indent.h 39-40)
   and the mirror copy in `include/indent.rs` use `EMACS_INT`: on AArch64 a 32-bit parameter
   leaves the upper half of the register unspecified, so the six call sites now pass
   `EMACS_INT`.  rustc still warns (`clashing_extern_declarations`) because the mirror's copy
   cannot name `struct window` and spells `*mut c_void`; the ABI is the same pointer.
8. **`Fwindow_cursor_info`'s dead arm is carried** under `#[cfg (feature = "window_system")]`
   instead of recorded — see the correction under "Dead arms"; it is the one dead arm whose
   tokens cannot be exempted (the `type` family's occurrence check).

### Integrator to-do list (the mirror gaps this unit reports and does not touch)

1. `include/window.rs`: the accessors and the `window_wants_*` family above should move into the
   mirror when the display chain's `CURRENT_*_LINE_HEIGHT` prerequisites land (PORTING.md P-26:
   one spelling per construct); the unit's copies are then the duplicates to delete.
2. `include/dispextern.rs`: `struct face`/`struct face_cache`, `FACE_FROM_ID_OR_NULL`
   (this file re-declares a minimal `face`/`face_cache` with measured offsets, the
   `term.rs`/`alloc.rs` shape).  **`MOUSE_HL_INFO` is done** (status 3.109): it is a macro in
   `frame.h` 1023-1039, not a declaration in dispextern.h, and it now lives in `include/frame.rs`
   - this unit's use-site `unsafe extern "C"` declaration and `src/dispnew.rs`'s two calls are the
   macro now, which is what removes the `_MOUSE_HL_INFO` symbol C never had.
3. `include/font.rs`: `struct font` is not mirrored at all (only `height`, `space_width` and
   `average_width` are read here).
4. `include/disptab.rs` declares `window_display_table` at its **top level** (disptab.h 67,
   "Defined in window.c"); the unit therefore defines it inside `window_defined_decls` and
   glob-imports (PORTING.md 0.1-10/P-21).
5. `docs/baseline/config/cond-eval-verified.txt`: add `src/window.c` (see the correction
   above).  That single line removes the need for most of the report's notes rows
   (`replace_window`, `make_window`, `candidate_window_p`, `run_window_change_functions`,
   `window_from_coordinates`, `Frecenter`, `set_window_scroll_bars`) and lets the carried
   `window_system` arm go back to being recorded.
6. `docs/ports/macro-body-notes.tsv`: append the rows listed in the report's "给 integrator 的
   追加行" section (10 rows: the null-pointer `0` family, the `double` family, the
   `switch/case/default/break` and `goto` families, the dead-arm token lists and `syms_of_window
   doc`).  With them applied, `extract-abi.py --check` prints `OK`.
7. `include/dispextern.rs`: `struct it` (dispextern.h 2430-2936, 4,968 bytes) is not mirrored;
   this unit declares its own measured copy (named fields + byte pads + `const _` asserts) —
   the `indent.rs` precedent.  Same for `enum display_element_type`, `IT_STACK_SIZE`,
   `OVERLAY_STRING_CHUNK_SIZE`, `enum move_operation_enum`, `SET_TEXT_POS`, `IT_BYTEPOS`,
   `WINDOW_BUFFER`, `FUNCTIONP`, `KVAR`, `VECSIZE`, `margin_unit`, `SET_FRAME_GARBAGED`,
   `FRAME_WINDOWS_FROZEN` (writer) and `redisplay`/`can_set_window_size` (writers).

### Gate evidence for this DONE state

See the final report accompanying this ledger, which quotes every gate's last line:

* `build-tu src/window.c` = `built src/window.rs -> ../build/femacs/window.o`;
* `rustc --cfg enable_checking ../build/femacs/units/window/_root.rs` = exit 0 (3 warnings);
* `python3 tools/extract-abi.py --check emacs/src/window.c femacs/src/window.rs` = `OK …`
  **with the 10 rows of the "追加行" section appended to
  `docs/ports/macro-body-notes.tsv`**; with the shared file untouched it reports the 26
  token-class items those rows cover (no missing function/subr/field/macro and no line drift);
* `make -C build/emacs-ref/src window.o` then `python3 tools/check-symbols.py --dir src` = 73
  units compared, 2 mismatched (`src/coding`, `src/keyboard` — other lanes' units), 0 still
  pending; `src/window` is neither missing nor invented;
* `python3 tools/check-globals.py` = 273 units, 0 DEFVAR_* variables without a definition in
  their own unit; `python3 tools/check-mirror-dupes.py` = 1 duplicate (the ratcheted
  `insdel.rs` one), 0 failures; `python3 tools/check-verbatim.py` = 6 trees, 4,607 files,
  0 failures.

## 2026-10-04 (integrator): src/window.c joins cond-eval-verified.txt, and the rows shrink

The lane's report was right: `src/window.c` was missing from
`docs/baseline/config/cond-eval-verified.txt` (only `window.h` was listed) even though
`tools/check-cond-eval.py src/window.c` reports `17 conditions, 0 unknown, 0 disagreements
[VERIFIED]`.  The integrator added the line (the whole list re-checks: 5,734 conditions, 4
unknown to the evaluator, 0 disagreements), so the extractor now decides those `#if`s and stops
demanding the dead-arm text.  Consequences, each measured with `extract-abi.py --check`:

- the long row for `replace_window`/`make_window`/`candidate_window_p`/
  `run_window_change_functions`/`window_from_coordinates` is deleted (its 22 tokens were the
  HAVE_WINDOW_SYSTEM arms the ledger records);
- the `set_window_scroll_bars` row is deleted (`USE_HORIZONTAL_SCROLL_BARS` is an object-like
  `false` in the frozen table, so the `#else` single statement is the live text);
- the `Frecenter` row keeps only `uninit` (the two `minimize_*` tokens were the same cause).

`extract-abi.py --check emacs/src/window.c femacs/src/window.rs` is still `OK` after the prune.
