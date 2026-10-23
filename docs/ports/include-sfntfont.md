# emacs/src/sfntfont.h (78 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _SFNTFONT_H_  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **Conditions, frozen.**  `tools/check-cond-eval.py src/sfntfont.h` -> `3 conditions, 0 unknown to
  the evaluator, 0 disagreements`, and the file is on `docs/baseline/config/cond-eval-verified.txt`
  (362-363), so P-08's answer is what the mirror carries: C 62-66's `HAVE_MMAP` arm is **live**
  (`sfntfont_detect_sigbus`, C 64) and C 72-76's `HAVE_HARFBUZZ` arm is **dead**, carried under
  `#[cfg (feature = "harfbuzz")]` (PORTING.md P-51).  Nothing on this target compiles
  `src/sfntfont.c` or `src/sfnt.c`, so no unit `include!`s the mirror and the declarations are
  deferred to the SFNT targets with their definitions (the `include/sfnt.rs` reading).
- **`glyph_string` is `*mut c_void`, and the measurement says that is the standing spelling.**  The
  integrator's 3.170 note recorded two `E0425`s when the mirror named the record; re-measured here,
  the reason is that `include/dispextern.rs` 1362 declares `glyph_string`
  `#[cfg (feature = "have_window_system")]` - correctly, because dispextern.h 1358 has it inside
  `#ifdef HAVE_WINDOW_SYSTEM` and the frozen configuration has no window system - so the name exists
  in no crate this target builds.  Naming it un-gated reddens `xtask layout-check` every round; gating
  the two declarations with that feature would deviate from C's unconditional text.  The two sites
  (`sfntfont_draw` C 41 and the `sfntfont_put_glyph_proc` typedef C 49-51) therefore keep P-55/P-72
  rule 3, with C's prototype in the `/// C:` line above each; the reading of `glyph_string` as a
  windowed record is `include/haikuterm.rs` 489-494's.
- **Four more pointers have no owner mirror and keep C's depth**: `struct font_metrics` and
  `struct font_driver` are `font.h`'s and `include/font.rs` 129 says `font_driver` has not landed, and
  the dead arm's `hb_font_t` is `<hb.h>`'s.  Each has its C text in a `/// C:` line above the
  declaration it belongs to.
- **Names that *are* reachable are named**: `Lisp_Object` (lisp.h is every unit's include, 0.1-9),
  `struct frame` (`frame.rs`), `struct font` (`font.rs`), `struct sfnt_raster` (`sfnt.rs`) - the
  three mirrors C 24-26 includes are `include!`ed, and the function-pointer typedef is an `Option`
  (P-27).  C 44's `unsigned[256]` parameter decays to `*mut c_uint` (P-55's pointer reading), and
  `sfntfont_encode_char`'s `unsigned int` return is `c_uint`.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/sfntfont.h src/include/sfntfont.rs`
  -> `OK emacs/src/sfntfont.h -> src/include/sfntfont.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/sfntfont.h` -> `3 conditions, 0 unknown to the evaluator,
  0 disagreements`; `grep -n sfntfont docs/baseline/config/cond-eval-verified.txt` -> 362-363.
- `python3 tools/extract-abi.py emacs/src/sfntfont.h` -> this ledger: 0 subrs, 0 functions defined,
  1 macro (the guard), 0 types, 0 file-scope objects (`--check` asks only for the guard, and the
  four declarations groups are carried because they are the header's live text).
- The `glyph_string` measurement: `sed -n '1358,1366p' src/include/dispextern.rs` ->
  `#[cfg (feature = "have_window_system")]` immediately above `pub struct glyph_string`; the mirror's head
  NOTE records the two `E0425`s the integrator's edit was made from.
- **Compile evidence**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask
  -- layout-check` -> `layout-check: 80 of 91 mirror(s) compiled, 11 platform-exclusive skipped
  (android-asset.rs, android.rs, androidgui.rs, androidterm.rs, dispextern.rs, gtkutil.rs,
  haiku_support.rs, haikugui.rs, haikuterm.rs, nsgui.rs, pgtkterm.rs)` and `layout-check passed
  (see build/femacs/layout_asserts.rs)`.  This mirror is among the 80.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 596 .rs files / 593 map targets / 0 findings.
