# emacs/src/widget.h (102 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 36
- [x] XtCBarCursor  (line 51, body d9ae1f43d258)
- [x] XtCBellVolume  (line 55, body bf31cde8cef9)
- [x] XtCCursorColor  (line 49, body 143c414101df)
- [x] XtCEmacsFrame  (line 67, body b7b32d8e9cd7)
- [x] XtCFont  (line 39, body df29b4979226)
- [x] XtCForeground  (line 45, body a55df86b1c09)
- [x] XtCGeometry  (line 73, body ed018bf890e0)
- [x] XtCInitialGeometry  (line 86, body fd0422337609)
- [x] XtCInterline  (line 33, body dcaf5384a0da)
- [x] XtCInternalBorderWidth  (line 31, body bba52625947d)
- [x] XtCMinibuffer  (line 27, body 4dd03d93aa38)
- [x] XtCUnsplittable  (line 29, body 5b4bf4afcbe6)
- [x] XtCVisualBell  (line 54, body fc6c726ae4ab)
- [x] XtNallowResize  (line 79, body 0557fef1099a)
- [x] XtNbarCursor  (line 50, body 7c1f2855d900)
- [x] XtNbellVolume  (line 56, body 67eb320f8a0d)
- [x] XtNcursorColor  (line 48, body 6d992ed17483)
- [x] XtNemacsFrame  (line 66, body 899e1c26b35d)
- [x] XtNfont  (line 36, body 32f8a9f5c169)
- [x] XtNforeground  (line 42, body 08533fbfdd86)
- [x] XtNgcPointer  (line 64, body 68a2c391d5b7)
- [x] XtNgeometry  (line 70, body e12dfa1a5c88)
- [x] XtNinitialGeometry  (line 85, body a2925865ea4c)
- [x] XtNinterline  (line 32, body 3ae80260ea62)
- [x] XtNinternalBorderWidth  (line 30, body 87ab5f61754e)
- [x] XtNminibuffer  (line 26, body 396d732dc1a0)
- [x] XtNmodeLinePointer  (line 63, body 92f12dbb9023)
- [x] XtNpointerBackground  (line 58, body 6eb6aac2c81f)
- [x] XtNpointerColor  (line 59, body 98749d78a88f)
- [x] XtNresizeToPreferred  (line 82, body 68d557e08f21)
- [x] XtNshowGrip  (line 76, body 1b01e0922aa4)
- [x] XtNspacePointer  (line 62, body c87242ef5405)
- [x] XtNtextPointer  (line 61, body 3089ca4c865e)
- [x] XtNunsplittable  (line 28, body c56f34e40dd2)
- [x] XtNvisualBell  (line 53, body bb6237b09f8e)
- [x] _EmacsFrame_h  (line 22, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern struct _DisplayContext display_context  (line 95)

## Decisions

- **Nothing on this target compiles the header.**  `USE_X_TOOLKIT` and `USE_LUCID` are undefined,
  `src/widget.c` is not in the frozen build's source list, and the types the header names
  (`Widget`, `WidgetClass`, `XtN*`) are Xaw's, which has no mirror here.  The mirror carries the
  header's text the way `src/w32select.h` (3.158) and `include/macfont.rs` (3.170) do.
- **The string half is frozen-visible; the Xaw half is behind `#[cfg (feature = "use_x_toolkit")]`.**
  The 35 resource-name macros are plain C string literals, so they are `pub const … : &CStr` and
  compared as values by the gate; the opaque typedefs, `emacsFrameClass` and the three entry
  points name Xaw's types and are spelled with C's own names under the arm that has them (P-51's
  non-frozen side).  `use_lucid` is the other half of the same build; the toolkit feature is the
  one this repository declares, and the reader should read both when that target is frozen.
- **The guarded macros keep their guards in `/// C:` lines.**  C 34-46 wraps six of them in
  `#ifndef` because older Xaw headers may define the name already; the port emits the constant
  unconditionally (Rust has no "define if absent" for a name this file owns) and the ledger says
  why - the same reading `src/w32common.h` records for its `DWORD_PTR`.
- **`display_context` keeps C's pointer depth with an untyped pointee.**  `struct _DisplayContext`
  is `src/widget.c`'s own record (C 92 forward-declares it), so the object is `*mut c_void` with
  C's declaration in a `/// C:` line (P-55).
- **`--check` asks for less than fidelity does**: the five declarations are not definitions, so
  they are carried because C's text has them.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/widget.h femacs/src/include/widget.rs`
  -> `OK emacs/src/widget.h -> femacs/src/include/widget.rs` (no `- ` problem lines).
- `python3 tools/extract-abi.py emacs/src/widget.h` -> this ledger: 0 subrs, 0 functions defined,
  36 macros (the guard and the 35 resource names), 0 types, 1 file-scope object.
- **No unit includes this mirror** (`rg -n 'include/widget' femacs` -> no hits for `widget.rs`);
  it is compiled by `xtask layout-check`'s single crate (the `use_x_toolkit` arm is inert there).
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures;
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures.
