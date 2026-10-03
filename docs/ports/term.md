# emacs/src/term.c (5479 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 18
- [x] tty-display-color-p  (Ftty_display_color_p, 0/1, doc 2d5b4281f3a6, line 2284)
- [x] tty-display-color-cells  (Ftty_display_color_cells, 0/1, doc 140016d08375, line 2299)
- [x] tty-type  (Ftty_type, 0/1, doc 500b58d8b78b, line 2452)
- [x] controlling-tty-p  (Fcontrolling_tty_p, 0/1, doc 9b2f34f4e1c6, line 2465)
- [x] tty-no-underline  (Ftty_no_underline, 0/1, doc 7eabc4197276, line 2478)
- [x] tty-enable-overline  (Ftty_enable_overline, 1/2, doc 9beafd4cdf8d, line 2496)
- [x] tty-top-frame  (Ftty_top_frame, 0/1, doc 232bef3db730, line 2517)
- [x] suspend-tty  (Fsuspend_tty, 0/1, doc 7109f51b1532, line 2534)
- [x] resume-tty  (Fresume_tty, 0/1, doc 691bc7888baa, line 2603)
- [x] tty--set-output-buffer-size  (Ftty__set_output_buffer_size, 1/2, doc 3db8d2649445, line 2696)
- [x] tty--output-buffer-size  (Ftty__output_buffer_size, 0/1, doc b7633f56435e, line 2718)
- [x] tty-frame-at  (Ftty_frame_at, 2/2, doc 8f510a53a6b2, line 2913)
- [x] tty-frame-geometry  (Ftty_frame_geometry, 0/1, doc 7ac5af92e600, line 5231)
- [x] tty-frame-edges  (Ftty_frame_edges, 0/2, doc cb12a8d313d7, line 5239)
- [x] tty-frame-list-z-order  (Ftty_frame_list_z_order, 0/1, doc 1f78c111c670, line 5249)
- [x] tty-frame-restack  (Ftty_frame_restack, 2/3, doc 6e58a21c9609, line 5260)
- [x] tty-display-pixel-width  (Ftty_display_pixel_width, 0/1, doc 31d2e3844a4e, line 5300)
- [x] tty-display-pixel-height  (Ftty_display_pixel_height, 0/1, doc 8116264bf9b3, line 5311)

## Functions defined — 86
- [x] tty_ring_bell  (line 154, static)
- [x] tty_send_additional_strings  (line 170, static)
- [x] tty_set_terminal_modes  (line 202, static)
- [x] tty_reset_terminal_modes  (line 231, static)
- [x] tty_update_end  (line 254, static)
- [x] tty_set_terminal_window  (line 268, static)
- [x] tty_set_scroll_region  (line 278, static)
- [x] tty_turn_on_insert  (line 300, static)
- [x] tty_turn_off_insert  (line 308)
- [x] tty_turn_off_highlight  (line 318, static)
- [x] tty_turn_on_highlight  (line 326, static)
- [x] tty_toggle_highlight  (line 334, static)
- [x] tty_hide_cursor  (line 346)
- [x] tty_show_cursor  (line 363)
- [x] tty_background_highlight  (line 384, static)
- [x] tty_highlight_if_desired  (line 395, static)
- [x] tty_cursor_to  (line 408, static)
- [x] tty_raw_cursor_to  (line 430, static)
- [x] tty_clear_to_end  (line 449, static)
- [x] tty_clear_frame  (line 472, static)
- [x] tty_clear_end_of_line  (line 494, static)
- [x] encode_terminal_code  (line 543)
- [x] tty_write_glyphs_1  (line 760, static)
- [x] tty_write_glyphs_with_face  (line 823, static)
- [x] tty_insert_glyphs  (line 883, static)
- [x] tty_write_glyphs  (line 973, static)
- [x] tty_delete_glyphs  (line 1009, static)
- [x] tty_ins_del_lines  (line 1042, static)
- [x] string_cost  (line 1110)
- [x] string_cost_one_line  (line 1124, static)
- [x] per_line_cost  (line 1138)
- [x] calculate_ins_del_char_costs  (line 1160, static)
- [x] calculate_costs  (line 1223)
- [x] term_get_fkeys  (line 1403, static)
- [x] term_get_fkeys_1  (line 1419, static)
- [x] append_glyph  (line 1544, static)
- [x] tty_append_glyph  (line 1615)
- [x] produce_glyphs  (line 1640)
- [x] append_composite_glyph  (line 1779, static)
- [x] composite_glyph_is_emoji_sequence  (line 1863, static)
- [x] produce_composite_glyph  (line 1909, static)
- [x] append_glyphless_glyph  (line 1945, static)
- [x] produce_glyphless_glyph  (line 2024, static)
- [x] turn_on_face  (line 2111, static)
- [x] turn_off_face  (line 2203, static)
- [x] tty_capable_p  (line 2247)
- [x] tty_default_color_capabilities  (line 2327, static)
- [x] tty_setup_colors  (line 2353)
- [x] set_tty_color_mode  (line 2410)
- [x] tty_type_name  (line 2445)
- [x] tty_draw_row_with_mouse_face  (line 2746)
- [x] tty_frame_at  (line 2833, static)
- [x] tty_menu_create  (line 3224, static)
- [x] tty_menu_make_room  (line 3233, static)
- [x] tty_menu_search_pane  (line 3253, static)
- [x] tty_menu_calc_size  (line 3273, static)
- [x] mouse_get_xy  (line 3293, static)
- [x] tty_menu_display  (line 3315, static)
- [x] tty_menu_add_pane  (line 3369, static)
- [x] tty_menu_add_selection  (line 3391, static)
- [x] tty_menu_locate  (line 3420, static)
- [x] save_and_enable_current_matrix  (line 3442, static)
- [x] restore_desired_matrix  (line 3472, static)
- [x] free_saved_screen  (line 3505, static)
- [x] screen_update  (line 3525, static)
- [x] read_menu_input  (line 3545, static)
- [x] tty_menu_activate  (line 3617, static)
- [x] tty_menu_destroy  (line 3879, static)
- [x] tty_menu_help_callback  (line 3902, static)
- [x] tty_pop_down_menu  (line 3930, static)
- [x] tty_menu_last_menubar_item  (line 3942, static)
- [x] tty_menu_new_item_coords  (line 3971, static)
- [x] tty_menu_show  (line 4023)
- [x] create_tty_output  (line 4311)
- [x] tty_free_frame_resources  (line 4326, static)
- [x] clear_tty_hooks  (line 4362, static)
- [x] set_tty_hooks  (line 4405, static)
- [x] dissociate_if_controlling_tty  (line 4437, static)
- [x] init_tty  (line 4476)
- [x] vfatal  (line 5037, static)
- [x] maybe_fatal  (line 5052, static)
- [x] fatal  (line 5072)
- [x] delete_tty  (line 5086, static)
- [x] tty_frame_geometry  (line 5149, static)
- [x] tty_display_dimension  (line 5272, static)
- [x] syms_of_term  (line 5323)

## Macros defined — 15
- [x] CONDITIONAL_REASSIGN  (line 1496, body b72bd309bca6)
- [x] MAY_USE_WITH_COLORS_P  (line 2102, body 9899932dfb7b)
- [x] OUTPUT  (line 86, body c2a61fa452cf)
- [x] OUTPUT1  (line 91, body 07b54b7ae0ca)
- [x] OUTPUT1_IF  (line 100, body 7be0f3b5f417)
- [x] OUTPUTL  (line 92, body 982f306a5493)
- [x] OUTPUT_IF  (line 94, body 8b8e4c28f7d8)
- [x] TTYM_FAILURE  (line 3196, body 7984b0a0e139)
- [x] TTYM_IA_SELECT  (line 3199, body 77de68daecd8)
- [x] TTYM_NEXT  (line 3200, body 1b6453892473)
- [x] TTYM_NO_SELECT  (line 3198, body da4b9237bacc)
- [x] TTYM_PREV  (line 3201, body ac3478d69a3c)
- [x] TTYM_SUCCESS  (line 3197, body 356a192b7913)
- [x] TTY_CAPABLE_P_TRY  (line 2250, body 1de75bad6865)
- [x] char_ins_del_cost  (line 1157, body 180df1221cea)

## Types and their fields — 3
- [x] fkey_table: const char* cap, const char* name
- [x] tty_menu_state: struct glyph_matrix* screen_behind, tty_menu* menu, int pane, int x, int y
- [x] tty_pop_down_menu: tty_menu* menu, struct buffer* buffer

## File-scope objects — 16
- [x] struct tty_display_info tty_list  (line 108)
- [x] static int max_frame_cols  (line 132, static)
- [x] static ptrdiff_t encode_terminal_src_size  (line 535, static)
- [x] static ptrdiff_t encode_terminal_dst_size  (line 536, static)
- [x] int char_ins_del_vector  (line 1155)
- [x] static char term_get_fkeys_address  (line 1394, static)
- [x] static KBOARD term_get_fkeys_kboard  (line 1395, static)
- [x] static int default_max_colors  (line 2318, static)
- [x] static int default_no_color_video  (line 2319, static)
- [x] static char default_orig_pair  (line 2320, static)
- [x] static char default_set_foreground  (line 2321, static)
- [x] static char default_set_background  (line 2322, static)
- [x] static const char menu_help_message  (line 3204, static)
- [x] static const char prev_menu_help_message  (line 3204, static)
- [x] static int menu_help_paneno  (line 3207, static)
- [x] static int menu_help_itemno  (line 3207, static)

## Decisions

- **The file is unverified, so both arms are carried.**  `python3 tools/check-cond-eval.py
  src/term.c` reports `59 conditions, 0 unknown, 1 disagreements [NOT VERIFIED]` (C 4443's
  `#ifdef TIOCNOTTY`).  `tools/extract-abi.py` therefore keeps *both* arms of every `#if`
  live and the per-function token check demands the text of the arm the frozen build does not
  compile (docs/status.md 3.49; the `sysdep.rs`/`callproc.rs`/`timefns.rs` precedent).  The
  live arms are ordinary text; the dead ones are carried as `#[cfg (feature = "…")]` text
  (`have_android`, `windowsnt`, `msdos`, `have_gpm`, `tiocnotty`) and no such feature is ever
  set.  Dead **regions that contain whole functions** the ledger does not list (the
  `#ifdef HAVE_GPM` block C 2932-3173, whose functions the extractor drops with the arm) are
  not carried: no live listed body reaches them, so recording them would add text no gate
  reads.  The `#if 0` at C 4198-4211 is dead in every configuration and is kept as a comment.
- **Dead-arm policy applied.**  `#ifdef WINDOWSNT` (C 351-374) carries `w32con_show_cursor`/
  `w32con_hide_cursor`; `#ifdef TERMINFO` (the frozen target *defines* TERMINFO) is live and
  the TERMCAP `#else` arms are carried only where their tokens are not already present;
  `#ifdef MSDOS`/`DOS_NT`/`WINDOWSNT` in `init_tty` and `set_tty_hooks` are carried as
  `init_tty_msdos`/`set_tty_hooks_msdos`/`init_tty_android` text; `#ifdef TIOCNOTTY` is the
  one disagreement and is carried under `tiocnotty`.
- **Spelling deviations (all recorded in the note files).**  C's `switch`/`case`/`default`/
  `break`/`goto`/`done`/`va_list`/`va_start`/`doc:`/`UNINIT` and the `do…while (0)` of
  `OUTPUT_IF`/`OUTPUT1_IF` are `match`, early `return`, `break 'label`, the C-variadic
  `mut args: …`, positional `DEFVAR_*!` arguments and explicit initialisers; each is listed
  in `docs/ports/macro-body-notes.tsv` per function.  `memset`/`write_bytes` is a
  `token-aliases.tsv` family.  The five `tty_bits!`/`it_bits!`/`cm_bits!`/`face_bits!`/
  `glyph_u_cmp_bits!` member-name macros keep C's field names as macro arguments (P-80), so
  every `bool_bf` read/write is still name-checked.  `KEYMAPP!` is the keymap mirror's macro
  and `KVAR` is a token-pasting macro with no Rust counterpart, so the port names the field
  it pastes (`Vinput_decode_map_`) and records `KVAR` in the notes.
- **Use-site declarations (mirror gaps, not mirror edits).**  `struct it`, `struct bidi_it`,
  `struct composition_it`, `struct tty_menu_struct`/`tty_menu`, `struct tty_menu_state`,
  `struct tty_pop_down_menu`, `struct fkey_table`, `enum display_element_type`, the
  `glyphless_display_method`/`mi_result`/`no_color_bit` enumerations, the `TTY_CAP_*` bits,
  `FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL`/`FRAME_FACE_CACHE`, `WINDOW_XFRAME`/
  `WINDOW_LEFT_EDGE_X`/`WINDOW_TOP_EDGE_Y`/`WINDOW_RIGHTMOST_P`/`WINDOW_RIGHT_MARGIN_WIDTH`,
  `SET_FRAME_VISIBLE`/`SET_FRAME_GARBAGED`, the disptab.h glyph-table macros and the
  `struct face` prefix are declared here with clang-measured offsets (sizeof/alignof and
  every named offset asserted).  C's functions whose mirrors stop earlier
  (`decode_live_frame`, `decode_tty_frame`, `decode_*_terminal`, `create_terminal`,
  `delete_terminal`(-`_internal`), `get_named_terminal`, `root_frame`, `child_xy`, `root_xy`,
  `frame_ancestor_p`, `frames_in_reverse_z_order`, `free_frame_faces`,
  `update_frame_with_menu`, `display_tty_menu_item`, `row_hash`, `mouse_position`,
  `read_menu_command`, `show_help_echo`, `discard_mouse_events`, `kbd_buffer_events_waiting`,
  `clear_input_pending`, `lookup_derived_face`, `init_sys_modes`/`reset_sys_modes`,
  `get_tty_size`, `tabs_safe_p`, `init_baud_rate`, `change_frame_size`,
  `add_keyboard_wait_descriptor`/`delete_keyboard_wait_descriptor`, `allocate_kboard`,
  `string_to_multibyte`, `do_line_insertion_deletion_costs`, `quit`, `tty_raise_lower_frame`,
  `tty_defined_color`, `tty_read_avail_input`, `dupstring`, `emacs_fdopen`, `coding_charset_list`,
  `terminal_glyph_code`, `produce_stretch_glyph`, `merge_glyphless_glyph_face`,
  `lookup_glyphless_char_display`, `cursor_to`/`raw_cursor_to`/`clear_to_end`/
  `clear_end_of_line`/`write_glyphs`/`insert_glyphs`/`delete_glyphs`) are declared at their
  use site the same way.
- **The hook fields degrade a pointee.**  `termhooks.rs` cannot name `struct frame`, so its
  hook members are `*mut c_void`; C's `terminal->hook = &tty_*` is therefore a
  function-pointer conversion in the port, spelled once as the `hook!` macro with the field's
  own type (a function pointer's ABI does not depend on its pointee).
- **Two mirror edits (shared; one consumer of each rebuilt).**  `termhooks.rs` gained the
  nine remaining macros (`FRAME_MUST_WRITE_SPACES`, `FRAME_LINE_INS_DEL_OK`,
  `FRAME_CHAR_INS_DEL_OK`, `FRAME_MEMORY_BELOW_FRAME`, `FRAME_TERMINAL_CODING`,
  `FRAME_KEYBOARD_CODING`, `FRAME_RIF`, `cursorX`/`cursorY`) and `FRAME_SCROLL_REGION_COST`
  became an lvalue; four new `macro-body-notes.tsv` rows exempt the packed member token, as
  the pre-existing `FRAME_SCROLL_REGION_OK` row does.  `dispextern.rs`'s
  `SET_GLYPH_FROM_CHAR_GLYPH` needed `($glyph).u.ch as c_int` (the macro assigned an
  `unsigned` to the `int` member `GLYPH.ch`); `composite.rs` was rebuilt as the consumer.
- **`DEFVAR_*` carriers.**  C's storage is `globals.f_<name>`; per docs/status.md 3.29 the
  owning unit defines the ten carriers (`Vsuspend_tty_functions`, `Vresume_tty_functions`,
  `Vauto_composition_emoji_tty_eligible_codepoints`,
  `Vauto_composition_emoji_unqualified_tail_initials`, `system_uses_terminfo`,
  `visible_cursor`, `tty_menu_calls_mouse_position_function`,
  `tty_cursor_movement_use_TAB_BS`, `tty_cursor_movement_use_TAB`,
  `tty_display_emoji_force_wide`), and `check-globals.py` reports 0.
- **Line-count relation.**  5,479 C lines (5,021 live) → 5,606 Rust lines (+2.3 %), inside the
  5 % default tolerance, so no `line-drift-tolerance.tsv` row is needed.  The excess is the
  carried dead arms plus the clang-measured type declarations.
- **D7 call-relationship check.**  `tty_capable_p`/`tty_setup_colors`/`calculate_costs`/
  `string_cost`/`per_line_cost`/`encode_terminal_code`/`create_tty_output`/`init_tty`/
  `syms_of_term`/`fatal` keep their C linkage (`#[unsafe(no_mangle)]`); the four helper
  definitions the extractor calls out (`tty_turn_off_insert`, `tty_hide_cursor`,
  `tty_show_cursor`, `encode_terminal_code`) are exported as C does.  `check-linkage.py`
  reports 0 missing symbols for the unit and `check-undef.py` reports `0 new`.
