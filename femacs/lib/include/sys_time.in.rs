// femacs: port of lib/sys_time.in.h @ fd7afced403 (gnulib's <sys/time.h>; PORTING.md P-26/P-66).
//
// **The criterion is the generated header, and it exists.**  `build/emacs-ref/lib/sys/time.h`
// (33,335 bytes) is produced from this template by `gnulib.mk`'s rule, and every `@NAME@` in it
// has become a literal number there - which is what decides each arm:
//
//     $ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/sys_time.in.h) build/emacs-ref/lib/sys/time.h
//     (the bulk of it is the `c++defs.h` / `arg-nonnull.h` / `warn-on-use.h` text the build
//      inlines at C 70/72/74; those definitions belong to their own mirrors, not to this file)
//
// The substitution table the frozen build actually wrote (line numbers are the generated
// header's, read with `awk 'NR>=..&&NR<=..'`):
//
//   | C   | `@NAME@`                  | generated value                | arm it selects |
//   | --- | --- | --- | --- |
//   | 20  | `@GUARD_PREFIX@`          | `GL` (21)                      | the guard is `_GL_SYS_TIME_H` |
//   | 23  | `@PRAGMA_SYSTEM_HEADER@`  | `#pragma GCC system_header` (24) | C 22's `__GNUC__ >= 3` arm is live |
//   | 25  | `@PRAGMA_COLUMNS@`        | empty (26-27)                  | nothing to add |
//   | 40  | `@INCLUDE_NEXT@ @NEXT_SYS_TIME_H@` | `# include_next <sys/time.h>` (41, 46) | the system half |
//   | 44  | `@HAVE_SYS_TIME_H@`       | `1` (45)                       | the include_next is live, C 56-58's `<time.h>` fallback is dead |
//   | 64  | `@REPLACE_STRUCT_TIMEVAL@`| `0` (65)                       | dead together with `_MSC_VER` |
//   | 64  | `@HAVE_WINSOCK2_H@`       | `0` (65)                       | the whole winsock2 block is dead |
//   | 80  | `@HAVE_STRUCT_TIMEVAL@`   | `1` (627)                      | C 87-92's own `struct timeval` is **dead** |
//   | 80  | `@REPLACE_STRUCT_TIMEVAL@`| `0` (627)                      | C 83's `#define timeval rpl_timeval` is dead |
//   | 101 | `@GNULIB_GETTIMEOFDAY@`   | `1` (648)                      | this module is in the build |
//   | 102 | `@REPLACE_GETTIMEOFDAY@`  | `0` (649)                      | the **SYS** arm (C 113-122) is live, the RPL arm is dead |
//   | 113 | `@HAVE_GETTIMEOFDAY@`     | `1` (660)                      | `_GL_FUNCDECL_SYS` is not needed either |
//   | 129 | `@REPLACE_STRUCT_TIMEVAL@`| `0` (676)                      | the C++ namespace typedef is dead |
//   | 145/161/222 | `@GUARD_PREFIX@`    | `GL`                           | `_GL_UNISTD_H` / `_GL_SYS_SOCKET_H` / `_GL_SYS_SELECT_H` |
//
// `python3 tools/check-cond-eval.py lib/sys_time.in.h` -> `31 conditions, 1 unknown, 2
// disagreements [NOT VERIFIED]`, so this file is **blind-read** (P-51/P-66): both arms of every
// `#if` are carried, the dead one behind the feature that names the non-frozen side, and the
// live one is the measurement above - not the evaluator's reading.  The two disagreements:
//
//   | line | condition | clang | evaluator | why the table cannot decide |
//   | --- | --- | --- | --- | --- |
//   | 30 | `#if !_GL_CONFIG_H_INCLUDED` | False | True | `_GL_CONFIG_H_INCLUDED` is config.h's own marker, not a configuration macro |
//   | 39 | `#if defined _CYGWIN_SYS_TIME_H \|\| defined _SYS_TIME_H \|\| defined _SYS_TIME_H_` | False | True | the operand is the *system* `<sys/time.h>`'s guard, which the table does not carry |
//
// **The system half is `sys_time.rs`** (P-26/P-50: the frozen target's `<sys/time.h>`, which has
// no reference file of its own), reached through C 40/45's `#include_next`.  On this target the
// system header *is* what provides `struct timeval` and `gettimeofday` - measured, the generated
// 627 reads `#if !1 || 0` - so the port's live `struct timeval` (with its measured layout
// assertions) lives there, and the copy below stays behind the feature that names the side where
// gnulib has to define it itself.
//
// The gate demands 21 macros, 1 constant and the type `timeval` (the union of every arm, P-66);
// the names below are C's own, in C's order, and the `_GL_*` calls this file makes
// (`_GL_FUNCDECL_SYS` / `_GL_CXXALIAS_SYS_CAST` / `_GL_CXXALIASWARN` / `_GL_WARN_ON_USE`) are the
// `c++defs.h` / `arg-nonnull.h` / `warn-on-use.h` mirrors' business.

/* C 40/45: `# @INCLUDE_NEXT@ @NEXT_SYS_TIME_H@` is the system half, `sys_time.rs` (which the
   real consumers include; measured, `include!("sys_time.rs")` here cannot stand alone - it takes
   `time_t`/`c_int` from its own includer, exactly as every lib mirror takes its scalars).  */

/* C 20/49: `#ifndef _GL_SYS_TIME_H` / `#define _GL_SYS_TIME_H` - the guard has no Rust
   counterpart (PORTING.md 0.1-4); the name stays as its marker.  */
pub const _GL_SYS_TIME_H: bool = true;

/* C 22-25: `#if __GNUC__ >= 3` / `@PRAGMA_SYSTEM_HEADER@` / `@PRAGMA_COLUMNS@` - a `#pragma` has
   no Rust item; measured, the generated 24 carries `#pragma GCC system_header` and the
   `@PRAGMA_COLUMNS@` line is empty.  */

/* C 30-32 and C 52-54: `#if !_GL_CONFIG_H_INCLUDED` / `#error "Please include config.h first."` -
   `_GL_CONFIG_H_INCLUDED` is the generated `<config.h>`'s own marker (measured: clang False, so
   both errors are dead and this file is reached with config.h already included).  */

/* C 39-41: "On Cygwin and on many BSDish systems, <sys/time.h> includes itself recursively via
   <sys/select.h>.  Simply delegate to the system's header in this case" - the condition names the
   *system* header's guards, clang measured it False, so C 42's `#else` (this file's real body) is
   the live arm and the `# include_next` at C 40 is dead.  */

/* C 56-58: `#if ! @HAVE_SYS_TIME_H@` / `# include <time.h>` - measured `#if ! 1`, dead: the
   target has `<sys/time.h>`, so C 45's `#include_next` is the one that runs.  */

/* C 64-68: the winsock2 block.  Measured `#if (defined _MSC_VER || 0) && 0 && ...`, dead on this
   target; the name it defines is C's own (P-51: the dead arm is carried behind the feature that
   names the side where it is live).  */
#[cfg (feature = "windowsnt")]
macro_rules! _GL_INCLUDING_WINSOCK2_H { () => { 1 }; }

/* C 80-95: gnulib's own `struct timeval`.  Measured `#if !1 || 0` (the generated 627), i.e. dead:
   on this target the type comes from the system header - `sys_time.rs`, whose layout assertions
   are the port's live copy - and this text is the side where gnulib defines it itself.  */
#[cfg (any (feature = "no_have_struct_timeval", feature = "replace_struct_timeval"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct timeval
{
  pub tv_sec: time_t,
  pub tv_usec: c_long,
}

/* C 92: `#  define GNULIB_defined_struct_timeval 1` (C body is the literal `1`).  */
#[cfg (any (feature = "no_have_struct_timeval", feature = "replace_struct_timeval"))]
pub const GNULIB_defined_struct_timeval: core::ffi::c_int = 1;

/* C 82-84: `# if @REPLACE_STRUCT_TIMEVAL@` / `#  define timeval rpl_timeval` - measured `# if 0`,
   dead (this is the macro half of the type: C renames the system type to gnulib's replacement). */
#[cfg (feature = "replace_struct_timeval")]
macro_rules! timeval { () => { rpl_timeval }; }

/* C 101-140: `gettimeofday`.  Measured `#if 1` / `# if 0` (generated 648-649), so the live arm is
   C 113-122's SYS half: `@HAVE_GETTIMEOFDAY@` is 1 (the generated 660 reads `#  if !1`), which is
   why neither `_GL_FUNCDECL_SYS` nor the `rpl_gettimeofday` rename is carried as live.

   /// C 114-116: `_GL_FUNCDECL_SYS (gettimeofday, int, (struct timeval *restrict, void *restrict), );`
   /// (dead: `! @HAVE_GETTIMEOFDAY@` is false - the system declares it already)
   /// C 120-121: `_GL_CXXALIAS_SYS_CAST (gettimeofday, int, (struct timeval *restrict, void *restrict));`
   /// C 123: `_GL_CXXALIASWARN (gettimeofday);`  */
unsafe extern "C" {
  pub fn gettimeofday (tv: *mut timeval, tz: *mut c_void) -> c_int;
}

/* C 102-112: the replacement arm, dead (measured `# if 0`): `# undef gettimeofday` /
   `#  define gettimeofday rpl_gettimeofday` plus `_GL_FUNCDECL_RPL` / `_GL_CXXALIAS_RPL`.  */
#[cfg (feature = "replace_gettimeofday")]
macro_rules! gettimeofday { () => { rpl_gettimeofday }; }

/* C 124-134: the C++ namespace typedef (`namespace GNULIB_NAMESPACE { typedef ::timeval ... }`) -
   C++-only text with no Rust item; measured `#  if 0` at the generated 676, so no rename either. */

/* C 135-140: `#elif defined GNULIB_POSIXCHECK` / `# if HAVE_RAW_DECL_GETTIMEOFDAY` /
   `_GL_WARN_ON_USE (gettimeofday, "gettimeofday is unportable - …")` - dead (`@GNULIB_GETTIMEOFDAY@`
   is 1, so the `#if` arm above was taken and this `#elif` is not reached).  */

/* C 142-231: "Hide some function declarations from <winsock2.h>."  Measured
   `#if defined _MSC_VER && 0` (generated 691) - the whole block is dead on this target.  Its names
   are C's own and are the ones the gate asks for, each renaming the function to the
   `*_used_without_including_*` spelling the C text gives it.  */
#[cfg (feature = "windowsnt")]
macro_rules! close { () => { close_used_without_including_unistd_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! gethostname { () => { gethostname_used_without_including_unistd_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! socket { () => { socket_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! connect { () => { connect_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! accept { () => { accept_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! bind { () => { bind_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! getpeername { () => { getpeername_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! getsockname { () => { getsockname_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! getsockopt { () => { getsockopt_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! listen { () => { listen_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! recv { () => { recv_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! send { () => { send_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! recvfrom { () => { recvfrom_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! sendto { () => { sendto_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! setsockopt { () => { setsockopt_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! shutdown { () => { shutdown_used_without_including_sys_socket_h }; }
#[cfg (feature = "windowsnt")]
macro_rules! select { () => { select_used_without_including_sys_select_h }; }

/* C 233-235: the three closing `#endif`s of the split double-inclusion guard.  */
