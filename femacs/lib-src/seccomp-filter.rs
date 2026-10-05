// femacs: port of lib-src/seccomp-filter.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same
// error model.  C's `static` helpers stay plain `unsafe fn` except the one whose address C hands
// to `atexit`, which is `unsafe extern "C" fn` (PORTING.md 1.4); the one non-`static` C
// definition, `main`, carries `#[unsafe(no_mangle)]` (PORTING.md 0.2).
//
// **This is one of P-86's class: the frozen configuration compiles nothing from this file, and
// the gate is the build's own, not a `#if` in the text.**  `lib-src/Makefile.in` 232-246 sets
// `SECCOMP_FILTER=1` only when `HAVE_SECCOMP=yes` (configure.ac 5918-5927: `<linux/seccomp.h>`,
// `<linux/filter.h>` and the two `SECCOMP_*` declarations) and `HAVE_LIBSECCOMP=yes`
// (configure.ac 5929: `libseccomp >= 2.5.2`) and `uname -m` is `x86_64` and `SIZEOF_LONG` is 8
// and the kernel is at least 4.14; C 248-250 then puts the program in `DONT_INSTALL`, and
// `DONT_INSTALL` is where the file is built.  This target is darwin: the frozen build's
// `docs/baseline/frozen-build-sources.txt` records no `lib-src/seccomp-filter.o` (its batch-F
// section names this file among those that appear in no list at all), `tools/extract-abi.py`'s
// checklist for it is empty (P-86 2), and the port's frozen object is the 336-byte empty one
// (Evidence 1).  The **file gate is `feature = "seccomp_filter"`** - the lower-case of the
// Makefile's own variable, exactly as `lib-src/update-game-score.rs`'s is `use_gamedir` - on
// every item *and* on the include graph (P-86 8).
//
// Three sub-gates, each the lower-case of the macro C's own text tests, both arms carried
// (P-86 1):
//
//   * C 65-67's `#ifndef ARCH_CET_STATUS` / `#define ARCH_CET_STATUS 0x3001` -> the C 49
//     `<asm/prctl.h>` section carries the arm where the header defines it
//     (`feature = "arch_cet_status"`) and C 65-67 carries the fallback
//     (`not (feature = "arch_cet_status")`);
//   * C 69-72's `#ifndef MAP_DROPPABLE` / `#define MAP_DROPPABLE 0x08` -> the same pair, with
//     `feature = "map_droppable"` (`<sys/mman.h>`'s arm is the C 51 section);
//   * C 259-261's `#ifdef __NR_faccessat2` -> `feature = "__nr_faccessat2"` (the `#endif` has no
//     `#else`, so the other arm is empty in C: nothing is transcribed for it).
//
// **`text-only`** (P-86 3-2): this machine is darwin.  There is **no x86-64 GNU/Linux
// toolchain** (`which x86_64-linux-gnu-gcc` is empty), **no glibc header tree**
// (`ls /usr/include/linux` -> `No such file or directory`), and the program's own library,
// **libseccomp (`<seccomp.h>`, >= 2.5.2), is not installed** - so nothing here is compiled
// against the target the Makefile gate requires and no read of this unit can be anything but a
// compile reading under this machine's toolchain.  What the round *does* show is that both arms
// of every gate type-check under the pinned `rustc` (Evidence 2-3) and that those readings are
// real (Evidence 4).
//
// `<assert.h>`, `<errno.h>`, `<limits.h>`, `<stdarg.h>`, `<stdcountof.h>`, `<stdlib.h>`,
// `<stdint.h>`, `<stdio.h>`, `<asm/termbits.h>`, `<time.h>`, `<asm/prctl.h>`, `<sys/ioctl.h>`,
// `<sys/mman.h>`, `<sys/prctl.h>`, `<sys/types.h>`, `<sys/stat.h>`, `<linux/futex.h>`,
// `<linux/filter.h>`, `<linux/seccomp.h>`, `<fcntl.h>`, `<sched.h>`, `<seccomp.h>` and
// `<unistd.h>` are *system* headers of that absent toolchain; the reference tree has no file for
// any of them, so this unit declares what its text reads, at the `#include` line that brings them
// (P-88 1) - `<config.h>` alone is consumed as the port's mirror (C 36).  The Linux half is
// measured, not remembered: the constants and the `struct scmp_arg_cmp` layout below are clang
// readings against the **NDK's x86-64 Linux sysroot** (a kernel-ABI + x86-64 ABI measurement; the
// command, the paths and the readings are in `docs/ports/seccomp-filter.md`'s Evidence), and the
// libseccomp half against **libseccomp v2.5.2's own `include/seccomp.h.in` and
// `include/seccomp-syscalls.h`** (fetched, since the library is not installed; its version is the
// one configure.ac 5929 asks for at minimum).  Which of the two halves is measured and which
// values are unverifiable is recorded in the ledger's Deviations and Owed sections.
//
// The tree's mirrors do carry some of these names (`lib/include/fcntl.in.rs` 99-185,
// `lib/include/limits.in.rs` 35-49, `lib/include/stdio.rs`, `lib/include/stdlib.rs`,
// `lib/include/errno.rs`, `lib/include/unistd.in.rs` and `lib/include/stdint.in.rs`), but their
// values are the **frozen target's** (darwin's) and this program is built only for x86-64
// GNU/Linux, so consuming them would put the wrong flag values into the filter: `O_CLOEXEC` is
// the worked example - the mirror's measured 0x1000000 against Linux's 0x80000, both in this
// unit's Evidence.  The unit therefore declares the x86-64 Linux values itself and leaves the
// mirrors in their own files (deviation 1).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_assignments,
         unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn, unused_parens,
         unused_comparisons, overflowing_literals, unused_must_use, clippy::all)]

use std::ffi::{c_char, c_int, c_long, c_longlong, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");                          // C 36: `#include <config.h>`

/* Generate a Secure Computing filter definition file.

   This program creates a small Secure Computing filter usable for a typical minimal Emacs sandbox.
   See the man page for `seccomp' for details about Secure Computing filters.  This program
   requires the `libseccomp' library.  However, the resulting filter file requires only a Linux
   kernel supporting the Secure Computing extension.

   Usage:

     seccomp-filter out.bpf out.pfc out-exec.bpf out-exec.pfc

   This writes the raw `struct sock_filter' array to out.bpf and a human-readable representation
   to out.pfc.  Additionally, it writes variants of those files that can be used to sandbox Emacs
   before 'execve' to out-exec.bpf and out-exec.pfc.  (C 21-34.)  */

// ---------------------------------------------------------------------------------------------
// C 38-63: the system headers.  Each section is the stand-in for the header C's `#include` line
// brings (P-88 1), in C's order; the file gate is on every item (P-86 8).
// ---------------------------------------------------------------------------------------------

/* C 38: `#include <assert.h>`.  `conf_post.h` 28-29 derives `NDEBUG` from the absent
   `ENABLE_CHECKING`, and `docs/baseline/config/darwin-arm64.macros.tsv` 2892 records the frozen
   configuration's `NDEBUG`, so C's `assert` is `((void) 0)` and does not evaluate its operand
   (PORTING.md 2.5).  The port keeps C's name and call shape with that frozen body (the
   `lib-src/ebrowse.rs` 93 reading; deviation 2).  */
/// C: <assert.h>: `#define assert(expr) ((void) 0)` (the `NDEBUG` arm)
#[cfg (feature = "seccomp_filter")]
macro_rules! assert {
  ($expr:expr) => { () };
}

/* `static_assert` is C23's keyword and the port's own macro (PORTING.md 2.5: a
   `const`-item assertion at the call site, written with the path-qualified `core::assert!` so no
   `macro_rules!` named `assert` can shadow it - status 3.432).  */
/// C: `static_assert (cond)` (C23's keyword)
#[cfg (feature = "seccomp_filter")]
macro_rules! static_assert {
  ($cond:expr) => {
    const _: () = core::assert! ($cond);
  };
}

/* C 39: `#include <errno.h>`.  */
#[cfg (feature = "seccomp_filter")]
unsafe extern "C" {
  /// C: <errno.h> (glibc): `extern int *__errno_location (void) __THROW;`
  pub fn __errno_location () -> *mut c_int;
}
/// C: <errno.h>: `#define errno (*__errno_location ())` - the port's accessor keeps C's name at
/// the assignment sites (`*errno () = error;`), the `lib/include/errno.rs` 17 shape, and
/// `__errno_location` below it is the x86-64 Linux spelling of the accessor (this machine's is
/// `__error ()`).
#[cfg (feature = "seccomp_filter")]
pub unsafe fn errno () -> *mut c_int { __errno_location () }
/// C: <errno.h>: `#define EPERM 1`
#[cfg (feature = "seccomp_filter")]
pub const EPERM: c_int = 1;
/// C: <errno.h>: `#define EACCES 13`
#[cfg (feature = "seccomp_filter")]
pub const EACCES: c_int = 13;
/// C: <errno.h>: `#define EINVAL 22`
#[cfg (feature = "seccomp_filter")]
pub const EINVAL: c_int = 22;
/// C: <errno.h>: `#define EINTR 4`
#[cfg (feature = "seccomp_filter")]
pub const EINTR: c_int = 4;

/* C 40: `#include <limits.h>`.  */
/// C: <limits.h>: `#define CHAR_BIT 8`
#[cfg (feature = "seccomp_filter")]
pub const CHAR_BIT: c_int = 8;
/// C: <limits.h>: `#define INT_MIN (-2147483647 - 1)`
#[cfg (feature = "seccomp_filter")]
pub const INT_MIN: c_int = -2147483648;
/// C: <limits.h>: `#define INT_MAX 2147483647`
#[cfg (feature = "seccomp_filter")]
pub const INT_MAX: c_int = 2147483647;
/// C: <limits.h>: `#define LONG_MIN (-LONG_MAX - 1L)`
#[cfg (feature = "seccomp_filter")]
pub const LONG_MIN: c_long = c_long::MIN;
/// C: <limits.h>: `#define LONG_MAX 9223372036854775807L`
#[cfg (feature = "seccomp_filter")]
pub const LONG_MAX: c_long = c_long::MAX;

/* C 41: `#include <stdarg.h>`.  Nothing is declared for it: C's `va_list ap; va_start (ap,
   format); ... va_end (ap);` have no port site - the `...` parameter *is* the list (PORTING.md
   P-04, the `lib-src/make-docfile.rs` 128-140 reading).  */

/* C 42: `#include <stdcountof.h>`.  */
/// C: <stdcountof.h>: `#define countof(a) (sizeof (a) / sizeof *(a))`
#[cfg (feature = "seccomp_filter")]
macro_rules! countof {
  ($a:expr) => { core::mem::size_of_val (&$a) / core::mem::size_of_val (&$a[0]) };
}

/* C 43: `#include <stdlib.h>`.  */
#[cfg (feature = "seccomp_filter")]
unsafe extern "C" {
  /// C: <stdlib.h>: `_Noreturn void exit (int status);`
  pub fn exit (status: c_int) -> !;
  /// C: <stdlib.h>: `int atexit (void (*function) (void));`
  pub fn atexit (function: unsafe extern "C" fn ()) -> c_int;
}
/// C: <stdlib.h>: `#define EXIT_FAILURE 1`
#[cfg (feature = "seccomp_filter")]
pub const EXIT_FAILURE: c_int = 1;

/* C 44: `#include <stdint.h>`.  */
/// C: <stdint.h>: `typedef unsigned int uint32_t;` (the x86-64 Linux ABI)
#[cfg (feature = "seccomp_filter")]
pub type uint32_t = c_uint;
/// C: <stdint.h>: `typedef unsigned long uintptr_t;` (the x86-64 Linux ABI)
#[cfg (feature = "seccomp_filter")]
pub type uintptr_t = usize;
/// C: <stdint.h>: `#define INT32_MIN (-2147483647 - 1)`
#[cfg (feature = "seccomp_filter")]
pub const INT32_MIN: c_int = -2147483648;
/// C: <stdint.h>: `#define INT32_MAX 2147483647`
#[cfg (feature = "seccomp_filter")]
pub const INT32_MAX: c_int = 2147483647;
/// C: <stdint.h>: `#define INT64_MIN (-9223372036854775807LL - 1)`
#[cfg (feature = "seccomp_filter")]
pub const INT64_MIN: c_longlong = c_longlong::MIN;
/// C: <stdint.h>: `#define INT64_MAX 9223372036854775807LL`
#[cfg (feature = "seccomp_filter")]
pub const INT64_MAX: c_longlong = c_longlong::MAX;

/* C 45: `#include <stdio.h>`.  */
/// C: <stdio.h>: `typedef struct _IO_FILE FILE;` - C's `FILE` is incomplete here, and only
/// pointers to it appear (PORTING.md 0.1-2's opaque-pointer reading, the `lib/include/stdio.rs`
/// convention of keeping C's name).
#[cfg (feature = "seccomp_filter")]
#[repr (C)]
pub struct _IO_FILE { _private: [u8; 0] }
#[cfg (feature = "seccomp_filter")]
pub type FILE = _IO_FILE;
#[cfg (feature = "seccomp_filter")]
unsafe extern "C" {
  /// C: <stdio.h>: `extern FILE *stderr;`
  pub static mut stderr: *mut FILE;
  /// C: <stdio.h>: `int vfprintf (FILE *restrict stream, const char *restrict format, va_list
  /// args);` - the `va_list` parameter is the port's `VaList<'_>` (the
  /// `lib/include/stdio.in.rs` 290-291 spelling, P-04).
  pub fn vfprintf (stream: *mut FILE, format: *const c_char, args: core::ffi::VaList<'_>)
    -> c_int;
  /// C: <stdio.h>: `int fputc (int c, FILE *stream);`
  pub fn fputc (c: c_int, stream: *mut FILE) -> c_int;
  /// C: <stdio.h>: `int fputs (const char *restrict s, FILE *restrict stream);`
  pub fn fputs (s: *const c_char, stream: *mut FILE) -> c_int;
  /// C: <stdio.h>: `void perror (const char *s);`
  pub fn perror (s: *const c_char);
  /// C: <stdio.h>: `int fflush (FILE *stream);`
  pub fn fflush (stream: *mut FILE) -> c_int;
}

/* C 46: `#include <asm/termbits.h>`  (mandatory accordingly to latest ioctl_tty(2)).  */
/// C: <asm/termbits.h>: `#define TCGETS 0x5401`
#[cfg (feature = "seccomp_filter")]
pub const TCGETS: c_int = 0x5401;

/* C 47: `#include <time.h>`.  */
/// C: <time.h>: `#define CLOCK_REALTIME 0`
#[cfg (feature = "seccomp_filter")]
pub const CLOCK_REALTIME: c_int = 0;

/* C 49: `#include <asm/prctl.h>`.  */
/// C: <asm/prctl.h>: `#define ARCH_SET_FS 0x1002`
#[cfg (feature = "seccomp_filter")]
pub const ARCH_SET_FS: c_int = 0x1002;
/// C: <asm/prctl.h>: `#define ARCH_CET_STATUS 0x3001` - the arm where the system header defines
/// the name; C 65-67's `#ifndef` carries the other one.  (UNVERIFIED as a header reading: neither
/// this machine nor the NDK sysroot's `asm/prctl.h` defines it - see the ledger's Owed 2.)
#[cfg (all (feature = "seccomp_filter", feature = "arch_cet_status"))]
pub const ARCH_CET_STATUS: c_int = 0x3001;

/* C 50: `#include <sys/ioctl.h>`.  */
/// C: <asm/ioctls.h> (through <sys/ioctl.h>): `#define TIOCGPGRP 0x540F`
#[cfg (feature = "seccomp_filter")]
pub const TIOCGPGRP: c_int = 0x540f;

/* C 51: `#include <sys/mman.h>`.  */
/// C: <asm/mman.h>: `#define PROT_NONE 0x0`
#[cfg (feature = "seccomp_filter")]
pub const PROT_NONE: c_int = 0x0;
/// C: <asm/mman.h>: `#define PROT_READ 0x1`
#[cfg (feature = "seccomp_filter")]
pub const PROT_READ: c_int = 0x1;
/// C: <asm/mman.h>: `#define PROT_WRITE 0x2`
#[cfg (feature = "seccomp_filter")]
pub const PROT_WRITE: c_int = 0x2;
/// C: <asm/mman.h>: `#define PROT_EXEC 0x4`
#[cfg (feature = "seccomp_filter")]
pub const PROT_EXEC: c_int = 0x4;
/// C: <asm/mman.h>: `#define MAP_SHARED 0x01`
#[cfg (feature = "seccomp_filter")]
pub const MAP_SHARED: c_int = 0x01;
/// C: <asm/mman.h>: `#define MAP_PRIVATE 0x02`
#[cfg (feature = "seccomp_filter")]
pub const MAP_PRIVATE: c_int = 0x02;
/// C: <asm/mman.h>: `#define MAP_FILE 0`
#[cfg (feature = "seccomp_filter")]
pub const MAP_FILE: c_int = 0;
/// C: <asm/mman.h>: `#define MAP_ANONYMOUS 0x20`
#[cfg (feature = "seccomp_filter")]
pub const MAP_ANONYMOUS: c_int = 0x20;
/// C: <asm/mman.h>: `#define MAP_FIXED 0x10`
#[cfg (feature = "seccomp_filter")]
pub const MAP_FIXED: c_int = 0x10;
/// C: <asm/mman.h>: `#define MAP_DENYWRITE 0x0800`
#[cfg (feature = "seccomp_filter")]
pub const MAP_DENYWRITE: c_int = 0x0800;
/// C: <asm/mman.h>: `#define MAP_STACK 0x20000`
#[cfg (feature = "seccomp_filter")]
pub const MAP_STACK: c_int = 0x020000;
/// C: <asm/mman.h>: `#define MAP_NORESERVE 0x4000`
#[cfg (feature = "seccomp_filter")]
pub const MAP_NORESERVE: c_int = 0x4000;
/// C: <asm/mman.h>: `#define MAP_DROPPABLE 0x08` - the arm where the system header defines the
/// name; C 69-72's `#ifndef` carries the other one.
#[cfg (all (feature = "seccomp_filter", feature = "map_droppable"))]
pub const MAP_DROPPABLE: c_int = 0x08;

/* C 52: `#include <sys/prctl.h>`.  */
/// C: <linux/prctl.h>: `#define PR_SET_NAME 15`
#[cfg (feature = "seccomp_filter")]
pub const PR_SET_NAME: c_int = 15;
/// C: <linux/prctl.h>: `#define PR_CAPBSET_READ 23`
#[cfg (feature = "seccomp_filter")]
pub const PR_CAPBSET_READ: c_int = 23;
/// C: <linux/prctl.h>: `#define PR_SET_NO_NEW_PRIVS 38`
#[cfg (feature = "seccomp_filter")]
pub const PR_SET_NO_NEW_PRIVS: c_int = 38;

/* C 53: `#include <sys/types.h>`.  The file's text names nothing from it.  */
/* C 54: `#include <sys/stat.h>`.  The file's text names nothing from it (`0644` at C 150 is an
   integer literal, not a name).  */

/* C 55: `#include <linux/futex.h>`.  */
/// C: <linux/futex.h>: `#define FUTEX_WAKE 1`
#[cfg (feature = "seccomp_filter")]
pub const FUTEX_WAKE: c_int = 1;
/// C: <linux/futex.h>: `#define FUTEX_PRIVATE_FLAG 128`
#[cfg (feature = "seccomp_filter")]
pub const FUTEX_PRIVATE_FLAG: c_int = 128;
/// C: <linux/futex.h>: `#define FUTEX_WAKE_PRIVATE (FUTEX_WAKE | FUTEX_PRIVATE_FLAG)`
#[cfg (feature = "seccomp_filter")]
pub const FUTEX_WAKE_PRIVATE: c_int = FUTEX_WAKE | FUTEX_PRIVATE_FLAG;

/* C 56: `#include <linux/filter.h>`.  The file's text names nothing from it (`struct
   sock_filter` appears only in C 21-34's description).  */

/* C 57: `#include <linux/seccomp.h>`.  */
/// C: <linux/seccomp.h>: `#define SECCOMP_SET_MODE_FILTER 1`
#[cfg (feature = "seccomp_filter")]
pub const SECCOMP_SET_MODE_FILTER: c_int = 1;
/// C: <linux/seccomp.h>: `#define SECCOMP_FILTER_FLAG_TSYNC (1UL << 0)`
#[cfg (feature = "seccomp_filter")]
pub const SECCOMP_FILTER_FLAG_TSYNC: c_ulong = 1 << 0;

/* C 58: `#include <fcntl.h>`.  */
#[cfg (feature = "seccomp_filter")]
unsafe extern "C" {
  /// C: <fcntl.h>: `int open (const char *file, int oflag, ...);`
  pub fn open (file: *const c_char, oflag: c_int, ...) -> c_int;
}
/// C: <fcntl.h>: `#define O_RDONLY 0`
#[cfg (feature = "seccomp_filter")]
pub const O_RDONLY: c_int = 0o0;
/// C: <fcntl.h>: `#define O_WRONLY 01`
#[cfg (feature = "seccomp_filter")]
pub const O_WRONLY: c_int = 0o1;
/// C: <fcntl.h>: `#define O_RDWR 02`
#[cfg (feature = "seccomp_filter")]
pub const O_RDWR: c_int = 0o2;
/// C: <fcntl.h>: `#define O_CREAT 0100`
#[cfg (feature = "seccomp_filter")]
pub const O_CREAT: c_int = 0o100;
/// C: <fcntl.h>: `#define O_TRUNC 01000`
#[cfg (feature = "seccomp_filter")]
pub const O_TRUNC: c_int = 0o1000;
/// C: <fcntl.h>: `#define O_CLOEXEC 02000000`
#[cfg (feature = "seccomp_filter")]
pub const O_CLOEXEC: c_int = 0o2000000;
/// C: <fcntl.h>: `#define O_PATH 010000000`
#[cfg (feature = "seccomp_filter")]
pub const O_PATH: c_int = 0o10000000;
/// C: <fcntl.h>: `#define O_DIRECTORY 00200000`
#[cfg (feature = "seccomp_filter")]
pub const O_DIRECTORY: c_int = 0o200000;
/// C: <fcntl.h>: `#define O_NOFOLLOW 00400000`
#[cfg (feature = "seccomp_filter")]
pub const O_NOFOLLOW: c_int = 0o400000;
/// C: <fcntl.h>: `#define O_BINARY 0` (the gnulib wrapper `lib/fcntl.in.h` supplies the name
/// where the system header has none; `lib/include/fcntl.in.rs` 185 measured the same 0).
#[cfg (feature = "seccomp_filter")]
pub const O_BINARY: c_int = 0;
/// C: <fcntl.h>: `#define F_GETFL 3`
#[cfg (feature = "seccomp_filter")]
pub const F_GETFL: c_int = 3;

/* C 59: `#include <sched.h>`.  */
/// C: <sched.h>: `#define CLONE_VM 0x00000100`
#[cfg (feature = "seccomp_filter")]
pub const CLONE_VM: c_int = 0x00000100;
/// C: <sched.h>: `#define CLONE_FS 0x00000200`
#[cfg (feature = "seccomp_filter")]
pub const CLONE_FS: c_int = 0x00000200;
/// C: <sched.h>: `#define CLONE_FILES 0x00000400`
#[cfg (feature = "seccomp_filter")]
pub const CLONE_FILES: c_int = 0x00000400;
/// C: <sched.h>: `#define CLONE_SIGHAND 0x00000800`
#[cfg (feature = "seccomp_filter")]
pub const CLONE_SIGHAND: c_int = 0x00000800;
/// C: <sched.h>: `#define CLONE_THREAD 0x00010000`
#[cfg (feature = "seccomp_filter")]
pub const CLONE_THREAD: c_int = 0x00010000;
/// C: <sched.h>: `#define CLONE_SYSVSEM 0x00040000`
#[cfg (feature = "seccomp_filter")]
pub const CLONE_SYSVSEM: c_int = 0x00040000;
/// C: <sched.h>: `#define CLONE_SETTLS 0x00080000`
#[cfg (feature = "seccomp_filter")]
pub const CLONE_SETTLS: c_int = 0x00080000;
/// C: <sched.h>: `#define CLONE_PARENT_SETTID 0x00100000`
#[cfg (feature = "seccomp_filter")]
pub const CLONE_PARENT_SETTID: c_int = 0x00100000;
/// C: <sched.h>: `#define CLONE_CHILD_CLEARTID 0x00200000`
#[cfg (feature = "seccomp_filter")]
pub const CLONE_CHILD_CLEARTID: c_int = 0x00200000;

/* C 60: `#include <seccomp.h>` - the program's own library.  Its surface is declared here
   (P-88 1) with libseccomp v2.5.2's own shapes and values (the version configure.ac 5929
   requires at minimum; the header is not installed on this machine, so it was read from the
   library's `include/seccomp.h.in` and `include/seccomp-syscalls.h` - Evidence 5).  */
/// C: <seccomp.h>: `typedef void *scmp_filter_ctx;` - C's `const scmp_filter_ctx` parameters
/// (seccomp_export_bpf/pfc) lose their constness the way every other pointer parameter of the
/// port does (PORTING.md 0.1-2).
#[cfg (feature = "seccomp_filter")]
pub type scmp_filter_ctx = *mut c_void;

/// C: <seccomp.h>: `typedef uint64_t scmp_datum_t;`
#[cfg (feature = "seccomp_filter")]
pub type scmp_datum_t = u64;

/// C: <seccomp.h>: `enum scmp_filter_attr`; the port keeps C's ABI (`int`, PORTING.md 1.3) and
/// names the three members the file's text reads.
#[cfg (feature = "seccomp_filter")]
pub type scmp_filter_attr = c_int;
/// C: <seccomp.h>: `SCMP_FLTATR_ACT_BADARCH = 2`
#[cfg (feature = "seccomp_filter")]
pub const SCMP_FLTATR_ACT_BADARCH: scmp_filter_attr = 2;
/// C: <seccomp.h>: `SCMP_FLTATR_CTL_NNP = 3`
#[cfg (feature = "seccomp_filter")]
pub const SCMP_FLTATR_CTL_NNP: scmp_filter_attr = 3;
/// C: <seccomp.h>: `SCMP_FLTATR_CTL_TSYNC = 4`
#[cfg (feature = "seccomp_filter")]
pub const SCMP_FLTATR_CTL_TSYNC: scmp_filter_attr = 4;

/// C: <seccomp.h>: `enum scmp_compare`; the three members the file's text reads.
#[cfg (feature = "seccomp_filter")]
pub type scmp_compare = c_int;
/// C: <seccomp.h>: `SCMP_CMP_NE = 1`
#[cfg (feature = "seccomp_filter")]
pub const SCMP_CMP_NE: scmp_compare = 1;
/// C: <seccomp.h>: `SCMP_CMP_EQ = 4`
#[cfg (feature = "seccomp_filter")]
pub const SCMP_CMP_EQ: scmp_compare = 4;
/// C: <seccomp.h>: `SCMP_CMP_MASKED_EQ = 7`
#[cfg (feature = "seccomp_filter")]
pub const SCMP_CMP_MASKED_EQ: scmp_compare = 7;

/// C: <seccomp.h>: `struct scmp_arg_cmp` - `unsigned int arg; enum scmp_compare op;
/// scmp_datum_t datum_a; scmp_datum_t datum_b;`, measured at `sizeof` 24 with `arg`@0, `op`@4,
/// `datum_a`@8 and `datum_b`@16 on x86-64 (Evidence 6).  `Copy` because C's compound literals
/// are copied into the array `RULE` builds (PORTING.md 0.1-9).
#[cfg (feature = "seccomp_filter")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct scmp_arg_cmp {
  pub arg: c_uint,
  pub op: scmp_compare,
  pub datum_a: scmp_datum_t,
  pub datum_b: scmp_datum_t,
}

/// C: <seccomp.h>: `#define SCMP_ACT_KILL_PROCESS 0x80000000U`
#[cfg (feature = "seccomp_filter")]
pub const SCMP_ACT_KILL_PROCESS: c_uint = 0x80000000;
/// C: <seccomp.h>: `#define SCMP_ACT_ALLOW 0x7fff0000U`
#[cfg (feature = "seccomp_filter")]
pub const SCMP_ACT_ALLOW: c_uint = 0x7fff0000;
/// C: <seccomp.h>: `#define SCMP_ACT_ERRNO(x) (0x00050000U | ((x) & 0x0000ffffU))`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_ACT_ERRNO {
  ($x:expr) => { 0x00050000u32 | (($x) as c_uint & 0x0000ffffu32) };
}

/* C: <seccomp.h>: `#define SCMP_SYS(x) (__SNR_##x)`, whose `__SNR_*` names
   `include/seccomp-syscalls.h` defines as `__NR_*` when the architecture has the syscall and as
   `__PNR_*` when it has not.  Rust has no token pasting (the `comp.rs` DEF_DLL_FN reading), so
   the one macro keeps C's name and shape and each of the 72 names the file's text passes gets
   its own arm, carrying the number that name has on **x86-64** - the only architecture
   `lib-src/Makefile.in` 234 builds this program for.  The 66 kernel-ABI numbers were measured
   from `<asm/unistd_64.h>`, and the six pseudo numbers the architecture has no syscall for
   (`fcntl64` -10009, `fstatat64` -10011, `lstat64` -10028, `sigaction` -10054, `sigprocmask`
   -10057, `stat64` -10062) from libseccomp's own `__PNR_*` definitions (Evidence 5).  */
/// C: <seccomp.h>: `#define SCMP_SYS(x) (__SNR_##x)`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_SYS {
  (access) => { 21 };
  (arch_prctl) => { 158 };
  (brk) => { 12 };
  (clock_gettime) => { 228 };
  (clone) => { 56 };
  (clone3) => { 435 };
  (close) => { 3 };
  (dup) => { 32 };
  (dup2) => { 33 };
  (eventfd) => { 284 };
  (eventfd2) => { 290 };
  (execve) => { 59 };
  (exit) => { 60 };
  (exit_group) => { 231 };
  (faccessat) => { 269 };
  (faccessat2) => { 439 };
  (fcntl) => { 72 };
  (fcntl64) => { -10009 };
  (fstat) => { 5 };
  (fstatat64) => { -10011 };
  (futex) => { 202 };
  (getcwd) => { 79 };
  (geteuid) => { 107 };
  (getpgrp) => { 111 };
  (getpid) => { 39 };
  (getrandom) => { 318 };
  (getrlimit) => { 97 };
  (gettid) => { 186 };
  (gettimeofday) => { 96 };
  (getuid) => { 102 };
  (ioctl) => { 16 };
  (lseek) => { 8 };
  (lstat) => { 6 };
  (lstat64) => { -10028 };
  (mmap) => { 9 };
  (mprotect) => { 10 };
  (munmap) => { 11 };
  (newfstatat) => { 262 };
  (open) => { 2 };
  (openat) => { 257 };
  (pidfd_open) => { 434 };
  (pipe) => { 22 };
  (pipe2) => { 293 };
  (poll) => { 7 };
  (prctl) => { 157 };
  (pread64) => { 17 };
  (prlimit64) => { 302 };
  (read) => { 0 };
  (readlink) => { 89 };
  (readlinkat) => { 267 };
  (rseq) => { 334 };
  (rt_sigaction) => { 13 };
  (rt_sigprocmask) => { 14 };
  (sched_getaffinity) => { 204 };
  (sched_getscheduler) => { 145 };
  (seccomp) => { 317 };
  (set_robust_list) => { 273 };
  (set_tid_address) => { 218 };
  (sigaction) => { -10054 };
  (sigaltstack) => { 131 };
  (sigprocmask) => { -10057 };
  (socket) => { 41 };
  (stat) => { 4 };
  (stat64) => { -10062 };
  (statfs) => { 137 };
  (time) => { 201 };
  (timer_create) => { 222 };
  (timerfd_create) => { 283 };
  (umask) => { 95 };
  (uname) => { 63 };
  (wait4) => { 61 };
  (write) => { 1 };
}

/* C: <seccomp.h>: `#define SCMP_CMP64(...) ((struct scmp_arg_cmp){__VA_ARGS__})` - C's compound
   literal zero-fills the members `__VA_ARGS__` does not name, and widens them to `scmp_datum_t`;
   the port writes both explicitly (the `as scmp_datum_t` is C's implicit conversion).  */
/// C: <seccomp.h>: `#define SCMP_CMP64(...) ((struct scmp_arg_cmp){__VA_ARGS__})`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_CMP64 {
  ($arg:expr, $op:expr, $a:expr) => {
    scmp_arg_cmp { arg: $arg, op: $op, datum_a: ($a) as scmp_datum_t, datum_b: 0 }
  };
  ($arg:expr, $op:expr, $a:expr, $b:expr) => {
    scmp_arg_cmp { arg: $arg, op: $op, datum_a: ($a) as scmp_datum_t,
                   datum_b: ($b) as scmp_datum_t }
  };
}
/* C: <seccomp.h>: `#define SCMP_CMP32(x, y, ...)` -> `_SCMP_CMP32_{1,2}(x, y, ...)` ->
   `SCMP_CMP64(x, y, (uint32_t)(z), (uint32_t)(q))`: the two `(uint32_t)` casts are C's own.  */
/// C: <seccomp.h>: `#define SCMP_CMP32(x, y, ...) _SCMP_MACRO_DISPATCHER(_SCMP_CMP32_, ...)`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_CMP32 {
  ($arg:expr, $op:expr, $a:expr) => {
    SCMP_CMP64! ($arg, $op, ($a as u32) as scmp_datum_t)
  };
  ($arg:expr, $op:expr, $a:expr, $b:expr) => {
    SCMP_CMP64! ($arg, $op, ($a as u32) as scmp_datum_t, ($b as u32) as scmp_datum_t)
  };
}
/// C: <seccomp.h>: `#define SCMP_A0_32(x, ...) SCMP_CMP32(0, x, __VA_ARGS__)`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_A0_32 { ($($a:expr),* $(,)?) => { SCMP_CMP32! (0, $($a),*) }; }
/// C: <seccomp.h>: `#define SCMP_A1_32(x, ...) SCMP_CMP32(1, x, __VA_ARGS__)`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_A1_32 { ($($a:expr),* $(,)?) => { SCMP_CMP32! (1, $($a),*) }; }
/// C: <seccomp.h>: `#define SCMP_A2_32(x, ...) SCMP_CMP32(2, x, __VA_ARGS__)`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_A2_32 { ($($a:expr),* $(,)?) => { SCMP_CMP32! (2, $($a),*) }; }
/// C: <seccomp.h>: `#define SCMP_A3_32(x, ...) SCMP_CMP32(3, x, __VA_ARGS__)`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_A3_32 { ($($a:expr),* $(,)?) => { SCMP_CMP32! (3, $($a),*) }; }
/// C: <seccomp.h>: `#define SCMP_A0_64(...) SCMP_CMP64(0, __VA_ARGS__)`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_A0_64 { ($($a:expr),* $(,)?) => { SCMP_CMP64! (0, $($a),*) }; }
/// C: <seccomp.h>: `#define SCMP_A1_64(...) SCMP_CMP64(1, __VA_ARGS__)`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_A1_64 { ($($a:expr),* $(,)?) => { SCMP_CMP64! (1, $($a),*) }; }
/// C: <seccomp.h>: `#define SCMP_A2_64(...) SCMP_CMP64(2, __VA_ARGS__)`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_A2_64 { ($($a:expr),* $(,)?) => { SCMP_CMP64! (2, $($a),*) }; }
/// C: <seccomp.h>: `#define SCMP_A3_64(...) SCMP_CMP64(3, __VA_ARGS__)`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_A3_64 { ($($a:expr),* $(,)?) => { SCMP_CMP64! (3, $($a),*) }; }
/// C: <seccomp.h>: `#define SCMP_A4_64(...) SCMP_CMP64(4, __VA_ARGS__)`
#[cfg (feature = "seccomp_filter")]
macro_rules! SCMP_A4_64 { ($($a:expr),* $(,)?) => { SCMP_CMP64! (4, $($a),*) }; }

#[cfg (feature = "seccomp_filter")]
unsafe extern "C" {
  /// C: <seccomp.h>: `scmp_filter_ctx seccomp_init (uint32_t def_action);`
  pub fn seccomp_init (def_action: u32) -> scmp_filter_ctx;
  /// C: <seccomp.h>: `void seccomp_release (scmp_filter_ctx ctx);`
  pub fn seccomp_release (ctx: scmp_filter_ctx);
  /// C: <seccomp.h>: `int seccomp_attr_set (scmp_filter_ctx ctx, enum scmp_filter_attr attr,
  /// uint32_t value);`
  pub fn seccomp_attr_set (ctx: scmp_filter_ctx, attr: scmp_filter_attr, value: u32) -> c_int;
  /// C: <seccomp.h>: `int seccomp_rule_add (scmp_filter_ctx ctx, uint32_t action, int syscall,
  /// unsigned int arg_cnt, ...);`
  pub fn seccomp_rule_add (ctx: scmp_filter_ctx, action: u32, syscall: c_int, arg_cnt: c_uint, ...)
    -> c_int;
  /// C: <seccomp.h>: `int seccomp_rule_add_array (scmp_filter_ctx ctx, uint32_t action, int
  /// syscall, unsigned int arg_cnt, const struct scmp_arg_cmp *arg_array);`
  pub fn seccomp_rule_add_array (ctx: scmp_filter_ctx, action: u32, syscall: c_int,
                                 arg_cnt: c_uint, arg_array: *const scmp_arg_cmp) -> c_int;
  /// C: <seccomp.h>: `int seccomp_export_pfc (const scmp_filter_ctx ctx, int fd);`
  pub fn seccomp_export_pfc (ctx: scmp_filter_ctx, fd: c_int) -> c_int;
  /// C: <seccomp.h>: `int seccomp_export_bpf (const scmp_filter_ctx ctx, int fd);`
  pub fn seccomp_export_bpf (ctx: scmp_filter_ctx, fd: c_int) -> c_int;
}

/* C 61: `#include <unistd.h>`.  */
#[cfg (feature = "seccomp_filter")]
unsafe extern "C" {
  /// C: <unistd.h>: `int close (int fd);`
  pub fn close (fd: c_int) -> c_int;
}
/// C: <unistd.h>: `#define STDIN_FILENO 0`
#[cfg (feature = "seccomp_filter")]
pub const STDIN_FILENO: c_int = 0;
/// C: <unistd.h>: `#define STDERR_FILENO 2`
#[cfg (feature = "seccomp_filter")]
pub const STDERR_FILENO: c_int = 2;

/* C 63: `#include <attribute.h>`.  `ATTRIBUTE_FORMAT_PRINTF` is an attribute with no Rust
   counterpart (PORTING.md 8/P-13), so the C name stays in `fail`'s comment and nothing is
   declared here.  */

/* C 65-67: `#ifndef ARCH_CET_STATUS` / `#define ARCH_CET_STATUS 0x3001` / `#endif` - the arm a
   system header without the name selects; C 49's `<asm/prctl.h>` section carries the other.  */
/// C: #define ARCH_CET_STATUS 0x3001
#[cfg (all (feature = "seccomp_filter", not (feature = "arch_cet_status")))]
pub const ARCH_CET_STATUS: c_int = 0x3001;

/* C 69-72: `#ifndef MAP_DROPPABLE` / `#define MAP_DROPPABLE 0x08` / `#endif` - the arm a system
   header without the flag selects; C 51's `<sys/mman.h>` section carries the other.  */
/// C: #define MAP_DROPPABLE 0x08
#[cfg (all (feature = "seccomp_filter", not (feature = "map_droppable")))]
pub const MAP_DROPPABLE: c_int = 0x08;

// ---------------------------------------------------------------------------------------------
// The file's own text (C 74-409).
// ---------------------------------------------------------------------------------------------

/* C 21-34's description is carried above; C 74-76 is
   `static ATTRIBUTE_FORMAT_PRINTF (2, 3) _Noreturn void fail (int error, const char *format,
   ...)`: the attribute has no Rust counterpart (PORTING.md 8), `_Noreturn` is the port's `-> !`,
   and C's `va_list ap; va_start (ap, format); ... va_end (ap);` have no port site - the `...`
   parameter *is* the list (P-04).  */
#[cfg (feature = "seccomp_filter")]
unsafe extern "C" fn fail (error: c_int, format: *const c_char, mut ap: ...) -> !
{
  vfprintf (stderr, format, ap);
  if error == 0
    {
      fputc (b'\n' as c_int, stderr);
    }
  else
    {
      fputs (c": ".as_ptr (), stderr);
      *errno () = error;
      perror (core::ptr::null ());
    }
  fflush (core::ptr::null_mut ());
  exit (EXIT_FAILURE);
}

/* This binary is trivial, so we use a single global filter context
   object that we release using `atexit'.  */

// C 96: `static scmp_filter_ctx ctx;`
#[cfg (feature = "seccomp_filter")]
static mut ctx: scmp_filter_ctx = core::ptr::null_mut ();

// C 98-99: `static void release_context (void)` - `unsafe extern "C" fn` because C hands its
// address to `atexit` (PORTING.md 1.4).
#[cfg (feature = "seccomp_filter")]
unsafe extern "C" fn release_context ()
{
  seccomp_release (ctx);
}

/* Wrapper functions and macros for libseccomp functions.  We exit
   immediately upon any error to avoid error checking noise.  */

// C 107-108: `static void set_attribute (enum scmp_filter_attr attr, uint32_t value)`
#[cfg (feature = "seccomp_filter")]
unsafe fn set_attribute (attr: scmp_filter_attr, value: u32)
{
  let status: c_int = seccomp_attr_set (ctx, attr, value);
  if status < 0
    {
      fail (-status, c"seccomp_attr_set (ctx, %u, %u)".as_ptr (), attr, value);
    }
}

/* Like `seccomp_rule_add (ACTION, SYSCALL, ...)', except that you
   don't have to specify the number of comparator arguments, and any
   failure will exit the process.  */

/* C 119-130.  C's `do { ... } while (false)` is the port's block expression (PORTING.md 2.1);
   `enum { arg_cnt = countof (arg_array) };` is the port's `countof!` over the same array, and
   `#action`/`#syscall`/`#__VA_ARGS__` are `stringify!` (the message C's `fail` is handed).  */
/// C: #define RULE(action, syscall, ...) do { const struct scmp_arg_cmp arg_array[] =
/// {__VA_ARGS__}; enum { arg_cnt = countof (arg_array) }; int status =
/// seccomp_rule_add_array (ctx, action, syscall, arg_cnt, arg_array); if (status < 0)
/// fail (-status, "seccomp_rule_add_array (%s, %s, %d, {%s})", #action, #syscall, arg_cnt,
/// #__VA_ARGS__); } while (false)
#[cfg (feature = "seccomp_filter")]
macro_rules! RULE {
  ($action:expr, $syscall:expr $(, $cmp:expr)* $(,)?) => {
    {
      let arg_array: [scmp_arg_cmp; _] = [$($cmp),*];
      let arg_cnt: c_uint = countof! (arg_array) as c_uint;
      let status: c_int
        = seccomp_rule_add_array (ctx, $action, $syscall, arg_cnt, arg_array.as_ptr ());
      if status < 0
        {
          fail (-status, c"seccomp_rule_add_array (%s, %s, %d, {%s})".as_ptr (),
                stringify! ($action).as_ptr () as *const c_char,
                stringify! ($syscall).as_ptr () as *const c_char,
                arg_cnt as c_int,
                stringify! ($($cmp),*).as_ptr () as *const c_char);
        }
    }
  };
}

/// C: #define RULE0(action, syscall) do { int status = seccomp_rule_add (ctx, action, syscall,
/// 0); if (status < 0) fail (-status, "seccomp_rule_add (%s, %s, 0)", #action, #syscall); }
/// while (false)
#[cfg (feature = "seccomp_filter")]
macro_rules! RULE0 {
  ($action:expr, $syscall:expr) => {
    {
      let status: c_int = seccomp_rule_add (ctx, $action, $syscall, 0);
      if status < 0
        {
          fail (-status, c"seccomp_rule_add (%s, %s, 0)".as_ptr (),
                stringify! ($action).as_ptr () as *const c_char,
                stringify! ($syscall).as_ptr () as *const c_char);
        }
    }
  };
}

/* C 141-144: `static void export_filter (const char *file, int (*function) (const
   scmp_filter_ctx, int), const char *name)` - the function-pointer parameter keeps C's single
   ABI (PORTING.md 1.4).  */
#[cfg (feature = "seccomp_filter")]
unsafe fn export_filter (file: *const c_char,
                         function: unsafe extern "C" fn (ctx: scmp_filter_ctx, fd: c_int)
                           -> c_int,
                         name: *const c_char)
{
  // C 146: `int fd;` (zero-initialised, PORTING.md 1.5)
  let mut fd: c_int = 0;
  /* C 147-151: `do fd = open (file, O_WRONLY | O_CREAT | O_TRUNC | O_BINARY | O_CLOEXEC,
     0644); while (fd < 0 && errno == EINTR);` - C's `do`/`while` loop is the port's `loop`
     with the test at the tail (PORTING.md 2.2's `loop { ... break; }` shape).  */
  loop
    {
      fd = open (file, O_WRONLY | O_CREAT | O_TRUNC | O_BINARY | O_CLOEXEC, 0o644);
      if ! (fd < 0 && *errno () == EINTR)
        {
          break;
        }
    }
  if fd < 0
    {
      fail (*errno (), c"open %s".as_ptr (), file);
    }
  let status: c_int = function (ctx, fd);
  if status < 0
    {
      fail (-status, c"%s".as_ptr (), name);
    }
  if close (fd) != 0
    {
      fail (*errno (), c"close".as_ptr ());
    }
}

/// C: #define EXPORT_FILTER(file, function) export_filter (file, function, #function)
#[cfg (feature = "seccomp_filter")]
macro_rules! EXPORT_FILTER {
  ($file:expr, $function:expr) => {
    export_filter ($file, $function, stringify! ($function).as_ptr () as *const c_char)
  };
}

// C 164-165: `int main (int argc, char **argv)`
#[cfg (feature = "seccomp_filter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  if argc != 5
    {
      fail (0, c"usage: %s out.bpf out.pfc out-exec.bpf out-exec.pfc".as_ptr (), *argv);
    }

  /* Any unhandled syscall should abort the Emacs process.  */
  ctx = seccomp_init (SCMP_ACT_KILL_PROCESS);
  if ctx.is_null ()
    {
      fail (0, c"seccomp_init".as_ptr ());
    }
  atexit (release_context);

  /* We want to abort immediately if the architecture is unknown.  */
  set_attribute (SCMP_FLTATR_ACT_BADARCH, SCMP_ACT_KILL_PROCESS);
  set_attribute (SCMP_FLTATR_CTL_NNP, 1);
  set_attribute (SCMP_FLTATR_CTL_TSYNC, 1);

  static_assert! (CHAR_BIT == 8);
  static_assert! (core::mem::size_of::<c_int> () == 4 && INT_MIN == INT32_MIN
                  && INT_MAX == INT32_MAX);
  static_assert! (core::mem::size_of::<c_long> () == 8 && LONG_MIN == INT64_MIN
                  && LONG_MAX == INT64_MAX);
  static_assert! (core::mem::size_of::<*mut c_void> () == 8);
  assert! (core::ptr::null::<c_void> () as uintptr_t == 0);

  /* Allow a clean exit.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (exit));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (exit_group));

  /* Allow `mmap' and friends.  This is necessary for dynamic loading,
     reading the portable dump file, and thread creation.  We don't
     allow pages to be both writable and executable.  */
  static_assert! (MAP_PRIVATE != 0);
  static_assert! (MAP_SHARED != 0);
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (mmap),
         SCMP_A2_32! (SCMP_CMP_MASKED_EQ,
                      !(PROT_NONE | PROT_READ | PROT_WRITE)),
         /* Only support known flags.  MAP_DENYWRITE is ignored, but
            some versions of the dynamic loader still use it.  Also
            allow allocating thread stacks.  */
         SCMP_A3_32! (SCMP_CMP_MASKED_EQ,
                      !(MAP_SHARED | MAP_PRIVATE | MAP_FILE | MAP_DROPPABLE
                        | MAP_ANONYMOUS | MAP_FIXED | MAP_DENYWRITE
                        | MAP_STACK | MAP_NORESERVE),
                      0));
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (mmap),
         SCMP_A2_32! (SCMP_CMP_MASKED_EQ,
                      !(PROT_NONE | PROT_READ | PROT_EXEC)),
         /* Only support known flags.  MAP_DENYWRITE is ignored, but
            some versions of the dynamic loader still use it. */
         SCMP_A3_32! (SCMP_CMP_MASKED_EQ,
                      !(MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED
                        | MAP_DENYWRITE),
                      0));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (munmap));
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (mprotect),
         /* Don't allow making pages executable.  */
         SCMP_A2_32! (SCMP_CMP_MASKED_EQ,
                      !(PROT_NONE | PROT_READ | PROT_WRITE), 0));

  /* Allow restartable sequences.  The dynamic linker uses them.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (rseq));

  /* Futexes are used everywhere.  */
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (futex),
         SCMP_A1_32! (SCMP_CMP_EQ, FUTEX_WAKE_PRIVATE));

  /* Allow basic dynamic memory management.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (brk));

  /* Allow some status inquiries.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (uname));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (getuid));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (geteuid));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (getpid));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (gettid));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (getpgrp));

  /* Allow operations on open file descriptors.  File descriptors are
     capabilities, and operating on them shouldn't cause security
     issues.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (read));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (pread64));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (write));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (close));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (lseek));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (dup));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (dup2));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (fstat));

  /* Allow read operations on the filesystem.  If necessary, these
     should be further restricted using mount namespaces.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (access));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (faccessat));
  /* C 259-261: `#ifdef __NR_faccessat2` / `RULE0 (SCMP_ACT_ALLOW, SCMP_SYS (faccessat2));` /
     `#endif`.  The `#endif` has no `#else`, so the arm where the architecture has no
     `__NR_faccessat2` is empty in C and nothing is transcribed for it.  */
  #[cfg (feature = "__nr_faccessat2")]
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (faccessat2));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (stat));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (stat64));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (lstat));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (lstat64));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (fstatat64));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (newfstatat));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (readlink));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (readlinkat));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (getcwd));

  /* Allow opening files, assuming they are only opened for
     reading.  */
  static_assert! (O_WRONLY != 0);
  static_assert! (O_RDWR != 0);
  static_assert! (O_CREAT != 0);
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (open),
         SCMP_A1_32! (SCMP_CMP_MASKED_EQ,
                      !(O_RDONLY | O_BINARY | O_CLOEXEC | O_PATH
                        | O_DIRECTORY | O_NOFOLLOW),
                      0));
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (openat),
         SCMP_A2_32! (SCMP_CMP_MASKED_EQ,
                      !(O_RDONLY | O_BINARY | O_CLOEXEC | O_PATH
                        | O_DIRECTORY | O_NOFOLLOW),
                      0));

  /* Allow `tcgetpgrp'.  */
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (ioctl),
         SCMP_A0_32! (SCMP_CMP_EQ, STDIN_FILENO),
         SCMP_A1_32! (SCMP_CMP_EQ, TIOCGPGRP));

  /* Allow `tcgetattr' call of glibc on physical terminal devices. */
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (ioctl),
         SCMP_A0_32! (SCMP_CMP_EQ, STDERR_FILENO),
         SCMP_A1_32! (SCMP_CMP_EQ, TCGETS));

  /* Allow reading (but not setting) file flags.  */
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (fcntl),
         SCMP_A1_32! (SCMP_CMP_EQ, F_GETFL));
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (fcntl64),
         SCMP_A1_32! (SCMP_CMP_EQ, F_GETFL));

  /* Allow reading random numbers from the kernel.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (getrandom));

  /* Changing the umask is uncritical.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (umask));

  /* Allow creation of pipes.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (pipe));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (pipe2));

  /* Allow reading (but not changing) resource limits.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (getrlimit));
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (prlimit64),
         SCMP_A0_32! (SCMP_CMP_EQ, 0) /* pid == 0 (current process) */,
         SCMP_A2_64! (SCMP_CMP_EQ, 0) /* new_limit == NULL */);

  /* Allow reading the scheduler policy and affinity, so num_processors
     can determine the number of usable CPUs.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (sched_getaffinity));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (sched_getscheduler));

  /* Block changing resource limits, but don't crash.  */
  RULE! (SCMP_ACT_ERRNO! (EPERM), SCMP_SYS! (prlimit64),
         SCMP_A0_32! (SCMP_CMP_EQ, 0) /* pid == 0 (current process) */,
         SCMP_A2_64! (SCMP_CMP_NE, 0) /* new_limit != NULL */);

  /* Emacs installs signal handlers, which is harmless.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (sigaction));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (rt_sigaction));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (sigprocmask));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (rt_sigprocmask));

  /* Allow reading the current time.  */
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (clock_gettime),
         SCMP_A0_32! (SCMP_CMP_EQ, CLOCK_REALTIME));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (time));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (gettimeofday));

  /* Allow timer support.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (timer_create));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (timerfd_create));

  /* Allow thread creation.  See the NOTES section in the manual page
     for the `clone' function.  */
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (clone),
         SCMP_A0_64! (SCMP_CMP_MASKED_EQ,
                      /* Flags needed to create threads.  See
                         create_thread in libc.  */
                      !(CLONE_VM | CLONE_FS | CLONE_FILES
                        | CLONE_SYSVSEM | CLONE_SIGHAND | CLONE_THREAD
                        | CLONE_SETTLS | CLONE_PARENT_SETTID
                        | CLONE_CHILD_CLEARTID),
                      0));
  /* glibc 2.34+ pthread_create uses clone3.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (clone3));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (sigaltstack));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (set_robust_list));

  /* Allow setting the process name for new threads.  */
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (prctl),
         SCMP_A0_32! (SCMP_CMP_EQ, PR_SET_NAME));

  /* Allow some event handling functions used by glib.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (eventfd));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (eventfd2));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (wait4));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (poll));
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (pidfd_open),
         SCMP_A1_32! (SCMP_CMP_EQ, 0));

  /* Don't allow creating sockets (network access would be extremely
     dangerous), but also don't crash.  */
  RULE0! (SCMP_ACT_ERRNO! (EACCES), SCMP_SYS! (socket));

  EXPORT_FILTER! (*argv.offset (1), seccomp_export_bpf);
  EXPORT_FILTER! (*argv.offset (2), seccomp_export_pfc);

  /* When applying a Seccomp filter before executing the Emacs binary
     (e.g. using the `bwrap' program), we need to allow further system
     calls.  Firstly, the wrapper binary will need to `execve' the
     Emacs binary.  Furthermore, the C library requires some system
     calls at startup time to set up thread-local storage.  */
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (execve));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (set_tid_address));
  RULE! (SCMP_ACT_ERRNO! (EINVAL), SCMP_SYS! (prctl),
         SCMP_A0_32! (SCMP_CMP_EQ, PR_CAPBSET_READ));
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (arch_prctl),
         SCMP_A0_32! (SCMP_CMP_EQ, ARCH_SET_FS));
  RULE! (SCMP_ACT_ERRNO! (EINVAL), SCMP_SYS! (arch_prctl),
         SCMP_A0_32! (SCMP_CMP_EQ, ARCH_CET_STATUS));
  RULE0! (SCMP_ACT_ALLOW, SCMP_SYS! (statfs));

  /* We want to allow starting the Emacs binary itself with the
     --seccomp flag, so we need to allow the `prctl' and `seccomp'
     system calls.  */
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (prctl),
         SCMP_A0_32! (SCMP_CMP_EQ, PR_SET_NO_NEW_PRIVS),
         SCMP_A1_64! (SCMP_CMP_EQ, 1), SCMP_A2_64! (SCMP_CMP_EQ, 0),
         SCMP_A3_64! (SCMP_CMP_EQ, 0), SCMP_A4_64! (SCMP_CMP_EQ, 0));
  RULE! (SCMP_ACT_ALLOW, SCMP_SYS! (seccomp),
         SCMP_A0_32! (SCMP_CMP_EQ, SECCOMP_SET_MODE_FILTER),
         SCMP_A1_32! (SCMP_CMP_EQ, SECCOMP_FILTER_FLAG_TSYNC));

  EXPORT_FILTER! (*argv.offset (3), seccomp_export_bpf);
  EXPORT_FILTER! (*argv.offset (4), seccomp_export_pfc);

  return 0;
}

