// femacs: port of lib/timespec.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): constants and the header's `_GL_TIMESPEC_INLINE`
// functions, plus the declarations C spells `extern`.  It is a *reference* file
// (gnulib's timespec module), so `tools/extract-abi.py --check lib/timespec.h
// femacs/lib/include/timespec.rs` compares it to the C header.
//
// It includes `lib/include/time.in.rs` for `time_t`/`struct timespec`, mirroring C's
// `#include <time.h>`; `buffer.rs` reaches the same mirror directly, so a unit that includes
// both (`data.c` will) hits the same file twice — `xtask`'s include-graph expansion makes
// that one copy (PORTING.md 0.1-4/P-61), which is what lets a mirror keep its own includes
// instead of asking every translation unit to list the closure.
//
// C's `arg-nonnull.h` include is dropped: the only thing timespec.h takes from it is the
// `_GL_ARG_NONNULL` attribute, which is recorded in docs/ports/excluded-macros.txt
// (attributes have no Rust counterpart; PORTING.md 0.3).  `_GL_INLINE_HEADER_BEGIN`/`END`,
// `_GL_INLINE` and `_GL_TIMESPEC_INLINE` are attribute/spelling macros with the same
// disposition; `_GL_CMP` is a real value macro and lives in gnulib_config.rs.

include!("time.in.rs");        // #include <time.h> (`time_t`, `struct timespec`)
include!("gnulib_config.rs");

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const TIMESPEC_H: bool = true;

// C's `#if !_GL_CONFIG_H_INCLUDED` / `#error "Please include config.h first."` / `#endif` block:
// the generated config header sets it (src/config.h 38 of the frozen build), and the
// port's config.rs — generated from docs/baseline/config/*.json — carries it.
const _: () = core::assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

/* Inverse resolution of timespec timestamps (in units per second),
   and log base 10 of the inverse resolution.  */

pub const TIMESPEC_HZ: c_int = 1000000000;
pub const LOG10_TIMESPEC_HZ: c_int = 9;

/* Obsolescent names for backward compatibility.
   They are misnomers, because TIMESPEC_RESOLUTION is not a resolution.  */

pub const TIMESPEC_RESOLUTION: c_int = TIMESPEC_HZ;
pub const LOG10_TIMESPEC_RESOLUTION: c_int = LOG10_TIMESPEC_HZ;

/* Return a timespec with seconds S and nanoseconds NS.  */

#[inline(always)]
pub fn make_timespec (s: time_t, ns: c_long) -> timespec
{
  timespec { tv_sec: s, tv_nsec: ns }
}

/* Return negative, zero, positive if A < B, A == B, A > B, respectively.  */

#[inline(always)]
pub fn timespec_cmp (a: timespec, b: timespec) -> c_int
{
  /* C: 2 * _GL_CMP (a.tv_sec, b.tv_sec) + _GL_CMP (a.tv_nsec, b.tv_nsec) — an integer
     expression, so C's `2 *` is Rust's `2 *`.  */
  2 * _GL_CMP! (a.tv_sec, b.tv_sec) + _GL_CMP! (a.tv_nsec, b.tv_nsec)
}

/* Return -1, 0, 1, depending on the sign of A.  A.tv_nsec must be
   nonnegative.  */

#[inline(always)]
pub fn timespec_sign (a: timespec) -> c_int
{
  _GL_CMP! (a.tv_sec | a.tv_nsec, 0)
}

/* Return an approximation to A, of type 'double'.  */

#[inline(always)]
pub fn timespectod (a: timespec) -> c_double
{
  a.tv_sec as c_double + a.tv_nsec as c_double / 1e9
}

/* The seven declarations in a whole-import module: `femacs/lib/gettime.rs` and
   `femacs/lib/timespec.rs` define `gettime`/`current_timespec` themselves, and a definition
   shadows a glob import instead of colliding with it (PORTING.md 0.1-10).  */
pub mod timespec_decls {
  use super::*;

  unsafe extern "C" {
    // Parameter names come from the definitions in lib/timespec.c (C's prototypes have none).
    pub fn timespec_add (a: timespec, b: timespec) -> timespec;
    pub fn timespec_sub (a: timespec, b: timespec) -> timespec;
    pub fn dtotimespec (sec: c_double) -> timespec;
    pub fn gettime_res () -> c_long;
    pub fn current_timespec () -> timespec;
    pub fn gettime (ts: *mut timespec);
    pub fn settime (ts: *const timespec) -> c_int;
  }
}
use timespec_decls::*;
