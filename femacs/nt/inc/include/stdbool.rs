// femacs: mirror of nt/inc/stdbool.h @ fd7afced403.
//
// The Windows port's `<stdbool.h>` shim.  GCC has `<stdbool.h>` and only needs the include
// passed on, so on the MinGW build this shim serves - `__GNUC__` is what MinGW is - the whole
// file is one `#include_next`.  The `!__GNUC__` arm below it is the MSVC case: it spells a
// boolean type and its three literals.  The file is not on
// `docs/baseline/config/cond-eval-verified.txt`, so the extractor reads both arms as live text,
// and nothing in the frozen configuration includes it (no `nt/` translation unit is compiled -
// docs/baseline/frozen-build-sources.txt's batch-F section - and no `src/` unit includes a
// `nt/inc/` header), so `xtask layout-check` never compiles this mirror; in a MinGW build
// `src/emacs-module.in.h` and `lib/mktime.c` are among the units whose `#include <stdbool.h>`
// resolves here.

// C 2: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _NT_STDBOOL_H_: bool = true;

/// C 10: `# define _Bool signed char` - the MSVC boolean type; the port spells it `c_schar`,
/// and Rust's own `bool` is what a unit would use (the type-rename row is in
/// `docs/ports/excluded-macros.txt`, because the gate's item scan reads macros and constants,
/// not types).
pub type _Bool = c_schar;

// C 11-13: `# define bool _Bool`, `# define false 0`, `# define true 1`.  All three names are
// Rust keywords rather than items - `const true …`, `type bool …` and even `macro_rules! true`
// are rejected by the parser - and Rust's own `bool`/`false`/`true` are the counterparts, so
// nothing is declared for them and the three rows in `docs/ports/excluded-macros.txt` record
// it (the `alignas` shape: the counterpart is a language feature, not an item).
