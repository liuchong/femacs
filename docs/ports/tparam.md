# emacs/src/tparam.c (284 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 4
- [x] tparam  (line 39)
- [x] tgoto  (line 57)
- [x] tparam1  (line 68, static)
- [x] main  (line 272)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 3
- [x] char BC  (line 51)
- [x] char UP  (line 52)
- [x] static char tgoto_buf  (line 54, static)

## Open items

- [ ] line-count parity with C's 284 lines.  The port is 383 lines (34.9% longer) because
  C's three labels and four fall-throughs must be spelled as one copy of each shared
  statement plus the stage variables that select the entry points, while C gets them from
  the `switch` and the two `goto`s.  The token multiset, the statement order and every C
  identifier match, which is what PORTING.md 2.2 asks for; what is left is either
  compressing the port's formatting or teaching the drift rule about structural expansion
  (the same class as P-76's `defsubr` fix).  Until then the file stays WIP by design.

## Decisions

- **The three labels and four fall-throughs are one copy of each shared statement.**  The
  `%d`/`%3`/`%2` chain is entered through `digits` (1 = `%3`, 2 = the `twodigit:` label,
  3 = `onedigit:`), the `%C`/`%+`/`%.` chain through `plus` (2 = the `%+` step) and `dot`
  (the `%.` step, whose fall-through into `%f` is `drop_arg`); C's `ordinary:` label is the
  `ordinary` flag that sends both the `else` branch and `case '%'` to the tail statement.
  Duplicating the tails instead was the first draft and is what the ledger records as
  rejected (it makes the token multiset and the line count worse, not better).
- **`ckd_add` is the `overflowing_add` pair** (P-15), as in `bignum.rs`/`doprnt.rs`.
- **`BC`/`UP`/`tgoto_buf` are the file's own objects**, `#[unsafe(no_mangle)]` where C gives
  them external linkage; the `DEBUG` main keeps C's text under `#[cfg(feature = "debug")]`
  (P-51) so the dead-arms build compiles it.
- **`memcpy`/`strlen`/`stpcpy`/`atoi` are declared in this unit's own `unsafe extern` block**
  with their `<string.h>`/`<stdlib.h>` homes, the doprnt.rs precedent.
