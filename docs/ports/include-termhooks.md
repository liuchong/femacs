# emacs/src/termhooks.h (993 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 4
- [x] TERMINALP  (line 889)
- [x] XTERMINAL  (line 895)
- [x] tset_charset_list  (line 904)
- [x] tset_selection_alist  (line 909)

## Macros defined — 17
- [x] EMACS_TERMHOOKS_H  (line 22, body da39a3ee5e6b)
- [x] EVENT_INIT  (line 414, body 5b51055d64cb)
- [x] FRAME_CHAR_INS_DEL_OK  (line 919, body 0d4fdde159ba)
- [x] FRAME_KEYBOARD_CODING  (line 925, body fdc419ac88a6)
- [x] FRAME_LINE_INS_DEL_OK  (line 918, body b3f54832342c)
- [x] FRAME_MEMORY_BELOW_FRAME  (line 922, body 3a672f562c50)
- [x] FRAME_MUST_WRITE_SPACES  (line 917, body a8c69100d5a2)
- [x] FRAME_RIF  (line 930, body b828f603a91c)
- [x] FRAME_SCROLL_REGION_COST  (line 921, body 5d826efc733b)
- [x] FRAME_SCROLL_REGION_OK  (line 920, body d2b07c4a3e83)
- [x] FRAME_TERMINAL  (line 932, body a119c2cfa155)
- [x] FRAME_TERMINAL_CODING  (line 924, body f526927eb355)
- [x] TERMINAL_ACTIVE_P  (line 935, body a37a94a45e9f)
- [x] TERMINAL_KEYBOARD_CODING  (line 928, body 80dcf7ec3221)
- [x] TERMINAL_TERMINAL_CODING  (line 927, body e6fda61a3ed1)
- [x] cursorX  (line 984, body c86a224c6a7f)
- [x] cursorY  (line 985, body d3ec6a6cfaae)

## Types and their fields — 2
- [x] input_event: ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH, ENUM_BF (scroll_bar_part) part : 16, unsigned code, unsigned modifiers, Lisp_Object x, Lisp_Object y, Time timestamp, Lisp_Object frame_or_window, Lisp_Object arg, Lisp_Object device
- [x] terminal: union vectorlike_header header, Lisp_Object param_alist, Lisp_Object charset_list, Lisp_Object Vselection_alist, Lisp_Object glyph_code_table, struct terminal* next_terminal, int id, int reference_count, enum output_method type, char* name, struct kboard* kboard, struct tty_display_info* tty, struct x_display_info* x, struct w32_display_info* w32, struct ns_display_info* ns, struct pgtk_display_info* pgtk, struct haiku_display_info* haiku, struct android_display_info* android, struct coding_system* terminal_coding, struct coding_system* keyboard_coding, struct redisplay_interface* rif, void (*) cursor_to_hook, void (*) raw_cursor_to_hook, void (*) clear_to_end_hook, void (*) clear_frame_hook, void (*) clear_end_of_line_hook, void (*) ins_del_lines_hook, void (*) insert_glyphs_hook, void (*) write_glyphs_hook, void (*) delete_glyphs_hook, void (*) ring_bell_hook, void (*) toggle_invisible_pointer_hook, void (*) reset_terminal_modes_hook, void (*) set_terminal_modes_hook, void (*) update_begin_hook, void (*) update_end_hook, void (*) set_terminal_window_hook, void (*) query_frame_background_color, Lisp_Object (*) get_focus_frame, void (*) focus_frame_hook, void (*) frame_rehighlight_hook, void (*) frame_raise_lower_hook, void (*) frame_visible_invisible_hook, void (*) fullscreen_hook, void (*) iconify_frame_hook, int width, register int yoff, void (*) set_frame_alpha_hook, bool (*) set_bitmap_icon_hook, Lisp_Object title, void (*) change_tab_bar_height_hook, void (*) change_tool_bar_height_hook, void (*) set_scroll_bar_default_width_hook, void (*) set_scroll_bar_default_height_hook, void (*) condemn_scroll_bars_hook, void (*) redeem_scroll_bar_hook, void (*) judge_scroll_bars_hook, void (*) frame_up_to_date_hook, void (*) buffer_flipping_unblocked_hook, void (*) delete_frame_hook, void (*) delete_terminal_hook, void (*) toolkit_position_hook

## File-scope objects — 2
- [x] extern struct terminal terminal_list  (line 915)
- [x] extern struct terminal initial_terminal  (line 971)

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

# ----------------------------------------------------------------------
## Decisions

- **`struct terminal` is 520/8 and every offset is asserted.**  The hook table is a
  contiguous run of 51 function pointers from 112 to 512, so a single misplaced entry
  would move all of them; the baseline pins each one.
- **Pointee degradations (P-72 rule ③).**  `frame.h`/`window.h`/`termchar.h` and the
  platform display headers are not included by `termhooks.h`, so C only ever sees those
  types as incomplete tags behind pointers here: the port spells them `*mut c_void` with a
  `/// C:` note.  `struct glyph *` and `struct redisplay_interface *` *are* complete in C's
  view (dispextern.h is included) but their mirrors come later in the display chain, so they
  carry the same note until then.
- **A union's inner members are not the container's.**  `display_info` is an anonymous union
  whose members the scanner reports with the enclosing container and a `nesting` path; the
  layout generator now keys only one-level members (`display_info`), because C rejects
  `offsetof (struct terminal, tty)`.
- **`type` is a Rust keyword**, so the member is `r#type` (the generator's `rust_ident`
  spells the C key the same way).
- **Still blocked (12 macros).**  `FRAME_MUST_WRITE_SPACES`, `FRAME_LINE_INS_DEL_OK`,
  `FRAME_CHAR_INS_DEL_OK`, `FRAME_SCROLL_REGION_OK`, `FRAME_SCROLL_REGION_COST`,
  `FRAME_MEMORY_BELOW_FRAME`, `FRAME_TERMINAL_CODING`, `FRAME_KEYBOARD_CODING`, `FRAME_RIF`,
  `FRAME_TERMINAL` dereference `struct frame` (frame.rs); `TERMINAL_ACTIVE_P` dereferences
  `display_info.tty` (termchar.h); `cursorX`/`cursorY` call `curX`/`curY` from termchar.h.

# ----------------------------------------------------------------------
## Decisions (third slice: the last nine macros and the prototypes)

- **The nine `FRAME_*`/`TERMINAL_*` macros are mirrors of C's macro shape.**  The five that
  dereference `struct tty_display_info` (`FRAME_MUST_WRITE_SPACES`, `FRAME_LINE_INS_DEL_OK`,
  `FRAME_CHAR_INS_DEL_OK`, `FRAME_MEMORY_BELOW_FRAME`, and the already-present
  `FRAME_SCROLL_REGION_OK`/`_COST`) stay `macro_rules!`: the type is only reachable at the
  use site through `FRAME_TTY` (a termchar.rs macro), and a `fn` here would be E0412 for every
  unit that includes termhooks.rs without termchar.rs (the §3.54 hazard).  Their four packed
  `bool_bf` members are exempted in `docs/ports/macro-body-notes.tsv`, one row each, exactly
  as `FRAME_SCROLL_REGION_OK`'s `scroll_region_ok` row already did.
- **`FRAME_TERMINAL_CODING`/`FRAME_KEYBOARD_CODING`/`FRAME_RIF` are `fn`s**: `struct terminal`
  *is* declared by this mirror, so nothing is degraded (the `TERMINAL_TERMINAL_CODING` shape).
  `FRAME_SCROLL_REGION_COST` was an rvalue (an `unsafe { … }` block) and had to become C's
  lvalue for term.c's `FRAME_SCROLL_REGION_COST (frame) = string_cost (f)`.
- **`cursorX`/`cursorY`** are `#define cursorX(t) curX(t)` on this target (WINDOWSNT is
  `#undef`), so both stay macros that expand `curX!`/`curY!` from cm.h at the use site.
- **Still not mirrored**: termhooks.h 940-1000's prototypes and the
  `#ifdef HAVE_WINDOW_SYSTEM`/`HAVE_GPM`/`WINDOWSNT` arms (each is declared at its use site in
  `term.rs`, the documented P-66 boundary), and `TERMINAL_FONT_CACHE` (every arm is dead on
  this target).
- **Gate**: `python3 tools/extract-abi.py --check emacs/src/termhooks.h
  femacs/src/include/termhooks.rs` → `OK`.
