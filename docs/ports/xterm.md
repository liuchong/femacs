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
> same frozen reading, which is *not* the configuration that compiles the file (Decisions 2); slice
9 moves it to **212** (Decision 21) slice 10 (C 9786-11915, Decision 22) to **185** slice 11 (C 12066-12095, Decision 23) to **184** slice 12 (C 26217-26347, Decision 24) to **176** slice 13 (C 26349-26689, Decision 25) to **165** and slice 14 (C 26704-26992, Decision 26) to **163**.  Slice 15 (C 26994-27126, Decision 27) moves it to **159**: three functions retire, the two token rows the first reading brought are closed by `docs/ports/macro-body-notes.tsv` rows, and the invented-export row slice 14 left behind is gone.  Slice 16 (C 27128-27194, Decision 28) moves it to **158**: `x_new_font` retires as a missing item, and no token row came with it.  Slice 17 (C 27195-27284, Decision 29) **stays 158**: its two functions live inside
`HAVE_X11R6_XIM`, which the frozen table leaves `undef` (P-66's dead text), so the tool never
demanded them.  Slice 18 (C 27286-27342, Decision 30) **stays 158** as well: `xim_open_dpy` sits inside
C 27200's `#ifdef HAVE_X_I18N`, which is `undef` in the frozen table too.

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
- [x] x_dnd_get_window_proxy  (line 4469, static)
- [x] x_dnd_get_window_proto  (line 4511, static)
- [x] x_dnd_send_enter  (line 4545, static)
- [x] x_dnd_send_position  (line 4583, static)
- [x] x_dnd_send_leave  (line 4658, static)
- [x] x_dnd_send_drop  (line 4682, static)
- [x] x_dnd_do_drop  (line 4716, static)
- [x] x_set_dnd_targets  (line 4729, static)
- [x] x_free_dnd_targets  (line 4744, static)
- [x] x_clear_dnd_variables  (line 4758, static)
- [x] x_free_dnd_toplevels  (line 4765, static)
- [x] x_restore_events_after_dnd  (line 4779, static)
- [x] x_dnd_cleanup_drag_and_drop  (line 4867, static)
- [x] x_dnd_note_self_position  (line 4938, static)
- [x] x_dnd_note_self_wheel  (line 4959, static)
- [x] x_dnd_note_self_drop  (line 4987, static)
- [x] x_flush  (line 5074, static)
- [x] x_toolkit_position  (line 5160, static)
- [x] x_update_opaque_region  (line 5197, static)
- [x] x_set_clip_rectangles  (line 6479, static)
- [x] x_reset_clip_rectangles  (line 6495, static)
- [x] x_fill_rectangle  (line 6571, static)
- [x] x_clear_rectangle  (line 6685, static)
- [x] x_draw_rectangle  (line 6733, static)
- [x] x_clear_window  (line 6751, static)
- [x] x_dpyinfo  (line 6883)
- [x] x_display_info_for_display  (line 6895)
- [x] x_find_topmost_parent  (line 6907, static)
- [x] x_set_frame_alpha  (line 6932, static)
- [x] x_update_begin  (line 7419, static)
- [x] x_draw_vertical_window_border  (line 7443, static)
- [x] x_draw_window_divider  (line 7464, static)
- [x] x_flip_and_flush  (line 7576, static)
- [x] x_update_end  (line 7604, static)
- [x] XTframe_up_to_date  (line 7631, static)
- [x] x_clear_under_internal_border  (line 7696)
- [x] x_after_update_window_line  (line 7752, static)
- [x] x_premultiply_pixel  (line 7821, static)
- [x] x_draw_fringe_bitmap  (line 7852, static)
- [x] x_display_set_last_user_time  (line 8087, static)
- [x] x_create_special_window  (line 8206, static)
- [x] x_update_frame_user_time_window  (line 8226, static)
- [x] x_set_last_user_time_from_lisp  (line 8303)
- [x] x_set_cursor_gc  (line 8314, static)
- [x] x_set_mouse_face_gc  (line 8369, static)
- [x] x_set_mode_line_face_gc  (line 8409, static)
- [x] x_set_glyph_string_gc  (line 8420, static)
- [x] x_set_glyph_string_clipping  (line 8462, static)
- [x] x_set_glyph_string_clipping_exactly  (line 8478, static)
- [x] x_compute_glyph_string_overhangs  (line 8496, static)
- [x] x_clear_glyph_string_rect  (line 8530, static)
- [x] x_clear_point  (line 8538, static)
- [x] x_draw_glyph_string_background  (line 8568, static)
- [x] x_draw_glyph_string_foreground  (line 8612, static)
- [x] x_draw_composite_glyph_string_foreground  (line 8706, static)
- [x] x_draw_glyphless_glyph_string_foreground  (line 8816, static)
- [x] x_color_cells  (line 9104, static)
- [x] x_query_colors  (line 9137)
- [x] x_query_frame_background_color  (line 9205, static)
- [x] x_hash_string_ignore_case  (line 9239, static)
- [x] x_parse_color  (line 9260)
- [x] x_alloc_nearest_color_1  (line 9379, static)
- [x] x_alloc_nearest_color  (line 9521)
- [x] x_copy_color  (line 9545)
- [x] x_alloc_lighter_color  (line 9591, static)
- [x] x_setup_relief_color  (line 9668, static)
- [x] x_setup_relief_colors  (line 9713, static)
- [x] x_fill_triangle  (line 9746, static)
- [x] x_make_point  (line 9760, static)
- [x] x_inside_rect_p  (line 9771, static)
- [x] x_draw_relief_rect  (line 9797, static)
- [x] x_draw_box_rect  (line 9999, static)
- [x] x_draw_glyph_string_box  (line 10040, static)
- [x] x_composite_image  (line 10109, static)
- [x] x_draw_image_foreground  (line 10166, static)
- [x] x_draw_image_relief  (line 10289, static)
- [x] x_draw_image_foreground_1  (line 10383, static)
- [x] x_draw_glyph_string_bg_rect  (line 10465, static)
- [x] x_draw_image_glyph_string  (line 10496, static)
- [x] x_draw_stretch_glyph_string  (line 10636, static)
- [x] x_get_scale_factor  (line 10747, static)
- [x] x_draw_underwave  (line 10774, static)
- [x] x_draw_dash  (line 10850, static)
- [x] x_fill_underline  (line 10900, static)
- [x] x_draw_glyph_string  (line 10940, static)
- [x] x_shift_glyphs_for_insert  (line 11309, static)
- [x] x_delete_glyphs  (line 11324, static)
- [x] x_clear_area1  (line 11335, static)
- [x] x_clear_area  (line 11344)
- [x] x_clear_frame  (line 11404, static)
- [x] x_send_hourglass_message  (line 11427, static)
- [x] x_show_hourglass  (line 11449, static)
- [x] x_hide_hourglass  (line 11531, static)
- [x] XTflash  (line 11551, static)
- [x] XTring_bell  (line 11657, static)
- [x] x_ins_del_lines  (line 11696, static)
- [x] x_scroll_run  (line 11705, static)
- [ ] x_frame_highlight  (line 11925, static)
- [ ] x_frame_unhighlight  (line 11952, static)
- [ ] x_new_focus_frame  (line 11990, static)
- [x] make_invisible_cursor  (line 12068, static)
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
- [x] x_is_serial_more_than  (line 26261, static)
- [x] x_is_serial_more_than_or_equal_to  (line 26270, static)
- [x] x_is_serial_less_than  (line 26279, static)
- [x] x_is_serial_less_than_or_equal_to  (line 26288, static)
- [x] x_find_error_handler  (line 26297, static)
- [x] x_unwind_errors_to  (line 26317)
- [x] x_error_catcher  (line 26331, static)
- [x] x_catch_errors_with_handler  (line 26373)
- [x] x_catch_errors  (line 26390)
- [x] x_request_can_fail  (line 26398, static)
- [x] x_clean_failable_requests  (line 26421, static)
- [x] x_ignore_errors_for_next_request  (line 26464)
- [x] x_stop_ignoring_errors  (line 26520)
- [x] x_uncatch_errors_after_check  (line 26551)
- [x] x_uncatch_errors  (line 26568)
- [x] x_check_errors  (line 26614)
- [x] x_had_errors_p  (line 26651)
- [x] x_clear_errors  (line 26680)
- [x] x_try_restore_frame  (line 26729, static)
- [x] x_connection_closed  (line 26744, static)
- [x] x_error_handler  (line 27000, static)
- [x] x_error_quitter  (line 27082, static)
- [x] x_io_error_quitter  (line 27115, static)
- [x] x_new_font  (line 27137, static)
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
- [x] HIGHLIGHT_COLOR_DARK_BOOST_LIMIT  (line 9580, body bdbb3f155244)
- [x] OPAQUE  (line 6929, body d524558178b7)
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
- [x] x_error_message_stack: char* string, Display* dpy, x_special_error_handler handler, void* handler_data, struct x_error_message_stack* prev, unsigned long first_request
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
- [x] static struct x_error_message_stack x_error_message  (line 26249, static)
- [x] int x_error_message_count  (line 26252)
- [x] static char error_msg  (line 26722, static)
- [ ] static bool x_initialized  (line 30491, static)
- [ ] static unsigned x_display_id  (line 30595, static)

## Gate mapping

One row per build condition this file's *landed* text carries, in C's order: the file gate is the
one condition the file itself has, and every other row is a sub-gate a flip must set *besides*
`have_x_windows` (P-86 clause 6: setting only the file gate selects a dead arm and reports a
false "done").  Slices 1-7's Decisions record the口径 in prose; this table is the machine-readable
half.  Later slices append their rows.

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`X` in `WINDOW_SYSTEM_OBJ`, i.e. `configure.ac`'s `HAVE_X_WINDOWS`) | `feature = "have_x_windows"` (on every item *and* on the include graph) | an X11 build |
| C 797 `#if defined USE_XCB && defined USE_CAIRO_XCB` -> `USE_CAIRO_XCB_SURFACE` | `all (feature = "use_xcb", feature = "use_cairo_xcb")` | the XCB cairo build |
| C 801-805 `#if XCB_SHAPE_MAJOR_VERSION > 1 || (== 1 && MINOR >= 1)` -> `HAVE_XCB_SHAPE_INPUT_RECTS` | `feature = "have_xcb_shape_input_rects"` (the derived name, Decision 3) | xcb shape >= 1.1 |
| C 807-816 `#ifdef USE_GTK` (the pending quit event) | `feature = "use_gtk"` | a GTK build |
| C 828-837 `#ifdef USE_X_TOOLKIT` (the Xt objects) | `feature = "use_x_toolkit"` | an Xt build |
| C 858-861 `#ifdef HAVE_X_I18N` (`emacs_class`) | `feature = "have_x_i18n"` | an XIM build |
| C 863-867 `#ifdef USE_GTK` (the XTread_socket counters) | `feature = "use_gtk"` | a GTK build |
| C 869-881 `#ifndef X_XIGrabDevice`/`X_XIUngrabDevice`/`X_XIAllowEvents` | the guard reading (constant, no `cfg`) | an XI2 build |
| C 891-990 slices 1 (the selection-request chain) | `feature = "have_x_windows"` | an X11 build |
| C 1214-1548 the drag-and-drop state | `all (feature = "have_x_windows", feature = "have_xinput2")` on the XI2 pair | an XI2 build |
| C 1550/1552/1553 the `XM_BYTE_ORDER_*` / `SWAPCARD*` arms | `feature = "words_bigendian"`, `feature = "enable_checking"` | big-endian / `--enable-checking` |
| C 1763-2173 slices 3 (Motif map, targets table, drag window) | `all (feature = "have_x_windows", feature = "use_motif")` where C guards it | a Motif drag build |
| C 2175-3614 slices 4-6 (the Motif senders/readers, the toplevel list) | `feature = "have_x_windows"` + the inner arms below | an X11 build |
| C 3013-3614 slice 6's `USE_XCB`/`HAVE_XCB_SHAPE`/`HAVE_XCB_SHAPE_INPUT_RECTS` arms | `feature = "use_xcb"`, `feature = "have_xcb_shape"`, `feature = "have_xcb_shape_input_rects"` | an XCB build |
| C 3616-4466 slice 7's `USE_XCB`, `HAVE_XSHAPE`, `HAVE_XCOMPOSITE` arms | `feature = "use_xcb"`, `feature = "have_xshape"`, `all (feature = "have_xcomposite", feature = "have_xcomposite_overlay")` (the derived pair, Decision 18) | an XCB / XShape / composite build |
| C 4804-4864 `#ifdef HAVE_XINPUT2` (`x_dnd_cancel_dnd_early`) | `feature = "have_xinput2"` | an XI2 build |
| C 4791-4792 `#ifdef HAVE_XKB` (`x_restore_events_after_dnd`) | `feature = "have_xkb"` | an XKB build |
| C 4921-4923 `#ifdef USE_GTK` (`current_hold_quit`) | `feature = "use_gtk"` | a GTK build |
| C 5087-5103 `#ifdef HAVE_XDBE` | `feature = "have_xdbe"` | an Xdbe build |
| C 5105-5118 `#ifdef HAVE_XRENDER` (`x_xr_ensure_picture`) | `feature = "have_xrender"` | an XRender build |
| C 5127-5156 `#if false` (`record_event`) | dead text in every configuration (comment) | - |
| C 5163-5193 `#ifdef USE_GTK` / `#elif defined USE_X_TOOLKIT` / `#else` | `feature = "use_gtk"` / `all (not (feature = "use_gtk"), feature = "use_x_toolkit")` / neither | a GTK / Xt / plain X11 build |
| C 5215-5239 `#ifndef HAVE_GTK3` / `#else` | `not (feature = "have_gtk3")` / `feature = "have_gtk3"` | GTK3 or not |
| C 5245-5287 `#if defined USE_CAIRO || defined HAVE_XRENDER` (the GC ext data) | `any (feature = "use_cairo", feature = "have_xrender")` | a cairo or XRender build |
| C 5289-5297 `#ifdef USE_CAIRO` (`FRAME_CR_*`) | `feature = "use_cairo"` | a cairo build |
| C 5303-5326 `#ifdef HAVE_XINPUT2` + `# if defined USE_X_TOOLKIT || !(defined USE_GTK && defined HAVE_GTK3)` | `feature = "have_xinput2"` + `any (feature = "use_x_toolkit", not (all (feature = "use_gtk", feature = "have_gtk3")))` | an XI2 build that is not GTK3-GTK |
| C 5350-5352 `#ifdef USE_GTK static #endif` (`xi_convert_event_state`) | `feature = "use_gtk"` for the storage class only | a GTK build |
| C 5382-5959 the `HAVE_XINPUT2_1` / `HAVE_XINPUT2_2` device arms | `feature = "have_xinput2_1"` / `feature = "have_xinput2_2"` | XI2 >= 2.1 / 2.2 |
| C 5966-6451 `#ifdef USE_CAIRO` (the cairo helpers) | `feature = "use_cairo"` + the `CAIRO_HAS_*` derived names below | a cairo build |
| C 6019-6026 `#ifdef USE_CAIRO_XCB_SURFACE` | `all (feature = "use_xcb", feature = "use_cairo_xcb")` | an XCB cairo build |
| C 6361-6395 `#ifdef CAIRO_HAS_PDF_SURFACE` / `_PNG_FUNCTIONS` / `_PS_SURFACE` / `_SVG_SURFACE` | the derived names `cairo_has_pdf_surface`, `cairo_has_png_functions`, `cairo_has_ps_surface`, `cairo_has_svg_surface` (cairo.h's own pair shape, Decision 3) | a cairo build with that surface |
| C 6453-6477 `#if defined HAVE_XRENDER` (the ext clip) | `feature = "have_xrender"` | an XRender build |
| C 6479-6509 `#if defined USE_CAIRO || defined HAVE_XRENDER` (the clip bookkeeping) | `any (feature = "use_cairo", feature = "have_xrender")` | a cairo or XRender build |
| C 6511-6538 `# if !defined USE_CAIRO && (RENDER_MAJOR > 0 || RENDER_MINOR >= 2)` | `all (not (feature = "use_cairo"), feature = "have_xrender_0_2")` (the derived name) | an XRender >= 0.2 build |
| C 6594-6629 `#if RENDER_MAJOR > 0 || (RENDER_MINOR >= 10)` | `feature = "have_xrender_0_10"` (the derived name) | an XRender >= 0.10 build |
| C 6757-6774 `#ifndef USE_GTK` + `#ifdef HAVE_XDBE` inside its condition | `not (feature = "use_gtk")`, `feature = "have_xdbe"` | a non-GTK Xdbe build |
| C 6779-6865 `#ifdef USE_CAIRO` (the relief trapezoids, corners, wave) | `feature = "use_cairo"` | a cairo build |
| C 6994 `#if defined HAVE_XSYNC && !defined USE_GTK && defined HAVE_CLOCK_GETTIME` | `all (feature = "have_xsync", not (feature = "use_gtk"), feature = "have_clock_gettime")` | an XSync build that is not GTK, with `clock_gettime` |
| C 7249 / 7369 `#ifdef HAVE_XSYNCTRIGGERFENCE` | `feature = "have_xsynctriggerfence"` | an XSync build whose fences are available |
| C 7101 / 7271 `#ifdef FRAME_DEBUG` | `feature = "frame_debug"` | the source-level knob C defines by hand (the `glyph_debug` shape) |
| C 7818 / 8535 / 9744 `#ifndef USE_CAIRO` (the three non-cairo helpers) | `not (feature = "use_cairo")` | a build without cairo |
| C 7421-7437 / 7588 / 7609-7624 / 7637-7643 / 7674 `HAVE_XDBE` arms | `feature = "have_xdbe"` (the C 5087 row's gate, carried on) | an Xdbe build |
| C 7636 / 7888 / 8659-8661 / 8733-8809 `USE_CAIRO` arms | `feature = "use_cairo"` (the C 5289/5966 row's gate, carried on) | a cairo build |
| C 8181 `#ifdef USE_GTK` (`x_set_gtk_user_time`) | `feature = "use_gtk"` | a GTK build |
| C 8198 `#if !defined USE_GTK \|\| defined HAVE_XFIXES` | `any (not (feature = "use_gtk"), feature = "have_xfixes")` | a non-GTK or XFixes build |
| C 8223-8300 `#ifndef USE_GTK` | `not (feature = "use_gtk")` | a non-GTK build |
| C 8834/8836 `#ifdef GCC_LINT` / `#else` | `feature = "gcc_lint"` / `not (feature = "gcc_lint")` | `GCC_LINT` is 1 in this configuration (conf_post.h 486), so the first arm is the live text |
| C 8900 `#ifdef USE_LUCID` (inside `USE_X_TOOLKIT`) | `all (feature = "use_x_toolkit", feature = "use_lucid")` | a Lucid widget build |
| C 9506 / 9562 `#ifdef DEBUG_X_COLORS` | `feature = "debug_x_colors"` | the source-level knob C defines by hand |
| C 9802 / 9874 `#ifdef USE_CAIRO` (`x_draw_relief_rect`'s two arms) | `feature = "use_cairo"` / `not (feature = "use_cairo")` | a cairo build / none |
| C 10106 / 10160 `#ifndef USE_CAIRO` (`x_composite_image`) | `not (feature = "use_cairo")` | a non-cairo build |
| C 10110-10112 / 10117 / 10124 `HAVE_XRENDER` (`x_composite_image`'s parameter and its two calls) | `feature = "have_xrender"` (the C 5105 row's gate, carried on) | an XRender build |
| C 10498-10504 / 10526 / 10589 `#ifndef USE_CAIRO` (`x_draw_image_glyph_string`) | `not (feature = "use_cairo")` | a non-cairo build |
| C 10557 `#if defined HAVE_XRENDER && (RENDER_MAJOR > 0 \|\| (RENDER_MINOR >= 2))` | `all (feature = "have_xrender", feature = "have_xrender_0_2")` (the C 6511 row's pair) | an XRender >= 0.2 build |
| C 10785 / 10842 `#ifdef USE_CAIRO` (`x_draw_underwave`) | `feature = "use_cairo"` / `not (feature = "use_cairo")` | a cairo build / none |
| C 10853 / 10892 `#ifndef USE_CAIRO` (`x_draw_dash`) | `not (feature = "use_cairo")` / `feature = "use_cairo"` | a non-cairo build / a cairo one |
| C 11333 `#if defined USE_GTK \|\| !defined USE_CAIRO` (`x_clear_area1`) | `any (feature = "use_gtk", not (feature = "use_cairo"))` | a GTK or non-cairo build |
| C 11346 / 11397 `#ifdef USE_CAIRO` (`x_clear_area`) | `feature = "use_cairo"` / `not (feature = "use_cairo")` | a cairo build / none |
| C 11350 / 11358 / 11392 `#ifndef USE_GTK` (`x_clear_area`) | `not (feature = "use_gtk")` | a non-GTK build |
| C 11355 `#ifdef HAVE_XDBE` (inside that condition) | `feature = "have_xdbe"` (the C 5087 row's gate, carried on) | an Xdbe build |
| C 11366 `#if defined HAVE_XRENDER && (RENDER_MAJOR > 0 \|\| (RENDER_MINOR >= 2))` (`x_clear_area`) | `all (feature = "have_xrender", feature = "have_xrender_0_2")` | an XRender >= 0.2 build |
| C 11468 / 11470 `#ifdef USE_X_TOOLKIT` (`x_show_hourglass`) | `feature = "use_x_toolkit"` | an Xt build |
| C 11476 / 11510 `#ifndef USE_XCB` and C 11489 / 11514 `#else` (`x_show_hourglass`) | `not (feature = "use_xcb")` / `feature = "use_xcb"` | the two XCB arms |
| C 11479 / 11491 `#ifdef USE_GTK` (`parent`) and C 11538 / 11540 (`x_hide_hourglass`) | `feature = "use_gtk"` / `not (feature = "use_gtk")`, then `not (feature = "use_xcb")` / `feature = "use_xcb"` | the GTK and non-GTK arms |
| C 11677 `#ifdef HAVE_XKB` (`XTring_bell`) | `feature = "have_xkb"` (the C 4791 row's gate, carried on) | an XKB build |
| C 11753-11843 `#ifdef HAVE_XWIDGETS` (`x_scroll_run`) | `feature = "have_xwidgets"` | an xwidget build |
| C 11845 / 11872 `#ifdef USE_CAIRO_XCB_SURFACE` (`x_scroll_run`) | `all (feature = "use_xcb", feature = "use_cairo_xcb")` (the C 6019 row's pair) | an XCB cairo build |
| C 11853 / 11906 `#ifdef USE_CAIRO` (`x_scroll_run`) | `feature = "use_cairo"` / `not (feature = "use_cairo")` | a cairo build / none |

## C line ranges

One row per C block, in C's order - the table P-86 clause 2 requires of a file whose body the
frozen configuration leaves false, and the same shape `docs/ports/w32term.md`'s table uses: a
row's range runs to the line before the next row's start, so the table is contiguous and every
non-blank line of the reference is accounted for (`check-dead-file-coverage.py` reads it).  The
`arm` cell says what the port does with the block; the per-function lines of slices 1-8 are in
their Decisions, and the ones of slice 9 are in Decision 21.

| C | block | arm |
| --- | --- | --- |
| 1-624 | the file's own licence/declaration block and the long X11/input prose comment | dropped (P-86 clause 1; the port opens with the `// femacs: port of ...` line) |
| 625-635 | `<config.h>`, `<stdlib.h>`, `<math.h>`, `<signal.h>` | the file gate |
| 636-645 | `"lisp.h"`, `"blockinput.h"`, `"sysstdio.h"`, `"xterm.h"`, `"textconv.h"`, the X11 headers | the file gate |
| 646-701 | the X11 extension headers, `"sys/types.h"` ... `<byteswap.h>` | the file gate |
| 702-793 | `"character.h"` ... `"widget.h"`, the XKB/XI2 headers, `xgselect.h`, `bitmaps/gray.xbm` | the file gate (the two last under `USE_GTK`/`#if 0`) |
| 794-806 | the two X error-handler typedefs, `USE_CAIRO_XCB_SURFACE`, `HAVE_XCB_SHAPE_INPUT_RECTS` | the file gate; both arms (`use_xcb`+`use_cairo_xcb`, `have_xcb_shape_input_rects`) |
| 807-899 | the file-scope objects: `xg_pending_quit_event`, `any_help_event_p`, `x_display_list`, the Xt objects, `x_noop_count`, `emacs_class`, the XTread_socket counters, the XI2 request codes, `X_NEXT_KBD_EVENT` | the file gate + `use_gtk`/`use_x_toolkit`/`have_x_i18n`/`have_xinput2` arms |
| 900-990 | slice 1: `x_defer_selection_requests`, `x_release_selection_requests`, `x_release_selection_requests_and_flush`, `ATOM_REFS_INIT` | the file gate |
| 991-1130 | the `x_atom_refs` table (C 976-987's `x_atom_ref` record and the 122 rows) | **not landed** (Decision 9) |
| 1131-1762 | slice 2: the `xembed` enums, `X_EVENT_*`, the drag-and-drop state, the five `XM_DRAG_*` families and the two byte-order constants | the file gate + `words_bigendian`/`enable_checking` |
| 1763-2173 | slice 3: the Motif side-effect map, the targets-table reader/writer, the two error handlers, `x_special_window_exists_p` and the drag window | the file gate |
| 2175-2612 | slice 4: `xm_setup_dnd_targets`, the drag atom, the three Motif message senders | the file gate |
| 2614-3012 | slice 5: the Motif senders and readers, the toplevel teardown | the file gate |
| 3013-3614 | slice 6: `x_dnd_free_toplevels`, `x_dnd_compute_toplevels` and the `x11_ext` arm's five members | the file gate + `use_xcb`/`have_xcb_shape`/`have_xcb_shape_input_rects` |
| 3616-4466 | slice 7: the drag-and-drop target-window discovery (`x_dnd_io_error_handler` ... `x_dnd_get_target_window`) | the file gate + `use_xcb`/`have_xshape`/`have_xcomposite`(+`_overlay`) |
| 4467-4510 | `x_dnd_get_window_proxy` (C 4468-4508) | the file gate |
| 4511-4544 | `x_dnd_get_window_proto` (C 4510-4542) | the file gate |
| 4545-4582 | `x_dnd_send_enter` (C 4544-4580) | the file gate |
| 4583-4657 | `x_dnd_send_position` (C 4582-4655) | the file gate |
| 4658-4681 | `x_dnd_send_leave` (C 4657-4679) | the file gate |
| 4682-4715 | `x_dnd_send_drop` (C 4681-4713) | the file gate |
| 4716-4728 | `x_dnd_do_drop` (C 4715-4726) | the file gate |
| 4729-4743 | `x_set_dnd_targets` (C 4728-4741) | the file gate |
| 4744-4757 | `x_free_dnd_targets` (C 4743-4755) | the file gate |
| 4758-4764 | `x_clear_dnd_variables` (C 4757-4762) | the file gate |
| 4765-4778 | `x_free_dnd_toplevels` (C 4764-4776) | the file gate |
| 4779-4864 | `x_restore_events_after_dnd` (C 4778-4802, with C 4788's `HAVE_XKB` arm) and C 4804-4864's `x_dnd_cancel_dnd_early` | the file gate; `have_xkb`; `have_xinput2` |
| 4865-4937 | `x_dnd_cleanup_drag_and_drop` (C 4866-4919) and C 4921-4923's `USE_GTK` arm | the file gate; `use_gtk` |
| 4938-4986 | `x_dnd_note_self_position` (C 4937-4956) and `x_dnd_note_self_wheel` (C 4958-4984) | the file gate |
| 4987-5073 | `x_dnd_note_self_drop` (C 4986-5071) | the file gate |
| 5074-5086 | `x_flush` (C 5073-5085) | the file gate |
| 5087-5118 | `x_drop_xrender_surfaces` (C 5088-5103) and `x_xr_ensure_picture` (C 5105-5118) | the file gate; `have_xdbe`, `have_xrender` |
| 5119-5157 | the `Debugging` banner and C 5127-5153's `#if false` `record_event` | dead text in every configuration (C's own text inside a comment) |
| 5158-5196 | `x_toolkit_position` (C 5159-5194) | the file gate + `use_gtk`/`use_x_toolkit`/neither |
| 5197-5243 | `x_update_opaque_region` (C 5196-5243) | the file gate + `have_gtk3`/not |
| 5244-5287 | `x_gc_free_ext_data_private`, `x_gc_get_ext_data`, `x_extension_initialize` (C 5245-5287) | `any (feature = "use_cairo", feature = "have_xrender")` |
| 5288-5297 | the `FRAME_CR_*` macros (C 5289-5297) | `use_cairo` |
| 5298-5328 | `xi_frame_selected_for` (C 5303-5328) | `have_xinput2` + C 5305's toolkit arm |
| 5329-5378 | `xi_convert_button_state`, `xi_convert_event_state`, `xi_convert_event_keyboard_state` (C 5331-5378) | `have_xinput2` (+ `use_gtk` for the storage class) |
| 5379-5427 | `x_free_xi_devices` (C 5380-5427) | `have_xinput2` + `have_xinput2_1`/`_2` |
| 5428-5706 | `struct xi_known_valuator`, `xi_populate_scroll_valuator`, `xi_populate_device_from_info` (C 5430-5706) | `have_xinput2` + `have_xinput2_1`/`_2` |
| 5707-5792 | `x_cache_xi_devices` (C 5708-5740) and `x_get_scroll_valuator_delta` (C 5742-5792) | `have_xinput2` (+ `have_xinput2_1`) |
| 5793-5811 | `xi_device_from_id` (C 5794-5811) | `have_xinput2` |
| 5812-5963 | `xi_link_touch_point`, `xi_unlink_touch_point`, `xi_unlink_touch_points`, `xi_find_touch_point`, `xi_reset_scroll_valuators_for_device_id` (C 5813-5963) | `have_xinput2_2` / `have_xinput2_1` |
| 5964-6451 | the cairo block: `x_cr_destroy_frame_context` ... `x_cr_export_frames` (C 5966-6451) | `use_cairo` + `cairo_has_*` + `use_xcb`+`use_cairo_xcb` |
| 6452-6509 | `x_xr_apply_ext_clip`, `x_xr_reset_ext_clip` (C 6453-6477) and `x_set_clip_rectangles`, `x_reset_clip_rectangles` (C 6479-6509) | the file gate; `have_xrender`; `any (use_cairo, have_xrender)` for the bookkeeping |
| 6510-6569 | `x_xrender_color_from_gc_foreground` (C 6511-6538) and `x_xrender_color_from_gc_background` (C 6541-6569) | `have_xrender` + `not (use_cairo)` + `have_xrender_0_2` |
| 6570-6684 | `x_fill_rectangle` (C 6571-6683) | the file gate + `use_cairo` / `have_xrender`+`have_xrender_0_2`+`have_xrender_0_10` |
| 6685-6732 | `x_clear_rectangle` (C 6685-6731) | as above, without the `0_10` arm |
| 6733-6750 | `x_draw_rectangle` (C 6733-6749) | the file gate + `use_cairo` |
| 6751-6777 | `x_clear_window` (C 6751-6777) | the file gate + `use_cairo` / `not (use_gtk)`+`have_xdbe` |
| 6778-6865 | `x_fill_trapezoid_for_relief`, `enum corners`, `x_erase_corners_for_relief`, `x_draw_horizontal_wave` (C 6779-6865) | `use_cairo` |
| 6866-6893 | `x_dpyinfo` (C 6883-6893) | the file gate |
| 6894-6905 | `x_display_info_for_display` (C 6895-6905) | the file gate |
| 6906-6928 | `x_find_topmost_parent` (C 6907-6928) | the file gate |
| 6929-6989 | `OPAQUE` (C 6929) and `x_set_frame_alpha` (C 6932-6989) | the file gate |
| 6990-6993 | the section banner (`Starting and ending an update`, C's boxed comment) | carried as a comment |
| 6994-7410 | C 6994's `#if defined HAVE_XSYNC && !defined USE_GTK && defined HAVE_CLOCK_GETTIME`: `x_if_event` (C 7002), `x_sync_get_monotonic_time` (7053), `x_sync_current_monotonic_time` (7071), `x_sync_note_frame_times` (7084, with C 7101's `FRAME_DEBUG` block), `x_sync_is_frame_drawn_event` (7109), `x_sync_wait_for_frame_drawn_event` (7132), `x_sync_update_begin` (7187), C 7249's `HAVE_XSYNCTRIGGERFENCE` trio (`x_sync_trigger_fence` with C 7271's `FRAME_DEBUG` block, `x_sync_init_fences`, `x_sync_free_fences`), `x_sync_update_finish` (7339, with C 7369's inner arm) and `x_sync_handle_frame_drawn` (7383) | `all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"), feature = "have_clock_gettime")`, plus `have_xsynctriggerfence` and `frame_debug` |
| 7412-7438 | `x_update_begin` (C 7419) | the file gate; C 7421's condition again inside, plus `have_xdbe` (C 7424/7432) |
| 7440-7459 | `x_draw_vertical_window_border` (C 7443) | the file gate; `use_cairo` / not (C 7453) |
| 7461-7524 | `x_draw_window_divider` (C 7464) | the file gate |
| 7526-7570 | `#ifdef HAVE_XDBE`: `show_back_buffer` (C 7533) | `have_xdbe`, plus the `have_xsync` triple and `use_cairo` for its inner arms |
| 7572-7598 | `x_flip_and_flush` (C 7576) | the file gate; `have_xdbe` (C 7588) |
| 7600-7625 | `x_update_end` (C 7604) | the file gate; `use_cairo`+`have_xdbe` (C 7609-7614); the `have_xsync` triple (C 7619) |
| 7627-7672 | `XTframe_up_to_date` (C 7631) | the file gate; `have_xdbe` (C 7637); `have_xsync`+`not have_gtk3` / `have_xsync`+`have_gtk3` (C 7646-7670) |
| 7674-7685 | `#ifdef HAVE_XDBE`: `XTbuffer_flipping_unblocked_hook` (C 7676) | `have_xdbe` (no checklist box: dead in the frozen reading, carried per P-51) |
| 7687-7743 | `x_clear_under_internal_border` (C 7696) | the file gate (C's definition is not `static`) |
| 7745-7809 | `x_after_update_window_line` (C 7752) | the file gate; `use_x_toolkit` (C 7759's block) |
| 7811-7849 | `#ifndef USE_CAIRO`: `x_premultiply_pixel` (C 7821) | `not (feature = "use_cairo")` |
| 7851-8050 | `x_draw_fringe_bitmap` (C 7852) | the file gate; `use_cairo` / not (C 7888/7916) and `have_xrender` (C 7926-7931, 7984-8033) |
| 8052-8062 | the `Glyph display` banner and C 8056-8062's three forward declarations | carried as comments: PORTING.md has no item for a prototype (slice 1's `x_push_selection_request` precedent) |
| 8064-8176 | `x_display_set_last_user_time` (C 8087) | the file gate; the `have_xsync` triple (C 8090); `not use_gtk` (C 8095, 8158); `enable_checking` (C 8103) |
| 8178-8196 | `#ifdef USE_GTK`: `x_set_gtk_user_time` (C 8181) | `use_gtk` |
| 8198-8218 | `#if !defined USE_GTK \|\| defined HAVE_XFIXES`: `x_create_special_window` (C 8206) | `any (not (feature = "use_gtk"), feature = "have_xfixes")` |
| 8220-8300 | `#ifndef USE_GTK`: `x_update_frame_user_time_window` (C 8226) | `not (feature = "use_gtk")` |
| 8302-8307 | `x_set_last_user_time_from_lisp` (C 8303) | the file gate (C's definition is not `static`) |
| 8309-8363 | `x_set_cursor_gc` (C 8314) | the file gate |
| 8365-8401 | `x_set_mouse_face_gc` (C 8369) | the file gate |
| 8403-8412 | `x_set_mode_line_face_gc` (C 8409) | the file gate |
| 8414-8455 | `x_set_glyph_string_gc` (C 8420) | the file gate |
| 8457-8470 | `x_set_glyph_string_clipping` (C 8462) | the file gate |
| 8472-8489 | `x_set_glyph_string_clipping_exactly` (C 8478) | the file gate |
| 8491-8524 | `x_compute_glyph_string_overhangs` (C 8496) | the file gate |
| 8526-8533 | `x_clear_glyph_string_rect` (C 8530) | the file gate |
| 8535-8559 | `#ifndef USE_CAIRO`: `x_clear_point` (C 8538) | `not (feature = "use_cairo")` |
| 8561-8606 | `x_draw_glyph_string_background` (C 8568) | the file gate |
| 8608-8701 | `x_draw_glyph_string_foreground` (C 8612) | the file gate; `use_cairo` (C 8640-8699) |
| 8703-8810 | `x_draw_composite_glyph_string_foreground` (C 8706) | the file gate; `use_cairo` (C 8733-8809) |
| 8812-8896 | `x_draw_glyphless_glyph_string_foreground` (C 8816) | the file gate; `gcc_lint` (C 8834's `#ifdef GCC_LINT` pair) |
| 8898-9095 | `#ifdef USE_X_TOOLKIT`: `x_frame_of_widget` (C 8905) and `x_alloc_lighter_color_for_widget` (8941) under C 8900's `USE_LUCID`, C 8952-8967's two objects, `cvt_string_to_pixel` (8987) and `cvt_pixel_dtor` (9073) | `use_x_toolkit` (+ `use_lucid`); the `XtConvertArgRec` array is recorded as C's text (no libXt on this machine) |
| 9097-9126 | `x_color_cells` (C 9104) | the file gate |
| 9128-9200 | `x_query_colors` (C 9137) | the file gate (C's definition is not `static`) |
| 9202-9236 | `x_query_frame_background_color` (C 9205) | the file gate; `use_cairo` / not (C 9208-9230) |
| 9238-9251 | `x_hash_string_ignore_case` (C 9239) | the file gate |
| 9253-9370 | `x_parse_color` (C 9260) | the file gate (C's definition is not `static`) |
| 9372-9512 | `x_alloc_nearest_color_1` (C 9379) | the file gate; `debug_x_colors` (C 9506) |
| 9514-9537 | `x_alloc_nearest_color` (C 9521) | the file gate (C's definition is not `static`) |
| 9539-9566 | `x_copy_color` (C 9545) | the file gate (C's definition is not `static`); `debug_x_colors` (C 9562) |
| 9568-9580 | the `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT` comment and `#define HIGHLIGHT_COLOR_DARK_BOOST_LIMIT 48000` (C 9580) | the file gate (a value: the port writes the constant) |
| 9583-9657 | `x_alloc_lighter_color` (C 9591) | the file gate |
| 9659-9707 | `x_setup_relief_color` (C 9668) | the file gate |
| 9709-9742 | `x_setup_relief_colors` (C 9713) | the file gate |
| 9744-9785 | `#ifndef USE_CAIRO`: `x_fill_triangle` (C 9746), `x_make_point` (C 9760) and `x_inside_rect_p` (C 9771) | `not (feature = "use_cairo")` |
| 9786-9988 | `x_draw_relief_rect` (C 9797) | the file gate; `use_cairo` for C 9802's arm and `not (feature = "use_cairo")` for C 9874's |
| 9990-10034 | `x_draw_box_rect` (C 9999) | the file gate |
| 10036-10103 | `x_draw_glyph_string_box` (C 10040) | the file gate |
| 10105-10160 | `#ifndef USE_CAIRO`: `x_composite_image` (C 10109) | `not (feature = "use_cairo")`; C 10110-10112's `HAVE_XRENDER` adds the `destination` parameter, so the unit carries one definition per arm under `feature = "have_xrender"` |
| 10162-10283 | `x_draw_image_foreground` (C 10166) | the file gate; `use_cairo`/`not (feature = "use_cairo")`; `have_xrender` for C 10249-10252's and C 10265-10269's extra argument |
| 10285-10376 | `x_draw_image_relief` (C 10289) | the file gate |
| 10378-10458 | `#ifndef USE_CAIRO`: `x_draw_image_foreground_1` (C 10383) | `not (feature = "use_cairo")`; `have_xrender` for C 10425's `None` argument |
| 10460-10478 | `x_draw_glyph_string_bg_rect` (C 10465) | the file gate |
| 10480-10630 | `x_draw_image_glyph_string` (C 10496) | the file gate; `not (use_cairo)`/`use_cairo` for C 10498-10504's block, and `have_xrender`+`have_xrender_0_2` for C 10557's `RENDER_MAJOR`/`RENDER_MINOR` pair |
| 10632-10744 | `x_draw_stretch_glyph_string` (C 10636) | the file gate |
| 10745-10761 | `x_get_scale_factor` (C 10747) | the file gate |
| 10763-10843 | `x_draw_underwave` (C 10774) | the file gate; `use_cairo` / `not (feature = "use_cairo")` |
| 10845-10893 | `x_draw_dash` (C 10850) | the file gate; `not (feature = "use_cairo")` / `use_cairo` |
| 10895-10935 | `x_fill_underline` (C 10900) | the file gate |
| 10937-11304 | `x_draw_glyph_string` (C 10940) | the file gate |
| 11306-11318 | `x_shift_glyphs_for_insert` (C 11309) | the file gate |
| 11320-11327 | `x_delete_glyphs` (C 11324) | the file gate |
| 11329-11341 | `#if defined USE_GTK \|\| !defined USE_CAIRO`: `x_clear_area1` (C 11335) and C 11341's `#endif` | `any (feature = "use_gtk", not (feature = "use_cairo"))` |
| 11342-11398 | `x_clear_area` (C 11344) | the file gate (C's definition is not `static`); `use_cairo`/`not (use_cairo)`; C 11350-11356's `#ifndef USE_GTK` pair and `not (feature = "use_gtk")`, C 11355's `HAVE_XDBE` term; `have_xrender`+`have_xrender_0_2` for C 11366-11377 |
| 11400-11419 | `x_clear_frame` (C 11404) | the file gate |
| 11421-11444 | `x_send_hourglass_message` (C 11427) | the file gate |
| 11446-11526 | `x_show_hourglass` (C 11449) | the file gate; `use_x_toolkit` for C 11468-11472; `use_xcb`/`not (feature = "use_xcb")` for the two blocks, `use_gtk` inside each |
| 11528-11546 | `x_hide_hourglass` (C 11531) | the file gate; `not (feature = "use_xcb")` / `use_xcb` |
| 11548-11652 | `XTflash` (C 11551) | the file gate |
| 11654-11686 | `XTring_bell` (C 11657) | the file gate; `have_xkb`/`not (feature = "have_xkb")` for C 11677-11681 |
| 11688-11699 | the `Line Dance` banner (C 11688-11690, C's boxed comment), C 11692-11693's comment and `x_ins_del_lines` (C 11696) | the file gate; the banner and the comment are carried as comments (slice 1's `x_push_selection_request` precedent) |
| 11701-11915 | `x_scroll_run` (C 11705) | the file gate; `have_xwidgets` for C 11753-11843; `all (feature = "use_xcb", feature = "use_cairo_xcb")` for C 11845-11851 and for C 11872-11883's `else if`; `use_cairo` for C 11853-11906 |
| 11916-33125 | **the remainder** (C 11925's `x_frame_highlight` on: the frame highlight/focus hooks, the FX/pointer blanking, `XTread_socket` and the whole event loop, the toolkit and scroll-bar code, `x_initialize` and `syms_of_xterm`) | not in this slice: recorded here so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows (the next start is **C 11916**, the comment that documents `x_frame_highlight`) |

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

### 20. Slice 8 (C 4469-6989): the Xdnd senders and the self-drop path, the XInput2 device plumbing, the cairo and XRender drawing helpers, and the frame properties

The ledger's next thirty unchecked boxes, C 4469-6989, 2521 lines of C: the seven Xdnd message
senders, the five DnD bookkeeping functions, the cleanup hook, the three self-drop recorders
(`x_dnd_note_self_position`/`_wheel`/`_drop`), `x_flush`, `x_toolkit_position`,
`x_update_opaque_region`, `x_set_clip_rectangles`/`x_reset_clip_rectangles`, the five drawing
primitives (`x_fill_rectangle`, `x_clear_rectangle`, `x_draw_rectangle`, `x_clear_window`,
`x_set_frame_alpha`), `x_dpyinfo`/`x_display_info_for_display`/`x_find_topmost_parent`, and the
**forty-five** functions this file defines inside conditions the frozen table leaves false, which C
carries and the port carries too (P-51): `x_dnd_cancel_dnd_early` (`HAVE_XINPUT2`),
`x_drop_xrender_surfaces` (`HAVE_XDBE`), `x_xr_ensure_picture`, `x_xr_apply_ext_clip`,
`x_xr_reset_ext_clip` and the two `x_xrender_color_from_gc_*` (`HAVE_XRENDER`),
`x_gc_free_ext_data_private`/`x_gc_get_ext_data`/`x_extension_initialize`
(`USE_CAIRO || HAVE_XRENDER`), `xi_frame_selected_for`, `xi_convert_button_state`,
`xi_convert_event_state`, `xi_convert_event_keyboard_state`, `x_free_xi_devices`,
`xi_populate_scroll_valuator`, `xi_populate_device_from_info`, `x_cache_xi_devices`,
`x_get_scroll_valuator_delta`, `xi_device_from_id`, the five touch-point helpers and
`xi_reset_scroll_valuators_for_device_id` (`HAVE_XINPUT2` and its `_1`/`_2` splits), the cairo
block's seventeen (`USE_CAIRO`) and the three relief/wave helpers (`USE_CAIRO`).  The file-scope
objects C defines here are carried where C defines them: `OPAQUE` (C 6929, a demanded constant),
the three `FRAME_CR_*` macros (C 5289-5297), the two cairo user-data keys (C 6116-6118) and
`max_fringe_bmp`/`fringe_bmp` (C 6233-6234).

**The Xdnd senders and the three self-drop recorders.**  Each message is a `XEvent` whose
`xclient` fields C writes one by one; the port keeps C's statement order and C's own spellings,
including the `?:` C writes as the port's `if`/`else` (`x_dnd_send_enter`'s shift expression) and
the four implicit narrowings C performs into the `xm_drop_start_message` fields the record
declares as `uint32_t` (`timestamp`, `index_atom`, `source_window`: C 1604-1611), which the port
writes out (Decision 14's rule).  `x_dnd_note_self_drop`'s `ie.kind = DRAG_N_DROP_EVENT` is the
`input_event` bitfield the mirror packs into `bits`, so the port's store is the masked write
`xselect.rs` 3610 spells, and its `alloca` is the stack pool (P-05); both are the one
`macro-body-notes.tsv` row this slice appends (`docs/ports/macro-body-notes.tsv` 1269).

**Four arms the frozen reading calls dead, each carried under its feature.**  The `USE_CAIRO` GC
bookkeeping, the XRender extended clip, the cairo drawing helpers and the XInput2 device records
are all text-only work: no arm of this tree turns any of them on, so every reading below that
moves, moves in the *unit*'s own error list and nowhere else.

**Five derived feature names, each for a condition whose macro this tree cannot see** (the
`have_xcb_shape_input_rects` shape of Decision 3; the first four are C 6361-6395's cairo.h
`CAIRO_HAS_*` guards, the last two Xrender.h's `RENDER_MAJOR`/`RENDER_MINOR` pairs at C 6512,
6594 and 6752):

| C condition | the name the port carries it under |
| --- | --- |
| `#ifdef CAIRO_HAS_PDF_SURFACE` (C 6361) | `cairo_has_pdf_surface` |
| `#ifdef CAIRO_HAS_PNG_FUNCTIONS` (C 6369, 6444) | `cairo_has_png_functions` |
| `#ifdef CAIRO_HAS_PS_SURFACE` (C 6376) | `cairo_has_ps_surface` |
| `#ifdef CAIRO_HAS_SVG_SURFACE` (C 6385) | `cairo_has_svg_surface` |
| `RENDER_MAJOR > 0 \|\| RENDER_MINOR >= 2` (C 6512, 6596/6728) | `have_xrender_0_2` |
| `RENDER_MAJOR > 0 \|\| (RENDER_MINOR >= 10)` (C 6594) | `have_xrender_0_10` |

`x_cr_export_frames` is where the cairo four matter: C adds one `else if` per guard and its
trailing `abort ()` is the arm the last *live* guard reaches, so the port keeps one `if`/`else`
chain whose compile-time halves are `cfg`-defined conditions and whose bodies carry their own
`#[cfg]` (a cairo build without a surface still names no function it lacks).  `cairo_has_pdf_surface`
and its three siblings already exist in `femacs/xtask/Cargo.toml` (rows 47-50); the two
`have_xrender_0_*` names do not and are requested in this round's report.

**The flip arm's new errors are all recorded classes.**  The nine functions of the *live* part
that this slice adds call names this graph has no mirror for, which is the class slices 4-7 already
carry: Xlib's own (`XChangeProperty`, `XFree`, `XSendEvent`, `XGetWindowProperty`,
`XSelectInput`, `XDeleteProperty`, `XGetGCValues`, `XSetClipRectangles`, `XSetClipMask`,
`XFillRectangle`, `XSetForeground`, `XDrawRectangle`, `XClearWindow`, `XQueryTree`, `XFlush`,
`XGetWindowAttributes`, `XGetAtomNames`, `Unsorted`, `XGCValues`, `XWindowAttributes`) and this
file's own later slices (`x_ignore_errors_for_next_request`, `x_stop_ignoring_errors`,
`x_had_errors_p`, `x_top_window_to_frame`, `x_translate_coordinates`, `x_catch_errors`,
`x_uncatch_errors_after_check`, `x_atom_to_symbol`, `x_get_atom_name`, `x_clear_area`, all defined
at C 9137+, and `x_dpyinfo`'s two callers now resolve).  The `xclient`/`xconfigure`/`r#type` field
reads on the mirror's *opaque* `XEvent` are slice 2/5's class carried forward (the ledger
`docs/ports/include-xlib.md` owes the union's 33 members), and the one
`no field menu_bar_window on frame` is the mirror's own `have_window_system` gate, which the flip
arm does not set.

The readings are three, and the third is the one worth reading twice: **the unit's error list
grows by 98** (241 -> 339: `E0425` 151 -> 201, `E0609` 84 -> 131, `E0308` 4 -> 5) and **not one of
them is a class this ledger has not already recorded**.  The `E0308` is the interesting one: it is
*not* in this slice's text at all - `x_dpyinfo` now exists, so the type checker reaches slice 6's
`x_dnd_free_toplevels`, whose display local is the `*mut c_void` of slice 6's model
(Decision 19 clause 4), and reports the mismatch C never has (`Display *` there).  Twelve `E0425`s
go the other way: `x_dnd_get_window_proxy`, `x_dnd_get_window_proto` and `x_dpyinfo` were "cannot
find" names for slices 5-7 and are definitions now.

**What this slice does not claim.**  The whole body is under `have_x_windows`, so the frozen
`build-tu` prints `built` while compiling none of it (Decision 1); the `use_cairo`, `have_xrender`,
`have_xdbe`, `have_xinput2*` and `cairo_has_*` arms have never been compiled by anything, and the
plain flip arm is the only reading that reaches the rest.  No X11 target build, no link, no runtime
evidence: `x_set_frame_alpha`, `x_fill_rectangle` and the senders call functions this file defines
in later slices, so no X11 link of this unit is in reach this round either.  The `XEvent` member
writes (`xclient`, `xconfigure`, `r#type`) are C's shape and the mirror's owed members, not a
working XEvent.

### 21. Slice 9 (C 6990-9785): the frame-synchronization block, the update hooks, the fringe and glyph-string drawing, the toolkit colour helpers and the colour cache

The ledger's remainder row's first 2796 lines of C: the `HAVE_XSYNC` frame-synchronization block
(C 6994-7410, ten functions), `x_update_begin`/`x_update_end`/`XTframe_up_to_date` and the two
buffer-flip hooks, the two window-border hooks and `x_clear_under_internal_border`,
`x_after_update_window_line`, `x_premultiply_pixel` and `x_draw_fringe_bitmap`, the five
glyph-string GC/clipping helpers, `x_compute_glyph_string_overhangs`, the four glyph-string
drawers, the whole `#ifdef USE_X_TOOLKIT` colour block, `x_color_cells`/`x_query_colors`/
`x_query_frame_background_color`/`x_hash_string_ignore_case`/`x_parse_color`, the nearest-colour
allocator with its two wrappers, the `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT` constant, the lighter
colour and relief helpers and C 9744's three non-cairo geometry helpers.  Forty-two checklist
boxes move (`--check` **255 -> 212**), and the range table's last row becomes the new remainder.

**The `have_xsync` block is text-only, and its gate is three names.**  `HAVE_XSYNC` and
`HAVE_XSYNCTRIGGERFENCE` are both `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so
`x_if_event`, the two monotonic-time readers, `x_sync_note_frame_times`, the frame-drawn test and
wait, `x_sync_update_begin`/`_finish`, the fence trio and `x_sync_handle_frame_drawn` are all dead
in the frozen reading: they have no checklist box (P-51's reading), and no arm of this tree
compiles one.  The condition is C's three-term `#if` (`HAVE_XSYNC`, `!USE_GTK`,
`HAVE_CLOCK_GETTIME`), so the item gate is the three lowercased names joined with `all (...)`;
`have_clock_gettime` is **not** a row of `femacs/xtask/Cargo.toml` yet (the Owed list asks for it),
and the two `#ifdef FRAME_DEBUG` blocks (C 7101-7106, 7271-7273) carry `frame_debug`, which is
also unregistered - the `glyph_debug` shape `docs/ports/dispnew.md` 220-224 records for a knob
`configure` knows nothing about.  C 7271's `fprintf` is the one place this file needs
`PRIuFAST64`, which this unit's include graph does not reach (`lib/include/inttypes.in.rs` is not
on any path here); the arm keeps C's two-part format with the macro that spells it, and the gap is
in the Owed list rather than papered over.

**Five sub-gates the `## Gate mapping` rows now carry** (C's order): `have_xsynctriggerfence`
(C 7249, 7369), `frame_debug` (7101, 7271), `not (feature = "use_cairo")` for C 7818/8535/9744's
three `#ifndef USE_CAIRO` helpers, `any (not (feature = "use_gtk"), feature = "have_xfixes")` for
C 8206's `#if !defined USE_GTK || defined HAVE_XFIXES`, `gcc_lint` for C 8834's pair, `use_lucid`
inside `use_x_toolkit` for C 8900, and `debug_x_colors` for C 9506/9562.

**Three records this unit had to declare, and one it could not.**  `include/dispextern.rs` keeps
`struct face`, `glyph_string` and `draw_fringe_bitmap_params` behind `dispextern_face_records`
(or `have_window_system`), switches no configuration sets, and status 3.220's ruling is that the
units keep their **live typed copies**; this slice is the first in xterm to need them, so
`femacs/src/xterm.rs` now carries (a) `struct face` at the X11 arm's measured layout
(`docs/baseline/face-layout.tsv`: `sizeof` 296, `gc`@168, `stipple`@176, the bit run at 256) with
the `face_bits!`/`glyph_u_cmp_bits!`/`glyphless_bits!` accessors and their masks, (b)
`draw_fringe_bitmap_params` with the mirror's measured offsets (64/52/56) and the two
`get_/set_cursor_p`/`get_/set_overlay_p` readers, and (c) `glyph_string` in C's declaration order
with the six-member `glyph_string_bits!` run - and (c) carries **no** layout assertions, because
the mirror's own note records that record's layout as unmeasured on this target (the ledger
repeats it rather than inventing numbers).  The one record that could not be declared is
`XtConvertArgRec`: this machine has Xlib's headers under `/opt/homebrew/include/X11` but **not**
libXt's, so C 8955-8961's `cvt_string_to_pixel_args` initializer - the only value of that type
this file writes - is carried as C's own text in a comment and the ledger says the arm is
text-only for want of that toolchain (P-86 clause 3's reading).  `cvt_string_to_pixel_value`
(a plain `Pixel`) lands as a `static mut`.

**The five prototypes the unit declares at its use site** (P-55, the `fringe.rs` 846-850 shape):
`get_glyph_string_clip_rects` (dispextern.h 3661), `prepare_face_for_display` (3810),
`x_free_colors` (3791), `parse_color_spec` (3798), `gamma_correct` (3839) and `x_free_dpy_colors`
(xterm.h 1980); none is a row of the mirror this unit includes.  `register_color` (xfaces.c 422,
under that file's own `#ifdef DEBUG_X_COLORS`) is named and **not** declared, because nothing in
this tree's configurations compiles the arm that calls it.  `x_color_cache_bucket_size`,
`x_clear_area`, `x_wm_supports`, `x_catch_errors`, `x_had_errors_p`, `x_ignore_errors_for_next_request`,
`x_stop_ignoring_errors`, `x_uncatch_errors_after_check` and `x_check_font` are this file's own
later slices (C 11344+, 17833+, 26249+) or `#ifdef GLYPH_DEBUG` (C 30433), so the port names them
and declares none - slice 8's rule.

**The three statements the `--check` gate asked for** are `docs/ports/macro-body-notes.tsv` rows
1678 (`x_alloc_nearest_color_1`'s `goto`) and 1679 (`x_draw_fringe_bitmap`'s `cursor_p`, `goto`,
`overlay_p`); the third finding was mine and is fixed rather than noted: C's `eassert (s->gc != 0)`
now keeps C's `0` (`0 as *mut c_void`, the `w32term.rs` 2000 shape) instead of a `.is_null ()`
that lost the literal.

**The flip arm's new errors are all recorded classes.**  The plain `have_x_windows` probe grows by
**192** errors (339 -> 531 on the last measured run; the three fix batches after that run - the
`glyph_string`/`draw_fringe_bitmap_params` copies, the `(*((*s).face as *mut face))` groupings and
the accessor bodies' `unsafe` - were *not* re-measured, the round's probe budget being spent) and
every code is one the ledger already carries: `E0425` for the Xlib/Xt/XInput2/GTK names and for
this file's later slices (326 of the 531's names are in the unit's own text, 3 in the mirrors),
`E0609` for the members the mirrors keep as comments or gate elsewhere (`xclient`/`xbutton`/
`xconfigure`/`r#type` on the opaque `XEvent`, `borders_respect_alpha_background` on `frame`,
`mouse_face_defer` on `Mouse_HLInfo`, `valid` on `color_name_cache_entry`, `sie` on
`buffered_input_event`, `menu_bar_window`), `E0308` for the five `*mut c_void`-vs-`*mut _XDisplay`
sites slices 6-8 already record, and one `E0423` family that is **the mirror's**: C 8649's
`VCENTER_BASELINE_OFFSET` expands `include/dispextern.rs` 2658-2663, whose body names
`FONT_DESCENT`/`FONT_HEIGHT`/`FRAME_FONT`/`FRAME_LINE_HEIGHT` as *calls* - names this unit's graph
has as macros or not at all (10 `E0423`).  This slice is what first reaches that macro, so the
request goes to the Owed list, not to a per-function workaround.  `E0432`, `E0606`, `E0573`,
`E0608`, `E0614`, `E0133` and `E0277` all appeared in this round's intermediate runs and were
fixed in the unit (imports of names `x11_constants` does not carry; `*(x as T)` groupings that
bound the `as` too late; place-vs-pointer conventions in the two accessor families; the
`isize`/`c_int` widenings C performs implicitly).

**What this slice does not claim.**  The whole body is under `have_x_windows`, so the frozen
`build-tu` prints `built` while compiling none of it; the `have_xsync`, `have_xdbe`, `use_cairo`,
`have_xrender`, `use_x_toolkit`, `gcc_lint` and `debug_x_colors` arms have never been compiled by
anything, and the plain flip arm is the only reading that reaches the rest.  No X11 target build,
no link and no runtime evidence: `x_frame_of_widget`, `x_alloc_lighter_color_for_widget` and the
cvt pair are Xt text, the two `#ifdef FRAME_DEBUG` blocks are unbuilt by any arm, and the
`glyph_string`/`face` copies are text whose layouts this target cannot check.  `XtConvertArgRec`
is not even spellable here.

### 22. Slice 10 (C 9786-11915): the relief and box drawing, the image and glyph-string drawing, the frame clearing, the hourglass and the visible bell, and `x_scroll_run`

Thirty C blocks in the file's order (the `## C line ranges` rows 9786-11915), 27 functions and the
`Line Dance` banner.  What the slice decided, beyond "transcribe C":

1. **The two arms that change a *signature* get one definition each.**
   `x_composite_image`'s C 10110-10112 `#ifdef HAVE_XRENDER` adds a `Picture destination`
   parameter, so the unit carries two definitions under `have_xrender` and
   `not (feature = "have_xrender")`; `extract-abi.py`'s candidate loop compares the arm the
   configuration selects.  The three *call* sites that C writes with a cfg'd argument
   (`x_draw_image_foreground`'s two, `x_draw_image_foreground_1`'s one) are carried as
   `#[cfg]`'d statement pairs, the `ftfont.rs` 2552 shape - the port's first choice, since a
   Rust argument list cannot hold a conditional argument.
2. **Two conditions C writes as `if`/`else if` whose arms are configuration-dependent keep C's
   chain by nesting**: `x_clear_area`'s `#if defined HAVE_XRENDER …`/`#else` and
   `x_scroll_run`'s `#ifdef USE_CAIRO_XCB_SURFACE … #else`.  Each computes C's condition once
   (three cfg'd spellings where `#ifndef USE_GTK` and `#ifdef HAVE_XDBE` both feed it) or nests
   the selected arm, so the port has one copy of the shared body and C's control flow.
3. **The `--check` token gate's two switch functions.**  `x_fill_underline`'s
   `switch (style)` and `x_draw_glyph_string`'s `switch (s->first_glyph->type)` are `match`es,
   so `switch`/`case`/`default`/`break` cannot reappear and the `FALLTHROUGH` from the DOTS arm
   is the shared entry stage: two rows are appended to `docs/ports/macro-body-notes.tsv`
   (1705, 1706) with the `w32_fill_underline`/`haiku_fill_underline` rows' reasoning.
   `x_draw_glyph_string`'s third token, `cmp`, is C 11015's `s->first_glyph->u.cmp.automatic`,
   which the port reads through `glyph_u_cmp_bits! (automatic, …)` - a name that does not carry
   the standalone token, so that row lists it too.  The third row this slice produced,
   `x_composite_image does not mention dstx, dsty, srcx, srcy`, is answered in the text: C's
   four parameters are spelled `srcX`/`srcY`/`dstX`/`dstY` and the port keeps C's own names
   (the `include/xwidget.rs` `xvWindow` practice), which is what the token comparison asks for
   and what `non_snake_case` (already in the unit's allow list) permits.
4. **The `face` accessors this unit's copy lacked.**  Slice 9's local `face_bits!` named three
   members; C 11064-11226 reads six more (`underline`, `underline_defaulted_p`,
   `underline_at_descent_line_p`, `underline_pixels_above_descent_line` as a *member*,
   `overline_p`, `overline_color_defaulted_p`, `strike_through_p`,
   `strike_through_color_defaulted_p`), so the unit's macro grows four masked arms and the
   constants dispextern.h 1893-1935 gives them, and its `face` copy renames the slice-9
   placeholder `_pad_underline_pixels` to C's member name (a name-only change; the layout
   assertions are untouched).  The single source for both is `include/dispextern.rs`'s own
   `face_bits!` (owed - clause 3 of the Owed list).
5. **`x_clear_area` is C's one non-`static` definition in the slice**, so the unit exports it
   under C's name (`#[unsafe(no_mangle)] pub unsafe extern "C"`, the `x_dpyinfo` shape); the
   five prefix call sites that read `E0425` before this slice resolve because of it (the
   probe's one *reduction*, below).
6. **Everything this slice names and no mirror declares stays bare** (P-51/P-86's practice for
   this file): the Xlib entry points (`XSetForeground`, `XFillRectangle`, `XGetGCValues`,
   `XDrawLine`, `XSetFillStyle`, `XCopyArea`, `XChangeGC`, `XSetClipMask`,
   `XSetClipRectangles`, `XSetTSOrigin`, `XClearArea`, `XCreatePixmap`, `XMapRaised`,
   `XSetDashes`, `XSetLineAttributes`, `XUnmapWindow`, `ConnectionNumber`, `XBell`, `XFlush`,
   `XSendEvent`, `XCreateWindow`), the X11 constants (`FillSolid`, `FillOpaqueStippled`,
   `GCFunction`, `GCClipMask`, `GCClipXOrigin`, `GCClipYOrigin`, `GCLineStyle`, `GCLineWidth`,
   `GXcopy`, `GXinvert`, `LineSolid`, `LineOnOffDash`, `CapButt`, `JoinRound`, `Convex`,
   `CoordModeOrigin`, `CWCursor`, `CopyFromParent`, `InputOnly`, `Unsorted`), the two records
   (`XPoint`, `XSetWindowAttributes`), the other units' functions (`window_box`,
   `window_box_left`, `window_box_left_offset`, `window_box_right`,
   `cursor_in_mouse_face_p`, `font_for_underline_metrics`, `image_ascent`, `gui_clear_cursor`,
   `popup_activated`, `detect_input_pending`, `x_scroll_bar_clear`) and this file's own later
   slices (`x_ignore_errors_for_next_request`, `x_stop_ignoring_errors`).  The ledger's Owed
   list names them per mirror.  `flush_frame` is the mirror's *macro* (`include/frame.rs`
   1908), so its site is `flush_frame! (f)`; its body's own `redisplay_interface` gap is the
   mirror's (clause 4 of the Owed list).
7. **C's `switch`-free text stays C's**: the `s->first_glyph->type` walks, `s->img->mask`
   null tests (`XEmacs_Pixmap` is the mirror's `*mut c_void` for `pixmap`/`mask`, so the test is
   `.is_null ()`), the `unsigned long` underline arithmetic (C's wraps written with
   `wrapping_sub`, the `haikuterm.rs` 1400-1520 reading of the same code), and the
   `memset (&attrs, 0, sizeof attrs)` shapes (the unit's line 8560 spelling).

### 23. Slice 11 (C 12066-12095): `make_invisible_cursor`, and the two deferrals beside it

Slice 10 ended at C 11915; the next text that needs no bitfield machinery is `make_invisible_cursor`
(C 12067-12095).  Its four Xlib names (`XCreatePixmapCursor`, the `XColor.flags` bits
`DoRed`/`DoGreen`/`DoBlue`, `XGetErrorText` before slice 12's mirror round) stand at the use site
with their headers' lines (P-88 1), and `None` is `include/xlib.rs`'s `x11_constants::None`.
**Deliberately not in the slice, for measured reasons**: `x_toggle_visible_pointer` (C 12097-12139),
`xfixes_toggle_visible_pointer` (C 12053-12062) and `XTtoggle_invisible_pointer` (C 12141-12153) all
store `f->pointer_invisible`, a `frame` `bool_bf` whose shift numbering `include/frame.rs` carries
for the frozen arm only (`src/androidterm.rs` 214-217 records the same bound), and
`x_fixes_pointer_blanking_supported` (C 12040-12045) reads `dpyinfo->xfixes_supported_p` /
`xfixes_major`, which `include/xterm.rs` does not carry.  Their boxes stay unchecked on purpose.

### 24. Slice 12 (C 26217-26347): the error-handling block's first half

The record `x_error_message_stack` (C 26217-26239) with `x_special_error_handler` declared at the
use site (xterm.h 1748-1749), the `x_error_message` pointer and the **exported**
`x_error_message_count` (C 26249/26252, PORTING.md 0.2), the four serial comparators (C's
`unsigned int` wrap written `wrapping_sub`, its widening `as c_ulong`, and the 32-bit truncation of
64-bit serials written `as c_uint` at the call sites - C 26257-26258 says the functions do that),
`x_find_error_handler`, `x_unwind_errors_to` and `x_error_catcher` (whose `buf` is zeroed, Deviation
12).  The block's second half is deferred to slice 13 because it needs the request-serial
expressions (Decision 25).

### 25. Slice 13 (C 26349-26689): four shapes, one mirror round and two move-backs

* **The request-serial macros are degraded, not re-implemented**: C's `NextRequest` /
  `LastKnownRequestProcessed` read `_XPrivDisplay`'s `request` / `last_request_read`, the private
  display record this tree does not carry (the `struct _XDisplay` whose definition lives in
  `<X11/Xlibint.h>`), so `include/xlib.rs` keeps C's macro names with `XNextRequest` /
  `XLastKnownRequestProcessed` (Xlib.h 1806/1809) for their bodies - the same degradation class as
  `DefaultScreenOfDisplay`'s row above, and the same reading obligation: the equivalence is that
  those procedures return the values the C macros compute.
* **The `failable_requests` field is an array**: `struct x_display_info` carries
  `[x_failable_request; N_FAILABLE_REQUESTS]`, so C's pointer decay is written
  `(*dpyinfo).failable_requests.as_mut_ptr ()` at its six uses, and `next_failable_request` stays the
  pointer it is in C.
* **The call sites of a newly defined name are part of the slice**: `x_had_errors_p` returns C's
  `bool`, so the nineteen earlier `… == 0` / `… != 0` sites become `!x_had_errors_p (…)` /
  `x_had_errors_p (…)`, and the five `XSync (…, False)` sites pass `0` because the parameter is
  Xlib's `Bool` (`c_int`).  Both are spellings of C's own truth tests, not behaviour changes.
* **Two P-90 5 move-backs and one mirror row retired**: `src/xfns.rs` 1954-1955's `XNextRequest`
  and `src/gtkutil.rs` 258-261's `XSync` retire into `include/xlib.rs` (they were
  `check-mirror-dupes.py`'s two failures), and `include/xterm.rs`'s `x_error_message_count` row
  retires because the defining unit is its includer (P-90 7; it was slice 12's one `E0428`, which
  that slice's class counts did not include - recorded here rather than smoothed over).

### 26. Slice 14 (C 26704-26992): `x_connection_closed`, and three shapes it fixes rather than follows

* **`error_msg` is the stack pool**, not a heap buffer: C 26766 hands a **file-scope** pointer an
  `alloca` result, which is sound only because C 26991's `error` is a non-local exit; the port keeps
  the pointer file-scope and takes the bytes from `sa_stack_alloc` (P-05), the same spelling slice
  13's `x_check_errors` uses.
* **The two `_Noreturn` XIO handlers take C's `int` return.**  The earlier slices wrote them `-> !`
  (a Rust convenience), which cannot be passed to `XSetIOErrorHandler`'s
  `Emacs_XIOErrorHandler` parameter (`Option<unsafe extern "C" fn (*mut c_void) -> c_int>`, unit
  701).  C declares them `_Noreturn int`; the port keeps the `int` and their diverging tails
  (`emacs_abort ()`, `siglongjmp` + `unreachable! ()`), which is what C 26783 and the motif path's
  C 1803 need.  The reading that named it is this slice's own two `E0308`s.
* **C's two backward `goto`s are labelled `loop`s** (`'delete_more_minibuffer_frames`,
  `'delete_more_frames`) with `continue 'label` for the jump and `break` after the frame walk
  (PORTING.md 2.2, the C 1960 `'retry_drag_window` idiom this file already uses), and
  `dpyinfo->failable_requests` keeps the `.as_mut_ptr ()` decay slice 13's reading established.

### 27. Slice 15 (C 26994-27126): the first-level error handler, and five shapes it fixes

* **C 851/858's two members were in the wrong record.**  `supports_xi2` and `xi2_opcode` belong to
  `struct x_display_info` (C 851/858, the `#ifdef HAVE_XINPUT2` run that follows C 847's
  `supports_xdbe`), and the first draft put their rows in `x_output`, because C's `x_output`
  carries an `XIEventMask *xi_masks` of its own (C 1353) and the two names sit next to it.  The
  reading that told them apart is the probe's own `E0609 no field supports_xi2 on type
  x_display_info`: the compiler named the record the *use site* hands over.  They now stand in
  `x_display_info` at C's own position, after the `use_xcb` pair (C 841-844), and the mirror's
  comment names the members of that run still owed (`supports_xdbe` C 847, `xi2_version` C 855,
  `num_devices` C 861, `devices` C 865, the three `pending_keystroke_*` C 868-880 and
  `client_pointer_device` C 885).
* **C's integer promotions are written out at each comparison.**  `event->error_code` and
  `event->minor_code` are `unsigned char` fields while `dpyinfo->xi2_opcode` is `int`, so C
  compares after the usual arithmetic conversions: the port writes `BadMatch as c_uchar` /
  `X_SetInputFocus as c_uchar` against the byte fields (C 27007-27009) and
  `(*event).request_code as c_int` against the `int` one (C 27059 - the `E0308 expected u8, found
  i32` that reading produced is exactly the widening C hides).
* **C 869-881's three request codes are macros and are invoked, not read** (`X_XIGrabDevice! () as
  c_uchar`, C 27060-27062): the first draft wrote the bare name and the arm answered `E0423 cannot
  find value X_XIGrabDevice`.
* **A `static` C function carries no `#[unsafe(no_mangle)]`.**  Slice 14's `x_connection_closed`
  had one although C 26743 spells the definition `static`; `--check` reads it as an invented
  exported symbol (`fn x_connection_closed carries #[unsafe(no_mangle)] but the reference does not
  define a non-static function of that name`), and the row is removed here (PORTING.md 0.2's rule
  for internal linkage - `static` is neither `pub` nor exported).
* **The four X11 names are use-site measurements** (P-72 rule 4, the `src/xselect.rs` 136-138
  precedent): `BadWindow`/`BadMatch`/`BadName` are X.h 353/358/373 and `X_SetInputFocus` is
  Xproto.h 2063, each read on this machine's xorgproto 2025.1 and each carrying its citation.  The
  owed move is `include/xlib.rs`'s `x11_constants` module - the home `docs/ports/xselect.md`'s Owed
  11 already names for its own copy of this surface - and that move should carry a re-measurement of
  `xselect.rs` 136's `<X11/Xproto.h> 110: #define BadWindow 3L`, which this machine's `Xproto.h`
  does not define (it names no `Bad*` code at all; `X.h` 353 is where `BadWindow` lives).

### 28. Slice 16 (C 27128-27194): `x_new_font`, and three shapes the mirror owed

* **The frame.h accessors C *writes* need the second macro arm.**  `include/frame.rs` carries the
  getters of `FRAME_COLUMN_WIDTH`, `FRAME_LINE_HEIGHT`, `FRAME_MENU_BAR_HEIGHT`,
  `FRAME_TAB_BAR_HEIGHT` and `FRAME_CONFIG_SCROLL_BAR_COLS`; C 27151-27171 assigns to all five, so
  the unit declares the two-arm macros the `src/pgtkterm.rs` 312-336 block already uses for the
  same five names (`FRAME_X (f)` is the getter, `FRAME_X! (f, v)` the write).
* **`font->baseline_offset` lives behind the mirror's own `have_window_system`.**  font.h's
  `int baseline_offset` (font.h 368) sits inside the header's `#if defined HAVE_WINDOW_SYSTEM`
  block, and `include/font.rs` gates that whole run; C's xterm.c reads it with no guard of its own,
  because configure defines `HAVE_WINDOW_SYSTEM` for every window-system build (configure.ac 7517).
  The write therefore carries `#[cfg (feature = "have_window_system")]` on a *block* - a statement
  attribute is what Rust accepts, not one on an expression (`E0658 attributes on expressions are
  experimental` is the reading that said so) - the `src/xfns.rs` 1595-1597 shape, and its comment
  records the guard.  The arms do **not** simply add the feature: with `have_window_system` on,
  `include/dispextern.rs` defines eight names twice (`glyph_string` and the seven
  `GLYPH_STRING_*` constants at 1608/1776-1788), which is an owed mirror defect, measured (that arm
  reads `E0428 x8` and drops every other class).
* **A `#[cfg]` above a comment-only row leaks onto the next item.**  Fifteen rows of
  `include/xterm.rs` (C's bitfields, carried as `// C: bool_bf … : 1;` lines) had their guard on a
  `#[cfg]` line of their own; a comment is not an item, so the attribute attaches to whatever
  follows - which is how `pub xic: XIC` ended up requiring `have_xdbe` as well as `have_x_i18n`
  (the arm-D reading's `E0609 no field xic`, and arm E - the same arm plus `have_xdbe` - reading
  clean is what proved it).  All fifteen now fold their guard into the comment text
  (`// C guard (feature = "have_xdbe"): bool_bf … : 1;`), so no attribute can leak; the four other
  files with the same shape (`femacs/lib/file-has-acl.rs` 1, `include/dispextern.rs` 1,
  `src/pgtkselect.rs` 1, `src/w32xfns.rs` 1) are owed to their own lanes, and the whole-tree count
  is 19 rows in 5 files.
* **The XIM surface stands at the use site.**  `include/xterm.rs`'s `struct x_output` rows
  (`xic`, `xic_style`, `xic_xfs`), `x_display_info`'s (`xim`, `xim_styles`) and its five `STYLE_*`
  constants take `<X11/Xlib.h>`'s types (1090-1167) and masks (1169-1177), and no mirror of this
  unit's graph declared any of them - so the `have_x_i18n` arm carried twenty `E0425` rows plus one
  `E0308` (`STYLE_OVERTHESPOT`'s `XIMStyle` against the two `long`-typed declarations the first
  attempt made).  The five types and the six masks are this unit's use-site declarations (P-72
  rule 4, the `src/xfns.rs` 3835-4140 block's shape), measured against the installed Xlib
  (libx11 1.8.13) and typed `XIMStyle` because that is the type their only users take (the
  `src/xfns.rs` 4112-4135 spelling).  They are **not** in the mirror this round: `src/xfns.rs`
  includes that mirror and carries its own copies, so a mirror row would be a second definition in
  its crate (P-90 ⑦) - the owed move is one commit that puts both copies into `include/xlib.rs`'s
  X11 SDK section.

* **A `#[cfg]` above a comment-only row leaks onto the next item.**  Fifteen rows of
  `include/xterm.rs` (C's bitfields, carried as `// C: bool_bf … : 1;` lines) had their guard on a
  `#[cfg]` line of their own; a comment is not an item, so the attribute attaches to whatever
  follows - which is how `pub xic: XIC` ended up requiring `have_xdbe` as well as `have_x_i18n`
  (the arm-D reading's `E0609 no field xic`, and arm E - the same arm plus `have_xdbe` - reading
  clean is what proved it).  All fifteen now fold their guard into the comment text
  (`// C guard (feature = "have_xdbe"): bool_bf … : 1;`), so no attribute can leak; the four other
  files with the same shape (`femacs/lib/file-has-acl.rs` 1, `include/dispextern.rs` 1,
  `src/pgtkselect.rs` 1, `src/w32xfns.rs` 1) are owed to their own lanes, and the whole-tree count
  is 19 rows in 5 files.
* **The XIM surface belongs to the mirror that names it.**  `include/xterm.rs`'s `struct x_output`
  rows (`xic`, `xic_style`, `xic_xfs`) and `x_display_info`'s (`xim`, `xim_styles`) take
  `<X11/Xlib.h>`'s types, and its five `STYLE_*` constants take Xlib.h 1169-1177's masks - none of
  which any mirror declared, so the `have_x_i18n` arm carried twenty `E0425` rows plus one
  `E0308` (`STYLE_OVERTHESPOT`'s `XIMStyle` against undeclared `long` masks).  The five types and
  the six masks now stand in that mirror, measured against the installed Xlib (libx11 1.8.13), with
  the masks typed `XIMStyle` (the type C's own use site converts them to, the `src/xfns.rs`
  4112-4135 spelling); the owed move is `include/xlib.rs`'s X11 SDK section, together with
  `xfns.rs`'s copies.  The unit's own XIM arm then reads the masks directly, with no cast.


### 29. Slice 17 (C 27195-27284): the XIM block's first two functions, and `XFree`'s prototypes

* **The three nested guards are three features.**  C 27200/27202/27210 test `HAVE_X_I18N`,
  `HAVE_X11R6` and `HAVE_X11R6_XIM`; the first two are `undef` in the frozen table
  (`docs/baseline/config/darwin-arm64.macros.tsv` 1686-1687), so both functions are **dead text**
  there - which is why `--check` never listed them as missing (P-66's evaluator) and why this
  slice's `--check` reading does not move.
* **Declaring `XFree` turned sixteen unresolved rows into sixteen call sites that needed C's own
  conversion.**  The unit had sixteen `XFree (…)` calls whose name nothing declared (sixteen
  `E0425`); declaring Xlib's `int XFree (void *)` at the use site resolved all sixteen and, in the
  same reading, named fifteen of them as `E0308` - C's `void *` parameter accepts any object
  pointer and Rust's does not, so each site now writes `as *mut c_void` (the `src/xterm.rs` 4113 /
  5186 / 13429 sites already spelled it).  That is the whole `-16 E0425` this slice's arm F reads.
* **A doc comment on an `unsafe extern "C"` block is an unused doc comment.**  The first layout put
  the two use-site declarations under the function's `///` block; rustc answered
  `unused doc comment` (a warning, and this file's warning count is part of every reading), so the
  block stands above the comment.
* **`EVENT_INIT` takes the zeroed `input_event`** (Deviation 12, the `src/pgtkterm.rs` 7714
  shape), `PREEDIT_TEXT_EVENT` is written through this unit's `input_event_bits!` (the C 1416-1417
  spelling), and `FRAME_X_OUTPUT!`'s union member is cast at the use site (the C 7428 shape).


### 30. Slice 18 (C 27286-27342): `xim_open_dpy`, and `XFree`'s declaration moved to every X arm

* **`XFree` belongs to every X arm, not to one gate.**  Slice 17 declared Xlib's
  `int XFree (void *)` under its `HAVE_X11R6_XIM` arm because that is where it first met the name;
  the unit's X text frees Xlib memory in sixteen places that are *not* behind that arm, so the
  declaration now stands under `have_x_windows` alone.  That is what moves every arm this slice
  reads: A **563 -> 547**, B **563 -> 547**, C **591 -> 575**, D **564 -> 548**, each `-16 E0425`
  and nothing else, because the sixteen calls were unresolved names.  The sixteen also had to
  spell C's conversion (`as *mut c_void`), which slice 17's reading had already named for fifteen
  of them.
* **Both arms of `HAVE_XIM` are carried.**  The macro is `undef` in the frozen table
  (`darwin-arm64.macros.tsv` 1700), so C's own `xim_open_dpy` body there is the two lines around
  the guard - `dpyinfo->xim = NULL;` - and the port carries the live text as the
  `not (feature = "have_xim")` arm beside the `have_xim` one (P-86 ①'s rule, the
  `src/gtkutil.rs` shape).
* **The XIM surface this function needs is use-site, and the `XN*` names are functions.**  `Status`
  is Xlib.h 79's `#define Status int` (the `src/xmenu.rs` 477 spelling); `XIMProc` and
  `XIMCallback` are Xlib.h 1144-1148 and 1230-1234 (`sizeof 16 align 8`, offsets `client_data 0`
  `callback 8`, the `src/xfns.rs` 3883-3895 measurement); `XNQueryInputStyle`/`XNDestroyCallback`
  are Xlib.h 1180/1187 and are *functions* returning `*mut c_char`, because that is what they are
  passed as (the `src/xfns.rs` 4171/4177 pair of the same macro family); and the five entry points
  (Xlib.h 3836-3861) are declared here since none is in `include/xlib.rs` yet.
* **A variadic call's trailing `NULL` is typed.**  C passes `NULL` as `XGetIMValues`' last
  argument; the port's first attempt wrote `core::ptr::null_mut ()` and rustc answered
  `E0283 type annotations needed` - the spelling is `std::ptr::null_mut::<c_void> ()` (the
  `src/xfns.rs` 4960 site).


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

### Slice 8's readings (C 4469-6989), as landed and measured (2026-10-05)

Commands verbatim; the before readings were taken on the tree as it stood when the slice opened.

```
$ python3 tools/check-cond-eval.py src/xterm.c          # before and after, unchanged
src/xterm.c: 852 conditions, 0 unknown, 0 disagreements [VERIFIED]
```

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # before
286
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ grep -c '^- \[ \]' docs/ports/xterm.md      # before: 297 (30 of them this slice's)
297
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # after
255
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ grep -c '^- \[ \]' docs/ports/xterm.md      # after: 267
267
# the rows that moved: every "missing function"/"missing constant OPAQUE" line of C 4469-6989 is
# gone (30 rows), and one row is *new*:
#   - line count drifted 86.3%: C 33125 vs Rust 4525   ->   - line count drifted 77.3%: C 33125 vs Rust 7508
```

```
$ python3 tools/extract-abi.py --all emacs/src/xterm.c femacs/src/xterm.rs     # after (the ledger's
                                                                               # WIP reading)
WIP  src/xterm.c -> femacs/src/xterm.rs (255 differences, docs/ports/xterm.md has unchecked boxes)
...
checked 590 ported files: 565 done, 3 in progress, 22 failing
# this file stays one of the three "in progress" rows (the ledger is still 267 ticks short).  The
# tree-wide counts are *not* this slice's: the before reading of the same command said
# `532 done, 3 in progress, 55 failing`, and the 33 files between the two readings are other
# lanes' in-flight edits (`femacs/src/image.rs`, `femacs/src/sfnt.rs`, `femacs/src/w32term.rs`,
# `femacs/xtask/src/main.rs`), which were mid-write when the before reading ran.
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm-lane/target cargo run -q -p xtask -- build-tu src/xterm.c
warning: 22 warnings emitted
built src/xterm.rs -> ../build/femacs/xterm.o
# the 22 warnings are the mirrors' own (unused_attributes, unreachable_code); the frozen
# configuration compiles none of this slice.
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm-lane/target cargo run -q -p xtask -- probe \
      src/xterm.rs /tmp/xterm-lane/x.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="have_x_windows"'
rustc failed for src/xterm.rs (exit status: 1)
# before: error: aborting due to 241 previous errors
#   151 E0425   84 E0609   4 E0308   1 E0381   1 E0080
# after:  error: aborting due to 339 previous errors
#   201 E0425  131 E0609   5 E0308   1 E0381   1 E0080
# by file unchanged: _root.rs (the unit, src/xterm.rs) and one E0080 in include/systime.rs
```

The delta, by name and by class - the table Decision 20 reads:

```
# E0425: 62 inside this slice's lines, every one a recorded class
#   Xlib names with no mirror: XChangeProperty 5, XFree 5, XSendEvent 4, XWindowAttributes 2,
#   XGetWindowProperty 2, XDeleteProperty 2, XFillRectangle 2, XSetForeground 2, Unsorted 1,
#   XGCValues 1, XGetWindowAttributes 1, XSelectInput 1, XGetGCValues 1, XGetAtomNames 1,
#   XDrawRectangle 1, XSetClipRectangles 1, XSetClipMask 1, XClearWindow 1, XQueryTree 1, XFlush 1
#   this file's later slices: x_ignore_errors_for_next_request 5, x_stop_ignoring_errors 5,
#   x_had_errors_p 3, x_top_window_to_frame 3, x_translate_coordinates 3, x_catch_errors 2,
#   x_uncatch_errors_after_check 2, x_atom_to_symbol 1, x_get_atom_name 1, x_clear_area 1
# and 12 go away: x_dnd_get_window_proto 7, x_dnd_get_window_proxy 4, x_dpyinfo 1
# E0609: 47 inside this slice's lines
#   xclient on XEvent 43, xconfigure on XEvent 2, r#type on XEvent 1   (the opaque-XEvent class)
#   menu_bar_window on frame 1                                        (the mirror's own
#                                                                       have_window_system gate)
```

```
$ python3 tools/check-dead-file-coverage.py
143 ledger(s) bound (a C range table), 0 failure(s)
$ python3 tools/check-ledger-ticks.py
238 ledger(s) bound (a function tick to compare), 0 failure(s), 0 carried by another file,
352 row(s) with no function tick - a unit with no function at all, or one whose body the frozen
configuration leaves under a false condition (check-port-lines.py is that class's gate),
0 row(s) without a ledger
$ python3 tools/check-citations.py
check-citations: 676 file(s) scanned, 0 drift, 0 range, 1814 external, 36 allowed, 0 stale
$ python3 tools/check-declarations.py
17 declaration sites (0 forbidden, 17 allowed literals), 0 failures
$ python3 tools/check-mirror-dupes.py
17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)
# the ratchet read 18 rows when round 18 landed (this ledger's Decision 19 note); the move to 17
# is commit `4b8decec`'s (the two stale `euidaccess` rows), not this slice's - no row of the
# ratchet names this file, before or after.
```

The three files' own contributions to the tree-wide readings are the append to
`docs/ports/macro-body-notes.tsv` (one row, 1269) and nothing else: `emacs/**` was read only,
``femacs/src/xterm.rs`` and this ledger are the writing set, and the tree's other modified files
(`femacs/src/image.rs`, `femacs/src/sfnt.rs`, `femacs/src/w32term.rs`, their ledgers and
`femacs/xtask/`) belong to lanes that were in flight while this round ran.

### Slice 9's readings (C 6990-9785), as landed and measured (2026-10-05)

The `--check` reading of the round, with the before copy in `/tmp` as the checkpoint procedure
asks:

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # before
255
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # after
212
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
# the rows that moved: the 41 missing-function rows of C 6990-9785, the `missing macro`/`missing
# constant` pair of HIGHLIGHT_COLOR_DARK_BOOST_LIMIT (43 removals), and three *new* rows the
# slice's own text produced - `fn x_alloc_nearest_color_1 does not mention goto`,
# `fn x_draw_fringe_bitmap does not mention cursor_p, goto, overlay_p`,
# `fn x_set_mouse_face_gc does not mention 0` - of which the first two are answered by the two
# `docs/ports/macro-body-notes.tsv` rows this round appends (1678, 1679) and the third by keeping
# C's own `0` in the `eassert` (255 - 43 + 3 - 3 = 212).
# the drift row changed with the file: `line count drifted 77.3%: C 33125 vs Rust 7508` ->
# `line count drifted 66.9%: C 33125 vs Rust 10952`.
```

```
$ python3 tools/check-port-lines.py
0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new, 0 stale
$ python3 tools/check-dead-file-coverage.py
info docs/ports/xterm.md: 104 range row(s) cover src/xterm.c (33125 lines; 49 blank line(s)
between blocks need no row)
info docs/ports/xterm.md: 104 range row(s) cover src/xterm.h (2045 lines; 0 blank line(s) ...)
145 ledger(s) bound (a C range table), 0 failure(s)
$ python3 tools/check-ledger-ticks.py
238 ledger(s) bound (a function tick to compare), 0 failure(s), 0 carried by another file,
352 row(s) with no function tick ..., 0 row(s) without a ledger
# the 42 ticks this round adds (41 functions + HIGHLIGHT_COLOR_DARK_BOOST_LIMIT) are all in the
# unit's text; the 352 untickable rows are the class that gate describes (empty units and files
# whose body the frozen configuration leaves false).
$ python3 tools/check-citations.py
check-citations: 676 file(s) scanned, 0 drift, 0 range, 1905 external, 36 allowed, 0 stale
# the 24 external citations this slice adds are its `/// C:` lines for the two dispextern.h
# prototypes and the Xlib/Xt names (`docs/ports/include-*`'s own accounting).
$ python3 tools/check-declarations.py
17 declaration sites (0 forbidden, 17 allowed literals), 0 failures
$ python3 tools/check-mirror-dupes.py
17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)
# no row of the ratchet names this file, before or after, and the unit's three new records are
# `struct` definitions, which that gate never reports.
```

The one compile-reading that the frozen configuration gives, and the probe:

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm-lane/target cargo run -q -p xtask -- build-tu src/xterm.c
warning: 22 warnings emitted
built src/xterm.rs -> ../build/femacs/xterm.o
# the same twenty-two mirror warnings slice 8's reading carries; the frozen configuration
# compiles none of this slice.  Five runs in this round, not three: two aborted on this slice's
# own syntax (`E0758` for a comment that spelled C's `*\/` as if Rust escaped slashes, and the
# `if`/`else` split across a `#[cfg]` block in `x_draw_fringe_bitmap`), and three were clean.

$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm-lane/target cargo run -q -p xtask -- probe \
      src/xterm.rs /tmp/xterm-lane/x.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="have_x_windows"'
rustc failed for src/xterm.rs (exit status: 1)
# before: error: aborting due to 339 previous errors
#   201 E0425  131 E0609    5 E0308   1 E0381   1 E0080
# last measured after (the round's third probe run):
#   error: aborting due to 531 previous errors
#   330 E0425  146 E0609   10 E0423   8 E0308   1 E0381   1 E0080
# by file: E0425 - 327 in the unit's own text, 2 in include/xterm.rs, 1 in include/frame.rs;
#   E0609 - all 146 in the unit's text; E0423 - 6 in include/dispextern.rs, 4 in
#   include/xterm.rs (the mirror macro of the paragraph below).
# the delta's classes, each already in this ledger: Xlib/Xt/XInput2/GTK names with no mirror
# (XPoint, XFillPolygon, Convex, CoordModeOrigin, FillOpaqueStippled, FillSolid,
# GCGraphicsExposures, GCLineWidth, GCStipple, GCClipMask, GCClipXOrigin, GCClipYOrigin,
# XCreateGC/XChangeGC/XGetGCValues' record, XdbeSwapInfo, XParseColor, XAllocColor, XFreeColors,
# XQueryColors, XDisplayCells, WhitePixelOfScreen, BlackPixelOfScreen, DisplayOfScreen,
# XCreateWindow, XDestroyWindow, XDrawLine, XDrawPoint, XFillRectangle, XSetFillStyle,
# XSetWindowAttributes, XCreatePixmapFromBitmapData, XCopyArea, XFreePixmap, XChangeGC,
# Status, XSetWindowAttributes, GtkWidget/GdkWindow/GdkFrameClock and the gtk/gdk calls); this
# file's own later slices (x_catch_errors, x_had_errors_p, x_ignore_errors_for_next_request,
# x_stop_ignoring_errors, x_uncatch_errors*, x_window_to_frame, x_top_window_to_frame,
# x_translate_coordinates, x_atom_to_symbol, x_get_atom_name, x_clear_area, x_wm_supports,
# x_get_window_below, x_tooltip_window_to_frame, x_handle_pending_selection_requests,
# x_push_selection_request, request_sigio/unrequest_sigio, x_check_font); and other units'
# (lookup_basic_face, note_mouse_highlight, gui_intersect_rectangles, x_clip_to_row, xstrcasecmp);
# E0609's members the mirrors comment out or gate elsewhere (`xclient`/`xbutton`/`xconfigure`/
# `r#type` on `XEvent`, `borders_respect_alpha_background` on `frame`, `mouse_face_defer` on
# `Mouse_HLInfo`, `valid` on `color_name_cache_entry`, the pre-existing `sie`/`menu_bar_window`);
# and E0423's ten, which are **the mirror's**: C 8649's `VCENTER_BASELINE_OFFSET` expands
# `include/dispextern.rs` 2658-2663, whose body names FONT_DESCENT/FONT_HEIGHT/FRAME_FONT/
# FRAME_LINE_HEIGHT as calls that this unit's graph has as macros or not at all.
# NOT re-measured: the three fix batches after that run (the two record copies, the
# `(*((*s).face as *mut face))` groupings, the accessor bodies' `unsafe` and the `0` in the
# `eassert`) - the round's probe budget (three runs) was spent, so the number above is the last
# reading and the fix batches' effect on it is unmeasured.  The `--check` and the six gates
# above were re-run after every batch.

```

### Slice 10's readings (C 9786-11915), as landed and measured (2026-10-05)

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm2-lane cargo run -q -p xtask -- build-tu src/xterm.c
warning: 30 warnings emitted
built src/xterm.rs -> ../build/femacs/xterm.o          # exit 0
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm2-lane cargo run -q -p xtask -- build-tu src/xterm.c
warning: 31 warnings emitted
built src/xterm.rs -> ../build/femacs/xterm.o          # exit 0, after the fix batch below
# the warnings are the mirrors' own (unused_doc_comments, unused_parens, unused_attributes,
# unreachable_code, suspicious_runtime_symbol_definitions); the frozen configuration compiles
# none of this slice, so build-tu proves parsing and nothing else here.  Two extra
# `rustc -Zparse-crate-root-only src/xterm.rs` runs (syntax only, no artifact, so not a
# build-tu reading) were the ones that caught this slice's first text slip: C's
# `msg.xclient.type` is `r#type` in Rust.
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm2-lane cargo run -q -p xtask -- probe \
      src/xterm.rs /tmp/xterm2-lane/x.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="have_x_windows"'
rustc failed for src/xterm.rs (exit status: exit status: 1)
# before this slice (the checkpoint the round started from):
#   error: aborting due to 508 previous errors
#     313 E0425  161 E0609   17 E0614   13 E0308   1 E0606   1 E0381   1 E0277   1 E0080
#   by file: _root.rs 502, include/xterm.rs 4, include/frame.rs 1, include/systime.rs 1
# first after (this slice as it was first written):
#   error: aborting due to 658 previous errors
#     439 E0425  176 E0609   17 E0614   21 E0308   1 E0423   1 E0606   1 E0381   1 E0277   1 E0080
# last after (after the fix batch below):
#   error: aborting due to 636 previous errors
#     436 E0425  166 E0609   17 E0614   13 E0308   1 E0606   1 E0381   1 E0277   1 E0080
#   by file: _root.rs 629, include/xterm.rs 4, include/frame.rs 2, include/systime.rs 1
# the whole delta, and it is arithmetic: _root.rs 502 + 132 (this slice's own lines) - 5 (the five
# prefix `x_clear_area` call sites this slice's definition resolves - the reading's one reduction)
# = 629; include/frame.rs 1 + 1 = 2 (the mirror's `flush_frame!` body, whose own
# `redisplay_interface` gap is named by this slice's one call to it); this slice contributes no
# E0614, no E0308, no E0423, and moves no other error.
```

The 132 errors inside this slice's lines, by class (the `--check` and the six gates below were
re-run after every fix batch):

```
# E0425 127
#   Xlib entry points with no mirror row, 68: XSetForeground 14, XFillRectangle 9,
#     XGetGCValues 7, XDrawLine 6, XSetFillStyle 6, XCopyArea 5, XChangeGC 4, XSetClipMask 2,
#     XSetClipRectangles 2, XSetTSOrigin 2, XClearArea 1, XCreatePixmap 1, XMapRaised 1,
#     XSetDashes 1, XSetLineAttributes 1, XUnmapWindow 1, ConnectionNumber 1, XBell 1,
#     XFlush 1, XSendEvent 1, XCreateWindow 1  (the class slices 1-9 record)
#   X11 constants with no mirror row, 32: GCFunction 3, FillOpaqueStippled 3, FillSolid 3,
#     GCClipMask 2, GCClipXOrigin 2, GCClipYOrigin 2, GXcopy 2, GCLineStyle 2, GCLineWidth 2,
#     Unsorted 2, LineSolid 2, CapButt 1, JoinRound 1, LineOnOffDash 1, CWCursor 1,
#     InputOnly 1, CopyFromParent 1, GXinvert 1  (Decision 15's rule; each is in `x11_constants`
#     or in the owed list below)
#   X11/Xt records with no mirror row, 1: XSetWindowAttributes 1 (XPoint 6 is pre-existing)
#   other units' functions with no usable row in this graph, 24: get_glyph_string_clip_rect 6,
#     image_ascent 3, gui_intersect_rectangles 3, window_box 2, window_box_right 2,
#     window_box_left 1, window_box_left_offset 1, cursor_in_mouse_face_p 1,
#     font_for_underline_metrics 1, gui_clear_cursor 1, popup_activated 1,
#     detect_input_pending 1, x_scroll_bar_clear 1  (the first two are the mirror's own
#     `have_window_system` gate: include/dispextern.rs's `dispextern_late_decls` rows exist but are
#     not selected by this arm)
#   this file's own later slices, 2: x_ignore_errors_for_next_request 1, x_stop_ignoring_errors 1
# E0609 5
#   `xclient` on the opaque XEvent: 5  (C 11435-11443's five member writes in
#   `x_send_hourglass_message`; the `r#type` write is one of them, and the 33-member union
#   `docs/ports/include-xlib.md` owes is the single fix)
```

The fix batch the last reading follows (four items, each found by the probe and by nothing else -
the syntax gate sees none of them): `glyph_string_bits! (for_overlaps, …)` is a 3-bit field and
therefore an `c_int`, so its eight condition sites compare with 0; `x_draw_xwidget_glyph_string`
takes the mirror's `*mut c_void`; `XCreateGC`'s `XGCValues *` argument is the declared
`*mut c_void` (the same cast the prefix's own sites write); and `flush_frame` is the mirror's
macro, not a function (`flush_frame! (f)`).  Three further items are name-level and are *not*
re-measured by the probe: `x_composite_image`'s four parameters keep C's own spelling
(`srcX`/`srcY`/`dstX`/`dstY`, what `--check`'s token comparison asks for), `x_scroll_run`'s
XWIDGETS arm keeps C's own shadowing `int y = view->y + …` (`let mut y`, the C 11803 name), and
the unit's `face` copy renames slice 9's `_pad_underline_pixels` placeholder to C's
`underline_pixels_above_descent_line`.  None of the three can move an error count or kind (two
renames of already-used bindings and a field rename with the same type and offset), the `--check`
and the six gates were re-run after each, and the round's probe budget (three runs) was spent on
the two readings above.  The last probe reading therefore predates those three edits by name only.

The other two gates, before and after:

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # before
212
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # after
185
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
# the rows that moved: 27 missing-function rows of C 9786-11915, minus the three *new* rows this
# slice's own text produced - `function x_composite_image does not mention dstx, dsty, srcx,
# srcy` (answered in the text: C's parameter names are kept), `function x_fill_underline does not
# mention break, case, default, fallthrough, switch` and `function x_draw_glyph_string does not
# mention break, case, cmp, default, switch` (answered by the two `macro-body-notes.tsv` rows this
# slice appends, 1705-1706).  212 - 27 + 3 - 3 = 185.
# the drift row changed with the file: `line count drifted 66.9%: C 33125 vs Rust 10952` ->
# `line count drifted 59.1%: C 33125 vs Rust 13538`.
```

```
$ python3 tools/check-port-lines.py
0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new, 0 stale
$ python3 tools/check-dead-file-coverage.py
info docs/ports/xterm.md: 131 range row(s) cover src/xterm.c (33125 lines; 73 blank line(s)
between blocks need no row)
145 ledger(s) bound (a C range table), 0 failure(s)
$ python3 tools/check-ledger-ticks.py
238 ledger(s) bound (a function tick to compare), 0 failure(s), 0 carried by another file,
352 row(s) with no function tick ..., 0 row(s) without a ledger
# the 27 ticks this round flips are all in the unit's text; the row count is unchanged because
# this slice adds no function the reference does not have.
$ python3 tools/check-citations.py
check-citations: 676 file(s) scanned, 4 drift, 0 range, 1991 external, 32 allowed, 4 stale
# the drift and stale rows are include/lisp.rs's and image.rs's own (other lanes' files, in flight
# while this round ran - the reading moved from 5 drift to 4 between two runs of this gate, and no
# run named this unit or this ledger); the external count grew by this slice's `/// C:` lines
# (1905 -> 1991 tree-wide, most of them the x11-constants and XGCValues rows).
$ python3 tools/check-declarations.py
17 declaration sites (0 forbidden, 17 allowed literals), 0 failures
$ python3 tools/check-mirror-dupes.py
16 duplicate declaration(s), 17 ratchet row(s), 1 failure(s)
# the failure is a STALE row of `femacs/src/sysdep.rs` (`pipe2 from
# lib/include/unistd.in.rs is gone`), and the count read 18/17/3 an hour earlier - both readings
# are other lanes' files (image.rs's `cairo_destroy` row came and went); no row names this unit,
# before or after, and this slice adds no declaration to a mirror.
$ python3 tools/check-citation-text.py | tail -1
checked 1156 / mismatched 97 / ranges 789 / unresolved 731
# the mismatched rows are xfns.rs/xmenu.rs/xselect.rs/xwidget.rs's own; none is in this unit's
# text or in this ledger.
```

Line count and hash: `femacs/src/xterm.rs` **10,952 -> 13,538 lines**, md5
`c6f1b725b1ffc98a2d5618cf61d4cef1` -> `d694654e84e3355e5bf83b8d24f60bc5`; this ledger's own
`## C line ranges` table grew the 27 rows above and the remainder row moved to `11916-33125`.

### Slice 11's readings (C 12066-12095), as landed and measured (2026-10-06)

**What it lands.**  `make_invisible_cursor` (C 12067-12095) - the one function of the exposure/focus
neighbourhood whose text needs no bitfield machinery: C's function-`static` `no_data` array stays in
the function (PORTING.md 1.3), `XColor col` is zeroed because C hands it to `XCreatePixmapCursor` by
address uninitialised (Deviation 12), C 12084's chained `col.red = col.green = col.blue = 0` is
written out in C's order, `col.flags` is XColor's `char` field (C's flag expression narrowed, `as
c_char`), `None` is `include/xlib.rs`'s `x11_constants::None` and C's `!x_had_errors_p (dpy)` is the
port's `== 0` (the int-returning predicate).

**The probe (the lane's command, both sides).**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xt11 cargo run -q -p xtask -- probe src/xterm.rs /tmp/xt11/new.o \
      --crate-type=lib --emit=metadata -C panic=abort --cfg 'feature="have_x_windows"'
# before this slice (HEAD's unit, HEAD's mirror - the same probe):
#   error: aborting due to 638 previous errors; 54 warnings emitted
#     436 E0425   13 E0308   166 E0609   17 E0614   1 E0381
# after (this unit, this mirror):
#   error: aborting due to 642 previous errors; 54 warnings emitted
#     440 E0425   13 E0308   166 E0609   17 E0614   1 E0381
# the whole delta, and it is arithmetic: +4 E0425 and nothing else - the four call sites this
# slice's text has into the three entry points the unit has not reached yet: x_catch_errors
# (C 26390), x_uncatch_errors (C 26568) and x_had_errors_p (C 26651, two calls).  Every other
# class reads exactly what it read before, and the warning count does not move.
```

**No mirror change this slice** (`check-mirror-dupes` 16/16/0): the four Xlib names the include
graph does not carry - `XCreatePixmapCursor` (Xlib.h 1554-1562) and the `XColor.flags` bits
`DoRed`/`DoGreen`/`DoBlue` (X.h 622-624, measured 1/2/4) - are declared at the use site (P-88 1,
the `src/xfns.rs` 1155-1161 Xlib rows' shape).  Moving them into `include/xlib.rs` (with the
`x11_constants` module's three constants) is **owed** and belongs to that mirror's own round, which
is also where the row for `XCreatePixmapCursor`'s half of the 68-entry Xlib inventory recorded in
slice 10's reading goes.

**Two neighbours are deliberately not in this slice**, and the reason is measured, not stylistic:
`x_toggle_visible_pointer` (C 12097-12139), `xfixes_toggle_visible_pointer` (C 12053-12062) and
`XTtoggle_invisible_pointer` (C 12141-12153) all store **`f->pointer_invisible`**, a `frame`
`bool_bf` for which no window-system-numbered accessor exists - `include/frame.rs`'s shift
numbering is carried for the frozen arm only (the `src/androidterm.rs` 214-217 note records the
same bound), so they wait on the `include/frame.rs` round; and `x_fixes_pointer_blanking_supported`
(C 12040-12045) reads `dpyinfo->xfixes_supported_p`/`xfixes_major`, which `include/xterm.rs` does
not carry yet.  Their checklist boxes stay unchecked on purpose.

**What `--check` reads.**  `extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs` goes
**185 rows -> 184**: `make_invisible_cursor` retires as a missing function; the line-count row stays
(13,607 Rust lines against C's 33,125, 58.9%).  The six text gates are unchanged
(`check-declarations` 18/0/18/0, `check-citations` 676/0/0/2129/36/0, `check-dead-file-coverage`
145/0, `check-port-lines` 0/0/0/0, `check-ledger-ticks` 238/0, `check-mirror-dupes` 16/16/0).

**Not claimed.**  The frozen configuration compiles none of this file, so the reading is a
**text** reading of an arm this machine cannot build: no cursor was created, no display was opened,
and the four `E0425` rows above are unresolved names, not behaviour.

### Slice 12's readings (C 26217-26347), as landed and measured (2026-10-06)

**What it lands.**  The first half of the error-handling block, the part that needs no request-serial
machinery: `struct x_error_message_stack` (C 26217-26239, with `handler`'s `x_special_error_handler`
type from xterm.h 1748-1749 declared at the use site), the two objects (`x_error_message` C 26249,
`x_error_message_count` C 26252 - the latter **non-`static`**, so exported, PORTING.md 0.2), the four
serial comparators (C 26260-26294, with C's `unsigned int` wrap written `wrapping_sub` and its
widening `as c_ulong`), `x_find_error_handler` (C 26296-26314), `x_unwind_errors_to` (C 26317-26323)
and `x_error_catcher` (C 26330-26347, whose `buf` is zeroed because C hands it to `XGetErrorText`
uninitialised, Deviation 12).

**The probe (the lane's command, both sides).**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xt12 cargo run -q -p xtask -- probe src/xterm.rs /tmp/xt12/new.o \
      --crate-type=lib --emit=metadata -C panic=abort --cfg 'feature="have_x_windows"'
# before (HEAD's unit + HEAD's mirror):
#   error: aborting due to 642 previous errors; 54 warnings emitted
#     440 E0425   13 E0308   166 E0609   17 E0614   1 E0381   1 E0080   1 E0277
# after (this unit + this mirror):
#   error: aborting due to 644 previous errors; 54 warnings emitted
#     441 E0425   13 E0308   166 E0609   17 E0614   1 E0381   1 E0080   1 E0277
# the whole delta, and it is arithmetic: +1 E0425 and nothing else - C 26322's x_uncatch_errors,
# whose own definition (C 26568) is the next slice's work.  Every other class reads what it read
# before and the warning count does not move.
```
**Arm reach is measured** (P-86 7): injecting `let _: c_int = true;` as the first statement of
`x_is_serial_more_than` moves E0308 to 14 anchored at the injected line (`_root.rs:13677`); removed,
the file re-measured at 13,770 lines.

**The mirror round** (`include/xlib.rs`, the mirror grows by the names a ported arm reads - its own
file note): `XErrorEvent` (Xlib.h 920-928) with the layout **measured** on this machine
(`sizeof=40 align=8`, `type` 0, `display` 8, `resourceid` 16, `serial` 24, `error_code` 32,
`request_code` 33, `minor_code` 34 - three `core::mem::` assertions at its declaration), and
`XGetErrorText` (Xlib.h 2610-2615) beside the mirror's other entry points.  **P-87's two legs**:
`check-mirror-dupes` **16/16/0** and `tools/check-all-units.sh` **`checked 83 unit(s), 0 failure(s)`
+ `checking 83 unit(s), 0 unit failure(s)`**; the frozen list holds no X unit, so the named reading
is that mirror's other consumer `src/xfns.rs` (probed with `have_x_windows` before and after the
mirror change, the unit unchanged): **6 error-lines / 9 warnings on both sides, sets diff empty**.

**What `--check` reads.**  `extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs` goes
**184 rows -> 176**: the seven functions and `x_error_message_count` retire as missing items; the
line-count row stays (13,770 against C's 33,125, 58.4%).  The six text gates are unchanged.

**Owed, named.**  The block's second half - `x_catch_errors_with_handler` (C 26372),
`x_catch_errors` (C 26390), `x_request_can_fail`/`x_clean_failable_requests` (C 26397/26420),
`x_ignore_errors_for_next_request`/`x_stop_ignoring_errors` (C 26463/26519),
`x_uncatch_errors_after_check`/`x_uncatch_errors` (C 26550/26567), `x_check_errors` (C 26613),
`x_had_errors_p` (C 26650) and `x_clear_errors` (C 26679) - needs `XNextRequest`, `NextRequest` and
`LastKnownRequestProcessed`, the three **macros of `<X11/Xlibint.h>`** that read fields of the
private `struct _XDisplay` (`request`, `last_request_read`, and the 64-bit `_upper32bit` pair), so
it is a measurement round of its own (P-88 2's clang probe) before those functions can land.  The
`x_error_message_count`'s read-only users in other units keep the mirror's declaration (the
`x_display_list` shape).

**Not claimed.**  The frozen configuration compiles none of this file, so this is a **text** reading
of an arm this machine cannot build: no error was trapped, no display was opened, and the one
`E0425` row above is an unresolved name, not behaviour.


### Slice 13's readings (C 26349-26689), as landed and measured (2026-10-06)

**What it lands.**  The error-handling block's second half: `x_catch_errors_with_handler` (C 26372)
and `x_catch_errors` (C 26389), `x_request_can_fail` (C 26397), `x_clean_failable_requests` (C 26420),
`x_ignore_errors_for_next_request` (C 26463) with its C 26474-26482 `HAVE_GTK3` arm,
`x_stop_ignoring_errors` (C 26519) with its C 26533-26538 one, `x_uncatch_errors_after_check`
(C 26550), `x_uncatch_errors` (C 26567), `x_check_errors` (C 26613, whose C 26640 `alloca` is the
port's stack pool, P-05), `x_had_errors_p` (C 26650) and `x_clear_errors` (C 26679) - and C 26691-26702's
`#if false` block (`x_fully_uncatch_errors`) carried as the comment it is, the C 5127-5156 shape.
Rust: 13,770 -> **14,188** lines.

**The probe (the lane's command, two arms, both sides).**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xt13 cargo run -q -p xtask -- probe src/xterm.rs /tmp/xt13/new.o \
      --crate-type=lib --emit=metadata -C panic=abort --cfg 'feature="have_x_windows"' \
      [--cfg 'feature="have_gtk3"']
# arm A (have_x_windows), before -> after:
#   644 error-lines / 54 warnings   441 E0425  13 E0308 166 E0609 17 E0614 1 E0381 1 E0080 1 E0277
#   568 error-lines / 54 warnings   366 E0425  13 E0308 166 E0609 17 E0614 1 E0381 1 E0080 1 E0277
# arm B (have_x_windows + have_gtk3), before -> after:
#   649 error-lines / 54 warnings   443 E0425  ...
#   579 error-lines / 54 warnings   374 E0425  ...
# the whole delta in both arms is E0425 and nothing else: -75 in arm A and -69 in arm B, i.e. the
# call sites of the eleven functions this slice defines (plus the five `XSync` rows and the two
# pre-existing `x_had_errors_p`-class rows the same definitions close).  No other class moves and
# the warning count is unchanged.
```
**Arm reach is measured** (P-86 7): injecting `let _: c_int = true;` as the first statement of
`x_catch_errors_with_handler` moves E0308 to 14 anchored at the injected line (`_root.rs:13819`);
removed, the file re-measured at 14,188 lines.

**The first readings named five fault classes of this slice's own text** (all fixed here; the
arithmetic above is the reading *after* the fix batch, so slice 12's reading sat inside them):
⑴ an `E0428` for `x_error_message_count` - **slice 12's own** defect, which that slice's class
counts did not include: `include/xterm.rs` 1645 declares the object and the unit defines it, so the
mirror's row is retired with a note (P-90 7; the units that read it, `src/eval.rs` 1835-1836,
declare it themselves).  ⑵ twenty `E0308`s at **earlier call sites**: `x_had_errors_p` returns
C's `bool` there, so `… == 0` / `… != 0` become `!x_had_errors_p (…)` and `x_had_errors_p (…)`
(twelve and seven sites, plus the two in `make_invisible_cursor`); C's `sprintf`-style truth test
and the port's differ only in spelling.  ⑶ `dpyinfo->failable_requests` is an **array** in the
record (`[x_failable_request; 128]`, `N_FAILABLE_REQUESTS`), so C's pointer decay is written
`… .as_mut_ptr ()` at its six uses.  ⑷ `XSync` had no row anywhere (`docs/ports/xterm.md`'s own
text called it) - it is declared in `include/xlib.rs` now (Xlib.h 3419-3420) and the five call
sites pass C's `False` as `0` because the parameter is Xlib's `Bool` (`c_int`), which also retires
the two pre-existing `E0425` rows.  ⑸ seven `variable does not need to be mutable` warnings (six
here, one carried from slice 11's `pix`), cleared - the warning count is back to its 54.

**The mirror round** (three files): `include/xlib.rs` gains the two **degraded** request-serial
macros - `NextRequest` and `LastKnownRequestProcessed` keep C's names with `XNextRequest` /
`XLastKnownRequestProcessed` (Xlib.h 1806/1809) for their bodies, because C's own bodies read the
private `_XPrivDisplay` record this tree does not carry (the `DefaultScreenOfDisplay` degradation
class, recorded as Decision 25) - plus `XSync`; `include/gtk.rs` gains
`gdk_x11_display_error_trap_push`/`_pop_ignored` (the two `HAVE_GTK3` arms' calls); and
`include/xterm.rs` loses the `x_error_message_count` row (above).  **P-90 5 move-backs in the same
commit**: `src/xfns.rs` 1954-1955's `XNextRequest` and `src/gtkutil.rs` 258-261's `XSync`
declarations retire, their mirror rows being the home now.
**P-87's two legs**: `check-mirror-dupes` **16/16/0** (the move-backs were its two failures:
`18 duplicate declaration(s), 16 ratchet row(s), 2 failure(s)` before them) and
`tools/check-all-units.sh` **`checked 83 unit(s), 0 failure(s)`** + **`checking 83 unit(s), 0 unit
failure(s)`**.  The frozen list holds no X unit, so two named readings carry the mirror's consumers:
`src/xfns.rs` reads **5 -> 4 previous errors** with the same classes and warnings (the moved row's
own site), and `src/gtkutil.rs` reads **0 error-lines / 0 warnings on both sides**.

**What `--check` reads.**  `184 -> 176` was slice 12; **this slice goes 176 -> 165**: the eleven
functions retire as missing items, the two token rows the first reading brought
(`x_check_errors does not mention alloca, false` and three more `does not mention false` rows) are
closed by `docs/ports/macro-body-notes.tsv` rows, and the line-count row stays (14,188 against
C's 33,125, 57.2%).  The six text gates are unchanged.

**Not claimed.**  The frozen configuration compiles none of this file, so both arms are **text**
readings of an arm this machine cannot build: no error was trapped, no display was opened and no
request serial was read; the remaining `E0425` rows are unresolved names, not behaviour.


### Slice 14's readings (C 26704-26992), as landed and measured (2026-10-06)

**What it lands.**  The `Handling X errors` banner with the file-scope `error_msg` pointer
(C 26722), `x_try_restore_frame` (C 26728-26738) and **`x_connection_closed` (C 26743-26992)** - the
connection-loss teardown, 250 C lines and this file's second-longest function: the recursive-call
guard through its own function-`static` `current_display`, the drag-and-drop cancellation arm with
its `sigsetjmp`/`XSetIOErrorHandler` pair and the Motif drop-message construction, the two
`FOR_EACH_FRAME` delete passes with their backward labels, the GTK arm's unconditional
`shut_down_emacs`/`emacs_abort`, the error-handler dump, and the final non-local exit
(C 26991's `error`).  C 26704-26713's second `#if false` pair is carried as the comment it is.
Rust: 14,188 -> **14,541** lines.

**The probe (the lane's command, three arms, both sides).**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xt14 cargo run -q -p xtask -- probe src/xterm.rs /tmp/xt14/fin.o \\
      --crate-type=lib --emit=metadata -C panic=abort --cfg 'feature="have_x_windows"' \\
      [--cfg 'feature="have_gtk3"'] [--cfg 'feature="use_gtk"']
# arm A (have_x_windows), before -> after:
#   568 error-lines / 54 warnings   366 E0425  13 E0308 166 E0609 17 E0614 1 E0381 1 E0080 1 E0277
#   564 error-lines / 54 warnings   362 E0425  13 E0308 166 E0609 17 E0614 1 E0381 1 E0080 1 E0277
# arm B (A + have_gtk3): 579 / 54 (374 E0425) -> 575 / 54 (370 E0425)
# arm C (A + use_gtk):   565 / 59 (360 E0425, 15 E0308, 167 E0609) -> 562 / 60 (357 E0425, 15, 167)
# the delta: -4 E0425 in A and B and -3 in C, and no other class moves in any arm.  Those rows are
# the earlier call sites the new use-site declarations resolve (do_switch_frame, delete_frame twice,
# clear_waiting_for_input, XSetIOErrorHandler); this slice's own two functions are `static` and their
# callers (x_error_quitter, x_io_error_quitter) are still ahead, so they add no resolution of their
# own.  Arm C's one extra warning is C's own structure: the USE_GTK arm ends in `emacs_abort ()` and
# C's statements after `#endif` follow it, which Rust reports as an unreachable statement (the
# mirrors' own two `unreachable expression` warnings are that class); A and B keep their 54.
```
**Arm reach is measured** (P-86 7): injecting `let _: c_int = true;` as the first statement of
`x_try_restore_frame` moves E0308 to 14 anchored at the injected line (`_root.rs:14244`); removed,
the file re-measured at 14,541 lines.

**The first readings named five of this slice's own fault classes**, all fixed here: ⑴ `FRAME_KBOARD`,
`FRAME_MINIBUF_WINDOW` and `FRAME_TOOLTIP_P` are the mirror's **functions**, not the macros other
units' copies of those names are (three rows); ⑵ `EQ` and `FRAME_LIVE_P` return `bool`, so C's
`== 0` tests are the port's `!` (three sites); ⑶ the `XM_DRAG_*`/`XM_DROP_*` names are enum members
behind the unit's own modules and the `xm_drop_start_message` fields are narrower than the values C
assigns (`as u8`/`as u32`/`as u16`, six sites - the C 26805-26817 block is the same text the
`xm_send_drop_message` path already spells at C 4943-4950); ⑷ `x_dnd_finish_display` is a
`*mut c_void` row (the P-72 rule 3 wording that path uses), so the C 26787 comparison writes the
cast; ⑸ the `-> !` return types the earlier slices gave `xm_drag_window_io_error_handler` and
`x_dnd_io_error_handler` are **incompatible with `XSetIOErrorHandler`'s `Emacs_XIOErrorHandler`
parameter**, so both take C's own `int` and keep their diverging tails - the two `E0308`s the first
reading named were that mismatch surfacing at C 26783 and at the motif path's C 1803.  Two
`variable does not need to be mutable` warnings and one `partially assigned binding` (`dmsg`,
zeroed per Deviation 12) were cleared with them.

**What `--check` reads.**  **165 -> 163**: `x_try_restore_frame` and `x_connection_closed` retire as
missing functions and the one token row this slice brought
(`function x_connection_closed does not mention alloca, goto`) is closed by a
`docs/ports/macro-body-notes.tsv` row (P-05's stack pool and PORTING.md 2.2's labelled `loop`s);
the line-count row stays (14,541 against C's 33,125, 56.1%).  The six text gates are unchanged, and
**no mirror changed this slice**, so P-87's two legs do not apply to it.

**Not claimed.**  The frozen configuration compiles none of this file, so all three arms are
**text** readings of an arm this machine cannot build: no connection was lost, no error was trapped
and no display was opened; the remaining `E0425` rows are unresolved names, not behaviour.


## Owed by the integrator (nothing here is claimed as done)

**1. Two `femacs/xtask/Cargo.toml` rows.**  `have_xrender_0_2` (`RENDER_MAJOR > 0 ||
RENDER_MINOR >= 2`, C 6512/6596/6728) and `have_xrender_0_10` (`RENDER_MAJOR > 0 ||
(RENDER_MINOR >= 10)`, C 6594) are derived names (Decision 20's table); Xrender.h is not in this
tree, so the names have to be declared before an XRender arm can be selected.  The other names
slice 8's arms carry already exist: `use_cairo` (Cargo.toml 790), `use_cairo_xcb` (375),
`use_xcb`, `have_xrender` (370), `have_xdbe` (364), `have_xinput2` and its `_1`/`_2` (367-368),
`have_gtk3` (321), `use_gtk` (792), `use_x_toolkit` (806) and the four `cairo_has_*` (47-50).

**2. Mirror rows the carried arms read, none of which this writing set could add** (each is a
`P-26` "the owning mirror takes it" note, the same shape Decision 19's list has):

* `struct xi_scroll_valuator_t`'s two bitfields `horizontal` and `invalid_p` (xterm.h 238-256).
  `include/xterm.rs` records them as comments (its 274-296), so `xi_populate_scroll_valuator`,
  `x_get_scroll_valuator_delta` and `xi_reset_scroll_valuators_for_device_id` write C's spelling of
  a member that is not an identifier in the Rust text; the `bits` accessors `PORTING.md` 1.3/P-80
  prescribes are what those sites need.
* `struct x_display_info`'s `ext_codes` (xterm.h 838) and `supports_xkb` (911) - both already on
  Decision 19's "still uncarried" list, now *read* by `x_gc_get_ext_data`/`x_extension_initialize`
  and `x_restore_events_after_dnd`.
* `<math.h>`'s `lrint` (used by `xi_link_touch_point`'s `HAVE_XINPUT2_2` arm): neither
  `lib/include/math.rs` nor any other mirror carries it.
* The `use_gtk` arm's names: `FRAME_TOOLBAR_TOP_HEIGHT`/`FRAME_TOOLBAR_LEFT_WIDTH` (xterm.h
  1469/1477) and `FRAME_GTK_OUTER_WIDGET` (1449) live only in `include/pgtkterm.rs` in this tree,
  `xg_get_scale` is `src/gtkutil.rs`'s exported function with no prototype in this graph,
  `gtk_widget_intersect` and `G_OBJECT_GET_CLASS` have no row anywhere (`G_OBJECT_GET_CLASS` is a
  *per-unit* macro in `xfns.rs` 1017 and `xwidget.rs` 473, not a mirror row).
* The cairo names the `use_cairo` arms use (`cairo_t`, `cairo_surface_t`, `cairo_pattern_t`, the
  `cairo_*` functions, `cairo_user_data_key_t`, `cairo_matrix_t`, `CAIRO_*`): they live in
  `include/gtk.rs` (242-300), which this unit reaches only under `use_gtk` - a `use_cairo` build
  without GTK would need that mirror's edge (or the cairo rows moved to one).
* The X11 SDK types and constants the XRender arms use (`XGCValues`, `XRenderColor`,
  `XRenderPictureAttributes`, `Picture`, `Unsorted`, `GCFillStyle`, `GCStipple`, `FillSolid`,
  `FillOpaqueStippled`, `PictOpSrc`, `PictOpOver`, `PictStandardA1`, `CPRepeat`, `CPClipMask`,
  `RepeatNormal`, `XQueryTree`), the XInput2 ones (`XIDeviceEvent`, `XIDeviceInfo`,
  `XIScrollClassInfo`, `XIValuatorClassInfo`, `XITouchClassInfo`, `XIButtonState`, `XIEventMask`,
  `XIMaskLen`, `XIMaskIsSet`, `XIScrollClass`, `XIValuatorClass`, `XITouchClass`, `XIModeAbsolute`,
  `XIDirectTouch`, `XIScrollTypeHorizontal`, `Button1Mask`) and the GC extension records
  (`XEDataObject`, `XExtData`, `XExtCodes`, `XEHeadOfExtensionList`, `XFindOnExtensionList`,
  `XAddToExtensionList`, `XAddExtension`): Xlib/XInput2/XRender names with no mirror in this
  graph, kept bare as Decision 15's rule says.
* The display-iterators' arm names this slice reaches through `XWindowAttributes`' `your_event_mask`
  (which *does* resolve) and through `XImage`-side helpers in the cairo arms (no XImage row).

**3. Slice 6's `*mut c_void` display model, now visible.**  `x_dpyinfo`'s definition turned slice
6's `x_dnd_free_toplevels` call site into the flip arm's one new `E0308` (`*mut c_void` vs C's
`Display *`), the class Decision 19 clause 4 already records.  A unit-lane edit that moves those
locals onto `*mut Display` (or casts at the call) makes it go away; nothing in this slice chose it.

**4. The mirror's `XEvent` is still opaque**, and this file's text writes its members
(`msg.xclient`, `msg.xbutton`, `configure->xconfigure`, `x_dnd_pending_send_position.type`).  The
count is now 82 pre-existing sites plus this slice's 43 (`xclient`) and 3 (`xconfigure`/`r#type`);
`docs/ports/include-xlib.md` owes the union's 33 member records, and uniting the spellings is that
ledger's call, not a per-function choice - C's spelling is what the token gate demands and what slices
2, 5 and 6 already write.

**5. The next slice.**  **C 11916-…** (C 11925's `x_frame_highlight` on: the frame highlight
and focus hooks, the XFixes pointer blanking, `XTread_socket` and the event loop, the toolkit and
scroll-bar code, `x_initialize` and `syms_of_xterm`); this ledger's `## C line ranges` row
`11916-33125` is the remainder that row's lane replaces, and the gates it needs are the ones
`## Gate mapping` lists.  (This item carried slice 8's next-slice text - "C 6991-…" and the
`6990-33125` remainder - through slice 9's round, which had already landed that range; slice 10
corrects it, the same `C 11916` the range table names.)

**6. What moved outside this writing set: nothing.**  `emacs/**` was read-only; no mirror, no
`xtask`, no `docs/baseline` and no other lane's file changed.  The one append is
`docs/ports/macro-body-notes.tsv` 1269 (`x_dnd_note_self_drop`'s `alloca`/`kind`), the row the
`--check` gate asks for.

**7. Slice 9's requests (C 6990-9785, this round).**

* Three `femacs/xtask/Cargo.toml` rows: `have_clock_gettime` (C 6994's third term),
  `frame_debug` (C 7101/7271's `#ifdef FRAME_DEBUG`) and `debug_x_colors` (C 9506/9562's
  `#ifdef DEBUG_X_COLORS`).  The other names this slice's arms carry already exist (Decision 21's
  sub-gate paragraph names them).
* **`include/dispextern.rs`'s `VCENTER_BASELINE_OFFSET` cannot resolve in this unit** (10
  `E0423`, measured): its body (dispextern.rs 2658-2663) names `FONT_DESCENT`/`FONT_HEIGHT` and
  `FRAME_FONT`/`FRAME_LINE_HEIGHT` as *calls*, while this graph has `FONT_HEIGHT` as
  `include/font.rs` 258's macro, `FRAME_FONT` as `include/xterm.rs` 1119's macro, and neither
  `FONT_DESCENT` nor `FRAME_LINE_HEIGHT` at all.  The fix belongs to the mirror (spell the two
  macros as `FONT_HEIGHT!`/`FRAME_FONT!` and name the two it lacks); this slice is merely the
  first call site to reach the macro.
* Members the mirrors keep as comments or gate behind another feature, each measured this round:
  `frame`'s `borders_respect_alpha_background` (11 `E0609`; C 1851-1861's `#ifdef
  HAVE_WINDOW_SYSTEM` region - the mirror carries `foreground_pixel`/`background_pixel` but not
  this one), `Mouse_HLInfo`'s `mouse_face_defer` (1), `color_name_cache_entry`'s `valid` bitfield
  (3; the mirror's comment at `include/xterm.rs` 261 says so, so the port keeps C's spelling as
  the `xi_scroll_valuator_t` row already does), and `glyph_string`'s `gc` (the record's own gate,
  see the next clause).
* **The `dispextern_face_records`/`have_window_system` group**: this unit now carries its own
  `struct face` (X11 arm, `docs/baseline/face-layout.tsv`), `draw_fringe_bitmap_params` (the
  mirror's offsets) and `glyph_string` (C's order, **no** asserts: the mirror's own note records
  that record's layout as unmeasured here).  Status 3.220's ruling makes the copies legitimate;
  whoever retires the units' copies (the `xfaces.rs`/`window.rs`/`alloc.rs`/`term.rs`/`dispnew.rs`
  list) has to take `femacs/src/xterm.rs`'s three with them, and only then can `glyph_string_bits!`
  and `get_/set_cursor_p`/`get_/set_overlay_p` come from the mirror instead of the unit.
* The X11 macros this slice's live text names and `include/xlib.rs`'s `x11_constants` does not
  carry: `FillOpaqueStippled`, `FillSolid`, `GCGraphicsExposures`, `GCLineWidth`, `GCStipple`,
  `GCClipMask`, `GCClipXOrigin`, `GCClipYOrigin`, `Convex`, `CoordModeOrigin` (`<X11/X.h>`/
  `<X11/Xlib.h>` values, each clang-measurable the way that module's rows were); the record
  `XGCValues` (Xlib.h 177-203) is now declared **in the unit** the way `src/xftfont.rs` 132-171
  declares it, plus `XPoint` (Xlib.h 322-325) and `XdbeSwapInfo` (Xdbe.h), which nothing carries.
* The two `unsafe extern "C"` blocks this unit now holds (`get_glyph_string_clip_rects`,
  `prepare_face_for_display`, `x_free_colors`, `parse_color_spec`, `gamma_correct`,
  `x_free_dpy_colors`, each with C's own line) move into the owning mirror when it grows them
  (P-26).
* `x_color_cache_bucket_size` is `DEFVAR_INT`'d at C 32958, so it lands with that slice; until
  then the four call sites the port writes are `E0425` names, the class slices 4-8 record.

**8. What moved outside this writing set (slice 9).**  `emacs/**` was read-only; no mirror, no
`xtask` row, no `docs/baseline` file and no other lane's file changed.  The two appends are
`docs/ports/macro-body-notes.tsv` 1678-1679 (`x_alloc_nearest_color_1`'s `goto`; `x_draw_fringe_bitmap`'s
`cursor_p`/`goto`/`overlay_p`), the rows the `--check` gate asks for; both were written by copying
the table to `/tmp` first and appending, so no other lane's row was rewritten.  The build
artifacts this round left (`build/femacs/units/xterm`, `build/probes/xterm`, `build/femacs/xterm.o`)
were deleted after each compile.

**9. One `femacs/xtask/Cargo.toml` row: `have_xkb`.**  This slice's `XTring_bell` carries C 11677's
`#ifdef HAVE_XKB` (and slice 8's `x_restore_events_after_dnd` already carried C 4791's), and the
feature table has no `have_xkb` row (measured: `grep -n xkb femacs/xtask/Cargo.toml` finds none),
so the arm cannot be selected today.  Every other name this slice's arms carry exists:
`use_cairo`, `use_gtk`, `use_x_toolkit`, `use_xcb`, `use_cairo_xcb`, `have_xdbe`, `have_xrender`,
`have_xrender_0_2`, `have_xwidgets`.

**10. Mirror rows this slice's carried arms read** (each a `P-26`/`P-55` "the owning mirror takes
it" note, the same shape Decision 22's clause 6 and slice 9's Owed items have):

* `include/xlib.rs`'s `x11_constants`: the 20 X11 constants this slice's live text names and the
  module does not carry - `FillSolid`, `FillOpaqueStippled`, `GCFunction`, `GCClipMask`,
  `GCClipXOrigin`, `GCClipYOrigin`, `GCLineStyle`, `GCLineWidth`, `GXcopy`, `GXinvert`,
  `LineSolid`, `LineOnOffDash`, `CapButt`, `JoinRound`, `Convex`, `CoordModeOrigin`, `CWCursor`,
  `CopyFromParent`, `InputOnly`, `Unsorted` - each an `<X11/X.h>`/`<X11/Xlib.h>` value
  clang-measurable the way that module's rows already are (`docs/ports/include-xlib.md`).
* `include/xlib.rs`: the `XPoint` record (Xlib.h 322-325) and `XSetWindowAttributes` (Xlib.h
  243-252, the cursor field this slice's `x_show_hourglass` writes), plus the `XEvent` union the
  ledger's Decision 20 and `docs/ports/include-xlib.md` already owe - this slice's five `xclient`
  member writes are five more sites of that class (108 -> 113 in this arm's reading).
* The dispextern.h prototypes a flip needs and this graph declares nowhere:
  `get_glyph_string_clip_rect` (C 3663) and `gui_intersect_rectangles` (C 3676) *do* exist in
  `include/dispextern.rs`'s `dispextern_late_decls`, but under `feature = "have_window_system"`,
  so a flip that sets `have_x_windows` alone leaves this slice's nine call sites `E0425` (the
  ledger's evidence above counts them there); `window_box` (C 3574-3575), `window_box_left`
  (3579), `window_box_left_offset` (3580), `window_box_right` (3581), `font_for_underline_metrics`
  (3638), `gui_clear_cursor` (3657), `cursor_in_mouse_face_p` (3692), `image_ascent` (3774) and
  `popup_activated` (3879) have no row anywhere - the `w32term.rs` 1750/3134-3141 use-site
  declarations are the shape they should take, and `detect_input_pending` (lisp.h 5095) is the
  same case.
* `include/frame.rs` 1908-1914's `flush_frame!` body names `redisplay_interface` and no unit of
  this graph declares it, so *every* call site of that macro reads one `E0425` (`widget.rs` 1400
  records the same finding; `haikufns.rs` 3664 is another site, and this slice's `XTflash` call
  path is the third).
* `include/dispextern.rs`'s own `face_bits!` (and the window mirror's `window_bits!` /
  `window_box` rows): this slice extends the *unit's* copy with the six accessors C 11064-11226
  reads, and calls `window_bits! (pseudo_window_p, …)`, which no unit can resolve today
  (`dispnew.rs` 396 and `fringe.rs` 865 keep their own copies of that macro for the same reason).

**11. Two defects this slice found in slices 1-9's text and did not edit** (they are outside its C
range; the round attributes them instead, and both readings carry them unchanged):

* **17 `E0614`** (`type \`face\`/\`glyph\` cannot be dereferenced`): the unit's own `face_bits!`,
  `glyph_u_cmp_bits!` and `glyphless_bits!` dereference their argument, and every one of their
  call sites passes a *value* rather than a pointer - `face_bits!` at 9687, 9736, 9806, 9856, 9889,
  9899, 9912, 9952, 10924; `glyph_u_cmp_bits!` at 9836; `glyphless_bits!` at 9974, 9976, 9984,
  9986, 9994, 9996, 10027.  Each is written `(*((*s).face as *mut face))` (or `(*glyph)`) where
  the macro's body is `(*$f)…`; the `w32term.rs` 2235 and `haikuterm.rs` 350 call sites show the
  intended spelling (pass the pointer), which is also what the macro's own `$f:expr` grouping
  wants.  Slice 9's ledger records these as "the `(*((*s).face as *mut face))` groupings", one of
  the three fix batches it landed *after* its last probe run - which is why they are 17 in both
  readings of this round.
* **2 of the prefix's 13 `E0308`**: `x_draw_glyph_string_background`'s two
  `if glyph_string_bits! (for_overlaps, (*s))` (9728 and 9750 in the current text) use that
  macro's `c_int` arm directly as a condition; `!= 0` is the fix, and it is the one this slice's
  own eight `for_overlaps` sites use.

**12. What this slice does not claim.**  No X11 build, no cairo/GTK/XRender/XCB/xwidget/XKB build
and no runtime were exercised: the frozen configuration compiles none of this file, the probe
compiles the plain-X11 arm (`have_x_windows`, no `use_cairo`, no `have_xrender`, no `use_gtk`, no
`have_xdbe`, no `have_xwidgets`, no `use_xcb`, no `have_xkb`), and everything under those features
is carried text that **no configuration in this tree has compiled**.  `build-tu` proves parsing;
the probe's numbers are unresolved names, not behaviour; and nothing here is device, GUI or
end-to-end evidence for any of the functions this slice lands.


### Slice 15's readings (C 26994-27126), as landed and measured (2026-10-06)

**What it lands.**  C 26994's prototype for `x_error_quitter` (carried as the comment it is:
Rust has no prototypes, and the definition keeps C's position), `x_error_handler`
(C 26999-27073, the first-level handler - the `USE_GTK && HAVE_GTK3` early-out, the
`x_request_can_fail` arm with its `memmove` compaction of `dpyinfo->failable_requests`, the
`x_handle_selection_error` arm and the XI2 arm), `x_error_quitter` (C 27081-27107, `NO_INLINE`,
`buf`/`buf1` and the message that reaches `x_connection_closed`) and `x_io_error_quitter`
(C 27114-27126, `NO_INLINE`, the `DisplayString` degradation, C's function-`static` `fmt` and the
`SAFE_ALLOCA` buffer).  Rust 14,541 -> 14,722 lines.

**What the probes read.**  Three arms, both sides, baseline = HEAD's unit with the *same* mirror
(this slice changes the mirror, so the baseline takes the new one to keep the delta the unit's
own): arm A (`have_x_windows`) **564 error-lines / 54 warnings -> 563 / 55**; arm B (A +
`have_gtk3` + `use_gtk`) **564 / 59 -> 563 / 60**; arm C (A + `have_xinput2`) **592 / 56 -> 591 /
57**.  Every arm's delta is **-1 `E0425`** (the `XDisplayString` use-site declaration retires the
row the arm had been carrying; this slice's own three functions are `static` and their callers are
already in the file) plus **+1 warning** - `value assigned to sa_avail is never read`, the class
`docs/ports/atimer.md` 106 records: C's `AVAIL_ALLOCA` also updates `sa_avail` and never reads it
again in a function that allocates once.  No other class moved in any arm.

**Arm reach (P-86 7).**  Injecting `let _: c_int = true;` as the first statement of
`x_error_handler` moves `E0308` from 13 to 14, anchored at the injected line (`_root.rs:14594`);
removed, arm A re-measures at 563 error-lines / 55 warnings and the file at 14,722 lines.

**The mirror round (P-87).**  `include/xterm.rs` moves the two rows Decision 27 records from
`x_output` to `x_display_info`.  Leg one: `tools/check-mirror-dupes.py` **16 duplicate
declaration(s), 16 ratchet row(s), 0 failure(s)**.  Leg two: `tools/check-all-units.sh` **checked 83
unit(s), 0 failure(s)** and **checking 83 unit(s), 0 unit failure(s)**.  The named reading is
another consumer of that mirror, `src/xfns.rs`: probed with `have_x_windows` it reads **6
error-lines / 9 warnings** before and after, and with `have_x_windows` + `have_xinput2` **7 / 9**
before and after, the sets diffing empty in both arms.

**What `--check` reads.**  **163 -> 159**: the three `missing function` rows retire, the two token
rows the first reading brought (`x_error_quitter does not mention 800, int_strlen_bound, xid` and
`x_io_error_quitter does not mention displaystring`) are closed by
`docs/ports/macro-body-notes.tsv` rows, and slice 14's invented-export row is removed (Decision
27).  The line-count row stays (14,722 against C's 33,125, 55.6%).

**Not claimed.**  The frozen configuration compiles none of this file, so all three arms are
**text** readings of an arm this machine cannot build: no error was trapped, no display was
opened, no connection was lost and no request serial was read; the remaining `E0425` rows are
unresolved names, not behaviour.  `x_error_handler`'s address is what C 30791/32358 hand to
`XSetErrorHandler`, and nothing here has run.


### Slice 16's readings (C 27128-27194), as landed and measured (2026-10-06)

**What it lands.**  The `Changing the font of the frame` banner and `x_new_font` (C 27136-27194) -
the fontset id, the frame's font/baseline/column-width/line-height writes, the scroll-bar column
computation with its `Bug#17163` note, the tip-frame guard around `adjust_frame_size`, and the
`HAVE_X_I18N` arm that re-sets the XIC's fontset through `xic_set_xfontset`.  Rust 14,722 -> 14,857
lines.  The five two-arm frame.h macros and the use-site externs (`fontset.h` 39/44, `dispextern.h`
3631, `window.h` 930, `xfns.c` 3814) are Decision 28's.

**What the probes read.**  Three arms, both sides, baseline = HEAD's unit with the same mirror:
arm A (`have_x_windows`) **563 error-lines / 55 warnings -> 563 / 55**; arm B (A + `have_gtk3` +
`use_gtk`) **563 / 60 -> 563 / 60**; arm C (A + `have_xinput2`) **591 / 57 -> 591 / 57**.  Every
arm's delta is **zero** - this slice's text resolves every name it uses, which is the reading
`s16dA` produced after the two first-reading faults were fixed (the `have_window_system` block and
`let unit`, which the arm named `unused_mut`).

**Arm D, the `have_x_i18n` text (new this slice).**  `have_x_windows` + `have_x_i18n`: baseline
**584 error-lines / 55 warnings -> 564 / 55**, the whole delta `-20 E0425` (the XIM types and masks
Decision 28 declares) with every other class and the warning count unmoved.  The named reading of the *other* consumer of
that mirror, `src/xfns.rs`, reads **6 error-lines / 9 warnings** on both sides with
`have_x_windows` (sets identical) and **20 / 12 -> 9 / 12** with `have_x_windows` + `have_x_i18n` -
the leak fix Decision 28 records is what removes those eleven `no field xic`-class rows there,
warnings unchanged.  The arm exists because
this is the first slice whose text carries a `HAVE_X_I18N` branch of its own; the frozen
configuration compiles none of it.

**What `--check` reads.**  **159 -> 158**: `missing function x_new_font` retires and no token row
arrives.  The line-count row stays (14,857 against C's 33,125, 55.1%).

**Not claimed.**  The frozen configuration compiles none of this file, so all four arms are **text**
readings of an arm this machine cannot build: no frame was given a font, no scroll bar was sized
and no frame was resized; the remaining `E0425` rows are unresolved names, not behaviour.


### Slice 17's readings (C 27195-27284), as landed and measured (2026-10-06)

**What it lands.**  The `X Input Methods` banner (C 27196-27198), C 27204-27208's note on the two
R6 macros, `x_maybe_clear_preedit` (C 27218-27243) and `xim_destroy_callback` (C 27249-27280),
all under C's three nested guards.  Rust 14,857 -> 15,007 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**563 error-lines / 55 warnings -> 563 / 55**; arm B **563 / 60 -> 563 / 60**; arm C
**591 / 57 -> 591 / 57**; arm D (`have_x_i18n`) **564 / 55 -> 564 / 55** - the new text is behind
`have_x11r6_xim`, so none of the four arms sees it.  Arm F (`have_x_windows` + `have_x_i18n` +
`have_x11r6` + `have_x11r6_xim`), which is the arm this text exists in: **563 / 55 -> 547 / 55**,
the whole delta **-16 `E0425`** (the sixteen `XFree` calls Decision 29 records) with every other
class and the warning count unmoved.

**Arm reach (P-86 7).**  Injecting `let _: c_int = true;` as the first statement of
`x_maybe_clear_preedit` moves `E0308` from 13 to 14, anchored at the injected line
(`_root.rs:14942`); removed, arm F re-measures at 547 error-lines / 55 warnings and the file at
15,007 lines.

**What `--check` reads.**  **158 -> 158**: neither function is a demanded item, because the frozen
table leaves `HAVE_X11R6_XIM` `undef` and the tool reads C 27210-27282 as dead text (P-66).  The
line-count row moves to 15,007 against C's 33,125 (54.7%).

**Not claimed.**  The frozen configuration compiles none of this text: no preedit was cancelled,
no input-method connection died and no frame was walked.  The remaining `E0425` rows are
unresolved names, not behaviour.


### Slice 18's readings (C 27286-27342), as landed and measured (2026-10-06)

**What it lands.**  `xim_open_dpy` (C 27289-27341), with C's `#ifdef HAVE_XIM` body (C 27296-27338)
and its `#else` text (C 27340), inside C 27200's `HAVE_X_I18N`; plus the XIM surface it calls
(Decision 30) and `XFree`'s declaration widened to every X arm.  Rust 15,007 -> 15,125 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**563 / 55 -> 547 / 55**; arm B **563 / 60 -> 547 / 60**; arm C **591 / 57 -> 575 / 57**; arm D
**564 / 55 -> 548 / 55** - each arm's whole delta is **-16 `E0425`** (the `XFree` calls), every
other class and the warning count unmoved.  Arm G (`have_x_windows` + `have_x_i18n` + `have_xim`),
the arm this slice's own text exists in: **564 / 55 -> 545 / 55**, the delta **-19 `E0425`** - the
same sixteen `XFree` rows plus three `Status` rows the declaration resolves - with no other class
moving.

**Arm reach (P-86 7).**  Injecting `let _: c_int = true;` as the first statement of the
`have_xim` arm moves `E0308` from 13 to 14, anchored at the injected line (`_root.rs:15083`),
and the arm also reads `unused_mut` if `xim` is declared `mut` - which is how the first reading
named that; removed, arm G re-reads 545 / 55 and the file 15,125 lines.

**What `--check` reads.**  **158 -> 158**: `xim_open_dpy` is dead text for the tool too (C 27200's
`HAVE_X_I18N` is `undef` in the frozen table).  The line-count row moves to 15,125 against C's
33,125 (54.3%).

**Not claimed.**  The frozen configuration compiles none of this text: no input-method connection
was opened or closed, no style list was queried and no coding system was chosen.  The remaining
`E0425` rows are unresolved names, not behaviour.
