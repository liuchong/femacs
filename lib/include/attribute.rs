// femacs: port of lib/attribute.h @ fd7afced403.
//
// The header is a thin mapping: every `ATTRIBUTE_*`, `DEPRECATED`, `FALLTHROUGH`, … name
// forwards to the private `_GL_ATTRIBUTE_*`/`_GL_UNNAMED` spelling, which gnulib's
// configuration block in `config.h` defines as a C attribute (`__attribute__ ((...))` or
// C23's `[[...]]` form).  Rust's attributes are a different mechanism, and PORTING.md 8
// records the mapping: `#[inline(never)]`/`#[inline(always)]`, `#[must_use]`,
// `#[repr(packed)]`, `#[cold]`, one `#[unsafe(link_section)]` where a section is wanted, and
// no counterpart for the rest - the port's macro is then the empty expansion its sites do
// not need.
//
// Two layers are kept so the file reads as C's: each `_GL_ATTRIBUTE_*` name is here as the
// no-op its attribute is (the names live in the generated `config.h`, which the port has no
// macro-shaped mirror for; the `__attribute_noinline__`/`__attribute_warn_unused_result__`
// pair below is the same arrangement for two `lib/cdefs.h` names that were already here), and
// each public name forwards to its private one, so a use site that keeps C's token still
// expands to nothing.  The extractor compares a macro's body tokens, which is why the
// forwarding bodies name the private macro rather than being empty.
//
// `_GL_UNNAMED (id)` is empty on the frozen target (`__STDC_VERSION__` is 202311, so
// `config.h`'s glibc-only `unnamed_##id _GL_ATTRIBUTE_UNUSED` arm is dead text).

// [U1-MODULE-MIRROR]
use super::*;
/* attribute.h 69: the include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _GL_ATTRIBUTE_H: bool = true;

/* attribute.h 109: `#define DEPRECATED _GL_ATTRIBUTE_DEPRECATED` - warn if the entity is
   used; the port's counterpart is `#[deprecated]` where a site wants the warning.  */
macro_rules! _GL_ATTRIBUTE_DEPRECATED {
  () => {};
}
macro_rules! DEPRECATED {
  () => { _GL_ATTRIBUTE_DEPRECATED! () };
}

/* attribute.h 113/117: warn or fail when a call is not optimized away.  */
macro_rules! _GL_ATTRIBUTE_WARNING {
  ($msg:tt) => {};
}
macro_rules! ATTRIBUTE_WARNING {
  ($msg:tt) => { _GL_ATTRIBUTE_WARNING! ($msg) };
}
macro_rules! _GL_ATTRIBUTE_ERROR {
  ($msg:tt) => {};
}
macro_rules! ATTRIBUTE_ERROR {
  ($msg:tt) => { _GL_ATTRIBUTE_ERROR! ($msg) };
}

/* attribute.h 124: the function returns a pointer to freshly allocated memory.  */
macro_rules! _GL_ATTRIBUTE_MALLOC {
  () => {};
}
macro_rules! ATTRIBUTE_MALLOC {
  () => { _GL_ATTRIBUTE_MALLOC! () };
}

/* attribute.h 131: `ATTRIBUTE_ALLOC_SIZE ((N))` / `((M, N))` - the argument that carries
   the size of the returned memory block.  */
macro_rules! _GL_ATTRIBUTE_ALLOC_SIZE {
  ($args:tt) => {};
}
macro_rules! ATTRIBUTE_ALLOC_SIZE {
  ($args:tt) => { _GL_ATTRIBUTE_ALLOC_SIZE! ($args) };
}

/* attribute.h 139/140: the returned pointers can be freed by passing them as the Ith
   argument to F, or through `free`.  */
macro_rules! _GL_ATTRIBUTE_DEALLOC {
  ($f:tt, $i:tt) => {};
}
macro_rules! ATTRIBUTE_DEALLOC {
  ($f:tt, $i:tt) => { _GL_ATTRIBUTE_DEALLOC! ($f, $i) };
}
macro_rules! _GL_ATTRIBUTE_DEALLOC_FREE {
  () => {};
}
macro_rules! ATTRIBUTE_DEALLOC_FREE {
  () => { _GL_ATTRIBUTE_DEALLOC_FREE! () };
}

/* attribute.h 148: the function expects a trailing NULL argument.  */
macro_rules! _GL_ATTRIBUTE_SENTINEL {
  ($pos:tt) => {};
}
macro_rules! ATTRIBUTE_SENTINEL {
  ($pos:tt) => { _GL_ATTRIBUTE_SENTINEL! ($pos) };
}

/* attribute.h 167: `ATTRIBUTE_FORMAT ((ARCHETYPE, STRING-INDEX, FIRST-TO-CHECK))`.  */
macro_rules! _GL_ATTRIBUTE_FORMAT {
  ($spec:tt) => {};
}
macro_rules! ATTRIBUTE_FORMAT {
  ($spec:tt) => { _GL_ATTRIBUTE_FORMAT! ($spec) };
}

/* attribute.h 172: the named pointer arguments must not be NULL.  */
macro_rules! _GL_ATTRIBUTE_NONNULL {
  ($args:tt) => {};
}
macro_rules! ATTRIBUTE_NONNULL {
  ($args:tt) => { _GL_ATTRIBUTE_NONNULL! ($args) };
}

/* attribute.h 177: argument NP (a pointer) must not be NULL if NI is nonzero.  */
macro_rules! _GL_ATTRIBUTE_NONNULL_IF_NONZERO {
  ($np:tt, $ni:tt) => {};
}
macro_rules! ATTRIBUTE_NONNULL_IF_NONZERO {
  ($np:tt, $ni:tt) => { _GL_ATTRIBUTE_NONNULL_IF_NONZERO! ($np, $ni) };
}

/* attribute.h 182: the function's return value is a non-NULL pointer.  */
macro_rules! _GL_ATTRIBUTE_RETURNS_NONNULL {
  () => {};
}
macro_rules! ATTRIBUTE_RETURNS_NONNULL {
  () => { _GL_ATTRIBUTE_RETURNS_NONNULL! () };
}

/* attribute.h 187: warn if the caller does not use the return value; the port's counterpart
   is `#[must_use]`, and the declaration sites that keep C's token (systhread.h 152-155)
   expand to nothing here.  */
macro_rules! _GL_ATTRIBUTE_NODISCARD {
  () => {};
}
macro_rules! NODISCARD {
  () => { _GL_ATTRIBUTE_NODISCARD! () };
}

/* attribute.h 200: do not warn if the entity is not used (`#[allow(dead_code)]` where a
   site wants the effect).  */
macro_rules! _GL_ATTRIBUTE_MAYBE_UNUSED {
  () => {};
}
macro_rules! MAYBE_UNUSED {
  () => { _GL_ATTRIBUTE_MAYBE_UNUSED! () };
}

/* attribute.h 205: the character array is not meant to be NUL-terminated.  */
macro_rules! _GL_ATTRIBUTE_NONSTRING {
  () => {};
}
macro_rules! ATTRIBUTE_NONSTRING {
  () => { _GL_ATTRIBUTE_NONSTRING! () };
}

/* attribute.h 210: the control flow falls through to the next `case`/`default` label; the
   port has no statement attribute for it (the call sites keep C's token: ebrowse.rs 1861,
   3593, 3715).  */
macro_rules! _GL_ATTRIBUTE_FALLTHROUGH {
  () => {};
}
macro_rules! FALLTHROUGH {
  () => { _GL_ATTRIBUTE_FALLTHROUGH! () };
}

/* attribute.h 223: the number of elements of the array member is given by the field C.  */
macro_rules! _GL_ATTRIBUTE_COUNTED_BY {
  ($c:tt) => {};
}
macro_rules! COUNTED_BY {
  ($c:tt) => { _GL_ATTRIBUTE_COUNTED_BY! ($c) };
}

/* attribute.h 232: omit the function from stack traces when debugging.  */
macro_rules! _GL_ATTRIBUTE_ARTIFICIAL {
  () => {};
}
macro_rules! ATTRIBUTE_ARTIFICIAL {
  () => { _GL_ATTRIBUTE_ARTIFICIAL! () };
}

/* attribute.h 236: make the entity visible to debuggers even with `-fwhole-program`; the
   port's counterpart is `#[unsafe(no_mangle)] pub` (PORTING.md 8).  */
macro_rules! _GL_ATTRIBUTE_EXTERNALLY_VISIBLE {
  () => {};
}
macro_rules! ATTRIBUTE_EXTERNALLY_VISIBLE {
  () => { _GL_ATTRIBUTE_EXTERNALLY_VISIBLE! () };
}

/* attribute.h 245: the function does not throw exceptions.  */
macro_rules! _GL_ATTRIBUTE_NOTHROW {
  () => {};
}
macro_rules! ATTRIBUTE_NOTHROW {
  () => { _GL_ATTRIBUTE_NOTHROW! () };
}

/* attribute.h 249: do not inline the function; the port writes `#[inline(never)]`.  */
macro_rules! _GL_ATTRIBUTE_NOINLINE {
  () => {};
}
macro_rules! ATTRIBUTE_NOINLINE {
  () => { _GL_ATTRIBUTE_NOINLINE! () };
}

/* attribute.h 254: always inline the function; the port writes `#[inline(always)]`.  */
macro_rules! _GL_ATTRIBUTE_ALWAYS_INLINE {
  () => {};
}
macro_rules! ATTRIBUTE_ALWAYS_INLINE {
  () => { _GL_ATTRIBUTE_ALWAYS_INLINE! () };
}

/* attribute.h 270: the function's result is determined solely by its arguments' values.  */
macro_rules! _GL_ATTRIBUTE_CONST {
  () => {};
}
macro_rules! ATTRIBUTE_CONST {
  () => { _GL_ATTRIBUTE_CONST! () };
}

/* attribute.h 289: it is OK for the compiler to move or omit the call (C23
   `unsequenced`).  */
macro_rules! _GL_ATTRIBUTE_UNSEQUENCED {
  () => {};
}
macro_rules! UNSEQUENCED {
  () => { _GL_ATTRIBUTE_UNSEQUENCED! () };
}

/* attribute.h 303: like ATTRIBUTE_CONST, but the function may read observable state.  */
macro_rules! _GL_ATTRIBUTE_PURE {
  () => {};
}
macro_rules! ATTRIBUTE_PURE {
  () => { _GL_ATTRIBUTE_PURE! () };
}

/* attribute.h 319: it is OK for the compiler to move or omit the call if the observable
   state is the same (C23 `reproducible`).  */
macro_rules! _GL_ATTRIBUTE_REPRODUCIBLE {
  () => {};
}
macro_rules! REPRODUCIBLE {
  () => { _GL_ATTRIBUTE_REPRODUCIBLE! () };
}

/* attribute.h 323: the function is rarely executed; the port's counterpart is `#[cold]`.  */
macro_rules! _GL_ATTRIBUTE_COLD {
  () => {};
}
macro_rules! ATTRIBUTE_COLD {
  () => { _GL_ATTRIBUTE_COLD! () };
}

/* attribute.h 329: the function executes code from its own unit only by return or by
   exception handling.  */
macro_rules! _GL_ATTRIBUTE_LEAF {
  () => {};
}
macro_rules! ATTRIBUTE_LEAF {
  () => { _GL_ATTRIBUTE_LEAF! () };
}

/* attribute.h 336: the member/struct has the smallest possible alignment; the port's
   counterpart is `#[repr(packed)]`.  */
macro_rules! _GL_ATTRIBUTE_PACKED {
  () => {};
}
macro_rules! ATTRIBUTE_PACKED {
  () => { _GL_ATTRIBUTE_PACKED! () };
}

/* attribute.h 347: pointers to the type may point to the same storage as pointers to other
   types, disabling strict-aliasing optimization.  */
macro_rules! _GL_ATTRIBUTE_MAY_ALIAS {
  () => {};
}
macro_rules! ATTRIBUTE_MAY_ALIAS {
  () => { _GL_ATTRIBUTE_MAY_ALIAS! () };
}

/* attribute.h 361: `#define UNNAMED(id) _GL_UNNAMED (id)` - the "name" of an unnamed
   function parameter, which cannot be used; `config.h`'s definition is empty on this
   target (see the file note), so the port's expansion is too.  */
macro_rules! _GL_UNNAMED {
  ($id:tt) => {};
}
macro_rules! UNNAMED {
  ($id:tt) => { _GL_UNNAMED! ($id) };
}

/* --- two `lib/cdefs.h` names this mirror already carried (no attribute.h counterpart) ----
   The reference reaches them through `libc-config.h`'s `#include <cdefs.h>`.  */
/* cdefs.h 503: `#define __attribute_noinline__ __attribute__ ((__noinline__))`.  */
macro_rules! __attribute_noinline__ {
  () => {};
}
/* cdefs.h 578: `__attribute__ ((__warn_unused_result__))`.  */
macro_rules! __attribute_warn_unused_result__ {
  () => {};
}
/* libc-config.h 139/144 undefine and re-define this for the non-glibc build; the frozen
   target's definition is the empty one.  */
macro_rules! attribute_compat_text_section {
  () => {};
}
