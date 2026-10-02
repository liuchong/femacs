# emacs/src/keyboard.h (535 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 9
- [x] kset_default_minibuffer_frame  (line 188)
- [x] kset_defining_kbd_macro  (line 193)
- [x] kset_input_decode_map  (line 198)
- [x] kset_last_command  (line 203)
- [x] kset_last_kbd_macro  (line 208)
- [x] kset_prefix_arg  (line 213)
- [x] kset_system_key_alist  (line 218)
- [x] kset_window_system  (line 223)
- [x] kbd_buffer_store_event_hold  (line 502)

## Macros defined — 18
- [x] EMACS_KEYBOARD_H  (line 21, body da39a3ee5e6b)
- [x] ENCODE_MENU_STRING  (line 397, body d9188e9ba52a)
- [x] EVENT_CLICK_COUNT  (line 427, body ef5d61019ad6)
- [x] EVENT_END  (line 424, body f49b46d3442d)
- [x] EVENT_HAS_PARAMETERS  (line 406, body 345fde7e1b42)
- [x] EVENT_HEAD  (line 410, body dbd7d83a8e77)
- [x] EVENT_HEAD_KIND  (line 448, body 7a9a2f7de89c)
- [x] EVENT_START  (line 417, body 98a87d19ffa5)
- [x] KVAR  (line 38, body 0cc5f75755e6)
- [x] POSN_BUFFER_POSN  (line 445, body 81543de49de3)
- [x] POSN_INBUFFER_P  (line 444, body 0bf636cc9806)
- [x] POSN_POSN  (line 431, body 97a797999863)
- [x] POSN_SCROLLBAR_PART  (line 435, body 5d791ef17e81)
- [x] POSN_SET_POSN  (line 432, body 1bfa24c5712d)
- [x] POSN_STRING  (line 440, body 5d791ef17e81)
- [x] POSN_TIMESTAMP  (line 434, body cc5fce14506c)
- [x] POSN_WINDOW  (line 430, body 45e4359bb95b)
- [x] POSN_WINDOW_POSN  (line 433, body 6dacb2cb29b7)

## Types and their fields — 2
- [x] buffered_input_event: ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH, struct input_event ie
- [x] kboard: KBOARD* next_kboard, Lisp_Object Voverriding_terminal_local_map_, Lisp_Object Vlast_command_, Lisp_Object Vreal_last_command_, Lisp_Object Vkeyboard_translate_table_, Lisp_Object Vlast_repeatable_command_, Lisp_Object Vprefix_arg_, Lisp_Object Vlast_prefix_arg_, Lisp_Object kbd_queue_, Lisp_Object defining_kbd_macro_, Lisp_Object* kbd_macro_buffer, Lisp_Object* kbd_macro_ptr, Lisp_Object* kbd_macro_end, ptrdiff_t kbd_macro_bufsize, Lisp_Object Vlast_kbd_macro_, Lisp_Object Vsystem_key_alist_, Lisp_Object system_key_syms_, Lisp_Object Vwindow_system_, Lisp_Object Vlocal_function_key_map_, Lisp_Object Vinput_decode_map_, Lisp_Object Vdefault_minibuffer_frame_, int reference_count, Lisp_Object echo_string_, bool_bf kbd_queue_has_data, bool_bf immediate_echo : 1, Lisp_Object echo_prompt_

## File-scope objects — 25
- [x] extern KBOARD initial_kboard  (line 238)
- [x] extern KBOARD current_kboard  (line 246)
- [x] extern uintmax_t num_input_events  (line 257)
- [x] extern ptrdiff_t point_before_last_command_or_undo  (line 262)
- [x] extern struct buffer buffer_before_last_command_or_undo  (line 267)
- [x] extern struct buffer prev_buffer  (line 269)
- [x] extern int poll_suppress_count  (line 272)
- [x] extern Lisp_Object this_command_keys  (line 278)
- [x] extern ptrdiff_t this_command_key_count  (line 279)
- [x] extern Lisp_Object internal_last_event_frame  (line 286)
- [x] extern Lisp_Object item_properties  (line 292)
- [x] extern Lisp_Object menu_items  (line 344)
- [x] extern bool menu_items_inuse  (line 348)
- [x] extern int menu_items_allocated  (line 351)
- [x] extern int menu_items_used  (line 354)
- [x] extern int menu_items_n_panes  (line 358)
- [x] extern struct timespec input_available_clear_time  (line 453)
- [x] extern union buffered_input_event kbd_buffer  (line 455)
- [x] extern union buffered_input_event kbd_fetch_ptr  (line 456)
- [x] extern union buffered_input_event kbd_store_ptr  (line 457)
- [x] extern bool ignore_mouse_drag_p  (line 459)
- [x] extern Lisp_Object real_this_command  (line 467)
- [x] extern int quit_char  (line 469)
- [x] extern bool input_was_pending  (line 470)
- [x] extern char dev_tty  (line 529)

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

# ----------------------------------------------------------------------
## Decisions

- **`bool_bf` is one byte, not four.**  conf_post.h 58-62 makes it `typedef bool bool_bf;` on
  this target, so `kbd_queue_has_data` (184) and the `immediate_echo : 1` unit (185) are two
  separate bytes — measured with both set (`01 01`) and with only `immediate_echo` (`00 01`).
  The port stores the flag in its own `u8` and masks bit 0.  (An earlier comment in
  `dispextern.rs` said `bool_bf` was `unsigned int`; that was wrong and is corrected.)
- **`union buffered_input_event` cannot carry a Rust bitfield.**  C's first member is
  `ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH`, the low 16 bits of the bytes `ie` starts
  with; the port declares a `u16` view of exactly those bits next to the complete `input_event`.
  C's `static_assert` on the two types' size and alignment is kept, as the port's
  `static_assert!`, inside `kbd_buffer_store_event_hold` where C has it.
- **`KVAR` is excluded.**  `((kboard)->field ## _)` pastes a token; Rust has no token pasting,
  so the port writes the field at each use site (every field it names ends in `_`).  Recorded in
  `docs/ports/excluded-macros.txt`, like `lisp_h_Qnil`.
- **The `lisp.rs` stand-in is gone, and `KBOARD` moved here.**  `lisp.rs` used to declare an
  opaque zero-sized `kboard` for `typedef struct kboard KBOARD;`; the real definition is this
  file's, so the stand-in was deleted and the typedef came with it (PORTING.md P-70/P-72).
  `lisp.h 5081`'s `extern struct kboard *echo_kboard;` stays in `lisp.rs`, because that is the
  header whose ledger must list it, and its pointee degrades to `*mut c_void` there — the tag is
  incomplete in a unit that includes only `lisp.h`, exactly as in C.
- **`sizeof`/`alignof` are now token families.**  `docs/ports/token-aliases.tsv` records that
  C's `sizeof`/`alignof` are Rust's `size_of`/`align_of`, which turns their occurrence into a
  checked invariant for every mirror; adding it regressed no existing file.
- **The gate caught a real typo here.**  `EVENT_END`'s first draft dropped the outer
  `CAR_SAFE`; the token check reported "does not mention car_safe" before the commit.
- **Not yet done: none for this header.**  `extract-abi.py --check emacs/src/keyboard.h
  femacs/src/include/keyboard.rs` reports `OK`; the prototypes after line 520 are declarations
  (outside the checker's inventory, P-64) and land with keyboard.c.
