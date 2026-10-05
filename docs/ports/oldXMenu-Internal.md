# emacs/oldXMenu/Internal.c (1023 lines)

## Decisions

`emacs/oldXMenu/Internal.c` @ `fd7afced403`; port `femacs/oldXMenu/Internal.rs` (**1163 lines**);
the gate is `feature = "have_x_windows"` (P-86 clause 8), which is the build's condition rather
than one the C text spells: `configure.ac` 7287-7295 sets `HAVE_OLDXMENU=yes` and
`LIBXMENU='$(oldXMenudir)/libXMenu11.a'` in the arm `HAVE_X11 = yes` + `HAVE_GTK != yes` +
`USE_X_TOOLKIT = none`, and `oldXMenu/Makefile.in`'s `OBJS` block (60-85) lists every object of the
directory unconditionally.  **The reference has no macro of its own for this file**, so P-86
clause 8's "the reference's own macro, lower-cased" has no subject here (Deviation 1 names the
resolution: the repo's feature for the X11 window system, `have_x_windows`, the name
`docs/baseline/config/darwin-arm64.macros.tsv` records as `undef` and `src/xfns.rs`'s unit uses).

- **The file is unconditional text**: `grep -n '^#' emacs/oldXMenu/Internal.c` finds only C 50's
  `#include` and C 55/56's two `#define`s.  There is no `#if`/`#ifdef` in it, so P-86 clause 8's
  per-arm rule has no arm to carry and the whole body sits under the one file gate - on every item
  and on the include graph (P-86 clause 8's own judgement: the frozen build leaves the 336-byte
  empty object, Evidence 1).
- **The frozen configuration builds none of it** and this machine has no X11 *program* to run it
  against: the file is `text-only` in P-86 clause 3(3)'s second sense - the arm **type-checks**
  (Evidence 2, 0 errors) and nothing here executes.
- **`<X11/*>` is measured, not remembered**: the X11 names `include/xlib.rs` does not carry are
  declared in the unit at their use site (P-88 clause 1) and every value, size and offset comes
  from a clang reading of `/opt/homebrew/include` (xorgproto 2025.1, libX11 1.8.13; Evidence 4).
- **The three Xlib macros** C 530-536 and 675-676 call (`DefaultScreen`, `DisplayWidth`,
  `DisplayHeight`) have no symbol: their bodies read Xlib's private display record
  (`_XPrivDisplay`, Xlib.h 85-119), which no mirror here carries.  The port calls the three entry
  points that read the same fields (`XDefaultScreen`/`XDisplayWidth`/`XDisplayHeight`, all three
  measured present in `libX11.6.dylib`), the reading `include/xlib.rs` already takes for
  `DefaultScreenOfDisplay` (Deviation 7; xlib.rs's own note is the precedent).
- **`config.rs` is not included, and the unit brings its own type-alias set**: the C text reaches
  `<config.h>` only through `XMenuInt.h` 38, and that mirror's own note records that the header
  uses nothing from it; nothing here names a frozen configuration constant (the gate is a `cfg`
  feature, not a `config.rs` value, unlike the `HAVE_X_WINDOWS` constant of the same name).  The
  C fixed-width spellings (`c_int`, `c_char`, …) therefore come from the unit's own
  `use std::ffi::{…}` line - the shape `lib-src` and `exec` units use - where every unit that
  includes `lisp.h` takes them from that mirror's crate-root list instead; `include/xlib.rs` reads
  the same set at the crate root of whatever unit includes it (its `xlib_surface` module's
  `c_uchar` is what made the omission visible: the arm fails with `E0425` without it).
- The unit's own `C <line>` notes are the per-block map; the table below is the same map at block
  granularity, and `docs/port-map.md` binds this ledger to `oldXMenu/Internal.c` alone.

## C line ranges

The rows are contiguous and ascending; C's blank lines between the blocks need no row (the gate
`tools/check-dead-file-coverage.py` reads it that way).

| C | block | arm |
| --- | --- | --- |
| 1-49 | the file's opening block (the two notices and the XMenu description comment) | dropped (PORTING.md 0.1-19, P-86 clause 1) |
| 50 | `#include "XMenuInt.h"` | file gate; the mirror is included inside `mod xmenu_int` and glob-imported (Deviation 2) |
| 52-56 | the queue-size comment and `#define S_QUE_SIZE 300` / `#define P_QUE_SIZE 20` | file gate |
| 58-73 | `typedef struct _xmwinquedef { ... } XMWinQue;` | file gate |
| 74-83 | `_XMWinQueIsInit` (static), `_XMWinQue` (static), `_XMErrorCode` | file gate |
| 84-103 | `_XMErrorList` - the seventeen description strings | file gate (Deviation 3) |
| 105-108 | `_XMEventHandler` | file gate |
| 110-142 | `_XMWinQueInit` | file gate |
| 144-176 | `_XMWinQueAddPane` | file gate |
| 178-210 | `_XMWinQueAddSelection` | file gate |
| 212-350 | `_XMWinQueFlush` | file gate (both queue arms, the two pane loops and the selection loop) |
| 352-385 | `_XMGetPanePtr` | file gate |
| 387-421 | `_XMGetSelectionPtr` | file gate |
| 423-537 | `_XMRecomputeGlobals` | file gate (the `max` loops and the screen clamp) |
| 539-683 | `_XMRecomputePane` | file gate; C 566-583 and 656-671 are C `switch`es over `menu_style`/`p_style` and become `match` arms (`docs/ports/macro-body-notes.tsv`) |
| 685-805 | `_XMRecomputeSelection` | file gate; C 779-794 is a C `switch` over `s_style` and becomes `match` (`docs/ports/macro-body-notes.tsv`) |
| 807-873 | `_XMTransToOrigin` | file gate |
| 875-938 | `_XMRefreshPane` | file gate |
| 940-1023 | `_XMRefreshSelection` | file gate |

## Findings

1. **The queue is two fixed-size arrays inside one record** (`XMWinQue.sq`/`pq`) with a
   "next free slot" pointer each, and the flush routine walks the pane queue **twice** - downwards
   from the last slot to the current pane and then upwards from slot 0 to it, breaking at the pane
   - so the current pane ends up on top of the stacking order (C 245-291).  Both walks carry the
   same `XCreateWindow`/`XMakeAssoc`/`XSelectInput` triple; the first one breaks *before* the
   increment, which is why the port's descending loop `break`s before its `pq_index -= 1`.
2. **`_XMWinQueFlush`'s last two arguments are both pointers C passes as `0`** (C 160, 194:
   `_XMWinQueFlush (display, menu, 0, 0)`), and the C 212-350 body tests them against `pane` only -
   so a `0` pane means "the caller has no pane to stop at" and the loops run to the end.
3. **The pane/selection serial number doubles as the layout dirty flag**: `_XMRecomputePane` and
   `_XMRecomputeSelection` write the new serial first and then compare coordinates/dimensions,
   which is how a freshly created (serial `-1`) window gets its coordinates (C 701-709's own
   comment).
4. **The screen clamp** (C 526-536) reads `DisplayWidth`/`DisplayHeight` twice per axis (once in
   the condition, once in the assignment), and only clamps to the *lower* bound when the position
   is negative - the reference's own text, carried as it stands.
5. **One attribute record serves the whole flush**: C 229-230 takes the address of a stack
   `XSetWindowAttributes`, C 240-243 fills three of its members once, and every window of both
   loops is created from the same pointer - the selection windows with a value mask of `0`
   (C 321), so for them nothing in the record is read; only the panes' `XCreateWindow` calls pass
   the mask the three assignments build.

## Evidence

1. **frozen reading** (the file gate off): `cd femacs && CARGO_TARGET_DIR=/tmp/oxm-lane
   cargo run -q -p xtask -- build-tu oldXMenu/Internal.c` -> **exit 0**,
   `build/femacs/Internal.o` **336 bytes**, `sha256
   d1ea88b2fe3d960975d134e566e6593217e9ad67abdc2db5a451ea9620edd0ed` - the empty object the gate
   leaves (the same reading `docs/ports/lwlib-Xlw.md` records for its file).
2. **arm flip** (the final revision, `sha256
   1f2ff321c009a63abbf92ddecb003c55cc9b4221c696a3fcd91cb8b5633fa099`):
   `cargo run -q -p xtask -- probe oldXMenu/Internal.rs /tmp/oxm-lane/x.rlib --crate-type=lib
   -C panic=abort --cfg 'feature="have_x_windows"'` -> **exit 0, 0 errors, 85 warnings, a
   274,064-byte rlib** (`sha256 20c559cd674a875b6fc6f77fe1ebf8cfcdb3549c80e66f4a6a64f79ec5dc9db0`).
   The file has no sub-gate (no `#if`), so the file gate is the whole set P-86 clause 8(6) asks
   for.  The 85 warnings (the run's own summary count) are `improper_ctypes`/`dead_code` classes:
   the opaque `_XDisplay`/`_XGC` records and the unused Xlib entry points `include/xlib.rs`
   carries.  The lane's budget is three `probe` runs and this file used **four**: one on the first
   working text, one on the clean revision, one for the mutation, and one after the self-review fix
   `## Findings` 6 records (the fix landed after the clean run, so the shipped revision needed its
   own reading).
3. **mutation reading** (P-86 clause 7's judgement: an arm flip whose include graph is broken
   reports "0 errors of this unit" vacuously).  With `let mut i: c_int = c"x".as_ptr ();` injected
   at the unit's line 413 (marker comment `/* MUTATION (probe 3) */`) the *same* command gives
   **exit 1** and exactly **one** error - `error[E0308]: mismatched types` naming
   `oldXMenu/Internal.rs:413` - so the reading above is the unit's own text and not a mirror's
   failure mode.  That run was taken on the revision before the self-review fix (Findings 6,
   `sha256 be3c5a98e734b44e7304de8fb58284489bd1649d59b90e3d5164e74ea99ea7fb`); the same one-line
   mutation sits at the same unit line on the final revision, and re-running it there with the same
   rustc command line on a locally materialised copy gives the same single `E0308` at
   `Internal.rs:413`.  Both times the file was reverted and its hash re-checked.
4. **the X11 surface is measured, not remembered** - `clang -I/opt/homebrew/include` on this
   machine (Apple clang 21.0.0.0, target arm64-apple-darwin25.6.0, xorgproto 2025.1, libX11
   1.8.13):
   - `XSetWindowAttributes` `[sizeof=112, align=8]`, `background_pixmap` 0, `border_pixmap` 16,
     `border_pixel` 24, `bit_gravity` 32, `event_mask` 72, `override_redirect` 88, `colormap` 96,
     `cursor` 104 - the three fields C 241-243 writes and the two the flush call passes.
   - `XWindowChanges` `[sizeof=40, align=8]`, `x` 0, `y` 4, `width` 8, `height` 12,
     `border_width` 16, `sibling` 24, `stack_mode` 32.
   - `XFontStruct` `[sizeof=96, align=8]`, `fid` 8, `direction` 16, `all_chars_exist` 36,
     `properties` 48, `min_bounds` 56, `max_bounds` **68**, `per_char` 80, `ascent` 88,
     `descent` 92; `XCharStruct` `[sizeof=12, align=2]`, `lbearing` 0, `width` 4, `ascent` 6,
     `descent` 8, `attributes` 10; `XFontProp` `[sizeof=16, align=8]`, `name` 0, `card32` 8.
   - the constants: `True` 1, `False` 0, `InputOutput` 1, `InputOnly` 2, `CopyFromParent` 0,
     `CWBackPixmap` 1, `CWBorderPixel` 8, `CWX` 1, `CWY` 2, `CWWidth` 4, `CWHeight` 8,
     `CWOverrideRedirect` 512 (the last is `include/xlib.rs`'s row).
   - `XEvent` is 192 bytes and `Display*`/`GC`/`Window`/`XID` are 8 (the mirror's own rows).
   - the entry points the unit declares: `nm -gU /opt/homebrew/lib/libX11.6.dylib` shows
     `_XCreateWindow`, `_XSelectInput`, `_XFlush`, `_XConfigureWindow`, `_XClearWindow`,
     `_XFillRectangle`, `_XDrawString`, `_XDrawLine`, `_XDrawRectangle`, `_XClearArea`,
     `_XDefaultScreen`, `_XDisplayWidth`, `_XDisplayHeight` (and `_XDefaultScreenOfDisplay`).
5. **the four gates this lane owed**:
   - `python3 tools/extract-abi.py --check emacs/oldXMenu/Internal.c
     femacs/oldXMenu/Internal.rs` -> **one finding**: `line count drifted 13.7%: C 1023 vs Rust
     1163` (`## Owed` 1).  Its inventory for this file is empty (P-86 clause 2: the file is not
     built on this target), and the port's own reading is the DEFUN-free comparison: no missing
     function/macro/constant/object/field, no stub and no token missing after the two
     `macro-body-notes.tsv` rows.
   - `python3 tools/check-declarations.py` -> **0 failures**.
   - `python3 tools/check-citations.py` -> **0 range, 0 drift** for this unit; its `Xlib.h`/`X.h`/
     `XMenuInt.h` citations are in the `external` bucket (system headers, counted, never failed).
   - `python3 tools/check-dead-file-coverage.py` -> **every non-blank line of the reference is in
     a row** (the table above): `docs/ports/oldXMenu-Internal.md: 19 range row(s) cover
     oldXMenu/Internal.c (1023 lines; 15 blank line(s) between blocks need no row)`.
   - `python3 tools/check-port-lines.py` -> `stale oldXMenu/Internal.c: no longer under the
     threshold`, **0 new** ratchet rows (the file is now 113% of its reference, so the row
     `docs/baseline/port-lines-pending.tsv` still carries is fixed and `--write-pending` is the
     integrator's step, `## Owed` 4).

## Deviations

1. **The gate has no reference macro** (P-86 clause 8 asks for one lower-cased).  The reference's
   condition lives in `configure.ac` 7287-7295, not in a header, and `oldXMenu/Makefile.in` names
   no macro for any of the directory's objects; the port uses `have_x_windows`, the repo's feature
   for that configure result (`HAVE_X_WINDOWS`, `undef` in the frozen table) and the gate
   `src/xfns.rs` already uses.
2. **C 50's `#include` is wrapped in a module** (`#[cfg (feature = "have_x_windows")] #[macro_use]
   mod xmenu_int { use super::*; include!("include/XMenuInt.rs"); } use xmenu_int::*;`).  The
   mirror `oldXMenu/include/XMenuInt.rs` declares the twelve functions this unit *defines* at its
   top level, and Rust rejects a declaration and a definition of one name in one scope (`E0428`,
   measured); `use super::*` inside the wrapper is what keeps the mirror's own type references
   (`c_int`, `Window`, `XEvent`) resolvable, and `#[macro_use]` is what carries `min`/`max`/`abs`
   across (a glob import does not import `macro_rules!` items).  The prescribed shape is the
   mirror's own `pub mod <name>_decls { use super::*; … } use <name>_decls::*;` (`lwlib-Xaw.rs`,
   PORTING.md 0.1-10 / P-21); moving the mirror's block into one is `## Owed` 2, and the wrapper
   then goes away.
3. **`_XMErrorList` is `pub static mut`** where C's array is `char const *const` (read-only).  A
   non-`mut` Rust `static` holding raw pointers demands `Sync`, which `*const c_char` is not
   (`E0277`, measured); `keyboard.rs`'s `lispy_accent_keys`/`modifier_names` are the same C
   construct and the same reading.  The element type stays C's pointer (not `&CStr`), so a C
   reader of the symbol sees the same layout.
4. **The two records C declares and fills per call are zero-initialised** (`XSetWindowAttributes
   attributes_buf`, `XWindowChanges changes`): C 229 declares `attributes_buf` uninitialised and
   C 632/755 declare `changes` uninitialised, and Rust has no partial initialisation (PORTING.md
   0.1-13).  The three (resp. four) field assignments that follow are C's, in C's order.
5. **C's implicit conversions are spelled out** where Rust has no counterpart: `0` to a pointer at
   the two `_XMWinQueFlush (display, menu, 0, 0)` call sites (`0 as *mut XMPane`,
   `0 as *mut XMSelect` - C's own token, kept), `CopyFromParent` to `int` and to `Visual *`,
   `InputOutput`/`InputOnly` to `unsigned int`, `menu->p_events`/`s_events` (`int`) to `long`,
   `change_mask` (`unsigned long`) to `unsigned int`, the `int` width/height arguments to
   `unsigned int`, and `XCharStruct`'s `short` fields to `int` on the three arithmetic sites
   (C 675-676, 798, 894, 902).  Each is the conversion C performs implicitly.
6. **A self-review finding, fixed after the first clean probe**: C's two blanking loops
   (C 128-129, 131-132) each initialise `i` in their own `for` header, and the port's first version
   carried one `let mut i` through both of its `while` loops - so the second loop would have run
   from `S_QUE_SIZE` and cleared nothing.  The final revision assigns `i = 0;` before the second
   loop (under the C 131-132 note), which is the text probe Evidence 2 compiles.  The
   `extract-abi --check` token comparison cannot see the difference (both spellings mention the
   same tokens), so this reading came from comparing the two texts, not from a gate.
7. **Three C *macros* become three port functions**: `DefaultScreen`, `DisplayWidth` and
   `DisplayHeight` (Xlib.h 89, 97, 98) expand inline over `_XPrivDisplay` and name no symbol, so
   `#[inline(always)] pub unsafe fn` wrappers over `XDefaultScreen`/`XDisplayWidth`/
   `XDisplayHeight` keep C's *call shape* (a `macro_rules!` cannot be called as `f (x)`; the
   `include/xlib.rs` `XGetPixel` note).  Same values, different spelling; `include/xlib.rs`'s
   `DefaultScreenOfDisplay` records the same class of degradation.
8. **The X11 surface this unit needs is declared in the unit** (P-88 clause 1) although P-90 clause
   1 names `include/xlib.rs` as the shared mirror for `<X11/Xlib.h>`: the mirror is included for
   the names it carries (`Display`, `Window`, `GC`, `XID`, `Bool`, `Pixmap`, `Cursor`, `Atom`,
   `Drawable`, `Font`, `Visual`, `XExtData`, `XEvent`, and `x11_constants`'
   `CopyFromParent`/`InputOnly`/`CWOverrideRedirect`), and the names it lacks - `XFontStruct`,
   `XFontProp`, `XCharStruct`, `XSetWindowAttributes`, `XWindowChanges`, `True`/`False`,
   `InputOutput` and the six `CW*` mask bits, and the thirteen entry points - are declared at their
   use site with C's own prototype in a `/// C:` line (P-88 clause 1 / P-55).  `## Owed` 3 names
   the move back into the mirror, the `xfns.rs` precedent.

## Owed

- **`docs/ports/line-drift-tolerance.tsv`**: the `Internal` row for the line-count drift, which
  P-86 clause 10's second reading prints and which is not the porter's row to write.  Measured
  value 13.7%, so the row is `0.15` (`max (0.05, tolerance)`, the peers' margin convention); the
  reason text the row needs:

  ```
  Internal	0.15	C 1023 vs the port's 1163 (13.7%, the measured value + 0.01): the whole file is transcribed (twelve functions, both pane loops and the selection loop of `_XMWinQueFlush`, the two `switch`es as `match` arms), and the difference is the port's fixed costs - the `have_x_windows` gate on every item and on the include graph (P-86 clause 8), the in-unit `<X11/Xlib.h>` surface with a citation and a measured-layout assertion per record (P-88 clause 1), C's implicit pointer/integer conversions spelled out, and C's comment blocks carried as prose under a `C <line>` note	total
  ```

  Every peer X11 P-86 unit carries such a row (`lwlib-Xaw` 0.79, `lwlib-Xlw` 0.80, `xsmfns` 0.80,
  `xfont` 0.49); this one's measured value is the smallest of them because the port compresses
  C's comment blocks to one line each.
- **`oldXMenu/include/XMenuInt.rs`**: the twelve internal-routine prototypes and the
  `_XMEventHandler` declaration need the `pub mod xmenu_int_decls { use super::*; … }
  use xmenu_int_decls::*;` shape (PORTING.md 0.1-10 / P-21, the `lwlib-Xaw.rs` precedent) so the
  unit can include the mirror directly and Deviation 2's wrapper disappears.  The mirror's own
  notes about the X11 names it leaves to the unit (`Display`, `XEvent`, `XID`) stay true either
  way (this unit supplies them through `include/xlib.rs` and its in-unit segment).
- **`include/xlib.rs`**: the five records, the eight constants and the thirteen entry points
  Deviation 8 lists, with the measured layouts of Evidence 4 - the unit is their only consumer
  today (P-90 clause 1's judgement), and their home is that mirror's `<X11/Xlib.h>` section
  (xorgproto 2025.1 / libX11 1.8.13).  `XFontStruct`/`XCharStruct` belong there before any other
  X11 unit needs a font metric.
- **`docs/port-map.md` and `docs/baseline/port-lines-pending.tsv`**: this file's map row still
  reads 3 port lines and the ratchet table still carries its row; `tools/gen-port-map.py` and
  `tools/check-port-lines.py --write-pending` are the generators and both are the integrator's
  step (P-86 clause 4 - neither file is in this lane's write scope).  The map's *reference* count
  (1,023) is what `check-dead-file-coverage.py` reads and is unaffected.
- **`femacs/xtask/Cargo.toml`**: nothing - every feature this unit names (`have_x_windows`) is
  already registered in the generated `cfg` namespace.  Recorded so the absence is a reading
  rather than an omission.
- **`docs/ports/Internal.md`** is the generated P-86 checklist for this reference file and stays
  as the generator wrote it (empty sections + the deferred-declaration sentence).  This ledger is
  the same file's record under the name `check-dead-file-coverage.py` and `check-ledger-ticks.py`
  resolve first (`oldXMenu-Internal.md`, the `<dir>-<stem>` rule).

## Not claimed

- **Nothing here runs**: the frozen configuration compiles none of the file, this machine has no
  X11 program, and the arm flip is a *type check* (Evidence 2) - not a menu that was drawn, a
  window stack that was verified or a cursor that was grabbed.  The five `X*` drawing calls of
  C 875-1023 have no behavioural evidence at all.
- **The X11 layout numbers are this machine's X11, not the frozen target's**: measured against
  `/opt/homebrew/include` (xorgproto 2025.1, libX11 1.8.13) rather than a target build, and the
  three records carried whole (`XFontStruct` with `XFontProp`/`XCharStruct`,
  `XSetWindowAttributes`, `XWindowChanges`) are checked only against that reading.
- **The C 1-49 block is not carried** and this ledger records it by range rather than by text
  (PORTING.md 0.1-19); what the block *says* is therefore not part of the port's acceptance.
- **The stale-looking `docs/ports/Internal.md` is not this file's verdict**: it is the generated
  checklist (0 items, P-86 clause 2) and this ledger is the record.
