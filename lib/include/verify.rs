// femacs: port of lib/verify.h @ fd7afced403.
//
// The header is a family of compile-time checks.  The frozen target is the C configuration
// with clang and C23 (`__STDC_VERSION__` 202311L, `_GL_HAVE__STATIC_ASSERT` and
// `_GL_HAVE__STATIC_ASSERT1` both 1, `__clang_major__` 21), so the arms it compiles are:
// `_GL_VERIFY` is `static_assert` (C 227), `_GL_VERIFY_TYPE` is the `_Static_assert` arm
// (C 209-211), `assume` is the `__builtin_unreachable` arm (C 389) and the
// `#ifdef _GL_STATIC_ASSERT_H` block (C 240-317) is dead text, which the mirror still
// carries because the gate reads a header's `#if`s blind unless the header is in
// docs/baseline/config/cond-eval-verified.txt (P-08/P-66), and this one is not.
//
// Rust's `assert!` in a `const` item is the counterpart of C's compile-time checking
// (PORTING.md 2.5): `verify (R)` is the declaration-context form (an item, so its call site
// writes `verify! (R);`) and `verify_expr (R, E)` the expression-context one.  `assume` is
// PORTING.md 2.6's `core::hint::unreachable_unchecked`.
//
// `macro_rules!` names resolve where they are *used* (PORTING.md 0.1-18), so the definitions
// stand in C's order and every user expands them after the whole mirror, exactly as the C
// preprocessor sees the file.
//
// Deviations, all recorded in docs/ports/include-verify.md: Rust cannot paste preprocessor
// tokens, so `_GL_CONCAT0` yields its second argument and the caller spells the pasted name
// (the `DYNARRAY_CONCAT0` shape), `_GL_COUNTER` takes C's own `__LINE__` fallback arm
// (`line! ()`), and C's `sizeof` is `core::mem::size_of`.

/* verify.h 21: the include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _GL_VERIFY_H: bool = true;

/* verify.h 41/45: the two selects for a working `_Static_assert`, with and without a
   message; C23 is in force, so both are 1.  */
pub const _GL_HAVE__STATIC_ASSERT: c_int = 1;
pub const _GL_HAVE__STATIC_ASSERT1: c_int = 1;

/* verify.h 170-171: `_GL_CONCAT (x, y)` / `_GL_CONCAT0 (x, y)` paste two preprocessor tokens
   into one identifier.  Rust has no token pasting (PORTING.md 2.1; the `DYNARRAY_CONCAT0`
   row in docs/ports/dynarray-skeleton.md and the `STAT_TIMESPEC`/`KVAR` rows in
   docs/ports/excluded-macros.txt), so `_GL_CONCAT0` yields its second argument and the
   caller spells the pasted name.  */
macro_rules! _GL_CONCAT {
  ($x:tt, $($y:tt)*) => { _GL_CONCAT0! ($x, $($y)*) };
}
macro_rules! _GL_CONCAT0 {
  ($x:tt, $($y:tt)*) => { $($y)* };
}

/* verify.h 177-183: `_GL_COUNTER` is `__COUNTER__` when that works and `__LINE__`
   otherwise, so it is an integer that differs from use to use.  Rust has no `__COUNTER__`;
   `line! ()` is the same kind of per-use integer as the reference's own fallback arm, so the
   port takes that arm.  */
macro_rules! _GL_COUNTER {
  () => { line! () };
}

/* verify.h 186: `_GL_GENSYM (prefix)` makes a declaration name unique per use.  Rust cannot
   paste, so the port keeps the call text and hands the counter back: the expansion is the
   integer, not a name.  The frozen target's `_GL_VERIFY` arm needs no gensym (it is
   `static_assert`), so nothing expands this macro.  */
macro_rules! _GL_GENSYM {
  ($prefix:tt) => { _GL_CONCAT! ($prefix, _GL_COUNTER! ()) };
}

/* verify.h 192: `_GL_VERIFY_TRUE (R, DIAGNOSTIC)` is C's `!!sizeof (_GL_VERIFY_TYPE (R,
   DIAGNOSTIC))`, a constant expression that is 1 and fails the compilation if R is false;
   `sizeof` is the port's `core::mem::size_of` (PORTING.md 2.6).  */
macro_rules! _GL_VERIFY_TRUE {
  ($r:expr, $diagnostic:expr) => {
    (!! (core::mem::size_of::<_GL_VERIFY_TYPE! ($r, $diagnostic)> () != 0))
  };
}

/* verify.h 199-202: the C++ arm's `template <int w> struct _gl_verify_type { unsigned int
   _gl_verify_error_if_negative: w; };` - a negative width is the compile-time error.  The
   port is the C configuration, which does not take this arm, so the type is the C++ arm's
   spelling kept so the two arms' names line up; its one member is the bitfield's
   `unsigned int` storage unit (PORTING.md 1.3/P-80) with C's width in the comment above it.
   No layout key exists for it in docs/baseline/layout, because the C target has no such
   type.  */
#[repr(C)]
pub struct _gl_verify_type {
  /* unsigned int _gl_verify_error_if_negative: w;  */
  pub _gl_verify_error_if_negative: c_uint,
}

/* verify.h 201: the C++ arm's "the template is defined once" select.  */
pub const GNULIB_defined_struct__gl_verify_type: c_int = 1;

/* verify.h 205-210, the `_GL_HAVE__STATIC_ASSERT` arm the frozen target takes: C's
   `struct { _Static_assert (R, DIAGNOSTIC); int _gl_dummy; }` is a type whose only jobs are
   the check and a non-zero size.  The port spells it as an array whose length is a const
   block holding the check, so the type stays usable where C's is (an operand of
   `size_of`), and the element type is C's `int _gl_dummy` member.  */
macro_rules! _GL_VERIFY_TYPE {
  ($r:expr, $diagnostic:expr) => {
    [c_int; { const _: () = core::assert! ($r, $diagnostic); 1 }]
  };
}

/* verify.h 227, the arm the frozen target takes: `_GL_VERIFY (R, DIAGNOSTIC, ...)` is
   `static_assert (R, DIAGNOSTIC)`.  The declaration-context form, so its call site is an
   item; the third argument and beyond are C's variadic tail, which lets one spelling serve
   the two-argument and the one-argument `static_assert` alike.  */
macro_rules! _GL_VERIFY {
  ($r:expr, $diagnostic:expr $(, $rest:tt)*) => { static_assert! { $r, $diagnostic } };
}

/* verify.h 267-270, the arm the reference's `_GL_STATIC_ASSERT_H` block selects when
   `_Static_assert` is not a keyword: `_GL_VERIFY ((R), "static assertion failed", -)`.  The
   block is dead text for the frozen configuration (see the file note); the port carries it
   because the gate reads both arms of a header it cannot evaluate.  */
macro_rules! _Static_assert {
  ($($args:tt)*) => { _GL_VERIFY! { $($args)*, "static assertion failed", - } };
}

/* verify.h 300: `_GL_EXPAND (x)` is `x`, the MSVC arm's identity wrapper around one more
   macro call.  */
macro_rules! _GL_EXPAND {
  ($x:tt) => { $x };
}

/* verify.h 301-304: the two preprocessor tricks for a compiler whose `static_assert` takes
   two arguments but not one (g++ 4.7-5 with `-std=c++11`, and MSVC 14's C++ mode).  Rust's
   `static_assert!` takes both spellings, so `_GL_SA1` is the two-argument form and
   `_GL_SA2`/`_GL_SA3` forward whatever they are given; `_GL_SA_PICK` yields its fourth
   argument, exactly as C's `x4` does.  */
macro_rules! _GL_SA1 {
  ($a1:expr) => { static_assert! { $a1, "static assertion failed" } };
}
macro_rules! _GL_SA2 {
  ($($args:tt)*) => { static_assert! { $($args)* } };
}
macro_rules! _GL_SA3 {
  ($($args:tt)*) => { static_assert! { $($args)* } };
}
macro_rules! _GL_SA_PICK {
  ($x1:tt, $x2:tt, $x3:tt, $x4:tt $(, $rest:tt)*) => { $x4 };
}

/* verify.h 307: the guard that keeps a toolchain from complaining that a keyword is being
   macroized; only that toolchain's own arm sets it.  */
pub const _ALLOW_KEYWORD_MACROS: c_int = 1;

/* verify.h 309, the `#else` arm of the C23 test (C defines this name three times, once per
   arm of the trick above): `#define static_assert _Static_assert`.  The frozen target has
   neither the trick nor this macro - C23's `static_assert` is a keyword - so the port keeps
   the one name PORTING.md 2.5 gives that keyword: a `const`-item assertion, written
   `static_assert! (cond);` at its call sites.  C's optional diagnostic argument is the
   second one.  */
macro_rules! static_assert {
  ($cond:expr $(, $msg:expr)? $(,)?) => {
    const _: () = core::assert! ($cond $(, $msg)?);
  };
}

/* verify.h 323/334: whether the compiler has the two builtins that `assume`'s other arms
   name.  The frozen target reports 1 for both, so `assume` takes the first arm; C defines
   each of these four times (one per toolchain test) and the gate compares no body for a
   name with several definitions (P-08).  */
pub const _GL_HAS_BUILTIN_TRAP: c_int = 1;
pub const _GL_HAS_BUILTIN_UNREACHABLE: c_int = 1;

/* verify.h 350: `verify_expr (R, E)` is the expression-context check - an expression that
   fails the compilation unless R holds and whose value is E.  C's `? (E) : (E)` evaluates E
   once, which the port's `if` keeps; the diagnostic is C's stringised pair.  */
macro_rules! verify_expr {
  ($r:expr, $e:expr) => {
    if _GL_VERIFY_TRUE! ($r, concat! ("verify_expr (", stringify! ($r), ", ",
                                      stringify! ($e), ")")) { $e } else { $e }
  };
}

/* verify.h 362: `verify (R)` is the declaration-context form, `_GL_VERIFY (R, "verify ("
   #R ")", -)`; PORTING.md 2.5's two states, with the item as the call site's shape.  */
macro_rules! verify {
  ($r:expr) => { _GL_VERIFY! { $r, concat! ("verify (", stringify! ($r), ")"), - } };
}

/* verify.h 389, the `_GL_HAS_BUILTIN_UNREACHABLE` arm the frozen target takes:
   `((R) ? (void) 0 : __builtin_unreachable ())`.  PORTING.md 2.6 gives the builtin the
   port's `core::hint::unreachable_unchecked`; the condition is evaluated exactly once, as in
   C (P-16), and the macro expands to a statement, so its call sites keep C's `assume (R);`.
   */
macro_rules! assume {
  ($r:expr) => {
    if !($r) { unsafe { core::hint::unreachable_unchecked () } }
  };
}
