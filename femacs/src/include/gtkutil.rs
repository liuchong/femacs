// femacs: port of src/gtkutil.h @ fd7afced403.
//
// Header mirror of the dead-by-construction class (the include/gnutls.rs and
// include/arg-nonnull.rs shape): `USE_GTK` is `undef` in the frozen configuration
// (docs/baseline/config/darwin-arm64.macros.tsv), so C 24-244 - the header's whole body - is not
// compiled, and P-08 records it instead of transcribing it.  `tools/check-cond-eval.py
// src/gtkutil.h` reports `10 conditions, 0 unknown, 0 disagreements [VERIFIED]`: its nine inner
// conditions (`HAVE_PGTK` four times, `USE_CAIRO`, `HAVE_GTK3 && HAVE_XINPUT2`, and the two
// `#else` arms) are all decided, and the outer `#ifdef USE_GTK` (C 24) is false, so none of them
// is reached.  The live inventory is the guard alone - `tools/extract-abi.py` reports 1 macro /
// 0 types / 0 objects - which is what this mirror carries.
//
// **C 26's `<gtk/gtk.h>` edge is still owed, and this round measured the whole blocker set**
// (P-86 ⑧'s edge rule, P-90 ②③): it would name the surface's single home, `include/gtk.rs`, under
// C 24's `USE_GTK`, and its four consumers are `src/gtkutil.rs` at C 44, `src/pgtkfns.rs` at C 28,
// `src/xwidget.rs` at C 30 and `src/xfns.rs` at C 63.  Measured with the edge in place
// (`check-mirror-dupes.py`: 35 findings that no ratchet row covers; the `E0428`s its item-namespace
// names would add are more): `src/pgtkfns.rs` 13 + `src/xwidget.rs` 21 + `src/xfns.rs` 1 tracked
// names, and `src/pgtkfns.rs`/`src/xwidget.rs` move their copies through their own stand-in edges
// this round (the `src/xwidget.rs` C 30 stand-in shape).  `src/xfns.rs` is the blocker that does not
// close from this round's scope: its `GtkWidgetClass` carries `style_updated`, which
// `include/gtk.rs`'s record does not have yet (it carries `get_preferred_width`, gtkutil.c's
// member), so the edge needs a declaration *inside* that mirror - and this round's write scope is
// this file plus the three consumers, not the surface.  `## Owed` of
// `docs/ports/include-gtkutil.md` carries the reading and the three-step fix; the mirror's own
// in-flight change (the `src/xgselect.rs` slice's four `gdk_x11_*` gates, status 3.477) landed at
// `bbb378d` while this round ran, so the next round may write it.
//
// What the rest of the dead region holds, named so the ledger can be re-derived from the
// reference: the five scroll-bar bounds macros `XG_SB_MIN`/`XG_SB_MAX`/`XG_SB_RANGE`/`YG_SB_MIN`/
// `YG_SB_MAX`/`YG_SB_RANGE` (C 38-43), the two data keys `XG_FRAME_DATA`/`XG_ITEM_DATA` (C 46/49),
// the `EVENT` alias (C 30/33) and the `lwlib-widget.h` include (C 27), whose `GCallback`,
// `gulong`, `gpointer` and `GtkWidget` are what make the three callback records
// `xg_list_node` (C 52-56), `xg_menu_cb_data` (C 63-72) and `xg_menu_item_cb_data` (C 75-84)
// unusable here; the whole prototype block (C 86-230); and the two flags
// `xg_ignore_gtk_scrollbar`/`xg_gtk_initialized` (C 240/242).  Eight prototypes are
// `HAVE_PGTK`-dependent and would need that arm chosen by a PGTK configuration
// (`xg_display_open`/`xg_display_close`/`xg_create_default_cursor` at C 171-179, again the
// tooltip one at C 211-215, `xg_create_frame_outer_widgets` with the three dark-mode helpers at
// C 187-192, `xg_set_frame_icon` at C 194-198 and `xg_filter_key` at C 228-230).
//
// src/gtkutil.c is not in the frozen build (docs/port-map.md 450-451), so no translation unit
// reaches this mirror until a GTK target is frozen; the GTK-side record of what the port still
// owes there lives in docs/ports/include-gtkutil.md.

// gtkutil.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const GTKUTIL_H: bool = true;

/* C 26-34: `#include <gtk/gtk.h>` (C 26), `#include "../lwlib/lwlib-widget.h"` (C 27),
   `#ifdef HAVE_PGTK` (C 28) / `#include "pgtkterm.h"` (C 29) / `#define EVENT GdkEvent` (C 30) /
   `#else` (C 31) / `#include "xterm.h"` (C 32) / `#define EVENT XEvent` (C 33) / `#endif` (C 34).
   All four edges are **owed** and their text stands in the consumers at the position that makes
   the names visible (`src/pgtkfns.rs` and `src/xwidget.rs` for C 26 and the two terminal headers,
   each with the reason written beside it; `src/gtkutil.rs` 173-176 and `src/xfns.rs` 327-341 for
   the two they declare).  C 26's blocker set is the head note above; C 27's `lwlib-widget.h` has
   no mirror in this tree at all.  `## Owed` of `docs/ports/include-gtkutil.md` carries all four.  */
