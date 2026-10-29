# emacs/src/xml.c (358 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 1
- [x] libxml-available-p  (Flibxml_available_p, 0/0, doc eb5cc6fe3238, line 321)

## Functions defined — 1
- [x] syms_of_xml  (line 349)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **`#ifdef HAVE_LIBXML2` is dead on this target** (docs/baseline/config/darwin-arm64.macros.tsv
  records it undefined), so C compiles C 24-317's parser, the two `libxml-parse-*-region`
  subrs and their helpers to nothing; the port carries no text for them (status §3.25's
  wholly-dead class at the arm's granularity) and records the redirection in
  `libxml2_arm`.  The live text is `libxml-available-p`'s `#else` arm and `syms_of_xml`.
- **The dead arm's two `defsubr` calls are not written** (C 351-354): `extract-abi.py`'s
  Rust-side `defsubr` scan does not consult the condition table, so the calls would read as
  live registrations (P-76's data.c finding).  The names are recorded in the comment and here.
- **`libxml-available-p`'s `HAVE_LIBXML2` arm** (with its inner `#ifdef WINDOWSNT` half) is kept
  as `#[cfg (feature = "libxml2")]` text: it names `Vlibrary_cache`, `Fassq`, `XCDR`,
  `Fcons`, `Qlibxml2`, `Qt` and `init_libxml2_functions`, none of which is resolved unless the
  feature is on, which is what lets the arm sit there without the DLL machinery.
- The file's `<libxml/*.h>` includes (C 27-29) have no counterpart on this target — the headers
  are not installed and the code that uses them is dead — so they are not mirrored; the unit's
  include list is C 20-23's (`config.h`, `lisp.h`, `buffer.h`) plus the closure those mirrors
  pull in (`character`, `itree`, `systime`, `dispextern`, `composite`, `coding`).


## Owned macOS capability restoration (P-399, 2026-10-27)

The table above records the historical feature-disabled port, not full XML
coverage. The owned macOS source now contains both parser subrs, the DOM
conversion and gap-aware region implementation, public system-library ABI,
registration and shutdown cleanup. The HAVE_LIBXML2 switch uses its actual
normalized cfg name, and both target library manifests select system xml2.
Stable31.1 xml.c at a360712c9d272d950d8d8255ef74570f7e90b7d9 matches
the pinned source; neither read-only checkout nor an installed header is a
production input. Library ABI/layout and original stable plus boundary behavior
acceptance remain pending. D26 is an older frozen snapshot without this change
and cannot certify it. Windows delayed loading is still not implemented.

P-399 integration is now wired through `tools/xml-acceptance.py`: actual arm64
SDK C layouts/constants and parser-owned DOM traversal are compared to the
owned Rust types; functional execution requires both normal/checking baseline
products and the installed stable comparator, checks actual ERT discovery,
and runs all13 boundary cases plus the unchanged original XML files with
per-case native outcomes. Complete product/resource identities are checked at
both boundaries. These are authored checks, not execution results.
