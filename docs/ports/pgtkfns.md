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
> `femacs/src/pgtkfns.rs` (**2,950** lines, from the 11 of the placeholder), carrying **C 1-1147**,
> **C 1149-1196**, **C 1197-1752** and **C 1754-1996** - four slices - with **both arms of every
> `#if`**; after them the file's next live definition is `Fpgtk_set_resource` (**C 1999**).
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
| 1999-3906 | **the remainder of the file** - from C 1999's `DEFUN ("pgtk-set-resource", …)` ((`Fpgtk_set_resource`, **the next live definition**) through C 3906's `syms_of_pgtkfns`: the resource-key helpers and the `HAVE_GSETTINGS` pair (1999-2021), the ten display subrs (2022-2217), `check_x_display_info`/the scroll-bar defaults/`pgtk_get_string_resource`/`pgtk_get_focus_frame` (2218-2282), the eleven colour/geometry subrs (2283-2576), the tip frame (2577-3311), the geometry/edges/mouse/print/file-dialog/subr tail (3312-3774) and `syms_of_pgtkfns` (3775-3906) | not in these slices: recorded here so the table covers the reference, to be replaced by the next lane's rows |

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
three-line pair.  Rust: 2,578 -> **2,950** lines.

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
appears, and the line-count row moves 34.0% -> **24.5%** (C 3,906 vs Rust 2,950).  The first run read
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
