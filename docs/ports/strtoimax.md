# emacs/lib/strtoimax.c (70 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] Strtoimax  (line 61)

## Macros defined — 5
- [x] Int  (line 53, body c5c7aa1df4fb)
- [x] Strtoimax  (line 54, body 59dc0687ba20)
- [x] Strtol  (line 55, body 4d93dd47aa52)
- [x] Strtoll  (line 56, body 9ac4bdbe28fa)
- [x] Unsigned  (line 57, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **gnulib compiles this one source twice.**  `UNSIGNED` is not in the frozen configuration
  table (gnulib's build defines it for the second compilation only), so the evaluator asks for
  both arms and the port carries the second configuration under `#[cfg (feature = "unsigned")]`:
  the five `#define`s as cfg-paired `macro_rules!` (their bodies hold the names C's expand to),
  the `strtoul`/`strtoull` declarations and the `strtoumax` definition.
- **The definition keeps C's spelling `Strtoimax`** (a macro that expands to the build's function
  name) and emits the symbol each configuration defines through `#[unsafe(export_name = …)]`
  (`strtoimax` / `strtoumax`), the mempcpy precedent (PORTING.md 10.13).
- **`HAVE_DECL_STRTOLL` is undefined in the frozen configuration**, so C's `strtoll` declaration
  arm is the live text and the port declares `strtoll` (and `strtol`); `HAVE_DECL_STRTOULL` is
  not in the table, so C's arm for it - the bare string literal that makes a missing
  configure-time test a compile error - is kept as a constant under the same feature.
- **`static_assert` is the port's `const _: () = assert! (…)`** (the memclear class); the macro's
  keyword has no counterpart and is a row in docs/ports/macro-body-notes.tsv.
- **`intmax_t`/`uintmax_t` were added to the stdint mirror** (C reaches them through
  `<inttypes.h>`, C 26).
- **The line-count drift (C 70 vs the port's 97, 38.6%) is declared** in
  docs/ports/line-drift-tolerance.tsv with this file's reason (both configurations in one file).
- **Call-relationship differences (D7)**: none beyond the systems' own `strtol`/`strtoll` above.
