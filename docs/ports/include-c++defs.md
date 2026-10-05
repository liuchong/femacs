# emacs/lib/c++defs.h (367 lines)

> **P-86: the frozen configuration compiles nothing here.**  Every name in the file either sits
> in an `#if` that is false for the frozen target - `defined __cplusplus` (C 90/98/134) or
> `defined __cplusplus && defined GNULIB_NAMESPACE` (C 21 and the seven alias blocks C 165-365) -
> or spells a declaration only those C++ arms use (the nine unconditional `#define`s, C 125-128 /
> 151-152 / 163-164 / 192-193 / 225-226), so the extractor's live-line filter lists no item and
> `tools/extract-abi.py --check` used to pass *vacuously* against a three-line note.  What follows is P-86 ②'s substitute for the empty
> checklist: the **C line-range table**, the **Gate mapping**, the **Evidence**, the
> **Deviations** and the **Owed**.  Port: `femacs/lib/include/c++defs.rs` (507 lines, 2026-10-05),
> every `#if` carried with both arms, C's own arm first and gated (`lib/include/fcntl.in.rs`'s
> order and reason), the `#else` arm ungated.
>
> History, kept because it is why the file is read blindly: the checklist below was regenerated
> in 2026-10-04 (status 3.202) after `tools/check-cond-eval.py`'s marker bug was fixed.  Before
> that this header sat in `docs/baseline/config/cond-eval-verified.txt` while the evaluator
> recorded *every* branch clang takes as dead - the probe marker is built from the file's stem
> and `c++defs` is not a C identifier, so the "verification" could never fire.  With the marker
> sanitised the file left the list and the extractor reads it blindly (P-66's conservative
> policy): both arms of every condition are asked for, which is what the mirror now carries.

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

- These 23 names are the extractor's whole item list for the file (no function, type or object).
  A tick is the mirror's claim that the name is carried: 20 of them as `macro_rules!` items (each a
  gated C++ arm and/or the ungated `#else` arm, which is what `macro_rules!`'s shadowing allows),
  and `_GL_CXXDEFS_H`, `_GL_BEGIN_NAMESPACE` and `_GL_END_NAMESPACE` as a `pub const`.  The
  **C line ranges** table below is P-86 ②'s coverage evidence - not these ticks, and not
  `--check`, which is quiet for the reason the Evidence records.

## Gate mapping

| C condition | Port gate | Compiled when |
| --- | --- | --- |
| C 90/98/134 `defined __cplusplus` (`_GL_EXTERN_C`, `_GL_EXTERN_C_FUNC`, `_GL_FUNCDECL_SYS_NAME`) | `feature = "cplusplus"` (already registered, `femacs/xtask/Cargo.toml` 80) | a C++ configuration |
| C 21/165/200/238/263/289/318/345 `defined __cplusplus && defined GNULIB_NAMESPACE` (the namespace pair at C 21-27 and the seven alias blocks C 165-365) | `feature = "cplusplus_gnulib_namespace"` (**new**) | C++ with `GNULIB_NAMESPACE` defined |
| C 325/353 `!(defined __GNUC__ && !defined __clang__ && __OPTIMIZE__)` (the two warning arms) | `not (feature = "gnuc_optimize_without_clang")` (**new name**), nested inside the C++ gate | a C++ build that is clang, or is not optimising |
| C 330 `(__GNUC__ >= 3 \|\| defined __clang__) && GNULIB_STRICT_CHECKING` | `feature = "gnuc3_or_clang_gnulib_strict_checking"` (**new**), nested inside the two above | a C++ build with `GNULIB_STRICT_CHECKING` |
| C 17 `#ifndef _GL_CXXDEFS_H` (the include guard) | ungated `pub const _GL_CXXDEFS_H: bool = true;` | every configuration |
| C 20, 29-86, 107-128, 140-152, 154-164, 185-193, 195-199, 220-226, 228-237, 258-262, 283-288, 314-317, 342-344, 367 (the notes and the nine unconditional `#define`s) | ungated | every configuration |

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-15 | C's licence/declaration block | dropped (P-86 ①: the port opens with the `// femacs: port of …` line; PORTING.md 0.1-19) |
| 17-18 | `#ifndef _GL_CXXDEFS_H` / `#define _GL_CXXDEFS_H` - the guard, whose C body is empty | ungated `pub const _GL_CXXDEFS_H` (the marker shape; an empty `macro_rules!` body is refused by the extractor) |
| 20-27 | C 20's `Begin/end the GNULIB_NAMESPACE namespace.` + `#if defined __cplusplus && defined GNULIB_NAMESPACE` (C 22-23 the C++ arm, C 25-26 the frozen arm) | C++ arm: no Rust item can hold `namespace GNULIB_NAMESPACE {` or `}` - one ungated marker `pub const` per name, both C arms quoted at it and recorded in `## Deviations`; frozen arm: the same empty marker |
| 29-86 | the three-use-case note (`@GNULIB_FOO@` / `@REPLACE_FOO@` / `@HAVE_FOO@` examples) | carried verbatim as prose (the mirror's `/* C 29-86, verbatim:` block) |
| 88-94 | `_GL_EXTERN_C`'s note (88-89) + the `#if defined __cplusplus` pair (C 91 C++ `extern "C"`, C 93 frozen `extern`) | C++ arm gated `cplusplus`; frozen arm ungated |
| 96-105 | `_GL_EXTERN_C_FUNC`'s note (96-97) + C 98-105 (C 99 C++ `extern "C"`, C 101-103 the comment inside the `#else`, C 104 the empty frozen arm) | C++ arm gated `cplusplus`; frozen arm ungated (empty expansion) |
| 107-128 | `_GL_FUNCDECL_RPL`'s note (107-124) + its two `#define`s (C 125-126 and C 127-128) | ungated (C defines both unconditionally) |
| 130-138 | `_GL_FUNCDECL_SYS_NAME`'s note (130-133) + `#ifdef __cplusplus` (C 135, C 137) | C++ arm gated `cplusplus`; frozen arm ungated |
| 140-152 | `_GL_FUNCDECL_SYS`'s note (140-150) + the `#define` (151-152) | ungated |
| 154-164 | `_GL_CXXALIAS_RPL`'s note (154-162) + the `#define` (163-164) | ungated |
| 165-183 | `_GL_CXXALIAS_RPL_1`: C 166-179 the namespace-wrapper C++ arm, C 181-182 the frozen `_gl_cxxalias_dummy` | C++ arm gated `cplusplus_gnulib_namespace`; frozen arm ungated |
| 185-193 | `_GL_CXXALIAS_MDA`'s note (185-191) + the `#define` (192-193) | ungated |
| 195-218 | `_GL_CXXALIAS_RPL_CAST_1`'s note (195-199) + C 200-218 (C 201-214 the C++ arm, C 216-217 the frozen arm) | as C 165-183 |
| 220-226 | `_GL_CXXALIAS_MDA_CAST`'s note (220-224) + the `#define` (225-226) | ungated |
| 228-256 | `_GL_CXXALIAS_SYS`'s note (228-237) + C 238-256 (C 239-252, C 254-255) | as C 165-183 |
| 258-281 | `_GL_CXXALIAS_SYS_CAST`'s note (258-262) + C 263-281 (C 264-277, C 279-280) | as C 165-183 |
| 283-312 | `_GL_CXXALIAS_SYS_CAST2`'s note (283-288) + C 289-312 (C 290-294 the inner cast note, C 295-308 the C++ arm, C 310-311 the frozen arm) | as C 165-183 |
| 314-340 | `_GL_CXXALIASWARN`'s note (314-317) + C 318-340: C 319-320 `_GL_CXXALIASWARN`, C 321-322 `_GL_CXXALIASWARN_1`, C 323-324's note, C 325-336 `_GL_CXXALIASWARN_2`'s three arms, C 338-339 the frozen `_GL_CXXALIASWARN` | `_GL_CXXALIASWARN` as C 165-183; `_GL_CXXALIASWARN_1` gated on the C++ condition alone; the three `_2` arms on the nested conditions (Gate mapping) |
| 342-365 | `_GL_CXXALIASWARN1`'s note (342-344) + C 345-365: C 346-348, C 349-350, C 351-352's note, C 354-361's two arms, C 363-364 the frozen arm | as C 314-340 |
| 367 | the trailing `#endif` whose comment names `_GL_CXXDEFS_H` | the guard's `pub const` is the whole of it (the port has no preprocessor) |

## Evidence

```
$ python3 tools/check-cond-eval.py lib/c++defs.h
DISAGREE lib/c++defs.h:17 `ifndef _GL_CXXDEFS_H` — clang True, evaluator False
lib/c++defs.h: 27 conditions, 0 unknown, 1 disagreements [NOT VERIFIED]
27 conditions, 0 unknown to the evaluator, 1 disagreements
exit=1

# The single disagreement is C 17's own guard: the file defines the name on the next line, so
# clang (which sees the prefix `#include <config.h>` plus the file's text up to the condition)
# says the branch is taken, while the evaluator's frozen table has no such macro.  It decides
# no item - the guard's body is the whole file either way - and the file is read blindly
# (P-66), which is why every arm below is carried rather than chosen.

$ python3 tools/extract-abi.py --check emacs/lib/c++defs.h femacs/lib/include/c++defs.rs
  (23 C macros have a recorded Rust counterpart: _GL_BEGIN_NAMESPACE, _GL_CXXALIASWARN, …)
OK emacs/lib/c++defs.h -> femacs/lib/include/c++defs.rs
exit=0

# The `OK` above is quiet by construction: every one of the 23 names still has a row in
# docs/ports/excluded-macros.txt (the P-86 ⑪ boilerplate this lane inherits), and an excluded
# name skips the body token comparison too.  The mirror now carries all 23, so those rows are
# stale (see Owed 5).  To measure the bodies anyway, the same rows were lifted into a /tmp copy
# of the table:

# (/tmp/cxxdefs-lane/excluded-minus-cxxdefs.txt is the table with the 23 c++defs rows removed: 242 -> 217 rows)
$ python3 tools/extract-abi.py --check --excluded /tmp/cxxdefs-lane/excluded-minus-cxxdefs.txt \
    emacs/lib/c++defs.h femacs/lib/include/c++defs.rs
  (14 C macros have several definitions (configuration branches), body not compared: _GL_BEGIN_NAMESPACE, …)
- macro _GL_FUNCDECL_RPL does not mention __va_args (C body: _GL_FUNCDECL_RPL_1 (rpl_##func, rettype, parameters, __VA_AR)
- macro _GL_FUNCDECL_RPL_1 does not mention __va_args (C body: _GL_EXTERN_C_FUNC __VA_ARGS__ rettype rpl_func parameters)
- macro _GL_FUNCDECL_SYS does not mention __va_args (C body: _GL_EXTERN_C_FUNC __VA_ARGS__ rettype _GL_FUNCDECL_SYS_NAME (func) par
FAIL emacs/lib/c++defs.h -> femacs/lib/include/c++defs.rs

# ... and with the three rows this lane appended to docs/ports/macro-body-notes.tsv in place, the
# lifted run is `OK` - so every other token of the nine single-definition bodies matches, the 14
# multi-definition names are read by name only (P-66), and C's `__VA_ARGS__` - PORTING.md 2.1's
# `$($args:tt)*` tail - is the one recorded class that cannot be spelled:

$ python3 tools/extract-abi.py --check --excluded /tmp/cxxdefs-lane/excluded-minus-cxxdefs.txt \
    emacs/lib/c++defs.h femacs/lib/include/c++defs.rs
  (14 C macros have several definitions (configuration branches), body not compared: _GL_BEGIN_NAMESPACE, …)
OK emacs/lib/c++defs.h -> femacs/lib/include/c++defs.rs
exit=0

# The frozen arm of every pair, as its own crate root:

$ cd femacs && CARGO_TARGET_DIR=/tmp/cxxdefs-lane/target cargo run -q -p xtask -- build-tu lib/c++defs.h
warning: crate `c__defs` should have a snake case name
built lib/include/c++defs.rs -> ../build/femacs/include_c++defs.o
exit=0   # the object is 336 bytes - the mirror defines no symbol, which is the header-mirror rule
         # (PORTING.md 0.1-2/0.1-9).  The warning is xtask's crate name: `crate_name("c++defs")`
         # sanitises the `+` to `c__defs`, and it is cosmetic (exit 0).

# The C++ arms are dead in the frozen configuration and `rustc` removes a `#[cfg]`-false item
# before parsing it, so the build above never looked at their text.  Two arm flips do:

$ cargo run -q -p xtask -- probe lib/include/c++defs.rs /tmp/cxxdefs-lane/x.rlib --crate-type=lib \
    -C panic=abort --cfg 'feature="cplusplus"' --cfg 'feature="cplusplus_gnulib_namespace"'
exit=0   # rlib 20,616 bytes: the C++ arms of the A-group and the two `extern "C"` pairs are live
$ cargo run -q -p xtask -- probe lib/include/c++defs.rs /tmp/cxxdefs-lane/x2.rlib --crate-type=lib \
    -C panic=abort --cfg 'feature="cplusplus"' --cfg 'feature="cplusplus_gnulib_namespace"' \
    --cfg 'feature="gnuc_optimize_without_clang"' --cfg 'feature="gnuc3_or_clang_gnulib_strict_checking"'
exit=0   # rlib 20,208 bytes: the other two arms of the `_GL_CXXALIASWARN*_2` pair are live as well

# The flips above ran on the mirror revision with md5 b9d444a2ad48ed9391361dfa23fdd970 (only
# comment wording changed afterwards).  The frozen `build-tu` was re-run on the shipped revision
# (md5 a8669f0d331ac3e3516e93fd43d60a65): exit=0, `../build/femacs/include_c++defs.o` 336 bytes,
# i.e. the shipped text compiles and defines no symbol.

# The negative control that says the probe really compiles this file (P-86 ⑦'s mutation reading),
# a plain `rustc` run on a copy with one live item broken:

$ sed 's/^pub const _GL_CXXDEFS_H: bool = true;/pub const _GL_CXXDEFS_H: bool = 1;/' \
    femacs/lib/include/c++defs.rs > /tmp/cxxdefs-lane/neg.rs
$ rustc --edition 2024 --crate-type=lib --crate-name cxxdefs_neg -C panic=abort \
    /tmp/cxxdefs-lane/neg.rs -o /tmp/cxxdefs-lane/neg.rlib
error[E0308]: mismatched types
  --> /tmp/cxxdefs-lane/neg.rs:75:33
75 | pub const _GL_CXXDEFS_H: bool = 1;
   |                          ----   ^ expected `bool`, found integer

# What the flips do *not* prove: nothing in the tree expands one of these macros, so the C++ arms
# are validated as balanced token trees only - the emitted tokens are never parsed or
# type-checked.  `--check`'s token comparison is the text-level counterpart, and the matchers
# read each C parameter as one token tree (`:tt`), which is exact for the C spellings the
# generated headers use and is listed in Owed 8.

# This lane's other gates, each run after the mirror and the ledger were written:

$ python3 tools/check-dead-file-coverage.py | tail -1
142 ledger(s) bound (a C range table), 0 failure(s)
# NOTE: this table is NOT one of the 142.  `tools/check-dead-file-coverage.py`'s `ledger_for` has
# no `include-` branch (check-ledger-ticks.py's copy does), so for `femacs/lib/include/*.rs` it
# looks for `lib-<stem>.md` / `<stem>.md` and never finds `include-<stem>.md`:
#   $ python3 -c "…; print(m.ledger_for('femacs/lib/include/c++defs.rs', set()))"  ->  None
# The table was therefore measured with the gate's own functions (parse_ranges +
# coverage_failures) against the reference's real blank lines - the file has 30 blank lines in
# C 1-367 and 18 of them sit between blocks, outside the table's rows:
#   20 range row(s), ascending, non-overlapping, the last ending at 367 -> 0 failures, 18 slack
# See Owed 6.

$ python3 tools/check-port-lines.py | tail -2
stale    lib/c++defs.h: no longer under the threshold
0 file(s) under ten percent, 0 reference line(s); 2 ratchet row(s), 0 new, 2 stale
# (the second stale row, src/w32xfns.c, is another lane's in-flight file, not this one's)

$ python3 tools/check-ledger-ticks.py | tail -1
238 ledger(s) bound (a function tick to compare), 0 failure(s), … 0 row(s) without a ledger

$ python3 tools/check-declarations.py | tail -1
exit=0 (no finding)
$ python3 tools/check-mirror-dupes.py | tail -1
17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)
$ python3 tools/check-citations.py | tail -1
check-citations: 676 file(s) scanned, 0 drift, 0 range, 1812 external, 36 allowed, 0 stale
# this file contributes no citation: `// C <n>` has no path, and the `.rs` mirrors the header
# prose names are not citation paths the gate reads.
```

## Deviations

1. **C 21-27's `#if defined __cplusplus && defined GNULIB_NAMESPACE` has no cfg'd arm pair for the
   two namespace names** (the carve-out P-86 ① allows for a name Rust cannot spell).  C 22's C++
   arm is `namespace GNULIB_NAMESPACE {` and C 23's is `}`; the frozen arm (C 25-26) defines both
   names empty.  Rust has no namespace blocks, an item cannot carry an unbalanced `{`/`}`, and a
   `macro_rules!` body must be a balanced token tree - `}` alone is not a token tree at all, and
   an unbalanced `{` would make the definition swallow whatever follows it.  The substitute is the
   same-name marker the (nonexistent) use site needs, `pub const _GL_BEGIN_NAMESPACE: () = ();` /
   `pub const _GL_END_NAMESPACE: () = ();` (the `lib/include/_Noreturn.rs` shape), with the C++
   arm's text quoted at the item, exhaustively listed in the C line-range table, and appended to
   `docs/ports/excluded-macros.txt` (two new rows superseding the two older rows for these names).
   Both arms are accounted for; only the C++ arm's *form* is prose instead of a `#[cfg]`d item.
2. **C 17-18's guard is a `const`, not a `macro_rules!`** - the `_GL_CXXDEFS_H` body is empty, and
   an empty-expansion macro body is the one thing `tools/extract-abi.py` refuses (the
   `lib/include/gnulib_config.rs` `_GL_INLINE`/`_GL_EXTERN_INLINE` markers are the precedent).
3. **Every other `#if`'s arms are carried in C's order** (C's own arm first, gated on the
   condition's lower-cased feature; the `#else` arm ungated last), so the extractor - which reads
   a name's first definition - compares C's text while `rustc` compiles the frozen arm
   (`lib/include/fcntl.in.rs`'s rule).  The names and arms C defines *only* inside a C++ arm -
   `_GL_CXXALIASWARN_1` (C 321-322), `_GL_CXXALIASWARN1_1` (C 349-350) and the five warning arms
   of `_GL_CXXALIASWARN_2` / `_GL_CXXALIASWARN1_2` (C 326-335, C 354-360) - have no ungated twin,
   because C's `#else` arm does not define them either: the frozen configuration has no such
   macro, exactly as C does not.
4. **Token pasting is spelled as two tokens.**  C's `rpl_##func`, `_##func`, `_gl_##func##_wrapper`
   become `rpl_ $func`, `_ $func`, `_gl_ $func _wrapper` (a Rust `macro_rules!` has no `##`), and
   C's `__VA_ARGS__` becomes the `$($args:tt)*` tail (PORTING.md 2.1) - the three
   `docs/ports/macro-body-notes.tsv` rows this lane appended record the latter, which is the one
   token of a body that has no counterpart.
5. **The `_GL_CXXALIAS_*` names are declaration spelling, not runtime constructs**, so their use
   sites are the units' own declarations and comments (`fcntl.in.rs` 274-276 for creat/open/openat,
   `time.in.rs` 156, `unistd.in.rs` 24, `sys_stat.in.rs` 43/175, `dirent.in.rs` 48/115/134-135,
   `string.in.rs` 28-29/71, `stdio.in.rs` 27-37, `sys_time.in.rs` 27/50, `sys_select.in.rs`
   26/53, `inttypes.in.rs` 19/65-67).  No unit `include!`s this mirror, measured: the only
   reference files that name the header are `lib/gnulib.mk.in` and the `build-aux/snippet`
   machinery, so its compile evidence is that it stands as its own crate root (Evidence).
6. **The features are new names, not C's macro names**: `cplusplus` (registered),
   `cplusplus_gnulib_namespace`, `gnuc_optimize_without_clang` and
   `gnuc3_or_clang_gnulib_strict_checking` (three new registrations; the last two are also used
   negated, to select the arms before/after their chains).  Their C conditions are in the Gate
   mapping, one row each.

## Owed (nothing here is claimed as done)

1. **Line-drift row (suggested, for the integrator to add to `docs/ports/line-drift-tolerance.tsv`)**:
   `c++defs\t0.39\tC 367 vs the port's 507 (38.1%): C's comment blocks are carried verbatim as prose
   (C 29-86, 107-124, 140-150 and the rest), and a `macro_rules!` arm pair costs the matcher line
   plus the item's own lines where C spends two `#define` lines - and the C++ arms are one physical
   line each, continuations joined as `lib/include/cdefs.rs` does.\ttotal`
   Today the check does not run for this file at all: `tools/extract-abi.py`'s drift step is
   skipped for `.h` mirrors (`is_header_like`), so the row is for the day that stops being true.
2. **Register the three new features** in `femacs/xtask/Cargo.toml`'s `[features]` table (the
   integrator's file, not this lane's): `cplusplus_gnulib_namespace`,
   `gnuc_optimize_without_clang`, `gnuc3_or_clang_gnulib_strict_checking`, plus their
   `docs/baseline/feature-coverage.tsv` rows (`tools/feature-coverage.py --write`).
3. **`docs/baseline/port-lines-pending.tsv`'s row** `lib/c++defs.h\t367\t22\t36\tknown-unported
   target (plan batch F)` is now stale - the ratchet says so (`stale lib/c++defs.h: no longer
   under the threshold`).  Re-run `python3 tools/check-port-lines.py --write-pending`.
4. **`docs/port-map.md`'s row 64 and its progress totals** are generated: re-run
   `tools/gen-port-map.py` so the target's line count (22) and the state follow the mirror (507).
5. **The two append-only tables**: this lane appended 2 rows to `docs/ports/excluded-macros.txt`
   (the superseded `_GL_BEGIN_NAMESPACE` / `_GL_END_NAMESPACE` rows) and 3 rows to
   `docs/ports/macro-body-notes.tsv` (C's `__VA_ARGS__` in the three variadic macros).  Rows
   204-226 of `excluded-macros.txt` were written when the file had no Rust item at all; they are
   now stale for the 21 names the mirror holds (their reasons say "nothing below needs a Rust
   item").  Only the integrator can delete rows - this lane may append only - so they are left in
   place on purpose; with them deleted, `--check` stays `OK` because the three note rows above
   carry the one token class that cannot match (Evidence).
6. **`tools/check-dead-file-coverage.py` cannot bind a `lib/include/*.rs` ledger.**  Its
   `ledger_for` (unlike `check-ledger-ticks.py`'s, whose docstring it cites) lacks the
   `is_include` branch, so it looks for `lib-<stem>.md` / `<stem>.md` and never finds
   `include-<stem>.md`: it returns `None` for `femacs/lib/include/c++defs.rs` *and* for the
   already-ported `femacs/lib/include/fcntl.in.rs`.  The gate printed `142 ledger(s) bound, 0
   failure(s)` - this file among the unbound.  Until that is fixed the reading of this table is
   manual; the Evidence records the same measurement taken with the gate's own functions
   (20 rows, 0 failures) and could be turned into the gate's own output by copying
   `check-ledger-ticks.py`'s `ledger_for` branch across.
7. **`build-tu`'s crate name for this stem warns** (`crate c__defs should have a snake case name`)
   because `crate_name` maps `+` to `_`; the build is exit 0 and the object is 336 bytes.  No
   action needed unless the tree wants the warning gone.
8. **The matchers read each C parameter as one token tree** (`$rettype:tt`, `$parameters:tt`, …).
   C's parameters are token *sequences* (a `rettype` can be `char **restrict`, a `parameters` can
   be `(int, char *) __attribute__((…))`), so a real invocation of one of these macros would need
   `$($rettype:tt)*`-style captures - which `macro_rules!` cannot mix freely with the following
   fragment.  Nothing invokes them today (no unit includes the mirror), so this is recorded rather
   than fixed; a consumer that ever calls one should re-open the matcher.
9. **Nothing expands the C++ arms**, so their text is validated as balanced token trees only (the
   two arm-flip probes and the frozen build): the *emitted* tokens are never parsed or
   type-checked, and C++-only spellings (`namespace`, `reinterpret_cast<type>(…)`, `inline
   operator type ()`) are not checked against any C++ compiler by this lane.
