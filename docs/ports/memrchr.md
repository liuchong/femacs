# emacs/lib/memrchr.c (158 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __memrchr  (line 46)

## Macros defined — 2
- [x] __memrchr  (line 41, body dbf28286890c)
- [x] reg_char  (line 29, body 71fafc4e2fc1)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **`#if defined _LIBC` picks the `#else` arm.**  The frozen configuration does not define
  `_LIBC`, so C 27-30's live text is `#include <config.h>` and `#define reg_char char`; the
  `#include <memcopy.h>` arm is dead text in the reference file (PORTING.md 0.1-1).
- **The function's C name and the symbol it emits differ.**  C 40-42's `#ifndef weak_alias` /
  `#define __memrchr memrchr` is live (the configuration has no `weak_alias`), so the
  definition C 45-46 writes as `__memrchr` emits the symbol `memrchr` — the one gnulib's
  <string.h> declares for this configuration (emacs/lib/string.in.h 516).  The port keeps C's
  name in `fn __memrchr` (that is what this ledger compares) and emits the real symbol with
  `#[unsafe(export_name = "memrchr")]` (the mempcpy precedent, PORTING.md 10.13; the macro it
  expands is recorded in docs/ports/excluded-macros.txt).
- **`reg_char`**: C 29's `#define reg_char char` has no macro in the port; the alias
  `type reg_char = c_uchar;` carries the name the body reads, and the row is in
  docs/ports/excluded-macros.txt (the macro's own body cannot be compared).
- **`typedef unsigned long int longword`**: the port keeps the alias in the body, with
  `c_ulong`, and its `_GL_ATTRIBUTE_MAY_ALIAS` is the attribute spelling with no counterpart
  (the rawmemchr.c decision, same class).
- **C 81's `0xffffffffU`** is written `0xffffffffu32`: `0xffffffffu` is not a Rust literal,
  so the token cannot be spelled identically; the row is in
  docs/ports/macro-body-notes.tsv.
- **The unaligned-head loop and the tail loop keep C's step order.**  `for (…; n > 0 && …;
  --n)` decrements `n` *after* the body while `*--char_ptr` decrements the pointer *before*
  the compare, so the port's `while` performs `char_ptr = char_ptr.sub (1)` first and
  `n -= 1` last; `while (n-- > 0)` becomes `while n > 0 { n -= 1; … }`, which is C's
  test-then-decrement order.
- **Call-relationship differences (D7)**: none.  The unit calls nothing.

## 2026-10-05: C's wrap-around must be spelled out - the first `message` panicked (status 3.617)

`memrchr`'s haszero trick subtracts `repeated_one` from `longword1` and *relies* on the wrap-around:
in C the subtraction is unsigned and defined to wrap.  The port wrote C's expression verbatim, and
Rust's `-` panics on underflow in a debug build - which is how the unit is compiled - so the first
`(message ...)` (through `message_dolog -> scan_newline -> find_newline -> memrchr`) aborted with
`attempt to subtract with overflow` instead of searching.  Now `longword1.wrapping_sub (repeated_one)`,
per PORTING.md 1.1 ("凡 C 里依赖二进制补码回绕的表达式，必须显式写 wrapping_*").
