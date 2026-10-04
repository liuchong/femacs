# emacs/src/pgtkterm.h (681 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _PGTKTERM_H_  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The whole body is dead text and is not transcribed** (PORTING.md P-08).  `HAVE_PGTK` is
  `undef` in the frozen table, so the single `#ifdef HAVE_PGTK` at C 29 excludes C 29-680; the
  mirror carries the guard only.  The condition reading is the evidence that the arm was decided
  rather than assumed: `src/pgtkterm.h: 8 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- **The five live `#include` lines (C 23-27) are not mirrored**: they are unconditional C text,
  but they introduce no name the header's live text uses (everything else is inside the guard),
  and every one of those mirrors is either already present in the port or, for `sysselect.rs`,
  unrelated to this header's declarations.  A PGTK target adds them with the region.
- **The dead region's contents are listed by name and C line range** in the mirror and here, so
  the eventual PGTK port starts from an inventory rather than from re-reading 681 lines: four
  small records, the two large display/output records, the PGTK `struct x_output`, the selection
  record, the colour and `FRAME_*`/`SELECTION_EVENT_*` macro families, `x_display_list` and the
  prototype block.  No exclusion row is needed - the gate sees none of them, because the
  inventory is the guard alone.
- **No `include!` lines**, so the mirror names no GTK type and compiles as a crate root on its
  own; that is what the two dead-by-construction mirrors (gtkutil.rs, gnutls.rs) do as well.
- **`src/pgtkterm.c` is not in the frozen build** (docs/port-map.md 511-512), so the compile smoke
  is the note-only placeholder unit `femacs/src/pgtkterm.rs` (docs/status.md 3.31).

## Evidence

- `python3 tools/check-cond-eval.py src/pgtkterm.h` ->
  `src/pgtkterm.h: 8 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- `python3 tools/extract-abi.py --check emacs/src/pgtkterm.h femacs/src/include/pgtkterm.rs`
  -> `OK emacs/src/pgtkterm.h -> femacs/src/include/pgtkterm.rs` (no `- ` problem lines).
- `CARGO_TARGET_DIR=/tmp/femacs-target-mir-pgtkterm cargo run -q -p xtask -- build-tu
  src/pgtkterm.c` -> `built src/pgtkterm.rs -> ../build/femacs/pgtkterm.o`.
