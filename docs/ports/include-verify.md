# emacs/lib/verify.h (407 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 24
- [x] GNULIB_defined_struct__gl_verify_type  (line 201, body 356a192b7913)
- [x] _ALLOW_KEYWORD_MACROS  (line 307, body 356a192b7913)
- [x] _GL_CONCAT  (line 170, body ea3352f99989)
- [x] _GL_CONCAT0  (line 171, body 0ce45b6b6634)
- [x] _GL_COUNTER  (line 181, body a51b95694c81)
- [x] _GL_EXPAND  (line 300, body 11f6ad8ec52a)
- [x] _GL_GENSYM  (line 186, body 1f85e34b1b33)
- [x] _GL_HAS_BUILTIN_TRAP  (line 323, body b6589fc6ab0d)
- [x] _GL_HAS_BUILTIN_UNREACHABLE  (line 334, body b6589fc6ab0d)
- [x] _GL_HAVE__STATIC_ASSERT  (line 41, body 356a192b7913)
- [x] _GL_HAVE__STATIC_ASSERT1  (line 45, body 356a192b7913)
- [x] _GL_SA1  (line 301, body 390bc31f5f3a)
- [x] _GL_SA2  (line 302, body 9f3c943e7622)
- [x] _GL_SA3  (line 303, body 9f3c943e7622)
- [x] _GL_SA_PICK  (line 304, body 2bfbe0bc5327)
- [x] _GL_VERIFY  (line 231, body 9a8c2ea5461d)
- [x] _GL_VERIFY_H  (line 21, body da39a3ee5e6b)
- [x] _GL_VERIFY_TRUE  (line 192, body ff4ac0f21baf)
- [x] _GL_VERIFY_TYPE  (line 212, body 1ed0e1124595)
- [x] _Static_assert  (line 267, body dd76aea63f76)
- [x] assume  (line 402, body f6d4f88a5042)
- [x] static_assert  (line 309, body ccfa477ead56)
- [x] verify  (line 362, body fcaa5a14f662)
- [x] verify_expr  (line 350, body 8e91a8ebf185)

## Types and their fields — 1
- [x] _gl_verify_type: unsigned int _gl_verify_error_if_negative : w

## File-scope objects — 0

## Decisions

`emacs/lib/verify.h` @ `fd7afced403`; port `lib/include/verify.rs`.  Every checklist
item is ticked and `extract-abi.py --check` reports `OK` for the pair.

**The arms the frozen target takes** (clang, `-std=gnu23`; `docs/baseline/config/
darwin-arm64.macros.tsv` has `__STDC_VERSION__` 202311L, `__GNUC__` 4.2.1, `__clang_major__`
21).  The port spells these arms, as P-08 requires:

- `_GL_HAVE__STATIC_ASSERT` and `_GL_HAVE__STATIC_ASSERT1` are 1 (C 41/45).
- `_GL_VERIFY (R, DIAGNOSTIC, ...)` is `static_assert (R, DIAGNOSTIC)` (C 227);
  `_GL_VERIFY_TYPE` is the `_GL_HAVE__STATIC_ASSERT` arm, `struct { _Static_assert (R,
  DIAGNOSTIC); int _gl_dummy; }` (C 209-211).
- `_GL_HAS_BUILTIN_TRAP` and `_GL_HAS_BUILTIN_UNREACHABLE` are both 1 (clang's `__GNUC__`
  is 4, so C 325's test holds, and `__has_builtin` answers 1 for `__builtin_unreachable`), so
  `assume` is C 389's `((R) ? (void) 0 : __builtin_unreachable ())`.
- The `#ifdef _GL_STATIC_ASSERT_H` block (C 240-317) is **dead text** for a plain
  `#include <verify.h>`: it is live only when `lib/assert.in.h` includes the header after
  defining that select.  The mirror still carries `_Static_assert`, `_GL_EXPAND`,
  `_GL_SA1`/`_GL_SA2`/`_GL_SA3`/`_GL_SA_PICK`, `_ALLOW_KEYWORD_MACROS` and `static_assert`
  because the header is **not** in `docs/baseline/config/cond-eval-verified.txt`, so
  `extract-abi.py` reads every `#if` arm as live (P-08's blind default; P-66's verified list
  is the mechanism that would let it decide, and adding a header to it is the integrator's
  call, not a mirror's).

**The four token-pasting names.**  Rust has no preprocessor token pasting (PORTING.md 2.1;
the `KVAR`, `DOHASH`, `STAT_TIMESPEC` and `NUMNAME` rows in `docs/ports/excluded-macros.txt`
are the same construct).  `_GL_CONCAT0 (x, y)` is `x##y`, so it yields the name its caller
passes already pasted - the shape `DYNARRAY_CONCAT0` uses
(`docs/ports/dynarray-skeleton.md`, "Parameterisation").  `_GL_COUNTER` takes C's own
fallback arm: Rust has no `__COUNTER__`, and `line! ()` is the same kind of per-use integer
as the reference's `__LINE__` arm (C 177-183).  `_GL_GENSYM` therefore yields the counter
rather than a pasted name; nothing expands it, because the only user is C 233-238's
`_GL_VERIFY` fallback, which this configuration does not take.

**`static_assert` is defined here, not in `lisp.rs`.**  PORTING.md 2.5 says the port's
`static_assert` (C23's keyword) is "lisp.rs 里的同名 `macro_rules!`".  `extract-abi.py` names
the macro in this header (C 309, and again in the two C++ trick arms), and the mirror has to
carry the name, so the *definition* is here and `src/include/lisp.rs` includes this
file at its C 44 position (`#include <verify.h>`) instead of declaring its own.  The macro is
still in lisp.rs's scope for the ~200 units that include that mirror, which is what 2.5
needs, and PORTING.md 0.1-3's "one name, one place" now holds: before this change
`macro_rules! static_assert` and `macro_rules! assume` existed twice in the tree (lisp.rs and
this mirror), so the first unit that mirrored a header including both would have hit `E0428`.
2.5's sentence may want the pointer (integrator's file).

**Convergence.**  `src/include/lisp.rs`: the local `assume` and `static_assert`
definitions were deleted and the mirror was added to its include block at C's position.
Nothing else in the tree defined a name this mirror adds (`check-mirror-dupes.py` stayed at
its one ratchet row), and the four `lib/` units that keep a local `static_assert!` for the
C23 keyword (`mktime.rs`, `nanosleep.rs`, `file-has-acl.rs`, `filevercmp.rs`) do **not**
include `<verify.h>` in C, so they are not in this mirror's include graph and were left
alone.

**The type.**  `_gl_verify_type` is the **C++ arm**'s `template <int w> struct
_gl_verify_type { unsigned int _gl_verify_error_if_negative: w; };` (C 199-202) - the C
configuration this port is (P-01) never has it, `_GL_VERIFY_TYPE` taking the
`_Static_assert` arm instead.  The mirror keeps the name and its single member as the
bitfield's `unsigned int` storage unit (PORTING.md 1.3/P-80) so the C++ arm's text is
readable.  No layout key exists for it in `docs/baseline/layout/c-layout-darwin-arm64.tsv`
(the reference's C target has no such type), so `xtask layout-asserts` generates no
assertion for it - there is no C number to assert against.

`_GL_VERIFY_TYPE!` is spelled as an array whose length is a const block holding the check
(`[c_int; { const _: () = assert! (…); 1 }]`), so the type stays usable in `size_of` where
C's `sizeof` uses it; `_GL_VERIFY`/`_GL_SA*`/`_Static_assert` invoke the item-producing
`static_assert!` with **braces** (`static_assert! { … }`), which Rust requires for a macro
that expands to items.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/verify.h lib/include/verify.rs
  (13 C macros have several definitions (configuration branches), body not compared:
   _GL_COUNTER, _GL_HAS_BUILTIN_TRAP, _GL_HAS_BUILTIN_UNREACHABLE, _GL_SA1, _GL_SA2,
   _GL_SA3 ...)
OK emacs/lib/verify.h -> lib/include/verify.rs

$ cd /tmp/femacs-probe && rustc --edition 2024 --crate-type=lib --emit=obj \
    -C panic=abort verify_probe.rs -o /tmp/femacs-probe/verify_probe.o
exit=0        # the probe is the unit's `#![allow (…)]`, `use std::ffi::{c_int, c_long,
              # c_uint}`, `include! ("/tmp/verify.rs")`, then one use of each macro:
              # static_assert!/verify! as items, assume!/verify_expr! in a body,
              # size_of::<_GL_VERIFY_TYPE! (…)> (), _GL_GENSYM!/_GL_COUNTER!/_GL_SA_PICK!/
              # _GL_EXPAND! as expressions and every `pub const` in a `const` initialiser.

$ cd femacs && ./target/debug/xtask build-tu lib/regex_internal.c
built lib/regex_internal.rs -> ../build/femacs/regex_internal.o      # exit 0
# lib/regex_internal.h 39 is C's `#include <verify.h>`, and DEBUG_ASSERT (regex_internal.h
# 36-44) is the `assume` arm; the build compiles the mirror inside a real include graph.

$ cd femacs && ./target/debug/xtask build-tu src/bignum.c
built src/bignum.rs -> ../build/femacs/bignum.o                     # exit 0
# the lisp.rs path: `eassume!` (bignum.rs 347, 571) now expands `assume!` out of the
# verify.h mirror, with lisp.rs's own copy deleted.

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
```

The stale pointers this work leaves for the integrator (they say `<verify.h>` has no mirror,
or that `verify_expr` is only recorded, at `src/alloc.rs` 3411, `src/
chartab.rs` 218 and `src/include/charset.rs` 11/494-496) are *not* edited here: the
first two are other units' text and the last is `charset.rs`'s note for its
`ENCODE_CHAR`, which `docs/ports/macro-body-notes.tsv` already carries as a row
(`verify_expr`, a file this lane must not edit).
