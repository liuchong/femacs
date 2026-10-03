# emacs/src/dispnew.c (7584 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 13
- [x] dump-redisplay-history  (Fdump_redisplay_history, 0/0, doc 2427f46058e8, line 214)
- [x] redraw-frame  (Fredraw_frame, 0/1, doc a0d2f8557950, line 3243)
- [x] redraw-display  (Fredraw_display, 0/0, doc 6d7f6b186061, line 3252)
- [x] frame--z-order-lessp  (Fframe__z_order_lessp, 2/2, doc 76df1d5f0d5e, line 3441)
- [x] display--update-for-mouse-movement  (Fdisplay__update_for_mouse_movement, 3/3, doc 87bbdaebdc9b, line 4228)
- [x] open-termscript  (Fopen_termscript, 1/1, doc 6662ee81df06, line 6775)
- [x] send-string-to-terminal  (Fsend_string_to_terminal, 1/2, doc 724b1bc8c622, line 6807)
- [x] ding  (Fding, 0/1, doc 4aee4a11ad26, line 6854)
- [x] sleep-for  (Fsleep_for, 1/2, doc f857420921f8, line 6894)
- [x] redisplay  (Fredisplay, 0/1, doc 6fae1ce5d99a, line 7020)
- [x] frame-or-buffer-changed-p  (Fframe_or_buffer_changed_p, 0/1, doc b02723d179d4, line 7048)
- [x] internal-show-cursor  (Finternal_show_cursor, 2/2, doc 7cea39c65552, line 7428)
- [x] internal-show-cursor-p  (Finternal_show_cursor_p, 0/1, doc 6d842a4adec6, line 7444)

## Functions defined — 150
- [x] add_window_display_history  (line 171, static)
- [x] add_frame_display_history  (line 198, static)
- [x] __executable_start  (line 239)
- [x] new_glyph_matrix  (line 256, static)
- [x] free_glyph_matrix  (line 284, static)
- [x] margin_glyphs_to_reserve  (line 315, static)
- [x] verify_row_hash  (line 333, static)
- [x] adjust_glyph_matrix  (line 361, static)
- [x] reverse_rows  (line 649, static)
- [x] rotate_matrix  (line 678)
- [x] increment_matrix_positions  (line 703)
- [x] clear_glyph_matrix_rows  (line 720)
- [x] clear_glyph_matrix  (line 743)
- [x] shift_glyph_matrix  (line 758)
- [x] clear_current_matrices  (line 791)
- [x] clear_desired_matrices  (line 826)
- [x] clear_window_matrices  (line 856, static)
- [x] clear_glyph_row  (line 889)
- [x] blank_row  (line 902)
- [x] increment_row_positions  (line 931, static)
- [x] swap_glyph_pointers  (line 1009, static)
- [x] copy_row_except_pointers  (line 1039, static)
- [x] assign_row  (line 1053, static)
- [x] glyph_row_slice_p  (line 1069, static)
- [x] prepare_desired_row  (line 1114)
- [x] line_hash_code  (line 1171, static)
- [x] line_draw_cost  (line 1213, static)
- [x] row_equal_p  (line 1272, static)
- [x] new_glyph_pool  (line 1348, static)
- [x] free_glyph_pool  (line 1369, static)
- [x] realloc_glyph_pool  (line 1394, static)
- [x] flush_stdout  (line 1442)
- [x] check_matrix_pointer_lossage  (line 1454)
- [x] matrix_row  (line 1469)
- [x] allocate_matrices_for_frame_redisplay  (line 1645, static)
- [x] required_matrix_height  (line 1758, static)
- [x] required_matrix_width  (line 1786, static)
- [x] allocate_matrices_for_window_redisplay  (line 1814, static)
- [x] adjust_frame_glyphs  (line 1850)
- [x] showing_window_margins_p  (line 1884, static)
- [x] fake_current_matrices  (line 1906, static)
- [x] save_current_matrix  (line 1960, static)
- [x] restore_current_matrix  (line 2004, static)
- [x] adjust_frame_glyphs_for_frame_redisplay  (line 2049, static)
- [x] adjust_frame_glyphs_for_window_redisplay  (line 2177, static)
- [x] adjust_decode_mode_spec_buffer  (line 2311, static)
- [x] free_glyphs  (line 2332)
- [x] free_window_matrices  (line 2409)
- [x] check_glyph_memory  (line 2437)
- [x] build_frame_matrix  (line 2521, static)
- [x] build_frame_matrix_from_window_tree  (line 2543, static)
- [x] build_frame_matrix_from_leaf_window  (line 2569, static)
- [x] spec_glyph_lookup_face  (line 2707)
- [x] fill_up_glyph_row_with_spaces  (line 2735, static)
- [x] fill_up_glyph_row_area_with_spaces  (line 2747, static)
- [x] fill_up_frame_row_with_spaces  (line 2769)
- [x] make_current  (line 2798, static)
- [x] mirror_make_current  (line 2839, static)
- [x] mirrored_line_dance  (line 2893)
- [x] sync_window_with_frame_matrix_rows  (line 2938, static)
- [x] frame_row_to_window  (line 2975, static)
- [x] mirror_line_dance  (line 3006, static)
- [x] check_window_matrix_pointers  (line 3119, static)
- [x] check_matrix_pointers  (line 3149, static)
- [x] window_to_frame_vpos  (line 3182, static)
- [x] window_to_frame_hpos  (line 3197, static)
- [x] redraw_frame  (line 3212)
- [x] rect_intersect  (line 3305, static)
- [x] root_xy  (line 3323)
- [x] child_xy  (line 3337)
- [x] frame_rect_abs  (line 3349, static)
- [x] max_child_z_order  (line 3359)
- [x] frame_ancestors_visible_p  (line 3375, static)
- [x] frames_with_root  (line 3394, static)
- [x] frames_with_parent  (line 3413, static)
- [x] frame_z_order_cmp  (line 3430, static)
- [x] frames_in_reverse_z_order  (line 3454)
- [x] tty_raise_lower_frame  (line 3468)
- [x] is_tty_frame  (line 3490)
- [x] is_tty_child_frame  (line 3498)
- [x] is_tty_root_frame  (line 3506)
- [x] is_tty_root_frame_with_visible_child  (line 3515)
- [x] first_enabled_row  (line 3527, static)
- [x] make_matrix_current  (line 3539, static)
- [x] prepare_desired_root_row  (line 3555, static)
- [x] make_glyph_space  (line 3591, static)
- [x] neutralize_wide_char  (line 3603, static)
- [x] box_from_display_table  (line 3630, static)
- [x] box_default  (line 3651, static)
- [x] box_glyph  (line 3684, static)
- [x] produce_box_glyphs  (line 3697, static)
- [x] produce_box_sides  (line 3725, static)
- [x] produce_box_line  (line 3742, static)
- [x] copy_child_glyphs  (line 3759, static)
- [x] update_menu_bar  (line 3837, static)
- [x] update_bar_window  (line 3847, static)
- [x] update_tab_bar  (line 3868, static)
- [x] update_tool_bar  (line 3877, static)
- [x] update_window_frame  (line 3886, static)
- [x] update_initial_frame  (line 3900, static)
- [x] flush_terminal  (line 3908, static)
- [x] update_tty_frame  (line 3916, static)
- [x] abs_cursor_pos  (line 3930, static)
- [x] is_in_matrix  (line 3955, static)
- [x] frame_selected_window_frame  (line 3968, static)
- [x] is_cursor_obscured  (line 3982, static)
- [x] terminal_cursor_magic  (line 4014, static)
- [x] combine_updates_for_frame  (line 4055)
- [x] combine_updates_for_frame  (line 4120)
- [x] combine_updates  (line 4129)
- [x] update_frame  (line 4143)
- [x] update_frame_with_menu  (line 4164)
- [x] update_mouse_position  (line 4205)
- [x] update_window_tree  (line 4255, static)
- [x] update_single_window  (line 4272)
- [x] redraw_overlapped_rows  (line 4294, static)
- [x] redraw_overlapping_rows  (line 4340, static)
- [x] check_current_matrix_flags  (line 4402, static)
- [x] update_window  (line 4423, static)
- [x] gui_update_window_begin  (line 4635)
- [x] gui_update_window_end  (line 4670)
- [x] update_marginal_area  (line 4718, static)
- [x] update_text_area  (line 4739, static)
- [x] update_window_line  (line 5000, static)
- [x] set_window_cursor_after_update  (line 5079, static)
- [x] set_window_update_flags  (line 5151, static)
- [x] add_row_entry  (line 5226, static)
- [x] scrolling_window  (line 5277, static)
- [x] tty_set_cursor  (line 5672, static)
- [x] write_matrix  (line 5759, static)
- [x] scrolling  (line 5789, static)
- [x] count_blanks  (line 5893, static)
- [x] count_match  (line 5910, static)
- [x] write_row  (line 5934, static)
- [x] buffer_posn_from_coords  (line 6260)
- [x] mode_line_string  (line 6443)
- [x] marginal_area_string  (line 6518)
- [x] handle_window_change_signal  (line 6611, static)
- [x] deliver_window_change_signal  (line 6652, static)
- [x] do_pending_window_change  (line 6664)
- [x] change_frame_size_1  (line 6693, static)
- [x] change_frame_size  (line 6742)
- [x] frame_size_change_delayed  (line 6766)
- [x] bitch_at_user  (line 6874)
- [x] sit_for  (line 6947)
- [x] init_faces_initial  (line 7177, static)
- [x] init_display_interactive  (line 7193, static)
- [x] init_display  (line 7412)
- [x] syms_of_display  (line 7460)
- [x] syms_of_display_for_pdumper  (line 7581, static)

## Macros defined — 4
- [x] CHANGED_LEAF_MATRIX  (line 1642, body 8ef58e36184c)
- [x] NEW_LEAF_MATRIX  (line 1637, body ee799cd75778)
- [x] REDISPLAY_HISTORY_SIZE  (line 151, body 22d200f8670d)
- [x] char_ins_del_cost  (line 5927, body 311dd53d586a)

## Types and their fields — 4
- [x] dim: int width, int height
- [x] rect: int x, int y, int w, int h
- [x] redisplay_history: char trace
- [x] row_entry: int old_uses, int new_uses, int new_line_number, ptrdiff_t bucket, struct glyph_row* row, struct row_entry* next

## File-scope objects — 20
- [x] static bool delayed_size_change  (line 123, static)
- [x] struct glyph space_glyph  (line 127)
- [x] static int glyph_matrix_count  (line 134, static)
- [x] static int glyph_pool_count  (line 135, static)
- [x] static struct redisplay_history redisplay_history  (line 155, static)
- [x] static int history_idx  (line 159, static)
- [x] static uintmax_t history_tick  (line 164, static)
- [x] static struct row_entry row_entry_pool  (line 5194, static)
- [x] static ptrdiff_t row_entry_pool_size  (line 5195, static)
- [x] static ptrdiff_t row_entry_idx  (line 5199, static)
- [x] static struct row_entry row_table  (line 5205, static)
- [x] static ptrdiff_t row_table_size  (line 5206, static)
- [x] static struct row_entry old_lines  (line 5211, static)
- [x] static struct row_entry new_lines  (line 5211, static)
- [x] static ptrdiff_t old_lines_size  (line 5212, static)
- [x] static ptrdiff_t new_lines_size  (line 5212, static)
- [x] static struct run run_pool  (line 5216, static)
- [x] static ptrdiff_t runs_size  (line 5217, static)
- [x] static struct run runs  (line 5221, static)
- [x] static Lisp_Object frame_and_buffer_state  (line 7045, static)

# ----------------------------------------------------------------------
## Decisions

`emacs/src/dispnew.c` @ `fd7afced403`; port `femacs/src/dispnew.rs` (8,565 lines vs C's 7,584
- +12.9% on the **total** basis, declared in `docs/ports/line-drift-tolerance.tsv` with the
reason: the both-arms-carry policy below, the use-site records and the docstring literals).

- **Subrs 13/13, functions 150/150, macros 4/4, types 4/4, file-scope objects 20/20** — all
boxes above are ticked, and `python3 tools/extract-abi.py --check emacs/src/dispnew.c
femacs/src/dispnew.rs` prints `OK`.

- **`src/dispnew.c` is not in `cond-eval-verified.txt`, so both arms of every `#if` are
  carried** (P-51/status 3.49; `python3 tools/check-cond-eval.py src/dispnew.c` prints
  `84 conditions, 0 unknown, 0 disagreements [VERIFIED]`, but the list is the gate's input and
  this unit does not own it).  The dead arms stay as text under a `#[cfg (feature = "…")]`
  naming the non-frozen side: `glyph_debug` (the redisplay-history ring buffer,
  `add_window_display_history`/`add_frame_display_history`/`Fdump_redisplay_history`,
  `glyph_row_slice_p`, `flush_stdout`, `check_matrix_pointer_lossage`, `matrix_row`,
  `check_window_matrix_pointers`, `check_matrix_pointers`, `check_current_matrix_flags`,
  the two `(void) glyph->method` `strcpy`s), `glyph_debug` + `enable_checking` (the two
  allocation counters), `profiling` (`__executable_start`), `window_system` /
  `window_system_no_ext_menu_bar` / `window_system_no_ext_tool_bar` (the menu/tab/tool bar
  matrices in `adjust_frame_glyphs_for_window_redisplay`, `clear_current_matrices`,
  `clear_desired_matrices`, `free_glyphs`, `update_menu_bar`/`update_bar_window`/
  `update_tab_bar`/`update_tool_bar`, `redraw_overlapped_rows`/`redraw_overlapping_rows`,
  `gui_update_window_begin`/`gui_update_window_end`, the frame/face arms of
  `required_matrix_height`/`required_matrix_width`, the `it.what == IT_IMAGE` arms),
  `ntgui` (`w32_use_visible_system_caret`), `android` (the empty
  `combine_updates_for_frame`), `x_windows`/`use_ncurses`/`android`/`ns`/`pgtk`/`haiku`/
  `windowsnt`/`msdos` (the window-system probes and the two platform arms of
  `init_display_interactive`).  `#if 0` regions (`check_rows`, `find_glyph_row_slice`,
  `check_matrix_invariants`, `swap_glyphs_in_rows`, `IF_DEBUG (check_matrix_invariants (w))`)
  are dead in C itself and are recorded here, not carried.

- **Carried text that is *live* on the frozen target**: `#ifndef HAVE_ANDROID` (C 1165-3827,
  3921-4843 and 5925-5926: `line_hash_code`, `line_draw_cost`, the tty child-frame section,
  `abs_cursor_pos` … `combine_updates_for_frame`, `scrolling`, `char_ins_del_cost`),
  `#ifdef SIGWINCH` (C 6606-6657), `#ifndef WINDOWSNT` (`init_display_interactive`'s daemon
  return), `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` (`gobble_input`),
  `#ifndef HAVE_ANDROID` around the two `char_ins_del_cost` comparisons in `write_row`, and
  `#ifdef MSDOS`'s `#else` arms.

- **`defsubr` liveness is decided by the frozen macro table, not by the blindness rule**:
  C 7487-7489's `defsubr (&Sdump_redisplay_history)` sits in `#ifdef GLYPH_DEBUG`, and
  `tools/extract-abi.py`'s `c_defsubr_live` reads `GLYPH_DEBUG` as undefined, so the call is
  **not** transcribed (it would be reported as an unexpected `defsubr`) and the symbol name is
  the one body token the port cannot spell (`docs/ports/macro-body-notes.tsv`).  The record
  `Sdump_redisplay_history` and the `DEFUN!` exist as text under `#[cfg (feature =
  "glyph_debug")]`; `Fdump_redisplay_history` therefore cannot be reached from Lisp until
  `GLYPH_DEBUG` is a frozen-defined macro for this file (`docs/ports/font.md`'s `FONT_DEBUG`
  record is the same class of table gap).

- **Use-site declarations (mirror gaps, PORTING.md P-72 rule 3)**.  `dispnew.h` has no mirror
  at all, and the headers this file includes carry only part of what it reaches:
  * records with clang-measured offsets and `const _` assertions: `dim`, `rect`, `row_entry`,
    `redisplay_history`, `redisplay_interface` (prefix + the nine members this unit calls),
    `face`/`face_cache` (prefixes + `background`/`faces_by_id`/`used`), `it`/`composition_it`/
    `bidi_it` (pads + the members `buffer_posn_from_coords` reads);
  * the geometry-macro prelude (`WINDOW_FRAME`/`WINDOW_XFRAME`/`WINDOW_BOTTOMMOST_P`/
    `WINDOW_BOTTOM_DIVIDER_WIDTH`/`WINDOW_RIGHT_DIVIDER_WIDTH`/`WINDOW_CONFIG_SCROLL_BAR_HEIGHT`/
    `WINDOW_SCROLL_BAR_AREA_HEIGHT`/`WINDOW_MODE_LINE_HEIGHT`/`WINDOW_HEADER_LINE_HEIGHT`/
    `WINDOW_TAB_LINE_HEIGHT`/`WINDOW_BOX_HEIGHT_NO_MODE_LINE`/`WINDOW_LEFT_FRINGE_WIDTH`/
    `WINDOW_RIGHT_FRINGE_WIDTH`/`WINDOW_FRINGES_WIDTH`/`WINDOW_TOP_EDGE_Y`/
    `WINDOW_FRAME_COLUMN_WIDTH`/`WINDOW_RIGHTMOST_P`, `MATRIX_*_LINE_HEIGHT`,
    `CURRENT_*_LINE_HEIGHT`, `CHARPOS`/`BYTEPOS`/`CHAR_GLYPH_SPACE_P`, `SET_GLYPH_FROM_CHAR`/
    `SET_GLYPH_FROM_GLYPH_CODE`, the glyph-table family, `FRAME_MESSAGE_BUF_SIZE`'s
    neighbours, `FRAME_PIXEL_TO_TEXT_WIDTH`/`_HEIGHT`, `SET_FRAME_GARBAGED`, `CHECK_FRAME`,
    `AUTO_FRAME_ARG`, `clip_to_bounds`'s callers, `static_assert`'s two sites);
  * the flag dispatchers `window_bits!` (`update_mode_line`, `window_end_valid`,
    `must_be_updated_p`, `pseudo_window_p`, `cursor_off_p`, `phys_cursor_on_p`), the frame-flag
    accessors (`fonts_changed`, `glyphs_initialized_p`, `after_make_frame`, `garbaged`,
    `new_size_p`, `output_method`), `term_initted!`, and the `xwidget_end_redisplay!` no-op
    (xwidget.h 26/238: with `HAVE_XWIDGETS` undef C's call *is* the empty `INLINE` body, which
    is why the reference object has no such reference);
  * the extern declarations the included mirrors do not carry: `row_hash`,
    `window_wants_{mode_line,header_line,tab_line}`, `window_box`, `window_box_width`,
    `window_box_height`, `window_text_bottom_y`, `estimate_mode_line_height`, `merge_faces`,
    `lookup_basic_face`, `update_begin`/`update_end`/`clear_frame`, `cursor_to`,
    `tty_hide_cursor`/`tty_show_cursor`, `tty_set_cursor`'s callees,
    `decode_live_frame`/`frame_redisplay_p`/`root_frame`/`frame_ancestor_p`/`next_almost_prime`,
    `calculate_costs`, `write_glyphs`/`insert_glyphs`/`delete_glyphs`/`clear_end_of_line`,
    `scrolling_max_lines_saved`/`scrolling_1`, `note_mouse_highlight`/`gen_help_event` and the
    five `help_echo_*` globals, `detect_input_pending(_run_timers)`, `gobble_input`/
    `swallow_events`/`ring_bell`/`request_sigio`/`unrequest_sigio`,
    `wait_reading_process_output`, `redisplay_preserve_echo_area`, `adjust_frame_size`,
    `frame_size_history_extra`, `get_tty_size`, `emacs_fopen`, `decode_live_terminal`,
    `decode_any_window`, `create_tty_output`, `init_foreground_group`, `init_tty`,
    `will_dump_p`, `x_display_ok`, `fatal`, `exit`, `putchar`, `IMAGE_OPT_FROM_ID`,
    `bidi_shelve_cache`/`bidi_unshelve_cache`/`start_display`/`move_it_to`/
    `move_it_in_display_line`, `automatic_hscrolling`.
  * Two mirror defects were **shadowed** by corrected copies of C's text while this unit was
    ported (Rust's textual `macro_rules!` scoping makes the later definition win from that
    point on): `SET_CHAR_GLYPH`/`SET_CHAR_GLYPH_FROM_GLYPH` (the mirror wrote a `c_int` into the
    `c_uint` union member `glyph_u::ch`, `E0308`) and `MATRIX_ROW_VPOS` (it dereferenced the
    row pointer before the difference, `E0599`).  **Both mirrors are fixed now and the two
    shadowing copies are gone** (status 3.84): the first spells C's implicit conversion
    (`($code) as c_uint`) and the second is a pointer difference, as C's is.

- **`DEFVAR_*` carriers this file owns** are defined at the crate root with
  `#[unsafe(no_mangle)]` (the term.rs/casetab.rs shape, status 3.29): `baud_rate`,
  `inverse_video`, `visible_bell`, `no_redraw_on_reenter`, `Vinitial_window_system`,
  `cursor_in_echo_area`, `mouse_prefer_closest_glyph`, `Vglyph_table`,
  `Vstandard_display_table`, `Vx_show_tooltip_timeout`, `Vtab_bar_position`; `frame_and_buffer_state`
  stays C's internal-linkage `static`.  `DEFVAR_KBOARD ("window-system", …)` registers
  `std::mem::offset_of! (kboard, Vwindow_system_)`, the shape `frame.rs`/`macros.rs` use for
  their own kboard variables (C's macro is `offsetof (struct kboard, vname##_)`).

- **Deviations recorded**: (a) the `defsubr` above; (b) C's chained assignments become one
  statement per assignment in C's right-to-left order; (c) C's `0`/`1`/`NULL` for `bool` and
  pointer parameters are `false`/`true`/`null_mut ()` (the `0`/`1` tokens are exempted per
  function in `docs/ports/macro-body-notes.tsv`, with the P-78 residual that a `false`↔`true`
  swap in those bodies is invisible to the token check); (d) `goto` becomes `break` out of a
  labelled block (the bytecode.rs shape), `switch` becomes `match`; (e) `struct` locals C
  declares uninitialised are zero-initialised (`it`, `width`/`height`, `sec`, `dline`, `rect`
  `r`, `dir`-class locals), and struct copies use the derived `Copy` of the use-site records;
  (f) `memclear`/`memcpy`/`memmove`/`memset` are the mirror's/lisp.rs's helpers and
  `std::ptr::copy*`/`write_bytes` (the token-alias families); (g) the added `#[derive(Clone,
  Copy)]` on the use-site records (C structs are trivially copyable; needed for C's struct
  assignment and for `md5`-class returns by value).

### What no gate can see

(a) **`defsubr` liveness for `Sdump_redisplay_history`** (above): the ledger is green, but Lisp
cannot reach `Fdump_redisplay_history` until the macro table knows `GLYPH_DEBUG` is defined for
this file.  (b) Every `#[cfg (feature = …)]` arm in this unit is *never compiled* by any of the
gates (`xtask dead-arms` covers four other units and cannot flip these features, status 3.49),
so their text is name-resolved by nothing; several arms therefore reference names the mirrors
do not carry (`Mouse_HLInfo`, `gui_draw_right_divider`, `display_and_set_cursor`,
`w32_use_visible_system_caret`, `image`/`xwidget`, `Vtab_bar_position`'s GUI heights, the
`HAVE_MSDOS` output slots).  (c) The `.unwrap ()` calls on `redisplay_interface`'s hooks and on
the two terminal hooks mirror C's "this node is dereferenced" reading; if a hook were NULL, C
would crash and the port aborts — a human argument, not a gate's.  (d) The
`macro-body-notes.tsv` rows for the `0`/`1` tokens leave the P-78 residual (a `false`↔`true`
swap is invisible); the notes for `goto`/`switch`/`static_assert` are the standard
structural-keyword class.  (e) `docs/baseline/undef-pending.txt` needs **one** new entry,
`_automatic_hscrolling` (xdisp.c's `auto-hscroll-mode` carrier, referenced by
`buffer_posn_from_coords`): `python3 tools/check-undef.py` reports it as the unit's only new
orphan, and refreshing the ratchet is a `docs/baseline` write outside this lane's authority
(see the integrator list).  (f) `build/emacs-ref/src/dispnew.o` was built so that the
undefined-reference gate's domain (and the symbol gate's reference) covers this unit; it is not
part of the frozen baseline the repository ships.

### Integrator to-do list (this unit's mirror gaps and the one blocked block)

1. `docs/baseline/undef-pending.txt`: add `_automatic_hscrolling` (and re-take the ratchet —
   the gate reports 17 stale entries this round, mostly fileio's and this unit's `_Vglyph_table`).
2. ~~`dispextern.rs`: fix `SET_CHAR_GLYPH`/`SET_CHAR_GLYPH_FROM_GLYPH` and `MATRIX_ROW_VPOS`,
   deleting this unit's shadowing copies in the same commit.~~ **Done in status 3.84.**
3. `window.rs`: the missing accessors this lane declares (`WINDOW_RIGHTMOST_P`,
   `WINDOW_FRAME`/`WINDOW_XFRAME`, the `window_wants_*` entries, the `WINDOW_*_LINE_HEIGHT`
   family, `WINDOW_FRAME_COLUMN_WIDTH`, the `window_bits!` dispatcher, `window_box`,
   `window_box_width`/`_height`, `window_text_bottom_y`, `decode_any_window`) — and
   `frame.rs`: the `FRAME_*` flag accessors, `FRAME_PIXEL_TO_TEXT_WIDTH`/`_HEIGHT`,
   `SET_FRAME_GARBAGED`, `CHECK_FRAME`, `AUTO_FRAME_ARG`, `FRAME_FOREGROUND_PIXEL`/
   `FRAME_BACKGROUND_PIXEL`, `FRAME_MESSAGE_BUF_SIZE`, `root_frame`/`frame_ancestor_p`/
   `decode_live_frame`/`frame_redisplay_p`/`adjust_frame_size`/`frame_size_history_extra`.
4. `dispextern.h`'s floor of records (`struct it` and friends, `struct redisplay_interface`,
   `struct face`/`face_cache`, `xwidget.h`) and `dispextern.rs`'s `FACE_FROM_ID` family: this
   file declares prefixes; when the mirrors land the duplicates must be removed in the same
   commit (the mirror-dupes gate will then catch them).
5. `GLYPH_DEBUG` as a frozen-defined macro for this file would make C 7487-7489 reachable
   (then `syms_of_display` must transcribe the `defsubr`).

### D7 call relationships

`nm -u build/emacs-ref/src/dispnew.o` (158 names) vs `nm -u build/femacs/dispnew.o` (186):
**152 shared, 6 C-only, 34 port-only**.  C-only: `___chkstk_darwin`/`___stack_chk_fail`/
`___stack_chk_guard` (clang codegen), `_globals` (C's single `globals` object, which this unit
reads for every `DEFVAR_*` variable where the port has a per-variable carrier, P-21),
`_current_thread` and `_putc_unlocked` (C's `<stdio.h>` expands `putchar (07)` to
`putc_unlocked (07, stdout)`; the port calls the mirror's `putchar`).  Port-only: the seven
`__RNv*` core-panic/monomorphization symbols, the mirror helpers `--crate-type=lib` emits
(`EQ`/`NILP`-class `cons_to_signed`/`cons_to_unsigned`, `specpdl`, `current_buffer`,
`sa_stack_alloc`, `setjmp`, `memset`, `memory_full_up`, `fileno`, `putchar`), and the carrier
symbols of units that are not ported yet (`_automatic_hscrolling`, `_Vexecuting_kbd_macro`,
`_Vchar_width_table`, `_mode_line_in_non_selected_windows`, `_frame_size_history`,
`_symbols_with_pos_enabled`, `_noninteractive1`).  No callee of C's source text is missing
from the port's set and no callee is renamed.
