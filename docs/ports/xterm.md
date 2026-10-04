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
> before slice 1, **443** after it, **324** after slice 2 (C 1131-1762, the drag-and-drop state
> and the Motif message layouts) and **293** after slice 7 (C 3616-4466, the target-window
> discovery; slices 3-6 are Decisions 11-15, slice 7 is Decision 18); the list below is that
> same frozen reading, which is *not* the configuration that compiles the file (Decisions 2).

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
- [x] xm_setup_dnd_targets  (line 2176, static)
- [x] xm_get_drag_atom_1  (line 2403, static)
- [x] xm_get_drag_atom  (line 2512, static)
- [x] xm_setup_drag_info  (line 2526, static)
- [x] xm_send_drop_message  (line 2563, static)
- [x] xm_send_top_level_enter_message  (line 2588, static)
- [x] xm_send_drag_motion_message  (line 2615, static)
- [x] xm_send_top_level_leave_message  (line 2646, static)
- [x] xm_read_drop_start_reply  (line 2705, static)
- [x] xm_read_drop_start_message  (line 2736, static)
- [x] xm_read_drag_receiver_info  (line 2774, static)
- [x] xm_read_drag_motion_message  (line 2830, static)
- [x] xm_read_drag_motion_reply  (line 2864, static)
- [x] x_dnd_send_xm_leave_for_drop  (line 2897, static)
- [x] x_dnd_free_toplevels  (line 2921, static)
- [x] x_dnd_compute_toplevels  (line 3013, static)
- [x] x_dnd_io_error_handler  (line 3617, static)
- [x] x_dnd_get_target_window_1  (line 3721, static)
- [x] x_dnd_get_wm_state_and_proto  (line 3820, static)
- [x] x_dnd_do_unsupported_drop  (line 3986, static)
- [x] x_dnd_send_unsupported_drop  (line 4082, static)
- [x] x_dnd_fill_empty_target  (line 4128, static)
- [x] x_dnd_get_target_window  (line 4140, static)
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
- [x] X_DND_SUPPORTED_VERSION  (line 3626, body ac3478d69a3c)
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


### 12. Slice 4 (C 2175-2612), and the X11-macro ruling carried out

Six more functions - `xm_setup_dnd_targets` (C's `goto retry_drag_window`, 2192/2224, becomes the
labelled `loop`; C 2316-2319's four `ckd_*` calls keep their short circuit as the first eager
plus block expressions after `||`, the `ccl.rs` 2913 shape), `xm_get_drag_atom_1`,
`xm_get_drag_atom`, `xm_setup_drag_info`, `xm_send_drop_message` and
`xm_send_top_level_enter_message` - all ticked.  Three points:

* **The ruling on X11's macros is implemented in the mirror.**  `femacs/src/include/xterm.rs`
  now has a `pub mod x11_constants` (gated on `have_x_windows`) with the fourteen macros this lane
  names - `None`, `Success`, `CopyFromParent`, `NoEventMask`, `StructureNotifyMask`,
  `ClientMessage`, `InputOnly`, `CWOverrideRedirect`, `DestroyAll`, `RetainPermanent`,
  `PropModeReplace`, `PropModeAppend`, `XA_ATOM`, `XA_WINDOW` - each with the header line its
  value is quoted from.  **The values are measured, not remembered**: the machine has the
  `xorgproto` headers at `/opt/homebrew/include`, so `clang -E -dM` over `X11/X.h`,
  `X11/Xatom.h` and `X11/Xproto.h` gives `None 0L` (X.h 115), `Success 0` (350), `XA_WINDOW
  ((Atom) 33)` (Xatom.h 41), `StructureNotifyMask (1L<<17)` (168), `PropModeAppend 2` (477) and
  the rest (the ledger's Evidence has the commands).  The reference tree defines none of them
  (measured: `grep -rn '# *define *<name>' emacs/` is empty for every one), so the headers are
  the only source there is.  They sit in a **module** rather than at the mirror's top level, and
  each function imports the few it needs (`use crate::x11_constants::{None, Success};` inside the
  body), because a top-level `None` would shadow Rust's prelude `Option::None` for the whole
  unit.  The mirror's own gate is still OK (`--check emacs/src/xterm.h ...`).
* **C's `False`/`True` are Rust's `false`/`true`** in the ported statements: the token check
  folds case (that is what the equivalence set is for), and `False` is not a Rust literal.  The
  other X11 macros keep C's spelling because Rust has none for them.
* **Nine bodies still carry a `does not mention` row** (the `- ` list's 305-313): `uint8_t`/
  `uint16_t`/`uint32_t` (C's fixed-width spellings; the port writes `u8`/`u16`/`u32`), `alloca`
  (the stack pool, P-05), and for `xm_setup_dnd_targets` also `goto` and `uninit` (the labelled
  loop, and `UNINIT` = conf_post.h 369's zero marker, which the port writes as the explicit
  `std::ptr::null_mut ()` initialiser).  Three `macro-body-notes.tsv` rows cover all nine; they
  are requested in this round's report.  The boxes are ticked because the text is written and
  compiles; the rows are the gate's record of the substitutions.


### 13. The mirror's x11-arm coverage gaps: filled, and the口径 of the gating written down

`spikes/mirror-struct-coverage.py --arm x11` (the integrator's new tool, 3.282/3.283) reported
`98 gap(s) over 4 struct(s)` in the x11 arm, **80 of them `struct x_display_info` and 2
`struct x_output` - both this lane's**.  They are filled now and the reading is **0** for the
whole tree (`282 mirrored struct(s): ... 0 gap(s)`; the other 16 were dispextern's and landed
with that lane's work).

What went into `femacs/src/include/xterm.rs` (+131 lines):

* **85 `x_display_info` members** in C's order: 80 the x11 arm reads (C 734-990 - `wm_type`,
  `x_dnd_atoms`/`_size`/`_length`, `motif_drag_atom`/`_name`/`_time`/`_owner`/`motif_drag_window`,
  the EWMH/XSettings/SM/XDND/XDS `Xatom_*` families, `net_supported_atoms`/
  `nr_net_supported_atoms`/`net_supported_window`, `modmap`, `screen_width`/`_height`/`_mm_*`,
  `failable_requests`/`next_failable_request`, `pending_dnd_time`, `quit_keysym`/
  `quit_keysym_time`) plus the five `#ifdef HAVE_XKB` virtual-modifier atoms (gated
  `have_xkb`; the x11 arm does not define that macro, so the tool does not count them - they are
  here because `xterm.c`'s `ATOM_REFS_INIT` table reaches them).  `motif_drag_atom_name`'s
  length is the arm's own reading of C's constant expression: **33**.
* **2 `x_output` members** (`black_relief`, `white_relief`) and the record they need: C defines
  `struct relief { GC gc; unsigned long pixel; }` *inside* `struct x_output` (C 1308-1313), and
  Rust has no nested struct definitions, so the port hoists it next to its user.
* **`X_WMTYPE_UNKNOWN`/`_A`/`_B`**, the members of C 740-745's anonymous enum (the port keeps
  `wm_type: c_int` and writes the constants, PORTING.md 1.3).

**The口径 the integrator asked to have written down: this gating is the port's, not C's.**
In C, `struct x_display_info` and `struct x_output` are **ungated** - they sit inside
`XTERM_H` and are compiled whenever the header is read, whatever the configuration.  The port
carries both under `#[cfg (feature = "have_x_windows")]`, and the reason is type availability,
not C's own condition: their members name Xlib types (`Display`, `Screen`, `XVisualInfo`,
`Atom`, `Window`, `GC`, `Pixmap`, `KeySym`, `XModifierKeymap`, ...) that no mirror in this tree
defines, so a frozen-configuration layout assertion for them cannot be compiled.  The same
sentence is now in the mirror's own ledger (`docs/ports/include-xterm.md`), and it is the reason
`mirror-struct-coverage.py --arm frozen` reports these records as `unavailable` rather than
complete: the port narrowed the gate, and the narrowing is what the gated-arm reading measures.

The reading, before and after (both `--arm x11`):

```
282 mirrored struct(s): 846 arm(s) attempted, 239 unreadable, 114 gap(s)     # before
## x11: 98 gap(s) over 4 struct(s)
   struct x_display_info            80 missing e.g. ['wm_type', 'x_dnd_atoms', ...]
   struct x_output                   2 missing e.g. ['black_relief', 'white_relief']
282 mirrored struct(s): 846 arm(s) attempted, 241 unreadable, 0 gap(s)       # after
```

The X11-arm probe moved with it: the unit's own `no field` errors went **20 → 2** (the two that
remain are `buffered_input_event.sie`, Decision 5), while the *mirror* side's errors grew
(153 → 220) because the members just carried are live under the arm and name Xlib types - the
expected direction: the mirror now holds more of the gated surface, and that surface cannot
compile until an X11 header mirror exists.

One process note, because it is the kind of thing this lane has to watch: the hoisted `relief`
record was first inserted between `struct x_output`'s attributes and the struct's own `pub struct`
line, which un-gated `x_output` and made `layout-check` fail with `cannot find type Pixmap` in
`include/xterm.rs`.  The failure was shown to be *mine* rather than pre-existing by restoring
`HEAD`'s mirror (layout-check passed), and the fix was to move the hoisted record below the
struct's closing brace.  `layout-check passed` again.


### 14. Slice 5 (C 2614-3012): the Motif senders and readers, and the toplevel teardown

Nine functions - `xm_send_drag_motion_message`, `xm_send_top_level_leave_message`,
`xm_read_drop_start_reply`, `xm_read_drop_start_message`, `xm_read_drag_receiver_info`,
`xm_read_drag_motion_message`, `xm_read_drag_motion_reply`, `x_dnd_send_xm_leave_for_drop` and
`x_dnd_free_toplevels` - all ticked.  C's `x_dnd_compute_toplevels` (C 3013, with its
`USE_XCB`/`HAVE_XCB_SHAPE` arms) opens the **next** slice.

Three shapes worth recording:

* **The `XM_DRAG_*` macros are int-typed now, and that is a correction to slice 2's text.**  C's
  macros do their arithmetic in `int` (the arguments promote out of the `uint8_t`/`uint16_t`
  fields), and slice 2's spelling kept the arguments' own widths, so the X11 probe reported
  `no implementation for u8 | i32` and `mismatched types` at C 2661-2667 / 2903-2904 and in the
  four readers' comparisons.  Each macro now casts its operands to `c_int` inside the body
  (`(($op) as c_int | (($site) as c_int) << 4 | ...)`), which is what C's promotions do, and the
  call sites narrow at the store (`... as u16`).  The token sets are unchanged - `c_int` is in
  the gate's skip list.
* **The message readers keep C's own puns**: `data = (const uint8_t *) &msg->xclient.data.b[0]`
  becomes `core::ptr::addr_of! ((*msg).xclient.data.b[0])`, `*(data++)` becomes a read plus
  `data = data.add (1)`, and `*(uint16_t *) (data + 2)` stays a cast-deref
  (`*(data.add (2) as *const u16)`).  `data[0]` keeps its subscript (`*data.offset (0)`) so the
  `0` stays in the compared token set, and `msg.xclient.type` is `r#type` (Decision 12's note).
* **`x_dnd_free_toplevels`**: C's `UNINIT` locals are `std::ptr::null_mut ()` initialisers
  (Decision 12), `record_unwind_protect_ptr (xfree, ...)` uses the mirror's `xfree_wrapper` (the
  choice `lisp.rs` 3033 makes), the `#ifdef HAVE_XSHAPE` frees and `XShapeSelectInput` are gated
  `have_xshape` (P-51), and `x_dpyinfo`/`x_dnd_fix_motif_leave` are this file's own later items,
  so they are called bare.

**The `does not mention` rows are now 17** (the `- ` list's 296-312): sixteen bodies whose C
signature or bodies spell `uint8_t`/`uint16_t`/`uint32_t` (the port writes `u8`/`u16`/`u32`),
`alloca` (the stack pool), `goto`/`uninit` (`xm_setup_dnd_targets`) and `display`
(`x_dnd_free_toplevels`, whose `Display *dpy` is the port's opaque `*mut c_void` + `/// C:` line,
P-72 rule 3).  Four `macro-body-notes.tsv` rows cover them; the request is in this round's report.


### 15. Slice 6 (C 3013-3614): the toplevel list, and the `x11_ext` arm's five members

> **Landing note (3.349).**  This tick, this decision and its evidence landed in `c8a3d8a`
> (3.336, the `xlib.rs` round) as a *whole file*, while the code half - 666 lines of
> `femacs/src/xterm.rs` carrying `x_dnd_compute_toplevels` - stayed in the lane's working
> tree.  HEAD then contradicted itself for two rounds: the ledger quoted readings (the `300`
> below, `build-tu` built, `layout-check passed`) that the committed tree could not produce
> (`git show HEAD:femacs/src/xterm.rs | grep -c x_dnd_compute_toplevels` was **0** while this
> file ticked it).  The 3.349 commit lands the code half, so the readings below are now HEAD's
> own.  Nothing else in this decision changes.

**The function.**  `x_dnd_compute_toplevels` (C 3013-3614, 601 lines) is ticked - the whole
function, both of its configuration arms carried under their features (P-51):

* `#ifndef USE_XCB` / `USE_XCB` run through the entire body: `#[cfg (not (feature = "use_xcb"))]`
  and `#[cfg (feature = "use_xcb")]` blocks at C 3023-3047 (the two local sets), 3093-3155 (the
  six `SAFE_NALLOCA!` calls and the xcb requests), 3207-3289 (the xcb reply reads), 3293-3298,
  3314-3324, 3333-3345, 3350-3352, 3385-3419, 3421-3459, 3507-3517, 3546-3577, 3580-3595 and
  3606-3608 - each tagged with the C line it comes from.
* inside them `HAVE_XSHAPE`, `HAVE_XCB_SHAPE` and `HAVE_XCB_SHAPE_INPUT_RECTS` become
  `have_xshape` / `have_xcb_shape` / `have_xcb_shape_input_rects`.
* C 3461's `#ifdef ShapeInput` asks whether the X11 shape header is new enough to define the
  shape-input selector.  The X11 headers on this machine do (`X11/extensions/shapeconst.h` 48),
  so the block is carried without a gate of its own and the decision is recorded here.
* C's `XWindowAttributes attrs` (the non-xcb arm) and `xcb_get_window_attributes_reply_t attrs`
  (the xcb arm) are the same Rust name under two `#[cfg]`s, as C has it; the xcb *types* stay
  bare (`xcb_get_property_cookie_t`, `xcb_rectangle_iterator_t`, ...), the way this file's own
  mirror keeps `KeyPressMask`.
* One port-side binding: C 3493-3528 splits one condition across the two arms (the bounding
  rectangle versus the window dimensions), and the port names it `dims_ok` so the `if` stays a
  single statement.
* Seven X11 macros this function names - `AnyPropertyType` (X11/X.h 131, `0L`), `XA_CARDINAL`
  (X11/Xatom.h 14, `6`), `IsUnmapped` (X.h 445, `0`), `PropertyChangeMask` (X.h 173, `1L<<22`),
  `ShapeBounding` (shapeconst.h 46, `0`), `ShapeInput` (48, `2`) and `ShapeNotifyMask` (50,
  `1L << 0`) - are **not** in the mirror's `x11_constants` module, so the unit keeps them bare
  (the mirror's own convention for X11 names).  They were added to the module and then taken
  back out, because this round's writing set is `femacs/src/xterm.rs` + this ledger + the
  mirror's five `HAVE_XSYNC` members only; the measured values are in this round's report for
  the next mirror round.

**The five `HAVE_XSYNC` members.**  The gated probe grew an `x11_ext` arm (3.309:
`x11`'s defines plus `HAVE_XSYNC` and `HAVE_X_I18N`), which turned the whole-tree reading from
`0 gap` into `5 gap`, all of them `struct x_display_info`:

```
xsync_supported_p   xsync_major   xsync_minor   server_time_monotonic_p   server_time_offset
```

They are now in the mirror, in C's order and with C's types, gated where C gates them:
`xsync_supported_p`/`xsync_major`/`xsync_minor` (C 911-915) under `have_xsync`, and
`server_time_monotonic_p`/`server_time_offset` (C 974-983) under
`all (have_xsync, not (use_gtk), have_clock_gettime)` - C's own
`#if defined HAVE_XSYNC && !defined USE_GTK && defined HAVE_CLOCK_GETTIME`.  **No frozen-arm
field was touched**, and `layout-check` still passes, which is the assertion that says so.

**The reading's provenance** (what the integrator asked to have written down): the arm's member
lists come from `spikes/gated-layout-probe.py 'struct x_display_info' xterm.h`'s four-arm
output (`frozen`/`x11`/`ns_cocoa`/`x11_ext`; `frozen` and `ns_cocoa` read `unavailable` for this
record).  Measured with that command, the **direct-member counts are `x11` 207 and `x11_ext`
218** - a delta of eleven, not nine: the six `HAVE_X_I18N` members (`xim`, `xim_styles`,
`xim_callback_data`, `preferred_xim_style`, `xim_coding`, `use_xim`; five were already in the
mirror) plus the five `HAVE_XSYNC` ones.  The integrator's 3.309 note says `190` versus `181`;
that is a different count of the same output (the machine-readable screen the tool uses), and
the two are worth reconciling in one place - what both agree on, and what `mirror-struct-
coverage.py` measures, is that only the five `HAVE_XSYNC` names were missing.


### 16. A mirror for Xlib itself: `femacs/src/include/xlib.rs` (the decision, written before the file)

`dispextern.h` **44** is `typedef XRectangle Emacs_Rectangle;`, and `frame.h`'s
`struct MonitorInfo` is two `Emacs_Rectangle`s plus four `int`s, a `char *` and a `bool` - so the
same record is **32 bytes in the x11 arm and 48 in `ns_cocoa`**, and the difference is the
*typedef*, not the record:

```
$ python3 spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h
## frozen       unavailable (incomplete type: this configuration has no window system)
## x11          [sizeof=32, align=8]      (XRectangle: 4 shorts, 8 bytes)
## ns_cocoa     [sizeof=48, align=8]      (the non-X struct: 4 ints)
## x11_ext      [sizeof=32, align=8]
```

The port's `include/dispextern.rs` carries only the non-X struct, so the port would give 48 under
X11 - a divergence recorded in `include/frame.rs` 1200-1208.  The fix is a home for **Xlib's own
declarations**, and this is the decision about it (the file lands below, in the same round):

* **Place**: `femacs/src/include/xlib.rs`.  The `src/include` tree is where the X11 arms' mirrors
  live (`xterm.rs` is next door) and the units that need it are `src/*.c`'s.
* **First line**: `// femacs: mirror of the X11 SDK's <X11/Xlib.h> (system header of another
  target's toolchain, PORTING.md P-26/P-49).` - the shape `tools/check-target-names.py` accepts
  as a declared non-port (`^// femacs: mirror of the .+?'s <`), taken verbatim from
  `femacs/src/include/jni.rs` (the Android NDK's `<jni.h>`), which is this repo's precedent for a
  system header of *another* target's toolchain.
* **What it carries in this round**: `XRectangle` (Xlib.h 426-429: `short x, y; unsigned short
  width, height;` - 8 bytes, align 2, asserted) and the `pub mod x11_constants` block **moved out
  of `include/xterm.rs`**, where it was an ad-hoc home, extended with the seven macros slice 6
  needed and had to leave bare (`AnyPropertyType`, `XA_CARDINAL`, `IsUnmapped`,
  `PropertyChangeMask`, `ShapeBounding`, `ShapeInput`, `ShapeNotifyMask`), each with the header
  line its measured value came from.
* **The split**: `include/dispextern.rs` gains C 29's include and C 44's / 66-70's arms -
  `#[cfg (feature = "have_x_windows")] pub type Emacs_Rectangle = XRectangle;` and the
  `not (…)` arm keeping today's struct - so `MonitorInfo` **reproduces** the x11 arm's 32/8
  instead of recording the divergence.
* What it deliberately does **not** carry yet: `XColor`/`Cursor`/`Pixmap`/`XGCValues` (which
  `dispextern.h` 38-45 also aliases) and the Xlib *functions* the gated arms call.  They are the
  same job's next piece, and the acceptance for this round is the record above.


### 17. The Xlib mirror's first consumer: the Xrm surface `src/xrdb.c` needs

`src/xrdb.c` (513 lines, target `femacs/src/xrdb.rs` 11, **not** a member of
`docs/baseline/frozen-build-sources.txt` - so like `xterm.c` itself, its "live" side is the X11
one) is the next X11 unit on this lane's path, and it names **17 X11 names this tree did not have
at all** (measured by the integrator against the whole tree minus the six verbatim ones, 3.320).
The names come from **two** headers, which is why this mirror now carries both:

```
$ ls -l /opt/homebrew/include/X11/Xlib.h /opt/homebrew/include/X11/Xresource.h
lrwxr-xr-x  1 liu  admin  45 Jul  7 14:55 /opt/homebrew/include/X11/Xlib.h -> ../../Cellar/libx11/1.8.13/include/X11/Xlib.h
lrwxr-xr-x  1 liu  admin  50 Jul  7 14:55 /opt/homebrew/include/X11/Xresource.h -> ../../Cellar/libx11/1.8.13/include/X11/Xresource.h
```

| name | C | mirror item |
|---|---|---|
| `DefaultScreenOfDisplay` | Xlib.h 120 (`#define`) | `macro_rules!` (see the degradation below) |
| `XResourceManagerString` | Xlib.h 1715 | `unsafe extern "C"` |
| `XScreenResourceString` | Xlib.h 1718 | `unsafe extern "C"` |
| `XrmInitialize` | Xlib.h 1483 | `unsafe extern "C"` |
| `XrmName` / `XrmClass` / `XrmRepresentation` | Xresource.h 136 / 142 / 156 | `pub type … = XrmQuark` |
| `XrmValue` | Xresource.h 160-163 | `#[repr(C)] pub struct` + 4 assertions |
| `XrmStringToQuark` | Xresource.h 91 | `unsafe extern "C"` |
| `XrmStringToQuarkList` | Xresource.h 119-122 | `unsafe extern "C"` (the two macros' target) |
| `XrmStringToNameList` / `XrmStringToClassList` | Xresource.h 140 / 146 (`#define`) | `macro_rules!`, C's body kept |
| `XrmGetFileDatabase` / `XrmGetStringDatabase` / `XrmMergeDatabases` / `XrmPutLineResource` / `XrmQGetResource` | Xresource.h 266 / 276 / 285 / 210 / 215-223 | `unsafe extern "C"` |
| `XrmStringType` | **`src/xrdb.c` 372**, not a header (`#define XrmStringType "String"`) | `pub const XrmStringType: &CStr = c"String"` (+ `.as_ptr ()` at the use site) |

The supporting names come with them because the prototypes need them: `XPointer` (Xlib.h 76,
`char *`), `Bool` (Xlib.h **78**, `#define Bool int` - not a typedef, so the port writes the alias
C's body names), `XrmQuark`/`XrmQuarkList` (Xresource.h 84), `XrmString` (87), `XrmNameList` (137),
`XrmClassList` (143), `XrmValuePtr` (163) and `XDefaultScreenOfDisplay` (Xlib.h 1832, the macro's
body here).

**Shapes** (the rule the last round's decision used, applied per name):

* scalars and one-line typedefs are **declared** (`Bool`, `XPointer`, the seven `Xrm*` aliases);
* `XrmValue` is carried **by value** with its layout asserted, because C passes it by value
  (`XrmValue value;`, `src/xrdb.c` **478**) - measured 16/8, `size`@0 `addr`@8, from a clang probe
  against the installed header (Evidence);
* the three records that appear **only** as pointers here - `Display` (Xlib.h 483, incomplete in C
  too), `Screen` (Xlib.h 253-271, an Xlib record nothing in this tree reads) and
  `XrmDatabase` (Xresource.h 175, `struct _XrmHashBucketRec *`) - degrade to `*mut c_void` with
  C's text in the `/// C:` line above the declaration (P-72 rule 3).  Keeping C's spelling for
  them was rejected here on purpose: `include/xterm.rs` already names `Display`/`Screen` bare and
  the x11-arm probe counts 15+ unresolved X11 type names for that reason (Decision 5), and adding
  a *second* home for those two names would be the collision PORTING.md 0.1-3 forbids.

Two call-site spellings follow from C's own kinds (PORTING.md P-37): the Xrm procedures keep
C's `NAME (…)` because they are procedures, and the three macros below are `macro_rules!`, so a
consumer writes `DefaultScreenOfDisplay! (display)` where C writes `DefaultScreenOfDisplay
(display)` - the standing P-37 trade-off.

**The one degradation**: `DefaultScreenOfDisplay`.  C's body (Xlib.h 120) is
`ScreenOfDisplay(dpy,DefaultScreen(dpy))`, and both of those are Xlib macros over `_XPrivDisplay`
- Xlib's own complete inline copy of its private display record (Xlib.h 489-546), which no mirror
here carries and which is ~50 private members wide.  Measured: the C macro expands **inline and
names no symbol at all**, while `XDefaultScreenOfDisplay` (Xlib.h 1832) is a symbol and returns
the same `Screen *`.  So the port keeps C's macro **name** (the C text of every consumer is what
has to be preserved) and takes the public call as the body.  This is the only place in this file
where the Rust body is not C's body.

**One bit of the integrator's ruling could not be followed as stated, and this is the reading.**
`XrmDatabase` was to keep its "single existing spelling"
(`include/xterm.rs:370` / `include/pgtkgui.rs:122` of `pub type XrmDatabase = *mut c_void`).
Measured:

```
$ sed -n '369p' femacs/src/include/xterm.rs
  pub rdb: XrmDatabase,   // C: XrmDatabase rdb;
$ rg -n 'pub type XrmDatabase' femacs/
femacs/src/include/pgtkgui.rs:122:pub type XrmDatabase = *mut c_void;
$ rg -n 'pgtkgui.rs' femacs --glob '*.rs'
femacs/src/include/dispextern.rs:2393:// pgtkgui.rs 214, nsgui.rs 91, androidgui.rs 245): a second `macro_rules! NativeRectangle` in
$ rg -n 'include!\("pgtkgui.rs"\)' femacs | wc -l
       0
```

`include/xterm.rs` **uses** the name (line 369, inside the `have_x_windows` arm), and its only
*declaration* in the tree is `include/pgtkgui.rs` 122 - in a mirror no include graph reaches, so
it is not in scope at any unit's crate root.  Per the ruling this round therefore declares
nothing: the Xrm prototypes spell it `*mut c_void` with C's text in the `/// C:` line.  If the
X11 graph should own the name, its P-72 rule-1 home is **this** file (`<X11/Xresource.h>` is its
header) and the change is one line - the integrator decides; the pgtkgui copy can stay, since
nothing includes that mirror.

**`XrmStringType` is carried here, not left for the unit.**  C defines it in `src/xrdb.c` 372, so
`src/xrdb.rs` will want it; putting it in the mirror gives the name one home and keeps the unit's
`--check xrdb.c xrdb.rs` honest (`tools/extract-abi.py` follows `include!` chains, so the mirror's
`pub const XrmStringType` is what the check matches C 372's constant against).  Its shape is
`&CStr = c"String"`, not the `&str` that `include/lisp.rs` uses for `#define pI "l"`: C's literal
carries a terminating NUL and `src/xrdb.c` 386 hands it to a C function, which is the
`<config.h>`-string shape of `src/emacs.rs` (status 3.64).  Measured on a scratch pair outside the
repo (`--reference`/`--root` pointed at it): the extractor compares the constant's **bytes** and
accepts both spellings, and rejects a wrong value under either.  When `xrdb.rs` lands it must *use*
this, not declare a copy: a second `const` of the same name in the same scope is `E0428`, and
`check-mirror-dupes.py` would see the copy.

**Ledger placement**: this stays in `docs/ports/xterm.md` rather than a new
`docs/ports/include-xlib.md`.  Reasons: `xlib.rs` has no reference file to be named after (the
tree's `docs/ports/include-*.md` ledgers are one per *reference* header, and this mirror's headers
are another toolchain's), its only includers are this lane's X11 mirrors, and Decision 16 above is
already the record of its location and first line - splitting it now would put one file's two
halves in two ledgers.

**Not link-verifiable on this machine**: only the headers are installed (no `libX11` stubs), which
the C probe measures (Evidence).  These are declarations, so they add no reference and do not move
`tools/check-link.py`'s ratchet; the acceptance for them is C's header text, the two layouts, and
the x11-arm probe.

**One unrequested fix, in this file, revertible in one line**: the `x11_constants` module's
twenty-one constants are typed `c_int`/`c_long`/`c_ulong`, and a *nested module* does not see the
including unit's imports - measured: without the `use core::ffi::{c_int, c_long, c_ulong};` line
now at the module's top, all twenty-one are `E0425: cannot find type c_long` in the x11-arm probe,
which buries the arm's real gaps (411 errors) among twenty-one scoping errors (432 before).  That
is why the line is there; PORTING.md 0.1-9 forbids `use crate::…` in a mirror, and `core::ffi` is
the absolute path.


### 18. Slice 7 (C 3616-4466): the target-window discovery, and the two functions C keeps inside its own conditions

Seven checklist functions - `x_dnd_io_error_handler` (C 3616-3624), `x_dnd_get_target_window_1`
(3720-3817), `x_dnd_get_wm_state_and_proto` (3819-3980), `x_dnd_do_unsupported_drop` (3985-4079),
`x_dnd_send_unsupported_drop` (4081-4125), `x_dnd_fill_empty_target` (4127-4137) and
`x_dnd_get_target_window` (4139-4466), 851 lines of C - plus `X_DND_SUPPORTED_VERSION` (C 3626),
the version byte the later `x_dnd_send_enter` writes into the XdndAware property.  The slice
stops before C 4468 (`x_dnd_get_window_proxy`, then `x_dnd_get_window_proto` at 4510) on purpose:
those two are the query helpers the drag *senders* are built on, while everything here is the
search that decides which window to send to.

**Two functions the checklist does not list.**  C 3632-3696 (`x_dnd_get_proxy_proto`, inside
`#ifdef USE_XCB`) and C 3698-3718 (`x_dnd_get_target_window_2`, inside `#ifdef HAVE_XSHAPE`) sit
in branches the frozen reading calls dead, so the inventory has no box for either - the state
`x_dnd_compute_toplevels`'s inner arms are in.  Per P-51 the port carries both functions whole,
under `use_xcb` and `have_xshape`; being unlisted they are compared by no gate, which is why the
reading below moves by the nine rows the seven listed functions and the constant clear, and by
neither of these two.

**The composite arm gets a derived name.**  `x_dnd_get_target_window` wraps its overlay-window
code in C 4225's and C 4377/4400's
`#if defined HAVE_XCOMPOSITE && (XCOMPOSITE_MAJOR > 0 || XCOMPOSITE_MINOR > 2)`.  The first half
is `have_xcomposite`; the second names an X11 header's major/minor pair, which this tree cannot
see, so the port carries it under the derived name **`have_xcomposite_overlay`** (the
`have_xcb_shape_input_rects` shape of Decision 3):
`#[cfg (all (feature = "have_xcomposite", feature = "have_xcomposite_overlay"))]`.  Measured with
the frozen table, every line of both arms is dead (`tools/cinventory.py`'s `dead_branch_map`:
4225-4266 and 4377-4443 all `False`), so nothing in them is demanded by the token check.  The arm
reads three `x_display_info` members - `composite_supported_p`, `composite_major`,
`composite_minor` (xterm.h 921-925) - that **no arm of the mirror carries** (the `x11_ext` arm
defines neither `HAVE_XCOMPOSITE` nor them), so the port names them the way the C text does; the
three members are carried now, under `have_xcomposite` (§19①).  `Xatom_NET_WM_CM_Sn` (C 609,
ungated) is already in the mirror.

**Seven X11 names the module did not carry, carried in §19③ now** (Decision 15's ruling is what
kept them bare through this slice; the values below are this round's measurement, and §19③ records
what still has to happen in the *unit* before they resolve):

| name | header:line | measured value | site |
| --- | --- | --- | --- |
| `NormalState` | Xutil.h 147 | `1` | C 3734 |
| `IsViewable` | X.h 447 | `2` | C 4245, 4422 |
| `XA_PRIMARY` | Xatom.h 9 | `((Atom) 1)` | C 4038 |
| `ButtonPress` | X.h 183 | `4` | C 4062 |
| `ButtonRelease` | X.h 184 | `5` | C 4069 |
| `ButtonPressMask` | X.h 153 | `(1L<<2)` | C 4067 |
| `ButtonReleaseMask` | X.h 154 | `(1L<<3)` | C 4073 |

`<X11/Xutil.h>` has no mirror *file* of its own in this tree, and it does not need one: this
file's five headers are one mirror's business (`src/xrdb.c` 37-41, the "one mirror for the five"
note in `src/xrdb.rs`), so `NormalState` is a module row like the three `<X11/extensions/
shapeconst.h>` ones above it, and §19② records the measurement and the reasoning in full.

**Two declarations move into the body.**  `x_dnd_do_unsupported_drop` calls
`decode_window_system_frame` (declared `frame.h` 1474-1475, defined `frame.c` 124) and
`x_own_selection` (declared `xterm.h` 1947-1948, defined `xselect.c` 280).  This unit's include
graph reaches no mirror that carries either - `include/frame.rs` records the frame prototypes as
text and there is no `xselect.h` mirror at all - so both are transcribed into the function that
needs them, which is what the xterm.h mirror's own note says the recorded prototype block is for
(the `font.rs` 413-418 `decode_window_system_frame` block, the `keyboard.rs`
`haiku_handle_selection_clear` shape).  **Those two inline declarations are the port's, not this
mirror's, and they are recorded as such**: if the owning unit lands (`src/xselect.rs`; the block
`src/frame.rs` will carry `frame.h` 1474-1475 in), the declaration moves back into that mirror and
the inline copy in `src/xterm.rs` 3982-3988 is deleted (P-26).  `x_get_atom_name` (declared
`xterm.h` 1844, *defined* `xterm.c` 29984) needs none: Rust resolves same-unit items by name.

**Five port-side shapes, each recorded rather than silent.**

* C's `for (tem = x_dnd_toplevels; tem; tem = tem->next)` runs its increment on `continue` too;
  the port keeps the increment after a labelled `'body:` block and writes C's `continue` as
  `break 'body` (the `overlay_arrow_in_current_buffer_p` shape, deviation 133).
* A bare `break` inside that labelled block is `E0695`, so the loop itself is labelled and C's two
  `break`s are `break 'toplevels` - found by the x11-arm probe, not by any text gate.
* C 3776-3782 writes `chosen = tem; break;` once, shared by the `#ifdef HAVE_XSHAPE` arm; the port
  names that arm's condition `bounding_only` so the `if` stays one statement (the `dims_ok` shape,
  Decision 15).
* C's comma operator (`free (error), rc = false;`, C 3878-3879 and its siblings) has no Rust
  spelling, so the two statements stand on their own lines (the `magic_db` row's class); and the
  widenings C performs implicitly are written out where the arm would need them
  (`make_fixnum (root_x as EMACS_INT)`, `xm_drag_style::XM_DRAG_STYLE_NONE as c_int`).
* C leaves `x_dnd_get_proxy_proto`'s cookies and several `x_dnd_do_unsupported_drop`/
  `x_dnd_get_target_window` locals uninitialised; the port writes the zero (PORTING.md 1.5).  The
  xcb type names stay bare, which leaves `free (error)`/`free (reply)` needing the pun
  `free (x as *mut c_void)`: that spelling is this slice's, while slice 6 modelled the same
  pointers as `*mut c_void` up front.  Both readings are text-only (nothing with `use_xcb`
  compiles here), and unifying the two spellings is a follow-up, not something this round claims
  to have settled.

**Two new `does not mention` rows, both table requests.**  The gate now reads
`function x_dnd_get_target_window_1 does not mention continue` and
`function x_dnd_send_unsupported_drop does not mention alloca`; the reasons are the `'body:`
shape above and PORTING.md P-05 (C's `alloca` is the port's `sa_stack_alloc`).  They are the same
class as Decision 12's three `alloca` rows and Decision 14's `goto, uninit` row, still open
because `docs/ports/macro-body-notes.tsv` is not in a code lane's writing set - all five are
requested together in this round's report.

**What this slice does not claim.**  The whole body is under `have_x_windows`, so the frozen
`build-tu` prints `built` while compiling none of it (Decision 1); the `use_xcb`, `have_xshape`
and `have_xcomposite` arms have never been compiled by anything, and the x11-arm probe is the
only reading that reaches the rest.  `x_dnd_get_target_window` still calls five helpers that are
later slices of this same file (`x_dnd_get_window_proxy`, `x_dnd_get_window_proto`,
`x_top_window_to_frame`, `x_tooltip_window_to_frame`, `x_get_window_below`), so no X11 link of
this unit is in reach this round.

### 19. The `have_x_windows` header surface slices 6 and 7 named (2026-10-05)

Slice 7's decision recorded three header gaps "for this round's report" - the five `xterm.h`
members, the `<X11/Xutil.h>` question and the seven X11 macros - and slice 6's own note recorded a
fourth: the xcb type names.  This is that round, and its writing set is this lane's two mirrors
(`src/include/xterm.rs`, `src/include/xlib.rs`) plus this ledger: **no unit changed**, so every
reading below that moves, moves in the mirror.

**Measurement first, because two of the four questions turn on it.**  This machine *has* the X11
headers (`xorgproto` + `libX11` under `/opt/homebrew/include/X11`) and the XCB ones
(`/opt/homebrew/include/xcb`), so neither `<X11/Xutil.h>` nor the xcb half is a `text-only`
question:

```
$ ls /opt/homebrew/include/X11/Xutil.h /opt/homebrew/include/xcb/xcb.h
/opt/homebrew/include/X11/Xutil.h
/opt/homebrew/include/xcb/xcb.h
$ grep -n 'define NormalState' /opt/homebrew/include/X11/Xutil.h
147:#define NormalState 1	/* most applications want to start this way */
$ grep -n 'define IsViewable\|define ButtonPress\|define ButtonRelease' /opt/homebrew/include/X11/X.h
153:#define ButtonPressMask			(1L<<2)
154:#define ButtonReleaseMask		(1L<<3)
183:#define ButtonPress		4
184:#define ButtonRelease		5
447:#define IsViewable		2
$ grep -n 'define XA_PRIMARY' /opt/homebrew/include/X11/Xatom.h
9:#define XA_PRIMARY ((Atom) 1)
```

**① The five `struct x_display_info` members.**  Each is a *record member* - an object - not a
macro or a typedef, so each is carried as the member C declares, under C's own condition and at
C's own position in the struct:

| member | C | mirror (gated) |
| --- | --- | --- |
| `xcb_connection_t *xcb_connection` | xterm.h 842, inside `#ifdef USE_XCB` (841-844) | `#[cfg (feature = "use_xcb")] pub xcb_connection: *mut xcb_connection_t` |
| `xcb_visualtype_t *xcb_visual` | xterm.h 843 | `#[cfg (feature = "use_xcb")] pub xcb_visual: *mut xcb_visualtype_t` |
| `bool composite_supported_p` | xterm.h 922, inside `#ifdef HAVE_XCOMPOSITE` (921-925) | `#[cfg (feature = "have_xcomposite")] pub composite_supported_p: bool` |
| `int composite_major` | xterm.h 923 | `#[cfg (feature = "have_xcomposite")] pub composite_major: c_int` |
| `int composite_minor` | xterm.h 924 | `#[cfg (feature = "have_xcomposite")] pub composite_minor: c_int` |

`xterm.h` 841-844 is where the two xcb pointers come from (the round's list wrote 840-843; the
measured guard is 841 and the fields 842/843), and the composite trio is 922-924 inside 921-925.
The xcb pair's pointee types are the new xcb mirror's (§④), which is also where C's own
`<xcb/xproto.h>`/`<xcb/xcb.h>` includes are recorded (`src/xterm.c` 643-646, and again through
`xterm.h` 117-119's `<X11/Xlib-xcb.h>`).  Both additions are inert in the frozen configuration, and
the mirror's pairing is unmoved: `extract-abi --check emacs/src/xterm.h
femacs/src/include/xterm.rs` still reads `OK`.

**The rest of C 823-910 is still uncarried, and the list is now written down** rather than implied:
`xrandr_supported_p`/`_event_base`/`_error_base`/`_major_version`/`_minor_version` (824-828),
`last_monitor_attributes_list` (834), `ext_codes` (838), `supports_xdbe` (847), `supports_xi2`,
`xi2_version`, `xi2_opcode`, `num_devices`, `devices`, `pending_keystroke_time`,
`pending_keystroke_source`, `client_pointer_device` (851-907), `supports_xkb`, `xkb_event_type`,
`xkb_desc` (911-916), `prefer_native_input` (919), `xrender_supported_p`/`xrender_major`/
`xrender_minor` (899-901), `xfixes_supported_p`/`_major`/`_minor`/`_event_base` (905-908),
`xinerama_supported_p` (918) and `xshape_supported_p`/`_major`/`_minor`/`_event_base`/
`_error_base` (928-932).  The xrender trio is the one that is *already* referred to by a carried
span of this mirror: `FRAME_CHECK_XR_VERSION` (`include/xterm.rs`, C 1385-1390) reads all three
under `have_xrender`, so that macro cannot resolve until they land - the same class as the
composite trio was, found while measuring it and left on this list because this round's list has
five names.

**② `<X11/Xutil.h>`: the header is here, and it already has a home - no `Xutil.rs` was created.**
`src/xrdb.c` 37-41 includes `<X11/Xutil.h>` beside `<X11/Xlib.h>`, `<X11/Xatom.h>`, `<X11/X.h>` and
`<X11/Xresource.h>`, and this tree recorded those five as *one* mirror's business when
`include/xlib.rs` landed (`src/xrdb.rs`'s include comment: "one mirror for the five", 3.336).  The
file's `x11_constants` module already carries a *third-party* header's macros the same way - the
three `<X11/extensions/shapeconst.h>` rows since 3.336 - so `NormalState` went in beside them.  A
new `include/Xutil.rs` would have been a second home for a header this tree has already homed
(P-26 / P-72 rule 1) and would have made `src/xrdb.rs`'s own comment false; if a later round wants
the header to have a file of its own, moving one constant is the whole change.  So the mirror
question is answered by *measurement plus the existing home*, not by `text-only`.

**③ The seven constants, each with its own measured basis** (all values verified this round, never
remembered; each is in the `x11_constants` module, whose rows already cite their own headers):

| name | header:line | C text | Rust | why that type |
| --- | --- | --- | --- | --- |
| `NormalState` | Xutil.h 147 | `#define NormalState 1` | `pub const NormalState: c_int = 1` | the `WM_STATE` first long (`xterm.c` 3734's `!= NormalState as c_ulong`) |
| `IsViewable` | X.h 447 | `#define IsViewable 2` | `pub const IsViewable: c_int = 2` | a `map_state` value, like `IsUnmapped` above it |
| `XA_PRIMARY` | Xatom.h 9 | `#define XA_PRIMARY ((Atom) 1)` | `pub const XA_PRIMARY: c_ulong = 1` | the three `XA_*` rows above it (`Atom` is `unsigned long`) |
| `ButtonPress` | X.h 183 | `#define ButtonPress 4` | `pub const ButtonPress: c_int = 4` | an `XEvent` type, like `ClientMessage` above it |
| `ButtonRelease` | X.h 184 | `#define ButtonRelease 5` | `pub const ButtonRelease: c_int = 5` | as above |
| `ButtonPressMask` | X.h 153 | `#define ButtonPressMask (1L<<2)` | `pub const ButtonPressMask: c_long = 1 << 2` | a mask, like `NoEventMask`/`PropertyChangeMask` |
| `ButtonReleaseMask` | X.h 154 | `#define ButtonReleaseMask (1L<<3)` | `pub const ButtonReleaseMask: c_long = 1 << 3` | as above |

**The class those seven leave, named per site.**  They are in a *module*, and the module's own rule
is that "a consumer imports the few names it needs where it needs them" - so the unit's bare uses
stay `E0425` until the unit lane adds `use crate::x11_constants::{…}` per function.  On the flip arm
the eight sites are (measured, `--cfg 'feature="have_x_windows"'`):

```
  include/xterm.rs:103   ButtonPressMask      (the mirror's own STANDARD_EVENT_SET, see below)
  include/xterm.rs:103   ButtonReleaseMask    (same)
  src/xterm.rs:3677      NormalState
  src/xterm.rs:4040      XA_PRIMARY
  src/xterm.rs:4064      ButtonPress
  src/xterm.rs:4069      ButtonPressMask
  src/xterm.rs:4071      ButtonRelease
  src/xterm.rs:4075      ButtonReleaseMask
```

`IsViewable`'s two sites (`src/xterm.rs` 4277 and 4481) are inside the `have_xcomposite_overlay`
arms, which the plain arm does not enable, so they do not appear in that reading.  The mirror's own
two are a *second* instance of the same class and a new finding: `STANDARD_EVENT_SET`
(`include/xterm.rs` 101-104, C 144-155) is a single constant whose body names **eleven** masks bare
- `KeyPressMask`, `ExposureMask`, `ButtonPressMask`, `ButtonReleaseMask`, `PointerMotionMask`,
`StructureNotifyMask`, `FocusChangeMask`, `LeaveWindowMask`, `EnterWindowMask`,
`PropertyChangeMask`, `VisibilityChangeMask` - of which two are now carried by the module and nine
have no home anywhere in the tree.  Fixing them is a decision this round did not take: the gate
compares a macro's body *digest*, so qualifying the body's names would need a
`docs/ports/token-aliases.tsv` row, and declaring them at the mirror's top level would be the
`True`/`False` shape's second home (3.270's pending move).  It is registered here instead.

**④ The xcb surface gets a mirror, in `include/xlib.rs`.**  The two `xterm.h` members need
`xcb_connection_t`/`xcb_visualtype_t` to exist, and both of slice 6's and slice 7's spellings need
the rest of the surface, so `include/xlib.rs` gains a fourth-header section: `<xcb/xcb.h>` and
`<xcb/xproto.h>` under `all (have_x_windows, use_xcb)`, `<xcb/shape.h>` additionally under
`have_xcb_shape` (C's own reachability: 643-646, and 681-683 nested inside it).  It carries

* the scalars `xcb_window_t`, `xcb_atom_t`, `xcb_colormap_t`, `xcb_drawable_t`, `xcb_visualid_t`,
  `xcb_shape_kind_t`, and the opaque `xcb_connection_t` (xcb.h 108's incomplete struct, P-70's
  stand-in);
* the records the arms pass **by value** or read fields of - the five cookies, the five replies,
  `xcb_generic_error_t`, `xcb_visualtype_t`, `xcb_rectangle_t` and `xcb_rectangle_iterator_t` -
  each with C's field list and a `const` assertion for the size/alignment, and the extra offsets of
  the members the arms read;
* the two *named* enums as Rust enums (`xcb_map_state_t`, `xcb_shape_sk_t`; the `xm_drag_style`
  shape, since C's members are `int` and the arm must cast into the `uint8_t` field/argument);
* the fourteen procedures, with C's integer widths (`uint8_t`/`uint16_t`/`uint32_t`/`int16_t`,
  `void *` as `*mut c_void`), and the `r#type` raw identifier for C's `type` member.

Every layout is measured, not remembered: a C probe against `/opt/homebrew/include/xcb` prints the
sizes, alignments and offsets, and the assertions are *live* - flipping one to a wrong number fails
the mirror's own compile with `E0080` (the scratch readings are in this round's Evidence).

**The two spellings, and what can and cannot be settled by a mirror.**  With the mirror in place,
slice 7's spelling is the C-shaped one: its cookie is a `xcb_get_property_cookie_t` passed **by
value** (C 6996-6998), its reply is a `*mut xcb_get_property_reply_t` whose `format`/`type` it
reads, and its `free (error as *mut c_void)` is the port's pun for C's `free (error)`.  Slice 6's
spelling cannot be reconciled with any faithful mirror, and the reasons are all readable from C:

* its cookies/replies/`attrs` are `*mut c_void`/`c_void` locals, so the seven stores at C 3113-3136
  print `expected c_void, found xcb_get_window_attributes_cookie_t` - the call *returns* a 4-byte
  record (measured) and the store slot is untyped;
* C passes `window_attribute_cookies[i]` to the reply call (C 3209-3212, 3221-3224, 3233, 3281),
  while the port passes the array *base* and never indexes: `*(window_attribute_cookies as *mut
  c_void) = …` writes offset 0 on every iteration.  That is not only a type error - `SAFE_NALLOCA!`
  sizes the block from `pointee_size ($buf)`, and `size_of::<c_void>()` is **1** (measured), so the
  allocation is `nitems` bytes while each store writes four;
* its field reads are on `c_void` (`attrs.map_state`, `(*reply).format`, `(*coordinates_reply).dst_x`
  …), which is where the arm's sixteen `no field … on type std::ffi::c_void` errors come from;
* and two *pre-typing* blockers stop the arm before any of that: `{ SAFE_FREE! (); }` (C 3607 is
  `SAFE_FREE ()`, but the port's macro takes the `sa_count` expression) and `sa_count` itself,
  which the port's `#[cfg (feature = "use_xcb")] { … }` block scopes to the cookie block, while C's
  `#ifdef` adds no braces - C 3094's `USE_SAFE_ALLOCA;` is a *function-scope* declaration that C
  3607's `SAFE_FREE ()` closes over.

So the unification is a unit-lane edit (slice 6's xcb arm moved onto slice 7's spelling), not a
mirror choice; the mirror's job was to make C's types exist, and that is what it does.  Slice 7's
own remaining errors are eight sites of one class - the implicit conversions C performs and the
port must write out (Decision 14's rule): `XA_ATOM`/`XA_WINDOW` are `c_ulong` in `x11_constants`
and `xcb_atom_t` is `u32` (four argument sites, three comparisons), and
`xcb_get_property_value`'s `void *` is the port's `*mut c_void` while `xmdata` is `*mut u8`.

**⑤ Two entries retired, with the rule that replaces them.**  `x_own_selection` (declared xterm.h
1947-1948, defined `xselect.c` 280) and `decode_window_system_frame` (declared frame.h 1474-1475,
defined `frame.c` 124) are no longer mirror gaps: the one function that calls them declares them
inline (`src/xterm.rs` 3982-3988's `unsafe extern "C"`, the `font.rs` 413-418 shape), which is a
*unit* file this round may not write and which is already in the tree.  They come off this ledger's
"requested" list; **if the owning unit lands** (`src/xselect.rs`; `src/frame.rs`'s prototype
block), the declaration moves back into that mirror and the inline copy is deleted - P-26, and the
same sentence `include-xterm.md` carries for its own prototype block.

**What this round does not claim.**  Nothing was compiled with the features *on* except the
scratch readings in the Evidence: the frozen `build-tu` prints `built` while compiling none of
`have_x_windows`, and the plain flip arm's 490 errors are unmoved (every name this round adds is
behind `use_xcb`, `have_xcomposite` or the `x11_constants` module's own import rule).  No X11
target build, no link, no runtime evidence; the composite arms, the xcb arms and the `<X11/Xutil.h>`
value are text + measurement + a mirror-level typecheck only.

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

Slice 4 (C 2175-2612) and the mirror addition:

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
314
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
  305-313: nine "does not mention" rows (uint8_t/uint16_t/uint32_t, alloca, goto, uninit)
$ python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs
OK emacs/src/xterm.h -> femacs/src/include/xterm.rs
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
```

The X11 constants, as measured (`/opt/homebrew/include` is the `xorgproto` headers this machine
has; the reference tree defines none of them):

```
$ printf '#include <X11/X.h>\n#include <X11/Xatom.h>\n#include <X11/Xproto.h>\n' | clang -E -dM -I /opt/homebrew/include -x c - | grep -E '^#define (None|Success|...) '
#define None 0L                                  X11/X.h 115
#define Success 0                                X11/X.h 350
#define XA_WINDOW ((Atom) 33)                    X11/Xatom.h 41
#define StructureNotifyMask (1L<<17)             X11/X.h 168
```

The X11-arm probe after slice 4: **169 errors in the unit's own text, every one a recorded gap**
(148 `cannot find <name>` - X11 names, or functions this file defines later - and 20
`no field <name>` - the mirror's missing members); the other 153 are in the mirrors.  The probe
caught three real defects in this round's draft, none visible to `build-tu` or `--check`:
`msg.xclient.type` is Rust's keyword (`r#type`; the `r#` prefix is stripped by the token
comparison, which is why the port keeps C's name), C's `False`/`True` had to become Rust's
`false`/`true` (case folding keeps the tokens), and `xm_get_drag_window_1`'s import list was
missing `PropModeReplace`.

`tools/check-link.py` reports `1 failure(s), 1 stale`: the failure is `_cursor_row_p` and the
stale row is `_set_cursor_from_row`, both xdisp's (`femacs/src/xdisp.rs` is modified in this
working tree) - nothing xterm's, and `docs/baseline/link-pending.txt` was not touched here.

Slice 5 (C 2614-3012):

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
313
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
  296-312: seventeen "does not mention" rows; the nine "missing function" lines for this slice
            are gone (grep for them: 0)
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
```

The X11-arm probe after slice 5: **two errors in the unit's own text**, both the recorded
`buffered_input_event.sie` (Decision 5); every other in-unit error is a `cannot find <name>` for
an X11 name or a function this file defines later, and the mirror side carries 219 of the same
class.  The probe is what caught this round's real defects: fifteen enum/width errors from the
`XM_DRAG_*` macros (fixed by making them int-typed, above) and, before that, the same class of
"inert text that would not compile when flipped" that `build-tu` cannot see.

Slice 6 (C 3013-3614) and the `x11_ext` members:

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
300
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
  earliest missing: "missing function XTcondemn_scroll_bars"; the five "does not mention" rows
  now read (295-299): xm_find_targets_table_idx/xm_write_targets_table/
  xm_write_drag_initiator_info -> alloca; xm_setup_dnd_targets -> goto, uninit;
  x_dnd_free_toplevels -> display, uninit
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
$ python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs
OK emacs/src/xterm.h -> femacs/src/include/xterm.rs
$ python3 spikes/mirror-struct-coverage.py
284 mirrored struct(s): 1136 arm(s) attempted, 320 unreadable (...), 0 gap(s)
```

The X11-arm probe after slice 6: **two errors in the unit's own text**, both the recorded
`buffered_input_event.sie`; everything else is a `cannot find <name>` for an X11/xcb name or a
function this file defines later, with 226 of the same class on the mirror side.  Slice 6 needed
no fix from the probe - the three defects it caught in the earlier slices (the `r#type` keyword,
C's `False`/`True`, the `XM_DRAG_*` widths) had already settled the shapes this one reuses.

### The Xlib mirror and the `MonitorInfo` acceptance (Decision 16), as landed and measured

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs
OK emacs/src/xterm.h -> femacs/src/include/xterm.rs      (the constants moved out; C 23's
                                                          include of <X11/Xlib.h> is in)
$ python3 tools/extract-abi.py --check emacs/src/dispextern.h femacs/src/include/dispextern.rs
OK emacs/src/dispextern.h -> femacs/src/include/dispextern.rs
$ python3 spikes/mirror-struct-coverage.py
284 mirrored struct(s): 1136 arm(s) attempted, 320 unreadable (...), 0 gap(s)
   (the tool's struct scan matches lower-case names only, so `struct XRectangle` is not one of
    its structs and never asks for a C header - a fact worth knowing, not a gap)
$ python3 spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h
## frozen      unavailable (invalid application of 'sizeof' to an incomplete type)
## x11         [sizeof=32, align=8]
## ns_cocoa    [sizeof=48, align=8]
## x11_ext     [sizeof=32, align=8]
$ cd femacs && CARGO_TARGET_DIR=/tmp/... cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
300        (unchanged: the earliest missing is still `missing function XTcondemn_scroll_bars`,
            and the five "does not mention" rows are the ones decision 15 records)
```

The X11-arm probe (the unit materialised by `build-tu`, then `rustc --cfg
'feature="have_x_windows"'`): the unit's own text now has **two** errors, both the recorded
`buffered_input_event.sie`, and the four macros slice 6 left bare - `AnyPropertyType`,
`XA_CARDINAL`, `IsUnmapped`, `PropertyChangeMask` - are gone from the list because they are in
`xlib.rs`'s `x11_constants` module now and slice 6's `use` takes them.


### Decision 17's readings: the Xrm surface, and what it moved

The two record layouts, from a clang probe built against the installed headers
(round 188, 2026-10-05; commands verbatim, probe written to `/tmp/xg188-xrm/` and deleted after):

```
$ clang -std=gnu23 -I /opt/homebrew/include -o probe probe.c && ./probe
XrmValue   sizeof 16 align 8 size@0 addr@8
XRectangle sizeof 8 align 2
XrmQuark 4  Bool 4  XPointer 8  XrmName 4  XrmClass 4  XrmRepresentation 4
```

The same probe source, linked: the declarations are right and the symbols are not on this machine
(only `libX11`'s *headers* are installed; the two macro calls expand to the same real procedure,
which is why `_XrmStringToQuarkList` is listed twice):

```
$ clang -I /opt/homebrew/include link2.c -o link2
Undefined symbols for architecture arm64:
  "_XResourceManagerString", ... "_XrmGetFileDatabase", "_XrmGetStringDatabase",
  "_XrmInitialize", "_XrmMergeDatabases", "_XrmPutLineResource", "_XrmQGetResource",
  "_XrmStringToQuark", "_XrmStringToQuarkList" (twice, the two macros), "_main"
ld: symbol(s) not found for architecture arm64
```

`DefaultScreenOfDisplay` is the one name the linker does **not** list - C's macro expands inline
through `_XPrivDisplay`:

```
$ nm -u link.o     # the probe that calls DefaultScreenOfDisplay (d)
  (no _XDefaultScreenOfDisplay: the macro names no symbol)
```

The gates after the change (all re-run in this round; the file's own readings):

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs
OK emacs/src/xterm.h -> femacs/src/include/xterm.rs
$ python3 tools/extract-abi.py --check emacs/src/dispextern.h femacs/src/include/dispextern.rs
OK emacs/src/dispextern.h -> femacs/src/include/dispextern.rs
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
300                      # unchanged by this mirror: it is not a unit (first row
                        # `missing function XTcondemn_scroll_bars`)
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ python3 tools/check-mirror-dupes.py
67 duplicate declaration(s), 111 ratchet row(s), 44 failure(s)
# red tree-wide, and none of it is this lane: all 44 failures are STALE ratchet rows of
# `femacs/src/xdisp.rs` (12 against `include/dispextern.rs`, 32 against `include/window.rs`) -
# the xdisp lane's "shadowing macro removed" slice (status 3.325) plus this gate's new macro
# namespace (status 3.326), both landed while this round was in flight.  This lane's file is in
# no row of the ratchet and in no failure:
$ grep -cE 'xlib|XrmStringToNameList|XrmStringToClassList|DefaultScreenOfDisplay' docs/baseline/mirror-dupes.tsv
0
# (With the gate as it stood at the top of this round - before 3.326 - the same tree read
#  `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`, the single row being
#  `FONT_INVALID_CODE`; this round does not touch that row.)
$ python3 tools/check-target-names.py
check-target-names: 628 .rs files, 591 map targets, 0 finding(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ grep -c '^- \[ \]' docs/ports/xterm.md     # unchanged: a mirror has no checklist row
305
$ python3 spikes/mirror-struct-coverage.py
284 mirrored struct(s): 1136 arm(s) attempted, 320 unreadable (...), 0 gap(s)
$ python3 spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h
## frozen       unavailable (incomplete type: this configuration has no window system)
## x11          [sizeof=32, align=8]      (XRectangle geom: 4 shorts, 8 bytes)
## ns_cocoa     [sizeof=48, align=8]
## x11_ext      [sizeof=32, align=8]
# re-taken after status 3.327 changed the probe's tables; the x11 arm still reproduces 32/8, so
# Decision 16's acceptance ("the record's arm is reproduced, not recorded") still holds.
```

`build-tu` and the x11 arm (the unit materialised by `build-tu`, then compiled with the feature
on):

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_x_windows"' -o /tmp/xg188-xrm/xterm_probe.o build/femacs/units/xterm/_root.rs
432 errors before the `use core::ffi::{c_int, c_long, c_ulong};` line, 411 after (exactly the
twenty-one scoping errors, all of them `xlib.rs` lines 50-91, all inside `x11_constants`);
**xlib.rs now contributes 0 errors**, and the 400 remaining `E0425` spell 47 distinct names, the
largest `Atom` 161, `Window` 64, `Cursor` 19, `XEvent` 10, and the six `x_*` error/selection
helpers this lane's later slices define (`x_had_errors_p` 10, `x_catch_errors` 7,
`x_ignore_errors_for_next_request` 6, `x_stop_ignoring_errors` 6, `x_uncatch_errors_after_check`
4, `x_uncatch_errors` 3); the rest are one to six each - Decision 5's and Decision 13's recorded
gaps, not new ones.
```

One shared-table row has to move because of Decision 16's insertion (this lane's earlier round):
`docs/baseline/citations-allowlist.tsv` line 37 names `femacs/src/include/dispextern.rs:567`, and
the finding it explains is now at **587** - the same 20 lines Decision 16 added above it.  Measured:

```
$ python3 tools/check-citations.py
stale	femacs/src/include/dispextern.rs:567	dispextern.h 648-661 is right; `struct glyph *` is a parameter type	the row no longer matches a finding
drift	femacs/src/include/dispextern.rs:587	dispextern.h 648-661	`struct glyph *` is at dispextern.h:754
check-citations: 666 file(s) scanned, 40 drift, 1 range, 343 external, 37 allowed, 1 stale
```

The gate is red on that one stale row (a shared table, which this lane may not write): the fix is
`:567` -> `:587` on line 37 of the allowlist, reason unchanged.

The `XrmStringType` shape, measured on a scratch pair outside the repo (the C side is the macro
alone, so `tools/extract-abi.py` compares the constant's bytes; `--reference`/`--root` point at
the scratch directory):

```
$ python3 tools/extract-abi.py --check /tmp/xg188-xrm/t.c /tmp/xg188-xrm/t1.rs --reference /tmp/xg188-xrm --root /tmp/xg188-xrm
OK /tmp/xg188-xrm/t.c -> /tmp/xg188-xrm/t1.rs      # pub const XrmStringType: &str = "String";
$ ... t2.rs                                        # pub const XrmStringType: &core::ffi::CStr = c"String";
OK /tmp/xg188-xrm/t.c -> /tmp/xg188-xrm/t2.rs
$ ... t3.rs                                        # the same, value c"Strung"
- constant XrmStringType: C "String" vs Rust c"Strung" (different bytes)
FAIL /tmp/xg188-xrm/t.c -> /tmp/xg188-xrm/t3.rs
$ ... t4.rs                                        # &str with "Strung"
- constant XrmStringType: C "String" vs Rust "Strung" (different bytes)
FAIL /tmp/xg188-xrm/t.c -> /tmp/xg188-xrm/t4.rs
```

### Slice 7's readings: the target-window discovery, as landed and measured

The `- ` reading, before and after, with the rows that moved (`extract-abi.py --check`, then
`grep '^- '` on each output and `diff`):

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # before this slice
300
$ ...                                                                          # after it
293
$ <the two `grep '^- '` outputs, diffed>
< - missing function x_dnd_io_error_handler            # the seven functions
< - missing function x_dnd_get_target_window_1
< - missing function x_dnd_get_wm_state_and_proto
< - missing function x_dnd_do_unsupported_drop
< - missing function x_dnd_send_unsupported_drop
< - missing function x_dnd_fill_empty_target
< - missing function x_dnd_get_target_window
< - missing macro X_DND_SUPPORTED_VERSION              # the constant
< - missing constant X_DND_SUPPORTED_VERSION
< - line count drifted 89.4%: C 33125 vs Rust 3510
> - line count drifted 86.3%: C 33125 vs Rust 4524
> - function x_dnd_get_target_window_1 does not mention continue   # the `'body:` shape
> - function x_dnd_send_unsupported_drop does not mention alloca   # PORTING.md P-05
$ grep -c '^- \[ \]' docs/ports/xterm.md
305 -> 297                                                     # seven functions + one macro ticked
```

`build-tu` (twice in the round: the second run re-materialised the unit after the `E0695` fix
below) and the three shared gates, each on the final text:

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm-s7 cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ python3 tools/check-citations.py
check-citations: 675 file(s) scanned, 0 drift, 0 range, 350 external, 33 allowed, 0 stale
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-mirror-dupes.py
18 duplicate declaration(s), 18 ratchet row(s), 0 failure(s)
# the ratchet read 23 rows when this round opened and 18 after the two commits another lane
# landed while it was in flight (`72f935a` -> `334448a`); 0 failures in both readings, and this
# lane's file is in no row of either.
```

The X11 values the module does not carry, measured with `clang -E -dM -I/opt/homebrew/include`
over a probe that includes `<X11/X.h>` and `<X11/Xatom.h>`, plus the one `Xutil.h` line the probe
does not reach:

```
#define ButtonPress 4
#define ButtonPressMask (1L<<2)
#define ButtonRelease 5
#define ButtonReleaseMask (1L<<3)
#define IsUnmapped 0
#define IsViewable 2
#define XA_PRIMARY ((Atom) 1)
$ grep -n 'define NormalState' /opt/homebrew/include/X11/Xutil.h
147:#define NormalState 1	/* most applications want to start this way */
```

The composite arms are dead in the frozen reading (`tools/cinventory.py`'s `dead_branch_map`
over `src/xterm.c`, set up the way `extract-abi.py` sets it up):

```
4225 False '#if defined HAVE_XCOMPOSITE && (XCOMPOSITE_MAJOR > 0 || XCOM'
4226 False '      if (dpyinfo->composite_supported_p'
4245 False '          if (attrs.map_state == IsViewable)'
4258 False '                      return proxy;'
4377 False '#if defined HAVE_XCOMPOSITE && (XCOMPOSITE_MAJOR > 0 || XCOM'
4378 False '  if (child != dpyinfo->root_window)'
4381 True  '      if (child != None)'
4445 False '  if (child != None)'
```

The x11-arm probe (the unit materialised by `build-tu`, compiled with the feature on) is what
caught the three defects in this slice that no text gate can see:

```
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_x_windows"' -o /tmp/s7/xterm_probe.o build/femacs/units/xterm/_root.rs
# with the slice as first written
error: aborting due to 505 previous errors
  492 E0425   7 E0133   2 E0609   2 E0308   1 E0080   1 E0695
# after `break 'toplevels` (E0695), the two `make_fixnum` widenings (E0308) and the thirteen bare
# `XM_DRAG_STYLE_NONE`s (E0425; the enum lives in `xm_drag_style`, so the port names it)
error: aborting due to 490 previous errors
  480 E0425   7 E0133   2 E0609   1 E0080
# of the 480 E0425, 81 fall inside this slice's lines and every one is a class already recorded:
# X11-header names with no mirror (`Window` 34, `Atom`, `XEvent`, `XFree` 2, `XGetAtomNames`,
# `XGetSelectionOwner`, `XGetWindowProperty`, `XSendEvent` 2, `XTranslateCoordinates` 2,
# `NormalState`, `XA_PRIMARY`, `ButtonPress`, `ButtonRelease`, `ButtonPressMask`,
# `ButtonReleaseMask`) and this file's own later slices (`x_dnd_get_window_proto` 7,
# `x_uncatch_errors` 6, `x_dnd_get_window_proxy` 4, `x_catch_errors` 3, `x_had_errors_p` 2,
# `x_top_window_to_frame` 2, and one each of `x_get_atom_name`, `x_get_window_below`,
# `x_ignore_errors_for_next_request`, `x_stop_ignoring_errors`, `x_tooltip_window_to_frame`,
# `x_uncatch_errors_after_check`).  The other classes are not this lane's: the 7 E0133 are
# `include/xterm.rs` 1458-1465's INLINE bodies outside an unsafe block, E0609 is Decision 5's
# pair, and E0080 is `include/systime.rs` 27's own frozen-configuration assertion.
```

### The `have_x_windows` header surface's readings (2026-10-05)

Round 18's Decision 19, measured.  `build-tu` (twice: before and after the mirror edits, since
`materialize` snapshots the include graph), then the plain flip arm, the mirror self-check and the
widest X11 arm the ported text names.  Commands verbatim; the error tables are `rustc`'s own.

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/x11fix cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
```

```
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_x_windows"' -o /tmp/x11fix-after.o build/femacs/units/xterm/_root.rs
error: aborting due to 490 previous errors
  480 E0425   7 E0133   2 E0609   1 E0080
# by file: _root.rs (the unit, src/xterm.rs) 288, include/xterm.rs 201, include/systime.rs 1 -
# byte-identical with the before reading, and that is the expected shape: every name this round
# adds is under `use_xcb`, under `have_xcomposite`, or inside `x11_constants`, and this arm
# enables none of them.  The eight sites the seven constants leave are listed in Decision 19③.
```

```
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_x_windows"' --cfg 'feature="use_xcb"' --cfg 'feature="have_xcb_shape"' \
      --cfg 'feature="have_xcomposite"' --cfg 'feature="have_xcomposite_overlay"' \
      -o /tmp/x11fix-after-xcb.o build/femacs/units/xterm/_root.rs
# before the mirror landed, the same arm set printed one error and stopped there:
#   error: unexpected end of macro invocation  ->  `{ SAFE_FREE! (); }` (_root.rs:3504, C 3607)
# after it, with that one macro invocation patched **on the copy only** to C's own shape
# (`SAFE_FREE! (sa_count)`), the arm reaches type checking:
error: aborting due to 564 previous errors
  499 E0425   38 E0308   19 E0609   7 E0133   1 E0080
# of the 38 E0308 and 19 E0609: 30 + 16 are slice 6's `*mut c_void` model (Decision 19④), 8 E0308
# are slice 7's written-out conversions, and `cannot find` on an xcb *type or function* is 0 -
# the mirror's names all resolve.  The remaining xcb-named E0425 are the two enum members the port
# must path-qualify (`XCB_MAP_STATE_UNMAPPED`, `XCB_SHAPE_SK_BOUNDING`) and the other classes are
# the ones Decision 5 and the uncarried `x_display_info` members already own (`sie` x2,
# `xshape_supported_p` x1).
# by file: _root.rs 362, include/xterm.rs 201, include/systime.rs 1 - and **include/xlib.rs 0**:
# the mirror that carries the new xcb section contributes no error to the widest arm.
```

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs
OK emacs/src/xterm.h -> femacs/src/include/xterm.rs
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs | grep -c '^- '
286
$ python3 tools/check-citations.py
check-citations: 675 file(s) scanned, 0 drift, 0 range, 395 external, 33 allowed, 0 stale
# 395 external = 350 + this round's 45 header citations (X.h/Xatom.h/Xutil.h/xcb/*.h are not in
# this tree, so they are counted and never a failure - the class xlib.rs's first rows are already
# in)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-mirror-dupes.py
18 duplicate declaration(s), 18 ratchet row(s), 0 failure(s)
$ python3 tools/check-target-names.py
check-target-names: 628 .rs files, 591 map targets, 0 finding(s)
$ python3 tools/check-dead-file-coverage.py
1 ledger(s) bound (a C range table), 0 failure(s)
$ python3 tools/check-citation-text.py | tail -1
checked 631 / mismatched 35 / ranges 221 / unresolved 98
# unchanged in kind: the 98 unresolved are the external-header citations (this round's 45 among
# them), and the 35 mismatched are other lanes' single-line quotes.
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/x11fix cargo run -q -p xtask -- build-tu src/xrdb.c
built src/xrdb.rs -> ../build/femacs/xrdb.o
$ ... build-tu src/dispextern.h
error: rustc failed for src/include/dispextern.rs (exit status: 1)
# 180 `cannot find type c_int`, 22 `ptrdiff_t`, 19 `Lisp_Object`, ... - the documented class of a
# *mirror compiled on its own* (PORTING.md 0.1-9: those names come from the unit's own includes;
# include-xterm.md records the same reading for src/xterm.h).  Not attributable to this round: the
# failing names are unit-root imports, and no version of xlib.rs supplied them.  `xrdb.c`, the
# consumer this mirror was built for, is `built`.
```

```
# the xcb mirror's own typecheck (not a unit flip arm: a scratch crate that includes xlib.rs only)
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj \
      --cfg 'feature="have_x_windows"' --cfg 'feature="use_xcb"' --cfg 'feature="have_xcb_shape"' \
      -o /tmp/xcb-mirror-probe.o /tmp/xcb-mirror-probe.rs
exit 0        # cookies passed by value, reply fields read, the iterator walked - no error
# ... and the layout assertions are live, not decorative:
$ sed 's/== 32)/== 999)/' femacs/src/include/xlib.rs > /tmp/xlib-flip.rs   # on the copy only
error[E0080]: evaluation panicked: assertion failed: core::mem::size_of::<xcb_get_property_reply_t>() == 999
```

```
# the layouts, measured against the installed headers (a C probe, compiled and run)
$ clang -I/opt/homebrew/include -o /tmp/xcb-layout-probe /tmp/xcb-layout-probe.c && /tmp/xcb-layout-probe
sizeof  xcb_get_property_cookie_t   4        alignof 4
sizeof  xcb_get_property_reply_t   32        alignof 4    format 1   type 8   value_len 16
sizeof  xcb_get_window_attributes_cookie_t   4          sizeof  xcb_get_window_attributes_reply_t 44
                                        alignof 4    map_state 26   colormap 28   your_event_mask 36
sizeof  xcb_translate_coordinates_cookie_t   4          sizeof  xcb_translate_coordinates_reply_t 16
                                        alignof 4    child 8    dst_x 12   dst_y 14
sizeof  xcb_get_geometry_cookie_t   4                    sizeof  xcb_get_geometry_reply_t 24
                                        alignof 4    root 8     width 16   height 18  border_width 20
sizeof  xcb_generic_error_t        36        alignof 4    error_code 1   resource_id 4
sizeof  xcb_shape_get_rectangles_cookie_t   4           sizeof  xcb_shape_get_rectangles_reply_t 32
                                        alignof 4    rectangles_len 8
sizeof  xcb_rectangle_t             8        alignof 2    x 0  y 2  width 4  height 6
sizeof  xcb_rectangle_iterator_t   16        alignof 8    data 0  rem 8  index 12
sizeof  xcb_visualtype_t           24        alignof 4    visual_id 0  _class 4  colormap_entries 6
const   XCB_MAP_STATE_UNMAPPED 0        XCB_SHAPE_SK_BOUNDING 0        XCB_SHAPE_SK_INPUT 2
```

```
# one more measurement Decision 19④ leans on: the port's `c_void` is one byte wide
$ rustc --edition 2024 /tmp/cvoid-size2.rs      # const asserts: size_of::<c_void>() == 1, align 1
asserts held
```

The two `c_void` readings above are what turn slice 6's `*mut c_void` cookie model from a style
question into a defect: `SAFE_NALLOCA!` sizes the block with `pointee_size ($buf)` and four-byte
cookies are stored into it.

The round's last edit of `include/xlib.rs` is a comment re-wording (the xcb section's header note
and one sentence of `x11_constants`' new comment).  The unit readings above were taken before it -
a doc comment cannot move an error set, and the mirror self-check was re-run after it and still
reads `exit 0` with zero errors.
