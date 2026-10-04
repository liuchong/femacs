# emacs/src/pgtkgui.h (121 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 38
- [x] AllValues  (line 84, body 652902e8f0f1)
- [x] CONVERT_FROM_EMACS_RECT  (line 109, body 38273a6d66a7)
- [x] CONVERT_TO_EMACS_RECT  (line 103, body 4bba530e72d0)
- [x] CenterGravity  (line 72, body ac3478d69a3c)
- [x] EastGravity  (line 73, body c1dfd96eea8c)
- [x] ForgetGravity  (line 67, body b6589fc6ab0d)
- [x] HeightValue  (line 83, body 92df71781c5c)
- [x] NativeRectangle  (line 101, body 5912a6232d22)
- [x] NoValue  (line 79, body bd0d3415ccf6)
- [x] NorthEastGravity  (line 70, body 77de68daecd8)
- [x] NorthGravity  (line 69, body da4b9237bacc)
- [x] NorthWestGravity  (line 68, body 356a192b7913)
- [x] PAspect  (line 96, body f5a08b8b3015)
- [x] PBaseSize  (line 97, body 08b008cb3ba4)
- [x] PMaxSize  (line 94, body 2a0a8ece4dfd)
- [x] PMinSize  (line 93, body 7f2dac477a2d)
- [x] PPosition  (line 91, body b56fff76c324)
- [x] PResizeInc  (line 95, body b307cdcc94a2)
- [x] PSize  (line 92, body a33785b93413)
- [x] PWinGravity  (line 98, body 148030b1dc2b)
- [x] STORE_NATIVE_RECT  (line 115, body cc3225e87153)
- [x] STORE_XCHAR2B  (line 36, body 66bf9d819857)
- [x] SouthEastGravity  (line 76, body 0ade7c2cf97f)
- [x] SouthGravity  (line 75, body fe5dbbcea5ce)
- [x] SouthWestGravity  (line 74, body 902ba3cda188)
- [x] StaticGravity  (line 77, body b1d5781111d8)
- [x] USPosition  (line 88, body 1e2ed944d274)
- [x] USSize  (line 89, body 72fd47b601c4)
- [x] WINDOW_HANDLE_UINTPTR  (line 52, body 8dde79acf9e5)
- [x] WestGravity  (line 71, body 1b6453892473)
- [x] WidthValue  (line 82, body eeed498aee81)
- [x] XCHAR2B_BYTE1  (line 39, body 7dcd93a9afd2)
- [x] XCHAR2B_BYTE2  (line 42, body 97ec1ccb392c)
- [x] XNegative  (line 85, body d73afeb43f0b)
- [x] XValue  (line 80, body f2b1649be8f5)
- [x] YNegative  (line 86, body 19fbaf699b05)
- [x] YValue  (line 81, body 5f399b2ba212)
- [x] __PGTKGUI_H__  (line 20, body da39a3ee5e6b)

## Types and their fields — 2
- [x] <anon@60>: int x, int y, unsigned width, unsigned height
- [x] _XCharStruct: int rbearing, int lbearing, int width, int ascent, int descent

## File-scope objects — 0

## Decisions

- **The header is pgtk-build text, and its guard lives in its includers.**  C 19-121 has no `#if` of
  its own; `dispextern.h` 151-157 (`#ifdef HAVE_PGTK`) and `src/pgtkterm.h` 29 are what decide
  whether it is ever compiled, and `HAVE_PGTK`/`USE_GTK` are `undef` in
  `docs/baseline/config/darwin-arm64.macros.tsv`.  The frozen clang measures the two names C
  repeats: a TU that includes `dispextern.h` and then `pgtkgui.h` fails with
  `typedef redefinition with different types ('struct _GdkCursor *' vs 'void *')` (pgtkgui.h 46
  against dispextern.h 175) and `'NativeRectangle' macro redefined` (101 against dispextern.h 179).
  In a pgtk build `HAVE_PGTK` implies `HAVE_WINDOW_SYSTEM`, so both fallbacks are dead and C 46/101
  win.  No unit here includes this mirror (`rg -n 'include/pgtkgui' femacs` -> no hits) and no
  `src/pgtk*.c` is in the frozen build's source list.
- **Two halves, split by whether another *live* mirror declares the name.**  Live: the guard,
  `_XCharStruct`/`XCharStruct`, `XRectangle`, `unichar`/`XChar2b`, the three `XCHAR2B` macros,
  `Color` and `XrmDatabase` - all plain C text that collides with nothing in this tree.
  `#[cfg (feature = "have_pgtk")]`: `Emacs_Cursor` (also `dispextern.rs` 62), `Window`,
  `WINDOW_HANDLE_UINTPTR`, the eleven gravity, eight value and ten position constants,
  `NativeRectangle` and the three rectangle conversions - `include/w32gui.rs` 57-148 and 68 already
  declare those names live, and C keeps the two apart with `HAVE_NTGUI` versus `HAVE_PGTK` (the
  `include/msdos.rs` 207-228 reading of the same collision).
- **One feature, not two.**  A pgtk build defines *both* `HAVE_PGTK` and `USE_GTK`
  (`configure.ac` 3826 sets `USE_GTK` for every GTK build; pgtk forces `with_gtk3=yes` at 3170-3176),
  and this header's text is unconditional, so a second feature would separate nothing.  `have_pgtk`
  is the macro that names the side whose text this is.
- **`Display` (C 50) is recorded, not mirrored.**  `typedef struct _GdkDisplay Display;` names
  GDK's incomplete record, and Rust cannot alias a name it does not have (measured: `pub type
  Display = _GdkDisplay;` alone is `error[E0425]: cannot find type`).  The pgtk target supplies it -
  the `include/emacsgtkfixed.rs` reading of `EmacsFixedPrivate`, a typedef of an incomplete record
  that no item in the mirror needs.
- **`Emacs_Cursor` keeps P-55's pointer.**  The pointee is GDK's incomplete record with no mirror
  here, so the type is `*mut c_void` with C's declaration in a `/// C:` line, as
  `include/nsgui.rs` 57 and `include/w32gui.rs` 64 do for their own cursor handles.
- **`_XCharStruct` and `XRectangle` are measured, not guessed**, and every number is asserted in the
  mirror; `XChar2b`'s 2/2 is asserted too.  The anonymous `XRectangle` typedef is written under the
  typedef's own name (a Rust struct needs one; PORTING.md P-67, the `dispextern.rs`
  `Emacs_Rectangle` reading) - which is also the name the extractor's alias resolution matches.
- **`--check` asks for less than fidelity does**: the file has no functions, objects or subrs, so the
  ledger's only content is the 38 macros and the two records.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/pgtkgui.h femacs/src/include/pgtkgui.rs`
  -> `OK emacs/src/pgtkgui.h -> femacs/src/include/pgtkgui.rs` (no `- ` problem lines).
- `python3 tools/extract-abi.py emacs/src/pgtkgui.h` -> this ledger: 0 subrs, 0 functions defined,
  38 macros, 2 types, 0 file-scope objects.
- The measurements (`clang -std=gnu23 -O0 -I emacs/src` on a TU that includes `pgtkgui.h` and prints
  `sizeof`/`_Alignof`/`offsetof`; Apple clang 21.0.0): `XCharStruct` 20/4 with offsets 0/4/8/12/16,
  `XRectangle` 16/4 with 0/4/8/12, `XChar2b` 2/2.
- The collision measurement: the same TU with `dispextern.h` included first fails as quoted above.
- **No unit includes this mirror** (`rg -n 'include/pgtkgui' femacs` -> no hits); it is compiled by
  `xtask layout-check`'s single crate (the `have_pgtk` arm is inert there).
- `CARGO_TARGET_DIR=/tmp/femacs-target-s188 cargo run -q -p xtask -- layout-check` ->
  `layout-check: 87 of 98 mirror(s) compiled, 11 platform-exclusive skipped (...)`, `layout probe ok`,
  `layout-check passed`.
- `build-tu src/pgtkgui.h` -> `error[E0425]: cannot find type 'c_int' in this scope` - the mirror is
  C 19-121's own text with no `lisp.h` include (C has none either), so it takes `c_int`/`c_void` from
  the *unit*, exactly as `src/widget.h` (`cannot find type 'c_void'`), `src/w32font.h` and
  `src/w32gui.h` do (all measured on this worktree).  The compile evidence for this class of mirror
  is `layout-check`, and the same is written in `include-widget.md`'s Evidence.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-target-names` 612 .rs files / 593 map targets / 0 findings; `check-mirror-dupes`
  1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures.
