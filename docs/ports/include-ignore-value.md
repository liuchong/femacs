# emacs/lib/ignore-value.h (51 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 2
- [x] _GL_IGNORE_VALUE_H  (line 36, body da39a3ee5e6b)
- [x] ignore_value  (line 48, body d25aa8a1311b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The criterion is the header itself, and the frozen compiler takes its `#else` arm.**  The
  header is not a template (`_GL_IGNORE_VALUE_H` is the guard; there is no `.in.h` substitution),
  so what decides its arms is the compiler this configuration uses - Apple clang 21:
  `(3 < __GNUC__ + (4 <= __GNUC_MINOR__)) && !defined __clang__` is false, so C 47-49's
  `#define ignore_value(x) ((void) (x))` is the live definition and C 45-46's
  `__extension__`/`__typeof__` statement expression is dead text (P-51, under
  `feature = "gnu_warn_unused_result"`).  The measurement is in Evidence.
- **Both arms are carried; the Rust spelling of each is the discard.**  C's live arm is the
  `(void)` cast, which Rust does not have: the port's discard is a `let _ =` binding, which keeps
  C's single evaluation, and the dead arm's statement expression is the same block with the
  temporary named.  `tools/extract-abi.py` sees two definitions of `ignore_value`, so it compares
  the *name* (printed in Evidence) and not the bodies - the bodies are therefore the ledger's
  record, and the probe below is what pins the live one.
- **The one consumer is another lane's file, so the mirror's compiler is a `/tmp` probe.**
  C's only includer is `src/keyboard.c` 83 (`#include <ignore-value.h>`, used at 12582:
  `ignore_value (write (STDOUT_FILENO, msg, strlen (msg)));`), whose port `femacs/src/keyboard.rs`
  is in flight in another lane and is not this batch's to touch.  The probe supplies the same
  shape a unit would (`include!` of the mirror, `assert! (!cfg! (feature = …))`, a single-
  evaluation assertion) and its commands are in Evidence.
- Nothing else in the header needs a decision: C 20-33 is the usage comment (recorded in the
  mirror's own comment), C 18's `Written by …` line has no Rust counterpart, and C 51's `#endif`
  closes the guard.

## Evidence

```
$ clang --version | head -1
Apple clang version 21.0.0 (clang-2100.3.34.2)

$ printf '#include "ignore-value.h"\nignore_value (x)\n' > /tmp/iv.c
$ clang -std=gnu23 -I emacs/lib -E /tmp/iv.c | tail -1
((void) (x))

$ python3 tools/extract-abi.py --check emacs/lib/ignore-value.h femacs/lib/include/ignore-value.rs
  (1 C macros have several definitions (configuration branches), body not compared: ignore_value)
OK emacs/lib/ignore-value.h -> femacs/lib/include/ignore-value.rs

# the negative control: the check compares the name (the bodies are exempt for a branched macro)
$ grep -v "ignore_value" femacs/lib/include/ignore-value.rs > /tmp/gt-neg2/n1.rs
$ python3 tools/extract-abi.py --check emacs/lib/ignore-value.h /tmp/gt-neg2/n1.rs
- missing macro ignore_value
FAIL emacs/lib/ignore-value.h -> /tmp/gt-neg2/n1.rs
# (mutating the live arm's body is *not* reported: the C name has two definitions, so `--check`
#  compares its presence; the probe below is the body's guard - measured, /tmp/gt-neg2/n1b.rs)

# the mirror compiles and the live arm behaves: one evaluation, the value discarded.
# The probe is a transient /tmp crate (the errno.in/dirent.in practice, not committed): it
# `include!`s the mirror, asserts the dead features off, and exercises the live arm.
$ cd /tmp && rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type bin -C panic=abort \
    -o probe_ignore_value /tmp/femacs-gt-probe-ignore/probe_ignore_value.rs
$ /tmp/probe_ignore_value
exit=0        # the `assert! (!cfg! (feature = "gnu_warn_unused_result"))` guard, the single
              # evaluation of `ignore_value! (evaluations += 1)`, and the discarded block that
              # still runs

$ rg -n '#include <ignore-value.h>' emacs/src emacs/lib
emacs/src/keyboard.c:83:#include <ignore-value.h>          # the only includer
$ rg -n "ignore_value" femacs/src femacs/lib femacs/lib-src | grep -v include/ignore-value
(no output: no ported unit names the macro yet - `femacs/src/keyboard.rs` is the in-flight file
that will, and it is another lane's)

# and what the condition evaluator says about the file (it is not on the verified list, so
# `extract-abi.py` reads every arm as live - which is why the mirror carries both)
$ python3 tools/check-cond-eval.py lib/ignore-value.h
DISAGREE lib/ignore-value.h:35 `ifndef _GL_IGNORE_VALUE_H` - clang False, evaluator True
lib/ignore-value.h: 2 conditions, 0 unknown, 1 disagreements [NOT VERIFIED]
# (the one disagreement is the guard's own `#ifndef`, which the harness has already defined by
#  pass two; the arm that matters - C 44's GCC test - is agreed on: clang takes C 47-49's `#else`)
```
