// femacs: port of src/systhread.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// parameter, type).  The twelve definitions are C's exported symbols (none is `static`), so
// they carry `#[unsafe(no_mangle)]` (PORTING.md 0.2/P-52).
//
// The frozen configuration has `THREADS_ENABLED` undefined
// (docs/baseline/config/darwin-arm64.macros.tsv 3678), so C 33's `#ifndef THREADS_ENABLED`
// arm is the live text and the rest of the file is the threaded build's:
//
// - C 29-31 and C 13's `#include <pthread_np.h>`: `HAVE_PTHREAD_SET_NAME_NP` is undefined and
//   the include sits inside its own `#ifdef`.
// - C 104-548, the `#elif defined (HAVE_PTHREAD)` arm: `pthread_mutex_init` with the
//   `PTHREAD_MUTEX_ERRORCHECK` attribute under ENABLE_CHECKING, the NS and
//   `pthread_setname_np`/`pthread_set_name_np` spellings of `sys_thread_set_name`, and C 280's
//   `#elif defined (WINDOWSNT)` arm with its `CreateThread`/critical-section code.
// - C 25-27's `#include "nsterm.h"`: `HAVE_NS` is undefined.
//
// They are recorded in docs/ports/systhread.md rather than transcribed (PORTING.md P-08: the
// frozen configuration decides which arm is live, and the dead one is not part of this
// program).  The mirror carries the matching arm of src/systhread.h (the three dummy types).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe, unused_mut)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/string.rs imported via common
// [U1] include/lisp.rs imported via common
// The `<pthread.h>` surface lives in the mirror now: mirrors that name these types
// (`include/systhread.rs`, `include/thread.rs`) cannot see a unit-local declaration (P-240).
// [U1] include/systhread.rs imported via common
// [U1] include/systhread.rs imported via common

// C 13-14's `<setjmp.h>` and `<stdio.h>` come through lisp.h's own chain (lisp.h includes
// both), so the port's include list is C's minus the two duplicates (PORTING.md 0.1-4).

/* C 33's `#ifndef THREADS_ENABLED` arm (live: the macro is undefined on this target).  */

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_mutex_init (m: *mut sys_mutex_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  *m = 0;

  }
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_mutex_lock (m: *mut sys_mutex_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_mutex_unlock (m: *mut sys_mutex_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_init (c: *mut sys_cond_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  *c = 0;

  }
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_wait (c: *mut sys_cond_t, m: *mut sys_mutex_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_signal (c: *mut sys_cond_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_broadcast (c: *mut sys_cond_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_destroy (c: *mut sys_cond_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_self () -> sys_thread_t
{
  return 0;
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_equal (t: sys_thread_t, u: sys_thread_t) -> bool
{
  return t == u;
}
#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_set_name (name: *const c_char)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_create (t: *mut sys_thread_t,
                                            func: thread_creation_function,
                                            datum: *mut c_void) -> bool
{
  return false;
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_yield ()
{
}

/* ------------------------------------------------------------------------------------------
   C 104-278: `#elif defined (HAVE_PTHREAD)` - the arm a threaded pthread build reads.  It is
   dead in the frozen configuration (`THREADS_ENABLED` is undef, macros.tsv 3678) and it is the
   one arm of this file that is a *real* compile on this machine, because pthreads are here
   (docs/ports/systhread.md `## Evidence`).
   ------------------------------------------------------------------------------------------ */

/* C 104's arm needs `<pthread.h>` - the header of C 28 in the reference's src/systhread.h, which
   this arm is the consumer of.  `<pthread.h>` has no mirror in this tree, and the two units that
   already declare parts of it for their own arms (`src/android.rs`, `src/sysdep.rs`) do so with
   *their* platform's numbers, so the surface stands here (P-88 clause 1; the `src/android.rs`
   158-176 shape).  The sizes and the two constants are clang measurements of this target's own
   `<pthread.h>` (docs/ports/systhread.md `## Evidence`).  */

#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
unsafe extern "C" {
  /* int pthread_mutexattr_init (pthread_mutexattr_t *);  */
  pub fn pthread_mutexattr_init (attr: *mut pthread_mutexattr_t) -> c_int;
  /* int pthread_mutexattr_settype (pthread_mutexattr_t *, int);  */
  pub fn pthread_mutexattr_settype (attr: *mut pthread_mutexattr_t, kind: c_int) -> c_int;
  /* int pthread_mutexattr_destroy (pthread_mutexattr_t *);  */
  pub fn pthread_mutexattr_destroy (attr: *mut pthread_mutexattr_t) -> c_int;
  /* int pthread_mutex_init (pthread_mutex_t *, const pthread_mutexattr_t *);  */
  pub fn pthread_mutex_init (mutex: *mut pthread_mutex_t,
                             attr: *const pthread_mutexattr_t) -> c_int;
  /* int pthread_mutex_lock (pthread_mutex_t *);  */
  pub fn pthread_mutex_lock (mutex: *mut pthread_mutex_t) -> c_int;
  /* int pthread_mutex_unlock (pthread_mutex_t *);  */
  pub fn pthread_mutex_unlock (mutex: *mut pthread_mutex_t) -> c_int;
  /* int pthread_cond_init (pthread_cond_t *, const pthread_condattr_t *);  */
  pub fn pthread_cond_init (cond: *mut pthread_cond_t, attr: *const c_void) -> c_int;
  /* int pthread_cond_wait (pthread_cond_t *, pthread_mutex_t *);  */
  pub fn pthread_cond_wait (cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
  /* int pthread_cond_signal (pthread_cond_t *);  */
  pub fn pthread_cond_signal (cond: *mut pthread_cond_t) -> c_int;
  /* int pthread_cond_broadcast (pthread_cond_t *);  */
  pub fn pthread_cond_broadcast (cond: *mut pthread_cond_t) -> c_int;
  /* int pthread_cond_destroy (pthread_cond_t *);  */
  pub fn pthread_cond_destroy (cond: *mut pthread_cond_t) -> c_int;
  /* pthread_t pthread_self (void);  */
  pub fn pthread_self () -> pthread_t;
  /* int pthread_equal (pthread_t, pthread_t);  */
  pub fn pthread_equal (t: pthread_t, u: pthread_t) -> c_int;
  /* int pthread_attr_init (pthread_attr_t *);  */
  pub fn pthread_attr_init (attr: *mut pthread_attr_t) -> c_int;
  /* int pthread_attr_destroy (pthread_attr_t *);  */
  pub fn pthread_attr_destroy (attr: *mut pthread_attr_t) -> c_int;
  /* int pthread_attr_getstacksize (const pthread_attr_t *, size_t *);  */
  pub fn pthread_attr_getstacksize (attr: *const pthread_attr_t, size: *mut usize) -> c_int;
  /* int pthread_attr_setstacksize (pthread_attr_t *, size_t);  */
  pub fn pthread_attr_setstacksize (attr: *mut pthread_attr_t, size: usize) -> c_int;
  /* int pthread_attr_setdetachstate (pthread_attr_t *, int);  */
  pub fn pthread_attr_setdetachstate (attr: *mut pthread_attr_t, state: c_int) -> c_int;
  /* int pthread_create (pthread_t *, const pthread_attr_t *, void *(*)(void *), void *);  */
  pub fn pthread_create (thread: *mut pthread_t, attr: *const pthread_attr_t,
                         start: thread_creation_function, arg: *mut c_void) -> c_int;
}

/* C 106: `#include <sched.h>`.  `<sched.h>` has no mirror and `sched_yield` is its only name
   this arm uses, so it stands here (P-88 clause 1).  */
/// C: sched.h: `int sched_yield (void);`
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
unsafe extern "C" {
  pub fn sched_yield () -> c_int;
}

/// C: nsterm.h: `extern void ns_run_loop_break (void);` - used by C 191-197's `#ifdef HAVE_NS`
/// block inside `sys_cond_broadcast` (the mirror carries it at `include/nsterm.rs` 877-882).
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_ns"))]
unsafe extern "C-unwind" {
  pub fn ns_run_loop_break ();
}

/* C 108-136: `sys_mutex_init`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_mutex_init (sys_mutex: *mut sys_mutex_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let mutex: *mut pthread_mutex_t = SYSTHREAD_ALIGN_PTR! (pthread_mutex_t, sys_mutex);
  let mut attr_ptr: *mut pthread_mutexattr_t;
  /* C: `pthread_mutexattr_t attr;` - uninitialized until `pthread_mutexattr_init` writes it; the
     port zeroes it for the E0381 reason `sys_thread_create`'s `attr` carries (## Deviations).  */
  #[cfg (feature = "enable_checking")]
  let mut attr: pthread_mutexattr_t = core::mem::zeroed ();
  #[cfg (feature = "enable_checking")]
  {
    let error = pthread_mutexattr_init (&raw mut attr);
    eassert! (error == 0);
    let error = pthread_mutexattr_settype (&raw mut attr, PTHREAD_MUTEX_ERRORCHECK);
    eassert! (error == 0);
  }
  #[cfg (feature = "enable_checking")]
  { attr_ptr = &raw mut attr; }
  #[cfg (not (feature = "enable_checking"))]
  { attr_ptr = core::ptr::null_mut (); }
  let mut error = pthread_mutex_init (mutex, attr_ptr);
  /* We could get ENOMEM.  Can't do anything except aborting.  */
  if error != 0
    {
      fprintf ((*stderr.as_mut_ptr ()), c"\npthread_mutex_init failed: %s\n".as_ptr (), strerror (error));
      emacs_abort ();
    }
  #[cfg (feature = "enable_checking")]
  { error = pthread_mutexattr_destroy (&raw mut attr); eassert! (error == 0); }

  }
}

/* C 138-145: `sys_mutex_lock`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_mutex_lock (sys_mutex: *mut sys_mutex_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let mutex: *mut pthread_mutex_t = SYSTHREAD_ALIGN_PTR! (pthread_mutex_t, sys_mutex);
  let error = pthread_mutex_lock (mutex);
  eassert! (error == 0);

  }
}

/* C 147-152: `sys_mutex_unlock`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_mutex_unlock (sys_mutex: *mut sys_mutex_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let mutex: *mut pthread_mutex_t = SYSTHREAD_ALIGN_PTR! (pthread_mutex_t, sys_mutex);
  let error = pthread_mutex_unlock (mutex);
  eassert! (error == 0);

  }
}

/* C 154-166: `sys_cond_init`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_init (sys_cond: *mut sys_cond_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let cond: *mut pthread_cond_t = SYSTHREAD_ALIGN_PTR! (pthread_cond_t, sys_cond);
  let error = pthread_cond_init (cond, core::ptr::null ());
  /* We could get ENOMEM.  Can't do anything except aborting.  */
  if error != 0
    {
      fprintf ((*stderr.as_mut_ptr ()), c"\npthread_cond_init failed: %s\n".as_ptr (), strerror (error));
      emacs_abort ();
    }

  }
}

/* C 168-175: `sys_cond_wait`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_wait (sys_cond: *mut sys_cond_t, sys_mutex: *mut sys_mutex_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let cond: *mut pthread_cond_t = SYSTHREAD_ALIGN_PTR! (pthread_cond_t, sys_cond);
  let mutex: *mut pthread_mutex_t = SYSTHREAD_ALIGN_PTR! (pthread_mutex_t, sys_mutex);
  let error = pthread_cond_wait (cond, mutex);
  eassert! (error == 0);

  }
}

/* C 177-183: `sys_cond_signal`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_signal (sys_cond: *mut sys_cond_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let cond: *mut pthread_cond_t = SYSTHREAD_ALIGN_PTR! (pthread_cond_t, sys_cond);
  let error = pthread_cond_signal (cond);
  eassert! (error == 0);

  }
}

/* C 185-199: `sys_cond_broadcast`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_broadcast (sys_cond: *mut sys_cond_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let cond: *mut pthread_cond_t = SYSTHREAD_ALIGN_PTR! (pthread_cond_t, sys_cond);
  let error = pthread_cond_broadcast (cond);
  eassert! (error == 0);
  #[cfg (feature = "have_ns")]
  {
    /* Send an app defined event to break out of the NS run loop.  It seems that if ns_select is
       running the NS run loop, this broadcast has no effect until the loop is done, breaking a
       couple of tests in thread-tests.el.  */
    ns_run_loop_break ();
  }

  }
}

/* C 201-206: `sys_cond_destroy`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_destroy (sys_cond: *mut sys_cond_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let cond: *mut pthread_cond_t = SYSTHREAD_ALIGN_PTR! (pthread_cond_t, sys_cond);
  let error = pthread_cond_destroy (cond);
  eassert! (error == 0);

  }
}

/* C 208-212: `sys_thread_self`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_self () -> sys_thread_t
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  return pthread_self ();

  }
}

/* C 214-218: `sys_thread_equal`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_equal (t: sys_thread_t, u: sys_thread_t) -> bool
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  return pthread_equal (t, u) != 0;

  }
}

/* C 220-242: `sys_thread_set_name`.  C spells all three arities of `pthread_setname_np` with
   the same name (the `_1ARG`/`_3ARG` macros pick the prototype); Rust cannot overload, so the
   three declarations below carry `#[link_name = "pthread_setname_np"]` and differ only in
   their Rust names (the `charset.rs` 115-116 / pthread_sigmask.rs shape).  */
/// C: string.h: `char *strncpy (char *restrict, const char *restrict, size_t);` - the mirror
/// (`lib/include/string.rs`) carries `strcmp`/`strlen` and friends but not this entry point, and
/// this arm is its only consumer, so it is declared here (P-88 clause 1).
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
unsafe extern "C" {
  pub fn strncpy (dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
}

/// C: pthread.h (the `HAVE_PTHREAD_SETNAME_NP_1ARG` spelling): `int pthread_setname_np (const char *);`
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_pthread_setname_np"))]
unsafe extern "C" {
  #[link_name = "pthread_setname_np"]
  pub fn pthread_setname_np_1 (name: *const c_char) -> c_int;
}
/// C: pthread.h (the `HAVE_PTHREAD_SETNAME_NP_3ARG` spelling): `int pthread_setname_np (pthread_t, const char *, const char *);`
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_pthread_setname_np",
            feature = "have_pthread_setname_np_3arg"))]
unsafe extern "C" {
  #[link_name = "pthread_setname_np"]
  pub fn pthread_setname_np_3 (thread: pthread_t, format: *const c_char,
                               name: *const c_char) -> c_int;
}
/// C: pthread.h (the darwin/glibc-2.12 spelling): `int pthread_setname_np (pthread_t, const char *);`
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_pthread_setname_np",
            not (feature = "have_pthread_setname_np_1arg"),
            not (feature = "have_pthread_setname_np_3arg")))]
unsafe extern "C" {
  #[link_name = "pthread_setname_np"]
  pub fn pthread_setname_np_2 (thread: pthread_t, name: *const c_char) -> c_int;
}
/// C: pthread.h (the BSD spelling): `void pthread_set_name_np (pthread_t, const char *);`
#[cfg (all (feature = "threads_enabled", feature = "have_pthread", feature = "have_pthread_set_name_np"))]
unsafe extern "C" {
  pub fn pthread_set_name_np (thread: pthread_t, name: *const c_char);
}

#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_set_name (name: *const c_char)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  #[cfg (feature = "have_pthread_setname_np")]
  {
    /* We need to truncate here otherwise pthread_setname_np fails to set the name.  TASK_COMM_LEN
       is what the length is called in the Linux kernel headers (Bug#38632).  */
    const TASK_COMM_LEN: usize = 16;
    let mut p_name: [c_char; TASK_COMM_LEN] = [0; TASK_COMM_LEN];
    strncpy (p_name.as_mut_ptr (), name, TASK_COMM_LEN - 1);
    p_name[TASK_COMM_LEN - 1] = 0;
    #[cfg (feature = "have_pthread_setname_np_1arg")]
    { pthread_setname_np_1 (p_name.as_ptr ()); }
    #[cfg (all (feature = "have_pthread_setname_np_3arg",
                not (feature = "have_pthread_setname_np_1arg")))]
    { pthread_setname_np_3 (pthread_self (), c"%s".as_ptr (), p_name.as_ptr ()); }
    #[cfg (all (not (feature = "have_pthread_setname_np_1arg"),
                not (feature = "have_pthread_setname_np_3arg")))]
    { pthread_setname_np_2 (pthread_self (), p_name.as_ptr ()); }
  }
  #[cfg (all (not (feature = "have_pthread_setname_np"), feature = "have_pthread_set_name_np"))]
  {
    /* The name will automatically be truncated if it exceeds a system-specific length.  */
    pthread_set_name_np (pthread_self (), name);
  }

  }
}

/* C 244-273: `sys_thread_create`.  C's `out:` label (C 268) and its one forward `goto out;`
   (C 262) become one labelled block (PORTING.md 2.2).  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_create (thread_ptr: *mut sys_thread_t,
                                           func: thread_creation_function,
                                           arg: *mut c_void) -> bool
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  /* C declares this uninitialized (`pthread_attr_t attr;`); Rust rejects a `&raw mut` borrow of
     an uninitialized binding (E0381), so the port zeroes the storage first - the first statement
     that reads it is the `pthread_attr_init` call below, which initializes it either way
     (docs/ports/systhread.md `## Deviations`).  */
  let mut attr: pthread_attr_t = core::mem::zeroed ();
  let mut result = false;

  if pthread_attr_init (&raw mut attr) != 0
    { return false; }

  /* Avoid crash on macOS with deeply nested GC (Bug#30364).  */
  /* C: `size_t stack_size;` - uninitialized until `pthread_attr_getstacksize` writes it; the port
     zeroes it for the same E0381 reason as `attr` above.  */
  let mut stack_size: usize = core::mem::zeroed ();
  let required_stack_size: usize = core::mem::size_of::<*mut c_void> () * 1024 * 1024;
  'out: {
    if pthread_attr_getstacksize (&raw const attr, &raw mut stack_size) == 0
      && stack_size < required_stack_size
      {
        if pthread_attr_setstacksize (&raw mut attr, required_stack_size) != 0
          { break 'out; }
      }

    if pthread_attr_setdetachstate (&raw mut attr, PTHREAD_CREATE_DETACHED) == 0
      { result = pthread_create (thread_ptr, &raw const attr, func, arg) == 0; }
  }
  let error = pthread_attr_destroy (&raw mut attr);
  eassert! (error == 0);

  return result;

  }
}

/* C 275-278: `sys_thread_yield`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_yield ()
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  sched_yield ();

  }
}

/* ------------------------------------------------------------------------------------------
   C 280-547: `#elif defined (WINDOWSNT)` - the critical-section arm.  **Text only on this
   machine** (no Windows toolchain: `which -a x86_64-w64-mingw32-gcc` is empty), but the arm does
   *type check* under its own features, which is the second reading `docs/ports/systhread.md`
   `## Evidence` records.  The mutex/cond records, the `CONDV_*` enum and the three typedefs come
   from the header mirror (`include/systhread.rs`, C 88-121); the `<windows.h>` surface the arm
   names is declared here, because no mirror in this tree carries it and this arm is its only
   consumer (P-88 clause 1).  Every width below is C's own spelling, not a measurement.
   ------------------------------------------------------------------------------------------ */

/// C: w32term.h: `#define ALIGN_STACK __attribute__ ((__force_align_arg_pointer__))` - a
/// declaration attribute with no Rust counterpart (PORTING.md 0.3), so it is a no-op macro and is
/// not written at the declaration site (the `NODISCARD` shape).
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
macro_rules! ALIGN_STACK { () => { () }; }

#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type LPCRITICAL_SECTION = *mut w32thread_critsect;   // C: `typedef CRITICAL_SECTION *LPCRITICAL_SECTION;`
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type HANDLE = *mut c_void;      // src/w32fns.rs 232's spelling
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type BOOL = c_int;              // src/w32fns.rs 222's spelling
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type DWORD = u32;               // src/w32fns.rs 212's spelling
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type DWORD_PTR = usize;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type ULONG_PTR = usize;         // src/w32fns.rs 429's spelling
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type UINT = c_uint;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type HRESULT = c_long;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type LPCSTR = *const c_char;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type LPWSTR = *mut c_ushort;    // C's `wchar_t *` on Windows is 16-bit: src/w32.rs 1413-1414
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type PCWSTR = *const c_ushort;  // ... the same reading for the `const wchar_t *`
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub const FALSE: BOOL = 0;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub const TRUE: BOOL = 1;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub const INFINITE: DWORD = 0xFFFFFFFF;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub const WAIT_OBJECT_0: DWORD = 0;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub const S_OK: HRESULT = 0;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub const MB_ERR_INVALID_CHARS: DWORD = 0x00000008;
/// C 425-427: `#define MS_VC_EXCEPTION 0x406d1388UL`
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub const MS_VC_EXCEPTION: DWORD = 0x406d1388;

/* C 282-283's `<mbctype.h>` and `"w32term.h"` entry points, plus C 287's `_beginthread`, which the
   file declares itself (`<process.h>` cannot be included - C's own comment).  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
unsafe extern "C" {
  /* LPCWSTR/LPARAM-free subset: exactly the calls C 293-547 makes.  */
  pub fn InitializeCriticalSection (critical_section: LPCRITICAL_SECTION);
  pub fn EnterCriticalSection (critical_section: LPCRITICAL_SECTION);
  pub fn LeaveCriticalSection (critical_section: LPCRITICAL_SECTION);
  pub fn DeleteCriticalSection (critical_section: LPCRITICAL_SECTION);
  pub fn CreateEvent (attributes: *mut c_void, manual_reset: BOOL, initial_state: BOOL,
                      name: LPCSTR) -> HANDLE;
  pub fn WaitForMultipleObjects (count: DWORD, handles: *const HANDLE, wait_all: BOOL,
                                 milliseconds: DWORD) -> DWORD;
  pub fn SetEvent (event: HANDLE) -> BOOL;
  pub fn ResetEvent (event: HANDLE) -> BOOL;
  pub fn CloseHandle (object: HANDLE) -> BOOL;
  pub fn GetCurrentThreadId () -> DWORD;
  pub fn GetCurrentThread () -> HANDLE;
  pub fn Sleep (milliseconds: DWORD);
  pub fn RaiseException (code: DWORD, flags: DWORD, nargs: DWORD, args: *const ULONG_PTR);
  pub fn GetACP () -> UINT;
  /// C: mbctype.h: `int _getmbcp (void);`
  pub fn _getmbcp () -> c_int;
  /// C 287: `uintptr_t _beginthread (void (__cdecl *)(void *), unsigned, void *);`
  pub fn _beginthread (start: Option<unsafe extern "C" fn (*mut c_void)>, stack_size: c_uint,
                       arg: *mut c_void) -> usize;
}

/* C 293-297: `sys_mutex_init` - mutexes are critical sections here.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_mutex_init (r#mutex: *mut sys_mutex_t)
{
  InitializeCriticalSection (r#mutex as LPCRITICAL_SECTION);
}

/* C 299-305: `sys_mutex_lock`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_mutex_lock (r#mutex: *mut sys_mutex_t)
{
  /* FIXME: What happens if the owning thread exits without releasing the mutex?  According to
     MSDN, the result is undefined behavior.  */
  EnterCriticalSection (r#mutex as LPCRITICAL_SECTION);
}

/* C 307-311: `sys_mutex_unlock`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_mutex_unlock (r#mutex: *mut sys_mutex_t)
{
  LeaveCriticalSection (r#mutex as LPCRITICAL_SECTION);
}

/* C 313-326: `sys_cond_init`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_init (cond: *mut sys_cond_t)
{
  (*cond).initialized = false;
  (*cond).wait_count = 0;
  /* Auto-reset event for signal.  */
  (*cond).events[CONDV_SIGNAL as usize] =
    CreateEvent (core::ptr::null_mut (), FALSE, FALSE, core::ptr::null ());
  /* Manual-reset event for broadcast.  */
  (*cond).events[CONDV_BROADCAST as usize] =
    CreateEvent (core::ptr::null_mut (), TRUE, FALSE, core::ptr::null ());
  if (*cond).events[CONDV_SIGNAL as usize].is_null ()
    || (*cond).events[CONDV_BROADCAST as usize].is_null ()
    { return; }
  InitializeCriticalSection (&raw mut (*cond).wait_count_lock as LPCRITICAL_SECTION);
  (*cond).initialized = true;
}

/* C 328-363: `sys_cond_wait`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_wait (cond: *mut sys_cond_t, r#mutex: *mut sys_mutex_t)
{
  let wait_result: DWORD;
  let mut last_thread_waiting: bool;

  if !(*cond).initialized
    { return; }

  /* Increment the wait count avoiding race conditions.  */
  EnterCriticalSection (&raw mut (*cond).wait_count_lock as LPCRITICAL_SECTION);
  (*cond).wait_count += 1;
  LeaveCriticalSection (&raw mut (*cond).wait_count_lock as LPCRITICAL_SECTION);

  /* Release the mutex and wait for either the signal or the broadcast event.  */
  LeaveCriticalSection (r#mutex as LPCRITICAL_SECTION);
  wait_result = WaitForMultipleObjects (2, (*cond).events.as_ptr (), FALSE, INFINITE);

  /* Decrement the wait count and see if we are the last thread waiting on the condition
     variable.  */
  EnterCriticalSection (&raw mut (*cond).wait_count_lock as LPCRITICAL_SECTION);
  (*cond).wait_count -= 1;
  last_thread_waiting =
    wait_result == WAIT_OBJECT_0 + CONDV_BROADCAST as DWORD
    && (*cond).wait_count == 0;
  LeaveCriticalSection (&raw mut (*cond).wait_count_lock as LPCRITICAL_SECTION);

  /* Broadcast uses a manual-reset event, so when the last thread is released, we must manually
     reset that event.  */
  if last_thread_waiting
    { ResetEvent ((*cond).events[CONDV_BROADCAST as usize]); }

  /* Per the API, re-acquire the mutex.  */
  EnterCriticalSection (r#mutex as LPCRITICAL_SECTION);
}

/* C 365-379: `sys_cond_signal`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_signal (cond: *mut sys_cond_t)
{
  let mut threads_waiting: bool;

  if !(*cond).initialized
    { return; }

  EnterCriticalSection (&raw mut (*cond).wait_count_lock as LPCRITICAL_SECTION);
  threads_waiting = (*cond).wait_count > 0;
  LeaveCriticalSection (&raw mut (*cond).wait_count_lock as LPCRITICAL_SECTION);

  if threads_waiting
    { SetEvent ((*cond).events[CONDV_SIGNAL as usize]); }
}

/* C 381-395: `sys_cond_broadcast`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_broadcast (cond: *mut sys_cond_t)
{
  let mut threads_waiting: bool;

  if !(*cond).initialized
    { return; }

  EnterCriticalSection (&raw mut (*cond).wait_count_lock as LPCRITICAL_SECTION);
  threads_waiting = (*cond).wait_count > 0;
  LeaveCriticalSection (&raw mut (*cond).wait_count_lock as LPCRITICAL_SECTION);

  if threads_waiting
    { SetEvent ((*cond).events[CONDV_BROADCAST as usize]); }
}

/* C 397-411: `sys_cond_destroy`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_cond_destroy (cond: *mut sys_cond_t)
{
  if !(*cond).events[CONDV_SIGNAL as usize].is_null ()
    { CloseHandle ((*cond).events[CONDV_SIGNAL as usize]); }
  if !(*cond).events[CONDV_BROADCAST as usize].is_null ()
    { CloseHandle ((*cond).events[CONDV_BROADCAST as usize]); }

  if !(*cond).initialized
    { return; }

  /* FIXME: What if wait_count is non-zero, i.e. there are still threads waiting on this condition
     variable?  */
  DeleteCriticalSection (&raw mut (*cond).wait_count_lock as LPCRITICAL_SECTION);
}

/* C 413-417: `sys_thread_self`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_self () -> sys_thread_t
{
  return GetCurrentThreadId () as sys_thread_t;
}

/* C 419-423: `sys_thread_equal`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_equal (t: sys_thread_t, u: sys_thread_t) -> bool
{
  return t == u;
}

/* C 429-436: `typedef struct _THREADNAME_INFO { … } THREADNAME_INFO;` - the record used to
   communicate the thread name to a debugger.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[repr (C)]
pub struct THREADNAME_INFO
{
  pub r#type: DWORD_PTR,     // C 432: `DWORD_PTR  type;`
  pub name: LPCSTR,          // C 433: `LPCSTR name;`
  pub thread_id: DWORD_PTR,  // C 434: `DWORD_PTR  thread_id;`
  pub reserved: DWORD_PTR,   // C 435: `DWORD_PTR  reserved;`
}

/* C 438-443: the three function pointers `w32term.c` defines.  C 438/441-442 spell the two
   `typedef`s; C 439/440/443 declare the objects (`is_debugger_present` may be NULL, which C 471
   tests for, so it is an `Option` here; the other two are called only inside the branch that
   proves they were loaded).  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type IsDebuggerPresent_Proc = Option<unsafe extern "system" fn () -> BOOL>;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type SetThreadDescription_Proc = Option<unsafe extern "system" fn (thread: HANDLE,
                                                                     description: PCWSTR) -> HRESULT>;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub type MultiByteToWideChar_Proc = unsafe extern "system" fn (codepage: UINT, flags: DWORD,
                                                              input: LPCSTR, input_len: c_int,
                                                              output: LPWSTR,
                                                              output_len: c_int) -> c_int;
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
unsafe extern "C" {
  pub static is_debugger_present: VmGlobal<IsDebuggerPresent_Proc>;
  pub static pMultiByteToWideChar: VmGlobal<MultiByteToWideChar_Proc>;
  pub static set_thread_description: VmGlobal<SetThreadDescription_Proc>;
}

/* C 446-482: `w32_set_thread_name` (static).  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
unsafe fn w32_set_thread_name (thread_id: DWORD, name: *const c_char)
{
  if name.is_null () || *name == 0
    { return; }

  /* Use the new API provided since Windows 10, if available.  */
  if let Some (set_description) = (*set_thread_description.as_mut_ptr ())
    {
      /* GDB pulls only the first 1024 characters of thread's name.  */
      let mut name_w: [c_ushort; 1025] = [0; 1025];
      /* The thread name is encoded in locale's encoding, but SetThreadDescription wants a
         `wchar_t` string.  */
      let mut codepage: c_int = _getmbcp ();
      if codepage == 0
        { codepage = GetACP () as c_int; }
      let cnv_result = (*pMultiByteToWideChar.as_mut_ptr ()) (codepage as UINT, MB_ERR_INVALID_CHARS, name, -1,
                                             name_w.as_mut_ptr (), 1025);
      if cnv_result != 0
        && set_description (GetCurrentThread (), name_w.as_ptr ()) == S_OK
        { return; }
    }
  /* We can only support this fallback method when Emacs is being debugged.  */
  let debugged = match (*is_debugger_present.as_mut_ptr ())
    {
      Some (f) => f () != 0,
      None => false,
    };
  if !debugged
    { return; }

  let mut tninfo: THREADNAME_INFO = core::mem::zeroed ();   // C declares it uninitialized (E0381)

  tninfo.r#type = 0x1000;	/* magic constant */
  tninfo.name = name;
  tninfo.thread_id = thread_id as DWORD_PTR;   // C: `DWORD` widens to the record's `DWORD_PTR`
  tninfo.reserved = 0;
  RaiseException (MS_VC_EXCEPTION, 0,
                  (core::mem::size_of::<THREADNAME_INFO> () / core::mem::size_of::<ULONG_PTR> ())
                  as DWORD,
                  &raw const tninfo as *const ULONG_PTR);
}

/* C 484: `static thread_creation_function *thread_start_address;`  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
pub static thread_start_address: VmGlobal<thread_creation_function> = VmGlobal::<thread_creation_function>::new (None);

/* C 486-490: `sys_thread_set_name`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_set_name (name: *const c_char)
{
  w32_set_thread_name (GetCurrentThreadId (), name);
}

/* C 495-499: `w32_beginthread_wrapper` (static).  C's `ALIGN_STACK` is a declaration attribute
   with no Rust counterpart, so it is the no-op macro above and is not written here.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
unsafe extern "C" fn w32_beginthread_wrapper (arg: *mut c_void)
{
  match (*thread_start_address.as_mut_ptr ())
    {
      Some (f) => { f (arg); }
      None => {}
    }
}

/* C 501-541: `sys_thread_create`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_create (thread_ptr: *mut sys_thread_t,
                                           func: thread_creation_function,
                                           arg: *mut c_void) -> bool
{
  /* FIXME: Do threads that run Lisp require some minimum amount of stack?  Zero here means each
     thread will get the same amount as the main program.  On GNU/Linux, it seems like the stack
     is 2MB by default, overridden by RLIMIT_STACK at program start time.  Not sure what to do
     with this.  See also the comment in w32proc.c:new_child.  */
  const stack_size: c_uint = 0;
  let thandle: usize;

  (*thread_start_address.as_mut_ptr ()) = func;

  /* We use `_beginthread` rather than `CreateThread` because the former arranges for the thread
     handle to be automatically closed when the thread exits, thus preventing handle leaks and/or
     the need to track all the threads and close their handles when they exit.  Also, MSDN seems
     to imply that code which uses CRT _must_ call `_beginthread`, although if that is true, we
     already violate that rule in many places...  */
  thandle = _beginthread (Some (w32_beginthread_wrapper), stack_size, arg);
  if thandle == usize::MAX    /* C: `(uintptr_t) -1L` */
    { return false; }

  /* Kludge alert!  We use the Windows thread ID, an unsigned 32-bit number, as the `sys_thread_t`
     type, because that ID is the only unique identifier of a thread on Windows.  But
     `_beginthread` returns a handle of the thread, and there's no easy way of getting the thread
     ID given a handle (`GetThreadId` is available only since Vista, so we cannot use it
     portably).  Fortunately, the value returned by `sys_thread_create` is not used by its
     callers; instead, `run_thread`, which runs in the context of the new thread, calls
     `sys_thread_self` and uses its return value; `sys_thread_self` in this implementation calls
     `GetCurrentThreadId`.  Therefore, we return some more or less arbitrary value of the thread
     ID from this function.  */
  *thread_ptr = (thandle & 0xFFFFFFFF) as sys_thread_t;
  return true;
}

/* C 543-547: `sys_thread_yield`.  */
#[cfg (all (feature = "threads_enabled", not (feature = "have_pthread"), feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn sys_thread_yield ()
{
  Sleep (0);
}
