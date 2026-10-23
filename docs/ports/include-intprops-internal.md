# emacs/lib/intprops-internal.h (413 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 29
- [x] LLONG_MAX  (line 94, body fca493fe7470)
- [x] LLONG_MIN  (line 95, body df887d1338b2)
- [x] _GL_EXPR_SIGNED  (line 69, body 8f1f4cc828b2)
- [x] _GL_HAS_BUILTIN_ADD_OVERFLOW  (line 144, body b6589fc6ab0d)
- [x] _GL_HAS_BUILTIN_MUL_OVERFLOW  (line 152, body 7ddbad474819)
- [x] _GL_HAS_BUILTIN_OVERFLOW_P  (line 165, body 4c204927eca2)
- [x] _GL_HAVE___TYPEOF__  (line 112, body b6589fc6ab0d)
- [x] _GL_INTPROPS_INTERNAL_H  (line 19, body da39a3ee5e6b)
- [x] _GL_INT_ADD_RANGE_OVERFLOW  (line 364, body f01fc668986e)
- [x] _GL_INT_ADD_WRAPV  (line 184, body 316764f0de9e)
- [x] _GL_INT_CONVERT  (line 47, body 5d760e572b9e)
- [x] _GL_INT_MAXIMUM  (line 85, body e68223ccce9b)
- [x] _GL_INT_MINIMUM  (line 81, body 66829e820f41)
- [x] _GL_INT_MULTIPLY_RANGE_OVERFLOW  (line 391, body a17a633247c8)
- [x] _GL_INT_MULTIPLY_WRAPV  (line 207, body c05bdbe22745)
- [x] _GL_INT_NEGATE_OVERFLOW  (line 334, body 642fa8e6f9fb)
- [x] _GL_INT_NEGATE_RANGE_OVERFLOW  (line 128, body cf489e9d72cc)
- [x] _GL_INT_OP_CALC  (line 319, body 8dac520f448d)
- [x] _GL_INT_OP_WRAPV  (line 274, body a04a7e57e96d)
- [x] _GL_INT_OP_WRAPV_LONGISH  (line 304, body 3a26774cc19a)
- [x] _GL_INT_OP_WRAPV_SMALLISH  (line 264, body 296b29d6e07d)
- [x] _GL_INT_OP_WRAPV_VIA_UNSIGNED  (line 355, body 65c665ba2c3b)
- [x] _GL_INT_SUBTRACT_RANGE_OVERFLOW  (line 379, body 38eb6f57d649)
- [x] _GL_INT_SUBTRACT_WRAPV  (line 186, body a660fdf052b4)
- [x] _GL_SIGNED_INT_MAXIMUM  (line 89, body 80ef84f9ae4f)
- [x] _GL_SIGNED_TYPE_OR_EXPR  (line 122, body 356a192b7913)
- [x] _GL_TYPE_SIGNED  (line 59, body cef46a241d44)
- [x] _GL_TYPE_WIDTH  (line 77, body 1650cb3b87a2)
- [x] _GL__GENERIC_BOGUS  (line 37, body b6589fc6ab0d)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The gate reads this file blind - by measurement, and the gate cannot arbitrate its `#if`s.**
  `lib/intprops-internal.h` is not in `docs/baseline/config/cond-eval-verified.txt`, and after
  status 3.202's fix of the evaluator's branch marker - the stem's `-` used to make clang's
  answer meaningless, which was the finding this lane reported from readutmp.h - the reading is
  `python3 tools/check-cond-eval.py lib/intprops-internal.h` -> `24 conditions, 1 unknown,
  7 disagreements [NOT VERIFIED]`.  All seven are real, and **none of them is about a
  configuration macro**, so no mirror text could make the file verified:

  | line | condition | clang | evaluator | why a table cannot decide it |
  | --- | --- | --- | --- | --- |
  | 119 | `#if _GL_HAVE___TYPEOF__` | True | False | the value is this file's *own* C 105-113 arm, whose condition is the unknown below |
  | 137 | `#elif defined __has_builtin` | True | False | `__has_builtin` is a *compiler* builtin, not a config.h macro |
  | 162 | `#elif defined __has_builtin` | True | False | the same |
  | 168 | `#if (!defined _GL_STDCKDINT_H && 202311 <= __STDC_VERSION__ && ! (_GL_HAS_BUILTIN_ADD_OVERFLOW && _GL_HAS_BUILTIN_MUL_OVERFLOW))` | False | True | `_GL_HAS_BUILTIN_*` hold `__has_builtin (…)` expressions, not numbers |
  | 177 | `#if _GL_HAS_BUILTIN_ADD_OVERFLOW` | True | False | the same |
  | 189 | `#if _GL_HAS_BUILTIN_MUL_OVERFLOW` | True | False | the same |
  | 258 | `#if _GL_HAVE___TYPEOF__` | True | False | the same as C 119 |

  The one unknown is C 105's five-way `__typeof__` probe
  (`python3 tools/check-cond-eval.py --show-unknown lib/intprops-internal.h` names it); the
  evaluator keeps its branch live (conservative), clang takes it.  The mirror therefore carries
  **both arms of every `#if`** (P-51) and no arm was chosen to make a gate green.

- **The criterion for the arms is a direct measurement** that does not use the branch-marker
  mechanism at all: `clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I
  emacs/src -I emacs/lib -include config.h [-include intprops-internal.h] -x c /dev/null`,
  diffed against the `config.h`-only control (51 lines vs 89; `comm -13`).  The live arms, with
  the C lines they come from:

  | C lines | live value |
  | --- | --- |
  | 34-38 | `_GL__GENERIC_BOGUS 1` (`defined __clang__`) |
  | 53-67 | `_GL_TYPE_SIGNED (t) (! ((t) 0 < (t) -1))` - the `_Generic` arm (64-67) is dead because `_GL__GENERIC_BOGUS` is 1 |
  | 93-96 | dead (`<limits.h>` already defines `LLONG_MAX`/`LLONG_MIN`; the surviving spelling is `__LONG_LONG_MAX__`, not `__INT64_MAX`) |
  | 105-113 | `_GL_HAVE___TYPEOF__ 1` (`__clang_major__` 21 >= 4) |
  | 119-126 | `_GL_SIGNED_TYPE_OR_EXPR (t) _GL_TYPE_SIGNED (__typeof__ (t))`; C 125's `1` dead |
  | 138-146 | `_GL_HAS_BUILTIN_ADD_OVERFLOW __has_builtin (__builtin_add_overflow)` |
  | 148-153 | `_GL_HAS_BUILTIN_MUL_OVERFLOW _GL_HAS_BUILTIN_ADD_OVERFLOW` (21 is not < 21) |
  | 155-166 | `_GL_HAS_BUILTIN_OVERFLOW_P __has_builtin (__builtin_mul_overflow_p)` - **Apple clang answers 0 for `__builtin_mul_overflow_p`**, which is why C 336-337's range arm is the live `_GL_INT_NEGATE_OVERFLOW` |
  | 173-188 | `_GL_INT_ADD_WRAPV`/`_GL_INT_SUBTRACT_WRAPV` are the `__builtin_*_overflow` arms |
  | 191-214 | `_GL_INT_MULTIPLY_WRAPV` is C 197-203's GCC-bug-91450 wrapper (`!defined __clang__` is false) |
  | 260-272 | `_GL_INT_OP_WRAPV_SMALLISH` is C 264's `__typeof__` arm |
  | 219-303 | `_GL_INT_OP_WRAPV` is C 274's `sizeof` chain (C 219's `_Generic` arm is dead) |
  | 304-311 | `_GL_INT_OP_WRAPV_LONGISH` is the `#ifdef LLONG_MAX` arm |
  | 333-338 | `_GL_INT_NEGATE_OVERFLOW` is the range arm (see the `__builtin_mul_overflow_p` row) |
  | 46-129, 319-321, 355-356, 364-407 | `_GL_INT_CONVERT`, `_GL_EXPR_SIGNED`, `_GL_TYPE_WIDTH`, the three `*_MINIMUM`/`*_MAXIMUM`, `_GL_INT_NEGATE_RANGE_OVERFLOW`, `_GL_INT_OP_CALC`, `_GL_INT_OP_WRAPV_VIA_UNSIGNED` and the three `*_RANGE_OVERFLOW` are unconditional (and are the 15 body-compared macros) |

  One attribution to keep in mind: the same diff prints `_GL_COB4/8/16/32/64/128`,
  `_GL_INTEGER_WIDTH`, `LLONG_WIDTH` and `_GL_LIMITS_H`, which come from the *include chain*
  (`<limits.h>`) and belong to `include/limits.in.rs` - the mirror must not carry them.

- **The demand is 29 macros, 0 types, 0 constants.**  `--check` against an empty file prints 29
  `missing macro` lines and nothing else; `--json`'s `containers` is empty.  Fifteen of the 29
  have one definition and are compared *by body*; the other fourteen have several definitions
  (configuration branches), so the gate asks only for their presence.  The three constants the
  extractor lists (`_GL__GENERIC_BOGUS` 0, `_GL_HAVE___TYPEOF__` 0, `_GL_HAS_BUILTIN_ADD_OVERFLOW`
  0) are all `dup>1`, which the constant rule skips.

- **Every body is C's frozen text.**  These macros have no expanding consumer in the port: the
  public names are `intprops.h`'s, and the units that need the semantics write the Rust
  primitive at the use site (`checked_*`/`overflowing_*`/`is_signed`, the
  `strftime.rs`/`openat-proc.rs` precedent).  So the mirror keeps C's tokens rather than
  inventing a Rust shape the gate would stop comparing - the policy the parent ratified for
  readutmp.h's `HAVE_STRUCT_XTMP_*`.  Consequences, both measured: a `?:` chain, a `sizeof`, a
  `__typeof__` and a `(void)` cast stay C's text inside an unexpanded `macro_rules!`, and C's
  backslash line-continuations are **dropped** because they are not Rust tokens
  (`error: unknown start of token: \`; 116 of them; the token gate compares identifiers and
  numbers, which a continuation never was).

- **The dead arms, each under the feature naming the condition that selects it** (P-51; none of
  these is in `xtask dead-arms`' FLIPS list - they are compiler/platform predicates, status
  3.9): `no_clang` (C 34-38's `0`), `no_generic_bogus` (C 64-67's `_Generic` `_GL_TYPE_SIGNED`
  and C 219-259's `_Generic` `_GL_INT_OP_WRAPV`), `openvms` (C 94-95's `LLONG_MAX`/`LLONG_MIN`),
  `no_typeof` (C 111-112's `0`, C 125's `1`, C 269-272's `_GL_INT_OP_WRAPV_SMALLISH`),
  `edg` (C 139-140, C 156), `gcc_lt_7` (C 144-145), `no_has_builtin_gcc_lt_7` (C 146),
  `clang_lt_21` (C 149-151), `no_has_builtin` (C 166), `ckd` (C 182-183, C 208-209),
  `no_builtin_add_overflow` (C 185-188), `gcc_9_3` (C 194), `no_builtin_mul_overflow`
  (C 210-214), `builtin_overflow_p` (C 335), `no_llong_max` (C 313-318).  Each arm was checked
  against the table so that exactly one definition can be compiled (the earlier
  `all (not (feature = …))` form for C 146 was replaced by the single feature above for that
  reason).

- **Recorded here, not carried.**  C 24-29's and C 41-43's `#pragma GCC diagnostic` lines (a
  `#pragma` has no Rust item; both are dead anyway - the target is clang and
  `__STDC_VERSION__` is 202311), C 21's `#include <limits.h>` and C 30's author line, C 168-170's
  `#include <stdckdint.h>` (dead for clang, live for the evaluator, and there is no
  `stdckdint.h` mirror), and C 409's `#endif /* _GL_INTPROPS_INTERNAL_H */`.

- **The mirror is its own translation unit**: 29 macros plus one marker constant, no scalar from
  an includer and no `include!`, so `build-tu lib/intprops-internal.h` reaches it as the crate
  root and compiles.

## Evidence

```
$ python3 tools/check-cond-eval.py lib/intprops-internal.h
DISAGREE lib/intprops-internal.h:119 `if _GL_HAVE___TYPEOF__` — clang True, evaluator False
DISAGREE lib/intprops-internal.h:137 `elif defined __has_builtin` — clang True, evaluator False
DISAGREE lib/intprops-internal.h:162 `elif defined __has_builtin` — clang True, evaluator False
DISAGREE lib/intprops-internal.h:168 `if (!defined _GL_STDCKDINT_H && 202311 <= __STDC_VERSION__  && ! (_GL_HAS_BUILTIN_ADD_OVERFLOW && _GL_HAS_BUILTIN_MUL_OVERFLOW))` — clang False, evaluator True
DISAGREE lib/intprops-internal.h:177 `if _GL_HAS_BUILTIN_ADD_OVERFLOW` — clang True, evaluator False
DISAGREE lib/intprops-internal.h:189 `if _GL_HAS_BUILTIN_MUL_OVERFLOW` — clang True, evaluator False
DISAGREE lib/intprops-internal.h:258 `if _GL_HAVE___TYPEOF__` — clang True, evaluator False
lib/intprops-internal.h: 24 conditions, 1 unknown, 7 disagreements [NOT VERIFIED]
$ python3 tools/check-cond-eval.py --show-unknown lib/intprops-internal.h | grep UNKNOWN
UNKNOWN lib/intprops-internal.h:105 `if ((defined __GNUC__ && 2 <= __GNUC__)  || (defined __clang_major__ && 4 <= __clang_major__)  || (defined __IBMC__ && 1210 <= __IBMC__ && defined __IBM__TYPEOF__)  || (defined __SUNPRO_C && 0x5110 <= __SUNPRO_C && !__STDC__)  || (defined _MSC_VER && 1939 <= _MSC_VER && !defined __cplusplus))`

$ python3 tools/extract-abi.py emacs/lib/intprops-internal.h
# emacs/lib/intprops-internal.h (413 lines)
## Subrs (DEFUN) - 0
## Functions defined - 0
## Macros defined - 29   (29/29 ticked above)
## Types and their fields - 0
## File-scope objects - 0

$ python3 tools/extract-abi.py --check emacs/lib/intprops-internal.h /tmp/g188i/empty.rs   # the demand
- missing macro LLONG_MAX
... (29 lines, all `missing macro`)
FAIL emacs/lib/intprops-internal.h -> /tmp/g188i/empty.rs
# no `missing constant` and no `missing type` line

$ python3 tools/extract-abi.py --check emacs/lib/intprops-internal.h lib/include/intprops-internal.rs
  (14 C macros have several definitions (configuration branches), body not compared: _GL_HAS_BUILTIN_ADD_OVERFLOW, _GL_HAS_BUILTIN_MUL_OVERFLOW, _GL_HAS_BUILTIN_OVERFLOW_P, _GL_HAVE___TYPEOF__, _GL_INT_ADD_WRAPV, _GL_INT_MULTIPLY_WRAPV ...)
OK emacs/lib/intprops-internal.h -> lib/include/intprops-internal.rs
# no `- ` problem line (`| grep -c '^- '` -> 0)

# the criterion: the control diff
$ INC="-I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib"
$ clang -dM -E -std=gnu23 $INC -include config.h -x c /dev/null | grep -E '^#define (_GL_|LLONG_)' | wc -l
51
$ clang -dM -E -std=gnu23 $INC -include config.h -include intprops-internal.h -x c /dev/null | grep -E '^#define (_GL_|LLONG_)' | wc -l
89
$ comm -13 <(…control…) <(…with-file…) | grep -E '_GL__GENERIC_BOGUS|_GL_TYPE_SIGNED|_GL_HAVE___TYPEOF__|_GL_HAS_BUILTIN|_GL_INT_ADD_WRAPV|_GL_INT_MULTIPLY_WRAPV|_GL_INT_SUBTRACT_WRAPV|_GL_INT_NEGATE_OVERFLOW|_GL_INT_OP_WRAPV|LLONG_M'
#define LLONG_MAX __LONG_LONG_MAX__
#define LLONG_MIN (-__LONG_LONG_MAX__-1LL)
#define _GL_HAS_BUILTIN_ADD_OVERFLOW __has_builtin (__builtin_add_overflow)
#define _GL_HAS_BUILTIN_MUL_OVERFLOW _GL_HAS_BUILTIN_ADD_OVERFLOW
#define _GL_HAS_BUILTIN_OVERFLOW_P __has_builtin (__builtin_mul_overflow_p)
#define _GL_HAVE___TYPEOF__ 1
#define _GL_INT_ADD_WRAPV(a,b,r) __builtin_add_overflow (a, b, r)
#define _GL_INT_NEGATE_OVERFLOW(a) _GL_INT_NEGATE_RANGE_OVERFLOW (a, _GL_INT_MINIMUM (a), _GL_INT_MAXIMUM (a))
#define _GL_INT_MULTIPLY_WRAPV(a,b,r) ((!_GL_SIGNED_TYPE_OR_EXPR (*(r)) && _GL_EXPR_SIGNED (a) && _GL_EXPR_SIGNED (b) && _GL_INT_MULTIPLY_RANGE_OVERFLOW (a, b, (__typeof__ (*(r))) 0, (__typeof__ (*(r))) -1)) ? ((void) __builtin_mul_overflow (a, b, r), 1) : __builtin_mul_overflow (a, b, r))
#define _GL_INT_OP_WRAPV(a,b,r,op,overflow) (sizeof *(r) == sizeof (signed char) ? _GL_INT_OP_WRAPV_SMALLISH (a, b, r, op, overflow, signed char, SCHAR_MIN, SCHAR_MAX, unsigned char, UCHAR_MAX) : sizeof *(r) == sizeof (short int) ? _GL_INT_OP_WRAPV_SMALLISH (a, b, r, op, overflow, short int, SHRT_MIN, SHRT_MAX, unsigned short int, USHRT_MAX) : sizeof *(r) == sizeof (int) ? (_GL_EXPR_SIGNED (*(r)) ? _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, int, INT_MIN, INT_MAX) : _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, unsigned int, 0, UINT_MAX)) : _GL_INT_OP_WRAPV_LONGISH(a, b, r, op, overflow))
#define _GL_INT_OP_WRAPV_LONGISH(a,b,r,op,overflow) (sizeof *(r) == sizeof (long int) ? (_GL_EXPR_SIGNED (*(r)) ? _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long int, long int, LONG_MIN, LONG_MAX) : _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long int, unsigned long int, 0, ULONG_MAX)) : (_GL_EXPR_SIGNED (*(r)) ? _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long long int, long long int, LLONG_MIN, LLONG_MAX) : _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long long int, unsigned long long int, 0, ULLONG_MAX)))
#define _GL_INT_OP_WRAPV_SMALLISH(a,b,r,op,overflow,st,smin,smax,ut,umax) (_GL_TYPE_SIGNED (__typeof__ (*(r))) ? _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, st, smin, smax) : _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, ut, 0, umax))
#define _GL_INT_SUBTRACT_WRAPV(a,b,r) __builtin_sub_overflow (a, b, r)
#define _GL_SIGNED_TYPE_OR_EXPR(t) _GL_TYPE_SIGNED (__typeof__ (t))
#define _GL_TYPE_SIGNED(t) (! ((t) 0 < (t) -1))
#define _GL__GENERIC_BOGUS 1
# and the chain's own names (they belong to limits.in.rs, not here): _GL_COB4/8/16/32/64/128,
# _GL_INTEGER_WIDTH, LLONG_WIDTH, _GL_LIMITS_H

# the two Rust-token rules the C text needed
$ cd femacs && CARGO_TARGET_DIR=… cargo run -q -p xtask -- build-tu lib/intprops-internal.h
error: unknown start of token: \        # 116 C line-continuations -> dropped
… then, after the fix: built lib/include/intprops-internal.rs -> ../build/femacs/include_intprops-internal.o
```
