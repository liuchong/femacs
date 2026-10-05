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

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, unused_mut,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

include!("include/config.rs");
include!("../lib/include/string.rs");	// lisp.h 30: <string.h>
include!("include/lisp.rs");
include!("include/systhread.rs");

// C 13-14's `<setjmp.h>` and `<stdio.h>` come through lisp.h's own chain (lisp.h includes
// both), so the port's include list is C's minus the two duplicates (PORTING.md 0.1-4).

/* C 33's `#ifndef THREADS_ENABLED` arm (live: the macro is undefined on this target).  */

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_mutex_init (m: *mut sys_mutex_t)
{
  *m = 0;
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_mutex_lock (m: *mut sys_mutex_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_mutex_unlock (m: *mut sys_mutex_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_init (c: *mut sys_cond_t)
{
  *c = 0;
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_wait (c: *mut sys_cond_t, m: *mut sys_mutex_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_signal (c: *mut sys_cond_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_broadcast (c: *mut sys_cond_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_destroy (c: *mut sys_cond_t)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_self () -> sys_thread_t
{
  return 0;
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_equal (t: sys_thread_t, u: sys_thread_t) -> bool
{
  return t == u;
}
#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_set_name (name: *const c_char)
{
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_create (t: *mut sys_thread_t,
                                            func: thread_creation_function,
                                            datum: *mut c_void) -> bool
{
  return false;
}

#[cfg (not (feature = "threads_enabled"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_yield ()
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
#[derive (Copy, Clone)]
#[repr (C, align (8))]
pub struct pthread_mutex_t { pub __opaque: [c_uchar; 64] }       // clang: sizeof 64, align 8
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[derive (Copy, Clone)]
#[repr (C, align (8))]
pub struct pthread_cond_t { pub __opaque: [c_uchar; 48] }        // clang: sizeof 48, align 8
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[derive (Copy, Clone)]
#[repr (C, align (8))]
pub struct pthread_attr_t { pub __opaque: [c_uchar; 64] }        // clang: sizeof 64
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[derive (Copy, Clone)]
#[repr (C, align (8))]
pub struct pthread_mutexattr_t { pub __opaque: [c_uchar; 16] }   // clang: sizeof 16
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
pub type pthread_t = *mut c_void;                                // clang: sizeof 8 (an opaque pointer)
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
pub const PTHREAD_MUTEX_ERRORCHECK: c_int = 1;                   // clang: 1
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
pub const PTHREAD_CREATE_DETACHED: c_int = 2;                    // clang: 2

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
unsafe extern "C" {
  pub fn ns_run_loop_break ();
}

/* C 108-136: `sys_mutex_init`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_mutex_init (sys_mutex: *mut sys_mutex_t)
{
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
      fprintf (stderr, c"\npthread_mutex_init failed: %s\n".as_ptr (), strerror (error));
      emacs_abort ();
    }
  #[cfg (feature = "enable_checking")]
  { error = pthread_mutexattr_destroy (&raw mut attr); eassert! (error == 0); }
}

/* C 138-145: `sys_mutex_lock`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_mutex_lock (sys_mutex: *mut sys_mutex_t)
{
  let mutex: *mut pthread_mutex_t = SYSTHREAD_ALIGN_PTR! (pthread_mutex_t, sys_mutex);
  let error = pthread_mutex_lock (mutex);
  eassert! (error == 0);
}

/* C 147-152: `sys_mutex_unlock`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_mutex_unlock (sys_mutex: *mut sys_mutex_t)
{
  let mutex: *mut pthread_mutex_t = SYSTHREAD_ALIGN_PTR! (pthread_mutex_t, sys_mutex);
  let error = pthread_mutex_unlock (mutex);
  eassert! (error == 0);
}

/* C 154-166: `sys_cond_init`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_init (sys_cond: *mut sys_cond_t)
{
  let cond: *mut pthread_cond_t = SYSTHREAD_ALIGN_PTR! (pthread_cond_t, sys_cond);
  let error = pthread_cond_init (cond, core::ptr::null ());
  /* We could get ENOMEM.  Can't do anything except aborting.  */
  if error != 0
    {
      fprintf (stderr, c"\npthread_cond_init failed: %s\n".as_ptr (), strerror (error));
      emacs_abort ();
    }
}

/* C 168-175: `sys_cond_wait`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_wait (sys_cond: *mut sys_cond_t, sys_mutex: *mut sys_mutex_t)
{
  let cond: *mut pthread_cond_t = SYSTHREAD_ALIGN_PTR! (pthread_cond_t, sys_cond);
  let mutex: *mut pthread_mutex_t = SYSTHREAD_ALIGN_PTR! (pthread_mutex_t, sys_mutex);
  let error = pthread_cond_wait (cond, mutex);
  eassert! (error == 0);
}

/* C 177-183: `sys_cond_signal`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_signal (sys_cond: *mut sys_cond_t)
{
  let cond: *mut pthread_cond_t = SYSTHREAD_ALIGN_PTR! (pthread_cond_t, sys_cond);
  let error = pthread_cond_signal (cond);
  eassert! (error == 0);
}

/* C 185-199: `sys_cond_broadcast`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_broadcast (sys_cond: *mut sys_cond_t)
{
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

/* C 201-206: `sys_cond_destroy`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_destroy (sys_cond: *mut sys_cond_t)
{
  let cond: *mut pthread_cond_t = SYSTHREAD_ALIGN_PTR! (pthread_cond_t, sys_cond);
  let error = pthread_cond_destroy (cond);
  eassert! (error == 0);
}

/* C 208-212: `sys_thread_self`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_self () -> sys_thread_t
{
  return pthread_self ();
}

/* C 214-218: `sys_thread_equal`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_equal (t: sys_thread_t, u: sys_thread_t) -> bool
{
  return pthread_equal (t, u) != 0;
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
pub unsafe extern "C" fn sys_thread_set_name (name: *const c_char)
{
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

/* C 244-273: `sys_thread_create`.  C's `out:` label (C 268) and its one forward `goto out;`
   (C 262) become one labelled block (PORTING.md 2.2).  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_create (thread_ptr: *mut sys_thread_t,
                                           func: thread_creation_function,
                                           arg: *mut c_void) -> bool
{
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

/* C 275-278: `sys_thread_yield`.  */
#[cfg (all (feature = "threads_enabled", feature = "have_pthread"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_yield ()
{
  sched_yield ();
}
