# emacs/src/terminal.c (725 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 9
- [x] delete-terminal  (Fdelete_terminal, 0/2, doc 9538c66fd764, line 381)
- [x] frame-terminal  (Fframe_terminal, 0/1, doc 940578f34b18, line 424)
- [x] terminal-live-p  (Fterminal_live_p, 1/1, doc c1ea2d48aeeb, line 443)
- [x] frame-initial-p  (Fframe_initial_p, 0/1, doc c6d87762c1ec, line 482)
- [x] terminal-list  (Fterminal_list, 0/0, doc 546de847650a, line 501)
- [x] terminal-name  (Fterminal_name, 0/1, doc 13bcb7beb0ae, line 517)
- [x] terminal-parameters  (Fterminal_parameters, 0/1, doc fa6989cbcabb, line 553)
- [x] terminal-parameter  (Fterminal_parameter, 2/2, doc 0aea7771cfbb, line 565)
- [x] set-terminal-parameter  (Fset_terminal_parameter, 3/3, doc ad30fe1b1267, line 575)

## Functions defined — 28
- [x] tset_param_alist  (line 48, static)
- [x] ring_bell  (line 56)
- [x] update_begin  (line 82)
- [x] update_end  (line 89)
- [x] set_terminal_window  (line 101)
- [x] cursor_to  (line 111)
- [x] raw_cursor_to  (line 129)
- [x] clear_to_end  (line 148)
- [x] clear_frame  (line 157)
- [x] clear_end_of_line  (line 169)
- [x] write_glyphs  (line 179)
- [x] insert_glyphs  (line 190)
- [x] delete_glyphs  (line 202)
- [x] ins_del_lines  (line 211)
- [x] decode_terminal  (line 223, static)
- [x] decode_live_terminal  (line 238)
- [x] decode_tty_terminal  (line 251)
- [x] get_named_terminal  (line 263)
- [x] allocate_terminal  (line 282, static)
- [x] create_terminal  (line 292)
- [x] delete_terminal  (line 330)
- [x] delete_terminal_internal  (line 360)
- [x] store_terminal_param  (line 536, static)
- [x] terminal_glyph_code  (line 628)
- [x] initial_free_frame_resources  (line 653, static)
- [x] init_initial_terminal  (line 663)
- [x] delete_initial_terminal  (line 683, static)
- [x] syms_of_terminal  (line 693)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 3
- [x] struct terminal terminal_list  (line 36)
- [x] static int next_terminal_id  (line 39, static)
- [x] struct terminal initial_terminal  (line 42)

## Decisions

- **No C counterpart (PORTING.md 0.1-12 / 1.5 / P-21 / P-51 / P-55 / P-72 / status 3.29):**
  - C 44's `static void delete_initial_terminal (struct terminal *);` forward declaration is
    deleted (a Rust item needs no declaration; the definition keeps C's place).
  - `cursor_to`/`raw_cursor_to`'s `int x, y;` (C leaves them to `root_xy`) start at `0`
    (PORTING.md 1.5), and `delete_terminal`/`Fterminal_list`/`Fframe_terminal`'s
    uninitialized `Lisp_Object`s start at `Qnil ()`.  The assignments still happen in C's
    order.
  - `create_terminal`'s `terminal->id = next_terminal_id++;` is the pair
    `(*terminal).id = next_terminal_id; next_terminal_id += 1;` (C's post-increment).
  - `delete_terminal_internal`'s `for (tp = &terminal_list; *tp != terminal; tp = &(*tp)->next_terminal)`
    becomes `tp = &raw mut terminal_list; while *tp != terminal { … tp = &raw mut (**tp).next_terminal; }`
    — the increment still runs after the body.
  - `--terminal->kboard->reference_count == 0` keeps C's single `if` by making the
    pre-decrement a `&&` block operand: `!kboard.is_null () && { …; (*kb).reference_count -= 1;
    (*kb).reference_count == 0 }`.
  - The three file-scope objects live in `pub mod terminal_globals` + `use terminal_globals::*;`
    because `include/termhooks.rs` declares `terminal_list`/`initial_terminal` at the crate
    root: an item shadows a glob import, while a second crate-root item would be `E0428`
    (P-21's mechanism from the other side).  `next_terminal_id` stays unexported (C's
    `static`); the other two carry `#[unsafe(no_mangle)]`.
  - `Vring_bell_function`/`Vdelete_terminal_functions` are defined here because this unit's
    `DEFVAR_LISP` owns them (status §3.29).
  - The hook calls: C's `if (t->hook) (*t->hook) (…)` is
    `if let Some (hook) = (*t).hook { hook (…) }` (`.unwrap ()` is forbidden), and the
    `struct frame *`/`struct glyph *` arguments the mirror spells `*mut c_void` (P-72 rule ③)
    are cast once per call.  `FRAME_TERMINAL`/`TERMINAL_ACTIVE_P` (termhooks.h 932/935) are
    `macro_rules!` in the mirror because they dereference types that header cannot name, and
    `FOR_EACH_FRAME` (frame.h 1404) is the mirror's macro with the body as its last argument.
- **Dead arms (P-51):** `#if HAVE_STRUCT_UNIPAIR_UNICODE` (C 29-33 and 587-622) is undefined in
  the frozen configuration.  The two *call sites* keep C's text behind
  `feature = "unipair_unicode"` (what the token check matches); `calculate_glyph_code_table`
  itself is not written out, because its body needs `<errno.h>`, `<linux/kd.h>`,
  `<sys/ioctl.h>` (no mirror carries them) and a `struct unipair`/`struct unimapdesc` the port
  would have to invent.  `HAVE_STRUCT_UNIPAIR_UNICODE` and `HAVE_ANDROID` (C 117/135's
  `#ifndef` arms, whose dead side is kept behind `feature = "have_android"`) have no entry in
  docs/baseline/config, so no `const _: () = assert!` binds either feature and `dead-arms`
  cannot flip them (the scroll.rs `glyph_debug` precedent).
- **Mirror corrections/additions made for this unit:**
  - `include/termhooks.rs`: `defined_color_hook`'s return type was `()`; C's is `bool`
    (termhooks.h 593-596, and image.c reads the value), so the field now returns `bool`.
    `FRAME_TERMINAL` and `TERMINAL_ACTIVE_P` added as `macro_rules!` (see above).
  - `include/frame.rs`: `FOR_EACH_FRAME` added as `macro_rules!` with the body as its last
    argument; the one token with no counterpart (`for`) is recorded in
    docs/ports/macro-body-notes.tsv.
- **Prototypes declared at their use site (their mirrors do not carry them yet, the
  doc.rs/scroll.rs shape):** `root_xy` (dispnew.c), `setup_coding_system` (coding.h 686),
  `delete_frame` (frame.h 1509), `delete_kboard` (keyboard.h 478), `decode_live_frame`
  (frame.h 1476), `free_frame_faces` (xfaces.c), `tty_defined_color` (xfaces.c 1144), and the
  dead arm's `encode_coding_utf_8`/`calculate_glyph_code_table`.
- **Line drift** is registered in docs/ports/line-drift-tolerance.tsv (`terminal 0.15`).
