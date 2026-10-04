# emacs/lib/limits.in.h (151 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 28
- [x] BOOL_MAX  (line 133, body 356a192b7913)
- [x] BOOL_WIDTH  (line 130, body 356a192b7913)
- [x] CHAR_WIDTH  (line 112, body c91c491e54d6)
- [x] INT_WIDTH  (line 117, body df6cb1beed2c)
- [x] LLONG_MAX  (line 62, body 1a916b81d003)
- [x] LLONG_MIN  (line 53, body 85c2c80cd328)
- [x] LLONG_WIDTH  (line 121, body 7b8b518ebea4)
- [x] LONG_BIT  (line 98, body c66c65175fec)
- [x] LONG_WIDTH  (line 119, body 915e12b91f4e)
- [x] MB_LEN_MAX  (line 104, body 1574bddb75c7)
- [x] SCHAR_WIDTH  (line 113, body bd79bc2c3a55)
- [x] SHRT_WIDTH  (line 115, body f135b7f02e36)
- [x] SSIZE_MAX  (line 145, body f5c01bcbc752)
- [x] UCHAR_WIDTH  (line 114, body b2bac1efaa85)
- [x] UINT_WIDTH  (line 118, body 6d5d59c996c8)
- [x] ULLONG_MAX  (line 71, body 01d100f1e347)
- [x] ULLONG_WIDTH  (line 122, body ad3b3ae9220e)
- [x] ULONG_WIDTH  (line 120, body 0e0b44b7ebd5)
- [x] USHRT_WIDTH  (line 116, body ce2f65e126aa)
- [x] WORD_BIT  (line 91, body cb4e5208b4cd)
- [x] _  (line 45, body cc2c62e673e9)
- [x] _GL_COB128  (line 82, body e46b9f95e123)
- [x] _GL_COB16  (line 85, body e769a204b41a)
- [x] _GL_COB32  (line 84, body 5a2f3796fcdb)
- [x] _GL_COB4  (line 87, body 795c8c573e42)
- [x] _GL_COB64  (line 83, body 9aeac64b1c0e)
- [x] _GL_COB8  (line 86, body 854f7f3b7ba8)
- [x] _GL_INTEGER_WIDTH  (line 81, body 261b6c61c5f3)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/limits.in.h` @ `fd7afced403`; port `femacs/lib/include/limits.in.rs`.

**The template.**  The reference is a gnulib template: `@GUARD_PREFIX@`,
`@PRAGMA_SYSTEM_HEADER@`, `@PRAGMA_COLUMNS@`, `@INCLUDE_NEXT@` and `@NEXT_LIMITS_H@` are
configured in.  The frozen substitution is the generated `build/emacs-ref/lib/limits.h`
(guard `_GL_LIMITS_H`, `#include_next <limits.h>`), and the port has no `#include_next`, so
the mirror is both halves: the system `<limits.h>` values C 40 pulls in (measured with the
frozen target's clang) and the names C 44-148 defines itself.

**Every `#ifndef` in the template is false on the frozen target.**  darwin's `<limits.h>`
already defines `LLONG_MIN`/`LLONG_MAX`/`ULLONG_MAX`, `WORD_BIT`, `LONG_BIT`, `MB_LEN_MAX`,
`SSIZE_MAX` and the whole C23 width family, so the template's own definitions of them are
dead text.  `extract-abi.py` reads a header's `#if`s **blind** unless the header is in
`docs/baseline/config/cond-eval-verified.txt`, and this one is not (P-08/P-66), so the mirror
carries every name the template can define, with the value the frozen preprocessor has:

- `WORD_BIT` 32, `LONG_BIT` 64, the eleven widths (8/8/8/16/16/32/32/64/64/64/64),
  `BOOL_WIDTH`/`BOOL_MAX` 1, `SSIZE_MAX` `LONG_MAX` - each agrees with the template's text.
- `BOOL_MAX` is the one name the template *does* define for this configuration: the system
  header defines `BOOL_WIDTH` but not `BOOL_MAX`, so C 133-134's `#elif` arm is live.
- `MB_LEN_MAX` is the one name whose frozen value differs from the template's text (6 vs 16,
  see below), and it is the mirror's single unchecked box.

The width constants keep C's expansion shape: `_GL_INTEGER_WIDTH (MIN, MAX)`, with the signed
minimum and the unsigned maximum of the type (`ULLONG_MAX` arrives as `-1`'s bit pattern, and
`_GL_COB*` shifts the word logically, which is what C's preprocessor does with an unsigned
maximum).  The port's own probe asserts all eleven widths and every limit against the numbers
the frozen target's clang prints, so the values are measured rather than derived.

**`MB_LEN_MAX`** (C 103-106): the template says 16 under `#ifndef MB_LEN_MAX`, and the system
header the template includes next defines it as **6**, so 16 never appears in the frozen
build.  6 is also what the ported units need: `lib/regex_internal.rs` 182-185 and
`lib/regcomp.rs` 259 size multibyte buffers with it and `DEBUG_ASSERT (MB_LEN_MAX >=
pstr->mb_cur_max)` must check what the reference checks.  The mirror therefore spells C's
*shape* (`macro_rules! MB_LEN_MAX`) with the frozen value, and the pair needs one row in
`docs/ports/excluded-macros.txt`, which this lane must not write (the integrator owns that
file):

```
MB_LEN_MAX	dead-text	limits.in.h 103-106's `#ifndef MB_LEN_MAX / # define MB_LEN_MAX 16` is dead on the frozen target: the system `<limits.h>` the template includes next (its C 40) already defines `MB_LEN_MAX` as 6, measured with the frozen compiler, and the port keeps that value because the ported units size multibyte buffers with it (`lib/regex_internal.rs` 182-185, `lib/regcomp.rs` 259) and `DEBUG_ASSERT (MB_LEN_MAX >= pstr->mb_cur_max)` must check what the reference checks; the mirror is `femacs/lib/include/limits.in.rs` and docs/ports/include-limits.in.md records the measurement
```

With that row in place the pair is `OK` (observed before the row was reverted for the
integrator to own it); without it `--check` reports exactly two lines, both about the
template's 16 (`macro MB_LEN_MAX does not mention 16`, `missing constant MB_LEN_MAX`).  A
`pub const MB_LEN_MAX: c_int = 6;` **cannot** be made to pass: the constants rule compares a
defined constant's value against the template's literal and never consults the exclusion list,
so only the macro shape (a name that is *not* a `const`) can be excluded.  That is why the
mirror uses `MB_LEN_MAX! ()` at the three call sites that read it.

**Convergence** (PORTING.md 0.1-3/0.1-8, the same batch): the names this mirror adds had five
use-site copies, all deleted here with the consumer left reading the mirror -

| unit | copy | now |
| --- | --- | --- |
| `src/indent.rs` 141 | `pub const SHRT_WIDTH` (a "mirror gap", P-26) | `SHRT_WIDTH` from the mirror |
| `src/fileio.rs` 2975 | `pub const SSIZE_MAX: c_long` | mirror |
| `lib/getdelim.rs` 47 | `pub const SSIZE_MAX: isize` | mirror |
| `lib/careadlinkat.rs` 27 | `pub const SSIZE_MAX: c_long` | mirror |
| `lib/include/regex_internal.rs` 147 | `pub const SSIZE_MAX: isize` | mirror; `IDX_MAX` casts it to `Idx` (C's `SSIZE_MAX` is a `long`, the mirror's `Idx` is `regoff_t` = `isize`) |
| `lib/rawmemchr.rs` 45 | a function-local `const UCHAR_WIDTH: usize = 8` | mirror (`as usize` in the assertion) - flagged by `check-mirror-dupes.py`, which sees function-local constants |
| `lib-src/make-fingerprint.rs` 62 | `const SSIZE_MAX: isize` (its ledger is finished, so the copy would have broken that unit's build) | mirror |

`MB_LEN_MAX`'s three call sites (`lib/regex_internal.rs` 185/271, in `#[cfg (feature =
"libc")]` arms, and `lib/regcomp.rs` 259/285/359) now spell `MB_LEN_MAX! ()`.

**Not edited** (another lane's files, reported instead of patched):

- `femacs/lib-src/etags.rs` 72 declares `pub const CHAR_MAX: c_int = 127`, and this mirror
  now carries `CHAR_MAX`; its unit includes `limits.in.rs` (C 109), so that copy is an
  `E0428` waiting for its build.  The ledger (`docs/ports/etags.md`) still has 141 unchecked
  boxes, so no gate compiles the unit yet.  Delete the copy, or keep `CHAR_MAX` in the mirror
  and read it from here.
- `femacs/src/sysdep.rs` 3911's `SSIZE_MAX` is behind `#[cfg (feature = "no_ssize_max")]`,
  which the frozen configuration does not set (its own comment says C's `#ifndef SSIZE_MAX` is
  dead on this target), so it stays as the carried dead arm and cannot collide.

**The `_` box** is C's guard read through the template's `@GUARD_PREFIX@` placeholder
(`_@GUARD_PREFIX@_LIMITS_H` is not an identifier, so the extractor sees `_`); the mirror
writes the frozen substitution `_GL_LIMITS_H`, and `_` has a tree-wide
`docs/ports/excluded-macros.txt` row already.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/limits.in.h femacs/lib/include/limits.in.rs
  (2 C macros have a recorded Rust counterpart: MB_LEN_MAX, _)      # while the row above was in place
  (6 C macros have several definitions (configuration branches), body not compared:
   BOOL_MAX, LLONG_MAX, LLONG_MIN, LONG_BIT, SSIZE_MAX, ULLONG_MAX)
OK emacs/lib/limits.in.h -> femacs/lib/include/limits.in.rs
# after reverting that row for the integrator to own it, the same command reports exactly:
#   - macro MB_LEN_MAX does not mention 16 (C body: 16)
#   - missing constant MB_LEN_MAX
#   FAIL emacs/lib/limits.in.h -> femacs/lib/include/limits.in.rs

$ cd /tmp/femacs-probe && rustc --edition 2024 --crate-type=lib --emit=obj \
    -C panic=abort limits_probe.rs -o /tmp/femacs-probe/limits_probe.o
exit=0        # the probe includes the mirror and asserts the value of every name it defines:
              # the eleven C23 widths (8/8/8/16/16/32/32/64/64/64/64), CHAR_BIT 8, the six
              # signed minima, UCHAR_MAX/USHRT_MAX/UINT_MAX/ULONG_MAX, LONG_BIT 64,
              # WORD_BIT 32, BOOL_WIDTH/BOOL_MAX 1, MB_LEN_MAX! () == 6, SSIZE_MAX ==
              # c_long::MAX, PATH_MAX 1024 and the core::ffi long-long limits.
              # (The probe asserted INTMAX_WIDTH/UINTMAX_WIDTH 64 while this mirror carried
              # them; the stdint round moved the pair to `stdint.in.rs`, where C defines them
              # at lib/stdint.in.h 725-726, so the assertion lives in that mirror's probe.)

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-limits \
    cargo run -q -p xtask -- build-tu <unit.c>          # each printed "built <unit.rs>", exit 0
lib/openat-proc.c      (the integrator's consumer; `lib/openat-proc.rs` includes limits.in.rs)
lib/mktime.c           (the task's named consumer)
lib/rawmemchr.c  lib/regex_internal.c  lib/regcomp.c  lib/getdelim.c  lib/careadlinkat.c
src/fileio.c  src/indent.c  lib-src/make-fingerprint.c

# the edited pairs keep their inventory verdicts:
$ python3 tools/extract-abi.py --check emacs/<c> femacs/<rs>
OK emacs/lib/regex_internal.c -> femacs/lib/regex_internal.rs
OK emacs/lib/regex_internal.h -> femacs/lib/include/regex_internal.rs
OK emacs/lib/regcomp.c -> femacs/lib/regcomp.rs
OK emacs/lib/rawmemchr.c -> femacs/lib/rawmemchr.rs
OK emacs/lib/getdelim.c -> femacs/lib/getdelim.rs
OK emacs/lib/careadlinkat.c -> femacs/lib/careadlinkat.rs
OK emacs/lib-src/make-fingerprint.c -> femacs/lib-src/make-fingerprint.rs
OK emacs/src/indent.c -> femacs/src/indent.rs
OK emacs/src/fileio.c -> femacs/src/fileio.rs

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
```

`femacs/lib/ftoastr.rs` 26's comment still says "`MB_LEN_MAX` is in `include/limits.in.rs`"
(true) and `docs/ports/make-fingerprint.md`'s prose about the local `SSIZE_MAX` is now stale -
both are the integrator's to refresh if the ledger text matters.

**Measured, not predicted** (the row above is the integrator's to add, so it was proved with a
copy): `extract-abi.py` takes `--excluded`, so the row was appended to a copy in `/tmp` and

```
$ python3 tools/extract-abi.py --check --excluded /tmp/mem-excl.txt \
    emacs/lib/limits.in.h femacs/lib/include/limits.in.rs
OK emacs/lib/limits.in.h -> femacs/lib/include/limits.in.rs
```

so the mirror plus that one row is the complete pair.  Nothing in the shared file was written.
