# emacs/src/ccl.h (88 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 8
- [x] CCL_STAT_INVALID_CMD  (line 33, body 77de68daecd8)
- [x] CCL_STAT_QUIT  (line 35, body 1b6453892473)
- [x] CCL_STAT_SUCCESS  (line 30, body b6589fc6ab0d)
- [x] CCL_STAT_SUSPEND_BY_DST  (line 32, body da4b9237bacc)
- [x] CCL_STAT_SUSPEND_BY_SRC  (line 31, body 356a192b7913)
- [x] CHECK_CCL_PROGRAM  (line 82, body a775b3626306)
- [x] CODING_SPEC_CCL_PROGRAM  (line 73, body 8110e1555d5b)
- [x] EMACS_CCL_H  (line 27, body da39a3ee5e6b)

## Types and their fields — 2
- [x] ccl_program: ptrdiff_t idx, int size, Lisp_Object* prog, int ic, int eof_ic, int reg, int status, int buf_magnification, int stack_idx, int consumed, int produced, bool_bf last_block : 1, bool_bf quit_silently : 1
- [x] ccl_spec: struct ccl_program ccl

## File-scope objects — 0

## Decisions

- **What the mirror carries**: `EMACS_CCL_H` (the include guard), the five `CCL_STAT_*` constants,
  `struct ccl_program` with its `bits` byte, `struct ccl_spec`, `CODING_SPEC_CCL_PROGRAM`,
  the two prototypes in `ccl_decls` + glob import (this unit defines them, PORTING.md 0.1-10), and
  `CHECK_CCL_PROGRAM` last, in C's declaration order.
- **The two `bool_bf : 1` members** of `struct ccl_program` are packed into one `bits: u8` byte
  (PORTING.md 1.3, P-80); `docs/ports/token-aliases.tsv` carries the `last_block` and
  `quit_silently` families so `ccl.c`'s reads and writes stay occurrence-checked.  C's
  `CHECK_CCL_PROGRAM`'s `do { … } while (false)` is a block in the port, so its `false` is the one
  token without a match and `docs/ports/macro-body-notes.tsv` records it.
- **Layout** (measured with `clang -std=gnu23` against the frozen target; the probe is the two
  structs plus `<stddef.h>`): `sizeof (struct ccl_program) == 88`, align 8, `idx`@0, `size`@8,
  `prog`@16, `ic`@24, `eof_ic`@28, `reg`@32, `status`@64, `buf_magnification`@68, `stack_idx`@72,
  `consumed`@76, `produced`@80, `bits`@84; `sizeof (struct ccl_spec) == 88` with `ccl`@0.  The
  mirror pins them with `const _` assertions.  Neither type is in
  `docs/baseline/layout/c-layout-*.tsv` (the frozen baseline's key set is a ratchet that does not
  name them), so `xtask layout-asserts` generates no assertion for them and the measurement is
  recorded here instead.
- **PORTING.md P-70/P-72**: landing this mirror **deleted** the zero-sized `pub struct ccl_spec`
  stand-in `femacs/src/include/coding.rs` carried for `coding.h` 447 and made that field the
  ABI-equal `*mut c_void` with a `/// C:` note (P-72 ③).  That is a small edit to another unit's
  mirror, required by PORTING.md's own text ("等 `ccl.rs` 落地时同一个提交里删掉替身") and by the
  stand-in's own comment ("deletes it when ccl.rs lands"); `docs/ports/ccl.md` records it, and
  this unit does not touch `docs/ports/coding.md` (another unit's ledger).
- **`CODING_SPEC_CCL_PROGRAM`** keeps both `ccl` member names (`spec.ccl.ccl`); no ported unit
  reaches it yet (coding.c is its first reader), so the macro is declared but unexpanded, exactly
  as the union member is only a pointer in `coding.rs`.
