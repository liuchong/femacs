// femacs: mirror of the frozen target's <stdcountof.h> (C23; system header, P-26).
//
// The whole header is one macro, and the frozen configuration has HAVE_STDCOUNTOF_H = 1.

#![forbid(unsafe_code)]
// [U1-MODULE-MIRROR]
use super::*;
#[inline(always)]
pub const fn femacs_countof_ptr<T, const N: usize>(_p: *const [T; N]) -> usize {
    N
}
#[inline(always)]
pub const fn femacs_sizeof_ptr<T>(_p: *const T) -> usize {
    core::mem::size_of::<T>()
}
#[inline(always)]
pub const fn femacs_array_ptr<T, const N: usize>(p: *const [T; N]) -> *const T {
    p as *const T
}
#[inline(always)]
pub const fn femacs_array_mut_ptr<T, const N: usize>(p: *mut [T; N]) -> *mut T {
    p as *mut T
}
/* #define countof(a) (sizeof (a) / sizeof *(a))  */
macro_rules! countof {
    ($a:expr) => {
        femacs_countof_ptr(&raw const $a)
    };
}
