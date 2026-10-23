// femacs: mirror of lib/unlocked-io.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1, P-26).  The header is one `#if` pair per stdio entry: an
// entry whose host has the `*_unlocked` form is redefined to call it, and one whose host does
// not gets the `*_unlocked` name defined to call the ordinary entry, plus the three
// unconditional `flockfile`/`ftrylockfile`/`funlockfile` no-ops.
//
// **Both arms are in the mirror, and the criterion for the live one is measured, not
// assumed.**  `lib/unlocked-io.h` is *not* in docs/baseline/config/cond-eval-verified.txt -
// `python3 tools/check-cond-eval.py lib/unlocked-io.h` prints `16 conditions, 0 unknown,
// 9 disagreements [NOT VERIFIED]`, because macOS's `<stdio.h>` defines `clearerr_unlocked` and
// its siblings as macros, which is the second disjunct of each condition and which the
// configuration's own table cannot see.  So `tools/extract-abi.py` reads the file blind and
// requires all 32 names, and the live arm per name was measured with the frozen clang:
//
//     $ clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src \
//         -I emacs/lib -include config.h -include unlocked-io.h -x c /dev/null \
//         | grep -E '#define (UNLOCKED_IO_H|clearerr|feof|…|putchar_unlocked)\b'
//
// The surviving definitions are: `UNLOCKED_IO_H 1`; `clearerr(x) clearerr_unlocked (x)`,
// `feof`, `ferror`, `fileno`, `getc`, `getchar`, `putc`, `putchar` (the first arm, whose
// `HAVE_DECL_*_UNLOCKED` is 1 *and* whose `*_unlocked` name the system header defines as a
// macro - the surviving `clearerr_unlocked(p) __sclearerr(p)` is the system header's);
// `fflush_unlocked(x) fflush (x)`, `fgets_unlocked`, `fputc_unlocked`, `fputs_unlocked`,
// `fread_unlocked`, `fwrite_unlocked` (the `#else` arm, whose `HAVE_DECL_*_UNLOCKED` is 0 and
// whose `*_unlocked` name has no system macro); and the three no-ops
// `flockfile(x) ((void) 0)`, `ftrylockfile(x) 0`, `funlockfile(x) ((void) 0)`.
//
// Each gated arm below is the *other* arm of one `#if`, and its feature names the condition
// that selects it, so the frozen configuration enables none of them (P-51):
//
//   * `no_decl_<x>_unlocked` - the eight `<x>_unlocked` definitions (C 45, 52, 59, 80, 115,
//     122, 129, 136), live only where `HAVE_DECL_<X>_UNLOCKED` is 0 *and* the system header
//     has no `<x>_unlocked` macro.
//   * `decl_<x>_unlocked` - the six ordinary-entry definitions (C 64, 71, 85, 92, 99, 106),
//     live only where `HAVE_DECL_<X>_UNLOCKED` is 1 or the system header has the macro.
//
// C's `# undef <name>` lines (C 42, 49, 56, 63, 70, 77, 84, 91, 98, 105, 112, 119, 126, 133,
// 139, 142, 145) have no Rust counterpart: the port has no preprocessor to undefine anything
// from (PORTING.md 0.1), and the macros above are the file's whole effect.  C 39's
// `#include <stdio.h>` is not repeated either - a unit names `include/stdio.rs` in its own
// include list (PORTING.md 0.1-4), where the ordinary `clearerr`/`fflush`/`getchar`/...
// declarations live, and pulling that mirror in would make this one depend on the includer's
// scalar scope.  This mirror spells the one scalar type it needs with an absolute
// `core::ffi::` path, so `build-tu lib/unlocked-io.h` compiles it as its own crate root.

/* C 20-21: `#ifndef UNLOCKED_IO_H / # define UNLOCKED_IO_H 1` - the guard has no Rust
   counterpart (PORTING.md 0.1-4); the name stays as its marker.  */
pub const UNLOCKED_IO_H: core::ffi::c_int = 1;

/* C 41-46: `clearerr`.  Live arm: the host has `clearerr_unlocked`.  */
macro_rules! clearerr { ($x:expr) => { clearerr_unlocked ($x) }; }
#[cfg (feature = "no_decl_clearerr_unlocked")]
macro_rules! clearerr_unlocked { ($x:expr) => { clearerr ($x) }; }

/* C 48-53: `feof`.  */
macro_rules! feof { ($x:expr) => { feof_unlocked ($x) }; }
#[cfg (feature = "no_decl_feof_unlocked")]
macro_rules! feof_unlocked { ($x:expr) => { feof ($x) }; }

/* C 55-60: `ferror`.  */
macro_rules! ferror { ($x:expr) => { ferror_unlocked ($x) }; }
#[cfg (feature = "no_decl_ferror_unlocked")]
macro_rules! ferror_unlocked { ($x:expr) => { ferror ($x) }; }

/* C 62-67: `fflush`.  Live arm: the `#else` side - the host has no `fflush_unlocked`, so the
   `*_unlocked` name is the one defined, and an ordinary `fflush` call is left alone.  */
#[cfg (feature = "decl_fflush_unlocked")]
macro_rules! fflush { ($x:expr) => { fflush_unlocked ($x) }; }
macro_rules! fflush_unlocked { ($x:expr) => { fflush ($x) }; }

/* C 69-74: `fgets`.  */
#[cfg (feature = "decl_fgets_unlocked")]
macro_rules! fgets { ($x:expr, $y:expr, $z:expr) => { fgets_unlocked ($x, $y, $z) }; }
macro_rules! fgets_unlocked { ($x:expr, $y:expr, $z:expr) => { fgets ($x, $y, $z) }; }

/* C 76-81: `fileno`.  */
macro_rules! fileno { ($x:expr) => { fileno_unlocked ($x) }; }
#[cfg (feature = "no_decl_fileno_unlocked")]
macro_rules! fileno_unlocked { ($x:expr) => { fileno ($x) }; }

/* C 83-88: `fputc`.  */
#[cfg (feature = "decl_fputc_unlocked")]
macro_rules! fputc { ($x:expr, $y:expr) => { fputc_unlocked ($x, $y) }; }
macro_rules! fputc_unlocked { ($x:expr, $y:expr) => { fputc ($x, $y) }; }

/* C 90-95: `fputs`.  */
#[cfg (feature = "decl_fputs_unlocked")]
macro_rules! fputs { ($x:expr, $y:expr) => { fputs_unlocked ($x, $y) }; }
macro_rules! fputs_unlocked { ($x:expr, $y:expr) => { fputs ($x, $y) }; }

/* C 97-102: `fread`.  */
#[cfg (feature = "decl_fread_unlocked")]
macro_rules! fread { ($w:expr, $x:expr, $y:expr, $z:expr) => { fread_unlocked ($w, $x, $y, $z) }; }
macro_rules! fread_unlocked { ($w:expr, $x:expr, $y:expr, $z:expr) => { fread ($w, $x, $y, $z) }; }

/* C 104-109: `fwrite`.  */
#[cfg (feature = "decl_fwrite_unlocked")]
macro_rules! fwrite { ($w:expr, $x:expr, $y:expr, $z:expr) => { fwrite_unlocked ($w, $x, $y, $z) }; }
macro_rules! fwrite_unlocked { ($w:expr, $x:expr, $y:expr, $z:expr) => { fwrite ($w, $x, $y, $z) }; }

/* C 111-116: `getc`.  */
macro_rules! getc { ($x:expr) => { getc_unlocked ($x) }; }
#[cfg (feature = "no_decl_getc_unlocked")]
macro_rules! getc_unlocked { ($x:expr) => { getc ($x) }; }

/* C 118-123: `getchar` (no argument).  */
macro_rules! getchar { () => { getchar_unlocked () }; }
#[cfg (feature = "no_decl_getchar_unlocked")]
macro_rules! getchar_unlocked { () => { getchar () }; }

/* C 125-130: `putc`.  */
macro_rules! putc { ($x:expr, $y:expr) => { putc_unlocked ($x, $y) }; }
#[cfg (feature = "no_decl_putc_unlocked")]
macro_rules! putc_unlocked { ($x:expr, $y:expr) => { putc ($x, $y) }; }

/* C 132-137: `putchar`.  */
macro_rules! putchar { ($x:expr) => { putchar_unlocked ($x) }; }
#[cfg (feature = "no_decl_putchar_unlocked")]
macro_rules! putchar_unlocked { ($x:expr) => { putchar ($x) }; }

/* C 139-146: the three unconditional no-ops.  C's bodies do not read their argument
   (`((void) 0)` and `0`), so the arms do not either - P-16's evaluation count, the `assert`
   shape `include/android-asset.rs` uses.  */
macro_rules! flockfile { ($x:expr) => { 0 }; }
macro_rules! ftrylockfile { ($x:expr) => { 0 }; }
macro_rules! funlockfile { ($x:expr) => { 0 }; }
