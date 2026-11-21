// femacs: mirror of the frozen target's <stdcountof.h> (C23; system header, P-26).
//
// The whole header is one macro, and the frozen configuration has HAVE_STDCOUNTOF_H = 1.

// [U1-MODULE-MIRROR]
use super::*;
/* #define countof(a) (sizeof (a) / sizeof *(a))  */
macro_rules! countof {
    ($a:expr) => {
        (std::mem::size_of_val (&$a) / std::mem::size_of_val (&$a[0]))
    };
}
