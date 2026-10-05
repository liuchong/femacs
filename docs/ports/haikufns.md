# emacs/src/haikufns.c (3324 lines)

> **One of P-86's class: the frozen configuration compiles nothing here.**  `configure.ac` 3097
> clears `HAIKU_OBJ` and 3114 fills it with the Haiku platform objects under
> `test "${HAVE_BE_APP}" = "yes"` - the same test that `AC_DEFINE`s `HAVE_HAIKU` (3106-3110) - and
> `build/emacs-ref/src/Makefile` reads `HAIKU_OBJ = ` (empty), so
> `docs/baseline/frozen-build-sources.txt` lists no `haikufns.o` for this target and
> `extract-abi.py`'s checklist for the file is empty (P-86 ②).  What follows is that clause's
> substitute: the **C line-range table**, the `## Evidence` readings and the `## Deviations`.
>
> Port: `femacs/src/haikufns.rs` (**4,331 lines**), which carries the file **whole, C 1-3324** -
> no slice.  The 56 functions, 34 `DEFUN`s, five macros, two structs, the four cursor/handler
> tables and the seven file-scope `tip_*` objects of the reference all stand at C's own line, in
> C's own order, with every `#if`'s arms carried (P-51/P-86 ①).
>
> **Acquisition note - this port is `text-only` (P-86 ③⑵).**  This machine has no Haiku SDK and no
> Haiku toolchain (the measurement is `docs/ports/haikuterm.md` Evidence ②, same machine): there
> is **no arm flip on a Haiku target**, and nothing here is compiled or linked against a Haiku
> system.  The `<InterfaceKit.h>`/`<ApplicationKit.h>` surface this file's headers pull in is
> already mirrored by `include/haikugui.rs` and `include/haiku_support.rs`; the reference-tree
> names **no** mirror carries are declared at their use site (P-88 ①), each with one `/// C:` line
> and the standing `UNVERIFIED` mark.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`HAIKU_OBJ`, i.e. `HAVE_HAIKU` <- `HAVE_BE_APP`, configure.ac 3106-3114) | `feature = "have_haiku"` (on every item *and* on C 21-42's include graph, P-86 ⑧) | a Haiku app-kit configuration |
| C 787-792 and C 1078-1083 `#ifdef USE_BE_CAIRO` with the nested `HAVE_HARFBUZZ` | `feature = "be_cairo"` (+ `feature = "harfbuzz"`) | a Haiku build configured with cairo |
| C 1879-1886 `#ifndef USE_BE_CAIRO` (around the `if`/`else` of `haiku_set_inhibit_double_buffering`) | `not (feature = "be_cairo")` | a Haiku build without cairo |
| C 3311-3321 `#ifdef USE_BE_CAIRO` (`cairo-version-string`) | `feature = "be_cairo"` | a Haiku build configured with cairo |
| C 2404-2406 and C 2638-2651 `#ifdef ENABLE_CHECKING` | `feature = "enable_checking"` | a build with `ENABLE_CHECKING` |
| `HAVE_WINDOW_SYSTEM` / `HAVE_EXT_MENU_BAR` - not gates of this file: `lisp.h` 3925-3936 defines both from `HAVE_HAIKU` | `feature = "have_window_system"`, `feature = "have_ext_menu_bar"` (set by the pre-flight; a Haiku configuration always has them) | every Haiku configuration |

`HAIKU_OBJ`/`HAVE_HAIKU` is the only build condition; the four sub-gates above are this file's own
`#if`s.  `tools/check-cond-eval.py src/haikufns.c` reads the file as
`9 conditions, 0 unknown, 0 disagreements [VERIFIED]`, and all nine take their false arm on the
frozen target.

## C line ranges

| 1-18 | the file's licence/declaration block (PORTING.md 0.1-19) | dropped |
| 19-20 | `#include <config.h>` | ungated |
| 21-42 | the include list (C 21-42), the two `.xbm`s included | file gate |
| 44-48 | the four colour macros | file gate |
| 50-67 | the seven file-scope tip objects | file gate |
| 69-70 | the two `static` forward declarations | dropped (PORTING.md 0.1-12) |
| 72-104 | `get_geometry_from_preferences` | file gate |
| 105-117 | `haiku_update_after_decoration_change` | file gate |
| 119-166 | `haiku_change_tool_bar_height` | file gate |
| 168-229 | `haiku_change_tab_bar_height` | file gate |
| 231-237 | `haiku_set_no_focus_on_map` | file gate |
| 239-257 | `haiku_set_tool_bar_lines` | file gate |
| 259-284 | `haiku_set_tool_bar_position` | file gate |
| 286-306 | `haiku_set_tab_bar_lines` | file gate |
| 308-323 | `gamma_correct` | file gate |
| 325-391 | `haiku_get_color` | file gate |
| 393-407 | `haiku_display_info_for_name` | file gate |
| 409-443 | `check_haiku_display_info` | file gate |
| 445-454 | `haiku_set_title_bar_text` | file gate |
| 456-471 | `haiku_set_title` | file gate |
| 473-495 | `haiku_set_child_frame_border_width` | file gate |
| 497-538 | `haiku_set_parent_frame` | file gate |
| 540-579 | `haiku_set_z_group` | file gate |
| 581-585 | `haiku_explicitly_set_name` | file gate |
| 587-598 | `haiku_set_no_accept_focus` | file gate |
| 600-607 | `initial_setup_back_buffer` | file gate |
| 609-629 | `unwind_create_frame` | file gate |
| 631-639 | `unwind_create_tip_frame` | file gate |
| 641-652 | `haiku_decode_color` | file gate |
| 654-675 | `haiku_set_foreground_color` | file gate |
| 677-681 | `do_unwind_create_frame` | file gate |
| 683-1018 | `haiku_create_frame` | file gate |
| 1020-1246 | `haiku_create_tip_frame` | file gate |
| 1249-1318 | `compute_tip_xy` | file gate |
| 1320-1370 | `haiku_hide_tip` | file gate |
| 1372-1385 | `haiku_set_undecorated` | file gate |
| 1387-1401 | `haiku_set_override_redirect` | file gate |
| 1403-1443 | `haiku_set_menu_bar_lines` | file gate |
| 1445-1547 | `frame_geometry` | file gate |
| 1549-1571 | `haiku_set_background_color` | file gate |
| 1573-1597 | `haiku_set_cursor_color` | file gate |
| 1599-1603 | `haiku_set_cursor_type` | file gate |
| 1605-1624 | `haiku_get_pixel` | file gate |
| 1626-1655 | `haiku_put_pixel` | file gate |
| 1657-1722 | `haiku_free_frame_resources` | file gate |
| 1724-1736 | `haiku_iconify_frame` | file gate |
| 1738-1760 | `haiku_visualize_frame` | file gate |
| 1762-1773 | `haiku_unvisualize_frame` | file gate |
| 1775-1794 | `haiku_set_internal_border_width` | file gate |
| 1796-1803 | `haiku_set_frame_visible_invisible` | file gate |
| 1805-1814 | `frame_set_mouse_pixel_position` | file gate |
| 1816-1824 | `haiku_query_color` | file gate |
| 1826-1839 | `check_x_display_info` | file gate |
| 1840-1869 | `haiku_set_name` | file gate |
| 1871-1891 | `haiku_set_inhibit_double_buffering` | file gate |
| 1893-1900 | `haiku_set_sticky` | file gate |
| 1901-1914 | ``struct user_cursor_info`` | file gate |
| 1916-1931 | ``struct user_cursor_bitmap_info`` | file gate |
| 1933-1935 | ``INIT_USER_CURSOR`` | file gate |
| 1937-1955 | ``custom_cursors[]`` | file gate |
| 1957-1978 | ``cursor_bitmaps[]`` | file gate |
| 1979-2009 | ``cursor_bitmaps_for_id[28]`` | file gate |
| 2011-2054 | `haiku_create_colored_cursor` | file gate |
| 2055-2086 | `haiku_free_custom_cursors` | file gate |
| 2088-2173 | `haiku_set_mouse_color` | file gate |
| 2175-2180 | `haiku_set_use_frame_synchronization` | file gate |
| 2182-2186 | `haiku_should_pass_control_tab_to_system` | file gate |
| 2190-2208 | `DEFUN haiku-set-mouse-absolute-pixel-position` | file gate |
| 2210-2232 | `DEFUN haiku-mouse-absolute-pixel-position` | file gate |
| 2234-2241 | `DEFUN xw-display-color-p` | file gate |
| 2243-2253 | `DEFUN xw-color-defined-p` | file gate |
| 2255-2273 | `DEFUN xw-color-values` | file gate |
| 2275-2283 | `DEFUN x-display-grayscale-p` | file gate |
| 2285-2309 | `DEFUN x-open-connection` | file gate |
| 2311-2322 | `DEFUN x-display-pixel-width` | file gate |
| 2324-2335 | `DEFUN x-display-pixel-height` | file gate |
| 2337-2346 | `DEFUN x-display-mm-height` | file gate |
| 2349-2358 | `DEFUN x-display-mm-width` | file gate |
| 2360-2366 | `DEFUN x-create-frame` | file gate |
| 2368-2387 | `DEFUN x-display-visual-class` | file gate |
| 2389-2701 | `DEFUN x-show-tip` | file gate |
| 2703-2708 | `DEFUN x-hide-tip` | file gate |
| 2710-2718 | `DEFUN x-close-connection` | file gate |
| 2720-2728 | `DEFUN x-display-list` | file gate |
| 2730-2736 | `DEFUN x-server-vendor` | file gate |
| 2738-2744 | `DEFUN x-server-version` | file gate |
| 2746-2752 | `DEFUN x-display-screens` | file gate |
| 2754-2763 | `DEFUN haiku-get-version-string` | file gate |
| 2765-2773 | `DEFUN x-display-color-cells` | file gate |
| 2775-2783 | `DEFUN x-display-planes` | file gate |
| 2785-2793 | `DEFUN x-double-buffered-p` | file gate |
| 2795-2817 | `DEFUN x-display-backing-store` | file gate |
| 2819-2861 | `DEFUN haiku-frame-geometry` | file gate |
| 2863-2885 | `DEFUN haiku-frame-edges` | file gate |
| 2887-2944 | `DEFUN haiku-read-file-name` | file gate |
| 2946-2957 | `DEFUN haiku-put-resource` | file gate |
| 2959-2995 | `DEFUN haiku-frame-list-z-order` | file gate |
| 2997-3014 | `DEFUN x-display-save-under` | file gate |
| 3016-3072 | `DEFUN haiku-frame-restack` | file gate |
| 3074-3096 | `DEFUN haiku-save-session-reply` | file gate |
| 3098-3147 | `DEFUN haiku-display-monitor-attributes-list` | file gate |
| 3149-3202 | ``haiku_frame_parm_handlers[]`` | file gate |
| 3204-3324 | `syms_of_haikufns` | file gate |

The table is machine-read by `python3 tools/check-dead-file-coverage.py`: every non-blank line of
C 1-3324 is covered, the rows ascend without overlapping, and the last row ends at C 3324.  The
blank lines between blocks carry no row.

## Evidence

**① The frozen configuration (`build-tu`), twice, plus a `/tmp` reproduction at the final
revision.**  `cd femacs && CARGO_TARGET_DIR=/tmp/haik-lane/target cargo run -q -p xtask -- build-tu
src/haikufns.c` → `built src/haikufns.rs -> ../build/femacs/haikufns.o`, **exit 0**, and
`build/femacs/haikufns.o` is **336 bytes** - an empty object.  That is the expected reading, not a
proof of the new text: every item of the port sits behind `have_haiku`, so under the frozen feature
set rustc compiles only `include/config.rs` and the `#![allow]` attribute (P-86 ⑧'s criterion).
`shasum -a 256 build/femacs/haikufns.o` = `d1ea88b2fe3d960975d134e566e6593217e9ad67abdc2db5a451ea9620edd0ed`.
The first run (the port as it stood before the two `.xbm` objects and the two cursor structs were
gated) failed with eleven `E0425`s - `c_char`, `Lisp_Object`, `ptrdiff_t`, `c_int`/`c_void` not in
scope - which is P-86 ⑧'s criterion measuring exactly the ungated items.  The reading above is the
second run; the unit's text was then edited twice more (the `enable_checking` local move and the
`check-mirror-dupes` module reshape, Deviations 12/14), and with the budget spent the final
revision's frozen reading is the same compile reproduced in `/tmp` with xtask's own rustc arguments
(`rustc --edition 2024 --crate-type lib --emit obj --crate-name haikufns -C panic=abort
-C codegen-units=1 -C opt-level=0 src/haikufns.rs`) → **exit 0**, **336 bytes**,
`/tmp/haik-lane/frozen_final.o`, same sha256.  It is the same reading, not a third `build-tu` run,
and it is named as such.  (The `/tmp/haik-lane` scratch directory this ledger names throughout was
deleted when the lane finished, as the budget requires; every reading is reproduced by the command
as written.)

**② No Haiku-target arm flip exists on this machine (P-86 ③⑵, `text-only`).**  The measurement is
`docs/ports/haikuterm.md` Evidence ② (same machine, same toolchains): no Haiku SDK, no
`x86_64-unknown-haiku`/`i686-unknown-haiku` std, no `libbe`.  So the port is **not compiled or
linked against a Haiku system**, and the reading that replaces the flip is ③.

**③ The arm-reach pre-flight (xtask's `probe`, and the four sub-gate arms).**
`cd femacs && CARGO_TARGET_DIR=/tmp/haik-lane/target cargo run -q -p xtask -- probe
src/haikufns.rs /tmp/haik-lane/probe_arm.o --crate-type lib --emit metadata
--cfg 'feature="have_haiku"' --cfg 'feature="have_window_system"' --cfg 'feature="have_ext_menu_bar"'`
→ **exit 1, one error, 0 in this unit's own text**:

* **the one error is in a mirror copy**: `error[E0080]: evaluation panicked: assertion failed:
  !cfg!(feature = "have_haiku") || HAIKU` at
  `include/systime.rs` 28 - the HAIKU constant's assertion, the same gap
  `docs/ports/haikuterm.md` Owed 6 names (`include/systime.rs`'s `HAIKU` constant); no Haiku
  configuration can reach a full crate until that mirror round lands.  7 warnings, all mirror-side
  `unused doc comment`s.

The same command with the sub-gates **off** (`have_haiku`+`have_window_system`+`have_ext_menu_bar`
only) reads identically: exit 1, one error (the same mirror assertion), 0 in the unit.  Because
both `probe` runs were spent before the last two edits, the final revision's four arm readings were
reproduced in `/tmp` with the same materialization rule and rustc arguments (`_root.rs` per file,
one copy per mirror, `[xtask dedupe]` for a repeated edge, cfg attributes kept so rustc itself
evaluates the gated edges - validated by reproducing `probe`'s reading exactly at the revision it
was run on):

| pre-flight arm | cfgs | exit | errors | in the unit | in mirrors |
| --- | --- | ---: | ---: | ---: | --- |
| file gate + every sub-gate | + `be_cairo harfbuzz enable_checking` | 1 | 1 | **0** | 1 (`include/systime.rs` 28) |
| file gate only (sub-gates false) | - | 1 | 1 | **0** | 1 (same) |
| `be_cairo`+`harfbuzz` only | - | 1 | 1 | **0** | 1 (same) |
| `enable_checking` only | - | 1 | 1 | **0** | 1 (same) |

(`--cfg 'feature="have_haiku"'` **alone** is not a configuration: `lisp.h` 3925-3936 makes
`HAVE_WINDOW_SYSTEM` and `HAVE_EXT_MENU_BAR` follow `HAVE_HAIKU`, and without them the mirrors'
types are absent - measured: 84 errors, 65 in the unit.  The pre-flight sets them, as
`docs/ports/haikuterm.md` does.)

**③a Mutation test (P-86 ⑦: the reading is not vacuous).**  Injecting one deliberate type error
into the unit's own text (`let injected_control: i32 = c"x".as_ptr ();` into `haiku_query_color`)
changes the reading to **one new error**, `error[E0308]: mismatched types` naming exactly that
line, and no other error moves.  Run twice: on the official `probe` (revision before the last two
edits) and on the final revision through the `/tmp` reproduction above.  The file was restored from
a copy and `shasum -a 256` verified identical before and after
(`3a412031264fc593e5ca7dc54f9e1d2b9cc6e3c75f9baf0183edc9a79a073bc1`).

**④ Inventory.**  `python3 tools/extract-abi.py --check emacs/src/haikufns.c
femacs/src/haikufns.rs` → **one row and `FAIL`**: `line count drifted 30.3%: C 3324 vs Rust 4331`.
Nothing else fails: no missing function, macro, type, field or object, and no missing body token -
the five token rows the check first reported (`1`, `output_data`, `0`, `doc`, `goto`) are now the
five `docs/ports/macro-body-notes.tsv` rows this lane appended (Deviations 19), and the row above
is the expected `line-drift-tolerance.tsv` row that is the integrator's.

**⑤ The other gates (repo root, final revision), one line each.**  (The whole-tree numbers moved
while this lane ran - other lanes landed and fixed their own files - so the readings below are the
last ones this lane took.)
`check-declarations.py` → `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`, and
this unit has no finding. - `check-mirror-dupes.py` → `17 duplicate declaration(s), 17 ratchet
row(s), 0 failure(s)`; **this file has none** (the eight it first reported were fixed by
Deviations 14). -
`check-ledger-ticks.py` → `233 ledger(s) bound (a function tick to compare), 0 failure(s), ...`. -
`check-dead-file-coverage.py` → `106 ledger(s) bound (a C range table), 0 failure(s)`, with
`info docs/ports/haikufns.md: 103 range row(s) cover src/haikufns.c (3324 lines; 99 blank line(s)
between blocks need no row)`. -
`check-citations.py` → `676 file(s) scanned, 0 drift, 0 range, 1514 external, 36 allowed, 0 stale`
- this unit's citations are clean.  **Not run here** (outside this lane's budget): `gate-fast`,
`check-all-units.sh`, `cargo test`, `extract-abi --all`.

## Deviations

1. **The file gate is on the include graph too** (P-86 ⑧): every `include!` of C 21-42 sits behind
   `#[cfg (feature = "have_haiku")]`, which is why the frozen object is 336 bytes (Evidence ①).
2. **C 37-38's two `.xbm` files have no mirror**; P-88 ①'s device declares the two objects in the
   unit, byte for byte from `bitmaps/leftptr.xbm`/`leftpmsk.xbm`.  The four `#define`s those files
   also carry (`left_ptr_width`/`_height`/`_x_hot`/`_y_hot`, `left_ptrmsk_width`/`_height`) are read
   by no statement of this file and are not transcribed.
3. **The two `static` forward declarations at C 69-70 are dropped** (PORTING.md 0.1-12); each
   definition stands at C's own line.
4. **The `static` helpers whose address C 3149-3203's table takes are `unsafe extern "C" fn`**
   (a function-pointer member cannot be a Rust-ABI item, PORTING.md 0.1-6/0.2): 17 of them
   (`haiku_set_child_frame_border_width`, `haiku_set_menu_bar_lines`, `haiku_set_mouse_color`,
   `haiku_explicitly_set_name`, `haiku_set_title`, `haiku_set_tab_bar_lines`,
   `haiku_set_tool_bar_lines`, `haiku_set_sticky`, `haiku_set_tool_bar_position`,
   `haiku_set_inhibit_double_buffering`, `haiku_set_undecorated`, `haiku_set_parent_frame`,
   `haiku_set_no_focus_on_map`, `haiku_set_no_accept_focus`, `haiku_set_z_group`,
   `haiku_set_override_redirect`, `haiku_set_use_frame_synchronization`).
5. **`frame.h`'s accessors C *writes* get a two-arm unit-local macro under C's name** (the
   `src/pgtkfns.rs` 436-560 shape): `FRAME_TOOL_BAR_LINES`/`_HEIGHT`, `FRAME_TAB_BAR_LINES`/
   `_HEIGHT`, `FRAME_MENU_BAR_LINES`/`_HEIGHT`, `FRAME_EXTERNAL_MENU_BAR`, `FRAME_HAIKU_MENU_BAR`,
   `FRAME_Z_GROUP`, `FRAME_FOREGROUND_PIXEL`, `FRAME_BACKGROUND_PIXEL`, `FRAME_UNDECORATED`,
   `FRAME_OVERRIDE_REDIRECT`, `FRAME_NO_FOCUS_ON_MAP`, `FRAME_NO_ACCEPT_FOCUS`, `FRAME_TOTAL_COLS`,
   `FRAME_PIXEL_WIDTH`, `FRAME_PIXEL_HEIGHT`, `FRAME_DISPLAY_INFO` (write arm, shadowing the
   mirror's read-only macro), `FRAME_OUTPUT_DATA` (same), and the `bool_bf` member macros
   (`explicit_name`, `no_split`, `can_set_window_size`, `was_invisible`, `tooltip`,
   `wants_modeline`, `tab_bar_resized`, `tool_bar_resized`, `tab_bar_redisplayed`,
   `tool_bar_redisplayed`).
6. **`frame_set_flag` writes one bit**, so the three multi-bit fields use C's own masked
   read-modify-write: `output_method!` (4 bits, C 743), `want_fullscreen!` (read, C 1001) and
   `FRAME_Z_GROUP!` (2 bits, C 557/562/567).
7. **`window_bits!`'s `update_mode_line` write arm takes `true` where C 1430 writes `1`** (the
   macro's arms are bool-typed; a recorded spelling row for the token check).
8. **C's anonymous local record in `get_geometry_from_preferences` (C 76-80) is named `r`** after
   C's own variable; Rust's separate type/value namespaces let the two names coexist.
9. **`Window` resolves to `include/nsgui.rs`'s `c_int` in this crate** (the measured X-compat
   collision `docs/ports/include-haikugui.md` records), while C's `haiku_output.parent_desc` is
   `haiku` (haikugui.h 97); the two writes at C 1059/1086 therefore carry an explicit `as Window`
   cast.  The member is never read by this unit.
10. **C's two `goto start_timer` jumps (C 2472/2497) and their label (C 2693) are the labelled
    block `'start_timer`** (PORTING.md 2.2), and C's `continue` in `Fhaiku_frame_list_z_order`
    (C 2980) is the labelled block `'continue_:` whose `break 'continue_;` is the same jump (the
    `src/androidterm.rs` 279-293 shape; the label keeps C's token).  The `goto` row is registered in
    `macro-body-notes.tsv`.
11. **`Emacs_Cursor *recolored` (C 2090) is `Emacs_Cursor` in the port**: C stores
    `haiku_create_colored_cursor`'s `void *` in it through an implicit `void *` conversion the port
    cannot spell; the value is what C actually stores.
12. **Uninitialised C locals are zeroed and named** (PORTING.md 1.5): `x`/`y` (C 1268),
    `r16`/`g16`/`b16`/`ui_color` (C 328/331/332), `n`/`rc` (C 2091), `old`/`frame_cursor` (C 2090),
    the four `Emacs_Color`s, `frame_geometry`'s eight geometry locals and `MonitorInfo`
    (C 3113), and the `#ifdef ENABLE_CHECKING` locals `row`/`end` are declared at C's own position
    (C 2404-2406) and assigned in C 2638-2651's block.
13. **The five `int`/`bool` mixes C writes** (`haiku_set_name (…, 1)`, `adjust_frame_size (…, 0, …)`,
    `SET_FRAME_ICONIFIED (…, true)`, `BWindow_set_visible (…, true)`, `BView_draw_lock (…, false, …)`)
    are written with the Rust literal that matches the parameter's type (`true`, `false`,
    `true as c_int`); two of them are `macro-body-notes.tsv` rows.
14. **Eight `haiku_support.h`/`haikuterm.h` prototypes whose mirrored copies carry no return type**
    (`be_get_display_planes`, `be_get_display_color_cells`, `be_is_display_grayscale`,
    `EmacsView_double_buffered_p`, `be_plain_font_height`, `be_string_width_with_plain_font`,
    `be_get_display_screens`, `haiku_term_init`) are declared in a **unit-local module** and
    imported by name, so neither the unit nor the mirror re-declares the name in the same scope
    (P-26/P-39 as `check-mirror-dupes.py` reads it).  The mirror round that owes the corrected
    prototypes is `## Owed` 2.
15. **The reference-tree names no mirror carries are declared at their use site** (P-88 ① / P-72
    rule 4, the `src/pgtkfns.rs` 296-323 shape), each with one `/// C:` line: `frame.h`'s
    `gui_set_*` family (for C 3149-3203), `adjust_frame_size`, `free_frame_menubar`,
    `fset_parent_frame`, `decode_window_system_frame`, `check_window_system`, `make_frame`,
    `make_minibuffer_frame`, `make_frame_without_minibuffer`, `delete_frame`,
    `frame_set_id_from_params`, `gui_figure_window_size`, `set_frame_cursor_types`,
    `store_frame_param`, `get_frame_param`, `check_int_nonnegative`; `dispextern.h`'s
    `try_window`, `gui_update_cursor`, `update_face_from_frame_parameter`, `init_frame_faces`,
    `free_frame_faces`, `redraw_frame`, `adjust_frame_glyphs`, `free_glyphs`, `clear_glyph_matrix`,
    `clear_current_matrices`, `update_single_window`, `clear_frame`, `gui_display_get_arg`,
    `gui_default_parameter`, `request_sigio`, `unrequest_sigio`, `parse_color_spec`, `xstrcasecmp`,
    `face_change`, and the `resource_types` enum's four members; `font.h`'s `register_font_driver`
    and the two `ftcr*` drivers; `lisp.h`'s `make_specified_string`, `fatal`, `quit`,
    `process_pending_signals`; `window.h`'s `set_window_buffer`; `termhooks.h`'s
    `decode_live_terminal`; `haiku_support.h`'s `be_create_cursor_from_id`;
    `<kernel/OS.h>`'s `write_port` and `<cairo.h>`'s three version macros (**UNVERIFIED**: no
    Haiku/cairo header on this machine).  `struct redisplay_interface` (C 2695's `flush_frame`)
    is carried in the unit too, behind `not (feature = "dispextern_face_records")`, because the
    mirror's copy is gated behind that feature and is off in every configuration today.
16. **`include/dispextern.rs` keeps its text-position accessors inside its (non-glob-imported)
    `dispextern_unit_names` module**, and a `use` of that module does not carry its
    `macro_rules!`s, so `SET_TEXT_POS!` is a unit-local copy of C's own body (the
    `src/xdisp.rs` 1918-1926 shape).
17. **`include/frame.rs`'s `flush_frame!` is shadowed by a unit-local copy** with the
    `*mut c_void` cast this unit's `FRAME_RIF!` needs (C 2695's statement); measured: the mirror's
    macro is an `E0308` at its own line when expanded from this unit (`## Owed` 3).
18. **`haiku_create_colored_cursor`'s pixel test keeps C's arithmetic**
    (`mask[x / 8] >> (x % 8) & 1` on a `const char *`), and C's `if (w > width) w = width;`
    (C 2467, an assignment with no effect) is kept as written.
19. **`docs/ports/macro-body-notes.tsv`**: five rows appended at the end of the file (one append):
    `haiku_explicitly_set_name` → `1`, `haiku_free_frame_resources` → `output_data`,
    `haiku_set_internal_border_width` → `0`, `syms_of_haikufns` → `doc`, `Fx_show_tip` → `goto`.
20. **Line count.**  C 3,324 vs Rust 4,331 (30.3%): Rust needs a line per item's gate attribute and
    per `unsafe`/cast that C leaves implicit.  The `docs/ports/line-drift-tolerance.tsv` row is the
    integrator's (`## Owed` 1).

## Owed

1. `docs/ports/line-drift-tolerance.tsv`: one row, `haikufns`, tolerance >= 0.31, basis `total`
   (Deviations 20).  Not in this lane's writable set.
2. The mirror round's eight prototypes with no return type: `include/haiku_support.rs` 686-704
   (`be_get_display_planes`, `be_get_display_color_cells`, `be_is_display_grayscale`,
   `EmacsView_double_buffered_p`, `be_plain_font_height`, `be_string_width_with_plain_font`,
   `be_get_display_screens`) and `include/haikuterm.rs` 449 (`haiku_term_init`, whose C prototype
   returns `struct haiku_display_info *`).  Owner: `docs/ports/include-haiku_support.md` /
   `docs/ports/include-haikuterm.md`.  Measured here; the unit works around them (Deviations 14).
3. `include/frame.rs` 1835-1840's `flush_frame!` omits the `*mut c_void` cast (an `E0308` at its own
   line when expanded from this unit).  Owner: `docs/ports/include-frame.md`; worked around
   (Deviations 17).
4. `include/systime.rs` 28's `const _: () = core::assert! (!cfg!(feature = "have_haiku") || HAIKU);`
   is the one error of the arm-reach pre-flight, so no Haiku configuration can compile a full crate
   until the `HAIKU` constant lands.  Owner: `docs/ports/include-systime.md` (the same item
   `docs/ports/haikuterm.md` Owed 6 names).
5. `check-port-lines.py`'s ratchet row for `src/haikufns.c`: 4,331 Rust lines for 3,324 C lines -
   `--write-pending` is the integrator's step.
6. This ledger's `## C line ranges` table is machine-read by `check-dead-file-coverage.py`; a future
   slice that moves an item must keep the table ascending and complete.

## Not claimed

- **No Haiku-target compile or link.** There is no Haiku SDK or toolchain on this machine, so the
  arm flips of Evidence ③ are a `/tmp` harness reading, not a Haiku build; nothing here runs.
- **No runtime, display or device behaviour.** Fonts, cursors, the tooltip frames, the menu bar, the
  file panel and the monitor-attribute list are transcribed, not exercised.
- **`xtask gate-fast`, `check-all-units.sh`, `cargo test` and `extract-abi --all` were not run**
  (the lane's budget names them as out of scope), and the C++ half of the backend (`libbe`,
  `be_*`'s implementations) is out of reach here.
- **The four `#define`s of the two `.xbm`s** and the `.xbm` dimension constants are not transcribed
  (Deviations 2).
- **The five `macro-body-notes.tsv` rows are declarations of a spelling difference**, not proof the
  bodies match; the body comparison itself is the `--check` reading of Evidence ④.
