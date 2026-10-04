// femacs: mirror of nt/inc/signal.h @ fd7afced403.
//
// The Windows port's addition to the system `<signal.h>`: POSIX puts `sig2str` there and the
// tree assumes it, but MinGW's build cannot use the Gnulib header, so the one constant and the
// two prototypes are re-declared here and the real header is pulled in after them
// (`#include_next <signal.h>`, C 37).  The file has no `#if` for the extractor to choose
// between and is not on `docs/baseline/config/cond-eval-verified.txt`, so the whole file is live
// text.  Nothing in the frozen configuration includes it (no `nt/` translation unit is compiled
// - docs/baseline/frozen-build-sources.txt's batch-F section - and no `src/` unit includes a
// `nt/inc/` header), so `xtask layout-check` never compiles this mirror; in a MinGW build
// `src/comp.c`, `src/filelock.c`, `src/w32term.c`, `src/w32menu.c` and `src/msdos.h` are among
// the many units whose `#include <signal.h>` resolves here.
//
// The `#include_next` target is MinGW's own `<signal.h>`, not the frozen target's, so the port's
// `<signal.h>` mirror (`lib/include/signal.rs`) is deliberately not pulled in - the same choice
// `nt/inc/sys/include/time.rs` records for `<sys/time.h>`.

// C 23: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _NT_INC_SIGNAL_H_: bool = true;

/// C 31: `#  define SIG2STR_MAX (5 + 1 + 10 + 1 + 2)` - the longest name `sig2str` can return,
/// NUL included (`/* Maximum size of a signal name returned by sig2str() … */`).
pub const SIG2STR_MAX: c_int = 5 + 1 + 10 + 1 + 2;

pub mod signal_decls {
  use super::*;

  unsafe extern "C" {
    /// C 34: `int sig2str (int, char *);`
    pub fn sig2str (sig: c_int, buffer: *mut c_char) -> c_int;
    /// C 35: `int str2sig (char const *, int *);`
    pub fn str2sig (name: *const c_char, sig: *mut c_int) -> c_int;
  }
}
use signal_decls::*;

// C 37: `# include_next <signal.h>` - the system header; see the head note.
