// femacs: WIP mirror of lib/verify.h @ fd7afced403 (PORTING.md P-26/P-11).
//
// The regex cluster uses `assume` (through regex_internal.h 40-44's DEBUG_ASSERT) and one
// `verify`.  The remaining arms of verify.h are tracked in docs/ports/include-verify.md.
// `assume (R)` is `((R) ? (void) 0 : __builtin_unreachable ())` (verify.h 389); the port
// keeps it as a macro so the C's single-evaluation shape survives (PORTING.md P-16).

/* verify.h 389 (the GCC/Clang arm the frozen target takes).  */
macro_rules! assume {
    ($r:expr) => {
        if !($r) { unsafe { core::hint::unreachable_unchecked () } }
    };
}

/* verify.h 360/362: `verify (R)` is a compile-time assertion.  */
macro_rules! verify {
    ($r:expr) => {
        const _: () = assert! ($r);
    };
}
