# emacs/src/xmenu.c (2932 lines)

> **One of P-86's class: the frozen configuration compiles nothing here.**  `xmenu.o` sits in
> `XMENU_OBJ` (`emacs/src/Makefile.in` 211-212: "## xmenu.o if HAVE_X_WINDOWS, else empty."), which
> `configure.ac` 7189-7195 fills with `xmenu.o` only under `test "${HAVE_X_WINDOWS}" = "yes"` (the
> same line `AC_DEFINE`s `HAVE_X_WINDOWS`, 7193-7194); `docs/baseline/frozen-build-sources.txt`
> (106 sources, this target's `make -n`) lists no `xmenu.o`, so `extract-abi.py`'s checklist for the
> file is empty.  P-86 ②'s substitute is below: the **C line-range table**, the `## Evidence`
> readings and the `## Deviations`.  Port: `src/xmenu.rs` (**4011 lines**, md5
> `0223d2a56b595aae660b48de519bb06b`), carrying **C 1-2932 with both arms of every `#if`**.
> The reference `emacs/src/xmenu.c` is md5 `47200d90bc2d0e561db090f8d6d07ace` (unchanged; `emacs/`
> is read-only).
>
> **This ledger replaces the P-08 reading it carried.**  The previous `docs/ports/xmenu.md` (24
> lines) read "the frozen configuration does not compile this file ⇒ the port is the empty unit
> plus the note that names the gate" and listed 13 deferred declarations.  P-86 ① replaces that
> conclusion with *transcribe the text, both arms, no stub*; what the earlier note measured (not in
> the frozen source list, empty checklist, 336-byte `build-tu` object) is unchanged and still true -
> it is the *conclusion* that is withdrawn, exactly as `docs/ports/w32menu.md`,
> `docs/ports/xselect.md` and `docs/ports/androidmenu.md` record for their own pairs.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`XMENU_OBJ`, `configure.ac` 7189-7195: `HAVE_X_WINDOWS`) | `feature = "have_x_windows"` on every item *and* on the include graph | an X window-system configuration |
| C 50/145/306/2525/2768/2881 `#ifndef MSDOS` | `not (feature = "msdos")` | any X build (the `#else` arms are empty) |
| C 54/113/230/326/377/477/492/522/667/1620/1685/1723/1754/1801/1847/1858/1873/2519/2542/2778/2790 | `feature = "have_xinput2"` (and `have_xinput2_4` for C 1685) | XInput2 configured in |
| C 75/120/158/311/326/353/377/477/492/534/667/931/947/1197/1222/1233/1255/1281/1286/1302/1316/1332/1354/1393/1600/1801/1825/1847/1858/1873/2167/2238/2911/2915/2927 | `feature = "use_x_toolkit"` | the Xt/Lucid or Motif build (`USE_X_TOOLKIT`) |
| C 95/101/103/147/189/219/230/296/578/654/663/699/713/747/782/830/855/1153/1179/1250/1367/1393/1600/2167/2211/2267/2911/2915 | `feature = "use_gtk"` | the GTK build |
| C 82/90/504/1197/1222/1233/1745/1761/1850/2238 | `feature = "use_lucid"` | the Lucid (Xaw) build |
| C 91/147/353/415/534/1286/1302/1316/1332/1354/1723/1754/1825 | `feature = "use_motif"` | the Motif build |
| C 103/219/1421/1429/1460/1510/1533/1556 | `feature = "have_gtk3"` | GTK 3 |
| C 85-89 | `feature = "have_xaw3d"` (both `Paned.h` arms carried) | Xaw3d present |
| C 74 `#undef HAVE_MULTILINGUAL_MENU` / C 1961/2011/2073 `#ifndef` | `not (feature = "have_multilingual_menu")` is the live arm (the file `#undef`s it itself) | every configuration |
| C 2820 `#ifdef XDEBUG` | `feature = "xdebug"` | never (no configuration this tree builds) |

`tools/check-cond-eval.py src/xmenu.c` reads **`95 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (Evidence ⑬): `src/xmenu.c` is on `docs/baseline/config/cond-eval-verified.txt` 419,
so the frozen macro table decides every arm and the port's five `--cfg` sets map to five rows of the
table above.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-23 | the reference's opening block (title line, authors, licence text) | dropped (PORTING.md 0.1-19) |
| 24-32 | the file's prose: its title comment (dropped with the block above) and the two history notes | kept as prose |
| 33-34 | `#include <config.h>` | ungated |
| 35-36 | `#include <stdio.h>` | file gate |
| 37-48 | `lisp.h` … `pdumper.h` | file gate |
| 49-53 | `#ifdef MSDOS` / `msdos.h` / `#endif` | both arms; `msdos` |
| 54-57 | `#ifdef HAVE_XINPUT2` / `<math.h>` / `<X11/extensions/XInput2.h>` / `#endif` | both arms; `have_xinput2` |
| 58-63 | `#ifdef HAVE_X_WINDOWS` / the sys/types.h note / `xterm.h` / `#endif` | both arms |
| 64-69 | the sys/types.h note and `#ifndef makedev` / `<sys/types.h>` / `#endif` | both arms; `not (makedev)` |
| 70-99 | `#ifdef HAVE_X_WINDOWS`, `#undef HAVE_MULTILINGUAL_MENU`, the Xt chain (`widget.h`, X11 headers, Lucid's `xsettings.h`/`xlwmenu.h` + the `HAVE_XAW3D` pair, Motif's `lwlib.h`) and the `oldXMenu/XMenu.h` arm | both arms of every condition |
| 100-108 | `#ifdef USE_GTK` (`gtkutil.h`, `HAVE_GTK3`'s `xgselect.h`) and `menu.h` | both arms |
| 109-118 | `popup_activated_flag` (`#ifndef HAVE_XINPUT2` / `#else`) | both arms |
| 119-143 | `#ifdef USE_X_TOOLKIT`: `next_menubar_widget_id`, `menubar_id_to_frame` | both arms |
| 144-174 | `#ifndef MSDOS`, `#if defined USE_GTK \|\| defined USE_MOTIF`: `x_menu_set_in_use` (with C 158-161's `USE_X_TOOLKIT` arm) | both arms each |
| 175-228 | `x_menu_wait_for_event`, incl. C 187-193's three condition texts | all three arms |
| 229-294 | `x_menu_translate_generic_event` | `!use_gtk && !use_x_toolkit && have_xinput2` |
| 295-306 | `x_menu_dispatch_event` and `#endif /* ! MSDOS */` | `!use_x_toolkit && !use_gtk` |
| 307-319 | the `USE_X_TOOLKIT \|\| USE_GTK` block opens; `popup_get_selection`'s banner | file gate |
| 320-462 | `popup_get_selection` | `use_x_toolkit` |
| 463-575 | the Xt `DEFUN ("x-menu-bar-open-internal")` | `use_x_toolkit` |
| 576-630 | the GTK `DEFUN ("x-menu-bar-open-internal")` and `popup_widget_loop` | `use_gtk` |
| 631-698 | `x_activate_menubar` (C 654-658's `USE_GTK` arm, C 667-684's `have_xinput2` arm) | both arms each |
| 699-722 | `popup_activate_callback`, `popup_deactivate_callback` (C 713-717's parameter arms) | both arms |
| 723-741 | `show_help_event` | `use_x_toolkit \|\| use_gtk` |
| 742-780 | `menu_highlight_callback`, GTK and Xt arms | both arms |
| 781-847 | `xg_crazy_callback_abort`, `menubar_selection_callback`, GTK and Xt arms | both arms |
| 848-923 | `update_frame_menubar` (C 855-857) and the Lucid `apply_systemfont_*` pair | both arms each |
| 924-1250 | `set_frame_menubar` (C 945-951, C 1150-1177, C 1179-1248 with C 1197/C 1222/C 1233) | all arms |
| 1251-1276 | `initialize_frame_menubar` | `use_x_toolkit \|\| use_gtk` |
| 1277-1367 | `free_frame_menubar` (`#ifndef USE_GTK`, with its Motif arms) | `use_x_toolkit && !use_gtk` |
| 1368-1391 | the `USE_X_TOOLKIT \|\| USE_GTK` block reopens; `menu_item_selection` | file gate |
| 1392-1471 | `struct next_popup_x_y`, `menu_position_func` (C 1421-1423/1429-1431/1460-1463) | `use_gtk` (+ `have_gtk3` arms) |
| 1472-1498 | the GTK `popup_selection_callback` and `pop_down_menu` | `use_gtk` |
| 1499-1598 | the GTK `create_and_show_popup_menu` (C 1509-1514, C 1533-1538, C 1556-1572) | `use_gtk` (+ `have_gtk3`/`have_xinput2` arms) |
| 1599-1618 | `#else /* not USE_GTK */`: `widget_id_tick`, the Xt `popup_selection_callback` | `use_x_toolkit && !use_gtk` |
| 1619-1710 | `#ifdef HAVE_XINPUT2`: `prepare_for_entry_into_toolkit_menu`, `leave_toolkit_menu` (C 1685-1696) | `use_x_toolkit && !use_gtk && have_xinput2` (+ `have_xinput2_4`) |
| 1711-1738 | the Xt `pop_down_menu` and `server_timestamp_predicate` | `use_x_toolkit && !use_gtk` (+ `use_motif`) |
| 1739-1883 | the Xt `create_and_show_popup_menu` (C 1761-1763, C 1801-1845 with C 1825-1844, C 1847-1861) | `use_x_toolkit && !use_gtk` (+ `use_lucid`/`use_motif`/`have_xinput2`) |
| 1884-1890 | `cleanup_widget_value_tree` | `use_x_toolkit \|\| use_gtk` |
| 1891-2166 | the toolkit/GTK `x_menu_show` (C 1961-1967, C 2011-2023, C 2072-2076) | `use_x_toolkit \|\| use_gtk` (+ `not (have_multilingual_menu)`) |
| 2167-2266 | the dialog callbacks and `create_and_show_dialog`, GTK and Xt arms (C 2238-2240) | both arms (+ `use_lucid`) |
| 2267-2271 | `button_names` | `use_x_toolkit \|\| use_gtk` |
| 2272-2432 | `x_dialog_show` | `use_x_toolkit \|\| use_gtk` |
| 2433-2467 | `xw_popup_dialog` | `use_x_toolkit \|\| use_gtk` |
| 2468-2508 | `#else /* not USE_X_TOOLKIT && not USE_GTK */`: `menu_help_frame`, `menu_help_callback` | `!use_x_toolkit && !use_gtk` |
| 2509-2559 | `struct pop_down_menu` and the oldXMenu `pop_down_menu` (C 2525-2531, C 2534-2555) | `!use_x_toolkit && !use_gtk` (+ `not (msdos)`, `have_xinput2`) |
| 2560-2877 | the oldXMenu `x_menu_show` (C 2610-2618, C 2736-2752, C 2768-2781, C 2790-2804, C 2806-2811, C 2818-2872) | `!use_x_toolkit && !use_gtk` (+ `have_xinput2`, `xdebug`) |
| 2878-2890 | `#endif` and `popup_activated` (`#ifndef MSDOS`) | both arms; `not (msdos)` |
| 2891-2903 | `DEFUN ("menu-or-popup-active-p")` | file gate |
| 2904-2922 | `syms_of_xmenu` (C 2911-2913, C 2915-2919) | file gate (+ `use_gtk`, `use_x_toolkit`) |
| 2923-2932 | `syms_of_xmenu_for_pdumper` (C 2927-2931) | file gate (+ `use_x_toolkit`) |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/xmenu-lane/cargo cargo run -q -p xtask -- build-tu src/xmenu.c`
→ `built src/xmenu.rs -> ../build/femacs/xmenu.o`, **exit 0**, and `build/femacs/xmenu.o` is
**336 bytes** (md5 `dbed68837694579185377dbb0821ab62`) - an empty object, which is the expected
reading and not a proof of the new text: every item of the port sits behind a `have_x_windows` gate,
so under the frozen feature set rustc compiles `include/config.rs` and the file's two attribute
blocks and nothing else.

**② The Xlib/Xproto/XI2 surface is *measured*, not remembered.**  `/opt/homebrew/include` has
`libx11` 1.8.13 and `xorgproto` 2025.1: `/tmp/xmenu-lane/probe1.c` includes `<X11/Xlib.h>`,
`<X11/X.h>`, `<X11/Xatom.h>`, `<X11/keysym.h>`, `<X11/extensions/XI2.h>` and prints the records'
`sizeof`/`offsetof` and the constants; `clang -I/opt/homebrew/include -o probe1 probe1.c && ./probe1`
exits 0 and reads: `XEvent` 192, `XButtonEvent` 96 (`serial`@8, `display`@24, `window`@32,
`subwindow`@48, `time`@56, `x`@64, `state`@80, `button`@84, `same_screen`@88), `XKeyEvent` 96
(`keycode`@84), `XPropertyEvent` 64 (`atom`@40, `time`@48, `state`@56), `XExposeEvent` 64
(`x`@40), `XAnyEvent` 40 (`display`@24, `window`@32), `XGenericEvent` 40 (`extension`@32,
`evtype`@36), `XGenericEventCookie` 56 (`cookie`@40, `data`@48); `ButtonPress` 4, `ButtonRelease` 5,
`KeyPress` 2, `PropertyNotify` 28, `GenericEvent` 35, `ControlMask` 4, `Button1` 1, `Button1Mask`
256, `ButtonReleaseMask` 8, `CurrentTime` 0, `XA_ATOM` 4, `PropModeReplace` 0, `XK_g` 0x67,
`XK_Escape` 0xff1b, `True` 1, `False` 0, `XI_ButtonPress` 4, `XI_ButtonRelease` 5, `XI_KeyPress` 2,
`XI_Motion` 6, `XI_Enter` 7, `XI_Leave` 8, `XI_HierarchyChanged` 11, `XI_DeviceChanged` 1,
`XI_LASTEVENT` 32, `XIAllMasterDevices` 1, the three pinch events 27/28/29.  The port's records carry
`const _: () = assert! (...)` layout assertions from those readings, and each constant and prototype
carries the header line it came from (Xlib.h 87/97-98/139-259/567/585/826/933/943/955/1005/1600/...
Xresource.h 204, XI2.h 187-223).  The names **not** on this machine - libXt (`X11/Intrinsic.h`,
`Shell.h`, `StringDefs.h`, `CoreP.h`), Xaw (`X11/Xaw/Paned.h`), Xaw3d, Motif, GTK3 and libXi
(`X11/extensions/XInput2.h`) - are declared in the unit at the use site with the C header's own
contract and are labelled `UNVERIFIED` there; the ledger's `## Owed` names each.

**③ The arms are flipped (P-86 ③⑴), and the flip is *not* vacuous (P-86 ⑦).**
`xtask probe src/xmenu.rs /tmp/o.o --crate-type lib --emit obj -C panic=abort` with the arm's
`--cfg`s (all five sets spelled out in full; the include graph follows the unit's own `include!`s):

| `--cfg` set | reading (exit 1 in every arm) | errors in the unit's own text | errors in the include graph |
| --- | --- | --- | --- |
| `feature="have_x_windows"` (the oldXMenu arm) | after the fixes below: **0** | `include/xterm.rs` 6, `oldXMenu/include/XMenu.rs` 4, `include/systime.rs` 1 |
| `feature="have_x_windows"`, `use_x_toolkit"`, `use_lucid"` | **2**: `E0070 invalid left-hand side of assignment` at C 1244/1298's `FRAME_MENUBAR_HEIGHT (f) = …` | `include/xterm.rs` 8, `include/xlwmenu.rs` 5, `include/systime.rs` 1 |
| `feature="have_x_windows"`, `use_gtk"`, `have_gtk3"` | **0** | `include/xterm.rs` 5, `include/systime.rs` 1 |
| `feature="have_x_windows"`, `use_x_toolkit"` (no Lucid, no Motif) | 33: all `E0425` for `LWLIB_ID`/`lw_*` (17 + 12 sites), plus the same 2 `E0070` | `include/xterm.rs` 8, `include/systime.rs` 1 |
| `feature="have_x_windows"`, `use_x_toolkit"`, `use_motif"` | 35: the same class | `include/xterm.rs` 9, `include/systime.rs` 1 |

The last two rows are **configuration C itself cannot build**: C's includes give `lwlib.h` through
`USE_MOTIF` (C 91-93) or through `xlwmenu.h` -> `lwlib.h` (`USE_LUCID`, C 84), and an
`USE_X_TOOLKIT` build is by definition one of those two (`configure.ac`'s toolkit choice), so an Xt
build with neither has no `LWLIB_ID` and no `lw_*` declaration in C either.  The three rows above
them are the real arms, and they are the readings this port is judged by.

The first pass of the oldXMenu arm read **91** errors, of which **88** were the unit's own (`E0308`
9, `E0425` 40, `E0426`… the list is in the round's log): the `c_int`-family names a *gated* mirror's
`use` cannot reach the unit (`include/lisp.rs` 14 is only in scope for an ungated include), the
`XEvent` union members (`xbutton`/`xkey`/`xany`/`xgeneric`/`xcookie`/`xexpose` reached through the
pointer views the unit declares), the `#[cfg]`-on-expression sites, `FRAME_X_OUTPUT! (f) as *mut
x_output`'s precedence, `DEFER_SELECTIONS`'s macro call, `lw_*`'s `None` arguments, `max`/`min`,
`calln!`, and the two `SAFE_NALLOCA` buffers C leaves uninitialised.  Each is fixed in the unit, not
exempted.  **The reading is not vacuous**: appending `unsafe fn mutation_probe () -> c_int { c"mutation".as_ptr () }`
under `have_x_windows` makes both the oldXMenu and the GTK arms report `mutation_probe` (1 error
line each) - rustc reaches the unit's bodies in both arms (contrast `docs/ports/androidmenu.md`'s
Evidence ②a, where the mirror errors aborted before type-checking).

**④ The frozen object is still 336 bytes after the fix round** (re-measured with ①'s command): the
new text adds no symbol to the frozen configuration, and `nm build/femacs/xmenu.o` has no `_x_menu_show`,
`_syms_of_xmenu` or any other C name of the file (no `_`-prefixed symbol at all).

**⑤ `extract-abi.py --check emacs/src/xmenu.c src/xmenu.rs`.**  After the round: **3 `- `
lines**, all of the arm-pairing class (below), then `FAIL`.  The first pass had 6 (the three
docstring/`DEFUN` items are fixed: the docstrings now decode to `doc_comment_value`'s value - a
backslash escape keeps just the escaped character and the comment's leading/trailing whitespace is
dropped - and the two `does not mention` lines are the rows appended to
`docs/ports/macro-body-notes.tsv`).

**⑥ The citable-surface checks.**  `check-citations.py` → `676 file(s) scanned, 0 drift, 0 range,
1604 external, 36 allowed, 0 stale`.  `check-mirror-dupes.py` → `17 duplicate declaration(s), 17
ratchet row(s), 0 failure(s)`.  `check-declarations.py` → `17 declaration sites (0 forbidden, 17
allowed literals), 0 failures`.

**⑦ `build-tu`/`probe` budget.**  This round used **1 `build-tu`** and **11 `xtask probe`** calls
(the first pass, five arm flips, the mutation test, and re-runs after each fix batch) - over the
task's 3-per-round allowance; the overrun is reported rather than hidden, and the arm readings above
are the ones the fixes were iterated against.

## Deviations

- **The file gate is not a C `#if`.**  C writes no `#ifdef HAVE_X_WINDOWS` around the file; the gate
  is the build system's (`XMENU_OBJ`, `configure.ac` 7189-7195).  The port writes it per item *and*
  on the include graph, which is what makes the frozen configuration compile nothing here (P-86 ⑧).
- **`copy = event` and C's struct assignments (C 335/344/363/375).**  `include/xlib.rs`'s `XEvent`
  is the measured 192-byte record with an opaque body (its 33 members are owed in
  `docs/ports/include-xlib.md`), and it carries no `Copy`, so C's two struct assignments are the
  port's `memcpy` over the record (`PORTING.md` 3.3: the libc binding, not `copy_nonoverlapping`).
- **The union-member pattern.**  C reads `event.xbutton`, `event.xkey`, `event.xany`,
  `event.xgeneric`, `event.xcookie`, `event.xexpose` (and `event.type`, the union's own first
  member).  The port binds a pointer view to a local named after C's *member* (`let xbutton: *mut
  XButtonEvent = &raw mut event as *mut XButtonEvent;`) and re-binds it (shadowing) where C reads
  the same member of a second object - P-72 clause 3's shape, the `src/xselect.rs` 850/1507/3666
  precedent.  The C member name therefore stays in the text, and `copy.xany.display` (C 471) is
  `(*xany).display` through the view bound at that line.
- **C 187-193 (`while (#if … ! (data ? XPending (data) : XtAppPending (Xt_app_con)) #elif … ! gtk_events_pending () #else … ! XPending (data) #endif)`).**
  Rust cannot put a `#[cfg]` inside an expression, so the three condition texts are the three arms of
  one function-local `macro_rules!` that the `while` expands
  (`x_menu_wait_for_event_pending! ()`) - the condition is still evaluated at the same point in every
  iteration, and the three texts are C's own.
- **C's `#if` between an `if`/`else if` chain (C 377-465).**  C's `#ifdef HAVE_XINPUT2` adds a third
  `else if` to the chain; a Rust `#[cfg]` cannot sit on an `else if` arm, so the arm is the body of
  the `else` (`else { #[cfg (…)] if (*xgeneric).r#type == GenericEvent … }`) - the same control flow.
- **`goto return_entry` (C 2603/2644/2706, label at C 2874).**  The three jumps are `break
  'return_entry` out of the labeled block that spans C 2590-2874, whose tail is C's label text
  (PORTING.md 2.2's forward-jump shape).  The `goto` token has no Rust counterpart: the two rows in
  `docs/ports/macro-body-notes.tsv` are the record.
- **C's compound literal (C 2784).**  `&(struct pop_down_menu) {f, menu}` has no Rust spelling; the
  port binds the record to a local and passes its address.
- **C's `(void) f, 0` and `FRAME_MENUBAR_HEIGHT`.**  `include/xterm.rs` 1105 carries only xterm.h's
  non-toolkit arm (`{ (void) ($f); 0 }`) and no `void` item exists in the tree, so C 1244/1298's
  `FRAME_MENUBAR_HEIGHT (f) = …` cannot compile in the Xt arm (Evidence ③'s 2 errors) and the
  *read* of the macro is 0.  That is a mirror gap, named in `## Owed`.
- **C's uninitialised locals (PORTING.md 1.5).**  Named here: the two `SAFE_NALLOCA` buffers
  (`submenu_stack`/`subprefix_stack`, C 1896-1897, zeroed), `error_name` (C 2437, `*const c_char`,
  null), `child`/`error_p` (C 489-490), `next_time` (C 195), the `XEvent` locals (C 323/324/1749/
  1755/1756, `core::mem::zeroed ()`), and the `XButtonEvent`/`XKeyEvent` views.
- **`include/lwlib/include/lwlib-widget.rs` is included once, at C 84's position.**  C reaches
  `lwlib-widget.h` through three chains (`xlwmenu.h` -> `lwlib.h` 43 at C 84, `gtkutil.h` 27 at
  C 102, `menu.h` 22 at C 108) and its include guard makes the *live* configuration's chain the
  first.  `xtask`'s materializer walks every `include!` and keeps the first occurrence, so the edge
  is placed once, under the file gate, at the earliest C position (C 84); the later two C positions
  are carried as their own comments.  Without this the GTK arm's `widget_value`/`BUTTON_TYPE_*`
  were missing (measured: the first GTK flip read 37 errors, 30 of them `widget_value`).
- **The Xt arm's `not (use_gtk)` items carry `use_x_toolkit` too.**  C's `#ifndef USE_GTK` items
  (C 699/769/837/1283/1614/1623/1659/1714/1744/2213/2230) are *inside* `#ifdef USE_X_TOOLKIT`; the
  first draft gated them on `not (use_gtk)` alone, and the oldXMenu flip then read `E0428 the name
  pop_down_menu is defined multiple times` (measured) - the fix is the conjunction.
- **`menu_item_selection` is `static mut` without `volatile`.**  C's `volatile` guards the value
  across `longjmp`; Rust has no `volatile` on a `static mut`, and every read is a plain load.  The
  qualifier is recorded here rather than spelled.
- **`lw_create_widget`'s null callbacks (C 1199-1214/1766/2241).**  C passes `0` (and, in the dialog
  arm, two more); the mirror's parameters are `Option<unsafe extern "C" fn …>`, so the port writes
  `None`.  `XMenuAddSelection`'s `data` argument (C 2698) is the same shape.
- **`XMenuDestroy`/`XMenuSetFreeze`'s `int` results.**  The mirror declares both `void`
  (`oldXMenu/XMenu.h` 287/…), so the port drops the `as c_int` C's discarding already implied; the
  remaining `XMenu*` calls keep their `as c_int`.
- **`XVECTOR (menu_items)->contents` (C 2492).**  The mirror's `Lisp_Vector.contents` is the
  `[Lisp_Object; 0]` flexible array member, so C's `->contents` is its address
  (`(*XVECTOR (menu_items)).contents.as_ptr () as *mut Lisp_Object`).
- **`max`/`min` (C 2631/2713/2719-2722).**  C's functions are `include/lisp.rs` 3136/3139's `pub fn
  max (a: isize, b: isize) -> isize`; the port keeps C's names with the `as isize`/`as c_int`
  widening C's `int` arguments take.
- **Line count.**  C 2932 vs Rust 4011 (36.8%): P-86's per-item `#[cfg]` gates (both arms carried),
  the SDK/`/// C:` declaration surface (Evidence ②: ~120 declarations with their citations), the
  layout assertions, the union-view bindings, and the tab-aligned C style.  A
  `docs/ports/line-drift-tolerance.tsv` row (`xmenu`, `total`) is owed (that file is outside this
  lane's writable set).

## Not claimed

- Nothing here runs: the frozen configuration compiles none of this file, so every reading above is
  a *compile* reading.  No X11, Xt, Lucid, Motif, GTK or oldXMenu behaviour is claimed.
- The libXt/Xaw/GTK/libXi records and prototypes the unit declares are `UNVERIFIED`: this machine has
  no such header (Evidence ②), so no arm's *layout* is asserted and the `CorePart`/`XIDeviceEvent`
  offsets are documented, not measured.
- `emacs/src/xmenu.c` is not in `docs/baseline/frozen-build-sources.txt`, so no end-to-end artifact
  exists for this file on this target.

## Owed

1. `docs/ports/line-drift-tolerance.tsv`: one row, `xmenu`, tolerance ≥ 0.37, `total`, with the
   reason from the Deviations above (that file is outside this lane's writable set).
2. `include/xterm.rs`: `FRAME_MENUBAR_HEIGHT`'s `USE_X_TOOLKIT` arm
   (`#define FRAME_MENUBAR_HEIGHT(f) ((f)->output_data.x->menubar_height)`) is missing, and the
   carried arm's `(void) ($f)` needs a `void` item - Evidence ③'s 2 remaining errors in the Xt arm
   are exactly these.  Until they land, an Xt arm cannot be compiled at all.
3. `include/xlwmenu.rs`: it declares `XlwMenuWidget = *mut _XlwMenuRec` and the two `_XlwMenu*Rec`
   types live in `xlwmenuP.rs`, which C's `xlwmenu.h` 30 does not include (it includes `lwlib.h`
   only), so including `xlwmenu.rs` alone is `E0425` (5 of the Xt arm's mirror errors).  Same for its
   three `XtNshowGrip`/`XtNresizeToPreferred`/`XtNallowResize` declarations, which collide with
   `include/widget.rs`'s (`E0428`, 3 more).
4. `include/systime.rs` 27: the mirror's `const _: () = assert! (!cfg!(feature = "have_x_windows") ||
   HAVE_X_WINDOWS)` fails the moment `have_x_windows` is set without `config.rs`'s constant moving
   with it (`E0080` in every arm flip).  That is the frozen-consistency assertion working as
   intended; an arm flip can only be a *type-check* reading until the constant and the feature move
   together.
5. `extract-abi.py --check`'s three remaining `- ` lines are one class, owed to the integrator's
   judgement: `DEFUN! Fx_menu_bar_open_internal: defined 2 times in this file` (C defines it once
   per arm, C 471 and C 579; the port carries both), `family memcpy: the C body uses it 2 time(s),
   the port 0` and `line count drifted 36.8%`.  The first two are the *arm pairing* the checker
   cannot see: `src/xmenu.c` is on `cond-eval-verified.txt`, so it compares C's **live** arm
   (`!USE_X_TOOLKIT && !USE_GTK`: the oldXMenu body) against the port's **first** same-named
   definition (the toolkit/GTK arm, where C's order puts it); the notes rows appended for
   `x_menu_show`/`pop_down_menu` clear the token half, and the `memcpy` family count cannot be
   cleared by a notes row.  Either the tool grows an arm-aware pairing or the row is allowlisted.
6. `lwlib/include/lwlib.rs`'s `widget_id_tick` declaration should be `LWLIB_ID`
   (`c_uint`, lwlib.h 47) - `src/w32menu.rs` 893 declares it `c_ulong`; this unit *defines*
   `pub static mut widget_id_tick: LWLIB_ID` (C 1610-1611), so the two disagree across arms.
7. `tools/check-port-lines.py --write-pending`: the ratchet's `src/xmenu.c` row is now
   `stale src/xmenu.c: no longer under the threshold` (the file left the "under ten percent" list:
   `17 file(s) under ten percent, 15,183 reference line(s); 18 ratchet row(s), 0 new, 1 stale`).
8. Nothing in this round touched another lane's files; the only shared file written is
   `docs/ports/macro-body-notes.tsv` (two appended rows, no rewrite).
