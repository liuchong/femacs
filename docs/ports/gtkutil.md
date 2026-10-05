# emacs/src/gtkutil.c (6753 lines) — first slice: C 1-1296

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
> (**2593 lines**, from the 11 of the placeholder), carrying **C 1-1296** with **both arms of every
> `#if`**.
>
> **This slice is C 1-1296**, from the file's first line through `xg_frame_set_char_size`'s closing
> brace (C 1176-1296): the include list and this file's own declarations (C 20-146), the
> `EmacsMenuBar` GTK3 type and its four functions, the display functions (`xg_set_screen`,
> `xg_display_open`, `xg_get_gdk_scale`, `xg_get_scale`, `xg_display_close`,
> `xg_create_default_cursor`), the pixbuf/image helpers (`xg_get_pixbuf_from_pix_and_mask`,
> `xg_get_pixbuf_from_surface`, `file_for_image`, `xg_get_image_for_pixmap`), the two list and the
> utf8 helpers, `xg_check_special_colors`, the whole Tooltips section (both the X arm's
> `hierarchy_ch_cb`/`qttip_cb`/`xg_prepare_tooltip`/`xg_show_tooltip`/`xg_hide_tooltip` and the pgtk
> arm's two entry points), `my_log_handler`, `xg_is_menu_window`, `xg_set_geometry`,
> `xg_frame_resized` and `xg_frame_set_char_size`.  **The next live definition is
> `xg_frame_set_size_and_position` (C 1316-1397)** - the first item of the next lane's slice
> (C 1313-1315 is its comment and `void` line; C 1301-1311 is the `#if 0` block that holds
> `xg_height_or_width_changed`, dead text C itself excludes, and C 1399-1425 is
> `xg_win_to_widget`, whose definition this slice *calls* at C 1037 but does not carry).  The
> remainder (C 1297-6753) is the table's last row.
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
> **`gtk_check_version_ge_3_22`** - **neither is registered in `femacs/xtask/Cargo.toml`** (`## Owed`
> 1).  The pragma-shaped `#if GTK_CHECK_VERSION (3, 10, 0)` at C 104 gates a forward declaration
> only, which PORTING.md 0.1-12 drops anyway, so its arm is C's text plus that feature name.
>
> **The toolkit surface is declared in this unit (P-88 ①, which P-90 ① keeps for this round), and
> this batch is `text-only` (P-86 ③⑵).**  Measured on this machine:
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
| 1297-6753 | **the remainder** — from C 1297's blank through C 6753: C 1301-1311's `#if 0` block (`xg_height_or_width_changed`), `xg_frame_set_size_and_position` (C 1316, the next live definition), `xg_win_to_widget`, `xg_set_widget_bg`, the dark-mode trio, `style_changed_cb`, `delete_cb`, `xg_create_frame_widgets`, `xg_free_frame_widgets`, `xg_wm_set_size_hint`, the frame-parameter setters, the whole dialog/file-chooser half (`create_dialog`, `xg_dialog_run`, `xg_get_file_with_chooser`, `xg_get_file_with_selection`, `xg_get_font`), the menu machinery (`create_menus`, `xg_update_menubar`, …), scroll bars, printing, the tool bar and `xg_initialize`/`xg_filter_key` at the end | not this slice |

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
