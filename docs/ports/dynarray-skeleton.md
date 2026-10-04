# emacs/lib/malloc/dynarray-skeleton.c (528 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] DYNARRAY_FREE  (line 207, static)

## Macros defined — 7
- [x] DYNARRAY_CONCAT0  (line 149, body 542c9cc27da7)
- [x] DYNARRAY_CONCAT1  (line 150, body 2dba12282fb0)
- [x] DYNARRAY_FREE  (line 155, body d654b07cb38f)
- [x] DYNARRAY_HAVE_SCRATCH  (line 120, body 356a192b7913)
- [x] DYNARRAY_INITIAL_SIZE  (line 118, body 11c34e0d68c0)
- [x] DYNARRAY_NAME  (line 151, body 762ddb937b50)
- [x] DYNARRAY_SCRATCH  (line 161, body eef19c54306d)

## Types and their fields — 1
- [x] DYNARRAY_STRUCT: struct dynarray_header dynarray_abstract, size_t used, size_t allocated, DYNARRAY_ELEMENT* array, <aggregate> dynarray_header, <aggregate> u, DYNARRAY_ELEMENT scratch

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`emacs/lib/malloc/dynarray-skeleton.c` @ `fd7afced403`; port
`femacs/lib/malloc/dynarray-skeleton.rs`.  The reference is a **template** (C 19-86 documents
its parameters), and Rust has no textual include or `##` pasting, so PORTING.md 10.13(2)
applies: the shared text lands as the prelude `macro_rules! dynarray_skeleton_body` and every
instantiation point goes through it.  This file **instantiates nothing** (the
`lib/at-func.rs`/`lib/ftoastr.rs` shape): C's includers do, and the frozen build has exactly
one.

**Instantiation points.**  `lib/regexec.c` 1437-1441 (`DYNARRAY_STRUCT regmatch_list`,
`DYNARRAY_ELEMENT regmatch_t`, `DYNARRAY_PREFIX regmatch_list_`) ->
`femacs/lib/regexec.rs`, which keeps the three `macro_rules!` bindings C's `#define`s write,
includes this mirror at C 1441's position and invokes `dynarray_skeleton_body!` once.
`lib/malloc/dynarray_*.c` do **not** instantiate it: they include `<dynarray.h>` with none of
the three names defined, so the wrapper takes its `#else` arm (`<malloc/dynarray.h>`).

**Parameterisation.**  C's `DYNARRAY_NAME (x)` is `DYNARRAY_PREFIX##x`, and Rust cannot paste
identifiers (`concat_idents!` is not stable; PORTING.md P-37's `BVAR` decision), so the macro
takes the *pasted names* as arguments - the same shape `DEFUN!` uses for `$cname` and
`ftoastr_body!`/`at_func_body!` for their entries.  The 24 arguments are:

| macro argument | C |
| --- | --- |
| `$struct` | `DYNARRAY_STRUCT` |
| `$u` | the anonymous `union { … } u`'s port name |
| `$header` | the anonymous `struct { size_t used; size_t allocated; DYNARRAY_ELEMENT *array; }`'s port name |
| `$element` | `DYNARRAY_ELEMENT` |
| `$initial_size` | `DYNARRAY_INITIAL_SIZE` |
| `$name_free__elements__` … `$name_finalize` (19) | `DYNARRAY_NAME (free__elements__)` … `DYNARRAY_NAME (finalize)`, in C's declaration order (`DYNARRAY_FREE` is `DYNARRAY_NAME (f)` + `ree`) |

The seven parameter macros stay as records so the ledger and a reader can see where each name
comes from: `DYNARRAY_CONCAT0`/`DYNARRAY_CONCAT1`/`DYNARRAY_NAME`/`DYNARRAY_FREE` (C 143-156,
the pasting) and `DYNARRAY_HAVE_SCRATCH` (C 109-121), `DYNARRAY_SCRATCH` (C 157-161) and
`DYNARRAY_INITIAL_SIZE` (C 115-121, whose token `DYNARRAY_ELEMENT` is the macro parameter's
own name so the comparison sees C's spelling).  `DYNARRAY_SCRATCH!` and
`DYNARRAY_INITIAL_SIZE!` are the two the expanded body actually invokes; C's
`(list)->scratch` decays an array to a pointer, which Rust does not, so the macro spells the
same pointer with `as_mut_ptr`.

**The arms this build selects** (the macro emits only these; each other arm is a one-line note
at its C position here and in `docs/ports/regexec.md`):

- `DYNARRAY_ELEMENT_FREE`/`DYNARRAY_ELEMENT_INIT` undefined -> `free__elements__` is empty
  (C 170-178), `emplace__tail__` has neither `DYNARRAY_ELEMENT_INIT` nor `memset` (C 325-332)
  and `resize` takes C's `#else` arm, `__libc_dynarray_resize` (C 383-406).
- `DYNARRAY_INITIAL_SIZE` undefined by every includer -> C 115-121's default
  (`sizeof (regmatch_t) > 64 ? 2 : 128 / sizeof (regmatch_t)`), measured as 8 for
  `sizeof (regmatch_t) == 16` (a `/tmp` probe), and `DYNARRAY_HAVE_SCRATCH` = 1, so the
  `scratch` member is present.
- `DYNARRAY_FINAL_TYPE` undefined -> C 488-513's `size_t *lengthp` variant of `finalize`
  (C 447-513's `#ifdef` arm is dead).

**Deviations and notes.**

- The `static`/`static inline` qualifiers are not carried (they are linkage/optimization
  hints; the instantiation text this macro replaces did not carry them either, and
  PORTING.md 10.6 gives `_GL_*_INLINE` the same treatment).
- The template's C 88 `#include <malloc/dynarray.h>` and C 92-94's `<errno.h>`, `<stdlib.h>`,
  `<string.h>` are the *instantiation site's* mirrors: this file carries no `include!`, and
  the expanded body's `dynarray_header`, `__dynarray_error_marker`, `__libc_dynarray_*`,
  `__glibc_unlikely`, `free`, `memset`, `errno`, `ENOMEM` and `null_mut` are in scope where
  the macro is invoked (C 88 is a no-op there, since `lib/regexec.c` already reaches the shared
  header through `regex_internal.h` 35).
- C 1-17's opening block is not carried (PORTING.md 0.1-19); C 19-86's parameter documentation
  is carried verbatim.
- **Line count**: C 528 vs the port's 446 (15.5%).  The delta is the licence block that is not
  carried, the pasting records and the per-arm notes; a `dynarray-skeleton 0.17` row (15.5% measured) in
  `docs/ports/line-drift-tolerance.tsv` is needed (`total` basis).

## Evidence

- `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-dyn cargo run -q -p xtask -- build-tu
  lib/malloc/dynarray-skeleton.c` -> `built lib/malloc/dynarray-skeleton.rs ->
  ../build/femacs/dynarray-skeleton.o` (no reference object of its own exists: the template is
  only ever included).
- `python3 tools/extract-abi.py --check emacs/lib/malloc/dynarray-skeleton.c
  femacs/lib/malloc/dynarray-skeleton.rs` -> `FAIL` with exactly **one** problem:
  `- line count drifted 15.5%: C 528 vs Rust 446`.  The 9 checklist boxes stay unchecked until
  that row lands (the ledger rule is "only a green `--check` ticks a box").
- manual `rustc --edition 2024 --crate-type=lib --emit=metadata ... units/dynarray-skeleton/
  _root.rs` -> exit 0.
- The instantiation's own evidence is in `docs/ports/regexec.md`.
