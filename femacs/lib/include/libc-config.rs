// femacs: mirror of lib/libc-config.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The file exists so that code shared with glibc can include it
// as its first header: on this target it pulls `<config.h>` and `<errno.h>` (both are the units'
// own includes) and then defines the handful of glibc-internal spellings that gnulib's own
// `<cdefs.h>` and the SDK's headers do not supply.  The arms taken here are measured and named
// per item below.  Nothing in it needs a name from an includer, so it compiles as its own
// translation unit (`build-tu lib/libc-config.h`), which is the shape the checks below rely on.
//
// Two C arms have no Rust item and are recorded in docs/ports/excluded-macros.txt: `__inline`
// (C's `inline` keyword) and the `compat_symbol`/`versioned_symbol` pair (C's body is
// `extern int dummy`, a declaration glibc's compat machinery needs and the port has nothing for).

/// C 89: `# define _FEATURES_H 1` - the marker `<cdefs.h>` checks before it includes
/// `<features.h>`.
pub const _FEATURES_H: core::ffi::c_int = 1;

/// C 93: `# define __GNULIB_CDEFS`, whose body is empty - a marker that tells `<cdefs.h>` which
/// build it is in.
pub const __GNULIB_CDEFS: bool = true;

/// C 46: `# define __set_errno(val) (errno = (val))` - the fallback arm, live here because this
/// target's `<errno.h>` has no `__set_errno`.  `errno ()` is the port's accessor for the same
/// object (`lib/include/errno.rs`).
macro_rules! __set_errno {
  ($val:expr) => { *errno () = ($val) };
}

/// C 60: `# define __GNUC_PREREQ(maj, min) 0` - the arm no compiler here reaches (the
/// `defined __clang__` arm above it spells `((maj) < 4 + ((min) <= 2))`, and a predicate has no
/// use in the port: `lib/md5.rs` 53 carries the same spelling for the one unit that asks).
macro_rules! __GNUC_PREREQ {
  ($maj:expr, $min:expr) => { 0 };
}

/// C 78: `# define __glibc_clang_prereq(maj, min) 0` - the fallback arm of the same kind.
macro_rules! __glibc_clang_prereq {
  ($maj:expr, $min:expr) => { 0 };
}

/// C 207: `# define SHLIB_COMPAT(lib, introduced, obsoleted) 0` - gnulib's substitute for
/// glibc's compatibility test always says "no".
macro_rules! SHLIB_COMPAT {
  ($lib:expr, $introduced:expr, $obsoleted:expr) => { 0 };
}

/// C 198-204: the six glibc-internal markers, all with empty bodies in gnulib's substitute
/// (`attribute_hidden`, `libc_hidden_proto`, `libc_hidden_def`, `libc_hidden_weak`,
/// `libc_hidden_ver`, `strong_alias`, `weak_alias`).  They are no-ops here; the symbol visibility
/// they would control is the reference build's business, not the port's.
macro_rules! attribute_hidden { () => {} }
macro_rules! libc_hidden_proto { ($name:ident) => {} }
macro_rules! libc_hidden_def { ($name:ident) => {} }
macro_rules! libc_hidden_weak { ($name:ident) => {} }
macro_rules! libc_hidden_ver { ($local:ident, $name:ident) => {} }
macro_rules! strong_alias { ($name:ident, $aliasname:ident) => {} }
macro_rules! weak_alias { ($name:ident, $aliasname:ident) => {} }

/// C 190: `# define __inline` with an empty body (this target defines `HAVE___INLINE`, so the
/// file's `# define __inline inline` arm is skipped and the marker it leaves behind is what C
/// code tests).
pub const __inline: bool = true;
