// femacs: WIP mirror of lib/getopt-core.h @ fd7afced403 (PORTING.md P-26).
//
// The header declares the four variables and `getopt` itself; the `__BEGIN_DECLS` /
// `__END_DECLS` pair around them is `extern "C"` (getopt-cdefs.in.rs), and the names are the
// `rpl_`-prefixed ones because getopt-pfx-core.h re-includes this header with
// `__GETOPT_PREFIX` = `rpl_` in the frozen configuration (PORTING.md 10.13).
//
// Declarations live in `pub mod decls` + glob import (the stdio.rs/utimens.rs shape): the unit
// that defines them (lib/getopt.rs) has items of the same name, and an explicit item shadows
// a glob import (PORTING.md 0.1-10).
//
// `getopt` has a second, unprefixed declaration in this unit's text - the system <getopt.h>
// that getopt.h `#include_next`s (C's `_GL_SYSTEM_GETOPT` arm).  Only the prefixed entry is
// declared here: it is the one the frozen build links.

pub mod getopt_core_decls {
  use super::*;

  unsafe extern "C" {
    /* extern char *optarg;  */
    #[link_name = "rpl_optarg"]
    pub static mut optarg: *mut c_char;
    /* extern int optind;  */
    #[link_name = "rpl_optind"]
    pub static mut optind: c_int;
    /* extern int opterr;  */
    #[link_name = "rpl_opterr"]
    pub static mut opterr: c_int;
    /* extern int optopt;  */
    #[link_name = "rpl_optopt"]
    pub static mut optopt: c_int;

    /* extern int getopt (int ___argc, char *const *___argv, const char *__shortopts)
         __THROW _GL_ARG_NONNULL ((2, 3));  */
    #[link_name = "rpl_getopt"]
    pub fn getopt (___argc: c_int, ___argv: *const *mut c_char, __shortopts: *const c_char)
      -> c_int;
  }
}
use getopt_core_decls::*;

/* C 21: the include guard, whose C body is `1`.  */
pub const _GETOPT_CORE_H: core::ffi::c_int = 1;
