# emacs/src/w32gdiplus.h (145 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The file has no include guard, and its two halves are decided by `WINDOWSNT`.**  C 1 opens
  `#ifdef WINDOWSNT`, C 142 closes it, and C 144-145 (the two entry points) are live text on every
  platform (measured: `head -1` is `#ifdef WINDOWSNT`, and `rg -n 'ifndef .*_H'` finds nothing).
  `WINDOWSNT` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so C 1-142 is dead here
  and carries `#[cfg (feature = "windowsnt")]` per item - P-51's arm for the non-frozen side, the
  `include/w32font.rs` (3.176) / `include/macfont.rs` (3.170) shape.  Its C readers are Windows units
  (`src/w32image.c` and the `w32fns.c` image paths), none of them in
  `docs/baseline/frozen-build-sources.txt`, so no unit here includes it.
- **C's own names are kept inside that arm.**  `GpStatus`, `GpImage`, `GpGraphics`, `GpBitmap`,
  `IStream`, `HDC`, `HBITMAP`, `HPALETTE`, `WCHAR`, `UINT`, `PROPID`, `GUID`, `CLSID`,
  `PropertyItem`, `ImageCodecInfo`, `EncoderParameters`, `PixelFormat`, `InterpolationMode`,
  `GpUnit`, `RotateFlipType`, `ARGB`, `ULONG_PTR`, `DrawImageAbort`, `GetThumbnailImageAbort` come
  from `<windows.h>` and GDI+, which have no mirror in this tree; the Windows target supplies them,
  the `include/ftfont.rs` 9-18 reading of `Display`/`XftFont`/`FcPattern`.  Writing stand-ins would
  be an unverifiable ABI guess for a dozen GDI+ records at once.
- **The calling convention and the nullability are spelled the way `include/w32.rs` spells them.**
  `WINGDIPAPI`/`WINAPI` is `__stdcall`, which the port writes `extern "system"` (w32.rs 317-329),
  and a C function pointer is nullable, so each of the 27 typedefs is an `Option<…>` (w32.rs
  323-329).  `GDIPCONST` is C's `const`, so those parameters are `*const T`.
- **C's two repeated names are written once, at their first C line.**  `GdipCreateBitmapFromHBITMAP_Proc`
  (C 26-27 and 53-54), `fn_GdipCreateBitmapFromHBITMAP` (C 68 and 83),
  `#undef GdipCreateBitmapFromHBITMAP` (C 96 and 112) and its `#define` (C 125 and 141) are the same
  declaration twice; Rust has one name per scope, so the repeats are recorded in the mirror.
- **The `#undef`/`#define` pairs are nullary `macro_rules!`.**  C renames each GDI+ entry point to its
  loaded pointer; the port keeps one name per object (`fn_GdiplusStartup`, declared in the
  `unsafe extern "C"` block) and spells the alias as a nullary macro whose body is the C name - the
  `include/macfont.rs` 62-66 / `include/w32gui.rs` 41 reading of an object-like `#define`.  The 27
  `#undef`s have no Rust counterpart and are recorded in the `/// C:` line above each macro.
- **C 111's `#undef GdipSaveImageRotateFlip` stays as C writes it.**  No typedef or object of that
  name exists in the file (C 140 defines `GdipImageRotateFlip`); the port records the line rather
  than silently correcting the reference.
- **No layout assertion is written.**  Nothing in the file is a record the frozen clang could
  compile: every type it names is `<windows.h>`'s or GDI+'s (the `include/macfont.rs` 26-27 rule),
  and `tools/extract-abi.py` inventories the file as 0 macros, 0 types, 0 objects and 0 functions -
  so the mirror's required content is empty and its fidelity is C's text, not the gate's.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/w32gdiplus.h src/include/w32gdiplus.rs`
  -> `OK emacs/src/w32gdiplus.h -> src/include/w32gdiplus.rs` (no `- ` problem lines).
- `python3 tools/extract-abi.py emacs/src/w32gdiplus.h` -> this ledger: 0 subrs, 0 functions defined,
  0 macros, 0 types, 0 file-scope objects (everything is inside the dead `#ifdef WINDOWSNT`).
- `C 144-145` are outside that guard and are carried live: `w32_gdip_get_encoder_clsid (const char *,
  CLSID *)` and `w32_gdip_export_frame (HWND, Lisp_Object, CLSID *)`, with `CLSID`/`HWND` as P-55's
  untyped pointers and `Lisp_Object` named as the unit's (the `include/nsxwidget.rs` 26-28 reading).
- **No unit includes this mirror** (`rg -n 'include/w32gdiplus' femacs` -> no hits); it is compiled by
  `xtask layout-check`'s single crate (the `windowsnt` arm is inert there).
- `CARGO_TARGET_DIR=/tmp/femacs-target-s188 cargo run -q -p xtask -- layout-check` ->
  `layout-check: 87 of 98 mirror(s) compiled, 11 platform-exclusive skipped (...)`, `layout probe ok`,
  `layout-check passed`.
- `build-tu src/w32gdiplus.h` -> `error[E0425]: cannot find type 'c_char' in this scope` (then
  `c_void`/`c_int`): C 1-145's text includes no header, so the names come from the unit - the same
  class as `src/w32gui.h`/`src/widget.h` (measured on this worktree, `cannot find type 'c_void'`).
  The compile evidence for this mirror is `layout-check`.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-target-names` 612 .rs files / 593 map targets / 0 findings; `check-mirror-dupes`
  1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures.
