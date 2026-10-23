// femacs: mirror of nt/inc/stdalign.h @ fd7afced403.
//
// The Windows port's `<stdalign.h>` shim, for the MSVC tool chain (GCC has the header).
// Rust aligns with the `#[repr(align(N))]` attribute and measures with
// `core::mem::align_of::<T> ()`.  The file is not on
// `docs/baseline/config/cond-eval-verified.txt` and has no `#if` that changes what it declares,
// so the whole file is live text.  Nothing in the frozen configuration includes it (no `nt/`
// translation unit is compiled - docs/baseline/frozen-build-sources.txt's batch-F section - and
// no `src/` unit includes a `nt/inc/` header), so `xtask layout-check` never compiles this
// mirror; in a MinGW build `nt/inc` is on the include path for a unit that includes
// `<stdalign.h>`.
//
// `#include <stddef.h>` (C 4) is where C's `offsetof` comes from; the port's units write
// `core::mem::offset_of!`, and the name has no mirror here.

// C 2: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _NT_STDALIGN_H_: bool = true;

/// C 7 and 9: `# define _Alignof(type) …` - two arms (a C++ template helper, then C's
/// `offsetof (struct { char __a; type __b; }, __b)` for C); C keeps the second, and Rust's
/// counterpart for both is `core::mem::align_of::<T> ()`.
macro_rules! _Alignof { ($type:ty) => { core::mem::align_of::<$type> () }; }

/// C 11: `#define alignof _Alignof`
macro_rules! alignof { ($type:ty) => { _Alignof! ($type) }; }

// C 14: `# define _Alignas(a) __attribute__ ((__aligned__ (a)))` and C 17:
// `# define alignas _Alignas`.  Both are recorded in `docs/ports/excluded-macros.txt` (the
// `alignas` row has been there since an earlier round): the Rust counterpart is the
// `#[repr(align(N))]` attribute, which a `macro_rules!` cannot produce, so neither name is
// declared here.
