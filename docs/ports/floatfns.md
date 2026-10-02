# emacs/src/floatfns.c (656 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 25
- [x] acos  (Facos, 1/1, doc c949fc1d56cb, line 85)
- [x] asin  (Fasin, 1/1, doc 9068b2231204, line 94)
- [x] atan  (Fatan, 1/2, doc c63d2bba3111, line 103)
- [x] cos  (Fcos, 1/1, doc 21e86428b4d8, line 123)
- [x] sin  (Fsin, 1/1, doc 67ea2cc774a2, line 132)
- [x] tan  (Ftan, 1/1, doc 8da4ccf0d80f, line 141)
- [x] isnan  (Fisnan, 1/1, doc 4b7d8cc11d30, line 150)
- [x] copysign  (Fcopysign, 2/2, doc 6df0eb8aced2, line 164)
- [x] frexp  (Ffrexp, 1/1, doc 45f7e9d72b7e, line 182)
- [x] ldexp  (Fldexp, 2/2, doc ef8925fd59e5, line 200)
- [x] exp  (Fexp, 1/1, doc 9b9cdfe0e8d1, line 210)
- [x] expt  (Fexpt, 2/2, doc a40a7bdf14bb, line 219)
- [x] log  (Flog, 1/2, doc e7e34bc2346b, line 234)
- [x] sqrt  (Fsqrt, 1/1, doc 416c0a9daaf6, line 259)
- [x] abs  (Fabs, 1/1, doc 8305b7b63d56, line 268)
- [x] float  (Ffloat, 1/1, doc bbfa2fbd98f5, line 296)
- [x] logb  (Flogb, 1/1, doc d276a7a3d95b, line 305)
- [x] ceiling  (Fceiling, 1/2, doc 86c715e9a855, line 521)
- [x] floor  (Ffloor, 1/2, doc 335bc2662ee5, line 530)
- [x] round  (Fround, 1/2, doc 7e48eaffce19, line 539)
- [x] truncate  (Ftruncate, 1/2, doc c8244a0222b5, line 559)
- [x] fceiling  (Ffceiling, 1/1, doc 3ca76783da44, line 584)
- [x] ffloor  (Fffloor, 1/1, doc 00fed21d5a90, line 595)
- [x] fround  (Ffround, 1/1, doc b93282dfcac5, line 606)
- [x] ftruncate  (Fftruncate, 1/1, doc 36e36abc8c9e, line 616)

## Functions defined — 15
- [x] CHECK_FLOAT  (line 69, static)
- [x] extract_float  (line 77)
- [x] double_integer_scale  (line 346)
- [x] rescale_for_division  (line 379, static)
- [x] rounding_driver  (line 404, static)
- [x] ceiling2  (line 448, static)
- [x] floor2  (line 454, static)
- [x] truncate2  (line 460, static)
- [x] round2  (line 466, static)
- [x] rounddiv_q  (line 485, static)
- [x] emacs_rint  (line 505, static)
- [x] trunc  (line 515)
- [x] identity  (line 554, static)
- [x] fmod_float  (line 570)
- [x] syms_of_floatfns  (line 628)

## Macros defined — 2
- [x] emacs_rint  (line 502, body 5b8ef7192fe7)
- [x] signbit  (line 161, body 03778c78aed2)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/floatfns.c` @ `fd7afced403`; port `femacs/src/floatfns.rs` (677 lines).

What is verified, and by what.  `python3 tools/extract-abi.py --check emacs/src/floatfns.c
femacs/src/floatfns.rs` -> `OK`; `xtask build-tu src/floatfns.c` compiles the unit in both
configurations (plain and `--cfg enable_checking`); `xtask gate-fast` (inventory, `cargo
check`, every finished translation unit, the probe programs, `fam`/`shadow`/`macro`, the
layout assertions); `xtask gate-probes` for the C differential of the mirrors this unit
uses.  There is no C-vs-Rust *behaviour* differential for this unit yet: the port's
`float`/rounding semantics have not been compared against the reference binary, which is
P-30's territory and needs the loading ladder.

- **Configuration-gated arms (P-08).**  The frozen configuration has `HAVE_LOG2` = 1,
  `HAVE_RINT` = 1, `HAVE_TRUNC` = 1 and `HAIKU` undefined
  (`docs/baseline/config/darwin-arm64.json`, read from the reference
  `build/emacs-ref/src/config.h`: 1068, 1327, 1739, 330).  `Flog`'s `else if (b == 2.0)`
  arm is live and keeps the constant in the condition (`HAVE_LOG2 && b == 2.0`), which is
  what the preprocessor does when it is 0; the `#else` bodies of `emacs_rint` and `trunc`,
  and `double_integer_scale`'s `#ifdef HAIKU` block, are dead text under
  `feature = "no_rint"`, `"no_trunc"` and `"haiku"`, each tied to the frozen value by a
  `const _: () = assert! (!cfg! (…) || …)` (the `buffer.rs` shape, P-08).
- **`<math.h>` and `<limits.h>` (P-26).**  C gets every libm call and `INT_MIN`/`INT_MAX`
  from system headers the port has not mirrored yet, so they live in the system mirrors
  `femacs/lib/include/math.rs` (new: the 22 libm functions, the `HUGE_VAL`/`FP_ILOGB0`
  constants measured from the frozen target's clang, and the `isnan`/`isfinite`/`isinf`/
  `signbit` macros) and `femacs/lib/include/limits.in.rs` (extended: `INT_MIN`, `INT_MAX`).
  `DBL_MANT_DIG` (53) and `DBL_MIN_EXP` (-1021), which `double_integer_scale` and
  `rescale_for_division` need, were added to `lib/include/float.in.rs` from the same
  measured report.
- **Exported symbols (review A's F1, P-52).**  `extract_float`, `fmod_float` and
  `double_integer_scale` are non-`static` C definitions that `lisp.h` declares and five
  other units call, so they are `#[unsafe(no_mangle)] pub unsafe extern "C" fn` here (a
  plain `pub fn` had no C symbol at all), and `lisp.rs` declares them the way lisp.h does.
  `nm -g build/femacs/floatfns.o` shows `_extract_float`, `_fmod_float`,
  `_double_integer_scale`, `_syms_of_floatfns` and `_Fabs`; `extract-abi.py` now fails a
  non-`static` C definition without the attribute.
- **`signbit`.**  `floatfns.c` re-defines `signbit` under `#ifndef signbit`, but this
  target's `<math.h>` *does* define it, so the fallback is dead text (`feature =
  "no_signbit"`) and the live macro — `math.rs`'s — is `is_sign_negative`.  The two differ
  for a NaN: the fallback says "not negative" (so `(abs -nan)` would return the NaN
  unchanged), the live one keeps the sign bit, which is what the reference binary does.
  The port keeps the reference's behaviour and records the difference here.
- **Uninitialised C locals** (`double f1, f2;` in `Fcopysign`/`fmod_float`, `int exponent;`
  in `Ffrexp`, `EMACS_INT value;` in `Flogb`, `int ivalue;` in `Flogb`,
  `double (*…)` parameters) are written with a zero initialiser: Rust requires definite
  initialisation, and every one of them is assigned by the very next call (`frexp`,
  `ilogb`) or by every branch.  The one warning the unit produces is that zero.
- **`mpz_t` values** are passed as `…as_mut_ptr ()`/`…as_ptr ()`: C's `mpz_t` is the
  array-of-one typedef, so a value decays to `__mpz_struct *` at the call, which is the
  type the declarations in `bignum.rs` use (P-49).
- **`emacs_rint`.**  It is a macro in C (`rint` here) and a function only in the dead
  `#else` arm, so the port keeps both and `Fround`/`Ffround` select by the same feature;
  C's `rounding_driver (…, emacs_rint, …)` therefore reads `rint` in the live arm.
- **Function-pointer parameters** of `rounding_driver` are `unsafe extern "C" fn`, one ABI
  for the libm functions and for C's own helpers (`ceiling2`, `floor2`, `round2`,
  `truncate2`, `identity`, `rounddiv_q`), which the port also declares `extern "C"`.
- **`eabs`.**  `Flogb`/`round2` call lisp.h 5597's *macro*, for integers; the port's
  `lisp.rs` carried an `f64` function instead, which could not.  It is now the macro
  again, with C's double evaluation.
- **`ckd_*`.**  `bignum.rs`'s `mpz_set_intmax`/`mpz_set_uintmax` use `ckd_add`, which the
  port spells `overflowing_add` (P-15); the family is machine-checked by
  `docs/ports/token-aliases.tsv` (P-44) rather than exempted.
- **No Rust counterpart.**  `#if defined __FINITE_MATH_ONLY__ && __FINITE_MATH_ONLY__
  #error` is a check on the C compiler's flags; there is nothing for the port to mirror.
- **The fallback compiles** (review B's B3/A's F2): it is C's `($x) == 0.0`, not Rust's
  bitwise `!`, and `feature = "no_signbit"` is tied to the frozen `HAVE_SIGNBIT` by a
  `const _` assertion; `xtask dead-arms` compiles this unit (and the other three finished
  ones) with the non-frozen arms selected, which is what turned that arm's E0600 into a
  checked property.
- Call-relationship differences (D7): outbound, the three exports above (F1); inbound,
  none beyond the above — `syms_of_floatfns` defines the same 25 subrs in C's order, and no other unit defines a function this one calls
  (`make_float`, `expt_integer`, `double_to_integer`, `overflow_error`, `eabs`, the mpz
  helpers) — those are declared here and defined in `alloc.c`, `data.c`, `bignum.c` and
  `lisp.rs`, and P-39's rule keeps them out of this unit's own definitions.

- Evidence: `--check` -> OK; `build-tu` -> `built src/floatfns.rs ->
  build/femacs/floatfns.o` (both configurations); the layout keys of the types this unit
  touches are in docs/baseline/layout/c-layout-darwin-arm64.tsv.
