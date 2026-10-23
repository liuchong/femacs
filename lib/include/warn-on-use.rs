// femacs: mirror of lib/warn-on-use.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1, P-26).  The header declares four function-like macros and
// nothing else - no function, constant, type or file-scope object - and all four names are
// the frozen configuration's own: docs/baseline/config/darwin-arm64.macros.tsv carries
// `_GL_WARN_EXTERN_C` (4268), `_GL_WARN_ON_USE` (4269), `_GL_WARN_ON_USE_ATTRIBUTE` (4270)
// and `_GL_WARN_ON_USE_CXX` (4271) with the clang spellings the generated headers use.  Each
// of the file's three guards tests one of those names (`#ifndef _GL_WARN_ON_USE` C 86,
// `#ifndef _GL_WARN_ON_USE_CXX` C 119, `#ifndef _GL_WARN_EXTERN_C` C 147), so every
// `#define` in the file is dead text and the live values are the configuration's.
//
// Measured, not assumed: `python3 tools/extract-abi.py emacs/lib/warn-on-use.h` lists
// 0 functions / 0 macros / 0 types / 0 objects, and `python3 tools/check-cond-eval.py
// lib/warn-on-use.h` prints `lib/warn-on-use.h: 12 conditions, 0 unknown, 0 disagreements
// [VERIFIED]`, so the branch verdicts are the ones clang takes.  Of the four arms the file
// writes, the live one is clang's `__attribute__ ((__diagnose_if__ (1, message, "warning")))`
// (C 96-103); gcc's `__warning__` arm (C 90-93), the `GNULIB_STRICT_CHECKING` arm (C 103-105)
// and the unsupported `int _gl_warn_on_use` arm (C 107-109) are dead.
//
// Every arm is an attribute spelling or an unused extern declaration.  Attributes are syntax
// rather than semantics and are dropped structurally (PORTING.md 0.3 and the attribute family
// in 8), and the port's aggregates name their own declarations instead of inheriting them
// (PORTING.md 0.1-4), so this mirror carries the disposition and no item: `--check` against
// the file below is `OK` with no problem lines.
//
// No translation unit includes this mirror, which is the reference's own shape.  Measured:
// `grep -rn 'warn-on-use.h' emacs/src emacs/lib` prints only `emacs/lib/gnulib.mk.in` - the
// build machinery substitutes the header into other gnulib headers through its
// `$(WARN_ON_USE_H)` variable, and nothing spells the include itself.  Those users' mirrors
// (include/string.in.rs, include/inttypes.in.rs, include/sys_stat.in.rs) spell the resulting
// warnings as ordinary declarations, which is where the call sites are accounted for.
//
// The file is its own compilation unit here: `cd femacs && CARGO_TARGET_DIR=... cargo run -q
// -p xtask -- build-tu lib/warn-on-use.h` reaches `lib/include/warn-on-use.rs` as the
// crate root (xtask's `port_path`), and a mirror with no item needs no includer's scope.
