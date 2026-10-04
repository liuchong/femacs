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

`emacs/src/doprnt.c` @ `fd7afced403`; port `femacs/src/doprnt.rs`.

What is verified, and what is not (review A's F4, measured on this file).  The gates are
`python3 tools/extract-abi.py --check emacs/src/doprnt.c femacs/src/doprnt.rs` -> `OK`;
`xtask gate-fast` (inventory, `cargo check`, every finished translation unit in both
configurations, the probe programs' Rust side, `fam`/`shadow`/`macro`, the layout
assertions); and `xtask gate-probes` (the C differential: `inline`/`accessor`/`write`/`va`).
A twenty-mutation matrix on this file shows what each tier is worth: the token gate catches
only mutations that *delete* a token (a dropped `SIZE_BOUND_EXTRA`, `tem = 0` for
`CHAR_STRING`), the C-vs-Rust differential catches the sign of `minlen` and `tem > bufsize`,
and the whole gate set was blind to `tail = 2` -> `1`, an inverted `if`, and a changed
string literal.  The tiers are complementary; none of them proves the formatted output is
right end to end — that is the ERT ladder (`gate-full`, not implemented).

- **Variadics (P-04)**: `docs/spikes/2026-10-02-variadics.md` fixes the mapping and the
  toolchain.  A `...` parameter *is* the `VaList` (no `va_start`/`va_end`), `va_arg` is
  `next_arg::<T> ()`, `va_copy` is `clone ()`, and C's by-value `va_list` pass is a
  `clone ()` at the call site, because a plain pass *moves* in Rust on this target.  The
  platform model is recorded correctly now: on `aarch64-apple-darwin` `va_list` is
  `char *` (clang: `__builtin_types_compatible_p (va_list, char *) == 1`, `sizeof == 8`),
  so C's by-value pass copies a pointer — it is *not* a struct copy (review D's F5; the
  x86-64 warning about array-typed `va_list` sharing state still stands for that target).  `spikes/va-probe.sh` proves the three shapes agree with C (6 lines identical).
  The toolchain is pinned to `nightly-2026-10-01` (PORTING.md P-04): `c_variadic` is stable
  from 1.99.0, both dated nightlies here are newer than that and both compile this unit
  without `#![feature]`, and the rolling channel drifts.  (The first version of this entry
  and of the spike document claimed `nightly-2026-09-14` fails; reviews A and D measured it
  and it compiles.)
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
- **File-scope constants this port invents** (C has no such names, so nothing in the
  artifact compares them; review A's F6 asked for them by name, P-48 tracks the general
  fix): `SIZE_BOUND_EXTRA` (C: a function-local `enum { SIZE_BOUND_EXTRA = DBL_MAX_10_EXP +
  50 }`, doprnt.c 127 — lifted to file scope so the layout of `tembuf` is visible),
  `TBUF_LEN` (= C's `char tembuf[SIZE_BOUND_EXTRA + 50]`, doprnt.c 197) and `PRIdMAX` (C:
  `<inttypes.h>`, doprnt.c 223 reads `sizeof PRIdMAX - 2`; the port has no `<inttypes>`
  mirror yet, so the value the C compiler reports for the frozen target is written out).
- **`eassert (0 <= tem)`** (doprnt.c 413) is compiled out on both sides: the frozen
  configuration has `ENABLE_CHECKING` undefined, so C's expression is
  `((void) (false && (0 <= tem)))` and the port's `#[cfg(enable_checking)]` block is gone.
  `gate-fast` compiles the `--cfg enable_checking` variant, which proves the mirror
  *compiles* in that shape — it does not evaluate it (review A's F5).
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
  frozen configuration has neither, so the port wraps it in `#[cfg(feature = "have_x_windows")]`
  with a `const _` assertion tying the feature to the frozen constants (P-08);
  `USE_X_TOOLKIT` was added to `docs/baseline/config/darwin-arm64.json` for that assertion.
- **`error`/`verror`** were added to `lisp.rs`'s `other_units` module (lisp.h 4835-4836;
  `AVOID` is `-> !`), which is also the port's first variadic *call*.
- Call-relationship differences (D7): `doprnt_non_null_end` and `doprnt` forward the list by
  value (`doprnt` receives it moved), which is what C's by-value pass means here; nothing
  else differs.

- Evidence: `--check` -> OK; `xtask build-tu src/doprnt.c` -> `built src/doprnt.rs ->
  build/femacs/doprnt.o` (the first variadic unit, and the third of the A6 trial trio);
  `xtask gate-fast` green; `xtask gate-probes` -> `inline-probe: 6081 lines identical`,
  `accessor-probe: 11`, `write-probe: 4`, `va-probe: 6`; `spikes/miri-probe.sh` clean on
  the pinned toolchain; the mutation matrix in `docs/reviews/2026-10-02-doprnt-review.md`
  (round-4 reviews A and D, every finding disposed of there).
