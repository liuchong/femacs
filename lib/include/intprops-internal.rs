// femacs: mirror of lib/intprops-internal.h @ fd7afced403.
#![forbid(unsafe_code)]
//
// Header mirror (PORTING.md 0.1, P-26).  The internal half of gnulib's integer-property
// machinery: the `_GL_*` arithmetic macros `intprops.h` (the outer, public half) builds its
// `INT_*` names on, plus the compiler-capability probes that decide which arm of each is
// compiled.  It is read by `lib/intprops.h` alone (C 20 of that file).
//
// **The gate reads this file blind, and that is measured, not assumed.**  `lib/intprops-internal.h`
// is *not* in docs/baseline/config/cond-eval-verified.txt, and after status 3.202's fix of the
// evaluator's branch marker (the stem's `-` used to make clang's verdict meaningless, which is
// what status 3.201/3.202 records) `python3 tools/check-cond-eval.py lib/intprops-internal.h`
// prints:
//
//     lib/intprops-internal.h: 24 conditions, 1 unknown, 7 disagreements [NOT VERIFIED]
//
// The seven are real readings and **the gate cannot arbitrate them**, which is why the mirror
// carries both arms of each `#if` below (P-51) instead of letting a table decide:
//
//   | line | condition | clang | evaluator | why the table cannot decide |
//   | --- | --- | --- | --- | --- |
//   | 119 | `#if _GL_HAVE___TYPEOF__` | True | False | the value is the file's *own* C 105-113 arm, whose condition is the unknown below |
//   | 137 | `#elif defined __has_builtin` | True | False | `__has_builtin` is a *compiler* builtin the configuration table need not carry |
//   | 162 | `#elif defined __has_builtin` | True | False | the same |
//   | 168 | `#if (!defined _GL_STDCKDINT_H && 202311 <= __STDC_VERSION__ && ! (…))` | False | True | `_GL_HAS_BUILTIN_*` are `__has_builtin (…)` expressions, not numbers |
//   | 177 | `#if _GL_HAS_BUILTIN_ADD_OVERFLOW` | True | False | the same |
//   | 189 | `#if _GL_HAS_BUILTIN_MUL_OVERFLOW` | True | False | the same |
//   | 258 | `#if _GL_HAVE___TYPEOF__` | True | False | the same as C 119 |
//
// and the one unknown is C 105's
// `((defined __GNUC__ && 2 <= __GNUC__) || (defined __clang_major__ && 4 <= __clang_major__) || …)`;
// the evaluator keeps its branch live (conservative), clang takes it.  Nothing here is a
// disagreement about a *configuration* macro, so no mirror edit could make the file verified -
// and none was made for a gate: every arm below was decided by the direct measurement in the
// ledger (`clang -dM -E … -include config.h [-include intprops-internal.h]`, a control diff that
// does not use the branch-marker mechanism at all).
//
// The demand is **29 macros, 0 types, 0 constants** (`--check` against an empty file: 29
// `missing macro` lines and nothing else; `--json`'s `containers` is empty).  Fifteen of the 29
// have a single definition and are compared *by body* - every one of those is transcribed below
// as C's own frozen text, which is exactly what the gate compares - and the other fourteen have
// several definitions (configuration branches), so the gate asks only for their presence.
//
// **All bodies are C's frozen text verbatim** (arguments written as `$`-metavariables), the
// policy the parent ratified for readutmp.h's `HAVE_STRUCT_XTMP_*`: these macros have **no
// expanding consumer in the port** (the public names are `intprops.h`'s, and the units that need
// their semantics write the Rust primitive at the use site - `checked_*`/`overflowing_*`/
// `is_signed`, the `strftime.rs`/`openat-proc.rs` precedent), so the mirror keeps C's tokens
// rather than inventing a Rust shape the gate would then stop comparing.  A `?:` chain, a
// `sizeof` or a `__typeof__` therefore stays C's text inside an unexpanded `macro_rules!`.
//
// C's backslash line-continuations are not Rust tokens (measured: `error: unknown start of
// token: \`), so the bodies below drop them; the token gate compares identifiers and numbers,
// which the continuations never were.
//
// The mirror is its own translation unit: it declares only macros and one marker constant, needs
// no scalar from an includer and no `include!`, so `build-tu lib/intprops-internal.h` compiles
// it as a crate root.
//
// Live arm per name (measured; the ledger has the commands and the full `#define` diff):
// `_GL__GENERIC_BOGUS` 1 (clang defines `__clang__`) - which is what turns C 53's `_Generic` arm
// off and C 274's `sizeof` chain on; `_GL_HAVE___TYPEOF__` 1 (`__clang_major__` 21 >= 4);
// `_GL_HAS_BUILTIN_ADD_OVERFLOW` `__has_builtin (__builtin_add_overflow)`,
// `_GL_HAS_BUILTIN_MUL_OVERFLOW` `_GL_HAS_BUILTIN_ADD_OVERFLOW`,
// `_GL_HAS_BUILTIN_OVERFLOW_P` `__has_builtin (__builtin_mul_overflow_p)` - where Apple clang
// answers **0** for `__builtin_mul_overflow_p`, which is why `_GL_INT_NEGATE_OVERFLOW` is C
// 336-337's range arm; `_GL_INT_ADD_WRAPV`/`_GL_INT_SUBTRACT_WRAPV` are the `__builtin_*` arms,
// `_GL_INT_MULTIPLY_WRAPV` is C 197-203's GCC-bug-91450 wrapper (`!defined __clang__` is false),
// `_GL_INT_OP_WRAPV_SMALLISH` is C 264's `__typeof__` arm and `_GL_INT_OP_WRAPV_LONGISH` C 304's.
//
// C 93-96's `LLONG_MAX`/`LLONG_MIN` arm is **dead**: `<limits.h>` has already defined both
// (measured: the surviving definition is `LLONG_MAX __LONG_LONG_MAX__`, limits.h's spelling, not
// `__INT64_MAX`), which is why `port below` carries it under `openvms` and why the port's two
// values live in `include/limits.in.rs` (55-57) instead.

// [U1-MODULE-MIRROR]
use super::*;
/* C 19: `#ifndef _GL_INTPROPS_INTERNAL_H` - the guard has no Rust counterpart (PORTING.md
   0.1-4); the name stays as its marker.  */
pub const _GL_INTPROPS_INTERNAL_H: bool = true;

/* C 21's `#include <limits.h>` is the includer's (`include/limits.in.rs`), C 24-29's
   `4 < __GNUC__ + (3 <= __GNUC_MINOR__) && !defined __clang__` `#pragma` is dead (the target
   is clang) and C 41-43's `#pragma` is dead too (`__STDC_VERSION__` is 202311): a `#pragma`
   has no Rust item, so both are recorded in the ledger.  */

/* C 34-38: "nonzero if this compiler has GCC bug 68193 or Clang bug 25764" - the flag that
   turns the `_Generic` arms below off.  Live arm: clang, so `1`; the `#else`'s `0` is dead. */
macro_rules! _GL__GENERIC_BOGUS { () => { 1 }; }
#[cfg (feature = "no_clang")]
macro_rules! _GL__GENERIC_BOGUS { () => { 0 }; }

/* C 47: `_GL_INT_CONVERT (e, v)` - "return a value with the common real type of E and V and
   the value of V.  Do not evaluate E."  */
macro_rules! _GL_INT_CONVERT { ($e:expr, $v:expr) => { ((1 ? 0 : ($e)) + ($v)) }; }

/* C 53-67: `_GL_TYPE_SIGNED (t)`.  Live: C 59's `! ((t) 0 < (t) -1)` arm, because
   `_GL__GENERIC_BOGUS` is 1; the `_Generic` arm (C 64-67) is dead and carried under
   `no_generic_bogus`.  */
macro_rules! _GL_TYPE_SIGNED { ($t:ty) => { (! (($t) 0 < ($t) -1)) }; }
#[cfg (feature = "no_generic_bogus")]
macro_rules! _GL_TYPE_SIGNED {
  ($t:ty) => {
    (_Generic (($t) {0},
               bool: 0, char: CHAR_MIN < 0, signed char: 1, unsigned char: 0,
               short int: 1, unsigned short int: 0, int: 1, unsigned int: 0,
               long int: 1, unsigned long int: 0, long long int: 1, unsigned long long int: 0,
               float: 1, double: 1, long double: 1))
  };
}

/* C 69-70: "return 1 if the real expression E, after promotion, has a signed or floating
   type.  Do not evaluate E."  */
macro_rules! _GL_EXPR_SIGNED { ($e:expr) => { (_GL_INT_CONVERT ($e, -1) < 0) }; }

/* C 77: "the width in bits of the integer type or expression T."  */
macro_rules! _GL_TYPE_WIDTH { ($t:ty) => { (sizeof ($t) * CHAR_BIT) }; }

/* C 81-89: the minimum/maximum of the type of an expression.  C's `?:` is kept as C's text
   (this mirror's bodies are C's frozen text; nothing in the port expands them).  */
macro_rules! _GL_INT_MINIMUM {
  ($e:expr) => { (_GL_EXPR_SIGNED ($e) ? ~ _GL_SIGNED_INT_MAXIMUM ($e) : _GL_INT_CONVERT ($e, 0)) };
}
macro_rules! _GL_INT_MAXIMUM {
  ($e:expr) => { (_GL_EXPR_SIGNED ($e) ? _GL_SIGNED_INT_MAXIMUM ($e) : _GL_INT_CONVERT ($e, -1)) };
}
macro_rules! _GL_SIGNED_INT_MAXIMUM {
  ($e:expr) => { (((_GL_INT_CONVERT ($e, 1) << (_GL_TYPE_WIDTH (+ ($e)) - 2)) - 1) * 2 + 1) };
}

/* C 93-96: the OpenVMS C99 workaround, "`#if !defined LLONG_MAX && defined __INT64_MAX`".
   Dead on the frozen target - `<limits.h>` defines `LLONG_MAX` (measured: the surviving
   definition is `#define LLONG_MAX __LONG_LONG_MAX__`) - so the port's values are
   `include/limits.in.rs`'s (55-57) and these are the dead arm's text.  */
#[cfg (feature = "openvms")]
macro_rules! LLONG_MAX { () => { __INT64_MAX }; }
#[cfg (feature = "openvms")]
macro_rules! LLONG_MIN { () => { __INT64_MIN }; }

/* C 105-113: "does the `__typeof__` keyword work?".  Live: `1` (clang 21 >= 4); C 111-112's
   `0` is dead.  */
macro_rules! _GL_HAVE___TYPEOF__ { () => { 1 }; }
#[cfg (feature = "no_typeof")]
macro_rules! _GL_HAVE___TYPEOF__ { () => { 0 }; }

/* C 119-126: `_GL_SIGNED_TYPE_OR_EXPR (t)`.  Live: C 119-120's `__typeof__` arm; C 125's
   `1` is dead and carried.  */
macro_rules! _GL_SIGNED_TYPE_OR_EXPR { ($t:ty) => { _GL_TYPE_SIGNED (__typeof__ ($t)) }; }
#[cfg (feature = "no_typeof")]
macro_rules! _GL_SIGNED_TYPE_OR_EXPR { ($t:ty) => { 1 }; }

/* C 128-129: "return 1 if - A would overflow in [MIN,MAX] arithmetic."  */
macro_rules! _GL_INT_NEGATE_RANGE_OVERFLOW {
  ($a:expr, $min:expr, $max:expr) => { (($min) < 0 ? ($a) < - ($max) : 0 < ($a)) };
}

/* C 138-146: "true if `__builtin_add_overflow (A, B, P)` and `__builtin_sub_overflow` work
   when P is non-null".  Live: C 141-142's `__has_builtin` arm (measured: the surviving
   definition is `__has_builtin (__builtin_add_overflow)`); the `__EDG__` arm (C 139-140), the
   `7 <= __GNUC__` arm (C 144-145, `gcc_lt_7`) and the `#else`'s `0` (C 146,
   `no_has_builtin_gcc_lt_7`) are dead.  */
macro_rules! _GL_HAS_BUILTIN_ADD_OVERFLOW { () => { __has_builtin (__builtin_add_overflow) }; }
#[cfg (feature = "edg")]
macro_rules! _GL_HAS_BUILTIN_ADD_OVERFLOW { () => { 0 }; }
#[cfg (feature = "gcc_lt_7")]
macro_rules! _GL_HAS_BUILTIN_ADD_OVERFLOW { () => { 1 }; }
#[cfg (feature = "no_has_builtin_gcc_lt_7")]
macro_rules! _GL_HAS_BUILTIN_ADD_OVERFLOW { () => { 0 }; }

/* C 148-153: `_GL_HAS_BUILTIN_MUL_OVERFLOW`.  Live: C 152's `else` arm, `_GL_HAS_BUILTIN_ADD_OVERFLOW`
   (clang 21 is not < 21); C 149-151's `0` is dead.  */
macro_rules! _GL_HAS_BUILTIN_MUL_OVERFLOW { () => { _GL_HAS_BUILTIN_ADD_OVERFLOW } }
#[cfg (feature = "clang_lt_21")]
macro_rules! _GL_HAS_BUILTIN_MUL_OVERFLOW { () => { 0 }; }

/* C 155-166: `_GL_HAS_BUILTIN_OVERFLOW_P`.  Live: C 163-164's `__has_builtin` arm; the
   `__EDG__` arm (C 156) and the `(7 <= __GNUC__)` arm (C 166) are dead.  Apple clang answers
   0 for `__builtin_mul_overflow_p`, which is what makes `_GL_INT_NEGATE_OVERFLOW` below take
   its range arm.  */
macro_rules! _GL_HAS_BUILTIN_OVERFLOW_P { () => { __has_builtin (__builtin_mul_overflow_p) }; }
#[cfg (feature = "edg")]
macro_rules! _GL_HAS_BUILTIN_OVERFLOW_P { () => { 0 }; }
#[cfg (feature = "no_has_builtin")]
macro_rules! _GL_HAS_BUILTIN_OVERFLOW_P { () => { (7 <= __GNUC__) }; }

/* C 168-170: `#if (!defined _GL_STDCKDINT_H && 202311 <= __STDC_VERSION__ &&
   ! (_GL_HAS_BUILTIN_ADD_OVERFLOW && _GL_HAS_BUILTIN_MUL_OVERFLOW)) # include <stdckdint.h>`
   - clang takes it as dead (the two `__builtin_*_overflow` builtins are present), the
   evaluator keeps it live because their values are `__has_builtin` expressions.  An
   `#include` has no Rust item and the port has no `stdckdint.h` mirror, so it is recorded in
   the ledger, not carried.  */

/* C 173-188: `_GL_INT_ADD_WRAPV`/`_GL_INT_SUBTRACT_WRAPV`.  Live: C 180-181's
   `__builtin_*_overflow` arms.  Dead: the `ckd_add`/`ckd_sub` arms (C 182-183) and the
   `_GL_INT_OP_WRAPV` fallbacks (C 185-188).  */
macro_rules! _GL_INT_ADD_WRAPV {
  ($a:expr, $b:expr, $r:expr) => { __builtin_add_overflow ($a, $b, $r) };
}
macro_rules! _GL_INT_SUBTRACT_WRAPV {
  ($a:expr, $b:expr, $r:expr) => { __builtin_sub_overflow ($a, $b, $r) };
}
#[cfg (feature = "ckd")]
macro_rules! _GL_INT_ADD_WRAPV {
  ($a:expr, $b:expr, $r:expr) => { ckd_add ($r, + ($a), + ($b)) };
}
#[cfg (feature = "ckd")]
macro_rules! _GL_INT_SUBTRACT_WRAPV {
  ($a:expr, $b:expr, $r:expr) => { ckd_sub ($r, + ($a), + ($b)) };
}
#[cfg (feature = "no_builtin_add_overflow")]
macro_rules! _GL_INT_ADD_WRAPV {
  ($a:expr, $b:expr, $r:expr) => { _GL_INT_OP_WRAPV ($a, $b, $r, +, _GL_INT_ADD_RANGE_OVERFLOW) };
}
#[cfg (feature = "no_builtin_add_overflow")]
macro_rules! _GL_INT_SUBTRACT_WRAPV {
  ($a:expr, $b:expr, $r:expr) => { _GL_INT_OP_WRAPV ($a, $b, $r, -, _GL_INT_SUBTRACT_RANGE_OVERFLOW) };
}

/* C 191-214: `_GL_INT_MULTIPLY_WRAPV`.  Live: C 197-203's wrapper ("work around GCC bug
   91450"), because C 194's `!defined __clang__` term is false; C 194's plain
   `__builtin_mul_overflow` arm and the two other branches are dead.  */
macro_rules! _GL_INT_MULTIPLY_WRAPV {
  ($a:expr, $b:expr, $r:expr) => {
    ((!_GL_SIGNED_TYPE_OR_EXPR (*(r)) && _GL_EXPR_SIGNED (a) && _GL_EXPR_SIGNED (b) 
      && _GL_INT_MULTIPLY_RANGE_OVERFLOW (a, b, 
                                          (__typeof__ (*(r))) 0, 
                                          (__typeof__ (*(r))) -1)) 
     ? ((void) __builtin_mul_overflow (a, b, r), 1) 
     : __builtin_mul_overflow (a, b, r))
  };
}
#[cfg (feature = "gcc_9_3")]
macro_rules! _GL_INT_MULTIPLY_WRAPV {
  ($a:expr, $b:expr, $r:expr) => { __builtin_mul_overflow ($a, $b, $r) };
}
#[cfg (feature = "ckd")]
macro_rules! _GL_INT_MULTIPLY_WRAPV {
  ($a:expr, $b:expr, $r:expr) => { ckd_mul ($r, + ($a), + ($b)) };
}
#[cfg (feature = "no_builtin_mul_overflow")]
macro_rules! _GL_INT_MULTIPLY_WRAPV {
  ($a:expr, $b:expr, $r:expr) => { _GL_INT_OP_WRAPV ($a, $b, $r, *, _GL_INT_MULTIPLY_RANGE_OVERFLOW) };
}

/* C 219-259: `_GL_INT_OP_WRAPV`'s `_Generic` arm (C 219's `201112 <= __STDC_VERSION__ &&
   !_GL__GENERIC_BOGUS` is false here) - dead, carried under `no_generic_bogus`.  */
#[cfg (feature = "no_generic_bogus")]
macro_rules! _GL_INT_OP_WRAPV {
  ($a:expr, $b:expr, $r:expr, $op:tt, $overflow:path) => {
    (_Generic 
     (*(r), 
      signed char: 
        _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, 
                         signed char, SCHAR_MIN, SCHAR_MAX), 
      unsigned char: 
        _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, 
                         unsigned char, 0, UCHAR_MAX), 
      short int: 
        _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, 
                         short int, SHRT_MIN, SHRT_MAX), 
      unsigned short int: 
        _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, 
                         unsigned short int, 0, USHRT_MAX), 
      int: 
        _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, 
                         int, INT_MIN, INT_MAX), 
      unsigned int: 
        _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, 
                         unsigned int, 0, UINT_MAX), 
      long int: 
        _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long int, 
                         long int, LONG_MIN, LONG_MAX), 
      unsigned long int: 
        _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long int, 
                         unsigned long int, 0, ULONG_MAX), 
      long long int: 
        _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long long int, 
                         long long int, LLONG_MIN, LLONG_MAX), 
      unsigned long long int: 
        _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long long int, 
                         unsigned long long int, 0, ULLONG_MAX)))
  };
}

/* C 260-272: `_GL_INT_OP_WRAPV_SMALLISH`.  Live: C 264-267's `__typeof__` arm
   (`_GL_HAVE___TYPEOF__` is 1); C 269-272's arm is dead.  */
macro_rules! _GL_INT_OP_WRAPV_SMALLISH {
  ($a:expr,$b:expr,$r:expr,$op:tt,$overflow:path,$st:ty,$smin:expr,$smax:expr,$ut:ty,$umax:expr) => {
    (_GL_TYPE_SIGNED (__typeof__ (*(r))) 
     ? _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, st, smin, smax) 
     : _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, ut, 0, umax))
  };
}
#[cfg (feature = "no_typeof")]
macro_rules! _GL_INT_OP_WRAPV_SMALLISH {
  ($a:expr,$b:expr,$r:expr,$op:tt,$overflow:path,$st:ty,$smin:expr,$smax:expr,$ut:ty,$umax:expr) => {
    (overflow (a, b, smin, smax) 
     ? (overflow (a, b, 0, umax) 
        ? (*(r) = _GL_INT_OP_WRAPV_VIA_UNSIGNED (a,b,op,unsigned,st), 1) 
        : (*(r) = _GL_INT_OP_WRAPV_VIA_UNSIGNED (a,b,op,unsigned,st)) < 0) 
     : (overflow (a, b, 0, umax) 
        ? (*(r) = _GL_INT_OP_WRAPV_VIA_UNSIGNED (a,b,op,unsigned,st)) >= 0 
        : (*(r) = _GL_INT_OP_WRAPV_VIA_UNSIGNED (a,b,op,unsigned,st), 0)))
  };
}

/* C 274-303: `_GL_INT_OP_WRAPV`'s live arm - the `sizeof` chain.  */
macro_rules! _GL_INT_OP_WRAPV {
  ($a:expr, $b:expr, $r:expr, $op:tt, $overflow:path) => {
    (sizeof *(r) == sizeof (signed char) 
     ? _GL_INT_OP_WRAPV_SMALLISH (a, b, r, op, overflow, 
                                  signed char, SCHAR_MIN, SCHAR_MAX, 
                                  unsigned char, UCHAR_MAX) 
     : sizeof *(r) == sizeof (short int) 
     ? _GL_INT_OP_WRAPV_SMALLISH (a, b, r, op, overflow, 
                                  short int, SHRT_MIN, SHRT_MAX, 
                                  unsigned short int, USHRT_MAX) 
     : sizeof *(r) == sizeof (int) 
     ? (_GL_EXPR_SIGNED (*(r)) 
        ? _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, 
                           int, INT_MIN, INT_MAX) 
        : _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned int, 
                           unsigned int, 0, UINT_MAX)) 
     : _GL_INT_OP_WRAPV_LONGISH(a, b, r, op, overflow))
  };
}

/* C 304-311: `_GL_INT_OP_WRAPV_LONGISH`.  Live: C 304-311's `#ifdef LLONG_MAX` arm (the name
   is defined by `<limits.h>`); C 313-318's arm is dead.  */
macro_rules! _GL_INT_OP_WRAPV_LONGISH {
  ($a:expr, $b:expr, $r:expr, $op:tt, $overflow:path) => {
    (sizeof *(r) == sizeof (long int) 
     ? (_GL_EXPR_SIGNED (*(r)) 
        ? _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long int, 
                           long int, LONG_MIN, LONG_MAX) 
        : _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long int, 
                           unsigned long int, 0, ULONG_MAX)) 
     : (_GL_EXPR_SIGNED (*(r)) 
        ? _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long long int, 
                           long long int, LLONG_MIN, LLONG_MAX) 
        : _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long long int, 
                           unsigned long long int, 0, ULLONG_MAX)))
  };
}
#[cfg (feature = "no_llong_max")]
macro_rules! _GL_INT_OP_WRAPV_LONGISH {
  ($a:expr, $b:expr, $r:expr, $op:tt, $overflow:path) => {
    (_GL_EXPR_SIGNED (*(r)) 
     ? _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long int, 
                        long int, LONG_MIN, LONG_MAX) 
     : _GL_INT_OP_CALC (a, b, r, op, overflow, unsigned long int, 
                        unsigned long int, 0, ULONG_MAX))
  };
}

/* C 319-321: "store the low-order bits of A <op> B into *R ... Return 1 if the result
   overflows."  */
macro_rules! _GL_INT_OP_CALC {
  ($a:expr, $b:expr, $r:expr, $op:tt, $overflow:path, $ut:ty, $t:ty, $tmin:expr, $tmax:expr) => {
    (overflow (a, b, tmin, tmax) 
     ? (*(r) = _GL_INT_OP_WRAPV_VIA_UNSIGNED (a, b, op, ut, t), 1) 
     : (*(r) = _GL_INT_OP_WRAPV_VIA_UNSIGNED (a, b, op, ut, t), 0))
  };
}

/* C 333-338: `_GL_INT_NEGATE_OVERFLOW`.  Live: C 336-337's range arm, because Apple clang
   answers 0 for `__builtin_mul_overflow_p` (measured); C 335's `__builtin_sub_overflow_p`
   arm is dead.  */
macro_rules! _GL_INT_NEGATE_OVERFLOW {
  ($a:expr) => { _GL_INT_NEGATE_RANGE_OVERFLOW ($a, _GL_INT_MINIMUM ($a), _GL_INT_MAXIMUM ($a)) };
}
#[cfg (feature = "builtin_overflow_p")]
macro_rules! _GL_INT_NEGATE_OVERFLOW {
  ($a:expr) => { __builtin_sub_overflow_p (0, $a, _GL_INT_CONVERT (- ($a), 0)) };
}

/* C 355-356: "return the low-order bits of A <op> B ... convert the result to type T."  */
macro_rules! _GL_INT_OP_WRAPV_VIA_UNSIGNED {
  ($a:expr, $b:expr, $op:tt, $ut:ty, $t:ty) => { (($t) (($ut) ($a) $op ($ut) ($b))) };
}

/* C 364-407: the three "do the numeric values A + B, A - B, A * B fall outside
   TMIN..TMAX?" predicates, C's text verbatim.  */
macro_rules! _GL_INT_ADD_RANGE_OVERFLOW {
  ($a:expr, $b:expr, $tmin:expr, $tmax:expr) => {
    ((b) < 0 
     ? (((tmin) 
         ? ((_GL_EXPR_SIGNED (_GL_INT_CONVERT (a, (tmin) - (b))) || (b) < (tmin)) 
            && (a) < (tmin) - (b)) 
         : (a) <= -1 - (b)) 
        || ((_GL_EXPR_SIGNED (a) ? 0 <= (a) : (tmax) < (a)) && (tmax) < (a) + (b))) 
     : (a) < 0 
     ? (((tmin) 
         ? ((_GL_EXPR_SIGNED (_GL_INT_CONVERT (b, (tmin) - (a))) || (a) < (tmin)) 
            && (b) < (tmin) - (a)) 
         : (b) <= -1 - (a)) 
        || ((_GL_EXPR_SIGNED (_GL_INT_CONVERT (a, b)) || (tmax) < (b)) 
            && (tmax) < (a) + (b))) 
     : (tmax) < (b) || (tmax) - (b) < (a))
  };
}
macro_rules! _GL_INT_SUBTRACT_RANGE_OVERFLOW {
  ($a:expr, $b:expr, $tmin:expr, $tmax:expr) => {
    (((a) < 0) == ((b) < 0) 
     ? ((a) < (b) 
        ? !(tmin) || -1 - (tmin) < (b) - (a) - 1 
        : (tmax) < (a) - (b)) 
     : (a) < 0 
     ? ((!_GL_EXPR_SIGNED (_GL_INT_CONVERT ((a) - (tmin), b)) && (a) - (tmin) < 0) 
        || (a) - (tmin) < (b)) 
     : ((! (_GL_EXPR_SIGNED (_GL_INT_CONVERT (tmax, b)) 
            && _GL_EXPR_SIGNED (_GL_INT_CONVERT ((tmax) + (b), a))) 
         && (tmax) <= -1 - (b)) 
        || (tmax) + (b) < (a)))
  };
}
macro_rules! _GL_INT_MULTIPLY_RANGE_OVERFLOW {
  ($a:expr, $b:expr, $tmin:expr, $tmax:expr) => {
    ((b) < 0 
     ? ((a) < 0 
        ? (_GL_EXPR_SIGNED (_GL_INT_CONVERT (tmax, b)) 
           ? (a) < (tmax) / (b) 
           : ((_GL_INT_NEGATE_OVERFLOW (b) 
               ? _GL_INT_CONVERT (b, tmax) >> (_GL_TYPE_WIDTH (+ (b)) - 1) 
               : (tmax) / -(b)) 
              <= -1 - (a))) 
        : _GL_INT_NEGATE_OVERFLOW (_GL_INT_CONVERT (b, tmin)) && (b) == -1 
        ? (_GL_EXPR_SIGNED (a) 
           ? 0 < (a) + (tmin) 
           : 0 < (a) && -1 - (tmin) < (a) - 1) 
        : (tmin) / (b) < (a)) 
     : (b) == 0 
     ? 0 
     : ((a) < 0 
        ? (_GL_INT_NEGATE_OVERFLOW (_GL_INT_CONVERT (a, tmin)) && (a) == -1 
           ? (_GL_EXPR_SIGNED (b) ? 0 < (b) + (tmin) : -1 - (tmin) < (b) - 1) 
           : (tmin) / (a) < (b)) 
        : (tmax) / (b) < (a)))
  };
}
