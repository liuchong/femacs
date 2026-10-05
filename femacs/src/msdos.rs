// femacs: port of src/msdos.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  The whole
// body sits inside C 34's `#ifdef MSDOS`, `MSDOS` is `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv` 2849, and the MS-DOS (djgpp) build is driven by
// the `msdos/` subdirectory's own make files - `emacs/msdos/sed1v2.inp` 136 rewrites the generated
// `src/Makefile.in` 307's empty `MSDOS_OBJ` into `= dosfns.o msdos.o w16select.o` (the macro
// itself is `MSDOS`, `configure.ac` 6953's `AH_TEMPLATE`), and
// `docs/baseline/frozen-build-sources.txt` lists no `msdos.o` for this target.  So the text can
// never be *compiled* here, `tools/extract-abi.py`'s checklist for the file is empty (P-86 ②) and
// this unit's `## C line ranges` table in `docs/ports/msdos.md` is that clause's substitute.
//
// **This batch is P-86 ④'s first slice: C 1-1295.**  It ends at the `}` of `IT_update_end`
// (C 1287-1295), from the file's first line through the whole mouse-control and the first half of
// the screen-control half: `event_timestamp`, the mouse section (`mouse_on`/`mouse_off`, the
// `msdos-set-mouse-buttons` subr, `mouse_get_pos`, `mouse_check_moved`, `mouse_clear_clicks`,
// `mouse_init`), the C 402-478 and C 480-1295 `#ifndef HAVE_X_WINDOWS` screen machinery
// (`dosv_refresh_virtual_screen`, `dos_direct_output`, `bright_bg`, `maybe_enable_blinking`,
// `vga_installed`, `dos_set_window_size`, `mouse_off_maybe`, `msdos_set_cursor_shape`,
// `IT_set_cursor_type`, `IT_ring_bell`, `IT_set_face`, `IT_write_glyphs`, `popup_activated`,
// `IT_write_glyphs_with_face`, `tty_draw_row_with_mouse_face`, `IT_clear_end_of_line`,
// `IT_clear_screen`, `IT_clear_to_end`, `IT_cursor_to`, `IT_display_cursor`, `IT_cmgoto`,
// `IT_update_begin`, `IT_update_end`).  **The next live definition is C 1297-1333's
// `IT_frame_up_to_date`** (C 1296 is its blank line), and the remainder (C 1296-4350) is the
// table's last row.  Both arms of every `#if` in the slice are carried.
//
// **The file gate is `feature = "msdos"`** (C 34's `MSDOS`), on every item *and on the include
// graph* (P-86 ⑧: an ungated `include!` would break the frozen build at once, which is why the
// reading in the ledger's Evidence ① is an empty object).  The slice's own conditions are:
//
//   * C 62 `#if (__DJGPP__ + (__DJGPP_MINOR__ > 3)) >= 3` - the djgpp version pair, spelled as the
//     tree already spells it for `src/msdos.h`/`src/conf_post.h` (`include/msdos.rs` 26/135,
//     `include/conf_post.rs` 98): the *else* arm (djgpp 2.3 and older, `SYS_ENVIRON = environ`)
//     carries `djgpp_2_3`, the true arm (`_environ`) is its negation.  The feature name is
//     registered (`femacs/xtask/Cargo.toml` 62).
//   * C 95 `#ifndef _USE_LFN` and C 99 `#ifndef _dos_ds` - the runtime's own macro either exists
//     (`<libc/dosio.h>`/`<go32.h>` supply it) or the reference defines the fallback below; the
//     port spells the two arms with the unregistered features `_use_lfn` / `_dos_ds` (see Owed).
//   * C 109 `#ifndef SYSTEM_MALLOC`, C 111 `#ifdef GNU_MALLOC`, C 119 `#ifdef REL_ALLOC` - the
//     allocator triple.  `no_system_malloc` and `rel_alloc` are registered (Cargo.toml 438/490);
//     `gnu_malloc` is not (Owed).
//   * C 402/C 480 `#ifndef HAVE_X_WINDOWS` - `not (feature = "have_x_windows")` (registered,
//     Cargo.toml 182); both open the MS-DOS screen code, which is why C 1784/3565/3671's
//     `#ifdef HAVE_X_WINDOWS` arms (past this slice) are the other side of the same switch.
//   * C 1171 `#if 0` - dead in every configuration; C's text is kept as a comment (the
//     `src/cm.rs` 86 / `src/character.rs` 118 shape).
//
// **The DJGPP runtime surface is declared in this unit (P-88 ①), and this batch is `text-only`
// (P-86 ③⑵).**  `<dos.h>`, `<dpmi.h>`, `<go32.h>`, `<sys/farptr.h>`, `<pc.h>`, `<conio.h>`,
// `<dir.h>`, `<io.h>`, `<libc/dosio.h>` and `<crt0.h>` have no file in the reference tree and no
// tree on this machine: there is **no DJGPP toolchain** (`which -a i586-pc-msdosdjgpp-gcc
// i686-pc-msdosdjgpp-gcc djgpp-gcc` is empty; no `djgpp` or `go32` include directory under
// `/opt/homebrew`, `/usr/local` or `/Library`), so **no arm flip on a DJGPP target** and **no
// measurement of any record or macro from those headers** - every size, offset and value of theirs
// is **UNVERIFIED** and named in the ledger's Owed list.  What this batch carries instead: the
// frozen `build-tu` reading, the P-86 ② machine-checked range table, the `#if` verdicts of
// `tools/check-cond-eval.py`, a per-item C citation, and a per-file error census of the `msdos`
// arm (whose errors are *all* in other files - the mirror graph has never been compiled with
// `msdos` on).
//
// `docs/ports/msdos.md` carries the C line ranges, the acceptance readings, every deviation and
// the items the integrator owes.  No behaviour of this file was executed anywhere: there is no
// MS-DOS target to compile the arm against and no DOS to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

/* C 20-21: the file's two contributor lines.  */
/* Contributed by Morten Welinder */
/* New display, keyboard, and mouse control by Kim F. Storm */

/* C 23-28: the file's own note on its encoding (it is unibyte on purpose; see
   `struct dos_keyboard_map` further down the reference).  */
/* Note: This file MUST use a unibyte encoding, to both display the
   keys on the non-US keyboard layout as their respective labels, and
   provide the correct byte values for the keyboard input to inject
   into Emacs.  See 'struct dos_keyboard_map' below.  As long as there
   are only European keyboard layouts here, we are OK with DOS
   codepage 850 encoding.  */

/* C 30: where part of the file came from.  */
/* Note: some of the stuff here was taken from end of sysdep.c in demacs. */

include!("include/config.rs");                 // C 32: `#include <config.h>`

/* C 34: `#ifdef MSDOS` - the file gate.  C 35 through the end of the file sit inside it, and
   every `include!` below carries it (P-86 ⑧); the C 20-30 notes and C 32's `<config.h>` are the
   only text outside it.  */

/* C 35's `#include <setjmp.h>` has no mirror of its own: `jmp_buf` and `sys_setjmp` are
   `include/lisp.rs`'s (C 36, the `src/thread.rs` 31-32 / `src/image.rs` 78-79 reading), and this
   unit reaches them through C 36.  */
/* C 36's `lisp.h` 30 is `#include <string.h>`, and this unit's `strcmp` (C 1794) is that
   header's; the mirror does not carry the edge, so the unit spells it here (the C 36
   `<stdcountof.h>` edge below is the same shape - Deviations 13).  */
#[cfg (feature = "msdos")]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (`strcmp`)
#[cfg (feature = "msdos")]
include!("include/lisp.rs");                   // C 36: `#include "lisp.h"`
#[cfg (feature = "msdos")]
include!("../lib/include/stdcountof.rs");      // lisp.h 28: `#include <stdcountof.h>` (`countof`)
#[cfg (feature = "msdos")]
include!("../lib/include/stdio.rs");           // C 37: `#include <stdio.h>` (`fprintf`, `fputc`, `FILE`)
#[cfg (feature = "msdos")]
include!("../lib/include/time.in.rs");         // C 38: `#include <time.h>` (`clock_t`)
/* C 39's `#include <sys/param.h>` has no mirror; no name this slice takes comes from it (the
   `src/pdumper.rs` 82 reading).  */
#[cfg (feature = "msdos")]
include!("../lib/include/sys_time.rs");        // C 40: `#include <sys/time.h>`

/* C 43-47's `#define gettime dos_h_gettime_` / `#undef gettime` (and the `settime` twin) move
   `<dos.h>`'s prototypes out of gnulib's way.  Rust declares neither, so the pair has no
   counterpart here: the only `gettime` in this unit's scope is gnulib's, through C 69's
   `systime.h` chain (`lib/include/timespec.rs` 91).  One `## Deviations` entry.  */

/* ----------------------------------------------------------------------------------------------
   C 45, 51, 56, 57, 58, 59 and 60's includes: `<dos.h>`, `<dir.h>`, `<io.h>`, `<dpmi.h>`,
   `<sys/farptr.h>`, `<libc/dosio.h>` and `<conio.h>` - the DJGPP runtime's own headers.  None of
   them has a file in the reference tree (P-88 ①: the declaration site is C's own `#include` line,
   and no file is added under `femacs/src/include/`).

   **No measurement was possible**: this machine has no DJGPP toolchain (see the file note), so no
   size, offset, value or prototype below was read from a header - each is spelled from this
   file's own *call sites* and is **UNVERIFIED**; the ledger's Owed list names the toolchain.
   `<dir.h>`'s `getdisk`, `<io.h>`'s `setmode` and `<conio.h>`'s `cputs` (C 51/56/60's own
   comments) are the names of those headers this slice does *not* use, so nothing is declared for
   them here.  ---------------------------------------------------------------------------------- */
// C 45: `#include <dos.h>` - the header this section stands for.
/// C: `union REGS { struct { unsigned short …, ax, … } x; struct { unsigned char …, al, ah, … } h; … };`
/// C 45's `<dos.h>`: `union REGS`, the register block `int86`/`intdos` pass to the BIOS/DOS.
/// **UNVERIFIED**: no DJGPP toolchain, so the view order, the record's size and the aliasing
/// between the two views are unmeasured; the port carries exactly the members this slice reads or
/// writes (the `src/gtkutil.rs` 539 `GdkRGBA` reading).
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct RegsX
{
  pub ax: c_ushort,                     // C 175/192/242/…: `regs.x.ax`
  pub bx: c_ushort,                     // C 274/276/281/…: `regs.x.bx`
  pub cx: c_ushort,                     // C 244/258/260/…: `regs.x.cx`
  pub dx: c_ushort,                     // C 245/259/261/…: `regs.x.dx`
  pub es: c_ushort,                     // C 458/1433: `regs.x.es`
  pub di: c_ushort,                     // C 459/1438: `regs.x.di`
}
/// C 45's `<dos.h>`: the 8-bit halves (`regs.h.…`) of the same block.  **UNVERIFIED** for the
/// same reason (and the `al`/`ah` half of `ax` is the header's business, not this record's).
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct RegsH
{
  pub al: c_uchar,                      // C 523/…: `regs.h.al`
  pub ah: c_uchar,                      // C 457/735/793: `regs.h.ah`
  pub bl: c_uchar,                      // C 495/509: `regs.h.bl`
  pub dl: c_uchar,                      // C 794: `inregs.h.dl`
}
#[cfg (feature = "msdos")]
#[repr(C)]
pub union REGS
{
  pub x: RegsX,
  pub h: RegsH,
}
/// C 45's `<dos.h>`: the two interrupt entry points this slice calls.
/// C: int int86 (int, union REGS *, union REGS *);
/// C: int intdos (union REGS *, union REGS *);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn int86 (intno: c_int, inregs: *mut REGS, outregs: *mut REGS) -> c_int;
  pub fn intdos (inregs: *mut REGS, outregs: *mut REGS) -> c_int;
}
// C 57: `#include <dpmi.h>` (`/* for __dpmi_xxx stuff */`) - the header this section stands for.
/// C 57's `<dpmi.h>`: `__dpmi_regs`, the DPMI register block `__dpmi_int` passes to a real-mode
/// interrupt.  **UNVERIFIED**: the same missing toolchain; only the members C 457-461/735-737
/// touch are carried.
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct DpmiRegsH
{
  pub ah: c_uchar,                      // C 457/735: `regs.h.ah`
}
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct DpmiRegsX
{
  pub es: c_ushort,                     // C 458: `regs.x.es`
  pub di: c_ushort,                     // C 459: `regs.x.di`
  pub cx: c_ushort,                     // C 460/737: `regs.x.cx`
}
#[cfg (feature = "msdos")]
#[repr(C)]
pub union __dpmi_regs
{
  pub h: DpmiRegsH,
  pub x: DpmiRegsX,
}
/// C 57's `<dpmi.h>`: the interrupt call (C 461, 737).
/// C: int __dpmi_int (int, __dpmi_regs *);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn __dpmi_int (vector: c_int, regs: *mut __dpmi_regs) -> c_int;
}
/// C 57's `<dpmi.h>`: `_go32_dpmi_registers`, the *dpmi* register block (a different header type
/// from `__dpmi_regs` above) that `dos_set_keyboard` (C 2055) and `dos_rawgetc` (C 2429) use.
/// **UNVERIFIED**: no DJGPP toolchain, so the record's size and the view order are unmeasured;
/// slice 2 carries only the members its own call sites touch (C 2060-2063: `x.ax`, `x.ss`, `x.sp`,
/// `x.flags`, `h.al`).
/// C: struct _go32_dpmi_registers { struct { unsigned short ax, …, ss, sp, flags; } x; struct { unsigned char al, ah, …; } h; … };
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct Go32DpmiRegsX
{
  pub ax: c_ushort,                     // C 2060: `regs.x.ax`
  pub ss: c_ushort,                     // C 2061: `regs.x.ss`
  pub sp: c_ushort,                     // C 2061: `regs.x.sp`
  pub flags: c_ushort,                  // C 2061: `regs.x.flags`
}
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct Go32DpmiRegsH
{
  pub al: c_uchar,                      // C 2063: `regs.h.al`
}
#[cfg (feature = "msdos")]
#[repr(C)]
pub union _go32_dpmi_registers
{
  pub x: Go32DpmiRegsX,
  pub h: Go32DpmiRegsH,
}
/// C 57's `<dpmi.h>`: the real-mode interrupt simulator (C 2062).
/// C: int _go32_dpmi_simulate_int (int, _go32_dpmi_registers *);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn _go32_dpmi_simulate_int (vector: c_int, regs: *mut _go32_dpmi_registers);
}
// C 58: `#include <sys/farptr.h>` (`/* for _farsetsel, _farnspokeb */`) - the header this
// section stands for.
/// C 58's `<sys/farptr.h>`: the far-memory accessors.  `<sys/farptr.h>` spells them as inlines
/// over `%fs`, so there is no library symbol; the port still declares them `unsafe extern "C"`
/// (P-88 ①) and **every prototype is UNVERIFIED** - the widths below follow the names' own
/// byte/word suffixes, not a header this machine has not got.
/// C: void _farsetsel (unsigned short); unsigned char _farpeekb (unsigned short, unsigned long);
/// C: unsigned short _farpeekw (unsigned short, unsigned long); unsigned short _farnspeekw (unsigned long);
/// C: void _farnspokeb (unsigned long, unsigned char); void _farnspokew (unsigned long, unsigned short);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn _farsetsel (selector: c_ushort);
  pub fn _farpeekb (selector: c_ushort, offset: c_ulong) -> c_uchar;
  pub fn _farpeekw (selector: c_ushort, offset: c_ulong) -> c_ushort;
  pub fn _farnspeekw (offset: c_ulong) -> c_ushort;
  pub fn _farnspokeb (offset: c_ulong, value: c_uchar);
  pub fn _farnspokew (offset: c_ulong, value: c_ushort);
}
/* C 59's `<libc/dosio.h>` (`/* for _USE_LFN */`) and C 60's `<conio.h>` (`/* for cputs */`): the
   only name this slice takes from either is the `_USE_LFN` *test* at C 95, which is carried at
   C 95 below.  */

#[cfg (feature = "msdos")]
include!("../lib/include/errno.rs");           // C 48: `#include <errno.h>`
#[cfg (feature = "msdos")]
include!("../lib/include/sys_stat.in.rs");     // C 49: `#include <sys/stat.h>`
#[cfg (feature = "msdos")]
include!("../lib/include/unistd.in.rs");       // C 50: `#include <unistd.h>`
/* C 50's `<unistd.h>` is gnulib's, and `lib/unistd.in.h` 122-125 includes `<stdlib.h>` for
   every non-glibc target; this unit's `getenv` (C 1780/1808/1834) is that header's.  The
   mirror does not carry the edge, so the unit spells it here (the C 36 `<stdcountof.h>`
   shape - Deviations 13).  */
#[cfg (feature = "msdos")]
include!("../lib/include/stdlib.rs");          // unistd.in.h 122-125: `#include <stdlib.h>` (`getenv`)
/* C 51's `<dir.h>` (`getdisk`), C 52's `#pragma pack(0)` and C 53's `#undef opendir` (C 56's
   `<io.h>`, `setmode`) belong to names past this slice: the pragma has no Rust counterpart and
   the `#undef` has none either (the port never spells `opendir` as `<dirent.h>`'s macro).  Two
   `## Deviations` entries.  */
#[cfg (feature = "msdos")]
include!("../lib/include/dirent.in.rs");       // C 54: `#include <dirent.h>`
#[cfg (feature = "msdos")]
include!("../lib/include/fcntl.in.rs");        // C 55: `#include <fcntl.h>`

/* C 62-66: `#if (__DJGPP__ + (__DJGPP_MINOR__ > 3)) >= 3` - `SYS_ENVIRON` (the `environ` the
   environment setup at the end of the file reads).  The reference's two arms pick `_environ` for
   djgpp 2.4 and later and `environ` for 2.3 and older; the port spells them with `djgpp_2_3`
   (the name `include/msdos.rs` 26/135 uses for the 2.3-and-older side) and its negation
   (`__DJGPP__ + (__DJGPP_MINOR__ > 3) >= 3` is exactly `__DJGPP__ == 2 && __DJGPP_MINOR__ < 4`'s
   complement for the djgpp 2 line).  Both names are *variable* macros, so they are nullary
   `macro_rules!` (PORTING.md 2.1, the `include/conf_post.rs` 119 `strtold` shape); neither
   expansion is reached in this slice.  */
/// C: #define SYS_ENVIRON _environ
#[cfg (feature = "msdos")]
#[cfg (not (feature = "djgpp_2_3"))]
macro_rules! SYS_ENVIRON { () => { _environ } }
/// C: #define SYS_ENVIRON environ
#[cfg (feature = "msdos")]
#[cfg (feature = "djgpp_2_3")]
macro_rules! SYS_ENVIRON { () => { environ } }

#[cfg (feature = "msdos")]
include!("include/msdos.rs");                  // C 68: `#include "msdos.h"`
#[cfg (feature = "msdos")]
include!("include/systime.rs");                // C 69: `#include "systime.h"` (`gettime`, `timespec`)
#[cfg (feature = "msdos")]
include!("include/frame.rs");                  // C 70: `#include "frame.h"`
/* `struct face`, `Mouse_HLInfo`'s accessors and `FACE_FROM_ID_OR_NULL` live in the mirror's
   `frame_face_accessors` module, which `include/frame.rs` leaves behind
   `feature = "dispextern_face_records"` (its own ruling, dispextern.rs 2694's neighbourhood) and
   does not glob-import; the arm flip therefore names that feature too, and this unit takes the
   module the way the mirror's help says (see the ledger's Owed 4).  */
#[cfg (all (feature = "msdos", feature = "dispextern_face_records"))]
use frame_face_accessors::*;
/* `include/dispextern.rs` 2694's `dispextern_unit_names` is likewise never glob-imported by the
   mirror (`include/syssignal.rs` 52's `use syssignal_decls::*` is the shape it lacks), so the
   names this slice takes from it (`FACE_TTY_DEFAULT_*` at C 803-809, `reset_mouse_highlight` at
   C 1276/1282) are reachable only through this edge.  The ledger's Owed 5 asks the mirror round
   to decide between the mirror's own glob and this per-consumer edge.  */
#[cfg (feature = "msdos")]
use dispextern_unit_names::*;
#[cfg (feature = "msdos")]
include!("include/termhooks.rs");              // C 71: `#include "termhooks.h"`
#[cfg (feature = "msdos")]
include!("include/termchar.rs");               // C 72: `#include "termchar.h"`
#[cfg (feature = "msdos")]
include!("include/dispextern.rs");             // C 73: `#include "dispextern.h"`
#[cfg (feature = "msdos")]
include!("include/dosfns.rs");                 // C 74: `#include "dosfns.h"`
#[cfg (feature = "msdos")]
include!("include/termopts.rs");               // C 75: `#include "termopts.h"`
#[cfg (feature = "msdos")]
include!("include/character.rs");              // C 76: `#include "character.h"`
#[cfg (feature = "msdos")]
include!("include/coding.rs");                 // C 77: `#include "coding.h"`
#[cfg (feature = "msdos")]
include!("include/disptab.rs");                // C 78: `#include "disptab.h"`
#[cfg (feature = "msdos")]
include!("include/window.rs");                 // C 79: `#include "window.h"`
#[cfg (feature = "msdos")]
include!("include/menu.rs");                   // C 80: `#include "menu.h"`
#[cfg (feature = "msdos")]
include!("include/buffer.rs");                 // C 81: `#include "buffer.h"`
#[cfg (feature = "msdos")]
include!("include/commands.rs");               // C 82: `#include "commands.h"`
#[cfg (feature = "msdos")]
include!("include/blockinput.rs");             // C 83: `#include "blockinput.h"`
#[cfg (feature = "msdos")]
include!("include/keyboard.rs");               // C 84: `#include "keyboard.h"`
#[cfg (feature = "msdos")]
include!("include/intervals.rs");              // C 85: `#include "intervals.h"`

/* ----------------------------------------------------------------------------------------------
   C 86/87/88's `<go32.h>`, `<pc.h>` and `<ctype.h>`.  `<ctype.h>`'s mirror is
   `lib/include/ctype.rs`; the two DJGPP headers have none (P-88 ①), so the names this slice takes
   from them are declared here and **every one is UNVERIFIED** - no DJGPP toolchain (see the file
   note), and the reference's own comments name neither header, so even *which* of the two
   supplies `ScreenPrimary`/`_go32_info_block` is unmeasured.  ----------------------------------- */
#[cfg (feature = "msdos")]
include!("../lib/include/ctype.rs");           // C 88: `#include <ctype.h>`
// C 86: `#include <go32.h>` - the header this section stands for.
/// C 86's `<go32.h>`: the DPMI info block `_dos_ds` (C 100) reads.  **UNVERIFIED** (no toolchain);
/// only the member C 100 names is carried.
/// C: struct _go32_info_block { int selector_for_linear_memory; … }; extern struct _go32_info_block _go32_info_block;
#[cfg (feature = "msdos")]
#[repr(C)]
pub struct _go32_info_block
{
  pub selector_for_linear_memory: c_int,
}
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub static mut _go32_info_block: _go32_info_block;
}
// C 87: `#include <pc.h>` - the header this section stands for.
/// C 87's `<pc.h>`: the text-mode screen surface.  **UNVERIFIED**: the same missing toolchain; the
/// widths are the names' own usage (`(int) ScreenPrimary` at C 468, `ScreenAttrib`'s byte stores
/// at C 853/862), not a reading.
/// C: extern unsigned char ScreenAttrib; extern unsigned long ScreenPrimary;
/// C: int ScreenRows (void); int ScreenCols (void); void ScreenGetCursor (int *, int *);
/// C: void ScreenSetCursor (int, int); void ScreenClear (void); void ScreenVisualBell (void);
/// C: void _set_screen_lines (int); int dosmemput (const void *, int, int);
/// C: int ScreenMode (void); void ScreenRetrieve (void *);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub static mut ScreenPrimary: c_ulong;
  pub fn ScreenRows () -> c_int;
  pub fn ScreenCols () -> c_int;
  pub fn ScreenGetCursor (row: *mut c_int, col: *mut c_int);
  pub fn ScreenSetCursor (row: c_int, col: c_int);
  pub fn ScreenClear ();
  pub fn ScreenVisualBell ();
  pub fn _set_screen_lines (lines: c_int);
  pub fn dosmemput (buffer: *const c_void, length: c_int, offset: c_int) -> c_int;
  pub fn ScreenMode () -> c_int;
  pub fn ScreenRetrieve (memory: *mut c_void);
}

/* C 89-93: the `spawnve` the local `<process.h>` would have supplied (which C 90-91's comment
   refuses).  */
/// C: #define P_WAIT 1
#[cfg (feature = "msdos")]
pub const P_WAIT: c_int = 1;
/// C: extern int spawnve (int, const char *, char *const [], char *const []);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn spawnve (mode: c_int, path: *const c_char, argv: *const *mut c_char,
                  envp: *const *mut c_char) -> c_int;
}

/* C 95-97: `#ifndef _USE_LFN` - `<libc/dosio.h>` normally defines it; the reference supplies 0
   as its fallback.  The `#else` side is that header's own definition, which the port does not
   repeat (the `include/msdos.rs` 84-87 `O_CLOEXEC` shape), so the arm carries the feature.  */
/// C: #define _USE_LFN 0
#[cfg (feature = "msdos")]
#[cfg (not (feature = "_use_lfn"))]
pub const _USE_LFN: c_int = 0;

/* C 99-101: `#ifndef _dos_ds` - `<go32.h>` defines it (see the section above); the reference
   supplies the fallback expression as its `#else` side.  A *place* macro in C (`_dos_ds` is read
   as a selector), so the port carries it as a nullary `macro_rules!`.  */
/// C: #define _dos_ds _go32_info_block.selector_for_linear_memory
#[cfg (feature = "msdos")]
#[cfg (not (feature = "_dos_ds"))]
macro_rules! _dos_ds { () => { _go32_info_block.selector_for_linear_memory } }

#[cfg (feature = "msdos")]
include!("../lib/include/signal.rs");          // C 103: `#include <signal.h>`
#[cfg (feature = "msdos")]
include!("include/syssignal.rs");              // C 104: `#include "syssignal.h"`
#[cfg (feature = "msdos")]
include!("../lib/include/careadlinkat.rs");    // C 106: `#include "careadlinkat.h"`
#[cfg (feature = "msdos")]
include!("../lib/include/allocator.rs");       // C 107: `#include "allocator.h"`

/* C 109-126: the allocator triple.  C 109 `#ifndef SYSTEM_MALLOC` (feature `no_system_malloc`,
   the registered `!SYSTEM_MALLOC`, Cargo.toml 438), C 111 `#ifdef GNU_MALLOC` (feature
   `gnu_malloc`, *not* registered - Owed) and C 119's `#ifdef REL_ALLOC` / `#else` pair (feature
   `rel_alloc`, Cargo.toml 490).  Neither C 109 nor C 111 has an `#else` arm in C, so each arm
   below is the whole of its condition; the other side of C 119 is `_CRT0_FLAG_UNIX_SBRK` alone.
   `_crt0_startup_flags` is a *definition* (a non-`static` object), so it carries
   `#[unsafe(no_mangle)]`.  */

// C 117: `#include <crt0.h>` - the header this section stands for.
/// C 117's `<crt0.h>`: the two startup-flag bits C 120/122 combine.  **UNVERIFIED** (no DJGPP
/// toolchain): the values below are placeholders, *not* readings, and the ledger owes them.
/// C: #define _CRT0_FLAG_UNIX_SBRK …; #define _CRT0_FLAG_FILL_SBRK_MEMORY …
#[cfg (feature = "msdos")]
#[cfg (feature = "no_system_malloc")]
#[cfg (feature = "gnu_malloc")]
pub const _CRT0_FLAG_UNIX_SBRK: c_int = 0;
#[cfg (feature = "msdos")]
#[cfg (feature = "no_system_malloc")]
#[cfg (feature = "gnu_malloc")]
pub const _CRT0_FLAG_FILL_SBRK_MEMORY: c_int = 0;

#[cfg (feature = "msdos")]
#[cfg (feature = "no_system_malloc")]
#[cfg (feature = "gnu_malloc")]
#[cfg (feature = "rel_alloc")]
#[unsafe(no_mangle)]
pub static mut _crt0_startup_flags: c_int = _CRT0_FLAG_UNIX_SBRK;
#[cfg (feature = "msdos")]
#[cfg (feature = "no_system_malloc")]
#[cfg (feature = "gnu_malloc")]
#[cfg (not (feature = "rel_alloc"))]
#[unsafe(no_mangle)]
pub static mut _crt0_startup_flags: c_int =
  (_CRT0_FLAG_UNIX_SBRK | _CRT0_FLAG_FILL_SBRK_MEMORY);

/* C's `f->mouse_moved` (`frame.h 434`, a `bool_bf`): the reader and writer the `struct frame`
   flag run needs.  `include/frame.rs` carries the shift and unit constants but no accessor, and
   the tree defines this per consuming unit (`src/window.rs` 152's `set_frame_flag1!`, the
   `src/androidterm.rs` 235 `mouse_moved!` shape); this unit's copy carries the file gate.
   One `## Deviations` entry.  */
#[cfg (feature = "msdos")]
macro_rules! set_frame_flag1 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}
#[cfg (feature = "msdos")]
macro_rules! mouse_moved {
  ($f:expr) => { (*$f).bits1 >> FRAME_MOUSE_MOVED_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_MOUSE_MOVED_SHIFT, $v) };
}

/* Return the current timestamp in milliseconds since midnight.  */
#[cfg (feature = "msdos")]
unsafe fn event_timestamp () -> c_ulong            // C 129-130
{
  let mut t: timespec = std::mem::zeroed ();
  let mut s: c_ulong;

  gettime (&raw mut t);
  s = t.tv_sec as c_ulong;
  s %= 86400;
  s *= 1000;
  s += (t.tv_nsec * 1000000) as c_ulong;

  return s;
}

/* ------------------------ Mouse control ---------------------------
 *
 * Coordinates are in screen positions and zero based.
 * Mouse buttons are numbered from left to right and also zero based.
 */

/* This used to be in termhooks.h, but mainstream Emacs code no longer
   uses it, and it was removed...  */
/// C 153: `#define NUM_MOUSE_BUTTONS (5)`
#[cfg (feature = "msdos")]
pub const NUM_MOUSE_BUTTONS: c_int = (5);

/// C 155: `int have_mouse;` (`0: no, 1: enabled, -1: disabled`)
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub static mut have_mouse: c_int = 0;
#[cfg (feature = "msdos")]
static mut mouse_visible: c_int = 0;

#[cfg (feature = "msdos")]
static mut mouse_last_x: c_int = 0;
#[cfg (feature = "msdos")]
static mut mouse_last_y: c_int = 0;

#[cfg (feature = "msdos")]
static mut mouse_button_translate: [c_int; NUM_MOUSE_BUTTONS as usize] =
  [0; NUM_MOUSE_BUTTONS as usize];
#[cfg (feature = "msdos")]
static mut mouse_button_count: c_int = 0;

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn mouse_on ()                                 // C 164-165
{
  let mut regs: REGS = std::mem::zeroed ();

  if have_mouse > 0 && mouse_visible == 0
    {
      let tty = CURTTY! ();

      if !(*tty).termscript.is_null ()
	{ fprintf ((*tty).termscript, c"<M_ON>".as_ptr ()); }
      regs.x.ax = 0x0001;
      int86 (0x33, &raw mut regs, &raw mut regs);
      mouse_visible = 1;
    }
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn mouse_off ()                                // C 181-182
{
  let mut regs: REGS = std::mem::zeroed ();

  if have_mouse > 0 && mouse_visible != 0
    {
      let tty = CURTTY! ();

      if !(*tty).termscript.is_null ()
	{ fprintf ((*tty).termscript, c"<M_OFF>".as_ptr ()); }
      regs.x.ax = 0x0002;
      int86 (0x33, &raw mut regs, &raw mut regs);
      mouse_visible = 0;
    }
}

#[cfg (feature = "msdos")]
unsafe fn mouse_setup_buttons (n_buttons: c_int)   // C 198-199
{
  if n_buttons == 3
    {
      mouse_button_count = 3;
      mouse_button_translate[0] = 0; /* Left */
      mouse_button_translate[1] = 2; /* Middle */
      mouse_button_translate[2] = 1; /* Right */
    }
  else	/* two, what else? */
    {
      mouse_button_count = 2;
      mouse_button_translate[0] = 0;
      mouse_button_translate[1] = 1;
    }
}

#[cfg (feature = "msdos")]
DEFUN! ("msdos-set-mouse-buttons", Fmsdos_set_mouse_buttons, Smsdos_set_mouse_buttons,
       1, 1, "NSet number of mouse buttons to: ",
       doc: "Set the number of mouse buttons to use by Emacs.\nThis is useful with mice that report the number of buttons inconsistently,\ne.g., if the number of buttons is reported as 3, but Emacs only sees 2 of\nthem.  This happens with wheeled mice on Windows 9X, for example.  ",   // C 216-222
       (nbuttons: Lisp_Object),
       {
  let mut n: c_int;

  CHECK_FIXNUM (nbuttons);
  n = XFIXNUM (nbuttons) as c_int;
  if n < 2 || n > 3
    {
      xsignal2 (Qargs_out_of_range (),
		build_string (c"only 2 or 3 mouse buttons are supported".as_ptr ()),
		nbuttons);
    }
  mouse_setup_buttons (n);
  return Qnil ();
});

#[cfg (feature = "msdos")]
unsafe fn mouse_get_xy (x: *mut c_int, y: *mut c_int)   // C 236-237
{
  let mut regs: REGS = std::mem::zeroed ();
  let f = SELECTED_FRAME ();

  regs.x.ax = 0x0003;
  int86 (0x33, &raw mut regs, &raw mut regs);
  *x = (regs.x.cx / 8) as c_int - (*f).left_pos;
  *y = (regs.x.dx / 8) as c_int - (*f).top_pos;
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn mouse_moveto (x: c_int, y: c_int)           // C 248-249
{
  let mut regs: REGS = std::mem::zeroed ();
  let tty = CURTTY! ();
  let f = SELECTED_FRAME ();

  if !(*tty).termscript.is_null ()
    { fprintf ((*tty).termscript, c"<M_XY=%dx%d>".as_ptr (), x, y); }
  regs.x.ax = 0x0004;
  regs.x.cx = (x * 8) as c_ushort;
  mouse_last_x = regs.x.cx as c_int;	/* C's `mouse_last_x = regs.x.cx = x * 8` */
  regs.x.dx = (y * 8) as c_ushort;
  mouse_last_y = regs.x.dx as c_int;	/* C's `mouse_last_y = regs.x.dx = y * 8` */
  regs.x.cx = (regs.x.cx as c_int + (*f).left_pos * 8) as c_ushort;
  regs.x.dx = (regs.x.dx as c_int + (*f).top_pos * 8) as c_ushort;
  int86 (0x33, &raw mut regs, &raw mut regs);
}

#[cfg (feature = "msdos")]
unsafe fn mouse_pressed (b: c_int, xp: *mut c_int, yp: *mut c_int) -> c_int   // C 265-266
{
  let mut regs: REGS = std::mem::zeroed ();
  let f = SELECTED_FRAME ();

  if b >= mouse_button_count
    { return 0; }
  regs.x.ax = 0x0005;
  regs.x.bx = mouse_button_translate[b as usize] as c_ushort;
  int86 (0x33, &raw mut regs, &raw mut regs);
  if regs.x.bx != 0
    {
      *xp = (regs.x.cx / 8) as c_int - (*f).left_pos;
      *yp = (regs.x.dx / 8) as c_int - (*f).top_pos;
    }
  return (regs.x.bx != 0) as c_int;
}

#[cfg (feature = "msdos")]
unsafe fn mouse_released (b: c_int, xp: *mut c_int, yp: *mut c_int) -> c_int  // C 284-285
{
  let mut regs: REGS = std::mem::zeroed ();

  if b >= mouse_button_count
    { return 0; }
  regs.x.ax = 0x0006;
  regs.x.bx = mouse_button_translate[b as usize] as c_ushort;
  int86 (0x33, &raw mut regs, &raw mut regs);
  if regs.x.bx != 0
    {
      *xp = (regs.x.cx / 8) as c_int;
      *yp = (regs.x.dx / 8) as c_int;
    }
  return (regs.x.bx != 0) as c_int;
}

#[cfg (feature = "msdos")]
unsafe fn mouse_button_depressed (b: c_int, xp: *mut c_int, yp: *mut c_int)
    -> c_int                                              // C 299-300
{
  let mut regs: REGS = std::mem::zeroed ();

  if b >= mouse_button_count
    { return 0; }
  regs.x.ax = 0x0003;
  int86 (0x33, &raw mut regs, &raw mut regs);
  if (regs.x.bx & (1 << mouse_button_translate[b as usize])) != 0
    {
      *xp = (regs.x.cx / 8) as c_int;
      *yp = (regs.x.dx / 8) as c_int;
      return 1;
    }
  return 0;
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn mouse_get_pos (f: *mut *mut frame, insist: c_int, bar_window: *mut Lisp_Object,
			     part: *mut scroll_bar_part, x: *mut Lisp_Object,
			     y: *mut Lisp_Object, time: *mut Time)   // C 317-318
{
  let mut ix: c_int = 0;
  let mut iy: c_int = 0;
  let mut frame: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object = Qnil ();

  /* Clear the mouse-moved flag for every frame on this display.  */
  FOR_EACH_FRAME! (tail, frame, {
    mouse_moved! (XFRAME (frame), 0);
  });

  *f = SELECTED_FRAME ();
  *bar_window = Qnil ();
  mouse_get_xy (&raw mut ix, &raw mut iy);
  *time = event_timestamp ();
  mouse_last_x = ix;			/* C's `make_fixnum (mouse_last_x = ix)`, hoisted */
  *x = make_fixnum (mouse_last_x as EMACS_INT);
  mouse_last_y = iy;			/* C's `make_fixnum (mouse_last_y = iy)`, hoisted */
  *y = make_fixnum (mouse_last_y as EMACS_INT);
}

#[cfg (feature = "msdos")]
unsafe fn mouse_check_moved ()                     // C 337-338
{
  let mut x: c_int = 0;
  let mut y: c_int = 0;

  mouse_get_xy (&raw mut x, &raw mut y);
  mouse_moved! (SELECTED_FRAME (),
		(mouse_moved! (SELECTED_FRAME ()) as c_int)
		| (((x != mouse_last_x) || (y != mouse_last_y)) as c_int));
  mouse_last_x = x;
  mouse_last_y = y;
}

/* Force the mouse driver to ``forget'' about any button clicks until
   now.  */
#[cfg (feature = "msdos")]
unsafe fn mouse_clear_clicks ()                    // C 350-351
{
  let mut b: c_int;

  b = 0;
  while b < mouse_button_count
    {
      let mut dummy_x: c_int = 0;
      let mut dummy_y: c_int = 0;

      mouse_pressed (b, &raw mut dummy_x, &raw mut dummy_y);
      mouse_released (b, &raw mut dummy_x, &raw mut dummy_y);
      b += 1;
    }
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn mouse_init ()                               // C 364-365
{
  let mut regs: REGS = std::mem::zeroed ();
  let tty = CURTTY! ();

  if !(*tty).termscript.is_null ()
    { fprintf ((*tty).termscript, c"<M_INIT>".as_ptr ()); }

  regs.x.ax = 0x0021;
  int86 (0x33, &raw mut regs, &raw mut regs);

  /* Reset the mouse last press/release info.  It seems that Windows
     doesn't do that automatically when function 21h is called, which
     causes Emacs to ``remember'' the click that switched focus to the
     window just before Emacs was started from that window.  */
  mouse_clear_clicks ();

  regs.x.ax = 0x0007;
  regs.x.cx = 0;
  regs.x.dx = (8 * (ScreenCols () - 1)) as c_ushort;
  int86 (0x33, &raw mut regs, &raw mut regs);

  regs.x.ax = 0x0008;
  regs.x.cx = 0;
  regs.x.dx = (8 * (ScreenRows () - 1)) as c_ushort;
  int86 (0x33, &raw mut regs, &raw mut regs);

  mouse_moveto (0, 0);
  mouse_visible = 0;
}

/* ------------------------- Screen control ----------------------
 *
 */

#[cfg (feature = "msdos")]
static mut internal_terminal: c_int = 0;

/* C 402-478 and C 480-1771 are two separate `#ifndef HAVE_X_WINDOWS` blocks (C 478's `#endif`
   closes the first); everything until C 1771 is the MS-DOS screen machinery and carries
   `not (feature = "have_x_windows")` on top of the file gate.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe extern "C" {
  /// C 403: `extern unsigned char ScreenAttrib;` - C's own re-declaration of `<pc.h>`'s object
  /// (see the C 87 section above, which deliberately does not declare it too).
  pub static mut ScreenAttrib: c_uchar;
}
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut screen_face: c_int = 0;

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut screen_size_X: c_int = 0;
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut screen_size_Y: c_int = 0;
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut screen_size: c_int = 0;

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut current_pos_X: c_int = 0;
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut current_pos_Y: c_int = 0;
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut new_pos_X: c_int = 0;
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut new_pos_Y: c_int = 0;

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut startup_screen_buffer: *mut c_void = core::ptr::null_mut ();
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut startup_screen_size_X: c_int = 0;
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut startup_screen_size_Y: c_int = 0;
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut startup_pos_X: c_int = 0;
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut startup_pos_Y: c_int = 0;
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut startup_screen_attrib: c_uchar = 0;

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut startup_time: clock_t = 0;

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut term_setup_done: c_int = 0;

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut outside_cursor: c_ushort = 0;

/* The only display since MS-DOS does not support multiple ones.  */
/// C 429: `struct tty_display_info the_only_display_info;` (`include/msdos.rs` declares the same
/// object for the other units; see the ledger's Owed list).
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub static mut the_only_display_info: tty_display_info = unsafe { std::mem::zeroed () };

/* The only tty_output, since MS-DOS supports only 1 display.  */
/// C 432: `struct tty_output the_only_tty_output;` (same mirror note as C 429).
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub static mut the_only_tty_output: tty_output = unsafe { std::mem::zeroed () };

/* Support for DOS/V (allows Japanese characters to be displayed on
   standard, non-Japanese, ATs).  Only supported for DJGPP v2 and later.  */

/* Holds the address of the text-mode screen buffer.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut screen_old_address: c_ulong = 0;
/* Segment and offset of the virtual screen.  If 0, DOS/V is NOT loaded.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut screen_virtual_segment: c_ushort = 0;
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut screen_virtual_offset: c_ushort = 0;

/* The screen colors of the current frame, which serve as the default
   colors for newly-created frames.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut initial_screen_colors: [c_int; 2] = [0; 2];

/* Update the screen from a part of relocated DOS/V screen buffer which
   begins at OFFSET and includes COUNT characters.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn dosv_refresh_virtual_screen (offset: c_int, count: c_int)   // C 449-450
{
  let mut regs: __dpmi_regs = std::mem::zeroed ();

  if offset < 0 || count < 0	/* paranoia; invalid values crash DOS/V */
    { return; }

  regs.h.ah = 0xff;	/* update relocated screen */
  regs.x.es = screen_virtual_segment;
  regs.x.di = (screen_virtual_offset as c_int + offset) as c_ushort;
  regs.x.cx = count as c_ushort;
  __dpmi_int (0x10, &raw mut regs);
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn dos_direct_output (y: c_int, x: c_int, mut buf: *mut c_char, mut len: c_int)  // C 464-465
{
  let t0: c_int = 2 * (x + y * screen_size_X);
  let mut t: c_int = t0 + ScreenPrimary as c_int;
  let l0: c_int = len;

  /* This is faster.  */
  _farsetsel (_dos_ds! () as c_ushort);
  while { len -= 1; len >= 0 }
    {
      _farnspokeb (t as c_ulong, *buf as c_uchar);
      t += 2;
      buf = buf.offset (1);
    }

  if screen_virtual_segment != 0
    { dosv_refresh_virtual_screen (t0, l0); }
}
/* C 478: `#endif` - the C 402 block ends here.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut blink_bit: c_int = -1;	/* the state of the blink bit at startup */

/* Enable bright background colors.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn bright_bg ()                             // C 485-486
{
  let mut regs: REGS = std::mem::zeroed ();

  /* Remember the original state of the blink/bright-background bit.
     It is stored at 0040:0065h in the BIOS data area.  */
  if blink_bit == -1
    {
      blink_bit = ((_farpeekb (_dos_ds! () as c_ushort, 0x465) & 0x20) == 0x20) as c_int;
    }

  regs.h.bl = 0;
  regs.x.ax = 0x1003;
  int86 (0x10, &raw mut regs, &raw mut regs);
}

/* Disable bright background colors (and enable blinking) if we found
   the video system in that state at startup.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn maybe_enable_blinking ()                 // C 502-503
{
  if blink_bit == 1
    {
      let mut regs: REGS = std::mem::zeroed ();

      regs.h.bl = 1;
      regs.x.ax = 0x1003;
      int86 (0x10, &raw mut regs, &raw mut regs);
    }
}

/* Return non-zero if the system has a VGA adapter.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn vga_installed () -> c_int                // C 516-517
{
  let mut regs: REGS = std::mem::zeroed ();

  regs.x.ax = 0x1a00;
  int86 (0x10, &raw mut regs, &raw mut regs);
  if regs.h.al == 0x1a && regs.h.bl > 5 && regs.h.bl < 13
    { return 1; }
  return 0;
}

/* Set the screen dimensions so that it can show no less than
   ROWS x COLS frame.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub unsafe fn dos_set_window_size (rows: *mut c_int, cols: *mut c_int)   // C 531-532
{
  let mut regs: REGS = std::mem::zeroed ();
  let mut video_mode: Lisp_Object;
  let mut video_mode_value: c_int = 0;
  let mut have_vga: c_int = 0;
  let current_rows: c_int = ScreenRows ();
  let current_cols: c_int = ScreenCols ();

  if *rows == current_rows && *cols == current_cols
    { return; }

  mouse_off ();
  have_vga = vga_installed ();

  /* If the user specified a special video mode for these dimensions,
     use that mode.  */
  video_mode
    = Fsymbol_value (Fintern_soft (make_formatted_string
				   (c"screen-dimensions-%dx%d".as_ptr (),
				    *rows, *cols), Qnil ()));

  if FIXNUMP (video_mode)
    {
      video_mode_value = XFIXNUM (video_mode) as c_int;
      if video_mode_value > 0
	{
	  regs.x.ax = video_mode_value as c_ushort;
	  int86 (0x10, &raw mut regs, &raw mut regs);

	  if have_mouse != 0
	    {
	      /* Must hardware-reset the mouse, or else it won't update
		 its notion of screen dimensions for some non-standard
		 video modes.  This is *painfully* slow...  */
	      regs.x.ax = 0;
	      int86 (0x33, &raw mut regs, &raw mut regs);
	    }
	}
    }

  /* Find one of the dimensions supported by standard EGA/VGA
     which gives us at least the required dimensions.  */
  else
    {
      #[repr(C)]
      struct StdDimension                    /* C's anonymous struct */
      {
	rows: c_int, need_vga: c_int,
      }
      static std_dimension: [StdDimension; 6] = [
	  StdDimension { rows: 25, need_vga: 0 },
	  StdDimension { rows: 28, need_vga: 1 },
	  StdDimension { rows: 35, need_vga: 0 },
	  StdDimension { rows: 40, need_vga: 1 },
	  StdDimension { rows: 43, need_vga: 0 },
	  StdDimension { rows: 50, need_vga: 1 },
      ];
      let mut i: c_int = 0;

      while i < countof! (std_dimension) as c_int	/* C's int-vs-size_t comparison */
	{
	 if std_dimension[i as usize].need_vga <= have_vga
	     && std_dimension[i as usize].rows >= *rows
	   {
	     if std_dimension[i as usize].rows != current_rows
		 || *cols != current_cols
	       { _set_screen_lines (std_dimension[i as usize].rows); }
	     break;
	   }
	 i += 1;
	}
    }


  if have_mouse != 0
    {
      mouse_init ();
      mouse_on ();
    }

  /* Tell the caller what dimensions have been REALLY set.  */
  *rows = ScreenRows ();
  *cols = ScreenCols ();

  /* Update Emacs's notion of screen dimensions.  */
  screen_size_X = *cols;
  screen_size_Y = *rows;
  screen_size = *cols * *rows;

  /* If the dimensions changed, the mouse highlight info is invalid.  */
  if current_rows != *rows || current_cols != *cols
    {
      let f = SELECTED_FRAME ();
      let hlinfo = MOUSE_HL_INFO! (f);
      let window: Lisp_Object = (*hlinfo).mouse_face_window;

      if !NILP (window) && XFRAME ((*XWINDOW (window)).frame) == f
	{ reset_mouse_highlight (hlinfo); }
    }

  /* Enable bright background colors.  */
  bright_bg ();

  /* FIXME: I'm not sure the above will run at all on DOS/V.  But let's
     be defensive anyway.  */
  if screen_virtual_segment != 0
    { dosv_refresh_virtual_screen (0, *cols * *rows); }
}

/* If we write a character in the position where the mouse is,
   the mouse cursor may need to be refreshed.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn mouse_off_maybe ()                       // C 637-638
{
  let mut x: c_int = 0;
  let mut y: c_int = 0;

  if mouse_visible == 0
    { return; }

  mouse_get_xy (&raw mut x, &raw mut y);
  if y != new_pos_Y || x < new_pos_X
    { return; }

  mouse_off ();
}

/// C 652-654: the three cursor-shape constants.
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
pub const DEFAULT_CURSOR_START: c_int = (-1);
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
pub const DEFAULT_CURSOR_WIDTH: c_int = (-1);
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
pub const BOX_CURSOR_WIDTH: c_int = (-32);

/* Set cursor to begin at scan line START_LINE in the character cell
   and extend for WIDTH scan lines.  Scan lines are counted from top
   of the character cell, starting from zero.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn msdos_set_cursor_shape (f: *mut frame, start_line: c_int, width: c_int)  // C 659-660
{
  let mut desired_cursor: c_uint;
  let mut regs: __dpmi_regs = std::mem::zeroed ();
  let mut max_line: c_int;
  let mut top_line: c_int = 0;
  let mut bot_line: c_int = 0;
  let tty = FRAME_TTY! (f);

  /* Avoid the costly BIOS call if F isn't the currently selected
     frame.  Allow for NULL as unconditionally meaning the selected
     frame.  */
  if !f.is_null () && f != SELECTED_FRAME ()
    { return; }

  if !(*tty).termscript.is_null ()
    { fprintf ((*tty).termscript, c"\nCURSOR SHAPE=(%d,%d)".as_ptr (), start_line, width); }

  /* The character cell size in scan lines is stored at 40:85 in the
     BIOS data area.  */
  max_line = _farpeekw (_dos_ds! () as c_ushort, 0x485) as c_int - 1;
  match max_line
    {
      /* this relies on CGA cursor emulation being ON! */
      7 => { bot_line = 7; }
      9 => { bot_line = 9; }
      13 => { bot_line = 12; }
      15 => { bot_line = 14; }
      _ => { bot_line = 7; }		/* C's `default: … case 7:` (fallthrough) */
    }

  if width < 0
    {
      if width == BOX_CURSOR_WIDTH
	{
	  top_line = 0;
	  bot_line = max_line;
	}
      else if start_line != DEFAULT_CURSOR_START
	{
	  top_line = start_line;
	  bot_line = top_line - width - 1;
	}
      else if width != DEFAULT_CURSOR_WIDTH
	{
	  top_line = 0;
	  bot_line = -1 - width;
	}
      else
	{ top_line = bot_line + 1; }
    }
  else if width == 0
    {
      /* [31, 0] seems to DTRT for all screen sizes.  */
      top_line = 31;
      bot_line = 0;
    }
  else	/* WIDTH is positive */
    {
      if start_line != DEFAULT_CURSOR_START
	{ bot_line = start_line; }
      top_line = bot_line - (width - 1);
    }

  /* If the current cursor shape is already what they want, we are
     history here.  */
  desired_cursor = (((top_line & 0x1f) << 8) | (bot_line & 0x1f)) as c_uint;
  if (desired_cursor as c_int) == _farpeekw (_dos_ds! () as c_ushort, 0x460) as c_int
    { return; }

  regs.h.ah = 1;
  regs.x.cx = desired_cursor as c_ushort;
  __dpmi_int (0x10, &raw mut regs);
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_set_cursor_type (f: *mut frame, cursor_type: Lisp_Object)   // C 740-741
{
  if EQ (cursor_type, Qbar ()) || EQ (cursor_type, Qhbar ())
    {
      /* Just BAR means the normal EGA/VGA cursor.  */
      msdos_set_cursor_shape (f, DEFAULT_CURSOR_START, DEFAULT_CURSOR_WIDTH);
    }
  else if CONSP (cursor_type)
	   && (EQ (XCAR (cursor_type), Qbar ())
	       || EQ (XCAR (cursor_type), Qhbar ()))
    {
      let bar_parms: Lisp_Object = XCDR (cursor_type);

      if FIXNUMP (bar_parms)
	{
	  /* Feature: negative WIDTH means cursor at the top
	     of the character cell, zero means invisible cursor.  */
	  let width: c_int = XFIXNUM (bar_parms) as c_int;
	  msdos_set_cursor_shape (f,
				  if width >= 0 { DEFAULT_CURSOR_START } else { 0 },
				  width);
	}
      else if CONSP (bar_parms)
	       && FIXNUMP (XCAR (bar_parms))
	       && FIXNUMP (XCDR (bar_parms))
	{
	  let start_line: c_int = XFIXNUM (XCDR (bar_parms)) as c_int;

	  let width: c_int = XFIXNUM (XCAR (bar_parms)) as c_int;
	  msdos_set_cursor_shape (f, start_line, width);
	}
    }
  else
    {
      /* Treat anything unknown as "box cursor".  This includes nil, so
	 that a frame which doesn't specify a cursor type gets a box,
	 which is the default in Emacs.  */
      msdos_set_cursor_shape (f, 0, BOX_CURSOR_WIDTH);
    }
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_ring_bell (f: *mut frame)             // C 782-783
{
  if visible_bell
    {
      mouse_off ();
      ScreenVisualBell ();
    }
  else
    {
      let mut inregs: REGS = std::mem::zeroed ();
      let mut outregs: REGS = std::mem::zeroed ();
      inregs.h.ah = 2;
      inregs.h.dl = 7;
      intdos (&raw mut inregs, &raw mut outregs);
    }
}

/* If otherwise than -1, a face ID that will override the FACE argument
   to IT_set_face.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut it_face_override: c_int = -1;

/* Given a face id FACE, extract the face parameters to be used for
   display until the face changes.  The face parameters (actually, its
   color) are used to construct the video attribute byte for each
   glyph during the construction of the buffer that is then blitted to
   the video RAM.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_set_face (f: *mut frame, face_id: c_int)   // C 808-809
{
  let face: c_int = (if it_face_override == -1 { face_id } else { it_face_override });
  let mut fp: *mut face = FACE_FROM_ID_OR_NULL (f, face);
  let dfp: *mut face = FACE_FROM_ID_OR_NULL (f, DEFAULT_FACE_ID);
  let mut fg: c_ulong;
  let mut bg: c_ulong;
  let dflt_fg: c_ulong;
  let dflt_bg: c_ulong;
  let tty = FRAME_TTY! (f);

  if fp.is_null ()
    {
      fp = dfp;
      /* The default face for the frame should always be realized and
	 cached.  */
      if fp.is_null ()
	{ emacs_abort (); }
    }
  screen_face = face;
  fg = (*fp).foreground;
  bg = (*fp).background;
  dflt_fg = (*dfp).foreground;
  dflt_bg = (*dfp).background;

  /* Don't use invalid colors.  In particular, FACE_TTY_DEFAULT_* colors
     mean use the colors of the default face.  Note that we assume all
     16 colors to be available for the background, since Emacs switches
     on this mode (and loses the blinking attribute) at startup.  */
  if fg == (FACE_TTY_DEFAULT_COLOR as c_ulong) || fg == (FACE_TTY_DEFAULT_FG_COLOR as c_ulong)
    { fg = FRAME_FOREGROUND_PIXEL (f); }
  else if fg == (FACE_TTY_DEFAULT_BG_COLOR as c_ulong)
    { fg = FRAME_BACKGROUND_PIXEL (f); }
  if bg == (FACE_TTY_DEFAULT_COLOR as c_ulong) || bg == (FACE_TTY_DEFAULT_BG_COLOR as c_ulong)
    { bg = FRAME_BACKGROUND_PIXEL (f); }
  else if bg == (FACE_TTY_DEFAULT_FG_COLOR as c_ulong)
    { bg = FRAME_FOREGROUND_PIXEL (f); }

  /* Make sure highlighted lines really stand out, come what may.  */
  if (*fp).tty_reverse_p != 0 && (fg == dflt_fg && bg == dflt_bg)
    {
      let tem: c_ulong = fg;

      fg = bg;
      bg = tem;
    }
  /* If the user requested inverse video, obey.  */
  if inverse_video
    {
      let tem2: c_ulong = fg;

      fg = bg;
      bg = tem2;
    }
  if !(*tty).termscript.is_null ()
    { fprintf ((*tty).termscript, c"<FACE %d: %lu/%lu[FG:%lu/BG:%lu]>".as_ptr (), face,
	       (*fp).foreground, (*fp).background, fg, bg); }
  if fg >= 0 && fg < 16
    {
      ScreenAttrib &= 0xf0;
      ScreenAttrib |= fg as c_uchar;
    }
  if bg >= 0 && bg < 16
    {
      ScreenAttrib &= 0x0f;
      ScreenAttrib |= ((bg & 0x0f) << 4) as c_uchar;
    }
}

/* According to RBIL (INTERRUP.A, V-1000), 160 is the maximum possible
   width of a DOS display in any known text mode.  We multiply by 2 to
   accommodate the screen attribute byte.  */
/// C 878: `#define MAX_SCREEN_BUF 160*2`
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
pub const MAX_SCREEN_BUF: usize = 160 * 2;

/// C 880-881: the reference's own declaration of `encode_terminal_code` (the definition is
/// `src/term.c`'s; no mirror in this unit's graph carries it - see the ledger's Owed 3).
/// C: extern unsigned char *encode_terminal_code (struct glyph *, int, struct coding_system *);
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe extern "C" {
  pub fn encode_terminal_code (g: *mut glyph, n: c_int, coding: *mut coding_system) -> *mut c_uchar;
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_write_glyphs (f: *mut frame, mut str: *mut glyph, str_len: c_int)   // C 883-884
{
  let mut screen_buf: [c_uchar; MAX_SCREEN_BUF] = [0; MAX_SCREEN_BUF];
  let mut screen_bp: *mut c_uchar;
  let mut bp: *mut c_uchar;
  let offset: c_int = 2 * (new_pos_X + screen_size_X * new_pos_Y);
  let mut sl: c_int = str_len;
  let tty = FRAME_TTY! (f);
  let mut conversion_buffer: *mut c_uchar;
  let mut cf_frame: *mut c_void = f as *mut c_void;

  /* If terminal_coding does any conversion, use it, otherwise use
     safe_terminal_coding.  We can't use CODING_REQUIRE_ENCODING here
     because it always returns 1 if terminal_coding.src_multibyte is 1.  */
  /* The 3.455 mirror round (`include/termhooks.rs` 420's macro) spells the two coding fields
     `*mut c_void`, and every consumer casts at its use site; this unit is the one consumer
     the round left behind.  One `## Deviations` entry.  */
  let mut coding: *mut coding_system = FRAME_TERMINAL_CODING! (f) as *mut coding_system;

  if ((*coding).bits as c_int & CODING_REQUIRE_ENCODING_MASK as c_int) == 0
    { coding = &raw mut safe_terminal_coding; }

  if str_len <= 0 { return; }

  /* Since faces get cached and uncached behind our back, we can't
     rely on their indices in the cache being consistent across
     invocations.  So always reset the screen face to the default
     face of the frame, before writing glyphs, and let the glyphs
     set the right face if it's different from the default.  */
  IT_set_face (f, DEFAULT_FACE_ID);

  /* The mode bit CODING_MODE_LAST_BLOCK should be set to 1 only at
     the tail.  */
  (*coding).bits &= !((CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT);
  screen_bp = &raw mut screen_buf[0];
  while sl > 0
    {
      let mut cf: c_int;
      let mut n: c_int;

      /* If the face of this glyph is different from the current
	 screen face, update the screen attribute byte.  */
      cf = glyph_bits! (face_id, *str);
      if (cf != screen_face && cf != it_face_override)
	  || cf_frame != (*str).frame
	{ IT_set_face ((*str).frame as *mut frame, cf); }	/* handles invalid faces gracefully */
      cf_frame = (*str).frame;

      /* Identify a run of glyphs with the same face.  */
      n = 1;
      while n < sl
	{
	  if glyph_bits! (face_id, *(str.offset (n as isize))) != cf
	    { break; }
	  n += 1;
	}

      if n >= sl
	/* This is the last glyph.  */
	{ (*coding).bits |= (CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT; }

      conversion_buffer = encode_terminal_code (str, n, coding);
      if (*coding).produced > 0
	{
	  /* Copy the encoded bytes to the screen buffer.  */
	  bp = conversion_buffer;
	  while { let old = (*coding).produced; (*coding).produced -= 1; old != 0 }
	    {
	      /* Paranoia: discard bytes that would overrun the end of
		 the screen buffer.  */
	      if (screen_bp as *const c_uchar).offset_from (screen_buf.as_ptr ())
		   <= (MAX_SCREEN_BUF - 2) as isize
		{
		  *screen_bp = *bp as c_uchar;
		  screen_bp = screen_bp.offset (1);
		  *screen_bp = ScreenAttrib;
		  screen_bp = screen_bp.offset (1);
		}
	      if !(*tty).termscript.is_null ()
		{ fputc (*bp as c_int, (*tty).termscript); }
	      bp = bp.offset (1);
	    }
	}
      /* Update STR and its remaining length.  */
      str = str.offset (n as isize);
      sl -= n;
    }

  /* Dump whatever we have in the screen buffer.  */
  mouse_off_maybe ();
  dosmemput (screen_buf.as_ptr () as *const c_void,
	     (screen_bp as *const c_uchar).offset_from (screen_buf.as_ptr ()) as c_int,
	     ScreenPrimary as c_int + offset);
  if screen_virtual_segment != 0
    {
      dosv_refresh_virtual_screen (offset,
				   ((screen_bp as *const c_uchar)
				      .offset_from (screen_buf.as_ptr ()) / 2) as c_int);
    }
  new_pos_X += ((screen_bp as *const c_uchar).offset_from (screen_buf.as_ptr ()) / 2) as c_int;
}

/************************************************************************
			  Mouse Highlight (and friends..)
 ************************************************************************/

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut mouse_preempted: c_int = 0;	/* non-zero when XMenu gobbles mouse events */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub unsafe fn popup_activated () -> c_int              // C 972-973
{
  return mouse_preempted;
}

/* Write a string of glyphs STRING of length LEN to F, disregarding
   their invidiual faces in favor of FACE.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_write_glyphs_with_face (f: *mut frame, string: *mut glyph,
					   len: c_int, face_id: c_int)   // C 981-982
{
  it_face_override = face_id;
  IT_write_glyphs (f, string, len);
  it_face_override = -1;
}

/* Draw TEXT_AREA glyphs between START and END of glyph row ROW on
   window W.  X is relative to TEXT_AREA in W.  HL is a face override
   for drawing the glyphs.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub unsafe fn tty_draw_row_with_mouse_face (w: *mut window, row: *mut glyph_row,
					    start_hpos: c_int, end_hpos: c_int,
					    hl: draw_glyphs_face)   // C 993-996
{
  let f = XFRAME (WINDOW_FRAME (w));
  let tty = FRAME_TTY! (f);
  let hlinfo = &raw mut (*tty).mouse_highlight;
  let mut nglyphs: c_int = end_hpos - start_hpos;
  let mut pos_y: c_int;
  let mut pos_x: c_int;
  let mut save_y: c_int;
  let mut save_x: c_int;

  if end_hpos >= (*row).used[TEXT_AREA as usize] as c_int
    { nglyphs = (*row).used[TEXT_AREA as usize] as c_int - start_hpos; }
  pos_y = (*row).y + WINDOW_TOP_EDGE_Y! (w);
  pos_x = ((*row).used[LEFT_MARGIN_AREA as usize] as c_int + start_hpos
	   + WINDOW_LEFT_EDGE_X! (w));

  /* Save current cursor position.  */
  save_x = (*f).left_pos + new_pos_X;
  save_y = (*f).top_pos + new_pos_Y;

  /* This applies child frame offsets.  */
  cursor_to (f, pos_y, pos_x);
  mouse_off ();

  /* Write the glyphs and restore cursor position.  */

  if hl == DRAW_MOUSE_FACE
    {
      let glyph: *mut glyph = (*row).glyphs[TEXT_AREA as usize].offset (start_hpos as isize);
      let face_id: c_int = (*tty).mouse_highlight.mouse_face_face_id;
      IT_write_glyphs_with_face (f, glyph, nglyphs, face_id);
    }
  else
    {
      IT_write_glyphs (f, (*row).glyphs[TEXT_AREA as usize].offset (start_hpos as isize),
		       nglyphs);
    }

  mouse_on ();
  new_pos_X = save_x;
  new_pos_Y = save_y;
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_clear_end_of_line (f: *mut frame, first_unused: c_int)   // C 1035-1036
{
  let mut spaces: *mut c_char;
  let mut sp: *mut c_char;
  let mut i: c_int;
  let mut j: c_int;
  let offset: c_int = 2 * (new_pos_X + screen_size_X * new_pos_Y);
  let tty = FRAME_TTY! (f);

  if new_pos_X >= first_unused || fatal_error_in_progress
    { return; }

  IT_set_face (f, 0);
  j = first_unused - new_pos_X;
  i = j * 2;
  if !(*tty).termscript.is_null ()
    { fprintf ((*tty).termscript, c"<CLR:EOL[%d..%d)>".as_ptr (), new_pos_X, first_unused); }
  /* C's `spaces = sp = alloca (i)`: the port has no `alloca` (PORTING.md P-05), so the block
     comes from the stack pool `sa_stack_alloc` serves - the `src/coding.rs` 8842 shape, whose
     `sa_avail` accounting C does not do here either.  One `## Deviations` entry.  */
  sp = sa_stack_alloc (i as usize) as *mut c_char;
  spaces = sp;

  while { j -= 1; j >= 0 }
    {
      *sp = ' ' as c_char;
      sp = sp.offset (1);
      *sp = ScreenAttrib as c_char;
      sp = sp.offset (1);
    }

  mouse_off_maybe ();
  dosmemput (spaces as *const c_void, i, ScreenPrimary as c_int + offset);
  if screen_virtual_segment != 0
    { dosv_refresh_virtual_screen (offset, i / 2); }

  /* clear_end_of_line_raw on term.c leaves the cursor at first_unused.
     Let's follow their lead, in case someone relies on this.  */
  new_pos_X = first_unused;
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_clear_screen (f: *mut frame)          // C 1067-1068
{
  let tty = FRAME_TTY! (f);

  if !(*tty).termscript.is_null ()
    { fprintf ((*tty).termscript, c"<CLR:SCR>".as_ptr ()); }
  /* We are sometimes called (from clear_garbaged_frames) when a new
     frame is being created, but its faces are not yet realized.  In
     such a case we cannot call IT_set_face, since it will fail to find
     any valid faces and will abort.  Instead, use the initial screen
     colors; that should mimic what a Unix tty does, which simply clears
     the screen with whatever default colors are in use.  */
  if FACE_FROM_ID_OR_NULL (f, DEFAULT_FACE_ID).is_null ()
    {
      ScreenAttrib = ((initial_screen_colors[0] << 4)
		      | initial_screen_colors[1]) as c_uchar;
    }
  else
    { IT_set_face (f, 0); }
  mouse_off ();
  ScreenClear ();
  if screen_virtual_segment != 0
    { dosv_refresh_virtual_screen (0, screen_size); }
  new_pos_X = 0;
  new_pos_Y = 0;
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_clear_to_end (f: *mut frame)          // C 1091-1092
{
  let tty = FRAME_TTY! (f);

  if !(*tty).termscript.is_null ()
    { fprintf ((*tty).termscript, c"<CLR:EOS>".as_ptr ()); }

  while new_pos_Y < screen_size_Y {
    new_pos_X = 0;
    IT_clear_end_of_line (f, screen_size_X);
    new_pos_Y += 1;
  }
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_cursor_to (f: *mut frame, y: c_int, x: c_int)   // C 1106-1107
{
  let tty = FRAME_TTY! (f);

  if !(*tty).termscript.is_null ()
    { fprintf ((*tty).termscript, c"\n<XY=%dx%d>".as_ptr (), x, y); }
  new_pos_X = x;
  new_pos_Y = y;
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut cursor_cleared: c_int = 0;

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_display_cursor (on: c_int)            // C 1119-1120
{
  let tty = CURTTY! ();

  if on != 0 && cursor_cleared != 0
    {
      ScreenSetCursor (current_pos_Y, current_pos_X);
      cursor_cleared = 0;
      if !(*tty).termscript.is_null ()
	{
	  fprintf ((*tty).termscript, c"\nCURSOR ON (%dx%d)".as_ptr (),
		   current_pos_Y, current_pos_X);
	}
    }
  else if on == 0 && cursor_cleared == 0
    {
      ScreenSetCursor (-1, -1);
      cursor_cleared = 1;
      if !(*tty).termscript.is_null ()
	{
	  fprintf ((*tty).termscript, c"\nCURSOR OFF (%dx%d)".as_ptr (),
		   current_pos_Y, current_pos_X);
	}
    }
}

/* Emacs calls cursor-movement functions a lot when it updates the
   display (probably a legacy of old terminals where you cannot
   update a screen line without first moving the cursor there).
   However, cursor movement is expensive on MSDOS (it calls a slow
   BIOS function and requires 2 mode switches), while actual screen
   updates access the video memory directly and don't depend on
   cursor position.  To avoid slowing down the redisplay, we cheat:
   all functions that move the cursor only set internal variables
   which record the cursor position, whereas the cursor is only
   moved to its final position whenever screen update is complete.

   `IT_cmgoto' is called from the keyboard reading loop and when the
   frame update is complete.  This means that we are ready for user
   input, so we update the cursor position to show where the point is,
   and also make the mouse pointer visible.

   Special treatment is required when the cursor is in the echo area,
   to put the cursor at the end of the text displayed there.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_cmgoto (f: *mut frame)                // C 1161-1162
{
  /* Only set the cursor to where it should be if the display is
     already in sync with the window contents.  */
  let mut update_cursor_pos: c_int = 1; /* MODIFF == unchanged_modified; */
  let tty = FRAME_TTY! (f);

  /* FIXME: This needs to be rewritten for the new redisplay, or
     removed.  */
  /* C 1171-1211 is `#if 0`: `previous_pos_X` and the echo-area cursor restore.  C's own
     preprocessor never compiles it, so the port keeps C's text as a comment (the
     `src/cm.rs` 86 / `src/character.rs` 118 `#if 0` shape).

#if 0
  static int previous_pos_X = -1;

  update_cursor_pos = 1;	/* temporary!!! */

  /* If the display is in sync, forget any previous knowledge about
     cursor position.  This is primarily for unexpected events like
     C-g in the minibuffer.  */
  if (update_cursor_pos && previous_pos_X >= 0)
    previous_pos_X = -1;
  /* If we are in the echo area, put the cursor at the
     end of the echo area message.  */
  if (!update_cursor_pos
      && WINDOW_TOP_EDGE_LINE (XWINDOW (FRAME_MINIBUF_WINDOW (f))) <= new_pos_Y)
    {
      int tem_X = current_pos_X, dummy;

      if (echo_area_glyphs)
	{
	  tem_X = echo_area_glyphs_length;
	  /* Save current cursor position, to be restored after the
	     echo area message is erased.  Only remember one level
	     of previous cursor position.  */
	  if (previous_pos_X == -1)
	    ScreenGetCursor (&dummy, &previous_pos_X);
	}
      else if (previous_pos_X >= 0)
	{
	  /* We wind up here after the echo area message is erased.
	     Restore the cursor position we remembered above.  */
	  tem_X = previous_pos_X;
	  previous_pos_X = -1;
	}

      if (current_pos_X != tem_X)
	{
	  new_pos_X = tem_X;
	  update_cursor_pos = 1;
	}
    }
#endif
  */

  if update_cursor_pos != 0
      && (current_pos_X != new_pos_X || current_pos_Y != new_pos_Y)
    {
      current_pos_Y = new_pos_Y;	/* C's `ScreenSetCursor (current_pos_Y = …, current_pos_X = …)` */
      current_pos_X = new_pos_X;
      ScreenSetCursor (current_pos_Y, current_pos_X);
      if !(*tty).termscript.is_null ()
	{
	  fprintf ((*tty).termscript, c"\n<CURSOR:%dx%d>".as_ptr (),
		   current_pos_X, current_pos_Y);
	}
    }

  /* Maybe cursor is invisible, so make it visible.  */
  IT_display_cursor (1);

  /* Mouse pointer should be always visible if we are waiting for
     keyboard input.  */
  if mouse_visible == 0
    { mouse_on (); }
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_update_begin (f: *mut frame)          // C 1230-1231
{
  let display_info = FRAME_DISPLAY_INFO! (f);
  let hlinfo: *mut Mouse_HLInfo = &raw mut (*display_info).mouse_highlight;
  let mouse_face_frame: *mut frame = (*hlinfo).mouse_face_mouse_frame as *mut frame;

  if !(*display_info).termscript.is_null ()
    { fprintf ((*display_info).termscript, c"\n\n<UPDATE_BEGIN".as_ptr ()); }

  block_input ();

  if !f.is_null () && f == mouse_face_frame
    {
      /* Don't do highlighting for mouse motion during the update.  */
      set_mouse_face_defer (hlinfo, true);

      /* If F needs to be redrawn, simply forget about any prior mouse
	 highlighting.  */
      if FRAME_GARBAGED_P (f)
	{ (*hlinfo).mouse_face_window = Qnil (); }

      /* Can we tell that this update does not affect the window
	 where the mouse highlight is?  If so, no need to turn off.
	 Likewise, don't do anything if none of the enabled rows
	 contains glyphs highlighted in mouse face.  */
      if !NILP ((*hlinfo).mouse_face_window)
	  && WINDOWP ((*hlinfo).mouse_face_window)
	{
	  let w = XWINDOW ((*hlinfo).mouse_face_window);
	  let mut i: c_int;

	  /* If the mouse highlight is in the window that was deleted
	     (e.g., if it was popped by completion), clear highlight
	     unconditionally.  */
	  if NILP ((*w).contents)
	    { (*hlinfo).mouse_face_window = Qnil (); }
	  else
	    {
	      i = 0;
	      while i < (*( (*w).desired_matrix as *mut glyph_matrix )).nrows
		{
		  if MATRIX_ROW_ENABLED_P! ((*w).desired_matrix as *mut glyph_matrix, i)
		      && glyph_row_bits! (mouse_face_p,
					  MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, i))
		    { break; }
		  i += 1;
		}
	    }

	  if NILP ((*w).contents) || i < (*( (*w).desired_matrix as *mut glyph_matrix )).nrows
	    { clear_mouse_face (hlinfo); }
	}
    }
  else if !mouse_face_frame.is_null () && !FRAME_LIVE_P (mouse_face_frame)
    /* If the frame with mouse highlight was deleted, invalidate the
       highlight info.  */
    { reset_mouse_highlight (hlinfo); }

  unblock_input ();
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_update_end (f: *mut frame)            // C 1287-1288
{
  let dpyinfo = FRAME_DISPLAY_INFO! (f);

  if !(*dpyinfo).termscript.is_null ()
    { fprintf ((*dpyinfo).termscript, c"\n<UPDATE_END\n".as_ptr ()); }
  set_mouse_face_defer (&raw mut (*dpyinfo).mouse_highlight, false);
}

/* ----------------------------------------------------------------------------------------------
   C 1296-1771: the rest of the second `#ifndef HAVE_X_WINDOWS` block (C 480's).  Every item in
   this span carries `not (feature = "have_x_windows")` on top of the file gate; C 1771's `#endif`
   closes it, so the items after that line (`internal_terminal_init` onward) are file-gated only.
   Both arms of every `#if` in the span are carried: C 1784's `#ifdef HAVE_X_WINDOWS`, C 1796's
   `#ifndef HAVE_X_WINDOWS` and C 1907's `#ifndef`/C 1912's `#else`.
   ---------------------------------------------------------------------------------------------- */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_frame_up_to_date (f: *mut frame)      // C 1297-1298
{
  let frame_desired_cursor: Lisp_Object;
  let new_cursor: Lisp_Object;
  let sw: *mut window;

  FRAME_MOUSE_UPDATE! (f);

  /* Set the cursor type to whatever they wanted.  In a minibuffer
     window, we want the cursor to appear only if we are reading input
     from this window, and we want the cursor to be taken from the
     frame parameters.  For the selected window, we use either its
     buffer-local value or the value from the frame parameters if the
     buffer doesn't define its local value for the cursor type.  */
  sw = XWINDOW ((*f).selected_window);
  frame_desired_cursor = Fcdr (Fassq (Qcursor_type (), (*f).param_alist));
  if cursor_in_echo_area
      && FRAME_HAS_MINIBUF_P (f)
      && EQ (FRAME_MINIBUF_WINDOW (f), echo_area_window)
      && sw == XWINDOW (echo_area_window)
    { new_cursor = frame_desired_cursor; }
  else
    {
      let b: *mut buffer = XBUFFER ((*sw).contents);

      if EQ (BVAR! (b, cursor_type_), Qt ())
	{ new_cursor = frame_desired_cursor; }
      else if NILP (BVAR! (b, cursor_type_)) /* nil means no cursor */
	{ new_cursor = Fcons (Qbar (), make_fixnum (0)); }
      else
	{ new_cursor = BVAR! (b, cursor_type_); }
    }

  IT_set_cursor_type (f, new_cursor);

  IT_cmgoto (f);  /* position cursor when update is done */
}

/* Copy LEN glyphs displayed on a single line whose vertical position
   is YPOS, beginning at horizontal position XFROM to horizontal
   position XTO, by moving blocks in the video memory.  Used by
   functions that insert and delete glyphs.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_copy_glyphs (xfrom: c_int, xto: c_int, mut len: usize, ypos: c_int)  // C 1339-1340
{
  /* The offsets of source and destination relative to the
     conventional memory selector.  */
  let mut from: c_int = 2 * (xfrom + screen_size_X * ypos) + ScreenPrimary as c_int;
  let mut to: c_int = 2 * (xto + screen_size_X * ypos) + ScreenPrimary as c_int;

  if from == to || len <= 0
    { return; }

  _farsetsel (_dos_ds! () as c_ushort);

  /* The source and destination might overlap, so we need to move
     glyphs non-destructively.  */
  if from > to
    {
      while len != 0
	{
	  _farnspokew (to as c_ulong, _farnspeekw (from as c_ulong));
	  from += 2;
	  to += 2;
	  len -= 1;
	}
    }
  else
    {
      from += ((len - 1) * 2) as c_int;
      to += ((len - 1) * 2) as c_int;
      while len != 0
	{
	  _farnspokew (to as c_ulong, _farnspeekw (from as c_ulong));
	  from -= 2;
	  to -= 2;
	  len -= 1;
	}
    }
  if screen_virtual_segment != 0
    { dosv_refresh_virtual_screen (ypos * screen_size_X * 2, screen_size_X); }
}

/* Insert and delete glyphs.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_insert_glyphs (f: *mut frame, start: *mut glyph, len: c_int)   // C 1371-1372
{
  let shift_by_width: c_int = screen_size_X - (new_pos_X + len);

  /* Shift right the glyphs from the nominal cursor position to the
     end of this line.  */
  IT_copy_glyphs (new_pos_X, new_pos_X + len, shift_by_width as usize, new_pos_Y);

  /* Now write the glyphs to be inserted.  */
  IT_write_glyphs (f, start, len);
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_delete_glyphs (f: *mut frame, n: c_int)   // C 1384-1385
{
  emacs_abort ();
}

/* This was copied from xfaces.c  */

/* IT_set_terminal_modes is called when emacs is started,
   resumed, and whenever the screen is redrawn!  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_set_terminal_modes (term: *mut terminal)   // C 1395-1396
{
  let mut tty: *mut tty_display_info;

  /* If called with initial terminal, it's too early to do anything
     useful.  */
  if (*term).r#type == output_initial
    { return; }

  tty = (*term).display_info.tty as *mut tty_display_info;

  if !(*tty).termscript.is_null ()
    { fprintf ((*tty).termscript, c"\n<SET_TERM>".as_ptr ()); }

  screen_size_X = ScreenCols ();
  screen_size_Y = ScreenRows ();
  screen_size = screen_size_X * screen_size_Y;

  /* C's `new_pos_X = new_pos_Y = 0;` and `current_pos_X = current_pos_Y = -1;` are chained
     assignments; the port writes the two stores of each (the `haikuterm.rs` 1255 shape).  */
  new_pos_Y = 0;
  new_pos_X = 0;
  current_pos_Y = -1;
  current_pos_X = -1;

  if term_setup_done != 0
    { return; }
  term_setup_done = 1;

  startup_screen_size_X = screen_size_X;
  startup_screen_size_Y = screen_size_Y;
  startup_screen_attrib = ScreenAttrib;

  /* Is DOS/V (or any other RSIS software which relocates
     the screen) installed?  */
  {
    let mut es_value: c_ushort;
    let mut regs: __dpmi_regs = std::mem::zeroed ();

    regs.h.ah = 0xfe;	/* get relocated screen address */
    if ScreenPrimary == 0xb0000 || ScreenPrimary == 0xb8000
      { regs.x.es = ((ScreenPrimary >> 4) & 0xffff) as c_ushort; }
    else if screen_old_address != 0 /* already switched to Japanese mode once */
      { regs.x.es = ((screen_old_address >> 4) & 0xffff) as c_ushort; }
    else
      { regs.x.es = if ScreenMode () == 7 { 0xb000 } else { 0xb800 }; }
    regs.x.di = 0;
    es_value = regs.x.es;
    __dpmi_int (0x10, &raw mut regs);

    if regs.x.es != es_value
      {
	/* screen_old_address is only set if ScreenPrimary does NOT
	   already point to the relocated buffer address returned by
	   the Int 10h/AX=FEh call above.  DJGPP v2.02 and later sets
	   ScreenPrimary to that address at startup under DOS/V.  */
	if regs.x.es != ((ScreenPrimary >> 4) & 0xffff) as c_ushort
	  { screen_old_address = ScreenPrimary; }
	screen_virtual_segment = regs.x.es;
	screen_virtual_offset  = regs.x.di;
	ScreenPrimary = ((screen_virtual_segment << 4) + screen_virtual_offset) as c_ulong;
      }
  }

  ScreenGetCursor (&raw mut startup_pos_Y, &raw mut startup_pos_X);
  /* C's `ScreenRetrieve (startup_screen_buffer = xmalloc (screen_size * 2))`: the store and the
     call, with the assignment's value kept (the `xrdb.rs` 368 chained-assignment shape).  */
  startup_screen_buffer = xmalloc ((screen_size * 2) as usize);
  ScreenRetrieve (startup_screen_buffer);

  bright_bg ();
}

/* IT_reset_terminal_modes is called when emacs is
   suspended or killed.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_reset_terminal_modes (term: *mut terminal)   // C 1465-1466
{
  let mut display_row_start: c_int = ScreenPrimary as c_int;
  let saved_row_len: c_int     = startup_screen_size_X * 2;
  let mut update_row_len: c_int    = ScreenCols () * 2;
  let mut current_rows: c_int = ScreenRows ();
  let to_next_row: c_int       = update_row_len;
  let mut saved_row: *mut c_uchar = startup_screen_buffer as *mut c_uchar;
  let mut cursor_pos_X: c_int = ScreenCols () - 1;
  let mut cursor_pos_Y: c_int = ScreenRows () - 1;
  let tty: *mut tty_display_info = (*term).display_info.tty as *mut tty_display_info;

  if !(*tty).termscript.is_null ()
    { fprintf ((*tty).termscript, c"\n<RESET_TERM>".as_ptr ()); }

  if term_setup_done == 0
    { return; }

  mouse_off ();

  /* Leave the video system in the same state as we found it,
     as far as the blink/bright-background bit is concerned.  */
  maybe_enable_blinking ();

  /* We have a situation here.
     We cannot just do ScreenUpdate(startup_screen_buffer) because
     the luser could have changed screen dimensions inside Emacs
     and failed (or didn't want) to restore them before killing
     Emacs.  ScreenUpdate() uses the *current* screen dimensions and
     thus will happily use memory outside what was allocated for
     `startup_screen_buffer'.
     Thus we only restore as much as the current screen dimensions
     can hold, and clear the rest (if the saved screen is smaller than
     the current) with the color attribute saved at startup.  The cursor
     is also restored within the visible dimensions.  */

  ScreenAttrib = startup_screen_attrib;

  /* Don't restore the screen if we are exiting less than 2 seconds
     after startup: we might be crashing, and the screen might show
     some vital clues to what's wrong.  */
  if clock () - startup_time >= 2*CLOCKS_PER_SEC
    {
      ScreenClear ();
      if screen_virtual_segment != 0
	{ dosv_refresh_virtual_screen (0, screen_size); }

      if update_row_len > saved_row_len
	{ update_row_len = saved_row_len; }
      if current_rows > startup_screen_size_Y
	{ current_rows = startup_screen_size_Y; }

      if !(*tty).termscript.is_null ()
	{ fprintf ((*tty).termscript, c"<SCREEN RESTORED (dimensions=%dx%d)>\n".as_ptr (),
		   update_row_len / 2, current_rows); }

      while current_rows != 0   /* C's `while (current_rows--)` test */
	{
	  current_rows -= 1;
	  dosmemput (saved_row as *const c_void, update_row_len, display_row_start);
	  if screen_virtual_segment != 0
	    { dosv_refresh_virtual_screen (display_row_start - ScreenPrimary as c_int,
					   update_row_len / 2); }
	  saved_row         = saved_row.offset (saved_row_len as isize);
	  display_row_start += to_next_row;
	}
    }
  if startup_pos_X < cursor_pos_X
    { cursor_pos_X = startup_pos_X; }
  if startup_pos_Y < cursor_pos_Y
    { cursor_pos_Y = startup_pos_Y; }

  ScreenSetCursor (cursor_pos_Y, cursor_pos_X);
  xfree (startup_screen_buffer as *mut c_void);
  startup_screen_buffer = core::ptr::null_mut ();

  term_setup_done = 0;
}

/* Remember the screen colors of the current frame, to serve as the
   default colors for newly-created frames.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
DEFUN! ("msdos-remember-default-colors", Fmsdos_remember_default_colors,
       Smsdos_remember_default_colors, 1, 1, 0,
       doc: "Remember the screen colors of the current frame.  ",   // C 1544-1546
       (frame: Lisp_Object),
       {
  let mut f: *mut frame;

  CHECK_FRAME! (frame);
  f = XFRAME (frame);

  /* This function is called after applying default-frame-alist to the
     initial frame.  At that time, if reverse-colors option was
     specified in default-frame-alist, it was already applied, and
     frame colors are reversed.  */
  initial_screen_colors[0] = FRAME_FOREGROUND_PIXEL (f) as c_int;
  initial_screen_colors[1] = FRAME_BACKGROUND_PIXEL (f) as c_int;

  return Qnil ();
});

/* `FRAME_FOREGROUND_PIXEL`/`FRAME_BACKGROUND_PIXEL` are C's plain field macros (frame.h
   1375-1376): `include/frame.rs` declares the readers as functions (1274/1276) and the tree
   spells the write form per consuming unit (`src/frame.rs` 134/137, `src/dispnew.rs`
   7369-7370).  This unit's copy carries the file gate; C 1584-1585/1638/1648/1668/1678/1747/1754
   are its uses.  One `## Deviations` entry.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
macro_rules! FRAME_FOREGROUND_PIXEL {
  ($f:expr) => { (*$f).foreground_pixel };
}
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
macro_rules! FRAME_BACKGROUND_PIXEL {
  ($f:expr) => { (*$f).background_pixel };
}
/* C's `f->default_face_done_p` (frame.h 340, a `bool_bf`): the same per-unit reader/writer the
   `mouse_moved!` pair above is (`include/frame.rs` 343-344 carries the measured shift and unit,
   and the field's unit is 1 on every arm, so the `set_frame_flag1!` helper serves it).  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
macro_rules! default_face_done_p {
  ($f:expr) => { (*$f).bits1 >> FRAME_DEFAULT_FACE_DONE_P_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_DEFAULT_FACE_DONE_P_SHIFT, $v) };
}
/* frame.h 1273's `FRAME_NO_SPECIAL_GLYPHS` (`f->no_special_glyphs`): `include/frame.rs` 1218
   declares the reader, and C 1738 writes it, so this unit adds the writer arm (the
   `src/frame.rs` 382 shape, one `## Deviations` entry).  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
macro_rules! FRAME_NO_SPECIAL_GLYPHS {
  ($f:expr) => { FRAME_NO_SPECIAL_GLYPHS ($f) };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_NO_SPECIAL_GLYPHS_SHIFT, $v) };
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub unsafe fn IT_set_frame_parameters (f: *mut frame, alist: Lisp_Object)   // C 1564-1565
{
  let mut tail: Lisp_Object;
  let mut i: c_int;
  let mut j: c_int;
  let length: c_int = XFIXNUM (Flength (alist)) as c_int;
  /* C's two `alloca (length * word_size)` requests (C 1569-1572): the port has no `alloca`
     (PORTING.md P-05), so each block comes from the stack pool `sa_stack_alloc` serves - the
     C 1050 deviation's shape.  C accounts neither request against `sa_avail` and neither does
     the port.  */
  let parms: *mut Lisp_Object
    = sa_stack_alloc ((length as isize * word_size) as usize) as *mut Lisp_Object;
  let values: *mut Lisp_Object
    = sa_stack_alloc ((length as isize * word_size) as usize) as *mut Lisp_Object;
  /* Do we have to reverse the foreground and background colors?  */
  let mut reverse: c_int = EQ (Fcdr (Fassq (Qreverse (), (*f).param_alist)), Qt ()) as c_int;
  let mut redraw: c_int = 0;
  let mut fg_set: c_int = 0;
  let mut bg_set: c_int = 0;
  let mut orig_fg: c_ulong;
  let mut orig_bg: c_ulong;
  let tty = FRAME_TTY! (f);

  /* If we are creating a new frame, begin with the original screen colors
     used for the initial frame.  */
  if !default_face_done_p! (f)
      && initial_screen_colors[0] != -1 && initial_screen_colors[1] != -1
    {
      FRAME_FOREGROUND_PIXEL! (f) = initial_screen_colors[0] as c_ulong;
      FRAME_BACKGROUND_PIXEL! (f) = initial_screen_colors[1] as c_ulong;
      init_frame_faces (f);
      default_face_done_p! (f, 1);
    }
  orig_fg = if reverse != 0 { FRAME_BACKGROUND_PIXEL (f) } else { FRAME_FOREGROUND_PIXEL (f) };
  orig_bg = if reverse != 0 { FRAME_FOREGROUND_PIXEL (f) } else { FRAME_BACKGROUND_PIXEL (f) };

  /* Extract parm names and values into those vectors.  */
  i = 0;
  tail = alist;
  while CONSP (tail)
    {
      let elt: Lisp_Object = XCAR (tail);
      *parms.add (i as usize) = Fcar (elt);
      CHECK_SYMBOL (*parms.add (i as usize));
      *values.add (i as usize) = Fcdr (elt);
      i += 1;
      tail = XCDR (tail);
    }

  j = i;

  i = 0;
  while i < j
    {
      let prop: Lisp_Object;
      let val: Lisp_Object;

      prop = *parms.add (i as usize);
      val  = *values.add (i as usize);

      if EQ (prop, Qreverse ())
	{ reverse = EQ (val, Qt ()) as c_int; }
      i += 1;
    }

  if !(*tty).termscript.is_null () && reverse != 0
    { fprintf ((*tty).termscript, c"<INVERSE-VIDEO>\n".as_ptr ()); }

  /* Now process the alist elements in reverse of specified order.  */
  i -= 1;
  while i >= 0
    {
      let prop: Lisp_Object;
      let val: Lisp_Object;

      prop = *parms.add (i as usize);
      val  = *values.add (i as usize);

      if EQ (prop, Qforeground_color ())
	{
	  let new_color: c_ulong = load_color (f, std::ptr::null_mut::<face> (), val,
					       if reverse != 0 { LFACE_BACKGROUND_INDEX }
					       else { LFACE_FOREGROUND_INDEX });
	  if new_color !=  FACE_TTY_DEFAULT_COLOR
	      && new_color != FACE_TTY_DEFAULT_FG_COLOR
	      && new_color != FACE_TTY_DEFAULT_BG_COLOR
	    {
	      if reverse == 0
		{
		  FRAME_FOREGROUND_PIXEL! (f) = new_color;
		  /* Make sure the foreground of the default face for
		     this frame is changed as well.  */
		  update_face_from_frame_parameter (f, Qforeground_color (), val);
		  fg_set = 1;
		  if !(*tty).termscript.is_null ()
		    { fprintf ((*tty).termscript, c"<FGCOLOR %lu>\n".as_ptr (), new_color); }
		}
	      else
		{
		  FRAME_BACKGROUND_PIXEL! (f) = new_color;
		  update_face_from_frame_parameter (f, Qbackground_color (), val);
		  bg_set = 1;
		  if !(*tty).termscript.is_null ()
		    { fprintf ((*tty).termscript, c"<BGCOLOR %lu>\n".as_ptr (), new_color); }
		}
	      redraw = 1;
	    }
	}
      else if EQ (prop, Qbackground_color ())
	{
	  let new_color: c_ulong = load_color (f, std::ptr::null_mut::<face> (), val,
					       if reverse != 0 { LFACE_FOREGROUND_INDEX }
					       else { LFACE_BACKGROUND_INDEX });
	  if new_color != FACE_TTY_DEFAULT_COLOR
	      && new_color != FACE_TTY_DEFAULT_FG_COLOR
	      && new_color != FACE_TTY_DEFAULT_BG_COLOR
	    {
	      if reverse == 0
		{
		  FRAME_BACKGROUND_PIXEL! (f) = new_color;
		  /* Make sure the background of the default face for
		     this frame is changed as well.  */
		  bg_set = 1;
		  update_face_from_frame_parameter (f, Qbackground_color (), val);
		  if !(*tty).termscript.is_null ()
		    { fprintf ((*tty).termscript, c"<BGCOLOR %lu>\n".as_ptr (), new_color); }
		}
	      else
		{
		  FRAME_FOREGROUND_PIXEL! (f) = new_color;
		  fg_set = 1;
		  update_face_from_frame_parameter (f, Qforeground_color (), val);
		  if !(*tty).termscript.is_null ()
		    { fprintf ((*tty).termscript, c"<FGCOLOR %lu>\n".as_ptr (), new_color); }
		}
	      redraw = 1;
	    }
	}
      else if EQ (prop, Qtitle ())
	{
	  x_set_title (f as *mut c_void, val);
	  if !(*tty).termscript.is_null ()
	    { fprintf ((*tty).termscript, c"<TITLE: %s>\n".as_ptr (), SDATA (val)); }
	}
      else if EQ (prop, Qcursor_type ())
	{
	  IT_set_cursor_type (f, val);
	  if !(*tty).termscript.is_null ()
	    { fprintf ((*tty).termscript, c"<CTYPE: %s>\n".as_ptr (),
		       if EQ (val, Qbar ())
			  || EQ (val, Qhbar ())
			  || (CONSP (val) && (EQ (XCAR (val), Qbar ())
					      || EQ (XCAR (val), Qhbar ())))
			 { c"bar".as_ptr () } else { c"box".as_ptr () }); }
	}
      else if EQ (prop, Qtty_type ())
	{
	  internal_terminal_init ();
	  if !(*tty).termscript.is_null ()
	    { fprintf ((*tty).termscript, c"<TERM_INIT done, TTY_TYPE: %.*s>\n".as_ptr (),
		       SBYTES (val), SDATA (val)); }
	}
      store_frame_param (f, prop, val);
      i -= 1;
    }

  /* If this frame has become a child frame, process its visibility and
     a nuumber of other flags.  */
  if is_tty_child_frame (f)
    {
      let w: c_int = tty_child_size_param (f, Qwidth (), alist, (*f).total_cols);
      let h: c_int = tty_child_size_param (f, Qheight (), alist, (*f).total_lines);
      if w != (*f).total_cols || h != (*f).total_lines
	{ change_frame_size (f, w, h, false, false, false); }

      let x: c_int = tty_child_pos_param (f, Qleft (), alist, (*f).left_pos, w);
      let y: c_int = tty_child_pos_param (f, Qtop (), alist, (*f).top_pos, h);
      if x != (*f).left_pos || y != (*f).top_pos
	{
	  (*f).left_pos = x;
	  (*f).top_pos = y;
	  SET_FRAME_GARBAGED! (root_frame (f));
	}

      let visible: Lisp_Object = Fassq (Qvisibility (), alist);
      if CONSP (visible)
	{ SET_FRAME_VISIBLE! (f, !NILP (Fcdr (visible))); }

      let no_special: Lisp_Object = Fassq (Qno_special_glyphs (), alist);
      if CONSP (no_special)
	{ FRAME_NO_SPECIAL_GLYPHS! (f, !NILP (Fcdr (no_special)) as u32); }
    }

  /* If they specified "reverse", but not the colors, we need to swap
     the current frame colors.  */
  if reverse != 0
    {
      if fg_set == 0
	{
	  FRAME_FOREGROUND_PIXEL! (f) = orig_bg;
	  update_face_from_frame_parameter (f, Qforeground_color (),
					    tty_color_name (f, orig_bg));
	  redraw = 1;
	}
      if bg_set == 0
	{
	  FRAME_BACKGROUND_PIXEL! (f) = orig_fg;
	  update_face_from_frame_parameter (f, Qbackground_color (),
					    tty_color_name (f, orig_fg));
	  redraw = 1;
	}
    }

  if redraw != 0
    {
      face_change = true;	/* forces xdisp.c to recompute basic faces */
      if f == SELECTED_FRAME ()
	{ redraw_frame (f); }
    }
}

/* C 1769's `extern void init_frame_faces (struct frame *);` is C's own re-declaration of
   `dispextern.h` 3821's prototype (the call at C 1586 and C 1867); the port does not repeat a
   declaration the mirror carries, so the line is dropped (PORTING.md 0.1-12).  One `## Deviations`
   entry.  */

/* C 1771: `#endif` - the C 480 `#ifndef HAVE_X_WINDOWS` block ends here.  Everything below this
   line is gated by the file alone, plus each item's own conditions.  */

/* The `hook!` cast this unit's hook assignments need is `src/term.rs` 140-144's: C stores the
   address of a function whose Rust ABI is "Rust" in an `Option<unsafe extern "C" fn …>` field, so
   each assignment transmutes the address (the P-90 ① per-unit helper shape).  */
#[cfg (feature = "msdos")]
macro_rules! hook {
  ($target:ty, $f:expr) => { unsafe {
    let p: *const c_void = $f as *const c_void;
    std::mem::transmute::<*const c_void, Option<$target>> (p) } };
}
/* C's `f->output_method = …` writes frame.h 446's four-bit field: `include/frame.rs` 1360
   declares the reader (`output_method (f)`), and this unit adds the writer arm (the
   `src/dispnew.rs` 7349 / `src/frame.rs` 342 `set_output_method!` shape).  C 1799/1805 are its
   uses.  */
#[cfg (feature = "msdos")]
macro_rules! output_method {
  ($f:expr) => { output_method ($f) };
  ($f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (FRAME_OUTPUT_METHOD_MASK << FRAME_OUTPUT_METHOD_SHIFT))
      | ((($v) as u32) << FRAME_OUTPUT_METHOD_SHIFT) };
}

/* Do we need the internal terminal?  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn internal_terminal_init ()            // C 1776-1777
{
  static mut init_needed: c_int = 1;
  let term: *mut c_char = getenv (c"TERM".as_ptr ());
  let mut colors: *mut c_char;
  let sf: *mut frame = SELECTED_FRAME ();
  let mut tty: *mut tty_display_info;

  /* C 1784-1787: `#ifdef HAVE_X_WINDOWS` - both arms (the `#else` arm is empty in C).  */
  #[cfg (feature = "have_x_windows")]
  {
  if !inhibit_window_system
    { return; }
  }

  /* If this is the initial terminal, we are done here.  */
  if FRAME_INITIAL_P (sf)
    { return; }

  internal_terminal
    = ((!noninteractive) && !term.is_null () && strcmp (term, c"internal".as_ptr ()) == 0) as c_int;

  /* C 1796-1870: `#ifndef HAVE_X_WINDOWS` - both arms (the `#else` arm is empty in C).  */
  #[cfg (not (feature = "have_x_windows"))]
  {
  if internal_terminal == 0 || inhibit_window_system
    {
      output_method! (sf, output_termcap);
      return;
    }

  tty = FRAME_TTY! (sf);
  kset_window_system (current_kboard, Qpc ());
  output_method! (sf, output_msdos_raw);
  if init_needed != 0
    {
      if (*tty).termscript.is_null () && !getenv (c"EMACSTEST".as_ptr ()).is_null ()
	{ (*tty).termscript = fopen (getenv (c"EMACSTEST".as_ptr ()), c"wt".as_ptr ()); }
      if !(*tty).termscript.is_null ()
	{
	  let mut now: time_t = time (std::ptr::null_mut ());
	  let tnow: *mut tm = localtime (&raw mut now);
	  let mut tbuf: [c_char; 100] = [0; 100];

	  strftime (&raw mut tbuf[0], countof! (tbuf) - 1,
		    c"%a %b %e %Y %H:%M:%S %Z".as_ptr (), tnow);
	  fprintf ((*tty).termscript, c"\nEmacs session started at %s\n".as_ptr (),
		   tbuf.as_mut_ptr ());
	  fprintf ((*tty).termscript,   c"=====================\n\n".as_ptr ());
	}

      Vinitial_window_system = Qpc ();
      (*(*tty).terminal).r#type = output_msdos_raw;

      /* If Emacs was dumped on DOS/V machine, forget the stale VRAM
	 address.  */
      screen_old_address = 0;

      /* Forget the stale screen colors as well.  */
      /* C's `initial_screen_colors[0] = initial_screen_colors[1] = -1;` is a chained
	 assignment; the port writes the two stores (the `haikuterm.rs` 1255 shape).  */
      initial_screen_colors[1] = -1;
      initial_screen_colors[0] = -1;

      FRAME_BACKGROUND_PIXEL! (SELECTED_FRAME ()) = 7; /* White */
      FRAME_FOREGROUND_PIXEL! (SELECTED_FRAME ()) = 0; /* Black */
      bright_bg ();
      colors = getenv (c"EMACSCOLORS".as_ptr ());
      if !colors.is_null () && strlen (colors) >= 2
	{
	  /* The colors use 4 bits each (we enable bright background).  */
	  if isdigit (*colors.offset (0) as c_int) != 0
	    { *colors.offset (0) -= b'0' as c_char; }
	  else if isxdigit (*colors.offset (0) as c_int) != 0
	    { *colors.offset (0) = (*colors.offset (0) as c_int
				   - ((if isupper (*colors.offset (0) as c_int) != 0
				       { b'A' } else { b'a' }) as c_int - 10)) as c_char; }
	  if (*colors.offset (0) as c_int) >= 0 && (*colors.offset (0) as c_int) < 16
	    { FRAME_FOREGROUND_PIXEL! (SELECTED_FRAME ()) = *colors.offset (0) as c_ulong; }
	  if isdigit (*colors.offset (1) as c_int) != 0
	    { *colors.offset (1) -= b'0' as c_char; }
	  else if isxdigit (*colors.offset (1) as c_int) != 0
	    { *colors.offset (1) = (*colors.offset (1) as c_int
				   - ((if isupper (*colors.offset (1) as c_int) != 0
				       { b'A' } else { b'a' }) as c_int - 10)) as c_char; }
	  if (*colors.offset (1) as c_int) >= 0 && (*colors.offset (1) as c_int) < 16
	    { FRAME_BACKGROUND_PIXEL! (SELECTED_FRAME ()) = *colors.offset (1) as c_ulong; }
	}

      reset_mouse_highlight (&raw mut the_only_display_info.mouse_highlight);

      if have_mouse != 0	/* detected in dos_ttraw, which see */
	{
	  have_mouse = 1;	/* enable mouse */
	  mouse_visible = 0;
	  mouse_setup_buttons (mouse_button_count);
	  (*(*tty).terminal).mouse_position_hook
	    = hook! (unsafe extern "C" fn (*mut *mut c_void, c_int, *mut Lisp_Object,
					   *mut scroll_bar_part, *mut Lisp_Object,
					   *mut Lisp_Object, *mut Time), mouse_get_pos);
	  mouse_init ();
	}

      if !(*tty).termscript.is_null () && screen_size != 0
	{ fprintf ((*tty).termscript, c"<SCREEN SAVED (dimensions=%dx%d)>\n".as_ptr (),
		   screen_size_X, screen_size_Y); }

      init_frame_faces (sf);
      init_needed = 0;
    }
  }
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn initialize_msdos_display (term: *mut terminal)   // C 1873-1874
{
  (*term).rif = std::ptr::null_mut ();	/* we don't support window-based display */
  /* C's chained assignment here is `cursor_to_hook` = (`raw_cursor_to_hook` = IT_cursor_to);
     the port writes the two stores (the `haikuterm.rs` 1255 shape).  */
  (*(term)).raw_cursor_to_hook
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int), IT_cursor_to);
  (*(term)).cursor_to_hook
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int), IT_cursor_to);
  (*term).clear_to_end_hook = hook! (unsafe extern "C" fn (*mut c_void), IT_clear_to_end);
  (*term).clear_frame_hook = hook! (unsafe extern "C" fn (*mut c_void), IT_clear_screen);
  (*term).clear_end_of_line_hook
    = hook! (unsafe extern "C" fn (*mut c_void, c_int), IT_clear_end_of_line);
  (*term).ins_del_lines_hook = None;
  (*term).insert_glyphs_hook
    = hook! (unsafe extern "C" fn (*mut c_void, *mut c_void, c_int), IT_insert_glyphs);
  (*term).write_glyphs_hook
    = hook! (unsafe extern "C" fn (*mut c_void, *mut c_void, c_int), IT_write_glyphs);
  (*term).delete_glyphs_hook
    = hook! (unsafe extern "C" fn (*mut c_void, c_int), IT_delete_glyphs);
  (*term).ring_bell_hook = hook! (unsafe extern "C" fn (*mut c_void), IT_ring_bell);
  (*term).reset_terminal_modes_hook
    = hook! (unsafe extern "C" fn (*mut terminal), IT_reset_terminal_modes);
  (*term).set_terminal_modes_hook
    = hook! (unsafe extern "C" fn (*mut terminal), IT_set_terminal_modes);
  (*term).set_terminal_window_hook = None;
  (*term).update_begin_hook = hook! (unsafe extern "C" fn (*mut c_void), IT_update_begin);
  (*term).update_end_hook = hook! (unsafe extern "C" fn (*mut c_void), IT_update_end);
  (*term).frame_up_to_date_hook
    = hook! (unsafe extern "C" fn (*mut c_void), IT_frame_up_to_date);
  (*term).mouse_position_hook = None; /* set later by dos_ttraw */
  (*term).menu_show_hook
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int, Lisp_Object,
				   *mut *const c_char) -> Lisp_Object, x_menu_show);
  (*term).frame_rehighlight_hook = None;
  (*term).frame_raise_lower_hook = None;
  (*term).set_vertical_scroll_bar_hook = None;
  (*term).condemn_scroll_bars_hook = None;
  (*term).redeem_scroll_bar_hook = None;
  (*term).judge_scroll_bars_hook = None;
  (*term).read_socket_hook
    = hook! (unsafe extern "C" fn (*mut terminal, *mut input_event) -> c_int, tty_read_avail_input);
                                                    /* from keyboard.c */
  (*term).defined_color_hook
    = hook! (unsafe extern "C" fn (*mut c_void, *const c_char, *mut Emacs_Color, bool, bool)
	     -> bool, tty_defined_color);           /* from xfaces.c */
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn dos_get_saved_screen (screen: *mut *mut c_char, rows: *mut c_int,
				    cols: *mut c_int) -> c_int   // C 1904-1905
{
  /* C 1907-1914: `#ifndef HAVE_X_WINDOWS` / `#else` / `#endif` - both arms.  */
  #[cfg (not (feature = "have_x_windows"))]
  {
  *screen = startup_screen_buffer as *mut c_char;
  *cols = startup_screen_size_X;
  *rows = startup_screen_size_Y;
  return (*screen != std::ptr::null_mut ()) as c_int;
  }
  #[cfg (feature = "have_x_windows")]
  {
  return 0;
  }
}

/* C 1916-1927: the keyboard-control banner.  C 1917 is a form feed (the reference's page
   separator); the coverage table's blank-line rule leaves it out and the port carries no line
   for it (one `## Deviations` entry).  */

/* ----------------------- Keyboard control ----------------------
 *
 * Keymaps reflect the following keyboard layout:
 *
 *    0  1  2  3  4  5  6  7  8  9  10 11 12  BS
 *    TAB 15 16 17 18 19 20 21 22 23 24 25 26 (41)
 *    CLOK 30 31 32 33 34 35 36 37 38 39 40 (41) RET
 *    SH () 45 46 47 48 49 50 51 52 53 54  SHIFT
 *                    SPACE
 */

/* C 1929-1940: the ten scan-code class bits (`#define`s in C, so the port carries them as
   `pub const` of C's own type; the `unsigned short` tables below cast each combination).  */
/// C: #define Ignore	0x0000
#[cfg (feature = "msdos")]
pub const Ignore: c_int = 0x0000;
/// C: #define Normal	0x0000	/* normal key - alt changes scan-code */
#[cfg (feature = "msdos")]
pub const Normal: c_int = 0x0000;	/* normal key - alt changes scan-code */
/// C: #define FctKey	0x1000	/* func key if c == 0, else c */
#[cfg (feature = "msdos")]
pub const FctKey: c_int = 0x1000;	/* func key if c == 0, else c */
/// C: #define Special	0x2000	/* func key even if c != 0 */
#[cfg (feature = "msdos")]
pub const Special: c_int = 0x2000;	/* func key even if c != 0 */
/// C: #define ModFct	0x3000	/* special if mod-keys, else 'c' */
#[cfg (feature = "msdos")]
pub const ModFct: c_int = 0x3000;	/* special if mod-keys, else 'c' */
/// C: #define Map	0x4000	/* alt scan-code, map to unshift/shift key */
#[cfg (feature = "msdos")]
pub const Map: c_int = 0x4000;	/* alt scan-code, map to unshift/shift key */
/// C: #define KeyPad	0x5000	/* map to insert/kp-0 depending on c == 0xe0 */
#[cfg (feature = "msdos")]
pub const KeyPad: c_int = 0x5000;	/* map to insert/kp-0 depending on c == 0xe0 */
/// C: #define Grey	0x6000	/* Grey keypad key */
#[cfg (feature = "msdos")]
pub const Grey: c_int = 0x6000;	/* Grey keypad key */
/// C: #define Alt	0x0100	/* alt scan-code */
#[cfg (feature = "msdos")]
pub const Alt: c_int = 0x0100;	/* alt scan-code */
/// C: #define Ctrl	0x0200	/* ctrl scan-code */
#[cfg (feature = "msdos")]
pub const Ctrl: c_int = 0x0200;	/* ctrl scan-code */
/// C: #define Shift	0x0400	/* shift scan-code */
#[cfg (feature = "msdos")]
pub const Shift: c_int = 0x0400;	/* shift scan-code */

/// C 1942: `static int extended_kbd; /* 101 (102) keyboard present. */`
#[cfg (feature = "msdos")]
static mut extended_kbd: c_int = 0;

/// C 1944-1948: `struct kbd_translate { unsigned char sc; unsigned char ch; unsigned short code; };`
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct kbd_translate {
  pub sc: c_uchar,
  pub ch: c_uchar,
  pub code: c_ushort,
}

/* C 1950-1956: `struct dos_keyboard_map`.  C spells the three string slots `char *`; the port
   writes `*const c_char` for a slot that only ever holds a literal (the `include/termchar.rs`
   178 `TS_orig_pair` reading) - one `## Deviations` entry.  */
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct dos_keyboard_map
{
  pub unshifted: *const c_char,
  pub shifted: *const c_char,
  pub alt_gr: *const c_char,
  pub translate_table: *mut kbd_translate,
}

/* C 1959-1967: `us_keyboard`.  */
#[cfg (feature = "msdos")]
static mut us_keyboard: dos_keyboard_map = dos_keyboard_map {
/* 0         1         2         3         4         5      */
/* 01234567890123456789012345678901234567890 123 45678901234 */
  unshifted: c"`1234567890-=  qwertyuiop[]   asdfghjkl;'\\  \\zxcvbnm,./  ".as_ptr (),
/* 0123456789012345678901234567890123456789 012345678901234 */
  shifted: c"~!@#$%^&*()_+  QWERTYUIOP{}   ASDFGHJKL:\"|  |ZXCVBNM<>?  ".as_ptr (),
  alt_gr: std::ptr::null (),			/* no Alt-Gr key */
  translate_table: std::ptr::null_mut ()	/* no translate table */
};

/* C 1969-1978: `fr_keyboard` - the file's non-ASCII text (DOS codepage 850; the file note at
   C 23-28 says why).  The port spells each byte as `\xNN`, so the port's own text stays ASCII
   and the bytes are C's exactly.  */
#[cfg (feature = "msdos")]
static mut fr_keyboard: dos_keyboard_map = dos_keyboard_map {
/* 0         1         2         3         4         5      */
/* 012 3456789012345678901234567890123456789012345678901234 */
  unshifted: c"\xfd&\x82\"'(-\x8a_\x80\x85)=  azertyuiop^$   qsdfghjklm\x97*  <wxcvbn,;:!  ".as_ptr (),
/* 0123456789012345678901234567890123456789012345678901234 */
  shifted: c" 1234567890\xf8+  AZERTYUIOP\xf9\x9c   QSDFGHJKLM%\xe6  >WXCVBN?./\xf5  ".as_ptr (),
/* 01234567 89012345678901234567890123456789012345678901234 */
  alt_gr: c"  ~#{[|`\\^@]}             \xcf                              ".as_ptr (),
  translate_table: std::ptr::null_mut ()	/* no translate table */
};

/*
 * Italian keyboard support, country code 39.
 * '<' 56:3c*0000
 * '>' 56:3e*0000
 * added also {,},` as, respectively, AltGr-8, AltGr-9, AltGr-'
 * Donated by Stefano Brozzi <brozzis@mag00.cedi.unipr.it>
 */

#[cfg (feature = "msdos")]
static mut it_kbd_translate_table: [kbd_translate; 3] = [
  kbd_translate { sc: 0x56, ch: 0x3c, code: (Normal | 13) as c_ushort },
  kbd_translate { sc: 0x56, ch: 0x3e, code: (Normal | 27) as c_ushort },
  kbd_translate { sc: 0, ch: 0, code: 0 }
];
#[cfg (feature = "msdos")]
static mut it_keyboard: dos_keyboard_map = dos_keyboard_map {
/* 0          1         2         3         4         5     */
/* 0 123456789012345678901234567890123456789012345678901234 */
  unshifted: c"\\1234567890'\x8d< qwertyuiop\x8a+>  asdfghjkl\x95\x85\x97  <zxcvbnm,.-  ".as_ptr (),
/* 01 23456789012345678901234567890123456789012345678901234 */
  shifted: c"|!\"\x9c$%&/()=?^> QWERTYUIOP\x82*   ASDFGHJKL\x87\xf8\xf5  >ZXCVBNM;:_  ".as_ptr (),
/* 0123456789012345678901234567890123456789012345678901234 */
  alt_gr: c"        {}~`             []             @#               ".as_ptr (),
  translate_table: unsafe { &raw mut it_kbd_translate_table[0] }
};

#[cfg (feature = "msdos")]
static mut dk_keyboard: dos_keyboard_map = dos_keyboard_map {
/* 0         1         2         3         4         5      */
/* 0123456789012345678901234567890123456789012345678901234 */
  unshifted: c"\xab1234567890+|  qwertyuiop\x86~   asdfghjkl\x91\x9b'  <zxcvbnm,.-  ".as_ptr (),
/* 01 23456789012345678901234567890123456789012345678901234 */
  shifted: c"\xf5!\"#$%&/()=?`  QWERTYUIOP\x8f^   ASDFGHJKL\x92\x9d*  >ZXCVBNM;:_  ".as_ptr (),
/* 0123456789012345678901234567890123456789012345678901234 */
  alt_gr: c"  @\x9c$  {[]} |                                             ".as_ptr (),
  translate_table: std::ptr::null_mut ()	/* no translate table */
};

#[cfg (feature = "msdos")]
static mut jp_kbd_translate_table: [kbd_translate; 7] = [
  kbd_translate { sc: 0x73, ch: 0x5c, code: (Normal | 0) as c_ushort },
  kbd_translate { sc: 0x73, ch: 0x5f, code: (Normal | 0) as c_ushort },
  kbd_translate { sc: 0x73, ch: 0x1c, code: (Map | 0) as c_ushort },
  kbd_translate { sc: 0x7d, ch: 0x5c, code: (Normal | 13) as c_ushort },
  kbd_translate { sc: 0x7d, ch: 0x7c, code: (Normal | 13) as c_ushort },
  kbd_translate { sc: 0x7d, ch: 0x1c, code: (Map | 13) as c_ushort },
  kbd_translate { sc: 0, ch: 0, code: 0 }
];
#[cfg (feature = "msdos")]
static mut jp_keyboard: dos_keyboard_map = dos_keyboard_map {
/*  0         1          2         3         4         5     */
/*  0123456789012 345678901234567890123456789012345678901234 */
  unshifted: c"\\1234567890-^\\ qwertyuiop@[   asdfghjkl;:]   zxcvbnm,./  ".as_ptr (),
/*  01 23456789012345678901234567890123456789012345678901234 */
  shifted: c"_!\"#$%&'()~=~| QWERTYUIOP`{   ASDFGHJKL+*}   ZXCVBNM<>?  ".as_ptr (),
  alt_gr: std::ptr::null (),			/* no Alt-Gr key */
  translate_table: unsafe { &raw mut jp_kbd_translate_table[0] }
};

/* C 2034-2045: the layout list.  C's `struct keyboard_layout_list` is a tag and the array of
   the same name; Rust keeps both names (they live in different namespaces).  */
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct keyboard_layout_list
{
  pub country_code: c_int,
  pub keyboard_map: *mut dos_keyboard_map,
}
#[cfg (feature = "msdos")]
static mut keyboard_layout_list: [keyboard_layout_list; 5] =
[
  keyboard_layout_list { country_code: 1, keyboard_map: &raw mut us_keyboard },
  keyboard_layout_list { country_code: 33, keyboard_map: &raw mut fr_keyboard },
  keyboard_layout_list { country_code: 39, keyboard_map: &raw mut it_keyboard },
  keyboard_layout_list { country_code: 45, keyboard_map: &raw mut dk_keyboard },
  keyboard_layout_list { country_code: 81, keyboard_map: &raw mut jp_keyboard }
];

/// C 2047: `static struct dos_keyboard_map *keyboard;`
#[cfg (feature = "msdos")]
static mut keyboard: *mut dos_keyboard_map = std::ptr::null_mut ();
/// C 2048: `static int keyboard_map_all;`
#[cfg (feature = "msdos")]
static mut keyboard_map_all: c_int = 0;
/// C 2049: `static int international_keyboard;`
#[cfg (feature = "msdos")]
static mut international_keyboard: c_int = 0;


/* C 2051-2080: `dos_set_keyboard`.  C's `_go32_dpmi_registers` block is declared in the C 57
   `<dpmi.h>` section above (P-88 ①).  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn dos_set_keyboard (code: c_int, always: c_int) -> c_int   // C 2051-2052
{
  let mut i: c_int;
  let mut regs: _go32_dpmi_registers = std::mem::zeroed ();

  /* See if Keyb.Com is installed (for international keyboard support).
     Note: calling Int 2Fh via int86 wedges the DOS box on some versions
     of Windows 9X!  So don't do that!  */
  regs.x.ax = 0xad80;
  /* C's `regs.x.ss = regs.x.sp = regs.x.flags = 0;` is a chained assignment; the port writes
     the three stores, innermost first (the `haikuterm.rs` 1255 shape).  */
  regs.x.flags = 0;
  regs.x.sp = 0;
  regs.x.ss = 0;
  _go32_dpmi_simulate_int (0x2f, &raw mut regs);
  if regs.h.al == 0xff
    { international_keyboard = 1; }

  /* Initialize to US settings, for countries that don't have their own.  */
  keyboard = keyboard_layout_list[0].keyboard_map;
  keyboard_map_all = always;
  dos_keyboard_layout = 1;

  i = 0;
  while i < countof! (keyboard_layout_list) as c_int	/* C's int-vs-size_t comparison */
    {
    if code == keyboard_layout_list[i as usize].country_code
      {
	keyboard = keyboard_layout_list[i as usize].keyboard_map;
	keyboard_map_all = always;
	dos_keyboard_layout = code as EMACS_INT;
	return 1;
      }
    i += 1;
    }
  return 0;
}

/* C 2082-2100: `keypad_translate_map`.  C's struct type is anonymous; the port names its
   element type (PORTING.md 1.3, the `src/charset.rs` 386 `charset_map_entry` shape) - one
   `## Deviations` entry.  */
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct keypad_translate_map_entry
{
	pub char_code: c_uchar,	/* normal code	*/
	pub meta_code: c_uchar,	/* M- code	*/
	pub keypad_code: c_uchar,	/* keypad code	*/
	pub editkey_code: c_uchar,	/* edit key	*/
}
#[cfg (feature = "msdos")]
static mut keypad_translate_map: [keypad_translate_map_entry; 11] = [
  keypad_translate_map_entry { char_code: b'0', meta_code: b'0', keypad_code: 0xb0, /* kp-0 */	editkey_code: 0x63 /* insert */ },
  keypad_translate_map_entry { char_code: b'1', meta_code: b'1', keypad_code: 0xb1, /* kp-1 */	editkey_code: 0x57 /* end */    },
  keypad_translate_map_entry { char_code: b'2', meta_code: b'2', keypad_code: 0xb2, /* kp-2 */	editkey_code: 0x54 /* down */   },
  keypad_translate_map_entry { char_code: b'3', meta_code: b'3', keypad_code: 0xb3, /* kp-3 */	editkey_code: 0x56 /* next */   },
  keypad_translate_map_entry { char_code: b'4', meta_code: b'4', keypad_code: 0xb4, /* kp-4 */	editkey_code: 0x51 /* left */   },
  keypad_translate_map_entry { char_code: b'5', meta_code: b'5', keypad_code: 0xb5, /* kp-5 */	editkey_code: 0xb5 /* kp-5 */   },
  keypad_translate_map_entry { char_code: b'6', meta_code: b'6', keypad_code: 0xb6, /* kp-6 */	editkey_code: 0x53 /* right */  },
  keypad_translate_map_entry { char_code: b'7', meta_code: b'7', keypad_code: 0xb7, /* kp-7 */	editkey_code: 0x50 /* home */   },
  keypad_translate_map_entry { char_code: b'8', meta_code: b'8', keypad_code: 0xb8, /* kp-8 */	editkey_code: 0x52 /* up */     },
  keypad_translate_map_entry { char_code: b'9', meta_code: b'9', keypad_code: 0xb9, /* kp-9 */	editkey_code: 0x55 /* prior */  },
  keypad_translate_map_entry { char_code: b'.', meta_code: b'-', keypad_code: 0xae, /* kp-decimal */	editkey_code: 0xff  /* delete */}
];

/* C 2102-2112: `grey_key_translate_map` (the same anonymous-struct shape; the element type is
   named here).  */
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct grey_key_translate_map_entry
{
	pub char_code: c_uchar,	/* normal code	*/
	pub keypad_code: c_uchar,	/* keypad code	*/
}
#[cfg (feature = "msdos")]
static mut grey_key_translate_map: [grey_key_translate_map_entry; 5] = [
  grey_key_translate_map_entry { char_code: b'/',  keypad_code: 0xaf /* kp-decimal */  },
  grey_key_translate_map_entry { char_code: b'*',  keypad_code: 0xaa /* kp-multiply */ },
  grey_key_translate_map_entry { char_code: b'-',  keypad_code: 0xad /* kp-subtract */ },
  grey_key_translate_map_entry { char_code: b'+',  keypad_code: 0xab /* kp-add */      },
  grey_key_translate_map_entry { char_code: b'\r', keypad_code: 0x8d  /* kp-enter */   }
];

/* C 2114-2305: `ibmpc_translate_map` - the file's one big table (167 entries, scan codes 00
   through a6).  C's element type is `unsigned short`, so every combination's `int` value
   needs C's implicit conversion spelled (`as c_ushort`) - one `## Deviations` entry.  */
#[cfg (feature = "msdos")]
static mut ibmpc_translate_map: [c_ushort; 167] =
[
  /* --------------- 00 to 0f --------------- */
  (Normal | 0xff) as c_ushort,	/* Ctrl Break + Alt-NNN */
  (Alt | ModFct | 0x1b) as c_ushort,	/* Escape */
  (Normal | 1) as c_ushort,	/* '1' */
  (Normal | 2) as c_ushort,	/* '2' */
  (Normal | 3) as c_ushort,	/* '3' */
  (Normal | 4) as c_ushort,	/* '4' */
  (Normal | 5) as c_ushort,	/* '5' */
  (Normal | 6) as c_ushort,	/* '6' */
  (Normal | 7) as c_ushort,	/* '7' */
  (Normal | 8) as c_ushort,	/* '8' */
  (Normal | 9) as c_ushort,	/* '9' */
  (Normal | 10) as c_ushort,	/* '0' */
  (Normal | 11) as c_ushort,	/* '-' */
  (Normal | 12) as c_ushort,	/* '=' */
  (Special | 0x08) as c_ushort,	/* Backspace */
  (ModFct | 0x74) as c_ushort,	/* Tab/Backtab */

  /* --------------- 10 to 1f --------------- */
  (Map | 15) as c_ushort,	/* 'q' */
  (Map | 16) as c_ushort,	/* 'w' */
  (Map | 17) as c_ushort,	/* 'e' */
  (Map | 18) as c_ushort,	/* 'r' */
  (Map | 19) as c_ushort,	/* 't' */
  (Map | 20) as c_ushort,	/* 'y' */
  (Map | 21) as c_ushort,	/* 'u' */
  (Map | 22) as c_ushort,	/* 'i' */
  (Map | 23) as c_ushort,	/* 'o' */
  (Map | 24) as c_ushort,	/* 'p' */
  (Map | 25) as c_ushort,	/* '[' */
  (Map | 26) as c_ushort,	/* ']' */
  (ModFct | 0x0d) as c_ushort,	/* Return */
  (Ignore) as c_ushort,	/* Ctrl */
  (Map | 30) as c_ushort,	/* 'a' */
  (Map | 31) as c_ushort,	/* 's' */

  /* --------------- 20 to 2f --------------- */
  (Map | 32) as c_ushort,	/* 'd' */
  (Map | 33) as c_ushort,	/* 'f' */
  (Map | 34) as c_ushort,	/* 'g' */
  (Map | 35) as c_ushort,	/* 'h' */
  (Map | 36) as c_ushort,	/* 'j' */
  (Map | 37) as c_ushort,	/* 'k' */
  (Map | 38) as c_ushort,	/* 'l' */
  (Map | 39) as c_ushort,	/* ';' */
  (Map | 40) as c_ushort,	/* '\'' */
  (Map |  0) as c_ushort,	/* '`' */
  (Ignore) as c_ushort,	/* Left shift */
  (Map | 41) as c_ushort,	/* '\\' */
  (Map | 45) as c_ushort,	/* 'z' */
  (Map | 46) as c_ushort,	/* 'x' */
  (Map | 47) as c_ushort,	/* 'c' */
  (Map | 48) as c_ushort,	/* 'v' */

  /* --------------- 30 to 3f --------------- */
  (Map | 49) as c_ushort,	/* 'b' */
  (Map | 50) as c_ushort,	/* 'n' */
  (Map | 51) as c_ushort,	/* 'm' */
  (Map | 52) as c_ushort,	/* ',' */
  (Map | 53) as c_ushort,	/* '.' */
  (Map | 54) as c_ushort,	/* '/' */
  (Ignore) as c_ushort,	/* Right shift */
  (Grey | 1) as c_ushort,	/* Grey * */
  (Ignore) as c_ushort,	/* Alt */
  (Normal | 55) as c_ushort,	/* ' ' */
  (Ignore) as c_ushort,	/* Caps Lock */
  (FctKey | 0xbe) as c_ushort,	/* F1 */
  (FctKey | 0xbf) as c_ushort,	/* F2 */
  (FctKey | 0xc0) as c_ushort,	/* F3 */
  (FctKey | 0xc1) as c_ushort,	/* F4 */
  (FctKey | 0xc2) as c_ushort,	/* F5 */

  /* --------------- 40 to 4f --------------- */
  (FctKey | 0xc3) as c_ushort,	/* F6 */
  (FctKey | 0xc4) as c_ushort,	/* F7 */
  (FctKey | 0xc5) as c_ushort,	/* F8 */
  (FctKey | 0xc6) as c_ushort,	/* F9 */
  (FctKey | 0xc7) as c_ushort,	/* F10 */
  (Ignore) as c_ushort,	/* Num Lock */
  (Ignore) as c_ushort,	/* Scroll Lock */
  (KeyPad | 7) as c_ushort,	/* Home */
  (KeyPad | 8) as c_ushort,	/* Up */
  (KeyPad | 9) as c_ushort,	/* Page Up */
  (Grey | 2) as c_ushort,	/* Grey - */
  (KeyPad | 4) as c_ushort,	/* Left */
  (KeyPad | 5) as c_ushort,	/* Keypad 5 */
  (KeyPad | 6) as c_ushort,	/* Right */
  (Grey | 3) as c_ushort,	/* Grey + */
  (KeyPad | 1) as c_ushort,	/* End */

  /* --------------- 50 to 5f --------------- */
  (KeyPad | 2) as c_ushort,	/* Down */
  (KeyPad | 3) as c_ushort,	/* Page Down */
  (KeyPad | 0) as c_ushort,	/* Insert */
  (KeyPad | 10) as c_ushort,	/* Delete */
  (Shift | FctKey | 0xbe) as c_ushort,	/* (Shift) F1 */
  (Shift | FctKey | 0xbf) as c_ushort,	/* (Shift) F2 */
  (Shift | FctKey | 0xc0) as c_ushort,	/* (Shift) F3 */
  (Shift | FctKey | 0xc1) as c_ushort,	/* (Shift) F4 */
  (Shift | FctKey | 0xc2) as c_ushort,	/* (Shift) F5 */
  (Shift | FctKey | 0xc3) as c_ushort,	/* (Shift) F6 */
  (Shift | FctKey | 0xc4) as c_ushort,	/* (Shift) F7 */
  (Shift | FctKey | 0xc5) as c_ushort,	/* (Shift) F8 */
  (Shift | FctKey | 0xc6) as c_ushort,	/* (Shift) F9 */
  (Shift | FctKey | 0xc7) as c_ushort,	/* (Shift) F10 */
  (Ctrl | FctKey | 0xbe) as c_ushort,	/* (Ctrl) F1 */
  (Ctrl | FctKey | 0xbf) as c_ushort,	/* (Ctrl) F2 */

  /* --------------- 60 to 6f --------------- */
  (Ctrl | FctKey | 0xc0) as c_ushort,	/* (Ctrl) F3 */
  (Ctrl | FctKey | 0xc1) as c_ushort,	/* (Ctrl) F4 */
  (Ctrl | FctKey | 0xc2) as c_ushort,	/* (Ctrl) F5 */
  (Ctrl | FctKey | 0xc3) as c_ushort,	/* (Ctrl) F6 */
  (Ctrl | FctKey | 0xc4) as c_ushort,	/* (Ctrl) F7 */
  (Ctrl | FctKey | 0xc5) as c_ushort,	/* (Ctrl) F8 */
  (Ctrl | FctKey | 0xc6) as c_ushort,	/* (Ctrl) F9 */
  (Ctrl | FctKey | 0xc7) as c_ushort,	/* (Ctrl) F10 */
  (Alt | FctKey | 0xbe) as c_ushort,	/* (Alt) F1 */
  (Alt | FctKey | 0xbf) as c_ushort,	/* (Alt) F2 */
  (Alt | FctKey | 0xc0) as c_ushort,	/* (Alt) F3 */
  (Alt | FctKey | 0xc1) as c_ushort,	/* (Alt) F4 */
  (Alt | FctKey | 0xc2) as c_ushort,	/* (Alt) F5 */
  (Alt | FctKey | 0xc3) as c_ushort,	/* (Alt) F6 */
  (Alt | FctKey | 0xc4) as c_ushort,	/* (Alt) F7 */
  (Alt | FctKey | 0xc5) as c_ushort,	/* (Alt) F8 */

  /* --------------- 70 to 7f --------------- */
  (Alt | FctKey | 0xc6) as c_ushort,	/* (Alt) F9 */
  (Alt | FctKey | 0xc7) as c_ushort,	/* (Alt) F10 */
  (Ctrl | FctKey | 0x6d) as c_ushort,	/* (Ctrl) Sys Rq */
  (Ctrl | KeyPad | 4) as c_ushort,	/* (Ctrl) Left */
  (Ctrl | KeyPad | 6) as c_ushort,	/* (Ctrl) Right */
  (Ctrl | KeyPad | 1) as c_ushort,	/* (Ctrl) End */
  (Ctrl | KeyPad | 3) as c_ushort,	/* (Ctrl) Page Down */
  (Ctrl | KeyPad | 7) as c_ushort,	/* (Ctrl) Home */
  (Alt | Map | 1) as c_ushort,	/* '1' */
  (Alt | Map | 2) as c_ushort,	/* '2' */
  (Alt | Map | 3) as c_ushort,	/* '3' */
  (Alt | Map | 4) as c_ushort,	/* '4' */
  (Alt | Map | 5) as c_ushort,	/* '5' */
  (Alt | Map | 6) as c_ushort,	/* '6' */
  (Alt | Map | 7) as c_ushort,	/* '7' */
  (Alt | Map | 8) as c_ushort,	/* '8' */

  /* --------------- 80 to 8f --------------- */
  (Alt | Map | 9) as c_ushort,	/* '9' */
  (Alt | Map | 10) as c_ushort,	/* '0' */
  (Alt | Map | 11) as c_ushort,	/* '-' */
  (Alt | Map | 12) as c_ushort,	/* '=' */
  (Ctrl | KeyPad | 9) as c_ushort,	/* (Ctrl) Page Up */
  (FctKey | 0xc8) as c_ushort,	/* F11 */
  (FctKey | 0xc9) as c_ushort,	/* F12 */
  (Shift | FctKey | 0xc8) as c_ushort,	/* (Shift) F11 */
  (Shift | FctKey | 0xc9) as c_ushort,	/* (Shift) F12 */
  (Ctrl | FctKey | 0xc8) as c_ushort,	/* (Ctrl) F11 */
  (Ctrl | FctKey | 0xc9) as c_ushort,	/* (Ctrl) F12 */
  (Alt | FctKey | 0xc8) as c_ushort,	/* (Alt) F11 */
  (Alt | FctKey | 0xc9) as c_ushort,	/* (Alt) F12 */
  (Ctrl | KeyPad | 8) as c_ushort,	/* (Ctrl) Up */
  (Ctrl | Grey | 2) as c_ushort,	/* (Ctrl) Grey - */
  (Ctrl | KeyPad | 5) as c_ushort,	/* (Ctrl) Keypad 5 */

  /* --------------- 90 to 9f --------------- */
  (Ctrl | Grey | 3) as c_ushort,	/* (Ctrl) Grey + */
  (Ctrl | KeyPad | 2) as c_ushort,	/* (Ctrl) Down */
  (Ctrl | KeyPad | 0) as c_ushort,	/* (Ctrl) Insert */
  (Ctrl | KeyPad | 10) as c_ushort,	/* (Ctrl) Delete */
  (Ctrl | FctKey | 0x09) as c_ushort,	/* (Ctrl) Tab */
  (Ctrl | Grey | 0) as c_ushort,	/* (Ctrl) Grey / */
  (Ctrl | Grey | 1) as c_ushort,	/* (Ctrl) Grey * */
  (Alt | FctKey | 0x50) as c_ushort,	/* (Alt) Home */
  (Alt | FctKey | 0x52) as c_ushort,	/* (Alt) Up */
  (Alt | FctKey | 0x55) as c_ushort,	/* (Alt) Page Up */
  (Ignore) as c_ushort,	/* NO KEY */
  (Alt | FctKey | 0x51) as c_ushort,	/* (Alt) Left */
  (Ignore) as c_ushort,	/* NO KEY */
  (Alt | FctKey | 0x53) as c_ushort,	/* (Alt) Right */
  (Ignore) as c_ushort,	/* NO KEY */
  (Alt | FctKey | 0x57) as c_ushort,	/* (Alt) End */

  /* --------------- a0 to af --------------- */
  (Alt | KeyPad | 2) as c_ushort,	/* (Alt) Down */
  (Alt | KeyPad | 3) as c_ushort,	/* (Alt) Page Down */
  (Alt | KeyPad | 0) as c_ushort,	/* (Alt) Insert */
  (Alt | KeyPad | 10) as c_ushort,	/* (Alt) Delete */
  (Alt | Grey | 0) as c_ushort,	/* (Alt) Grey / */
  (Alt | FctKey | 0x09) as c_ushort,	/* (Alt) Tab */
  (Alt | Grey | 4) as c_ushort,	/* (Alt) Keypad Enter */
];

/* C 2307-2316: the BIOS modifier bit positions.  */
#[cfg (feature = "msdos")]
pub const SHIFT_P: c_int = 0x0003;	/* two bits! */
#[cfg (feature = "msdos")]
pub const CTRL_P: c_int = 0x0004;
#[cfg (feature = "msdos")]
pub const ALT_P: c_int = 0x0008;
#[cfg (feature = "msdos")]
pub const SCRLOCK_P: c_int = 0x0010;
#[cfg (feature = "msdos")]
pub const NUMLOCK_P: c_int = 0x0020;
#[cfg (feature = "msdos")]
pub const CAPSLOCK_P: c_int = 0x0040;
#[cfg (feature = "msdos")]
pub const ALT_GR_P: c_int = 0x0800;
#[cfg (feature = "msdos")]
pub const SUPER_P: c_int = 0x4000;	/* pseudo */
#[cfg (feature = "msdos")]
pub const HYPER_P: c_int = 0x8000;	/* pseudo */


/* These bit-positions corresponds to values returned by BIOS */
#[cfg (feature = "msdos")]
unsafe fn dos_get_modifiers (keymask: *mut c_int) -> c_int   // C 2318-2319
{
  let mut regs: REGS = std::mem::zeroed ();
  let mut mask: c_int;
  let mut modifiers: c_int = 0;

  /* Calculate modifier bits */
  regs.h.ah = if extended_kbd != 0 { 0x12 } else { 0x02 };
  int86 (0x16, &raw mut regs, &raw mut regs);

  if extended_kbd == 0
    {
      mask = regs.h.al as c_int & (SHIFT_P | CTRL_P | ALT_P |
				   SCRLOCK_P | NUMLOCK_P | CAPSLOCK_P);
    }
  else
    {
      mask = regs.h.al as c_int & (SHIFT_P |
				   SCRLOCK_P | NUMLOCK_P | CAPSLOCK_P);

      /* Do not break international keyboard support.   */
      /* When Keyb.Com is loaded, the right Alt key is  */
      /* used for accessing characters like { and }  */
      if (regs.h.ah as c_int & 2) != 0		/* Left ALT pressed ? */
	{ mask |= ALT_P; }

      if (regs.h.ah as c_int & 8) != 0		/* Right ALT pressed ? */
	{
	  mask |= ALT_GR_P;
	  if dos_hyper_key == 1
	    {
	      mask |= HYPER_P;
	      modifiers |= hyper_modifier;
	    }
	  else if dos_super_key == 1
	    {
	      mask |= SUPER_P;
	      modifiers |= super_modifier;
	    }
	  else if international_keyboard == 0
	    {
	      /* If Keyb.Com is NOT installed, let Right Alt behave
		 like the Left Alt.  */
	      mask &= ! ALT_GR_P;
	      mask |= ALT_P;
	    }
	}

      if (regs.h.ah as c_int & 1) != 0		/* Left CTRL pressed ? */
	{ mask |= CTRL_P; }

      if (regs.h.ah as c_int & 4) != 0		/* Right CTRL pressed ? */
	{
	  if dos_hyper_key == 2
	    {
	      mask |= HYPER_P;
	      modifiers |= hyper_modifier;
	    }
	  else if dos_super_key == 2
	    {
	      mask |= SUPER_P;
	      modifiers |= super_modifier;
	    }
	  else
	    { mask |= CTRL_P; }
	}
    }

  if (mask & SHIFT_P) != 0
    { modifiers |= shift_modifier; }
  if (mask & CTRL_P) != 0
    { modifiers |= ctrl_modifier; }
  if (mask & ALT_P) != 0
    { modifiers |= meta_modifier; }

  if !keymask.is_null ()
    { *keymask = mask; }
  return modifiers;
}

/// C 2398: `#define NUM_RECENT_DOSKEYS (100)`
#[cfg (feature = "msdos")]
pub const NUM_RECENT_DOSKEYS: c_int = (100);
/// C 2399: `int recent_doskeys_index;	/* Index for storing next element into recent_doskeys */`
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub static mut recent_doskeys_index: c_int = 0;
/// C 2400: `int total_doskeys;		/* Total number of elements stored into recent_doskeys */`
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub static mut total_doskeys: c_int = 0;
/// C 2401: `Lisp_Object recent_doskeys;	/* A vector, holding the last 100 keystrokes */`
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub static mut recent_doskeys: Lisp_Object = unsafe { std::mem::zeroed () };

#[cfg (feature = "msdos")]
DEFUN! ("recent-doskeys", Frecent_doskeys, Srecent_doskeys, 0, 0, 0,
       doc: "Return vector of last 100 keyboard input values seen in dos_rawgetc.\nEach input key receives two values in this vector: first the ASCII code,\nand then the scan code.  ",   // C 2403-2406
       (),
       {
  let mut val: Lisp_Object;
  let keys: *mut Lisp_Object = (*XVECTOR (recent_doskeys)).contents.as_mut_ptr ();

  if total_doskeys < NUM_RECENT_DOSKEYS
    { return Fvector (total_doskeys as ptrdiff_t, keys); }
  else
    {
      val = Fvector (NUM_RECENT_DOSKEYS as ptrdiff_t, keys);
      vcopy (val, 0, keys.offset (recent_doskeys_index as isize),
	     (NUM_RECENT_DOSKEYS - recent_doskeys_index) as isize);
      vcopy (val, (NUM_RECENT_DOSKEYS - recent_doskeys_index) as isize,
	     keys, recent_doskeys_index as isize);
      return val;
    }
});

/* C 2423 is the blank line and C 2424-2426 the banner of `dos_rawgetc`, the next live
   definition; C 2423-4350 is the unported remainder (see `docs/ports/msdos.md`).  */
