// femacs: port of lib/sys_select.in.h @ fd7afced403 (gnulib's <sys/select.h>; PORTING.md P-26/P-66).
//
// **The criterion is the generated header, and it exists.**  `build/emacs-ref/lib/sys/select.h`
// (36,552 bytes) is what the build writes from this template; every `@NAME@` has become a literal
// number there:
//
//     $ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/sys_select.in.h) build/emacs-ref/lib/sys/select.h
//
// The substitution table the frozen build actually wrote (generated-header line numbers):
//
//   | C   | `@NAME@`                        | value | arm it selects |
//   | --- | --- | --- | --- |
//   | 18  | `@PRAGMA_SYSTEM_HEADER@`        | `#pragma GCC system_header` | C 17's `__GNUC__ >= 3` arm live |
//   | 20  | `@PRAGMA_COLUMNS@`              | empty | nothing |
//   | 32  | `@HAVE_SYS_SELECT_H@`           | `1` (33) | the redirect chain is reachable |
//   | 39  | `@INCLUDE_NEXT@ @NEXT_SYS_SELECT_H@` | `# include_next <sys/select.h>` (40, 52, 61, 88) | system half |
//   | 41  | `@HAVE_SYS_SELECT_H@`           | `1` (42) | the second redirect arm is reachable |
//   | 58  | `@HAVE_SYS_SELECT_H@`           | `1` (59) | the OpenBSD pthread arm is reachable but its guards are not |
//   | 76  | `@HAVE_SYS_SELECT_H@`           | `1` (77) | the real body's include_next is live |
//   | 110 | `@HAVE_SYS_SELECT_H@`           | `0` after `!` (111: `#if !1`) | C 112-144 is **dead** |
//   | 134 | `@HAVE_WINSOCK2_H@`             | `0` | the winsock2 includes inside the dead arm stay dead |
//   | 154 | `@HAVE_WINSOCK2_H@`             | `0` (661: `#if 0`) | `suseconds_t`, `rpl_fd_isset` and the `FD_ISSET` override are **dead** |
//   | 176 | `@HAVE_WINSOCK2_H@`             | `0` (696: `#if 0`) | the 18 `*_used_without_including_*` renames are dead |
//   | 256 | `@GNULIB_PSELECT@`              | `1` (777) | `pselect` is in the build |
//   | 257 | `@REPLACE_PSELECT@`             | `0` (778) | the **SYS** arm (C 268-288) is live |
//   | 269 | `@HAVE_PSELECT@`                | `1` (790) | `_GL_FUNCDECL_SYS` is not needed |
//   | 292 | `@GNULIB_SELECT@`               | `0` (813: `#if 0`) | the whole `select` block is **dead** |
//   | 293 | `@REPLACE_SELECT@`              | `0` (814) | (inside the dead block) |
//
// `python3 tools/check-cond-eval.py lib/sys_select.in.h` -> `40 conditions, 18 unknown, 29
// disagreements [NOT VERIFIED]`, so this unit is **blind-read** (P-51/P-66) and both arms of every
// `#if` are carried.  The twelve disagreements that decide structure:
//
//   | line | condition | clang | evaluator | live arm |
//   | --- | --- | --- | --- | --- |
//   | 32 | `(@HAVE_SYS_SELECT_H@ && !defined _GL_SYS_SELECT_H_REDIRECT_FROM_SYS_TYPES_H && (defined __sun && …))` | False | True | dead (`__sun` is not this target) |
//   | 41 | `(@HAVE_SYS_SELECT_H@ && (defined _CYGWIN_SYS_TIME_H \|\| …))` | False | True | dead |
//   | 58 | `@HAVE_SYS_SELECT_H@ && defined __OpenBSD__ && …` | False | True | dead |
//   | 103 | `!((defined __GLIBC__ \|\| defined __CYGWIN__ \|\| defined __KLIBC__) && !defined __UCLIBC__)` | True | True | **live**: `<signal.h>` is included, C 104 |
//   | 154 | `@HAVE_WINSOCK2_H@` | False | True | dead |
//   | 176/192 | `@HAVE_WINSOCK2_H@` / `!defined _GL_SYS_SOCKET_H` | False | True | dead |
//   | 313 | `!GNULIB_SELECT` | False | True | dead (it is the `#elif` of a dead `#if`) |
//   ... (the rest are the same shape: `__sun`/`__OpenBSD__`/`_MSC_VER` operand families)
//
// **The system half is `sys_select.rs`** (P-26/P-50: the frozen target's `<sys/select.h>`, no
// reference file; it carries `fd_set` and the three `FD_*` accessors).  The template reaches it at
// C 39/51/60/87 (`#include_next`) and at C 75 (`<sys/types.h>`, `sys_types.in.rs`); C 103-105's
// `<signal.h>` is live and is `signal.in.rs`/`signal.rs`'s business; `<sys/time.h>` (`sys_time.rs`)
// is reached at C 84 inside a live include.
//
// The gate demands 24 macros, the two constants `GNULIB_defined_suseconds_t` and
// `GNULIB_defined_rpl_fd_isset`, and the function `rpl_fd_isset` - the union of every arm (P-66).
// The `_GL_*` calls this file makes (`_GL_FUNCDECL_SYS` / `_GL_CXXALIAS_SYS_CAST` / `_GL_CXXALIASWARN`
// / `_GL_WARN_ON_USE`) are the `c++defs.h` / `arg-nonnull.h` / `warn-on-use.h` mirrors' business.

/* C 17-20: `# if __GNUC__ >= 3` / `@PRAGMA_SYSTEM_HEADER@` / `@PRAGMA_COLUMNS@` - a `#pragma` has
   no Rust item; measured, the generated 24 carries `#pragma GCC system_header`.  */

/* C 26-27 and C 68-69: `#if !_GL_CONFIG_H_INCLUDED` / `#error "Please include config.h first."` -
   `_GL_CONFIG_H_INCLUDED` is the generated `<config.h>`'s marker, so both errors are dead.  */

/* C 32-39: the first redirect arm - "On Solaris 2.6, <sys/types.h> and <sys/time.h> both include
   <sys/select.h>".  Measured dead (its `__sun` operands are not this target's), so C 62's `#else`
   is the live arm; the name it defines is C's own and is carried here.  */
#[cfg (feature = "sun")]
pub const _GL_SYS_SELECT_H_REDIRECT_FROM_SYS_TYPES_H: bool = true;

/* C 41-51: the second redirect arm (`_CYGWIN_SYS_TIME_H`, `__OpenBSD__`, `__sun`).  Measured dead. */
#[cfg (any (feature = "cygwin", feature = "openbsd", feature = "sun"))]
pub const _GL_SYS_SELECT_H_REDIRECT_FROM_SYS_TIME_H: bool = true;

/* C 58-60: the OpenBSD 5.0 `_PTHREAD_H_` arm - measured dead ("`defined __OpenBSD__`" is false).  */

/* C 64/107-108: `#ifndef _GL_SYS_SELECT_H` / `#define _GL_SYS_SELECT_H` - the split
   double-inclusion guard has no Rust counterpart (PORTING.md 0.1-4); the name stays as its marker. */
pub const _GL_SYS_SELECT_H: bool = true;

/* C 75: `#include <sys/types.h>` - `sys_types.in.rs`; C 77-77's `#if @HAVE_SYS_SELECT_H@` (measured
   `1`) makes C 87's `#include_next` (the system half, `sys_select.rs`) and C 104's `<signal.h>`
   (`signal.in.rs`) live.  Both are includes, so they have no Rust item of their own here.  */

/* C 110-144: `#if !@HAVE_SYS_SELECT_H@` (measured `#if !1`) - dead on this target: the
   `<sys/time.h>` / `<string.h>` / `<winsock2.h>` / `<io.h>` includes and the `_GL_INCLUDING_WINSOCK2_H`
   marker are the side where the platform has no `<sys/select.h>` at all.  */
#[cfg (feature = "no_have_sys_select_h")]
macro_rules! _GL_INCLUDING_WINSOCK2_H { () => { 1 }; }

/* C 146-152: "The definitions of _GL_FUNCDECL_RPL etc. are copied here." / "The definition of
   _GL_WARN_ON_USE is copied here." - those definitions live in their own mirrors.  */

/* C 154-174: `#if @HAVE_WINSOCK2_H@` (measured `#if 0`) - dead.  C's own text: the `suseconds_t`
   typedef, then `rpl_fd_isset` with its body, then the `FD_ISSET` override.  */
#[cfg (feature = "windowsnt")]
pub type suseconds_t = c_int;

/* C 158-159: `# if !GNULIB_defined_suseconds_t` / `#  define GNULIB_defined_suseconds_t 1`.  */
#[cfg (feature = "windowsnt")]
pub const GNULIB_defined_suseconds_t: core::ffi::c_int = 1;

/* C 161-168: "Re-define FD_ISSET to avoid a WSA call while we are not using network sockets."
   `static int rpl_fd_isset (SOCKET fd, const fd_set * set)` - the body is C's, token for token. */
#[cfg (feature = "windowsnt")]
pub const GNULIB_defined_rpl_fd_isset: core::ffi::c_int = 1;

#[cfg (feature = "windowsnt")]
unsafe fn rpl_fd_isset (fd: SOCKET, set: *const fd_set) -> c_int
{
  if set == NULL
    { return 0; }

  let mut i: u_int = 0;
  while i < (*set).fd_count
    {
      if (*set).fd_array[i as usize] == fd
        { return 1; }
      i += 1;
    }

  return 0;
}

/* C 170-171: `# undef FD_ISSET` / `# define FD_ISSET(fd, set) rpl_fd_isset(fd, set)`.  */
#[cfg (feature = "windowsnt")]
macro_rules! FD_ISSET { ($fd:expr, $set:expr) => { rpl_fd_isset ($fd, $set) }; }

/* C 176-254: "Hide some function declarations from <winsock2.h>."  Measured `#if 0` (generated
   696) - dead; the names are C's own, each renaming the function to the
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

/* C 256-290: `pselect`.  Measured `#if 1` / `# if 0` (generated 777-778), so C 268-288's SYS arm is
   live and `@HAVE_PSELECT@` is 1 (generated 790 reads `#  if !1`): the system declares it, C renames
   nothing, and the only live text is C 275-288's alias.

   /// C 270-273: `_GL_FUNCDECL_SYS (pselect, int, (int, fd_set *restrict, fd_set *restrict,
   ///             fd_set *restrict, struct timespec const *restrict, const sigset_t *restrict), );`
   ///             (dead: `! @HAVE_PSELECT@` is false)
   /// C 279: `_GL_CXXALIASWARN (pselect);` (inside `# if __GLIBC__ >= 2`, dead on this target)  */
unsafe extern "C" {
  pub fn pselect (nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set,
                  exceptfds: *mut fd_set, timeout: *const timespec,
                  sigmask: *const sigset_t) -> c_int;
}

/* C 257-266: the replacement arm, dead (measured `# if 0`): `# undef pselect` /
   `# define pselect rpl_pselect` plus `_GL_FUNCDECL_RPL` / `_GL_CXXALIAS_RPL`.  */
#[cfg (feature = "replace_pselect")]
macro_rules! pselect { () => { rpl_pselect }; }

/* C 291-292: `#elif defined GNULIB_POSIXCHECK` / `# if HAVE_RAW_DECL_PSELECT` /
   `_GL_WARN_ON_USE (pselect, "pselect is not portable - " …)` - dead (`@GNULIB_PSELECT@` is 1).  */

/* C 292-306: `select`.  Measured `#if 0` (generated 813), i.e. `@GNULIB_SELECT@` is 0: the module is
   not in this build, so every arm is dead - the `rpl_select` rename, the SYS alias, and the
   `#elif @HAVE_WINSOCK2_H@` arm's `select_used_without_requesting_gnulib_module_select`.  */
#[cfg (feature = "gnulib_select")]
macro_rules! select { () => { rpl_select }; }
#[cfg (feature = "windowsnt")]
macro_rules! select_used_without_requesting_gnulib_module_select { () => { 1 }; }

/* C 308-311: `#elif @HAVE_WINSOCK2_H@` / `# if !GNULIB_SELECT` / `#  undef select` /
   `#  define select select_used_without_requesting_gnulib_module_select` - measured `#elif 0`
   (generated 833) and `GNULIB_SELECT` is undefined, so dead; C 312-317's
   `GNULIB_POSIXCHECK` arm (`_GL_WARN_ON_USE (select, "select is not always POSIX compliant - …")`)
   is dead with it.  */

/* C 319-341: the closing guards, then "Includes that provide only macros that don't need to be
   overridden" - C 334-339's `<string.h>` include for OpenBSD / AIX / Solaris / BeOS is measured
   dead (none of those, and `__GLIBC__` is the wrong kind of operand for the table anyway).  */
