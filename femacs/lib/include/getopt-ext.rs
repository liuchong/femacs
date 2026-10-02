// femacs: WIP mirror of lib/getopt-ext.h @ fd7afced403 (PORTING.md P-26).
//
// The GNU extensions: `struct option` and the two entry points.  As in getopt-core.rs the
// names are the `rpl_`-prefixed ones (getopt-pfx-ext.h's rename layer, measured on the
// reference object: `nm build/emacs-ref/lib/getopt1.o` defines `_rpl_getopt_long` and
// `_rpl_getopt_long_only`), and the argv parameter is `char **` because the frozen
// configuration defines `__getopt_argv_const` as empty.

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const _GETOPT_EXT_H: c_int = 1;

/* getopt-ext.h 45-53: `struct option`.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct option {
  pub name: *const c_char,
  /* has_arg can't be an enum because some compilers complain about
     type mismatches in all the code that assumes it is an int.  */
  pub has_arg: c_int,
  pub flag: *mut c_int,
  pub val: c_int,
}

/* getopt-ext.h 57-59: names for the values of the 'has_arg' field.  */
pub const no_argument: c_int = 0;
pub const required_argument: c_int = 1;
pub const optional_argument: c_int = 2;

pub mod getopt_ext_decls {
  use super::*;

  unsafe extern "C" {
    /* extern int getopt_long (int ___argc, char *__getopt_argv_const *___argv,
                               const char *__shortopts,
                               const struct option *__longopts, int *__longind);  */
    #[link_name = "rpl_getopt_long"]
    pub fn getopt_long (___argc: c_int, ___argv: *mut *mut c_char, __shortopts: *const c_char,
                        __longopts: *const option, __longind: *mut c_int) -> c_int;
    /* extern int getopt_long_only (int ___argc, char *__getopt_argv_const *___argv,
                                    const char *__shortopts,
                                    const struct option *__longopts, int *__longind);  */
    #[link_name = "rpl_getopt_long_only"]
    pub fn getopt_long_only (___argc: c_int, ___argv: *mut *mut c_char,
                             __shortopts: *const c_char, __longopts: *const option,
                             __longind: *mut c_int) -> c_int;
  }
}
use getopt_ext_decls::*;
