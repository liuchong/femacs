// femacs: mirror of lib/min-max.h @ fd7afced403 ; PORTING.md 0.1.
//
// The header is two macros and their two `#ifndef` guards, nothing else - no include guard, no
// types, no declarations.  `min-max.h` is not `lib/minmax.h` (which this tree also has, and
// which is mirrored separately as `minmax.rs` with the upper-case names): gnulib keeps both
// spellings, and the two macros below are the lower-case pair.
//
// C's `#ifndef min` / `#ifndef max` guards guard against a *system* definition of those names;
// this target's headers define neither, so both arms are live in the sense that matters - the
// port defines the macros and a unit that includes this mirror gets them exactly as C does.
// Like C's macros (and unlike `minmax.rs`'s `MIN`/`MAX`), each expands its arguments twice: the
// port keeps that, because these are value macros whose callers pass plain variables
// (`min (min (TYPE_MAXIMUM (off_t), PTRDIFF_MAX), min (SIZE_MAX, SSIZE_MAX))` in
// lib-src/make-fingerprint.c is the worked example).

/* C 2: `# define min(a,b) ((a) < (b) ? (a) : (b))`  */
macro_rules! min { ($a:expr, $b:expr) => { if ($a) < ($b) { $a } else { $b } }; }

/* C 5: `# define max(a,b) ((a) > (b) ? (a) : (b))`  */
macro_rules! max { ($a:expr, $b:expr) => { if ($a) > ($b) { $a } else { $b } }; }
