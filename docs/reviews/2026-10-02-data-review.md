# Adversarial review: `src/data.c` segment I (data.c 168-705) — 2026-10-02

Scope: `femacs/src/data.rs`'s 40 subrs (the type predicates and list accessors, data.c
168-705), the two `lisp.rs` prototypes the port added for `position-symbol`/`fset`
(`build_symbol_with_pos`, `valid_lisp_object_p`, lisp.h 4655/4661), and the ledger.

Method: two reviewers in separate contexts, each handed only the paths and the rule that
they get the change and no implementer reasoning, told to find *why it is wrong*. Both
worked read-only (`/tmp` for scratch), ran no build (the tool run was
`python3 tools/extract-abi.py --check`, ~4 s), and re-derived the docstring rule from
`lib-src/make-docfile.c` 283-473 instead of trusting the port's own digest comparison.

## Findings and disposition

| # | Finding (reviewer) | Disposition |
| --- | --- | --- |
| 1 | **`user-ptrp` inverted the frozen configuration.** The `DEFUN` (data.c 499-508) and its `defsubr` (data.c 4360-4362) are both inside `#ifdef HAVE_MODULES`, which the frozen table leaves `undef`, so the reference builds neither. The port wrote it live: an invented subr (D3). (A-P1, B-F1) | Fixed: the arm is not ported (the `#[cfg(feature = "modules")]` text tried first was reverted — with data.c in the condition table the gate reports such an arm as `the reference file has no such DEFUN`). The remaining half — the tool's `defsubr` extraction does not consult the condition table — is recorded as **P-76** in PORTING.md, and `syms_of_data` must treat `Suser_ptrp` as a ledger deviation. |
| 2 | **`MODULE_FUNCTIONP` / `XMODULE_FUNCTION` carried a `cfg` C does not have** (lisp.h 4872-4883 is unconditional; the region's `#ifdef HAVE_MODULES` opens later, at 4885). The frozen build compiled a `false` stub, so this file's `module-function-p` could never return `t`. (A-P2, B-F2) | Fixed in `lisp.rs`: both predicates are unconditional `PSEUDOVECTORP`/`XUNTAG!` again; the `not(feature = "modules")` stub is gone. `--check` of lisp.h is still OK. |
| 3 | **`NO_INLINE`/`AVOID` on the wrong functions.** C marks `slow_eq` `NO_INLINE` (data.c 159) and `wrong_length_argument` `static AVOID` (data.c 118 = `_Noreturn ATTRIBUTE_COLD void`, lisp.h 153). (B-F4) | Fixed: `#[inline(never)]` moved to `slow_eq`; the `AVOID` helper keeps `-> !` and drops `ATTRIBUTE_COLD` (PORTING.md 490-491). |
| 4 | **Ledger counts wrong**: “Four subrs wait” (actually five — `type-of` waits on this file's `cl-type-of`), “18 subrs” with attributes (15 in this range: 9 `const` + 6 `pure`), “the `A*` constants” (12: 7 `A*` + 5 `bool_vector_*`). (A-P3, A-P4, B-F3a/c/d) | Fixed in `docs/ports/data.md`; `type-of` and the four header-blocked subrs are now named one by one. |
| 5 | **Nineteen functions of data.c 41-165 were ported but unticked** (“Functions defined 0/60”), which misleads every count taken from the ledger. (B-F3b) | Fixed: the 19 boxes are ticked. |
| 6 | **The gate is blind to this file's `#ifdef`s**, because `src/data.c` was not in the condition table, which is how finding 1 stayed invisible. (B-F1's remedy) | Accepted: `tools/check-cond-eval.py src/data.c` reports 9 conditions, 0 unknown to the evaluator, 0 disagreements with clang; the whitelist is 18 files and the diff is one line. |
| 7 | Reviewer B also suggested adding `("modules", "HAVE_MODULES", "true")` to `xtask`'s `FLIPS`. | Tried together with the `#[cfg]` arm and reverted with it: with the arm gone nothing is gated on that feature, and a flip entry without a subject is dead mechanism. |

## Independent confirmations (no defect found)

Both reviewers re-checked, and agreed with the port: `Feq`'s `EQ` vs `slow_eq`'s `BASE_EQ`
(lisp.rs 787-795 ≡ lisp.h 1315-1323, three arms, no early return); `CAR`/`CDR`/`CAR_SAFE`/
`CDR_SAFE`/`XSETCAR`/`XSETCDR`; the predicate family (`RECORDP`, `ARRAYP`, `BOOL_VECTOR_P`,
`USER_PTRP`, `CLOSUREP`, `STRING_MULTIBYTE`, `CHAR_TABLE_P`, `SUBRP`, `MARKERP`, `BUFFERP`)
against their `lisp.h`/`buffer.h` definitions and the `pvec_type` values; `AREF (object,
CLOSURE_CODE as isize)` (CLOSURE_CODE is `c_int` in the mirror, `AREF` takes `isize`);
`SREF (...) == b':' as u8` (`SREF` is `unsigned char`/`u8` on both sides, and the NUL
terminator makes index 0 legal for the empty name, as in C); `mpz_sgn (xbignum_val (...))`
without the C's array-decay `*`; and every `DEFUN!` field (name, C name, S name, min/max,
intspec `0`/`NULL`) plus all 40 docstrings, byte for byte.

## Evidence after the fixes

- `cargo xtask build-tu src/data.c` → `built src/data.rs -> ../build/femacs/data.o`.
- `extract-abi.py --check emacs/src/data.c femacs/src/data.rs` → 324 items, **all of them
  "missing"** (77 subrs, 36 functions, 1 macro, 12 constants, 120 `defsubr` calls, the
  line-count drift of a partial file); no invented item left besides the documented
  `missing defsubr (&raw const Suser_ptrp)` (P-76).
- `extract-abi.py --check emacs/src/lisp.h femacs/src/include/lisp.rs` → OK.
- `extract-abi.py --all` → 17 done / 10 WIP / 0 failing.
- `xtask gate-fast` → 0 (condition evaluator, inventory, workspace, 10 translation units,
  probes, `fam`/`shadow`/`macro`, 688 layout assertions).
- `xtask gate-probes` → 0 (inline 6081, accessor 11, write 4, va 6, syshdrs 30, gmp-abi 159,
  lisp-abi 82, dead arms: bignum/casetab/doprnt/floatfns/sort).

## Residual, not a defect

Five subrs of this segment stay unported until `thread.rs` and `font.rs` exist (`type-of` →
`cl-type-of` → `THREADP`/`MUTEXP`/`CONDVARP`/`FONT_*_P`), and the file's `defsubr` list is
still empty because `syms_of_data` (data.c 4021) is not ported.
