# emacs/lib/stpcpy.c (49 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __stpcpy  (line 36)

## Macros defined — 1
- [x] __stpcpy  (line 31, body fca2361a49d6)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- `weak_alias` is not defined for this target, so C 30-32's `#ifndef` arm is live: the
  preprocessor renames the definition below to `stpcpy`.  The port keeps **C's source name**
  (`__stpcpy`) on the item and emits the real symbol with
  `#[unsafe(export_name = "stpcpy")]` (PORTING.md 10.13①, the mempcpy precedent: the two
  cannot be combined, and `--check`'s export rule accepts either).
- The `__stpcpy` macro keeps its ledger row as a nullary `macro_rules!` whose body is C's
  replacement text (`stpcpy`) — the rename the port expresses with `export_name`.
- C 26-28 (`#ifdef _LIBC` undefining `stpcpy`) and C 47-49 (`#ifdef weak_alias` calling the
  alias) are both dead; the second's one live-looking line is a *call*, not a definition, and
  is named here for the record.
- C 41-43's `do *d++ = *s; while (*s++ != '\0');` becomes a `loop` whose body performs C's
  two post-increment steps in C's order (PORTING.md 2.2); `return d - 1` is `d.offset (-1)`.
- Line-count drift (C 49 vs the port's 59) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
