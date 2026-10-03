# emacs/src/termchar.h (257 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] CURTTY  (line 249, body e52f55948d8c)
- [x] EMACS_TERMCHAR_H  (line 20, body da39a3ee5e6b)
- [x] FRAME_TTY  (line 243, body 7ba0901323ca)

## Types and their fields — 2
- [x] tty_display_info: struct tty_display_info* next, char* name, char* type, FILE* input, FILE* output, ptrdiff_t output_buffer_size, FILE* termscript, struct emacs_tty* old_tty, bool_bf term_initted : 1, int reference_count, struct terminal* terminal, struct cm* Wcm, Lisp_Object top_frame, struct frame* previous_frame, int previous_color_mode, Mouse_HLInfo mouse_highlight, char termcap_term_buffer, char termcap_strings_buffer, const char* TS_ins_line, const char* TS_ins_multi_lines, const char* TS_bell, const char* TS_clr_to_bottom, const char* TS_clr_line, const char* TS_clr_frame, const char* TS_set_scroll_region, const char* TS_set_scroll_region_1, const char* TS_del_char, const char* TS_del_multi_chars, const char* TS_del_line, const char* TS_del_multi_lines, const char* TS_delete_mode, const char* TS_end_delete_mode, const char* TS_end_insert_mode, const char* TS_ins_char, const char* TS_ins_multi_chars, const char* TS_insert_mode, const char* TS_pad_inserted_char, const char* TS_end_keypad_mode, const char* TS_keypad_mode, const char* TS_pad_char, const char* TS_repeat, const char* TS_end_standout_mode, const char* TS_fwd_scroll, const char* TS_standout_mode, const char* TS_rev_scroll, const char* TS_end_termcap_modes, const char* TS_termcap_modes, const char* TS_visible_bell, const char* TS_cursor_normal, const char* TS_cursor_visible, const char* TS_cursor_invisible, const char* TS_set_window, const char* TS_enter_bold_mode, const char* TS_enter_italic_mode, const char* TS_enter_dim_mode, const char* TS_enter_reverse_mode, const char* TS_exit_underline_mode, const char* TS_enter_underline_mode, const char* TS_enter_strike_through_mode, const char* TS_enter_overline_mode, const char* TS_enter_alt_charset_mode, const char* TS_exit_alt_charset_mode, const char* TS_exit_attribute_mode, int TN_no_color_video, int TN_max_colors, const char* TS_orig_pair, const char* TS_set_foreground, const char* TS_set_background, int TF_rgb_separate, int TF_hazeltine, int TF_insmode_motion, int TF_standout_motion, int TF_underscore, int TF_teleray, const char* TF_set_underline_style, const char* TF_set_underline_color, int RPov, bool_bf delete_in_insert_mode : 1, bool_bf costs_set : 1, bool_bf insert_mode : 1, bool_bf standout_mode : 1, int meta_key, int specified_window, bool_bf cursor_hidden : 1, bool_bf flow_control : 1, bool_bf showing_menu : 1, bool_bf must_write_spaces : 1, bool_bf line_ins_del_ok : 1, bool_bf char_ins_del_ok : 1, bool_bf scroll_region_ok : 1, bool_bf memory_below_frame : 1, int scroll_region_cost
- [x] tty_output: struct tty_display_info* display_info

## File-scope objects — 1
- [x] extern struct tty_display_info tty_list  (line 240)

## Decisions

- **`bool_bf` runs are `bits_*` bytes with `TTY_*` masks.**  `bool_bf` is `bool` on this target
  (conf_post.h 59-61), one byte wide, so each run of consecutive `bool_bf : 1` members occupies
  one byte: `term_initted` alone (62), the four mode flags 174-179 (`bits_modes`), and the eight
  capability flags 196-213 (`bits_caps`).  This is `itree.h`'s `itree_node.bits` shape and
  `frame.h`'s `bits1`/`bits2`; `docs/ports/token-aliases.tsv` carries the C spellings
  (`scroll_region_ok`, `costs_set`, …) so use-site bodies stay comparable.  The offsets of the
  non-bitfield members around them are asserted by the layout probe
  (`tools/gen-layout-fields.py`).
- **`struct emacs_tty *old_tty` and `struct cm *Wcm` keep the untyped pointer** (PORTING.md P-72
  rule ③): sysdep.h's `emacs_tty` and cm.h's `cm` have no mirror yet, and both fields are only
  passed around as pointers.  Each carries its `/// C: …` line, and the use site casts.
- **`FRAME_TTY`/`CURTTY` stay macros** (`macro_rules!`), because C expands them at the use site
  and their bodies name `frame`'s `output_method` and `struct terminal`'s
  `display_info.tty` — a translation unit that includes `termchar.rs` also includes
  `frame.rs`/`termhooks.rs` through this mirror's `include!` list, and the macro is only
  resolved where it is used (PORTING.md P-33's shape).
- **`output_method` is read through a reader of that name** (`frame.rs`'s
  `pub fn output_method`), so `FRAME_TTY`'s body keeps both C tokens (`output_method`, `0`) and
  `extract-abi.py --check` is `OK` without a `macro-body-notes.tsv` exemption.
- **`(f)->terminal->display_info.tty` is `*mut c_void` in termhooks.rs** (termhooks.h 533-542's
  union carries the un-mirrored pointee types), so `FRAME_TTY` casts it back to
  `*mut tty_display_info`; the cast is the P-55 degradation, not a new pointer type.
- **`tty_handle_tab_bar_click`'s `struct input_event *`** is termhooks.h's type, mirrored there
  (PORTING.md P-70).
