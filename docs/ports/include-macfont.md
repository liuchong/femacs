# emacs/src/macfont.h (88 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] MAC_FONT_CHARACTER_SET_STRING_ATTRIBUTE  (line 77, body 78e8a1b9f572)
- [x] kCTVersionNumber10_9  (line 75, body 15d5e9fd0921)
- [x] mac_font_get_glyph_for_cid  (line 71, body 8db4e18953bc)

## Types and their fields — 1
- [x] mac_glyph_layout: CFRange comp_range, CFIndex string_index, CGFloat advance_delta, CGFloat baseline_delta, CGFloat advance, CGGlyph glyph_id

## File-scope objects — 0

## Decisions

- **The header is the Cocoa font backend's, and nothing on this target compiles it.**  `HAVE_NS` is
  undefined, `src/macfont.c` is not in the frozen build's source list (its platform object variables
  are empty), and every type the file names comes from CoreFoundation/CoreGraphics, which this tree
  has no mirror for.  The mirror carries the header's live text the way `src/w32select.h` (3.158)
  and `src/xsettings.h` (3.163) do.
- **The CoreFoundation half is carried under `#[cfg (feature = "macos_sdk")]`, P-51's shape for a
  platform arm no build here enables.**  `struct mac_glyph_layout` keeps C's field names *and* C's
  field types (`CFRange`, `CFIndex`, `CGFloat`, `CGGlyph`); the three Core Text `enum` blocks keep
  C's members and their Core Text values (`kCTFontItalicTrait`, …); the smoothing setter keeps
  `CGContextRef`.  Spelling those names rather than a stand-in is deliberate: the macOS target
  supplies them from the SDK, nothing here is compiled on this target, and the gate reads text.  No
  layout assertion is written for the record - no compiler here could measure it (the
  `include-w32heap.md`/`include-emacsgtkfixed.md` reading).
- **`kCTFontTraitColorGlyphs` is C's one double-defined member** (C 53-58: `kCTFontColorGlyphsTrait`
  under `#if MAC_OS_X_VERSION_MAX_ALLOWED >= 1070`, `(1 << 13)` otherwise), and the file's
  conditions are blind to the evaluator, so the extractor's C value is the two arms *concatenated*.
  The port spells the SDK arm's identifier and the other arm's tokens are recorded in
  `docs/ports/macro-body-notes.tsv` with that reason.
- **`mac_font_get_glyph_for_cid` and `MAC_FONT_CHARACTER_SET_STRING_ATTRIBUTE` are text macros**, so
  the mirror writes `macro_rules!` bodies that are never expanded: `CFSTR` is CoreFoundation's and
  the alias target is Core Text's, and a body is not resolved until it is used (the
  `include-w32common.rs` `ROUND_UP` reading).
- **`CGContextSetFontSmoothingStyle` is `weak_import` in C, and Rust has no spelling for it.**  The
  mirror declares the symbol plainly and this ledger records the deviation: a macOS target resolves
  it with `dlsym` (or a linker flag) instead of relying on the weak binding.
- **`--check` asks for less than fidelity does**: C 82-87 are declarations, not definitions, so the
  four entry points are carried because C's text has them, not because an item asks.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/macfont.h femacs/src/include/macfont.rs`
  -> `OK emacs/src/macfont.h -> femacs/src/include/macfont.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/macfont.h` -> `4 conditions, 0 unknown to the evaluator,
  0 disagreements`.
- `python3 tools/extract-abi.py emacs/src/macfont.h` -> this ledger: 0 subrs, 0 functions defined,
  3 macros, 1 type (`mac_glyph_layout`), 0 file-scope objects.
- **No unit includes this mirror** (`rg -n 'include/macfont' femacs` -> no hits); it is compiled by
  `xtask layout-check`'s single crate like every other `src/include` mirror (the `macos_sdk` arm is
  inert there).
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures;
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures.
