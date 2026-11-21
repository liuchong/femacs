// femacs: port of src/blockinput.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): only declarations, constants and inline functions.  The
// two `INLINE` bodies (`block_input` and `input_blocked_p`) are `#[inline(always)]` fns, the
// three `extern` entries are declarations of keyboard.c's definitions, and the one object is
// declared the way the mirror of a header does (P-21's shadowing is what lets keyboard.rs own
// the definition).
//
// `INLINE_HEADER_BEGIN`/`END` are attribute/spelling macros with no counterpart
// (docs/ports/excluded-macros.txt).

// blockinput.h 21: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_BLOCKINPUT_H: bool = true;

/* Emacs should avoid doing anything hairy in a signal handler, because
   so many system functions are non-reentrant. … */

unsafe extern "C-unwind" { pub static mut interrupt_input_blocked: c_int; }        // blockinput.h 50

/* Begin critical section. */

/* INLINE void block_input (void)  */
#[inline(always)]
pub unsafe fn block_input ()
{
  interrupt_input_blocked += 1;
}

unsafe extern "C-unwind" {
  pub fn unblock_input ();                                                   // blockinput.h 57
  pub fn totally_unblock_input ();                                           // blockinput.h 58
  pub fn unblock_input_to (level: c_int);                                    // blockinput.h 59
}

/* In critical section?  */

/* INLINE bool input_blocked_p (void)  */
#[inline(always)]
pub unsafe fn input_blocked_p () -> bool
{
  return interrupt_input_blocked > 0;
}
