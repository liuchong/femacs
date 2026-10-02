# emacs/src/doprnt.c (607 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 6
- [x] parse_format_integer  (line 133, static)
- [x] doprnt_non_null_end  (line 154, static)
- [x] doprnt  (line 187)
- [x] esprintf  (line 552)
- [x] exprintf  (line 571)
- [x] evxprintf  (line 587)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/doprnt.c` @ `fd7afced403`; port `femacs/src/doprnt.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/src/doprnt.c femacs/src/doprnt.rs` -> `OK`.

- **Variadics (P-04)**: `docs/spikes/2026-10-02-variadics.md` fixes the mapping and the
  toolchain.  A `...` parameter *is* the `VaList` (no `va_start`/`va_end`), `va_arg` is
  `next_arg::<T> ()`, `va_copy` is `clone ()`, and C's by-value `va_list` pass (a struct
  copy on this target) is a `clone ()` at the call site, because a plain pass *moves* in
  Rust.  `spikes/va-probe.sh` proves the three shapes agree with C (6 lines identical).
  The toolchain is pinned to `nightly-2026-10-01`: `c_variadic` is stable from 1.99.0, but
  the dated `nightly-2026-09-14` predates the stabilization, and the rolling channel drifts.
- **`goto doit` / `goto doit1` / `goto signed_pD_modifier`**: three *forward* jumps inside
  one loop iteration, two of them into a shared tail.  The port keeps the shared code where
  C puts it and drives it with a `tail` flag (0 ordinary character, 1 `doit`, 2 `doit1`);
  the unsigned cases' jump into the signed `pD` block is the same code path, so both
  conversions call it.  No state machine is needed, and no helper is invented.
- **C spellings the port expresses differently (all recorded in PORTING.md)**: `TYPE_MAXIMUM
  (ptrdiff_t)` (gnulib `intprops.h`) is `isize::MAX`; `min`/`max` are `.min()`/`.max()`;
  `ckd_mul`/`ckd_add` are `overflowing_mul`/`overflowing_add` (P-15, with the wrapped value
  and the overflow flag, as gnulib defines them); `memcpy` is `copy_nonoverlapping`;
  `mempcpy` is `rpl_mempcpy` (the frozen config has no `HAVE_MEMPCPY`, so gnulib's
  replacement is what C calls); the `pD`/`pI` macros come from `lisp.rs`.
- **New externs declared by this unit** (C gets them from `<stdio.h>`/`<string.h>`, whose
  WIP mirrors do not carry them yet, P-26): `sprintf` (variadic), `strnlen`, `strncmp`,
  `rpl_mempcpy`, `xpalloc`.  `PRIdMAX` has no `inttypes` mirror yet, so the value the C
  compiler reports for the frozen target (`jd`, verified with
  `printf ("[%s] [%s] [%s]\n", pD, pI, PRIdMAX)` -> `[t] [l] [jd]`) is a local constant.
- **`<float.h>`**: gnulib has no wrapper, so `lib/include/float.in.rs` is a *system* mirror
  (values read from the C compiler: `2 24 128 -125 308`); `lisp.rs` includes it where
  lisp.h includes `<float.h>`, and `LOG2_FLT_RADIX`/`IEEE_FLOATING_POINT` are now C's
  expressions instead of folded constants.
- **`exprintf`** is gated in C on `HAVE_X_WINDOWS && USE_X_TOOLKIT` (doprnt.c 561); the
  frozen configuration has neither, so the port wraps it in `#[cfg(feature = "x_windows")]`
  with a `const _` assertion tying the feature to the frozen constants (P-08);
  `USE_X_TOOLKIT` was added to `docs/baseline/config/darwin-arm64.json` for that assertion.
- **`error`/`verror`** were added to `lisp.rs`'s `other_units` module (lisp.h 4835-4836;
  `AVOID` is `-> !`), which is also the port's first variadic *call*.
- Call-relationship differences (D7): `doprnt_non_null_end` and `doprnt` forward the list by
  value (`doprnt` receives it moved), which is what C's by-value pass means here; nothing
  else differs.

- Evidence: `--check` -> OK; `xtask build-tu src/doprnt.c` -> `built src/doprnt.rs ->
  build/femacs/doprnt.o` (the first variadic unit, and the third of the A6 trial trio);
  `spikes/va-probe.sh` 6 lines identical.
