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
> `femacs/src/pgtkfns.rs` (1899 lines, from the 11 of the placeholder), carrying **C 1-1147** with
> **both arms of every `#if`**.
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
| 1197-3906 | **the remainder of the file** - from C 1197 (`Fx_create_frame`, **the next live definition**) through C 3906's `syms_of_pgtkfns`: `Fx_create_frame` (C 1197-1757), the restack pair (1758-1796), the resource-key helpers and the `HAVE_GSETTINGS` pair (1798-1998), `Fpgtk_set_resource` (1999-2021), the ten display subrs (2022-2217), `check_x_display_info`/the scroll-bar defaults/`pgtk_get_string_resource`/`pgtk_get_focus_frame` (2218-2282), the eleven colour/geometry subrs (2283-2576), the tip frame (2577-3311), the geometry/edges/mouse/print/file-dialog/subr tail (3312-3774) and `syms_of_pgtkfns` (3775-3906) | not in these slices: recorded here so the table covers the reference, to be replaced by the next lane's rows |

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
