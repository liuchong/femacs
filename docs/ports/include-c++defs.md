# emacs/lib/c++defs.h (367 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 23
- [x] _GL_BEGIN_NAMESPACE  (line 25, body da39a3ee5e6b)
- [x] _GL_CXXALIASWARN  (line 338, body b25da74b5ddd)
- [x] _GL_CXXALIASWARN1  (line 363, body b25da74b5ddd)
- [x] _GL_CXXALIASWARN1_1  (line 349, body 5c403b095ea6)
- [x] _GL_CXXALIASWARN1_2  (line 359, body b25da74b5ddd)
- [x] _GL_CXXALIASWARN_1  (line 321, body 52f71663411a)
- [x] _GL_CXXALIASWARN_2  (line 334, body b25da74b5ddd)
- [x] _GL_CXXALIAS_MDA  (line 192, body 4f9f1e31af3f)
- [x] _GL_CXXALIAS_MDA_CAST  (line 225, body de4488a988c6)
- [x] _GL_CXXALIAS_RPL  (line 163, body b619500f8d3e)
- [x] _GL_CXXALIAS_RPL_1  (line 181, body b25da74b5ddd)
- [x] _GL_CXXALIAS_RPL_CAST_1  (line 216, body b25da74b5ddd)
- [x] _GL_CXXALIAS_SYS  (line 254, body b25da74b5ddd)
- [x] _GL_CXXALIAS_SYS_CAST  (line 279, body b25da74b5ddd)
- [x] _GL_CXXALIAS_SYS_CAST2  (line 310, body b25da74b5ddd)
- [x] _GL_CXXDEFS_H  (line 18, body da39a3ee5e6b)
- [x] _GL_END_NAMESPACE  (line 26, body da39a3ee5e6b)
- [x] _GL_EXTERN_C  (line 93, body bf1ee3119b17)
- [x] _GL_EXTERN_C_FUNC  (line 104, body da39a3ee5e6b)
- [x] _GL_FUNCDECL_RPL  (line 125, body ba420a912aae)
- [x] _GL_FUNCDECL_RPL_1  (line 127, body 248cd57169d7)
- [x] _GL_FUNCDECL_SYS  (line 151, body 03604a18d063)
- [x] _GL_FUNCDECL_SYS_NAME  (line 137, body 8bb2e794d1ca)

## Types and their fields — 0

## File-scope objects — 0

- **Status: the unit is PORTED** (`lib/c++defs`'s mirror, 2026-10-04).

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

