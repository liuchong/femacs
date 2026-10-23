// femacs: port of lib/arg-nonnull.h @ fd7afced403 (gnulib); PORTING.md 0.1.
//
// Header mirror (PORTING.md 0.1): the header declares exactly one macro and has no
// functions, constants, structs or file-scope objects, so what the file carries is the
// disposition of that macro, not a declaration.
//
// `tools/check-cond-eval.py lib/arg-nonnull.h` reports 2 conditions, 0 unknown, 0
// disagreements: for the frozen target's clang C 21's `defined __clang__` term is true, so
// C 22's arm is the live one and `_GL_ARG_NONNULL (params)` spells
// `__attribute__ ((__nonnull__ params))`; C 24's empty arm is dead text.  Both arms are
// attribute spellings, and attributes are syntax rather than semantics: the port drops them
// structurally (PORTING.md 0.3) and the macro owns a row in
// docs/ports/excluded-macros.txt (`_GL_ARG_NONNULL  syntax  attribute (lib/arg-nonnull.h);
// no Rust counterpart`), which is where the call sites that spell it — getdelim.rs,
// canonicalize-lgpl.rs, and the mirrors that copy the prototypes (`include/getopt-core.rs`,
// `include/getopt.in.rs`, `include/timespec.rs`) — are accounted for.
//
// No translation unit includes this mirror.  In the reference the header is a gnulib module
// snippet installed by the build machinery (`emacs/lib/gnulib.mk.in` 3159-3171) and reached
// from `system.h`'s `_GL_ARG_NONNULL` uses only through that spelling; the port's aggregates
// name the declarations they take from each other directly (PORTING.md 0.1-4 / P-61), so
// there is nothing for a unit to include.
