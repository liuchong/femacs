# emacs/src/w32gui.h (115 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 39
- [x] AllValues  (line 78, body 652902e8f0f1)
- [x] CONVERT_FROM_EMACS_RECT  (line 102, body 15595bc8aee4)
- [x] CONVERT_TO_EMACS_RECT  (line 96, body 51a1392cc194)
- [x] CenterGravity  (line 66, body ac3478d69a3c)
- [x] EMACS_W32GUI_H  (line 20, body da39a3ee5e6b)
- [x] EastGravity  (line 67, body c1dfd96eea8c)
- [x] FACE_DEFAULT  (line 55, body ea8c29ed8cbf)
- [x] ForgetGravity  (line 61, body b6589fc6ab0d)
- [x] HeightValue  (line 77, body 92df71781c5c)
- [x] NativeRectangle  (line 94, body 75e1bc97e17f)
- [x] NoValue  (line 73, body bd0d3415ccf6)
- [x] NorthEastGravity  (line 64, body 77de68daecd8)
- [x] NorthGravity  (line 63, body da4b9237bacc)
- [x] NorthWestGravity  (line 62, body 356a192b7913)
- [x] PAspect  (line 90, body f5a08b8b3015)
- [x] PBaseSize  (line 91, body 08b008cb3ba4)
- [x] PMaxSize  (line 88, body 2a0a8ece4dfd)
- [x] PMinSize  (line 87, body 7f2dac477a2d)
- [x] PPosition  (line 85, body b56fff76c324)
- [x] PResizeInc  (line 89, body b307cdcc94a2)
- [x] PSize  (line 86, body a33785b93413)
- [x] PWinGravity  (line 92, body 148030b1dc2b)
- [x] STORE_NATIVE_RECT  (line 108, body 4e4a3d496696)
- [x] SouthEastGravity  (line 70, body 0ade7c2cf97f)
- [x] SouthGravity  (line 69, body fe5dbbcea5ce)
- [x] SouthWestGravity  (line 68, body 902ba3cda188)
- [x] StaticGravity  (line 71, body b1d5781111d8)
- [x] USPosition  (line 82, body 1e2ed944d274)
- [x] USSize  (line 83, body 72fd47b601c4)
- [x] WINDOW_HANDLE_UINTPTR  (line 36, body 8dde79acf9e5)
- [x] WestGravity  (line 65, body 1b6453892473)
- [x] WidthValue  (line 76, body eeed498aee81)
- [x] XNegative  (line 79, body d73afeb43f0b)
- [x] XValue  (line 74, body f2b1649be8f5)
- [x] YNegative  (line 80, body 19fbaf699b05)
- [x] YValue  (line 75, body 5f399b2ba212)
- [x] local_alloc  (line 27, body 6a50d2b137ec)
- [x] local_free  (line 28, body 1de34554d409)
- [x] local_heap  (line 26, body f45b7efa690e)

## Types and their fields — 1
- [x] _XImage: unsigned char* data, BITMAPINFO info

## File-scope objects — 1
- [x] extern HINSTANCE hinst  (line 57)

## Decisions

- **The header is unconditional, and nothing on this target compiles it.**  C 19-20 is the guard,
  C 21/C 23 are `#include`s, and the rest is live text - no `#if` anywhere
  (`tools/check-cond-eval.py src/w32gui.h` -> `1 conditions, 0 unknown, 0 disagreements
  [VERIFIED]`, the guard).  The frozen configuration's `WINDOW_SYSTEM_OBJ` is empty
  (docs/baseline/frozen-build-sources.txt; docs/status.md 3.31), so no unit `include!`s the mirror
  and the declarations are deferred to the Windows target.
- **Every `<windows.h>` handle is `*mut c_void`, with C's text in a `/// C:` line.**  `HBITMAP`,
  `HWND`, `HDC`, `HCURSOR` (C 30-34) and `HINSTANCE` (C 57) are all `void *` at the ABI, which is
  how the port's other platform mirrors spell their own handle typedefs (`haikugui.rs` 41-44,
  `androidgui.rs` 45-53).  C 33's `HDC` spelling is kept in the comment because it is the reason
  this header's `Display` is not the X11 one; C 33's `Emacs_Cursor` is the one name with a `cfg`
  (below).
- **`Emacs_Cursor` carries `#[cfg (feature = "have_window_system")]`.**  `dispextern.h` 175 declares
  the same name on the other side of `#ifndef HAVE_WINDOW_SYSTEM`, and `dispextern.rs` 62 carries
  that arm (the frozen target has no window system).  Compiled into one crate the two aliases are
  two items and the crate reports `E0428` - measured when `xtask layout-check` includes every
  mirror at once.  C's own guards keep the two apart, and the port spells the window-system side
  with the feature that names the non-frozen configuration (PORTING.md P-51).  With the feature
  unset the layout crate holds one `Emacs_Cursor`; the `androidterm.rs` 35-37 note records the
  same interaction from Android's end.
- **C 26-28's three macros name `<windows.h>`'s `GetProcessHeap`/`HeapAlloc`/`HeapFree` and the
  `HEAP_ZERO_MEMORY` flag without declaring them.**  A `macro_rules!` body is not resolved until
  it is expanded, so the mirror needs no invented declaration, and a Windows unit takes the three
  from that target (PORTING.md 0.1-9/P-72; the `nt/inc/include/ms-w32.rs` convention for the
  platform's own names).  `LPVOID` in `local_free`'s body is the same reading: C's cast is kept as
  `as LPVOID`.
- **C 39-44's `XImage`/`_XImage` record is deferred.**  C embeds `<wingdi.h>`'s `BITMAPINFO`
  **by value**, the port has no mirror of that header, and no compiler on this target could
  measure the record - so the mirror carries no invented layout and the checklist box stays
  unchecked.  The extractor reports the gap on stderr as `missing type deferred: src/w32gui.h
  XImage/_XImage (the ledger has unchecked boxes)`; that note, not a `- ` problem, is what the
  unchecked box is for.
- **C 46's `struct image;` gets no item.**  It is a forward declaration of `dispextern.h` 3215's
  record, which this header does not include; the port writes no item for a tag it does not define
  (the `haikuterm.rs` reading of `struct haiku_font_change_event;`), and the two pointers in C 47
  are `*mut c_void`.  `Lisp_Object` stays named (lisp.h is every unit's include, PORTING.md 0.1-9).
- **The X-compat block keeps C's spellings and order**: the eleven `*Gravity` (C 61-71) and eight
  `*Value` (C 73-80) constants as `c_int`, the ten `*Position` flags (C 82-92) as `c_long` with
  C's `1L << n` token pair in the value (the `haikugui.rs` 126-134 spelling of the same block),
  `FACE_DEFAULT` as `!0` for `(~0)` (C 55), and `NativeRectangle` (C 94) as a nullary
  `macro_rules!` because C uses the name in type position (the `haikugui.rs` 63 / `androidgui.rs`
  241 spelling).  The three rectangle conversions (C 96-112) are blocks of the same assignments -
  Rust has no comma operator.
- **`--check` asks for less than fidelity does**: C 46-57 are declarations with no checklist row,
  and the four typedefs of C 30-34 / C 94's macro are carried because they are live text.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/w32gui.h femacs/src/include/w32gui.rs`
  -> `OK emacs/src/w32gui.h -> femacs/src/include/w32gui.rs` (no `- ` problem lines), with the
  single stderr note `missing type deferred: src/w32gui.h XImage/_XImage (the ledger has unchecked
  boxes)` - the C 39-44 deferral recorded above.  The first attempt reported one problem,
  `macro local_free does not mention lpvoid (C body: (HeapFree (local_heap, 0, ((LPVOID) (p)))))`,
  which is why C's cast is written `as LPVOID` in the body.
- `python3 tools/check-cond-eval.py src/w32gui.h` -> `1 conditions, 0 unknown to the evaluator,
  0 disagreements` (the guard).
- `python3 tools/extract-abi.py emacs/src/w32gui.h` -> this ledger: 0 subrs, 0 functions defined,
  39 macros, 1 type (`_XImage`, 2 fields), 1 file-scope object (`hinst`).
- **Compile evidence**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask -- layout-check`
  -> `layout-check: 66 of 77 mirror(s) compiled, 11 platform-exclusive skipped (android-asset.rs,
  android.rs, androidgui.rs, androidterm.rs, dispextern.rs, gtkutil.rs, haiku_support.rs,
  haikugui.rs, haikuterm.rs, nsgui.rs, pgtkterm.rs)`, `layout probe ok`,
  `layout-check passed (see build/femacs/layout_asserts.rs)`.  **This mirror is among the 66** (it
  is not in that skip list), so the gate compiles it every round.  An earlier run of the same gate,
  with only `cygw32.rs` of this batch present, printed `layout probe ok`; a later one failed with
  `error[E0428]: the name Emacs_Cursor is defined multiple times … nsgui.rs:57:1 …
  dispextern.rs:62:1` - two mirrors of another lane, both of which had entered the skip list by the
  time of the passing run, and no diagnostic of either run named a mirror of this batch.  Its `Emacs_Cursor`, `Window` and `Display` sit behind the `have_window_system` feature, so the crate holds no duplicate of the other platform mirrors' names.

- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 566 .rs files / 593 map targets / 0 findings.

## Decisions (stale-box sweep: **not stale**, 2026-10-04)

The 1 box(es) this sweep tentatively ticked are **real pending work, not stale bookkeeping**,
and the tick was reverted.  Measured: while the ledger has unchecked boxes,
`python3 tools/extract-abi.py --check emacs/src/w32gui.h femacs/src/include/w32gui.rs` prints `0` `- ` lines **because the tool
defers this file's missing items to stderr** (`missing type deferred: src/w32gui.h <name> (the ledger has
unchecked boxes)`); with the boxes ticked the deferral lifts and the same command prints

    - missing type XImage/_XImage

i.e. the type is genuinely absent from femacs/src/include/w32gui.rs.  So: src/w32gui.h's `XImage`/`_XImage` is the Windows GDI record; dead-platform text (no `_WIN32`), P-51/3.9 shape.

## Decisions / Evidence (the eight true gaps, 2026-10-04)

`XImage`/`_XImage` (C 38-44) landed as dead platform text under `windows` (P-51/3.9).  The other boxes this ledger lists were already satisfied and were ticked in the same
pass (they had been left unchecked, and the earlier sweep had un-ticked some of them by
position rather than by name - corrected here).

    $ python3 tools/extract-abi.py --check emacs/src/w32gui.h femacs/src/include/w32gui.rs
    OK emacs/src/w32gui.h -> femacs/src/include/w32gui.rs                     # `- ` lines: 0, no `missing type deferred`
    $ cargo run -q -p xtask -- layout-check
    layout-check passed (see build/femacs/layout_asserts.rs)

(**before** this pass: 1 unchecked box(es); **after**: 0)

# ----------------------------------------------------------------------
## `XImage`'s gate: `windows` becomes `have_ntgui` (2026-10-05, `docs/ports/w32fns.md` owed 8's neighbour)

C 38-44's record was carried under `#[cfg (feature = "windows")]` - the `WINDOWS` macro, a name
this header never asks about.  The file has no `#if` of its own, and its only two includers are
`dispextern.h` 135 and `w32term.h` 21, both inside a `HAVE_NTGUI` guard; the item is therefore
reachable **only** in a Windows GUI build, and that is the gate it now carries.  Consequence,
stated: under the frozen configuration nothing changes (the item is still compiled out), and the
`have_ntgui` arm is the one place a consumer can read it - `docs/ports/w32fns.md`'s
`w32_create_pixmap_from_bitmap_data` call chain is that consumer.

**Evidence.** `python3 tools/extract-abi.py --check emacs/src/w32gui.h
femacs/src/include/w32gui.rs` -> `OK emacs/src/w32gui.h -> femacs/src/include/w32gui.rs`;
`python3 tools/check-mirror-dupes.py` -> `18 duplicate declaration(s), 18 ratchet row(s),
0 failure(s)`.

# ----------------------------------------------------------------------
## The six declarations move into modules, and C's two casts are written out (2026-10-05)

**C 46-57.**  The header's six prototypes/objects are definitions in the Windows GUI units
(`w32image.c` implements the image entry points; `w32fns.c` defines C 57's `hinst`), so a
crate-root declaration here collides with the unit's own item - measured for `hinst` by
`xtask probe src/w32fns.rs … --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'`
(`docs/ports/w32fns.md` owed 5).  They now sit in `pub mod w32gui_fn_decls` and
`pub mod w32gui_objs` with `use …::*;` after each (the `window.rs` 302-367 shape, PORTING.md
0.1-10 / P-21).  The block has no `#if` of its own, so neither module carries a `cfg`.

**C 96-112's two conversions.**  C's `CONVERT_TO_EMACS_RECT` assigns `(nr).right - (nr).left`
(`LONG`, this target's `i32`) to `(xr).width` (`unsigned`, `c_uint`), and
`CONVERT_FROM_EMACS_RECT` assigns `(xr).x + (xr).width` (`int + unsigned`) to `LONG`.  Rust
refuses both implicit forms, so each keeps C's own conversion: `as c_uint` on the way in and
`(x as c_uint).wrapping_add (width) as c_int` on the way out (C computes the sum in `unsigned`
and converts the result).  Measured before the change: six `E0308`/`E0277` sites in
`include_w32gui_rs.rs` under the `have_ntgui` + `windowsnt` probe; gone after.
`STORE_NATIVE_RECT` needed nothing (the same types on both sides).

**Evidence.**  `python3 tools/extract-abi.py --check emacs/src/w32gui.h
femacs/src/include/w32gui.rs` -> `OK` (its one stderr note, `NativeRectangle`'s recorded Rust
counterpart, is unchanged); `xtask layout-check` passed; `check-citations` 0 drift;
`check-mirror-dupes` 18 ratchet rows / 0 failure for this file; `check-all-units.sh` 83/0 both legs.
