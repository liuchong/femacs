// femacs: mirror of lib/intprops.h @ fd7afced403.
#![forbid(unsafe_code)]
//
// Header mirror (PORTING.md 0.1, P-26): the public half of gnulib's integer-property
// machinery - `TYPE_*`, `INT_*`, `EXPR_SIGNED` and the string/buffer bounds - built on the
// internal half C 21 includes.  The units that need the semantics write the Rust primitive at
// the use site (`lib/strftime.rs`'s `INT_STRLEN_BOUND` and `lib/openat-proc.rs`'s are the
// worked examples); the mirror keeps C's text (see the policy note below).
//
// **Blind, by measurement.**  `lib/intprops.h` is not in
// docs/baseline/config/cond-eval-verified.txt, and after status 3.202's fix of the evaluator's
// branch marker the reading is `python3 tools/check-cond-eval.py lib/intprops.h` ->
// `4 conditions, 0 unknown, 2 disagreements [NOT VERIFIED]`, both on the same condition:
//
//     DISAGREE lib/intprops.h:28 `if (__STDC_VERSION__ < 201112 || (defined _MSC_VER && _MSC_VER < 1944)  || _GL__GENERIC_BOGUS)` — clang True, evaluator False
//     DISAGREE lib/intprops.h:66 `if (__STDC_VERSION__ < 201112 || (defined _MSC_VER && _MSC_VER < 1944)  || _GL__GENERIC_BOGUS)` — clang True, evaluator False
//
// `_GL__GENERIC_BOGUS` is 1 for the frozen clang (measured; the `-dM` diff in the ledger shows
// the surviving `#define _GL__GENERIC_BOGUS 1`, which is
// `include/intprops-internal.rs`'s C 34-38 arm), so both conditions are true and this file
// takes the **non-`_Generic`** side of C 28-40 and C 66-92.  The evaluator answers False
// because the table cannot see a macro the included template defines - the same reason the
// inner half is blind - so the mirror carries both arms of each `#if` (P-51), the live one
// being the arm the direct measurement selects.
//
// The demand is **38 macros, 0 constants, 0 types** (`--json`: `constants: {}`,
// `containers: []`).  Thirty-two have one definition and are compared *by body*; six
// (`TYPE_IS_INTEGER`, `TYPE_MINIMUM`, `TYPE_MAXIMUM`, `_GL_ADD_OVERFLOW`,
// `_GL_SUBTRACT_OVERFLOW`, `_GL_MULTIPLY_OVERFLOW`) have several definitions, so the gate asks
// only for their presence.
//
// Live arms (measured with the `clang -dM -E` control diff; commands in the ledger):
// `TYPE_IS_INTEGER` = C 30's `((t) 1.5 == 1)`; `TYPE_MINIMUM`/`TYPE_MAXIMUM` = C 68-72's cast
// arms; the three `_GL_*_OVERFLOW` = C 227-239's `#else` arms, because **Apple clang answers 0
// for `__builtin_mul_overflow_p`** and so `_GL_HAS_BUILTIN_OVERFLOW_P` is 0 (C 220-225's
// `__builtin_*_overflow_p` arms are dead).  The `_Generic` arms (C 34-39, C 76-90) are dead.
//
// **All bodies are C's frozen text** (arguments as `$`-metavariables): these macros have no
// expanding consumer in the port, so the mirror keeps C's tokens rather than inventing a Rust
// shape the gate would stop comparing - the policy ratified for readutmp.h's
// `HAVE_STRUCT_XTMP_*`.  A `?:` chain, a `sizeof`, a `__typeof__` and C's `!` stay C's text
// inside an unexpanded `macro_rules!`, and C's backslash line-continuations are dropped (not
// Rust tokens: `error: unknown start of token: \`, the rule this batch's inner file measured).
//
// The mirror is its own translation unit like its inner half: macros plus one marker
// constant, no scalar from an includer, and the one include below is C's own C 21.

// [U1-MODULE-MIRROR]
use super::*;
// [U1] intprops-internal.rs imported via super

/* C 18-19: the guard - no Rust counterpart (PORTING.md 0.1-4); the name stays as its marker. */
pub const _GL_INTPROPS_H: bool = true;

/* C 28-40: `TYPE_IS_INTEGER (t)`.  Live: C 30's cast arm; C 34-39's `_Generic` arm is dead. */
macro_rules! TYPE_IS_INTEGER { ($t:ty) => { (($t) 1.5 == 1) }; }
#[cfg (feature = "no_generic_bogus")]
macro_rules! TYPE_IS_INTEGER {
  ($t:ty) => {
    (_Generic (($t) {0},
               bool: 1, char: 1, signed char: 1, unsigned char: 1,
               short int: 1, unsigned short int: 1, int: 1, unsigned int: 1,
               long int: 1, unsigned long int: 1, long long int: 1, unsigned long long int: 1,
               float: 0, double: 0, long double: 0))
  };
}

/* C 43: `TYPE_SIGNED (t)`. */
macro_rules! TYPE_SIGNED { ($t:ty) => { _GL_TYPE_SIGNED ($t) }; }

/* C 47: `EXPR_SIGNED (e)`. */
macro_rules! EXPR_SIGNED { ($e:expr) => { _GL_EXPR_SIGNED ($e) }; }

/* C 55: `INT_PROMOTE (e)` - "the integer promotion of E ... does not evaluate E". */
macro_rules! INT_PROMOTE { ($e:expr) => { (+ ($e)) }; }

/* C 63: `TYPE_WIDTH (t)`. */
macro_rules! TYPE_WIDTH { ($t:ty) => { _GL_TYPE_WIDTH ($t) }; }

/* C 66-92: `TYPE_MINIMUM (t)` / `TYPE_MAXIMUM (t)`.  Live: C 68-72's cast arms; C 76-90's
   `_Generic` arms are dead. */
macro_rules! TYPE_MINIMUM { ($t:ty) => { (($t) ~ TYPE_MAXIMUM ($t)) }; }
macro_rules! TYPE_MAXIMUM {
  ($t:ty) => { (($t) (! TYPE_SIGNED ($t) ? ($t) -1 : (((($t) 1 << (TYPE_WIDTH ($t) - 2)) - 1) * 2 + 1))) };
}
#[cfg (feature = "no_generic_bogus")]
macro_rules! TYPE_MINIMUM {
  ($t:ty) => {
    (_Generic (($t) {0},
              bool: (bool) 0, char: (char) CHAR_MIN,
              signed char: (signed char) SCHAR_MIN, unsigned char: (unsigned char) 0,
              short int: (short int) SHRT_MIN, unsigned short int: (unsigned short int) 0,
              int: INT_MIN, unsigned int: 0u,
              long int: LONG_MIN, unsigned long int: 0ul,
              long long int: LLONG_MIN, unsigned long long int: 0ull))
  };
}
#[cfg (feature = "no_generic_bogus")]
macro_rules! TYPE_MAXIMUM {
  ($t:ty) => {
    (_Generic (($t) {0},
              bool: (bool) 1, char: (char) CHAR_MAX,
              signed char: (signed char) SCHAR_MAX, unsigned char: (unsigned char) -1,
              short int: (short int) SHRT_MAX, unsigned short int: (unsigned short int) -1,
              int: INT_MAX, unsigned int: -1u,
              long int: LONG_MAX, unsigned long int: -1ul,
              long long int: LLONG_MAX, unsigned long long int: -1ull))
  };
}

/* C 95-114: the string-length and buffer-size bounds. */
macro_rules! INT_BITS_STRLEN_BOUND { ($b:expr) => { ((($b) * 146 + 484) / 485) }; }
macro_rules! INT_STRLEN_BOUND {
  ($t:ty) => { (INT_BITS_STRLEN_BOUND (TYPE_WIDTH ($t) - _GL_SIGNED_TYPE_OR_EXPR ($t)) + _GL_SIGNED_TYPE_OR_EXPR ($t)) };
}
macro_rules! INT_BUFSIZE_BOUND { ($t:ty) => { (INT_STRLEN_BOUND ($t) + 1) }; }

/* C 154-212: the "does A <op> B fall outside MIN..MAX" predicates. */
macro_rules! INT_ADD_RANGE_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => { (($b) < 0 ? ($a) < ($min) - ($b) : ($max) - ($b) < ($a)) };
}
macro_rules! INT_SUBTRACT_RANGE_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => { (($b) < 0 ? ($max) + ($b) < ($a) : ($a) < ($min) + ($b)) };
}
macro_rules! INT_NEGATE_RANGE_OVERFLOW {
  ($a:expr, $min:expr, $max:expr) => { _GL_INT_NEGATE_RANGE_OVERFLOW ($a, $min, $max) };
}
macro_rules! INT_MULTIPLY_RANGE_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => {
    (($b) < 0 ? (($a) < 0 ? ($a) < ($max) / ($b) : ($b) == -1 ? 0 : ($min) / ($b) < ($a)) : ($b) == 0 ? 0 : (($a) < 0 ? ($a) < ($min) / ($b) : ($max) / ($b) < ($a)))
  };
}
macro_rules! INT_DIVIDE_RANGE_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => { (($min) < 0 && ($b) == -1 && ($a) < - ($max)) };
}
macro_rules! INT_REMAINDER_RANGE_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => { INT_DIVIDE_RANGE_OVERFLOW ($a, $b, $min, $max) };
}
macro_rules! INT_LEFT_SHIFT_RANGE_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => { (($a) < 0 ? ($a) < ($min) >> ($b) : ($max) >> ($b) < ($a)) };
}

/* C 241-255: the divide/remainder helpers and the unsigned-negation one. */
macro_rules! _GL_DIVIDE_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => {
    (($min) < 0 ? ($b) == _GL_INT_CONVERT ($min, -1) && ($a) < - ($max) : ($a) < 0 ? ($b) <= ($a) + ($b) - 1 : ($b) < 0 && ($a) + ($b) <= ($a))
  };
}
macro_rules! _GL_REMAINDER_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => {
    (($min) < 0 ? ($b) == _GL_INT_CONVERT ($min, -1) && ($a) < - ($max) : ($a) < 0 ? ($a) % ($b) != (($max) - ($b) + 1) % ($b) : ($b) < 0 && ! _GL_UNSIGNED_NEG_MULTIPLE ($a, $b, $max))
  };
}
macro_rules! _GL_UNSIGNED_NEG_MULTIPLE {
  ($a:expr, $b:expr, $max:expr) => {
    ((($b) < -_GL_SIGNED_INT_MAXIMUM ($b) ? (_GL_SIGNED_INT_MAXIMUM ($b) == ($max) ? ($a) : ($a) % (_GL_INT_CONVERT ($a, _GL_SIGNED_INT_MAXIMUM ($b)) + 1)) : ($a) % - ($b)) == 0)
  };
}

/* C 219-240: the three `_GL_*_OVERFLOW` predicates.  Live: C 227-239's `#else` arms
   (`_GL_HAS_BUILTIN_OVERFLOW_P` is 0 for Apple clang); C 220-225's
   `__builtin_*_overflow_p` arms are dead. */
macro_rules! _GL_ADD_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => {
    (($min) < 0 ? INT_ADD_RANGE_OVERFLOW ($a, $b, $min, $max) : ($a) < 0 ? ($b) <= ($a) + ($b) : ($b) < 0 ? ($a) <= ($a) + ($b) : ($a) + ($b) < ($b))
  };
}
macro_rules! _GL_SUBTRACT_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => {
    (($min) < 0 ? INT_SUBTRACT_RANGE_OVERFLOW ($a, $b, $min, $max) : ($a) < 0 ? 1 : ($b) < 0 ? ($a) - ($b) <= ($a) : ($a) < ($b))
  };
}
macro_rules! _GL_MULTIPLY_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => {
    ((($min) == 0 && ((($a) < 0 && 0 < ($b)) || (($b) < 0 && 0 < ($a)))) || INT_MULTIPLY_RANGE_OVERFLOW ($a, $b, $min, $max))
  };
}
#[cfg (feature = "builtin_overflow_p")]
macro_rules! _GL_ADD_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => { __builtin_add_overflow_p ($a, $b, _GL_INT_CONVERT (($a) + ($b), 0)) };
}
#[cfg (feature = "builtin_overflow_p")]
macro_rules! _GL_SUBTRACT_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => { __builtin_sub_overflow_p ($a, $b, _GL_INT_CONVERT (($a) - ($b), 0)) };
}
#[cfg (feature = "builtin_overflow_p")]
macro_rules! _GL_MULTIPLY_OVERFLOW {
  ($a:expr, $b:expr, $min:expr, $max:expr) => { __builtin_mul_overflow_p ($a, $b, _GL_INT_CONVERT (($a) * ($b), 0)) };
}

/* C 314-327: the `INT_*_OVERFLOW` family. */
macro_rules! INT_ADD_OVERFLOW { ($a:expr, $b:expr) => { _GL_BINARY_OP_OVERFLOW ($a, $b, _GL_ADD_OVERFLOW) }; }
macro_rules! INT_SUBTRACT_OVERFLOW { ($a:expr, $b:expr) => { _GL_BINARY_OP_OVERFLOW ($a, $b, _GL_SUBTRACT_OVERFLOW) }; }
macro_rules! INT_MULTIPLY_OVERFLOW { ($a:expr, $b:expr) => { _GL_BINARY_OP_OVERFLOW ($a, $b, _GL_MULTIPLY_OVERFLOW) }; }
macro_rules! INT_NEGATE_OVERFLOW { ($a:expr) => { _GL_INT_NEGATE_OVERFLOW ($a) }; }
macro_rules! INT_DIVIDE_OVERFLOW { ($a:expr, $b:expr) => { _GL_BINARY_OP_OVERFLOW ($a, $b, _GL_DIVIDE_OVERFLOW) }; }
macro_rules! INT_REMAINDER_OVERFLOW { ($a:expr, $b:expr) => { _GL_BINARY_OP_OVERFLOW ($a, $b, _GL_REMAINDER_OVERFLOW) }; }
macro_rules! INT_LEFT_SHIFT_OVERFLOW {
  ($a:expr, $b:expr) => { INT_LEFT_SHIFT_RANGE_OVERFLOW ($a, $b, _GL_INT_MINIMUM ($a), _GL_INT_MAXIMUM ($a)) };
}

/* C 333-336: the helper the six `INT_*_OVERFLOW` names go through.  Its third argument is
   used as a *macro* (`op_result_overflow (a, b, …)`), which Rust cannot do with a
   metavariable, so the body stays C's text like every other body here. */
macro_rules! _GL_BINARY_OP_OVERFLOW {
  ($a:expr, $b:expr, $op_result_overflow:path) => {
    (op_result_overflow ($a, $b, _GL_INT_MINIMUM (_GL_INT_CONVERT ($a, $b)), _GL_INT_MAXIMUM (_GL_INT_CONVERT ($a, $b))))
  };
}

/* C 342-345: the `*_WRAPV` family. */
macro_rules! INT_ADD_WRAPV { ($a:expr, $b:expr, $r:expr) => { _GL_INT_ADD_WRAPV ($a, $b, $r) }; }
macro_rules! INT_SUBTRACT_WRAPV { ($a:expr, $b:expr, $r:expr) => { _GL_INT_SUBTRACT_WRAPV ($a, $b, $r) }; }
macro_rules! INT_MULTIPLY_WRAPV { ($a:expr, $b:expr, $r:expr) => { _GL_INT_MULTIPLY_WRAPV ($a, $b, $r) }; }

/* C 373-375: the `*_OK` negations. */
macro_rules! INT_ADD_OK { ($a:expr, $b:expr, $r:expr) => { (! INT_ADD_WRAPV ($a, $b, $r)) }; }
macro_rules! INT_SUBTRACT_OK { ($a:expr, $b:expr, $r:expr) => { (! INT_SUBTRACT_WRAPV ($a, $b, $r)) }; }
macro_rules! INT_MULTIPLY_OK { ($a:expr, $b:expr, $r:expr) => { (! INT_MULTIPLY_WRAPV ($a, $b, $r)) }; }
