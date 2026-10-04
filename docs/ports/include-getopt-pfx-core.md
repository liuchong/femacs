# emacs/lib/getopt-pfx-core.h (76 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 10
- [x] _GETOPT  (line 65, body da39a3ee5e6b)
- [x] _GETOPT_PFX_CORE_H  (line 21, body 356a192b7913)
- [x] __GETOPT_CONCAT  (line 45, body 72559cbdecbc)
- [x] __GETOPT_ID  (line 47, body 1945c027c447)
- [x] __GETOPT_XCONCAT  (line 46, body 67afd7eb824c)
- [x] getopt  (line 54, body fecea2498d13)
- [x] optarg  (line 55, body 7ddebd5d6125)
- [x] opterr  (line 56, body b8bb2d144702)
- [x] optind  (line 57, body c71199e6aeef)
- [x] optopt  (line 58, body 14d45120b942)

## Types and their fields — 0

## File-scope objects — 0

- **Status: the unit is PORTED** (`lib/getopt-pfx-core`'s mirror, 2026-10-04).

## Decisions

**Status: the unit is PORTED; the checklist above was regenerated in 2026-10-04 (status 3.202)**
after `tools/check-cond-eval.py`'s marker bug was fixed.  Until then this header was listed in
`docs/baseline/config/cond-eval-verified.txt`, so the extractor consulted the frozen macro table
for it — but the tool built the marker as `EMACS_COND_TAKEN_{stem}_{line}` and a stem with a
hyphen or a dot made that an invalid C identifier, so clang's verdict was recorded as "dead" for
*every* branch the frozen build takes and the file's "verification" was vacuous.  With the marker
sanitised, this header is no longer verifiable (its disagreements are clang builtins and
`__cplusplus` predicates the table cannot decide), so it left the list and the extractor now reads
it blindly (P-66's conservative policy): both arms of every `#if` are demanded, which is what the
regenerated checklist shows.  Every name the mirror cannot hold as a Rust item is recorded with its
reason in `docs/ports/excluded-macros.txt` - the table is the port's answer to "C has this name and
Rust has no spelling for it".

