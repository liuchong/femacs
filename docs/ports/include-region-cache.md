# emacs/src/region-cache.h (112 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_REGION_CACHE_H  (line 22, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **What the mirror carries, and what it does not.**  C's `struct region_cache` is *defined* in
  `src/region-cache.c`, not here (C 60's `struct region_cache` is a forward declaration), so the
  record stays in `femacs/src/region-cache.rs` and this mirror declares the six entry points with
  the record spelled `*mut c_void` - the ABI-equal form 0.1-2/P-72 rule 3 asks for, and the same
  spelling `include/buffer.rs` already uses for the three cache fields (`newline_cache`,
  `width_run_cache`, `bidi_paragraph_cache`).  The six declarations sit in `region_cache_decls`
  + glob import (0.1-10), because `femacs/src/region-cache.rs` defines every one of them.
- **`struct buffer` is kept, not hidden.**  C 59 forward-declares it and C's prototypes spell
  `struct buffer *BUF`; buffer.rs's mirror owns the record, so this mirror `include!`s
  buffer.rs and keeps `*mut buffer` - the intervals.rs precedent (whose `offset_intervals` and
  `graft_intervals_into_buffer` do the same).  The include is also what makes the layout probe
  order-independent (0.1-4's graph puts buffer.rs before this file wherever it is reached from).
- **The four consumers.**  `bidi.rs` (C 247), `insdel.rs` (C 30), `search.rs` (C 32) and the
  definition unit `region-cache.rs` (C 27) now include this mirror at C's own position instead of
  declaring the same six entries themselves; the copies that stood in `bidi.rs`, `insdel.rs` and
  `search.rs` went away in the same change, because two same-named declarations in one unit are
  an `E0428` (the regex-emacs.rs/mirror precedent, status 3.68).  `search.rs` keeps one local
  `pub type region_cache = c_void;` because six of its local declarations spell C's type name
  (P-72 rule 3); nothing in any body moved.
- **Gates.**  `extract-abi.py --check emacs/src/region-cache.h femacs/src/include/region-cache.rs`
  -> `OK`; the four consumers rebuild (`build-tu`) and their `--check` is `OK`; `layout-check`
  passed (791 assertions) with this mirror in the probe crate.
