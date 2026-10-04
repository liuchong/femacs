# emacs/lib/warn-on-use.h (153 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] _GL_WARN_EXTERN_C  (line 151, body bf1ee3119b17)
- [x] _GL_WARN_ON_USE  (line 107, body 8feeee2ca003)
- [x] _GL_WARN_ON_USE_ATTRIBUTE  (line 109, body da39a3ee5e6b)
- [x] _GL_WARN_ON_USE_CXX  (line 139, body 8feeee2ca003)

## Types and their fields — 0

## File-scope objects — 0

- **Status: the unit is PORTED** (`lib/warn-on-use`'s mirror, 2026-10-04).

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

