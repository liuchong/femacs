#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// femacs: port of lib/pthread_sigmask.c @ fd7afced403.  The file has no
// live text in the frozen configuration — `HAVE_PTHREAD_SIGMASK` is 1 and none of
// `PTHREAD_SIGMASK_INEFFECTIVE`/`PTHREAD_SIGMASK_NOT_IN_LIBC`/`PTHREAD_SIGMASK_FAILS_WITH_ERRNO`
// is defined, so gnulib does not compile it (there is no `build/emacs-ref/lib/pthread_sigmask.o`)
// and the port must read it in the configuration in which it *is* built, namely glibc's model:
// `# undef pthread_sigmask` at C 18 (between the declarator and the body) undoes the
// `#define pthread_sigmask rpl_pthread_sigmask` the generated <signal.h> gives when the
// replacement exists.  So C's definition is `rpl_pthread_sigmask` and its body's
// `pthread_sigmask (how, new_mask, old_mask_ptr)` is the *system* one — the port emits the
// renamed symbol with `#[unsafe(export_name = "rpl_pthread_sigmask")]` while keeping C's
// source name on the item (PORTING.md 10.13①, the mempcpy precedent), and the body calls the
// system entry through a `#[link_name]` declaration (a same-named Rust item would resolve the
// call to the definition itself — the defect docs/ports/pthread_sigmask.md records in
// lib/nanosleep.rs's landed port of the same idiom).
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");      // <errno.h>
include!("include/signal.rs");     // <signal.h> (`sigset_t`)
include!("include/string.rs");     // <string.h> (the `PTHREAD_SIGMASK_INEFFECTIVE` arm)

/* The native Windows implementation is defined in sigprocmask.c.  */
/* C 7's `#if !(defined _WIN32 && !defined __CYGWIN__)` is live.  */

#[unsafe(export_name = "rpl_pthread_sigmask")]
pub unsafe extern "C" fn pthread_sigmask (how: c_int, new_mask: *const sigset_t,
                                          old_mask: *mut sigset_t) -> c_int
{
  /* C 21-32 and C 36-54 (`#if PTHREAD_SIGMASK_INEFFECTIVE`, undefined) and C 55-58
     (`#if PTHREAD_SIGMASK_FAILS_WITH_ERRNO`, undefined) are this target's dead arms: the
     first declares `sigset_t omask`/`omask_copy`, `sigemptyset`/`sigaddset (SIGILL)`,
     `memcpy` and compares with `memeq`; the second re-checks by calling
     `pthread_sigmask (1729, …)` and falls back to `sigprocmask`; the third converts a -1
     return to `errno`.  Their tokens are not required here (the file is in
     cond-eval-verified.txt, so the condition table decides them — PORTING.md P-75).  */

  /* C 31 (the live arm of the inner `#if`).  */
  let old_mask_ptr: *mut sigset_t = old_mask;

  /* C 34: C's `pthread_sigmask` here is the *system* entry (the `# undef` at C 18 undid the
     rename), reached through the signal mirror's module rather than the glob, because the
     definition above shadows the glob's name (PORTING.md 0.1-10).  */
  let ret: c_int = signal_decls::pthread_sigmask (how, new_mask, old_mask_ptr);

  /* C 59.  */
  return ret;
}
