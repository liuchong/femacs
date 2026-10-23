# emacs/src/getpagesize.h (48 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of src/getpagesize.h, complete and **empty by construction**: the header's whole text sits
under `#ifndef HAVE_GETPAGESIZE` (C 20), and the frozen configuration defines HAVE_GETPAGESIZE
(config.h; `docs/baseline/config/darwin-arm64.macros.tsv`), so `tools/extract-abi.py` reports
0 subr / 0 function / 0 macro / 0 type / 0 object for the live text - there is nothing to
transcribe, and `#define getpagesize ()` defines nothing, exactly as C's preprocessor leaves it.
The dead arms are recorded here for a configuration that lacks the function: `sysconf
(_SC_PAGESIZE)`, `EXEC_PAGESIZE`, `NBPG * CLSIZE` (with its `# define CLSIZE 1`), `PAGESIZE`, and
the `8192` punt.  The header is still included by sources the frozen build compiles (src/emacs.c,
src/gmalloc.c, src/ralloc.c), which is why the mapping exists at all - a unit that includes it
gets nothing from it here, as in C.

Evidence: `python3 tools/extract-abi.py --check emacs/src/getpagesize.h
src/include/getpagesize.rs` -> `OK`; `xtask layout-check` compiles every `src/include/*.rs`
into one crate and passes with this file present (an empty mirror still has to parse).
