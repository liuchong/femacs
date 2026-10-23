# emacs/lib/ftoastr.h (162 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 17
- [x] DBL_BUFSIZE_BOUND  (line 154, body 5ec838b0018e)
- [x] DBL_STRLEN_BOUND  (line 149, body 6b30cadd1d3f)
- [x] DBL_STRLEN_BOUND_L  (line 143, body ef331f284637)
- [x] FLT_BUFSIZE_BOUND  (line 153, body 4fb5bfc821db)
- [x] FLT_STRLEN_BOUND  (line 148, body 468a34076bf9)
- [x] FLT_STRLEN_BOUND_L  (line 142, body 6a90e6de61c9)
- [x] LDBL_BUFSIZE_BOUND  (line 155, body 5c53308b618b)
- [x] LDBL_STRLEN_BOUND  (line 150, body 0b4b864d0027)
- [x] LDBL_STRLEN_BOUND_L  (line 144, body db8ea2e072b7)
- [x] _GL_DBL_PREC_BOUND  (line 116, body cbb86b6d7446)
- [x] _GL_FLOAT_DIG_BITS_BOUND  (line 99, body 356a192b7913)
- [x] _GL_FLOAT_EXPONENT_STRLEN_BOUND  (line 126, body c61575c7bca2)
- [x] _GL_FLOAT_PREC_BOUND  (line 112, body 6840a67b8aa1)
- [x] _GL_FLOAT_STRLEN_BOUND_L  (line 139, body ad47d2efdbfc)
- [x] _GL_FLT_PREC_BOUND  (line 115, body a790848efc86)
- [x] _GL_FTOASTR_H  (line 21, body da39a3ee5e6b)
- [x] _GL_LDBL_PREC_BOUND  (line 117, body 716139fe4980)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header is in the verified list, so the mirror carries the frozen text and the ledger
  carries the dead arms.**  `python3 tools/check-cond-eval.py lib/ftoastr.h` ->
  `lib/ftoastr.h: 5 conditions, 0 unknown, 0 disagreements [VERIFIED]` (the file is line 78 of
  `docs/baseline/config/cond-eval-verified.txt`), so `tools/extract-abi.py` decides its `#if`s
  against `docs/baseline/config/darwin-arm64.macros.tsv` instead of reading both arms (P-66).
  The arms the frozen configuration does **not** compile, recorded here as text rather than
  transcribed (the comp.c / treesit.c / gnutls.c arrangement `line_drift_rule` names):

  | C lines | arm | why it is dead |
  | --- | --- | --- |
  | 90-94 | `# if FLT_RADIX == 10`: the three `*_PREC_BOUND` definitions with `*_MANT_DIG` bodies | `FLT_RADIX` is 2, not 10 |
  | 100-101 | `# elif FLT_RADIX <= 16`: `_GL_FLOAT_DIG_BITS_BOUND 4` | `FLT_RADIX == 2` took the first arm |
  | 102-104 | `# else`: `_GL_FLOAT_DIG_BITS_BOUND ((int) TYPE_WIDTH (int) - 1)` | the same |
  | 23-25, 27-29, 158-160 | `#include "intprops.h"` / `<float.h>` / `<stddef.h>` and the C++ `extern "C"` guards | the includes are named by the including unit; `__cplusplus` is not defined for a C translation unit |

  `FLT_RADIX` is 2 by measurement: `docs/baseline/config/darwin-arm64.macros.tsv` 1030
  `FLT_RADIX __FLT_RADIX__` and 6219 `__FLT_RADIX__ 2`.

- **`_GL_FLOAT_STRLEN_BOUND_L` keeps C's text minus the paste operator.**  C 136 composes its
  inner names by pasting (`_GL_##t##_PREC_BOUND`, `t##_MIN_10_EXP`, `t##_MAX_10_EXP`).  Rust
  has no token pasting, and a `##` in a `macro_rules!` body is not even a legal token tree:
  measured, carrying C's text verbatim makes `build-tu lib/ftoastr.h` fail with
  `error: reserved multi-hash token is forbidden` (4 occurrences, one per pasted name).  The
  body therefore writes C's expression with the paste operator dropped -
  `_GL_ $t _PREC_BOUND`, `$t _MIN_10_EXP`, `$t _MAX_10_EXP` - which keeps exactly the
  identifiers the macro gate compares (`_GL_`, `PREC_BOUND`, `_MIN_10_EXP`, `_MAX_10_EXP`
  normalize to `_gl`, `prec_bound`, `min_10_exp`, `max_10_exp`, which is what `--check` asked
  for) and every number and call name of the expression.  Nothing in the port expands this
  macro; the three `*_STRLEN_BOUND_L` lifts above it do keep C's argument shape exactly
  (`_GL_FLOAT_STRLEN_BOUND_L ( FLT, pointlen)`), which is the text the gate compares for them.

- **`INT_BITS_STRLEN_BOUND`, `INT_STRLEN_BOUND` and `TYPE_WIDTH` have no mirror.**  They come
  from `"intprops.h"`, which is not ported (the same state `docs/ports/strftime.md` and
  `docs/ports/openat-proc.md` record: those units write the expression out at the use site
  instead).  The macro bodies here are transcriptions, so those three names stay C's text and
  no unit expands them.

- **The bound values the port actually uses are not in this mirror.**  The live values the
  frozen target reports (`_GL_FLT/DBL/LDBL_PREC_BOUND` 9/17/17, `FLT/DBL/LDBL_BUFSIZE_BOUND`
  21/30/30) are carried by `lib/ftoastr.rs`, which defined them before this mirror
  existed (its own ledger's "No `ftoastr.h` mirror yet; the header's constants are carried
  here").  The mirror spells the same names as macros (C's shape) and constants (the gate's),
  and leaves the unit's `pub const` copies in place: a macro and a `const` are different
  namespaces, so a unit that includes both keeps compiling, and the unit's own top-level
  constants shadow the glob-imported ones (PORTING.md 0.1-10).  Rewiring `lib/ftoastr.rs` to
  include this mirror and drop its copies is a unit-side change, deliberately left to that
  file's owner.

- **The five entry points are declared, not defined.**  C 53-60 declares them; the definitions
  are `lib/ftoastr.rs`'s (through the `ftoastr_body!` instantiation `lib/dtoastr.rs`).  They
  sit in `ftoastr_decls` with a glob import so the defining unit's own definitions shadow them
  (the `include/careadlinkat.rs` shape).  C's `long double` parameter is the frozen target's
  8-byte, 8-aligned type - measured, `clang -std=gnu23` prints `4 8 8 8 8` for
  `sizeof (float/double/long double)` and `_Alignof (double/long double)` - which is the port's
  `c_longdouble = c_double` (`src/editfns.rs` 197), so it is spelled `core::ffi::c_double`.

- **The mirror is its own compilation unit**: it spells every scalar with an absolute
  `core::ffi::` path instead of taking the includer's scope, because a mirror may not carry a
  `use` (E0252 against the including unit's import) and `build-tu lib/ftoastr.h` compiles the
  mirror as the crate root.  It is therefore *not* a case of the `include-sfnt.md` /
  `include-charset.md` note ("header mirrors are not standalone translation units"): those
  mirrors name `c_int` in the includer's scope and take it from there.

- **The C++ guards are dropped structurally** (PORTING.md 0.3): C 27-29 and C 158-160 wrap the
  prototypes in `extern "C" { }`, which Rust gets from the declarations' own `unsafe extern
  "C"` blocks, so there is no item to carry.  `--check` reports no
  problem line, and 11 warnings remain (`unused macro definition` for the transcribed bounds,
  `unused import` for the two glob imports), which a mirror cannot silence: an inner attribute
  in an included file is E0753.

## Evidence

```
$ python3 tools/extract-abi.py emacs/lib/ftoastr.h
# emacs/lib/ftoastr.h (162 lines)
## Subrs (DEFUN) - 0
## Functions defined - 0
## Macros defined - 17   (one box per row: 17/17 ticked above)
## Types and their fields - 0
## File-scope objects - 0

$ python3 tools/extract-abi.py --check emacs/lib/ftoastr.h lib/include/ftoastr.rs
OK emacs/lib/ftoastr.h -> lib/include/ftoastr.rs
# no `- ` problem line (`| grep -c '^- '` -> 0)

$ python3 tools/check-cond-eval.py lib/ftoastr.h
lib/ftoastr.h: 5 conditions, 0 unknown, 0 disagreements [VERIFIED]

$ grep -n 'FLT_RADIX' docs/baseline/config/darwin-arm64.macros.tsv
1030:FLT_RADIX	__FLT_RADIX__
6219:__FLT_RADIX__	2

# the `##` control: with C's paste operator carried verbatim, the crate root rejects it
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188 cargo run -q -p xtask -- build-tu lib/ftoastr.h
error: reserved multi-hash token is forbidden        (4 occurrences)
error: rustc failed for lib/include/ftoastr.rs (exit status: 1)

# the measured bytes of C's long double, which the declarations use
$ clang --version | head -1
Apple clang version 21.0.0 (clang-2100.3.34.2)
$ clang -std=gnu23 -o /tmp/ld /tmp/ld.c && /tmp/ld
4 8 8 8 8

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188 cargo run -q -p xtask -- build-tu lib/ftoastr.h
warning: 11 warnings emitted
built lib/include/ftoastr.rs -> ../build/femacs/include_ftoastr.o
```
