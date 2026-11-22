// femacs: port of src/sort.c @ fd7afced403.
//
// Core Timsort + Powersort algorithm lives in `#![forbid(unsafe_code)]` `crate::safe_sort`
// (`U7` / `P-486`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;
use crate::roots::RootedVec;

struct SeqWritebackGuard {
  dst: *mut Lisp_Object,
  values: RootedVec,
}

impl Drop for SeqWritebackGuard {
  fn drop (&mut self) {
    if !self.dst.is_null () && !self.values.is_empty () {
      // SAFETY: `tim_sort` caller owns `dst[0..values.len()]` in non-moving vector or `SAFE_ALLOCA`
      // storage for the duration of `tim_sort`; writing back on both normal return and unwind
      // preserves all elements without loss or duplication.
      unsafe {
        core::ptr::copy_nonoverlapping (
          self.values.as_slice ().as_ptr (),
          self.dst,
          self.values.len (),
        );
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn tim_sort (
  predicate: Lisp_Object,
  keyfunc: Lisp_Object,
  seq: *mut Lisp_Object,
  length: ptrdiff_t,
  reverse: bool,
)
{
  if length <= 0 || seq.is_null () {
    return;
  }
  let len = length as usize;
  // SAFETY: `tim_sort` ABI contract supplies `length` initialized `Lisp_Object` elements at `seq`.
  let init_slice = unsafe { core::slice::from_raw_parts (seq, len) };
  let mut guard = SeqWritebackGuard {
    dst: seq,
    values: RootedVec::from_slice (init_slice),
  };
  crate::safe_sort::tim_sort_rooted (predicate, keyfunc, &mut guard.values, reverse);
}
