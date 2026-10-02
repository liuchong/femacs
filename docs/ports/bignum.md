# emacs/src/bignum.c (603 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 33
- [x] xrealloc_for_gmp  (line 40, static)
- [x] xfree_for_gmp  (line 46, static)
- [x] init_bignum  (line 52)
- [x] bignum_to_double  (line 73)
- [x] double_to_integer  (line 81)
- [x] make_bignum_bits  (line 92, static)
- [x] make_bignum  (line 112, static)
- [x] make_bigint  (line 119)
- [x] make_biguint  (line 126)
- [x] make_neg_biguint  (line 135)
- [x] make_integer_mpz  (line 146)
- [x] mpz_set_intmax_slow  (line 185)
- [x] mpz_set_uintmax_slow  (line 208)
- [x] mpz_to_intmax  (line 227)
- [x] mpz_to_uintmax  (line 268)
- [x] bignum_to_intmax  (line 305)
- [x] bignum_to_uintmax  (line 311)
- [x] emacs_mpz_size  (line 345, static)
- [x] emacs_mpz_mul  (line 359)
- [x] emacs_mpz_mul_2exp  (line 367)
- [x] emacs_mpz_pow_ui  (line 381)
- [x] mpz_bufsize  (line 399, static)
- [x] bignum_bufsize  (line 404)
- [x] mpz_get_d_rounded  (line 412)
- [x] bignum_to_c_string  (line 437)
- [x] bignum_to_string  (line 449)
- [x] make_bignum_str  (line 466)
- [x] check_integer_range  (line 480)
- [x] check_uinteger_max  (line 493)
- [x] check_int_nonnegative  (line 506)
- [x] get_random_limb  (line 515, static)
- [x] get_random_limb_lim  (line 535, static)
- [x] get_random_bignum  (line 558)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 1
- [x] mpz_t mpz  (line 37)

## Decisions

`emacs/src/bignum.c` @ `fd7afced403`; port `femacs/src/bignum.rs` (633 lines).

What is verified, and by what.  `python3 tools/extract-abi.py --check emacs/src/bignum.c
femacs/src/bignum.rs` -> `OK`; `xtask build-tu src/bignum.c` compiles the unit in both
configurations (plain and `--cfg enable_checking`); `xtask gate-fast`; `gate-probes`
(including `spikes/gmp-abi-probe.sh`, which type-checks the gmp declarations this unit
calls).  There is no C-vs-Rust *behaviour* differential for this unit yet (P-30): the
bignum arithmetic its callers depend on has not been compared against the reference
binary — round 5's reviewer A built such a harness for `floatfns.c` (a glue object plus
`lib/mini-gmp.c`), and the same shape is what this unit needs.

- **`mpz[5]`** is the one *definition* this unit owns; `mini-gmp.rs` declares it inside its
  `gmp` module (P-49) and this `#[unsafe(no_mangle)] pub static mut mpz` shadows the
  import, so no E0428 when both land in the same crate (P-19/P-21/P-39).  It is
  zero-initialised exactly as C's BSS is, and `init_bignum` runs `mpz_init` on each limb
  vector, as C does.
- **`<stdlib.h>`, `<stdcountof.h>` and gnulib's intprops.h have no mirrors yet (P-26),** so
  the unit declares what it reads: `strtod`, the three `TYPE_WIDTH` widths
  (`ULONG_WIDTH`/`INTMAX_WIDTH`/`UINTMAX_WIDTH`) and `countof!`.  The file-scope enums
  `GMP_NLIMBS_MAX`/`NLIMBS_LIMIT` are C's `min (…)` over *mixed* integer types; a Rust
  `const` needs a const-evaluable `min`, so the port adds `min_const` and records the
  token deviation in docs/ports/macro-body-notes.tsv.  `LONG_MIN`/`LONG_MAX`/`ULONG_MAX`
  come from `lib/include/limits.in.rs`, `INTMAX_MIN`/`INTMAX_MAX`/`UINTMAX_MAX` from
  `stdint.in.rs`, `DBL_DIG` from `float.in.rs`.
- **C's unsigned wraparound is spelled out.**  `-lim` (get_random_limb_lim),
  `-u` and `(two << (UINTMAX_WIDTH - 1)) - 1` (mpz_set_intmax_slow) are `wrapping_neg` /
  `wrapping_sub` in the port: Rust rejects the constant overflow that C performs silently,
  and the values are identical.
- **`do { … } while (c)` becomes `loop { … if !(c) { break; } }`** (C's post-test loops in
  `make_integer_mpz`, `mpz_set_int*_slow`, `mpz_to_int*max`, `get_random_limb_lim`,
  `get_random_bignum`), and C's decrementing `for (i = n - 1; 0 < i--; )` becomes
  `while 0 < i { i -= 1; … }`, which runs the body with the same sequence of `i` values
  (the loop variable's final value differs by one and is not read afterwards).
- **Locals C leaves uninitialised** — `limb`/`diff`/`remainder` (get_random_limb_lim),
  `edgy` (get_random_bignum), `v`/`i`/`shift` (the max conversions) — start at zero, as
  PORTING.md 1.5 records.
- **C's implicit conversions are written out**, because Rust has no narrowing: `mpz_size`'s
  `size_t` to `mp_size_t`, `int` to `mp_size_t` for `mpz_getlimbn`/`mpz_limbs_*`,
  `EMACS_INT` for `FIXNUM_OVERFLOW_P`/`make_int`/`make_uint`/`make_fixnum`, `uintmax_t`
  for the `-MOST_NEGATIVE_FIXNUM < n` comparison, `c_int` for `FIXNUM_BITS`.  Two call
  sites where the *mirror's* type was wrong were fixed in lisp.rs in the same commit:
  `make_unibyte_string`/`make_string` were declared with a `usize` length where lisp.h
  4541/4544 say `ptrdiff_t`.
- **`mpz_t` values** are passed as `…as_mut_ptr ()`/`…as_ptr ()` (P-49); C's `mpz_t const
  *lim`/`mpz_t const *pn`-shaped declarations decay to `*const __mpz_struct` and are
  recorded in docs/ports/macro-body-notes.tsv.
- **`min_const`** is the only helper the port adds (three lines); it exists because C's
  `enum { … = min (…) }` is a constant expression and lisp.rs's `min` is a runtime
  function.  P-48's residual covers it: the ledger names it, and its value is pinned by
  the arithmetic it feeds.
- Call-relationship differences (D7): inbound, `alloc.c` writes `integer_width` and
  `emacs.c` calls `init_bignum` (nothing else defines what this unit uses); outbound, the
  unit calls `make_integer_mpz`/`make_bignum_bits` internally and
  `overflow_error`/`args_out_of_range_3`/`xrealloc`/`xmalloc`/`xfree`/`get_random_ulong`
  from other units — all declared where lisp.h declares them.
- Evidence: `--check` -> OK; `build-tu` -> `built src/bignum.rs -> build/femacs/bignum.o`
  (both configurations); the four `apply`/`mpz`-typed layout keys are asserted by
  `layout-check`; `spikes/gmp-abi-probe.sh` covers the 159 gmp declarations this unit
  calls into.
