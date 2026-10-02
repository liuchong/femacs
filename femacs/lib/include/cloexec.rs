// femacs: mirror of lib/cloexec.h @ fd7afced403 (GNU Emacs 32.0.50); PORTING.md 0.1.
//
// The reference header is two prototypes and the comments that go with them; the definitions
// live in `femacs/lib/cloexec.rs`.  C's `#include "cloexec.h"` in that unit is this mirror.

/// C: int set_cloexec_flag (int desc, bool value)
/// C: int dup_cloexec (int fd)
/* Whole-import module: `femacs/lib/cloexec.rs` defines both names (PORTING.md 0.1-10).  */
pub mod cloexec_decls {
  use super::*;

  unsafe extern "C" {
    pub fn set_cloexec_flag (desc: c_int, value: bool) -> c_int;
    pub fn dup_cloexec (fd: c_int) -> c_int;
  }
}
use cloexec_decls::*;
