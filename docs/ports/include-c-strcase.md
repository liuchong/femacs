# emacs/lib/c-strcase.h (61 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] C_STRCASE_H  (line 19, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The criterion is the header itself, and the extractor's item list is the guard alone.**  The
  header is not a template, so its conditions are decided by the compiler this configuration
  uses: C 22-24's `#if !_GL_CONFIG_H_INCLUDED / #error` is dead (the configuration's `config.h`
  defines `_GL_CONFIG_H_INCLUDED`, `docs/baseline/config/darwin-arm64.macros.tsv`), and
  C 39-41's `extern "C" {` with C 56-58's `}` are dead (`__cplusplus` is undefined for a C
  translation unit).  C 26's `#include <stddef.h>` is live, and `size_t` is the one name the live
  text takes from it: PORTING.md 1.1 maps it to `usize`, which is how the other `lib/` mirrors
  spell it in a declaration (`string.rs` 32/36, `unistd.in.rs` 39), with C's spelling kept in the
  `/// C:` line.
- **The two declarations are carried although no item list asks for them** (`## Functions
  defined — 0` counts definitions, not declarations), because C 44-53 is live text: they sit in
  `pub mod c_strcase_decls` with a glob import (PORTING.md 0.1-10/P-17/P-21), the shape
  `haiku_support.rs` uses for a header whose definitions live in other units - here
  `lib/c-strcasecmp.c` and `lib/c-strncasecmp.c`, whose ports are the two consumers.  C's
  parameters are named, so the port keeps C's names rather than `argN`.
- **`_GL_ATTRIBUTE_PURE` is a no-op and stays in the `/// C:` line.**  It is
  `__attribute__ ((__pure__))` - a purity hint whose Rust counterpart is the absence of the
  attribute (PORTING.md 8; `attribute.rs` 303 already carries the name as a no-op for the
  definitions that keep C's token), so the mirror does not `include!` `attribute.rs` for it.
- **The two consumers are wired**, which is also the mirror's compile evidence:
  `femacs/lib/c-strcasecmp.rs` and `femacs/lib/c-strncasecmp.rs` now `include!` it, with their
  own definitions shadowing the mirror's declarations (PORTING.md 0.1-10) - their `--check`s and
  `check-mirror-dupes` are unchanged (Evidence).
- No shared-table row is needed: `--check` compares the guard's presence and the declarations are
  gate-free text (the `w32select.rs` situation).

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/c-strcase.h femacs/lib/include/c-strcase.rs
OK emacs/lib/c-strcase.h -> femacs/lib/include/c-strcase.rs

# the negative control (the guard is the only inventoried item)
$ grep -v "C_STRCASE_H" femacs/lib/include/c-strcase.rs > /tmp/gt-neg2/n4.rs
$ python3 tools/extract-abi.py --check emacs/lib/c-strcase.h /tmp/gt-neg2/n4.rs
- missing macro C_STRCASE_H
FAIL emacs/lib/c-strcase.h -> /tmp/gt-neg2/n4.rs

# the two consumers build, with the declarations shadowed by their own definitions
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-gt cargo run -q -p xtask -- build-tu lib/c-strcasecmp.c
built lib/c-strcasecmp.rs -> ../build/femacs/c-strcasecmp.o
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-gt cargo run -q -p xtask -- build-tu lib/c-strncasecmp.c
built lib/c-strncasecmp.rs -> ../build/femacs/c-strncasecmp.o
$ python3 tools/extract-abi.py --check emacs/lib/c-strcasecmp.c femacs/lib/c-strcasecmp.rs
OK emacs/lib/c-strcasecmp.c -> femacs/lib/c-strcasecmp.rs
$ python3 tools/extract-abi.py --check emacs/lib/c-strncasecmp.c femacs/lib/c-strncasecmp.rs
OK emacs/lib/c-strncasecmp.c -> femacs/lib/c-strncasecmp.rs
$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ rg -n '#include "c-strcase.h"' emacs/lib
emacs/lib/c-strcasecmp.c:20:#include "c-strcase.h"
emacs/lib/c-strncasecmp.c:20:#include "c-strcase.h"

# the file is not on the verified list (the guard's `#ifndef` is the evaluator's one
# disagreement), so `extract-abi.py` reads both arms and the mirror records the dead one
$ python3 tools/check-cond-eval.py lib/c-strcase.h
DISAGREE lib/c-strcase.h:18 `ifndef C_STRCASE_H` - clang False, evaluator True
lib/c-strcase.h: 4 conditions, 0 unknown, 1 disagreements [NOT VERIFIED]
```
