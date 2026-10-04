// femacs: mirror of nt/inc/langinfo.h @ fd7afced403.
//
// The Windows port's own `<langinfo.h>`: an anonymous `enum` of locale items with the
// `nl_langinfo` entry point, plus - for each member - a self-referential `#define` whose only
// purpose is to make the member visible to `#ifdef`.  The file is not on
// `docs/baseline/config/cond-eval-verified.txt` and has no `#if` of its own, so the whole file
// is live text.  Nothing in the frozen configuration includes it (no `nt/` translation unit is
// compiled - docs/baseline/frozen-build-sources.txt's batch-F section - and no `src/` unit
// includes a `nt/inc/` header), so `xtask layout-check` never compiles this mirror; in a MinGW
// build `src/fns.c`, `src/w32proc.c`, `lib-src/ntlib.c` and `lib/regex_internal.h` are among
// the units whose `#include <langinfo.h>` resolves here.
//
// PORTING.md 1.3's enum rule is what the members below follow: the C enum becomes `c_int` with
// one constant per member, keeping C's values.  The self-referential `#define X X` lines have no
// Rust counterpart beyond the constant itself; those 22 rows are recorded in
// `docs/ports/macro-body-notes.tsv` (the same shape nt/inc/sys/resource.h's `RLIMIT_*` rows).

// C 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _LANGINFO_H: bool = true;

include!("nl_types.rs");   // C 23: `<nl_types.h>` (nl_item)

// C 25-33: `enum { CODESET, DAY_1, …, MON_12, _NL_PAPER_WIDTH, _NL_PAPER_HEIGHT, _NL_NUM }` -
// C's declaration order and values.  `nl_item` is the included `nl_types.rs`'s typedef, i.e.
// PORTING.md 1.3's "the C enum becomes its integer type plus one constant per member".
pub const CODESET: nl_item = 0;
pub const DAY_1: nl_item = 1;
pub const DAY_2: nl_item = 2;
pub const DAY_3: nl_item = 3;
pub const DAY_4: nl_item = 4;
pub const DAY_5: nl_item = 5;
pub const DAY_6: nl_item = 6;
pub const DAY_7: nl_item = 7;
pub const MON_1: nl_item = 8;
pub const MON_2: nl_item = 9;
pub const MON_3: nl_item = 10;
pub const MON_4: nl_item = 11;
pub const MON_5: nl_item = 12;
pub const MON_6: nl_item = 13;
pub const MON_7: nl_item = 14;
pub const MON_8: nl_item = 15;
pub const MON_9: nl_item = 16;
pub const MON_10: nl_item = 17;
pub const MON_11: nl_item = 18;
pub const MON_12: nl_item = 19;
pub const _NL_PAPER_WIDTH: nl_item = 20;
pub const _NL_PAPER_HEIGHT: nl_item = 21;
/// C 32: `_NL_NUM`  `/* Number of enumerated values.  */` - the enum's last member.
pub const _NL_NUM: nl_item = 22;

// C 36-60: the 22 `#define CODESET CODESET` … `#define _NL_PAPER_HEIGHT _NL_PAPER_HEIGHT`
// self-references.  Each names itself and nothing else, so it adds no value the constant above
// does not already carry - its only purpose in C is to make the member visible to `#if`, which
// Rust has no preprocessor for.  The constant is therefore the whole counterpart, and the 22
// rows are recorded in `docs/ports/macro-body-notes.tsv` (one globbed row: every one of those
// bodies is a single self-reference, so the token list is exactly the macro's own name); this is
// the same shape nt/inc/sys/resource.h recorded for its `RLIMIT_*` self-references.

pub mod langinfo_decls {
  use super::*;

  unsafe extern "C" {
    /// C 62: `extern char *nl_langinfo (nl_item);`
    pub fn nl_langinfo (item: nl_item) -> *mut c_char;
  }
}
use langinfo_decls::*;
