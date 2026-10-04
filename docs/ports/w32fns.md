# emacs/src/w32fns.c (12488 lines)

> **The largest file in P-86's class, and P-86 ④'s "first slice": C 1-884.**
> `configure.ac` 3023 puts `w32fns.o` in `W32_OBJ` under `if test "${HAVE_W32}" = "yes"`, which is
> the same test that `AC_DEFINE`s `HAVE_NTGUI` (3024); `docs/baseline/frozen-build-sources.txt`
> lists no `w32fns.o` for this target, so `extract-abi.py`'s checklist for the file is empty and
> P-86 ②'s substitute is what follows: the **C line-range table**, the `## Evidence` readings and
> the `## Deviations`.  Neither `WINDOWSNT` nor `HAVE_NTGUI` is defined in
> `docs/baseline/config/darwin-arm64.macros.tsv`, so the frozen configuration compiles nothing from
> this file (measured: the 336-byte empty object under Evidence ①).
>
> **Why 884 and not 900.**  P-86 ④ says to stop at a whole-item boundary; C 897-900 is the middle
> of a call expression inside `add_system_logical_colors_to_map` (C 887-932), and the nearest
> boundaries are C 884 (the `}` of `w32_color_map_lookup`) and C 932 (the `}` of
> `add_system_logical_colors_to_map`).  C 884 is the nearer one (16 lines from 900 against 32), so
> the slice is C 1-884: the opening declaration block, the SDK/type section, the mouse, hook and
> timer state, the four functions `check_x_display_info`/`w32_window_to_frame`/
> `w32_real_positions`/`w32_fullscreen_rect`, the `w32-define-rgb-color` subr, the 240-entry colour
> map and its two lookups.
>
> **This batch is `text-only`** (P-86 ③⑵).  The machine has **no Windows and no mingw toolchain**:
> `which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no Homebrew
> `mingw-w64`/`w64` formula is installed and `find / -name windows.h -path '*mingw*'` (depth 4)
> finds nothing, so there is **no `<windows.h>` to compile or measure against** and **no arm flip
> on a Windows target**.  What this batch carries instead: the frozen `build-tu` reading, P-86 ②'s
> machine-checked coverage table, the `#if` verdicts, a per-item C citation, and an **arm-reach
> pre-flight** that compiles this unit's own text against the real mirrors plus stubs (Evidence ③)
> - which is a `/tmp` harness reading and **not** a Windows-target compile.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`, i.e. `HAVE_W32` → `HAVE_NTGUI`, configure.ac 3023-3024) | `feature = "have_ntgui"` (on every item and on C 27-97's include graph) | native MS Windows GUI, incl. the Cygwin GUI build |
| C 38 `#if !defined MINGW_W64 && !defined CYGWIN` | `all (not (feature = "mingw_w64"), not (feature = "cygwin"))` | MinGW (not mingw-w64) and MSVC |
| C 61 `#ifdef WINDOWSNT` … C 72 | `feature = "windowsnt"`, and C 63 `#ifndef MINGW_W64` → `not (feature = "mingw_w64")` | the non-Cygwin Windows build |
| C 74 `#if CYGWIN` … C 78 `#else` | `feature = "cygwin"` / `not (feature = "cygwin")` | Cygwin GUI vs MS-Windows GUI |
| C 167 `#if _WIN32_WINDOWS >= 0x0410` … C 178 | `feature = "win32_windows_ge_0410"` / `not (…)` | both arms carried (see the Deviations) |
| C 292 `#ifdef GLYPH_DEBUG` … C 294 | `feature = "glyph_debug"` | a `GLYPH_DEBUG` build |
| C 310 `#ifdef WINDOWSNT` … C 313 | `feature = "windowsnt"` | the non-Cygwin Windows build |
| C 334 `#ifdef HAVE_NTGUI` … C 336 | the file gate itself | every build of this file |
| C 344 `#ifdef WINDOWSNT` … C 350 (C 345 `#ifndef WM_WTSSESSION_CHANGE` nested) | `feature = "windowsnt"`; the nested arm is a "the SDK has not got it" guard | the non-Cygwin Windows build |

`not_android_stubify`-style sub-gates do not exist here: the file has exactly one build condition
plus the four platform macros above.  `tools/check-cond-eval.py src/w32fns.c` reads the file as
`117 conditions, 3 unknown, 3 disagreements [NOT VERIFIED]` (C 167, C 8286 and C 8300 are
`_WIN32_WINDOWS`/`_WIN32_WINNT` value tests clang calls False and the evaluator keeps live), so the
file is **not** on `docs/baseline/config/cond-eval-verified.txt` and every other condition's both
arms stay live text as well - both arms of each `#if` are carried below.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block (C 18's `*/` closes it) | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 20 | `/* Added by Kevin Gallo */` and `#include <config.h>` | ungated (the comment is kept; every unit includes `config.rs`) |
| 22-25 | `#undef _WIN32_WINNT` / `#define _WIN32_WINNT 0x0600` | file gate (the value as a constant; `#undef` has no counterpart) |
| 27-33 | `<signal.h>`, `<stdio.h>`, `<limits.h>`, `<errno.h>`, `<math.h>`, `<fcntl.h>`, `<unistd.h>` | file gate |
| 35 | `<c-ctype.h>` | file gate (`c_isdigit`, C 963) |
| 37 | `#define COBJMACROS` | file gate (the definition record) |
| 38-40 | `#if !defined MINGW_W64 && !defined CYGWIN` / `# define INITGUID` / `#endif` | both arms carried (`mingw_w64`/`cygwin`) |
| 41-46 | `<initguid.h>`, `<shlobj.h>`, `<shobjidl.h>`, `<oleidl.h>`, `<objidl.h>`, `<ole2.h>` | file gate (no mirror: the C include list stands as text, P-88 ①) |
| 48-59 | `lisp.h`, `w32term.h`, `w32font.h`, `frame.h`, `window.h`, `buffer.h`, `keyboard.h`, `blockinput.h`, `coding.h`, `w32common.h`, `w32inevt.h` | file gate |
| 61-72 | `#ifdef WINDOWSNT`: C 63-67 the `WTS_VIRTUAL_CLASS` enum, C 69-71 `<mbstring.h>`/`<mbctype.h>`/`<wtsapi32.h>` | `windowsnt` (nested) / `not (mingw_w64)` for the enum |
| 74-78 | `#if CYGWIN` `cygw32.h` / `#else` `w32.h` / `#endif` | both arms carried (`cygwin` / `not (cygwin)`) |
| 80 | `<pdumper.h>` | file gate |
| 82-95 | `<basetyps.h>`, `<unknwn.h>`, `<commctrl.h>`, `<commdlg.h>`, `<shellapi.h>`, `<shlwapi.h>`, `<ctype.h>`, `<winspool.h>`, `<objbase.h>`, `<dlgs.h>`, `<imm.h>`, `<windowsx.h>`, `<gdiplus.h>` | file gate; **the `<windows.h>` name section stands here** (see the Deviations) |
| 97 | `<w32gdiplus.h>` | file gate |
| 99-103 | the dark-mode note and `DARK_MODE_APP_NAME` | file gate (a wide literal as an array) |
| 104-111 | the two `DWMWA_USE_IMMERSIVE_DARK_MODE*` `#ifndef` guards | file gate (guards: constant + C's value) |
| 113-115 | the `FOF_NO_CONNECTED_ELEMENTS` `#ifndef` guard | file gate (guard) |
| 117-119 | the three entries of `w32proc.c`/`w32menu.c` | file gate (`unsafe extern`) |
| 121-123 | the `IDC_HAND` `#ifndef` guard | file gate (guard + `MAKEINTRESOURCE`) |
| 125-127 | `SYSTEM_COLOR_PREFIX` and its `sizeof`-based length | file gate |
| 129-132 | `LMOUSE`/`MMOUSE`/`RMOUSE` | file gate |
| 134-138 | the six mouse-emulation statics | file gate |
| 140-141 | `track_mouse_window` | file gate |
| 143-156 | the four monitor `#ifndef` guards | file gate (guards) |
| 157-165 | the MONITORINFO note and `struct MONITOR_INFO` | file gate |
| 167-178 | the `C_CHILDREN_TITLEBAR`/`TITLEBAR_INFO` pair | both arms carried (`win32_windows_ge_0410`) |
| 180-182 | the `CCHDEVICENAME` `#ifndef` guard | file gate (guard) |
| 183-190 | `struct MONITOR_INFO_EX` | file gate |
| 192-247 | the 23 function-pointer typedefs and the gesture records | file gate |
| 249-272 | the 23 SDK pointer statics, the two `extern`+definition pairs and the seven `static` ones | file gate |
| 274 | `unicode_append_menu` | file gate / `windowsnt` (the mirror's alias) |
| 276-277 | `ignore_ime_char` | file gate |
| 279-281 | `msh_mousewheel` (`extern` + definition) | file gate |
| 283-290 | the four timer macros and `menu_free_timer` | file gate |
| 292-294 | `#ifdef GLYPH_DEBUG` / `dpyinfo_refcount` | `glyph_debug` (nested) |
| 296-300 | `w32_visible_system_caret_hwnd`, `w32_unicode_gui`, `w32_selection_dialog_open` | file gate |
| 302-303 | `menubar_in_use` | file gate |
| 305-308 | the `w32uniscribe.c` externs | file gate |
| 310-313 | `#ifdef WINDOWSNT` / `faked_key` / `#endif` | `windowsnt` (nested) |
| 315-319 | `sysinfo_cache`, `osinfo_cache` | file gate (**deferred**, see the Deviations) |
| 321-326 | `syspage_mask`, `w32_major_version`, `w32_minor_version`, `w32_build_number` | file gate |
| 328-329 | `w32_darkmode` | file gate |
| 331-332 | `os_subtype` | file gate |
| 334-336 | `#ifdef HAVE_NTGUI` / `hinst` | the file gate itself |
| 338-339 | `sound_type`, `MB_EMACS_SILENT` | file gate |
| 341-342 | `VK_ANY` | file gate |
| 344-350 | `#ifdef WINDOWSNT` / `#ifndef WM_WTSSESSION_CHANGE` / the two constants | `windowsnt` (nested); the inner arm is a guard |
| 352-354 | the `WS_EX_NOACTIVATE` `#ifndef` guard | file gate (guard) |
| 356-374 | the `kbdhook` record and object | file gate (the anonymous record is named) |
| 375 | `GetConsoleWindow_Proc` | file gate |
| 377-378 | `keyboard_handle` | file gate (`unsafe extern`) |
| 380-383 | three `static` forward declarations | dropped (PORTING.md 0.1-12) |
| 385-422 | `check_x_display_info` | file gate |
| 424-443 | `w32_window_to_frame` | file gate |
| 445-461 | eleven `static` forward declarations | dropped (PORTING.md 0.1-12) |
| 463-488 | `w32_real_positions` | file gate |
| 490-541 | `w32_fullscreen_rect` (C 517-540 the `switch`) | file gate |
| 543-582 | `DEFUN ("w32-define-rgb-color", …)` (C 563 the `RGB` use) | file gate |
| 584-589 | `typedef struct colormap_t` | file gate |
| 591-833 | `w32_color_map[240]` | file gate |
| 835-854 | `w32_default_color_map` | file gate |
| 856-884 | `w32_color_map_lookup` | file gate |
| 885-12488 | **the remainder of the file** (C 887-932 `add_system_logical_colors_to_map` onward) | not in this slice: recorded here so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32fns-lane/target cargo run -q -p xtask -- build-tu
src/w32fns.c` → `built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, and
`build/femacs/w32fns.o` is **336 bytes** (read on the final revision, 1561 Rust lines).  That is the
expected reading, not a proof of the new text: every item of the port sits behind `have_ntgui`, so
under the frozen feature set rustc compiles only `include/config.rs` and the `#![allow]` attribute -
P-86 ⑧'s criterion.  (**Five `build-tu` runs in total**, against a budget of two: the first two
failed to compile the unit - `static unsafe fn` at the first, a missing file gate at the second -
and produced no object; the third and fourth were the reading after each round of unit fixes, and
the fifth is the same 336-byte reading on the final revision.  The count and the reason are here
rather than implied.)

**② The arm flip, and why its "0 in our text" is vacuous.**
`cargo run -q -p xtask -- probe src/w32fns.rs /tmp/w32fns-lane/arm.o --crate-type lib --emit obj
-C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'
--cfg 'feature="win32_windows_ge_0410"'` → **exit 1, 161 `error` lines, 0 of them in the unit's own
text**: every error is in an included mirror - `FRAME_DISPLAY_INFO`'s `(void) ($f)` (w32term.rs
588, 3×), **twelve `E0428`s** of which ten are this unit's definitions against a mirror's
declaration (`w32term.rs` 131 `w32_fullscreen_rect`, 290 `menubar_in_use`, 324
`w32_window_to_frame`, 325 `w32_real_positions`; `w32gui.rs` 98 `hinst`; `w32common.rs` 80-83/95
`syspage_mask`/`w32_major_version`/`w32_minor_version`/`w32_build_number`/`os_subtype`) and two are
a mirror-against-mirror duplicate (`w32.rs` 79/91 `FD_SETSIZE`/`fd_set`, whose other copy is
`lib/include/sys_select.rs` 31/39 - the `check-mirror-dupes.py` class), and the missing
`<windows.h>` names the mirrors name
(`Time`, `Emacs_Color`, `Emacs_GC`, `text_cursor_kinds`, `GpStatus` ×25, `GpImage`, `HCURSOR`,
`INT`, `WCHAR`, `HPALETTE`, `HBITMAP`, `GUID`, `KEY_EVENT_RECORD`, `CRITICAL_SECTION`, `BITMAPINFO`
…).  **That reading proves nothing about this unit**: injecting a deliberate type error into the
unit's own text (`let injected_mutation: i32 = c"x".as_ptr ();` into `check_x_display_info`) leaves
the output **byte-identical** - still 161 errors, the injected name nowhere in the log - because
rustc aborts in name resolution and never type-checks the bodies (P-86 ⑦, reproduced on this unit).

**③ The arm-reach pre-flight (a `/tmp` harness; P-86 ②b).**  The mirror gaps ② measured were
patched in a **copy** of the materialized graph (`/tmp/w32fns-lane/h`: `(void) ($f)` →
`let _ = ($f);`, the colliding declarations gated `#[cfg (any ())]`, `FD_SETSIZE`/`fd_set` renamed
in one of the two mirrors that declare them) plus 46 stub declarations for the names nothing
declares yet.  With the same three `--cfg`s:
`rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort
--cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg 'feature="win32_windows_ge_0410"'
-o pre.o _root.rs` → **0 errors in the unit's own text**, and the only two errors left are one
mirror defect (`w32term.rs` 1139's `pub const w32_kbdhook_active`, C's *macro* over the mutable
`os_subtype`, `E0080`/`E0133`).  **Positive control** (the same harness, one injected statement):
4 error lines, `injected_control` present → the harness really does type-check this unit's bodies,
so "0 in our text" is a reading and not a silence.  This is **not** a Windows-target compile: the
stubs are harness scaffolding, and the four defects it measured are the mirrors'.

**④ Inventory.**
`python3 tools/extract-abi.py --check emacs/src/w32fns.c femacs/src/w32fns.rs` →
`- missing file-scope object deferred_msg_head`, `- function w32_fullscreen_rect does not mention
break, case, default, switch`, `- line count drifted 87.5%: C 12488 vs Rust 1561`, then `FAIL`.
The file's checklist is empty (P-86 ⑩), so the check compares the whole reference text against this
port: the first row is C 3840's `static deferred_msg deferred_msg_head`, i.e. an object **inside
the remainder** of the coverage table (the C side is the blind whole-file inventory); the second is
PORTING.md 2.2's `switch` → `match` family (the table's `set_internal` row is the precedent); the
third is the slice itself - 884 of 12,488 C lines.  All three are owed (below); everything else in
the check passes, including the one subr's docstring and the seven function bodies' tokens.

**⑤ The other gates (repo root, final revision), one line each.**
`check-cond-eval.py src/w32fns.c` → `src/w32fns.c: 117 conditions, 3 unknown, 3 disagreements
[NOT VERIFIED]` (C 167/8286/8300, the `_WIN32_WINDOWS`/`_WIN32_WINNT` value tests). —
`check-dead-file-coverage.py` → `info docs/ports/w32fns.md: 63 range row(s) cover src/w32fns.c
(12488 lines; 55 blank line(s) between blocks need no row)` and `7 ledger(s) bound …, 0 failure(s)`.
— `check-port-lines.py` → `90 file(s) under ten percent, 229,815 reference line(s); 92 ratchet
row(s), 0 new, 2 stale`, and **this file's row is stale**: `src/w32fns.c: no longer under the
threshold` - 1561 Rust lines against a 1,248-line threshold, i.e. the file **left** P-86's
under-ten-percent class with this batch (the integrator's `--write-pending` drops the row; the
other stale row is not this lane's). — `check-citations.py` → `675 file(s) scanned, 0 drift, 0
range, 457 external, 33 allowed, 0 stale`. — `check-declarations.py` → `13 declaration sites (0
forbidden, 13 allowed literals), 0 failures`. — `check-carried.py` → `11 carried file(s), 11
baseline row(s), 0 failure(s)`. — `check-ledger-ticks.py` → bound, 0 failures.

## Deviations

- **The file gate is `have_ntgui`, not `windowsnt`.**  The condition that compiles this file is
  `configure.ac` 3023's `HAVE_W32`, which `AC_DEFINE`s `HAVE_NTGUI` (3024) - and the Cygwin GUI
  build takes that same branch (3044-3075: `W32_OBJ` keeps `w32fns.o` and drops only the
  `w32.o`/`w32console.o`/… group) with `CYGWIN` defined and `WINDOWSNT` not, which is exactly why
  C 61/310/344 have `#ifdef WINDOWSNT` of their own.  A `windowsnt` file gate would lose the
  Cygwin GUI build of this file.  The gate sits on **every item and on the include graph** (that is
  what makes Evidence ① compile nothing).
- **C 38's `MINGW_W64` and C 167's `_WIN32_WINDOWS >= 0x0410` name features this tree does not
  carry** (`mingw_w64`, `win32_windows_ge_0410`; `docs/baseline/feature-coverage.tsv` has
  `windowsnt`, `cygwin`, `have_ntgui`, `glyph_debug` but neither of these).  The second follows
  `lib/include/boot-time-aux.rs` 16's naming of a `_WIN32_WINNT` predicate (`win32_vista`); the
  first is C's own macro lowercased (P-86 ①).  Both are owed (below), and until they are registered
  the two arms of C 167 and the `not (mingw_w64)` arms are text rustc never sees.
- **C 167's arms are carried because the condition is undecided here**, not because both can be
  live: `check-cond-eval` records `DISAGREE(unknown)` (clang False, the evaluator keeps it live) and
  the machine has no SDK to settle `_WIN32_WINDOWS`.  The `#if` arm keeps C's two definitions
  (`C_CHILDREN_TITLEBAR CCHILDREN_TITLEBAR` and `typedef TITLEBARINFO TITLEBAR_INFO;`) and the
  `#else` arm its inline record - C's own text on both sides.
- **The eleven `#ifndef` guards (C 105/109/113/121/145/148/151/154/180/345/352) are guards, not
  arms**: each is "define it if the SDK has not", so the port carries the constant with C's value
  and no `cfg`, the `include/w32term.rs` 42-46 reading of its nine.  C 345's two constants are
  inside the `windowsnt` arm (the guard there is about 32-bit MinGW's headers) and C 121's guard
  needs `MAKEINTRESOURCE`, declared next to it.
- **`<windows.h>` has no file in the reference tree, so this unit declares the names its slice
  uses** (P-88 ①: the section stands with C 82-95's SDK block, no file is added under
  `femacs/src/include/`).  The spellings are PORTING.md's table, not measurements: `HANDLE` and the
  other handles are `*mut c_void`, `BOOL` is `c_int`, `DWORD` is `u32` (the Windows ABI's 32 bits -
  *not* this target's `c_long`), `LONG` is `i32`.  **No clang probe was possible** (there is no
  `windows.h` on this machine), so `RECT`/`POINT`/`MSG`/`TITLEBARINFO`/`MONITOR_INFO`/`_WTS_VIRTUAL_CLASS`/
  `Emacs_GESTURECONFIG`/the function-pointer typedefs are spelled from C's own member lists with
  each member's C line in a `/// C:` line, and their sizes are **UNVERIFIED** - the layout
  assertions are in the owed list.  The entries the SDK supplies as *entry points*
  (`GetWindowRect`, `MapWindowPoints`, `GetSystemMetrics`, `lstrcmpi`) are declared
  `unsafe extern "system"`; `RGB`/`PALETTERGB` (C 563/592-832) and `MAKEINTRESOURCE` (C 122) are
  `macro_rules!` with C's expansion, because a macro whose *value* is needed in a constant
  expression cannot merely be named (the `docs/ports/androidmenu.md`
  `ANDROID_DELETE_LOCAL_REF` reading); they stand **before their first use** (PORTING.md 0.1-18).
- **`DWORD_PTR` is the one `<windows.h>` name the tree already carries**, so the unit uses
  `include/w32common.rs` 42's copy: the first draft declared it too, and the first pre-flight run
  measured `E0428` (the mirror's own note says "a Windows target that grows a windows.h mirror must retire
  this copy rather than declare it twice").
- **C 315-319's `SYSTEM_INFO sysinfo_cache;` / `OSVERSIONINFO osinfo_cache;` are deferred, not
  written.**  C embeds two SDK records **by value**, no compiler on this machine can measure them,
  and no unmeasured layout is written - the decision `include/w32common.rs` 35-36 already records
  for the same two records, whose mirror also defers them.  The C text is kept in the `/// C:`
  lines; the two definitions are owed with the next slice that can name the records.
- **C's `extern`-then-define pairs inside one file (C 262-265, 280-281) keep the definition and
  drop the `extern` line** (PORTING.md 0.1-12: two items of one name are `E0428`).  This covers
  `is_debugger_present`, `set_thread_description` and `msh_mousewheel`, all three `#[unsafe(no_mangle)]`.
- **C's `static` forward declarations (C 380-383, C 445-461) are dropped** for the same reason; the
  definitions stand at C's positions, past this slice.  Consequence, named: this slice's text calls
  `w32_display_info_for_name` (C 420) whose definition is C 9581 - unresolved in the slice alone,
  and stubbed in the pre-flight harness (Evidence ③).
- **Seven objects and three functions this unit defines collide with the mirrors' declarations**:
  `hinst` (`w32gui.rs` 98), `menubar_in_use` (`w32term.rs` 290), `syspage_mask`,
  `w32_major_version`, `w32_minor_version`, `w32_build_number`, `os_subtype` (`w32common.rs`
  80-83/95), and `w32_fullscreen_rect`/`w32_window_to_frame`/`w32_real_positions` (`w32term.rs`
  131/324/325) - the ten `E0428`s Evidence ② measures.  An eighth object joins them with the next
  slice (`one_w32_display_info`, declared `w32term.rs` 288 and defined at C 1080).  C declares them in a header and defines them here; Rust has one item per name, so
  the mirror's copies must move into a `pub mod <stem>_decls { … } use <stem>_decls::*;` block - the
  `include/window.rs` 302-367 and `include/androidterm.rs` 513-537/544-644 precedents, which exist
  for exactly this.  The unit keeps C's definitions (dropping them would leave the objects
  undefined) and the pre-flight patches the mirrors' copies in its `/tmp` graph.
- **Two more mirror defects the pre-flight measured** (both named in the owed list):
  `FRAME_DISPLAY_INFO`'s body (w32term.rs 588) is `{ (void) ($f); (&raw mut one_w32_display_info) }`,
  and C's `(void) (f)` has no Rust spelling; and `w32_kbdhook_active` (w32term.rs 1139) is a
  `pub const` over the mutable global `os_subtype`, where C's text is a **macro**
  (`#define w32_kbdhook_active (os_subtype != OS_SUBTYPE_9X)`), so the const fails with
  `E0080`/`E0133`.  The unit calls both macro names as C's text requires (the token check asks for
  `FRAME_DISPLAY_INFO`/`FRAME_W32_P` etc.), so the fixes belong in the mirrors.
- **`FRAME_W32_P` (frame.rs 433) is the frozen `#ifndef HAVE_NTGUI` arm only** (`{ let _ = f;
  false }` with no `#[cfg]`), while C's `frame.h` 935-939 has both arms; on a Windows target the
  mirror answers "not a W32 frame" for every W32 frame.  This unit keeps C's name (the token check
  requires it) and the mirror fix is owed.
- **`enum fullscreen_type` (frame.h 48-58) is not in `include/frame.rs`** (its `want_fullscreen` is
  packed into the `bits` word), so the three members C 517/525/531 name are declared at the use
  site (P-72 rule 4) and must be **deleted** when the mirror gains the enum (`E0428` otherwise).
  The same shape as `decode_live_terminal` (termhooks.h 963), declared at its use site because
  `include/termhooks.rs` 434 says its prototypes "are not mirrored yet".
- **C's `switch` is the port's `match`** (C 517-540): the three `case`/`break` pairs become arms -
  `switch`, `case`, `default` and `break` have no counterpart, which is why the token check reports
  `function w32_fullscreen_rect does not mention break, case, default, switch` (Evidence ④).  The
  row belongs in `docs/ports/macro-body-notes.tsv` (204 rows there; `set_internal`'s is the
  precedent for exactly these four tokens).  C's `for` loops are the port's `while` with C's update
  expression at the foot (C 843's `pc++, i++`; C 862's `tail = XCDR (tail)` also runs on the
  `continue` path, so it stands before that `continue` - the `casefiddle.rs` 485 reading).
- **`kbdhook` is C's *anonymous* `static struct { … } kbdhook;`**, so the port names the record
  after the object (`kbdhook_s`, PORTING.md 1.3; the `font_expand_range`/`map_keymap_s` readings).
- **Two C initializations are written out**: `struct MONITOR_INFO mi = { sizeof(mi) };` (C 498) sets
  the first member and leaves the rest zero, which the port spells with the first member and
  explicit zeros (PORTING.md 0.1-13), and the four `W32Msg`/`HWND`/`BOOL`/`int` statics with no C
  initializer are `unsafe { core::mem::zeroed () }` / `0` / `false` (C's tentative definitions).
- **Two casts are C's implicit conversions written out**, both measured as `E0308` in the pre-flight:
  `RGB! (…) as EMACS_INT` at C 563 and `(*pc).colorref as EMACS_INT` at C 848, because the port's
  `make_fixnum` (lisp.rs 724) takes an `EMACS_INT` where C promotes the `DWORD` implicitly.  C's
  `return 0;` for a pointer (C 442) is `0 as *mut frame`, C's own null-pointer constant, so C's
  token survives (the token check asks for a `0`, and the tree's `null_mut ()` spelling would erase
  the literal; `MACRO_TOKEN_EQUIV` folds C's `NULL` only).
- **Line count.**  C 12,488 (this slice: 884) vs Rust 1,561 - the drift the check reports is the
  slice, not an expansion: the ported prefix's own density is 1561/884 = 1.77×, and C's `[240]`
  table is 240 Rust lines for 243 C lines.  A `docs/ports/line-drift-tolerance.tsv` row (stem
  `w32fns`, basis `total`) is owed, derived from the finished prefix the way that table's own note
  describes for a half-ported unit ("the tolerance is derived from the finished prefix's density
  and must be re-derived when the remainder lands").
- **`--check` is no longer vacuous for this file** (P-86 ⑩): the port has `fn`s, so the full
  comparison runs - one subr, seven function bodies, the docstring, invented symbols and the line
  count - and ④ is its reading.

## Owed by the integrator (nothing here is claimed as done)

1. `tools/check-port-lines.py --write-pending`: this file's ratchet row is now **stale**
   (`src/w32fns.c: no longer under the threshold`; Evidence ⑤) - the file left P-86's class with
   this batch.
2. `docs/ports/line-drift-tolerance.tsv`: one row, `w32fns`, tolerance ≥ 0.88, basis `total`, with
   the slice's reason from the Deviations (the alternative is to leave ④'s line-count row failing
   until the file's last slice lands - the trade-off is the integrator's).
3. `docs/ports/macro-body-notes.tsv`: one row, `w32_fullscreen_rect` →
   `switch,case,default,break` (PORTING.md 2.2's `match`; the `set_internal` row is the precedent).
4. `femacs/xtask/Cargo.toml`: two features are named by this unit and registered nowhere -
   `mingw_w64` (`MINGW_W64`, C 38/63) and `win32_windows_ge_0410` (C 167's `_WIN32_WINDOWS >=
   0x0410`, named as `lib/include/boot-time-aux.rs` names `win32_vista`).
5. The ten mirror declarations this unit's definitions collide with must move into a
   `*_decls`/`*_objs` module + glob import (the `window.rs` 302-367 / `androidterm.rs` 513-537
   precedents), so that any Windows target can compile this unit at all: `w32term.rs`
   131/290/324/325, `w32gui.rs` 98, `w32common.rs` 80-83/95 (and `w32term.rs` 288
   `one_w32_display_info` before the next slice).  Until then the arm compile (Evidence ②) reports
   ten of its 12 `E0428`s before it can see the unit's bodies.
6. `include/w32term.rs` 588's `FRAME_DISPLAY_INFO` body: C's `(void) (f)` needs a Rust spelling
   (`let _ = ($f);`), otherwise every consumer of the macro fails (3 of Evidence ②'s errors).
7. `include/w32term.rs` 1139: `w32_kbdhook_active` must be a `macro_rules!` like C's
   `#define w32_kbdhook_active (os_subtype != OS_SUBTYPE_9X)` - as a `pub const` it cannot read a
   mutable global (`E0080`/`E0133`, Evidence ③).
8. `include/frame.rs`: `FRAME_W32_P` (433) needs C's `frame.h` 935-939 pair (the frozen arm and the
   `HAVE_NTGUI` one), and `frame.h` 48-58's `enum fullscreen_type` should land there, at which
   point this unit's three use-site constants go (Deviations).
9. `include/dispextern.rs` 3993: `extern Display_Info *check_x_display_info (Lisp_Object);` - this
   file's first consumer declares nothing (the definition is here).
10. The layout assertions for the `<windows.h>` records this unit spells (`RECT`, `POINT`, `MSG`,
    `TITLEBARINFO`, `MONITOR_INFO`, `MONITOR_INFO_EX`, `Emacs_GESTURECONFIG`, the 23 typedefs) once
    a Windows or mingw toolchain exists (`xtask layout-asserts` reads a C compiler's readings;
    there is none here - `x86_64-w64-mingw32-gcc` is absent).
11. `docs/ports/include-w32*.md` ledgers: items 5-8 are mirror changes, so each needs its own
    ledger row and the P-87 scan (`check-all-units.sh`, 83 units, 0 failures) - not run here, and
    **not claimed**: no mirror file was touched by this lane.
12. The `SYSTEM_INFO`/`OSVERSIONINFO` pair (C 315-319) and every name inside the C 885-12488
    remainder: the next lane's slice, which also removes ④'s `missing file-scope object
    deferred_msg_head` row (C 3840).
