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

- **The symbol is `rpl_mempcpy` - WRONG, see the correction at the end (2026-10-04, status 3.112).**  The **generated** `<string.h>`
  (`build/emacs-ref/lib/string.h` 1027-1028: `# undef mempcpy` / `# define mempcpy rpl_mempcpy`)
  renames the identifier in every translation unit of this configuration, so C's `mempcpy.c`
  defines `rpl_mempcpy` and its callers - `src/doprnt.c` 284's call among them - compile to it.
  The port's object emitted `_mempcpy` (a plain `#[unsafe(no_mangle)]`) while `src/doprnt.rs`
  asked for `_rpl_mempcpy`, so `xtask gate-load` could not resolve the reference; the definition
  is `#[unsafe(export_name = "rpl_mempcpy")]` now and `nm build/femacs/mempcpy.o` prints
  `T _rpl_mempcpy`.  (The reference's own `doprnt.o` shows no mempcpy symbol at all - clang
  optimises the call away - so this pair is a link issue of the port's compilation mode, which
  keeps the call at `opt-level=0`.)

**The correction (status 3.205).**  `tools/check-symbols.py --dir lib --pending
docs/baseline/symbol-pending.txt` reported `lib/mempcpy: missing=['_mempcpy']
invented=['_rpl_mempcpy']`, and the measurement settles it: the generated header's two rename
lines are inside `# if 0` (its line 1025), so the live arm is the `# else` one at 1036
(`_GL_FUNCDECL_SYS`/`_GL_CXXALIAS_SYS`, no rename) - and `nm -g build/emacs-ref/lib/mempcpy.o`
prints `T _mempcpy`.  The 3.112 reading quoted the inner lines without their enclosing condition,
the same class as 3.196/3.198/3.202.  The unit is `#[unsafe(no_mangle)]` now, `src/doprnt.rs`
calls `mempcpy` (C's own name), and the symbol gate reports `86 units compared, 0 mismatched,
1 still pending`, the pending one being `lib/regex`'s documented aggregate split.
