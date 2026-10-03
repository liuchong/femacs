// femacs: port of src/profiler.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  The nine
// `DEFUN`s use `DEFUN!` (PORTING.md 5.1); `malloc_probe`, `mark_profiler` and
// `syms_of_profiler` are C's other exported symbols and carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2/P-52), as does C's one non-static object, `profiler_memory_running`.
// The file's own helpers stay plain `fn` (no pointers, e.g. `saturated_add`) or `unsafe fn`
// (pointer arguments: `mark_log`, `make_log`, ...), so none of them emits a symbol.
//
// The dead region is `#ifdef HAVE_ITIMERSPEC`, recorded in docs/ports/profiler.md rather
// than transcribed: C 352-355's `timer_t`/`profiler_timer_ok`, the `TIMER_SETTIME_RUNNING`
// enumerator, C 404-406's `timer_getoverrun` block, C 432-459's `timer_create`/
// `timer_settime` block with its `system_clock` table, and C 495-500's stop arm.  It needs
// `timer_t`, `struct itimerspec`, `struct sigevent`, `timer_create`, `timer_settime` and
// `timer_getoverrun`, none of which this target's headers have: the frozen table has
// HAVE_TIMER_SETTIME undefined, and syssignal.h 44-46 turns that into "HAVE_ITIMERSPEC is
// not defined" (docs/ports/include-syssignal.md, measured there).  `HAVE_SETITIMER` is 1,
// so `setup_cpu_timer`'s setitimer tail is live; `PROFILER_CPU_SUPPORT` is defined (SIGPROF
// 27, PROFILING undefined), so C 343-524 and C 703-712 are live text.
//
// C 188's forward declaration `static Lisp_Object export_log (struct profiler_log *);` has
// no Rust counterpart (PORTING.md 0.1-12: Rust's items are order-independent), so the
// definition stays at C 562's position and the declaration is dropped (docs/ports/profiler.md).
//
// The include list is C 20-24's - config.h, lisp.h, syssignal.h, systime.h, pdumper.h -
// through the mirrors those pull in: `syssignal.rs` for `emacs_sigaction_init`/
// `deliver_process_signal`, `systime.rs` for `make_timespec`/`make_timeval`, `pdumper.rs`
// for the dump hooks.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Profiler implementation.

Copyright (C) 2012-2026 Free Software Foundation, Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at
your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

include!("include/config.rs");
include!("../lib/include/stdio.rs");	// lisp.h 34: FILE
include!("../lib/include/string.rs");	// lisp.h 30: <string.h> (`memcpy`)
include!("include/lisp.rs");
include!("include/syssignal.rs");	// C 22 (and through it <signal.h>: SIGPROF, SIG_IGN)
include!("include/systime.rs");		// C 23 (make_timespec, make_timeval, setitimer)
include!("include/pdumper.rs");		// C 24

// C 374's `TYPE_MAXIMUM (time_t)` comes from gnulib's <intprops.h>; the lib/ ports write
// it as a unit-local macro (docs/ports/token-aliases.tsv's TYPE_MAXIMUM row, the
// syntax.rs shape), placed here because `include!` order is textual.
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }

/* Return A + B, but return the maximum fixnum if the result would overflow.
   Assume A and B are nonnegative and in fixnum range.  */

fn saturated_add (a: EMACS_INT, b: EMACS_INT) -> EMACS_INT
{
  return min (a + b, MOST_POSITIVE_FIXNUM);
}

/* Logs.  */

/* A fully associative cache of size SIZE, mapping vectors of DEPTH
   Lisp objects to counts.  */
#[repr(C)]
pub struct log_t {
  /* We use `int' throughout for table indices because anything bigger
     is overkill.  (Maybe we should make a typedef, but int is short.)  */
  pub size: c_int,		        /* number of entries */
  pub depth: c_int,			/* elements in each key vector */
  pub index_size: c_int,	        /* size of index */
  pub trace: *mut Lisp_Object,		/* working trace, `depth' elements */
  pub index: *mut c_int,		/* `index_size' indices or -1 if nothing */
  pub next: *mut c_int,			/* `size' indices to next bucket or -1 */
  pub hash: *mut EMACS_UINT,	        /* `size' hash values */
  pub keys: *mut Lisp_Object,		/* `size' keys of `depth' objects each */
  pub counts: *mut EMACS_INT,		/* `size' entries, 0 indicates unused entry */
  pub next_free: c_int,			/* next free entry, -1 if all taken */
}

unsafe fn mark_log (log: *mut log_t)
{
  if log.is_null ()
    { return; }
  let size: c_int = (*log).size;
  let depth: c_int = (*log).depth;
  let mut i: c_int = 0;
  while i < size
    {
      if *(*log).counts.offset (i as isize) > 0	/* Only mark valid keys.  */
	{ mark_objects ((*log).keys.offset ((i * depth) as isize), depth as ptrdiff_t); }
      i += 1;
    }
}

unsafe fn make_log (size: c_int, depth: c_int) -> *mut log_t
{
  let log: *mut log_t = xmalloc (std::mem::size_of::<log_t> ()) as *mut log_t;
  (*log).size = size;
  (*log).depth = depth;

  /* The index size is arbitrary but for there to be any point it should be
     bigger than SIZE.  FIXME: make it a power of 2 or a (pseudo)prime.  */
  let index_size: c_int = size * 2 + 1;
  (*log).index_size = index_size;

  (*log).trace = xnmalloc (depth as isize, std::mem::size_of::<Lisp_Object> () as isize)
    as *mut Lisp_Object;

  (*log).index = xnmalloc (index_size as isize, std::mem::size_of::<c_int> () as isize)
    as *mut c_int;
  let mut i: c_int = 0;
  while i < index_size
    {
      *(*log).index.offset (i as isize) = -1;
      i += 1;
    }

  (*log).next = xnmalloc (size as isize, std::mem::size_of::<c_int> () as isize) as *mut c_int;
  i = 0;
  while i < size - 1
    {
      *(*log).next.offset (i as isize) = i + 1;
      i += 1;
    }
  *(*log).next.offset ((size - 1) as isize) = -1;
  (*log).next_free = 0;

  (*log).hash = xnmalloc (size as isize, std::mem::size_of::<EMACS_UINT> () as isize)
    as *mut EMACS_UINT;
  let mut size_x_depth: usize = 0;
  // C: `if (ckd_mul (&size_x_depth, size, depth)) memory_full_up ();` - the store and the
  // overflow flag are `overflowing_mul`'s pair (P-15, docs/ports/token-aliases.tsv).
  let (size_x_depth_prod, size_x_depth_overflowed) = (size as usize).overflowing_mul (depth as usize);
  size_x_depth = size_x_depth_prod;
  if size_x_depth_overflowed
    { memory_full_up (); }
  (*log).keys = xcalloc (size_x_depth, std::mem::size_of::<Lisp_Object> ()) as *mut Lisp_Object;
  (*log).counts = xcalloc (size as usize, std::mem::size_of::<EMACS_INT> ()) as *mut EMACS_INT;

  return log;
}

unsafe fn free_log (log: *mut log_t)
{
  xfree ((*log).trace as *mut c_void);
  xfree ((*log).index as *mut c_void);
  xfree ((*log).next as *mut c_void);
  xfree ((*log).hash as *mut c_void);
  xfree ((*log).keys as *mut c_void);
  xfree ((*log).counts as *mut c_void);
  xfree (log as *mut c_void);
}

#[inline(always)]
unsafe fn get_log_count (log: *mut log_t, idx: c_int) -> EMACS_INT
{
  eassume! (idx >= 0 && idx < (*log).size);
  return *(*log).counts.offset (idx as isize);
}

#[inline(always)]
unsafe fn set_log_count (log: *mut log_t, idx: c_int, val: EMACS_INT)
{
  eassume! (idx >= 0 && idx < (*log).size && val >= 0);
  *(*log).counts.offset (idx as isize) = val;
}

#[inline(always)]
unsafe fn get_key_vector (log: *mut log_t, idx: c_int) -> *mut Lisp_Object
{
  eassume! (idx >= 0 && idx < (*log).size);
  return (*log).keys.offset ((idx * (*log).depth) as isize);
}

#[inline(always)]
unsafe fn log_hash_index (log: *mut log_t, hash: EMACS_UINT) -> c_int
{
  /* FIXME: avoid division.  */
  return (hash % (*log).index_size as EMACS_UINT) as c_int;
}

unsafe fn remove_log_entry (log: *mut log_t, idx: c_int)
{
  eassume! (idx >= 0 && idx < (*log).size);
  /* Remove from index.  */
  let hidx: c_int = log_hash_index (log, *(*log).hash.offset (idx as isize));
  let mut p: *mut c_int = (*log).index.offset (hidx as isize);
  while *p != idx
    {
      eassert! (*p >= 0 && *p < (*log).size);
      p = (*log).next.offset (*p as isize);
    }
  *p = *(*log).next.offset (*p as isize);
  /* Invalidate entry and put it on the free list.  */
  *(*log).counts.offset (idx as isize) = 0;
  *(*log).next.offset (idx as isize) = (*log).next_free;
  (*log).next_free = idx;
}

unsafe fn trace_equal (bt1: *mut Lisp_Object, bt2: *mut Lisp_Object, depth: c_int) -> bool
{
  let mut i: c_int = 0;
  while i < depth
    {
      if !BASE_EQ (*bt1.offset (i as isize), *bt2.offset (i as isize))
	&& NILP (Ffunction_equal (*bt1.offset (i as isize), *bt2.offset (i as isize)))
	{ return false; }
      i += 1;
    }
  return true;
}

unsafe fn trace_hash (trace: *mut Lisp_Object, depth: c_int) -> EMACS_UINT
{
  let mut hash: EMACS_UINT = 0;
  let mut i: c_int = 0;
  while i < depth
    {
      let f: Lisp_Object = *trace.offset (i as isize);
      let hash1: EMACS_UINT
	= if CLOSUREP (f) { XHASH (AREF (f, CLOSURE_CODE as isize)) } else { XHASH (f) };
      hash = sxhash_combine (hash, hash1);
      i += 1;
    }
  return hash;
}

#[repr(C)]
pub struct profiler_log {
  pub log: *mut log_t,
  pub gc_count: EMACS_INT,  /* Samples taken during GC.  */
  pub discarded: EMACS_INT, /* Samples evicted during table overflow.  */
}

// C 188's `static Lisp_Object export_log (struct profiler_log *);` is dropped: a Rust item
// is visible to the whole crate regardless of position (PORTING.md 0.1-12), so the
// definition below stays at C 562's place and no declaration is needed.

unsafe fn make_profiler_log () -> profiler_log
{
  let size: c_int = clip_to_bounds (0, profiler_log_size, min (MOST_POSITIVE_FIXNUM, INT_MAX as EMACS_INT)) as c_int;
  let max_stack_depth: c_int = clip_to_bounds (0, profiler_max_stack_depth, INT_MAX as EMACS_INT) as c_int;
  return profiler_log { log: make_log (size, max_stack_depth), gc_count: 0, discarded: 0 };
}

unsafe fn free_profiler_log (plog: *mut profiler_log)
{
  free_log ((*plog).log);
  (*plog).log = std::ptr::null_mut ();
}


/* Evict the least used half of the hash_table.

   When the table is full, we have to evict someone.
   The easiest and most efficient is to evict the value we're about to add
   (i.e. once the table is full, stop sampling).

   We could also pick the element with the lowest count and evict it,
   but finding it is O(N) and for that amount of work we get very
   little in return: for the next sample, this latest sample will have
   count==1 and will hence be a prime candidate for eviction :-(

   So instead, we take O(N) time to eliminate more or less half of the
   entries (the half with the lowest counts).  So we get an amortized
   cost of O(1) and we get O(N) time for a new entry to grow larger
   than the other least counts before a new round of eviction.  */

unsafe fn approximate_median (log: *mut log_t, start: c_int, size: c_int) -> EMACS_INT
{
  eassert! (size > 0);
  if size < 2
    { return get_log_count (log, start); }
  if size < 3
    /* Not an actual median, but better for our application than
       choosing either of the two numbers.  */
    { return ((get_log_count (log, start)
	       + get_log_count (log, start + 1))
	      / 2); }
  else
    {
      let newsize: c_int = size / 3;
      let start2: c_int = start + newsize;
      let i1: EMACS_INT = approximate_median (log, start, newsize);
      let i2: EMACS_INT = approximate_median (log, start2, newsize);
      let i3: EMACS_INT = approximate_median (log, start2 + newsize,
					      size - 2 * newsize);
      return if i1 < i2
	{ if i2 < i3 { i2 } else { max (i1, i3) } }
      else
	{ if i1 < i3 { i1 } else { max (i2, i3) } };
    }
}

unsafe fn evict_lower_half (plog: *mut profiler_log)
{
  let log: *mut log_t = (*plog).log;
  let size: c_int = (*log).size;
  let median: EMACS_INT = approximate_median (log, 0, size);

  let mut i: c_int = 0;
  while i < size
    {
      let count: EMACS_INT = get_log_count (log, i);
      /* Evict not only values smaller but also values equal to the median,
	 so as to make sure we evict something no matter what.  */
      if count <= median
	{
	  (*plog).discarded = saturated_add ((*plog).discarded, count);
	  remove_log_entry (log, i);
	}
      i += 1;
    }
}

/* Record the current backtrace in LOG.  COUNT is the weight of this
   current backtrace: interrupt counts for CPU, and the allocation
   size for memory.  */

unsafe fn record_backtrace (plog: *mut profiler_log, count: EMACS_INT)
{
  let log: *mut log_t = (*plog).log;
  get_backtrace ((*log).trace, (*log).depth as ptrdiff_t);
  let hash: EMACS_UINT = trace_hash ((*log).trace, (*log).depth);
  let hidx: c_int = log_hash_index (log, hash);
  let mut idx: c_int = *(*log).index.offset (hidx as isize);
  while idx >= 0
    {
      if *(*log).hash.offset (idx as isize) == hash
	  && trace_equal ((*log).trace, get_key_vector (log, idx), (*log).depth)
	{
	  /* Found existing entry.  */
	  set_log_count (log, idx,
			 saturated_add (get_log_count (log, idx), count));
	  return;
	}
      idx = *(*log).next.offset (idx as isize);
    }

  /* Add new entry.  */
  if (*log).next_free < 0
    { evict_lower_half (plog); }
  idx = (*log).next_free;
  eassert! (idx >= 0);
  (*log).next_free = *(*log).next.offset (idx as isize);
  *(*log).next.offset (idx as isize) = *(*log).index.offset (hidx as isize);
  *(*log).index.offset (hidx as isize) = idx;
  eassert! (*(*log).counts.offset (idx as isize) == 0);
  *(*log).hash.offset (idx as isize) = hash;
  memcpy (get_key_vector (log, idx) as *mut c_void, (*log).trace as *const c_void,
	  (*log).depth as usize * std::mem::size_of::<Lisp_Object> ());
  *(*log).counts.offset (idx as isize) = count;

  /* FIXME: If the hash-table is almost full, we should set
     some global flag so that some Elisp code can offload its
     data elsewhere, so as to avoid the eviction code.
     There are 2 ways to do that:
     - Set a flag checked in maybe_quit, such that maybe_quit can then
     call Fprofiler_cpu_log and stash the full log for later use.
     - Set a flag check in post-gc-hook, so that Elisp code can call
     profiler-cpu-log.  That gives us more flexibility since that
     Elisp code can then do all kinds of fun stuff like write
     the log to disk.  Or turn it right away into a call tree.
     Of course, using Elisp is generally preferable, but it may
     take longer until we get a chance to run the Elisp code, so
     there's more risk that the table will get full before we
     get there.  */
}

/* Sampling profiler.  */

/* Signal handler for sampling profiler.  */

unsafe fn add_sample (plog: *mut profiler_log, count: EMACS_INT)
{
  if BASE_EQ (backtrace_top_function (), QAutomatic_GC ()) /* bug#60237 */
    /* Special case the time-count inside GC because the hash-table
       code is not prepared to be used while the GC is running.
       More specifically it uses ASIZE at many places where it does
       not expect the ARRAY_MARK_FLAG to be set.  We could try and
       harden the hash-table code, but it doesn't seem worth the
       effort.  */
    { (*plog).gc_count = saturated_add ((*plog).gc_count, count); }
  else
    { record_backtrace (plog, count); }
}

/* The sampling interval specified.  */
static mut profiler_cpu_interval: Lisp_Object = lisp_h_Qnil ();

/* Status of sampling profiler.  */
// C 358-366 declares `static enum profiler_cpu_running { NOT_RUNNING,
// TIMER_SETTIME_RUNNING, SETITIMER_RUNNING } profiler_cpu_running;`.  The two enumerators
// this target keeps are C's own names as `c_int` constants, so that C's
// `profiler_cpu_running = status;` (an `int`) stays a plain assignment; the dead
// TIMER_SETTIME_RUNNING is in docs/ports/profiler.md.
pub const NOT_RUNNING: c_int = 0;
pub const SETITIMER_RUNNING: c_int = 1;
static mut profiler_cpu_running: c_int = NOT_RUNNING;

/* Hash-table log of CPU profiler.  */
static mut cpu: profiler_log = profiler_log { log: std::ptr::null_mut (), gc_count: 0, discarded: 0 };

/* The current sampling interval in nanoseconds.  */
static mut current_sampling_interval: EMACS_INT = 0;

unsafe extern "C" fn handle_profiler_signal (signal: c_int)
{
  let mut count: EMACS_INT = 1;
  // C 404-406's `#if defined HAVE_ITIMERSPEC && defined HAVE_TIMER_GETOVERRUN` arm is
  // dead on this target (docs/ports/profiler.md).
  add_sample (&raw mut cpu, count);
}

unsafe extern "C" fn deliver_profiler_signal (signal: c_int)
{
  deliver_process_signal (signal, Some (handle_profiler_signal));
}

unsafe fn setup_cpu_timer (sampling_interval: Lisp_Object) -> c_int
{
  let billion: EMACS_INT = 1000000000;

  if !RANGED_FIXNUMP (1, sampling_interval,
		      (if (TYPE_MAXIMUM! (time_t) as EMACS_INT) < EMACS_INT_MAX / billion
		       { (TYPE_MAXIMUM! (time_t) as EMACS_INT) * billion + (billion - 1) }
		       else { EMACS_INT_MAX }))
    { return -1; }

  current_sampling_interval = XFIXNUM (sampling_interval);
  let interval: timespec
    = make_timespec ((current_sampling_interval / billion) as time_t,
		     (current_sampling_interval % billion) as c_long);
  let mut action: sigaction = std::mem::zeroed ();
  emacs_sigaction_init (&mut action, Some (deliver_profiler_signal));
  sigaction (SIGPROF, &action, 0 as *mut sigaction);

  // C 432-459's `#ifdef HAVE_ITIMERSPEC` block (the `system_clock` table, `timer_create`
  // and `timer_settime`) is dead on this target (docs/ports/profiler.md).

  // C 461-466's `#ifdef HAVE_SETITIMER` arm; HAVE_SETITIMER is 1.
  let mut timer: itimerval = std::mem::zeroed ();
  timer.it_interval = make_timeval (interval);
  timer.it_value = timer.it_interval;
  if setitimer (ITIMER_PROF, &timer, 0 as *mut itimerval) == 0
    { return SETITIMER_RUNNING; }

  return NOT_RUNNING;
}

DEFUN! ("profiler-cpu-start", Fprofiler_cpu_start, Sprofiler_cpu_start,
       1, 1, 0,
       doc: "Start or restart the cpu profiler.\nIt takes call-stack samples each SAMPLING-INTERVAL nanoseconds, approximately.\nSee also `profiler-log-size' and `profiler-max-stack-depth'.",
       (sampling_interval: Lisp_Object),
       {
  if profiler_cpu_running != NOT_RUNNING
    { error (c"CPU profiler is already running".as_ptr ()); }

  if cpu.log.is_null ()
    { cpu = make_profiler_log (); }

  let status: c_int = setup_cpu_timer (sampling_interval);
  if status < 0
    {
      profiler_cpu_running = NOT_RUNNING;
      error (c"Invalid sampling interval".as_ptr ());
    }
  else
    {
      profiler_cpu_interval = sampling_interval;
      profiler_cpu_running = status;
      if profiler_cpu_running == NOT_RUNNING
	{ error (c"Unable to start profiler timer".as_ptr ()); }
    }

  return Qt ();
});

DEFUN! ("profiler-cpu-stop", Fprofiler_cpu_stop, Sprofiler_cpu_stop,
       0, 0, 0,
       doc: "Stop the cpu profiler.  The profiler log is not affected.\nReturn non-nil if the profiler was running.",
       (),
       {
  // C 485-508's `switch` with its two live arms (TIMER_SETTIME_RUNNING is dead, see
  // docs/ports/profiler.md).  Rust's `match` needs a `_` arm for the type's other values.
  match profiler_cpu_running
    {
      NOT_RUNNING =>
	{ return Qnil (); }

      SETITIMER_RUNNING =>
	{
	  let disable: itimerval = std::mem::zeroed ();
	  setitimer (ITIMER_PROF, &disable, 0 as *mut itimerval);
	}

      _ => { }
    }

  signal (SIGPROF, SIG_IGN);
  profiler_cpu_running = NOT_RUNNING;
  return Qt ();
});

DEFUN! ("profiler-cpu-running-p",
       Fprofiler_cpu_running_p, Sprofiler_cpu_running_p,
       0, 0, 0,
       doc: "Return non-nil if cpu profiler is running.",
       (),
       {
  return if profiler_cpu_running != NOT_RUNNING { Qt () } else { Qnil () };
});

DEFUN! ("profiler-cpu-log", Fprofiler_cpu_log, Sprofiler_cpu_log,
       0, 0, 0,
       doc: "Return the current cpu profiler log.\nThe log is a hash-table mapping backtraces to counters which represent\nthe amount of time spent at those points.  Every backtrace is a vector\nof functions, where the last few elements may be nil.\n\nIf the profiler has not run since the last invocation of\n`profiler-cpu-log' (or was never run at all), return nil.  If the\nprofiler is currently running, allocate a new log for future samples\nbefore returning.",
       (),
       {
  /* Temporarily stop profiling to avoid it interfering with our data
     access.  */
  let prof_cpu: bool = profiler_cpu_running != NOT_RUNNING;
  if prof_cpu
    { Fprofiler_cpu_stop (); }

  let ret: Lisp_Object = export_log (&raw mut cpu);

  if prof_cpu
    { Fprofiler_cpu_start (profiler_cpu_interval); }

  return ret;
});

/* Extract log data to a Lisp hash table.  The log data is then erased.  */
unsafe fn export_log (plog: *mut profiler_log) -> Lisp_Object
{
  if (*plog).log.is_null ()
    { return Qnil (); }

  let log: *mut log_t = (*plog).log;
  /* The returned hash table uses `equal' as key equivalence predicate
     which is more discriminating than the `function-equal' used by
     the log but close enough, and will never confuse two distinct
     keys in the log.  */
  let h: Lisp_Object = make_hash_table (&raw const hashtest_equal, DEFAULT_HASH_SIZE as EMACS_INT,
					Weak_None);
  let mut i: c_int = 0;
  while i < (*log).size
    {
      let count: c_int = get_log_count (log, i) as c_int;
      if count > 0
	{ Fputhash (Fvector ((*log).depth as ptrdiff_t, get_key_vector (log, i)),
		    make_fixnum (count as EMACS_INT), h); }
      i += 1;
    }
  if (*plog).gc_count != 0
    { Fputhash (CALLN! (Fvector, QAutomatic_GC (), Qnil ()),
		make_fixnum ((*plog).gc_count),
		h); }
  if (*plog).discarded != 0
    { Fputhash (CALLN! (Fvector, QDiscarded_Samples (), Qnil ()),
		make_fixnum ((*plog).discarded),
		h); }
  free_profiler_log (plog);
  return h;
}

/* Memory profiler.  */

/* Hash-table log of Memory profiler.  */
static mut memory: profiler_log = profiler_log { log: std::ptr::null_mut (), gc_count: 0, discarded: 0 };

/* True if memory profiler is running.  */
#[unsafe(no_mangle)]
pub static mut profiler_memory_running: bool = false;

DEFUN! ("profiler-memory-start", Fprofiler_memory_start, Sprofiler_memory_start,
       0, 0, 0,
       doc: "Start/restart the memory profiler.\nThe memory profiler will take samples of the call-stack whenever a new\nallocation takes place.  Note that most small allocations only trigger\nthe profiler occasionally.\nSee also `profiler-log-size' and `profiler-max-stack-depth'.",
       (),
       {
  if profiler_memory_running
    { error (c"Memory profiler is already running".as_ptr ()); }

  if memory.log.is_null ()
    { memory = make_profiler_log (); }

  profiler_memory_running = true;

  return Qt ();
});

DEFUN! ("profiler-memory-stop",
       Fprofiler_memory_stop, Sprofiler_memory_stop,
       0, 0, 0,
       doc: "Stop the memory profiler.  The profiler log is not affected.\nReturn non-nil if the profiler was running.",
       (),
       {
  if !profiler_memory_running
    { return Qnil (); }
  profiler_memory_running = false;
  return Qt ();
});

DEFUN! ("profiler-memory-running-p",
       Fprofiler_memory_running_p, Sprofiler_memory_running_p,
       0, 0, 0,
       doc: "Return non-nil if memory profiler is running.",
       (),
       {
  return if profiler_memory_running { Qt () } else { Qnil () };
});

DEFUN! ("profiler-memory-log",
       Fprofiler_memory_log, Sprofiler_memory_log,
       0, 0, 0,
       doc: "Return the current memory profiler log.\nThe log is a hash-table mapping backtraces to counters which represent\nthe amount of memory allocated at those points.  Every backtrace is a vector\nof functions, where the last few elements may be nil.\n\nIf the profiler has not run since the last invocation of\n`profiler-memory-log' (or was never run at all), return nil.  If the\nprofiler is currently running, allocate a new log for future samples\nbefore returning.",
       (),
       {
  let prof_mem: bool = profiler_memory_running;
  if prof_mem
    { Fprofiler_memory_stop (); }

  let ret: Lisp_Object = export_log (&raw mut memory);

  if prof_mem
    { Fprofiler_memory_start (); }

  return ret;
});


/* Signals and probes.  */

/* Record that the current backtrace allocated SIZE bytes.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc_probe (size: usize)
{
  add_sample (&raw mut memory, min (size as EMACS_INT, MOST_POSITIVE_FIXNUM));
}

DEFUN! ("function-equal", Ffunction_equal, Sfunction_equal, 2, 2, 0,
       doc: "Return non-nil if F1 and F2 come from the same source.\nUsed to determine if different closures are just different instances of\nthe same lambda expression, or are really unrelated function.",
       (f1: Lisp_Object, f2: Lisp_Object),
       {
  let mut res: bool;
  if EQ (f1, f2)
    { res = true; }
  else if CLOSUREP (f1) && CLOSUREP (f2)
    { res = EQ (AREF (f1, CLOSURE_CODE as isize), AREF (f2, CLOSURE_CODE as isize)); }
  else
    { res = false; }
  return if res { Qt () } else { Qnil () };
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_profiler ()
{
  mark_log (cpu.log);
  mark_log (memory.log);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_profiler ()
{
  DEFVAR_INT! (c"profiler-max-stack-depth", profiler_max_stack_depth,
	       "Number of elements from the call-stack recorded in the log.");
  profiler_max_stack_depth = 16;
  DEFVAR_INT! (c"profiler-log-size", profiler_log_size,
	       "Number of distinct call-stacks that can be recorded in a profiler log.\nIf the log gets full, some of the least-seen call-stacks will be evicted\nto make room for new entries.");
  profiler_log_size = 10000;

  DEFSYM! (QDiscarded_Samples, "Discarded Samples");

  defsubr (&raw mut Sfunction_equal);

  profiler_cpu_running = NOT_RUNNING;
  defsubr (&raw mut Sprofiler_cpu_start);
  defsubr (&raw mut Sprofiler_cpu_stop);
  defsubr (&raw mut Sprofiler_cpu_running_p);
  defsubr (&raw mut Sprofiler_cpu_log);
  profiler_memory_running = false;
  defsubr (&raw mut Sprofiler_memory_start);
  defsubr (&raw mut Sprofiler_memory_stop);
  defsubr (&raw mut Sprofiler_memory_running_p);
  defsubr (&raw mut Sprofiler_memory_log);
}
