# emacs/src/pgtkterm.c (7,780 lines) — slices 1-4: C 1-1078, C 1079-1279, C 1281-1383 and C 1385-1483

> **One of P-86's class: the frozen configuration compiles nothing here.**  `pgtkterm.o` sits in
> `PGTK_OBJ` (`emacs/src/Makefile.in` 325, `PGTK_OBJ=@PGTK_OBJ@`, filled by `configure.ac`
> 3875-3878 - `PGTK_OBJ="pgtkfns.o pgtkterm.o pgtkselect.o pgtkmenu.o pgtkim.o xsettings.o"` under
> `if test "$window_system" = "pgtk"`) and `docs/baseline/frozen-build-sources.txt` lists no
> pgtk object for this target - so the text can never be *compiled* here.  Note the difference from
> the class P-86 was written for: this file's own `#if`s are all *decided*
> (`31 conditions, 0 unknown`), so `extract-abi.py`'s checklist is **not** empty
> (`python3 tools/extract-abi.py emacs/src/pgtkterm.c` reads `## Functions defined — 199`) and the
> `--check` reading in Evidence ③ was never vacuous.  What this ledger adds is the mandate's P-86 ②
> shape - the **C line-range table**, accounting for every line of the reference because the arm
> cannot be compiled on this machine - beside the `## Evidence` arm readings and the
> `## Deviations`.  Port: `femacs/src/pgtkterm.rs` (**2,071 lines**), carrying **C 1-1078**,
> **C 1079-1279**, **C 1281-1383** and **C 1385-1483** - four slices - with **both arms of every
> `#if`**; the **2,044** this line used to
> read was a stale figure (the file measured 1,493 lines at the second slice's HEAD, i.e. the
> number had been wrong for a while - the second slice's section records the measurement, and
> `docs/port-map.md` is the generated authority).  **The next live definition is
> `pgtk_draw_composite_glyph_string_foreground` (C 1487, its comment at C 1485).**
>
> **This ledger replaces, not extends, what it said before.**  Until this batch it read "the frozen
> configuration does not compile this file ⇒ the port is the empty unit plus the note that names the
> gate" (the P-08 reading, `docs/status.md` 3.31), which is what the 11-line placeholder was.  P-86 ①
> replaces that with *transcribe the text, both arms, no stub*; what the earlier note measured (not in
> the frozen source list, empty checklist, 336-byte object) is unchanged and still true here - it is
> the *conclusion* that is withdrawn, exactly as `docs/ports/w32.md` and `docs/ports/w32term.md`
> record for their own files.
>
> **Why 1078 and not 1080.**  C 1078 is the `}` of `pgtk_set_no_accept_focus` (C 1072-1078); the
> neighbouring whole-item boundaries are `pgtk_set_no_focus_on_map`'s `}` at C 1070 (8 short) and
> `pgtk_set_z_group`'s `}` at C 1141 (63 past).  **The next live definition is `pgtk_set_z_group`
> (C 1080-1141)** - the first item of the next lane's slice (C 1079 is its `void` line).
>
> **The file gate is `have_pgtk`** (`HAVE_PGTK`, `configure.ac` 3176's `AC_DEFINE`), on every item
> *and on the include graph* (P-86 ⑧), and the slice's own conditions are C 24's
> `#if 10 <= __GNUC__`, C 68's `#ifdef GDK_WINDOWING_WAYLAND` and C 457's `#ifdef HAVE_XWIDGETS`.
> `python3 tools/check-cond-eval.py --show-unknown src/pgtkterm.c` reads
> **`31 conditions, 0 unknown, 0 disagreements [VERIFIED]`** (`src/pgtkterm.c` **is** on
> `docs/baseline/config/cond-eval-verified.txt` 346), so all three are decided: the pragma block is
> dead text (clang's `__GNUC__` is 4 - carried as C's own text, the `src/window.rs` 14 /
> `src/timefns.rs` 79 reading) and the two `#ifdef`s are false in the frozen configuration and
> become the features `gdk_windowing_wayland` and `have_xwidgets`, **both arms carried**.
>
> **Acquisition: this batch is `text-only` (P-86 ③⑵).**  Measured on this machine:
> `ls -d /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0` → both
> `No such file or directory`; `pkg-config` is **not installed** (`/bin/bash: pkg-config: command
> not found`); `find /opt/homebrew /usr/local /Library -maxdepth 4 -name gtk.h -path "*gtk*"` and
> `-name gdk.h` → nothing.  So there is **no GTK/GDK tree to compile or measure against**: no arm
> flip on a GTK target and no layout reading for any GDK/GTK record.  **cairo and GLib/GObject *are*
> present** (`/opt/homebrew/include/cairo/cairo.h`, cairo 1.18.4; `/opt/homebrew/include/glib-2.0`,
> GLib 2.90.0), so every cairo and GLib declaration in the port quotes the header and line it was
> read from and `GList`'s layout is a real clang reading (Evidence ②) - which is why the toolkit
> layer is *declared* (P-88 ①) rather than left as a bare gap.  The missing toolchain is named in
> `## Owed`.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`PGTK_OBJ` ⇔ `window_system = pgtk`) | `feature = "have_pgtk"` (on every item *and* on C 29-66's include graph) | a pure GTK3 (pgtk) build |
| C 24 `#if 10 <= __GNUC__` (the `#pragma GCC diagnostic`) | no arm: C's text, recorded (clang reports `__GNUC__` 4) | never on this target |
| C 68 `#ifdef GDK_WINDOWING_WAYLAND` / `#include <gdk/gdkwayland.h>` | `all (feature = "have_pgtk", feature = "gdk_windowing_wayland")` | a Wayland GDK build |
| C 457 `#ifdef HAVE_XWIDGETS` / `kill_frame_xwidget_views (f);` | `all (feature = "have_pgtk", feature = "have_xwidgets")` | a pgtk build with xwidgets |
| C 468-469 `#define CLEAR_IF_EQ(FIELD) …` / C 532 `#undef` | C's own text as a function-scoped `macro_rules!`, C's invocation text kept (`highlight_frame`, `im.focused_frame`) | the whole gate |
| C 728's bare `return;`, C 784-785's commented-out `SET_FRAME_VISIBLE`/`SET_FRAME_ICONIFIED` | carried as C's text | the whole gate |

The features this slice **names but the tree has not registered** (`femacs/xtask/Cargo.toml`):
**`gdk_windowing_wayland`** (C 68 - a GDK header macro, not in
`docs/baseline/config/darwin-arm64.macros.tsv`, so `tools/check-cond-eval.py` decides the `#ifdef`
on an absent name as false).  `have_pgtk` (151), `have_window_system` (163), `have_xwidgets` (183),
`use_gtk` (589), `have_gtk3` (132) and `glyph_debug` (83) are registered - owed, below.
  ✅ **Stale for the names that matter here (3.474)**: `use_lucid` (line 626) and
  `use_toolkit_scroll_bars` (line 628) are registered, as are `gdk_windowing_wayland` and
  `have_xwidgets`; `tools/feature-coverage.py` lists all of them with their sites.


## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; PORTING.md 0.1-19 - the port opens with `// femacs: port of src/pgtkterm.c @ fd7afced403.`) |
| 19-22 | the blank, `#include <config.h>`, the blank | carried (`include!("include/config.rs")`) |
| 23-26 | the GCC-bug-102671 comment, `#if 10 <= __GNUC__`, `# pragma GCC diagnostic ignored "-Wanalyzer-null-dereference"`, `#endif` | C's text in a comment; no arm (clang's `__GNUC__` is 4) |
| 27-30 | the blank, `#include <cairo.h>`, `#include <fcntl.h>`, `#include <math.h>` | file gate; cairo declarations in the unit (P-88 ①); the two mirrors |
| 31-31 | `#include <pthread.h>` | third-party header, no file in the reference tree, no mirror; this slice names nothing from it |
| 32-40 | `<sys/types.h>`, `<time.h>`, `<signal.h>`, `<unistd.h>`, `<errno.h>`, `<c-ctype.h>`, `<c-strcase.h>`, `<ftoastr.h>` | file gate; mirrors |
| 41-43 | the blank, `#include <dlfcn.h>`, the blank | third-party header, no mirror; nothing named |
| 44-54 | the eleven `"…"` includes (`"lisp.h"` … `"ccl.h"`) | file gate; mirrors (the five placeholder mirrors are named below) |
| 55-66 | the eleven `"…"` includes (`"termhooks.h"` … `"emacsgtkfixed.h"`) | file gate; mirrors |
| 67-71 | the blank, `#ifdef GDK_WINDOWING_WAYLAND`, `#include <gdk/gdkwayland.h>`, `#endif`, the blank | `gdk_windowing_wayland`; nothing named from it |
| 72-74 | `FRAME_CR_CONTEXT`, `FRAME_CR_ACTIVE_CONTEXT`, `FRAME_CR_SURFACE` | carried as this unit's `macro_rules!` (C 280-281 assigns the second, hence its second arm) |
| 75-109 | the comment + `any_help_event_p`, the comment + `x_display_list`, `struct event_queue_t` + `event_q`, the comment + `ignore_next_mouse_click_timeout`, the comment + `xg_default_icon_file`, the comment + `current_drop_context`, the comment + `current_drop_context_drop`, the comment + `current_drop_time` | file gate; `x_display_list` is the slice's one non-`static` object |
| 110-120 | the seven `static` forward declarations | dropped (PORTING.md 0.1-12); each line recorded in the unit (four of their definitions are outside the slice - Deviations 5) |
| 121-152 | `pgtk_device_added_or_removal_cb`, `pgtk_seat_added_cb`, `pgtk_seat_removed_cb` | file gate; the three are C callbacks (`extern "C"`) |
| 153-218 | `pgtk_enumerate_devices` | file gate |
| 219-243 | `pgtk_free_devices`, `pgtk_regenerate_devices` | file gate |
| 244-285 | `pgtk_get_device_for_event`, the "not a flip context" comment, `flip_cr_context` | file gate |
| 286-355 | `evq_enqueue`, `evq_flush` | file gate; C's `q[nr++]` and the `K & !I` kind reads are Deviations 6/7 |
| 356-403 | `mark_pgtkterm`, `get_keysym_name`, `frame_set_mouse_pixel_position` | file gate; all three non-`static`; the last has C's empty body (kept empty) |
| 404-439 | `pgtk_raise_frame`, `pgtk_lower_frame`, `pgtk_frame_raise_lower` | file gate |
| 440-443 | the comment + `static void pgtk_unlink_touch_points (struct frame *);` | dropped (PORTING.md 0.1-12); C 456's call is kept and its definition is C 6655 (outside the slice) |
| 444-598 | `pgtk_free_frame_resources` | file gate; C 457-459's `HAVE_XWIDGETS` arm (`have_xwidgets`); C 468-469/532's macro; C 525-530's `im.focused_frame` |
| 599-613 | `pgtk_destroy_window` | file gate |
| 614-687 | `pgtk_calc_absolute_position` | file gate; C 638/663's uninitialised `Lisp_Object frame` is zero-initialised (PORTING.md 1.5) |
| 688-748 | `pgtk_set_offset`, `pgtk_set_window_size` | file gate; C 728's bare `return;` kept |
| 749-788 | `pgtk_iconify_frame` | file gate; C 784-785's commented-out calls stay comments |
| 789-835 | the two `…wait_for_event` callbacks, `pgtk_wait_for_event` | file gate; both callbacks are `extern "C"` (their address goes through `G_CALLBACK`) |
| 836-874 | `pgtk_make_frame_visible`, `pgtk_make_frame_invisible`, `pgtk_make_frame_visible_invisible` | file gate |
| 875-936 | `pgtk_new_font` | file gate; C's `static` kept (`docs/ports/w32.md`'s shape: the definition is internal) |
| 937-954 | `pgtk_display_pixel_height`, `pgtk_display_pixel_width` | file gate |
| 955-1071 | `pgtk_set_parent_frame`, `pgtk_set_no_focus_on_map` | file gate; C 1005-1011's seven `FRAME_OUTPUT_DATA` writes |
| 1072-1078 | `pgtk_set_no_accept_focus` | file gate - **this slice's last definition** |
| 1079-1279 | `pgtk_set_z_group` (C 1080-1118), `pgtk_initialize_display_info` (C 1120-1137), the six GC helpers `pgtk_set_cursor_gc`/`pgtk_set_mouse_face_gc`/`pgtk_set_mode_line_face_gc`/`pgtk_set_glyph_string_gc`/`pgtk_set_glyph_string_clipping` (C 1142-1275, five of them `static` in C) | the file gate - **transcribed** (the second slice) |
| 1281-1383 | `pgtk_set_glyph_string_clipping_exactly` (C 1281-1293), `pgtk_compute_glyph_string_overhangs` (C 1295-1333, C's `alloca` going to the port's stack pool), `pgtk_clear_glyph_string_rect` (C 1335-1344), `fill_background_by_face` (C 1346-1377) and `fill_background` (C 1379-1383) | the file gate - **transcribed** (the third slice) |
| 1385-1483 | `pgtk_draw_glyph_string_background` (C 1385-1422), `pgtk_draw_rectangle` (C 1425-1437) and `pgtk_draw_glyph_string_foreground` (C 1440-1483) | the file gate - **transcribed** (the fourth slice) |
| 1485-7780 | **the remainder of the file** - from C 1485's comment and C 1487's `pgtk_draw_composite_glyph_string_foreground` (**the next live definition**) through C 7780's `syms_of_pgtkterm`: the scroll-bar layer, `pgtk_read_socket`, the event handlers, `pgtk_term_init`, `pgtk_delete_display` (C 7314), `syms_of_pgtkterm` (C 7472) and the cairo surface helpers | not in this slice: recorded here so the table covers the reference, to be replaced by the next lane's rows |

## Evidence

**① The frozen configuration (`build-tu`, four runs - one over budget).**
`cd femacs && CARGO_TARGET_DIR=/tmp/pgtk1-lane/cargo cargo run -q -p xtask -- build-tu src/pgtkterm.c`
-> the first run **failed**: `error[E0758]: unterminated block comment` at
`build/femacs/units/pgtkterm/_root.rs:69`, because the port's C 23-26 note quoted C's
`/* Work around GCC bug 102671.` - a `/*` inside a Rust block comment opens a *nested* comment (the
same defect `docs/ports/w32.md` records for its own first run).  The fix (naming the comment instead
of quoting it) is in the port; the three later runs are
-> `built src/pgtkterm.rs -> ../build/femacs/pgtkterm.o`, **exit 0**, `build/femacs/pgtkterm.o` is
**336 bytes** - an empty object, which is the expected reading here and not a proof of the new text:
every item sits behind `have_pgtk`, so under the frozen feature set rustc compiles only
`include/config.rs` and the `#![allow]` attribute.  **Budget note: the mandate allowed three
`build-tu` runs and this batch used four** (the failed one above counts), because the last two edits
(the `pgtk_new_font` linkage and the `FRAME_OUTPUT_DATA` sites the checker caught) came after the
first successful run; `docs/ports/w32.md` reports the same four-for-three for the same reason.

**② The arm readings (three, `text-only`, P-86 ③⑵).**  The `have_pgtk` arm cannot be compiled on a
GTK target here (no GTK tree), so the readings below are the unit's own text compiled with the arm's
cfgs by rustc, on the root `build-tu` materialises (`build/femacs/units/pgtkterm/_root.rs`); the
`xtask probe` subcommand is the same compile with the materialisation step inside it, and one
official `probe` run agrees with them (155 `error[`s, the same classes, on the revision before the
last two fixes).

```
$ rustc --edition 2024 --crate-type=lib --emit=metadata -O build/femacs/units/pgtkterm/_root.rs \
    -o /tmp/pgtk1-lane/narrow.meta --cfg 'feature="have_pgtk"' --cfg 'feature="have_window_system"'
156 error[..]  (147 in this unit's items, 9 anchored in mirrors)

$ ... + --cfg 'feature="gdk_windowing_wayland"' --cfg 'feature="have_xwidgets"'
157 error[..]  (+1 mirror-anchored: include/xwidget.rs 203)

$ ... + --cfg 'feature="use_gtk"' --cfg 'feature="have_gtk3"'
156 error[..]  (identical classes to the narrow set)
```

Classified, **every** error is one of four named things, and **none** is a defect of this slice's own
statement text:

| class | count | anchor |
| --- | --- | --- |
| `cannot find type pgtk_output` | 122 | `include/pgtkterm.rs` (the placeholder mirror: the record is pgtkterm.h 286-455) |
| `cannot find type pgtk_display_info` | 14 | `include/pgtkterm.rs` (pgtkterm.h 146-281) |
| `cannot find type pgtk_device_t` | 7 | `include/pgtkterm.rs` (pgtkterm.h 51-64) |
| `cannot find function pgtk_frame_rehighlight` / `pgtk_new_focus_frame` / `pgtk_set_event_handler` / `pgtk_unlink_touch_points` / `pgtk_fill_rectangle` | 5 | this file, later in it (C 4770 / 4835 / 6831 / 6655 / 5035) - outside the slice; the fifth arrived with the third slice's `pgtk_clear_glyph_string_rect`, which calls the definition at C 5035 |
| `evaluation panicked: size_of::<font> () == 184` (and 7 more offsets in `font`/`font_driver`) | 8 | **`include/font.rs` 143-149/577-583**: those assertions are the *frozen* arm's measurements and the mirror's `#[cfg(feature = "have_window_system")]` members change the record |
| `unstable library feature c_variadic` | 1 | **`include/lisp.rs` 3756**: `verror`'s `core::ffi::VaList<'_>` parameter |
| `dereference of raw pointer is unsafe …` | 1 (only with `have_xwidgets`) | **`include/xwidget.rs` 203**: `XWIDGET_LIVE_P`'s `(*w).buffer` |

Three of those are worth the integrator's attention because they are the **first** time those mirror
arms have ever been compiled (the frozen configuration defines neither `have_window_system` nor
`have_xwidgets`): the `font.rs`/`lisp.rs`/`xwidget.rs` rows are mirror-side items, not this unit's
(`## Owed`).

**③ `extract-abi.py --check`.**  `python3 tools/extract-abi.py --check emacs/src/pgtkterm.c
femacs/src/pgtkterm.rs` -> `FAIL`, **195 `- ` lines**: 165 `missing function`, 12 `missing macro`,
9 `missing constant`, 6 `missing file-scope object` (all of them the unported remainder of the
7780-line file - P-86 ⑩: the check stops being vacuous as soon as the port has a `fn`, and the
comparison is against the file's *whole* inventory, `python3 tools/extract-abi.py
emacs/src/pgtkterm.c` reads `## Functions defined — 199`), plus
`macro CLEAR_IF_EQ does not mention 0, false`, `function pgtk_free_frame_resources does not
mention false` and `line count drifted 73.7%: C 7780 vs Rust 2044`.  The two token lines are the
`do { … } while (false)` that C writes *inside* `pgtk_free_frame_resources` and the port writes as a
function-scoped macro (Deviations 8); they are owed to `docs/ports/macro-body-notes.tsv`.  The
invented-exported-symbol rule **caught a real defect** early in this batch - the port had
`#[unsafe(no_mangle)] pub` on `pgtk_new_font`, which C declares `static` - and the line is gone now.

**④ The self-checks.**  `python3 tools/check-dead-file-coverage.py` ->
`27 ledger(s) bound (a C range table), 0 failure(s)` (this ledger's table is one of them);
`python3 tools/check-declarations.py` -> `14 declaration sites (0 forbidden, 14 allowed literals),
0 failures`, `read 682 files …`; `python3 tools/check-port-lines.py` -> `stale src/pgtkterm.c: no
longer under the threshold` (the ratchet's own state; owed, below).

**⑤ The readings the declarations were copied from** (the port's `/// C:` lines name each):
`/opt/homebrew/include/cairo/cairo.h` 135/164 (cairo_t, cairo_surface_t), 543/546/2120/2489 (the four
entry points); `/opt/homebrew/include/glib-2.0` (GLib 2.90.0) `glib/gtypes.h` 52/54/55/56/60/61/109,
`glib/glist.h` 39-46/53, `glib/gmacros.h` 966-972, `glib/gmain.h` 202/855/950-952,
`gobject/gobject.h` 54/192/513/515/694-695, `gobject/gcloseh` 77/92, `gobject/gsignal.h` 184-206,
225-228, 440-446, 483-492, 554-555, 635-638, `gobject/gtype.h` 425/528/2668-2672.  **Measured**
(every assertion in the port's `GList`/`GList`-adjacent block):

```
$ clang -std=gnu23 -O0 -I /opt/homebrew/include/glib-2.0 -I /opt/homebrew/lib/glib-2.0/include \
    /tmp/pgtk1-lane/glist.c -o /tmp/pgtk1-lane/glist && /tmp/pgtk1-lane/glist
sizeof_GList=24 align=8 data=0 next=8 prev=16
sizeof_gpointer=8 sizeof_gboolean=4 sizeof_guint=4 sizeof_gulong=8 sizeof_guint32=4 sizeof_gchar=1
TRUE=1 FALSE=0
```

`GDK`'s and `GTK`'s records and entry points carry **the header alone, no line**, because no GTK tree
exists here (above); the three records with a layout claim in this slice (`GtkAllocation`, the
`GdkInputSource` enum, `GTK_STYLE_PROVIDER_PRIORITY_USER`) are marked UNVERIFIED in the port and
their assertions are owed.

## Deviations

1. **The licence block is not carried** (C 1-18, P-86 ① / PORTING.md 0.1-19): the port opens with
   `// femacs: port of src/pgtkterm.c @ fd7afced403.`
2. **The toolkit layer is declared in the unit** (P-88 ①): `<cairo.h>` (C 28), `<gtk/gtk.h>`
   (reached through `"gtkutil.h"`, C 48) and `<gdk/gdkwayland.h>` (C 69) have no file in the
   reference tree and no mirror, so their declarations stand in `femacs/src/pgtkterm.rs` rather than
   in a new file under `femacs/src/include/`.  cairo and GLib are read from this machine's headers
   (Evidence ⑤); **GTK and GDK are not on this machine**, so those declarations cite the header
   alone, each record is the never-dereferenced `[u8; 0]` shape (`src/xfns.rs` 1004-1009), and
   `GtkAllocation`'s four-`c_int` layout is **spelled, not measured** (owed).
3. **C's forward declarations are dropped** (C 111-119, C 442-443; PORTING.md 0.1-12): Rust has no
   forward declaration and its items are order-independent, so the definition stays at C's line and
   each declaration line is recorded in the port.  Four calls in this slice resolve through names
   whose definitions are **outside** it - `pgtk_frame_rehighlight` (C 4770), `pgtk_new_focus_frame`
   (C 4835), `pgtk_set_event_handler` (C 6831), `pgtk_unlink_touch_points` (C 6655) - and the calls
   are kept as C's calls (the `src/xfns.rs` 243 reading of the same rule); they are Evidence ②'s
   second class.
4. **The five placeholder mirrors stay placeholders.**  `include/pgtkterm.rs` (the records
   `pgtk_display_info`, `pgtk_output`, `pgtk_device_t` and the `FRAME_*` accessors),
   `include/gtkutil.rs` (the `xg_*` entry points), `include/xwidget.rs`, `include/emacsgtkfixed.rs`
   and `include/pgtkgui.rs`'s GTK pointees carry no body today (their own ledgers say why:
   `docs/ports/include-pgtkterm.md`, "Why the pgtk records are still not here").  This unit therefore
   (a) declares the **functions** it needs at the use site with the reference's own prototype in a
   `/// C:` line (P-72 rule 4: `make_formatted_string`, `mark_object`, `store_frame_param`,
   `fset_parent_frame`, `check_window_system`, `free_frame_faces`, `get_font_ascent_descent`,
   `adjust_frame_size`, `reset_mouse_highlight`, the five `xg_*`, `kill_frame_xwidget_views`) - each
   belongs in the mirror that owns it and **this unit drops its copies in the commit that makes it**
   (the `src/image.rs` 181-201 shape) - and (b) **spells the `pgtkterm.h` accessors it uses**
   (`FRAME_X_OUTPUT`, `FRAME_OUTPUT_DATA`, `FRAME_DISPLAY_INFO`, `FRAME_FONT`, `FRAME_FONTSET`,
   `FRAME_BASELINE_OFFSET`, `FRAME_GTK_WIDGET`, `FRAME_GTK_OUTER_WIDGET`, `FRAME_WIDGET`) with the
   reference's own line, while the **records** those macros deref stay C's tokens and are the named
   gap Evidence ② counts (143 of the 156 errors).  Skipping the accessors would have made the whole
   body unreadable; spelling the records would contradict `docs/ports/include-pgtkterm.md`'s
   measurement (GTK pointees, no probe).
5. **`pgtk_new_font`'s `static` is kept** (C 875-876): `unsafe extern "C" fn` without
   `#[unsafe(no_mangle)]`, because C's address is taken (C 7480's `pgtk_frame_parm_handlers` table)
   but the definition is internal - the `docs/ports/w32.md` shape.  The three callbacks whose address
   goes through `G_CALLBACK` (C 121, 789, 799) are `unsafe extern "C" fn` too (PORTING.md 1.4).
6. **C's post-increment inside a subscript is two statements** (`evq->q[evq->nr++] = *ev`, C 306):
   `*(*evq).q.offset ((*evq).nr as isize) = *ev;` then `(*evq).nr += 1;` - the same effect order (the
   index is the old `nr`), and raw-pointer indexing is C's `q[i]` spelling because Rust cannot index
   a `*mut T` (four sites).
7. **The two `input_event` bitfield reads are this unit's own macro** (C 308/309, 370/371): the
   mirror carries `input_event::bits` with `INPUT_EVENT_KIND_MASK` instead of a `kind` member
   (PORTING.md 1.3 / P-80), so the port spells `input_event_bits! (kind, &raw mut (*ev).ie)` - the
   `src/keyboard.rs` 4078-4085 / `src/androidterm.rs` 200 shape.
8. **`CLEAR_IF_EQ` is a function-scoped `macro_rules!`** (C 468-469): C's `do { … } while (false)`
   wrapper and its `= 0` are what Evidence ③'s two token lines ask about; the port's parameter is a
   token sequence so C's own invocation text survives (`CLEAR_IF_EQ (im.focused_frame)`), and the
   assignment is `std::ptr::null_mut ()` (the P-55 null spelling).  C's `#undef CLEAR_IF_EQ` (C 532)
   has no Rust counterpart - the macro is scoped to the body.
9. **The eight `frame.h` l-value macros this slice writes are two-arm `macro_rules!`**
   (`FRAME_COLUMN_WIDTH`, `FRAME_LINE_HEIGHT`, `FRAME_TAB_BAR_HEIGHT`,
   `FRAME_CONFIG_SCROLL_BAR_COLS`, `FRAME_CONFIG_SCROLL_BAR_LINES`, `FRAME_NO_FOCUS_ON_MAP`,
   `FRAME_NO_ACCEPT_FOCUS`, `FRAME_FONTSET`, `FRAME_FONT`, `FRAME_BASELINE_OFFSET`,
   `FRAME_GTK_OUTER_WIDGET`): `include/frame.rs` carries them as **read-only getters** and C's macro
   is an l-value, so the write arm is C's own member (`src/haikuterm.rs` 636-662's treatment) and the
   invocation is `NAME! (f, v)` instead of C's `NAME (f) = v`.
10. **Implicit conversions the port must spell**: `gpointer` ↔ typed object pointers
    (`g_object_ref (seat as gpointer)`, `G_OBJECT! (…) as gpointer`), `int` → `bool` (C 1023's `0` is
    `false`), `int` → `c_int` (C 644/670's `XFIXNUM` difference narrows), `bool` → `c_int`
    (`SET_FRAME_ICONIFIED (f, false as c_int)`), `*mut frame` ← the mirror's `*mut c_void`
    (`hlinfo->mouse_face_mouse_frame as *mut frame`), and `null` for a `GClosure *` argument.
11. **`G_CALLBACK` keeps its name but not its literal cast** (`gtk/gtk.h`): Rust refuses a
    signature-changing cast between fn pointers (`error: non-primitive cast`, measured), so the body
    is `Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn ()> ($f as *const ())
    })`; `G_OBJECT` keeps its name and its result type but not GLib's checked-cast chain (the
    `src/xfns.rs` 1010-1015 treatment).  `g_clear_object` is spelled as the **declared function**
    (`gobject.h` 694) with the macro's line quoted beside it and the argument converted;
    `g_signal_connect` / `g_signal_handlers_disconnect_by_func` keep GLib's own bodies
    (`gsignal.h` 554-555 / 635-638).
12. **C's uninitialised locals are zero-initialised** (PORTING.md 1.5): C 638/663's `Lisp_Object
    frame`, C 982's `GtkAllocation alloc`, C 814/815's `found`/`timed_out` and C 878-879's
    `font_ascent`/`font_descent`.
13. **`FRAME_PGTK_P` is the mirror's non-pgtk arm** (C 964): `include/frame.rs` 856-866 carries the
    `#ifndef HAVE_PGTK` body (`false`) with no feature gate, so a pgtk build reads `false` here until
    that mirror gains its arm - `docs/ports/include-frame.md` 838 already records the deferral.  It is
    a *pre-existing* mirror gap, not something this slice can fix (a same-name definition in the unit
    would be `E0428`).

## Owed

1. `femacs/xtask/Cargo.toml`: one feature is named by this unit and registered nowhere -
   **`gdk_windowing_wayland`** (C 68).  It is a GDK header macro (not in
   `docs/baseline/config/darwin-arm64.macros.tsv`), so the naming rule is P-51's "the macro
   lower-cased" and the evaluator's `#ifdef`-on-an-absent-name verdict is `false`.
2. `include/pgtkterm.rs`: the five records this unit's 143 name errors are (`pgtk_display_info`,
   `pgtk_output`, `pgtk_device_t`, plus `pgtk_bitmap_record`/`scroll_bar`) and the `FRAME_*` accessors
   the unit currently spells itself (Deviation 4).  **A PGTK target cannot compile this unit until
   they land**; `docs/ports/include-pgtkterm.md` is where the measurement and the reason are already
   written.
3. `include/font.rs` (143-149, 577-583): the eight `font`/`font_driver` layout assertions are the
   **frozen** arm's measurements, and the mirror's `#[cfg(feature = "have_window_system")]` members
   (font.rs 120-127) change both records - so *any* window-system unit that includes the mirror fails
   all eight (`error[E0080]`, measured here for the first time).  Either the assertions move under the
   arm they were measured for or the window-system measurements are added.
4. `include/lisp.rs` 3756: `verror`'s `core::ffi::VaList<'_>` parameter needs the unstable
   `c_variadic` feature (`error[E0658]`), so the `have_window_system` arm cannot compile as it
   stands.  (`...`-style extern declarations such as the port's `make_formatted_string` are fine -
   `src/alloc.rs` 2643 and `src/frame.rs` 4770 do the same.)
5. `include/xwidget.rs` 203: `XWIDGET_LIVE_P` dereferences a raw pointer outside an `unsafe` block,
   so the `have_xwidgets` arm adds one `error[E0133]` (measured).  Another mirror row.
6. `docs/ports/macro-body-notes.tsv`: one row for `CLEAR_IF_EQ` (`0`, `false`) and one for
   `pgtk_free_frame_resources` (`false`), the tokens C's `do { … } while (false)` carries inside the
   function body (Deviation 8).  Neither file is on the porter's writable list (P-86 ⑩'s own lesson
   from `docs/ports/android.md`).
7. `docs/ports/line-drift-tolerance.tsv`: one row, `pgtkterm`, tolerance ≥ 0.74, basis `total` - C
   7780 vs the port's 2044 (Evidence ③).  `tools/check-port-lines.py --write-pending` is the
   integrator's step and drops this file's ratchet row (`stale src/pgtkterm.c: no longer under the
   threshold`).
8. `docs/ports/token-aliases.tsv`: `wchar_t`-style rows are not needed here, but two spellings recur -
   `NULL` → `std::ptr::null_mut ()` and C's `->`-chain through the `output_data` union →
   `(*((*f).output_data.pgtk as *mut pgtk_output)).field` - if the fleet wants them registered.
9. **The GTK toolchain itself**: `GtkAllocation`'s layout, the `GdkInputSource` enum width, the
   `GTK_STYLE_PROVIDER_PRIORITY_USER` value and every GDK/GTK prototype in the port are cited from the
   headers **without a line** and cannot be measured here (no GTK tree; Deviation 2).  A GTK3 build
   host is what closes them.

## Not claimed

- **No behaviour of this file was executed anywhere.**  There is no GTK tree to compile the arm
  against and no PGTK target to run it on: the frozen reading is a 336-byte empty object (Evidence ①)
  and the arm readings are rustc's *name-resolution and type* errors on the unit's own text
  (Evidence ②).  Nothing here is a statement about how Emacs behaves in a pgtk build.
- The 143 name errors are **not** evidence that the transcription is correct; they are evidence that
  the records the mirrors owe are still owed.  Conversely, "no error in this unit's own statements"
  is a *very* weak reading: rustc stops evaluating what it cannot resolve, and P-86 ⑦'s mutation test
  (an injected type error leaving the output byte-identical) is exactly why the arm reading is
  reported as a harness reading and not as a compile.
- `extract-abi --check`'s 195 problem lines include the **whole unported remainder** of the file
  (192 of them); that number is a coverage statement, not a defect list.  The two token lines and the
  drift line are the ones that need baseline rows (Owed 6/7).
- The three mirror-side classes (font.rs's assertions, lisp.rs's `VaList`, xwidget.rs's `unsafe`)
  were found by this batch but are **not this file's to fix**; they are reported so the integrator can
  decide where they belong.
- This slice says nothing about C 1080-7780 (`pgtk_set_z_group` onward): no part of it is ported here,
  and its rows are in the table only so the file is covered.

14. **`s->face` is the mirror's `*mut c_void`, so the six GC helpers cast it.**  `glyph_string.face`
    is `*mut c_void` (`include/dispextern.rs` 1657 - the record it points to is the `face` record at
    2223, gated `dispextern_face_records`), so each helper opens with
    `let face: *mut face = (*s).face as *mut face;` and reads `(*face).background`/`.foreground`/
    `.stipple`, exactly as `src/xterm.rs`'s `x_set_cursor_gc`/`x_set_mouse_face_gc` and
    `src/w32term.rs` 1869 do; `s->face->font` needs `as *mut font` for the same reason (the field
    is `*mut c_void`).
15. **`s->stippled_p = v` is the mirror's bitfield pair.**  C's `bool_bf stippled_p : 1` is
    `glyph_string_bits! (stippled_p, (*s), v)` here (`include/dispextern.rs` 1839's read/write
    macro, PORTING.md 1.3's device for C bitfields) - five sites in `pgtk_set_glyph_string_gc`.
16. **Three conversions C does implicitly are written out.**  `dpyinfo->resx`/`resy` are C's
    `double` fields assigned the integer `96` (C 1126/1127) - the port writes `96.0`; and C 1147's
    `!s->cmp` is a pointer test on the port's `*mut c_void` - `(*s).cmp.is_null ()`.
17. **The arm set gains `dispextern_face_records` with this slice, and four citation comments are
    block comments.**  This is the first slice of this unit to name `struct face`, whose record the
    mirror gates (`include/dispextern.rs` 2223); the flag is the same one
    `docs/ports/w32term.md` 251's arm set carries, and the three arms below are measured with it on
    *both* sides (so the deltas stay comparable).  Separately, a `///` citation above the new
    `extern` blocks reads as an `unused doc comment` (four warnings, measured - the
    `docs/ports/pgtkfns.md` Deviation 2 shape), so all four are `/* */`.

18. **C's `XRectangle` in this file is `pgtkgui.h`'s, not Xlib's.**  `dst->clip[0]` (C 1285-1288)
    and the `XRectangle r[2]` of the first slice take the *pgtk* record - `{ int x, y; unsigned
    width, height; }` (`include/pgtkgui.rs` 139, under `all (have_pgtk, not (have_x_windows))`;
    `include/xlib.rs` 45's `short`/`unsigned short` one is gated `have_x_windows` and is not in
    this file's arms, status 3.533/3.542), so the four writes are a plain `int` copy plus the two
    `int` -> `unsigned` conversions - *not* the `as c_short`/`as c_ushort` an Xlib-typed copy would
    need.  The port still goes through the cast the mirror's degraded `[c_int; 2]` `clip` needs
    (`src/xterm.rs`'s `x_set_glyph_string_clipping_exactly` device), and the first reading of this
    slice named the four sites (`E0308`, 4).
19. **C 1309's `alloca` goes to the port's stack pool.**  `sa_stack_alloc (sizeof (unsigned) *
    s->nchars)` with the `src/alloc.rs` 7948-7957 comment shape (PORTING.md P-05: a bare `alloca`
    site takes the pool directly and accounts nothing against `sa_avail`), and the token `alloca`
    cannot reappear - so the body gets a `docs/ports/macro-body-notes.tsv` row
    (`pgtk_compute_glyph_string_overhangs`, the `Freplace_region_contents` row's reason), and every
    other token of the body is still compared.
20. **Two cairo rows landed in the mirror.**  `cairo_set_source_rgba` (cairo.h 708-710) and
    `cairo_mask` (cairo.h 980) stand in `include/gtk.rs` beside the surface's other cairo rows -
    cairo *is* on this machine, so both carry line citations (`src/ftcrfont.rs` 283-286's
    `cairo_rectangle` row is the same class).  `check-mirror-dupes.py` stays 16/16/0 and the
    consumer sweep reads `checked 83 unit(s), 0 failure(s)`.

21. **The `FACE_*` mask table and `face_bits!` are this unit's own copy.**  C's `ENUM_BF (face_box_type)
    box : 2` / `underline : 3` and the seventeen `bool_bf` members (dispextern.h 1893-1935, stored by
    `include/dispextern.rs` 2256 as one `u32`) are named by masks a unit carries: the
    `src/haikuterm.rs` 300-348 and `src/font.rs` 228-250 copies are the precedent, and this file's
    copy is the same twenty-one rows in C's declaration order with a `face_bits!` covering the two
    members this slice reads (`box`, `overstrike`).  One spelling difference is deliberate: the call
    sites pass **the pointer** (`face_bits! (box, (*s).face as *mut face)`), because a
    `$f:expr` fragment is a value context - `src/xterm.rs` 9673/9722's value form
    (`face_bits! (box, (*((*s).face as *mut face)))`) is that unit's registered `E0614` class
    (`docs/ports/xterm.md` 2931 and its own note at 2942), not a shape this file copies.  Measured:
    the pointer form reads zero `E0614` in all three arms below, the value form read two.
22. **C's `short` -> `int` promotions are written out.**  `struct glyph`'s `pixel_width` is C's
    `short` (dispextern.h 503, the mirror's `c_short`), so `g->pixel_width - 1` and
    `x += g->pixel_width` take `as c_int` (C 1461/1463); and `s->for_overlaps` is C's
    `unsigned for_overlaps : 3` (dispextern.h 1456), not a `bool_bf`, so the read is
    `glyph_string_bits! (for_overlaps, (*s)) != 0` - the `src/haikuterm.rs` 1380 and
    `src/w32term.rs` 2273/2457 spelling, where `src/xterm.rs` 9714/9736's bare form is that unit's
    own registered mismatch (`docs/ports/xterm.md` 2942).
23. **Two cairo rows landed in the mirror, and two copies left `src/image.rs`.**  `cairo_set_line_width`
    (cairo.h 811) and `cairo_stroke` (cairo.h 990) stand in `include/gtk.rs` beside the surface's
    other cairo rows, both with line citations (cairo is on this machine) - and
    `check-mirror-dupes.py` immediately read `18 duplicate declaration(s), 2 failure(s)` because
    `src/image.rs` 467-491 declared both itself under `all (have_window_system, use_cairo)`.  The two
    copies are dropped there under P-26's one spelling per construct with that unit's call sites
    unchanged, and it is measured rather than assumed that this is neutral for it: its own probe
    (`have_ns`, the arm its ledger reads) is **clean, exit 0** on both sides of the drop.  The gate
    returns to 16/16/0 and the consumer sweep reads `checked 83 unit(s), 0 failure(s)` with
    `checking 83 unit(s), 0 unit failure(s)`.

## Owed (2026-10-05, PORTING.md P-90)

- **The SDK surface moves out of the unit.**  `femacs/src/pgtkterm.rs` carries the cairo, GLib,
  GDK and GTK declarations as in-unit segments because it was the surface's first (and, at that
  moment, only) consumer.  With `gtkutil.c` as the second consumer, P-90 requires one dedicated
  mirror - `femacs/src/include/gtk.rs` with its own ledger `docs/ports/include-gtk.md`, the
  `include-xlib.rs` shape - and the in-unit segments moved into it (P-88 ⑤'s move-back).  The
  mirror is not created yet: this file's own slice was landed without it, and the move-back is
  owed to the round that creates it.  Until then the arm's readings carry the declarations twice
  (once per unit), which is harmless across crates and wrong only inside one TU.

## P-90 mirror round (2026-10-05): the surface moved out of this unit

- **`femacs/src/include/gtk.rs` is the third-party surface's single home** (P-90 ①/②/③).  The
  in-unit segment this file carried (C 28's `<cairo.h>` plus C 48's chain: cairo, GLib/GObject,
  GDK, GTK) was moved there verbatim; the missing declarations of the other two consumers went with
  it, and this unit now reaches it through `#[cfg (feature = "have_pgtk")]
  include!("include/gtk.rs");` at C 28's position.  It also gains the edge to the header it *is*
  (`include!("include/pgtkterm.rs")`, at C 48's chain), which supplies `pgtk_output`,
  `pgtk_display_info`, `pgtk_device_t`, `x_display_list` and the pgtk `FRAME_*` family: **the nine
  `FRAME_*` macros C 474-516 owns were deleted from this unit in the same edit** (their two-arm
  spelling is the mirror's now), while the seven `frame.h`-derived accessors stay (C writes them
  here and `include/frame.rs` carries only the getter - Owed 5).
- **Arm reading** (`text-only`, `use_gtk + have_pgtk + have_window_system + have_gtk3 +
  use_cairo + have_xinput2`): **158 errors -> 18** (unit **147 -> 2**).  The 143 record errors are
  gone; the two that remain are this file's own `pgtk_new_focus_frame`/`pgtk_unlink_touch_points`,
  whose definitions sit at C 900+/1400+ and this slice has not reached.  Four unit-side defects the
  mirror exposed were fixed in the same edit: `dpyinfo as gpointer` at the two
  `g_signal_connect` sites, the `last_user_time` (`guint32`) comparisons against `Time`,
  `has_been_visible != 0`, `FRAME_WIDGET! (f) as gpointer`, and the paren slip at C 993/1024
  (`(*(*f).output_data.pgtk as *mut pgtk_output).field` -> `(*((…)))`).
- **This unit's remaining `FRAME_*` copies** are Owed 1 of `docs/ports/include-pgtkterm.md`.
- **The layout round (2026-10-05)** changed only the GTK mirror's own text (P-55 stand-ins and the
  `have_gtk3` gate on `gtk_style_context_remove_provider`) and two inline casts in `src/gtkutil.rs`;
  this unit's arm reading re-measured unchanged: **19** (2 unit + 16 mirror + rustc's `aborting`
  row; `docs/ports/include-pgtkterm.md`'s `## Evidence` records the same pair).

## The second slice (C 1079-1279): the z-group setter, the display-info initializer and the six GC helpers

**What is in it.**  C 1080-1118's `pgtk_set_z_group` (the four `FRAME_Z_GROUP` arms with their
`gtk_window_set_keep_above`/`_below` pairs and C's `error`), C 1120-1137's
`pgtk_initialize_display_info` (the ten field writes and the `reset_mouse_highlight` call), and the
glyph-string GC layer: `pgtk_set_cursor_gc` (C 1142-1175, the merge ladder), `pgtk_set_mouse_face_gc`
(C 1180-1202), `pgtk_set_mode_line_face_gc` (C 1209-1214), `pgtk_set_glyph_string_gc`
(C 1221-1255, C's five-arm `s->hl` dispatch with `emacs_abort` last) and
`pgtk_set_glyph_string_clipping` (C 1261-1275, the `XRectangle r[2]` out-array and the
`cairo_rectangle`/`cairo_clip` pair).  Rust: 1,493 -> **1,744** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` on both sides - Deviation 17).**
```
A narrow = have_pgtk + have_window_system          : HEAD 39 errors / 19 warnings -> 39 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 40 / 20 -> 40 / 20
C = A + use_gtk + have_gtk3                        : 17 / 18 -> 17 / 18
```
**Every arm reads exactly the same number on both sides and no class changes**: the slice's text
adds no error and no warning.  Getting there took four repairs, and each is recorded rather than
absorbed: the `face` casts and the `glyph_string_bits!` device (Deviations 14-15), the two
written-out conversions (Deviation 16), and the four `///` citations that read as `unused doc
comment` warnings (Deviation 17) - the first reading of this text was A 72/23, then 51/23, then
40/23, and the final one is the table above.

**What the slice calls, and what `--check` reads.**  Two GTK rows landed in `include/gtk.rs`
(`gtk_window_set_keep_above`/`_below`, gtk/gtkwindow.h - the mirror is this unit's include graph),
and four declarations stand in the unit (`prepare_face_for_display` dispextern.h 3810,
`get_glyph_string_clip_rects` dispextern.h 3661-3662, `cairo_rectangle` cairo.h 954-956,
`cairo_clip` cairo.h 1033) plus the two-arm `FRAME_Z_GROUP` write macro (frame.h 1262-1267; the
mirror has the reader only - the `src/haikufns.rs` 403-412 shape).
`extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs` goes **193 rows -> 186**: the
seven items retire (`pgtk_set_z_group` and the six helpers/initializer), and the file's own
`line count drifted 80.8%` row is re-measured at **77.6%** (C 7,780 vs the port's 1,744) - the
class this unit carries until the port reaches C's size, and the reason this ledger's range table
(not the checklist) is the coverage evidence (P-86 ②).

**Gates and what is not claimed.**  `check-declarations.py` **18 declaration sites (0 forbidden,
18 allowed literals), 0 failures**; `check-citations.py` **676 file(s) scanned, 0 drift, 0 range,
2045 external, 36 allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound,
0 failure(s)** (**34 range rows** over `src/pgtkterm.c`); `check-port-lines.py` **0 file(s) under ten
percent, 0 reference line(s), 0 ratchet row(s), 0 new, 0 stale**; `check-ledger-ticks.py`
**238 ledger(s) bound, 0 failure(s)**; `check-cond-eval.py src/pgtkterm.c` **31 conditions,
0 unknown, 0 disagreements [VERIFIED]**; `check-mirror-dupes.py` **16 duplicate(s), 16 ratchet
row(s), 0 failure(s)** (after this round's two GTK rows were checked against the units whose include
graph reaches the mirror - none had declared them).  A mirror changed this round, so P-87 ①'s
consumer sweep was run and reads `checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s),
0 unit failure(s)`.  **Not claimed**: nothing here has been *run* - the frozen configuration compiles
none of this file, there is no GTK to link, `pgtk_set_z_group` is called through the frame-parameter
handler table by C that this port cannot build here, and the arm readings are **compile** readings of
text this machine cannot build.

## The third slice (C 1281-1383): the clipping pair, the overhang computation and the background fill

**What is in it.**  `pgtk_set_glyph_string_clipping_exactly` (C 1281-1293: the four `dst->clip[0]`
writes and the `cairo_rectangle`/`cairo_clip` pair), `pgtk_compute_glyph_string_overhangs`
(C 1295-1333: the `CHAR_GLYPH`/`COMPOSITE_GLYPH` fork, the `alloca`'d `code` array and the
`font->driver->text_extents` call, the `composition_gstring_*` pair, and the two overhang write
pairs), `pgtk_clear_glyph_string_rect` (C 1335-1344, C's `pgtk_fill_rectangle` call - the definition
is C 5035, outside this slice), `fill_background_by_face` (C 1346-1377: the cairo operator/clip
block, the four channel folds, the stipple arm's `bitmaps[face->stipple - 1].pattern` and
`cairo_mask`) and `fill_background` (C 1379-1383).  Rust: 1,744 -> **1,891** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` both sides - Deviation 17).**
```
A narrow = have_pgtk + have_window_system          : HEAD 39 errors / 19 warnings -> 40 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 40 / 20 -> 41 / 20
C = A + use_gtk + have_gtk3                        : 17 / 18 -> 18 / 18
```
All three arms read the same **+1**, and it is the class this ledger's Evidence ③ table already
names: `cannot find function pgtk_fill_rectangle`, whose C definition is 5035 - i.e. the call
`pgtk_clear_glyph_string_rect` makes into the part of this file that is still the remainder.  **No
new class and no new warning.**  The first reading of this text was 44/45/22 with four `E0308` sites
(`XRectangle`'s pgtk record, Deviation 18) and one `alloca` token row (Deviation 19); both are
recorded rather than absorbed.

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **186 rows -> 181**: the five items retire (`pgtk_set_glyph_string_clipping_exactly`,
`pgtk_compute_glyph_string_overhangs`, `pgtk_clear_glyph_string_rect`, `fill_background_by_face`,
`fill_background`), the `alloca` body-token row retires against Deviation 19's row, and the file's
own `line count drifted 77.6%` row is re-measured at **75.7%** (C 7,780 vs the port's 1,891) - the
class this unit carries until the port reaches C's size.

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2045/36/0**; `check-dead-file-coverage.py` **145/0** (**35 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0**; P-87 ①'s
consumer sweep reads `checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s), 0 unit
failure(s)`.  **Not claimed**: none of this has been run - the frozen configuration compiles none of
this file, there is no GTK or cairo drawing to link here, and the three arm readings are **compile**
readings of text this machine cannot build.

## The fourth slice (C 1385-1483): the background, the rectangle and the string foreground

**What is in it.**  `pgtk_draw_glyph_string_background` (C 1385-1422: the
`background_filled_p` guard, the `max (box_horizontal_line_width, 0)` fold, the stipple arm's
`fill_background` and the four-condition `FONT_HEIGHT`/`FONT_TOO_HIGH`/`font_not_found_p`/
`extends_to_end_of_line_p`/`force_p` arm with `pgtk_clear_glyph_string_rect`),
`pgtk_draw_rectangle` (C 1425-1437: `pgtk_begin_cr_clip` through
`pgtk_set_cr_source_with_color`, the `x + 0.5` coordinates, `cairo_set_line_width (cr, 1)` and
`cairo_stroke`) and `pgtk_draw_glyph_string_foreground` (C 1440-1483: the `box`/
`left_box_line_p` offset, the `font_not_found_p` rectangle arm, and the `font->driver->draw` trio
with `VCENTER_BASELINE_OFFSET` and `overstrike`).  Rust: 1,891 -> **2,071** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` both sides - Deviation 17).**
```
A narrow = have_pgtk + have_window_system          : HEAD 40 errors / 19 warnings -> 40 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 41 / 20 -> 41 / 20
C = A + use_gtk + have_gtk3                        : 18 / 18 -> 18 / 18
```
**Every arm reads exactly the same number on both sides and no class changes**: the slice's text
adds no error and no warning.  Three repairs got it there, each recorded: the unit's own
`FACE_*`/`face_bits!` block with the **pointer** call form (Deviation 21 - the first reading used
xterm.rs's value form and read two `E0614`s), C's two `short` -> `int` promotions and
`for_overlaps != 0` (Deviation 22 - the first reading read one mismatch), and two cairo rows in the
mirror (Deviation 23).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **181 rows -> 178**: `pgtk_draw_glyph_string_background`, `pgtk_draw_rectangle` and
`pgtk_draw_glyph_string_foreground` retire, and the file's own `line count drifted` row is
re-measured 75.7% -> **73.4%** (C 7,780 vs the port's 2,071) - the class this unit carries until the
port reaches C's size.

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2047/36/0**; `check-dead-file-coverage.py` **145/0** (**36 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0**; P-87 ①'s
consumer sweep reads `checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s), 0 unit
failure(s)`.  **Not claimed**: none of this has been run - the frozen configuration compiles none of
this file, there is no GTK or cairo to draw with here, and the three arm readings are **compile**
readings of text this machine cannot build.
