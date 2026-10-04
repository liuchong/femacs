// femacs: mirror of msdos/inttypes.h @ fd7afced403.
//
// The MS-DOS replacement header for `<inttypes.h>`: DJGPP v2.04's own header omits `stdint.h`,
// so this puts it back and, below 2.04, supplies the four macros the C tree needs.  The frozen
// configuration never puts `msdos/` on its include path and compiles no `msdos/` unit, so the
// mirror is text only.  `msdos/inttypes.h` is not in
// `docs/baseline/config/cond-eval-verified.txt`, so both arms of C 30's `__DJGPP__` test are
// carried (PORTING.md P-51), the dead one under the feature that names its condition.

/* C 25-28: `# include <stdint.h>` is unconditional, and C 31's `# include_next <inttypes.h>`
   is the compiler's own header - an `#include` has no mirror to name here, so both are recorded
   (the binary-io.rs dead-arm note).  */

/* C 20-21's include guard.  */
pub const _REPL_INTTYPES_H: bool = true;

/* C 32: `# include <stdlib.h>` - the arm that needs the two `strto*` names below.  */

/* C 33-34: `# define strtoumax strtoull` and `# define strtoimax strtoll`, kept as the
   function macros C's text spells (PORTING.md 0.1: a rename has no Rust counterpart).  */
#[cfg (feature = "no_djgpp_2_4")]
macro_rules! strtoumax {
  ($nptr:expr, $endptr:expr, $base:expr) => { strtoull ($nptr, $endptr, $base) };
}
#[cfg (feature = "no_djgpp_2_4")]
macro_rules! strtoimax {
  ($nptr:expr, $endptr:expr, $base:expr) => { strtoll ($nptr, $endptr, $base) };
}

/* C 35-36: `# define PRIuMAX "llu"` and `# define PRIdMAX "lld"` - the DJGPP spellings of the
   `printf` length modifier (the frozen target's own `<inttypes.h>` says `"ju"`/`"jd"`, which is
   why the port keeps the two apart per header).  */
#[cfg (feature = "no_djgpp_2_4")]
pub const PRIuMAX: &core::ffi::CStr = c"llu";
#[cfg (feature = "no_djgpp_2_4")]
pub const PRIdMAX: &core::ffi::CStr = c"lld";
