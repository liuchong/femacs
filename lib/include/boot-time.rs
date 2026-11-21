// femacs: port of lib/boot-time.h @ fd7afced403 (gnulib).
//
// Header mirror (PORTING.md 0.1): the `_BOOT_TIME_H` guard, `#include <time.h>` (for
// `struct timespec`) and the one prototype.  The declaration lives in `boot_time_decls` +
// glob import (the tempname.rs shape), because `boot-time.c`'s port defines `get_boot_time`
// and an explicit item shadows a glob import (PORTING.md 0.1-10).
//
// C 24-26/45-47's `#ifdef __cplusplus` arm has no Rust counterpart: there is no C++ here.

// C 19-20: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const _BOOT_TIME_H: bool = true;

// [U1] time.in.rs imported via super

pub mod boot_time_decls {
  use super::*;

  unsafe extern "C" {
    /* C 42: `extern int get_boot_time (struct timespec *p_boot_time);` - parameter name
       from the definition in lib/boot-time.c, C's prototype has none.  */
    pub fn get_boot_time (p_boot_time: *mut timespec) -> c_int;
  }
}
pub use self::boot_time_decls::*;
