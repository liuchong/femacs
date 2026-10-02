# emacs/src/termhooks.h (993 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 4
- [ ] TERMINALP  (line 889)
- [ ] XTERMINAL  (line 895)
- [ ] tset_charset_list  (line 904)
- [ ] tset_selection_alist  (line 909)

## Macros defined — 17
- [x] EMACS_TERMHOOKS_H  (line 22, body da39a3ee5e6b)
- [x] EVENT_INIT  (line 414, body 5b51055d64cb)
- [ ] FRAME_CHAR_INS_DEL_OK  (line 919, body 0d4fdde159ba)
- [ ] FRAME_KEYBOARD_CODING  (line 925, body fdc419ac88a6)
- [ ] FRAME_LINE_INS_DEL_OK  (line 918, body b3f54832342c)
- [ ] FRAME_MEMORY_BELOW_FRAME  (line 922, body 3a672f562c50)
- [ ] FRAME_MUST_WRITE_SPACES  (line 917, body a8c69100d5a2)
- [ ] FRAME_RIF  (line 930, body b828f603a91c)
- [ ] FRAME_SCROLL_REGION_COST  (line 921, body 5d826efc733b)
- [ ] FRAME_SCROLL_REGION_OK  (line 920, body d2b07c4a3e83)
- [ ] FRAME_TERMINAL  (line 932, body a119c2cfa155)
- [ ] FRAME_TERMINAL_CODING  (line 924, body f526927eb355)
- [ ] TERMINAL_ACTIVE_P  (line 935, body a37a94a45e9f)
- [ ] TERMINAL_KEYBOARD_CODING  (line 928, body 80dcf7ec3221)
- [ ] TERMINAL_TERMINAL_CODING  (line 927, body e6fda61a3ed1)
- [ ] cursorX  (line 984, body c86a224c6a7f)
- [ ] cursorY  (line 985, body d3ec6a6cfaae)

## Types and their fields — 2
- [x] input_event: ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH, ENUM_BF (scroll_bar_part) part : 16, unsigned code, unsigned modifiers, Lisp_Object x, Lisp_Object y, Time timestamp, Lisp_Object frame_or_window, Lisp_Object arg, Lisp_Object device
- [ ] terminal: union vectorlike_header header, Lisp_Object param_alist, Lisp_Object charset_list, Lisp_Object Vselection_alist, Lisp_Object glyph_code_table, struct terminal* next_terminal, int id, int reference_count, enum output_method type, char* name, struct kboard* kboard, struct tty_display_info* tty, struct x_display_info* x, struct w32_display_info* w32, struct ns_display_info* ns, struct pgtk_display_info* pgtk, struct haiku_display_info* haiku, struct android_display_info* android, struct coding_system* terminal_coding, struct coding_system* keyboard_coding, struct redisplay_interface* rif, void (*) cursor_to_hook, void (*) raw_cursor_to_hook, void (*) clear_to_end_hook, void (*) clear_frame_hook, void (*) clear_end_of_line_hook, void (*) ins_del_lines_hook, void (*) insert_glyphs_hook, void (*) write_glyphs_hook, void (*) delete_glyphs_hook, void (*) ring_bell_hook, void (*) toggle_invisible_pointer_hook, void (*) reset_terminal_modes_hook, void (*) set_terminal_modes_hook, void (*) update_begin_hook, void (*) update_end_hook, void (*) set_terminal_window_hook, void (*) query_frame_background_color, Lisp_Object (*) get_focus_frame, void (*) focus_frame_hook, void (*) frame_rehighlight_hook, void (*) frame_raise_lower_hook, void (*) frame_visible_invisible_hook, void (*) fullscreen_hook, void (*) iconify_frame_hook, int width, register int yoff, void (*) set_frame_alpha_hook, bool (*) set_bitmap_icon_hook, Lisp_Object title, void (*) change_tab_bar_height_hook, void (*) change_tool_bar_height_hook, void (*) set_scroll_bar_default_width_hook, void (*) set_scroll_bar_default_height_hook, void (*) condemn_scroll_bars_hook, void (*) redeem_scroll_bar_hook, void (*) judge_scroll_bars_hook, void (*) frame_up_to_date_hook, void (*) buffer_flipping_unblocked_hook, void (*) delete_frame_hook, void (*) delete_terminal_hook, void (*) toolkit_position_hook

## File-scope objects — 2
- [ ] extern struct terminal terminal_list  (line 915)
- [ ] extern struct terminal initial_terminal  (line 971)

# ----------------------------------------------------------------------
## Decisions

- **`struct input_event` is the first slice.**  `keyboard.h`'s `union buffered_input_event`
  embeds this structure **by value**, so it has to be complete before that header can be
  mirrored (port-plan.md §4.1).
- **The two bitfields share one unit.**  `kind : EVENT_KIND_WIDTH` (16) and `part : 16` are
  one 4-byte unit at offset 0 (`34 12 78 56` for kind 0x1234 / part 0x5678), so the port stores
  `bits: u32` with `INPUT_EVENT_KIND_MASK`/`INPUT_EVENT_PART_SHIFT`; `sizeof` 64 / `alignof` 8
  and the eight offsets after the unit (code 4, modifiers 8, x 16, y 24, timestamp 32,
  frame_or_window 40, arg 48, device 56) are asserted.
- **`ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH`** is `enum event_kind` on this target
  (lisp.h 480-483 switches to `unsigned int` only for `__IBMC__`/`__SUNPRO_C`), and its 37
  members plus `scroll_bar_part`'s 17 are compared by the constant gate.
- **`EVENT_INIT`** is C's compound literal `{.device = Qt}`: the members it does not name are
  zero-filled, which for the two Lisp_Objects C leaves is nil (NIL_IS_ZERO), so the port spells
  them `lisp_h_Qnil ()` and only `device` is `Qt ()`.
- **What still blocks this header (next slice).**  `struct terminal` (63 live fields) names
  `struct frame`, `struct window`, `struct glyph`, `struct redisplay_interface`, `struct kboard`
  and the seven platform `*_display_info` types, none mirrored yet — each gets a P-70 stand-in
  (the seven `display_info` members are live, not conditioned; measured).  `cursorX`/`cursorY`
  call `curX`/`curY` from `termchar.h`, so they wait for that header.  The `FRAME_*`/`TERMINAL_*`
  macros, `TERMINALP`/`XTERMINAL`, `tset_charset_list`/`tset_selection_alist` and the two
  file-scope objects land with `terminal`.
