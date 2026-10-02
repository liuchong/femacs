// femacs: the gnulib configuration macros the port needs, in one place (PORTING.md 0.1-5).
//
// These are not in any reference header: the reference gets them from the configure-
// generated config header (gnulib's `lib/config.h`, emitted from m4/gnulib-common.m4's
// AH_VERBATIM blocks; Emacs reaches it through src/config.h).  The port's
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
