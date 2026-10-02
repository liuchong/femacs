// femacs: port of src/thread.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror (P-66): `lisp.h` 2289 includes thread.h unconditionally, and data.c's
// `cl-type-of`, `threadp`, `mutexp` and `condition-variable-p` need the three predicates
// below.  `struct thread_state`, `struct Lisp_Mutex`, `struct Lisp_CondVar`, the checkpoint
// machinery and the rest of the header land with thread.c (batch D), which is when
// docs/ports/include-thread.md's remaining boxes are ticked.

// thread.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const THREAD_H: bool = true;

/* thread.h 222-226: THREADP.  */
#[inline(always)]
pub fn THREADP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_THREAD) }

/* thread.h 269-273: MUTEXP.  */
#[inline(always)]
pub fn MUTEXP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_MUTEX) }

/* thread.h 303-307: CONDVARP.  */
#[inline(always)]
pub fn CONDVARP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_CONDVAR) }
