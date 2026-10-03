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
