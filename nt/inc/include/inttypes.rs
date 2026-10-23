// femacs: mirror of nt/inc/inttypes.h @ fd7afced403.
//
// The Windows port's `<inttypes.h>` shim: MinGW's own header is used unchanged, and only the
// MSVC tool chain (where there is no `<inttypes.h>`) gets the two `strto*max` renames out of
// `"stdint.h"`.  The file is not on `docs/baseline/config/cond-eval-verified.txt`, so the
// extractor resolves no `#if` in it and reads both arms as live text.  Nothing in the frozen
// configuration includes it (no `nt/` translation unit is compiled -
// docs/baseline/frozen-build-sources.txt's batch-F section - and no `src/` unit includes a
// `nt/inc/` header), so `xtask layout-check` never compiles this mirror; in a MinGW build
// `src/lisp.h`, `src/itree.h`, `src/sfnt.c`, `lib-src/hexl.c` and `lib-src/emacsclient.c`
// are among the units whose `#include <inttypes.h>` resolves here.

// C 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _REPL_INTTYPES_H: bool = true;

// C 26: `#include "stdint.h"` - the sibling shim, and the arm that carries the two renames
// below (the `__MINGW32__` arm above it is the `#include_next <inttypes.h>` one, dead text on
// the MSVC tool chain this block is for).
include!("stdint.rs");

// C 31-32: `#define strtoumax strtoul` and `#define strtoimax strtol` - the 64-bit spellings
// in the `_WIN64` arm above them (`_strtoui64`/`_strtoi64`) are the same rename for that
// configuration, and C defines both pairs, so only the presence of the names is compared.
macro_rules! strtoumax { ($($args:tt)*) => { strtoul ($($args)*) }; }
macro_rules! strtoimax { ($($args:tt)*) => { strtol ($($args)*) }; }
