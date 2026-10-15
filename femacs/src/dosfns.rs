// femacs: port of src/dosfns.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  The whole
// body sits inside C 23's `#ifdef MSDOS`; `MSDOS` is `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv` 2849, and `dosfns.o` belongs to the MS-DOS
// build's own object list rather than this target's: `emacs/src/Makefile.in` 306 leaves
// `MSDOS_OBJ` empty and `emacs/msdos/sed1v2.inp` 136 rewrites that line to
// `= dosfns.o msdos.o w16select.o` when the MS-DOS make files edit the generated Makefile,
// while `docs/baseline/frozen-build-sources.txt` lists no `dosfns.o` for this target.  So the
// text can never be *compiled* here, `tools/extract-abi.py`'s checklist for the file is empty
// (P-86 ②) and this unit's `## C line ranges` table in `docs/ports/dosfns.md` is that clause's
// substitute.
//
// **The file gate is `feature = "msdos"`** (C 23's `MSDOS`, registered in
// `femacs/xtask/Cargo.toml` 400), on every item *and on the include graph* (P-86 ⑧: an ungated
// `include!` would break the frozen compile at once, which is why the ledger's Evidence ① is an
// empty object).  C 21's `#include <config.h>` sits outside the gate, as C has it.  The
// conditions *inside* the file are:
//
//   * C 163, C 252, C 321, C 375, C 675 and C 738's `#ifndef HAVE_X_WINDOWS` - `not (feature =
//     "have_x_windows")` (registered, Cargo.toml 359), the spelling `src/msdos.rs` 20 already
//     uses for the same macro.  The frozen target leaves `HAVE_X_WINDOWS` undefined, so these
//     arms are the live side *of their condition*; the whole file is still behind the `msdos`
//     gate, which no configuration this machine can build sets.
//   * C 275 and C 537/C 572's `#ifndef SYSTEM_MALLOC` - `feature = "no_system_malloc"` (the
//     registered `!SYSTEM_MALLOC`, Cargo.toml 438), the spelling `src/msdos.rs` 659 uses for the
//     same condition.  C 537-539 and C 572-577 each have an *implied* `#else` arm that the
//     preprocessor forms from the unguarded statements below them (P-51); both arms are carried.
//
// **Acquisition: this piece is `text-only` (P-86 ③⑵).**  The machine has no DJGPP toolchain
// (`which -a i586-pc-msdosdjgpp-gcc i686-pc-msdosdjgpp-gcc djgpp-gcc` is empty, and no `djgpp` or
// `go32` include directory exists under `/opt/homebrew`, `/usr/local` or `/Library`), and the
// reference tree carries none of the headers this file includes from the runtime (`<dos.h>`,
// `<dpmi.h>`, `<go32.h>`, `<dirent.h>`, `<sys/vfs.h>`, `<unistd.h>`, `<grp.h>`, `<crt0.h>`).
// There is therefore **no arm flip on a DJGPP target** and **no measurement of any record, value
// or prototype of theirs**: every one of them is `UNVERIFIED` (documented DJGPP text, not a
// reading) and the ledger's Owed list names the toolchain.  What this unit carries instead is the
// frozen `build-tu` reading, the P-86 ② machine-checked range table, the `#if` verdicts of
// `tools/check-cond-eval.py`, a per-item C citation and the per-file error census of the `msdos`
// arm.
//
// `docs/ports/dosfns.md` carries the C line ranges, the acceptance readings, every deviation and
// the items the integrator owes.  No behaviour of this file was executed anywhere: there is no
// MS-DOS target to compile the arm against and no DOS to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

/* C 1-19 is the file's opening ownership/title block, dropped (PORTING.md 0.1-19); the port's
   first line is the revision line above.  */

include!("include/config.rs");                 // C 21: `#include <config.h>`
/* C's `config.h` ends with `AH_BOTTOM([#include <conf_post.h>])`, so the preprocessor pulls that
   header in here; the mirror is *not* carried, because no name this unit uses comes from it, and
   under the `msdos` gate its three MS-DOS overrides (`ENOTSUP`, `EOPNOTSUPP`,
   `_GL_DT_NOTDIR`) collide with the values `lib/include/errno.rs` and `dirent.rs` already
   declare - measured, and recorded in the ledger as owed (the `src/msdos.rs` 92 reading of the
   same edge).  */

/* C 23: `#ifdef MSDOS` - the file gate.  C 24's note is C's own text and is carried.  */
/* The entire file is within this conditional */

#[cfg (feature = "msdos")]
include!("../lib/include/stdio.rs");            // C 26: `#include <stdio.h>`

/* C 27-28: C's note on why `<dos.h>` is reached through a rename; carried as C's comment.  */
/* gettime and settime in dos.h clash with their namesakes from
   gnulib, so we move out of our way the prototypes in dos.h.  */

/* C 29-30's `#define gettime dos_h_gettime_` / `#define settime dos_h_settime_` and C 32-33's
   `#undef` pair move `<dos.h>`'s two prototypes out of gnulib's way.  Rust declares neither, so
   the pair has no counterpart here: the port's `<dos.h>` section below declares no `gettime` or
   `settime` (the `src/msdos.rs` 171-174 reading of the same four lines in the sibling file).  */

/* ----------------------------------------------------------------------------------------------
   C 31: `#include <dos.h>`.

   The names this file takes from it stand here (P-88 ①: the declaration site is C's own
   `#include` line, and no file is added under `femacs/src/include/`).  **No measurement was
   possible** - this machine has no DJGPP toolchain (see the file note) - so the view order, the
   record sizes and every value below are `UNVERIFIED`; the port carries exactly the members this
   file's own call sites read or write (the `src/android.rs` `GdkRGBA` reading, P-88 ②).
   ---------------------------------------------------------------------------------------------- */
/// C 31's `<dos.h>`: `union REGS`, the register block `int86`/`intdos` pass to the BIOS and DOS.
/// **UNVERIFIED** (no DJGPP toolchain here): the two views' order and the record's size are the
/// header's documented ones, not a reading.
/// C: `union REGS { struct { unsigned short ax, bx, cx, dx, si, di, cflag, flags; } x; struct { unsigned char al, ah, …; } h; … };`
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct RegsX
{
  pub ax: c_ushort,                     // C 83/281/304: `regs.x.ax`
  pub bx: c_ushort,                     // C 84/298/319: `regs.x.bx`
  pub cx: c_ushort,                     // C 85: `regs.x.cx`
  pub dx: c_ushort,                     // C 86: `regs.x.dx`
  pub si: c_ushort,                     // C 87: `regs.x.si`
  pub di: c_ushort,                     // C 88: `regs.x.di`
  pub cflag: c_ushort,                  // C 89/306: `regs.x.cflag`
  pub flags: c_ushort,                  // C 90: `regs.x.flags`
}
/// C 31's `<dos.h>`: the 8-bit halves (`regs.h.…`) of the same block.  **UNVERIFIED** for the
/// same reason; only the half of `ax` this file reads is carried.
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct RegsH
{
  pub al: c_uchar,                      // C 283/342: `regs.h.al`
  pub ah: c_uchar,                      // C 283/344: `regs.h.ah`
}
#[cfg (feature = "msdos")]
#[repr(C)]
pub union REGS
{
  pub x: RegsX,
  pub h: RegsH,
}
/// C 31's `<dos.h>`: the two interrupt entry points this file calls (C 86/282/305) and the
/// millisecond delay `restore_parent_vm_title` waits with (C 264).  None of the three has a
/// declaration anywhere in the reference tree (they are DJGPP runtime names), so which DJGPP
/// header supplies each is unmeasured; the section is `<dos.h>` because that is where this
/// unit's runtime names live.  **UNVERIFIED** for the same missing toolchain.
/// C: int int86 (int, union REGS *, union REGS *);
/// C: int intdos (union REGS *, union REGS *);
/// C: void delay (unsigned);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn int86 (intno: c_int, inregs: *mut REGS, outregs: *mut REGS) -> c_int;
  pub fn intdos (inregs: *mut REGS, outregs: *mut REGS) -> c_int;
  pub fn delay (msec: c_uint);
}

/* C 35-47: the file's own mirrors.  C 35's `lisp.h` is first, as PORTING.md 0.1-14 requires
   (`lisp.rs` carries the `eassert!`/`CHECK_*` machinery the others expand).  */
#[cfg (feature = "msdos")]
include!("include/lisp.rs");                   // C 35: `#include "lisp.h"`
/* lisp.h 28's `<stdcountof.h>` is `lib/include/stdcountof.rs`'s mirror (the `src/msdos.rs`
   146 reading of the same edge); C 398 and C 416 use `countof`.  */
#[cfg (feature = "msdos")]
include!("../lib/include/stdcountof.rs");
/* lisp.h 69-73's `max`/`min` macros are `lib/include/min-max.rs`'s mirror (the
   `src/w32console.rs` 346 reading of the same edge); C 655 uses `max`.  */
#[cfg (feature = "msdos")]
include!("../lib/include/min-max.rs");
#[cfg (feature = "msdos")]
include!("include/character.rs");              // C 36: `#include "character.h"`
#[cfg (feature = "msdos")]
include!("include/buffer.rs");                 // C 37: `#include "buffer.h"`
#[cfg (feature = "msdos")]
include!("include/termchar.rs");               // C 38: `#include "termchar.h"`
#[cfg (feature = "msdos")]
include!("include/frame.rs");                  // C 39: `#include "frame.h"`
#[cfg (feature = "msdos")]
include!("include/termhooks.rs");              // C 40: `#include "termhooks.h"`
#[cfg (feature = "msdos")]
include!("include/blockinput.rs");             // C 41: `#include "blockinput.h"`
#[cfg (feature = "msdos")]
include!("include/window.rs");                 // C 42: `#include "window.h"`
#[cfg (feature = "msdos")]
include!("include/dosfns.rs");                 // C 43: `#include "dosfns.h"`
#[cfg (feature = "msdos")]
include!("include/msdos.rs");                  // C 44: `#include "msdos.h"`
#[cfg (feature = "msdos")]
include!("include/dispextern.rs");             // C 45: `#include "dispextern.h"`
#[cfg (feature = "msdos")]
include!("include/coding.rs");                 // C 46: `#include "coding.h"`
#[cfg (feature = "msdos")]
include!("include/process.rs");                // C 47: `#include "process.h"`

/* ----------------------------------------------------------------------------------------------
   C 48-54's system headers.  `<unistd.h>`, `<grp.h>`, `<dirent.h>` and the `struct statfs` that
   `<sys/vfs.h>` carries have mirrors in this tree; `<dpmi.h>`, `<go32.h>` and `<crt0.h>` have none
   (P-88 ①: the declaration site is C's own `#include` line, and no file is added under
   `femacs/src/include/`).  **No measurement was possible** for the three (see the file note), so
   every record, member and value in their sections is `UNVERIFIED` and the ledger's Owed list
   names the toolchain.
   ---------------------------------------------------------------------------------------------- */
#[cfg (feature = "msdos")]
include!("../lib/include/unistd.in.rs");       // C 52: `#include <unistd.h>`
#[cfg (feature = "msdos")]
include!("../lib/include/grp.rs");             // C 53: `#include <grp.h>`
#[cfg (feature = "msdos")]
include!("../lib/include/dirent.in.rs");       // C 50: `#include <dirent.h>`

/* C 50's `<dirent.h>` on DJGPP owns two names the tree's own mirror does not declare: the
   directory-scan flag block `__opendir_flags` and the `__OPENDIR_FIND_HIDDEN` bit C 372 stores in
   it.  Both are the DJGPP header's, not gnulib's.  **UNVERIFIED**: the toolchain is absent, so
   the bit's value is the header's documented one and not a reading (the ledger's Owed).  */
/// C: extern int __opendir_flags;
/// C: #define __OPENDIR_FIND_HIDDEN 0x0001
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub static mut __opendir_flags: c_int;
}
#[cfg (feature = "msdos")]
pub const __OPENDIR_FIND_HIDDEN: c_int = 0x0001;

/* C 51's `<sys/vfs.h>` carries `struct statfs` and `statfs`; the tree's mirror for the frozen
   target's spelling of the same pair is `lib/include/sys_mount.rs`, whose C 55-70 `struct statfs`
   (the four fields C 496-498 reads) and its own `statfs` declaration at its 98 are both in this
   unit's graph, so neither is repeated here.  */
#[cfg (feature = "msdos")]
include!("../lib/include/sys_mount.rs");       // C 51: `#include <sys/vfs.h>`

/* C 48: `#include <dpmi.h>` - the header this section stands for.  */
/// C 48's `<dpmi.h>`: `_go32_dpmi_registers`, the DPMI register block `init_dosfns` and
/// `w95_set_virtual_machine_title` pass to the real-mode interrupt simulator.  **UNVERIFIED** (no
/// DJGPP toolchain): the view order and the record's size are the header's documented ones; only
/// the members this file's call sites touch are carried.
/// C: struct _go32_dpmi_registers { struct { unsigned short ax, bx, cx, dx, si, di, ds, es, ss, sp, flags; } x; struct { unsigned char al, ah, …; } h; … };
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct Go32DpmiRegsX
{
  pub ax: c_ushort,                     // C 286/325/351: `dpmiregs.x.ax`
  pub bx: c_ushort,                     // C 298: `dpmiregs.x.bx`
  pub cx: c_ushort,                     // C 353: `dpmiregs.x.cx`
  pub dx: c_ushort,                     // C 288/352: `dpmiregs.x.dx`
  pub si: c_ushort,                     // C: the view's own member
  pub di: c_ushort,                     // C 355: `dpmiregs.x.di`
  pub ds: c_ushort,                     // C 287: `dpmiregs.x.ds`
  pub es: c_ushort,                     // C 354: `dpmiregs.x.es`
  pub ss: c_ushort,                     // C 289/326/356: `dpmiregs.x.ss`
  pub sp: c_ushort,                     // C 289/326/356: `dpmiregs.x.sp`
  pub flags: c_ushort,                  // C 289/291/326/356: `dpmiregs.x.flags`
}
/// C 48's `<dpmi.h>`: the 8-bit halves (`dpmiregs.h.…`) of the same block.  **UNVERIFIED** for
/// the same reason; only the pair C 339-340 reads is carried.
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct Go32DpmiRegsH
{
  pub al: c_uchar,                      // C 339-340: `dpmiregs.h.al`
  pub ah: c_uchar,                      // C 340/344: `dpmiregs.h.ah`
}
#[cfg (feature = "msdos")]
#[repr(C)]
pub union _go32_dpmi_registers
{
  pub x: Go32DpmiRegsX,
  pub h: Go32DpmiRegsH,
}
/// C 48's `<dpmi.h>`: the real-mode interrupt simulator (C 290/327/357).
/// C: int _go32_dpmi_simulate_int (int, _go32_dpmi_registers *);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn _go32_dpmi_simulate_int (vector: c_int, regs: *mut _go32_dpmi_registers);
}
/// C 48's `<dpmi.h>`: the DPMI memory report `dos_memory_info` reads.
/// **UNVERIFIED**: the record's layout and every value in it are the header's documented ones;
/// only the four members C 644-664 reads are carried.
/// C: struct _go32_dpmi_meminfo { unsigned long …, available_memory, available_physical_pages, …, max_pages_in_paging_file, …, total_physical_pages, …; };
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct _go32_dpmi_meminfo
{
  pub available_memory: c_ulong,                // C 644: `info.available_memory`
  pub available_physical_pages: c_ulong,        // C 645: `info.available_physical_pages`
  pub max_pages_in_paging_file: c_ulong,        // C 658: `info.max_pages_in_paging_file`
  pub total_physical_pages: c_ulong,            // C 662/664: `info.total_physical_pages`
}
/// C: void _go32_dpmi_get_free_memory_information (_go32_dpmi_meminfo *);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn _go32_dpmi_get_free_memory_information (info: *mut _go32_dpmi_meminfo);
}

/* C 49: `#include <go32.h>` - the header this section stands for.  */
/// C 49's `<go32.h>`: the info block `init_dosfns` reads the transfer buffer's linear address
/// from.  **UNVERIFIED** (no DJGPP toolchain); only the member C 273 names is carried.
/// C: struct _go32_info_block { int selector_for_linear_memory; …; unsigned long linear_address_of_transfer_buffer; … }; extern struct _go32_info_block _go32_info_block;
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct _go32_info_block
{
  pub selector_for_linear_memory: c_int,
  pub linear_address_of_transfer_buffer: c_ulong,   // C 273: `_go32_info_block.linear_address_of_transfer_buffer`
}
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub static mut _go32_info_block: _go32_info_block;
  /// C 49's `<go32.h>`: the two DOS-memory transfers this file makes (C 94, C 116, C 147).
  /// **UNVERIFIED**: the return type is the header's documented `void`, which is also why no
  /// call below reads one (`src/msdos.rs` 603 spells `dosmemput` `-> c_int`; the two units'
  /// excerpts disagree and the ledger records it).
  /// C: void dosmemget (int, int, void *);
  /// C: void dosmemput (const void *, int, int);
  pub fn dosmemget (offset: c_int, length: c_int, memory: *mut c_void);
  pub fn dosmemput (buffer: *const c_void, length: c_int, offset: c_int);
}
/// C 49's `<go32.h>`: `__tb`, the transfer buffer's linear address, which C's header defines as
/// the info block's field.  A *place* macro in C, so the port carries it as a nullary
/// `macro_rules!` (the `include/msdos.rs` 635 `_dos_ds` shape).  **UNVERIFIED** for the same
/// missing toolchain.
/// C: #define __tb (_go32_info_block.linear_address_of_transfer_buffer)
#[cfg (feature = "msdos")]
macro_rules! __tb { () => { _go32_info_block.linear_address_of_transfer_buffer } }

/* C 54: `#include <crt0.h>` - the header this section stands for.  */
/// C 54's `<crt0.h>`: the startup file's argument vector, which `system_process_attributes` walks
/// to rebuild the command line.  **UNVERIFIED** (no DJGPP toolchain); both are the header's
/// documented names.
/// C: int __crt0_argc; char **__crt0_argv;   (read at C 551, 581, 583-584, 587, 596, 598)
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub static mut __crt0_argc: c_int;
  pub static mut __crt0_argv: *mut *mut c_char;
}

/* ----------------------------------------------------------------------------------------------
   Names this unit reaches whose owners are elsewhere in the tree and which no mirror in this
   unit's graph declares: each stands here at its use site (P-88 ①) with C's own text above it.
   ---------------------------------------------------------------------------------------------- */
/// C 569/663's `sbrk` is `<unistd.h>`'s; `lib/include/unistd.in.rs` does not declare it, and
/// `src/gmalloc.rs` 1807-1810 is the tree's shape for the same declaration.
/// C: unistd.h: `void *sbrk (intptr_t increment);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn sbrk (increment: ptrdiff_t) -> *mut c_void;
}
/// C 551's `basename` and C 584/598's `strpbrk` are `<string.h>`'s and no mirror in this unit's
/// graph declares either (the `lib-src/pop.rs` 314 / `nt/cmdproxy.rs` 320 reading of the same
/// gap).  `strcpy`, `strlen` and `memcpy` are `lib/include/string.rs`'s.
/// C: string.h: `char *basename (char *);`
/// C: string.h: `char *strpbrk (const char *, const char *);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn basename (path: *mut c_char) -> *mut c_char;
  pub fn strpbrk (s: *const c_char, accept: *const c_char) -> *mut c_char;
}
/// C 543's `getlogin` is `<unistd.h>`'s; this tree's `unistd.in.rs` declares only the
/// `replace_getlogin` rename macro, so the call stands here (the `lib-src/pop.rs` 314 shape).
/// C: unistd.h: `char *getlogin (void);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn getlogin () -> *mut c_char;
}
/// C 684's `is_tty_frame` is `dispextern.h` 4012's declaration of `dispnew.c`'s definition, and
/// the mirror in this unit's graph does not carry it; declared here with C's own prototype.
/// C: dispextern.h 4012: `bool is_tty_frame (struct frame *f);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn is_tty_frame (f: *mut frame) -> bool;
}
/// C 276/278's `get_lim_data` and C 538/575's `ret_lim_data` are `vm-limit.c`'s, defined in this
/// tree by `src/vm-limit.rs` 115/131; no mirror declares either.
/// C: lisp.h: `void get_lim_data (void);`
/// C: vm-limit.c: `unsigned long ret_lim_data (void);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn get_lim_data ();
  pub fn ret_lim_data () -> c_ulong;
}

/* C 56-97: `int86`, the MS-DOS interrupt subr.  C's `union REGS inregs, outregs;` has no
   initializer and the port zeroes both blocks (PORTING.md 0.1-13: Rust has no partial
   initialization, so a block takes C's zero value first and every field C assigns below keeps its
   own store); C's `register int i` is a plain `let mut i: c_int`.  */
#[cfg (feature = "msdos")]
DEFUN! ("int86", Fint86, Sint86, 2, 2, 0,
       doc: "Call specific MS-DOS interrupt number INTERRUPT with REGISTERS.\nReturn the updated REGISTER vector.\n\nINTERRUPT should be an integer in the range 0 to 255.\nREGISTERS should be a vector produced by `dos-make-register' and\n`dos-set-register-value'.  ",   // C 57-62
       (interrupt: Lisp_Object, registers: Lisp_Object),
       {
  let mut i: c_int;
  let mut no: c_int;
  let mut inregs: REGS = std::mem::zeroed ();
  let mut outregs: REGS = std::mem::zeroed ();

  CHECK_FIXNUM (interrupt);
  no = (XFIXNUM (interrupt) as c_ulong) as c_int;
  CHECK_VECTOR (registers);
  if no < 0 || no > 0xff || ASIZE (registers) != 8
    { return Qnil (); }
  i = 0;
  while i < 8
    {
      CHECK_FIXNUM (AREF (registers, i as isize));
      i += 1;
    }

  inregs.x.ax    = (XFIXNUM (AREF (registers, 0)) as c_ulong) as c_ushort;
  inregs.x.bx    = (XFIXNUM (AREF (registers, 1)) as c_ulong) as c_ushort;
  inregs.x.cx    = (XFIXNUM (AREF (registers, 2)) as c_ulong) as c_ushort;
  inregs.x.dx    = (XFIXNUM (AREF (registers, 3)) as c_ulong) as c_ushort;
  inregs.x.si    = (XFIXNUM (AREF (registers, 4)) as c_ulong) as c_ushort;
  inregs.x.di    = (XFIXNUM (AREF (registers, 5)) as c_ulong) as c_ushort;
  inregs.x.cflag = (XFIXNUM (AREF (registers, 6)) as c_ulong) as c_ushort;
  inregs.x.flags = (XFIXNUM (AREF (registers, 7)) as c_ulong) as c_ushort;

  int86 (no, &raw mut inregs, &raw mut outregs);

  ASET (registers, 0, make_fixnum (outregs.x.ax as EMACS_INT));
  ASET (registers, 1, make_fixnum (outregs.x.bx as EMACS_INT));
  ASET (registers, 2, make_fixnum (outregs.x.cx as EMACS_INT));
  ASET (registers, 3, make_fixnum (outregs.x.dx as EMACS_INT));
  ASET (registers, 4, make_fixnum (outregs.x.si as EMACS_INT));
  ASET (registers, 5, make_fixnum (outregs.x.di as EMACS_INT));
  ASET (registers, 6, make_fixnum (outregs.x.cflag as EMACS_INT));
  ASET (registers, 7, make_fixnum (outregs.x.flags as EMACS_INT));

  return registers;
});

/* C 100-122: `msdos-memget`.  C's `buf = alloca (len)` is the port's one deviation here: there is
   no `alloca` (PORTING.md P-05), so the block comes from the stack pool `sa_stack_alloc` serves -
   the `src/msdos.rs` 1749 shape, whose `sa_avail` accounting C does not do here either.  C's
   `buf[i]` is a signed `char` on DJGPP and the port keeps `c_char` (this target's `c_char` is
   `i8`/`u8` by platform; the ledger records that the sign is the DJGPP one only).  */
#[cfg (feature = "msdos")]
DEFUN! ("msdos-memget", Fdos_memget, Sdos_memget, 2, 2, 0,
       doc: "Read DOS memory at offset ADDRESS into VECTOR.\nReturn the updated VECTOR.  ",   // C 102
       (address: Lisp_Object, vector: Lisp_Object),
       {
  let mut i: c_int;
  let mut offs: c_int;
  let mut len: c_int;
  let mut buf: *mut c_char;

  CHECK_FIXNUM (address);
  offs = (XFIXNUM (address) as c_ulong) as c_int;
  CHECK_VECTOR (vector);
  len = ASIZE (vector) as c_int;
  if len < 1 || len > 2048 || offs < 0 || offs > 0xfffff - len
    { return Qnil (); }
  buf = sa_stack_alloc (len as usize) as *mut c_char;
  dosmemget (offs, len, buf as *mut c_void);

  i = 0;
  while i < len
    {
      ASET (vector, i as isize, make_fixnum (*buf.offset (i as isize) as EMACS_INT));
      i += 1;
    }

  return vector;
});

/* C 124-148: `msdos-memput` - the same `alloca` deviation as C 100's.  */
#[cfg (feature = "msdos")]
DEFUN! ("msdos-memput", Fdos_memput, Sdos_memput, 2, 2, 0,
       doc: "Write DOS memory at offset ADDRESS from VECTOR.  ",   // C 126
       (address: Lisp_Object, vector: Lisp_Object),
       {
  let mut i: c_int;
  let mut offs: c_int;
  let mut len: c_int;
  let mut buf: *mut c_char;

  CHECK_FIXNUM (address);
  offs = (XFIXNUM (address) as c_ulong) as c_int;
  CHECK_VECTOR (vector);
  len = ASIZE (vector) as c_int;
  if len < 1 || len > 2048 || offs < 0 || offs > 0xfffff - len
    { return Qnil (); }
  buf = sa_stack_alloc (len as usize) as *mut c_char;

  i = 0;
  while i < len
    {
      CHECK_FIXNUM (AREF (vector, i as isize));
      *buf.offset (i as isize) = ((XFIXNUM (AREF (vector, i as isize)) as c_uchar) & 0xFF) as c_char;
      i += 1;
    }

  dosmemput (buf as *const c_void, len, offs);
  return Qt ();
});

/* C 150-161: `msdos-set-keyboard`.  C's `!dos_set_keyboard (…)` is written as the `== 0` test its
   `int` return means.  */
#[cfg (feature = "msdos")]
DEFUN! ("msdos-set-keyboard", Fmsdos_set_keyboard, Smsdos_set_keyboard, 1, 2, 0,
       doc: "Set keyboard layout according to COUNTRY-CODE.\nIf the optional argument ALLKEYS is non-nil, the keyboard is mapped for\nall keys; otherwise it is only used when the ALT key is pressed.\nThe current keyboard layout is available in dos-keyboard-code.  ",   // C 151-154
       (country_code: Lisp_Object, allkeys: Lisp_Object),
       {
  CHECK_FIXNUM (country_code);
  if dos_set_keyboard (XFIXNUM (country_code) as c_int, (! NILP (allkeys)) as c_int) == 0
    { return Qnil (); }
  return Qt ();
});

/* C 163-176: `#ifndef HAVE_X_WINDOWS` - the mouse-presence subr.  C's comment (C 164-165) is
   C's own text and is carried.  */
/* Later we might want to control the mouse interface with this function,
   e.g., with respect to non-80 column screen modes.  */

#[cfg (feature = "msdos")]
#[cfg (not (feature = "have_x_windows"))]
DEFUN! ("msdos-mouse-p", Fmsdos_mouse_p, Smsdos_mouse_p, 0, 0, 0,
       doc: "Report whether a mouse is present.  ",   // C 168
       (),
       {
  if have_mouse != 0
    { return Qt (); }
  else
    { return Qnil (); }
});

/* C 178-210: the three mouse-control subrs.  */
#[cfg (feature = "msdos")]
DEFUN! ("msdos-mouse-init", Fmsdos_mouse_init, Smsdos_mouse_init, 0, 0, "",
       doc: "Initialize and enable mouse if available.  ",   // C 179
       (),
       {
  if have_mouse != 0
    {
      have_mouse = 1;
      mouse_init ();
      return Qt ();
    }
  return Qnil ();
});

#[cfg (feature = "msdos")]
DEFUN! ("msdos-mouse-enable", Fmsdos_mouse_enable, Smsdos_mouse_enable, 0, 0, "",
       doc: "Enable mouse if available.  ",   // C 192
       (),
       {
  if have_mouse != 0
    {
      have_mouse = 1;
      mouse_on ();
    }
  return if have_mouse != 0 { Qt () } else { Qnil () };
});

#[cfg (feature = "msdos")]
DEFUN! ("msdos-mouse-disable", Fmsdos_mouse_disable, Smsdos_mouse_disable, 0, 0, "",
       doc: "Disable mouse if available.  ",   // C 204
       (),
       {
  mouse_off ();
  if have_mouse != 0 { have_mouse = -1; }
  return Qnil ();
});

/* C 212-234: `insert-startup-screen`.  C's `s += 2` is the pointer's own advance (the text-mode
   screen holds a character and an attribute byte per cell).  */
#[cfg (feature = "msdos")]
DEFUN! ("insert-startup-screen", Finsert_startup_screen, Sinsert_startup_screen, 0, 0, "",
       doc: "Insert copy of screen contents prior to starting Emacs.\nReturn nil if startup screen is not available.  ",   // C 213-214
       (),
       {
  /* C's `char *s; int rows, cols;` are written by `dos_get_saved_screen`; Rust cannot take a
     `&raw mut` of an uninitialized binding, so the three carry C's own zero value first
     (PORTING.md 0.1-13).  */
  let mut s: *mut c_char = std::ptr::null_mut ();
  let mut rows: c_int = 0;
  let mut cols: c_int = 0;
  let mut i: c_int;
  let mut j: c_int;

  if dos_get_saved_screen (&raw mut s, &raw mut rows, &raw mut cols) == 0
    { return Qnil (); }

  i = 0;
  while i < rows
    {
      j = 0;
      while j < cols
	{
	  insert_char (*s as c_int);
	  s = s.offset (2);
	  j += 1;
	}
      insert_char ('\n' as c_int);
      i += 1;
    }

  return Qt ();
});

/* C 236-250: the two country-information blocks.  C's `unsigned char dos_country_info[34];` is a
   tentative definition and Rust spells the zero its own initializer supplies; the `usa_country_info`
   array is `static` in C, so it stays a plain `static mut` here.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub static mut dos_country_info: [u8; DOS_COUNTRY_INFO] = [0; DOS_COUNTRY_INFO];
#[cfg (feature = "msdos")]
static mut usa_country_info: [u8; DOS_COUNTRY_INFO] = [
  0, 0,					/* date format */
  b'$' as u8, 0, 0, 0, 0,		/* currency string */
  b',' as u8, 0,			/* thousands separator */
  b'.' as u8, 0,			/* decimal separator */
  b'/' as u8, 0,			/* date separator */
  b':' as u8, 0,			/* time separator */
  0,					/* currency format */
  2,					/* digits after decimal in currency */
  0,					/* time format */
  0, 0, 0, 0,				/* address of case map routine, GPF if used */
  b' ' as u8, 0,			/* data-list separator (?) */
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0		/* reserved */
];

/* C 252-266: `#ifndef HAVE_X_WINDOWS` - the MS-Windows version record and the VM title.  C 255's
   forward declaration of `w95_set_virtual_machine_title` is dropped (PORTING.md 0.1-12); its
   definition is C 445 below, inside the same condition.  */
#[cfg (feature = "msdos")]
#[cfg (not (feature = "have_x_windows"))]
static mut dos_windows_version: c_uint = 0;
#[cfg (feature = "msdos")]
#[cfg (not (feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub static mut parent_vm_title: [c_char; 50] = [0; 50];	/* Ralf Brown says 30 is enough */

#[cfg (feature = "msdos")]
#[cfg (not (feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_parent_vm_title ()          // C 257-258
{
  if NILP (Vdos_windows_version)
    { return; }
  if (dos_windows_version & 0xff) >= 4 && parent_vm_title[0] != 0
    { w95_set_virtual_machine_title (std::ptr::addr_of! (parent_vm_title) as *const c_char); }
  delay (50);
}

/* C 268-373: `init_dosfns`.  C 275-279's `#ifndef SYSTEM_MALLOC` declares `get_lim_data` (dropped,
   0.1-12) and calls it; the arm is the `no_system_malloc` feature.  C 291's `dpmiregs.x.flags & 1`
   is written as the `!= 0` its test means.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_dosfns ()          // C 268-269
{
  let mut regs: REGS = std::mem::zeroed ();
  let mut dpmiregs: _go32_dpmi_registers = std::mem::zeroed ();
  let xbuf: c_ulong = _go32_info_block.linear_address_of_transfer_buffer;

  #[cfg (feature = "no_system_malloc")]
  {
  get_lim_data (); /* why the hell isn't this called elsewhere? */
  }

  regs.x.ax = 0x3000;
  intdos (&raw mut regs, &raw mut regs);
  Vdos_version = Fcons (make_fixnum (regs.h.al as EMACS_INT),
			make_fixnum (regs.h.ah as EMACS_INT));

  /* Obtain the country code via DPMI, use DJGPP transfer buffer.  */
  dpmiregs.x.ax = 0x3800;
  dpmiregs.x.ds = (xbuf >> 4) as c_ushort;
  dpmiregs.x.dx = 0;
  /* C's `dpmiregs.x.ss = dpmiregs.x.sp = dpmiregs.x.flags = 0;` is a chained assignment; the
     port writes the three stores, innermost first (the `src/msdos.rs` 2998 shape).  */
  dpmiregs.x.flags = 0;
  dpmiregs.x.sp = 0;
  dpmiregs.x.ss = 0;
  _go32_dpmi_simulate_int (0x21, &raw mut dpmiregs);
  if dpmiregs.x.flags & 1 != 0
    {
      dos_country_code = 1;	/* assume USA if 213800 failed */
      memcpy (std::ptr::addr_of_mut! (dos_country_info) as *mut c_void,
	      std::ptr::addr_of! (usa_country_info) as *const c_void, DOS_COUNTRY_INFO);
    }
  else
    {
      dos_country_code = dpmiregs.x.bx as EMACS_INT;
      dosmemget (xbuf as c_int, DOS_COUNTRY_INFO as c_int,
		 std::ptr::addr_of_mut! (dos_country_info) as *mut c_void);
    }

  dos_set_keyboard (dos_country_code as c_int, 0);

  regs.x.ax = 0x6601;
  intdos (&raw mut regs, &raw mut regs);
  if regs.x.cflag != 0
    /* Estimate code page from country code */
    { match dos_country_code
      {
      45 | 47 =>			/* Denmark, Norway */
	{
	  dos_codepage = 865;
	}
      _ =>
	{
	  /* US */
	  dos_codepage = 437;
	}
      }
    }
  else
    { dos_codepage = (regs.x.bx & 0xffff) as EMACS_INT; }

  #[cfg (not (feature = "have_x_windows"))]
  {
  parent_vm_title[0] = 0;

  /* If we are running from DOS box on MS-Windows, get Windows version.  */
  dpmiregs.x.ax = 0x1600;	/* enhanced mode installation check */
  /* C's second chained assignment, in its own inner-to-outer order.  */
  dpmiregs.x.flags = 0;
  dpmiregs.x.sp = 0;
  dpmiregs.x.ss = 0;
  _go32_dpmi_simulate_int (0x2f, &raw mut dpmiregs);
  /* We only support Windows-specific features when we run on Windows 9X
     or on Windows 3.X/enhanced mode.

     Int 2Fh/AX=1600h returns:

     AL = 00:  no Windows at all;
     AL = 01:  Windows/386 2.x;
     AL = 80h: Windows 3.x in mode other than enhanced;
     AL = FFh: Windows/386 2.x

     We also check AH > 0 (Windows 3.1 or later), in case AL tricks us.  */
  if dpmiregs.h.al as c_int > 2 && dpmiregs.h.al != 0x80 && dpmiregs.h.al != 0xff
      && (dpmiregs.h.al as c_int > 3 || dpmiregs.h.ah as c_int > 0)
    {
      dos_windows_version = dpmiregs.x.ax as c_uint;
      Vdos_windows_version =
	Fcons (make_fixnum (dpmiregs.h.al as EMACS_INT), make_fixnum (dpmiregs.h.ah as EMACS_INT));

      /* Save the current title of this virtual machine, so we can restore
	 it before exiting.  Otherwise, Windows 95 will continue to use
	 the title we set even after we are history, stupido...  */
      if dpmiregs.h.al as c_int >= 4
	{
	  dpmiregs.x.ax = 0x168e;
	  dpmiregs.x.dx = 3;	/* get VM title */
	  dpmiregs.x.cx = (std::mem::size_of::<[c_char; 50]> () - 1) as c_ushort;
	  dpmiregs.x.es = (__tb! () >> 4) as c_ushort;
	  dpmiregs.x.di = (__tb! () & 15) as c_ushort;
	  /* C's third chained assignment, in its own inner-to-outer order.  */
	  dpmiregs.x.flags = 0;
	  dpmiregs.x.ss = 0;
	  dpmiregs.x.sp = 0;
	  _go32_dpmi_simulate_int (0x2f, &raw mut dpmiregs);
	  if dpmiregs.x.ax == 1
	    { dosmemget (__tb! () as c_int, std::mem::size_of::<[c_char; 50]> () as c_int,
			 std::ptr::addr_of_mut! (parent_vm_title) as *mut c_void); }
	}
    }
  else
    {
      dos_windows_version = 0;
      Vdos_windows_version = Qnil ();
    }
  }

  /* Without this, we never see hidden files.
     Don't OR it with the previous value, so the value recorded at dump
     time, possibly with `preserve-case' flags set, won't get through.  */
  __opendir_flags = __OPENDIR_FIND_HIDDEN;
}

/* C 375-480: `#ifndef HAVE_X_WINDOWS` - the emulation of the X colour entry points.  C's comment
   (C 377) is carried as C's text.  */
/* Emulation of some X window features from xfns.c and xfaces.c.  */

/* Standard VGA colors, in the order of their standard numbering
   in the default VGA palette.  */
#[cfg (feature = "msdos")]
#[cfg (not (feature = "have_x_windows"))]
static mut vga_colors: [*const c_char; 16] = [
  c"black".as_ptr (), c"blue".as_ptr (), c"green".as_ptr (), c"cyan".as_ptr (), c"red".as_ptr (),
  c"magenta".as_ptr (), c"brown".as_ptr (),
  c"lightgray".as_ptr (), c"darkgray".as_ptr (), c"lightblue".as_ptr (), c"lightgreen".as_ptr (),
  c"lightcyan".as_ptr (),
  c"lightred".as_ptr (), c"lightmagenta".as_ptr (), c"yellow".as_ptr (), c"white".as_ptr ()
];

/* Given a color name, return its index, or -1 if not found.  Note
   that this only performs case-insensitive comparison against the
   standard names.  For anything more sophisticated, like matching
   "gray" with "grey" or translating X color names into their MSDOS
   equivalents, call the Lisp function Qtty_color_desc (defined
   on lisp/term/tty-colors.el).  */
#[cfg (feature = "msdos")]
#[cfg (not (feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn msdos_stdcolor_idx (name: *const c_char) -> c_int   // C 393-394
{
  let mut i: c_int;

  i = 0;
  while i < countof! (vga_colors) as c_int
    {
      if dispextern_externs::xstrcasecmp (name, vga_colors[i as usize]) == 0
	{ return i; }
      i += 1;
    }

  /* C's three `unsigned long` colour constants narrow to the subr's `int` return on the way out
     (C's implicit conversion), and `unpecified_fg`/`unspecified_bg` are `char[]` in C, addressed
     explicitly here.  */
  return
    if strcmp (name, std::ptr::addr_of! (dispextern_unit_names::unspecified_fg) as *const c_char) == 0
      { dispextern_unit_names::FACE_TTY_DEFAULT_FG_COLOR as c_int }
    else { if strcmp (name, std::ptr::addr_of! (dispextern_unit_names::unspecified_bg) as *const c_char) == 0
	     { dispextern_unit_names::FACE_TTY_DEFAULT_BG_COLOR as c_int }
	   else
	     { dispextern_unit_names::FACE_TTY_DEFAULT_COLOR as c_int } };
}

/* Given a color index, return its standard name.  */
#[cfg (feature = "msdos")]
#[cfg (not (feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn msdos_stdcolor_name (idx: c_int) -> Lisp_Object   // C 409-410
{
  if idx as c_ulong == dispextern_unit_names::FACE_TTY_DEFAULT_FG_COLOR
    { return build_string (std::ptr::addr_of! (dispextern_unit_names::unspecified_fg) as *const c_char); }
  else if idx as c_ulong == dispextern_unit_names::FACE_TTY_DEFAULT_BG_COLOR
    { return build_string (std::ptr::addr_of! (dispextern_unit_names::unspecified_bg) as *const c_char); }
  else if idx >= 0 && (idx as usize) < countof! (vga_colors)
    { return build_string (vga_colors[idx as usize]); }
  else
    { return Qunspecified (); }	/* meaning the default */
}

/* Support for features that are available when we run in a DOS box
   on MS-Windows.  */
#[cfg (feature = "msdos")]
#[cfg (not (feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ms_windows_version () -> c_int   // C 424-425
{
  return dos_windows_version as c_int;
}

/* Set the title of the current virtual machine, to appear on
   the caption bar of that machine's window.  */

#[cfg (feature = "msdos")]
#[cfg (not (feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w95_set_virtual_machine_title (title_string: *const c_char) -> c_int   // C 433-434
{
  /* Only Windows 9X (version 4 and higher) support this function.  */
  if ! NILP (Vdos_windows_version)
      && (dos_windows_version & 0xff) >= 4
    {
      let mut regs: _go32_dpmi_registers = std::mem::zeroed ();
      dosmemput (title_string as *const c_void, (strlen (title_string) + 1) as c_int, __tb! () as c_int);
      regs.x.ax = 0x168e;
      regs.x.dx = 1;
      regs.x.es = (__tb! () >> 4) as c_ushort;
      regs.x.di = (__tb! () & 15) as c_ushort;
      /* C's chained assignment, inner-to-outer.  */
      regs.x.flags = 0;
      regs.x.ss = 0;
      regs.x.sp = 0;
      _go32_dpmi_simulate_int (0x2f, &raw mut regs);
      return (regs.x.ax == 1) as c_int;
    }
  return 0;
}

/* Change the title of frame F to NAME.
   If NAME is nil, use the frame name as the title.

   If Emacs is not run from a DOS box on Windows 9X, this only
   sets the name in the frame struct, but has no other effects.  */

#[cfg (feature = "msdos")]
#[cfg (not (feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_set_title (f: *mut frame, name: Lisp_Object)   // C 459-460
{
  /* C 470-471 reassign the `name` parameter; DEFUN-less C functions take a rebinding here (the
     `src/androidfns.rs` 581 shape).  */
  let mut name = name;

  /* Don't change the title if it's already NAME.  */
  if EQ (name, (*f).title)
    { return; }

  update_mode_lines = 13;

  fset_title (f, name);

  if NILP (name)
    { name = (*f).name; }

  if FRAME_MSDOS_P (f)
    {
      block_input ();
      w95_set_virtual_machine_title (SDATA (name) as *mut c_char);
      unblock_input ();
    }
}

/* C 482-501: `file-system-info`.  The docstring is C's own `SKIP:` marker, which
   `lib-src/make-docfile.c` reads out of `fileio.c`'s real doc (PORTING.md 5.1).  C 490
   reassigns the `filename` parameter, so the port rebinds it (the `src/androidfns.rs` 581
   shape).  */
#[cfg (feature = "msdos")]
DEFUN! ("file-system-info", Ffile_system_info, Sfile_system_info, 1, 1, 0,
       doc: "SKIP: real doc in fileio.c.  ",   // C 483
       (filename: Lisp_Object),
       {
  let mut stfs: statfs = std::mem::zeroed ();
  let mut encoded: Lisp_Object;
  let mut value: Lisp_Object;

  /* C 490 reassigns the parameter.  */
  let mut filename = filename;

  CHECK_STRING (filename);
  filename = Fexpand_file_name (filename, Qnil ());
  encoded = ENCODE_FILE (filename);

  if statfs (SDATA (encoded) as *const c_char, &raw mut stfs) != 0
    { value = Qnil (); }
  else
    { value = list3 (make_float ((stfs.f_bsize as f64) * (stfs.f_blocks as f64)),
		     make_float ((stfs.f_bsize as f64) * (stfs.f_bfree as f64)),
		     make_float ((stfs.f_bsize as f64) * (stfs.f_bavail as f64))); }

  return value;
});

/* System depended enumeration of and access to system processes a-la
   ps(1).  Here, we only return info about the running Emacs process.
   (There are no other processes on DOS, right?)  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn list_system_processes () -> Lisp_Object   // C 507-508
{
  let mut proclist: Lisp_Object = Qnil ();

  proclist = Fcons (INT_TO_INTEGER (getpid () as EMACS_INT), proclist);

  return proclist;
}

/* C 517-629: `system_process_attributes`.  C's `struct group *gr` and `char cmd[FILENAME_MAX]`
   are the `lib/include/grp.rs` record and a fixed array; `char *cmdline = NULL` is a null
   pointer, not an empty buffer.  C 538's forward declaration of `ret_lim_data` is dropped
   (PORTING.md 0.1-12); its declaration stands with the other use-site ones above, and C 572's
   `#ifndef SYSTEM_MALLOC` / `#endif` pair is carried as both arms (P-51): with the macro
   undefined, C 573-575 compute `pmem` and clamp it; with it defined, C 577 is the whole arm.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_process_attributes (pid: Lisp_Object) -> Lisp_Object   // C 517-518
{
  let mut proc_id: c_int;
  let mut attrs: Lisp_Object = Qnil ();

  CHECK_NUMBER (pid);
  proc_id = XFLOATINT (pid) as c_int;

  if proc_id == getpid ()
    {
      let mut uid: EMACS_INT;
      let mut gid: EMACS_INT;
      let mut usr: *mut c_char;
      let mut gr: *mut group;
      let mut cmd: [c_char; FILENAME_MAX as usize] = [0; FILENAME_MAX as usize];
      let mut cmdline: *mut c_char = std::ptr::null_mut ();
      let mut p: *mut c_char;
      let mut q: *mut c_char;
      let mut cmdline_size: usize = 0;
      let mut i: c_int;
      let mut cmd_str: Lisp_Object;
      let mut decoded_cmd: Lisp_Object;
      let mut tem: Lisp_Object;
      let mut pmem: f64;

      uid = getuid () as EMACS_INT;
      attrs = Fcons (Fcons (Qeuid (), INT_TO_INTEGER (uid)), attrs);
      usr = getlogin ();
      if ! usr.is_null ()
	{ attrs = Fcons (Fcons (Quser (), build_string (usr)), attrs); }
      gid = getgid () as EMACS_INT;
      attrs = Fcons (Fcons (Qegid (), INT_TO_INTEGER (gid)), attrs);
      gr = getgrgid (gid as gid_t);
      if ! gr.is_null ()
	{ attrs = Fcons (Fcons (Qgroup (), build_string ((*gr).gr_name)), attrs); }
      strcpy (std::ptr::addr_of_mut! (cmd) as *mut c_char, basename (*__crt0_argv));
      /* Command name is encoded in locale-coding-system; decode it.  */
      cmd_str = build_unibyte_string (std::ptr::addr_of! (cmd) as *const c_char);
      decoded_cmd = code_convert_string_norecord (cmd_str,
						  Vlocale_coding_system, false);
      attrs = Fcons (Fcons (Qcomm (), decoded_cmd), attrs);
      /* Pretend we have 0 as PPID.  */
      attrs = Fcons (Fcons (Qppid (), make_fixnum (0)), attrs);
      attrs = Fcons (Fcons (Qpgrp (), pid), attrs);
      attrs = Fcons (Fcons (Qttname (), build_string (c"/dev/tty".as_ptr ())), attrs);
      /* We are never idle!  */
      tem = Fget_internal_run_time ();
      attrs = Fcons (Fcons (Qtime (), tem), attrs);
      attrs = Fcons (Fcons (Qthcount (), make_fixnum (1)), attrs);
      attrs = Fcons (Fcons (Qstart (),
			    Fsymbol_value (Qbefore_init_time ())),
		     attrs);
      attrs = Fcons (Fcons (Qvsize (),
			    INT_TO_INTEGER (((sbrk (0) as c_ulong) / 1024) as EMACS_INT)),
		     attrs);
      attrs = Fcons (Fcons (Qetime (), tem), attrs);
      #[cfg (feature = "no_system_malloc")]
      {
      /* ret_lim_data is on vm-limit.c, which is not compiled in under
	 SYSTEM_MALLOC.  */
      pmem = ((sbrk (0) as c_ulong) as f64) / (ret_lim_data () as f64) * 100.0;
      if pmem > 100.0
	{ pmem = 100.0; }
      }
      #[cfg (not (feature = "no_system_malloc"))]
      { pmem = 100.0; }
      attrs = Fcons (Fcons (Qpmem (), make_float (pmem)), attrs);
      /* Pass 1: Count how much storage we need.  */
      i = 0;
      while i < __crt0_argc
	{
	  cmdline_size += strlen (*__crt0_argv.offset (i as isize)) + 1; /* +1 for blank delim */
	  if ! strpbrk (*__crt0_argv.offset (i as isize), c" \t\n\r\x0b\x0c".as_ptr ()).is_null ()
	    {
	      cmdline_size += 2;
	      p = *__crt0_argv.offset (i as isize);
	      while *p != 0
		{
		  if *p == b'"' as c_char
		    { cmdline_size += 1; }
		  p = p.offset (1);
		}
	    }
	  i += 1;
	}
      /* Pass 2: Allocate storage and concatenate argv[].  */
      cmdline = xmalloc (cmdline_size + 1) as *mut c_char;
      i = 0;
      q = cmdline;
      while i < __crt0_argc
	{
	  if ! strpbrk (*__crt0_argv.offset (i as isize), c" \t\n\r\x0b\x0c".as_ptr ()).is_null ()
	    {
	      *q = b'"' as c_char;
	      q = q.offset (1);
	      p = *__crt0_argv.offset (i as isize);
	      while *p != 0
		{
		  if *p == b'"' as c_char
		    {
		      *q = b'\\' as c_char;
		      q = q.offset (1);
		    }
		  *q = *p;
		  q = q.offset (1);
		  p = p.offset (1);
		}
	      *q = b'"' as c_char;
	      q = q.offset (1);
	    }
	  else
	    {
	      strcpy (q, *__crt0_argv.offset (i as isize));
	      q = q.offset (strlen (*__crt0_argv.offset (i as isize)) as isize);
	    }
	  *q = b' ' as c_char;
	  q = q.offset (1);
	  i += 1;
	}
      /* Remove the trailing blank.  */
      if q > cmdline
	{ *q.offset (-1) = 0; }

      /* Command line is encoded in locale-coding-system; decode it.  */
      cmd_str = build_unibyte_string (cmdline);
      decoded_cmd = code_convert_string_norecord (cmd_str,
						  Vlocale_coding_system, false);
      xfree (cmdline as *mut c_void);
      attrs = Fcons (Fcons (Qargs (), decoded_cmd), attrs);
    }

  return attrs;
}

/* Support for memory-info.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dos_memory_info (totalram: *mut c_ulong, freeram: *mut c_ulong,
					   totalswap: *mut c_ulong, freeswap: *mut c_ulong) -> c_int   // C 632-634
{
  let mut info: _go32_dpmi_meminfo = std::mem::zeroed ();
  let mut mem1: c_ulong;
  let mut mem2: c_ulong;
  let mut freemem: c_ulong;

  _go32_dpmi_get_free_memory_information (&raw mut info);
  /* DPMI server of Windows NT and its descendants reports in
     info.available_memory a much lower amount that is really
     available, which causes bogus "past 95% of memory limit"
     warnings.  Try to overcome that via circumstantial evidence.  */
  mem1 = info.available_memory;
  mem2 = info.available_physical_pages;
  /* DPMI Spec: "Fields that are unavailable will hold -1."  */
  if (mem1 as c_long) == -1
    { mem1 = 0; }
  if (mem2 as c_long) == -1
    { mem2 = 0; }
  else
    { mem2 *= 4096; }
  /* Surely, the available memory is at least what we have physically
     available, right?  */
  freemem = max! (mem1, mem2);
  *freeram = freemem;
  *totalswap =
    if (info.max_pages_in_paging_file as c_long) == -1
    { 0 }
    else { info.max_pages_in_paging_file * 4096 };
  *totalram =
    if (info.total_physical_pages as c_long) == -1
    { freemem + (sbrk (0) as c_ulong) + *totalswap }
    else { info.total_physical_pages * 4096 };
  *freeswap = 0;
  return 0;
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dos_cleanup ()          // C 670-671
{
  let mut tty: *mut tty_display_info;

  #[cfg (not (feature = "have_x_windows"))]
  {
  restore_parent_vm_title ();
  }
  /* Make sure the termscript file is committed, in case we are
     crashing and some vital info was written there.  */
  if FRAMEP (selected_frame)
    {
      let sf: *mut frame = XFRAME (selected_frame);

      if FRAME_LIVE_P (sf) && is_tty_frame (sf)
	{
	  tty = CURTTY! ();
	  if ! (*tty).termscript.is_null ()
	    {
	      fflush ((*tty).termscript);
	      fsync (fileno ((*tty).termscript));
	    }
	}
    }
}

/* The eleven `globals.f_<name>` fields this unit's `syms_of_dosfns` registers (its `DEFVAR_INT`/
   `DEFVAR_LISP` calls at C 715, 719, 731, 735, 739, 743, 751, 756, 761, 785 and 790; C 739's
   `dos-windows-version` carries C 738's `#ifndef HAVE_X_WINDOWS` too), defined here as the port's
   storage: C zero-initialises `globals` and the macros take each field's address, so the
   initialisers are `lisp_h_Qnil ()` / `0` (docs/status.md 3.29, the `src/cmds.rs` 489-492 row).  */
#[cfg (feature = "msdos")] #[unsafe(no_mangle)] pub static mut dos_country_code: EMACS_INT = 0;   // C 715
#[cfg (feature = "msdos")] #[unsafe(no_mangle)] pub static mut dos_codepage: EMACS_INT = 0;   // C 719
#[cfg (feature = "msdos")] #[unsafe(no_mangle)] pub static mut dos_timezone_offset: EMACS_INT = 0;   // C 731
#[cfg (feature = "msdos")] #[unsafe(no_mangle)] pub static mut Vdos_version: Lisp_Object = lisp_h_Qnil ();   // C 735
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))] #[unsafe(no_mangle)] pub static mut Vdos_windows_version: Lisp_Object = lisp_h_Qnil ();   // C 739
#[cfg (feature = "msdos")] #[unsafe(no_mangle)] pub static mut Vdos_display_scancodes: Lisp_Object = lisp_h_Qnil ();   // C 743
#[cfg (feature = "msdos")] #[unsafe(no_mangle)] pub static mut dos_hyper_key: EMACS_INT = 0;   // C 751
#[cfg (feature = "msdos")] #[unsafe(no_mangle)] pub static mut dos_super_key: EMACS_INT = 0;   // C 756
#[cfg (feature = "msdos")] #[unsafe(no_mangle)] pub static mut dos_keypad_mode: EMACS_INT = 0;   // C 761
#[cfg (feature = "msdos")] #[unsafe(no_mangle)] pub static mut dos_keyboard_layout: EMACS_INT = 0;   // C 785
#[cfg (feature = "msdos")] #[unsafe(no_mangle)] pub static mut dos_decimal_point: EMACS_INT = 0;   // C 790

/*
 *	Define everything
 */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_dosfns ()          // C 699-700
{
  defsubr (&raw mut Sint86);
  defsubr (&raw mut Sdos_memget);
  defsubr (&raw mut Sdos_memput);
  defsubr (&raw mut Smsdos_mouse_init);
  defsubr (&raw mut Smsdos_mouse_enable);
  defsubr (&raw mut Smsdos_set_keyboard);
  defsubr (&raw mut Sinsert_startup_screen);
  defsubr (&raw mut Smsdos_mouse_disable);
  defsubr (&raw mut Sfile_system_info);
  #[cfg (not (feature = "have_x_windows"))]
  { defsubr (&raw mut Smsdos_mouse_p); }

  DEFVAR_INT! (c"dos-country-code", dos_country_code,
	      "The country code returned by Dos when Emacs was started.\nUsually this is the international telephone prefix.  ");   // C 716-717

  DEFVAR_INT! (c"dos-codepage", dos_codepage,
	      "The codepage active when Emacs was started.\nThe following are known:\n\t437\tUnited States\n\t850\tMultilingual (Latin I)\n\t852\tSlavic (Latin II)\n\t857\tTurkish\n\t860\tPortugal\n\t861\tIceland\n\t863\tCanada (French)\n\t865\tNorway/Denmark  ");   // C 720-729

  DEFVAR_INT! (c"dos-timezone-offset", dos_timezone_offset,
	      "The current timezone offset to UTC in minutes.\nImplicitly modified when the TZ variable is changed.  ");   // C 732-733

  DEFVAR_LISP! (c"dos-version", Vdos_version,
	       "The (MAJOR . MINOR) Dos version (subject to modification with setver).  ");   // C 736

  #[cfg (not (feature = "have_x_windows"))]
  {
  DEFVAR_LISP! (c"dos-windows-version", Vdos_windows_version,
	       "The (MAJOR . MINOR) Windows version for DOS session on MS-Windows.  ");   // C 740
  }

  DEFVAR_LISP! (c"dos-display-scancodes", Vdos_display_scancodes,
	       "Whether DOS raw keyboard events are displayed as you type.\nWhen non-nil, the keyboard scan-codes are displayed at the bottom right\ncorner of the display (typically at the end of the mode line).\nThe output format is: scan code:char code*modifiers.  ");   // C 744-747

  Vdos_display_scancodes = Qnil ();

  DEFVAR_INT! (c"dos-hyper-key", dos_hyper_key,
	      "If set to 1, use right ALT key as hyper key.\nIf set to 2, use right CTRL key as hyper key.  ");   // C 752-753
  dos_hyper_key = 0;

  DEFVAR_INT! (c"dos-super-key", dos_super_key,
	      "If set to 1, use right ALT key as super key.\nIf set to 2, use right CTRL key as super key.  ");   // C 757-758
  dos_super_key = 0;

  DEFVAR_INT! (c"dos-keypad-mode", dos_keypad_mode,
	      "Controls what key code is returned by a key in the numeric keypad.\nThe `numlock ON' action is only taken if no modifier keys are pressed.\nThe value is an integer constructed by adding the following bits together:\n\n  0x00\tDigit key returns digit    (if numlock ON)\n  0x01\tDigit key returns kp-digit (if numlock ON)\n  0x02\tDigit key returns M-digit  (if numlock ON)\n  0x03\tDigit key returns edit key (if numlock ON)\n\n  0x00\tGrey key returns char      (if numlock ON)\n  0x04\tGrey key returns kp-key    (if numlock ON)\n\n  0x00\tDigit key returns digit    (if numlock OFF)\n  0x10\tDigit key returns kp-digit (if numlock OFF)\n  0x20\tDigit key returns M-digit  (if numlock OFF)\n  0x30\tDigit key returns edit key (if numlock OFF)\n\n  0x00\tGrey key returns char      (if numlock OFF)\n  0x40\tGrey key returns kp-key    (if numlock OFF)\n\n  0x200\tALT-0..ALT-9 in top-row produces shifted codes.  ");   // C 762-782
  dos_keypad_mode = 0x75;

  DEFVAR_INT! (c"dos-keyboard-layout", dos_keyboard_layout,
	      "Contains the country code for the current keyboard layout.\nUse msdos-set-keyboard to select another keyboard layout.  ");   // C 786-787
  dos_keyboard_layout = 1;	/* US */

  DEFVAR_INT! (c"dos-decimal-point", dos_decimal_point,
	      "The character to produce when kp-decimal key is pressed.\nIf non-zero, this variable contains the character to be returned when the\ndecimal point key in the numeric keypad is pressed when Num Lock is on.\nIf zero, the decimal point key returns the country code specific value.  ");   // C 791-794
  dos_decimal_point = 0;
  DEFSYM! (Qbefore_init_time, "before-init-time");
}

/* C 798: `#endif /* MSDOS */` - the file gate's own closing line.  Every item of this unit
   carries `#[cfg (feature = "msdos")]` instead, so the line has no counterpart; it is the end of
   `emacs/src/dosfns.c` and of this port.  */
