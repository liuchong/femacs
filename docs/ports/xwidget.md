# emacs/src/xwidget.c (4,304 lines) — first slice: C 1-1180

> **One of P-86's class: the frozen configuration compiles nothing here.**  `xwidget.o` joins the
> build's object list only when xwidgets are on - `configure.ac` 4422 starts `HAVE_XWIDGETS=no`,
> 4434 sets `XWIDGETS_OBJ="xwidget.o"` in the GTK branch (under 4433's
> `HAVE_XWIDGETS=$HAVE_WEBKIT`) and 4463 sets it again in the Cocoa branch, and 4474 is
> `AC_DEFINE ([HAVE_XWIDGETS], [1], [Define to 1 if you have xwidgets support.])`.
> `docs/baseline/frozen-build-sources.txt` lists no `xwidget.o`;
> `docs/baseline/config/darwin-arm64.macros.tsv` 1717 has `HAVE_XWIDGETS undef` and 3861
> `USE_GTK undef`, so the text can never be *compiled* here.  Unlike the class P-86 was written
> for, this file's conditions are all *decided*: `python3 tools/check-cond-eval.py src/xwidget.c`
> reads `121 conditions, 0 unknown, 0 disagreements [VERIFIED]` (the file is
> `docs/baseline/config/cond-eval-verified.txt` 428), so `tools/extract-abi.py`'s checklist is
> real and the `--check` reading in Evidence ③ is not vacuous.  What this ledger adds is P-86 ②'s
> substitute shape - the **C line-range table**, accounting for every line of the reference -
> beside the `## Evidence` readings and the `## Deviations`.
>
> **This ledger replaces, not extends, what it said before.**  Until this batch it read "the
> frozen configuration does not compile this file ⇒ the port is the empty unit with this note"
> (the P-08 reading, `docs/status.md` 3.31), which is what the 11-line placeholder was.  P-86 ①
> replaces that with *transcribe the text, both arms, no stub*; what the earlier note measured
> (not in the frozen source list, empty checkbox sections, 336-byte object) is unchanged and still
> true here - it is the *conclusion* that is withdrawn, exactly as `docs/ports/gtkutil.md`,
> `docs/ports/pgtkterm.md` and `docs/ports/androidmenu.md` record for their own files.  Port:
> `femacs/src/xwidget.rs` (**2,321 lines**, from the 11 of the placeholder), carrying **C 1-1180**
> with **both arms of every `#if`**.
>
> **This slice is C 1-1180**, from the file's first line through `run_file_chooser_cb`'s closing
> brace (C 1122-1180): the include list (C 20-54), the four file-scope objects (C 56-59), the
> seven forward declarations and the two allocators (C 61-92), `struct widget_search_data`
> (C 118-125), the PGTK pair `xw_forward_event_translate`/`xw_forward_event_from_view`
> (C 134-227), the X11 pair `enum xw_crossing_mode`/`xw_translate_x_modifiers` (C 231-266), the
> four subrs `Fmake_xwidget` (C 280-481), `Fxwidget_live_p` (C 483-491),
> `Fxwidget_perform_lispy_event` (C 504-694) and `Fget_buffer_xwidgets` (C 696-719), the two
> helpers `xwidget_hidden`/`xwidget_from_id` (C 721-737) and the file's wide GTK arm from
> `pick_embedded_child` (C 740-764) through `run_file_chooser_cb` (C 1122-1180), including
> `record_osr_embedder` (C 766-782), `find_xwidget_for_offscreen_window` (C 784-804),
> `from_embedder`/`to_embedder` (C 806-878), `find_suitable_pointer`/`find_suitable_keyboard`
> (C 880-921), `find_widget_cb`/`find_widget` (C 923-1018), `find_widget_at_pos` (C 1020-1062) and
> the PGTK trio `cursor_for_hit`/`define_cursors`/`mouse_target_changed` (C 1064-1119).
> **The next slice opens at C 1182's `#ifdef HAVE_X_WINDOWS`**, whose first definition is
> `xv_drag_begin_cb` (C 1184-1200); C 1181 is the blank the table needs no row for.  The remainder
> (C 1182-4304) is the table's last row.
>
> **The file gate is `feature = "use_gtk"`** - the mandate's ruling, and C 37's `#ifdef USE_GTK` is
> the file's first and widest one - on every item *and on the include graph* (P-86 ⑧: an
> `include!` outside the gate makes the frozen build fail instead of producing the empty object the
> class expects; this slice's C 22-34 list is written outside any `#if` and carries the gate anyway,
> which is `## Deviations` 3).  The slice's inner conditions are C 41's `#ifndef HAVE_PGTK`, C 47's
> `#ifdef HAVE_XINPUT2`, C 50's `#elif defined NS_IMPL_COCOA`, C 61/98/298/329/493/513/527/739's
> `#ifdef USE_GTK`, C 62/230/1027's `#ifdef HAVE_X_WINDOWS`, C 64/339/396's `WEBKIT_CHECK_VERSION`,
> C 130/451/1064's `#ifdef HAVE_PGTK`, C 423's `#if !defined HAVE_XINPUT2 && !defined HAVE_PGTK`,
> C 519/537/624/684's `#ifdef HAVE_XINPUT2`, C 572/604/774/813/850's `#ifndef HAVE_PGTK` and
> C 644's `#ifndef HAVE_X_WINDOWS` - all decided by the frozen table, and **both arms of every one
> of them are carried** (the `## Gate mapping` table below).
>
> **`WEBKIT_CHECK_VERSION` is decided by `clang`, not by a `configure` macro.**  The reference tree
> defines it nowhere (`grep -rn WEBKIT_CHECK_VERSION emacs/src/*.h` is empty); it comes from
> WebKitGTK's headers, and on this machine
> `clang -dM -E -std=gnu23 … /tmp/xw1/p.c` reports
> `error: function-like macro 'WEBKIT_CHECK_VERSION' is not defined` for C 64's condition, so no
> branch is taken (the same answer `tools/check-cond-eval.py` records: 0 disagreements).  The port
> spells the arms with derived-condition features after the `android_api_le_11` /
> `gtk_check_version_ge_3_10` naming: **`webkit_check_version_ge_2_16`**, **`_ge_2_26`**,
> **`_ge_2_34`** - **none registered in `femacs/xtask/Cargo.toml`** (`## Owed` 1).
>
> **The toolkit surface is declared in this unit (P-88 ①, which P-90 ① keeps for this round), and
> this batch is `text-only` (P-86 ③⑵).**  Measured on this machine:
> `ls -d /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0
> /opt/homebrew/include/webkit2gtk-4.0 /opt/homebrew/include/webkitgtk-4.0
> /opt/homebrew/include/JavaScriptCore` -> all `No such file or directory`; `pkg-config` is **not
> installed**; `find / -maxdepth 6 -name gdktypes.h` and `-name gdkevents.h` -> nothing (the same
> reading `docs/ports/gtkutil.md` records for the sibling lane).  So **GDK, GTK and WebKit have no
> tree to compile or measure against**: every citation of theirs names the header alone (no line,
> no probe), every size, offset, enum width and enumerator *value* of theirs is **UNVERIFIED**, no
> arm flip is possible and no layout assertion is written for a GDK/GTK/WebKit record.
> **GLib/GObject, cairo and X11 *are* on this machine** (`/opt/homebrew/include/glib-2.0`, GLib
> 2.90.0, with the generated `glibconfig.h` under
> `/opt/homebrew/Cellar/glib/2.90.0/lib/glib-2.0/include`; `/opt/homebrew/include/cairo`;
> `/opt/homebrew/include/X11`), so the GLib declarations quote the header *and the line* they were
> read from, `GList`/`GSList`'s layouts are real clang readings (Evidence ⑤) and `ShiftMask`/
> `ControlMask` carry `X11/X.h`'s own expressions.  P-88 ⑤/P-90 ③: the move into a dedicated SDK
> mirror (`femacs/src/include/gtk.rs`, PORTING.md P-90) is owed (`## Owed` 2), and this unit's copy
> plus the segment copies in `src/gtkutil.rs` and `src/pgtkterm.rs` are the ones to drop there.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 4422-4474: `XWIDGETS_OBJ="xwidget.o"` at 4434 and 4463, `AC_DEFINE ([HAVE_XWIDGETS], [1])` at 4474) | `feature = "use_gtk"` on every item *and on the include graph* (P-86 ⑧; C 37's `#ifdef USE_GTK` is the file's own gate) | a GTK xwidgets configuration (the mandate's ruling; `## Deviations` 3) |
| C 37 `#ifdef USE_GTK` … C 50 `#elif defined NS_IMPL_COCOA` … C 52 (the bottom-end include chain) | `feature = "use_gtk"` / `all (feature = "use_gtk", feature = "ns_impl_cocoa")` | GTK / Cocoa |
| C 41 `#ifndef HAVE_PGTK` (C 42-43 `<cairo-xlib.h>` + `<X11/Xlib.h>`, C 45 `#else`'s `<gtk/gtk.h>`) | `not (feature = "have_pgtk")` / `feature = "have_pgtk"` | X11 GTK / PGTK |
| C 47 `#ifdef HAVE_XINPUT2` (C 48 `<X11/extensions/XInput2.h>`) | `feature = "have_xinput2"` | XInput2 |
| C 62 `#ifdef HAVE_X_WINDOWS` (C 63 `x_window_to_xwv_map`) and C 64 `#if WEBKIT_CHECK_VERSION (2, 34, 0)` (C 65 `dummy_tooltip_string`) | `feature = "have_x_windows"`, then `feature = "webkit_check_version_ge_2_34"` (`## Owed` 1) | X11 GTK with WebKit ≥ 2.34 |
| C 98 `#ifdef USE_GTK` (C 118-125 `widget_search_data`, C 127 `find_widget`) | `feature = "use_gtk"` | GTK |
| C 130 `#ifdef HAVE_PGTK` (C 134-227) | `all (feature = "use_gtk", feature = "have_pgtk")` | PGTK |
| C 230 `#ifdef HAVE_X_WINDOWS` (C 231-277) | `all (feature = "use_gtk", feature = "have_x_windows")` | X11 GTK |
| C 298 `#ifdef USE_GTK` (the `xg_gtk_initialized` check) | `feature = "use_gtk"` | GTK |
| C 329 `#ifdef USE_GTK` … C 476 `#elif defined NS_IMPL_COCOA` … C 478 | `feature = "use_gtk"` / `all (feature = "use_gtk", feature = "ns_impl_cocoa")` | GTK / Cocoa (`## Deviations` 4) |
| C 339 `#if WEBKIT_CHECK_VERSION (2, 26, 0)` and C 396 `#if WEBKIT_CHECK_VERSION (2, 16, 0)` | `feature = "webkit_check_version_ge_2_26"` / `_ge_2_16` (`## Owed` 1) | WebKit ≥ those versions |
| C 423 `#if !defined HAVE_XINPUT2 && !defined HAVE_PGTK` (C 424 `synthesize_focus_in_event`) | `all (not (feature = "have_xinput2"), not (feature = "have_pgtk"))` | neither |
| C 451 `#ifdef HAVE_PGTK` (the `mouse-target-changed` connection) | `feature = "have_pgtk"` | PGTK |
| C 493 `#ifdef USE_GTK` (C 494-501 `set_widget_if_text_view`) | `feature = "use_gtk"` | GTK |
| C 513/527 `#ifdef USE_GTK` (the declarations and the body of `Fxwidget_perform_lispy_event`) | `feature = "use_gtk"` | GTK |
| C 519/537/624/684 `#ifdef HAVE_XINPUT2` (the four XI2 blocks) | `feature = "have_xinput2"` | XInput2 |
| C 572/604 `#ifndef HAVE_PGTK` … `#else` … C 582/616 | `not (feature = "have_pgtk")` / `feature = "have_pgtk"` | X11 GTK / PGTK |
| C 644 `#ifndef HAVE_X_WINDOWS` … `#else` (C 646-649) … C 650 | `not (feature = "have_x_windows")` / `feature = "have_x_windows"` | PGTK / X11 GTK |
| C 739 `#ifdef USE_GTK` (C 739-2281: the file's wide GTK arm; this slice carries C 739-1180) | `feature = "use_gtk"` | GTK |
| C 774/813/850 `#ifndef HAVE_PGTK` … `#else` … C 778/840/877 | `not (feature = "have_pgtk")` / `feature = "have_pgtk"` | X11 GTK / PGTK |
| C 1027 `#ifdef HAVE_X_WINDOWS` (the passive-grab arm of `find_widget_at_pos`) | `feature = "have_x_windows"` | X11 GTK |
| C 1064 `#ifdef HAVE_PGTK` (C 1065-1119 `cursor_for_hit`/`define_cursors`/`mouse_target_changed`) | `all (feature = "use_gtk", feature = "have_pgtk")` | PGTK |
| the records this slice reads: `include/xwidget.rs` gates `struct xwidget`/`struct xwidget_view`/`XXWIDGET`/`XXWIDGET_VIEW`/`CHECK_LIVE_XWIDGET` … on `have_xwidgets` (the header's own `#ifdef HAVE_XWIDGETS`, xwidget.h 31) | the mirror's gate; the port adds nothing (`## Deviations` 6) | xwidgets enabled |

Frozen values behind every arm: `USE_GTK undef` (`darwin-arm64.macros.tsv` 3861), `HAVE_PGTK undef`
(1489), `HAVE_XINPUT2 undef` (1702), `HAVE_X_WINDOWS undef` (1720), `NS_IMPL_COCOA undef` (2996),
`HAVE_XWIDGETS undef` (1717), `HAVE_GTK3 undef` (1385).

## C line ranges

The rows are contiguous on purpose: `docs/port-map.md` binds this ledger to `src/xwidget.c` *and*
to `src/xwidget.h` (both map to a file named `xwidget`), and `tools/check-dead-file-coverage.py`
reads each range against both references - so every row opens on the blank line that precedes its
block, and the union covers C 1-4304 without a gap.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-20 | the blank and `#include <config.h>` | ungated (every unit includes it) |
| 21-34 | the blank and the include list: `buffer.h`, `coding.h`, `xwidget.h`, `lisp.h`, `blockinput.h`, `dispextern.h`, `frame.h`, `keyboard.h`, `gtkutil.h`, `sysstdio.h`, `termhooks.h`, `window.h`, `process.h` | file gate (and `lisp.h` is included first, `## Deviations` 1) |
| 35-54 | the blank, `/* Include xwidget bottom end headers.  */`, C 37 `#ifdef USE_GTK`/C 50 `#elif defined NS_IMPL_COCOA`/C 52 `#endif` (C 38-45 WebKit/cairo/X11/GTK, C 41 `#ifndef HAVE_PGTK`, C 47 `#ifdef HAVE_XINPUT2`, C 51 `nsxwidget.h`) and C 54 `#include <math.h>` | file gate / arms (the declarations this chain owes stand here, `## Deviations` 7) |
| 55-59 | the blank and the four file-scope objects `id_to_xwidget_map`, `internal_xwidget_view_list`, `internal_xwidget_list`, `xwidget_counter` | file gate |
| 60-77 | the blank and C 61 `#ifdef USE_GTK` (C 62 `#ifdef HAVE_X_WINDOWS` with C 63 `x_window_to_xwv_map` and C 64 `#if WEBKIT_CHECK_VERSION (2, 34, 0)` with C 65 `dummy_tooltip_string`) and C 68-76's seven forward declarations | file gate / `have_x_windows` / version arm; the declarations are dropped (`## Deviations` 2) |
| 78-83 | the blank and `allocate_xwidget` | file gate |
| 84-89 | the blank and `allocate_xwidget_view` | file gate |
| 90-92 | the blank, `XSETXWIDGET` and `XSETXWIDGET_VIEW` | file gate |
| 93-96 | the blank and `xwidget_view_lookup`/`kill_xwidget`'s forward declarations | dropped (`## Deviations` 2) |
| 97-128 | the blank and C 98 `#ifdef USE_GTK` (C 99-116 seven forward declarations, C 118-125 `struct widget_search_data`, C 127 `find_widget` declaration) | file gate; the declarations are dropped |
| 129-228 | the blank and C 130 `#ifdef HAVE_PGTK` (C 131-132 `mouse_target_changed` declaration, C 134-202 `xw_forward_event_translate`, C 204-227 `xw_forward_event_from_view`) | `have_pgtk` arm |
| 229-278 | the blank and C 230 `#ifdef HAVE_X_WINDOWS` (C 231-236 `enum xw_crossing_mode`, C 238-266 `xw_translate_x_modifiers`, C 268-277 the three declarations) | `have_x_windows` arm; the declarations are dropped |
| 279-481 | the blank and `DEFUN ("make-xwidget", …)` - C 298-301 the `xg_gtk_initialized` arm, C 329-478 the main arm (C 339-342 and C 396-403 the two `WEBKIT_CHECK_VERSION` blocks, C 423-425 the `synthesize_focus_in_event` arm, C 451-456 the `mouse-target-changed` arm, C 476-477 the `#elif defined NS_IMPL_COCOA` arm) | file gate + arms |
| 482-491 | the blank and `DEFUN ("xwidget-live-p", …)` | file gate |
| 492-502 | the blank and C 493 `#ifdef USE_GTK` (`set_widget_if_text_view`) | `use_gtk` arm |
| 503-694 | the blank and `DEFUN ("xwidget-perform-lispy-event", …)` - C 513-523 and C 527-691 the two `USE_GTK` blocks (C 519-522/537-548/624-631/684-689 the XI2 arms, C 572-582 and C 604-616 the `HAVE_PGTK` pairs, C 644-650 the `HAVE_X_WINDOWS` pair) | file gate + arms |
| 695-719 | the blank and `DEFUN ("get-buffer-xwidgets", …)` | file gate |
| 720-725 | the blank and `xwidget_hidden` | file gate |
| 726-737 | the blank and `xwidget_from_id` | file gate |
| 738-1180 | the blank and C 739 `#ifdef USE_GTK` (whose `#endif` is C 2281): `pick_embedded_child` (C 740-764), `record_osr_embedder` (C 766-782), `find_xwidget_for_offscreen_window` (C 784-804), `from_embedder` (C 806-841), `to_embedder` (C 843-878), `find_suitable_pointer` (C 880-909), `find_suitable_keyboard` (C 911-921), `find_widget_cb` (C 923-927), `find_widget` (C 929-1018), `find_widget_at_pos` (C 1020-1062), C 1064 `#ifdef HAVE_PGTK`'s `cursor_for_hit` (C 1065-1082), `define_cursors` (C 1084-1111), `mouse_target_changed` (C 1113-1119) and `run_file_chooser_cb` (C 1122-1180) | `use_gtk` arm (+ `have_pgtk`/`have_x_windows` inside it) |
| 1181-4304 | **the remainder** - the blank and C 1182's `#ifdef HAVE_X_WINDOWS` through C 4304: the X11 drag/button/motion machinery (`xv_drag_begin_cb`, `xwidget_button_1`, `xwidget_button`, the XI2 group `xwidget_motion_notify`/`xwidget_scroll`/`xwidget_pinch`, the crossing machinery `window_coords_from_toplevel`/`xw_find_common_ancestor`/`xw_notify_virtual_upwards_until`/`xw_notify_virtual_downwards_until`/`xw_update_cursor_for_view`/`xw_last_crossing_cursor_cb`/`xw_maybe_synthesize_crossing`/`xwidget_motion_or_crossing`), the four `#else`/`#elif` arms of C 739 at C 2159/2175/2212/2261, the WebKit callbacks (`offscreen_damage_event`, `synthesize_focus_in_event`, `webkit_view_load_changed_cb`, `webkit_create_cb`, `webkit_javascript_finished_cb`, `webkit_download_cb`, `webkit_decide_policy_cb`, `webkit_script_dialog_cb`), the `NS_IMPL_COCOA`/`HAVE_PGTK` `#elif` bodies (C 2758-3321), the store/kill/redisplay helpers, the remaining ~60 subrs (`xwidget_resize` … `xwidget-webkit-stop-loading`) and the `DEFVAR`s and `syms_of_xwidget` at the end | not this slice: recorded here so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows |

## Evidence` readings and the `## Deviations`.
>
> **This ledger replaces, not extends, what it said before.**  Until this batch it read "the
> frozen configuration does not compile this file ⇒ the port is the empty unit with this note"
> (the P-08 reading, `docs/status.md` 3.31), which is what the 11-line placeholder was.  P-86 ①
> replaces that with *transcribe the text, both arms, no stub*; what the earlier note measured
> (not in the frozen source list, empty checkbox sections, 336-byte object) is unchanged and still
> true here - it is the *conclusion* that is withdrawn, exactly as `docs/ports/gtkutil.md`,
> `docs/ports/pgtkterm.md` and `docs/ports/androidmenu.md` record for their own files.  Port:
> `femacs/src/xwidget.rs` (**2,321 lines**, from the 11 of the placeholder), carrying **C 1-1180**
> with **both arms of every `#if`**.
>
> **This slice is C 1-1180**, from the file's first line through `run_file_chooser_cb`'s closing
> brace (C 1122-1180): the include list (C 20-54), the four file-scope objects (C 56-59), the
> seven forward declarations and the two allocators (C 61-92), `struct widget_search_data`
> (C 118-125), the PGTK pair `xw_forward_event_translate`/`xw_forward_event_from_view`
> (C 134-227), the X11 pair `enum xw_crossing_mode`/`xw_translate_x_modifiers` (C 231-266), the
> four subrs `Fmake_xwidget` (C 280-481), `Fxwidget_live_p` (C 483-491),
> `Fxwidget_perform_lispy_event` (C 504-694) and `Fget_buffer_xwidgets` (C 696-719), the two
> helpers `xwidget_hidden`/`xwidget_from_id` (C 721-737) and the file's wide GTK arm from
> `pick_embedded_child` (C 740-764) through `run_file_chooser_cb` (C 1122-1180), including
> `record_osr_embedder` (C 766-782), `find_xwidget_for_offscreen_window` (C 784-804),
> `from_embedder`/`to_embedder` (C 806-878), `find_suitable_pointer`/`find_suitable_keyboard`
> (C 880-921), `find_widget_cb`/`find_widget` (C 923-1018), `find_widget_at_pos` (C 1020-1062) and
> the PGTK trio `cursor_for_hit`/`define_cursors`/`mouse_target_changed` (C 1064-1119).
> **The next slice opens at C 1182's `#ifdef HAVE_X_WINDOWS`**, whose first definition is
> `xv_drag_begin_cb` (C 1184-1200); C 1181 is the blank the table needs no row for.  The remainder
> (C 1182-4304) is the table's last row.
>
> **The file gate is `feature = "use_gtk"`** - the mandate's ruling, and C 37's `#ifdef USE_GTK` is
> the file's first and widest one - on every item *and on the include graph* (P-86 ⑧: an
> `include!` outside the gate makes the frozen build fail instead of producing the empty object the
> class expects; this slice's C 22-34 list is written outside any `#if` and carries the gate anyway,
> which is `## Deviations` 3).  The slice's inner conditions are C 41's `#ifndef HAVE_PGTK`, C 47's
> `#ifdef HAVE_XINPUT2`, C 50's `#elif defined NS_IMPL_COCOA`, C 61/98/298/329/493/513/527/739's
> `#ifdef USE_GTK`, C 62/230/1027's `#ifdef HAVE_X_WINDOWS`, C 64/339/396's `WEBKIT_CHECK_VERSION`,
> C 130/451/1064's `#ifdef HAVE_PGTK`, C 423's `#if !defined HAVE_XINPUT2 && !defined HAVE_PGTK`,
> C 519/537/624/684's `#ifdef HAVE_XINPUT2`, C 572/604/774/813/850's `#ifndef HAVE_PGTK` and
> C 644's `#ifndef HAVE_X_WINDOWS` - all decided by the frozen table, and **both arms of every one
> of them are carried** (the `## Gate mapping` table below).
>
> **`WEBKIT_CHECK_VERSION` is decided by `clang`, not by a `configure` macro.**  The reference tree
> defines it nowhere (`grep -rn WEBKIT_CHECK_VERSION emacs/src/*.h` is empty); it comes from
> WebKitGTK's headers, and on this machine
> `clang -dM -E -std=gnu23 … /tmp/xw1/p.c` reports
> `error: function-like macro 'WEBKIT_CHECK_VERSION' is not defined` for C 64's condition, so no
> branch is taken (the same answer `tools/check-cond-eval.py` records: 0 disagreements).  The port
> spells the arms with derived-condition features after the `android_api_le_11` /
> `gtk_check_version_ge_3_10` naming: **`webkit_check_version_ge_2_16`**, **`_ge_2_26`**,
> **`_ge_2_34`** - **none registered in `femacs/xtask/Cargo.toml`** (`## Owed` 1).
>
> **The toolkit surface is declared in this unit (P-88 ①, which P-90 ① keeps for this round), and
> this batch is `text-only` (P-86 ③⑵).**  Measured on this machine:
> `ls -d /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0
> /opt/homebrew/include/webkit2gtk-4.0 /opt/homebrew/include/webkitgtk-4.0
> /opt/homebrew/include/JavaScriptCore` -> all `No such file or directory`; `pkg-config` is **not
> installed**; `find / -maxdepth 6 -name gdktypes.h` and `-name gdkevents.h` -> nothing (the same
> reading `docs/ports/gtkutil.md` records for the sibling lane).  So **GDK, GTK and WebKit have no
> tree to compile or measure against**: every citation of theirs names the header alone (no line,
> no probe), every size, offset, enum width and enumerator *value* of theirs is **UNVERIFIED**, no
> arm flip is possible and no layout assertion is written for a GDK/GTK/WebKit record.
> **GLib/GObject, cairo and X11 *are* on this machine** (`/opt/homebrew/include/glib-2.0`, GLib
> 2.90.0, with the generated `glibconfig.h` under
> `/opt/homebrew/Cellar/glib/2.90.0/lib/glib-2.0/include`; `/opt/homebrew/include/cairo`;
> `/opt/homebrew/include/X11`), so the GLib declarations quote the header *and the line* they were
> read from, `GList`/`GSList`'s layouts are real clang readings (Evidence ⑤) and `ShiftMask`/
> `ControlMask` carry `X11/X.h`'s own expressions.  P-88 ⑤/P-90 ③: the move into a dedicated SDK
> mirror (`femacs/src/include/gtk.rs`, PORTING.md P-90) is owed (`## Owed` 2), and this unit's copy
> plus the segment copies in `src/gtkutil.rs` and `src/pgtkterm.rs` are the ones to drop there.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 4422-4474: `XWIDGETS_OBJ="xwidget.o"` at 4434 and 4463, `AC_DEFINE ([HAVE_XWIDGETS], [1])` at 4474) | `feature = "use_gtk"` on every item *and on the include graph* (P-86 ⑧; C 37's `#ifdef USE_GTK` is the file's own gate) | a GTK xwidgets configuration (the mandate's ruling; `## Deviations` 3) |
| C 37 `#ifdef USE_GTK` … C 50 `#elif defined NS_IMPL_COCOA` … C 52 (the bottom-end include chain) | `feature = "use_gtk"` / `all (feature = "use_gtk", feature = "ns_impl_cocoa")` | GTK / Cocoa |
| C 41 `#ifndef HAVE_PGTK` (C 42-43 `<cairo-xlib.h>` + `<X11/Xlib.h>`, C 45 `#else`'s `<gtk/gtk.h>`) | `not (feature = "have_pgtk")` / `feature = "have_pgtk"` | X11 GTK / PGTK |
| C 47 `#ifdef HAVE_XINPUT2` (C 48 `<X11/extensions/XInput2.h>`) | `feature = "have_xinput2"` | XInput2 |
| C 62 `#ifdef HAVE_X_WINDOWS` (C 63 `x_window_to_xwv_map`) and C 64 `#if WEBKIT_CHECK_VERSION (2, 34, 0)` (C 65 `dummy_tooltip_string`) | `feature = "have_x_windows"`, then `feature = "webkit_check_version_ge_2_34"` (`## Owed` 1) | X11 GTK with WebKit ≥ 2.34 |
| C 98 `#ifdef USE_GTK` (C 118-125 `widget_search_data`, C 127 `find_widget`) | `feature = "use_gtk"` | GTK |
| C 130 `#ifdef HAVE_PGTK` (C 134-227) | `all (feature = "use_gtk", feature = "have_pgtk")` | PGTK |
| C 230 `#ifdef HAVE_X_WINDOWS` (C 231-277) | `all (feature = "use_gtk", feature = "have_x_windows")` | X11 GTK |
| C 298 `#ifdef USE_GTK` (the `xg_gtk_initialized` check) | `feature = "use_gtk"` | GTK |
| C 329 `#ifdef USE_GTK` … C 476 `#elif defined NS_IMPL_COCOA` … C 478 | `feature = "use_gtk"` / `all (feature = "use_gtk", feature = "ns_impl_cocoa")` | GTK / Cocoa (`## Deviations` 4) |
| C 339 `#if WEBKIT_CHECK_VERSION (2, 26, 0)` and C 396 `#if WEBKIT_CHECK_VERSION (2, 16, 0)` | `feature = "webkit_check_version_ge_2_26"` / `_ge_2_16` (`## Owed` 1) | WebKit ≥ those versions |
| C 423 `#if !defined HAVE_XINPUT2 && !defined HAVE_PGTK` (C 424 `synthesize_focus_in_event`) | `all (not (feature = "have_xinput2"), not (feature = "have_pgtk"))` | neither |
| C 451 `#ifdef HAVE_PGTK` (the `mouse-target-changed` connection) | `feature = "have_pgtk"` | PGTK |
| C 493 `#ifdef USE_GTK` (C 494-501 `set_widget_if_text_view`) | `feature = "use_gtk"` | GTK |
| C 513/527 `#ifdef USE_GTK` (the declarations and the body of `Fxwidget_perform_lispy_event`) | `feature = "use_gtk"` | GTK |
| C 519/537/624/684 `#ifdef HAVE_XINPUT2` (the four XI2 blocks) | `feature = "have_xinput2"` | XInput2 |
| C 572/604 `#ifndef HAVE_PGTK` … `#else` … C 582/616 | `not (feature = "have_pgtk")` / `feature = "have_pgtk"` | X11 GTK / PGTK |
| C 644 `#ifndef HAVE_X_WINDOWS` … `#else` (C 646-649) … C 650 | `not (feature = "have_x_windows")` / `feature = "have_x_windows"` | PGTK / X11 GTK |
| C 739 `#ifdef USE_GTK` (C 739-2281: the file's wide GTK arm; this slice carries C 739-1180) | `feature = "use_gtk"` | GTK |
| C 774/813/850 `#ifndef HAVE_PGTK` … `#else` … C 778/840/877 | `not (feature = "have_pgtk")` / `feature = "have_pgtk"` | X11 GTK / PGTK |
| C 1027 `#ifdef HAVE_X_WINDOWS` (the passive-grab arm of `find_widget_at_pos`) | `feature = "have_x_windows"` | X11 GTK |
| C 1064 `#ifdef HAVE_PGTK` (C 1065-1119 `cursor_for_hit`/`define_cursors`/`mouse_target_changed`) | `all (feature = "use_gtk", feature = "have_pgtk")` | PGTK |
| the records this slice reads: `include/xwidget.rs` gates `struct xwidget`/`struct xwidget_view`/`XXWIDGET`/`XXWIDGET_VIEW`/`CHECK_LIVE_XWIDGET` … on `have_xwidgets` (the header's own `#ifdef HAVE_XWIDGETS`, xwidget.h 31) | the mirror's gate; the port adds nothing (`## Deviations` 6) | xwidgets enabled |

Frozen values behind every arm: `USE_GTK undef` (`darwin-arm64.macros.tsv` 3861), `HAVE_PGTK undef`
(1489), `HAVE_XINPUT2 undef` (1702), `HAVE_X_WINDOWS undef` (1720), `NS_IMPL_COCOA undef` (2996),
`HAVE_XWIDGETS undef` (1717), `HAVE_GTK3 undef` (1385).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 20 | `#include <config.h>` | ungated (every unit includes it) |
| 22-34 | `buffer.h`, `coding.h`, `xwidget.h`, `lisp.h`, `blockinput.h`, `dispextern.h`, `frame.h`, `keyboard.h`, `gtkutil.h`, `sysstdio.h`, `termhooks.h`, `window.h`, `process.h` | file gate (and `lisp.h` is included first, `## Deviations` 1) |
| 36-54 | `/* Include xwidget bottom end headers.  */`, C 37 `#ifdef USE_GTK`/C 50 `#elif defined NS_IMPL_COCOA`/C 52 `#endif` (C 38-45 WebKit/cairo/X11/GTK, C 41 `#ifndef HAVE_PGTK`, C 47 `#ifdef HAVE_XINPUT2`, C 51 `nsxwidget.h`) and C 54 `#include <math.h>` | file gate / arms (the declarations this chain owes stand here, `## Deviations` 7) |
| 56-59 | `id_to_xwidget_map`, `internal_xwidget_view_list`, `internal_xwidget_list`, `xwidget_counter` | file gate |
| 61-77 | C 61 `#ifdef USE_GTK` (C 62 `#ifdef HAVE_X_WINDOWS` with C 63 `x_window_to_xwv_map` and C 64 `#if WEBKIT_CHECK_VERSION (2, 34, 0)` with C 65 `dummy_tooltip_string`) and C 68-76's seven forward declarations | file gate / `have_x_windows` / version arm; the declarations are dropped (`## Deviations` 2) |
| 79-83 | `allocate_xwidget` | file gate |
| 85-89 | `allocate_xwidget_view` | file gate |
| 91-92 | `XSETXWIDGET`, `XSETXWIDGET_VIEW` | file gate |
| 94-96 | `xwidget_view_lookup`, `kill_xwidget` forward declarations | dropped (`## Deviations` 2) |
| 98-128 | C 98 `#ifdef USE_GTK` (C 99-116 seven forward declarations, C 118-125 `struct widget_search_data`, C 127 `find_widget` declaration) | file gate; the declarations are dropped |
| 130-228 | C 130 `#ifdef HAVE_PGTK` (C 131-132 `mouse_target_changed` declaration, C 134-202 `xw_forward_event_translate`, C 204-227 `xw_forward_event_from_view`) | `have_pgtk` arm |
| 230-278 | C 230 `#ifdef HAVE_X_WINDOWS` (C 231-236 `enum xw_crossing_mode`, C 238-266 `xw_translate_x_modifiers`, C 268-277 the three declarations) | `have_x_windows` arm; the declarations are dropped |
| 280-481 | `DEFUN ("make-xwidget", …)` — C 298-301 the `xg_gtk_initialized` arm, C 329-478 the main arm (C 339-342 and C 396-403 the two `WEBKIT_CHECK_VERSION` blocks, C 423-425 the `synthesize_focus_in_event` arm, C 451-456 the `mouse-target-changed` arm, C 476-477 the `#elif defined NS_IMPL_COCOA` arm) | file gate + arms |
| 483-491 | `DEFUN ("xwidget-live-p", …)` | file gate |
| 493-502 | C 493 `#ifdef USE_GTK` (`set_widget_if_text_view`) | `use_gtk` arm |
| 504-694 | `DEFUN ("xwidget-perform-lispy-event", …)` — C 513-523 and C 527-691 the two `USE_GTK` blocks (C 519-522/537-548/624-631/684-689 the XI2 arms, C 572-582 and C 604-616 the `HAVE_PGTK` pairs, C 644-650 the `HAVE_X_WINDOWS` pair) | file gate + arms |
| 696-719 | `DEFUN ("get-buffer-xwidgets", …)` | file gate |
| 721-725 | `xwidget_hidden` | file gate |
| 727-737 | `xwidget_from_id` | file gate |
| 739-1180 | C 739 `#ifdef USE_GTK` (whose `#endif` is C 2281): `pick_embedded_child` (C 740-764), `record_osr_embedder` (C 766-782), `find_xwidget_for_offscreen_window` (C 784-804), `from_embedder` (C 806-841), `to_embedder` (C 843-878), `find_suitable_pointer` (C 880-909), `find_suitable_keyboard` (C 911-921), `find_widget_cb` (C 923-927), `find_widget` (C 929-1018), `find_widget_at_pos` (C 1020-1062), C 1064 `#ifdef HAVE_PGTK`'s `cursor_for_hit` (C 1065-1082), `define_cursors` (C 1084-1111), `mouse_target_changed` (C 1113-1119) and `run_file_chooser_cb` (C 1122-1180) | `use_gtk` arm (+ `have_pgtk`/`have_x_windows` inside it) |
| 1182-4304 | **the remainder** — C 1182's `#ifdef HAVE_X_WINDOWS` through C 4304: the X11 drag/button/motion machinery (`xv_drag_begin_cb`, `xwidget_button_1`, `xwidget_button`, the XI2 group `xwidget_motion_notify`/`xwidget_scroll`/`xwidget_pinch`, the crossing machinery `window_coords_from_toplevel`/`xw_find_common_ancestor`/`xw_notify_virtual_upwards_until`/`xw_notify_virtual_downwards_until`/`xw_update_cursor_for_view`/`xw_last_crossing_cursor_cb`/`xw_maybe_synthesize_crossing`/`xwidget_motion_or_crossing`), the two `#else` arms of C 739 at C 2159/2175/2212/2261, the WebKit callbacks (`offscreen_damage_event`, `synthesize_focus_in_event`, `webkit_view_load_changed_cb`, `webkit_create_cb`, `webkit_javascript_finished_cb`, `webkit_download_cb`, `webkit_decide_policy_cb`, `webkit_script_dialog_cb`), the `NS_IMPL_COCOA`/`HAVE_PGTK` `#elif` bodies (C 2758-3321), the store/kill/redisplay helpers, the remaining ~60 subrs and `syms_of_xwidget`/`syms_of_xwidget`'s `DEFVAR`s at the end | not this slice: recorded here so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows |

## Evidence

**① The frozen reading — `build-tu` (the P-86 ⑧ gate check).**
`cd femacs && CARGO_TARGET_DIR=/tmp/xw1-lane/cargo cargo run -q -p xtask -- build-tu src/xwidget.c`
-> `built src/xwidget.rs -> ../build/femacs/xwidget.o`, **exit 0**, and `build/femacs/xwidget.o`
is **336 bytes** - an empty object, which is the expected reading and not a proof of the new text:
every item sits behind `use_gtk`, so under the frozen feature set rustc compiles only
`include/config.rs` and the `#![allow]` attribute (the same 336 bytes `docs/ports/gtkutil.md`,
`docs/ports/pgtkterm.md` and `docs/ports/androidmenu.md` report for their own files).  The run also
materialises `build/femacs/units/xwidget/_root.rs` (**2,321 lines**, the unit's own line count),
which is what readings ② below compile.

**② The arm readings (three, `text-only`, P-86 ③⑵).**  There is no GTK tree on this machine
(above), so no arm can be *compiled against GTK*; the readings are rustc's name-resolution and type
errors on the materialised root, with the features a real target would set.  Counted **by file**
(P-86 ⑦), the errors split like this:

```
$ rustc --edition 2024 --crate-type=lib --emit=metadata -C panic=abort
        --cfg 'feature="use_gtk"' --cfg 'feature="have_xwidgets"'
        --cfg 'feature="have_x_windows"' --cfg 'feature="have_window_system"'
        build/femacs/units/xwidget/_root.rs                                          # ARM A
exit 1, 52 error(s): include/xterm.rs 20, the unit's own text 8, include/buffer.rs 8,
include/dispextern.rs 7, include/xwidget.rs 5, include/termhooks.rs 3, include/lisp.rs 1

$ rustc … --cfg 'feature="use_gtk"' --cfg 'feature="have_pgtk"' --cfg 'feature="have_xwidgets"'
        --cfg 'feature="have_window_system"' …                                       # ARM B
exit 1, 43 error(s): the unit's own text 21, include/buffer.rs 8, include/dispextern.rs 6,
include/xwidget.rs 4, include/termhooks.rs 3, include/lisp.rs 1

$ rustc … --cfg 'feature="use_gtk"' --cfg 'feature="have_xwidgets"' …                # ARM C
exit 1, 40 error(s): the unit's own text 19, include/buffer.rs 8, include/xwidget.rs 5,
include/dispextern.rs 4, include/termhooks.rs 3, include/lisp.rs 1
```

**Every one of this unit's own errors is one of the two classes named below - none is a defect of a
C statement's transcription:**

| arm | this unit's errors | class | anchor |
| --- | ---: | --- | --- |
| A | 6 | `cannot find value …` for `webkit_download_cb` (C 363), `webkit_view_load_changed_cb` (C 444), `webkit_decide_policy_cb` (C 449), `webkit_create_cb` (C 459), `webkit_script_dialog_cb` (C 463), `offscreen_damage_event` (C 472) | definitions in the *remainder* (C 2561/2427/2576/2407/2647/2247); the call sites stay as C's (the `docs/ports/gtkutil.md` Deviations-3 shape) |
| A | 1 | `cannot find function synthesize_focus_in_event` (C 424) | the remainder (C 2110) |
| A | 1 | `no field kill_without_query on type xwidget` (C 319) | `include/xwidget.rs` 107-108 records the `bool_bf … : 1` member instead of storing it (P-80's member-dispatch accessor is the fix) |
| B | 14 | `cannot find type pgtk_output` (4), `cannot find macro FRAME_X_OUTPUT` (3), `FRAME_GTK_WIDGET` (2), `FRAME_DISPLAY_INFO` (2), `FRAME_GTK_OUTER_WIDGET` (1), `FRAME_OUTPUT_DATA` (1), `cannot find type pgtk_display_info` (1) | `include/pgtkterm.rs` is a **36-line placeholder** (it carries the include guard and a note; none of pgtkterm.h's records or the fifty `FRAME_*` accessors) |
| B | 7 | the same seven as arm A | as above |
| C | 11 | ten `cannot find macro` for `FRAME_GTK_OUTER_WIDGET` (4), `FRAME_GTK_WIDGET` (4) and `FRAME_DISPLAY_INFO` (2), plus `cannot find type x_display_info` (1) | with `have_x_windows` off, `include/xterm.rs`'s body is inert, so the X-only names have no definition - the same class as arm B's pgtkterm placeholder, in the other direction |
| C | 8 | the same eight as arm A | as above |

The **44 mirror-anchored errors of arm A** are not this unit's: `include/xterm.rs`'s 20 are sixteen
`E0133` (`pub fn` bodies of the X11 helpers dereferencing `dpyinfo`/`visual`), `include/buffer.rs`'s
8 and `include/dispextern.rs`'s 7 are names the two mirrors' own gates do not supply
(`STRING_CHAR`/`MAX_CHAR`/`font`/`Emacs_Pix_Container`), `include/xwidget.rs`'s 5 are `Emacs_Cursor`
and `Time`/`cairo_t`/`cairo_surface_t` in the record it declares, and `termhooks.rs`/`lisp.rs` have
one each.  **The same reading appears on the sibling lane's root**: compiling
`build/femacs/units/gtkutil/_root.rs` with the same four `--cfg`s exits 1 with 15 of its errors in
the same `include/xterm.rs`, 8 in the same `include/buffer.rs` and 7 in the same
`include/dispextern.rs` - the arm reading's mirror half is a property of the mirrors, not of this
slice.

**②a The arm reading is not vacuous - measured (P-86 ⑦).**  Injecting a deliberate type error into
the unit's own text (`let _mutation_probe: c_int = c"injected".as_ptr ();` in `xwidget_hidden`)
changes arm A's output as it should: the injected error is reported at the unit's own line
(`1814 |   let _mutation_probe: c_int = c"injected".as_ptr ();`) and the run's own summary moves
from `aborting due to 52 previous errors` to `aborting due to 53 previous errors`.  The include graph therefore
resolves through this unit's text and the reading above is a real one.

**②b The `have_xinput2` arms are *not* flipped.**  `HAVE_XINPUT2` is `undef` on this target and the
XI2 blocks (C 519-522/537-548/624-631/684-689) need XInput2's headers, which this machine has not
got for the GTK lane; they are carried as text only (`## Owed` 6).

**③ Inventory.**
`python3 tools/extract-abi.py --check emacs/src/xwidget.c femacs/src/xwidget.rs` ->
**85 `- ` lines, then `FAIL`**, in five groups: 18 `- missing function …`, 30 `- missing subr …`,
34 `- missing defsubr …` and 2 `- missing macro …` (`CHECK_WEBKIT_WIDGET`, `WEBKIT_FN_INIT`) - all
of them **the unported remainder** (C 1182-4304: the `webkit_*` callbacks, `lookup_xwidget`,
`kill_xwidget`, `xwidget_view_lookup`, the 30 remaining subrs, the `defsubr` calls inside
`syms_of_xwidget`) - plus `- line count drifted 46.1%: C 4304 vs Rust 2321` (the whole-file ratio
against a one-slice port).  **No `- function <name> does not mention …` line and no invented
symbol appear**, i.e. every function body this slice carries passes the token comparison, and the
unit's four subrs are *not* in the missing list.  The 34 `- missing defsubr` rows are the
`defsubr` calls inside `syms_of_xwidget`, which is in the remainder.  The line-count row is owed
(`## Owed` 5).

**④ The other gates (repo root, final revision).**

```
python3 tools/check-cond-eval.py src/xwidget.c
  → src/xwidget.c: 121 conditions, 0 unknown, 0 disagreements [VERIFIED]              [exit 0]
python3 tools/check-declarations.py
  → 10 declaration sites (0 forbidden, 10 allowed literals), 0 failures              [exit 0]
python3 tools/check-mirror-dupes.py
  → 19 duplicate declaration(s), 19 ratchet row(s), 0 failure(s)                      [exit 0]
python3 tools/check-dead-file-coverage.py
  → info docs/ports/xwidget.md: 22 range row(s) cover src/xwidget.c (4304 lines; 0 blank line(s)
    between blocks need no row)
    info docs/ports/xwidget.md: 22 range row(s) cover src/xwidget.h (243 lines; 0 blank line(s)
    between blocks need no row)                        … 44 ledger(s) bound, 0 failure(s) [exit 0]
python3 tools/check-ledger-ticks.py
  → 234 ledger(s) bound (a function tick to compare), 0 failure(s), 0 carried by another file,
    356 row(s) with no function tick … 0 row(s) without a ledger                       [exit 0]
```

**⑤ The readings the declarations were copied from, and the one probe.**  Measured on this machine
with clang and the headers themselves (GLib 2.90.0 at `/opt/homebrew/include/glib-2.0` plus
`glibconfig.h`; the X11 headers under `/opt/homebrew/include/X11`):

```
$ clang -std=gnu23 -O0 -I /opt/homebrew/include/glib-2.0 \
      -I /opt/homebrew/Cellar/glib/2.90.0/lib/glib-2.0/include /tmp/xw1/probe.c -o /tmp/xw1/probe \
      && /tmp/xw1/probe
sizeof_GList=24 data=0 next=8 prev=16
sizeof_GSList=16 data=0 next=8
sizeof_gsize=8 sizeof_gulong=8 sizeof_guint=4 sizeof_gboolean=4 sizeof_gint=4 sizeof_gpointer=8
```

Every `core::assert!` in the port is one of those five readings (`GList`'s size/offsets and
`GSList`'s).  The GLib/GObject declarations cite the header *and the line* (`gtypes.h` 52/55/56/60/
61/109, `glibconfig.h` 55/81, `glist.h` 41-46/53, `gslist.h` 41-45/52/125, `gobject.h` 54/455-457/
513/515/597-599, `gsignal.h` 184-206/440-446/554-556, `gmem.h` 74).  The **GDK, GTK and WebKit**
declarations **cite the header alone, with no line** (no such tree here, `## Deviations` 7), and
`ShiftMask`/`ControlMask` quote `X11/X.h` 221/223's own expressions.  The two `WEBKIT_CHECK_VERSION`
readings are the probes quoted in the file note (clang: *function-like macro … is not defined*;
evaluator: `0 disagreements`).

## Deviations

1. **Include order.**  C's list starts at `"buffer.h"` (C 22) and reaches `"lisp.h"` at C 25;
   PORTING.md 0.1-14 requires `config.rs` → `lisp.rs` → the rest, because `include/buffer.rs`,
   `include/coding.rs` and `include/xwidget.rs` expand `eassert!`/`XUNTAG!`/`CHECK_TYPE` textually
   inside their bodies.  The port therefore includes `lisp.rs` first and the remaining mirrors in
   C's order, and adds two edges `include/lisp.rs`'s own chain owes (`<string.h>` for `strcmp`
   at C 595, `<limits.h>` for `INT_MAX` at C 554; lisp.h 30/33) at the C position that makes them
   visible.
2. **C's forward declarations are dropped** (PORTING.md 0.1-12), and each definition stays at its C
   position: C 68-76's seven (`offscreen_damage_event`, `synthesize_focus_in_event`,
   `find_suitable_keyboard`, `webkit_script_dialog_cb`, `record_osr_embedder`, `from_embedder`,
   `to_embedder`, `pick_embedded_child`), C 94-96's two, C 99-116's seven, C 131-132's one and
   C 268-277's three.  Five of the drop targets are defined inside this slice; the other eight are
   in the remainder, so their call sites stay as C's and the arm compile reports them (Evidence ②).
3. **The file gate is applied to C's ungated items.**  C writes C 22-34's include list,
   `allocate_xwidget`/`allocate_xwidget_view` (C 79-89), the four subrs (C 280/483/504/696) and
   `xwidget_hidden`/`xwidget_from_id` (C 721-737) outside any `#if`; the port carries
   `feature = "use_gtk"` on them because the reference's build only compiles `xwidget.o` when
   xwidgets are enabled (configure.ac 4424-4474) and because P-86 ⑧ puts the gate on the include
   graph.  The consequence: the *mirror* records this slice reads are `have_xwidgets`-gated
   (`include/xwidget.rs` 10), so a `use_gtk`-only flip reads `cannot find type xwidget` - that is
   the mirror's own gate, not a second file gate the port invents (`## Owed` 3).
4. **C 476's `#elif defined NS_IMPL_COCOA` arm carries both names.**  C's `#elif` means
   `not USE_GTK ∧ NS_IMPL_COCOA`; the port keeps the file gate on every item (Deviation 3) and so
   spells it `all (feature = "use_gtk", feature = "ns_impl_cocoa")`.  The arm is dead in every
   configuration this tree can build (both macros are `undef`), and `nsxwidget_init` is declared at
   its C use site (P-72 rule 4, nsxwidget.h 53) because `include/nsxwidget.rs` belongs to the Cocoa
   lane.
5. **C's implicit conversions are explicit `as` casts in the port.**  Rust has no implicit
   arithmetic or pointer conversion, so every one of the following is spelled out at the C line it
   belongs to, with C's own expression unchanged: `lrint`'s `long` into `find_widget_at_pos`'s
   `int` parameters and back into the `gdouble` members (C 148-195, 757-758); `XFIXNAT`'s
   `EMACS_INT` into the `int` members `height`/`width` (C 317-318); `xwidget_id`'s `uint32_t` into
   `make_fixnum` (C 327); `*mut Widget` into `gpointer` (the eleven `g_signal_connect`s, C 361-472,
   and `g_object_set_data`/`g_object_set`/`g_object_ref`), `*mut xwidget_view` into `gpointer`
   (`mouse_target_changed`, C 1118) and the reverse (`user_data` in C 208, `xw` in C 1118);
   `Emacs_Cursor` (*mut GdkCursor*) into the mirror's `*mut c_void` stand-in and back (C 1101/1106);
   the `guint`/`GdkModifierType`/`gboolean` mixes of `xw_translate_x_modifiers` (C 244-263) and
   `gtk_bindings_activate` (C 670); `type`/`title` etc. unchanged.  The one place this changes a
   *macro* is the unit's own copy of `g_signal_connect` (C 361's `#define`), whose two arguments
   are cast inside the macro rather than at the eleven call sites.
6. **Names this slice takes from placeholders or from no mirror at all.**  `xg_gtk_initialized`
   (gtkutil.h 242), `decode_window_system_frame` (frame.h 1475), `parse_modifiers` (keyboard.h 461),
   `lispy_function_keys` (keyboard.h 525), `catch_child_signal` (process.h 303),
   `x_emacs_to_x_modifiers` (xterm.h 1809), `pgtk_emacs_to_gtk_modifiers` (pgtkterm.h 678),
   `nsxwidget_init` (nsxwidget.h 53) and `lrint` (math.h) are declared **at their C use site**
   (P-72 rule 4 / P-88 ①) because the mirror that owns each of them does not carry it; each `extern`
   has the mirror gap named in its own comment and all of them are listed in `## Owed` 3.  The
   `HAVE_PGTK`/`HAVE_X_WINDOWS` arms read `FRAME_GTK_WIDGET`, `FRAME_GTK_OUTER_WIDGET`,
   `FRAME_X_OUTPUT`, `FRAME_DISPLAY_INFO` and `frame`'s four `*_mod_mask` members through
   `include/xterm.rs` (the edge gtkutil.h 32 owes) - and through nothing in the PGTK arm, where
   `include/pgtkterm.rs` is a placeholder (Evidence ②).  `lispy_function_keys` is C's *incomplete*
   array; the port declares the 256 slots C's own loop bound (C 592) and keyboard.rs's X11 table
   name, because an incomplete array cannot be indexed.
7. **A GDK/GTK/WebKit record's members and every enumerator value are UNVERIFIED.**  The records
   carry exactly the members C reads (`GdkEvent`'s six union members plus `type`, `GdkEventAny`,
   `GdkEventButton`, `GdkEventScroll`, `GdkEventMotion`, `GdkEventCrossing`, `GdkEventKey`,
   `GtkAllocation`, `GtkContainerClass.forall`), and their sizes/offsets are not measured - no GDK
   or GTK header exists on this machine.  The enumerators and flag masks are **placeholders**: each
   keeps the reference's arms distinct (a different `switch` case, a different bit) without
   claiming GTK's number, because the port may not invent a value it cannot measure (PORTING.md
   P-88 ② and the mandate's "不许凭记忆写数字").  The names are `GDK_NOTHING`,
   `GDK_BUTTON_PRESS`, `GDK_BUTTON_RELEASE`, `GDK_2BUTTON_PRESS`, `GDK_3BUTTON_PRESS`, `GDK_SCROLL`,
   `GDK_MOTION_NOTIFY`, `GDK_ENTER_NOTIFY`, `GDK_LEAVE_NOTIFY`, `GDK_KEY_PRESS`, `GDK_KEY_RELEASE`,
   `GDK_MOD1_MASK`, `GDK_META_MASK`, `GDK_SUPER_MASK`, `GDK_HYPER_MASK`, `GDK_CONTROL_MASK`,
   `GDK_SHIFT_MASK`, `GDK_SEAT_CAPABILITY_ALL_POINTING`, `GDK_SOURCE_TOUCHPAD`,
   `GTK_RESPONSE_ACCEPT`, `GTK_FILE_CHOOSER_ACTION_OPEN`, `WEBKIT_HIT_TEST_RESULT_CONTEXT_*` (5),
   `WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER` (`## Owed` 2).  The four `GTK_IS_*`/`WEBKIT_IS_*`
   type checks keep their names with a null test instead of GLib's
   `G_TYPE_CHECK_INSTANCE_TYPE` chain (the `src/gtkutil.rs` 876-893 treatment).
8. **Locals C leaves to their callee or writes field by field are initialised first**
   (PORTING.md 0.1-13).  `GdkEvent event` and `GtkAllocation new_allocation` (C 746/933) carry
   C's own assignments but Rust cannot partially initialise, so the port zeroes them;
   `int xout, yout` (C 747), `gint tx, ty` (C 951) and `int xoff, yoff` (C 816/853) are written
   through their addresses and are zeroed for the same reason; `new_x`/`new_y` (C 139-140) likewise.
   `widget_search_data new_data` (C 994) needs the record to be `Copy`, which the unit's own copy
   derives.  `Fget_buffer_xwidgets`'s `buffer` parameter is reassigned at C 706, so the port rebinds
   it (`let mut buffer = buffer;`, the `src/androidmenu.rs` `contents` shape).
9. **Two C statements cannot be carried and are named rather than stubbed.**  `(*xw).kill_without_query
   = false;` (C 319) has no field to write - `include/xwidget.rs` records the bitfield instead of
   storing it - and the arm compile reports it (Evidence ②); `XG_XWIDGET`/`XG_XWIDGET_VIEW` are
   `&CStr` constants in that mirror, so C's string macro arrives as `XG_XWIDGET.as_ptr ()` at
   C 436-437.
10. **`xw_forward_event_translate` returns C's `int`, not a Rust `bool`.**  C 217 assigns it to a
    `bool`, and the port keeps the C type on the function and spells the conversion `!= 0` at the
    call site (the `src/androidmenu.rs` `popup_activated_flag` shape).

## Owed by the integrator (nothing here is claimed as done)

1. `femacs/xtask/Cargo.toml`'s `[features]` table: the three derived-condition names this slice
   needs - **`webkit_check_version_ge_2_16`**, **`webkit_check_version_ge_2_26`**,
   **`webkit_check_version_ge_2_34`** - and a `docs/baseline/feature-coverage.tsv` row each.  Until
   they exist, the three `WEBKIT_CHECK_VERSION` arms cannot be selected by any `--cfg`.
2. **The SDK mirror `femacs/src/include/gtk.rs`** (PORTING.md P-90: GTK/GDK/GLib/cairo/WebKit are
   surfaces with more than one consumer).  This unit's segment is the one to move; the segment
   copies in `src/gtkutil.rs` (its 220 names) and `src/pgtkterm.rs` must move in the same commit
   (P-90 ②/③).  The names this unit contributes: **GLib/GObject** - `gchar`, `gint`, `gboolean`,
   `gulong`, `guint`, `gpointer`, `guint32`, `gsize`, `TRUE`, `FALSE`, `_GList`/`GList`,
   `_GSList`/`GSList`, `GObject`, `GClosure`, `GClosureNotify`, `GConnectFlags`, `GCallback`,
   `G_OBJECT`, `G_CALLBACK`, `g_signal_connect`, `g_signal_connect_data`, `g_free`, `g_list_free`,
   `g_slist_length`, `g_slist_free`, `g_object_ref`, `g_object_unref`, `g_object_set`,
   `g_object_set_data`; **GDK** - `GdkWindow`, `GdkDevice`, `GdkSeat`, `GdkDisplay`, `GdkCursor`,
   `GdkEvent` (+ `GdkEventAny`, `GdkEventButton`, `GdkEventScroll`, `GdkEventMotion`,
   `GdkEventCrossing`, `GdkEventKey`), `GdkEventType`, `GdkCrossingMode`, `GdkModifierType`,
   `GdkSeatCapabilities`, `GdkInputSource`, `GDK_DEVICE`, the seventeen `GDK_*` constants of
   Deviations 7, and the eleven `gdk_*` entry points this slice calls; **GTK** - `GtkWidget`,
   `GtkWindow`, `GtkContainer`, `GtkTextView`, `GtkFileChooserNative`, `GtkFileFilter`,
   `GtkFileChooser`, `GtkNativeDialog`, `GtkAllocation`, `GtkCallback`, `GtkContainerClass`,
   `GtkResponseType`, `GtkFileChooserAction`, `GTK_WIDGET`, `GTK_WINDOW`, `GTK_CONTAINER`,
   `GTK_CONTAINER_CLASS`, `GTK_FILE_CHOOSER`, `GTK_NATIVE_DIALOG`, the three `GTK_IS_*`,
   `GTK_RESPONSE_ACCEPT`, `GTK_FILE_CHOOSER_ACTION_OPEN`, `G_OBJECT_GET_CLASS` and the seventeen
   `gtk_*` entry points; **WebKit** - `WebKitWebView`, `WebKitWebContext`, `WebKitSettings`,
   `WebKitHitTestResult`, `WebKitFileChooserRequest`, `WebKitScriptDialog`, `WebKitPolicyDecision`,
   `WebKitPolicyDecisionType`, `WebKitNavigationAction`, `WebKitLoadEvent`, `WebKitDownload`,
   `WebKitHardwareAccelerationPolicy`, `WEBKIT_WEB_VIEW`, `WEBKIT_IS_WEB_VIEW`,
   `WEBKIT_HIT_TEST_RESULT_CONTEXT_*`, `WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER` and the
   fourteen `webkit_*` entry points.  Every GDK/GTK/WebKit layout and enumerator value is
   **UNVERIFIED** and must be re-derived from a real tree in that commit (P-88 ②/P-90 ④).
3. **The declaration gaps the mirrors owe** (each is named at its use site in the unit): 
   `include/gtkutil.rs` - `xg_gtk_initialized` (gtkutil.h 242); `include/frame.rs` -
   `decode_window_system_frame` (frame.h 1475); `include/keyboard.rs` - `parse_modifiers`
   (keyboard.h 461) and `lispy_function_keys` (keyboard.h 525); `include/process.rs` -
   `catch_child_signal` (process.h 303); `include/xterm.rs` - `x_emacs_to_x_modifiers`
   (xterm.h 1809); `include/pgtkterm.rs` - everything (36-line placeholder: `pgtk_display_info`,
   `pgtk_output`, `FRAME_X_OUTPUT`, `FRAME_DISPLAY_INFO`, the fifty `FRAME_*` accessors,
   `pgtk_emacs_to_gtk_modifiers`, pgtkterm.h 678).  Two of those are **linkage** gaps as well:
   `src/keyboard.rs` 5975's `lispy_function_keys` and `src/process.rs` 8661's `catch_child_signal`
   are plain Rust items (`static mut` / `pub unsafe fn`), so a real GTK build cannot link the C
   names this unit references.  `include/dispextern.rs`'s `Emacs_Cursor` (dispextern.h 39,
   `typedef Cursor Emacs_Cursor;` under `HAVE_WINDOW_SYSTEM`) and `include/xterm.rs`'s sixteen
   `pub fn` bodies that dereference raw pointers (20 `E0133` in Evidence ②; the same reading
   appears on `src/gtkutil.rs`'s root) are the other two.
4. **A first arm flip that can actually link.**  With ③'s gaps closed, ARM A should be re-run; the
   eight unit-anchored errors of Evidence ② are the only errors this slice can remove on its own,
   and seven of them need the remainder (C 1182-4304) ported.
5. `docs/ports/line-drift-tolerance.tsv`: one row, stem `xwidget`, `total`, with the basis from
   Evidence ③ (C 4,304 vs Rust 2,321 is the *whole file* against a one-slice port; this slice's own
   ratio is C 1,180 → 2,321 lines, i.e. the P-86 per-item `#[cfg]` gates, the ~600 lines of
   third-party declarations the class requires, and Rust's explicit casts).
6. `HAVE_XINPUT2`'s four blocks (C 519-522/537-548/624-631/684-689) are carried as text but were
   never flipped: this machine has no XInput2 headers for the GTK lane, so the reading is owed to a
   configuration that can compile them.
7. **The next slice.**  C 1182-4304, opening at `#ifdef HAVE_X_WINDOWS` with `xv_drag_begin_cb`
   (C 1184-1200); this table's last row is the placeholder for it.
