// femacs: port of lib/pathmax.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  gnulib's `PATH_MAX` fallback: C 18-19 is the include guard,
// C 42-45 the "config.h first" check, C 47/C 49 the two includes (`<unistd.h>`, `<limits.h>`),
// C 51-53 the `_POSIX_PATH_MAX` default, and C 55-86 four arms that try to *supply* `PATH_MAX`
// when the system has none (`<sys/param.h>`'s `MAXPATHLEN`, HP-UX's 1023-that-is-really-1024, and
// mingw's 260).  The file's own comment (C 21-40) is the contract: it does *not* define
// `PATH_MAX` always, and a program that needs a bound writes its own fallback.
//
// **On this target every `PATH_MAX` arm is dead, because the name is already defined.**  The
// frozen macro table records `PATH_MAX 1024` (the system `<limits.h>`, which C 49 includes), so:
//
//   * C 55-58's `#if defined HAVE_SYS_PARAM_H && !defined PATH_MAX && !defined MAXPATHLEN` is
//     false (HAVE_SYS_PARAM_H is 1, PATH_MAX is defined) - the `<sys/param.h>` include is not
//     taken, and `MAXPATHLEN` reaches the port from `lib/include/sys_mount.rs` 45 instead;
//   * C 60-62 (`#if !defined PATH_MAX && defined MAXPATHLEN`) is false for the same reason;
//   * C 64-71 (`#ifdef __hpux`) and C 73-86 (`#if defined _WIN32 && !defined __CYGWIN__`) ask
//     about other systems' compilers entirely.
//
// `tools/check-cond-eval.py lib/pathmax.h` -> `10 conditions, 0 unknown to the evaluator,
// 1 disagreement` (`lib/pathmax.h` is not on docs/baseline/config/cond-eval-verified.txt, so the
// extractor resolves no `#if` in it and P-51's "carry the dead text too" applies to all four
// arms).  **The frozen `PATH_MAX` is `lib/include/limits.in.rs` 131's** (`pub const PATH_MAX:
// c_long = 1024;`, with the macro table as its evidence), so this mirror must not become a second
// owner of the name on this configuration - hence the arms below each carry a `cfg` naming the
// configuration they belong to, exactly as `include/msdos.rs` handles C's `EINPROGRESS`/
// `O_CLOEXEC` overrides of names the port already spells.  With the features unset nothing here
// is compiled, and a Windows or HP-UX target picks its own arm.
//
// The three `PATH_MAX` spellings are written in C's *last-definition-first* order (the Windows
// arm, then HP-UX, then the `MAXPATHLEN` alias), which is how `lib/root-uid.h`'s two-armed mirror
// orders its arms (status 3.156).  The gate does not require the order: the extractor reports this
// name as a branch macro ("1 C macros have several definitions (configuration branches), body not
// compared: PATH_MAX"), so all three arms are compared by presence only.

// C 18-19: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const _PATHMAX_H: bool = true;

/// C 42-45: `#if !_GL_CONFIG_H_INCLUDED` / `#error "Please include config.h first."` - C's
/// include-order guard; the port has no preprocessor, so a unit cannot get the order wrong
/// (`config.rs` is the first mirror every unit includes).

/// C 51-53: `#ifndef _POSIX_PATH_MAX` / `# define _POSIX_PATH_MAX 256` - the POSIX minimum, live
/// here (`_POSIX_PATH_MAX` is 256 in the frozen macro table, and no other mirror owns the name).
/// C: # ifndef _POSIX_PATH_MAX
/// C: #  define _POSIX_PATH_MAX 256
/// C: # endif
pub const _POSIX_PATH_MAX: c_int = 256;

/* ---- the four `PATH_MAX` arms, each dead on this target (see the file note) -------------- */

/// C 73-86: `#if defined _WIN32 && ! defined __CYGWIN__` / `# undef PATH_MAX` / `# define
/// PATH_MAX 260` - the mingw arm C 74-83 explains ("the maximum size of a filename, including
/// the terminating NUL byte, is 260 = 3 + 256 + 1 ... mingw's `<limits.h>` gets it wrong").
/// Written first because it is the reference's last definition (see the file note).
/// C: #  undef PATH_MAX
/// C: #  define PATH_MAX 260
#[cfg (feature = "windows")]
pub const PATH_MAX: c_int = 260;

/// C 64-71: `#ifdef __hpux` - "on HP-UX, `PATH_MAX` designates the maximum number of bytes in a
/// filename, *not* including the terminating NUL byte, and is set to 1023", so C redefines it as
/// 1024.  The arm ends with the value C 70 spells.
/// C: #  undef PATH_MAX
/// C: #  define PATH_MAX 1024
#[cfg (feature = "hpux")]
pub const PATH_MAX: c_int = 1024;

/// C 55-62: the two `MAXPATHLEN` arms - `# include <sys/param.h>` (C 55-58) and `# define
/// PATH_MAX MAXPATHLEN` (C 60-62).  Both ask for `!defined PATH_MAX`, which is false here; the
/// second is spelled with C's own alias, `MAXPATHLEN` (`lib/include/sys_mount.rs` 45 owns the
/// name for the frozen target).
/// C: # if defined HAVE_SYS_PARAM_H && !defined PATH_MAX && !defined MAXPATHLEN
/// C: #  include <sys/param.h>
/// C: # endif
/// C: # if !defined PATH_MAX && defined MAXPATHLEN
/// C: #  define PATH_MAX MAXPATHLEN
/// C: # endif
#[cfg (feature = "no_path_max")]
pub const PATH_MAX: c_int = MAXPATHLEN;
