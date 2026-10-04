# emacs/nt/inc/langinfo.h (64 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 23
- [x] CODESET  (line 36, body d796871128ae)
- [x] DAY_1  (line 38, body 5e8c072a8341)
- [x] DAY_2  (line 39, body 2eeaa05a0021)
- [x] DAY_3  (line 40, body 0e3bb6f98fd3)
- [x] DAY_4  (line 41, body bf11f3be721f)
- [x] DAY_5  (line 42, body 229210c4b564)
- [x] DAY_6  (line 43, body d0e66262c477)
- [x] DAY_7  (line 44, body 38fec50957ac)
- [x] MON_1  (line 46, body bf494c776cbf)
- [x] MON_10  (line 55, body fd26a02ea505)
- [x] MON_11  (line 56, body 0d44ec27ef7c)
- [x] MON_12  (line 57, body 5eb3affdb900)
- [x] MON_2  (line 47, body 45aebc353662)
- [x] MON_3  (line 48, body 9f7576b4c418)
- [x] MON_4  (line 49, body 9bfa5bd2bdf0)
- [x] MON_5  (line 50, body f2eae0429bd3)
- [x] MON_6  (line 51, body 5cea19bdb05d)
- [x] MON_7  (line 52, body a779b97684cb)
- [x] MON_8  (line 53, body be409dc166a7)
- [x] MON_9  (line 54, body ac2334ec07df)
- [x] _LANGINFO_H  (line 21, body da39a3ee5e6b)
- [x] _NL_PAPER_HEIGHT  (line 60, body be93a2cbad5f)
- [x] _NL_PAPER_WIDTH  (line 59, body 3eeb87162ae3)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/langinfo.h, complete for this target: the include guard, the anonymous `enum`'s
23 members as `const` (C's values: `CODESET` 0 through `_NL_NUM` 22) with `nl_item` spelled
`c_int` per PORTING.md 1.3, and the one prototype.

**Which arm**: the file has no `#if` and is not on
`docs/baseline/config/cond-eval-verified.txt`, so the extractor resolves nothing in it - the
whole file is live text.

**The 22 self-referential `#define`s**: C writes `#define CODESET CODESET` … `#define
_NL_PAPER_HEIGHT _NL_PAPER_HEIGHT` so that the enum members are visible to `#if` as well as to the
compiler.  Each body is that one self-reference and nothing else, so the constant above is its
whole counterpart - there is no place in the constant for the name to appear as a token - and the
22 are recorded in `docs/ports/macro-body-notes.tsv` as **one globbed row**
(`CODESET,DAY_*,_NL_PAPER_WIDTH,_NL_PAPER_HEIGHT,MON_*` with token `*`): every one of those C
bodies is a single self-reference, so `*` and the per-macro token are the same allowance.  This is
the shape nt/inc/sys/resource.h used for its three `RLIMIT_*` self-references, which are three
individual rows because each has a different reason to record.

**Includes**: C 23's `#include <nl_types.h>` resolves to this tree's own `nt/inc/nl_types.h` in a
MinGW build, so the mirror pulls in `include/nl_types.rs`, which is where `nl_item` comes from;
the mirror also restates it as `c_int` so the constants read the way PORTING.md 1.3 prescribes.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check --macro-notes <this ledger plus the row above>
emacs/nt/inc/langinfo.h femacs/nt/inc/include/langinfo.rs` -> `OK`; without that row the check
reports the 22 self-references as "macro X does not mention x", which is what the row records.
