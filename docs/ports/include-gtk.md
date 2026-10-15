# the GTK SDK surface — `femacs/src/include/gtk.rs` (1584 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

> **This mirror has no reference file**: it is the port's declaration of a *system header of
> another target's toolchain* (PORTING.md P-26/P-49/P-90), the same class as `include/xlib.rs`
> (`<X11/Xlib.h>`), `include/jni.rs` (the NDK's `<jni.h>`) and `lib/include/haiku_support.rs`
> (the Haiku SDK).  `tools/check-target-names.py` accepts its first line
> (`// femacs: mirror of the GTK's <gtk/gtk.h> (system header of another target's toolchain,
> PORTING.md P-26/P-49).`) as that declaration; it has no row in `docs/port-map.md` and no
> `--check` reading, which is why the sections above are empty.

## What it carries (P-90 ①/②: the surface's single home)

The ~350 declarations that were the in-unit segments of `src/gtkutil.rs` (C 44/49's chain) and
`src/pgtkterm.rs` (C 28/C 48's chain), moved here in the P-90 mirror round (P-88 ⑤'s move-back),
by header:

- **GLib/GObject** (`<glib.h>`, `<glib-object.h>`, from `/opt/homebrew/include/glib-2.0`,
  GLib 2.90.0): `gchar`/`gint`/`gboolean`/`guchar`/`gulong`/`guint`/`guint32`/`gsize`/`gpointer`/
  `gssize`, `GQuark`/`GType`/`GList`/`_GList`/`GObject`/`GClosure`/`GCallback`/`GClosureNotify`/
  `GConnectFlags`/`GSignalMatchType`/`GSourceFunc`/`GError`/`GConvertError`/`GLogLevelFlags`/
  `GLogFunc`, `TRUE`/`FALSE`, the `G_OBJECT`/`G_CALLBACK`/`g_list_next`/`g_signal_connect`/
  `g_signal_handlers_disconnect_by_func`/`g_clear_object` macros, and forty entry points
  (`g_signal_connect_data`, `g_signal_handler_find`, `g_signal_handlers_disconnect_matched`,
  `g_object_*`, `g_list_free`, `g_log_*`, `g_source_remove`, `g_timeout_add`, `g_strdup`,
  `g_strdup_printf`, …).  **Measured on
  this machine**: `GList` (24/8, `data` 0/`next` 8/`prev` 16), `GType` (8), `GError` (16, `code` 4,
  `message` 8) and the enumerators `G_CONVERT_ERROR_ILLEGAL_SEQUENCE` 1,
  `G_LOG_FLAG_RECURSION` 1/`G_LOG_FLAG_FATAL` 2/`G_LOG_LEVEL_WARNING` 16,
  `G_SIGNAL_MATCH_FUNC` 1 << 3, `G_SIGNAL_MATCH_DATA` 1 << 4 - those are the port's only
  `core::assert!`s.
- **cairo** (`/opt/homebrew/include/cairo`, 1.18.4): `cairo_t`/`cairo_surface_t`/
  `cairo_pattern_t`, `cairo_format_t`/`cairo_operator_t`/`cairo_pattern_type_t`/
  `cairo_status_t`/`cairo_surface_type_t`/`cairo_region_t`, the `CAIRO_FORMAT_ARGB32`/
  `CAIRO_OPERATOR_SOURCE`/`CAIRO_PATTERN_TYPE_SURFACE`/`CAIRO_SURFACE_TYPE_*` values (measured:
  `CAIRO_FORMAT_ARGB32` 0, `CAIRO_OPERATOR_SOURCE` 1, `CAIRO_PATTERN_TYPE_SURFACE` 1) and
  nineteen entry points.
- **GDK** (`<gdk/gdk.h>` and friends): the opaque handles `GdkDisplay`/`GdkScreen`/`GdkCursor`/
  `GdkWindow`/`GdkPixbuf`/`GdkVisual`/`GdkSeat`/`GdkDevice`/`GdkDragContext`/`GdkColormap`/
  `_GdkAtom`, the words `GdkRGBA`/`GdkColor`/`GdkGeometry`/`GdkRectangle`, the enumerations
  `GdkEventType`/`GdkEventMask`/`GdkWindowHints`/`GdkGravity`/`GdkColorspace`/`GdkCursorType`/
  `GdkInputSource`, `GdkWindowTypeHint` (one named member, a placeholder, C 2754's
  `GDK_WINDOW_TYPE_HINT_TOOLTIP`), the fifteen `GdkCursorType` enumerators C 1556-1591's `INSTALL_CURSOR`
  paste names (`GDK_LEFT_PTR` and, added by the pgtkfns third slice, `GDK_XTERM`/`GDK_HAND2`/
  `GDK_WATCH`/`GDK_SB_H_DOUBLE_ARROW`/`GDK_SB_V_DOUBLE_ARROW`/`GDK_LEFT_SIDE`/`GDK_RIGHT_SIDE`/
  `GDK_TOP_SIDE`/`GDK_BOTTOM_SIDE`/`GDK_TOP_LEFT_CORNER`/`GDK_TOP_RIGHT_CORNER`/
  `GDK_BOTTOM_RIGHT_CORNER`/`GDK_BOTTOM_LEFT_CORNER`), the event records `GdkEvent`/`GdkEventAny`/`GdkEventKey`/`GdkEventProperty`/
  `GdkEventSelection`/`GdkEventSequence`, the monitor handle `GdkMonitor` and the ten monitor
  entry points the pgtkfns subrs walk (`gdk_display_get_n_monitors`, `gdk_display_get_monitor`,
  `gdk_display_get_monitor_at_window`, `gdk_monitor_get_geometry`, `gdk_monitor_get_workarea`,
  `gdk_monitor_get_height_mm`, `gdk_monitor_get_width_mm`, `gdk_monitor_get_model`,
  `gdk_monitor_get_scale_factor`, `gdk_monitor_is_primary`), the three seat/device rows the
  tooltip position query needs (`gdk_display_get_default_seat`, `gdk_seat_get_pointer`,
  `gdk_device_get_position`), and fifty-three entry points.
- **GTK** (`<gtk/gtk.h>`): the opaque handles `GtkWidget`/`GtkWindow`/`GtkMenu`/`GtkImage`/
  `GtkBin`/`GtkContainer`/`GtkLabel`/`GtkMenuShell`/`GtkMenuBar`/`GtkMenuBarClass`/
  `GtkWidgetClass`/`GtkTooltip`/`GtkSettings`/`GtkStyleContext`/`GtkStyle`/`GtkRcStyle`/
  `GtkIMContext`/`GtkCssProvider`/`GtkStyleProvider`/`GtkBox`/`GtkFixed`, the words
  `GtkAllocation`/`GtkRequisition`, the enumerations `GtkWindowType`/`GtkOrientation`/
  `GtkStateType`/`GtkStateFlags`/`GtkTextDirection`, the cast/check macros
  (`GTK_WIDGET`/`GTK_WINDOW`/`GTK_BOX`/`GTK_CONTAINER`/`GTK_LABEL`/`GTK_MENU_SHELL`/
  the four window rows the pgtkfns tooltip frame needs (`gtk_window_set_transient_for`,
  `gtk_window_set_attached_to`, `gtk_window_set_destroy_with_parent`, `gtk_window_set_type_hint`),
  `GTK_WIDGET_CLASS`/`GTK_STYLE_PROVIDER`/`GTK_IS_*`/`GDK_WINDOW`/`GDK_DISPLAY_XDISPLAY`), the
  GTK2 compatibility macros (`gtk_box_new`, `gtk_scrollbar_new`, the three font-chooser renames)
  and a hundred entry points.

**GDK and GTK are not on this machine**, so every citation of theirs names the **header alone, no
line**, **no layout assertion** is written for a GDK/GTK record, and every enumerator/flag value of
theirs is a **placeholder** (`// placeholder, not GDK's value`) - the `src/xwidget.rs` 495-520 rule:
the toolkit's values are not measurable here, so the port keeps the arms and the bit tests distinct
rather than inventing behaviour.  `ls -d /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0`
-> both absent; `pkg-config` is not installed; no `gtk/gtk.h`/`gdk/gdk.h` under `/opt/homebrew`,
`/usr/local` or `/Library`.

## Consumers (P-90 ②: the edge at the referencing header's C `#include`)

| includer | at | gate |
| --- | --- | --- |
| `femacs/src/gtkutil.rs` | C 44's chain (`"gtkutil.h"` 26's `<gtk/gtk.h>`, with C 49's `<gdk/gdkkeysyms.h>`) | `feature = "use_gtk"` |
| `femacs/src/pgtkterm.rs` | C 28's `<cairo.h>` / C 48's chain | `feature = "have_pgtk"` |
| `include/pgtkterm.rs` | its C 31 (`#include <gtk/gtk.h>`) | `feature = "have_pgtk"` |

**`include/gtkutil.rs`'s C 26 edge is owed** (Owed 1 below): its consumer set includes
`src/pgtkfns.rs`, which still carries its own in-unit segment, so the edge would land twice in that
TU.

## Deviations

1. **The SDK's own first line** is the P-90 declaration and not `// femacs: port of …` - the file
   is not a port of a reference file (`docs/ports/include-xlib.md`'s shape).
2. **The items carry the *surface's* gate at the edge, not per item**: the moved text kept the
   `#[cfg (feature = "use_gtk")]` / `#[cfg (feature = "have_pgtk")]` attributes its former units
   gave it, and a pass dropped exactly those two conjuncts, keeping the real arm splits
   (`have_gtk3` 23, `not (have_gtk3)` 10, `not (have_pgtk)` 4, `not (gtk_check_version_ge_3_22)` 1,
   `all (have_gtk3, have_xinput2)` 1).  Without that, a `use_gtk`-only arm carried the *users* of a
   `have_pgtk`-gated declaration and reported nine spurious `E0425`s (measured: arm A of
   `gtkutil.c` read 62 = 19 + 43 before the pass, 52 = 19 + 33 after).  Four GTK3 records and
   `GTK_IS_LABEL` were widened from `not (have_pgtk)` to `have_gtk3`, and the GTK entry points of
   two mixed blocks were widened the same way, because GTK3 provides them in **both** arms and
   `src/pgtkterm.c` calls `gtk_widget_get_parent`/`gtk_im_multicontext_new`; the four X11-GDK
   entries (`gdk_x11_*`, `GDK_DISPLAY_XDISPLAY`) and the two `x_display_pixel_*` stayed X-only.
3. **`g_clear_object` is both a macro and a function**: gobject.h 694 declares
   `void g_clear_object (GObject **object_ptr);` one line above 695's macro of the same name, and
   `src/pgtkterm.c` calls the function, so the mirror carries both spellings (the
   `src/image.rs` 1849-1872 same-name pair; a `macro_rules!` and a same-named `pub fn` coexist and
   the call site's spelling picks).
4. **No behaviour was executed anywhere**: there is no GTK tree to compile an arm against and no
   GTK target to run on.
5. **Three names the mirror used but did not own** (the layout round below): `VisualID` is spelled
   as the integer it aliases (`c_ulong`, P-55) and `x_display_info` as `*mut c_void` (its record
   is `include/xterm.rs`'s, and the two entry points' consumer casts at the call site - the
   `include/xsettings.rs` 12-14 rule); `gtk_style_context_remove_provider` took the `have_gtk3`
   gate of the two GTK records it names instead of standing ungated.  No name was dropped and no
   signature changed shape beyond those stand-ins.

## Evidence

- **`layout-check` is this mirror's own compile** (P-87 ③): every `src/include/*.rs` becomes one
  crate with no `#[cfg (feature = …)]` on, i.e. the only place the mirror is compiled *without* an
  includer.  `cd femacs && CARGO_TARGET_DIR=/tmp/gtklc2/cargo cargo run -q -p xtask -- layout-check`
  -> `0 error`s, `119 warnings emitted`, `layout probe ok`, `layout-check passed`, exit 0.  The
  same command on the round's first version read **5 `error[E0425]: cannot find type`** - one
  `VisualID`, two `x_display_info`, one `GtkStyleContext`, one `GtkStyleProvider`.

| site | name | its home | repair |
| --- | --- | --- | --- |
| `gdk_x11_screen_lookup_visual`'s `xvisualid` | `VisualID` | `include/xlib.rs` 842-844 (`X11/X.h` 76: `typedef unsigned long VisualID;`) | P-55's ABI-equivalent scalar spelling, `c_ulong` |
| `x_display_pixel_width`/`x_display_pixel_height`'s `dpyinfo` | `x_display_info` | `include/xterm.rs` 383 (`struct x_display_info`) | P-55's `*mut c_void` (the record only ever appears behind a pointer) |
| `gtk_style_context_remove_provider`'s two parameters | `GtkStyleContext`, `GtkStyleProvider` | **this mirror** (its GTK3 block) | the entry point took the `have_gtk3` gate its two names already carry |

- **Why the *unit* could resolve the three and the *probe* could not** (the lesson worth keeping):
  a unit crate carries the whole include graph and the arm's conditions, so `have_x_windows` on
  brings `include/xterm.rs`'s `x_display_info` and `include/xlib.rs`'s `VisualID`, and `have_gtk3`
  on brings the two GTK records - the mirror's references resolved through the *includer's* names
  and every arm reading stayed green; `layout-check` compiles the mirror as a crate root where no
  includer and no feature exists, so the same references had no declaration at all and only that
  gate could see it.  (Measured the same way: `include/pgtkterm.rs`'s names, whose guards are the
  same class, read `E0425`s only there too.)
- **`x_display_info`'s stand-in keeps the units compiling** the way `include/xsettings.rs` 12-14
  records for the same pair (PORTING.md 0.1-9): the consumer converts at the call site, and
  `src/gtkutil.rs` C 1076/1082 now spell
  `x_display_pixel_width (FRAME_DISPLAY_INFO! (f) as *mut c_void)` (an inline cast; no line was
  added, and these are the tree's only two call sites).
- **The arm readings did not move** (`text-only`, materialised `_root.rs`; A = X11 GTK3, B = pgtk
  GTK3, C = GTK2; the count includes rustc's trailing `error: aborting due to N previous errors`
  as its own span-less row): `gtkutil.c` A **53 -> 53** (19 unit + 33 mirror + 1), B **30 -> 30**
  (16 + 13 + 1), C **66 -> 64** (30 + 33 + 1 - the same two `GtkStyleContext`/`GtkStyleProvider`
  `E0425`s the probe found, gone in the GTK2 arm too); `pgtkterm.c` arm B **19 -> 19** (2 + 16 + 1).
  `pgtkfns.c` was not re-measured (its segment is its own lane's Owed 1).
- `python3 tools/check-mirror-dupes.py` -> `19 duplicate declaration(s), 19 ratchet row(s),
  0 failure(s)`; `python3 tools/check-target-names.py` -> `629 .rs files, 591 map targets,
  0 finding(s)`; `python3 tools/extract-abi.py --check emacs/src/pgtkterm.h
  femacs/src/include/pgtkterm.rs` -> `OK emacs/src/pgtkterm.h -> femacs/src/include/pgtkterm.rs`.

## Owed

1. **`include/gtkutil.rs`'s C 26 edge, and `src/pgtkfns.rs`'s segment** (one round, together):
   `src/pgtkfns.rs` still declares `GdkPixbuf`, `GtkCssProvider`, `GtkWidget`, `GtkWindow`,
   `GdkWindow` and `GTK_WINDOW` in-unit (its segment was landed before the mirror existed, and it
   is outside this round's write scope).  Adding the mirror edge to `include/gtkutil.rs` today puts
   those six declarations and the mirror's into **one** TU -> E0428 x6.  The fix is the pgtkfns
   lane's own P-88 ⑤ move: delete its segment in the commit that adds the `include/gtkutil.rs`
   edge.
2. **The real GDK/GTK values and layouts**: the placeholders (`GDK_NOTHING`, the fourteen
   `GDK_*_MASK`, the seven `GDK_HINT_*`, the ten `GDK_GRAVITY_*`, the fourteen `GdkCursorType`
   enumerators the pgtkfns third slice added, `GTK_WINDOW_TOPLEVEL`/`_POPUP`,
   `GTK_ORIENTATION_*`, `GTK_STATE_NORMAL`, `GTK_STYLE_PROVIDER_PRIORITY_APPLICATION`) and the
   records' `sizeof`/`offsetof` (`GdkGeometry`, `GdkRectangle`, `GdkEvent*`, `GtkAllocation`,
   `GtkRequisition`, `GtkRGBA`-family) are owed to a GTK3 (and, for `GtkStyle`/`GdkColor`/
   `GtkRcStyle`/`GdkColormap`, a GTK2) toolchain.  The missing resource is `docs/goal.md` §9's
   second clause.
3. **`docs/port-map.md` has no row for this file** and none is owed (it is a declared non-port
   artifact, like `include/xlib.rs`); the ledger is the artifact record.
## P-90's blockers inside this mirror: closed (2026-10-05, status 3.481)

The second step measured two blockers *inside this file* and could not touch them; the integrator
wrote both:

* the `GdkEvent` union gains `button`/`scroll`/`motion`/`crossing`/`key`, with the four missing
  records and `GdkEventKey` upgraded from its zero-sized placeholder to the record carrying
  `state`/`keyval` (the pointer readers - `pgtk_im_filter_keypress`, pgtkterm.h 666 - are
  unaffected);
* `GtkWidgetClass` gains `style_updated`, which `src/xfns.rs`'s in-unit copy carries and
  `include/gtkutil.rs`'s C 26 edge needs.

Measured after them: `src/xwidget.rs`'s move lands (56 duplicate declarations deleted, its arm A
51 -> 46 errors, 0 `E0428`), `layout-check` passed, `tools/check-all-units.sh` 83/0 + 83/0.  What
is *not* closed: the moves that still need these names' consumers (`src/xfns.rs`'s three copies,
`include/gtkutil.rs`'s C 26 edge, `include/pgtkterm.rs`'s C 31 edge) - the declarations now exist,
the deletions and the edge landings are the next round's.

### status 3.496 (the mirror round this file took part in)

- The edge/declaration changes are recorded in `docs/status.md` 3.496: `include/dispextern.rs`'s
  `include!("xlib.rs")` edge lost its `#[cfg]` (the `xtask` dedupe gives the first edge the slot),
  `include/xlib.rs` widened `XRectangle`/`Window`/`XrmDatabase`/`XID` to
  `any (have_x_windows, have_pgtk)`, `include/pgtkterm.rs` fixed a dangling `///` (a syntax error
  that made every `have_pgtk` TU unparsable), its `pgtk_im_set_cursor_location` prototype and its
  two bare `atimer` tags, `include/font.rs` gained the `harfbuzz` arm of the `font_driver` size
  assertion (clang-measured 208/224), `include/xterm.rs`'s three INLINE bodies became `unsafe fn`,
  `include/gtk.rs` gained `GtkWidgetClass.get_preferred_height`, and `include/emacsgtkfixed.rs`
  moved `emacs_fixed_new` into a `_decls` module.  The two arm flips that were blocked now read
  **exit 0**: `src/pgtkim.rs` (`have_pgtk` + `have_gtk3`, 14,083,208-byte rlib) and
  `src/hbfont.rs` (`harfbuzz`, 14,580,952-byte rlib); `layout-check` passes and
  `tools/check-all-units.sh` is **83/0 + 83/0**.

### status 3.716 (the pgtkfns third-slice round this file took part in)

The fourteen `GdkCursorType` enumerators above landed here because `src/pgtkfns.c`'s
`INSTALL_CURSOR` (C 1577-1591) names them and the mirror carried only `GDK_LEFT_PTR`; they are
placeholders by the rule this file's head states (no GDK header on this machine), and the consumer's
ledger records the pair (`docs/ports/pgtkfns.md` Deviations 23-24).  The mirror change was verified
by its consumer sweep (`tools/check-all-units.sh`, the P-87 ① leg) as well as by this file's own
`--check` and `check-mirror-dupes.py`.

### status 3.718 (the pgtkfns fifth-slice round this file took part in)

`src/pgtkfns.c`'s eleven `x-*` display subrs (C 1999-2208) are the first consumers of GDK's
*monitor* API in this tree, so `GdkMonitor` and the five `gdk_monitor_*`/`gdk_display_get_*monitor*`
entry points above landed here, and `g_strdup_printf` (C 2014's `g_strdup_printf ("%s.%s", …)`)
joined the GLib group.  No GDK header exists on this machine, so the five GDK rows cite the header
alone and `GdkMonitor` is the never-dereferenced opaque shape; `g_strdup_printf`'s citation is a
machine reading (`/opt/homebrew/include/glib-2.0/glib/gstrfuncs.h` 286).  The mirror change was
verified the way P-87 ① asks: `tools/check-all-units.sh` reads `checked 83 unit(s), 0 failure(s)`
with `checking 83 unit(s), 0 unit failure(s)`, beside this file's own `--check` and
`check-mirror-dupes.py`; the consumer's ledger records the pair (`docs/ports/pgtkfns.md`
Deviations 36-42).

### status 3.719 (the pgtkfns sixth-slice round this file took part in)

`src/pgtkfns.c`'s two scaling monitor walks (C 2329-2403, `x-display-pixel-width`/`-height`) scale
each monitor's rectangle by `pgtk_get_monitor_scale_factor (gdk_monitor_get_model (monitor))` and
fall back to `gdk_monitor_get_scale_factor`, so those two GDK rows landed here (header-only
citations: no GDK header on this machine).  P-87 ①'s consumer sweep reads `checked 83 unit(s), 0
failure(s)` with `checking 83 unit(s), 0 unit failure(s)`; the consumer's ledger records the pair
(`docs/ports/pgtkfns.md` Deviations 43-47 and Evidence ㉔).

### status 3.720 (the pgtkfns seventh-slice round this file took part in)

`src/pgtkfns.c`'s monitor-attribute builder and `pgtk_frame_scale_factor` (C 2405-2531) walk the
frames and monitors through `gdk_display_get_monitor_at_window`, `gdk_monitor_is_primary` and
`gdk_monitor_get_workarea`, so those three rows landed here (header-only citations: no GDK header on
this machine).  P-87 ①'s consumer sweep reads `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`; the consumer's ledger records the pair
(`docs/ports/pgtkfns.md` Deviations 48-52 and Evidence ㉘).

### status 3.721 (the pgtkfns eighth-slice round this file took part in)

`src/pgtkfns.c`'s tooltip frame (C 2746-2755) sets its window up through four `gtk_window_set_*`
calls and one `GdkWindowTypeHint` member, so those rows landed here (header-only citations: no
GTK/GDK header on this machine, and the one named enumerator is a placeholder).  P-87 ①'s consumer
sweep reads `checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s), 0 unit failure(s)`; the
consumer's ledger records the pair (`docs/ports/pgtkfns.md` Deviations 53-57 and Evidence ㉜).

### status 3.722 (the pgtkfns ninth-slice round this file took part in)

`src/pgtkfns.c`'s `compute_tip_xy` (C 2866-2872) queries the pointer through
`gdk_display_get_default_seat`, `gdk_seat_get_pointer` and `gdk_device_get_position`, so those three
rows landed here (header-only citations: no GDK header on this machine) - and they retired two
copies in `src/xwidget.rs`, which had declared the first two itself until `check-mirror-dupes.py`
read `18 duplicate declaration(s), 2 failure(s)` and both were dropped there (P-26).  P-87 ①'s
consumer sweep reads `checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s), 0 unit
failure(s)`; the consumer's ledger records the pair (`docs/ports/pgtkfns.md` Deviations 58-62 and
Evidence ㊱).

### status 3.725 (the pgtkfns eleventh-slice round this file took part in)

`src/pgtkfns.c`'s frame-geometry reader and its two mouse subrs (C 3325-3525) call
`gtk_window_get_position` (gtk/gtkwindow.h), `gdk_window_get_display` (gdk/gdkwindow.h) and
`gdk_device_warp` (gdk/gdkdevice.h), so those three rows landed here (header-only citations: no
GTK/GDK header on this machine).  `check-mirror-dupes.py` immediately read `17 duplicate
declaration(s), 1 failure` - `src/pgtkselect.rs` had declared `gdk_window_get_display` itself at its
C 958 - and that copy is dropped there under P-26's one spelling per construct with its call site
unchanged; that unit's own probe reads **9 errors / 8 warnings** on both sides of the drop, and
P-87 ①'s consumer sweep reads `checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s),
0 unit failure(s)`.  The consumer's ledger records the pair (`docs/ports/pgtkfns.md` Deviations
70-73 and Evidence ㊹-㊻).

### status 3.726 (the pgtkfns twelfth-slice round this file took part in)

`src/pgtkfns.c`'s `pgtk-backend-display-class` (C 3674) reads a display's type name through
`G_OBJECT_TYPE_NAME (G_OBJECT (gdpy))`, so the GObject pieces moved into their single home: this
file gains `GTypeClass` and `GTypeInstance` (the two records `G_TYPE_FROM_INSTANCE!` reads, with
their measured `sizeof 8` and the two offsets asserted), the four macros
`G_TYPE_FROM_CLASS`/`G_TYPE_FROM_INSTANCE`/`G_OBJECT_TYPE`/`G_OBJECT_TYPE_NAME` (gtype.h 646/657 and
gobject.h 98/108, real chains rather than approximations) and `g_type_name` (gtype.h 743).  The rows
are ungated, so `src/gfilenotify.rs` - which includes this mirror (its line 27) - dropped the copies
it carried under `have_gfilenotify` (P-26, call site unchanged) and now reads **exit 0, 0 errors,
0 warnings** on its own probe, the same as before the move.  `check-mirror-dupes.py` stays 16/16/0
and P-87 ①'s consumer sweep reads `checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s),
0 unit failure(s)`.  The consumer's ledger records the pair (`docs/ports/pgtkfns.md` Deviations
74-77 and Evidence ㊽-㊿).

### status 3.727 (the pgtkfns thirteenth-slice round this file took part in)

`src/pgtkfns.c`'s `x-gtk-debug` (C 3728), its `x-gtk-launch-uri` (C 3756/3763) and that subr's
`GError` handling call three GTK entry points, so this file gains
`gtk_window_set_interactive_debugging` (gtk/gtkwindow.h), `gtk_get_current_event_time`
(gtk/gtkmain.h) and `gtk_show_uri_on_window` (gtk/gtkshow.h) - header-only citations, no GTK header
on this machine, and the headers are the API's own (checked against the GTK 3 sources: the show-uri
entry point is `gtk/gtkshow.h`'s rather than `gtk/gtkutils.h`'s).  `check-mirror-dupes.py` stays
16/16/0, no consumer in the include graph had declared them, and P-87 ①'s consumer sweep reads
`checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s), 0 unit failure(s)`.  The consumer's
ledger records the trio (`docs/ports/pgtkfns.md` Deviations 78-81 and the thirteenth slice's
section).

### status 4.333 (the gtkutil third-slice round this file took part in)

`src/gtkutil.c`'s "Dialog functions" (C 2282-2561) is the round's consumer, and P-90 (2) puts its
names here rather than inline in the unit: **seventeen entry points** -
`gtk_widget_unmap`, `gtk_widget_set_sensitive`, `gtk_widget_set_halign`,
`gtk_widget_set_valign`, `gtk_builder_new`, `gtk_buildable_get_internal_child`,
`gtk_dialog_get_action_area`, `gtk_dialog_get_content_area`, `gtk_misc_set_alignment`,
`gtk_button_new_with_label`, `gtk_window_set_modal`, `gtk_box_set_spacing`, `gtk_true`,
`g_main_loop_new`, `g_main_loop_run`, `g_main_loop_quit` and `g_main_loop_unref` - each with its
header in the `/// C:` citation, four of them inside `#[cfg (feature = "gtk_check_version_ge_3_14")]`
/ `not (…)` arms because that is where their C call sites are; **five records** (`GtkDialog`,
`GtkBuilder`, `GtkBuildable`, `GtkMisc` - opaque, `_private: [u8; 0]`, the `GtkBox` shape - and
GLib's `GMainLoop`); **three checked-cast wrappers** (`GTK_DIALOG`, `GTK_MISC`, `GTK_BUILDABLE`);
**`GtkAlign`** with `GTK_ALIGN_START`/`GTK_ALIGN_CENTER` (placeholders: no GTK header on this
machine, `## Owed` 6); and **`GTK_RESPONSE_CANCEL`** (a placeholder too, -6 with that number named
in the comment as this tree's stand-in rather than GTK's).  `gtk_misc_set_alignment`'s two
parameters are Rust's `f32`: GLib's `gfloat` is `typedef float gfloat` and this mirror has no
`gfloat` alias, so the spelling carries the note.

**The unit's own edge.**  C 44's chain also needed `"gtkutil.h"` 27's
`#include "../lwlib/lwlib-widget.h"` (the `widget_value` record every function in the section
takes): `src/gtkutil.rs` now carries `include!("../lwlib/include/lwlib-widget.rs")` at that
position (the `src/pgtkmenu.rs` 95 device), and `check-mirror-dupes.py` stays green because the
record has exactly one mirror.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> 0 failures; P-87 (1)'s consumer sweep ->
`checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s), 0 unit failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object (every item behind `use_gtk`), and
`src/gtkutil.rs`'s own probe reads 29 errors with none anchored in the third slice's text.  The
consumer's ledger records the reading (`docs/ports/gtkutil.md`'s slice-3 section and Owed 17).

**One name moved in and two unit copies out.**  `gtk_dialog_get_content_area` was declared by
`src/pgtkterm.rs` (its C 7048 call) and by `src/xwidget.rs` (C 2693), and by neither's mirror,
because `tools/check-mirror-dupes.py` counts a *unit-vs-mirror* collision: adding the row here
before the copies left meant two findings with no ratchet row.  P-90 (1) ("one surface, one
mirror") decides the direction, so the row stands in this mirror and both unit copies are deleted
in the same commit (each site keeps a comment naming the mirror and the edge that resolves it);
`python3 tools/check-mirror-dupes.py` reads `16 duplicate declaration(s), 16 ratchet row(s),
0 failure(s)` and `tools/check-all-units.sh` still reads `checked 83 unit(s), 0 failure(s)`.

### status 4.334 (the gtkutil fourth-slice round this file took part in)

`src/gtkutil.c`'s "File dialog functions" (C 2562-2843) is the round's consumer: this mirror gains
**fifteen entry points** - `gtk_file_chooser_dialog_new` (C's variadic tail kept: C 2661-2666 pass
four arguments plus `NULL`), `gtk_file_chooser_get_filename`,
`gtk_file_chooser_set_extra_widget`, `gtk_file_chooser_set_current_folder`,
`gtk_file_chooser_set_filename`, `gtk_file_chooser_set_current_name`,
`gtk_file_chooser_set_local_only`, `gtk_check_button_new_with_label`,
`gtk_toggle_button_get_active`, `gtk_toggle_button_set_active`, `gtk_check_version`,
`g_signal_handlers_block_by_func`, `g_signal_handlers_unblock_by_func`, and - under the new
`have_gtk_file_selection_new` feature - `gtk_file_selection_new`,
`gtk_file_selection_set_filename`, `gtk_file_selection_get_filename` and
`gtk_file_selection_hide_fileop_buttons`; **four records** (`GtkFileChooser`, `GtkToggleButton`,
`GtkFileChooserDialog`, and `GtkFileSelection` carrying C 2780's one read member under that
feature); **three checked-cast wrappers** (`GTK_FILE_CHOOSER`, `GTK_TOGGLE_BUTTON`,
`GTK_FILE_SELECTION`); **`GtkFileChooserAction`** with its three action values and
**`GTK_RESPONSE_OK`** - every value a placeholder and every prototype `UNVERIFIED`, as above.

`gtk_misc_set_alignment`'s neighbour note applies here too: the GTK tree would measure the
placeholders, and `GtkFileSelection.selection_entry`'s offset is the one member without one (the
`GtkStyle` `bg`/`fg` class - **no number from memory**, P-88 (2)).

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a
336-byte object; `src/gtkutil.rs`'s arm readings carry one anchor in the fourth slice's text (the
`GParamSpec.name` read).  The consumer's ledger records all of it (`docs/ports/gtkutil.md`'s
slice-4 section).

### status 4.335 (the gtkutil fifth-slice round this file took part in)

`src/gtkutil.c`'s "GTK font chooser" (C 2844-3002) is the round's consumer, and it is the first
round that needs **Pango**: this mirror gains **four records** (`PangoFontFamily`, `PangoFontFace`,
`PangoFontDescription` and GTK3's `GtkFontChooser`, the pointee of the `GTK_FONT_CHOOSER` checked
cast), **the `PangoWeight`/`PangoStyle` typedefs with their fifteen enumerators** (twelve
`PANGO_WEIGHT_*`, three `PANGO_STYLE_*`), **`PangoFontFilterFunc`** (GTK's own callback typedef,
quoted from `gtk/gtkfontchooser.h`) and GLib's **`GDestroyNotify`**, **nine Pango entry points**
(`pango_font_family_get_name`, the seven `pango_font_description_*` calls, `pango_units_to_double`),
and **four `gtk_font_chooser_*` calls** with a `gtk_font_chooser_dialog_new` **forwarding macro**
(C 2923 names that function in both arms, so the port's uniform spelling is the macro form, exactly
as `gtk_box_new`/`gtk_scrollbar_new` above) - plus the **GTK2 font trio**
(`GtkFontSelectionDialog`, `gtk_font_selection_dialog_new` and its `_set_font_name`/
`_get_font_name` pair) that C 110-117's macros in the consumer forward to.

**The Pango rank is the GTK rank, and this is the first file where it shows.**  `/opt/homebrew`
carries `glib-2.0` and `cairo` but neither `gtk-3.0` nor `pango-1.0`, and `pkg-config` is not
installed - so every Pango prototype here is `UNVERIFIED` with no line number, and, unlike the GTK
enumerators the earlier rounds could at least keep distinct by hand, the fifteen Pango values are
**placeholders** whose real numbers a Pango host must supply (`xg_weight_to_symbol`'s chain depends
on their order, so the round records that dependency rather than inventing values - P-88 (2), the
`src/xwidget.rs` 495-520 rule).

`GDestroyNotify` is the exception and is quoted with its line (`gtypes.h` 140), because GLib *is*
on this machine; the same is true of nothing else this round added.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a
336-byte object; `src/gtkutil.rs`'s arm readings carry **no anchor in the fifth slice's text** under
arm A (with `pango_version_ge_1_36_7` on or off), arm C or arm C plus `have_freetype`, and only the
known dedupe artifact under arm B (Owed 17).  The consumer's ledger records all of it
(`docs/ports/gtkutil.md`'s slice-5 section).

### status 4.337 (the gtkutil seventh-slice round this file took part in)

`src/gtkutil.c`'s `xg_create_one_menuitem` and `create_menus` (C 3316-3532) are the round's
consumers: this mirror gains **`GtkMenuItem`** with its **`GTK_MENU_ITEM`** checked cast and **five
entry points** - `gtk_menu_new` (C 3424), `gtk_menu_bar_new` (C 3441, the `#ifndef HAVE_GTK3` arm's
constructor), `gtk_separator_menu_item_new` (C 3498), `gtk_menu_item_set_submenu` (C 3523) and
`gtk_menu_shell_append` (C 3527).  All five are **fresh names**: the four units that carry their own
copies of the menu-item names (`src/pgtkmenu.rs`, `src/xmenu.rs`, `src/xwidget.rs`,
`src/pgtkterm.rs`) declare none of them, so the mirror takes them without an `E0428` - which is
exactly why the last slice's seven had to stay in the unit instead.  Every prototype is
`UNVERIFIED` (no GTK tree on this machine).

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a
336-byte object; `src/gtkutil.rs`'s arm readings carry **no anchor in the seventh slice's text**
under arm A (with or without `gtk_check_version_ge_3_14`), arm C or arm C plus `have_freetype`, and
the slice's out-of-tree probe (nine errors plus rustc's `aborting` line, none of them this slice's -
and the `Emacs_Cursor` `E0428` it found).  The ledger's slice-7 section describes that probe.

### status 4.338 (the gtkutil eighth-slice round this file took part in)

`src/gtkutil.c`'s `xg_get_menu_item_label` (C 3601-3606) is the round's consumer: this mirror gains
**`gtk_label_get_label`** (gtk/gtklabel.h: `const gchar *gtk_label_get_label (GtkLabel *label);`),
the first `gtk_label_*` accessor the tree needs - it carried the `gtk_label_new` constructor alone
until now - and no other unit declares it, so P-90 (2) puts it here.  The prototype is
`UNVERIFIED` (no GTK tree on this machine).

This round's other three functions (`xg_create_widget`, `xg_item_label_same_p`,
`xg_destroy_widgets`) needed **no** new mirror name: `create_dialog` and `xg_set_cursor` are the
unit's own, `create_menus` is the last round's, and the rest of their GTK surface
(`gtk_window_set_transient_for`, `gtk_window_set_destroy_with_parent`, `gtk_window_set_modal`,
`gtk_widget_realize`, the `GTK_WINDOW`/`GTK_WIDGET`/`GTK_LABEL` casts and `g_list_next`) was
already here - which is what the round's arm readings show on the marks.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a
336-byte object; `src/gtkutil.rs`'s arm readings carry **no anchor in the eighth slice's text**
under arm A (with or without `gtk_check_version_ge_3_14`), arm C or arm C plus `have_freetype`, and
the pgtk arm's out-of-tree probe reads 10 `^error` matches with none of them this slice's.

### status 4.339 (the gtkutil ninth-slice round this file took part in)

`src/gtkutil.c`'s `xg_update_menubar` (C 3653-3830) is the round's consumer: this mirror gains
**`gtk_menu_shell_insert`** (gtk/gtkmenushell.h - C 3673/3794/3814's three inserts),
**`gtk_label_set_text`** (gtk/gtklabel.h - C 3763's relabel, the second `gtk_label_*` accessor this
tree needs after status 4.338's `gtk_label_get_label`) and **`g_object_notify`**.  The first two are
fresh names and are `UNVERIFIED`; `g_object_notify` is GLib's, so it quotes `gobject.h` 499-501 and
is the round's *verified* entry - GLib is on this machine.

The function's other 28 call sites needed nothing new: `gtk_container_remove`,
`gtk_container_get_children`, `g_object_ref`, `g_object_unref`, `g_list_free`,
`gtk_menu_item_set_submenu`, `gtk_widget_destroy`, `gtk_menu_item_new_with_label` (the unit's) and
the `GTK_MENU_SHELL`/`GTK_MENU_ITEM`/`GTK_LABEL`/`GTK_CONTAINER`/`GTK_WIDGET` casts were all
already here, which is what the round's zero in-slice arm anchors record.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a
336-byte object; `src/gtkutil.rs`'s arm readings carry **no anchor in the ninth slice's text** under
arm A (with or without `gtk_check_version_ge_3_14`), arm C or arm C plus `have_freetype`, and the
pgtk arm's out-of-tree probe reads 10 `^error` matches with none of them this slice's.

### status 4.340 (the gtkutil tenth-slice round this file took part in)

`src/gtkutil.c`'s `xg_update_menu_item` (C 3837-3945) is the round's consumer: this mirror gains
**`GTK_IS_BOX`** (gtk/gtk.h, C 3859 - kept as this mirror's `GTK_IS_*` family above keeps its four,
i.e. a null test, with the `G_TYPE_CHECK_INSTANCE_TYPE` chain owed) and
**`gtk_widget_get_sensitive`** (gtk/gtkwidget.h, C 3915/3917's sensitivity pair).  Both are fresh
names (`GTK_IS_BOX` and `gtk_widget_get_sensitive` read zero tree-wide before this round) and both
are `UNVERIFIED` (no GTK tree on this machine).

The slice's other call sites needed nothing new - `gtk_container_get_children`,
`gtk_container_remove`, `gtk_container_add`, `g_object_ref`, `g_object_unref`, `g_list_free`,
`g_signal_handler_disconnect` and `gtk_widget_set_sensitive` were already here, and
`gtk_label_get_label` (4.338) with `gtk_label_set_text` and `g_object_notify` (4.339) cover the
rest - which is what the round's zero in-slice arm anchors record.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a
336-byte object; `src/gtkutil.rs`'s arm readings carry **no anchor in the tenth slice's text** under
arm A (with or without `gtk_check_version_ge_3_14`), arm C or arm C plus `have_freetype`, and the
pgtk arm's out-of-tree probe reads 10 `^error` matches with none of them this slice's.

### status 4.341 (the gtkutil eleventh-slice round this file took part in)

`src/gtkutil.c`'s `xg_update_submenu` (C 3975-4094) is the round's consumer: this mirror gains the
three type-check macros **`GTK_IS_SEPARATOR_MENU_ITEM`**, **`GTK_IS_CHECK_MENU_ITEM`** and
**`GTK_IS_MENU_ITEM`** (each kept as the `GTK_IS_*` family above keeps its four: a null test, with
the `G_TYPE_CHECK_INSTANCE_TYPE` chain owed) and **`gtk_menu_item_get_submenu`**
(gtk/gtkmenuitem.h, C 4038).  All four read zero tree-wide before this round, so the mirror can
take them.

**One macro the round could not take, and the family's state after it.**  `GTK_IS_RADIO_MENU_ITEM`
- the fourth the same function asks - is already copied by `src/pgtkmenu.rs` 229-236 (under
`have_pgtk`) and `src/xmenu.rs` 783-786 (under `have_x_windows`), both of which `include!` this
mirror, so a mirror item would be an `E0428` in whichever has its feature on; it stands in the
consumer instead and the reconciliation is Owed 12 of `docs/ports/gtkutil.md` (which also notes
that `src/xmenu.rs`'s body returns `true` while the other two spell the family's
`(! ($o).is_null ())`).

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a
336-byte object; `src/gtkutil.rs`'s arm readings carry **no anchor in the eleventh slice's text**
under arm A (with or without `gtk_check_version_ge_3_14`), arm C or arm C plus `have_freetype`, and
the pgtk arm's out-of-tree probe reads 10 `^error` matches with none of them this slice's.

### status 4.343 (the gtkutil thirteenth-slice round this file took part in)

`src/gtkutil.c`'s `xg_update_frame_menubar` (C 4193-4234) is the round's consumer: this mirror
gains **`gtk_box_reorder_child`** (gtk/gtkbox.h, C 4210 - the reorder that puts the menubar first
in the frame's vbox) and **`gdk_screen_get_system_visual`** (gdk/gdkscreen.h, C 4221 - the visual
of the `HAVE_GTK3` 32-bit fix-up).  Both read zero tree-wide before this round and both are
`UNVERIFIED` (no GTK or GDK tree on this machine).

**One name the round could not take.**  `gtk_widget_get_mapped` (C 4200's first test, and C 4333's
per-child test in the next slice) is already copied by `src/xmenu.rs` 794, `src/pgtkmenu.rs` 243
and `src/xterm.rs` 31215 - all three `include!` this mirror - so a mirror item would be an `E0428`
in each of them; it stands in the consumer, and the same class is already Owed 10 (the menu-item
names) and Owed 12 (`GTK_IS_RADIO_MENU_ITEM`) in `docs/ports/gtkutil.md`.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a
336-byte object; `src/gtkutil.rs`'s arm readings carry **no anchor in the thirteenth slice's text**
under arm A (with or without `gtk_check_version_ge_3_14`), arm C, arm C plus `have_freetype` or the
pgtk probe.

### status 4.344 (the gtkutil fourteenth-slice round this file took part in)

`src/gtkutil.c`'s `xg_event_is_for_menubar` (C 4260-4338) is the round's consumer, and it needed one
*change* to an existing item rather than a new one: **`GTK_IS_MENU_BAR`** (gtk/gtkmenubar.h) was
gated `all (have_gtk3, have_xinput2)` - pinned to this lane's then-only consumer, C 1039-1046's site
inside C 1033's `HAVE_GTK3 && HAVE_XINPUT2` fold.  C 4304 tests a widget that came back from
`gtk_get_event_widget` **outside** any GTK3 fold, so the macro's gate is widened to a `have_gtk3` /
`not (have_gtk3)` pair with the family's own body `(! ($o).is_null ())` in both arms.  No unit
carries its own `GTK_IS_MENU_BAR`, so the widening can collide with nothing (the class Owed 10/12
record does not arise here).

**The round's new name.**  **`gtk_widget_is_ancestor`** (gtk/gtkwidget.h, C 4306's
`gtk_widget_is_ancestor (x->menubar_widget, gwdesc)`) - it read zero tree-wide before this round, and
it is `UNVERIFIED` (no GTK tree on this machine).

**One name the round did not take.**  `gtk_widget_intersect` (C 4333's per-child test) is a name
the tree declares nowhere: `src/xterm.rs` 5619/37551 call it and that unit's own ledger records
the gap (`docs/ports/xterm.md` 8054).  This mirror is therefore its eventual home, but moving it
here would change another unit's name resolution and arm readings in the same round this slice
becomes its first *declarer* - so it stands at the use site for now (Owed 18 of
`docs/ports/gtkutil.md`).

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a
336-byte object; `src/gtkutil.rs`'s arm readings read **28** `^error` matches under arm A and **62**
under arm C - the same multisets the pre-slice tree at `a1c7041b` produces, so the widening adds no
error and, in the arm C face, its own new definition is what C 4304 needs; the pgtk face compiles
none of the consumer's body (C 4258's `#ifndef HAVE_PGTK`), so its reading is the out-of-tree
probe's.

### status 4.345 (the gtkutil fifteenth-slice round this file took part in)

`src/gtkutil.c`'s index-to-widget map (C 4342-4472) is the round's consumer.  The mirror gains
**`GTK_IS_SCROLLBAR`** (gtk/gtkscrollbar.h, C 4444) and **`GTK_IS_EVENT_BOX`** (gtk/gtkeventbox.h,
C 4463) - the two tests the GTK3 arm of `xg_get_widget_from_map` asks; both keep the `GTK_IS_*`
family's body `(! ($o).is_null ())` with their `G_TYPE_CHECK_INSTANCE_TYPE` chains owed (the family
this file's own 809-821 and 2755-2776 carry), both read zero tree-wide before this round, so both are
safe mirror items, and both are `UNVERIFIED` (no GTK tree on this machine).

**Two names the round did not take.**  `gtk_container_forall` (gtk/gtkcontainer.h, C 4464) is
already copied by `src/xwidget.rs` 504-506, which `include!`s this mirror, so a mirror item would be
an `E0428` there; `TYPE_MAXIMUM` (min-max.h, C 4384) is a token macro each unit spells for itself.
Both are Owed 18 of `docs/ports/gtkutil.md`.

**And a name whose home was already here.**  The round's draft declared `xnrealloc` (lisp.h 5546,
C 4388) in its own unit, on the reading that no mirror carries it; `tools/check-mirror-dupes.py`
said otherwise - `17 duplicate declaration(s), 16 ratchet row(s), 1 failure(s)`, "already declared by
`femacs/src/include/lisp.rs` (in the include graph)" - because this file's sibling `include/lisp.rs`
carries it at 3772 inside `lisp_safe_decls` and re-exports it at 3268's `use lisp_safe_decls::*;`.
Deleting the copy brought the gate back to `16/16/0` with every arm reading unchanged, so the
lisp.h mirror's row was in place all along.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a
336-byte object; `src/gtkutil.rs`'s arm readings are **byte-identical multisets to the pre-slice
tree at `b75d7cca`** (arm A 28 `^error` matches, arm A plus `gtk_check_version_ge_3_14` 28, arm A
without `have_xinput2` 27, arm C 62, arm C plus `have_freetype` 62, and the out-of-tree pgtk probe 26
- the pgtk probe is what caught the one spelling this round had to split by arm, C 4384's
`Window`-typed arithmetic), so the round adds no error in any face.

### status 4.346 (the gtkutil sixteenth-slice round this file took part in)

`src/gtkutil.c`'s theme measurement (C 4473-4532) is the round's consumer, and the mirror takes the
two names it could not have taken any other way:

- **`GtkObject`** (gtk/gtktypeutils.h, GTK2's) - C 4480/4504 declare `vadj`/`hadj` as it in the
  `#else` arms of C 4477/4501's `HAVE_GTK3` pair, and GTK2's `gtk_adjustment_new` returns it.  The
  `not (feature = "have_gtk3")` gate is that arm's; the `_private: [u8; 0]` head is this file's
  placeholder shape for a record it cannot lay out (GTK2 is not on this machine), and the name is
  fresh.
- **`gtk_widget_style_get`** (gtk/gtkwidget.h, C 4488/4512) - the variadic style reader whose tail
  carries the property names and their out-parameters.  Fresh, ungated (both GTK faces call it), and
  UNVERIFIED as every GTK prototype here is.

**Six names this round could not take, all for one reason.**  `GtkAdjustment` (gtk/gtkadjustment.h)
is declared by `src/pgtkterm.rs` 5759, a unit that `include!`s this mirror, so a mirror item would be
an `E0428` there; `gtk_adjustment_new`, GTK3's `gtk_scrollbar_new`, GTK2's `gtk_hscrollbar_new` /
`gtk_vscrollbar_new` and the `GTK_ADJUSTMENT` cast macro all name that record, so they stand in the
consumer too.  `g_object_ref_sink` (gobject/gobject.h 509-510) is the GLib instance of the same
class: `src/xsettings.rs` 337-338 declares it and that unit includes this mirror.  All seven rows are
Owed 18 of `docs/ports/gtkutil.md`; the round's `check-mirror-dupes.py` reading (`16 duplicate
declaration(s), 16 ratchet row(s), 0 failure(s)`) is what proves none of them leaked into the mirror
as a copy.

**One family this round completed.**  C 119-124's two GTK2 macros (`gtk_box_new`,
`gtk_scrollbar_new`) had their GTK3 function side here for the first only; this round adds
`gtk_scrollbar_new`'s GTK3 function and its forwarding macro in the consumer, which is the shape this
file's own note at 2566-2572 already described.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings improve
in five of six faces (arm A 28 -> 26, arm A plus `gtk_check_version_ge_3_14` 28 -> 26, arm A without
`have_xinput2` 27 -> 25, the pgtk probe 26 -> 24) and the GTK2 face reads 62 -> 64 with four new
references to Owed 6's untranscribed enumerators.  No other tree is touched by this round.

### status 4.347 (the gtkutil seventeenth-slice round this file took part in)

`src/gtkutil.c`'s scroll-bar id lookup and its two callbacks (C 4533-4594) are the round's
consumers, and this file took three edits:

- **`gtk_get_current_event`** (gtk/gtkmain.h, C 4575) - declared beside `gtk_get_current_event_time`,
  that family's other row.  Fresh name, UNVERIFIED.
- **`GDK_CONFIGURE`** (gdk/gdktypes.h, C 4578) - C's second consumer of a `GdkEventType` member after
  the rows the earlier rounds added, declared with the family's other values and UNVERIFIED like
  them.
- **`GdkEventConfigure` grows GDK's trailing `width`/`height`** (C 4586/4587 write both into a
  synthetic event).  This file's own note had left them out on the rule that it carries "the members
  the reference's text touches"; the first text that touches them is this round's.  **They are also
  the two members `src/xterm.rs` 29935-29938 names as owed** for that unit's own reads of the same
  record (C 18071-18072), so that lane's Owed row can be closed now - this round does not touch the
  file, and says so rather than editing another unit's ledger.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)` (the two fresh names are in no other unit either); `tools/check-all-units.sh` ->
`checked 83 unit(s), 0 failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a 336-byte
object; `src/gtkutil.rs`'s six arm readings are **identical multisets** to the pre-slice tree at
`2b9e5858` (26/26/25/64/64/24) - expected, because none of the slice's three functions has a caller
until C 4882-4900's creator lands.

### status 4.348 (the gtkutil eighteenth-slice round this file took part in)

`src/gtkutil.c`'s `xg_finish_scroll_bar_creation` (C 4595-4687) is the round's consumer, and this
file took one addition: **`GtkUpdateType`** (gtk/gtkenums.h, GTK2's), the type of C 4611's
`GTK_UPDATE_CONTINUOUS`.  It sits with the other GTK2 rows (`GtkOrientation`, `GtkStateType`) and is
the *type* only - the enumerator is deliberately not carried, because its value cannot be measured
here and inventing it is what Owed 6 refuses.  **UNVERIFIED**, like every GTK2 row.

**Six names the consumer keeps.**  `GtkRange` / `GTK_RANGE` / `gtk_range_set_update_policy`
(gtk/gtkrange.h) and `gtk_event_box_new` (gtk/gtkeventbox.h) stand in the unit - the first three
because `src/pgtkterm.rs` 5757 declares the record over this mirror, the last because that unit's
5221 does the same for its own C 3708.  All five rows are Owed 18 of `docs/ports/gtkutil.md`.  The
round also measured the two `E0609`s that the mirrors' `FRAME_OUTPUT_DATA!`/`FRAME_X_OUTPUT!`
expansions produce when their `c_void`-typed member is dereferenced without the consumer's alias
cast - a consumer-side lesson, recorded in that ledger's slice-18 section rather than here.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings move by
owed rows only (arm A 26 -> 27, arm A plus `gtk_check_version_ge_3_14` 26 -> 27, arm A without
`have_xinput2` 25 -> 26, arm C 64 -> 66, arm C plus `have_freetype` 64 -> 66) and the out-of-tree
pgtk probe is **byte-identical at 24**.

### status 4.350 (the gtkutil twentieth-slice round this file took part in)

`src/gtkutil.c`'s `xg_update_scrollbar_pos` (C 4770-4876) is the round's consumer, and this file
gains its two fresh names:

- **`gtk_container_child_get`** (gtk/gtkcontainer.h, C 4804-4805) - the variadic property read of the
  event box's position in its fixed container ("x"/"y" with `gint` out-parameters), declared with the
  variadic tail the style reader above already has.  UNVERIFIED.
- **`gtk_widget_get_size_request`** (gtk/gtkwidget.h, C 4806) - the widget's requested size, the pair
  that C 4836 tests.  UNVERIFIED.

**Three names the consumer keeps.**  `gtk_widget_queue_allocate` (gtk/gtkwidget.h, C 4830),
`XLowerWindow` (Xlib.h 3419, C 4856) and `x_clear_area` (xterm.h 1798, C 4842) are functions whose
home is a mirror but whose names `src/pgtkterm.rs` 4452, `src/xwidget.rs` 6443 and `src/xfns.rs` 2180
already declare over theirs, so the round keeps copies at the use site (Owed 18 of
`docs/ports/gtkutil.md`).  The `pgtk_clear_area` the `#else` arm needs comes from this tree's
`include/pgtkterm.rs` 686 as it should, and Xlib's `True`/`False` pair comes from
`include/xterm.rs` 76/79 - whose note already records that `include/xlib.rs`'s frozen
`x11_constants` module is that pair's eventual home, so this round did not move it.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings move by
owed rows only - arm A 27 -> 28 (and the same +1 with `gtk_check_version_ge_3_14` or the new
`gtk_check_version_ge_3_20`), arm A without `have_xinput2` 26 -> 27, arm C 70 -> 71, arm C plus
`have_freetype` 70 -> 71 - one more `GTK_WIDGET_TO_X_WIN!` instance in each face, while the
out-of-tree pgtk probe stays **byte-identical at 24**.  This round also registered the
`gtk_check_version_ge_3_20` feature in `femacs/xtask/Cargo.toml` (C 4829's condition) and
re-recorded `docs/baseline/feature-coverage.tsv`.

### status 4.354 (the gtkutil twenty-fourth-slice round this file took part in)

`src/gtkutil.c`'s `xg_event_is_for_scrollbar` (C 5112-5195) is the round's consumer and this file
takes its largest batch for a while - seven names, all fresh:

- **`gtk_grab_get_current`** (gtk/gtkmain.h, C 5189) - the widget that holds the grab, the answer
  the function's release/motion arm gives.
- **`GDK_MOTION_NOTIFY`**, **`GDK_BUTTON_PRESS`** and **`GDK_BUTTON_RELEASE`**
  (gdk/gdktypes.h, C 5183/5139/5182) - the `GdkEventType` members the pgtk face of the two arm
  choosers tests.  The family's own comment already named the two button ones as rows owed to
  Owed 21's round, so this round closes that part of it; all three carry GDK's ordinals and the
  family's UNVERIFIED marking.
- **`GdkDeviceManager`** (gdk/gdkdevicemanager.h, the GTK < 3.20 face's record) and the four entry
  points C 5156-5161 uses: **`gdk_display_get_device_manager`**,
  **`gdk_device_manager_get_client_pointer`**, **`gdk_device_get_window_at_position`** and
  **`gdk_display_get_window_at_pointer`**.  The first of those four is also the name
  `src/xterm.rs` 20066 uses and no row anywhere declared (that unit's C 30644 site), so the mirror
  closes an owed name for that lane too - the class slice 17's `GdkEventConfigure` belonged to.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings are
**identical** to the pre-slice tree in seven of the eight sets (29/29/29/29/28/72/72) and the pgtk
probe reads 23 = 23 once its set omits `have_xinput2` - which is the reference's own configuration,
since its pgtk build defines no XInput2 (see that ledger's slice-24 section for the 45-anchor reading
of the combination C cannot compile either).

### status 4.355 (the gtkutil twenty-fifth-slice round this file took part in)

`src/gtkutil.c`'s Printing section (C 5196-5298, under `#ifdef USE_CAIRO`) is this round's consumer,
and it is the largest mirror batch a round has taken: **twenty-two fresh items**.

- **Four records**: `GtkPrintSettings`, `GtkPageSetup`, `GtkPrintOperation` and `GtkPrintContext`,
  each an opaque `_private: [u8; 0]` head (the `GtkPrintSettings`-family shape this file already uses
  for records it cannot lay out - GTK is not on this machine, so no size is asserted).
- **Three enumerations with the needed values**: `GtkPageOrientation` and all four of its labels
  (C 5228-5238's `switch`), `GtkUnit` with `GTK_UNIT_POINTS` (C 5244's macro), and
  `GtkPrintOperationResult`/`GtkPrintOperationAction` with the two the section compares against.
- **Eleven entry points**: `gtk_print_settings_new`, `gtk_print_run_page_setup_dialog`,
  `gtk_page_setup_new`, `gtk_page_setup_get_orientation`, the six `gtk_page_setup_get_*` readers, the
  four `gtk_print_operation_*` setters and `_run`/`_get_print_settings`, and
  `gtk_print_context_get_cairo_context` (whose return type is this file's `cairo_t`, already here).

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings are
**identical multisets to the pre-slice tree in all eight sets** (29/29/29/29/28/72/72/23), which for a
twenty-two-item mirror batch is the reading that matters: none of the new records or prototypes
collides with a name any other unit declares.


### status 4.357 (the gtkutil twenty-seventh-slice round this file took part in)

`src/gtkutil.c`'s tool bar body (C 5381-5464) is this round's consumer, and it asks for **five
items** - the smallest batch a round has taken since the scroll-bar rounds, and the first since
`status 4.355` (slice 26 asked for nothing, which is why this file has no `status 4.356` section).

- **`GTK_IS_IMAGE`** (macro family) - the second of the two cast/check macros the tool bar's item
  boxes are classified with (`GTK_IS_LABEL` is already here), written with the family's body and its
  `G_TYPE_CHECK_INSTANCE_TYPE` chain still owed, like its neighbours.
- **`GDK_ENTER_NOTIFY`** (`= 10`) - the next row of the GDK event-type family the mirror already
  carries; `xg_tool_bar_help_callback`'s C 5415 compares the crossing event's type against it.
- **`GdkEventExpose`** - a fresh record for GTK2's expose event, following `GdkEventAny`'s head
  (`type`, `window`, `send_event`) and then `GdkRectangle area`, which is the member C 5452-5459
  rewrites on all four sides.
- **`GdkDrawable`** - GTK2's abstract drawable, a second opaque `_private: [u8; 0]` head; the one
  call site casts GTK's `GdkWindow` to it (P-72 rule 3), which is why the abstract record is kept
  rather than aliased away.
- **`gdk_drawable_get_size`** - the GTK2 entry point C 5451 calls, gated `not (have_gtk3)` because
  the whole callback that uses it stands inside C 5443's `#ifndef HAVE_GTK3`.

The first four are **fresh names**; `gdk_drawable_get_size` is the only one whose gate the round has
to state, and it is C's own fold rather than a new one.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings are
**identical multisets to the pre-slice tree in all eight sets** (29/29/29/29/28/72/72/23), which is
the reading that matters for a five-item batch: none of the new records or prototypes collides with a
name any other unit declares.

**UNVERIFIED.**  There is no GDK or GTK tree on this machine, so `GdkEventExpose`'s layout, the
`GDK_ENTER_NOTIFY` ordinal and both entry points' signatures are taken from the family's already
recorded shapes and this file's conventions, not from headers on disk - the same boundary `status
4.355` recorded for the GtkPrint* surface.


### status 4.358 (the gtkutil twenty-eighth-slice round this file took part in)

`src/gtkutil.c`'s tool bar packing and creation (C 5465-5559) is this round's consumer, and it asks
for **ten items** - nine fresh ones and one *widening* of a row this file already carried.

- **`GtkOrientable`** and **`GtkToolbar`** - two opaque `_private: [u8; 0]` heads, the first for the
  interface `gtk_orientable_set_orientation` takes (introduced in GTK 2.16, so it stands in both
  faces) and the second for the bar itself (both faces; deprecated in GTK3 but present).
- **`GTK_ORIENTABLE`** and **`GTK_TOOLBAR`** - the cast family's ninth and tenth members, the
  `GTK_CONTAINER` shape (GLib's chain owed, the result type kept).
- **`gtk_orientable_set_orientation`**, **`gtk_toolbar_new`**, **`gtk_toolbar_set_style`** - C 5474,
  C 5546 and C 5550's calls, all in both faces.
- **`GtkToolbarStyle`** with **`GTK_TOOLBAR_ICONS`** - C 5550's second argument; the type is
  `c_int` and the value is a **placeholder** (`0`, GTK's own first row of the enum).  The three
  enumerators after it are not carried, because no C text in this tree names them.
- **`GTK_TEXT_DIR_NONE`** - C 5539 writes it into `xg_frame_tb_info.dir`, a member whose *type*
  (`GtkTextDirection`) this file has carried since slice 1 while the enumerator was missing: the
  same "the type is here, the value is not" gap the `GtkUpdateType` row records.  Value `0`
  (GTK's own first row), **placeholder**.
- **`gtk_style_context_add_class`** (GTK3's gtkstylecontext.h) - C 5557's call, which stands inside
  C's own `#ifdef HAVE_GTK3`, so the row carries that gate.
- **The widening.**  `GTK_ORIENTATION_HORIZONTAL`/`GTK_ORIENTATION_VERTICAL` were declared for the
  GTK3 face alone (slice 1, where C's `gtk_box_new` macro body reached them); C 5474-5477's
  `gtk_orientable_set_orientation` and C 5551-5552's are **not** inside a version fold, and GTK2's
  own `gtkenums.h` has both enumerators, so this round declares them for **both** faces (the two
  values stay the placeholders they were).  Measured effect on the unit's GTK2 arm: 17
  `cannot find value GTK_ORIENTATION_HORIZONTAL` and 5 `... _VERTICAL` instances retire, i.e. that
  face's error total falls from 71 to 53.  The round also *replaced* the doc comment above the type
  to record the second consumer rather than adding a second declaration.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings move
by exactly the three classes the unit's ledger records (`A/A+3.14/A+3.18/A+3.20` 29 -> 33, `AN`
28 -> 32, `C` 72 -> 54, the pgtk probe 1,286 -> 1,326 lines-instances), and no item of this batch
collides with a name any other unit declares.

**UNVERIFIED.**  No GTK tree is on this machine, so all nine fresh items' signatures, the two
placeholders' values and both opaque heads are taken from GTK's documented shapes and this file's
conventions, not from headers on disk - the boundary `status 4.355` recorded for the GtkPrint*
surface.


### status 4.359 (the gtkutil twenty-ninth-slice round this file took part in)

`src/gtkutil.c`'s tool bar item maker (C 5593-5691) is this round's consumer, and it asks for **ten
items** - the tool item, the button, and the two spellings of the `focus-on-click` setter that GTK
splits at 3.20.

- **`GtkToolItem`** with **`gtk_tool_item_new`** and **`gtk_tool_item_set_homogeneous`** - the item
  record (an opaque head; `GtkContainer` receives it at C 5642) and C 5600/5663's two calls.
- **`GtkButton`** with **`GTK_BUTTON`** (the cast family's eleventh member), **`gtk_button_new`** and
  **`gtk_button_set_relief`** - C 5605 and C 5639's calls, both faces.
- **`GtkReliefStyle`** with **`GTK_RELIEF_NONE`** - C 5639's second argument.  Unlike this file's
  other unverifiable enumerators the value is **not** a placeholder: GTK's documented order is
  `GTK_RELIEF_NORMAL`, `GTK_RELIEF_HALF`, `GTK_RELIEF_NONE`, so the one row this tree names is `2`,
  and no C text depends on the other two (the row says so).
- **`gtk_button_set_focus_on_click`** under `not (feature = "gtk_check_version_ge_3_20")` and
  **`gtk_widget_set_focus_on_click`** under the feature itself - the two arms of C 5634's
  `GTK_CHECK_VERSION (3, 20, 0)` fold, each row carrying its own gate so both faces resolve: before
  3.20 the setter is the *button's*, from 3.20 it is the *widget's*.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings move by
exactly one class on the GTK2 face only (`cannot find function gtk_hbox_new`/`gtk_vbox_new`, 7 -> 8
each, from C 5601's `gtk_box_new` call - the C 119-121 row the unit's ledger already carries), while
the four GTK3 faces and `AN` keep their error multisets exactly (33/33/33/33, 32); and no item of this
batch collides with a name any other unit declares.

**UNVERIFIED.**  No GTK tree is on this machine, so all ten rows' signatures are taken from GTK's
documented shapes and this file's conventions, not from headers on disk - the boundary `status 4.355`
recorded for the GtkPrint* surface.


### status 4.360 (the gtkutil thirtieth-slice round this file took part in)

`src/gtkutil.c`'s orientation and reuse tests (C 5692-5810) are this round's consumer, and they ask
for **three items** - the smallest batch since `status 4.357`, and the first in which every one of
them is a *version arm* of the same question.

- **`GTK_IS_VBOX`** and **`GTK_IS_HBOX`** under `not (feature = "have_gtk3")` - the two type-check
  macros C 5706 asks when the build has no `GtkOrientable` query (`is_box_type`'s `#else` arm), kept
  as the `GTK_IS_*` family keeps its own: a null test, with the `G_TYPE_CHECK_INSTANCE_TYPE` chain
  owed.  The records they name (GTK2's `GtkVBox`/`GtkHBox`) stay owed with the two constructors C
  119-121's macro body names (Owed 6) - the gates say why: what this file needs is the *test*, not
  the type.
- **`gtk_orientable_get_orientation`** under `feature = "gtk_check_version_ge_3_20"`'s sibling, the
  plain `have_gtk3` - C 5700's read, inside C's own `#ifdef HAVE_GTK3`.  GTK 2.16's `GtkOrientable`
  interface carries the *setter* (`gtk_orientable_set_orientation`, added in `status 4.358`); the
  *getter* is GTK3's, so this row carries the feature and its two siblings above carry its negation -
  the three together are `is_box_type`'s two arms.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings move by
exactly one retirement and one instance (A family 33 -> 33, AN 32 -> 32, C 56 -> 56: the
`xg_update_tool_bar_sizes` row retires while `no field toolbar_in_hbox` goes 2 -> 3 for C 5771's
read), and no item of this batch collides with a name any other unit declares.

**UNVERIFIED.**  No GTK tree is on this machine, so all three rows are taken from GTK's documented
shapes and this file's conventions, not from headers on disk - the boundary `status 4.355` recorded
for the GtkPrint* surface.


### status 4.361 (the gtkutil thirty-first-slice round this file took part in)

`src/gtkutil.c`'s icon-name resolver (C 5812-5846) is this round's consumer, and it asks for **six
items** - the icon-theme surface, and the two GTK2 rows its stock-id arm needs.

- **`GtkIconTheme`** (an opaque head), **`gtk_icon_theme_get_for_screen`** and
  **`gtk_icon_theme_has_icon`** - C 5906's screen lookup (the next slice's line, in the file already)
  and C 5826/5834's two membership tests, all in both faces.
- **`GtkStockItem`** and **`gtk_stock_lookup`**, both under `not (feature = "have_gtk3")` - C 5818's
  address-only local and C 5831's call, which C itself puts inside `#ifndef HAVE_GTK3`.  The record's
  `modifier` member is C's `GdkModifierType`: that *type* is `src/pgtkterm.rs` 4932's own row (the
  unit `include!`s this mirror, so the name cannot also live here - the `GTK_IS_*` family's reverse
  situation), and since only the record's **address** is ever passed, the field keeps the scalar's
  width with C's type named in the comment.  The whole record is **UNVERIFIED**: GTK2 is not on this
  machine, so its documented five-member layout is quoted, not measured.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings show
**no row from this unit's own text** - the four GTK3 faces and `AN` move by one `E0658 c_variadic`
whose span is `include/lisp.rs` 3828's `verror` declaration, a mirror site that is not reproducible
between invocations on the same input (see the unit ledger's correction), and no item of this batch
collides with a name any other unit declares.

**UNVERIFIED.**  All six rows are taken from GTK's and GTK2's documented shapes and this file's
conventions, not from headers on disk - the boundary `status 4.355` recorded for the GtkPrint*
surface.


### status 4.362 (the gtkutil thirty-second-slice round this file took part in)

`src/gtkutil.c`'s `update_frame_tool_bar` (C 5851-6162) is this round's consumer, and it asks for
**sixteen items** - the largest batch since `status 4.355`, and the first in which the *same* setter
family appears twice under complementary gates.

- **The tool-bar item surface**: `gtk_toolbar_get_nth_item` and `gtk_toolbar_insert` (C 5954/6143's
  reads, C 5965/6131's inserts), `GtkIconSize` (a `c_int` typedef; no enumerator of it is named here)
  with `gtk_toolbar_get_icon_size` (C 6015), and the separator side: `GtkSeparatorToolItem`,
  `GTK_IS_SEPARATOR_TOOL_ITEM` (the cast/check family's seventh member) and
  `gtk_separator_tool_item_new` (C 5959/5964/5973).
- **`gtk_widget_get_direction`** plus the **`GTK_TEXT_DIR_RTL`** enumerator of the
  `GtkTextDirection` family whose type and `NONE` row arrived in `status 4.358`; C 6022 compares it.
  The value is a **placeholder** (`2`, GTK's documented third row).
- **The image constructors**: `gtk_image_new_from_icon_name` (both faces) and, under
  `not (feature = "have_gtk3")`, GTK2's `gtk_image_new_from_stock` - C 6088/6090's two arms.
- **`g_object_set_data_full`** - C 6092/6099's two calls, whose fourth argument is C's
  `(GDestroyNotify) xfree`.  The port names that type in the body (`{ let destroy: GDestroyNotify =
  Some (xfree); destroy }`) rather than passing a bare `Some (xfree)`: `extract-abi` read a
  `does not mention gdestroynotify` row for the bare form, because C's body names the type.
- **The 3.14 pair**, complementary gates again: `gtk_widget_set_margin_start`/`_end`/`_top`/`_bottom`
  under `feature = "gtk_check_version_ge_3_14"` (C 6118-6121) and `gtk_misc_set_padding` under its
  negation (C 6124) - the same shape `status 4.359` used for the two `focus_on_click` setters.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings move by
exactly one retirement (`cannot find function update_frame_tool_bar` - the definition landed), one
widened and one new Owed 11 row (`toolbar_is_packed` 2 -> 3, `tool_bar_resized` entering) and the
mirror's unstable `E0658 c_variadic` row (see the unit ledger's correction); and no item of this batch
collides with a name any other unit declares.

**UNVERIFIED.**  No GTK tree is on this machine, so all sixteen rows' signatures are taken from GTK's
documented shapes and this file's conventions, not from headers on disk - the boundary `status 4.355`
recorded for the GtkPrint* surface.  `GtkIconSize`'s enumerators are not carried at all (no C text
here names one), so the typedef is the whole row.


### status 4.363 (the gtkutil thirty-third-slice round this file took part in)

`src/gtkutil.c`'s teardown, re-position and GTK setup (C 6167-6312) are this round's consumer, and
they ask for **twelve items** - the binding-set surface, the two `G_TYPE_*` pairs the file's first
`GTK_TYPE_*` uses need, two GDK key constants and the two pre-3.16/Xft rows.

- **`GtkBindingSet`** (an opaque head) with **`gtk_binding_set_by_class`** and
  **`gtk_binding_entry_add_signal`** - C 6293-6301's two lookups and two installs.  The installer's
  modifiers parameter is C's `GdkModifierType` spelled as the scalar it is, with C's type named in
  the comment (that *name* is `src/pgtkterm.rs` 4932's row, the `GtkStockItem.modifier` situation).
- **`g_type_class_ref`** plus the two `G_TYPE_*` pairs: `gtk_dialog_get_type` with the
  `GTK_TYPE_DIALOG` macro, and `gtk_menu_shell_get_type` with `GTK_TYPE_MENU_SHELL`.  C writes the
  header macros *without* parentheses (they are object-like), so the port's macro takes `()` and the
  call sites spell `GTK_TYPE_DIALOG! ()` - the name stays in the body, which is what the token check
  reads.
- **`GDK_CONTROL_MASK`** (`1 << 2`) and **`GDK_KEY_g`** (`0x0067`) - C 6294/6300's two key
  arguments; both values are GDK's documented ones, not placeholders.
- **`gtk_settings_set_string_property`** under `not (feature = "gtk_check_version_ge_3_16")` - the
  pre-3.16 arm of C 6282's fold, which C also uses for the menu-bar accelerator at C 6274 (that one
  is inside `#ifndef HAVE_GTK3`).
- **`XftInit`** under `feature = "have_xft"` - C 6255's call, whose argument is the literal `0`;
  libXft is not on this machine.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings move by
five instances of the Owed 11 bit class only (A family 33 -> 38, the new `A+gtk_check_version_ge_3_16`
set 34 -> 39, AN 32 -> 37, C 56 -> 61, pgtk probe 1,467 -> 1,505); and no item of this batch collides
with a name any other unit declares.

**UNVERIFIED.**  No GTK or libXft tree is on this machine, so all twelve rows are taken from GTK's
documented shapes and this file's conventions, not from headers on disk - the boundary `status 4.355`
recorded for the GtkPrint* surface.  The round also registers the ninth version feature the unit
needs (`gtk_check_version_ge_3_16`) in `femacs/xtask/Cargo.toml`.


### status 4.364 (the gtkutil thirty-fourth-slice round this file took part in)

`src/gtkutil.c`'s modifier translators and input-method callbacks (C 6315-6446) are this round's
consumer, and they ask for **eleven items** - the seven remaining GDK modifier bits plus the Pango
preedit surface.

- **The seven GDK modifier bits** C 6325-6354 reads and writes: `GDK_MOD1_MASK` (1 << 3) through
  `GDK_MOD5_MASK` (1 << 7), `GDK_SUPER_MASK` (1 << 26), `GDK_HYPER_MASK` (1 << 27) and
  `GDK_META_MASK` (1 << 28).  `GDK_CONTROL_MASK` (1 << 2) is this file's row already
  (`status 4.363`), so the family is now complete except for the three shift/lock bits no C text here
  names.  The shifts are GDK's documented ones, quoted rather than measured.
- **`PangoAttrList`** (an opaque head), **`gtk_im_context_get_preedit_string`** and
  **`pango_attr_list_unref`** - C 6418's read of the preedit string, its attribute list and the
  cursor, and C 6434's release of that list.  C 2339-2349 already holds the `GtkIMContext` and
  connects the three callbacks, so the getter is the only `gtk_im_context_*` entry point the file had
  left; the record joins the four Pango records the `HAVE_GTK3` block carries.

**Checks.**  `python3 tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings move by
the slice's five retirements (`xg_add_virtual_mods`, `xg_virtual_mods_to_x` and the three
`xg_im_context_*` callbacks - the group the first slice's forward declarations left open) with no new
class, and the pgtk probe reads 1,505 -> 1,520.

**UNVERIFIED.**  Neither GTK nor Pango is on this machine, so all eleven rows are taken from the
documents' shapes and this file's conventions, not from headers on disk - the boundary `status 4.355`
recorded for the GtkPrint* surface.


### Correction (2026-10-15): nine rows this file had taken in belong to their units, not here

The gtkutil lane's slices 27-34 added eleven GDK/Pango names here; an audit of every declaration the
lane added (75 names, compared against every other unit's declaration lines) found that **nine of
them are already declared by a unit that `include!`s this mirror**:

* `GDK_ENTER_NOTIFY` - `src/xwidget.rs` 298;
* `GDK_CONTROL_MASK`, `GDK_MOD1_MASK`, `GDK_SUPER_MASK`, `GDK_HYPER_MASK`, `GDK_META_MASK` -
  `src/xwidget.rs` 306-314 and `src/pgtkterm.rs` 7461-7469;
* `PangoAttrList`, `gtk_im_context_get_preedit_string`, `pango_attr_list_unref` -
  `src/pgtkim.rs` 119-219.

In a GTK build those units would have defined each name twice (`E0428`).  The round therefore
**removed the nine rows from this file** and put them in `src/gtkutil.rs` under that unit's gate;
the four modifier bits no other unit declares (`GDK_MOD2_MASK` through `GDK_MOD5_MASK`) stay here.
Measured: `src/xwidget.rs`'s arm goes 21 -> 15 `E0428` rows (our six -> 0) and `src/pgtkim.rs`'s
5 -> 2 (our three -> 0).  **No gate in the tree read this**: `check-mirror-dupes.py` compares mirror
rows with mirror rows, and the frozen sweep compiles `use_gtk`-gated text out - only a consuming
unit's arm reading sees it (recorded in `docs/ports/gtkutil.md`'s correction and in
`docs/status.md`).


### status 4.365 (the gtkutil thirty-fifth-slice round this file took part in): no new rows

`src/gtkutil.c`'s `xg_widget_key_press_event_cb` (C 6450-6627) is a *consumer* of names this file
could not take, and the round records that instead of adding rows: the four `Is…Key` macros and the
twenty-two `GDK_KEY_*` keysym rows are declared by `src/pgtkterm.rs` 7607-7845 (and, for the macros,
`src/xterm.rs` 31012) over this mirror, `gunichar` is `src/pgtkterm.rs` 4930-4936's - and
`gdk_keyval_to_unicode`, whose signature names that type, could not stand here even alone, because a
mirror is textually included into units that do not define it - and `XNoOp` is Xlib's, which
`src/xterm.rs` 29742/38643 leaves to the recorded bare class.  All of them therefore stand in
`src/gtkutil.rs` (its `## Owed` 18 names them for the round that reconciles the copies).  The keysym
values the round carries are **measured** in `/opt/homebrew/include/X11/keysymdef.h` with the
header's line on each row; `GDK_KEY_dead_degree` is the one name that table lacks, so it keeps the
`gdk_key_dead_degree` feature `status 4.363`'s neighbour introduced.

**Checks.**  `tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet row(s),
0 failure(s)` (unchanged, as it must be: nothing was added); `tools/check-all-units.sh` ->
`checked 83 unit(s), 0 failure(s)`; the frozen `build-tu src/gtkutil.c` stays exit 0 with a 336-byte
object; `src/gtkutil.rs`'s arm readings move by exactly one retirement
(`cannot find value xg_widget_key_press_event_cb`) with no addition.


### status 4.366 (the gtkutil thirty-sixth-slice round this file took part in)

`src/gtkutil.c`'s `xg_filter_key` (C 6629-6727) is this round's consumer, and it is the lane's last
mirror batch: **seven rows**, all fresh names by the declaration audit the collision correction
introduced.

- **`GdkKeymap`** (an opaque head), **`gdk_keymap_get_for_display`** and
  **`gdk_keymap_translate_keyboard_state`** - C 6639's read and C 6683-6688/6706-6711's two calls, in
  both faces.
- **`gdk_x11_keymap_key_is_modifier`** under the round's new `gtk_check_version_ge_3_6` feature
  (registered in `femacs/xtask/Cargo.toml`, citing C 6692) - C 6693-6694's and C 6715-6716's calls.
- **`GdkDeviceType`** with **`GDK_DEVICE_TYPE_MASTER`** (the documented first member) and
  **`gdk_device_manager_list_devices`** - C 6654-6655's read, which C's own `#elif
  GTK_CHECK_VERSION (3, 16, 0)` guards but whose *row* carries no gate (the arm's gate is the unit's).
- **`GDK_SOURCE_KEYBOARD`** (`4`, the fifth member of the `GdkInputSource` family whose sixth,
  `GDK_SOURCE_TOUCHSCREEN = 5`, this file already carried) - C 6662's comparison.

**Everything else this slice needs is a unit-local copy**, for the reasons the previous rounds
recorded: `GdkModifierType`, `gdk_event_new`/`gdk_event_free`, `gtk_im_context_filter_keypress`,
`gdk_event_set_device`/`gdk_seat_get_keyboard` (all declared by `src/xwidget.rs`, `src/pgtkterm.rs` or
`src/pgtkim.rs` over this mirror), `GDK_KEY_PRESS`/`GDK_KEY_RELEASE` (`src/xwidget.rs` 303-304), the
Xlib event-type constants `KeyPress`/`XI_KeyPress` (which `src/xfns.rs`, `src/xmenu.rs` and
`src/xterm.rs` each keep per unit), `XkbGroupForCoreState` (XKB's, and unmeasurable here), and
`XKeyEvent` - whose home is this mirror's `XEvent`, where the 192-byte stand-in at line 1556 still
carries no members (Owed 10/12/14's class).

**Checks.**  `tools/check-mirror-dupes.py` -> `16 duplicate declaration(s), 16 ratchet row(s),
0 failure(s)`; `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)`; the frozen
`build-tu src/gtkutil.c` stays exit 0 with a 336-byte object; `src/gtkutil.rs`'s arm readings move by
the mirror's unstable `E0658 c_variadic` row alone in all eight faces (the two retirements of
`xg_filter_key`/`xg_widget_style_updated` land against no addition), and the pgtk probe's total is
unchanged at 1,505.

**UNVERIFIED.**  Neither GTK nor GDK nor XInput2 is on this machine, so the seven rows are taken from
the documents' shapes and this file's conventions, not from headers on disk - the boundary
`status 4.355` recorded for the GtkPrint* surface.
