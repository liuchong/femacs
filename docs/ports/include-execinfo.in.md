# emacs/lib/execinfo.in.h (69 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] backtrace  (line 39)
- [x] backtrace_symbols  (line 47)
- [x] backtrace_symbols_fd  (line 55)

## Macros defined — 2
- [x] _GL_EXECINFO_H  (line 21, body da39a3ee5e6b)
- [ ] _GL_EXECINFO_INLINE  (line 30, body a960df3ab827)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **There is no judge for this template, and that is measured.**  gnulib only writes
  `lib/execinfo.h` where the system has no `<execinfo.h>`; the frozen macro table records
  `HAVE_EXECINFO_H 1` and `build/emacs-ref/lib/execinfo.h` **does not exist** (`ls`:
  "No such file or directory"), so the frozen build never uses this template and its position is
  `lib/assert.in.h`'s (status 3.157, its generated header is absent too): the mirror carries the
  template's own text.
- **The three functions are the fallback, not the port's live spellings.**  C 38-60 defines
  `backtrace` (returns 0), `backtrace_symbols` (returns a null pointer) and `backtrace_symbols_fd`
  (does nothing) as `_GL_EXECINFO_INLINE` definitions; on this target the *system's* `<execinfo.h>` is
  what the C units call, and `femacs/src/sysdep.rs` 563-565 declares the two entries `src/sysdep.c`
  2311-2331 uses at that use site.  Nothing `include!`s this mirror, so the two spellings coexist
  without a name clash; a future unit that wants the fallback would need to stop declaring the
  system's.
- **The bodies are C's, with the one unavoidable spelling change.**  C's `(void) buffer;` casts have
  no Rust counterpart, so each parameter is consumed by `let _ = buffer; let _ = size;` (the
  `include/msdos.rs` note on `(void)`); C 42/C 50's `return 0;` is the port's `return 0;` for the
  `int` case and `return 0 as *mut *mut c_char;` for C 51's pointer case - a Rust integer-to-pointer
  cast, which is the only way to keep the reference's own literal in a function whose result is a
  pointer (the house alternative is a `macro-body-notes.tsv` row for the `0`, which this lane may not
  write; it is listed in `## Evidence` for the integrator, and the spelling is chosen so that no row
  is needed).
- **`_GL_EXECINFO_INLINE` has no Rust item, and that is the one gap.**  C 29-31 points it at
  `_GL_INLINE`, gnulib's `static inline`: the port writes `#[inline(always)] pub fn` (the
  `_GL_TIMESPEC_INLINE` row of `docs/ports/excluded-macros.txt`, whose kind=syntax states exactly
  this), so nothing declares the qualifier macro - the row this file needs is in `## Evidence`.
- **Conditions.**  `tools/check-cond-eval.py lib/execinfo.in.h` -> `5 conditions, 0 unknown to the
  evaluator, 2 disagreements` (the file is not on the verified list): the two `__cplusplus` linkage
  arms (C 33-35, C 63-65) have no Rust counterpart, and C 24-26's `_GL_CONFIG_H_INCLUDED` check is
  C's include-order guard.

## Evidence

- `python3 tools/extract-abi.py --check emacs/lib/execinfo.in.h femacs/lib/include/execinfo.in.rs`
  -> **FAIL** with exactly one line and no others:
  ```
  - missing macro _GL_EXECINFO_INLINE
  ```
  The three function bodies are accepted (including `backtrace_symbols`'s `0 as *mut *mut c_char`
  spelling of C 51's `return 0;`), and so is the guard.
- **The row this file needs in `docs/ports/excluded-macros.txt`** (reported for the integrator):
  `_GL_EXECINFO_INLINE	syntax	execinfo.in.h 29-31: gnulib's `_GL_INLINE` (`static inline`); the port writes `#[inline(always)] pub fn`, so no macro exists to compare (the _GL_TIMESPEC_INLINE row)`
- `python3 tools/check-cond-eval.py lib/execinfo.in.h` -> `5 conditions, 0 unknown to the evaluator,
  2 disagreements`.
- `python3 tools/extract-abi.py emacs/lib/execinfo.in.h` -> this ledger: 0 subrs, **3 functions
  defined** (`backtrace`, `backtrace_symbols`, `backtrace_symbols_fd`), 2 macros, 0 types,
  0 file-scope objects.
- `ls build/emacs-ref/lib/execinfo.h` -> `No such file or directory` (the file note's judge reading);
  `grep -n 'HAVE_EXECINFO_H' docs/baseline/config/darwin-arm64.macros.tsv` -> `HAVE_EXECINFO_H	1`.
- **Compile evidence** is the lane-local probe of this batch: it `include!`s this mirror and calls all
  three fallbacks (`assert_eq! (backtrace (null_mut (), 0), 0)`,
  `assert! (backtrace_symbols (null (), 0).is_null ())`, `backtrace_symbols_fd (null (), 0, 2)`) ->
  `lib mirror probe ok (16 bytes gl_directory)`.  The probe was deleted afterwards.
