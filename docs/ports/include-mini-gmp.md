# emacs/lib/mini-gmp.h (314 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 6
- [x] MINI_GMP_LIMB_TYPE  (line 58, body bd3027fa569e)
- [x] MPZ_ROINIT_N  (line 267, body 037b59db7c9f)
- [x] __MINI_GMP_H__  (line 40, body da39a3ee5e6b)
- [x] mpn_invert_limb  (line 123, body 546d692848ce)
- [x] mpz_even_p  (line 133, body ab15a144b764)
- [x] mpz_odd_p  (line 132, body 3a054a42b2de)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern const int mp_bits_per_limb  (line 83)

## Decisions

`emacs/lib/mini-gmp.h` @ `fd7afced403`; port `lib/include/mini-gmp.rs` (292 lines).

- **What this file is.**  `<gmp.h>` has no `emacs/lib/gmp.h` of its own: gnulib's libgmp
  module *generates* one as `#include "mini-gmp.h"` (verified in the reference build:
  `build/emacs-ref/lib/gmp.h` is those two lines plus the optional `mini-mpq.h`), so this
  header is what every `<gmp.h>` user in the tree actually gets.  It is a real reference
  file with a 1:1 target, unlike the *system* headers of P-50, and it is the last piece of
  P-49: `bignum.rs` used to carry the subset it needed, and this commit moves the whole
  header here.  `lib/mini-gmp.c` (4,699 lines) stays in batch F and defines every one of
  these functions.
- **Ordering/plumbing.**  The declarations live in `pub mod gmp { … }` + `use gmp::*;`
  because `bignum.c` *defines* all of them, `mpz` included: a crate-root `unsafe extern`
  declaration would collide with the definition (E0428) the moment that unit lands, which
  is exactly the P-19/P-21/P-39 shape `lisp.rs` uses.  The typedefs, `__mpz_struct`,
  `mpz_t` and the four macros stay at the crate root (the macros must be textually before
  their users, so a unit includes this file *before* `bignum.rs`, as C's
  `#include <gmp.h>` sits inside `bignum.h`).
- **`__mpz_struct`/`mpz_t`.**  `typedef struct { int _mp_alloc; int _mp_size; mp_limb_t
  *_mp_d; } __mpz_struct` is `#[repr(C)] pub struct __mpz_struct`, and `mpz_t` keeps C's
  array-of-one typedef (`[__mpz_struct; 1]`).  The layout baseline pins `sizeof
  __mpz_struct` 16, `offsetof ._mp_alloc/._mp_size/._mp_d` = 0/4/8, `sizeof mpz_t` 16.
- **`FILE`.**  `mpz_out_str` is declared under `#if defined (FILE) …`; the frozen build
  always has `FILE` (lisp.h includes <stdio.h> before anything includes <gmp.h>), so the
  port declares the parameter as `*mut FILE` with `pub enum FILE {}` standing in for
  <stdio.h>'s opaque type until P-26 mirrors stdio.
- **Parameter names.**  C's prototypes have none, so the mirror names them positionally
  (`a`, `b`, …) beyond the first seven; nothing in the gate or the ABI depends on them.
- **`MINI_GMP_LIMB_TYPE`** is recorded in `docs/ports/excluded-macros.txt` (`kind=syntax`):
  it spells the limb type (`unsigned MINI_GMP_LIMB_TYPE mp_limb_t`), and the port writes
  the resolved `pub type mp_limb_t = c_ulong` (P-49).
- **Ungated-ness, and the fix.**  The pair's checklist has no function definitions (all 159
  prototypes are declarations), so `--check` compares the six macros and the one
  file-scope object only; the *declarations* are checked by `spikes/gmp-abi-probe.sh`,
  which regenerates a C harness out of this very file (`static RET (*p_NAME)(ARGS) =
  NAME;`) and compiles it against the reference header, so a wrong width, const-ness or
  return type is a compile error.
- Evidence: `--check` -> OK (with `MINI_GMP_LIMB_TYPE` recorded); `xtask build-tu
  src/floatfns.c` compiles with this mirror in the chain; `spikes/gmp-abi-probe.sh` and
  `spikes/syshdrs-probe.sh` are green; the layout gate asserts the four type keys.
