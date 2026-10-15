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
| **slice 4 (C 2562-2843).** C 2571's `#ifdef HAVE_GTK_FILE_SELECTION_NEW` (`xg_uses_old_file_dialog`), C 2739's same fold (the `xg_get_file_name_from_*`/`with_*` pair) and C 2820's (`xg_get_file_name`'s chooser choice) | `all (use_gtk, have_gtk_file_selection_new)` - registered by this slice | a GTK build whose headers still carry the old file-selection dialog (GTK2) |
| **slice 5 (C 2844-3002).** C 2848's `#ifdef HAVE_FREETYPE` (the whole font-chooser half), C 2850/2886/2947/2961's `HAVE_GTK3` pairs, C 2918's `#ifdef HAVE_PGTK` and C 2859's `#if PANGO_VERSION_CHECK (1, 36, 7)` (the `PANGO_WEIGHT_SEMILIGHT` step of `xg_weight_to_symbol`) | `all (use_gtk, have_freetype)` for the section, `have_gtk3` for the two symbol walks/`xg_font_filter`/the chooser's desc path, `have_pgtk` inside `xg_get_font`, and `pango_version_ge_1_36_7` - registered by this slice | a GTK build with FreeType and GTK3; a pure-GTK (pgtk) build; a Pango >= 1.36.7 build |
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
| 2282-2561 | the "Dialog functions" banner (C 2282-2284) and the whole section: `get_dialog_title` (C 2288-2316), `dialog_delete_callback` (C 2329-2334), **`create_dialog`** (C 2343-2457; C 2363's `GTK_CHECK_VERSION (3, 12, 0)` and C 2419's `(3, 14, 0)` pairs), the `xg_dialog_data` record (C 2459-2465), `xg_dialog_response_cb` (C 2471-2479), `pop_down_dialog` (C 2484-2497), `xg_maybe_add_timer` (C 2502-2519) and `xg_dialog_run` (C 2526-2560, C 2561's blank) | file gate (`use_gtk`); `gtk_check_version_ge_3_12` / `_3_14` inside `create_dialog`; slice 3 below |
| 2562-2843 | the "File dialog functions" banner (C 2562-2565) and the whole section: `xg_uses_old_file_dialog` (C 2568-2576), the `xg_get_file_func` typedef (C 2579), `xg_get_file_name_from_chooser` (C 2584-2588), `xg_toggle_visibility_cb` (C 2593-2600), `xg_toggle_notify_cb` (C 2609-2629), **`xg_get_file_with_chooser`** (C 2642-2737), the `HAVE_GTK_FILE_SELECTION_NEW` pair `xg_get_file_name_from_selector` (C 2744-2749) and `xg_get_file_with_selection` (C 2761-2787), and `xg_get_file_name` (C 2803-2842) | file gate (`use_gtk`); `have_gtk_file_selection_new` for the middle pair; `have_pgtk` inside `xg_get_file_name`; slice 4 below |
| 2844-3002 | the "GTK font chooser" banner (C 2844-2846) and the whole `HAVE_FREETYPE` section: `xg_weight_to_symbol` (C 2852-2870, including C 2859-2861's version-guarded step), `xg_style_to_symbol` (C 2872-2879), `x_last_font_name` (C 2884), `xg_font_filter` (C 2887-2898) and **`xg_get_font`** (C 2911-3001: the `HAVE_PGTK` guard, the chooser dialog, the GTK3 `desc` path and the GTK2 name path) | file gate (`use_gtk`); `have_freetype`; `have_gtk3`; `have_pgtk`; `pango_version_ge_1_36_7`; slice 5 below |
| 3003-3301 | C 3003-3005's blanks and form feed, the "Menu functions." banner (C 3006-3008), the `MENU_ITEM_NAME` macro (C 3014), the two file-scope lists `xg_menu_cb_list`/`xg_menu_item_cb_list` (C 3019/3023), **`make_cl_data`** (C 3035-3053), `update_cl_data` (C 3066-3078), `unref_cl_data` (C 3083-3095), **`xg_mark_data`** (C 3099-3142), `menuitem_destroy_callback` (C 3148-3157), `menuitem_highlight_callback` (C 3166-3189), `menu_destroy_callback` (C 3195-3199), `make_widget_for_menu_item` (C 3206-3235) and **`make_menu_item`** (C 3251-3300) | file gate (`use_gtk`); the gtkutil.h records C 3035-3095 use stand in the unit (Owed 10); `have_pgtk` for C 3133-3141's arm and C 3294-3297's; `gtk_check_version_ge_3_14` for C 3218-3226's pair; slice 6 below |
| 3302-3533 | C 3301-3302's blank and C 3302-3314's comment above `xg_create_one_menuitem` [it was inside slice 6's row until this slice, which carries it here because it belongs to the function below], **`xg_create_one_menuitem`** (C 3316-3363), `menu_bar_button_pressed_cb` under `#ifdef HAVE_PGTK` (C 3366-3381), the `XG_FRAME_DATA` key (C 3459's first use; gtkutil.h 46), and **`create_menus`** (C 3404-3532, C 3384-3402's comment) plus C 3533's blank | file gate (`use_gtk`); `have_gtk3` for C 3440-3444's `#ifndef`/`#else` pair; `have_pgtk` for C 3365-3382 and the two C 3446-3474 arms; slice 7 below |
| 3534-3640 | C 3534-3547's comment above `xg_create_widget`, **`xg_create_widget`** (C 3549-3597), `xg_get_menu_item_label` (C 3601-3606), `xg_item_label_same_p` (C 3610-3623) and `xg_destroy_widgets` (C 3627-3639), plus C 3640's blank | file gate (`use_gtk`); the mirror gains `gtk_label_get_label` (C 3605's read); slice 8 below |
| 3641-3831 | C 3641-3651's comment above `xg_update_menubar` and **`xg_update_menubar`** (C 3653-3830), plus C 3831's blank | file gate (`use_gtk`); the mirror gains `gtk_menu_shell_insert`, `gtk_label_set_text` and `g_object_notify` (C 3673/3763/3764); slice 9 below |
| 3832-3962 | C 3832-3835's comment and **`xg_update_menu_item`** (C 3837-3945), C 3947's comment and `xg_update_toggle_item` (C 3949-3953), C 3955's comment and `xg_update_radio_item` (C 3957-3961), plus C 3962's blank | file gate (`use_gtk`); the mirror gains `GTK_IS_BOX` and `gtk_widget_get_sensitive` (C 3859/3915); slice 10 below |
| 3963-4095 | C 3963-3973's comment above `xg_update_submenu` and **`xg_update_submenu`** (C 3975-4094 - the recursive submenu reconciler, C 4050 calling itself), plus C 4095's blank | file gate (`use_gtk`); the mirror gains `GTK_IS_SEPARATOR_MENU_ITEM`, `GTK_IS_CHECK_MENU_ITEM`, `GTK_IS_MENU_ITEM` and `gtk_menu_item_get_submenu`, and the unit carries `GTK_IS_RADIO_MENU_ITEM` (Owed 12); slice 11 below |
| 4096-4189 | C 4096-4104's comment and **`xg_modify_menubar_widgets`** (C 4105-4170, the menubar updater that calls `xg_update_menubar` first and then walks the sub menus when `deep_p`), C 4172-4174's comment and `menubar_map_cb` (C 4176-4188), plus C 4189's blank | file gate (`use_gtk`); `include/xterm.rs`'s `FRAME_MENUBAR_HEIGHT` gains its live arm and loses its uncompilable `void` body (this slice; see the round section); slice 12 below |
| 4190-4257 | C 4190-4191's comment and **`xg_update_frame_menubar`** (C 4193-4234, the pack/reorder/show/measure/`adjust_frame_size` body with C 4217-4226's `HAVE_GTK3` arm), C 4236-4237's comment and **`free_frame_menubar`** (C 4239-4256), plus C 4257's blank | file gate (`use_gtk`); the mirror gains `gtk_box_reorder_child` and `gdk_screen_get_system_visual`, and the unit carries `gtk_widget_get_mapped` (Owed 10/12's class); slice 13 below |
| 4258-4341 | C 4258's `#ifndef HAVE_PGTK` and **`xg_event_is_for_menubar`** (C 4260-4338 - the non-`static` predicate `gtkutil.h` 114 declares and `xterm.c` calls: reject the event unless it is inside the frame's menu-bar rectangle (`XIDeviceEvent` under `HAVE_XINPUT2`, `XButtonEvent` otherwise), look the frame's `GdkWindow` up, ask GTK which widget the event belongs to, then walk the menubar's children for a mapped one the 1x1 rectangle intersects), C 4339's `#endif`, C 4340-4341's blanks | file gate (`use_gtk`) **and** `not (feature = "have_pgtk")` (C 4258); the mirror widens `GTK_IS_MENU_BAR` to a `have_gtk3`/`not (have_gtk3)` pair and gains `gtk_widget_is_ancestor`, and the unit carries the four X11 records and `gtk_widget_intersect` (Owed 10/12's class); slice 14 below |
| 4342-4472 | C 4342's form feed, the “Scroll bar functions” banner (C 4343-4345) and **`xg_ignore_gtk_scrollbar`** (C 4351), the two theme `static int`s (C 4354/4355), then C 4357's `#if defined HAVE_PGTK || !defined HAVE_GTK3`: the `id_to_widget` record (C 4363-4368) and `ID_TO_WIDGET_INCR` (C 4372), **`xg_store_widget_in_map`** (C 4376-4412), **`xg_remove_widget_from_map`** (C 4417-4425), C 4430's **`xg_get_widget_from_map`**, C 4438's `#else` with **`find_scrollbar_cb`** (C 4439-4446) and C 4449's second **`xg_get_widget_from_map`**, C 4472's `#endif` | file gate (`use_gtk`) plus C's own `have_pgtk`-or-`not have_gtk3` fold; the mirror gains `GTK_IS_SCROLLBAR` and `GTK_IS_EVENT_BOX`, and the unit carries `gtk_container_forall`, `xnrealloc` and a `TYPE_MAXIMUM!` accessor (Owed 18's class); slice 15 below |
| 4473-4532 | C 4473's blank, **`update_theme_scrollbar_width`** (C 4475-4496 - the theme's scroll-bar width, measured by building a vertical scroll bar and reading its style, with C 4477/4492's `HAVE_GTK3` pair and C 4492-4494's GTK2-only width floor), **`update_theme_scrollbar_height`** (C 4499-4518 - the other axis, whose floor at C 4516 is *not* inside that pair), **`xg_get_default_scrollbar_width`** (C 4520-4524) and **`xg_get_default_scrollbar_height`** (C 4526-4531, C 4529's comment explaining why it multiplies the *width*), plus C 4532's blank | file gate (`use_gtk`); the mirror gains GTK2's `GtkObject` and `gtk_widget_style_get`, and the unit carries `GtkAdjustment`, `GTK_ADJUSTMENT`, `gtk_adjustment_new`, GTK3's `gtk_scrollbar_new`, GTK2's `gtk_hscrollbar_new`/`gtk_vscrollbar_new`, `g_object_ref_sink` and the four `XG_SB_*`/`YG_SB_*` bounds (Owed 18's class); slice 16 below |
| 4533-4594 | C 4533's `#ifndef HAVE_GTK3` and C 4534-4535's comment with **`xg_get_scroll_id_for_window`** (C 4537-4553 - slice 15's map read backwards: the X window's widget, then its index in `id_to_widget`, or C's `-1`), C 4556-4558's comment and C 4560's `#if !defined HAVE_GTK3 || defined HAVE_PGTK` with **`xg_gtk_scroll_destroy`** (C 4561-4566 - the `destroy` handler that removes the index), then C 4569's `#if defined HAVE_GTK3 && !defined HAVE_PGTK` with **`xg_scroll_bar_size_allocate_cb`** (C 4570-4592 - the GTK3/X11 `size-allocate` handler: a live configure event goes to `x_scroll_bar_configure` as is, otherwise a synthetic `GdkEvent` is filled from the allocation), plus C 4594's blank | file gate (`use_gtk`) plus C's own three folds; the mirror gains `gtk_get_current_event` and `GDK_CONFIGURE` and `GdkEventConfigure` grows GDK's `width`/`height`, and the unit declares `x_scroll_bar_configure` (Owed 18); slice 17 below |
| 4595-6753 | **the remainder** — from C 4595's `static void xg_finish_scroll_bar_creation (...)` (C 4596-4706, the shared tail of both scroll-bar constructors) through C 6753: `xg_create_scroll_bar`/`xg_create_horizontal_scroll_bar`, the position/thumb machinery, printing (`draw_page`, `print_settings`), the tool bar (`xg_create_tool_bar`, …) and `xg_initialize`/`xg_filter_key`/`xg_widget_style_updated`/`xg_im_context_*` at the end | not this slice |

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

**Correction (2026-10-15, the sixteenth slice's round): two rows above are spent and one has moved.**
The two `update_theme_scrollbar_width` / `_height` anchors leave every arm - C 4475-4496 and
C 4499-4518 are transcribed now - so the GTK3/pgtk faces read `26`/`26`/`25` (were `28`/`28`/`27`)
and the C arm `64` (was `62`): those two anchors leave that face too, and four references to Owed 6's
untranscribed GTK2 enumerators arrive, which is why the `GTK_ORIENTATION_HORIZONTAL` / `_VERTICAL` row
reads `14 + 4` rather than `3 + 1` - C 4486/4510 reach each of the two through C 122-124's macro
twice, once as the argument and once in the macro's own `if` test.  **No `E0381` appears in either
tree**, and that is the round's own repair rather than a reading: with the two names unresolved,
typeck failed in `style_changed_cb` and borrowck never ran there, so the slice fixed the latent
defect it exposed (see the slice-16 section) instead of recording a new anchor.

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
19. **The C-side names the mirrors do not carry stand in the unit** (P-72 rule 4): `EMACS_CLASS`
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

9. **Two same-program prototypes belong to header mirrors this lane cannot extend** (added
   by slice 5): **`font_is_ignored`** (font.h 861) is defined in `src/font.rs` 3592 and
   **`dupstring`** (lisp.h 5555) in `src/alloc.rs` 970, and both of those units `include!`
   their header's own mirror (`src/font.rs` 87, `src/alloc.rs` 70), so moving either
   prototype into `include/font.rs` or `include/lisp.rs` would give the defining unit a
   duplicate item (`E0428`).  Until those mirrors grow a way to carry a same-program
   declaration, `src/gtkutil.rs` keeps its own gated copies (the `src/term.rs` 384-385 shape
   that the four other `dupstring` callers already use).

10. **The menu section's names stand in the unit** (added by slice 6).  `include/gtkutil.rs`
    is a placeholder (Owed 11 and `docs/ports/include-gtkutil.md`), so the two callback
    records (`xg_menu_cb_data`, gtkutil.h 63-72; `xg_menu_item_cb_data`, gtkutil.h 75-84) and
    the key `XG_ITEM_DATA` (gtkutil.h 49) are carried in the unit at their C position, the
    shape C 619's `xg_list_node` already has and `src/pgtkmenu.rs` 147-170 / `src/xmenu.rs`
    822-846 already use.  The round that writes that mirror deletes all four copies.
    **It also has to fix a layout defect this slice found**: `src/xmenu.rs` 828-834's copy of
    `xg_menu_cb_data` opens with `f` where gtkutil.h 65 puts the `xg_list_node ptrs` head, so
    every member it carries sits at the wrong offset for the record gtkutil.c itself
    allocates (PORTING.md 10's rule); the mirror round is where one shape replaces both.
    Two more same-program declarations land the same way, each with six or more sibling
    callers already doing it: `mark_object` (lisp.h 4436; `include/lisp.rs` carries only the
    plural `mark_objects`) and `xg_pending_quit_event` (xterm.h 2012; the xterm.h mirror
    leaves the object to `src/xterm.rs`'s own `xterm_globals` module).  The four
    `input_event_bits!` arms are the per-unit accessor every terminal unit carries
    (PORTING.md P-80), and the seven GTK/GDK names the builders need
    (`GtkCallback`, `GtkCheckMenuItem`, `GtkRadioMenuItem`, `GTK_CHECK_MENU_ITEM`,
    `GTK_RADIO_MENU_ITEM`, `GDK_LEAVE_NOTIFY` and the eight menu-item constructors) cannot
    move into `include/gtk.rs` without an `E0428` in the four units that already copy them
    (`src/pgtkmenu.rs`, `src/xmenu.rs`, `src/xwidget.rs`, `src/pgtkterm.rs`).

11. **Three more same-program declarations, and a pgtk-arm mirror collision** (added by
    slice 7).  `menu_separator_name_p` (keyboard.h 473), `popup_activated`
    (dispextern.h 3879) and `set_frame_menubar` (frame.h 1881) are defined by `keyboard.c`
    and `xdisp.c` and declared by no mirror this unit reaches, so the unit declares them
    itself - the shape `src/androidmenu.rs` 91, `src/androidterm.rs` 219 and
    `src/w32fns.rs` 2645 already use for the same three names.  The mirrors that
    should carry them are `include/keyboard.rs`, `include/dispextern.rs` and
    `include/frame.rs`.  **And this slice measured a defect the dedupe artifact had hidden**:
    with `have_pgtk` on, `include/dispextern.rs` and `include/pgtkgui.rs` both define
    `Emacs_Cursor`, which is an `E0428` in that arm (the probe in the slice-7 section says
    how it was measured).  It belongs to those two mirrors, not to this unit.

12. **`GTK_IS_RADIO_MENU_ITEM` cannot join the mirror's `GTK_IS_*` family** (added by slice
    11).  `include/gtk.rs`'s family grew by three this round
    (`GTK_IS_SEPARATOR_MENU_ITEM`, `GTK_IS_CHECK_MENU_ITEM`, `GTK_IS_MENU_ITEM`), but the
    fourth macro `xg_update_submenu` asks is already copied by two units that `include!`
    that mirror (`src/pgtkmenu.rs` 229-236 under `have_pgtk`, `src/xmenu.rs` 783-786 under
    `have_x_windows`), so a mirror item would read `E0428` in whichever of them has the
    feature on.  The unit therefore carries it (the shape Owed 10 records for the menu-item
    names), and the round that consolidates the family deletes both units' copies and the
    unit's.  The two bodies also disagree today - `src/xmenu.rs`'s returns `true` where
    `src/pgtkmenu.rs`'s and this unit's spell the family's `(! ($o).is_null ())` - which is
    the same reconciliation.


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

## Slice 3 (2026-10-15, status 4.333): C 2282-2561 — the Dialog functions

**What this slice is.**  The reference's "Dialog functions" section: `get_dialog_title`
(C 2288-2316), `dialog_delete_callback` (C 2329-2334), **`create_dialog`** (C 2343-2457),
the `xg_dialog_data` record (C 2459-2465), `xg_dialog_response_cb` (C 2471-2479),
`pop_down_dialog` (C 2484-2497), `xg_maybe_add_timer` (C 2502-2519) and `xg_dialog_run`
(C 2526-2560).  Port: 2,816 -> **3,207 lines**; the table's last row splits at C 2561 - the next
live definition is C 2569's `xg_uses_old_file_dialog` (the file chooser half).

**The mirror this slice needed.**  `include/gtk.rs` gains the seventeen entry points, four records
(`GtkDialog`, `GtkBuilder`, `GtkBuildable`, `GtkMisc`, and GLib's `GMainLoop`), three checked-cast
wrappers (`GTK_DIALOG`, `GTK_MISC`, `GTK_BUILDABLE`), `GtkAlign` with its two constants and
`GTK_RESPONSE_CANCEL`; and the unit gains the edge `"gtkutil.h"` 27's
`#include "../lwlib/lwlib-widget.h"` needs (`include!("../lwlib/include/lwlib-widget.rs")`, the
`src/pgtkmenu.rs` 95 device - the `widget_value` record every function here takes).  Both are
recorded in `docs/ports/include-gtk.md`'s round section.

**Two features this slice registered.**  C 2363's `GTK_CHECK_VERSION (3, 12, 0)` and C 2419's
`(3, 14, 0)` had no feature name, so the tree could not spell the pair: `femacs/xtask/Cargo.toml`
now carries `gtk_check_version_ge_3_12` and `gtk_check_version_ge_3_14` (each with C's condition
and site in the comment) and `python3 tools/feature-coverage.py --write` re-recorded
`docs/baseline/feature-coverage.tsv`.  The two names join `gtk_check_version_ge_3_10`/`_3_22`,
which slice 1 registered (Owed 1).

**Deviations this slice adds.**

- **C's two callback *variables* bypass the `g_signal_connect!` macro.**  C 2402/2403 pass
  `deactivate_cb` and C 2442 passes `select_cb` - values of type `GCallback`, not function items -
  while the port's `g_signal_connect!` wraps its handler in `G_CALLBACK!`, which casts an *item*
  address (measured: `E0605 non-primitive cast`, this slice's first arm run).  Those three calls
  therefore stand in the macro's own expansion (`g_signal_connect_data` with the handler, `NULL`
  and `(GConnectFlags) 0` spelled out).  The sites where C writes `G_CALLBACK (…)` itself keep
  C's name in the macro's body (Deviation 12).
- **C's `(guint) -1`** (C 2515's `((guint) -1 - ms) / 1000`) is the port's `! 0 as guint`, the
  `lib/include/w32.c` device for a negative unsigned constant, and the whole bound is converted to
  `c_long` for the comparison with C's `time_t s`.
- **`xg_dialog_run`'s `struct xg_dialog_data dd`** is zeroed before C 2538-2541's four member
  stores, because Rust's definite-initialisation analysis rejects field-by-field assignment
  (PORTING.md 1.5); C's own `dd.w = 0` at C 2556 is `std::ptr::null_mut ()`.
- **`gtk_box_new` is called in the macro form** (`gtk_box_new! (…)`), as slice 2's four sites do:
  the GTK2 arm's is C 119-121's macro, which this unit carries (Deviation 13), and the GTK3 arm's
  forwards to the function in the mirror.

**Readings.**

1. **The frozen reading - `build-tu`.**  `cargo run -q -p xtask -- build-tu src/gtkutil.c` ->
   exit 0, `build/femacs/gtkutil.o` is **336 bytes** (the empty object: every item is behind
   `use_gtk`, and the frozen configuration has it `undef`).
2. **The arm readings** (the slice-2 arrangement: rustc on the materialised
   `build/femacs/units/gtkutil/_root.rs`, `--emit=metadata`, the three `--cfg` sets below).
   Arm A: **29 errors, 0 anchored in this slice's text** (the 29 are the classes slices 1-2 already
   record: `new_size_p` ×5, `redisplay_interface`, the two `emacs_menu_bar_*` names, the two
   `xg_*` definitions the remainder carries, `FRAME_MENUBAR_HEIGHT`, and the mirror-side
   `XIEventMask`/`GDK_WINDOW_XID`/`GdkScrollDirection`/`void` rows).  Arm C (GTK2): **52 errors,
   3 anchored in this slice** - all three are Owed 6's GTK2 class (`GTK_ORIENTATION_*` and the
   `gtk_hbox_new`/`gtk_vbox_new` pair C 119-121's macro calls), which slice 2's own four
   `gtk_box_new!` sites carry too.  Arm B: **543 errors, and it is an artifact, not this slice** -
   the materialiser dedupes the unit's `#[cfg (feature = "use_gtk")] include!("include/gtk.rs")`
   edge to `include/xterm.rs` 53's (root line 185's `// [xtask dedupe] … already included`), and in
   arm B that earlier edge's own gate is off, so the whole GTK surface is absent and every
   `GTK_WINDOW!`/`GtkWidget` in slices 1-3 reads as missing (the first errors are anchored in slice
   2's text).  `## Owed` 17 records it.
3. **`extract-abi.py --check`** -> 149 -> **116 `- ` rows** (9 functions retired, plus one
   `missing constant` group the mirror provided); the three new `does not mention` rows
   (`get_dialog_title`'s `switch`/`case`/`break`, and `g_callback` in `create_dialog` and
   `xg_dialog_run`) are recorded in `docs/ports/macro-body-notes.tsv`; the line-count row reads
   **52.5%** (C 6,753 vs 3,207) - the file is still open, and its `line-drift-tolerance.tsv` row
   lands with the closing slice.
4. The ten gates: all `rc=0`, 0 FAIL.

**Not claimed.**  No arm was *compiled* - there is no GTK tree on this machine (the arm readings
are rustc's name-resolution and type errors with the features a real target would set), so every
GTK/GLib prototype and every value this slice added to the mirror stays UNVERIFIED.

17. **The materialiser's `include!` dedupe is cfg-sensitive, and slice 3's arm-B reading is where it
    shows.**  `build/femacs/units/gtkutil/_root.rs` line 185 rewrites this unit's
    `#[cfg (feature = "use_gtk")] include!("include/gtk.rs")` edge into
    `// [xtask dedupe] … already included`, because `include/xterm.rs` 53 includes the same mirror
    and the unit reaches `xterm.rs` at C 34.  The deduped first edge is *that* one, so when arm B
    (`use_gtk, have_pgtk, have_window_system, have_gtk3, use_cairo, have_xinput2`) turns
    `include/xterm.rs`'s own gate off, no edge brings `include/gtk.rs` in at all: the whole GTK
    surface disappears and arm B reads **543** errors instead of the ~60 the file's Evidence (2)
    recorded at slice 2 - every `GTK_WINDOW!`/`GtkWidget`/`TRUE` in slices 1-3 among them (the first
    anchors are in slice 2's text).  The reading is an artifact of the materialiser, not of any
    slice's transcription, and the fix belongs to `femacs/xtask` (the dedupe should either keep the
    edge where its own cfg is on, or place the shared mirror's copy outside every gate); until then
    the file's arm-B reading is reported with this reason, as slice 3's section does.

18. **`gtk_widget_intersect` and the four X11 records stand in the unit** (added by slice 14).
    `gtk_widget_intersect` (gtk/gtkwidget.h, C 4333) is a name **no row anywhere in the tree
    declares**: `src/xterm.rs` 5619 and 37551 call it (C 5616-5620's `*tool_bar_p =
    gtk_widget_intersect (...)` and C 24819-24820's) and that unit's own ledger records the gap
    (`docs/ports/xterm.md` 8054: "`gtk_widget_intersect` and `G_OBJECT_GET_CLASS` have no row
    anywhere").  Its home is therefore `include/gtk.rs`, but moving it there changes another
    unit's name resolution - and that lane's recorded arm readings - in the same round this slice
    becomes its first *declarer*, so it stays at the use site until a round can measure the move.
    `GenericEvent`, `XButtonEvent`, `XGenericEventCookie` and `XIDeviceEvent` are the same class
    with a different home: `include/xlib.rs`'s 192-byte `XEvent` stand-in has no members, so the
    three records' real home is that mirror's union (the `src/xmenu.rs` 205-215 note, which is why
    `src/xmenu.rs` 276/423 and `src/xwidget.rs` 2957 carry their own copies too) - Owed 10/Owed
    12's class, and `XIDeviceEvent` adds the extra cost that its header is not on this machine, so
    its copy stays UNVERIFIED until a machine with `X11/extensions/XInput2.h` measures it.
    **Slice 15 adds one more of each class.**  `gtk_container_forall` (gtk/gtkcontainer.h, C 4464)
    is copied by `src/xwidget.rs` 504-506, whose unit `include!`s `include/gtk.rs`, so the mirror
    cannot take it; `TYPE_MAXIMUM (Window)` (C 4384) is min-max.h's token macro, which each unit
    spells for itself (`src/androidterm.rs` 7268 is the precedent) until PORTING.md gives the family
    one shape.  (`xnrealloc` looked like a third - it is called by four units and declared by none
    *visible* to them - but the lisp.h mirror does carry it inside `lisp_safe_decls` and re-export it,
    so nothing was owed: the slice-15 draft's in-unit copy was removed, and the section below records
    that reading.)

    **Slice 16 adds six more, all for the same reason (they name `GtkAdjustment`) plus one GLib
    name another unit already declares.**  `GtkAdjustment` itself (gtk/gtkadjustment.h) is declared
    by `src/pgtkterm.rs` 5759 and that unit `include!`s the mirror, so `include/gtk.rs` cannot take
    it; the same is true of `gtk_adjustment_new`, GTK3's `gtk_scrollbar_new`, GTK2's
    `gtk_hscrollbar_new`/`gtk_vscrollbar_new` and the `GTK_ADJUSTMENT` cast macro - all six name that
    record.  `g_object_ref_sink` (gobject/gobject.h 509-510) is the GLib case:
    `src/xsettings.rs` 337-338 declares it and that unit includes the mirror too.  The round's
    regression risk is exactly what the dupe gate measures, and it read `16 duplicate
    declaration(s), 16 ratchet row(s), 0 failure(s)` on this round's text for that reason.
    The four `gtkutil.h` 38-42 bounds (`XG_SB_MIN`/`XG_SB_MAX`/`YG_SB_MIN`/`YG_SB_MAX`) are the
    placeholder-mirror class this ledger's Owed 10/Owed 11 already record; their
    `XG_SB_RANGE`/`YG_SB_RANGE` siblings stay undeclared until the slice that reads them.
    **Slice 17 adds one more, and it is a same-program declaration rather than a third-party one**:
    `x_scroll_bar_configure` (xterm.h 1848) is defined by `xterm.c` 18058-18084 / `src/xterm.rs`
    29944 and declared by no mirror this unit reaches, so it stands at the use site under its
    definition's gate; `include/xterm.rs` is its home, and that unit's own note already names this
    file's C 4579/4590 as its two callers.

## Slice 4 (2026-10-15, status 4.334): C 2562-2843 — the File dialog functions

**What this slice is.**  The reference's "File dialog functions" section: `xg_uses_old_file_dialog`
(C 2568-2576), the `xg_get_file_func` typedef (C 2579), `xg_get_file_name_from_chooser`
(C 2584-2588), `xg_toggle_visibility_cb` (C 2593-2600), `xg_toggle_notify_cb` (C 2609-2629),
**`xg_get_file_with_chooser`** (C 2642-2737), the `HAVE_GTK_FILE_SELECTION_NEW` pair
(`xg_get_file_name_from_selector`, C 2744-2749, and `xg_get_file_with_selection`, C 2761-2787) and
`xg_get_file_name` (C 2803-2842).  Port: 3,207 -> **3,592 lines**; the table's last row splits at
C 2843 - the next live definition is C 2848's `xg_font_filter` (the font-chooser half).

**The mirror this slice needed.**  `include/gtk.rs` gains the fifteen entry points
(`gtk_file_chooser_dialog_new` with C's variadic tail, the six `gtk_file_chooser_*` calls,
`gtk_check_button_new_with_label`, the two `gtk_toggle_button_*` calls, `gtk_check_version`, the
two `g_signal_handlers_*_by_func` entries and, under the new feature, the four GTK2
`gtk_file_selection_*` calls), the four records (`GtkFileChooser`, `GtkToggleButton`,
`GtkFileChooserDialog` and - under `have_gtk_file_selection_new` - `GtkFileSelection` with C 2780's
one read member), the three checked-cast wrappers (`GTK_FILE_CHOOSER`, `GTK_TOGGLE_BUTTON`,
`GTK_FILE_SELECTION`), `GtkFileChooserAction` with its three values and `GTK_RESPONSE_OK`.  Every
GTK/GLib prototype and value stays **UNVERIFIED** (no GTK tree here), and `GtkFileSelection`'s
member has no measured offset (Owed 6's class).

**The feature this slice registered.**  C 2571/2739/2820's `#ifdef HAVE_GTK_FILE_SELECTION_NEW` had
no name, so the tree could not spell the arm: `femacs/xtask/Cargo.toml` now carries
`have_gtk_file_selection_new` (C's condition and its three sites in the comment) and
`python3 tools/feature-coverage.py --write` re-recorded `docs/baseline/feature-coverage.tsv`
(6 uses, one definition).  The gate table above gains its row.

**Deviations this slice adds.**

- **`GParamSpec` keeps its opaque head**, so C 2612's `arg1->name` read stands in the port's text
  and the arm reports `no field name on type GParamSpec` - exactly the class C 798-800's
  `gsty->bg[…]`/`fg[…]` reads carry for `GtkStyle` (Owed 6).  The member is *not* the record's
  first (`GParamSpec` opens with its `GTypeInstance`), so inventing an offset would be a wrong
  layout rather than a conservative one (P-88 (2)).
- **C's `gpointer cb` reaches the prototype's `GCallback` parameter** by the
  `core::mem::transmute::<gpointer, GCallback>` the file already uses for the reverse direction
  (C 2344's device), and C 2620's `!!visible != !!toggle_on` is the port's
  `(visible != 0) != (toggle_on != 0)` (C's `!!` on an int).
- **C 2652's `UNINIT` declarator** is a zeroed local plus C's own guard (PORTING.md 1.5),
  C 2692-2694's three adjacent literals are one `c"…"` (Rust has no adjacent-string
  concatenation), and C 2491-like `x_gtk_…` Lisp booleans enter the variadic `g_object_set` and
  `gtk_toggle_button_set_active` as `as gboolean` (C's `bool` promotion).
- **`g_signal_connect!`'s data argument** needs C's implicit pointer conversion spelled out
  (`filewin as gpointer`, `wtoggle as gpointer`), the same shape slice 2's sites use.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings.**  Arm A: **30 errors, 1 anchored in this slice** (the `GParamSpec.name`
   read above).  Arm A plus `have_gtk_file_selection_new`: **30 errors, the same single anchor** -
   the new arm adds no class.  Arm C (GTK2): **60 errors, 2 anchored here** (the same read plus
   `GTK_ORIENTATION_VERTICAL`, Owed 6's GTK2 class).  Arm B: **647 errors, the materialiser's
   dedupe artifact** (Owed 17) - the first anchors are in slice 2's text.
3. **`extract-abi.py --check`** -> 116 -> **107 `- ` rows** (9 more functions retired); the two new
   `does not mention` rows (`xg_uses_old_file_dialog`'s `0`, and `g_callback`/`uninit` in
   `xg_get_file_with_chooser`) are recorded in `docs/ports/macro-body-notes.tsv`; the line-count
   row reads **46.8%** (C 6,753 vs 3,592) - the file is open and its tolerance row lands with the
   closing slice.
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled (no GTK tree on this machine); every GTK/GLib prototype and
value this slice added is UNVERIFIED.

## Slice 5 (2026-10-15, status 4.335): C 2844-3002 — the GTK font chooser

**What this slice is.**  The reference's "GTK font chooser" section: `xg_weight_to_symbol`
(C 2852-2870, whose chain C 2859-2861 wraps one step in `PANGO_VERSION_CHECK (1, 36, 7)`),
`xg_style_to_symbol` (C 2872-2879), the file-scope `x_last_font_name` (C 2884), `xg_font_filter`
(C 2887-2898) and **`xg_get_font`** (C 2911-3001) - the GTK3 `PangoFontDescription` path and, under
C 2952/2985's `#else`, the GTK2 `gtk_font_selection_dialog_get_font_name` path.  Port: 3,592 ->
**3,855 lines**; the table's last row splits at C 3002 - the next live text is C 3006's "Menu
functions" banner.

**A stale reading this slice corrects.**  The header's port line above and Evidence ②S2 both read
`4200 lines`; the unit is **3,855 lines** and the materialised root **3,862 lines** on this
revision, and neither file has ever been that long (the port was 2,816 lines entering slice 3,
3,207 entering slice 4 and 3,592 entering this one, and text is only ever added).  Those two
figures are therefore wrong rather than old; they are left in place here so the record shows both
readings.

**The mirror this slice needed.**  `include/gtk.rs` gains the Pango surface the GTK3 arm names -
four records (`PangoFontFamily`, `PangoFontFace`, `PangoFontDescription`, `GtkFontChooser`), the
`PangoWeight`/`PangoStyle` typedefs with their fifteen enumerators, `PangoFontFilterFunc` and
GLib's `GDestroyNotify`, nine Pango entry points, and the four `gtk_font_chooser_*` calls with the
`GTK_FONT_CHOOSER` checked cast and a `gtk_font_chooser_dialog_new` forwarding macro (C 2923 calls
that name in both arms, so the port's uniform spelling is the macro form - the
`gtk_box_new`/`gtk_scrollbar_new` shape) - plus the GTK2 font trio (`GtkFontSelectionDialog`,
`gtk_font_selection_dialog_new`, `..._set_font_name` and `..._get_font_name`, which C 110-117's own
macros in this unit forward to).  Every Pango/GTK prototype stays **UNVERIFIED**, because neither
tree exists here (`ls -d /opt/homebrew/include/pango-1.0` -> absent and `pkg-config pango` fails,
the GTK rank); every Pango enumerator is a **placeholder**, so `xg_weight_to_symbol`'s twelve steps
stay distinguishable instead of carrying numbers from memory (the `src/xwidget.rs` 495-520 rule).
`GDestroyNotify` is GLib's, so it quotes `gtypes.h` 140 and its line.

**The feature this slice registered.**  C 2859's `#if PANGO_VERSION_CHECK (1, 36, 7)` had no name,
so the tree could not spell the arm: `femacs/xtask/Cargo.toml` now carries
`pango_version_ge_1_36_7` (C's condition and its one site in the comment) and
`python3 tools/feature-coverage.py --write` re-recorded `docs/baseline/feature-coverage.tsv`.  The
gate table above gains its row.

**Deviations this slice adds.**

- **C's chained conditional is the port's `if`/`else if` chain**, and because C 2859-2861 removes
  one step of that chain, `xg_weight_to_symbol` stands as the tree's two-`fn` arm pair (Deviation
  3's shape) rather than as a `#[cfg]`ed fragment inside one expression; each arm keeps C's chain
  whole and every step carries C's own `/* 100 */`-style comment.
- **Two same-program declarations stand in this unit.**  C 2895 calls `font_is_ignored` (font.h
  861) and C 2980 calls `dupstring` (lisp.h 5555); the port defines the first in `src/font.rs` 3592
  and the second in `src/alloc.rs` 970, and both of those units `include!` their header's mirror
  (`src/font.rs` 87, `src/alloc.rs` 70), so the prototype cannot move into `include/font.rs` or
  `include/lisp.rs` without a duplicate item in the defining unit.  The declarations therefore
  stand here, each gated like its call site (the `src/term.rs` 384-385 shape that the four other
`dupstring` callers already use), and the Owed above names the mirror work.
- **C 2927's callback is the one place C's implicit conversion changes a signature.**  GTK's
  `PangoFontFilterFunc` takes two non-const pointers where C's own `xg_font_filter` declarator
  makes them const, so the port spells the conversion as the function's address plus a pointer
  `transmute` (PORTING.md 1.4; the `G_CALLBACK!` shape the mirror already carries).  C 2892's own
  cast of the parameter is the port's `as *mut PangoFontFamily`, and C 2914's unassigned `w` is the
  port's deferred `let` (C assigns it at C 2923 on every path).

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings.**  Arm A: **30 errors, none anchored in this slice** (the anchors are
   slices 1-4's, e.g. Owed 6's `GParamSpec.name`); the same arm with `pango_version_ge_1_36_7` on:
   **30 errors, the same anchor set**, so both arms of the new pair compile.  Arm C (GTK2):
   **60 errors, none here**; arm C plus `have_freetype` - the GTK2 font arm, C 2985-2995's text,
   which is a reading this slice added because the prescribed arm C leaves the whole
   `HAVE_FREETYPE` half switched off: **60 errors, none here** too.  Arm B: **717 errors, 43 of
   them in this slice**, every one of the "cannot find `GtkWidget`/`gpointer`/`TRUE`/`gboolean`"
   class - the materialiser's dedupe artifact (Owed 17), which drops the whole GTK surface in that
   arm; the first anchors are in slice 2's text.  473 anchor lines in all, 43 in this slice.
3. **`extract-abi.py --check`** -> 105 -> **100 `- ` rows** (five retired: the four functions
   `xg_get_font`, `xg_weight_to_symbol`, `xg_style_to_symbol`, `xg_font_filter`, and the
   file-scope object `x_last_font_name`, which closes one of the `missing file-scope object` rows
   the earlier slices carried); no new `does not mention` row was needed; the line-count row reads
   **42.9%** (C 6,753 vs 3,855) - the file is still open, and its `line-drift-tolerance.tsv` row
   lands with the closing slice.
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled - there is no GTK and no Pango tree on this machine - so
every Pango/GTK prototype and every Pango enumerator this slice added is UNVERIFIED, and the twelve
weight and three style values remain placeholders owed to a Pango host.

## Slice 6 (2026-10-15, status 4.336): C 3003-3315 — the menu section's opening

**What this slice is.**  The reference's "Menu functions." banner and everything up to
`make_menu_item`'s closing brace: the `MENU_ITEM_NAME` macro (C 3014), the two file-scope list
heads `xg_menu_cb_list`/`xg_menu_item_cb_list` (C 3019/3023), the callback-data trio
(`make_cl_data`, C 3035-3053; `update_cl_data`, C 3066-3078; `unref_cl_data`, C 3083-3095), the
GC hook **`xg_mark_data`** (C 3099-3142), the three widget callbacks
(`menuitem_destroy_callback`, `menuitem_highlight_callback`, `menu_destroy_callback`), and the two
menu-item builders `make_widget_for_menu_item` (C 3206-3235) and **`make_menu_item`**
(C 3251-3300).  Port: 3,855 -> **4,479 lines**; the table's last row splits at C 3315 - the next
live definition is C 3316's `xg_create_one_menuitem`.

**The declarations this section needed.**  `include/gtkutil.rs` is still the placeholder its own
ledger records, so the two callback records the section allocates and walks (gtkutil.h 63-72 and
75-84) and the `XG_ITEM_DATA` key (gtkutil.h 49) are carried in the unit at their C position -
the shape C 619's `xg_list_node` already has.  `mark_object` (lisp.h 4436) and
`xg_pending_quit_event` (xterm.h 2012) are same-program declarations the mirrors do not carry,
and the four `input_event_bits!` arms are the per-unit accessor every terminal unit writes.  The
seven GTK/GDK names the builders need cannot go into `include/gtk.rs`, because the four units that
already copy them (`src/pgtkmenu.rs`, `src/xmenu.rs`, `src/xwidget.rs`, `src/pgtkterm.rs`) all
`include!` that mirror and would read `E0428`; they stand in the unit.  All of it is Owed 10,
which also records the layout defect this slice found in `src/xmenu.rs`'s copy of
`xg_menu_cb_data`.

**Deviations this slice adds.**

- **C's two `for` heads are the port's `while`** with C's update step at the loop's end
  (PORTING.md 2.2), each written at C's own line - the `iter = iter->next` of C 3105/3108.
- **C 3258's and C 3182's `0`** are the port's `std::ptr::null_mut ()` (PORTING.md 1.5's
  pointer-null rule), which is why both functions carry a `does not mention 0` row in
  `docs/ports/macro-body-notes.tsv`; C 3271/3286's two `*group = NULL` stores are the same shape.
- **C 3274/3282's `item->selected`** is the `widget_value` mirror's `bool`, so it enters the
  variadic-free `gtk_check_menu_item_set_active` as `as gboolean` (C's own usual conversion, the
  `x_gtk_*` row's class).
- **C 3183's cast** is `core::mem::transmute::<GCallback, GtkCallback>`, the reverse direction of
  the `gpointer`-to-`GCallback` device C 2344's site already uses, and C 3184's `(*func) (…)` is
  the port's `(func.unwrap ()) (…)` as at C 2838.
- **C 3171's uninitialised `GdkEvent ev`** is the port's zeroed local (PORTING.md 1.5), and its one
  written member reaches `gtk_get_event_widget` through `&raw mut`.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings.**  Arm A: **30 errors, none anchored in this slice** (the same anchor set as
   slice 5's, slices 1-4's text); arm A plus `gtk_check_version_ge_3_14` - which lights C
   3218-3226's other arm: **30 errors, the same set**.  Arm C (GTK2): **64 errors, 1 anchored
   here** - C 3213's `GTK_ORIENTATION_HORIZONTAL`, the mirror item gated `have_gtk3` is configured
   out in that arm (rustc's note says so), i.e. Owed 6's GTK2 class, which the same `gtk_box_new!`
   call sites in slices 1-2 already carry; arm C plus `have_freetype`: the same **64 and the same
   single anchor**.  Arm B: **790 errors, 51 of them in this slice**, every one of the
   `cannot find GtkWidget/gpointer/TRUE/gboolean` class - the materialiser's dedupe artifact
   (Owed 17), whose first anchors are in slice 2's text.
3. **`extract-abi.py --check`** -> 100 -> **87 `- ` rows** (ten retired: `MENU_ITEM_NAME`,
   `xg_mark_data`, `make_cl_data`, `update_cl_data`, `unref_cl_data`, `make_menu_item`,
   `make_widget_for_menu_item`, `menuitem_destroy_callback`, `menuitem_highlight_callback`,
   `menu_destroy_callback`; two new `does not mention 0` rows recorded in
   `macro-body-notes.tsv`); the line-count row reads **33.7%** (C 6,753 vs 4,479) - the file is
   still open.
4. **One reading had to be redone after the round lost the file to its own tooling.**  An `awk`
   rewrite whose program lacked a default `print` truncating `src/gtkutil.rs` to four lines; the
   text was recovered from the round's own materialised `build/femacs/units/gtkutil/_root.rs`
   (whose tail region is this unit's text at a fixed `+7`-line offset once the include-edge
   rewrites of the top of the file are accounted for), and the recovery is *proved* rather than
   asserted: the first 3,855 recovered lines are byte-identical to the previous commit's file
   (`head -n 3855 recovered | diff - <commit's file>` is empty), the recovered file is 4,476 lines
   as `root - 7` predicts, and the three readings above (frozen object size, five arm counts,
   `--check`) reproduce exactly.  Including the two retries this cost, the round spent **4
   `build-tu` runs** - one above the lane's three-per-slice budget - and 15 arm runs (the
   five-arm set, three times: before and after the duplicate removal, and after the recovery).
5. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled - there is no GTK or Pango tree on this machine - so the
seven GTK/GDK names this slice added are UNVERIFIED and `GDK_LEAVE_NOTIFY`'s value is a
placeholder; `GSList` is the exception and is real, because GLib is on this machine (its four
layout assertions are checked whenever arm A compiles).

## Slice 7 (2026-10-15, status 4.337): C 3302-3533 — the menu builders

**What this slice is.**  From C 3302-3314's comment above `xg_create_one_menuitem` (moved out of
slice 6's row, where it never belonged) through C 3533's blank: **`xg_create_one_menuitem`**
(C 3316-3363), `menu_bar_button_pressed_cb` inside `#ifdef HAVE_PGTK` (C 3366-3381), the
`XG_FRAME_DATA` key (gtkutil.h 46, first used at C 3459) and **`create_menus`** (C 3404-3532, with
C 3384-3402's comment) - the recursive builder that walks a `widget_value` list into a GTK menu.
Port: 4,479 -> **4,798 lines**; the table's last row splits at C 3533 - the next text is
C 3534-3547's comment above `xg_create_widget`.

**The mirror this slice needed.**  `include/gtk.rs` gains its menu-shell surface: `GtkMenuItem`
with the `GTK_MENU_ITEM` checked cast, and the five entry points C 3424/3441/3498/3523/3527 calls
(`gtk_menu_new`, `gtk_menu_bar_new`, `gtk_separator_menu_item_new`, `gtk_menu_item_set_submenu`,
`gtk_menu_shell_append`).  These are **fresh names** - none of the four units that already copy
the menu-item names slice 6 had to leave in the unit declares any of them (the tree-wide counts
were zero), which is what lets them live in the mirror where P-90 (2) puts them.  Every prototype
is `UNVERIFIED` (no GTK tree here).

**The probe that turned arm B's artifact into a reading.**  Every `have_pgtk` arm reads the
materialiser's dedupe artifact (Owed 17) because the unit's own `include!("include/gtk.rs")` edge
is deduped away in favour of `include/xterm.rs` 55's, and the unit reaches `xterm.rs` only under
`all (use_gtk, not (have_pgtk))` (C 34 is inside C 31-34's `#else`): with `have_pgtk` on, **no edge
brings `include/gtk.rs` in at all**.  This slice ran a probe outside the tree - a copy of the
materialised `_root.rs` with that one edge restored - and the pgtk arm drops from **810 `^error`
matches to 10**: nine errors plus rustc's `aborting` line.  All nine are the pre-existing mirror
gaps (`emacs_menu_bar_parent_class`/`emacs_menu_bar_get_type`, `redisplay_interface`,
`update_theme_scrollbar_width`/`_height` and two `update_frame_tool_bar` sites), **none in this
slice** - so C 3366-3381 and the two C 3446-3474 arms do compile.  The same probe surfaced one real
defect the artifact had hidden: an `E0428` for `Emacs_Cursor`, defined by both
`include/dispextern.rs` and `include/pgtkgui.rs` (Owed 11).  The patch must not be applied to the
non-pgtk arms - there `xterm.rs`'s own edge is live and the restored one would duplicate the whole
surface (measured: 708 errors).

**Deviations this slice adds.**

- **C 3358-3359's `g_signal_connect` passes the variable `select_cb`**, which the port's
  `g_signal_connect!` macro would wrap a second time in `G_CALLBACK!` (that macro needs a function
  item: `E0605`, measured in slice 3), so that call stands in the macro's own expansion as
  `g_signal_connect_data` with the `GCallback` argument spelled directly; C 3467-3473's two
  `deactivate_cb` arms are the same shape, and the three function-item sites keep the macro.  The
  two functions therefore carry a `does not mention g_callback` row in `macro-body-notes.tsv`.
- **C 3504's `item->contents ? 0 : select_cb`** is the port's `if`/`else` over C's null
  `GCallback` (`{ select_cb } else { None }`), and C 3513-3522's recursive call passes C's three
  `0` arguments as the port's `false`, `false` and null pointer plus the null `name`.
- **C 3477's `for` head** is the port's `while` with C's `item = item->next` at the loop's end
  (PORTING.md 2.2), as the two GC walks of slice 6 are.
- **C 3374's `!popup_activated ()`** is the port's `== 0` over the `int` dispextern.h 3879
  declares, and C 3380's `false` (a `stdbool` value in the C text) is the `gboolean` `FALSE` the
  function returns.
- Three same-program declarations (`menu_separator_name_p`, `popup_activated`,
  `set_frame_menubar`) stand in the unit - Owed 11.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings.**  Arm A: **30 errors, none anchored in this slice**; arm A plus
   `gtk_check_version_ge_3_14`: **30, the same set**.  Arm C (GTK2): **64 errors, none in this
   slice**; arm C plus `have_freetype`: **64, the same set** - both sets are slice 6's readings
   unchanged, so this slice's three functions add no anchor to either.  Arm B (and the same set
   plus `have_x_windows`, which does not change it): the dedupe artifact, 39 anchors in this slice.
   The probe above is the arm-B/pgtk reading that replaces them: **9 errors, none here**.
3. **`extract-abi.py --check`** -> 87 -> **84 `- ` rows** (three retired: `xg_create_one_menuitem`,
   `menu_bar_button_pressed_cb`, `create_menus`; two new `does not mention g_callback` rows
   recorded in `macro-body-notes.tsv`); the line-count row reads **29.0%** (C 6,753 vs 4,798).
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled against a real GTK tree - the seven names this slice added to
the mirror are UNVERIFIED prototypes - and the pgtk text's reading comes from the probe described
above rather than from a tree build, because no `have_pgtk` arm of the materialised unit can see
the GTK surface at all (Owed 17).

## Slice 8 (2026-10-15, status 4.338): C 3534-3640 — `xg_create_widget` and the label helpers

**What this slice is.**  C 3534-3547's comment and the four functions under it: **`xg_create_widget`**
(C 3549-3597, the dispatcher that builds a dialog, a popup menu or a menu bar from C's `type`
string), `xg_get_menu_item_label` (C 3601-3606), `xg_item_label_same_p` (C 3610-3623) and
`xg_destroy_widgets` (C 3627-3639).  Port: 4,799 -> **4,950 lines**; the table's last row splits at
C 3640 - the next text is C 3641-3651's comment above `xg_update_menubar`.

**The mirror this slice needed.**  `include/gtk.rs` gains **`gtk_label_get_label`**
(gtk/gtklabel.h, C 3605's read): it is the first `gtk_label_*` *accessor* this tree needs - the
mirror carried the `gtk_label_new` constructor alone - and no other unit declares it, so P-90 (2)
puts it there.  The prototype is `UNVERIFIED` (no GTK tree on this machine).  Everything else the
four functions call was already in place: `create_dialog` and `xg_set_cursor` are this unit's own
(C 2343/C 603), `create_menus` is slice 7's, and the GTK entry points
(`gtk_window_set_transient_for`, `gtk_window_set_destroy_with_parent`, `gtk_window_set_modal`,
`gtk_widget_realize`, `gtk_label_get_label` aside) plus the `GTK_LABEL`/`GTK_WINDOW`/`GTK_WIDGET`
casts, `XG_BIN_CHILD` (C 127, the unit's) and `g_list_next` are the mirror's.

**Deviations this slice adds.**

- **C's `type` parameter is the port's `type_`** (a Rust keyword), as the mirror's `gtk_window_new`
  already spells its `GtkWindowType` parameter.
- **C 3614's `witem ? xg_get_menu_item_label (witem) : 0`** is the port's `if`/`else` over the same
  null test with C's `0` as the null `const char *`, and C 3616-3618's nested conditional is an
  `if`/`else` whose two arms are C's own `strcmp` test and `!utf8_label` fall-through.
- **C 3632's `for` head** is the port's `while` with C's `g_list_next` step at the loop's end
  (PORTING.md 2.2).
- C 3555/3556's two `bool` locals are the port's `bool` from C's `strcmp (…) == 0`, C 3554's
  `= 0` is the null pointer, C 3570-3579's recursive call passes C's two `0` arguments as the
  null `topmenu` and `cl_data`, and C 3592's `fprintf` carries C's format string through
  `lib/include/stdio.rs`.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings.**  Arm A: **30 errors, none anchored in this slice**; arm A plus
   `gtk_check_version_ge_3_14`: **30, the same set**.  Arm C (GTK2): **64 errors, none here**;
   arm C plus `have_freetype`: **64, the same set**.  Arm B (and the same set plus
   `have_x_windows`): the dedupe artifact again, 19 anchors in this slice.  The out-of-tree probe
   slice 7 introduced reproduces its reading on this revision: **10 `^error` matches, none in this
   slice** - so `xg_create_widget`'s three branches compile in that arm too.
3. **`extract-abi.py --check`** -> 84 -> **80 `- ` rows** (four retired: `xg_create_widget`,
   `xg_get_menu_item_label`, `xg_item_label_same_p`, `xg_destroy_widgets`; no new
   `does not mention` row was needed); the line-count row reads **26.7%** (C 6,753 vs 4,950).
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled against a real GTK tree, so `gtk_label_get_label`'s
prototype is `UNVERIFIED`; the pgtk arm's reading is the probe's, not a tree build's (Owed 17).

## Slice 9 (2026-10-15, status 4.339): C 3641-3831 — `xg_update_menubar`

**What this slice is.**  C 3641-3651's comment and **`xg_update_menubar`** (C 3653-3830, 178
lines) - the recursive walk that reconciles a live GTK menu bar with a `widget_value` list, with
its four outcomes (remove an item, rename one, insert one, move one) and the five early exits that
end the recursion.  Port: 4,950 -> **5,178 lines**; the table's last row splits at C 3831 - the
next text is C 3832-3835's comment above `xg_update_menu_item`.

**The mirror this slice needed.**  `include/gtk.rs` gains **`gtk_menu_shell_insert`**
(gtk/gtkmenushell.h, C 3673/3794/3814's three inserts), **`gtk_label_set_text`** (gtk/gtklabel.h,
C 3763's relabel - the second `gtk_label_*` accessor this tree needs) and **`g_object_notify`**
(gobject.h 499-501, C 3764's notification).  The first two are fresh names and come out
`UNVERIFIED`; the third is GLib's and GLib is on this machine, so it quotes its line and its
signature is real.  Everything else the function calls was already in place: `xg_destroy_widgets`
and `xg_item_label_same_p` are slice 8's, `create_menus` and `xg_create_one_menuitem` are slice
7's, and `gtk_container_remove`/`gtk_container_get_children`/`g_object_ref`/`g_object_unref`/
`g_list_free`/`gtk_menu_item_set_submenu`/`gtk_widget_destroy` plus the four cast macros are the
mirror's.

**Deviations this slice adds.**

- **C 3737/3799/3820's `while (nr-- > 0)`** keeps C's post-decrement inside the condition, since
  that decrement also happens on the test that fails (the `src/xfns.rs` 4526-4528 shape):
  `while { nr -= 1; nr + 1 > 0 }`.
- **C 3736/3798/3819's chained `*list = iter = gtk_container_get_children (…)`** is one call with
  two stores; Rust has no chained assignment, so the port writes the store and then reads it back
  into `iter` (same value, same order).
- **C 3708-3710 and C 3718's two `for` heads** are the port's `while` with C's update step at the
  loop's end (PORTING.md 2.2), C 3702/3703's `bool … = 0` locals are the port's `false`, C 3674's
  `""` is the one-element `c""`, C 3684's `0, 1, …, 0` are `false`, `true`, … and the null `name`,
  and C 3730/3812's `g_object_ref`/C 3816's `g_object_unref` take C's `GObject *` through the
  port's `gpointer`.
- The function is recursive and `static`, like slice 7's `create_menus`, so it is a plain
  `unsafe fn` whose own name is in scope at C 3828's tail call.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings.**  Arm A: **30 errors, none anchored in this slice**; arm A plus
   `gtk_check_version_ge_3_14`: **30, the same set**.  Arm C (GTK2): **64 errors, none here**;
   arm C plus `have_freetype`: **64, the same set**.  Arm B (and the same set plus
   `have_x_windows`): the dedupe artifact, 40 anchors in this slice.  The out-of-tree probe reads
   **10 `^error` matches, none in this slice** - so this slice's 228 lines, the `while (nr-- > 0)`
   shapes included, compile in that arm too.
3. **`extract-abi.py --check`** -> 80 -> **79 `- ` rows** (one retired: `xg_update_menubar`; no
   new `does not mention` row was needed); the line-count row reads **23.3%** (C 6,753 vs 5,178).
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled against a real GTK tree, so `gtk_menu_shell_insert`'s and
`gtk_label_set_text`'s prototypes are `UNVERIFIED`; the pgtk arm's reading is the probe's (Owed 17).

## Slice 10 (2026-10-15, status 4.340): C 3832-3962 — the three item updaters

**What this slice is.**  C 3832-3835's comment and **`xg_update_menu_item`** (C 3837-3945, 109
lines - the one that rewrites a single menu item's label, key and sensitivity and re-wires its
select callback), plus the two one-statement updaters under it, `xg_update_toggle_item`
(C 3949-3953) and `xg_update_radio_item` (C 3957-3961), each with its own comment.  Port: 5,178 ->
**5,352 lines**; the table's last row splits at C 3962 - the next text is C 3963-3973's comment
above `xg_update_submenu`.

**The mirror this slice needed.**  `include/gtk.rs` gains **`GTK_IS_BOX`** (gtk/gtk.h; C 3859's
test of `XG_BIN_CHILD`'s result - kept as this mirror's `GTK_IS_*` family keeps its four, all of
them a null test with the `G_TYPE_CHECK_INSTANCE_TYPE` chain owed) and
**`gtk_widget_get_sensitive`** (gtk/gtkwidget.h; C 3915/3917's two reads).  Both are fresh names
and both are `UNVERIFIED`.  Everything else was in place: `gtk_container_get_children`/
`gtk_container_remove`/`gtk_container_add`/`g_object_ref`/`g_object_unref`/`g_list_free`/
`g_signal_handler_disconnect`/`gtk_widget_set_sensitive` are the mirror's, `gtk_label_get_label`
is slice 8's, `gtk_label_set_text` and `g_object_notify` are slice 9's, and
`make_widget_for_menu_item`/`XG_BIN_CHILD`/`GTK_CHECK_MENU_ITEM` are the unit's own.

**Deviations this slice adds.**

- **C 3933-3934's `g_signal_connect` passes the *variable* `select_cb`**, so that call stands in
  the `g_signal_connect!` macro's own `g_signal_connect_data` expansion (the macro needs a
  function item: `E0605`, measured in slice 3) - the third site of that class after C 3359 and
  C 3469 - and the function therefore carries a `does not mention g_signal_connect` row in
  `docs/ports/macro-body-notes.tsv`.
- **C 3863-3864/3888-3889's `list->data`/`list->next->data`** are the `GList` mirror's two
  members read through the pointer C walks by hand, and C 3870/3873's `g_object_ref`/
  `g_object_unref` take C's `GObject *` through the port's `gpointer`.
- **C 3900/3906's two tests** keep C's `strcmp` comparisons and spell C's `! old_key`/`! old_label`
  as null checks; C 3915/3917's `gtk_widget_get_sensitive` results are the port's `!= 0`/`== 0`
  (the `int`-result idiom this file already uses), and C 3952/3960's `val->selected` is the
  `widget_value` mirror's `bool` entering a `gboolean` parameter as `as gboolean`.
- C 3928's `val->call_data && ! val->contents` is the port's pair of null checks, and
  C 3931/3936's `select_id` tests are `== 0`/`!= 0` over a `gulong`.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings.**  Arm A: **30 errors, none anchored in this slice**; arm A plus
   `gtk_check_version_ge_3_14`: **30, the same set**.  Arm C (GTK2): **64 errors, none here**;
   arm C plus `have_freetype`: **64, the same set**.  Arm B (and the same set plus
   `have_x_windows`): the dedupe artifact.  The out-of-tree probe reads **10 `^error` matches,
   none in this slice**.
3. **`extract-abi.py --check`** -> 79 -> **77 `- ` rows** (three retired: the three functions;
   one new `does not mention g_signal_connect` row recorded in `macro-body-notes.tsv`); the
   line-count row reads **20.7%** (C 6,753 vs 5,352).
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled against a real GTK tree, so the two prototypes this slice
added are `UNVERIFIED`; the pgtk arm's reading is the probe's (Owed 17).

## Slice 11 (2026-10-15, status 4.341): C 3963-4095 — `xg_update_submenu`

**What this slice is.**  C 3963-3973's comment and **`xg_update_submenu`** (C 3975-4094, 120
lines) - the recursive reconciler that walks a submenu's live children against a `widget_value`
list, breaks at the first structural difference, tracks the first radio button of a group so a
mismatch rebuilds the whole group, and hands any added tail to `create_menus`.  Port: 5,352 ->
**5,526 lines**; the table's last row splits at C 4095 - the next text is C 4096-4104's comment
above `xg_modify_menubar_widgets`.

**The mirror this slice needed.**  `include/gtk.rs` gains the three type-check macros
**`GTK_IS_SEPARATOR_MENU_ITEM`**, **`GTK_IS_CHECK_MENU_ITEM`** and **`GTK_IS_MENU_ITEM`** (each
kept as that mirror's `GTK_IS_*` family keeps its four: a null test with the
`G_TYPE_CHECK_INSTANCE_TYPE` chain owed) and **`gtk_menu_item_get_submenu`** (gtk/gtkmenuitem.h,
C 4038's read).  All four are fresh names.  The function's **fourth** type-check macro,
`GTK_IS_RADIO_MENU_ITEM`, could **not** go there - two units that `include!` the mirror already
copy it - so it stands in the unit at the function's position (the general case is now **Owed 12**,
which also records that `src/xmenu.rs`'s copy returns `true` where the other two spell the
family's body).  Everything else was in place: `BUTTON_TYPE_NONE`/`BUTTON_TYPE_TOGGLE`/
`BUTTON_TYPE_RADIO` are the lwlib mirror's, `xg_destroy_widgets` is slice 8's, the three updaters
are slice 10's, `create_menus` is slice 7's, and `gtk_container_get_children`/`g_object_ref`/
`gtk_widget_destroy`/`g_list_free`/`gtk_menu_item_set_submenu`/`GTK_WIDGET`/`GTK_MENU_ITEM` are the
mirror's.

**Deviations this slice adds.**

- **C 3993/3995's two-part `for` head and step** (`cur = val, iter = list` and
  `iter = g_list_next (iter), cur = cur->next`) are written at their own lines around the port's
  `while` condition, so both stores happen in C's order (PORTING.md 2.2) - which matters here,
  because every `break` leaves through them.
- **C 4006's `first_radio = 0`** and C 4043/4084-4088's four `0` arguments are the port's null
  pointers, which is why the function carries a `does not mention 0` row in
  `macro-body-notes.tsv`.
- C 3984-3988's five locals keep C's two initialisers (`list = 0`, `first_radio = 0`) as nulls and
  its three uninitialised declarators as Rust declarators; C 3997's `GTK_WIDGET (iter->data)` is
  the mirror's cast over `GList`'s `gpointer` member; C 4042's `g_object_ref` takes C's
  `GObject *` through the port's `gpointer`; and the function is recursive and `static` like slices
  7 and 9, so a plain `unsafe fn`.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings.**  Arm A: **30 errors, none anchored in this slice**; arm A plus
   `gtk_check_version_ge_3_14`: **30, the same set**.  Arm C (GTK2): **64 errors, none here**;
   arm C plus `have_freetype`: **64, the same set**.  Arm B (and the same set plus
   `have_x_windows`): the dedupe artifact.  The out-of-tree probe reads **10 `^error` matches,
   none in this slice**.
3. **`extract-abi.py --check`** -> 77 -> **75 `- ` rows** (one retired: `xg_update_submenu`; one
   new `does not mention 0` row recorded in `macro-body-notes.tsv`); the line-count row reads
   **18.2%** (C 6,753 vs 5,526).
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled against a real GTK tree, so the four prototypes this slice
added to the mirror and the one macro it left in the unit are `UNVERIFIED`; the pgtk arm's reading
is the probe's (Owed 17).

## Slice 12 (2026-10-15, status 4.342): C 4096-4189 — the menubar updater and its map callback

**What this slice is.**  C 4096-4104's comment and **`xg_modify_menubar_widgets`** (C 4105-4170 -
the non-`static` entry point `xmenu.c` calls: it takes the menubar's live children, refreshes the
top-level names through `xg_update_menubar`, and, when `deep_p`, walks every sub menu through
`xg_update_submenu`), plus C 4172-4174's comment and **`menubar_map_cb`** (C 4176-4188 - the map
handler that measures the menubar and resizes the frame).  Port: 5,526 -> **5,658 lines**; the
table's last row splits at C 4189 - the next text is C 4190-4192's comment above
`xg_update_frame_menubar`.

**The mirror defect this slice fixed - and the two pre-existing arm errors it removes.**
`menubar_map_cb` *writes* `FRAME_MENUBAR_HEIGHT (f)` (C 4185), and the mirror
`include/xterm.rs` carried **only C's `#else` variant** of that macro (`#define
FRAME_MENUBAR_HEIGHT(f) ((void) f, 0)`, xterm.h 1463-1464) with its body written as
`{ (void) ($f); 0 }` - which is not Rust (`E0425 void`, the three errors every arm reading has
carried since slice 2, and the reason `src/xfns.rs` 7033-7040 spells the same C line through
`(*((*f).output_data.x as *mut x_output)).menubar_height`).  This slice closes both halves:

- **the live arm is added** (`#if defined (USE_X_TOOLKIT) || defined (USE_GTK)`, xterm.h
  1461-1462) under `all (have_x_windows, any (use_x_toolkit, use_gtk))`, its body the pointer
  read C writes; and
- **the `#else` arm's gate narrows** to `all (have_x_windows, not (use_x_toolkit), not (use_gtk))`
  and its body becomes `{ let _ = &$f; 0 }` - the same "evaluate `f`, yield `0`" C's cast says,
  now in Rust.

So C 4183's read and C 4185's write are **one** spelling each, as C writes them, in both the GTK
and the pgtk arm - and the three `void` errors are gone from every arm (arm A 30 -> 27 `^error`
matches, arm C 64 -> 61).  `docs/ports/xfns.md`'s Owed row for the body and its `src/xfns.rs`
7033-7039 note both name this row; this round is the one that closes it, and that ledger carries
the closure line.

**Deviations this slice adds.**

- **C 4186's `0` is the `bool pretend` parameter** of `adjust_frame_size` (frame.h 1511-1512), so
  the port writes `false` there - the round's own arm reading caught it as `E0308` on the first
  pass, which is why the line carries the note it does.
- C 4118's `&list` is the port's `&raw mut list` (the parameter `list` is written by the callee),
  C 4132/4140's two `for` heads are `while` loops with C's step at the end (PORTING.md 2.2),
  C 4135/4137's two `= 0` locals are the null pointer and the null `GtkMenuItem *`, and C 4160's
  `witem != 0` is the port's null check.
- `adjust_frame_size` needs **no** new declaration here: the unit already carries it at its C 1280
  use site (unit 497-500).  A first draft added a second one and the arm reading reported
  `E0428` - recorded because it is the shape of that class of mistake.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings.**  Arm A: **27 `^error` matches (26 errors), none anchored in this
   slice** - the three `void` errors of every earlier arm reading are gone; arm A plus
   `gtk_check_version_ge_3_14`: the same 27.  Arm C (GTK2): **61, none here** (was 64); arm C
   plus `have_freetype`: the same 61.  Arm B (and the same set plus `have_x_windows`): the dedupe
   artifact.  The out-of-tree probe reads **10 `^error` matches, none in this slice**.
3. **`extract-abi.py --check`** -> 77 -> **73 `- ` rows** (two retired: `xg_modify_menubar_widgets`
   and `menubar_map_cb`; no new `does not mention` row was needed); the line-count row reads
   **16.2%** (C 6,753 vs 5,658).
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled against a real GTK tree, so this slice's mirror change is
evidenced by rustc's name resolution alone; the pgtk arm's reading is the probe's (Owed 17).

## Slice 13 (2026-10-15, status 4.343): C 4190-4257 — the menubar's frame hooks

**What this slice is.**  C 4190-4191's comment and **`xg_update_frame_menubar`** (C 4193-4234 -
the `xdisp.c`-called hook that packs the menubar into the frame's vbox, reorders it to the front,
connects `menubar_map_cb`, shows and measures it, fixes up the visual for a 32-bit display under
GTK3, and resizes the frame when the height changed), plus C 4236-4237's comment and
**`free_frame_menubar`** (C 4239-4256 - the removal hook `frame.c` calls).  Port: 5,658 ->
**5,780 lines**; the table's last row splits at C 4257 - the next text is C 4258's
`#ifndef HAVE_PGTK` and `xg_event_is_for_menubar`.

**The mirror this slice needed.**  `include/gtk.rs` gains **`gtk_box_reorder_child`**
(gtk/gtkbox.h, C 4210) and **`gdk_screen_get_system_visual`** (gdk/gdkscreen.h, C 4221), both fresh
names and both `UNVERIFIED`.  **`gtk_widget_get_mapped`** (C 4200) could not join them: three units
that `include!` the mirror already carry their own copy (`src/xmenu.rs` 794, `src/pgtkmenu.rs` 243,
`src/xterm.rs` 31215), so it stands in the unit - the class Owed 10 and Owed 12 record.  Everything
else was in place, including slice 12's `FRAME_MENUBAR_HEIGHT` fix, which is what makes C 4228/4230
one spelling each here.

**Deviations this slice adds.**

- **C 4196/4242's `f->output_data.xp`** is C 35/38's `#define xp` (a *token* macro Rust cannot spell
  inside an expression), so the port writes the member C's own `#define` selects as a `#[cfg]` pair
  (`output_data.x` / `output_data.pgtk`) - the shape the file's header note at 108-112 records.
  These two functions differ from the slice-1/2 bodies that do it: they stand in **both** arms.
- **C 4212's `G_CALLBACK (menubar_map_cb)`** is spelled by the mirror's `g_signal_connect!` macro,
  so the function carries a `does not mention g_callback` row in `macro-body-notes.tsv`.
- C 4197's uninitialised `GtkRequisition req` is the port's zeroed value (PORTING.md 1.5),
  C 4231/4253's `0` is `adjust_frame_size`'s `bool pretend` parameter (the slice-12 class), and
  C 4251's `= 0` is the null pointer.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings.**  Arm A: **27 `^error` matches (26 errors), none anchored in this slice**;
   arm A plus `gtk_check_version_ge_3_14`: the same 27.  Arm C (GTK2): **61, none here**; arm C
   plus `have_freetype`: the same 61.  Arm B and the pgtk set: the dedupe artifact.  The
   out-of-tree probe reads **10 `^error` matches, none in this slice** - which matters here,
   because this is the first slice whose text stands in both the `have_pgtk` and the non-pgtk arm
   (the `#[cfg]` pair above).
3. **`extract-abi.py --check`** -> 73 -> **71 `- ` rows** (two retired: `xg_update_frame_menubar`
   and `free_frame_menubar`; one new `does not mention g_callback` row recorded in
   `macro-body-notes.tsv`); the line-count row reads **14.4%** (C 6,753 vs 5,780).
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled against a real GTK tree, so the two prototypes this slice
added are `UNVERIFIED`; the pgtk arm's reading is the probe's (Owed 17).

## Slice 14 (2026-10-15, status 4.344): C 4258-4341 — `xg_event_is_for_menubar`

**What this slice is.**  C 4258's `#ifndef HAVE_PGTK` (its `#endif` is C 4339) and C 4259-4338's
**`xg_event_is_for_menubar`** (C 4260-4338) - the non-`static` predicate `gtkutil.h` 114 declares and
`xterm.c` calls to decide whether a button event belongs to the menu bar: reject the event unless it
is inside the frame's menu-bar rectangle (through XInput2's `XIDeviceEvent` when `HAVE_XINPUT2` is
on, through `XButtonEvent` otherwise), look the frame's GDK window up, ask GTK which widget the
event belongs to, and then walk the menubar's children looking for a mapped child the 1x1 rectangle
intersects.  Port: 5,780 -> **6,029 lines**; the table's last row splits at C 4341 - the next text
is C 4342's form feed, the "Scroll bar functions" banner at C 4343-4345 and `xg_ignore_gtk_scrollbar`
at C 4351.

**The five declarations this slice needed, and where each could stand.**  No mirror this unit
`include!`s reaches them:

- `GenericEvent` (X.h 223 - the event-type constant C 4275/4294/4313 test) is carried in-unit at the
  same value `src/xwidget.rs` 2808 already gives it.
- `XButtonEvent` (Xlib.h 585) and `XGenericEventCookie` (Xlib.h 963) - `include/xlib.rs` 1556 carries
  a 192-byte `XEvent` stand-in with no members for either (the `src/xterm.rs` 23225-23228 note), so
  the port declares them at the use site and reads C's member text through a local cast: the
  `src/xmenu.rs` 212/296-302 shape, which is the same record's other consumer.  Their size and offset
  assertions are **clang readings on this machine** - `XButtonEvent` is 96 bytes with `window`@32,
  `x`@64, `y`@68 and `same_screen`@88; `XGenericEventCookie` is 56 bytes with `evtype`@36 and
  `data`@48 (`src/xwidget.rs` 2969-2973 asserts the same two offsets).
- `XIDeviceEvent` - XInput2's record, and **`X11/extensions/XInput2.h` is not on this machine**, so
  its member list is `src/xmenu.rs` 730-750's in-unit copy and it is **UNVERIFIED**: no offset
  assertion is written, because an assertion that cannot be checked is not evidence.
- `gtk_widget_intersect` (C 4333) stands at the use site: **no row anywhere in the tree declares
  it ("`gtk_widget_intersect` and `G_OBJECT_GET_CLASS` have no row anywhere")**: `src/xterm.rs`
  5619 and 37551 call it, so `include/gtk.rs` is its eventual home - but moving it there changes
  another lane's name resolution and arm readings in the same round this slice becomes the name's
  first *declarer*, so it stays at the use site (Owed 18).

**The mirror this slice needed.**  `include/gtk.rs`'s **`GTK_IS_MENU_BAR`** was gated
`have_gtk3 && have_xinput2`, pinned to the unit's then-only consumer (C 1039-1046's site, itself
inside that fold).  C 4304 uses it **outside** any GTK3 fold, so the gate is widened to a `have_gtk3`
/ `not (have_gtk3)` pair.  **`gtk_widget_is_ancestor`** (gtk/gtkwidget.h, C 4306) is the round's
other new name; both are `UNVERIFIED` (no GTK tree on this machine).

**Deviations this slice adds.**

- C 4273/4283, C 4293/4296 and C 4312/4326's three `#ifdef HAVE_XINPUT2` / `#else` pairs are three
  `#[cfg]` pairs in the port, so the `xev->` reads (C 4277-4280, C 4295, C 4315-4316) keep C's member
  text through the UNVERIFIED record while the `event->xbutton.*` reads go through the local cast.
- C 4281/4290's `return 0` are `false` (the C `bool` return), C 4289's `event->xbutton.same_screen`
  is `!= 0` (the `Bool`-to-`bool` test) and C 4306's `! gtk_widget_is_ancestor (...)` is `== 0`
  (the mirror returns `gboolean`, so the `&&` chain needs a `bool`).
- C 4330's `for (iter = list ; iter; iter = g_list_next (iter))` is the port's `while` with C's own
  step at the loop end (PORTING.md 2.2), and C 4334's `break` is the `for`'s own exit.
- C 4295/4298's two `gdk_x11_window_lookup_for_display` results take `as *mut GdkWindow` (the port's
  `GdkWindow` stand-in is a distinct type), C 4334's `NULL` argument is `std::ptr::null_mut ()`, and
  C 4268's `GdkEvent gevent` is the zeroed mirror union (PORTING.md 1.5).
- C 4274's `XIDeviceEvent *xev = (XIDeviceEvent *) event->xcookie.data;` becomes a `#[cfg]`-gated
  `let` under the `have_xinput2` arm only, the C 1546-1548 shape the file already uses.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings, this round against a materialised pre-slice baseline.**  A temporary
   `git worktree` at the previous commit (`a1c7041b`) materialised the pre-slice unit, and the same
   five sets were run on both trees: arm A **28 `^error` matches (27 errors) before and after, the
   error multisets identical**; arm A plus `gtk_check_version_ge_3_14`: 28 before and after; arm A
   without `have_xinput2`: 27 before and after; arm C (GTK2): 62 before and after; arm C plus
   `have_freetype`: 62 before and after.  So this slice adds **no anchor anywhere**, not merely none
   in its own text - a stronger reading than the earlier slices' "none in this slice".  (Slices 12
   and 13 recorded 27/61 for the same sets; those numbers do not reproduce on the pre-slice tree
   with this round's exact set, and no unit or mirror text changed in the interval - the identical
   multisets are the property this ritual is about.)
3. **The pgtk probe.**  The slice's body stands under `not (feature = "have_pgtk")`, so the pgtk face
   compiles none of it; the out-of-tree probe (a copy of the materialised `_root.rs` whose deduped
   `include/gtk.rs` edge is restored) reads **26 `^error` matches under arm A's set and 25 under arm
   C's, both identical to the same probe on the pre-slice tree**, none of them this slice's.
4. **`extract-abi.py --check`** -> 71 -> **71 `- ` rows**: one retired (`missing function
   xg_event_is_for_menubar`) and one added (`function xg_event_is_for_menubar does not mention
   xbutton, xcookie`, recorded in `macro-body-notes.tsv` - C's `event->xbutton.*` and
   `event->xcookie.data` reads are the local casts above); the line-count row reads **10.7%**
   (C 6,753 vs 6,029).
5. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled against a real GTK or XInput2 tree - `X11/extensions/XInput2.h`
is not on this machine - so `XIDeviceEvent`'s member list and offsets, and every GTK prototype and
value this slice added, are UNVERIFIED; the pgtk arm's reading is the probe's (Owed 17).

## Slice 15 (2026-10-15, status 4.345): C 4342-4472 — the scroll-bar section's opening and the widget map

**What this slice is.**  C 4342's form feed, the "Scroll bar functions" banner (C 4343-4345), the flag
the callbacks read (`xg_ignore_gtk_scrollbar`, C 4351), the two theme `static int`s (C 4354/4355), and
then C 4357's `#if defined HAVE_PGTK || !defined HAVE_GTK3` with the whole index-to-widget map: the
record (C 4363-4368) and its increment (C 4372), `xg_store_widget_in_map` (C 4376-4412),
`xg_remove_widget_from_map` (C 4417-4425), the map arm's `xg_get_widget_from_map` (C 4429-4436),
C 4438's `#else` with `find_scrollbar_cb` (C 4439-4446) and the GTK3 arm's `xg_get_widget_from_map`
(C 4448-4471).  Port: 6,029 -> **6,285 lines**; the table's last row splits at C 4472 - the next text
is C 4473's blank and `update_theme_scrollbar_width`.

**The one deviation the readings caught, and it is C's own.**  C 4384's
`TYPE_MAXIMUM (Window) - ID_TO_WIDGET_INCR < id_to_widget.max_size` is arithmetic in `Window`'s own
type, and that type differs between the two faces this fold admits: `include/xlib.rs` 1144's
`unsigned long` under X11 and `include/pgtkgui.rs` 102's `int` under pgtk (C 49's
`typedef int Window;`).  A single spelling is therefore wrong in one face - the first draft's
`as c_ulong` pair type-errored in the pgtk probe (`E0277` and two `E0308`, which is the class the
probe exists for) - so the test is a `#[cfg (feature = "have_pgtk")]` / `not (…)` pair, each arm
keeping C's conversion order (the `int` constant widens into `Window`'s type, the `ptrdiff_t` right
side converts at the comparison).

**The mirror this slice needed.**  `include/gtk.rs` gains **`GTK_IS_SCROLLBAR`** and
**`GTK_IS_EVENT_BOX`** (gtk/gtkscrollbar.h and gtk/gtkeventbox.h, C 4444/4463) - both fresh names (no
other unit declares either), added to the same `GTK_IS_*` family the slice-11 round extended, both
**UNVERIFIED**.  Two more names could not go there and stand at the use site (Owed 18's class,
extended this round): `gtk_container_forall` (C 4464 - `src/xwidget.rs` 504-506 already carries its
own copy) and the per-unit `TYPE_MAXIMUM!` accessor (`src/androidterm.rs` 7268's shape: a *token*
macro Rust cannot spell inside an expression).  **One name this slice first declared in-unit and then
undeclared**: `xnrealloc` (C 4388) already lives in the lisp.h mirror (`include/lisp.rs` 3772, inside
`lisp_safe_decls`, re-exported at 3268's `use lisp_safe_decls::*;`), which `tools/check-mirror-dupes.py`
is the gate that knows - it read `17 duplicate declaration(s) ... 1 failure(s)` on the draft and
`16/16/0` once the copy was deleted, and the four units that call the name had already made the same
point.  That reading is the round's second tool-caught correction, after the C 4384 arm pair below.

**Deviations this slice adds.**

- C 4363-4368's anonymous file-scope record is named **`id_to_widget_s`** (PORTING.md 1.3: an
  anonymous struct must be named) and the object keeps C's name; the port's three `const _` asserts
  read the layout that declaration implies (24 bytes, 0/8/16, no padding on this target).
- C 4392/4401/4403/4420/4422/4432/4433's `id_to_widget.widgets[i]` cannot be Rust indexing (a raw
  pointer has no `Index`), so each of the seven sites is `*…offset (i as isize)` - the shape the
  file's own pixel walkers use.
- C 4388's `xnrealloc` takes and returns `void *`, so the call carries `as *mut c_void` and the
  assignment `as *mut *mut GtkWidget`; C's `sizeof (GtkWidget *)` is
  `core::mem::size_of::<*mut GtkWidget> () as ptrdiff_t`.
- C 4435's `0` and C 4468's `NULL` are `std::ptr::null_mut ()` (two new `does not mention 0` rows in
  `macro-body-notes.tsv`, one per arm of C 4430/4449), C 4420/4432's `!= 0` are `.is_null ()` tests,
  C 4468's `return NULL;` is the port's block-wrapped `return`, and the three file-scope objects keep
  C's zero-initialisation (`false` for the flag, `0` for the two `int`s).
- C 4464-4465's callback argument is the port's `Some (find_scrollbar_cb)`, because the declared
  parameter type is `GtkCallback`, an `Option` (the `src/xwidget.rs` 1972 shape).
- C 4359-4361's, C 4370-4371's, C 4374's, C 4396-4398's, C 4414-4415's and C 4427's comments are
  carried as C's own text at their positions; the two `for` heads (C 4391/4399) are the port's `while`
  with C's own step at the loop end (PORTING.md 2.2).

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.  (This
   unit's `build-tu` is also a parse gate even though the frozen configuration compiles none of it:
   the first draft's C 4384 spelling stopped it with two parse errors, which is how that arm pair
   was found before any arm run.)
2. **The arm readings, against a materialised baseline.**  A temporary `git worktree` at the previous
   commit (`b75d7cca`) provided the pre-slice unit, and all six readings are **identical multisets
   before and after**: arm A 28 `^error` matches, arm A plus `gtk_check_version_ge_3_14` 28, arm A
   without `have_xinput2` 27, arm C (GTK2) 62, arm C plus `have_freetype` 62, and the out-of-tree pgtk
   probe 26 (25 under arm C's set).  So this slice adds **no anchor anywhere** - not merely none in
   its own text.
3. **`extract-abi.py --check`** -> 70 -> **61 `- ` rows**: eight retired (`missing function` for the
   four functions, `missing file-scope object` for the three objects, and `ID_TO_WIDGET_INCR`'s macro
   and constant rows) and three `does not mention 0` rows added to `macro-body-notes.tsv` (without
   which the reading is 64).  The line-count row reads **7.0%** (C 6,753 vs 6,280).
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled against a real GTK tree - there is none on this machine - so
`GTK_IS_SCROLLBAR`, `GTK_IS_EVENT_BOX` and `gtk_container_forall` are UNVERIFIED, and
`id_to_widget_s`'s layout is the port's own reading of a declaration whose C compiler measurement is
not in the baseline (three pointer-wide members, so the two readings cannot differ on this target);
the pgtk arm's reading is the probe's (Owed 17).

## Slice 16 (2026-10-15, status 4.346): C 4473-4532 — the theme measurement and its two readers

**What this slice is.**  The two `static` theme measurements C 4474-4496 and C 4498-4518
(`update_theme_scrollbar_width` and `update_theme_scrollbar_height`, each building a scroll bar from
a fresh `GtkAdjustment` and reading `slider-width`/`trough-border` off its style) and their two
non-`static` readers C 4520-4524 and C 4526-4531 (`xg_get_default_scrollbar_width` and
`xg_get_default_scrollbar_height`, the second of which multiplies the *width* because GTK offers no
theme height).  Port: 6,280 -> **6,472 lines**; the table's last row splits at C 4532 - the next text
is C 4533's `#ifndef HAVE_GTK3` and C 4534-4535's comment above `xg_get_scroll_id_for_window`.

**The declarations this slice needed, and where each could stand.**

- `GtkAdjustment` (gtk/gtkadjustment.h) stands in the unit, in **both** GTK faces (GTK2 has the
  record too, which is what C 4480's `#else` arm casts an `GtkObject *` to it proves): the mirror
  cannot take it because `src/pgtkterm.rs` 5759 declares the same name and that unit `include!`s the
  mirror.  Its opaque head is the one that unit's copy carries, with no layout assertion.
- `GTK_ADJUSTMENT` (the checked cast of C 4486/4510), `gtk_adjustment_new` (whose GTK2 face returns
  `GtkObject *` - C 4480/4504 are what prove it, so its two arms differ in the return type alone),
  GTK3's `gtk_scrollbar_new`, GTK2's `gtk_hscrollbar_new`/`gtk_vscrollbar_new` and the four
  `gtkutil.h` 38-42 bounds are the rest of that class: the five prototypes name the record, and the
  bounds' home (`include/gtkutil.rs`) is the placeholder Owed 10/Owed 11 record.
- The **mirror** takes the two fresh names: GTK2's `GtkObject` (gated `not have_gtk3`) and
  `gtk_widget_style_get` (gtkwidget.h's variadic style reader, C 4488/4512).

**One macro this slice had to complete the family with.**  C 122-124's `gtk_scrollbar_new` is a
*macro* in GTK2 (carried in the unit since slice 1) and a *function* in GTK3, and C 4486/4510 write
one text for both - so the round adds the GTK3 forwarding macro over the new function, exactly the
`gtk_box_new`/`gtk_font_chooser_dialog_new` shape the mirror's own note at 2566-2572 already names
`gtk_scrollbar_new` as the next member of.  This is the first slice whose call sites expand C 122-124
and C 119-121's macros, which is also why GTK2's `gtk_hscrollbar_new`/`gtk_vscrollbar_new` had to
arrive with it.

**The one-line repair this slice carried, and why it belongs here.**  With the two
`update_theme_scrollbar_*` names unresolved, typeck failed inside `style_changed_cb` and rustc never
ran borrowck on it; resolving them exposed `E0381: used binding event isn't initialized` at that
function's `EVENT_INIT (&raw mut event)` (C 1509-1559's `struct input_event event;`).  **No `E0381`
is left in any face**, because the round repaired the local the way this file's other `EVENT_INIT`
sites and every other unit spell it - `let mut event: input_event = std::mem::zeroed ();`
(PORTING.md 1.5's uninitialised-local rule, the `src/pgtkterm.rs` 5721/7724/7752 shape) - rather
than transcribing C's uninitialised declaration.  C's `EVENT_INIT` writes the fields it needs before
any read, so the zeroing is invisible in C's semantics and mandatory in Rust's.

**Deviations this slice adds.**

- C 4477/4492 and C 4501's `HAVE_GTK3` pairs are three `#[cfg]` pairs: the two `vadj`/`hadj`
  declarations (`*mut GtkAdjustment` / `*mut GtkObject`) and C 4492-4494's GTK2-only width floor.
  C 4516's `if (w < 12) w = 12;` is *outside* any pair in C and stays ungated here.
- C 4485/4509's `XG_SB_MIN`/`XG_SB_MAX`/`YG_SB_*` are `c_int` constants (C's `#define`s are int
  literals, and C 5034's `clip_to_bounds` call needs them as `ptrdiff_t`), so each of the twelve
  `gdouble` arguments those calls pass carries C's implicit integer-to-`double` conversion as an
  explicit `as c_double` - the file's `scale as f64` shape.
- C 4488/4512's variadic call passes the property names as `c"slider-width".as_ptr ()` /
  `c"trough-border".as_ptr ()` (the file's C-string shape), the two out-parameters as
  `(&raw mut w)` / `(&raw mut b)`, and C's terminating `NULL` as `std::ptr::null::<c_char> ()` (the
  explicit type is what the variadic position needs).
- C 4487/4511's `G_OBJECT (...)` is the mirror's `G_OBJECT!` with `as gpointer` (its two readers
  take `gpointer`, the `src/xsettings.rs` 1872 spelling), C 4478/4502's uninitialised declarations
  are the port's deferred `let` bindings, and the two `return` statements keep C's shape.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings, against a materialised baseline** (a temporary `git worktree` at `516f4772`):
   arm A **28 -> 26** `^error` matches, arm A plus `gtk_check_version_ge_3_14` **28 -> 26**, arm A
   without `have_xinput2` **27 -> 25**, arm C (GTK2) **62 -> 64**, arm C plus `have_freetype`
   **62 -> 64**, the out-of-tree pgtk probe **26 -> 24**.  The two anchors that leave every face are
   the `update_theme_scrollbar_width` / `_height` rows this file's Evidence (2) table carried; the
   C arm's two extra rows are four new references to Owed 6's untranscribed GTK2
   `GTK_ORIENTATION_VERTICAL`/`_HORIZONTAL` (C 4486/4510 reach each twice through C 122-124's macro)
   minus the same two anchors.  The correction paragraph under Evidence (2) records both movements.
3. **`extract-abi.py --check`** -> 61 -> **56 `- ` rows**, all five retirements: the four
   `missing function` rows (`update_theme_scrollbar_width`/`_height`,
   `xg_get_default_scrollbar_width`/`_height`) and the `line count drifted` row, which drops out
   because the ratio falls to **4.2%** (C 6,753 vs 6,472) - below the row's reporting threshold.  No
   new `does not mention` row was needed.
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled against a real GTK tree - there is none on this machine - so
every prototype and every value this slice added is UNVERIFIED, `GtkAdjustment`'s layout is the
port's opaque stand-in, and GTK2's two enumerators remain Owed 6's; the pgtk arm's reading is the
probe's (Owed 17).

## Slice 17 (2026-10-15, status 4.347): C 4533-4594 — the id lookup, the destroy handler and the size-allocate callback

**What this slice is.**  Three functions with C's own folds around them:
**`xg_get_scroll_id_for_window`** (C 4537-4553, under C 4533's `#ifndef HAVE_GTK3` - slice 15's map
read backwards: look the X window's widget up, then walk `id_to_widget` for that pointer and answer
its index, or C 4552's `-1`), **`xg_gtk_scroll_destroy`** (C 4561-4566, under C 4560's
`#if !defined HAVE_GTK3 || defined HAVE_PGTK` - the `destroy` signal handler that takes the index C
4596's creator will pass and removes it from the map), and **`xg_scroll_bar_size_allocate_cb`**
(C 4570-4592, under C 4569's `#if defined HAVE_GTK3 && !defined HAVE_PGTK` - the GTK3/X11
`size-allocate` handler: a live configure event goes to `x_scroll_bar_configure` as is, otherwise C
4582-4590's synthetic `GdkEvent` is filled from the allocation's rectangle and consumed the same
way).  Port: 6,472 -> **6,574 lines**; the table's last row splits at C 4594 - the next text is
C 4595's `static void xg_finish_scroll_bar_creation (...)`.

**The mirror this slice needed.**  `include/gtk.rs` gains two fresh names - **`gtk_get_current_event`**
(gtk/gtkmain.h, C 4575) and **`GDK_CONFIGURE`** (gdk/gdktypes.h, C 4578, declared beside the family's
other `GdkEventType` members) - and **`GdkEventConfigure` grows GDK's trailing `width`/`height`**
(C 4586/4587's two stores).  The record's own note had left those two out on the rule that it carries
"the members the reference's text touches"; this slice's text is the first that touches them, they sit
at GDK's own positions right after `y`, and they are **the two members `src/xterm.rs` 29935-29938
names as owed** for that unit's own `GdkEventConfigure` reads (C 18071-18072) - so that lane's Owed
row can be closed without this round touching its file.  All three additions are UNVERIFIED
(no GDK tree on this machine).

**One declaration that stands in the unit.**  `x_scroll_bar_configure` (xterm.h 1848:
`extern void x_scroll_bar_configure (GdkEvent *);`) is defined by `xterm.c` 18058-18084 and carried
by `src/xterm.rs` 29944 as `#[unsafe(no_mangle)] pub unsafe extern "C" fn`, and `include/xterm.rs`
does not declare it - while that unit's own note already names **this** file's C 4579/4590 as its two
callers.  It therefore stands at the use site under the gate its definition carries
(`all (have_x_windows, have_gtk3)`), the class Owed 18 records and Owed 11 documents with three
older names.

**Deviations this slice adds.**

- C 4583's `FALSE` is the mirror's `gboolean` while the member is GDK's `gint8`, so the store is
  `FALSE as i8` - C's conversion made explicit, the `w32.rs` `FALSE as DWORD` shape.
- C 4576's uninitialised `GdkEvent dummy` is the port's zeroed value (PORTING.md 1.5); C writes each
  of the six members it reads, and the zeroing is what keeps `&raw mut dummy` legal in Rust.
- C 4578's `event->any.type` is `(*event).any.r#type` (the `src/xwidget.rs` 1000 shape for the same
  union), C 4590's `&dummy` is `(&raw mut dummy)`, C 4547's `for` head is the port's `while` with C's
  own step at the loop end (PORTING.md 2.2), C 4548's `widgets[idx]` is the `.offset (…)` spelling a
  raw pointer needs, C 4552's `-1` is C's sentinel and C 4564's `(intptr_t) data` is
  `data as intptr_t` (`include/lisp.rs` 1077's `isize`).
- C 4534-4535's and C 4556-4558's comments and C 4582's one-liner are carried as C's own text at their
  positions; C 4561/4570's callbacks keep the C ABI (`unsafe extern "C" fn`, the `find_scrollbar_cb`
  shape) and C's unused `widget`/`user_data` parameters stay unused, as they are in C.

**Readings.**

1. **The frozen reading - `build-tu`.**  exit 0, `build/femacs/gtkutil.o` is **336 bytes**.
2. **The arm readings, against a materialised baseline** (a temporary `git worktree` at `2b9e5858`):
   **all six are identical multisets before and after** - arm A 26, arm A plus
   `gtk_check_version_ge_3_14` 26, arm A without `have_xinput2` 25, arm C (GTK2) 64, arm C plus
   `have_freetype` 64, the out-of-tree pgtk probe 24.  That is the reading this slice should have:
   none of the three functions has a caller yet (C 4882-4900 are the first), so nothing can change in
   any face until the creator slice lands.
3. **`extract-abi.py --check`** -> 56 -> **53 `- ` rows**, all three retirements being the
   `missing function` rows of this slice's functions; no new `does not mention` row was needed.
4. The ten gates: all `rc=0`, 0 FAIL (and `tools/check-all-units.sh` reads `checked 83 unit(s),
   0 failure(s)`).

**Not claimed.**  No arm was compiled against a real GTK or GDK tree - there is none on this machine
- so `gtk_get_current_event`, `GDK_CONFIGURE` and the extended record are UNVERIFIED, and
`x_scroll_bar_configure` is used here exactly as `src/xterm.rs` defines it (no signature measurement
of its own); the pgtk arm's reading is the probe's (Owed 17).
