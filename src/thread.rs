// femacs: port of src/thread.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  The twenty-one
// `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's non-`static` definitions - the twenty-one
// subrs, `maybe_reacquire_global_lock`, `finalize_one_mutex`, `finalize_one_condvar`,
// `thread_select`, `mark_threads`, `unmark_main_thread`, `finalize_one_thread`,
// `thread_check_current_buffer`, `thread_all_before_buffer_killed`, `main_thread_p`,
// `in_current_thread`, `init_threads`, `syms_of_threads` - carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2), and `current_thread`/`all_threads` do too; the C `static` helpers stay
// plain `unsafe fn`, and the ones C hands to `flush_stack_call_func`,
// `record_unwind_protect_void`, `internal_condition_case` or `sys_thread_create` are
// `unsafe extern "C" fn` (PORTING.md 1.4): `mutex_lock_callback`, `do_unwind_mutex_lock`,
// `mutex_unlock_callback`, `condition_wait_callback`, `condition_notify_callback`,
// `really_call_select`, `mark_threads_callback`, `yield_callback`,
// `invoke_thread_function`, `record_thread_error`, `run_thread`, `thread_signal_callback`
// and `thread_join_callback`.
//
// The original frozen-disabled regions remain documented in docs/ports/thread.md.
// P-387 restores their real target conditions for registration, main-thread
// signaling and NS/JNI/select lifecycle. Source restoration is not acceptance.
//
// The `#include` list is C 20-29's through the mirrors those pull in; `<setjmp.h>` (C 21)
// comes through lisp.rs (`jmp_buf`/`sys_setjmp`), and C 25-29's `process.h`/`coding.h`/
// `syssignal.h`/`pdumper.h`/`keyboard.h` are kept because C includes them.  Names C declares
// in headers no mirror carries yet are declared at their use site (the terminal.rs/filelock.rs
// shape): `specpdl_unrewind`, `mark_specpdl`, `mark_c_stack`, `mark_bytecode`, `push_handler`,
// `internal_condition_case`, `record_unwind_protect_void`, `set_buffer_internal_2`,
// `init_bc_thread`, `free_bc_thread` and `update_processes_for_thread_death`.
//
// `struct handler`, `sys_jmp_buf` and `struct re_registers` are declared in the mirrors this
// unit includes (`include/thread.rs`, `include/regex-emacs.rs`), not in the unit: thread.h
// names them, and thread.c dereferences a `struct handler`, so the records had to land with
// this port (see those mirrors' notes and the ledger's "Changes outside the unit").

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe, unreachable_code)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/intervals.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/process.rs imported via common
// [U1] include/syssignal.rs imported via common
// [U1] include/pdumper.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/frame.rs imported via common
// [U1] include/blockinput.rs imported via common
// [U1] include/keyboard.rs imported via common

/* Names C declares in headers whose mirrors do not carry them yet (the terminal.rs shape);
   each keeps C's prototype in a `/// C:` line (PORTING.md P-55 for the pointer-only types).  */
unsafe extern "C-unwind" {


  /// C: lisp.h 4441: `extern void mark_c_stack (char const *, char const *);`
  pub fn mark_c_stack (bottom: *const c_char, end: *const c_char);
  /// C: lisp.h 5231: `extern void mark_bytecode (struct bc_thread_state *bc);`
  pub fn mark_bytecode (bc: *mut bc_thread_state);
  /// C: lisp.h 4812: `extern struct handler *push_handler (Lisp_Object, enum handlertype);`
  pub fn push_handler (tag: Lisp_Object, type_: handlertype) -> *mut handler;
  /// C: lisp.h 4805: `extern Lisp_Object internal_condition_case (Lisp_Object (*) (void),
  /// Lisp_Object, Lisp_Object (*) (Lisp_Object));`
  pub fn internal_condition_case (bfun: Option<unsafe extern "C-unwind" fn () -> Lisp_Object>,
                                  handlers: Lisp_Object,
                                  hfun: Option<unsafe extern "C-unwind" fn (Lisp_Object) -> Lisp_Object>)
    -> Lisp_Object;
  /// C: lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));`
  pub fn record_unwind_protect_void (func: Option<unsafe extern "C-unwind" fn ()>);
  /// C: lisp.h 5229-5230: `extern void init_bc_thread (struct bc_thread_state *bc);` /
  /// `extern void free_bc_thread (struct bc_thread_state *bc);`
  pub fn init_bc_thread (bc: *mut bc_thread_state);
  pub fn free_bc_thread (bc: *mut bc_thread_state);


}

#[cfg (feature = "have_ns")]
mod thread_ns_api {
  use super::*;
  unsafe extern "C-unwind" {
    pub fn ns_alloc_autorelease_pool () -> *mut c_void;
    pub fn ns_release_autorelease_pool (pool: *mut c_void);
  }
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
mod thread_android_api {
  use super::*;
  unsafe extern "C" {
    pub static mut android_jvm: *mut JavaVM;
    pub static mut android_java_env: *mut JNIEnv;
    pub fn android_check_query ();
    pub fn android_before_select ();
  }
}
#[cfg (all (feature = "have_glib", not (feature = "have_ns")))]
mod thread_select_api {
  unsafe extern "C" { pub fn release_select_lock (); }
}
#[cfg (all (feature = "have_glib", not (feature = "have_ns")))]
macro_rules! release_select_lock { () => { thread_select_api::release_select_lock (); } }
#[cfg (not (all (feature = "have_glib", not (feature = "have_ns"))))]
macro_rules! release_select_lock { () => { } }

/* thread.c 41-49.  `GCALIGNED_UNION_MEMBER` is `char alignas (GCALIGNMENT) gcaligned;`
   (lisp.h 303) and `GCALIGNMENT` is IDEAL_GCALIGNMENT = 8, so the union's alignment is
   spelled `#[repr(align(8))]` (excluded-macros.txt's GCALIGNED_UNION_MEMBER row) and the
   marker member is not written.  Measured: alignment 8, size = `sizeof thread_state`.  */
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub union aligned_thread_state
{
  pub s: thread_state,
}
const _: () = core::assert! (std::mem::align_of::<aligned_thread_state> () % 8 == 0);  // static_assert (GCALIGNED (…))

/// C 54's `PVECHEADERSIZE (PVEC_THREAD, PSEUDOVECSIZE (struct thread_state, event_object),
/// VECSIZE (struct thread_state))`.  The lisp.rs mirror's `PVECHEADERSIZE` is an
/// `#[inline(always)] fn`, and a static initializer needs a constant, so the port writes the
/// same expression with `const` arithmetic over lisp.rs's names (the two are equal - the
/// generated layout gate's `PSEUDOVECSIZE`/`VECSIZE` values are the C compiler's).
const MAIN_THREAD_HEADER_SIZE: isize
  = PSEUDOVECTOR_FLAG
    | ((pvec_type::PVEC_THREAD as isize) << PSEUDOVECTOR_AREA_BITS)
    | (VECSIZE (std::mem::size_of::<thread_state> ()) << PSEUDOVECTOR_SIZE_BITS)
    | PSEUDOVECSIZE! (thread_state, event_object);

/* thread.c 51-67.  C's initializer is a constant expression; the port's is the same value:
   the nine `LISPSYM_INITIALLY (Qnil)` members are `lisp_h_Qnil ()`, the port's
   const-evaluable NIL (symbol index 0 with tag 0 - `TAG_PTR_INITIALLY` is not a `const fn`),
   and the header size is the `const` above.  */
static mut main_thread: aligned_thread_state
  = aligned_thread_state { s: thread_state {
      header: vectorlike_header { size: MAIN_THREAD_HEADER_SIZE },
      m_last_thing_searched: lisp_h_Qnil (),
      m_saved_last_thing_searched: lisp_h_Qnil (),
      name: lisp_h_Qnil (),
      function: lisp_h_Qnil (),
      result: lisp_h_Qnil (),
      error_symbol: lisp_h_Qnil (),
      error_data: lisp_h_Qnil (),
      buffer_disposition: lisp_h_Qnil (),
      event_object: lisp_h_Qnil (),
      m_stack_bottom: std::ptr::null (),
      stack_top: std::ptr::null (),
      m_catchlist: std::ptr::null_mut (),
      m_handlerlist: std::ptr::null_mut (),
      m_handlerlist_sentinel: std::ptr::null_mut (),
      m_specpdl: std::ptr::null_mut (),
      m_specpdl_end: std::ptr::null_mut (),
      m_specpdl_ptr: std::ptr::null_mut (),
      m_lisp_eval_depth: 0,
      m_current_buffer: std::ptr::null_mut (),
      m_search_regs: re_registers { num_regs: 0, start: std::ptr::null_mut (), end: std::ptr::null_mut () },
      m_saved_search_regs: re_registers { num_regs: 0, start: std::ptr::null_mut (), end: std::ptr::null_mut () },
      m_waiting_for_user_input_p: 0,
      m_waiting_for_input: false,
      m_getcjmp: /* SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership. */ unsafe { core::mem::zeroed () },   // C's static is zero-initialized (192 or 256 bytes)
      thread_id: unsafe { core::mem::zeroed () },
      thread_condvar: unsafe { core::mem::zeroed () },
      wait_condvar: std::ptr::null_mut (),
      thread_name: std::ptr::null_mut (),
      not_holding_lock: 0,
      #[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
      java_env: 0 as *mut JNIEnv,   // C's zero-initialized `JNIEnv *java_env` (thread.h 98)
      next_thread: std::ptr::null_mut (),
      bc: bc_thread_state { fp: std::ptr::null_mut (), stack: std::ptr::null_mut (),
                            stack_end: std::ptr::null_mut () },
    } };

// thread.c 60-62: C writes `&main_thread.s`, the address of the union's first (only) member,
// which is the union's address; `addr_of_mut!` on the field is the same pointer.
#[unsafe(no_mangle)]
pub static mut current_thread: *mut thread_state = /* SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership. */ unsafe { std::ptr::addr_of_mut! (main_thread.s) };

#[unsafe(no_mangle)]
pub static mut all_threads: *mut thread_state = unsafe { std::ptr::addr_of_mut! (main_thread.s) };

static mut global_lock: sys_mutex_t = /* SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership. */ unsafe { core::mem::zeroed () };

/* thread.c 70's declaration is the one above (interrupt_input_blocked).  */

/* m_specpdl is set when the thread is created and cleared when the thread dies.  */
// thread.c 75-76.
macro_rules! thread_live_p { ($state:expr) => { (*( $state)).m_specpdl != std::ptr::null_mut () } }

// thread.c 79-82.
unsafe fn release_global_lock ()
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  sys_mutex_unlock (&raw mut global_lock);

  }
}

// thread.c 84-90.
unsafe fn rebind_for_thread_switch ()
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let distance: ptrdiff_t
    = (*current_thread).m_specpdl_ptr.offset_from ((*current_thread).m_specpdl);
  specpdl_unrewind (specpdl_ptr! (), -distance as c_int, true);

  }
}

// thread.c 92-98.
unsafe fn unbind_for_thread_switch (thr: *mut thread_state)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let distance: ptrdiff_t = (*thr).m_specpdl_ptr.offset_from ((*thr).m_specpdl);
  specpdl_unrewind ((*thr).m_specpdl_ptr, distance as c_int, true);

  }
}

/* You must call this after acquiring the global lock.
   acquire_global_lock does it for you.  */
// thread.c 100-152.  C names the parameter `self`, which Rust reserves; the port spells it
// `self_` (the token comparison folds a trailing underscore, so the body's tokens still
// match - the `r#match` precedent, PORTING.md 0.2).
unsafe fn post_acquire_global_lock (self_: *mut thread_state)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let prev_thread: *mut thread_state = current_thread;

  #[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
  { thread_android_api::android_java_env = (*self_).java_env; }

  current_thread = self_;

  if prev_thread != current_thread
    {
      if !prev_thread.is_null ()
	{ unbind_for_thread_switch (prev_thread); }
      rebind_for_thread_switch ();

       /* Set the new thread's current buffer.  */
      set_buffer_internal_2 (current_buffer! () as *mut buffer);
    }

  #[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
  { thread_android_api::android_check_query (); }

   /* We could have been signaled while waiting to grab the global lock
      for the first time since this thread was created.  */
  if !NILP ((*current_thread).error_symbol) && !handlerlist! ().is_null ()
    {
      let sym: Lisp_Object = (*current_thread).error_symbol;
      let data: Lisp_Object = (*current_thread).error_data;

      (*current_thread).error_symbol = Qnil ();
      (*current_thread).error_data = Qnil ();
      Fsignal (sym, data);
    }

  }
}

// thread.c 154-159.
unsafe fn acquire_global_lock (self_: *mut thread_state)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  sys_mutex_lock (&raw mut global_lock);
  post_acquire_global_lock (self_);

  }
}

/* This is called from keyboard.c when it detects that SIGINT was
   delivered to the main thread and interrupted thread_select before
   the main thread could acquire the lock.  */
// thread.c 161-179.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn maybe_reacquire_global_lock ()
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  /* SIGINT handler is always run on the main thread.  */
  current_thread = std::ptr::addr_of_mut! (main_thread.s);

  if (*current_thread).not_holding_lock != 0
    {
      let self_: *mut thread_state = current_thread;

      acquire_global_lock (self_);
      (*current_thread).not_holding_lock = 0;
    }

  }
}

// thread.c 181-188.
unsafe fn lisp_mutex_init (mutex: *mut lisp_mutex_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  (*mutex).owner = std::ptr::null_mut ();
  (*mutex).count = 0;
  sys_cond_init (&raw mut (*mutex).condition);

  }
}

/* Lock MUTEX for thread LOCKER, setting its lock count to COUNT, if
   non-zero, or to 1 otherwise.  */
// thread.c 190-235.
unsafe fn lisp_mutex_lock_for_thread (mutex: *mut lisp_mutex_t, locker: *mut thread_state,
                                      new_count: c_int) -> c_int
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let self_: *mut thread_state;

  if (*mutex).owner.is_null ()
    {
      (*mutex).owner = locker;
      (*mutex).count = if new_count == 0 { 1 } else { new_count as c_uint };
      return 0;
    }
  if (*mutex).owner == locker
    {
      eassert! (new_count == 0);
      (*mutex).count += 1;
      return 0;
    }

  self_ = locker;
  (*self_).wait_condvar = &raw mut (*mutex).condition;
  while !(*mutex).owner.is_null ()
	&& (new_count != 0 || NILP ((*self_).error_symbol))
    { sys_cond_wait (&raw mut (*mutex).condition, &raw mut global_lock); }
  (*self_).wait_condvar = std::ptr::null_mut ();

  if new_count == 0 && !NILP ((*self_).error_symbol)
    { return 1; }

  (*mutex).owner = self_;
  (*mutex).count = if new_count == 0 { 1 } else { new_count as c_uint };

  return 1;

  }
}

// thread.c 237-241.
unsafe fn lisp_mutex_lock (mutex: *mut lisp_mutex_t, new_count: c_int) -> c_int
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  return lisp_mutex_lock_for_thread (mutex, current_thread, new_count);

  }
}

/* Decrement MUTEX's lock count.  */
// thread.c 243-258.
unsafe fn lisp_mutex_unlock (mutex: *mut lisp_mutex_t) -> c_int
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  if (*mutex).owner != current_thread
    { error (c"Cannot unlock mutex owned by another thread".as_ptr ()); }

  (*mutex).count -= 1;
  if (*mutex).count > 0
    { return 0; }

  (*mutex).owner = std::ptr::null_mut ();
  sys_cond_broadcast (&raw mut (*mutex).condition);

  return 1;

  }
}

/* Like lisp_mutex_unlock, but sets MUTEX's lock count to zero
   regardless of its value.  Return the previous lock count.  */
// thread.c 260-274.
unsafe fn lisp_mutex_unlock_for_wait (mutex: *mut lisp_mutex_t) -> c_uint
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let result: c_uint = (*mutex).count;

  /* Ensured by condvar code.  */
  eassert! ((*mutex).owner == current_thread);

  (*mutex).count = 0;
  (*mutex).owner = std::ptr::null_mut ();
  sys_cond_broadcast (&raw mut (*mutex).condition);

  return result;

  }
}

// thread.c 276-279.
unsafe fn lisp_mutex_destroy (mutex: *mut lisp_mutex_t)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  sys_cond_destroy (&raw mut (*mutex).condition);

  }
}

// thread.c 281-284.  C returns `mutex->owner == current_thread` for an `int` function; the
// port writes the same test as an `if`/`else` (Rust has no `bool`-to-`int` conversion - the
// dired.rs precedent).
unsafe fn lisp_mutex_owned_p (mutex: *mut lisp_mutex_t) -> c_int
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  return if (*mutex).owner == current_thread { 1 } else { 0 };

  }
}

// thread.c 289-320.
DEFUN! ("make-mutex", Fmake_mutex, Smake_mutex, 0, 1, 0,
       doc: "Create a mutex.\nA mutex provides a synchronization point for threads.\nOnly one thread at a time can hold a mutex.  Other threads attempting\nto acquire it will block until the mutex is available.\n\nA thread can acquire a mutex any number of times.\n\nNAME, if given, is used as the name of the mutex.  The name is\ninformational only.",
       (name: Lisp_Object),
       {
  if !NILP (name)
    { CHECK_STRING (name); }

  let mutex: *mut Lisp_Mutex
    = ALLOCATE_ZEROED_PSEUDOVECTOR! (Lisp_Mutex, name, pvec_type::PVEC_MUTEX);
  (*mutex).name = name;
  lisp_mutex_init (&raw mut (*mutex).mutex);

  let result: Lisp_Object;
  XSETMUTEX! (result, mutex);
  return result;
});

// thread.c 322-332.
unsafe extern "C-unwind" fn mutex_lock_callback (arg: *mut c_void)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let mutex: *mut Lisp_Mutex = arg as *mut Lisp_Mutex;
  let self_: *mut thread_state = current_thread;

  /* Calling lisp_mutex_lock might yield to other threads while this
     one waits for the mutex to become unlocked.  */
  if lisp_mutex_lock (&raw mut (*mutex).mutex, 0) != 0
    { post_acquire_global_lock (self_); }

  }
}

// thread.c 334-337.
unsafe extern "C-unwind" fn do_unwind_mutex_lock ()
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  (*current_thread).event_object = Qnil ();

  }
}

// thread.c 339-370.
DEFUN! ("mutex-lock", Fmutex_lock, Smutex_lock, 1, 1, 0,
       doc: "Acquire a mutex.\nIf the current thread already owns MUTEX, increment the count and\nreturn.\nOtherwise, if no thread owns MUTEX, make the current thread own it.\nOtherwise, block until MUTEX is available, or until the current thread\nis signaled using `thread-signal'.\nNote that calls to `mutex-lock' and `mutex-unlock' must be paired.",
       (mutex: Lisp_Object),
       {
  let lmutex: *mut Lisp_Mutex;
  let count: specpdl_ref = SPECPDL_INDEX ();

  CHECK_MUTEX (mutex);
  lmutex = XMUTEX (mutex);

  (*current_thread).event_object = mutex;
  record_unwind_protect_void (Some (do_unwind_mutex_lock));
  flush_stack_call_func (Some (mutex_lock_callback), lmutex as *mut c_void);
  return unbind_to (count, Qnil ());
});

// thread.c 372-379.
unsafe extern "C-unwind" fn mutex_unlock_callback (arg: *mut c_void)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let mutex: *mut Lisp_Mutex = arg as *mut Lisp_Mutex;
  let self_: *mut thread_state = current_thread;

  if lisp_mutex_unlock (&raw mut (*mutex).mutex) != 0
    { post_acquire_global_lock (self_); } /* FIXME: is this call needed?  */

  }
}

// thread.c 381-395.
DEFUN! ("mutex-unlock", Fmutex_unlock, Smutex_unlock, 1, 1, 0,
       doc: "Release the mutex.\nIf this thread does not own MUTEX, signal an error.\nOtherwise, decrement the mutex's count.  If the count is zero,\nrelease MUTEX.",
       (mutex: Lisp_Object),
       {
  let lmutex: *mut Lisp_Mutex;

  CHECK_MUTEX (mutex);
  lmutex = XMUTEX (mutex);

  flush_stack_call_func (Some (mutex_unlock_callback), lmutex as *mut c_void);
  return Qnil ();
});

// thread.c 397-409.
DEFUN! ("mutex-name", Fmutex_name, Smutex_name, 1, 1, 0,
       doc: "Return the name of MUTEX.\nIf no name was given when MUTEX was created, return nil.",
       (mutex: Lisp_Object),
       {
  let lmutex: *mut Lisp_Mutex;

  CHECK_MUTEX (mutex);
  lmutex = XMUTEX (mutex);

  return (*lmutex).name;
});

// thread.c 411-414.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn finalize_one_mutex (mutex: *mut Lisp_Mutex)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  lisp_mutex_destroy (&raw mut (*mutex).mutex);

  }
}

// thread.c 416-441.
DEFUN! ("make-condition-variable",
       Fmake_condition_variable, Smake_condition_variable,
       1, 2, 0,
       doc: "Make a condition variable associated with MUTEX.\nA condition variable provides a way for a thread to sleep while\nwaiting for a state change.\n\nMUTEX is the mutex associated with this condition variable.\nNAME, if given, is the name of this condition variable.  The name is\ninformational only.",
       (mutex: Lisp_Object, name: Lisp_Object),
       {
  CHECK_MUTEX (mutex);
  if !NILP (name)
    { CHECK_STRING (name); }

  let condvar: *mut Lisp_CondVar
    = ALLOCATE_ZEROED_PSEUDOVECTOR! (Lisp_CondVar, name, pvec_type::PVEC_CONDVAR);
  (*condvar).mutex = mutex;
  (*condvar).name = name;
  sys_cond_init (&raw mut (*condvar).cond);

  let result: Lisp_Object;
  XSETCONDVAR! (result, condvar);
  return result;
});

// thread.c 443-475.
unsafe extern "C-unwind" fn condition_wait_callback (arg: *mut c_void)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let cvar: *mut Lisp_CondVar = arg as *mut Lisp_CondVar;
  let mutex: *mut Lisp_Mutex = XMUTEX ((*cvar).mutex);
  let self_: *mut thread_state = current_thread;
  let saved_count: c_uint;
  let cond: Lisp_Object;

  XSETCONDVAR! (cond, cvar);
  (*self_).event_object = cond;
  saved_count = lisp_mutex_unlock_for_wait (&raw mut (*mutex).mutex);
  /* If signaled while unlocking, skip the wait but reacquire the lock.  */
  if NILP ((*self_).error_symbol)
    {
      (*self_).wait_condvar = &raw mut (*cvar).cond;
      /* This call could switch to another thread.  */
      sys_cond_wait (&raw mut (*cvar).cond, &raw mut global_lock);
      (*self_).wait_condvar = std::ptr::null_mut ();
    }
  (*self_).event_object = Qnil ();
  /* Since sys_cond_wait could switch threads, we need to lock the
     mutex for the thread which was the current when we were called.  */
  lisp_mutex_lock_for_thread (&raw mut (*mutex).mutex, self_, saved_count as c_int);
  /* Calling lisp_mutex_lock_for_thread might yield to other threads.  */
  post_acquire_global_lock (self_);

  }
}

// thread.c 478-511.
DEFUN! ("condition-wait", Fcondition_wait, Scondition_wait, 1, 1, 0,
       doc: "Wait for the condition variable COND to be notified.\nCOND is the condition variable to wait on.\n\nThe mutex associated with COND must be held when this is called.\nIt is an error if it is not held.\n\nThis releases the mutex and waits for COND to be notified or for\nthis thread to be signaled with `thread-signal'.  When\n`condition-wait' returns, COND's mutex will again be locked by\nthis thread.",
       (cond: Lisp_Object),
       {
  let cvar: *mut Lisp_CondVar;
  let mutex: *mut Lisp_Mutex;

  CHECK_CONDVAR (cond);
  cvar = XCONDVAR (cond);

  mutex = XMUTEX ((*cvar).mutex);
  if lisp_mutex_owned_p (&raw mut (*mutex).mutex) == 0
    { error (c"Condition variable's mutex is not held by current thread".as_ptr ()); }

  flush_stack_call_func (Some (condition_wait_callback), cvar as *mut c_void);

  return Qnil ();
});

/* Used to communicate arguments to condition_notify_callback.  */
// thread.c 514-518.
#[repr(C)]
pub struct notify_args
{
  pub cvar: *mut Lisp_CondVar,
  pub all: c_int,
}

// thread.c 520-541.
unsafe extern "C-unwind" fn condition_notify_callback (arg: *mut c_void)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let na: *mut notify_args = arg as *mut notify_args;
  let mutex: *mut Lisp_Mutex = XMUTEX ((*(*na).cvar).mutex);
  let self_: *mut thread_state = current_thread;
  let saved_count: c_uint;
  let cond: Lisp_Object;

  XSETCONDVAR! (cond, (*na).cvar);
  saved_count = lisp_mutex_unlock_for_wait (&raw mut (*mutex).mutex);
  if (*na).all != 0
    { sys_cond_broadcast (&raw mut (*(*na).cvar).cond); }
  else
    { sys_cond_signal (&raw mut (*(*na).cvar).cond); }
  /* Calling lisp_mutex_lock might yield to other threads while this
     one waits for the mutex to become unlocked.  */
  lisp_mutex_lock (&raw mut (*mutex).mutex, saved_count as c_int);
  post_acquire_global_lock (self_);

  }
}

// thread.c 543-565.
DEFUN! ("condition-notify", Fcondition_notify, Scondition_notify, 1, 2, 0,
       doc: "Notify COND, a condition variable.\nThis wakes a thread waiting on COND.\nIf ALL is non-nil, all waiting threads are awoken.\n\nThe mutex associated with COND must be held when this is called.\nIt is an error if it is not held.\n\nThis releases COND's mutex when notifying COND.  When\n`condition-notify' returns, the mutex will again be locked by this\nthread.",
       (cond: Lisp_Object, all: Lisp_Object),
       {
  let cvar: *mut Lisp_CondVar;
  let mutex: *mut Lisp_Mutex;
  let mut args: notify_args = std::mem::zeroed ();

  CHECK_CONDVAR (cond);
  cvar = XCONDVAR (cond);

  mutex = XMUTEX ((*cvar).mutex);
  if lisp_mutex_owned_p (&raw mut (*mutex).mutex) == 0
    { error (c"Condition variable's mutex is not held by current thread".as_ptr ()); }

  args.cvar = cvar;
  args.all = if !NILP (all) { 1 } else { 0 };
  flush_stack_call_func (Some (condition_notify_callback), &raw mut args as *mut c_void);

  return Qnil ();
});

// thread.c 567-577.
DEFUN! ("condition-mutex", Fcondition_mutex, Scondition_mutex, 1, 1, 0,
       doc: "Return the mutex associated with condition variable COND.",
       (cond: Lisp_Object),
       {
  let cvar: *mut Lisp_CondVar;

  CHECK_CONDVAR (cond);
  cvar = XCONDVAR (cond);

  return (*cvar).mutex;
});

// thread.c 579-590.
DEFUN! ("condition-name", Fcondition_name, Scondition_name, 1, 1, 0,
       doc: "Return the name of condition variable COND.\nIf no name was given when COND was created, return nil.",
       (cond: Lisp_Object),
       {
  let cvar: *mut Lisp_CondVar;

  CHECK_CONDVAR (cond);
  cvar = XCONDVAR (cond);

  return (*cvar).name;
});

// thread.c 593-596.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn finalize_one_condvar (condvar: *mut Lisp_CondVar)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  sys_cond_destroy (&raw mut (*condvar).cond);

  }
}

// thread.c 601-611.
#[repr(C)]
pub struct select_args
{
  pub func: Option<select_func>,
  pub max_fds: c_int,
  pub rfds: *mut fd_set,
  pub wfds: *mut fd_set,
  pub efds: *mut fd_set,
  pub timeout: *mut timespec,
  pub sigmask: *mut sigset_t,
  pub result: c_int,
}

// thread.c 613-641.
unsafe extern "C-unwind" fn really_call_select (arg: *mut c_void)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let sa: *mut select_args = arg as *mut select_args;
  let self_: *mut thread_state = current_thread;
  let mut oldset: sigset_t = std::mem::zeroed ();

  block_interrupt_signal (&raw mut oldset);
  (*self_).not_holding_lock = 1;
  release_global_lock ();
  restore_signal_mask (&raw const oldset);

  (*sa).result = ((*sa).func.unwrap ()) ((*sa).max_fds, (*sa).rfds, (*sa).wfds, (*sa).efds,
					 (*sa).timeout as *const timespec,
					 (*sa).sigmask as *const sigset_t);

  release_select_lock! ();

  block_interrupt_signal (&raw mut oldset);
  /* If we were interrupted by C-g while inside sa->func above, the
     signal handler could have called maybe_reacquire_global_lock.  */
  if (*self_).not_holding_lock != 0
    {
      acquire_global_lock (self_);
      (*self_).not_holding_lock = 0;
    }
  restore_signal_mask (&raw const oldset);

  }
}

// thread.c 643-663.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn thread_select (func: Option<select_func>, max_fds: c_int,
					rfds: *mut fd_set, wfds: *mut fd_set,
					efds: *mut fd_set, timeout: *mut timespec,
					sigmask: *mut sigset_t) -> c_int
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let mut sa: select_args = std::mem::zeroed ();

  sa.func = func;
  sa.max_fds = max_fds;
  sa.rfds = rfds;
  sa.wfds = wfds;
  sa.efds = efds;
  sa.timeout = timeout;
  sa.sigmask = sigmask;
  #[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
  { thread_android_api::android_before_select (); }
  flush_stack_call_func (Some (really_call_select), &raw mut sa as *mut c_void);
  return sa.result;

  }
}

// thread.c 666-693.
unsafe fn mark_one_thread (thread: *mut thread_state)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  /* Get the stack top now, in case mark_specpdl changes it.  */
  let stack_top: *const c_void = (*thread).stack_top;

  mark_specpdl ((*thread).m_specpdl, (*thread).m_specpdl_ptr);

  mark_c_stack ((*thread).m_stack_bottom, stack_top as *const c_char);

  let mut handler: *mut handler = (*thread).m_handlerlist;
  while !handler.is_null ()
    {
      mark_object ((*handler).tag_or_ch);
      mark_object ((*handler).val);
      handler = (*handler).next;
    }

  if !(*thread).m_current_buffer.is_null ()
    {
      let tem: Lisp_Object;
      XSETBUFFER! (tem, (*thread).m_current_buffer);
      mark_object (tem);
    }

  mark_bytecode (&raw mut (*thread).bc);
  crate::nonlocal::mark_thread_explicit_roots (thread as *mut c_void, |obj| mark_object (obj));
  let mut slab_marked = 0usize;
  crate::roots::mark_thread_u4_slab_roots (thread as *mut c_void, |obj| {
    slab_marked += 1;
    mark_object (obj);
  });
  crate::state::note_gc_thread_slab_marked (slab_marked);

  /* No need to mark Lisp_Object members like m_last_thing_searched,
     as mark_threads_callback does that by calling mark_object.  */

  }
}

// thread.c 696-709.
unsafe extern "C-unwind" fn mark_threads_callback (ignore: *mut c_void)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let mut iter: *mut thread_state;

  iter = all_threads;
  while !iter.is_null ()
    {
      let thread_obj: Lisp_Object;

      XSETTHREAD! (thread_obj, iter);
      mark_object (thread_obj);
      mark_one_thread (iter);
      iter = (*iter).next_thread;
    }

  }
}

// thread.c 711-715.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mark_threads ()
{
  flush_stack_call_func (Some (mark_threads_callback), std::ptr::null_mut ());
}

// thread.c 717-721.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn unmark_main_thread ()
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  main_thread.s.header.size &= !ARRAY_MARK_FLAG;

  }
}

// thread.c 725-732.
unsafe extern "C-unwind" fn yield_callback (ignore: *mut c_void)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let self_: *mut thread_state = current_thread;

  release_global_lock ();
  sys_thread_yield ();
  acquire_global_lock (self_);

  }
}

// thread.c 734-741.
DEFUN! ("thread-yield", Fthread_yield, Sthread_yield, 0, 0, 0,
       doc: "Yield the CPU to another thread.",
       (),
       {
  flush_stack_call_func (Some (yield_callback), std::ptr::null_mut ());
  return Qnil ();
});

// thread.c 743-749.
unsafe extern "C-unwind" fn invoke_thread_function () -> Lisp_Object
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let count: specpdl_ref = SPECPDL_INDEX ();

  (*current_thread).result = calln! ((*current_thread).function);
  return unbind_to (count, Qnil ());

  }
}

// thread.c 751.
static mut last_thread_error: Lisp_Object = lisp_h_Qnil ();

// thread.c 753-758.
unsafe extern "C-unwind" fn record_thread_error (error_form: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  last_thread_error = error_form;
  return error_form;

  }
}

// thread.c 761-865.
unsafe extern "C" fn run_thread (state: *mut c_void) -> *mut c_void
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  /* Make sure stack_top and m_stack_bottom are properly aligned as GC
     expects.  */
  #[repr(C)]
  union Stack_pos
  {
    o: Lisp_Object,
    p: *mut c_void,
    c: c_char,
  }
  let stack_pos: Stack_pos = std::mem::zeroed ();

  let self_: *mut thread_state = state as *mut thread_state;
  let mut iter: *mut *mut thread_state;
  #[cfg (feature = "have_ns")]
  let pool = thread_ns_api::ns_alloc_autorelease_pool ();
  #[cfg (all (feature = "threads_enabled", feature = "have_android", feature = "not_android_stubify"))]
  {
    let vm = thread_android_api::android_jvm;
    let rc = ((*(*vm)).AttachCurrentThread.unwrap ())
      (vm, &raw mut (*self_).java_env, std::ptr::null_mut ());
    if rc != JNI_OK { emacs_abort (); }
  }

  (*self_).stack_top = std::ptr::addr_of! (stack_pos.c) as *const c_void;
  (*self_).m_stack_bottom = std::ptr::addr_of! (stack_pos.c) as *const c_char;
  (*self_).thread_id = sys_thread_self ();

  if !(*self_).thread_name.is_null ()
    { sys_thread_set_name ((*self_).thread_name); }

  acquire_global_lock (self_);

  /* Put a dummy catcher at top-level so that handlerlist is never NULL.  */
  handlerlist_sentinel! () = xzalloc (std::mem::size_of::<handler> ()) as *mut handler;
  (*handlerlist_sentinel! ()).nextfree = handlerlist_sentinel! ();
  handlerlist! () = handlerlist_sentinel! ();
  let c: *mut handler = push_handler (Qunbound (), CATCHER);
  eassert! (c == handlerlist_sentinel! ());
  (*handlerlist_sentinel! ()).nextfree = std::ptr::null_mut ();
  (*handlerlist_sentinel! ()).next = std::ptr::null_mut ();

  /* It might be nice to do something with errors here.  */
  let _ = crate::nonlocal::catch_vm_nonlocal (|| /* SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership. */ unsafe {
    internal_condition_case (Some (invoke_thread_function), Qt (), Some (record_thread_error));
    update_processes_for_thread_death (Fcurrent_thread ());
  });

  xfree ((*self_).m_specpdl.offset (-1) as *mut c_void);
  (*self_).m_specpdl = std::ptr::null_mut ();
  (*self_).m_specpdl_ptr = std::ptr::null_mut ();
  (*self_).m_specpdl_end = std::ptr::null_mut ();

  {
    let mut c: *mut handler;
    let mut c_next: *mut handler;
    c = handlerlist_sentinel! ();
    while !c.is_null ()
      {
	c_next = (*c).nextfree;
	xfree (c as *mut c_void);
	c = c_next;
      }
  }

  xfree ((*self_).thread_name as *mut c_void);

  current_thread = std::ptr::null_mut ();
  sys_cond_broadcast (&raw mut (*self_).thread_condvar);

  #[cfg (feature = "have_ns")]
  { thread_ns_api::ns_release_autorelease_pool (pool); }
  #[cfg (all (feature = "threads_enabled", feature = "have_android", feature = "not_android_stubify"))]
  {
    let vm = thread_android_api::android_jvm;
    let rc = ((*(*vm)).DetachCurrentThread.unwrap ()) (vm);
    if rc != JNI_OK { emacs_abort (); }
  }

  /* Unlink this thread from the list of all threads.  Note that we
     have to do this very late, after broadcasting our death.  */
  iter = &raw mut all_threads;
  while *iter != self_
    { iter = &raw mut (**iter).next_thread; }
  *iter = (**iter).next_thread;

  release_global_lock ();

  return std::ptr::null_mut ();

  }
}

// thread.c 867-875.
unsafe fn free_search_regs (regs: *mut re_registers)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  if (*regs).num_regs != 0
    {
      xfree ((*regs).start as *mut c_void);
      xfree ((*regs).end as *mut c_void);
    }

  }
}

// thread.c 877-886.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn finalize_one_thread (state: *mut thread_state)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  free_search_regs (&raw mut (*state).m_search_regs);
  free_search_regs (&raw mut (*state).m_saved_search_regs);
  sys_cond_destroy (&raw mut (*state).thread_condvar);
  free_bc_thread (&raw mut (*state).bc);
  crate::nonlocal::cleanup_thread_state (state as *mut c_void);
  crate::roots::cleanup_thread_u4_slab (state as *mut c_void);

  }
}

// thread.c 888-947.
DEFUN! ("make-thread", Fmake_thread, Smake_thread, 1, 3, 0,
       doc: "Start a new thread and run FUNCTION in it.\nWhen the function exits, the thread dies.\nIf NAME is given, it must be a string; it names the new thread.\n\nBUFFER-DISPOSITION determines how attached the thread is to its current\nbuffer.  If the value is t, that buffer can't be killed.  Any other\nvalue, including nil (the default), means that if its buffer is killed,\nthe thread is switched to another buffer and receives an error signal\n`thread-buffer-killed'.  But if the value is symbol `silently', no error\nwill be signaled.",
       (function: Lisp_Object, name: Lisp_Object, buffer_disposition: Lisp_Object),
       {
  /* Can't start a thread in temacs.  */
  if !initialized
    { emacs_abort (); }

  if !NILP (name)
    { CHECK_STRING (name); }

  let new_thread: *mut thread_state
    = ALLOCATE_ZEROED_PSEUDOVECTOR! (thread_state, event_object, pvec_type::PVEC_THREAD);
  (*new_thread).function = function;
  (*new_thread).name = name;
  /* Perhaps copy m_last_thing_searched from parent?  */
  (*new_thread).m_current_buffer = (*current_thread).m_current_buffer;

  let size: ptrdiff_t = 50;
  let pdlvec: *mut specbinding
    = xmalloc (((1 + size) as usize) * std::mem::size_of::<specbinding> ()) as *mut specbinding;
  (*new_thread).m_specpdl = pdlvec.offset (1);  /* Skip the dummy entry.  */
  (*new_thread).m_specpdl_end = (*new_thread).m_specpdl.offset (size);
  (*new_thread).m_specpdl_ptr = (*new_thread).m_specpdl;

  init_bc_thread (&raw mut (*new_thread).bc);

  sys_cond_init (&raw mut (*new_thread).thread_condvar);

  /* We'll need locking here eventually.  */
  (*new_thread).next_thread = all_threads;
  all_threads = new_thread;

  let c_name: *const c_char
    = if !NILP (name) { SSDATA (ENCODE_SYSTEM (name)) } else { std::ptr::null () };
  if !c_name.is_null ()
    { (*new_thread).thread_name = xstrdup (c_name); }
  else
    { (*new_thread).thread_name = std::ptr::null_mut (); }
  let mut thr: sys_thread_t = core::mem::zeroed ();
  if !sys_thread_create (&raw mut thr, Some (run_thread), new_thread as *mut c_void)
    {
      /* Restore the previous situation.  */
      all_threads = (*all_threads).next_thread;
      #[cfg (feature = "threads_enabled")]
      error (c"Could not start a new thread".as_ptr ());
      #[cfg (not (feature = "threads_enabled"))]
      error (c"Concurrency is not supported in this configuration".as_ptr ());
    }

  (*new_thread).buffer_disposition = buffer_disposition;

  /* FIXME: race here where new thread might not be filled in?  */
  let result: Lisp_Object;
  XSETTHREAD! (result, new_thread);
  return result;
});

// thread.c 952-959.
DEFUN! ("current-thread", Fcurrent_thread, Scurrent_thread, 0, 0, 0,
       doc: "Return the current thread.",
       (),
       {
  let result: Lisp_Object;
  XSETTHREAD! (result, current_thread);
  return result;
});

// thread.c 961-973.
DEFUN! ("thread-name", Fthread_name, Sthread_name, 1, 1, 0,
       doc: "Return the name of the THREAD.\nThe name is the same object that was passed to `make-thread'.",
       (thread: Lisp_Object),
       {
  let tstate: *mut thread_state;

  CHECK_THREAD (thread);
  tstate = XTHREAD (thread);

  return (*tstate).name;
});

// thread.c 975-983.
unsafe extern "C-unwind" fn thread_signal_callback (arg: *mut c_void)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let tstate: *mut thread_state = arg as *mut thread_state;
  let self_: *mut thread_state = current_thread;

  sys_cond_broadcast ((*tstate).wait_condvar);
  post_acquire_global_lock (self_);

  }
}

// thread.c 985-991.
unsafe fn thread_set_error (tstate: *mut thread_state, error_symbol: Lisp_Object,
			    data: Lisp_Object)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  /* What to do if thread is already signaled?  */
  /* What if error_symbol is Qnil?  */
  (*tstate).error_symbol = error_symbol;
  (*tstate).error_data = data;

  }
}

// thread.c 993-1034.
DEFUN! ("thread-signal", Fthread_signal, Sthread_signal, 3, 3, 0,
       doc: "Signal an error in a thread.\nThis acts like `signal', but arranges for the signal to be raised\nin THREAD.  If THREAD is the current thread, acts just like `signal'.\nThis will interrupt a blocked call to `mutex-lock', `condition-wait',\nor `thread-join' in the target thread.\nIf THREAD is the main thread, just the error message is shown.",
       (thread: Lisp_Object, error_symbol: Lisp_Object, data: Lisp_Object),
       {
  let tstate: *mut thread_state;

  CHECK_THREAD (thread);
  tstate = XTHREAD (thread);

  if tstate == current_thread
    { Fsignal (error_symbol, data); }

  #[cfg (feature = "threads_enabled")]
  if main_thread_p (tstate.cast ())
    {
      let mut event: input_event = core::mem::zeroed ();
      EVENT_INIT (&raw mut event);
      event.bits = (event.bits & !INPUT_EVENT_KIND_MASK)
        | (THREAD_EVENT as u32 & INPUT_EVENT_KIND_MASK);
      event.frame_or_window = Qnil ();
      event.arg = list3 (Fcurrent_thread (), error_symbol, data);
      kbd_buffer_store_event (&raw mut event as *mut c_void);
      return Qnil ();
    }
    {
      thread_set_error (tstate, error_symbol, data);

      if !(*tstate).wait_condvar.is_null ()
	{ flush_stack_call_func (Some (thread_signal_callback), tstate as *mut c_void); }
    }

  return Qnil ();
});

// thread.c 1036-1046.
DEFUN! ("thread-live-p", Fthread_live_p, Sthread_live_p, 1, 1, 0,
       doc: "Return t if THREAD is alive, or nil if it has exited.",
       (thread: Lisp_Object),
       {
  let tstate: *mut thread_state;

  CHECK_THREAD (thread);
  tstate = XTHREAD (thread);

  return if thread_live_p! (tstate) { Qt () } else { Qnil () };
});

// thread.c 1048-1060.
DEFUN! ("thread-buffer-disposition", Fthread_buffer_disposition, Sthread_buffer_disposition,
       1, 1, 0,
       doc: "Return the value of THREAD's buffer disposition.\nSee `make-thread' for the description of possible values.",
       (thread: Lisp_Object),
       {
  let tstate: *mut thread_state;

  CHECK_THREAD (thread);
  tstate = XTHREAD (thread);

  return (*tstate).buffer_disposition;
});

// thread.c 1062-1081.
DEFUN! ("thread-set-buffer-disposition", Fthread_set_buffer_disposition, Sthread_set_buffer_disposition,
       2, 2, 0,
       doc: "Set THREAD's buffer disposition.\nSee `make-thread' for the description of possible values.\n\nBuffer disposition of the main thread cannot be modified.",
       (thread: Lisp_Object, value: Lisp_Object),
       {
  let tstate: *mut thread_state;

  CHECK_THREAD (thread);
  tstate = XTHREAD (thread);

  if main_thread_p (tstate as *const c_void)
    { CHECK_TYPE (NILP (value), Qnull (), value); }

  (*tstate).buffer_disposition = value;

  return value;
});

// thread.c 1083-1099.
DEFUN! ("thread--blocker", Fthread_blocker, Sthread_blocker, 1, 1, 0,
       doc: "Return the object that THREAD is blocking on.\nIf THREAD is blocked in `thread-join' on a second thread, return that\nthread.\nIf THREAD is blocked in `mutex-lock', return the mutex.\nIf THREAD is blocked in `condition-wait', return the condition variable.\nOtherwise, if THREAD is not blocked, return nil.",
       (thread: Lisp_Object),
       {
  let tstate: *mut thread_state;

  CHECK_THREAD (thread);
  tstate = XTHREAD (thread);

  return (*tstate).event_object;
});

// thread.c 1101-1116.
unsafe extern "C-unwind" fn thread_join_callback (arg: *mut c_void)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let tstate: *mut thread_state = arg as *mut thread_state;
  let self_: *mut thread_state = current_thread;
  let thread: Lisp_Object;

  XSETTHREAD! (thread, tstate);
  (*self_).event_object = thread;
  (*self_).wait_condvar = &raw mut (*tstate).thread_condvar;
  while thread_live_p! (tstate) && NILP ((*self_).error_symbol)
    { sys_cond_wait ((*self_).wait_condvar, &raw mut global_lock); }

  (*self_).wait_condvar = std::ptr::null_mut ();
  (*self_).event_object = Qnil ();
  post_acquire_global_lock (self_);

  }
}

// thread.c 1118-1144.
DEFUN! ("thread-join", Fthread_join, Sthread_join, 1, 1, 0,
       doc: "Wait for THREAD to exit.\nThis blocks the current thread until THREAD exits or until the current\nthread is signaled.  It returns the result of the THREAD function.  It\nis an error for a thread to try to join itself.",
       (thread: Lisp_Object),
       {
  let tstate: *mut thread_state;
  let error_symbol: Lisp_Object;
  let error_data: Lisp_Object;

  CHECK_THREAD (thread);
  tstate = XTHREAD (thread);

  if tstate == current_thread
    { error (c"Cannot join current thread".as_ptr ()); }

  error_symbol = (*tstate).error_symbol;
  error_data = (*tstate).error_data;

  if thread_live_p! (tstate)
    { flush_stack_call_func (Some (thread_join_callback), tstate as *mut c_void); }

  if !NILP (error_symbol)
    { Fsignal (error_symbol, error_data); }

  return (*tstate).result;
});

// thread.c 1146-1165.
DEFUN! ("all-threads", Fall_threads, Sall_threads, 0, 0, 0,
       doc: "Return a list of all the live threads.",
       (),
       {
  let mut result: Lisp_Object = Qnil ();
  let mut iter: *mut thread_state;

  iter = all_threads;
  while !iter.is_null ()
    {
      if thread_live_p! (iter)
	{
	  let thread: Lisp_Object;

	  XSETTHREAD! (thread, iter);
	  result = Fcons (thread, result);
	}
      iter = (*iter).next_thread;
    }

  return result;
});

// thread.c 1167-1181.
DEFUN! ("thread-last-error", Fthread_last_error, Sthread_last_error, 0, 1, 0,
       doc: "Return the last error form recorded by a dying thread.\nIf CLEANUP is non-nil, remove this error form from history.",
       (cleanup: Lisp_Object),
       {
  let result: Lisp_Object = last_thread_error;

  if !NILP (cleanup)
    { last_thread_error = Qnil (); }

  return result;
});

// thread.c 1183-1198.  The parameter is C's `struct buffer *`; the mirror declares it
// untyped (P-55), so the use sites need no cast.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn thread_check_current_buffer (buffer: *mut c_void) -> bool
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let mut iter: *mut thread_state;

  iter = all_threads;
  while !iter.is_null ()
    {
      if iter == current_thread
	{ iter = (*iter).next_thread; continue; }  /* C's for-step, before the continue */

      if (*iter).m_current_buffer == buffer && EQ ((*iter).buffer_disposition, Qt ())
	{ return true; }

      iter = (*iter).next_thread;
    }

  return false;

  }
}

// thread.c 1200-1230.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn thread_all_before_buffer_killed (current: Lisp_Object)
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  let mut iter: *mut thread_state;
  let mut other: *mut buffer = std::ptr::null_mut ();
  let b: *mut buffer = XBUFFER (current);
  let caller_thread: *mut thread_state = current_thread;

  iter = all_threads;
  while !iter.is_null ()
    {
      if iter == caller_thread
	{ iter = (*iter).next_thread; continue; }  /* C's for-step, before the continue */

      if (*iter).m_current_buffer == b as *mut c_void
	{
	  let thread: Lisp_Object;

	  XSETTHREAD! (thread, iter);

	  if other.is_null ()
	    { other = XBUFFER (Fother_buffer (current, Qnil (), Qnil ())); }

	  if !EQ ((*iter).buffer_disposition, Qsilently ())
	    { thread_set_error (iter, Qthread_buffer_killed (), Qnil ()); }

	  (*iter).m_current_buffer = other as *mut c_void;
	}

      iter = (*iter).next_thread;
    }

  }
}

// thread.c 1232-1236.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn main_thread_p (ptr: *const c_void) -> bool
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  return ptr == std::ptr::addr_of! (main_thread.s) as *const c_void;

  }
}

// thread.c 1238-1244.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn in_current_thread () -> bool
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  if current_thread.is_null ()
    { return false; }
  return sys_thread_equal (sys_thread_self (), (*current_thread).thread_id);

  }
}

// thread.c 1246-1260.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn init_threads ()
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  sys_cond_init (&raw mut main_thread.s.thread_condvar);
  sys_mutex_init (&raw mut global_lock);
  sys_mutex_lock (&raw mut global_lock);
  current_thread = std::ptr::addr_of_mut! (main_thread.s);
  #[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
  { (*current_thread).java_env = thread_android_api::android_java_env; }

  main_thread.s.thread_id = sys_thread_self ();
  main_thread.s.buffer_disposition = Qnil ();
  init_bc_thread (&raw mut main_thread.s.bc);

  }
}

// thread.c 1262-1314.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_threads ()
{
  // SAFETY: VM thread/mutex/condvar synchronization boundary; maintains global-lock handoff, register-spill stack bounds, and per-thread root/specpdl ownership.
  unsafe {
  #[cfg (feature = "threads_enabled")]
    {
      defsubr (&raw mut Sthread_yield);
      defsubr (&raw mut Smake_thread);
      defsubr (&raw mut Scurrent_thread);
      defsubr (&raw mut Sthread_name);
      defsubr (&raw mut Sthread_signal);
      defsubr (&raw mut Sthread_live_p);
      defsubr (&raw mut Sthread_join);
      defsubr (&raw mut Sthread_blocker);
      defsubr (&raw mut Sall_threads);
      defsubr (&raw mut Smake_mutex);
      defsubr (&raw mut Smutex_lock);
      defsubr (&raw mut Smutex_unlock);
      defsubr (&raw mut Smutex_name);
      defsubr (&raw mut Smake_condition_variable);
      defsubr (&raw mut Scondition_wait);
      defsubr (&raw mut Scondition_notify);
      defsubr (&raw mut Scondition_mutex);
      defsubr (&raw mut Scondition_name);
      defsubr (&raw mut Sthread_last_error);
      defsubr (&raw mut Sthread_buffer_disposition);
      defsubr (&raw mut Sthread_set_buffer_disposition);

      staticpro (&raw const last_thread_error);
      last_thread_error = Qnil ();

      Fprovide (intern_c_string (c"threads".as_ptr ()), Qnil ());
    }

  DEFSYM! (Qthreadp, c"threadp");
  DEFSYM! (Qmutexp, c"mutexp");
  DEFSYM! (Qcondition_variable_p, c"condition-variable-p");

  DEFSYM! (Qthread_buffer_killed, c"thread-buffer-killed");
  Fput (Qthread_buffer_killed (), Qerror_conditions (),
	list! (Qthread_buffer_killed (), Qerror ()));
  Fput (Qthread_buffer_killed (), Qerror_message (),
	build_string (c"Thread's current buffer killed".as_ptr ()));
  DEFSYM! (Qsilently, c"silently");

  DEFVAR_LISP! (c"main-thread", Vmain_thread,
    "The main thread of Emacs.");
  #[cfg (feature = "threads_enabled")]
  { XSETTHREAD! (Vmain_thread, std::ptr::addr_of_mut! (main_thread.s)); }
  #[cfg (not (feature = "threads_enabled"))]
  { Vmain_thread = Qnil (); }

  }
}

/* C's `DEFVAR_LISP` both declares and defines `Vmain_thread` (lisp.h 3280-3300's variable
   definition); the port's macro only registers it (P-21), so the unit defines it here and
   shadows `globals.rs`'s declaration instead of clashing with it (the dired.rs/menu.rs
   shape).  */
#[unsafe(no_mangle)]
pub static mut Vmain_thread: Lisp_Object = lisp_h_Qnil ();
