# emacs/src/frame.h (1956 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 37
- [x] fset_buffer_list  (line 753)
- [x] fset_buried_buffer_list  (line 758)
- [x] fset_condemned_scroll_bars  (line 763)
- [x] fset_face_hash_table  (line 768)
- [x] fset_focus_frame  (line 780)
- [x] fset_icon_name  (line 785)
- [x] fset_menu_bar_items  (line 790)
- [x] fset_menu_bar_vector  (line 795)
- [x] fset_name  (line 807)
- [x] fset_param_alist  (line 812)
- [x] fset_root_window  (line 817)
- [x] fset_scroll_bars  (line 822)
- [x] fset_selected_window  (line 827)
- [x] fset_old_selected_window  (line 832)
- [x] fset_title  (line 837)
- [x] fset_tab_bar_items  (line 842)
- [x] fset_tool_bar_items  (line 864)
- [x] fset_tool_bar_position  (line 890)
- [x] NUMVAL  (line 896)
- [x] default_pixels_per_inch_x  (line 902)
- [x] default_pixels_per_inch_y  (line 911)
- [x] FRAME_PARENT_FRAME  (line 1234)
- [x] SET_FRAME_ICONIFIED  (line 1437)
- [x] window_system_available  (line 1495)
- [x] SET_FRAME_VISIBLE  (line 1537)
- [x] frame_dimension  (line 1570)
- [x] FRAME_FRINGE_COLS  (line 1585)
- [x] FRAME_LEFT_FRINGE_WIDTH  (line 1593)
- [x] FRAME_RIGHT_FRINGE_WIDTH  (line 1598)
- [x] FRAME_TOTAL_FRINGE_WIDTH  (line 1606)
- [x] FRAME_CHILD_FRAME_BORDER_WIDTH  (line 1612)
- [x] FRAME_INTERNAL_BORDER_WIDTH  (line 1621)
- [x] FRAME_RIGHT_DIVIDER_WIDTH  (line 1632)
- [x] FRAME_BOTTOM_DIVIDER_WIDTH  (line 1638)
- [ ] FACE_FROM_ID  (line 1646)
- [ ] FACE_FROM_ID_OR_NULL  (line 1656)
- [x] flush_frame  (line 1914)

## Macros defined — 139
- [x] AUTO_FRAME_ARG  (line 1425, body 769ac627b88c)
- [x] CHECK_FRAME  (line 1387, body 01f72077840d)
- [x] CHECK_LIVE_FRAME  (line 1390, body 0ae9d7cf2b91)
- [x] EMACS_FRAME_H  (line 20, body da39a3ee5e6b)
- [x] FOR_EACH_FRAME  (line 1404, body 7564f393794a)
- [x] FRAME_ANDROID_P  (line 961, body 7cb6efb98ba5)
- [x] FRAME_BACKGROUND_PIXEL  (line 1376, body bf6ad7d89d52)
- [x] FRAME_BLINK_OFF_CURSOR  (line 1371, body 67b0fb1def2c)
- [x] FRAME_BLINK_OFF_CURSOR_WIDTH  (line 1373, body 03f53c56bc6d)
- [x] FRAME_BOTTOM_MARGIN  (line 1146, body 79f4d67e6940)
- [x] FRAME_BOTTOM_MARGIN_HEIGHT  (line 1151, body 14e999b114f6)
- [x] FRAME_CANON_X_FROM_PIXEL_X  (line 1710, body 5b14eae937e2)
- [x] FRAME_CANON_Y_FROM_PIXEL_Y  (line 1719, body 9181ee17c51a)
- [x] FRAME_COLS  (line 1075, body 30fd881a25f1)
- [x] FRAME_COLUMN_WIDTH  (line 1565, body 97db34c66ab0)
- [x] FRAME_COL_TO_PIXEL_X  (line 1736, body 4a8784889829)
- [x] FRAME_CONDEMNED_SCROLL_BARS  (line 1365, body 17eb00844b59)
- [x] FRAME_CONFIG_SCROLL_BAR_COLS  (line 1296, body be2478e9ae9b)
- [x] FRAME_CONFIG_SCROLL_BAR_HEIGHT  (line 1291, body 6194b41530ee)
- [x] FRAME_CONFIG_SCROLL_BAR_LINES  (line 1301, body ff8150f5b827)
- [x] FRAME_CONFIG_SCROLL_BAR_WIDTH  (line 1286, body 5277adf1e3a0)
- [x] FRAME_COST_BAUD_RATE  (line 1368, body c2e3131ef75c)
- [x] FRAME_CURSOR_WIDTH  (line 1372, body b631ef843fa0)
- [x] FRAME_CURSOR_X_LIMIT  (line 1360, body 7302c30e83b6)
- [x] FRAME_DELETEN_COST  (line 1211, body 554af61cb7c5)
- [x] FRAME_DELETE_COST  (line 1209, body 9fb23121a618)
- [x] FRAME_DESIRED_CURSOR  (line 1370, body a70ca93457ac)
- [x] FRAME_EXTERNAL_MENU_BAR  (line 1082, body 7cb6efb98ba5)
- [x] FRAME_EXTERNAL_TOOL_BAR  (line 1097, body 7cb6efb98ba5)
- [ ] FRAME_FACE_CACHE  (line 1379, body 69ba185823e0)
- [x] FRAME_FOCUS_FRAME  (line 1212, body cb630408b5df)
- [x] FRAME_FOREGROUND_PIXEL  (line 1375, body 8e3285c066a2)
- [x] FRAME_GARBAGED_P  (line 1177, body 05b6649f148b)
- [x] FRAME_HAIKU_P  (line 956, body 7cb6efb98ba5)
- [x] FRAME_HAS_HORIZONTAL_SCROLL_BARS  (line 1280, body a674ada39a9f)
- [x] FRAME_HAS_MINIBUF_P  (line 1051, body 54cbeda3e44a)
- [x] FRAME_HAS_VERTICAL_SCROLL_BARS  (line 1228, body a674ada39a9f)
- [x] FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT  (line 1229, body a674ada39a9f)
- [x] FRAME_HAS_VERTICAL_SCROLL_BARS_ON_RIGHT  (line 1230, body a674ada39a9f)
- [x] FRAME_ICONIFIED_P  (line 1170, body 7742b1544ffe)
- [x] FRAME_IMAGE_CACHE  (line 922, body fedd7cf55a6d)
- [x] FRAME_INITIAL_P  (line 932, body 928ca5040b4d)
- [x] FRAME_INNER_HEIGHT  (line 1820, body 65b48da73bd2)
- [x] FRAME_INNER_WIDTH  (line 1816, body dfb474bcb6d0)
- [x] FRAME_INSERTN_COST  (line 1210, body e4b5966a1d9d)
- [x] FRAME_INSERT_COST  (line 1208, body b6cf1fbffe47)
- [x] FRAME_KBOARD  (line 919, body 403c8709bf50)
- [x] FRAME_LEFT_SCROLL_BAR_AREA_WIDTH  (line 1304, body 31d504cb0f63)
- [x] FRAME_LEFT_SCROLL_BAR_COLS  (line 1319, body e5d40bd25ff8)
- [x] FRAME_LINES  (line 1076, body 372c0b4a5bc3)
- [x] FRAME_LINE_HEIGHT  (line 1561, body 4218c838d76a)
- [x] FRAME_LINE_TO_PIXEL_Y  (line 1732, body 9d533ace5275)
- [x] FRAME_LIVE_P  (line 1043, body fe9f32688149)
- [x] FRAME_MARGINS  (line 1156, body 5d4570af7a2c)
- [x] FRAME_MARGIN_HEIGHT  (line 1161, body 88a0c974550a)
- [x] FRAME_MENU_BAR_HEIGHT  (line 1087, body f2679d3349f1)
- [x] FRAME_MENU_BAR_ITEMS  (line 1367, body 81b3e9364177)
- [x] FRAME_MENU_BAR_LINES  (line 1086, body da0101b0f678)
- [x] FRAME_MESSAGE_BUF_SIZE  (line 1385, body 0031c2bd5895)
- [x] FRAME_MINIBUF_ONLY_P  (line 1046, body 8278024232e6)
- [x] FRAME_MINIBUF_WINDOW  (line 1198, body 67f2bfd42132)
- [x] FRAME_MOUSE_UPDATE  (line 1411, body 90a2b64b485d)
- [x] FRAME_MSDOS_P  (line 941, body 7cb6efb98ba5)
- [x] FRAME_NO_ACCEPT_FOCUS  (line 1265, body a674ada39a9f)
- [x] FRAME_NO_FOCUS_ON_MAP  (line 1264, body a674ada39a9f)
- [x] FRAME_NO_SPECIAL_GLYPHS  (line 1273, body 6efde46628cc)
- [x] FRAME_NO_SPLIT_P  (line 1180, body b2fd8332188e)
- [x] FRAME_NS_P  (line 946, body 7cb6efb98ba5)
- [x] FRAME_OLD_SELECTED_WINDOW  (line 1206, body 278b2aaf09f4)
- [x] FRAME_OVERRIDE_REDIRECT  (line 1262, body a674ada39a9f)
- [x] FRAME_PGTK_P  (line 951, body 7cb6efb98ba5)
- [x] FRAME_PIXEL_HEIGHT  (line 1067, body f558544b33dc)
- [x] FRAME_PIXEL_HEIGHT_TO_TEXT_LINES  (line 1781, body aad7a4760a40)
- [x] FRAME_PIXEL_TO_TEXT_HEIGHT  (line 1810, body ead9bbfd1c3c)
- [x] FRAME_PIXEL_TO_TEXT_WIDTH  (line 1804, body 351e76ed5004)
- [x] FRAME_PIXEL_WIDTH  (line 1066, body 14ebe643fac9)
- [x] FRAME_PIXEL_WIDTH_TO_TEXT_COLS  (line 1774, body bb0dbb9ca55f)
- [x] FRAME_PIXEL_X_FROM_CANON_X  (line 1697, body c2873ad6a998)
- [x] FRAME_PIXEL_X_TO_COL  (line 1768, body 2928949deb96)
- [x] FRAME_PIXEL_Y_FROM_CANON_Y  (line 1703, body 12326d138e40)
- [x] FRAME_PIXEL_Y_TO_LINE  (line 1756, body c6ef1a25ea83)
- [x] FRAME_RES  (line 1018, body 79bb8d9629c4)
- [x] FRAME_RES_X  (line 1016, body ffbed1418c6d)
- [x] FRAME_RES_Y  (line 1017, body 79bb8d9629c4)
- [x] FRAME_RIGHT_SCROLL_BAR_AREA_WIDTH  (line 1310, body 054d10a68bea)
- [x] FRAME_RIGHT_SCROLL_BAR_COLS  (line 1328, body 740b8c43285f)
- [x] FRAME_ROOT_WINDOW  (line 1201, body b996cb0df6ea)
- [x] FRAME_SCALE_FACTOR  (line 1061, body 356a192b7913)
- [x] FRAME_SCROLL_BARS  (line 1364, body 095e1e62befe)
- [x] FRAME_SCROLL_BAR_AREA_HEIGHT  (line 1342, body 706916e05eee)
- [x] FRAME_SCROLL_BAR_AREA_WIDTH  (line 1335, body d1097c7c7af7)
- [x] FRAME_SCROLL_BAR_COLS  (line 1348, body 14cafb8da592)
- [x] FRAME_SCROLL_BAR_LINES  (line 1354, body c3c7af6b4a20)
- [x] FRAME_SELECTED_WINDOW  (line 1204, body 8d9affd0b1cd)
- [x] FRAME_SKIP_TASKBAR  (line 1263, body a674ada39a9f)
- [x] FRAME_SMALLEST_CHAR_WIDTH  (line 1826, body 98538baa3818)
- [x] FRAME_SMALLEST_FONT_HEIGHT  (line 1830, body 144c4761556f)
- [x] FRAME_TAB_BAR_HEIGHT  (line 1091, body fbd9d6b157db)
- [x] FRAME_TAB_BAR_LINES  (line 1090, body 4cad5140081d)
- [x] FRAME_TERMCAP_P  (line 933, body 160b9ad63c45)
- [x] FRAME_TEXT_COLS_TO_PIXEL_WIDTH  (line 1742, body 7b7de3550aca)
- [x] FRAME_TEXT_HEIGHT  (line 1074, body d62da9d58229)
- [x] FRAME_TEXT_LINES_TO_PIXEL_HEIGHT  (line 1748, body 05df9e6d8c8b)
- [x] FRAME_TEXT_TO_PIXEL_HEIGHT  (line 1796, body 30ed3124147c)
- [x] FRAME_TEXT_TO_PIXEL_WIDTH  (line 1790, body ae2afd94326c)
- [x] FRAME_TEXT_WIDTH  (line 1073, body 0ffff95eaf80)
- [x] FRAME_TOOLTIP_P  (line 1270, body 7ef9278e16c2)
- [x] FRAME_TOOL_BAR_BOTTOM_HEIGHT  (line 1123, body 50f3013f22f7)
- [x] FRAME_TOOL_BAR_BOTTOM_LINES  (line 1127, body b5f9b198e2df)
- [x] FRAME_TOOL_BAR_HEIGHT  (line 1108, body 0c73bb0857bb)
- [x] FRAME_TOOL_BAR_LINES  (line 1107, body 98f4a3397441)
- [x] FRAME_TOOL_BAR_POSITION  (line 1104, body 89c9678ae6d9)
- [x] FRAME_TOOL_BAR_TOP_HEIGHT  (line 1113, body 84973bb2624f)
- [x] FRAME_TOOL_BAR_TOP_LINES  (line 1117, body 7f6225b04d97)
- [x] FRAME_TOP_MARGIN  (line 1132, body e12ac62be61d)
- [x] FRAME_TOP_MARGIN_HEIGHT  (line 1139, body 87aeb8371c0f)
- [x] FRAME_TOTAL_COLS  (line 1068, body e5dcea87ed73)
- [x] FRAME_TOTAL_LINES  (line 1069, body d291c9567447)
- [x] FRAME_UNDECORATED  (line 1239, body cbb34082cf17)
- [x] FRAME_VERTICAL_SCROLL_BAR_TYPE  (line 1226, body 0a89fade8d82)
- [x] FRAME_VISIBLE_P  (line 1167, body 13aa5bcc4505)
- [x] FRAME_W32_P  (line 936, body 7cb6efb98ba5)
- [x] FRAME_WANTS_MODELINE_P  (line 1183, body c30d7c2f173a)
- [x] FRAME_WINDOWS_FROZEN  (line 1187, body 62d94e561a3c)
- [x] FRAME_WINDOW_CHANGE  (line 1191, body c5570e3fd7e6)
- [x] FRAME_WINDOW_P  (line 987, body 1379974e8198)
- [x] FRAME_WINDOW_STATE_CHANGE  (line 1195, body 3147954ae44a)
- [x] FRAME_X_P  (line 934, body 1d9e8dd80cfc)
- [x] FRAME_Z_GROUP  (line 1266, body a41827de359d)
- [x] FRAME_Z_GROUP_ABOVE  (line 1268, body 1379974e8198)
- [x] FRAME_Z_GROUP_BELOW  (line 1269, body 1379974e8198)
- [x] FRAME_Z_GROUP_NONE  (line 1267, body c3e599a9b9ef)
- [x] MOUSE_HL_INFO  (line 1038, body 7bdaea6e5b2b)
- [x] SELECTED_FRAME  (line 1526, body 40b81fe06248)
- [x] SET_FRAME_GARBAGED  (line 1173, body 7740c4b03c8e)
- [x] WINDOW_FRAME  (line 929, body d0a15ef83288)
- [x] WINDOW_SYSTEM_RETURN  (line 1471, body 01de564736b0)
- [x] XFRAME  (line 924, body 52a85b0509a9)
- [x] XSETFRAME  (line 926, body 49c7992aa036)

## Types and their fields — 1
- [x] frame: union vectorlike_header header, Lisp_Object name, Lisp_Object icon_name, Lisp_Object title, Lisp_Object parent_frame, Lisp_Object last_mouse_device, Lisp_Object focus_frame, Lisp_Object root_window, Lisp_Object selected_window, Lisp_Object old_selected_window, Lisp_Object minibuffer_window, Lisp_Object param_alist, Lisp_Object scroll_bars, Lisp_Object condemned_scroll_bars, Lisp_Object menu_bar_items, Lisp_Object face_hash_table, Lisp_Object menu_bar_vector, Lisp_Object buffer_predicate, Lisp_Object buffer_list, Lisp_Object buried_buffer_list, Lisp_Object tool_bar_position, Lisp_Object tab_bar_items, Lisp_Object tool_bar_items, struct face_cache* face_cache, EMACS_UINT id, int last_tab_bar_item, int menu_bar_items_used, struct glyph_pool* current_pool, struct glyph_pool* desired_pool, struct glyph_matrix* desired_matrix, struct glyph_matrix* current_matrix, bool_bf glyphs_initialized_p : 1, bool_bf resized_p : 1, bool_bf default_face_done_p : 1, bool_bf already_hscrolled_p : 1, bool_bf updated_p : 1, bool_bf fonts_changed : 1, bool_bf cursor_type_changed : 1, bool_bf redisplay : 1, unsigned visible : 1, bool_bf iconified : 1, bool_bf garbaged : 1, bool_bf wants_modeline : 1, bool_bf auto_raise : 1, bool_bf auto_lower : 1, bool_bf no_split : 1, bool_bf explicit_name : 1, bool_bf window_change : 1, bool_bf window_state_change : 1, bool_bf mouse_moved : 1, bool_bf pointer_invisible : 1, bool_bf frozen_window_starts : 1, ENUM_BF (output_method) output_method : 4, bool_bf undecorated : 1, bool_bf no_accept_focus : 1, bool_bf no_special_glyphs : 1, bool_bf can_set_window_size : 1, bool_bf after_make_frame : 1, bool_bf tab_bar_redisplayed : 1, bool_bf tab_bar_resized : 1, bool_bf tool_bar_redisplayed : 1, bool_bf tool_bar_resized : 1, bool_bf face_change : 1, bool_bf inhibit_clear_image_cache : 1, bool_bf new_size_p : 1, bool_bf was_invisible : 1, bool_bf select_mini_window_flag : 1, bool_bf borders_respect_alpha_background : 1, int change_stamp, ptrdiff_t number_of_windows, int tab_bar_lines, int tab_bar_height, int n_tab_bar_rows, int n_tab_bar_items, int tool_bar_lines, int tool_bar_height, int n_tool_bar_rows, int n_tool_bar_items, char* decode_mode_spec_buffer, int* insert_line_cost, int* delete_line_cost, int* insert_n_lines_cost, int* delete_n_lines_cost, int text_cols, int text_lines, int text_width, int text_height, int total_cols, int total_lines, int pixel_width, int pixel_height, int new_width, int new_height, int left_pos, int top_pos, int win_gravity, int size_hint_flags, int border_width, int child_frame_border_width, int internal_border_width, int right_divider_width, int bottom_divider_width, int left_fringe_width, int right_fringe_width, int fringe_cols, int menu_bar_lines, int menu_bar_height, int column_width, int line_height, struct terminal* terminal, struct tty_output* tty, struct x_output* x, struct w32_output* w32, struct ns_output* ns, struct pgtk_output* pgtk, struct haiku_output* haiku, struct android_output* android, struct font_driver_list* font_driver_list, enum text_cursor_kinds desired_cursor, int cursor_width, enum text_cursor_kinds blink_off_cursor, int blink_off_cursor_width, int config_scroll_bar_width, int config_scroll_bar_cols, int config_scroll_bar_height, int config_scroll_bar_lines, intmax_t cost_calculation_baud_rate, double alpha, double alpha_background, double gamma, int extra_line_spacing, int extra_line_spacing_above, unsigned long background_pixel, unsigned long foreground_pixel, int z_order

## File-scope objects — 7
- [x] extern EMACS_UINT frame_next_id  (line 1428)
- [x] extern bool frame_garbaged  (line 1433)
- [x] extern Lisp_Object selected_frame  (line 1459)
- [x] extern Lisp_Object old_selected_frame  (line 1460)
- [x] extern int frame_default_tab_bar_height  (line 1462)
- [x] extern int frame_default_tool_bar_height  (line 1465)
- [x] extern Lisp_Object Vframe_list  (line 1521)

# ----------------------------------------------------------------------
## Decisions (first slice: the structure)

- **`struct frame` is 568/8**, and all 94 keys assert against the C compiler.
- **The 37 flag members are two 4-byte units.**  Measured with every flag set the bytes at
  240-247 are `ff ff ff ff ff 00 00 00`: the first unit holds the first 21 flags (bits 0…20),
  then `output_method : 4` (bits 21…24) and seven more flags (bits 25…31); the second unit holds
  the last eight.  A single flag set alone lands at bit 0 of unit 1 / bit 7 of unit 2, which is
  how the boundaries were fixed.  The port stores `bits1`/`bits2` and names each member's shift.
- **Pointer degradations (P-72 rule ③).**  `face_cache`, `current_pool`, `desired_pool`,
  `desired_matrix`, `current_matrix` and `font_driver_list` point at display/font types whose
  mirrors are still ahead in the chain (the first five are dispextern.h's, the last is font.h's),
  so they carry `*mut c_void` with a `/// C:` note.  `terminal` *is* mirrored, so it stays
  `*mut terminal`.  The seven `union output_data` members are incomplete in C's view too
  (`termchar.h`/`xterm.h`/… are not included by frame.h), so `*mut c_void` is faithful there.
- **`union output_data`.**  C names both the tag and the member `output_data`; the port keeps
  both (a Rust union type and the frame's field can share the name), and the member is keyed.
- **A scanner gap fixed on the way (P-74).**  `}` and its declarator were on separate lines
  (frame.h 653-663), so `output_data` was invisible to the field scan and the run's 8 bytes went
  unkeyed; the closing-line rule now reads the declarator from the next line.
- **Still to do in this file (177 items).**  The 139 `FRAME_*` macros, 37 inlines and 7
  file-scope objects land next.

# ----------------------------------------------------------------------
## Decisions (second slice: the accessors)

- **136 of the 184 items.**  This slice lands `XFRAME`/`XSETFRAME`/`SELECTED_FRAME`, the 18
  `fset_*` setters, the geometry and predicate accessors, `NUMVAL`,
  `default_pixels_per_inch_x/y`, `window_system_available`, `frame_dimension`, the
  fringe/divider helpers, `SET_FRAME_ICONIFIED` and the seven file-scope objects.
- **The 37 packed flags are read through the two units.**  `FRAME_VISIBLE_P`,
  `FRAME_ICONIFIED_P`, `FRAME_GARBAGED_P`, `FRAME_NO_SPLIT_P`, `FRAME_WANTS_MODELINE_P`,
  `FRAME_WINDOWS_FROZEN`, `FRAME_WINDOW_CHANGE`, `FRAME_WINDOW_STATE_CHANGE`,
  `FRAME_NO_SPECIAL_GLYPHS` and `FRAME_UNDECORATED` mask `bits1`; `output_method` is the
  four-bit field in the same unit, read by `frame_output_method`.  The C names cannot occur in
  a Rust body, so `docs/ports/macro-body-notes.tsv` records each.
- **`XSETFRAME` stays a macro.**  C writes into its first argument, so a function would take the
  value instead of the caller's slot; the port uses a `macro_rules!` like the other `XSET*`
  setters.
- **`terminal` stays a real pointer.**  `FRAME_KBOARD` returns `*mut c_void` because
  `terminal.kboard` is one (the tag is incomplete in termhooks.h — P-72 rule ③), and so do the
  two `INSERT/DELETE_COST` families' `struct frame *` parameters.
- **A gate defect fixed on the way (P-75).**  `c_body_text` kept the `#ifdef`-dead lines of a
  function, so `SET_FRAME_ICONIFIED` was required to mention `gui_consider_frame_title`,
  `frame_window_p`, `XSETFRAME` and `frame` — text the frozen configuration never compiles.  The
  body comparison now drops dead lines (and `body_is_empty` does the same), which is what the
  frozen configuration's bodies are.
- **Still blocked (47)** - `MOUSE_HL_INFO` left the list in status 3.109, whose
  recipe is the `macro_rules!` in `frame.rs`:  `FRAME_DISPLAY_INFO`, `FRAME_IMAGE_CACHE`, `FRAME_SMALLEST_*`,
  `FRAME_*_PIXEL_*`'s display-dependent forms, `FRAME_MOUSE_UPDATE`, `FACE_FROM_ID`,
  `FACE_FROM_ID_OR_NULL`, `flush_frame` and `SET_FRAME_VISIBLE` need dispextern.h's display
  structures (image cache, face cache, `Mouse_HLInfo`) or the redisplay functions.
- **`MOUSE_HL_INFO` (2026-10-04, status 3.109).**  C 1023-1039 has three arms and the frozen
  configuration (no window system) takes the TTY one, `(&(F)->output_data.tty->display_info->
  mouse_highlight)`; the other two are recorded there (P-08).  The macro lives here because C's
  does, and its body names `tty_output`/`tty_display_info`/`mouse_highlight` from `termchar.rs`,
  which the *including* unit brings in - `frame.rs` cannot include it, because `termchar.rs`
  includes `frame.rs` (C's macro is unhygienic in the same way).  Before this the units that need
  it (`src/window.rs`, `src/dispnew.rs`) declared `MOUSE_HL_INFO` as an `unsafe extern "C"` fn,
  which is why `gate-load` reported `_MOUSE_HL_INFO` as an undefined symbol that C never has:
  the reference objects carry no such symbol at all (clang inlines the macro away).  Measured
  after the change: `gate-load --report` 165 -> 164 undefined symbols and the name is gone;
  `build-tu src/window.c` and `src/dispnew.c` both `built`.

### Integrator addition (2026-10-04, from window.h's geometry family)

- `WINDOW_FRAME` (929) is **named at its use point** in `include/window.rs` for now: window.h's
  `WINDOW_XFRAME` (620) is `XFRAME (WINDOW_FRAME (W))` and C defines `WINDOW_FRAME` here, in
  frame.h, whose mirror has not reached it.  The port keeps C's name and body (a macro, because
  `((w)->frame)` is a *place*) next to `WINDOW_XFRAME` (PORTING.md P-72 rule ③), and this
  header's box stays open until the macro lands in `include/frame.rs` - at which point the
  window.h copy must be deleted (P-26, one landing point), together with the six units that
  used to declare their own (`window.rs`, `dispnew.rs`, `menu.rs`, `xfaces.rs`, `indent.rs`,
  `minibuf.rs`; all deleted with this change except the in-flight `xdisp.rs`/`keyboard.rs`).

## Decisions (third slice: the geometry, the flag writers and the two remaining enums)

- **STATE: 33 of the 47 gap lines closed, and the two shared rows this file needs are listed
  below.**  What landed (all in `femacs/src/include/frame.rs`, appended): `enum internal_border_part`
  is *not* among it - see the ownership bullet - but `enum z_group`'s four members are (C 61-67 is
  dead-arm text, while C 1266's live `#else` arm of `FRAME_Z_GROUP` names `z_group_none`, the
  `sys_types.in.rs` reading), the 18 geometry members (C 1697-1830) as `#[inline(always)] pub fn`s
  over the accessors the second slice already had, `FRAME_Z_GROUP`/`_NONE`/`_ABOVE`/`_BELOW`
  (C 1266-1269, the live `#else` arm), `FRAME_IMAGE_CACHE` (C 922) and `FRAME_SMALLEST_CHAR_WIDTH`/
  `_FONT_HEIGHT` (C 1826-1832) as macros over members no ported type has yet, `WINDOW_FRAME` (C 929)
  as a *function*, `AUTO_FRAME_ARG` (C 1425), `CHECK_FRAME`/`CHECK_LIVE_FRAME` (C 1387-1391, macros
  because every call site writes `NAME!`), `FRAME_MOUSE_UPDATE` (C 1411), `SET_FRAME_GARBAGED`
  (C 1173), `SET_FRAME_VISIBLE` (C 1537-1552, `macro_rules!`) and `flush_frame` (C 1914-1921, the
  same).  The four `missing function` items are three definitions plus one declaration: `SET_FRAME_VISIBLE`
  and `flush_frame` are landed, and `FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL` are the ownership bullet.
- **The four functions' liveness, measured**: `clang -E -std=gnu23 -include
  build/emacs-ref/src/config.h -I build/emacs-ref/src -I emacs/src … emacs/src/frame.h` prints
  `SET_FRAME_VISIBLE`, `FACE_FROM_ID`, `FACE_FROM_ID_OR_NULL` and `flush_frame` once each (all four
  live); the same run's `-dM` macro table has **all 33 macros live**, including `FRAME_IMAGE_CACHE`
  and the `FRAME_Z_GROUP` family whose arms look window-system-shaped.
- **The `visible` token-aliases family needs a spelling in this file's bodies.**
  `docs/ports/token-aliases.tsv` 57 canonicalises frame.h's 1-bit `visible` field with intervals.h's
  (the family `visible,interval_visible_bit`) and its occurrence check asks the port for one
  occurrence per C use.  C's `FRAME_VISIBLE_P` body (`((f)->visible)`) had none, so that one body
  now names the shift it reads (`let visible = FRAME_VISIBLE_SHIFT;`) - the only edit this round
  makes to a line the second slice wrote - and the new `SET_FRAME_VISIBLE` names it twice, as C's
  body uses it twice.
- **The ownership finding: 12 of the remaining `--check` lines name items defined by *units*, not
  by this mirror.**
  `internal_border_part` and its nine `INTERNAL_BORDER_*` members are defined twice already
  (`src/frame.rs` 351-362, whose comment says "the mirror does not carry it", and
  `src/keyboard.rs`), and `FRAME_FACE_CACHE`/`FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL` are defined three
  times (`src/dispnew.rs`, `src/term.rs`, `src/xfaces.rs` - and those three are *typed* ports over
  `dispextern.rs`'s `face_cache`/`face`, better than anything this mirror can spell while its
  `face_cache` field stays `*mut c_void`).  The mirror cannot carry them: a copy is `E0428` in every
  one of those units (measured: `build-tu src/frame.c` reported the ten `E0428`s and
  `build-tu src/term.c` the `FRAME_FACE_CACHE` one before the copies here were dropped), and the
  unit that a copy would fix is `src/keyboard.rs`, which is another lane's **in-flight** file.  So
  the ten and the three are recorded, not taken: the boxes stay unchecked, `--check` keeps reporting
  them, and the ownership move (the mirror owns the header item, the units' copies retire - the
  `stdbit.in.rs` rule of status 3.181) is the integrator's call, item by item.
- **Two shared-table rows are requested** (listed verbatim under `## Evidence`): one
  `docs/ports/excluded-macros.txt` row for `WINDOW_SYSTEM_RETURN` (`syntax`: the qualifier spells
  `_Noreturn ATTRIBUTE_COLD`, and the port writes `-> !` on the declaration it marks - the `AVOID`
  row of line 13 is the same shape) and one `docs/ports/macro-body-notes.tsv` row for
  `FRAME_MOUSE_UPDATE`'s `false` (C's body is a `do { … } while (false)`, whose `while`/`do` the
  tokeniser already skips but whose `false` the port's block has no occasion to spell - the
  `FD_SET`/`FD_CLR` rows of the same table are the precedent).
- **`WINDOW_FRAME` is a function here, not a macro**, and that is a measurement: `window.rs` 406
  already owns the macro spelling (its units call `WINDOW_FRAME! (w)`), and a second
  `macro_rules!` of one name is `E0659`-ambiguous at those call sites (`build-tu src/frame.c` printed
  `WINDOW_FRAME is ambiguous` at `window.rs` 407 and `frame.rs` 2569).  A function and a macro are
  different namespaces, so both spellings coexist; every unit that includes this mirror includes
  `window.rs` too (font/term/cm/ccl/scroll/keymap, measured), so the parameter's type resolves.

## Evidence (third slice)

- **`--check`**: `python3 tools/extract-abi.py --check emacs/src/frame.h
  femacs/src/include/frame.rs` -> `FAIL`, **14** `- ` lines, down from 47: the four functions minus
  the two that are unit-owned, the 33 macros minus the two rows and `FRAME_MOUSE_UPDATE`'s token
  line, and the nine constants.  With the two requested rows added to `/tmp` copies of the tables
  the count is **12**, all of them the unit-owned items above (measured).
- **The two rows requested** (`<TAB>` is a literal tab):
  - `docs/ports/excluded-macros.txt`:
    `WINDOW_SYSTEM_RETURN<TAB>syntax<TAB>frame.h 1471-1475: `#ifdef HAVE_WINDOW_SYSTEM / # define WINDOW_SYSTEM_RETURN / #else / # define WINDOW_SYSTEM_RETURN _Noreturn ATTRIBUTE_COLD / #endif`; this target has no window system, so the live arm is the `_Noreturn ATTRIBUTE_COLD` qualifier, and the port writes it structurally as `-> !` on the one declaration C marks with it (`decode_window_system_frame`, which `src/font.rs` 689 declares) - the `AVOID` row's class (line 13)`
  - `docs/ports/macro-body-notes.tsv`:
    `FRAME_MOUSE_UPDATE<TAB>false<TAB>frame.h 1411-1423: C's body is `do { … } while (false)`, and the port's block (a `macro_rules!` whose body runs once) has no occasion to spell `false`; `do`/`while` are already token-skipped, and the rest of the body (MOUSE_HL_INFO, hlinfo, mouse_face_mouse_frame/_x/_y, block_input, note_mouse_highlight, unblock_input) is spelled`
- **Compile evidence** (the four units that reach this mirror and the three that would have clashed,
  all after the last edit):
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-frame cargo run -q -p xtask -- layout-check` ->
  `layout-check: 90 of 100 mirror(s) compiled, 10 platform-exclusive skipped (…)`, `layout probe ok`,
  `layout-check passed`, exit 0 (`frame.rs` is one of the 100 seeds, so this is real coverage);
  `build-tu src/frame.c` -> `built src/frame.rs -> ../build/femacs/frame.o`; `src/font.c` ->
  `built src/font.rs -> …/font.o`; `src/term.c` -> `built src/term.rs -> …/term.o`; `src/cm.c` ->
  `built src/cm.rs -> …/cm.o`; and the three units the ownership bullet names, `src/keyboard.c` ->
  `built src/keyboard.rs -> …/keyboard.o`, `src/dispnew.c` -> `built src/dispnew.rs -> …/dispnew.o`,
  `src/xfaces.c` -> `built src/xfaces.rs -> …/xfaces.o`.
- **The two ownership measurements**: with the ten enum names and the three face-cache items in this
  mirror, `build-tu src/frame.c` printed ten `error[E0428]` lines (`internal_border_part`,
  `INTERNAL_BORDER_NONE`, … `INTERNAL_BORDER_BOTTOM_LEFT_CORNER`) and one
  `error[E0659]: WINDOW_FRAME is ambiguous`, and `build-tu src/term.c` printed
  `error[E0428]: the name FRAME_FACE_CACHE is defined multiple times`; after dropping them, all
  seven units build and `layout-check` is green.  `femacs/src/frame.rs` is **byte-identical to
  HEAD** at the end of the round (`git diff --stat femacs/src/frame.rs` -> empty), so the ownership
  question is left exactly as it was found.

# ----------------------------------------------------------------------
## Decisions (the ownership ruling and the two shared rows, status 3.211)

- **The mirror now owns `enum internal_border_part` and its nine enumerators.**  The third slice
  left them out because two units defined them (`femacs/src/frame.rs` 351-362,
  `femacs/src/keyboard.rs` 6155-6166, each with a comment saying the mirror did not carry them) and
  a second copy would be `E0428`.  That is the `stdbit.in.rs` relation of 3.181, so the ruling is
  the mirror owns the header item and the units' copies retire - done in the same commit, and the
  type and the nine constants arrive through the mirror every including unit already reads.  The
  checklist above has no row for the six of them: `cinventory` folds an `enum`'s members into the
  type view, while `--check` reports them as constants, so this section is their record.  Measured
  after the move: `build-tu src/frame.c` -> `built src/frame.rs`, `build-tu src/keyboard.c` ->
  `built src/keyboard.rs`, and the three units that read the type indirectly still build
  (`src/term.c`, `src/dispnew.c`, `src/xfaces.c`).
- **The two shared rows the third slice asked for were added** (they were the integrator's):
  `docs/ports/excluded-macros.txt`'s `WINDOW_SYSTEM_RETURN` (kind=syntax, C 1471's
  `_Noreturn ATTRIBUTE_COLD struct frame *`, the `AVOID` precedent - the port writes `-> !`) and
  `docs/ports/macro-body-notes.tsv`'s `FRAME_MOUSE_UPDATE` `false` (C 1411-1423's
  `do { … } while (false)`, the `FD_SET`/`FD_CLR` class).  With those, `--check` is down from 14
  rows to the three below.
- **`FRAME_FACE_CACHE` / `FACE_FROM_ID` / `FACE_FROM_ID_OR_NULL` stay unchecked, and that is the
  ruling, not an omission.**  Each is text C pastes into every unit that includes this header, so
  the port's 1:1 form for an inline definition is one copy per including unit - measured:
  `macro_rules! FRAME_FACE_CACHE` in `src/alloc.rs` 3467, `src/font.rs` 545, `src/xfaces.rs` 445,
  `src/xdisp.rs` 1101, `src/window.rs` 920, with the `fn` forms beside them in `src/term.rs`
  364-377 and the other includers.  A copy in this mirror is `E0659` (macro-name ambiguity) or
  `E0428` in those units - the third-slice measurement stands (10 `E0428` plus
  `E0659: WINDOW_FRAME is ambiguous` with `build-tu src/frame.c`, `E0428` with
  `build-tu src/term.c`).  **The closure precondition** is a shared home for the two types the
  bodies need: `struct face_cache` and `struct face` are `dispextern.h`'s, whose mirror is the
  in-flight A-group unit; once it carries them, `frame.face_cache` here can be typed, the mirror
  can hold the three items with C's own bodies, and the units' copies can retire - the same move
  this commit makes for the enum.
- **A measurement rule this commit earned:** never write the include-directive spelling inside a
  comment in a *unit* file.  `xtask`'s dedupe pass reads that spelling as a directive and replaces
  the whole line, which is what made the first version of the note above break
  `build-tu src/frame.c` (`error: prefix \`mirror\` is unknown` at the generated
  `build/femacs/units/frame/_root.rs` 352).  The comment now names the mirror by path only.
