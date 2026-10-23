// femacs: port of lib/execinfo.in.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1) of a gnulib *template* whose generated header does not exist on
// this target: `build/emacs-ref/lib/execinfo.h` is absent, because gnulib only writes it where
// the system has no `<execinfo.h>`, and the frozen macro table records `HAVE_EXECINFO_H` 1
// (`docs/baseline/config/darwin-arm64.macros.tsv`).  So there is no judge to read here - the
// position `lib/assert.in.h` is in (status 3.157, its generated header is absent too) - and the
// mirror carries the template's own text: the guard, the one inline qualifier and the three
// fallback definitions.
//
// **What the fallback is for.**  C 33-60 defines `backtrace`, `backtrace_symbols` and
// `backtrace_symbols_fd` as no-ops, so a program can call them on a system that lacks the header:
// `backtrace` returns 0 ("no frames"), `backtrace_symbols` a null pointer, and
// `backtrace_symbols_fd` does nothing.  On this target the *system's* `<execinfo.h>` is what the C
// units use, and `src/sysdep.c` 2311-2331 calls it - the port declares the two entries it needs at
// that use site (`src/sysdep.rs` 563-565, `<execinfo.h>`'s own readings), which is why this
// mirror's definitions are a record of the *fallback* rather than the port's live spellings of
// those names.  Nothing includes this mirror either: its only C consumers are the gnulib units that
// replace the system header, and the frozen configuration replaces nothing.
//
// **Conditions.**  `tools/check-cond-eval.py lib/execinfo.in.h` -> `5 conditions, 0 unknown to the
// evaluator, 2 disagreements`: the file is not on docs/baseline/config/cond-eval-verified.txt, so
// the extractor resolves nothing in it (P-66's conservative reading).  The two `__cplusplus` arms
// (C 33-35, C 63-65) are C++ linkage text with no Rust counterpart; the `_GL_CONFIG_H_INCLUDED`
// check (C 24-26) is C's include-order guard, which the port cannot get wrong.
//
// `_GL_INLINE` is gnulib's `static inline` spelling and C 29-31 points `_GL_EXECINFO_INLINE` at it:
// the port writes `#[inline(always)] pub fn` (the `_GL_TIMESPEC_INLINE` row of
// docs/ports/excluded-macros.txt, whose kind=syntax says exactly this), so the qualifier macro has
// no Rust item - see docs/ports/include-execinfo.in.md for the row this file needs.

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const _GL_EXECINFO_H: bool = true;

/// C 38-44: `_GL_EXECINFO_INLINE int backtrace (void **buffer, int size)` - the fallback returns
/// 0 and touches nothing; C's `(void) buffer;` casts have no Rust spelling (the `include/msdos.rs`
/// note), so the two pointers are consumed by `let _` and C's `return 0;` is the port's.
/// C: _GL_EXECINFO_INLINE int
/// C: backtrace (void **buffer, int size)
/// C: {
/// C:   (void) buffer;
/// C:   (void) size;
/// C:   return 0;
/// C: }
#[inline(always)]
pub fn backtrace (buffer: *mut *mut c_void, size: c_int) -> c_int
{
  let _ = buffer; let _ = size;
  return 0;
}

/// C 46-52: `_GL_EXECINFO_INLINE char ** backtrace_symbols (void *const *buffer, int size)` - the
/// fallback returns a null pointer.  C spells it `return 0;`, which Rust cannot write for a
/// pointer type, so the return keeps C's literal through the integer-to-pointer cast Rust
/// requires for it (the house alternative is a `macro-body-notes.tsv` row for the literal, which
/// this lane may not write - recorded in the ledger instead).
/// C: _GL_EXECINFO_INLINE char **
/// C: backtrace_symbols (void *const *buffer, int size)
/// C: {
/// C:   (void) buffer;
/// C:   (void) size;
/// C:   return 0;
/// C: }
#[inline(always)]
pub fn backtrace_symbols (buffer: *const *mut c_void, size: c_int) -> *mut *mut c_char
{
  let _ = buffer; let _ = size;
  return 0 as *mut *mut c_char;
}

/// C 54-60: `_GL_EXECINFO_INLINE void backtrace_symbols_fd (void *const *buffer, int size, int
/// fd)` - the fallback does nothing at all.
/// C: _GL_EXECINFO_INLINE void
/// C: backtrace_symbols_fd (void *const *buffer, int size, int fd)
/// C: {
/// C:   (void) buffer;
/// C:   (void) size;
/// C:   (void) fd;
/// C: }
#[inline(always)]
pub fn backtrace_symbols_fd (buffer: *const *mut c_void, size: c_int, fd: c_int)
{
  let _ = buffer; let _ = size; let _ = fd;
}
