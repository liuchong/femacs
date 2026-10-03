// femacs: mirror of src/systhread.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): the `SYSTHREAD_H` guard, the three dummy thread types this
// target selects, the `thread_creation_function` typedef and the thirteen prototypes.
//
// The frozen configuration has `THREADS_ENABLED` undefined
// (docs/baseline/config/darwin-arm64.macros.tsv 3678), so C 130-137's `#else` arm is the live
// text and everything in C 24-128 is the threaded build's: the `#include <pthread.h>`, the
// `SYSTHREAD_ALIGN_ROOM`/`SYSTHREAD_ALIGN_PTR` pair with its `ALIGNOF_PTHREAD_{COND,MUTEX}_T`
// comparison, the `pthread_mutex_t`/`pthread_cond_t`/`pthread_t` typedefs, the WINDOWSNT
// `w32thread_critsect`/`w32thread_cond_t` records and the `#error port me` fallback.  None of
// them is transcribed; docs/ports/include-systhread.md records the whole region.
//
// The prototypes live in `systhread_decls` + glob import (the tempname.rs shape), because
// src/systhread.c's port defines every one of them and an explicit item shadows a glob import
// (PORTING.md 0.1-10).

// C 19-20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const SYSTHREAD_H: bool = true;

include!("../../lib/include/attribute.rs");   // C 22: `#include <attribute.h>` (`NODISCARD`)

/* For the no-threads case we can simply use dummy definitions.  */
pub type sys_mutex_t = c_int;   // C 133
pub type sys_cond_t = c_int;    // C 134
pub type sys_thread_t = c_int;  // C 135

// C 139: `typedef void *(thread_creation_function) (void *);` - a function *type* in C, which
// its users pass as `thread_creation_function *`; the port names the function-pointer type
// the way the atimer.h mirror names `atimer_callback` (P-27), and call sites write it bare.
pub type thread_creation_function
  = Option<unsafe extern "C" fn (arg: *mut c_void) -> *mut c_void>;

pub mod systhread_decls {
  use super::*;

  unsafe extern "C" {
    pub fn sys_mutex_init (sys_mutex: *mut sys_mutex_t);                        // C 141
    pub fn sys_mutex_lock (sys_mutex: *mut sys_mutex_t);                        // C 142
    pub fn sys_mutex_unlock (sys_mutex: *mut sys_mutex_t);                      // C 143
    pub fn sys_cond_init (c: *mut sys_cond_t);                                  // C 145
    pub fn sys_cond_wait (c: *mut sys_cond_t, m: *mut sys_mutex_t);             // C 146
    pub fn sys_cond_signal (c: *mut sys_cond_t);                                // C 147
    pub fn sys_cond_broadcast (c: *mut sys_cond_t);                             // C 148
    pub fn sys_cond_destroy (c: *mut sys_cond_t);                               // C 149
    // C 151-153/155-156 carry NODISCARD in front of the three declarations below; the token
    // has no Rust form (PORTING.md 0.3, attribute.h 187), so it is the no-op macro defined in
    // attribute.rs and not written at the declaration site.
    pub fn sys_thread_self () -> sys_thread_t;                                  // C 151
    pub fn sys_thread_equal (t: sys_thread_t, u: sys_thread_t) -> bool;         // C 152
    pub fn sys_thread_create (t: *mut sys_thread_t, func: thread_creation_function,
                              datum: *mut c_void) -> bool;                      // C 154-155
    pub fn sys_thread_yield ();                                                 // C 157
    pub fn sys_thread_set_name (name: *const c_char);                           // C 158
  }
}
use systhread_decls::*;
