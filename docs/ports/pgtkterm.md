# emacs/src/pgtkterm.c (7780 lines) — first slice: C 1-1078

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
> `## Deviations`.  Port: `femacs/src/pgtkterm.rs` (**2044 lines**), carrying **C 1-1078** with
> **both arms of every `#if`**.
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
| 1079-7780 | **the remainder of the file** - from C 1080 (`pgtk_set_z_group`, **the next live definition**) through C 7780's `syms_of_pgtkterm`: the scroll-bar layer, `pgtk_read_socket`, the event handlers, `pgtk_term_init`, `pgtk_delete_display` (C 7314), `syms_of_pgtkterm` (C 7472) and the cairo surface helpers | not in this slice: recorded here so the table covers the reference, to be replaced by the next lane's rows |

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
| `cannot find function pgtk_frame_rehighlight` / `pgtk_new_focus_frame` / `pgtk_set_event_handler` / `pgtk_unlink_touch_points` | 4 | this file, later in it (C 4770 / 4835 / 6831 / 6655) - outside the slice |
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

## Owed (2026-10-05, PORTING.md P-90)

- **The SDK surface moves out of the unit.**  `femacs/src/pgtkterm.rs` carries the cairo, GLib,
  GDK and GTK declarations as in-unit segments because it was the surface's first (and, at that
  moment, only) consumer.  With `gtkutil.c` as the second consumer, P-90 requires one dedicated
  mirror - `femacs/src/include/gtk.rs` with its own ledger `docs/ports/include-gtk.md`, the
  `include-xlib.rs` shape - and the in-unit segments moved into it (P-88 ⑤'s move-back).  The
  mirror is not created yet: this file's own slice was landed without it, and the move-back is
  owed to the round that creates it.  Until then the arm's readings carry the declarations twice
  (once per unit), which is harmless across crates and wrong only inside one TU.
