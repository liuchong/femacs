# emacs/src/pgtkterm.c (7,780 lines) — slices 1-27: C 1-1078, C 1079-1279, C 1281-1383, C 1385-1483, C 1485-1572, C 1575-1662, C 1664-1804, C 1806-1951, C 1953-2089, C 2091-2190, C 2192-2292, C 2294-2359, C 2361-2455, C 2457-2522, C 2524-2825, C 2827-2914, C 2916-3012, C 3014-3110, C 3112-3255, C 3257-3379, C 3381-3465, C 3467-3567, C 3569-3645, C 3647-3752, C 3754-3798, C 3800-3922 and C 3924-3970

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
> `## Deviations`.  Port: `femacs/src/pgtkterm.rs` (**5,684 lines**), carrying **C 1-1078**,
> **C 1079-1279**, **C 1281-1383**, **C 1385-1483**, **C 1485-1572**, **C 1575-1662**,
> **C 1664-1804**, **C 1806-1951**, **C 1953-2089**, **C 2091-2190**, **C 2192-2292**,
> **C 2294-2359**, **C 2361-2455**, **C 2457-2522**, **C 2524-2825**, **C 2827-2914**,
> **C 2916-3012**, **C 3014-3110**, **C 3112-3255**, **C 3257-3379**, **C 3381-3465**, **C 3467-3567**, **C 3569-3645**, **C 3647-3752**, **C 3754-3798**, **C 3800-3922** and **C 3924-3970** - twenty-seven
> slices - with **both arms of every `#if`**; the **2,044** this line used to
> read was a stale figure (the file measured 1,493 lines at the second slice's HEAD, i.e. the
> number had been wrong for a while - the second slice's section records the measurement, and
> `docs/port-map.md` is the generated authority).  **The next live definition is
> `pgtk_send_scroll_bar_event` (C 3977-3979; C 3975's `window_being_scrolled` object and its comment
> are above it).**
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
| 1485-1572 | `pgtk_draw_composite_glyph_string_foreground` (C 1485-1572: the box-line offset, the `font_not_found_p` rectangle arm, the non-automatic `COMPOSITION_GLYPH`/`offsets` loop and the `LGSTRING_GLYPH`/`LGLYPH_*` automatic arm with its two deferred draws) | the file gate - **transcribed** (the fifth slice) |
| 1575-1662 | `pgtk_draw_glyphless_glyph_string_foreground` (C 1575-1662: the box-line offset, the `char2b[8]` local C hands to `s->char2b`, the `GCC_LINT` pair, the acronym and hex-code arms of `glyph->u.glyphless`, the two `font->driver` calls with the slice offsets, the `THIN_SPACE` rectangle guard and the closing `s->char2b = NULL`) | the file gate - **transcribed** (the sixth slice) |
| 1664-1804 | `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT` (C 1664-1675), `pgtk_compute_lighter_color` (C 1684-1746), `pgtk_fill_trapezoid_for_relief` (C 1748-1762), `enum corners` (C 1764-1771) and `pgtk_erase_corners_for_relief` (C 1773-1804) | the file gate - **transcribed** (the seventh slice) |
| 1806-1951 | `pgtk_setup_relief_color` (C 1806-1822), `pgtk_setup_relief_colors` (C 1824-1853), `pgtk_set_clip_rectangles` (C 1855-1866) and `pgtk_draw_relief_rect` (C 1868-1951) | the file gate - **transcribed** (the eighth slice) |
| 1953-2089 | `pgtk_draw_box_rect` (C 1960-2000), `pgtk_draw_glyph_string_box` (C 2005-2048) and `pgtk_draw_horizontal_wave` (C 2050-2089) | the file gate - **transcribed** (the ninth slice) |
| 2091-2190 | `pgtk_draw_underwave` (C 2091-2098) and `pgtk_draw_image_relief` (C 2102-2190) | the file gate - **transcribed** (the tenth slice) |
| 2192-2292 | `pgtk_draw_glyph_string_bg_rect` (C 2195-2203), `pgtk_cr_draw_image` (C 2205-2239) and `pgtk_draw_image_foreground` (C 2243-2292) | the file gate - **transcribed** (the eleventh slice) |
| 2294-2359 | `pgtk_draw_image_glyph_string` (C 2308-2359, with C 2294-2306's ASCII-art comment) | the file gate - **transcribed** (the twelfth slice) |
| 2361-2455 | `pgtk_draw_stretch_glyph_string` (C 2363-2455) | the file gate - **transcribed** (the thirteenth slice) |
| 2457-2522 | `pgtk_draw_dash` (C 2462-2481) and `pgtk_fill_underline` (C 2487-2522), with C 2458-2460/2483-2485's comments | the file gate - **transcribed** (the fourteenth slice) |
| 2524-2825 | `pgtk_draw_glyph_string` (C 2524-2825: the successor-background loop, the relief/clipping prologue, the six-arm `match` over the glyph types and the underline/overline/strike-through epilogue) | the file gate - **transcribed** (the fifteenth slice) |
| 2827-2914 | `pgtk_define_frame_cursor` (C 2829-2836), `pgtk_after_update_window_line` (C 2838-2867), `pgtk_clear_frame_area` (C 2869-2873) and `pgtk_draw_hollow_cursor` (C 2877-2914) | the file gate - **transcribed** (the sixteenth slice) |
| 2916-3012 | `pgtk_draw_bar_cursor` (C 2923-3012: the glyph lookup, the xwidget/image early exits, the image arm's `draw_phys_cursor_glyph` and the bar/hbar arms with their two `pgtk_fill_rectangle` calls) | the file gate - **transcribed** (the seventeenth slice) |
| 3014-3110 | `pgtk_draw_window_cursor` (C 3016-3077: the `on_p` arm, the fringe early path, the five-arm `match` over `text_cursor_kinds` and the IM cursor-location call) and `pgtk_copy_bits` (C 3079-3110: the similar surface, the source-surface pair, the two paints and the destroy) | the file gate - **transcribed** (the eighteenth slice) |
| 3112-3255 | `pgtk_scroll_run` (C 3114-3255: the `window_box` bounding box, the up/down branches, the `HAVE_XWIDGETS` block, `gui_clear_cursor` and the two `cairo_rectangle_t` records into `pgtk_copy_bits`) | the file gate - **transcribed** (the nineteenth slice) |
| 3257-3379 | `pgtk_bitmap_icon` (C 3259-3326, whose body is C 3267-3324's `#if 0` carried as C's own text), `pgtk_text_icon` (C 3332-3342), `pgtk_update_begin` (C 3354-3358) and `pgtk_draw_vertical_window_border` (C 3362-3379) | the file gate - **transcribed** (the twentieth slice) |
| 3381-3465 | `pgtk_draw_window_divider` (C 3383-3442), `pgtk_update_end` (C 3447-3452) and `pgtk_frame_up_to_date` (C 3454-3465) | the file gate - **transcribed** (the twenty-first slice) |
| 3467-3567 | `pgtk_mouse_position` (C 3487-3567, with C 3467-3485's comment above it: the frame loop that clears `mouse_moved`, the grabbed branch, the else branch's seat/device query and `pgtk_any_window_to_frame`, the terminal-frame early return and the `remember_mouse_glyph` pair) | the file gate - **transcribed** (the twenty-second slice) |
| 3569-3645 | `max_fringe_bmp` and `fringe_bmp` (C 3571-3572), `pgtk_define_fringe_bitmap` (C 3574-3611), `pgtk_destroy_fringe_bitmap` (C 3613-3626) and `pgtk_clip_to_row` (C 3628-3645) | the file gate - **transcribed** (the twenty-third slice) |
| 3647-3752 | `pgtk_draw_fringe_bitmap` (C 3647-3689), `hourglass_atimer`/`hourglass_enter_count` (C 3691-3692), `hourglass_cb` (C 3694-3698), `pgtk_show_hourglass` (C 3700-3727), `pgtk_hide_hourglass` (C 3729-3746) and `pgtk_flush_display` (C 3748-3752) | the file gate - **transcribed** (the twenty-fourth slice) |
| 3754-3798 | C 3754's `extern frame_parm_handler pgtk_frame_parm_handlers[];` declaration, `pgtk_redisplay_interface` (C 3756-3785, the 28-entry interface object) and `pgtk_clear_frame` (C 3787-3798) | the file gate - **transcribed** (the twenty-fifth slice) |
| 3800-3922 | `recover_from_visible_bell` (C 3800-3813), `pgtk_flash` (C 3815-3905) and `pgtk_ring_bell` (C 3907-3922) | the file gate - **transcribed** (the twenty-sixth slice) |
| 3924-3970 | `pgtk_read_socket` (C 3924-3970, with C 3924-3930's comment above it) | the file gate - **transcribed** (the twenty-seventh slice) |
| 3972-7780 | **the remainder of the file** - from C 3972-3974's comment, C 3975's `window_being_scrolled` and C 3977's `pgtk_send_scroll_bar_event` (**the next live definition**) through C 7780's `syms_of_pgtkterm`: the fringe bitmaps, the scroll-bar layer, `pgtk_read_socket`, the event handlers, `pgtk_term_init`, `pgtk_delete_display` (C 7314), `syms_of_pgtkterm` (C 7472) and the cairo surface helpers | not in this slice: recorded here so the table covers the reference, to be replaced by the next lane's rows |

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
| `cannot find function pgtk_any_window_to_frame` | 1 | this file, later in it (C 7314's definition, forward-declared at C 118) - the name this ledger's twenty-second slice reaches first (C 3526, inside `pgtk_mouse_position`) |
| `cannot find function pgtk_clip_to_row` | **retired** | the class the sixteenth slice reached first (`pgtk_draw_hollow_cursor`, C 2910) and the seventeenth slice grew to two sites (`pgtk_draw_bar_cursor`); the **twenty-third slice** transcribes its C 3628-3645 definition, and with it both sites leave this table |
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

24. **`glyph_u_cmp_bits!` is this unit's own copy, and one index takes a cast.**  C 1513's
    `s->first_glyph->u.cmp.automatic` is the `bool_bf automatic : 1; unsigned id : 31;` pair C
    declares at dispextern.h 597-598, whose constants are `include/dispextern.rs` 454-455's and
    whose macro the units carry themselves (`src/xterm.rs` 9596, `src/w32term.rs` 1732, `src/term.rs`
    840): this file's copy has the two arms and, like Deviation 21's `face_bits!`, its call sites
    pass the **pointer** (`glyph_u_cmp_bits! (automatic, (*s).first_glyph)`) rather than the value
    form those units' registered `E0614` class records.  C 1539's `LGSTRING_GLYPH (gstring, i)`
    takes the mirror's `ptrdiff_t` index, so the port writes `i as ptrdiff_t` (C's `int` -> the
    `ASIZE`-derived index type; `src/character.rs` 430's `j as isize` is the same conversion) - the
    first reading of this slice named that one site.

25. **Three per-unit copies and one cast.**  `glyphless_display_method` with
    `GLYPHLESS_DISPLAY_THIN_SPACE`/`_EMPTY_BOX`/`_ACRONYM`/`_HEX_CODE` (dispextern.h 2347-2365) and
    the four-arm `glyphless_bits!` macro (C 590-638's `u.glyphless` members, whose masks are
    `include/dispextern.rs` 478-481's) are this unit's own copies - the names live inside that
    mirror's unit-names module, which a `use` cannot reach, and the `src/term.rs` 288-292 /
    `src/xdisp.rs` 1188-1191 and `src/xterm.rs` 9596-class precedents carry them the same way; the
    macro's call sites pass the **pointer** (Deviations 21/24's reason).  C 1595-1601's
    `#ifdef GCC_LINT` pair is two `const PACIFY_GCC_BUG_81401` arms under `gcc_lint` / `not
    gcc_lint` (the frozen table defines `GCC_LINT`, conf_post.h 486, so the first arm is the
    compiled text - the `src/xterm.rs` 9910-9913 shape).  And C 1616's
    `CHAR_TABLE_REF (Vglyphless_char_display, glyph->u.glyphless.ch)` takes the mirror's `isize`
    index, so the port writes `glyphless_bits! (ch, glyph) as isize` - the first reading of this
    slice named that one site.

26. **C's `for` head is the port's `while`, and C's two comma expressions are one statement per
    assignment.**  C 1783's `for (i = 0; i < CORNER_LAST; i++)` keeps C's `int i` declaration, the
    init and C's own step at the end of the body (`src/xterm.rs` 7290-7321 spells the same C text
    with a `for … in 0..` head, which drops C's local - a spelling this unit does not follow because
    the declaration is part of the port's name set); C 1789/1791/1793/1795's
    `xm = x - margin, xc = xm + radius;` is the comma operator, so the port puts the two stores on
    their own lines in C's left-to-right order (the `src/xterm.rs` 7275-7276 note's class).  C's
    `x + width + margin` stays C's **int** addition, widened where C widens it
    (`(x + width) as c_double + margin`): C's usual arithmetic conversions evaluate the inner int sum
    first, so the sibling unit's spelling is the faithful one and the C-exact reading was measured
    against C's text before it was copied.
27. **Five declarations this slice needs, and the one mirror it changes.**  `M_PI_2` (math.h 752,
    `1.57079632679489661923132169163975144`) lands in the `femacs/lib/include/math.rs` mirror - C
    1798's `cairo_arc` angles are C's `i * M_PI_2` after the int -> `double` promotion, so the port
    writes `i as c_double * M_PI_2` and `(i + 1) as c_double * M_PI_2` (the `src/xterm.rs` 7320
    spelling of the same widening, which had no declaration to reach: that lane's `use_cairo` arm is
    never compiled, so the constant is what its own text was missing).  A mirror changed, so P-87's
    two legs were run: `check-mirror-dupes.py` **16/16/0** (the new row duplicates nothing) and the
    consumer sweep `checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s), 0 unit failure(s)`.
    The four cairo entry points (`cairo_move_to` cairo.h 906-907, `cairo_line_to` 912-913,
    `cairo_arc` 921-925, `cairo_fill` 995-996) are declared **in this unit**, each in its own
    `unsafe extern "C"` block with the header line it was read from, beside the C 1271/1273 pair at
    1519-1530: this unit's own P-88 ① toolkit segment is where `cairo_rectangle`/`cairo_clip` already
    stand, and `src/image.rs` 470-473 / `src/ftcrfont.rs` 287 carry their own rows for the same
    construct.  `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT` is this unit's own `pub const: c_int = 48000`
    under the file gate (the `src/xterm.rs` 10773 shape; the constant is not in a mirror, so this
    unit's copy is not a duplicate - it is C's own `#define` at C 1675).

28. **`pgtk_output`'s three `bool_bf` members are stored now (the mirror's first consumer).**  C 394's
    `toolbar_in_hbox`, C 395's `toolbar_is_packed` and C 440's `relief_background_valid_p` were
    *recorded, not stored* in `femacs/src/include/pgtkterm.rs` (P-80's member dispatch) until this
    slice's `pgtk_setup_relief_colors` needed C 1843-1847: C reads the flag in a condition and writes
    it in the body, and a record's field cannot be declared at a use site, so the record gained the
    P-80 ① storage unit (`pub bits: c_uint`) plus the member-name-dispatched accessor
    (`pgtk_output_bits! (relief_background_valid_p, di[, v])`, and the two `toolbar_*` names with it),
    the `glyph_bits!`/`glyph_row_bits!`/`glyph_matrix_bits!` shape.  The unit's offset and the three
    masks are the port's storage assignment and are **not** a layout claim: no GTK toolchain here can
    measure the record (P-88 ②, which is why the mirror already labels the whole record
    `UNVERIFIED`).  A mirror changed, so P-87's two legs were run: `check-mirror-dupes.py`
    **16/16/0** and the consumer sweep `checked 83 unit(s), 0 failure(s)` with
    `checking 83 unit(s), 0 unit failure(s)`; the mirror's own ledger
    (`docs/ports/include-pgtkterm.md`) carries the round paragraph.
29. **Four declarations this slice needs, and one accessor arm.**  `face_bits!` gains the
    `use_box_color_for_shadows_p` arm (C 1865's member; `FACE_USE_BOX_COLOR_FOR_SHADOWS_P` was already
    in the unit's own table as `1 << 5`, so the arm is the only addition - the `src/xterm.rs`
    9575-9589 shape); `IMAGE_BACKGROUND` and `IMAGE_BACKGROUND_TRANSPARENT` (dispextern.h 3860-3872)
    are this unit's own copies of the pair the mirror does not carry, with their two prototypes
    (`image_background`, `image_background_transparent`, dispextern.h 3769-3772, defined in
    `src/image.rs`) declared at the use site - the `src/w32term.rs` 1781-1806 / `src/image.rs`
    2241-2260 precedent; and `RGB_PIXEL_COLOR` (dispextern.h 3760-3762's `unsigned long` arm, the one
    this configuration takes) is this unit's own `pub type`, the `src/image.rs` 2210-2215 copy.
    None of the four is a new mirror: the unit-local declaration is where the corpus puts them.
30. **The arm set gains `use_cairo`, and the dispextern mirror's `have_pgtk` exclusion is lifted.**
    C 1835/1836's third argument is an `Emacs_Pix_Context`, and that type comes from C 122-132's
    `#ifdef USE_CAIRO` block for a pgtk build (C 149-155's pgtk segment defines `XImagePtr`/
    `XImagePtr_or_DC` and *not* `Emacs_Pix_Context`), so the three arms now carry
    `--cfg feature="use_cairo"` - the configuration a pgtk build always has, and the first reading of
    this text named the missing name.  The mirror's `Emacs_Pix_Context`/`PIX_CONTAINER_TO_CONTEXT`
    were gated `not (feature = "have_pgtk")`, which that reading showed to be a mirror gap rather
    than a C fact: C 195's `#ifndef PIX_CONTAINER_TO_CONTEXT` holds for pgtk (only `w32gui.h`,
    C 139, defines that macro), so C 196/197 are the pgtk definitions too; the two cfg lines lost the
    exclusion and their comment records the measurement.  Second mirror change of the round: the same
    P-87 legs cover it, and `docs/ports/include-dispextern.md` carries the note.  The arm readings
    with `use_cairo` are the ones the section below reports (the HEAD baseline was re-measured with
    it: 33/19, 34/20, 11/18 errors/warnings before this slice's text).

31. **Two use-site declarations, one unit-local accessor arm, and the `///`-on-`extern` warning.**
    The unit gains `window_box_right` (dispextern.h 3581) and `get_glyph_string_clip_rect`
    (dispextern.h 3663 - the singular; `include/dispextern.rs` 4044 carries a copy whose second
    parameter the mirror spells `*mut c_void`, so the unit declares C's `XRectangle` shape, the
    `src/w32term.rs` 1750-1768 reading of the same two prototypes) plus `cairo_rel_line_to`
    (cairo.h 944-945); `window_bits!` is this unit's own copy of C's `pseudo_window_p` read
    (window.h 438, bit 11 of `window::bits`, whose shift `include/window.rs` 195 names) because the
    mirror's `WINDOW_PSEUDO_P` reader would lose C's member token - the `src/w32term.rs` 937-945
    arm of the same rule.  No mirror changed this round, so P-87's two legs do not apply; the
    consumer sweep is re-read anyway and stays green.  **The first reading of this slice named two
    warnings**: a `///` doc comment on an `unsafe extern "C"` block is rustc's `unused doc comment`
    (both of the blocks above had one), so both are `/* */` - the rule slice 3's four blocks already
    record for the same construct.

32. **One use-site declaration and two written-out promotions.**  `image_ascent` (dispextern.h 3774;
    defined in `src/image.rs` with `#[unsafe(no_mangle)]`) is declared at the use site with C's
    prototype, C's `&s->slice` being the port's `&raw mut` - the `src/w32term.rs` 3121-3129 copy of
    the same declaration.  C 2181/2183 compare `s->slice.x + s->slice.width` with `s->img->width`;
    the mirror's `glyph_slice_bits!` yields the record's storage-unit `u32`, while C's two 16-bit
    bit-field reads promote to `int` (the image record's `width`/`height` are `int`, dispextern.h
    3304), so the port writes both reads `as c_int` - and the **first reading of this slice named
    exactly those two sites** as `mismatched types` (E0308, expected `u32`, found `i32`).  They were
    the only two errors that text produced, so the repaired reading is a **zero delta** (the tenth
    slice's section).  No mirror changed this round, so P-87's two legs do not apply; the consumer
    sweep is re-read anyway and stays green.

33. **Six cairo rows this slice needs, all at the use site.**  `cairo_set_source` (cairo.h 701-702),
    `cairo_translate` (870-871), `cairo_fill_preserve` (998-999) and
    `cairo_image_surface_get_format` (2728-2729) are declared in this unit, each in its own
    `unsafe extern "C"` block with the header line it was read from, and the two
    `CAIRO_FORMAT_*` values C 2226 tests (`CAIRO_FORMAT_A8` 2, `CAIRO_FORMAT_A1` 3) are the unit's
    constants beside them (`CAIRO_FORMAT_RGB24` 1 joins them because it is the same enum run; the
    mirror carries `CAIRO_FORMAT_ARGB32` and `src/image.rs` 353-358 is the tree's copy of the same
    measurement).  The other five cairo names this slice calls are the mirror's rows
    (`cairo_pattern_get_surface`, `cairo_mask` and the operator/format rows in `include/gtk.rs`).
    No mirror changed, so P-87's two legs do not apply; the consumer sweep is re-read anyway and
    stays green.  C's `cairo_surface_t *surface;` is uninitialised and takes the port's zero
    initialisation (PORTING.md 1.5), and `cr_data` - the mirror's `*mut c_void`, C's `void
    *cr_data;` - is cast at the `pgtk_cr_draw_image` call (P-72 rule 3).

34. **Two promotions and four pointer truth tests.**  C 2318's `s->slice.y + s->slice.height >=
    s->img->height` is the same class as Deviation 32: the mirror's `glyph_slice_bits!` yields the
    record's storage-unit `u32` while C's two 16-bit bit-field reads promote to `int`, so both are
    `as c_int` (the image record's `height` is `int`, dispextern.h 3304), and C 2325's
    `height > s->slice.height` takes the same cast on the right-hand side.  C's four truth tests over
    the image record are the port's spellings of the member types the mirror carries: `hmargin` and
    `vmargin` are `c_int` (`!= 0`), while `mask` and `pixmap` are pointers (`!(…).is_null ()` /
    `(…).is_null ()` - the `src/w32term.rs` 2757-2761 reading of the same four tests).  C 2324's
    `s->stippled_p = …` and C 2348's `s->background_filled_p = true` are `glyph_string_bits!`'s
    **setter** arms (Deviation 15's pair; both were already in the mirror, so this slice adds no
    name).  No mirror changed, so P-87's two legs do not apply.

35. **Three use-site declarations.**  `window_box_left` (dispextern.h 3579),
    `window_box_left_offset` (dispextern.h 3580) and `cursor_in_mouse_face_p` (dispextern.h 3692)
    are declared in this unit, each in its own `unsafe extern "C"` block - `src/xdisp.rs` defines all
    three and no mirror carries them, so the declaration sits at the C 2377/2389/2414 calls (the
    `src/w32term.rs` 3134-3141 block of the same three prototypes; `window_box_right`, the ninth
    slice's, is the fourth of that group).  C's `eassert (s->first_glyph->type == STRETCH_GLYPH)`
    keeps the macro with the glyph record's `type` read, `x_stretch_cursor_p` is the globals mirror's
    `bool` (so C's `!` is the port's), `FRAME_COLUMN_WIDTH` is `include/frame.rs` 1353's `pub fn`,
    and C's `min (FRAME_COLUMN_WIDTH (s->f), background_width)` is the port's `isize` fold
    (Deviation 22's class).  No mirror changed, so P-87's two legs do not apply.

36. **One cairo row, and the first `macro-body-notes.tsv` row of this unit.**  `cairo_set_dash`
    (cairo.h 861-865) is declared in this unit at its C 2475 use (`cairo_set_line_width`/
    `cairo_stroke`/`cairo_move_to`/`cairo_line_to` are the mirror's and the seventh slice's rows).
    The `--check` reading then named one *token* finding the gate reports for a function body:
    `function pgtk_fill_underline does not mention break, case, default, fallthrough, switch` - C's
    `switch (style)` is the port's `match` over the labels it names and the `FACE_UNDERLINE_DOTS`
    arm's `FALLTHROUGH;` is written in that arm's own body (one copy of the shared `pgtk_draw_dash`
    statement, the `src/w32term.rs` 4083-4108 reading of the same C text, PORTING.md 2.2), so the
    five tokens **cannot** reappear in the ported text.  That is `macro-body-notes.tsv`'s shape
    (its header: "C macros *and functions* whose Rust body legitimately cannot mention one of the
    tokens of the C body"), so this unit's first row lands there with the reason, and the gate
    returns to the "function retires" reading alone.  No mirror changed, so P-87's two legs do not
    apply.

37. **The dispatcher's own declarations, the seven accessor arms it needed, and its `switch`.**  The
    unit gains `font_for_underline_metrics` (dispextern.h 3638, defined in `src/xdisp.rs`; the
    `src/haikuterm.rs` 172 declaration of the same prototype), `cairo_save` (cairo.h 561-562) and
    `cairo_restore` (cairo.h 564-565) at their use sites; `face_bits!` gains the seven arms the
    epilogue reads (`underline`, `underline_defaulted_p`, `underline_at_descent_line_p`,
    `overline_p`, `overline_color_defaulted_p`, `strike_through_p`,
    `strike_through_color_defaulted_p` - all seven positions were already in the unit's own `FACE_*`
    table, so this slice adds arms and no constants); `x_draw_xwidget_glyph_string` is called with
    the mirror's `*mut c_void` parameter (`s as *mut c_void`, the use-site cast P-72 rule 3); and
    C's `s->row->stipple_p = s->face->stipple` is `glyph_row_bits!`'s setter with the value's truth
    conversion (the record stores the field in its measured 1-bit run).  C's `switch
    (s->first_glyph->type)` then produced this unit's second `macro-body-notes.tsv` row
    (`break,case,cmp,default,switch`; the `cmp` token is C's `s->first_glyph->u.cmp.automatic` read,
    which the port spells `glyph_u_cmp_bits! (automatic, …)` - Deviation 24's accessor), so the gate
    returns to the "function retires" reading alone.  **The first readings of this text named three
    defects**, all fixed and all in this slice's own lines: two extra dereferences
    (`(*(*prev)).hl` / `(*(*next)).hl` where the corpus spells `(*prev).hl` / `(*next).hl` - the
    `src/xterm.rs` 12752 reading of the same C text) and a stipple write that named the field
    instead of the storage unit.  No mirror changed, so P-87's two legs do not apply.

38. **Three use-site declarations, one unit-local accessor, and a second registered class.**  The unit
    gains `get_phys_cursor_glyph` (window.h 1186, defined in `src/window.rs` 8061) and
    `get_phys_cursor_geometry` (dispextern.h 3652-3653, defined in `src/abi_shim.rs` 2419) at their
    C 2886/2891 uses; `pointer_invisible!` is this unit's own reader of `frame.h` 437's
    `bool_bf pointer_invisible : 1` (the mirror carries the shift, `include/frame.rs` 540-545, and no
    accessor - the `src/androidterm.rs` 240-243 copy of the same macro over `bits1`); and the
    condition's two comma expressions (`(f = XFRAME (…), width = …, width != 0)` and
    `(height = …, height > 0)`) are the port's **blocks in the condition**, the
    `src/w32term.rs` 1431-1441 reading of the same C text (C's assignment-as-operand has no Rust
    spelling).  `gdk_window_set_cursor` (`.h`'s row in `include/gtk.rs` 564) takes the mirror's
    `*mut GdkCursor`, so the call passes `cursor as *mut GdkCursor` (P-72 rule 3), and C 2903's
    `cursor_glyph->pixel_width > wd` takes `as c_int` for the record's `short` (Deviation 22's
    class).  **One new registered class**: `pgtk_clip_to_row` (C 3629's definition, still in the
    remainder) is a name this slice reaches first; the Evidence ② table gains its row, and
    C 2849's `redraw_fringe_bitmaps_p = 1` produced this unit's third `macro-body-notes.tsv` row
    (the literal `1` cannot reappear: the value is `glyph_row_bits!`'s bool setter).  The first
    reading named one warning - a `///` doc comment on the two new `unsafe extern "C"` blocks - which
    is `/* */` now.  No mirror changed, so P-87's two legs do not apply.

39. **One use-site declaration, one unit-local inline copy, and two written-out reads.**  The unit
    declares `draw_phys_cursor_glyph` (dispextern.h 3649-3651, defined in `src/abi_shim.rs` 2499) at
    its C 2948 use, and carries its own copy of frame.h 1379/1643-1666's **inline**
    `FRAME_FACE_CACHE`/`FACE_FROM_ID` - C pastes that definition into every unit that includes the
    header, so the port keeps one copy per including unit (the `src/term.rs` 360-371 form;
    `include/frame.rs` 1534-1552 records why that mirror's own copy is a module the unit cannot
    reach).  C 2952's `cursor_glyph->face_id` is the glyph record's bit-field, so the port reads it
    with `glyph_bits! (face_id, …)`, and C 2947's `MATRIX_ROW (w->current_matrix, …)` takes the
    window record's `*mut c_void` cast to the typed matrix (`MATRIX_ROW! ((*w).current_matrix as
    *mut glyph_matrix, …)`, the `src/dispnew.rs` 3150 spelling).  **The first reading of this text
    named all four** - the missing `FACE_FROM_ID`, the two E0609 field reads and the matrix cast
    (`src/pgtkterm.rs` was the only unit in the tree that had never carried the inline pair) - and
    they are the only defects it produced.  No mirror changed, so P-87's two legs do not apply.

40. **Six declarations, one accessor arm, and this function's `switch`.**  The unit gains
    `draw_fringe_bitmap` (dispextern.h 3703, defined in `src/fringe.rs` 1280) at its C 3034 use; the
    cairo content enum (`CAIRO_CONTENT_COLOR` 0x1000 / `_ALPHA` 0x2000 / `_COLOR_ALPHA` 0x3000,
    cairo.h 392-396) with its `cairo_content_t` alias, `cairo_surface_create_similar`
    (cairo.h 2358-2361) and the `cairo_rectangle_t` record (cairo.h 1054-1056, the two
    `pgtk_copy_bits` parameters) - all four at the use site, because the mirror carries the cairo
    pointers and `cairo_format_t` and none of these; `window_bits!` gains the `phys_cursor_on_p`
    **setter** arm (C 3026 writes the window record's measured `bits` word at
    `WINDOW_PHYS_CURSOR_ON_P_SHIFT` 7, `include/window.rs` 191).  C's `switch (cursor_type)` then
    produced this unit's fourth `macro-body-notes.tsv` row (`break,case,default,switch`; the same
    class as the dispatcher's).  **The first reading of this text named two defects**, both fixed:
    the missing `cairo_rectangle_t` (two errors) and C 3031's
    `w->phys_cursor.hpos >= glyph_row->used[TEXT_AREA]`, where `used`'s elements are the record's
    `short` and the comparison promotes them to `int` (so the port writes `as c_int` - Deviation
    22's class).  With those in place the repaired reading is a **zero delta**: this text adds no
    error and no warning.  No mirror changed, so P-87's two legs do not apply.

41. **Three declarations, eight zero-initialised locals, and the `HAVE_XWIDGETS` arm's four names.**
    The unit gains `window_box` (dispextern.h 3574-3575, defined in `src/window.rs`) and
    `gui_clear_cursor` (dispextern.h 3657) at their C 3123/3203 and C 3246 uses, plus
    `gtk_widget_queue_allocate` (C 3236) with the GTK segment's own convention - the header alone and
    no line, because this machine has no GTK tree.  C's eight locals that the reference fills by
    *address* (`window_box (w, ANY_AREA, &x, …)` and the xwidgets arm's four text-area outputs) are
    zero-initialised in the port: Rust's definite-initialisation analysis cannot see the callee's
    writes through a raw pointer, and PORTING.md 1.5 is the rule the corpus reads them by (Deviation
    12's class) - the first readings named exactly those eight `E0381`s and nothing else.  Inside the
    dead `#ifdef HAVE_XWIDGETS` block the port keeps C's names with the mirror's shapes:
    `XG_XWIDGET_VIEW` is a `&CStr`, so the `g_object_get_data` call passes its `.as_ptr ()` (the
    `src/xwidget.rs` 4788 reading), and `XXWIDGET` is a **function** in the mirror rather than C's
    macro (so `XXWIDGET ((*view).model)` and not `XXWIDGET! (…)`); C's three cast macros
    (`GTK_CONTAINER`, `GTK_FIXED`, `G_OBJECT`) are the port's pointer casts (P-72 rule 3), and
    `Emacs_Rectangle`'s `c_uint` members take C's implicit widenings from the window's `int` fields.
    With those in place the repaired readings are a **zero delta in both arms** (A and the
    `have_xwidgets` B arm): this text adds no error and no warning anywhere.  No mirror changed, so
    P-87's two legs do not apply.

42. **The first `#if 0` body in this unit, one unit-local inline copy, and one mirror row that is only
    a comment.**  `pgtk_bitmap_icon`'s whole body is C 3267-3324's `#if 0` - dead text in every
    configuration - so the port carries C's text inside a comment and compiles only C 3325's
    `return false;` (the `cm.rs` 86 reading, the `src/w32term.rs` 133-139 precedent); the function's
    two parameters are therefore unused in the live text (`let _ =` keeps the names).
    `pgtk_draw_vertical_window_border` needed the third member of frame.h's inline trio, so the unit
    now carries `FACE_FROM_ID_OR_NULL` beside `FRAME_FACE_CACHE`/`FACE_FROM_ID` (the
    `src/term.rs` 372-377 form - C pastes the inline definitions into every including unit).
    `include/gtk.rs` carries `gtk_window_set_icon`'s prototype as a *doc comment* (1423-1424) with no
    `pub fn` row, so the unit declares it at its use site with the GTK segment's convention (the
    header alone, no line); the mirror's missing row is owed to that mirror's owner (the same class
    as the placeholder note at the head of `include/pgtkterm.rs`).  C's two `GTK_WINDOW` cast macros
    are the port's pointer casts (P-72 rule 3), which produced this unit's sixth
    `macro-body-notes.tsv` row (`gtk_window`: the cast's token cannot reappear).  With those in
    place the repaired readings are a **zero delta in all three arms**.  No mirror changed, so P-87's
    two legs do not apply.

43. **Three use-site declarations, one accessor arm, and a declaration the mirror owes.**  The unit
    gains `buffer_flipping_blocked_p` (dispextern.h 3719, defined in `src/xdisp.rs`; the
    `src/androidterm.rs` 167 and `src/haikuterm.rs` 180 copies of the same declaration) and
    `gtk_widget_queue_draw` (C 3462, the GTK segment's convention: the header alone, no line) at
    their uses; `FRAME_MOUSE_UPDATE!`'s expansion needs `note_mouse_highlight` in scope, and
    `include/dispextern.rs`'s own note (4050) keeps it in that mirror's "owed, not declared" list,
    so the unit declares it with the `*mut c_void` frame parameter the mirror's macro body passes
    (`include/frame.rs` 1725-1729 records the same reading) - a `*mut frame` declaration is `E0308`
    at the macro's call site, which is how the first reading named it.  C's seven
    `f->borders_respect_alpha_background` reads are the unit's own `borders_respect_alpha_background!`
    reader (frame.h 540's `bool_bf`, whose shift `include/frame.rs` 746-751 names for this arm - the
    `src/frame.rs` 298-301 copy of the same macro over `bits2`), and C 3451's
    `MOUSE_HL_INFO (f)->mouse_face_defer = false` is the mirror's writer pair
    (`set_mouse_face_defer`), which produced this unit's eighth `macro-body-notes.tsv` row
    (`mouse_face_defer`).  With those in place the repaired readings are a **zero delta in all three
    arms**.  No mirror changed, so P-87's two legs do not apply.
44. **Three use-site declarations, one GDK type, one accessor, three zero-initialised locals and
    one cast.**  The unit gains `gui_mouse_grabbed` (dispextern.h 4006, defined by `src/frame.rs`
    6420; the `src/androidterm.rs` 163-166 copy of the same declaration, with this unit's own
    `pgtk_display_info` for dispextern.h 152's `Display_Info`), `remember_mouse_glyph`
    (dispextern.h 3586, with pgtkgui.h 101's `#define NativeRectangle XRectangle` - the field
    `pgtk_display_info.last_mouse_glyph` is that record) and `gdk_window_get_device_position`
    (C 3523/3549; the GTK segment's convention: the header alone, no line, since there is no GDK
    tree here); `GdkModifierType` (C 3497's local, written through a pointer by both calls) is the
    unit's own `c_int` alias, the `src/xwidget.rs` 260-263 row.  C 3509's
    `XFRAME (frame)->mouse_moved = false` is the unit's own `mouse_moved!` **writer** (frame.h 434's
    `bool_bf`, whose shift `include/frame.rs` 529-537 names for this arm; the `src/androidterm.rs`
    234-238 copy of the same macro carries both arms - this one only what C's body performs).  C's
    uninitialised `win_x`/`win_y`/`mask` are zero-initialised (PORTING.md 1.5, Deviation 10's class),
    `insist` is kept with `let _ =` (unused in C's body), and C's two `XSETINT` calls carry the cast
    C performs implicitly (`win_x as EMACS_INT`; the `src/androidterm.rs` 814-815 spelling of the
    same site) - without it the first reading was two `E0308`s.  `pgtk_any_window_to_frame` is a
    registered class of its own (C 7314, in the remainder).  No mirror changed, so P-87's two legs
    do not apply.
45. **One cairo record, six cairo declarations, four zero-initialised locals, and a latent defect
    the retirement of `pgtk_clip_to_row` unmasked.**  The unit gains `cairo_rectangle_int_t`
    (cairo.h 532-535, C 3633's local - and since this machine *has* that header,
    `/opt/homebrew/include/cairo/cairo.h`, the record is a read of it rather than a placeholder;
    C assigns its four members one at a time, which Rust's no-partial-initialisation rule has no
    counterpart for, PORTING.md 1.13) and the six cairo entry points C 3594-3622 call that no mirror
    carried (`cairo_image_surface_create`, `cairo_image_surface_get_stride`,
    `cairo_image_surface_get_data`, `cairo_surface_mark_dirty`,
    `cairo_pattern_create_for_surface`, `cairo_pattern_destroy` - P-88 ①'s use-site convention, the
    seventh, `cairo_surface_destroy`, being `include/gtk.rs` 338).  `pgtk_clip_to_row`'s definition
    retires the last `cannot find function pgtk_clip_to_row` site, which **unmasked** three `E0381`s
    in `pgtk_draw_hollow_cursor` (`let mut x: c_int;` / `y` / `h`, C 2881, passed by `&raw mut`):
    a `cannot find function` error inside an item stops that item before MIR borrow-checking, so
    seven slices' arm readings had hidden them behind the E0425 in the same function - a one-line
    rustc reading (`let mut x: c_int; g (&raw mut x);`) reproduces the rule.  The four locals are
    zero-initialised (PORTING.md 1.5, Deviation 12's class).  One token row follows from the same
    text: `pgtk_destroy_fringe_bitmap`'s two null sites are the port's `.is_null ()` /
    `core::ptr::null_mut ()`, recorded in `docs/ports/macro-body-notes.tsv` (the
    `Fpgtk_selection_exists_p` row's class).  No mirror changed, so P-87's two legs do not apply.
46. **The record's name is its module path, two use-site declarations, one zero-initialised record
    and the two empty C bodies.**  `include/dispextern.rs` carries the five records of C 2159-3200
    inside `pub mod dispextern_unit_names`, so `pgtk_draw_fringe_bitmap`'s parameter, its two
    `bool_bf` accessors (`get_cursor_p`/`get_overlay_p`) and the `max_used_fringe_bitmap` extern are
    named through that path (`src/w32term.rs` 1489's reading; the unqualified form is the six
    `E0425`s this slice's first reading produced).  The unit declares `gui_define_fringe_bitmap`
    (dispextern.h 3711; the mirror's extern block carries 3710's `max_used_fringe_bitmap` and not
    this line) and `gtk_event_box_new` (the GTK segment's convention: the header alone, no line) at
    their C 3676/3708 uses.  C's `Emacs_GC gcv;` is zero-initialised because C fills two of its
    members one at a time (PORTING.md 1.13), C's `true` handed to a `gboolean` parameter is the
    port's `true as gboolean` (`src/gtkutil.rs` 2119's spelling of the same conversion), and the two
    empty C bodies (`hourglass_cb` C 3694-3698 and `pgtk_flush_display` C 3748-3752) are kept empty
    (the `frame_set_mouse_pixel_position` shape).  C's `hourglass_enter_count++ == 0` and
    `--hourglass_enter_count == 0` keep the step inside the condition as block expressions
    (`src/keyboard.rs` 3889's shape).  One token row follows: C's `p->cursor_p`/`p->overlay_p` are
    the accessor pair, recorded in `docs/ports/macro-body-notes.tsv`.  No mirror changed, so P-87's
    two legs do not apply.
47. **The port's first redisplay-interface object: C's positional initializer, the entry types, and
    the fifteen definitions whose address it takes.**  `pgtk_redisplay_interface` (C 3756-3785) is
    written as a same-named-field literal whose names and order are the mirror's
    (`dispextern_unit_names::redisplay_interface`, C 3069-3200; measured **28 entries = 28 fields**, so
    nothing is left to C's implicit zero) - the rule now stands in PORTING.md §1.4.  An entry's type is
    the *field's*: five are `Some (name)` directly (the three fringe hooks, the overhang hook and
    `draw_glyph_string`, whose signatures already match), three are `None` (C's two commented-out
    `NULL`s at C 3764-3765 and C 3781's `shift_glyphs_for_insert`), one is the array's address
    (`&raw mut pgtk_frame_parm_handlers as *mut c_void`), and **nineteen** are the
    `G_CALLBACK`-class transmute (Deviation 11's idiom) because the mirror spells C's
    `struct frame *`/`struct window *` parameters `*mut c_void` (P-55) - a direct `as` is `E0605`,
    measured.  C's first entry forced two more readings: `pgtk_frame_parm_handlers` is **defined in
    pgtkfns.c** (C 963-1015) and the port defines it in `src/pgtkfns.rs` 1919 as
    `[frame_parm_handler; 51]`, so this unit declares an extern array of that measured shape (the
    `src/frame.rs` 5011-5014 copy of dispextern.h 3063's typedef), and `gui_clear_window_mouse_face`
    (C 3767) is `src/xdisp.rs` 31968's definition, so it joins the six `gui_*` use-site declarations
    whose C prototypes are dispextern.h 3636-3648's.  **The fifteen `static` definitions whose address
    this object takes become `unsafe extern "C" fn`** (`pgtk_scroll_run`, `pgtk_after_update_window_line`,
    `pgtk_flush_display`, `pgtk_draw_fringe_bitmap`, `pgtk_define_fringe_bitmap`,
    `pgtk_destroy_fringe_bitmap`, `pgtk_compute_glyph_string_overhangs`, `pgtk_draw_glyph_string`,
    `pgtk_define_frame_cursor`, `pgtk_clear_frame_area`, `pgtk_draw_window_cursor`,
    `pgtk_draw_vertical_window_border`, `pgtk_draw_window_divider`, `pgtk_show_hourglass`,
    `pgtk_hide_hourglass`) - PORTING.md §1.4's "one ABI per function-pointer position" (the
    `floatfns.c` lesson; the first reading named exactly the four that still had mismatched
    signatures as four `E0308`s).  No mirror changed, so P-87's two legs do not apply.
48. **Three use-site rows, two pointer casts, and two standing `E0308`s repaired.**  The unit gains
    `cairo_set_source_rgb` (cairo.h 705-706, C 3844) and `CAIRO_OPERATOR_DIFFERENCE` (its *value*
    measured with clang against this machine's header: **23**; cairo.h 690, the same reading
    `include/gtk.rs` 296-297 carries for `CAIRO_OPERATOR_SOURCE = 1`) as cairo-segment rows of its
    own (this file's practice), and `gtk_widget_error_bell` (C 3919, the GTK segment's header-alone
    convention) - the `///` form on an `extern` block is a warning, so the first row is `/* */`
    (Deviation 2's shape; the first reading of this slice reproduced that warning).  C's integer
    literals handed to `double` parameters are the port's `0 as c_double` / `1 as c_double`
    (`src/gtkutil.rs` 807's spelling, which also keeps C's own token), and the two `atimer_visible_bell`
    sites carry the pointer cast the mirror's bare-tag field needs (`*mut c_void` where C 422 says
    `struct atimer *`: `as *mut atimer` on the `cancel_atimer` argument, `as *mut c_void` on the
    `start_atimer` result - Owed 10).  **`pgtk_free_frame_resources`'s two `cancel_atimer` calls
    (this unit, C 529/536) carried the same mismatch as two standing `E0308`s since the second slice**;
    this slice repairs them, which is why all three arms read **minus two** rather than zero.
    `recover_from_visible_bell` is `unsafe extern "C" fn` because C 3900 takes its address
    (PORTING.md 1.4; its signature matches atimer.h 44's callback type exactly, so the entry needs no
    conversion), while `pgtk_ring_bell` stays a plain `unsafe fn` until the slice that takes its
    address (C 7442) converts it.  No mirror changed, so P-87's two legs do not apply.
49. **One function, two `int`-to-`bool` conversions and one parameter C never reads.**
    `pgtk_read_socket` (C 3932-3970) is a plain `unsafe fn` (C 7447 takes its address later, when
    the terminal's `read_socket_hook` is assigned, and that slice converts it - the `pgtk_ring_bell`
    note above); C's `g_main_context_acquire` result and `g_main_context_pending` test are `int`
    expressions on a `bool` local and in a `while`, so the port writes `!= 0` (the unit's standing
    spelling for C's truthiness), `hold_quit` is `*mut input_event` in `include/termhooks.rs` 114
    (`struct terminal *` is that mirror's 196), and C's body never reads `terminal`, so the port keeps
    the name with `let _ =` (the Deviation 44 shape).  All four GLib rows are
    `include/gtk.rs` 1919-1925's, i.e. header readings against this machine's GLib 2.90.0 - nothing
    new was declared.  No mirror changed, so P-87's two legs do not apply.

## Owed (2026-10-05, PORTING.md P-90)

- **The SDK surface moves out of the unit.**  `femacs/src/pgtkterm.rs` carries the cairo, GLib,
  GDK and GTK declarations as in-unit segments because it was the surface's first (and, at that
  moment, only) consumer.  With `gtkutil.c` as the second consumer, P-90 requires one dedicated
  mirror - `femacs/src/include/gtk.rs` with its own ledger `docs/ports/include-gtk.md`, the
  `include-xlib.rs` shape - and the in-unit segments moved into it (P-88 ⑤'s move-back).  The
  mirror is not created yet: this file's own slice was landed without it, and the move-back is
  owed to the round that creates it.  Until then the arm's readings carry the declarations twice
  (once per unit), which is harmless across crates and wrong only inside one TU.

10. `include/pgtkterm.rs` 346/355: `pgtk_output.atimer_visible_bell` and `scale_factor_atimer` are
    declared with the *bare-tag* shape (`*mut c_void`) where C 422/452 say `struct atimer *`;
    `include/atimer.rs` 27-60 has the record now, so the two fields can take `*mut atimer` and the
    three use-site casts the twenty-sixth slice writes (`as *mut atimer` on `cancel_atimer`'s
    argument in `pgtk_flash` and in `pgtk_free_frame_resources`'s two sites - Deviation 48) retire.
    Until then those casts are the P-72 rule 3 spelling, and the reading they replace was two
    standing `E0308`s.

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

## The fifth slice (C 1485-1571): the composite glyph-string foreground

**What is in it.**  `pgtk_draw_composite_glyph_string_foreground` (C 1485-1572): the box-line
offset, the `font_not_found_p`/`cmp_from == 0` rectangle arm, the two-arm `u.cmp.automatic` fork -
the non-automatic one walking `s->cmp_from .. s->cmp_from + s->nchars` with `COMPOSITION_GLYPH`'s
TAB test and the two `offsets` reads, the automatic one walking `cmp_from .. cmp_to` over
`LGSTRING_GLYPH`/`LGLYPH_ADJUSTMENT`/`LGLYPH_WIDTH`/`LGLYPH_XOFF`/`LGLYPH_YOFF`/`LGLYPH_WADJUST`
with its two deferred `font->driver->draw` pairs and the closing `j < i` flush.  Rust:
2,071 -> **2,199** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` both sides - Deviation 17).**
```
A narrow = have_pgtk + have_window_system          : HEAD 40 errors / 19 warnings -> 40 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 41 / 20 -> 41 / 20
C = A + use_gtk + have_gtk3                        : 18 / 18 -> 18 / 18
```
**Every arm reads exactly the same number on both sides and no class changes**: the slice's text
adds no error and no warning.  One repair got it there and it is recorded: the mirror's
`LGSTRING_GLYPH` index is a `ptrdiff_t`, so C 1539 takes `i as ptrdiff_t` (Deviation 24) - the first
reading of this text read one mismatch.  The unit's own `glyph_u_cmp_bits!` copy uses the pointer
call form for the same reason Deviation 21 gives for `face_bits!`.

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **178 rows -> 177**: `pgtk_draw_composite_glyph_string_foreground` retires, and the file's own
`line count drifted` row is re-measured 73.4% -> **71.7%** (C 7,780 vs the port's 2,199).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2049/36/0**; `check-dead-file-coverage.py` **145/0** (**37 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round - the round reads the sweep anyway and it is `checked 83 unit(s), 0 failure(s)`
with `checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the
frozen configuration compiles none of this file, there is no GTK or cairo to draw with here, and
the three arm readings are **compile** readings of text this machine cannot build.

## The sixth slice (C 1575-1662): the glyphless glyph-string foreground

**What is in it.**  `pgtk_draw_glyphless_glyph_string_foreground` (C 1575-1662): the box-line
offset, the `unsigned char2b[8]` local whose address C hands to `s->char2b` (and clears at the
end), C 1595-1601's `#ifdef GCC_LINT` pair as two `const`s, the acronym arm (the `CHAR_TABLE_P` /
`CHAR_TABLE_EXTRA_SLOTS` guards, the `CHAR_TABLE_REF` / `extras[0]` fork on `for_no_font`, and the
`CONSP`/`XCAR`/`STRINGP` chain), the hex-code arm (`eassume`, `sprintf (buf, "%0*X", …)`), the two
`font->driver->draw` calls with the four `slice.glyphless` offsets, the `THIN_SPACE` guard around
`pgtk_draw_rectangle`, and the closing `s->char2b = NULL`.  Rust: 2,199 -> **2,334** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` both sides - Deviation 17).**
```
A narrow = have_pgtk + have_window_system          : HEAD 40 errors / 19 warnings -> 40 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 41 / 20 -> 41 / 20
C = A + use_gtk + have_gtk3                        : 18 / 18 -> 18 / 18
```
**Every arm reads exactly the same number on both sides and no class changes**: the slice's text
adds no error and no warning.  One repair got it there (the `CHAR_TABLE_REF` index cast, Deviation
25); the unit's own `glyphless_bits!`/`GLYPHLESS_DISPLAY_*` copies and the two `PACIFY_GCC_BUG_81401`
arms are recorded in the same deviation.

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **177 rows -> 176**: `pgtk_draw_glyphless_glyph_string_foreground` retires, and the file's own
`line count drifted` row is re-measured 71.7% -> **70.0%** (C 7,780 vs the port's 2,334).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2049/36/0**; `check-dead-file-coverage.py` **145/0** (**38 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The seventh slice (C 1664-1804): the lighter-color computation and the relief primitives

**What is in it.**  `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT` (C 1664-1675),
`pgtk_compute_lighter_color` (C 1684-1746: C's `Emacs_Color color, new;`, the `pgtk_query_color`
read, `eassert`, the three `min (0xffff, factor * color.<channel>)` writes, `bright`, the dark-boost
arm's `dimness`/`min_delta` with its two `factor < 1` halves, the two `new.pixel` folds and the
closing `*pixel = new.pixel`), `pgtk_fill_trapezoid_for_relief` (C 1748-1762: the clip pair, the
four cairo path calls and `cairo_fill`), `enum corners` (C 1764-1771) and
`pgtk_erase_corners_for_relief` (C 1773-1804: the `CORNER_LAST` walk with C's four coordinate
branches, `cairo_arc` over `i * M_PI_2`, and the closing
`cairo_clip`/`cairo_rectangle`/`cairo_fill`).  Rust: 2,334 -> **2,561** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` both sides - Deviation 17).**
```
A narrow = have_pgtk + have_window_system          : HEAD 40 errors / 19 warnings -> 40 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 41 / 20 -> 41 / 20
C = A + use_gtk + have_gtk3                        : 18 / 18 -> 18 / 18
```
**Every arm reads exactly the same number on both sides, and the error and warning sets were diffed,
not just counted**: the slice's text adds no error and no warning.  The two declarations it needs
(Deviation 27) were in place before the reading.  **Arm reach is measured, not assumed** (P-86 ⑦):
injecting `let _: c_int = true;` as the first statement of `pgtk_compute_lighter_color` moves arm A
from 40 errors to **41**, anchored at that line (the injected statement was then removed and the file
re-measured at 2,561 lines).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **176 rows -> 166**: `pgtk_compute_lighter_color`, `pgtk_fill_trapezoid_for_relief` and
`pgtk_erase_corners_for_relief` retire, `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT` retires as a **macro and
as a constant**, and the five `CORNER_*` constants retire - ten rows; the file's own
`line count drifted` row is re-measured 70.0% -> **67.1%** (C 7,780 vs the port's 2,561).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2054/36/0** (five of the externals are this slice's `cairo.h` and `math.h` rows);
`check-dead-file-coverage.py` **145/0** (**39 range rows** over `src/pgtkterm.c`);
`check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** with the consumer
sweep `checked 83 unit(s), 0 failure(s)` / `checking 83 unit(s), 0 unit failure(s)` (a mirror changed
this round - Deviation 27).  **Not claimed**: none of this has been run - the frozen configuration
compiles none of this file, there is no GTK or cairo to draw with here, and the three arm readings
are **compile** readings of text this machine cannot build.

## The eighth slice (C 1806-1951): the relief colours, the clip rectangles and the relief rectangle

**What is in it.**  `pgtk_setup_relief_color` (C 1806-1822: the `Emacs_GC` local, the
`pgtk_compute_lighter_color` call and C's chained `xgcv.foreground = relief->pixel = pixel;`),
`pgtk_setup_relief_colors` (C 1824-1853: the three-arm colour choice with the two image macros and
the `relief_background_valid_p` guard around the two `pgtk_setup_relief_color` calls),
`pgtk_set_clip_rectangles` (C 1855-1866: the `XRectangle` walk into `cairo_rectangle` and
`cairo_clip`) and `pgtk_draw_relief_rect` (C 1868-1951: the raised/not colour pair, the six
`pgtk_fill_rectangle` calls, the two `pgtk_fill_trapezoid_for_relief` calls, the two `corners |=`
writes and the `pgtk_erase_corners_for_relief` close).  Rust: 2,561 -> **2,790** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides -
Deviation 30).**
```
A narrow = have_pgtk + have_window_system          : HEAD 33 errors / 19 warnings -> 39 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 34 / 20 -> 40 / 20
C = A + use_gtk + have_gtk3                        : 11 / 18 -> 17 / 18
```
**Every arm's delta is one registered class and nothing else** - the error sets were diffed, not just
counted: `cannot find function pgtk_fill_rectangle` goes 1 -> **7** (the six call sites this slice's
`pgtk_draw_relief_rect` adds to the C 5035 definition the Evidence ② table already records), with 0
new warnings and 0 new classes.  The first reading of this text named four defects (the mirror's
storage unit, Deviation 28; the `face_bits!` arm and the image pair, 29; the missing
`Emacs_Pix_Context`, 30; and two `if (cond) call (...)` bodies Rust needs braces around); the
readings above are the repaired text's.

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **166 rows -> 162**: the four items retire, and the file's own `line count drifted` row is
re-measured 67.1% -> **64.1%** (C 7,780 vs the port's 2,790).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2054/36/0** (this slice's `dispextern.h` citations resolve in the reference tree, so the
external count is unchanged); `check-dead-file-coverage.py` **145/0** (**40 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** with the consumer
sweep `checked 83 unit(s), 0 failure(s)` / `checking 83 unit(s), 0 unit failure(s)` (two mirrors
changed - Deviations 28 and 30).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The ninth slice (C 1953-2089): the box rectangle, the glyph-string box and the horizontal wave

**What is in it.**  `pgtk_draw_box_rect` (C 1960-2000: the `foreground_backup` save/restore pair
around C's four `pgtk_fill_rectangle` calls), `pgtk_draw_glyph_string_box` (C 2005-2048: the `last_x`
fold over `full_width_p`/`pseudo_window_p`, C's `last_glyph` two-arm choice, `eabs!`, the
`FACE_RAISED_BOX`/`FACE_SIMPLE_BOX` fork with the `DRAW_MOUSE_FACE` `prev`/`next` tests, and the two
calls the slice above defined) and `pgtk_draw_horizontal_wave` (C 2050-2089: the clip pair, the
`xoffset`/`n` fold, the `--n >= 0` loop over `cairo_rel_line_to` and the stroke).  Rust: 2,790 ->
**2,995** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides - the
arm set Deviation 30 fixed).**
```
A narrow = have_pgtk + have_window_system          : HEAD 39 errors / 19 warnings -> 43 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 40 / 20 -> 44 / 20
C = A + use_gtk + have_gtk3                        : 17 / 18 -> 21 / 18
```
**Every arm's delta is one registered class and nothing else** - the error sets were diffed:
`cannot find function pgtk_fill_rectangle` goes 7 -> **11** (the four call sites
`pgtk_draw_box_rect` adds), with 0 new warnings and 0 new classes.  **Arm reach is measured**
(P-86 ⑦): injecting `let _: c_int = true;` as the first statement of `pgtk_draw_glyph_string_box`
moves arm A to 44 errors, anchored at that line (the injected statement was then removed and the
file re-measured at 2,995 lines).  The first reading named two `unused doc comment` warnings on the
two new `extern` blocks; the readings above are the repaired text's (Deviation 31).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **162 rows -> 159**: the three items retire, and the file's own `line count drifted` row is
re-measured 64.1% -> **61.5%** (C 7,780 vs the port's 2,995).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2055/36/0** (the `cairo.h 944-945` row is the one new external);
`check-dead-file-coverage.py` **145/0** (**41 range rows** over `src/pgtkterm.c`);
`check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The tenth slice (C 2091-2190): the under-wave and the image relief

**What is in it.**  `pgtk_draw_underwave` (C 2091-2098: C's two local constants handed to the wave
above) and `pgtk_draw_image_relief` (C 2102-2190: the `image_ascent` offset, the box-line shift, the
two margin reads, the `DRAW_IMAGE_SUNKEN`/`DRAW_IMAGE_RAISED` fork over the two button-relief
globals with C's `min (…, 1000000)`, the two `V*_button_margin` chains, C's four comma expressions
and the `pgtk_draw_relief_rect` close).  Rust: 2,995 -> **3,136** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 43 errors / 19 warnings -> 43 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 44 / 20 -> 44 / 20
C = A + use_gtk + have_gtk3                        : 21 / 18 -> 21 / 18
```
**Every arm reads exactly the same number on both sides, and the error and warning sets diff empty**:
this slice's text adds no error and no warning - the first slice in this unit whose delta is not a
registered class.  One repair got it there: C 2181/2183's two promotions (Deviation 32), which the
first reading named as two `mismatched types` sites.  **Arm reach is measured** (P-86 ⑦): injecting
`let _: c_int = true;` as the first statement of `pgtk_draw_image_relief` moves arm A to 44 errors,
anchored at that line (the injected statement was then removed and the file re-measured at 3,136
lines).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **159 rows -> 157**: the two items retire, and the file's own `line count drifted` row is
re-measured 61.5% -> **59.7%** (C 7,780 vs the port's 3,136).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2055/36/0**; `check-dead-file-coverage.py` **145/0** (**42 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The eleventh slice (C 2192-2292): the background rectangle, the cairo image call and the image foreground

**What is in it.**  `pgtk_draw_glyph_string_bg_rect` (C 2195-2203: the `stippled_p` fork over
`fill_background`/`pgtk_clear_glyph_string_rect`), `pgtk_cr_draw_image` (C 2205-2239: the clip pair,
the `overlay_p` fork with `cairo_fill_preserve`, the translate, the `cairo_pattern_get_surface` /
`cairo_image_surface_get_format` pair and the `CAIRO_FORMAT_A8`/`_A1` fork between `cairo_set_source`
+ `cairo_fill` and `cairo_mask`) and `pgtk_draw_image_foreground` (C 2243-2292: the `image_ascent`
offset, the box-line shift, the two margin reads, the `cr_data` branch with its `DRAW_CURSOR`
relief rectangle and the unloaded-image rectangle).  Rust: 3,136 -> **3,293** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 43 errors / 19 warnings -> 43 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 44 / 20 -> 44 / 20
C = A + use_gtk + have_gtk3                        : 21 / 18 -> 21 / 18
```
**Every arm reads exactly the same number on both sides and both sets diff empty**: like the tenth
slice, this text adds no error and no warning - the first reading of it already resolved every name
(the six cairo rows are Deviation 33).  **Arm reach is measured** (P-86 ⑦): injecting
`let _: c_int = true;` as the first statement of both `pgtk_cr_draw_image` and
`pgtk_draw_image_foreground` moves arm A to 47 errors with two diagnostics anchored at those lines
(the injected statements were then removed and the file re-measured at 3,293 lines).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **157 rows -> 154**: the three items retire, and the file's own `line count drifted` row is
re-measured 59.7% -> **57.7%** (C 7,780 vs the port's 3,293).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2063/36/0** (this slice's cairo rows are the new externals);
`check-dead-file-coverage.py` **145/0** (**43 range rows** over `src/pgtkterm.c`);
`check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The twelfth slice (C 2294-2359): the image glyph string

**What is in it.**  `pgtk_draw_image_glyph_string` (C 2308-2359) with C 2294-2306's ASCII-art comment
carried verbatim above it: the two `max` folds over the face's box line widths, the two `height`
reductions, the `stippled_p`/`background_filled_p` **writes** through `glyph_string_bits!`'s setter
arms, the eight-condition background test with its nested `x`/`y`/`width` adjustments, and the three
calls this unit defines above (`pgtk_draw_glyph_string_bg_rect`, `pgtk_draw_image_foreground`,
`pgtk_draw_image_relief`).  Rust: 3,293 -> **3,369** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 43 errors / 19 warnings -> 43 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 44 / 20 -> 44 / 20
C = A + use_gtk + have_gtk3                        : 21 / 18 -> 21 / 18
```
**Every arm reads exactly the same number on both sides and both sets diff empty** - the third slice
in a row whose text adds no error and no warning (the two promotions and four pointer tests of
Deviation 34 were right the first time).  **Arm reach is measured** (P-86 ⑦): injecting
`let _: c_int = true;` as the first statement of `pgtk_draw_image_glyph_string` moves arm A to 46
errors, anchored at that line (the injected statement was then removed and the file re-measured at
3,369 lines).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **154 rows -> 153**: `pgtk_draw_image_glyph_string` retires, and the file's own
`line count drifted` row is re-measured 57.7% -> **56.7%** (C 7,780 vs the port's 3,369).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2063/36/0**; `check-dead-file-coverage.py` **145/0** (**44 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The thirteenth slice (C 2361-2455): the stretch glyph string

**What is in it.**  `pgtk_draw_stretch_glyph_string` (C 2363-2455): C's `eassert` over the glyph
record's `type`, the `DRAW_CURSOR && !x-stretch-cursor` arm with its two `reversed_p` halves
(`window_box_left_offset`/`window_box_right`), the `width` fold, the cursor rectangle, the "clear the
rest" arm with `mouse_face_p`/`cursor_in_mouse_face_p`/`face->background`, the clip-rectangle pair and
the `stipple` fork, plus the second arm's `text_left_x` guard and the closing
`background_filled_p = true`.  Rust: 3,369 -> **3,494** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 43 errors / 19 warnings -> 44 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 44 / 20 -> 45 / 20
C = A + use_gtk + have_gtk3                        : 21 / 18 -> 22 / 18
```
**Every arm's delta is one registered class and nothing else** - the error sets were diffed:
`cannot find function pgtk_fill_rectangle` goes 11 -> **12** (the one call in C 2430's `else` arm of
the stipple fork), with 0 new warnings and 0 new classes.  **Arm reach is measured** (P-86 ⑦):
injecting `let _: c_int = true;` as the first statement of `pgtk_draw_stretch_glyph_string` moves arm
A to 45 errors, anchored at that line (the injected statement was then removed and the file
re-measured at 3,494 lines).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **153 rows -> 152**: `pgtk_draw_stretch_glyph_string` retires, and the file's own
`line count drifted` row is re-measured 56.7% -> **55.1%** (C 7,780 vs the port's 3,494).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2063/36/0**; `check-dead-file-coverage.py` **145/0** (**45 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The fourteenth slice (C 2457-2522): the dash and the underline filler

**What is in it.**  `pgtk_draw_dash` (C 2462-2481: the clip pair, `cr_segment`'s widening, C's int
sum before the `thickness / 2.0` term, `cairo_set_dash`, the line width, the two path calls and the
stroke) and `pgtk_fill_underline` (C 2487-2522: the `segment = thickness * 3` default and C's
`switch (style)` as the port's `match` over the same labels, with the `FALLTHROUGH` of the
`FACE_UNDERLINE_DOTS` arm carried in that arm and `emacs_abort` for the two remaining labels and the
wildcard).  Rust: 3,494 -> **3,587** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 44 errors / 19 warnings -> 45 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 45 / 20 -> 46 / 20
C = A + use_gtk + have_gtk3                        : 22 / 18 -> 23 / 18
```
**Every arm's delta is one registered class and nothing else** - the error sets were diffed:
`cannot find function pgtk_fill_rectangle` goes 12 -> **13** (the one call in the SINGLE/DOUBLE_LINE
arm of the `match`), with 0 new warnings and 0 new classes.  **Arm reach is measured** (P-86 ⑦):
injecting `let _: c_int = true;` as the first statement of both new functions moves arm A to 49
errors with two diagnostics anchored at those lines (the injected statements were then removed and
the file re-measured at 3,587 lines).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **152 rows -> 150**: the two items retire and the one token finding this slice's `match`
produced is the sanctioned `macro-body-notes.tsv` row (Deviation 36), and the file's own
`line count drifted` row is re-measured 55.1% -> **53.9%** (C 7,780 vs the port's 3,587).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2064/36/0** (the `cairo.h 861-865` row is the new external);
`check-dead-file-coverage.py` **145/0** (**46 range rows** over `src/pgtkterm.c`);
`check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The fifteenth slice (C 2524-2825): the glyph-string dispatcher

**What is in it.**  `pgtk_draw_glyph_string` (C 2524-2825), the file's drawing entry point: the
successor-background loop, the relief/clipping prologue with C's three-way clip choice, the six-arm
`match` over `s->first_glyph->type` (image, xwidget, stretch, char, composite, glyphless, plus the
`emacs_abort` wildcard), the underline epilogue (the wave arm, the shared style from `s->prev`, the
`font_for_underline_metrics` path with `x_underline_at_descent_line` /
`x_use_underline_position_properties` / `underline_minimum_offset`, the sanity clamps, the two
`pgtk_fill_underline` calls for the double line), the overline and strike-through rectangles, the
two `prev`/`next` overhang re-draw loops with `cairo_save`/`cairo_restore`, and the closing stipple
write and clip reset.  Rust: 3,587 -> **3,971** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 45 errors / 19 warnings -> 49 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 46 / 20 -> 50 / 20
C = A + use_gtk + have_gtk3                        : 23 / 18 -> 27 / 18
```
**Every arm's delta is one registered class and nothing else** - the error sets were diffed:
`cannot find function pgtk_fill_rectangle` goes 13 -> **17** (the four calls in the epilogue's
underline/overline/strike-through rectangles), with 0 new warnings and 0 new classes.  **Arm reach is
measured** (P-86 ⑦): injecting `let _: c_int = true;` as the first statement of
`pgtk_draw_glyph_string` moves arm A to 50 errors, anchored at that line (the injected statement was
then removed and the file re-measured at 3,971 lines).  **The first readings named three defects**,
all in this slice's own lines and all fixed (Deviation 37): two extra dereferences in the `prev`/
`next` comparisons and a stipple write that named the field instead of its storage unit.

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **150 rows -> 149**: `pgtk_draw_glyph_string` retires and the one token finding its `match`
produced is the sanctioned `macro-body-notes.tsv` row (Deviation 37); the file's own
`line count drifted` row is re-measured 53.9% -> **49.0%** (C 7,780 vs the port's 3,971).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2065/36/0**; `check-dead-file-coverage.py` **145/0** (**47 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The sixteenth slice (C 2827-2914): the frame cursor, the window-line hook, the area clear and the hollow cursor

**What is in it.**  `pgtk_define_frame_cursor` (C 2829-2836: the `pointer_invisible` read, the
`current_cursor` comparison and the two GDK/GTK calls), `pgtk_after_update_window_line`
(C 2838-2867: the `mode_line_p`/`pseudo_window_p` guard, the `redraw_fringe_bitmaps_p` write, the
five-part condition with its two comma expressions and the two `pgtk_clear_frame_area` calls),
`pgtk_clear_frame_area` (C 2869-2873) and `pgtk_draw_hollow_cursor` (C 2877-2914: the glyph lookup,
the geometry call, the R2L adjustment and the clipped rectangle).  Rust: 3,971 -> **4,117** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 49 errors / 19 warnings -> 50 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 50 / 20 -> 51 / 20
C = A + use_gtk + have_gtk3                        : 27 / 18 -> 28 / 18
```
**Every arm's delta is one new registered class and nothing else** - the error sets were diffed:
`cannot find function pgtk_clip_to_row` (+1, C 3629's definition in the remainder; the Evidence ② table
gains its row), with 0 new warnings.  **Arm reach is measured** (P-86 ⑦): injecting
`let _: c_int = true;` as the first statement of `pgtk_draw_hollow_cursor` moves arm A to 51 errors,
anchored at that line (the injected statement was then removed and the file re-measured at 4,117
lines).  The first reading named one warning - a `///` doc comment on the two new `extern` blocks -
which is `/* */` now (Deviation 38).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **149 rows -> 145**: the four items retire, and the one token finding the hook produced
(C 2849's `= 1`) is the sanctioned `macro-body-notes.tsv` row (Deviation 38); the file's own
`line count drifted` row is re-measured 49.0% -> **47.1%** (C 7,780 vs the port's 4,117).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2065/36/0**; `check-dead-file-coverage.py` **145/0** (**48 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The seventeenth slice (C 2916-3012): the bar cursor

**What is in it.**  `pgtk_draw_bar_cursor` (C 2923-3012): the
`get_phys_cursor_glyph` lookup with its out-of-bounds early return, the xwidget and image early
exits (the image arm calling `draw_phys_cursor_glyph` through `MATRIX_ROW`), the
`FACE_FROM_ID`/`cursor_color` colour choice, `pgtk_clip_to_row`, and the two `kind` arms - the
`BAR_CURSOR` one (`WINDOW_TEXT_TO_FRAME_PIXEL_X`, `FRAME_CURSOR_WIDTH`, the `min` fold, the
`phys_cursor_width` write and the R2L shift) and the `HBAR_CURSOR` one (its three dummy outputs,
the second `min`, `get_phys_cursor_geometry` and the wider rectangle).  Rust: 4,117 -> **4,248**
lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 50 errors / 19 warnings -> 53 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 51 / 20 -> 54 / 20
C = A + use_gtk + have_gtk3                        : 28 / 18 -> 31 / 18
```
**Every arm's delta is two registered classes and nothing else** - the error sets were diffed:
`cannot find function pgtk_clip_to_row` 1 -> 2 and `pgtk_fill_rectangle` 17 -> **19** (the bar and
hbar rectangles), with 0 new warnings.  **Arm reach is measured** (P-86 ⑦): injecting
`let _: c_int = true;` as the first statement of `pgtk_draw_bar_cursor` moves arm A to 54 errors,
anchored at that line (the injected statement was then removed and the file re-measured at 4,248
lines).  **The first reading named four defects**, all in this slice's own lines and all fixed
(Deviation 39): the unit had never carried frame.h's inline `FACE_FROM_ID`/`FRAME_FACE_CACHE` pair,
`cursor_glyph->face_id` is a bit-field, and `MATRIX_ROW` needs the typed matrix cast.

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **145 rows -> 144**: `pgtk_draw_bar_cursor` retires, and the file's own
`line count drifted` row is re-measured 47.1% -> **45.4%** (C 7,780 vs the port's 4,248).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2065/36/0**; `check-dead-file-coverage.py` **145/0** (**49 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round - the new `FRAME_FACE_CACHE`/`FACE_FROM_ID` copies are *definitions* with bodies,
so the tool reports them as definitions and never as duplicates, the same ruling the `src/term.rs`,
`src/dispnew.rs`, `src/font.rs`, `src/alloc.rs`, `src/window.rs`, `src/fringe.rs` and `src/image.rs`
copies rest on; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The eighteenth slice (C 3014-3110): the window cursor and the bit copy

**What is in it.**  `pgtk_draw_window_cursor` (C 3016-3077): the `on_p` guard with the
`phys_cursor_type`/`phys_cursor_on_p` writes, the exact-window-width fringe early path (the
`reversed_p` fork over `phys_cursor.hpos` and the `draw_fringe_bitmap` call), the five-arm `match`
over `text_cursor_kinds` (`HOLLOW_BOX_CURSOR`, `FILLED_BOX_CURSOR`, `BAR_CURSOR`, `HBAR_CURSOR`,
`NO_CURSOR` plus the `emacs_abort` wildcard) and the `XWINDOW (f->selected_window)` IM
cursor-location call; and `pgtk_copy_bits` (C 3079-3110): the similar surface, the source-surface
pair with its two negated offsets, the two clipped paints and the destroy.  Rust: 4,248 -> **4,395**
lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 53 errors / 19 warnings -> 53 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 54 / 20 -> 54 / 20
C = A + use_gtk + have_gtk3                        : 31 / 18 -> 31 / 18
```
**Every arm reads exactly the same number on both sides, and both sets diff empty**: this text adds
no error and no warning - the second slice in this unit (after the tenth) whose repaired reading is a
zero delta, and its two registrable names (`draw_fringe_bitmap`, `pgtk_im_set_cursor_location`) are
mirror rows rather than unresolved ones.  **Arm reach is measured** (P-86 ⑦): injecting
`let _: c_int = true;` as the first statement of both new functions moves arm A to 57 errors with two
diagnostics anchored at those lines (the injected statements were then removed and the file
re-measured at 4,395 lines).  **The first reading named two defects**, both fixed (Deviation 40): the
missing `cairo_rectangle_t` record and the `short`-to-`int` promotion on
`glyph_row->used[TEXT_AREA]`.

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **144 rows -> 142**: the two items retire and the one token finding the `match` produced is the
sanctioned `macro-body-notes.tsv` row (Deviation 40); the file's own `line count drifted` row is
re-measured 45.4% -> **43.5%** (C 7,780 vs the port's 4,395).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2072/36/0** (the cairo.h rows this slice adds are the new externals);
`check-dead-file-coverage.py` **145/0** (**50 range rows** over `src/pgtkterm.c`);
`check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The nineteenth slice (C 3112-3255): the scroll run

**What is in it.**  `pgtk_scroll_run` (C 3114-3255): the `window_box` bounding box, the
`from_y`/`to_y`/`bottom_y` fold with its up and down branches (the mode-line clamps and the
"destination off the top" adjustment), `block_input`, C's whole `#ifdef HAVE_XWIDGETS` block
(C 3160-3243) as this unit's `have_xwidgets` feature arm - the child list, the per-view rectangle
intersection, the clip-top/clip-bottom fold and the GTK move/size/queue calls - `gui_clear_cursor`,
the two `cairo_rectangle_t` records and `pgtk_copy_bits`, and `unblock_input`.  Rust: 4,395 ->
**4,593** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 53 errors / 19 warnings -> 53 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 54 / 20 -> 54 / 20
C = A + use_gtk + have_gtk3                        : 31 / 18 -> 31 / 18
```
**Every arm reads exactly the same number on both sides, and both sets diff empty** - including the
`have_xwidgets` arm, the only one in which C's dead block is compiled at all: this text adds no error
and no warning.  **Arm reach is measured** (P-86 ⑦): injecting `let _: c_int = true;` as the first
statement of `pgtk_scroll_run` moves arm A to 56 errors, anchored at that line (the injected
statement was then removed and the file re-measured at 4,593 lines).  **The first readings named
twelve defects**, all fixed (Deviation 41): eight `E0381`s for C's address-filled locals (zero
initialisation, PORTING.md 1.5) and four names in the xwidgets arm (`XXWIDGET` is a mirror function
rather than a macro, and `XG_XWIDGET_VIEW` needs `.as_ptr ()`).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **142 rows -> 141**: `pgtk_scroll_run` retires, and the file's own `line count drifted` row is
re-measured 43.5% -> **41.0%** (C 7,780 vs the port's 4,593).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2072/36/0**; `check-dead-file-coverage.py` **145/0** (**51 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The twentieth slice (C 3257-3379): the icon pair, the update hook and the vertical border

**What is in it.**  `pgtk_bitmap_icon` (C 3259-3326, whose body is `#if 0` dead text carried as C's
own text plus the live `return false;`), `pgtk_text_icon` (C 3332-3342: a **non-`static`** C
definition, so `#[unsafe(no_mangle)]` and `extern "C"`, with the two GTK window calls),
`pgtk_update_begin` (C 3354-3358: one call to the file's own
`pgtk_clear_under_internal_border`) and `pgtk_draw_vertical_window_border` (C 3362-3379: the
`FACE_FROM_ID_OR_NULL` face lookup and the one-pixel rectangle).  Rust: 4,593 -> **4,758** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 53 errors / 19 warnings -> 53 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 54 / 20 -> 54 / 20
C = A + use_gtk + have_gtk3                        : 31 / 18 -> 31 / 18
```
**Every arm reads exactly the same number on both sides and both sets diff empty**: this text adds no
error and no warning - the fourth zero-delta reading in this unit.  **Arm reach is measured**
(P-86 ⑦): injecting `let _: c_int = true;` as the first statement of
`pgtk_draw_vertical_window_border` moves arm A to 54 errors, anchored at that line (the injected
statement was then removed and the file re-measured at 4,758 lines).  **The first reading named one
defect** - `gtk_window_set_icon`, whose mirror row is a doc comment only - and the repaired reading
is the zero delta above (Deviation 42).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **141 rows -> 137**: the four items retire and the one token finding the cast produced is the
sanctioned `macro-body-notes.tsv` row (Deviation 42); the file's own `line count drifted` row is
re-measured 41.0% -> **38.8%** (C 7,780 vs the port's 4,758).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2072/36/0**; `check-dead-file-coverage.py` **145/0** (**52 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The twenty-first slice (C 3381-3465): the window divider, the update end and the up-to-date hook

**What is in it.**  `pgtk_draw_window_divider` (C 3383-3442: the three divider faces through
`FACE_FROM_ID_OR_NULL`, the three `FRAME_FOREGROUND_PIXEL` fallbacks, `borders_respect_alpha_background`
on every source-colour call and the vertical/horizontal/single rectangle arms), `pgtk_update_end`
(C 3447-3452: the `mouse_face_defer` write) and `pgtk_frame_up_to_date` (C 3454-3465:
`FRAME_MOUSE_UPDATE!`, the `buffer_flipping_blocked_p` guard, `flip_cr_context` and the GTK queue-draw
call).  Rust: 4,758 -> **4,895** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 53 errors / 19 warnings -> 53 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 54 / 20 -> 54 / 20
C = A + use_gtk + have_gtk3                        : 31 / 18 -> 31 / 18
```
**Every arm reads exactly the same number on both sides and both sets diff empty**: this text adds no
error and no warning - the fifth zero-delta reading in this unit.  **Arm reach is measured**
(P-86 ⑦): injecting `let _: c_int = true;` as the first statement of `pgtk_frame_up_to_date` moves
arm A to 56 errors, anchored at that line (the injected statement was then removed and the file
re-measured at 4,895 lines).  **The first readings named three defects**, all fixed (Deviation 43):
`buffer_flipping_blocked_p` and `gtk_widget_queue_draw` (missing declarations) and the seven
`borders_respect_alpha_background` reads, which are the frame record's bit-field and so need the
unit's own accessor.

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **137 rows -> 134**: the three items retire and the one token finding the `mouse_face_defer`
write produced is the sanctioned `macro-body-notes.tsv` row (Deviation 43); the file's own
`line count drifted` row is re-measured 38.8% -> **37.1%** (C 7,780 vs the port's 4,895).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2072/36/0**; `check-dead-file-coverage.py` **145/0** (**53 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round; the consumer sweep is re-read and stays `checked 83 unit(s), 0 failure(s)` with
`checking 83 unit(s), 0 unit failure(s)`).  **Not claimed**: none of this has been run - the frozen
configuration compiles none of this file, there is no GTK or cairo to draw with here, and the three
arm readings are **compile** readings of text this machine cannot build.

## The twenty-second slice (C 3467-3567): the mouse position

**What is in it.**  `pgtk_mouse_position` (C 3487-3567, with C 3467-3485's comment above it): the
`FOR_EACH_FRAME` run that clears `mouse_moved` on every frame of this display (C 3506-3509), the
`gui_mouse_grabbed` branch that takes `last_mouse_frame` unless `track_mouse` is `Qdropping` or
`Qdrag_source` (C 3513-3516), the `else` branch's widget/window/seat/device query, its
`pgtk_any_window_to_frame` conversion and its `SELECTED_FRAME` + `return_frame_flag =
EQ (track_mouse, Qdrag_source)` fallback (C 3517-3536), the terminal-frame guard (`Bug#50322`,
C 3538-3543), the second seat/device query and the `remember_mouse_glyph` /
`last_mouse_glyph_frame` pair with the six outputs (`*bar_window`, `*part`, `*fp`, `*x`, `*y`,
`*timestamp`, C 3545-3564).  Rust: 4,895 -> **5,049** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 53 errors / 19 warnings -> 54 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 54 / 20 -> 55 / 20
C = A + use_gtk + have_gtk3                        : 31 / 18 -> 32 / 18
```
The delta is **exactly one class, once, in every arm** - `cannot find function
pgtk_any_window_to_frame` (C 3526; the definition is C 7314, in the remainder, and C 118's forward
declaration is dropped under PORTING.md 0.1-12), which the Evidence ② table now carries.  Both the
error set and the warning set were diffed line by line against the HEAD copy: no other change, and
**0 new warnings**.  **Arm reach is measured** (P-86 ⑦): injecting `let _: c_int = true;` as the
first statement of `pgtk_mouse_position` moves arm A to 55 errors, anchored at that line (the
injected statement was then removed and the file re-measured at 4,049 -> **5,049** lines).
**The first reading named two defects of this slice's own text**: C 3561-3562's two `XSETINT` calls
pass C's `int` where the mirror's `make_fixnum` wants `EMACS_INT` (two `E0308`s) - the port writes
the cast C performs implicitly (`win_x as EMACS_INT`, the `src/androidterm.rs` 814-815 spelling of
the same site), which is Deviation 44.

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs`
goes **134 rows -> 133**: `pgtk_mouse_position` retires, no `does not mention` row appears, and the
file's own `line count drifted` row is re-measured 37.1% -> **35.1%** (C 7,780 vs the port's 5,049).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2072/36/0**; `check-dead-file-coverage.py` **145/0** (**54 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round, so P-87's two legs do not apply).  **Not claimed**: none of this has been run -
the frozen configuration compiles none of this file, there is no GTK or cairo to query a seat or a
device with here, and the three arm readings are **compile** readings of text this machine cannot
build.  Note also Deviation 13's standing gap: `FRAME_PGTK_P` is the mirror's non-pgtk arm, so a
pgtk build reads `false` at C 3507/3531/3539 until `include/frame.rs` gains that arm.

## The twenty-third slice (C 3569-3645): the fringe-bitmap pair and the row clip

**What is in it.**  `max_fringe_bmp` and `fringe_bmp` (C 3571-3572, the file-scope pair - C's `= 0`
initialiser for the pointer is the port's null), `pgtk_define_fringe_bitmap` (C 3574-3611: the
growth arm with `which + 20` and the `xrealloc` whose result C assigns back, the `while` that nulls
the new tail, then the `CAIRO_FORMAT_A1` surface, the stride/data pair, the per-row
`*((unsigned short *) data) = bits[i]` store with `data += stride`, and the mark-dirty / pattern /
destroy triple), `pgtk_destroy_fringe_bitmap` (C 3613-3626: the early return past the table's end,
the guarded `cairo_pattern_destroy` pair and the closing null store) and `pgtk_clip_to_row`
(C 3628-3645: `window_box`'s five outputs with C's `0` height, the four record assignments with C's
two `max` calls, and the `cairo_rectangle`/`cairo_clip` pair).  Rust: 5,049 -> **5,193** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 54 errors / 19 warnings -> 52 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 55 / 20 -> 53 / 20
C = A + use_gtk + have_gtk3                        : 32 / 18 -> 30 / 18
```
Every arm **loses two and gains none**: both `cannot find function pgtk_clip_to_row` sites retire as
C 3628-3645 is transcribed (the class this table carried since the sixteenth slice), with the error
set and the warning set diffed line by line against the HEAD copy - **0 new errors, 0 new warnings**.

**The retirement unmasked three latent defects of an earlier slice, and that is the reading worth
keeping.**  A `cannot find function` error *inside an item* stops that item before MIR
borrow-checking, so the three `E0381`s this slice's first reading produced - `pgtk_draw_hollow_cursor`
(slice 16) declares `let mut x: c_int;` / `y` / `h` at C 2881 and passes them by `&raw mut` to
`get_phys_cursor_geometry` - had been invisible for seven slices behind the E0425 that sat in the same
function.  A one-line rustc reading settles the rule instead of arguing it: `let mut x: c_int;
g (&raw mut x);` is `E0381`, so the four locals (`x`, `y`, `wd`, `h`) are zero-initialised
(PORTING.md 1.5, Deviation 45).  With that, the delta is exactly the two retirements above in every
arm.  **Arm reach is measured** (P-86 ⑦): injecting `let _: c_int = true;` as the first statement of
`pgtk_define_fringe_bitmap` moves arm A to 53 errors anchored at that line (the injected statement
was then removed and the file re-measured at **5,193** lines).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs` goes
**133 rows -> 128**: five retire (`pgtk_define_fringe_bitmap`, `pgtk_destroy_fringe_bitmap`,
`pgtk_clip_to_row` and the two file-scope objects) and one arrives -
`function pgtk_destroy_fringe_bitmap does not mention 0` - which the sanctioned
`docs/ports/macro-body-notes.tsv` row closes (C's two null sites are the port's `.is_null ()` /
`core::ptr::null_mut ()`; `pgtk_define_fringe_bitmap` keeps its own `0` in C 3598's `i = 0`).  The
file's own `line count drifted` row is re-measured 35.1% -> **33.3%** (C 7,780 vs the port's 5,193).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2081/36/0** (`2072 -> 2081`: this slice's section and the `macro-body-notes.tsv` row
add the references above); `check-dead-file-coverage.py` **145/0** (**55 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round, so P-87's two legs do not apply).  **Not claimed**: none of this has been run -
the frozen configuration compiles none of this file, there is no GTK tree here, and the three arm
readings are **compile** readings of text this machine cannot build.  The cairo side is a real
header read (`/opt/homebrew/include/cairo/cairo.h`) but not a runtime one: no fringe bitmap was ever
drawn.  Deviation 13's standing gap (`FRAME_PGTK_P` is the mirror's non-pgtk arm) is unchanged.

## The twenty-fourth slice (C 3647-3752): the fringe-bitmap draw, the hourglass trio and the empty flush

**What is in it.**  `pgtk_draw_fringe_bitmap` (C 3647-3689: the clip call, the `bx`/`overlay_p`
background fill, the three-part guard (`which`, `where < max_fringe_bmp`, `< max_used_fringe_bitmap`),
the missing-pattern arm with C's comment and the `gui_define_fringe_bitmap` call, the
`Emacs_GC gcv` foreground ternary with its nested `overlay_p` read, and the
`pgtk_cr_draw_image` call), the hourglass run (`hourglass_atimer` and `hourglass_enter_count`,
C 3691-3692; `hourglass_cb`, C 3694-3698, **whose C body is empty**; `pgtk_show_hourglass`,
C 3700-3727, with the four GTK calls, the cursor call and the `hourglass_enter_count++ == 0` timer
arm; `pgtk_hide_hourglass`, C 3729-3746, with `--hourglass_enter_count == 0`, the cancel pair and
the destroy/null pair) and `pgtk_flush_display` (C 3748-3752, **also empty in C**).  Rust: 5,193 ->
**5,348** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 52 errors / 19 warnings -> 52 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 53 / 20 -> 53 / 20
C = A + use_gtk + have_gtk3                        : 30 / 18 -> 30 / 18
```
**Every arm reads exactly the same number on both sides and both sets diff empty**: this text adds no
error and no warning - the sixth zero-delta reading in this unit.  **The first reading named six
defects of one class**, all fixed (Deviation 46): `draw_fringe_bitmap_params`, `get_cursor_p`,
`get_overlay_p` and `max_used_fringe_bitmap` are **inside `pub mod dispextern_unit_names`**, so the
unqualified names are `E0425`; `src/w32term.rs` 1489 is the unit that had measured this before, and
the port now names all four through the module path.  **Arm reach is measured** (P-86 ⑦): injecting
`let _: c_int = true;` as the first statement of `pgtk_draw_fringe_bitmap` moves arm A to 53 errors
anchored at that line (the injected statement was then removed and the file re-measured at **5,348**
lines).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs` goes
**128 rows -> 121**: seven retire (the five functions above and the two file-scope objects) and one
arrives - `function pgtk_draw_fringe_bitmap does not mention cursor_p, overlay_p` - which the
sanctioned `docs/ports/macro-body-notes.tsv` row closes (C's two members are the mirror's accessor
pair on the record's `flags` byte).  The file's own `line count drifted` row is re-measured 33.3% ->
**31.3%** (C 7,780 vs the port's 5,348); the 121 rows are 104 `missing function`, 11 `missing macro`,
3 `missing constant`, 2 `missing file-scope object` and the line-count row.

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2081/36/0** (unchanged from slice 23's re-measurement); `check-dead-file-coverage.py` **145/0** (**56 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round, so P-87's two legs do not apply).  **Not claimed**: none of this has been run -
the frozen configuration compiles none of this file, there is no GTK tree here, no fringe bitmap was
drawn and no hourglass appeared; the three arm readings are **compile** readings of text this machine
cannot build.  Deviation 13's standing gap (`FRAME_PGTK_P` is the mirror's non-pgtk arm) is unchanged.

## The twenty-fifth slice (C 3754-3798): the redisplay interface and `pgtk_clear_frame`

**What is in it.**  C 3754's `extern frame_parm_handler pgtk_frame_parm_handlers[];` (the table
`pgtkfns.c` 963-1015 defines), the **28-entry `pgtk_redisplay_interface`** (C 3756-3785: C's
positional initializer, the three `NULL` slots and the two commented-out hooks) and
`pgtk_clear_frame` (C 3787-3798: the default-face guard, `mark_window_cursors_off` and the
`pgtk_clear_area` pair).  Rust: 5,348 -> **5,458** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 52 errors / 19 warnings -> 52 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 53 / 20 -> 53 / 20
C = A + use_gtk + have_gtk3                        : 30 / 18 -> 30 / 18
```
**Every arm reads exactly the same number on both sides and both sets diff empty**: this text adds no
error and no warning - the seventh zero-delta reading in this unit.  **The first reading named eight
defects in three classes**, all fixed (Deviation 47): the three C macros the mirror carries as
functions (C 3793/3796's `FRAME_ROOT_WINDOW`/`FRAME_PIXEL_WIDTH`/`FRAME_PIXEL_HEIGHT` are
`include/frame.rs`'s `pub fn`s, not `*!` macros), `gui_clear_window_mouse_face`'s missing use-site
declaration, and the four `E0308`s that settled the interface rule - the fifteen `static` definitions
whose address the object takes are `unsafe extern "C" fn` (PORTING.md §1.4), which is why five
entries could be written `Some (name)` directly.  **Arm reach is measured** (P-86 ⑦): injecting
`let _: c_int = true;` as the first statement of `pgtk_clear_frame` moves arm A to 53 errors anchored
at that line (the injected statement was then removed and the file re-measured at **5,458** lines).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs` goes
**121 rows -> 119**: two retire (`pgtk_redisplay_interface` and `pgtk_clear_frame`), no
`does not mention` row appears, and the file's own `line count drifted` row is re-measured 31.3% ->
**29.8%** (C 7,780 vs the port's 5,458).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2081/36/0**; `check-dead-file-coverage.py` **145/0** (**57 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round - PORTING.md §1.4 and this ledger are the only non-`femacs/` files - so P-87's
two legs do not apply).  **Not claimed**: none of this has been run - the frozen configuration
compiles none of this file, the interface object's 28 pointers are never *called* here (the struct
is one more static the redisplay code would read), and the three arm readings are **compile**
readings of text this machine cannot build.  Deviation 13's standing gap (`FRAME_PGTK_P` is the
mirror's non-pgtk arm) is unchanged.

## The twenty-sixth slice (C 3800-3922): the visible bell and the frame flash

**What is in it.**  `recover_from_visible_bell` (C 3800-3813, the atimer callback that undoes the
flash), `pgtk_flash` (C 3815-3905: the context guard, the similar surface, the five `cairo_*` calls,
the flash rectangles built from the frame's pixel/line/border/margin readings, and the timer pair)
and `pgtk_ring_bell` (C 3907-3922, `visible_bell`'s fork).  Rust: 5,458 -> **5,628** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 52 errors / 19 warnings -> 50 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 53 / 20 -> 51 / 20
C = A + use_gtk + have_gtk3                        : 30 / 18 -> 28 / 18
```
Every arm **loses two and gains none**: `pgtk_free_frame_resources`'s two `cancel_atimer` calls (this
unit, C 529/536) carried a standing `mismatched types` (`E0308`) since the second slice - the mirror
declares `pgtk_output.atimer_visible_bell`/`scale_factor_atimer` with the bare-tag shape (`*mut
c_void`) where C 422/452 say `struct atimer *` - and this slice repairs them with the P-72 rule 3
pointer cast (Deviation 48, Owed 10).  The error sets and the warning sets were diffed line by line
against the HEAD copy; **the first reading's warning** (`unused doc comment`, the `///` form on an
`extern` block) is fixed in the same slice.  **Arm reach is measured** (P-86 ⑦): injecting
`let _: c_int = true;` as the first statement of `pgtk_flash` moves arm A to 51 errors anchored at
that line (removed, the file re-measured at **5,628** lines).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs` goes
**119 rows -> 116**: the three items retire, no `does not mention` row appears, and the file's own
`line count drifted` row is re-measured 29.8% -> **27.7%** (C 7,780 vs the port's 5,628).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2084/36/0** (`2081 -> 2084`: this slice's own ledger section); `check-dead-file-coverage.py` **145/0** (**58 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round, so P-87's two legs do not apply).  **Not claimed**: none of this has been run -
the frozen configuration compiles none of this file, no bell was rung and no flash was drawn (the
slice's own cairo calls are declarations against this machine's installed header, not a runtime
reading), and the three arm readings are **compile** readings of text this machine cannot build.
Deviation 13's standing gap (`FRAME_PGTK_P` is the mirror's non-pgtk arm) is unchanged.

## The twenty-seventh slice (C 3924-3970): the socket read

**What is in it.**  `pgtk_read_socket` (C 3924-3970, with C 3924-3930's comment above it): the first
`evq_flush` with its early return, the GLib main-context acquire/pending/dispatch/release run around
`block_input`, and the second `evq_flush`.  Rust: 5,628 -> **5,684** lines.

**The arm readings (three arms, both sides, `dispextern_face_records` + `use_cairo` both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 50 errors / 19 warnings -> 50 / 19
B = A + gdk_windowing_wayland + have_xwidgets      : 51 / 20 -> 51 / 20
C = A + use_gtk + have_gtk3                        : 28 / 18 -> 28 / 18
```
**Every arm reads exactly the same number on both sides and both sets diff empty** (the only line the
error-set diff prints is the probe's own `_baseprobe.rs`/`.rs` file name): this text adds no error and
no warning - the eighth zero-delta reading in this unit, and the first one with **nothing declared**:
`evq_flush` is this file's own C 604 definition and the four GLib calls are `include/gtk.rs`
1919-1925's rows.  **Arm reach is measured** (P-86 ⑦): injecting `let _: c_int = true;` as the first
statement of `pgtk_read_socket` moves arm A to 51 errors anchored at that line (removed, the file
re-measured at **5,684** lines).

**What `--check` reads.**  `extract-abi.py --check emacs/src/pgtkterm.c femacs/src/pgtkterm.rs` goes
**116 rows -> 115** (the item retires, no `does not mention` row appears) and the file's own
`line count drifted` row is re-measured 27.7% -> **26.9%** (C 7,780 vs the port's 5,684).

**Gates and what is not claimed.**  `check-declarations.py` **18/0/18/0**; `check-citations.py`
**676/0/0/2084/36/0**; `check-dead-file-coverage.py` **145/0** (**59 range rows** over
`src/pgtkterm.c`); `check-port-lines.py` **0/0/0/0**; `check-ledger-ticks.py` **238/0**;
`check-cond-eval.py src/pgtkterm.c` **31/0/0**; `check-mirror-dupes.py` **16/16/0** (no mirror
changed this round, so P-87's two legs do not apply).  **Not claimed**: none of this has been run -
the frozen configuration compiles none of this file, no socket was read, and the three arm readings
are **compile** readings of text this machine cannot build.  Deviation 13's standing gap
(`FRAME_PGTK_P` is the mirror's non-pgtk arm) is unchanged.
