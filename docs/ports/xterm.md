# emacs/src/xterm.c (33125 lines)

> **This file is not compiled by the frozen configuration**: `docs/baseline/frozen-build-sources.txt`
> has no `src/xterm.c` (this target's platform object variables are empty - `WINDOW_SYSTEM_OBJ=`,
> `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=` in `build/emacs-ref/src/Makefile`), so `make -n`
> never names the file.  Until round 188 the port was the recorded empty unit; round 188 (lane
> `xterm`) opened it and landed **slice 1 = C 625-990** and **slice 2 = C 1131-1762**
> (`femacs/src/xterm.rs`), so the checklist below is now a real list again and the unchecked
> boxes are the lane's remaining demand.  C 987-1130 (the `x_atom_refs` table) is deliberately
> not landed yet - Decisions 9 says why.
>
> The unit keeps the file's own header mirror's shape (`femacs/src/include/xterm.rs`, b03d883):
> the whole body carries `#[cfg (feature = "have_x_windows")]` - P-51's arm for the non-frozen
> side - and every inner condition becomes the lowercased feature name C's macro spells.  What
> that means for verification is in Decisions 1 and 2, and the measured readings are in Evidence.
>
> `tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs` was **460** `- ` lines
> before slice 1 and is **443** after it, and **324** after slice 2 (C 1131-1762, the drag-and-drop state and the
> Motif message layouts); the list below is that same frozen reading, which is
> *not* the configuration that compiles the file (Decisions 2).

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 310
- [x] x_defer_selection_requests  (line 900)
- [x] x_release_selection_requests  (line 947, static)
- [x] x_release_selection_requests_and_flush  (line 953)
- [x] xm_side_effect_from_action  (line 1764, static)
- [x] xm_operations_from_actions  (line 1779, static)
- [x] xm_read_targets_table_header  (line 1801, static)
- [x] xm_read_targets_table_rec  (line 1826, static)
- [x] xm_find_targets_table_idx  (line 1859, static)
- [x] x_atoms_compare  (line 1884, static)
- [x] xm_write_targets_table  (line 1890, static)
- [x] xm_write_drag_initiator_info  (line 1937, static)
- [x] xm_drag_window_error_handler  (line 1961, static)
- [x] xm_drag_window_io_error_handler  (line 1970, static)
- [x] x_special_window_exists_p  (line 1980, static)
- [x] xm_get_drag_window_1  (line 2015, static)
- [x] xm_get_drag_window  (line 2166, static)
- [ ] xm_setup_dnd_targets  (line 2176, static)
- [ ] xm_get_drag_atom_1  (line 2403, static)
- [ ] xm_get_drag_atom  (line 2512, static)
- [ ] xm_setup_drag_info  (line 2526, static)
- [ ] xm_send_drop_message  (line 2563, static)
- [ ] xm_send_top_level_enter_message  (line 2588, static)
- [ ] xm_send_drag_motion_message  (line 2615, static)
- [ ] xm_send_top_level_leave_message  (line 2646, static)
- [ ] xm_read_drop_start_reply  (line 2705, static)
- [ ] xm_read_drop_start_message  (line 2736, static)
- [ ] xm_read_drag_receiver_info  (line 2774, static)
- [ ] xm_read_drag_motion_message  (line 2830, static)
- [ ] xm_read_drag_motion_reply  (line 2864, static)
- [ ] x_dnd_send_xm_leave_for_drop  (line 2897, static)
- [ ] x_dnd_free_toplevels  (line 2921, static)
- [ ] x_dnd_compute_toplevels  (line 3013, static)
- [ ] x_dnd_io_error_handler  (line 3617, static)
- [ ] x_dnd_get_target_window_1  (line 3721, static)
- [ ] x_dnd_get_wm_state_and_proto  (line 3820, static)
- [ ] x_dnd_do_unsupported_drop  (line 3986, static)
- [ ] x_dnd_send_unsupported_drop  (line 4082, static)
- [ ] x_dnd_fill_empty_target  (line 4128, static)
- [ ] x_dnd_get_target_window  (line 4140, static)
- [ ] x_dnd_get_window_proxy  (line 4469, static)
- [ ] x_dnd_get_window_proto  (line 4511, static)
- [ ] x_dnd_send_enter  (line 4545, static)
- [ ] x_dnd_send_position  (line 4583, static)
- [ ] x_dnd_send_leave  (line 4658, static)
- [ ] x_dnd_send_drop  (line 4682, static)
- [ ] x_dnd_do_drop  (line 4716, static)
- [ ] x_set_dnd_targets  (line 4729, static)
- [ ] x_free_dnd_targets  (line 4744, static)
- [ ] x_clear_dnd_variables  (line 4758, static)
- [ ] x_free_dnd_toplevels  (line 4765, static)
- [ ] x_restore_events_after_dnd  (line 4779, static)
- [ ] x_dnd_cleanup_drag_and_drop  (line 4867, static)
- [ ] x_dnd_note_self_position  (line 4938, static)
- [ ] x_dnd_note_self_wheel  (line 4959, static)
- [ ] x_dnd_note_self_drop  (line 4987, static)
- [ ] x_flush  (line 5074, static)
- [ ] x_toolkit_position  (line 5160, static)
- [ ] x_update_opaque_region  (line 5197, static)
- [ ] x_set_clip_rectangles  (line 6479, static)
- [ ] x_reset_clip_rectangles  (line 6495, static)
- [ ] x_fill_rectangle  (line 6571, static)
- [ ] x_clear_rectangle  (line 6685, static)
- [ ] x_draw_rectangle  (line 6733, static)
- [ ] x_clear_window  (line 6751, static)
- [ ] x_dpyinfo  (line 6883)
- [ ] x_display_info_for_display  (line 6895)
- [ ] x_find_topmost_parent  (line 6907, static)
- [ ] x_set_frame_alpha  (line 6932, static)
- [ ] x_update_begin  (line 7419, static)
- [ ] x_draw_vertical_window_border  (line 7443, static)
- [ ] x_draw_window_divider  (line 7464, static)
- [ ] x_flip_and_flush  (line 7576, static)
- [ ] x_update_end  (line 7604, static)
- [ ] XTframe_up_to_date  (line 7631, static)
- [ ] x_clear_under_internal_border  (line 7696)
- [ ] x_after_update_window_line  (line 7752, static)
- [ ] x_premultiply_pixel  (line 7821, static)
- [ ] x_draw_fringe_bitmap  (line 7852, static)
- [ ] x_display_set_last_user_time  (line 8087, static)
- [ ] x_create_special_window  (line 8206, static)
- [ ] x_update_frame_user_time_window  (line 8226, static)
- [ ] x_set_last_user_time_from_lisp  (line 8303)
- [ ] x_set_cursor_gc  (line 8314, static)
- [ ] x_set_mouse_face_gc  (line 8369, static)
- [ ] x_set_mode_line_face_gc  (line 8409, static)
- [ ] x_set_glyph_string_gc  (line 8420, static)
- [ ] x_set_glyph_string_clipping  (line 8462, static)
- [ ] x_set_glyph_string_clipping_exactly  (line 8478, static)
- [ ] x_compute_glyph_string_overhangs  (line 8496, static)
- [ ] x_clear_glyph_string_rect  (line 8530, static)
- [ ] x_clear_point  (line 8538, static)
- [ ] x_draw_glyph_string_background  (line 8568, static)
- [ ] x_draw_glyph_string_foreground  (line 8612, static)
- [ ] x_draw_composite_glyph_string_foreground  (line 8706, static)
- [ ] x_draw_glyphless_glyph_string_foreground  (line 8816, static)
- [ ] x_color_cells  (line 9104, static)
- [ ] x_query_colors  (line 9137)
- [ ] x_query_frame_background_color  (line 9205, static)
- [ ] x_hash_string_ignore_case  (line 9239, static)
- [ ] x_parse_color  (line 9260)
- [ ] x_alloc_nearest_color_1  (line 9379, static)
- [ ] x_alloc_nearest_color  (line 9521)
- [ ] x_copy_color  (line 9545)
- [ ] x_alloc_lighter_color  (line 9591, static)
- [ ] x_setup_relief_color  (line 9668, static)
- [ ] x_setup_relief_colors  (line 9713, static)
- [ ] x_fill_triangle  (line 9746, static)
- [ ] x_make_point  (line 9760, static)
- [ ] x_inside_rect_p  (line 9771, static)
- [ ] x_draw_relief_rect  (line 9797, static)
- [ ] x_draw_box_rect  (line 9999, static)
- [ ] x_draw_glyph_string_box  (line 10040, static)
- [ ] x_composite_image  (line 10109, static)
- [ ] x_draw_image_foreground  (line 10166, static)
- [ ] x_draw_image_relief  (line 10289, static)
- [ ] x_draw_image_foreground_1  (line 10383, static)
- [ ] x_draw_glyph_string_bg_rect  (line 10465, static)
- [ ] x_draw_image_glyph_string  (line 10496, static)
- [ ] x_draw_stretch_glyph_string  (line 10636, static)
- [ ] x_get_scale_factor  (line 10747, static)
- [ ] x_draw_underwave  (line 10774, static)
- [ ] x_draw_dash  (line 10850, static)
- [ ] x_fill_underline  (line 10900, static)
- [ ] x_draw_glyph_string  (line 10940, static)
- [ ] x_shift_glyphs_for_insert  (line 11309, static)
- [ ] x_delete_glyphs  (line 11324, static)
- [ ] x_clear_area1  (line 11335, static)
- [ ] x_clear_area  (line 11344)
- [ ] x_clear_frame  (line 11404, static)
- [ ] x_send_hourglass_message  (line 11427, static)
- [ ] x_show_hourglass  (line 11449, static)
- [ ] x_hide_hourglass  (line 11531, static)
- [ ] XTflash  (line 11551, static)
- [ ] XTring_bell  (line 11657, static)
- [ ] x_ins_del_lines  (line 11696, static)
- [ ] x_scroll_run  (line 11705, static)
- [ ] x_frame_highlight  (line 11925, static)
- [ ] x_frame_unhighlight  (line 11952, static)
- [ ] x_new_focus_frame  (line 11990, static)
- [ ] make_invisible_cursor  (line 12068, static)
- [ ] x_toggle_visible_pointer  (line 12100, static)
- [ ] XTtoggle_invisible_pointer  (line 12142, static)
- [ ] x_focus_changed  (line 12163, static)
- [ ] x_window_to_frame  (line 12232, static)
- [ ] x_tooltip_window_to_frame  (line 12285, static)
- [ ] x_top_window_to_frame  (line 12489)
- [ ] x_next_event_from_any_display  (line 12495, static)
- [ ] x_handle_pending_selection_requests_1  (line 12543, static)
- [ ] x_handle_pending_selection_requests  (line 12559)
- [ ] x_push_selection_request  (line 12573, static)
- [ ] x_detect_pending_selection_requests  (line 12584)
- [ ] x_clear_dnd_action  (line 12590, static)
- [ ] x_dnd_delete_action_list  (line 12597, static)
- [ ] x_dnd_lose_ownership  (line 12620, static)
- [ ] x_dnd_process_quit  (line 12636, static)
- [ ] x_dnd_begin_drag_and_drop  (line 12703)
- [ ] x_detect_focus_change  (line 13980, static)
- [ ] x_mouse_leave  (line 14039)
- [ ] XTframe_rehighlight  (line 14095, static)
- [ ] x_frame_rehighlight  (line 14101, static)
- [ ] x_find_modifier_meanings  (line 14135, static)
- [ ] x_x_to_emacs_modifiers  (line 14309)
- [ ] x_emacs_to_x_modifiers  (line 14338)
- [ ] get_keysym_name  (line 14371)
- [ ] x_compute_root_window_offset  (line 14389, static)
- [ ] x_translate_coordinates  (line 14404)
- [ ] x_translate_coordinates_to_root  (line 14444)
- [ ] x_handle_translate_coordinates  (line 14481)
- [ ] x_query_pointer_1  (line 14537, static)
- [ ] x_query_pointer  (line 14611)
- [ ] x_construct_mouse_click  (line 14668, static)
- [ ] x_note_mouse_movement  (line 14739, static)
- [ ] x_get_window_below  (line 14787, static)
- [ ] x_fast_mouse_position  (line 14862, static)
- [ ] XTmouse_position  (line 15001, static)
- [ ] x_window_to_scroll_bar  (line 15343, static)
- [ ] x_scroll_bar_create  (line 16715, static)
- [ ] x_scroll_bar_set_handle  (line 16843, static)
- [ ] x_scroll_bar_remove  (line 16964, static)
- [ ] XTset_vertical_scroll_bar  (line 16995, static)
- [ ] XTset_horizontal_scroll_bar  (line 17120, static)
- [ ] XTcondemn_scroll_bars  (line 17262, static)
- [ ] XTredeem_scroll_bar  (line 17288, static)
- [ ] XTjudge_scroll_bars  (line 17372, static)
- [ ] x_scroll_bar_handle_exposure  (line 17402, static)
- [ ] x_scroll_bar_redraw  (line 17459, static)
- [ ] x_scroll_bar_handle_click  (line 17503, static)
- [ ] x_scroll_bar_note_movement  (line 17606, static)
- [ ] x_scroll_bar_report_motion  (line 17638, static)
- [ ] x_horizontal_scroll_bar_report_motion  (line 17707, static)
- [ ] x_scroll_bar_clear  (line 17777, static)
- [ ] x_net_wm_state  (line 18011, static)
- [ ] x_flush_dirty_back_buffer_on  (line 18043, static)
- [ ] mouse_or_wdesc_frame  (line 18095, static)
- [ ] x_dnd_compute_tip_xy  (line 18119, static)
- [ ] x_dnd_update_tooltip_position  (line 18190, static)
- [ ] x_dnd_update_tooltip_now  (line 18229, static)
- [ ] x_dnd_update_state  (line 18264, static)
- [ ] x_display_pixel_height  (line 18482)
- [ ] x_display_pixel_width  (line 18491)
- [ ] x_wait_for_cell_change  (line 18502)
- [ ] x_find_monitors_changed_event  (line 18619, static)
- [ ] x_coords_from_dnd_message  (line 18685, static)
- [ ] x_handle_wm_state  (line 18764, static)
- [ ] handle_one_xevent  (line 18876, static)
- [ ] x_dispatch_event  (line 25645, static)
- [ ] XTread_socket  (line 25672, static)
- [ ] x_clip_to_row  (line 25831, static)
- [ ] x_draw_hollow_cursor  (line 25857, static)
- [ ] x_draw_bar_cursor  (line 25913, static)
- [ ] x_define_frame_cursor  (line 26020, static)
- [ ] x_clear_frame_area  (line 26032, static)
- [ ] x_draw_window_cursor  (line 26041, static)
- [ ] x_bitmap_icon  (line 26104, static)
- [ ] x_text_icon  (line 26194)
- [ ] x_is_serial_more_than  (line 26261, static)
- [ ] x_is_serial_more_than_or_equal_to  (line 26270, static)
- [ ] x_is_serial_less_than  (line 26279, static)
- [ ] x_is_serial_less_than_or_equal_to  (line 26288, static)
- [ ] x_find_error_handler  (line 26297, static)
- [ ] x_unwind_errors_to  (line 26317)
- [ ] x_error_catcher  (line 26331, static)
- [ ] x_catch_errors_with_handler  (line 26373)
- [ ] x_catch_errors  (line 26390)
- [ ] x_request_can_fail  (line 26398, static)
- [ ] x_clean_failable_requests  (line 26421, static)
- [ ] x_ignore_errors_for_next_request  (line 26464)
- [ ] x_stop_ignoring_errors  (line 26520)
- [ ] x_uncatch_errors_after_check  (line 26551)
- [ ] x_uncatch_errors  (line 26568)
- [ ] x_check_errors  (line 26614)
- [ ] x_had_errors_p  (line 26651)
- [ ] x_clear_errors  (line 26680)
- [ ] x_try_restore_frame  (line 26729, static)
- [ ] x_connection_closed  (line 26744, static)
- [ ] x_error_handler  (line 27000, static)
- [ ] x_error_quitter  (line 27082, static)
- [ ] x_io_error_quitter  (line 27115, static)
- [ ] x_new_font  (line 27137, static)
- [ ] x_calc_absolute_position  (line 27497, static)
- [ ] x_set_offset  (line 27590, static)
- [ ] x_get_wm_check_window  (line 27692, static)
- [ ] x_wm_supports_1  (line 27728)
- [ ] x_wm_supports  (line 27826)
- [ ] set_wm_state  (line 27833, static)
- [ ] x_set_sticky  (line 27858)
- [ ] x_set_shaded  (line 27870)
- [ ] x_set_skip_taskbar  (line 27892)
- [ ] x_set_z_group  (line 27927)
- [ ] x_get_current_wm_state  (line 27978, static)
- [ ] do_ewmh_fullscreen  (line 28090, static)
- [ ] XTfullscreen_hook  (line 28222, static)
- [ ] x_handle_net_wm_state  (line 28234, static)
- [ ] x_check_fullscreen  (line 28270, static)
- [ ] x_check_expected_move  (line 28340, static)
- [ ] x_sync_with_move  (line 28384, static)
- [ ] x_wait_for_event  (line 28431)
- [ ] x_set_window_size_1  (line 28484, static)
- [ ] x_suspend_background_fills  (line 28565, static)
- [ ] x_restore_background_fills  (line 28574, static)
- [ ] x_set_window_size  (line 28587)
- [ ] x_set_window_size_and_position_1  (line 28624, static)
- [ ] x_set_window_size_and_position  (line 28666)
- [ ] frame_set_mouse_pixel_position  (line 28705)
- [ ] x_raise_frame  (line 28731, static)
- [ ] x_lower_frame_1  (line 28746, static)
- [ ] x_lower_frame  (line 28779, static)
- [ ] XTframe_raise_lower  (line 28801, static)
- [ ] xembed_request_focus  (line 28812, static)
- [ ] server_timestamp_predicate  (line 28822, static)
- [ ] x_get_server_time  (line 28839, static)
- [ ] x_ewmh_activate_frame  (line 28882, static)
- [ ] x_get_focus_frame  (line 28950, static)
- [ ] x_set_input_focus  (line 28964, static)
- [ ] x_focus_frame  (line 29024, static)
- [ ] xembed_set_info  (line 29151, static)
- [ ] xembed_send_message  (line 29166, static)
- [ ] x_make_frame_visible  (line 29202)
- [ ] x_make_frame_invisible  (line 29407)
- [ ] x_make_frame_visible_invisible  (line 29475, static)
- [ ] x_create_font_cursor  (line 29484)
- [ ] x_iconify_frame  (line 29498)
- [ ] x_free_frame_resources  (line 29620)
- [ ] x_destroy_window  (line 29857, static)
- [ ] x_intern_cached_atom  (line 29897)
- [ ] x_get_atom_name  (line 29984)
- [ ] x_intern_atoms  (line 30084)
- [ ] x_embed_frame  (line 30129)
- [ ] x_wm_set_size_hint  (line 30159)
- [ ] x_wm_set_window_state  (line 30345, static)
- [ ] x_wm_set_icon_pixmap  (line 30363, static)
- [ ] x_wm_set_icon_position  (line 30411)
- [ ] x_free_pixmap  (line 30448, static)
- [ ] same_x_server  (line 30496, static)
- [ ] get_bits_and_offset  (line 30546, static)
- [ ] x_display_ok  (line 30571)
- [ ] x_term_init  (line 30668)
- [ ] x_delete_selection_requests  (line 31846, static)
- [ ] x_delete_display  (line 31873, static)
- [ ] x_delete_terminal  (line 32064)
- [ ] x_create_terminal  (line 32233, static)
- [ ] x_initialize  (line 32306, static)
- [ ] init_xterm  (line 32394)
- [ ] mark_xterm  (line 32420)
- [ ] x_catch_errors_for_lisp  (line 32476)
- [ ] x_check_errors_for_lisp  (line 32485)
- [ ] x_uncatch_errors_for_lisp  (line 32493)
- [ ] x_preserve_selections  (line 32506)
- [ ] x_get_keyboard_modifiers  (line 32643)
- [ ] syms_of_xterm  (line 32665)

## Macros defined — 21
- [x] ATOM_REFS_INIT  (line 989, body 38ea51a9ed88)
- [ ] HIGHLIGHT_COLOR_DARK_BOOST_LIMIT  (line 9580, body bdbb3f155244)
- [ ] OPAQUE  (line 6929, body d524558178b7)
- [ ] STORE_KEYSYM_FOR_DEBUG  (line 17826, body faabc0e0dbea)
- [x] SWAPCARD16  (line 1584, body f50347531e4d)
- [x] SWAPCARD32  (line 1583, body 393d2f43a49a)
- [ ] XEMBED_VERSION  (line 29148, body b6589fc6ab0d)
- [x] XM_DRAG_OPERATION_IS_LINK  (line 1706, body 19c218f1041c)
- [x] XM_DRAG_PROTOCOL_VERSION  (line 1761, body b6589fc6ab0d)
- [x] XM_DRAG_REASON  (line 1716, body 9456c43f5e20)
- [x] XM_DRAG_REASON_CODE  (line 1718, body 7810497b8cdc)
- [x] XM_DRAG_REASON_ORIGINATOR  (line 1717, body 49d23216e03e)
- [x] XM_DRAG_SIDE_EFFECT  (line 1687, body fcd0aa2b3860)
- [x] XM_DRAG_SIDE_EFFECT_DROP_ACTION  (line 1695, body d8acb947f1f0)
- [x] XM_DRAG_SIDE_EFFECT_OPERATION  (line 1692, body af8a320359fb)
- [x] XM_DRAG_SIDE_EFFECT_SITE_STATUS  (line 1693, body cc712b03bf55)
- [x] XM_DRAG_STYLE_IS_DROP_ONLY  (line 1746, body 8a5d0800d81c)
- [x] XM_DRAG_STYLE_IS_DYNAMIC  (line 1748, body ff5b4bd375aa)
- [ ] X_DND_SUPPORTED_VERSION  (line 3626, body ac3478d69a3c)
- [x] X_NEXT_KBD_EVENT  (line 888, body 1bf69f7be3e8)
- [ ] x_any_window_to_frame  (line 12486, body c8c88ba3cad5)

## Types and their fields — 14
- [ ] x_atom_ref: const char* name, int offset
- [ ] x_client_list_window: Window window, Display* dpy, int x, int y, int width, int height, long previous_event_mask, unsigned long wm_state, struct x_client_list_window* next, int frame_extents_left, int frame_extents_right, int frame_extents_top, int frame_extents_bottom, uint8_t xm_protocol_style, bool mapped_p
- [ ] x_error_message_stack: char* string, Display* dpy, x_special_error_handler handler, void* handler_data, struct x_error_message_stack* prev, unsigned long first_request
- [ ] x_selection_request_event: struct selection_input_event se, struct x_selection_request_event* next
- [ ] xm_drag_initiator_info: uint8_t byteorder, uint8_t protocol, uint16_t table_index, uint32_t selection
- [ ] xm_drag_motion_message: uint8_t reason, uint8_t byteorder, uint16_t side_effects, uint32_t timestamp, uint16_t x, uint16_t y
- [ ] xm_drag_motion_reply: uint8_t reason, uint8_t byte_order, uint16_t side_effects, uint32_t timestamp, uint16_t better_x, uint16_t better_y
- [ ] xm_drag_receiver_info: uint8_t byteorder, uint8_t protocol, uint8_t protocol_style, uint8_t unspecified0, uint32_t unspecified1, uint32_t unspecified2, uint32_t unspecified3
- [ ] xm_drop_start_message: uint8_t reason, uint8_t byte_order, uint16_t side_effects, uint32_t timestamp, uint16_t x, uint16_t y, uint32_t index_atom, uint32_t source_window
- [ ] xm_drop_start_reply: uint8_t reason, uint8_t byte_order, uint16_t side_effects, uint16_t better_x, uint16_t better_y
- [ ] xm_targets_table_header: uint8_t byte_order, uint8_t protocol, uint16_t target_list_count, uint32_t total_data_size
- [ ] xm_targets_table_rec: uint16_t n_targets, uint32_t targets
- [ ] xm_top_level_enter_message: uint8_t reason, uint8_t byteorder, uint16_t zero, uint32_t timestamp, uint32_t source_window, uint32_t index_atom
- [ ] xm_top_level_leave_message: uint8_t reason, uint8_t byteorder, uint16_t zero, uint32_t timestamp, uint32_t source_window

## File-scope objects — 80
- [x] static bool any_help_event_p  (line 821, static)
- [x] struct x_display_info x_display_list  (line 826)
- [x] static Time ignore_next_mouse_click_timeout  (line 843, static)
- [x] static struct x_display_info mouse_click_timeout_display  (line 847, static)
- [x] static int x_noop_count  (line 851, static)
- [x] static int x_use_pending_selection_requests  (line 885, static)
- [x] static struct x_selection_request_event pending_selection_requests  (line 974, static)
- [x] bool x_dnd_in_progress  (line 1217)
- [x] struct frame x_dnd_frame  (line 1220)
- [x] struct frame x_dnd_finish_frame  (line 1225)
- [x] bool x_dnd_waiting_for_finish  (line 1231)
- [x] static bool x_dnd_run_unsupported_drop_function  (line 1236, static)
- [x] static Time x_dnd_unsupported_drop_time  (line 1239, static)
- [x] static Window x_dnd_unsupported_drop_window  (line 1242, static)
- [x] static Lisp_Object x_dnd_unsupported_drop_data  (line 1245, static)
- [x] static bool x_dnd_update_tooltip  (line 1249, static)
- [x] static Lisp_Object x_dnd_monitors  (line 1252, static)
- [x] static Display x_dnd_finish_display  (line 1256, static)
- [x] static int x_dnd_waiting_for_motif_finish  (line 1267, static)
- [x] static struct x_display_info x_dnd_waiting_for_motif_finish_display  (line 1271, static)
- [x] static bool x_dnd_xm_use_help  (line 1277, static)
- [x] static bool x_dnd_motif_setup_p  (line 1280, static)
- [x] static Atom x_dnd_motif_atom  (line 1283, static)
- [x] static Window x_dnd_pending_finish_target  (line 1287, static)
- [x] static int x_dnd_waiting_for_finish_proto  (line 1290, static)
- [x] static bool x_dnd_allow_current_frame  (line 1294, static)
- [x] static bool x_dnd_init_type_lists  (line 1298, static)
- [x] static int x_dnd_return_frame  (line 1305, static)
- [x] static struct frame x_dnd_return_frame_object  (line 1309, static)
- [x] static Window x_dnd_last_seen_window  (line 1314, static)
- [x] static Window x_dnd_last_seen_toplevel  (line 1317, static)
- [x] static Window x_dnd_end_window  (line 1321, static)
- [x] static int x_dnd_last_protocol_version  (line 1325, static)
- [x] static bool x_dnd_last_window_is_frame  (line 1329, static)
- [x] static int x_dnd_last_motif_style  (line 1337, static)
- [x] static Time x_dnd_selection_timestamp  (line 1341, static)
- [x] static Window x_dnd_mouse_rect_target  (line 1344, static)
- [x] static XRectangle x_dnd_mouse_rect  (line 1348, static)
- [x] static Window x_dnd_waiting_for_status_window  (line 1352, static)
- [x] static XEvent x_dnd_pending_send_position  (line 1356, static)
- [x] static bool x_dnd_pending_send_position_button  (line 1359, static)
- [x] static int x_dnd_pending_send_position_root_x  (line 1362, static)
- [x] static int x_dnd_pending_send_position_root_y  (line 1365, static)
- [x] static bool x_dnd_need_send_drop  (line 1369, static)
- [x] static int x_dnd_send_drop_proto  (line 1372, static)
- [x] static Atom x_dnd_action  (line 1384, static)
- [x] static Lisp_Object x_dnd_action_symbol  (line 1388, static)
- [x] static Atom x_dnd_wanted_action  (line 1393, static)
- [x] static uint8_t x_dnd_motif_operations  (line 1397, static)
- [x] static uint8_t x_dnd_first_motif_operation  (line 1401, static)
- [x] static Atom x_dnd_targets  (line 1404, static)
- [x] static int x_dnd_n_targets  (line 1407, static)
- [x] static XWindowAttributes x_dnd_old_window_attrs  (line 1413, static)
- [x] static bool x_dnd_unwind_flag  (line 1417, static)
- [x] static struct frame x_dnd_movement_frame  (line 1420, static)
- [x] static int x_dnd_movement_x  (line 1424, static)
- [x] static int x_dnd_movement_y  (line 1424, static)
- [x] static struct frame x_dnd_wheel_frame  (line 1427, static)
- [x] static int x_dnd_wheel_x  (line 1430, static)
- [x] static int x_dnd_wheel_y  (line 1430, static)
- [x] static int x_dnd_wheel_button  (line 1433, static)
- [x] static int x_dnd_wheel_state  (line 1436, static)
- [x] static Time x_dnd_wheel_time  (line 1439, static)
- [x] static sigjmp_buf x_dnd_disconnect_handler  (line 1448, static)
- [x] static bool x_dnd_inside_handle_one_xevent  (line 1452, static)
- [x] static int x_dnd_recursion_depth  (line 1456, static)
- [x] static Lisp_Object x_dnd_selection_alias_cell  (line 1461, static)
- [x] static int x_dnd_last_tooltip_x  (line 1464, static)
- [x] static int x_dnd_last_tooltip_y  (line 1464, static)
- [x] static bool x_dnd_last_tooltip_valid  (line 1467, static)
- [x] static struct x_client_list_window x_dnd_toplevels  (line 1536, static)
- [x] static bool x_dnd_use_toplevels  (line 1540, static)
- [x] static volatile bool xm_drag_window_error  (line 1548, static)
- [ ] static struct x_display_info XTread_socket_fake_io_error  (line 17833, static)
- [ ] static struct x_display_info next_noop_dpyinfo  (line 17840, static)
- [ ] static struct x_error_message_stack x_error_message  (line 26249, static)
- [ ] int x_error_message_count  (line 26252)
- [ ] static char error_msg  (line 26722, static)
- [ ] static bool x_initialized  (line 30491, static)
- [ ] static unsigned x_display_id  (line 30595, static)

## Decisions

### 1. What this lane is for (and what it is not)

`src/xterm.c` is the X11 backend.  It is not a member of the frozen build, so its port is
**text carried for an X11 target** - a 1:1 transcript of the reference file (PORTING.md P-42: the
scope is the reference tree's own files, not the frozen link list; the same rule that made
`vm-limit.rs` a 1:1 port of a file this configuration does not link).  Two consequences have to
be said out loud, because they are easy to get wrong later:

* **The final Lisp test suite never reaches this file.**  The frozen darwin-arm64 build has no
  window system, so no code path in it calls anything here; a green or red ERT run says nothing
  about this unit either way.  Its evidence is the gate readings below and, one day, a build of
  the X11 target - not a test result from this repository.
* **It is not a compilation unit here.**  Everything the slice names that lives in X11 headers
  (`Display`, `XErrorEvent`, `XtAppContext`, the `XfooMask` constants, ...) has no Rust
  counterpart in this tree, so with `have_x_windows` on the mirrors themselves do not compile
  (140 of the 144 errors the probe in Evidence measures are in `include/xterm.rs` and friends,
  and every one of them is an X11-header name).  That is exactly the state
  `femacs/src/include/xterm.rs` is in, and why `xtask build-tu src/xterm.c` proves parsing, not
  typing.

### 2. Which configuration counts as "live": measured, and what I recommend

The frozen macro table does decide this file (it is in
`docs/baseline/config/cond-eval-verified.txt`, `tools/check-cond-eval.py src/xterm.c` ->
`852 conditions, 0 unknown, 0 disagreements [VERIFIED]`), but the configuration it decides is
one that **can never exist**: `HAVE_X_WINDOWS` is undef, and a build with no window system never
compiles `xterm.c` at all.  The three readings, measured with `tools/cinventory.py` (script in
Evidence):

| reading | functions | macros | constants | file-scope objects | record fields | total demanded | C lines live |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| frozen table (what the gate uses today) | 310 | 21 | 48 | 80 | 78 | 537 | 19,809 |
| A: an X11/GTK3 table (the file's own platform) | 389 | 32 | 61 | 99 | 86 | 667 | 26,914 |
| B: blind - every arm of every `#if` | 433 | 35 | 62 | 105 | 86 | 721 | 33,006 |

Route A needs a *new mechanism* (a per-file condition table: `tools/gen-cond-macros.py` and
`tools/check-cond-eval.py` today key the table on one target, and `cond-eval-verified.txt` is
per-file but per-*target*); route B needs nothing new (one line out of
`cond-eval-verified.txt`, i.e. the file stops consulting the table and every arm is demanded).

**Recommendation: neither, for now - keep the frozen reading and carry the arms anyway (P-51),
which is what the header mirror of this same file already does.**  Reasons, in order of weight:

1. **Symmetric with the mirror.**  `femacs/src/include/xterm.rs` is already the shape "whole
   body under `have_x_windows`, checklist from the frozen reading, arms outside it recorded in
   the ledgers".  Two readings for one subsystem would be worse than either.
2. **The checklist is a floor, not the scope.**  The scope is P-42's 1:1 text, and the lane
   carries the arms that the frozen reading leaves out (16 of them in slice 1 alone, listed in
   Decision 4) - so the work happens regardless of which reading is chosen; only the *mandatory*
   set moves.  Route A would in fact *under*-count too: it drops the Lucid/Motif/Xaw/Xt arms
   (59 names), which are also part of the 1:1 text.
3. **Cost.**  Route B's only real price is that `--check` compares arms that can never coexist;
   that is a text comparison, and `macro_duplicates`/cfg make it harmless.  Route A's price is a
   tool change plus a second `check-cond-eval` verification pass under a configuration that
   cannot be built here.

If you want the checklist to *cover* the X11-only arms, the cheapest correct move is route B
(blind) - it needs no tooling and its 721-item checklist is exactly the union of C's arms.  I am
not asking for it this round because it would change a shared file
(`docs/baseline/config/cond-eval-verified.txt`) and the printed "424 files" of every gate run;
say the word and the next round ships it.

### 3. The conditions slice 1 crosses, with C line numbers

| C lines | condition | arm taken here | how the port carries it |
| --- | --- | --- | --- |
| 625-728 | the header preamble, incl. 639-641 `HAVE_X_I18N`, 643-646 `USE_XCB`, 649-683 the X11 extension headers, 687-689 `#ifndef makedev`, 723-726 / 730-735 / 737-739 / 741-743 / 745-748 / 750-779 / 781-783 the toolkit headers | all dead (the file is not compiled) | includes only, in C's order; the headers with no mirror are listed in Decision 6 |
| 766-768 | `#ifndef XtNpickTop` inside `USE_X_TOOLKIT` + `USE_TOOLKIT_SCROLL_BARS` + `#else` of `USE_MOTIF` | dead | `macro_rules! XtNpickTop` under `use_x_toolkit` + `use_toolkit_scroll_bars` + `not (use_motif)` |
| 774-778 | `#ifndef XtNinitialState` inside `USE_X_TOOLKIT` | dead | `macro_rules! XtNinitialState` under `use_x_toolkit` |
| 797-799 | `#if defined USE_XCB && defined USE_CAIRO_XCB` | dead | empty-body macro under `use_xcb` + `use_cairo_xcb` |
| 801-805 | `#if XCB_SHAPE_MAJOR_VERSION > 1 \|\| (... >= 1)` | the three version macros are unknown here, so C's own answer is "false" | empty-body macro under the derived name `have_xcb_shape_input_rects` |
| 807-816 | `#ifdef USE_GTK` - `xg_pending_quit_event` | dead | gated `use_gtk`, carried (P-51) |
| 828-837 | `#ifdef USE_X_TOOLKIT` - `Xt_app_con`, `Xt_default_resources`, `toolkit_scroll_bar_interaction` | dead | gated `use_x_toolkit`, carried |
| 853-856 | `#ifdef USE_GTK` - `xg_default_icon_file` | dead | gated `use_gtk`, carried |
| 858-861 | `#ifdef HAVE_X_I18N` - `emacs_class` | dead | gated `have_x_i18n`, carried; `EMACS_CLASS` is frame.h 1840's `"Emacs"` |
| 863-867 | `#ifdef USE_GTK` - `current_count`/`current_finish`/`current_hold_quit` | dead | gated `use_gtk`, carried |
| 869-881 | `#ifdef HAVE_XINPUT2` + three `#ifndef` request codes | dead | three numeric macros under `have_xinput2`, carried |
| 891 | `static void x_push_selection_request (...)` - a forward declaration | live as text | not an item in Rust (no prototypes); the definition is C 12573, a later slice |
| 900-959 | the three selection-request functions | **live** under the frozen reading (all three are checklist rows) | transcribed; the bitfield read uses the per-unit `input_event_bits!` (PORTING.md P-80) |

`#ifndef` guards (766, 776, 870, 874, 878) test whether *X11's own headers* already define the
name.  The port cannot see those headers, so each arm is carried under its enclosing features and
the guard itself is recorded here rather than modelled.

### 4. What slice 1 carries that the checklist does not demand

Sixteen items are carried under their features although the frozen reading calls them dead
(P-51; the ledger's boxes are that reading's, so these have no box): `XtNpickTop` (C 767),
`XtNinitialState` (777), `USE_CAIRO_XCB_SURFACE` (798), `HAVE_XCB_SHAPE_INPUT_RECTS` (804),
`X_XIGrabDevice`/`X_XIUngrabDevice`/`X_XIAllowEvents` (871/875/879), `xg_pending_quit_event`
(815), `Xt_app_con` (831), `Xt_default_resources` (832), `toolkit_scroll_bar_interaction` (835),
`xg_default_icon_file` (855), `emacs_class` (860), `current_count`/`current_finish`/
`current_hold_quit` (864-866).

### 5. Gaps slice 1 exposes (each is a reading, none is a stub)

* `buffered_input_event.sie` (C `event->sie`, xterm.c 917/920): keyboard.h 228-235 puts that
  member inside `#if defined HAVE_X11 || defined HAVE_PGTK`, and `include/keyboard.rs` carries
  that arm **as a comment only** (its 66-68).  So C 917's `&event->sie` has no field to name
  until the keyboard.h mirror grows the arm.  That mirror is another lane's file; the request is
  in this round's report, and the two E0609s are in Evidence.
* `x_handle_pending_selection_requests` (xterm.c 958): xterm.h 1857 declares it and
  `include/xterm.rs` does not carry the declaration (it appears only inside the
  `DEFER_SELECTIONS` macro at the mirror's 1271).  Request in the report.
* `x_push_selection_request` (xterm.c 917): defined at xterm.c 12573, a later slice; Rust
  resolves items by name, so the call is correct as written and the name simply does not exist
  yet.
* The `x_atom_refs` table (C 987-1130, 122 entries) needs `struct x_display_info`'s 115
  `Xatom_*` members; the mirror carries 61 of them (the rest are inside xterm.h's conditional
  arms).  It opens slice 2, together with the table's `offsetof` use.

### 6. Headers slice 1 has no mirror for

C 637 `<X11/cursorfont.h>`; 644-645 xcb; 650-683 the X11 extension headers (Xfixes, Xdbe,
XInput2, Xrandr, sync, Xinerama, Xcomposite, shape) and 682's xcb/shape; 691 `<sys/ioctl.h>`;
724-725 `<X11/Shell.h>`/`ShellP.h`; 733 `<X11/Xproto.h>`; 738 `../lwlib/xlwmenu.h`;
742 `<cairo-xlib.h>`; 746-747 and 755 the Xm headers; 759-764 the Xaw/Xaw3d headers;
785 `bitmaps/gray.xbm`; 788 `<X11/XKBlib.h>`.  The X11 ones are the X11 target's own headers -
they are not this tree's to mirror - and `<sys/ioctl.h>` is a system header with no gnulib
mirror.  Every header that *does* have a mirror is included at C's position.

### 7. Feature names slice 1 needs that `xtask`'s `[features]` table does not declare yet

`use_toolkit_scroll_bars` (C 753), `use_motif` (C 754/745), `use_xcb` (C 643/797),
`use_cairo_xcb` (C 797), `have_xcb_shape_input_rects` (C 804).  The rest of the names slice 1
uses (`have_x_windows`, `use_x_toolkit`, `use_gtk`, `have_xinput2`, `have_x_i18n`) are already
in the table from the xterm.h mirror's round.  Requested in this round's report.

### 8. Slice 2 (C 1131-1762): what it landed, and the three shapes worth recording

Slice 2 is the drag-and-drop state block (C 1214-1548), `struct x_client_list_window`
(1478-1533), the Motif message layouts (1550-1685) and the protocol's bit packers
(1687-1761): **66 file-scope objects + 13 macros**, all ticked, plus 3 enums, 1 typedef'd
enum, 9 records and 2 XEmbed enums carried.  The conditions it crosses - with C line numbers -
are `HAVE_XSHAPE` (1511-1526, the two shape rectangles inside `x_client_list_window`),
`HAVE_XKB` (1441-1444, `x_dnd_keyboard_state`), `HAVE_XINPUT2` (1469-1476, the two device ids),
`ENABLE_CHECKING` (1561-1585, the `SWAPCARD*` arm) and `WORDS_BIGENDIAN` (1554-1558,
`XM_BYTE_ORDER_CUR_FIRST`); each is carried under the lowercased feature name, dead arms
included (P-51), so nothing in the range is dropped.

Three decisions this slice had to make, because C cannot be transcribed literally:

* **X11 names.**  Slice 1 followed P-72 rule 3 for the three Xlib *pointer* typedefs
  (`Display`, `XErrorEvent`, `XtAppContext` -> opaque `*mut c_void` + a `/// C:` line).  Slice 2
  keeps every other X11 name as C spells it - `Window`, `Atom`, `XRectangle`, `XEvent`,
  `XWindowAttributes` - which is what this file's own header mirror does with `*mut XEvent` and
  `XRectangle` (its 457, 605), and what makes the by-value objects (`x_dnd_mouse_rect`,
  `x_dnd_pending_send_position`, `x_dnd_old_window_attrs`) expressible at all: an aggregate by
  value cannot be an opaque pointer, and its storage is C's zeroed static storage, written here
  as `std::mem::zeroed ()` (the `buffer.rs` 166 precedent).  Cost, measured: with
  `have_x_windows` on, the unit's own text has **15** unresolved X11 type names (8 `Window`,
  4 `Atom`, and one each of `XWindowAttributes`/`XRectangle`/`XEvent`) - the same class of
  error the mirror already reports 142 times.  If you would rather have one spelling for the
  whole lane, the alternative is to *name* the X11 types inside the arm (`*mut Display`,
  `Window`, ...) and wait for an X11 header mirror to define them; slice 1's three `*mut c_void`
  lines would then be the ones to change.  Say which you want and the next slice follows it.
* **`XM_BYTE_ORDER_CUR_FIRST` (C 1554-1558).**  With `WORDS_BIGENDIAN` undef, C's third enum
  member has the *same value* as `XM_BYTE_ORDER_LSB_FIRST`, and Rust refuses two variants with
  one discriminant (`E0081`).  The port therefore keeps the two real members in the enum and
  writes C's two spellings of the current order as cfg'd `const`s aliasing whichever member C
  would have made them equal to; the `#else` arm is carried under `words_bigendian`.
* **`SWAPCARD32`/`SWAPCARD16` (C 1561-1585).**  `tools/extract-abi.py`'s `macro_body_of` answers
  for a macro name with the **first** `macro_rules!` in the file, so the frozen arm (`#else`,
  gnulib's `bswap_32`/`bswap_16`) is written **first** and the `ENABLE_CHECKING` arm after it -
  with a comment at the definition saying why, so a later reader does not "restore" C's order and
  break the token check.  The checking arm's bitfield trick is written as a `u32`/`u16` plus a
  `c_char` pointer swap (Rust has no bitfields); it is dead here and carried for the flip.
* **`XM_DRAG_PROTOCOL_VERSION` (C 1759-1761)** is an object-like integer macro, which the
  extractor reads as a *constant* and demands as one: written as `pub const ... : c_int = 0`.

### 9. Deferred by slice 2, and what it needs

**`x_atom_refs` (C 987-1130, 122 entries) is not landed**: its `ATOM_REFS_INIT` rows name
**115** different `Xatom_*` members of `struct x_display_info`, and `femacs/src/include/xterm.rs`
carries **61** of them.  The 58 that are missing are xterm.h's own **unconditional** text at
**xterm.h 772-817** (measured: none of them is inside an `#if` other than the include guard), so
that mirror is simply a WIP slice of its header.  Writing the table now would put 58 names in
the file that nothing defines.  It opens the next slice as soon as the mirror has them (or you
rule that the table is written 1:1 and the names wait), and the request is in this round's
report.

### 10. Feature names slice 2 adds to the request list

`enable_checking` (C 1561), `have_xshape` (C 1511), `have_xkb` (C 1441).  The previous round's
five (`use_toolkit_scroll_bars`, `use_motif`, `use_xcb`, `use_cairo_xcb`,
`have_xcb_shape_input_rects`) are still outstanding too.

### 11. Slice 3 (C 1763-2173): the Motif side-effect map, the targets table and the drag window

Thirteen static functions - `xm_side_effect_from_action`, `xm_operations_from_actions`,
`xm_read_targets_table_header`, `xm_read_targets_table_rec`, `xm_find_targets_table_idx`,
`x_atoms_compare`, `xm_write_targets_table`, `xm_write_drag_initiator_info`,
`xm_drag_window_error_handler`, `xm_drag_window_io_error_handler`, `x_special_window_exists_p`,
`xm_get_drag_window_1`, `xm_get_drag_window` - all ticked.  What the round had to decide, and
what the two measurements (the gate and the X11 probe) caught:

* **`xm_byte_order` was modelled wrongly in slice 2 and is corrected here.**  C 1808 *stores a
  wire byte* into the enum before comparing it (`header_return->byte_order = *byteorder_return =
  *(bytes++)`), so it is a byte holder, not a closed set; it is now `pub type xm_byte_order =
  c_int` plus the three constants (the `dispextern.rs` `glyph_row_area` shape), which also
  dissolves the `E0081` problem slice 2 recorded (two members with one discriminant).
* **`alloca` is the port's stack pool** (P-05): `USE_SAFE_ALLOCA!`/`SAFE_ALLOCA!`/`SAFE_FREE!`
  at C 1867, 1899, 1943 (`bignum.rs` 428 is the shape).
* **Three libc entry points are declared in the unit** because C reaches them through its own
  headers and no mirror carries them - `qsort` (C 2202; `buffer.rs` 132 declares it the same
  way), and `sigsetjmp`/`siglongjmp` (C 2061/1974, whose whole point is that they save the
  signal mask, which C's own `sys_setjmp` spelling does not).  **Measured, not assumed**: a
  program calling all three links, and `nm -u` shows `_qsort`, `_sigsetjmp`, `_siglongjmp` -
  they are functions, not macros, so this is not one of the invented externs of status 3.251.
* **Xlib's own names stay bare** (`XChangeProperty`, `XGetWindowProperty`, `XFree`,
  `XSetErrorHandler`, `XSetIOErrorHandler`, `XSetCloseDownMode`, `XSync`, `XSelectInput`,
  `XCreateWindow`, `XKillClient`, `XGrabServer`, `XUngrabServer`, `XSendEvent`, `XInternAtom`,
  `None`, `Success`, `False`, `True`, `XA_WINDOW`, `PropModeReplace`, `PropModeAppend`,
  `CopyFromParent`, `InputOnly`, `RetainPermanent`, `DestroyAll`, `CWOverrideRedirect`,
  `StructureNotifyMask`), the way this file's own header mirror keeps `KeyPressMask`.  One of
  them is a hazard worth your ruling: **`None` is Rust's prelude `Option::None`**, so the arm
  cannot be a straight flip; the choice is either an Xlib mirror that defines these names, or
  unit-local constants (`pub const None: Window = 0;`).  Same question for `Success`.
* **Six `macro-body-notes.tsv` rows are needed** (requested in this round's report): the six
  functions above whose C signature or body spells `uint8_t`/`uint16_t`/`uint32_t` and/or
  `alloca`, which the port writes `u8`/`u16`/`u32` and the stack pool.  The tree already has
  this shape (`conv_sockaddr_to_lisp`'s `uint16_t` row, `Freplace_region_contents`'s `alloca`
  row).
* **`xm_drag_window_io_error_handler`** is C's `static _Noreturn int`, so the port writes
  `-> !` and adds the `unreachable! ()` Rust's divergence needs (`lread.rs` 720's shape);
  `xm_drag_window_error_handler` keeps C's `int` return.
* Still deferred: the `x_atom_refs` table (C 987-1130) - unchanged from Decision 9.


## Evidence

Readings, in the order they were taken (round 188, 2026-10-04; commands verbatim).

```
$ python3 tools/check-cond-eval.py src/xterm.c
src/xterm.c: 852 conditions, 0 unknown, 0 disagreements [VERIFIED]
```

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs    # before slice 1,
                                                                               # with the file
                                                                               # written as a unit
                                                                               # (one probe `fn`)
460
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
```

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs    # after slice 1
443
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
```

```
$ grep -c '^- ' <that output>          # 443
$ grep -c '^\- \[ \]' docs/ports/xterm.md   # 425 -> 413 after ticking slice 1
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
```

The three readings of Decision 2, from `tools/cinventory.py` with the frozen table, with that
table overridden to an X11/GTK3 set, and with no table at all (blind):

```
frozen  {'defuns': 0, 'functions': 310, 'macros': 21, 'constants': 48, 'globals': 80, 'struct_fields': 78}
routeA  {'defuns': 0, 'functions': 389, 'macros': 32, 'constants': 61, 'globals': 99, 'struct_fields': 86}
blind   {'defuns': 0, 'functions': 433, 'macros': 35, 'constants': 62, 'globals': 105, 'struct_fields': 86}
```

and their live-line counts (`dead_branch_map`):

```
frozen live 19809   routeA live 26914 (adds 9137, loses 2032)   blind live 33006   total 33126
```

The X11 arm's own type check, as a probe rather than a gate (the unit materialised by
`build-tu`, then `rustc --cfg 'feature="have_x_windows"'`):

```
$ rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_x_windows"' -o /tmp/xg188/xterm_probe.o build/femacs/units/xterm/_root.rs
144 errors, 4 of them in the unit's own text:
  1x `x_push_selection_request` not found            (C 12573, a later slice)
  1x `x_handle_pending_selection_requests` not found (xterm.h 1857, gap of Decision 5)
  2x no field `sie` on `buffered_input_event`        (keyboard.h 228-235, gap of Decision 5)
the other 140 are in `include/xterm.rs` and the other mirrors, all unresolved X11-header names
(`KeyPressMask`, `Pixmap`, `Display`, `Screen`, `XVisualInfo`, ...).
```

The probe is what caught two real defects in the first draft of the slice (`build-tu` cannot:
it strips the whole body): the module's statics had to be `pub` for the glob import to reach
them (4x E0425), and C's `event->ie.kind` is a read of `input_event`'s bitfield, so the accessor
takes `&raw mut (*event).ie` rather than the union (E0609).

Slice 2 (C 1131-1762), same readings:

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
324
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
$ grep -c '^- \[ \]' docs/ports/xterm.md        # 425 - 91
334
```

The X11-arm probe again, after slice 2 (same command as above, re-run):

```
159 errors, 19 of them in the unit's own text:
  8x `Window`, 4x `Atom`, 1x `XWindowAttributes`, 1x `XRectangle`, 1x `XEvent`
       (slice 2's kept X11 names - Decision 8)
  2x no field `sie` on `buffered_input_event`          (slice 1's gap, Decision 5)
  1x `x_push_selection_request` not found              (C 12573, a later slice)
  1x `x_handle_pending_selection_requests` not found   (xterm.h 1857, slice 1's gap)
```

The three defects the probe and the gate caught in slice 2's draft, all fixed before landing:
the extractor reads a macro's body from the **first** `macro_rules!` of that name (so
`SWAPCARD32`'s `#else` arm had to come first, or it reported "does not mention bswap_32");
`XM_DRAG_PROTOCOL_VERSION` had to be a `const` (the extractor reads it as a constant); and
`SWAPCARD16`'s live arm takes C's parameter name `l`, not `s` (C 1584).

Slice 3 (C 1763-2173):

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
317
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
  311-316: six "does not mention" rows, one per function, for uint8_t/uint16_t/uint32_t/alloca
           (the six notes rows Decision 11 requests)
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
$ python3 tools/check-link.py
libc-or-platform=257 port-owed=15 rust-runtime=20 unknown=92
384 undefined symbol(s) from 76 program + 87 libgnu object(s), 384 ratchet row(s), 0 failure(s)
$ python3 tools/check-link.py --report | grep port-owed      # all fifteen are xdisp's, not xterm's
_count_lines, _disp_char_vector, _display_tty_menu_item, _get_font_ascent_descent, _init_xdisp,
_invisible_prop, _mark_window_display_accurate, _note_mouse_highlight, _produce_stretch_glyph,
_redisplay_preserve_echo_area, _row_hash, _syms_of_xdisp, _try_window,
_unwind_display_working_on_window, _update_redisplay_ticks
```

The X11-arm probe (the unit materialised by `build-tu`, then `rustc --cfg
'feature="have_x_windows"'`): **107 errors in the unit's own text, and every one is a recorded
gap** - 94 `cannot find <name>` (X11 names, or functions this file defines in a later slice) and
13 `no field <name>` (`x_display_info`'s missing members, `buffered_input_event.sie`); the other
139 are in the mirrors, plus `include/xterm.rs`'s own
`assert! (!cfg!(feature = "have_x_windows") || HAVE_X_WINDOWS)` const assertion, which is the
mirror refusing to be flipped before its target exists.

That probe caught three real defects in this round's draft, none of which `build-tu` can see:
**65 of slice 2's 66 objects were not `pub`** (a `static` in the private module is not reachable
through the glob import - E0425 in 20 places), C 1808's *chained assignment* was written as
`(a = b) as u8` (E0605: Rust has no assignment expression), and `-> !` on
`xm_drag_window_io_error_handler` needs a diverging final expression (E0308).
