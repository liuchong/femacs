# emacs/src/w32font.h (120 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 11
- [x] CACHE_BLOCKSIZE  (line 84, body b4182bff4b3c)
- [x] EMACS_W32FONT_H  (line 20, body da39a3ee5e6b)
- [x] FONT_HANDLE  (line 81, body 23c03c0655d3)
- [x] FONT_TEXTMETRIC  (line 82, body b836671c4298)
- [x] NTMFLAGS_OPENTYPE  (line 36, body 115b97b040ee)
- [x] NTM_PS_OPENTYPE  (line 27, body 92168ce25c16)
- [x] NTM_TT_OPENTYPE  (line 30, body 8d1cb7d32ebd)
- [x] NTM_TYPE1  (line 33, body 1373527f4186)
- [x] W32METRIC_FAIL  (line 46, body da4b9237bacc)
- [x] W32METRIC_NO_ATTEMPT  (line 44, body b6589fc6ab0d)
- [x] W32METRIC_SUCCESS  (line 45, body 356a192b7913)

## Types and their fields — 3
- [x] uniscribe_font_info: struct w32font_info w32_font, void* cache, double scale, void* dwrite_cache, float dwrite_font_size, bool dwrite_skip_font
- [x] w32_metric_cache: short lbearing, short rbearing, short width, short ascent, short descent, unsigned char status
- [x] w32font_info: struct font font, TEXTMETRICW metrics, unsigned int glyph_idx, struct w32_metric_cache** cached_metrics, int n_cache_blocks, HFONT hfont

## File-scope objects — 0

## Decisions

- **The header splits in two, the way `include/macfont.rs` does.**  Everything that names a
  `<windows.h>` type lives under `#[cfg (feature = "windowsnt")]` (P-51's arm for the non-frozen
  side) spelled with **C's own names** - `struct w32font_info` (`TEXTMETRICW`, `HFONT`),
  `struct uniscribe_font_info` and the eighteen prototypes (`LOGFONT`, `HDC`, `COLORREF`) - because
  that target supplies them and inventing stand-ins here would throw the platform's types away.
  The frozen half is the guard, the numeric macros and `struct w32_metric_cache`.
- **`struct w32_metric_cache` is measured, not guessed.**  Its fields are C scalars only, so the
  frozen clang can compile and measure it even without `<windows.h>`: `sizeof` 12, `alignof` 2 and
  the six offsets are asserted in the mirror (`lbearing` 0, `rbearing` 2, `width` 4, `ascent` 6,
  `descent` 8, `status` 10), and `layout-check` evaluates them every round.  The two Windows
  records have **no** assertions, because no compiler on this target could measure them.
- **`struct glyph_string *` keeps the P-55 stand-in even under the Windows arm.**  The record is
  dispextern.h's and this mirror does not include it (status 3.170/3.171 measured the same shape
  for `include/sfntfont.rs`), so `w32font_draw`'s first parameter is `*mut c_void` with C's
  prototype in a `/// C:` line.
- **`FONT_HANDLE`/`FONT_TEXTMETRIC` are `macro_rules!` whose bodies name the Windows record.**
  A body is not resolved until it is expanded, so the frozen build compiles the mirror without the
  cfg; C's cast and member read are both kept in the body.
- **`--check` asks for less than fidelity does**: C 86-120 are declarations, not definitions, so
  the prototypes are carried because C's text has them, not because an item asks.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/w32font.h femacs/src/include/w32font.rs`
  -> `OK emacs/src/w32font.h -> femacs/src/include/w32font.rs` (no `- ` problem lines).
- `python3 tools/extract-abi.py emacs/src/w32font.h` -> this ledger: 0 subrs, 0 functions defined,
  11 macros, 3 types, 0 file-scope objects.
- The measurement of `w32_metric_cache` (`clang -std=gnu23` on a file that repeats C 40-45's
  declaration): `sizeof 12 align 2 lb 0 rb 2 w 4 a 6 d 8 st 10`.
- **No unit includes this mirror** (`rg -n 'include/w32font' femacs` -> no hits); it is compiled by
  `xtask layout-check`'s single crate like every other `src/include` mirror (the `windowsnt` arm is
  inert there).
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures;
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures.
