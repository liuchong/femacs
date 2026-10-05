// femacs: port of exec/trace.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same error
// model.  The C `static` helpers stay plain `unsafe fn`; the eight non-`static` definitions
// (`user_alloca`, `user_copy`, `exec_init`, `tracing_execve`, `after_fork`, `exec_waitpid` and the
// two `__aarch64__` register accessors) carry `#[unsafe(no_mangle)]` (PORTING.md 0.2).
//
// **This is one of P-86's class: the frozen configuration compiles nothing from this file, and the
// gate is the build's own rather than a `#if` in the text.**  `configure.ac` 178-220 configures
// `exec/` with the build directory set to `exec` only inside the `XCONFIGURE=android` block (1343
// sets that variable from `--with-android`), and `exec/configure.ac` 235 then puts `trace.o` into
// the `OBJS` list that `exec/Makefile.in`'s `libexec.a` rule - and the `loader` rule that links it -
// are built from.  `docs/baseline/frozen-build-sources.txt`'s batch-F section records no `exec/`
// object for this target ("the reference build has no exec/ directory and no rule reaches it"), so
// `tools/extract-abi.py`'s checklist for this file is empty (P-86 2) and the port's frozen object is
// the 336-byte empty one (Evidence 1).  The **file gate is `feature = "have_android"`** - the
// registered name for the build this file belongs to (`docs/baseline/feature-coverage.tsv` 11, the
// gate `src/androidmenu.rs` and `src/sfnt.rs` use) - on every item *and* on the include graph
// (P-86 8).
//
// The C text is generic across the five hosts `exec/configure.ac` 240-400 configures
// (`x86_64-*linux*`, `i[34567]86-*linux*`, `aarch64-*linux*`, `arm*linux*eabi* | armv7*linux*` and
// `mipsel*linux*`), so **both arms of every one of its 73 conditions are carried** (P-86 1) under a
// feature named for the macro the condition tests; `## Gate mapping` in `docs/ports/trace.md` is the
// row-per-condition table, and `## C line ranges` the coverage table.  Four of the conditions are
// `#ifndef X` / `#define X` fallback pairs (`SYS_SECCOMP` C 53, `PTRACE_GETEVENTMSG` C 57,
// `PTRACE_EVENT_SECCOMP` C 1856, `PTRACE_O_TRACESECCOMP` C 1860, `SIGSYS` C 1864): the system
// header's arm and C's own arm both define the same name, which is why the later `#ifdef SIGSYS` /
// `#ifdef PTRACE_*SECCOMP` sites carry the same feature as the pair that settles it.
//
// **`text-only` is *not* the reading here.**  This machine has the Android NDK (29.0.14206865, the
// one `spikes/jni-probe.sh` defaults to), so the file's own target is measurable with its `clang`
// (`aarch64-linux-android24`, `x86_64-linux-android24`, `armv7a-linux-androideabi24`;
// `docs/ports/trace.md`'s Evidence 5 holds the commands and the readings).  Every constant and
// record offset below is one of those readings unless its line says **UNVERIFIED** and names the
// source it was taken from instead.  The mipsel host has no toolchain here
// (`exec/configure.ac` 390-400's `struct mipsel_regs` arm), so its readings are named, not measured.
//
// Two C constructs have no Rust counterpart and are spelled the one way the tree already spells
// them (each is a `## Deviations` row in the ledger):
//
//   * `<assert.h>`'s `assert` (a `#if`-shaped macro: live without `NDEBUG`, `((void) 0)` with it) is
//     a `macro_rules! assert` with both arms - the `lib-src/ebrowse.rs` 93 shape;
//   * C's `#define stpcpy rpl_stpcpy` (C 954) is `use rpl_stpcpy as stpcpy;`, the rename form the
//     `lib/include/string.in.rs` 173-176 mirror already uses for the same gnulib `REPLACE_STPCPY`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals,
         unused_must_use, clippy::all)]

use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");   // C 20: `#include <config.h>`

/* C 36: `#include "exec.h"`.  The mirror declares `struct exec_tracee` and prototypes the eight
   routines this file defines, and this unit defines all eight - the P-39 `E0428` class - so it is
   consumed through a module plus a glob import: a same-named definition shadows a glob import
   (P-21/P-28, the `femacs/lib-src/ntlib.rs` 379-386 shape for the same reason).  */
#[cfg (feature = "have_android")]
mod exec_decls
{
  use super::*;
  include!("include/exec.rs");                 // C 36: `#include "exec.h"`
}
#[cfg (feature = "have_android")]
use exec_decls::*;

/* C 20's `<config.h>` carries two macros this file's text uses (`MAX`/`MIN`, C 84's `MAX_TRACEES`
   is the file's own); they are declared here with C's own bodies, the `femacs/src/sfnt.rs` 295-298
   shape for the same pair (the `config.rs` mirror holds generated *values*, not macros).  */
/// C: <config.h> 174-176: `#ifndef MIN` / `#define MIN(a, b) ((a) < (b) ? (a) : (b))`
#[cfg (feature = "have_android")]
macro_rules! MIN { ($a:expr, $b:expr) => { if ($a) < ($b) { $a } else { $b } }; }

/* ----------------------------------------------------------------------------------------------
   C 22-68: the system-header surface, in C's order.  Every one of these headers is the *target's*
   (a Linux/Android one), and no mirror in this tree carries the target's spellings: the landed
   mirrors that name the same identifiers carry the frozen target's values instead
   (`lib/include/errno.rs` 69's `ENAMETOOLONG` is 63 and not 36, `lib/include/signal.rs` 25/28's
   `SIGSTOP`/`SIGSYS` are 17/12 and not 19/31, `lib/include/fcntl.in.rs`'s `AT_FDCWD` is not -100,
   `lib/include/limits.in.rs`'s `PATH_MAX` is that target's own), and this file's text is the loader
   of a Linux system (`exec/README`).  So each name is declared at the C `#include` line it comes
   from (P-88 1) with the target's reading, and the readings are this machine's clang against the
   NDK sysroots unless the line says otherwise.
   ---------------------------------------------------------------------------------------------- */

/* C 22: `#include <sys/ptrace.h>`.  The request numbers, the five option bits and the five event
   codes are the kernel uapi's (`linux/ptrace.h`), read through the three NDK sysroots;
   `PTRACE_GETREGS`/`PTRACE_SETREGS` are the x86 family's and absent from the aarch64 header, and
   `PTRACE_SET_SYSCALL` is the `__arm__` arm's.  The four names C 53-59 and C 1856-1866 may define
   itself are *not* declared here - their two arms are at those lines.  */
/// C: <sys/ptrace.h>: `long ptrace (enum __ptrace_request, ...);`
#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub fn ptrace (request: c_uint, ...) -> c_long;
}
/// C: <linux/ptrace.h>: `#define PTRACE_TRACEME 0`
#[cfg (feature = "have_android")]
pub const PTRACE_TRACEME: c_uint = 0;
/// C: <linux/ptrace.h>: `#define PTRACE_PEEKTEXT 1`
#[cfg (feature = "have_android")]
pub const PTRACE_PEEKTEXT: c_uint = 1;
/// C: <linux/ptrace.h>: `#define PTRACE_PEEKDATA 2`
#[cfg (feature = "have_android")]
pub const PTRACE_PEEKDATA: c_uint = 2;
/// C: <linux/ptrace.h>: `#define PTRACE_POKETEXT 4`
#[cfg (feature = "have_android")]
pub const PTRACE_POKETEXT: c_uint = 4;
/// C: <linux/ptrace.h>: `#define PTRACE_POKEDATA 5`
#[cfg (feature = "have_android")]
pub const PTRACE_POKEDATA: c_uint = 5;
/// C: <linux/ptrace.h>: `#define PTRACE_CONT 7`
#[cfg (feature = "have_android")]
pub const PTRACE_CONT: c_uint = 7;
/// C: <linux/ptrace.h>: `#define PTRACE_DETACH 17`
#[cfg (feature = "have_android")]
pub const PTRACE_DETACH: c_uint = 17;
/// C: <linux/ptrace.h>: `#define PTRACE_SYSCALL 24`
#[cfg (feature = "have_android")]
pub const PTRACE_SYSCALL: c_uint = 24;
/// C: <linux/ptrace.h>: `#define PTRACE_SETOPTIONS 0x4200`
#[cfg (feature = "have_android")]
pub const PTRACE_SETOPTIONS: c_uint = 0x4200;
/// C 22's `<sys/ptrace.h>`: C 57-59's fallback pair settles it, and this is the header's arm -
/// the name C 478 passes to `ptrace`.
/// C: <sys/ptrace.h>: `#define PTRACE_GETEVENTMSG 0x4201`
#[cfg (all (feature = "have_android", feature = "ptrace_geteventmsg"))]
pub const PTRACE_GETEVENTMSG: c_uint = 0x4201;
/// C: <linux/ptrace.h>: `#define PTRACE_GETREGSET 0x4204`
#[cfg (feature = "have_android")]
pub const PTRACE_GETREGSET: c_uint = 0x4204;
/// C: <linux/ptrace.h>: `#define PTRACE_SETREGSET 0x4205`
#[cfg (feature = "have_android")]
pub const PTRACE_SETREGSET: c_uint = 0x4205;
/// C: <linux/ptrace.h>: `#define PTRACE_GETSIGINFO 0x4202`
#[cfg (feature = "have_android")]
pub const PTRACE_GETSIGINFO: c_uint = 0x4202;
/// C 22's `<sys/ptrace.h>`: the x86 family's two request numbers, the `not (feature = "aarch64")`
/// arms of C 299, C 805, C 870, C 1205, C 1233, C 1346, C 1478, C 1501, C 1823, C 1846, C 2167.
/// C: <linux/ptrace.h>: `#define PTRACE_GETREGS 12`
#[cfg (all (feature = "have_android", not (feature = "aarch64")))]
pub const PTRACE_GETREGS: c_uint = 12;
/// C: <linux/ptrace.h>: `#define PTRACE_SETREGS 13`
#[cfg (all (feature = "have_android", not (feature = "aarch64")))]
pub const PTRACE_SETREGS: c_uint = 13;
/// C 22's `<sys/ptrace.h>`: the `__arm__` arm's own request (C 1421-1428, C 1766-1773).
/// C: <linux/ptrace.h>: `#define PTRACE_SET_SYSCALL 23`
#[cfg (all (feature = "have_android", feature = "arm"))]
pub const PTRACE_SET_SYSCALL: c_uint = 23;
/// C: <linux/ptrace.h>: `#define PTRACE_O_TRACESYSGOOD 1`
#[cfg (feature = "have_android")]
pub const PTRACE_O_TRACESYSGOOD: c_int = 1;
/// C 22's `<sys/ptrace.h>`: the four event-derived option bits C 615-619 and C 1939-1946 set.
/// C: <linux/ptrace.h>: `#define PTRACE_O_TRACEFORK (1 << PTRACE_EVENT_FORK)`
#[cfg (feature = "have_android")]
pub const PTRACE_O_TRACEFORK: c_int = 1 << PTRACE_EVENT_FORK;
/// C: <linux/ptrace.h>: `#define PTRACE_O_TRACEVFORK (1 << PTRACE_EVENT_VFORK)`
#[cfg (feature = "have_android")]
pub const PTRACE_O_TRACEVFORK: c_int = 1 << PTRACE_EVENT_VFORK;
/// C: <linux/ptrace.h>: `#define PTRACE_O_TRACECLONE (1 << PTRACE_EVENT_CLONE)`
#[cfg (feature = "have_android")]
pub const PTRACE_O_TRACECLONE: c_int = 1 << PTRACE_EVENT_CLONE;
/// C: <linux/ptrace.h>: `#define PTRACE_O_TRACEEXIT (1 << PTRACE_EVENT_EXIT)`
#[cfg (feature = "have_android")]
pub const PTRACE_O_TRACEEXIT: c_int = 1 << PTRACE_EVENT_EXIT;
/// C: <linux/ptrace.h>: `#define PTRACE_EVENT_FORK 1`
#[cfg (feature = "have_android")]
pub const PTRACE_EVENT_FORK: c_int = 1;
/// C: <linux/ptrace.h>: `#define PTRACE_EVENT_VFORK 2`
#[cfg (feature = "have_android")]
pub const PTRACE_EVENT_VFORK: c_int = 2;
/// C: <linux/ptrace.h>: `#define PTRACE_EVENT_CLONE 3`
#[cfg (feature = "have_android")]
pub const PTRACE_EVENT_CLONE: c_int = 3;
/// C: <linux/ptrace.h>: `#define PTRACE_EVENT_EXIT 6`
#[cfg (feature = "have_android")]
pub const PTRACE_EVENT_EXIT: c_int = 6;

/* C 23: `#include <sys/types.h>`.  `pid_t` is `int` and `ssize_t` is `long` on every host this file
   configures (measured on all three NDK targets); they are declared here rather than taken from a
   mirror so that the file's target's widths are in force.  */
/// C: <sys/types.h>: `typedef __pid_t pid_t;`
#[cfg (feature = "have_android")]
pub type pid_t = c_int;
/// C: <sys/types.h>: `typedef __ssize_t ssize_t;`
#[cfg (feature = "have_android")]
pub type ssize_t = c_long;

/* C 24: `#include <sys/wait.h>`.  No mirror in this tree carries the header; the three macros this
   file reads and the one entry it calls are declared here with the target's bodies (P-88 1; the
   `femacs/exec/test.rs` 35-44 shape, which reads the same values).  */
// C: `#define WIFSTOPPED(status) (((status) & 0xff) == 0x7f)`
#[cfg (feature = "have_android")]
#[inline(always)] pub fn WIFSTOPPED (status: c_int) -> bool { (status & 0xff) == 0x7f }
// C: `#define WSTOPSIG(status) WEXITSTATUS(status)`
#[cfg (feature = "have_android")]
#[inline(always)] pub fn WSTOPSIG (status: c_int) -> c_int { (status & 0xff00) >> 8 }
/// C: <linux/wait.h>: `#define __WALL 0x40000000`
#[cfg (feature = "have_android")]
pub const __WALL: c_int = 0x40000000;
/// C: <sys/wait.h>: `pid_t waitpid (pid_t, int *, int);`
#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub fn waitpid (pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
}

/* C 26: `#include <limits.h>`.  The two names this file reads, with the target's readings
   (`PATH_MAX` 4096, `SSIZE_MAX` `LONG_MAX`; the mirror `lib/include/limits.in.rs` spells the frozen
   target's `PATH_MAX`).  */
/// C: <linux/limits.h>: `#define PATH_MAX 4096`
#[cfg (feature = "have_android")]
pub const PATH_MAX: usize = 4096;
/// C: <limits.h>: `#define SSIZE_MAX LONG_MAX`
#[cfg (feature = "have_android")]
pub const SSIZE_MAX: ssize_t = c_long::MAX;

/* C 27: `#include <stddef.h>`.  `size_t` is `usize` in this port (PORTING.md 1.1) and `ptrdiff_t`
   is `isize`; C 1030's one `ptrdiff_t` parameter is why the alias is here at all, and `NULL` is
   what C passes as `ptrace`'s third and fourth arguments at every call site in this file.  */
/// C: <stddef.h>: `#define std::ptr::null_mut::<c_void> () ((void *) 0)`
/// C: <stddef.h>: `typedef long int ptrdiff_t;`
#[cfg (feature = "have_android")]
pub type ptrdiff_t = isize;

/* C 28: `#include <string.h>`.  The seven entries C's text calls, declared at the C include line
   (P-88 1); `stpcpy` is the one whose availability is a `#if` in the C text, so its two arms are at
   C 933-955.  */
/// C: <string.h>: `void *memcpy (void *, const void *, size_t);`
/// C: <string.h>: `void *memmove (void *, const void *, size_t);`
/// C: <string.h>: `void *memchr (const void *, int, size_t);`
/// C: <string.h>: `size_t strlen (const char *);`
/// C: <string.h>: `int strcmp (const char *, const char *);`
/// C: <string.h>: `char *strncpy (char *, const char *, size_t);`
/// C: <string.h>: `char *strdup (const char *);`
#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub fn memcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
  pub fn memmove (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
  pub fn memchr (s: *const c_void, c: c_int, n: usize) -> *mut c_void;
  pub fn strlen (s: *const c_char) -> usize;
  pub fn strcmp (s1: *const c_char, s2: *const c_char) -> c_int;
  pub fn strncpy (dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
  pub fn strdup (s: *const c_char) -> *mut c_char;
}

/* C 29: `#include <assert.h>`.  This file's translation unit is the `exec/` one, whose `config.h`
   (`exec/config.h.in`) has no `NDEBUG` line and whose `CFLAGS` carry none, so the `<assert.h>` this
   file sees has the *live* `assert`: `src/conf_post.h` 28-29's `# define NDEBUG` is reached through
   `src/config.h`, which is not this unit's `<config.h>`.  Both arms are carried under `no_ndebug`
   (the name `src/sfnt.rs` uses for `#ifndef NDEBUG`, P-51's non-frozen-side rule): the live arm
   checks its operand, and the `NDEBUG` arm is C's `((void) 0)` and does not evaluate it
   (PORTING.md 2.5).  */
/// C: <assert.h>: `# define assert(expr) ((void) 0)` when `NDEBUG`, else
/// C: `((expr) ? (void) 0 : __assert_fail (#expr, __FILE__, __LINE__, __func__))`
#[cfg (all (feature = "have_android", feature = "no_ndebug"))]
macro_rules! assert {
  ($expr:expr) => { if !($expr) { abort () } };
}
#[cfg (all (feature = "have_android", not (feature = "no_ndebug")))]
macro_rules! assert {
  ($expr:expr) => { () };
}

/* C 30: `#include <signal.h>`.  The signal numbers, `SI_KERNEL` and `siginfo_t` are the target's
   (measured: `SIGTRAP` 5, `SIGSTOP` 19, `SI_KERNEL` 0x80); the landed `lib/include/signal.rs`
   mirror spells the frozen target's numbers (5/17) and is therefore not in this unit's graph.
   `SIGSYS` is not declared here: C 1864's fallback pair settles it (its header arm is at C 61-69's
   block, which carries `feature = "sigsys"`).  */
/// C: <signal.h>: `#define SIGTRAP 5`
#[cfg (feature = "have_android")]
pub const SIGTRAP: c_int = 5;
/// C: <signal.h>: `#define SIGSTOP 19`
#[cfg (feature = "have_android")]
pub const SIGSTOP: c_int = 19;
/// C: <signal.h>: `#define SI_KERNEL 0x80`
#[cfg (feature = "have_android")]
pub const SI_KERNEL: c_int = 0x80;
/// C: <signal.h>: `#define SIGSYS 31`.  This is the header arm of the pair C 1864-1866 carries the
/// fallback for, and it is *not* under `have_seccomp`: C 679's and C 2163's `#ifdef SIGSYS` are
/// outside that block.
#[cfg (all (feature = "have_android", feature = "sigsys"))]
pub const SIGSYS: c_int = 31;
/// C: <signal.h>: `int raise (int);` (C 1891).
#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub fn raise (sig: c_int) -> c_int;
}
/// C: <asm-generic/siginfo.h>: `typedef struct siginfo { union { __SIGINFO; int _si_pad[SI_MAX_SIZE / sizeof(int)]; }; } siginfo_t;`
/// The measured `asm-generic` layout is `si_signo` 0, `si_errno` 4, `si_code` 8, the `__sifields`
/// union at 16 with `_sigsys`'s `_call_addr` at 16 and `_syscall` at 24 (the `si_syscall` macro of
/// `asm-generic/siginfo.h` 116), and the record is `SI_MAX_SIZE` 128 bytes.  Only the members this
/// file reads are named; the union's other arms are not.
#[cfg (feature = "have_android")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct siginfo_t
{
  pub si_signo: c_int,           /* 0 */
  pub si_errno: c_int,           /* 4 */
  pub si_code: c_int,            /* 8 */
  pub _preamble: c_int,          /* 12: `__ARCH_SI_PREAMBLE_SIZE` is 16 on this target */
  pub _call_addr: *mut c_void,   /* 16: union __sifields._sigsys._call_addr */
  pub si_syscall: c_int,         /* 24: the `si_syscall` macro's member */
  pub _arch: c_uint,             /* 28 */
  pub _pad: [u8; 96],            /* to `SI_MAX_SIZE` 128 */
}
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<siginfo_t> () == 128);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (siginfo_t, si_code) == 8);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (siginfo_t, si_syscall) == 24);

/* C 31: `#include <unistd.h>`.  The two entries this file calls.  */
/// C: <unistd.h>: `ssize_t readlink (const char *, char *, size_t);`
/// C: <unistd.h>: `int execve (const char *, char *const [], char *const []);`
#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub fn readlink (path: *const c_char, buf: *mut c_char, len: usize) -> ssize_t;
  pub fn execve (path: *const c_char, argv: *const *mut c_char,
                 envp: *const *mut c_char) -> c_int;
}

/* C 32: `#include <stdlib.h>`.  The three entries C's text reaches (`malloc`/`free` at C 522-583,
   `abort` through the `assert` above).  */
/// C: <stdlib.h>: `void *malloc (size_t);`
/// C: <stdlib.h>: `void free (void *);`
/// C: <stdlib.h>: `_Noreturn void abort (void);`
#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub fn malloc (size: usize) -> *mut c_void;
  pub fn free (ptr: *mut c_void);
  pub fn abort () -> !;
}

/* C 33: `#include <errno.h>`.  `errno` is `(*__errno_location ())` on this target and not the
   `(*__error ())` the landed mirror spells (`lib/include/errno.rs` 12), so the accessor and the
   four values this file reads are declared here - `ENAMETOOLONG` is 36 and not the mirror's 63.  */
/// C: <errno.h>: `int *__errno_location (void);`
#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub fn __errno_location () -> *mut c_int;
}
/// C: <errno.h>: `#define errno (*__errno_location ())` - C's macro, carried as a function so that
/// `errno = x` and `errno` keep C's lvalue shape at the use sites.
#[cfg (feature = "have_android")]
#[inline(always)]
pub unsafe fn errno () -> *mut c_int { __errno_location () }
/// C: <errno.h>: `#define EINTR 4`
#[cfg (feature = "have_android")]
pub const EINTR: c_int = 4;
/// C: <errno.h>: `#define EIO 5`
#[cfg (feature = "have_android")]
pub const EIO: c_int = 5;
/// C: <errno.h>: `#define ENOMEM 12`
#[cfg (feature = "have_android")]
pub const ENOMEM: c_int = 12;
/// C: <errno.h>: `#define ENAMETOOLONG 36`
#[cfg (feature = "have_android")]
pub const ENAMETOOLONG: c_int = 36;

/* C 34: `#include <fcntl.h>`.  One name: `AT_FDCWD`, the `dirfd` value C 1033 and C 1103 compare
   against (measured -100; the mirror `fcntl.in.rs` spells the frozen target's).  */
/// C: <fcntl.h>: `#define AT_FDCWD -100`
#[cfg (feature = "have_android")]
pub const AT_FDCWD: c_int = -100;

/* ----------------------------------------------------------------------------------------------
   C 38-39: `#include SYSCALL_HEADER` and `#include USER_HEADER`, and the two dozen `AC_DEFINE`d
   names `exec/configure.ac` 240-400 supplies.  Like `USER_WORD`/`USER_HEADER` in the
   `femacs/exec/include/exec.rs` mirror's own note, these are the *host's* names rather than the
   file's: an `#include` has no Rust counterpart, and the names are declared here with the reading
   of the host this port spells (the Android `aarch64` one) and the other hosts' readings named
   (P-88 1, P-72 3).
   ---------------------------------------------------------------------------------------------- */

/// C 39's `USER_HEADER` (`exec/configure.ac` 269/302/334/367's `$user_h` or `<asm/ptrace.h>`) and
/// its `USER_REGS_STRUCT` (241/275/307/341/370).  The C text reaches the register record *through
/// the host's member macros* - `regs->STACK_POINTER`, `regs.SYSCALL_ARG_REG`,
/// `regs.SYSCALL_NUM_REG` - so the port's record carries those macro names as its member names: one
/// spelling every host's arm can share, because the C text names the same macro at the same place
/// on all of them.  The measured `aarch64` `struct user_regs_struct` is `uint64_t regs[31]` at 0,
/// `sp` 248, `pc` 256, `pstate` 264, 272 bytes.
/// C: <sys/user.h>: `struct user_regs_struct { uint64_t regs[31]; uint64_t sp; uint64_t pc; uint64_t pstate; };`
#[cfg (feature = "have_android")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USER_REGS_STRUCT
{
  /// `exec/configure.ac` 253's `AC_DEFINE([STACK_POINTER], [rsp])` and its four siblings (aarch64
  /// `sp`, x86-64 `rsp`, i386 `esp`, arm `uregs[13]`).
  pub STACK_POINTER: USER_WORD,
  /// 254/285/317/350's `SYSCALL_ARG_REG` (x86-64 `rdi`, aarch64/arm/mips `regs[0]`/`uregs[0]`).
  pub SYSCALL_ARG_REG: USER_WORD,
  /// 255/286/318/351's `SYSCALL_ARG1_REG`.
  pub SYSCALL_ARG1_REG: USER_WORD,
  /// 256/287/319/352's `SYSCALL_ARG2_REG`.
  pub SYSCALL_ARG2_REG: USER_WORD,
  /// 257/288/320/353's `SYSCALL_ARG3_REG`.
  pub SYSCALL_ARG3_REG: USER_WORD,
  /// 252/283/315/348's `SYSCALL_NUM_REG` (aarch64 `regs[8]`, x86-64 `orig_rax`).
  pub SYSCALL_NUM_REG: USER_WORD,
  /// 251/282/314/347's `SYSCALL_RET_REG` (aarch64 `regs[0]`, x86-64 `rax`), written at C 1510,
  /// C 1542, C 1827 and C 1855.
  pub SYSCALL_RET_REG: USER_WORD,
  /// 253's `PROGRAM_COUNTER`: a member of the same host record.  This file never reads it; it is
  /// declared because the record the C text names has it.
  pub PROGRAM_COUNTER: USER_WORD,
  /// `__aarch64__`'s `struct user_regs_struct` member, indexed literally at C 149 and C 219-221
  /// (`regs->regs[8]`).
  #[cfg (feature = "aarch64")]
  pub regs: [USER_WORD; 31],
  /// `__mips__`'s `struct mipsel_regs` member, indexed literally at C 1469-1471 and C 1492-1494
  /// (`regs.gregs[2]`, `regs.gregs[7]`).  **UNVERIFIED**: no mipsel toolchain here, and
  /// `exec/configure.ac` 390-400 is that record's own definition.
  #[cfg (feature = "mips")]
  pub gregs: [USER_WORD; 45],
}
/// `exec/configure.ac` 252/286/318/351's `AC_DEFINE([USER_WORD], [uintptr_t])`, which every host
/// configures the same way (PORTING.md 1.1: `usize`).  The mirror declares the same name for the
/// same reason (`femacs/exec/include/exec.rs` 181-184).
#[cfg (feature = "have_android")]
pub type USER_WORD = usize;
/// `exec/configure.ac` 253/287/319/352's `AC_DEFINE([USER_SWORD], [intptr_t])`.
#[cfg (feature = "have_android")]
pub type USER_SWORD = isize;

/// C 38's `SYSCALL_HEADER` (`<asm/unistd.h>`) and the three call numbers every host defines.  The
/// aarch64 reading is measured; the x86-64/i386/arm/mipsel ones are **UNVERIFIED** and named (the
/// `AC_DEFINE`s are per host, and the C text never writes a number of its own).
/// C: <asm/unistd.h>: `#define __NR_execve 221` (aarch64; x86-64 59, i386 11, arm 11, mipsel 4005+11)
#[cfg (feature = "have_android")]
pub const EXEC_SYSCALL: USER_WORD = 221;
/// C: <asm/unistd.h>: `#define __NR_readlinkat 78` (aarch64; x86-64 267, arm `__NR_SYSCALL_BASE+332`)
#[cfg (feature = "have_android")]
pub const READLINKAT_SYSCALL: USER_WORD = 78;
/// C: <asm/unistd.h>: `#define __NR_openat 56` (aarch64; x86-64 257, arm `__NR_SYSCALL_BASE+322`)
#[cfg (feature = "have_android")]
pub const OPENAT_SYSCALL: USER_WORD = 56;
/// C 38's `SYSCALL_HEADER`: `READLINK_SYSCALL` is defined by `exec/configure.ac` on the hosts that
/// *have* `readlink` (269-370's i386, arm and mipsel arms) and by neither the x86-64 nor the aarch64
/// one, which is exactly C 1049/1290/1685's `#ifdef READLINK_SYSCALL`.  The value is the host's: the
/// x86-64 reading (89) is measured and spelled, the i386/arm (85) and mipsel (4090) ones are
/// **UNVERIFIED** and named.
/// C: <asm/unistd.h>: `#define __NR_readlink 89`
#[cfg (all (feature = "have_android", feature = "readlink_syscall"))]
pub const READLINK_SYSCALL: USER_WORD = 89;
/// C 38's `SYSCALL_HEADER`: the same for `OPEN_SYSCALL` (C 1139, C 1195, C 1314, C 1366, C 1702).
/// C: <asm/unistd.h>: `#define __NR_open 2`
#[cfg (all (feature = "have_android", feature = "open_syscall"))]
pub const OPEN_SYSCALL: USER_WORD = 2;

/* ----------------------------------------------------------------------------------------------
   C 41-69: the four conditional include blocks, and the headers only their arms reach.  Each arm's
   surface is declared under that arm's own gate as well as the file gate (P-86 8: the gate goes on
   the include graph too).
   ---------------------------------------------------------------------------------------------- */

/* C 41-44: `#ifdef __aarch64__` - `<sys/uio.h>` for `struct iovec` and `<linux/elf.h>` for the two
   `NT_*` notes C 124/143 and C 157 name.  `struct iovec`'s measured layout is `iov_base` 0,
   `iov_len` 8, 16 bytes on all three NDK targets.  */
/// C 42's `<sys/uio.h>`: `struct iovec { void *iov_base; size_t iov_len; };`
#[cfg (all (feature = "have_android", any (feature = "aarch64", feature = "have_sys_uio_h")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct iovec
{
  pub iov_base: *mut c_void,
  pub iov_len: usize,
}
#[cfg (all (feature = "have_android", any (feature = "aarch64", feature = "have_sys_uio_h")))]
const _: () = core::assert! (std::mem::size_of::<iovec> () == 16);
#[cfg (all (feature = "have_android", any (feature = "aarch64", feature = "have_sys_uio_h")))]
const _: () = core::assert! (std::mem::offset_of! (iovec, iov_base) == 0);
#[cfg (all (feature = "have_android", any (feature = "aarch64", feature = "have_sys_uio_h")))]
const _: () = core::assert! (std::mem::offset_of! (iovec, iov_len) == 8);
/// C 43's `<linux/elf.h>`: the two note types C 124/143 and C 157 pass to `PTRACE_*REGSET`
/// (measured: `NT_PRSTATUS` 1, `NT_ARM_SYSTEM_CALL` 0x404).
/// C: <linux/elf.h>: `#define NT_PRSTATUS 1`
#[cfg (all (feature = "have_android", feature = "aarch64"))]
pub const NT_PRSTATUS: c_int = 1;
/// C: <linux/elf.h>: `#define NT_ARM_SYSTEM_CALL 0x404`
#[cfg (all (feature = "have_android", feature = "aarch64"))]
pub const NT_ARM_SYSTEM_CALL: c_int = 0x404;

/* C 47-49: `#ifdef HAVE_SYS_UIO_H` - `<sys/uio.h>` again (the same record as C 42's, which is why
   `iovec` above carries `any (aarch64, have_sys_uio_h)`), and `<dlfcn.h>` under the nested
   `#ifndef HAVE_PROCESS_VM`.  `process_vm_readv`/`process_vm_writev` are the two entries C 91-98
   takes the addresses of; C 204/341 call them through those pointers and C 2252/2254 resolve them
   with `dlsym`.  */
/// C: <sys/uio.h>: `ssize_t process_vm_readv (pid_t, const struct iovec *, unsigned long, const struct iovec *, unsigned long, unsigned long);`
/// C: <sys/uio.h>: `ssize_t process_vm_writev (pid_t, const struct iovec *, unsigned long, const struct iovec *, unsigned long, unsigned long);`
#[cfg (all (feature = "have_android", feature = "have_sys_uio_h", feature = "have_process_vm"))]
unsafe extern "C" {
  pub fn process_vm_readv (pid: pid_t, local: *const iovec, liovcnt: c_ulong,
                           remote: *const iovec, riovcnt: c_ulong, flags: c_ulong) -> ssize_t;
  pub fn process_vm_writev (pid: pid_t, local: *const iovec, liovcnt: c_ulong,
                            remote: *const iovec, riovcnt: c_ulong, flags: c_ulong) -> ssize_t;
}
/// C 49's `<dlfcn.h>`: `dlsym` and the `RTLD_DEFAULT` handle C 2252/2254 resolve the two entries
/// above with (measured: the NDK's `dlfcn.h` 169 and 213 define the handle as a null `void *`).
/// C: <dlfcn.h>: `void *dlsym (void *, const char *);`
#[cfg (all (feature = "have_android", feature = "have_sys_uio_h",
            not (feature = "have_process_vm")))]
pub const RTLD_DEFAULT: *mut c_void = std::ptr::null_mut ();
/// C: <dlfcn.h>: `#define RTLD_DEFAULT ((void *) 0)`
#[cfg (all (feature = "have_android", feature = "have_sys_uio_h",
            not (feature = "have_process_vm")))]
unsafe extern "C" {
  pub fn dlsym (handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

/* C 53-55: `#ifndef SYS_SECCOMP` / `#define SYS_SECCOMP 1` / `#endif` - the fallback arm of the
   pair whose header arm is the `linux/seccomp.h` line below (C 62).  C's text guarantees the name
   is defined from here on, which is why C 764 and C 2182 read it unguarded.  */
/// C: `#define SYS_SECCOMP 1`
#[cfg (all (feature = "have_android", not (feature = "sys_seccomp")))]
pub const SYS_SECCOMP: c_int = 1;

/* C 57-59: `#ifndef PTRACE_GETEVENTMSG` / `#define PTRACE_GETEVENTMSG 0x4201` / `#endif` - the
   pair whose header arm is the `sys/ptrace.h` line above.  */
/// C: `#define PTRACE_GETEVENTMSG 0x4201`
#[cfg (all (feature = "have_android", not (feature = "ptrace_geteventmsg")))]
pub const PTRACE_GETEVENTMSG: c_uint = 0x4201;

/* C 61-69: `#ifdef HAVE_SECCOMP` - `<linux/seccomp.h>`, `<linux/filter.h>`, `<sys/utsname.h>`,
   `<sys/prctl.h>` and `<stdio.h>`.  The BPF program C 1540-1665 builds and the three `prctl`
   requests C 1649-1661 and C 2230-2246 issue are declared here at the C include lines (P-88 1),
   with the measured readings of the three NDK targets.  */
/// C 62's `<linux/seccomp.h>`: the filter mode and the two return actions C 1583-1588 and C 1663
/// name.
/// C: <linux/seccomp.h>: `#define SECCOMP_MODE_FILTER 2`
/// C: <linux/seccomp.h>: `#define SECCOMP_RET_ALLOW 0x7fff0000U`
/// C: <linux/seccomp.h>: `#define SECCOMP_RET_TRACE 0x7ff00000U`
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const SECCOMP_MODE_FILTER: c_ulong = 2;
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const SECCOMP_RET_ALLOW: c_uint = 0x7fff0000;
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const SECCOMP_RET_TRACE: c_uint = 0x7ff00000;
/// C 61's `<sys/ptrace.h>` surface (this block's arm): the two names C 1856-1862's fallback pair
/// settles, whose header arm is here (C 2111's `PTRACE_EVENT_SECCOMP` and C 626's
/// `PTRACE_O_TRACESECCOMP`).
/// C: <linux/ptrace.h>: `#define PTRACE_EVENT_SECCOMP 7`
#[cfg (all (feature = "have_android", feature = "have_seccomp",
            feature = "ptrace_event_seccomp"))]
pub const PTRACE_EVENT_SECCOMP: c_int = 7;
/// C: <linux/ptrace.h>: `#define PTRACE_O_TRACESECCOMP (1 << PTRACE_EVENT_SECCOMP)`
#[cfg (all (feature = "have_android", feature = "have_seccomp",
            feature = "ptrace_o_traceseccomp"))]
pub const PTRACE_O_TRACESECCOMP: c_int = 1 << PTRACE_EVENT_SECCOMP;
/// C 62's `<linux/seccomp.h>`: `SYS_SECCOMP`, the `si_code` C 764 and C 763's two comparisons read.
/// It is *not* declared here: C 53-55's fallback pair settles it, and the header's arm (this gate)
/// is at that pair.
/// C: <linux/seccomp.h>: `#define SYS_SECCOMP 1`
#[cfg (all (feature = "have_android", feature = "have_seccomp", feature = "sys_seccomp"))]
pub const SYS_SECCOMP: c_int = 1;
/// C 65's `<sys/utsname.h>`: `uname` and the record C 2226-2229 reads `release` from.  The measured
/// layout is six `char[65]` members (390 bytes) with `release` at 130.
/// C: <sys/utsname.h>: `int uname (struct utsname *);`
/// C: <sys/utsname.h>: `struct utsname { char sysname[65], nodename[65], release[65], version[65], machine[65], domainname[65]; };`
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
unsafe extern "C" {
  pub fn uname (name: *mut utsname) -> c_int;
}
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct utsname
{
  pub sysname: [c_char; 65],
  pub nodename: [c_char; 65],
  pub release: [c_char; 65],
  pub version: [c_char; 65],
  pub machine: [c_char; 65],
  pub domainname: [c_char; 65],
}
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
const _: () = core::assert! (std::mem::size_of::<utsname> () == 390);
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
const _: () = core::assert! (std::mem::offset_of! (utsname, release) == 130);
/// C 66's `<sys/prctl.h>`: `prctl` and the three requests C 1649, C 1658, C 2230 and C 2237 use.
/// C: <sys/prctl.h>: `int prctl (int, ...);`
/// C: <linux/prctl.h>: `#define PR_SET_NO_NEW_PRIVS 38`
/// C: <linux/prctl.h>: `#define PR_SET_SECCOMP 22`
/// C: <linux/prctl.h>: `#define PR_GET_SECCOMP 21`
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
unsafe extern "C" {
  pub fn prctl (option: c_int, ...) -> c_int;
}
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const PR_SET_NO_NEW_PRIVS: c_int = 38;
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const PR_SET_SECCOMP: c_int = 22;
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const PR_GET_SECCOMP: c_int = 21;
/// C 68's `<stdio.h>`: `sscanf` only (C 2229).
/// C: <stdio.h>: `int sscanf (const char *, const char *, ...);`
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
unsafe extern "C" {
  pub fn sscanf (s: *const c_char, format: *const c_char, ...) -> c_int;
}
/// C 63's `<linux/filter.h>`: the two records the filter program is built from (C 1540-1655).
/// Measured: `struct sock_filter` is `__u16 code` 0, `__u8 jt` 2, `__u8 jf` 3, `__u32 k` 4, 8
/// bytes; `struct sock_fprog` is `unsigned short len` 0 and `filter` 8, 16 bytes.
/// C: <linux/filter.h>: `struct sock_filter { __u16 code; __u8 jt; __u8 jf; __u32 k; };`
/// C: <linux/filter.h>: `struct sock_fprog { unsigned short len; struct sock_filter *filter; };`
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sock_filter
{
  pub code: c_ushort,
  pub jt: c_uchar,
  pub jf: c_uchar,
  pub k: c_uint,
}
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sock_fprog
{
  pub len: c_ushort,
  pub filter: *mut sock_filter,
}
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
const _: () = core::assert! (std::mem::size_of::<sock_filter> () == 8);
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
const _: () = core::assert! (std::mem::offset_of! (sock_filter, k) == 4);
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
const _: () = core::assert! (std::mem::size_of::<sock_fprog> () == 16);
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
const _: () = core::assert! (std::mem::offset_of! (sock_fprog, filter) == 8);
/// C 62's `<linux/seccomp.h>`: `struct seccomp_data`, whose `nr` C 1560 offsets (measured: `nr` 0,
/// `arch` 4, `instruction_pointer` 8, `args` 16, 64 bytes).
/// C: <linux/seccomp.h>: `struct seccomp_data { int nr; __u32 arch; __u64 instruction_pointer; __u64 args[6]; };`
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct seccomp_data
{
  pub nr: c_int,
  pub arch: c_uint,
  pub instruction_pointer: c_ulonglong,
  pub args: [c_ulonglong; 6],
}
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
const _: () = core::assert! (std::mem::size_of::<seccomp_data> () == 64);
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
const _: () = core::assert! (std::mem::offset_of! (seccomp_data, nr) == 0);
/// C 63's `<linux/filter.h>`: the seven BPF encodings C 1556-1596 composes (measured on all three
/// NDK targets: `BPF_LD` 0, `BPF_W` 0, `BPF_ABS` 0x20, `BPF_JMP` 5, `BPF_JEQ` 0x10, `BPF_K` 0,
/// `BPF_RET` 6).
/// C: <linux/filter.h>: `#define BPF_LD 0x00`
/// C: <linux/filter.h>: `#define BPF_W 0x00`
/// C: <linux/filter.h>: `#define BPF_ABS 0x20`
/// C: <linux/filter.h>: `#define BPF_JMP 0x05`
/// C: <linux/filter.h>: `#define BPF_JEQ 0x10`
/// C: <linux/filter.h>: `#define BPF_K 0x00`
/// C: <linux/filter.h>: `#define BPF_RET 0x06`
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const BPF_LD: c_ushort = 0x00;
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const BPF_W: c_ushort = 0x00;
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const BPF_ABS: c_ushort = 0x20;
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const BPF_JMP: c_ushort = 0x05;
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const BPF_JEQ: c_ushort = 0x10;
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const BPF_K: c_ushort = 0x00;
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub const BPF_RET: c_ushort = 0x06;
/// C 63's `<linux/filter.h>`: the two statement constructors of C 1556-1596, carried with C's own
/// names and bodies.
/// C: <linux/filter.h>: `#define BPF_STMT(code, k) ((struct sock_filter) { (unsigned short) (code), 0, 0, k })`
/// C: <linux/filter.h>: `#define BPF_JUMP(code, k, jt, jf) ((struct sock_filter) { (unsigned short) (code), jt, jf, k })`
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
macro_rules! BPF_STMT {
  ($code:expr, $k:expr) => { sock_filter { code: $code, jt: 0, jf: 0, k: $k } };
}
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
macro_rules! BPF_JUMP {
  ($code:expr, $k:expr, $jt:expr, $jf:expr) =>
    { sock_filter { code: $code, jt: $jt, jf: $jf, k: $k } };
}
/// C 63's `<linux/filter.h>`: C 1560's `offsetof (struct seccomp_data, nr)`, carried with C's name
/// over Rust's own construct.
/// C: <stddef.h>: `#define offsetof(type, member) __builtin_offsetof (type, member)`
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
macro_rules! offsetof {
  ($t:ty, $f:ident) => { core::mem::offset_of! ($t, $f) };
}

/* ----------------------------------------------------------------------------------------------
   C 71-2258: the file's own text, in C's order.
   ---------------------------------------------------------------------------------------------- */

/* C 73-79: "Program tracing functions.  The main entry point is the function `tracing_execve', …"
   - C's own commentary, carried in the ledger for this file.  */

/* C 83-84: `#define MAX_TRACEES 4096`.  */
/// C: `#define MAX_TRACEES 4096`
#[cfg (feature = "have_android")]
pub const MAX_TRACEES: c_int = 4096;

/* C 86-100: the two function pointers `have_process_vm` is reached through when it is *not* a
   configured symbol.  C declares them as `static ssize_t (*process_vm_readv) (…);` and fills them
   from `dlsym` at C 2252/2254; the port uses `Option<unsafe extern "C" fn …>` with `None` as C's
   null and writes them through a `*mut *mut c_void` cast, which is C's `*(void **) (&name) = …`
   exactly.  `` `process_vm_readv` '' used as a condition at C 201/339 is `is_some ()` here
   (`## Deviations`).  */
/// C: `static ssize_t (*process_vm_readv) (pid_t, const struct iovec *, unsigned long, const struct iovec *, unsigned long, unsigned long);`
#[cfg (all (feature = "have_android", feature = "have_sys_uio_h",
            not (feature = "have_process_vm")))]
pub static mut process_vm_readv: Option<unsafe extern "C" fn (pid_t, *const iovec, c_ulong,
                                                              *const iovec, c_ulong, c_ulong)
                                                             -> ssize_t> = None;
/// C: `static ssize_t (*process_vm_writev) (pid_t, const struct iovec *, unsigned long, const struct iovec *, unsigned long, unsigned long);`
#[cfg (all (feature = "have_android", feature = "have_sys_uio_h",
            not (feature = "have_process_vm")))]
pub static mut process_vm_writev: Option<unsafe extern "C" fn (pid_t, *const iovec, c_ulong,
                                                               *const iovec, c_ulong, c_ulong)
                                                              -> ssize_t> = None;

/* C 102-109: `#ifdef HAVE_SECCOMP` - the seccomp switch, and C's own `#else` that makes it the
   constant `false`.  */
/// C: `static bool use_seccomp_p;` (C 105)
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub static mut use_seccomp_p: bool = false;
/// C 107-108's `#else` arm: `#define use_seccomp_p (false)` - carried as the const the C macro
/// spells, so both the read sites (C 536/544/…) and the writes keep one shape.
/// C: `#define use_seccomp_p (false)`
#[cfg (all (feature = "have_android", not (feature = "have_seccomp")))]
pub const use_seccomp_p: bool = false;

/* C 111-165: `#ifdef __aarch64__` - the two register accessors of that host.  They are C's only
   two `__aarch64__` definitions; every other `__aarch64__` site in the file is a call site or a
   `#else` arm.  */
/// C 118: `int aarch64_get_regs (pid_t pid, USER_REGS_STRUCT *regs)`.
#[cfg (all (feature = "have_android", feature = "aarch64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aarch64_get_regs (pid: pid_t, regs: *mut USER_REGS_STRUCT) -> c_int
{
  let mut iov: iovec = std::mem::zeroed ();              // C 120: `struct iovec iov;` (C fills it field by field)

  iov.iov_base = regs as *mut c_void;                    // C 122: `iov.iov_base = regs;`
  iov.iov_len = std::mem::size_of::<USER_REGS_STRUCT> ();  // C 123: `iov.iov_len = sizeof *regs;`

  /* C 125-126.  */
  return (ptrace (PTRACE_GETREGSET, pid, NT_PRSTATUS,
		  &raw mut iov) != 0) as c_int;
}

/// C 137-138: `int aarch64_set_regs (pid_t pid, USER_REGS_STRUCT *regs, bool syscall_p)`.
#[cfg (all (feature = "have_android", feature = "aarch64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aarch64_set_regs (pid: pid_t, regs: *mut USER_REGS_STRUCT,
                                           syscall_p: bool) -> c_int
{
  let mut iov: iovec = std::mem::zeroed ();              // C 140: `struct iovec iov;` (C fills it field by field)
  let mut callno: USER_WORD;                             // C 141: `USER_WORD callno;`
  let mut rc: c_long;                                    // C 142: `long rc;`

  /* Write the user registers.  */

  iov.iov_base = regs as *mut c_void;                    // C 148: `iov.iov_base = regs;`
  iov.iov_len = std::mem::size_of::<USER_REGS_STRUCT> ();  // C 149: `iov.iov_len = sizeof *regs;`

  rc = ptrace (PTRACE_SETREGSET, pid, NT_PRSTATUS, &raw mut iov);   // C 151-152
  if rc < 0
    { return 1; }                                        // C 153-154

  /* Now, write the system call number if necessary.  */

  if syscall_p                                            // C 159
    {
      callno = (*regs).regs[8];                           // C 161: `callno = regs->regs[8];`
      iov.iov_base = (&raw mut callno) as *mut c_void;    // C 162: `iov.iov_base = &callno;`
      iov.iov_len = std::mem::size_of::<USER_WORD> ();    // C 163: `iov.iov_len = sizeof callno;`

      /* C 165-166.  */
      return (ptrace (PTRACE_SETREGSET, pid, NT_ARM_SYSTEM_CALL,
		      &raw mut iov) != 0) as c_int;
    }

  return 0;                                               // C 169
}

/* C 175-176: `static struct exec_tracee *tracing_processes;` - the list every other function here
   walks.  */
/// C 176: `static struct exec_tracee *tracing_processes;`
#[cfg (feature = "have_android")]
pub static mut tracing_processes: *mut exec_tracee = std::ptr::null_mut ();

/* C 180-241: `read_memory` - the one reader of a tracee's memory.  Its first half is the
   `process_vm_readv` fast path (C 186-207's `#ifdef HAVE_SYS_UIO_H`, whose inner
   `#ifndef HAVE_PROCESS_VM` arm is the `dlsym`-resolved pointer's null test) and its second half
   reads whole words with `PTRACE_PEEKTEXT`.  */
/// C 180-181: `static void read_memory (struct exec_tracee *tracee, char *buffer, USER_WORD n, USER_WORD address)`.
#[cfg (feature = "have_android")]
unsafe fn read_memory (tracee: *mut exec_tracee, mut buffer: *mut c_char,
                       n: USER_WORD, mut address: USER_WORD)
{
  let mut word: USER_WORD = 0;                           // C 184: `USER_WORD word, n_words, n_bytes, i;` (C's first write is the `ptrace`/`memcpy` pair below)
  let mut n_words: USER_WORD;
  let mut n_bytes: USER_WORD;
  let mut i: USER_WORD;
  let mut rc: c_long;                                    // C 185: `long rc;`

  /* C 186-207: `#ifdef HAVE_SYS_UIO_H`.  */
  #[cfg (feature = "have_sys_uio_h")]
  {
    let mut iov: iovec = std::mem::zeroed ();            // C 187: `struct iovec iov, remote;` (C fills it field by field)
    let mut remote: iovec = std::mem::zeroed ();

    /* If `process_vm_readv' is available, use it instead.  */

    iov.iov_base = buffer as *mut c_void;                // C 191: `iov.iov_base = buffer;`
    iov.iov_len = n;                                     // C 192: `iov.iov_len = n;`
    remote.iov_base = address as *mut c_void;            // C 193: `remote.iov_base = (void *) address;`
    remote.iov_len = n;                                  // C 194: `remote.iov_len = n;`

    /* Return immediately if successful.  As long as some bytes were
       read, consider the read to have been a success.  */

    /* C 199-205.  The `#ifndef HAVE_PROCESS_VM` arm C 200-202 adds `&& process_vm_readv`, C's
       null test on the pointer - the port spells it `is_some ()` on the pointer and calls the
       library entry of the other arm directly (`## Deviations`).  */
    #[cfg (feature = "have_process_vm")]
    {
      if n <= SSIZE_MAX as USER_WORD
	&& process_vm_readv ((*tracee).pid, &raw const iov, 1,
			     &raw const remote, 1, 0) != -1
	{ return; }
    }
    #[cfg (not (feature = "have_process_vm"))]
    {
      if n <= SSIZE_MAX as USER_WORD
	&& process_vm_readv.is_some ()
	&& (process_vm_readv.unwrap ()) ((*tracee).pid, &raw const iov, 1,
					 &raw const remote, 1, 0) != -1
	{ return; }
    }

  }

  /* First, read entire words from the tracee.  */
  n_words = n & !(std::mem::size_of::<USER_WORD> () - 1);  // C 210

  /* Next, determine the number of bytes to read from the last
     word.  */
  n_bytes = n & (std::mem::size_of::<USER_WORD> () - 1);   // C 214

  /* Start reading words.  */
  i = 0;                                                   // C 217: `i = 0;`
  while n_words != 0                                       // C 218: `while (n_words)`
    {
      rc = ptrace (PTRACE_PEEKTEXT, (*tracee).pid,
		   (address + i) as *mut c_void, std::ptr::null_mut::<c_void> ());   // C 220-221
      word = rc as USER_WORD;                              // C 222: `word = rc;`
      memcpy (buffer as *mut c_void, (&raw const word) as *const c_void,
              std::mem::size_of::<USER_WORD> ());          // C 223: `memcpy (buffer, &word, sizeof word);`
      buffer = buffer.add (std::mem::size_of::<USER_WORD> ());  // C 224: `buffer += sizeof word;`
      i += std::mem::size_of::<USER_WORD> ();              // C 225: `i += sizeof word;`
      n_words -= std::mem::size_of::<USER_WORD> ();        // C 226: `n_words -= sizeof word;`
    }

  /* Now, read the remaining bytes.  */
  assert! (n_bytes < std::mem::size_of::<USER_WORD> ());    // C 230: `assert! (n_bytes < sizeof (word));`

  if n_bytes != 0                                          // C 232: `if (n_bytes)`
    {
      rc = ptrace (PTRACE_PEEKTEXT, (*tracee).pid,
		   (address + i) as *mut c_void, std::ptr::null_mut::<c_void> ());   // C 234-235
      word = rc as USER_WORD;                              // C 236: `word = rc;`

      /* Copy only n_bytes to the caller.  */
      memcpy (buffer as *mut c_void, (&raw const word) as *const c_void, n_bytes);  // C 239
    }
}

/* C 252-315: `user_alloca`.  Its three `#ifdef STACK_GROWS_DOWNWARDS` pairs are *live* in every
   configuration `exec/configure.ac` supports (269/302/335/368 all `AC_DEFINE` it to 1), so the
   positive arm is ungated and the dead arm carries `no_stack_grows_downwards` (P-51's
   non-frozen-side naming).  C 261's `#if RED_ZONE_SIZE` tests a name nothing defines: the
   `ABI_RED_ZONE` template of `exec/configure.ac` 124 is what is `AC_DEFINE`d (255, the x86-64 arm),
   so that block's live arm is the empty one and `feature = "red_zone_size"` selects C's text as it
   stands (`## Deviations`).  */
/// C 252-254: `USER_WORD user_alloca (struct exec_tracee *tracee, USER_REGS_STRUCT *regs, USER_REGS_STRUCT *new_regs, USER_WORD n)`.
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_alloca (tracee: *mut exec_tracee, regs: *mut USER_REGS_STRUCT,
                                      new_regs: *mut USER_REGS_STRUCT, n: USER_WORD) -> USER_WORD
{
  let mut sp: USER_WORD;                                 // C 256: `USER_WORD sp, old_sp;`
  let mut old_sp: USER_WORD;

  /* Get the current stack pointer.  C's chained assignment is two statements in Rust.  */
  sp = (*new_regs).STACK_POINTER;                        // C 259: `old_sp = sp = new_regs->STACK_POINTER;`
  old_sp = sp;

  /* C's `fail:` label (C 311) is the port's labeled block tail, which is what PORTING.md 2.2's
     shared-branch spelling of a `goto` is: the `break 'fail` statements below are C's `goto fail`.  */
  'fail: {
  /* C 261-272: `#if RED_ZONE_SIZE` - dead text in every configuration this file has, because the
     name `RED_ZONE_SIZE` is defined nowhere and `ABI_RED_ZONE` is the template's (`## Deviations`).
     Both of the block's arms are carried, and its inner `#ifdef STACK_GROWS_DOWNWARDS` pair too.  */
  #[cfg (feature = "red_zone_size")]
  {
    /* Some ABI rules specify a ``red zone'' around the stack pointer
       that is reserved for compiler optimizations.  */

    #[cfg (not (feature = "no_stack_grows_downwards"))]
    {
      if sp == (*regs).STACK_POINTER                       // C 266-267
	{ sp -= RED_ZONE_SIZE; }
    }
    #[cfg (feature = "no_stack_grows_downwards")]
    {
      if sp == (*regs).STACK_POINTER                       // C 269-270
	{ sp += RED_ZONE_SIZE; }
    }
  }

  /* Now take N off the stack.  */

  /* C 276-290: the live arm (`STACK_GROWS_DOWNWARDS` is defined on every host this file
     configures, so `no_stack_grows_downwards` is the dead side) and C's other one.  */
  #[cfg (not (feature = "no_stack_grows_downwards"))]
  {
    sp = sp - n;                                           // C 277: `sp = sp - n;`

    /* Check for overflow.  */

    if sp > (*new_regs).STACK_POINTER                      // C 281-282
      { return 0; }
  }
  #[cfg (feature = "no_stack_grows_downwards")]
  {
    sp = sp + n;                                           // C 284: `sp = sp + n;`

    /* Check for overflow.  */

    if sp < (*new_regs).STACK_POINTER                      // C 288-289
      { return 0; }
  }

  /* Set the stack pointer.  */
  (*new_regs).STACK_POINTER = sp;                          // C 293

  /* C 295-302: the `__aarch64__` arm and its `#else`.  */
  #[cfg (feature = "aarch64")]
  {
    if aarch64_set_regs ((*tracee).pid, new_regs, false) != 0
      { break 'fail; }                                     // C 296-297: `goto fail;`
  }
  #[cfg (not (feature = "aarch64"))]
  {
    if ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (),
	       new_regs) != 0
      { break 'fail; }                                     // C 299-301: `goto fail;`
  }

  /* Now return the start of the new area.  */
  #[cfg (not (feature = "no_stack_grows_downwards"))]
  { return sp; }                                           // C 306: `return sp;`
  #[cfg (feature = "no_stack_grows_downwards")]
  { return sp - n; }                                        // C 308: `return sp - n;`
  }

  /* fail: */
  /* Restore the old stack pointer.  */
  (*new_regs).STACK_POINTER = old_sp;                      // C 313
  return 0;                                                // C 314
}

/// C 261's `#if RED_ZONE_SIZE`: the block's one token, `ABI_RED_ZONE`'s value on the x86-64 arm
/// (`exec/configure.ac` 255's `128`) - the name C tests is never defined, so the block is dead text
/// here either way.  **UNVERIFIED** as a `RED_ZONE_SIZE` reading: no configuration defines that
/// name.
/// C: exec/configure.ac 255: `AC_DEFINE([ABI_RED_ZONE], [128])`
#[cfg (all (feature = "have_android", feature = "red_zone_size"))]
pub const RED_ZONE_SIZE: USER_WORD = 128;

/* C 320-397: `user_copy` - the writer that mirrors `read_memory`.  */
/// C 320-322: `int user_copy (struct exec_tracee *tracee, const unsigned char *buffer, USER_WORD address, USER_WORD n)`.
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_copy (tracee: *mut exec_tracee, mut buffer: *const c_uchar,
                                    address: USER_WORD, n: USER_WORD) -> c_int
{
  let mut start: USER_WORD;                              // C 324: `USER_WORD start, end, word;`
  let mut end: USER_WORD;
  let mut word: USER_WORD = 0;                           // C 324: `USER_WORD start, end, word;` (C's first write is the `memcpy`/`ptrace` pair below)
  let mut bytes: *mut c_uchar;                           // C 325: `unsigned char *bytes;`

  /* C 326-344: `#ifdef HAVE_SYS_UIO_H`.  */
  #[cfg (feature = "have_sys_uio_h")]
  {
    let mut iov: iovec = std::mem::zeroed ();            // C 327: `struct iovec iov, remote;` (C fills it field by field)
    let mut remote: iovec = std::mem::zeroed ();

    /* Try to use `process_vm_writev' if possible, but fall back to
       ptrace if something bad happens.  */

    iov.iov_base = buffer as *mut c_void;                // C 332: `iov.iov_base = (void *) buffer;`
    iov.iov_len = n;                                     // C 333: `iov.iov_len = n;`
    remote.iov_base = address as *mut c_void;            // C 334: `remote.iov_base = (void *) address;`
    remote.iov_len = n;                                  // C 335: `remote.iov_len = n;`

    /* C 337-343.  */
    #[cfg (feature = "have_process_vm")]
    {
      if n <= SSIZE_MAX as USER_WORD
	&& process_vm_writev ((*tracee).pid, &raw const iov, 1,
			      &raw const remote, 1, 0) == n as ssize_t
	{ return 0; }
    }
    #[cfg (not (feature = "have_process_vm"))]
    {
      if n <= SSIZE_MAX as USER_WORD
	&& process_vm_writev.is_some ()
	&& (process_vm_writev.unwrap ()) ((*tracee).pid, &raw const iov, 1,
					  &raw const remote, 1, 0) == n as ssize_t
	{ return 0; }
    }
  }

  /* Calculate the start and end positions for the write.  */

  start = address;                                       // C 347: `start = address;`
  end = address + n;                                     // C 348: `end = address + n;`

  /* Write from start to the last word.  */

  while start < end                                       // C 351
    {
      if start + std::mem::size_of::<USER_WORD> () <= end  // C 353
	{
	  /* Write a word by itself and increment start.  */
	  memcpy ((&raw mut word) as *mut c_void, buffer as *const c_void,
                  std::mem::size_of::<USER_WORD> ());        // C 356: `memcpy (&word, buffer, sizeof word);`
	  buffer = buffer.add (std::mem::size_of::<USER_WORD> ());   // C 357: `buffer += sizeof word;`

	  if ptrace (PTRACE_POKEDATA, (*tracee).pid,
		      start as *mut c_void, word as *mut c_void) != 0
	    { return 1; }                                    // C 359-361

	  start += std::mem::size_of::<USER_WORD> ();        // C 363: `start += sizeof word;`
	}
      else
	{
	  /* Only end - start bytes should be written.
	     Read the word at start from tracee->pid, then write
	     it back with changes.  */

	  word = ptrace (PTRACE_PEEKDATA, (*tracee).pid,
			 start as *mut c_void, std::ptr::null_mut::<c_void> ()) as USER_WORD;  // C 369-370
	  bytes = (&raw mut word) as *mut c_uchar;           // C 371: `bytes = (unsigned char *) &word;`
	  memcpy (bytes as *mut c_void, buffer as *const c_void, end - start);  // C 372

	  if ptrace (PTRACE_POKEDATA, (*tracee).pid,
		      start as *mut c_void, word as *mut c_void) != 0
	    { return 1; }                                    // C 374-376

	  /* Writing was successful.  */
	  return 0;                                          // C 379
	}
    }

  return 0;                                              // C 383
}

/* C 401: `static struct exec_tracee *free_tracees;` - the free list `remove_tracee` feeds.  */
/// C 401: `static struct exec_tracee *free_tracees;`
#[cfg (feature = "have_android")]
pub static mut free_tracees: *mut exec_tracee = std::ptr::null_mut ();

/// C 405-406: `static void remove_tracee (struct exec_tracee *tracee)`.
#[cfg (feature = "have_android")]
unsafe fn remove_tracee (tracee: *mut exec_tracee)
{
  let mut last: *mut *mut exec_tracee;                   // C 408: `struct exec_tracee **last;`

  last = &raw mut tracing_processes;                     // C 410: `last = &tracing_processes;`
  while !(*last).is_null ()                              // C 411: `while (*last)`
    {
      if *last == tracee                                // C 413
	{
	  *last = (*tracee).next;                        // C 415: `*last = tracee->next;`

	  /* Link the tracee onto the list of free tracees.  */
	  (*tracee).next = free_tracees;                 // C 418

	  /* Free the exec file, if any.  */
	  free ((*tracee).exec_file as *mut c_void);     // C 421
	  (*tracee).exec_file = std::ptr::null_mut ();   // C 422

	  /* Likewise with any loader instructions that might be
	     present.  */
	  free ((*tracee).exec_data as *mut c_void);     // C 426
	  (*tracee).exec_data = std::ptr::null_mut ();   // C 427

	  /* Return this tracee to the list of free ones.  */
	  free_tracees = tracee;                         // C 430
	  return;                                        // C 431
	}
      else
	{ last = &raw mut (**last).next as *mut *mut exec_tracee; }   // C 434: `last = &(*last)->next;`
    }
}

/* C 442: `static struct exec_tracee static_tracees[MAX_TRACEES];` - the arena tracees are handed
   out of, and C 445's count of them.  */
/// C 442: `static struct exec_tracee static_tracees[MAX_TRACEES];`
#[cfg (feature = "have_android")]
pub static mut static_tracees: [exec_tracee; MAX_TRACEES as usize] =
  [(exec_tracee { next: std::ptr::null_mut (), sp: 0, callno: 0, exec_file: std::ptr::null_mut (),
                  exec_data: std::ptr::null_mut (), data_size: 0, pid: 0, bits: 0 });
   MAX_TRACEES as usize];
/// C 445: `static int tracees;`
#[cfg (feature = "have_android")]
pub static mut tracees: c_int = 0;

/// C 451-452: `static struct exec_tracee *find_tracee (pid_t process)`.
#[cfg (feature = "have_android")]
unsafe fn find_tracee (process: pid_t) -> *mut exec_tracee
{
  let mut tracee: *mut exec_tracee;                      // C 454: `struct exec_tracee *tracee;`

  tracee = tracing_processes;                            // C 456: `for (tracee = tracing_processes; tracee; tracee = tracee->next)`
  loop
    {
      if tracee.is_null ()
	{ break; }
      if (*tracee).pid == process                       // C 458
	{ return tracee; }                              // C 459
      tracee = (*tracee).next;
    }

  return std::ptr::null_mut ();                          // C 462: `return NULL;`
}

/// C 471-472: `static void handle_clone_prepare (struct exec_tracee *parent)`.
#[cfg (feature = "have_android")]
unsafe fn handle_clone_prepare (parent: *mut exec_tracee)
{
  let mut rc: c_long;                                    // C 474: `long rc;`
  let mut pid: c_ulong = 0;                              // C 475: `unsigned long pid;` (C's first write is the `PTRACE_GETEVENTMSG` call below)
  let mut tracee: *mut exec_tracee;                      // C 476: `struct exec_tracee *tracee;`

  rc = ptrace (PTRACE_GETEVENTMSG, (*parent).pid, std::ptr::null_mut::<c_void> (),
	       &raw mut pid);                            // C 478-479: `rc = ptrace (PTRACE_GETEVENTMSG, parent->pid, NULL, &pid);`
  if rc != 0
    { return; }                                          // C 480-481

  /* See if the tracee already exists.  */
  tracee = find_tracee (pid as pid_t);                   // C 484

  if !tracee.is_null ()                                  // C 486: `if (tracee)`
    {
      /* Continue the tracee.  Record its command line, as that has
	 not yet been done.  */

      assert! (exec_tracee_bit_p (tracee, EXEC_TRACEE_NEW_CHILD_BIT));   // C 491
      exec_tracee_bit_set (tracee, EXEC_TRACEE_NEW_CHILD_BIT, false);   // C 492: `tracee->new_child = false;`
      (*tracee).exec_file = std::ptr::null_mut ();       // C 493: `tracee->exec_file = NULL;`
      ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
	      (*tracee).pid, 0, 0);                      // C 494-495

      if !(*parent).exec_file.is_null ()                 // C 497: `if (parent->exec_file)`
	{ (*tracee).exec_file = strdup ((*parent).exec_file); }   // C 498: `tracee->exec_file = strdup (parent->exec_file);`
      return;                                            // C 499
    }

  if !free_tracees.is_null ()                            // C 502: `if (free_tracees)`
    {
      tracee = free_tracees;                             // C 504
      free_tracees = (*free_tracees).next;               // C 505
    }
  else if tracees < MAX_TRACEES                          // C 507: `else if (tracees < MAX_TRACEES)`
    {
      tracee = &raw mut static_tracees[tracees as usize];  // C 509: `tracee = &static_tracees[tracees];`
      tracees += 1;                                      // C 510
    }
  /* Try to allocate a tracee using `malloc'.  */
  else                                                   // C 513: `else if ((tracee = malloc (sizeof *tracee))) ;`
    {
      tracee = malloc (std::mem::size_of::<exec_tracee> ()) as *mut exec_tracee;
      if tracee.is_null ()                               // C 514-515's `else return;`
	{ return; }
    }

  (*tracee).pid = pid as pid_t;                          // C 517: `tracee->pid = pid;`
  (*tracee).next = tracing_processes;                    // C 518
  exec_tracee_bit_set (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT, false);  // C 519: `tracee->waiting_for_syscall = false;`
  exec_tracee_bit_set (tracee, EXEC_TRACEE_NEW_CHILD_BIT, true);            // C 520: `tracee->new_child = true;`
  (*tracee).exec_file = std::ptr::null_mut ();           // C 521
  tracing_processes = tracee;                            // C 522

  /* Copy over the command line.  */

  if !(*parent).exec_file.is_null ()                     // C 526
    { (*tracee).exec_file = strdup ((*parent).exec_file); }   // C 527
}

/// C 538-539: `static int handle_clone (struct exec_tracee *tracee, pid_t pid)`.
#[cfg (feature = "have_android")]
unsafe fn handle_clone (mut tracee: *mut exec_tracee, pid: pid_t) -> c_int
{
  let mut rc: c_long;                                    // C 541: `long rc;`
  let mut flags: c_int;                                  // C 542: `int flags, value;`
  let mut value: c_int;

  /* Now allocate a new tracee, either from static_tracees or the free
     list, if no tracee was supplied.  */

  value = 0;                                             // C 547: `value = 0;`

  /* C's `bail:` label (C 613) is the port's labeled-block tail, the PORTING.md 2.2 spelling of the
     two `goto bail` statements below.  */
  'bail: {
  if tracee.is_null ()                                   // C 549: `if (!tracee)`
    {
      if !free_tracees.is_null ()                        // C 551
	{
	  tracee = free_tracees;                         // C 553
	  free_tracees = (*free_tracees).next;           // C 554
	}
      else if tracees < MAX_TRACEES                      // C 556
	{
	  tracee = &raw mut static_tracees[tracees as usize];  // C 558
	  tracees += 1;                                  // C 559
	}
      /* Try to allocate a tracee using `malloc'.  */
      else                                               // C 561: `else if ((tracee = malloc (sizeof *tracee))) ;`
	{
	  tracee = malloc (std::mem::size_of::<exec_tracee> ()) as *mut exec_tracee;
	  if tracee.is_null ()                           // C 562-563's `else return 1;`
	    { return 1; }
	}

      (*tracee).pid = pid;                               // C 565
      (*tracee).next = tracing_processes;                // C 566
      exec_tracee_bit_set (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT, false);  // C 567
      (*tracee).exec_file = std::ptr::null_mut ();       // C 568
      (*tracee).exec_data = std::ptr::null_mut ();       // C 569
      (*tracee).data_size = 0;                           // C 570
      tracing_processes = tracee;                        // C 571
      exec_tracee_bit_set (tracee, EXEC_TRACEE_NEW_CHILD_BIT, true);            // C 572

      /* Wait for the ptrace-stop to happen in the parent.  */
      value = 2;                                         // C 575
    }
  else
    /* Clear the flag saying that this is a newly created child
       process.  */
    { exec_tracee_bit_set (tracee, EXEC_TRACEE_NEW_CHILD_BIT, false); }         // C 579

  /* Apply required options to the child, so that the kernel
     automatically traces children and makes it easy to differentiate
     between system call traps and other kinds of traps.  */

  flags  = PTRACE_O_TRACECLONE;                          // C 586
  flags |= PTRACE_O_TRACEVFORK;                          // C 587
  flags |= PTRACE_O_TRACEFORK;                           // C 588
  flags |= PTRACE_O_TRACESYSGOOD;                        // C 589
  flags |= PTRACE_O_TRACEEXIT;                           // C 590

  #[cfg (feature = "have_seccomp")]                      // C 592-594: `#ifdef HAVE_SECCOMP`
  {
    if use_seccomp_p
      { flags |= PTRACE_O_TRACESECCOMP; }
  }

  rc = ptrace (PTRACE_SETOPTIONS, pid, 0, flags);        // C 596: `rc = ptrace (PTRACE_SETOPTIONS, pid, 0, flags);`

  if rc != 0
    { break 'bail; }                                 // C 598-599: `if (rc) goto bail;`

  if value != 2                                          // C 601
    {
      /* The new tracee is currently stopped.  Continue it until the next
	 system call.  */

      rc = ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
		   pid, 0, 0);                           // C 605-606

      if rc != 0
	{ break 'bail; }                                 // C 608-609: `goto bail;`
    }

  return value;                                          // C 611
  }

  /* bail: */
  remove_tracee (tracee);                                // C 614
  return 1;                                              // C 615
}

/* C 622-623: "NOTICE: none of these functions should ever call `malloc' or another async signal
   unsafe function." - C's own commentary, carried in the ledger for this file.  */

/* C 626: `static const char *loader_name;` - the loader's file name C 884 and C 897-899 read.  */
/// C 626: `static const char *loader_name;`
#[cfg (feature = "have_android")]
pub static mut loader_name: *const c_char = std::ptr::null ();

/// C 633-634: `static bool syscall_trap_p (siginfo_t *signal)`.
#[cfg (feature = "have_android")]
unsafe fn syscall_trap_p (signal: *mut siginfo_t) -> bool
{
  /* SIGTRAP delivered by the kernel means this is a system call
     stop.  */
  return (*signal).si_code == SIGTRAP                       // C 638-639
    || (*signal).si_code == (SIGTRAP | SI_KERNEL);
}

/// C 648-649: `static int check_signal (struct exec_tracee *tracee, int status)`.
#[cfg (feature = "have_android")]
unsafe fn check_signal (tracee: *mut exec_tracee, status: c_int) -> c_int
{
  let mut siginfo: siginfo_t = std::mem::zeroed ();      // C 651: `siginfo_t siginfo;` (C leaves it uninitialised; `PTRACE_GETSIGINFO` fills it - PORTING.md 0.1-13)

  /* C 653's `switch ((status & 0xfff00) >> 8)`.  C's `case SIGTRAP` has no `break` and falls into
     `case SIGTRAP | 0x80`, which is why the port's first arm carries both arms' statements and its
     second is empty.  */
  match ((status & 0xfff00) >> 8)
    {
      c if c == SIGTRAP =>                               // C 655
	{
	  /* Now, use PTRACE_GETSIGINFO to determine whether or not the
	     signal was delivered in response to a system call.  */

	  if ptrace (PTRACE_GETSIGINFO, (*tracee).pid, 0, &raw mut siginfo) != 0
	    { return -1; }                               // C 659-660

	  if !syscall_trap_p (&raw mut siginfo)          // C 662
	    {
	      if siginfo.si_code < 0                     // C 664
		/* SIGTRAP delivered from userspace.  Pass it on.  */
		{ ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
			  (*tracee).pid, 0, SIGTRAP); }  // C 666-667
	      else
		{ ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
			  (*tracee).pid, 0, 0); }        // C 669-670

	      return 1;                                  // C 672
	    }

	  /* fall through to `case SIGTRAP | 0x80` (C 675-677): `break;` - nothing further.  */
	}
      /* C 675-676: `case SIGTRAP | 0x80:` - SIGTRAP | 0x80 specifically refers to system call
	 traps.  Its whole body is C 677's `break;`, so the arm above falls into this one and this
	 one has no statement of its own.  */
      c if c == SIGTRAP | 0x80 => {}
      /* C 679-705: `#ifdef SIGSYS` - the arm C 1864-1866's fallback pair settles (`feature =
	 "sigsys"` is the header's side of that pair).  */
      #[cfg (feature = "sigsys")]
      c if c == SIGSYS =>                                // C 680
	{
	  if ptrace (PTRACE_GETSIGINFO, (*tracee).pid, 0, &raw mut siginfo) != 0
	    { return -1; }                               // C 681-682

	  /* Continue the process until the next syscall, but don't
	     pass through the signal if an emulated syscall led to
	     it.  */
	  /* C 687-703: `#ifdef HAVE_SIGINFO_T_SI_SYSCALL` and, inside it, `#ifndef __arm__`.  */
	  #[cfg (feature = "have_siginfo_t_si_syscall")]
	  {
	    #[cfg (not (feature = "arm"))]
	    { ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL }, (*tracee).pid,
		      0, if siginfo.si_code == SYS_SECCOMP         // C 689-692
			   && siginfo.si_syscall == -1
			 { 0 } else { status }); }
	    #[cfg (feature = "arm")]
	    { ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL }, (*tracee).pid,
		      0, if siginfo.si_code == SYS_SECCOMP         // C 694-697
			   && siginfo.si_syscall == 222
			 { 0 } else { status }); }
	  }
	  #[cfg (not (feature = "have_siginfo_t_si_syscall"))]
	  {
	    /* Drop this signal, since what caused it is unknown.  */
	    ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL }, (*tracee).pid,
		    0, 0);                                      // C 701-702
	  }
	  return 1;                                          // C 704
	}

      _ =>                                                  // C 707: `default:`
	{
	  /* Continue the process until the next syscall.  */
	  ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
		  (*tracee).pid, 0, status);                 // C 709-710
	  return 1;                                          // C 711
	}
    }

  return 0;                                             // C 714
}

/* C 728-729: `static int handle_exec (struct exec_tracee *tracee, USER_REGS_STRUCT *regs)`.  Its
   two `goto`s are the port's labeled blocks / loop (PORTING.md 2.2).  */
/// C 728-729: `static int handle_exec (struct exec_tracee *tracee, USER_REGS_STRUCT *regs)`.
#[cfg (feature = "have_android")]
unsafe fn handle_exec (tracee: *mut exec_tracee, regs: *mut USER_REGS_STRUCT) -> c_int
{
  let mut buffer: [c_char; PATH_MAX + 80] = [0; PATH_MAX + 80];  // C 731: `char buffer[PATH_MAX + 80], *area;`
  let mut area: *mut c_char;
  let mut original: USER_REGS_STRUCT = std::mem::zeroed ();   // C 732: `USER_REGS_STRUCT original;` (C's first write is the `memcpy` below)
  let mut size: usize = 0;                               // C 733: `size_t size, loader_size;` (C's first write is `exec_0`'s out-parameter)
  let mut loader_size: usize;
  let mut loader: USER_WORD;                             // C 734: `USER_WORD loader;`

  /* Read the file name.  */
  read_memory (tracee, &raw mut buffer as *mut c_char, PATH_MAX,
	       (*regs).SYSCALL_ARG_REG);                     // C 738: `read_memory (tracee, buffer, PATH_MAX, regs->SYSCALL_ARG_REG);`

  /* Make sure BUFFER is NULL terminated.  */
  if memchr ((&raw const buffer) as *const c_void, 0, PATH_MAX).is_null()   // C 742
    {
      *errno () = ENAMETOOLONG;                          // C 744: `errno = ENAMETOOLONG;`
      return 1;                                          // C 745
    }

  /* Copy over the registers as they originally were.  */
  memcpy ((&raw mut original) as *mut c_void, regs as *const c_void,
          std::mem::size_of::<USER_REGS_STRUCT> ());     // C 748: `memcpy (&original, regs, sizeof *regs);`

  /* Figure out what the loader needs to do.  */
  /* C 751's `again1:` label.  */
  loop
    {
      area = exec_0 ((&raw mut buffer) as *mut c_char, tracee, &raw mut size, regs as *mut c_void);  // C 753

      if area.is_null ()                                 // C 755: `if (!area)`
	{
	  /* Handle SIGINTR errors caused by IO.  */
	  if *errno () == EINTR                          // C 758: `if (errno == EINTR)`
	    { continue; }                                // C 759: `goto again1;`

	  return 1;                                      // C 761
	}
      break;
    }

  /* C's `free_data_error:` label (C 833) is the port's labeled-block tail below.  */
  'free_data_error: {
  /* Save this area in the tracee.  */
  assert! ((*tracee).exec_data.is_null ());               // C 764: `assert! (!tracee->exec_data);`
  (*tracee).exec_data = malloc (size) as *mut c_char;    // C 765: `tracee->exec_data = malloc (size);`
  if (*tracee).exec_data.is_null ()                      // C 766
    {
      *errno () = ENOMEM;                                // C 768: `errno = ENOMEM;`
      return 1;                                          // C 769
    }
  memcpy ((*tracee).exec_data as *mut c_void, area as *const c_void, size);  // C 771
  (*tracee).data_size = size;                            // C 772: `tracee->data_size = size;`

  /* Rewrite the first argument to point to the loader.  */
  loader_size = strlen (loader_name) + 1;                // C 775: `loader_size = strlen (loader_name) + 1;`
  loader = user_alloca (tracee, &raw mut original, regs,
			loader_size);                    // C 776-777

  if loader == 0                                         // C 779: `if (!loader)`
    {
      *errno () = ENOMEM;                                // C 781: `errno = ENOMEM;`
      break 'free_data_error;                            // C 782: `goto free_data_error;`
    }

  if user_copy (tracee, loader_name as *const c_uchar,
		loader, loader_size) != 0
    {
      *errno () = EIO;                                   // C 787: `errno = EIO;`
      break 'free_data_error;                            // C 788: `goto free_data_error;`
    }

  (*regs).SYSCALL_ARG_REG = loader;                      // C 790: `regs->SYSCALL_ARG_REG = loader;`

  /* C 792-814: the `__aarch64__` arm and its `#else`.  */
  #[cfg (feature = "aarch64")]
  {
    if aarch64_set_regs ((*tracee).pid, regs, false) != 0
      {
	*errno () = EIO;                                 // C 796: `errno = EIO;`
	break 'free_data_error;                          // C 797: `goto free_data_error;`
      }
  }
  #[cfg (not (feature = "aarch64"))]
  {
    if ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (), regs) != 0
      {
	*errno () = EIO;                                 // C 808: `errno = EIO;`
	break 'free_data_error;                          // C 809: `goto free_data_error;`
      }
  }

  /* Resume the process till the loader is executed.  */
  if ptrace (PTRACE_SYSCALL, (*tracee).pid, std::ptr::null_mut::<c_void> (),
	     std::ptr::null_mut::<c_void> ()) != 0
    {
      *errno () = EIO;                                   // C 819: `errno = EIO;`
      break 'free_data_error;                            // C 820: `goto free_data_error;`
    }

  /* Now that the loader has been executed, record the value to
     substitute for /proc/self/exe.  Don't give up just because strdup
     fails.

     Note that exec_0 copies the absolute file name into buffer.  */

  if !(*tracee).exec_file.is_null ()                     // C 828: `if (tracee->exec_file)`
    { free ((*tracee).exec_file as *mut c_void); }       // C 829: `free (tracee->exec_file);`
  (*tracee).exec_file = strdup ((&raw const buffer) as *const c_char);   // C 830: `tracee->exec_file = strdup (buffer);`
  return 0;                                              // C 831
  }

  /* free_data_error: */
  free ((*tracee).exec_data as *mut c_void);             // C 836: `free (tracee->exec_data);`
  (*tracee).exec_data = std::ptr::null_mut ();           // C 837: `tracee->exec_data = NULL;`
  return 1;                                              // C 838
}

/* C 849-850: `static void finish_exec (struct exec_tracee *tracee, USER_REGS_STRUCT *regs)`.  */
/// C 849-850: `static void finish_exec (struct exec_tracee *tracee, USER_REGS_STRUCT *regs)`.
#[cfg (feature = "have_android")]
unsafe fn finish_exec (tracee: *mut exec_tracee, regs: *mut USER_REGS_STRUCT)
{
  let mut size1: USER_WORD;                              // C 852: `USER_WORD size1, loader;`
  let mut loader: USER_WORD;
  let mut original: USER_REGS_STRUCT = std::mem::zeroed ();   // C 853: `USER_REGS_STRUCT original;` (C's first write is the `memcpy` below)

  size1 = (*tracee).data_size;                           // C 855: `size1 = tracee->data_size;`

  /* Record the registers' values as they originally were.  */
  memcpy ((&raw mut original) as *mut c_void, regs as *const c_void,
          std::mem::size_of::<USER_REGS_STRUCT> ());     // C 858: `memcpy (&original, regs, sizeof *regs);`

  /* C's `error:` label (C 924) is the port's labeled-block tail, the PORTING.md 2.2 spelling of
     the five `goto error` statements below.  */
  'error: {
  /* Any non-zero value of `original.SYSCALL_RET_REG' indicates that the
     reissued `exec' call was unsuccessful, and the loader is not
     executing.  Restore the previous stack pointer and permit the
     tracee to run to completion.  */

  if original.SYSCALL_RET_REG != 0                       // C 865
    {
      (*regs).STACK_POINTER = (*tracee).sp;              // C 867: `regs->STACK_POINTER = tracee->sp;`
      /* C 868-872: the `__aarch64__` arm and its `#else`, neither of whose results C checks.  */
      #[cfg (feature = "aarch64")]
      { aarch64_set_regs ((*tracee).pid, regs, false); }
      #[cfg (not (feature = "aarch64"))]
      { ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (), regs); }

      /* Continue; not much in the way of remediation is available if
	 either of PTRACE_SETREGS and this resumption fails.  */
      ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
	      (*tracee).pid, 0, 0);                      // C 876-877
      break 'error;                                      // C 878: `goto error;`
    }

  /* Write the loader area to the stack, followed by its size and the
     original stack pointer.  */

  loader = user_alloca (tracee, &raw mut original, regs,
			size1 + std::mem::size_of::<USER_WORD> () as USER_WORD * 2);  // C 884-885
  if loader == 0                                         // C 886: `if (!loader)`
    { break 'error; }                                    // C 887: `goto error;`

  /* C 889-891: `#ifndef STACK_GROWS_DOWNWARDS` - C's deliberate "this host cannot work" marker.
     It is the arm no configuration this file configures reaches, and it is carried as the same
     error C's own text is: a `compile_error!` (the `lib/include/frame.rs` 320 precedent), gated on
     P-51's non-live-side name so the live arm below needs no gate.  */
  #[cfg (feature = "no_stack_grows_downwards")]
  compile_error! ("not implemented, you lose.");

  #[cfg (not (feature = "no_stack_grows_downwards"))]
  {
    /* C 893-897.  */
    if user_copy (tracee, (*tracee).exec_data as *const c_uchar,
		  loader + std::mem::size_of::<USER_WORD> () as USER_WORD * 2, size1) != 0
	|| user_copy (tracee, (&raw const size1) as *const c_uchar,
		      loader + std::mem::size_of::<USER_WORD> () as USER_WORD, size1) != 0
      { break 'error; }                                  // C 897: `goto error;`

    size1 = original.STACK_POINTER;                      // C 899: `size1 = original.STACK_POINTER;`

    if user_copy (tracee, (&raw const size1) as *const c_uchar,
		  loader, size1) != 0                    // C 901-902
      { break 'error; }                                  // C 903: `goto error;`
  }

  /* Continue.  */
  if ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
	     (*tracee).pid, 0, 0) != 0
    { break 'error; }                                    // C 910: `goto error;`

  /* C 912-922: `#if 0` / `{ int rc, wstatus; again1: … }` / `#endif` - dead in every
     configuration: dropped (the `femacs/src/w32term.rs` 685 precedent).  */
  }

  /* error: */
  free ((*tracee).exec_data as *mut c_void);             // C 925: `free (tracee->exec_data);`
  (*tracee).exec_data = std::ptr::null_mut ();           // C 926: `tracee->exec_data = NULL;`
}

/* C 933-955: `#if !defined HAVE_STPCPY || !defined HAVE_DECL_STPCPY` - the portable `rpl_stpcpy`
   and C's `#define stpcpy rpl_stpcpy`.  The condition is a gnulib one whose two names are tested
   with `defined`, so the pair is `any (not (feature = "have_stpcpy"),
   not (feature = "have_decl_stpcpy"))`; the live arm is C 933-955's `#else` (nothing: the C library
   provides the entry), carried as the declaration below.  */
/// C: <string.h>: `char *stpcpy (char *, const char *);`
#[cfg (all (feature = "have_android", feature = "have_stpcpy", feature = "have_decl_stpcpy"))]
unsafe extern "C" {
  pub fn stpcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char;
}
/// C 938-939: `static char *rpl_stpcpy (char *dest, const char *src)`.
/// C: `static char *rpl_stpcpy (char *dest, const char *src)`
#[cfg (all (feature = "have_android",
            any (not (feature = "have_stpcpy"), not (feature = "have_decl_stpcpy"))))]
unsafe fn rpl_stpcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char
{
  let mut d: *mut c_char;                                // C 941: `register char *d;`
  let mut s: *const c_char;                              // C 942: `register const char *s;`

  d = dest;                                              // C 944: `d = dest;`
  s = src;                                               // C 945: `s = src;`

  loop                                                   // C 947-949: `do *d++ = *s; while (*s++ != '\0');`
    {
      *d = *s;
      d = d.add (1);
      let c = *s;
      s = s.add (1);
      if c == 0
	{ break; }
    }

  return d.sub (1);                                      // C 951: `return d - 1;`
}
/// C 954: `#define stpcpy rpl_stpcpy` - the object-like rename, carried as a rename import (the
/// `lib/include/string.in.rs` 173-176 shape for the same gnulib selector).
#[cfg (all (feature = "have_android",
            any (not (feature = "have_stpcpy"), not (feature = "have_decl_stpcpy"))))]
use rpl_stpcpy as stpcpy;

/* C 968-969: `static int canon_path (struct exec_tracee *tracee, int fd, char *buffer, ptrdiff_t size)`.
   `sizeof "/proc//fd/"` is 9, so C 970's array is 57 bytes (carried as the number, with C's
   expression in the citation).  */
/// C 968-969: `static int canon_path (struct exec_tracee *tracee, int fd, char *buffer, ptrdiff_t size)`.
#[cfg (feature = "have_android")]
unsafe fn canon_path (tracee: *mut exec_tracee, fd: c_int, buffer: *mut c_char,
                      size: ptrdiff_t) -> c_int
{
  let mut link: [c_char; 57] = [0; 57];                  // C 970: `char link[sizeof "/proc//fd/" + 48], *p;`
  let mut p: *mut c_char;
  let mut target: [c_char; PATH_MAX] = [0; PATH_MAX];    // C 971: `char target[PATH_MAX];`
  let mut rc: ssize_t;                                   // C 972: `ssize_t rc, length;`
  let mut length: ssize_t;

  if *buffer == b'/' as c_char                          // C 974: `if (buffer[0] == '/')`
    /* Absolute file name; return immediately.  */
    { return 0; }                                        // C 976
  else if fd == AT_FDCWD                                 // C 977
    {
      p = stpcpy (&raw mut link as *mut c_char, c"/proc/".as_ptr ());   // C 979: `p = stpcpy (link, "/proc/");`
      p = format_pid (p, (*tracee).pid as c_uint);        // C 980: `p = format_pid (p, tracee->pid);`
      stpcpy (p, c"/cwd".as_ptr ());                      // C 981: `stpcpy (p, "/cwd");`
    }
  else if fd < 0                                          // C 983: `else if (fd < 0)`
    /* Invalid file descriptor.  */
    { return 1; }                                        // C 985
  else
    {
      p = stpcpy (&raw mut link as *mut c_char, c"/proc/".as_ptr ());   // C 988
      p = format_pid (p, (*tracee).pid as c_uint);        // C 989
      p = stpcpy (p, c"/fd/".as_ptr ());                  // C 990
      format_pid (p, fd as c_uint);                       // C 991: `format_pid (p, fd);`
    }

  /* Read LINK's target, and should it be oversized, punt.  */
  rc = readlink ((&raw const link) as *const c_char, &raw mut target as *mut c_char,
                 PATH_MAX);                              // C 994: `rc = readlink (link, target, PATH_MAX);`
  if rc < 0 || rc >= PATH_MAX as ssize_t                 // C 995
    { return 1; }                                        // C 996

  /* Consider the amount by which BUFFER's existing contents should be
     displaced.  */

  length = (strlen (buffer) + 1) as ssize_t;             // C 1000: `length = strlen (buffer) + 1;`
  if (length + rc + if target[rc as usize - 1] != b'/' as c_char { 1 } else { 0 }) > size as ssize_t
    /* Punt if this would overflow.  */
    { return 1; }                                        // C 1003

  /* C 1005-1006: `memmove ((buffer + rc + (target[rc - 1] != '/')), buffer, length);`.  */
  memmove (buffer.add ((rc + if target[rc as usize - 1] != b'/' as c_char { 1 } else { 0 }) as usize)
	   as *mut c_void, buffer as *const c_void, length as usize);

  /* Copy the new file name into BUFFER.  */
  memcpy (buffer as *mut c_void, (&raw const target) as *const c_void, rc as usize);   // C 1009

  /* Insert separator in between if need be.  */
  if target[rc as usize - 1] != b'/' as c_char            // C 1012
    { *buffer.add (rc as usize) = b'/' as c_char; }       // C 1013: `buffer[rc] = '/';`

  return 0;                                              // C 1015
}

/// C 1032-1034: `static int handle_readlinkat (USER_WORD callno, USER_REGS_STRUCT *regs, struct exec_tracee *tracee, USER_WORD *result)`.
#[cfg (feature = "have_android")]
unsafe fn handle_readlinkat (callno: USER_WORD, regs: *mut USER_REGS_STRUCT,
                             tracee: *mut exec_tracee, result: *mut USER_WORD) -> c_int
{
  let mut buffer: [c_char; PATH_MAX + 1] = [0; PATH_MAX + 1];   // C 1036: `char buffer[PATH_MAX + 1];`
  let mut address: USER_WORD;                            // C 1037: `USER_WORD address, return_buffer, size;`
  let mut return_buffer: USER_WORD;
  let mut size: USER_WORD;
  let mut length: usize;                                 // C 1038: `size_t length;`
  let mut proc_pid_exe: [c_char; 35] = [0; 35];          // C 1039: `char proc_pid_exe[sizeof "/proc//exe" + 24], *p;`
  let mut p: *mut c_char;
  let mut dirfd: c_int;                                  // C 1040: `int dirfd;`

  /* Read the file name.  */

  /* C 1047-1058: `#ifdef READLINK_SYSCALL` - the arm of the hosts that have `readlink`, and C's
     `#else`.  */
  #[cfg (feature = "readlink_syscall")]
  {
    if callno == READLINK_SYSCALL                        // C 1050
      {
	dirfd     = AT_FDCWD;                            // C 1052
	address   = (*regs).SYSCALL_ARG_REG;             // C 1053
	return_buffer = (*regs).SYSCALL_ARG1_REG;        // C 1054
	size      = (*regs).SYSCALL_ARG2_REG;            // C 1055
      }
    else
      {                                            /* C 1057-1063: the `#else` arm's four statements.  */
	dirfd     = (*regs).SYSCALL_ARG_REG as USER_SWORD as c_int;   // C 1059
	address   = (*regs).SYSCALL_ARG1_REG;                         // C 1060
	return_buffer = (*regs).SYSCALL_ARG2_REG;                     // C 1061
	size      = (*regs).SYSCALL_ARG3_REG;                         // C 1062
      }
  }
  #[cfg (not (feature = "readlink_syscall"))]
  {                                                  /* C 1057-1063: the `#else` arm's four statements.  */
    dirfd     = (*regs).SYSCALL_ARG_REG as USER_SWORD as c_int;   // C 1059
    address   = (*regs).SYSCALL_ARG1_REG;                         // C 1060
    return_buffer = (*regs).SYSCALL_ARG2_REG;                     // C 1061
    size      = (*regs).SYSCALL_ARG3_REG;                         // C 1062
  }

  read_memory (tracee, &raw mut buffer as *mut c_char, PATH_MAX, address);   // C 1065

  /* Make sure BUFFER is NULL terminated.  */
  if memchr ((&raw const buffer) as *const c_void, 0, PATH_MAX).is_null()   // C 1069
    {
      *errno () = ENAMETOOLONG;                          // C 1071: `errno = ENAMETOOLONG;`
      return 1;                                          // C 1072
    }

  /* Expand BUFFER into an absolute file name.  TODO:
     AT_SYMLINK_FOLLOW? */

  if canon_path (tracee, dirfd, &raw mut buffer as *mut c_char,
		 std::mem::size_of_val (&buffer) as ptrdiff_t) != 0   // C 1077
    { return 0; }                                        // C 1078

  /* Now check if the caller is looking for /proc/self/exe or its
     equivalent with the PID made explicit.

     dirfd can be ignored, as for now only absolute file names are
     handled.  FIXME.  */

  p = stpcpy (&raw mut proc_pid_exe as *mut c_char, c"/proc/".as_ptr ());   // C 1087
  p = format_pid (p, (*tracee).pid as c_uint);           // C 1088
  stpcpy (p, c"/exe".as_ptr ());                         // C 1089

  if (strcmp ((&raw const buffer) as *const c_char, c"/proc/self/exe".as_ptr ()) != 0
      && strcmp ((&raw const buffer) as *const c_char,
                 (&raw const proc_pid_exe) as *const c_char) != 0)         // C 1091-1093
      || (*tracee).exec_file.is_null ()
    { return 0; }                                        // C 1094

  /* Copy over tracee->exec_file.  Truncate it to PATH_MAX, length, or
     size, whichever is smaller.  */

  length = strlen ((*tracee).exec_file);                 // C 1099
  length = MIN! (size, MIN! (PATH_MAX, length));         // C 1100: `length = MIN (size, MIN (PATH_MAX, length));`
  strncpy (&raw mut buffer as *mut c_char, (*tracee).exec_file, length);   // C 1101

  if user_copy (tracee, (&raw const buffer) as *const c_uchar,
		return_buffer, length) != 0              // C 1103-1104
    {
      *errno () = EIO;                                   // C 1106: `errno = EIO;`
      return 1;                                          // C 1107
    }

  *result = length;                                      // C 1110: `*result = length;`
  return 2;                                              // C 1111
}

/// C 1122-1124: `static int handle_openat (USER_WORD callno, USER_REGS_STRUCT *regs, struct exec_tracee *tracee, USER_WORD *result)`.
#[cfg (feature = "have_android")]
unsafe fn handle_openat (callno: USER_WORD, regs: *mut USER_REGS_STRUCT,
                         tracee: *mut exec_tracee, result: *mut USER_WORD) -> c_int
{
  let mut buffer: [c_char; PATH_MAX + 1] = [0; PATH_MAX + 1];   // C 1126: `char buffer[PATH_MAX + 1];`
  let mut address: USER_WORD;                            // C 1127: `USER_WORD address;`
  let mut length: usize;                                 // C 1128: `size_t length;`
  let mut original: USER_REGS_STRUCT;                    // C 1129: `USER_REGS_STRUCT original;`
  let mut proc_pid_exe: [c_char; 35] = [0; 35];          // C 1130: `char proc_pid_exe[sizeof "/proc//exe" + 24], *p;`
  let mut p: *mut c_char;
  let mut dirfd: c_int;                                  // C 1131: `int dirfd;`

  /* Read the file name.  */

  /* C 1137-1146: `#ifdef OPEN_SYSCALL` - the arm of the hosts that have `open`, and C's `#else`.  */
  #[cfg (feature = "open_syscall")]
  {
    if callno == OPEN_SYSCALL                            // C 1140
      {
	dirfd   = AT_FDCWD;                              // C 1142
	address = (*regs).SYSCALL_ARG_REG;               // C 1143
      }
    else
      {                                            /* C 1145-1151: the `#else` arm's two statements.  */
	dirfd   = (*regs).SYSCALL_ARG_REG as USER_SWORD as c_int;   // C 1147
	address = (*regs).SYSCALL_ARG1_REG;                         // C 1148
      }
  }
  #[cfg (not (feature = "open_syscall"))]
  {                                                /* C 1145-1151: the `#else` arm's two statements.  */
    dirfd   = (*regs).SYSCALL_ARG_REG as USER_SWORD as c_int;       // C 1147
    address = (*regs).SYSCALL_ARG1_REG;                             // C 1148
  }

  /* Read the file name into the buffer and verify that it is NULL
     terminated.  */
  read_memory (tracee, &raw mut buffer as *mut c_char, PATH_MAX, address);   // C 1155

  if memchr ((&raw const buffer) as *const c_void, 0, PATH_MAX).is_null()   // C 1157
    {
      *errno () = ENAMETOOLONG;                          // C 1159
      return 1;                                          // C 1160
    }

  /* Expand BUFFER into an absolute file name.  TODO:
     AT_SYMLINK_FOLLOW? */

  if canon_path (tracee, dirfd, &raw mut buffer as *mut c_char,
		 std::mem::size_of_val (&buffer) as ptrdiff_t) != 0   // C 1165
    { return 0; }                                        // C 1166

  /* Now check if the caller is looking for /proc/self/exe or its
     equivalent with the PID made explicit.  */

  p = stpcpy (&raw mut proc_pid_exe as *mut c_char, c"/proc/".as_ptr ());   // C 1171
  p = format_pid (p, (*tracee).pid as c_uint);           // C 1172
  stpcpy (p, c"/exe".as_ptr ());                         // C 1173

  if (strcmp ((&raw const buffer) as *const c_char, c"/proc/self/exe".as_ptr ()) != 0
      && strcmp ((&raw const buffer) as *const c_char,
                 (&raw const proc_pid_exe) as *const c_char) != 0)         // C 1175-1177
      || (*tracee).exec_file.is_null ()
    { return 0; }                                        // C 1178

  /* Copy over tracee->exec_file.  This doesn't correctly handle the
     scenario where tracee->exec_file is longer than PATH_MAX, but
     that has yet to be encountered in practice.  */

  /* C's `fail:` label (C 1214) is the port's labeled-block tail.  */
  'fail: {
  original = *regs;                                      // C 1185: `original = *regs;`
  length   = strlen ((*tracee).exec_file);               // C 1186
  address  = user_alloca (tracee, &raw mut original, regs, length + 1);   // C 1187

  if address == 0
      || user_copy (tracee, (*tracee).exec_file as *const c_uchar,
		    address, length + 1) != 0            // C 1189-1191
    { break 'fail; }                                     // C 1192: `goto fail;`

  /* Replace the file name buffer with ADDRESS.  */

  /* C 1196-1199: `#ifdef OPEN_SYSCALL` - C 1196-1197's arm and C 1199's `else`.  */
  #[cfg (feature = "open_syscall")]
  {
    if callno == OPEN_SYSCALL                            // C 1197
      { (*regs).SYSCALL_ARG_REG = address; }
    else
      { (*regs).SYSCALL_ARG1_REG = address; }            // C 1199
  }
  #[cfg (not (feature = "open_syscall"))]
  { (*regs).SYSCALL_ARG1_REG = address; }                // C 1199

  /* C 1202-1208: the `__aarch64__` arm and its `#else`.  */
  #[cfg (feature = "aarch64")]
  {
    if aarch64_set_regs ((*tracee).pid, regs, false) != 0
      { break 'fail; }                                   // C 1204: `goto fail;`
  }
  #[cfg (not (feature = "aarch64"))]
  {
    if ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (), regs) != 0
      { break 'fail; }                                   // C 1206: `goto fail;`
  }

  /* Resume the system call.  */
  return 0;                                              // C 1212
  }

  /* fail: */
  *errno () = EIO;                                       // C 1215: `errno = EIO;`
  return 1;                                              // C 1216
}

/* C 1225-1512: `process_system_call`.  C's switch has two labels that share one body (C 1290's
   `#ifdef READLINK_SYSCALL` label and C 1292's `READLINKAT_SYSCALL`, C 1314's `OPEN_SYSCALL` and
   C 1316's `OPENAT_SYSCALL`), and five `goto`s into three tails (`report_syscall_error`,
   `emulate_syscall`/`common` and the `continue_syscall` case label).  Rust's `match` has no
   two-labels-one-arm form and no `goto`, so the port carries each label's arm with the body
   (the `#[cfg]` pairs below) and spells the three tails as a labelled block plus the loop
   C 1386-1397's `default:`/`continue_syscall:` pair needs (PORTING.md 2.2's spelling for `goto`).  */
/// C 1231: `static void process_system_call (struct exec_tracee *tracee)`.
#[cfg (feature = "have_android")]
unsafe fn process_system_call (tracee: *mut exec_tracee)
{
  let mut regs: USER_REGS_STRUCT = std::mem::zeroed ();  // C 1233: `USER_REGS_STRUCT regs;` (C's first write is `ptrace`/`aarch64_get_regs`)
  let mut rc: c_int;                                     // C 1234: `int rc, wstatus, save_errno;`
  let mut wstatus: c_int = 0;                            // C 1234: `int rc, wstatus, save_errno;` (C leaves it uninitialised; `waitpid` fills it)
  let mut save_errno: c_int;
  let mut callno: USER_WORD;                             // C 1235: `USER_WORD callno, sp;`
  let mut sp: USER_WORD;
  let mut result: USER_WORD = 0;                         // C 1236: `USER_WORD result;` (C's first write is a handler's out-parameter)
  let mut reporting_error: bool = false;                 // C 1237: `bool reporting_error;` (the two tails set it before any read)

  /* C 1239-1242: `#ifdef __aarch64__` and its `#else`.  */
  #[cfg (feature = "aarch64")]
  { rc = aarch64_get_regs ((*tracee).pid, &raw mut regs); }
  #[cfg (not (feature = "aarch64"))]
  { rc = ptrace (PTRACE_GETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (), &raw mut regs) as c_int; }

  /* TODO: what to do if this fails? */
  if rc < 0
    { return; }                                          // C 1245-1246

  /* Save the stack pointer.  */
  sp = regs.STACK_POINTER;                               // C 1249: `sp = regs.STACK_POINTER;`

  /* Now dispatch based on the system call.  If TRACEE->exec_data is
     set, this must be exec, whatever the value of SYSCALL_NUM_REG,
     which is erased when exec loads another image.  */

  /* C 1255-1258.  */
  callno = if (*tracee).exec_data.is_null ()
    { if !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT)
	{ regs.SYSCALL_NUM_REG } else { (*tracee).callno } }
    else { EXEC_SYSCALL };
  (*tracee).callno = callno;                             // C 1259: `tracee->callno = callno;`

  /* C 1426-1430's three labels are the port's one labelled block (`common`) plus the loop the
     `default:`/`continue_syscall:` pair becomes.  */
  'common: {
    'dispatch: loop
      {
      if callno == EXEC_SYSCALL                          // C 1262: `case EXEC_SYSCALL:`
	{
	  if !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT)   // C 1265
	    {
	      /* The outstanding syscall flag must not be inconsistent with
		 the presence of instructions for the loader.  */
	      assert! ((*tracee).exec_data.is_null ());    // C 1270: `assert! (!tracee->exec_data);`
	      rc = handle_exec (tracee, &raw mut regs);   // C 1271

	      if rc != 0
		{ reporting_error = true; break 'common; }   // C 1273-1275: `goto report_syscall_error;`

	      /* The process has been resumed.  Assert that the instructions
		 for loading this executable have been generated and
		 recorded, and set waiting_for_syscall.  */
	      exec_tracee_bit_set (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT, true);  // C 1280
	      assert! (!(*tracee).exec_data.is_null ());   // C 1281: `assert! (tracee->exec_data);`

	      /* Record the initial stack pointer also.  */
	      (*tracee).sp = sp;                          // C 1284
	    }
	  else
	    {
	      assert! (!(*tracee).exec_data.is_null ());   // C 1288
	      finish_exec (tracee, &raw mut regs);        // C 1289

	      /* The process has been resumed and has become capable of
		 executing independently.  */
	      exec_tracee_bit_set (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT, false);  // C 1293
	    }

	  break 'dispatch;                                 // C 1296: `break;`
	}

      /* C 1290-1292's `#ifdef READLINK_SYSCALL` label and its `case READLINKAT_SYSCALL`: one body,
	 carried once per label.  */
      #[cfg (feature = "readlink_syscall")]
      if callno == READLINK_SYSCALL || callno == READLINKAT_SYSCALL
	{
	  /* This system call is already in progress if
	     TRACEE->waiting_for_syscall is true.  */
	  if !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT)   // C 1302
	    {
	      /* Handle this readlinkat system call.  */
	      rc = handle_readlinkat (callno, &raw mut regs, tracee,
				      &raw mut result);            // C 1305-1306

	      /* rc means the same as in `handle_exec'.  */
	      if rc == 1
		{ reporting_error = true; break 'common; }     // C 1310: `goto report_syscall_error;`
	      else if rc == 2
		{ reporting_error = false; break 'common; }    // C 1312: `goto emulate_syscall;`
	    }

	  /* C 1315: `goto continue_syscall;` - the tail below, which this arm falls into.  */
	}
      #[cfg (not (feature = "readlink_syscall"))]
      if callno == READLINKAT_SYSCALL
	{
	  if !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT)   // C 1302
	    {
	      rc = handle_readlinkat (callno, &raw mut regs, tracee,
				      &raw mut result);            // C 1305-1306
	      if rc == 1
		{ reporting_error = true; break 'common; }     // C 1310
	      else if rc == 2
		{ reporting_error = false; break 'common; }    // C 1312
	    }

	  /* C 1315: `goto continue_syscall;` - the tail below.  */
	}

      /* C 1314-1316's `#ifdef OPEN_SYSCALL` label and its `case OPENAT_SYSCALL`: the same, and this
	 one runs on into C 1386's `default:`/`continue_syscall:` tail.  */
      #[cfg (feature = "open_syscall")]
      if callno == OPEN_SYSCALL || callno == OPENAT_SYSCALL
	{
	  /* This system call is already in progress if
	     TRACEE->waiting_for_syscall is true.  */
	  if !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT)   // C 1326
	    {
	      /* Handle this open system call.  */
	      rc = handle_openat (callno, &raw mut regs, tracee, &raw mut result);   // C 1329

	      /* rc means the same as in `handle_exec', except that `open'
		 is never emulated.  */
	      if rc == 1
		{ reporting_error = true; break 'common; }     // C 1336: `goto report_syscall_error;`

	      /* The stack pointer must be restored after it was modified
		 by `user_alloca'; record sp in TRACEE, which will be
		 restored after this system call completes.  */
	      (*tracee).sp = sp;                             // C 1342
	    }
	  else
	    {
	      /* Restore that stack pointer.  */
	      regs.STACK_POINTER = (*tracee).sp;             // C 1349

	      /* C 1350-1356: the `__aarch64__` arm and its `#else`.  */
	      #[cfg (feature = "aarch64")]
	      { if aarch64_set_regs ((*tracee).pid, &raw mut regs, false) != 0 { return; } }
	      #[cfg (not (feature = "aarch64"))]
	      { if ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (),
			   &raw mut regs) != 0 { return; } }
	    }

	  /* Fallthrough.  */
	}
      #[cfg (not (feature = "open_syscall"))]
      if callno == OPENAT_SYSCALL
	{
	  if !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT)   // C 1326
	    {
	      rc = handle_openat (callno, &raw mut regs, tracee, &raw mut result);   // C 1329
	      if rc == 1
		{ reporting_error = true; break 'common; }     // C 1336
	      (*tracee).sp = sp;                             // C 1342
	    }
	  else
	    {
	      regs.STACK_POINTER = (*tracee).sp;             // C 1349
	      #[cfg (feature = "aarch64")]
	      { if aarch64_set_regs ((*tracee).pid, &raw mut regs, false) != 0 { return; } }
	      #[cfg (not (feature = "aarch64"))]
	      { if ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (),
			   &raw mut regs) != 0 { return; } }
	    }
	}

      /* default:  */
      /* continue_syscall: (C 1396-1397) */
      /* Don't wait for the system call to finish; instead, the system
	 will DTRT upon the next call to PTRACE_SYSCALL after the
	 syscall-trap signal is delivered.  */

      /* C 1380-1395: `#ifdef OPEN_SYSCALL`'s term inside the request selection, and its `#else`.  */
      #[cfg (feature = "open_syscall")]
      { rc = ptrace (if use_seccomp_p
		     /* open and openat are not processed synchronously,
			nor can they afford to dispense with
			post-syscall finalization.  */
		     && ((callno != OPENAT_SYSCALL
			  && callno != OPEN_SYSCALL)
			 || !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT))
		     { PTRACE_CONT } else { PTRACE_SYSCALL }, (*tracee).pid,
		     std::ptr::null_mut::<c_void> (), std::ptr::null_mut::<c_void> ()) as c_int; }   // C 1390-1393
      #[cfg (not (feature = "open_syscall"))]
      { rc = ptrace (if use_seccomp_p
		     && (callno != OPENAT_SYSCALL
			 || !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT))
		     { PTRACE_CONT } else { PTRACE_SYSCALL }, (*tracee).pid,
		     std::ptr::null_mut::<c_void> (), std::ptr::null_mut::<c_void> ()) as c_int; }
      if rc < 0
	{ return; }                                          // C 1394-1395

      /* C 1397-1405: `#ifdef HAVE_SECCOMP`'s guard on the flag update, with C 1383-1385's
	 `#ifdef OPEN_SYSCALL` term inside it; the two `#[cfg]` arms below carry the two readings.  */
      #[cfg (all (feature = "have_seccomp", feature = "open_syscall"))]
      { if !(use_seccomp_p
	     && ((callno != OPENAT_SYSCALL
		  && callno != OPEN_SYSCALL)
		 || !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT)))
	  { exec_tracee_bit_set (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT,
				 !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT)); } }
      #[cfg (all (feature = "have_seccomp", not (feature = "open_syscall")))]
      { if !(use_seccomp_p
	     && (callno != OPENAT_SYSCALL
		 || !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT)))
	  { exec_tracee_bit_set (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT,
				 !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT)); } }
      #[cfg (not (feature = "have_seccomp"))]
      { exec_tracee_bit_set (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT,
			     !exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT)); }

      break 'dispatch;
      }

    return;                                              // C 1424: `return;`
  }

  /* report_syscall_error: (C 1426-1428) / emulate_syscall: (C 1430) / common: (C 1431) */
  /* Reporting an error or emulating a system call works by replacing
     the system call number with -1 or another nonexistent syscall,
     letting it continue, and then substituting errno for ENOSYS in the
     case of an error.

     Make sure that the stack pointer is restored to its original
     position upon exit, or bad things can happen.  */

  /* First, save errno; system calls below will clobber it.  */
  save_errno = *errno ();                                // C 1444: `save_errno = errno;`

  regs.SYSCALL_NUM_REG = -1isize as USER_WORD;           // C 1446: `regs.SYSCALL_NUM_REG = -1;`
  regs.STACK_POINTER   = sp;                             // C 1447: `regs.STACK_POINTER = sp;`

  /* C 1449-1470: the `__aarch64__` arm, and inside its `#else` the `__arm__` pair.  */
  #[cfg (feature = "aarch64")]
  {
    if aarch64_set_regs ((*tracee).pid, &raw mut regs, true) != 0
      { return; }                                        // C 1451-1452
  }
  #[cfg (not (feature = "aarch64"))]
  {
    #[cfg (feature = "arm")]
    {
      /* On ARM systems, a special request is used to update the system
	 call number as known to the kernel.  In addition, the system call
	 number must be valid, so use `tuxcall'.  Hopefully, nobody will
	 run this on a kernel with Tux.  */
      if ptrace (PTRACE_SET_SYSCALL, (*tracee).pid, std::ptr::null_mut::<c_void> (), 222) != 0
	{ return; }                                      // C 1459-1460
    }

    if ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (), &raw mut regs) != 0
      { return; }                                        // C 1463-1464
  }

  /* Do this invalid system call.  */
  if ptrace (PTRACE_SYSCALL, (*tracee).pid, std::ptr::null_mut::<c_void> (),
	     std::ptr::null_mut::<c_void> ()) != 0
    { return; }                                          // C 1467-1468

  /* C 1470's `again1:` label - the loop C 1473 and C 1492 jump back to.  */
  loop
    {
      rc = waitpid ((*tracee).pid, &raw mut wstatus, __WALL);   // C 1471
      if rc == -1 && *errno () == EINTR
	{ continue; }                                        // C 1472-1473: `goto again1;`

      /* Return if waitpid fails.  */
      if rc == -1
	{ return; }                                          // C 1477-1478

      /* If the process received a signal, see if the signal is SIGSYS and
	 from seccomp.  If so, discard it.  */

      if WIFSTOPPED (wstatus)                                // C 1485
	{
	  rc = check_signal (tracee, wstatus);               // C 1487

	  if rc == -1
	    { return; }                                      // C 1489-1490
	  else if rc != 0
	    { continue; }                                    // C 1492: `goto again1;`
	}
      break;
    }

  if !WIFSTOPPED (wstatus)
    /* The process has been killed in response to a signal.  In this
       case, simply unlink the tracee and return.  */
    { remove_tracee (tracee); }                          // C 1498-1500
  else if reporting_error
    {
      /* C 1504-1508: the `__mips__` arm and its `#else`.  */
      #[cfg (feature = "mips")]
      {
	/* MIPS systems place errno in v0 and set a3 to 1.  */
	regs.gregs[2] = save_errno as USER_WORD;          // C 1507
	regs.gregs[7] = 1;                                // C 1508
      }
      #[cfg (not (feature = "mips"))]
      { regs.SYSCALL_RET_REG = (-save_errno) as USER_WORD; }   // C 1510: `regs.SYSCALL_RET_REG = -save_errno;`

      /* Report errno.  */
      /* C 1515-1521: the `__aarch64__` arm and its `#else`, neither result checked.  */
      #[cfg (feature = "aarch64")]
      { aarch64_set_regs ((*tracee).pid, &raw mut regs, false); }
      #[cfg (not (feature = "aarch64"))]
      { ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (), &raw mut regs); }

      /* Now wait for the next system call to happen.  */
      ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
	      (*tracee).pid, std::ptr::null_mut::<c_void> (), std::ptr::null_mut::<c_void> ());   // C 1524-1525
    }
  else
    {
      /* No error is being reported.  Return the result in the
	 appropriate registers.  */
      /* C 1536-1540: the `__mips__` arm and its `#else`.  */
      #[cfg (feature = "mips")]
      {
	/* MIPS systems place errno in v0 and set a3 to 1.  */
	regs.gregs[2] = result;                           // C 1539
	regs.gregs[7] = 0;                                // C 1540
      }
      #[cfg (not (feature = "mips"))]
      { regs.SYSCALL_RET_REG = result; }                // C 1542: `regs.SYSCALL_RET_REG = result;`

      /* Report errno.  */
      /* C 1547-1553: the `__aarch64__` arm and its `#else`.  */
      #[cfg (feature = "aarch64")]
      { aarch64_set_regs ((*tracee).pid, &raw mut regs, false); }
      #[cfg (not (feature = "aarch64"))]
      { ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (), &raw mut regs); }

      /* Now wait for the next system call to happen.  */
      ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
	      (*tracee).pid, std::ptr::null_mut::<c_void> (), std::ptr::null_mut::<c_void> ());   // C 1556-1557
    }
}

/* ----------------------------------------------------------------------------------------------
   C 1514-1867: `#ifdef HAVE_SECCOMP` - the whole seccomp accelerator.  Everything in the block is
   behind the file gate and this one (P-86 1), and the block's own three `#ifndef` fallbacks are at
   its end.
   ---------------------------------------------------------------------------------------------- */

/// C 1522: `static bool kernel_4_7_or_earlier;` - C's tentative definition, i.e. `false`.
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
pub static mut kernel_4_7_or_earlier: bool = false;

/* C 1525-1537: `interesting_syscalls[]`, whose length is `ARRAYELTS` and whose elements C's own
   `#ifdef`s add.  C declares it `static int interesting_syscalls[] = {...}`; the port declares the
   four readings as `const` arrays (a `static`'s length is not a constant expression, and
   `ARRAYELTS` below needs one), each carrying the elements its arm has.  */
/// C: `static int interesting_syscalls[] = { EXEC_SYSCALL, OPEN_SYSCALL, OPENAT_SYSCALL, READLINK_SYSCALL, READLINKAT_SYSCALL, };`
#[cfg (all (feature = "have_android", feature = "have_seccomp",
            feature = "open_syscall", feature = "readlink_syscall"))]
pub const interesting_syscalls: [c_int; 5] =
  [EXEC_SYSCALL as c_int, OPEN_SYSCALL as c_int, OPENAT_SYSCALL as c_int,
   READLINK_SYSCALL as c_int, READLINKAT_SYSCALL as c_int];
#[cfg (all (feature = "have_android", feature = "have_seccomp",
            feature = "open_syscall", not (feature = "readlink_syscall")))]
pub const interesting_syscalls: [c_int; 4] =
  [EXEC_SYSCALL as c_int, OPEN_SYSCALL as c_int, OPENAT_SYSCALL as c_int, READLINKAT_SYSCALL as c_int];
#[cfg (all (feature = "have_android", feature = "have_seccomp",
            not (feature = "open_syscall"), feature = "readlink_syscall"))]
pub const interesting_syscalls: [c_int; 4] =
  [EXEC_SYSCALL as c_int, OPENAT_SYSCALL as c_int,
   READLINK_SYSCALL as c_int, READLINKAT_SYSCALL as c_int];
#[cfg (all (feature = "have_android", feature = "have_seccomp",
            not (feature = "open_syscall"), not (feature = "readlink_syscall")))]
pub const interesting_syscalls: [c_int; 3] =
  [EXEC_SYSCALL as c_int, OPENAT_SYSCALL as c_int, READLINKAT_SYSCALL as c_int];

/// C 1540: `#define ARRAYELTS(arr) (sizeof (arr) / sizeof (arr)[0])` - carried with C's name over
/// the construct that answers the same question for a Rust array.
/// C: `#define ARRAYELTS(arr) (sizeof (arr) / sizeof (arr)[0])`
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
macro_rules! ARRAYELTS {
  ($arr:expr) => { ($arr).len () };
}

/// C 1545-1546: `static int establish_seccomp_filter (void)`.
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
unsafe fn establish_seccomp_filter () -> c_int
{
  let mut statements: [sock_filter; 1 + interesting_syscalls.len () + 2] =
    [sock_filter { code: 0, jt: 0, jf: 0, k: 0 }; 1 + interesting_syscalls.len () + 2];  // C 1547
  let mut program: sock_fprog = std::mem::zeroed ();     // C 1548: `struct sock_fprog program;` (C fills it field by field)
  let mut index: c_int;                                  // C 1549: `int index, rc;`
  let mut rc: c_int;

  index = 0;                                             // C 1551: `index = 0;`

  /* As the exec wrapper will reject executables for an inappropriate
     architecture, verifying the same here would only be redundant.
     Proceed to load the current system call number.  */

  statements[index as usize] = BPF_STMT! (BPF_LD + BPF_W + BPF_ABS,
					  offsetof! (seccomp_data, nr) as c_uint);
  index += 1;                                            // C 1556-1558

  /* Search for system calls of interest.  */

  statements[index as usize] =
    BPF_JUMP! (BPF_JMP + BPF_JEQ + BPF_K, EXEC_SYSCALL as c_uint,
	       ARRAYELTS! (interesting_syscalls) as c_uchar, 0); index += 1;   // C 1562-1564
  #[cfg (feature = "open_syscall")]
  {
    statements[index as usize] =
      BPF_JUMP! (BPF_JMP + BPF_JEQ + BPF_K, OPEN_SYSCALL as c_uint,
		 (ARRAYELTS! (interesting_syscalls) - index as usize + 1) as c_uchar, 0);
    index += 1;                                          // C 1566-1569
  }
  statements[index as usize] =
    BPF_JUMP! (BPF_JMP + BPF_JEQ + BPF_K, OPENAT_SYSCALL as c_uint,
	       (ARRAYELTS! (interesting_syscalls) - index as usize + 1) as c_uchar, 0);
  index += 1;                                            // C 1570-1573
  #[cfg (feature = "readlink_syscall")]
  {
    statements[index as usize] =
      BPF_JUMP! (BPF_JMP + BPF_JEQ + BPF_K, READLINK_SYSCALL as c_uint,
		 (ARRAYELTS! (interesting_syscalls) - index as usize + 1) as c_uchar, 0);
    index += 1;                                          // C 1574-1577
  }
  statements[index as usize] =
    BPF_JUMP! (BPF_JMP + BPF_JEQ + BPF_K, READLINKAT_SYSCALL as c_uint,
	       (ARRAYELTS! (interesting_syscalls) - index as usize + 1) as c_uchar, 0);
  index += 1;                                            // C 1578-1581

  /* If not intercepted above, permit this system call to execute as
     normal.  */
  statements[index as usize] = BPF_STMT! (BPF_RET + BPF_K, SECCOMP_RET_ALLOW);
  index += 1;                                            // C 1585-1586
  statements[index as usize] = BPF_STMT! (BPF_RET + BPF_K, SECCOMP_RET_TRACE);
  index += 1;                                            // C 1587-1588

  rc = prctl (PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);           // C 1591
  if rc != 0
    { return 1; }                                        // C 1592-1593

  program.len = ARRAYELTS! (statements) as c_ushort;     // C 1595
  program.filter = &raw mut statements as *mut sock_filter;   // C 1596
  rc = prctl (PR_SET_SECCOMP, SECCOMP_MODE_FILTER as c_int, &raw mut program);   // C 1597
  if rc != 0
    { return 1; }                                        // C 1598-1599

  return 0;                                              // C 1601
}

/// C 1605-1606: `static void seccomp_system_call (struct exec_tracee *tracee)`.
#[cfg (all (feature = "have_android", feature = "have_seccomp"))]
unsafe fn seccomp_system_call (tracee: *mut exec_tracee)
{
  let mut regs: USER_REGS_STRUCT = std::mem::zeroed ();  // C 1607: `USER_REGS_STRUCT regs;` (C's first write is `ptrace`/`aarch64_get_regs`)
  let mut rc: c_int;                                     // C 1608: `int rc, wstatus, save_errno;`
  let mut wstatus: c_int = 0;                            // C 1608: `int rc, wstatus, save_errno;` (C leaves it uninitialised; `waitpid` fills it)
  let mut save_errno: c_int;
  let mut callno: USER_WORD;                             // C 1609: `USER_WORD callno, sp;`
  let mut sp: USER_WORD;
  let mut result: USER_WORD = 0;                         // C 1610: `USER_WORD result;`
  let mut reporting_error: bool = false;                 // C 1611: `bool reporting_error;`

  if kernel_4_7_or_earlier                               // C 1613
    {
      /* On kernel 4.7 and earlier, following a PTRACE_EVENT_SECCOMP by
	 a PTRACE_SYSCALL will give rise to a syscall-entry stop event,
	 and seccomp filters will be suppressed till the system call
	 runs its course.  */
      ptrace (PTRACE_SYSCALL, (*tracee).pid, 0, 0);      // C 1620
      return;                                            // C 1621
    }

  /* C 1624-1627: the `__aarch64__` arm and its `#else`.  */
  #[cfg (feature = "aarch64")]
  { rc = aarch64_get_regs ((*tracee).pid, &raw mut regs); }
  #[cfg (not (feature = "aarch64"))]
  { rc = ptrace (PTRACE_GETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (), &raw mut regs) as c_int; }

  /* TODO: what to do if this fails? */
  if rc < 0
    { return; }                                          // C 1630-1631

  /* On kernel 4.8, processes resumed after being paused so as to
     produce a PTRACE_EVENT_SECCOMP will execute till the system call
     completes, or indefinitely if resumed with PTRACE_CONT.

     In this context processes are resumed with PTRACE_CONT unless it is
     an `open' syscall that is being intercepted, which, if successfully
     intercepted, they must receive adjustments to their stack pointer
     upon completion of said system call.  */
  assert! (!exec_tracee_bit_p (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT));   // C 1642

  /* Save the stack pointer.  */
  sp = regs.STACK_POINTER;                               // C 1645

  /* Now dispatch based on the system call.  */
  callno = regs.SYSCALL_NUM_REG;                         // C 1648

  /* Record the call number, which may be required if one of the
     following handlers should arrange for process_system_call to
     intercede after the system call completes.  */
  (*tracee).callno = callno;                             // C 1654

  'common: {
    'dispatch: loop
      {
      if callno == EXEC_SYSCALL                          // C 1657
	{
	  assert! ((*tracee).exec_data.is_null ());       // C 1658
	  rc = handle_exec (tracee, &raw mut regs);      // C 1659

	  if rc != 0
	    { reporting_error = true; break 'common; }   // C 1662: `goto report_syscall_error;`

	  /* The process has been resumed.  Assert that the instructions for
	     loading this executable have been generated and recorded, and
	     set waiting_for_syscall.  */
	  exec_tracee_bit_set (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT, true);  // C 1668
	  assert! (!(*tracee).exec_data.is_null ());      // C 1669

	  /* Record the initial stack pointer also.  */
	  (*tracee).sp = sp;                             // C 1672
	  break 'dispatch;                               // C 1673: `break;`
	}

      /* C 1675-1687: the `#ifdef READLINK_SYSCALL` label and `case READLINKAT_SYSCALL`, one body
	 per label.  */
      #[cfg (feature = "readlink_syscall")]
      if callno == READLINK_SYSCALL || callno == READLINKAT_SYSCALL
	{
	  /* Handle this readlinkat system call.  */
	  rc = handle_readlinkat (callno, &raw mut regs, tracee, &raw mut result);   // C 1690-1691

	  /* rc means the same as in `handle_exec'.  */
	  if rc == 1
	    { reporting_error = true; break 'common; }   // C 1695: `goto report_syscall_error;`
	  else if rc == 2
	    { reporting_error = false; break 'common; }  // C 1697: `goto emulate_syscall;`

	  /* C 1699: `goto continue_syscall;` - the tail below.  */
	}
      #[cfg (not (feature = "readlink_syscall"))]
      if callno == READLINKAT_SYSCALL
	{
	  rc = handle_readlinkat (callno, &raw mut regs, tracee, &raw mut result);   // C 1690-1691
	  if rc == 1
	    { reporting_error = true; break 'common; }   // C 1695
	  else if rc == 2
	    { reporting_error = false; break 'common; }  // C 1697
	}

      /* C 1702-1704: the `#ifdef OPEN_SYSCALL` label and `case OPENAT_SYSCALL`; this arm does not
	 fall through (it `break`s after setting the flag).  */
      #[cfg (feature = "open_syscall")]
      if callno == OPEN_SYSCALL || callno == OPENAT_SYSCALL
	{
	  /* Handle this open system call.  */
	  rc = handle_openat (callno, &raw mut regs, tracee, &raw mut result);   // C 1707

	  /* rc means the same as in `handle_exec', except that `open'
	     is never emulated.  */
	  if rc == 1
	    { reporting_error = true; break 'common; }   // C 1714: `goto report_syscall_error;`

	  /* The stack pointer must be restored after it was modified
	     by `user_alloca'; record sp in TRACEE, which will be
	     restored after this system call completes.  */
	  (*tracee).sp = sp;                             // C 1720

	  /* As such, arrange to enter `process_system_call' on its
	     completion.  */
	  rc = ptrace (PTRACE_SYSCALL, (*tracee).pid,
		       std::ptr::null_mut::<c_void> (), std::ptr::null_mut::<c_void> ()) as c_int;   // C 1725-1726
	  if rc < 0
	    { return; }                                  // C 1727-1728

	  exec_tracee_bit_set (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT, true);  // C 1730
	  break 'dispatch;                               // C 1731: `break;`
	}
      #[cfg (not (feature = "open_syscall"))]
      if callno == OPENAT_SYSCALL
	{
	  rc = handle_openat (callno, &raw mut regs, tracee, &raw mut result);   // C 1707
	  if rc == 1
	    { reporting_error = true; break 'common; }   // C 1714
	  (*tracee).sp = sp;                             // C 1720
	  rc = ptrace (PTRACE_SYSCALL, (*tracee).pid,
		       std::ptr::null_mut::<c_void> (), std::ptr::null_mut::<c_void> ()) as c_int;   // C 1725-1726
	  if rc < 0
	    { return; }                                  // C 1727-1728
	  exec_tracee_bit_set (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT, true);  // C 1730
	  break 'dispatch;                               // C 1731
	}

      /* default:  */
      /* continue_syscall: (C 1734-1735) */
      rc = ptrace (PTRACE_CONT, (*tracee).pid, std::ptr::null_mut::<c_void> (),
		   std::ptr::null_mut::<c_void> ()) as c_int;      // C 1736
      if rc < 0
	{ return; }                                      // C 1737-1738
      break 'dispatch;
      }

    return;                                              // C 1741: `return;`
  }

  /* report_syscall_error: (C 1743-1745) / emulate_syscall: (C 1747) / common: (C 1748) */
  /* Reporting an error or emulating a system call works by replacing
     the system call number with -1 or another nonexistent syscall,
     letting it continue, and then substituting errno for ENOSYS in the
     case of an error.

     Make sure that the stack pointer is restored to its original
     position upon exit, or bad things can happen.  */

  /* First, save errno; system calls below will clobber it.  */
  save_errno = *errno ();                                // C 1761: `save_errno = errno;`

  regs.SYSCALL_NUM_REG = -1isize as USER_WORD;           // C 1763
  regs.STACK_POINTER   = sp;                             // C 1764

  /* C 1766-1787: the `__aarch64__` arm with its `__arm__` sibling inside the `#else`.  */
  #[cfg (feature = "aarch64")]
  {
    if aarch64_set_regs ((*tracee).pid, &raw mut regs, true) != 0
      { return; }                                        // C 1768-1769
  }
  #[cfg (not (feature = "aarch64"))]
  {
    #[cfg (feature = "arm")]
    {
      /* On ARM systems, a special request is used to update the system
	 call number as known to the kernel.  In addition, the system call
	 number must be valid, so use `tuxcall'.  Hopefully, nobody will
	 run this on a kernel with Tux.  */
      if ptrace (PTRACE_SET_SYSCALL, (*tracee).pid, std::ptr::null_mut::<c_void> (), 222) != 0
	{ return; }                                      // C 1776-1777
    }

    if ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (), &raw mut regs) != 0
      { return; }                                        // C 1780-1781
  }

  /* Do this invalid system call.  */
  if ptrace (PTRACE_SYSCALL, (*tracee).pid, std::ptr::null_mut::<c_void> (),
	     std::ptr::null_mut::<c_void> ()) != 0
    { return; }                                          // C 1784-1785

  /* C 1787's `again1:` label.  */
  loop
    {
      rc = waitpid ((*tracee).pid, &raw mut wstatus, __WALL);   // C 1788
      if rc == -1 && *errno () == EINTR
	{ continue; }                                        // C 1789-1790: `goto again1;`

      /* Return if waitpid fails.  */
      if rc == -1
	{ return; }                                          // C 1794-1795

      /* If the process received a signal, see if the signal is SIGSYS
	 and/or from seccomp.  If so, discard it.  */
      if WIFSTOPPED (wstatus)                                // C 1802
	{
	  rc = check_signal (tracee, wstatus);               // C 1804

	  if rc == -1
	    { return; }                                      // C 1806-1807
	  else if rc != 0
	    { continue; }                                    // C 1809: `goto again1;`
	}
      break;
    }

  if !WIFSTOPPED (wstatus)
    /* The process has been killed in response to a signal.  In this
       case, simply unlink the tracee and return.  */
    { remove_tracee (tracee); }                          // C 1815-1817
  else if reporting_error
    {
      /* C 1821-1825: the `__mips__` arm and its `#else`.  */
      #[cfg (feature = "mips")]
      {
	/* MIPS systems place errno in v0 and set a3 to 1.  */
	regs.gregs[2] = save_errno as USER_WORD;          // C 1824
	regs.gregs[7] = 1;                                // C 1825
      }
      #[cfg (not (feature = "mips"))]
      { regs.SYSCALL_RET_REG = (-save_errno) as USER_WORD; }   // C 1827

      /* Report errno.  */
      /* C 1832-1838: the `__aarch64__` arm and its `#else`.  */
      #[cfg (feature = "aarch64")]
      { aarch64_set_regs ((*tracee).pid, &raw mut regs, false); }
      #[cfg (not (feature = "aarch64"))]
      { ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (), &raw mut regs); }

      /* Resume the process till the next interception by its filter.  */
      ptrace (PTRACE_CONT, (*tracee).pid, std::ptr::null_mut::<c_void> (),
	      std::ptr::null_mut::<c_void> ());                    // C 1840
    }
  else
    {
      /* No error is being reported.  Return the result in the
	 appropriate registers.  */
      /* C 1849-1853: the `__mips__` arm and its `#else`.  */
      #[cfg (feature = "mips")]
      {
	/* MIPS systems place errno in v0 and set a3 to 1.  */
	regs.gregs[2] = result;                           // C 1852
	regs.gregs[7] = 0;                                // C 1853
      }
      #[cfg (not (feature = "mips"))]
      { regs.SYSCALL_RET_REG = result; }                // C 1855

      /* Report errno.  */
      /* C 1860-1866: the `__aarch64__` arm and its `#else`.  */
      #[cfg (feature = "aarch64")]
      { aarch64_set_regs ((*tracee).pid, &raw mut regs, false); }
      #[cfg (not (feature = "aarch64"))]
      { ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (), &raw mut regs); }

      /* Resume the process till the next interception by its filter.  */
      ptrace (PTRACE_CONT, (*tracee).pid, std::ptr::null_mut::<c_void> (),
	      std::ptr::null_mut::<c_void> ());                    // C 1868
    }
}

/* C 1871-1877: the three `#ifndef` fallbacks the seccomp block closes with.  Each is a pair whose
   other arm is the system header's (declared at C 61-69); the C text's whole point is that the
   name is defined after this point either way.  */
/// C: `#define PTRACE_EVENT_SECCOMP 7`
#[cfg (all (feature = "have_android", feature = "have_seccomp",
            not (feature = "ptrace_event_seccomp")))]
pub const PTRACE_EVENT_SECCOMP: c_int = 7;
/// C: `#define PTRACE_O_TRACESECCOMP (1 << PTRACE_EVENT_SECCOMP)`
#[cfg (all (feature = "have_android", feature = "have_seccomp",
            not (feature = "ptrace_o_traceseccomp")))]
pub const PTRACE_O_TRACESECCOMP: c_int = 1 << PTRACE_EVENT_SECCOMP;
/// C: `#define SIGSYS 31`
#[cfg (all (feature = "have_android", feature = "have_seccomp", not (feature = "sigsys")))]
pub const SIGSYS: c_int = 31;

/* C 1882-1883: `int tracing_execve (const char *file, char *const *argv, char *const *envp)`.  */
/// C 1882-1883: `int tracing_execve (const char *file, char *const *argv, char *const *envp)`.
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracing_execve (file: *const c_char, argv: *const *mut c_char,
                                          envp: *const *mut c_char) -> c_int
{
  let mut rc: c_int;                                     // C 1885: `int rc;`

  /* Start tracing self.  */
  rc = ptrace (PTRACE_TRACEME, 0, std::ptr::null_mut::<c_void> (),
	       std::ptr::null_mut::<c_void> ()) as c_int;   // C 1888
  if rc != 0
    { return rc; }                                       // C 1889-1890

  /* Notify the parent to enter signal-delivery-stop.  */
  raise (SIGSTOP);                                       // C 1893

  #[cfg (feature = "have_seccomp")]                      // C 1895-1898
  {
    /* Install the seccomp filter.  */
    if use_seccomp_p && establish_seccomp_filter () != 0
      { return 1; }                                      // C 1897
  }

  return execve (file, argv, envp);                      // C 1900
}

/* C 1907-1908: `int after_fork (pid_t pid)`.  Its `goto again` is the port's loop, and C 1952-2023's
   `#if defined HAVE_SECCOMP && __ANDROID__` block (with its `else` attached to it) is carried whole
   under those two features.  */
/// C 1907-1908: `int after_fork (pid_t pid)`.
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn after_fork (pid: pid_t) -> c_int
{
  let mut wstatus: c_int = 0;                            // C 1910: `int wstatus, rc, flags;` (C leaves it uninitialised; `waitpid` fills it)
  let mut rc: c_int;
  let mut flags: c_int;
  let mut tracee: *mut exec_tracee;                      // C 1911: `struct exec_tracee *tracee;`
  #[cfg (all (feature = "have_seccomp", feature = "__android__"))]   // C 1913: `#if defined HAVE_SECCOMP && __ANDROID__`
  let mut statusarg: c_int;                              // C 1914: `int statusarg;`
  #[cfg (all (feature = "have_seccomp", feature = "__android__"))]
  let mut regs: USER_REGS_STRUCT;                        // C 1915: `USER_REGS_STRUCT regs;`

  /* First, wait for something to happen to PID.  */
  /* C 1918's `again:` label.  */
  loop
    {
      rc = waitpid (pid, &raw mut wstatus, __WALL);      // C 1919
      if rc != pid && *errno () == EINTR
	{ continue; }                                    // C 1920-1921: `goto again;`
      break;
    }

  if rc != pid
    { return 1; }                                        // C 1923-1924

  /* If the child exited (or in general wasn't traced), return 2.  */
  if !WIFSTOPPED (wstatus)
    { return 2; }                                        // C 1928-1929

  /* Apply required options to the child, so that the kernel
     automatically traces children and makes it easy to differentiate
     between system call traps and other kinds of traps.  */

  flags  = PTRACE_O_TRACECLONE;                          // C 1935
  flags |= PTRACE_O_TRACEVFORK;                          // C 1936
  flags |= PTRACE_O_TRACEFORK;                           // C 1937
  flags |= PTRACE_O_TRACESYSGOOD;                        // C 1938
  flags |= PTRACE_O_TRACEEXIT;                           // C 1939
  #[cfg (feature = "have_seccomp")]                      // C 1941-1944
  {
    if use_seccomp_p
      { flags |= PTRACE_O_TRACESECCOMP; }
  }

  rc = ptrace (PTRACE_SETOPTIONS, pid, 0, flags) as c_int;   // C 1946

  if rc != 0
    {
      /* If the kernel can't trace child processes upon creation and
	 exit, then it can't work reliably.  */
      ptrace (PTRACE_DETACH, pid, 0, 0);                 // C 1952
      return 1;                                          // C 1953
    }

  /* C 1955-2023: `#if defined HAVE_SECCOMP && __ANDROID__` - the Android-kernel probe - and the
     `else` arm that C's `#endif` leaves attached to the plain resumption below.  */
  #[cfg (all (feature = "have_seccomp", feature = "__android__"))]
  {
    /* Certain Android kernels have received backports of those new
       PTRACE_EVENT_SECCOMP semantics which were introduced in kernel
       version 4.8, so that it is necessary to actively establish which
       variant is in place.  */

    if kernel_4_7_or_earlier && use_seccomp_p                // C 1962
      {
	/* Request that the child stop upon the next `exec' system call,
	   one of which is assumed to always be issued by the child, as
	   below, but await the next stop, and examine its contents.
	   Anciently, the syscall-stop preceded events marked
	   PTRACE_EVENT_SECCOMP, whereas this sequence is reversed in
	   4.8+, and in releases with these changes backported.  */

	rc = ptrace (PTRACE_SYSCALL, pid, 0, 0);          // C 1972
	if rc != 0
	  { return 1; }                                  // C 1973-1974

	loop                                             // C 1976: `while (true)`
	  {
	    rc = waitpid (pid, &raw mut wstatus, __WALL); // C 1978
	    if rc != pid
	      { return 1; }                              // C 1979-1980

	    if WIFSTOPPED (wstatus)                      // C 1982
	      {
		/* Verify that this system call is `exec', not one issued
		   between PTRACE_TRACEME and `exec' intercepted by
		   PTRACE_SYSCALL.  */
		/* C 1987-1990: the `__aarch64__` arm and its `#else`.  */
		#[cfg (feature = "aarch64")]
		{ rc = aarch64_get_regs (pid, &raw mut regs); }
		#[cfg (not (feature = "aarch64"))]
		{ rc = ptrace (PTRACE_GETREGS, pid, std::ptr::null_mut::<c_void> (),
			       &raw mut regs) as c_int; }
		if rc != 0
		  { return 1; }                              // C 1991-1992

		if regs.SYSCALL_NUM_REG == EXEC_SYSCALL       // C 1994
		  {
		    statusarg = (wstatus & 0xfff00) >> 8;    // C 1996

		    if statusarg == (SIGTRAP | (PTRACE_EVENT_SECCOMP << 8))    // C 1998
		      {
			/* The first event to be delivered is a seccomp
			   stop, indicating that this is an unmodified
			   <4.7 kernel.  Return to await the subsequent
			   syscall-stop, which should be received and
			   acted on by process_system_call.  */
			rc = ptrace (PTRACE_SYSCALL, pid, 0, 0);   // C 2006
			break;                                   // C 2007
		      }
		    else if statusarg == (SIGTRAP | 0x80)        // C 2009
		      {
			/* Syscall-traps take priority.  This is a
			   doctored 4.7 kernel.  */
			kernel_4_7_or_earlier = false;           // C 2013
			rc = ptrace (PTRACE_CONT, pid, 0, 0);    // C 2014
			break;                                   // C 2015
		      }
		  }

		rc = ptrace (PTRACE_SYSCALL, pid, 0, 0);     // C 2018
		if rc != 0
		  { return 1; }                              // C 2019-2020
	      }
	    else
	      { return 1; }                                // C 2023
	  }
      }
    else
      {
	/* Request that the child stop upon the next system call, or the
	   next filter event.  */
	rc = ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
		     pid, 0, 0);                         // C 2028-2029
      }
  }
  #[cfg (not (all (feature = "have_seccomp", feature = "__android__")))]
  {
    /* C 2025-2029's `else` arm - the one C's `#endif` leaves attached to the plain statement.  */
    rc = ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
		 pid, 0, 0) as c_int;
  }
  if rc != 0
    { return 1; }                                        // C 2030-2031

  /* Enroll the child into `tracing_processes'.  */

  if !free_tracees.is_null ()                            // C 2035
    {
      tracee = free_tracees;                             // C 2037
      free_tracees = (*free_tracees).next;               // C 2038
    }
  else
    { tracee = malloc (std::mem::size_of::<exec_tracee> ()) as *mut exec_tracee; }   // C 2040

  if tracee.is_null ()
    { return 1; }                                        // C 2042-2043

  (*tracee).pid = pid;                                   // C 2045
  (*tracee).callno = 0;                                  // C 2046
  (*tracee).next = tracing_processes;                    // C 2047
  exec_tracee_bit_set (tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT, false);   // C 2048
  exec_tracee_bit_set (tracee, EXEC_TRACEE_NEW_CHILD_BIT, false);             // C 2049
  (*tracee).exec_file = std::ptr::null_mut ();           // C 2050
  (*tracee).exec_data = std::ptr::null_mut ();           // C 2051
  (*tracee).data_size = 0;                               // C 2052
  tracing_processes = tracee;                            // C 2053
  return 0;                                              // C 2054
}

/* C 2059-2060: `pid_t exec_waitpid (pid_t pid, int *wstatus, int options)`.  */
/// C 2059-2060: `pid_t exec_waitpid (pid_t pid, int *wstatus, int options)`.
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exec_waitpid (pid: pid_t, wstatus: *mut c_int,
                                       options: c_int) -> pid_t
{
  let mut status: c_int = 0;                             // C 2062: `int status;` (C leaves it uninitialised; `waitpid` fills it)
  let mut tracee: *mut exec_tracee;                      // C 2063: `struct exec_tracee *tracee;`
  let mut siginfo: siginfo_t = std::mem::zeroed ();      // C 2064: `siginfo_t siginfo;` (C leaves it uninitialised; `PTRACE_GETSIGINFO` fills it)

  let mut pid = waitpid (pid, &raw mut status, options | __WALL);   // C 2066: `pid = waitpid (pid, &status, options | __WALL);`
  if pid < 0
    { return pid; }                                      // C 2067-2068

  /* Copy status into *WSTATUS if specified.  */
  if !wstatus.is_null ()                                 // C 2071: `if (wstatus)`
    { *wstatus = status; }                               // C 2072: `*wstatus = status;`

  /* WIFSTOPPED (status) means that the process has been stopped in
     response to a system call.  Find its tracee and process the
     system call.  */

  if WIFSTOPPED (status)                                 // C 2078
    {
      tracee = find_tracee (pid);                        // C 2080

      if tracee.is_null ()
	  || exec_tracee_bit_p (tracee, EXEC_TRACEE_NEW_CHILD_BIT)   // C 2082: `if (!tracee || tracee->new_child)`
	{
	  if WSTOPSIG (status) == SIGSTOP                    // C 2084
	    /* A new process has been created and stopped.  Record
	       it now.  */
	    { handle_clone (tracee, pid); }                  // C 2087

	  return -1;                                         // C 2089
	}

      /* Now extract the stop signal, including ptrace event bits.  */
      let mut status = status & 0xfff00;                 // C 2092: `status &= 0xfff00;`
      status = status >> 8;                              // C 2093: `status = status >> 8;`

      match status
	{
	  c if c == SIGTRAP =>                           // C 2097: `case SIGTRAP:`
	    {
	      /* Now, use PTRACE_GETSIGINFO to determine whether or not the
		 signal was delivered in response to a system call.  */

	      if ptrace (PTRACE_GETSIGINFO, pid, 0, &raw mut siginfo) != 0
		{ return -1; }                           // C 2102-2103

	      if !syscall_trap_p (&raw mut siginfo)      // C 2105
		{
		  if siginfo.si_code < 0
		    /* SIGTRAP delivered from userspace.  Pass it on.  */
		    { ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
			      pid, 0, SIGTRAP); }        // C 2108-2109
		  else
		    { ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
			      pid, 0, 0); }              // C 2111-2112

		  return -1;                             // C 2114
		}
	      /* C 2116-2117: `case SIGTRAP | 0x80:` - the fall-through target of the arm above,
		 "system call traps".  */
	      process_system_call (tracee);              // C 2119
	      return -1;                                 // C 2120
	    }
	  c if c == SIGTRAP | 0x80 =>                        // C 2116: `case SIGTRAP | 0x80:`
	    {
	      /* Otherwise, process the system call and continue waiting.  */
	      process_system_call (tracee);              // C 2119
	      return -1;                                 // C 2120
	    }

	  c if c == SIGTRAP | (PTRACE_EVENT_EXIT << 8) =>    // C 2122: `case SIGTRAP | (PTRACE_EVENT_EXIT << 8):`
	    {
	      /* The tracee has exited.  Make it finish correctly.  */
	      ptrace (PTRACE_SYSCALL, pid, 0, 0);        // C 2125
	      remove_tracee (tracee);                    // C 2126
	      return -1;                                 // C 2127
	    }

	  /* C 2129-2131's three labels: `PTRACE_EVENT_FORK`, `PTRACE_EVENT_VFORK` and
	     `PTRACE_EVENT_CLONE` all arrive at one body.  */
	  c if c == SIGTRAP | (PTRACE_EVENT_FORK << 8)         // C 2129
	    || c == SIGTRAP | (PTRACE_EVENT_VFORK << 8)        // C 2130
	    || c == SIGTRAP | (PTRACE_EVENT_CLONE << 8) =>     // C 2131
	    {
	      /* Both PTRACE_EVENT_CLONE and SIGSTOP must arrive before a
		 process is continued.  Otherwise, its parent's cmdline
		 cannot be obtained and propagated.

		 If the PID of the new process is currently not being
		 traced, create a new tracee.  Set `new_child' to true,
		 and copy over the old command line in preparation for a
		 SIGSTOP signal being delivered to it.

		 Otherwise, start the tracee running until the next
		 syscall.  */

	      handle_clone_prepare (tracee);               // C 2149

	      /* These events are handled by tracing SIGSTOP signals sent
		 to unknown tracees.  Make sure not to pass through
		 status, as there's no signal really being delivered.  */
	      ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
		      pid, 0, 0);                          // C 2155-2156
	      return -1;                                   // C 2157
	    }

	  #[cfg (feature = "have_seccomp")]                // C 2159-2163
	  c if c == SIGTRAP | (PTRACE_EVENT_SECCOMP << 8) =>
	    {
	      /* Intercept and process this system call if the event was
		 produced by our filter.  */
	      seccomp_system_call (tracee);                // C 2166
	      return -1;                                   // C 2167
	    }

	  /* C 2170-2189: `#ifdef SIGSYS`'s arm, with its `#ifdef HAVE_SIGINFO_T_SI_SYSCALL` and
	     `#ifndef __arm__` pair inside.  */
	  #[cfg (feature = "sigsys")]
	  c if c == SIGSYS =>                              // C 2172: `case SIGSYS:`
	    {
	      if ptrace (PTRACE_GETSIGINFO, pid, 0, &raw mut siginfo) != 0
		{ return -1; }                           // C 2174-2175

	      /* Continue the process until the next syscall, but don't
		 pass through the signal if an emulated syscall led to
		 it.  */
	      #[cfg (feature = "have_siginfo_t_si_syscall")]
	      {
		#[cfg (not (feature = "arm"))]
		{ ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
			  pid, 0, if siginfo.si_code == SYS_SECCOMP   // C 2180-2183
				  && siginfo.si_syscall == -1
			    { 0 } else { status }); }
		#[cfg (feature = "arm")]
		{ ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
			  pid, 0, if siginfo.si_code == SYS_SECCOMP   // C 2185-2188
				  && siginfo.si_syscall == 222
			    { 0 } else { status }); }
	      }
	      #[cfg (not (feature = "have_siginfo_t_si_syscall"))]
	      {
		/* Drop this signal, since what caused it is unknown.  */
		ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL }, pid,
			0, 0);                           // C 2192-2193
	      }
	      return -1;                                 // C 2195
	    }

	  _ =>                                             // C 2197: `default:`
	    {
	      /* Resume the process as appropriate.  */
	      ptrace (if use_seccomp_p { PTRACE_CONT } else { PTRACE_SYSCALL },
		      pid, 0, status);                   // C 2199-2200
	      return -1;                                 // C 2201
	    }
	}
    }
  else
    {
      /* The process has exited.  Unlink the associated tracee.  */
      tracee = find_tracee (pid);                        // C 2207

      if !tracee.is_null ()                              // C 2209
	{ remove_tracee (tracee); }                      // C 2210

      return pid;                                        // C 2212
    }
}

/* C 2220-2221: `void exec_init (const char *loader)`.  */
/// C 2220-2221: `void exec_init (const char *loader)`.
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exec_init (loader: *const c_char)
{
  loader_name = loader;                                // C 2228: `loader_name = loader;`

  #[cfg (feature = "have_seccomp")]                      // C 2222-2225
  {
    let mut u: utsname = std::mem::zeroed ();            // C 2223: `struct utsname u;` (C's first write is `uname`)
    let mut major: c_int = 0;                            // C 2224: `int major, minor;` (C's first write is `sscanf`)
    let mut minor: c_int = 0;

    *errno () = 0;                                       // C 2230: `errno = 0;`
    prctl (PR_GET_SECCOMP);                              // C 2231

    /* PR_GET_SECCOMP should not set errno if the kernel was configured
       with support for seccomp.  */
    if *errno () == 0                                    // C 2235: `if (!errno)`
      { use_seccomp_p = true; }                          // C 2236
    else                                                 // C 2237
      { return; }                                        // C 2238

    /* Establish whether the kernel is 4.7.x or older.  */
    uname (&raw mut u);                                  // C 2241: `uname (&u);`
    if sscanf ((&raw const u.release) as *const c_char, c"%d.%d".as_ptr (),
	       &raw mut major, &raw mut minor) == 2      // C 2242
      {
	/* Certain required ptrace features were introduced in kernel
	   3.5.  */
	if major < 3 || (major == 3 && minor < 5)        // C 2247
	  { use_seccomp_p = false; }                     // C 2248
	else
	  {
	    if major < 4 || (major == 4 && minor <= 7)   // C 2252
	      { kernel_4_7_or_earlier = true; }          // C 2253
	  }
      }
  }

  /* C 2256-2263: `#if defined HAVE_SYS_UIO_H && !defined HAVE_PROCESS_VM` - the two `dlsym`
     lookups, written through the address of each pointer as C writes them.  */
  #[cfg (all (feature = "have_sys_uio_h", not (feature = "have_process_vm")))]
  {
    *((&raw mut process_vm_readv) as *mut *mut c_void)
      = dlsym (RTLD_DEFAULT, c"process_vm_readv".as_ptr ());      // C 2258-2259
    *((&raw mut process_vm_writev) as *mut *mut c_void)
      = dlsym (RTLD_DEFAULT, c"process_vm_writev".as_ptr ());     // C 2260-2261
  }
}
