# emacs/src/buffer.h (1824 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 112
- [ ] SET_PT  (line 170)
- [ ] TEMP_SET_PT  (line 175)
- [ ] SET_PT_BOTH  (line 180)
- [ ] TEMP_SET_PT_BOTH  (line 185)
- [ ] BUF_TEMP_SET_PT  (line 190)
- [ ] BUFFERP  (line 757)
- [ ] CHECK_BUFFER  (line 763)
- [ ] XBUFFER  (line 769)
- [ ] bset_bidi_paragraph_direction  (line 779)
- [ ] bset_cache_long_scans  (line 784)
- [ ] bset_case_canon_table  (line 789)
- [ ] bset_case_eqv_table  (line 794)
- [ ] bset_directory  (line 799)
- [ ] bset_display_count  (line 804)
- [ ] bset_left_margin_cols  (line 809)
- [ ] bset_right_margin_cols  (line 814)
- [ ] bset_display_time  (line 819)
- [ ] bset_downcase_table  (line 824)
- [ ] bset_enable_multibyte_characters  (line 829)
- [ ] bset_filename  (line 834)
- [ ] bset_keymap  (line 839)
- [ ] bset_last_selected_window  (line 844)
- [ ] bset_local_var_alist  (line 849)
- [ ] bset_mark_active  (line 854)
- [ ] bset_point_before_scroll  (line 859)
- [ ] bset_read_only  (line 864)
- [ ] bset_truncate_lines  (line 869)
- [ ] bset_undo_list  (line 874)
- [ ] bset_upcase_table  (line 879)
- [ ] bset_width_table  (line 884)
- [ ] bset_text_conversion_style  (line 890)
- [ ] BUFFER_CEILING_OF  (line 901)
- [ ] BUFFER_FLOOR_OF  (line 907)
- [ ] BUF_BEGV  (line 917)
- [ ] BUF_BEGV_BYTE  (line 925)
- [ ] BUF_PT  (line 934)
- [ ] BUF_PT_BYTE  (line 942)
- [ ] BUF_ZV  (line 951)
- [ ] BUF_ZV_BYTE  (line 959)
- [ ] BUF_BEG  (line 970)
- [ ] BUF_BEG_BYTE  (line 976)
- [ ] BUF_GPT_ADDR  (line 983)
- [ ] BUF_Z_ADDR  (line 990)
- [ ] BUF_GAP_END_ADDR  (line 997)
- [ ] BUF_COMPUTE_UNCHANGED  (line 1007)
- [ ] SET_BUF_BEGV  (line 1034)
- [ ] SET_BUF_ZV  (line 1041)
- [ ] SET_BUF_BEGV_BOTH  (line 1048)
- [ ] SET_BUF_ZV_BOTH  (line 1055)
- [ ] SET_BUF_PT_BOTH  (line 1062)
- [ ] BYTE_POS_ADDR  (line 1078)
- [ ] CHAR_POS_ADDR  (line 1086)
- [ ] CHAR_TO_BYTE  (line 1096)
- [ ] BYTE_TO_CHAR  (line 1104)
- [ ] PTR_BYTE_POS  (line 1112)
- [ ] BUFFER_PVEC_INIT  (line 1135)
- [ ] BUFFER_LIVE_P  (line 1143)
- [ ] BUFFER_HIDDEN_P  (line 1152)
- [ ] BUFFER_CHECK_INDIRECTION  (line 1160)
- [ ] BUF_TS_LINECOL_BEGV  (line 1177)
- [ ] BUF_TS_LINECOL_POINT  (line 1182)
- [ ] BUF_TS_LINECOL_ZV  (line 1188)
- [ ] SET_BUF_TS_LINECOL_BEGV  (line 1194)
- [ ] SET_BUF_TS_LINECOL_POINT  (line 1200)
- [ ] SET_BUF_TS_LINECOL_ZV  (line 1206)
- [ ] decode_buffer  (line 1275)
- [ ] set_buffer_internal  (line 1290)
- [ ] record_unwind_current_buffer  (line 1300)
- [ ] buffer_intervals  (line 1335)
- [ ] set_buffer_intervals  (line 1344)
- [ ] buffer_has_overlays  (line 1353)
- [ ] FETCH_MULTIBYTE_CHAR  (line 1367)
- [ ] BUF_FETCH_MULTIBYTE_CHAR  (line 1378)
- [ ] FETCH_CHAR_AS_MULTIBYTE  (line 1391)
- [ ] FETCH_CHAR  (line 1402)
- [ ] BUF_BYTE_ADDRESS  (line 1413)
- [ ] BUF_CHAR_ADDRESS  (line 1423)
- [ ] BUF_PTR_BYTE_POS  (line 1433)
- [ ] BUF_FETCH_BYTE  (line 1443)
- [ ] BUF_FETCH_CHAR_AS_MULTIBYTE  (line 1453)
- [ ] buffer_window_count  (line 1465)
- [ ] overlay_start  (line 1476)
- [ ] overlay_end  (line 1484)
- [ ] OVERLAY_START  (line 1495)
- [ ] OVERLAY_END  (line 1503)
- [ ] OVERLAY_PLIST  (line 1511)
- [ ] OVERLAY_BUFFER  (line 1519)
- [ ] OVERLAY_REAR_ADVANCE_P  (line 1527)
- [ ] OVERLAY_FRONT_ADVANCE_P  (line 1535)
- [ ] PER_BUFFER_VALUE_P  (line 1575)
- [ ] SET_PER_BUFFER_VALUE_P  (line 1585)
- [ ] PER_BUFFER_IDX  (line 1612)
- [ ] per_buffer_default  (line 1621)
- [ ] set_per_buffer_default  (line 1627)
- [ ] per_buffer_value  (line 1636)
- [ ] set_per_buffer_value  (line 1642)
- [ ] downcase  (line 1649)
- [ ] upcase  (line 1658)
- [ ] uppercasep  (line 1667)
- [ ] lowercasep  (line 1674)
- [ ] sanitize_tab_width  (line 1682)
- [ ] SANE_TAB_WIDTH  (line 1689)
- [ ] sanitize_char_width  (line 1697)
- [ ] CHARACTER_WIDTH  (line 1708)
- [ ] fetch_char_advance  (line 1723)
- [ ] fetch_char_advance_no_check  (line 1749)
- [ ] buf_next_char_len  (line 1769)
- [ ] next_char_len  (line 1776)
- [ ] buf_prev_char_len  (line 1785)
- [ ] prev_char_len  (line 1794)
- [ ] inc_both  (line 1802)
- [ ] dec_both  (line 1812)

## Macros defined — 53
- [ ] BEGV  (line 41, body c8dfe9c7da08)
- [ ] BEGV_ADDR  (line 75, body eb5b453c5777)
- [ ] BEGV_BYTE  (line 42, body 1a5b72c1341c)
- [ ] BEG_ADDR  (line 72, body 53f96c84615c)
- [ ] BEG_UNCHANGED  (line 156, body 855f47f3017f)
- [ ] BUF_AUTOSAVE_MODIFF  (line 136, body 121f27c20c05)
- [ ] BUF_BEG_ADDR  (line 117, body 9d16478b5cd5)
- [ ] BUF_BEG_UNCHANGED  (line 149, body c8442338d88a)
- [ ] BUF_BYTES_MAX  (line 199, body 03da28938830)
- [ ] BUF_CHARS_MODIFF  (line 126, body 8a5ee5f3800d)
- [ ] BUF_COMPACT  (line 139, body c1849c8c5c5a)
- [ ] BUF_END_UNCHANGED  (line 150, body 41681c343561)
- [ ] BUF_GAP_SIZE  (line 120, body 81659fb12fc0)
- [ ] BUF_GPT  (line 109, body 1424d49a72a6)
- [ ] BUF_GPT_BYTE  (line 110, body fa14f702252e)
- [ ] BUF_MARKERS  (line 142, body 363d46a22f18)
- [ ] BUF_MODIFF  (line 123, body b711c6f1b293)
- [ ] BUF_OVERLAY_MODIFF  (line 132, body 11931c49e4ac)
- [ ] BUF_OVERLAY_UNCHANGED_MODIFIED  (line 147, body bf79d1401038)
- [ ] BUF_SAVE_MODIFF  (line 129, body 081acbf615dc)
- [ ] BUF_UNCHANGED_MODIFIED  (line 144, body b95d7ce5b6bc)
- [ ] BUF_Z  (line 113, body 567174119a24)
- [ ] BUF_Z_BYTE  (line 114, body 0480de55ccde)
- [ ] BVAR  (line 308, body cb903ab34bf2)
- [ ] CHARS_MODIFF  (line 99, body 5559dc5fea88)
- [ ] CHECK_FIXNUM_COERCE_MARKER  (line 1251, body ef5adbbb05af)
- [x] EMACS_BUFFER_H  (line 21, body da39a3ee5e6b)
- [ ] END_UNCHANGED  (line 157, body f51497530ea3)
- [ ] FETCH_BYTE  (line 219, body 64458896d8ca)
- [ ] FOR_EACH_LIVE_BUFFER  (line 1329, body a586c01aa2d9)
- [ ] FOR_EACH_PER_BUFFER_OBJECT_AT  (line 1555, body 06ae1c125bed)
- [ ] GAP_END_ADDR  (line 84, body bc3191687a57)
- [ ] GAP_SIZE  (line 93, body 1add0c898666)
- [ ] GET_OVERLAYS_AT  (line 1311, body ff6daacd77db)
- [ ] GPT  (line 50, body 8a1a3abbf391)
- [ ] GPT_ADDR  (line 81, body 7cb80f245628)
- [ ] GPT_BYTE  (line 51, body 0e22261b2251)
- [ ] MODIFF  (line 96, body b26d13a1e5cf)
- [ ] OVERLAY_MODIFF  (line 102, body c7af12a49405)
- [ ] OVERLAY_UNCHANGED_MODIFIED  (line 154, body f7514b242610)
- [ ] PER_BUFFER_VAR_IDX  (line 1566, body c3b6ab579f67)
- [ ] PER_BUFFER_VAR_OFFSET  (line 1548, body 3af9f4472250)
- [ ] PT  (line 46, body fa8ee60c12f2)
- [ ] PT_ADDR  (line 78, body 25571d393b56)
- [ ] PT_BYTE  (line 47, body 59b494a289bb)
- [ ] SAVE_MODIFF  (line 105, body a964ef8bd0d0)
- [ ] UNCHANGED_MODIFIED  (line 152, body c47ea34a043c)
- [ ] Z  (line 58, body 1d98ebdf63a8)
- [ ] ZV  (line 54, body d9bae3fef90c)
- [ ] ZV_ADDR  (line 87, body c3830b53fe9f)
- [ ] ZV_BYTE  (line 55, body a53a8fcf8ed7)
- [ ] Z_ADDR  (line 90, body 1dc469aff198)
- [ ] Z_BYTE  (line 59, body e56907e9cafd)

## Types and their fields — 4
- [x] buffer: union vectorlike_header header, Lisp_Object name_, Lisp_Object last_name_, Lisp_Object filename_, Lisp_Object directory_, Lisp_Object backed_up_, Lisp_Object save_length_, Lisp_Object auto_save_file_name_, Lisp_Object read_only_, Lisp_Object mark_, Lisp_Object local_var_alist_, Lisp_Object major_mode_, Lisp_Object local_minor_modes_, Lisp_Object mode_name_, Lisp_Object mode_line_format_, Lisp_Object header_line_format_, Lisp_Object tab_line_format_, Lisp_Object keymap_, Lisp_Object abbrev_table_, Lisp_Object syntax_table_, Lisp_Object category_table_, Lisp_Object tab_width_, Lisp_Object fill_column_, Lisp_Object left_margin_, Lisp_Object auto_fill_function_, Lisp_Object downcase_table_, Lisp_Object upcase_table_, Lisp_Object case_canon_table_, Lisp_Object case_eqv_table_, Lisp_Object truncate_lines_, Lisp_Object word_wrap_, Lisp_Object ctl_arrow_, Lisp_Object bidi_display_reordering_, Lisp_Object bidi_paragraph_direction_, Lisp_Object bidi_paragraph_separate_re_, Lisp_Object bidi_paragraph_start_re_, Lisp_Object selective_display_, Lisp_Object selective_display_ellipses_, Lisp_Object overwrite_mode_, Lisp_Object abbrev_mode_, Lisp_Object display_table_, Lisp_Object mark_active_, Lisp_Object enable_multibyte_characters_, Lisp_Object buffer_file_coding_system_, Lisp_Object file_format_, Lisp_Object auto_save_file_format_, Lisp_Object cache_long_scans_, Lisp_Object width_table_, Lisp_Object pt_marker_, Lisp_Object begv_marker_, Lisp_Object zv_marker_, Lisp_Object point_before_scroll_, Lisp_Object file_truename_, Lisp_Object invisibility_spec_, Lisp_Object last_selected_window_, Lisp_Object display_count_, Lisp_Object left_margin_cols_, Lisp_Object right_margin_cols_, Lisp_Object left_fringe_width_, Lisp_Object right_fringe_width_, Lisp_Object fringes_outside_margins_, Lisp_Object scroll_bar_width_, Lisp_Object scroll_bar_height_, Lisp_Object vertical_scroll_bar_type_, Lisp_Object horizontal_scroll_bar_type_, Lisp_Object indicate_empty_lines_, Lisp_Object indicate_buffer_boundaries_, Lisp_Object fringe_indicator_alist_, Lisp_Object fringe_cursor_alist_, Lisp_Object display_time_, Lisp_Object scroll_up_aggressively_, Lisp_Object scroll_down_aggressively_, Lisp_Object cursor_type_, Lisp_Object extra_line_spacing_, Lisp_Object ts_parser_list_, Lisp_Object text_conversion_style_, Lisp_Object cursor_in_non_selected_windows_, struct buffer_text own_text, struct buffer_text* text, ptrdiff_t pt, ptrdiff_t pt_byte, ptrdiff_t begv, ptrdiff_t begv_byte, ptrdiff_t zv, ptrdiff_t zv_byte, struct buffer* base_buffer, int indirections, int window_count, char local_flags, struct timespec modtime, off_t modtime_size, modiff_count auto_save_modified, modiff_count display_error_modiff, time_t auto_save_failure_time, ptrdiff_t last_window_start, struct region_cache* newline_cache, struct region_cache* width_run_cache, struct region_cache* bidi_paragraph_cache, bool_bf prevent_redisplay_optimizations_p : 1, bool_bf clip_changed : 1, bool_bf inhibit_buffer_hooks : 1, bool_bf long_line_optimizations_p : 1, struct itree_tree* overlays, struct ts_linecol ts_linecol_begv, struct ts_linecol ts_linecol_point, struct ts_linecol ts_linecol_zv, Lisp_Object undo_list_
- [x] buffer_text: unsigned char* beg, ptrdiff_t gpt, ptrdiff_t z, ptrdiff_t gpt_byte, ptrdiff_t z_byte, ptrdiff_t gap_size, modiff_count modiff, modiff_count chars_modiff, modiff_count save_modiff, modiff_count overlay_modiff, modiff_count compact, ptrdiff_t beg_unchanged, ptrdiff_t end_unchanged, modiff_count unchanged_modified, modiff_count overlay_unchanged_modified, INTERVAL intervals, struct Lisp_Marker* markers, bool_bf inhibit_shrinking : 1, bool_bf redisplay : 1
- [x] sortvec: Lisp_Object overlay, ptrdiff_t beg, ptrdiff_t end, EMACS_INT priority, EMACS_INT spriority
- [x] ts_linecol: ptrdiff_t bytepos, ptrdiff_t line, ptrdiff_t col

## File-scope objects — 6
- [x] extern struct buffer buffer_defaults  (line 1220)
- [x] extern struct buffer buffer_local_flags  (line 1236)
- [x] extern struct buffer buffer_local_symbols  (line 1242)
- [x] extern Lisp_Object interval_insert_behind_hooks  (line 1246)
- [x] extern Lisp_Object interval_insert_in_front_hooks  (line 1247)
- [x] extern Lisp_Object Vbuffer_alist  (line 1324)

## Decisions (WIP, PORTING.md P-26/P-34)

`emacs/src/buffer.h` @ `fd7afced403`; port `femacs/src/include/buffer.rs`; the reference
file's 1,824 lines.

**Carried in this revision**: the four structs, the constants and the six globals.  The
layout is now proven, not assumed: `sizeof buffer == 984`, `sizeof buffer_text == 144`,
`sizeof sortvec == 40`, `sizeof ts_linecol == 24` and the 99 `buffer.*` / 19
`buffer_text.*` offsets are asserted from the C compiler's numbers (`xtask layout-check`,
200 assertions).  Two field types needed care and are recorded here:

- `char local_flags[MAX_PER_BUFFER_VARS]` (a 50-byte array) is
  `[c_char; MAX_PER_BUFFER_VARS as usize]`; C's enum member is an `int`, so the length
  needs the cast, and the C spelling is kept in the name.
- `struct region_cache *newline_cache`/`width_run_cache`/`bidi_paragraph_cache`: C's
  `struct region_cache` is incomplete in this header (its definition lives in
  region-cache.h, which buffer.h does not include), so the port keeps `*mut c_void` with
  the C spelling in a trailing comment, like `lisp.rs` does for `struct buffer *`
  (PORTING.md 0.1-2).  Naming the real type would force every translation unit that
  includes buffer.rs to include region-cache.rs too.
- `bool_bf` members (`buffer_text`'s two, `struct buffer`'s four) are packed into one
  `bits: u8` each, the width of C's storage unit (`conf_post.h`: `bool_bf` is one byte);
  `sizeof` and the offsets around them are asserted, but the unit's width itself is
  recorded, not proven (P-35).
- The four tree-sitter members (`ts_parser_list_`, `ts_linecol_begv/point/zv`) are behind
  `#[cfg(feature = "tree_sitter")]` (PORTING.md 7.2: the frozen configuration does not
  define `HAVE_TREE_SITTER`, and their layout keys are absent from the baseline for the
  same reason).
- `struct timespec`/`time_t`/`off_t` come from the WIP system mirrors
  `lib/include/time.in.rs` and `lib/include/sys_types.in.rs`, reached the way C's
  `<time.h>`/`<sys/types.h>` are (P-26).
- `modiff_count` and `INTERVAL` (`lisp.h` 4140/1423) were added to `lisp.rs`;
  `INTERVAL` is an opaque pointer because `struct interval` is incomplete there.

**Still to write** (the unchecked boxes): the 53 code macros (`BEGV`, `PT`, `Z`,
`BUF_*`, `BVAR`, …) and the 112 inline accessors (`SET_PT`, `bset_*`, `BUF_*`, …),
including the ones `casetab.c` needs (`BVAR (current_buffer, downcase_table)`,
`bset_downcase_table`).  Until they land the file stays "in progress" and is not
compiled as a translation unit by `gate-fast`; the layout probe does compile it, which is
how the assertions above run.
