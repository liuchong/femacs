#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::sys_common::*;
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// femacs: port of lib/memset_explicit.c @ fd7afced403.  Blind file.
// The frozen configuration gives the `HAVE_MEMSET_S` arm (macros.tsv 1432 is 1, with
// HAVE_EXPLICIT_MEMSET undefined and HAVE_MEMSET_S_SUPPORTS_ZERO 1), so C 16's
// `(void) memset_s (s, len, c, len);` and C 17's `return s;` are the live text; the other
// three arms are named at their sites.  The reference object confirms the name:
// `nm -g build/emacs-ref/lib/memset_explicit.o` prints `T _memset_explicit`.
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/string.rs imported via common

/* Set S's bytes to C, where S has LEN bytes.  The compiler will not
   optimize effects away, even if S is dead after the call.  */

/* void *memset_explicit (void *s, int c, size_t len)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset_explicit (s: *mut c_void, c: c_int, len: usize) -> *mut c_void
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  /* C 10-36's four arms, in C's order.  `HAVE_EXPLICIT_MEMSET` and `HAVE_MEMSET_S` are the
     configuration's; the third (`__GNUC__ && !__clang__`) cannot apply to rustc, and the port is
     compiled by LLVM, so the clang arm (`memset` plus a barrier) is the one that stands for both
     when neither macro is set - which is the Android configuration's case (status 4.717).  */
  #[cfg (feature = "have_explicit_memset")]
  {
    /* C 10-11: `return explicit_memset (s, c, len);`  */
    return explicit_memset (s, c, len);
  }

  #[cfg (all (not (feature = "have_explicit_memset"), feature = "have_memset_s"))]
  {
    /* C 13-17: the `# if !HAVE_MEMSET_S_SUPPORTS_ZERO` guard, then the call.  */
    #[cfg (not (feature = "have_memset_s_supports_zero"))]
    {
      if len > 0
        { let _ = memset_s (s, len, c, len); }
      return s;
    }
    #[cfg (feature = "have_memset_s_supports_zero")]
    {
      let _ = memset_s (s, len, c, len);
      return s;
    }
  }

  #[cfg (all (not (feature = "have_explicit_memset"), not (feature = "have_memset_s")))]
  {
    /* C 23-28: "With asm ("" ::: "memory") LLVM analyzes uses of 's' and finds that the whole
       thing is dead and eliminates it.  Use 'g' to work around this problem."  Rust's documented
       equivalent of that barrier is `black_box` (it keeps the operand live and the write
       unremovable); Rust has no inline asm on every target, so the barrier is spelled this way
       and the deviation from C's inline assembly is recorded here (PORTING.md P-309).  */
    let _ = memset (s, c, len);
    let _ = std::hint::black_box (s);
    return s;
  }

  }
}
