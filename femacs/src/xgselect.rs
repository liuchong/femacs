// femacs: port of src/xgselect.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// file-scope object, local), the same control flow and the same error model.  The non-`static` C
// definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); `acquire_select_lock` stays a plain
// `unsafe fn`.
//
// The file gate is `feature = "have_glib"` (C 24's `#ifdef HAVE_GLIB`, on every item *and on C 26-31's
// include graph*, P-86 ⑧).  Three nested conditions are carried with both arms (P-86 ①): C 42/54's
// `#if GNUC_PREREQ (4, 7, 0)` (the port's `GNUC_PREREQ!`, the `include/conf_post.rs` macro, so the
// predicate is the frozen compiler's own), C 119/129's `#ifdef USE_GTK`, and C 131's
// `#ifndef HAVE_PGTK`; C 267's `#ifdef HAVE_XWIDGETS` adds `catch_child_signal`.
//
// **`__atomic_*` (C 43/55) follow PORTING.md §6**: the same object becomes a
// `core::sync::atomic::AtomicIsize` and the call gets the same ordering.  This file is one of the two
// places §6 names as *mixed* (C 46's plain `--` reads the same variable C 43 accesses atomically), and
// Rust has one kind per object, so the `#else` arm's `++`/`--` are that object's atomic ops with
// `SeqCst` - recorded as a Deviation, which is what §6's "逐点核对并记录" asks for.
//
// C 26's `<glib.h>` has no mirror of its own: its names live in `femacs/src/include/gtk.rs` (P-90 ②,
// the GLib surface's one home), which is why that edge is spelled here - the plane
// `docs/ports/include-gtk.md` records.  No GLib tree exists on this machine, so those declarations are
// UNVERIFIED placeholders and the arm flip is a name-and-type reading, not a build against GLib.
// `x_gtk_use_native_input` (C 132) is `xterm.h`'s; C reaches it through `process.h`'s chain and the
// mirror set spells that as its own edge (the `src/msdos.c` C 36 reading).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, clippy::all)]

/* This unit's own `use std::ffi::{c_char, c_int, c_void};` was removed at 3.477: `include/lisp.rs`
   14-15 imports exactly those names at this scope, so spelling them twice is `E0252` on the arm
   flip (`c_char`, `c_int`, `c_void` were the first three errors the GLib flip reported).  The
   names this file uses outside that include come from `include/xgselect.rs`'s own module.  */

include!("include/config.rs");        // C 20: `#include <config.h>`
/* config.h's AH_BOTTOM `#include <conf_post.h>` (the `src/emacs.rs` C 22 reading): `GNUC_PREREQ`
   is that mirror's macro (`include/conf_post.rs` 50), and C 42/54's two `#if GNUC_PREREQ (4, 7, 0)`
   arms are spelled with it.  Ungated, because config.h is outside `#ifdef HAVE_GLIB` in C too.  */
include!("include/conf_post.rs");
include!("include/xgselect.rs");      // C 22: `#include "xgselect.h"`

#[cfg (feature = "have_glib")]
include!("include/gtk.rs");           // C 26: `#include <glib.h>` (the GLib surface's home, P-90 ②)
#[cfg (feature = "have_glib")]
include!("../lib/include/errno.rs");  // C 27: `#include <errno.h>`
#[cfg (feature = "have_glib")]
include!("include/lisp.rs");          // C 28: `#include "lisp.h"`
/* lisp.h 28's `#include <stdcountof.h>`: C 115's `countof (gfds_buf)` is the only use.  The
   mirror is `lib/include/stdcountof.rs` (the live C23 definition on the frozen target); lisp.rs
   carries neither it nor `<stdint.h>`, so the edge is spelled here (the src/emacs.rs 130 and
   src/editfns.rs 62 readings).  */
#[cfg (feature = "have_glib")]
include!("../lib/include/stdcountof.rs");
#[cfg (feature = "have_glib")]
include!("include/blockinput.rs");    // C 29: `#include "blockinput.h"`
#[cfg (feature = "have_glib")]
include!("include/systime.rs");       // C 30: `#include "systime.h"`
#[cfg (feature = "have_glib")]
include!("include/process.rs");       // C 31: `#include "process.h"`
/* C 31's chain: process.h 22-24's `#ifdef HAVE_SYS_TYPES_H` `#include <sys/types.h>` is where
   `pid_t` comes from - process.h 134 is the field that names it - and the mirror is
   `lib/include/sys_types.in.rs`, the edge the `src/callproc.rs` 63 reading spells.  */
#[cfg (feature = "have_glib")]
include!("../lib/include/sys_types.in.rs");
/* C 31's chain: `thread_select` is `thread.h`'s (the mirror set spells it as its own edge, the
   `src/msdos.c` C 36 reading), and C 132's `x_gtk_use_native_input` is `xterm.h`'s.  */
#[cfg (feature = "have_glib")]
include!("include/thread.rs");
#[cfg (all (feature = "have_glib", feature = "use_gtk"))]
include!("include/xterm.rs");

// C 33: `static ptrdiff_t threads_holding_glib_lock;` - an `AtomicIsize` because C 43/55 access it
// atomically (PORTING.md §6; the `#else` arm's plain accesses are the same object's atomic ops,
// Deviation 2).
#[cfg (feature = "have_glib")]
static mut threads_holding_glib_lock: core::sync::atomic::AtomicIsize
  = core::sync::atomic::AtomicIsize::new (0);
// C 34: `static GMainContext *glib_main_context;`
#[cfg (feature = "have_glib")]
static mut glib_main_context: *mut GMainContext = core::ptr::null_mut ();
// C 37: `static int xg_select_suppress_count;`
#[cfg (feature = "have_glib")]
static mut xg_select_suppress_count: c_int = 0;

// C 40-49: `void release_select_lock (void)`
#[cfg (feature = "have_glib")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn release_select_lock ()
{
  use core::sync::atomic::Ordering;
  /* C 42-48: `#if GNUC_PREREQ (4, 7, 0)` - the atomic arm and the plain one, both carried; C's
     `__atomic_sub_fetch` returns the *new* value, so "== 0" is Rust's `fetch_sub` returning 1
     (Rust returns the previous value).  */
  if GNUC_PREREQ! (4, 7, 0)
    {
      if threads_holding_glib_lock.fetch_sub (1, Ordering::AcqRel) == 1
        {
          g_main_context_release (glib_main_context);
        }
    }
  else
    {
      if threads_holding_glib_lock.fetch_sub (1, Ordering::SeqCst) == 1
        {
          g_main_context_release (glib_main_context);
        }
    }
}

// C 51-73: `static void acquire_select_lock (GMainContext *context)`
#[cfg (feature = "have_glib")]
unsafe fn acquire_select_lock (context: *mut GMainContext)
{
  use core::sync::atomic::Ordering;
  /* C 54-72: the two `GNUC_PREREQ` arms, both carried (the `__atomic_fetch_add` of C 55 returns the
     *previous* value, which is Rust's `fetch_add`, so `== 0` is spelled the same way).  */
  if GNUC_PREREQ! (4, 7, 0)
    {
      if threads_holding_glib_lock.fetch_add (1, Ordering::AcqRel) == 0
        {
          glib_main_context = context;
          // C 58-62: `while (!g_main_context_acquire (context)) { /* Spin. */ }`
          while g_main_context_acquire (context) == 0
            {
              /* Spin.  */
            }
        }
    }
  else
    {
      if threads_holding_glib_lock.fetch_add (1, Ordering::SeqCst) == 0
        {
          glib_main_context = context;
          while g_main_context_acquire (context) == 0
            {
              /* Spin.  */
            }
        }
    }
}

/* C 75-81: `void suppress_xg_select (void)` - "call this to not use xg_select when using it would be
   a bad idea, i.e. during drag-and-drop".  */
#[cfg (feature = "have_glib")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn suppress_xg_select ()
{
  xg_select_suppress_count += 1;
}

// C 83-90: `void release_xg_select (void)`
#[cfg (feature = "have_glib")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn release_xg_select ()
{
  if xg_select_suppress_count == 0
    {
      emacs_abort ();
    }

  xg_select_suppress_count -= 1;
}

// C 103-294: `int xg_select (int fds_lim, fd_set *rfds, fd_set *wfds, fd_set *efds,
//            struct timespec *timeout, sigset_t *sigmask)`
#[cfg (feature = "have_glib")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_select (mut fds_lim: c_int, rfds: *mut fd_set, wfds: *mut fd_set,
                                    efds: *mut fd_set, timeout: *mut timespec,
                                    sigmask: *mut sigset_t) -> c_int
{
  // C 107-118: the locals.  PORTING.md 1.5: `all_rfds`/`all_wfds`/`tmo` are written before their
  // first read on every path; `gfds` starts at the stack buffer.
  let mut all_rfds: fd_set = core::mem::zeroed ();                       /* C 107 */
  let mut all_wfds: fd_set = core::mem::zeroed ();                       /* C 107 */
  let mut tmo: timespec = core::mem::zeroed ();                          /* C 108 */
  let mut tmop: *mut timespec = timeout;                                 /* C 109 */

  let mut context: *mut GMainContext;                                     /* C 111 */
  let mut have_wfds: bool = !wfds.is_null ();                             /* C 112 */
  let mut gfds_buf: [GPollFD; 128] = [GPollFD { fd: 0, events: 0, revents: 0 }; 128];  /* C 113 */
  let mut gfds: *mut GPollFD = gfds_buf.as_mut_ptr ();                    /* C 114 */
  let mut gfds_size: c_int = countof! (gfds_buf) as c_int;                /* C 115 */
  let mut n_gfds: c_int;                                                  /* C 116 */
  let mut retval: c_int = 0;                                              /* C 116 */
  let mut our_fds: c_int = 0;                                             /* C 116 */
  let mut max_fds: c_int = fds_lim - 1;                                   /* C 116 */
  let mut i: c_int;                                                       /* C 117 */
  let mut nfds: c_int;                                                    /* C 117 */
  /* C 117: declared without a value and written by `g_main_context_query` through the pointer
     (the C 116-117 shape PORTING.md 1.5 records), so the port initialises it and passes the
     same address: `&raw mut` needs a live value in Rust where C's `&tmo_in_millisec` does not.  */
  let mut tmo_in_millisec: c_int = 0;                                     /* C 117 */
  let mut must_free: c_int = 0;                                            /* C 117 */
  let need_to_dispatch: bool;                                             /* C 118 */
  /* C 119-121: `#ifdef USE_GTK`  */
  #[cfg (feature = "use_gtk")]
  let already_has_events: bool;

  // C 123-124
  if xg_select_suppress_count != 0
    {
      return pselect (fds_lim, rfds, wfds, efds, timeout, sigmask);
    }

  // C 126-127
  context = g_main_context_default ();
  acquire_select_lock (context);

  /* C 129-134: `#ifdef USE_GTK` - the pending-events probe, with C 131's `#ifndef HAVE_PGTK`.  */
  #[cfg (feature = "use_gtk")]
  {
    already_has_events = g_main_context_pending (context) != 0;
    #[cfg (not (feature = "have_pgtk"))]
    {
      already_has_events = already_has_events && x_gtk_use_native_input;
    }
  }

  // C 136-139
  if !rfds.is_null ()
    {
      all_rfds = *rfds;
    }
  else
    {
      FD_ZERO (&raw mut all_rfds);
    }
  if !wfds.is_null ()
    {
      all_wfds = *wfds;
    }
  else
    {
      FD_ZERO (&raw mut all_wfds);
    }

  // C 141-142
  n_gfds = g_main_context_query (context, G_PRIORITY_LOW, &raw mut tmo_in_millisec,
                                 gfds, gfds_size);

  // C 144-154
  if gfds_size < n_gfds
    {
      /* Avoid using SAFE_NALLOCA, as that implicitly refers to the current thread.  Using xnmalloc
	 avoids thread-switching problems here.  */
      gfds = xnmalloc (n_gfds as isize, core::mem::size_of::<GPollFD> () as isize) as *mut GPollFD;
      must_free = 1;
      gfds_size = n_gfds;
      n_gfds = g_main_context_query (context, G_PRIORITY_LOW, &raw mut tmo_in_millisec,
                                     gfds, gfds_size);
    }

  // C 156-169
  i = 0;
  while i < n_gfds
    {
      if (*gfds.offset (i as isize)).events as c_int & G_IO_IN != 0
        {
          FD_SET ((*gfds.offset (i as isize)).fd, &raw mut all_rfds);
          if (*gfds.offset (i as isize)).fd > max_fds
            {
              max_fds = (*gfds.offset (i as isize)).fd;
            }
        }
      if (*gfds.offset (i as isize)).events as c_int & G_IO_OUT != 0
        {
          FD_SET ((*gfds.offset (i as isize)).fd, &raw mut all_wfds);
          if (*gfds.offset (i as isize)).fd > max_fds
            {
              max_fds = (*gfds.offset (i as isize)).fd;
            }
          have_wfds = true;
        }
      i += 1;
    }

  // C 171-172
  if must_free != 0
    {
      xfree (gfds as *mut c_void);
    }

  // C 174-180
  if n_gfds >= 0 && tmo_in_millisec >= 0
    {
      tmo = make_timespec (tmo_in_millisec as i64 / 1000,
                           1000 * 1000 * (tmo_in_millisec as i64 % 1000));
      if timeout.is_null () || timespec_cmp (tmo, *timeout) < 0
        {
          tmop = &raw mut tmo;
        }
    }

  /* C 182-220: the pselect call, with the two arms (C 182-186's `#ifndef USE_GTK`,
     C 187-220's `#else`; both carried).  On PGTK an already-pending key press means we must not sleep in pselect (Bug#52761).  */
  #[cfg (not (feature = "use_gtk"))]
  {
    fds_lim = max_fds + 1;
    nfds = thread_select (Some (pselect), fds_lim, &raw mut all_rfds,
                          if have_wfds { &raw mut all_wfds } else { core::ptr::null_mut () },
                          efds, tmop, sigmask);
  }
  #[cfg (feature = "use_gtk")]
  {
    if !already_has_events
      {
        fds_lim = max_fds + 1;
        nfds = thread_select (Some (pselect), fds_lim, &raw mut all_rfds,
                              if have_wfds { &raw mut all_wfds } else { core::ptr::null_mut () },
                              efds, tmop, sigmask);
      }
    else
      {
        /* Emulate return values */
        nfds = 1;
        FD_ZERO (&raw mut all_rfds);
        if have_wfds
          {
            FD_ZERO (&raw mut all_wfds);
          }
        if !efds.is_null ()
          {
            FD_ZERO (efds);
          }
        our_fds += 1;
      }
  }

  // C 222-247
  if nfds < 0
    {
      retval = nfds;
    }
  else if nfds > 0
    {
      i = 0;
      while i < fds_lim
        {
          if FD_ISSET (i, &raw mut all_rfds)
            {
              if !rfds.is_null () && FD_ISSET (i, rfds)
                {
                  retval += 1;
                }
              else
                {
                  our_fds += 1;
                }
            }
          else if !rfds.is_null ()
            {
              FD_CLR (i, rfds);
            }

          if have_wfds && FD_ISSET (i, &raw mut all_wfds)
            {
              if !wfds.is_null () && FD_ISSET (i, wfds)
                {
                  retval += 1;
                }
              else
                {
                  our_fds += 1;
                }
            }
          else if !wfds.is_null ()
            {
              FD_CLR (i, wfds);
            }

          if !efds.is_null () && FD_ISSET (i, efds)
            {
              retval += 1;
            }
          i += 1;
        }
    }

  /* C 249-255: "If Gtk+ is in use eventually gtk_main_iteration will be called, unless retval is
     zero."  */
  #[cfg (feature = "use_gtk")]
  {
    need_to_dispatch = retval == 0;
  }
  #[cfg (not (feature = "use_gtk"))]
  {
    need_to_dispatch = true;
  }

  /* C 257-269: xwidgets make heavy use of GLib subprocesses, which add their own SIGCHLD handler;
     once GLib is done with them we restore ours (which chains the GLib one).  */
  #[cfg (feature = "have_xwidgets")]
  {
    catch_child_signal ();
  }

  // C 271-285
  if need_to_dispatch
    {
      acquire_select_lock (context);

      /* Prevent g_main_dispatch recursion, that would occur without block_input wrapper, because
	 event handlers call unblock_input.  Event loop recursion was causing Bug#15801.  */
      let pselect_errno: c_int = *errno ();
      block_input ();
      while g_main_context_pending (context) != 0
        {
          g_main_context_dispatch (context);
        }
      unblock_input ();
      *errno () = pselect_errno;
      release_select_lock ();
    }

  // C 287-291: "To not have to recalculate timeout, return like this."
  if (our_fds > 0 || (nfds == 0 && tmop == &raw mut tmo)) && retval == 0
    {
      retval = -1;
      *errno () = EINTR;
    }

  // C 294
  retval
}
/* C 296: `#endif /* HAVE_GLIB */` - the file gate.  */
