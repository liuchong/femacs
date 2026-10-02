# emacs/src/keyboard.h (535 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 9
- [ ] kset_default_minibuffer_frame  (line 188)
- [ ] kset_defining_kbd_macro  (line 193)
- [ ] kset_input_decode_map  (line 198)
- [ ] kset_last_command  (line 203)
- [ ] kset_last_kbd_macro  (line 208)
- [ ] kset_prefix_arg  (line 213)
- [ ] kset_system_key_alist  (line 218)
- [ ] kset_window_system  (line 223)
- [ ] kbd_buffer_store_event_hold  (line 502)

## Macros defined — 18
- [ ] EMACS_KEYBOARD_H  (line 21, body da39a3ee5e6b)
- [ ] ENCODE_MENU_STRING  (line 397, body d9188e9ba52a)
- [ ] EVENT_CLICK_COUNT  (line 427, body ef5d61019ad6)
- [ ] EVENT_END  (line 424, body f49b46d3442d)
- [ ] EVENT_HAS_PARAMETERS  (line 406, body 345fde7e1b42)
- [ ] EVENT_HEAD  (line 410, body dbd7d83a8e77)
- [ ] EVENT_HEAD_KIND  (line 448, body 7a9a2f7de89c)
- [ ] EVENT_START  (line 417, body 98a87d19ffa5)
- [ ] KVAR  (line 38, body 0cc5f75755e6)
- [ ] POSN_BUFFER_POSN  (line 445, body 81543de49de3)
- [ ] POSN_INBUFFER_P  (line 444, body 0bf636cc9806)
- [ ] POSN_POSN  (line 431, body 97a797999863)
- [ ] POSN_SCROLLBAR_PART  (line 435, body 5d791ef17e81)
- [ ] POSN_SET_POSN  (line 432, body 1bfa24c5712d)
- [ ] POSN_STRING  (line 440, body 5d791ef17e81)
- [ ] POSN_TIMESTAMP  (line 434, body cc5fce14506c)
- [ ] POSN_WINDOW  (line 430, body 45e4359bb95b)
- [ ] POSN_WINDOW_POSN  (line 433, body 6dacb2cb29b7)

## Types and their fields — 2
- [ ] buffered_input_event: ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH, struct input_event ie
- [ ] kboard: KBOARD* next_kboard, Lisp_Object Voverriding_terminal_local_map_, Lisp_Object Vlast_command_, Lisp_Object Vreal_last_command_, Lisp_Object Vkeyboard_translate_table_, Lisp_Object Vlast_repeatable_command_, Lisp_Object Vprefix_arg_, Lisp_Object Vlast_prefix_arg_, Lisp_Object kbd_queue_, Lisp_Object defining_kbd_macro_, Lisp_Object* kbd_macro_buffer, Lisp_Object* kbd_macro_ptr, Lisp_Object* kbd_macro_end, ptrdiff_t kbd_macro_bufsize, Lisp_Object Vlast_kbd_macro_, Lisp_Object Vsystem_key_alist_, Lisp_Object system_key_syms_, Lisp_Object Vwindow_system_, Lisp_Object Vlocal_function_key_map_, Lisp_Object Vinput_decode_map_, Lisp_Object Vdefault_minibuffer_frame_, int reference_count, Lisp_Object echo_string_, bool_bf kbd_queue_has_data, bool_bf immediate_echo : 1, Lisp_Object echo_prompt_

## File-scope objects — 25
- [ ] extern KBOARD initial_kboard  (line 238)
- [ ] extern KBOARD current_kboard  (line 246)
- [ ] extern uintmax_t num_input_events  (line 257)
- [ ] extern ptrdiff_t point_before_last_command_or_undo  (line 262)
- [ ] extern struct buffer buffer_before_last_command_or_undo  (line 267)
- [ ] extern struct buffer prev_buffer  (line 269)
- [ ] extern int poll_suppress_count  (line 272)
- [ ] extern Lisp_Object this_command_keys  (line 278)
- [ ] extern ptrdiff_t this_command_key_count  (line 279)
- [ ] extern Lisp_Object internal_last_event_frame  (line 286)
- [ ] extern Lisp_Object item_properties  (line 292)
- [ ] extern Lisp_Object menu_items  (line 344)
- [ ] extern bool menu_items_inuse  (line 348)
- [ ] extern int menu_items_allocated  (line 351)
- [ ] extern int menu_items_used  (line 354)
- [ ] extern int menu_items_n_panes  (line 358)
- [ ] extern struct timespec input_available_clear_time  (line 453)
- [ ] extern union buffered_input_event kbd_buffer  (line 455)
- [ ] extern union buffered_input_event kbd_fetch_ptr  (line 456)
- [ ] extern union buffered_input_event kbd_store_ptr  (line 457)
- [ ] extern bool ignore_mouse_drag_p  (line 459)
- [ ] extern Lisp_Object real_this_command  (line 467)
- [ ] extern int quit_char  (line 469)
- [ ] extern bool input_was_pending  (line 470)
- [ ] extern char dev_tty  (line 529)

# ----------------------------------------------------------------------
## Decisions (pre-implementation, measured 2026-10-02)

- **Nothing about the two types is blocked any more.**  `union buffered_input_event` holds
  `struct input_event ie` plus the `kind : EVENT_KIND_WIDTH` unit, and `struct input_event`
  landed with termhooks.h (commit 4f5e38c); `struct kboard`'s 26 live fields are Lisp_Objects,
  ints, `char *` and `KBOARD *next_kboard` (a pointer to its own type), so no stand-in is
  needed for the types themselves.
- **What still blocks the file's *functions*.**  `keyboard.h`'s prototypes name
  `struct terminal`, `struct frame`, `struct buffer` (✓ mirrored) and `struct timespec`
  (✓ mirrored as `Time`-adjacent), so `struct terminal` (termhooks) and `struct frame`
  (frame.h) have to exist — or get P-70 stand-ins — before those prototypes are declared.
  The 25 file-scope objects are `Lisp_Object V*` declarations, which the port spells the way
  P-21 does for other headers.
