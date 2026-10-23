# emacs/msdos/inttypes.h (38 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 5
- [x] PRIdMAX  (line 35, body 8be33cc4f88e)
- [x] PRIuMAX  (line 34, body bc98c6ecde70)
- [x] _REPL_INTTYPES_H  (line 21, body da39a3ee5e6b)
- [x] strtoimax  (line 33, body 9ac4bdbe28fa)
- [x] strtoumax  (line 32, body 5e42748aa545)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of msdos/inttypes.h, complete for this target: the include guard, the two "as of May 2011,
DJGPP v2.04 does not include stdint.h in its inttypes.h" facts (C 25-28's unconditional
`# include <stdint.h>` and C 31's `# include_next <inttypes.h>`, both recorded - an `#include` has
no Rust counterpart), and the four macros of C 33-36 (`strtoumax`/`strtoimax` as function macros
over `strtoull`/`strtoll`, `PRIuMAX`/`PRIdMAX` as the `"llu"`/`"lld"` string literals the DJGPP
arm spells).

`msdos/inttypes.h` is not in `docs/baseline/config/cond-eval-verified.txt`, so both arms of
C 30's `__DJGPP__ > 2 || __DJGPP_MINOR__ >= 4` test are carried (PORTING.md P-51), the dead one
under the feature `no_djgpp_2_4` - the arm that C's text spells for older DJGPP, which is the
only arm with content.  Nothing compiles the mirror (no `msdos/` unit is built and
`xtask layout-check` covers `src/include` and `lib/include`), so the renames and the two
`printf` length modifiers are text a future DJGPP build would consume.

**Ledger name**: `include-msdos-inttypes.md` for the same reason as the langinfo pair
(`nt/inc/inttypes.h` shares the stem; status 3.103's third candidate).

Evidence: `python3 tools/extract-abi.py --check emacs/msdos/inttypes.h
msdos/include/inttypes.rs` -> `OK` (4 ledger items).
