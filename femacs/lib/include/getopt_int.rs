// femacs: WIP mirror of lib/getopt_int.h @ fd7afced403 (PORTING.md P-26).
//
// The internal declarations lib/getopt.c and lib/getopt1.c share.  `_getopt_internal` is
// prefixed by getopt-pfx-ext.h and is `rpl_getopt_internal` in the frozen build (measured:
// `nm build/emacs-ref/lib/getopt.o` defines `_rpl_getopt_internal`); the three `_*_r`
// functions are *not* in the prefix list and keep their C names (measured: getopt.o defines
// `__getopt_internal_r` and getopt1.o `__getopt_long_r`, `__getopt_long_only_r`).
//
// The declarations are in `pub mod decls` + glob import so that lib/getopt.c, which defines
// `_getopt_internal` and `_getopt_internal_r`, shadows them with its own items
// (PORTING.md 0.1-10).

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const _GETOPT_INT_H: c_int = 1;

/* getopt_int.h 41-44: `enum __ord`.  */
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum __ord {
  REQUIRE_ORDER = 0,
  PERMUTE = 1,
  RETURN_IN_ORDER = 2,
}

/* getopt_int.h 47-83: `struct _getopt_data`.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _getopt_data {
  /* These have exactly the same meaning as the corresponding global
     variables, except that they are used for the reentrant
     versions of getopt.  */
  pub optind: c_int,
  pub opterr: c_int,
  pub optopt: c_int,
  pub optarg: *mut c_char,

  /* Internal members.  */

  /* True if the internal members have been initialized.  */
  pub __initialized: c_int,

  /* The next char to be scanned in the option-element
     in which the last option character we returned was found.
     This allows us to pick up the scan where we left off.

     If this is zero, or a null string, it means resume the scan
     by advancing to the next ARGV-element.  */
  pub __nextchar: *mut c_char,

  /* See __ord above.  */
  pub __ordering: __ord,

  /* Handle permutation of arguments.  */

  /* Describe the part of ARGV that contains non-options that have
     been skipped.  'first_nonopt' is the index in ARGV of the first
     of them; 'last_nonopt' is the index after the last of them.  */

  pub __first_nonopt: c_int,
  pub __last_nonopt: c_int,
}

/* getopt_int.h 86-88: the initializer C uses for a fresh `struct _getopt_data`.  */
macro_rules! _GETOPT_DATA_INITIALIZER {
  () => { _getopt_data { optind: 1, opterr: 1, optopt: 0, optarg: std::ptr::null_mut (),
                         __initialized: 0, __nextchar: std::ptr::null_mut (),
                         __ordering: __ord::REQUIRE_ORDER, __first_nonopt: 0,
                         __last_nonopt: 0 } };
}

pub mod getopt_int_decls {
  use super::*;

  unsafe extern "C" {
    /* extern int _getopt_internal (int ___argc, char **___argv, const char *__shortopts,
                                    const struct option *__longopts, int *__longind,
                                    int __long_only, int __posixly_correct);  */
    #[link_name = "rpl_getopt_internal"]
    pub fn _getopt_internal (___argc: c_int, ___argv: *mut *mut c_char,
                             __shortopts: *const c_char, __longopts: *const option,
                             __longind: *mut c_int, __long_only: c_int,
                             __posixly_correct: c_int) -> c_int;
    /* extern int _getopt_internal_r (int ___argc, char **___argv, const char *__shortopts,
                                      const struct option *__longopts, int *__longind,
                                      int __long_only, struct _getopt_data *__data,
                                      int __posixly_correct);  */
    pub fn _getopt_internal_r (___argc: c_int, ___argv: *mut *mut c_char,
                               __shortopts: *const c_char, __longopts: *const option,
                               __longind: *mut c_int, __long_only: c_int,
                               __data: *mut _getopt_data, __posixly_correct: c_int) -> c_int;
    /* extern int _getopt_long_r (int ___argc, char **___argv, const char *__shortopts,
                                  const struct option *__longopts, int *__longind,
                                  struct _getopt_data *__data);  */
    pub fn _getopt_long_r (___argc: c_int, ___argv: *mut *mut c_char,
                           __shortopts: *const c_char, __longopts: *const option,
                           __longind: *mut c_int, __data: *mut _getopt_data) -> c_int;
    /* extern int _getopt_long_only_r (int ___argc, char **___argv, const char *__shortopts,
                                       const struct option *__longopts, int *__longind,
                                       struct _getopt_data *__data);  */
    pub fn _getopt_long_only_r (___argc: c_int, ___argv: *mut *mut c_char,
                                __shortopts: *const c_char, __longopts: *const option,
                                __longind: *mut c_int, __data: *mut _getopt_data) -> c_int;
  }
}
use getopt_int_decls::*;
