// femacs: port of exec/exec.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, field, file-scope object, local), the same control flow and the same error model.
//
// **The frozen configuration compiles nothing here**, and PORTING.md P-86 governs the file: the
// exec loader belongs to the unexec dump path (`exec/configure.ac` 235's `OBJS="exec.o trace.o"` is
// reached only from the `--with-android` configuration at `configure.ac` 178-220) while this build
// is `HAVE_PDUMPER=1`, so `docs/baseline/frozen-build-sources.txt`'s batch-F section lists no
// `exec/` object.  This file's previous revision was the three-line P-08 note; the whole body is
// transcribed as it stands, both arms of every condition, nothing stubbed.  The file has **no
// `#if` of its own** - `python3 tools/check-cond-eval.py exec/exec.c` reads its 22 conditions as 0
// unknown, 0 disagreements [VERIFIED] - so the arms below are the evaluator's split, and its
// per-block verdicts are the ledger's `## Gate mapping`.
//
// **The system surface is declared at its own C `#include` line** (P-88 (1)), not taken from the
// landed mirrors: every one of these headers is the *target's* (a Linux system's, `exec/README`) and
// the mirrors spell the frozen target's readings instead (`lib/include/errno.rs` 12's `errno` is
// `(*__error ())` and its `ENAMETOOLONG` 63 and not 36; `lib/include/limits.in.rs`'s `PATH_MAX` 1024
// and not 4096; `lib/include/unistd.in.rs` 143's `_SC_PAGESIZE` 29 and not 39), while
// `lib/include/string.rs` 38 carries `stpcpy` where this file's own C 44 `#if` decides it - and one
// translation unit cannot hold both answers (the `E0252` class the `exec/exec1.rs` round measured).
// The readings are this machine's clang against the NDK sysroots (`aarch64-linux-android24`,
// 29.0.14206865) unless a line says UNVERIFIED.
//
// **One host name the file reaches through `exec.h` has no mirror**: `USER_REGS_STRUCT`, the
// register record `exec/configure.ac` 242/276/308/341/367 defines.  The port carries it here with
// C's macro names as its fields (P-72 (3), the `exec/trace.rs` 380-412 shape; that unit's `## Owed`
// 1 and this ledger's are the same per-host-table round).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe, unused_parens,
         static_mut_refs, unsafe_op_in_unsafe_fn, unused_assignments, unused_labels,
         unused_comparisons, unreachable_code, unused_must_use, while_true, clippy::all)]

use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short,
              c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");   // C 20: `#include <config.h>`

/* ----------------------------------------------------------------------------------------------
   C 22-31: the system-header surface, in C's order (P-88 (1); see the file note above).
   ---------------------------------------------------------------------------------------------- */

/* C 22: `#include <errno.h>`.  */
/// C: <errno.h>: `int *__errno_location (void);`
unsafe extern "C" {
  pub fn __errno_location () -> *mut c_int;
}

/// C: <errno.h>: `#define errno (*__errno_location ())` - C's macro, carried as a function so that
/// `errno = x` and `errno` keep C's lvalue shape at the use sites (the `lib/include/errno.rs` 12
/// shape; C 704, C 709, C 860, C 967, C 1181).
#[inline(always)]
pub unsafe fn errno () -> *mut c_int
{
  return __errno_location ();
}

/// C: <errno.h>: `#define ENOEXEC 8`
pub const ENOEXEC: c_int = 8;
/// C: <errno.h>: `#define ENOMEM 12`
pub const ENOMEM: c_int = 12;
/// C: <errno.h>: `#define ENAMETOOLONG 36`
pub const ENAMETOOLONG: c_int = 36;

/* C 23: `#include <unistd.h>`.  The eight entries this file calls.  */
/// C: <unistd.h>: `ssize_t read (int, void *, size_t);`
/// C: <unistd.h>: `int close (int);`
/// C: <unistd.h>: `int access (const char *, int);`
/// C: <unistd.h>: `ssize_t readlink (const char *, char *, size_t);`
/// C: <unistd.h>: `off_t lseek (int, off_t, int);`
/// C: <unistd.h>: `ssize_t pread (int, void *, size_t, off_t);`
/// C: <unistd.h>: `long sysconf (int);`
/// C: <unistd.h>: `int getpagesize (void);`
unsafe extern "C" {
  pub fn read (fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
  pub fn close (fd: c_int) -> c_int;
  pub fn access (file: *const c_char, mode: c_int) -> c_int;
  pub fn readlink (path: *const c_char, buf: *mut c_char, len: usize) -> ssize_t;
  pub fn lseek (fd: c_int, offset: off_t, whence: c_int) -> off_t;
  pub fn pread (fd: c_int, buf: *mut c_void, count: usize, offset: off_t) -> ssize_t;
  pub fn sysconf (name: c_int) -> c_long;
  pub fn getpagesize () -> c_int;
}

/// C: <unistd.h>: `#define SEEK_SET 0`
pub const SEEK_SET: c_int = 0;
/// C: <unistd.h>: `#define X_OK 1`
pub const X_OK: c_int = 1;
/// C: <unistd.h>: `#define _SC_PAGESIZE 39`.  The target's value (measured 0x27; the mirror's is
/// the frozen target's 29).
pub const _SC_PAGESIZE: c_int = 39;

/* C 24: `#include <fcntl.h>`.  One name and the `open` C 514, C 990 and C 1005 call.  */
/// C: <fcntl.h>: `int open (const char *, int, ...);`
unsafe extern "C" {
  pub fn open (path: *const c_char, flags: c_int, ...) -> c_int;
}

/// C: <fcntl.h>: `#define O_RDONLY 0`
pub const O_RDONLY: c_int = 0;

/* C 25: `#include <assert.h>`.  This translation unit is the `exec/` one, whose own `config.h`
   (`exec/config.h.in`) has no `NDEBUG` line and whose makefile passes none (`exec/Makefile.in` 37's
   `CFLAGS = @CFLAGS@`), so the `<assert.h>` this file sees has the *live* `assert`
   (`src/conf_post.h` 28-29's `# define NDEBUG` is reached through the editor's `config.h`, which is
   not this unit's).  Both arms are carried under `no_ndebug`, the name `exec/trace.rs` uses for the
   same pair (P-51); the failure entry is `abort`, as it is there (PORTING.md 2.5).  */
/// C: <assert.h>: `# define assert(expr) ((void) 0)` when `NDEBUG`, else
/// C: `((expr) ? (void) 0 : __assert_fail (#expr, __FILE__, __LINE__, __func__))`
#[cfg (feature = "no_ndebug")]
macro_rules! assert {
  ($expr:expr) => { if !($expr) { abort () } };
}
#[cfg (not (feature = "no_ndebug"))]
macro_rules! assert {
  ($expr:expr) => { () };
}

/* C 26: `#include <string.h>`.  The eight entries C's text calls, declared at the C include line
   (P-88 (1)).  `stpcpy` is the one whose availability is a `#if` in this file, so its two arms are
   at C 44-66.  */
/// C: <string.h>: `size_t strlen (const char *);`
/// C: <string.h>: `void *memcpy (void *, const void *, size_t);`
/// C: <string.h>: `void *memset (void *, int, size_t);`
/// C: <string.h>: `char *strncpy (char *, const char *, size_t);`
/// C: <string.h>: `char *strcpy (char *, const char *);`
/// C: <string.h>: `size_t strnlen (const char *, size_t);`
/// C: <string.h>: `int strcmp (const char *, const char *);`
/// C: <string.h>: `void *memchr (const void *, int, size_t);`
unsafe extern "C" {
  pub fn strlen (s: *const c_char) -> usize;
  pub fn memcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
  pub fn memset (s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
  pub fn strncpy (dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
  pub fn strcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  pub fn strnlen (s: *const c_char, maxlen: usize) -> usize;
  pub fn strcmp (s1: *const c_char, s2: *const c_char) -> c_int;
  pub fn memchr (s: *const c_void, c: c_int, n: usize) -> *mut c_void;
}

/* C 27: `#include <stdlib.h>`.  C's own text calls nothing of it; it is the header whose `abort`
   the `assert` above reaches (`exec/trace.rs` 278-283 records the same reading for the same
   unit).  */
/// C: <stdlib.h>: `_Noreturn void abort (void);` (`_Noreturn` is the port's `-> !`, PORTING.md
/// 0.2).
unsafe extern "C" {
  pub fn abort () -> !;
}

/* C 29: `#include <sys/ptrace.h>`.  The call and the three requests this file passes; the readings
   are the NDK's `linux/ptrace.h`/`sys/ptrace.h`, and `PTRACE_SETREGS` is the x86 family's request,
   absent from the aarch64 header (the `exec/trace.rs` 92-148 readings of the same surface).  */
/// C: <sys/ptrace.h>: `long ptrace (enum __ptrace_request, ...);`
unsafe extern "C" {
  pub fn ptrace (request: c_uint, ...) -> c_long;
}

/// C: <sys/ptrace.h>: `#define PTRACE_PEEKDATA 2`
pub const PTRACE_PEEKDATA: c_uint = 2;
/// C: <sys/ptrace.h>: `#define PTRACE_POKETEXT 4`
pub const PTRACE_POKETEXT: c_uint = 4;
/// C: <sys/ptrace.h>: `#define PTRACE_SETREGS 13` (the x86 family's; measured with the
/// `x86_64-linux-android24` sysroot).
pub const PTRACE_SETREGS: c_uint = 13;

/* C 30: `#include <sys/param.h>`.  The two macros C 494 and C 732 name, with C's own bodies; the
   editor's `config.h` carries the same pair (the `exec/trace.rs` 71-77 note for it).  */
/// C: <sys/param.h>: `#define PATH_MAX 4096` (the target's reading; the mirror
/// `lib/include/limits.in.rs` 131 spells the frozen target's `1024`).
pub const PATH_MAX: usize = 4096;
/// C: <sys/param.h>: `#define MIN(a, b) ((a) < (b) ? (a) : (b))`
macro_rules! MIN {
  ($a:expr, $b:expr) => { if ($a) < ($b) { $a } else { $b } };
}
/// C: <sys/param.h>: `#define MAX(a, b) ((a) > (b) ? (a) : (b))`
macro_rules! MAX {
  ($a:expr, $b:expr) => { if ($a) > ($b) { $a } else { $b } };
}

/* C 31: `#include <sys/mman.h>`.  The six constants C 331, C 335-342 and C 365 build the `mmap`
   protection and flags words from (measured against the NDK's `sys/mman.h`).  */
/// C: <sys/mman.h>: `#define PROT_READ 0x1`
pub const PROT_READ: c_int = 0x1;
/// C: <sys/mman.h>: `#define PROT_WRITE 0x2`
pub const PROT_WRITE: c_int = 0x2;
/// C: <sys/mman.h>: `#define PROT_EXEC 0x4`
pub const PROT_EXEC: c_int = 0x4;
/// C: <sys/mman.h>: `#define MAP_PRIVATE 0x02`
pub const MAP_PRIVATE: c_int = 0x02;
/// C: <sys/mman.h>: `#define MAP_FIXED 0x10`
pub const MAP_FIXED: c_int = 0x10;
/// C: <sys/mman.h>: `#define MAP_ANONYMOUS 0x20`
pub const MAP_ANONYMOUS: c_int = 0x20;

/* C 33: `#include "exec.h"`.  The mirror carries the ELF records, `struct exec_tracee` and the
   loader's prototypes, and this unit *defines* `exec_0` and `format_pid` - the `E0428` class - so
   it is consumed through a module plus a glob import: a same-named definition shadows a glob
   import (P-21/P-28, the `exec/trace.rs` 63-69 shape for the same reason).  */
mod exec_decls
{
  use super::*;
  include!("include/exec.rs");                 // C 33: `#include "exec.h"`
}
use exec_decls::*;

/* C 33's `exec.h` 24: `#include <sys/types.h>` (the mirror carries it).  This file's own text names
   one type it does not supply: `ssize_t`, C 100 and C 926 (`exec/trace.rs` 187-190 declares the
   same pair in-unit).  */
/// C: <sys/types.h>: `typedef __ssize_t ssize_t;` (8 bytes on the target).
pub type ssize_t = c_long;

/* ----------------------------------------------------------------------------------------------
   C 33's `exec.h` 181-197 and `exec/configure.ac` 240-400: the *host's* names.  `USER_WORD` is
   `uintptr_t` on every host the script configures (252/286/318/351) and the mirror declares it
   `usize`; the other three are declared here because no mirror names them.
   ---------------------------------------------------------------------------------------------- */

/// The host's register record `exec/configure.ac` | 242/276/308/341/367 | names
/// `USER_REGS_STRUCT` (`struct user_regs_struct`, `struct user_regs` or `struct pt_regs`).  The C
/// text reaches it *through* that macro (`regs->SYSCALL_ARG1_REG` at C 693 and C 839), and Rust has
/// no member-name macro, so each field keeps the macro's own name (P-72 (3); the `exec/trace.rs`
/// 380-412 shape, whose `## Owed` 1 is this record's per-host table).  **UNVERIFIED**: no Linux
/// toolchain here defines the record the *C* build sees, so the member list is
/// `exec/configure.ac`'s own and the layout is documented, not measured.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USER_REGS_STRUCT
{
  /// | 253 | `AC_DEFINE([STACK_POINTER], [rsp])` and its four siblings (aarch64 `sp`, i386 `esp`,
  /// arm/mips `uregs[13]`/`gregs[29]`).
  pub STACK_POINTER: USER_WORD,
  /// | 254 | `SYSCALL_ARG_REG` (x86-64 `rdi`, aarch64/arm/mips `regs[0]`/`uregs[0]`/`gregs[4]`).
  pub SYSCALL_ARG_REG: USER_WORD,
  /// | 255 | `SYSCALL_ARG1_REG` - the one member this file reads (C 693, C 839).
  pub SYSCALL_ARG1_REG: USER_WORD,
  /// | 256 | `SYSCALL_ARG2_REG`.
  pub SYSCALL_ARG2_REG: USER_WORD,
  /// | 257 | `SYSCALL_ARG3_REG`.
  pub SYSCALL_ARG3_REG: USER_WORD,
  /// | 252 | `SYSCALL_NUM_REG` (aarch64 `regs[8]`, x86-64 `orig_rax`).
  pub SYSCALL_NUM_REG: USER_WORD,
  /// | 251 | `SYSCALL_RET_REG` (aarch64 `regs[0]`, x86-64 `rax`).
  pub SYSCALL_RET_REG: USER_WORD,
  /// | 253 | `PROGRAM_COUNTER`: a member of the same host record.
  pub PROGRAM_COUNTER: USER_WORD,
  /// `__aarch64__`'s `struct user_regs_struct` member `regs`.
  #[cfg (feature = "aarch64")]
  pub regs: [USER_WORD; 31],
  /// `__mips__`'s `struct mipsel_regs` member `gregs`.  **UNVERIFIED**: no mipsel toolchain here.
  #[cfg (feature = "mips")]
  pub gregs: [USER_WORD; 45],
}

/* C 33's `exec.h` 190-207: the prototypes this unit calls and does not define.  The mirror spells
   `USER_REGS_STRUCT`'s pointers `*mut c_void` because it cannot name the record, and P-72 (3) is
   what a use site does with that spelling: it converts explicitly.  (A second, C-spelled prototype
   here would shadow the mirror's but emit `clashing_extern_declarations` - measured: the frozen
   reading of this unit lists it - so the conversion is the reading the tree keeps.)  */

/* C 33's `exec.h` 210-211 and `exec/configure.ac` 240-400: the two base addresses the file's
   bodies use (C 455, C 633, C 636, C 1071, C 1115-1116), which no mirror names.  The script defines
   them per host (269-273 and its four siblings); the readings below are the x86-64 arm's, and the
   aarch64 arm's are `0x3000000000`/`0x3f00000000` while the i386 and arm arms' are `0x0f000000`
   with `0xaf000000` or `0x1f000000` - **UNVERIFIED**, because no configuration the port builds runs
   that script on this target.  */
/// configure.ac 273: `AC_DEFINE([EXECUTABLE_BASE], [0x555555554000])` - the x86-64 arm's reading.
pub const EXECUTABLE_BASE: USER_WORD = 0x555555554000;
/// configure.ac 274: `AC_DEFINE([INTERPRETER_BASE], [0x600000000000])` - the x86-64 arm's reading.
pub const INTERPRETER_BASE: USER_WORD = 0x600000000000;

/* C 35-37: `#if defined __mips__ && !defined MIPS_NABI` / `#include "mipsfpu.h"`.  A gated include
   edge (P-86 (8); P-89: no other reach in this unit's graph names the file).  */
#[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
include!("include/mipsfpu.rs");              // C 36: `#include "mipsfpu.h"`

/* C 39-41: three form feeds and blank lines (the file's page breaks, PORTING.md 0.1).  */

/* C 42: "Define replacements for required string functions."  */

/* C 44-66: the `HAVE_STPCPY`/`HAVE_DECL_STPCPY` pair.  The evaluator reads C 44 as *true* on the
   frozen macro table (`HAVE_STPCPY` is defined, `HAVE_DECL_STPCPY` is not), so the body below is
   the live arm there; both arms are carried (P-51) and the other one is C's one-line `#endif`.  */
#[cfg (any (not (feature = "have_stpcpy"), not (feature = "have_decl_stpcpy")))]
/// C 46-47: "Copy SRC to DEST, returning the address of the terminating '\\0' in DEST."
unsafe fn rpl_stpcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char
{
  let mut d: *mut c_char;                              // C 52: `register char *d;`
  let mut s: *const c_char;                            // C 53: `register const char *s;`

  d = dest;                                            // C 55
  s = src;                                             // C 56

  loop
    {
      *d = *s;                                         // C 59: `*d++ = *s;`
      d = d.offset (1);
      let last: c_char = *s;
      s = s.offset (1);                                // C 60: `while (*s++ != '\\0');`
      if last == 0
	{ break; }
    }

  return d.offset (-1);                                // C 62: `return d - 1;`
}

/* C 65: `#define stpcpy rpl_stpcpy`.  The rename form the `lib/include/string.in.rs` mirror uses
   for the same gnulib `REPLACE_STPCPY` (`use rpl_stpcpy as stpcpy;`, PORTING.md 0.1-2).  */
#[cfg (any (not (feature = "have_stpcpy"), not (feature = "have_decl_stpcpy")))]
use rpl_stpcpy as stpcpy;

/* C 44-66's other arm: both macros are defined, so C's `stpcpy` is the C library's.  Declared here
   at the C `#if` (P-88 (1)) because this unit cannot also carry `string.rs`'s reading of the same
   question (see the file note).  */
#[cfg (all (feature = "have_stpcpy", feature = "have_decl_stpcpy"))]
unsafe extern "C" {
  /// C: <string.h>: `char *stpcpy (char *, const char *);`
  pub fn stpcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char;
}

/* C 70-84: "Executable reading functions. …" - C's own commentary, carried in the ledger rather
   than repeated here (P-86 (1)).  */

/* C 87-93: the `check_interpreter` banner.  */

/// C 95-96: `static const char *check_interpreter (const char *name, int fd, const char **extra)`.
unsafe fn check_interpreter (name: *const c_char, fd: c_int, extra: *mut *const c_char)
  -> *const c_char
{
  static mut buffer: [c_char; PATH_MAX] = [0; PATH_MAX];   // C 98: `static char buffer[PATH_MAX], *start;`
  static mut start: *mut c_char = std::ptr::null_mut ();   // C 98
  let mut first: [c_char; 2] = [0; 2];                     // C 99: `char first[2], *end, *ws;` (C's first write is the `read` below)
  let mut end: *mut c_char = std::ptr::null_mut ();        // C 99
  let mut ws: *mut c_char = std::ptr::null_mut ();         // C 99
  let mut rc: ssize_t = 0;                                 // C 100: `ssize_t rc;`

  /* C 155-164: C's two labels (`nomatch:` and `fail:`) are the port's labeled blocks, and the
     `break 'x` statements are its `goto x` (PORTING.md 2.2).  */
  'fail: {
    'nomatch: {
      /* Read the first character.  */
      rc = read (fd, (&raw mut first) as *mut c_void, 2);  // C 103

      if rc != 2
	{ break 'fail; }                                   // C 105-106

      if first[0] != (b'#' as c_char) || first[1] != (b'!' as c_char)
	{ break 'nomatch; }                                // C 108-109

      rc = read (fd, (&raw mut buffer) as *mut c_void, PATH_MAX);   // C 111

      if rc < 0
	{ break 'fail; }                                   // C 113-114

      /* Strip leading whitespace.  */
      start = &raw mut buffer as *mut c_char;              // C 117: `start = buffer;`
      loop
	{
	  if !(start < (&raw mut buffer as *mut c_char).offset (rc as isize)
		&& (*start == (b' ' as c_char) || *start == (b'\t' as c_char)))
	    { break; }                                     // C 118
	  start = start.offset (1);                        // C 119: `++start;`
	}

      /* Look for a newline character.  */
      end = memchr (start as *const c_void, b'\n' as c_int,
		    (&raw mut buffer as *mut c_char).offset (rc as isize)
		      .offset_from (start) as usize) as *mut c_char;   // C 122

      if end.is_null ()
	{ break 'fail; }                                   // C 124-125

      /* The string containing the interpreter is now in start.  NULL terminate it.  */
      *end = 0;                                            // C 129: `*end = '\\0';`

      /* Now look for any whitespace characters.  */
      ws = start;                                          // C 132: `for (ws = start; *ws && *ws != ' ' && *ws != '\\t'; ws++)`
      loop
	{
	  if *ws == 0 || *ws == (b' ' as c_char) || *ws == (b'\t' as c_char)
	    { break; }                                     // C 132
	  ws = ws.offset (1);                              // C 132
	}

      /* If there's no whitespace, return the entire start.  */
      if *ws == 0
	{
	  if lseek (fd, 0, SEEK_SET) != 0
	    { break 'fail; }                               // C 139-140

	  return start;                                    // C 142
	}

      /* Otherwise, split the string at the whitespace and return the additional argument.  */
      *ws = 0;                                             // C 147: `*ws = '\\0';`

      if lseek (fd, 0, SEEK_SET) != 0
	{ break 'fail; }                                   // C 149-150

      *extra = ws.offset (1);                              // C 152: `*extra = ws + 1;`
      return start;                                        // C 153
    }

    /* nomatch: */
    /* There's no interpreter.  */
    if lseek (fd, 0, SEEK_SET) != 0
      { break 'fail; }                                     // C 157-158

    return name;                                           // C 160
  }

  /* fail: */
  *errno () = ENOEXEC;                                     // C 163
  return std::ptr::null ();                                // C 164
}

/* C 167-168: `static char loader_area[65536];` - "Static area used to store data placed on the
   loader's stack."  */
static mut loader_area: [c_char; 65536] = [0; 65536];

/* C 170-171: `static int loader_area_used;` - "Number of bytes used in that area."  */
static mut loader_area_used: c_int = 0;

/* C 175-176: "Structure definitions for commands placed in the loader area.  Arrange these so that
   each member is naturally aligned."  */

/// C 178-185: `struct exec_open_command`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct exec_open_command
{
  /// Word identifying the type of this command.
  pub command: USER_WORD,
}

/// C 187-209: `struct exec_map_command`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct exec_map_command
{
  /// Word identifying the type of this command.
  pub command: USER_WORD,

  /// Where the file will be mapped.
  pub vm_address: USER_WORD,

  /// Offset into the file to map from.
  pub file_offset: USER_WORD,

  /// Memory protection for mprotect.
  pub protection: USER_WORD,

  /// Number of bytes to be mapped.
  pub length: USER_WORD,

  /// Flags for mmap.
  pub flags: USER_WORD,

  /// Number of bytes to clear at the end of this mapping.
  pub clear: USER_WORD,
}

/// C 211-238: `struct exec_jump_command`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct exec_jump_command
{
  /// Word identifying the type of this command.
  pub command: USER_WORD,

  /// Address to jump to.
  pub entry: USER_WORD,

  /// The value of AT_ENTRY inside the aux vector.
  pub at_entry: USER_WORD,

  /// The value of AT_PHENT inside the aux vector.
  pub at_phent: USER_WORD,

  /// The value of AT_PHNUM inside the aux vector.
  pub at_phnum: USER_WORD,

  /// The value of AT_PHDR inside the aux vector.
  pub at_phdr: USER_WORD,

  /// The value of AT_BASE inside the aux vector.
  pub at_base: USER_WORD,

  /// C 234-237: `#if defined __mips__ && !defined __LP64__` - "The FPU mode to apply.  Not used
  /// when !MIPS_NABI."
  #[cfg (all (feature = "mips", not (feature = "lp64")))]
  pub fpu_mode: USER_WORD,
}

/* Arithmetic from the field types (`USER_WORD` is the mirror's `usize`, PORTING.md 1.1), not a
   measurement: no Linux toolchain here compiles this unit, and C's own alignment comment (C 176)
   is what puts the words on word boundaries.  */
const _: () = core::assert! (std::mem::size_of::<exec_open_command> () == 8);
const _: () = core::assert! (std::mem::size_of::<exec_map_command> () == 56);
const _: () = core::assert! (std::mem::offset_of! (exec_map_command, clear) == 48);
#[cfg (not (all (feature = "mips", not (feature = "lp64"))))]
const _: () = core::assert! (std::mem::size_of::<exec_jump_command> () == 56);
/* C 234-237's field is the eighth word, and only that arm has it.  */
#[cfg (all (feature = "mips", not (feature = "lp64")))]
const _: () = core::assert! (std::mem::size_of::<exec_jump_command> () == 64);

/* C 295-318: the `#ifndef PAGE_MASK` half of `write_load_command`.  The `static long pagesize;` C
   declares at C 296 is a *preprocessor* dependency of the two `#define`s at C 316-317 (a `#define`
   is in effect for the rest of the file), so both are crate-root items here and the C 320 onward
   uses read them: `#[cfg]` on a function-local `static mut` would be reachable from a
   `macro_rules!` in the same body, but the *scope* a C `#define` has is the file's (PORTING.md
   2.1's "one macro, one home").  */
/// C 296: `static long pagesize;`
#[cfg (not (feature = "page_mask"))]
static mut pagesize: c_long = 0;
/// C 316: `#define PAGE_MASK (~(pagesize - 1))`.  C's expression, with the `long`-to-`uintptr_t`
/// conversion C 320's `&` performs made explicit.
#[cfg (not (feature = "page_mask"))]
macro_rules! PAGE_MASK {
  () => { (!(pagesize - 1)) as USER_WORD };
}
/// C 317: `#define PAGE_SIZE (pagesize)`
#[cfg (not (feature = "page_mask"))]
macro_rules! PAGE_SIZE {
  () => { pagesize as USER_WORD };
}

/* C 295-318's other arm: the system defines the pair.  Linux's `<asm-generic/page.h>` is its source
   (`PAGE_SHIFT` 12, `PAGE_SIZE (1 << PAGE_SHIFT)`, `PAGE_MASK (~(PAGE_SIZE - 1))`); **UNVERIFIED**
   as a clang reading, because the Android sysroots carry those names only in the kernel uapi
   headers.  */
/// C: <asm-generic/page.h>: `#define PAGE_SHIFT 12`
#[cfg (feature = "page_mask")]
pub const PAGE_SHIFT: c_uint = 12;
/// C: <asm-generic/page.h>: `#define PAGE_SIZE (1 << PAGE_SHIFT)`
#[cfg (feature = "page_mask")]
macro_rules! PAGE_SIZE {
  () => { (1 as USER_WORD) << PAGE_SHIFT };
}
/// C: <asm-generic/page.h>: `#define PAGE_MASK (~(PAGE_SIZE - 1))`
#[cfg (feature = "page_mask")]
macro_rules! PAGE_MASK {
  () => { !(PAGE_SIZE! () - 1) };
}

/* C 242-244: the `write_open_command` banner.  */

/// C 246-247: `static int write_open_command (const char *name, bool alternate)`.
unsafe fn write_open_command (name: *const c_char, alternate: bool) -> c_int
{
  let mut command: exec_open_command = std::mem::zeroed ();   // C 249: `struct exec_open_command command;` (C fills it below)
  let mut size: usize = 0;                                    // C 250: `size_t size;`

  /* First, write the command to open NAME.  This is followed by NAME itself, padded to
     sizeof (USER_WORD) bytes.  */

  command.command = if alternate { 16 } else { 0 };           // C 255
  if std::mem::size_of_val (&loader_area).wrapping_sub (loader_area_used as usize)
    < std::mem::size_of_val (&command)
    { return 1; }                                             // C 256-257
  memcpy (((&raw mut loader_area) as *mut c_char).offset (loader_area_used as isize)
	    as *mut c_void, (&raw const command) as *const c_void,
	  std::mem::size_of_val (&command));                      // C 258
  loader_area_used += std::mem::size_of_val (&command) as c_int;   // C 259

  /* Calculate the length of NAME.  */
  size = strlen (name) + 1;                                   // C 262

  /* Round it up.  */
  size = size.wrapping_add (std::mem::size_of::<USER_WORD> () - 1)
    & !(std::mem::size_of::<USER_WORD> () - 1);               // C 265-266

  if std::mem::size_of_val (&loader_area).wrapping_sub (loader_area_used as usize) < size
    { return 1; }                                             // C 268-269

  /* Now copy name to the loader area, filling the padding with NULL bytes.  */
  strncpy (((&raw mut loader_area) as *mut c_char).offset (loader_area_used as isize), name,
	   size);                                                 // C 273

  /* Increase loader_area_used.  */
  loader_area_used += size as c_int;                          // C 276
  return 0;                                                    // C 277
}

/* C 280-285: the `write_load_command` banner.  */

/// C 287-289: `static int write_load_command (program_header *header, bool use_alternate,
/// USER_WORD offset)`.
unsafe fn write_load_command (header: *mut program_header, use_alternate: bool, offset: USER_WORD)
  -> c_int
{
  let mut command: exec_map_command = std::mem::zeroed ();    // C 291 (C fills it below)
  let mut command1: exec_map_command = std::mem::zeroed ();   // C 292 (C fills it below)
  let mut start: USER_WORD;                                    // C 293: `USER_WORD start, end;`
  let mut end: USER_WORD;                                      // C 293
  let mut need_command1: bool;                                 // C 294

  /* First, write the commands necessary to map the specified segment itself.

     This is the area between header->p_vaddr and header->p_filesz, rounded up to the page size.  */

  /* C 305-318: `#ifndef PAGE_MASK` - "This system doesn't define a fixed page size." - with the
     `HAVE_GETPAGESIZE` pair inside it.  The `static long pagesize;` and the two `#define`s are at
     C 295-318's position above (see the note there).  */
  #[cfg (not (feature = "page_mask"))]
  {
    #[cfg (feature = "have_getpagesize")]
    {
      if pagesize == 0
	{ pagesize = getpagesize () as c_long; }               // C 309-310
    }
    #[cfg (not (feature = "have_getpagesize"))]
    {
      if pagesize == 0
	{ pagesize = sysconf (_SC_PAGESIZE); }                  // C 312-313
    }
  }

  start = ((*header).p_vaddr as USER_WORD) & PAGE_MASK! ();     // C 320
  end = ((((*header).p_vaddr.wrapping_add ((*header).p_filesz)) as USER_WORD)
	   .wrapping_add (PAGE_SIZE! ())) & PAGE_MASK! ();       // C 321-323

  command.command = if use_alternate { 17 } else { 1 };          // C 325
  command.vm_address = start;                                   // C 326
  command.file_offset = ((*header).p_offset as USER_WORD) & PAGE_MASK! ();   // C 327
  command.protection = 0;                                       // C 328
  command.length = end.wrapping_sub (start);                    // C 329
  command.clear = 0;                                            // C 330
  command.flags = (MAP_PRIVATE | MAP_FIXED) as USER_WORD;       // C 331

  /* Apply the memory protection specified in the header.  */

  if (*header).p_flags & 4 != 0                                 // C 335: `if (header->p_flags & 4) /* PF_R */`
    { command.protection |= PROT_READ as USER_WORD; }           // C 336

  if (*header).p_flags & 2 != 0                                 // C 338: `/* PF_W */`
    { command.protection |= PROT_WRITE as USER_WORD; }          // C 339

  if (*header).p_flags & 1 != 0                                 // C 341: `/* PF_X */`
    { command.protection |= PROT_EXEC as USER_WORD; }           // C 342

  /* Next, write any command necessary to map pages in the area between p_filesz and p_memsz.  */
  need_command1 = false;                                        // C 346

  if (*header).p_memsz > (*header).p_filesz                     // C 348
    {
      /* If there are bytes after end which need to be initialized, do that now.  */
      command.clear = end.wrapping_sub ((*header).p_vaddr as USER_WORD)
	.wrapping_sub ((*header).p_filesz as USER_WORD);        // C 352
      start = end;                                              // C 353
      end = ((*header).p_vaddr.wrapping_add ((*header).p_memsz) as USER_WORD)
	.wrapping_add (PAGE_SIZE! ());                          // C 354
      end &= PAGE_MASK! ();                                     // C 355

      if end > start                                            // C 357
	{
	  command1.command = 4;                                  // C 359
	  command1.vm_address = start;                           // C 360
	  command1.file_offset = 0;                              // C 361
	  command1.length = end.wrapping_sub (start);            // C 362
	  command1.clear = 0;                                    // C 363
	  command1.protection = command.protection;              // C 364
	  command1.flags = (MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED) as USER_WORD;   // C 365
	  need_command1 = true;                                  // C 366
	}
    }

  /* Apply the offset to both commands if necessary.  */

  if offset != 0                                                // C 372
    {
      if need_command1
	{ command1.vm_address = command1.vm_address.wrapping_add (offset); }   // C 374-375

      command.vm_address = command.vm_address.wrapping_add (offset);          // C 377
    }

  /* Write both commands.  */

  if std::mem::size_of_val (&loader_area).wrapping_sub (loader_area_used as usize)
    < std::mem::size_of_val (&command)
    { return 1; }                                               // C 382-383

  memcpy (((&raw mut loader_area) as *mut c_char).offset (loader_area_used as isize)
	    as *mut c_void, (&raw const command) as *const c_void,
	  std::mem::size_of_val (&command));                       // C 385-386
  loader_area_used += std::mem::size_of_val (&command) as c_int;   // C 387

  if !need_command1
    { return 0; }                                                // C 389-390

  if std::mem::size_of_val (&loader_area).wrapping_sub (loader_area_used as usize)
    < std::mem::size_of_val (&command1)
    { return 1; }                                                // C 392-393

  memcpy (((&raw mut loader_area) as *mut c_char).offset (loader_area_used as isize)
	    as *mut c_void, (&raw const command1) as *const c_void,
	  std::mem::size_of_val (&command1));                      // C 395-396
  loader_area_used += std::mem::size_of_val (&command1) as c_int;   // C 397

  return 0;                                                     // C 399
}

/* C 402-421: `#if defined __mips__ && !defined MIPS_NABI` - the four static storage objects the
   MIPS ABI-flag path records into.  C leaves them zero-initialized; Rust's static initializer
   spells C's own zero value (PORTING.md 0.1-13, and 1.5 for the locals of the same class).  */
#[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
/// C 404-405: `static struct mips_elf_abi_flags exec_abi, interpreter_abi;`
static mut exec_abi: mips_elf_abi_flags = unsafe { std::mem::zeroed () };
#[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
/// C 404-405
static mut interpreter_abi: mips_elf_abi_flags = unsafe { std::mem::zeroed () };

#[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
/// C 407-408: `static elf_header exec_interpreter_header;`
static mut exec_interpreter_header: elf_header = unsafe { std::mem::zeroed () };

#[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
/// C 410-411: `static elf_header *interpreter_header;`
static mut interpreter_header: *mut elf_header = std::ptr::null_mut ();

#[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
/// C 413-415: `static struct mips_elf_abi_flags *exec_abiflags;`
static mut exec_abiflags: *mut mips_elf_abi_flags = std::ptr::null_mut ();

#[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
/// C 417-419: `static struct mips_elf_abi_flags *interpreter_abiflags;`
static mut interpreter_abiflags: *mut mips_elf_abi_flags = std::ptr::null_mut ();

/* C 423-431: the `process_interpreter_1` banner.  */

/// C 433-436: `static int process_interpreter_1 (const char *name, int fd, program_header *header,
/// elf_header *elf_header)`.
unsafe fn process_interpreter_1 (name: *const c_char, fd: c_int, header: *mut program_header,
				 elf_header: *mut elf_header) -> c_int
{
  let mut rc: c_int;                                           // C 438: `int rc;`
  #[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
  let mut rc1: ssize_t = 0;                                    // C 440: `ssize_t rc1;`

  /* C 445's `default:` arm is the `_` arm below (Rust's `match` has no arm order to keep, and a
     leading `_` would make the rest unreachable).  */
  match (*header).p_type
    {
      1 =>                                                     // C 449: `case 1: /* PT_LOAD */`
	{
	  /* This describes a segment in the file that must be loaded.  Write the appropriate load
	     command.  */

	  if (*elf_header).e_type == 3                            // C 453: `/* ET_DYN */`
	    { rc = write_load_command (header, true, INTERPRETER_BASE); }      // C 454-455
	  else
	    { rc = write_load_command (header, true, 0); }        // C 457
	}

      #[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
      0x70000003 =>                                            // C 462: `case 0x70000003: /* PT_MIPS_ABIFLAGS */`
	{
	  /* Record this header for later use.  */
	  rc1 = pread (fd, (&raw mut interpreter_abi) as *mut c_void,
		       std::mem::size_of_val (&interpreter_abi),
		       (*header).p_offset as off_t);              // C 464-465

	  if rc1 != std::mem::size_of_val (&interpreter_abi) as ssize_t
	    { return 1; }                                         // C 467-468

	  interpreter_abiflags = &raw mut interpreter_abi;        // C 470
	  rc = 0;                                                 // C 471
	}

      _ =>                                                        // C 445: `default: /* PT_NULL, PT_NOTE, PT_DYNAMIC, PT_INTERP, et cetera */`
	{ rc = 0; }                                               // C 446
    }

  return rc;                                                      // C 475
}

/* C 478-482: the `process_interpreter` banner.  */

/// C 484-486: `static int process_interpreter (int fd, program_header *prog_header, USER_WORD
/// *entry)`.  C 514 assigns `fd`, so the parameter is `mut` (Rust 0.1-13's class: the name and the
/// value are C's, the binding is the language's).
unsafe fn process_interpreter (mut fd: c_int, prog_header: *mut program_header,
			       entry: *mut USER_WORD) -> c_int
{
  let mut buffer: [c_char; PATH_MAX + 1] = [0; PATH_MAX + 1];   // C 488: `char buffer[PATH_MAX + 1];`
  let mut rc: c_int = 0;                                        // C 489: `int rc, size, i;`
  let mut size: c_int = 0;                                      // C 489
  let mut i: c_int = 0;                                         // C 489
  let mut header: elf_header = std::mem::zeroed ();              // C 490 (C's first write is the `read` below)
  let mut program: program_header = std::mem::zeroed ();         // C 491 (C's first write is the `read` below)

  /* Read the interpreter name.  */
  size = MIN! ((*prog_header).p_filesz as USER_WORD, PATH_MAX as USER_WORD) as c_int;   // C 494
  rc = pread (fd, (&raw mut buffer) as *mut c_void, size as usize,
	      (*prog_header).p_offset as off_t) as c_int;         // C 495
  if rc < size
    { return 1; }                                                 // C 496-497

  /* Make sure the name is NULL terminated.  */
  buffer[size as usize] = 0;                                      // C 500: `buffer[size] = '\\0';`

  /* Check if the file is executable.  This is unfortunately not atomic.  */

  if access ((&raw const buffer) as *const c_char, X_OK) != 0
    { return 1; }                                                 // C 505-506

  /* Read the interpreter's header much like exec_0.

     However, use special command codes in `process_program_header` if it is position independent.
     That way, the loader knows it should use the open interpreter instead.  */

  fd = open ((&raw const buffer) as *const c_char, O_RDONLY);      // C 514

  if fd < 0
    { return 1; }                                                 // C 516-517

  rc = read (fd, (&raw mut header) as *mut c_void,
	     std::mem::size_of_val (&header)) as c_int;            // C 519

  /* C 589-591's `fail:` label is the port's labeled block (PORTING.md 2.2).  */
  'fail: {
    if rc < std::mem::size_of_val (&header) as c_int
      { break 'fail; }                                            // C 521-522

    #[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
    {
      /* Record this interpreter's header for later use determining the floating point ABI.  */
      exec_interpreter_header = header;                            // C 527
      interpreter_header = &raw mut exec_interpreter_header;       // C 528
    }

    /* Verify that this is indeed an ELF file.  */

    if header.e_ident[0] != 0x7f
      || header.e_ident[1] != b'E'
      || header.e_ident[2] != b'L'
      || header.e_ident[3] != b'F'
      { break 'fail; }                                            // C 533-537

    /* Now check that the class is correct.  */
    #[cfg (feature = "exec_64")]
    {
      if header.e_ident[4] != 2
	{ break 'fail; }                                          // C 541-542
    }
    #[cfg (not (feature = "exec_64"))]
    {
      if header.e_ident[4] != 1
	{ break 'fail; }                                          // C 544-545
    }

    /* And the endianness.  */
    #[cfg (not (feature = "words_bigendian"))]
    {
      if header.e_ident[5] != 1
	{ break 'fail; }                                          // C 550-551
    }
    #[cfg (feature = "words_bigendian")]
    {
      if header.e_ident[5] != 2
	{ break 'fail; }                                          // C 553-554
    }

    /* Check that this is an executable.  */
    if header.e_type != 2 && header.e_type != 3
      { break 'fail; }                                            // C 558-559

    /* Now check that the ELF program header makes sense.  */
    if header.e_phnum > 0xffff
      || (header.e_phentsize as usize != std::mem::size_of::<program_header> ())
      { break 'fail; }                                            // C 562-565

    if write_open_command ((&raw const buffer) as *const c_char, true) != 0
      { break 'fail; }                                            // C 567-568

    i = 0;                                                        // C 570: `for (i = 0; i < header.e_phnum; ++i)`
    loop
      {
	if i >= header.e_phnum as c_int
	  { break; }                                              // C 570
	rc = read (fd, (&raw mut program) as *mut c_void,
		   std::mem::size_of_val (&program)) as c_int;    // C 572
	if rc < std::mem::size_of_val (&program) as c_int
	  { break 'fail; }                                        // C 573-574

	if process_interpreter_1 ((&raw const buffer) as *const c_char, fd, &raw mut program,
				  &raw mut header) != 0
	  { break 'fail; }                                        // C 576-578
	i += 1;                                                   // C 570
      }

    if header.e_type == 3                                         // C 581: `/* ET_DYN */`
      { *entry = (header.e_entry as USER_WORD).wrapping_add (INTERPRETER_BASE); }   // C 582
    else
      { *entry = header.e_entry as USER_WORD; }                   // C 584

    close (fd);                                                   // C 586
    return 0;                                                     // C 587
  }

  /* fail: */
  close (fd);                                                     // C 590
  return 1;                                                       // C 591
}

/* C 594-606: the `process_program_header` banner.  */

/// C 608-613: `static int process_program_header (const char *name, int fd, program_header *header,
/// elf_header *elf_header, USER_WORD *entry, USER_WORD *offset)`.
unsafe fn process_program_header (name: *const c_char, fd: c_int, header: *mut program_header,
				  elf_header: *mut elf_header, entry: *mut USER_WORD,
				  offset: *mut USER_WORD) -> c_int
{
  let mut rc: c_int;                                              // C 615: `int rc;`
  #[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
  let mut rc1: ssize_t = 0;                                       // C 617: `ssize_t rc1;`

  /* C 622's `default:` arm is the `_` arm below (see `process_interpreter_1`).  */
  match (*header).p_type
    {
      1 =>                                                        // C 626: `case 1: /* PT_LOAD */`
	{
	  /* This describes a segment in the file that must be loaded.  Write the appropriate load
	     command.  */

	  if (*elf_header).e_type == 3                              // C 630: `/* ET_DYN */`
	    {
	      rc = write_load_command (header, false, EXECUTABLE_BASE);   // C 632-633

	      if rc == 0 && !offset.is_null () && *offset == USER_WORD::MAX
		{ *offset = EXECUTABLE_BASE.wrapping_add ((*header).p_vaddr as USER_WORD); }   // C 635-636
	    }
	  else
	    {
	      rc = write_load_command (header, false, 0);             // C 640

	      if rc == 0 && !offset.is_null () && *offset == USER_WORD::MAX
		{ *offset = (*header).p_vaddr as USER_WORD; }          // C 642-643
	    }
	}

      3 =>                                                        // C 648: `case 3: /* PT_INTERP */`
	{
	  /* This describes another executable that must be loaded.  Open the interpreter and
	     process each of its headers as well.  */
	  rc = process_interpreter (fd, header, entry);               // C 651
	}

      1685382481 =>                                               // C 654: `case 1685382481: /* PT_GNU_STACK */`
	{
	  /* TODO */
	  rc = 0;                                                     // C 656
	}

      #[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
      0x70000003 =>                                               // C 660: `case 0x70000003: /* PT_MIPS_ABIFLAGS */`
	{
	  /* Record this header for later use.  */
	  rc1 = pread (fd, (&raw mut exec_abi) as *mut c_void, std::mem::size_of_val (&exec_abi),
		       (*header).p_offset as off_t);                  // C 662-663

	  if rc1 != std::mem::size_of_val (&exec_abi) as ssize_t
	    { return 1; }                                             // C 665-666

	  exec_abiflags = &raw mut exec_abi;                          // C 668
	  rc = 0;                                                     // C 669
	}

      _ =>                                                        // C 622: `default: /* PT_NULL, PT_NOTE, PT_DYNAMIC, et cetera */`
	{ rc = 0; }                                                   // C 623
    }

  return rc;                                                       // C 673
}

/* C 676-681: the `insert_args` banner.  */

/// C 683-685: `static int insert_args (struct exec_tracee *tracee, USER_REGS_STRUCT *regs, const
/// char *arg1, const char *arg2, const char *arg3)`.
unsafe fn insert_args (tracee: *mut exec_tracee, regs: *mut USER_REGS_STRUCT, arg1: *const c_char,
		       arg2: *const c_char, arg3: *const c_char) -> c_int
{
  let mut argv: USER_WORD = 0;                                    // C 687: `USER_WORD argv, argc, word, new;`
  let mut argc: USER_WORD = 0;                                    // C 687
  let mut word: USER_WORD = 0;                                    // C 687
  let mut new: USER_WORD = 0;                                     // C 687
  let mut new1: USER_WORD = 0;                                    // C 688: `USER_WORD new1, new2, new3, i;`
  let mut new2: USER_WORD = 0;                                    // C 688
  let mut new3: USER_WORD = 0;                                    // C 688
  let mut i: USER_WORD = 0;                                       // C 688
  let mut text_size: usize = 0;                                   // C 689: `size_t text_size, effective_size;`
  let mut effective_size: usize = 0;                              // C 689
  let mut original: USER_REGS_STRUCT = std::mem::zeroed ();        // C 690 (C's first write is the `memcpy` below)

  /* C 853-861's `fail:` label is the port's labeled block (PORTING.md 2.2).  */
  'fail: {
    /* First, get a pointer to the current argument vector.  */
    argv = (*regs).SYSCALL_ARG1_REG;                              // C 693

    /* Now figure out how many arguments there are.  */
    argc = 0;                                                     // C 696
    loop                                                          // C 697: `while (true)`
      {
	/* Clear errno.  PTRACE_PEEKDATA returns the word read the same way failure indications
	   are returned, so the only way to catch IO errors is by clearing errno before the call to
	   ptrace and checking it afterwards.  */

	*errno () = 0;                                            // C 704
	word = ptrace (PTRACE_PEEKDATA, (*tracee).pid, argv as *mut c_void,
		       std::ptr::null_mut::<c_void> ()) as USER_WORD;   // C 705-706
	argv = argv.wrapping_add (std::mem::size_of::<USER_WORD> ());   // C 707: `argv += sizeof (USER_WORD);`

	if *errno () != 0
	  { return 1; }                                           // C 709-710

	if word == 0
	  { break; }                                              // C 712-713

	argc += 1;                                                // C 715
      }

    /* Allocate enough to hold that many arguments, alongside the argc text.  */

    text_size = strlen (arg1) + 1
      + (if !arg2.is_null () { strlen (arg2) + 1 } else { 0 })
      + strlen (arg3) + 1;                                        // C 721-723

    /* Round it up to the user word size.  */
    text_size = text_size.wrapping_add (std::mem::size_of::<USER_WORD> () - 1);   // C 726
    text_size &= !(std::mem::size_of::<USER_WORD> () - 1);        // C 727

    /* Now allocate the new argv.  Make sure argc is at least 1; it needs to hold ARG3.  */

    effective_size = std::mem::size_of_val (&word)
      .wrapping_mul (MAX! (1 as USER_WORD, argc) as usize + 2)
      .wrapping_add (text_size);                                  // C 732

    if !arg2.is_null ()
      { effective_size += std::mem::size_of_val (&word); }        // C 734-735

    /* Copy regs to original so that user_alloca knows it should append the ABI red zone.  */

    memcpy ((&raw mut original) as *mut c_void, regs as *const c_void,
	    std::mem::size_of_val (&*regs));                       // C 740
    new = user_alloca (tracee, (&raw mut original) as *mut c_void, regs as *mut c_void,
		       effective_size);                             // C 741-742

    if new == 0
      { break 'fail; }                                            // C 744-745

    /* Figure out where argv starts.  */

    new3 = new.wrapping_add (text_size);                          // C 749

    /* Now write the first two strings.  */

    new1 = new.wrapping_add (strlen (arg1) + 1);                  // C 753
    new2 = new1.wrapping_add (if !arg2.is_null () { strlen (arg2) + 1 } else { 0 });   // C 754

    if user_copy (tracee, arg1 as *const c_uchar, new, new1.wrapping_sub (new)) != 0
      { break 'fail; }                                            // C 756-758

    if !arg2.is_null ()
      && user_copy (tracee, arg2 as *const c_uchar, new1, new2.wrapping_sub (new1)) != 0
      { break 'fail; }                                            // C 760-762

    /* Write the replacement arg3, the file name of the executable.  */

    if user_copy (tracee, arg3 as *const c_uchar, new2, new3.wrapping_sub (new2)) != 0
      { break 'fail; }                                            // C 766-768

    /* Start copying argv back to new2.  First, write the one or two new arguments.  */

    if ptrace (PTRACE_POKETEXT, (*tracee).pid, new3 as *mut c_void, new as *mut c_void) != 0
      { break 'fail; }                                            // C 773-775

    new3 = new3.wrapping_add (std::mem::size_of_val (&new3));      // C 777

    if !arg2.is_null ()
      && ptrace (PTRACE_POKETEXT, (*tracee).pid, new3 as *mut c_void, new1 as *mut c_void) != 0
      { break 'fail; }                                            // C 779-781
    else if !arg2.is_null ()
      { new3 = new3.wrapping_add (std::mem::size_of_val (&new3)); }   // C 782-783

    /* Next, write the third argument.  */

    if ptrace (PTRACE_POKETEXT, (*tracee).pid, new3 as *mut c_void, new2 as *mut c_void) != 0
      { break 'fail; }                                            // C 787-789

    new3 = new3.wrapping_add (std::mem::size_of_val (&new3));      // C 791

    /* Copy the remaining arguments back.  */

    argv = (*regs).SYSCALL_ARG1_REG;                              // C 795

    if argc != 0                                                  // C 797
      {
	/* Make sure the trailing NULL is included.  */
	argc += 1;                                                // C 800

	/* Now copy each argument in argv, starting from argv[1].  */

	i = 1;                                                    // C 804: `for (i = 1; i < argc; ++i)`
	loop
	  {
	    if i >= argc
	      { break; }                                          // C 804

	    /* Read one argument.  */
	    word = ptrace (PTRACE_PEEKDATA, (*tracee).pid,
			   argv.wrapping_add (i.wrapping_mul (std::mem::size_of_val (&argv)
							      as USER_WORD)) as *mut c_void,
			   std::ptr::null_mut::<c_void> ()) as USER_WORD;   // C 807-808

	    /* Write one argument, then increment new3.  */

	    if ptrace (PTRACE_POKETEXT, (*tracee).pid, new3 as *mut c_void,
		       word as *mut c_void) != 0
	      { break 'fail; }                                        // C 812-814

	    new3 = new3.wrapping_add (std::mem::size_of_val (&new3));  // C 816
	    i += 1;                                                   // C 804
	  }
      }
    else
      {
	/* Just write the trailing NULL.  */

	if ptrace (PTRACE_POKETEXT, (*tracee).pid, new3 as *mut c_void,
		   std::ptr::null_mut::<c_void> ()) != 0
	  { break 'fail; }                                          // C 823-825

	new3 = new3.wrapping_add (std::mem::size_of_val (&new3));    // C 827
      }

    /* Assert that new3 is not out of bounds.  */
    assert! (new3 == new.wrapping_add (effective_size));           // C 831

    /* And that it is properly aligned.  */
    assert! ((new3 & (std::mem::size_of_val (&new3) - 1)) == 0);   // C 834: `assert (!(new3 & (sizeof new3 - 1)));`

    /* Now modify the system call argument to point to new + text_size.  */

    (*regs).SYSCALL_ARG1_REG = new.wrapping_add (text_size);       // C 839

    #[cfg (feature = "aarch64")]
    {
      if aarch64_set_regs ((*tracee).pid, regs as *mut c_void, false) != 0
	{ break 'fail; }                                            // C 842-843
    }
    #[cfg (not (feature = "aarch64"))]
    {
      if ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (),
		 regs as *mut c_void) != 0
	{ break 'fail; }                                            // C 845-846
    }

    /* Success.  */

    return 0;                                                       // C 851
  }

  /* fail: */
  /* Restore the original stack pointer.  */
  #[cfg (feature = "aarch64")]
  { aarch64_set_regs ((*tracee).pid, (&raw mut original) as *mut c_void, false); }   // C 856
  #[cfg (not (feature = "aarch64"))]
  {
    ptrace (PTRACE_SETREGS, (*tracee).pid, std::ptr::null_mut::<c_void> (),
	    (&raw mut original) as *mut c_void);                    // C 858
  }
  *errno () = ENOMEM;                                              // C 860
  return 1;                                                        // C 861
}

/* C 866-868: the `format_pid` banner, carried as C's own text (its "REM should be NULL" sentence
   names a parameter the function does not have; kept as it stands, PORTING.md P-86 (1)).  */

/// C 870-871: `char *format_pid (char *in, unsigned int pid)` - a non-`static` definition, so it
/// keeps `#[unsafe(no_mangle)]` (PORTING.md 0.2); `in` is a Rust keyword and takes the raw
/// identifier spelling (0.2).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn format_pid (mut r#in: *mut c_char, mut pid: c_uint) -> *mut c_char
{
  let mut digits: [c_uint; 32] = [0; 32];                        // C 873: `unsigned int digits[32], *fill;`
  let mut fill: *mut c_uint = std::ptr::null_mut ();             // C 873

  fill = &raw mut digits as *mut c_uint;                         // C 875: `fill = digits;`

  loop                                                           // C 877: `for (; pid != 0; pid = pid / 10)`
    {
      if pid == 0
	{ break; }                                               // C 877
      *fill = pid % 10;                                          // C 878: `*fill++ = pid % 10;`
      fill = fill.offset (1);
      pid = pid / 10;                                            // C 877
    }

  /* Insert 0 if the number would otherwise be empty.  */

  if fill == (&raw mut digits as *mut c_uint)                     // C 882
    {
      *fill = 0;                                                 // C 883: `*fill++ = 0;`
      fill = fill.offset (1);
    }

  while fill != (&raw mut digits as *mut c_uint)                  // C 885
    {
      fill = fill.offset (-1);                                    // C 887: `--fill;`
      *r#in = ((b'0' as c_uint).wrapping_add (*fill)) as c_char;   // C 888: `*in++ = '0' + *fill;`
      r#in = r#in.offset (1);
    }

  *r#in = 0;                                                      // C 891: `*in = '\\0';`
  return r#in;                                                    // C 892
}

/* C 895-907: the `exec_0` banner.  */

/// C 909-911: `char *exec_0 (char *name, struct exec_tracee *tracee, size_t *size, USER_REGS_STRUCT
/// *regs)` - non-`static`, so `#[unsafe(no_mangle)]` (PORTING.md 0.2).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exec_0 (name: *mut c_char, tracee: *mut exec_tracee, size: *mut usize,
				 regs: *mut USER_REGS_STRUCT) -> *mut c_char
{
  let mut fd: c_int = 0;                                          // C 913: `int fd, rc, i;`
  let mut rc: c_int = 0;                                          // C 913
  let mut i: c_int = 0;                                           // C 913
  let mut header: elf_header = std::mem::zeroed ();                // C 914 (C's first write is the `read` below)
  let mut interpreter_name: *const c_char = std::ptr::null ();     // C 915: `const char *interpreter_name, *extra;` (C assigns `extra` at C 996)
  let mut extra: *const c_char = std::ptr::null ();                // C 915
  let mut program: program_header = std::mem::zeroed ();           // C 916 (C's first write is the `read` below)
  let mut entry: USER_WORD = 0;                                    // C 917: `USER_WORD entry, program_entry, offset;`
  let mut program_entry: USER_WORD = 0;                            // C 917
  let mut offset: USER_WORD = 0;                                   // C 917
  let mut header_offset: USER_WORD = 0;                            // C 918
  let mut name_len: USER_WORD = 0;                                 // C 919: `USER_WORD name_len, aligned_len;`
  let mut aligned_len: USER_WORD = 0;                              // C 919
  let mut jump: exec_jump_command = std::mem::zeroed ();            // C 920 (C fills it field by field)
  #[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
  let mut fpu_mode: c_int = 0;                                     // C 923: `int fpu_mode;`
  let mut buffer: [c_char; 80] = [0; 80];                          // C 925: `char buffer[80], buffer1[PATH_MAX + 80], *rewrite;`
  let mut buffer1: [c_char; PATH_MAX + 80] = [0; PATH_MAX + 80];    // C 925
  let mut rewrite: *mut c_char = std::ptr::null_mut ();            // C 925
  let mut link_size: ssize_t = 0;                                  // C 926: `ssize_t link_size;`
  let mut remaining: usize = 0;                                    // C 927: `size_t remaining;`

  /* C 1180-1184: C's two labels (`fail1:` and `fail:`) are the port's nested labeled blocks, and
     `fail1` runs into `fail` exactly as it does in C (PORTING.md 2.2).  */
  'fail: {
    'fail1: {
      /* If the process is trying to run /proc/self/exe, make it run itself instead.  */

      if strcmp (name, c"/proc/self/exe".as_ptr ()) == 0 && !(*tracee).exec_file.is_null ()
	{
	  strncpy (name, (*tracee).exec_file, PATH_MAX - 1);        // C 934
	  *name.offset (PATH_MAX as isize) = 0;                     // C 935: `name[PATH_MAX] = '\\0';`
	}
      else
	{
	  /* If name is not absolute, then make it relative to TRACEE's cwd.  Do not use sprintf at
	     it is not reentrant and it mishandles results longer than INT_MAX.  */

	  if *name != 0 && *name != (b'/' as c_char)                // C 943
	    {
	      /* Clear both buffers.  */
	      memset ((&raw mut buffer) as *mut c_void, 0, std::mem::size_of_val (&buffer));      // C 946
	      memset ((&raw mut buffer1) as *mut c_void, 0, std::mem::size_of_val (&buffer1));    // C 947

	      /* Copy over /proc, the PID, and /cwd/.  */
	      rewrite = stpcpy ((&raw mut buffer) as *mut c_char, c"/proc/".as_ptr ());   // C 950
	      rewrite = format_pid (rewrite, (*tracee).pid as c_uint);   // C 951
	      strcpy (rewrite, c"/cwd".as_ptr ());                      // C 952

	      /* Resolve this symbolic link.  */

	      link_size = readlink ((&raw const buffer) as *const c_char,
				    (&raw mut buffer1) as *mut c_char, PATH_MAX + 1);   // C 956-957

	      if link_size < 0
		{ return std::ptr::null_mut (); }                       // C 959-960

	      /* Check that the name is a reasonable size.  */

	      if link_size > PATH_MAX as ssize_t                        // C 964
		{
		  /* The name is too long.  */
		  *errno () = ENAMETOOLONG;                              // C 967
		  return std::ptr::null_mut ();                          // C 968
		}

	      /* Add a directory separator if necessary.  */

	      if link_size == 0
		|| *((&raw const buffer1) as *const c_char).offset (link_size as isize - 1)
		   != (b'/' as c_char)
		{
		  /* C 974: `buffer1[link_size] = '/', link_size++;` - C's comma operator is two
		     statements.  */
		  *((&raw mut buffer1) as *mut c_char).offset (link_size as isize) = b'/' as c_char;
		  link_size += 1;
		}

	      rewrite = ((&raw mut buffer1) as *mut c_char).offset (link_size as isize);   // C 976
	      remaining = ((((&raw mut buffer1) as *mut c_char)
			    .offset (std::mem::size_of_val (&buffer1) as isize))
			   .offset_from (rewrite) as usize) - 1;        // C 977: `remaining = buffer1 + sizeof buffer1 - rewrite - 1;`
	      memcpy (rewrite as *mut c_void, name as *const c_void,
		      strnlen (name, remaining));                        // C 978

	      /* Replace name with buffer1.  */
	      strcpy (name, (&raw const buffer1) as *const c_char);     // C 981
	    }
	}

      /* Check that the file is accessible and executable.  */

      if access (name, X_OK) != 0
	{ return std::ptr::null_mut (); }                            // C 987-988

      fd = open (name, O_RDONLY);                                    // C 990
      if fd < 0
	{ return std::ptr::null_mut (); }                            // C 991-992

      /* Now read the header.  */

      extra = std::ptr::null ();                                     // C 996
      interpreter_name = check_interpreter (name, fd, &raw mut extra);   // C 997
      if interpreter_name.is_null ()
	{ break 'fail; }                                             // C 998-999

      /* Open the interpreter instead, if necessary.  */
      if interpreter_name != name                                    // C 1002
	{
	  close (fd);                                                // C 1004
	  fd = open (interpreter_name, O_RDONLY);                     // C 1005
	  if fd < 0
	    { return std::ptr::null_mut (); }                        // C 1006-1007

	  /* Now, rewrite the argument list to include `interpreter_name' and perhaps `extra'.  */

	  if insert_args (tracee, regs, interpreter_name, extra, name) != 0
	    { break 'fail1; }                                        // C 1012-1014
	}

      rc = read (fd, (&raw mut header) as *mut c_void,
		 std::mem::size_of_val (&header)) as c_int;          // C 1017

      if rc < std::mem::size_of_val (&header) as c_int
	{ break 'fail1; }                                            // C 1019-1020

      /* Verify that this is indeed an ELF file.  */

      if header.e_ident[0] != 0x7f
	|| header.e_ident[1] != b'E'
	|| header.e_ident[2] != b'L'
	|| header.e_ident[3] != b'F'
	{ break 'fail1; }                                            // C 1024-1028

      /* Now check that the class is correct.  */
      #[cfg (feature = "exec_64")]
      {
	if header.e_ident[4] != 2
	  { break 'fail1; }                                          // C 1032-1033
      }
      #[cfg (not (feature = "exec_64"))]
      {
	if header.e_ident[4] != 1
	  { break 'fail1; }                                          // C 1035-1036
      }

      /* And the endianness.  */
      #[cfg (not (feature = "words_bigendian"))]
      {
	if header.e_ident[5] != 1
	  { break 'fail1; }                                          // C 1041-1042
      }
      #[cfg (feature = "words_bigendian")]
      {
	if header.e_ident[5] != 2
	  { break 'fail1; }                                          // C 1044-1045
      }

      /* Check that this is an executable.  */
      if header.e_type != 2 && header.e_type != 3
	{ break 'fail1; }                                            // C 1049-1050

      /* Now check that the ELF program header makes sense.  */
      if header.e_phnum > 0xffff
	|| (header.e_phentsize as usize != std::mem::size_of::<program_header> ())
	{ break 'fail1; }                                            // C 1053-1056

      /* Seek to the first program header and read each one.  */
      rc = lseek (fd, header.e_phoff as off_t, SEEK_SET) as c_int;    // C 1059
      if rc < 0
	{ break 'fail1; }                                            // C 1060-1061
      loader_area_used = 0;                                          // C 1062

      /* Write the command used to open the executable.  */
      if write_open_command (interpreter_name, false) != 0
	{ break 'fail1; }                                            // C 1065-1066

      /* Apply base addresses for PIC code.  */

      if header.e_type == 3                                          // C 1070: `/* ET_DYN */`
	{ offset = EXECUTABLE_BASE; }                                // C 1071
      else
	{ offset = 0; }                                              // C 1073

      /* entry and program_entry are initially the same, but entry may be set to that of the
	 interpreter if one is present.  */

      entry = (header.e_entry as USER_WORD).wrapping_add (offset);    // C 1078
      program_entry = header.e_entry as USER_WORD;                   // C 1079

      #[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
      {
	/* Clear MIPS ABI flags.  */
	exec_abiflags = std::ptr::null_mut ();                       // C 1083
	interpreter_abiflags = std::ptr::null_mut ();                // C 1084
	interpreter_header = std::ptr::null_mut ();                  // C 1085
      }

      /* Set header_offset to -1; `process_program_header' then updates it to that of the first
	 mapping.  */
      header_offset = USER_WORD::MAX;                                // C 1090

      i = 0;                                                         // C 1092: `for (i = 0; i < header.e_phnum; ++i)`
      loop
	{
	  if i >= header.e_phnum as c_int
	    { break; }                                               // C 1092
	  rc = read (fd, (&raw mut program) as *mut c_void,
		     std::mem::size_of_val (&program)) as c_int;     // C 1094
	  if rc < std::mem::size_of_val (&program) as c_int
	    { break 'fail1; }                                        // C 1095-1096

	  if process_program_header (interpreter_name, fd, &raw mut program, &raw mut header,
				     &raw mut entry, &raw mut header_offset) != 0
	    { break 'fail1; }                                        // C 1098-1101
	  i += 1;                                                    // C 1092
	}

      /* Write the entry point and program entry.  */

      jump.command = 3;                                              // C 1106
      jump.entry = entry;                                            // C 1107

      /* Now calculate values for the aux vector.  */

      jump.at_entry = program_entry.wrapping_add (offset);            // C 1111
      jump.at_phent = header.e_phentsize as USER_WORD;                // C 1112
      jump.at_phnum = header.e_phnum as USER_WORD;                    // C 1113
      jump.at_base = if entry == (header.e_entry as USER_WORD).wrapping_add (offset)
	{
	  EXECUTABLE_BASE
	}
      else
	{
	  INTERPRETER_BASE
	};                                                            // C 1114-1116

      #[cfg (all (feature = "mips", not (feature = "mips_nabi")))]
      {
	/* Finally, calculate the FPU mode wanted by the executable.  */

	if determine_fpu_mode (&raw mut header, interpreter_header, &raw mut fpu_mode,
			       exec_abiflags, interpreter_abiflags) != 0
	  /* N.B. that `determine_fpu_mode' sets errno.  */
	  { break 'fail; }                                            // C 1121-1125

	/* If the processor is too new to support FR0 operation, place the executable in floating
	   point emulation mode.  */

	if fpu_mode == FP_FR0 && !cpu_supports_fr0_p ()
	  { fpu_mode = FP_FRE; }                                     // C 1130-1131

	jump.fpu_mode = fpu_mode as USER_WORD;                       // C 1133
      }
      /* C 1134-1135: C's `#elif` is reached only when the arm above is off, so the port's
	 condition is that arm's complement plus C 1134's own terms.  */
      #[cfg (all (feature = "mips", feature = "mips_nabi", not (feature = "lp64")))]
      {
	jump.fpu_mode = 0;                                           // C 1135
      }

      /* The offset used for at_phdr should be that of the first mapping.  */

      if header_offset == USER_WORD::MAX
	{ header_offset = 0; }                                       // C 1141-1142

      jump.at_phdr = (header.e_phoff as USER_WORD).wrapping_add (header_offset);   // C 1144

      if std::mem::size_of_val (&loader_area).wrapping_sub (loader_area_used as usize)
	< std::mem::size_of_val (&jump)
	{ break 'fail1; }                                            // C 1146-1147

      memcpy (((&raw mut loader_area) as *mut c_char).offset (loader_area_used as isize)
		as *mut c_void, (&raw const jump) as *const c_void,
	      std::mem::size_of_val (&jump));                         // C 1149-1150
      loader_area_used += std::mem::size_of_val (&jump) as c_int;    // C 1151

      /* Copy the length of NAME and NAME itself to the loader area.  */
      name_len = strlen (name) as USER_WORD;                        // C 1154
      aligned_len = name_len.wrapping_add (1)
	.wrapping_add (std::mem::size_of_val (&name_len))
	.wrapping_sub (1)
	& !(std::mem::size_of_val (&name_len) - 1);                  // C 1155-1156
      if std::mem::size_of_val (&loader_area).wrapping_sub (loader_area_used as usize)
	< aligned_len.wrapping_add (std::mem::size_of_val (&name_len))
	{ break 'fail1; }                                            // C 1157-1159
      memcpy (((&raw mut loader_area) as *mut c_char).offset (loader_area_used as isize)
		as *mut c_void, (&raw const name_len) as *const c_void,
	      std::mem::size_of_val (&name_len));                    // C 1160
      loader_area_used += std::mem::size_of_val (&name_len) as c_int;   // C 1161
      memcpy (((&raw mut loader_area) as *mut c_char).offset (loader_area_used as isize)
		as *mut c_void, name as *const c_void, name_len as usize + 1);   // C 1162
      loader_area_used += (name_len + 1) as c_int;                   // C 1163

      /* Properly align the loader area.  */
      offset = aligned_len.wrapping_sub (name_len.wrapping_add (1));   // C 1166
      loop                                                           // C 1167: `while (offset--)`
	{
	  let was: USER_WORD = offset;
	  offset = offset.wrapping_sub (1);
	  if was == 0
	    { break; }                                               // C 1167
	  *((&raw mut loader_area) as *mut c_char).offset (loader_area_used as isize) = 0;   // C 1168
	  loader_area_used += 1;                                     // C 1168
	}

      /* Close the file descriptor and return the number of bytes used.  */

      close (fd);                                                   // C 1173
      *size = loader_area_used as usize;                            // C 1174

      /* Make sure the loader area is properly aligned.  */
      assert! (((loader_area_used as usize)
		& (std::mem::size_of::<USER_WORD> () - 1)) == 0);    // C 1177
      return (&raw mut loader_area) as *mut c_char;                  // C 1178
    }

    /* fail1: */
    *errno () = ENOEXEC;                                            // C 1181
  }

  /* fail: */
  close (fd);                                                       // C 1183
  return std::ptr::null_mut ();                                     // C 1184
}
