// femacs: mirror of lib/assert.in.h @ fd7afced403 (gnulib template); PORTING.md 0.1.
//
// Header mirror (PORTING.md 0.1): this template carries **no item** at all, and that is measured
// rather than assumed - `tools/extract-abi.py lib/assert.in.h` reports 0 subrs, 0 functions,
// 0 macros, 0 types and 0 file-scope objects, because everything the file contains is
// preprocessor text:
//
//   * C 17-18 says why the include is *not* guarded: <assert.h> is supposed to define `assert`
//     every time it is included, so this template has no include guard either - there is no guard
//     name to carry;
//   * C 20-22 is `#if __GNUC__ >= 3` around `@PRAGMA_SYSTEM_HEADER@`.  That condition is exactly
//     the one the frozen table cannot settle: `tools/check-cond-eval.py lib/assert.in.h` reports
//     `1 conditions, 0 unknown, 1 disagreements` (clang sees `__GNUC__` undefined in this
//     configuration, the evaluator's copy of the macro table does not), and either arm carries the
//     same single `#pragma GCC system_header` line - a pragma is not a declaration, so the
//     disagreement cannot change what the mirror holds;
//   * C 23 is `@PRAGMA_COLUMNS@`, the same class;
//   * C 25 is `#@INCLUDE_NEXT@ @NEXT_ASSERT_H@`, i.e. `#include_next <assert.h>`: the system
//     header is the target's own, and the port's units reach it through their own includes
//     (PORTING.md 0.1-9);
//   * C 27's comment is the template's note about the copied `static_assert`.
//
// Where C's `assert` macro is actually needed, the unit that needs it mirrors the macro itself,
// the way `lib-src/ebrowse.rs` and `include/android-asset.rs` do - this template has nothing to
// hand them.
