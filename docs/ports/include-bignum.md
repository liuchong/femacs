# emacs/src/bignum.h (135 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 7
- [x] XBIGNUM  (line 65)
- [x] mpz_set_intmax  (line 72)
- [x] mpz_set_uintmax  (line 84)
- [x] bignum_val  (line 96)
- [x] xbignum_val  (line 101)
- [x] bignum_integer  (line 110)
- [x] mpz_set_integer  (line 125)

## Macros defined — 2
- [x] BIGNUM_H  (line 23, body da39a3ee5e6b)
- [x] FASTER_BIGNUM  (line 31, body 356a192b7913)

## Types and their fields — 1
- [x] Lisp_Bignum: union vectorlike_header header, mpz_t value

## File-scope objects — 1
- [x] extern mpz_t mpz  (line 45)

## Decisions

`emacs/src/bignum.h` @ `fd7afced403`; port `src/include/bignum.rs`.

- **`<gmp.h>` is deferred (new item P-49).**  C includes `<gmp.h>`, which the frozen build
  resolves to gnulib's `lib/gmp.h` -> `lib/mini-gmp.h`.  `lib/` is batch F in
  docs/port-plan.md §4 (`lib/mini-gmp.c` is 4,699 lines), so the mirror of
  `lib/mini-gmp.h` does not exist yet: `bignum.rs` carries the subset bignum.h, bignum.c
  and floatfns.c use (7 inline functions and 16 `mpz_*`/helper declarations), quoted C
  line by C line in the reference header's own order.  Batch F
  moves those declarations into `lib/include/mini-gmp.rs` unchanged, and
  `lib/include/mini-gmp.h`'s row in docs/port-map.md stays unported until then.
- **`mpz_t` keeps C's array-of-one typedef** (`typedef __mpz_struct mpz_t[1]`): the mirror
  is `pub type mpz_t = [__mpz_struct; 1]`, so `sizeof (mpz_t)` = 16, `Lisp_Bignum`'s
  layout and `mpz[5]`'s 80 bytes all come out of the same declaration the C compiler sees.
  A `mpz_t`-shaped *parameter* decays to a pointer in C, so the declarations use
  `*mut __mpz_struct` / `*const __mpz_struct`, and a `mpz_t` *value* is passed as
  `value.as_mut_ptr ()` / `value.as_ptr ()` at the call sites (P-49).
- **The `mpz` bound is gated.**  C's `extern mpz_t mpz[5]` used to be the one layout key
  with no assertion (review B's B1: `[mpz_t; 6]` passed `gate-fast`, `extract-abi` and
  `spikes/run.sh`); `xtask layout-asserts` now resolves a key whose root is a file-scope
  object through the mirror's own declaration, so `sizeof mpz` asserts
  `size_of::<[mpz_t; 5]>() == 80` and the mutation fails the gate.
- **Layout evidence.**  `tools/gen-layout-fields.py` now generates `Lisp_Bignum`'s fields,
  `spikes/layout_c.c` prints `__mpz_struct`, `mpz_t`, `mpz` and `Lisp_Bignum`, and the
  frozen baseline grew from 207 to 218 keys (11 new: `sizeof __mpz_struct` 16,
  `offsetof __mpz_struct._mp_alloc` 0 / `._mp_size` 4 / `._mp_d` 8, `sizeof mpz_t` 16,
  `sizeof mpz` 80, `sizeof Lisp_Bignum` 24, `alignof Lisp_Bignum` 8,
  `offsetof Lisp_Bignum.header` 0 / `.value` 8, `alignof __mpz_struct` 8).  Re-running
  `spikes/run.sh` reports `shared keys: 46, mismatches: 0` and no Rust-only key, so the
  Rust declarations agree with the C compiler on every one of them.
- **The `mpz[5]` bound is gated** (review B's B1): a layout key whose root is a file-scope
  object is now resolved through the mirror's own declaration
  (`pub static mut mpz: [mpz_t; 5]`), so `sizeof mpz` asserts
  `size_of::<[mpz_t; 5]>() == 80`; `[mpz_t; 6]` fails the layout gate (it passed
  everything before).
- **Two recorded token deviations** (`docs/ports/macro-body-notes.tsv`): `GMP_NUMB_BITS`
  (`TYPE_WIDTH (mp_limb_t)` is written as `size_of::<mp_limb_t> () * 8`, because the port
  has no `CHAR_BIT` yet) and `bignum_integer` (C's cast `(mpz_t const *) tmp` is
  `(*tmp).as_ptr ()` in the port).
- **`ckd_add` in `mpz_set_intmax`/`mpz_set_uintmax`** is the `ckd_add` family of
  `docs/ports/token-aliases.tsv` (P-44), occurrence-checked rather than exempted.
- **`struct Lisp_Bignum`'s `GCALIGNED_STRUCT`** is `#[repr(C, align(8))]`, the same
  spelling the other `GCALIGNED_STRUCT` mirrors use; the layout gate pins it.
