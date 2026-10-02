# emacs/lib/getopt1.c (166 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [x] getopt_long  (line 36)
- [x] _getopt_long_r  (line 44)
- [x] getopt_long_only  (line 58)
- [x] _getopt_long_only_r  (line 67)
- [x] main  (line 82)

## Macros defined — 1
- [x] ARGV_CAST  (line 32, body f003ae3b36a5)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/getopt1.c` @ `fd7afced403`; port `femacs/lib/getopt1.rs` (200 lines).  All checklist
items ticked; `build-tu lib/getopt1.c` -> `built lib/getopt1.rs -> ../build/femacs/getopt1.o`
(0 errors), `--check` -> `OK`.

- **This file is *not* in `docs/baseline/config/cond-eval-verified.txt`**, so the gate does not
  consult the frozen table for it and treats both arms of every `#if` as live text.
  `tools/check-cond-eval.py lib/getopt1.c` reports one disagreement - C 29's
  `#ifdef __getopt_argv_const_is_empty` (clang true, the evaluator false) - because the macro is
  defined by getopt-pfx-ext.h (C 47-50) rather than by config.h, which is the table's source.
  Consequences the ledger records: the checklist carries `main` from the dead `#ifdef TEST`
  arm, and `ARGV_CAST` has two definitions, so its body is not compared ("several definitions
  (configuration branches)").
- **`ARGV_CAST` (C 29-33) follows clang**: the frozen configuration defines
  `__getopt_argv_const_is_empty`, so the `#ifdef` arm - the identity - is the live one, and the
  `#else` arm's `((char **) (argv))` is named at its site (P-51).  The two arms are the same
  expression in Rust because getopt-ext.rs's prototype is already `char **argv`.
- **The `#ifdef TEST` program (C 76-166) is kept as gated text** under
  `#[cfg (feature = "test")]` with its own `unsafe extern` block for `printf` and `exit` (the
  group-member.c shape); its `main` is `#[unsafe(no_mangle)]` because C defines it non-static in
  that arm.  C's `switch (c)` becomes the port's `match c` with `if` guards for the character
  labels, which is why `docs/ports/macro-body-notes.tsv` records `main switch,case,default`
  (the doprnt precedent).
- **Symbols** (PORTING.md 10.13, measured on the reference object): `getopt_long` and
  `getopt_long_only` are in getopt-pfx-ext.h's prefix list and carry
  `#[unsafe(export_name = "rpl_getopt_long")]` / `..._only")`; `_getopt_long_r` and
  `_getopt_long_only_r` are not prefixed and keep C's names with `#[unsafe(no_mangle)]`.
  `nm -g build/femacs/getopt1.o` now defines exactly the reference's set:
  `_rpl_getopt_long`, `_rpl_getopt_long_only`, `__getopt_long_r`, `__getopt_long_only_r`.
- Line drift: `line-drift-tolerance.tsv` records `getopt1 0.25` (20.5%).
- Not verified: gate-fast/gate-probes, ERT, and any behavioural test of the two entry points.
