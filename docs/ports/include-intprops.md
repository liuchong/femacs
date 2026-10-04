# emacs/lib/intprops.h (377 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 38
- [x] EXPR_SIGNED  (line 47, body 559efa8d8827)
- [x] INT_ADD_OK  (line 373, body 61dc0778ff49)
- [x] INT_ADD_OVERFLOW  (line 316, body 5f4a6b9893f5)
- [x] INT_ADD_RANGE_OVERFLOW  (line 156, body 6749834db5c5)
- [x] INT_ADD_WRAPV  (line 342, body 1d423fe55178)
- [x] INT_BITS_STRLEN_BOUND  (line 97, body 49cd01af3277)
- [x] INT_BUFSIZE_BOUND  (line 114, body 6b184fade9e8)
- [x] INT_DIVIDE_OVERFLOW  (line 323, body 8b8dfbf20f56)
- [x] INT_DIVIDE_RANGE_OVERFLOW  (line 192, body d6f34bb24289)
- [x] INT_LEFT_SHIFT_OVERFLOW  (line 327, body 0612ae05148a)
- [x] INT_LEFT_SHIFT_RANGE_OVERFLOW  (line 210, body 14efcddb655c)
- [x] INT_MULTIPLY_OK  (line 375, body 95633bb59c5f)
- [x] INT_MULTIPLY_OVERFLOW  (line 321, body 98fc906b9d44)
- [x] INT_MULTIPLY_RANGE_OVERFLOW  (line 177, body b61df5f73478)
- [x] INT_MULTIPLY_WRAPV  (line 344, body 0866c6cc1445)
- [x] INT_NEGATE_OVERFLOW  (line 320, body d977b7b2a01f)
- [x] INT_NEGATE_RANGE_OVERFLOW  (line 170, body 6075fd466fab)
- [x] INT_PROMOTE  (line 55, body cd0d4b9433e2)
- [x] INT_REMAINDER_OVERFLOW  (line 325, body 68292833cb25)
- [x] INT_REMAINDER_RANGE_OVERFLOW  (line 200, body 8753f0ea4bfb)
- [x] INT_STRLEN_BOUND  (line 108, body 42a0eb2dbeed)
- [x] INT_SUBTRACT_OK  (line 374, body c852a8524ec0)
- [x] INT_SUBTRACT_OVERFLOW  (line 318, body f640e830b40d)
- [x] INT_SUBTRACT_RANGE_OVERFLOW  (line 163, body b57fc0cffabb)
- [x] INT_SUBTRACT_WRAPV  (line 343, body 5e1a9ffd615d)
- [x] TYPE_IS_INTEGER  (line 34, body 07bad2282c06)
- [x] TYPE_MAXIMUM  (line 84, body 4e9ec2549cdf)
- [x] TYPE_MINIMUM  (line 76, body bf49654cb29f)
- [x] TYPE_SIGNED  (line 43, body bda122eb2938)
- [x] TYPE_WIDTH  (line 63, body dcf2c2590a5c)
- [x] _GL_ADD_OVERFLOW  (line 227, body 76143c4ae41c)
- [x] _GL_BINARY_OP_OVERFLOW  (line 335, body 98fe78a2bb27)
- [x] _GL_DIVIDE_OVERFLOW  (line 241, body 93bb7d6faa39)
- [x] _GL_INTPROPS_H  (line 19, body da39a3ee5e6b)
- [x] _GL_MULTIPLY_OVERFLOW  (line 237, body a91635575d0d)
- [x] _GL_REMAINDER_OVERFLOW  (line 245, body ba328b91b443)
- [x] _GL_SUBTRACT_OVERFLOW  (line 232, body fb9f7333abed)
- [x] _GL_UNSIGNED_NEG_MULTIPLE  (line 254, body 982ac68565c2)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **Blind reading, with the two disagreements named and the criterion written down.**  This file
  is *not* in `docs/baseline/config/cond-eval-verified.txt`, and after status 3.202 (the branch
  marker's hyphen bug fix; this lane reported the finding from readutmp.h) the reading is
  `python3 tools/check-cond-eval.py lib/intprops.h` -> `4 conditions, 0 unknown,
  2 disagreements [NOT VERIFIED]`:

  | line | condition | clang | evaluator | why the table cannot decide it |
  | --- | --- | --- | --- | --- |
  | 28 | `#if (__STDC_VERSION__ < 201112 \|\| (defined _MSC_VER && _MSC_VER < 1944) \|\| _GL__GENERIC_BOGUS)` | True | False | `_GL__GENERIC_BOGUS` is defined by the *included* `intprops-internal.h` (its C 34-38 arm), not by config.h |
  | 66 | the same condition | True | False | the same |

  `_GL__GENERIC_BOGUS` is **1** for the frozen clang - measured, the `-dM` diff below shows the
  surviving `#define _GL__GENERIC_BOGUS 1` - so both conditions are true and this file takes the
  **non-`_Generic`** side of C 28-40 and C 66-92.  The mirror therefore carries both arms of
  each `#if` (P-51) with the measured arm live; no text was written to make a gate green.

- **Live arms, per name** (criterion: the `clang -dM -E … -include config.h [-include
  intprops.h]` control diff, commands in Evidence; the control defines *none* of these names):

  | C lines | name | live arm |
  | --- | --- | --- |
  | 28-40 | `TYPE_IS_INTEGER` | C 30's `((t) 1.5 == 1)` (C 34-39's `_Generic` dead) |
  | 66-92 | `TYPE_MINIMUM`, `TYPE_MAXIMUM` | C 68's `((t) ~ TYPE_MAXIMUM (t))` and C 69-72's `((t) (! TYPE_SIGNED (t) ? …))` (C 76-90's `_Generic` dead) |
  | 219-240 | `_GL_ADD_OVERFLOW`, `_GL_SUBTRACT_OVERFLOW`, `_GL_MULTIPLY_OVERFLOW` | C 227-239's `#else` arms - **Apple clang answers 0 for `__builtin_mul_overflow_p`**, so `_GL_HAS_BUILTIN_OVERFLOW_P` is 0 (C 220-225's `__builtin_*_overflow_p` arms dead) |
  | 43, 47, 55, 63 | `TYPE_SIGNED`, `EXPR_SIGNED`, `INT_PROMOTE`, `TYPE_WIDTH` | unconditional (measured values match C exactly) |
  | 95-114 | `INT_BITS_STRLEN_BOUND`, `INT_STRLEN_BOUND`, `INT_BUFSIZE_BOUND` | unconditional |
  | 154-212 | the seven `*_RANGE_OVERFLOW` predicates | unconditional |
  | 241-255 | `_GL_DIVIDE_OVERFLOW`, `_GL_REMAINDER_OVERFLOW`, `_GL_UNSIGNED_NEG_MULTIPLE` | unconditional |
  | 314-327, 333-336, 342-345, 373-375 | the `INT_*_OVERFLOW` / `*_WRAPV` / `*_OK` families and `_GL_BINARY_OP_OVERFLOW` | unconditional |

- **Demand: 38 macros, 0 constants, 0 types.**  `--json` reports `constants: {}` and
  `containers: []`, so the ledger's checklist is 38 macro rows and nothing else; 32 have one
  definition (compared by body, all transcribed as C's text) and 6 have several (presence only).

- **All bodies are C's frozen text** (arguments as `$`-metavariables).  These macros have no
  expanding consumer in the port: the units that need the semantics write the Rust primitive at
  the use site (`lib/strftime.rs` 542's `INT_STRLEN_BOUND` and `lib/openat-proc.rs` 36-37 are the
  worked examples).  Consequences, both measured in this batch: a `?:` chain, a `sizeof`, a
  `__typeof__`, a `(t) 1.5 == 1` cast and C's `!` stay C's text inside an unexpanded
  `macro_rules!`, and C's backslash line-continuations are **dropped** (not Rust tokens:
  `error: unknown start of token: \`).  `_GL_BINARY_OP_OVERFLOW`'s third argument is used as a
  *macro* in C (`op_result_overflow (a, b, …)`), which a Rust metavariable cannot do, so its body
  is C's text too and nothing expands it.

- **Dead arms under the feature naming the condition that selects them** (P-51; the status 3.9
  note applies): `no_generic_bogus` (C 34-39, C 76-90) and `builtin_overflow_p` (C 220-225) -
  each name's arms are mutually exclusive, and the features follow the names
  `include/intprops-internal.rs` already uses for the same conditions.

- **The one include is C's own.**  C 21's `#include "intprops-internal.h"` is
  `include!("intprops-internal.rs")`; `xtask`'s include-graph dedupe makes a unit that pulls in
  both halves include it once (PORTING.md 0.1-4).  The mirror needs no scalar from an includer,
  so `build-tu lib/intprops.h` compiles it as its own crate root (with its inner half expanded
  into it).

- **Ten affected names elsewhere are untouched**: `TYPE_SIGNED`/`EXPR_SIGNED`/`TYPE_WIDTH`/
  `INT_STRLEN_BOUND` are the names the ported units currently spell out at their use sites (the
  `strftime.rs`/`openat-proc.rs`/`focused` precedent); re-pointing them to this mirror is a
  unit-side change for those files' owners, and nothing here was changed to force it.

## Evidence

```
$ python3 tools/check-cond-eval.py lib/intprops.h
DISAGREE lib/intprops.h:28 `if (__STDC_VERSION__ < 201112 || (defined _MSC_VER && _MSC_VER < 1944)  || _GL__GENERIC_BOGUS)` — clang True, evaluator False
DISAGREE lib/intprops.h:66 `if (__STDC_VERSION__ < 201112 || (defined _MSC_VER && _MSC_VER < 1944)  || _GL__GENERIC_BOGUS)` — clang True, evaluator False
lib/intprops.h: 4 conditions, 0 unknown, 2 disagreements [NOT VERIFIED]

$ python3 tools/extract-abi.py emacs/lib/intprops.h
# emacs/lib/intprops.h (377 lines)
## Subrs (DEFUN) - 0
## Functions defined - 0
## Macros defined - 38   (38/38 ticked above)
## Types and their fields - 0
## File-scope objects - 0
$ python3 tools/extract-abi.py --json emacs/lib/intprops.h | …    # constants: {}   containers: []

$ python3 tools/extract-abi.py --check emacs/lib/intprops.h femacs/lib/include/intprops.rs
  (6 C macros have several definitions (configuration branches), body not compared: TYPE_IS_INTEGER, TYPE_MAXIMUM, TYPE_MINIMUM, _GL_ADD_OVERFLOW, _GL_MULTIPLY_OVERFLOW, _GL_SUBTRACT_OVERFLOW)
OK emacs/lib/intprops.h -> femacs/lib/include/intprops.rs
# no `- ` problem line (`| grep -c '^- '` -> 0)

# the criterion: the control defines none of these; intprops.h's live text is this
$ INC="-I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib"
$ P='^#define (TYPE_IS_INTEGER|TYPE_MINIMUM|TYPE_MAXIMUM|_GL_ADD_OVERFLOW|_GL_SUBTRACT_OVERFLOW|_GL_MULTIPLY_OVERFLOW|EXPR_SIGNED|INT_PROMOTE|INT_BITS_STRLEN_BOUND|INT_STRLEN_BOUND|TYPE_WIDTH|TYPE_SIGNED)\b'
$ clang -dM -E -std=gnu23 $INC -include config.h -x c /dev/null | grep -E "$P"      # no output
$ clang -dM -E -std=gnu23 $INC -include config.h -include intprops.h -x c /dev/null | grep -E "$P"
#define EXPR_SIGNED(e) _GL_EXPR_SIGNED (e)
#define INT_BITS_STRLEN_BOUND(b) (((b) * 146 + 484) / 485)
#define INT_PROMOTE(e) (+ (e))
#define INT_STRLEN_BOUND(t) (INT_BITS_STRLEN_BOUND (TYPE_WIDTH (t) - _GL_SIGNED_TYPE_OR_EXPR (t)) + _GL_SIGNED_TYPE_OR_EXPR (t))
#define TYPE_IS_INTEGER(t) ((t) 1.5 == 1)
#define TYPE_MAXIMUM(t) ((t) (! TYPE_SIGNED (t) ? (t) -1 : ((((t) 1 << (TYPE_WIDTH (t) - 2)) - 1) * 2 + 1)))
#define TYPE_MINIMUM(t) ((t) ~ TYPE_MAXIMUM (t))
#define TYPE_SIGNED(t) _GL_TYPE_SIGNED (t)
#define TYPE_WIDTH(t) _GL_TYPE_WIDTH (t)
#define _GL_ADD_OVERFLOW(a,b,min,max) ((min) < 0 ? INT_ADD_RANGE_OVERFLOW (a, b, min, max) : (a) < 0 ? (b) <= (a) + (b) : (b) < 0 ? (a) <= (a) + (b) : (a) + (b) < (b))
#define _GL_MULTIPLY_OVERFLOW(a,b,min,max) (((min) == 0 && (((a) < 0 && 0 < (b)) || ((b) < 0 && 0 < (a)))) || INT_MULTIPLY_RANGE_OVERFLOW (a, b, min, max))
#define _GL_SUBTRACT_OVERFLOW(a,b,min,max) ((min) < 0 ? INT_SUBTRACT_RANGE_OVERFLOW (a, b, min, max) : (a) < 0 ? 1 : (b) < 0 ? (a) - (b) <= (a) : (a) < (b))
# (the `_Generic` and `__builtin_*_overflow_p` arms appear nowhere: dead)

$ cd femacs && CARGO_TARGET_DIR=… cargo run -q -p xtask -- build-tu lib/intprops.h
built lib/include/intprops.rs -> ../build/femacs/include_intprops.o
```
