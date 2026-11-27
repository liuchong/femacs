// femacs: port of src/atimer.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  The seven
// non-`static` definitions (`start_atimer`, `cancel_atimer`, `stop_other_atimers`,
// `run_all_atimers`, `do_pending_atimers`, `turn_on_atimers`, `init_atimer`) are C's exported
// symbols, so they carry `#[unsafe(no_mangle)]` (PORTING.md 0.2/P-52); the five `static`
// helpers stay plain `unsafe fn`, and `handle_alarm_signal` is the `extern "C"` function
// `emacs_sigaction_init` is handed (C has it `static`: no symbol either way).
//
// The four dead regions are recorded in docs/ports/atimer.md rather than transcribed:
// `#ifdef HAVE_ITIMERSPEC` (C 54-66's statics, C 303-336's `if (0 <= timerfd || alarm_timer_ok)`
// block, C 475-482) and `#ifdef HAVE_TIMERFD` (C 28-34's includes, C 418-448's
// `timerfd_callback`, C 580-591's `have_buggy_timerfd`) need `timer_t`,
// `struct itimerspec`, `timer_create`, `timer_settime`, `timerfd_settime`, `TFD_TIMER_ABSTIME`
// and `add_timer_wait_descriptor`, none of which this target's headers have (measured:
// `clang -std=gnu23` rejects `timer_t`, `struct itimerspec`, `timerfd_settime` and
// `timer_create` on darwin-arm64 — the frozen table has HAVE_TIMER_SETTIME and HAVE_TIMERFD
// undefined, and syssignal.h 44-46 turns that into "HAVE_ITIMERSPEC is not defined");
// `#ifdef MSDOS` (C 36-38) is another target's header; and `#ifdef ENABLE_CHECKING`
// (C 489-576) is off in config.h, and its `defsubr (&Sdebug_timer_check)` must not appear in
// Rust at all (P-76: the port's source-level scan does not consult the condition table, so the
// text would read as a live registration).
//
// The include list is C 19-26's, through the mirrors those pull in: `config.h` (19),
// `lisp.h` (21: `pending_signals`, `xzalloc`, `memset`, `eassert`, `NULL`), `syssignal.h`
// (23, the mirror that pulls `<signal.h>`), `systime.h` (24: `make_timeval`, and through it
// `<sys/time.h>`'s `setitimer`), `atimer.h` (25) and `<unistd.h>` (26: `alarm`).  C 22's
// `keyboard.h` is the one include left out: its mirror names `struct input_event` (and with it
// termhooks.h's whole display closure) in declarations this unit's live text never touches,
// and the single name this file takes from C's keyboard chain — `pending_signals` — is
// declared by lisp.h 3879, which is where C's own text gets it (docs/ports/atimer.md).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/string.rs imported via common
// [U1] ../lib/include/unistd.in.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/syssignal.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/atimer.rs imported via common

// C 28-34's `#ifdef HAVE_TIMERFD` includes (<errno.h>, <sys/timerfd.h>, and Cygwin's
// <sys/utsname.h> inside it) and C 36-38's `#ifdef MSDOS` (<msdos.h>) are dead text: no such
// header exists on this target (docs/ports/atimer.md).

/* Free-list of atimer structures.  */

static free_atimers: VmGlobal<*mut atimer> = VmGlobal::<*mut atimer>::new (std::ptr::null_mut ());

/* List of currently not running timers due to a call to
   lock_atimer.  */

static stopped_atimers: VmGlobal<*mut atimer> = VmGlobal::<*mut atimer>::new (std::ptr::null_mut ());

/* List of active atimers, sorted by expiration time.  The timer that
   will become ripe next is always at the front of this list.  */

static atimers: VmGlobal<*mut atimer> = VmGlobal::<*mut atimer>::new (std::ptr::null_mut ());

/* C 54-66: `#ifdef HAVE_ITIMERSPEC` - the alarm timer, whether it was initialized, and the
   timerfd.  Dead on this target: HAVE_ITIMERSPEC is not defined (C 44-46 of syssignal.h
   defines it only when HAVE_TIMER_SETTIME is, and the frozen table has that undefined), and
   the `timer_t` the block is built on does not exist in this target's <time.h>.  */

/* Block/unblock SIGALRM.  */

unsafe fn block_atimers (oldset: *mut sigset_t)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut blocked: sigset_t = std::mem::zeroed ();
  sigemptyset (&mut blocked);
  sigaddset (&mut blocked, SIGALRM);
  sigaddset (&mut blocked, SIGINT);
  pthread_sigmask (SIG_BLOCK, &blocked, oldset);

  }
}
unsafe fn unblock_atimers (oldset: *const sigset_t)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  pthread_sigmask (SIG_SETMASK, oldset, 0 as *mut sigset_t);

  }
}

/* Function prototypes.  */

// C 85-88 forward-declares the three static helpers below (`set_alarm`, `schedule_atimer`,
// `append_atimer_lists`); Rust items are order-independent, so C's prototype-plus-definition
// pair has no counterpart (PORTING.md 0.1-12).

/* Start a new atimer of type TYPE.  TIMESTAMP specifies when the timer is
   ripe.  FN is the function to call when the timer fires.
   CLIENT_DATA is stored in the client_data member of the atimer
   structure returned and so made available to FN when it is called.

   If TYPE is ATIMER_ABSOLUTE, TIMESTAMP is the absolute time at which the
   timer fires.

   If TYPE is ATIMER_RELATIVE, the timer is ripe TIMESTAMP seconds in the
   future.

   In both cases, the timer is automatically freed after it has fired.

   If TYPE is ATIMER_CONTINUOUS, the timer fires every TIMESTAMP seconds.

   Value is a pointer to the atimer started.  It can be used in calls
   to cancel_atimer; don't free it yourself.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn start_atimer (r#type: atimer_type, mut timestamp: timespec,
                                       r#fn: atimer_callback,
                                       client_data: *mut c_void) -> *mut atimer
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut t: *mut atimer;
  let mut oldset: sigset_t = std::mem::zeroed ();

  /* Round TIMESTAMP up to the next full second if we don't have itimers.  */
  // C 118-121's `#if ! (defined HAVE_ITIMERSPEC || defined HAVE_SETITIMER)` arm is dead:
  // HAVE_SETITIMER is 1 on this target, so the `make_timespec (timestamp.tv_sec + 1, 0)`
  // rounding is not compiled (docs/ports/atimer.md).

  /* Get an atimer structure from the free-list, or allocate
     a new one.  */
  if !(*free_atimers.as_mut_ptr ()).is_null ()
    {
      t = (*free_atimers.as_mut_ptr ());
      (*free_atimers.as_mut_ptr ()) = (*t).next;
      memset (t as *mut c_void, 0, std::mem::size_of::<atimer> ());
    }
  else
    { t = xzalloc (std::mem::size_of::<atimer> ()) as *mut atimer; }

  /* Fill the atimer structure.  */
  (*t).r#type = r#type;
  (*t).r#fn = r#fn;
  (*t).client_data = client_data;

  block_atimers (&mut oldset);

  /* Compute the timer's expiration time.  */
  match r#type
    {
    ATIMER_ABSOLUTE =>
      {
        (*t).expiration = timestamp;
      }

    ATIMER_RELATIVE =>
      {
        (*t).expiration = timespec_add (current_timespec (), timestamp);
      }

    ATIMER_CONTINUOUS =>
      {
        (*t).expiration = timespec_add (current_timespec (), timestamp);
        (*t).interval = timestamp;
      }

    // C's `switch` has no `default:` arm: a value outside the three members leaves the
    // timer's expiration unset, exactly as C does (PORTING.md 2.2; the cm.rs/data.rs
    // `_ => { }` shape).
    _ => { }
    }

  /* Insert the timer in the list of active atimers.  */
  schedule_atimer (t);
  unblock_atimers (&oldset);

  /* Arrange for a SIGALRM at the time the next atimer is ripe.  */
  set_alarm ();

  return t;

  }
}


/* Cancel and free atimer TIMER.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn cancel_atimer (timer: *mut atimer)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut i: c_int = 0;
  let mut oldset: sigset_t = std::mem::zeroed ();

  block_atimers (&mut oldset);

  while i < 2
    {
      let mut t: *mut atimer;
      let mut prev: *mut atimer;
      let list: *mut *mut atimer = if i != 0 { stopped_atimers.as_mut_ptr () }
                                   else { atimers.as_mut_ptr () };

      /* See if TIMER is active or stopped.  */
      t = *list;
      prev = std::ptr::null_mut ();
      while !t.is_null () && t != timer
        { prev = t; t = (*t).next; }

      /* If it is, take it off its list, and put in on the free-list.
	 We don't bother to arrange for setting a different alarm time,
	 since a too early one doesn't hurt.  */
      if !t.is_null ()
        {
          if !prev.is_null ()
            { (*prev).next = (*t).next; }
          else
            { *list = (*t).next; }

          (*t).next = (*free_atimers.as_mut_ptr ());
          (*free_atimers.as_mut_ptr ()) = t;
          break;
        }
      i += 1;
    }

  /* We shouldn't be called with timers which aren't on either list.  */
  eassert! (i != 2);

  unblock_atimers (&oldset);

  }
}


/* Append two lists of atimers LIST_1 and LIST_2 and return the
   result list.  */

unsafe fn append_atimer_lists (list_1: *mut atimer, list_2: *mut atimer) -> *mut atimer
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if list_1.is_null ()
    { return list_2; }
  else if list_2.is_null ()
    { return list_1; }
  else
    {
      let mut p: *mut atimer;

      p = list_1;
      while !(*p).next.is_null ()
        { p = (*p).next; }
      (*p).next = list_2;
      return list_1;
    }

  }
}


/* Stop all timers except timer T.  T null means stop all timers.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn stop_other_atimers (mut t: *mut atimer)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut oldset: sigset_t = std::mem::zeroed ();
  block_atimers (&mut oldset);

  if !t.is_null ()
    {
      let mut p: *mut atimer;
      let mut prev: *mut atimer;

      /* See if T is active.  */
      p = (*atimers.as_mut_ptr ());
      prev = std::ptr::null_mut ();
      while !p.is_null () && p != t
        { prev = p; p = (*p).next; }

      if p == t
        {
          if !prev.is_null ()
            { (*prev).next = (*t).next; }
          else
            { (*atimers.as_mut_ptr ()) = (*t).next; }
          (*t).next = std::ptr::null_mut ();
        }
      else
	/* T is not active.  Let's handle this like T == 0.  */
	{ t = std::ptr::null_mut (); }
    }

  (*stopped_atimers.as_mut_ptr ()) = append_atimer_lists ((*atimers.as_mut_ptr ()), (*stopped_atimers.as_mut_ptr ()));
  (*atimers.as_mut_ptr ()) = t;
  unblock_atimers (&oldset);

  }
}


/* Run all timers again, if some have been stopped with a call to
   stop_other_atimers.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn run_all_atimers ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if !(*stopped_atimers.as_mut_ptr ()).is_null ()
    {
      let mut t: *mut atimer = (*atimers.as_mut_ptr ());
      let mut next: *mut atimer;
      let mut oldset: sigset_t = std::mem::zeroed ();

      block_atimers (&mut oldset);
      (*atimers.as_mut_ptr ()) = (*stopped_atimers.as_mut_ptr ());
      (*stopped_atimers.as_mut_ptr ()) = std::ptr::null_mut ();

      while !t.is_null ()
	{
	  next = (*t).next;
	  schedule_atimer (t);
	  t = next;
	}

      unblock_atimers (&oldset);
    }

  }
}


/* Arrange for a SIGALRM to arrive when the next timer is ripe.  */

unsafe fn set_alarm ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if !(*atimers.as_mut_ptr ()).is_null ()
    {
      // C 303-336's `#ifdef HAVE_ITIMERSPEC` block (the POSIX timer and the timerfd, with
      // Cygwin's `# ifdef CYGWIN` split inside it) is dead on this target: the entries it
      // calls (`timer_settime`, `timerfd_settime`, `add_timer_wait_descriptor`) and the
      // record it fills (`struct itimerspec`) do not exist here (docs/ports/atimer.md).

      /* Determine interval till the next timer is ripe.  */
      let now: timespec = current_timespec ();
      if timespec_cmp ((*(*atimers.as_mut_ptr ())).expiration, now) <= 0
	{
	  /* Timer is (over)due -- just trigger the signal right way.  */
	  raise (SIGALRM);
	}
      else
	{
	  let interval: timespec = timespec_sub ((*(*atimers.as_mut_ptr ())).expiration, now);

	  // C 349's `#ifdef HAVE_SETITIMER` is live (the table has it 1) and C 352-353's
	  // `#else` arm (`alarm (max (interval.tv_sec, 1))`) is dead with it.
	  let it = itimerval { it_interval: timeval { tv_sec: 0, tv_usec: 0 },
			       it_value: make_timeval (interval) };
	  setitimer (ITIMER_REAL, &it, std::ptr::null_mut ());
	}
    }

  }
}


/* Insert timer T into the list of active atimers `atimers', keeping
   the list sorted by expiration time.  T must not be in this list
   already.  */

unsafe fn schedule_atimer (t: *mut atimer)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut a: *mut atimer = (*atimers.as_mut_ptr ());
  let mut prev: *mut atimer = std::ptr::null_mut ();

  /* Look for the first atimer that is ripe after T.  */
  while !a.is_null () && timespec_cmp ((*a).expiration, (*t).expiration) < 0
    { prev = a; a = (*a).next; }

  /* Insert T in front of the atimer found, if any.  */
  if !prev.is_null ()
    { (*prev).next = t; }
  else
    { (*atimers.as_mut_ptr ()) = t; }

  (*t).next = a;

  }
}

unsafe fn run_timers ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let now: timespec = current_timespec ();

  while !(*atimers.as_mut_ptr ()).is_null () && timespec_cmp ((*(*atimers.as_mut_ptr ())).expiration, now) <= 0
    {
      let t: *mut atimer = (*atimers.as_mut_ptr ());
      (*atimers.as_mut_ptr ()) = (*(*atimers.as_mut_ptr ())).next;
      // C's `t->fn (t)`: the field is the mirror's `Option<unsafe extern "C" fn (…)>`, and
      // every atimer on `atimers` was filled in by `start_atimer`, so the callback is set
      // (the json.rs/vm-limit.rs `unwrap ()` shape, PORTING.md 5.2).
      ((*t).r#fn).unwrap () (t);

      if (*t).r#type == ATIMER_CONTINUOUS
	{
	  (*t).expiration = timespec_add (now, (*t).interval);
	  schedule_atimer (t);
	}
      else
	{
	  (*t).next = (*free_atimers.as_mut_ptr ());
	  (*free_atimers.as_mut_ptr ()) = t;
	}
    }

  set_alarm ();

  }
}


/* Signal handler for SIGALRM.  SIGNO is the signal number, i.e.
   SIGALRM.  */

unsafe extern "C" fn handle_alarm_signal (sig: c_int)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  (*pending_signals.as_mut_ptr ()) = true;

  }
}

/* C 418-448: `#ifdef HAVE_TIMERFD` - dead for the frozen configuration, live for the Android one
   (`HAVE_TIMERFD` is 1 there, and `process.c` 8360's `add_timer_wait_descriptor` installs this
   callback; status 4.729).  C's `static int timerfd` sits in the `HAVE_ITIMERSPEC` block at 62,
   which this configuration does not record, so the port carries the variable under the same
   `have_timerfd` key that guards its only reader and writer (the timerfd arm of
   `set_alarm`) - recorded in docs/ports/atimer.md as a configuration question, not a rewrite.  */
#[cfg (feature = "have_timerfd")]
static timerfd: VmGlobal<c_int> = VmGlobal::<c_int>::new (0);

/// C 421-446: "Called from wait_reading_process_output when FD, which should be equal to
/// TIMERFD, is available for reading."
#[cfg (feature = "have_timerfd")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn timerfd_callback (fd: c_int, _arg: *mut c_void)
{
  /* C 423-424: "Timer should expire just once."  */
  eassert! (fd == (*timerfd.as_mut_ptr ()));
  let mut expirations: u64 = 0;
  let nbytes: ptrdiff_t = emacs_read (fd, &raw mut expirations as *mut c_void,
				      std::mem::size_of::<u64> () as ptrdiff_t);

  if nbytes == std::mem::size_of::<u64> () as ptrdiff_t
    {
      eassert! (expirations == 1);
      do_pending_atimers ();
    }
  else if nbytes < 0
    /* C 436-441: "For some not yet known reason, we may get weird event and no data on timer
       descriptor."  */
    { eassert! (*errno () == EAGAIN); }
  else
    /* C 442-444: "I don't know what else can happen with this descriptor."  */
    { emacs_abort (); }
}

/* Do pending timers.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn do_pending_atimers ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if !(*atimers.as_mut_ptr ()).is_null ()
    {
      let mut oldset: sigset_t = std::mem::zeroed ();
      block_atimers (&mut oldset);
      run_timers ();
      unblock_atimers (&oldset);
    }

  }
}


/* Turn alarms on/off.  This seems to be temporarily necessary on
   some systems like HPUX (see process.c).  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn turn_on_atimers (on: bool)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if on
    { set_alarm (); }
  else
    {
      // C 475-482's `#ifdef HAVE_ITIMERSPEC` block (the two `timer_settime` calls that
      // disarm the POSIX timer and the timerfd) is dead on this target; C 483's
      // `alarm (0)` below is the live text (docs/ports/atimer.md).
      alarm (0);
    }

  }
}

// C 489-576's `#ifdef ENABLE_CHECKING` block - `MAXTIMERS`, `struct atimer_result`,
// `debug_timer_callback` and the `debug-timer-check` subr - is dead in the frozen
// configuration (config.h does not define ENABLE_CHECKING).  It is not transcribed: its
// `wait_reading_process_output` (C 549/562) is not declared by any mirror, and its
// `defsubr (&Sdebug_timer_check)` (C 623) must not appear in Rust at all (P-76).  The names
// and the C line ranges are recorded in docs/ports/atimer.md.

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn init_atimer ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  // C 596-613's `#ifdef HAVE_ITIMERSPEC` block (the `timerfd_create` call guarded by
  // `EMACS_IGNORE_TIMERFD`, the `struct sigevent sigev` and the `timer_create` that arms the
  // POSIX timer) is dead on this target (docs/ports/atimer.md).

  let null: *mut atimer = std::ptr::null_mut ();
  (*free_atimers.as_mut_ptr ()) = null;
  (*stopped_atimers.as_mut_ptr ()) = null;
  (*atimers.as_mut_ptr ()) = null;

  #[cfg (feature = "isolate_vm")]
  if crate::sys::isolate::is_isolate_active () {
    static ATIMER_SIG_ONCE: std::sync::Once = std::sync::Once::new ();
    ATIMER_SIG_ONCE.call_once (|| {
      let mut action: sigaction = std::mem::zeroed ();
      emacs_sigaction_init (&mut action, Some (handle_alarm_signal));
      sigaction (SIGALRM, &action, 0 as *mut sigaction);
    });
    return;
  }

  /* pending_signals is initialized in init_keyboard.  */
  let mut action: sigaction = std::mem::zeroed ();
  emacs_sigaction_init (&mut action, Some (handle_alarm_signal));
  sigaction (SIGALRM, &action, 0 as *mut sigaction);

  // C 621-624's `#ifdef ENABLE_CHECKING` (`if (!initialized) defsubr (&Sdebug_timer_check);`)
  // is dead here; `defsubr` is not written (P-76), see the block comment above.

  }
}
