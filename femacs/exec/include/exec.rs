// femacs: mirror of exec/exec.h @ fd7afced403.
//
// The exec loader's own header: the 32- and 64-bit ELF structure sets, the typedefs that
// select one set, `struct exec_tracee`, and the loader's prototypes.  The frozen configuration
// compiles nothing under `exec/` (docs/baseline/frozen-build-sources.txt: "the reference build
// has no exec/ directory and no rule reaches it"), and `xtask layout-check` covers
// `src/include` and `lib/include` only, so the mirror is text: it is the target the port-map
// names for this header.
//
// **Four names in this header are the host's, not the file's** - they come from
// `exec/configure.ac`, and the reference build never runs it on this target:
//   * `USER_WORD` is `uintptr_t` on every host the file configures (configure.ac 252/286/318/351),
//     so the port spells it as the measured `uintptr_t` (PORTING.md 1.1: `usize`);
//   * `USER_REGS_STRUCT` is the host's register structure - `struct user_regs_struct`,
//     `struct user_regs` or `struct pt_regs` (configure.ac 242/276/308/341/367) - and the port
//     has no mirror for any of them, so the pointers are spelled the way the `nt/inc` mirrors
//     spell `PSECURITY_DESCRIPTOR` (P-72 (3)) with the macro named here;
//   * `USER_HEADER` is that structure's header (`<sys/user.h>`, `<asm/ptrace.h>`, ...; 241-242,
//     275-276, 307-308, 340-341, 366-367); an `#include` has no Rust counterpart and is recorded.
//   * `HAVE_STDINT_H` is C 20-22's guard on `<stdint.h>`, and the frozen configuration's is 1.
// `exec/exec.h` is not in `docs/baseline/config/cond-eval-verified.txt`, so both arms of
// C 30's `UINT64_MAX` test and C 135's `EXEC_64` test are carried (PORTING.md P-51).

include!("../../lib/include/stdint.in.rs");    // C 20-22: `#include <stdint.h>` (HAVE_STDINT_H)
include!("../../lib/include/sys_types.in.rs"); // C 24: `#include <sys/types.h>` (pid_t)

/* C 26: `#include USER_HEADER` - recorded above, with the hosts' values the reference
   configure script writes.  */

/* C 23-24's include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _EXEC_H_: bool = true;

/* C 30-37: "Define a replacement for `uint64_t' if it's not present in the C library."  */

/* C 32-36: `#ifndef UINT64_MAX` / `typedef struct { uint32_t word1; uint32_t word2; }
   xint64_t;`.  */
#[cfg (feature = "no_uint64_max")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xint64_t
{
  pub word1: uint32_t,
  pub word2: uint32_t,
}
/* C 38-39: `#else` / `typedef uint64_t xint64_t;`.  */
#[cfg (not (feature = "no_uint64_max"))]
pub type xint64_t = uint64_t;

/* C 43-60: "32-bit ELF headers."  */
/// C 44-60: `struct elf_header_32`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct elf_header_32
{
  pub e_ident: [u8; 16],        /* unsigned char e_ident[16]; */
  pub e_type: uint16_t,
  pub e_machine: uint16_t,
  pub e_version: uint32_t,
  pub e_entry: uint32_t,
  pub e_phoff: uint32_t,
  pub e_shoff: uint32_t,
  pub e_flags: uint32_t,
  pub e_ehsize: uint16_t,
  pub e_phentsize: uint16_t,
  pub e_phnum: uint16_t,
  pub e_shentsize: uint16_t,
  pub e_shnum: uint16_t,
  pub e_shstrndx: uint16_t,
}

/// C 62-72: `struct program_header_32`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct program_header_32
{
  pub p_type: uint32_t,
  pub p_offset: uint32_t,
  pub p_vaddr: uint32_t,
  pub p_paddr: uint32_t,
  pub p_filesz: uint32_t,
  pub p_memsz: uint32_t,
  pub p_flags: uint32_t,
  pub p_align: uint32_t,
}

/// C 74-78: `struct dt_entry_32`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dt_entry_32
{
  pub d_tag: uint32_t,
  pub d_val: uint32_t,
}

/// C 84-101: `struct elf_header_64`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct elf_header_64
{
  pub e_ident: [u8; 16],        /* unsigned char e_ident[16]; */
  pub e_type: uint16_t,
  pub e_machine: uint16_t,
  pub e_version: uint32_t,
  pub e_entry: xint64_t,
  pub e_phoff: xint64_t,
  pub e_shoff: xint64_t,
  pub e_flags: uint32_t,
  pub e_ehsize: uint16_t,
  pub e_phentsize: uint16_t,
  pub e_phnum: uint16_t,
  pub e_shentsize: uint16_t,
  pub e_shnum: uint16_t,
  pub e_shstrndx: uint16_t,
}

/// C 103-113: `struct program_header_64` - the field order is C's (the flag word comes second
/// here, unlike the 32-bit structure's third).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct program_header_64
{
  pub p_type: uint32_t,
  pub p_flags: uint32_t,
  pub p_offset: xint64_t,
  pub p_vaddr: xint64_t,
  pub p_paddr: xint64_t,
  pub p_filesz: xint64_t,
  pub p_memsz: xint64_t,
  pub p_align: xint64_t,
}

/// C 115-119: `struct dt_entry_64`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dt_entry_64
{
  pub d_tag: xint64_t,
  pub d_val: xint64_t,
}

/* The layout is arithmetic from the field types, not a measurement: no aarch64/x86-64 Linux
   toolchain is used by this port and `exec/` is compiled by no configuration it builds
   (docs/baseline/frozen-build-sources.txt's batch-F section).  */
const _: () = core::assert! (std::mem::size_of::<elf_header_32> () == 52);
const _: () = core::assert! (std::mem::size_of::<program_header_32> () == 32);
const _: () = core::assert! (std::mem::size_of::<dt_entry_32> () == 8);
const _: () = core::assert! (std::mem::size_of::<elf_header_64> () == 64);
const _: () = core::assert! (std::mem::size_of::<program_header_64> () == 56);
const _: () = core::assert! (std::mem::size_of::<dt_entry_64> () == 16);

/* C 125-133: "Define some types to the correct values."  C 127-129's `#ifdef EXEC_64` arm
   selects the 64-bit set and C 131-133's `#else` the 32-bit one.  */
#[cfg (feature = "exec_64")]
pub type elf_header = elf_header_64;
#[cfg (feature = "exec_64")]
pub type program_header = program_header_64;
#[cfg (feature = "exec_64")]
pub type dt_entry = dt_entry_64;
#[cfg (not (feature = "exec_64"))]
pub type elf_header = elf_header_32;
#[cfg (not (feature = "exec_64"))]
pub type program_header = program_header_32;
#[cfg (not (feature = "exec_64"))]
pub type dt_entry = dt_entry_32;

/* C 139: "Defined in trace.c."  */

/* `USER_WORD` is the first of the four host-supplied names the file note lists: the reference's
   `configure.ac` 252/286/318/351 make it `uintptr_t`, which the port spells `usize`
   (PORTING.md 1.1).  It had no declaration anywhere in the tree until `exec/test.rs` became the
   first unit to include this mirror (status 3.446).  */
pub type USER_WORD = usize;

/// C 143-177: `struct exec_tracee` - "Structure describing a process being traced."
/// `USER_WORD` is the host's `uintptr_t` (see the header note).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct exec_tracee
{
  /// The next process being traced.
  pub next: *mut exec_tracee,

  /// Address of any stack pointer to restore after system call completion.
  pub sp: USER_WORD,

  /// ID of the system call that is pending completion.  This value is not available as the
  /// call number is overwritten on success.
  pub callno: USER_WORD,

  /// Name of the executable being run.
  pub exec_file: *mut c_char,

  /// Pointer to a storage area holding instructions for loading an executable if an `exec`
  /// system call is outstanding, or NULL.
  pub exec_data: *mut c_char,

  /// Number of bytes in exec_data.
  pub data_size: usize,         /* size_t */

  /// The thread ID of this process.
  pub pid: pid_t,

  /// C 171-176's two `bool : 1` members: `waiting_for_syscall` and `new_child` share one
  /// storage byte in C.
  pub bits: u8,
}

/* The two `bool : 1` members of `struct exec_tracee`, named as C names them (PORTING.md P-80).  */
pub const EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT: u32 = 1 << 0;
pub const EXEC_TRACEE_NEW_CHILD_BIT: u32 = 1 << 1;

#[inline(always)]
pub fn exec_tracee_bit_p (t: *const exec_tracee, bit: u32) -> bool
{
  return (unsafe { (*t).bits } as u32 & bit) != 0;
}

#[inline(always)]
pub fn exec_tracee_bit_set (t: *mut exec_tracee, bit: u32, value: bool)
{
  if value { unsafe { (*t).bits |= bit as u8 } }
  else { unsafe { (*t).bits &= !(bit as u8) } }
}

macro_rules! exec_tracee_bits {
  (waiting_for_syscall, $t:expr) => { exec_tracee_bit_p (($t) as *const exec_tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT) };
  (waiting_for_syscall, $t:expr, $value:expr) =>
    { exec_tracee_bit_set (($t) as *mut exec_tracee, EXEC_TRACEE_WAITING_FOR_SYSCALL_BIT, $value) };
  (new_child, $t:expr) => { exec_tracee_bit_p (($t) as *const exec_tracee, EXEC_TRACEE_NEW_CHILD_BIT) };
  (new_child, $t:expr, $value:expr) =>
    { exec_tracee_bit_set (($t) as *mut exec_tracee, EXEC_TRACEE_NEW_CHILD_BIT, $value) };
}

const _: () = core::assert! (std::mem::size_of::<exec_tracee> () == 56);
const _: () = core::assert! (std::mem::offset_of! (exec_tracee, bits) == 52);

/* C 179-186: `#ifdef __aarch64__` - the two register accessors of that host.  */
#[cfg (feature = "aarch64")]
unsafe extern "C" {
  /// C 183: `extern int aarch64_get_regs (pid_t, USER_REGS_STRUCT *);`
  pub fn aarch64_get_regs (pid: pid_t, regs: *mut c_void) -> c_int;
  /// C 184: `extern int aarch64_set_regs (pid_t, USER_REGS_STRUCT *, bool);`
  pub fn aarch64_set_regs (pid: pid_t, regs: *mut c_void, set: bool) -> c_int;
}

/* C 190-197: the four declarations trace.c defines.  `USER_REGS_STRUCT`'s pointers are
   `*mut c_void` (see the header note).  */
unsafe extern "C" {
  /// C 190: `extern char *format_pid (char *, unsigned int);`
  pub fn format_pid (buffer: *mut c_char, pid: c_uint) -> *mut c_char;
  /// C 191-192: `extern USER_WORD user_alloca (struct exec_tracee *, USER_REGS_STRUCT *,
  /// USER_REGS_STRUCT *, USER_WORD);`
  pub fn user_alloca (tracee: *mut exec_tracee, first: *mut c_void, second: *mut c_void,
                      size: USER_WORD) -> USER_WORD;
  /// C 193-194: `extern int user_copy (struct exec_tracee *, const unsigned char *,
  /// USER_WORD, USER_WORD);`
  pub fn user_copy (tracee: *mut exec_tracee, from: *const u8, to: USER_WORD,
                    size: USER_WORD) -> c_int;
  /// C 195: `extern void exec_init (const char *);`
  pub fn exec_init (file: *const c_char);
}

/* C 199-202: the three declarations exec1.c defines.  */
unsafe extern "C" {
  /// C 199-200: `extern int tracing_execve (const char *, char *const *, char *const *);`
  pub fn tracing_execve (file: *const c_char, argv: *const *mut c_char,
                         envp: *const *mut c_char) -> c_int;
  /// C 201: `extern int after_fork (pid_t);`
  pub fn after_fork (child: pid_t) -> c_int;
  /// C 202: `extern pid_t exec_waitpid (pid_t, int *, int);`
  pub fn exec_waitpid (pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
}

/* C 204: "Defined in exec.c."  */
unsafe extern "C" {
  /// C 206-207: `extern char *exec_0 (char *, struct exec_tracee *, size_t *,
  /// USER_REGS_STRUCT *);`
  pub fn exec_0 (args: *mut c_char, tracee: *mut exec_tracee, count: *mut usize,
                 regs: *mut c_void) -> *mut c_char;
}
