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
