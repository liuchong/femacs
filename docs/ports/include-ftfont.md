# emacs/src/ftfont.h (94 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_FTFONT_H  (line 23, body da39a3ee5e6b)

## Types and their fields — 1
- [x] font_info: struct font font, FT_Size ft_size, int index, FT_Matrix matrix, Display* display, XftFont* xftfont, unsigned x_display_id

## File-scope objects — 0

## Decisions

- **Nothing on this target compiles the header, and the platform half keeps C's own names.**  No
  window system is configured (`HAVE_X_WINDOWS` undefined), `src/ftfont.c` is not in the frozen
  build's source list, and FreeType/Xft/Fontconfig have no mirrors here.  `struct font_info` and
  the three Fontconfig entry points are therefore carried under
  `#[cfg (feature = "have_x_windows")]` - P-51's arm for the non-frozen side, with the feature
  named the way status 3.171's rule requires (the C macro's literal lowercased spelling) - and
  spell `FT_Size`, `FT_Matrix`, `Display`, `XftFont` and `FcPattern` as C does, because that
  target supplies them.  Writing stand-ins for those would throw the platform's types away; the
  `include/macfont.rs` (3.170) and `include/w32font.rs` (3.176) mirrors make the same split.
- **The other arms are recorded, not mirrored.**  `HAVE_LIBOTF` (`maybe_otf`, `otf`),
  `HAVE_HARFBUZZ` (`hb_font`) and `USE_CAIRO || USE_BE_CAIRO` (`cr_scaled_font`,
  `bitmap_position_unit`, `metrics`, `metrics_nrows`) are dead on this target; the mirror keeps
  their names and guards in `/// C:` lines so a cairo/HarfBuzz target knows which `#if` to re-read
  (P-51), and the extractor's field list agrees - it asks for none of them.
- **The C 26-43 include chain is recorded, not imitated.**  `ft2build.h`/FreeType, `cairo.h`,
  `hb.h`/`hb-ft.h` and `otf.h`/`m17n-flt.h` have no mirrors, and the mirror names no type from them
  outside the `have_x_windows` arm (PORTING.md 0.1-9: a unit reaches those through its own
  includes).
- **`FcPattern *`/`XftFont *`/`Display *` keep C's pointer depth** and are `*mut c_void` where the
  pointee is Fontconfig's or Xft's own record; `Lisp_Object` is named (every unit that would take
  this mirror has lisp.h).
- **`--check` asks for less than fidelity does**: the four prototypes are declarations, not
  definitions (`## Functions defined — 0`), so they are carried because C's text has them.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/ftfont.h femacs/src/include/ftfont.rs`
  -> `OK emacs/src/ftfont.h -> femacs/src/include/ftfont.rs` (no `- ` problem lines).
- `python3 tools/extract-abi.py emacs/src/ftfont.h` -> this ledger: 0 subrs, 0 functions defined,
  1 macro (the guard), 1 type (`font_info`), 0 file-scope objects.
- **No unit includes this mirror** (`rg -n 'include/ftfont' femacs` -> no hits); it is compiled by
  `xtask layout-check`'s single crate like every other `src/include` mirror (the `have_x_windows`
  arm is inert there).
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures;
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures.
