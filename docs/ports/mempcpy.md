# emacs/lib/mempcpy.c (33 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] mempcpy  (line 28)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The identifier is renamed by gnulib's header, the symbol is not.**  The frozen
  configuration leaves HAVE_MEMPCPY undefined, so `#if !HAVE_MEMPCPY`'s body is live, and
  `<string.h>` turns the identifier `mempcpy` into `rpl_mempcpy` in this translation unit:
  C's object emits `rpl_mempcpy` (the symbol doprnt.rs declares).  The port keeps C's
  function name - which is what the ledger compares - and emits C's symbol with
  `#[unsafe(export_name = "rpl_mempcpy")]` (PORTING.md 10.13; Rust forbids combining it with
  `no_mangle`, and extract-abi now accepts either).
- **`memcpy` is declared in the unit**, as the only library entry the body calls.
