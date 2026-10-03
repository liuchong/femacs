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
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* System thread definitions
Copyright (C) 2012-2026 Free Software Foundation, Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

include!("include/config.rs");
include!("../lib/include/string.rs");	// lisp.h 30: <string.h>
include!("include/lisp.rs");
include!("include/systhread.rs");

// C 13-14's `<setjmp.h>` and `<stdio.h>` come through lisp.h's own chain (lisp.h includes
// both), so the port's include list is C's minus the two duplicates (PORTING.md 0.1-4).

/* C 33's `#ifndef THREADS_ENABLED` arm (live: the macro is undefined on this target).  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_mutex_init (m: *mut sys_mutex_t)
{
  *m = 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_mutex_lock (m: *mut sys_mutex_t)
{
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_mutex_unlock (m: *mut sys_mutex_t)
{
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_init (c: *mut sys_cond_t)
{
  *c = 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_wait (c: *mut sys_cond_t, m: *mut sys_mutex_t)
{
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_signal (c: *mut sys_cond_t)
{
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_broadcast (c: *mut sys_cond_t)
{
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_cond_destroy (c: *mut sys_cond_t)
{
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_self () -> sys_thread_t
{
  return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_equal (t: sys_thread_t, u: sys_thread_t) -> bool
{
  return t == u;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_set_name (name: *const c_char)
{
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_create (t: *mut sys_thread_t,
                                            func: thread_creation_function,
                                            datum: *mut c_void) -> bool
{
  return false;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_thread_yield ()
{
}
