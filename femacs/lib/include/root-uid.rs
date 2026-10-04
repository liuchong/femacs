// femacs: mirror of lib/root-uid.h @ fd7afced403 (gnulib); PORTING.md 0.1.
//
// Header mirror (PORTING.md 0.1).  The header contributes exactly one macro.  C 20-21 is the
// include guard (carried below as a name; the guard itself is the include graph's job,
// PORTING.md 0.1-4), C 23 is a comment, and C 24-28 is the pair
// `#ifdef __TANDEM / # define ROOT_UID 65535 / #else / # define ROOT_UID 0 / #endif`.
//
// `__TANDEM` is the NonStop compiler's own macro, which the frozen table cannot see, so
// `tools/check-cond-eval.py lib/root-uid.h` reports 2 conditions with 1 disagreement - and the
// evaluator is right to be unsure: on the frozen target the arm C 27 selects is `0`.  Both arms are
// therefore carried, C 27's first because C's own `#else` is the definition the extractor records
// (it keeps the last), and C 25's behind the condition's name as P-51 requires - the shape the
// gnulib mirrors use for a condition the frozen table cannot decide (status 3.131-3.132).  The
// feature name drops the leading underscores the way `HAVE_ANDROID` becomes `have_android`.
//
// C's macro has no type of its own (PORTING.md 1.1); `uid_t` is the port's spelling because the
// header's only user, `lib/euidaccess.c`, compares it against `geteuid ()`, and that unit provides
// `uid_t` through its own includes (PORTING.md 0.1-9).  Before this mirror existed, euidaccess.rs
// carried the value as a use-site copy and described this file as "two lines", which it is not.

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const ROOT_UID_H_: bool = true;

/* C 27: `# define ROOT_UID 0` - the arm the frozen configuration selects.  */
pub const ROOT_UID: uid_t = 0;

/* C 25: `# define ROOT_UID 65535` - the `__TANDEM` arm, carried behind the condition's name.  */
#[cfg (feature = "tandem")]
pub const ROOT_UID: uid_t = 65535;
