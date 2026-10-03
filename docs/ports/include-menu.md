# emacs/src/menu.h (72 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] MENU_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/menu.h` @ `fd7afced403`; mirror `femacs/src/include/menu.rs` (60 lines vs C's 72).
The gate is `python3 tools/extract-abi.py --check emacs/src/menu.h
femacs/src/include/menu.rs` -> `OK`; nothing here is defined, so any unit may include it.

`menu.h`'s live text for this target is the include guard, the three `MENU_*` bits
(`enum { MENU_KEYMAPS = 0x1, MENU_FOR_CLICK = 0x2, MENU_KBD_NAVIGATION = 0x4 }`), the six
unconditional declarations, `tty_menu_show`, `menu_item_width`, `x_popup_menu_1`, and the
`#include "../lwlib/lwlib-widget.h"` that names nothing live.

### Dead text (not transcribed)

`HAVE_EXT_MENU_BAR` is lisp.h 3929-3932's conditional define and is undefined here (no
USE_X_TOOLKIT/USE_GTK/HAVE_NTGUI/HAVE_NS/HAVE_HAIKU), and `HAVE_X_WINDOWS`, `MSDOS`,
`HAVE_NTGUI`, `HAVE_NS` and `HAVE_PGTK` are undefined in
`docs/baseline/config/darwin-arm64.macros.tsv`, so these declarations are not carried:

- C 41-48: the `HAVE_EXT_MENU_BAR` block - `free_menubar_widget_value_tree` (41),
  `update_submenu_strings` (42), `find_and_call_menu_selection` (43-44), `make_widget_value`
  (45) and `digest_single_submenu` (46).  They take `widget_value *`, which is
  `../lwlib/lwlib-widget.h`'s and has no mirror (the atimer.rs precedent: a header whose names
  the live text never needs is left out and recorded).
- C 51-53: `#if defined (HAVE_X_WINDOWS) || defined (MSDOS)` - `x_menu_show`,
  `x_activate_menubar`.
- C 56-60: `#ifdef HAVE_NTGUI` - `w32_menu_show`, `w32_activate_menubar`.
- C 61-65: `#ifdef HAVE_NS` - `ns_menu_show`, `ns_activate_menubar`.
- C 66-70: `#ifdef HAVE_PGTK` - `pgtk_menu_show`, `pgtk_activate_menubar`.

### Deviations recorded

The five declarations of functions `src/menu.c` itself defines (`init_menu_items`,
`finish_menu_items`, `discard_menu_items`, `save_menu_items`, `parse_single_submenu`) sit in
`pub mod menu_decls` + glob import, so that unit's definitions shadow the glob instead of
colliding with it (PORTING.md 0.1-10/P-21, the `stdio.rs`/`time_r.rs` shape).  `tty_menu_show`'s
`struct frame *` is spelled `*mut c_void` with the `/// C:` line above it (PORTING.md 0.1-2/P-55);
its `const char **` error-name out-parameter is the same in both languages.
