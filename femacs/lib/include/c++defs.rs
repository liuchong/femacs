// femacs: port of lib/c++defs.h @ fd7afced403 (gnulib); PORTING.md 0.1.
//
// Header mirror (PORTING.md 0.1).  This header has no item the frozen target needs: every
// definition in it lives in a C++-only arm (`#if defined __cplusplus && defined
// GNULIB_NAMESPACE` for the namespace and alias machinery at C 21-27, 165-183, 200-218,
// 238-256, 263-281, 289-312, 318-340, 345-362; `#if defined __cplusplus` for the `_GL_EXTERN_C`
// pair at C 90-105 and C 134-138), and the `#else` arms those guards select in C are no-op
// spellings of the same names.  `tools/extract-abi.py lib/c++defs.h` therefore reports 0 subrs,
// 0 functions, 0 macros, 0 types and 0 file-scope objects, and `tools/check-cond-eval.py
// lib/c++defs.h` confirms the file's 27 conditions against clang (0 unknown, 0 disagreements),
// so the reading is measured rather than assumed.
//
// What the file *is* for still has an owner on this side: the `_GL_CXXALIAS_*` / `_GL_FUNCDECL_*`
// macros are how the generated gnulib headers name the system entry point they resolve to, and
// the mirrors that use them record the resolution at each use site - fcntl.in.rs 274-276 (creat,
// open, openat), time.in.rs 156, unistd.in.rs 24, sys_stat.in.rs 43 and 175.  None of those
// mirrors includes this one, because an include whose expansion carries no item would be a
// no-op include; that deviation is recorded in docs/ports/include-c++defs.md rather than left
// implicit.  C's own include guard is carried below, the way the other mirrors carry theirs
// (PORTING.md 0.1-4: the guard's job is the include graph's dedup, the name is what the ledger
// compares).
pub const _GL_CXXDEFS_H: bool = true;
