// femacs: WIP mirror of lib/attribute.h @ fd7afced403 (PORTING.md P-26).
//
// The regex cluster is the first port that includes <attribute.h>; this mirror carries
// only the macros it names, and the rest of the header is tracked in
// docs/ports/include-attribute.md.  The C body of each is an `__attribute__`, which the
// port records as a no-op (PORTING.md 8); the *names* stay so the ledger and the
// declaration sites read the same.  `lib/cdefs.h` (which the reference pulls in through
// libc-config.h) supplies the same two `__attribute_*` spellings; they live here because
// the port has no cdefs.h mirror yet and both are pure attributes.

/* attribute.h 210: `#define FALLTHROUGH _GL_ATTRIBUTE_FALLTHROUGH` -- GCC/Clang accept
   the `fallthrough` statement attribute, which Rust has no counterpart for.  */
macro_rules! FALLTHROUGH {
    () => {};
}

/* cdefs.h 503: `#define __attribute_noinline__ __attribute__ ((__noinline__))` -- the
   port spells `#[inline(never)]` where it matters (PORTING.md 8).  */
macro_rules! __attribute_noinline__ {
    () => {};
}

/* cdefs.h 578: `#define __attribute_warn_unused_result__ __attribute__ ((__warn_unused_result__))`
   -- Rust's `#[must_use]` is the equivalent; kept as a no-op token at the same sites.  */
macro_rules! __attribute_warn_unused_result__ {
    () => {};
}

/* attribute.h 187: `#define NODISCARD _GL_ATTRIBUTE_NODISCARD`, which lib/cdefs.h spells
   `__attribute__ ((__warn_unused_result__))`; systhread.h 152-155 puts the token in front of
   three declarations, and the port keeps the token as a no-op at the same sites (the
   `__attribute_warn_unused_result__` row above; the three `NODISCARD` sites themselves are
   recorded in docs/ports/include-systhread.md).  */
macro_rules! NODISCARD {
    () => {};
}

/* libc-config.h 139/144 undefine and re-define these for the non-glibc build; the frozen
   target's definition is the empty one, so the port's no-op matches.  */
macro_rules! attribute_compat_text_section {
    () => {};
}
