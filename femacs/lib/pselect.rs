/* pselect - synchronous I/O multiplexing

   Copyright 2011-2026 Free Software Foundation, Inc.

   This file is part of gnulib.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// femacs: port of lib/pselect.c @ fd7afced403 (GNU Emacs 32.0.50).  The file is in
// cond-eval-verified.txt and `HAVE_PSELECT` is 1 (macros.tsv 1469), so C 36-85's
// `#if !HAVE_PSELECT` arm (the `pselect` emulation built from `select` and
// `pthread_sigmask`) is dead and C 87-115's `#else` arm is the live text; the dead arm needs
// no Rust shape here (P-75 drops its tokens).
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");        // <errno.h>
include!("include/signal.rs");       // <signal.h> (`sigset_t`)
include!("include/sys_select.rs");   // <sys/select.h> (`fd_set`, `FD_SETSIZE`, `FD_ISSET`)
include!("include/time.in.rs");      // <time.h> (`struct timespec`)
include!("include/unistd.in.rs");    // <unistd.h> (`dup2`)

/* Examine the size-NFDS file descriptor sets in RFDS, WFDS, and XFDS
   to see whether some of their descriptors are ready for reading,
   ready for writing, or have exceptions pending.  Wait for at most
   TIMEOUT seconds, and use signal mask SIGMASK while waiting.  A null
   pointer parameter stands for no descriptors, an infinite timeout,
   or an unaffected signal mask.  */

/* C 36-85's `#if !HAVE_PSELECT` arm is dead: it defines the `pselect` emulation whose
   body allocates a `struct timeval` from the timeout, munges the signal mask with
   `pthread_sigmask (SIG_SETMASK, …)` and calls `select`.  Its tokens are not required
   (P-75).  C 88-89's `<unistd.h>` include and `# undef pselect` are live.  */

/* int rpl_pselect (int nfds, fd_set *restrict rfds, fd_set *restrict wfds,
                    fd_set *restrict xfds, struct timespec const *restrict timeout,
                    sigset_t const *restrict sigmask)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_pselect (nfds: c_int, rfds: *mut fd_set, wfds: *mut fd_set,
                                      xfds: *mut fd_set, timeout: *const timespec,
                                      sigmask: *const sigset_t) -> c_int
{
  /* FreeBSD 8.2 has a bug: it does not always detect invalid fds.  */
  if nfds < 0 || nfds > FD_SETSIZE
    {
      *errno () = EINVAL;
      return -1;
    }
  /* C 103's `for (int i = 0; i < nfds; i++)`.  */
  let mut i: c_int = 0;
  while i < nfds
    {
      if ((! rfds.is_null () && FD_ISSET (i, rfds))
          || (! wfds.is_null () && FD_ISSET (i, wfds))
          || (! xfds.is_null () && FD_ISSET (i, xfds)))
        && dup2 (i, i) != i
        { return -1; }
      i += 1;
    }

  /* C 112: the *system* `pselect` (C 89's `# undef pselect` makes the call resolve to it,
     since the definition above carries the renamed name gnulib's <sys/select.h> gives it).  */
  return pselect (nfds, rfds, wfds, xfds, timeout, sigmask);
}
