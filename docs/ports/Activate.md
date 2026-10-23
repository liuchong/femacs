# emacs/oldXMenu/Activate.c (642 lines)

> Filled in: the port is `oldXMenu/Activate.rs` (**926 lines**).  Until this round it was the
> empty unit that docs/status.md 3.31 prescribed; P-86 ① overturns that reading ("3 行的头注释不是
> '落完'，是'还没开始'"), and `oldXMenu/Internal.rs` is the sibling that already carries this
> directory's shape.

**File gate**: `feature = "have_x_windows"` (P-86 clause 8).  The reference text carries no macro of
its own for this file: `oldXMenu/Makefile.in`'s `OBJS` block lists the object unconditionally, and it
is `configure.ac` 7287-7295 (`HAVE_X11 = yes`, `HAVE_GTK != yes`, `USE_X_TOOLKIT = none`) that
decides whether the library is built at all.  `oldXMenu/Internal.rs` 8-16 records the same reading,
and `## Deviations` 1 below keeps the reference's own words for it.

**Nothing in the frozen configuration compiles this unit**: `HAVE_X11`, `HAVE_X_WINDOWS` and
`USE_X_TOOLKIT` are `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so `xtask build-tu`
leaves the 336-byte empty object (P-08), and this target has no X11 window system to run the arm
against.  The arm reading below is therefore a `rustc` reading of the materialized mirror graph.

**The one sub-gate the reference spells** is `HAVE_XINPUT2` at C 529-533 (`case GenericEvent`); it is
carried as `feature = "have_xinput2"`, and its `#else` arm is empty.  `#if 1` at C 429-442 is live in
every configuration, so it is carried as text (P-86 clause 1).  No reading taken here flips the
sub-gate: the probe flips the file gate alone.

**This unit is the directory's first with X11 calls of its own** (eighteen entry points, five `XEvent`
member records, ten event/mask constants, one keysym pair), where `SetAEQ.rs`-class units and
`insque.rs` name only XMenu's own types.  Two consequences the readings made visible, both recorded
in `## Deviations` and `## Owed`: the include-graph dedupe expands `include/xlib.rs` inside
`mod xmenu_int` (so this unit imports the six names it spells from `xlib_surface`), and
`include/xlib.rs` still lacks `XFontStruct`, which `XMenu.h` 274/288 names.

## C line ranges

The rows are contiguous and ascending; C's blank lines between the blocks need no row (the gate
`tools/check-dead-file-coverage.py` reads it that way).

| C | block | arm |
| --- | --- | --- |
| 1-114 | the file's opening block (the two notices and the XMenuActivate description comment) | dropped (PORTING.md 0.1-19, P-86 clause 1) |
| 116 | `#include "XMenuInt.h"` | the file gate; the mirror is included inside `mod xmenu_int` and glob-imported (`## Deviations` 8) |
| 117 | `#include <X11/keysym.h>` | the file gate; no mirror holds it, so the two names it gives are declared in the unit (P-88 clause 1) |
| 119-120 | the debug comment and `static int x_menu_grab_keyboard = 1;` | the file gate; C's `static` stays an unexported `static mut` |
| 122-124 | `static Wait_func wait_func;` / `static void* wait_data;` / `static Translate_func translate_func = NULL;` | the file gate; the two function pointers are `Option`s (P-27) |
| 126-131 | `XMenuActivateSetWaitFunction` | the file gate |
| 133-137 | `XMenuActivateSetTranslateFunction` | the file gate |
| 139-149 | `XMenuActivate`'s definition head (nine parameters, `register` on two) | the file gate |
| 150-183 | the locals: four `int`s, five pointers, two `XEvent`s, two `Bool`s, `root`/`child`/`root_x`/`root_y`/`win_x`/`win_y`/`mask`/`keysym`, the `XMEventQue` typedef and its two queue pointers | the file gate; C's uninitialised locals are zero-initialised (`## Deviations` 4) |
| 184-192 | the "no panes" test - `XME_NOT_INIT` and `XM_FAILURE` | the file gate |
| 194-201 | `_XMGetPanePtr`, the null test, `cur_p->activated = cur_p->active` | the file gate |
| 203-209 | the current-selection comment and `_XMGetSelectionPtr` | the file gate |
| 211-221 | `_XMTransToOrigin` and the `menu->x_pos`/`menu->y_pos` stores | the file gate |
| 223-225 | `XMenuRecompute` | the file gate |
| 227-235 | `_XMWinQueFlush` | the file gate |
| 237-244 | the two raising loops (`menu->p_list->next` forward, `->prev` back) | the file gate |
| 246-255 | the `XMapSubwindows` loop | the file gate |
| 257-264 | `XSync (display, 0)` | the file gate |
| 266-296 | `XGrabPointer` / the `XGrabKeyboard` pair with `x_menu_grab_keyboard`, and the `XME_GRAB_MOUSE` return | the file gate; `None`/`True`/`False`/`GrabModeAsync` are the unit's declarations |
| 298-312 | mapping the panes and the `XRaiseWindow` of the current one | the file gate |
| 314 | `cur_s = NULL;` | the file gate |
| 316-321 | the event loop's head: the `wait_func` call and `XNextEvent` | the file gate |
| 322-355 | `case Expose:` (the association lookup, the AEQ queueing, the two background calls, `_XMRefreshPane`) | the file gate; `event.xexpose` is a pointer view of the mirror's `XEvent` (`## Deviations` 7) |
| 356-445 | `case EnterNotify:` (the pass-through tests, the selection branch, the pane branch with `XQueryPointer`, the raise walk and the live `#if 1` block) | the file gate; C's five inner `break`s are `## Deviations` 6 |
| 446-467 | `case LeaveNotify:` (the deactivation and `cur_s = NULL`) | the file gate |
| 469-500 | `case ButtonPress:` / `case ButtonRelease:` (the `p_num`/`s_num`/`data` stores and the three return states) | the file gate |
| 501-512 | `case KeyPress:` / `case KeyRelease:` (`XLookupKeysym`, `XK_g`+`ControlMask`, `XK_Escape`) | the file gate |
| 513-528 | `default:` (the AEQ queueing and the `_XMEventHandler` call) | the file gate |
| 529-534 | `#ifdef HAVE_XINPUT2` / `case GenericEvent:` / `#endif`, then the `}` that closes the `switch` | `feature = "have_xinput2"`; its `#else` arm is empty, and the closing brace is outside the condition |
| 535-539 | the "selection has been made" break | the file gate |
| 541-549 | unmapping the panes | the file gate |
| 551-555 | `XUngrabPointer` / `XUngrabKeyboard` | the file gate |
| 557-565 | deactivating the current selection | the file gate |
| 567-571 | deactivating the current pane | the file gate |
| 573-576 | `XSync (display, 0)` | the file gate |
| 578-585 | the remaining-queue loop's head and `XNextEvent` | the file gate; `QLength` is the `XQLength` entry point (`## Deviations` 5) |
| 587-608 | the five "does this event belong to XMenu" cases, the `continue` and `FALLTHROUGH;` | the file gate; the five cases are the merged `_` arm (PORTING.md 2.2) |
| 609-623 | the `default:` arm's foreign-event queueing and the `}` that closes the `switch` | the file gate |
| 624-632 | returning the queued events (`XPutBackEvent`) and freeing them | the file gate |
| 634 | `wait_func = 0;` | the file gate; the null function pointer is `None` |
| 636-642 | `_XMErrorCode = XME_NO_ERROR; return (ret_val);` and the closing brace | the file gate |

## Evidence

1. **frozen reading** (the file gate off): `cd femacs && CARGO_TARGET_DIR=/tmp/activate-lane/
   cargo-target cargo run -q -p xtask -- build-tu oldXMenu/Activate.c` -> **exit 0**,
   `build/femacs/Activate.o` **336 bytes**, `md5 dbed68837694579185377dbb0821ab62` - the empty object
   the gate leaves (the same reading `docs/ports/oldXMenu-Internal.md` records for its file).
2. **arm flip**, three takes, each
   `cargo run -q -p xtask -- probe oldXMenu/Activate.rs /tmp/activate-lane/activate.rlib
   --crate-type=lib -C panic=abort --cfg 'feature="have_x_windows"'`:
   - take 1: **exit 1, 69 errors**, and they name this unit's own text (`_root.rs` 536's
     `None as Window`, 665/740's `help_callback (…)`, 901's `wait_func = None`) as well as the
     mirrors' - so the reading is this unit's, not a vacuous "0 errors of a broken graph".
   - take 2 (after the `xlib_surface` import, the qualified `None` and the two
     `unwrap_unchecked` calls): **exit 1, 5 errors**, all in the mirrors: `XFontStruct` x2 in
     `oldXMenu/include/XMenu.rs` 275/289 (reported twice) and `c_double` in
     `lib/include/stdlib.rs` 16.
   - take 3 (the revision in this ledger's `## Hashes` row below): **exit 0, 0 errors, 86 warnings** - `extern block uses type
     `_XDisplay`/`_XGC`, which is not FFI-safe` (83 + 3), the `improper_ctypes` class the opaque
     `Display`/`GC` records raise for every X11 unit (`oldXMenu-Internal.md` Evidence 2 records the
     same class at 85).
3. **the X11 surface is measured, not remembered** - `clang -std=gnu23 -I/opt/homebrew/include` on
   this machine (Apple clang 21.0.0.0, target arm64-apple-darwin25.6.0, xorgproto 2025.1, libX11
   1.8.13), with `_Alignof`/`offsetof` printed for each record:
   - `XAnyEvent` `[sizeof=40, align=8]`, `display` 24, `window` 32.
   - `XKeyEvent` `[sizeof=96, align=8]`, `window` 32, `state` 80, `same_screen` 88.
   - `XButtonEvent` `[sizeof=96, align=8]`, `window` 32, `button` 84.
   - `XCrossingEvent` `[sizeof=104, align=8]`, `window` 32, `same_screen` 88, `state` 96.
   - `XExposeEvent` `[sizeof=64, align=8]`, `window` 32, `count` 56.
   - the constants: `KeyPress` 2, `KeyRelease` 3, `ButtonPress` 4, `ButtonRelease` 5, `EnterNotify`
     7, `LeaveNotify` 8, `Expose` 12, `GenericEvent` 35, `ControlMask` 4, `GrabModeAsync` 1,
     `Success` 0, `None` 0, `CurrentTime` 0, `XK_g` 0x67, `XK_Escape` 0xff1b, `Time` 8, `XEvent` 192.
   - the entry points the unit declares: `nm -gU /opt/homebrew/lib/libX11.6.dylib` shows
     `_XLookupKeysym`, `_XGrabKeyboard`, `_XGrabPointer`, `_XMapSubwindows`, `_XMapWindow`,
     `_XNextEvent`, `_XPeekEvent`, `_XPending`, `_XPutBackEvent`, `_XQLength`, `_XQueryPointer`,
     `_XRaiseWindow`, `_XSetWindowBackground`, `_XSetWindowBackgroundPixmap`, `_XSync`,
     `_XUngrabKeyboard`, `_XUngrabPointer`, `_XUnmapWindow`.
4. **the gates this lane owed**:
   - `python3 tools/extract-abi.py --check emacs/oldXMenu/Activate.c oldXMenu/Activate.rs` ->
     **one finding**: `line count drifted 44.2%: C 642 vs Rust 926` (`## Owed` 1).  The name
     comparison is clean: no missing function (the three definitions in the C text are all carried),
     no missing file-scope object (`x_menu_grab_keyboard`, `wait_func`, `wait_data`,
     `translate_func`), no missing field of `_xmeventque`, no stub and no invented exported symbol.
     The one token finding the first take printed (`XMenuActivate does not mention case, default,
     fallthrough, qlength, switch`) is recorded in `docs/ports/macro-body-notes.tsv` (one row,
     appended).
   - `python3 tools/check-declarations.py` -> **0 failures**.
   - `python3 tools/check-mirror-dupes.py` -> **0 failures** (the tool's scope is `src` and
     `lib`; `oldXMenu` units are not scanned, and this unit declares no name an
     in-scope mirror carries).
   - `python3 tools/check-citations.py` -> **0 range, 0 drift**; this unit's `Xlib.h`/`X.h`/
     `keysym.h`/`keysymdef.h` citations are in the `external` bucket (system headers, counted, never
     failed), and its `XMenuInt.h`/`XMenu.h` citations resolve.
   - `python3 tools/check-dead-file-coverage.py` -> **every non-blank line of the reference is in the
     table above** (0 failures); the rows ascend, do not overlap and end at C 642.
   - `python3 tools/check-ledger-ticks.py` -> **0 failures** (the three ticks below all occur in the
     port's text).

## Hashes

| file | lines | md5 | sha256 |
| --- | ---: | --- | --- |
| `oldXMenu/Activate.rs` (this revision) | 926 | `bfbfa03d427a24de677a03c0031a1f66` | `be50fdebdc93cc80d93cdd3069b37ac86259b021bbae8d38dff91f85b83166a1` |
| `build/femacs/Activate.o` (frozen reading) | - | `dbed68837694579185377dbb0821ab62` | - |
| `/tmp/activate-lane/activate.rlib` (arm reading, take 3) | - | - | `6654b607159adc008672cd1766eb76f737b6c66645ee56a785e03eaed63f5f47` (246,360 bytes) |

## Functions defined — 3

- [x] XMenuActivateSetWaitFunction  (line 126)
- [x] XMenuActivateSetTranslateFunction  (line 133)
- [x] XMenuActivate  (line 139)

## Macros defined — 0

## Types and their fields — 1

- [x] _xmeventque: XEvent event, struct _xmeventque * next  (C 177-180, a `typedef` local to
  `XMenuActivate`; the port declares the same record and its typedef name inside that function)

## File-scope objects — 4

- [x] x_menu_grab_keyboard  (line 120, static)
- [x] wait_func  (line 122, static)
- [x] wait_data  (line 123, static)
- [x] translate_func  (line 124, static)

## Deviations

1. **The file gate is ours, not the reference's**: `oldXMenu/Makefile.in`'s `OBJS` block lists the
   object unconditionally and the configure arm decides whether the library exists (P-86 clause 8;
   `oldXMenu/Internal.rs` records the same reading).
2. **`register` has no Rust counterpart**: C's storage hint is dropped and the three declared types
   are kept (PORTING.md 1.2).
3. **C 1-115 is not transcribed**: the two notices and the file's description block - PORTING.md
   0.1-19, which the P-86 class keeps as the `// femacs: port of …` first line.
4. **C's uninitialised locals are zero-initialised** (PORTING.md 1.5): `status`, `orig_x`, `orig_y`,
   `ret_val`, `root`, `child`, `root_x`, `root_y`, `win_x`, `win_y`, `mask`, `keysym` and the two
   `XEvent`s C leaves for `XNextEvent`/`XQueryPointer` to write.  The cost is the `unused_assignments`
   class of warnings, which the unit's `#![allow (…)]` covers; the values are written before any read
   in every path, as in C.
5. **`QLength` is a macro over Xlib's private display record**: its body is
   `((_XPrivDisplay)(dpy))->qlen` (Xlib.h 96) and no mirror here carries `_XPrivDisplay`, so the port
   calls `XQLength` (Xlib.h 2926), the entry point that reads the same field and is measured present
   in `libX11.6.dylib`.  This is `oldXMenu/Internal.rs` 246-251's reading of `DefaultScreen`/
   `DisplayWidth`/`DisplayHeight`, and the token is one of the `macro-body-notes.tsv` row's.
6. **C's `switch`es are `match`es, and C's `break`s are the arm ends**: C 322-534's dispatch is a
   `match` on the `XEvent` type with `_` for `default:`, and each `break;` is the end of its arm
   (PORTING.md 2.2; the token row).  Two consequences:
   - C 356-445's five inner `break;` (out of the `switch`) are `break 'arm;` out of a labelled block,
     the `src/alloc.rs`/`src/androidmenu.rs` spelling (1154 uses in this tree).
   - C 529-533's `case GenericEvent:` sits *after* its `default:` textually; Rust's `match` reaches the
     first matching arm, so that arm is placed before the `_` arm.  It is `#[cfg]`-gated, so the
     frozen reading is unaltered and every reading taken here has it removed.
   - C 608's `FALLTHROUGH;` into the `default:` of the second dispatch is the merged
     `Expose | EnterNotify | LeaveNotify | ButtonPress | ButtonRelease | _` arm (the case names stay
     in the pattern, and C's `continue` is Rust's `continue`).
7. **`event.type` and the four member records**: `include/xlib.rs`'s `XEvent` is the measured
   192-byte record with an opaque body, so the port reads `event.type` through the `XAnyEvent` view of
   the same storage and each arm's member through its own view (`xexpose`, `xcrossing`, `xbutton`,
   `xkey`), bound to a local named after C's member - P-72 clause 3, the `src/xmenu.rs` 1163-1172 and
   `src/xsettings.rs` 1788-1792 shape.  The five records are declared in this unit with C's own text
   and a measured offset per field, because no mirror carries them (`## Owed` 2).
8. **The unit takes its Xlib names from `xlib_surface`**: `include/xlib.rs` ends with
   `use xlib_surface::*;`, but that is a *private* import of whatever module the mirror expands into,
   and the include-graph dedupe (PORTING.md 0.1-4) expands the mirror inside `mod xmenu_int` - the
   crate-root `include!` the P-86 shape prescribes is the later reach and is replaced by
   `// [xtask dedupe] … already included` + `const _: () = ();`.  A private `use` is not re-exported
   by the `use xmenu_int::*;` glob, so the six names this unit spells (`Cursor`, `Display`, `KeySym`,
   `Pixmap`, `Window`, `XEvent`) are imported from `xlib_surface::{…}` - PORTING.md 0.1-9's rule
   (take the names you need from the module), the shape `oldXMenu/Internal.rs` 60-63 uses for
   `x11_constants`.  `## Owed` 4 names the second half of this: `oldXMenu/Internal.rs`'s own
   crate-root include is now in the same position and its arm reading is stale.
9. **`XFontStruct` is declared in this unit** (P-88 ①): `XMenu.h` 274/288's `*mut XFontStruct` is the
   one name `include/xlib.rs` still lacks, and the include is reached as a *mirror* (so the
   declaration has to sit at this crate root for `mod xmenu_int`'s `use super::*;` to see it).  The
   port declares the tag opaquely - the `lwlib/lwlib-Xaw.rs` 153-158 spelling for the same name,
   sufficient because only the pointer is spelled - and `## Owed` 2 records that the *measured* record
   (`XFontStruct` 96 bytes with `XCharStruct`/`XFontProp`) is the one `oldXMenu/Internal.rs` 105-183
   already carries and that the mirror's move-back should take.
10. **`malloc`/`free` come from the `<stdlib.h>` mirror**: C reaches them through `XMenu.h` 37's
    `#include <stdlib.h>`, which that header's mirror deliberately does not pull in because the header
    itself uses nothing from it, so this unit includes `lib/include/stdlib.rs` at that edge's
    position.  `c_double` joins the unit's `use std::ffi::{…}` set for that mirror's `strtod` alone
    (the `lib-src/hexl.rs` 23 and `exec/exec1.rs` 21 sets carry it for the same reason).
11. **`help_callback` is an `Option`** (P-27: a nullable C function pointer is an `Option` of an
    `extern "C"` fn), so C's two direct calls are `(help_callback.unwrap_unchecked ()) (…)` - the
    shape `include/xlib.rs`'s `XGetPixel`/`XPutPixel` bodies use.  C's declaration gives it no null
    test either, so the two readings agree on the contract.
12. **The `#if 1` block at C 429-442 is carried live** rather than behind a `cfg`: nothing turns it off
    in any configuration (P-86 clause 1's "both arms" has an empty `#else` here).

## Owed

- **`docs/ports/line-drift-tolerance.tsv`**: the `Activate` row for the line-count drift, which
  P-86 clause 10's second reading prints and which is not the porter's row to write.  Measured value
  **44.2%**, so the row is `0.45` (`max (0.05, tolerance)`, the peers' margin convention); the reason
  text the row needs:

  ```
  Activate	0.45	C 642 vs the port's 926 (44.2%, the measured value + 0.01): the whole file is transcribed (the three definitions, both `switch`es as `match`es with the C 608 fallthrough merged into the `default` arm), and the difference is the port's fixed costs - the `have_x_windows` gate on every item and on the four include edges (P-86 (8)), the in-unit `<X11/*>` surface this unit is the directory's first consumer of (five `XEvent` member records with a measured offset per field, ten event/mask constants, eighteen entry points, `Time`/`True`/`False`, each with a citation line; P-88 (1)), the `xlib_surface` import the include dedupe forces, the twelve zero-initialised locals (PORTING.md 1.5), the pointer views of `XEvent`, and C's comment blocks carried as prose	total
  ```
- **`include/xlib.rs`**: the five `XEvent` member records (`XAnyEvent`, `XKeyEvent`, `XButtonEvent`,
  `XCrossingEvent`, `XExposeEvent`), `Time`, `True`/`False`, the ten event/mask/keysym constants and
  the eighteen entry points this unit declares, plus `XFontStruct` (the measured record is in
  `oldXMenu/Internal.rs` 105-183) - all with the measured layouts of `## Evidence` 3.  The unit is
  their consumer today (P-90 clause 1), and their home is that mirror's `<X11/Xlib.h>` section
  (xorgproto 2025.1 / libX11 1.8.13).
- **`docs/port-map.md` and `docs/baseline/port-lines-pending.tsv`**: this file's map row still reads
  3 port lines and the ratchet table still carries its row; `tools/gen-port-map.py` and
  `tools/check-port-lines.py --write-pending` are the generators and both are the integrator's step
  (P-86 clause 4 - neither file is in this lane's write scope).  The map's *reference* count (642) is
  what `check-dead-file-coverage.py` reads and is unaffected.
- **`xtask/Cargo.toml`**: nothing - `have_x_windows` and `have_xinput2` are both already
  registered in the generated `cfg` namespace.  Recorded so the absence is a reading, not an
  omission.
- **A finding about `oldXMenu/Internal.rs`'s arm reading (integrator)**: `oldXMenu/include/
  XMenuInt.rs` 40's `include!("../../src/include/xlib.rs")` edge is reached first, because every
  oldXMenu unit's own `include!` of `xlib.rs` carries `#[cfg (feature = "have_x_windows")]` (so
  `xtask`'s `ungated_reachable` sees no ungated edge and the P-89 relocation does not apply).  The
  net is that only a mirror's expansion is live, and a *private* `use xlib_surface::*;` is not
  re-exported by `use xmenu_int::*;` - measured directly: `mod m { pub mod s { pub type W = u64; }
  use s::*; } use m::*; fn f (w: W) {}` fails with `E0425 cannot find type W`, while
  `use s::W;` after the glob succeeds.  `oldXMenu/Internal.rs`'s own declarations spell bare
  `Window`/`Pixmap`/`Cursor`/`Colormap`/`Display`/`GC`/`Drawable`/`Visual`/`XEvent` at its crate root,
  so its recorded arm reading (Evidence 2 there: "0 errors") predates XMenuInt.rs's edge and its
  current reading is expected to carry that class of errors; `oldXMenu/Internal.md`'s `## Owed` 2 (the
  `pub mod xmenu_int_decls` wrapper) is the same fix from the mirror side.  This lane did not edit
  that file (`emacs/**` and other lanes' files are out of its scope).

## Not claimed

- **Nothing here runs**: the frozen configuration compiles none of the file, this machine has no X11
  program, and the arm flip is a *type check* (Evidence 2, take 3) - not a menu that was drawn, a
  window that was mapped, a pointer that was grabbed or a key that was read.  The three functions
  have no behavioural evidence at all.
- **The `#ifdef HAVE_XINPUT2` arm is not compiled by any reading taken here**: the probe flips the
  file gate alone, so C 529-533's `translate_func` call has no arm reading and `GenericEvent`'s
  placement (Deviation 6) is a text-level judgement.
- **The X11 layout numbers are this machine's X11, not the frozen target's**: measured against
  `/opt/homebrew/include` (xorgproto 2025.1, libX11 1.8.13) rather than a target build, and the six
  records are checked only against that reading.
- **The C 1-114 block is not carried** and this ledger records it by range rather than by text
  (PORTING.md 0.1-19); what the block *says* is therefore not part of the port's acceptance.
- **The five `FALLTHROUGH`-merged cases have no separate reading**: C 587-608's case list is one
  `match` arm here, so "which event fell into which body" is a text-level claim (the case names are
  in the pattern).
- **`docs/ports/Activate.md` was the generated P-86 checklist** (0 items, "the port is an empty unit
  plus the note that names the gate") until this round; the sections above replace it, the
  `oldXMenu-SetAEQ`-class convention.
