// femacs: mirror of nt/inc/stdint.h @ fd7afced403.
//
// The Windows port's `<stdint.h>` shim.  GCC has the header and only needs the include passed
// on, so on the MinGW build this shim serves - `__GNUC__` is what MinGW is - the whole file is
// one `#include_next`.  The `!__GNUC__` block below it is the MSVC case: the minimum set of
// integer typedefs and limits for a tool chain without `<stdint.h>`, in two arms (`_WIN64` and
// 32-bit).  The file is not on `docs/baseline/config/cond-eval-verified.txt`, so the extractor
// reads every arm as live text, and nothing in the frozen configuration includes it (no `nt/`
// translation unit is compiled - docs/baseline/frozen-build-sources.txt's batch-F section - and
// no `src/` unit includes a `nt/inc/` header), so `xtask layout-check` never compiles this
// mirror; in a MinGW build `src/lisp.h`, `src/itree.h`, `src/sfnt.c` and `lib-src/hexl.c` are
// among the units whose `#include <stdint.h>` resolves here.
//
// The two arms define the same names with different widths; where a name exists in both, the
// mirror spells the arm C keeps (the 32-bit one, which is what the extractor records - `intmax_t`
// at line 63, `uintmax_t` at line 62) and records the other in a `///` note; the names only the
// `_WIN64` arm has are marked `#[cfg (feature = "win64")]` the way `msdos/inttypes.rs` marks its
// dead arm.  `intmax_t`/`uintmax_t` are `#define`s rather than typedefs, so they also have rows
// in `docs/ports/excluded-macros.txt` (the `file_offset` type-rename shape).

// C 21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _NT_STDINT_H_: bool = true;

// C 23-24: `#ifdef __GNUC__ / # include_next <stdint.h>` - the arm a MinGW build takes; the
// compiler's own header, which has no mirror to name here.

/// C 29 and 49: `intptr_t` - the `_WIN64` arm (`typedef __int64 intptr_t;`) and the 32-bit arm
/// (`typedef int intptr_t;`); C keeps the latter, which is the one the mirror spells, and the
/// extractor records it the same way.
pub type intptr_t = c_int;
/// C 30 and 50: `uint32_t` - `typedef unsigned int uint32_t;`, the same in both arms.
pub type uint32_t = c_uint;
/// C 31: `typedef unsigned __int64 uint64_t;` - only the `_WIN64` arm declares it.
#[cfg (feature = "win64")]
pub type uint64_t = c_ulonglong;

/// C 62: `#define uintmax_t unsigned long` (32-bit) / `unsigned __int64` (`_WIN64`) - a type
/// rename, so the port spells it as an alias; the row is in `docs/ports/excluded-macros.txt`.
pub type uintmax_t = c_ulong;
/// C 63: `#define intmax_t long` (32-bit) / `__int64` (`_WIN64`) - same as `uintmax_t`.
pub type intmax_t = c_long;

// C 51-52: the 32-bit arm's `UINT32_MAX`/`UINT32_MIN`.
pub const UINT32_MAX: c_uint = 4294967295;
pub const UINT32_MIN: c_uint = 0;
// C 53-54: `INT32_MAX` and `INT32_MIN` - the latter is C's `(~INT32_MAX)`, and Rust's bitwise
// not is the same operation on an integer.
pub const INT32_MAX: c_int = 2147483647;
pub const INT32_MIN: c_int = !INT32_MAX;

// C 34-35: the `_WIN64` arm's `UINT64_MAX`/`UINT64_MIN` (C's value carries MSVC's `i64` suffix,
// which Rust has no spelling for; the numeric value is the same).
#[cfg (feature = "win64")]
pub const UINT64_MAX: c_ulonglong = 18446744073709551615;
#[cfg (feature = "win64")]
pub const UINT64_MIN: c_ulonglong = 0;
// C 37-38: the `_WIN64` arm's `INT64_MAX`/`INT64_MIN`.
#[cfg (feature = "win64")]
pub const INT64_MAX: c_longlong = 9223372036854775807;
#[cfg (feature = "win64")]
pub const INT64_MIN: c_longlong = !INT64_MAX;

// C 55-56: `#define INTPTR_MAX INT32_MAX` / `INTPTR_MIN INT32_MIN` (the 32-bit arm; the
// `_WIN64` arm says `INT64_MAX`/`INT64_MIN`).
pub const INTPTR_MAX: intptr_t = INT32_MAX;
pub const INTPTR_MIN: intptr_t = INT32_MIN;
// C 57: `#define UINTPTR_MAX UINT32_MAX` (the `_WIN64` arm says `UINT64_MAX`).
pub const UINTPTR_MAX: c_uint = UINT32_MAX;
// C 58-59: `#define UINTMAX_MAX UINT32_MAX` / `UINTMAX_MIN UINT32_MIN`.
pub const UINTMAX_MAX: uintmax_t = UINT32_MAX;
pub const UINTMAX_MIN: uintmax_t = UINT32_MIN;
// C 60-61: `#define INTMAX_MAX INT32_MAX` / `INTMAX_MIN INT32_MIN`.
pub const INTMAX_MAX: intmax_t = INT32_MAX;
pub const INTMAX_MIN: intmax_t = INT32_MIN;

/// C 66-67: `#define PTRDIFF_MAX INTPTR_MAX` / `PTRDIFF_MIN INTPTR_MIN` - `ptrdiff_t` is
// PORTING.md 1.1's `isize`.
pub const PTRDIFF_MAX: isize = INTPTR_MAX as isize;
pub const PTRDIFF_MIN: isize = INTPTR_MIN as isize;
/// C 68: `#define SIZE_MAX UINTPTR_MAX` - `size_t` is PORTING.md 1.1's `usize`.
pub const SIZE_MAX: usize = UINTPTR_MAX as usize;
