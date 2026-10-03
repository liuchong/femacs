# emacs/src/menu.c (1638 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 3
- [x] menu-bar-menu-at-x-y  (Fmenu_bar_menu_at_x_y, 2/3, doc 6b58a5118997, line 1049)
- [x] x-popup-menu  (Fx_popup_menu, 2/2, doc 5e36404a6b3e, line 1433)
- [x] x-popup-dialog  (Fx_popup_dialog, 2/3, doc 1b855d18935a, line 1515)

## Functions defined — 22
- [x] have_boxes  (line 48, static)
- [x] init_menu_items  (line 78)
- [x] finish_menu_items  (line 98)
- [x] unuse_menu_items  (line 103)
- [x] discard_menu_items  (line 112)
- [x] restore_menu_items  (line 128, static)
- [x] save_menu_items  (line 145)
- [x] ensure_menu_items  (line 160, static)
- [x] push_left_right_boundary  (line 199, static)
- [x] push_menu_pane  (line 210, static)
- [x] push_menu_item  (line 232, static)
- [x] single_keymap_panes  (line 267, static)
- [x] single_menu_item  (line 314, static)
- [x] keymap_panes  (line 440, static)
- [x] encode_menu_string  (line 458, static)
- [x] list_of_items  (line 469, static)
- [x] list_of_panes  (line 496)
- [x] parse_single_submenu  (line 521)
- [x] menu_item_width  (line 1035)
- [x] x_popup_menu_1  (line 1111)
- [x] emulate_dialog_with_menu  (line 1487, static)
- [x] syms_of_menu  (line 1614)

## Macros defined — 0

## Types and their fields — 1
- [x] skp: Lisp_Object pending_maps, int maxdepth, int notbuttons

## File-scope objects — 6
- [x] Lisp_Object menu_items  (line 58)
- [x] bool menu_items_inuse  (line 62)
- [x] int menu_items_allocated  (line 65)
- [x] int menu_items_used  (line 68)
- [x] int menu_items_n_panes  (line 72)
- [x] static int menu_items_submenu_depth  (line 75, static)

## Decisions

`emacs/src/menu.c` @ `fd7afced403`; port `femacs/src/menu.rs` (1209 lines vs C's 1638, of which
**1072** are live - the 12.9% drift is declared in `docs/ports/line-drift-tolerance.tsv` with
the `live` basis, because this port records the dead region instead of carrying it).  The gates
are `python3 tools/extract-abi.py --check emacs/src/menu.c femacs/src/menu.rs` -> `OK` and
`build-tu src/menu.c` -> `built`; the two token rows this port needs
(`have_boxes`' `0`, `syms_of_menu`'s `doc`) are in `docs/ports/macro-body-notes.tsv`.

### Dead text (not transcribed)

This target has no window system at all: `HAVE_WINDOW_SYSTEM`, `HAVE_X_WINDOWS`, `HAVE_NTGUI`,
`HAVE_NS`, `HAVE_PGTK`, `HAVE_HAIKU`, `HAVE_ANDROID`, `USE_GTK`, `USE_X_TOOLKIT`, `USE_LUCID`
and `MSDOS` are all `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, and lisp.h
3929-3932 defines `HAVE_EXT_MENU_BAR` only under the first five of those, so it is undefined
too.  567 of C's 1638 lines are dead; each region is named here instead:

- C 36-37: `#ifdef HAVE_WINDOW_SYSTEM`'s `#include TERM_HEADER` (the one include whose header
  is the window system's).
- C 40-41: `#ifdef HAVE_NTGUI`'s `extern AppendMenuW_Proc unicode_append_menu;`.
- C 50-53: `have_boxes`' window-system arm (`FRAME_WINDOW_P (XFRAME (Vmenu_updating_frame))` ->
  `return 1`); C 55's `return 0` is the live text.
- C 170-193: `#if defined HAVE_EXT_MENU_BAR || defined HAVE_ANDROID` - `push_submenu_start`
  (C 175) and `push_submenu_end` (C 186).
- C 422-432: `single_menu_item`'s toolkit submenu (`push_submenu_start` /
  `single_keymap_panes` / `push_submenu_end`).
- C 566-873: the `widget_value` tree - `make_widget_value` (571),
  `free_menubar_widget_value_tree` (593), `digest_single_submenu` (619) and
  `update_submenu_strings` (846) - which needs lwlib-widget.h's `widget_value` (no mirror; see
  `femacs/src/include/menu.rs`).  C 694/781's `#elif !defined (HAVE_MULTILINGUAL_MENU)` arms
  are inside it.
- C 875-964: `find_and_call_menu_selection` (884), the same guard plus `HAVE_HAIKU`.
- C 967-1031: `find_and_return_menu_selection` (971), `#ifdef HAVE_NS`.
- C 1193-1209: `x_popup_menu_1`'s `#ifdef HAVE_X_WINDOWS` arm (`FRAME_X_P` and
  `x_relative_mouse_position`); the block after it is C's live text.
- C 1307-1312: `#if defined (USE_GTK) || defined (HAVE_NS)` around the `Qhide` text-property
  test of the keymap prompt.
- C 1371-1374: `#ifdef HAVE_WINDOW_SYSTEM`'s `Fx_hide_tip ()`.
- C 1377-1388: `#ifdef HAVE_NTGUI`'s `current_popup_menu` block.
- C 1395-1403: `#ifdef HAVE_WINDOW_SYSTEM`'s `cancel_hourglass ()`.
- C 1420-1426: `#ifdef HAVE_NTGUI`'s `FRAME_DISPLAY_INFO (f)->grabbed = 0;` (the condition whose
  trailing comment the `#if` evaluator could not parse until this round; see below).
- C 1600-1605: `Fx_popup_dialog`'s `#ifdef HAVE_NTGUI` arm (the `unsupported--w32-dialog` test).

`menu.h`'s own dead declarations live in `docs/ports/include-menu.md`: the
`HAVE_EXT_MENU_BAR` block, the `HAVE_X_WINDOWS || MSDOS` pair, and the `HAVE_NTGUI`, `HAVE_NS`
and `HAVE_PGTK` pairs.

### Deviations recorded

1. **C 262-263's forward declaration** of `single_menu_item` is deleted; the definition keeps
   its C place (PORTING.md 0.1-12).
2. **C's uninitialized locals are zero-initialized** (PORTING.md 1.5), because Rust needs a
   value to take their address: `struct skp skp;` (C 269, C assigns all three fields right
   after) is `std::mem::zeroed ()`; `int row, col;` (C 1055) are `0`; `int ch_len` (C 1038) is
   `0`; `Lisp_Object bar_window; enum scroll_bar_part part; Time time;` (C 1198-1200, written by
   the mouse-position hook) are `Qnil`/`0`/`0`.
3. **The `menu_show_hook` call** (C 1412) is `if let Some (hook) = ... { selection = hook (…) }`:
   C calls the hook unconditionally, the port's `Option` carries the same call and skips it
   only for the NULL hook C would have dereferenced (`docs/ports/terminal.md`'s shape).
4. **`MATRIX_HEADER_LINE_HEIGHT`/`MATRIX_TAB_LINE_HEIGHT`** (use-site macros, dispextern.h
   1529/1538) bind their matrix argument once, while C reads `(W)->current_matrix` three times
   through `MATRIX_*_LINE_ROW`; both call sites pass exactly that field, so the value is the
   same (the `itree` `t` precedent, PORTING.md P-33).
5. **`have_boxes`' `return 0`** for a `bool` function is `return false`, and **`syms_of_menu`'s
   `doc:` label** has no counterpart because the mirror's `DEFVAR_LISP!` takes the docstring
   positionally; both are in `docs/ports/macro-body-notes.tsv` (the `syms_of_casefiddle` /
   `syms_of_data` precedents), and the docstrings themselves are still compared.
6. **Integer narrowing**: C assigns `ptrdiff_t`/`EMACS_INT` into `int` fields and adds an
   `intmax_t` to an `int` (`menu_items_allocated = ASIZE (menu_items)`,
   `xpos += check_integer_range (…)`), which the port spells with explicit `as c_int` /
   `as intmax_t` casts.
7. **`r#type`** is C's `type` parameter of `push_menu_item` (C 232) and C's local of the same
   name in `single_menu_item` (C 339), per PORTING.md 0.2's keyword list.

### Names written at their use site

The mirrors this unit includes do not carry these, so each is declared where it is called
(the terminal.rs/scroll.rs shape), with C's own body for the macros:
`larger_vector` (lisp.h 4245), `record_unwind_protect_void` (lisp.h 4825), `parse_menu_item`
(keyboard.h 474), `init_raw_keybuf_count` (keyboard.h 476), `pixel_to_glyph_coords`
(dispextern.h 3584, its `NativeRectangle *` parameter spelled `*mut c_void` per P-55),
`window_box_left` (dispextern.h 3579), `is_tty_frame` (dispextern.h 4012), `decode_any_frame`
(frame.h 1477), `window_wants_header_line`/`window_wants_tab_line` (window.h 1214/1215),
`estimate_mode_line_height` (dispextern.h 3582), and the macros `WINDOW_FRAME` (frame.h 929),
`WINDOW_XFRAME` (window.h 620), `WINDOW_LEFT_EDGE_X` (window.h 758), `WINDOW_TOP_EDGE_Y`
(window.h 797), `WINDOW_HEADER_LINE_HEIGHT` (window.h 1030), `WINDOW_TAB_LINE_HEIGHT`
(window.h 1040), `MATRIX_HEADER_LINE_HEIGHT`/`MATRIX_TAB_LINE_HEIGHT` (dispextern.h
1529/1538), `CURRENT_HEADER_LINE_HEIGHT` (dispextern.h 1616), `CURRENT_TAB_LINE_HEIGHT`
(dispextern.h 1648) and their two `..._ACTIVE_FACE_ID` helpers (dispextern.h 1594/1624).
`selected_window` (window.h 1100) and the five `menu_items*` objects keyboard.h 344-358
declares come from the mirrors instead (see the next item).

### Changes outside the unit (for integrator review)

1. `tools/cinventory.py`: `_dead_branch_scan` now strips a comment a `#if` line leaves **open**
   (C's translation phase 3 removes it before the directive is read).  C 1420's
   `#ifdef HAVE_NTGUI /* W32 specific because other terminals clear … */` spans five physical
   lines, so the condition answered `unknown` and kept that branch live although clang
   compiles it as dead - the one disagreement that kept `src/menu.c` out of
   `docs/baseline/config/cond-eval-verified.txt`, whose absence would have made this port
   demand the whole window-system half of the file (and its line-count gate the full 1638
   lines).  With the fix `python3 tools/check-cond-eval.py src/menu.c` reports
   `21 conditions, 0 unknown, 0 disagreements [VERIFIED]`, and a full
   `--verified docs/baseline/config/cond-eval-verified.txt` run reports
   `5696 conditions, 4 unknown, 0 disagreements` with the rewritten list byte-identical to the
   committed one - the change moves no other file's verdicts (it can only turn an `unknown`
   into a decided one, and only for a condition whose logical line holds an unterminated
   `/*`).  `src/menu.c` is added to that list.
2. `femacs/src/include/keyboard.rs`: keyboard.h 344-358's five `menu_items*` objects are
   `src/menu.c`'s own definitions, so they moved into `pub mod keyboard_objs` + glob import
   (the globals.rs `variables` shape, PORTING.md 0.1-10/P-21) - otherwise this unit's
   `#[unsafe(no_mangle)] pub static mut` definitions collide with the mirror's declarations
   (E0428).  `--check emacs/src/keyboard.h femacs/src/include/keyboard.rs` is still `OK`.
