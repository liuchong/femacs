// femacs: port of lib/ignore-value.h @ fd7afced403 (gnulib's <ignore-value.h>; PORTING.md P-26).
//
// The header is not a template (`_GL_IGNORE_VALUE_H` is the guard, there is no `.in.h`
// substitution), so the criterion is the header itself as the frozen compiler reads it.  It
// defines one macro, `ignore_value`, in two arms; the frozen configuration is Apple clang
// (`clang --version` -> `Apple clang version 21.0.0`), C 44's condition needs a GCC without
// `__clang__`, and the measurement is:
//
//     $ printf '#include "ignore-value.h"\nignore_value (x)\n' > /tmp/iv.c
//     $ clang -std=gnu23 -I emacs/lib -E /tmp/iv.c | tail -1
//     ((void) (x))
//
// so C 47-49's `#else` arm is the live one and C 45-46's `__extension__`/`__typeof__`
// workaround is dead text (P-51).  The live arm is the port's "evaluate the expression and
// discard the value": Rust has no `(void)` cast and, more to the point, Rust has no
// `warn_unused_result` diagnostic for the workaround to silence (PORTING.md 8 - attributes
// map to Rust attributes or have no counterpart), so the mirror writes the discard as the
// macro body.
//
// C's arm is written first because `tools/extract-abi.py` compares the first definition of a
// name, and the ungated arm second, which is the one rustc compiles (the `errno.in.rs` /
// `fcntl.in.rs` shape).  The two definitions are why `--check` compares only the name's
// presence for `ignore_value` (its `macro_duplicates` count is 2).

// [U1-MODULE-MIRROR]
use super::*;
/* C 35-36: the include guard has no Rust counterpart (PORTING.md 0.1-4).  */
pub const _GL_IGNORE_VALUE_H: bool = true;

/* C 44-46: `#define ignore_value(x) (__extension__ ({ __typeof__ (x) __x = (x); (void) __x; }))`
   - the GCC 3.4+ workaround, dead on this target (C 44 needs `!defined __clang__`).  The
   statement-expression binds the value once and discards it, which is what the port's block
   spells; the token check exempts a branched macro's body, so this arm's text is the
   recorded one.  */
#[cfg (feature = "gnu_warn_unused_result")]
macro_rules! ignore_value {
  ($x:expr) => { { let __x = ($x); let _ = __x; } };
}

/* C 47-49: the frozen arm, `#define ignore_value(x) ((void) (x))` - evaluate and discard.
   C's `(void)` cast has no Rust counterpart; the `let _ =` binding is the port's discard and
   keeps C's single evaluation.  */
macro_rules! ignore_value {
  ($x:expr) => { { let _ = ($x); } };
}

/* C 51's `#endif` closes the guard, C 20-33 is the header's own comment and C 18's
   `Written by …` line has no Rust counterpart (PORTING.md 0.1).  */
