# emacs/lib/rawmemchr.c (158 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] rawmemchr  (line 35)

## Macros defined — 1
- [x] __has_feature  (line 29, body b6589fc6ab0d)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **`#if !HAVE_RAWMEMCHR` is live text, not a `cfg` arm.**  The frozen configuration leaves
  HAVE_RAWMEMCHR undefined (docs/baseline/config/darwin-arm64.macros.tsv 916), so C 23's
  condition takes the body and the port keeps it unconditionally (PORTING.md 0.1-1).
- **The two nested arms the condition table cannot decide.**  rawmemchr.c is not one of the
  files in docs/baseline/config/cond-eval-verified.txt, so the 4956-macro table is not
  applied to it (P-66) and the extractor compares the arms as written:
  - C 28-30's `#ifndef __has_feature` fallback is recorded in
    docs/ports/excluded-macros.txt; the frozen configuration defines neither the builtin nor
    the macro, so C's own `0` is what C 56's condition saw, which makes C 56's arm live and
    C 147's `#endif` the end of the live text.  The port states that value in a comment
    instead of carrying a macro item.
  - C 37-45's `#ifdef __CHERI_PURE_CAPABILITY__` arm is dead: the macro is not in the table
    and the arm calls `__builtin_cheri_length_get`/`__builtin_cheri_offset_get`, builtins
    this target has not got.  The three tokens plus `static_assert` are the row this file
    adds to docs/ports/macro-body-notes.tsv; the arm's text is in the reference file.
- **`typedef uintptr_t longword`: `type longword = usize`.**  C's `uintptr_t` is the port's
  `usize`; the `_GL_ATTRIBUTE_MAY_ALIAS` on the same line is an attribute spelling with no
  Rust counterpart (the class `docs/ports/excluded-macros.txt` records for that family), and
  the alias stays in the function body where C declares it.
- **C 61's `static_assert` keeps its place in the body.**  Rust allows `const _: () = assert!
  (…)` in a block, so the assertion is not hoisted; the two configuration macros it compares
  are the table's own expressions (macros.tsv 1825, 1852), whose frozen values (`UCHAR_WIDTH`
  = `CHAR_BIT` = 8, `UINTPTR_WIDTH` = 64) the port spells as the constants it compares, the
  way a mirror records a configuration constant (PORTING.md 0.1).
- **`sizeof`/`alignof`** are the token-aliases families (docs/ports/token-aliases.tsv), so
  the four `sizeof (longword)` uses are `size_of::<longword> ()`.
- **The byte loops.**  C's `for (…; …; ++char_ptr)` and `while (*char_ptr != c) char_ptr++;`
  become the port's `while` loops with the pointer step in the body, since Rust has no
  for-increment statement; the statements and their order are C's.
- **Call-relationship differences (D7)**: none.  The unit calls nothing.
