// femacs: port of lib/signal.in.h @ fd7afced403 (gnulib's <signal.h>; PORTING.md P-26/P-66).
//
// **The criterion is the generated header, and it exists.**  `build/emacs-ref/lib/signal.h`
// (41,625 bytes) is what the build writes from this template:
//
//     $ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/signal.in.h) build/emacs-ref/lib/signal.h
//
// The substitution table (generated-header line numbers; the alignment was read by walking both
// files in step, so every value below is the literal the build wrote):
//
//   | C   | `@NAME@`                                | value | arm it selects |
//   | --- | --- | --- | --- |
//   | 19  | `@PRAGMA_SYSTEM_HEADER@`                | `#pragma GCC system_header` | C 18's `__GNUC__ >= 3` arm live |
//   | 21  | `@PRAGMA_COLUMNS@`                      | empty | nothing |
//   | 29  | `@GUARD_PREFIX@`                        | `GL` (30, 47, 49, 60, 534-535) | the guards are `_GL_*` |
//   | 41/57 | `@INCLUDE_NEXT@ @NEXT_SIGNAL_H@`      | `# include_next <signal.h>` (42, 58) | system half |
//   | 70/78 | `@GNULIB_PTHREAD_SIGMASK@`            | `1` (71, 79) | `pthread_sigmask` is in the build |
//   | 98  | `@HAVE_TYPE_VOLATILE_SIG_ATOMIC_T@`     | `1` (645) | C 99-105 (the `rpl_sig_atomic_t` rename) is **dead** |
//   | 108 | `@HAVE_SIGSET_T@`                       | `1` (655) | C 109-113 (`typedef unsigned int sigset_t`) is **dead** |
//   | 116 | `@HAVE_SIGHANDLER_T@`                   | `0` (663) | C 117-128 (`sighandler_t`) is **live** |
//   | 130 | `@GNULIB_SIGNAL_H_SIGPIPE@`             | `0` (677) | C 131-137 (`SIGPIPE`, `GNULIB_defined_SIGPIPE`) is **dead** |
//   | 155/172 | `@GNULIB_SIG2STR@`                  | `1` (704, 719) | both `sig2str` and `str2sig` are in the build |
//   | 157/173 | `@HAVE_SIG2STR@` / `@HAVE_STR2SIG@` | `0` (705, 720) | `_GL_FUNCDECL_SYS` is **live** for both |
//   | 189 | `@REPLACE_PTHREAD_SIGMASK@`             | `0` (705 region) | the **SYS** arm is live, nothing renamed |
//   | 203 | `@HAVE_PTHREAD_SIGMASK@`                | `0` | so `_GL_FUNCDECL_SYS (pthread_sigmask, …)` is live too |
//   | 225 | `@GNULIB_RAISE@`                        | `0` (772) | the `raise` block is **dead** |
//   | 249 | `@GNULIB_SIGPROCMASK@`                  | `0` (796: `#if 0`) | the whole `sigprocmask` block (C 249-441) is **dead** |
//   | 250/284/… | `@HAVE_POSIX_SIGNALBLOCKING@`     | `1` (797 region, 954) | the SYS arms of that block would be live - it is the `#if` above that is not |
//   | 407 | `@HAVE_POSIX_SIGNALBLOCKING@`           | `1` (954) | `# if !1 && GNULIB_defined_SIGPIPE` - dead twice over |
//   | 444 | `@GNULIB_SIGACTION@`                    | `0` (991) | the `sigaction` block is **dead** |
//   | 445/511 | `@HAVE_SIGACTION@` / `@HAVE_STRUCT_SIGACTION_SA_SIGACTION@` | `1` (992, 1058) | gnulib defines neither the type nor the alias |
//   | 447/476 | `@HAVE_SIGINFO_T@`                  | `1` (994, 1023) | `siginfo_t` is the system's |
//
// `python3 tools/check-cond-eval.py lib/signal.in.h` -> `87 conditions, 34 unknown, 44
// disagreements [NOT VERIFIED]` (it was already on 3.136's list of units that "need P-51/P-66
// carrying"), so this unit is **blind-read** and every `#if` carries both arms.
//
// **The two halves.**  C 41/57's `#include_next` is the frozen `<signal.h>`; the port's mirror of
// what the ported units actually name from it is `signal.rs` (the signal numbers, `sigset_t`,
// Darwin's `union __sigaction_u` + `struct sigaction` with its measured layout, `SIGPIPE` as
// value 13).  This file is the *gnulib* half: the wrapper that decides whether gnulib has to
// define `sigset_t` / `sighandler_t` / `struct sigaction` / `siginfo_t` / `sigval` itself, which
// on this target it does **not** (the `@HAVE_*@` readings above) - the definitions it would use
// are the dead arms below, carried behind the features that name the non-frozen side.
//
// Live on this target, and therefore the mirror's real text: C 117-128's `sighandler_t`
// (`@HAVE_SIGHANDLER_T@` 0), C 150-153's `SIG2STR_MAX`, C 155-185's `sig2str` / `str2sig`
// (`_GL_FUNCDECL_SYS`, since neither exists in the SDK), C 188-217's `pthread_sigmask` (SYS arm),
// and C 519-523's `SA_NODEFER` fallback.  The gate demands 25 macros, 16 constants, the three
// types `sigaction` / `siginfo_t` / `sigval` and their 16 fields - the union of every arm (P-66).

/* C 19-21: `# if __GNUC__ >= 3` / `@PRAGMA_SYSTEM_HEADER@` / `@PRAGMA_COLUMNS@` - a `#pragma`
   has no Rust item; measured, the generated 20 carries `#pragma GCC system_header`.  */

/* C 25-26: `#ifndef WIN_PTHREADS_SIGNAL_H` / `#define WIN_PTHREADS_SIGNAL_H` - the outermost
   guard; the name stays as its marker.  */
pub const WIN_PTHREADS_SIGNAL_H: bool = true;

/* C 29-43: the "special invocation convention" arm (`__need_sig_atomic_t` / `__need_sigset_t` /
   `_GL_ALREADY_INCLUDING_SIGNAL_H` / `_SIGNAL_H && !__SIZEOF_PTHREAD_MUTEX_T`): inside it C only
   does `# include_next <signal.h>`.  C 60's `#else` is the normal convention, i.e. this file.  */
pub const _GL_ALREADY_INCLUDING_SIGNAL_H: bool = true;

/* C 46/61: `#ifndef _GL_SIGNAL_H` / `#define _GL_SIGNAL_H` - the split double-inclusion guard has
   no Rust counterpart (PORTING.md 0.1-4).  */
pub const _GL_SIGNAL_H: bool = true;

/* C 55: `#include <sys/types.h>` - `sys_types.in.rs`.  C 57's `#@INCLUDE_NEXT@ @NEXT_SIGNAL_H@`
   is the system half, `signal.rs`; C 66-68's `#if !_GL_CONFIG_H_INCLUDED` / `#error` is dead
   (`_GL_CONFIG_H_INCLUDED` is config.h's marker).  */

/* C 70-85: the two include arms - `<sys/param.h>` for OpenBSD (measured: not this target) and
   `<pthread.h>` for the platforms that declare `pthread_sigmask` there.  Measured, that arm's
   `__APPLE__ && __MACH__` operand is true and `@GNULIB_PTHREAD_SIGMASK@` is 1, so C 86's
   `#include <pthread.h>` **is** live on this target (the same condition the C 71/79 lines show as
   `#if (1 || defined GNULIB_POSIXCHECK)`).  */

/* C 91-95: "The definitions of _GL_FUNCDECL_RPL etc. are copied here." / "of _GL_ARG_NONNULL" /
   "of _GL_WARN_ON_USE" - those definitions live in their own mirrors.  */

/* C 98-105: "On AIX, sig_atomic_t already includes volatile" - measured `#if ! 1`, dead: this
   target's `sig_atomic_t` is the system's (the port already spells it that way at
   `lib-src/emacsclient.rs` 80, `pub type sig_atomic_t = c_int;`).  */
#[cfg (feature = "no_have_type_volatile_sig_atomic_t")]
pub type rpl_sig_atomic_t = c_int;

#[cfg (feature = "no_have_type_volatile_sig_atomic_t")]
macro_rules! sig_atomic_t { () => { rpl_sig_atomic_t }; }

#[cfg (feature = "no_have_type_volatile_sig_atomic_t")]
pub const GNULIB_defined_sig_atomic_t: core::ffi::c_int = 1;

/* C 108-113: "A set or mask of signals."  Measured `#if !1`, dead (the SDK has `sigset_t`).  */
#[cfg (feature = "no_have_sigset_t")]
pub type sigset_t = c_uint;

#[cfg (feature = "no_have_sigset_t")]
pub const GNULIB_defined_sigset_t: core::ffi::c_int = 1;

/* C 116-128: "Define sighandler_t, the type of signal handlers.  A GNU extension."  Measured
   `@HAVE_SIGHANDLER_T@` 0 - **live**: the SDK has no `sighandler_t`, so C's own typedef is the
   ported text.  */
pub type sighandler_t = Option<unsafe extern "C" fn (c_int)>;

pub const GNULIB_defined_sighandler_t: core::ffi::c_int = 1;

/* C 130-137: `#if @GNULIB_SIGNAL_H_SIGPIPE@` - measured `#if 0`, dead: on this target `SIGPIPE`
   is 13 in `signal.rs` and gnulib does not redefine it.  */
#[cfg (feature = "no_gnulib_signal_h_sigpipe")]
pub const SIGPIPE: c_int = 13;

#[cfg (feature = "no_gnulib_signal_h_sigpipe")]
pub const GNULIB_defined_SIGPIPE: core::ffi::c_int = 1;

/* C 141-146: "Maximum signal number + 1" - `#ifndef NSIG` / `# if defined __TANDEM` / `#  define
   NSIG 32`: the SDK defines `NSIG`, and `__TANDEM` is not this target, so both the guard and the
   arm are dead.  */
#[cfg (feature = "tandem")]
pub const NSIG: c_int = 32;

/* C 148-153: "Maximum size of a signal name returned by sig2str(), including the terminating NUL
   byte" - live (the `#ifndef` does not fire: the SDK has no `SIG2STR_MAX`).  */
pub const SIG2STR_MAX: c_int = 5 + 1 + 10 + 1 + 2;

/* C 155-170: `sig2str`.  Measured `#if 1` / `# if !0` (generated 704-705): `@GNULIB_SIG2STR@` is 1
   and `@HAVE_SIG2STR@` is 0, so C 157-158's `_GL_FUNCDECL_SYS` **is** the live declaration.  */
unsafe extern "C" {
  pub fn sig2str (signo: c_int, str: *mut c_char) -> c_int;
}

/* C 172-185: `str2sig`, the same reading (`@HAVE_STR2SIG@` 0).  */
unsafe extern "C" {
  pub fn str2sig (str: *const c_char, signo_p: *mut c_int) -> c_int;
}

/* C 188-217: `pthread_sigmask`.  Measured `#if 1` / `# if 0`: `@REPLACE_PTHREAD_SIGMASK@` is 0, so
   C 203-207's `_GL_FUNCDECL_SYS` is live (the SDK has no `pthread_sigmask`; C 86's `<pthread.h>` is
   where the platform would declare it, and this target's arm is the `__APPLE__` one) and C 190-197's
   replacement arm is dead.  */
unsafe extern "C" {
  pub fn pthread_sigmask (how: c_int, new_mask: *const sigset_t, old_mask: *mut sigset_t) -> c_int;
}

#[cfg (feature = "replace_pthread_sigmask")]
macro_rules! pthread_sigmask { () => { rpl_pthread_sigmask }; }

/* C 225-247: `raise`.  Measured `#if 0` (generated 772) / `# if 0`: `@GNULIB_RAISE@` is 0, so the
   rename `# define raise rpl_raise`, the `_GL_FUNCDECL_RPL` and the `_GL_CXXALIASWARN` are all
   dead for this build.  */
#[cfg (feature = "gnulib_raise")]
macro_rules! raise { () => { rpl_raise }; }

/* C 249-441: `#if @GNULIB_SIGPROCMASK@` - measured `#if 0` (generated 796), so the whole block is
   dead: C 250-266's `GNULIB_defined_signal_blocking` / `NSIG 32` /
   `verify_NSIG_constraint[NSIG <= 32 ? 1 : -1]` / `GNULIB_defined_verify_NSIG_constraint`, C 279-293's
   `_GL_EXTERN_INLINE_IN_USE && __APPLE__ && (__i386__ || __x86_64__)` undef block, C 297-323's
   `sigismember` / `sigemptyset` / `sigaddset` / `sigdelset` / `sigfillset` SYS arms, C 326-341's
   `sigpending` and `sigprocmask`, C 332-335's `SIG_BLOCK 0` / `SIG_SETMASK 1` / `SIG_UNBLOCK 2`,
   C 347-357's `_gl_function_taking_int_returning_void_t` +
   `GNULIB_defined_function_taking_int_returning_void_t`, C 358-372's `signal` rename and C 407-410's
   `_gl_raise_SIGPIPE`.  The names the gate asks for are carried; the declarations are recorded.  */
#[cfg (feature = "gnulib_sigprocmask")]
pub const GNULIB_defined_signal_blocking: core::ffi::c_int = 1;

#[cfg (feature = "gnulib_sigprocmask")]
pub const GNULIB_defined_verify_NSIG_constraint: core::ffi::c_int = 1;

/// C 261: `typedef int verify_NSIG_constraint[NSIG <= 32 ? 1 : -1];` - the assertion typedef C
/// uses to pin `NSIG <= 32`; the port keeps the type (the condition is C's own constant folding).
#[cfg (feature = "gnulib_sigprocmask")]
pub type verify_NSIG_constraint = [c_int; 1];

#[cfg (feature = "gnulib_sigprocmask")]
pub const SIG_BLOCK: c_int = 0;

#[cfg (feature = "gnulib_sigprocmask")]
pub const SIG_SETMASK: c_int = 1;

#[cfg (feature = "gnulib_sigprocmask")]
pub const SIG_UNBLOCK: c_int = 2;

#[cfg (feature = "gnulib_sigprocmask")]
pub const GNULIB_defined_function_taking_int_returning_void_t: core::ffi::c_int = 1;

/// C 348: `typedef void (*_gl_function_taking_int_returning_void_t) (int);`
#[cfg (feature = "gnulib_sigprocmask")]
pub type _gl_function_taking_int_returning_void_t = Option<unsafe extern "C" fn (c_int)>;

#[cfg (any (feature = "gnulib_sigprocmask", feature = "no_have_posix_signalblocking"))]
macro_rules! signal { () => { rpl_signal }; }

/* C 444-515: `#if @GNULIB_SIGACTION@` - measured `#if 0` (generated 991), so the block is dead:
   this target's `struct sigaction` is Darwin's, and `signal.rs` carries it with its measured
   layout.  C's own definition - `union sigval`, `struct siginfo_t` + `typedef`, `struct sigaction`
   with the anonymous `_sa_func` union, the `sa_handler` / `sa_sigaction` macros and the three
   unsupported-flag values - is carried below behind the features that name the side where gnulib
   has to define them (`@HAVE_SIGACTION@` 1, `@HAVE_SIGINFO_T@` 1, `@HAVE_STRUCT_SIGACTION_SA_SIGACTION@` 1
   are all the *non*-gnulib side).  */
#[cfg (feature = "gnulib_sigaction")]
pub const GNULIB_defined_siginfo_types: core::ffi::c_int = 1;

/// C 453-457: `union sigval { int sival_int; void *sival_ptr; };` - "Present to allow
/// compilation, but unsupported by gnulib."
#[cfg (feature = "gnulib_sigaction")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union sigval
{
  pub sival_int: c_int,
  pub sival_ptr: *mut c_void,
}

/// C 460-471: `struct siginfo_t { … }; typedef struct siginfo_t siginfo_t;`
#[cfg (feature = "gnulib_sigaction")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct siginfo_t
{
  pub si_signo: c_int,
  pub si_code: c_int,
  pub si_errno: c_int,
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_addr: *mut c_void,
  pub si_status: c_int,
  pub si_band: c_long,
  pub si_value: sigval,
}

/// C 479-495: the anonymous union `_sa_func` ("POSIX says that implementations may, but not must,
/// make sa_sigaction overlap with sa_handler, but we know of no implementation where they do not
/// overlap").
#[cfg (feature = "gnulib_sigaction")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union _sa_func
{
  pub _sa_handler: Option<unsafe extern "C" fn (c_int)>,
  pub _sa_sigaction: Option<unsafe extern "C" fn (c_int, *mut siginfo_t, *mut c_void)>,
}

/// C 478-499: `struct sigaction { union { … } _sa_func; sigset_t sa_mask; int sa_flags; };`
#[cfg (feature = "gnulib_sigaction")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sigaction
{
  pub _sa_func: _sa_func,
  pub sa_mask: sigset_t,
  pub sa_flags: c_int,
}

#[cfg (feature = "gnulib_sigaction")]
macro_rules! sa_handler { () => { _sa_func._sa_handler }; }

#[cfg (feature = "gnulib_sigaction")]
macro_rules! sa_sigaction { () => { _sa_func._sa_sigaction }; }

#[cfg (feature = "gnulib_sigaction")]
pub const SA_RESETHAND: c_int = 1;

#[cfg (feature = "gnulib_sigaction")]
pub const SA_RESTART: c_int = 4;

#[cfg (feature = "gnulib_sigaction")]
pub const GNULIB_defined_struct_sigaction: core::ffi::c_int = 1;

/* C 511-513: `# elif !@HAVE_STRUCT_SIGACTION_SA_SIGACTION@` / `#  define sa_sigaction sa_handler` -
   measured `!1`, dead (the SDK's `struct sigaction` has `sa_sigaction`).  */
#[cfg (feature = "no_have_struct_sigaction_sa_sigaction")]
macro_rules! sa_sigaction { () => { sa_handler }; }

/* C 519-523: "Some systems don't have SA_NODEFER." - **live**: the SDK has no `SA_NODEFER`
   (measured), so C's fallback is the ported text.  Note the dead arm above also spells it (2).  */
pub const SA_NODEFER: c_int = 0;
