// femacs: mirror of nt/inc/nl_types.h @ fd7afced403.
//
// The Windows port's replacement for `<nl_types.h>`, which NT has not got: one typedef and
// nothing else.  The file is not on `docs/baseline/config/cond-eval-verified.txt`, but it has
// no `#if` for the extractor to resolve, so the whole file is live text.  Nothing in the frozen
// configuration includes it (no `nt/` translation unit is compiled -
// docs/baseline/frozen-build-sources.txt's batch-F section - and no `src/` unit includes a
// `nt/inc/` header), so `xtask layout-check` never compiles this mirror; in a MinGW build
// `src/w32proc.c`, `lib-src/ntlib.c` and this tree's own `nt/inc/langinfo.h` are the units whose
// `#include <nl_types.h>` resolves here.

// C 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _NL_TYPES_H: bool = true;

/// C 23: `typedef int nl_item;` - the locale-item type `nl_langinfo` takes.
pub type nl_item = c_int;
