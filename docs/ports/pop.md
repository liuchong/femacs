# emacs/lib-src/pop.c (1703 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file is on the verified list and every condition it contains is decided by the frozen
  table: `MAIL_USE_POP` is undefined, so the body is dead text and nothing is demanded of the
  port.  The gate is `#ifdef MAIL_USE_POP` at C line 26, and the frozen value is
  `docs/baseline/config/darwin-arm64.macros.tsv` 2691 (`MAIL_USE_POP<TAB>undef`).
- The reference object is the measurement, not an inference.  `lib-src/Makefile` 431 lists
  `pop.o` as a prerequisite of the movemail program and 435 makes it a target of its own, so
  the file *is* compiled - yet the resulting object carries no symbol (`nm` reports only its
  own `ltmp0` local marker) and `size` prints `0 0 0 0 0 0`.  The port's own object from
  `build-tu`, `build/femacs/pop.o`, reproduces both lines exactly, which is what "one-to-one"
  means for a translation unit whose frozen text is zero definitions.
- Policy (docs/status.md 3.25): a file whose *whole* body is inside a disabled `#if` is ported
  as an empty unit with the `// femacs:` note - the same treatment P-51 gives an arm whose body
  needs headers the target has not got, applied at file granularity.  docs/status.md 3.49 then
  settles the P-08/P-51 choice for this file: it leaves a file the evaluator *can* decide on the
  P-08 basis (this one is in `docs/baseline/config/cond-eval-verified.txt`, and 3.49's
  transcribe-both-arms rule is for the files the evaluator answers `unknown` for), so the dead
  region is recorded, not carried - an arm this configuration does not compile is an invented
  item, the D3 class `src/data.rs` records for the `#[cfg (feature = "modules")]` arm it
  reverted.  There is therefore no `#[cfg (feature = "mail_use_pop")]` transcription: the 1,703
  lines of POP3 client, socket setup and three platform variants (HESIOD, KERBEROS v4/v5,
  WINDOWSNT) are recorded here instead of being written out.
- What the whole body holds, for a future configuration that turns the gate on: the eighteen
  prototypes `lib-src/pop.h` declares are defined here - `pop_open`, `pop_stat`, `pop_list`,
  `pop_retrieve`/`_first`/`_next`/`_flush`, `pop_top_first`/`_next`/`_flush`,
  `pop_multi_first`/`_next`/`_flush`, `pop_delete`, `pop_noop`, `pop_last`, `pop_reset`,
  `pop_quit`, `pop_close` - with the seven static helpers `socket_connection`, `pop_getline`,
  `sendline`, `fullwrite`, `getok`, `pop_trash`, `find_crlf`; the file-scope objects `pop_error`
  (C 133, `ERROR_MAX` 160) and `pop_debug` (C 134); the macros `ERROR_MAX` 160, `POP_PORT` 110,
  `POP_SERVICE` `"pop3"` plus `KPOP_PORT`/`KPOP_SERVICE` under KERBEROS; and two literal-`#if 0`
  regions (C 117-119's `gettermination` prototype, C 1455-1485's definition) that are dead in
  every configuration.
- **Use-point declarations (PORTING.md 0.1-3): nothing to route.**  C's only use of the
  `<pop.h>` declarations is `#include <pop.h>` at C 53, and that line is inside the same dead
  region, so this unit reaches neither the `pop_decls` prototypes nor the `popserver` record and
  repeats no declaration.  The mirror is untouched, and its own `--check` remains the authority
  for its eight boxes.  One consequence this lane cannot fix: `docs/ports/include-pop.md` 4's
  layout assertion block is still reached by no unit, because both consumers of the header use
  it only in configuration-dead text - the `build-tu` proof item 4 expects from "when movemail.c
  or pop.c lands" does not materialise.
- **Line-count relation 0** (a 30-line note against 1,703 C lines).  The
  `docs/ports/line-drift-tolerance.tsv` row is not this lane's file, so `--check`'s only
  remaining problem is the drift until the integrator appends it; every other rule is clean.

## Evidence

- `python3 tools/check-cond-eval.py lib-src/pop.c` ->
  `lib-src/pop.c: 31 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- `make -C build/emacs-ref/lib-src pop.o` -> `CC pop.o`;
  `nm build/emacs-ref/lib-src/pop.o` -> `0000000000000000 t ltmp0`;
  `size build/emacs-ref/lib-src/pop.o` -> `__TEXT __DATA __OBJC others dec hex` /
  `0 0 0 0 0 0`.
- `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-pop cargo run -q -p xtask -- build-tu
  lib-src/pop.c` -> `built lib-src/pop.rs -> ../build/femacs/pop.o`; `nm` and `size` on the
  port's object print the reference's two lines unchanged.
- `python3 tools/extract-abi.py --check emacs/lib-src/pop.c femacs/lib-src/pop.rs` ->
  `- line count drifted 98.2%: C 1703 vs Rust 30`, then
  `FAIL emacs/lib-src/pop.c -> femacs/lib-src/pop.rs`.  One problem, and it is the tolerance
  row: the 0/0 checklist leaves no box unticked.
- `rustc --edition 2024 --crate-type=lib --emit=metadata --crate-name pop_check -C panic=abort
  -C codegen-units=1 -C opt-level=0 ../build/femacs/units/pop/_root.rs -o /tmp/pop.rmeta` ->
  exit 0.
- `check-globals` -> `check-globals: 274 units, 0 DEFVAR_* variables without a definition in
  their own unit`; `check-declarations` -> `8 declaration sites (0 forbidden, 8 allowed
  literals), 0 failures`; `check-mirror-dupes` -> `1 duplicate declaration(s), 1 ratchet
  row(s), 0 failure(s)`; `check-verbatim` -> `6 trees, 4607 files, 0 failures`.
- There is no behavioural evidence and none is possible in this configuration: the reference
  compiles no definition from the file either.
