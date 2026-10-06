# emacs/src/pgtkfns.c (3906 lines) — first slice: C 1-1147

> **One of P-86's class: the frozen configuration compiles nothing here.**  `pgtkfns.o` sits in
> `PGTK_OBJ` (`emacs/src/Makefile.in` 325, `PGTK_OBJ=@PGTK_OBJ@`, filled by `configure.ac`
> 3875-3878 - `PGTK_OBJ="pgtkfns.o pgtkterm.o pgtkselect.o pgtkmenu.o pgtkim.o xsettings.o"` under
> `if test "$window_system" = "pgtk"`) and `docs/baseline/frozen-build-sources.txt` lists no pgtk
> object for this target, so the text can never be *compiled* here.  As with `pgtkterm.c` and
> `gtkutil.c`, this file's own `#if`s are all *decided* -
> `python3 tools/check-cond-eval.py src/pgtkfns.c` reads **`11 conditions, 0 unknown, 0
> disagreements [VERIFIED]`** and the file is on `docs/baseline/config/cond-eval-verified.txt` 341 -
> so `extract-abi.py`'s checklist is not *empty* by construction, it is empty of *live* text (the
> file's whole body needs `HAVE_PGTK`); the `--check` reading in Evidence ③ is a coverage statement,
> not a vacuous pass.  What this ledger adds is P-86 ②'s substitute shape - the **C line-range
> table**, accounting for every line of the reference because the arm cannot be compiled on this
> machine - beside the `## Evidence` arm readings and the `## Deviations`.  Port:
> `femacs/src/pgtkfns.rs` (**4,859** lines, from the 11 of the placeholder), carrying **C 1-1147**,
> **C 1149-1196**, **C 1197-1752**, **C 1754-1996**, **C 1999-2208**, **C 2210-2403**,
> **C 2405-2549**, **C 2551-2838**, **C 2840-3025**, **C 3027-3317** and **C 3319-3525** - eleven
> slices - with **both arms of every `#if`**; the next live definition is `Fpgtk_page_setup_dialog`
> (**C 3528**).
>
> **This ledger replaces, not extends, what it said before.**  Until this batch it read "the frozen
> configuration does not compile this file ⇒ the port is the empty unit plus the note that names the
> gate" (the P-08 reading, `docs/status.md` 3.31), which is what the 11-line placeholder was.  P-86 ①
> replaces that with *transcribe the text, both arms, no stub*; what the earlier note measured (not
> in the frozen source list, empty checklist, 336-byte object) is unchanged and still true here - it
> is the *conclusion* that is withdrawn, exactly as `docs/ports/pgtkterm.md`, `docs/ports/gtkutil.md`
> and `docs/ports/w32.md` record for their own files.
>
> **Why 1147 and not 1131.**  The file's item order is `pgtk_default_font_parameter` (C 1074-1131)
> then `update_watched_scale_factor` (C 1133-1147); C 1132 is blank.  The neighbouring whole-item
> boundaries are `pgtk_default_font_parameter`'s `}` at C 1131 (16 short of the slice) and the
> `DEFUN ("pgtk-set-monitor-scale-factor", …)` at C 1155.  **The next live definition is
> `Fpgtk_set_monitor_scale_factor` (C 1155-1196)** - the first item of the next lane's slice
> (C 1149-1154 is the "Lisp definitions" banner and C 1148 is blank).
>
> **The file gate is `have_pgtk`** (`HAVE_PGTK`, `configure.ac` 3176's `AC_DEFINE`) on every item
> *and on the include graph* (P-86 ⑧), and this slice's own conditions are C 776/785's
> `GDK_PIXBUF_VERSION_2_44` `#if` pair (two `#pragma GCC diagnostic` blocks - dead text, no Rust
> spelling) and the four `#ifdef CAIRO_HAS_*` of the export-type chain (C 928/933/942/947), all
> decided `false` by the frozen table.
>
> **Acquisition: this batch is `text-only` (P-86 ③⑵).**  Measured on this machine:
> `ls -d /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0` and
> `/opt/homebrew/include/gdk-pixbuf-2.0` → all `No such file or directory`; `pkg-config` is **not
> installed**; `find /opt/homebrew /usr/local /Library -name gtk.h -path "*gtk*"` → nothing.  So
> there is no GTK/GDK tree to compile the arm against and no layout reading for any GDK/GTK record.
> **cairo and GLib/GObject are present** (`/opt/homebrew/include/cairo`, cairo 1.18.4;
> `/opt/homebrew/include/glib-2.0`, GLib 2.90.0 with `glibconfig.h`), so every cairo and GLib
> declaration in the port quotes the header and line it was read from and
> `cairo_surface_type_t`'s layout is a real clang reading (Evidence ②) - which is why the toolkit
> layer is *declared* (P-88 ①) rather than left as a bare gap.  The missing toolchain is named in
> `## Owed`.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`PGTK_OBJ` ⇔ `window_system = pgtk`) | `feature = "have_pgtk"` (on every item *and* on C 21-37's include graph) | a pure GTK3 (pgtk) build |
| C 23-24 `<math.h>`, `<c-strcase.h>` (gnulib mirrors) | `have_pgtk` | a pgtk build |
| C 776-781 / C 785-789 `#if (defined GDK_PIXBUF_VERSION_2_44 && …)` around two `#pragma GCC diagnostic` | no arm: C's text, recorded (a pragma has no Rust spelling) | never on this target |
| C 928 `#ifdef CAIRO_HAS_PDF_SURFACE` (`if (NILP (type) || EQ (type, Qpdf))` / `else`) | `cairo_has_pdf_surface` on the arm chooser (Deviation 4) | a cairo with the PDF surface |
| C 933 `#ifdef CAIRO_HAS_PNG_FUNCTIONS` (the `Qpng` arm) | `cairo_has_png_functions` | a cairo with PNG functions |
| C 942 `#ifdef CAIRO_HAS_PS_SURFACE` (the `Qpostscript` arm) | `cairo_has_ps_surface` | a cairo with the PS surface |
| C 947 `#ifdef CAIRO_HAS_SVG_SURFACE` (the `Qsvg` arm) | `cairo_has_svg_surface` | a cairo with the SVG surface |

The four cairo feature names are **not registered** in `docs/baseline/feature-coverage.tsv` (Owed 4).
`have_ext_menu_bar`/`have_ext_tool_bar` are the arm `lisp.h` 3931-3939 gives a GTK build and this
tree already has (`include/frame.rs` gates `FRAME_EXTERNAL_*_BAR` on them); the slice *writes* both
accessors, and the unit's write-arm macros name the mirror's `FRAME_EXTERNAL_*_BAR_FLAG_*` constants,
which exist only under those two features (Owed 6).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's opening comment (title, copyright, licence) | dropped (P-86 ①; the port opens with `// femacs: port of src/pgtkfns.c @ fd7afced403.`) |
| 21 | `#include <config.h>` | ungated (every unit includes it) |
| 23-24 | `<math.h>`, `<c-strcase.h>` | file gate |
| 26-37 | `lisp.h`, `blockinput.h`, `gtkutil.h`, `window.h`, `character.h`, `buffer.h`, `keyboard.h`, `termhooks.h`, `fontset.h`, `font.h`, `xsettings.h`, `atimer.h` | file gate |
| 39-41 | C's two forward declarations (`x_decode_color`, `pgtk_display_info_for_name`) | file gate; dropped (PORTING.md 0.1-12) |
| 43 | `pgtk_app_name` | file gate |
| 45-46 | `monitor_scale_factor_alist` | file gate |
| 48-52 | the "Internal utility functions" banner | file gate |
| 54-73 | `pgtk_get_monitor_scale_factor` | file gate |
| 75-109 | `check_pgtk_display_info` | file gate |
| 111-129 | the Wayland comment and `is_wayland_display` | file gate |
| 131-164 | `pgtk_display_info_for_name` | file gate |
| 166-172 | the "Frame parameter setters" banner | file gate |
| 173-197 | `pgtk_set_foreground_color` | file gate |
| 199-222 | `pgtk_set_background_color` | file gate |
| 224-240 | `pgtk_set_alpha_background` | file gate |
| 242-251 | `pgtk_set_border_color` | file gate |
| 253-295 | `pgtk_set_cursor_color` | file gate |
| 297-315 | `pgtk_set_name_internal` | file gate |
| 317-350 | `pgtk_set_name` | file gate |
| 352-361 | `pgtk_explicitly_set_name` (with its comment) | file gate |
| 363-372 | `pgtk_implicitly_set_name` | file gate |
| 374-395 | `pgtk_set_title` | file gate |
| 397-435 | `pgtk_set_menu_bar_lines` | file gate |
| 437-459 | `pgtk_set_tab_bar_lines` | file gate |
| 461-523 | `pgtk_change_tab_bar_height` | file gate |
| 525-548 | `x_change_tool_bar_height` | file gate |
| 550-574 | `pgtk_set_tool_bar_lines` | file gate |
| 576-599 | `pgtk_set_child_frame_border_width` | file gate |
| 601-617 | `pgtk_set_internal_border_width` | file gate |
| 619-647 | `pgtk_set_icon_type` | file gate |
| 649-679 | `pgtk_set_icon_name` | file gate |
| 681-685 | `pgtk_set_cursor_type` | file gate |
| 687-690 | `pgtk_set_mouse_color` (C's own empty body) | file gate |
| 692-701 | `pgtk_set_undecorated` | file gate |
| 703-712 | `pgtk_set_skip_taskbar` | file gate |
| 714-729 | `pgtk_set_override_redirect` | file gate |
| 731-767 | `xg_set_icon` | file gate |
| 768-800 | `xg_set_icon_from_xpm_data` (C 776-781 and 785-789 the two `#if`/`#pragma` blocks) | file gate / dead text |
| 802-813 | `pgtk_set_sticky` | file gate |
| 815-831 | `pgtk_set_tool_bar_position` | file gate |
| 833-864 | `pgtk_set_scroll_bar_foreground` | file gate |
| 866-897 | `pgtk_set_scroll_bar_background` | file gate |
| 899-960 | the "Printing" banner and `DEFUN ("x-export-frames", …)` (C 928-956 the four `#ifdef CAIRO_HAS_*_SURFACE` arms) | file gate / the four `cairo_has_*` features |
| 962-1016 | `pgtk_frame_parm_handlers[]` (C 963's redundant `extern` re-declaration of the object C defines on 964) | file gate |
| 1018-1043 | `unwind_create_frame` (with its comment) | file gate |
| 1045-1049 | `do_unwind_create_frame` | file gate |
| 1051-1072 | `x_decode_color` (with its comment) | file gate |
| 1074-1131 | `pgtk_default_font_parameter` | file gate |
| 1133-1147 | `update_watched_scale_factor` - **this slice's last definition** | file gate |
| 1149-1196 | the "Lisp definitions" banner (C 1149-1153) and `DEFUN ("pgtk-set-monitor-scale-factor", …)` (C 1155-1196: the `CHECK_STRING`/`CHECK_NUMBER` gates, the `FIXNUMP`/`FLOATP` split with its two identical `error` texts, the `Fassoc` lookup and the `Fcons`/`Fsetcdr` pair on `monitor_scale_factor_alist`) | the file gate - **transcribed** (the second slice) |
| 1197-1752 | `Fx_create_frame` - the file's largest item: the terminal/display lookup and `check_pgtk_display_info`, the `minibuffer` four-way fork and the `parent_frame` test, the `undecorated`/`override_redirect` reads, the frame's officialisation (`XSETFRAME`, the two css providers, the colour reference-count block, `parent_desc`/`explicit_parent`, the `specbind`), the two font drivers, the parameter chain and `init_frame_faces`, the two `adjust_frame_size` calls, `xg_create_frame_widgets`/`pgtk_set_event_handler` and the fifteen `INSTALL_CURSOR` installs, the frame's own reference counts and `Vframe_list`, the GTK re-parenting and the `GList` walk, `initialize_frame_menubar`/`xg_wm_set_size_hint`, the visibility block, the trailing alist loop and the scale-factor atimer | the file gate - **transcribed** (the third slice; C 1753 is the blank line between it and the restack pair) |
| 1754-1996 | the restack pair and the GSETTINGS block: C 1754-1757's comment and `pgtk_frame_restack` (C 1758-1764), `DEFUN ("pgtk-frame-restack", …)` with C's own 15-line docstring (C 1766-1791), and C 1793-1996's `#ifdef HAVE_GSETTINGS` region - the five `#define`s, the three `static inline` predicates, `parse_resource_key`, `pgtk_check_resource_key_length`, `pgtk_get_defaults_value`, `pgtk_set_defaults_value`, the five `#undef`s and the `#else` arm's own pair | the file gate plus `have_gsettings` for the `#if` arm (the `#else` arm is the frozen configuration's) - **transcribed** (the fourth slice) |
| 1999-2208 | the resource subr, the eleven `x-*` display subrs and `pgtk-font-name`: `Fpgtk_set_resource` (C 1999-2019), `x-server-max-request-size`/`x-display-screens` (2022-2039), the two GDK monitor walks `x-display-mm-height`/`x-display-mm-width` (2042-2107), `x-display-backing-store`/`x-display-visual-class`/`x-display-save-under` (2110-2133), `x-open-connection`/`x-close-connection` (2136-2172), `x-display-list` (2175-2186) and `pgtk-font-name` (2188-2208) | the file gate - **transcribed** (the fifth slice) |
| 2210-2403 | the miscellaneous group's first five definitions and the first seven colour/geometry subrs: C 2210-2214's banner, `check_x_display_info` (2216-2221), `pgtk_set_scroll_bar_default_width`/`_height` (2223-2241), `pgtk_get_string_resource` (2243-2268), `pgtk_get_focus_frame` (2270-2281) and `xw-color-defined-p`/`xw-color-values`/`xw-display-color-p`/`x-display-grayscale-p`/`x-display-pixel-width`/`x-display-pixel-height` (2283-2403) | the file gate - **transcribed** (the sixth slice) |
| 2405-2549 | the monitor-attribute builder and the group's last three definitions: `Fpgtk_display_monitor_attributes_list` (C 2405-2510: the `FOR_EACH_FRAME` filing walk, the per-monitor records and the `make_monitor_attribute_list`/`free_monitors` pair), `pgtk_frame_scale_factor` (2512-2531), `x-display-planes` (2533-2539) and `x-display-color-cells` (2542-2549) | the file gate - **transcribed** (the seventh slice) |
| 2551-2838 | the tooltip group: C 2551-2553's banner, the six file-scope statics (2555-2573), `unwind_create_tip_frame` (2576-2587) and `pgtk_create_tip_frame` (2590-2838: the frame's officialisation, the colour/name blocks, the two font drivers, the thirteen `gui_default_parameter` calls, the GTK window setup, the three `AUTO_FRAME_ARG` blocks and the tail) | the file gate - **transcribed** (the eighth slice) |
| 2840-3025 | `compute_tip_xy` (C 2846-2943: the four parameter reads, the GDK pointer query, the monitor walk and the two coordinate ladders) and `pgtk_hide_tip` (C 2946-3025: the `Qcancel_timer` call, C's five-condition early return, the two specbinds, the `xg_hide_tooltip` attempt and the delete-or-hide fork) | the file gate - **transcribed** (the ninth slice) |
| 3027-3317 | `Fx_show_tip` (C 3027-3309: both `ENABLE_CHECKING` arms, the `'start_timer` label block, the `tip_frame` fork, the tooltip frame's setup, the two `Fset` writes and the `FRAME_TOTAL_COLS` sizing) and `Fx_hide_tip` (C 3312-3317) | the file gate - **transcribed** (the tenth slice) |
| 3319-3525 | `frame_geometry` (C 3325-3414: the three edge branches and the association list), `Fpgtk_frame_geometry` (C 3416-3458), `Fpgtk_frame_edges` (C 3460-3481), `Fpgtk_set_mouse_absolute_pixel_position` (C 3483-3502) and `Fpgtk_mouse_absolute_pixel_position` (C 3504-3525) | the file gate - **transcribed** (the eleventh slice) |
| 3527-3906 | **the remainder of the file** - from C 3528's `DEFUN ("pgtk-page-setup-dialog", …)` (**the next live definition**) through C 3906's `syms_of_pgtkfns`: the page-setup/get-page-setup/print-frames dialogs, `Fx_file_dialog`, the backend-display-class and font subrs, the GTK debug and launch-uri pairs (3528-3774) and `syms_of_pgtkfns` (3775-3906) | not in these slices: recorded here so the table covers the reference, to be replaced by the next lane's rows |

## Evidence

**① The frozen configuration (`build-tu`, two runs).**
`cd femacs && CARGO_TARGET_DIR=/tmp/pgtkfns-lane/cargo cargo run -q -p xtask -- build-tu src/pgtkfns.c`
-> the first run **failed**: `error: expected identifier, found keyword `type`` twice at
`build/femacs/units/pgtkfns/_root.rs` 452/457, because two use-site declarations spelled
`gui_display_get_arg`'s/`gui_default_parameter`'s last parameter `type` where the C header calls it
`enum resource_types` and Rust needs the raw identifier.  The fix (`r#type`) is in the port; the
second run is
-> `built src/pgtkfns.rs -> ../build/femacs/pgtkfns.o`, **exit 0**, `md5 dbed68837694579185377dbb0821ab62`,
`build/femacs/pgtkfns.o` **336 bytes** - an empty object, which is the expected reading here and not a
proof of the new text: every item sits behind `have_pgtk`, so under the frozen feature set rustc
compiles only `include/config.rs` and the `#![allow]` attribute.  **Budget: the mandate allowed three
`build-tu` runs and this batch used two** (the failed one above counts).

**② The arm readings (three, `text-only`, P-86 ③⑵).**  The `have_pgtk` arm cannot be compiled on a
GTK target here (no GTK tree), so the readings below are the unit's own text compiled with the arm's
cfgs by rustc, on the root `build-tu` materialises
(`build/femacs/units/pgtkfns/_root.rs`):

```
$ rustc --edition 2024 --crate-type=lib --emit=metadata -O build/femacs/units/pgtkfns/_root.rs \
    -o /tmp/pgtkfns-lane/narrow.meta --cfg 'feature="have_pgtk"' --cfg 'feature="have_window_system"'
exit 1; 67 lines matching ^error  (66 errors + rustc's `aborting due to 66 previous errors`)

$ ... + --cfg 'feature="use_gtk"' --cfg 'feature="have_gtk3"' --cfg 'feature="have_ext_menu_bar"' \
      --cfg 'feature="have_ext_tool_bar"' --cfg 'feature="gdk_windowing_wayland"' \
      --cfg 'feature="have_xwidgets"'
exit 1; 63 lines matching ^error  (62 errors + the abort line)
```

Classified, **every** error is one of six named things, and **none** is a defect of this slice's own
statement text:

| class | count (narrow / wide) | anchor |
| --- | --- | --- |
| `cannot find type pgtk_output` | 43 / 43 | `include/pgtkterm.rs` (the placeholder mirror: the record is pgtkterm.h 286-455) - the errors are anchored at this unit's use sites, which is what a name-resolution error means |
| `cannot find type pgtk_display_info` | 9 / 9 | `include/pgtkterm.rs` (pgtkterm.h 146-281) |
| `cannot find value FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT` | 2 / 0 | `include/frame.rs` (the constant exists only under `have_ext_menu_bar`) |
| `cannot find value FRAME_EXTERNAL_TOOL_BAR_FLAG_SHIFT` | 2 / 0 | `include/frame.rs` (`have_ext_tool_bar`) - the two rows are what the third reading (above) removes |
| `evaluation panicked: size_of::<font>…` (and 7 more `font`/`font_driver` offsets) | 8 / 8 | **`include/font.rs` 137/149/577-589**: those assertions are the *frozen* arm's measurements and the mirror's `#[cfg(feature = "have_window_system")]` members change the record - the same row `docs/ports/pgtkterm.md` records |
| `unstable library feature c_variadic` | 1 / 1 | **`include/lisp.rs` 3756**: `verror`'s `core::ffi::VaList<'_>` parameter |
| `mismatched types` (`FRAME_SCALE_FACTOR (f)`, C 1137) | 1 / 1 | **`include/frame.rs` 961**: the mirror carries the frozen TTY arm (`-> c_int`, `1`) while `frame.h` 1057-1061's `HAVE_PGTK` arm returns `pgtk_frame_scale_factor (f)` (`double`).  The port keeps C's own token and the error stands as the mirror's (Owed 5) |

The last three mirror rows are the *first* time those mirror arms have ever been compiled (the frozen
configuration defines neither `have_window_system` nor `have_xwidgets`); they are the integrator's
(see `## Owed`), exactly as `docs/ports/pgtkterm.md`'s Evidence ② records for the same three files.

**③ `extract-abi.py --check`.**  `python3 tools/extract-abi.py --check emacs/src/pgtkfns.c
femacs/src/pgtkfns.rs` -> `FAIL`, **109 `- ` lines**: 101 `missing …` (all of them the unported
remainder of the 3906-line file - P-86 ⑩: the check stops being vacuous as soon as the port has a
`fn`, and the comparison is against the file's *whole* inventory), plus **7 token lines** and the
drift line:

```
- function check_pgtk_display_info does not mention 0            (C 86's `x_display_list != 0`)
- function pgtk_display_info_for_name does not mention 0         (C 162's `if (dpyinfo == 0)`)
- function pgtk_set_cursor_color does not mention 0              (C 272's `FRAME_X_WINDOW (f) != 0`)
- function pgtk_explicitly_set_name does not mention true        (C 360's `pgtk_set_name (f, arg, true)`)
- function pgtk_implicitly_set_name does not mention false       (C 371's `pgtk_set_name (f, arg, false)`)
- function pgtk_set_menu_bar_lines does not mention update_mode_line     (C 421's `->update_mode_line = true`)
- function x_change_tool_bar_height does not mention update_mode_line    (C 535's same)
- line count drifted 51.4%: C 3906 vs Rust 1899
```

The five `0`/`true`/`false` rows are the pointer-null and C-`int`-boolean spellings
(`docs/ports/token-aliases.tsv` 94's `NULL` family: the pointer tests are the sanctioned
`.is_null ()`, and C's `true`/`false` are the `1`/`0` a `c_int` parameter takes); the two
`update_mode_line` rows are the `struct window` bitfield C 421/535 *write*, which the mirror exposes
only through `wset_update_mode_line (w)` (`include/window.rs` 328, the same setter `src/window.rs`
2056 uses).  Both are `docs/ports/macro-body-notes.tsv` rows, and that file is not on the porter's
writable list (P-86 ⑩) - Owed 9.  The line-drift row is Owed 10.

**④ The self-checks.**  `python3 tools/check-cond-eval.py src/pgtkfns.c` ->
`11 conditions, 0 unknown, 0 disagreements [VERIFIED]`;
`python3 tools/check-dead-file-coverage.py` -> this ledger's table is bound and passes;
`python3 tools/check-declarations.py` -> `10 declaration sites (0 forbidden, 10 allowed literals),
0 failures`; `python3 tools/check-mirror-dupes.py` -> `19 duplicate declaration(s), 19 ratchet
row(s), 0 failure(s)`; `python3 tools/check-ledger-ticks.py` -> 0 failures (the checklist is empty
for this file, so nothing is bound).

**⑤ The readings the declarations were copied from** (the port's `/// C:` lines name each):
`/opt/homebrew/include/cairo/cairo.h` 2563-2589 (`cairo_surface_type_t`, the enum) and 3244
(`cairo_region_t`); `/opt/homebrew/include/glib-2.0/glib/gtypes.h` 52/55/56/109,
`glib/glibconfig.h` 80 (`gssize`, from `/opt/homebrew/lib/glib-2.0/include/glibconfig.h`),
`glib/gerror.h` 43/208, `gobject/gobject.h` 515.  **Measured** (the one layout claim in the port):

```
$ clang -std=gnu23 -I /opt/homebrew/include/cairo /tmp/cairoprobe.c -o /tmp/cairoprobe && ...
sizeof_surface_type=4
PDF=1 IMAGE=0 PS=2 SVG=10
```

GTK's, GDK's and gdk-pixbuf's records and entry points carry **the header alone, no line**, because
no such tree exists here (above); the four records (`GdkPixbuf`, `GtkCssProvider`, `GtkWidget`,
`GtkWindow`, `GdkWindow`, `GError`) are the never-dereferenced `[u8; 0]` shape (`src/xfns.rs`
1004-1009) and **their layout is UNVERIFIED** - the missing toolchain is named in `## Owed`.

## Deviations

1. **The licence block is not carried** (C 1-19, P-86 ① / PORTING.md 0.1-19): the port opens with
   `// femacs: port of src/pgtkfns.c @ fd7afced403.`
2. **C's forward declarations are dropped** (C 39-41; PORTING.md 0.1-12): Rust has no forward
   declaration and its items are order-independent, so both definitions stay at C's line (C 1055 and
   C 133, both inside this slice).  C 963's `extern frame_parm_handler pgtk_frame_parm_handlers[];`
   is the same class - a redundant re-declaration of the object C defines on the next line - and the
   definition is written once.
3. **The toolkit surface stands in this unit** (P-88 ①, P-90 ①): `<gtk/gtk.h>` and the
   `<gdk/gdk.h>`/`<gdk-pixbuf/gdk-pixbuf.h>`/`<glib-object.h>`/`<cairo.h>` chain C reaches through
   `"gtkutil.h"` have no file in the reference tree and no mirror, so 31 declarations (6 records, 5 GLib
   scalar typedefs, `cairo_surface_type_t` with its four constants and `cairo_region_t`, the
   `GTK_WINDOW` macro, and 13 entry points) stand at C 28's position rather than in a new file under
   `femacs/src/include/`.  cairo's and GLib's are read from this machine's headers (Evidence ⑤);
   **GTK, GDK and gdk-pixbuf are not on this machine**, so those cite the header alone, their records
   are the never-dereferenced shape and every layout claim about them is UNVERIFIED.  P-90 ③'s move
   into one `femacs/src/include/gtk.rs` is owed (Owed 3).
4. **The export-type chain's four `#ifdef`s are an arm chooser** (C 928/933/942/947).  C's text is
   `#ifdef CAIRO_HAS_PDF_SURFACE / if (…) …; else / #endif` four times, so the statement C compiles
   is a chain of exactly the arms whose cairo macro is defined, ending in C 957's `error`.  Rust
   cannot leave the `else` of one `#[cfg]` arm to be supplied by the next arm - an attribute on the
   `if` would carry the whole statement away with it - so each arm's *presence* is that feature's
   compile-time `cfg!` on C's own chain, written once in C's order: a disabled arm takes C's `false`
   and its body is never reached, so the run-time chain and the compiled text are C's for every
   enablement.  This is the arm-chooser idiom `docs/ports/gtkutil.md`'s Deviation 4 records for
   `xg_set_geometry` (`take_body`).  The four feature names are unregistered (Owed 4).
5. **C's implicit integer and pointer conversions are written as `as` casts at the use site**
   (P-72 rule 3 / the `src/xfns.rs` shape, `docs/ports/gtkutil.md`'s Deviation 5): the three
   `x_decode_color` results (`int` → `unsigned long`, C 180/211/266/274), `cdef.pixel` (`c_ulong` →
   `int`, C 1071), `XFIXNUM (value)`/`XFIXNAT (value)` (an `EMACS_INT` → the local `int`, C 407/452),
   `XFIXNUM (cdr)` (`EMACS_INT` → `double`, C 68), `rgb.pixel` (`c_ulong` → `unsigned int`, C 877/889),
   `fset_redisplay (f as *mut c_void)` (C 410/523, the `include/window.rs` 888 spelling),
   `g_object_unref (pixbuf as gpointer)` (C 742/795) and `(*t).display_info.pgtk as *mut
   pgtk_display_info` (C 98, the mirror's union members are `*mut c_void`, P-72 rule ③).
6. **C's `f->output_data.pgtk->FIELD` keeps C's tokens and spells the union's cast** (P-72 rule ③,
   the `src/pgtkterm.rs` 764-774 shape): `(*((*f).output_data.pgtk as *mut pgtk_output)).FIELD`
   (C 264, 419, 429, 533).
7. **Read/write accessor pairs.**  `include/frame.rs` carries C's getters as functions, so where C
   *writes* one the port keeps the getter as the read arm and adds a two-arm `macro_rules!` of the
   same name whose write arm is C's member (or the mirror's `frame_set_flag`, for the `bool_bf`
   ones): `FRAME_MENU_BAR_LINES`/`_HEIGHT`, `FRAME_TAB_BAR_LINES`/`_HEIGHT`,
   `FRAME_TOOL_BAR_LINES`/`_HEIGHT`, `FRAME_FOREGROUND_PIXEL`, `FRAME_BACKGROUND_PIXEL`,
   `FRAME_EXTERNAL_MENU_BAR`, `FRAME_EXTERNAL_TOOL_BAR`, `FRAME_UNDECORATED`, `FRAME_SKIP_TASKBAR`,
   `FRAME_OVERRIDE_REDIRECT` (C 180/211/416/417/419/429/432/486/487/528/529/534/543/695/707/718);
   the `!` in an invocation is the port's spelling of C's assignment (the `src/pgtkterm.rs` 794-812
   reading).  The four `bool_bf` members C reaches as `f->NAME` - `explicit_name` (C 331/335/341),
   `tab_bar_redisplayed` (C 509), `tab_bar_resized` (C 494/508), `tool_bar_resized` (C 546/560) - get
   the same two-arm member-named macros over the mirror's `frame_flag`/`frame_set_flag` (the private
   `macro_rules!` family `src/frame.rs` 238-290 keeps for its own crate root).
8. **The 23 `static` setters whose address lands in `pgtk_frame_parm_handlers[]` are
   `unsafe extern "C" fn`** (C 967-1013).  A Rust fn item is the Rust ABI and does not unify with the
   array's `Option<unsafe extern "C" fn (…) >` element type (`E0308 expected "C" fn, found "Rust"
   fn`, measured in Evidence ②'s first draft); the `src/androidmenu.rs`
   `android_unwind_local_frame` and `src/callproc.rs` `call_process_kill` readings are the same rule
   (PORTING.md 1.4).  They stay non-`pub` (C's `static`).  `do_unwind_create_frame` (C 1045) is the
   same shape for `record_unwind_protect`'s parameter.
9. **`pgtk_set_mouse_color`'s body is empty** (C 687-690).  That is the reference's own text, not a
   stub the port introduced; the port keeps the empty body (P-86 ①: the C line is transcribed).
10. **The window bitfield C writes becomes the mirror's setter.**  C 421/535 read
    `XWINDOW (FRAME_SELECTED_WINDOW (f))->update_mode_line = true;`; `include/window.rs` 328 carries
    `wset_update_mode_line (w)` (the same write path `src/window.rs` 2056 uses), so the port calls it.
    The two token rows this leaves are Owed 9.
11. **`return 0;` in a `double` function** (C 63/66) is `return 0.0;` - C's `int`-constant return,
    converted by C's own promotion (the `docs/ports/token-aliases.tsv` family).
12. **`FRAME_EXTERNAL_MENU_BAR (f) == 1` (C 429) is `== true`**: the mirror's getter returns `bool`,
    while C's macro yields the `bool_bf` bit (an `int`).  The test is C's.
13. **C's `unsigned int` cast in the two CSS formats** (C 877/889) is `(rgb.pixel as c_uint) & 0xffffff`.
14. **The two `#if`/`#pragma GCC diagnostic` blocks** (C 776-781, 785-789) are kept as C's text and a
    comment: a pragma has no Rust spelling and the condition is decided `false`.
15. **C's `if`/`else` assignment pairs and its reassigned parameters.**  Where C assigns a local in
    both arms of an `if`, the port writes the `let` expression the `src/frame.rs` 665 shape uses
    (`pgtk_set_menu_bar_lines`'s `nlines` at C 406-409, `pgtk_set_tab_bar_lines`'s at C 452-455,
    `pgtk_set_child_frame_border_width`'s `border` at C 582-587, `pgtk_change_tab_bar_height`'s
    `lines = 1` at C 480-481); where C *reassigns a parameter* the port rebinds it
    (`let mut name = name;` in `pgtk_set_name` C 340/342/346 and `pgtk_set_title` C 388/391, and
    `let mut frames = frames;` in `Fx_export_frames` C 912-913) - the `src/term.rs` shape.  The
    four `x->cursor_xgcv`/`x` writes of `pgtk_set_cursor_color` (C 272-289) keep C's order.
16. **`Time` is not named by this slice** - it is `include/termhooks.rs`'s record member (`timestamp`)
    that needs it, and C reaches `"systime.h"` through `termhooks.h` 28.  The port adds that mirror to
    its graph (`include!("include/systime.rs")`) rather than leave a self-inflicted gap; it is the
    P-72 rule 4 remedy, not this file's include list.

17. **C's two l-value macros this slice writes are C's own member paths; C's `FRAME_OUTPUT_DATA`
    keeps its name.**  `FRAME_X_OUTPUT (f) = xzalloc (sizeof *FRAME_X_OUTPUT (f))` (C 1308),
    `FRAME_DISPLAY_INFO (f) = dpyinfo` (C 1335) and `FRAME_GTK_OUTER_WIDGET (f) = NULL` (C 1628) are
    assignments *into* a macro C uses as an l-value, and this unit's copies of those three carry the
    read arm only (the `FRAME_FOREGROUND_PIXEL!`/`FRAME_UNDECORATED!` pair below is the same file's
    rule: an arm is added where a call site needs it), so each write is C's own member path
    (`(*f).output_data.pgtk = xzalloc (std::mem::size_of::<pgtk_output> ())`,
    `(*FRAME_X_OUTPUT! (f)).display_info = dpyinfo`, `(*FRAME_X_OUTPUT! (f)).widget = …`) - the
    `src/androidfns.rs` 1222 shape.  The seven `FRAME_OUTPUT_DATA (f)->… = NULL;` writes (C 1629-1635)
    *do* keep C's macro name, because this slice adds the read-arm macro.  The record's size in the
    `xzalloc` comes from C's `sizeof` of the expression, i.e. the record's own size.
18. **C's implicit conversions are written as `as` casts at the use site (Deviation 5's rule, this
    slice's instances).**  The six `-1` writes into `unsigned long` members (C 1314-1319, 1322-1330)
    are `(-1 as c_long) as c_ulong`; the six `x_decode_color (f, black, …)` results are
    `… as c_ulong` (C 1322-1330, an `int` function into `unsigned long` members); `XFIXNAT (parent)`
    into the `Window` member is `XFIXNAT (parent) as Window` (C 1339); `explicit_parent`'s two
    `bool`s into the `char` member are `true as c_char`/`false as c_char` (C 1340/1344, the
    `src/abi_shim.rs` 811 shape); and `dpyinfo->terminal->kboard` is cast to the record it names
    (`*mut c_void` in `include/termhooks.rs` 220, P-72 rule ③).
19. **C's assignments hidden in a condition stand on their own line.**  C 838-843 and C 847-852
    write `tem = (gui_display_get_arg (…))` *inside* `if (!NILP (…))`; Rust's assignment is a
    statement, so the call is its own line and the `if` tests the value (the chained-assignment note
    in `src/frame.rs`; `src/androidfns.rs` 1203 is the same function's precedent).
20. **`f->output_method = output_pgtk;` (C 1308) is this unit's own writer pair.**  The mirror
    carries the reader (`FRAME_PGTK_P`, `include/frame.rs` 882) and the shift/mask, but
    `frame_output_method` is private to it, so `set_output_method!`/`output_method!` stand here with
    C's names (the `src/androidfns.rs` 283-297 device).
21. **The bit-field writes C spells as plain assignments are macros here.**  `f->no_split = …`
    (C 1533), `f->can_set_window_size = true;` (C 1706) and the three `f->was_invisible` writes
    (C 1731/1742/1745) go through `no_split!`, `can_set_window_size!` and `was_invisible!`, each
    through the mirror's `frame_set_flag` and the mirror's own `_SHIFT`/`_UNIT` constants (the
    `FRAME_UNDECORATED!` shape this file already uses).
22. **`KVAR (kb, Vdefault_minibuffer_frame)` (C 1745-1746) is written as the member.**  keyboard.h
    38's macro is a token paste (`(kboard)->field ## _`) and `include/keyboard.rs` 71 keeps the
    member's C name `Vdefault_minibuffer_frame_`, so the port writes the member (the
    `src/androidfns.rs` 1555 shape).
23. **`INSTALL_CURSOR` (C 1577-1591) cannot paste `GDK_` onto its second argument.**  Rust has no
    `##`, so the `macro_rules!` takes the pasted constant and each of the fifteen call sites passes
    it (`GDK_XTERM`, `GDK_LEFT_PTR`, …) - the `src/xfns.rs` treatment of the same C macro.  The
    macro's own metavariables keep C's names (`$field`, `$name`) and its body keeps C's
    `FRAME_X_DISPLAY` (this slice's read-arm macro) and `gdk_cursor_new_for_display`.  The token gap
    this leaves is *recorded*, not silent: two rows of `docs/ports/macro-body-notes.tsv`
    (`INSTALL_CURSOR` and `Fx_create_frame`) list exactly the tokens C's paste produces and the
    reason, so every *other* token of both bodies is still compared (Owed 12).
24. **The fifteen GDK cursor constants are `include/gtk.rs`'s, as placeholders.**  C 1580-1594 names
    `GDK_XTERM`/`GDK_HAND2`/`GDK_WATCH`/`GDK_SB_*`/`GDK_*_SIDE`/`GDK_*_CORNER` (fourteen distinct
    names) and the mirror carried only `GDK_LEFT_PTR`; **no GDK header exists on this machine**, so
    they are added in the same placeholder sense the mirror's existing GDK constants use, with the
    real values owed to a GTK toolchain (`docs/ports/include-gtk.md`).
25. **`update_watched_scale_factor` (C 1133-1134, the first slice's item) is now
    `unsafe extern "C" fn`.**  C 1755-1756 passes it to `start_atimer`, whose parameter is
    `Option<unsafe extern "C" fn (*mut atimer)>` (`include/atimer.rs` 29's `atimer_callback`), and a
    Rust `fn` item is the Rust ABI and does not unify - the same reason `do_unwind_create_frame`
    (this file, C 1045) is spelled that way.  Until this slice nothing took its address, so the first
    slice could leave it a plain `unsafe fn`; the signature is C's and nothing else about it changes.
26. **This slice's own declarations stand in this unit** (P-72 rule 4, the device the first slice set
    up): `fset_parent_frame` (frame.h 774-776, C's `INLINE` setter), `xg_create_frame_widgets`
    (gtkutil.h 181), `pgtk_set_event_handler` (pgtkterm.h 598) and `pgtk_iconify_frame`
    (pgtkterm.h 642), plus the six frame.h/dispextern.h/font.h entry points (`make_frame`,
    `make_minibuffer_frame`, `make_frame_without_minibuffer`, `delete_frame`,
    `gui_figure_window_size`, `init_frame_faces`, `register_font_driver`) and the two `font.h`
    driver objects (`ftcrfont_driver`, `ftcrhbfont_driver` under `harfbuzz`, C 1389-1391's `#ifdef`),
    each with C's line - because the mirrors that own them (`include/frame.rs`, `include/gtkutil.rs`,
    `include/pgtkterm.rs`, `include/font.rs`) do not carry them yet (Owed 1/2/7).

27. **C's `(void) 0;` NOPs are `let _ = 0;`** (C 1863/1867/1886/1894, `parse_resource_key`'s four
    "lower/numeric -> NOP" arms).  C's cast-to-void expression evaluates `0` and drops it; the port
    keeps the expression as a statement (`let _ = 0;`), so an empty block - which would have dropped
    C's expression - is not used (PORTING.md 2.1).
28. **C's post-increment in a condition is the load-then-step pair** (C 1877's `if (*sp++ != '.')`):
    the port loads `*sp` into a local, steps `sp` by one, and then tests the loaded value - the two
    statements C's post-increment performs (PORTING.md 2.2; the `pbm_next_char` shape of
    `src/image.rs` 11108).  C's `*dp++ = c;` (C 1870/1891/1898) and `sp++` (C 1850/1871/1899) are
    written the same way.
29. **C's implicit pointer conversions are written as `as` casts (Deviation 5's rule, this slice's
    instances)**: `g_object_unref (gs as gpointer)` (C 1952/1973) and `g_free (str_ as gpointer)`
    (C 1953) hand a `GSettings *` and a `gchar *` to the mirror's `gpointer` parameters; C's
    `NULL` for `g_settings_new_full`'s backend (C 1917) is `std::ptr::null_mut ()`.
30. **C's function-scope `static char holder[128]` (C 1948) keeps its storage duration.**  The port
    writes a function-scope `static mut HOLDER: [c_char; 128]` (zero-initialised once - PORTING.md
    0.1-13's tentative-definition half) and takes its address with `addr_of_mut!` rather than a
    reference, because Rust 2024's `static_mut_refs` refuses the reference form (the `src/alloc.rs`
    588 shape).  The local `holder` pointer is then exactly C's array name decaying, so the
    parameter C passes on is unchanged.
31. **The GSETTINGS `#define`s and `#undef`s.**  C 1795-1800's five `#define`s are `const` items;
    C 1976-1980's five `#undef`s have no Rust counterpart (an item is not un-defined, PORTING.md
    0.1).  `PATH_MAX_LEN`'s `max (sizeof PATH_FOR_CLASS_TYPE, sizeof PATH_PREFIX_FOR_NAME_TYPE)`
    cannot be a call in an array's length, so it is written as the constant C's expression denotes -
    **measured**: the two byte strings are 34 and 33 bytes including the NUL, so C's `max` is 34
    (and C's array is `char path[34 + 128]`, i.e. 162).
32. **The restack DEFUN's `\(stacking)` (C 1778) is spelled as its *value* `(stacking)`.**  C's
    `\(` is in a `doc: /* … */` comment, and make-docfile's escape processing turns `\(` into `(`
    (`tools/cinventory.py` 935's `doc_comment_value`), so the value the reference documents has no
    backslash; the port's literal carries that value.  **Measured**: with the literal written
    `\\(stacking)` (C's spelling carried through Rust's own escaping) `tools/extract-abi.py --check`
    reads `subr Fpgtk_frame_restack: docstring differs`; with the value it reads no row (Evidence
    ⑮).
33. **The GSETTINGS `#if`/`#else` pair defines the same two names.**  C's `#if` arm (C 1930-1974)
    and its `#else` arm (C 1984-1994) both define `pgtk_get_defaults_value` and
    `pgtk_set_defaults_value`, so the port carries one `#[cfg]`-selected definition each -
    `have_gsettings` for the `#if` arm and `not (feature = "have_gsettings")` for the live `#else`
    one.  `pgtk_get_defaults_value` is non-`static` in both (pgtkterm.h 583 declares it) and so
    carries `#[unsafe(no_mangle)]` (PORTING.md 0.2); `pgtk_set_defaults_value` is `static` in both
    and stays an `unsafe fn`.  The `#else` arm's unused parameters are `let _ = …;` (the
    `src/print.rs` 1693 shape), which is what keeps the arm's reading warning-free.
34. **The GIO surface stands in this unit** (P-88 ①'s third-party segment; the `src/xsettings.rs`
    286-370 device): C 1793's `#ifdef HAVE_GSETTINGS` reaches `<gio/gio.h>` through `"gtkutil.h"`
    26's `<gtk/gtk.h>` chain, and that header has no mirror, so the three opaque records
    (`GSettings`, `GSettingsSchema`, `GSettingsSchemaSource`) and the eight entry points
    (`g_settings_schema_source_get_default`, `g_settings_schema_source_lookup`,
    `g_settings_schema_has_key`, `g_settings_schema_unref`, `g_settings_new_full`,
    `g_settings_get_string`, `g_settings_set_string`, `g_settings_reset`) are declared here with the
    header and the line each was read from.  **GLib and GIO are on this machine**
    (`/opt/homebrew/include/glib-2.0`, GLib 2.90.0), so every one of them is a read citation and no
    record is dereferenced (`[u8; 0]`, the opaque shape) - and `include/gtk.rs`, which carries the
    GLib/GObject half (`g_free`, `g_object_unref`, `TRUE`), does **not** carry GIO's, which is why
    the segment stands here rather than there (Owed 13).

36. **A `///` citation above a `DEFUN!` invocation is an `unused doc comment`.**  This slice's nine
    `DEFUN!`s were first written with `///` lines above them (the shape the declarations in this
    file use), which added **seven warnings in all four arms** (measured: 11 -> 18 narrow, 10 -> 17
    wide); a macro invocation takes no doc comment, so every citation above a `DEFUN!` here is a
    block comment (`/* … */`) - the same fix the second and third slices recorded, and the reason
    this slice's own reading is warning-free.
37. **C's `for (i = 0; i < n_monitors; ++i)` is the file's `i = 0; while … { … i += 1; }`** (C 2055
    and C 2089, the two monitor walks; the `src/androidfns.rs` 1552 shape for C's `for` over an
    `int` counter).
38. **C's `max (a, b)` over two `int`s** (C 2065/2099) is the mirror's `max (a: isize, b: isize)
    -> isize` (`include/lisp.rs` 3149) with both conversions written and the result narrowed -
    `max (height_mm_at_0 as isize, mm as isize) as c_int` (the `src/abi_shim.rs` 2457 shape; the
    macro `lisp.h` spells cannot be a call in Rust).
39. **C's uninitialised `GdkRectangle rec;` (C 2057/2091) is zero-initialised.**  C 2060/2094 fills it
    through a pointer, so the port writes `let mut rec: GdkRectangle = std::mem::zeroed ();` and
    passes `&raw mut rec` (the `x_decode_color` C 1058 shape for C's uninitialised locals; the
    `src/pgtkterm.rs` 1391 shape for `&raw mut`).
40. **C's index reads and its `== 0` pointer test keep C's own `0` tokens.**  C 2011's `attr[0]` and
    C 2199's `nm[0]` are `*attr.offset (0)` / `*nm.offset (0)` (a raw pointer's element, the
    `src/alloc.rs` 3967 shape), and C 2148's `if (dpyinfo == 0)` is
    `dpyinfo == 0 as *mut pgtk_display_info` (the `src/w32term.rs` 2680 shape).  **Measured**: with
    `*attr`/`*nm`/`dpyinfo.is_null ()` the checker reads three rows - `subr Fpgtk_font_name does not
    mention 0`, `subr Fpgtk_set_resource does not mention 0`, `subr Fx_open_connection does not
    mention 0` (Evidence ㉑) - so the spelling here is the one that keeps C's tokens *and* C's
    meaning, and no `macro-body-notes.tsv` row is needed.
41. **C's `(void)` parameter list and C's unused parameters.**  C 2177's `(void)` is the port's `()`
    (the `src/haikufns.rs` 3133 shape), `Fx_display_visual_class`'s `terminal` (C 2122, never read in
    C's body) and `Fx_open_connection`'s `resource_string` (C 2138, never read either) are
    `let _ = …;` with C's reason in a comment (the `src/print.rs` 1693 shape).
42. **`include/gtk.rs` gains the GDK monitor surface and `g_strdup_printf`** (a *mirror* change, so
    P-87's two legs apply this round): `GdkMonitor` (opaque, `gdk/gdkmonitor.h`) and the five entry
    points C 2053-2096 calls, plus `g_strdup_printf` (C 2014; `gstrfuncs.h` 286, a machine reading -
    GLib is on this machine, GDK is not, so the five GDK rows cite the header alone).
    **Measured**: `tools/check-all-units.sh` reads `checked 83 unit(s), 0 failure(s)` with
    `checking 83 unit(s), 0 unit failure(s)`, `check-mirror-dupes.py` stays 16/16/0, and
    `docs/ports/include-gtk.md` names all six rows in its GDK/GLib bullets and its new
    `### status 3.718` section.

43. **The four `FRAME_CONFIG_SCROLL_BAR_*` macros are this unit's own two-arm copies.**  C writes
    all four as l-values (C 2229/2230/2239/2240) and `include/frame.rs` mirrors each as a reader
    only (1286/1291/1296/1301 in frame.h), so C's assignment shape stands here under C's names - the
    same device `src/androidfns.rs` 342-350 and `src/w32fns.rs` 3362-3372 use for their own copies.
44. **C's `unsigned short` colour members widened** (C 2309): `list3i (col.red, col.green, col.blue)`
    is `list3i (col.red as EMACS_INT, col.green as EMACS_INT, col.blue as EMACS_INT)`, the
    conversion C makes implicitly (Deviation 5's rule; `Emacs_Color`'s three members are `c_ushort`
    in `include/dispextern.rs` 28-30 and `list3i` takes `EMACS_INT`).
45. **C's `false` handed to an `int` parameter is `false as c_int`** (C 2292/2308): C's own
    `pgtk_defined_color (f, SSDATA (color), &col, false, false)` passes `false` twice, but C's
    prototype's fifth parameter is `int index` and the mirror keeps that type
    (`include/pgtkterm.rs`'s declaration, this file's own row 314) while Rust has no implicit
    conversion.  **Measured**: with a bare second `false` all four arms read **two new `E0308`s**
    (`expected `i32`, found `bool``, one per call site); `false as c_int` keeps C's token *and*
    C's value, and the arms return to the single `E0308` they started with (the C 1137
    `FRAME_SCALE_FACTOR` row).
46. **C's floating-point conversions in the two monitor walks are written out** (Deviation 5's
    rule): C 2345/2383's `double scale = 1;` is `1.0`; C 2353/2391's
    `scale = gdk_monitor_get_scale_factor (monitor)` widens an `int` to `double` and is
    `… as c_double`; C 2354-2357/2392-2395's four `rec.x = rec.x * scale + 0.5;` assignments
    truncate a `double` back into an `int` and are `(rec.x as c_double * scale + 0.5) as c_int`
    (each with C's own expression inside).
47. **The non-`static` definitions of this slice carry `#[unsafe(no_mangle)]` and C's ABI**
    (PORTING.md 0.2): `check_x_display_info` (C 2216-2221), `pgtk_set_scroll_bar_default_width`/
    `_height` (2223-2241), `pgtk_get_string_resource` (2243-2268) and `pgtk_get_focus_frame`
    (2270-2281), each an `unsafe extern "C" fn` with C's parameter types.  C's unused parameters
    are `let _ = …;` with C's reason (`pgtk_get_string_resource`'s `rdb`, C 2245 - the GSettings
    path never reads it - and `x-display-grayscale-p`'s `terminal`, C 2324), and `XrmDatabase`
    is `include/pgtkterm.rs`'s (xlib.h's record, reached through this file's ungated `xlib.rs`
    edge).

48. **C's function-scope `static const char *source = "Gdk";` (C 2430) is Rust's function-scope
    `static mut`.**  C's static has static storage duration; Rust's `static` of a raw-pointer type
    would need `Sync` (which `*const c_char` is not), so the port writes a function-scope
    `static mut source: *const c_char` with C's own value - the same device Deviation 30 records
    for C 1948's `static char holder[128]`.
49. **C's `&monitors[i]` (C 2461) is the element's address, not a reference**:
    `let mi: *mut MonitorInfo = monitors.offset (i as isize);` (Rust 2024's `static_mut_refs`-safe
    shape for C's array element; everything else about the record goes through `(*mi)`).
50. **`Emacs_Rectangle`'s two `unsigned` members** (`include/dispextern.rs` 72-73) take C's four
    `int` assignments (C 2488/2489/2492/2493) with the conversion written out - `rec.width as c_uint`
    - while the two `GdkRectangle`s C computes in stay `c_int` (GDK's own `gint` members).
51. **C's implicit conversions in this slice, written out (Deviation 5's rule)**: C 2436's
    `make_nil_vector (n_monitors)` widens a `gint` to `EMACS_INT`; C 2437's
    `xcalloc (n_monitors, sizeof *monitors)` is `xcalloc (n_monitors as size_t,
    std::mem::size_of::<MonitorInfo> ())` with the result cast to `*mut MonitorInfo`; C 2477-2484's
    eight `rec.x = rec.x * scale + 0.5;`-family assignments are the same double-to-int truncation
    Deviation 46 records, and C 2548's `1 << min (dpyinfo->n_planes, 24)` folds the mirror's `min`
    over two `isize`s with C's own shift kept.
52. **This slice's three GDK calls went into the mirror** (`gdk_display_get_monitor_at_window` C
    2450/2521, `gdk_monitor_is_primary` C 2465, `gdk_monitor_get_workarea` C 2471 - `include/gtk.rs`
    is the GDK surface's single home, and no GDK header exists here, so all three cite the header
    alone), so P-87's two legs apply: `tools/check-all-units.sh` reads `checked 83 unit(s), 0
    failure(s)` with `checking 83 unit(s), 0 unit failure(s)`, `check-mirror-dupes.py` stays 16/16/0,
    and the mirror's ledger carries the three rows with a `### status 3.720` section.  The one new
    declaration this unit needed of its own is `dupstring` (lisp.h 5555, this file's own block).

53. **The three frame bitfields this slice writes are this unit's own two-arm macros.**
    `f->wants_modeline = false;` (C 2623) and `f->tooltip = true;` (C 2639) are written through a
    new `wants_modeline!` and `tooltip!` (`no_split!` already stands above), each forwarding to the
    mirror's `frame_set_flag` with the mirror's own `_UNIT`/`_SHIFT` - the `src/haikufns.rs` 496-507
    device for the same two bits.
54. **`face_change` (C 2606/2834) is declared in this unit.**  The mirror carries it inside
    `include/dispextern.rs`'s `dispextern_unit_names` module (`dispextern.h` 2068), which this
    unit's include graph does not glob-import, so the row stands in this unit's own block (the
    `src/haikufns.rs` 857 shape).  **Measured**: without it all four arms read a new class -
    `cannot find value face_change` x2 - and with it the arms return to the classes the ledger
    already names (Evidence ㉜).
55. **The tooltip group's six file-scope statics are C's *tentative* definitions, i.e. zero**
    (C 2556/2560/2564/2567/2570/2573; PORTING.md 0.1-13), and each keeps C's own comment above it -
    this file's `monitor_scale_factor_alist` shape.  `unwind_create_tip_frame` (C 2576-2587) is
    `unsafe extern "C" fn` because C 2625 hands it to `record_unwind_protect`, whose parameter is
    `Option<unsafe extern "C" fn (Lisp_Object)>` (the `do_unwind_create_frame` device above).
56. **`pgtk_create_tip_frame` (C 2599-2838) reuses the devices `Fx_create_frame` established** -
    `output_method!`, the `FRAME_X_OUTPUT!` member paths for C's l-value macros (C 2634/2641/2642/
    2643/2738/2755), `explicit_name!`, the written-out `-1`/`x_decode_color` conversions (C
    2636/2637/2653-2663), `AUTO_FRAME_ARG!` for C's three `AUTO_FRAME_ARG` blocks (C 2774/2789/
    2809) and `calln!` for C's `calln (Qface_set_after_frame_default, …)` (C 2805) - so nothing in
    it is a new shape; its own `parms` reassignment (C 2611) is the `Fpgtk_set_monitor_scale_factor`
    shadow.
57. **The mirror gains the four `gtk_window_set_*` rows and `GdkWindowTypeHint`** (C 2749-2754; no
    GDK/GTK header on this machine, so all cite the header alone and the enum's one named member is
    a placeholder), so P-87's two legs apply: `tools/check-all-units.sh` reads `checked 83 unit(s),
    0 failure(s)` with `checking 83 unit(s), 0 unit failure(s)`, `check-mirror-dupes.py` stays
    16/16/0, and the mirror's ledger carries the rows with a `### status 3.721` section.

58. **C's `int`-and-`EMACS_INT` arithmetic in `compute_tip_xy` is written out** (Deviation 5's rule):
    C's `XFIXNUM` returns an `EMACS_INT`, so C 2888/2889's `max_x = min_x + XFIXNUM (…);` widens
    and then narrows again - `(min_x as EMACS_INT + XFIXNUM (Fnth (…))) as c_int` - and the four
    comparisons and four assignments of C 2916-2942 follow the same shape
    (`*root_y as EMACS_INT + XFIXNUM (dy) <= min_y as EMACS_INT`,
    `*root_y = (*root_y as EMACS_INT - height as EMACS_INT - XFIXNUM (dy)) as c_int`).
59. **The out-parameters keep C's pointer shape.**  `compute_tip_xy`'s `int *root_x`/`int *root_y`
    are `*mut c_int` here (`gdk_device_get_position`'s third and fourth arguments, C 2872), and the
    two `XFIXNUM` results written through them are the narrowing casts of Deviation 58;
    `pgtk_hide_tip`'s `bool delete` and `compute_tip_xy`'s two `int` sizes keep C's own types.
60. **This slice's declarations**: `pgtk_display_pixel_width`/`pgtk_display_pixel_height`
    (pgtkterm.h 601/602; `src/pgtkterm.rs` 1355 defines the first, the second is that unit's
    remainder) and `xg_hide_tooltip` (gtkutil.h 216; `src/gtkutil.rs` 1399/1438 defines it twice,
    one per arm) stand in this unit's own blocks because their mirrors have not landed those rows.
61. **The mirror change retired two copies in another unit.**  `src/xwidget.rs` declared
    `gdk_display_get_default_seat` and `gdk_seat_get_pointer` itself (its C 883/914 and C 908), and
    the moment this round's rows landed in `include/gtk.rs` - which is in *that* unit's include
    graph - `check-mirror-dupes.py` read `18 duplicate declaration(s), 16 ratchet row(s), 2
    failure(s)` and named both.  The two copies are dropped there (P-26's one spelling per
    construct), the gate returns to 16/16/0, and that unit's own probe
    (`have_xwidgets + have_window_system`) compiles clean (exit 0, 0 errors, 0 warnings) - P-87 ①'s
    consumer sweep was re-run on the final tree and reads `checked 83 unit(s), 0 failure(s)`.
62. **This unit's `docs/ports/line-drift-tolerance.tsv` row** (0.10, `total` basis): the port is
    now 4,278 lines against C's 3,906, **+372 lines (+9.5%)**, which the 5% rule reports as a
    `line count drifted` row.  The row and its reason (the fixed cost of a P-86 unit's per-item
    gates and citations, C's two-arm macros and written-out conversions, C's `for` heads as the
    init/`while`/step trio, and the two third-party segments and declaration blocks this unit
    carries) stand in that table, and the `--check` row retires with them (67 -> 59 -> 57 rows
    across this slice's steps, the last two being the drift row's return and its retirement).

63. **`FRAME_TOTAL_COLS` is a two-arm macro here.**  C 3235's `FRAME_TOTAL_COLS (w) = …` assigns, and
    `include/frame.rs` carries the macro as a reader only, so this unit shadows it with C's own body
    in two arms (`macro_rules! FRAME_TOTAL_COLS`, frame.h 1180) - the `FRAME_CONFIG_SCROLL_BAR_*`
    shape of Deviation 40, and the same reason that mirror's reader stays.  The two window bitfields
    this slice writes (`pseudo_window_p` at C 3209, `must_be_updated_p` at C 3297) get the same
    two-arm treatment from `include/window.rs`'s `_SHIFT` constants (window.h 437/438).
64. **Two `goto start_timer`s become a labelled block.**  C 3088 and C 3112 jump to C 3304's
    `start_timer:` label, which sits between the `if`/`else` chain and the tail C 3306-3309; Rust has
    no `goto`, so the slice writes `'start_timer: { … break 'start_timer; … }` with the tail outside
    the block (PORTING.md 2.2's device, `src/haikufns.rs`'s copy of the same function).
65. **The two `ENABLE_CHECKING` arms are `#[cfg]`-gated verbatim bodies.**  C 3042-3044 and
    C 3256-3269 are `#ifdef ENABLE_CHECKING`/`#else` pairs whose false arms are empty; both arms are
    transcribed (P-86 ①) under `#[cfg (feature = "enable_checking")]`, and C 3043's declaration
    `struct glyph_row *row, *end;` is two `let`s - it compiles even though no `glyph_row` record is
    in this unit's include graph, because the arm is dead in every measured configuration (the
    four-arm reading below is what says so, not the absence of a row here).
66. **C's implicit conversions and l-value forms, written out.**  C 3192's assignment to a global
    inside `NILP (…)` becomes an assignment statement plus the test; `compute_tip_xy`'s out-parameters
    are `&raw mut root_x`/`&raw mut root_y` at C 3082 (P-83's raw-borrow spelling);
    `f->output_data.pgtk->current_cursor` (C 3291) is written as the member path so C's
    `output_data`/`pgtk` tokens survive the accessor; and C 3224/3225's `XFIXNAT` narrowing and
    C 3274's `make_fixnum (w->pixel_height)` widening carry `as c_int` / `as EMACS_INT`.
67. **The first four-arm run was not clean, and its five syntax errors hid the rest.**  C 3140, C 3144,
    C 3166, C 3169 and C 3198 are single-statement bodies of an `else` or an `if` (C's own text has no
    braces); Rust needs them, and the five sites are `{ … }` now.  The probe's first reading of the
    finished text was **A 190 / B 178 / C 190 / D 178** with four `expected `{`` errors - and because
    a syntax error truncates its block's parse, that reading also *hid* more than twenty downstream
    diagnostics.  The repair is why the final readings are higher, not lower: **A 197 / B 185 /
    C 197 / D 185**, i.e. **+10 over HEAD in every arm**, all of them the unit's registered
    `cannot find type` class (`pgtk_output`, the slice's own accessor expansions) with **no new class
    and no new warning**.
68. **Four items the slice calls are declared or spelled here.**  `try_window` (C 3252; dispextern.h
    3573, `src/xdisp.rs` defines it) and `set_window_buffer` (C 3207; window.h 1139-1140,
    `src/window.rs`) stand in this unit's extern blocks with their C citations - no mirror carries
    them; `SET_TEXT_POS` (dispextern.h 291-294) is carried as a one-arm `macro_rules!` copy because
    `include/dispextern.rs` keeps it inside its `dispextern_unit_names` module, where a `use` cannot
    reach a macro (`src/haikufns.rs` 2064-2072's shape, `src/xdisp.rs` 1918-1926's before it); and the
    two call sites take the port's spellings - `flush_frame! (tip_f)` (C 3299: frame.h 1835's INLINE is
    a macro in the mirror) and `try_window (…) != 0` (C 3252: C's `int` into a `bool`, written out).
69. **This unit's `docs/ports/line-drift-tolerance.tsv` row moves 0.10 -> 0.20** with the final
    measurement: 4,671 lines against C's 3,906 is **+19.6%** (Deviation 62 recorded the 0.10 reading
    of the 4,278-line state).  The row's reason list grows by this slice's share - the arms it adds to
    existing `macro_rules!` pairs, the five brace sites of Deviation 67, the declaration and macro
    copies of Deviation 68 and the per-site citations - and the `--check` drift row stays retired at
    that tolerance.

70. **`FRAME_WIDGET` is a unit macro.**  C 3492/3514 read `FRAME_WIDGET (f)`, which pgtkterm.h
    485-487 defines as the outer widget if there is one and the edit widget otherwise; this unit's
    `FRAME_GTK_OUTER_WIDGET!`/`FRAME_GTK_WIDGET!` are its own (`include/pgtkterm.rs` is not in its
    include graph, P-72 rule 4), so the three-arm macro is spelled here against them.
71. **The pgtkterm.h toolbar chain is spelled here too.**  C 3362 reads `FRAME_TOOLBAR_HEIGHT (f)`,
    which pgtkterm.h 504-509 define as `toolbar_top_height + toolbar_bottom_height` through
    `FRAME_X_OUTPUT`; the two field macros and the sum stand in this unit against its own accessor.
    Measured: the macros alone add nothing - the probe on the tree with only them appended reads the
    same **197 errors / 11 warnings** as HEAD (the increase below is the slice body's).
72. **C's implicit conversions, written out (26 sites).**  `make_fixnum` takes an `EMACS_INT` and the
    geometry values are `int`, so every `make_fixnum` argument in `frame_geometry` and the two mouse
    subrs carries `as EMACS_INT` (C 3372-3413, C 3524), and C 3499's `gdk_device_warp (…, XFIXNUM
    (x), …)` hands an `EMACS_INT` to a `gint` parameter - written `XFIXNUM (x) as gint`.  The first
    reading of this slice's text was **A 235 / B 223 / C 235 / D 223** with 21 distinct
    mismatched-type sites; the repairs are what the final readings measure.
73. **Three mirror rows, one retired duplicate, and the tolerance row.**  `gtk_window_get_position`
    (gtk/gtkwindow.h), `gdk_window_get_display` (gdk/gdkwindow.h) and `gdk_device_warp`
    (gdk/gdkdevice.h) landed in `include/gtk.rs` (header-only citations: no GTK/GDK header on this
    machine), and `check-mirror-dupes.py` immediately read `17 duplicate declaration(s), 1 failure`
    because `src/pgtkselect.rs` declared `gdk_window_get_display` itself (its C 958) - that copy is
    dropped under P-26's one spelling per construct, its call site unchanged, and that unit's own
    probe reads **9 errors / 8 warnings** on both sides of the drop.  This unit's
    `line-drift-tolerance.tsv` row moves 0.20 -> 0.25 with the final measurement (4,859 lines
    against C's 3,906 is **+24.4%**), which retires the `line count drifted` row the `--check`
    reading brought back.

## Owed (2026-10-05, PORTING.md P-90)

1. **`include/pgtkterm.rs` is a placeholder and owes the records.**  `pgtk_output` (pgtkterm.h
   286-455) and `pgtk_display_info` (146-281) are the 52 name errors of Evidence ②; the `FRAME_*`
   accessors the unit declares at the use site (pgtkterm.h 474/477/478/483/484/490/517/518/526-529)
   and the sixteen `pgtk_*` prototypes it calls (pgtkterm.h 575/579/588/592/597/605-608/611/619/639/640/641/648/675)
   and `x_display_list` (284) belong in the same mirror; `docs/ports/include-pgtkterm.md` is where the
   measurement and the reason are written.
2. **`include/gtkutil.rs` is a placeholder** and owes the six `xg_*` prototypes this slice calls
   (gtkutil.h 158/159/160/183/200/202/205) - and `free_frame_menubar`/`adjust_frame_size`/
   `decode_window_system_frame`/`validate_x_resource_name` (frame.h 1883/1511/1474/1873),
   `check_int_nonnegative`/`set_frame_cursor_types`/`store_frame_param`/`get_frame_param`
   (lisp.h 607/4409/5117/5120), `font_open_by_name` (font.h 883), the 25 `gui_*`
   (frame.h 1844-1871, dispextern.h 3656/3994/4001) and the 13
   `pgtk_*` of Owed 1 should move to their own mirrors when those are completed.
3. **The third-party declarations should move into one `femacs/src/include/gtk.rs`** (P-90 ①/③; the
   `gtkutil.rs`/`pgtkterm.rs` in-unit segments are owed the same move).  The names this unit
   declares, by header: `<glib/gtypes.h>` `gpointer`, `gchar`, `gint`, `gboolean`;
   `<glib/glibconfig.h>` `gssize`; `<glib/gerror.h>` `GError`, `g_error_free`;
   `<gobject/gobject.h>` `g_object_unref`; `<cairo.h>` `cairo_surface_type_t`,
   `CAIRO_SURFACE_TYPE_{IMAGE,PDF,PS,SVG}`, `cairo_region_t`;
   `<gtk/gtk.h>`/`<gtk/gtkwidget.h>`/`<gtk/gtkwindow.h>`/`<gtk/gtkcssprovider.h>` `GtkWidget`,
   `GtkWindow`, `GtkCssProvider`, `GTK_WINDOW`, `gtk_widget_set_app_paintable`,
   `gtk_widget_get_realized`, `gtk_widget_get_window`, `gtk_window_set_icon`,
   `gtk_window_set_title`, `gtk_window_stick`, `gtk_window_unstick`,
   `gtk_css_provider_load_from_data`; `<gdk/gdkwindow.h>` `GdkWindow`,
   `gdk_window_set_opaque_region`; `<gdk-pixbuf/gdk-pixbuf-core.h>` `GdkPixbuf`,
   `gdk_pixbuf_new_from_file`, `gdk_pixbuf_new_from_xpm_data`.  A GTK3 toolchain is what closes the
   layout claims (this machine has none) - the missing resource `docs/goal.md` §9's second clause
   names.
4. **Four unregistered feature names**: `cairo_has_pdf_surface`, `cairo_has_png_functions`,
   `cairo_has_ps_surface`, `cairo_has_svg_surface` (C 928/933/942/947) need
   `docs/baseline/feature-coverage.tsv` rows (Deviation 4).
5. **`include/frame.rs` 961's `FRAME_SCALE_FACTOR` carries only the frozen TTY arm** (`-> c_int`,
   `1`); `frame.h` 1057-1061's `HAVE_PGTK` arm is `(FRAME_PGTK_P (f) ? pgtk_frame_scale_factor (f) :
   1)` (`double`).  This unit's C 1137 is the `E0308` of Evidence ② - a mirror row, not this file's.
6. **`include/frame.rs`'s `FRAME_EXTERNAL_MENU_BAR_FLAG_*` and `FRAME_EXTERNAL_TOOL_BAR_FLAG_*`**
   exist only under `have_ext_menu_bar`/`have_ext_tool_bar`; a pgtk build has both (lisp.h
   3931-3939), so the four `E0425`s of the first arm reading are the narrow cfg's, and the third
   reading (Evidence ②) is what shows them gone.
7. **`include/dispextern.rs` owes `enum resource_types`** (dispextern.h 3983-3992) and
   **`frame_parm_handler`** (dispextern.h 3063): this unit declares both at the use site, and
   `src/frame.rs` 5011-5014 carries a `frame_parm_handler` of its own - two use-site copies that
   become `E0428` when the mirror lands the names.
8. **Three mirror-side rows for the integrator**, found by this batch (they are *not* this file's):
   `include/font.rs` 137/149/577-589 (the eight `E0080` assertions are the frozen arm's
   measurements), `include/lisp.rs` 3756 (`c_variadic`), and `include/xwidget.rs` 203 (the
   `have_xwidgets` `E0133` `docs/ports/pgtkterm.md` records; this slice has no xwidgets gate, so the
   third reading shows no new one).
9. **`docs/ports/macro-body-notes.tsv` rows for the seven token lines of Evidence ③**:
   `check_pgtk_display_info` (`0`), `pgtk_display_info_for_name` (`0`), `pgtk_set_cursor_color`
   (`0`), `pgtk_explicitly_set_name` (`true`), `pgtk_implicitly_set_name` (`false`),
   `pgtk_set_menu_bar_lines` (`update_mode_line`) and `x_change_tool_bar_height`
   (`update_mode_line`).  That file is not on the porter's writable list (P-86 ⑩).
10. **`docs/ports/line-drift-tolerance.tsv`**: one row, `pgtkfns`, tolerance ≥ 0.51, basis `total` -
    C 3906 vs the port's 1899 (Evidence ③); and `docs/port-map.md` 506's Rust count (11) needs the
    integrator's `tools/check-port-lines.py --write-pending`.
11. **The five `#ifdef`s' arm texts.**  C's four `GDK_PIXBUF_VERSION_2_44` `#pragma` lines and the
    four cairo arms' `#ifdef`/`else`/`#endif` tokens have no Rust counterpart (Deviation 4); if the
    fleet wants them occurrence-checked, they are `token-aliases.tsv` rows.
12. **`docs/ports/macro-body-notes.tsv` and the restack DEFUN.**  This slice needed no row: the
    `doc:` literal carries make-docfile's value (Deviation 32) and the body's tokens are all spelled
    (the `let _ = 0;` arms supply C's `0`s).  The row this slice *could* have needed - C 1787's
    `!(FRAME_GTK_OUTER_WIDGET (f1) && FRAME_GTK_OUTER_WIDGET (f2))` written with `is_null ()` - is
    the class `check_pgtk_display_info`'s row already records for this file.
13. **The GIO segment's home.**  Deviation 34 puts the GIO declarations in this unit, which is the
    `src/xsettings.rs` shape: `<gio/gio.h>` is not in `include/gtk.rs`, and that mirror's ledger
    names GIO among the third-party headers it does not carry.  If that mirror grows a GIO part, the
    two units' segments move there in one commit (P-88 ⑤'s move rule), and this file's copy must be
    deleted in that same commit or the two collide (`E0428`).

## P-90's second step (2026-10-05): the toolkit segment moved out

- **The in-unit third-party segment is gone** (P-88 ⑤/P-90 ②③): C 28's chain
  (`<gtk/gtk.h>` through `"gtkutil.h"` 26) used to be declared here - `cairo_surface_type_t` and
  the four `CAIRO_SURFACE_TYPE_*` (`cursor` C 929/936/943/954), `cairo_region_t`, `gpointer`,
  `gchar`, `gint`, `gboolean`, `gssize`, `GError`, the five handles `GdkPixbuf`/`GtkCssProvider`/
  `GtkWidget`/`GtkWindow`/`GdkWindow` with `GTK_WINDOW`, and thirteen entry points - 155 lines in
  all, and every one of those names is `femacs/src/include/gtk.rs`'s (the third-party surface's
  single home).  This unit now reaches it through the edge `#[cfg (feature = "have_pgtk")]
  include!("include/gtk.rs");` at C 28's chain.  **The edge stands in the unit, not in
  `include/gtkutil.rs`**, which is the C position's owner: that mirror does not carry its C 26 edge
  yet, and its head note names the measured blocker set (`src/xfns.rs`'s `GtkWidgetClass` needs
  `include/gtk.rs`'s record to grow a member, and that mirror carries an unlanded change this
  round) - `## Owed` of `docs/ports/include-gtkutil.md`.  The edge's gate is this file's own,
  `have_pgtk`; C's `HAVE_PGTK` implies `USE_GTK`, and the narrow arm below is what shows why the
  reach, not C 24's `USE_GTK`, is the right condition for a stand-in edge.
- **Arm readings** (`text-only`; `rustc --edition 2024 --crate-type=lib --emit=metadata
  --crate-name unit_probe -Awarnings build/femacs/units/pgtkfns/_root.rs`, own = a `_root.rs` span,
  mirror = a copy's, spanless = rustc's `aborting due to N previous errors`):
  - **narrow** = `have_pgtk + have_window_system`: **67 -> 73** (own 57 -> 63, mirror 9).  The six
    new errors are two `GtkCssProvider` + four `gtk_css_provider_load_from_data` sites:
    `include/gtk.rs` gates that GTK3-only class and its loader on `have_gtk3` (gtkutil.c's uses are
    inside `#ifdef HAVE_GTK3`), while this unit's C text is `HAVE_PGTK`-only (pgtkfns.c 833-890
    has no `HAVE_GTK3` guard) - so an arm with `have_pgtk` and **no** `have_gtk3` now sees the
    mirror's gate.  That arm is not a real configuration (configure.ac's pgtk path implies GTK3),
    and the fix is either to add `have_gtk3` to this reading or to let `include/gtk.rs`'s owner
    decide the pgtk implication - it is recorded, not papered over.
  - **wide** = narrow + `use_gtk + have_gtk3 + have_ext_menu_bar + have_ext_tool_bar +
    gdk_windowing_wayland + have_xwidgets`: **63 -> 63** (own 53, mirror 9) - unmoved, and the
    segment's 32 names resolve through the mirror there.

## The second slice (C 1149-1196): `Fpgtk_set_monitor_scale_factor`

**⑥ The arm readings (both, `text-only`, P-86 ③⑵; own = a `pgtkfns.rs`/`_root.rs` span, mirror =
a copy's).**  Both configurations were run on **both sides of the slice** (the file before it, and
the file with it), which is what makes the reading a comparison rather than a number:
```
narrow = have_pgtk + have_window_system
  HEAD:   exit 1, aborting due to 65 previous errors; 11 warnings emitted
  slice:  exit 1, aborting due to 65 previous errors; 11 warnings emitted
  -> the same eight classes with the same counts on both sides: 43 `cannot find type
     pgtk_output` (the placeholder `include/pgtkterm.rs`, anchored at the FRAME_X_OUTPUT! macro
     definition), 9 `cannot find type pgtk_display_info`, 4 `gtk_css_provider_load_from_data` and
     2 `GtkCssProvider` (the `have_gtk3` gate of `include/gtk.rs`, Evidence ②'s recorded class),
     2 `FRAME_EXTERNAL_TOOL_BAR_FLAG_SHIFT`, 2 `FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT`, 2
     `gtk_window_set_icon` and the one `E0308` at C 1137's `FRAME_SCALE_FACTOR (f)` (the previous
     slice's row, `include/frame.rs` 961).

wide = narrow + use_gtk + have_gtk3 + have_ext_menu_bar + have_ext_tool_bar +
       gdk_windowing_wayland + have_xwidgets
  HEAD:   exit 1, aborting due to 55 previous errors; 10 warnings emitted
  slice:  exit 1, aborting due to 55 previous errors; 10 warnings emitted
  -> identical on both sides as well; the ten errors the wide arm retires are the two
     `*_FLAG_SHIFT` pairs and the two GtkCssProvider classes, i.e. exactly the features it turns on
```
**This slice's own text adds no error and no warning in either arm** - the one warning it *did*
add on the first run (`unused doc comment`, from a `/// C: …` line written above the `DEFUN!`
macro invocation) was retired by moving the citation to a trailing comment, which is this file's
existing spelling for the two `DEFUN!`s of the first slice.

**⑦ `extract-abi.py --check`.**  `python3 tools/extract-abi.py --check emacs/src/pgtkfns.c
femacs/src/pgtkfns.rs` -> `FAIL`, **103 `- ` rows -> 101**: the slice retires
`missing function Fpgtk_set_monitor_scale_factor` and
`missing subr Fpgtk_set_monitor_scale_factor (pgtk-set-monitor-scale-factor)`, and the line-count
row moves 54.7% -> **53.5%** (C 3,906 vs Rust 1,816).  The 101 rows left are the remainder's -
100 `missing …` rows (17 -> 16 functions, the rest the DEFUNs/subrs/objects of C 1197-3906) and
the drift row - so this slice is a step, not the file's close.

**⑧ The seven text gates.**  `check-declarations.py` **17 declaration sites (0 forbidden, 17
allowed literals), 0 failures**; `check-citations.py` **676 file(s) scanned, 0 drift, 0 range,
2029 external, 36 allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound, 0
failure(s)** (52 range rows now cover src/pgtkfns.c); `check-port-lines.py` **0 file(s) under ten
percent, 0 reference line(s); 0 ratchet row(s), 0 new, 0 stale**; `check-ledger-ticks.py` **238
ledger(s) bound, 0 failure(s)**; `check-cond-eval.py src/pgtkfns.c` **11 conditions, 0 unknown, 0
disagreements [VERIFIED]**; `build-tu src/pgtkfns.c` was not re-run (the unit is not in the frozen
build, so its object is the same empty one the first slice recorded).  No mirror changed.

**Not claimed**: no monitor scale factor was ever set - `monitor_scale_factor_alist` is only
written by this function, and `Fpgtk_set_monitor_scale_factor` is registered by this file's
`syms_of_pgtkfns` (C 3775-3906, still the remainder), so nothing in the port has called it; the
two arm readings are **compile** readings of text this machine cannot build.

## The third slice (C 1197-1757): `Fx_create_frame`

**⑨ The file's biggest function, and what is in it.**  C 1197-1200's `DEFUN ("x-create-frame",
Fx_create_frame, Sx_create_frame, 1, 1, 0,` with C's own short docstring; C 1217-1229 the
terminal/display lookup and `check_pgtk_display_info`; C 1231-1246 the `minibuffer` four-way fork
(`make_frame_without_minibuffer`/`make_minibuffer_frame`/`make_frame`) and the `parent_frame`
acceptance test; C 1248-1260 the two `undecorated`/`override_redirect` parameter reads; C 1302-1335
the frame's officialisation (`XSETFRAME`, `frame_set_id_from_params`, the two css providers, the
colour reference-count block, `parent_desc`/`explicit_parent`, the name and the `specbind`); C
1387-1403 the two font drivers and the font-backend default; C 1406-1471 the parameter chain
(border/child-border/divider/scroll-bar/colour/fringe), `init_frame_faces`, the `min-width`/
`min-height` stores and the first `adjust_frame_size`; C 1474-1520 the menu/tab/tool bar lines, the
seven further parameters, `gui_figure_window_size` and `no_split`; C 1545-1600 `xg_create_frame_widgets`,
`pgtk_set_event_handler`, the two `gtk_widget_realize` calls and the fifteen `INSTALL_CURSOR`
installs; C 1603-1654 the frame's own reference counts, `Vframe_list`, the icon/cursor/alpha
parameters, the parent-frame re-parenting through GTK (`g_object_ref`/`gtk_container_remove`/
`gtk_fixed_put`/`gtk_widget_show_all`/`g_object_unref`, `gtk_widget_destroy`, the seven
`FRAME_OUTPUT_DATA` writes) and the `GList` show-all walk; C 1686-1760 `initialize_frame_menubar`,
`can_set_window_size`, `xg_wm_set_size_hint`, the second `adjust_frame_size`, the `fullscreen`
parameter, the visibility block, the two `z-group`-family parameters, the
`default-minibuffer-frame` initialisation, the trailing parameter-alist loop, the
`border_color_css_provider`/`cr_surface_visible_bell`/`atimer_visible_bell`/`watched_scale_factor`
clears, the scale-factor atimer and `return unbind_to (count, frame);`.  Rust: 1,816 -> **2,578**
lines.

**⑩ The arm readings (both arms, `text-only`, the `text-only` class's two-sided shape; own = a
`pgtkfns.rs`/`_root.rs` span, mirror = a copy's).**  Measured on **both sides of the slice**, HEAD
vs slice, with the same two arm sets Evidence ⑥ used:
```
narrow = have_pgtk + have_window_system
  HEAD:   exit 1, 65 errors / 11 warnings
  slice:  exit 1, 143 errors / 11 warnings   (+78 errors, +0 warnings)
  75 x `cannot find type pgtk_output`       43 -> 118   (this slice's FRAME_X_OUTPUT! sites)
   1 x `cannot find type pgtk_display_info`  9 ->  10   (this slice's FRAME_X_DISPLAY!/… sites)
   2 x `cannot find function gtk_css_provider_new`  0 -> 2   (the have_gtk3 gate class)
  every other class is unchanged, count for count: 4 `gtk_css_provider_load_from_data`, 2
  `GtkCssProvider`, 2 + 2 `*_FLAG_SHIFT`, 2 `gtk_window_set_icon`, 1 `E0308` (C 1137's row)

wide = narrow + use_gtk + have_gtk3 + have_ext_menu_bar + have_ext_tool_bar +
       gdk_windowing_wayland + have_xwidgets
  HEAD:   exit 1, 55 errors / 10 warnings
  slice:  exit 1, 131 errors / 10 warnings   (+76 errors, +0 warnings: the two gtk_css_provider_new
                                               sites resolve under have_gtk3, the other two classes don't)
```
**No new class appears in either arm and no warning is added.**  The 78 (76 in the wide arm) are
*one per expansion site* of the three classes this lane already carries, and the per-site
attribution was measured, not assumed: rustc's `in this macro invocation` note points into
`pgtkfns.rs`, and the sites whose note falls inside C 1197-1757 are exactly 75 + 1 + 2 - the same
numbers as the deltas.  The reason the count * rises * at all is `## Owed` 1: `include/pgtkterm.rs`
is a placeholder, so `pgtk_output`/`pgtk_display_info` are unresolved for every accessor expansion
(`FRAME_X_OUTPUT!` alone expands 33 times in this slice, and `INSTALL_CURSOR`'s fifteen
instantiations carry a further fifteen); the first two slices' texts touch the two records far less
often (43 and 9 sites at HEAD).  This slice's own text therefore adds **no error and no warning of
its own** in the sense the rule asks for - nothing that is not a site of a class already named in
this ledger - and the three classes are *counted*, not waved away.

**⑪ `extract-abi.py --check`.**  `FAIL`, **101 `- ` rows -> 99**: `missing function Fx_create_frame`
and `missing subr Fx_create_frame (x-create-frame)` retire, and the line-count row moves 53.5% ->
**34.0%** (C 3,906 vs Rust 2,578).  Two rows that replacing them *exposed* are paid in the same
commit, as the image/sfnt lanes' rows were: C 1577-1591's `INSTALL_CURSOR` pastes `GDK_ ## NAME`,
and C's `FRAME_OUTPUT_DATA`/`FRAME_X_DISPLAY`/`FRAME_OUTPUT_DATA` tokens are spelled by this unit's
own read-arm macros, so the checker's own two findings (`macro INSTALL_CURSOR does not mention gdk`,
`subr Fx_create_frame does not mention bottom_left_corner, …, xterm`) are recorded as two rows of
`docs/ports/macro-body-notes.tsv` (Deviation 23) - every *other* token of both bodies is still
compared, which is what that table's shape is for.  The 99 rows left are the remainder's.

**⑫ The seven text gates.**  `check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed
literals), 0 failures**; `check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2,029
external, 36 allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound, 0 failure(s)**
(53 range rows now cover src/pgtkfns.c); `check-port-lines.py` **0 file(s) under ten percent, 0
reference line(s), 0 ratchet row(s), 0 new, 0 stale**; `check-ledger-ticks.py` **238 ledger(s)
bound, 0 failure(s)**; `check-cond-eval.py src/pgtkfns.c` **11 conditions, 0 unknown, 0
disagreements [VERIFIED]**; `check-mirror-dupes.py` **16 duplicate(s), 16 ratchet row(s), 0
failure(s)** (the five macros this slice adds are this unit's own copies of `pgtkterm.h`/
`frame.h` rows whose mirror is not in its include graph).  `build-tu src/pgtkfns.c` was not re-run
(the unit is not in the frozen build, so its object is the same empty one the first slice recorded).

**⑬ Not claimed.**  No frame was created: `Fx_create_frame` is registered by this file's
`syms_of_pgtkfns` (C 3775-3906, still the remainder), so nothing in the port calls it, and every
call it makes (`xg_create_frame_widgets` and the other `gtkutil.c`/`pgtkterm.c` sides) is a
declaration in this unit, not a definition - the port has no GTK to link.  The two arm readings are
**compile** readings of text this machine cannot build, and the object-sized reading other units
use does not exist for this one (the frozen object is the empty one).  Nothing was executed, no
warning was suppressed, and the thirteen GDK cursor constants are placeholders (Deviation 24).

## The fourth slice (C 1754-1996): the restack pair and the GSETTINGS block

**⑭ What is in it.**  C 1754-1757's comment and `static void pgtk_frame_restack` (C 1758-1764: the
`block_input`/`xg_frame_restack`/`unblock_input` triple); `DEFUN ("pgtk-frame-restack",
Fpgtk_frame_restack, Spgtk_frame_restack, 2, 3, 0,` (C 1766) with C's own 15-line docstring (C
1767-1781) and its two `decode_live_frame` calls, the pointer truth test and the `!NILP (above)`
call (C 1782-1791); and the whole `#ifdef HAVE_GSETTINGS` region: C 1795-1800's five `#define`s
(`RESOURCE_KEY_MAX_LEN`, `SCHEMA_ID`, the two path strings and their `max (sizeof …)` length), C
1802-1818's three `static inline` predicates, C 1820-1921's `parse_resource_key` (the class/name
fork, the upper-to-lower walk with its four NOP arms, the `*sp++` step and the GSettings lookup),
C 1923-1928's `pgtk_check_resource_key_length`, C 1930-1955's `pgtk_get_defaults_value` (the
`static char holder[128]` copy-and-free dance) and C 1957-1974's `pgtk_set_defaults_value`, C
1976-1980's five `#undef`s, and C 1982-1996's `#else /* not HAVE_GSETTINGS */` arm with its own
three-line pair.  Rust: 2,578 -> **2,954** lines.

**⑮ The arm readings (four arms; P-86 ⑥'s rule - every sub-gate must be set).**  This slice has a
second condition inside the file gate, so the lane's two arm sets are each measured with and
without `have_gsettings`: the `#if` arm's text compiles only with it, and the `#else` arm's only
without it, so a two-arm reading would have left half of this slice uncompiled.
```
A narrow = have_pgtk + have_window_system                     : HEAD 143 errors / 11 warnings
                                                                slice 145 / 11
B wide   = A + use_gtk + have_gtk3 + have_ext_menu_bar + have_ext_tool_bar +
           gdk_windowing_wayland + have_xwidgets              : 131 / 10 -> 133 / 10
C = A + have_gsettings                                        : 143 / 11 -> 145 / 11
D = B + have_gsettings                                        : 131 / 10 -> 133 / 10
```
In all four arms the delta is the same **+2 sites of the already-named `pgtk_output` class** (the
restack DEFUN's two `FRAME_GTK_OUTER_WIDGET!` uses - attributed the same way as Evidence ⑩: rustc's
`in this macro invocation` notes fall inside the new text for exactly two sites), with **no new
class and no new warning**.  The first run of the two `have_gsettings` arms read **+3 `E0308`**: C
1952/1953/1973's `g_object_unref (gs)` / `g_free (str)` hand a `GSettings *` and a `gchar *` to the
mirror's `gpointer` parameters, and C's implicit conversion is now written out (Deviation 29) -
before that fix the two arms read 148 and 136 errors, i.e. the arm pair is what caught a defect the
`#else` arms could not see, which is the whole reason for measuring both.

**⑯ `extract-abi.py --check`.**  `FAIL`, **99 `- ` rows -> 95**: `missing function
pgtk_frame_restack`, `missing function pgtk_get_defaults_value`, `missing function
pgtk_set_defaults_value` and `missing subr Fpgtk_frame_restack (pgtk-frame-restack)` retire, no row
appears, and the line-count row moves 34.0% -> **24.4%** (C 3,906 vs Rust 2,954 - the value this
slice's *committed* text measures; it read 2,950 lines / 24.5% when the reading was taken, and the
citation reword of Deviation 35 added the four lines afterwards, so the number here is the one the
tree prints).  The first run read
one *new* row - `subr Fpgtk_frame_restack: docstring differs` - which is the measured half of
Deviation 32: C's `\(stacking)` is make-docfile's escape and the literal must carry the value
`(stacking)`, not C's spelling.  The 95 rows left are the remainder's.

**⑰ The seven text gates.**  `check-declarations.py` **18 declaration sites (0 forbidden, 18 allowed
literals), 0 failures** - the eighteenth is this slice's `SCHEMA_ID` literal (Deviation 35), and it is
allowlisted with its reason the way the Windows registry-path and clipboard-class rows are;
`check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2,039 external, 36 allowed, 0 stale**
(the ten new external citations are the GIO segment's); `check-dead-file-coverage.py` **145 ledger(s) bound, 0 failure(s)**
(54 range rows now cover src/pgtkfns.c); `check-port-lines.py` **0 file(s) under ten percent, 0
reference line(s), 0 ratchet row(s), 0 new, 0 stale**; `check-ledger-ticks.py` **238 ledger(s)
bound, 0 failure(s)**; `check-cond-eval.py src/pgtkfns.c` **11 conditions, 0 unknown, 0
disagreements [VERIFIED]**; `check-mirror-dupes.py` **16 duplicate(s), 16 ratchet row(s), 0
failure(s)**.  No mirror changed, so P-87's two legs do not apply this time (the GIO segment is a
unit segment, Deviation 34).

**⑱ Not claimed.**  Nothing ran: `Fpgtk_frame_restack` is registered by this file's `syms_of_pgtkfns`
(still the remainder) and `pgtk_get_defaults_value`/`pgtk_set_defaults_value` are called only by C
1999-2021's `Fpgtk_set_resource` and C 2254-2256 (both still the remainder), so no restack was
performed and no GSettings schema was looked up.  The four arm readings are **compile** readings of
text this machine cannot build, and this unit has no object-byte reading (the frozen object is the
empty one).  `xg_frame_restack` is a declaration here - its definition is `src/gtkutil.rs` 2687's,
and the port has no GTK to link against.

35. **`SCHEMA_ID`'s literal is the one new *allowlisted* product-name site** (C 1796,
    `docs/baseline/declarations-allowlist.tsv`'s `femacs/src/pgtkfns.rs` row).  The value is the
    GSettings schema id the reference's own data file installs
    (the `etc/` schema XML whose *name* is that id - one of the verbatim-copied trees - so the
    ledger spells it in the allowlist row rather than in this prose; the gate's `title` class reads
    ledger prose) and that GIO's `g_settings_schema_source_lookup` resolves **by name**: other programs and the user's own
    GSettings database read it, so renaming it would change behaviour rather than make a statement
    about this repository - 0.1-19's eighth half (status 3.599), the class the Windows registry-path
    rows (`src/w32.rs` 4679, `femacs/nt/addpm.rs` 560, whose values are the allowlist's) and the X
    clipboard window class (`src/w32select.rs`) already carry.  **Measured**: the gate first read
    `literal x1: const SCHEMA_ID: … c"…".as_ptr ();` (a failure with no row - the value is the
    allowlist row's, quoted there), and after the row it reads `18 declaration sites (0 forbidden,
    18 allowed literals), 0 failures`.  The citation above the item deliberately does **not** quote the value:
    a `///` line that spells it is the gate's `title` class (measured: `title x1` on the first
    draft), which is why the comment names C's `#define` and leaves C's line to this ledger.

## The fifth slice (C 1999-2208): the resource subr, the eleven display subrs and `pgtk-font-name`

**⑲ What is in it.**  C 1999-2019's `DEFUN ("pgtk-set-resource", …)` (the `check_window_system`
call, the two `CHECK_STRING` gates, the `Vx_resource_name`/`Vx_resource_class` choice on the
attribute's first letter, the `g_strdup_printf` key and the `pgtk_set_defaults_value` call); C
2022-2039's `x-server-max-request-size` and `x-display-screens`; C 2042-2073's `x-display-mm-height`
and C 2076-2107's `x-display-mm-width` (the two GDK monitor walks, each summing the monitors whose
rectangle starts at 0 and adding the rest); C 2110-2133's `x-display-backing-store`,
`x-display-visual-class` and `x-display-save-under`; C 2136-2157's `x-open-connection` (the
`build_string ("")` default, `pgtk_term_init` and the `fatal`/`error` pair) and C 2160-2172's
`x-close-connection` (the reference-count gate and `pgtk_delete_terminal`); C 2175-2186's
`x-display-list` (the `x_display_list` walk) and C 2188-2208's `pgtk-font-name` (the `-` test, the
two `strstr` tests and `pgtk_xlfd_to_fontname`).  Rust: 2,954 -> **3,248** lines.

**⑳ The arm readings (four arms, `text-only`).**  This slice's text sits after the GSETTINGS
`#endif`, so it compiles in all four of the arm sets Evidence ⑮ established; all four were measured
on both sides of the slice, and all four read the same thing:
```
A narrow = have_pgtk + have_window_system          : HEAD 145 errors / 11 warnings -> 151 / 11
B wide   = A + use_gtk + have_gtk3 + have_ext_menu_bar + have_ext_tool_bar +
           gdk_windowing_wayland + have_xwidgets   : 133 / 10 -> 139 / 10
C = A + have_gsettings                             : 145 / 11 -> 151 / 11
D = B + have_gsettings                             : 133 / 10 -> 139 / 10
```
The delta is **+6 sites of the already-named `pgtk_display_info` class** in every arm - the six
declarations of that type the new text makes (`dpyinfo`/`ndi` locals; attributed the Evidence ⑩ way:
rustc's `in this macro invocation` notes fall inside the new text for exactly six sites, and the
sixth is C 2148's `== 0` cast, Deviation 40) - with no new class and no new warning.  The first run
read **+7 warnings in all four arms**: this slice's nine `///`-above-`DEFUN!` citations were
`unused doc comment`s (Deviation 36), retired by writing them as block comments.

**㉑ `extract-abi.py --check`.**  `FAIL`, **95 `- ` rows -> 83**: eleven `missing subr` rows retire
(`Fpgtk_set_resource`/`pgtk-set-resource`, `Fx_server_max_request_size`, `Fx_display_screens`,
`Fx_display_mm_height`, `Fx_display_mm_width`, `Fx_display_backing_store`,
`Fx_display_visual_class`, `Fx_display_save_under`, `Fx_open_connection`, `Fx_close_connection`,
`Fx_display_list`, `Fpgtk_font_name`) and the line-count row moves 24.4% -> **16.8%** (C 3,906 vs
Rust 3,248).  **Three new rows appeared and were retired properly, not allowlisted**: with C's
`attr[0]`/`nm[0]`/`== 0` written as `*attr`/`*nm`/`is_null ()` the checker reads `subr
Fpgtk_font_name does not mention 0`, `subr Fpgtk_set_resource does not mention 0` and `subr
Fx_open_connection does not mention 0`; the C spellings (`.offset (0)`, `0 as *mut …`) remove all
three (Deviation 40), so this slice adds no `macro-body-notes.tsv` row.

**㉒ The gates, P-87's two legs, and what is not claimed.**  `check-declarations.py` **18
declaration sites (0 forbidden, 18 allowed literals), 0 failures**; `check-citations.py` **676
file(s) scanned, 0 drift, 0 range, 2,040 external, 36 allowed, 0 stale**; `check-dead-file-coverage.py`
**145 ledger(s) bound, 0 failure(s)** (55 range rows now cover src/pgtkfns.c);
`check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s), 0 ratchet row(s), 0 new, 0
stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**; `check-cond-eval.py
src/pgtkfns.c` **11 conditions, 0 unknown, 0 disagreements [VERIFIED]**; `check-mirror-dupes.py`
**16 duplicate(s), 16 ratchet row(s), 0 failure(s)**.  **A mirror changed this round**
(`include/gtk.rs`, Deviation 42), so P-87 ①'s consumer sweep was run and reads `checked 83 unit(s),
0 failure(s)` with `checking 83 unit(s), 0 unit failure(s)`; the mirror's own ledger carries the six
rows and a `status 3.718` section.  **Not claimed**: no resource was set (the key is built and handed
to this file's own `pgtk_set_defaults_value`, which the frozen configuration's `#else` arm defines as
an `error`), no monitor was enumerated (`gdk_display_get_n_monitors` is a declaration - the port has
no GDK to link), and `pgtk-font-name` never converted anything (`pgtk_xlfd_to_fontname` is a
declaration of `pgtkfont.c`'s definition).  All four arm readings are **compile** readings of text
this machine cannot build.

## The sixth slice (C 2210-2403): the miscellaneous group's first five definitions and seven subrs

**㉓ What is in it.**  C 2210-2214's own banner ("Miscellaneous functions not called through
hooks", carried as the Rust block comment that heads this slice); C 2216-2221's
`check_x_display_info` (C's "Called from frame.c." comment, delegating to this file's
`check_pgtk_display_info`); C 2223-2231's `pgtk_set_scroll_bar_default_width` and C 2233-2241's
`pgtk_set_scroll_bar_default_height` (the `FRAME_COLUMN_WIDTH`/`FRAME_LINE_HEIGHT` unit, the
`xg_get_default_scrollbar_*` minima and the four `FRAME_CONFIG_SCROLL_BAR_*` writes); C 2243-2268's
`pgtk_get_string_resource` (the `check_window_system` gate, `inhibit_x_resources`, the two
`pgtk_get_defaults_value` lookups and the `YES`/`NO` translations); C 2270-2281's
`pgtk_get_focus_frame`; and the seven colour/geometry subrs C 2283-2403: `xw-color-defined-p`,
`xw-color-values`, `xw-display-color-p`, `x-display-grayscale-p` and the two scaling monitor walks
`x-display-pixel-width`/`x-display-pixel-height` (each multiplying the monitor's rectangle by
`pgtk_get_monitor_scale_factor (gdk_monitor_get_model (monitor))`, falling back to
`gdk_monitor_get_scale_factor`, and keeping the largest right/bottom edge).  Rust: 3,248 ->
**3,534** lines.

**㉔ The arm readings (four arms, both sides of the slice).**
```
A narrow = have_pgtk + have_window_system          : HEAD 151 errors / 11 warnings -> 156 / 11
B wide   = A + use_gtk + have_gtk3 + have_ext_menu_bar + have_ext_tool_bar +
           gdk_windowing_wayland + have_xwidgets   : 139 / 10 -> 144 / 10
C = A + have_gsettings                             : 151 / 11 -> 156 / 11
D = B + have_gsettings                             : 139 / 10 -> 144 / 10
```
All four arms read the same delta: **+4 sites of the `pgtk_display_info` class and +1 of the
`pgtk_output` class**, both already named by this ledger (the six `*mut pgtk_display_info`
declarations and `pgtk_get_focus_frame`'s single `FRAME_DISPLAY_INFO!` expansion - attributed the
Evidence ⑩ way: exactly four and one of the `in this macro invocation` notes fall inside the new
text), with **no new class and no new warning**.  The first run read **+2 `E0308`**: C 2292/2308's
second `false` goes to `pgtk_defined_color`'s `int index` parameter and Rust has no implicit
conversion (Deviation 45).

**㉕ `extract-abi.py --check`.**  `FAIL`, **83 `- ` rows -> 72**: five `missing function` rows retire
(`check_x_display_info`, `pgtk_get_focus_frame`, `pgtk_get_string_resource`,
`pgtk_set_scroll_bar_default_height`, `pgtk_set_scroll_bar_default_width`) and six `missing subr`
rows (`Fxw_color_defined_p`/`xw-color-defined-p`, `Fxw_color_values`, `Fxw_display_color_p`,
`Fx_display_grayscale_p`, `Fx_display_pixel_width`, `Fx_display_pixel_height`), no row appears, and
the line-count row moves 16.8% -> **9.5%** (C 3,906 vs Rust 3,534).

**㉖ The gates, P-87's two legs, and what is not claimed.**  `check-declarations.py` **18
declaration sites (0 forbidden, 18 allowed literals), 0 failures**; `check-citations.py` **676
file(s) scanned, 0 drift, 0 range, 2,040 external, 36 allowed, 0 stale**;
`check-dead-file-coverage.py` **145 ledger(s) bound, 0 failure(s)** (56 range rows now cover
src/pgtkfns.c); `check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s), 0 ratchet
row(s), 0 new, 0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**;
`check-cond-eval.py src/pgtkfns.c` **11 conditions, 0 unknown, 0 disagreements [VERIFIED]**;
`check-mirror-dupes.py` **16 duplicate(s), 16 ratchet row(s), 0 failure(s)**.  **A mirror changed
this round** (`include/gtk.rs` gains `gdk_monitor_get_model` and `gdk_monitor_get_scale_factor` -
the two GDK calls the scaling walks make; no GDK header exists here, so both cite the header alone),
so P-87 ①'s consumer sweep was run: `checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s), 0
unit failure(s)`, and the mirror's ledger names both rows and carries a `status 3.719` section.
**Not claimed**: no display was opened (`check_x_display_info` delegates to a function whose
`pgtk_display_info` record does not exist in the port), no scroll-bar default was computed
(`xg_get_default_scrollbar_*` are declarations of `gtkutil.c`'s definitions), no string resource was
looked up (`pgtk_get_defaults_value` is this file's own, and the frozen arm answers `error`), and no
monitor was enumerated.  All four arm readings are **compile** readings of text this machine cannot
build.

## The seventh slice (C 2405-2549): the monitor-attribute builder and the group's last three definitions

**㉗ What is in it.**  C 2405-2510's `DEFUN ("pgtk-display-monitor-attributes-list", …)` - the
`pgtk_display_info` lookup, `make_nil_vector` + `xcalloc` for `n_monitors` records, the
`FOR_EACH_FRAME` walk that files each frame under the monitor its `gtk_widget_get_window` reports
(C 2447-2453, `gdk_display_get_monitor_at_window` compared against each
`gdk_display_get_monitor`), the per-monitor loop (geometry, workarea, the two millimetre reads, the
scale factor fallback and the twelve `mi->` member writes through `&monitors[i]`), the
`make_monitor_attribute_list` call and `free_monitors`; C 2512-2531's `pgtk_frame_scale_factor` (the
same scale lookup for one frame, C's own `double` return); and C 2533-2549's `x-display-planes` and
`x-display-color-cells` (the `1 << min (n_planes, 24)` fold).  Rust: 3,534 -> **3,711** lines.

**㉘ The arm readings (four arms, both sides) - and the first slice of this unit whose first run was
clean.**
```
A narrow = have_pgtk + have_window_system          : HEAD 156 errors / 11 warnings -> 163 / 11
B wide   = A + use_gtk + have_gtk3 + have_ext_menu_bar + have_ext_tool_bar +
           gdk_windowing_wayland + have_xwidgets   : 144 / 10 -> 151 / 10
C = A + have_gsettings                             : 156 / 11 -> 163 / 11
D = B + have_gsettings                             : 144 / 10 -> 151 / 10
```
All four arms read the same delta: **+4 `pgtk_output` and +3 `pgtk_display_info` sites**, both
classes this ledger already names (attributed the Evidence ⑩ way: exactly four and three of the
`in this macro invocation` notes fall inside the new text), with **no new class and no new
warning**.  Unlike the previous four slices, the first run needed no repair: nothing in this slice's
own text had to be revisited after the readings (the `static mut source` spelling and the
`monitors.offset` element address were written from the recorded devices, Deviations 48-49).

**㉙ `extract-abi.py --check`.**  `FAIL`, **72 `- ` rows -> 67**: `missing function
pgtk_frame_scale_factor`, `missing subr Fpgtk_display_monitor_attributes_list
(pgtk-display-monitor-attributes-list)`, `missing subr Fx_display_planes (x-display-planes)` and
`missing subr Fx_display_color_cells (x-display-color-cells)` retire, no row appears - **and the
line-count row retires too**: the port is now 3,711 lines against C's 3,906, **4.99% under** C, i.e.
inside the 5% rule the row exists for (the row's history in this ledger: 54.7%, 53.5%, 51.4%,
34.0%, 24.4%, 16.8%, 9.5%, gone).  The 67 rows left are the tooltip group (C 2551-3311) and the tail
(C 3312-3906).

**㉚ The gates, P-87's two legs, and what is not claimed.**  `check-declarations.py` **18
declaration sites (0 forbidden, 18 allowed literals), 0 failures**; `check-citations.py` **676
file(s) scanned, 0 drift, 0 range, 2,040 external, 36 allowed, 0 stale**;
`check-dead-file-coverage.py` **145 ledger(s) bound, 0 failure(s)** (57 range rows now cover
src/pgtkfns.c); `check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s), 0 ratchet
row(s), 0 new, 0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**;
`check-cond-eval.py src/pgtkfns.c` **11 conditions, 0 unknown, 0 disagreements [VERIFIED]**;
`check-mirror-dupes.py` **16 duplicate(s), 16 ratchet row(s), 0 failure(s)**.  **A mirror changed
this round** (three GDK monitor rows, Deviation 52), so P-87 ①'s consumer sweep was run and reads `checked 83 unit(s), 0 failure(s)` and `checking 83 unit(s), 0 unit failure(s)`.  **Not claimed**: no monitor
attribute list was built (the DEFUN is registered by `syms_of_pgtkfns`, still the remainder, so
nothing calls it; `make_monitor_attribute_list`/`free_monitors` are `frame.c`'s declarations), no
frame's scale factor was computed (`pgtk_frame_scale_factor` is called by this file's own
`FRAME_SCALE_FACTOR` arm only), and the four arm readings are **compile** readings of text this
machine cannot build.

## The eighth slice (C 2551-2838): the tooltip group's state, its unwind helper and `pgtk_create_tip_frame`

**㉛ What is in it.**  C 2551-2553's "Tool tips" banner, carried as C's own box comment; C 2555-2573's
six file-scope statics (`tip_frame`, `tip_window`, `tip_timer`, `tip_last_string`, `tip_last_frame`,
`tip_last_parms`, each with its own comment); C 2576-2587's `unwind_create_tip_frame` (the
`unwind_create_frame` call and the two clears); and C 2590-2838's `pgtk_create_tip_frame` - the
`face_change` save (C 2606), the terminal gate, the `Fcopy_alist`/name block, `make_frame (false)`
with `wants_modeline` and the unwind protect, the frame's officialisation (the output method, the
`xzalloc`'d `pgtk_output`, `FRAME_FONTSET`, the two relief pixels, `tooltip`, the display info, the
parent descriptor and `explicit_parent`), the colour reference-count block, the name/`specbind`
block, the two font drivers, the thirteen `gui_default_parameter` calls with `init_frame_faces`, the
GTK window setup (the five `gtk_window_set_*` calls, `tip_window` and the cursor copy), the
`auto_raise`-through-`borders_respect_alpha_background` parameters, the three `AUTO_FRAME_ARG`
blocks (the `tooltip` and `display-type` defaults and the `calln (Qface_set_after_frame_default …)`
that restores `background-color`), `no_split`, the two reference counts, `Vframe_list`,
`can_set_window_size`, the `adjust_frame_size` with `Qtip_frame`, the `face_change` restore and
`return unbind_to (count, frame)`.  Rust: 3,711 -> **4,055** lines.

**㉜ The arm readings (four arms, both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 163 errors / 11 warnings -> 182 / 11
B wide   = A + use_gtk + have_gtk3 + have_ext_menu_bar + have_ext_tool_bar +
           gdk_windowing_wayland + have_xwidgets   : 151 / 10 -> 170 / 10
C = A + have_gsettings                             : 163 / 11 -> 182 / 11
D = B + have_gsettings                             : 151 / 10 -> 170 / 10
```
All four arms read the same delta: **+18 `pgtk_output` and +1 `pgtk_display_info` sites** (both
classes this ledger names; attributed the Evidence ⑩ way - the new text's `FRAME_X_OUTPUT!`/
`FRAME_DISPLAY_INFO!` expansions), with **no new class and no new warning**.  The first run read one
extra class in all four arms - `cannot find value face_change` x2 - because that global lives in a
non-glob-imported module of the mirror; the unit's own declaration (Deviation 54) retired it, and
nothing else in the slice's text needed revisiting.

**㉝ `extract-abi.py --check`.**  `FAIL`, **67 `- ` rows -> 59**: `missing function
pgtk_create_tip_frame`, `missing function unwind_create_tip_frame` and the six `missing file-scope
object tip_*` rows retire, no row appears, and the port stays inside the 5% rule the drift row
watches (C 3,906 vs Rust 4,055 = **+3.8%**, and the row has not come back).

**㉞ The gates, P-87's two legs, and what is not claimed.**  `check-declarations.py` **18 declaration
sites (0 forbidden, 18 allowed literals), 0 failures**; `check-citations.py` **676 file(s) scanned,
0 drift, 0 range, 2,040 external, 36 allowed, 0 stale**; `check-dead-file-coverage.py` **145
ledger(s) bound, 0 failure(s)** (58 range rows now cover src/pgtkfns.c); `check-port-lines.py` **0
file(s) under ten percent, 0 reference line(s), 0 ratchet row(s), 0 new, 0 stale**;
`check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**; `check-cond-eval.py src/pgtkfns.c`
**11 conditions, 0 unknown, 0 disagreements [VERIFIED]**; `check-mirror-dupes.py` **16 duplicate(s),
16 ratchet row(s), 0 failure(s)**.  **A mirror changed this round** (four GTK window rows and the
`GdkWindowTypeHint` enum, Deviation 57), so P-87 ①'s consumer sweep was run: `checked 83 unit(s), 0
failure(s)` and `checking 83 unit(s), 0 unit failure(s)`.  **Not claimed**: no tooltip frame was
created (`pgtk_create_tip_frame` is called by `Fx_show_tip`, still the remainder), no GTK window
exists (`gtk_window_set_*` are declarations - the port has no GTK to link), and the tooltip statics
are written by nothing in the port: they are zero-initialised declarations this slice's own
functions would write.  The four arm readings are **compile** readings of text this machine cannot
build.

## The ninth slice (C 2840-3025): `compute_tip_xy` and `pgtk_hide_tip`

**㉟ What is in it.**  C 2840-2844's comment and C 2846-2943's `compute_tip_xy` (the four
`Fcdr (Fassq (…))` parameter reads, the GDK pointer query through
`gdk_display_get_default_seat`/`gdk_seat_get_pointer`/`gdk_device_get_position`, the
`Fpgtk_display_monitor_attributes_list` walk that finds the monitor under the pointer by comparing
its `geometry` rectangle against the two root coordinates, the pixel-width/height fallback, and the
two ladders that decide `*root_y` and `*root_x`); and C 2946-3025's `pgtk_hide_tip` (the
`Qcancel_timer` call, C's five-condition early return, the `inhibit_redisplay`/`inhibit_quit`
specbinds, the `xg_hide_tooltip` attempt on `tip_last_frame`, the `use_system_tooltips` reset, and
the Emacs tip's delete-or-hide fork).  Rust: 4,055 -> **4,278** lines.

**㊱ The arm readings (four arms, both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 182 errors / 11 warnings -> 187 / 11
B wide   = A + use_gtk + have_gtk3 + have_ext_menu_bar + have_ext_tool_bar +
           gdk_windowing_wayland + have_xwidgets   : 170 / 10 -> 175 / 10
C = A + have_gsettings                             : 182 / 11 -> 187 / 11
D = B + have_gsettings                             : 170 / 10 -> 175 / 10
```
All four arms read the same delta: **+3 `pgtk_output` and +2 `pgtk_display_info` sites**, both
classes this ledger names - the three are the slice body's accessor expansions (attributed the
Evidence ⑩ way: exactly three of the `in this macro invocation` notes fall inside the new text) and
the two are the new `pgtk_display_pixel_width`/`_height` declarations' `*mut pgtk_display_info`
parameters (which sit in this unit's own block, above the slice's text, so the site count is read
from the whole diff, not the appended range).  **No new class and no new warning**, and this slice's
first run needed no repair of its own text.

**㊲ `extract-abi.py --check`.**  `missing function compute_tip_xy` and `missing function
pgtk_hide_tip` retire (59 -> 57 rows), **but the step exposed a reading, not just a retirement**:
the port's 4,278 lines are 9.5% over C's 3,906, so the `line count drifted` row came back for the
first time since the seventh slice.  The project's answer is the tolerance table, not a hedge: this
unit's `docs/ports/line-drift-tolerance.tsv` row (0.10, `total` basis) now records the measurement
and the reason (Deviation 62), and the `--check` drift row retires with it - so the reading is
**`FAIL` with 57 rows**, none of them a drift row, and the 57 are the text the file has not reached
yet.

**㊳ The gates, P-87's two legs, and what is not claimed.**  `check-declarations.py` **18 declaration
sites (0 forbidden, 18 allowed literals), 0 failures**; `check-citations.py` **676 file(s) scanned,
0 drift, 0 range, 2,040 external, 36 allowed, 0 stale**; `check-dead-file-coverage.py` **145
ledger(s) bound, 0 failure(s)** (59 range rows now cover src/pgtkfns.c); `check-port-lines.py` **0
file(s) under ten percent, 0 reference line(s), 0 ratchet row(s), 0 new, 0 stale**;
`check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**; `check-cond-eval.py src/pgtkfns.c`
**11 conditions, 0 unknown, 0 disagreements [VERIFIED]**; `check-mirror-dupes.py` **16 duplicate(s),
16 ratchet row(s), 0 failure(s)** - after this round's three new GDK rows first read 18/16/2 and the
two copies they exposed in `src/xwidget.rs` were dropped (Deviation 61).  **A mirror changed this
round**, so P-87 ①'s consumer sweep was run twice (once after the mirror change, once on the final
tree) and both readings are `checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s), 0 unit
failure(s)`.  **Not claimed**: no tooltip was positioned or hidden (`compute_tip_xy` and
`pgtk_hide_tip` are called only by `Fx_show_tip`/`Fx_hide_tip`, still the remainder), no GDK pointer
query ran (`gdk_device_get_position` is a declaration - the port has no GDK to link), and the four
arm readings are **compile** readings of text this machine cannot build.

## The tenth slice (C 3027-3317): `Fx_show_tip` and `Fx_hide_tip`

**㊴ What is in it.**  C 3027-3309's `Fx_show_tip` (the two `ENABLE_CHECKING` arms, the
parameter-list comparison with its two `goto start_timer`s, the `tip_frame` fork that creates a frame
through `pgtk_create_tip_frame`, the `tip_last_frame`/`tip_last_string`/`tip_last_parms` writes, the
`Fset_buffer`/`set_window_buffer` setup, `SET_TEXT_POS` with the `TRY_WINDOW_IGNORE_FONTS_CHANGE`
try, the `hide_tip` fork, the `FRAME_TOTAL_COLS`/`total_lines` sizing and the `flush_frame` tail) and
C 3312-3317's `Fx_hide_tip` (a one-line delegator to `pgtk_hide_tip`).  Rust: 4,278 -> **4,671**
lines (4,647 of the slice's own text plus the 24 lines Deviations 67 and 68 added).

**㊵ The arm readings (four arms, both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 187 errors / 11 warnings -> 197 / 11
B wide   = A + use_gtk + have_gtk3 + have_ext_menu_bar + have_ext_tool_bar +
           gdk_windowing_wayland + have_xwidgets   : 175 / 10 -> 185 / 10
C = A + have_gsettings                             : 187 / 11 -> 197 / 11
D = B + have_gsettings                             : 175 / 10 -> 185 / 10
```
All four arms read the same delta, **+10 sites of the unit's registered `cannot find type` class**
(the slice's accessor expansions; 7 of the 8 lines that read one more diagnostic than HEAD stand
inside the added range, the rest are the two new declarations' `*mut pgtk_display_info` parameters in
this unit's own block), **no new class and no new warning**.  The first reading of this text was not
that: it was 190/178/190/178 with five syntax errors, and the difference is Deviations 67-68 - a
truncated parse hides the diagnostics after it, so the repaired readings are the larger, correct ones.

**㊶ What the slice calls, and what `--check` reads.**  Four items are declared or spelled in this
unit (Deviation 68: `try_window`, `set_window_buffer`, the `SET_TEXT_POS` copy, `flush_frame!`/
`!= 0`), and two declarations the slice added in its own block are recorded in it (Deviation 65's
`ENABLE_CHECKING` arm has its own `glyph_row` shape; `xg_show_tooltip` at gtkutil.h 214 keeps the
`HAVE_PGTK` arm's two-argument form while `src/gtkutil.rs` defines both arms).  `extract-abi.py
--check emacs/src/pgtkfns.c femacs/src/pgtkfns.rs` reads **55 rows** (57 -> 55: `missing subr
Fx_show_tip (x-show-tip)` and `missing subr Fx_hide_tip (x-hide-tip)` retire, no row appears), and
the line-count row stays retired under Deviation 69's tolerance.

**㊷ Gates and what is not claimed.**  `check-declarations.py` **18 declaration sites (0 forbidden,
18 allowed literals), 0 failures**; `check-citations.py` **676 file(s) scanned, 0 drift, 0 range,
2040 external, 36 allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound,
0 failure(s)** (**60 range rows** now cover `src/pgtkfns.c`); `check-port-lines.py` **0 file(s) under
ten percent, 0 reference line(s), 0 ratchet row(s), 0 new, 0 stale**; `check-ledger-ticks.py`
**238 ledger(s) bound, 0 failure(s)**; `check-cond-eval.py src/pgtkfns.c` **11 conditions, 0 unknown,
0 disagreements [VERIFIED]**; `check-mirror-dupes.py` **16 duplicate(s), 16 ratchet row(s),
0 failure(s)** (no mirror changed this round).  **Not claimed**: no tooltip was shown or hidden
(`Fx_show_tip`/`Fx_hide_tip` are registered by `syms_of_pgtkfns`, still the remainder, so nothing in
the port calls them), `xg_show_tooltip` and `update_single_window` are declarations - the port has no
GTK to link - and the four arm readings are **compile** readings of text this machine cannot build.

## The eleventh slice (C 3319-3525): `frame_geometry` and the four geometry/mouse subrs

**㊸ What is in it.**  C 3319-3323's comment and C 3325-3414's `frame_geometry` (the three edge
branches `Qouter_edges`/`Qnative_edges`/`Qinner_edges`, the association list of eleven attributes,
the outer-widget/widget fork through `gtk_window_get_position` and `gtk_widget_get_allocation`, the
`GtkAllocation` fallback, and the tab-bar/tool-bar heights), `Fpgtk_frame_geometry` (C 3416-3458),
`Fpgtk_frame_edges` (C 3460-3481, C's three-way `type` choice), the mouse warp subr
`Fpgtk_set_mouse_absolute_pixel_position` (C 3483-3502) and `Fpgtk_mouse_absolute_pixel_position`
(C 3504-3525).  Rust: 4,671 -> **4,859** lines.

**㊹ The arm readings (four arms, both sides).**
```
A narrow = have_pgtk + have_window_system          : HEAD 197 errors / 11 warnings -> 209 / 11
B wide   = A + use_gtk + have_gtk3 + have_ext_menu_bar + have_ext_tool_bar +
           gdk_windowing_wayland + have_xwidgets   : 185 / 10 -> 197 / 10
C = A + have_gsettings                             : 197 / 11 -> 209 / 11
D = B + have_gsettings                             : 185 / 10 -> 197 / 10
```
All four arms read the same delta, **+12 sites of the unit's registered `cannot find type` class**
(the appended body's `FRAME_X_OUTPUT`-based accessor expansions), **no new class and no new
warning**.  The first reading of this text was 235/223/235/223 with 21 mismatched-type sites
(Deviation 72) and a `pgtk_output` count 12 higher than the final one, i.e. the repairs removed 26
error sites; Deviation 71 measures the new macros' own share as zero.

**㊺ What the slice calls, and what `--check` reads.**  Four items are declared or spelled here
(Deviations 70-71: `FRAME_WIDGET` and the three pgtkterm.h toolbar rows), three GTK/GDK entry points
landed in `include/gtk.rs` and one duplicate was retired in `src/pgtkselect.rs` (Deviation 73), and
`extract-abi.py --check emacs/src/pgtkfns.c femacs/src/pgtkfns.rs` reads **51 rows** (55 -> 51:
`missing function frame_geometry`, `missing subr Fpgtk_frame_geometry`, `missing subr
Fpgtk_frame_edges`, `missing subr Fpgtk_set_mouse_absolute_pixel_position` and `missing subr
Fpgtk_mouse_absolute_pixel_position` retire, and the four subrs' docstrings were compared by the
same reading - they are the C comments through `cinventory`'s make-docfile rule, so a wrong
transcription would have kept the row).

**㊻ Gates and what is not claimed.**  `check-declarations.py` **18 declaration sites (0 forbidden,
18 allowed literals), 0 failures**; `check-citations.py` **676 file(s) scanned, 0 drift, 0 range,
2040 external, 36 allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound,
0 failure(s)** (**61 range rows** now cover `src/pgtkfns.c`); `check-port-lines.py` **0 file(s) under
ten percent, 0 reference line(s), 0 ratchet row(s), 0 new, 0 stale**; `check-ledger-ticks.py`
**238 ledger(s) bound, 0 failure(s)**; `check-cond-eval.py src/pgtkfns.c` **11 conditions, 0 unknown,
0 disagreements [VERIFIED]**; `check-mirror-dupes.py` **16 duplicate(s), 16 ratchet row(s),
0 failure(s)** (after the pgtkselect copy was dropped).  A mirror changed this round, so P-87 ①'s
consumer sweep was run and reads `checked 83 unit(s), 0 failure(s)` with `checking 83 unit(s),
0 unit failure(s)`.  **Not claimed**: no frame geometry was read and no pointer was warped (all five
items are registered by `syms_of_pgtkfns`, still the remainder, so nothing in the port calls them),
`gdk_device_warp`/`gtk_window_get_position`/`gdk_window_get_display` are declarations - the port has
no GTK to link - and the four arm readings are **compile** readings of text this machine cannot
build.
