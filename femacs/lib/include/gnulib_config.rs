// femacs: the gnulib configuration macros the port needs, in one place (PORTING.md 0.1-5).
//
// These are not in any reference header: the reference gets them from the configure-
// generated config header (gnulib's `lib/config.h`, emitted from m4/gnulib-common.m4's
// AH_VERBATIM blocks; the reference reaches it through src/config.h).  The port's
// `src/include/config.rs` is generated from the curated value list in
// docs/baseline/config/*.json and holds value constants only, so the *macro-shaped* ones
// live here together with the reason each is needed.
//
// So far that is `_GL_CMP`, used by lib/timespec.h.

/// gnulib's `_GL_CMP (n1, n2)`: `(((n1) > (n2)) - ((n1) < (n2)))` — the branch-free
/// comparison of Hacker's Delight § 2-9 that gnulib prefers to
/// `(n1 > n2 ? 1 : n1 < n2 ? -1 : 0)` (m4/gnulib-common.m4 4746-4754).  It is a value
/// expression whose arguments are evaluated once each, so the port keeps the macro shape
/// (P-37) and C's body, with C's bool→int conversion written out.
macro_rules! _GL_CMP {
    ($n1:expr, $n2:expr) => {
        (($n1) > ($n2)) as c_int - (($n1) < ($n2)) as c_int
    };
}

/// gnulib's `_GL_EXTERN_INLINE` (m4/gnulib-common.m4's AH_VERBATIM; the frozen configuration's
/// value is the two keywords `extern inline`, docs/baseline/config/darwin-arm64.macros.tsv
/// 2104).  In the C99 inline model a header carries `_GL_INLINE`-qualified definitions and one
/// `.c` per header does `#define _GL_<X>_INLINE _GL_EXTERN_INLINE` before including it, so that
/// the translation unit emits the single external definition (PORTING.md 10.6).  The port's
/// functions are ordinary Rust items with one definition each, so the qualifier has no
/// counterpart; the name is a marker because the wrapper `.c` files define their own
/// `_GL_*_INLINE` in terms of it and that relationship is what the macro ledger compares.
pub const _GL_EXTERN_INLINE: () = ();

/// gnulib's `_GL_INLINE` (m4/gnulib-common.m4's AH_VERBATIM: `static inline` or `__inline`).
/// As with `_GL_EXTERN_INLINE`, the port's functions are ordinary Rust items, so the name is a
/// marker for the mirrors whose own `*_INLINE` qualifier is defined in terms of it.
pub const _GL_INLINE: () = ();
