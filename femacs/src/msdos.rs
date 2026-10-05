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
  let mut coding: *mut coding_system = FRAME_TERMINAL_CODING! (f);

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
