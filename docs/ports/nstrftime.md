# emacs/lib/nstrftime.c (19 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] my_strftime  (line 18, body f066d916b4b7)

## Types and their fields — 0

## File-scope objects — 0


# ----------------------------------------------------------------------
## Decisions

### Summary

`femacs/lib/nstrftime.rs` is the port of `emacs/lib/nstrftime.c`, the whole of whose C text
is a rename plus an include:

    #define my_strftime nstrftime
    #include "strftime.c"

so the port keeps C's two lines as (i) a `macro_rules! my_strftime` whose body is C's
`nstrftime` (the ledger's macro) and (ii) `include!("strftime.rs")`, which brings the shared
body.  PORTING.md 10.13②'s "prelude `macro_rules!` or parameterized, instantiated once per
shim" is what `strftime.rs` now provides: `macro_rules! strftime_body { ($name:ident,
$export:literal) => … }`, and this unit is its **only** instantiation (see
`docs/ports/strftime.md`'s "Round 52" section for why the shared file itself does not
instantiate it): the `$name` is C's identifier (`my_strftime`) and `$export` is the symbol
this unit must emit (`nstrftime`), exactly the mempcpy shape (PORTING.md 10.13①).

### Order, and the one deviation in it

C writes the `#define` (line 18) *before* the `#include` (line 19), and the included text's
`#ifdef my_strftime` then suppresses strftime.c's own `#define my_strftime strftime` — i.e.
the shim's rename wins.  The port writes the include first and the rename macro after, so
that the shim's `my_strftime` shadows the shared body's own (the same "the shim's rename
wins" result); the macro is never invoked, so nothing else depends on the order.  The
`#include` line is also what C 19 is, so the two code lines are in C's position, merely
swapped.

### Why the file carries no `// femacs:` header and no long prose

C's own file is 19 lines and the ±5% drift rule cannot absorb a provenance header
(PORTING.md 10.12's wrapper-file rule, the `strftime-body`/`streq.c` precedent); the
provenance is here instead.  The file's 22 lines are C's 16-line opening comment, the crate-level
`#![allow (…)]` block the port's units carry (3 lines), C 19's include, C 18's rename and the
instantiation.  `docs/ports/line-drift-tolerance.tsv` records `nstrftime 0.20` for the
15.8% drift that costs.

### The allow list lives here, not in `strftime.rs`

An inner attribute (`#![…]`) inside an included file is E0753, so the file that
`include!("strftime.rs")` splices cannot carry one; the unit that compiles the body carries
it instead.  The consequence for `strftime.rs`'s own `build-tu` is recorded in its ledger.

### Symbol evidence (the frozen build's own objects)

    $ nm -g build/emacs-ref/lib/nstrftime.o      $ nm -g build/femacs/nstrftime.o
    T _nstrftime                                 T _nstrftime
    U ___error  U _memcpy  U _memset             the same undefined set, plus
    U _mktime_z U _revert_tz U _set_tz           `_bzero` (Rust's lowering of the
    U _strftime U _strlen  U _tolower            zeroed buffers) and Rust-internal
    U _toupper                                   panic/monomorphization symbols

i.e. the object defines `nstrftime` and nothing else, which is what makes this the port of
the unit the reference build actually compiles (`lib/Makefile` builds `nstrftime.o`; there
is no `strftime.o`).

### Verification

* `python3 tools/check-cond-eval.py lib/nstrftime.c` → `0 conditions, 0 unknown,
  0 disagreements [VERIFIED]`.
* `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target cargo run -q -p xtask -- build-tu lib/nstrftime.c`
  → `built lib/nstrftime.rs -> ../build/femacs/nstrftime.o`, 0 errors (this is also the
  build that type-checks the shared body, exactly as C compiles it only through this shim).
* `python3 tools/extract-abi.py --check emacs/lib/nstrftime.c femacs/lib/nstrftime.rs` → OK.
* `python3 tools/extract-abi.py --check emacs/lib/strftime.c femacs/lib/strftime.rs` → OK
  (the shared file's body, its 51 macros and its 7 functions are unaffected).
* No behavioural differential probe was run (the round's workflow stops at
  `build-tu`/`--check`).
