# emacs/src/xsettings.c (1421 lines)

> **This file is P-86's class: the frozen configuration compiles none of it.**  C builds
> `xsettings.o` into `XOBJ` when `HAVE_X_WINDOWS` is set (`configure.ac` 7196's
> `XOBJ="xterm.o xfns.o xselect.o xrdb.o xsmfns.o xsettings.o"`) and into `PGTK_OBJ` when the
> window system is `pgtk` (`configure.ac` 3878); `HAVE_X11`/`HAVE_PGTK` are `undef` in
> `docs/baseline/config/darwin-arm64.macros.tsv` (1685, 1489) and `docs/baseline/frozen-build-sources.txt`
> lists no `xsettings.o` - so no object is built from the reference and this target's `build-tu`
> object is the 336-byte empty one.  The file has **no `#if` of its own** that covers it: the gate
> is the *build membership*, the `src/pgtkfns.rs` shape (`PGTK_OBJ` ⇔ `window_system = pgtk`) with a
> two-armed condition.  What this ledger adds is P-86 ②'s substitute shape - the **C line-range
> table** - beside the `## Evidence` arm readings and the `## Deviations`.
>
> **This ledger replaces, not extends, what it said before.**  Until this round it read "the frozen
> configuration does not compile this file ⇒ the port is the empty unit plus the note that names the
> exclusion" (the P-08 reading, `docs/status.md` 3.31), which is what the 4-line placeholder was.
> P-86 ① replaces that with *transcribe the text, both arms, no stub*; what the earlier note measured
> (not in the frozen source list, empty checkbox sections, 336-byte object) is unchanged and still
> true here - it is the *conclusion* that is withdrawn, exactly as `docs/ports/pgtkterm.md`,
> `docs/ports/gtkutil.md`, `docs/ports/xsmfns.md` and `docs/ports/xgselect.md` record for their own
> files.  Port: `src/xsettings.rs` (**2192 lines**, from the 4 of the placeholder), carrying
> **C 1-1421** with **both arms of every `#if`**.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`XOBJ` under `HAVE_X_WINDOWS`, `PGTK_OBJ` under `window_system = pgtk`) | `any (feature = "have_x_windows", feature = "have_pgtk")` (on every item **and on the include graph**, P-86 ⑧) | an X11 or a pure-GTK3 (pgtk) build - not this target, which is why the frozen object is the 336-byte empty one |
| C 29/41 `#ifndef HAVE_PGTK` … C 33/46 `#endif` (the `xterm.h`/`gtkutil.h` pair and the `Xproto.h`/`CARD16`+`CARD32` pair) | `not (feature = "have_pgtk")` / `feature = "have_pgtk"` | one arm each |
| C 48/226 `#ifdef HAVE_GSETTINGS` … C 51/240, C 483/1169 `#endif` (`<glib-object.h>`, `<gio/gio.h>`, the schema names, `gsettings_client`, `xg_settings_key_valid_p`, the three `apply_gsettings_font_*`, `something_changed_gsettingsCB`, `init_gsettings`) | `feature = "have_gsettings"` | a GLib/GIO build |
| C 53/485 `#ifdef HAVE_GCONF` … C 55/527, C 1258 `#endif` (`<gconf/gconf-client.h>`, the three GConf names, `gconf_client`, `something_changed_gconfCB`, `init_gconf`) | `feature = "have_gconf"` **(new name, `## Owed` 1)** | a GConf build |
| C 57 `#ifdef USE_CAIRO` / C 60 `#elif defined HAVE_XFT` / C 62 `#endif` and the `USE_CAIRO \|\| HAVE_XFT` family (C 109, 128, 188, 208, 230, 415, 487, 513, 529, 631, 680, 732, 806, 867, 1028, 1139, 1228, 1409) | `feature = "use_cairo"` / `feature = "have_xft"` (`any (…)` where C spells `\|\|`) | a cairo or an Xft build |
| C 64 `#if defined USE_CAIRO && defined CAIRO_HAS_FT_FONT` / C 66 `#endif` (`<cairo/cairo-ft.h>`) | `all (feature = "use_cairo", feature = "cairo_has_ft_font")` **(new name, `## Owed` 1)** | a cairo with the FreeType backend |
| C 748/894/935 `#ifdef FC_HINT_STYLE` | `feature = "fc_hint_style"` **(new name, `## Owed` 1)** | fontconfig with `FC_HINT_STYLE` |
| C 868 `(defined USE_CAIRO && defined CAIRO_HAS_FC_FONT && defined CAIRO_HAS_FT_FONT)` | `all (feature = "use_cairo", feature = "cairo_has_fc_font", feature = "cairo_has_ft_font")` (`cairo_has_fc_font` is a **new name, `## Owed` 1**) | a cairo with the fontconfig + FreeType backends |
| C 251 `#ifdef GLIB_VERSION_2_32` / C 266 `#else` and C 1101 `#if GLIB_CHECK_VERSION (2, 32, 0)` / C 1113 `#else` | `feature = "glib_version_2_32"` **(new name, `## Owed` 1)** | GLib 2.32 or newer |
| C 1097/1206 `#if ! GLIB_CHECK_VERSION (2, 36, 0)` | `feature = "glib_check_version_lt_2_36"` (already in the tree; `src/gfilenotify.rs` 297) | GLib older than 2.36 |
| C 1303 `#ifdef USE_LUCID` / C 1312 `#endif` (`xsettings_get_system_normal_font`) | `feature = "use_lucid"` | the Lucid toolkit |
| C 531-542's three `#ifndef FC_LCD_*` | none: the header has all three names, so the port declares each once in the fontconfig segment (the `src/ftfont.rs` 2793 reading) | never |

The *file* gate is one `any (…)` expression rather than a `#[cfg]` alias: Rust has no aliasing for
`cfg` predicates without a `build.rs`, so the same six-line condition sits on every item (the
`src/pgtkfns.rs` shape, whose single condition is one term shorter).  The arm flips below also set
`feature = "have_window_system"`, because `HAVE_X_WINDOWS`/`HAVE_PGTK` imply it and the mirrors this
graph reaches (`include/frame.rs`, `include/buffer.rs`, `include/dispextern.rs`'s aliases) are gated
on it - the coupling `docs/ports/xsmfns.md`'s `## Owed` 8 records for its own flips.

## C line ranges

One row per C block, contiguous (each row absorbs the blank lines that follow the one before),
covering C 1-1421 (`docs/port-map.md` pairs `src/xsettings.c` with this unit; there is no
`src/xsettings.h` row for it - the header maps to `src/include/xsettings.rs` separately, and
this table's rows are read against the `.c`'s 1421 lines).

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block (the `/* Functions for handling font and other changes dynamically.` title and the GPL block) | dropped (P-86 ①; the port opens with the `// femacs: port of …` line and this file's gate note) |
| 19-26 | the blank and C 20 `#include <config.h>`; C 22-24 `<float.h>`, `<limits.h>`, `<fcntl.h>`; the blank and C 26 `<byteswap.h>` | file gate on C 23/26's edges (`lib/include/limits.in.rs` 45's `LONG_MAX`, `lib/include/byteswap.in.rs` 27/31's `bswap_16`/`bswap_32`); `<float.h>`/`<fcntl.h>` are named and not edged (nothing in the file's live or dead text uses them, the `src/xsmfns.rs` 383 record) |
| 27-39 | the blank and the eleven includes: `lisp.h` (28), C 29-33's `#ifndef HAVE_PGTK` `xterm.h`/`gtkutil.h` pair, `xsettings.h` (34), `frame.h`/`keyboard.h`/`blockinput.h`/`termhooks.h`/`pdumper.h` (35-39) | file gate; `not (have_pgtk)`/`have_pgtk` on the pair; C 34's edge is *recorded*, not spelled (`E0428`, `## Owed` 2); the order of the edges is the materializer's (`## Deviations` 3) |
| 40-46 | the blank and C 41-46's `#ifndef HAVE_PGTK` `#include <X11/Xproto.h>` / `#else` `typedef unsigned short CARD16; typedef unsigned int CARD32;` | both arms; `CARD16`/`CARD32` stand in place (Xmd.h's, no mirror: `## Owed` 3) |
| 47-55 | the blank; C 48-51's `#ifdef HAVE_GSETTINGS` (`<glib-object.h>`, `<gio/gio.h>`); C 53-55's `#ifdef HAVE_GCONF` (`<gconf/gconf-client.h>`) | `have_gsettings` / `have_gconf`; both surfaces stand in place (GLib/GIO measured, GConf **UNVERIFIED**: `## Owed` 4) |
| 56-66 | the blank; C 57-62's `#ifdef USE_CAIRO` (`<fontconfig/fontconfig.h>`, `"ftfont.h"`) / `#elif defined HAVE_XFT` (`<X11/Xft/Xft.h>`); C 64-66's `#if defined USE_CAIRO && defined CAIRO_HAS_FT_FONT` (`<cairo/cairo-ft.h>`) | `use_cairo` / `have_xft`; fontconfig and cairo measured, Xft **UNVERIFIED** (`## Owed` 4) |
| 67-71 | the blank and C 68-71's four file-scope objects | file gate; `static` linkage kept (`## Deviations` 14) |
| 72-105 | the blank; C 73's `#if defined HAVE_GSETTINGS \|\| defined HAVE_GCONF \|\| !defined HAVE_PGTK`; `store_config_changed_event` (75-87); `dpyinfo_valid` (89-105) with C 97-101's `#ifndef HAVE_PGTK`/`#else` pair; C 105's `#endif` | `any (have_gsettings, have_gconf, not (have_pgtk))`; `not (have_pgtk)`/`have_pgtk` on the pair; C 80's `event.kind` is the per-unit `input_event_bits!` (`## Deviations` 10) |
| 106-124 | the blank and C 109's `#if (defined USE_CAIRO \|\| defined HAVE_XFT) && (defined HAVE_GSETTINGS \|\| defined HAVE_GCONF)`: `store_monospaced_changed` | that gate |
| 125-143 | the blank and C 128's `#if (defined USE_CAIRO \|\| defined HAVE_XFT) && (defined HAVE_GSETTINGS \|\| defined HAVE_GCONF \|\| !defined HAVE_PGTK)`: `store_font_name_changed` + C 143's `#endif` | that gate |
| 144-185 | the blank and C 145's `#if` region: `map_tool_bar_style` (145-167), `store_tool_bar_style_changed` (169-185) | `any (have_gsettings, have_gconf, not (have_pgtk))` |
| 186-194 | the blank and C 187's `#ifndef HAVE_PGTK`: `XSETTINGS_FONT_NAME`/`XSETTINGS_GDK_DPI_NAME`/`XSETTINGS_GDK_WSCALE_NAME` under C 188's `#if`, and `XSETTINGS_TOOL_BAR_STYLE` | `not (have_pgtk)`, the first three also `use_cairo \|\| have_xft`; the one macro C *defines* in live text is a `&CStr` (`## Deviations` 2) |
| 195-205 | the blank and C 196-205's anonymous `enum { SEEN_AA = 0x01, … SEEN_TB_STYLE = 0x80 }` | file gate; the members are `c_uint` (the field's type, PORTING.md 1.3) |
| 206-219 | the blank and C 206-219's `struct xsettings`, with C 208-214's `#if defined USE_CAIRO \|\| defined HAVE_XFT` members | the struct's own two arms, each with its layout assertions |
| 220-224 | the blank and C 221-224's `#ifdef HAVE_PGTK` `font_options` | `have_pgtk` |
| 225-244 | the blank and C 226's `#ifdef HAVE_GSETTINGS`: the six schema/key names and `gsettings_client` | `have_gsettings` (the two `HAVE_PGTK`-only names also `have_pgtk`) |
| 245-271 | the blank and `xg_settings_key_valid_p`, with C 251's `#ifdef GLIB_VERSION_2_32` / C 266 `#else` | `have_pgtk` (the function); `glib_version_2_32` (the pair) |
| 272-318 | the blank and `store_font_options_changed` (274-281) + `apply_gsettings_font_hinting` (283-318) | `have_pgtk` |
| 319-351 | the blank and `apply_gsettings_font_antialias` | `have_pgtk` |
| 352-390 | the blank and `apply_gsettings_font_rgba_order` + C 390's `#endif` | `have_pgtk` |
| 391-483 | the blank and `something_changed_gsettingsCB`, with C 415-444's `USE_CAIRO \|\| HAVE_XFT` arms, C 445-480's `HAVE_PGTK` arms and C 483's `#endif` | `have_gsettings`; `use_cairo \|\| have_xft`; `have_pgtk`; the arm chain is C's inside each gate (`## Deviations` 6) |
| 484-527 | the blank and C 485's `#ifdef HAVE_GCONF`: the three key names, `gconf_client`, `something_changed_gconfCB` (its `CONF_VALUE_STRING` test reads the record's `type` member) | `have_gconf` (the mono-font arm also `use_cairo \|\| have_xft`) |
| 528-542 | the blank and C 529's `#if defined USE_CAIRO \|\| defined HAVE_XFT`'s three `#ifndef FC_LCD_*` fallbacks | that gate; the three names are declared once in the fontconfig segment (no row of their own: the header has them) |
| 543-563 | the blank and C 544's `#ifndef HAVE_PGTK` `get_prop_window` | `not (have_pgtk)` |
| 564-567 | the blank, C 565's `#ifndef HAVE_PGTK` and C 567's `#define PAD(nr)` | `not (have_pgtk)` |
| 568-821 | the blank and `parse_settings` (C 569-820), with C 631-639's `USE_CAIRO \|\| HAVE_XFT` locals, C 680-686's `want_this` arms, C 732-799's arm chain with C 748-763's nested `#ifdef FC_HINT_STYLE`, C 806-817's GDK block and C 821's `#endif` | `not (have_pgtk)`; `use_cairo \|\| have_xft`; `fc_hint_style`; C's `switch` is a `match` (`## Deviations` 7) |
| 822-857 | the blank and `read_settings` + C 857's `#endif` | `not (have_pgtk)` |
| 858-1005 | the blank and C 859's `#ifndef HAVE_PGTK` `apply_xft_settings`, whose whole body is C 867-1003's `#if defined HAVE_XFT \|\| (defined USE_CAIRO && defined CAIRO_HAS_FC_FONT && defined CAIRO_HAS_FT_FONT)` with its inner `HAVE_XFT`/`FC_HINT_STYLE`/`USE_CAIRO` arms | `not (have_pgtk)`; the body's gate; C's `if (changed) … #ifdef HAVE_XFT … #else … #endif … else …` nesting keeps both arms (`## Deviations` 6) |
| 1006-1039 | the blank and `read_and_apply_settings` + C 1039's `#endif` | `not (have_pgtk)` |
| 1040-1086 | the blank and C 1041's `#ifndef HAVE_PGTK` `xft_settings_event` + C 1086's `#endif` | `not (have_pgtk)`; C 1050-1070's `switch` is a `match` over the three `XEvent` member records (`## Deviations` 5, 7) |
| 1087-1170 | the blank and `init_gsettings`, with C 1093-1169's `#ifdef HAVE_GSETTINGS`, C 1097-1099's `! GLIB_CHECK_VERSION (2, 36, 0)`, C 1101-1120's `GLIB_CHECK_VERSION (2, 32, 0)` pair, C 1139-1167's `USE_CAIRO \|\| HAVE_XFT` block and C 1160-1165's `HAVE_PGTK` block | `have_gsettings`; `glib_check_version_lt_2_36`; `glib_version_2_32`; `use_cairo \|\| have_xft`; `have_pgtk` |
| 1171-1196 | the blank and C 1174's `#if defined HAVE_PGTK && defined HAVE_GSETTINGS` `xg_get_system_dark_mode` | `have_pgtk && have_gsettings` |
| 1197-1259 | the blank and `init_gconf`, with C 1203-1258's `#if defined (HAVE_GCONF)`, C 1206-1208's `! GLIB_CHECK_VERSION (2, 36, 0)` and C 1228-1257's `USE_CAIRO \|\| HAVE_XFT` block | `have_gconf`; `glib_check_version_lt_2_36`; `use_cairo \|\| have_xft` |
| 1260-1281 | the blank and C 1261's `#ifndef HAVE_PGTK` `init_xsettings` | `not (have_pgtk)` |
| 1282-1292 | the blank and `xsettings_initialize`, with C 1288-1290's `#ifndef HAVE_PGTK` | file gate; `not (have_pgtk)` on the one call |
| 1293-1301 | the blank and `xsettings_get_system_font` | file gate; non-`static` (`## Deviations` 14) |
| 1302-1312 | the blank and C 1303's `#ifdef USE_LUCID` `xsettings_get_system_normal_font` | `use_lucid` |
| 1313-1327 | the blank and C 1314's `#ifdef HAVE_PGTK` `xsettings_get_font_options` | `have_pgtk` |
| 1328-1364 | the blank and the three `DEFUN`s (C 1329-1337 `font-get-system-normal-font`, C 1339-1346 `font-get-system-font`, C 1348-1364 `tool-bar-get-system-style`) | file gate; the docstrings are C's bytes (the subr rule) |
| 1365-1421 | the blank and `syms_of_xsettings` (C 1366-1421), with C 1375-1386's three `#ifdef`s, C 1409-1414's `USE_CAIRO \|\| HAVE_XFT` + `(HAVE_GCONF \|\| HAVE_GSETTINGS)` pair, and the two variables C declares through `DEFVAR_BOOL`/`DEFVAR_LISP` | file gate; those sub-gates; the two variables are defined in the unit (`## Deviations` 17) |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/xset-lane/cargo cargo run -q -p xtask -- build-tu src/xsettings.c`
-> `built src/xsettings.rs -> ../build/femacs/xsettings.o`, **exit 0**, and
`build/femacs/xsettings.o` is **336 bytes**, md5 `dbed68837694579185377dbb0821ab62`,
sha1 `e0431432b85052827312f8a0860ed38ef4868da8` - the **empty object** (`include/config.rs`, the one
`use std::ffi::CStr;` and the `#![allow]` are all the frozen feature set compiles), which is the
expected reading for a file every item of which sits behind the file gate (P-86 ⑧'s criterion).
**Four `build-tu` runs** (budget 3, over by one): the mid-round draft, the reading after the include
reordering, and the two final ones - all four the same 336 bytes, because everything that changed
between them is inside the gate.

**The revision every reading below is of**: `src/xsettings.rs`, **2192 lines**,
129,497 bytes, sha1 `fd4598d817d8fbc412ea04a9fb9490932ad1c0c7` (restored byte-identical after the
mutation test; `wc -l`/`shasum` re-run after the restore).

**② The measurements this file's declarations are fixed to** (P-88 ②; every value below is a clang
read of a header *on this machine*, and each is quoted in the unit beside its declaration).

*X11* (`/opt/homebrew/include`; libX11 1.8.13 + xorgproto 2025.1).  A nine-line probe plus
`sizeof`/`offsetof` prints: `CARD16=2 CARD32=4` (Xmd.h); `PropertyNewValue=0 DestroyNotify=17
PropertyNotify=28` (X.h 132/137/140); `LSBFirst=0 MSBFirst=1` (Xlib.h 79-80); `AnyPropertyType=0
Success=0 StructureNotifyMask=131072 PropertyChangeMask=4194304 ClientMessage=33 True=1 False=0`;
`sizeof XAnyEvent=40` (`type`@0, `serial`@8, `send_event`@16, `display`@24, `window`@32);
`sizeof XClientMessageEvent=96` (`message_type`@40, `format`@48, `data`@56, `data.l[1]`@64);
`sizeof XPropertyEvent=64` (`atom`@40, `time`@48, `state`@56).  The seven prototypes this unit
declares in place are quoted with their header lines (`XGrabServer` 2565, `XUngrabServer` 2571,
`XGetSelectionOwner` 1647, `XSelectInput` 3001, `XGetWindowProperty` 3020, `XFree` 1936,
`XScreenNumberOfScreen` 1839-1840).
*GLib/GIO* (`/opt/homebrew/include/glib-2.0`, GLib 2.90.0 + the generated `glibconfig.h`): the seven
opaque records are one pointer each (8 bytes); `G_VARIANT_TYPE_STRING`'s bytes are `"s"`
(`glib/gvarianttype.h` 122's `((const GVariantType *) "s")`); the thirteen GIO/GObject prototypes
quote gobject.h 459/509, gio/gsettings.h 76/80/112, gio/gsettingsschema.h 38/51/68/78 and
glib/gvariant.h 61/65/76/158.
*fontconfig* (`/opt/homebrew/include/fontconfig`, 2.18.3): `FcBool=4 FcChar8=1 FcChar32=4`;
`FC_HINT_NONE/SLIGHT/MEDIUM/FULL = 0/1/2/3` (208-211); `FC_RGBA_RGB/BGR/VRGB/VBGR/NONE = 1/2/3/4/5`
(201-205); `FC_LCD_NONE/DEFAULT = 0/1` (214-215); `FcMatchPattern=0` (336); the six `FC_*` name
macros' bytes (98/99/100/116/117/132).
*cairo* (`/opt/homebrew/include/cairo`, 1.18.4): `CAIRO_ANTIALIAS_NONE/GRAY/SUBPIXEL = 1/2/3`;
`CAIRO_HINT_STYLE_NONE/SLIGHT/MEDIUM/FULL = 1/2/3/4`; `CAIRO_SUBPIXEL_ORDER_RGB/BGR/VRGB/VBGR =
1/2/3/4`; `cairo-features.h` 10/12 define `CAIRO_HAS_FC_FONT`/`CAIRO_HAS_FT_FONT` as 1 on this
machine (which is why the two names are *features*: a cairo without them is C 868's `#else`).
**No GConf, Xft, GDK or GTK header exists here** (`ls -d /opt/homebrew/include/gconf
/opt/homebrew/include/X11/Xft/Xft.h /opt/homebrew/include/gtk-3.0` -> all absent), so those
declarations are header-only and stay **UNVERIFIED** (`## Deviations` 5).

**③ The arm flips (P-86 ③⑴/⑥) and the mutation test (P-86 ⑦).**  The toolchain is the repo's pin:
`rust-toolchain.toml` is `nightly-2026-10-01` (P-04), and `xtask probe`'s inner `rustc`
inherits it through cargo - a *hand* run must set `RUSTUP_TOOLCHAIN=nightly-2026-10-01`, or the
machine's stable 1.98.1 adds a spurious `error[E0658] … unstable library feature c_variadic` at
`include/lisp.rs` 3761 (measured both ways; the `docs/ports/kqueue.md` 142 / `gnutls.md` 246
reading).  The reported `probe` readings, with errors split into *this unit's own text* (the
`_root.rs` region) and *a mirror copy* (named by file and line):

| # | `probe` run | gate set | exit | errors | in the unit | in a mirror copy |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | mid-round | `have_window_system have_x_windows` | 1 | 2 | **1**: `_root.rs:851 error[E0425]: cannot find type CStr` (the `&CStr` constant added without its import - fixed) | 1: `include/systime.rs:27` (below) |
| 2 | **Arm A** | the same, final text | 1 | **1** | **0** | **1**: `include/systime.rs:27 error[E0080]: evaluation panicked: assertion failed: !cfg!(feature = "have_x_windows") \|\| HAVE_X_WINDOWS` |
| 3 | **Arm B** | `have_window_system have_pgtk have_gsettings use_cairo glib_version_2_32 have_gtk3`, final text | 1 | **2** | **0** | **2**: `include/pgtkgui.rs:122 error[E0428]: the name XrmDatabase is defined multiple times` and `:130 … XRectangle …` (both against `include/xlib.rs` 871/38) |
| 4 | mutation, first attempt | Arm A's command | 1 | 4 | **3**: `_root.rs:1575/1579/1580 error[E0308]: mismatched types` - `let dpy: c_int = (*dpyinfo).display;` and its two uses | 1: the same `include/systime.rs:27` |
| 5 | mutation, **single site** (the reported run) | Arm A's command | 1 | **2** | **1**: `_root.rs:1295 error[E0308]: mismatched types` - the injected `XGrabServer (dpy as c_int)` at the C 552 site | 1: the same `include/systime.rs:27` |

Run 3's reading is what the direct route below produces for the same gate set, error for error (2
`E0428`s, no warnings) - which is the equivalence the sub-gate matrix relies on.

*Arm A* is C's X11 shape: the live body (`parse_settings`, `read_settings`, `get_prop_window`,
`read_and_apply_settings`, `xft_settings_event`, `init_xsettings`, the `map_tool_bar_style`/
`store_*` helpers, the three subrs and `syms_of_xsettings`) type-checks against the mirror set, and
the only error is `include/systime.rs` 27's gate-consistency assert - the **hand-flip artifact**
`docs/ports/xsmfns.md`'s `## Owed` 8 records (`include/config.rs` hard-codes
`HAVE_X_WINDOWS: bool = false`; the mechanism that rewrites it is `xtask dead-arms`, out of this
lane's budget).  *Arm B* is C's pgtk shape (the pgtk + GSettings + cairo text), and its two errors
are the pgtkgui/xlib record clash.  Both reported runs: **zero errors in this unit's own text**.
*Mutation test*: injecting one type error at the C 552 site into the unit's own text and re-running
the **same** Arm A `probe` command turns 1 error into 2 - exactly one new row, at the injected
line - so the "0 in the unit" reading is a real reading of this unit's text and not a
name-resolution abort (P-86 ⑦'s criterion; the first attempt's three-new-rows reading is kept in the
table because a *multi-site* injection is the weaker form of the same evidence).  The file was then
restored byte-identical (`sha1 fd4598d817d8fbc412ea04a9fb9490932ad1c0c7`, `diff` empty).
**Five `probe` invocations against a budget of three** (over by two): the mid-round Arm A, the final
Arm A, Arm B, and the two mutation runs - every one of them reported above with its exit code and
its error split.

*The rest of the sub-gate matrix* (the remaining `#if` arms), read with the **same flag set and the
same toolchain** by direct `rustc` on the **same materialized root** `xtask` wrote (the practice
`docs/ports/xsmfns.md` names: the direct route reproduces `probe`'s reading exactly - verified here
by re-running Arm A both ways, and by diffing `xtask`'s root against the lane's re-materializer's
byte for byte):

| arm | gate set | exit | errors | in the unit |
| --- | --- | --- | --- | --- |
| A | `have_window_system have_x_windows` | 1 | 1 (`systime.rs:27`) | 0 |
| A + cairo | A `+ use_cairo cairo_has_ft_font cairo_has_fc_font fc_hint_style` | 1 | 2 (`systime.rs:27`, `include/font.rs:521 E0133`) | 0 |
| A + Xft | A `+ have_xft fc_hint_style` | 1 | 1 (`systime.rs:27`) | 0 |
| A + GConf | A `+ have_gconf use_cairo` | 1 | 2 (`systime.rs:27`, `font.rs:521`) | 0 |
| A + Lucid | A `+ use_lucid` | 1 | 1 (`systime.rs:27`) | 0 |
| B | `have_window_system have_pgtk have_gsettings use_cairo glib_version_2_32 have_gtk3` | 1 | 2 (`pgtkgui.rs:122/130`) | 0 |
| B - glib 2.32 | B **without** `glib_version_2_32` (C 1113's `#else` arm) | 1 | 2 (`pgtkgui.rs:122/130`) | 0 |

So every arm of every `#if` in the file has been read at least once, and **no reading has an error in
this unit's own text**: the four residual error rows are three mirror properties
(`systime.rs` 27's hand-flip assert, `pgtkgui.rs` 122/130's duplicate X11 records, `font.rs` 521's
`gc_in_progress` access) and the `c_variadic` row that a non-pinned toolchain adds.

**④ The acceptance-gate readings (this round).**

* `python3 tools/extract-abi.py --check emacs/src/xsettings.c src/xsettings.rs` -> **one
  problem, and it is the expected one**: `- line count drifted 54.3%: C 1421 vs Rust 2192`.  The
  other five rule classes pass: the 16 live functions, the 3 subrs (name, limits and docstring
  digest), the 2 live macros, the 9 constants (the `SEEN_*` values and
  `XSETTINGS_TOOL_BAR_STYLE`'s bytes), the `struct xsettings` fields and the 4 file-scope objects all
  match, and the per-body token check has no missing token left (its three exemptions are the
  `docs/ports/macro-body-notes.tsv` rows this round appends: `parse_settings`,
  `xft_settings_event`, `syms_of_xsettings`).  The tolerance row is the **integrator's** (`## Owed` 7);
  the measured drift is 54.3% (the port carries C's own comment blocks, a `// C <line>` citation or
  a `/// C:` line per item, the file gate on every item, and the in-place third-party surfaces - the
  class `docs/ports/xsmfns.md`'s 78.1% row and `docs/ports/pgtkim.md`'s 94.5% row belong to).
* `python3 tools/check-declarations.py` -> `17 declaration sites (0 forbidden, 17 allowed literals),
  0 failures`, and **this unit and this ledger contribute no site of any class**: the one comment
  that carried the reference's bug address (C 962-963's two message URLs) was rewritten
  (`## Deviations` 12).  Mid-round the tree read `18 sites / 1 forbidden / 1 failure`, the row being
  this unit's: the second of those two URLs contains the product name the gate's `title` class
  looks for.
* `python3 tools/check-mirror-dupes.py` -> `17 duplicate declaration(s), 17 ratchet row(s), 0
  failure(s)` (the same reading as before this round: this unit's use-site declarations name nothing
  a mirror of its graph carries - checked twice, once with the `g_object_get` copy the
  `include/gtk.rs` 186 declaration made a duplicate, removed).
* `python3 tools/check-citations.py` -> `676 file(s) scanned, 0 drift, 0 range, 1503 external, 36
  allowed, 0 stale`; this unit's `/// C:` lines are all `external` (the SDK headers it declares in
  place, each quoted with its line).
* `python3 tools/check-ledger-ticks.py` -> `233 ledger(s) bound (a function tick to compare), 0
  failure(s)` (this ledger has no tick section: P-86 ②'s range table replaces it).
* `python3 tools/check-dead-file-coverage.py` -> `99 ledger(s) bound (a C range table), 0
  failure(s)`, with `info docs/ports/xsettings.md: 40 range row(s) cover src/xsettings.c (1421
  lines; 0 blank line(s) between blocks need no row)` (the same 40 rows cover `src/xsettings.h`'s 51
  lines, which this ledger is also bound to through `src/include/xsettings.rs`).

## Deviations

1. **The file gate is the build membership, not a C `#if`** (`## Gate mapping`): no single macro in
   the file covers it, so `any (feature = "have_x_windows", feature = "have_pgtk")` stands on every
   item and on the include graph.  Rust cannot name a `cfg` predicate without a `build.rs`, so the
   condition is repeated rather than aliased.
2. C 193's `XSETTINGS_TOOL_BAR_STYLE` is a `&CStr` rather than the
   `*const c_char = c"…".as_ptr ()` the rest of the file's string macros get: `extract-abi.py`'s
   constants rule compares a *string macro*'s bytes only against a string literal (the
   `src/ftfont.rs` 90 use stands behind it), and its two call sites append `.as_ptr ()`.
3. **The include edges are in the materializer's order, not C's** (P-86 ⑧).  `xtask` dedupes by
   *file* and the first edge in the walk claims a mirror's one expansion slot, so the records this
   unit's own text needs in *both* arms (`frame.h`, `font.h`, `window.h`, `buffer.h`,
   `keyboard.h`, `blockinput.h`, `termhooks.h`, `pdumper.h`) are claimed **before** C 30/C 32's
   conditional chains, whose nested edges would otherwise take the slot and leave the file out of
   the other configuration's graph.  `include/gtk.rs` moves for the same reason and with a wider
   gate (`use_cairo`/`use_gtk` added): `include/xterm.rs` 53 has an **ungated** `include!("gtk.rs")`
   of its own (C 69's `#ifdef USE_GTK`), which otherwise claimed the surface's slot for a
   configuration the unit's own edge does not reach.  Every comment keeps C's own line.  Related,
   and measured: a `#[cfg]` written *across two lines* above an `include!` reads as **ungated** to
   `xtask` (`include_edges` looks at the current line's start only) - this lane's C 30 edge was
   wrapped at first and made `xterm.rs`'s whole chain "ungated reachable" (`## Owed` 10).
4. `Display_Info` (dispextern.h 115/152) is spelled through **this unit's own module**
   (`xsettings_dpi`, the P-21/P-39 shape, PORTING.md 0.1-10) rather than declared at the top level:
   `include/dispextern.rs` 698's alias of that name points at *its own module's opaque stand-in* for
   C 115's tag (its 2757-2758's `pub enum x_display_info {}`), and its pgtk arm (711) is a recorded
   comment with no alias at all, so a top-level alias here would be `E0428` against the mirror's in
   the X arm (measured, Evidence ③ row 1's neighbourhood).  The two arms are two different records
   (`x_display_info` / `pgtk_display_info`), which is what the module selects.
5. **The third-party surfaces stand in this unit** (P-88 ①), at the C `#include` line each belongs
   to, because no mirror in this tree carries them: the six Xlib entry points, the five X.h/Xlib.h
   constants, the three `XEvent` member records C 1050-1067 reads (the `src/widget.rs` 575
   `XExposeEvent` shape: the mirror's `XEvent` is an opaque 192-byte record) and
   `XScreenNumberOfScreen`; the GIO/GSettings half (its home would be an `include/gio.rs`); the
   fontconfig half (`src/ftfont.rs` 95-160 declares the same surface for its own consumers); the
   cairo font-options half; and the four names the mirrors' *own* headers declare but the mirrors
   lack (`dupstring`, `make_formatted_string`, `x_catch_errors`/`x_uncatch_errors`, `strstr`,
   `xg_update_dark_mode_for_all_displays`).  Every X11, fontconfig, cairo and GIO declaration is a
   clang reading (Evidence ②); **GConf, Xft, GDK and GTK are not on this machine**, so those stay
   header-only and **UNVERIFIED** (the `src/xsmfns.rs` 17 XSMP/ICE treatment).
6. C's `#if`-gated **`else if` chains** are carried inside C's `else` with each gate on a fresh
   chain (the `src/termcap.rs` 313-330 shape; the tested keys are distinct constants, so the
   flattened chain is C's): C 415-444 and C 445-480 in `something_changed_gsettingsCB`, C 513-524 in
   `something_changed_gconfCB`, C 732-799's arm chain in `parse_settings`.  C 748-763's *nested*
   `#ifdef FC_HINT_STYLE` keeps its arm with the **body** gated instead, because a Rust `#[cfg]`
   cannot sit between an `else` and its `if` (`error: expected expression, found keyword 'else'`,
   measured): C's arm *absence* is exactly the trailing `else`'s behaviour for that name, so the
   gated body spells it (`want_this = false`).  In `apply_xft_settings`, C's
   `if (changed) { … #ifdef HAVE_XFT … #else … #endif … } else …` keeps both arms with C's own
   nesting (the `else` calls `FcPatternDestroy` in both configurations, the cairo arm's *then*
   branch calls `XftDefaultSet` where the Xft one does).
7. `switch`/`case`/`default`/`break` become `match` arms with the `_` arm as C's `default:` (or its
   fall-through where C has none), and the tokens are recorded in
   `docs/ports/macro-body-notes.tsv`: `parse_settings` (C 688-723),
   `xft_settings_event` (C 1050-1070), `syms_of_xsettings` (`doc`, the DEFVAR positionally, the
   `syms_of_data` precedent).
8. C's implicit conversions are spelled: the signed/unsigned comparisons in `parse_settings`
   (`(bytes_parsed + 4) as c_ulong`, `bytes_parsed as c_ulong`), `AnyPropertyType as Atom`,
   `data.l[1] as c_ulong`, `ival as c_double`, `(ival != 0) as FcBool`, `gdk_* as c_int`,
   `G_OBJECT! (settings) as gpointer`, `oldsettings.aa as c_int` in the variadic call, `xfree (… as
   *mut c_void)` and `prop as *mut c_void`.
9. Values C leaves indeterminate get an initializer (PORTING.md 1.5): `n_settings` (C 627), `nlen`
   (C 654), `vlen` (C 655), `act_type`/`act_form`/`nitems`/`bytes_after` (C 831-833), `schema`
   (C 252) and the `input_event` record C 78's compound literal zero-fills.
10. C 80's `event.kind = CONFIG_CHANGED_EVENT;` is the per-unit `input_event_bits!` macro (termhooks.h
   373-412's 16-bit `kind` shares `include/termhooks.rs`'s `input_event::bits`, PORTING.md 1.3/P-80;
    the macro is body-identical to `src/inotify.rs` 289-293 and `src/xsmfns.rs` 460-468).
11. C 979's chained `dpyinfo->resy = dpyinfo->resx = settings->dpi;` is two statements (C's
    right-to-left assignment order kept), and C's `*schemas++` (C 1118) is the test then the step.
12. C 962-963's two mailing-list URLs are **paraphrased**: their text carries the reference's bug
    address, which `tools/check-declarations.py`'s `title` class forbids in a comment (`## Evidence`
    ④'s mid-round reading was this unit's row).
13. Three gates are *derived* conditions rather than `#if`s of this file: `use_lucid` (C 1303's
    `#ifdef USE_LUCID`), `glib_version_2_32` (C 251's `GLIB_VERSION_2_32` and C 1101's
    `GLIB_CHECK_VERSION (2, 32, 0)` - the same "GLib 2.32 or newer", so both carry it) and
    `glib_check_version_lt_2_36` (C 1097/1206, the name `src/gfilenotify.rs` 297 already uses).
14. Linkage: the six non-`static` definitions (`xsettings_initialize`, `xft_settings_event`,
    `xsettings_get_system_font`, `xsettings_get_system_normal_font`, `xsettings_get_font_options`,
    `xg_get_system_dark_mode`) plus `syms_of_xsettings` carry `#[unsafe(no_mangle)]` (PORTING.md
    0.2); the four file-scope objects, the helpers and the two callbacks stay plain `unsafe fn`s /
    `static mut`s.
15. C 34's `#include "xsettings.h"` has no Rust spelling in this unit (`E0428` against
    `src/include/xsettings.rs`'s three declarations) - recorded, with the owner, in
    `## Owed` 2.
16. C's `# ifdef FC_HINT_STYLE`-guarded `SEEN_HINTSTYLE` arm and the `#ifndef FC_LCD_*` fallbacks
    (C 531-542) are the three constants declared once in the fontconfig segment: the header has all
    three names, so the `#ifndef`s are no-ops (the `src/ftfont.rs` 2793 reading of the same pair).
17. The two variables C declares through `DEFVAR_BOOL`/`DEFVAR_LISP` (`use_system_font`,
    `Vxft_settings`) are **defined in this unit** (`#[unsafe(no_mangle)] pub static mut`), because
    they live in `struct emacs_globals` and `tools/check-globals.py` binds them to the unit that
    DEFVARs them (the `src/xsmfns.rs` 1015-1020 shape).
18. The `/// C:` lines above an `unsafe extern "C" { … }` block are *inside* the block (above the
    first declaration), which is what keeps them from being `unused doc comment` warnings - the
    mirror set's own convention (`include/xlib.rs` 1170-1190's sections); the six warnings the arm
    flips do emit are mirrors' (`include/frame.rs` 816/1360, `include/lisp.rs` 3939,
    `include/stdbit.in.rs` 644, `include/xterm.rs` 725/1508).

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **`xtask/Cargo.toml`'s `[features]` table: five names this lane introduces and cannot
   register.**  `have_gconf` (HAVE_GCONF), `cairo_has_ft_font` (CAIRO_HAS_FT_FONT), `cairo_has_fc_font`
   (CAIRO_HAS_FC_FONT), `fc_hint_style` (FC_HINT_STYLE) and `glib_version_2_32`
   (GLIB_VERSION_2_32).  Until they are in the table the arm flips that set them are hand runs, not
   `gate-probes` runs.  The four `cairo_has_*`/`fc_*` names are the same class `docs/ports/pgtkfns.md`'s
   Owed 4 and `src/ftfont.rs`'s Owed 5 name.  **NOT DONE.**
2. **`src/include/xsettings.rs`'s three declarations are `E0428` against this unit's
   definitions** (`xsettings_initialize`, `xft_settings_event`, `xsettings_get_system_font`), so
   C 34's `#include "xsettings.h"` is *recorded* here rather than spelled.  The fix is the P-39
   shape the `src/xgselect.rs` lane already applied to its own mirror: a `pub mod xsettings_decls
   { … }` with `use xsettings_decls::*;` around the three `unsafe extern "C"` lines.  The mirror is
   outside this lane's write scope.  **NOT DONE.**
3. **`src/include/xlib.rs`: the X11 surface this unit declares in place belongs there** -
   six entry points (`XGrabServer` 2565, `XUngrabServer` 2571, `XGetSelectionOwner` 1647,
   `XSelectInput` 3001, `XGetWindowProperty` 3020, `XFree` 1936, each with its measured line), the
   five constants (`PropertyNewValue`, `DestroyNotify`, `PropertyNotify`, `LSBFirst`, `MSBFirst` -
   the module's `x11_constants`), the three `XEvent` member records its own C 1162 note owes
   (`XAnyEvent`/`XClientMessageEvent`/`XPropertyEvent`) and `XScreenNumberOfScreen` (1839).  The
   records' layouts and the prototypes are measured (Evidence ②), so moving them is a copy plus the
   `x11_constants` module's `use`.  **NOT DONE.**
4. **A shared `include/fontconfig.rs`, `include/cairo.rs` and `include/gio.rs`** for the surfaces
   this unit and `src/ftfont.rs`/`src/xfont.rs`/`src/xftfont.rs` (fontconfig, cairo) and
   `src/gfilenotify.rs` (GIO) each declare in place - P-90 ①'s one-surface-one-mirror rule, which
   `src/ftfont.rs` 95-160's Owed 4 already names.  **NOT DONE.**
5. **Mirror gaps this unit works around at the use site**: `dupstring` (lisp.h 5555) and
   `make_formatted_string` (lisp.h 4542) are missing from `include/lisp.rs`; `strstr` is missing from
   `lib/include/string.rs`; `x_catch_errors`/`x_uncatch_errors` (xterm.h 1752/1764) are missing from
   `include/xterm.rs`; `xg_update_dark_mode_for_all_displays` (gtkutil.h 190) is a *comment* in
   `include/gtkutil.rs` rather than a declaration.  Each is declared here with its reference line and
   a gate, and moves when its mirror lands.  **NOT DONE.**
6. **`src/include/dispextern.rs` 698 and 711**: the X-arm `Display_Info` alias points at that
   mirror's module-local *opaque* stand-in for C 115's tag, and the pgtk arm is a recorded comment
   with no alias at all, so a unit that dereferences the record cannot use the name
   (`## Deviations` 4).  A `have_pgtk` arm like the other five (and an X-arm alias to the complete
   record, or a documented rule that units spell the record) would let this unit drop its own
   module.  **NOT DONE.**
7. **`docs/ports/line-drift-tolerance.tsv`: this stem has no row**, so `extract-abi.py --check`
   reports the lone `line count drifted 54.3%: C 1421 vs Rust 2192` FAIL.  The row (`total` basis,
   tolerance the measured value + 0.01, the reason = C's comment blocks + the per-item citation and
   gate lines + the in-place third-party surfaces) is the integrator's, and it is the last thing
   between this unit and a clean `--check`.  **NOT DONE.**
8. **The line-count ratchet needs nothing** (measured, so the integrator does not have to look):
   `python3 tools/check-port-lines.py` reads `28 file(s) under ten percent, 38,276 reference
   line(s); 28 ratchet row(s), 0 new, 0 stale`, and this stem is no longer among the 28 - the port
   is 2192 lines against C's 1421, so the placeholder's row (if it had one) is gone and none is
   owed.  **NOTHING TO DO (measurement recorded).**
9. **A `## Deviations`-class record, not this unit's gap: the four mirror properties the arm flips
   report** (Evidence ③): `include/systime.rs` 27's `E0080` (the hand-flip artifact - `xtask
   dead-arms` is the mechanism that rewrites `include/config.rs`'s frozen constants), `include/pgtkgui.rs`
   122/130's `XrmDatabase`/`XRectangle` duplicates against `include/xlib.rs` 38/871 (the P-72 class
   `include/pgtkgui.rs`'s own ledger should carry), `include/font.rs` 521's bare `gc_in_progress`
   read (edition 2024 wants an `unsafe` block there; landed units do not reach it because
   `have_window_system` is off in the frozen configuration) and `include/lisp.rs` 3761's
   `c_variadic` under a non-pinned toolchain.  Each is named with its owner lane.  **NOT DONE (the
   mirrors' own rounds).**
10. **`xtask/src/main.rs`'s `include_edges` only reads the *current* line's start**, so a
    `#[cfg]` written across two lines above an `include!` makes the edge read as **ungated** (this
    lane measured it: the wrapped C 30 gate made `include/xterm.rs`'s whole chain ungated-reachable,
    which changed `xtask`'s dedupe decisions and silently widened the graph).  Either the reader
    should join a wrapped attribute with its next line, or PORTING.md should state the one-line rule
    for `#[cfg]` above `include!` (this unit follows the one-line form, `## Deviations` 3).
    **NOT DONE.**

## Not claimed

- **Zero run evidence.**  The `rlib`s and the object are compile readings: nothing was linked,
  started, dumped or exercised, and none of the file's behaviour (the property parser, the DPI
  comparisons, the GSettings/GConf callbacks, the event filter) has been observed.
- **Not an X11, GTK, pgtk or GConf build.**  The arm flips compile against this tree's own mirror
  set: the X11, fontconfig, cairo and GIO declarations are measured against real headers on this
  machine, but `include/gtk.rs`'s GDK/GTK half and this unit's GConf, Xft and GTK-typed
  declarations are header-only placeholders and stay **UNVERIFIED** (Evidence ②/P-88 ②).
- The frozen `build-tu` object is the **336-byte empty** one, so it says nothing about the body; the
  "0 errors in this unit" readings are *name-and-type* readings of the mirror set, weaker than a
  build against the real SDKs (P-86 ⑦), and the four residual error rows are mirrors', not this
  unit's.
- **No differential or behavioural evidence exists for this unit**, and no C reference object exists
  to differ against on this target (the file is dead in the frozen configuration).
- The 54.3% line drift is *reported*, not justified as acceptable: the tolerance row is the
  integrator's (`## Owed` 7), and nothing here claims the port is line-for-line C.
- The `probe` budget was exceeded (five runs against three) and the `build-tu` budget by one (four
  against three); both are reported rather than smoothed over (Evidence ①/③).
