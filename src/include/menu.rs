// femacs: mirror of src/menu.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): types, constants, macros and the declarations C spells
// `extern`.  Nothing is defined here, so any unit may include it.  The declarations of the
// functions `src/menu.c` itself defines sit in `pub mod menu_decls` + glob import, so that
// unit's own definitions shadow the glob instead of colliding with it (PORTING.md 0.1-10,
// the `stdio.rs`/`time_r.rs` shape).
//
// `menu.h`'s live text for this target is the include guard, the three `MENU_*` bits of
// terminal-specific `menu_show_hook`s and the nine declarations below.  Everything else is
// dead: `HAVE_EXT_MENU_BAR` is lisp.h 3929-3932's conditional define and is undefined here
// (no USE_X_TOOLKIT/USE_GTK/HAVE_NTGUI/HAVE_NS/HAVE_HAIKU), and so are `HAVE_X_WINDOWS`,
// `MSDOS`, `HAVE_NTGUI`, `HAVE_NS` and `HAVE_PGTK` (docs/baseline/config/
// darwin-arm64.macros.tsv).  The dead declarations are `free_menubar_widget_value_tree`,
// `update_submenu_strings`, `find_and_call_menu_selection`, `make_widget_value`,
// `digest_single_submenu` (the `HAVE_EXT_MENU_BAR` block), `x_menu_show`/`x_activate_menubar`
// (`HAVE_X_WINDOWS || MSDOS`), `w32_menu_show`/`w32_activate_menubar` (`HAVE_NTGUI`),
// `ns_menu_show`/`ns_activate_menubar` (`HAVE_NS`) and `pgtk_menu_show`/
// `pgtk_activate_menubar` (`HAVE_PGTK`); they are recorded in docs/ports/menu.md.
//
// `menu.h` 22's `#include "../lwlib/lwlib-widget.h"` declares `widget_value`, which only the
// dead arms above name, so it has no mirror and is left out (the atimer.rs precedent).

// menu.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const MENU_H: bool = true;

/* Bit fields used by terminal-specific menu_show_hook.  */

pub const MENU_KEYMAPS: c_int = 0x1;
pub const MENU_FOR_CLICK: c_int = 0x2;
pub const MENU_KBD_NAVIGATION: c_int = 0x4;

pub mod menu_decls {
  use super::*;
  unsafe extern "C" {
    /// C: extern void init_menu_items (void);
    pub fn init_menu_items ();
    /// C: extern void finish_menu_items (void);
    pub fn finish_menu_items ();
    /// C: extern void discard_menu_items (void);
    pub fn discard_menu_items ();
    /// C: extern void save_menu_items (void);
    pub fn save_menu_items ();
    /// C: extern bool parse_single_submenu (Lisp_Object, Lisp_Object, Lisp_Object);
    pub fn parse_single_submenu (item_key: Lisp_Object, item_name: Lisp_Object,
				 maps: Lisp_Object) -> bool;
    /// C: extern void list_of_panes (Lisp_Object);
    pub fn list_of_panes (menu: Lisp_Object);
    /// C: extern Lisp_Object tty_menu_show (struct frame *, int, int, int, Lisp_Object,
    ///                                      const char **);  `struct frame` is a pointer
    /// parameter of a header that cannot name it (P-55/P-72 rule ③).
    pub fn tty_menu_show (f: *mut c_void, x: c_int, y: c_int, menuflags: c_int,
			  title: Lisp_Object, error_name: *mut *const c_char) -> Lisp_Object;
    /// C: extern ptrdiff_t menu_item_width (const unsigned char *);
    pub fn menu_item_width (str_: *const u8) -> ptrdiff_t;
    /// C: extern Lisp_Object x_popup_menu_1 (Lisp_Object position, Lisp_Object menu);
    pub fn x_popup_menu_1 (position: Lisp_Object, menu: Lisp_Object) -> Lisp_Object;
  }
}
pub use self::menu_decls::*;
