# emacs/src/gtkutil.c (6753 lines) — slice 1: C 1-1296; slice 2: C 1297-2281

> **One of P-86's class: the frozen configuration compiles nothing here.**  `gtkutil.o` joins
> `GTK_OBJ` only in the `USE_GTK` branch of the build (`configure.ac` 3827's
> `GTK_OBJ="gtkutil.o $GTK_OBJ"`, under the arm whose 3825 line is
> `AC_DEFINE ([USE_GTK], [1], [Define to 1 if using GTK.])` - the macro this file's own
> `#ifdef USE_GTK` (C 22) tests; `emacs/src/Makefile.in` 182's `GTK_OBJ=@GTK_OBJ@` and 452's
> `$(GTK_OBJ)` in the object list), `USE_GTK` is `undef` in
> `docs/baseline/config/darwin-arm64.macros.tsv` 3861, and `docs/baseline/frozen-build-sources.txt`
> lists no `gtkutil.o` for this target - so the text can never be *compiled* here.  Note the
> difference from the class P-86 was written for: this file's conditions are all *decided*
> (`python3 tools/check-cond-eval.py src/gtkutil.c` -> `212 conditions, 1 unknown, 1 disagreements
> [NOT VERIFIED]`; the unknown and the disagreement are the same line, C 6652's
> `#elif GTK_CHECK_VERSION (3, 16, 0)`, six times past this slice - `docs/status.md` 6868 records
> the same 212/1/1 reading for this file), so `tools/extract-abi.py`'s checklist is **not** empty:
> `python3 tools/extract-abi.py emacs/src/gtkutil.c` reads `## Functions defined — 148`, `## Macros
> defined — 19`, `## Types and their fields — 4`, `## File-scope objects — 9`, `## Subrs (DEFUN) —
> 0`, and the `--check` reading in Evidence ③ is not vacuous.  What this ledger adds is P-86 ②'s
> substitute shape - the **C line-range table**, accounting for every line of the reference - beside
> the `## Evidence` readings and the `## Deviations`.
>
> **This ledger replaces, not extends, what it said before.**  Until this batch it read "the frozen
> configuration does not compile this file ⇒ the port is the empty unit with this note" (the P-08
> reading, `docs/status.md` 3.31), which is what the 11-line placeholder was.  P-86 ① replaces that
> with *transcribe the text, both arms, no stub*; what the earlier note measured (not in the frozen
> source list, empty checkbox sections, 336-byte object) is unchanged and still true here - it is
> the *conclusion* that is withdrawn, exactly as `docs/ports/pgtkterm.md`, `docs/ports/w32.md` and
> `docs/ports/androidvfs.md` record for their own files.  Port: `femacs/src/gtkutil.rs`
> (**4200 lines**, from the 11 of the placeholder), carrying **C 1-1296** and **C 1297-2281** with
> **both arms of every `#if`**.
>
> **Slice 1 is C 1-1296**, from the file's first line through `xg_frame_set_char_size`'s closing
> brace (C 1176-1296): the include list and this file's own declarations (C 20-146), the
> `EmacsMenuBar` GTK3 type and its four functions, the display functions (`xg_set_screen`,
> `xg_display_open`, `xg_get_gdk_scale`, `xg_get_scale`, `xg_display_close`,
> `xg_create_default_cursor`), the pixbuf/image helpers (`xg_get_pixbuf_from_pix_and_mask`,
> `xg_get_pixbuf_from_surface`, `file_for_image`, `xg_get_image_for_pixmap`), the two list and the
> utf8 helpers, `xg_check_special_colors`, the whole Tooltips section (both the X arm's
> `hierarchy_ch_cb`/`qttip_cb`/`xg_prepare_tooltip`/`xg_show_tooltip`/`xg_hide_tooltip` and the pgtk
> arm's two entry points), `my_log_handler`, `xg_is_menu_window`, `xg_set_geometry`,
> `xg_frame_resized` and `xg_frame_set_char_size`.
>
> **Slice 2 is C 1297-2281**, from the blank after slice 1's last brace, through the `#if 0` block it
> opens with, to `xg_set_frame_icon`'s `#endif`: C 1298-1300's comment and C 1301-1311's `#if 0`
> block (`xg_height_or_width_changed`, dead text C itself excludes, recorded as a comment - P-86 ①),
> then `xg_frame_set_size_and_position` (C 1316-1391), `xg_win_to_widget` (C 1401-1420, under
> C 1393's `#ifndef HAVE_PGTK`), `xg_set_widget_bg` (C 1426-1458), the dark-mode trio (C 1464-1502,
> inside `#if defined HAVE_PGTK && defined HAVE_GSETTINGS`), `style_changed_cb` (C 1509-1559),
> `delete_cb` (C 1564-1570), `xg_create_frame_widgets` (C 1577-1831), `xg_create_frame_outer_widgets`
> (C 1835-1932, under `#ifdef HAVE_PGTK`), `xg_free_frame_widgets` (C 1936-1976),
> `xg_wm_set_size_hint` (C 1985-2108), `xg_set_background_color` (C 2117-2138) and the seven
> frame-parameter setters (C 2143-2277).  **The next live definition is `get_dialog_title` (C 2288)**
> - the dialog section C 2282's banner opens; C 2282-6753 is the table's last row.  This slice
> *calls* names the remainder carries (`xg_get_widget_from_map` at C 2130,
> `update_theme_scrollbar_width`/`_height` at C 1533-1534, `xg_widget_style_updated` at C 1613 and the
> two `xg_im_context_*` callbacks at C 1796-1806, `initial_set_up_x_back_buffer` at C 1752,
> `x_wm_supports` at C 2008-2009), and every one of them shows as a "cannot find ..." arm reading
> (Evidence ②) - the `docs/ports/pgtkterm.md` Deviations 3 shape.
>
> **The file gate is `feature = "use_gtk"`** (`USE_GTK`, `configure.ac` 3825's `AC_DEFINE`), on every
> item *and on the include graph* (P-86 ⑧ - an `include!` outside the gate breaks the frozen build at
> once, which is why the reading in Evidence ① is a 336-byte empty object).  The slice's own
> conditions are C 33's `#ifndef HAVE_PGTK` (feature `have_pgtk`), C 51's `#ifdef HAVE_XINPUT2`, C
> 55's `#ifdef HAVE_XFT`, C 59-64's `#ifdef HAVE_GTK3` pair, C 66's `#ifdef HAVE_XDBE`, C 108's
> `#ifndef HAVE_GTK3`, C 110's `#ifdef HAVE_FREETYPE`, C 142's `#ifdef HAVE_XWIDGETS`, C 204's
> `#ifndef HAVE_PGTK`, C 237/246/259's three arm-split declarators, C 270/287/290/300/306/331/366/372's
> `HAVE_GTK3`/`HAVE_PGTK` arms, C 428's `#if defined USE_CAIRO && !defined HAVE_GTK3`, C 512/544/552's
> cairo/GTK3 pair, C 759/790/954/1023/1033/1065/1072/1104/1186/1227/1237/1273's arms - all decided by
> the frozen table, and **both arms of every one of them are carried**.
>
> **`GTK_CHECK_VERSION` is decided by the reference's own text, not by a `configure` macro.**
> `xterm.h` 73-75 is `#ifndef USE_GTK` / `#define GTK_CHECK_VERSION(i, j, k) false` / `#endif` - the
> definition the frozen configuration evaluates (`include/xterm.rs` 70-76 mirrors it under
> `not (use_gtk)`), so in the frozen reading C 104's `#if GTK_CHECK_VERSION (3, 10, 0)` is false and
> C 1023/1065/1104's `#if ! GTK_CHECK_VERSION (3, 22, 0)` are true.  A `use_gtk` build takes GTK's
> own `gtk/gtkversion.h` instead, so the port spells each arm with a derived-condition feature after
> the `android_api_le_11`/`android_api_ge_9` naming: **`gtk_check_version_ge_3_10`** and
> **`gtk_check_version_ge_3_22`** - **both are registered in `femacs/xtask/Cargo.toml`** (100/101,
> with those C lines in their comments; slice 1's `## Owed` 1 was closed by a later batch, and slice
> 2 uses both).  The pragma-shaped `#if GTK_CHECK_VERSION (3, 10, 0)` at C 104 gates a forward declaration
> only, which PORTING.md 0.1-12 drops anyway, so its arm is C's text plus that feature name.
>
> **The toolkit surface is declared in this unit (P-88 ①, which P-90 ① keeps for this round), and
> both slices are `text-only` (P-86 ③⑵).**  Slice 2 adds its own names to that surface (the GDK
> event/hint/gravity and event-mask enumerations, `GdkGeometry` and `GdkVisual`, `GtkWindowType`,
> `GtkRcStyle` and the GTK2 `GdkColormap`/`GdkColor` pair, `GtkCssProvider`/`GtkStyleProvider`/`GtkBox`/`GParamSpec`, the
> ~40 entry points it calls, and `G_SIGNAL_MATCH_FUNC`/`g_signal_handler_find`/`g_clear_object` from
> GLib), and its GDK/GTK values are *placeholders* marked `// placeholder, not GDK's value` (the
> `src/xwidget.rs` 495-520 rule) - see `## Owed` 6/9.  Measured on this machine:
> `ls -d /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0` -> both
> `No such file or directory`; `pkg-config` is **not installed**;
> `find /opt/homebrew /usr/local /Library -maxdepth 4 -name gtk.h` (and `-name gdk.h`) -> nothing;
> `/opt/homebrew/include/pango-1.0` and `/opt/homebrew/include/gdk-pixbuf-2.0` -> absent too.  So
> **GDK, GTK, Pango and GdkPixbuf have no tree to compile or measure against**: every citation of
> theirs names the header alone (no line, no probe) and every size, offset and enum width of theirs
> is **UNVERIFIED** in the port - no arm flip is possible and no layout assertion is written for a
> GDK/GTK record.  **GLib/GObject, cairo and X11 *are* present** (`/opt/homebrew/include/glib-2.0`,
> GLib 2.90.0, plus the generated `glibconfig.h` under
> `/opt/homebrew/Cellar/glib/2.90.0/lib/glib-2.0/include`; `/opt/homebrew/include/cairo`, cairo
> 1.18.4; `/opt/homebrew/include/X11`), so their declarations quote the header and line they were
> read from, `GList`/`GError`/`GType`'s layouts are real clang readings (Evidence ②) and the X11
> constants C 1062-1112 tests are quoted from `X11/Xutil.h` and `X11/X.h`.  The missing toolchain is
> named in `## Owed`.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`GTK_OBJ` ⇔ the `USE_GTK` `AC_DEFINE`) | `feature = "use_gtk"` (on every item *and* on C 23-68's include graph) | a GTK build (X11-GTK or pgtk) |
| C 33 `#ifndef HAVE_PGTK` (C 34-36 vs C 38-40) | `all (use_gtk, not (have_pgtk))` / `all (use_gtk, have_pgtk)` | an X11-GTK build / a pure GTK (pgtk) build |
| C 51 `#ifdef HAVE_XINPUT2` (`#include <X11/extensions/XInput2.h>`) | `have_xinput2` (named where C asks: C 1033's `xg_is_menu_window`) | a GTK build with XInput2 |
| C 55 `#ifdef HAVE_XFT` (`#include <X11/Xft/Xft.h>`) | `have_xft` (nothing in this slice is taken from Xft) | a GTK build with Xft |
| C 59-64 `#ifdef HAVE_GTK3` / `#ifndef HAVE_PGTK` / `#include <gtk/gtkx.h>` / `#include "emacsgtkfixed.h"` | `all (use_gtk, have_gtk3, not (have_pgtk))` for `gtkx.h`; `all (use_gtk, have_gtk3)` for `include/emacsgtkfixed.rs` | a GTK3 build |
| C 66 `#ifdef HAVE_XDBE` (`#include <X11/extensions/Xdbe.h>`) | `have_xdbe` (nothing in this slice is taken from Xdbe) | a GTK build with Xdbe |
| C 70-78 `#ifdef HAVE_GTK3` / the three `XG_TEXT_*` ids / `#else` / GTK2's three stock ids | `all (use_gtk, have_gtk3)` / `all (use_gtk, not (have_gtk3))` | a GTK3 build / a GTK2 build |
| C 80-95 `#ifdef HAVE_GTK3` (the `EmacsMenuBar` type and `G_DEFINE_TYPE`) | `all (use_gtk, have_gtk3)` | a GTK3 build |
| C 97-102 / C 146 `#ifndef HAVE_PGTK` (forward declarations) | `all (use_gtk, not (have_pgtk))`; the declarations are dropped (PORTING.md 0.1-12) and the definitions are past this slice | an X11-GTK build |
| C 104 `#if GTK_CHECK_VERSION (3, 10, 0)` (one forward declaration) | `gtk_check_version_ge_3_10` - a feature name this tree has not registered (Owed 1) | a GTK >= 3.10 build |
| C 108-125 `#ifndef HAVE_GTK3` (five GTK2 compatibility macros; C 110-117 is `#ifdef HAVE_FREETYPE`) | `all (use_gtk, not (have_gtk3))` (+ `have_freetype` inside) | a GTK2 build |
| C 142 `#ifdef HAVE_XWIDGETS` (`bool xg_gtk_initialized;`) | `all (use_gtk, have_xwidgets)` | a GTK build with xwidgets |
| C 204 / C 237-241 / C 246-250 / C 259-263 (`xg_set_screen`, `xg_display_open`'s and `xg_create_default_cursor`'s declarators) | `not (have_pgtk)` / `have_pgtk` | as above |
| C 270 `#ifdef HAVE_GTK3`, C 287 `#ifdef HAVE_PGTK`, C 290 `#ifdef HAVE_GTK3` (`xg_get_gdk_scale`, `xg_get_scale`) | `have_gtk3` / `have_pgtk` / `have_gtk3` | as above |
| C 306-330 `#ifndef HAVE_PGTK` vs C 331-354 (`xg_display_close`'s two bodies) | `not (have_pgtk)` / `have_pgtk` (two `fn`s; Deviation 3) | as above |
| C 372 `#ifndef HAVE_PGTK`, C 378's `#ifndef HAVE_PGTK` (*pixbuf* pair), C 428 `#if defined USE_CAIRO && !defined HAVE_GTK3`, C 512/544/552's cairo/GTK3 pair | `not (have_pgtk)`; `all (not (have_pgtk), use_cairo, not (have_gtk3))` | an X11-GTK2 build without cairo for the pixmap path |
| C 743/768/790 `#ifdef HAVE_GTK3` and `#ifndef HAVE_PGTK` (`xg_check_special_colors`) | `have_gtk3` (+ `not (have_pgtk)` for the parse-color pair) | as above |
| C 818 `#ifndef HAVE_PGTK` / C 954 / C 994 (`#else  /* HAVE_PGTK */`) - the Tooltips section | `not (have_pgtk)` / `have_pgtk` (the pgtk arm's two entry points have different signatures; Deviation 3) | as above |
| C 1023 `#if ! GTK_CHECK_VERSION (3, 22, 0)` (`my_log_handler`) | `not (gtk_check_version_ge_3_22)` - unregistered (Owed 1) | a GTK < 3.22 build |
| C 1033 `#if defined HAVE_GTK3 && defined HAVE_XINPUT2` (`xg_is_menu_window`) | `all (have_gtk3, have_xinput2)` | a GTK3 build with XInput2 |
| C 1065 and C 1104 (`#if ! GTK_CHECK_VERSION (3, 22, 0)` opening `xg_set_geometry`'s `if` and closing its `else`) | `not (gtk_check_version_ge_3_22)` on the arm chooser (Deviation 4) | a GTK < 3.22 build |
| C 1186 `#ifndef HAVE_PGTK` (the two `gtk_window_get_size` call shapes), C 1227's `#ifndef HAVE_PGTK` / `#else`, C 1237's `#ifdef USE_CAIRO`, C 1273's `#ifndef HAVE_PGTK` (`xg_frame_set_char_size`) | `not (have_pgtk)` / `have_pgtk` / `use_cairo` / `not (have_pgtk)` | as above |
| **slice 2 (C 1297-2281).** C 1351/1363's `#ifndef HAVE_PGTK` / `#else`, C 1359's `#ifdef USE_CAIRO` and C 1383's `#ifndef HAVE_PGTK` (`xg_frame_set_size_and_position`) | `not (have_pgtk)` / `have_pgtk` / `use_cairo` | as above |
| C 1393's `#ifndef HAVE_PGTK` (`xg_win_to_widget`), C 1428's `#ifdef HAVE_GTK3` / C 1431's `#ifndef HAVE_PGTK` / C 1452's GTK2 `#else` (`xg_set_widget_bg`), C 1563's `#ifndef HAVE_PGTK` (`delete_cb`) | `not (have_pgtk)` / `have_gtk3` | as above |
| C 1462 `#if defined HAVE_PGTK && defined HAVE_GSETTINGS` (the dark-mode trio) and C 1824's same fold (`xg_create_frame_widgets`), C 1927's `#ifdef HAVE_GSETTINGS` (`xg_create_frame_outer_widgets`) | `all (have_pgtk, have_gsettings)` - `have_gsettings` is **not registered** in `femacs/xtask/Cargo.toml` (Owed 9) | a pgtk build with GSettings |
| C 1516/1522/1545's `#ifndef HAVE_PGTK` (`style_changed_cb`) | `not (have_pgtk)` / `have_pgtk` | as above |
| C 1582's `#ifndef HAVE_GTK3` and C 1632's `#ifdef HAVE_GTK3` (the `GtkRcStyle`/`wfixed` pair), C 1585/1593/1689/1699/1715/1747/1789's `#ifndef HAVE_PGTK`, C 1593's `#else`, C 1611's `#if GTK_CHECK_VERSION (3, 10, 0)`, C 1620's `#if (! defined HAVE_GTK3 && defined HAVE_GTK_WINDOW_SET_HAS_RESIZE_GRIP)`, C 1693's `#if ! GTK_CHECK_VERSION (3, 22, 0)`, C 1725's `#ifdef HAVE_PGTK`, C 1734's `#if !defined HAVE_PGTK` / `#else`, C 1759's `#ifndef HAVE_GTK3` / `#else` (+ C 1771's nested `#ifdef HAVE_PGTK`) (`xg_create_frame_widgets`) | `have_gtk3` / `not (have_gtk3)`; `not (have_pgtk)` / `have_pgtk`; `gtk_check_version_ge_3_10`; `not (gtk_check_version_ge_3_22)`; `have_gtk_window_set_has_resize_grip` **not registered** (Owed 9) | as above |
| C 1833's `#ifdef HAVE_PGTK` (`xg_create_frame_outer_widgets`), C 1883's `#if ! GTK_CHECK_VERSION (3, 22, 0)` | `have_pgtk`; `not (gtk_check_version_ge_3_22)` | a pgtk build / a GTK < 3.22 build |
| C 1954/1962's `#ifndef HAVE_PGTK` and C 1955's `#ifdef HAVE_XDBE` (`xg_free_frame_widgets`), C 2007's `#ifndef HAVE_PGTK` fold (`xg_wm_set_size_hint`) | `not (have_pgtk)`; `have_xdbe` | a GTK build with Xdbe / an X11-GTK build |
| C 2124 `#ifdef USE_TOOLKIT_SCROLL_BARS` (`xg_set_background_color`) | `use_toolkit_scroll_bars` - **not registered** (Owed 9) | a build with toolkit scroll bars |
| C 2145's `#ifdef HAVE_PGTK` (`xg_set_undecorated`), C 2175/2190's `#ifndef HAVE_PGTK` / `#else` (`xg_frame_restack`, `xg_set_skip_taskbar`), C 2209/2229's `#ifdef HAVE_PGTK` (`xg_set_no_focus_on_map`, `xg_set_no_accept_focus`), C 2261's `#ifndef HAVE_PGTK` + C 2268's dead inner `#ifdef HAVE_PGTK` (`xg_set_frame_icon`) | `have_pgtk` / `not (have_pgtk)`; the two in-expression arm splits are arm choosers (`## Deviations` 17) | as above |

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening comment (title, copyright, licence) | dropped (P-86 ①; the port opens with `// femacs: port of src/gtkutil.c @ fd7afced403.`) |
| 19-21 | C 19's blank, `#include <config.h>` and C 21's blank | ungated (C's own position: outside the file gate) |
| 22 | `#ifdef USE_GTK` | the file gate (every row below carries it too) |
| 23-47 | `#include <float.h>/<stdio.h>/<stdlib.h>` (C 23-25), `<c-ctype.h>` (C 27), the six Emacs headers (C 29-32), the `HAVE_PGTK` include pair (C 33-41), the six more (C 42-47) | file gate; C 33-41 is the `have_pgtk` pair |
| 48-68 | C 48's blank, `<gdk/gdkkeysyms.h>` (C 49), `HAVE_XINPUT2`'s (C 51-53), `HAVE_XFT`'s (C 55-57), `HAVE_GTK3`'s `gtkx.h` + `"emacsgtkfixed.h"` (C 59-64) and `HAVE_XDBE`'s (C 66-68) | file gate; C 49 is where the toolkit-surface segment stands (P-88 ①); the four inner arms are in the gate table |
| 69-78 | C 69's blank and `#ifdef HAVE_GTK3`'s three `XG_TEXT_*` ids with the GTK2 `#else` arm | `have_gtk3` / `not (have_gtk3)` |
| 79-95 | C 79's blank and the `#ifdef HAVE_GTK3` block: C 81-82's two forward declarations (dropped, 0.1-12), C 84-92's `EmacsMenuBar`/`EmacsMenuBarClass` and C 94's `G_DEFINE_TYPE` | `have_gtk3`; C 94's expansion is Owed 5 |
| 96-106 | C 96's blank; the `#ifndef HAVE_PGTK` forward declarations (C 97-102, dropped); C 104's `#if GTK_CHECK_VERSION (3, 10, 0)` declaration (C 105, dropped) | `not (have_pgtk)`; C 104-106's gate is `gtk_check_version_ge_3_10` |
| 107-125 | C 107's blank and `#ifndef HAVE_GTK3`'s five GTK2 macros (C 110-117 inside `#ifdef HAVE_FREETYPE`) | `not (have_gtk3)` (+ `have_freetype`) |
| 126-127 | C 126's blank and `#define XG_BIN_CHILD(x) gtk_bin_get_child (GTK_BIN (x))` | file gate |
| 128-130 | C 128's blank and the two `update_theme_scrollbar_*` forward declarations (definitions C 4475/4499) | dropped (0.1-12) |
| 131-140 | C 131's blank, `#define TB_INFO_KEY "xg_frame_tb_info"` and `struct xg_frame_tb_info` | file gate |
| 141-146 | C 141's blank, the `#ifdef HAVE_XWIDGETS` object `xg_gtk_initialized` (C 143) and C 146's `xg_get_widget_from_map` forward declaration (definition C 4430/4449) | `have_xwidgets`; C 146 dropped (0.1-12) |
| 147-185 | C 147-149's blank/form-feed, the banner (C 188-190 is the next banner) and the `#ifdef HAVE_GTK3` arm's four functions: `emacs_menu_bar_init` (C 151-155), `emacs_menu_bar_class_init` (C 157-164), `emacs_menu_bar_get_preferred_width` (C 166-177), `emacs_menu_bar_new` (C 179-183) | `have_gtk3` |
| 186-226 | the "Display handling functions" banner (C 188-190), the `gdpy_def` object (C 195) and `xg_set_screen` (C 201-226) | file gate; C 204-225 is the `have_pgtk` pair |
| 227-264 | `xg_display_open` (C 236-264; C 237-241 the declarator pair) | `not (have_pgtk)` / `have_pgtk` (two `fn`s) |
| 265-295 | `xg_get_gdk_scale` (C 267-282) and `xg_get_scale` (C 284-295) | file gate; C 270/287/290's inner arms |
| 296-355 | `xg_display_close` (C 299-355; C 300-304 the declarator pair, C 306-330 the X body, C 331-354 the pgtk one) | `not (have_pgtk)` / `have_pgtk` |
| 356-376 | the "Utility functions" banner (C 358-360) and `xg_create_default_cursor` (C 365-376) | `not (have_pgtk)` / `have_pgtk` |
| 377-478 | `#ifndef HAVE_PGTK`: `xg_get_pixbuf_from_pix_and_mask` (C 381-426) and, inside `#if defined USE_CAIRO && !defined HAVE_GTK3`, `xg_get_pixbuf_from_surface` (C 429-475) | `not (have_pgtk)`; the second also `use_cairo` and `not (have_gtk3)` |
| 479-493 | `file_for_image` (C 480-493) | file gate |
| 494-595 | C 495-504's comment and `xg_get_image_for_pixmap` (C 506-595; C 512-516 the cairo declaration pair, C 544's `#ifdef USE_CAIRO`, C 552's `#ifdef HAVE_GTK3`) | file gate; the inner pair is `use_cairo` and `have_gtk3` |
| 596-617 | `xg_set_cursor` (C 602-617) | file gate |
| 618-648 | `xg_list_insert` (C 621-630) and `xg_list_remove` (C 634-648); the `xg_list_node` record they take is `gtkutil.h` 52-56's and stands here | file gate |
| 649-729 | the `get_utf8_string` comment (C 650-653) and the function (C 655-729) | file gate |
| 730-810 | `xg_check_special_colors` (C 735-810; C 743-753 the declaration pair, C 759/790 the pgtk/parse pair) | file gate; `have_gtk3`; `not (have_pgtk)` |
| 811-1016 | the "Tooltips" banner (C 814-816) and the whole section: `hierarchy_ch_cb` (C 824-835), `qttip_cb` (C 843-888), `xg_prepare_tooltip` (C 893-942), `xg_show_tooltip` (C 947-965), `xg_hide_tooltip` (C 971-992) in the X arm, and C 994's `#else  /* HAVE_PGTK */` arm's `xg_show_tooltip` (C 996-1006) and `xg_hide_tooltip` (C 1008-1016) | `not (have_pgtk)` / `have_pgtk` |
| 1017-1031 | the "General functions …" banner (C 1019-1021) and, inside `#if ! GTK_CHECK_VERSION (3, 22, 0)`, `my_log_handler` (C 1024-1030) | `not (gtk_check_version_ge_3_22)` |
| 1032-1052 | `xg_is_menu_window` (C 1034-1051) | `all (have_gtk3, have_xinput2)` |
| 1053-1139 | the comment (C 1054-1057) and `xg_set_geometry` (C 1059-1139; C 1065 opens the `if (x_gtk_use_window_move)` block and C 1104 closes it after the `else` arm, C 1072's `#ifndef HAVE_PGTK` and C 1084's `#else`) | file gate; the arm chooser is `gtk_check_version_ge_3_22` (Deviation 4) |
| 1140-1172 | the comment (C 1141-1142) and `xg_frame_resized` (C 1143-1172) | file gate |
| 1173-1296 | the comment (C 1174-1175) and `xg_frame_set_char_size` (C 1176-1296; C 1186's `#ifndef HAVE_PGTK`, C 1227's, C 1237's `#ifdef USE_CAIRO`, C 1273's) | file gate; `not (have_pgtk)` / `have_pgtk`; `use_cairo` |
| 1297-1312 | C 1297's blank, the comment (C 1298-1300) and the `#if 0` block (C 1301-1311) that holds `xg_height_or_width_changed`, C 1312's blank | `#if 0`: dead text C itself excludes, recorded as a comment (P-86 ①) |
| 1313-1391 | the comment (C 1313-1315) and `xg_frame_set_size_and_position` (C 1316-1391; C 1351's `#ifndef HAVE_PGTK`, C 1359's `#ifdef USE_CAIRO`, C 1363's `#else`, C 1383's `#ifndef HAVE_PGTK`) | file gate; `not (have_pgtk)` / `have_pgtk`; `use_cairo` |
| 1392-1421 | C 1393's `#ifndef HAVE_PGTK` and `xg_win_to_widget` (C 1400-1420) | `not (have_pgtk)` |
| 1422-1458 | `xg_set_widget_bg` (C 1426-1458; C 1428's `#ifdef HAVE_GTK3`, C 1431/1433's pgtk pair, C 1452's GTK2 `#else`) | file gate; `have_gtk3`; `not (have_pgtk)` / `have_pgtk` |
| 1459-1503 | the dark-mode trio inside `#if defined HAVE_PGTK && defined HAVE_GSETTINGS` (C 1462-1503): `xg_set_gtk_theme_dark_mode` (C 1464-1468), `xg_update_dark_mode_for_all_displays` (C 1473-1492), `xg_set_initial_dark_mode` (C 1498-1502) | `have_pgtk` (+ `have_gsettings`, unregistered) |
| 1504-1571 | `style_changed_cb` (C 1509-1559; C 1516/1522/1545's `#ifndef HAVE_PGTK` arms) and `delete_cb` (C 1564-1570, under C 1563's `#ifndef HAVE_PGTK`) | file gate; `not (have_pgtk)` / `have_pgtk` |
| 1572-1831 | `xg_create_frame_widgets` (C 1577-1831; the fifteen arms of C 1582-1826) | file gate; `have_gtk3` / `not (have_gtk3)`; `not (have_pgtk)` / `have_pgtk`; `gtk_check_version_ge_3_10`; `not (gtk_check_version_ge_3_22)`; `have_gtk_window_set_has_resize_grip` (unregistered); `have_gsettings` |
| 1832-1933 | `xg_create_frame_outer_widgets` inside `#ifdef HAVE_PGTK` (C 1833-1932; C 1883's version arm, C 1927's `HAVE_GSETTINGS` pair) | `have_pgtk`; `not (gtk_check_version_ge_3_22)`; `have_gsettings` |
| 1934-1976 | `xg_free_frame_widgets` (C 1936-1976; C 1954/1962's `#ifndef HAVE_PGTK` and C 1955's `#ifdef HAVE_XDBE`) | file gate; `not (have_pgtk)`; `have_xdbe` |
| 1977-2108 | the comment (C 1978-1982) and `xg_wm_set_size_hint` (C 1985-2108; C 2007's `#ifndef HAVE_PGTK` fold) | file gate; `not (have_pgtk)` |
| 2109-2138 | the comment (C 2110-2114) and `xg_set_background_color` (C 2117-2138; C 2124's `#ifdef USE_TOOLKIT_SCROLL_BARS`) | file gate; `use_toolkit_scroll_bars` (unregistered) |
| 2139-2277 | the seven frame-parameter setters: `xg_set_undecorated` (C 2143-2156, C 2145's `#ifdef HAVE_PGTK`), `xg_frame_restack` (C 2162-2182, C 2175's pair), `xg_set_skip_taskbar` (C 2187-2202, C 2190's pair), `xg_set_no_focus_on_map` (C 2207-2222, C 2209's `#ifdef HAVE_PGTK`), `xg_set_no_accept_focus` (C 2226-2244, C 2229's), `xg_set_override_redirect` (C 2247-2259) and `xg_set_frame_icon` (C 2266-2277, under C 2261's `#ifndef HAVE_PGTK` with C 2268's dead `#ifdef HAVE_PGTK`) | file gate; `have_pgtk` / `not (have_pgtk)` |
| 2278-2281 | C 2278's `#endif`, C 2279-2280's blanks and C 2281's form feed | file gate |
| 2282-6753 | **the remainder** — from C 2282's banner through C 6753: `get_dialog_title` (C 2288, the next live definition), `create_dialog`, `xg_dialog_run`, the file chooser half (`xg_get_file_name_from_chooser`, `xg_get_file_with_chooser`, `xg_get_file_with_selection`, `xg_get_file_name`), `xg_get_font` and the font machinery, the menu machinery (`make_cl_data`, `xg_mark_data`, `create_menus`, `xg_update_menubar`, …), scroll bars, printing (`draw_page`, `print_settings`), the tool bar (`xg_create_tool_bar`, …) and `xg_initialize`/`xg_filter_key`/`xg_widget_style_updated`/`xg_im_context_*` at the end | not this slice |

The rows tile C 1-6753 **contiguously** (blank and form-feed lines included) rather than skipping the
blanks between blocks, and the last row runs on to C 6753, the reference's last line, so
`tools/check-dead-file-coverage.py`'s per-line classification finds no non-blank line without a row.

The gate binds this ledger to **two** references - `src/gtkutil.c` and `src/gtkutil.h` - because
`docs/port-map.md` pairs `femacs/src/include/gtkutil.rs`'s target with the same ledger by its
`include-` stem rule; the second binding is satisfied by the same ranges (they run past C 245 and
cover its blanks), exactly as `docs/ports/pgtkterm.md` records for `src/pgtkterm.h`.  The header
mirror's own state is `docs/ports/include-gtkutil.md`'s (a placeholder: guard only) and this table
says nothing about it.

## Evidence

**①S2 - ⑤S2 are slice 2's (C 1297-2281); ① - ⑤ above are slice 1's, unchanged.**

**①S2 The frozen reading - `build-tu` (the P-86 ⑧ gate check).**
`cd femacs && CARGO_TARGET_DIR=/tmp/gtk2-lane/cargo cargo run -q -p xtask -- build-tu src/gtkutil.c` ->
`built src/gtkutil.rs -> ../build/femacs/gtkutil.o`, **exit 0**, `build/femacs/gtkutil.o` is **336
bytes** - the same empty object slice 1 reads, for the same reason (every item is behind `use_gtk`).
**Budget note: the mandate allowed three `build-tu` runs and slice 2 used five**, two over: the
first three materialised the slice and the two rounds of arm findings, the fourth re-verified the
frozen object after the `x_output.hint_flags` cast post-dated run three, and the fifth re-verified it
after the `format` fix and the toolkit marker post-dated run four - every extra run closed a real
defect or re-read the frozen object on the revision being described.  Zero `probe` runs: the arm
readings are rustc on the materialised `_root.rs`, which is the compile `xtask probe` performs.

**②S2 The arm readings (three, `text-only`, P-86 ③⑵), on the final revision.**  Same arrangement as
② above: no GTK tree here, so these are rustc's name-resolution and type errors on the materialised
root `build/femacs/units/gtkutil/_root.rs` (**4200 lines**), with the features a real target would
set.  Classified by the *primary* span:

```
$ rustc --edition 2024 --crate-type=lib --emit=metadata -O build/femacs/units/gtkutil/_root.rs I<A>
53 error(s)   (19 in this unit, 34 anchored in mirrors)

$ rustc ... I<B>                                                                          # ARM B
115 error(s)  (99 in this unit, 16 anchored in mirrors)

$ rustc ... I<C>                                                                          # ARM C
64 error(s)   (30 in this unit, 34 anchored in mirrors)
```

Every one of this unit's own errors is a **mirror record or edge the tree owes**, or a definition
the remainder carries - **none is a defect of a C statement's transcription**:

| arm | unit errors | class | anchor |
| --- | ---: | --- | --- |
| A/C | 5 each | `no field new_size_p on type frame` | slice 1's (`include/frame.rs` 725, P-80's member dispatch) |
| A/C/B | 1 each | `cannot find type redisplay_interface` | slice 1's (`include/dispextern.rs`) |
| A/B | 2 each | `cannot find value emacs_menu_bar_parent_class` / `cannot find function emacs_menu_bar_get_type` | slice 1's C 94 `G_DEFINE_TYPE` expansion (Owed 5) |
| A/C | 1 each | `no field toolbar_is_packed on type x_output` | `xterm.h` 1106's `bool_bf toolbar_is_packed : 1`; `include/xterm.rs` 754 records the bit, not a field (Owed 11) |
| A/C | 1 each | `cannot find function update_frame_tool_bar` | `gtkutil.h` 158; `include/gtkutil.rs` is a placeholder (Owed 11) |
| A/C | 1 each | `cannot find function initial_set_up_x_back_buffer` | `xterm.h` (C 1752); `include/xterm.rs` does not carry it (Owed 11) |
| A/C | 2 each | `cannot find function x_wm_supports` | `xterm.h` 1897 (C 2008-2009); `include/xterm.rs` does not carry it (Owed 11) |
| A/C | 2 each | `cannot find function update_theme_scrollbar_width` / `_height` | this file, C 4475/4499 - definitions the remainder carries, forward-declared at C 128-130 |
| A/C | 4 each | `cannot find value xg_im_context_commit` / `xg_im_context_preedit_changed` / `xg_im_context_preedit_end` / `xg_widget_key_press_event_cb` | this file, C 6360-6448 - definitions the remainder carries (C 97-102's forward declarations) |
| B | 71 | `cannot find macro FRAME_GTK_OUTER_WIDGET` (38), `FRAME_GTK_WIDGET` (15), `FRAME_TOOLBAR_WIDTH`/`FRAME_TOOLBAR_HEIGHT`/`FRAME_MENUBAR_HEIGHT` (3 each), `FRAME_X_DISPLAY` (3), `FRAME_DISPLAY_INFO` (2), `FRAME_WIDGET` (2), `DEFAULT_GDK_DISPLAY` / `FRAME_X_WINDOW` (1 each) | C's pgtk build reaches every one through `"gtkutil.h"` 29's `#include "pgtkterm.h"`, whose mirror is a placeholder that `include/gtkutil.rs`'s `have_pgtk` arm does not `include!` (Owed 3/11) |
| B | 8 | `cannot find type pgtk_display_info` (6), `pgtk_output`, `x_display_list` | `include/pgtkterm.rs` (the same placeholder) |
| B | 1 | `cannot find type Display` | this unit's `gdk_x11_*` declarations are X-arm-only; C's pgtk text has no such call |
| B | 1 | `no field tooltip on type frame` | `frame.h` 457's `bool_bf tooltip : 1`; the mirror records the bit, not a field (Owed 11) - C 1602 is pgtk-only |
| B | 5 | `cannot find type GtkSettings` (2), `cannot find function gtk_settings_get_for_screen` (2), `gtk_tooltip_set_custom` (1) | slice 1's `#[cfg (all (use_gtk, not (have_pgtk)))]` block: those three are declared for the X arm, and C's pgtk text reaches them through the (absent) term header |
| B | 2 | `update_frame_tool_bar` | as above |
| B | 1 | `cannot find function xg_win_to_widget` | this file, C 1401 - the X arm's; C's pgtk text does not call it (slice 1's arm-B reading) |
| C | 3+1 | `cannot find value GTK_ORIENTATION_HORIZONTAL` / `_VERTICAL` | GTK2's enumerators - no GTK2 tree on this machine (Owed 6) |
| C | 2 | `cannot find value GTK_STATE_SELECTED` | slice 1's same gap (`gtk/gtkenums.h`, GTK2) |
| C | 2+2 | `cannot find function gtk_hbox_new` / `gtk_vbox_new` | C 119-121's GTK2 `gtk_box_new` macro body - GTK2's, no header here (Owed 6) |
| C | 1+1 | `no field bg` / `no field fg` on `GtkStyle` | slice 1's GTK2 gap (the `GtkStyle` record) |
| C | 1 | `no field bg_pixmap_name on type GtkRcStyle` | GTK2's `GtkRcStyle` record is an opaque handle here (Owed 6) |

**③S2 The self-checks (repo root, final revision).**

```
python3 tools/check-dead-file-coverage.py
  -> info docs/ports/gtkutil.md: 46 range row(s) cover src/gtkutil.c (6753 lines; 0 blank
     line(s) between blocks need no row)
     info docs/ports/gtkutil.md: 46 range row(s) cover src/gtkutil.h (245 lines; 0 blank
     line(s) between blocks need no row)
     ... 45 ledger(s) bound (a C range table), 0 failure(s)                        [exit 0]
python3 tools/check-declarations.py
  -> 10 declaration sites (0 forbidden, 10 allowed literals), 0 failures           [exit 0]
python3 tools/check-ledger-ticks.py
  -> 234 ledger(s) bound (a function tick to compare), 0 failure(s)                [exit 0]
python3 tools/check-mirror-dupes.py
  -> 19 duplicate declaration(s), 19 ratchet row(s), 0 failure(s)                  [exit 0]
python3 tools/check-port-lines.py
  -> 69 file(s) under ten percent, 88,245 reference line(s); 69 ratchet row(s),
     0 new, 0 stale                                                               [exit 0]
python3 tools/check-cond-eval.py src/gtkutil.c
  -> 212 conditions, 1 unknown, 1 disagreements [NOT VERIFIED]                     [exit 1]
```

`check-cond-eval.py`'s single disagreement is C 6652, past both slices (slice 1's row above);
`check-port-lines.py` still reports **no row for `src/gtkutil.c`** (the port crossed ten percent but
the file is not in `docs/baseline/port-lines-pending.tsv`), so no ratchet row is owed here.

**④S2 `extract-abi.py --check` (the ABI reading), final revision.**
`python3 tools/extract-abi.py --check emacs/src/gtkutil.c femacs/src/gtkutil.rs` -> **124 `- ` lines**
(slice 1: 153; the drop is the C 1297-2281 functions leaving the missing list), `FAIL` (the unported
remainder - C 2282-6753, ~95 functions - is most of them).  Every slice-2 name is present: the check
reports **no** `missing function` for any of `xg_frame_set_size_and_position`, `xg_win_to_widget`,
`xg_set_widget_bg`, `xg_set_gtk_theme_dark_mode`, `xg_update_dark_mode_for_all_displays`,
`xg_set_initial_dark_mode`, `style_changed_cb`, `delete_cb`, `xg_create_frame_widgets`,
`xg_create_frame_outer_widgets`, `xg_free_frame_widgets`, `xg_wm_set_size_hint`,
`xg_set_background_color`, `xg_set_undecorated`, `xg_frame_restack`, `xg_set_skip_taskbar`,
`xg_set_no_focus_on_map`, `xg_set_no_accept_focus`, `xg_set_override_redirect`,
`xg_set_frame_icon`.  The non-remainder rows are:

```
  (4 C macros have several definitions ..., body not compared: XG_TEXT_CANCEL, XG_TEXT_OK,
   XG_TEXT_OPEN, xp)                                            # slice 1's, unchanged
- macro gtk_box_new does not mention false, gtk_hbox_new, gtk_orientation_horizontal,
  gtk_vbox_new                                                    # slice 1's row (Owed 4)
- family ckd_add / ckd_mul ...                                    # slice 1's rows (Owed 4)
- function xg_create_frame_widgets does not mention
  have_gtk_window_set_has_resize_grip                             # slice 2's, Owed 10
- line count drifted 37.8%: C 6753 vs Rust 4200                    # the whole-file ratio
```

The one new row is a **tool artifact, not a stub**: the C token comes from the *continuation line* of
C 1620-1621's two-line `#if (! defined HAVE_GTK3 \` / ` && defined HAVE_GTK_WINDOW_SET_HAS_RESIZE_
GRIP)`, and `body_token_seq` strips `#[...]` attributes and string literals, so the port's
`#[cfg (feature = "have_gtk_window_set_has_resize_grip")]` can never supply it.  It needs a
`docs/ports/macro-body-notes.tsv` row, which this lane cannot write (Owed 10).  An earlier run of
this check also read `function xg_set_widget_bg does not mention format`; that was a **real defect of
slice 2's first draft** (the C 1442 local `format` had been inlined away), and naming it back - a
`const format` byte-string - removed the row.

**⑤S2 The readings the new declarations were copied from.**  GLib/GObject is on this machine, so the
four new GLib names quote the header and line (`gsignal.h` 218-226 `G_SIGNAL_MATCH_FUNC = 1 << 3` and
458-465's prototype; `gobject.h` 695 `g_clear_object` -> `g_clear_pointer ((pp), g_object_unref)`
with `gmem.h` 135-145's `__GNUC__` body); `glibconfig.h` 43/47 give `gint8`/`guint16`.  X11 is on the
machine too: `Xutil.h` 103 `USSize`, `X.h` 422-431's ten gravities, `Xlib.h` 2967-2969
`XQueryColor`, 1724-1726 `XVisualIDFromVisual`, 3419-3421 `XSync`, 81 `False` (already mirrored).
**GDK and GTK are not**, so their citations name the header alone and their constant values are
placeholders (Owed 6/12).

## Deviations

**① The frozen reading — `build-tu` (the P-86 ⑧ gate check).**
`cd femacs && CARGO_TARGET_DIR=/tmp/gtkutil1-lane/cargo cargo run -q -p xtask -- build-tu
src/gtkutil.c` ->
`built src/gtkutil.rs -> ../build/femacs/gtkutil.o`, **exit 0**, `build/femacs/gtkutil.o` is
**336 bytes** - an empty object, which is the expected reading here and not a proof of the new text:
every item sits behind `use_gtk`, so under the frozen feature set rustc compiles only
`include/config.rs` and the `#![allow]` attribute (the same 336 bytes
`docs/ports/pgtkterm.md` reports for its own file).  **Budget note: the mandate allowed three
`build-tu` runs and this batch used seven.**  The first is the placeholder's replacement; the other
six are re-materialisations that followed the arm readings' findings, and every one of them closed a
real defect: the `g_signal_connect` body (`NULL`/`(GConnectFlags) 0`), the `bytes_read`/
`bytes_written`/`c` pre-initialisations, the gate split the X-arm/GTK3-only declarations needed,
the two parameters Rust must declare `mut` (`old_widget`, `f`), the `GTK_IS_WINDOW` gate, and the
four `emacs_menu_bar_*` functions (C 151-183), which the first draft had skipped and a
C-inventory-versus-port name scan caught.  Zero official `probe` runs were needed: the arm readings
below are `rustc` on the **materialised** root `build/femacs/units/gtkutil/_root.rs` (2593 lines),
which is the same compile `xtask probe` performs with the materialisation step inside it - the
`docs/ports/pgtkterm.md` Evidence ② arrangement, reported as such and not as an `xtask` run.

**② The arm readings (three, `text-only`, P-86 ③⑵), on the final revision.**  There is no GTK tree
on this machine (above), so no arm can be *compiled*; the readings are rustc's name-resolution and
type errors on the unit's own text plus the mirrors it pulls in, with the features a real target
would set.  Classified, **every one of this unit's own errors is one of the classes named below -
none is a defect of a C statement's transcription**:

```
$ rustc --edition 2024 --crate-type=lib --emit=metadata -O build/femacs/units/gtkutil/_root.rs I<A>   # ARM A
43 error(s)   (10 in this unit, 33 anchored in mirrors)

$ rustc ... I<B>                                                                                   # ARM B
60 error(s)   (45 in this unit, 15 anchored in mirrors)

$ rustc ... I<C>                                                                                   # ARM C
42 error(s)   (11 in this unit, 31 anchored in mirrors)

I<A> is the nine --cfg pairs use_gtk, have_x_windows, have_window_system, have_gtk3, use_cairo,
have_xinput2, have_xft, have_xdbe, have_freetype; I<B> is use_gtk, have_pgtk, have_window_system,
have_gtk3, use_cairo, have_xinput2; I<C> is use_gtk, have_x_windows, have_window_system, use_cairo
(a GTK2 build).
```

| arm | this unit's errors | class | anchor |
| --- | ---: | --- | --- |
| A | 5 | `no field new_size_p on type frame` | `include/frame.rs` 725: the `bool_bf new_size_p : 1` member is recorded, not stored (P-80's member-dispatch accessor is the fix; the `src/xfns.rs` 980-1000 `wait_for_wm` precedent) |
| A | 1 | `cannot find type redisplay_interface` | `include/dispextern.rs` 3697: the record is behind the tree's own `dispextern_face_records` gate (status 3.220) *and* its `*mut font` members need `include/font.rs` on the graph |
| A | 2 | `cannot find value emacs_menu_bar_parent_class` (C 172) / `cannot find function emacs_menu_bar_get_type` (C 182) | C 94's `G_DEFINE_TYPE` expansion, which this slice does not carry (Owed 5) |
| A | 2 | `cannot find function xg_win_to_widget` / `xg_wm_set_size_hint` | this file, C 1401 / C 1985 - definitions the remainder slice carries; the calls stay as C's (the `docs/ports/pgtkterm.md` Deviations 3 shape) |
| B | 11+7+2+2+1+1+1 | `cannot find macro FRAME_GTK_OUTER_WIDGET` (11), `FRAME_GTK_WIDGET` (7), `FRAME_DISPLAY_INFO` (2), `FRAME_X_DISPLAY` (2), `FRAME_TOOLBAR_HEIGHT`/`FRAME_MENUBAR_HEIGHT`/`FRAME_TOOLBAR_WIDTH`/`DEFAULT_GDK_DISPLAY` (1 each) | C's pgtk build reaches these through `"gtkutil.h"` 29's `#include "pgtkterm.h"`, whose mirror is a placeholder that `include/gtkutil.rs`'s `have_pgtk` arm does not `include!` |
| B | 1+6+1 | `cannot find type pgtk_output` (C 40's typedef, 1), `cannot find type pgtk_display_info` (6), `cannot find value x_display_list` (1) | `include/pgtkterm.rs` (the same placeholder) |
| B | 1 | `cannot find type Display` | this unit's `gdk_x11_*`/`x_display_pixel_*` declarations are X-arm-only; the reading is the pgtk arm's, where C's own text has no such call |
| B | 10 | the A-list's classes (both `emacs_menu_bar_*` names, five `new_size_p`, `redisplay_interface`, both `xg_*` functions) | as above |
| C | 5+1+2 | `new_size_p` (5), `xg_wm_set_size_hint` (1), `GTK_STATE_SELECTED` (2) | the first two as above; `GTK_STATE_SELECTED` is gtk+-2.0's enumerator, whose value no header on this machine carries (Owed 6) |
| C | 1+1 | `no field bg on type GtkStyle` / `fg` | C 798-800's GTK2 arm reads two `GdkColor` arrays out of `struct _GtkStyle`; the record is GTK2's and **UNVERIFIED** here, so the port carries the opaque head and names the two member reads (Owed 6) |

**③ `extract-abi.py --check`.**  `python3 tools/extract-abi.py --check emacs/src/gtkutil.c
femacs/src/gtkutil.rs` -> `FAIL`, **149 `- ` lines**: 116 `missing function`, 9 `missing macro`,
6 `missing constant`, 8 `missing file-scope object`, plus the ten "does not mention"/byte-class rows
and the line-drift row (`61.6%: C 6753 vs Rust 2593`).  The file has no subr
(`## Subrs (DEFUN) — 0`), so no `missing subr` row exists.  P-86 ⑩: the check stops being vacuous as
soon as the port has a `fn`, and the comparison is against the file's *whole* inventory
(`## Functions defined — 148`, `## Macros defined — 19`, `## Types and their fields — 4`,
`## File-scope objects — 9`), so 148 − 116 = **32 of the functions are covered** by this slice's 30
`fn` definition sites (25 distinct C names, five of them written once per arm - Deviation 3; the
extractor counts those per arm) and the rest of the 149 lines are the boundary C 1297-6753.  The ten
remaining rows are these, and each is owed to a baseline table this lane cannot write (`## Owed` 4):

| row | what it is |
| --- | --- |
| `missing macro xp` | C 35/38's `#define xp x` / `#define xp pgtk` - a *token* macro used inside `f->output_data.xp`, which Rust cannot spell at all (the port writes the union member each arm's `#define` selects; Deviation 2) |
| `macro GTK_FONT_CHOOSER does not mention gtk_font_selection_dialog` | C 114's GTK2 body: the port's `GTK_FONT_CHOOSER!` casts to `GtkFontSelectionDialog`, whose *definition* is GTK2's and absent here (Deviation 13) |
| `constant TB_INFO_KEY: … different bytes` | C's `"xg_frame_tb_info"` is the port's `c"xg_frame_tb_info".as_ptr ()` - the `NULL`-family token row (Owed 4) |
| `function xg_display_open does not mention 0, strlen` | the `have_pgtk` arm's two tokens live in the *other* `fn` of the arm-split pair (Deviation 3) |
| `function xg_display_close does not mention pgtk_display_info` | same cause |
| `function xg_list_insert does not mention 0` / `xg_list_remove does not mention 0` | C's `node->prev = 0;` is `std::ptr::null_mut ()` (the `NULL` token row again) |
| `function qttip_cb does not mention g_callback` | C 883-884's `G_CALLBACK (hierarchy_ch_cb)` is inside the port's `g_signal_connect!` macro (Deviation 12) |
| `family ckd_add` / `family ckd_mul` (2 rows) | the C calls' `1`/`4` are the port's macro arguments (Deviation 11) |
| `line count drifted 61.6%: C 6753 vs Rust 2593` | the whole-file ratio against a partially ported file; this slice's own ratio is C 1296 → 2593 lines |

**④ The self-checks (repo root, final revision).**

```
python3 tools/check-dead-file-coverage.py
  → info docs/ports/gtkutil.md: 33 range row(s) cover src/gtkutil.c (6753 lines; 0 blank line(s)
    between blocks need no row)
    info docs/ports/gtkutil.md: 33 range row(s) cover src/gtkutil.h (245 lines; 0 blank line(s)
    between blocks need no row)
    … 34 ledger(s) bound (a C range table), 0 failure(s)                       [exit 0]
python3 tools/check-declarations.py
  → 10 declaration sites (0 forbidden, 10 allowed literals), 0 failures        [exit 0]
python3 tools/check-ledger-ticks.py
  → 234 ledger(s) bound (a function tick to compare), 0 failure(s), 0 carried by another file,
    356 row(s) with no function tick … 0 row(s) without a ledger               [exit 0]
python3 tools/check-port-lines.py
  → 75 file(s) under ten percent, 101,649 reference line(s); 75 ratchet row(s), 0 new, 0 stale
                                                                               [exit 0]
python3 tools/check-mirror-dupes.py
  → 19 duplicate declaration(s), 19 ratchet row(s), 0 failure(s)               [exit 0]
python3 tools/check-cond-eval.py src/gtkutil.c
  → DISAGREE(unknown) src/gtkutil.c:6652 `elif GTK_CHECK_VERSION (3, 16, 0)` — clang False,
    evaluator keeps the branch live
    212 conditions, 1 unknown, 1 disagreements [NOT VERIFIED]                  [exit 1]
```

One of those is **not this unit's**: `check-cond-eval.py`'s single disagreement is C 6652, six times
past this slice (the file is one of the four `docs/status.md` 6868 lists with this exact 212/1/1
reading).  An earlier run of `check-mirror-dupes.py` in this batch read `2 failure(s)`, both
`femacs/src/w32proc.rs`'s `signal` declaration - another lane's file, in flight while this slice was
being written, not touched here and cleared by that lane by the final run above.  `check-port-lines.py`
reports **no row for `src/gtkutil.c`**: neither before this batch nor after it does
`docs/baseline/port-lines-pending.tsv` carry the file, so no ratchet row is owed by this slice (the
port went from 11 to 2593 lines, i.e. from below the ten-percent threshold to 38.4% of the
reference, and the reading stayed `0 new, 0 stale`).

**⑤ The readings the declarations were copied from, and the one probe.**  Measured on this machine
with clang and the headers themselves (GLib 2.90.0 at `/opt/homebrew/include/glib-2.0` plus
`glibconfig.h`; cairo 1.18.4 at `/opt/homebrew/include/cairo`):

```
$ clang -std=gnu23 -O0 -I /opt/homebrew/include/glib-2.0       -I /opt/homebrew/Cellar/glib/2.90.0/lib/glib-2.0/include -I /opt/homebrew/include/cairo       /tmp/gtkutil1-lane/probe.c -o /tmp/gtkutil1-lane/probe && /tmp/gtkutil1-lane/probe
sizeof_GList=24 align=8 data=0 next=8 prev=16
sizeof_GType=8 sizeof_GQuark=4 sizeof_GError=16 domain=0 code=4 message=8
sizeof_gchar=1 sizeof_guchar=1 sizeof_gint=4 sizeof_gboolean=4 sizeof_guint=4 sizeof_gulong=8
sizeof_gsize=8 sizeof_guint32=4 sizeof_gpointer=8
sizeof_GLogLevelFlags=4 G_LOG_LEVEL_WARNING=16 G_LOG_FLAG_FATAL=2 G_LOG_FLAG_RECURSION=1
G_CONVERT_ERROR_ILLEGAL_SEQUENCE=1
CAIRO_FORMAT_ARGB32=0 CAIRO_OPERATOR_SOURCE=1 CAIRO_PATTERN_TYPE_SURFACE=1
sizeof_cairo_pattern_type_t=4 sizeof_cairo_format_t=4 sizeof_cairo_operator_t=4
```

Every `core::assert!` in the port is one of those four readings (`GList`'s size/align/offsets,
`GType`'s size, `GError`'s size/offsets).  The GLib/cairo declarations cite the header *and the
line* (`gtypes.h` 52/55/56/58/60/61/109, `glibconfig.h` 55/81, `gquark.h` 38, `gtype.h` 427,
`gmacros.h` 966-972, `glist.h` 39-46/53/175, `gerror.h` 43/47-52/208, `gconvert.h` 53-66/129,
`gmessages.h` 59-71/76/84/95, `gobject.h` 54/192/432/455/459/513/515/594/597, `gclosure.h` 77/92,
`gsignal.h` 184-206/440-446/554-556, `cairo.h` 135/164/229/325-374/431-441/539-540/546/662-696/699/
712-716/973/2489/2718-2724/2731-2735/2990-2997/2999-3000/3135-3137).  The **GDK and GTK**
declarations **cite the header alone, with no line** (no GTK tree here; Deviation 7), and the three
X11 additions C 1062-1112 needs quote `Xutil.h` 74/75/102/103/105, `X.h` 201/607 and `Xlib.h`
2630-2640.

## Deviations

17. **An `#if` that splits *inside* an expression or an `if` condition is an arm chooser.**  C writes
    these once, so the port keeps one copy and puts the arm's term in a `let`: `xg_set_widget_bg`'s
    shared C 1441-1451 block (`cond` is `XQueryColor (…) != 0` in the X arm, or the pgtk arm's six
    `xbg` assignments ending in `true`), `xg_set_skip_taskbar`'s shared body (`cond` is the X arm's
    `FRAME_GTK_WIDGET (f)` test or the pgtk arm's `FRAME_GTK_OUTER_WIDGET (f)` one),
    `xg_create_frame_widgets`'s `gtk_widget_add_events` argument (`events` accumulates the arm's
    masks), `style_changed_cb`'s and `xg_wm_set_size_hint`'s `if` conditions (`frame_p`,
    `suppressed`) - the `xg_set_geometry` `take_body` shape of Deviation 4.
18. **`gtk_box_new` is GTK2's macro (C 119-121, slice 1) and GTK3's function, and C's call sites keep
    C's text in both arms; the port's uniform spelling is the macro form**, so the GTK3 arm carries a
    forwarding `macro_rules! gtk_box_new` that calls the function (the `src/image.rs` 1849-1872
    `image_get_x_image_or_dc` treatment of the same shape).  `gtk_scrollbar_new` keeps C's macro
    alone (nothing calls it in this slice).
19. **The Emacs-side names the mirrors do not carry stand in the unit** (P-72 rule 4): `EMACS_CLASS`
    (frame.h 1840, C 1650/1855) and the per-unit `input_event_bits!` (termhooks.h 373-412's bit-field
    name; the `src/androidterm.rs` 201-207 shape).  `## Owed` 11 lists the moves.
20. **`&CStr` is spelled `&core::ffi::CStr`, and `gint8` is `i8`.**  Neither `CStr` nor `c_schar` is in
    the prelude this crate compiles with.  Slice 1's `TB_INFO_KEY: &CStr` (C 132) had the same defect -
    its arm-A reading counts that error - and slice 2's first arm run surfaced it; this lane fixed it
    in the same file (`&core::ffi::CStr`, the `src/alloc.rs` 5497 spelling) and records it as a
    slice-1 defect here.
21. **`(gpointer) G_CALLBACK (f)` is a transmute.**  C's `G_CALLBACK` is the function pointer and
    `(gpointer)` widens it to `void *`; Rust refuses `Option<unsafe extern "C" fn ()>` as `*mut c_void`,
    so C 1815/1918's argument is `core::mem::transmute::<GCallback, gpointer> (G_CALLBACK! (…))`.
22. **C's implicit conversions are `as` casts at the use site** (P-72 rule 3): `x_output.hint_flags` is
    C's `long` (`xterm.h` 1110), so its two stores read `hint_flags as c_long` and its two loads
    `as c_int`; `above_flag` (C's `bool` into a `gboolean`) and `(*f).alpha_background != 1.0` (C's
    `int`) are `as gboolean`; the `G_OBJECT! (…)` results and `G_CALLBACK! (…)` pass as `as gpointer`;
    `emacs_fixed_new`'s `*mut c_void` return is `as *mut GtkWidget`; `gdkwin` is `as *mut GdkWindow`;
    `flags` in `xg_frame_set_size_and_position` casts the `c_long` XSizeHints ids down.  C's local
    `type` is `r#type` (PORTING.md 0.2).
23. **C 1442's local `format` keeps its name** as a `const format: &[u8]` byte-string, and C's
    `sizeof format` becomes `format.len () + 1` - the array bound and the `sprintf` format pointer both
    use it (the `xg_set_geometry` 1113 shape).  An earlier draft inlined the literal and the ABI check
    reported `xg_set_widget_bg does not mention format`.
24. **C's `#if 0` block (C 1301-1311) is recorded as a comment** (P-86 ①): C itself excludes
    `xg_height_or_width_changed`, so both of C's arms for it are that text and the empty `#else`.
25. **The records and values slice 2 adds are opaque handles or placeholders.**  `GtkRcStyle`,
    `GdkColormap`, `GtkCssProvider`, `GtkStyleProvider`, `GtkIMContext`, `GtkBox`, `GParamSpec`,
    `GdkVisual` are `#[repr(C)]` handles with `_private: [u8; 0]`; `GTK_STATE_NORMAL`,
    `GTK_WINDOW_TOPLEVEL`/`GTK_WINDOW_POPUP`, `GTK_ORIENTATION_*`, the fourteen `GDK_*_MASK`, the seven
    `GDK_HINT_*`, the ten `GDK_GRAVITY_*`, `GDK_NOTHING` and `GTK_STYLE_PROVIDER_PRIORITY_APPLICATION`
    carry placeholder values marked `// placeholder, not GDK's value` (the `src/xwidget.rs` 495-520
    rule: no GDK/GTK tree here to read them from, and the real values are Owed 6/12); `GdkGeometry`
    carries **all eleven** members because C's `memset` and `memcmp` span the whole record - no size
    or offset assertion is written for any of them (**UNVERIFIED**).
26. **`emacsgtkfixed.h`'s mirror is now included** (C 63, `all (use_gtk, have_gtk3)`), because C 1633
    calls `emacs_fixed_new` - the edge slice 1's comment deferred to "a later slice", which is this
    one.

## Owed

9. **`femacs/xtask/Cargo.toml`: three feature names this slice uses and the tree has not registered**
   - **`have_gsettings`** (C 1462/1824/1927, `GSETTINGS`), **`have_gtk_window_set_has_resize_grip`**
   (C 1620) and **`use_toolkit_scroll_bars`** (C 2124, `USE_TOOLKIT_SCROLL_BARS`).  (Slice 1's Owed 1
   for `gtk_check_version_ge_3_10`/`_3_22` is closed: both are registered, `Cargo.toml` 100/101.)
   ✅ **Registered by the integrator in 3.465** (`have_gsettings` after `have_gpm`,
   `have_gtk_window_set_has_resize_grip` after `have_gtk3`, `use_toolkit_scroll_bars` after
   `use_ncurses`, each comment naming its C condition and site) and
   `python3 tools/feature-coverage.py --write` re-recorded the table.
10. **`docs/ports/macro-body-notes.tsv`: one row for `xg_create_frame_widgets`** - the token
    `have_gtk_window_set_has_resize_grip`, which comes from C 1620-1621's two-line `#if` continuation
    and which no `#[cfg]` attribute can supply (Evidence ④S2).  This lane cannot write that file
    (P-86 ⑩).  ✅ **Written by the integrator in 3.465**, together with the second row the same
    reading needs - `gtk_box_new`'s four tokens, which are the GTK2 arm's and cannot appear in the
    forwarding copy the checker compares (`docs/ports/macro-body-notes.tsv`).  Evidence ④'s
    `--check` reading now has no `does not mention` row at all (124 -> 122 `- ` lines).
11. **Mirror gaps this slice cannot close** (each is another lane's file): `include/gtkutil.rs` -
    `update_frame_tool_bar` (gtkutil.h 158), the `xg_*` prototypes C 97-102/128-130/146 declare and
    this slice calls (`update_theme_scrollbar_width`/`_height`, `xg_get_widget_from_map`,
    `xg_widget_style_updated`, `xg_im_context_commit`/`_preedit_changed`/`_preedit_end`,
    `xg_widget_key_press_event_cb`), and its `have_pgtk` arm's `include!` of `include/pgtkterm.rs`;
    `include/pgtkterm.rs` - `pgtk_output`, `pgtk_display_info`, `x_display_list`, `DEFAULT_GDK_DISPLAY`
    and the pgtk `FRAME_*` family (evidence ②S2's 99 arm-B errors are all this placeholder);
    `include/xterm.rs` - `initial_set_up_x_back_buffer` (C 1752), `x_wm_supports` (xterm.h 1897), the
    `toolbar_is_packed` and `new_size_p`/`tooltip` member storage (P-80's dispatch), and its own
    `FRAME_MENUBAR_HEIGHT` body's `void` cast (the `(void) (f)` in an arm the tree reads);
    `include/frame.rs` - the `tooltip` bit's storage and `EMACS_CLASS`; `include/emacsgtkfixed.rs`
    already carries `emacs_fixed_new` (now on the graph).
12. **The GDK/GTK values and layouts**: the real values behind slice 2's placeholders (`GDK_NOTHING`,
    the fourteen masks, seven hints and ten gravities, `GTK_WINDOW_TOPLEVEL`/`POPUP`,
    `GTK_ORIENTATION_*`, `GTK_STATE_NORMAL`, `GTK_STYLE_PROVIDER_PRIORITY_APPLICATION`) and
    `GdkGeometry`/`GtkRcStyle`/`GtkStyle`'s members (`bg_pixmap_name`, `bg`/`fg`) - a GTK3 build host
    closes the first group, a GTK2 one the second.  Same status as Owed 6.
13. **`docs/port-map.md`**: its target cell for `src/gtkutil.c` still reads 11 lines (the port is
    4200).  The map is generated (`tools/gen-port-map.py`), so the integrator's regeneration is the
    step; `tools/check-port-lines.py` itself still reports no row for this file (Evidence ③S2).
14. **The P-90 move grows**: `femacs/src/include/gtk.rs` (still unwritten) now owes slice 2's ~60 new
    GDK/GTK/GLib/cairo-surface names as well as slice 1's 220, and this unit's copies must be deleted
    in that commit.

## Not claimed

1. **The licence block is not carried** (C 1-18, P-86 ① / PORTING.md 0.1-19): the port opens with
   `// femacs: port of src/gtkutil.c @ fd7afced403.`
2. **`xp` is a token macro** (C 35/38's `#define xp x` / `#define xp pgtk`).  C uses it *inside* an
   expression (`f->output_data.xp`), which no Rust macro can fill, so every use site writes the union
   member C's own `#define` selects (`(*f).output_data.x` in the X arm, `(*f).output_data.pgtk` in
   the pgtk one) and the two `typedef`s C derives from it are carried as the arm-gated
   `type xp_output = x_output;` / `= pgtk_output;` (C 36/40).  `extract-abi.py`'s `missing macro xp`
   row is this deviation (Owed 4).
3. **C functions whose whole signature *and* body are version-specific are two `fn`s**, each under
   its own gate - the `src/sysdep.rs` 4680/4959/5183 `system_process_attributes` shape: `xg_display_open`
   (C 236-264: `Display **` vs `GdkDisplay **` and C 246-250/259-263's two statements),
   `xg_display_close` (C 299-355: the parameter is `dpy` in one arm and `gdpy` in the other),
   `xg_create_default_cursor` (C 365-376: same) and the Tooltips pair (`xg_show_tooltip` takes
   coordinates in the X arm and the string in the pgtk one; `xg_hide_tooltip`'s bodies differ).  This
   is why `extract-abi.py`'s two "does not mention" rows for those functions exist: the token lives
   in the sibling `fn`.
4. **`xg_set_geometry`'s `#if ! GTK_CHECK_VERSION (3, 22, 0)` opens inside the block C 1063 opens
   and its `#endif` (C 1104) closes *after* C 1106-1136's `else`.**  C's body appears once in C's
   text, so the port keeps one copy of it and puts the two features on the *arm chooser*
   (`take_body`), C's `if`/`else` and both bodies following in C's order: a GTK >= 3.22 build reads
   `take_body = true` and C 1069-1103's statements alone, an older one reads
   `take_body = x_gtk_use_window_move` with C 1106-1136's `else` arm behind its own `#[cfg]`.  The
   duplicated-body alternative (both arms restating C 1069-1103) was rejected because C writes those
   statements once.
5. **C's implicit integer and pointer conversions are written as `as` casts at the use site**
   (P-72 rule 3 / the `src/xfns.rs` shape): `Pixmap`/`Window`/`guchar*`/`guint32*` boundaries in the
   pixbuf pair, `img->pixmap`/`img->mask`/`img->cr_data` (the mirror keeps those `*mut c_void`),
   `Emacs_Color.pixel` (`c_ulong`) from `GdkColor.pixel` (`guint32`), `(*f).size_hint_flags`
   (`c_int`) against `USPosition | PPosition` (`c_long`), `FRAME_DISPLAY_INFO! (f)` as
   `*mut pgtk_display_info`, and `FRAME_RIF! (f)` as `*mut redisplay_interface` (the mirror's `rif`
   is `*mut c_void`).  The two `0`s C passes for `bool` parameters - `xg_wm_set_size_hint (f, 0, 0)`
   at C 1209 and `adjust_frame_size (…, 5, 0, …)` at C 1294 - are written `false` (Rust has no
   integer-to-`bool` conversion).
6. **Declarations C only forward-declares are dropped** (C 81-82, 98-101, 105, 129-130, 146;
   PORTING.md 0.1-12): Rust has no forward declarations and its items are order-independent, so each
   definition stays at C's line and each declaration line is recorded in the port - including C 94's
   `emacs_menu_bar_get_type` and `emacs_menu_bar_parent_class`, which `G_DEFINE_TYPE` *defines*
   (Deviation 8).
7. **The toolkit surface stands in this unit** (P-88 ①, P-90 ①): `<gdk/gdkkeysyms.h>` (C 49) and
   the `<gtk/gtk.h>`/`<glib-object.h>`/`<cairo.h>`/`<gdk-pixbuf/gdk-pixbuf.h>` names they pull in
   have no file in the reference tree and no mirror, so 220 declarations (86 types/constants, 31
   macros, 103 entry points) stand at C 49's position rather than in a new file under
   `femacs/src/include/`.  The records C reads members of carry exactly those members
   (`GtkAllocation`, `GtkRequisition`, `GdkRGBA`, `GdkColor`, `GtkWidgetClass.get_preferred_width`,
   `EmacsMenuBar.parent`, `EmacsMenuBarClass.parent`) and **every size and offset of theirs is
   UNVERIFIED** - no GTK tree exists here to measure with (P-88 ②).  P-90 ③'s move is owed (Owed 2).
8. **`G_DEFINE_TYPE (EmacsMenuBar, emacs_menu_bar, GTK_TYPE_MENU_BAR)` (C 94) is carried as C's
   invocation plus a note, not as an expansion.**  GLib's macro (`gtype.h` 1840's
   `G_DEFINE_TYPE(TN, t_n, T_P)` -> `G_DEFINE_TYPE_EXTENDED` -> `_G_DEFINE_TYPE_EXTENDED_BEGIN`/
   `_END`, `gtype.h` 2293/2317/2339/2348; its 1826-1839 documentation says it "declares a class
   initialization function, an instance initialization function … a static variable named
   `t_n_parent_class` … Furthermore, it defines a `*_get_type()` function") needs
   `GTK_TYPE_MENU_BAR`, `sizeof (GtkMenuBar)`, `sizeof (GtkMenuBarClass)`, `GTypeInfo` and
   `g_type_register_static_simple` - three of them GTK's, and GTK is not on this machine.  The
   unit therefore keeps C's two call sites (`emacs_menu_bar_parent_class` at C 172,
   `emacs_menu_bar_get_type` at C 182) as C writes them and leaves the expansion to Owed 5;
   **PORTING.md has no entry for an item-defining macro of a third-party header** (its §11 is where
   that ruling belongs, and this lane cannot write it).
9. **`GTK_CHECK_VERSION`'s arms are derived-condition features** (`gtk_check_version_ge_3_10`,
   `gtk_check_version_ge_3_22`) rather than macro-lower-cased names, because C's spelling is a
   function-like *version test* and not a macro of any header: the frozen verdict comes from
   `xterm.h` 73-75 (the reference's own `#define GTK_CHECK_VERSION(i, j, k) false`), and a `use_gtk`
   build takes GTK's `gtk/gtkversion.h`.  Both names are unregistered (Owed 1).
10. **C's `bool` returns are `bool`, C's `gboolean` ones are `gboolean`.**  The tooltip entry points
    (`xg_prepare_tooltip`, `xg_hide_tooltip`, `xg_check_special_colors`) are C `bool` functions, so
    their `TRUE`/`FALSE` become `true`/`false`; `qttip_cb`'s `gboolean` and `gtk_*`'s `gboolean`
    returns keep `TRUE`/`FALSE` (`c_int`).  The four `err`/`success_p`-style locals C leaves
    uninitialized are pre-initialized to C's zero value (PORTING.md 0.1-13): `bytes_read`,
    `bytes_written`, `alloc`, `c`, `surface`, `icon_buf`, `file`, `req`, `alloc` (the last one's
    initializer is also C 1235's four-value aggregate).
11. **`ckd_mul`/`ckd_add` are the P-15 pair**, spelled as the two function-scoped macros
    `ckd_mul!`/`ckd_add!` whose bodies are `src/coding.rs` 165-172's (the frozen target's
    `<stdckdint.h>` defines them as clang's `__builtin_*_overflow`, whose port counterpart is the
    `overflowing_*` pair with a write-back).  The two `family` rows `extract-abi.py` reports are
    this spelling; `SIZE_MAX < alloc` is written `SIZE_MAX < alloc as usize` (the `src/alloc.rs`
    846/863/919 shape).
12. **`g_signal_connect` and `g_list_next` are GLib's own macro bodies** (gsignal.h 554-556,
    glist.h 175), carried as `macro_rules!` beside the two function-scoped `ckd_*` pair; that is why
    `qttip_cb`'s `G_CALLBACK` token is invisible to the checker (Evidence ③).
13. **The GTK2 compatibility macros of C 110-125 keep C's names and bodies** (`gtk_box_new`,
    `gtk_scrollbar_new`, the three font-chooser renames) and the functions they call
    (`gtk_hbox_new`, `gtk_vbox_new`, `gtk_hscrollbar_new`, `gtk_vscrollbar_new`,
    `gtk_font_selection_dialog_new`/`_set_font_name`, `GtkFontSelectionDialog`) are GTK2's and have
    no header on this machine, so they are named but not declared (Owed 6).  The same holds for the
    GTK2 arm's `GTK_STOCK_CANCEL`/`GTK_STOCK_OK`/`GTK_STOCK_OPEN` (gtk/gtkstock.h) and
    `GTK_ORIENTATION_HORIZONTAL`/`GTK_STATE_SELECTED` (gtk/gtkenums.h).
14. **The `GTK_IS_*` and checked-cast macros keep their names and result types**, with the GLib cast
    chain owed (the `src/pgtkterm.rs` 288-296 / `src/xfns.rs` 1019-1031 treatment): `G_OBJECT`,
    `G_CALLBACK`, `GTK_WIDGET`, `GTK_WINDOW`, `GTK_MENU`, `GTK_IMAGE`, `GTK_BIN`, `GTK_CONTAINER`,
    `GTK_LABEL`, `GTK_MENU_SHELL`, `GTK_WIDGET_CLASS`, `GDK_WINDOW`, `GDK_DISPLAY_XDISPLAY`, and the
    four `GTK_IS_*` predicates whose C bodies are `G_TYPE_CHECK_INSTANCE_TYPE` chains (the port's
    bodies are the null test, recorded here).
15. **`include/gtkutil.rs` is a placeholder**, so the two names this slice takes from that header
    stand at their C use site (P-72 rule 4): `xg_list_node` (gtkutil.h 52-56, before C 621) and
    `xg_wm_set_size_hint`'s call (gtkutil.h 156; the definition is C 1985, Owed 3).  The same
    treatment covers the ten names the mirrors do not carry yet (Evidence ④'s list): `min`/`max`
    (lisp.h 71-73), `INT_STRLEN_BOUND` (intprops.h 88-90), `request_sigio`/`unrequest_sigio`
    (dispextern.h 3781-3782), `cancel_mouse_face` (3690), `change_frame_size` (3924),
    `image_find_image_file` (3748), `adjust_frame_size` (frame.h 1511), `gui_set_fullscreen` (1844),
    `frame_size_history_extra` (1515), `store_frame_param`/`get_frame_param` (lisp.h 5117/5120),
    `x_parse_color` (xterm.h 391), `x_wait_for_event` (1899), `x_display_pixel_width/height`
    (1863-1864), `x_cr_update_surface_desired_size` (1812), `XGetGeometry` (Xlib.h 2630-2640),
    `USPosition`/`PPosition`/`XNegative`/`YNegative` (Xutil.h 74/75/102/103/105),
    `XYPixmap`/`ConfigureNotify` (X.h 201/607) and `atol`/`strstr` (stdlib.h/string.h) - each owed
    to the mirror that owns it (Owed 3).
16. **`Frame`'s two `bool_bf` members this slice reads are the mirror's gap** (`new_size_p`,
    C 1149-1170): `include/frame.rs` 725 records the bit rather than storing it, so the six reads
    have no field - the `src/xfns.rs` 980-1000 `wait_for_wm` precedent, whose resulting `E0609` is
    this unit's arm reading (Evidence ②) and whose fix is P-80's member dispatch (Owed 3).

## Owed

1. **`femacs/xtask/Cargo.toml`: two feature names this unit uses and the tree has not registered** -
   **`gtk_check_version_ge_3_10`** (C 104-106) and **`gtk_check_version_ge_3_22`** (C 1023, 1065,
   1104, 2739-like sites in the remainder).  The naming follows `android_api_le_11`'s derived-condition
   row; the frozen verdicts come from `xterm.h` 73-75 (Deviation 9).
2. **The P-90 move: the cairo/GLib/GDK/GTK declarations should move into a dedicated SDK mirror
   `femacs/src/include/gtk.rs` (P-90; `src/pgtkterm.rs`'s in-unit copy owes the same move), and this
   unit's copy must be deleted in that commit.**  The 220 names are these: **GLib/GObject** - `gchar`,
   `guchar`, `gint`, `gboolean`, `guint`, `guint32`, `gulong`, `gsize`, `gpointer`, `GQuark`, `GType`,
   `GError` (`_GError`), `GConvertError`, `G_CONVERT_ERROR_ILLEGAL_SEQUENCE`, `GLogLevelFlags`,
   `G_LOG_FLAG_RECURSION`, `G_LOG_FLAG_FATAL`, `G_LOG_LEVEL_WARNING`, `GLogFunc`, `GList` (`_GList`),
   `GObject`, `GClosure`, `GCallback`, `GClosureNotify`, `GConnectFlags`, `TRUE`, `FALSE`, `G_OBJECT`,
   `G_CALLBACK`, `g_list_next`, `g_signal_connect`, `g_signal_connect_data`, `g_utf8_validate`,
   `g_locale_to_utf8`, `g_strdup`, `g_free`, `g_error_free`, `g_object_new`, `g_object_set`,
   `g_object_get`, `g_object_ref`, `g_object_unref`, `g_object_set_data`, `g_object_get_data`,
   `g_list_free`, `g_log_set_handler`, `g_log_remove_handler`; **cairo** - `cairo_t`,
   `cairo_surface_t`, `cairo_pattern_t`, `cairo_format_t`, `cairo_operator_t`, `cairo_pattern_type_t`,
   `cairo_status_t`, `CAIRO_FORMAT_ARGB32`, `CAIRO_OPERATOR_SOURCE`, `CAIRO_PATTERN_TYPE_SURFACE`,
   `cairo_image_surface_create_for_data`, `cairo_image_surface_get_width`,
   `cairo_image_surface_get_height`, `cairo_create`, `cairo_surface_destroy`,
   `cairo_set_source_surface`, `cairo_set_operator`, `cairo_paint`, `cairo_destroy`,
   `cairo_pattern_get_type`, `cairo_pattern_get_surface`; **GDK** - `GdkDisplay`, `GdkScreen`,
   `GdkCursor`, `GdkWindow`, `GdkPixbuf`, `GdkRGBA`, `GdkColor`, `GdkColorspace`,
   `GDK_COLORSPACE_RGB`, `GdkCursorType`, `GDK_LEFT_PTR`, `GdkDisplayManager`, `GDK_WINDOW`,
   `GDK_DISPLAY_XDISPLAY`, `gdk_display_open`, `gdk_display_manager_get`,
   `gdk_display_manager_set_default_display`, `gdk_x11_lookup_xdisplay`,
   `gdk_x11_display_get_xdisplay`, `gdk_display_get_default_screen`, `gdk_display_get_default`,
   `gdk_display_close`, `gdk_cursor_new_for_display`, `gdk_pixbuf_new`, `gdk_pixbuf_get_pixels`,
   `gdk_pixbuf_get_rowstride`, `gdk_window_peek_children`, `gdk_window_set_cursor`,
   `gdk_window_get_screen`, `gdk_rgba_free`, `gdk_flush`, `gdk_window_resize`; **GTK** - `GtkWidget`,
   `GtkWindow`, `GtkMenu`, `GtkImage`, `GtkBin`, `GtkContainer`, `GtkLabel`, `GtkMenuShell`,
   `GtkMenuBar`, `GtkMenuBarClass`, `GtkWidgetClass`, `GtkTooltip`, `GtkSettings`,
   `GtkStyleContext`, `GtkStyle`, `GtkAllocation`, `GtkRequisition`, `GtkTextDirection`,
   `GtkStateFlags`, `GtkStateType`, `GtkOrientation`, `GTK_STATE_FLAG_SELECTED`,
   `GTK_STATE_FLAG_FOCUSED`, `GTK_STYLE_PROPERTY_BACKGROUND_COLOR`, the six `GTK_*` cast/check
   macros and `XG_BIN_CHILD`'s dependents, and the 36 `gtk_*` entry points the port declares.
   (`gtk/gtkstock.h`'s three stock ids and gtk/gtkenums.h's two enumerators are *named* by the GTK2
   arm and could not be declared at all - Owed 6.)
3. **Mirror gaps this slice cannot close** (each is another lane's file):
   `include/gtkutil.rs` - `xg_list_node`, the `xg_*` prototypes (only C 1985's
   `xg_wm_set_size_hint` is *called* here) and, most importantly, its **`have_pgtk` arm must
   `include!` `include/pgtkterm.rs`** (C reaches it through `gtkutil.h` 29's
   `#include "pgtkterm.h"`; without that edge every arm-B error in Evidence ② stands);
   `include/pgtkterm.rs` - `pgtk_output`, `pgtk_display_info`, `x_display_list`, `FRAME_GTK_WIDGET`,
   `FRAME_GTK_OUTER_WIDGET`, `FRAME_DISPLAY_INFO`, `DEFAULT_GDK_DISPLAY`, `FRAME_X_DISPLAY`,
   `FRAME_TOOLBAR_HEIGHT/WIDTH`, `FRAME_MENUBAR_HEIGHT`; `include/frame.rs` - the `new_size_p`
   member's storage (P-80's dispatch) and `frame_size_history_extra`, `adjust_frame_size`,
   `gui_set_fullscreen`; `include/dispextern.rs` - `image_find_image_file`, `cancel_mouse_face`,
   `change_frame_size`, `request_sigio`/`unrequest_sigio`, and `redisplay_interface`'s gate
   (`dispextern_face_records`, status 3.220) plus the `include/font.rs` edge its `*mut font` members
   need; `include/xterm.rs` - `x_parse_color`, `x_wait_for_event`, `x_display_pixel_width/height`,
   `x_cr_update_surface_desired_size`; `include/xlib.rs` - `XGetGeometry`, `XYPixmap`,
   `ConfigureNotify` and the four `Xutil.h` position flags; `include/lisp.rs` - `min`/`max`
   (lisp.h 71-73); `lib/include/string.rs` - `strstr` (its head already claims that family) and
   `lib/include/stdlib.rs` - `atol`.  The four *inner* names of the cairo/GTK3 pair
   (`GtkRequisition`'s two members' record, `GtkAllocation`'s four) are Owed 2's mirror's.
4. **Baseline rows this lane cannot write** (the P-86 ⑩ lesson): `docs/ports/token-aliases.tsv` -
   the `NULL`-family rows (C's `0` → `std::ptr::null_mut ()` in `xg_list_insert`/`xg_list_remove`,
   `NULL` → `std::ptr::null ()`, `TB_INFO_KEY`'s byte comparison, the `xp` token macro, the GDK/GTK
   `GTK_IS_*` bodies) and `docs/ports/macro-body-notes.tsv` - one row for `gtk_font_chooser_dialog_new`/
   `GTK_FONT_CHOOSER`/`gtk_font_chooser_set_font` (`GTK_FONT_CHOOSER`'s body token), one for
   `qttip_cb`'s `G_CALLBACK` and one for the `ckd_mul`/`ckd_add` call arguments.  All ten rows of
   Evidence ③'s table are this list.
5. **`G_DEFINE_TYPE`'s expansion** (C 94) and **a PORTING.md §11 entry for an item-defining macro of
   a third-party header**: the expansion's `GTK_TYPE_MENU_BAR`, `sizeof (EmacsMenuBar)`,
   `sizeof (EmacsMenuBarClass)`, `GTypeInfo` and `g_type_register_static_simple` are what a GTK3
   toolchain must supply, together with the two names the unit's own text calls
   (`emacs_menu_bar_parent_class`, `emacs_menu_bar_get_type`).
6. **The GTK2/GTK3 toolchain itself**: `GtkStyle`'s `bg`/`fg` arrays and `GTK_STATE_SELECTED`'s value
   (the C-arm errors), `GtkAllocation`/`GtkRequisition`/`GdkRGBA`/`GdkColor`'s layouts and
   `GtkWidgetClass`'s member offset (the `dispextern`-style UNVERIFIED rows), `GtkTextDirection`'s
   width, the six GTK2 compatibility entry points and the four GTK2 macros' values.  A GTK3 build
   host closes the first group and a GTK2 one the second.
7. **`docs/port-map.md`**: its target cell for `src/gtkutil.c` still reads 11 lines (the port is
   2593).  The map is generated (`tools/gen-port-map.py`), so the integrator's regeneration is the
   step; `tools/check-port-lines.py` itself reports no row for this file (Evidence ④) and
   `check-dead-file-coverage.py` reads only the reference's count from the map.
8. **The mirror-side readings this batch produced for the first time** (they are not this unit's, and
   they are what any window-system unit including those mirrors will hit): `include/xterm.rs`'s 17
   errors under `have_x_windows + use_gtk` (10 `E0133` raw-pointer dereferences in its own INLINE
   bodies, `GtkCssProvider`, `GdkGeometry`, `GtkIMContext`, `XIEventMask`, `font`, `void`),
   `include/buffer.rs`'s 8 (`STRING_CHAR`, `UNIBYTE_TO_CHAR`, `string_char_and_length`,
   `BYTES_BY_CHAR_HEAD`, `raw_prev_char_len`), `include/dispextern.rs`'s 6 (`MAX_CHAR`,
   `CHAR_VALID_P`, `CHARACTERP`, `font`), `include/lisp.rs`'s 1 (`c_variadic`, the `verror` row
   `docs/ports/pgtkterm.md` Owed 4 already names) and `include/systime.rs`'s 1 (`E0080`, a layout
   assertion that fails under `have_window_system`).

## Not claimed

- **Slice 2 (C 1297-2281) was executed nowhere.**  Same reading as slice 1's first bullet: no GTK
  tree to compile an arm against and no GTK target to run it on; Evidence ①S2 is a 336-byte empty
  object and ②S2 is rustc's name-resolution and type errors.  Nothing here is a statement about how
  a GTK Emacs behaves.
- **The 19 (arm A) / 99 (arm B) / 30 (arm C) errors of ②S2 are not a correctness reading of slice
  2's transcription** - they are the mirror records and edges (Owed 11) and the remainder's
  definitions still missing, and they include slice 1's own eight (arm A) unchanged.  "No error
  beyond those classes" is a *weak* reading: rustc stops evaluating what it cannot resolve.
- **The GDK/GTK placeholder values are named, not measured.**  Every one of them may differ from the
  real toolkit's, and until a GTK toolchain is available the port's arms and bit tests are
  placeholders with C's member names (Deviations 25, Owed 12).
- **This slice says nothing about C 2282-6753**, whose rows are in the table only so the reference is
  covered.  In particular the remainder *defines* the six names slice 2 calls
  (`update_theme_scrollbar_width`/`_height`, `xg_get_widget_from_map`, `xg_widget_style_updated`, the
  two `xg_im_context_*` callbacks) and `xg_initialize`'s `Qsyms` (`Qundecorated`, `Qdark`, `Qlight`,
  `Qtheme_name`, `Qmaximized`, `Qfullboth` are read from `include/globals.rs`, which already has
  them).
- **The `have_gsettings`, `have_gtk_window_set_has_resize_grip` and `use_toolkit_scroll_bars` gates
  are unregistered** (Owed 9), so the dead-arm text stands under a feature name no build sets; the
  `xg_set_widget_bg`/`xg_set_skip_taskbar`/`events`/`suppressed`/`frame_p` arm choosers (Deviation
  17) and the `gtk_box_new` forwarding macro (Deviation 18) are **spelling** decisions, not claims
  that a GTK2 or GTK3 build compiles.

- **No behaviour of this file was executed anywhere.**  There is no GTK tree to compile an arm
  against and no GTK target to run it on: the frozen reading is a 336-byte empty object (Evidence ①)
  and the arm readings are rustc's *name-resolution and type* errors on the unit's own text
  (Evidence ②).  Nothing here is a statement about how a GTK Emacs behaves.
- The **eight (arm A) / eleven (arm C) / forty-three (arm B)** errors in this unit are **not** a
  correctness reading of the transcription: they are evidence that the records and edge the mirrors
  owe are still owed.  Conversely, "no error of this unit's own statements beyond those classes"
  is a *weak* reading - rustc stops evaluating what it cannot resolve, and P-86 ⑦'s mutation test is
  exactly why the arm readings are reported as harness readings and not as a compile.
- The 153 `- ` lines of `extract-abi.py --check` include the **whole unported remainder** of the file
  (120 of them are the C 1297-6753 functions); that number is a coverage statement, not a defect
  list.  The ten token/byte lines are Owed 4's.
- This slice says nothing about C 1297-6753: no part of it is ported here, and its rows are in the
  table only so the file is covered.  In particular the remainder **defines** the three names this
  slice calls (`xg_win_to_widget` at C 1401, `xg_wm_set_size_hint` at C 1985,
  `Qxg_frame_set_char_size` and the other `Qsyms` in `xg_initialize`).
- The GDK/GTK records' **UNVERIFIED** layouts are named, not measured: a GTK toolchain's readings may
  change `GtkAllocation`, `GtkRequisition`, `GdkRGBA`, `GdkColor`, `GtkStyle` and `GtkWidgetClass`'s
  members and offsets, and until then the port's shapes are placeholders with C's member names.
- The `take_body` arm chooser (Deviation 4), the two `ckd_*` macros' bodies (Deviation 11) and the
  pre-initialisations (Deviation 10) are **spelling** decisions this lane took and recorded; none of
  them is a claim that a GTK build compiles or behaves as C's does.

## P-90 mirror round (2026-10-05): the surface moved out of this unit

- **`femacs/src/include/gtk.rs` is now the third-party surface's single home** (P-90 ①/②/③; the
  P-88 ⑤ move-back).  The 1407-line in-unit segment this ledger's slices 1-2 declared (C 44/49's
  chain: GLib/GObject, cairo, GDK, GTK) was moved there verbatim and replaced, at the same C
  position, by the two edges `#[cfg (feature = "use_gtk")] include!("include/gtk.rs");` and
  `#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))] include!("include/pgtkterm.rs");`.
  The unit now reads 2812 lines (was 4200) and `## C line ranges`, `## Gate mapping` and the
  conditions are unchanged - the surface was never a C block of this file.
- **Two `## Deviations` items were added**, both exposed by the moved mirror typing names the slice
  had left untyped: (i) `FRAME_X_WINDOW! (f) = 0` in `xg_free_frame_widgets` (C 1961) is a `Window`
  (`c_ulong`) in the X arm and a `GtkWidget *` in the pgtk one, so the port splits the arm
  (`= 0` / `= std::ptr::null_mut ()`); (ii) `xg_wm_set_size_hint`'s `let mut size_hints: GdkGeometry;`
  (C 1989) is pre-initialised to `std::mem::zeroed ()` (PORTING.md 0.1-13) - `E0381` otherwise,
  because the port's `memset` takes `&raw mut size_hints` before any write.
- **The arm readings moved as the mirror arrived** (`text-only`, materialised `_root.rs`, A = X11
  GTK3, B = pgtk GTK3, C = GTK2): A **53 -> 53** (19 unit + 34 mirror), B **115 -> 38** (99 -> 21
  unit; the 76 closed are the `include/pgtkterm.rs` placeholder's `FRAME_*`/`pgtk_output`/
  `pgtk_display_info`/`x_display_list` errors), C **64 -> 66** (30 + 36).  **Correction
  (2026-10-05, layout round): A and C reproduce exactly, but B re-reads `115 -> 29` (16 unit + 13
  mirror + rustc's `aborting` row) on this revision - the `38` row is the outlier, not the code;
  see the layout-round bullet below.**  The five remaining
  arm-B unit errors that are not this unit's own are the mirror gaps the ledger already named
  (`GtkSettings`/`gtk_settings_get_for_screen`/`gtk_tooltip_set_custom` were `not (have_pgtk)`-gated
  and are now the surface's; `tooltip`/`new_size_p`/`toolbar_is_packed` are P-80's member
  dispatch; `update_frame_tool_bar`/`update_theme_scrollbar_*`/`xg_win_to_widget`/
  `emacs_menu_bar_*`/`redisplay_interface` stand as before).
- **Owed 14 is closed**; **Owed 11's `update_frame_tool_bar` / `update_theme_scrollbar_*` /
  `xg_get_widget_from_map` / `xg_widget_style_updated` / `xg_im_context_*` / `emacs_menu_bar_*`
  rows remain** (they are `include/gtkutil.rs`'s and the remainder's, not the surface's).

### Layout round (2026-10-05): the mirror's own compile, and this unit's two casts

- **`xtask layout-check` is the mirror's own compile** (P-87 ③: all `src/include/*.rs` in one
  crate, no feature on, so no includer to lend a name).  It read **5 `error[E0425]: cannot find
  type`** on the mirror the P-90 round landed: `VisualID`, `x_display_info` twice,
  `GtkStyleContext`, `GtkStyleProvider`.  All five are repaired *in the mirror*
  (`c_ulong` / `*mut c_void` / the `have_gtk3` gate on
  `gtk_style_context_remove_provider`); the table, the "why the unit saw them and the probe did
  not" sentence and the readings are `docs/ports/include-gtk.md`'s `## Evidence`.  **No name was
  deleted and no arm split was dropped.**
- **The `x_display_info` stand-in costs this unit two inline casts**: C 1076/1082's
  `x_display_pixel_width`/`x_display_pixel_height` call sites now pass
  `FRAME_DISPLAY_INFO! (f) as *mut c_void`, because the mirror's parameter is P-55's
  `*mut c_void` (the record is `include/xterm.rs`'s) - the `include/xsettings.rs` 12-14 rule that
  the consumer converts at the call site (PORTING.md 0.1-9).  No line was added, C's expressions
  and the arm chooser are untouched, and `## Gate mapping` is unchanged.
- **Arm readings re-measured after the repair** (same `text-only` shape and `--cfg` sets as ②S2;
  the totals below count rustc's trailing `error: aborting due to N previous errors` as one
  span-less row, which is why they sit one above the numbers written for the round): A **53 -> 53**
  (19 unit + 33 mirror + 1), B **30 -> 30** (16 + 13 + 1), C **66 -> 64** (30 + 33 + 1 - the same
  two `GtkStyleContext`/`GtkStyleProvider` `E0425`s the probe found, closed in the GTK2 arm too).
  `src/pgtkterm.rs`'s arm B is **19 -> 19** (2 + 16 + 1).  No regression; the GTK2 arm improves by
  two.
- **Budget**: this round used 3 `build-tu` runs (the cap) - `gtkutil.c` twice (the mirror's fix and
  the unit's two casts) and `pgtkterm.c` once - and re-ran `layout-check` 3 times; the arm readings
  were taken 5 times over the two states.
