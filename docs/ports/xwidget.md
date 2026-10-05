# emacs/src/xwidget.c (4,304 lines) — slices C 1-1180 and C 1181-2716

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
> **The file gate is `all (feature = "use_gtk", feature = "have_gtk3")`** (corrected 2026-10-05, status 3.596: it used
> to be `use_gtk` alone, which is C 37's *content* guard - `configure.ac` 4424-4434 build `XWIDGETS_OBJ="xwidget.o"`
> only under `USE_GTK_TOOLKIT = "GTK3"`, the same block as `AC_DEFINE([HAVE_GTK3])` at 3756-3759, while `USE_GTK`
> (3826) covers GTK2 as well).  C 37's `#ifdef USE_GTK` is
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

> **Slice 2 (C 1181-2716) — the `HAVE_X_WINDOWS` arm, the WebKit callbacks and the script
> dialog.**  This round took `femacs/src/xwidget.rs` from **2,014 to 4,492 lines**: C 1181's blank
> and C 1182's `#ifdef HAVE_X_WINDOWS` through C 2716's `#endif /* USE_GTK */` — the X11
> drag/button/motion and crossing machinery (C 1184-2105), `synthesize_focus_in_event` (C 2110),
> `xwidget_view_from_window` (C 2137), the show/hide pair (C 2150/2169),
> `xv_do_draw`/`xwidget_view_draw_cb` (C 2182/2214), `offscreen_damage_event` (C 2247),
> `xwidget_expose` (C 2274), the three `store_xwidget_*_event` helpers (C 2284-2331) and the seven
> WebKit callbacks (C 2333-2714).  It stops at C 2716; **C 2718's `xwidget_init_view` is the next
> slice's first definition** (the C line-range rows 1181-2107 / 2108-2281 / 2282-2331 /
> 2332-2716 are this slice's; 2717-4304 is the remainder).  Both arms of every `#if` are carried:
> `have_x_windows` (all of C 1182-2105), `have_xinput2` (C 1387-1553 and C 1555-1575),
> `have_xinput2_4` + `gtk_check_version_ge_3_18` (C 1489-1553), `have_pgtk` (C 2180-2242 and
> C 2155-2166/2173-2176/2259-2267's `#else` arms), `not (have_pgtk)` (C 2180-2210),
> `not (have_x_windows)` (the same three `#else` arms) and `webkit_check_version_ge_2_34`
> (C 1786-1789).
>
> **This slice reads a second, larger toolkit surface (P-88 ①; the P-90 ③ move is still owed).**
> The names stand at the head of the slice's text, each with a `/// C:` citation: GDK's notify/
> crossing/scroll/touchpad enumerations and `GdkEventType`'s two missing members
> (`GDK_FOCUS_CHANGE`, `GDK_TOUCHPAD_PINCH`), `GdkScrollDirection`, `GdkTouchpadGesturePhase`,
> `GdkNotifyType`, `GDK_BUTTON{1,2,3}_MASK`, `G_MAXINT32`, `GAsyncResult`; WebKit's navigation/
> policy/script-dialog enumerations and `WebKitJavascriptResult`, `WebKitURIRequest`,
> `WebKitNavigationPolicyDecision`, `WebKitResponsePolicyDecision`, `WebKitScriptDialogType` and
> `JSCValue` with the eleven `jsc_value_*` entry points; the six GDK/GLib calls and the eleven GTK
> calls (plus `GtkOffscreenWindow`/`GtkEntry`/`GtkDialog` and their three `GTK_*` casts); cairo's
> `save`/`restore`/`translate`; the XInput2 records
> `XIEnterEvent`/`XIGesturePinchEvent`/`XIAddData`/`XIButtonState` with `XIMaskIsSet!` and the five
> `XINotify*`/five `XI_*` values; and the X11 `XEvent` member records
> (`XAnyEvent`/`XMotionEvent`/`XCrossingEvent`/`XGenericEventCookie`, **measured** - Evidence ⑥)
> plus the six `XMoveWindow`-family entry points.  Every GDK/GTK/WebKit/XInput2 value and layout is
> **UNVERIFIED** here (no tree on this machine); the X11 ones are measured.  `XWIDGET_EVENT`/
> `XWIDGET_DISPLAY_EVENT` - termhooks.h's two `HAVE_XWIDGETS` `enum event_kind` members, which
> `include/termhooks.rs` does not carry - are declared here at the values a `HAVE_XWIDGETS` build
> gives them (28/29; `## Owed` 11).
>
> **The arm flip this round is `direct rustc` on the materialised root, not a GTK compile.**  GDK,
> GTK and WebKit still have no tree here, so the readings are rustc's name-resolution and type
> errors on the materialised `_root.rs` under hand-set `--cfg`s (Evidence ⑦/⑧), and
> `HAVE_X_WINDOWS`/`HAVE_PGTK` cannot be flipped together (they are mutually exclusive in
> `configure.ac`, and the combination aborts on a mirror `E0428`, Evidence ⑧).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 4422-4474: `XWIDGETS_OBJ="xwidget.o"` inside `with_xwidgets != no` **and** `USE_GTK_TOOLKIT = "GTK3"` at 4434, a second arm at 4463 for Cocoa, `AC_DEFINE ([HAVE_XWIDGETS], [1])` at 4474) | `all (feature = "use_gtk", feature = "have_gtk3")` on every item *and on the include graph* (P-86 ⑧; corrected in 3.596 - `USE_GTK` alone also covers GTK2, where C builds no `xwidget.o`) | a GTK**3** xwidgets configuration; the Cocoa arm is the two `ns_impl_cocoa` items |
| C 37 `#ifdef USE_GTK` … C 50 `#elif defined NS_IMPL_COCOA` … C 52 (the bottom-end include chain) | `all (feature = "use_gtk", feature = "have_gtk3")` / `all (feature = "use_gtk", feature = "ns_impl_cocoa")` | GTK3 / Cocoa |
| C 41 `#ifndef HAVE_PGTK` (C 42-43 `<cairo-xlib.h>` + `<X11/Xlib.h>`, C 45 `#else`'s `<gtk/gtk.h>`) | `not (feature = "have_pgtk")` / `feature = "have_pgtk"` | X11 GTK / PGTK |
| C 47 `#ifdef HAVE_XINPUT2` (C 48 `<X11/extensions/XInput2.h>`) | `feature = "have_xinput2"` | XInput2 |
| C 62 `#ifdef HAVE_X_WINDOWS` (C 63 `x_window_to_xwv_map`) and C 64 `#if WEBKIT_CHECK_VERSION (2, 34, 0)` (C 65 `dummy_tooltip_string`) | `feature = "have_x_windows"`, then `feature = "webkit_check_version_ge_2_34"` (`## Owed` 1) | X11 GTK with WebKit ≥ 2.34 |
| C 98 `#ifdef USE_GTK` (C 118-125 `widget_search_data`, C 127 `find_widget`) | `all (feature = "use_gtk", feature = "have_gtk3")` | GTK3 |
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
| C 1182 `#ifdef HAVE_X_WINDOWS` (C 1184-2105) | `all (feature = "use_gtk", feature = "have_x_windows")` | X11 GTK |
| C 1387/1555 `#ifdef HAVE_XINPUT2` (C 1389-1553, 1557-1573) | `all (…, feature = "have_xinput2")` | XInput2 |
| C 1489 `#ifdef HAVE_XINPUT2_4` + C 1493 `#if GTK_CHECK_VERSION (3, 18, 0)` (C 1491-1550) | `all (…, have_xinput2, have_xinput2_4)` + `feature = "gtk_check_version_ge_3_18"` (**not registered** - `## Owed` 9) | XI 2.4 with GTK >= 3.18 |
| C 1782 `#if WEBKIT_CHECK_VERSION (2, 34, 0)` (C 1786-1789) | `feature = "webkit_check_version_ge_2_34"` (`## Owed` 1) | WebKit >= 2.34 |
| C 2135/2153/2172/2258/2272 `#ifdef HAVE_X_WINDOWS` (C 2137-2145, 2155-2166, 2173-2176, 2259-2267, 2274-2279) | `feature = "have_x_windows"` (`all (use_gtk, …)` where C's item is the file's own) | X11 GTK |
| C 2180 `#ifndef HAVE_PGTK` … C 2212 `#else` … C 2242 | `all (use_gtk, not (have_pgtk))` / `all (use_gtk, have_pgtk)` | X11 GTK / PGTK |
| the two GTK3-only names this slice reads (`GTK_ORIENTATION_VERTICAL` C 2691, `gtk_box_new` C 2684) | the mirror `include/gtk.rs`'s `feature = "have_gtk3"` gate, so the arm flip must set it too (P-86 ⑥) | GTK3 (`HAVE_GTK3 undef` in the frozen table, 1385) |
| `HAVE_X_WINDOWS` and `HAVE_PGTK` are mutually exclusive (`configure.ac`'s GTK branches) | the two arms **cannot** be flipped together: the combination aborts on a *mirror* `E0428 Emacs_Cursor` (`## Evidence` ⑧) | - |

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
| 1181-2107 | **slice 2, the `HAVE_X_WINDOWS` arm** - C 1181's blank and C 1182's `#ifdef`: `xv_drag_begin_cb` (C 1184-1200), `xwidget_button_1` (C 1202-1326), `xwidget_button` (C 1328-1385), the XI2 trio `xwidget_motion_notify`/`xwidget_scroll`/`xwidget_pinch` (C 1389-1550; C 1387/1489's `#ifdef`s, C 1493's `GTK_CHECK_VERSION (3, 18, 0)`), `xi_translate_notify_detail` (C 1557-1573), `window_coords_from_toplevel` (C 1578-1607), `xw_find_common_ancestor` (C 1609-1648), `xw_notify_virtual_upwards_until`/`_downwards_until` (C 1650-1736), `xw_update_cursor_for_view` (C 1738-1759), `xw_last_crossing_cursor_cb` (C 1761-1767), `xw_maybe_synthesize_crossing` (C 1769-1943, C 1782's version arm) and `xwidget_motion_or_crossing` (C 1945-2105) plus C 2107's `#endif /* HAVE_X_WINDOWS */` | `all (feature = "use_gtk", feature = "have_x_windows")` (+ `have_xinput2`, `have_xinput2_4`, `gtk_check_version_ge_3_18`, `webkit_check_version_ge_2_34`) |
| 2108-2281 | `synthesize_focus_in_event` (C 2110-2133), `xwidget_view_from_window` (C 2137-2145), `xwidget_show_view`/`xwidget_hide_view` (C 2150-2178, C 2153/2172's `#ifdef`/`#else`), `xv_do_draw`/`xwidget_view_draw_cb` (C 2182-2242, C 2180's `#ifndef HAVE_PGTK`), `offscreen_damage_event` (C 2247-2270, C 2258's arm), `xwidget_expose` (C 2274-2279) and C 2281's `#endif /* USE_GTK */` | `feature = "use_gtk"` (+ the `have_x_windows`/`have_pgtk` arms) |
| 2282-2331 | C 2283's blank and the three `store_xwidget_*_event` helpers (`store_xwidget_event_string` C 2284-2296, `store_xwidget_download_callback_event` C 2298-2316, `store_xwidget_js_callback_event` C 2318-2331) | file gate |
| 2332-2716 | C 2333's `#ifdef USE_GTK`: `store_xwidget_display_event` (C 2335-2349), `webkit_ready_to_show` (C 2351-2379), `webkit_create_cb_1`/`webkit_create_cb` (C 2381-2425), `webkit_view_load_changed_cb` (C 2427-2451), `webkit_js_to_lisp` (C 2453-2518), `webkit_javascript_finished_cb` (C 2520-2559), `webkit_download_cb` (C 2561-2574), `webkit_decide_policy_cb` (C 2576-2645), `webkit_script_dialog_cb` (C 2647-2714) and C 2716's `#endif` | `feature = "use_gtk"` |
| 2717-2790 | `xwidget_init_view` (C 2720-2790), with C 2716's `#endif /* USE_GTK */` and C 2718-2719's comment | file gate - **transcribed** (slice 3, status 3.671) |
| 2791-3041 | **the remainder between the slices**: C 2793's `x_draw_xwidget_glyph_string` (C 2792-3041) and C 3042's blank | file gate - **owed** |
| 3042-3116 | the `CHECK_WEBKIT_WIDGET` (C 3042-3044) and `WEBKIT_FN_INIT` (C 3047-3050) macros, and the three accessors `xwidget-webkit-uri` (C 3052-3068), `xwidget-webkit-title` (C 3070-3085), `xwidget-webkit-estimated-load-progress` (C 3087-3115) | file gate - **transcribed** (slice 3, status 3.671) |
| 3117-3307 | the three WebKit subrs `xwidget-webkit-goto-uri` (C 3117-3133), `xwidget-webkit-goto-history` (C 3135-3169), `xwidget-webkit-zoom` (C 3171-3191), `save_script_callback` (C 3193-3215, `static`, in C 3193's `#ifdef USE_GTK`), `xwidget-webkit-execute-script` (C 3217-3250) and `xwidget-resize` (C 3252-3303) with C 3304-3307's blank lines | file gate - **transcribed** (slice 4, status 3.672) |
| 3308-3547 | `xwidget-size-request` (C 3308-3324), `xwidgetp` (C 3326-3333), `xwidget-view-p` (C 3335-3342), `xwidget-info` (C 3344-3355), `xwidget-view-info` (C 3357-3369), `xwidget-view-model` (C 3371-3379), `xwidget-view-window` (C 3381-3389), `delete-xwidget-view` (C 3392-3450), `xwidget-view-lookup` (C 3452-3476), `xwidget-plist` (C 3478-3486), `xwidget-buffer` (C 3488-3496), `set-xwidget-buffer` (C 3498-3509), `set-xwidget-plist` (C 3511-3523), `set-xwidget-query-on-exit-flag` (C 3525-3537) and `xwidget-query-on-exit-flag` (C 3539-3547) | file gate - **transcribed** (slice 5, status 3.673) |
| 3548-3770 | `xwidget-webkit-search` (C 3549-3607), `xwidget-webkit-next-result` (C 3609-3640), `xwidget-webkit-previous-result` (C 3642-3673), `xwidget-webkit-finish-search` (C 3675-3712), `kill-xwidget` (C 3714-3731) and `xwidget-webkit-load-html` (C 3734-3769, under C 3733's `#ifdef USE_GTK`, which is this file's gate) | file gate - **transcribed** (slice 6, status 3.674) |
| 3771-4027 | `xwidget-webkit-back-forward-list` (C 3771-3865, with C 3864's `#endif` closing C 3733's `#ifdef USE_GTK`), `xwidget-webkit-set-cookie-storage-file` (C 3866-3898), `xwidget-webkit-stop-loading` (C 3899-3925) and **`syms_of_xwidget`** (C 3928-4026, which registers every subr this unit ports) | file gate - **transcribed** (slice 7, status 3.675) |
| 4028-4304 | C 4028-4035's comment on `valid_xwidget_spec_p` and the store/kill helpers: `valid_xwidget_spec_p` (C 4036-4040), `xwidget_spec_value` (C 4044-4059), `xwidget_view_delete_all_in_window` (C 4062-4078), `xwidget_view_lookup` (C 4080-4090), `lookup_xwidget` (C 4092-4105), `xwidget_start_redisplay` (C 4108-4117), `xwidget_touch` (C 4121-4125), `xwidget_touched` (C 4127-4131), `xwidget_end_redisplay` (C 4134-4209), `lower_frame_xwidget_views` (C 4212-4224, C 4211's `#ifdef HAVE_X_WINDOWS`), `kill_frame_xwidget_views` (C 4228-4243, C 4227's `#ifndef NS_IMPL_COCOA`), `kill_xwidget` (C 4246-4285) and `kill_buffer_xwidgets` (C 4288-4304) | file gate; `have_x_windows` on C 4212-4224; `not ns_impl_cocoa` on C 4228-4243 - **transcribed** (slice 8, status 3.676) |

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
> **The file gate is `all (feature = "use_gtk", feature = "have_gtk3")`** (corrected 2026-10-05, status 3.596: it used
> to be `use_gtk` alone, which is C 37's *content* guard - `configure.ac` 4424-4434 build `XWIDGETS_OBJ="xwidget.o"`
> only under `USE_GTK_TOOLKIT = "GTK3"`, the same block as `AC_DEFINE([HAVE_GTK3])` at 3756-3759, while `USE_GTK`
> (3826) covers GTK2 as well).  C 37's `#ifdef USE_GTK` is
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
| the file itself (`configure.ac` 4422-4474: `XWIDGETS_OBJ="xwidget.o"` inside `with_xwidgets != no` **and** `USE_GTK_TOOLKIT = "GTK3"` at 4434, a second arm at 4463 for Cocoa, `AC_DEFINE ([HAVE_XWIDGETS], [1])` at 4474) | `all (feature = "use_gtk", feature = "have_gtk3")` on every item *and on the include graph* (P-86 ⑧; corrected in 3.596 - `USE_GTK` alone also covers GTK2, where C builds no `xwidget.o`) | a GTK**3** xwidgets configuration; the Cocoa arm is the two `ns_impl_cocoa` items |
| C 37 `#ifdef USE_GTK` … C 50 `#elif defined NS_IMPL_COCOA` … C 52 (the bottom-end include chain) | `all (feature = "use_gtk", feature = "have_gtk3")` / `all (feature = "use_gtk", feature = "ns_impl_cocoa")` | GTK3 / Cocoa |
| C 41 `#ifndef HAVE_PGTK` (C 42-43 `<cairo-xlib.h>` + `<X11/Xlib.h>`, C 45 `#else`'s `<gtk/gtk.h>`) | `not (feature = "have_pgtk")` / `feature = "have_pgtk"` | X11 GTK / PGTK |
| C 47 `#ifdef HAVE_XINPUT2` (C 48 `<X11/extensions/XInput2.h>`) | `feature = "have_xinput2"` | XInput2 |
| C 62 `#ifdef HAVE_X_WINDOWS` (C 63 `x_window_to_xwv_map`) and C 64 `#if WEBKIT_CHECK_VERSION (2, 34, 0)` (C 65 `dummy_tooltip_string`) | `feature = "have_x_windows"`, then `feature = "webkit_check_version_ge_2_34"` (`## Owed` 1) | X11 GTK with WebKit ≥ 2.34 |
| C 98 `#ifdef USE_GTK` (C 118-125 `widget_search_data`, C 127 `find_widget`) | `all (feature = "use_gtk", feature = "have_gtk3")` | GTK3 |
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
| C 1182 `#ifdef HAVE_X_WINDOWS` (C 1184-2105) | `all (feature = "use_gtk", feature = "have_x_windows")` | X11 GTK |
| C 1387/1555 `#ifdef HAVE_XINPUT2` (C 1389-1553, 1557-1573) | `all (…, feature = "have_xinput2")` | XInput2 |
| C 1489 `#ifdef HAVE_XINPUT2_4` + C 1493 `#if GTK_CHECK_VERSION (3, 18, 0)` (C 1491-1550) | `all (…, have_xinput2, have_xinput2_4)` + `feature = "gtk_check_version_ge_3_18"` (**not registered** - `## Owed` 9) | XI 2.4 with GTK >= 3.18 |
| C 1782 `#if WEBKIT_CHECK_VERSION (2, 34, 0)` (C 1786-1789) | `feature = "webkit_check_version_ge_2_34"` (`## Owed` 1) | WebKit >= 2.34 |
| C 2135/2153/2172/2258/2272 `#ifdef HAVE_X_WINDOWS` (C 2137-2145, 2155-2166, 2173-2176, 2259-2267, 2274-2279) | `feature = "have_x_windows"` (`all (use_gtk, …)` where C's item is the file's own) | X11 GTK |
| C 2180 `#ifndef HAVE_PGTK` … C 2212 `#else` … C 2242 | `all (use_gtk, not (have_pgtk))` / `all (use_gtk, have_pgtk)` | X11 GTK / PGTK |
| the two GTK3-only names this slice reads (`GTK_ORIENTATION_VERTICAL` C 2691, `gtk_box_new` C 2684) | the mirror `include/gtk.rs`'s `feature = "have_gtk3"` gate, so the arm flip must set it too (P-86 ⑥) | GTK3 (`HAVE_GTK3 undef` in the frozen table, 1385) |
| `HAVE_X_WINDOWS` and `HAVE_PGTK` are mutually exclusive (`configure.ac`'s GTK branches) | the two arms **cannot** be flipped together: the combination aborts on a *mirror* `E0428 Emacs_Cursor` (`## Evidence` ⑧) | - |

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
| 1181-2107 | **slice 2, the `HAVE_X_WINDOWS` arm** - C 1181's blank and C 1182's `#ifdef`: `xv_drag_begin_cb` (C 1184-1200), `xwidget_button_1` (C 1202-1326), `xwidget_button` (C 1328-1385), the XI2 trio `xwidget_motion_notify`/`xwidget_scroll`/`xwidget_pinch` (C 1389-1550; C 1387/1489's `#ifdef`s, C 1493's `GTK_CHECK_VERSION (3, 18, 0)`), `xi_translate_notify_detail` (C 1557-1573), `window_coords_from_toplevel` (C 1578-1607), `xw_find_common_ancestor` (C 1609-1648), `xw_notify_virtual_upwards_until`/`_downwards_until` (C 1650-1736), `xw_update_cursor_for_view` (C 1738-1759), `xw_last_crossing_cursor_cb` (C 1761-1767), `xw_maybe_synthesize_crossing` (C 1769-1943, C 1782's version arm) and `xwidget_motion_or_crossing` (C 1945-2105) plus C 2107's `#endif /* HAVE_X_WINDOWS */` | `all (feature = "use_gtk", feature = "have_x_windows")` (+ `have_xinput2`, `have_xinput2_4`, `gtk_check_version_ge_3_18`, `webkit_check_version_ge_2_34`) |
| 2108-2281 | `synthesize_focus_in_event` (C 2110-2133), `xwidget_view_from_window` (C 2137-2145), `xwidget_show_view`/`xwidget_hide_view` (C 2150-2178, C 2153/2172's `#ifdef`/`#else`), `xv_do_draw`/`xwidget_view_draw_cb` (C 2182-2242, C 2180's `#ifndef HAVE_PGTK`), `offscreen_damage_event` (C 2247-2270, C 2258's arm), `xwidget_expose` (C 2274-2279) and C 2281's `#endif /* USE_GTK */` | `feature = "use_gtk"` (+ the `have_x_windows`/`have_pgtk` arms) |
| 2282-2331 | C 2283's blank and the three `store_xwidget_*_event` helpers (`store_xwidget_event_string` C 2284-2296, `store_xwidget_download_callback_event` C 2298-2316, `store_xwidget_js_callback_event` C 2318-2331) | file gate |
| 2332-2716 | C 2333's `#ifdef USE_GTK`: `store_xwidget_display_event` (C 2335-2349), `webkit_ready_to_show` (C 2351-2379), `webkit_create_cb_1`/`webkit_create_cb` (C 2381-2425), `webkit_view_load_changed_cb` (C 2427-2451), `webkit_js_to_lisp` (C 2453-2518), `webkit_javascript_finished_cb` (C 2520-2559), `webkit_download_cb` (C 2561-2574), `webkit_decide_policy_cb` (C 2576-2645), `webkit_script_dialog_cb` (C 2647-2714) and C 2716's `#endif` | `feature = "use_gtk"` |
| 2717-2790 | `xwidget_init_view` (C 2720-2790), with C 2716's `#endif /* USE_GTK */` and C 2718-2719's comment | file gate - **transcribed** (slice 3, status 3.671) |
| 2791-3041 | **the remainder between the slices**: C 2793's `x_draw_xwidget_glyph_string` (C 2792-3041) and C 3042's blank | file gate - **owed** |
| 3042-3116 | the `CHECK_WEBKIT_WIDGET` (C 3042-3044) and `WEBKIT_FN_INIT` (C 3047-3050) macros, and the three accessors `xwidget-webkit-uri` (C 3052-3068), `xwidget-webkit-title` (C 3070-3085), `xwidget-webkit-estimated-load-progress` (C 3087-3115) | file gate - **transcribed** (slice 3, status 3.671) |
| 3117-3307 | the three WebKit subrs `xwidget-webkit-goto-uri` (C 3117-3133), `xwidget-webkit-goto-history` (C 3135-3169), `xwidget-webkit-zoom` (C 3171-3191), `save_script_callback` (C 3193-3215, `static`, in C 3193's `#ifdef USE_GTK`), `xwidget-webkit-execute-script` (C 3217-3250) and `xwidget-resize` (C 3252-3303) with C 3304-3307's blank lines | file gate - **transcribed** (slice 4, status 3.672) |
| 3308-3547 | `xwidget-size-request` (C 3308-3324), `xwidgetp` (C 3326-3333), `xwidget-view-p` (C 3335-3342), `xwidget-info` (C 3344-3355), `xwidget-view-info` (C 3357-3369), `xwidget-view-model` (C 3371-3379), `xwidget-view-window` (C 3381-3389), `delete-xwidget-view` (C 3392-3450), `xwidget-view-lookup` (C 3452-3476), `xwidget-plist` (C 3478-3486), `xwidget-buffer` (C 3488-3496), `set-xwidget-buffer` (C 3498-3509), `set-xwidget-plist` (C 3511-3523), `set-xwidget-query-on-exit-flag` (C 3525-3537) and `xwidget-query-on-exit-flag` (C 3539-3547) | file gate - **transcribed** (slice 5, status 3.673) |
| 3548-3770 | `xwidget-webkit-search` (C 3549-3607), `xwidget-webkit-next-result` (C 3609-3640), `xwidget-webkit-previous-result` (C 3642-3673), `xwidget-webkit-finish-search` (C 3675-3712), `kill-xwidget` (C 3714-3731) and `xwidget-webkit-load-html` (C 3734-3769, under C 3733's `#ifdef USE_GTK`, which is this file's gate) | file gate - **transcribed** (slice 6, status 3.674) |
| 3771-4027 | `xwidget-webkit-back-forward-list` (C 3771-3865, with C 3864's `#endif` closing C 3733's `#ifdef USE_GTK`), `xwidget-webkit-set-cookie-storage-file` (C 3866-3898), `xwidget-webkit-stop-loading` (C 3899-3925) and **`syms_of_xwidget`** (C 3928-4026, which registers every subr this unit ports) | file gate - **transcribed** (slice 7, status 3.675) |
| 4028-4304 | C 4028-4035's comment on `valid_xwidget_spec_p` and the store/kill helpers: `valid_xwidget_spec_p` (C 4036-4040), `xwidget_spec_value` (C 4044-4059), `xwidget_view_delete_all_in_window` (C 4062-4078), `xwidget_view_lookup` (C 4080-4090), `lookup_xwidget` (C 4092-4105), `xwidget_start_redisplay` (C 4108-4117), `xwidget_touch` (C 4121-4125), `xwidget_touched` (C 4127-4131), `xwidget_end_redisplay` (C 4134-4209), `lower_frame_xwidget_views` (C 4212-4224, C 4211's `#ifdef HAVE_X_WINDOWS`), `kill_frame_xwidget_views` (C 4228-4243, C 4227's `#ifndef NS_IMPL_COCOA`), `kill_xwidget` (C 4246-4285) and `kill_buffer_xwidgets` (C 4288-4304) | file gate; `have_x_windows` on C 4212-4224; `not ns_impl_cocoa` on C 4228-4243 - **transcribed** (slice 8, status 3.676) |

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


**⑥ The X11 records this slice declares are *measured*, unlike the GDK/GTK/WebKit ones.**
`clang -o /tmp/xwidget-lane/xprobe /tmp/xwidget-lane/xprobe.c -I/opt/homebrew/include` over
`<X11/Xlib.h>` printed `XAnyEvent 40 type@0 serial@8 send_event@16 display@24 window@32`,
`XMotionEvent 96 x@64 y@68 x_root@72 y_root@76 state@80 time@56`,
`XCrossingEvent 104 x@64 y@68 x_root@72 y_root@76 mode@80 detail@84 focus@92 state@96 time@56`,
`XGenericEventCookie 56 evtype@36 data@48` and `XEvent 192`; those are the `const _` assertions the
unit writes for the four member records, and they agree with `include/xlib.rs`'s measured `XEvent`.
The two XInput2 records (`XIEnterEvent`, `XIGesturePinchEvent`) have **no header on this machine**
and are reconstructed from C's use sites (C 1497-1507, C 1993-2040) - **UNVERIFIED** (`## Owed` 10).

**⑦ The frozen reading - `build-tu` (the P-86 ⑧ gate check), run twice.**
`cd femacs && CARGO_TARGET_DIR=/tmp/xwidget-lane/target cargo run -q -p xtask -- build-tu src/xwidget.c`
-> `built src/xwidget.rs -> ../build/femacs/xwidget.o`, **exit 0**, and `build/femacs/xwidget.o` is
**336 bytes** - an empty object, which is the expected reading and not a proof of the new text:
every item sits behind `use_gtk`, so under the frozen feature set rustc compiles only
`include/config.rs` and the `#![allow]` attribute.  Both runs (before and after the fix passes)
returned the same 336 bytes, and each run also materialises `build/femacs/units/xwidget/_root.rs`
(the unit's text plus every mirror it reaches), which is what reading ⑧ compiles.

**⑧ The arm readings (direct `rustc` on the materialised root; `text-only`, P-86 ③④⑤).**
There is still no GDK/GTK/WebKit tree on this machine, so no arm can be *compiled against GTK*;
each reading is rustc's name-resolution/type errors under a hand-set `--cfg` set, counted **by
file** (P-86 ⑦):

```
$ rustc --edition 2024 --crate-type=lib --emit=metadata -C panic=abort \
        --cfg 'feature="use_gtk"' --cfg 'feature="have_xwidgets"' \
        --cfg 'feature="have_x_windows"' --cfg 'feature="have_window_system"' \
        --cfg 'feature="have_gtk3"' --cfg 'feature="have_xinput2"' \
        --cfg 'feature="have_xinput2_4"' --cfg 'feature="gtk_check_version_ge_3_18"' \
        --cfg 'feature="webkit_check_version_ge_2_34"' \
        --cfg 'feature="webkit_check_version_ge_2_26"' \
        --cfg 'feature="webkit_check_version_ge_2_16"' \
        build/femacs/units/xwidget/_root.rs                        # ARM A11
$ rustc … --cfg 'feature="use_gtk"' --cfg 'feature="have_xwidgets"' \
        --cfg 'feature="have_pgtk"' --cfg 'feature="have_window_system"' \
        --cfg 'feature="have_gtk3"' build/femacs/units/xwidget/_root.rs   # ARM PG
```

- **ARM A11: 85 errors anchored in this unit**, and **all of them are registered gaps**: 84 `E0609`
  (the mirror's `GdkEvent`/`GdkEventButton`/`GdkEventScroll`/`GdkEventMotion`/`GdkEventCrossing`
  records carry only the members slice 1 read - the field-by-field list is in `## Owed` 8 - plus
  `xwidget.kill_without_query`, the P-80 bitfield slice 1 records) and 1 `E0425` (`kill_xwidget`,
  C 3727's definition, the unported remainder).  **No error is anchored in this slice's own text.**
  4 more are anchored in mirrors: `E0425 XIEventMask` (`include/xterm.rs` 919), `E0133`
  (`include/xwidget.rs` 204, `XWIDGET_LIVE_P`'s pointer deref), `E0080` (`include/systime.rs`'s
  `Time` assertion fires because `feature = "have_x_windows"` is set while the frozen
  `include/config.rs`'s `HAVE_X_WINDOWS` is false) and `E0658` (`VaList` in `include/lisp.rs`, an
  artefact of plain `rustc` without the probe's bootstrap flags).
- **ARM PG: 12 errors in this unit**, of which only **2 are this slice's**
  (`GdkEvent.focus_change` twice, `synthesize_focus_in_event`, C 2127-2128, mirror gap `## Owed` 8)
  and 10 are slice 1's latent defects that only this arm can see: `cursor_for_hit`/`define_cursors`
  (C 1065-1119) read `(*FRAME_OUTPUT_DATA! (frame) as *mut pgtk_output).nontext_cursor` and three
  siblings, measured as 5 `E0609` + 4 `E0605` (`pgtk_output` is a record in `include/pgtkterm.rs`,
  not a pointer) - `## Owed` 13.  Two mirror errors (`E0658` as above, `E0133` in the `xwidget`
  mirror).
- **The pre-fix reading (this round's own defects).**  The first `xtask probe` arm
  (`use_gtk,have_xwidgets,have_x_windows,have_window_system`) reported **70 errors**: 50 `E0609` +
  11 `E0308` + 3 `E0425` + 1 `E0606` + 1 `E0384` + 1 `E0133` + 1 `E0080`.  Of those, **13 were
  this round's own defects and were fixed**: eleven implicit `Time` (`c_ulong`) -> `guint32`
  narrowings onto the GDK records' `time` members, the missing parenthesisation of the
  `(* (FRAME_OUTPUT_DATA! (…) as *mut x_output)).nontext_cursor` cast (C 1756), and the `mut window`
  parameter of `window_coords_from_toplevel` (C 1585's loop reassigns it).  Two more were the
  mirror's `have_gtk3` gate on `GTK_ORIENTATION_VERTICAL`/`gtk_box_new` (which a real xwidgets build
  has set, and which ARM A11 sets).  A further pass moved the two `gdk_x11_cursor_*` declarations
  behind `have_x_windows` (their `Cursor`/`Display` return types) after ARM PG showed them
  unresolved there.
- **A combination reading is *vacuous* and is recorded as such** (P-86 ⑦): flipping
  `have_x_windows` **together with** `have_pgtk` (through `xtask probe`) aborts at
  `E0428 the name Emacs_Cursor is defined multiple times` (`include/pgtkgui.rs`'s
  `Emacs_Cursor = *mut c_void` against `include/dispextern.rs`'s `= Cursor`), so the run reports 7
  errors and never reaches this unit's text.  The two configurations are mutually exclusive in
  `configure.ac`; the ARM A11/ARM PG readings above are the ones this ledger counts.

**⑨ `extract-abi --check`: 84 -> 81 `missing` rows.**  Before: `- line count drifted 53.5%:
C 4304 vs Rust 2000`, 84 `- missing` rows (35 `defsubr`, 34 `subr`, 13 `function`, 2 `macro`), 0
`does not mention` rows.  After: **81 `- missing` rows** - the three that left are
`store_xwidget_event_string`, `store_xwidget_download_callback_event` and
`store_xwidget_js_callback_event`, the only names this slice's C range defines that the checklist
carried (the `xwidget_*`/`webkit_*` callbacks it adds are declared in `include/xwidget.rs`'s
`xwidget_decls`, so the checklist never listed them) - **0 `does not mention` rows**, and the
`line count drifted` row is **gone** (the port is 4,492 lines against C's 4,304, 4.4%, under the 5%
rule; see `## Owed` 12).

**⑩ The other gates (repo root, final revision).**  `python3 tools/check-cond-eval.py
src/xwidget.c` -> `121 conditions, 0 unknown, 0 disagreements [VERIFIED]`, exit 0.  `python3
tools/check-dead-file-coverage.py` -> `145 ledger(s) bound, 0 failure(s)`, with `docs/ports/
xwidget.md: 26 range row(s) cover src/xwidget.c (4304 lines; 0 blank line(s) between blocks need no
row)` and the same 26 covering `src/xwidget.h (243 lines)`.  `python3 tools/check-ledger-ticks.py`
-> `238 ledger(s) bound, 0 failure(s)`.  `python3 tools/check-citations.py` -> `676 file(s)
scanned, 0 drift, 0 range, 1881 external, 36 allowed, 0 stale` (this unit's citations are in the
`external` class, as slice 1's were).  `python3 tools/check-declarations.py` -> exit 0, no xwidget
row.  `python3 tools/check-mirror-dupes.py` -> `17 duplicate declaration(s), 17 ratchet row(s), 0
failure(s)` - none of them this unit's (the in-unit surface does not repeat the mirror).

**⑩ Slice 3 (C 2717-2790 and C 3042-3116): the readings of the round that landed it (2026-10-06,
status 3.671).**

a) `python3 tools/extract-abi.py --check emacs/src/xwidget.c femacs/src/xwidget.rs`: `84 -> 81
missing` rows was ⑨'s reading for slice 2; this slice retires **six** more - `xwidget_init_view`,
the two macros `CHECK_WEBKIT_WIDGET` / `WEBKIT_FN_INIT`, and the three subrs `Fxwidget_webkit_uri`
/ `Fxwidget_webkit_title` / `Fxwidget_webkit_estimated_load_progress` (with their `defsubr` +
`S*` rows).  The row that stays is the line count: `line count drifted 10.4%: C 4304 vs Rust
4753` - measured *after* this slice added 242 lines and **derived from the finished prefix's
density** (the same caveat ⑨'s row carries; 1,588 C lines are still owed).

b) **The frozen reading.**  `build-tu src/xwidget.c` -> `built src/xwidget.rs ->
../build/femacs/xwidget.o`, **exit 0**, `build/femacs/xwidget.o` is **336 bytes** - the empty
object of the P-86 class (the frozen configuration compiles none of this file), not a proof of the
new text.

c) **The three arm flips, and the baseline they are read against** (P-86 ⑦'s by-file counting).
The arm sets are Evidence ⑧'s, with the file gate's `have_gtk3` added (the 3.596 correction:
`all (feature = "use_gtk", feature = "have_gtk3")`).  The same three runs were made on **HEAD's**
`femacs/src/xwidget.rs` (and HEAD's `include/gtk.rs`) so the delta is the slice's, not the arm's:

| arm (on top of `use_gtk` + `have_gtk3` + `have_xwidgets`) | own-text rows, HEAD | own-text rows, this slice | added | include-graph rows |
| --- | --- | --- | --- | --- |
| A: `have_x_windows` + `have_window_system` | 50 | **50** | **0** | 2 (`include/systime.rs`, `include/xwidget.rs`) |
| B: `have_pgtk` + `have_window_system` | 12 | **12** | **0** | 1 (`include/xwidget.rs`) |
| C: (no window system - an arm set no configuration has) | 22 | 23 | **1** | 7 |

The one added row in C is `cannot find type glyph_string`, i.e. the arm cannot name
`xwidget_init_view`'s `struct glyph_string *s` because no window system is in scope; C's own body
needs `s->f`/`s->w` and a GTK build is a window-system build, so the row is the arm's, not the
text's (Deviation 8).  Every other own-text row in A/B/C is the pre-existing owed surface
(Evidence ⑧'s `E0609` list) - unchanged, none added.

d) **The four fix iterations behind c, all in this slice's own text.**  Run 1: `cannot find type
gdouble` (the WebKit declaration spelled GLib's typedef; the unit's own surface spells C's
`double` as `c_double`), `E0308` x2 (`xwidget_view.frame` is the mirror's `*mut c_void` and
`just_resized` its `c_int` for C's `bool`, so C's `false` is 0), `g_signal_connect` as a function
(this unit's own macro is called with `!`).  Run 2: `E0605: non-primitive cast Option<_> as u64`
(C 2752's `None` is X11/X.h 115's `#define None 0L`, which this unit imports as
`use x11_constants::None;` - line 3445's own shape); `cannot find type glyph_string` in the
`nsxwidget_init_view` declaration (its `struct glyph_string *` is not nameable in every arm, so
the declaration takes `*mut c_void`, P-72 rule 3).  Run 3 is the table above.

e) **One mirror edit, and P-87's two legs.**  `gtk_drawing_area_new` (C 2760's call) was added to
`femacs/src/include/gtk.rs` (the P-90 ② home of the GTK surface) rather than declared at the use
site; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` and
`cargo run -q -p xtask -- layout-check` -> `layout-check passed (see build/femacs/layout_asserts.rs)`
(both were run because the edit is a mirror edit; P-87 ①).


**⑪ Slice 4 (C 3117-3307): the readings of the round that landed it (2026-10-06, status 3.672).**

a) `python3 tools/extract-abi.py --check emacs/src/xwidget.c femacs/src/xwidget.rs`: **76 -> 71**
rows.  The five retired are `Fxwidget_webkit_goto_uri`, `Fxwidget_webkit_goto_history`,
`Fxwidget_webkit_zoom`, `Fxwidget_webkit_execute_script` and `save_script_callback` (plus
`Fxwidget_resize`'s `subr` row, which slice 3's "second round" had not reached).  **The four
`defsubr (&raw const Sxwidget_webkit_*)` rows and `Fxwidget_resize`'s own stay**, because the
`defsubr` calls live in `syms_of_xwidget` (C 3928-4035), which is still owed - a `DEFUN` and its
registration are two rows and land in two slices.  The drift row is now
`line count drifted 18.8%: C 4304 vs Rust 5114` (derived from the finished prefix's density, as
⑨/⑩ say).

b) **The frozen reading.**  `build-tu src/xwidget.c` -> `built ... -> ../build/femacs/xwidget.o`,
exit 0, **336 bytes** - the empty object of the P-86 class.

c) **The three arm flips, against HEAD's own file** (HEAD is exactly "before slice 4"; P-86 ⑦
counts by file):

| arm (`use_gtk` + `have_gtk3` + `have_xwidgets` + …) | own-text rows, HEAD | this slice | added |
| --- | --- | --- | --- |
| A: `have_x_windows` + `have_window_system` | 50 | **50** | **0** |
| B: `have_pgtk` + `have_window_system` | 12 | **12** | **0** |
| C: (no window system) | 23 | **23** | **0** |

All three arms read the same own-text counts as before the slice, i.e. this slice's text
type-checks in every one of them and adds no row anywhere (ARM C's 23 is slice 3's
`glyph_string` row plus the pre-existing `E0609` list).

d) **The three fix iterations behind c (nine edits, all this slice's own text).**  Run 1: `no rules
expected uri / script` - the `DEFUN!` macro's argument pattern is `$arg:ident : $ty:ty`, so C's
assignable parameters cannot be spelled `mut` there; both are rebound in the body instead (C 3125
and C 3230 assign them; the `window.rs` 4267 shape).  Run 2: `GAsyncReadyCallback` (declared beside
the unit's own `GAsyncResult`, the `include/gtk.rs` 93-96 `GCallback`/`GClosureNotify` shape),
`XFIXNUM`'s `EMACS_INT` into GLib's `gint` (cast), two chained assignments C writes
(`xw->script_callbacks = cbs = …` - Rust's `=` is not an expression), `xlispstrdup`'s `char *` into
`make_mint_ptr`'s `Lisp_Object` (cast through `*mut c_void`), `check_integer_range`'s bounds are
`intmax_t` (its own lisp.h 3773 declaration, not `EMACS_INT`) and `redisplay` (declared at the use
site: xdisp.c 17073, the `keyboard.rs` 680 shape).  Run 3: `webkit_javascript_finished_cb` is
`unsafe fn` in the port and C 3244 passes it as a `GAsyncReadyCallback`, whose type is
`unsafe extern "C" fn` - so the definition becomes `extern "C"` (the
`init_alloc_once_for_pdumper` correction, `femacs/src/alloc.rs` 7994).  Run 4 is the table above.

e) **No mirror was edited this round**, so P-87 ①'s `check-all-units.sh` / `layout-check` legs do
not apply: the WebKit record (`WebKitBackForwardList`, `WebKitBackForwardListItem`), the WebKit
functions, the GLib callback type and the four use-site declarations (`larger_vector` lisp.h 4245,
`xlispstrdup` lisp.h 5553, `FUNCTIONP` lisp.h 4799, `redisplay` xdisp.c 17073) all live in this
unit, which is where the file's third-party surface already is (its WebKit block, 553-740).
`check-dead-file-coverage.py` re-read after the table change: `145 ledger(s), 0 failure(s)`.


**⑫ Slice 5 (C 3308-3547): the readings of the round that landed it (2026-10-06, status 3.673).**

a) `python3 tools/extract-abi.py --check emacs/src/xwidget.c femacs/src/xwidget.rs`: **71 -> 56**
rows.  The fifteen retired are the fifteen subrs this slice writes
(`Fxwidget_size_request`, `Fxwidgetp`, `Fxwidget_view_p`, `Fxwidget_info`, `Fxwidget_view_info`,
`Fxwidget_view_model`, `Fxwidget_view_window`, `Fdelete_xwidget_view`, `Fxwidget_view_lookup`,
`Fxwidget_plist`, `Fxwidget_buffer`, `Fset_xwidget_buffer`, `Fset_xwidget_plist`,
`Fset_xwidget_query_on_exit_flag`, `Fxwidget_query_on_exit_flag`); their `defsubr (&raw const S… )`
rows stay until `syms_of_xwidget` lands (⑩ a).  Drift row: `25.7%: C 4304 vs Rust 5409`.

b) **The frozen reading.**  `build-tu src/xwidget.c` -> built, `build/femacs/xwidget.o` **336
bytes** - the empty object of the P-86 class.

c) **The three arm flips against HEAD's own files** (HEAD is exactly "before slice 5"):

| arm (`use_gtk` + `have_gtk3` + `have_xwidgets` + …) | own-text rows, HEAD | this slice | added | what the added rows are |
| --- | --- | --- | --- | --- |
| A: `have_x_windows` + `have_window_system` | 50 | 52 | **+2** | `no field kill_without_query` x2 (the registered class below) |
| B: `have_pgtk` + `have_window_system` | 12 | 14 | **+2** | idem |
| C: (no window system) | 23 | 25 | **+2** | idem |

Every added row is the **one registered mirror-gap class** this file already carries (Owed 9, the
P-80 storage unit for C 99's `bool_bf kill_without_query : 1`): C 3535 and C 3546 are the second and
third sites of it, C 319 the first.  Nothing else in this slice adds a row in any arm.

d) **The fix iterations behind c (four edits).**  Run 1: three `E0308`s - `g_signal_handler_disconnect`'s
first parameter is the port's `*mut c_void` (`include/gtk.rs`), so C 3415/3422/3424's three calls
cast their pointer.  Run 2: ARM C's one extra row (`no field widget on type xwidget_view`) showed
that C 3429-3430's `#else` (USE_GTK *without* HAVE_X_WINDOWS) must be spelled `have_pgtk` in this
tree: it is the arm in which the record's `widget` member exists (`include/xwidget.rs` 145-148 gates
`dpyinfo`/`widget` on `all (use_gtk, have_pgtk)`) - the port follows the record's own reading of the
arm, as Deviations 4 does for the Cocoa arms.  Run 3 is the table above.

e) **One mirror edit, and P-87's two legs.**  `gtk_widget_size_request` (C 3319's call; GTK3's
deprecated alias of `gtk_widget_get_preferred_size`, which `xterm.h` 1434-1445's GTK2 arm redefines)
was added to `femacs/src/include/gtk.rs` beside that twin - the P-90 ② home of the GTK surface.
Readings: `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` and
`cargo run -q -p xtask -- layout-check` -> `layout-check passed (see build/femacs/layout_asserts.rs)`
(both run because the edit is a mirror edit; P-87 ①).  `check-dead-file-coverage.py` re-read after the table
change: `145 ledger(s), 0 failure(s)`.


**⑬ Slice 6 (C 3548-3770): the readings of the round that landed it (2026-10-06, status 3.674).**

a) `python3 tools/extract-abi.py --check emacs/src/xwidget.c femacs/src/xwidget.rs`: **56 -> 51**
rows.  The subr rows this slice retires are `Fxwidget_webkit_search`, `Fxwidget_webkit_next_result`,
`Fxwidget_webkit_previous_result`, `Fxwidget_webkit_finish_search`, `Fkill_xwidget` and
`Fxwidget_webkit_load_html`; their `defsubr (&raw const S… )` rows stay for `syms_of_xwidget` (⑩ a).
**`missing function kill_xwidget` also stays**: this slice calls C 4247's `static` definition, and a
use-site declaration (which is what the port needs for the arm to resolve the name) does not
satisfy the "function" row - only C 4247's body does.  Drift row: `32.3%: C 4304 vs Rust 5694`.

b) **The frozen reading.**  `build-tu src/xwidget.c` -> built, `build/femacs/xwidget.o` **336
bytes** - the empty object of the P-86 class.

c) **The three arm flips against HEAD's own file** (HEAD is exactly "before slice 6"):

| arm (`use_gtk` + `have_gtk3` + `have_xwidgets` + …) | own-text rows, HEAD | this slice | added |
| --- | --- | --- | --- |
| A: `have_x_windows` + `have_window_system` | 52 | **51** | **0** (and one row *removed*) |
| B: `have_pgtk` + `have_window_system` | 14 | **13** | **0** (idem) |
| C: (no window system) | 25 | **24** | **0** (idem) |

Every arm is one row *lower* than before: the pre-existing `cannot find function kill_xwidget`
row is gone, because this slice declares the name at the use site with C's own prototype (C 96)
while C 4247's definition waits.  That is the first slice in this unit to leave an arm reading
better than it found it, and nothing else moved.

d) **Three fix iterations behind c.**  Run 1: thirty `E0425`s - the slice first wrote C's
`#ifdef USE_GTK` locals *inside* an inner `#[cfg]` block, which hides them from the body; this
file's gate **is** `USE_GTK` (the ledger's `## Gate mapping`), so those declarations belong at
function scope with no inner block (Deviation 1).  Run 2: one `E0308` - C 3602 passes its
`WebKitFindOptions opt` to the `guint32 find_options` parameter, so the port casts (`as guint32`).
Run 3 is the table above.

e) **No mirror was edited this round**, so P-87 ①'s legs do not apply: the WebKit find-controller
surface (`WebKitFindController`, `WebKitFindOptions` and its four enumerator placeholders) and the
five WebKit entry points go into this unit's own WebKit block (553-900), and `G_MAXUINT` sits
beside `G_MAXINT32` in the unit's GLib constant section - both are where this file's third-party
surface already lives.  `check-dead-file-coverage.py` re-read after the table change:
`145 ledger(s), 0 failure(s)`.


**⑭ Slice 7 (C 3771-4027): the readings of the round that landed it (2026-10-06, status 3.675).**

a) `python3 tools/extract-abi.py --check emacs/src/xwidget.c femacs/src/xwidget.rs`: **51 -> 14**
rows.  The thirty-seven retired are the three subrs' rows
(`Fxwidget_webkit_back_forward_list`, `Fxwidget_webkit_set_cookie_storage_file`,
`Fxwidget_webkit_stop_loading`), the row for `missing function syms_of_xwidget`, and **every
`defsubr (&raw const S… )` row the file carries** - 25 registrations, which is what the ledger's
Owed 7 predicted `syms_of_xwidget` would do.  What is left is the thirteen `missing function` rows
of C 4037-4304's store/kill helpers plus the drift row, now
`line count drifted 40.4%: C 4304 vs Rust 6044`.

b) **The frozen reading.**  `build-tu src/xwidget.c` -> built, `build/femacs/xwidget.o` **336
bytes** - the empty object of the P-86 class.

c) **The three arm flips against HEAD's own file**, and this one came in clean on the first run:

| arm (`use_gtk` + `have_gtk3` + `have_xwidgets` + …) | own-text rows, HEAD | this slice | added |
| --- | --- | --- | --- |
| A: `have_x_windows` + `have_window_system` | 51 | **51** | **0** |
| B: `have_pgtk` + `have_window_system` | 13 | **13** | **0** |
| C: (no window system) | 24 | **24** | **0** |

The slice applied slice 6's structural lesson up front (C's `#ifdef USE_GTK` locals at function
scope, no inner `#[cfg]` block), which is why the first run is the last one this time.

d) **Two things this slice adds that are not transcription**, both named in the deviations: the
`DEFVAR_BOOL` carrier (`pub static mut xwidget_webkit_disable_javascript: bool`, P-97's type and
P-21's shadowing of the globals mirror's `extern`) and the `webkit_check_version_ge_2_34` arm for
C 4019's `#if WEBKIT_CHECK_VERSION (2, 34, 0)` (the derivation `## Owed` 1 records; C 4019 is that
feature's fourth site).

e) **No mirror was edited this round**, so P-87 ①'s legs do not apply: the cookie-manager surface
(`WebKitCookieManager`, `WEBKIT_COOKIE_PERSISTENT_STORAGE_TEXT`), the nine WebKit entry points and
the `nsxwidget_webkit_stop_loading` declaration all live in this unit's own blocks.
`check-dead-file-coverage.py` re-read after the table change: `145 ledger(s), 0 failure(s)`.


**⑮ Slice 8 (C 4028-4304): the readings of the round that landed it (2026-10-06, status 3.676).**

a) `python3 tools/extract-abi.py --check emacs/src/xwidget.c femacs/src/xwidget.rs`: **14 -> 2**
rows.  The twelve retired are the store/kill helper rows (`valid_xwidget_spec_p`,
`xwidget_spec_value`, `xwidget_view_delete_all_in_window`, `xwidget_view_lookup`,
`lookup_xwidget`, `xwidget_start_redisplay`, `xwidget_touch`, `xwidget_touched`,
`xwidget_end_redisplay`, **`kill_xwidget`** - the definition slice 6's use-site declaration could
not stand in for - `kill_frame_xwidget_views` and `kill_buffer_xwidgets`).  **What is left is the
drift row** (`line count drifted 48.9%: C 4304 vs Rust 6407`) **and exactly one function**:
`missing function x_draw_xwidget_glyph_string` (C 2793-3041), the file's last owed text.

b) **The frozen reading.**  `build-tu src/xwidget.c` -> built, `build/femacs/xwidget.o` **336
bytes** - the empty object of the P-86 class.

c) **The three arm flips against HEAD's own file**: A 51 -> **51**, B 13 -> **13**, C 24 -> **24**,
added 0 in every arm (after two fixes, below).

d) **Two fixes behind c.**  Run 1: `E0428: the name kill_xwidget is defined multiple times` - slice
6's use-site declaration must go now that C 4247's *definition* is here (its own comment said so;
the replacement note records it), and ARM A's `E0605: non-primitive cast Option<_> as u64` in
`lower_frame_xwidget_views` - C 4221's `None` needs `use x11_constants::None;` (slice 3's import
shape).  Run 2 is the table above.

e) **No mirror was edited this round**, so P-87 ①'s legs do not apply: `XLowerWindow`,
`nsxwidget_show_view`/`nsxwidget_hide_view`/`nsxwidget_kill` and C 4221's `None` import all live in
this unit.  `check-dead-file-coverage.py` re-read after the table change:
`145 ledger(s), 0 failure(s)`.


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
9. **C statements the mirror has no field for, carried and named rather than stubbed.**  The
   `kill_without_query` one now has **three sites**: `(*xw).kill_without_query = false;` (C 319),
   `(*XXWIDGET (xwidget)).kill_without_query = NILP (flag);` (C 3535) and the getter's read
   (C 3546).  `include/xwidget.rs` records C 99's `bool_bf … : 1` instead of storing it (P-80
   requires the record's byte dump first, and this record cannot be compiled here), and the arm
   compile reports `no field kill_without_query` for each site (Evidence ⑫ c: two of the three are
   this round's).  **P-80's storage unit for `struct xwidget` is the fix, and it now unblocks three
   sites.**  Separately and unchanged: `XG_XWIDGET`/`XG_XWIDGET_VIEW` are `&CStr` constants in that
   mirror, so C's string macro arrives as `XG_XWIDGET.as_ptr ()` at C 436-437.
10. **`xw_forward_event_translate` returns C's `int`, not a Rust `bool`.**  C 217 assigns it to a
    `bool`, and the port keeps the C type on the function and spells the conversion `!= 0` at the
    call site (the `src/androidmenu.rs` `popup_activated_flag` shape).


### Slice 2 (C 1181-2716)

- **The toolkit surface this slice reads is declared in the unit** (P-88 ①; the P-90 ③/④ move into
  `femacs/src/include/gtk.rs` is still owed, `## Owed` 2), each name with the `/// C:` citation its
  header and the C line that reads it.  **GDK, GTK, WebKit and XInput2 are not on this machine**, so
  every record carries only the members C reads, every enumerator is a *distinct placeholder* (the
  real values are `## Owed` 2/8), and no layout assertion is written for them; **X11 is**, so the
  four `XEvent` member records carry `clang`-measured layouts and `const _` assertions
  (`## Evidence` ⑥) and the `XMoveWindow` family's prototypes are the header's own.  Two of the
  slice's names were **already** in the mirror under a `have_gtk3` gate
  (`GTK_ORIENTATION_VERTICAL`, `gtk_box_new`), which is why the arm flip sets that feature
  (`## Gate mapping`).
- **C's `#ifdef`-split `if`/`else if`/`else` is the port's `match` with the `#ifdef` arm gated**, in
  `xwidget_motion_or_crossing`: C 1951-1976 (`if (event->type != GenericEvent) … else …`) and
  C 1977-2105 (`if (MotionNotify) … else if (GenericEvent) … else …`) both become a `match` on
  `event->type` with a `#[cfg (feature = "have_xinput2")]` arm; the order and the conditions are
  C's, and the arms are braced blocks (a `#[cfg]` cannot sit between `if` and `else`, and
  duplicating C's shared block would be a second copy of it).
- **C's `switch` is the port's `match` with braced arms**, and a `switch` *without* `default` gets
  an empty `_` arm - C's own fall-through, not invented behaviour: `xi_translate_notify_detail`
  (C 1557-1573), `xwidget_pinch` (C 1531-1549), `webkit_create_cb` (C 2415-2422),
  `webkit_view_load_changed_cb` (C 2436-2450) and `webkit_decide_policy_cb` (C 2578-2643).
- **C's implicit conversions are explicit `as` casts** (PORTING.md 1.1), and the three the first arm
  reading measured as `E0308`/`E0606`/`E0384` were the fixes of this round (`## Evidence` ⑧):
  eleven `Time` (`c_ulong`) -> the GDK records' `guint32` `time` members; `c_int` -> `c_double` for
  every coordinate/member write (`x`/`y`/`x_root`/`y_root`/`delta_x`/`delta_y`/`clip_*`); `c_int` ->
  `c_uint` for `state`/`modifier_state`; the `lrint` results -> `c_int`; the `double` arguments C
  passes to `xw_maybe_synthesize_crossing`'s `int x`/`int y` (C 1703-1710, C 2161-2166) -> `as
  c_int`; `Window` -> `EMACS_INT` at C 2140's `make_fixnum`; `gpointer` <-> `*mut GdkWindow`/
  `*mut xwidget` at the `g_object_ref`/`g_object_unref`/`g_signal_handler_disconnect`/
  `g_signal_connect` sites; and `(*xev).mods.effective as c_uint`.
- **C's `XEvent` union member reads go through casts to the member record** C names
  (`XAnyEvent`/`XMotionEvent`/`XCrossingEvent`/`XGenericEventCookie`), because `include/xlib.rs`'s
  `XEvent` is the measured 192-byte **opaque** record (its 33 member records are owed in
  `docs/ports/include-xlib.md`) - the `src/xmenu.rs` 1086 `event->xcookie` shape.  The four records
  and their measured layouts stand in the unit (`## Evidence` ⑥), so the reading is a
  name-resolution one, not a claim that the union is spelled.
- **C's uninitialised locals are seeded and named** (PORTING.md 0.1-13/1.5): `x_out`/`y_out`
  (C 1581-1582), `cursor` (C 1741), `entry` (C 2660) and every `struct input_event`/`GdkEvent` C
  leaves for `EVENT_INIT`/field assignments.
- **C's `for` loops become `while` with the step at the end, in C's order**
  (`window_coords_from_toplevel` C 1586-1599, `xw_find_common_ancestor`'s two C 1615-1626,
  `xw_notify_virtual_upwards_until` C 1657-1661, `xw_notify_virtual_downwards_until` C 1697-1703,
  `offscreen_damage_event` C 2254, `webkit_ready_to_show` C 2362, `webkit_js_to_lisp` C 2485/2504),
  and C's `while ((parent = gdk_window_get_parent (window)) != toplevel)` (C 1585) is the assignment
  before the loop with the same step at the end - the `## Evidence` ⑧ `E0384` is what caught the
  missing `mut`.
- **`min (5, …)` is `min! (5, …)`** (lisp.h 71-73, `lib/include/min-max.rs`, the
  `src/w32console.rs` 346 shape; `include/lisp.rs` carries only the `isize` *functions* of the same
  names, with which the macro coexists), and **`event.kind = …` is the per-unit `input_event_bits!`
  macro** (P-80 member dispatch; the `src/gfilenotify.rs` 183-189 shape, body-identical).
  `XIMaskIsSet` (XInput2.h 183) and `g_clear_pointer` (gmem.h 135-145) keep C's names as
  `macro_rules!` with the headers' own bodies, because calling them as functions would name symbols
  the libraries do not export (the mirror's `g_clear_object` precedent, `include/gtk.rs` 813).
- **`XWIDGET_EVENT`/`XWIDGET_DISPLAY_EVENT` are declared in the unit**, at the values a
  `HAVE_XWIDGETS` build gives them (28/29): `include/termhooks.rs` carries the frozen
  configuration's members and stops before termhooks.h's `#ifdef HAVE_XWIDGETS` pair, so the port
  spells them (the `src/gfilenotify.rs` 161-169 `FILE_NOTIFY_EVENT` precedent), and the mirror round
  that adds them owns the real values (`## Owed` 11).
- **C's `focus_change.in` is `r#in`** (a Rust keyword), and both the union member and the record it
  names are mirror gaps (`## Owed` 8).
- **`memory_full` is declared in the unit** (C 2499's call): no mirror carries it -
  `include/lisp.rs` has only `memory_full_up`, which C 2483's call uses - so the prototype stands at
  its use site (P-72 rule 4) and joins `## Owed` 3's list.
- **The XInput2 records' member order and every type is a reading of C's use sites, not of a
  header**: this machine has `X11/extensions/XI2.h` (the constants and `XIMaskIsSet` are cited from
  its lines) but **not** `XInput2.h`, which is where `XIEnterEvent`/`XIGesturePinchEvent` are
  defined.  C 1993-2040 reads `event_x`/`event_y`/`root_x`/`root_y`/`time`/`focus`/`mode`/`detail`/
  `mods.effective`/`buttons.mask_len`/`buttons.mask`, and C 1497-1507 reads `event_x`/`event_y`/
  `delta_x`/`delta_y`/`delta_angle`/`scale`/`root_x`/`root_y`/`mods.effective`/`evtype`; those
  member names are C's, the records are **UNVERIFIED** and their measurements are owed (`## Owed`
  10).

### Slice 3 (C 2717-2790 and C 3042-3116, status 3.671)

1. **`xwidget_init_view` keeps C's `struct glyph_string *s`**, so the ARM C set (GTK with no window
   system - an arm no configuration has: C's own body reads `s->f`/`s->w`) cannot name the type;
   that arm's one added row is the arm's, and A/B are unchanged (Evidence ⑩ c).
2. **The mirror's field types, at the write sites**: `xwidget_view.frame` is `*mut c_void` (P-55:
   the record cannot name `struct frame`), so C's `xv->frame = s->f` is `(*s).f as *mut c_void`;
   `just_resized` is the mirror's `c_int` for C's `bool`, so C's `false` is `0` (the read side,
   C 3933, is the port's `!= 0`).
3. **`XG_XWIDGET_VIEW`** is the mirror's `&CStr` (`include/xwidget.rs` 248), so the call takes
   `.as_ptr ()` where C passes the string literal.
4. **C 2752's `None`** is imported as `use x11_constants::None;` inside the arm - the shape
   `include/xlib.rs` 60-70 documents and the unit already uses at line 3445; without the import the
   name resolves to Rust's `Option::None` (`E0605: non-primitive cast Option<_> as u64`).
5. **`g_signal_connect` is this unit's own macro** (C 225-233), so the two calls are
   `g_signal_connect! (...)`.
6. **`WEBKIT_FN_INIT`** declares `struct xwidget *xw` at its expansion site in C, which Rust macro
   hygiene cannot hand out (the `ITREE_FOREACH` class in `docs/ports/macro-body-notes.tsv`), so the
   port's macro runs C's three statements and answers `xw`; each caller binds it with C's name.
7. **`webkit_web_view_get_estimated_load_progress`'s `gdouble`** is spelled `c_double` in the
   unit's WebKit block (the block keeps GLib's names only for types the header defines, e.g.
   `gboolean`/`WebKitHardwareAccelerationPolicy`).
8. **`nsxwidget_init_view`'s third parameter** is `struct glyph_string *` in C and `*mut c_void` in
   the port's use-site declaration (P-72 rule 3), because the declaration must resolve in every arm
   the file gate admits.


### Slice 4 (C 3117-3307, status 3.672)

1. **C's two assignable `DEFUN` parameters are rebound, not `mut`**: the `DEFUN!` macro's pattern
   is `$arg:ident : $ty:ty`, so `uri` (C 3125) and `script` (C 3230) are bound as
   `let mut … = …;` at the top of the body (the `window.rs` 4267 / fringe slice 2 shape).
2. **`GAsyncReadyCallback`** is declared with the unit's own GLib surface (beside `GAsyncResult`),
   as `Option<unsafe extern "C" fn (…)>` - the `include/gtk.rs` 93-96 shape.
3. **`webkit_javascript_finished_cb` becomes `unsafe extern "C" fn`**: C 3244 passes it as a
   `GAsyncReadyCallback`, and C's `static void f (...)` has that type; the port's earlier slice
   wrote a plain `unsafe fn` because nothing called it through a function pointer yet (the
   `init_alloc_once_for_pdumper` correction).
4. **Two chained assignments are split** (`xw->script_callbacks = cbs = make_nil_vector (32);` and
   `… = cbs = larger_vector (cbs, 1, -1);`) - Rust's `=` is not an expression (the C 1662 shape).
5. **`XFIXNUM`'s `EMACS_INT` is cast to GLib's `gint`** where C passes it implicitly (C 3158).
6. **`check_integer_range`'s bounds are `intmax_t`** (lisp.h 3773), not `EMACS_INT`: the two calls
   cast `0` and `INT_MAX` accordingly.
7. **`xlispstrdup`'s `char *`** is cast through `*mut c_void` into `make_mint_ptr` (C 3212).
8. **Four use-site declarations** (this unit, P-72 rule 4): `larger_vector` (lisp.h 4245),
   `xlispstrdup` (lisp.h 5553), `FUNCTIONP` (lisp.h 4799 - the `androidselect.rs` 145-152 shape)
   and `redisplay` (xdisp.c 17073 - the `keyboard.rs` 680 shape).  Each is owed to the mirror that
   should carry it, and this unit's Owed list says so.


### Slice 5 (C 3308-3547, status 3.673)

1. **C 3429-3430's `#else` is spelled `have_pgtk`** (the arm in which `include/xwidget.rs` 145-148
   declares `dpyinfo`/`widget`), not `not (feature = "have_x_windows")` - see Evidence ⑫ d.
2. **C 3535 and C 3546 are carried as C writes them** (`(*XXWIDGET (xwidget)).kill_without_query =
   NILP (flag);` and the read in the getter), with the same comment the C 319 site carries: the
   record's `bool_bf … : 1` is **recorded, not packed** by the mirror, so there is no field to write
   and the arm compile reports `no field kill_without_query` - neither stubbed nor dropped
   (goal constraint 3).  **Owed 9 now names three sites** (C 319, C 3535, C 3546), which is the case
   for landing P-80's storage unit for this record.
3. **Three EMACS_INT casts**: `list2i` (lisp.h 2078) and `make_fixed_natnum` (lisp.h 781) take
   `EMACS_INT`, and C 3320/3354/3366-3368 pass `gint`/`c_int`/`EMACS_INT` mixes implicitly.
4. **`GtkRequisition requisition;`** (C 3318) is C's *uninitialised* local, so the port writes
   `core::mem::zeroed ()` (PORTING.md 1.5 / P-13) and passes `&mut requisition`.
5. **`g_signal_handler_disconnect`'s first parameter** is the port's `*mut c_void`; C 3415/3422/
   3424 pass `GdkWindow *`/`GtkWidget *`, so each call casts.
6. **`XDestroyWindow`** (C 3410) is declared at the use site (X11/Xlib.h's prototype in a `/// C:`
   line, the `XMoveWindow` family's shape at line 2694-2710).
7. **`nsxwidget_get_size`** (C 3322, nsxwidget.h 47) and **`nsxwidget_delete_view`** (C 3443,
   nsxwidget.h 48) are declared at the use site under the file gate + `ns_impl_cocoa` (P-72 rule 4).
8. **`xwidget-view-lookup`'s `window` parameter is rebound** (`let mut window = window;`) because
   C 3463 assigns it - the `window.rs` 4267 shape, spelled from the start rather than after a
   failed arm run.


### Slice 6 (C 3548-3770, status 3.674)

1. **C's `#ifdef USE_GTK` locals stand at function scope with no inner `#[cfg]`**: this file's gate
   is `USE_GTK` (the ledger's `## Gate mapping`), so an inner block would only hide them from the
   body - measured on the first arm run (thirty `E0425`s), and the fix is structural, not a
   suppression (Evidence ⑬ d).
2. **`opt as guint32`** at C 3602: C passes `WebKitFindOptions` (its enum, `c_int` here) to the
   `guint32 find_options` parameter.
3. **`kill_xwidget` is declared at the use site** with C 96's own prototype (`static void
   kill_xwidget (struct xwidget *);`) because C 4247's definition is a later slice's; the
   declaration carries the file gate, and the checker's `missing function kill_xwidget` row stays
   until the definition lands (the declaration is what the *arm* needs).
4. **The WebKit find-controller surface**: `WebKitFindController` is the `[u8; 0]` record shape the
   block's other records use, `WebKitFindOptions` is `c_int` (the port's enum spelling), and its
   four members are **distinct placeholders that keep C's bit shifts** (`1 << 0`, `1 << 3`,
   `1 << 4`), because C 3586-3594 OR's them - the values themselves are Owed 2/8's (no WebKit tree
   on this machine).  **UNVERIFIED**, as the block's note says.
5. **`G_MAXUINT`** (`glib/gmacros.h`'s `((guint) 0xffffffff)`) is declared beside the unit's
   `G_MAXINT32`, not in a mirror: it is the GLib constant section this unit already keeps
   (`include/xwidget.rs`'s arm has no GTK/GLib mirror to own it).
6. **`xwidget-webkit-load-html`'s three assignable parameters** (`base_uri` at C 3751, `text` at
   C 3756) are rebound at the top of the body- the `window.rs` 4267 shape.
7. **`xwidget-webkit-search`'s `query`** (C 3585) is rebound the same way, and the function's five
   `#ifdef USE_GTK` locals are declared at function scope (Deviation 1).
8. **The three `G_MAXUINT`-adjacent types are the mirror's**: `guint`/`guint32`/`gchar` come from
   `include/gtk.rs`, and `xstrdup`/`ENCODE_UTF_8`/`SSDATA`/`xfree`/`build_string` from the mirrors
   this unit includes - no new cross-unit declaration beyond `kill_xwidget` (Deviation 3).


### Slice 7 (C 3771-4027, status 3.675)

1. **The `DEFVAR_BOOL` carrier is the unit's own** (P-97): `pub static mut
   xwidget_webkit_disable_javascript: bool = false;` with `#[unsafe(no_mangle)]`, which shadows the
   `include/globals.rs` 7205 `extern` for this TU (P-21's "definition shadows the glob import").
   C 4001's `= false` is the initialiser.
2. **`staticpro` takes a `*const Lisp_Object`** in the port (`alloc.rs` 5815), so C's four
   `staticpro (&x)` sites are `staticpro (&raw const x)`.
3. **C 4019's `#if WEBKIT_CHECK_VERSION (2, 34, 0)`** is `feature = "webkit_check_version_ge_2_34"`,
   the spelling this file's head comment derives from the reference (`WEBKIT_CHECK_VERSION` is
   defined nowhere in the reference tree) - its fourth site; the three derived features are still
   unregistered in `femacs/xtask/Cargo.toml` (`## Owed` 1).
4. **The two GList walks** (C 3829-3837 and C 3847-3855) are C's comma-expression `for` written as
   an init plus a `while` plus the two update statements in C's order (`tem = tem->next; ++i;`) -
   the same shape `fringe.c` slice 2's loops carry.
5. **`lim = XFIXNAT (limit) as c_uint`** and **`i as EMACS_INT` / `-(i as EMACS_INT)`**: C's
   `unsigned int lim` and `make_fixnum (-i)` mix the widths implicitly.
6. **`build_string_from_utf8 (item_title ? item_title : "")`** is an `if`/`else` expression in the
   port (Rust has no `?:`), with `c""` for C's empty string literal.
7. **`internal_xwidget_list`/`internal_xwidget_view_list`/`id_to_xwidget_map`/
   `dummy_tooltip_string`** are this unit's own `static mut`s (already declared by slice 1's
   prefix), and C 4015-4022's `HAVE_X_WINDOWS` block carries the arm plus C 4019's version test
   nested inside it, as C nests them.


### Slice 8 (C 4028-4304, status 3.676)

1. **C 96's prototype and slice 6's use-site declaration are replaced by the definition**: C 4247's
   `kill_xwidget` is now this file's own `unsafe fn` (P-91: Rust has no prototype, so the file
   carries the definition where C defines it and the declaration where C declares it disappears with
   it).  The note left in the file says so.
2. **`lower_frame_xwidget_views` (C 4212-4224) is carried even though the frozen configuration never
   compiles it**: C 4211's `#ifdef HAVE_X_WINDOWS` arm is dead here (`docs/baseline/config` has
   `HAVE_X_WINDOWS undef`), but P-86 ① carries both arms of every `#if`, so the item exists under
   `have_window_system` + `have_x_windows` and its own `XLowerWindow`/`None` declarations sit at the
   use site.  The checker's `missing function` list never named it for the same reason (its
   live-line filter is empty for dead-config files) - which is why this slice retires twelve rows
   for *thirteen* functions.
3. **`kill_frame_xwidget_views`'s gate is `not ns_impl_cocoa`** (C 4227's `#ifndef`), spelled in the
   port's feature terms rather than as a negation of the file gate.
4. **C 4221's `None` needs the `x11_constants` import** inside `lower_frame_xwidget_views` (the
   `include/xlib.rs` 60-70 module; slice 3's note), otherwise it resolves to `Option::None`.
5. **`xwidget_end_redisplay`'s glyph walk** keeps C's pointer arithmetic (`glyph.add (…)`,
   `glyph < glyph_end`) and reads `glyph`'s two bitfields through the mirror's accessors
   (`glyph_bits! (type, …)`, `glyph_row_bits! (enabled_p, …)`) and its union member as
   `(*glyph).u.xwidget` (the mirror's `glyph_u`).
6. **`xfree (xmint_pointer (XCAR (cb)))`** in C 4272 passes `void *`; the port's `xfree` takes
   `*mut c_void` and `xmint_pointer` already answers it, so no cast is needed - recorded because the
   neighbouring sites in this file do need one.
7. **The two `for (Lisp_Object tail = …; …)` loops** are init + `while` + the update statement, and
   `kill_buffer_xwidgets`'s inner C block is kept as a bare block (C's own braces).


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
4. **An arm flip that can actually link.**  With ③'s (and 8-10's) gaps closed the arm readings
   should be re-run; **slice 2 has now been flipped** (Evidence ⑦/⑧: ARM A11 leaves 85 errors
   anchored in the unit, *none of them this slice's own text*, and ARM PG leaves 12, of which 10
   are slice 1's `cursor_for_hit`/`define_cursors` - owed 13), and **C 2718-4304 is still
   unported**, which is the other half of item 7.
5. `docs/ports/line-drift-tolerance.tsv`: **the row this item used to promise is not in the file**
   (`grep -n xwidget docs/ports/line-drift-tolerance.tsv` is empty, checked this round), and none
   is needed yet: the port is 4,492 Rust lines against C's 4,304 (4.4%, under the 5% rule), so
   `extract-abi --check`'s `line count drifted` row is gone (Evidence ⑨).  The row - stem
   `xwidget`, basis `total`, the basis being the per-item `#[cfg]` gates, the ~1,700 lines of
   third-party declarations this class requires and Rust's explicit casts - becomes necessary when
   the remainder lands (owed 12).
6. `HAVE_XINPUT2`'s four slice-1 blocks (C 519-522/537-548/624-631/684-689) were carried as text
   only; **slice 2 flips them** (ARM A11 sets `have_xinput2`/`have_xinput2_4`, Evidence ⑧) but the
   arm cannot *resolve* until `XIEventMask`/`x_display_info` land in `include/xterm.rs` (owed 10),
   so the reading is still a name-resolution one and not a compile.
7. **The next slice is the file's last one.**  Slices 3-8 (status 3.671-3.676) landed
   C 2717-2790, C 3042-3116, C 3117-3307, C 3308-3547, C 3548-3770, C 3771-4027 and C 4028-4304;
   **the only owed text left in this file is C 2791-3041** - C 2793's `x_draw_xwidget_glyph_string`,
   the densest function in the file (the `HAVE_X_WINDOWS`/`HAVE_PGTK`/`NS_IMPL_COCOA` arms,
   `XCreateWindow`/`XISelectEvents`/`cairo_xlib_surface_create`).  The `--check` reading is two rows:
   that function and the line-count drift, so land it and the unit is text-complete; the drift row
   then needs `docs/ports/line-drift-tolerance.tsv`'s decision (the three `failing` files at the end
   of status 3.676 are the same class: a port that carries *more* text than C because every arm is
   text).


8. **The mirror's GDK event records need the members this slice reads.**  The ARM A11 reading's 84
   `E0609`s are that list field by field: `GdkEvent`'s union needs `focus_change` (a
   `GdkEventFocus` with `window`/`in`) and `touchpad_pinch` (a `GdkEventTouchpadPinch` with
   `x`/`y`/`dx`/`dy`/`angle_delta`/`scale`/`x_root`/`y_root`/`state`/`n_fingers`/`phase`);
   `GdkEventScroll` needs `direction`/`device`/`x_root`/`y_root`/`state`/`delta_x`/`delta_y`/
   `is_stop`; `GdkEventMotion` needs `x_root`/`y_root`/`state`/`device`; `GdkEventCrossing` needs
   `x_root`/`y_root`/`mode`/`detail`/`focus`/`state`; `GdkEventButton` needs `x_root`/`y_root`/
   `button`/`state`/`device`.  With them, the real enumerator values of
   `GdkScrollDirection`/`GdkTouchpadGesturePhase`/`GdkNotifyType`/`GdkEventType`'s two new members.
   `femacs/src/include/gtk.rs` is outside this lane's write scope (`## Not claimed`).
9. **`femacs/xtask/Cargo.toml`'s `[features]` table** gains **`gtk_check_version_ge_3_18`** (the
   `gtk_check_version_ge_3_10` naming; C 1493's `GTK_CHECK_VERSION (3, 18, 0)`) on top of item 1's
   three `webkit_check_version_*` names - without it the C 1491-1550 body cannot be selected.
10. **The mirror gaps the arm flip trips** (all outside this lane's write scope): `XIEventMask` is
    unresolved in `include/xterm.rs` 919 (so the XInput2 arm cannot resolve at all), `x_display_info`
    is unresolved where the same mirror's `xw_translate_x_modifiers` names it, the two XInput2
    records have no mirror and their layouts are unmeasured (`## Evidence` ⑥), and
    `include/systime.rs`'s `Time` assertion (`const _: () = assert! (!cfg! (feature = "have_x_windows")
    || HAVE_X_WINDOWS);`) fails whenever `feature = "have_x_windows"` is set, because
    `include/config.rs`'s `HAVE_X_WINDOWS` const is the *frozen* `undef`.  A real arm flip needs the
    port's `HAVE_X_WINDOWS`/`HAVE_XINPUT2`/`HAVE_GTK3` constants to be true in `include/config.rs`.
11. **`XWIDGET_EVENT`/`XWIDGET_DISPLAY_EVENT` belong in `include/termhooks.rs`** under
    `have_xwidgets` (termhooks.h 273-281): this unit and `src/keyboard.rs` 4761/7973 both read them,
    and the unit's declaration (`## Deviations`) is a stand-in until they land there with the
    `HAVE_XWIDGETS` build's values.
12. **`docs/ports/line-drift-tolerance.tsv`: the `xwidget` row the old item 5 promised does not
    exist** (`grep -n xwidget docs/ports/line-drift-tolerance.tsv` is empty, checked this round).
    With the port at 4,492 Rust lines against C's 4,304 the 5% rule passes, so no row is needed
    *this round*; the remainder (C 2717-4304, 1,588 lines) will need the row, derived from a
    finished prefix the way that table's own note describes.
13. **The PGTK arm's 12 unit-anchored errors** (`## Evidence` ⑧, ARM PG): 2 are this slice's
    (`GdkEvent.focus_change`, item 8) and 10 are slice 1's latent defects, visible only under
    `have_pgtk` - `cursor_for_hit`/`define_cursors` (C 1065-1119) cast
    `(*FRAME_OUTPUT_DATA! (frame) as *mut pgtk_output)` where `pgtk_output` is a record in
    `include/pgtkterm.rs` rather than a pointer, measured as 4 `E0605` (non-primitive cast) + 5
    `E0609` (`nontext_cursor`/`text_cursor`/`vertical_drag_cursor`/`hand_cursor`) + the shared
    `kill_without_query` (item 8's P-80 bitfield).  The line is
    `(*FRAME_OUTPUT_DATA! (frame) as *mut pgtk_output).nontext_cursor`, whose correct spelling needs
    either the mirror's `x_output`-style pointer or the explicit parens the X11 arm's site now
    carries.

## P-90's second step (2026-10-05): the move is blocked, and the blocker is measured

- **This unit's segment did not move**, and the reason is not this file: `include/gtk.rs`'s
  `GdkEvent` union carries `r#type`/`any` (the two members `gtkutil.c`/`pgtkterm.c` read), while
  this unit's own union (C'the seven-member `_GdkEvent`) also carries `button`, `scroll`, `motion`,
  `crossing` and `key` - and this slice reads `(*xg_event).key.keyval`/`.key.state` at C 1315-1452.
  Landing the edge that brings the mirror's union in produces
  `E0428 the name GdkEvent is defined multiple times` plus `E0119`/`E0740`/two `E0609 no field key`
  (measured: arm A 53 -> 54 with those five classes) unless the union's members move into
  `include/gtk.rs` too - 4 records (`GdkEventButton`/`Scroll`/`Motion`/`Crossing`) and the `key`
  member.  That is a change *inside* the mirror, outside this round's write scope (the mirror's own
  in-flight change, the `src/xgselect.rs` slice's four `gdk_x11_*` gates, landed at `bbb378d`
  while this round ran), so the move waits for the next one - it is `## Owed` 2's remainder, now
  with the exact member list.
- The stand-in edge this unit will use is already in place at C 30 (`include/gtkutil.rs` first, then
  `include/gtk.rs`), and the 59 names to drop are named in Owed 2; nothing in this file changed.

## P-90's third step (2026-10-05, status 3.481): the move lands

The two blockers the second step measured were inside `include/gtk.rs` (the mirror), so the
integrator wrote them there and did the move in the same round:

* `include/gtk.rs` (1,654 -> **1,720** lines): the `GdkEvent` union gains its five members
  (`button`/`scroll`/`motion`/`crossing`/`key`), the four missing records are added
  (`GdkEventButton`/`GdkEventScroll`/`GdkEventMotion`/`GdkEventCrossing`), `GdkEventKey` is
  **upgraded from the zero-sized placeholder to the record with `state`/`keyval`** (it is also
  `pgtkterm.h` 666's pointer parameter type - one surface, one mirror, P-90 ②), and
  `GtkWidgetClass` gains `style_updated`.
* `femacs/src/xwidget.rs` (2,321 -> **2,014** lines): C 30's chain gains
  `include!("include/gtk.rs")` and the **56** declarations that duplicated the mirror are deleted
  (the set was fixed by the compiler: the edge's 56 `E0428`s and a name-by-name scan agreed
  exactly).

Readings (my own): arm A (`use_gtk,have_xwidgets,have_x_windows,have_window_system`)
**51 -> 46** errors (34 `E0425` + 11 `E0133` + 1 `E0609`, **0 `E0428`**); arm C (without
`have_x_windows`) 45 (33 + 1 + 1, 0 `E0428`); `extract-abi --check` leaves 84 `- missing` rows and
the 53.2% line drift (the unported remainder) and **no `does not mention` row**; `layout-check`
passed; `tools/check-all-units.sh` **83/0 + 83/0**.  Two self-inflicted defects were caught by the
gates and fixed in the same round: a deletion script that treated a declaration with a trailing
comment as unterminated (it swallowed 43 lines; `rustc`'s `unexpected closing delimiter` caught
it), and a forgotten `GdkEventKey` record after the placeholder became a comment
(`layout-check`'s `cannot find type GdkEventKey` caught it).  The records' layout and member
order stay **UNVERIFIED** - no GDK tree here, and no layout assertion is written.

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ⑦: a 336-byte empty object), so this is a *text* port: nothing here has been executed,
  and no GTK/WebKit window, offscreen surface, X11 window or input device was involved.  There is
  no behavioural test, no differential run, no ERT and no device evidence; the C 1181-2716 text has
  never been compiled against a real GDK/GTK/WebKit tree.
- **The arm readings are not GTK compiles.**  This machine has no `gtk-3.0`/`gtk-4.0`/
  `webkit2gtk-*`/`JavaScriptCore` tree and no `pkg-config`, so Evidence ⑦/⑧ are `rustc`
  name-resolution and type-check readings of the materialised root under hand-set `--cfg`s: 85
  errors anchored in this unit for ARM A11 (84 of them the mirror's GDK record members, 1 the
  unported `kill_xwidget`) and 12 for ARM PG (10 of them slice 1's).  "0 anchored in this slice's
  own text" means exactly that - not that the unit compiles, and not that either arm is a working
  GTK configuration.  The `have_x_windows` + `have_pgtk` combination is *not* a reading at all
  (Evidence ⑧'s vacuous run).
- **Every GDK/GTK/WebKit/XInput2 record in this slice stays unmeasured.**  No `#[repr (C)]` record
  of theirs carries a size or an offset: they carry the members C reads and nothing else, their
  enumerators are distinct placeholders (not the toolkit's values), and no layout assertion is
  written (owed 2/8/10).  Only the four X11 member records are measured (Evidence ⑥).
- **`extract-abi --check` still FAILs for the file**, by construction: 81 `- missing` rows remain
  (34 `subr`, 34 `defsubr`, 11 `function`, 2 `macro`) - they are C 2718-4304's and the header's
  Subrs.  The `line count drifted` row is gone (4.4%), which is a *totals* signal and not a
  statement that the remainder is ported; `## Owed` 7 names the C range that is still owed.
- **This ledger carries its Gate-mapping and C-line-range sections twice** (a
  pre-existing duplication from an earlier round, left in place this round: `python3
  tools/check-dead-file-coverage.py` reads the first table and both were updated identically).
- **Nothing in this round is committed**: no git write of any kind was performed (the lane's
  mandate), so this ledger's readings describe the working tree, not a revision.
