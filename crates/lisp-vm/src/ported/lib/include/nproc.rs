// femacs: mirror of lib/nproc.h @ fd7afced403 (PORTING.md P-26).
//
// The header is one enum (C 26-32) and one prototype (C 36).  It has no include guard (the
// file starts with a comment block and the `#ifdef __cplusplus` pair, which PORTING.md
// 0.1 drops), so there is no guard marker constant here.  C's `unsigned long int` is the
// port's `c_ulong`.
//
// The declaration sits in `pub mod nproc_decls` + glob import (the include/fsusage.rs
// shape): lib/nproc.c defines `num_processors`, and an explicit item shadows a glob import
// (PORTING.md 0.1-10).

// [U1-MODULE-MIRROR]
use super::*;
/* A "processor" in this context means a thread execution unit, that is either
   - an execution core in a (possibly multi-core) chip, in a (possibly multi-
     chip) module, in a single computer, or
   - a thread execution unit inside a core
     (hyper-threading, see <https://en.wikipedia.org/wiki/Hyper-threading>).
   Which of the two definitions is used, is unspecified.  */
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum nproc_query {
  NPROC_ALL,                 /* total number of processors */
  NPROC_CURRENT,             /* processors available to the current process */
  NPROC_CURRENT_OVERRIDABLE, /* likewise, but overridable through the
                                OMP_NUM_THREADS environment variable */
}

pub mod nproc_decls {
  use super::*;

  unsafe extern "C" {
    /* unsigned long int num_processors (enum nproc_query query);  */
    pub fn num_processors (query: nproc_query) -> c_ulong;
  }
}
pub use self::nproc_decls::*;
