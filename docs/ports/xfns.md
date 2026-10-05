# emacs/src/xfns.c (10662 lines) — first slice: C 1-802

> **P-86's first X11 unit, ported in slices.**  The frozen configuration compiles nothing here:
> `configure.ac` 7521 fills `WINDOW_SYSTEM_OBJ="fontset.o fringe.o image.o"` only under
> `HAVE_WINDOW_SYSTEM`, that macro is `undef` in
> `docs/baseline/config/darwin-arm64.macros.tsv`, and `docs/baseline/frozen-build-sources.txt`
> lists no `xfns.o`, so `extract-abi.py`'s checklist for the file is empty and its checkbox
> sections cannot be coverage evidence.  What follows is P-86 ②'s substitute: the **C line-range
> table** for this slice, the `## Evidence` readings and the `## Deviations`.  Port:
> `femacs/src/xfns.rs` (**1,120 lines** after this slice, measured with `grep -c ''`), both arms of
> every `#if` carried.
>
> **This ledger replaces the previous revision's conclusion** ("the file is not compiled by the
> frozen configuration, so the port is the empty unit plus a note"), exactly as
> `docs/ports/android.md` did with the same finding: P-86 ① rules that a three-line header comment
> is *not started*, not *done*.  The old note's measurements are still true and are still where the
> *file gate* comes from; what changed is the conclusion drawn from them.
>
> **The slice is C 1-802**, ending at the `}` of `x_set_alpha_background` (C 802).  It holds the
> include graph (C 20-126), the Motif hint block (C 134-161), `check_x_display_info` (C 171-205),
> `x_real_pos_and_offsets` (C 216-588), `x_real_positions` (C 594-599),
> `x_relative_mouse_position` (C 604-646), `gamma_correct` (C 650-659), `x_defined_color`
> (C 667-685), `x_decode_color` (C 692-718), `x_set_wait_for_wm` (C 726-730) and
> `x_set_alpha_background` (C 732-802).  **The next live definition is `x_set_tool_bar_position`
> (C 805)** - C 804 is its `static void` line - and the table's last row is "the remainder".

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`WINDOW_SYSTEM_OBJ`, i.e. `HAVE_WINDOW_SYSTEM`; `configure.ac` 7521) | `feature = "have_x_windows"` (on every item and on C 20-126's include graph) | an X11 configuration |
| C 40 `#ifdef HAVE_X_I18N` … C 42 (`textconv.h`) | `all (feature = "have_x_windows", feature = "have_x_i18n")` | an X11 configuration with X11 I18N |
| C 47 `#ifdef USE_XCB` … C 50 (the `<xcb/*>` headers) | `all (feature = "have_x_windows", feature = "use_xcb")` | X11, XCB transport |
| C 55 `#ifdef HAVE_XRANDR` … C 57 | `feature = "have_xrandr"` (**not registered** in `femacs/xtask/Cargo.toml`; owed, below) | an X11 configuration with Xrandr |
| C 58 `#ifdef HAVE_XINERAMA` … C 60 | `feature = "have_xinerama"` (**not registered**; owed) | an X11 configuration with Xinerama |
| C 62 `#ifdef USE_GTK` … C 64 (`gtkutil.h`) | `all (feature = "have_x_windows", feature = "use_gtk")` | the GTK configuration |
| C 66 `#ifdef HAVE_XDBE` … C 68 | `all (feature = "have_x_windows", feature = "have_xdbe")` | an X11 configuration with Xdbe |
| C 70 `#ifdef HAVE_XINPUT2` … C 72 | `all (feature = "have_x_windows", feature = "have_xinput2")` | an X11 configuration with XInput2 |
| C 74 `#ifdef USE_X_TOOLKIT` … C 122 | `all (feature = "have_x_windows", feature = "use_x_toolkit")` | an X toolkit configuration |
| C 78 `#ifdef HAVE_XAW3D` … C 84 (C 81 the `#else`) | `feature = "have_xaw3d"` (**not registered**; owed) | a Lucid toolkit configuration with Xaw3d |
| C 87 `#ifdef USG` … C 95 (C 93-95 the `#else`) | `feature = "usg"` for C's arm; `<X11/Xos.h>` is the toolchain's header and has no mirror, so both arms stand as the port's comment and there is no Rust item to gate (P-88 ①) | a USG-derived X11 configuration |
| C 101 `#ifdef USE_MOTIF` … C 108 | `feature = "use_motif"` (**not registered**; owed) | the Motif configuration |
| C 110 `#ifdef USE_LUCID` … C 112 (`xlwmenu.h`) | `all (feature = "have_x_windows", feature = "use_lucid")` | the Lucid configuration |
| C 124 `#ifdef USE_GTK` … C 126 (empty) | `feature = "use_gtk"` | the GTK configuration |
| C 130 `#ifdef GLYPH_DEBUG` … C 132 (`dpyinfo_refcount`) | `all (feature = "have_x_windows", feature = "glyph_debug")` | a GLYPH_DEBUG configuration |
| C 134-135 `#ifndef USE_MOTIF` / `#ifndef USE_GTK` … C 160-161 | `all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk"))` | neither Motif nor GTK |
| C 251/279/292/331/343/383/416/450/482/490/539/557/567 (`USE_XCB` and its `#else` partners, inside `x_real_pos_and_offsets`) | `use_xcb` / `not (use_xcb)` | both arms carried |
| C 673 `#ifdef USE_GTK` (inside `x_defined_color`) | `use_gtk`, nested in the file gate | the GTK configuration |
| C 699 `#if false` … C 712 (inside `x_decode_color`) | no gate: C's own condition is the literal `false` | never - recorded, not transcribed |
| C 742 `#ifdef HAVE_XRENDER` … C 756 (C 752-753 the `#else`) | `feature = "have_xrender"` / `not (feature = "have_xrender")` | both arms carried |
| C 764 `#ifdef USE_GTK` … C 771 (inside `x_set_alpha_background`) | `all (feature = "have_x_windows", feature = "use_gtk")` | the GTK configuration |
| C 737-740 / 794-798 (the `HAVE_GTK3` declarations and the class walk) | `all (feature = "have_x_windows", feature = "use_gtk", feature = "have_gtk3")` | the GTK3 configuration |
| C 776 `#ifndef HAVE_GTK3` … C 801 (C 777-782 the `else` arm, C 783-800 the `#else` arm) | `not (feature = "have_gtk3")` / `feature = "have_gtk3"` | both arms carried |

`use_xcb`, `have_x_i18n`, `glyph_debug`, `use_gtk`, `have_gtk3`, `have_xrender`, `use_x_toolkit`,
`use_lucid`, `have_xdbe`, `have_xinput2` and `usg` are names
`femacs/xtask/Cargo.toml` already registers.  `use_motif`, `have_xaw3d`, `have_xrandr` and
`have_xinerama` are the C macros this slice's heads test that the manifest does **not** register
yet: they are used in the port's `#[cfg]`s as C's own names lower-cased and are owed to the
manifest, not added here (the task's rule; the license to add them is the integrator's).

`HAVE_WINDOW_SYSTEM` has no `#if` of its own in this file, but it is the *build* gate
(`configure.ac` 7521), which is why the port's file gate is `have_x_windows`: the `frame.h` items
this slice names carry their own gates in the mirror (`FRAME_TOOLTIP_P` and `FRAME_Z_GROUP*` are
`have_window_system`'s, `FRAME_X_P` and `FRAME_LIVE_P` are ungated), and a real X11 configuration
defines both features.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-20 | the blank and `#include <config.h>` | ungated (every unit includes it) |
| 21-24 | `<stdio.h>`, `<stdlib.h>`, `<math.h>`, `<unistd.h>` | file gate |
| 25-38 | the blank at 25 and `lisp.h`, `character.h`, `xterm.h`, `frame.h`, `window.h`, `buffer.h`, `dispextern.h`, `keyboard.h`, `blockinput.h`, `charset.h`, `coding.h`, `termhooks.h`, `font.h` | file gate |
| 39-42 | `#ifdef HAVE_X_I18N` / `#include "textconv.h"` / `#endif` | `have_x_i18n`, nested in the file gate |
| 43-45 | `<sys/types.h>`, `<sys/stat.h>` | file gate |
| 46-50 | `#ifdef USE_XCB` / `<xcb/xcb.h>` / `<xcb/xproto.h>` / `#endif` | `use_xcb` (both headers are the toolchain's: no mirror, named below) |
| 51-53 | `bitmaps/gray.xbm`, `xsettings.h` | file gate (`gray.xbm` is a data file with no Rust counterpart; the C line is recorded) |
| 54-60 | `#ifdef HAVE_XRANDR`'s `<X11/extensions/Xrandr.h>` and `#ifdef HAVE_XINERAMA`'s `<X11/extensions/Xinerama.h>` | `have_xrandr` / `have_xinerama` (unregistered; both headers have no mirror) |
| 61-64 | `#ifdef USE_GTK` / `#include "gtkutil.h"` / `#endif` | `use_gtk`, nested in the file gate |
| 65-72 | `#ifdef HAVE_XDBE`'s `<X11/extensions/Xdbe.h>` and `#ifdef HAVE_XINPUT2`'s `<X11/extensions/XInput2.h>` | `have_xdbe` / `have_xinput2` (no mirrors) |
| 73-122 | the `USE_X_TOOLKIT` block: `<X11/Shell.h>`, the Xaw3d/Xaw pair (HAVE_XAW3D), `<X11/Xos.h>` with the `USG` dance (C 87-95), `widget.h` (C 97), `../lwlib/lwlib.h` (C 99), the six `<Xm/…>` headers (USE_MOTIF, C 101-108), `../lwlib/xlwmenu.h` (USE_LUCID, C 110-112), `extern LWLIB_ID widget_id_tick;` (C 114-116) and C 118-120's empty `USE_MOTIF` block | `use_x_toolkit`, `have_xaw3d`/`use_motif` (unregistered), `use_lucid`, `usg`; the two Emacs-side headers take mirrors, the X11/Xaw/Xm ones have none |
| 123-126 | the empty `#ifdef USE_GTK` block | `use_gtk` |
| 127-128 | `#define MAXREQUEST(dpy) (XMaxRequestSize (dpy))` | file gate |
| 129-132 | `#ifdef GLYPH_DEBUG` / `static int dpyinfo_refcount;` / `#endif` | `glyph_debug`, nested in the file gate |
| 133-161 | the Motif hints: `MWM_HINTS_DECORATIONS`, C's four commented-out members, `MWM_DECOR_ALL`, the commented-out `_XA_MOTIF_WM_HINTS`, `PropMotifWmHints` (C 149-157) and `PROP_MOTIF_WM_HINTS_ELEMENTS`, all inside C 134-135's `#ifndef USE_MOTIF` / `#ifndef USE_GTK` | `all (have_x_windows, not (use_motif), not (use_gtk))` |
| 162-164 | the two `static` forward declarations (`x_display_info_for_name`, `set_up_x_back_buffer`) | dropped (PORTING.md 0.1-12; both definitions are in the remainder - C 3972 and C 7613) |
| 165-205 | `check_x_display_info` (C 166-169 its comment) | file gate |
| 206-215 | the comment of `x_real_pos_and_offsets` | file gate |
| 216-588 | `x_real_pos_and_offsets` (C 251-262 the `USE_XCB`/`#else` declarations, C 279-282 `x_catch_errors`, C 292-321 the query-tree pair, C 331-484 the geometry/translate pair, C 487-535 `_NET_FRAME_EXTENTS`, C 537-565 the outer geometry, C 567-569 `x_uncatch_errors`) | file gate; `use_xcb` for every `#ifdef USE_XCB` arm and its `#else` partner |
| 589-599 | the comment of `x_real_positions` and the function | file gate |
| 600-646 | the comment of `x_relative_mouse_position` and the function | file gate |
| 647-659 | the comment of `gamma_correct` and the function | file gate |
| 660-685 | the comment of `x_defined_color` and the function (C 673 `#ifdef USE_GTK`) | file gate; `use_gtk` for `xg_check_special_colors` |
| 686-718 | the comment of `x_decode_color` and the function (C 699-706 the `#if false` region) | file gate; the `#if false` region is dead text, recorded not transcribed |
| 719-730 | the comment of `x_set_wait_for_wm` and the function | file gate (the one statement's member is the mirror gap named below) |
| 731-802 | `x_set_alpha_background` (C 742-756 the `HAVE_XRENDER` pair, C 764-771 the `USE_GTK` block, C 794-798 the `HAVE_GTK3` class walk, C 776-801 C's `#ifndef HAVE_GTK3` pair) | file gate; `have_xrender`, `use_gtk`, `have_gtk3` |
| 803-10662 | **the remainder** (C 10662 is the reference's last line): the file's other 9,860 lines, whose first live definition is `x_set_tool_bar_position` (C 805) - the next slice's first item | not ported in this slice |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/xfns1-lane/cargo cargo run -q -p xtask -- build-tu src/xfns.c`
→ `built src/xfns.rs -> ../build/femacs/xfns.o`, **exit 0**, and `build/femacs/xfns.o` is
**336 bytes** - an empty object.  That is the expected reading, not a proof of the new text: every
item of the port sits behind a `have_x_windows` gate, so under the frozen feature set rustc
compiles only `include/config.rs` and the `#![allow]` attribute (P-86 ⑧'s measurement: an include
edge outside the file gate fails the frozen build at once).

**② The X11 arm (P-86 ③⑴; `xtask probe`, whose materialization is a unit's).**

```
$ cd femacs && cargo run -q -p xtask -- probe src/xfns.rs /tmp/xfns1-lane/xfns_probe1.o \
      --crate-type lib --emit obj -C panic=abort --cfg 'feature="have_x_windows"'
23 errors; 2 of them in the unit's own text, the other 21 in the mirrors (18 + 3).
```

The two in-unit errors are the two classes this ledger records: `x_display_info_for_name` not
found (C 7613 defines it; the call is C 198, this slice's) and `no field wait_for_wm on type
x_output` (the `bool_bf` member `include/xterm.rs` records as a comment, P-80).  The
mirror-side errors are `Time` (18: `include/xterm.rs`'s un-mirrored X11 names, its own ledger's
list) plus the mirror INLINE functions' raw-pointer dereferences (`E0133`,
`include/xterm.rs` 1440/1511-1518).

**③ The XCB arm and the rest of the plain-X11 family.**

```
$ cd femacs && cargo run -q -p xtask -- probe src/xfns.rs /tmp/xfns1-lane/xfns_probe2.o \
      --crate-type lib --emit obj -C panic=abort --cfg 'feature="have_x_windows"' \
      --cfg 'feature="use_xcb"' --cfg 'feature="have_x_i18n"' --cfg 'feature="glyph_debug"' \
      --cfg 'feature="have_xrender"' --cfg 'feature="have_xdbe"' --cfg 'feature="have_xinput2"'
# with the slice as first written: 23 errors in the unit's own text
#   18 E0308 + 3 "arguments to this function are incorrect"  (the Window <-> xcb_window_t /
#     xcb_atom_t seam, which C converts implicitly and the port must spell)
#    5 E0609  (`xrender_supported_p`, `xrender_major` x2, `xrender_minor` x2)
# after adding those conversions: 3 in the unit's text
#   3 = the two classes of Evidence ② plus one REAL DEFECT this probe caught:
#     E0381 `outer_trans_cookie` possibly-uninitialized (C leaves it unassigned when
#     `FRAME_X_WINDOW (f) == FRAME_OUTER_WINDOW (f)`; the port now zero-initialises it,
#     PORTING.md 1.5)
# mirror-side on that run: 50 errors, all "SDK name with no mirror" (`Time` 11, `XIMStyle` 7,
#   `XIMStatusNothing` 4, `XIMPreeditNothing` 2, `XRenderPictFormat`, `XIM`, `XIMStyles`,
#   `xim_inst_t`, `Picture`, `XIC`, `XFontSet`, `XIEventMask`, the four `XIM*` constants) plus
#   the same 10 E0133 and the five `xrender_*` field reports in `include/xterm.rs`.
```

The `E0381` was fixed *after* that run; the run that re-measured with the fix (Evidence ④) does
not enable `use_xcb`, so the XCB config's final count is **2 in-unit errors** by construction of
the two runs, not by a re-run in that exact configuration.

**④ The GTK arm.**

```
$ cd femacs && cargo run -q -p xtask -- probe src/xfns.rs /tmp/xfns1-lane/xfns_probe3.o \
      --crate-type lib --emit obj -C panic=abort --cfg 'feature="have_x_windows"' \
      --cfg 'feature="use_gtk"' --cfg 'feature="have_gtk3"' --cfg 'feature="have_xrender"' \
      --cfg 'feature="have_x_i18n"'
2 errors in the unit's own text - the two classes of Evidence ② and nothing else; the
`HAVE_GTK3` arm's in-unit `GtkWidget`, `G_OBJECT_GET_CLASS!`, `GTK_WIDGET_CLASS!`,
`GtkWidgetClass`, `gtk_widget_set_app_paintable` and `xg_check_special_colors` all resolve.
mirror-side: 67 errors, all "SDK name with no mirror" (`Time` 9, `gtk_widget_get_window` 8,
`XIMStyle` 7, `XIMStatusNothing` 4, `GDK_WINDOW_XID` 4, `GtkCssProvider` 2, …) plus 10 E0133.
```

**⑤ The final revision, every arm at once** (the run the other readings' revision was built on).

```
$ cd femacs && cargo run -q -p xtask -- probe src/xfns.rs /tmp/xfns1-lane/xfns_probe4.o \
      --crate-type lib --emit obj -C panic=abort --cfg 'feature="have_x_windows"' \
      --cfg 'feature="use_xcb"' --cfg 'feature="use_gtk"' --cfg 'feature="have_gtk3"' \
      --cfg 'feature="have_xrender"' --cfg 'feature="have_x_i18n"' \
      --cfg 'feature="glyph_debug"' --cfg 'feature="have_xdbe"' --cfg 'feature="have_xinput2"'
2 errors in the unit's own text - the two classes of Evidence ②, nothing else.
74 errors in the mirrors, all "SDK name with no mirror" (`Time` 11, `gtk_widget_get_window` 10,
`XIMStyle` 7, `GDK_WINDOW_XID` 5, `XIMStatusNothing` 4, …) plus 10 E0133.
```

A comment-only pass (four C/SDK line citations corrected against the reference and the installed
headers) followed Evidences ②-④ and precedes this run: ⑤ is the port's final revision, and ②-④
describe the revision exactly one comment-only pass earlier.

**⑥ The three self-checks of the task, run on this revision.**

```
$ python3 tools/extract-abi.py --check emacs/src/xfns.c femacs/src/xfns.rs
- missing defsubr (&raw const Sx_backspace_delete_keys_p)          ... 45 rows
- missing function XDestroySubwindows                              ... 105 rows
- missing function XScreenNumberOfScreen
- missing macro INSTALL_CURSOR / RANDR13_LIBRARY / USG             ... 3 rows
- missing constant mouse_cursor_bottom_edge / … / mouse_cursor_vertical_drag   ... 16 rows
- missing file-scope object xic_default_fontset / tip_timer / tip_last_string /
  tip_last_frame / tip_last_parms                                  ... 5 rows
- function check_x_display_info does not mention 0
- function x_real_pos_and_offsets does not mention uninit
- line count drifted 89.5%: C 10662 vs Rust 1120
FAIL emacs/src/xfns.c -> femacs/src/xfns.rs
exit 1

$ python3 tools/check-dead-file-coverage.py
info docs/ports/xfns.md: 28 range row(s) cover src/xfns.c (10662 lines; 0 blank line(s)
  between blocks need no row)
18 ledger(s) bound (a C range table), 0 failure(s)
exit 0

$ python3 tools/check-declarations.py
FAIL femacs/src/android.rs: literal x1: macro_rules! PACKAGE_STRING { () => { "<the reference
  program's own version string>" }; }        # the literal is not reproduced here: quoting it
                                             # would be this ledger's own `title` finding
14 declaration sites (0 forbidden, 14 allowed literals), 1 failures
exit 1
```

`--check` FAILs on 227 `- missing …` rows that are the rest of the file (P-86 ⑩: the moment a
port has a `fn`, the checklist stops being vacuous), plus **two rows that are this slice's own** -
`check_x_display_info does not mention 0` (C's `x_display_list != 0` is the port's
`!…is_null ()`, the class `docs/ports/macro-body-notes.tsv` 112/265 already carries rows for) and
`x_real_pos_and_offsets does not mention uninit` (C's `Window wm_window UNINIT, rootw UNINIT;` is
the port's zero-initialiser, PORTING.md 1.5 - the class that file's rows 130/131/138/291/314/350/369
carry) - plus the line-count drift row every slice of a large file has.  All three are rows in
`docs/ports/macro-body-notes.tsv` and `docs/ports/line-drift-tolerance.tsv`, files this lane may
not write; "Owed" names them.  `check-dead-file-coverage.py` is 0 failures (the table above is its
reading: 28 rows, 0 blank lines needing a row).  `check-declarations.py`'s one failure is
`femacs/src/android.rs`'s `PACKAGE_STRING` literal (another lane's file, untouched here - and
quoted only by name, since reproducing the literal would make this ledger carry the class itself);
this lane's two files produce no finding.

**⑦ Disk and budget (over budget by two runs, reported).**  The tree's `CARGO_TARGET_DIR` was
`/tmp/xfns1-lane/cargo`, every probe object went to `/tmp/xfns1-lane/`, and `build-tu`/`probe`
always emit a single object (`--emit obj`), never an rlib in the working directory.  `build-tu`
was run **4** times against the budget of 3: run 1 failed on an unterminated block comment (the
port's own comment quoted C's `Xm/*.h` path, which Rust's nestable block comments read as an
opening `/*` - PORTING.md 0.3's trap), run 2 compiled the revision the first fix batch produced,
run 3 is the revision Evidences ②-④ measure, and run 4 is the revision Evidence ⑤ measures and the
one Evidence ① quotes.  `probe` was run **4** times against the
budget of 3: Evidences ②, ③, ④ and ⑤ (⑤ exists because the comment-only pass of Evidence ⑤'s last
paragraph came after ④).  Both objects are single `--emit obj` outputs of a few hundred bytes.

## Deviations

- **The licence block is not carried** (C 1-18, P-86 ① / PORTING.md 0.1-19): the port opens with
  `// femacs: port of src/xfns.c @ fd7afced403.`
- **C's forward declarations are dropped** (C 163-164, PORTING.md 0.1-12): Rust has no forward
  declaration, and both definitions sit in the remainder.  `x_display_info_for_name` is *called*
  by this slice (C 198), so that call is kept and its definition is owed to the remainder slice.
- **`UNINIT` is zero-initialisation** (PORTING.md 1.5, `include/conf_post.rs` 297-302's record):
  `x_real_pos_and_offsets`'s `Window wm_window UNINIT, rootw UNINIT;` and the XCB cookies C
  declares without an initialiser are `let mut x = 0;` / `std::mem::zeroed ()`.  Evidence ③'s
  `E0381` is what that rule is for: `outer_trans_cookie` is written only when
  `FRAME_X_WINDOW (f) != FRAME_OUTER_WINDOW (f)` and read only in the arm that tests so, and
  Rust's flow analysis will not accept C's invariant.
- **The `#if false` region of `x_decode_color` is recorded, not transcribed** (C 699-706,
  PORTING.md P-08 / docs/status.md 3.55, the `character.rs` / `cm.rs` `#if 0` precedent).  The
  region is C's own dead text about the default colormap; C's condition is the literal `false`, so
  there is no macro to lower-case into a feature.
- **The `bool_bf` member is kept by name, not by P-80's accessor** (`x_set_wait_for_wm`, C 729).
  P-80's shape needs the mirror to carry the bitfield's storage unit; `include/xterm.rs` records
  the six-member group (`hourglass_p`, `explicit_parent`, `asked_for_visible`, `has_been_visible`,
  `wait_for_wm`, `alpha_identical_p`) as comments and carries no storage, so the port keeps C's own
  member name in C's own statement (the `xterm.rs` `buffered_input_event.sie` precedent - a member
  the mirror does not carry yet) and the result is Evidence ②'s `E0609`.  The member's measured
  position is in the tree: `docs/baseline/x_output-bits.tsv` (`x11 wait_for_wm 0 1 4 1`: unit 1,
  bit 4, width 1) and `docs/baseline/x_output-layout.tsv` (byte 368);
  `docs/ports/include-xterm.md` 165 is where the mirror's row is owed.
- **`Emacs_Color` is the X11 arm's alias in C, not in the mirror** (`x_defined_color`, C 668-669;
  `x_decode_color`, C 695).  dispextern.h 38's x11 arm is `typedef XColor Emacs_Color;`, but
  `include/dispextern.rs` carries the `#else` arm's record unconditionally, so the port keeps C's
  `XColor` prototype for `x_parse_color` / `x_alloc_nearest_color` and converts at the two call
  sites (`color as *mut XColor`, `&raw mut cdef as *mut Emacs_Color`) - the P-72 rule 3
  "consumer converts at the call site" pattern the `include/termhooks.rs` union note prescribes.
  The owed mirror rows are in "Owed" below.
- **The Window ↔ `xcb_window_t` / Atom ↔ `xcb_atom_t` seam is spelled out** (`use_xcb` arms of
  C 331-484, 487-535, 537-565).  C converts implicitly (`Window` is `unsigned long`,
  `xcb_window_t` is `uint32_t`); the port writes `as xcb_window_t` / `as xcb_atom_t` /
  `as Window` at each site, exactly the shape `femacs/src/xterm.rs` 3004-3021 uses for the same
  seam.  The narrow reply fields (`int16_t`/`uint16_t`) take `as c_int` / `as c_uint`, C's own
  implicit widening.
- **`ow - fw + outer_x` keeps C's unsigned arithmetic** (C 586-587): `ow.wrapping_sub (fw)
  .wrapping_add (outer_x as c_uint)` then `as c_int` - C's `unsigned int` arithmetic wraps on
  underflow and the assignment narrows (PORTING.md's `wrapping_*` rule, `charset.rs` 242's shape).
- **`GTK_WIDGET_CLASS` / `G_OBJECT_GET_CLASS` keep their names and result types but not their
  macro chains** (`HAVE_GTK3` arm, C 794-798).  C's bodies are the GLib checked-cast chains
  (`G_TYPE_CHECK_CLASS_CAST` / `G_TYPE_INSTANCE_GET_CLASS` / `g_type_check_instance_cast` /
  `G_TYPE_OBJECT`, `G_TYPE_WIDGET`), which the port cannot spell without a GTK tree: this machine
  has none (`/opt/homebrew/include` has no `gtk`/`glib` tree), so P-86 ③⑵'s second tier applies -
  the arm is `text-only`, the two records (`GObjectClass`, `GtkWidgetClass`) carry only the members
  C reads, and the conversions the chains end in are what the port's bodies are.  The bodies'
  `/// C:` lines quote the SDK macros verbatim; the owed rows are in "Owed".
- **`gtk_widget_set_app_paintable`'s `bool` argument is converted** (C 768-770): C widens
  `f->alpha_background != 1.0` to GObject's `gboolean` (`int`), the port writes `(… != 1.0) as
  c_int`.
- **`x_set_alpha_background`'s `else` arm carries both `#ifndef HAVE_GTK3` arms** (C 776-801) as
  two `#[cfg]` statements inside C's `else` block, C's statement order kept.

## Owed

- **`include/xterm.rs`**: `struct x_output`'s `bool_bf wait_for_wm : 1` (P-80's storage unit at
  `docs/baseline/x_output-bits.tsv`'s measured `unit 1, shift 4`) so `x_set_wait_for_wm` (C 729)
  can write the member; the `xrender_supported_p` / `xrender_major` / `xrender_minor` members
  `FRAME_CHECK_XR_VERSION` itself reads (the mirror's own macro is `E0609` under
  `have_xrender`); the bare X11 names this slice's flip arms reach through the mirror's text
  (`Time`, `XIM*`, `XIC`, `XFontSet`, `XRenderPictFormat`, `Picture`, `XIEventMask`, the `XIM*`
  style constants).
- **`include/dispextern.rs`**: `#[cfg (feature = "have_x_windows")] pub type Emacs_Color = XColor;`
  (dispextern.h 38) and the `#else` struct's gate, so the two conversions of the deviation above
  disappear.
- **`include/xlib.rs`**: the Xlib entry points this slice declares at its use site -
  `XFree` (2526), `XGetGeometry` (2630), `XGetWindowProperty` (2686), `XChangeProperty` (2078),
  `XMaxRequestSize` (1709), `XQueryTree` (3027), `XTranslateCoordinates` (3456) - and the XCB
  query-tree pair (`xcb_query_tree`, `xcb_query_tree_reply`, `xcb_query_tree_cookie_t`,
  `xcb_query_tree_reply_t`) whose family the mirror's xcb section already carries for four other
  requests.  (`xlib.rs` declares eighteen Xlib entry points today - the Xrm family, the screen/resource
  accessors and the seven `image.c` reads; these seven are the ones an X11 unit needs first.)
- **`include/gtkutil.rs` / the GTK arm**: `GtkWidget` and the four `HAVE_GTK3` names (they are
  declared in `xfns.rs` today, P-88 ①) belong in the dead-by-construction GTK mirror once a GTK
  target is frozen, together with `GtkWidgetClass`'s full layout (P-80 requires a real
  compiler's byte dump, and this machine has no GTK tree).
- **`src/xfns.c` itself**: `x_display_info_for_name` (C 7613) and `set_up_x_back_buffer` (C 3972)
  - the two dropped forward declarations' definitions, both in the remainder.
- **`docs/ports/macro-body-notes.tsv`**: two rows for this slice's own text, in the format of
  that file's existing `FRAME_LIVE_P`/`chars_in_text` (`0`) and `UNINIT` (`uninit`) rows -
  `check_x_display_info` and `x_real_pos_and_offsets`; and a row for the `USG` macro row
  `extract-abi.py --check` prints (C 90's `#define USG` is a marker for a header no mirror
  carries, and the port's `#ifdef USG` arm is a comment).  **`docs/ports/line-drift-tolerance.tsv`**:
  the `xfns.c` row for the line-count drift, which every slice of a 10,662-line file owes until
  the last one lands.
- **`femacs/xtask/Cargo.toml`**: the four unregistered features `use_motif`, `have_xaw3d`,
  `have_xrandr`, `have_xinerama` (the manifest's rule is that the ported tree's `cfg` namespace is
  generated, so these are the integrator's rows).
- **`docs/port-map.md`**: the `src/xfns.c` row still reads 11 port lines (the map is generated by
  `tools/gen-port-map.py`; regenerating it is the integrator's step, P-86 ④).  The map's *reference*
  count (10,662) is what `check-dead-file-coverage.py` reads and is unaffected.
- **The X11 extension and toolkit headers** this slice's `#include`s name and no mirror carries:
  `<xcb/xcb.h>`/`<xcb/xproto.h>` (the mirror is `include/xlib.rs`'s xcb section),
  `<X11/extensions/Xrandr.h>`, `<X11/extensions/Xinerama.h>`, `<X11/extensions/Xdbe.h>`,
  `<X11/extensions/XInput2.h>`, `<X11/Shell.h>`, `<X11/Xaw3d/{Paned,Label}.h>`,
  `<X11/Xaw/{Paned,Label}.h>`, `<X11/Xos.h>`, the six `<Xm/…>` headers; and
  `bitmaps/gray.xbm`, a data file with no Rust counterpart.

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte empty object), so this is a *text* port with *compile* readings of its
  arms and no behaviour: nothing here has been executed, and no X11 display, X server, XCB
  connection or GTK toolkit was involved.
- **The three flip arms are compile readings, not acceptance.**  Evidence ②-④ are `rustc` exit
  codes and error classes under `--cfg` sets this machine can name but not satisfy: the X11 and
  XCB headers exist here (`/opt/homebrew/include`: libx11 1.8.13, xorgproto 2025.1), but the GTK
  arm has no toolkit tree at all, and the tree's **mirrors are not yet flip-clean** - 21, 50 and 67
  errors respectively are in the mirrors this slice includes (all recorded classes: un-mirrored
  SDK names and the mirror INLINEs' `E0133`), so no arm's reading is "0 errors" and none of them
  is claimed as a working configuration.
- **This slice's text is not proven beyond its two recorded error classes.**  Evidence ③'s
  post-fix reading (2 in-unit errors in the GTK-time configuration, both recorded) is a
  *compile* reading; the XCB configuration's own final count is stated as a consequence of two
  runs, not a re-measurement (Evidence ③).
- **The `#if false` region, C's forward declarations and the commented-out Motif members are
  dropped or recorded**, per the Deviations above - not "ported".
- **`extract-abi.py --check` is not coverage evidence here.**  P-86 ⑩: the moment a port has a
  `fn`, `--check` stops being vacuous and reports the checklist's `- ` rows; those rows (and the
  `macro-body-notes.tsv` / `line-drift-tolerance.tsv` rows a `goto`- or line-count-based check
  would want) live in files this lane may not write, so ⑥ records the reading rather than fixing
  it.
- The `## Evidence` numbers are from the revisions named there; the tree's `femacs/src/xfns.rs`
  may move under a later slice, and `docs/port-map.md`'s row is stale by design.

## P-90's fourth step (2026-10-05, status 3.484): the GTK names move to the mirror

The in-unit segment the Evidence section describes (the `GtkWidget`, `GtkWidgetClass` and
`gtk_widget_set_app_paintable` declarations this unit carried because "no mirror carried them")
is gone: C 63's chain (`"gtkutil.h"` 26's `<gtk/gtk.h>`) now carries `include!("include/gtk.rs")`
- the SDK surface's one mirror (P-90 ②) - and the three declarations were deleted in the same
round.  What stays here is what the mirror does not carry: `GObjectClass` (gtk.rs has no such
record), the two GLib cast macros (`G_OBJECT_GET_CLASS`/`GTK_WIDGET_CLASS`), and
`xg_check_special_colors` (gtkutil.h's own prototype).

Measured (my own): the unit's arm (`have_x_windows, use_gtk, have_gtk3`) went from **40 errors to
25** - 29 -> 14 `E0425`, the `E0133`/`E0609` classes unchanged and **no `E0428`** - because the
mirror supplies fifteen names the in-unit segment could not resolve to the rest of the graph;
`extract-abi --check` leaves 226 `- missing` rows and the drift (the unported remainder) and no
`does not mention` row; `layout-check` passed; `tools/check-all-units.sh` 83/0 + 83/0.
What is **not** done: the two *mirror-internal* edges (`include/gtkutil.rs` C 26 and
`include/pgtkterm.rs` C 31) still stand as comments in those two files, and landing them means
deleting the direct `include/gtk.rs` edges the consumers spell themselves (`src/gtkutil.rs` 174,
`src/pgtkterm.rs` 81, `src/image.rs` 166, `src/xwidget.rs`, `src/pgtkfns.rs`) in the same round -
otherwise gtk.rs is textually included twice in one translation unit.  That cascade is the next
mirror round's, not this unit's.
