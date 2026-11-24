// femacs: port of src/eval.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names
// (function, macro, union member, file-scope object, local), the same control flow, the same
// error model.  `union specbinding` and `struct handler` (plus the `specpdl`/`handlerlist`
// macros) come from the lisp.rs/thread.rs mirrors, exactly as C takes them from lisp.h;
// the 54 `DEFUN`s are `DEFUN!` forms whose bodies are the C bodies (PORTING.md 5.1).
//
// `src/eval.c` is **not** in docs/baseline/config/cond-eval-verified.txt, so
// tools/extract-abi.py's `blind_conditions` keeps both arms of every `#if` live and the
// per-function token check demands C's text on both sides: per docs/status.md 3.49
// (PORTING.md P-51) every arm the frozen build does not compile is carried here as
// `#[cfg (feature = "…")]` text naming the C condition's non-frozen side.  The file's eleven
// conditions are `#ifdef HAVE_WINDOW_SYSTEM` (C 293), a literal `#if 0` (C 316, dead
// everywhere and recorded in the ledger), `#ifdef HAVE_X_WINDOWS` (C 1438, 1862),
// `GCC_LINT && __GNUC__ && !__clang__` (C 1643), and `HAVE_MODULES`/`HAVE_NATIVE_COMP`
// (C 2349, 3378, 3382, 3513, 3840, 4364) — all false in the frozen configuration
// (docs/baseline/config/darwin-arm64.macros.tsv 1161/1471/1472/1677/1719/6238/7004).
//
// Names C declares in headers whose mirrors do not carry them yet (18 functions, the two
// `dispextern.h` counters, plus what each `#[cfg]` arm needs) are declared at their use sites
// below (the bytecode.rs/doc.rs shape, docs/status.md 3.60); the three `V*` objects and the 19
// `V*`/`defvar` storages this unit's `syms_of_eval` registers are defined here (status 3.29);
// `get_act_rec`/`set_act_rec` used to be defined here too - the mirror carries them now
// (`include/lisp.rs`, P-20's open item closed in 3.351), so this unit only calls them.
// `docs/ports/eval.md` records every deviation, the two mirror gaps this unit reports
// (`lisp.rs`'s `safe_calln!` calls `Ffuncall` + `maybe_quit` where C's `safe_calln` calls
// `safe_funcall`, and `dispextern.h`'s two counters have no mirror declaration), and the
// `tools/cinventory.py` blind spot that hid two of this file's C definitions (`backtrace_top_function`,
// `load_with_autoload_queue`) until the tool learned both declarator shapes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe, unused_assignments, unreachable_code, while_true, unused_parens, unused_mut)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/limits.in.rs imported via common
// [U1] ../lib/include/stdlib.rs imported via common
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] ../lib/include/sys_types.in.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] ../lib/include/mini-gmp.rs imported via common
// [U1] include/bignum.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/font.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/intervals.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/frame.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/blockinput.rs imported via common
// [U1] include/commands.rs imported via common
// [U1] include/keyboard.rs imported via common
// [U1] include/pdumper.rs imported via common
// [U1] include/atimer.rs imported via common

/* Names C declares in headers whose mirrors do not carry them yet (the bytecode.rs shape);
   each keeps C's prototype in a `/// C:` line (PORTING.md P-55 for the pointer-only
   types).  None of these is defined by this unit.  */
unsafe extern "C-unwind" {
  /* src/dispextern.h 3608-3609: the two counters `call_debugger` reads and resets.  */
  /// C: dispextern.h 3608: `extern bool redisplaying_p;`
  pub static redisplaying_p: VmGlobal<bool>;
  /// C: dispextern.h 3609: `extern unsigned int redisplay_counter;`
  pub static redisplay_counter: VmGlobal<c_uint>;
  /// C: lisp.h 4206: `extern void set_internal (Lisp_Object, Lisp_Object, Lisp_Object, enum Set_Internal_Bind);`
  pub fn set_internal (symbol: Lisp_Object, newval: Lisp_Object, r#let: Lisp_Object,
                       bindflag: Set_Internal_Bind);





  /// C: lisp.h 4432: `extern AVOID memory_full (size_t);`
  pub fn memory_full (size: usize) -> !;
  /// C: lisp.h 4378: `extern void add_to_log (char const *, ...);`
  pub fn add_to_log (format: *const c_char, ...);

  /// C: emacs.c: `void fatal (const char *, ...) ATTRIBUTE_NORETURN;` (lisp.h's EXFUN group)
  pub fn fatal (format: *const c_char, ...) -> !;

  /// C: lisp.h 4722: `ptrdiff_t evxprintf (char **, ptrdiff_t *, char *, ptrdiff_t, const char *, va_list);`
  pub fn evxprintf (buf: *mut *mut c_char, bufsize: *mut ptrdiff_t, nonheapbuf: *mut c_char,
                    bufsize_max: ptrdiff_t, format: *const c_char, ap: core::ffi::VaList<'_>)
    -> ptrdiff_t;
}
unsafe extern "C" {

  /// C: stdlib.h: `void abort (void) __attribute__ ((__noreturn__));`
  pub fn abort () -> !;

}

/* Non-nil means record all fset's and provide's, to be undone
   if the file being autoloaded is not fully loaded.
   They are recorded by being consed onto the front of Vautoload_queue:
   (FUN . ODEF) for a defun, (0 . OFEATURES) for a provide.  */

#[unsafe(no_mangle)]
pub static Vautoload_queue: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

/* This holds either the symbol `run-hooks' or nil.
   It is nil at an early stage of startup, and when Emacs
   is shutting down.  */
#[unsafe(no_mangle)]
pub static Vrun_hooks: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

/* The function from which the last `signal' was called.  Set in
   Fsignal.  */
/* FIXME: We should probably get rid of this!  */
#[unsafe(no_mangle)]
pub static Vsignaling_function: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

/* These would ordinarily be static, but they need to be visible to GDB.  */
/* C 58-61's declarations are dropped (PORTING.md 0.1-12); the definitions below carry the
   externally visible linkage the declarations asked for, `#[unsafe(no_mangle)] pub`.  */

/* C 54-56's three forward declarations are dropped (PORTING.md 0.1-12).  */

unsafe fn specpdl_symbol (pdl: *mut specbinding) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind >= SPECPDL_LET as u8);
  return (*pdl).r#let.symbol;

  }
}

unsafe fn specpdl_kind (pdl: *mut specbinding) -> u32
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind >= SPECPDL_LET as u8);
  return (*pdl).r#let.kind as u32;

  }
}

unsafe fn specpdl_old_value (pdl: *mut specbinding) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind >= SPECPDL_LET as u8);
  return (*pdl).r#let.old_value;

  }
}

unsafe fn set_specpdl_old_value (pdl: *mut specbinding, val: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind >= SPECPDL_LET as u8);
  (*pdl).r#let.old_value = val;

  }
}

unsafe fn specpdl_where (pdl: *mut specbinding) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind > SPECPDL_LET as u8);
  return (*pdl).r#let.r#where.buf;

  }
}

unsafe fn specpdl_kboard (pdl: *mut specbinding) -> *mut KBOARD
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind == SPECPDL_LET as u8);
  return (*pdl).r#let.r#where.kbd as *mut KBOARD;

  }
}

unsafe fn specpdl_arg (pdl: *mut specbinding) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind == SPECPDL_UNWIND as u8);
  return (*pdl).unwind.arg;

  }
}

/* To work around GDB bug 32313
   <https://sourceware.org/bugzilla/show_bug.cgi?id=32313> make
   backtrace_* functions visible-to-GDB pointers instead of merely
   being an externally visible functions themselves.  Declare the
   pointer first to pacify gcc -Wmissing-variable-declarations.  */
/* C 112-115's `GDB_FUNCPTR(func, resulttype, params)` declares the pointer and then defines
   it as `func##_body`; the port's macro does the same: the GDB-visible pointer is a
   `pub static` of the function-pointer type (Rust has no `const`-qualified pointer object and
   no attribute for `EXTERNALLY_VISIBLE`; PORTING.md 8 records the attribute as the
   `#[unsafe(no_mangle)] pub` the port already writes).  */
macro_rules! GDB_FUNCPTR {
  ($func:ident, $resulttype:ty, ($($params:ty),*), $_body:ident) => {
    #[unsafe(no_mangle)]
    pub static $func: Option<unsafe extern "C-unwind" fn ($($params),*) -> $resulttype> = Some ($_body);
  };
}

unsafe extern "C-unwind" fn backtrace_function_body (pdl: *mut specbinding) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind == SPECPDL_BACKTRACE as u8);
  return (*pdl).bt.function;

  }
}
GDB_FUNCPTR! (backtrace_function, Lisp_Object, (*mut specbinding), backtrace_function_body);

unsafe fn backtrace_nargs (pdl: *mut specbinding) -> ptrdiff_t
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind == SPECPDL_BACKTRACE as u8);
  return (*pdl).bt.nargs;

  }
}

unsafe extern "C-unwind" fn backtrace_args_body (pdl: *mut specbinding) -> *mut Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind == SPECPDL_BACKTRACE as u8);
  return (*pdl).bt.args;

  }
}
GDB_FUNCPTR! (backtrace_args, *mut Lisp_Object, (*mut specbinding), backtrace_args_body);

/* Functions to modify slots of backtrace records.  */

unsafe fn set_backtrace_args (pdl: *mut specbinding, args: *mut Lisp_Object, nargs: ptrdiff_t)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind == SPECPDL_BACKTRACE as u8);
  (*pdl).bt.args = args;
  (*pdl).bt.nargs = nargs;

  }
}

unsafe fn set_backtrace_debug_on_exit (pdl: *mut specbinding, doe: bool)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! ((*pdl).kind == SPECPDL_BACKTRACE as u8);
  (*pdl).bt.debug_on_exit = doe as u8;

  }
}

/* Helper functions to scan the backtrace.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn backtrace_p (pdl: *mut specbinding) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  if !(*current_thread.as_mut_ptr ()).is_null () && !specpdl! ().is_null () && !pdl.is_null ()
    { return pdl >= specpdl! (); }
  return false;

  }
}

unsafe fn backtrace_thread_p (tstate: *mut thread_state, pdl: *mut specbinding) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe { return pdl >= (*tstate).m_specpdl; 
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn backtrace_top () -> *mut specbinding
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* This is so "xbacktrace" doesn't crash in pdumped Emacs if they
     invoke the command before init_eval_once_for_pdumper initializes
     specpdl machinery.  See also backtrace_p above.  */
  if (*current_thread.as_mut_ptr ()).is_null () || specpdl! ().is_null ()
    { return std::ptr::null_mut (); }

  let mut pdl: *mut specbinding = (specpdl_ptr! ()).offset (-1);
  while backtrace_p (pdl) && (*pdl).kind != SPECPDL_BACKTRACE as u8
    { pdl = pdl.offset (-1); }
  return pdl;

  }
}

unsafe fn backtrace_thread_top (tstate: *mut thread_state) -> *mut specbinding
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut pdl: *mut specbinding = ((*tstate).m_specpdl_ptr).offset (-1);
  while backtrace_thread_p (tstate, pdl) && (*pdl).kind != SPECPDL_BACKTRACE as u8
    { pdl = pdl.offset (-1); }
  return pdl;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn backtrace_next (pdl: *mut specbinding) -> *mut specbinding
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut pdl: *mut specbinding = pdl.offset (-1);
  while backtrace_p (pdl) && (*pdl).kind != SPECPDL_BACKTRACE as u8
    { pdl = pdl.offset (-1); }
  return pdl;

  }
}

/* C 179's `static void init_eval_once_for_pdumper (void);` is dropped (PORTING.md 0.1-12).  */

unsafe fn backtrace_thread_next (tstate: *mut thread_state, pdl: *mut specbinding)
  -> *mut specbinding
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut pdl: *mut specbinding = pdl.offset (-1);
  while backtrace_thread_p (tstate, pdl) && (*pdl).kind != SPECPDL_BACKTRACE as u8
    { pdl = pdl.offset (-1); }
  return pdl;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn init_eval_once ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* Don't forget to update docs (lispref node "Eval").  */
  (*Vrun_hooks.as_mut_ptr ()) = Qnil ();
  pdumper_do_now_and_after_load (Some (init_eval_once_for_pdumper));

  }
}

unsafe extern "C-unwind" fn init_eval_once_for_pdumper ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let size: usize = 50;
  let pdlvec: *mut specbinding =
    malloc ((size + 1) * std::mem::size_of::<specbinding> ()) as *mut specbinding;
  specpdl! () = pdlvec.offset (1);
  specpdl_ptr! () = specpdl! ();
  specpdl_end! () = (specpdl! ()).offset (size as isize);

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn init_eval ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  specpdl_ptr! () = specpdl! ();
  { /* Put a dummy catcher at top-level so that handlerlist is never NULL.
       This is important since handlerlist->nextfree holds the freelist
       which would otherwise leak every time we unwind back to top-level.   */
    handlerlist_sentinel! () = xzalloc (std::mem::size_of::<handler> ()) as *mut handler;
    (*handlerlist_sentinel! ()).nextfree = handlerlist_sentinel! ();
    handlerlist! () = handlerlist_sentinel! ();
    let c: *mut handler = push_handler (Qunbound (), CATCHER);
    eassert! (c == handlerlist_sentinel! ());
    (*handlerlist_sentinel! ()).nextfree = std::ptr::null_mut ();
    (*handlerlist_sentinel! ()).next = std::ptr::null_mut ();
  }
  (*Vquit_flag.as_mut_ptr ()) = Qnil ();
  (*debug_on_next_call.as_mut_ptr ()) = false;
  lisp_eval_depth! () = 0;
  /* This is less than the initial value of num_nonmacro_input_events.  */
  (*when_entered_debugger.as_mut_ptr ()) = -1;

  }
}

unsafe extern "C-unwind" fn restore_stack_limits (data: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut old_depth: intmax_t = 0;
  integer_to_intmax (data, &mut old_depth);
  (*lisp_eval_depth_reserve.as_mut_ptr ()) += (*max_lisp_eval_depth.as_mut_ptr ()) - old_depth as EMACS_INT;
  (*max_lisp_eval_depth.as_mut_ptr ()) = old_depth as EMACS_INT;

  }
}

/* Try and ensure that we have at least B dpeth available.  */

unsafe extern "C-unwind" fn max_ensure_room (b: intmax_t)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let (ckd_add_value, ckd_add_overflow) =
    (lisp_eval_depth! () as intmax_t).overflowing_add (b);
  let mut sum = ckd_add_value;
  if ckd_add_overflow
    { sum = INTMAX_MAX; }
  let diff = min ((sum - (*max_lisp_eval_depth.as_mut_ptr ()) as intmax_t) as isize,
                      (*lisp_eval_depth_reserve.as_mut_ptr ()) as isize);
  if diff <= 0
    { return; }
  let old_depth: intmax_t = (*max_lisp_eval_depth.as_mut_ptr ()) as intmax_t;
  (*max_lisp_eval_depth.as_mut_ptr ()) += diff as EMACS_INT;
  (*lisp_eval_depth_reserve.as_mut_ptr ()) -= diff as EMACS_INT;
  /* Restore limits after leaving the debugger.  */
  record_unwind_protect (Some (restore_stack_limits), make_int (old_depth));

  }
}

/* Call the Lisp debugger, giving it argument ARG.  */

#[unsafe(no_mangle)]
pub extern "C-unwind" fn call_debugger (arg: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut debug_while_redisplaying: bool;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut val: Lisp_Object;

  /* The debugger currently requires 77 additional frames to print lists
     nested 8 deep (the value of print-level used in the debugger) using
     cl-prin1 (bug#31919), with a margin to be on the safe side.  */
  max_ensure_room (200);

  /* C 293-296's `#ifdef HAVE_WINDOW_SYSTEM` (dead in the frozen configuration; P-51).  */
  #[cfg (feature = "have_window_system")]
  {
    unsafe extern "C" {
      /// C: dispextern.h 3073: `extern bool display_hourglass_p;`
      pub static display_hourglass_p: VmGlobal<bool>;
    }
    if (*display_hourglass_p.as_mut_ptr ())
      { cancel_hourglass (); }
  }

  (*debug_on_next_call.as_mut_ptr ()) = false;
  (*when_entered_debugger.as_mut_ptr ()) = (*num_nonmacro_input_events.as_mut_ptr ());

  /* Resetting redisplaying_p to 0 makes sure that debug output is
     displayed if the debugger is invoked during redisplay.  */
  debug_while_redisplaying = (*redisplaying_p.as_mut_ptr ());
  let redisplay_counter_before: c_int = (*redisplay_counter.as_mut_ptr ()) as c_int;
  (*redisplaying_p.as_mut_ptr ()) = false;
  specbind (Qdebugger_may_continue (), if debug_while_redisplaying { Qnil () } else { Qt () });
  specbind (Qinhibit_redisplay (), Qnil ());
  specbind (Qinhibit_debugger (), Qt ());

  /* If we are debugging an error while `inhibit-changing-match-data'
     is bound to non-nil (e.g., within a call to `string-match-p'),
     then make sure debugger code can still use match data.  */
  specbind (Qinhibit_changing_match_data (), Qnil ());

  /* C 316-319 is a literal `#if 0` (dead everywhere): it binds
     `Qinhibit_eval_during_redisplay` around redisplay, which prevents Lisp execution during
     redisplay.  Recorded in docs/ports/eval.md, not transcribed.  */

  val = apply1 ((*Vdebugger.as_mut_ptr ()), arg);

  /* Interrupting redisplay and resuming it later is not safe under
     all circumstances.  So, when the debugger returns, abort the
     interrupted redisplay by going back to the top-level.  */
  if debug_while_redisplaying
    && redisplay_counter_before != (*redisplay_counter.as_mut_ptr ()) as c_int
    /* FIXME: Rather than jump all the way to `top-level`
       we should exit only the current redisplay.  */
    { Ftop_level (); }

  return unbind_to (count, val);

  }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn do_debug_on_call (code: Lisp_Object, count: specpdl_ref)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  (*debug_on_next_call.as_mut_ptr ()) = false;
  set_backtrace_debug_on_exit (specpdl_ref_to_ptr (count), true);
  call_debugger (list1 (code));

  }
}

DEFUN! ("or", For, Sor, 0, UNEVALLED, 0,
       doc: "Eval args until one of them yields non-nil, then return that value.\nThe remaining args are not evalled at all.\nIf all args return nil, return nil.\nusage: (or CONDITIONS...)",
       (args: Lisp_Object),
       {
  let mut args: Lisp_Object = args;   // C assigns to its parameter
  let mut val: Lisp_Object = Qnil ();

  while CONSP (args)
    {
      let arg: Lisp_Object = XCAR (args);
      args = XCDR (args);
      val = eval_sub (arg);
      if !NILP (val)
	{ break; }
    }

  return val;
});

DEFUN! ("and", Fand, Sand, 0, UNEVALLED, 0,
       doc: "Eval args until one of them yields nil, then return nil.\nThe remaining args are not evalled at all.\nIf no arg yields nil, return the last arg's value.\nusage: (and CONDITIONS...)",
       (args: Lisp_Object),
       {
  let mut args: Lisp_Object = args;   // C assigns to its parameter
  let mut val: Lisp_Object = Qt ();

  while CONSP (args)
    {
      let arg: Lisp_Object = XCAR (args);
      args = XCDR (args);
      val = eval_sub (arg);
      if NILP (val)
	{ break; }
    }

  return val;
});

DEFUN! ("if", Fif, Sif, 2, UNEVALLED, 0,
       doc: "If COND yields non-nil, do THEN, else do ELSE...\nReturns the value of THEN or the value of the last of the ELSE's.\nTHEN must be one expression, but ELSE... can be zero or more expressions.\nIf COND yields nil, and there are no ELSE's, the value is nil.\nusage: (if COND THEN ELSE...)",
       (args: Lisp_Object),
       {
  let cond: Lisp_Object;

  cond = eval_sub (XCAR (args));

  if !NILP (cond)
    { return eval_sub (Fcar (XCDR (args))); }
  return Fprogn (Fcdr (XCDR (args)));
});

DEFUN! ("cond", Fcond, Scond, 0, UNEVALLED, 0,
       doc: "Try each clause until one succeeds.\nEach clause looks like (CONDITION BODY...).  CONDITION is evaluated\nand, if the value is non-nil, this clause succeeds:\nthen the expressions in BODY are evaluated and the last one's\nvalue is the value of the cond-form.\nIf a clause has one element, as in (CONDITION), then the cond-form\nreturns CONDITION's value, if that is non-nil.\nIf no clause succeeds, cond returns nil.\nusage: (cond CLAUSES...)",
       (args: Lisp_Object),
       {
  let mut args: Lisp_Object = args;   // C assigns to its parameter
  let mut val: Lisp_Object = args;

  while CONSP (args)
    {
      let clause: Lisp_Object = XCAR (args);
      val = eval_sub (Fcar (clause));
      if !NILP (val)
	{
	  if !NILP (XCDR (clause))
	    { val = Fprogn (XCDR (clause)); }
	  break;
	}
      args = XCDR (args);
    }

  return val;
});

DEFUN! ("progn", Fprogn, Sprogn, 0, UNEVALLED, 0,
       doc: "Eval BODY forms sequentially and return value of last one.\nusage: (progn BODY...)",
       (body: Lisp_Object),
       {
  let mut body: Lisp_Object = body;   // C assigns to its parameter
  let mut val: Lisp_Object = Qnil ();

  while CONSP (body)
    {
      let form: Lisp_Object = XCAR (body);
      body = XCDR (body);
      val = eval_sub (form);
    }

  return val;
});

/* Evaluate BODY sequentially, discarding its value.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn prog_ignore (body: Lisp_Object)
{
  {
  Fprogn (body);

  }
}

DEFUN! ("prog1", Fprog1, Sprog1, 1, UNEVALLED, 0,
       doc: "Eval FIRST and BODY sequentially; return value from FIRST.\nThe value of FIRST is saved during the evaluation of the remaining args,\nwhose values are discarded.\nusage: (prog1 FIRST BODY...)",
       (args: Lisp_Object),
       {
  let val: Lisp_Object = eval_sub (XCAR (args));
  let _val_root = crate::roots::Rooted::new (crate::roots::LispVal::from_raw (val));
  prog_ignore (XCDR (args));
  return val;
});

#[inline(always)]
fn assq_lexenv (key: Lisp_Object, alist: Lisp_Object) -> Lisp_Object
{
  {
  let mut tail: Lisp_Object = alist;
  let mut steps: ptrdiff_t = 0;
  while CONSP (tail) && steps < 64
    {
      let elt: Lisp_Object = XCAR (tail);
      if CONSP (elt) && EQ (XCAR (elt), key)
	{ return elt; }
      tail = XCDR (tail);
      steps += 1;
    }
  if NILP (tail)
    { return Qnil (); }
  return Fassq (key, alist);

  }
}

#[inline(always)]
fn memq_lexenv (elt: Lisp_Object, list: Lisp_Object) -> Lisp_Object
{
  {
  let mut tail: Lisp_Object = list;
  let mut steps: ptrdiff_t = 0;
  while CONSP (tail) && steps < 64
    {
      if EQ (XCAR (tail), elt)
	{ return tail; }
      tail = XCDR (tail);
      steps += 1;
    }
  if NILP (tail)
    { return Qnil (); }
  return Fmemq (elt, list);

  }
}

#[inline(always)]
fn fast_list_length (list: Lisp_Object) -> ptrdiff_t
{
  {
  let mut tail: Lisp_Object = list;
  let mut i: ptrdiff_t = 0;
  while CONSP (tail) && i < 64
    {
      tail = XCDR (tail);
      i += 1;
    }
  if NILP (tail)
    { return i; }
  return list_length (list);

  }
}

DEFUN! ("setq", Fsetq, Ssetq, 0, UNEVALLED, 0,
       doc: "Set each SYM to the value of its VAL.\nThe symbols SYM are variables; they are literal (not evaluated).\nThe values VAL are expressions; they are evaluated.\nThus, (setq x (1+ y)) sets `x' to the value of `(1+ y)'.\nThe second VAL is not computed until after the first SYM is set, and so on;\neach VAL can use the new value of variables set earlier in the `setq'.\nThe return value of the `setq' form is the value of the last VAL.\nusage: (setq [SYM VAL]...)",
       (args: Lisp_Object),
       {
  let mut args: Lisp_Object = args;   // C assigns to its parameter
  let mut val: Lisp_Object = args;
  let mut tail: Lisp_Object = args;

  let mut nargs: EMACS_INT = 0;
  while CONSP (tail)
    {
      let sym: Lisp_Object = XCAR (tail);
      tail = XCDR (tail);
      if !CONSP (tail)
	{ xsignal2 (Qwrong_number_of_arguments (), Qsetq (), make_fixnum (nargs + 1)); }
      let arg: Lisp_Object = XCAR (tail);
      tail = XCDR (tail);
      val = eval_sub (arg);
      /* Like for eval_sub, we do not check declared_special here since
	 it's been done when let-binding.  */
      let lex_binding: Lisp_Object
	= if SYMBOLP (sym)
	    { assq_lexenv (sym, (*Vinternal_interpreter_environment.as_mut_ptr ())) }
	  else
	    { Qnil () };
      if !NILP (lex_binding)
	{ XSETCDR (lex_binding, val); } /* SYM is lexically bound.  */
      else
	{ Fset (sym, val); }	/* SYM is dynamically bound.  */
      nargs += 2;
    }

  return val;
});

DEFUN! ("quote", Fquote, Squote, 1, UNEVALLED, 0,
       doc: "Return the argument, without evaluating it.  `(quote x)' yields `x'.\nWarning: `quote' does not construct its return value, but just returns\nthe value that was pre-constructed by the Lisp reader (see info node\n`(elisp)Printed Representation').\nThis means that \\='(a . b) is not identical to (cons \\='a \\='b): the former\ndoes not cons.  Quoting should be reserved for constants that will\nnever be modified by side-effects, unless you like self-modifying code.\nSee the common pitfall in info node `(elisp)Rearrangement' for an example\nof unexpected results when a quoted object is modified.\nusage: (quote ARG)",
       (args: Lisp_Object),
       {
  if !NILP (XCDR (args))
    { xsignal2 (Qwrong_number_of_arguments (), Qquote (), Flength (args)); }
  return XCAR (args);
});

DEFUN! ("make-interpreted-closure", Fmake_interpreted_closure,
       Smake_interpreted_closure, 3, 5, 0,
       doc: "Make an interpreted closure.\nARGS should be the list of formal arguments.\nBODY should be a non-empty list of forms.\nENV should be a lexical environment, like the second argument of `eval'.\nIFORM if non-nil should be of the form (interactive ...).",
       (args: Lisp_Object, body: Lisp_Object, env: Lisp_Object,
        docstring: Lisp_Object, iform: Lisp_Object),
       {
  let ifcdr: Lisp_Object;
  let mut value: Lisp_Object = Qnil ();
  let mut slots: [Lisp_Object; 6] = [lisp_h_Qnil (); 6];

  CHECK_CONS (body);          /* Make sure it's not confused with byte-code! */
  CHECK_LIST (args);
  CHECK_LIST (iform);
  ifcdr = CDR (iform);
  if NILP (CDR (ifcdr))
    { value = CAR (ifcdr); }
  else
    { value = CALLN! (Fvector, XCAR (ifcdr), XCDR (ifcdr)); }
  slots[0] = args;
  slots[1] = body;
  slots[2] = env;
  slots[3] = Qnil ();
  slots[4] = docstring;
  slots[5] = value;
  /* Adjusting the size is indispensable since, as for byte-code objects,
     we distinguish interactive functions by the presence or absence of the
     iform slot.  */
  let val: Lisp_Object
    = Fvector (if !NILP (iform) { 6 } else if !NILP (docstring) { 5 } else { 3 },
               slots.as_mut_ptr ());
  XSETPVECTYPE! (XVECTOR (val), pvec_type::PVEC_CLOSURE);
  return val;
});

DEFUN! ("function", Ffunction, Sfunction, 1, UNEVALLED, 0,
       doc: "Like `quote', but preferred for objects which are functions.\nIn byte compilation, `function' causes its argument to be handled by\nthe byte compiler.  Similarly, when expanding macros and expressions,\nARG can be examined and possibly expanded.  If `quote' is used\ninstead, this doesn't happen.\n\nusage: (function ARG)",
       (args: Lisp_Object),
       {
  let quoted: Lisp_Object = XCAR (args);

  if !NILP (XCDR (args))
    { xsignal2 (Qwrong_number_of_arguments (), Qfunction (), Flength (args)); }

  if CONSP (quoted)
    && EQ (XCAR (quoted), Qlambda ())
    { /* This is a lambda expression within a lexical environment;
	 return an interpreted closure instead of a simple lambda.  */
      let mut cdr: Lisp_Object = XCDR (quoted);
      let args: Lisp_Object = Fcar (cdr);
      cdr = Fcdr (cdr);
      let mut docstring: Lisp_Object = Qnil ();
      let mut iform: Lisp_Object = Qnil ();
      if CONSP (cdr)
        {
          docstring = XCAR (cdr);
          if STRINGP (docstring)
            {
              let tmp: Lisp_Object = XCDR (cdr);
              if !NILP (tmp)
                { cdr = tmp; }
              else     /* It's not a docstring, it's a return value.  */
                { docstring = Qnil (); }
            }
          /* Handle the special (:documentation <form>) to build the docstring
	     dynamically.  */
          else if CONSP (docstring)
                   && EQ (QCdocumentation (), XCAR (docstring))
                   && ({ docstring = eval_sub (Fcar (XCDR (docstring))); true })
            { cdr = XCDR (cdr); }
          else
            { docstring = Qnil (); }   /* Not a docstring after all.  */
        }
      if CONSP (cdr)
        {
          iform = XCAR (cdr);
          if CONSP (iform)
              && EQ (Qinteractive (), XCAR (iform))
            { cdr = XCDR (cdr); }
          else
            { iform = Qnil (); }   /* Not an interactive-form after all.  */
        }
      if NILP (cdr)
        { cdr = Fcons (Qnil (), Qnil ()); } /* Make sure the body is never empty! */

      if NILP ((*Vinternal_interpreter_environment.as_mut_ptr ()))
          || NILP ((*Vinternal_make_interpreted_closure_function.as_mut_ptr ()))
        { return Fmake_interpreted_closure
            (args, cdr, (*Vinternal_interpreter_environment.as_mut_ptr ()), docstring, iform); }
      else
        {
          /* The hook may be advised or redefined even when it names the
             standard converter.  Its call is part of closure creation.  */
          return calln! ((*Vinternal_make_interpreted_closure_function.as_mut_ptr ()),
                         args, cdr, (*Vinternal_interpreter_environment.as_mut_ptr ()),
                         docstring, iform);
        }
    }
  else
    /* Simply quote the argument.  */
    { return quoted; }
});

DEFUN! ("defvaralias", Fdefvaralias, Sdefvaralias, 2, 3, 0,
       doc: "Make NEW-ALIAS a variable alias for symbol BASE-VARIABLE.\nAliased variables always have the same value; setting one sets the other.\nThird arg DOCSTRING, if non-nil, is documentation for NEW-ALIAS.  If it is\nomitted or nil, NEW-ALIAS gets the documentation string of BASE-VARIABLE,\nor of the variable at the end of the chain of aliases, if BASE-VARIABLE is\nitself an alias.  If NEW-ALIAS is bound, and BASE-VARIABLE is not,\nthen the value of BASE-VARIABLE is set to that of NEW-ALIAS.\nThe return value is BASE-VARIABLE.\n\nIf the resulting chain of variable definitions would contain a loop,\nsignal a `cyclic-variable-indirection' error.",
       (new_alias: Lisp_Object, base_variable: Lisp_Object, docstring: Lisp_Object),
       {
  CHECK_SYMBOL (new_alias);
  CHECK_SYMBOL (base_variable);

  if SYMBOL_CONSTANT_P (new_alias)
    /* Making it an alias effectively changes its value.  */
    { error (c"Cannot make a constant an alias: %s".as_ptr (),
	     SDATA (SYMBOL_NAME (new_alias))); }

  let sym: *mut Lisp_Symbol = XSYMBOL (new_alias);

  /* Ensure non-circularity.  */
  let mut s: *mut Lisp_Symbol = XSYMBOL (base_variable);
  loop
    {
      if s == sym
	{ xsignal1 (Qcyclic_variable_indirection (), base_variable); }
      if ((*s).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT != SYMBOL_VARALIAS
	{ break; }
      s = SYMBOL_ALIAS (s);
    }

  match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
    {
    SYMBOL_FORWARDED =>
      { error (c"Cannot make a built-in variable an alias: %s".as_ptr (),
	       SDATA (SYMBOL_NAME (new_alias))); }
    SYMBOL_LOCALIZED =>
      { error (c"Don't know how to make a buffer-local variable an alias: %s".as_ptr (),
	       SDATA (SYMBOL_NAME (new_alias))); }
    SYMBOL_PLAINVAL | SYMBOL_VARALIAS =>
      { }
    _ =>
      { emacs_abort (); }
    }

  /* https://lists.gnu.org/r/emacs-devel/2008-04/msg00834.html
     If n_a is bound, but b_v is not, set the value of b_v to n_a,
     so that old-code that affects n_a before the aliasing is setup
     still works.  */
  if NILP (Fboundp (base_variable))
    { set_internal (base_variable, find_symbol_value (new_alias),
                    Qnil (), SET_INTERNAL_BIND); }
  else if !NILP (Fboundp (new_alias))
           && !EQ (find_symbol_value (new_alias),
                   find_symbol_value (base_variable))
    {
      let mut message: Lisp_Object;
      let formatted: Lisp_Object;

      message = build_string (c"Overwriting value of `%s' by aliasing to `%s'".as_ptr ());
      formatted = CALLN! (Fformat_message, message, new_alias, base_variable);
      calln! (Qdisplay_warning (),
	      list3 (Qdefvaralias (), Qlosing_value (), new_alias),
	      formatted);
    }

  {
    let mut p: *mut specbinding = specpdl_ptr! ();

    while p > specpdl! ()
      {
        p = p.offset (-1);
        if (*p).kind >= SPECPDL_LET as u8
	    && (EQ (new_alias, specpdl_symbol (p)))
	  { error (c"Don't know how to make a let-bound variable an alias: %s".as_ptr (),
	           SDATA (SYMBOL_NAME (new_alias))); }
      }
  }

  if ((*sym).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK) >> SYMBOL_TRAPPED_WRITE_SHIFT
     == SYMBOL_TRAPPED_WRITE
    { notify_variable_watchers (new_alias, base_variable, Qdefvaralias (), Qnil ()); }

  (*sym).u.s.bits = (*sym).u.s.bits | SYMBOL_DECLARED_SPECIAL_BIT;
  (*XSYMBOL (base_variable)).u.s.bits =
    (*XSYMBOL (base_variable)).u.s.bits | SYMBOL_DECLARED_SPECIAL_BIT;
  (*sym).u.s.bits = ((*sym).u.s.bits & ! SYMBOL_REDIRECT_MASK)
                    | (SYMBOL_VARALIAS << SYMBOL_REDIRECT_SHIFT);
  SET_SYMBOL_ALIAS (sym, XSYMBOL (base_variable));
  (*sym).u.s.bits = ((*sym).u.s.bits & ! SYMBOL_TRAPPED_WRITE_MASK)
                    | ((*XSYMBOL (base_variable)).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK);
  LOADHIST_ATTACH (new_alias);
  /* Even if docstring is nil: remove old docstring.  */
  Fput (new_alias, Qvariable_documentation (), docstring);

  return base_variable;
});

DEFUN! ("internal-delete-indirect-variable", Finternal_delete_indirect_variable, Sinternal_delete_indirect_variable,
       1, 1, 0,
       doc: "Undeclare SYMBOL as variable alias, then unbind it.\nReturn SYMBOL.\nInternal use only.",
       (symbol: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  if ((*XSYMBOL (symbol)).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
     != SYMBOL_VARALIAS
    { xsignal2 (Qerror (),
		build_string (c"Cannot undeclare a variable that is not an alias".as_ptr ()),
		symbol); }
  (*XSYMBOL (symbol)).u.s.bits =
    ((*XSYMBOL (symbol)).u.s.bits & ! SYMBOL_REDIRECT_MASK)
    | (SYMBOL_PLAINVAL << SYMBOL_REDIRECT_SHIFT);
  Fput (symbol, Qvariable_documentation (), Qnil ());
  Fset (symbol, Qunbound ());
  return symbol;
});

unsafe fn default_toplevel_binding (symbol: Lisp_Object) -> *mut specbinding
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut pdl: *mut specbinding = specpdl! ();
  while pdl < specpdl_ptr! ()
    {
      if (*pdl).kind == SPECPDL_LET_DEFAULT as u8 || (*pdl).kind == SPECPDL_LET as u8
	{
	  if EQ (specpdl_symbol (pdl), symbol)
	    { return pdl; }
	}
      pdl = pdl.offset (1);
    }
  return std::ptr::null_mut ();

  }
}

unsafe fn local_toplevel_binding (symbol: Lisp_Object, buf: Lisp_Object) -> *mut specbinding
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut pdl: *mut specbinding = specpdl! ();
  while pdl < specpdl_ptr! ()
    {
      if (*pdl).kind == SPECPDL_LET_LOCAL as u8
	{
	  if BASE_EQ (specpdl_where (pdl), buf)
	      && EQ (specpdl_symbol (pdl), symbol)
	    { return pdl; }
	}
      pdl = pdl.offset (1);
    }
  return std::ptr::null_mut ();

  }
}

/* Look for a lexical-binding of SYMBOL somewhere up the stack.
   This will only find bindings created with interpreted code, since once
   compiled names of lexical variables are basically gone anyway.  */
unsafe fn lexbound_p (symbol: Lisp_Object) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut pdl: *mut specbinding = specpdl_ptr! ();
  while pdl > specpdl! ()
    {
      pdl = pdl.offset (-1);
      if (*pdl).kind == SPECPDL_LET_DEFAULT as u8 || (*pdl).kind == SPECPDL_LET as u8
	{
	  if BASE_EQ (specpdl_symbol (pdl), Qinternal_interpreter_environment ())
	    {
	      let env: Lisp_Object = specpdl_old_value (pdl);
	      if CONSP (env) && !NILP (Fassq (symbol, env))
		{ return true; }
	    }
	}
    }
  return false;

  }
}

DEFUN! ("default-toplevel-value", Fdefault_toplevel_value, Sdefault_toplevel_value, 1, 1, 0,
       doc: "Return SYMBOL's toplevel default value.\n\"Toplevel\" means outside of any let binding.\nSignals `void-variable' if there is no such value, which can\nhappen even if `default-boundp' is non-nil.\nNote: In most cases, you'll want to use `default-value' instead.",
       (symbol: Lisp_Object),
       {
  let binding: *mut specbinding = default_toplevel_binding (symbol);
  let value: Lisp_Object
    = if !binding.is_null () { specpdl_old_value (binding) } else { Fdefault_value (symbol) };
  if !BASE_EQ (value, Qunbound ())
    { return value; }
  xsignal1 (Qvoid_variable (), symbol);
});

DEFUN! ("set-default-toplevel-value", Fset_default_toplevel_value,
       Sset_default_toplevel_value, 2, 2, 0,
       doc: "Set SYMBOL's toplevel default value to VALUE.\n\"Toplevel\" means outside of any let binding.\nNote: In most cases, you'll want to use `set-default' instead.",
       (symbol: Lisp_Object, value: Lisp_Object),
       {
  let binding: *mut specbinding = default_toplevel_binding (symbol);
  if !binding.is_null ()
    { set_specpdl_old_value (binding, value); }
  else
    { Fset_default (symbol, value); }
  return Qnil ();
});

DEFUN! ("buffer-local-toplevel-value",
       Fbuffer_local_toplevel_value,
       Sbuffer_local_toplevel_value, 1, 2, 0,
       doc: "Return SYMBOL's toplevel buffer-local value in BUFFER.\n\"Toplevel\" means outside of any let binding.\nBUFFER defaults to the current buffer.\nIf SYMBOL has no local value in BUFFER, signals an error.",
       (symbol: Lisp_Object, buffer: Lisp_Object),
       {
  let mut buffer: Lisp_Object = buffer;   // C assigns to its parameter
  if NILP (buffer)
    { buffer = Fcurrent_buffer (); }
  if NILP (Flocal_variable_p (symbol, buffer))
    { xsignal1 (Qvoid_variable (), symbol); }
  let binding: *mut specbinding = local_toplevel_binding (symbol, buffer);
  return if !binding.is_null ()
    { specpdl_old_value (binding) }
  else
    { Fbuffer_local_value (symbol, buffer) };
});

DEFUN! ("set-buffer-local-toplevel-value",
       Fset_buffer_local_toplevel_value,
       Sset_buffer_local_toplevel_value, 2, 3, 0,
       doc: "Set SYMBOL's toplevel buffer-local value in BUFFER to VALUE.\n\"Toplevel\" means outside of any let-binding.\nBUFFER defaults to the current buffer.\nMakes SYMBOL buffer-local in BUFFER if it was not already.",
       (symbol: Lisp_Object, value: Lisp_Object, buffer: Lisp_Object),
       {
  let buf: Lisp_Object = if !NILP (buffer) { buffer } else { Fcurrent_buffer () };
  let binding: *mut specbinding = local_toplevel_binding (symbol, buf);

  if !binding.is_null ()
    { set_specpdl_old_value (binding, value); }
  else if NILP (buffer)
    { Fset (Fmake_local_variable (symbol), value); }
  else
    {
      let count: specpdl_ref = SPECPDL_INDEX ();
      record_unwind_current_buffer ();
      Fset_buffer (buffer);
      Fset (Fmake_local_variable (symbol), value);
      unbind_to (count, Qnil ());
    }

  return Qnil ();
});

DEFUN! ("internal--define-uninitialized-variable",
       Finternal__define_uninitialized_variable,
       Sinternal__define_uninitialized_variable, 1, 2, 0,
       doc: "Define SYMBOL as a variable, with DOC as its docstring.\nThis is like `defvar' and `defconst' but without affecting the variable's\nvalue.",
       (symbol: Lisp_Object, doc: Lisp_Object),
       {
  if (*XSYMBOL (symbol)).u.s.bits & SYMBOL_DECLARED_SPECIAL_BIT == 0
      && lexbound_p (symbol)
    /* This test tries to catch the situation where we do
       (let ((<foo-var> ...)) ...(<foo-function> ...)....)
       and where the `foo` package only gets loaded when <foo-function>
       is called, so the outer `let` incorrectly made the binding lexical
       because the <foo-var> wasn't yet declared as dynamic at that point.  */
    { xsignal2 (Qerror (),
		build_string (c"Defining as dynamic an already lexical var".as_ptr ()),
		symbol); }

  (*XSYMBOL (symbol)).u.s.bits =
    (*XSYMBOL (symbol)).u.s.bits | SYMBOL_DECLARED_SPECIAL_BIT;
  if !NILP (doc)
    {
      Fput (symbol, Qvariable_documentation (), doc);
    }
  LOADHIST_ATTACH (symbol);
  return Qnil ();
});

unsafe fn defvar (sym: Lisp_Object, initvalue: Lisp_Object, docstring: Lisp_Object, eval: bool)
  -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let tem: Lisp_Object;

  CHECK_SYMBOL (sym);

  tem = Fdefault_boundp (sym);

  /* Do it before evaluating the initial value, for self-references.  */
  Finternal__define_uninitialized_variable (sym, docstring);

  if NILP (tem)
    { Fset_default (sym, if eval { eval_sub (initvalue) } else { initvalue }); }
  else
    { /* Check if there is really a global binding rather than just a let
	     binding that shadows the global unboundness of the var.  */
      let binding: *mut specbinding = default_toplevel_binding (sym);
      if !binding.is_null () && BASE_EQ (specpdl_old_value (binding), Qunbound ())
	{
	  set_specpdl_old_value (binding,
	                         if eval { eval_sub (initvalue) } else { initvalue });
	}
    }
  return sym;

  }
}

DEFUN! ("defvar", Fdefvar, Sdefvar, 1, UNEVALLED, 0,
       doc: "Define SYMBOL as a variable, and return SYMBOL.\nYou are not required to define a variable in order to use it, but\ndefining it lets you supply an initial value and documentation, which\ncan be referred to by the Emacs help facilities and other programming\ntools.\n\nIf SYMBOL's value is void and the optional argument INITVALUE is\nprovided, INITVALUE is evaluated and the result used to set SYMBOL's\nvalue.  If SYMBOL is buffer-local, its default value is what is set;\nbuffer-local values are not affected.  If INITVALUE is missing,\nSYMBOL's value is not set.\n\nIf INITVALUE is provided, the `defvar' form also declares the variable\nas \"special\", so that it is always dynamically bound even if\n`lexical-binding' is t.  If INITVALUE is missing, the form marks the\nvariable \"special\" locally (i.e., within the current\nlexical scope, or the current file, if the form is at top-level),\nand does nothing if `lexical-binding' is nil.\n\nIf SYMBOL is let-bound, then this form does not affect the local let\nbinding but the toplevel default binding instead, like\n`set-toplevel-default-binding`.\n(`defcustom' behaves similarly in this respect.)\n\nThe optional argument DOCSTRING is a documentation string for the\nvariable.\n\nTo define a user option, use `defcustom' instead of `defvar'.\n\nTo define a buffer-local variable, use `defvar-local'.\nusage: (defvar SYMBOL &optional INITVALUE DOCSTRING)",
       (args: Lisp_Object),
       {
  let sym: Lisp_Object;
  let mut tail: Lisp_Object;

  sym = XCAR (args);
  tail = XCDR (args);

  CHECK_SYMBOL (sym);

  if !NILP (tail)
    {
      if !NILP (XCDR (tail)) && !NILP (XCDR (XCDR (tail)))
	{ error (c"Too many arguments".as_ptr ()); }
      let exp: Lisp_Object = XCAR (tail);
      tail = XCDR (tail);
      return defvar (sym, exp, CAR (tail), true);
    }
  else if !NILP ((*Vinternal_interpreter_environment.as_mut_ptr ()))
	   && (SYMBOLP (sym)
	       && (*XSYMBOL (sym)).u.s.bits & SYMBOL_DECLARED_SPECIAL_BIT == 0)
    /* A simple (defvar foo) with lexical scoping does "nothing" except
       declare that var to be dynamically scoped *locally* (i.e. within
       the current file or let-block).  */
    { (*Vinternal_interpreter_environment.as_mut_ptr ())
      = Fcons (sym, (*Vinternal_interpreter_environment.as_mut_ptr ())); }
  else
    {
      /* Simple (defvar <var>) should not count as a definition at all.
	 It could get in the way of other definitions, and unloading this
	 package could try to make the variable unbound.  */
    }

  return sym;
});

DEFUN! ("defvar-1", Fdefvar_1, Sdefvar_1, 2, 3, 0,
       doc: "Like `defvar' but as a function.\nMore specifically behaves like (defvar SYM \\='INITVALUE DOCSTRING).",
       (sym: Lisp_Object, initvalue: Lisp_Object, docstring: Lisp_Object),
       {
  return defvar (sym, initvalue, docstring, false);
});

DEFUN! ("defconst", Fdefconst, Sdefconst, 2, UNEVALLED, 0,
       doc: "Define SYMBOL as a constant variable.\nThis declares that neither programs nor users should ever change the\nvalue.  This constancy is not actually enforced by Emacs Lisp, but\nSYMBOL is marked as a special variable so that it is never lexically\nbound.\n\nThe `defconst' form always sets the value of SYMBOL to the result of\nevalling INITVALUE.  If SYMBOL is buffer-local, its default value is\nwhat is set; buffer-local values are not affected.  If SYMBOL has a\nlocal binding, then this form sets the local binding's value.\nHowever, you should normally not make local bindings for variables\ndefined with this form.\n\nThe optional DOCSTRING specifies the variable's documentation string.\nusage: (defconst SYMBOL INITVALUE [DOCSTRING])",
       (args: Lisp_Object),
       {
  let sym: Lisp_Object;
  let tem: Lisp_Object;

  sym = XCAR (args);
  CHECK_SYMBOL (sym);
  let mut docstring: Lisp_Object = Qnil ();
  if !NILP (XCDR (XCDR (args)))
    {
      if !NILP (XCDR (XCDR (XCDR (args))))
	{ error (c"Too many arguments".as_ptr ()); }
      docstring = XCAR (XCDR (XCDR (args)));
    }
  tem = eval_sub (XCAR (XCDR (args)));
  return Fdefconst_1 (sym, tem, docstring);
});

DEFUN! ("defconst-1", Fdefconst_1, Sdefconst_1, 2, 3, 0,
       doc: "Like `defconst' but as a function.\nMore specifically, behaves like (defconst SYM \\='INITVALUE DOCSTRING).",
       (sym: Lisp_Object, initvalue: Lisp_Object, docstring: Lisp_Object),
       {
  CHECK_SYMBOL (sym);
  let tem: Lisp_Object = initvalue;
  Finternal__define_uninitialized_variable (sym, docstring);
  Fset_default (sym, tem);      /* FIXME: set-default-toplevel-value? */
  Fput (sym, Qrisky_local_variable (), Qt ()); /* FIXME: Why?  */
  return sym;
});

/* Make SYMBOL lexically scoped.  */
DEFUN! ("internal-make-var-non-special", Fmake_var_non_special,
       Smake_var_non_special, 1, 1, 0,
       doc: "Internal function.",
       (symbol: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  (*XSYMBOL (symbol)).u.s.bits =
    (*XSYMBOL (symbol)).u.s.bits & ! SYMBOL_DECLARED_SPECIAL_BIT;
  return Qnil ();
});

DEFUN! ("let*", FletX, SletX, 1, UNEVALLED, 0,
       doc: "Bind variables according to VARLIST then eval BODY.\nThe value of the last form in BODY is returned.\nEach element of VARLIST is a symbol (which is bound to nil)\nor a list (SYMBOL VALUEFORM) (which binds SYMBOL to the value of VALUEFORM).\nEach VALUEFORM can refer to the symbols already bound by this VARLIST.\nusage: (let* VARLIST BODY...)",
       (args: Lisp_Object),
       {
  let mut var: Lisp_Object;
  let mut val: Lisp_Object;
  let mut elt: Lisp_Object;
  let mut lexenv: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();

  lexenv = (*Vinternal_interpreter_environment.as_mut_ptr ());

  let mut varlist: Lisp_Object = XCAR (args);
  FOR_EACH_TAIL! (varlist, {
      elt = XCAR (varlist);
      if SYMBOLP (elt)
	{
	  var = elt;
	  val = Qnil ();
	}
      else
	{
	  var = Fcar (elt);
	  if ! NILP (Fcdr (XCDR (elt)))
	    { signal_error (c"`let' bindings can have only one value-form".as_ptr (), elt); }
	  val = eval_sub (Fcar (XCDR (elt)));
	}

      var = maybe_remove_pos_from_symbol (var);
      CHECK_TYPE (BARE_SYMBOL_P (var), Qsymbolp (), var);
      if !NILP (lexenv)
	  && (*XBARE_SYMBOL (var)).u.s.bits & SYMBOL_DECLARED_SPECIAL_BIT == 0
	  && NILP (memq_lexenv (var, (*Vinternal_interpreter_environment.as_mut_ptr ())))
	/* Lexically bind VAR by adding it to the interpreter's binding
	   alist.  */
	{
	  let newenv: Lisp_Object
	    = Fcons (Fcons (var, val), (*Vinternal_interpreter_environment.as_mut_ptr ()));
	  if BASE_EQ ((*Vinternal_interpreter_environment.as_mut_ptr ()), lexenv)
	    /* Save the old lexical environment on the specpdl stack,
	       but only for the first lexical binding, since we'll never
	       need to revert to one of the intermediate ones.  */
	    { specbind (Qinternal_interpreter_environment (), newenv); }
	  else
	    { (*Vinternal_interpreter_environment.as_mut_ptr ()) = newenv; }
	}
      else
	{ specbind (var, val); }
  });
  CHECK_LIST_END (varlist, XCAR (args));

  val = Fprogn (XCDR (args));
  return unbind_to (count, val);
});

DEFUN! ("let", Flet, Slet, 1, UNEVALLED, 0,
       doc: "Bind variables according to VARLIST then eval BODY.\nThe value of the last form in BODY is returned.\nEach element of VARLIST is a symbol (which is bound to nil)\nor a list (SYMBOL VALUEFORM) (which binds SYMBOL to the value of VALUEFORM).\nAll the VALUEFORMs are evalled before any symbols are bound.\nusage: (let VARLIST BODY...)",
       (args: Lisp_Object),
       {
  let mut temps: *mut Lisp_Object;
  let mut tem: Lisp_Object;
  let mut lexenv: Lisp_Object;
  let mut elt: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut argnum: ptrdiff_t;

  let mut varlist: Lisp_Object = XCAR (args);

  if CONSP (varlist) && NILP (XCDR (varlist))
    {
      maybe_quit ();
      elt = XCAR (varlist);
      let var: Lisp_Object;
      if SYMBOLP (elt)
	{
	  var = maybe_remove_pos_from_symbol (elt);
	  tem = Qnil ();
	}
      else if CONSP (elt)
	{
	  let cdr_elt: Lisp_Object = XCDR (elt);
	  if CONSP (cdr_elt)
	    {
	      if ! NILP (XCDR (cdr_elt))
		{ signal_error (c"`let' bindings can have only one value-form".as_ptr (), elt); }
	      tem = eval_sub (XCAR (cdr_elt));
	    }
	  else if NILP (cdr_elt)
	    { tem = Qnil (); }
	  else
	    { signal_error (c"`let' bindings can have only one value-form".as_ptr (), elt); }
	  var = maybe_remove_pos_from_symbol (XCAR (elt));
	}
      else
	{
	  if ! NILP (Fcdr (Fcdr (elt)))
	    { signal_error (c"`let' bindings can have only one value-form".as_ptr (), elt); }
	  tem = eval_sub (Fcar (Fcdr (elt)));
	  var = maybe_remove_pos_from_symbol (Fcar (elt));
	}
      CHECK_TYPE (BARE_SYMBOL_P (var), Qsymbolp (), var);
      lexenv = (*Vinternal_interpreter_environment.as_mut_ptr ());
      if !NILP (lexenv)
	  && (*XBARE_SYMBOL (var)).u.s.bits & SYMBOL_DECLARED_SPECIAL_BIT == 0
	  && NILP (memq_lexenv (var, lexenv))
	{ specbind (Qinternal_interpreter_environment (), Fcons (Fcons (var, tem), lexenv)); }
      else
	{ specbind (var, tem); }
      elt = Fprogn (XCDR (args));
      return unbind_to (count, elt);
    }

  /* Make space to hold the values to give the bound variables.  */
  let varlist_len: EMACS_INT = fast_list_length (varlist);
  if varlist_len <= 8
    {
      let mut temps_buf: [Lisp_Object; 8] = [Qnil (); 8];
      temps = temps_buf.as_mut_ptr ();
      let _temps_span = crate::roots::ScopedRawSpan::new (temps, varlist_len as usize);
      let mut nvars: ptrdiff_t = varlist_len;
      argnum = 0;
      while argnum < nvars && CONSP (varlist)
	{
	  maybe_quit ();
	  elt = XCAR (varlist);
	  varlist = XCDR (varlist);
	  if SYMBOLP (elt)
	    { *temps.offset (argnum) = Qnil (); }
	  else if ! NILP (Fcdr (Fcdr (elt)))
	    { signal_error (c"`let' bindings can have only one value-form".as_ptr (), elt); }
	  else
	    { *temps.offset (argnum) = eval_sub (Fcar (Fcdr (elt))); }
	  argnum += 1;
	}
      nvars = argnum;
      lexenv = (*Vinternal_interpreter_environment.as_mut_ptr ());
      varlist = XCAR (args);
      argnum = 0;
      while argnum < nvars && CONSP (varlist)
	{
	  elt = XCAR (varlist);
	  varlist = XCDR (varlist);
	  let var: Lisp_Object = maybe_remove_pos_from_symbol (if SYMBOLP (elt) { elt }
							       else if CONSP (elt) { XCAR (elt) }
							       else { Fcar (elt) });
	  CHECK_TYPE (BARE_SYMBOL_P (var), Qsymbolp (), var);
	  tem = *temps.offset (argnum);

	  if !NILP (lexenv)
	      && (*XBARE_SYMBOL (var)).u.s.bits & SYMBOL_DECLARED_SPECIAL_BIT == 0
	      && NILP (memq_lexenv (var, (*Vinternal_interpreter_environment.as_mut_ptr ())))
	    /* Lexically bind VAR by adding it to the lexenv alist.  */
	    { lexenv = Fcons (Fcons (var, tem), lexenv); }
	  else
	    /* Dynamically bind VAR.  */
	    { specbind (var, tem); }
	  argnum += 1;
	}
      if !BASE_EQ (lexenv, (*Vinternal_interpreter_environment.as_mut_ptr ()))
	/* Instantiate a new lexical environment.  */
	{ specbind (Qinternal_interpreter_environment (), lexenv); }

      elt = Fprogn (XCDR (args));
      return unbind_to (count, elt);
    }

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  temps = std::ptr::null_mut ();
  SAFE_ALLOCA_LISP! (sa_avail, temps, varlist_len);
  let mut temps_span = crate::roots::ScopedRawSpan::new (temps, 0);
  let mut nvars: ptrdiff_t = varlist_len;

  /* Compute the values and store them in `temps'.  */

  argnum = 0;
  while argnum < nvars && CONSP (varlist)
    {
      maybe_quit ();
      elt = XCAR (varlist);
      varlist = XCDR (varlist);
      if SYMBOLP (elt)
	{ *temps.offset (argnum) = Qnil (); }
      else if ! NILP (Fcdr (Fcdr (elt)))
	{ signal_error (c"`let' bindings can have only one value-form".as_ptr (), elt); }
      else
	{ *temps.offset (argnum) = eval_sub (Fcar (Fcdr (elt))); }
      argnum += 1;
      temps_span.set_len (argnum as usize);
    }
  nvars = argnum;

  lexenv = (*Vinternal_interpreter_environment.as_mut_ptr ());

  varlist = XCAR (args);
  argnum = 0;
  while argnum < nvars && CONSP (varlist)
    {
      elt = XCAR (varlist);
      varlist = XCDR (varlist);
      let var: Lisp_Object = maybe_remove_pos_from_symbol (if SYMBOLP (elt) { elt }
						          else { Fcar (elt) });
      CHECK_TYPE (BARE_SYMBOL_P (var), Qsymbolp (), var);
      tem = *temps.offset (argnum);

      if !NILP (lexenv)
	  && (*XBARE_SYMBOL (var)).u.s.bits & SYMBOL_DECLARED_SPECIAL_BIT == 0
	  && NILP (memq_lexenv (var, (*Vinternal_interpreter_environment.as_mut_ptr ())))
	/* Lexically bind VAR by adding it to the lexenv alist.  */
	{ lexenv = Fcons (Fcons (var, tem), lexenv); }
      else
	/* Dynamically bind VAR.  */
	{ specbind (var, tem); }
      argnum += 1;
    }

  if !BASE_EQ (lexenv, (*Vinternal_interpreter_environment.as_mut_ptr ()))
    /* Instantiate a new lexical environment.  */
    { specbind (Qinternal_interpreter_environment (), lexenv); }

  elt = Fprogn (XCDR (args));
  return SAFE_FREE_UNBIND_TO! (sa_count, count, elt, sa_mark);
});

DEFUN! ("while", Fwhile, Swhile, 1, UNEVALLED, 0,
       doc: "If TEST yields non-nil, eval BODY... and repeat.\nThe order of execution is thus TEST, BODY, TEST, BODY and so on\nuntil TEST returns nil.\n\nThe value of a `while' form is always nil.\n\nusage: (while TEST BODY...)",
       (args: Lisp_Object),
       {
  let test: Lisp_Object;
  let body: Lisp_Object;

  test = XCAR (args);
  body = XCDR (args);
  while !NILP (eval_sub (test))
    {
      maybe_quit ();
      prog_ignore (body);
    }

  return Qnil ();
});

/* C 1223-1227's `struct funcall_with_delayed_message_data` (the one type eval.c declares;
   `struct atimer` comes from atimer.h, so this unit includes that mirror).  */
#[repr(C)]
pub struct funcall_with_delayed_message_data
{
  pub message: *mut Lisp_Object,
  pub timer: *mut atimer,
}
const _: () = core::assert! (std::mem::size_of::<funcall_with_delayed_message_data> () == 16);
const _: () = core::assert! (std::mem::offset_of! (funcall_with_delayed_message_data, message) == 0);
const _: () = core::assert! (std::mem::offset_of! (funcall_with_delayed_message_data, timer) == 8);

unsafe extern "C-unwind" fn with_delayed_message_display (timer: *mut atimer)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let data: *mut funcall_with_delayed_message_data
    = (*timer).client_data as *mut funcall_with_delayed_message_data;
  if !(*data).timer.is_null ()
    {
      message3 (*(*data).message);
      (*data).timer = std::ptr::null_mut ();
    }

  }
}

unsafe extern "C-unwind" fn with_delayed_message_cancel (datap: *mut c_void)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let data: *mut funcall_with_delayed_message_data
    = datap as *mut funcall_with_delayed_message_data;
  if !(*data).timer.is_null ()
    {
      cancel_atimer ((*data).timer);
      (*data).timer = std::ptr::null_mut ();
    }

  }
}

DEFUN! ("funcall-with-delayed-message",
       Ffuncall_with_delayed_message, Sfuncall_with_delayed_message,
       3, 3, 0,
       doc: "Like `funcall', but display MESSAGE if FUNCTION takes longer than TIMEOUT.\nTIMEOUT is a number of seconds, and can be an integer or a floating\npoint number.\n\nIf FUNCTION takes less time to execute than TIMEOUT seconds, MESSAGE\nis not displayed.",
       (timeout: Lisp_Object, message: Lisp_Object, function: Lisp_Object),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();

  CHECK_NUMBER (timeout);
  CHECK_STRING (message);

  /* Set up the atimer.  */
  let interval: timespec = dtotimespec (XFLOATINT (timeout));
  let mut data: funcall_with_delayed_message_data
    = funcall_with_delayed_message_data { message: &raw const message as *mut Lisp_Object,
                                          timer: std::ptr::null_mut () };
  data.timer = start_atimer (ATIMER_RELATIVE, interval,
			     Some (with_delayed_message_display),
			     &raw mut data as *mut c_void);
  record_unwind_protect_ptr (Some (with_delayed_message_cancel),
                             &raw mut data as *mut c_void);

  let result: Lisp_Object = calln! (function);

  return unbind_to (count, result);
});

DEFUN! ("macroexpand", Fmacroexpand, Smacroexpand, 1, 2, 0,
       doc: "Return result of expanding macros at top level of FORM.\nIf FORM is not a macro call, it is returned unchanged.\nOtherwise, the macro is expanded and the expansion is considered\nin place of FORM.  When a non-macro-call results, it is returned.\n\nThe second optional arg ENVIRONMENT specifies an environment of macro\ndefinitions to shadow the loaded ones for use in file byte-compilation.",
       (form: Lisp_Object, environment: Lisp_Object),
       {
  /* With cleanups from Hallvard Furuseth.  */
  let mut form: Lisp_Object = form;   // C assigns to its parameter
  let mut expander: Lisp_Object;
  let mut sym: Lisp_Object;
  let mut def: Lisp_Object;
  let mut tem: Lisp_Object;

  loop
    {
      /* Come back here each time we expand a macro call,
	 in case it expands into another macro call.  */
      if !CONSP (form)
	{ break; }
      /* Set SYM, give DEF and TEM right values in case SYM is not a symbol. */
      def = XCAR (form);
      sym = def;
      tem = Qnil ();
      /* Trace symbols aliases to other symbols
	 until we get a symbol that is not an alias.  */
      while SYMBOLP (def)
	{
	  maybe_quit ();
	  sym = def;
	  tem = if NILP (environment) { Qnil () } else { Fassq (sym, environment) };
	  if NILP (tem)
	    {
	      def = (*XSYMBOL (sym)).u.s.function;
	      if !NILP (def)
		{ continue; }
	    }
	  break;
	}
      /* Right now TEM is the result from SYM in ENVIRONMENT,
	 and if TEM is nil then DEF is SYM's function definition.  */
      if NILP (tem)
	{
	  /* SYM is not mentioned in ENVIRONMENT.
	     Look at its function definition.  */
	  if !CONSP (def)
	    /* Not defined or definition not suitable.  */
	    { break; }
	  if EQ (XCAR (def), Qautoload ())
	    {
	      def = Fautoload_do_load (def, sym, Qmacro ());
	      if !CONSP (def)
		/* Not defined or definition not suitable.  */
		{ break; }
	    }
	  if !EQ (XCAR (def), Qmacro ())
	    { break; }
	  else
	    { expander = XCDR (def); }
	}
      else
	{
	  expander = XCDR (tem);
	  if NILP (expander)
	    { break; }
	}
      {
	let newform: Lisp_Object = apply1 (expander, XCDR (form));
	if EQ (form, newform)
	  { break; }
	else
	  { form = newform; }
      }
    }
  return form;
});

DEFUN! ("catch", Fcatch, Scatch, 1, UNEVALLED, 0,
       doc: "Eval BODY allowing nonlocal exits using `throw'.\nTAG is evalled to get the tag to use; it must not be nil.\n\nThen the BODY is executed.\nWithin BODY, a call to `throw' with the same TAG exits BODY and this `catch'.\nIf no throw happens, `catch' returns the value of the last BODY form.\nIf a throw happens, it specifies the value to return from `catch'.\nusage: (catch TAG BODY...)",
       (args: Lisp_Object),
       {
  let tag: Lisp_Object = eval_sub (XCAR (args));
  return internal_catch (tag, Some (Fprogn as _), XCDR (args));
});

/* Assert that E is true, but do not evaluate E.  Use this instead of
   eassert (E) when E contains variables that might be clobbered by a
   longjmp.  */
/* C 1370-1373: `#define clobbered_eassert(E) static_assert (sizeof (E) != 0)`.  `sizeof`
   is not evaluated (C 1373's own comment), and Rust's `size_of_val` on a reference is the
   same compile-time constant for the expression's type; a bitfield (which C's `sizeof`
   rejects) has no such standalone place here, which the ledger records.  */
macro_rules! clobbered_eassert {
  ($e:expr) => {
    {
      /* C's `sizeof (E)` is not evaluated, and a Rust `const` item cannot name a type
	 through an expression that mentions the function's locals (E0435), so the port
	 asserts the size of the type C's `sizeof` rejects a bitfield for and keeps E's own
	 spelling unevaluated.  Recorded in docs/ports/eval.md.  */
      static_assert! (std::mem::size_of::<bool> () != 0);
      let _ = stringify! ($e);
    }
  };
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn pop_handler ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  handlerlist! () = (*handlerlist! ()).next;

  }
}

/* Set up a catch, then call C function FUNC on argument ARG.
   FUNC should return a Lisp_Object.
   This is how catches are done from within C code.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn internal_catch (tag: Lisp_Object,
                                         func: Option<unsafe extern "C-unwind" fn (Lisp_Object) -> Lisp_Object>,
                                         arg: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* This structure is made part of the chain `catchlist'.  */
  let c: *mut handler = push_handler (tag, CATCHER);
  let _arg_root = crate::nonlocal::ScopedLispRoot::new (&[tag, arg]);

  /* Call FUNC.  */
  match crate::nonlocal::catch_vm_nonlocal (|| (func.unwrap ()) (arg))
    {
      Ok (val) =>
        {
          eassert! (handlerlist! () == c);
          handlerlist! () = (*c).next;
          return val;
        }
      Err (exit) =>
        {
          if exit.target != c
            { crate::nonlocal::resume_vm_nonlocal (exit); }
          let val: Lisp_Object = (*handlerlist! ()).val;
          clobbered_eassert! (handlerlist! () == c);
          handlerlist! () = (*handlerlist! ()).next;
          return val;
        }
    }

  }
}

/* Unwind the specbind, catch, and handler stacks back to CATCH, and
   return VALUE as the value of that catch via explicit VM unwinding.

   This is the guts of Fthrow and Fsignal; they differ only in the way
   they choose the catch tag to throw to.  A catch tag for a
   condition-case form has a TAG of Qnil.

   Before each catch is discarded, unbind all special bindings and
   execute all unwind-protect clauses made above that catch.  Unwind
   the handler stack as we go, so that the proper handlers are in
   effect for each unwind-protect clause we run.  At the end, restore
   some static info saved in CATCH, and unwind to CATCH.

   This is used for correct unwinding in Fthrow and Fsignal.  */

unsafe fn unwind_to_catch (catch: *mut handler, r#type: nonlocal_exit,
                           value: Lisp_Object) -> !
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut last_time: bool;
  let _value_root = crate::nonlocal::ScopedLispRoot::new (&[value]);

  eassert! (!(*catch).next.is_null ());

  /* Save the value in the tag.  */
  (*catch).nonlocal_exit = r#type;
  (*catch).val = value;

  /* Restore certain special C variables.  */
  if (*catch).poll_suppress_count != (*poll_suppress_count.as_mut_ptr ())
    { set_poll_suppress_count ((*catch).poll_suppress_count); }
  if (*catch).interrupt_input_blocked != (*interrupt_input_blocked.as_mut_ptr ())
    { unblock_input_to ((*catch).interrupt_input_blocked); }

  /* C 1438-1443's `#ifdef HAVE_X_WINDOWS` arm (dead in the frozen configuration; P-51):
     restore the X error handler stack.  */
  #[cfg (feature = "have_x_windows")]
  {
    unsafe extern "C" {
      /// C: xterm.h 1763: `extern void x_unwind_errors_to (int);`
      pub fn x_unwind_errors_to (depth: c_int);
    }
    x_unwind_errors_to ((*catch).x_error_handler_depth);
  }

  loop
    {
      /* Unwind the specpdl stack, and then restore the proper set of
	 handlers.  */
      let target_pdl: specpdl_ref = (*handlerlist! ()).pdlcount;
      if SPECPDL_INDEX () != target_pdl
	{ unbind_to (target_pdl, Qnil ()); }
      last_time = handlerlist! () == catch;
      if ! last_time
	{ handlerlist! () = (*handlerlist! ()).next; }
      if last_time
        { break; }
    }

  eassert! (handlerlist! () == catch);

  lisp_eval_depth! () = (*catch).f_lisp_eval_depth;
  set_act_rec ((*current_thread.as_mut_ptr ()), (*catch).act_rec);

  crate::nonlocal::raise_vm_nonlocal (crate::nonlocal::NonLocalExit {
    kind: crate::nonlocal::NonLocalExitKind::from_raw (r#type),
    target: catch,
    value: (*catch).val,
  });

  }
}

DEFUN! ("throw", Fthrow, Sthrow, 2, 2, 0,
       doc: "Throw to the catch for TAG and return VALUE from it.\nBoth TAG and VALUE are evalled.",
       attributes: noreturn,
       (tag: Lisp_Object, value: Lisp_Object),
       {
  let mut c: *mut handler;

  if !NILP (tag)
    {
      c = handlerlist! ();
      while !c.is_null ()
	{
	  if (*c).r#type == CATCHER_ALL || (*c).r#type == CATCHER_ALL_DEBUGGABLE
	    { unwind_to_catch (c, NONLOCAL_EXIT_THROW, Fcons (tag, value)); }
	  if (*c).r#type == CATCHER && EQ ((*c).tag_or_ch, tag)
	    { unwind_to_catch (c, NONLOCAL_EXIT_THROW, value); }
	  c = (*c).next;
	}
    }
  xsignal2 (Qno_catch (), tag, value);
});

DEFUN! ("unwind-protect", Funwind_protect, Sunwind_protect, 1, UNEVALLED, 0,
       doc: "Do BODYFORM, protecting with UNWINDFORMS.\nIf BODYFORM completes normally, its value is returned\nafter executing the UNWINDFORMS.\nIf BODYFORM exits nonlocally, the UNWINDFORMS are executed anyway.\nusage: (unwind-protect BODYFORM UNWINDFORMS...)",
       (args: Lisp_Object),
       {
  let val: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();

  record_unwind_protect (Some (prog_ignore), XCDR (args));
  val = eval_sub (XCAR (args));
  return unbind_to (count, val);
});

DEFUN! ("condition-case", Fcondition_case, Scondition_case, 2, UNEVALLED, 0,
       doc: "Regain control when an error is signaled.\nExecutes BODYFORM and returns its value if no error happens.\nEach element of HANDLERS looks like (CONDITION-NAME BODY...)\nor (:success BODY...), where the BODY is made of Lisp expressions.\n\nA handler is applicable to an error if CONDITION-NAME is one of the\nerror's condition names.  Handlers may also apply when non-error\nsymbols are signaled (e.g., `quit').  A CONDITION-NAME of t applies to\nany symbol, including non-error symbols.  If multiple handlers are\napplicable, only the first one runs.\n\nThe car of a handler may be a list of condition names instead of a\nsingle condition name; then it handles all of them.  If the special\ncondition name `debug' is present in this list, it allows another\ncondition in the list to run the debugger if `debug-on-error' and the\nother usual mechanisms say it should (otherwise, `condition-case'\nsuppresses the debugger).\n\nWhen a handler handles an error, control returns to the `condition-case'\nand it executes the handler's BODY...\nwith VAR bound to (ERROR-SYMBOL . SIGNAL-DATA) from the error.\n(If VAR is nil, the handler can't access that information.)\nThen the value of the last BODY form is returned from the `condition-case'\nexpression.\n\nThe special handler (:success BODY...) is invoked if BODYFORM terminated\nwithout signaling an error.  BODY is then evaluated with VAR bound to\nthe value returned by BODYFORM.\n\nSee also the function `signal' for more info.\nusage: (condition-case VAR BODYFORM &rest HANDLERS)",
       (args: Lisp_Object),
       {
  let var: Lisp_Object = XCAR (args);
  let bodyform: Lisp_Object = XCAR (XCDR (args));
  let handlers: Lisp_Object = XCDR (XCDR (args));

  return internal_lisp_condition_case (var, bodyform, handlers);
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn push_handler_bind (conditions: Lisp_Object, handler: Lisp_Object,
                                            skip: c_int)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut conditions: Lisp_Object = conditions;   // C assigns to its parameter
  if !CONSP (conditions)
    { conditions = Fcons (conditions, Qnil ()); }
  let c: *mut handler = push_handler (conditions, HANDLER_BIND);
  (*c).val = handler;
  (*c).bytecode_dest = skip;

  }
}

DEFUN! ("handler-bind-1", Fhandler_bind_1, Shandler_bind_1, 1, MANY, 0,
       doc: "Set up error handlers around execution of BODYFUN.\nBODYFUN should be a function and it is called with no arguments.\nCONDITIONS should be a list of condition names (symbols).\nWhen an error is signaled during execution of BODYFUN, if that\nerror matches one of CONDITIONS, then the associated HANDLER is\ncalled with the error as argument.\nHANDLER should either transfer the control via a non-local exit,\nor return normally.\nIf it returns normally, the search for an error handler continues\nfrom where it left off.\n\nusage: (handler-bind BODYFUN [CONDITIONS HANDLER]...)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  eassert! (nargs >= 1);
  let bodyfun: Lisp_Object = *args.offset (0);
  let mut count: c_int = 0;
  if nargs % 2 == 0
    { error (c"Trailing CONDITIONS without HANDLER in `handler-bind`".as_ptr ()); }
  let mut i: ptrdiff_t = nargs - 2;
  while i > 0
    {
      let conditions: Lisp_Object = *args.offset (i);
      let handler: Lisp_Object = *args.offset (i + 1);
      if NILP (conditions)
        { i -= 2; continue; }
      push_handler_bind (conditions, handler, count);
      count += 1;
      i -= 2;
    }
  let ret: Lisp_Object = call0 (bodyfun);
  while count > 0
    { pop_handler (); count -= 1; }
  return ret;
});

/* Like Fcondition_case, but the args are separate
   rather than passed in a list.  Used by Fbyte_code.  */

#[unsafe(no_mangle)]
pub extern "C-unwind" fn internal_lisp_condition_case (var: Lisp_Object,
                                                       bodyform: Lisp_Object,
                                                       handlers: Lisp_Object)
  -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let oldhandlerlist: *mut handler = handlerlist! ();

  /* The number of non-success handlers, plus 1 for a sentinel.  */
  let mut clausenb: ptrdiff_t = 1;

  let mut var: Lisp_Object = maybe_remove_pos_from_symbol (var);
  CHECK_TYPE (BARE_SYMBOL_P (var), Qsymbolp (), var);

  let mut success_handler: Lisp_Object = Qnil ();

  let mut tail: Lisp_Object = handlers;
  while CONSP (tail)
    {
      let tem: Lisp_Object = XCAR (tail);
      if ! (NILP (tem)
	    || (CONSP (tem)
		&& (SYMBOLP (XCAR (tem))
		    || CONSP (XCAR (tem)))))
	{ error (c"Invalid condition handler: %s".as_ptr (),
	         SDATA (Fprin1_to_string (tem, Qt (), Qnil ()))); }
      if CONSP (tem) && EQ (XCAR (tem), QCsuccess ())
	{ success_handler = tem; }
      else
	{ clausenb += 1; }
      tail = XCDR (tail);
    }

  /* The first clause is the one that should be checked first, so it
     should be added to handlerlist last.  So build in CLAUSES a table
     that contains HANDLERS but in reverse order.  CLAUSES is pointer
     to volatile to avoid issues with setjmp and local storage.
     SAFE_ALLOCA won't work here due to the setjmp, so impose a
     MAX_ALLOCA limit.  */
  if MAX_ALLOCA / word_size < clausenb
    { memory_full_up (); }
  /* C's bare `alloca` is the port's stack pool (PORTING.md P-05's decision).  */
  let _cc_sa_guard = crate::common::SaStackUnwindGuard::enter (sa_stack_mark ());
  let mut clauses: *mut Lisp_Object
    = sa_stack_alloc (clausenb as usize * std::mem::size_of::<Lisp_Object> ())
      as *mut Lisp_Object;
  clauses = clauses.offset (clausenb);
  clauses = clauses.offset (-1);
  *clauses = make_fixnum (0);
  tail = handlers;
  while CONSP (tail)
    {
      let tem: Lisp_Object = XCAR (tail);
      if !(CONSP (tem) && EQ (XCAR (tem), QCsuccess ()))
	{ clauses = clauses.offset (-1); *clauses = tem; }
      tail = XCDR (tail);
    }
  let _cc_roots = crate::nonlocal::ScopedLispRoot::new (&[var, bodyform, handlers, success_handler]);
  let mut var_volatile: Lisp_Object = var;
  let mut val: Lisp_Object;
  let mut handler_body: Lisp_Object;
  let mut pushed_handlers: Vec<*mut handler> = Vec::with_capacity ((clausenb - 1).max (0) as usize);
  match crate::nonlocal::catch_vm_nonlocal (|| {
    let mut pcl: *mut Lisp_Object = clauses;
    while !BASE_EQ (*pcl, make_fixnum (0))
      {
        let clause: Lisp_Object = *pcl;
        let mut condition: Lisp_Object = if CONSP (clause) { XCAR (clause) } else { Qnil () };
        if !CONSP (condition)
          { condition = list1 (condition); }
        let c: *mut handler = push_handler (condition, CONDITION_CASE);
        pushed_handlers.push (c);
        pcl = pcl.offset (1);
      }
    eval_sub (bodyform)
  })
    {
      Ok (body_val) =>
        {
          val = body_val;
          handlerlist! () = oldhandlerlist;
          if NILP (success_handler)
            { return val; }
          #[cfg (feature = "gcc_lint_not_clang")]
          {
            var = var_volatile;
          }
          handler_body = XCDR (success_handler);
        }
      Err (exit) =>
        {
          if !pushed_handlers.contains (&exit.target)
            { crate::nonlocal::resume_vm_nonlocal (exit); }
          var = var_volatile;
          val = (*handlerlist! ()).val;
          let mut chosen_clause: *mut Lisp_Object = clauses;
          let oldh: *mut handler = oldhandlerlist;
          let mut h: *mut handler = (*handlerlist! ()).next;
          while h != oldh
            { chosen_clause = chosen_clause.offset (1); h = (*h).next; }
          handler_body = XCDR (*chosen_clause);
          handlerlist! () = oldh;
        }
    }

  if NILP (var)
    { return Fprogn (handler_body); }

  if !NILP ((*Vinternal_interpreter_environment.as_mut_ptr ()))
    {
      val = Fcons (Fcons (var, val),
		   (*Vinternal_interpreter_environment.as_mut_ptr ()));
      var = Qinternal_interpreter_environment ();
    }

  let count: specpdl_ref = SPECPDL_INDEX ();
  specbind (var, val);
  return unbind_to (count, Fprogn (handler_body));

  }
}

/* Call the function BFUN with no arguments, catching errors within it
   according to HANDLERS.  If there is an error, call HFUN with
   one argument which is the data that describes the error:
   (SIGNALNAME . DATA)

   HANDLERS can be a list of conditions to catch.
   If HANDLERS is Qt, catch all errors.
   If HANDLERS is Qerror, catch all errors
   but allow the debugger to run if that is enabled.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn internal_condition_case (
  bfun: Option<unsafe extern "C-unwind" fn () -> Lisp_Object>, handlers: Lisp_Object,
  hfun: Option<unsafe extern "C-unwind" fn (Lisp_Object) -> Lisp_Object>) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let c: *mut handler = push_handler (handlers, CONDITION_CASE);
  match crate::nonlocal::catch_vm_nonlocal (|| (bfun.unwrap ()) ())
    {
      Ok (val) =>
        {
          eassert! (handlerlist! () == c);
          handlerlist! () = (*c).next;
          return val;
        }
      Err (exit) =>
        {
          if exit.target != c
            { crate::nonlocal::resume_vm_nonlocal (exit); }
          let val: Lisp_Object = (*handlerlist! ()).val;
          clobbered_eassert! (handlerlist! () == c);
          handlerlist! () = (*handlerlist! ()).next;
          return (hfun.unwrap ()) (val);
        }
    }

  }
}

/* Like internal_condition_case but call BFUN with ARG as its argument.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn internal_condition_case_1 (
  bfun: Option<unsafe extern "C-unwind" fn (Lisp_Object) -> Lisp_Object>, arg: Lisp_Object,
  handlers: Lisp_Object,
  hfun: Option<unsafe extern "C-unwind" fn (Lisp_Object) -> Lisp_Object>) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let c: *mut handler = push_handler (handlers, CONDITION_CASE);
  let _arg_root = crate::nonlocal::ScopedLispRoot::new (&[arg, handlers]);
  match crate::nonlocal::catch_vm_nonlocal (|| (bfun.unwrap ()) (arg))
    {
      Ok (val) =>
        {
          eassert! (handlerlist! () == c);
          handlerlist! () = (*c).next;
          return val;
        }
      Err (exit) =>
        {
          if exit.target != c
            { crate::nonlocal::resume_vm_nonlocal (exit); }
          let val: Lisp_Object = (*handlerlist! ()).val;
          clobbered_eassert! (handlerlist! () == c);
          handlerlist! () = (*handlerlist! ()).next;
          return (hfun.unwrap ()) (val);
        }
    }

  }
}

/* Like internal_condition_case_1 but call BFUN with ARG1 and ARG2 as
   its arguments.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn internal_condition_case_2 (
  bfun: Option<unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object) -> Lisp_Object>,
  arg1: Lisp_Object, arg2: Lisp_Object, handlers: Lisp_Object,
  hfun: Option<unsafe extern "C-unwind" fn (Lisp_Object) -> Lisp_Object>) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let c: *mut handler = push_handler (handlers, CONDITION_CASE);
  let _arg_root = crate::nonlocal::ScopedLispRoot::new (&[arg1, arg2, handlers]);
  match crate::nonlocal::catch_vm_nonlocal (|| (bfun.unwrap ()) (arg1, arg2))
    {
      Ok (val) =>
        {
          eassert! (handlerlist! () == c);
          handlerlist! () = (*c).next;
          return val;
        }
      Err (exit) =>
        {
          if exit.target != c
            { crate::nonlocal::resume_vm_nonlocal (exit); }
          let val: Lisp_Object = (*handlerlist! ()).val;
          clobbered_eassert! (handlerlist! () == c);
          handlerlist! () = (*handlerlist! ()).next;
          return (hfun.unwrap ()) (val);
        }
    }

  }
}

/* Like internal_condition_case but call BFUN with NARGS as first,
   and ARGS as second argument.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn internal_condition_case_n (
  bfun: Option<unsafe extern "C-unwind" fn (isize, *mut Lisp_Object) -> Lisp_Object>,
  nargs: ptrdiff_t, args: *mut Lisp_Object, handlers: Lisp_Object,
  hfun: Option<unsafe extern "C-unwind" fn (Lisp_Object, isize, *mut Lisp_Object) -> Lisp_Object>)
  -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let c: *mut handler = push_handler (handlers, CONDITION_CASE);
  let arg_slice: &[Lisp_Object] = if nargs > 0 && !args.is_null ()
    { core::slice::from_raw_parts (args, nargs as usize) }
    else { &[] };
  let _arg_root = crate::nonlocal::ScopedLispRoot::new (arg_slice);
  match crate::nonlocal::catch_vm_nonlocal (|| (bfun.unwrap ()) (nargs, args))
    {
      Ok (val) =>
        {
          eassert! (handlerlist! () == c);
          handlerlist! () = (*c).next;
          return val;
        }
      Err (exit) =>
        {
          if exit.target != c
            { crate::nonlocal::resume_vm_nonlocal (exit); }
          let val: Lisp_Object = (*handlerlist! ()).val;
          clobbered_eassert! (handlerlist! () == c);
          handlerlist! () = (*handlerlist! ()).next;
          return (hfun.unwrap ()) (val, nargs, args);
        }
    }

  }
}

static Qcatch_all_memory_full: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

/* Like a combination of internal_condition_case_1 and internal_catch.
   Catches all signals and throws.  Never exits nonlocally; returns
   Qcatch_all_memory_full if no handler could be allocated.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn internal_catch_all (
  function: Option<unsafe extern "C-unwind" fn (*mut c_void) -> Lisp_Object>, argument: *mut c_void,
  handler: Option<unsafe extern "C-unwind" fn (nonlocal_exit, Lisp_Object) -> Lisp_Object>)
  -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let c: *mut handler = push_handler_nosignal (Qt (), CATCHER_ALL);
  if c.is_null ()
    { return (*Qcatch_all_memory_full.as_mut_ptr ()); }

  match crate::nonlocal::catch_vm_nonlocal (|| (function.unwrap ()) (argument))
    {
      Ok (val) =>
        {
          eassert! (handlerlist! () == c);
          handlerlist! () = (*c).next;
          return val;
        }
      Err (exit) =>
        {
          if exit.target != c
            { crate::nonlocal::resume_vm_nonlocal (exit); }
          eassert! (handlerlist! () == c);
          let r#type: nonlocal_exit = (*c).nonlocal_exit;
          let val: Lisp_Object = (*c).val;
          handlerlist! () = (*c).next;
          return (handler.unwrap ()) (r#type, val);
        }
    }

  }
}

#[inline(always)]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn push_handler (tag_ch_val: Lisp_Object, handlertype: handlertype)
  -> *mut handler
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let c: *mut handler = push_handler_nosignal (tag_ch_val, handlertype);
  if c.is_null ()
    { memory_full (std::mem::size_of::<handler> ()); }
  return c;

  }
}

#[inline(always)]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn push_handler_nosignal (tag_ch_val: Lisp_Object,
                                                handlertype: handlertype) -> *mut handler
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut c: *mut handler = (*handlerlist! ()).nextfree;
  if c.is_null ()
    {
      c = malloc (std::mem::size_of::<handler> ()) as *mut handler;
      if c.is_null ()
	{ return c; }
      if (*profiler_memory_running.as_mut_ptr ())
	{ malloc_probe (std::mem::size_of::<handler> ()); }
      (*c).nextfree = std::ptr::null_mut ();
      (*handlerlist! ()).nextfree = c;
    }
  (*c).r#type = handlertype;
  (*c).tag_or_ch = tag_ch_val;
  (*c).val = Qnil ();
  (*c).next = handlerlist! ();
  (*c).f_lisp_eval_depth = lisp_eval_depth! ();
  (*c).pdlcount = SPECPDL_INDEX ();
  (*c).act_rec = get_act_rec ((*current_thread.as_mut_ptr ()));
  (*c).poll_suppress_count = (*poll_suppress_count.as_mut_ptr ());
  (*c).interrupt_input_blocked = (*interrupt_input_blocked.as_mut_ptr ());
  /* C 1862-1864's `#ifdef HAVE_X_WINDOWS` arm (dead in the frozen configuration).  */
  #[cfg (feature = "have_x_windows")]
  {
    unsafe extern "C" {
      /// C: xterm.h 2019: `extern int x_error_message_count;`
      pub static x_error_message_count: VmGlobal<c_int>;
    }
    (*c).x_error_handler_depth = (*x_error_message_count.as_mut_ptr ());
  }
  handlerlist! () = c;
  return c;

  }
}

/* C 1867-1869's three forward declarations are dropped (PORTING.md 0.1-12).  */

unsafe fn process_quit_flag ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let flag: Lisp_Object = (*Vquit_flag.as_mut_ptr ());
  (*Vquit_flag.as_mut_ptr ()) = Qnil ();
  if EQ (flag, Qkill_emacs ())
    { Fkill_emacs (Qnil (), Qnil ()); }
  if EQ ((*Vthrow_on_input.as_mut_ptr ()), flag)
    { Fthrow ((*Vthrow_on_input.as_mut_ptr ()), Qt ()); }
  quit ();

  }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn probably_quit ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let gc_count: specpdl_ref = inhibit_garbage_collection ();
  if !NILP ((*Vquit_flag.as_mut_ptr ())) && NILP ((*Vinhibit_quit.as_mut_ptr ()))
    { process_quit_flag (); }
  else if (*pending_signals.as_mut_ptr ())
    { process_pending_signals (); }
  unbind_to (gc_count, Qnil ());

  }
}

DEFUN! ("signal", Fsignal, Ssignal, 1, 2, 0,
       doc: "Signal an error.  Args are ERROR-SYMBOL and associated DATA.\nThis function does not return.\n\nWhen signaling a new error, the DATA argument is mandatory.\nWhen re-signaling an error to propagate it to further handlers,\nDATA has to be omitted and the first argument has to be the whole\nerror descriptor.\n\nWhen `noninteractive' is non-nil (in particular, in batch mode), an\nunhandled error calls `kill-emacs', which terminates the Emacs\nsession with a non-zero exit code.\n\nAn error symbol is a symbol with an `error-conditions' property\nthat is a list of condition names.  The symbol should be non-nil.\nA handler for any of those names will get to handle this signal.\nThe symbol `error' should normally be one of them.\n\nDATA should be a list.  Its elements are printed as part of the error message.\nSee Info anchor `(elisp)Definition of signal' for some details on how this\nerror message is constructed.\nIf the signal is handled, DATA is made available to the handler.\nSee also the function `condition-case'.",
       attributes: noreturn,
       (error_symbol: Lisp_Object, data: Lisp_Object),
       {
  let mut error_symbol: Lisp_Object = error_symbol;   // C assigns to its parameter
  /* If they call us with nonsensical arguments, produce "peculiar error".  */
  if NILP (error_symbol) && !CONSP (data)
    { error_symbol = Qerror (); }
  signal_or_quit (error_symbol, data, false);
  eassume! (false);
  /* C 1892-1893: this subr is declared `_Noreturn` (lisp.h's EXFUN), so the port's body
     must diverge; C's `eassume (false)` above is `__builtin_unreachable ()` here
     (lib/verify.h 389), which Rust spells out.  */
  { core::hint::unreachable_unchecked () }
});

/* Quit, in response to a keyboard quit request.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn quit () -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  return signal_or_quit (Qquit (), Qnil (), true);

  }
}

/* Signal an error, or quit.  ERROR_SYMBOL and DATA are as with Fsignal.
   If CONTINUABLE, the caller allows this function to return
   (presumably after calling the debugger);
   Otherwise this function is like Fsignal and does not return.  */

unsafe fn signal_or_quit (error_symbol: Lisp_Object, data: Lisp_Object, continuable: bool)
  -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* When memory is full, ERROR-SYMBOL is nil,
     and DATA is (REAL-ERROR-SYMBOL . REAL-DATA).
     That is a special case--don't do this in other situations.  */
  let oom: bool = NILP (error_symbol);
  let error: Lisp_Object             /* The error object.  */
    = if oom { data }
      else if !SYMBOLP (error_symbol) && NILP (data) { error_symbol }
      else { Fcons (error_symbol, data) };
  let conditions: Lisp_Object;
  let string: Lisp_Object;
  let real_error_symbol: Lisp_Object
    = if CONSP (error) { XCAR (error) } else { error_symbol };
  let mut clause: Lisp_Object = Qnil ();
  let mut signal_roots = crate::nonlocal::ScopedLispRoot::new (
    &[error_symbol, data, error, real_error_symbol, Qnil (), Qnil ()]);
  let mut h: *mut handler;
  let mut skip: c_int;

  if (*gc_in_progress.as_mut_ptr ()) || waiting_for_input! ()
    { emacs_abort (); }

  /* This hook is used by edebug.  */
  if ! NILP ((*Vsignal_hook_function.as_mut_ptr ()))
      && !oom
    {
      let count: specpdl_ref = SPECPDL_INDEX ();
      max_ensure_room (20);
      /* FIXME: 'handler-bind' makes `signal-hook-function' obsolete?  */
      /* FIXME: Here we still "split" the error object
         into its error-symbol and its error-data?  */
      calln! ((*Vsignal_hook_function.as_mut_ptr ()), real_error_symbol,
	      if NILP (data) && CONSP (error) { XCDR (error) } else { data });
      unbind_to (count, Qnil ());
    }

  conditions = Fget (real_error_symbol, Qerror_conditions ());
  signal_roots.set (4, conditions);
  if NILP (conditions)
    { signal_error (c"Invalid error symbol".as_ptr (), real_error_symbol); }

  /* Remember from where signal was called.  Skip over the frame for
     `signal' itself.  If a frame for `error' follows, skip that,
     too.  Don't do this when ERROR_SYMBOL is nil, because that
     is a memory-full error.  */
  (*Vsignaling_function.as_mut_ptr ()) = Qnil ();
  if !oom
    {
      let mut pdl: *mut specbinding = backtrace_next (backtrace_top ());
      if backtrace_p (pdl) && EQ ((backtrace_function.unwrap ()) (pdl), Qerror ())
	{ pdl = backtrace_next (pdl); }
      if backtrace_p (pdl)
	{ (*Vsignaling_function.as_mut_ptr ()) = (backtrace_function.unwrap ()) (pdl); }
    }

  skip = 0;
  h = handlerlist! ();
  'handler_scan: while !h.is_null ()
    {
      let mut do_continue: bool = false;
      match (*h).r#type
        {
        CATCHER_ALL =>
          {
            clause = Qt ();
          }
        CATCHER_ALL_DEBUGGABLE =>
          {
            clause = Qdebug ();
          }
	CATCHER =>
	  {
	    do_continue = true;
	  }
        CONDITION_CASE =>
          {
            clause = find_handler_clause ((*h).tag_or_ch, conditions);
          }
	HANDLER_BIND =>
	  {
	    if !NILP (find_handler_clause ((*h).tag_or_ch, conditions))
	      {
	        let count: specpdl_ref = SPECPDL_INDEX ();
	        /* Add some room in case this is for debugging, as in
		   call_debugger.  */
	        max_ensure_room (200);
	        push_handler (make_fixnum (skip as EMACS_INT + (*h).bytecode_dest as EMACS_INT),
	                      SKIP_CONDITIONS);
	        calln! ((*h).val, error);
	        unbind_to (count, Qnil ());
	        pop_handler ();
	      }
	    do_continue = true;
	  }
	SKIP_CONDITIONS =>
	  {
	    let mut toskip: c_int = XFIXNUM ((*h).tag_or_ch) as c_int;
	    while toskip >= 0
	      { h = (*h).next; toskip -= 1; }
	    do_continue = true;
	  }
	_ =>
	  {
	    abort ();
	  }
	}
      if do_continue
        { skip += 1; h = (*h).next; continue 'handler_scan; }
      if !NILP (clause)
	{
	  signal_roots.set (5, clause);
	  break;
	}
      skip += 1;
      h = (*h).next;
    }

  if /* Don't run the debugger for a memory-full error.
	 (There is no room in memory to do that!)  */
      !oom
      && (!NILP ((*Vdebug_on_signal.as_mut_ptr ()))
	  /* If no handler is present now, try to run the debugger.  */
	  || NILP (clause)
	  /* A `debug' symbol in the handler list disables the normal
	     suppression of the debugger.  */
	  || EQ (clause, Qdebug ())
	  || (CONSP (clause) && !NILP (Fmemq (Qdebug (), clause)))
	  /* Special handler that means "print a message and run debugger
	     if requested".  */
	  || EQ (clause, Qerror ()))
    {
      let debugger_called: bool
	= maybe_call_debugger (conditions, error);
      /* We can't return values to code which signaled an error, but we
	 can continue code which has signaled a quit.  */
      if continuable && debugger_called
	{ return Qnil (); }
    }

  if !NILP (clause)
    { unwind_to_catch (h, NONLOCAL_EXIT_SIGNAL, error); }
  else if handlerlist! () != handlerlist_sentinel! ()
    /* FIXME: This will come right back here if there's no `top-level'
       catcher.  A better solution would be to abort here, and instead
       add a catch-all condition handler so we never come here.  */
    { Fthrow (Qtop_level (), Qt ()); }

  string = Ferror_message_string (error);
  fatal (c"%s".as_ptr (), SDATA (string));

  }
}

/* Like xsignal, but takes 0, 1, 2, or 3 args instead of a list.  */

#[unsafe(no_mangle)]
pub extern "C-unwind" fn xsignal0 (error_symbol: Lisp_Object) -> !
{
  xsignal (error_symbol, Qnil ());
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn xsignal1 (error_symbol: Lisp_Object, arg: Lisp_Object) -> !
{
  {
  xsignal (error_symbol, list1 (arg));

  }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn xsignal2 (error_symbol: Lisp_Object, arg1: Lisp_Object,
                                   arg2: Lisp_Object) -> !
{
  {
  xsignal (error_symbol, list2 (arg1, arg2));

  }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn xsignal3 (error_symbol: Lisp_Object, arg1: Lisp_Object,
                                   arg2: Lisp_Object, arg3: Lisp_Object) -> !
{
  {
  xsignal (error_symbol, list3 (arg1, arg2, arg3));

  }
}

/* Signal `error' with message S, and additional arg ARG.
   If ARG is not a proper list, make it a one-element list.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn signal_error (s: *const c_char, arg: Lisp_Object) -> !
{
  {
  let mut arg: Lisp_Object = arg;   // C assigns to its parameter
  if NILP (Fproper_list_p (arg))
    { arg = list1 (arg); }

  xsignal (Qerror (), Fcons (build_string (s), arg));

  }
}

/* Simplified version of 'define-error'.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn define_error (name: Lisp_Object, message: *const c_char,
                                       parent: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! (SYMBOLP (name));
  eassert! (SYMBOLP (parent));
  let parent_conditions: Lisp_Object = Fget (parent, Qerror_conditions ());
  eassert! (CONSP (parent_conditions));
  eassert! (!NILP (Fmemq (parent, parent_conditions)));
  eassert! (NILP (Fmemq (name, parent_conditions)));
  Fput (name, Qerror_conditions (), Fcons (name, parent_conditions));
  Fput (name, Qerror_message (), build_string (message));

  }
}

/* Use this for arithmetic overflow, e.g., when an integer result is
   too large even for a bignum.  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn overflow_error ()
{
  {
  xsignal0 (Qoverflow_error ());

  }
}


/* Return true if LIST is a non-nil atom or
   a list containing one of CONDITIONS.  */

unsafe fn wants_debugger (list: Lisp_Object, conditions: Lisp_Object) -> bool
{
  let mut conditions: Lisp_Object = conditions;   // C assigns to its parameter
  if NILP (list)
    { return false; }
  if ! CONSP (list)
    { return true; }

  while CONSP (conditions)
    {
      let this: Lisp_Object;
      let mut tail: Lisp_Object;
      this = XCAR (conditions);
      tail = list;
      while CONSP (tail)
	{
	  if EQ (XCAR (tail), this)
	    { return true; }
	  tail = XCDR (tail);
	}
      conditions = XCDR (conditions);
    }
  return false;
}

/* Return true if an error with condition-symbols CONDITIONS,
   and described by SIGNAL-DATA, should skip the debugger
   according to debugger-ignored-errors.  */

unsafe fn skip_debugger (conditions: Lisp_Object, data: Lisp_Object) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut tail: Lisp_Object;
  let mut first_string: bool = true;
  let mut error_message: Lisp_Object;

  error_message = Qnil ();
  tail = (*Vdebug_ignored_errors.as_mut_ptr ());
  while CONSP (tail)
    {
      if STRINGP (XCAR (tail))
	{
	  if first_string
	    {
	      error_message = Ferror_message_string (data);
	      first_string = false;
	    }

	  if fast_string_match (XCAR (tail), error_message) >= 0
	    { return true; }
	}
      else
	{
	  let mut contail: Lisp_Object;

	  contail = conditions;
	  while CONSP (contail)
	    {
	      if EQ (XCAR (tail), XCAR (contail))
		{ return true; }
	      contail = XCDR (contail);
	    }
	}
      tail = XCDR (tail);
    }

  return false;

  }
}

/* Say whether SIGNAL is a `quit' error (or inherits from it).  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn signal_quit_p (error: Lisp_Object) -> bool
{
  {
  let signal: Lisp_Object = if CONSP (error) { XCAR (error) } else { Qnil () };
  let list: Lisp_Object;

  return EQ (signal, Qquit ())
    || (SYMBOLP (signal)
	&& CONSP ({ let l = Fget (signal, Qerror_conditions ()); list = l; l })
	&& !NILP (Fmemq (Qquit (), list)));

  }
}

/* Call the debugger if calling it is currently enabled for CONDITIONS.
   SIG and DATA describe the signal.  There are two ways to pass them:
    = SIG is the error symbol, and DATA is the rest of the data.
    = SIG is nil, and DATA is (SYMBOL . REST-OF-DATA).
      This is for memory-full errors only.  */
unsafe fn maybe_call_debugger (conditions: Lisp_Object, error: Lisp_Object) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  if
      /* Don't try to run the debugger with interrupts blocked.
	 The editing loop would return anyway.  */
      ! input_blocked_p ()
      && NILP ((*Vinhibit_debugger.as_mut_ptr ()))
      /* Does user want to enter debugger for this kind of error?  */
      && (if signal_quit_p (error)
	    { (*debug_on_quit.as_mut_ptr ()) }
	  else
	    { wants_debugger ((*Vdebug_on_error.as_mut_ptr ()), conditions) })
      && ! skip_debugger (conditions, error)
      /* See commentary on definition of
         `internal-when-entered-debugger'.  */
      && (*when_entered_debugger.as_mut_ptr ()) < (*num_nonmacro_input_events.as_mut_ptr ())
    {
      call_debugger (list2 (Qerror (), error));
      return true;
    }

  return false;

  }
}

unsafe fn find_handler_clause (handlers: Lisp_Object, conditions: Lisp_Object)
  -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut h: Lisp_Object;

  /* t is used by handlers for all conditions, set up by C code.  */
  /* error is used similarly, but means print an error message
     and run the debugger if that is enabled.  */
  if !CONSP (handlers)
    { return handlers; }

  h = handlers;
  while CONSP (h)
    {
      let handler: Lisp_Object = XCAR (h);
      if !NILP (Fmemq (handler, conditions))
          /* t is also used as a catch-all by Lisp code.  */
          || EQ (handler, Qt ())
	{ return handlers; }
      h = XCDR (h);
    }

  return Qnil ();

  }
}


/* Format and return a string; called like vprintf.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn vformat_string (m: *const c_char, mut ap: core::ffi::VaList<'_>) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut buf: [c_char; 4000] = [0; 4000];
  let mut size: ptrdiff_t = std::mem::size_of_val (&buf) as ptrdiff_t;
  let size_max: ptrdiff_t = STRING_BYTES_BOUND + 1;
  let mut buffer: *mut c_char = buf.as_mut_ptr ();
  let used: ptrdiff_t;
  let string: Lisp_Object;

  used = evxprintf (&mut buffer, &mut size, buf.as_mut_ptr (), size_max, m, ap);
  string = make_string (buffer, used);
  if buffer != buf.as_mut_ptr ()
    { xfree (buffer as *mut c_void); }

  return string;

  }
}

/* Dump an error message; called like vprintf.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn verror (m: *const c_char, ap: core::ffi::VaList<'_>) -> !
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  xsignal1 (Qerror (), vformat_string (m, ap));

  }
}


/* Dump an error message; called like printf.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn error (m: *const c_char, mut args: ...) -> !
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  verror (m, args);

  }
}

static list_of_t: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());  /* Never-modified constant containing (t).  */

DEFUN! ("eval", Feval, Seval, 1, 2, 0,
       doc: "Evaluate FORM and return its value.\nIf LEXICAL is `t', evaluate using lexical binding by default.\nThis is the recommended value.\n\nIf absent or `nil', use dynamic scoping only.\n\nLEXICAL can also represent an actual lexical environment; see the Info\nnode `(elisp)Eval' for details.",
       (form: Lisp_Object, lexical: Lisp_Object),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();
  specbind (Qinternal_interpreter_environment (),
	    if CONSP (lexical) || NILP (lexical) { lexical } else { (*list_of_t.as_mut_ptr ()) });
  return unbind_to (count, eval_sub (form));
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn grow_specpdl_allocation ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassert! (specpdl_ptr! () == specpdl_end! ());

  let count: specpdl_ref = SPECPDL_INDEX ();
  let max_size: ptrdiff_t = PTRDIFF_MAX - 1000;
  let mut pdlvec: *mut specbinding = (specpdl! ()).offset (-1);
  let size: ptrdiff_t = specpdl_end! ().offset_from (specpdl! ()) as ptrdiff_t;
  let mut pdlvecsize: ptrdiff_t = size + 1;
  eassert! (max_size > size);
  pdlvec = xpalloc (pdlvec as *mut c_void, &mut pdlvecsize, 1, max_size + 1,
                    std::mem::size_of::<specbinding> () as ptrdiff_t) as *mut specbinding;
  specpdl! () = pdlvec.offset (1);
  specpdl_end! () = (specpdl! ()).offset (pdlvecsize - 1);
  specpdl_ptr! () = specpdl_ref_to_ptr (count);

  }
}

/* Eval a sub-expression of the current expression (i.e. in the same
   lexical scope).  */
#[unsafe(no_mangle)]
pub extern "C-unwind" fn eval_sub (form: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut form: Lisp_Object = form;   // C assigns to its parameter
  if SYMBOLP (form)
    {
      /* Look up its binding in the lexical environment.
	 We do not pay attention to the declared_special flag here, since we
	 already did that when let-binding the variable.  */
      let lex_binding: Lisp_Object
	= assq_lexenv (form, (*Vinternal_interpreter_environment.as_mut_ptr ()));
      return if !NILP (lex_binding) { XCDR (lex_binding) } else { Fsymbol_value (form) };
    }

  if !CONSP (form)
    { return form; }

  maybe_quit ();

  lisp_eval_depth! () += 1;
  if lisp_eval_depth! () as EMACS_INT > (*max_lisp_eval_depth.as_mut_ptr ())
    {
      if (*max_lisp_eval_depth.as_mut_ptr ()) < 100
	{ (*max_lisp_eval_depth.as_mut_ptr ()) = 100; }
      if lisp_eval_depth! () as EMACS_INT > (*max_lisp_eval_depth.as_mut_ptr ())
	{ xsignal1 (Qexcessive_lisp_nesting (), make_fixnum (lisp_eval_depth! () as EMACS_INT)); }
    }

  let original_fun: Lisp_Object = XCAR (form);
  let mut original_args: Lisp_Object = XCDR (form);
  CHECK_LIST (original_args);

  /* This also protects them from gc.  */
  let count: specpdl_ref = {
    let ct = (*current_thread.as_mut_ptr ());
    let pdl: *mut specbinding = (*ct).m_specpdl_ptr;
    let c = wrap_specpdl_ref (pdl as isize - ((*ct).m_specpdl) as isize);
    (*pdl).bt.kind = SPECPDL_BACKTRACE as u8;
    (*pdl).bt.debug_on_exit = 0;
    (*pdl).bt.function = original_fun;
    (*pdl).bt.args = &raw mut original_args;
    (*pdl).bt.nargs = UNEVALLED;
    let next = pdl.add (1);
    (*ct).m_specpdl_ptr = next;
    if next == (*ct).m_specpdl_end
      { grow_specpdl_allocation (); }
    c
  };

  maybe_gc ();

  if (*debug_on_next_call.as_mut_ptr ())
    { do_debug_on_call (Qt (), count); }

  let mut fun: Lisp_Object;
  let mut val: Lisp_Object;
  let mut funcar: Lisp_Object;
  /* Declare here, as this array may be accessed by call_debugger near
     the end of this function.  See Bug#21245.  */
  let mut argvals: [Lisp_Object; 8] = [Qnil (); 8];
  let argvals_ptr: *mut Lisp_Object = argvals.as_mut_ptr ();
  /* The stack overflow detection probably isn't worth the effort any more
     but this may be the least bad spot to feed it.  */
  (*(*current_thread.as_mut_ptr ())).stack_top = argvals_ptr as *const c_void;

  'retry: loop
    {
      /* Optimize for no indirection.  */
      fun = original_fun;
      if !SYMBOLP (fun)
	{ fun = Ffunction (list1 (fun)); }
      else if !NILP (fun)
	{
	  fun = (*XSYMBOL (fun)).u.s.function;
	  if SYMBOLP (fun)
	    { fun = indirect_function (fun); }
	}

      if SUBRP (fun) && !NATIVE_COMP_FUNCTION_DYNP (fun)
	{
	  let mut args_left: Lisp_Object = original_args;
	  let numargs: ptrdiff_t = fast_list_length (args_left);

	  if numargs < (*XSUBR (fun)).min_args as ptrdiff_t
	      || ((*XSUBR (fun)).max_args >= 0
		  && ((*XSUBR (fun)).max_args as ptrdiff_t) < numargs)
	    { xsignal2 (Qwrong_number_of_arguments (), original_fun,
			make_fixnum (numargs)); }

	  else if ((*XSUBR (fun)).max_args as isize) == UNEVALLED
	    { val = (((*XSUBR (fun)).function.aUNEVALLED).unwrap ()) (args_left); }
	  else if ((*XSUBR (fun)).max_args as isize) == MANY
		   || ((*XSUBR (fun)).max_args as ptrdiff_t) > 8

	    {
	      /* Pass a vector of evaluated arguments.  */
	      if numargs == 0
		{
		  set_backtrace_args (specpdl_ref_to_ptr (count), argvals_ptr, 0);
		  val = (((*XSUBR (fun)).function.aMANY).unwrap ()) (0, argvals_ptr);
		}
	      else if numargs == 1
		{
		  let a0: Lisp_Object = eval_sub (XCAR (args_left));
		  *argvals_ptr.offset (0) = a0;
		  set_backtrace_args (specpdl_ref_to_ptr (count), argvals_ptr, 1);
		  val = (((*XSUBR (fun)).function.aMANY).unwrap ()) (1, argvals_ptr);
		}
	      else if numargs <= 8
		{
		  {
		    let mut argvals_span: Option<crate::roots::ScopedRawSpan> = None;
		    let mut has_heap_arg: bool = false;
		    let mut argnum: ptrdiff_t = 0;
		    while CONSP (args_left) && argnum < numargs
		      {
			let arg: Lisp_Object = XCAR (args_left);
			args_left = XCDR (args_left);
			if CONSP (arg) && has_heap_arg && argvals_span.is_none ()
			  { argvals_span = Some (crate::roots::ScopedRawSpan::new (argvals_ptr, argnum as usize)); }
			let val_arg: Lisp_Object = eval_sub (arg);
			*argvals_ptr.offset (argnum) = val_arg;
			argnum += 1;
			if let Some (span) = argvals_span.as_mut ()
			  { span.set_len (argnum as usize); }
			else if !FIXNUMP (val_arg) && !NILP (val_arg)
			  { has_heap_arg = true; }
		      }
		    set_backtrace_args (specpdl_ref_to_ptr (count), argvals_ptr, argnum);
		  }
		  val = (((*XSUBR (fun)).function.aMANY).unwrap ()) (numargs, argvals_ptr);
		}
	      else
		{
		  let mut vals: *mut Lisp_Object;
		  let mut argnum: ptrdiff_t = 0;
		  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

		  vals = std::ptr::null_mut ();
		  SAFE_ALLOCA_LISP! (sa_avail, vals, numargs);
		  let mut vals_span = crate::roots::ScopedRawSpan::new (vals, 0);

		  while CONSP (args_left) && argnum < numargs
		    {
		      let arg: Lisp_Object = XCAR (args_left);
		      args_left = XCDR (args_left);
		      *vals.offset (argnum) = eval_sub (arg);
		      argnum += 1;
		      vals_span.set_len (argnum as usize);
		    }

		  set_backtrace_args (specpdl_ref_to_ptr (count), vals, argnum);

		  val = (((*XSUBR (fun)).function.aMANY).unwrap ()) (argnum, vals);
		  #[cfg (feature = "have_native_comp")]
		  crate::nonlocal::check_and_propagate_comp_native_exit ();

		  lisp_eval_depth! () -= 1;
		  /* Do the debug-on-exit now, while VALS still exists.  */
		  if backtrace_debug_on_exit (specpdl_ref_to_ptr (count))
		    { val = call_debugger (list2 (Qexit (), val)); }
		  SAFE_FREE! (sa_count, sa_mark);
		  specpdl_ptr! () = (specpdl_ptr! ()).offset (-1);
		  return val;
		}
	    }
	  else
	    {
	      let maxargs: c_int = (*XSUBR (fun)).max_args as c_int;
	      if numargs == 0
		{
		  set_backtrace_args (specpdl_ref_to_ptr (count), argvals_ptr, 0);
		}
	      else if numargs == 1
		{
		  let a0: Lisp_Object = eval_sub (XCAR (args_left));
		  *argvals_ptr.offset (0) = a0;
		  set_backtrace_args (specpdl_ref_to_ptr (count), argvals_ptr, 1);
		}
	      else
		{
		  let mut argvals_span: Option<crate::roots::ScopedRawSpan> = None;
		  let mut has_heap_arg: bool = false;
		  let mut i: c_int = 0;
		  while CONSP (args_left) && (i as ptrdiff_t) < numargs
		    {
		      let arg: Lisp_Object = XCAR (args_left);
		      args_left = XCDR (args_left);
		      if CONSP (arg) && has_heap_arg && argvals_span.is_none ()
			{ argvals_span = Some (crate::roots::ScopedRawSpan::new (argvals_ptr, i as usize)); }
		      let val_arg: Lisp_Object = eval_sub (arg);
		      *argvals_ptr.offset (i as isize) = val_arg;
		      i += 1;
		      if let Some (span) = argvals_span.as_mut ()
			{ span.set_len (i as usize); }
		      else if !FIXNUMP (val_arg) && !NILP (val_arg)
			{ has_heap_arg = true; }
		    }
		  set_backtrace_args (specpdl_ref_to_ptr (count), argvals_ptr, numargs);
		}

	      match maxargs
		{
		0 =>
		  {
		    val = (((*XSUBR (fun)).function.a0).unwrap ()) ();
		  }
		1 =>
		  {
		    val = (((*XSUBR (fun)).function.a1).unwrap ()) (*argvals_ptr.offset (0));
		  }
		2 =>
		  {
		    val = (((*XSUBR (fun)).function.a2).unwrap ())
			  (*argvals_ptr.offset (0), *argvals_ptr.offset (1));
		  }
		3 =>
		  {
		    val = (((*XSUBR (fun)).function.a3).unwrap ())
			  (*argvals_ptr.offset (0), *argvals_ptr.offset (1),
			   *argvals_ptr.offset (2));
		  }
		4 =>
		  {
		    val = (((*XSUBR (fun)).function.a4).unwrap ())
			  (*argvals_ptr.offset (0), *argvals_ptr.offset (1),
			   *argvals_ptr.offset (2), *argvals_ptr.offset (3));
		  }
		5 =>
		  {
		    val = (((*XSUBR (fun)).function.a5).unwrap ())
			  (*argvals_ptr.offset (0), *argvals_ptr.offset (1),
			   *argvals_ptr.offset (2), *argvals_ptr.offset (3),
			   *argvals_ptr.offset (4));
		  }
		6 =>
		  {
		    val = (((*XSUBR (fun)).function.a6).unwrap ())
			  (*argvals_ptr.offset (0), *argvals_ptr.offset (1),
			   *argvals_ptr.offset (2), *argvals_ptr.offset (3),
			   *argvals_ptr.offset (4), *argvals_ptr.offset (5));
		  }
		7 =>
		  {
		    val = (((*XSUBR (fun)).function.a7).unwrap ())
			  (*argvals_ptr.offset (0), *argvals_ptr.offset (1),
			   *argvals_ptr.offset (2), *argvals_ptr.offset (3),
			   *argvals_ptr.offset (4), *argvals_ptr.offset (5),
			   *argvals_ptr.offset (6));
		  }

		8 =>
		  {
		    val = (((*XSUBR (fun)).function.a8).unwrap ())
			  (*argvals_ptr.offset (0), *argvals_ptr.offset (1),
			   *argvals_ptr.offset (2), *argvals_ptr.offset (3),
			   *argvals_ptr.offset (4), *argvals_ptr.offset (5),
			   *argvals_ptr.offset (6), *argvals_ptr.offset (7));
		  }

		_ =>
		  {
		    /* Someone has created a subr that takes more arguments than
		       is supported by this code.  We need to either rewrite the
		       subr to use a different argument protocol, or add more
		       cases to this switch.  */
		    emacs_abort ();
		  }
		}
	    }
	  #[cfg (feature = "have_native_comp")]
	  crate::nonlocal::check_and_propagate_comp_native_exit ();
	}
      else if CLOSUREP (fun)
	       || NATIVE_COMP_FUNCTION_DYNP (fun)
	       || MODULE_FUNCTIONP (fun)
	{ return apply_lambda (fun, original_args, count); }
      else
	{
	  if NILP (fun)
	    { xsignal1 (Qvoid_function (), original_fun); }
	  if !CONSP (fun)
	    { xsignal1 (Qinvalid_function (), original_fun); }
	  funcar = XCAR (fun);
	  if !SYMBOLP (funcar)
	    { xsignal1 (Qinvalid_function (), original_fun); }
	  if EQ (funcar, Qautoload ())
	    {
	      Fautoload_do_load (fun, original_fun, Qnil ());
	      continue 'retry;
	    }
	  if EQ (funcar, Qmacro ())
	    {
	      let count1: specpdl_ref = SPECPDL_INDEX ();
	      let mut exp: Lisp_Object;
	      /* Bind lexical-binding during expansion of the macro, so the
		 macro can know reliably if the code it outputs will be
		 interpreted using lexical-binding or not.  */
	      specbind (Qlexical_binding (),
			if NILP ((*Vinternal_interpreter_environment.as_mut_ptr ())) { Qnil () } else { Qt () });

	      /* Make the macro aware of any defvar declarations in scope. */
	      let mut dynvars: Lisp_Object = (*Vmacroexp__dynvars.as_mut_ptr ());
	      let mut p: Lisp_Object = (*Vinternal_interpreter_environment.as_mut_ptr ());
	      while !NILP (p)
		{
		  let e: Lisp_Object = XCAR (p);
		  if SYMBOLP (e)
		    { dynvars = Fcons (e, dynvars); }
		  p = XCDR (p);
		}
	      if !EQ (dynvars, (*Vmacroexp__dynvars.as_mut_ptr ()))
		{ specbind (Qmacroexp__dynvars (), dynvars); }

	      exp = apply1 (Fcdr (fun), original_args);
	      exp = unbind_to (count1, exp);
	      val = eval_sub (exp);
	    }
	  else if EQ (funcar, Qlambda ())
	    { return apply_lambda (fun, original_args, count); }
	  else
	    { xsignal1 (Qinvalid_function (), original_fun); }
	}
      break;
    }

  lisp_eval_depth! () -= 1;
  if backtrace_debug_on_exit (specpdl_ref_to_ptr (count))
    { val = call_debugger (list2 (Qexit (), val)); }
  specpdl_ptr! () = (specpdl_ptr! ()).offset (-1);

  return val;

  }
}

#[inline(never)]
unsafe fn apply_spread_slow (nargs: isize, args: *mut Lisp_Object, mut fun: Lisp_Object,
			      mut spread_arg: Lisp_Object, numargs: ptrdiff_t) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut i: ptrdiff_t;
  let mut funcall_nargs: ptrdiff_t;
  let mut funcall_args: *mut Lisp_Object;

  /* Optimize for no indirection.  */
  if SYMBOLP (fun) && !NILP (fun)
    {
      fun = (*XSYMBOL (fun)).u.s.function;
      if SYMBOLP (fun)
	{
	  fun = indirect_function (fun);
	  if NILP (fun)
	    /* Let funcall get the error.  */
	    { fun = *args.offset (0); }
	}
    }

  let mut stack_buf: [Lisp_Object; 16] = [Qnil (); 16];
  if SUBRP (fun) && ((*XSUBR (fun)).max_args as ptrdiff_t) > numargs
      /* Don't hide an error by adding missing arguments.  */
      && numargs >= (*XSUBR (fun)).min_args as ptrdiff_t
    {
      funcall_nargs = 1 + (*XSUBR (fun)).max_args as ptrdiff_t;
      if funcall_nargs <= 16
	{
	  funcall_args = stack_buf.as_mut_ptr ();
	  memcpy (funcall_args as *mut c_void, args as *const c_void,
		  (nargs - 1) as usize * word_size as usize);
	  i = nargs - 1;
	  while !NILP (spread_arg)
	    {
	      *funcall_args.offset (i) = XCAR (spread_arg);
	      i += 1;
	      spread_arg = XCDR (spread_arg);
	    }
	  return Ffuncall (funcall_nargs, funcall_args);
	}
    }
  else if 1 + numargs <= 16
    {
      funcall_nargs = 1 + numargs;
      funcall_args = stack_buf.as_mut_ptr ();
      memcpy (funcall_args as *mut c_void, args as *const c_void,
	      (nargs - 1) as usize * word_size as usize);
      i = nargs - 1;
      while !NILP (spread_arg)
	{
	  *funcall_args.offset (i) = XCAR (spread_arg);
	  i += 1;
	  spread_arg = XCDR (spread_arg);
	}
      return Ffuncall (funcall_nargs, funcall_args);
    }

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  if SUBRP (fun) && ((*XSUBR (fun)).max_args as ptrdiff_t) > numargs
      /* Don't hide an error by adding missing arguments.  */
      && numargs >= (*XSUBR (fun)).min_args as ptrdiff_t
    {
      /* Avoid making funcall cons up a yet another new vector of arguments
	 by explicitly supplying nil's for optional values.  */
      funcall_args = std::ptr::null_mut ();
      SAFE_ALLOCA_LISP! (sa_avail, funcall_args, 1 + (*XSUBR (fun)).max_args as isize);
      memclear (funcall_args.offset (numargs + 1) as *mut c_void,
		(((*XSUBR (fun)).max_args as ptrdiff_t) - numargs) * word_size);
      funcall_nargs = 1 + (*XSUBR (fun)).max_args as ptrdiff_t;
    }
  else
    { /* We add 1 to numargs because funcall_args includes the
	 function itself as well as its arguments.  */
      funcall_args = std::ptr::null_mut ();
      SAFE_ALLOCA_LISP! (sa_avail, funcall_args, 1 + numargs);
      funcall_nargs = 1 + numargs;
    }

  memcpy (funcall_args as *mut c_void, args as *const c_void, nargs as usize * word_size as usize);
  /* Spread the last arg we got.  Its first element goes in
     the slot that it used to occupy, hence this value of I.  */
  i = nargs - 1;
  while !NILP (spread_arg)
    {
      *funcall_args.offset (i) = XCAR (spread_arg);
      i += 1;
      spread_arg = XCDR (spread_arg);
    }

  let retval: Lisp_Object = Ffuncall (funcall_nargs, funcall_args);

  SAFE_FREE! (sa_count, sa_mark);
  return retval;
  }
}

DEFUN! ("apply", Fapply, Sapply, 1, MANY, 0,
       doc: "Call FUNCTION with our remaining args, using our last arg as list of args.\nThen return the value FUNCTION returns.\nWith a single argument, call the argument's first element using the\nother elements as args.\nThus, (apply \\='+ 1 2 \\='(3 4)) returns 10.\nusage: (apply FUNCTION &rest ARGUMENTS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let spread_arg: Lisp_Object = *args.offset (nargs - 1);
  let fun: Lisp_Object = *args.offset (0);

  if NILP (spread_arg)
    { return Ffuncall (max (1, nargs - 1), args); }
  else if CONSP (spread_arg) && NILP (XCDR (spread_arg))
    {
      *args.offset (nargs - 1) = XCAR (spread_arg);
      return Ffuncall (nargs, args);
    }

  let numargs: ptrdiff_t = fast_list_length (spread_arg);
  if numargs == 0
    { return Ffuncall (max (1, nargs - 1), args); }
  else if numargs == 1
    {
      *args.offset (nargs - 1) = XCAR (spread_arg);
      return Ffuncall (nargs, args);
    }

  return apply_spread_slow (nargs, args, fun, spread_arg, numargs + nargs - 2);
});

/* Run hook variables in various ways.  */

unsafe extern "C-unwind" fn funcall_nil (nargs: ptrdiff_t, args: *mut Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  Ffuncall (nargs, args);
  return Qnil ();

  }
}

DEFUN! ("run-hooks", Frun_hooks, Srun_hooks, 0, MANY, 0,
       doc: "Run each hook in HOOKS.\nEach argument should be a symbol, a hook variable.\nThese symbols are processed in the order specified.\nIf a hook symbol has a non-nil value, that value may be a function\nor a list of functions to be called to run the hook.\nIf the value is a function, it is called with no arguments.\nIf it is a list, the elements are called, in order, with no arguments.\n\nMajor modes should not use this function directly to run their mode\nhook; they should use `run-mode-hooks' instead.\n\nDo not use `make-local-variable' to make a hook variable buffer-local.\nInstead, use `add-hook' and specify t for the LOCAL argument.\nusage: (run-hooks &rest HOOKS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut i: ptrdiff_t;

  i = 0;
  while i < nargs
    {
      run_hook (*args.offset (i));
      i += 1;
    }

  return Qnil ();
});

DEFUN! ("run-hook-with-args", Frun_hook_with_args,
       Srun_hook_with_args, 1, MANY, 0,
       doc: "Run HOOK with the specified arguments ARGS.\nHOOK should be a symbol, a hook variable.  The value of HOOK\nmay be nil, a function, or a list of functions.  Call each\nfunction in order with arguments ARGS.  The final return value\nis unspecified.\n\nDo not use `make-local-variable' to make a hook variable buffer-local.\nInstead, use `add-hook' and specify t for the LOCAL argument.\nusage: (run-hook-with-args HOOK &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  return run_hook_with_args (nargs, args, Some (funcall_nil));
});

/* NB this one still documents a specific non-nil return value.
   (As did run-hook-with-args and run-hook-with-args-until-failure
   until they were changed in 24.1.)  */
DEFUN! ("run-hook-with-args-until-success", Frun_hook_with_args_until_success,
       Srun_hook_with_args_until_success, 1, MANY, 0,
       doc: "Run HOOK with the specified arguments ARGS.\nHOOK should be a symbol, a hook variable.  The value of HOOK\nmay be nil, a function, or a list of functions.  Call each\nfunction in order with arguments ARGS, stopping at the first\none that returns non-nil, and return that value.  Otherwise (if\nall functions return nil, or if there are no functions to call),\nreturn nil.\n\nDo not use `make-local-variable' to make a hook variable buffer-local.\nInstead, use `add-hook' and specify t for the LOCAL argument.\nusage: (run-hook-with-args-until-success HOOK &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  return run_hook_with_args (nargs, args, Some (Ffuncall));
});

unsafe extern "C-unwind" fn funcall_not (nargs: ptrdiff_t, args: *mut Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  return if NILP (Ffuncall (nargs, args)) { Qt () } else { Qnil () };

  }
}

DEFUN! ("run-hook-with-args-until-failure", Frun_hook_with_args_until_failure,
       Srun_hook_with_args_until_failure, 1, MANY, 0,
       doc: "Run HOOK with the specified arguments ARGS.\nHOOK should be a symbol, a hook variable.  The value of HOOK\nmay be nil, a function, or a list of functions.  Call each\nfunction in order with arguments ARGS, stopping at the first\none that returns nil, and return nil.  Otherwise (if all functions\nreturn non-nil, or if there are no functions to call), return non-nil\n(do not rely on the precise return value in this case).\n\nDo not use `make-local-variable' to make a hook variable buffer-local.\nInstead, use `add-hook' and specify t for the LOCAL argument.\nusage: (run-hook-with-args-until-failure HOOK &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  return if NILP (run_hook_with_args (nargs, args, Some (funcall_not)))
           { Qt () } else { Qnil () };
});

unsafe extern "C-unwind" fn run_hook_wrapped_funcall (nargs: ptrdiff_t, args: *mut Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let tmp: Lisp_Object = *args.offset (0);
  let ret: Lisp_Object;
  *args.offset (0) = *args.offset (1);
  *args.offset (1) = tmp;
  ret = Ffuncall (nargs, args);
  *args.offset (1) = *args.offset (0);
  *args.offset (0) = tmp;
  return ret;

  }
}

DEFUN! ("run-hook-wrapped", Frun_hook_wrapped, Srun_hook_wrapped, 2, MANY, 0,
       doc: "Run HOOK, passing each function through WRAP-FUNCTION.\nI.e. instead of calling each function FUN directly with arguments ARGS,\nit calls WRAP-FUNCTION with arguments FUN and ARGS.\nAs soon as a call to WRAP-FUNCTION returns non-nil, `run-hook-wrapped'\naborts and returns that value.\nusage: (run-hook-wrapped HOOK WRAP-FUNCTION &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  return run_hook_with_args (nargs, args, Some (run_hook_wrapped_funcall));
});

/* ARGS[0] should be a hook symbol.
   Call each of the functions in the hook value, passing each of them
   as arguments all the rest of ARGS (all NARGS - 1 elements).
   FUNCALL specifies how to call each function on the hook.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn run_hook_with_args (
  nargs: ptrdiff_t, args: *mut Lisp_Object,
  funcall: Option<unsafe extern "C-unwind" fn (ptrdiff_t, *mut Lisp_Object) -> Lisp_Object>)
  -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let sym: Lisp_Object;
  let mut val: Lisp_Object;
  let mut ret: Lisp_Object = Qnil ();

  /* If we are dying or still initializing,
     don't do anything--it would probably crash if we tried.  */
  if NILP ((*Vrun_hooks.as_mut_ptr ()))
    { return Qnil (); }

  sym = *args.offset (0);
  val = find_symbol_value (sym);

  if BASE_EQ (val, Qunbound ()) || NILP (val)
    { return ret; }
  else if !CONSP (val) || FUNCTIONP (val)
    {
      *args.offset (0) = val;
      return (funcall.unwrap ()) (nargs, args);
    }
  else
    {
      let mut global_vals: Lisp_Object = Qnil ();

      while CONSP (val) && NILP (ret)
	{
	  if EQ (XCAR (val), Qt ())
	    {
	      /* t indicates this hook has a local binding;
		 it means to run the global binding too.  */
	      global_vals = Fdefault_value (sym);
	      if NILP (global_vals)
		{ val = XCDR (val); continue; }

	      if !CONSP (global_vals) || EQ (XCAR (global_vals), Qlambda ())
		{
		  *args.offset (0) = global_vals;
		  ret = (funcall.unwrap ()) (nargs, args);
		}
	      else
		{
		  while CONSP (global_vals) && NILP (ret)
		    {
		      *args.offset (0) = XCAR (global_vals);
		      /* In a global value, t should not occur.  If it does, we
			 must ignore it to avoid an endless loop.  */
		      if !EQ (*args.offset (0), Qt ())
			{ ret = (funcall.unwrap ()) (nargs, args); }
		      global_vals = XCDR (global_vals);
		    }
		}
	    }
	  else
	    {
	      *args.offset (0) = XCAR (val);
	      ret = (funcall.unwrap ()) (nargs, args);
	    }
	  val = XCDR (val);
	}

      return ret;
    }

  }
}

/* Run the hook HOOK, giving each function no args.  */

#[unsafe(no_mangle)]
pub extern "C-unwind" fn run_hook (hook: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut hook: Lisp_Object = hook;   // C passes the parameter's address, which the callee writes
  Frun_hook_with_args (1, &raw mut hook);

  }
}

/* Run the hook HOOK, giving each function the two args ARG1 and ARG2.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn run_hook_with_args_2 (hook: Lisp_Object, arg1: Lisp_Object,
                                               arg2: Lisp_Object) -> Lisp_Object
{
  return CALLN! (Frun_hook_with_args, hook, arg1, arg2);
}

/* Apply fn to arg.  */
#[inline(never)]
unsafe fn apply1_spread_slow (fn_: Lisp_Object, arg: Lisp_Object, cdr1: Lisp_Object, cdr2: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut buf: [Lisp_Object; 16] = [Qnil (); 16];
  buf[0] = fn_;
  buf[1] = XCAR (arg);
  buf[2] = XCAR (cdr1);
  let mut n: ptrdiff_t = 3;
  let mut s: Lisp_Object = cdr2;
  while CONSP (s) && n < 16
    {
      buf[n as usize] = XCAR (s);
      n += 1;
      s = XCDR (s);
    }
  if NILP (s)
    { return Ffuncall (n, buf.as_mut_ptr ()); }
  return CALLN! (Fapply, fn_, arg);
  }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn apply1 (fn_: Lisp_Object, arg: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  if NILP (arg)
    { return calln! (fn_); }
  if CONSP (arg)
    {
      let cdr1: Lisp_Object = XCDR (arg);
      if NILP (cdr1)
	{ return calln! (fn_, XCAR (arg)); }
      if CONSP (cdr1)
	{
	  let cdr2: Lisp_Object = XCDR (cdr1);
	  if NILP (cdr2)
	    { return calln! (fn_, XCAR (arg), XCAR (cdr1)); }
	  return apply1_spread_slow (fn_, arg, cdr1, cdr2);
	}
    }
  return CALLN! (Fapply, fn_, arg);

  }
}

DEFUN! ("functionp", Ffunctionp, Sfunctionp, 1, 1, 0,
       doc: "Return t if OBJECT is a function.\n\nAn object is a function if it is callable via `funcall'; this includes\nsymbols with function bindings, but excludes macros and special forms.\n\nOrdinarily return nil if OBJECT is not a function, although t might be\nreturned in rare cases.",
       (object: Lisp_Object),
       {
  if FUNCTIONP (object)
    { return Qt (); }
  return Qnil ();
});

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn FUNCTIONP (object: Lisp_Object) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut object: Lisp_Object = object;   // C assigns to its parameter
  if SYMBOLP (object) && !NILP (Ffboundp (object))
    {
      object = Findirect_function (object, Qt ());

      if CONSP (object) && EQ (XCAR (object), Qautoload ())
	{
	  /* Autoloaded symbols are functions, except if they load
	     macros or keymaps.  */
	  let mut i: c_int = 0;
	  while i < 4 && CONSP (object)
	    {
	      object = XCDR (object);
	      i += 1;
	    }

	  return ! (CONSP (object) && !NILP (XCAR (object)));
	}
    }

  if SUBRP (object)
    { return ((*XSUBR (object)).max_args as isize) != UNEVALLED; }
  else if CLOSUREP (object) || MODULE_FUNCTIONP (object)
    { return true; }
  else if CONSP (object)
    {
      let car: Lisp_Object = XCAR (object);
      return EQ (car, Qlambda ());
    }
  else
    { return false; }

  }
}

DEFUN! ("debugger-trap", Fdebugger_trap, Sdebugger_trap, 0, 0, "",
       doc: "Stop Emacs and hand over control to GDB.\nThe Emacs source file src/.gdbinit sets a breakpoint in this function.\n\nThis function does nothing.  It is not called by Emacs otherwise, and\nexists so that calling it or invoking it interactively will cause\nGDB to kick in.\n\nFor Lisp debugging see `debug', as well as `edebug', in the manual:\n\"(elisp) Debugging\".",
       (),
       {
  return Qnil ();
});

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn funcall_general (fun: Lisp_Object, numargs: ptrdiff_t,
                                          args: *mut Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut fun: Lisp_Object = fun;   // C's `fun = original_fun` below rebinds it
  let original_fun: Lisp_Object = fun;
  'retry: loop
    {
      if SYMBOLP (fun) && !NILP (fun)
	{
	  fun = (*XSYMBOL (fun)).u.s.function;
	  if SYMBOLP (fun)
	    { fun = indirect_function (fun); }
	}

      if SUBRP (fun) && !NATIVE_COMP_FUNCTION_DYNP (fun)
	{
	  let subr: *mut Lisp_Subr = XSUBR (fun);
	  if numargs == 1 && (*subr).min_args <= 1
	    {
	      if (*subr).max_args == 1
		{
		  let val = (((*subr).function.a1).unwrap_unchecked ()) (*args);
		  #[cfg (feature = "have_native_comp")]
		  crate::nonlocal::check_and_propagate_comp_native_exit ();
		  return val;
		}
	      if (*subr).max_args == 2
		{
		  let val = (((*subr).function.a2).unwrap_unchecked ()) (*args, Qnil ());
		  #[cfg (feature = "have_native_comp")]
		  crate::nonlocal::check_and_propagate_comp_native_exit ();
		  return val;
		}
	    }
	  return funcall_subr (subr, numargs, args);
	}
      else if CLOSUREP (fun)
	{
	  let syms_left: Lisp_Object = AREF (fun, CLOSURE_ARGLIST as isize);
	  if FIXNUMP (syms_left)
	    { return exec_byte_code (fun, XFIXNUM (syms_left), numargs, args); }
	  return funcall_lambda (fun, numargs, args);
	}
      else if NATIVE_COMP_FUNCTION_DYNP (fun)
	       || MODULE_FUNCTIONP (fun)
	{ return funcall_lambda (fun, numargs, args); }
      else
	{
	  if NILP (fun)
	    { xsignal1 (Qvoid_function (), original_fun); }
	  if !CONSP (fun)
	    { xsignal1 (Qinvalid_function (), original_fun); }
	  let funcar: Lisp_Object = XCAR (fun);
	  if !SYMBOLP (funcar)
	    { xsignal1 (Qinvalid_function (), original_fun); }
	  if EQ (funcar, Qlambda ())
	    { return funcall_lambda (fun, numargs, args); }
	  else if EQ (funcar, Qautoload ())
	    {
	      Fautoload_do_load (fun, original_fun, Qnil ());
	      fun = original_fun;
	      continue 'retry;
	    }
	  else
	    { xsignal1 (Qinvalid_function (), original_fun); }
	}
    }

  }
}

DEFUN! ("funcall", Ffuncall, Sfuncall, 1, MANY, 0,
       doc: "Call first argument as a function, passing remaining arguments to it.\nReturn the value that function returns.\nThus, (funcall \\='cons \\='x \\='y) returns (x . y).\nusage: (funcall FUNCTION &rest ARGUMENTS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let count: specpdl_ref;

  maybe_quit ();

  lisp_eval_depth! () += 1;
  if lisp_eval_depth! () as EMACS_INT > (*max_lisp_eval_depth.as_mut_ptr ())
    {
      if (*max_lisp_eval_depth.as_mut_ptr ()) < 100
	{ (*max_lisp_eval_depth.as_mut_ptr ()) = 100; }
      if lisp_eval_depth! () as EMACS_INT > (*max_lisp_eval_depth.as_mut_ptr ())
	{ xsignal1 (Qexcessive_lisp_nesting (), make_fixnum (lisp_eval_depth! () as EMACS_INT)); }
    }

  count = {
    let ct = (*current_thread.as_mut_ptr ());
    let pdl: *mut specbinding = (*ct).m_specpdl_ptr;
    let c = wrap_specpdl_ref (pdl as isize - ((*ct).m_specpdl) as isize);
    (*pdl).bt.kind = SPECPDL_BACKTRACE as u8;
    (*pdl).bt.debug_on_exit = 0;
    (*pdl).bt.function = *args.offset (0);
    (*pdl).bt.args = args.offset (1);
    (*pdl).bt.nargs = nargs - 1;
    let next = pdl.add (1);
    (*ct).m_specpdl_ptr = next;
    if next == (*ct).m_specpdl_end
      { grow_specpdl_allocation (); }
    c
  };

  maybe_gc ();

  if (*debug_on_next_call.as_mut_ptr ())
    { do_debug_on_call (Qlambda (), count); }

  let mut val: Lisp_Object = funcall_general (*args.offset (0), nargs - 1, args.offset (1));

  lisp_eval_depth! () -= 1;
  if backtrace_debug_on_exit (specpdl_ref_to_ptr (count))
    { val = call_debugger (list2 (Qexit (), val)); }
  specpdl_ptr! () = (specpdl_ptr! ()).offset (-1);
  return val;
});

unsafe extern "C-unwind" fn safe_eval_handler (arg: Lisp_Object, nargs: ptrdiff_t,
                                        args: *mut Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  add_to_log (c"Error muted by safe_call: %S signaled %S".as_ptr (),
	      Flist (nargs, args), arg);
  return Qnil ();

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn safe_funcall (nargs: ptrdiff_t, args: *mut Lisp_Object)
  -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let count: specpdl_ref = SPECPDL_INDEX ();
  /* FIXME: This function started its life in 'xdisp.c' for use internally
     by the redisplay.  So it was important to inhibit redisplay.
     Not clear if we still need this 'specbind' now that 'xdisp.c' has its
     own version of this code.  */
  specbind (Qinhibit_redisplay (), Qt ());
  /* Use Qt to ensure debugger does not run.  */
  let val: Lisp_Object = internal_condition_case_n (Some (Ffuncall), nargs, args, Qt (),
				                    Some (safe_eval_handler));
  return unbind_to (count, val);

  }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn safe_eval (sexp: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* C 3245-3248 is `return safe_calln (Qeval, sexp, Qt);`, and `safe_calln` is
     `CALLMANY (safe_funcall, ((Lisp_Object []) {__VA_ARGS__}))` (lisp.h 4846-4847): the call
     goes to *this* unit's `safe_funcall`, which catches errors and binds
     `inhibit-redisplay`.  The mirror's `safe_calln!` instead expands to `calln!` (`Ffuncall`)
     plus a `maybe_quit ()` C does not make at that point (src/include/lisp.rs
     3149-3154), so this unit spells C's own expansion out rather than becoming unfaithful
     through the macro; the defect is reported in docs/ports/eval.md for the integrator, who
     owns the mirror.  */
  let mut args: [Lisp_Object; 3] = [Qeval (), sexp, Qt ()];
  return safe_funcall (3, args.as_mut_ptr ());

  }
}

/* Apply a C subroutine SUBR to the NUMARGS evaluated arguments in ARG_VECTOR
   and return the result of evaluation.  */

#[inline(never)]
unsafe fn funcall_subr_4_to_8 (subr: *mut Lisp_Subr, numargs: ptrdiff_t, maxargs: ptrdiff_t,
			       args: *mut Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut argbuf: [Lisp_Object; 8] = [Qnil (); 8];
  let a: *mut Lisp_Object = argbuf.as_mut_ptr ();
  eassume! (maxargs <= 8);
  let mut i: ptrdiff_t = 0;
  while i < numargs
    {
      *a.offset (i) = *args.offset (i);
      i += 1;
    }
  match maxargs
    {
    0 => (((*subr).function.a0).unwrap_unchecked ()) (),
    1 => (((*subr).function.a1).unwrap_unchecked ()) (*a.offset (0)),
    2 => (((*subr).function.a2).unwrap_unchecked ()) (*a.offset (0), *a.offset (1)),
    3 => (((*subr).function.a3).unwrap_unchecked ()) (*a.offset (0), *a.offset (1), *a.offset (2)),
    4 => (((*subr).function.a4).unwrap_unchecked ()) (*a.offset (0), *a.offset (1), *a.offset (2), *a.offset (3)),
    5 => (((*subr).function.a5).unwrap_unchecked ()) (*a.offset (0), *a.offset (1), *a.offset (2), *a.offset (3), *a.offset (4)),
    6 => (((*subr).function.a6).unwrap_unchecked ()) (*a.offset (0), *a.offset (1), *a.offset (2), *a.offset (3), *a.offset (4), *a.offset (5)),
    7 => (((*subr).function.a7).unwrap_unchecked ()) (*a.offset (0), *a.offset (1), *a.offset (2), *a.offset (3), *a.offset (4), *a.offset (5), *a.offset (6)),
    8 => (((*subr).function.a8).unwrap_unchecked ()) (*a.offset (0), *a.offset (1), *a.offset (2), *a.offset (3), *a.offset (4), *a.offset (5), *a.offset (6), *a.offset (7)),
    _ => { eassume! (false); Qnil () }
    }
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn funcall_subr (subr: *mut Lisp_Subr, numargs: ptrdiff_t,
                                       args: *mut Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  eassume! (numargs >= 0);
  if numargs >= (*subr).min_args as ptrdiff_t
    {
      /* Conforming call to finite-arity subr.  */
      let maxargs: ptrdiff_t = (*subr).max_args as ptrdiff_t;
      if numargs <= maxargs && maxargs <= 8
	{
	  let val: Lisp_Object = if numargs == maxargs
	    {
	      match maxargs
		{
		0 => (((*subr).function.a0).unwrap_unchecked ()) (),
		1 => (((*subr).function.a1).unwrap_unchecked ()) (*args.offset (0)),
		2 => (((*subr).function.a2).unwrap_unchecked ()) (*args.offset (0), *args.offset (1)),
		3 => (((*subr).function.a3).unwrap_unchecked ()) (*args.offset (0), *args.offset (1), *args.offset (2)),
		4 => (((*subr).function.a4).unwrap_unchecked ()) (*args.offset (0), *args.offset (1), *args.offset (2), *args.offset (3)),
		5 => (((*subr).function.a5).unwrap_unchecked ()) (*args.offset (0), *args.offset (1), *args.offset (2), *args.offset (3), *args.offset (4)),
		6 => (((*subr).function.a6).unwrap_unchecked ()) (*args.offset (0), *args.offset (1), *args.offset (2), *args.offset (3), *args.offset (4), *args.offset (5)),
		7 => (((*subr).function.a7).unwrap_unchecked ()) (*args.offset (0), *args.offset (1), *args.offset (2), *args.offset (3), *args.offset (4), *args.offset (5), *args.offset (6)),
		8 => (((*subr).function.a8).unwrap_unchecked ()) (*args.offset (0), *args.offset (1), *args.offset (2), *args.offset (3), *args.offset (4), *args.offset (5), *args.offset (6), *args.offset (7)),
		_ => { eassume! (false); Qnil () }
		}
	    }
	  else if maxargs == 2
	    {
	      let a0: Lisp_Object = if numargs >= 1 { *args.offset (0) } else { Qnil () };
	      (((*subr).function.a2).unwrap_unchecked ()) (a0, Qnil ())
	    }
	  else if maxargs == 3
	    {
	      let a0: Lisp_Object = if numargs >= 1 { *args.offset (0) } else { Qnil () };
	      let a1: Lisp_Object = if numargs >= 2 { *args.offset (1) } else { Qnil () };
	      (((*subr).function.a3).unwrap_unchecked ()) (a0, a1, Qnil ())
	    }
	  else
	    { funcall_subr_4_to_8 (subr, numargs, maxargs, args) };
	  #[cfg (feature = "have_native_comp")]
	  crate::nonlocal::check_and_propagate_comp_native_exit ();
	  return val;
	}

      /* Call to n-adic subr.  */
      if maxargs == MANY || maxargs > 8
	{
	  let val = (((*subr).function.aMANY).unwrap ()) (numargs, args);
	  #[cfg (feature = "have_native_comp")]
	  crate::nonlocal::check_and_propagate_comp_native_exit ();
	  return val;
	}
    }

  /* Anything else is an error.  */
  let fun: Lisp_Object;
  XSETSUBR! (fun, subr);
  if (*subr).max_args as isize == UNEVALLED
    { xsignal1 (Qinvalid_function (), fun); }
  else
    { xsignal2 (Qwrong_number_of_arguments (), fun, make_fixnum (numargs)); }

  }
}

unsafe fn apply_lambda (fun: Lisp_Object, args: Lisp_Object, count: specpdl_ref)
  -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut arg_vector: *mut Lisp_Object;
  let mut tem: Lisp_Object;
  let numargs: ptrdiff_t = fast_list_length (args);
  if numargs <= 8
    {
      let mut arg_buf: [Lisp_Object; 8] = [Qnil (); 8];
      arg_vector = arg_buf.as_mut_ptr ();
      if numargs == 0
	{
	  set_backtrace_args (specpdl_ref_to_ptr (count), arg_vector, 0);
	}
      else if numargs == 1
	{
	  let a0: Lisp_Object = eval_sub (XCAR (args));
	  *arg_vector.offset (0) = a0;
	  set_backtrace_args (specpdl_ref_to_ptr (count), arg_vector, 1);
	}
      else
	{
	  let mut arg_span: Option<crate::roots::ScopedRawSpan> = None;
	  let mut has_heap_arg: bool = false;
	  let mut args_left: Lisp_Object = args;
	  let mut i: ptrdiff_t = 0;
	  while i < numargs
	    {
	      let arg: Lisp_Object = XCAR (args_left);
	      args_left = XCDR (args_left);
	      if CONSP (arg) && has_heap_arg && arg_span.is_none ()
		{ arg_span = Some (crate::roots::ScopedRawSpan::new (arg_vector, i as usize)); }
	      tem = eval_sub (arg);
	      *arg_vector.offset (i) = tem;
	      i += 1;
	      if let Some (span) = arg_span.as_mut ()
		{ span.set_len (i as usize); }
	      else if !FIXNUMP (tem) && !NILP (tem)
		{ has_heap_arg = true; }
	    }
	  set_backtrace_args (specpdl_ref_to_ptr (count), arg_vector, numargs);
	}
      tem = funcall_lambda (fun, numargs, arg_vector);
      lisp_eval_depth! () -= 1;
      /* Do the debug-on-exit now, while arg_vector still exists.  */
      if backtrace_debug_on_exit (specpdl_ref_to_ptr (count))
	{ tem = call_debugger (list2 (Qexit (), tem)); }
      specpdl_ptr! () = (specpdl_ptr! ()).offset (-1);
      return tem;
    }

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  arg_vector = std::ptr::null_mut ();
  SAFE_ALLOCA_LISP! (sa_avail, arg_vector, numargs);
  let mut arg_span = crate::roots::ScopedRawSpan::new (arg_vector, 0);
  let mut args_left: Lisp_Object = args;

  let mut i: ptrdiff_t = 0;
  while i < numargs
    {
      tem = Fcar (args_left);
      args_left = Fcdr (args_left);
      tem = eval_sub (tem);
      *arg_vector.offset (i) = tem;
      i += 1;
      arg_span.set_len (i as usize);
    }

  set_backtrace_args (specpdl_ref_to_ptr (count), arg_vector, numargs);
  tem = funcall_lambda (fun, numargs, arg_vector);

  lisp_eval_depth! () -= 1;
  /* Do the debug-on-exit now, while arg_vector still exists.  */
  if backtrace_debug_on_exit (specpdl_ref_to_ptr (count))
    { tem = call_debugger (list2 (Qexit (), tem)); }
  SAFE_FREE! (sa_count, sa_mark);
  specpdl_ptr! () = (specpdl_ptr! ()).offset (-1);
  return tem;

  }
}

/* Apply a Lisp function FUN to the NARGS evaluated arguments in ARG_VECTOR
   and return the result of evaluation.
   FUN must be either a lambda-expression, a compiled-code object,
   or a module function.  */

unsafe fn funcall_lambda (fun: Lisp_Object, nargs: ptrdiff_t, arg_vector: *mut Lisp_Object)
  -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* C declares and then assigns these in the branch chain below; Rust cannot use a binding
     before every path assigns it, so the port zeroes them first (PORTING.md 0.1-13's shape
     for "declare, then assign in the branches"); every path C takes assigns both.  */
  let mut syms_left: Lisp_Object = Qnil ();
  let mut lexenv: Lisp_Object = Qnil ();

  if CONSP (fun)
    {
      lexenv = Qnil ();
      syms_left = XCDR (fun);
      if CONSP (syms_left)
	{ syms_left = XCAR (syms_left); }
      else
	{ xsignal1 (Qinvalid_function (), fun); }
    }
  else if CLOSUREP (fun)
    {
      syms_left = AREF (fun, CLOSURE_ARGLIST as isize);
      /* Bytecode objects using lexical binding have an integral
	 ARGLIST slot value: pass the arguments to the byte-code
	 engine directly.  */
      if FIXNUMP (syms_left)
	{ return exec_byte_code (fun, XFIXNUM (syms_left), nargs, arg_vector); }
      /* Otherwise the closure either is interpreted
	 or uses dynamic binding and the ARGLIST slot contains a standard
	 formal argument list whose variables are bound dynamically below.  */
      lexenv = if CONSP (AREF (fun, CLOSURE_CODE as isize))
               { AREF (fun, CLOSURE_CONSTANTS as isize) }
               else
               { Qnil () };
    }
  /* C 3378-3381's `#ifdef HAVE_MODULES` arm and C 3382-3388's `#ifdef HAVE_NATIVE_COMP`
     arm: both are dead in the frozen configuration and carried as `#[cfg (feature = …)]`
     text in C's own `else if` position (PORTING.md P-51; the feature names are the C
     condition's non-frozen side, which never occurs here — `xtask dead-arms` can only
     flip `config.rs` keys).  */
  else if {
      let mut taken: bool = false;
      #[cfg (feature = "have_modules")]
      {
        if MODULE_FUNCTIONP (fun)
          { return funcall_module (fun, nargs, arg_vector); }
      }
      #[cfg (feature = "have_native_comp")]
      {
        if NATIVE_COMP_FUNCTION_DYNP (fun)
          {
            syms_left = (*XSUBR (fun)).lambda_list;
            lexenv = Qnil ();
            taken = true;
          }
      }
      taken
    }
    { }
  else
    { emacs_abort (); }

  let count: specpdl_ref = SPECPDL_INDEX ();
  if nargs == 1 && !NILP (lexenv) && CONSP (syms_left) && NILP (XCDR (syms_left))
     && CLOSUREP (fun) && CONSP (AREF (fun, CLOSURE_CODE as isize))
    {
      maybe_quit ();
      let next: Lisp_Object = maybe_remove_pos_from_symbol (XCAR (syms_left));
      if BARE_SYMBOL_P (next)
	 && !BASE_EQ (next, Qand_rest ())
	 && !BASE_EQ (next, Qand_optional ())
	{
	  specbind (Qinternal_interpreter_environment (),
		    Fcons (Fcons (next, *arg_vector), lexenv));
	  let val: Lisp_Object = Fprogn (AREF (fun, CLOSURE_CODE as isize));
	  return unbind_to (count, val);
	}
    }
  let mut i: ptrdiff_t = 0;
  let mut optional: bool = false;
  let mut rest: bool = false;
  let mut previous_rest: bool = false;
  while CONSP (syms_left)
    {
      maybe_quit ();

      let next: Lisp_Object = maybe_remove_pos_from_symbol (XCAR (syms_left));
      if !BARE_SYMBOL_P (next)
	{ xsignal1 (Qinvalid_function (), fun); }

      if BASE_EQ (next, Qand_rest ())
        {
          if rest || previous_rest
            { xsignal1 (Qinvalid_function (), fun); }
          rest = true;
	  previous_rest = true;
        }
      else if BASE_EQ (next, Qand_optional ())
        {
          if optional || rest || previous_rest
            { xsignal1 (Qinvalid_function (), fun); }
          optional = true;
        }
      else
	{
	  let mut arg: Lisp_Object;
	  if rest
	    {
	      arg = Flist (nargs - i, arg_vector.offset (i));
	      i = nargs;
	    }
	  else if i < nargs
	    { arg = *arg_vector.offset (i); i += 1; }
	  else if !optional
	    { xsignal2 (Qwrong_number_of_arguments (), fun, make_fixnum (nargs)); }
	  else
	    { arg = Qnil (); }

	  /* Bind the argument.  */
	  if !NILP (lexenv)
	    /* Lexically bind NEXT by adding it to the lexenv alist.  */
	    { lexenv = Fcons (Fcons (next, arg), lexenv); }
	  else
	    /* Dynamically bind NEXT.  */
	    { specbind (next, arg); }
	  previous_rest = false;
	}
      syms_left = XCDR (syms_left);
    }

  if !NILP (syms_left) || previous_rest
    { xsignal1 (Qinvalid_function (), fun); }
  else if i < nargs
    { xsignal2 (Qwrong_number_of_arguments (), fun, make_fixnum (nargs)); }

  if !BASE_EQ (lexenv, (*Vinternal_interpreter_environment.as_mut_ptr ()))
    /* Instantiate a new lexical environment.  */
    { specbind (Qinternal_interpreter_environment (), lexenv); }

  let val: Lisp_Object;
  if CONSP (fun)
    { val = Fprogn (XCDR (XCDR (fun))); }
  else if NATIVE_COMP_FUNCTIONP (fun)
    {
      eassert! (NATIVE_COMP_FUNCTION_DYNP (fun));
      /* No need to use funcall_subr as we have zero arguments by
	 construction.  */
      val = (((*XSUBR (fun)).function.a0).unwrap ()) ();
      #[cfg (feature = "have_native_comp")]
      crate::nonlocal::check_and_propagate_comp_native_exit ();
    }
  else
    {
      eassert! (CLOSUREP (fun));
      val = if CONSP (AREF (fun, CLOSURE_CODE as isize))
            /* Interpreted function.  */
            { Fprogn (AREF (fun, CLOSURE_CODE as isize)) }
            /* Dynbound bytecode.  */
            else
            { exec_byte_code (fun, 0, 0, std::ptr::null_mut ()) };
    }

  return unbind_to (count, val);

  }
}

DEFUN! ("func-arity", Ffunc_arity, Sfunc_arity, 1, 1, 0,
       doc: "Return minimum and maximum number of args allowed for FUNCTION.\nFUNCTION must be a function of some kind.\nThe returned value is a cons cell (MIN . MAX).  MIN is the minimum number\nof args.  MAX is the maximum number, or the symbol `many', for a\nfunction with `&rest' args, or `unevalled' for a special form.\n\nNote that this function might return inaccurate results in some cases,\nsuch as with functions defined using `apply-partially', functions\nadvised using `advice-add', and functions that determine their arg\nlist dynamically.",
       (function: Lisp_Object),
       {
  let original: Lisp_Object;
  let mut funcar: Lisp_Object;
  let mut result: Lisp_Object;

  original = function;

  'retry: loop
    {
      /* Optimize for no indirection.  */
      let mut function: Lisp_Object = original;
      if SYMBOLP (function) && !NILP (function)
	{
	  function = (*XSYMBOL (function)).u.s.function;
	  if SYMBOLP (function)
	    { function = indirect_function (function); }
	}

      if CONSP (function) && EQ (XCAR (function), Qmacro ())
	{ function = XCDR (function); }

      if SUBRP (function)
	{ result = Fsubr_arity (function); }
      else if CLOSUREP (function)
	{ result = lambda_arity (function); }
      else
	/* C 3513-3516's `#ifdef HAVE_MODULES` arm is part of *the same else chain*
	   (`else if (MODULE_FUNCTIONP (function)) …`), and C's fallback below is the chain's
	   final `else`.  Writing the guarded arm as a separate statement instead of an `else`
	   made the whole fallback unconditional, so `func-arity` signalled `invalid-function`
	   for every argument - a subr, an interpreted closure, a byte-code object alike (measured
	   2026-10-16: `(func-arity 'car)` -> `(invalid-function car)` where the reference answers
	   `(1 . 1)`), which is what stopped the byte-compiler at its inlined-defsubst arity check
	   in 46 of the 111 files of the emacs-lisp directory.  */
	{
	  #[cfg (feature = "have_modules")]
	  {
        unsafe extern "C" {
        }
        if MODULE_FUNCTIONP (function)
          { result = module_function_arity (XMODULE_FUNCTION (function) as *const c_void); }
        else
          {
            if NILP (function)
              { xsignal1 (Qvoid_function (), original); }
            if !CONSP (function)
              { xsignal1 (Qinvalid_function (), original); }
            funcar = XCAR (function);
            if !SYMBOLP (funcar)
              { xsignal1 (Qinvalid_function (), original); }
            if EQ (funcar, Qlambda ())
              { result = lambda_arity (function); }
            else if EQ (funcar, Qautoload ())
              {
                Fautoload_do_load (function, original, Qnil ());
                continue 'retry;
              }
            else
              { xsignal1 (Qinvalid_function (), original); }
          }
      }
	  #[cfg (not (feature = "have_modules"))]
	  {
	if NILP (function)
	  { xsignal1 (Qvoid_function (), original); }
	if !CONSP (function)
	  { xsignal1 (Qinvalid_function (), original); }
	funcar = XCAR (function);
	if !SYMBOLP (funcar)
	  { xsignal1 (Qinvalid_function (), original); }
	if EQ (funcar, Qlambda ())
	  { result = lambda_arity (function); }
	else if EQ (funcar, Qautoload ())
	  {
	    Fautoload_do_load (function, original, Qnil ());
	    continue 'retry;
	  }
	else
	  { xsignal1 (Qinvalid_function (), original); }
	  }
	}
      break;
    }
  return result;
});

/* FUN must be either a lambda-expression or a compiled-code object.  */
unsafe fn lambda_arity (fun: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut syms_left: Lisp_Object;

  if CONSP (fun)
    {
      syms_left = XCDR (fun);
      if CONSP (syms_left)
	{ syms_left = XCAR (syms_left); }
      else
	{ xsignal1 (Qinvalid_function (), fun); }
    }
  else if CLOSUREP (fun)
    {
      syms_left = AREF (fun, CLOSURE_ARGLIST as isize);
      if FIXNUMP (syms_left)
        { return get_byte_code_arity (syms_left); }
    }
  else
    { emacs_abort (); }

  let mut minargs: EMACS_INT = 0;
  let mut maxargs: EMACS_INT = 0;
  let mut optional: bool = false;
  while CONSP (syms_left)
    {
      let next: Lisp_Object = XCAR (syms_left);
      if !SYMBOLP (next)
	{ xsignal1 (Qinvalid_function (), fun); }

      if EQ (next, Qand_rest ())
	{ return Fcons (make_fixnum (minargs), Qmany ()); }
      else if EQ (next, Qand_optional ())
	{ optional = true; }
      else
	{
          if !optional
            { minargs += 1; }
          maxargs += 1;
        }
      syms_left = XCDR (syms_left);
    }

  if !NILP (syms_left)
    { xsignal1 (Qinvalid_function (), fun); }

  return Fcons (make_fixnum (minargs), make_fixnum (maxargs));

  }
}


/* Return true if SYMBOL's default currently has a let-binding
   which was made in the buffer that is now current.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn let_shadows_buffer_binding_p (symbol: *mut Lisp_Symbol) -> bool
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut p: *mut specbinding;
  let buf: Lisp_Object = Fcurrent_buffer ();

  p = specpdl_ptr! ();
  while p > specpdl! ()
    {
      p = p.offset (-1);
      if (*p).kind > SPECPDL_LET as u8
	{
	  let let_bound_symbol: *mut Lisp_Symbol = XSYMBOL (specpdl_symbol (p));
	  eassert! (((*let_bound_symbol).u.s.bits & SYMBOL_REDIRECT_MASK)
		    >> SYMBOL_REDIRECT_SHIFT != SYMBOL_VARALIAS);
	  if symbol == let_bound_symbol
	      && (*p).kind != SPECPDL_LET_LOCAL as u8 /* bug#62419 */
	      && BASE_EQ (specpdl_where (p), buf)
	    { return true; }
	}
    }

  return false;

  }
}

unsafe fn do_specbind (sym: *mut Lisp_Symbol, bind: *mut specbinding, value: Lisp_Object,
                       bindflag: Set_Internal_Bind)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
    {
    SYMBOL_PLAINVAL =>
      {
	if ((*sym).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK) >> SYMBOL_TRAPPED_WRITE_SHIFT
	   == SYMBOL_UNTRAPPED_WRITE
	  { SET_SYMBOL_VAL (sym, value); }
	else
          { set_internal (specpdl_symbol (bind), value, Qnil (), bindflag); }
      }

    SYMBOL_FORWARDED =>
      {
	if BUFFER_OBJFWDP (SYMBOL_FWD (sym) as *mut Lisp_Fwd)
	    && specpdl_kind (bind) == SPECPDL_LET_DEFAULT
	  {
            set_default_internal (specpdl_symbol (bind), value, bindflag,
				  std::ptr::null_mut ());
	    return;
	  }
        /* C's `FALLTHROUGH' into SYMBOL_LOCALIZED (PORTING.md 2.2): the port's arm
           reaches the same statements through the shared body below.  */
        set_internal (specpdl_symbol (bind), value, Qnil (), bindflag);
      }

    SYMBOL_LOCALIZED =>
      {
	set_internal (specpdl_symbol (bind), value, Qnil (), bindflag);
      }

    _ =>
      { emacs_abort (); }
    }

  }
}

/* `specpdl_ptr' describes which variable is
   let-bound, so it can be properly undone when we unbind_to.
   It can be either a plain SPECPDL_LET or a SPECPDL_LET_LOCAL/DEFAULT.
   - SYMBOL is the variable being bound.  Note that it should not be
     aliased (i.e. when let-binding V1 that's aliased to V2, we want
     to record V2 here).
   - WHERE tells us in which buffer the binding took place.
     This is used for SPECPDL_LET_LOCAL bindings (i.e. bindings to a
     buffer-local variable) as well as for SPECPDL_LET_DEFAULT bindings,
     i.e. bindings to the default value of a variable which can be
     buffer-local.  */

#[unsafe(no_mangle)]
pub extern "C-unwind" fn specbind (symbol: Lisp_Object, value: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* The caller must ensure that the SYMBOL argument is a bare symbol.  */
  let mut sym: *mut Lisp_Symbol = XBARE_SYMBOL (symbol);
  if ((*sym).u.s.bits & (SYMBOL_REDIRECT_MASK | SYMBOL_TRAPPED_WRITE_MASK)) == 0
    {
      let pdl: *mut specbinding = specpdl_ptr! ();
      (*pdl).r#let.kind = SPECPDL_LET as u8;
      (*pdl).r#let.symbol = symbol;
      (*pdl).r#let.old_value = (*sym).u.s.val.value;
      (*pdl).r#let.r#where.kbd = std::ptr::null_mut ();
      (*sym).u.s.val.value = value;
      grow_specpdl ();
      return;
    }
  let mut symbol: Lisp_Object = symbol;   // C's `XSETSYMBOL (symbol, sym)` rebinds it

  'start: loop
    {
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
	{
	SYMBOL_VARALIAS =>
	  {
	    sym = SYMBOL_ALIAS (sym);
	    XSETSYMBOL! (symbol, sym);
	    continue 'start;
	  }
	SYMBOL_PLAINVAL =>
	  {
	    /* The most common case is that of a non-constant symbol with a
	       trivial value.  Make that as fast as we can.  */
	    (*specpdl_ptr! ()).r#let.kind = SPECPDL_LET as u8;
	    (*specpdl_ptr! ()).r#let.symbol = symbol;
	    (*specpdl_ptr! ()).r#let.old_value = SYMBOL_VAL (sym);
	    (*specpdl_ptr! ()).r#let.r#where.kbd = std::ptr::null_mut ();
	  }
	SYMBOL_LOCALIZED | SYMBOL_FORWARDED =>
	  {
	    let ovalue: Lisp_Object = find_symbol_value (symbol);
	    (*specpdl_ptr! ()).r#let.kind = SPECPDL_LET_LOCAL as u8;
	    (*specpdl_ptr! ()).r#let.symbol = symbol;
	    (*specpdl_ptr! ()).r#let.old_value = ovalue;
	    (*specpdl_ptr! ()).r#let.r#where.buf = Fcurrent_buffer ();

	    eassert! (((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
		      != SYMBOL_LOCALIZED
		      || (BASE_EQ ((*(SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value)).r#where, Fcurrent_buffer ())));

	    if ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
	       == SYMBOL_LOCALIZED
	      {
		if !blv_found (SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value)
		  { (*specpdl_ptr! ()).r#let.kind = SPECPDL_LET_DEFAULT as u8; }
	      }
	    else if BUFFER_OBJFWDP (SYMBOL_FWD (sym) as *mut Lisp_Fwd)
	      {
		/* If SYMBOL is a per-buffer variable which doesn't have a
		   buffer-local value here, make the `let' change the global
		   value by changing the value of SYMBOL in all buffers not
		   having their own value.  This is consistent with what
		   happens with other buffer-local variables.  */
		if NILP (Flocal_variable_p (symbol, Qnil ()))
		  { (*specpdl_ptr! ()).r#let.kind = SPECPDL_LET_DEFAULT as u8; }
	      }
	    else if KBOARD_OBJFWDP (SYMBOL_FWD (sym) as *mut Lisp_Fwd)
	      {
		(*specpdl_ptr! ()).r#let.r#where.kbd = kboard_for_bindings ();
		(*specpdl_ptr! ()).r#let.kind = SPECPDL_LET as u8;
	      }
	    else
	      { (*specpdl_ptr! ()).r#let.kind = SPECPDL_LET as u8; }
	  }
	_ =>
	  { emacs_abort (); }
	}
      break;
    }
  grow_specpdl ();
  do_specbind (sym, (specpdl_ptr! ()).offset (-1), value, SET_INTERNAL_BIND);

  }
}

/* Push unwind-protect entries of various types.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_unwind_protect (
  function: Option<unsafe extern "C-unwind" fn (Lisp_Object)>, arg: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  (*specpdl_ptr! ()).unwind.kind = SPECPDL_UNWIND as u8;
  (*specpdl_ptr! ()).unwind.func = function;
  (*specpdl_ptr! ()).unwind.arg = arg;
  (*specpdl_ptr! ()).unwind.eval_depth = lisp_eval_depth! () as EMACS_INT;
  grow_specpdl ();

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_unwind_protect_array (array: *mut Lisp_Object, nelts: ptrdiff_t)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  (*specpdl_ptr! ()).unwind_array.kind = SPECPDL_UNWIND_ARRAY as u8;
  (*specpdl_ptr! ()).unwind_array.array = array;
  (*specpdl_ptr! ()).unwind_array.nelts = nelts;
  grow_specpdl ();

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_unwind_protect_ptr (
  function: Option<unsafe extern "C-unwind" fn (*mut c_void)>, arg: *mut c_void)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  (*specpdl_ptr! ()).unwind_ptr.kind = SPECPDL_UNWIND_PTR as u8;
  (*specpdl_ptr! ()).unwind_ptr.func = function;
  (*specpdl_ptr! ()).unwind_ptr.arg = arg;
  (*specpdl_ptr! ()).unwind_ptr.mark = None;
  grow_specpdl ();

  }
}

/* Like `record_unwind_protect_ptr', but also specifies a function
   for GC-marking Lisp objects only reachable through ARG.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_unwind_protect_ptr_mark (
  function: Option<unsafe extern "C-unwind" fn (*mut c_void)>, arg: *mut c_void,
  mark: Option<unsafe extern "C-unwind" fn (*mut c_void)>)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  (*specpdl_ptr! ()).unwind_ptr.kind = SPECPDL_UNWIND_PTR as u8;
  (*specpdl_ptr! ()).unwind_ptr.func = function;
  (*specpdl_ptr! ()).unwind_ptr.arg = arg;
  (*specpdl_ptr! ()).unwind_ptr.mark = mark;
  grow_specpdl ();

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_unwind_protect_int (
  function: Option<unsafe extern "C-unwind" fn (c_int)>, arg: c_int)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  (*specpdl_ptr! ()).unwind_int.kind = SPECPDL_UNWIND_INT as u8;
  (*specpdl_ptr! ()).unwind_int.func = function;
  (*specpdl_ptr! ()).unwind_int.arg = arg;
  grow_specpdl ();

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_unwind_protect_intmax (
  function: Option<unsafe extern "C-unwind" fn (i64)>, arg: i64)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  (*specpdl_ptr! ()).unwind_intmax.kind = SPECPDL_UNWIND_INTMAX as u8;
  (*specpdl_ptr! ()).unwind_intmax.func = function;
  (*specpdl_ptr! ()).unwind_intmax.arg = arg;
  grow_specpdl ();

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_unwind_protect_excursion ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  (*specpdl_ptr! ()).unwind_excursion.kind = SPECPDL_UNWIND_EXCURSION as u8;
  save_excursion_save (specpdl_ptr! ());
  grow_specpdl ();

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_unwind_protect_void (
  function: Option<unsafe extern "C-unwind" fn ()>)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  (*specpdl_ptr! ()).unwind_void.kind = SPECPDL_UNWIND_VOID as u8;
  (*specpdl_ptr! ()).unwind_void.func = function;
  grow_specpdl ();

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_unwind_protect_module (kind: u8, ptr: *mut c_void)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  (*specpdl_ptr! ()).kind = kind;
  (*specpdl_ptr! ()).unwind_ptr.func = None;
  (*specpdl_ptr! ()).unwind_ptr.arg = ptr;
  (*specpdl_ptr! ()).unwind_ptr.mark = None;
  grow_specpdl ();

  }
}

unsafe fn do_one_unbind (this_binding: *mut specbinding, unwinding: bool,
                         bindflag: Set_Internal_Bind)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut kbdwhere: *mut KBOARD = std::ptr::null_mut ();

  eassert! (unwinding || (*this_binding).kind >= SPECPDL_LET as u8);
  /* C's `switch (this_binding->kind)`: a Rust `match` cannot use the mirror's `u32` constants
     as patterns against the union's `u8` `kind` field, so the arms are the same comparisons
     in the same order (the data.rs `specpdl`-kind shape).  */
  let kind: u8 = (*this_binding).kind;
  if kind == SPECPDL_UNWIND as u8
    {
      lisp_eval_depth! () = (*this_binding).unwind.eval_depth as intmax_t;
      ((*this_binding).unwind.func.unwrap ()) ((*this_binding).unwind.arg);
    }
  else if kind == SPECPDL_UNWIND_ARRAY as u8
    {
      xfree ((*this_binding).unwind_array.array as *mut c_void);
    }
  else if kind == SPECPDL_UNWIND_PTR as u8
    {
      ((*this_binding).unwind_ptr.func.unwrap ()) ((*this_binding).unwind_ptr.arg);
    }
  else if kind == SPECPDL_UNWIND_INT as u8
    {
      ((*this_binding).unwind_int.func.unwrap ()) ((*this_binding).unwind_int.arg);
    }
  else if kind == SPECPDL_UNWIND_INTMAX as u8
    {
      ((*this_binding).unwind_intmax.func.unwrap ()) ((*this_binding).unwind_intmax.arg);
    }
  else if kind == SPECPDL_UNWIND_VOID as u8
    {
      ((*this_binding).unwind_void.func.unwrap ()) ();
    }
  else if kind == SPECPDL_UNWIND_EXCURSION as u8
    {
      save_excursion_restore ((*this_binding).unwind_excursion.marker,
			      (*this_binding).unwind_excursion.window);
    }
  else if kind == SPECPDL_BACKTRACE as u8 || kind == SPECPDL_NOP as u8
    { }
  else if kind == SPECPDL_LET as u8 || kind == SPECPDL_LET_DEFAULT as u8
    {
      if kind == SPECPDL_LET as u8
	{ /* If variable has a trivial value (no forwarding), and isn't
	     trapped, we can just set it.  */
	  let sym: Lisp_Object = specpdl_symbol (this_binding);
	  if SYMBOLP (sym)
	      && ((*XSYMBOL (sym)).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
		 == SYMBOL_PLAINVAL
	    {
	      if ((*XSYMBOL (sym)).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK) >> SYMBOL_TRAPPED_WRITE_SHIFT
		 == SYMBOL_UNTRAPPED_WRITE
		{ SET_SYMBOL_VAL (XSYMBOL (sym), specpdl_old_value (this_binding)); }
	      else
		{ set_internal (sym, specpdl_old_value (this_binding),
				Qnil (), bindflag); }
	      return;
	    }
	  /* Come here only if make_local_foo was used for the first time
	     on this var within this let or the symbol is not a plainval.  */
	  kbdwhere = specpdl_kboard (this_binding);
	}
      /* C's `FALLTHROUGH` into SPECPDL_LET_DEFAULT (PORTING.md 2.2).  */
      set_default_internal (specpdl_symbol (this_binding),
			    specpdl_old_value (this_binding),
			    bindflag, kbdwhere as *mut c_void);
    }
  else if kind == SPECPDL_LET_LOCAL as u8
    {
      let symbol: Lisp_Object = specpdl_symbol (this_binding);
      let r#where: Lisp_Object = specpdl_where (this_binding);
      let old_value: Lisp_Object = specpdl_old_value (this_binding);
      eassert! (BUFFERP (r#where));

      /* If this was a local binding, reset the value in the appropriate
	 buffer, but only if that buffer's binding still exists.  */
      if !NILP (Flocal_variable_p (symbol, r#where))
        { set_internal (symbol, old_value, r#where, bindflag); }
    }
  else
    {
      /* C 3840-3847's `#ifdef HAVE_MODULES` arms (dead in the frozen configuration; P-51).  */
      #[cfg (feature = "have_modules")]
      {
	unsafe extern "C-unwind" {
	  /// C: lisp.h 4908: `void finalize_runtime_unwind (void *);`
	  pub fn finalize_runtime_unwind (ptr: *mut c_void);
	  /// C: lisp.h 4909: `void finalize_environment_unwind (void *);`
	  pub fn finalize_environment_unwind (ptr: *mut c_void);
	}
	if kind == SPECPDL_MODULE_RUNTIME as u8
	  { finalize_runtime_unwind ((*this_binding).unwind_ptr.arg); }
	else
	  { finalize_environment_unwind ((*this_binding).unwind_ptr.arg); }
      }
    }

  }
}

unsafe extern "C-unwind" fn do_nothing ()
{}

/* Push an unwind-protect entry that does nothing, so that
   set_unwind_protect_ptr can overwrite it later.  */

#[unsafe(no_mangle)]
pub extern "C-unwind" fn record_unwind_protect_nothing ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  record_unwind_protect_void (Some (do_nothing));

  }
}

/* Clear the unwind-protect entry COUNT, so that it does nothing.
   It need not be at the top of the stack.  */

#[unsafe(no_mangle)]
pub extern "C-unwind" fn clear_unwind_protect (count: specpdl_ref)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let p: *mut specbinding = specpdl_ref_to_ptr (count);
  (*p).unwind_void.kind = SPECPDL_UNWIND_VOID as u8;
  (*p).unwind_void.func = Some (do_nothing);

  }
}

/* Set the unwind-protect entry COUNT so that it invokes FUNC (ARG).
   It need not be at the top of the stack.  Discard the entry's
   previous value without invoking it.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn set_unwind_protect (count: specpdl_ref,
                                             func: Option<unsafe extern "C-unwind" fn (Lisp_Object)>,
                                             arg: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let p: *mut specbinding = specpdl_ref_to_ptr (count);
  (*p).unwind.kind = SPECPDL_UNWIND as u8;
  (*p).unwind.func = func;
  (*p).unwind.arg = arg;
  (*p).unwind.eval_depth = lisp_eval_depth! () as EMACS_INT;

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn set_unwind_protect_ptr (
  count: specpdl_ref, func: Option<unsafe extern "C-unwind" fn (*mut c_void)>, arg: *mut c_void)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let p: *mut specbinding = specpdl_ref_to_ptr (count);
  (*p).unwind_ptr.kind = SPECPDL_UNWIND_PTR as u8;
  (*p).unwind_ptr.func = func;
  (*p).unwind_ptr.arg = arg;
  (*p).unwind_ptr.mark = None;

  }
}

/* Pop and execute entries from the unwind-protect stack until the
   depth COUNT is reached.  Return VALUE.  */

#[unsafe(no_mangle)]
pub extern "C-unwind" fn unbind_to (count: specpdl_ref, value: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut ct: *mut thread_state = (*current_thread.as_mut_ptr ());
  let mut target_ptr: *mut specbinding = ((*ct).m_specpdl as *mut u8)
    .offset (unwrap_specpdl_ref (count)) as *mut specbinding;
  if (*ct).m_specpdl_ptr == target_ptr
    { return value; }

  let quitf: Lisp_Object = (*Vquit_flag.as_mut_ptr ());
  let mut unbind_roots: Option<crate::nonlocal::ScopedLispRoot> = None;

  (*Vquit_flag.as_mut_ptr ()) = Qnil ();

  while (*ct).m_specpdl_ptr != target_ptr
    {
      /* Copy the binding, and decrement specpdl_ptr, before we do
	 the work to unbind it.  We decrement first
	 so that an error in unbinding won't try to unbind
	 the same entry again, and we copy the binding first
	 in case more bindings are made during some of the code we run.  */

      let p: *mut specbinding = ((*ct).m_specpdl_ptr).offset (-1);
      (*ct).m_specpdl_ptr = p;
      let kind: u8 = (*p).kind;
      if kind == SPECPDL_LET as u8
	{
	  let sym: Lisp_Object = (*p).r#let.symbol;
	  if SYMBOLP (sym)
	    {
	      let s: *mut Lisp_Symbol = XSYMBOL (sym);
	      if ((*s).u.s.bits & (SYMBOL_REDIRECT_MASK | SYMBOL_TRAPPED_WRITE_MASK)) == 0
		{
		  (*s).u.s.val.value = (*p).r#let.old_value;
		  continue;
		}
	    }
	}
      else if kind == SPECPDL_BACKTRACE as u8 || kind == SPECPDL_NOP as u8
	{ continue; }
      else if kind == SPECPDL_UNWIND as u8
	{
	  let depth = (*p).unwind.eval_depth as intmax_t;
	  let func = (*p).unwind.func.unwrap_unchecked ();
	  let arg = (*p).unwind.arg;
	  let roots = unbind_roots.get_or_insert_with (|| {
	    crate::nonlocal::ScopedLispRoot::new (&[value, quitf, Qnil ()])
	  });
	  roots.set (2, arg);
	  (*ct).m_lisp_eval_depth = depth;
	  func (arg);
	  roots.set (2, Qnil ());
	  ct = (*current_thread.as_mut_ptr ());
	  target_ptr = ((*ct).m_specpdl as *mut u8)
	    .offset (unwrap_specpdl_ref (count)) as *mut specbinding;
	  continue;
	}

      unbind_roots.get_or_insert_with (|| {
	crate::nonlocal::ScopedLispRoot::new (&[value, quitf, Qnil ()])
      });
      let mut this_binding: specbinding = *p;

      do_one_unbind (&mut this_binding, true, SET_INTERNAL_UNBIND);
      ct = (*current_thread.as_mut_ptr ());
      target_ptr = ((*ct).m_specpdl as *mut u8)
	.offset (unwrap_specpdl_ref (count)) as *mut specbinding;
    }

  if NILP ((*Vquit_flag.as_mut_ptr ())) && !NILP (quitf)
    { (*Vquit_flag.as_mut_ptr ()) = quitf; }

  return value;

  }
}

DEFUN! ("special-variable-p", Fspecial_variable_p, Sspecial_variable_p, 1, 1, 0,
       doc: "Return non-nil if SYMBOL's global binding has been declared special.\nA special variable is one that will be bound dynamically, even in a\ncontext where binding is lexical by default.",
       (symbol: Lisp_Object),
       {
   CHECK_SYMBOL (symbol);
   return if (*XSYMBOL (symbol)).u.s.bits & SYMBOL_DECLARED_SPECIAL_BIT != 0
          { Qt () } else { Qnil () };
});


unsafe fn get_backtrace_starting_at (base: Lisp_Object) -> *mut specbinding
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut base: Lisp_Object = base;   // C assigns to its parameter
  let mut pdl: *mut specbinding = backtrace_top ();

  if !NILP (base)
    { /* Skip up to `base'.  */
      let mut offset: c_int = 0;
      if CONSP (base) && FIXNUMP (XCAR (base))
        {
          offset = XFIXNUM (XCAR (base)) as c_int;
          base = XCDR (base);
        }
      base = Findirect_function (base, Qt ());
      while backtrace_p (pdl)
             && !EQ (base, Findirect_function ((backtrace_function.unwrap ()) (pdl), Qt ()))
        { pdl = backtrace_next (pdl); }
      /* P-128: C's condition is `offset-- > 0` - the test uses the *old* value, so an
         offset of 0 advances nothing; the port's `offset >= 0` advanced one frame too many,
         which dropped the innermost frame from every backtrace.  */
      while backtrace_p (pdl)
             && { let o: c_int = offset; offset -= 1; o > 0 }
        { pdl = backtrace_next (pdl); }
    }

  return pdl;

  }
}

unsafe fn get_backtrace_frame (nframes: Lisp_Object, base: Lisp_Object) -> *mut specbinding
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut i: EMACS_INT;

  CHECK_FIXNAT (nframes);
  let mut pdl: *mut specbinding = get_backtrace_starting_at (base);

  /* Find the frame requested.  */
  i = XFIXNAT (nframes);
  while i > 0 && backtrace_p (pdl)
    {
      pdl = backtrace_next (pdl);
      i -= 1;
    }

  return pdl;

  }
}

unsafe fn backtrace_frame_apply (function: Lisp_Object, pdl: *mut specbinding) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  if !backtrace_p (pdl)
    { return Qnil (); }

  let mut flags: Lisp_Object = Qnil ();
  if backtrace_debug_on_exit (pdl)
    { flags = list2 (QCdebug_on_exit (), Qt ()); }

  if backtrace_nargs (pdl) == UNEVALLED
    { return calln! (function, Qnil (), (backtrace_function.unwrap ()) (pdl),
                     *((backtrace_args.unwrap ()) (pdl)), flags); }
  else
    {
      let tem: Lisp_Object = Flist (backtrace_nargs (pdl), (backtrace_args.unwrap ()) (pdl));
      return calln! (function, Qt (), (backtrace_function.unwrap ()) (pdl), tem, flags);
    }

  }
}

DEFUN! ("backtrace-debug", Fbacktrace_debug, Sbacktrace_debug, 2, 3, 0,
       doc: "Set the debug-on-exit flag of eval frame LEVEL levels down to FLAG.\nLEVEL and BASE specify the activation frame to use, as in `backtrace-frame'.\nThe debugger is entered when that frame exits, if the flag is non-nil.",
       (level: Lisp_Object, flag: Lisp_Object, base: Lisp_Object),
       {
  CHECK_FIXNUM (level);
  let pdl: *mut specbinding = get_backtrace_frame (level, base);

  if backtrace_p (pdl)
    { set_backtrace_debug_on_exit (pdl, !NILP (flag)); }

  return flag;
});

DEFUN! ("mapbacktrace", Fmapbacktrace, Smapbacktrace, 1, 2, 0,
       doc: "Call FUNCTION for each frame in backtrace.\nIf BASE is non-nil, it should be a function and iteration will start\nfrom its nearest activation frame.\nFUNCTION is called with 4 arguments: EVALD, FUNC, ARGS, and FLAGS.  If\na frame has not evaluated its arguments yet or is a special form,\nEVALD is nil and ARGS is a list of forms.  If a frame has evaluated\nits arguments and called its function already, EVALD is t and ARGS is\na list of values.\nFLAGS is a plist of properties of the current frame: currently, the\nonly supported property is :debug-on-exit.  `mapbacktrace' always\nreturns nil.",
       (function: Lisp_Object, base: Lisp_Object),
       {
  let mut pdl: *mut specbinding = get_backtrace_starting_at (base);

  while backtrace_p (pdl)
    {
      let i: ptrdiff_t = pdl.offset_from (specpdl! ()) as ptrdiff_t;
      backtrace_frame_apply (function, pdl);
      /* Beware! PDL is no longer valid here because FUNCTION might
         have caused grow_specpdl to reallocate pdlvec.  We must use
         the saved index, cf. Bug#27258.  */
      pdl = backtrace_next ((specpdl! ()).offset (i));
    }

  return Qnil ();
});

DEFUN! ("backtrace-frame--internal", Fbacktrace_frame_internal,
       Sbacktrace_frame_internal, 3, 3, NULL,
       doc: "Call FUNCTION on stack frame NFRAMES away from BASE.\nReturn the result of FUNCTION, or nil if no matching frame could be found.",
       (function: Lisp_Object, nframes: Lisp_Object, base: Lisp_Object),
       {
  return backtrace_frame_apply (function, get_backtrace_frame (nframes, base));
});

DEFUN! ("backtrace--frames-from-thread", Fbacktrace_frames_from_thread,
       Sbacktrace_frames_from_thread, 1, 1, NULL,
       doc: "Return the list of backtrace frames from current execution point in THREAD.\nIf a frame has not evaluated the arguments yet (or is a special form),\nthe value of the list element is (nil FUNCTION ARG-FORMS...).\nIf a frame has evaluated its arguments and called its function already,\nthe value of the list element is (t FUNCTION ARG-VALUES...).\nA &rest arg is represented as the tail of the list ARG-VALUES.\nFUNCTION is whatever was supplied as car of evaluated list,\nor a lambda expression for macro calls.",
       (thread: Lisp_Object),
       {
  let tstate: *mut thread_state;
  CHECK_THREAD (thread);
  tstate = XTHREAD (thread);

  let mut pdl: *mut specbinding = backtrace_thread_top (tstate);
  let mut list: Lisp_Object = Qnil ();

  while backtrace_thread_p (tstate, pdl)
    {
      let frame: Lisp_Object;
      if backtrace_nargs (pdl) == UNEVALLED
	{ frame = Fcons (Qnil (),
			 Fcons ((backtrace_function.unwrap ()) (pdl),
				*((backtrace_args.unwrap ()) (pdl)))); }
      else
	{
	  let tem: Lisp_Object = Flist (backtrace_nargs (pdl), (backtrace_args.unwrap ()) (pdl));
	  frame = Fcons (Qt (), Fcons ((backtrace_function.unwrap ()) (pdl), tem));
	}
      list = Fcons (frame, list);
      pdl = backtrace_thread_next (tstate, pdl);
    }
  return Fnreverse (list);
});

/* For backtrace-eval, we want to temporarily unwind the last few elements of
   the specpdl stack, and then rewind them.  We store the pre-unwind values
   directly in the pre-existing specpdl elements (i.e. we swap the current
   value and the old value stored in the specpdl), kind of like the inplace
   pointer-reversal trick.  As it turns out, the rewind does the same as the
   unwind, except it starts from the other end of the specpdl stack, so we use
   the same function for both unwind and rewind.
   This same code is used when switching threads, except in that case
   we unwind/rewind the whole specpdl of the threads.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn specpdl_unrewind (pdl: *mut specbinding, mut distance: c_int,
                                           vars_only: bool)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut tmp: *mut specbinding = pdl;
  let mut step: c_int = -1;
  let mut kbdwhere: *mut KBOARD;

  if distance < 0
    { /* It's a rewind rather than unwind.  */
      tmp = tmp.offset ((distance - 1) as isize);
      step = 1;
      distance = -distance;
    }

  while distance > 0
    {
      tmp = tmp.offset (step as isize);
      kbdwhere = std::ptr::null_mut ();

      let kind: u8 = (*tmp).kind;
      if kind == SPECPDL_UNWIND as u8
	{
	  if vars_only
	    { distance -= 1; continue; }
	  if (*tmp).unwind.func == Some (set_buffer_if_live as unsafe extern "C-unwind" fn (Lisp_Object))
	    {
	      let oldarg: Lisp_Object = (*tmp).unwind.arg;
	      (*tmp).unwind.arg = Fcurrent_buffer ();
	      set_buffer_if_live (oldarg);
	    }
	}
      else if kind == SPECPDL_UNWIND_EXCURSION as u8
	{
	  if vars_only
	    { distance -= 1; continue; }
	  {
	    let marker: Lisp_Object = (*tmp).unwind_excursion.marker;
	    let window: Lisp_Object = (*tmp).unwind_excursion.window;
	    save_excursion_save (tmp);
	    save_excursion_restore (marker, window);
	  }
	}
      else if kind == SPECPDL_LET as u8 || kind == SPECPDL_LET_DEFAULT as u8
	{
	  let mut fell_through: bool = true;
	  if kind == SPECPDL_LET as u8
	    { /* If variable has a trivial value (no forwarding), we can
		 just set it.  No need to check for constant symbols here,
		 since that was already done by specbind.  */
	      let sym: Lisp_Object = specpdl_symbol (tmp);
	      if SYMBOLP (sym)
		  && ((*XSYMBOL (sym)).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
		     == SYMBOL_PLAINVAL
		{
		  let old_value: Lisp_Object = specpdl_old_value (tmp);
		  set_specpdl_old_value (tmp, SYMBOL_VAL (XSYMBOL (sym)));
		  SET_SYMBOL_VAL (XSYMBOL (sym), old_value);
		  fell_through = false;
		}
	      /* Come here only if make_local_foo was used for the first
		 time on this var within this let or the symbol is forwarded.  */
	      if fell_through
	        { kbdwhere = specpdl_kboard (tmp); }
	    }
	  if fell_through
	    {
	      let sym: Lisp_Object = specpdl_symbol (tmp);
	      let old_value: Lisp_Object = specpdl_old_value (tmp);
	      set_specpdl_old_value (tmp, default_value (sym));
	      set_default_internal (sym, old_value, SET_INTERNAL_THREAD_SWITCH,
				    kbdwhere as *mut c_void);
	    }
	}
      else if kind == SPECPDL_LET_LOCAL as u8
	{
	  let symbol: Lisp_Object = specpdl_symbol (tmp);
	  let r#where: Lisp_Object = specpdl_where (tmp);
	  let old_value: Lisp_Object = specpdl_old_value (tmp);
	  eassert! (BUFFERP (r#where));

	  /* If this was a local binding, reset the value in the appropriate
	     buffer, but only if that buffer's binding still exists.  */
	  if !NILP (Flocal_variable_p (symbol, r#where))
	    {
	      set_specpdl_old_value
		(tmp, buffer_local_value (symbol, r#where));
              set_internal (symbol, old_value, r#where,
                            SET_INTERNAL_THREAD_SWITCH);
	    }
	  else
	    /* If the var is not local any more, it can't be undone nor
               redone, so just zap it.
               This is important in case the buffer re-gains a local value
               before we unrewind again, in which case we'd risk applying
               this entry in the wrong direction.  */
	    { (*tmp).kind = SPECPDL_NOP as u8; }
	}
      distance -= 1;
    }

  }
}

unsafe extern "C-unwind" fn backtrace_eval_unrewind (distance: c_int)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  specpdl_unrewind (specpdl_ptr! (), distance, false);

  }
}

DEFUN! ("backtrace-eval", Fbacktrace_eval, Sbacktrace_eval, 2, 3, NULL,
       doc: "Evaluate EXP in the context of some activation frame.\nNFRAMES and BASE specify the activation frame to use, as in `backtrace-frame'.",
       (exp: Lisp_Object, nframes: Lisp_Object, base: Lisp_Object),
       {
  let pdl: *mut specbinding = get_backtrace_frame (nframes, base);
  let count: specpdl_ref = SPECPDL_INDEX ();
  let distance: ptrdiff_t = specpdl_ptr! ().offset_from (pdl) as ptrdiff_t;
  eassert! (distance >= 0);

  if !backtrace_p (pdl)
    { error (c"Activation frame not found!".as_ptr ()); }

  backtrace_eval_unrewind (distance as c_int);
  record_unwind_protect_int (Some (backtrace_eval_unrewind), -distance as c_int);

  /* Use eval_sub rather than Feval since the main motivation behind
     backtrace-eval is to be able to get/set the value of lexical variables
     from the debugger.  */
  return unbind_to (count, eval_sub (exp));
});

DEFUN! ("backtrace--locals", Fbacktrace__locals, Sbacktrace__locals, 1, 2, NULL,
       doc: "Return names and values of local variables of a stack frame.\nNFRAMES and BASE specify the activation frame to use, as in `backtrace-frame'.",
       (nframes: Lisp_Object, base: Lisp_Object),
       {
  let frame: *mut specbinding = get_backtrace_frame (nframes, base);
  let prevframe: *mut specbinding
    = get_backtrace_frame (make_fixnum (XFIXNAT (nframes) - 1), base);
  let distance: ptrdiff_t = specpdl_ptr! ().offset_from (frame) as ptrdiff_t;
  let mut result: Lisp_Object = Qnil ();
  eassert! (distance >= 0);

  if !backtrace_p (prevframe)
    { error (c"Activation frame not found!".as_ptr ()); }
  if !backtrace_p (frame)
    { error (c"Activation frame not found!".as_ptr ()); }

  /* The specpdl entries normally contain the symbol being bound along with its
     `old_value', so it can be restored.  The new value to which it is bound is
     available in one of two places: either in the current value of the
     variable (if it hasn't been rebound yet) or in the `old_value' slot of the
     next specpdl entry for it.
     `backtrace_eval_unrewind' happens to swap the role of `old_value'
     and "new value", so we abuse it here, to fetch the new value.
     It's ugly (we'd rather not modify global data) and a bit inefficient,
     but it does the job for now.  */
  backtrace_eval_unrewind (distance as c_int);

  /* Grab values.  */
  {
    let mut tmp: *mut specbinding = prevframe;
    while tmp > frame
      {
	let kind: u8 = (*tmp).kind;
	if kind == SPECPDL_LET as u8 || kind == SPECPDL_LET_DEFAULT as u8
	    || kind == SPECPDL_LET_LOCAL as u8
	  {
	    let sym: Lisp_Object = specpdl_symbol (tmp);
	    let val: Lisp_Object = specpdl_old_value (tmp);
	    if BASE_EQ (sym, Qinternal_interpreter_environment ())
	      {
		let mut env: Lisp_Object = val;
		while CONSP (env)
		  {
		    let binding: Lisp_Object = XCAR (env);
		    if CONSP (binding)
		      { result = Fcons (Fcons (XCAR (binding),
					       XCDR (binding)),
					result); }
		    env = XCDR (env);
		  }
	      }
	    else
	      { result = Fcons (Fcons (sym, val), result); }
	  }
	tmp = tmp.offset (-1);
      }
  }

  /* Restore values from specpdl to original place.  */
  backtrace_eval_unrewind (-distance as c_int);

  return result;
});


#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mark_specpdl (first: *mut specbinding, ptr: *mut specbinding)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let mut pdl: *mut specbinding = first;
  while pdl != ptr
    {
      /* C's `switch (pdl->kind)`; see do_one_unbind for why the arms are comparisons.  */
      let kind: u8 = (*pdl).kind;
      if kind == SPECPDL_UNWIND as u8
	{
	  mark_object (specpdl_arg (pdl));
	}
      else if kind == SPECPDL_UNWIND_ARRAY as u8
	{
	  mark_objects ((*pdl).unwind_array.array, (*pdl).unwind_array.nelts);
	}
      else if kind == SPECPDL_UNWIND_EXCURSION as u8
	{
	  mark_object ((*pdl).unwind_excursion.marker);
	  mark_object ((*pdl).unwind_excursion.window);
	}
      else if kind == SPECPDL_BACKTRACE as u8
	{
	  let mut nargs: ptrdiff_t = backtrace_nargs (pdl);
	  mark_object ((backtrace_function.unwrap ()) (pdl));
	  if nargs == UNEVALLED
	    { nargs = 1; }
	  mark_objects ((backtrace_args.unwrap ()) (pdl), nargs);
	}
      else if kind == SPECPDL_LET_DEFAULT as u8 || kind == SPECPDL_LET_LOCAL as u8
	{
	  mark_object (specpdl_where (pdl));
	  /* C's `FALLTHROUGH` into SPECPDL_LET (PORTING.md 2.2).  */
	  mark_object (specpdl_symbol (pdl));
	  mark_object (specpdl_old_value (pdl));
	}
      else if kind == SPECPDL_LET as u8
	{
	  mark_object (specpdl_symbol (pdl));
	  mark_object (specpdl_old_value (pdl));
	}
      else if kind == SPECPDL_UNWIND_PTR as u8
	{
	  if !(*pdl).unwind_ptr.mark.is_none ()
	    { ((*pdl).unwind_ptr.mark.unwrap ()) ((*pdl).unwind_ptr.arg); }
	}
      else if kind == SPECPDL_UNWIND_INT as u8
	      || kind == SPECPDL_UNWIND_INTMAX as u8
              || kind == SPECPDL_UNWIND_VOID as u8
	      || kind == SPECPDL_NOP as u8
	{ }
      else
	{
	  /* C 4364-4370's `#ifdef HAVE_MODULES` arms, then C's explicit
	     `default: emacs_abort ()`.  While other loops that scan the specpdl use
	     "default: break;" for simplicity, here we explicitly list all cases and abort
	     if we find an unexpected value, as a sanity check.  */
	  #[cfg (feature = "have_modules")]
	  {
	    unsafe extern "C-unwind" {
	      /// C: lisp.h 4907: `void mark_module_environment (void *);`
	      pub fn mark_module_environment (ptr: *mut c_void);
	    }
	    if kind == SPECPDL_MODULE_ENVIRONMENT as u8
	      { mark_module_environment ((*pdl).unwind_ptr.arg); }
	    else if kind != SPECPDL_MODULE_RUNTIME as u8
	      { emacs_abort (); }
	  }
	  #[cfg (not (feature = "have_modules"))]
	  { emacs_abort (); }
	}
      pdl = pdl.offset (1);
    }

  }
}

/* Fill ARRAY of size SIZE with backtrace entries, most recent call first.
   Truncate the backtrace if longer than SIZE; pad with nil if shorter.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn get_backtrace (array: *mut Lisp_Object, size: ptrdiff_t)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* Copy the backtrace contents into working memory.  */
  let mut pdl: *mut specbinding = backtrace_top ();
  let mut i: ptrdiff_t = 0;
  while i < size && backtrace_p (pdl)
    {
      *array.offset (i) = (backtrace_function.unwrap ()) (pdl);
      i += 1;
      pdl = backtrace_next (pdl);
    }
  while i < size
    {
      *array.offset (i) = Qnil ();
      i += 1;
    }

  }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn backtrace_top_function () -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let pdl: *mut specbinding = backtrace_top ();
  return if backtrace_p (pdl) { (backtrace_function.unwrap ()) (pdl) } else { Qnil () };

  }
}

/* The `V*`/`defvar`-owned storages of this unit (docs/status.md 3.29): C's storage is
   `globals.f_<name>` and the file whose `syms_of_*` registers it owns it, so the port
   defines each object here and the definition shadows the `globals.rs` declaration
   (PORTING.md P-21/P-39).  `tools/check-globals.py` is the gate.  */
#[unsafe(no_mangle)]
pub static max_lisp_eval_depth: VmGlobal<EMACS_INT> = VmGlobal::<EMACS_INT>::new (0);
#[unsafe(no_mangle)]
pub static lisp_eval_depth_reserve: VmGlobal<EMACS_INT> = VmGlobal::<EMACS_INT>::new (0);
#[unsafe(no_mangle)]
pub static Vquit_flag: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vinhibit_quit: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vinhibit_debugger: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vdebug_on_error: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vdebug_ignored_errors: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static debug_on_quit: VmGlobal<bool> = VmGlobal::<bool>::new (false);
#[unsafe(no_mangle)]
pub static debug_on_next_call: VmGlobal<bool> = VmGlobal::<bool>::new (false);
#[unsafe(no_mangle)]
pub static backtrace_on_redisplay_error: VmGlobal<bool> = VmGlobal::<bool>::new (false);
#[unsafe(no_mangle)]
pub static debugger_may_continue: VmGlobal<bool> = VmGlobal::<bool>::new (false);
#[unsafe(no_mangle)]
pub static debugger_stack_frame_as_list: VmGlobal<bool> = VmGlobal::<bool>::new (false);
#[unsafe(no_mangle)]
pub static Vdebugger: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vsignal_hook_function: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vdebug_on_signal: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static backtrace_on_error_noninteractive: VmGlobal<bool> = VmGlobal::<bool>::new (false);
#[unsafe(no_mangle)]
pub static when_entered_debugger: VmGlobal<EMACS_INT> = VmGlobal::<EMACS_INT>::new (0);
#[unsafe(no_mangle)]
pub static Vinternal_interpreter_environment: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vinternal_make_interpreted_closure_function: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_eval ()
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  DEFVAR_INT! (c"max-lisp-eval-depth", max_lisp_eval_depth,
	      "Limit on depth in `eval', `apply' and `funcall' before error.\n\nThis limit serves to catch infinite recursions for you before they cause\nactual stack overflow in C, which would be fatal for Emacs.\nYou can safely make it considerably larger than its default value,\nif that proves inconveniently small.  However, if you increase it too far,\nEmacs could overflow the real C stack, and crash.");
  (*max_lisp_eval_depth.as_mut_ptr ()) = 1600;

  DEFVAR_INT! (c"lisp-eval-depth-reserve", lisp_eval_depth_reserve,
	      "Extra depth that can be allocated to handle errors.\nThis is the max depth that the system will add to `max-lisp-eval-depth'\nwhen calling debuggers or `handler-bind' handlers.");
  (*lisp_eval_depth_reserve.as_mut_ptr ()) = 200;

  DEFVAR_LISP! (c"quit-flag", Vquit_flag,
	       "Non-nil causes `eval' to abort, unless `inhibit-quit' is non-nil.\nIf the value is t, that means do an ordinary quit.\nIf the value equals `throw-on-input', that means quit by throwing\nto the tag specified in `throw-on-input'; it's for handling `while-no-input'.\nTyping C-g sets `quit-flag' to t, regardless of `inhibit-quit',\nbut `inhibit-quit' non-nil prevents anything from taking notice of that.");
  (*Vquit_flag.as_mut_ptr ()) = Qnil ();

  DEFVAR_LISP! (c"inhibit-quit", Vinhibit_quit,
	       "Non-nil inhibits C-g quitting from happening immediately.\nNote that `quit-flag' will still be set by typing C-g,\nso a quit will be signaled as soon as `inhibit-quit' is nil.\nTo prevent this happening, set `quit-flag' to nil\nbefore making `inhibit-quit' nil.");
  (*Vinhibit_quit.as_mut_ptr ()) = Qnil ();

  DEFSYM! (Qsetq, "setq");
  DEFSYM! (Qinhibit_quit, "inhibit-quit");
  DEFSYM! (Qautoload, "autoload");
  DEFSYM! (Qinhibit_debugger, "inhibit-debugger");
  DEFSYM! (Qmacro, "macro");

  /* Note that the process handling also uses Qexit, but we don't want
     to staticpro it twice, so we just do it here.  */
  DEFSYM! (Qexit, "exit");

  DEFSYM! (Qinteractive, "interactive");
  DEFSYM! (Qcommandp, "commandp");
  DEFSYM! (Qand_rest, "&rest");
  DEFSYM! (Qand_optional, "&optional");
  DEFSYM! (QCdocumentation, ":documentation");
  DEFSYM! (Qdebug, "debug");
  DEFSYM! (Qdebug_early, "debug-early");
  DEFSYM! (Qdebug_early__handler, "debug-early--handler");
  DEFSYM! (Qdebugger_may_continue, "debugger-may-continue");
  DEFSYM! (Qdisplay_warning, "display-warning");
  DEFSYM! (Qlosing_value, "losing-value");

  DEFVAR_LISP! (c"inhibit-debugger", Vinhibit_debugger,
	       "Non-nil means never enter the debugger.\nNormally set while the debugger is already active, to avoid recursive\ninvocations.");
  (*Vinhibit_debugger.as_mut_ptr ()) = Qnil ();

  DEFVAR_LISP! (c"debug-on-error", Vdebug_on_error,
	       "Non-nil means enter debugger if an error is signaled.\nDoes not apply to errors handled by `condition-case' or those\nmatched by `debug-ignored-errors'.\nIf the value is a list, an error only means to enter the debugger\nif one of its condition symbols appears in the list.\nWhen you evaluate an expression interactively, this variable\nis temporarily non-nil if `eval-expression-debug-on-error' is non-nil.\nThe command `toggle-debug-on-error' toggles this.\nSee also the variable `debug-on-quit' and `inhibit-debugger'.");
  (*Vdebug_on_error.as_mut_ptr ()) = Qnil ();

  DEFVAR_LISP! (c"debug-ignored-errors", Vdebug_ignored_errors,
    "List of errors for which the debugger should not be called.\nEach element may be a condition-name or a regexp that matches error messages.\nIf any element applies to a given error, that error skips the debugger\nand just returns to top level.\nIf you invoke Emacs with --debug-init, and want to remove some\nelements from the default value of this variable, use `setq' to\nchange the value of the variable to a new list, rather than `delq'\nto remove some errors from the list.\nThis overrides the variable `debug-on-error'.\nIt does not apply to errors handled by `condition-case'.");
  (*Vdebug_ignored_errors.as_mut_ptr ()) = Qnil ();

  DEFVAR_BOOL! (c"debug-on-quit", debug_on_quit,
    "Non-nil means enter debugger if quit is signaled (C-g, for example).\nDoes not apply if quit is handled by a `condition-case'.");
  (*debug_on_quit.as_mut_ptr ()) = false;

  DEFVAR_BOOL! (c"debug-on-next-call", debug_on_next_call,
	       "Non-nil means enter debugger before next `eval', `apply' or `funcall'.");

  DEFVAR_BOOL! (c"backtrace-on-redisplay-error", backtrace_on_redisplay_error,
	       "Non-nil means create a backtrace if a lisp error occurs in redisplay.\nThe backtrace is written to buffer *Redisplay-trace*.");
  (*backtrace_on_redisplay_error.as_mut_ptr ()) = false;

  DEFVAR_BOOL! (c"debugger-may-continue", debugger_may_continue,
	       "Non-nil means debugger may continue execution.\nThis is nil when the debugger is called under circumstances where it\nmight not be safe to continue.");
  (*debugger_may_continue.as_mut_ptr ()) = true;

  DEFVAR_BOOL! (c"debugger-stack-frame-as-list", debugger_stack_frame_as_list,
	       "Non-nil means display call stack frames as lists.");
  (*debugger_stack_frame_as_list.as_mut_ptr ()) = false;

  DEFSYM! (Qdebugger, "debugger");
  DEFVAR_LISP! (c"debugger", Vdebugger,
	       "Function to call to invoke debugger.\nIf due to frame exit, arguments are `exit' and the value being returned;\n this function's value will be returned instead of that.\nIf due to error, arguments are `error' and a list of arguments to `signal'.\nIf due to `apply' or `funcall' entry, one argument, `lambda'.\nIf due to `eval' entry, one argument, t.\nIF the desired entry point of the debugger is higher in the call stack,\nit can be specified with the keyword argument `:backtrace-base', whose\nformat should be the same as the BASE argument of `backtrace-frame'.");
  (*Vdebugger.as_mut_ptr ()) = Qdebug_early ();

  DEFVAR_LISP! (c"signal-hook-function", Vsignal_hook_function,
	       "If non-nil, this is a function for `signal' to call.\nIt receives the same arguments that `signal' was given.\nThe Edebug package uses this to regain control.");
  (*Vsignal_hook_function.as_mut_ptr ()) = Qnil ();

  DEFVAR_LISP! (c"debug-on-signal", Vdebug_on_signal,
	       "Non-nil means call the debugger regardless of condition handlers.\nNote that `debug-on-error', `debug-on-quit' and friends\nstill determine whether to handle the particular condition.");
  (*Vdebug_on_signal.as_mut_ptr ()) = Qnil ();

  DEFVAR_BOOL! (c"backtrace-on-error-noninteractive",
               backtrace_on_error_noninteractive,
               "Non-nil means print backtrace on error in batch mode.\nIf this is nil, errors in batch mode will just print the error\nmessage upon encountering an unhandled error, without showing\nthe Lisp backtrace.");
  (*backtrace_on_error_noninteractive.as_mut_ptr ()) = true;

  /* The value of num_nonmacro_input_events as of the last time we
   started to enter the debugger.  If we decide to enter the debugger
   again when this is still equal to num_nonmacro_input_events, then we
   know that the debugger itself has an error, and we should just
   signal the error instead of entering an infinite loop of debugger
   invocations.  */
  DEFSYM! (Qinternal_when_entered_debugger, "internal-when-entered-debugger");
  DEFVAR_INT! (c"internal-when-entered-debugger", when_entered_debugger,
              "The number of keyboard events as of last time `debugger' was called.\nUsed to avoid infinite loops if the debugger itself has an error.\nDon't set this unless you're sure that can't happen.");

  /* When lexical binding is being used,
   Vinternal_interpreter_environment is non-nil, and contains an alist
   of lexically-bound variable, or (t), indicating an empty
   environment.  The lisp name of this variable would be
   `internal-interpreter-environment' if it weren't hidden.
   Every element of this list can be either a cons (VAR . VAL)
   specifying a lexical binding, or a single symbol VAR indicating
   that this variable should use dynamic scoping.  */
  DEFSYM! (Qinternal_interpreter_environment,
	  "internal-interpreter-environment");
  DEFVAR_LISP! (c"internal-interpreter-environment",
		Vinternal_interpreter_environment,
	       "If non-nil, the current lexical environment of the lisp interpreter.\nWhen lexical binding is not being used, this variable is nil.\nA value of `(t)' indicates an empty environment, otherwise it is an\nalist of active lexical bindings.");
  (*Vinternal_interpreter_environment.as_mut_ptr ()) = Qnil ();
  /* Don't export this variable to Elisp, so no one can mess with it
     (Just imagine if someone makes it buffer-local).  */
  Funintern (Qinternal_interpreter_environment (), Qnil ());

  DEFVAR_LISP! (c"internal-make-interpreted-closure-function",
	       Vinternal_make_interpreted_closure_function,
	       "Function to filter the env when constructing a closure.");
  (*Vinternal_make_interpreted_closure_function.as_mut_ptr ()) = Qnil ();

  (*Vrun_hooks.as_mut_ptr ()) = intern_c_string (c"run-hooks".as_ptr ());
  staticpro (Vrun_hooks.as_mut_ptr ());

  staticpro (Vautoload_queue.as_mut_ptr ());
  (*Vautoload_queue.as_mut_ptr ()) = Qnil ();
  staticpro (Vsignaling_function.as_mut_ptr ());
  (*Vsignaling_function.as_mut_ptr ()) = Qnil ();

  staticpro (Qcatch_all_memory_full.as_mut_ptr ());
  /* Make sure Qcatch_all_memory_full is a unique object.  We could
     also use something like Fcons (Qnil, Qnil), but json.c treats any
     cons cell as error data, so use an uninterned symbol instead.  */
  (*Qcatch_all_memory_full.as_mut_ptr ())
    = Fmake_symbol (build_string (c"catch-all-memory-full".as_ptr ()));

  staticpro (list_of_t.as_mut_ptr ());
  (*list_of_t.as_mut_ptr ()) = list1 (Qt ());

  defsubr (Sor.as_mut_ptr ());
  defsubr (Sand.as_mut_ptr ());
  defsubr (Sif.as_mut_ptr ());
  defsubr (Scond.as_mut_ptr ());
  defsubr (Sprogn.as_mut_ptr ());
  defsubr (Sprog1.as_mut_ptr ());
  defsubr (Ssetq.as_mut_ptr ());
  defsubr (Squote.as_mut_ptr ());
  defsubr (Sfunction.as_mut_ptr ());
  defsubr (Smake_interpreted_closure.as_mut_ptr ());
  defsubr (Sdefault_toplevel_value.as_mut_ptr ());
  defsubr (Sset_default_toplevel_value.as_mut_ptr ());
  defsubr (Sbuffer_local_toplevel_value.as_mut_ptr ());
  defsubr (Sset_buffer_local_toplevel_value.as_mut_ptr ());
  defsubr (Sdefvar.as_mut_ptr ());
  defsubr (Sdefvar_1.as_mut_ptr ());
  defsubr (Sdefvaralias.as_mut_ptr ());
  DEFSYM! (Qdefvaralias, "defvaralias");
  defsubr (Sinternal_delete_indirect_variable.as_mut_ptr ());
  defsubr (Sdefconst.as_mut_ptr ());
  defsubr (Sdefconst_1.as_mut_ptr ());
  defsubr (Sinternal__define_uninitialized_variable.as_mut_ptr ());
  defsubr (Smake_var_non_special.as_mut_ptr ());
  defsubr (Slet.as_mut_ptr ());
  defsubr (SletX.as_mut_ptr ());
  defsubr (Swhile.as_mut_ptr ());
  defsubr (Sfuncall_with_delayed_message.as_mut_ptr ());
  defsubr (Smacroexpand.as_mut_ptr ());
  defsubr (Scatch.as_mut_ptr ());
  defsubr (Sthrow.as_mut_ptr ());
  defsubr (Sunwind_protect.as_mut_ptr ());
  defsubr (Scondition_case.as_mut_ptr ());
  defsubr (Shandler_bind_1.as_mut_ptr ());
  DEFSYM! (QCsuccess, ":success");
  defsubr (Ssignal.as_mut_ptr ());
  defsubr (Scommandp.as_mut_ptr ());
  defsubr (Sautoload.as_mut_ptr ());
  defsubr (Sautoload_do_load.as_mut_ptr ());
  defsubr (Seval.as_mut_ptr ());
  defsubr (Sapply.as_mut_ptr ());
  defsubr (Sfuncall.as_mut_ptr ());
  defsubr (Sfunc_arity.as_mut_ptr ());
  defsubr (Srun_hooks.as_mut_ptr ());
  defsubr (Srun_hook_with_args.as_mut_ptr ());
  defsubr (Srun_hook_with_args_until_success.as_mut_ptr ());
  defsubr (Srun_hook_with_args_until_failure.as_mut_ptr ());
  defsubr (Srun_hook_wrapped.as_mut_ptr ());
  defsubr (Sbacktrace_debug.as_mut_ptr ());
  DEFSYM! (QCdebug_on_exit, ":debug-on-exit");
  defsubr (Smapbacktrace.as_mut_ptr ());
  defsubr (Sbacktrace_frame_internal.as_mut_ptr ());
  defsubr (Sbacktrace_frames_from_thread.as_mut_ptr ());
  defsubr (Sbacktrace_eval.as_mut_ptr ());
  defsubr (Sbacktrace__locals.as_mut_ptr ());
  defsubr (Sspecial_variable_p.as_mut_ptr ());
  DEFSYM! (Qfunctionp, "functionp");
  defsubr (Sfunctionp.as_mut_ptr ());
  defsubr (Sdebugger_trap.as_mut_ptr ());

  }
}

DEFUN! ("commandp", Fcommandp, Scommandp, 1, 2, 0,
       doc: "Non-nil if FUNCTION makes provisions for interactive calling.\nThis means it contains a description for how to read arguments to give it.\nThe value is nil for an invalid function or a symbol with no function\ndefinition.\n\nInteractively callable functions include strings and vectors (treated\nas keyboard macros), lambda-expressions that contain a top-level call\nto `interactive', autoload definitions made by `autoload' with non-nil\nfourth argument, and some of the built-in functions of Lisp.\n\nAlso, a symbol satisfies `commandp' if its function definition does so.\n\nIf the optional argument FOR-CALL-INTERACTIVELY is non-nil,\nthen strings and vectors are not accepted.",
       (function: Lisp_Object, for_call_interactively: Lisp_Object),
       {
  let mut fun: Lisp_Object;
  let mut genfun: bool = false; /* If true, we should consult `interactive-form'.  */

  fun = function;

  fun = indirect_function (fun);
  if NILP (fun)
    { return Qnil (); }

  /* Emacs primitives are interactive if their DEFUN specifies an
     interactive spec.  */
  if SUBRP (fun)
    {
      if !(*XSUBR (fun)).intspec.string.is_null ()
        { return Qt (); }
    }
  /* Bytecode objects are interactive if they are long enough to
     have an element whose index is CLOSURE_INTERACTIVE, which is
     where the interactive spec is stored.  */
  else if CLOSUREP (fun)
    {
      if PVSIZE (fun) > CLOSURE_INTERACTIVE as isize
        { return Qt (); }
      else if PVSIZE (fun) > CLOSURE_DOC_STRING as isize
        {
          let doc: Lisp_Object = AREF (fun, CLOSURE_DOC_STRING as isize);
          /* An invalid "docstring" is a sign that we have an OClosure.  */
          genfun = !(NILP (doc) || VALID_DOCSTRING_P (doc));
        }
    }

  else
    {
      /* C 2349-2357's `#ifdef HAVE_MODULES` arm (dead in the frozen configuration; P-51).
	 It sits in the `else` the live chain below would otherwise occupy because an
	 attribute cannot prefix an `else if`; the arm only ever returns, so the chain's
	 meaning is unchanged.  */
      #[cfg (feature = "have_modules")]
      {
        unsafe extern "C" {
        }
        /* Module functions are interactive if their `interactive_form'
           field is non-nil. */
        if MODULE_FUNCTIONP (fun)
          {
            if !NILP (module_function_interactive_form (XMODULE_FUNCTION (fun) as *const c_void))
              { return Qt (); }
          }
      }

      /* Strings and vectors are keyboard macros.  */
      if STRINGP (fun) || VECTORP (fun)

    { return if NILP (for_call_interactively) { Qt () } else { Qnil () }; }

  /* Lists may represent commands.  */
  else if !CONSP (fun)
    { return Qnil (); }
  else
    {
      let funcar: Lisp_Object = XCAR (fun);
      if EQ (funcar, Qautoload ())
        {
          if !NILP (Fcar (Fcdr (Fcdr (XCDR (fun)))))
            { return Qt (); }
        }
      else
        {
          let body: Lisp_Object = CDR_SAFE (XCDR (fun));
          if !EQ (funcar, Qlambda ())
	    { return Qnil (); }
	  if !NILP (Fassq (Qinteractive (), body))
	    { return Qt (); }
	  else
	    { return Qnil (); }
	}
        }
    }

  /* By now, if it's not a function we already returned nil.  */

  /* Check an `interactive-form' property if present, analogous to the
     function-documentation property.  */
  fun = function;
  while SYMBOLP (fun)
    {
      let tmp: Lisp_Object = Fget (fun, Qinteractive_form ());
      if !NILP (tmp)
	{ error (c"Found an 'interactive-form' property!".as_ptr ()); }
      fun = Fsymbol_function (fun);
    }

  /* If there's no immediate interactive form but it's an OClosure,
     then delegate to the generic-function in case it has
     a type-specific interactive-form.  */
  if genfun
    {
      let iform: Lisp_Object = calln! (Qinteractive_form (), fun);
      return if NILP (iform) { Qnil () } else { Qt () };
    }
  else
    { return Qnil (); }
});

DEFUN! ("autoload", Fautoload, Sautoload, 2, 5, 0,
       doc: "Define FUNCTION to autoload from FILE.\nFUNCTION is a symbol; FILE is a file name string to pass to `load'.\n\nThird arg DOCSTRING is documentation for the function.\n\nFourth arg INTERACTIVE if non-nil says function can be called\ninteractively.  If INTERACTIVE is a list, it is interpreted as a list\nof modes the function is applicable for.\n\nFifth arg TYPE indicates the type of the object:\n   nil or omitted says FUNCTION is a function,\n   `keymap' says FUNCTION is really a keymap, and\n   `macro' or t says FUNCTION is really a macro.\n\nThird through fifth args give info about the real definition.\nThey default to nil.\n\nIf FUNCTION is already defined other than as an autoload,\nthis does nothing and returns nil.",
       (function: Lisp_Object, file: Lisp_Object, docstring: Lisp_Object,
        interactive: Lisp_Object, r#type: Lisp_Object),
       {
  CHECK_SYMBOL (function);
  CHECK_STRING (file);

  /* If function is defined and not as an autoload, don't override.  */
  if !NILP ((*XSYMBOL (function)).u.s.function)
      && !AUTOLOADP ((*XSYMBOL (function)).u.s.function)
    { return Qnil (); }

  return Fdefalias (function,
		    list5 (Qautoload (), file, docstring, interactive, r#type),
		    Qnil ());
});

unsafe extern "C-unwind" fn un_autoload (oldqueue: Lisp_Object)
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  /* Queue to unwind is current value of Vautoload_queue.
     oldqueue is the shadowed value to leave in Vautoload_queue.  */
  let mut queue: Lisp_Object = (*Vautoload_queue.as_mut_ptr ());
  (*Vautoload_queue.as_mut_ptr ()) = oldqueue;
  while CONSP (queue)
    {
      let first: Lisp_Object = XCAR (queue);
      if CONSP (first) && BASE_EQ (XCAR (first), make_fixnum (0))
	{ (*Vfeatures.as_mut_ptr ()) = XCDR (first); }
      else
	{ Ffset (first, Fcar (Fcdr (Fget (first, Qfunction_history ())))); }
      queue = XCDR (queue);
    }

  }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn load_with_autoload_queue
  (file: Lisp_Object, noerror: Lisp_Object, nomessage: Lisp_Object,
   nosuffix: Lisp_Object, must_suffix: Lisp_Object) -> Lisp_Object
{
  // SAFETY: VM evaluator/bytecode/primitive boundary; called on the active Lisp thread with rooted `specpdl`/`bc` state, propagating failures via `NonLocalExit`.
  unsafe {
  let count: specpdl_ref = SPECPDL_INDEX ();

  /* If autoloading gets an error (which includes the error of failing
     to define the function being called), we use Vautoload_queue
     to undo function definitions and `provide' calls made by
     the function.  We do this in the specific case of autoloading
     because autoloading is not an explicit request "load this file",
     but rather a request to "call this function".

     The value saved here is to be restored into Vautoload_queue.  */
  record_unwind_protect (Some (un_autoload), (*Vautoload_queue.as_mut_ptr ()));
  (*Vautoload_queue.as_mut_ptr ()) = Qt ();
  let tem: Lisp_Object
    = save_match_data_load (file, noerror, nomessage, nosuffix, must_suffix);

  /* Once loading finishes, don't undo it.  */
  (*Vautoload_queue.as_mut_ptr ()) = Qt ();
  unbind_to (count, Qnil ());
  return tem;

  }
}

/* Load an autoloaded function.
   FUNNAME is the symbol which is the function's name.
   FUNDEF is the autoload definition (a list).  */

DEFUN! ("autoload-do-load", Fautoload_do_load, Sautoload_do_load, 1, 3, 0,
       doc: "Load FUNDEF which should be an autoload.\nIf non-nil, FUNNAME should be the symbol whose function value is FUNDEF,\nin which case the function returns the new autoloaded function value.\nIf equal to `macro', MACRO-ONLY specifies that FUNDEF should only be loaded if\nit defines a macro.",
       (fundef: Lisp_Object, funname: Lisp_Object, macro_only: Lisp_Object),
       {
  if !CONSP (fundef) || !EQ (Qautoload (), XCAR (fundef))
    { return fundef; }

  let kind: Lisp_Object = Fnth (make_fixnum (4), fundef);
  if EQ (macro_only, Qmacro ())
      && !(EQ (kind, Qt ()) || EQ (kind, Qmacro ()))
    { return fundef; }

  /* This is to make sure that loadup.el gives a clear picture
     of what files are preloaded and when.  */
  if will_dump_p () && !will_bootstrap_p ()
    {
      /* Avoid landing here recursively while outputting the
	 backtrace from the error.  */
      (*gflags.as_mut_ptr ()).will_dump = false;
      error (c"Attempt to autoload %s while preparing to dump".as_ptr (),
	     SDATA (SYMBOL_NAME (funname)));
    }

  CHECK_SYMBOL (funname);

  /* If `macro_only' is set and fundef isn't a macro, assume this autoload to
     be a "best-effort" (e.g. to try and find a compiler macro),
     so don't signal an error if autoloading fails.  */
  let ignore_errors: Lisp_Object
    = if EQ (kind, Qt ()) || EQ (kind, Qmacro ()) { Qnil () } else { macro_only };
  load_with_autoload_queue (Fcar (Fcdr (fundef)), ignore_errors, Qt (), Qnil (), Qt ());

  if NILP (funname) || !NILP (ignore_errors)
    { return Qnil (); }
  else
    {
      let fun: Lisp_Object = Findirect_function (funname, Qnil ());

      if !NILP (Fequal (fun, fundef))
	{ error (c"Autoloading file %s failed to define function %s".as_ptr (),
	         SDATA (Fcar (Fcar ((*Vload_history.as_mut_ptr ())))),
	         SDATA (SYMBOL_NAME (funname))); }
      else
	{ return fun; }
    }
});
