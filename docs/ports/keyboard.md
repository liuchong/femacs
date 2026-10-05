# emacs/src/keyboard.c (14682 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 38
- [x] recursive-edit  (Frecursive_edit, 0/0, doc 302f2141adf3, line 781)
- [x] command-error-default-function  (Fcommand_error_default_function, 3/3, doc d7be823f8752, line 1049)
- [x] top-level  (Ftop_level, 0/0, doc fa16f528f026, line 1199)
- [x] exit-recursive-edit  (Fexit_recursive_edit, 0/0, doc 1c23b1556486, line 1223)
- [x] abort-recursive-edit  (Fabort_recursive_edit, 0/0, doc 960143aa333a, line 1234)
- [x] internal--track-mouse  (Finternal_track_mouse, 1/1, doc 088a57fa7594, line 1267)
- [x] current-idle-time  (Fcurrent_idle_time, 0/0, doc 865cb82eef62, line 4949)
- [x] internal-event-symbol-parse-modifiers  (Fevent_symbol_parse_modifiers, 1/1, doc 93b81128a741, line 7609)
- [x] event-convert-list  (Fevent_convert_list, 1/1, doc 446cb1a37d24, line 7832)
- [x] internal-handle-focus-in  (Finternal_handle_focus_in, 1/1, doc 711ea1f4463e, line 7891)
- [x] read-char  (Fread_char, 0/3, doc 70285ed40d05, line 10700)
- [x] read-event  (Fread_event, 0/3, doc 5ddbfceb4fe8, line 10749)
- [x] read-char-exclusive  (Fread_char_exclusive, 0/3, doc e5a7c5674cd8, line 10784)
- [x] read-key-sequence  (Fread_key_sequence, 1/6, doc 58569e51213e, line 11961)
- [x] read-key-sequence-vector  (Fread_key_sequence_vector, 1/6, doc dd9cad359995, line 12022)
- [x] input-pending-p  (Finput_pending_p, 0/1, doc 12dd059be36d, line 12076)
- [x] insert-special-event  (Finsert_special_event, 1/1, doc 1e66da412198, line 12096)
- [x] lossage-size  (Flossage_size, 0/1, doc bd43255a778a, line 12173)
- [x] recent-keys  (Frecent_keys, 0/1, doc 8606d7e36a58, line 12213)
- [x] this-command-keys  (Fthis_command_keys, 0/0, doc 1929899c43b3, line 12244)
- [x] set--this-command-keys  (Fset__this_command_keys, 1/1, doc c27b95ad9a4d, line 12257)
- [x] this-command-keys-vector  (Fthis_command_keys_vector, 0/0, doc bb03ef057b92, line 12290)
- [x] this-single-command-keys  (Fthis_single_command_keys, 0/0, doc 109cc4488be2, line 12302)
- [x] this-single-command-raw-keys  (Fthis_single_command_raw_keys, 0/0, doc c31713d33004, line 12316)
- [x] clear-this-command-keys  (Fclear_this_command_keys, 0/1, doc 54e95ebb00df, line 12329)
- [x] recursion-depth  (Frecursion_depth, 0/0, doc 97e5dba2bab2, line 12350)
- [x] open-dribble-file  (Fopen_dribble_file, 1/1, doc 66bb95dc7b71, line 12359)
- [x] discard-input  (Fdiscard_input, 0/0, doc 45ee08fcbeb0, line 12400)
- [x] suspend-emacs  (Fsuspend_emacs, 0/1, doc ec4f11b4c989, line 12422)
- [x] set-input-interrupt-mode  (Fset_input_interrupt_mode, 1/1, doc ec2c5f2ea0bc, line 12771)
- [x] set-output-flow-control  (Fset_output_flow_control, 1/2, doc 979529ef403e, line 12818)
- [x] set-input-meta-mode  (Fset_input_meta_mode, 1/2, doc d0dc2c06edd4, line 12852)
- [x] set-quit-char  (Fset_quit_char, 1/1, doc 05a705a78eda, line 12908)
- [x] set-input-mode  (Fset_input_mode, 3/4, doc 88acce0d9fb9, line 12944)
- [x] current-input-mode  (Fcurrent_input_mode, 0/0, doc 68f4fa72d27f, line 12967)
- [x] posn-at-x-y  (Fposn_at_x_y, 2/4, doc ed0784f09fe9, line 13009)
- [x] posn-at-point  (Fposn_at_point, 0/2, doc b412ffe01d32, line 13054)
- [x] posn-point  (Fposn_point, 1/1, doc cbe72d645e3e, line 13099)

## Functions defined — 184
- [x] next_kbd_event  (line 416, static)
- [x] xevent_start  (line 425, static)
- [x] kset_echo_string  (line 432, static)
- [x] kset_echo_prompt  (line 437, static)
- [x] kset_kbd_queue  (line 442, static)
- [x] kset_keyboard_translate_table  (line 447, static)
- [x] kset_last_prefix_arg  (line 452, static)
- [x] kset_last_repeatable_command  (line 457, static)
- [x] kset_local_function_key_map  (line 462, static)
- [x] kset_overriding_terminal_local_map  (line 467, static)
- [x] kset_real_last_command  (line 472, static)
- [x] kset_system_key_syms  (line 477, static)
- [x] echo_keystrokes_p  (line 484, static)
- [x] echo_add_key  (line 496, static)
- [x] echo_dash  (line 552, static)
- [x] echo_update  (line 601, static)
- [x] echo_now  (line 631, static)
- [x] cancel_echoing  (line 659)
- [x] echo_length  (line 672, static)
- [x] echo_truncate  (line 684, static)
- [x] add_command_key  (line 697, static)
- [x] recursive_edit_1  (line 708)
- [x] record_auto_save  (line 766)
- [x] force_auto_save_soon  (line 775)
- [x] recursive_edit_unwind  (line 837)
- [x] not_single_kboard_state  (line 852)
- [x] push_kboard  (line 871)
- [x] pop_kboard  (line 883)
- [x] temporarily_switch_to_single_kboard  (line 918)
- [x] restore_kboard_configuration  (line 946, static)
- [x] cmd_error  (line 964, static)
- [x] cmd_error_internal  (line 1030)
- [x] command_loop  (line 1113)
- [x] command_loop_2  (line 1158)
- [x] top_level_2  (line 1170, static)
- [x] top_level_1  (line 1187, static)
- [x] user_error  (line 1218, static)
- [x] tracking_off  (line 1249, static)
- [x] some_mouse_moved  (line 1292, static)
- [x] command_loop_1  (line 1318, static)
- [x] read_menu_command  (line 1691)
- [x] adjust_point_for_property  (line 1719, static)
- [x] safe_run_hooks_1  (line 1898, static)
- [x] safe_run_hooks_error  (line 1908, static)
- [x] safe_run_hook_funcall  (line 1944, static)
- [x] safe_run_hooks  (line 1970)
- [x] safe_run_hooks_maybe_narrowed  (line 1981, static)
- [x] safe_run_hooks_2  (line 2003)
- [x] poll_for_input_1  (line 2031)
- [x] poll_for_input  (line 2043, static)
- [x] start_polling  (line 2055)
- [x] input_polling_used  (line 2094)
- [x] stop_polling  (line 2110)
- [x] set_poll_suppress_count  (line 2125)
- [x] bind_polling_period  (line 2145)
- [x] make_ctrl_char  (line 2178)
- [x] help_echo_substitute_command_keys  (line 2219, static)
- [x] show_help_echo  (line 2259)
- [x] read_char_help_form_unwind  (line 2306, static)
- [x] read_event_from_main_queue  (line 2323, static)
- [x] read_decoded_event_from_main_queue  (line 2415, static)
- [x] read_char  (line 2555, static)
- [x] record_menu_key  (line 3439, static)
- [x] help_char_p  (line 3462, static)
- [x] record_char  (line 3476, static)
- [x] save_getcjmp  (line 3626, static)
- [x] restore_getcjmp  (line 3632, static)
- [x] readable_events  (line 3644, static)
- [x] event_to_kboard  (line 3728, static)
- [x] kbd_buffer_nr_stored  (line 3750, static)
- [x] kbd_buffer_store_event  (line 3758)
- [x] kbd_buffer_store_buffered_event  (line 3774)
- [x] position_to_Time  (line 3891, static)
- [x] Time_to_position  (line 3901, static)
- [x] gen_help_event  (line 3923)
- [x] kbd_buffer_store_help_event  (line 3942)
- [x] discard_mouse_events  (line 3960)
- [x] kbd_buffer_events_waiting  (line 3984)
- [x] clear_event  (line 3999, static)
- [x] kbd_buffer_get_event_1  (line 4005, static)
- [x] kbd_buffer_get_event_2  (line 4020, static)
- [x] kbd_buffer_get_event  (line 4032, static)
- [x] process_special_events  (line 4595, static)
- [x] swallow_events  (line 4678)
- [x] timer_start_idle  (line 4695, static)
- [x] timer_stop_idle  (line 4711, static)
- [x] timer_resume_idle  (line 4719, static)
- [x] decode_timer  (line 4735, static)
- [x] timer_check_2  (line 4763, static)
- [x] timer_check  (line 4914)
- [x] make_lispy_tty_position  (line 5716, static)
- [x] make_lispy_position  (line 5785, static)
- [x] toolkit_menubar_in_use  (line 6104, static)
- [x] make_scroll_bar_position  (line 6117, static)
- [x] coords_in_menu_bar_window  (line 6130, static)
- [x] coords_in_tab_bar_window  (line 6153, static)
- [x] save_line_number_display_width  (line 6171, static)
- [x] line_number_mode_hscroll  (line 6192, static)
- [x] make_lispy_event  (line 6230, static)
- [x] make_lispy_movement  (line 7310, static)
- [x] make_lispy_switch_frame  (line 7337, static)
- [x] make_lispy_focus_in  (line 7343, static)
- [x] make_lispy_focus_out  (line 7349, static)
- [x] parse_modifiers_uncached  (line 7365, static)
- [x] apply_modifiers_uncached  (line 7479, static)
- [x] lispy_modifier_list  (line 7541, static)
- [x] parse_modifiers  (line 7566)
- [x] apply_modifiers  (line 7631, static)
- [x] reorder_modifiers  (line 7700)
- [x] modify_event_symbol  (line 7745, static)
- [x] parse_solitary_modifier  (line 7920)
- [x] lucid_event_type_list_p  (line 8010)
- [x] get_input_pending  (line 8052, static)
- [x] gobble_input  (line 8075)
- [x] tty_read_avail_input  (line 8170)
- [x] handle_async_input  (line 8346, static)
- [x] process_pending_signals  (line 8370)
- [x] unblock_input_to  (line 8382)
- [x] unblock_input  (line 8405)
- [x] totally_unblock_input  (line 8414)
- [x] handle_input_available_signal  (line 8422)
- [x] deliver_input_available_signal  (line 8441, static)
- [x] add_user_signal  (line 8468)
- [x] handle_user_signal  (line 8490, static)
- [x] deliver_user_signal  (line 8531, static)
- [x] find_user_signal_name  (line 8537, static)
- [x] store_user_signal_events  (line 8549, static)
- [x] menu_separator_name_p  (line 8608)
- [x] menu_bar_items  (line 8641)
- [x] menu_bar_item  (line 8787, static)
- [x] menu_item_eval_property_1  (line 8860, static)
- [x] eval_dyn  (line 8871, static)
- [x] menu_item_eval_property  (line 8879)
- [x] parse_menu_item  (line 8899)
- [x] tab_bar_items  (line 9211)
- [x] process_tab_bar_item  (line 9297, static)
- [x] set_prop_tab_bar  (line 9329, static)
- [x] parse_tab_bar_item  (line 9382, static)
- [x] init_tab_bar_items  (line 9527, static)
- [x] append_tab_bar_item  (line 9541, static)
- [x] tool_bar_items  (line 9595)
- [x] process_tool_bar_item  (line 9681, static)
- [x] set_prop  (line 9713, static)
- [x] parse_tool_bar_item  (line 9772, static)
- [x] init_tool_bar_items  (line 10041, static)
- [x] append_tool_bar_item  (line 10055, static)
- [x] read_char_x_menu_prompt  (line 10097, static)
- [x] follow_key  (line 10158, static)
- [x] read_char_minibuf_menu_prompt  (line 10165, static)
- [x] active_maps  (line 10391, static)
- [x] access_keymap_keyremap  (line 10434, static)
- [x] keyremap_step  (line 10487, static)
- [x] test_undefined  (line 10549, static)
- [x] init_raw_keybuf_count  (line 10557)
- [x] read_filtered_event  (line 10591, static)
- [x] restore_reading_key_sequence  (line 10832, static)
- [x] requeued_events_pending_p  (line 10850, static)
- [x] read_key_sequence  (line 10901, static)
- [x] read_key_sequence_vs  (line 11907, static)
- [x] detect_input_pending  (line 12037)
- [x] detect_input_pending_run_timers  (line 12045)
- [x] clear_input_pending  (line 12063)
- [x] requeued_command_events_pending_p  (line 12071)
- [x] update_recent_keys  (line 12152, static)
- [x] stuff_buffered_input  (line 12479)
- [x] set_waiting_for_input  (line 12511)
- [x] clear_waiting_for_input  (line 12525)
- [x] handle_interrupt_signal  (line 12539, static)
- [x] deliver_interrupt_signal  (line 12572, static)
- [x] write_stdout  (line 12580, static)
- [x] read_stdin  (line 12587, static)
- [x] handle_interrupt  (line 12610, static)
- [x] quit_throw_to_read_char  (line 12750, static)
- [x] init_kboard  (line 13126, static)
- [x] allocate_kboard  (line 13158)
- [x] wipe_kboard  (line 13174, static)
- [x] delete_kboard  (line 13182)
- [x] init_keyboard  (line 13223)
- [x] init_while_no_input_ignore_events  (line 13338, static)
- [x] is_ignored_event  (line 13361, static)
- [x] syms_of_keyboard  (line 13414)
- [x] syms_of_keyboard_for_pdumper  (line 14517, static)
- [x] keys_of_keyboard  (line 14548)
- [x] mark_kboards  (line 14633)

## Macros defined — 19
- [x] FUNCTION_KEY_OFFSET  (line 5505, body b97ec3f63bd3)
- [x] GROW_RAW_KEYBUF  (line 156, body 9b0de34030ee)
- [x] INPUT_EVENT_POS_MAX  (line 3882, body c3e4f484030e)
- [x] INPUT_EVENT_POS_MIN  (line 3885, body 1be3d3ca335f)
- [x] ISO_FUNCTION_KEY_OFFSET  (line 5597, body 99971e5035e1)
- [x] KEY_TO_CHAR  (line 7563, body f5a9f69847e3)
- [x] MAX_ENCODED_BYTES  (line 2421, body 1574bddb75c7)
- [x] MAX_NUM_RECENT_KEYS  (line 128, body fe20b26fbe46)
- [x] MIN_NUM_RECENT_KEYS  (line 122, body 8291135bcd97)
- [x] MULTI_LETTER_MOD  (line 7935, body 1a8f4777f4f3)
- [x] NUM_MOD_NAMES  (line 7535, body cfa87b670427)
- [x] PROP  (line 9711, body 6450e39d0a27)
- [x] PUSH_C_STR  (line 10189, body 23461b8ef01d)
- [x] READABLE_EVENTS_DO_TIMERS_NOW  (line 375, body ee799cd75778)
- [x] READABLE_EVENTS_FILTER_EVENTS  (line 376, body 8ef58e36184c)
- [x] READABLE_EVENTS_IGNORE_SQUEEZABLES  (line 377, body 866deb7d5e44)
- [x] RESUME_POLLING  (line 2318, body 0458ffaf5a2f)
- [x] SINGLE_LETTER_MOD  (line 7931, body 07ac213d862b)
- [x] STOP_POLLING  (line 2314, body d471605f3941)

## Types and their fields — 4
- [x] event_head: short var, short kind
- [x] kboard_stack: KBOARD* kboard, struct kboard_stack* next
- [x] timespec: struct timespec difference, struct timespec nexttime, Lisp_Object timers, Lisp_Object idle_timers
- [x] user_signal_info: int sig, char* name, int npending, struct user_signal_info* next

## File-scope objects — 84
- [x] KBOARD initial_kboard  (line 107)
- [x] KBOARD current_kboard  (line 108)
- [x] static KBOARD all_kboards  (line 109, static)
- [x] static bool single_kboard  (line 112, static)
- [x] bool reading_key_sequence  (line 117)
- [x] static int recent_keys_index  (line 132, static)
- [x] static int total_keys  (line 135, static)
- [x] static int lossage_limit  (line 138, static)
- [x] static Lisp_Object recent_keys  (line 141, static)
- [x] Lisp_Object this_command_keys  (line 148)
- [x] ptrdiff_t this_command_key_count  (line 149)
- [x] static Lisp_Object raw_keybuf  (line 153, static)
- [x] static int raw_keybuf_count  (line 154, static)
- [x] static ptrdiff_t this_single_command_key_start  (line 162, static)
- [x] sigjmp_buf return_to_command_loop  (line 167)
- [x] static Lisp_Object recover_top_level_message  (line 170, static)
- [x] static Lisp_Object regular_top_level_message  (line 175, static)
- [x] static bool echoing  (line 179, static)
- [x] static struct kboard ok_to_echo_at_next_pause  (line 184, static)
- [x] struct kboard echo_kboard  (line 191)
- [x] Lisp_Object echo_message_buffer  (line 196)
- [x] int quit_char  (line 214)
- [x] EMACS_INT command_loop_level  (line 217)
- [x] static Lisp_Object unread_switch_frame  (line 226, static)
- [x] static ptrdiff_t last_non_minibuf_size  (line 229, static)
- [x] uintmax_t num_input_events  (line 231)
- [x] ptrdiff_t point_before_last_command_or_undo  (line 232)
- [x] struct buffer buffer_before_last_command_or_undo  (line 233)
- [x] static intmax_t last_auto_save  (line 237, static)
- [x] static ptrdiff_t last_point_position  (line 240, static)
- [x] Lisp_Object internal_last_event_frame  (line 251)
- [x] static Lisp_Object read_key_sequence_cmd  (line 255, static)
- [x] static Lisp_Object read_key_sequence_remapped  (line 256, static)
- [x] static FILE dribble  (line 259, static)
- [x] bool input_pending  (line 262)
- [x] bool input_was_pending  (line 310)
- [x] union buffered_input_event kbd_buffer  (line 314)
- [x] union buffered_input_event kbd_fetch_ptr  (line 318)
- [x] union buffered_input_event kbd_store_ptr  (line 321)
- [x] unsigned timers_run  (line 340)
- [x] struct timespec input_available_clear_time  (line 344)
- [x] bool interrupt_input  (line 348)
- [x] bool interrupts_deferred  (line 351)
- [x] static struct timespec timer_idleness_start_time  (line 355, static)
- [x] static struct timespec timer_last_idleness_start_time  (line 360, static)
- [x] static Lisp_Object virtual_core_pointer_name  (line 364, static)
- [x] static Lisp_Object virtual_core_keyboard_name  (line 365, static)
- [x] static Lisp_Object menu_bar_touch_id  (line 369, static)
- [x] static struct kboard_stack kboard_stack  (line 868, static)
- [x] bool ignore_mouse_drag_p  (line 1289)
- [x] int poll_suppress_count  (line 2016)
- [x] static struct atimer poll_timer  (line 2023, static)
- [x] static Lisp_Object poll_timer_time  (line 2026, static)
- [x] static Lisp_Object help_form_saved_window_configs  (line 2304, static)
- [x] extern int stop_character  (line 3724)
- [x] Lisp_Object pending_funcalls  (line 4730)
- [x] static Lisp_Object accent_key_syms  (line 4968, static)
- [x] static Lisp_Object func_key_syms  (line 4969, static)
- [x] static Lisp_Object mouse_syms  (line 4970, static)
- [x] static Lisp_Object wheel_syms  (line 4971, static)
- [x] static Lisp_Object drag_n_drop_syms  (line 4972, static)
- [x] static Lisp_Object pinch_syms  (line 4973, static)
- [x] static Lisp_Object Vlispy_mouse_stem  (line 5620, static)
- [x] static Lisp_Object button_down_location  (line 5669, static)
- [x] static Lisp_Object frame_relative_event_pos  (line 5673, static)
- [x] static int down_mouse_line_number_width  (line 5677, static)
- [x] static int last_mouse_button  (line 5682, static)
- [x] static int last_mouse_x  (line 5683, static)
- [x] static int last_mouse_y  (line 5684, static)
- [x] static Time button_down_time  (line 5685, static)
- [x] static int double_click_count  (line 5689, static)
- [x] static Lisp_Object modifier_symbols  (line 7537, static)
- [x] static struct user_signal_info user_signals  (line 8465, static)
- [x] static Lisp_Object menu_bar_one_keymap_changed_items  (line 8578, static)
- [x] static Lisp_Object menu_bar_items_vector  (line 8583, static)
- [x] static int menu_bar_items_index  (line 8584, static)
- [x] Lisp_Object item_properties  (line 8784)
- [x] static Lisp_Object tab_bar_items_vector  (line 9186, static)
- [x] static Lisp_Object tab_bar_item_properties  (line 9191, static)
- [x] static int ntab_bar_items  (line 9195, static)
- [x] static Lisp_Object tool_bar_items_vector  (line 9570, static)
- [x] static Lisp_Object tool_bar_item_properties  (line 9575, static)
- [x] static int ntool_bar_items  (line 9579, static)
- [x] static int volatile force_quit_count  (line 12596, static)

# ----------------------------------------------------------------------
## Decisions

**Status: the unit is PORTED — every line of `emacs/src/keyboard.c` has a Rust counterpart.**
`emacs/src/keyboard.c` is 14,682 lines; the port covers **C 96-14682** (the whole file), and
`femacs/src/keyboard.rs` is **15,360 lines** — 4.6% over C's total, inside the 5% default drift
tolerance, so `tools/extract-abi.py --check` reports **no `line count drifted` row**.  Every
`DEFUN` (38) is ported, the temporary scaffold block is **gone**, and `--check` has **zero
`missing` rows**: what remains are **two `does not mention` rows**, each of which needs a
`docs/ports/macro-body-notes.tsv` line (`syms_of_keyboard`'s `doc`, `is_ignored_event`'s
`case, default`).  **The eighth round (C 10071-10855)** landed the key-remap and read-char layer:
`read_char_x_menu_prompt`, `follow_key`, `read_char_minibuf_menu_prompt` (with the `PUSH_C_STR`
macro), `active_maps`, the `keyremap` struct, `access_keymap_keyremap`, `keyremap_step`,
`test_undefined`, `init_raw_keybuf_count`, `read_filtered_event` (with C's `retry:` label loop),
the three subrs `read-char`/`read-event`/`read-char-exclusive`, the cfg-carried
`restore_reading_key_sequence` and `requeued_events_pending_p`.  **The ninth round
(C 10901-11904)** landed the one definition that was next: `read_key_sequence` itself (C
10857-10899 is its doc comment), the ~1,000-line key-sequence reader with C's `replay_entire_sequence`
/ `replay_sequence` / `replay_key` / `not_upcase` / `done` (and the cfg-carried
`disable_text_conversion`) label edges.  **The tenth round (C 11906-12530)** landed the reader's
callers and the input-pending / recent-keys / this-command-keys / suspend layer:
`read_key_sequence_vs`, the two `read-key-sequence` subrs, `detect_input_pending`,
`detect_input_pending_run_timers`, `clear_input_pending`, `requeued_command_events_pending_p`,
the `input-pending-p` and `insert-special-event` subrs, `update_recent_keys`, the
`lossage-size`/`recent-keys`/`this-command-keys`/`set--this-command-keys`/
`this-command-keys-vector`/`this-single-command-keys`/`this-single-command-raw-keys`/
`clear-this-command-keys`/`recursion-depth`/`open-dribble-file`/`discard-input`/
`suspend-emacs` subrs, and the three TTY/waiting entry points `stuff_buffered_input`,
`set_waiting_for_input` and `clear_waiting_for_input` (16 subrs and 9 functions in all; C
12532-12536 is `handle_interrupt_signal`'s comment and its definition begins at C 12538).  **The
eleventh round (C 12538-13166)** landed the SIGINT/interrupt layer (`handle_interrupt_signal`,
`deliver_interrupt_signal`, `write_stdout`, `read_stdin`, the `force_quit_count` object,
`handle_interrupt` and `quit_throw_to_read_char`), the whole input-mode subr family
(`set-input-interrupt-mode`, `set-output-flow-control`, `set-input-meta-mode`, `set-quit-char`,
`set-input-mode`, `current-input-mode`), the three position subrs (`posn-at-x-y`,
`posn-at-point`, `posn-point`) and the two kboard allocators (`init_kboard`,
`allocate_kboard`).  **Every `DEFUN` in the file is now ported** (38 subrs); the next live
definition is C 13173's `wipe_kboard`.  **The twelfth round (C 13173-14682)** closed the file:
`wipe_kboard`, `delete_kboard`, `init_keyboard`, the `struct event_head` + `head_table` pair,
`init_while_no_input_ignore_events`, `is_ignored_event` (the last scaffold entry),
`syms_of_keyboard` (87 `DEFVAR_*`, 129 `DEFSYM`, 38 `defsubr`), `syms_of_keyboard_for_pdumper`,
`keys_of_keyboard`, `mark_kboards`, and the **78 `DEFVAR_*` holders** the unit owes
(`check-globals.py` 78 → 0). What has landed past the second lane's C 4593 is: `process_special_events`,
`swallow_events`, the idle/timer layer (`timer_start_idle` … `decode_timer`, `timer_check_2`,
`timer_check`) and the `current-idle-time` subr; the whole event-translation *data* layer
(`lispy_accent_codes`/`lispy_accent_keys`, the three arms of `lispy_function_keys` with
`lispy_multimedia_keys`, `iso_lispy_function_keys`, `FUNCTION_KEY_OFFSET`/
`ISO_FUNCTION_KEY_OFFSET`, the `scroll_bar_parts`/`internal_border_parts` symbol-index tables, the
`modify_event_symbol` caches, the mouse-state objects and `enum frame_border_side`); and the
helpers `make_lispy_tty_position`, `make_lispy_position`, `toolkit_menubar_in_use`,
`make_scroll_bar_position`, `coords_in_menu_bar_window`, `coords_in_tab_bar_window`,
`save_line_number_display_width` and `line_number_mode_hscroll`; and this lane's
`make_lispy_event`, `make_lispy_movement`, `make_lispy_switch_frame`, `make_lispy_focus_in` and
`make_lispy_focus_out` (C 6230-7352); and this lane's modifier-symbol layer
(`parse_modifiers_uncached` … `lucid_event_type_list_p`, C 7356-8040, with the `KEY_TO_CHAR`,
`SINGLE_LETTER_MOD`, `MULTI_LETTER_MOD` and `NUM_MOD_NAMES` macros, `modifier_names`,
`modifier_symbols`, `reorder_modifiers` and the three subrs `internal-event-symbol-parse-modifiers`,
`event-convert-list` and `internal-handle-focus-in`) and the quiescent-input and user-signal layer
(`get_input_pending` … `store_user_signal_events`, C 8052-8574, with `struct user_signal_info` and
`user_signals`).

**`make_lispy_position` (C 5785-6099) landed in the recovery step that this text starts from**
(its `WINDOW_*` geometry macros — `WINDOW_TAB_LINE_HEIGHT`, `WINDOW_HEADER_LINE_HEIGHT`,
`WINDOW_SCROLL_BAR_AREA_WIDTH`/`_HEIGHT`, `WINDOW_LEFT_SCROLL_BAR_AREA_WIDTH`,
`WINDOW_RIGHT_DIVIDER_WIDTH`, `WINDOW_BOTTOM_DIVIDER_WIDTH`,
`WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_RIGHT`, `WINDOW_XFRAME` and the `WINDOW_*_EDGE_X`/`_Y` family —
are `include/window.rs`'s now, and `WINDOW_HAS_FRINGES_OUTSIDE_MARGINS` is that mirror's `bool`
function, window.h 872's accessor shape).  Its `HAVE_WINDOW_SYSTEM` and `HAVE_EXT_TOOL_BAR` arms
are carried per P-51, and its details are the recovery lane's Structural deviations and Gates
below.  This lane's `make_lispy_event` … `make_lispy_focus_out` (C 6230-7352) are the details of
its own Structural deviations and Gates further down.  **Nothing is unported** — the twelfth
round reached C 14682, the file's last line.  What the closing pass still owes is the two
`does not mention` rows above (both are `docs/ports/macro-body-notes.tsv` lines), and
`tools/check-undef.py` reports **two** undefined references from this unit (down from 36):
`_Vclear_message_function` (eval.c's `clear-message-function`) and `_tool_bar_max_label_size`
(dispnew.c) — both are other units' globals.  The round also turned **21 ratchet entries
stale (now satisfied)** by defining this unit's own `DEFVAR_*` storage.

**Line-count budget (a finding for the integrator).** The finished prefix (C 96-2532) is 2,632
lines for 2,532 C lines (+4.0%); the second lane's C 2533-4593 is 2,306 lines for 2,061 C lines
(+11.9%, of which 143 are the per-unit macros, the use-site declarations and the module headers
the port needs); this lane's text is 1,366 Rust lines for the 1,304 C lines it added (+4.8%:
C 4594-5698, 5713-5784 and 6093-6219); the recovery lane's `make_lispy_position` is the 360 Rust
lines of C 5785-6099 plus the `CHARPOS` macro and the seven use-site declarations (about 40
lines), i.e. 397 Rust lines for 315 C lines (+26.0%, the geometry macros expand from field
reads into calls); this lane's C 6230-7352 is 1,329 Rust lines for 1,123 C lines (+18.3%, its
carried `HAVE_NS`/`HAVE_NTGUI`/`USE_TOOLKIT_SCROLL_BARS` arms and the duplicated
`NON_ASCII_KEYSTROKE_EVENT` body included); the eighth round's C 10071-10855 is 776 Rust lines for
the 785 C lines it added (98.9%: the three subrs' doc strings are one Rust line each, and the
carried `HAVE_TEXT_CONVERSION`/`HAVE_WINDOW_SYSTEM` arms and the `#if 0` blocks are recorded rather
than written); the seventh round's C 8575-10070 is 1,635 Rust lines
for the 1,496 C lines it added (+9.3%: the three bar families, `separator_names`, the
`tool_bar_item_idx` constants at the use site, the `strcpy`/`strnlen` declarations and the carried
`HAVE_EXT_TOOL_BAR` arms); the sixth lane's C 7353-8574 is 1,325 Rust lines for
the 1,222 C lines it added (+8.4%: the modifier-symbol layer, the three new subrs, `struct
user_signal_info` and its four layout assertions, the seven new use-site declarations, the
`user_signals` object, the `FIONREAD` constant, the `INT_STRLEN_BOUND`/`term_initted` macros and
the carried `WINDOWSNT`/`MSDOS`/`HAVE_GPM`/`USG`/`AIX`/`not_aarch64` arms); the ninth round's
C 10901-11904 is 1,064 Rust lines for the 1,004 C lines it added (+6.0%: the cfg-carried
`HAVE_TEXT_CONVERSION` arms, the labelled-loop scaffolding for C's six `goto` targets, and the
`kbd_queue_head_is_switch_frame`/`goto_disable_text_conversion` state the port's two
comma-expression and intra-`if` label edges need).  the tenth round's C 11906-12530 is 613 Rust
lines for the 625 C lines it added (98.1%: the sixteen subrs' doc strings are one Rust line
each, and the ten new use-site declarations live in the file's existing mirror-gap block).
the eleventh round's C 12538-13166 is 622 Rust
lines for the 629 C lines it added (98.9%: the `#include <ignore-value.h>` line, four new
`tty_bits!` arms, seven use-site declarations, the `CHECK_LIVE_FRAME` macro and the carried
`#ifdef` arms); the twelfth round's C 13173-14682 is **1,920 Rust lines for the 1,510 C lines it
added (+27.2%)**: the 87 `DEFVAR_*` docstrings are one long literal each, the 129 `DEFSYM` and
38 `defsubr` lines are 1:1, and the 78 `DEFVAR_*` holders `check-globals.py` requires are one
line each.  The ten covered
slices are 14,518 C lines and 15,360 Rust lines (+5.8%), and the file is **4.6% over** C's 14,682
total — **inside the 5% default tolerance**, which is why `--check` reports no
`line count drifted` row — with nothing left unported.  Most of
this lane's density is **carried dead arms**: of
the 1,366 lines, about 600 are the text the frozen configuration does not compile — the twenty
`#ifdef XK_<name>` element pairs of `lispy_accent_codes` (80 lines for a table whose live value
is twenty zeros), the `HAVE_ANDROID` designated-initialiser table (338 entries, 87 of them
non-zero), the `HAVE_NTGUI` `lispy_function_keys`/`lispy_multimedia_keys` pair (309 lines) and
the two carried position functions.  A lane that skips those arms would land the same live text
in about 750 lines.  `docs/ports/line-drift-tolerance.tsv` has no `keyboard` row (this lane adds
none, per the lane's rules), so the 5% default governs: the file's budget is 14,682 × 1.05 =
15,416 lines, of which **15,360 are spent**: 56 lines of headroom under the 5% default, with
the unit fully covered (a future change of more than ~50 lines needs the integrator's call on
`docs/ports/line-drift-tolerance.tsv`).  Whether the carried arms are worth their lines is the integrator's call — this lane
wrote them because the file's own rule is to carry the text the frozen configuration does not
compile (P-51).

**Temporary scaffold (must be deleted as the remainder lands).** C names the unported functions
in its own prototype block (C 321-410); PORTING.md 0.1-12 drops that block from a *finished* unit
(a declaration plus a definition is `E0428`), but the ported text calls those functions, so the
file ends with an explicitly marked `unsafe extern "C"` block.  The first lane left nine
prototypes in it (`get_input_pending`, `readable_events`, `help_char_p`, `kbd_buffer_get_event`,
`quit_throw_to_read_char`, `read_key_sequence`, `save_getcjmp`, `restore_getcjmp`,
`timer_start_idle`); the second lane deleted the six whose definitions landed and added the
prototypes its own text needs; the third lane deleted the five whose definitions landed
(`timer_start_idle`, `timer_stop_idle`, `timer_resume_idle`, `swallow_events`, `timer_check`);
this lane deleted the three whose definitions landed (`make_lispy_event`, `make_lispy_movement`,
`make_lispy_switch_frame`); the sixth lane (C 7356-8574, below) deleted the five whose
definitions landed (`modify_event_symbol`, `apply_modifiers`, `find_user_signal_name`,
`get_input_pending`, `gobble_input`) and added none (its new callee `store_user_signal_events`
lives in the same batch and only held a temporary entry inside that batch); the seventh round
(C 8577-10070) added none; and the eighth round (C 10071-10855) deleted the four whose
definitions landed (`read_char_x_menu_prompt`, `read_char_minibuf_menu_prompt`, `test_undefined`
and the cfg-carried `restore_reading_key_sequence`) and added one for a callee of its own
(`requeued_command_events_pending_p`, C 12071); the ninth round (C 10901-11904) deleted the one
whose definition landed (`read_key_sequence`), the tenth round (C 11906-12530) deleted the two
whose definitions landed (`detect_input_pending_run_timers` and
`requeued_command_events_pending_p`), the eleventh round (C 12538-13166) deleted the two
whose definitions landed (`handle_interrupt` and `quit_throw_to_read_char`), and the twelfth
round deleted its last entry (`is_ignored_event`, C 13360) together with the block itself — so
**the file no longer has a scaffold**: every callee is defined in this unit or declared at its
use site by the mirror-gap block above (P-72③).  The block was never part of
the port's design: it existed only so that `xtask build-tu src/keyboard.c` would print `built`.  Declarations that *stay* are the mirror gaps (P-26/P-72③): they live in the
`unsafe extern "C"` block above with their `/// C:` line, not in this scaffold.

### Carried arms (P-51) and the frozen conditions

`src/keyboard.c` is **not** in `docs/baseline/config/cond-eval-verified.txt`
(`python3 tools/check-cond-eval.py src/keyboard.c` = `216 conditions, 0 unknown,
0 disagreements [VERIFIED]` — the frozen table decides every condition the way clang does, but the
file has not been added to the list), so the extractor keeps both arms of every `#if` live and the
port carries the text the frozen configuration does not compile, each under
`#[cfg (feature = "…")]`:

- C 44-46, 51-53, 87-89 and the `#include` arms: `HAVE_TEXT_CONVERSION`, `HAVE_ANDROID`,
  `HAVE_WINDOW_SYSTEM`, `MSDOS`, `USG5_4`, `WINDOWSNT` are all `#undef`.
- C 104-120 `HAVE_TEXT_CONVERSION` -> `have_text_conversion` (`reading_key_sequence`).
- C 127-141 `#if INTPTR_MAX <= INT_MAX` -> `intptr_leq_int` (`MAX_NUM_RECENT_KEYS`; its body is
  one of the five "several definitions" macros the gate does not compare).
- C 605-631 (`have_window_system`: `cancel_hourglass`), C 718-725 (`have_window_system`),
  C 951-955 (`have_window_system`, `display_hourglass_p`), C 1098-1100 (`windowsnt`,
  `w32_reset_stack_overflow_guard`), C 1205-1208 (`have_window_system`).
- C 1530-1541 and C 1556-1563 (`HAVE_WINDOW_SYSTEM`'s hourglass arm).
- C 2030-2040 and 2084-2096 (`cygwin_or_dos_nt`), C 2042-2050, 2054-2081, 2098-2108, 2110-2125,
  2127-2152 (`poll_for_input`), C 2443-2452 (`windowsnt`).
- C 1755-1768, 1811-1816 and 1843-1847 are `#if 0` blocks: `dead_branch_map` already sees them
  dead, so nothing is carried (`extract-abi.py`'s own note).  C 5468-5503 (`lispy_kana_keys`
  under `#if 0`) is the third lane's member of that family, and it is treated the same way.
- C 4598-4604 and 4605-4644 (`have_x11`/`have_pgtk`/`have_haiku` and their `#ifdef HAVE_X11`
  sub-arm), C 4645-4665 (`have_haiku`), C 4666-4673 (the chain's live `#else`, an `emacs_abort`):
  `process_special_events`.
- C 4977-5080: the twenty `#ifdef XK_dead_*` element pairs of `lispy_accent_codes`, each carried
  as a `#[cfg (feature = "xk_dead_<name>")]` / `#[cfg (not (…))]` pair of array elements.  The
  X11 keysym macros are not in the frozen macro table (the frozen configuration decides them all
  the same way: undefined), and their *values* are not part of the frozen baseline, so the
  carried arm names the macro the way the reference does.
- C 5109-5207 (`have_android`), C 5209-5461 (`have_ntgui`, `lispy_function_keys` and
  `lispy_multimedia_keys`): the three arms of `lispy_function_keys` are all carried; only the
  live (`#else /* not HAVE_NTGUI */`) one is compile-checked.  `FUNCTION_KEY_OFFSET` itself is
  defined once (the live `0xff00`), because Rust can only have one `const` of that name and the
  gate compares no body for a macro C defines in three branches (the `MAX_ENCODED_BYTES`
  precedent).
- C 5509-5512 (`#if defined HAVE_NS || !defined HAVE_WINDOW_SYSTEM` around the `static` keyword;
  the condition is true here, so the live definition is `static`), C 6124-6145
  (`window_system_no_ext_menu_bar`: `coords_in_menu_bar_window`) and C 6147-6168
  (`have_window_system`: `coords_in_tab_bar_window`) and C 6106-6107 (`have_ext_menu_bar`: the
  `#ifdef` arm of `toolkit_menubar_in_use` — `HAVE_EXT_MENU_BAR` is lisp.h 3929-3932's conditional
  define and is undefined here, so C's `#else` is the live arm, the reading the file's
  C 4297-4309 and C 4474-4490 sites already use).  `make_lispy_position`'s own `#ifdef HAVE_WINDOW_SYSTEM`
  blocks (C 5800-5874 and 6030-6040) and its `#ifndef HAVE_EXT_TOOL_BAR` clause are carried by the
  recovery step (see its Structural deviations below).

The live configuration (`docs/baseline/config/darwin-arm64.macros.tsv`): `HAVE_PTHREAD` 1,
`HAVE_STACK_OVERFLOW_HANDLING` 1, `USABLE_FIONREAD` 1, `subprocesses` 1, `HAVE_PDUMPER` 1;
`HAVE_EXT_MENU_BAR`/`HAVE_EXT_TOOL_BAR` are lisp.h 3929-3932/3935-3937's *conditional* defines
(`USE_X_TOOLKIT`/`HAVE_NTGUI`/`HAVE_NS`/`USE_GTK`/`HAVE_HAIKU` decide both, all `#undef` here),
so both are undefined on this target and the *terminal* menu-bar and tool-bar arms are the live
text — which is why the `#[cfg (feature = "have_ext_menu_bar")]` sites are the dead arms.  (The
`lisp.rs` mirror carries the `#define`'s body as the constant `true`; the enclosing condition is
what decides.  This lane corrected the port's own header comment, which had stated the reason the
other way round while reaching the same conclusion.)

### File-scope objects

`include/keyboard.rs` (238-470) and `include/blockinput.rs` (50) declare this file's objects at the
crate root, so the definitions live in `pub mod keyboard_globals` + glob import: a crate-root
definition of a name a mirror also declares at the crate root is `E0428` (the `frame.rs`
`frame_globals` shape). That module holds `dev_tty`, `interrupt_input_blocked`, `initial_kboard`,
`current_kboard`, `this_command_keys`, `this_command_key_count`, `quit_char`, `num_input_events`,
`point_before_last_command_or_undo`, `buffer_before_last_command_or_undo`,
`internal_last_event_frame`, `input_was_pending`, `kbd_buffer`, `kbd_fetch_ptr`, `kbd_store_ptr`,
`input_available_clear_time`, `prev_buffer`, `ignore_mouse_drag_p` and `poll_suppress_count`.
Objects the mirror declares only inside a module (`lisp.rs` 3853-3894) are plain crate-root
statics, which shadow that glob import as P-21 intends.

### Structural deviations (each also reported for `docs/ports/macro-body-notes.tsv`)

- **C's forward declarations are dropped** (PORTING.md 0.1-12): C 321-323, 325-326 and 379-410.
- **`command_loop` and `recursive_edit_unwind` carry no `#[unsafe(no_mangle)]`**: C 321/323
  declare them `static` and the definitions at C 810/1113 do not repeat the keyword, so C 6.2.2p4
  gives both definitions internal linkage and the reference object exports neither
  (`check-symbols.py --dir src` reported `invented=['_command_loop', '_recursive_edit_unwind']`
  until the attributes came off). They need `linkage` rows in `excluded-macros.txt` (the fns.c
  `concat_to_list` shape); the exact rows are in the lane's report, because this lane must not
  edit that file.
- **`prev_buffer` is a plain `static mut`** inside `keyboard_globals` for the same reason (C 269
  is `static`).
- **C's uninitialised locals are zero-initialised** (PORTING.md 1.5): `adjust_point_for_property`'s
  `beg`/`end`/`val`/`overlay`/`tmp` (rustc's definite assignment cannot see the `&&` chain's
  ordering).
- **`command_loop_1`'s local `prev_buffer` is `prev_buffer_`**: Rust forbids a `let` binding named
  the same as a static (`E0530`); the token comparison strips the trailing underscore.
- **`goto finalize` / `goto start` are the labelled block `'finalize` and the labelled loop
  `'start`** (PORTING.md 2.2's forward/backward-goto shapes).
- **`KVAR` is a per-unit `macro_rules!`** with one arm per member this file names (the term.rs
  shape), so every call site keeps C's tokens.
- **Bitfields go through member-named accessor macros** (P-80): `kboard_bits! (immediate_echo, …)`,
  `frame_bits! (glyphs_initialized_p, …)`, `frame_bits! (mouse_moved, …)`,
  `window_bits! (force_start, …)`, `window_bits! (optional_new_start, …)`,
  `buffer_bits! (long_line_optimizations_p, …)`, `coding_bits! (common_flags, …)`.
- **`STOP_POLLING`/`RESUME_POLLING`** are `do { … } while (0)` macros; the port keeps C's `0` in a
  comment (the notes row is in the report).
- **`local_getcjmp` and `locked` get local copies** where C takes an address of them
  (`&raw mut` of a parameter or of an `AUTO_STRING!` local is not possible).
- **`max`/`min`/`FUNCTIONP` are per-unit macros** (the alloc.rs precedent): the `lisp.rs` mirror
  carries `max`/`min` as `fn`s and does not carry `FUNCTIONP` at all.
- **`message3_frame`'s second argument is `*mut frame`** (lisp.h 4394), not a `Lisp_Object`.

### Structural deviations (the second lane's additions, C 2533-4593)

Each of these is also what the report's `docs/ports/macro-body-notes.tsv` rows answer for.

- **`read_char`'s `goto non_reread` (C 2838) enters the middle of the `wrong_kboard` loop**
  (C 3060) and its `goto wrong_kboard` (C 3100) jumps back inside that same span; the two spans
  cross, so no nesting of Rust's labelled blocks can express both.  The port carries the first
  edge with the two-state switch `goto_non_reread` (PORTING.md 2.2's state-machine arm) and the
  second as `continue 'wrong_kboard`.
- **`read_char`'s `reread_for_input_method:` (C 3259) and `from_macro:` (C 3260) are adjacent
  labels with no statement between them**, so one block (`'reread_for_input_method`) serves both
  gotos plus the fall-through; the port keeps one comment naming both.
- **C's `#ifdef` that cuts an `if`/`else` in two** (C 4148-4176's `HAVE_TEXT_CONVERSION` arm
  before the live chain, and C 4575-4579's `HAVE_X_WINDOWS` `else if`) has no Rust spelling
  (`#[cfg] else` is a parse error, measured): the port tests the carried flag in the chain's head
  (with the arm's own text behind its `#[cfg]`) for the first, and keeps both arms of the chain's
  tail inside the final `else` for the second.
- **`kbd_buffer_store_event` and `kbd_buffer_store_buffered_event` live in
  `pub mod keyboard_root_shadowed_2`** (glob-imported): `keyboard.h` 498/499's mirror already
  declares both at the crate root, so a crate-root definition would be `E0428` (the
  `keyboard_root_shadowed` shape the first lane used for
  `temporarily_switch_to_single_kboard`).  The mirror spells the first one's parameter
  `*mut c_void`, so its call sites cast.
- **Per-unit macros for header constructs the mirrors do not carry as macros** (the `KVAR`/
  `max`/`min` precedent): `input_event_bits!` (termhooks.h 373-412's `kind : 16` / `part : 16` in
  the mirror's `input_event::bits`), `tty_bits!` (termchar.h 217's `showing_menu : 1` in
  `tty_display_info::bits_caps`, the term.rs shape), `WINDOW_FRAME!` (the dispnew.rs shape),
  `CURTTY!` (termchar.h 235) and `TYPE_MAXIMUM!` (intprops.h, one instantiation,
  `TYPE_MAXIMUM (Time)`; `PTRDIFF_MIN`/`PTRDIFF_MAX` come from `lib/include/stdint.in.rs`).
- **`save_getcjmp`'s and `restore_getcjmp`'s parameter shapes**: C's `sys_jmp_buf` parameter is
  the `jmp_buf` array decaying to a pointer and `save_getcjmp`'s body writes through it, so the
  port defines it as `*mut sys_jmp_buf` and the two call sites pass `&raw mut` of the caller's
  local.  The first lane's scaffold declared it by value and called it that way, which would have
  written into the callee's copy; those two lines are corrected here (the scaffold line is
  deleted with the definitions).
- **C's `recorded++ < 0` in `record_char`'s `while`** runs the increment on the failing test too,
  so the port keeps it inside the condition (`while { recorded += 1; recorded - 1 < 0 } && …`).
- **C's uninitialised locals are zero-initialised** (PORTING.md 1.5) in `kbd_buffer_get_event`
  (`obj`, `str`, `pinch_dx`/`pinch_dy`/`pinch_angle`, `bar_window`, `x`, `y`) and `record_char`'s
  `ev1`/`ev2`/`ev3`/`ix1`/`ix2`/`ix3`/`help`/`last_help`/`window`/`last_window`.
- **`ok_to_echo_at_next_pause` and the `w->frame`-style reads** keep C's names through the
  file's existing shapes (`std::ptr::null_mut ()`, `WINDOW_FRAME! (…)`).

### Structural deviations (the third lane's additions, C 4594-6220)

Each of these is also what this lane's report's `docs/ports/macro-body-notes.tsv` rows answer
for.  The rows themselves are in the report: this lane must not edit that file.

- **The event tables are `static mut`, not `static`**: C's `lispy_accent_keys`,
  `lispy_function_keys`, `lispy_multimedia_keys`, `iso_lispy_function_keys`,
  `lispy_wheel_names` and `lispy_drag_n_drop_names` are `static const char *const []`; a Rust
  `static` of that type is rejected because `*const c_char` is not `Sync` (`E0277`), and the type
  is the one the rest of the port uses for C's `const char *`.  A `static mut` needs no `Sync`,
  which is the tree's existing answer (`callint.rs`'s `callint_argfuns`, `fileio.rs`'s `userenv`).
  The two symbol-index tables (`scroll_bar_parts`, `internal_border_parts`) and
  `lispy_accent_codes` are `[c_int]`/`[c_short]` and stay plain `static`s.
- **C's designated initialiser has no Rust counterpart**: the `HAVE_ANDROID` arm of
  `lispy_function_keys` (C 5116-5207) is `[111] = "escape", …` with 338 elements and 251 implicit
  zeros.  The port writes a `static mut` whose value is a const block that fills
  `[std::ptr::null (); 338]` and assigns the 87 designated indexes, which is the same array C's
  initialiser produces (checked element by element against the reference: 87 index/name pairs,
  338 total).
- **`SYMBOL_INDEX (Qx)` is the generated `iQx` constant** (`iQnil as c_short`, …), the
  `frame.rs`/`font.rs` shape: the port's `SYMBOL_INDEX` is a function over the generated symbol
  table and cannot be called from a static initialiser.
- **`process_special_events`'s `copy`/`moved_events` are declared inside the arm that uses
  them**: C declares them at the function's top under `#if defined HAVE_X11 || defined HAVE_PGTK
  || defined HAVE_HAIKU`, and only one arm of the chain can ever be live, so the declaration sits
  in the arm (the `kbd_buffer_get_event` shape this file already uses).  C's `event - kbd_buffer`,
  `kbd_buffer + KBD_BUFFER_SIZE - 1 - kbd_fetch_ptr` and `sizeof *kbd_buffer` are
  `offset_from`/`add`/`size_of` (<ptr> arithmetic, Rust has no `union` pointer arithmetic).
- **`timer_check_2`'s two `continue`s jump to the `do … while` test at the body's end**, which no
  Rust `continue` does: the body is the labelled block `'timers_and_idle_timers`, each `continue`
  is `break 'timers_and_idle_timers`, and the loop's break test is the C condition (PORTING.md
  2.2's loop shape).  C's `timers`/`idle_timers` are its parameters and are reassigned, so the
  port shadows them (`let mut timers = timers;`).
- **C's uninitialised locals are zero-initialised** (PORTING.md 1.5) in
  `make_lispy_tty_position` (`ix`), `save_line_number_display_width` (`pixel_width`) and
  `line_number_mode_hscroll` (`col_width`/`pixel_width`) — the latter three are the callee's
  out-parameters.
- **`!FRAME_UNDECORATED (f)` is `FRAME_UNDECORATED (f) == 0`**: the mirror's accessor is the
  `c_int` C returns (the `dispnew.rs`/`frame.rs` spelling), not a `bool`.
- **`event->part` is the bitfield accessor** `input_event_bits! (part, ev)` (P-80; the mirror
  packs `kind : 16 | part : 16` into `input_event::bits`), and the two `builtin_lisp_symbol`
  calls cast the `c_short` table element to `usize`, while `INT_TO_INTEGER (Time)` casts the
  `unsigned long` to `EMACS_INT`.
- **`make_lispy_tty_position`'s `break`** leaves `FOR_EACH_FRAME!`'s `while`, which is the loop
  C's `break` leaves (the macro's shape is the `dispnew.rs` one).
- **`coords_in_menu_bar_window`/`coords_in_tab_bar_window` are `#[cfg]`-gated whole functions**
  (C compiles neither on this target) whose bodies now spell C's geometry accessors as the
  `WINDOW_TOP_EDGE_Y! (window)`/`WINDOW_LEFT_EDGE_X! (window)`/`WINDOW_BOTTOM_EDGE_Y! (window)`/
  `WINDOW_RIGHT_EDGE_X! (window)` macros — the `term.rs`/`dispnew.rs` macro forms, which is also
  what `include/window.rs` exposes; the four names are compiled out and so unresolved.  (The
  recovery step corrected these two carried dead arms, which had been `WINDOW_TOP_EDGE_Y (window)`
  plain calls; the ledger's older description of them is superseded.)
- **`timer_check_2` cannot mention C's `continue`**: `tools/extract-abi.py --check` reports
  `function timer_check_2 does not mention continue`, which is the one row `keyboard` needs in
  `docs/ports/macro-body-notes.tsv` (this lane must not edit that file, so the row is in its
  report): C's two `continue`s re-test the `do { … } while (CONSP (timers) || CONSP
  (idle_timers))` condition at the body's end, an edge Rust's `continue` does not have, so the
  body is the labelled block `'timers_and_idle_timers` and each `continue` is a
  `break 'timers_and_idle_timers` with the condition as the loop's break test (PORTING.md 2.2's
  loop shape).  `decode_timer` was the other candidate (`vec[0]` written as `*vec`); the port now
  spells it `*vec.offset (0)`, so no note is needed for it.
- **`FUNCTION_KEY_OFFSET`'s two dead `#define`s are recorded, not transcribed** (the tables they
  head are): Rust has one `const` per name, so the single definition carries the frozen
  configuration's `0xff00` and the comment above it names `HAVE_ANDROID`'s `0` and
  `HAVE_NTGUI`'s `0x0` (the `MAX_ENCODED_BYTES` precedent).  `ISO_FUNCTION_KEY_OFFSET` is
  single-valued and is a plain `pub const`.

### Mirror gaps (reported, not fixed here; P-72③ declarations stand at the use site)

The port declares these at its own use site because the mirrors do not carry them:
`lisp.h`'s `larger_vector` (4245), `copy_text` (4323), `message3_nolog` (4393),
`truncate_echo_area` (4401), `quit` (5131), `signal_quit_p`, `clear_message` (4399),
`message_log_maybe_newline` (4400), `print_error_message` (5396), `internal_catch` (4803),
`internal_condition_case` (4805), `internal_condition_case_n` (4808), `pop_handler` (4814),
`push_handler_bind` (4815), `init_eval` (5160), `sit_for` (4373), `display_malloc_warning` (4653),
`safe_eval` (4412), `run_hook_with_args` (4783), `labeled_narrow_to_region` (4929), `find_symbol_value`
(3818 is mirrored, but `safe_funcall`'s module is not reachable here); `dispextern.h`'s
`redisplay_preserve_echo_area`, `display_prop_intangible_p`, `get_large_narrowing_begv/zv`,
`resize_echo_area_exactly`, `resize_mini_window`, `update_redisplay_ticks`; `intervals.h`'s
`get_property_and_range`, `get_char_property_and_overlay`; `<setjmp.h>`'s `sigsetjmp`;
`termchar.h`'s `is_tty_frame`; `coding.h`'s `raw_text_coding_system_p`. `termopts.h` has no mirror
at all (it declares only `interrupt_input`/`interrupts_deferred`, which this file defines).
The scratch declarations for `redisplaying_p`, `display_working_on_window_p`,
`help_echo_showing_p` and `composition_break_at_point` likewise stand in this unit.

The second lane's text adds these, all declared at their use site with a `/// C:` line:
`lisp.h` 4271 `nconc2`, 4397 `message_with_string`, 4809 `internal_condition_case_1`,
5619 `record_unwind_protect_void`; `coding.h` 800 `code_convert_string`; `process.h` 252-254
`hold_keyboard_input`/`unhold_keyboard_input`/`kbd_on_hold_p` and 261
`wait_reading_process_output`; systty.h's `sys_suspend`; `keyboard.h` 500-501's `request_sigio`/
`unrequest_sigio`; `dispextern.h` 3604's `redisplay`; and `<string.h>`'s `memcpy` (this unit's
include list does not pull `lib/include/string.rs` in, the coding.rs shape).  `getchar` and
`PTRDIFF_MIN` were briefly local declarations too; the integrator added both names to
`lib/include/stdio.rs` and `lib/include/stdint.in.rs`, so the local ones came out again
(`check-mirror-dupes.py` reports no keyboard row).

The third lane adds: `<string.h>`'s `memmove` (the `memcpy` gap again, used by
`process_special_events`'s carried `HAVE_X11`/`HAVE_PGTK` arm); `frame.h` 1508's `root_frame` and
1909's `frame_internal_border_part` (`frame.rs`/`dispnew.rs`/`term.rs` declare those for their own
callers, the mirrors do not); `dispextern.h` 4013-4015's `is_tty_child_frame` and
`is_tty_root_frame_with_visible_child` (alongside the first lane's `is_tty_frame`);
`lisp.h` 5111's `line_number_display_width` and 5120's `get_frame_param` (`get_frame_param` is
defined in `frame.rs`, so only the declaration is due here, and the `lisp.rs` mirror does not
carry it).  Two *types* also stand at the use site because no mirror carries them: `frame.h`
34-45's `enum internal_border_part` (the type and its nine members as `c_int` constants, PORTING.md
1.3) and — as one `pub type` plus five constants rather than a mirror — `keyboard.c`'s own
`enum frame_border_side` (C 5691-5698), which is this file's, not a header's.

This lane adds `dispextern.h` 3584-3585's `pixel_to_glyph_coords`, which it reads in **live**
text: `make_lispy_event`'s `MOUSE_CLICK_EVENT` arm calls it where C's
`#if defined HAVE_WINDOW_SYSTEM && !defined HAVE_EXT_MENU_BAR` arm (dead) leaves it after the
`#endif`.  The same arm's *dead* names are deliberately **not** declared — `menu_bar_window`
(`struct frame`, `include/frame.rs` does not carry it), `FRAME_TO_WINDOW_PIXEL_X`/`_Y` and
`x_y_to_column_row` — and neither are the other carried-arm-only names of this lane's text:
`coords_in_menu_bar_window` and `coords_in_tab_bar_window` (this file's own `#[cfg]`-gated
functions), `get_tab_bar_item_kbd`, `x_y_to_column_row`, `KEY_NS_PUT_WORKING_TEXT` and
`find_hot_spot`/`plist_get` (`dispextern.h` 3665/lisp.h 4280).  `dispextern.h` 3410-3430's
`enum tab_bar_item_idx` stands at the use site as one `pub type` plus its seven `c_int` members
(P-72③), because no mirror carries it.  `include/lisp.rs` also does not carry lisp.h 28's
`#include <stdcountof.h>` (P-26), which is why this unit includes
`lib/include/stdcountof.rs` itself (the `alloc.rs`/`bignum.rs` shape).

### Structural deviations (the recovery step's additions, C 5785-6219)

The recovery step's subject is `make_lispy_position` (C 5785-6099) plus the `CHARPOS` macro and
the seven use-site declarations its text needs.

- **`CHARPOS (POS)` is a use-site macro**: dispextern.h 287's `#define CHARPOS(POS) ((POS).charpos)`
  has no mirror, and this file's two uses (`CHARPOS (p.pos)` and `CHARPOS (p.string_pos)` inside
  `make_lispy_position`'s `buffer_posn_from_coords` arm) are the only ported text that names it —
  so the port keeps C's token as `macro_rules! CHARPOS { ($pos:expr) => { ($pos).charpos }; }`
  spelled `CHARPOS! (…)` at both sites (P-72③).
- **Seven symbols stand in the use-site `unsafe extern "C"` block, each with its `/// C:`
  prototype** (P-72③; no mirror carries them): `window.h` 1124-1125's `window_from_coordinates`,
  `dispextern.h` 3582's `estimate_mode_line_height`, 3578-3579's `window_box_width` /
  `window_box_left`, 3883-3887's `buffer_posn_from_coords`, 3888-3891's `mode_line_string` and
  3892-3895's `marginal_area_string`.  (This is why `check-declarations.py` counts more
  declaration sites than the third lane's run did.)
- **C's `#ifndef HAVE_EXT_TOOL_BAR` is written `!cfg! (feature = "have_ext_tool_bar")`**:
  `HAVE_EXT_TOOL_BAR` is lisp.h 3935-3937's *conditional* define (`USE_X_TOOLKIT`/`HAVE_NTGUI`/
  `HAVE_NS`/`USE_GTK`/`HAVE_HAIKU` decide it; all are `#undef` here), so it is undefined on this
  target and the tool-bar clause of `make_lispy_position`'s tab/tool-bar test is the one C does
  not compile.  A `#[cfg]` cannot sit inside an `&&`, so the port keeps C's one expression and
  drops the clause with a `cfg!` test (the C 4148-4176 shape).
- **C's chained assignments are split in C's evaluation order**: C's `dy = yret = wy - …` (and its
  `dx = xret = …` sibling) assigns right-to-left; Rust's assignment is not an expression, so the
  port keeps C's two effects in C's order (`yret = wy - …; dy = yret;`, the frame.rs shape).
- **C's comma expression becomes a block expression**: the carried `HAVE_WINDOW_SYSTEM` hotspot
  arm's `if ((image_map = plist_get (…), !NILP (image_map)) && (hotspot = …, CONSP (hotspot)) && …)`
  is a sequence of `{ …; test }` blocks — the assignments and the tests stay in C's order.
- **C's empty statement is the empty block `{ }`**: `if (… && make_lispy_tty_position (…) != 0) ;`
  has no Rust counterpart, so the port writes the body C's `;` denotes as `{ }` and the `else if`
  chain continues unchanged.
- **The two carried `coords_in_*` dead arms use the `NAME!` macros** (`WINDOW_TOP_EDGE_Y!` /
  `WINDOW_LEFT_EDGE_X!` / `WINDOW_BOTTOM_EDGE_Y!` / `WINDOW_RIGHT_EDGE_X!`), which is the
  `term.rs`/`dispnew.rs` spelling and what `include/window.rs` exposes — see the corrected bullet
  above; that supersedes the ledger's older description of them.
- **Names that only the carried arms mention are not declared**: `dispextern.h` 3665's
  `find_hot_spot` and `lisp.h` 4280's `plist_get` are named only inside the
  `#[cfg (feature = "have_window_system")]` hotspot block, and `struct frame`'s
  `menu_bar_window` / `tab_bar_window` / `tool_bar_window` members only inside that block and the
  two `#[cfg]`-gated `coords_in_*` functions; none of them is declared at the use site (the
  `cancel_hourglass` precedent — the compiled text never reaches them).  `include/frame.rs` does
  not carry the three members.

### Structural deviations (this lane's additions, C 6230-7352)

- **C's `switch (event->kind)` is a `match` on the same arms in the same order** (PORTING.md 2.2,
  the `kbd_buffer_get_event` shape), so `switch`/`case`/`default` cannot reappear; C's `#if`-only
  arms keep their `#[cfg]` (P-51) and C's `#if 0` block (C 6374-6383, the `XK_kana_A` arm) is
  dropped — nothing carries it (the C 5468-5503 rule).
- **C's `NS_NONKEY_EVENT`/`NON_ASCII_KEYSTROKE_EVENT` fall-through is the shared body the carried
  arm spells out under its `#[cfg]`** (the C 4311-4313 shape): `#[cfg (feature = "have_ns")]
  NS_NONKEY_EVENT` and the live `NON_ASCII_KEYSTROKE_EVENT` are two arms with the same text.
- **C's one `continue` inside a `for` becomes the `if` that wraps the rest of the loop's body**:
  C 7131's menu-bar skip in `TOUCHSCREEN_UPDATE_EVENT`'s loop, whose C increment runs after the
  body on every path (the buffer.rs `continue` shape), so the port writes `while CONSP (tem)`
  with the increment last.
- **C's `countof (x)` is the mirror's `countof! (x)`** (`lib/include/stdcountof.rs`).  C's
  `lisp.h` 28 includes `<stdcountof.h>`, which `include/lisp.rs` does not carry (P-26/P-72③), so
  this unit includes the mirror the way `alloc.rs`/`bignum.rs` do.  `#ifdef
  ISO_FUNCTION_KEY_OFFSET` is **live** text (the macro is defined at C 5597 in this file, not by
  the frozen table), so that arm is written unconditionally.
- **The dead `#if defined HAVE_WINDOW_SYSTEM && !defined HAVE_EXT_MENU_BAR` arm of
  `MOUSE_CLICK_EVENT` is carried as a `#[cfg (feature = "window_system_no_ext_menu_bar")]` block**
  next to the live `pixel_to_glyph_coords` call C's `#endif` leaves (`if … else #endif` cannot be
  spelled as a Rust `if`/`else` pair when only one arm compiles).  Its names — `menu_w`,
  `FRAME_TO_WINDOW_PIXEL_X`/`_Y`, `x_y_to_column_row` — are compiled out and never resolved.
- **`TAB_BAR_ITEM_*` stand at the use site**: dispextern.h 3410-3430's `enum tab_bar_item_idx`
  has no mirror and only the two carried `HAVE_WINDOW_SYSTEM` arms of `TOUCHSCREEN_BEGIN_EVENT`/
  `TOUCHSCREEN_END_EVENT` read it (P-72③, PORTING.md 1.3's constant shape).
- **C's unsuffixed integer comparisons are cast at the boundary**: `(*event).code`/`modifiers`
  are `c_uint` (the mirror's unit) and the modifier constants are `c_int`, so every C `&`/`|`
  against them is spelled `X as c_uint`; `XFIXNUM` yields `EMACS_INT` and C's `int max`/`intmax_t`
  locals take an `as` at the assignment (`last_mouse_x`, `fuzz`, `xdiff`/`ydiff`, `new_x`/`new_y`),
  and C's implicit `unsigned long` compare of `event->timestamp - button_down_time` against
  `XFIXNAT (Vdouble_click_time)` is `as Time`.
- **C's `double_click_fuzz as intmax_t` sub-expressions are parenthesised where C's unary minus
  meets a cast** (`(- double_click_fuzz as intmax_t) < xdiff`), because Rust reads `intmax_t <`
  as the start of generic arguments.
- **C's compound literal `(const char *[]) {"pinch"}` is a local array** (`let pinch_names:
  [*const c_char; 1] = [c"pinch".as_ptr ()];`): a C compound literal inside a function has block
  scope, which is what the local is.
- **C's `event->kind` reads are `input_event_bits! (kind, event)`** (P-80; the mirror packs
  `kind : 16 | part : 16` into `input_event::bits`), and the `switch`'s scrutinee is that same
  accessor.
- **The token row `make_lispy_event` needs is in the lane's report**, not in
  `docs/ports/macro-body-notes.tsv` (the integrator's file): its Rust body cannot carry
  `case`/`default`/`switch` (the `match`, as in `kbd_buffer_get_event`) nor C's single
  `continue` (the loop's `if`, as in `buffer.rs`).


### Gates (this lane's run, 2026-10-04)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
- `rustc --cfg enable_checking … build/femacs/units/keyboard/_root.rs` -> 0 errors
- `tools/check-globals.py` -> `273 units, 0 DEFVAR_* variables without a definition in their own unit`
- `tools/check-symbols.py --dir src` -> keyboard `invented=[]`, `missing` = the 75 symbols of the
  unported remainder; the run reports `2 mismatched` (this unit plus the in-flight `src/coding.rs`)
- `tools/check-undef.py` -> `29 NEW`, of which 25 are `referenced by keyboard.o` (the unported
  remainder and units not ported yet) and 4 belong to other in-flight units
- `tools/check-mirror-dupes.py` -> `0 failure(s)`; `tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`

### Gates (the second lane's run, 2026-10-04, C 2533-4593)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
  (green after every function of this lane; the last run is the file's current state)
- `rustc --edition 2024 --crate-type lib --emit obj -C panic=abort -C codegen-units=1
  -C opt-level=0 --cfg enable_checking ../build/femacs/units/keyboard/_root.rs` -> exit 0
  (11 warnings, no errors)
- `tools/extract-abi.py --check emacs/src/keyboard.c femacs/src/keyboard.rs` -> `FAIL`, with
  `212` `missing` rows (102 functions, 38 subrs, 38 `defsubr`s, 26 file-scope objects, 8 macros,
  6 constants) and `line count drifted 66.4%: C 14682 vs Rust 4938`
- `tools/check-symbols.py --dir src` -> keyboard `invented=[]`, `missing` = the 73 symbols of the
  unported remainder; the run reports `2 mismatched` (this unit plus the in-flight `src/window.rs`)
- `tools/check-globals.py` -> `273 units, 0 DEFVAR_* variables without a definition in their own unit`
- `tools/check-mirror-dupes.py` -> `0 failure(s)` (the `PTRDIFF_MIN` and `getchar` rows this lane
  had added were removed after the integrator added both names to the include graph)
- `tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`
- `tools/check-undef.py` -> the unit's undefined references are the scaffold's prototypes and the
  unported remainder (listed in this lane's report)

### Gates (the third lane's run, 2026-10-04, C 4594-4945 and 4966-6220)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
  (green after every batch of this lane; the last run is the file's current state)
- `rustc --edition 2024 --crate-type lib --emit obj -C panic=abort -C codegen-units=1
  -C opt-level=0 --cfg enable_checking ../build/femacs/units/keyboard/_root.rs` -> exit 0
  (no errors)
- `tools/extract-abi.py --check emacs/src/keyboard.c femacs/src/keyboard.rs` -> `FAIL`, with
  `173` `missing` rows (87 functions, 31 subrs, 38 `defsubr`s, 11 file-scope objects, 6 macros,
  0 constants) and `line count drifted 57.1%: C 14682 vs Rust 6304`
- `tools/check-symbols.py --dir src` -> keyboard `invented=[]`, `missing` = the unported
  remainder's 64 exported symbols; the run reports `1 mismatched` (this unit)
- `tools/check-globals.py` -> `274 units, 0 DEFVAR_* variables without a definition in their own
  unit`
- `tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`
  (the row is `insdel.rs`/`bset_redisplay`; no keyboard row)
- `tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`
- `tools/check-undef.py` -> `39 NEW undefined reference(s)`, of which 25 are
  `referenced by keyboard.o` (the scaffold's prototypes and the units not ported yet); none
  of this lane's six new callees appears (they are defined by `frame.rs`/`dispnew.rs`/`term.rs`)
- `tools/check-cond-eval.py src/keyboard.c` -> `216 conditions, 0 unknown, 0 disagreements
  [VERIFIED]` (unchanged; the file is still not on the verified list, which is why the arms above
  are carried)
- `xtask gate-fast` -> **not completed, and the reason is not this unit.**  The condition-evaluator
  step passed (`condition evaluator: verified list agrees with clang`), then the inventory gate
  (`tools/extract-abi.py --all`) ended **without its `checked N ported files` summary and without
  printing a single `FAIL` row**: the log's last lines are 17 `WIP` rows (all `lib/*.h` mirrors)
  and the per-file macro notes of `src/bytecode.c`.  A second run with no timeout on this lane's
  side ended the same way after about twenty minutes, with no traceback in the log; the first run
  this lane made was killed by its own 30-minute timeout at the same point.  The same comparisons
  pass individually (`--check emacs/src/keyboard.c …` and `--check emacs/src/bytecode.c …` both
  return their normal verdicts), so the whole-tree step dying before its summary looks like an
  environment/resource problem in that step rather than a defect this lane introduced — but this
  lane could not prove it, and the steps after the inventory gate (`cargo check`, the
  translation-unit build, the symbol gates) therefore did not run in this window.  The inventory
  gate itself treats this unit as WIP by design (its ledger still has unchecked boxes), so a
  finished `--all` would not fail on it.  Flagged for the integrator, with the log kept at
  `/tmp/gatefast.log`.

### Gates (the recovery step's run, 2026-10-04, C 5785-6219)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
- `rustc --edition 2024 --crate-type lib --emit metadata -C panic=abort -C codegen-units=1
  -C opt-level=0 ../build/femacs/units/keyboard/_root.rs -o /tmp/kbd5.rmeta` -> exit 0
- `tools/extract-abi.py --check emacs/src/keyboard.c femacs/src/keyboard.rs` -> `FAIL`, with
  `172` `missing` rows (86 functions, 31 subrs, 38 `defsubr`s, 11 file-scope objects, 6 macros,
  0 constants) and `line count drifted 54.4%: C 14682 vs Rust 6701`
- `tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`
  (the row is `insdel.rs`/`bset_redisplay`; no keyboard row)
- `tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed literals),
  0 failures`
- `tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`
- `tools/check-globals.py` -> `283 units, 0 DEFVAR_* variables without a definition in their own
  unit`

### Gates (this lane's run, 2026-10-04, C 6230-7352)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
- `rustc --edition 2024 --crate-type lib --emit metadata -C panic=abort -C codegen-units=1
  -C opt-level=0 ../build/femacs/units/keyboard/_root.rs -o /tmp/kbd5.rmeta` -> exit 0
  (11 warnings, no errors — the same eleven the recovery step's run had)
- `tools/extract-abi.py --check emacs/src/keyboard.c femacs/src/keyboard.rs` -> `FAIL`, with
  `167` `missing` rows (81 functions, 31 subrs, 38 `defsubr`s, 11 file-scope objects, 6 macros,
  0 constants), `line count drifted 45.3%: C 14682 vs Rust 8030`, and the one token row
  `function make_lispy_event does not mention case, continue, default, switch`
- `tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`
  (the row is `insdel.rs`/`bset_redisplay`; no keyboard row)
- `tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed literals),
  0 failures`
- `tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`
- `tools/check-globals.py` -> `283 units, 0 DEFVAR_* variables without a definition in their own
  unit`
- `tools/check-symbols.py --dir src` -> keyboard `invented=[]`, `missing` = the unported
  remainder's 64 exported symbols; the run reports `1 mismatched` (this unit)
- `tools/check-undef.py` -> `42 NEW undefined reference(s)`; keyboard.o's share is the
  scaffold's prototypes (including this lane's three new ones) and the DEFVAR carriers.  A
  read-only note: mid-run, another lane's in-flight `lib/include/limits.in.rs` briefly made every
  unit fail to compile (`c_longlong` not imported, `_GL_INTEGER_WIDTH`'s return type changed);
  it was green again before this row was taken, and neither `build-tu` nor the manual `rustc`
  above depends on this unit's text to pass.
- `python3 tools/check-cond-eval.py src/keyboard.c` -> `216 conditions, 0 unknown,
  0 disagreements [VERIFIED]` (unchanged; the file is still not on the verified list, which is
  why the arms above are carried)

### Structural deviations (the sixth lane's additions, C 7356-8574)

The lane's subject is the **modifier-symbol layer** (C 7356-8040: `parse_modifiers_uncached` …
`lucid_event_type_list_p`, with the four macros, `modifier_names`, `modifier_symbols`,
`reorder_modifiers` and the three subrs `internal-event-symbol-parse-modifiers`,
`event-convert-list`, `internal-handle-focus-in`) and the **quiescent-input and user-signal
layer** (C 8052-8574: `get_input_pending`, `gobble_input`, `tty_read_avail_input`,
`handle_async_input`, `process_pending_signals`, the three `unblock_input*` functions,
`handle_input_available_signal`/`deliver_input_available_signal`, `struct user_signal_info`,
`user_signals` and `add_user_signal`/`handle_user_signal`/`deliver_user_signal`/
`find_user_signal_name`/`store_user_signal_events`).

- **C's two unhygienic modifier macros are `macro_rules!` metavariable calls**: C 7390-7413's
  `SINGLE_LETTER_MOD`/`MULTI_LETTER_MOD` (defined a second time with different bodies at C
  7931-7940, inside `parse_solitary_modifier`) read the enclosing `i`, `name`, `this_mod_end` and
  `this_mod`.  Rust's `macro_rules!` binds such names where the macro is *defined*, so each pair
  is declared inside the function C defines it in (C's textual position) and every enclosing
  identifier is passed as a metavariable **named after the C identifier** it carries — the C
  bodies' tokens therefore all reappear, and C's `#undef` has no counterpart (Rust's textual
  scope ends with the block).  Both pairs are C macros with several definitions, so
  `tools/extract-abi.py` does not compare their bodies (only `NUM_MOD_NAMES`' is compared).
- **`NUM_MOD_NAMES` is a `macro_rules!`, not a `const`**: C's `countof (modifier_names)` names a
  `static mut` here (see the `modifier_names` bullet), and Rust refuses to read a mutable static
  from a `const` initialiser (`E0133`, measured), so the port keeps C's object-like macro as a
  `macro_rules!` whose body is C's own expression, evaluated at its one use site (the `alloc.rs`
  `countof! (spare_memory)` shape).
- **`modifier_names` is `static mut`**, the rule the event tables already follow (C's
  `static const char *const []` holds raw pointers, which are not `Sync`; C's `const` is kept by
  the table never being written).  It is one of the two `static const char *const []` tables the
  checklist does not track (the other is the menu-bar layer's `separator_names`, C 8587).
- **`KEY_TO_CHAR` is a file-scope `macro_rules!`** at C's own position (C 7563), and
  **`INT_STRLEN_BOUND` is a `macro_rules!` at its first reader** (`modify_event_symbol`): C's
  `<intprops.h>` has no mirror (P-26), so the port writes gnulib's own arithmetic
  (`((size_of::<T> () * 8 - 1) * 146 + 484) / 485 + 1`, the `openat-proc.rs` shape), and
  `INT_STRLEN_BOUND (EMACS_INT)` is 20.
- **C's `switch` is a `match`** in `parse_modifiers_uncached` and `parse_solitary_modifier`, so
  `case`/`switch` cannot reappear (and `parse_solitary_modifier`'s `break`s are arm exits
  `match` does not need): the two rows this lane hands to the integrator for
  `docs/ports/macro-body-notes.tsv` are in the report, as is `handle_async_input`'s `1` — C's
  `while (1)` is Rust's `loop`, which cannot spell the constant condition (the
  `itree_propagate_limit` class).  This lane must not edit that file.
- **A live arm the previous lane had gated as dead is corrected**: C 4108-4110's
  `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` guard around `read_char`'s
  `gobble_input ();` is **live** on this target (`USABLE_SIGIO` is `1` in
  `docs/baseline/config/darwin-arm64.macros.tsv` line 3829, and clang preprocesses the condition
  true — measured through `tools/check-cond-eval.py`'s probe shape).  The third lane had written
  it as `#[cfg (feature = "usable_sigio_or_usable_sigpoll")]` (its comment said "both `#undef` on
  this target"), which silently dropped a live call; the same condition's two later sites
  (C 8419-8445 and C 8515-8519) are written **live** here, which is what the frozen table says
  (the docs/ports/dispnew.md "carried text that is live on the frozen target" reading).  No other
  `#[cfg (feature = …)]` site in this file is affected: every other feature name stands for a
  condition whose macros are all `#undef` (checked name by name against the frozen table).
- **The use-site declarations this lane adds** (P-72③; no mirror carries them): `<string.h>`'s
  `memcmp`, `stpcpy`, `strcmp` and `memset` (the include list does not pull
  `lib/include/string.rs` in — the `memcpy`/`memmove` gap the third lane recorded), lisp.h 4717's
  `esprintf`, `<sys/ioctl.h>`'s `ioctl`, lisp.h 5137's `terminate_due_to_signal` and frame.h
  1507's `frame_make_pointer_visible`, plus the measured constant `<sys/ioctl.h>`'s
  `FIONREAD` (`0x4004667f`, 1074030207) and the `term_initted!` accessor for termchar.h 62-64's
  `bool_bf term_initted : 1` bit (the dispnew.rs shape).  The dead arms' names
  (`dos_keysns`, `dos_keyread`, `gpm_tty`, `gpm_fd`, `Gpm_GetEvent`, `handle_one_term_event`,
  `close_gpm`, `android_check_query_urgent`) are **not** declared: the compiled text never
  reaches them.
- **`struct user_signal_info`** is `#[repr(C)]` with four `const _: () = assert! (…)` layout
  assertions (measured: size 32, align 8, `sig` 0, `name` 8, `npending` 16, `next` 24), and
  `user_signals` is the `static mut` C declares.
- **`unblock_input`, `unblock_input_to` and `totally_unblock_input` are defined in
  `pub mod keyboard_root_shadowed_3` + glob import**: `include/blockinput.rs`'s `unsafe extern
  "C"` block declares all three at the crate root, so a crate-root definition would be `E0428`
  (the `keyboard_root_shadowed`/`_2` shape).  They still carry `#[unsafe(no_mangle)]` (C's
  definitions are non-`static`).
- **The carried arms of this lane's text** (P-51): C 8199-8202 (`windowsnt`),
  C 8210-8217 (`msdos`), C 8219-8240 (`have_gpm`), C 8256-8259 (`usg_or_cygwin`),
  C 8280-8287 (`aix_and_bsd`) and C 8289-8293 (`not_usable_fionread`), and C 8348-8354 and
  C 8424-8433 (`have_android`, the second with `__aarch64__`'s live arm inside it and its
  `#else` under `not_aarch64`).  C 8260-8262's `#else` is an `#error` arm: `#error` arms are not
  carried (the include-c-ctype.md rule).  The arms the frozen configuration *does* compile are
  written live and are named in the comments: `#ifndef WINDOWSNT` (C 8177-8179 and the whole
  C 8204-8296 body), `#ifdef subprocesses` (C 8183-8188, C 8264-8268), `#ifdef USABLE_FIONREAD`
  (C 8243-8255), `#ifndef DOS_NT` (C 8356-8366), `#if defined (USABLE_SIGIO) || defined
  (USABLE_SIGPOLL)` (C 8419-8445, C 8515-8519) and `#if defined __aarch64__` (C 8428).
- **C's `while (1)` is `loop`**, C's `do { … } while (p->npending > 0)` is `loop` with the test at
  the body's end, C's `for (p = user_signals; p; p = p->next)` is the `while !p.is_null ()` with
  C's increment last, and `tty_read_avail_input`'s `for (i = 0; …)` is the same shape (PORTING.md
  2.2).  `hold_quit`/`buf` are `input_event`s whose `kind` is written through
  `input_event_bits!` (the mirror packs `kind`/`part` into `bits`), and `kbd_buffer_store_event`
  is called with `as *mut c_void` because the mirror's `keyboard.h` 498 declaration spells the
  parameter that way (the `keyboard_root_shadowed_2` note above).

### Gates (this lane's run, 2026-10-04, C 7356-8574)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
- `rustc --edition 2024 --crate-type=lib --emit=metadata --crate-name kbd6_check -C panic=abort
  -C codegen-units=1 -C opt-level=0 ../build/femacs/units/keyboard/_root.rs -o /tmp/kbd6.rmeta`
  -> exit 0 (11 warnings, no errors — the same eleven the previous lane's run had; run from
  `femacs/`, so the pinned `nightly-2026-10-01` toolchain applies)
- `tools/extract-abi.py --check emacs/src/keyboard.c femacs/src/keyboard.rs` -> `FAIL`, with
  `134` `missing` rows (57 functions, 28 subrs, 38 `defsubr`s, 9 file-scope objects, 2 macros,
  0 constants), `line count drifted 36.3%: C 14682 vs Rust 9355`, and **three** `does not
  mention` token rows (`parse_modifiers_uncached`: `case, switch`; `parse_solitary_modifier`:
  `break, case, switch`; `handle_async_input`: `1`) plus
  `function detect_input_pending_run_timers is not static in C but has no #[unsafe(no_mangle)]
  in the port (no C symbol to link)` — that row is the scaffold's `extern` declaration of a
  function C 12045 has not been reached by any lane yet (the same row the previous lane's run
  carried for `gobble_input`, which this lane's definition removed)
- `tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`
  (the row is `insdel.rs`/`bset_redisplay`; no keyboard row)
- `tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed literals),
  0 failures`
- `tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`
- `tools/check-globals.py` -> `283 units, 0 DEFVAR_* variables without a definition in their own
  unit`
- `tools/check-symbols.py --dir src` -> keyboard `invented=[]`, `missing` = the unported
  remainder's 49 `_F*`/`_K` exports; the run reports `1 mismatched` (this unit)
- `tools/check-undef.py` -> `49 NEW undefined reference(s)`, of which 23 are
  `referenced by keyboard.o` (the scaffold's prototypes, the unported remainder and the DEFVAR
  carriers; none of this lane's new callees appears — they are defined by
  `emacs.rs`/`frame.rs`/`atimer.rs`/`sysdep.rs` or in this batch)
- `python3 tools/check-cond-eval.py src/keyboard.c` -> `216 conditions, 0 unknown,
  0 disagreements [VERIFIED]` (unchanged; the file is still not on the verified list, which is
  why the arms above are carried)

#### Rows for the integrator's `docs/ports/macro-body-notes.tsv` (this lane may not edit it)

```
parse_modifiers_uncached	case,switch	keyboard.c 7365-7476: C's `switch (SREF (name, i))` is a `match` on the same byte arms in the same order, so `case`/`switch` cannot reappear (the kbd_buffer_get_event rule)
parse_solitary_modifier	break,case,switch	keyboard.c 7920-8008: the same `switch` -> `match` (so `case`/`switch` are gone) and C's `break` after each arm is the arm's end, which a Rust `match` arm does not spell
handle_async_input	1	keyboard.c 8346-8367: C 8357's `while (1)` is Rust's `loop`, which cannot spell the constant condition (the itree_propagate_limit row)
```

### Structural deviations (the seventh round's additions, C 8577-10070)

The round's subject is the **menu/tab-bar/tool-bar family** (C 8577-10070): the nine bar objects,
`separator_names`/`menu_separator_name_p`, `menu_bar_items`, `item_properties`, `menu_bar_item`,
the three `menu_item_eval_property*` helpers, `parse_menu_item`, and the tab-bar and tool-bar
families each ending in `init_*_bar_items`/`append_*_bar_item`.

- **The bar objects are plain crate-root `static mut`s** (C's nine `static`s, `lisp_h_Qnil` for
  the `Lisp_Object`s), and `item_properties` (C 8784, non-`static`) lives in `keyboard_globals`
  because `keyboard.h` 292 declares it at the crate root (the `kbd_buffer` shape).  Its ledger
  row was already ticked before this round although no definition existed: the mirror's
  `unsafe extern "C"` declaration alone satisfies `tools/extract-abi.py`, which is why the row
  and the definition can differ — this round adds the definition with `#[unsafe(no_mangle)]`.
- **The dropped prototypes**: C 8577's `menu_bar_item` and C 9199-9203/C 9583-9587's eight
  `static` prototypes have no Rust counterpart (PORTING.md 0.1-12; each definition is below).
- **`separator_names` is `static mut`** (C's `static const char *const []`, the event-table
  rule), and `menu_separator_name_p`'s two `return 0`/one `return 1` are the port's
  `false`/`true` (PORTING.md 5.2).
- **C's `maps[nmaps++] = x` is `*maps.offset (nmaps) = x; nmaps += 1;`**: Rust cannot index a raw
  pointer (`E0608`, measured), and the post-increment's store-then-step order is the `ccl.rs`
  `*p++ =` shape.  **`maps` is zero-initialised at its declaration** because `SAFE_NALLOCA!`
  reads `$buf` by value (`pointee_size`), which an uninitialised binding cannot be (P-15's note
  in `include/lisp.rs`); C leaves the pointer uninitialised.
- **C's comma expressions become blocks**: `if (tem = get_local_map (PT, current_buffer,
  Qkeymap), !NILP (tem))` is `if { tem = …; !NILP (tem) }` (three sites), and
  `(item = XCDR (item), !CONSP (item))` inside the parsers' `else if` is the same shape.
- **C's `!inmenubar` is `inmenubar == 0`** (Rust has no `!` on an integer), which also keeps C's
  `0` token in the body; the parsers' `return 0`/`return 1` are `false`/`true`.  Together with the
  offset/step arithmetic (`offset (1)`, `ITEM_PROPERTY_MAX + 1`, `nmaps += 1`) every token C's
  bodies carry is present, so this round hands the integrator **no** new
  `docs/ports/macro-body-notes.tsv` row.
- **C's `XVECTOR (v)->contents + i` is `(*XVECTOR (v)).contents.as_mut_ptr ().offset (i)`** (the
  `alloc.rs` shape, the mirror's flexible-array member): spelling it that way keeps C's
  `XVECTOR`/`contents` tokens, which `xvector_contents` would have dropped (the gate reported
  both rows before this spelling was used).
- **`PROP` is a function-scope `macro_rules!` in each parser**: C defines it twice (C 9327 with
  `tab_bar_item_properties`, C 9711 with `tool_bar_item_properties`) and `#undef`s each at the
  parser's end, so the port puts one in `parse_tab_bar_item` and one in `parse_tool_bar_item`
  (a file-scope pair would be `E0428`).  C defines `PROP` more than once, so
  `tools/extract-abi.py` compares no body for it (only its presence).
- **`enum tool_bar_item_idx` stands at the use site** next to the tab-bar one: `dispextern.h`
  3451-3495 has no mirror, so the type and its thirteen `c_int` members are declared with the
  tab/tool-bar families (P-72③, PORTING.md 1.3's constant shape).
- **`#ifndef HAVE_EXT_TOOL_BAR` is live** (`HAVE_EXT_TOOL_BAR` is lisp.h 3935-3937's *conditional*
  define, and every macro that can define it is `#undef` here — measured with clang through the
  `check-cond-eval` probe shape), so `is_wrap` is a live local, C 9835-9844 and C 9996-10002 are
  live text, and only the `#else` arm `return false;` (C 9929-9932) is carried, under
  `#[cfg (feature = "have_ext_tool_bar")]` (P-51).
- **The use-site declarations this round adds** (P-72③, no mirror carries them): `<string.h>`'s
  `strnlen` (`menu_separator_name_p` is its first reader) and `strcpy` (`parse_tool_bar_item`),
  and lisp.h 4799's `FUNCTIONP` (data.c; `parse_menu_item` is its first reader).  `larger_vector`
  turned out to be already reachable through the include chain (the declaration was rejected with
  `E0428`, so it was not added).

### Gates (this round's run, 2026-10-04, C 8577-10070)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
- `rustc --edition 2024 --crate-type=lib --emit=metadata --crate-name kbd6_check -C panic=abort
  -C codegen-units=1 -C opt-level=0 ../build/femacs/units/keyboard/_root.rs -o /tmp/kbd6.rmeta`
  -> exit 0 (11 warnings, no errors; run from `femacs/`, so the pinned `nightly-2026-10-01`
  toolchain applies)
- `tools/extract-abi.py --check emacs/src/keyboard.c femacs/src/keyboard.rs` -> `FAIL`, with
  `105` `missing` rows (38 functions, 28 subrs, 38 `defsubr`s, 0 file-scope objects, 1 macro,
  0 constants), `line count drifted 25.1%: C 14682 vs Rust 10990`, **no** `does not mention` row
  from this round's text, and the one other row
  `function detect_input_pending_run_timers is not static in C but has no #[unsafe(no_mangle)]
  in the port` — the scaffold's declaration of C 12045, which no lane has reached yet
- `tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`
  (the row is `insdel.rs`/`bset_redisplay`; no keyboard row)
- `tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed literals),
  0 failures`
- `tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`
- `tools/check-globals.py` -> `283 units, 0 DEFVAR_* variables without a definition in their own
  unit`
- `tools/check-undef.py` -> `54 NEW undefined reference(s)`, of which 26 are
  `referenced by keyboard.o` (the scaffold's remaining prototypes, the unported helpers
  `echo_keystrokes_help`/`input_pending_p_filter_events`/`num_input_keys` and the DEFVAR
  carriers); none of this round's callees appears (they are defined in this unit or by
  `keymap.rs`/`alloc.rs`/`buffer.rs` and the data.c-side units)
- `python3 tools/check-cond-eval.py src/keyboard.c` -> `216 conditions, 0 unknown,
  0 disagreements [VERIFIED]` (unchanged)

### Structural deviations (the eighth round's additions, C 10071-10855)

The round's subject is the **key-remap and read-char layer**: `read_char_x_menu_prompt`,
`follow_key`, `read_char_minibuf_menu_prompt`, `active_maps`, `keyremap`, `access_keymap_keyremap`,
`keyremap_step`, `test_undefined`, `init_raw_keybuf_count`, `read_filtered_event`, the three subrs
`read-char`/`read-event`/`read-char-exclusive`, `restore_reading_key_sequence` and
`requeued_events_pending_p`.

- **C's `retry:` label is the labelled loop it is re-entered from**: `read_filtered_event`'s three
  `goto retry`s (C 10636, 10648, 10677) are `continue 'retry`, and the `do … while
  (FIXNUMP (val) && XFIXNUM (val) == -2)` is the inner `loop` whose test ends it (the PORTING.md
  2.2 loop shape — the same treatment `read_char` got, and the reason the gate reports the body's
  `goto` token).
- **C's int literals passed to `bool` parameters are `true`/`false`** (PORTING.md 5.2): the
  `get_keymap (…, 0, 1)`/`access_keymap (…, 1, 0)` pairs, `read_filtered_event`'s five bool
  arguments and `message_with_string`'s `log` argument.  Every one of them is a token the port's
  text cannot spell, so they are handed to the integrator as `macro-body-notes.tsv` rows rather
  than quietly dropped (the lane's report lists the seven rows verbatim); unaffected bodies that
  happen to carry the same digits elsewhere (`input - 1`, `end - start + 1`, `make_fixnum (0)`)
  keep their tokens and need no row.
- **C's `NULL` arguments are `std::ptr::null_mut ()`** (the `read_char` calls' `end_time` and
  `used_mouse_menu`), which the gate's token fold accepts for C's `NULL` (it did not report the
  `null` token for `read_filtered_event`/`read_char_minibuf_menu_prompt`).
- **`struct keyremap` is `#[repr(C)]` with six layout assertions** (measured: size 24, align 8,
  `parent` 0, `map` 8, `start` 16, `end` 20); C's `volatile keyremap *fkey` is the plain
  `*mut keyremap` (Rust has no `volatile`, the port's existing reading).
- **C's `PUSH_C_STR` macro lives in `read_char_minibuf_menu_prompt`**: C never `#undef`s it (its
  preprocessor scope is the rest of the file) but uses it only there, so the `macro_rules!` sits
  in that function — the closest analogue of C's textual scope.  C defines `PROP`/`SINGLE_LETTER_MOD`
  more than once and gets its bodies skipped; `PUSH_C_STR` is defined once, so its body *is*
  compared and carries C's `listvar`, `Fcons`, `build_unibyte_string` and `str` tokens as
  metavariable names.
- **C's chained/pre-increment assignments are split in C's order** (PORTING.md 2.2):
  `fkey->start = fkey->end += *diff` is `end += *diff; start = end;`, and
  `fkey->end = ++fkey->start` is `start += 1; end = start;`.
- **The carried arms of this round's text** (P-51; all four conditions are `#undef` here):
  C 10596-10598 and C 10605-10616 (`have_text_conversion`, `read_filtered_event`'s `count` and
  the disable/record pair), C 10693-10697 (`have_text_conversion`'s `return unbind_to (count,
  val)`), C 10600-10603 (`have_window_system`'s hourglass arm) and the whole of C 10829-10843
  (`have_text_conversion`'s `restore_reading_key_sequence` — the function is carried under the
  cfg, which is why the scaffold's entry for it can go: only the same cfg'd arms call it).
  C 10684-10691 is an `#if 0` block (the hourglass restart), dead in C itself, so it is recorded
  here and not transcribed (P-08).
- **The use-site declarations this round adds** (P-72③, no mirror carries them): lisp.h 4802's
  `apply1` (eval.c; `read_char_minibuf_menu_prompt`'s `message3_nolog` call) and lisp.h 5062's
  `barf_if_interaction_inhibited` (eval.c; the three subrs).  `message_with_string` and
  `requeued_command_events_pending_p` needed nothing new: the first is already declared in this
  unit (C 3439's `record_menu_key` band) and the second is a scaffold entry, because C 12071 has
  not been reached.

### Gates (this round's run, 2026-10-04, C 10071-10855)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
- `rustc --edition 2024 --crate-type=lib --emit=metadata --crate-name kb4_check -C panic=abort
  -C codegen-units=1 -C opt-level=0 ../build/femacs/units/keyboard/_root.rs -o /tmp/kb4.rmeta`
  -> exit 0 (11 warnings, no errors; run from `femacs/`, so the pinned `nightly-2026-10-01`
  toolchain applies)
- `tools/extract-abi.py --check emacs/src/keyboard.c femacs/src/keyboard.rs` -> `FAIL`, with
  `91` `missing` rows (27 functions, 25 subrs, 38 `defsubr`s, 1 file-scope object — the unported
  C 12596 `force_quit_count` — 0 macros, 0 constants),
  `line count drifted 19.9%: C 14682 vs Rust 11766`, **three doc-string rows pass** (the three
  subrs' digests `70285ed40d05`/`5ddbfceb4fe8`/`e5a7c5674cd8` were regenerated from the C
  comment and match), and seven `does not mention` rows this lane hands over:
  `read_char_x_menu_prompt` (`0, 1`), `follow_key` (`0, 1`), `access_keymap_keyremap` (`0`),
  `read_filtered_event` (`goto`), `Fread_char` (`0, 1`), `Fread_event` (`0`) and
  `Fread_char_exclusive` (`0, 1`).  Two `is not static in C but has no #[unsafe(no_mangle)]`
  rows are the scaffold's declarations of C 12045 (`detect_input_pending_run_timers`) and
  C 12071 (`requeued_command_events_pending_p`), both in the unported remainder.
- `tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`
  (the row is `insdel.rs`/`bset_redisplay`; no keyboard row)
- `tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed literals),
  0 failures`
- `tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`
- `tools/check-globals.py` -> `283 units, 0 DEFVAR_* variables without a definition in their own
  unit`
- `tools/check-undef.py` -> `76 NEW undefined reference(s)`, of which 36 are
  `referenced by keyboard.o`: 32 are the `_V*` DEFVAR carriers this unit's text reads (ten of
  them — e.g. `Venable_disabled_menus_and_buttons`, `Vunread_input_method_events`,
  `Vtool_bar_separator_image_expression` — are new here, because the eighth round's text reads
  them for the first time) and four are C globals other units own
  (`auto_save_interval`, `auto_save_no_message`, `double_click_fuzz`, `echo_keystrokes_help`).
  None of this round's function callees appears (they are defined in this unit, by
  `keymap.rs`/`eval.rs`/`print.rs`, or are the cfg'd dead arms' names)
- `python3 tools/check-cond-eval.py src/keyboard.c` -> `216 conditions, 0 unknown,
  0 disagreements [VERIFIED]` (unchanged)

#### Rows for the integrator's `docs/ports/macro-body-notes.tsv` (this lane may not edit it)

```
read_char_x_menu_prompt	0,1	keyboard.c 10096-10155: C's `get_keymap (map, 0, 1)` passes int literals to the two bool parameters, which the port spells `false`/`true` (PORTING.md 5.2, the Fset_case_table class)
follow_key	0,1	keyboard.c 10157-10162: the same `get_keymap (keymap, 0, 1)` / `access_keymap (…, 1, 0)` pair of int literals for bool parameters (PORTING.md 5.2)
access_keymap_keyremap	0	keyboard.c 10433-10473: C's `access_keymap (map, key, 1, 0)` passes `0` to `noinherit`, which the port spells `false`; the `1` stays in the body through `Fvector (end - start + 1, …)`
read_filtered_event	goto	keyboard.c 10590-10698: C's three `goto retry`s become `continue 'retry` on the labelled loop (PORTING.md 2.2), which cannot spell `goto`
Fread_char	0,1	keyboard.c 10700-10747: C's `read_filtered_event (1, 1, 1, …)` and `message_with_string (…, 0)` pass int literals to bool parameters, which the port spells `true`/`false` (PORTING.md 5.2)
Fread_event	0	keyboard.c 10749-10782: C's `read_filtered_event (0, 0, 0, …)` and `message_with_string (…, 0)` are the same bool-literal class
Fread_char_exclusive	0,1	keyboard.c 10784-10825: C's `read_filtered_event (1, 1, 0, …)` and `message_with_string (…, 0)` are the same bool-literal class
```

### Structural deviations (the ninth round's additions, C 10901-11904)

The round's subject is `read_key_sequence` itself — the key-sequence reader, one definition, 1,004
C lines (C 10901-11904; C 10857-10899 is its doc comment, transcribed as the port's leading
comment).

- **C's six label edges are the labelled loops and blocks of PORTING.md 2.2.** `replay_entire_sequence`
  and `replay_sequence` are the two outer `loop` blocks and `replay_key` the inner one: C's
  `goto replay_key` skips both the `while` test *and* the body's preamble (the six `eassert`s, the
  `first_unbound` rewrite, the `t >= READ_KEY_ELTS` check and the echo setup), so the port
  emulates the `while` as `'while_body: loop { if !(cond) { break 'while_body; } <preamble>
  'replay_key: loop { <body> break 'replay_key; } }` — `continue 'replay_key` re-enters the body
  without re-testing the condition, and the natural end of the body falls back to the test.
  `goto done` is `break 'done` from a labelled block that encloses the three loops and also owns
  the `read_key_sequence_cmd = current_binding;` statement (C's `done:` label *skips* it);
  `goto not_upcase` is `break 'not_upcase` from the labelled block around the
  upper-case-translation `if`, and the two `break`s inside that `if` (`dc == …` and the `else`)
  are that edge.  The label names survive as the port's block labels, so every C label token
  still appears.
- **Two of C's `goto`s become state, not labels.**  (1) The `#ifdef HAVE_TEXT_CONVERSION` block's
  `goto disable_text_conversion` has its target *inside* the `if`, and its `goto replay_key` is
  the fall-through to the label just below the block: the port writes the `for` scan as a `while`
  that sets `goto_disable_text_conversion` and breaks, then runs the three disable statements only
  when it is set.  (2) `if (CONSP (KVAR (…, kbd_queue)) && (key = XCAR (…), !(…)))` is C's comma
  expression with an assignment that must not run when the first conjunct is false: the port
  assigns `key` and computes `kbd_queue_head_is_switch_frame` inside the `CONSP` test, then tests
  `CONSP (…) && !kbd_queue_head_is_switch_frame` — the same side effect, evaluated once.
- **C's chained assignments are split in C's order** (the eighth round's `fkey->start = fkey->end
  += *diff` precedent): `indec.map = indec.parent = KVAR (…)` is `indec.parent = …; indec.map =
  indec.parent;` (and the `fkey`/`keytran` pairs), `indec.start = indec.end = 0` is `indec.end =
  0; indec.start = indec.end;`, and `indec.end = indec.start = min (last_real_key_start,
  indec.start)` (with the `fkey`/`keytran` pair nested inside the mouse-simplification) is
  `indec.start = min! (…); indec.end = indec.start;` — `min` reads the *old* `start` because the
  port assigns `start` first.
- **C's `int`-for-`bool` and `EMACS_INT`-for-`int` mixes**: no call in this body passes an int
  literal to a `bool` parameter (they are all already `true`); `read_char`'s
  `prevent_redisplay ? -2 : NILP (prompt)` is an `if` (C reads the bool as the `int`
  `commandflag`), and the cfg-carried `#ifdef HAVE_TEXT_CONVERSION` test's `t` is `t != 0`.
  C's `else while (fkey.end < indec.start) { … }` also needs a block in Rust
  (`else { while … }`), one extra brace level with no token of its own.
  `XFIXNUM` yields the port's `isize` (`EMACS_INT`), so the two `modifiers = XFIXNUM (…)` sites
  and the `KEY_TO_CHAR` comparisons carry `as c_int`/`as EMACS_INT`, and
  `XFIXNUM (key) == quit_char` is `== quit_char as EMACS_INT`.
- **`GROW_RAW_KEYBUF` is exercised for the first time, and its comparison needed the cast C's
  promotion performs**: `raw_keybuf_count` is `int` and `ASIZE` yields `ptrdiff_t`, so the macro
  is now `if raw_keybuf_count as isize == ASIZE (raw_keybuf)`.  Nothing had instantiated the
  macro before this round, so the mismatch had never reached a compiler.
- **The carried arms of this round's text** (P-51; the file's `HAVE_TEXT_CONVERSION` is `#undef`):
  C 10968-10972 (`disabled_conversion`'s declaration and its `= false`), C 11014-11026
  (`reading_key_sequence`'s unwind-protect), C 11072-11082 (`disable_text_conversion`'s immediate
  call and `resume_text_conversion`'s record) and C 11096-11129 (the in-loop disable block) — all
  four under `feature = "have_text_conversion"`.
- **The one use-site read this round adds**: `Vprefix_help_command` (C 13968's `DEFVAR_LISP`, in the
  unported remainder) — the read is one more `_Vprefix_help_command` undefined reference of the
  `_V*` DEFVAR-carrier class.  `last_nonmenu_event` needed nothing new (`editfns.rs`/`fns.rs`
  already carry the same reference), and every callee (`active_maps`, `follow_key`,
  `keyremap_step`, `test_undefined`, `read_char`, `add_command_key`, the `echo_*` helpers,
  `parse_modifiers`/`apply_modifiers`) is defined in this unit or a ported one.
- **The ledger's "File-scope objects" list predates `tools/cinventory.py`'s recent change and is
  missing `force_quit_count` (C 12596)** — it appears only in this file's prose (the `--check`
  count and the "not ported" paragraph), not as a checklist row.  Reported, not regenerated (the
  lane may not rewrite the ledger's accounting tables).

### Gates (this round's run, 2026-10-04, C 10901-11904)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
  (11 warnings, no errors; the warnings are the file's existing unused-doc-comment/lint set plus
  the `disable_text_conversion_p` parameter when the `have_text_conversion` feature is off)
- `tools/extract-abi.py --check emacs/src/keyboard.c femacs/src/keyboard.rs` -> `FAIL`, with
  **90** `missing` rows (26 functions — `read_key_sequence`'s row is gone — 25 subrs, 38
  `defsubr`s, 1 file-scope object, 0 macros, 0 constants), `line count drifted 12.6%: C 14682 vs
  Rust 12827`, one `missing type deferred: src/keyboard.c event_head` (the unported C 13310
  struct) and **one** new `does not mention` row this round adds:
  `read_key_sequence does not mention goto, uninit`; the two
  `is not static in C but has no #[unsafe(no_mangle)]` rows are unchanged
  (`detect_input_pending_run_timers`, `requeued_command_events_pending_p` — both scaffold
  declarations).  The doc-string rows are unchanged, and the eighth round's seven
  `does not mention` rows no longer appear: they are now recorded in
  `docs/ports/macro-body-notes.tsv` 1184-1190.
- `tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`
  (the row is `insdel.rs`/`bset_redisplay`; no keyboard row)
- `tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed literals),
  0 failures`
- `tools/check-globals.py` -> `283 units, 0 DEFVAR_* variables without a definition in their own
  unit`
- `tools/check-target-names.py` -> `568 .rs files, 593 map targets, 0 finding(s)`
- `tools/check-undef.py` -> `78 NEW undefined reference(s)`; 35 are `referenced by keyboard.o`
  (33 `_V*` DEFVAR carriers — `_Vprefix_help_command` is the one this round's text adds — plus
  `_auto_save_interval` and `_auto_save_no_message`).  No function this round calls appears
  (they are defined in this unit or a ported one).
- `python3 tools/check-cond-eval.py src/keyboard.c` -> `216 conditions, 0 unknown,
  0 disagreements [VERIFIED]` (unchanged)

#### Row for the integrator's `docs/ports/macro-body-notes.tsv` (this lane may not edit it)

```
read_key_sequence	goto,uninit	keyboard.c 10901-11904: C's six `goto` targets become the labelled loops and blocks of PORTING.md 2.2 (`replay_entire_sequence`/`replay_sequence`/`replay_key`/`not_upcase`/`done` and the cfg-carried `disable_text_conversion`), and the two `goto`s C spells inside an `if`/comma expression carry the `goto_disable_text_conversion`/`kbd_queue_head_is_switch_frame` state, so `goto` cannot reappear; `UNINIT` (C's `echo_start`/`echo_local_start`/`original_uppercase`) is conf_post.h's `= {0,}` marker (`GCC_LINT` defined in the frozen configuration), which the port writes as the zeroed initializers and the macro name cannot reappear (the Fmake_variable_buffer_local class)
```

### Structural deviations (the tenth round's additions, C 11906-12530)

The round's subject is the reader's callers plus the input-pending / recent-keys /
this-command-keys / suspend layer: `read_key_sequence_vs` (C 11906-11959), the two
`read-key-sequence` subrs, four input-pending predicates, the `input-pending-p` and
`insert-special-event` subrs, `update_recent_keys`, twelve more subrs and the three TTY/waiting
entry points.  Sixteen subrs and nine functions in all.

- **C's ternary chain with an `#ifdef` inside its arms becomes a labelled block.**  Rust has no
  attribute position for `else if` (measured: `#[cfg]` before `else if` is a syntax error), so
  `insert-special-event`'s `ie.kind = (… ? … : …)` chain is a `'kind: { … }` labelled block whose
  every arm is `if EQ (XCAR (event), Qfoo) { break 'kind KIND; }`, in C's order, with each dead
  arm behind its own `#[cfg (feature = "…")]` (`have_ntgui`, `have_ns` twice, `have_dbus`,
  `threads_enabled`, `use_file_notify`, `windowsnt`).  The value the block yields is then stored
  through the `input_event_bits!` accessor the file already keeps for the `kind : 16` bitfield
  (`ie.kind` has no Rust field of its own).  C's `(signal_error (…), NO_EVENT)` tail is the
  block's last two statements.
- **`kbd_buffer_store_event`'s mirror declaration wins over the definition** (P-72③/the
  `keyboard_root_shadowed_2` shape): the definition's parameter is `*mut input_event`, but
  `keyboard.h` 498's crate-root declaration spells it `*mut c_void`, so every call site casts —
  `insert-special-event` writes `kbd_buffer_store_event (&raw mut ie as *mut c_void)`, the
  `process_special_events`/`kbd_buffer_store_event_hold` shape already in the file.
- **`ckd_add` is the `overflowing_add` family** (P-15, `docs/ports/token-aliases.tsv`):
  `recursion-depth`'s `ckd_add (&sum, command_loop_level, minibuf_level);` — which C lets discard
  the overflow flag — is written as the `buffer.rs` pair (`let (sum_, ckd_add_overflow) = …;
  sum = sum_; let _ = ckd_add_overflow;`), so the `ckd_add` token survives as `overflowing_add`
  and the flag is still computed.
- **`emacs_fclose` is lisp.h 5315's object-like macro**, so the call site is the file's
  `emacs_fclose! (dribble)` macro, not a function call (a wrapper `fn` would be a different
  function, PORTING.md 0.1-6).
- **C's uninitialised locals are zero-initialised** (PORTING.md 1.5): `suspend-emacs`'s
  `old_width`/`old_height`/`width`/`height` are filled by `get_tty_size` through pointers, which
  Rust's definite-initialisation cannot see through (measured: `E0381` without the initialisers),
  and `read_key_sequence_vs`'s `Lisp_Object keybuf[READ_KEY_ELTS]` is the `command_loop_1`
  precedent's `[lisp_h_Qnil (); …]` array.
- **C's chained/inc-dec and loop shapes**: `stuff_buffered_input`'s `*p++` is the load-then-step
  pair, its `for (; kbd_fetch_ptr != kbd_store_ptr; kbd_fetch_ptr = next_kbd_event (…))` is the
  equivalent `while` (no `continue` in the body), `recent-keys`'s `do … while` is a `loop` with
  the test at the bottom, and `if (++i >= lossage_limit) i = 0;` is `i += 1; if i >= …`.
- **C's `int`-for-`bool`/pointer-literal tokens this round cannot spell** (handed to the
  integrator, not quietly dropped): `set_waiting_for_input`'s `quit_throw_to_read_char (0)` is
  `false` (its parameter is `bool`), and `clear_waiting_for_input`'s
  `input_available_clear_time = 0;` is `std::ptr::null_mut ()` (C's literal `0`, not `NULL`, so
  the gate's `null` fold does not cover it).
- **The carried arm of this round's text** (P-51): C 11930-11933, `read_key_sequence_vs`'s
  `#ifdef HAVE_WINDOW_SYSTEM` (`display_hourglass_p` / `cancel_hourglass`), under
  `feature = "have_window_system"`.  C 11941-11948 is an `#if 0` block (dead in C itself) —
  recorded, not transcribed (P-08).  `stuff_buffered_input`'s `#ifdef SIGTSTP` is **live**
  (`SIGTSTP` is 18 in the frozen configuration), so it is transcribed without a `cfg`.
- **The ten use-site declarations this round adds** (P-72③; they sit at the end of the file's
  mirror-gap `unsafe extern "C"` block): `make_event_array` (lisp.h 4562), `change_frame_size`
  (dispextern.h 3924), `get_tty_size` (dispextern.h 3780), `discard_tty_input` (lisp.h 5289),
  `init_all_sys_modes` (lisp.h 5292 — its function pointer is what `record_unwind_protect_void`
  records), `reset_all_sys_modes` (lisp.h 5293), `sys_subshell` (lisp.h 5287), `stuff_char`
  (lisp.h 5285), `emacs_fdopen` (lisp.h 5320) and `emacs_unlink` (lisp.h 5324).  `emacs_open`,
  `emacs_fclose!`, `report_file_error`, `run_hook`, `end_kbd_macro`,
  `Fcancel_kbd_macro_events`, `Fvconcat`, `Fnreverse`, `Fvector` and `Fexpand_file_name` were
  already reachable.
- **The one new undefined reference this round adds**: `_cannot_suspend` (the `DEFVAR_LISP` in
  the unported `syms_of_keyboard`), the `_V*` DEFVAR-carrier class.

### Gates (this round's run, 2026-10-04, C 11906-12530)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
  (11 warnings, no errors)
- `tools/extract-abi.py --check emacs/src/keyboard.c femacs/src/keyboard.rs` -> `FAIL`, with
  **65** `missing` rows (17 functions, 9 subrs, 38 `defsubr`s, 1 file-scope object, 0 macros,
  0 constants), `line count drifted 8.5%: C 14682 vs Rust 13440`, `missing type deferred:
  src/keyboard.c event_head` and **two** new `does not mention` rows this round adds
  (`set_waiting_for_input does not mention 0`, `clear_waiting_for_input does not mention 0`);
  the ninth round's `read_key_sequence does not mention goto, uninit` is now recorded in
  `docs/ports/macro-body-notes.tsv`, and the two `is not static in C but has no
  #[unsafe(no_mangle)]` rows are gone (both functions are defined here).  All sixteen subr
  rows pass: the doc digests the ledger records (`58569e51213e`, `dd9cad359995`, `12dd059be36d`,
  `1e66da412198`, `bd43255a778a`, `8606d7e36a58`, `1929899c43b3`, `c27b95ad9a4d`, `bb03ef057b92`,
  `109cc4488be2`, `c31713d33004`, `54e95ebb00df`, `97e5dba2bab2`, `66bb95dc7b71`,
  `45ee08fcbeb0`, `ec4f11b4c989`) are reproduced from the port's literals, and every
  `min/max` pair matches.
- `tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`
  (the row is `insdel.rs`/`bset_redisplay`; no keyboard row)
- `tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed literals),
  0 failures`
- `tools/check-globals.py` -> `283 units, 0 DEFVAR_* variables without a definition in their own
  unit`
- `tools/check-target-names.py` -> `580 .rs files, 592 map targets, 0 finding(s)`
- `tools/check-undef.py` -> `79 NEW undefined reference(s)`; `_cannot_suspend` is the one this
  round's text adds, and no callee of this round appears (they are defined in this unit, in a
  ported one, or are the mirror gaps declared above).
- `python3 tools/check-cond-eval.py src/keyboard.c` -> `216 conditions, 0 unknown,
  0 disagreements [VERIFIED]` (unchanged)

#### Rows for the integrator's `docs/ports/macro-body-notes.tsv` (this lane may not edit it)

```
set_waiting_for_input	0	keyboard.c 12510-12522: C's `quit_throw_to_read_char (0)` passes the int literal 0 to the `bool from_signal` parameter, which the port spells `false` (PORTING.md 5.2, the Fread_char class)
clear_waiting_for_input	0	keyboard.c 12524-12530: C's `input_available_clear_time = 0;` assigns the int literal 0 to a `struct timespec *`; the port writes `std::ptr::null_mut ()` (C's `0` is not `NULL`, so the gate's null fold does not cover it)
```

### Structural deviations (the eleventh round's additions, C 12538-13166)

The round's subject is the SIGINT/interrupt layer plus the input-mode subr family, the three
position subrs and the two kboard allocators: eight C functions, the `force_quit_count` object
and the **last nine** `DEFUN`s of the file.

- **Two of the eight are `unsafe extern "C" fn`** (PORTING.md 1.4: C stores their addresses in a
  signal action): `handle_interrupt_signal`'s address goes to `deliver_process_signal`, and
  `deliver_interrupt_signal`'s to `emacs_sigaction_init` (C 13262, still unported) — the
  `handle_user_signal` shape.  `write_stdout`/`read_stdin`/`handle_interrupt` keep plain
  `unsafe fn` (C never takes their address), and `quit_throw_to_read_char` keeps the scaffold's
  `-> !` (C's `AVOID`).  Both new `extern "C"` functions are private and not yet referenced, so
  rustc drops them and their reads from this unit's object — which is why this round adds no
  undefined reference (`tools/check-undef.py`'s `78 NEW` are the standing `_V*`/`_auto_save_*`
  carriers plus one more `_V*` reader, `_Vfunction_key_map`, from `init_kboard`).
- **`force_quit_count` (C 12596) lands with the object**: `static int volatile` becomes
  `static mut force_quit_count: c_int = 0;` (the `interrupt_input_blocked` reading: Rust has no
  `volatile` statics and every access is already in an `unsafe` block), which removes
  `--check`'s `missing file-scope object` row.  **The ledger's `## File-scope objects — 83`
  list still has no row for it** — the list predates the `tools/cinventory.py` change that let
  the extractor see it (recorded in status 3.162); this lane adds no row, per its brief, and
  the integrator picks it up at the unit's closing pass.
- **C's `#if`/`#else` pairs are carried as two cfg'd arms** (the `xk_dead_*` and `sysdep.rs`
  `usable_sigpoll` pair shape), each feature naming the *non-frozen* side's condition:
  `not_sigtstp` (the `#else` of `#ifdef SIGTSTP`, which is 18 here), `msdos` (five arms),
  `dos_nt` (the `#else` of the live `#ifndef DOS_NT`, with the live arm gated
  `not (feature = "dos_nt")`), `have_x_windows`, and the new `no_usable_sigio_or_sigpoll` for
  the `#else` of the live `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)`.
  `poll_for_input` and `threads_enabled` are single-gated dead arms; the live
  `#ifndef DOS_NT`/`#ifndef HAVE_NS` arms with no `#else` text are transcribed ungated (each
  with a comment naming the guard).
- **The one arm whose *fragment* cannot be a block**: inside the live `USABLE_SIGIO` arm, C
  12782-12790's `#ifdef HAVE_X_WINDOWS` is an `if`-prefix whose `else` introduces the live
  statement (`new_interrupt_input = !NILP (interrupt);`).  The port writes the live statement
  under `not (feature = "have_x_windows")` and the dead arm as C's `if`/`else` (whose `else`
  body *is* that statement) under `feature = "have_x_windows"` — the `sysdep.rs` shape, which
  is what keeps the dead `else` in the text.
- **A `cfg`'d *string fragment***: C 12682-12684's `#ifdef MSDOS` contributes a `"\r\n"` prefix
  to one `write_stdout` literal.  Rust has no attribute position inside a call's argument list,
  so that fragment is recorded in a comment; the live literal is C's live one, and the
  token gate blanks string literals, so no token is lost — unlike the arm's `0`s, which the
  MSDOS `cursor_to (SELECTED_FRAME (), 0, 0)` keeps in the text.
- **`tty->flow_control` is a `bool_bf : 1` member**: the port's `tty_display_info` packs it into
  the `bits_caps` byte, so this file's `tty_bits!` macro gains the `flow_control` reader and
  writer (the `term.rs` shape, one arm per member this file names).
- **Pointer/integer spellings this round pins down**: `(*( (*terminal).display_info.tty as
  *mut tty_display_info )).top_frame` (the member is the P-72③ `*mut c_void`; the parens are
  load-bearing — `*x as T` parses as `(*x) as T`, measured as `E0606`);
  `sys_longjmp (&raw mut getcjmp! () as *mut jmp_buf, 1)` (`getcjmp!` yields a `sys_jmp_buf`,
  the function takes `*mut jmp_buf` — the `sys_setjmp` call site's cast); and
  `XSETINT! (y, WINDOW_TO_FRAME_PIXEL_Y! (w, XFIXNUM (y) as c_int) as EMACS_INT)` (C promotes
  in one sum, the port splits the promotion at the macro boundary — the value is a pixel
  coordinate).
- **`CHECK_LIVE_FRAME` is defined here** (frame.h 1387-1390; the `frame.rs` P-72③ shape, since
  neither mirror carries it) and **seven use-site declarations** join the file's mirror-gap
  block: `decode_tty_terminal`/`get_named_terminal` (termhooks.h 962-963),
  `init_sys_modes`/`reset_sys_modes` (lisp.h 5290-5291), `decode_live_window` (window.h 1210),
  `window_box_left_offset` (dispextern.h 3580) and `do_switch_frame` (lisp.h 5119-5122).
  **`#include <ignore-value.h>` (C 83) is cut over**: `write_stdout` is its consumer, so the
  unit now includes `lib/include/ignore-value.rs` (the mirror committed with the five gnulib
  headers in `10ef0c6`).
- **A `DEFUN!` parameter cannot be `mut`**: `posn-at-x-y` reassigns `x`, `y` and
  `frame_or_window`, and `posn-at-point` reassigns `window`, so each keeps C's name in a local
  (the `open-dribble-file` shape).
- **The one token this round cannot spell**: `handle_interrupt_signal`'s `handle_interrupt (1)`
  passes the int literal `1` to a `bool` parameter (C 12567).

### Gates (this round's run, 2026-10-04, C 12538-13166)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
  (11 warnings, no errors)
- `tools/extract-abi.py --check emacs/src/keyboard.c femacs/src/keyboard.rs` -> `FAIL`, with
  **47** `missing` rows (9 functions, **0 subrs — every `DEFUN` in the file is now ported**, 38
  `defsubr`s, **0 file-scope objects** — `force_quit_count` is defined here now — 0 macros,
  0 constants); the `line count drifted` row is **gone** (14,062 vs 14,682 = 4.2% short, inside
  the 5% default tolerance), `missing type deferred: src/keyboard.c event_head` stands (the
  unported C 13310 struct), and **one** `does not mention` row this round adds:
  `handle_interrupt_signal does not mention 1`.  All nine new subr rows pass: the ledger's doc
  digests (`ec2c5f2ea0bc`, `979529ef403e`, `d0dc2c06edd4`, `05a705a78eda`, `88acce0d9fb9`,
  `68f4fa72d27f`, `ed0784f09fe9`, `b412ffe01d32`, `cbe72d645e3e`) are reproduced from the port's
  literals, and every `min/max` pair matches.
- `tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`
  (the row is `insdel.rs`/`bset_redisplay`; no keyboard row)
- `tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed literals),
  0 failures`
- `tools/check-globals.py` -> `283 units, 0 DEFVAR_* variables without a definition in their own
  unit`
- `tools/check-target-names.py` -> `596 .rs files, 593 map targets, 0 finding(s)`
- `tools/check-undef.py` -> `78 NEW undefined reference(s)`, 36 of them `referenced by
  keyboard.o` (34 `_V*` carriers, one more than last round — `_Vfunction_key_map`, read by
  `init_kboard` — plus `_auto_save_interval`/`_auto_save_no_message`).  No callee of this round
  appears: the two new signal entry points are private and unreferenced, so rustc drops them.
- `python3 tools/check-cond-eval.py src/keyboard.c` -> `216 conditions, 0 unknown,
  0 disagreements [VERIFIED]` (unchanged)

#### Row for the integrator's `docs/ports/macro-body-notes.tsv` (this lane may not edit it)

```
handle_interrupt_signal	1	keyboard.c 12538-12569: C's `handle_interrupt (1)` passes the int literal 1 to the `bool in_signal_handler` parameter, which the port spells `true` (PORTING.md 5.2, the Fread_char class)
```

### Structural deviations (the twelfth round's additions, C 13173-14682)

The round closes the file: nine functions, the `struct event_head` + `head_table` pair, the 78
`DEFVAR_*` holders the unit owes, and the removal of the scaffold.

- **The unit's own `DEFVAR_*` storage is defined here.**  C's `DEFVAR_LISP`/`_BOOL`/`_INT` take
  the address of `globals.f_<vname>` and `struct emacs_globals globals` is defined once, in
  `src/alloc.c`; the port's `include/globals.rs` mirrors the names as *declarations*, so
  `tools/check-globals.py` requires the unit that registers one to define it (status 3.29).  The
  round adds **78 holders** (`#[unsafe(no_mangle)] pub static mut NAME: T = lisp_h_Qnil () /
  false / 0;`, one line each, the `alloc.rs` shape) — 58 `Lisp_Object`, 15 `bool`, 5 `EMACS_INT`
  — which also *satisfies* 21 of `check-undef.py`'s ratchet entries.  Three of them
  (`auto_save_interval`, `num_nonmacro_input_events`,
  `multiple_terminals_merge_keyboards`) were previously only *declared* in the crate-root
  `unsafe extern "C"` block, which the definitions replace.
- **`DEFVAR_KBOARD`'s second argument is an offset, not a name.**  C's macro stores
  `offsetof (KBOARD, vname##_)` (lisp.h 3525-3530), so this round's eight call sites pass
  `std::mem::offset_of! (kboard, Vlast_command_)` &c — the `V*_` field spelling the file's own
  `KVAR!` arms already use.  **Finding for the integrator**: `src/callint.rs`'s two
  `DEFVAR_KBOARD!` call sites *do* pass the bare name, and the bare name resolves to
  `include/globals.rs`'s unrelated `pub static mut Vprefix_arg: *mut c_void` — so the recorded
  `kbdoffset` is a *pointer value*, not an offset (`check-undef.py` reports `_Vprefix_arg` /
  `_Vlast_prefix_arg` from `callint.o` for the same reason).  This lane did not touch that file;
  the same two-line fix (`offset_of! (kboard, Vprefix_arg_)`) applies there.
- **`declared_special` needs a per-unit accessor**: `Lisp_Symbol`'s `bits` unit is the mirror's,
  and the member-named writer lives in `lread.rs`; this file defines its own one-arm
  `set_symbol_bits!` (P-80, the `kboard_bits!`/`tty_bits!`/`input_event_bits!` shape) for C
  13978's `XSYMBOL (Qtop_level)->u.s.declared_special = false;`.
- **`struct event_head` and `head_table`** (C 13310-13335) land as `#[repr(C)] pub struct
  event_head { var: c_short, kind: c_short }` with four frozen-clang measurements (`sizeof` 4,
  `alignof` 2, `var` 0, `kind` 2) and `static head_table: [event_head; 12]`, whose
  `SYMBOL_INDEX (Qfoo)` elements are the generated `iQfoo` constants (P-17; the `frame_parms`
  precedent) — which is also what removes `--check`'s long-standing
  `missing type deferred: src/keyboard.c event_head` row.
- **`syms_of_keyboard`'s `DEFVAR_*` calls take C's docstring positionally**, so C's `doc:` label
  cannot reappear: the round's first `docs/ports/macro-body-notes.tsv` row.  Its 129 `DEFSYM`s
  and 38 `defsubr`s are 1:1 in C's order; the two loops (`head_table`'s `Fput` pass and
  `modifier_names`' `intern_c_string` pass) are `while` loops with `countof!`; C's
  `SYMBOL_INDEX`/`XSETINT`/`XSETFASTINT`/`list1`/`list2` keep their names with the port's
  spelling (`iQfoo`, `XSETINT!`, `XSETFASTINT!`, `list1`, `list2`).
- **`is_ignored_event`'s `switch` becomes a `match`** with `'switch:`/`break 'switch` (so
  `switch` and `break` survive as tokens), C's `#ifdef USE_FILE_NOTIFY`/`HAVE_DBUS` arms carried
  under their `#[cfg]`, and `case`/`default` unspellable: the round's second notes row.
- **The round's two `#if 0`/`#ifdef` sites**: `echo-area-clear-hook`'s `DEFVAR_LISP` inside
  `#if 0` (dead in C itself — recorded, not transcribed, P-08) and C 14101-14108's
  `#ifdef USE_LUCID` `DEFVAR_BOOL` + assignment (carried under `use_lucid`).
- **Three new cfg feature names** (P-51; the registry is `femacs/xtask/Cargo.toml`'s
  `[features]` table, 3.183 — this lane may not edit it): `no_usable_sigio_or_sigpoll`
  (`!(USABLE_SIGIO || USABLE_SIGPOLL)`, the outer `#else` in `set-input-interrupt-mode`),
  `no_interrupt_input` (`!INTERRUPT_INPUT`, `init_keyboard`'s `#else`) and `use_lucid`
  (`USE_LUCID`).  The round also *renamed* its first draft's `not_sigtstp` to the registry's
  existing **`no_sigtstp`** (`# !SIGTSTP`, already used by `process.rs`).
- **The one use-site declaration this round adds**: `mark_object` (lisp.h 4436; `alloc.rs`
  defines it, the `lisp.rs` mirror carries `mark_objects` only).
- **One process note**: `femacs/lib/include/stdbit.in.rs` (another lane's WIP mirror, reached
  through `include/lisp.rs`'s `#include <stdbit.h>`) was mid-edit during this round and broke
  `build-tu` for ~10 minutes with three missing `_gl_stdbit_popcount*!` macros; the lane waited
  rather than touching the peer's file, and the build was green again afterwards.

### Gates (this round's run, 2026-10-04, C 13173-14682)

- `xtask build-tu src/keyboard.c` -> `built src/keyboard.rs -> ../build/femacs/keyboard.o`
  (11 warnings, no errors)
- `tools/extract-abi.py --check emacs/src/keyboard.c femacs/src/keyboard.rs` -> `FAIL`, with
  **0 `missing` rows** (0 functions, 0 subrs, 0 `defsubr`s, 0 file-scope objects, 0 macros,
  0 constants), **no `line count drifted` row** (15,360 vs 14,682 = 4.6% over, inside the 5%
  default) and **no `missing type deferred` row** (the `event_head` type landed).  The only rows
  are the round's two `does not mention` rows: `syms_of_keyboard does not mention doc` and
  `is_ignored_event does not mention case, default`.
- `tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`
  (the row is `insdel.rs`/`bset_redisplay`; no keyboard row)
- `tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed literals),
  0 failures`
- `tools/check-globals.py` -> `283 units, 0 DEFVAR_* variables without a definition in their own
  unit` (**78 → 0** with this round's holders)
- `tools/check-target-names.py` -> `601 .rs files, 593 map targets, 0 finding(s)`
- `tools/check-undef.py` -> `27 NEW undefined reference(s)`; only **two** are `referenced by
  keyboard.o` (`_Vclear_message_function`, `_tool_bar_max_label_size` — both other units'
  globals), down from 36, and **21 ratchet entries are now `stale (now satisfied)`**
- `python3 tools/check-cond-eval.py src/keyboard.c` -> `216 conditions, 0 unknown,
  0 disagreements [VERIFIED]` (unchanged)

#### Rows for the integrator's shared tables (this lane may not edit them)

`docs/ports/macro-body-notes.tsv`:
```
syms_of_keyboard	doc	keyboard.c 13413-14514: the 87 `DEFVAR_*` invocations take C's docstring positionally (the mirror's macro parameter is `$doc`), so C's `doc:` label has no counterpart; the docstrings themselves are carried verbatim as the macro's third argument (the `syms_of_data` precedent)
is_ignored_event	case,default	keyboard.c 13360-13409: C's `switch (event->kind)` becomes the port's `match` on the same arms in the same order (PORTING.md 2.2) — the label `'switch` and its `break 'switch` keep `switch`/`break`, so `case` and `default` are the only tokens left
```
`femacs/xtask/Cargo.toml`'s `[features]` table (three new names; alphabetical order, comment = the C condition):
```
no_interrupt_input                          = []  # !INTERRUPT_INPUT
no_usable_sigio_or_sigpoll                  = []  # !(USABLE_SIGIO || USABLE_SIGPOLL)
use_lucid                                   = []  # USE_LUCID
```

## 2026-10-06: the memcpy family's element-vs-byte sweep (status 3.631)

`record_unwind_protect`'s argument copy (C 1958 `memcpy (newargs + 2, args + 2,
(nargs - 2) * word_size)`, port 2199) handed a byte count to `std::ptr::copy_nonoverlapping` with
`*mut Lisp_Object` pointers; it now goes through `u8` pointers, so the count is bytes as in C
(PORTING.md P-93).  Readings: `build-tu src/keyboard.c` -> `built`, `extract-abi --check` -> `OK`.
