# emacs/lib/tempname.c (284 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 9
- [x] mix_random_values  (line 74, static)
- [x] random_bits  (line 93, static)
- [x] try_file  (line 128, static)
- [x] try_dir  (line 137, static)
- [x] try_nocreate  (line 143, static)
- [x] gen_tempname_len  (line 174)
- [x] try_tempname_len  (line 191)
- [x] __gen_tempname  (line 272)
- [x] try_tempname  (line 279)

## Macros defined — 16
- [x] ATTEMPTS_MIN  (line 203, body 9aaa5d5d4b57)
- [x] BASE_62_DIGITS  (line 66, body b1d5781111d8)
- [x] BASE_62_POWER  (line 67, body e1e4efa3da44)
- [x] RANDOM_VALUE_MAX  (line 65, body 8925ab4ef859)
- [x] TMP_MAX  (line 27, body 1e3a5c429ba6)
- [x] __GT_DIR  (line 31, body 356a192b7913)
- [x] __GT_FILE  (line 30, body b6589fc6ab0d)
- [x] __GT_NOCREATE  (line 32, body da4b9237bacc)
- [x] __clock_gettime64  (line 57, body 2c33839e1c3c)
- [x] __gen_tempname  (line 52, body 131bc2b1ae9e)
- [x] __getrandom  (line 56, body 346ab8f68caf)
- [x] __lstat64_time64  (line 55, body b0743d78f427)
- [x] __mkdir  (line 53, body 90868ffdfa3a)
- [x] __open  (line 54, body 5fc7e38bffe0)
- [x] __timespec64  (line 58, body be884e02e476)
- [x] struct_stat64  (line 51, body 5a3ce7c5e736)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/tempname.c` @ `fd7afced403`; port `lib/tempname.rs` (405 lines).  All checklist
items ticked; `build-tu lib/tempname.c` -> `built lib/tempname.rs -> ../build/femacs/tempname.o`
(0 errors), `--check` -> `OK`.

- **Blind file.**  `lib/tempname.c` is **not** in `docs/baseline/config/cond-eval-verified.txt`,
  so the gate reads *both* arms of every `#if` as live text (P-66) and the checklist carries
  both.  The frozen build does compile it: `build/emacs-ref/lib/tempname.o` defines
  `_gen_tempname`, `_gen_tempname_len`, `_try_tempname`, `_try_tempname_len` (the same four the
  port's object defines) and calls `_clock_gettime`, `_getrandom`, `_mkdir`, `_open`,
  `_rpl_lstat`, `_strlen`, `_strspn`, which pins the `#else` arm of C 48-59 as the frozen one.
- **C 48-59's `#if _LIBC` / `#else`.**  The port keeps the `#else` arm's seven aliases as the
  items C's preprocessor produces: `struct_stat64` and `__timespec64` are type macros (the
  strtoimax.rs `Int!` shape), `__mkdir`/`__open`/`__lstat64_time64`/`__getrandom`/
  `__clock_gettime64` are `macro_rules!` twins with C's call shape, and `gen_tempname_len`'s
  `__lstat64_time64` reaches the native `lstat` through `mod system` +
  `#[link_name = "rpl_lstat"]` (gnulib renames it; `nm build/emacs-ref/lib/tempname.o` shows
  `_rpl_lstat`, the utimens.rs shape).  The `_LIBC` arm's two spellings (`struct __stat64_t64`,
  and C 122-125's `#if _LIBC` forward declaration) are named at their sites - the gate compares
  `struct_stat64`'s body only once and reports "several definitions (configuration branches)".
- **`__gen_tempname` is both a macro and a spelling.**  C 52's `# define __gen_tempname
  gen_tempname` rewrites C 271's definition, which is why the object exports `_gen_tempname`;
  the port keeps a `macro_rules! __gen_tempname` (expanding to the *name*) beside the function,
  which keeps C's source spelling and emits the real symbol with
  `#[unsafe(export_name = "gen_tempname")]` (PORTING.md 10.13, the mempcpy precedent).
- **C 109-116's `random_bits` arm.**  `CLOCK_REALTIME` is defined and `HAVE_CLOCK_GETTIME` is 1,
  so C 110-113 (`struct __timespec64 tv; __clock_gettime64 (CLOCK_REALTIME, &tv);` twice through
  `mix_random_values`) is the arm the build uses; C 115's `v = mix_random_values (v, clock ());`
  is the non-frozen side and is kept as `#[cfg (feature = "no_clock_gettime")]` text (P-51),
  with `clock`/`clock_gettime`/`CLOCK_REALTIME` declared in the unit (the gettime.rs precedent
  for the same two <time.h> entries).
- **Two Rust-only spellings, both recorded here.**  `vdigbuf` starts at C's zero for the type
  because Rust needs a value before the binding can be read (PORTING.md 0.1-13), and C's C99
  designated initializers for `tryfunc` become a const block that fills the three `__GT_*`
  slots by index (Rust has no designated initializers).
- **Mirrors.**  `include/tempname.rs` is new and complete
  (`tools/extract-abi.py --check emacs/lib/tempname.h lib/include/tempname.rs` -> `OK`).
  Two shared mirrors gained one item each, both after a tree-wide grep for same-named local
  declarations (status 3.24) and both verified by rebuilding their other consumers:
  `include/string.rs` + `strspn`, `include/stdint.in.rs` + `uint_fast64_t`/`UINT_FAST64_MAX`.
- Line drift: `line-drift-tolerance.tsv` records `tempname 0.45` (C 284 vs 405, 42.6%).
- Not verified: gate-fast/gate-probes, ERT, and any behavioural test of the generator.
