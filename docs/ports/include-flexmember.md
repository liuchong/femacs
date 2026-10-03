# emacs/lib/flexmember.h (77 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] FLEXALIGNOF  (line 39, body 22b79d4145b9)
- [x] FLEXNSIZEOF  (line 76, body 5b0a93db4009)
- [x] FLEXSIZEOF  (line 64, body fd0d7a3fa363)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`femacs/lib/include/flexmember.rs` mirrors `emacs/lib/flexmember.h`; all three of its macros
have a counterpart and `--check` is OK.

* C 36-40's `FLEXALIGNOF` has two arms; `tools/check-cond-eval.py` is VERIFIED for this
  header, and the frozen target's C11+ clang makes C 39's `_Alignof (type)` the live one,
  which the port writes as `align_of::<T> ()`.  The C spelling `_Alignof` cannot reappear in
  Rust, so the token is a `macro-body-notes.tsv` row.
* The metavariables are spelled `$type_` rather than `$t`: C's parameter is named `type`
  (a Rust keyword) and the ledger's comparison strips a trailing `_`, so `$type_` is the same
  token at every use — and the `type` alias family's occurrence count matches without a note.
* `FLEXSIZEOF` keeps C's `offsetof`/`FLEXALIGNOF` structure with `offset_of!`/`align_of`.
* `FLEXNSIZEOF` is expanded by no ported unit (only C 64's `FLEXSIZEOF` is used, by
  time_rz.c).  It is kept so the header's text and this ledger are complete; its body is C's
  place expression read through `size_of_val` (`sizeof` does not evaluate its operand in C).
  The `sizeof` token-aliases family therefore carries `size_of_val`.

