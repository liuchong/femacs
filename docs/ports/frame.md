# emacs/src/frame.c (7907 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 66
- [x] framep  (Fframep, 1/1, doc 4f91c4d2fbba, line 286)
- [x] frame-live-p  (Fframe_live_p, 1/1, doc 11ff775ef2e0, line 326)
- [x] frame-id  (Fframe_id, 0/1, doc f289fcbcb305, line 345)
- [x] window-system  (Fwindow_system, 0/1, doc 6a9b13da901e, line 415)
- [x] frame-windows-min-size  (Fframe_windows_min_size, 4/4, doc 76d779f19812, line 495)
- [x] make-terminal-frame  (Fmake_terminal_frame, 1/1, doc 8bebd66a1678, line 1736)
- [x] select-frame  (Fselect_frame, 1/2, doc 32cf6ee01793, line 2097)
- [x] handle-switch-frame  (Fhandle_switch_frame, 1/1, doc 35cbb1fd733e, line 2126)
- [x] selected-frame  (Fselected_frame, 0/0, doc 3c054f509beb, line 2142)
- [x] old-selected-frame  (Fold_selected_frame, 0/0, doc c56243113513, line 2149)
- [x] frame-list  (Fframe_list, 0/0, doc f76baa4a6a2d, line 2161)
- [x] frame-parent  (Fframe_parent, 0/1, doc a8eac84d5397, line 2180)
- [x] frame-ancestor-p  (Fframe_ancestor_p, 2/2, doc f73cd3411ba9, line 2242)
- [x] frame-root-frame  (Fframe_root_frame, 0/1, doc e5ae09bb56e9, line 2269)
- [x] next-frame  (Fnext_frame, 0/2, doc 9f1420a62c04, line 2415)
- [x] previous-frame  (Fprevious_frame, 0/2, doc 1cf34128b904, line 2441)
- [x] last-nonminibuffer-frame  (Flast_nonminibuf_frame, 0/0, doc 03a98ef0d3ae, line 2467)
- [x] delete-frame  (Fdelete_frame, 0/2, doc 25f1b9319c8e, line 3150)
- [x] mouse-position  (Fmouse_position, 0/0, doc bc805c817668, line 3271)
- [x] mouse-pixel-position  (Fmouse_pixel_position, 0/0, doc 10185049c011, line 3329)
- [x] set-mouse-position  (Fset_mouse_position, 3/3, doc 4de496c9960a, line 3407)
- [x] set-mouse-pixel-position  (Fset_mouse_pixel_position, 3/3, doc 7e21ca01935b, line 3457)
- [x] make-frame-visible  (Fmake_frame_visible, 0/1, doc eda3336bf392, line 3505)
- [x] make-frame-invisible  (Fmake_frame_invisible, 0/2, doc b0f1cef2fec7, line 3549)
- [x] iconify-frame  (Ficonify_frame, 0/1, doc f12199e6dfd4, line 3591)
- [x] frame-visible-p  (Fframe_visible_p, 1/1, doc 06bc2b37fec9, line 3633)
- [x] visible-frame-list  (Fvisible_frame_list, 0/0, doc 070c2a01cb3b, line 3652)
- [x] raise-frame  (Fraise_frame, 0/1, doc bd313f46ee9a, line 3667)
- [x] lower-frame  (Flower_frame, 0/1, doc 722310663b5f, line 3686)
- [x] redirect-frame-focus  (Fredirect_frame_focus, 1/2, doc c4706aeda107, line 3702)
- [x] frame-focus  (Fframe_focus, 0/1, doc f8019955b620, line 3746)
- [x] x-focus-frame  (Fx_focus_frame, 1/2, doc e4a8dad8e20e, line 3756)
- [x] frame-after-make-frame  (Fframe_after_make_frame, 2/2, doc d69a2444ddde, line 3772)
- [x] frame-parameters  (Fframe_parameters, 0/1, doc 5c11ee07a06d, line 4083)
- [x] frame-parameter  (Fframe_parameter, 2/2, doc c631c4276663, line 4172)
- [x] modify-frame-parameters  (Fmodify_frame_parameters, 2/2, doc a21a9785ca31, line 4243)
- [x] frame-char-height  (Fframe_char_height, 0/1, doc cb7b0296d98c, line 4345)
- [x] frame-char-width  (Fframe_char_width, 0/1, doc 4707b526fead, line 4363)
- [x] frame-native-width  (Fframe_native_width, 0/1, doc 6aea9f198b3b, line 4381)
- [x] frame-native-height  (Fframe_native_height, 0/1, doc 8e2052ba07d5, line 4401)
- [x] tool-bar-pixel-width  (Ftool_bar_pixel_width, 0/1, doc 6e27dc322b24, line 4430)
- [x] frame-text-cols  (Fframe_text_cols, 0/1, doc 47fd165280a5, line 4447)
- [x] frame-text-lines  (Fframe_text_lines, 0/1, doc 9e07436df740, line 4454)
- [x] frame-total-cols  (Fframe_total_cols, 0/1, doc 3c7c8e48908d, line 4461)
- [x] frame-total-lines  (Fframe_total_lines, 0/1, doc 11485c022463, line 4468)
- [x] frame-text-width  (Fframe_text_width, 0/1, doc fd99b4e35a90, line 4475)
- [x] frame-text-height  (Fframe_text_height, 0/1, doc 1295808bba9e, line 4482)
- [x] frame-scroll-bar-width  (Fscroll_bar_width, 0/1, doc a3048f67f659, line 4489)
- [x] frame-scroll-bar-height  (Fscroll_bar_height, 0/1, doc 3eb68e8173e2, line 4496)
- [x] frame-fringe-width  (Ffringe_width, 0/1, doc abc24b5a50f1, line 4503)
- [x] frame-child-frame-border-width  (Fframe_child_frame_border_width, 0/1, doc 00fa40692584, line 4510)
- [x] frame-internal-border-width  (Fframe_internal_border_width, 0/1, doc 288a148e2803, line 4524)
- [x] frame-right-divider-width  (Fright_divider_width, 0/1, doc adb7dfe3a0e3, line 4531)
- [x] frame-bottom-divider-width  (Fbottom_divider_width, 0/1, doc ec88c2cbd2cb, line 4538)
- [x] set-frame-height  (Fset_frame_height, 2/4, doc 52ddc411639f, line 4563)
- [x] set-frame-width  (Fset_frame_width, 2/4, doc 88eb7ce23a3a, line 4592)
- [x] set-frame-size  (Fset_frame_size, 3/4, doc a703d54a255e, line 4621)
- [x] frame-position  (Fframe_position, 0/1, doc 38244a590e10, line 4644)
- [x] set-frame-position  (Fset_frame_position, 3/3, doc 28e5d89a9307, line 4662)
- [x] set-frame-size-and-position-pixelwise  (Fset_frame_size_and_position_pixelwise, 5/6, doc cb9035db063c, line 4697)
- [x] frame-window-state-change  (Fframe_window_state_change, 0/1, doc 875aaffcc3bc, line 4760)
- [x] set-frame-window-state-change  (Fset_frame_window_state_change, 0/2, doc 33a3650845a0, line 4775)
- [x] frame-scale-factor  (Fframe_scale_factor, 0/1, doc d2d8b2c743dd, line 4793)
- [x] frame-pointer-visible-p  (Fframe_pointer_visible_p, 0/1, doc 38724019d098, line 7001)
- [x] mouse-position-in-root-frame  (Fmouse_position_in_root_frame, 0/0, doc bbe78b0cf8ed, line 7011)
- [x] frame--set-was-invisible  (Fframe__set_was_invisible, 2/2, doc c8a999cdfcac, line 7042)

## Functions defined — 57
- [x] fset_buffer_predicate  (line 87, static)
- [x] fset_minibuffer_window  (line 92, static)
- [x] decode_live_frame  (line 98)
- [x] decode_any_frame  (line 107)
- [x] decode_window_system_frame  (line 124)
- [x] check_window_system  (line 134)
- [x] check_tty  (line 145, static)
- [x] decode_tty_frame  (line 159)
- [x] frame_get  (line 170, static)
- [x] get_frame_param  (line 183)
- [x] frame_inhibit_resize  (line 200)
- [x] set_menu_bar_lines  (line 222, static)
- [x] set_tab_bar_lines  (line 266, static)
- [x] frame_set_id  (line 368)
- [x] frame_set_id_from_params  (line 401)
- [x] frame_redisplay_p  (line 452)
- [x] frame_windows_min_size  (line 528, static)
- [x] keep_ratio  (line 606, static)
- [x] frame_size_history_adjust  (line 698, static)
- [x] frame_size_history_plain  (line 737)
- [x] frame_size_history_extra  (line 753)
- [x] adjust_frame_size  (line 826)
- [x] allocate_frame  (line 1147, static)
- [x] make_frame  (line 1154)
- [x] make_frame_without_minibuffer  (line 1314)
- [x] make_minibuffer_frame  (line 1365)
- [x] frame_next_F_name  (line 1409, static)
- [x] make_initial_frame  (line 1423)
- [x] make_terminal_frame  (line 1480, static)
- [x] get_future_frame_param  (line 1623, static)
- [x] tty_child_pos_param  (line 1645)
- [x] tty_child_size_param  (line 1685)
- [x] tty_child_frame_rect  (line 1725, static)
- [x] do_switch_frame  (line 1922)
- [x] frame_ancestor_p  (line 2211)
- [x] frame_subsumes_p  (line 2229, static)
- [x] root_frame  (line 2261)
- [x] candidate_frame  (line 2299, static)
- [x] next_frame  (line 2342, static)
- [x] prev_frame  (line 2382, static)
- [x] other_frames  (line 2496, static)
- [x] delete_frame  (line 2598)
- [x] frame_internal_border_part  (line 3192)
- [x] mouse_position  (line 3291)
- [x] make_frame_visible_1  (line 3535, static)
- [x] frames_discard_buffer  (line 3795)
- [x] store_in_alist  (line 3812)
- [x] frame_name_fnn_p  (line 3822, static)
- [x] set_term_frame_name  (line 3839, static)
- [x] store_frame_param  (line 3872)
- [x] frame_unspecified_color  (line 4075, static)
- [x] check_frame_pixels  (line 4546, static)
- [x] frame_make_pointer_invisible  (line 6974)
- [x] frame_make_pointer_visible  (line 6989)
- [x] init_frame_once  (line 7198)
- [x] init_frame_once_for_pdumper  (line 7209, static)
- [x] syms_of_frame  (line 7216)

## Macros defined — 0

## Types and their fields — 1
- [x] frame_parm_table: const char* name, int sym

## File-scope objects — 8
- [x] Lisp_Object selected_frame  (line 58)
- [x] Lisp_Object old_selected_frame  (line 61)
- [x] static struct frame last_nonminibuf_frame  (line 66, static)
- [x] bool frame_garbaged  (line 69)
- [x] int frame_default_tab_bar_height  (line 72)
- [x] int frame_default_tool_bar_height  (line 78)
- [x] Lisp_Object Vframe_list  (line 284)
- [x] static intmax_t tty_frame_count  (line 1406, static)

# ----------------------------------------------------------------------
## Decisions

**STATE: COMPLETE.** 66/66 subrs, 57/57 functions, the `frame_parm_table` type, all 8
file-scope objects and the 51-entry `frame_parms` table are ported, and every box above is
ticked.  The gates, with the last line each produced:

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-frame cargo run -q -p xtask -- build-tu src/frame.c
built src/frame.rs -> ../build/femacs/frame.o

$ python3 tools/extract-abi.py --check emacs/src/frame.c femacs/src/frame.rs
OK emacs/src/frame.c -> femacs/src/frame.rs

$ python3 tools/check-globals.py
check-globals: 265 units, 0 DEFVAR_* variables without a definition in their own unit

$ python3 tools/check-undef.py
check-undef: 515 unit object(s), 151 reference object(s)/archive(s): 151 pending, 0 new
  (this unit's dependencies on unported units were added to
  `docs/baseline/undef-pending.txt` by hand; a later `--write-pending` from another lane pruned
  the entries whose owners' reference objects have meanwhile landed, and all 39 of frame.o's
  orphans are in the ratchet)

$ python3 tools/check-linkage.py
check-linkage: 3 unit object(s) compared against the non-static objects of the C sources the frozen build links, 0 with 0 missing symbol(s)   # src/frame is one of the three (its 6 non-static objects: _Vframe_list, _frame_garbaged, _frame_default_tab_bar_height, _frame_default_tool_bar_height, _old_selected_frame, _selected_frame)

$ cd femacs && rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort -C codegen-units=1 -C opt-level=0 --cfg enable_checking -o /tmp/frame-ec.o ../build/femacs/units/frame/_root.rs
exit 0   (two warnings, both from the mirrors: lisp.rs's `/// C:` doc comment on an extern block and frame.rs's doubled `#[inline(always)]`)

$ python3 tools/check-declarations.py
8 declaration sites (0 forbidden, 8 allowed literals), 0 failures

$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures

$ grep -c '^\- \[ \]' docs/ports/frame.md
0
```

### The dead arms (P-08: recorded here, not transcribed)

`python3 tools/check-cond-eval.py src/frame.c` = `105 conditions, 0 unknown, 0 disagreements
[VERIFIED]`.  The frozen configuration's undefined macros are (macros.tsv): `HAVE_WINDOW_SYSTEM`
1677, `HAVE_X_WINDOWS` 1719, `HAVE_ANDROID` 1239, `MSDOS` 2848, `USE_X_TOOLKIT` 3872,
`USE_MOTIF` 3864, `USE_GTK` 3861, `HAVE_NS` 1480, `HAVE_NTGUI` 1481, `NS_IMPL_COCOA` 2995,
`HAVE_PGTK` 1488, `HAVE_HAIKU` 1387, `HAVE_GPM` 1381, `HAVE_TEXT_CONVERSION` 1644, `WINDOWSNT`
4009, plus `HAVE_EXT_TOOL_BAR`, `FRAME_TOOLBAR_WIDTH` and `HAVE_XCB`, which are not in the table
at all — the evaluator answers `defined (NAME)` false for every one of them, exactly as clang
does for the frozen build.  Arms the port therefore does not carry:

  * **`#ifdef HAVE_WINDOW_SYSTEM`** — C 31-33 (`#include TERM_HEADER`), 44-46 (`fontset.h`),
    81-83 (`gui_report_frame_params`'s prototype), 115-121 (`display_available`), 128-130
    (`decode_window_system_frame`'s `return f;`), 136-139 (`check_window_system`'s
    `if (window_system_available (f)) return;`), 1187-1213 (`make_frame`'s flag run),
    1452-1455 (`make_initial_frame`), 1593-1596 (`make_terminal_frame`), 2167-2175
    (`Fframe_list`'s X arm and its `#else` counterpart is the live one), 2535-2547 (a comment
    block inside `other_frames`'s `#ifdef HAVE_XCB`), 2885-2888 (`delete_frame`),
    3371-3405 (the X mouse movers), 3764-3768 (`Fx_focus_frame`), 4156-4160 (`Fframe_parameters`),
    4189-4208 (`Fframe_parameter`'s window-system arms), 4261-4265, 4352-4358, 4371-4377,
    4393-4397, 4422-4426 (`Fframe_char_height`/`_width`/`native_width`/`native_height`),
    4438-4443 (`#ifdef FRAME_TOOLBAR_WIDTH`), 4680-4683 (`Fset_frame_position`), 4880-4886
    (the two `frame_parms` entries), 7055-7112 and 7120-7187 (the pointer and `Display_Info`
    machinery), 7251-7253, 7285-7287, 7316-7322, 7396-7426, 7455-7462, 7551-7552, 7685-7691,
    7877-7894 — and **the whole GUI frame-geometry implementation at C 4889-6970**.
  * **`#ifdef MSDOS`** — C 48-51, 1030-1043, 1568-1576, 1762-1764, 1782-1788, 3436-3442,
    3481-3487 (each of the last two keeps its `#else` text, which is live).
  * **`#ifdef HAVE_X_WINDOWS`** — C 482-491 (keeps the `#ifndef` arm's
    `return FRAME_VISIBLE_P (f);`), 943-945 (`USE_MOTIF` too), 1953-1979, 2607-2609, 2622-2626,
    2716-2719, 2852-2866, 2928-2933, 2950-2957, 7766-7768.
  * **`#ifndef HAVE_ANDROID`** — C 1477/1722 open *live* regions (HAVE_ANDROID is undef); C
    1755-1757, 7551's neighbour `#ifdef HAVE_ANDROID`, 2868-2877 and 7551-7552 are dead.
  * **`#ifdef HAVE_GPM`** — C 3446-3451 and 3491-3496 keep the `#else` text (`(void) xval;`).
  * **`#if 0`** — C 2558-2565 (inside `other_frames`) and 7828-7831 (`defsubr` arms).
  * **`#ifdef HAVE_TEXT_CONVERSION`** — C 1218-1226, 2963-2966.
  * **`#ifdef HAVE_EXT_TOOL_BAR`** — C 75-77, so the live arm is the `#else`'s
    `int frame_default_tool_bar_height;` (C 78), which is what the ledger's file-scope object
    is.

### Deviations (each one is also in `docs/ports/macro-body-notes.tsv` where it is a body token)

1. **Chained assignments are split.**  C's `A = B = C = v;` is one statement whose value is
   `v`; Rust's assignment is not an expression.  Five sites keep C's effects in C's order as
   two statements: `set_menu_bar_lines` (C 234, 238-239), `set_tab_bar_lines` (C 278),
   `make_frame` (C 1252-1256, four chains, each expanded to set every place to the same value)
   and `x = y = Qnil;` (C 3298, 3347).  `set-frame-window-state-change` (C 4790) keeps C's
   assign-then-test inside a block expression.
2. **`check_window_system` is `-> !`.**  C declares it `void`, but in this configuration its
   only live tail is `error (…)` (its `if (window_system_available (f)) return;` is in the
   dead `HAVE_WINDOW_SYSTEM` arm, C 136-139), so the port returns `!`; that is what lets
   `decode_window_system_frame` (C 123-131) keep C's live text, whose only `return f;` is in
   its own dead arm.
3. **`l-value` header macros are defined at the use site.**  `frame.h`'s accessors are macros
   in C; `include/frame.rs` mirrors the readers as functions and packs the `bool_bf` run into
   `bits1`/`bits2` (PORTING.md 1.3), so every macro this file *writes* to — `SET_FRAME_VISIBLE`,
   `SET_FRAME_GARBAGED`, `FRAME_COLS`, `FRAME_LINES`, `FRAME_TOTAL_COLS`, `FRAME_TOTAL_LINES`,
   `FRAME_TEXT_WIDTH`, `FRAME_TEXT_HEIGHT`, `FRAME_PIXEL_WIDTH`, `FRAME_PIXEL_HEIGHT`,
   `FRAME_MENU_BAR_LINES`/`_HEIGHT`, `FRAME_TAB_BAR_LINES`/`_HEIGHT`,
   `FRAME_FOREGROUND_PIXEL`/`FRAME_BACKGROUND_PIXEL`, `FRAME_NO_SPECIAL_GLYPHS`,
   `FRAME_WINDOW_STATE_CHANGE`, the geometry conversions (`FRAME_TEXT_TO_PIXEL_*`,
   `FRAME_PIXEL_TO_TEXT_*`, `FRAME_INNER_*`, `FRAME_PIXEL_WIDTH_TO_TEXT_COLS`, …) — plus the
   field-name readers/writers for that flag run (`visible!`, `resized_p!`, `wants_modeline!`,
   `mini!`, `output_method!`, …) and `WINDOW_FRAME!`/`WINDOW_XFRAME` are defined in this unit
   under C's own name, so the body keeps C's token and the write goes through the mirror's
   mask.  `NAME! (…)` is the Rust macro call; the token check ignores the `!`.
4. **Two modules exist for `E0428` reasons.**  `include/frame.rs` declares `selected_frame`,
   `old_selected_frame`, `frame_garbaged`, `frame_default_tab_bar_height`,
   `frame_default_tool_bar_height`, `frame_next_id` and `Vframe_list` at the crate root, and
   `frame_make_pointer_invisible` too, so the definitions live in `frame_globals` and
   `frame_pointer_fns`, which are glob-imported (PORTING.md P-21's mechanism from the other
   side; the terminal.rs/term.rs `*_globals` shape).
5. **`(void) x;` is `let _ = x;`** (C 3449-3450, 3494-3495, 4684-4685; the dired.rs shape) and
   C's `1`/`0` passed to a Rust `bool` parameter/field is `true`/`false` (C 1983's
   `resize_mini_window (…, 1)`, `make_frame`'s flag writes).
6. **`switch` → `match`** in `Fframep` (C 302-323) with C's arm order and `default:` as `_ =>`;
   **`goto next_name`** in `frame_next_F_name` (C 1418) is a labelled `loop`/`continue`
   (PORTING.md 2.2) because the jump target is inside the `FOR_EACH_FRAME` body.
7. **Token-pasting macros.**  `SYMBOL_INDEX (Qx)` in the `frame_parms` table is the pasted
   `iQx` constant (the P-17 generated names), and `KVAR (kb, Vdefault_minibuffer_frame)` is
   `(*kb).Vdefault_minibuffer_frame_` (keyboard.rs's field; `excluded-macros.txt`'s KVAR row).
8. **`UNINIT`** (`make_frame`'s `mw`, `delete_frame`'s `frame1`) is zero-initialized
   (PORTING.md 1.5), and `PRIdMAX` is written as the `intmax_t` conversion it expands to
   (`c"F%jd"`, the print.rs row).
9. **`ckd_mul (&pixel_size, sz, item_size)`** (C 4556) is `sz.overflowing_mul (item_size)` with
   the wrapped value stored and the `int`-fit check made explicit, because C's `ckd_mul`
   multiplies in the *destination's* type (`docs/ports/token-aliases.tsv`'s ckd_mul family,
   PORTING.md P-15).
10. **`attributes: const`** on `Fframe_windows_min_size` (C 498) has no counterpart in the
    mirror's `DEFUN!` and only affects optimization, so it is dropped (PORTING.md P-25's
    class); `--check` compares the `noreturn` flag only.
11. **Line drift.**  The port is **5,302 lines against C's 7,907**, i.e. 1.13 % against the
    **5,243 lines this configuration compiles**; the row
    `frame	0.05	…	live` in `docs/ports/line-drift-tolerance.tsv` selects the `live` basis
    (the dead region above is recorded here, not transcribed).  The port is *longer* than the
    live text by the `// femacs:` note, the `#![allow]`/include blocks, the use-site
    declarations (one `/// C:` line each), the 51-entry `frame_parms` table and the 26
    `DEFVAR_LISP`/`DEFVAR_BOOL` carriers, and *shorter* by the 66 docstrings.

### D7 call-relationship check

Both sides' "identifier followed by `(`" sets (comments and literals stripped, the C side
restricted to the live arms, macro `!` ignored) were subtracted.  **C-only: 8**, every one a
recorded spelling: `ckd_mul` (the alias family's canonical name for `overflowing_mul`),
`kvar` and `symbol_index` (the two token-pasting macros, `excluded-macros.txt`), `switch`,
`for`, `while`, `else` (the keywords `match` and the labelled/explicit loops replace) and
`defined` (a preprocessor directive inside the live stream, not a call).  **Rust-only: 140**,
all of them counterparts: the `Q*` Lisp-symbol *functions* (`P-17`: C's `Qname` is a constant,
the port's is `Qname ()`), the flag-mask macros and their `set_frame_flag1!`/`set_frame_flag2!`/
`set_output_method!` helpers (§3 above, C writes `f->field` or the `FRAME_*` macro), the hook
values the port calls through `if let Some (hook) = … { hook (…) }` where C writes
`(*FRAME_TERMINAL (f)->hook) (f)` (the field name is not followed by `(` in C's text), and
Rust/mirror spellings (`let`, `unsafe`, `include`, `as_ptr`, `is_null`, `is_some`, `null_mut`,
`addr_of`, `offset_of`, `zeroed`, `lisp_h_Qnil`, `check_type`, `is_daemon`, `fset_redisplay`,
`Some`).  No C callee is missing and the port adds no callee C does not name.

### Use-site declarations (headers without a mirror; each keeps C's prototype in `/// C:`)

`is_tty_frame`, `is_tty_child_frame`, `is_tty_root_frame`, `change_frame_size`,
`set_window_buffer`, `make_window`, `other_buffer_safely`, `get_minibuffer`,
`adjust_frame_glyphs`, `calculate_costs`, `resize_frame_windows`, `create_tty_output`,
`init_tty`, `get_tty_size`, `max_child_z_order`, `init_frame_faces`, `init_initial_terminal`,
`decode_live_terminal`, `store_in_alist` (this file's own; declared before its definition),
`window_box`, `pixel_to_glyph_coords`, `tty_raise_lower_frame`, `update_face_from_frame_parameter`,
`tty_color_name`, `mru_rooted_frame`, `free_glyphs`, `delete_all_child_windows`,
`delq_no_quit`, `free_font_driver_list`, `swallow_events`, `not_single_kboard_state`,
`resize_mini_window`, `move_minibuffers_onto_frame`, `fileno`, `hashtest_eq`,
`unspecified_fg`/`unspecified_bg`, and the `frame_parm_table`/`internal_border_part` types and
the `FACE_TTY_DEFAULT_*` constants (dispextern.h 1932-1940).

### Integrator note (mirror gaps this unit reports and does not touch)

1. **`include/frame.rs` (frame.h's mirror) is read-only WIP**: it carries the readers as
   functions and the flag run as `bits1`/`bits2` masks but no l-value macros, which is why
   §3 above defines ~40 macros at the use site.  When the mirror grows them, this unit's
   copies must be deleted in the same commit (the search.rs precedent).
2. **`include/frame.rs`'s crate-root declarations** of the seven file-scope objects and of
   `frame_make_pointer_invisible` are why `frame_globals`/`frame_pointer_fns` exist; moving
   those declarations into a `pub mod … ; use …::*;` (P-21) would let the definitions return
   to the crate root.
3. **`include/window.rs`** says `WINDOW_XFRAME` (window.h 1358) "lands with frame.rs"; it is
   defined here at the use site, and `fset_redisplay`'s mirror declaration takes
   `*mut c_void` where frame.h's prototype takes `struct frame *` (the port casts).
4. **Missing mirrors**: `systty.h`/`<stdio.h>`'s `fileno`, `frame.h`'s `CHECK_FRAME`/
   `CHECK_LIVE_FRAME`, `SET_FRAME_*` writers, `FRAME_INNER_*`, the pixel/text conversions,
   `FRAME_TEXT_COLS_TO_PIXEL_WIDTH`/`FRAME_PIXEL_HEIGHT_TO_TEXT_LINES`,
   `WINDOW_FRAME`/`WINDOW_XFRAME`, `lisp.h`'s `hashtest_eq`, `xfaces.c`'s `unspecified_fg`/
   `unspecified_bg` arrays, `dispextern.h`'s `internal_border_part` enum and
   `FACE_TTY_DEFAULT_*`.
5. **`docs/ports/line-drift-tolerance.tsv`** now carries the `frame … live` row;
   `docs/baseline/undef-pending.txt` gained this unit's 19 dependencies (dispnew.c, window.c,
   fns.c, keyboard.c, xfaces.c — all in `frozen-build-sources.txt`, none ported yet), of which
   two (`_is_tty_child_frame`, `_mru_rooted_frame`) a later `--write-pending` from another lane
   pruned once the reference objects defining them were built.
6. An intermediate run of `check-undef.py`, taken while this unit's `frame.o` was fresh but
   `font.o` (another lane's in-flight unit) still referenced nine names, reported `9 NEW`
   (`_Vdisplay_pixels_per_inch`, the three `_Vface_*`, `_assoc_no_quit`,
   `_face_at_buffer_position`, `_face_at_string_position`, `_lookup_basic_face`,
   `_recompute_basic_faces`) — none referenced by `frame.o`, and they are gone from the
   final measurement above; this unit did not absorb them into the ratchet.
