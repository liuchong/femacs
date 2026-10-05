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
/* `IS_DIRECTORY_SEP`/`IS_DEVICE_SEP` are `config.h` macros the *DOS* build defines - not by a
   header, but by `emacs/msdos/sed2v2.inp` 86-87, which rewrites the generated `src/config.h`
   (`#define IS_DIRECTORY_SEP(_c_) ((_c_) == '/' || (_c_) == '\\')` and
   `#define IS_DEVICE_SEP(_c_) ((_c_) == ':')`).  `include/config.rs` is the *frozen* target's
   value set, so the tree declares these two at their use sites instead (`src/callproc.rs`
   293-296's non-DOS pair, `src/buffer.rs` 4849-4853), and this unit carries the DOS values with
   their C source cited.  One `## Deviations` entry.  */
/// C: config.h (via `emacs/msdos/sed2v2.inp` 86): `#define IS_DIRECTORY_SEP(_c_) ((_c_) == '/' || (_c_) == '\\')`
#[cfg (feature = "msdos")]
#[inline(always)]
fn IS_DIRECTORY_SEP (_c_: c_int) -> bool { _c_ == b'/' as c_int || _c_ == b'\\' as c_int }
/// C: config.h (via `emacs/msdos/sed2v2.inp` 87): `#define IS_DEVICE_SEP(_c_) ((_c_) == ':')`
#[cfg (feature = "msdos")]
#[inline(always)]
fn IS_DEVICE_SEP (_c_: c_int) -> bool { _c_ == b':' as c_int }

/* C 34: `#ifdef MSDOS` - the file gate.  C 35 through the end of the file sit inside it, and
   every `include!` below carries it (P-86 ⑧); the C 20-30 notes and C 32's `<config.h>` are the
   only text outside it.  */

/* C 35's `#include <setjmp.h>` has no mirror of its own: `jmp_buf` and `sys_setjmp` are
   `include/lisp.rs`'s (C 36, the `src/thread.rs` 31-32 / `src/image.rs` 78-79 reading), and this
   unit reaches them through C 36.  */
/* Three `lisp.h` names no mirror in this unit's graph declares, declared here the way the tree's
   other units declare them (`src/emacs.rs` 242-243 and `src/keyboard.rs` 652-653 are the same
   three-row shape; `docs/ports/w32proc.md` Owed 3 counts this class).  */
/// C: lisp.h 5095: `extern bool detect_input_pending (void);`
/// C: lisp.h 5099: `extern void cmd_error_internal (Lisp_Object, const char *);`
/// C: lisp.h 5293: `extern void reset_all_sys_modes (void);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn detect_input_pending () -> bool;
  pub fn cmd_error_internal (data: Lisp_Object, context: *const c_char);
  pub fn reset_all_sys_modes ();
}

/* C 36's `lisp.h` 30 is `#include <string.h>`, and this unit's `strcmp` (C 1794) is that
   header's; the mirror does not carry the edge, so the unit spells it here (the C 36
   `<stdcountof.h>` edge below is the same shape - Deviations 13).  */
#[cfg (feature = "msdos")]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (`strcmp`)
/* The mirror declares `strlen`/`strcmp`/`strncmp`/`strcpy` but not `strcat`, which C 3465-3466
   needs; the tree declares that name at its use sites (`src/callproc.rs` 175,
   `src/pdumper.rs` 135, `src/xdisp.rs` 711), and the integrator's 3.449 ruling (this ledger's
   Owed 12) is that this unit does the same rather than waiting for the mirror.  */
/// C: <string.h> `extern char *strcat (char *, const char *);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn strcat (dest: *mut c_char, src: *const c_char) -> *mut c_char;
}
#[cfg (feature = "msdos")]
include!("include/lisp.rs");                   // C 36: `#include "lisp.h"`
#[cfg (feature = "msdos")]
include!("../lib/include/stdcountof.rs");      // lisp.h 28: `#include <stdcountof.h>` (`countof`)
#[cfg (feature = "msdos")]
include!("../lib/include/stdio.rs");           // C 37: `#include <stdio.h>` (`fprintf`, `fputc`, `FILE`)
#[cfg (feature = "msdos")]
include!("../lib/include/time.in.rs");         // C 38: `#include <time.h>` (`clock_t`)
/* The frozen SDK's `<time.h>` declares `clock_t` and `CLOCKS_PER_SEC` but **not** `clock`:
   measured here with `echo 'int main(void){return (int) clock();}' | clang -x c -` ->
   `error: call to undeclared function 'clock'` (the SDK dropped the legacy prototype).
   C's calls at 2251/4808 are the DJGPP arm's, so the prototype comes from DJGPP's
   `<time.h>` and is declared here (P-88 ①): `/// C: clock_t clock (void);`.  DJGPP's
   `clock_t` is unverifiable on this machine - **UNVERIFIED** (status 3.479).  */
unsafe extern "C" {
  pub fn clock () -> clock_t;
}
/* C 39's `#include <sys/param.h>` has no mirror; slice 3's two arrays (C 3457's `emacsroot`
   and C 3462's `result`) are its `MAXPATHLEN`, and the tree spells that constant at the use site
   with the measured value (the `src/sysdep.rs` 369-370 / `src/pdumper.rs` 218-219 reading).
   **UNVERIFIED** only in the sense that no `<sys/param.h>` is read here; the number is the
   frozen target's.  One `## Deviations` entry.  */
/// <sys/param.h>: `#define MAXPATHLEN 1024`
#[cfg (feature = "msdos")]
pub const MAXPATHLEN: usize = 1024;
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
   Every section below now carries the names its slices use; `<io.h>`'s section (C 56) is new in
   slice 4 and its `setmode` is the only name the file takes from that header.  ---------------- */
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
  pub cflag: c_ushort,                  // C 3666: `outregs.x.cflag` (slice 4)
  pub flags: c_ushort,                  // C 2442: `regs.x.flags` (slice 3)
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
/// C 45's `<dos.h>`: the interrupt entry points `int86`/`intdos`, `delay` (slice 3's
/// `dos_rawgetc`, C 2736), the Ctrl-Break pair `getcbrk`/`setcbrk` (slice 4's `dos_ttraw`,
/// C 3659-3660), `struct ftime` with `setftime` (slice 4's `futimens`, C 4012/4031) and the
/// `system` flags `__system_flags` and its six bits (slice 4's `run_msdos_command`, C 3823-3832).
/// None of them has a declaration anywhere in the reference tree (they are DJGPP runtime names,
/// like the `Screen*` family), so which DJGPP header supplies each is unmeasured; the section is
/// `<dos.h>` because that is where this unit's runtime names live.  **UNVERIFIED** for the same
/// missing toolchain (each return type is the call's own use).
/// C: int int86 (int, union REGS *, union REGS *);
/// C: int intdos (union REGS *, union REGS *);
/// C: void delay (unsigned);
/// C: int getcbrk (void); int setcbrk (int);
/// C: struct ftime { unsigned ft_tsec : 5, ft_min : 6, ft_hour : 5, ft_day : 5, ft_month : 4, ft_year : 7; };
/// C: int setftime (int, struct ftime *);
/// C: int __system_flags; (and the six `__system_*` bits below)
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn int86 (intno: c_int, inregs: *mut REGS, outregs: *mut REGS) -> c_int;
  pub fn intdos (inregs: *mut REGS, outregs: *mut REGS) -> c_int;
  pub fn delay (msec: c_uint);
  pub fn getcbrk () -> c_int;
  pub fn setcbrk (value: c_int) -> c_int;
  pub fn setftime (handle: c_int, time: *mut ftime) -> c_int;
}
/// C 45's `<dos.h>`: DJGPP's `struct ftime`, the bit-packed DOS time `futimens` fills.  The
/// port spells every member as C's bit-fields' shared byte (PORTING.md 1.3) - **UNVERIFIED**,
/// and only the six fields C 4023-4028 writes are carried.
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct ftime
{
  pub bits: c_ushort,	/* ft_tsec:5 | ft_min:6 | ft_hour:5 */
  pub bits2: c_ushort,	/* ft_day:5 | ft_month:4 | ft_year:7 */
}
/* C 4023-4028's six writes are `ftime`'s bit-fields; the port keeps C's member names in a
   per-unit accessor pair (the `mouse_moved!` shape), with the widths the header declares
   (`ft_tsec : 5`, `ft_min : 6`, `ft_hour : 5`, `ft_day : 5`, `ft_month : 4`, `ft_year : 7`).  */
#[cfg (feature = "msdos")]
macro_rules! ftime_bits {
  (tsec, $f:expr) => { (*$f).bits & 0x1f };
  (tsec, $f:expr, $v:expr) => { (*$f).bits = ((*$f).bits & !0x1fu16) | (($v as u16) & 0x1f) };
  (min, $f:expr) => { ((*$f).bits >> 5) & 0x3f };
  (min, $f:expr, $v:expr) => {
    (*$f).bits = ((*$f).bits & !(0x3fu16 << 5)) | ((($v as u16) & 0x3f) << 5) };
  (hour, $f:expr) => { ((*$f).bits >> 11) & 0x1f };
  (hour, $f:expr, $v:expr) => {
    (*$f).bits = ((*$f).bits & !(0x1fu16 << 11)) | ((($v as u16) & 0x1f) << 11) };
  (day, $f:expr) => { (*$f).bits2 & 0x1f };
  (day, $f:expr, $v:expr) => { (*$f).bits2 = ((*$f).bits2 & !0x1fu16) | (($v as u16) & 0x1f) };
  (month, $f:expr) => { ((*$f).bits2 >> 5) & 0x0f };
  (month, $f:expr, $v:expr) => {
    (*$f).bits2 = ((*$f).bits2 & !(0x0fu16 << 5)) | ((($v as u16) & 0x0f) << 5) };
  (year, $f:expr) => { ((*$f).bits2 >> 9) & 0x7f };
  (year, $f:expr, $v:expr) => {
    (*$f).bits2 = ((*$f).bits2 & !(0x7fu16 << 9)) | ((($v as u16) & 0x7f) << 9) };
}
/// C 45's `<dos.h>`: the six `__system_*` bits `run_msdos_command` ORs together, and the flag
/// word they go into (C 3823-3832).  **UNVERIFIED** (no DJGPP header); the values are not
/// readings and the ledger owes them.
/// C: int __system_flags = __system_redirect | __system_use_shell | __system_allow_multiple_cmds | …;
#[cfg (feature = "msdos")]
pub const __system_redirect: c_int = 0;
#[cfg (feature = "msdos")]
pub const __system_use_shell: c_int = 0;
#[cfg (feature = "msdos")]
pub const __system_allow_multiple_cmds: c_int = 0;
#[cfg (feature = "msdos")]
pub const __system_allow_long_cmds: c_int = 0;
#[cfg (feature = "msdos")]
pub const __system_handle_null_commands: c_int = 0;
#[cfg (feature = "msdos")]
pub const __system_emulate_chdir: c_int = 0;
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub static mut __system_flags: c_int;
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
/// C 57's `<dpmi.h>`: the cooperative yield `XMenuActivate` busy-waits with (C 3225/3235).
/// **UNVERIFIED** for the same missing toolchain; the return type is the call's own use (a
/// statement).
/// C: void __dpmi_yield (void);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn __dpmi_yield ();
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
/* C 59's `<libc/dosio.h>` (`/* for _USE_LFN */`): the only name this unit takes from it is the
   `_USE_LFN` *test* at C 95, which is carried at C 95 below.  */
// C 60: `#include <conio.h>` (`/* for cputs */`) - the header this section stands for.
/// C 60's `<conio.h>`: `cputs`, which `msdos_abort` writes its message with (C 4306).
/// **UNVERIFIED** (no DJGPP toolchain); the return type is the call's own use (a statement).
/// C: int cputs (const char *);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn cputs (string: *const c_char) -> c_int;
}

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
/* Slice 2 took the *system* half (`stdlib.rs`, `getenv`).  Slice 4 needs gnulib's own additions
   `setenv` (C 3519) and `putenv` (C 4151), and the natural edge would be `stdlib.in.rs` - but
   that mirror has never been compiled by any unit on this revision and names two types nothing
   in its scope supplies (`c_float`/`c_longlong`, its 235/240), so the measured choice is to keep
   this edge and declare the two names here, exactly as `src/android.rs` 1369 does for `setenv`
   (the "declared at its use site" class, `docs/ports/w32proc.md`'s Owed 3).  The mirror's own
   two unresolved types are recorded in this ledger's Owed 5.  One `## Deviations` entry.  */
#[cfg (feature = "msdos")]
include!("../lib/include/stdlib.rs");          // unistd.in.h 122-125: `#include <stdlib.h>`
/* `system` (C 3835) and `exit` (C 3881/4308) are `<stdlib.h>`'s and no mirror in this tree
   declares either; the tree declares them at their use sites (`src/sysdep.rs` 776/954 and the
   twelve `exit` declarations `docs/ports/w32proc.md`'s Owed 3 counts), so this unit does too.  */
/// C: <stdlib.h> `extern int system (const char *);`
/// C: <stdlib.h> `_Noreturn void exit (int);`
/// C: <stdlib.h> `extern int setenv (const char *, const char *, int);`
/// C: <stdlib.h> `extern int putenv (char *);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn system (command: *const c_char) -> c_int;
  pub fn exit (status: c_int) -> !;
  pub fn setenv (name: *const c_char, value: *const c_char, replace: c_int) -> c_int;
  pub fn putenv (string: *mut c_char) -> c_int;
}
// C 51: `#include <dir.h>` (`/* for getdisk */`) - the header this section stands for.
/// C 51's `<dir.h>`: `getdisk` (C 3377) and `_fixpath` (C 3360).  **UNVERIFIED**: no DJGPP
/// toolchain, so neither prototype is read from a header; the argument types follow this file's
/// own call sites (`_fixpath`'s first argument is `in_path`, a `char[4]`).
/// C 51's `<dir.h>`: `setdisk` (slice 4's `sys_chdir`, C 4265) and the `access` mode `D_OK`
/// (slice 4's `faccessat`, C 4094, and `init_environment`, C 3517).  **UNVERIFIED** with the
/// same toolchain note as the two above; C's `<dir.h>` is also where DJGPP puts `D_OK`'s
/// sibling `X_OK`/`W_OK`/`R_OK`, which this file does not use.
/// C: int getdisk (void); int _fixpath (const char *, char *); int setdisk (int);
/// C: #define D_OK 0x10
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn getdisk () -> c_int;
  pub fn _fixpath (path: *const c_char, resolved: *mut c_char) -> c_int;
  pub fn setdisk (drive: c_int) -> c_int;
}
/// C 51's `<dir.h>`: `D_OK`.  C's value is the header's; **UNVERIFIED** (no DJGPP header).
#[cfg (feature = "msdos")]
pub const D_OK: c_int = 0x10;
/* C 52's `#pragma pack(0)` and C 53's `#undef opendir` still have no counterpart: the pragma has
   none in Rust and neither does the `#undef` (the port never spells `opendir` as `<dirent.h>`'s
   macro).  Two `## Deviations` entries.  */
#[cfg (feature = "msdos")]
include!("../lib/include/dirent.in.rs");       // C 54: `#include <dirent.h>`
#[cfg (feature = "msdos")]
include!("../lib/include/fcntl.in.rs");        // C 55: `#include <fcntl.h>`
// C 56: `#include <io.h>` (`/* for setmode */`) - the header this section stands for.
/// C 56's `<io.h>`: `setmode`, which `dos_ttraw`/`dos_ttcooked` call (C 3699-3726).
/// **UNVERIFIED** (no DJGPP toolchain).
/// C: int setmode (int, int);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn setmode (handle: c_int, mode: c_int) -> c_int;
}

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
/* The two objects the arms above name are the DJGPP runtime's (`<stdlib.h>`'s `_environ` and
   its 2.3-era `environ`), and C 3822-3837's `run_msdos_command` reads and writes whichever the
   arm selected - so slice 4 is the first text that reaches them.  **UNVERIFIED** (no DJGPP
   toolchain); only the arm the default feature set selects is declared, because the mirror's
   `unistd.in.rs` carries the *frozen* target's `environ` (its 87-90 macOS arm).  */
/// C: extern char **_environ;
#[cfg (feature = "msdos")]
#[cfg (not (feature = "djgpp_2_3"))]
unsafe extern "C" {
  pub static mut _environ: *mut *mut c_char;
}
/// C: extern char **environ; (the 2.3-and-older arm)
#[cfg (feature = "msdos")]
#[cfg (feature = "djgpp_2_3")]
unsafe extern "C" {
  pub static mut environ: *mut *mut c_char;
}

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
/// C: int ScreenMode (void); void ScreenRetrieve (void *); void ScreenUpdate (void *);
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
  pub fn ScreenUpdate (memory: *mut c_void);
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

/* ----------------------------------------------------------------------------------------------
   C 2423-3468: slice 3 — `dos_rawgetc`, the key-availability pair, the whole MS-DOS X menu
   simulation (C 2787's `#ifndef HAVE_X_WINDOWS` block) and the DOS/UNIX conversion layer.
   C 2423 is `dos_rawgetc`'s blank line and C 2424 its banner; C 2425-2759 is the function.
   ---------------------------------------------------------------------------------------------- */

/* `input_event`'s `kind : 16` shares the mirror's `bits` word with `part : 16`
   (`include/termhooks.rs` 100-117, PORTING.md 1.3).  C writes `event.kind = x;`, so the port
   keeps C's member name in a per-unit macro — the `src/androidterm.rs` 201-210 /
   `src/androidselect.rs` 136-143 shape, body-identical.  */
#[cfg (feature = "msdos")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/* `Mouse_HLInfo`'s third flag bit — `dispextern.h` 3023's `bool_bf mouse_face_hidden : 1`, the
   `dispextern.rs` 2082 `MOUSE_FACE_HIDDEN` mask.  The mirror carries the `mouse_face_defer` pair
   but not this one (its own 2084-2095 note records that `docs/ports/w32term.md`'s owed 5 asks for
   it), and C 2640-2643/2671-2673 read and write the member directly, so this unit names the pair
   the way its `mouse_moved!` does.  One `## Deviations` entry.  */
#[cfg (feature = "msdos")]
macro_rules! mouse_face_hidden {
  ($h:expr) => { (*$h).bits & MOUSE_FACE_HIDDEN != 0 };
  ($h:expr, $v:expr) => {
    (*$h).bits = if ($v) as c_int != 0 { (*$h).bits | MOUSE_FACE_HIDDEN }
		 else { (*$h).bits & ! MOUSE_FACE_HIDDEN } };
}

/* Get a char from keyboard.  Function keys are put into the event queue.  */
#[cfg (feature = "msdos")]
unsafe fn dos_rawgetc () -> c_int                  // C 2425-2426
{
  let mut event: input_event = std::mem::zeroed ();
  let mut regs: REGS = std::mem::zeroed ();
  let hlinfo: *mut Mouse_HLInfo = MOUSE_HL_INFO! (SELECTED_FRAME ());
  EVENT_INIT (&raw mut event);

  /* C 2433-2436: `#ifndef HAVE_X_WINDOWS` - both arms (the `#else` arm is empty in C).  */
  #[cfg (not (feature = "have_x_windows"))]
  {
  /* Maybe put the cursor where it should be.  */
  IT_cmgoto (SELECTED_FRAME ());
  }

  /* The following condition is equivalent to `kbhit ()', except that
     it uses the bios to do its job.  This pleases DESQview/X.  */
  'dos_rawgetc_keys: while { regs.h.ah = if extended_kbd != 0 { 0x11 } else { 0x01 };
			     int86 (0x16, &raw mut regs, &raw mut regs);
			     (regs.x.flags & 0x40) == 0 }
    {
      let mut regs: REGS = std::mem::zeroed ();
      let mut c: c_uchar;
      let mut modifiers: c_int;
      let mut sc: c_int;
      let mut code: c_int = -1;
      let mut mask: c_int;
      let mut kp_mode: c_int;

      regs.h.ah = if extended_kbd != 0 { 0x10 } else { 0x00 };
      int86 (0x16, &raw mut regs, &raw mut regs);
      c = regs.h.al;
      sc = regs.h.ah as c_int;

      total_doskeys += 2;
      ASET (recent_doskeys, recent_doskeys_index as isize, make_fixnum (c as EMACS_INT));
      recent_doskeys_index+= 1;
      if recent_doskeys_index == NUM_RECENT_DOSKEYS
	{ recent_doskeys_index = 0; }
      ASET (recent_doskeys, recent_doskeys_index as isize, make_fixnum (sc as EMACS_INT));
      recent_doskeys_index+= 1;
      if recent_doskeys_index == NUM_RECENT_DOSKEYS
	{ recent_doskeys_index = 0; }

      modifiers = dos_get_modifiers (&raw mut mask);

      /* C 2465-2473: `#ifndef HAVE_X_WINDOWS` - both arms (the `#else` arm is empty in C).  */
      #[cfg (not (feature = "have_x_windows"))]
      {
      if !NILP (Vdos_display_scancodes)
	{
	  let mut buf: [c_char; 11] = [0; 11];
	  sprintf (&raw mut buf[0], c"%02x:%02x*%04x".as_ptr (),
		   (sc & 0xff) as c_uint, c as c_uint, mask as c_uint);
	  dos_direct_output (screen_size_Y - 2, screen_size_X - 12, &raw mut buf[0], 10);
	}
      }

      if sc == 0xe0
	{
	  match c
	    {
	    10 =>		/* Ctrl Grey Enter */
	      { code = Ctrl | Grey | 4; }
	    13 =>		/* Grey Enter */
	      { code = Grey | 4; }
	    b'/' =>		/* Grey / */
	      { code = Grey | 0; }
	    _ =>
	      { continue 'dos_rawgetc_keys; }
	    }
	  c = 0;
	}
      else
	{
	  /* Try the keyboard-private translation table first.  */
	  if !(*keyboard).translate_table.is_null ()
	    {
	      let mut p: *mut kbd_translate = (*keyboard).translate_table;

	      while (*p).sc != 0
		{
		  if (*p).sc as c_int == sc && (*p).ch as c_int == c as c_int
		    {
		      code = (*p).code as c_int;
		      break;
		    }
		  p = p.offset (1);
		}
	    }
	  /* If the private table didn't translate it, use the general
             one.  */
	  if code == -1
	    {
	      /* C's `sc >= countof (…)` is an int-vs-size_t comparison (Deviations 20).  */
	      if sc as usize >= countof! (ibmpc_translate_map)
		{ continue 'dos_rawgetc_keys; }
	      /* C's `(code = ibmpc_translate_map[sc]) == Ignore`, hoisted (Deviations 6).  */
	      code = ibmpc_translate_map[sc as usize] as c_int;
	      if code == Ignore
		{ continue 'dos_rawgetc_keys; }
	    }
	}

      if c == 0
	{
        /* We only look at the keyboard Ctrl/Shift/Alt keys when
           Emacs is ready to read a key.  Therefore, if they press
           `Alt-x' when Emacs is busy, by the time we get to
           `dos_get_modifiers', they might have already released the
           Alt key, and Emacs gets just `x', which is BAD.
           However, for keys with the `Map' property set, the ASCII
           code returns zero only if Alt is pressed.  So, when we DON'T
           have to support international_keyboard, we don't have to
           distinguish between the left and  right Alt keys, and we
           can set the META modifier for any keys with the `Map'
           property if they return zero ASCII code (c = 0).  */
        if (code & Alt) != 0
	     || ((code & 0xf000) == Map && international_keyboard == 0)
	    { modifiers |= meta_modifier; }
	  if (code & Ctrl) != 0
	    { modifiers |= ctrl_modifier; }
	  if (code & Shift) != 0
	    { modifiers |= shift_modifier; }
	}

      /* C 2543-2635's `switch (code & 0xf000)` falls through twice — `ModFct` into `FctKey`
	 into `Special`, and `Normal` into `Map` — which `match` cannot express; the port keeps
	 C's order with a fall-through state (the `src/ccl.rs` 1734 shape) and spells C's
	 `continue` with the loop label above (`src/bytecode.rs` 835).  C's `break` out of the
	 switch is this inner loop's `break`.  */
      let mut sw: c_int = code & 0xf000;
      loop
	{
	  match sw
	    {
	    ModFct =>
	      {
		if c != 0 && (mask & (SHIFT_P | ALT_P | CTRL_P | HYPER_P | SUPER_P)) == 0
		  { return c as c_int; }
		c = 0;		/* Special */
		sw = FctKey;
	      }
	    FctKey =>
	      {
		if c != 0
		  { return c as c_int; }
		sw = Special;
	      }
	    Special =>
	      {
		code |= 0xff00;
		break;
	      }
	    Normal =>
	      {
		if sc == 0
		  {
		    if c == 0	/* ctrl-break */
		      { continue 'dos_rawgetc_keys; }
		    return c as c_int;		/* ALT-nnn */
		  }
		if keyboard_map_all == 0
		  {
		    if c != b' '
		      { return c as c_int; }
		    code = c as c_int;
		    break;
		  }
		sw = Map;
	      }
	    Map =>
	      {
		if c != 0 && (mask & ALT_P) == 0
		    && !((mask & SHIFT_P) != 0 && (mask & CTRL_P) != 0)
		  {
		    if keyboard_map_all == 0
		      { return c as c_int; }
		  }

		code &= 0xff;
		if (mask & ALT_P) != 0 && code <= 10 && code > 0 && (dos_keypad_mode & 0x200) != 0
		  { mask |= SHIFT_P; }	/* ALT-1 => M-! etc. */

		if (mask & SHIFT_P) != 0
		  {
		    code = *(*keyboard).shifted.offset (code as isize) as c_int;
		    mask -= SHIFT_P;
		    modifiers &= !shift_modifier;
		  }
		else if (mask & ALT_GR_P) != 0 && !(*keyboard).alt_gr.is_null ()
			&& *(*keyboard).alt_gr.offset (code as isize) != b' ' as c_char
		  { code = *(*keyboard).alt_gr.offset (code as isize) as c_int; }
		else
		  { code = *(*keyboard).unshifted.offset (code as isize) as c_int; }
		break;
	      }
	    KeyPad =>
	      {
		code &= 0xff;
		if c == 0xe0	/* edit key */
		  { kp_mode = 3; }
		else if (mask & (NUMLOCK_P|CTRL_P|SHIFT_P|ALT_P)) == NUMLOCK_P /* numlock on */
		  { kp_mode = (dos_keypad_mode & 0x03) as c_int; }
		else
		  { kp_mode = ((dos_keypad_mode >> 4) & 0x03) as c_int; }

		match kp_mode
		  {
		  0 =>
		    {
		      if code == 10 && dos_decimal_point != 0
			{ return dos_decimal_point as c_int; }
		      return keypad_translate_map[code as usize].char_code as c_int;
		    }
		  1 =>
		    {
		      code = 0xff00 | keypad_translate_map[code as usize].keypad_code as c_int;
		    }
		  2 =>
		    {
		      code = keypad_translate_map[code as usize].meta_code as c_int;
		      modifiers = meta_modifier;
		    }
		  3 =>
		    {
		      code = 0xff00 | keypad_translate_map[code as usize].editkey_code as c_int;
		    }
		  _ => {}
		  }
		break;
	      }
	    Grey =>
	      {
		code &= 0xff;
		kp_mode = if (mask & (NUMLOCK_P|CTRL_P|SHIFT_P|ALT_P)) == NUMLOCK_P
			  { 0x04 } else { 0x40 };
		if (dos_keypad_mode & kp_mode as EMACS_INT) != 0
		  { code = 0xff00 | grey_key_translate_map[code as usize].keypad_code as c_int; }
		else
		  { code = grey_key_translate_map[code as usize].char_code as c_int; }
		break;
	      }
	    _ =>
	      { break; }
	    }
	}

      if code == 0
	{ continue 'dos_rawgetc_keys; }

      if !mouse_face_hidden! (hlinfo) && FIXNUMP (Vmouse_highlight)
	{
	  clear_mouse_face (hlinfo);
	  mouse_face_hidden! (hlinfo, 1);
	}

      if code >= 0x100
	{ input_event_bits! (kind, &raw mut event, NON_ASCII_KEYSTROKE_EVENT); }
      else
	{ input_event_bits! (kind, &raw mut event, ASCII_KEYSTROKE_EVENT); }
      event.code = code as c_uint;
      event.modifiers =	modifiers as c_uint;
      event.frame_or_window = selected_frame;
      event.arg = Qnil ();
      event.timestamp = event_timestamp ();
      kbd_buffer_store_event (&raw mut event as *mut c_void);
    }

  if have_mouse > 0 && mouse_preempted == 0
    {
      let mut but: c_int;
      let mut press: c_int;
      let mut x: c_int;
      let mut y: c_int;
      let mut ok: c_int;
      let mouse_prev_x: c_int = mouse_last_x;
      let mouse_prev_y: c_int = mouse_last_y;
      let mut mouse_window: Lisp_Object = Qnil ();

      /* Check for mouse movement *before* buttons.  */
      mouse_check_moved ();

      /* If the mouse moved from the spot of its last sighting, we
         might need to update mouse highlight.  */
      if mouse_last_x != mouse_prev_x || mouse_last_y != mouse_prev_y
	{
	  if mouse_face_hidden! (hlinfo)
	    {
	      mouse_face_hidden! (hlinfo, 0);
	      clear_mouse_face (hlinfo);
	    }

	  /* Generate SELECT_WINDOW_EVENTs when needed.  */
	  if !NILP (Vmouse_autoselect_window)
	    {
	      mouse_window = window_from_coordinates
		(SELECTED_FRAME () as *mut c_void, mouse_last_x, mouse_last_y,
		 std::ptr::null_mut (), false, false, false);
	      /* A window will be selected only when it is not
		 selected now, and the last mouse movement event was
		 not in it.  A minibuffer window will be selected iff
		 it is active.  */
	      if WINDOWP (mouse_window)
		  && !EQ (mouse_window, last_mouse_window)
		  && !EQ (mouse_window, selected_window)
		{
		  input_event_bits! (kind, &raw mut event, SELECT_WINDOW_EVENT);
		  event.frame_or_window = mouse_window;
		  event.arg = Qnil ();
		  event.timestamp = event_timestamp ();
		  kbd_buffer_store_event (&raw mut event as *mut c_void);
		}
	      /* Remember the last window where we saw the mouse.  */
	      last_mouse_window = mouse_window;
	    }

	  previous_help_echo_string = help_echo_string;
	  help_echo_string = Qnil ();
	  help_echo_object = Qnil ();
	  help_echo_window = Qnil ();
	  help_echo_pos = -1;
	  note_mouse_highlight (SELECTED_FRAME (), mouse_last_x, mouse_last_y);
	  /* If the contents of the global variable help_echo has
	     changed, generate a HELP_EVENT.  */
	  if !NILP (help_echo_string) || !NILP (previous_help_echo_string)
	    { gen_help_event (help_echo_string, selected_frame, help_echo_window,
			      help_echo_object, help_echo_pos); }
	}

      but = 0;
      while but < NUM_MOUSE_BUTTONS
	{
	  press = 0;
	  while press < 2
	  {
	    let mut button_num: c_int = but;

	    if press != 0
	      { ok = mouse_pressed (but, &raw mut x, &raw mut y); }
	    else
	      { ok = mouse_released (but, &raw mut x, &raw mut y); }
	    if ok != 0
	      {
		/* Allow a simultaneous press/release of Mouse-1 and
		   Mouse-2 to simulate Mouse-3 on two-button mice.  */
		if mouse_button_count == 2 && but < 2
		  {
		    let mut x2: c_int;
		    let mut y2: c_int;	/* don't clobber original coordinates */

		    /* If only one button is pressed, wait 100 msec and
		       check again.  This way, Speedy Gonzales isn't
		       punished, while the slow get their chance.  */
		    if (press != 0 && mouse_pressed (1-but, &raw mut x2, &raw mut y2) != 0)
			|| (press == 0 && mouse_released (1-but, &raw mut x2, &raw mut y2) != 0)
		      { button_num = 2; }
		    else
		      {
			delay (100);
			if (press != 0 && mouse_pressed (1-but, &raw mut x2, &raw mut y2) != 0)
			    || (press == 0 && mouse_released (1-but, &raw mut x2, &raw mut y2) != 0)
			  { button_num = 2; }
		      }
		  }

		input_event_bits! (kind, &raw mut event, MOUSE_CLICK_EVENT);
		event.code = button_num as c_uint;
		event.modifiers = (dos_get_modifiers (std::ptr::null_mut ())
				   | if press != 0 { down_modifier } else { up_modifier })
				  as c_uint;
		event.x = make_fixnum (x as EMACS_INT);
		event.y = make_fixnum (y as EMACS_INT);
		event.frame_or_window = selected_frame;
		event.arg = tty_handle_tab_bar_click (SELECTED_FRAME (), x, y, press != 0,
						      &raw mut event);
		event.timestamp = event_timestamp ();
		kbd_buffer_store_event (&raw mut event as *mut c_void);
	      }
	    press += 1;
	  }
	  but += 1;
	}
    }

  return -1;
}

#[cfg (feature = "msdos")]
static mut prev_get_char: c_int = -1;	/* C 2761: `static int prev_get_char = -1;` */

/* Return 1 if a key is ready to be read without suspending execution.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn dos_keysns () -> c_int               // C 2763-2765
{
  if prev_get_char != -1
    { return 1; }
  else
    {
      /* C's `((prev_get_char = dos_rawgetc ()) != -1)` hoists the store (Deviations 6).  */
      prev_get_char = dos_rawgetc ();
      return (prev_get_char != -1) as c_int;
    }
}

/* Read a key.  Return -1 if no key is ready.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn dos_keyread () -> c_int              // C 2773-2775
{
  if prev_get_char != -1
    {
      let c: c_int = prev_get_char;
      prev_get_char = -1;
      return c;
    }
  else
    { return dos_rawgetc (); }
}

/* C 2787: `#ifndef HAVE_X_WINDOWS` - the MS-DOS menu simulation, whose C 3303 `#endif` closes
   it.  Every item below carries `not (feature = "have_x_windows")` on top of the file gate.  */

/* Simulation of X's menus.  Nothing too fancy here -- just make it work
   for now.

   Actually, I don't know the meaning of all the parameters of the functions
   here -- I only know how they are called by xmenu.c.  I could of course
   grab the nearest Xlib manual (down the hall, second-to-last door on the
   left), but I don't think it's worth the effort.  */

/* These hold text of the current and the previous menu help messages.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut menu_help_message: *const c_char = std::ptr::null ();
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut prev_menu_help_message: *const c_char = std::ptr::null ();
/* Pane number and item number of the menu item which generated the
   last menu help message.  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut menu_help_paneno: c_int = 0;
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
static mut menu_help_itemno: c_int = 0;

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_menu_create () -> *mut XMenu          // C 2803-2804
{
  let mut menu: *mut XMenu;

  menu = xmalloc (std::mem::size_of::<XMenu> ()) as *mut XMenu;
  /* C's `menu->allocated = menu->count = menu->panecount = menu->width = 0;` is a chained
     assignment; the port writes the four stores, innermost first (Deviations 21).  */
  (*menu).width = 0;
  (*menu).panecount = 0;
  (*menu).count = 0;
  (*menu).allocated = 0;
  return menu;
}

/* Allocate some (more) memory for MENU ensuring that there is room for one
   for item.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_menu_make_room (menu: *mut XMenu)     // C 2816-2817
{
  if (*menu).allocated == 0
    {
      /* C's `int count = menu->allocated = 10;` hoists the store (Deviations 6).  */
      (*menu).allocated = 10;
      let count: c_int = 10;
      (*menu).text = xmalloc (count as usize * std::mem::size_of::<*mut c_char> ())
	as *mut *mut c_char;
      (*menu).submenu = xmalloc (count as usize * std::mem::size_of::<*mut XMenu> ())
	as *mut *mut XMenu;
      (*menu).panenumber = xmalloc (count as usize * std::mem::size_of::<c_int> ())
	as *mut c_int;
      (*menu).help_text = xmalloc (count as usize * std::mem::size_of::<*const c_char> ())
	as *mut *const c_char;
    }
  else if (*menu).allocated == (*menu).count
    {
      /* C's `int count = menu->allocated = menu->allocated + 10;` hoists the store.  */
      (*menu).allocated = (*menu).allocated + 10;
      let count: c_int = (*menu).allocated;
      (*menu).text = xrealloc ((*menu).text as *mut c_void,
			       count as usize * std::mem::size_of::<*mut c_char> ())
	as *mut *mut c_char;
      (*menu).submenu = xrealloc ((*menu).submenu as *mut c_void,
				  count as usize * std::mem::size_of::<*mut XMenu> ())
	as *mut *mut XMenu;
      (*menu).panenumber = xrealloc ((*menu).panenumber as *mut c_void,
				     count as usize * std::mem::size_of::<c_int> ())
	as *mut c_int;
      (*menu).help_text = xrealloc ((*menu).help_text as *mut c_void,
				    count as usize * std::mem::size_of::<*const c_char> ())
	as *mut *const c_char;
    }
}

/* Search the given menu structure for a given pane number.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_menu_search_pane (menu: *mut XMenu, pane: c_int) -> *mut XMenu   // C 2839-2840
{
  let mut i: c_int;
  let mut r#try: *mut XMenu;

  i = 0;
  while i < (*menu).count
    {
      if !(*(*menu).submenu.offset (i as isize)).is_null ()
	{
	  if pane == *(*menu).panenumber.offset (i as isize)
	    { return *(*menu).submenu.offset (i as isize); }
	  r#try = IT_menu_search_pane (*(*menu).submenu.offset (i as isize), pane);
	  if !r#try.is_null ()
	    { return r#try; }
	}
      i += 1;
    }
  return std::ptr::null_mut ();
}

/* Determine how much screen space a given menu needs.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_menu_calc_size (menu: *mut XMenu, width: *mut c_int, height: *mut c_int)   // C 2858-2859
{
  let mut i: c_int;
  let mut h2: c_int = 0;	/* C leaves it to the call below (PORTING.md 1.5) */
  let mut w2: c_int = 0;	/* the same */
  let mut maxsubwidth: c_int;
  let mut maxheight: c_int;

  maxsubwidth = 0;
  maxheight = (*menu).count;
  i = 0;
  while i < (*menu).count
    {
      if !(*(*menu).submenu.offset (i as isize)).is_null ()
	{
	  IT_menu_calc_size (*(*menu).submenu.offset (i as isize), &raw mut w2, &raw mut h2);
	  if w2 > maxsubwidth { maxsubwidth = w2; }
	  if i + h2 > maxheight { maxheight = i + h2; }
	}
      i += 1;
    }
  *width = (*menu).width + maxsubwidth;
  *height = maxheight;
}

/* Display MENU at (X,Y) using FACES.  */

/* C 2880-2887's `BUILD_CHAR_GLYPH` is a file-local macro, so the port keeps C's name; C's
   `(GLYPH).type = CHAR_GLYPH` is the `glyph_bits!` member the mirror names
   (`include/dispextern.rs` 531-534) and C's do-while is the block (the `SET_CHAR_GLYPH` reading,
   `docs/ports/macro-body-notes.tsv` 283).  */
#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
macro_rules! BUILD_CHAR_GLYPH {
  ($f:expr, $glyph:expr, $code:expr, $face_id:expr, $padding_p:expr) => { {
      glyph_bits! (type, $glyph, CHAR_GLYPH);
      SET_CHAR_GLYPH! ($f, $glyph, $code, $face_id, $padding_p);
      ($glyph).charpos = -1;
    } };
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
unsafe fn IT_menu_display (menu: *mut XMenu, y: c_int, x: c_int, pn: c_int,
			   faces: *mut c_int, disp_help: c_int)   // C 2889-2890
{
  let mut i: c_int;
  let mut j: c_int;
  let mut face: c_int;
  let width: c_int;
  let mut mx: c_int;
  let mut my: c_int;
  let mut enabled: c_int;
  let mut mousehere: c_int;
  let mut row: c_int;
  let mut col: c_int;
  let mut text: *mut glyph;
  let mut p: *mut glyph;
  let mut q: *const c_uchar;
  let sf: *mut frame = SELECTED_FRAME ();

  menu_help_message = std::ptr::null ();

  width = (*menu).width;
  /* We multiply width by 2 to account for possible control characters.
     FIXME: cater to non-ASCII characters in menus.  */
  text = xmalloc ((width * 2 + 2) as usize * std::mem::size_of::<glyph> ()) as *mut glyph;
  ScreenGetCursor (&raw mut row, &raw mut col);
  mouse_get_xy (&raw mut mx, &raw mut my);
  IT_update_begin (sf);
  i = 0;
  while i < (*menu).count
    {
      let mut max_width: c_int = width + 2;

      cursor_to (sf, y + i, x);
      enabled
	= (((*(*menu).submenu.offset (i as isize)).is_null ()
	    && *(*menu).panenumber.offset (i as isize) != 0)
	   || !(*(*menu).submenu.offset (i as isize)).is_null ()) as c_int;
      mousehere = (y + i == my && x <= mx && mx < x + max_width) as c_int;
      face = *faces.offset ((enabled + mousehere * 2) as isize);
      /* The following if clause means that we display the menu help
	 strings even if the menu item is currently disabled.  */
      if disp_help != 0 && enabled + mousehere * 2 >= 2
	{
	  menu_help_message = *(*menu).help_text.offset (i as isize);
	  menu_help_paneno = pn - 1;
	  menu_help_itemno = i;
	}
      p = text;
      BUILD_CHAR_GLYPH! (sf as *mut c_void, *p, ' ' as c_uint, face, false);
      p = p.offset (1);
      j = 0;
      q = *(*menu).text.offset (i as isize) as *const c_uchar;
      while *q != 0
	{
	  let c: c_uint = string_char_advance (&raw mut q) as c_uint;

	  if c > 26
	    {
	      BUILD_CHAR_GLYPH! (sf as *mut c_void, *p, c, face, false);
	      p = p.offset (1);
	    }
	  else	/* make '^x' */
	    {
	      BUILD_CHAR_GLYPH! (sf as *mut c_void, *p, '^' as c_uint, face, false);
	      p = p.offset (1);
	      j += 1;
	      BUILD_CHAR_GLYPH! (sf as *mut c_void, *p, c + 64, face, false);
	      p = p.offset (1);
	    }
	  j += 1;
	}
      /* Don't let the menu text overflow into the next screen row.  */
      if x + max_width > screen_size_X
	{
	  max_width = screen_size_X - x;
	  (*text.offset ((max_width - 1) as isize)).u.ch = b'$' as c_uint;
	  /* indicate it's truncated */
	}
      while j < max_width - 2
	{
	  BUILD_CHAR_GLYPH! (sf as *mut c_void, *p, ' ' as c_uint, face, false);
	  j += 1;
	  p = p.offset (1);
	}

      /* 16 is the character code of a character that on DOS terminal
	 produces a nice-looking right-pointing arrow glyph.  */
      BUILD_CHAR_GLYPH! (sf as *mut c_void, *p,
			 if !(*(*menu).submenu.offset (i as isize)).is_null ()
			 { 16 as c_uint } else { ' ' as c_uint }, face, false);
      p = p.offset (1);
      IT_write_glyphs (sf, text, max_width);
      i += 1;
    }
  IT_update_end (sf);
  cursor_to (sf, row, col);
  xfree (text as *mut c_void);
}

/* --------------------------- X Menu emulation ---------------------- */

/* Create a brand new menu structure.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub unsafe fn XMenuCreate (foo1: *mut Display, foo2: Window, foo3: *mut c_char)
			   -> *mut XMenu       // C 2968-2969
{
  return IT_menu_create ();
}

/* Create a new pane and place it on the outer-most level.  It is not
   clear that it should be placed out there, but I don't know what else
   to do.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub unsafe fn XMenuAddPane (foo: *mut Display, menu: *mut XMenu, txt: *const c_char,
			    enable: c_int) -> c_int             // C 2978-2979
{
  let mut len: c_int;
  let mut p: *const c_char;

  if enable == 0
    { emacs_abort (); }

  IT_menu_make_room (menu);
  *(*menu).submenu.offset ((*menu).count as isize) = IT_menu_create ();
  *(*menu).text.offset ((*menu).count as isize) = txt as *mut c_char;
  /* C's `menu->panenumber[menu->count] = ++menu->panecount;` hoists the pre-increment.  */
  (*menu).panecount += 1;
  *(*menu).panenumber.offset ((*menu).count as isize) = (*menu).panecount;
  *(*menu).help_text.offset ((*menu).count as isize) = std::ptr::null ();
  (*menu).count += 1;

  /* Adjust length for possible control characters (which will
     be written as ^x).  */
  len = strlen (txt) as c_int;
  p = txt;
  while *p != 0
    {
      if (*p as c_int) < 27
	{ len += 1; }
      p = p.offset (1);
    }

  if len > (*menu).width
    { (*menu).width = len; }

  return (*menu).panecount;
}

/* Create a new item in a menu pane.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub unsafe fn XMenuAddSelection (bar: *mut Display, mut menu: *mut XMenu, pane: c_int,
				 foo: c_int, txt: *mut c_char, enable: c_int,
				 help_text: *const c_char) -> c_int   // C 3008-3010
{
  let mut len: c_int;
  let mut p: *mut c_char;

  if pane != 0
    {
      /* C's `if (!(menu = IT_menu_search_pane (menu, pane)))`, hoisted (Deviations 6).  */
      menu = IT_menu_search_pane (menu, pane);
      if menu.is_null ()
	{ return XM_FAILURE; }
    }
  IT_menu_make_room (menu);
  *(*menu).submenu.offset ((*menu).count as isize) = std::ptr::null_mut ();
  *(*menu).text.offset ((*menu).count as isize) = txt;
  *(*menu).panenumber.offset ((*menu).count as isize) = enable;
  *(*menu).help_text.offset ((*menu).count as isize) = help_text;
  (*menu).count += 1;

  /* Adjust length for possible control characters (which will
     be written as ^x).  */
  len = strlen (txt) as c_int;
  p = txt;
  while *p != 0
    {
      if (*p as c_int) < 27
	{ len += 1; }
      p = p.offset (1);
    }

  if len > (*menu).width
    { (*menu).width = len; }

  return XM_SUCCESS;
}

/* Decide where the menu would be placed if requested at (X,Y).  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub unsafe fn XMenuLocate (foo0: *mut Display, menu: *mut XMenu, foo1: c_int, foo2: c_int,
			   x: c_int, y: c_int, ulx: *mut c_int, uly: *mut c_int,
			   width: *mut c_int, height: *mut c_int)   // C 3039-3041
{
  IT_menu_calc_size (menu, width, height);
  *ulx = x + 1;
  *uly = y;
  *width += 2;
}

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[repr(C)]
pub struct IT_menu_state
{
  pub screen_behind: *mut c_void,
  pub menu: *mut XMenu,
  pub pane: c_int,
  pub x: c_int,
  pub y: c_int,
}

/* Display menu, wait for user's response, and return that response.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub unsafe fn XMenuActivate (foo: *mut Display, menu: *mut XMenu, pane: *mut c_int,
			     selidx: *mut c_int, x0: c_int, y0: c_int, ButtonMask: c_uint,
			     txt: *mut *mut c_char,
			     help_callback: Option<unsafe extern "C" fn (*const c_char, c_int,
									 c_int)>)
			     -> c_int                      // C 3060-3063
{
  let mut state: *mut IT_menu_state;
  let mut statecount: c_int;
  let mut x: c_int;
  let mut y: c_int;
  let mut i: c_int;
  let mut b: c_int;
  let screensize: c_int;
  let mut leave: c_int;
  let mut result: c_int;
  let mut onepane: c_int;
  let mut title_faces: [c_int; 4] = [0; 4];	/* face to display the menu title */
  let mut faces: [c_int; 4] = [0; 4];
  let mut buffers_num_deleted: c_int = 0;
  let sf: *mut frame = SELECTED_FRAME ();
  let mut saved_echo_area_message: Lisp_Object;
  let mut selectface: Lisp_Object;
  let mut x0 = x0;
  let mut y0 = y0;

  /* Just in case we got here without a mouse present...  */
  if have_mouse <= 0
    { return XM_IA_SELECT; }
  /* Don't allow non-positive x0 and y0, lest the menu will wrap
     around the display.  */
  if x0 <= 0
    { x0 = 1; }
  if y0 <= 0
    { y0 = 1; }

  /* We will process all the mouse events directly, so we had
     better prevent dos_rawgetc from stealing them from us.  */
  mouse_preempted += 1;

  /* C's `state = alloca (…)`: the port has no `alloca` (PORTING.md P-05), so the block comes
     from the stack pool `sa_stack_alloc` serves (Deviations 16).  */
  state = sa_stack_alloc ((*menu).panecount as usize * std::mem::size_of::<IT_menu_state> ())
    as *mut IT_menu_state;
  screensize = screen_size * 2;
  faces[0]
    = lookup_derived_face (std::ptr::null_mut::<window> (), sf, Qmsdos_menu_passive_face (),
			   DEFAULT_FACE_ID, true);
  faces[1]
    = lookup_derived_face (std::ptr::null_mut::<window> (), sf, Qmsdos_menu_active_face (),
			   DEFAULT_FACE_ID, true);
  selectface = Qmsdos_menu_select_face ();
  faces[2] = lookup_derived_face (std::ptr::null_mut::<window> (), sf, selectface,
				  faces[0], true);
  faces[3] = lookup_derived_face (std::ptr::null_mut::<window> (), sf, selectface,
				  faces[1], true);

  /* Make sure the menu title is always displayed with
     `msdos-menu-active-face', no matter where the mouse pointer is.  */
  i = 0;
  while i < 4
    {
      title_faces[i as usize] = faces[3];
      i += 1;
    }

  statecount = 1;

  /* Don't let the title for the "Buffers" popup menu include a
     digit (which is ugly).

     This is a terrible kludge, but I think the "Buffers" case is
     the only one where the title includes a number, so it doesn't
     seem to be necessary to make this more general.  */
  if strncmp (*(*menu).text.offset (0), c"Buffers 1".as_ptr (), 9) == 0
    {
      *(*(*menu).text.offset (0)).offset (7) = 0;	/* '\0' */
      buffers_num_deleted = 1;
    }

  /* We need to save the current echo area message, so that we could
     restore it below, before we exit.  See the commentary below,
     before the call to message_with_string.  */
  saved_echo_area_message = Fcurrent_message ();
  (*state.offset (0)).menu = menu;
  mouse_off ();
  /* C's `ScreenRetrieve (state[0].screen_behind = xmalloc (screensize))` hoists the store
     (Deviations 21).  */
  (*state.offset (0)).screen_behind = xmalloc (screensize as usize);
  ScreenRetrieve ((*state.offset (0)).screen_behind);

  /* Turn off the cursor.  Otherwise it shows through the menu
     panes, which is ugly.  */
  IT_display_cursor (0);

  /* Display the menu title.  */
  IT_menu_display (menu, y0 - 1, x0 - 1, 1, &raw mut faces[0], 0);
  if buffers_num_deleted != 0
    { *(*(*menu).text.offset (0)).offset (7) = b' ' as c_char; }
  /* C's `if ((onepane = menu->count == 1 && menu->submenu[0]))`, hoisted (Deviations 6).  */
  onepane
    = ((*menu).count == 1 && !(*(*menu).submenu.offset (0)).is_null ()) as c_int;
  if onepane != 0
    {
      (*menu).width = (*(*(*menu).submenu.offset (0))).width;
      (*state.offset (0)).menu = *(*menu).submenu.offset (0);
    }
  else
    {
      (*state.offset (0)).menu = menu;
    }
  (*state.offset (0)).x = x0 - 1;
  (*state.offset (0)).y = y0;
  (*state.offset (0)).pane = onepane;

  mouse_last_x = -1;  /* A hack that forces display.  */
  leave = 0;
  while leave == 0
    {
      if mouse_visible == 0 { mouse_on (); }
      mouse_check_moved ();
      if mouse_moved! (sf)
	{
	  mouse_moved! (sf, 0);
	  result = XM_IA_SELECT;
	  mouse_get_xy (&raw mut x, &raw mut y);
	  i = 0;
	  while i < statecount
	    {
	    if (*state.offset (i as isize)).x <= x
		&& x < (*state.offset (i as isize)).x
			+ (*(*state.offset (i as isize)).menu).width + 2
	      {
		let dy: c_int = y - (*state.offset (i as isize)).y;
		if 0 <= dy && dy < (*(*state.offset (i as isize)).menu).count
		  {
		    if (*(*(*state.offset (i as isize)).menu).submenu.offset (dy as isize))
			 .is_null ()
		      {
			if *(* (*state.offset (i as isize)).menu ).panenumber
			     .offset (dy as isize) != 0
			  { result = XM_SUCCESS; }
			else
			  { result = XM_IA_SELECT; }
		      }
		    *pane = (*state.offset (i as isize)).pane - 1;
		    *selidx = dy;
		    /* We hit some part of a menu, so drop extra menus that
		       have been opened.  That does not include an open and
		       active submenu.  */
		    if i != statecount - 2
			|| *(*(*state.offset (i as isize)).menu).submenu.offset (dy as isize)
			   != (*state.offset ((i + 1) as isize)).menu
		      {
			while i != statecount - 1
			  {
			    statecount -= 1;
			    mouse_off ();
			    ScreenUpdate ((*state.offset (statecount as isize)).screen_behind);
			    if screen_virtual_segment != 0
			      { dosv_refresh_virtual_screen (0, screen_size); }
			    xfree ((*state.offset (statecount as isize)).screen_behind);
			  }
		      }
		    if i == statecount - 1
			&& !(*(*(*state.offset (i as isize)).menu).submenu
			     .offset (dy as isize)).is_null ()
		      {
			IT_menu_display ((*state.offset (i as isize)).menu,
					 (*state.offset (i as isize)).y,
					 (*state.offset (i as isize)).x,
					 (*state.offset (i as isize)).pane,
					 &raw mut faces[0], 1);
			(*state.offset (statecount as isize)).menu
			  = *(*(*state.offset (i as isize)).menu).submenu.offset (dy as isize);
			(*state.offset (statecount as isize)).pane
			  = *(* (*state.offset (i as isize)).menu).panenumber.offset (dy as isize);
			mouse_off ();
			/* C's `ScreenRetrieve (state[…] .screen_behind = xmalloc (screensize))`
			   hoists the store (Deviations 21).  */
			(*state.offset (statecount as isize)).screen_behind
			  = xmalloc (screensize as usize);
			ScreenRetrieve ((*state.offset (statecount as isize)).screen_behind);
			(*state.offset (statecount as isize)).x
			  = (*state.offset (i as isize)).x
			    + (*(*state.offset (i as isize)).menu).width + 2;
			(*state.offset (statecount as isize)).y = y;
			statecount += 1;
		      }
		  }
	      }
	    i += 1;
	    }
	  IT_menu_display ((*state.offset ((statecount - 1) as isize)).menu,
			   (*state.offset ((statecount - 1) as isize)).y,
			   (*state.offset ((statecount - 1) as isize)).x,
			   (*state.offset ((statecount - 1) as isize)).pane,
			   &raw mut faces[0], 1);
	}
      else
	{
	  if (!menu_help_message.is_null () || !prev_menu_help_message.is_null ())
	      && menu_help_message != prev_menu_help_message
	    {
	      (help_callback.unwrap ()) (menu_help_message,
					 menu_help_paneno, menu_help_itemno);
	      IT_display_cursor (0);
	      prev_menu_help_message = menu_help_message;
	    }
	  /* We are busy-waiting for the mouse to move, so let's be nice
	     to other Windows applications by releasing our time slice.  */
	  __dpmi_yield ();
	}
      b = 0;
      while b < mouse_button_count && leave == 0
	{
	  /* Only leave if user both pressed and released the mouse, and in
	     that order.  This avoids popping down the menu pane unless
	     the user is really done with it.  */
	  if mouse_pressed (b, &raw mut x, &raw mut y) != 0
	    {
	      while mouse_button_depressed (b, &raw mut x, &raw mut y) != 0
		{ __dpmi_yield (); }
	      leave = 1;
	    }
	  let _ = mouse_released (b, &raw mut x, &raw mut y);	/* C's `(void) …` */
	  b += 1;
	}
    }

  mouse_off ();
  ScreenUpdate ((*state.offset (0)).screen_behind);
  if screen_virtual_segment != 0
    { dosv_refresh_virtual_screen (0, screen_size); }

  /* We have a situation here.  ScreenUpdate has just restored the
     screen contents as it was before we started drawing this menu.
     That includes any echo area message that could have been
     displayed back then.  (In reality, that echo area message will
     almost always be the ``keystroke echo'' that echoes the sequence
     of menu items chosen by the user.)  However, if the menu had some
     help messages, then displaying those messages caused Emacs to
     forget about the original echo area message.  So when
     ScreenUpdate restored it, it created a discrepancy between the
     actual screen contents and what Emacs internal data structures
     know about it.

     To avoid this conflict, we force Emacs to restore the original
     echo area message as we found it when we entered this function.
     The irony of this is that we then erase the restored message
     right away, so the only purpose of restoring it is so that
     erasing it works correctly...  */
  if !NILP (saved_echo_area_message)
    { message_with_string (c"%s".as_ptr (), saved_echo_area_message, false); }
  message1 (std::ptr::null ());
  while statecount != 0
    {
      statecount -= 1;
      xfree ((*state.offset (statecount as isize)).screen_behind);
    }
  IT_display_cursor (1);	/* Turn cursor back on.  */
  /* Clean up any mouse events that are waiting inside Emacs event queue.
     These events are likely to be generated before the menu was even
     displayed, probably because the user pressed and released the button
     (which invoked the menu) too quickly.  If we don't remove these events,
     Emacs will process them after we return and surprise the user.  */
  discard_mouse_events ();
  mouse_clear_clicks ();
  if !kbd_buffer_events_waiting ()
    { clear_input_pending (); }
  /* Allow mouse events generation by dos_rawgetc.  */
  mouse_preempted -= 1;
  return result;
}

/* Dispose of a menu.  */

#[cfg (all (feature = "msdos", not (feature = "have_x_windows")))]
#[unsafe(no_mangle)]
pub unsafe fn XMenuDestroy (foo: *mut Display, menu: *mut XMenu)   // C 3286-3287
{
  let mut i: c_int;
  if (*menu).allocated != 0
    {
      i = 0;
      while i < (*menu).count
	{
	  if !(*(*menu).submenu.offset (i as isize)).is_null ()
	    { XMenuDestroy (foo, *(*menu).submenu.offset (i as isize)); }
	  i += 1;
	}
      xfree ((*menu).text as *mut c_void);
      xfree ((*menu).submenu as *mut c_void);
      xfree ((*menu).panenumber as *mut c_void);
      xfree ((*menu).help_text as *mut c_void);
    }
  xfree (menu as *mut c_void);
  /* C's `menu_help_message = prev_menu_help_message = NULL;` is a chained assignment
     (Deviations 21).  */
  prev_menu_help_message = std::ptr::null ();
  menu_help_message = std::ptr::null ();
}

/* C 3303: `#endif /* !HAVE_X_WINDOWS */` - every item from C 3305 on is gated by the file
   alone.  */

/* ----------------------- DOS / UNIX conversion --------------------- */

/* C 3307's `void msdos_downcase_filename (unsigned char *);` is C's own forward declaration of
   the definition at C 3417; the port drops it (PORTING.md 0.1-12, the C 1769 reading).  */

/* Destructively turn backslashes into slashes.  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn dostounix_filename (mut p: *mut c_char)   // C 3311-3312
{
  msdos_downcase_filename (p as *mut c_uchar);

  while *p != 0
    {
      if *p == b'\\' as c_char
	{ *p = b'/' as c_char; }
      p = p.offset (1);
    }
}

/* Destructively turn slashes into backslashes.  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn unixtodos_filename (mut p: *mut c_char)   // C 3326-3327
{
  if *p.offset (1) == b':' as c_char && *p >= b'A' as c_char && *p <= b'Z' as c_char
    {
      *p += (b'a' - b'A') as c_char;
      p = p.offset (2);
    }

  while *p != 0
    {
      if *p == b'/' as c_char
	{ *p = b'\\' as c_char; }
      p = p.offset (1);
    }
}

/* Get the default directory for a given drive.  0=def, 1=A, 2=B, ...  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn getdefdir (drive: c_int, dst: *mut c_char) -> c_int   // C 3345-3346
{
  let mut in_path: [c_char; 4] = [0; 4];
  let mut p: *mut c_char = &raw mut in_path[0];
  let e: c_char = *errno () as c_char;

  /* Generate "X:." (when drive is X) or "." (when drive is 0).  */
  if drive != 0
    {
      *p = (drive + b'A' as c_int - 1) as c_char;
      p = p.offset (1);
      *p = b':' as c_char;
      p = p.offset (1);
    }

  *p = b'.' as c_char;
  p = p.offset (1);
  *p = 0;		/* '\0' */
  *errno () = 0;
  _fixpath (&raw mut in_path[0], dst);
    /* _fixpath can set errno to ENOSYS on non-LFN systems because
       it queries the LFN support, so ignore that error.  */
  if (*errno () != 0 && *errno () != ENOSYS) || *dst == 0
    { return 0; }

  msdos_downcase_filename (dst as *mut c_uchar);

  *errno () = e as c_int;
  return 1;
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn emacs_root_dir () -> *mut c_char    // C 3372-3373
{
  static mut root_dir: [c_char; 4] = [0; 4];

  sprintf (&raw mut root_dir[0], c"%c:/".as_ptr (), b'A' as c_int + getdisk ());
  root_dir[0] = tolower (root_dir[0] as c_int) as c_char;
  return &raw mut root_dir[0];
}

/* Remove all CR's that are followed by a LF.  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn crlf_to_lf (n: c_int, mut buf: *mut c_uchar) -> c_int   // C 3384-3385
{
  let mut np: *mut c_uchar = buf;
  let startp: *mut c_uchar = buf;
  let endp: *mut c_uchar = buf.offset (n as isize);

  if n == 0
    { return n; }
  while buf < endp.offset (-1)
    {
      if *buf == 0x0d
	{
	  buf = buf.offset (1);
	  if *buf != 0x0a
	    { *np = 0x0d; np = np.offset (1); }
	}
      else
	{ *np = *buf; np = np.offset (1); buf = buf.offset (1); }
    }
  if buf < endp
    { *np = *buf; np = np.offset (1); buf = buf.offset (1); }
  return np.offset_from (startp) as c_int;
}

#[cfg (feature = "msdos")]
DEFUN! ("msdos-long-file-names", Fmsdos_long_file_names, Smsdos_long_file_names,
       0, 0, 0,
       doc: "Return non-nil if long file names are supported on MS-DOS.  ",   // C 3406-3408
       (),
       {
  return if _USE_LFN != 0 { Qt () } else { Qnil () };
});

/* Convert alphabetic characters in a filename to lower-case.  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn msdos_downcase_filename (mut p: *mut c_uchar)   // C 3416-3417
{
  /* Always lower-case drive letters a-z, even if the filesystem
     preserves case in filenames.
     This is so MSDOS filenames could be compared by string comparison
     functions that are case-sensitive.  Even case-preserving filesystems
     do not distinguish case in drive letters.  */
  if *p.offset (1) == b':' && *p >= b'A' && *p <= b'Z'
    {
      *p += b'a' - b'A';
      p = p.offset (2);
    }

  /* Under LFN we expect to get pathnames in their true case.  */
  if NILP (Fmsdos_long_file_names ())
    {
      while *p != 0
	{
	  if *p >= b'A' && *p <= b'Z'
	    { *p += b'a' - b'A'; }
	  p = p.offset (1);
	}
    }
}

#[cfg (feature = "msdos")]
DEFUN! ("msdos-downcase-filename", Fmsdos_downcase_filename, Smsdos_downcase_filename,
       1, 1, 0,
       doc: "Convert alphabetic characters in FILENAME to lower case and return that.\nWhen long filenames are supported, doesn't change FILENAME.\nIf FILENAME is not a string, returns nil.\nThe argument object is never altered--the value is a copy.  ",   // C 3437-3442
       (filename: Lisp_Object),
       {
  let mut tem: Lisp_Object;

  if ! STRINGP (filename)
    { return Qnil (); }

  tem = Fcopy_sequence (filename);
  msdos_downcase_filename (SDATA (tem));
  return tem;
});

/* The Emacs root directory as determined by init_environment.  */

#[cfg (feature = "msdos")]
static mut emacsroot: [c_char; MAXPATHLEN] = [0; MAXPATHLEN];

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn rootrelativepath (rel: *mut c_char) -> *mut c_char   // C 3459-3460
{
  static mut result: [c_char; MAXPATHLEN + 10] = [0; MAXPATHLEN + 10];

  strcpy (&raw mut result[0], &raw mut emacsroot[0]);
  strcat (&raw mut result[0], c"/".as_ptr ());
  strcat (&raw mut result[0], rel);
  return &raw mut result[0];
}

/* C 3470-3473 is `init_environment`'s own banner and C 3475 its first line: that function is
   the next live definition, and C 3469-4350 is the unported remainder (see
   `docs/ports/msdos.md`).  */

/* ----------------------------------------------------------------------------------------------
   C 3469-4350: slice 4, the file's remainder — `init_environment` (C 3475-3638), the tty mode
   pair, `run_msdos_command`, the gnulib emulations (`readlink`, `sys_opendir`, the `*at`
   family, `futimens`, `careadlinkat`), the `sys_select` arm, `sys_chdir`, `init_gettimeofday`
   and `syms_of_msdos`, through C 4350's `#endif /* MSDOS */`, which closes the file gate.
   ---------------------------------------------------------------------------------------------- */

/* Define a lot of environment variables if not already defined.  Don't
   remove anything unless you know what you're doing -- lots of code will
   break if one or more of these are missing.  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn init_environment (argc: c_int, argv: *mut *mut c_char,
				skip_args: c_int)   // C 3474-3475
{
  let mut s: *mut c_char;
  let mut t: *mut c_char;
  let mut root: *mut c_char;
  let mut len: c_int;
  let mut i: c_int;
  /* C's `static const char * const tempdirs[]` is a *local* static; the port keeps C's
     initializer and scope but not the storage class, because a Rust `static` of raw pointers is
     not `Sync` (E0277, measured) - the array is only read, so the two are behaviourally the
     same.  One `## Deviations` entry.  */
  let tempdirs: [*const c_char; 4] =
    [c"$TMPDIR".as_ptr (), c"$TEMP".as_ptr (), c"$TMP".as_ptr (), c"c:/".as_ptr ()];
  let imax: c_int = countof! (tempdirs) as c_int;	/* C's int-vs-size_t (Deviations 20) */

  /* Make sure they have a usable $TMPDIR.  Many Emacs functions use
     temporary files and assume "/tmp" if $TMPDIR is unset, which
     will break on DOS/Windows.  Refuse to work if we cannot find
     a directory, not even "c:/", usable for that purpose.  */
  i = 0;
  while i < imax
    {
      let mut tmp: *const c_char = tempdirs[i as usize];
      /* C 3491: `char buf[FILENAME_MAX];` - the mirror carries the measured constant
         (`lib/include/stdio.rs`, `FILENAME_MAX` = 1024), and Rust's array length is a
         `usize`, so the C-shaped bound is spelled with the cast (PORTING.md 1.1).  */
      let mut buf: [c_char; FILENAME_MAX as usize] = [0; FILENAME_MAX as usize];

      if *tmp == b'$' as c_char
	{
	  let mut tmp_len: c_int;

	  tmp = getenv (tmp.offset (1));
	  if tmp.is_null ()
	    { i += 1; continue; }

	  /* Some lusers set TMPDIR=e:, probably because some losing
	     programs cannot handle multiple slashes if they use e:/.
	     e: fails in `access' below, so we interpret e: as e:/.  */
	  tmp_len = strlen (tmp) as c_int;
	  if *tmp.offset ((tmp_len - 1) as isize) != b'/' as c_char
	      && *tmp.offset ((tmp_len - 1) as isize) != b'\\' as c_char
	    {
	      strcpy (&raw mut buf[0], tmp);
	      *(&raw mut buf[0]).offset (tmp_len as isize) = b'/' as c_char;
	      tmp_len += 1;
	      *(&raw mut buf[0]).offset (tmp_len as isize) = 0;
	      tmp = &raw mut buf[0];
	    }
	}

      /* Note that `access' can lie to us if the directory resides on a
	 read-only filesystem, like CD-ROM or a write-protected floppy.
	 The only way to be really sure is to actually create a file and
	 see if it succeeds.  But I think that's too much to ask.  */
      if !tmp.is_null () && access (tmp, D_OK) == 0
	{
	  setenv (c"TMPDIR".as_ptr (), tmp, 1);
	  break;
	}
      i += 1;
    }
  if i >= imax
    { cmd_error_internal (Fcons (Qerror (),
				Fcons (build_string (c"no usable temporary directories found!!".as_ptr ()),
				       Qnil ())),
			  c"While setting TMPDIR: ".as_ptr ()); }

  /* Note the startup time, so we know not to clear the screen if we
     exit immediately; see IT_reset_terminal_modes.
     (Yes, I know `clock' returns zero the first time it's called, but
     I do this anyway, in case some wiseguy changes that at some point.)  */
  startup_time = clock ();

  /* Find our root from argv[0].  Assuming argv[0] is, say,
     "c:/emacs/bin/emacs.exe" our root will be "c:/emacs".  */
  /* C's `alloca` requests are the port's stack pool (Deviations 32).  */
  root = sa_stack_alloc (MAXPATHLEN + 20) as *mut c_char;
  _fixpath (*argv.offset (0), root);
  msdos_downcase_filename (root as *mut c_uchar);
  len = strlen (root) as c_int;
  while len > 0 && *root.offset (len as isize) != b'/' as c_char
      && *root.offset (len as isize) != b':' as c_char
    { len -= 1; }
  *root.offset (len as isize) = 0;	/* '\0' */
  if len > 4
      && (strcmp (root.offset ((len - 4) as isize), c"/bin".as_ptr ()) == 0
	  || strcmp (root.offset ((len - 4) as isize), c"/src".as_ptr ()) == 0)
    { *root.offset ((len - 4) as isize) = 0; }	/* under a debugger */
  else
    { strcpy (root, c"c:/emacs".as_ptr ()); }	/* let's be defensive */
  len = strlen (root) as c_int;
  strcpy (&raw mut emacsroot[0], root);

  /* We default HOME to our root.  */
  setenv (c"HOME".as_ptr (), root, 0);

  /* We default EMACSPATH to root + "/bin".  */
  strcpy (root.offset (len as isize), c"/bin".as_ptr ());
  setenv (c"EMACSPATH".as_ptr (), root, 0);

  /* I don't expect anybody to ever use other terminals so the internal
     terminal is the default.  */
  setenv (c"TERM".as_ptr (), c"internal".as_ptr (), 0);

  /* C 3565-3568: `#ifdef HAVE_X_WINDOWS` - both arms (the `#else` arm is empty in C).  */
  #[cfg (feature = "have_x_windows")]
  {
  /* Emacs expects DISPLAY to be set.  */
  setenv (c"DISPLAY".as_ptr (), c"unix:0.0".as_ptr (), 0);
  }

  /* SHELL is a bit tricky -- COMSPEC is the closest we come, but we must
     downcase it and mirror the backslashes.  */
  s = getenv (c"COMSPEC".as_ptr ());
  if s.is_null () { s = c"c:/command.com".as_ptr () as *mut c_char; }
  t = sa_stack_alloc (strlen (s) + 1) as *mut c_char;
  strcpy (t, s);
  dostounix_filename (t);
  setenv (c"SHELL".as_ptr (), t, 0);

  /* PATH is also downcased and backslashes mirrored.  */
  s = getenv (c"PATH".as_ptr ());
  if s.is_null () { s = c"".as_ptr () as *mut c_char; }
  t = sa_stack_alloc (strlen (s) + 3) as *mut c_char;
  /* Current directory is always considered part of MsDos's path but it is
     not normally mentioned.  Now it is.  */
  strcat (strcpy (t, c".;".as_ptr ()), s);
  dostounix_filename (t); /* Not a single file name, but this should work.  */
  setenv (c"PATH".as_ptr (), t, 1);

  /* In some sense all dos users have root privileges, so...  */
  setenv (c"USER".as_ptr (), c"root".as_ptr (), 0);
  setenv (c"NAME".as_ptr (), getenv (c"USER".as_ptr ()), 0);

  /* Time zone determined from country code.  To make this possible, the
     country code may not span more than one time zone.  In other words,
     in the USA, you lose.  */
  if getenv (c"TZ".as_ptr ()).is_null ()
    {
      match dos_country_code
	{
	31 | 32 | 33 | 34 | 36 | 38 | 39 | 41 | 42 | 45 | 46 | 47 | 48 | 49 =>
	  {		/* Belgium, Netherlands, France, Spain, Hungary, Yugoslavia,
			   Italy, Switzerland, Tjekia, Denmark, Sweden, Norway,
			   Poland, Germany */
	    /* Daylight saving from last Sunday in March to last Sunday in
	       September, both at 2AM.  */
	    setenv (c"TZ".as_ptr (), c"MET-01METDST-02,M3.5.0/02:00,M9.5.0/02:00".as_ptr (), 0);
	  }
	44 | 351 | 354 =>
	  {		/* United Kingdom, Portugal, Iceland */
	    setenv (c"TZ".as_ptr (), c"GMT+00".as_ptr (), 0);
	  }
	81 | 82 =>
	  {		/* Japan, Korea */
	    setenv (c"TZ".as_ptr (), c"JST-09".as_ptr (), 0);
	  }
	90 | 358 =>
	  {		/* Turkey, Finland */
	    setenv (c"TZ".as_ptr (), c"EET-02".as_ptr (), 0);
	  }
	972 =>
	  {		/* Israel */
	    /* This is an approximation.  (For exact rules, use the
	       `zoneinfo/israel' file which comes with DJGPP, but you need
	       to install it in `/usr/share/zoneinfo/' directory first.)  */
	    setenv (c"TZ".as_ptr (), c"IST-02IDT-03,M4.1.6/00:00,M9.5.6/01:00".as_ptr (), 0);
	  }
	_ => {}
	}
    }
  tzset ();
}

/// C 3642: `static int break_stat;	 /* BREAK check mode status.	*/`
#[cfg (feature = "msdos")]
static mut break_stat: c_int = 0;
/// C 3643: `static int stdin_stat;	 /* stdin IOCTL status.		*/`
#[cfg (feature = "msdos")]
static mut stdin_stat: c_int = 0;

/* Turn off Dos' Ctrl-C checking and inhibit interpretation of
   control chars by DOS.   Determine the keyboard type.  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn dos_ttraw (tty: *mut tty_display_info) -> c_int   // C 3648-3649
{
  let mut inregs: REGS = std::mem::zeroed ();
  let mut outregs: REGS = std::mem::zeroed ();
  static mut first_time: c_int = 1;

  /* If we are called for the initial terminal, it's too early to do
     anything, and termscript isn't set up.  */
  if (*(*tty).terminal).r#type == output_initial
    { return 2; }

  break_stat = getcbrk ();
  setcbrk (0);

  if first_time != 0
    {
      inregs.h.ah = 0xc0;
      int86 (0x15, &raw mut inregs, &raw mut outregs);
      extended_kbd = (outregs.x.cflag == 0 && outregs.h.ah == 0) as c_int;

      have_mouse = 0;

      /* C 3670-3674's `if (1 #ifdef HAVE_X_WINDOWS && inhibit_window_system #endif )` is C's
	 "always, unless the window-system build says otherwise"; the port keeps both arms with
	 the `cfg!` idiom (`src/androidselect.rs` 585's shape) - one `## Deviations` entry.  */
      if !cfg! (feature = "have_x_windows") || inhibit_window_system
	{
	  inregs.x.ax = 0x0021;
	  int86 (0x33, &raw mut inregs, &raw mut outregs);
	  have_mouse = ((outregs.x.ax & 0xffff) == 0xffff) as c_int;
	  if have_mouse == 0
	    {
	      /* Reportedly, the above doesn't work for some mouse drivers.  There
		 is an additional detection method that should work, but might be
		 a little slower.  Use that as an alternative.  */
	      inregs.x.ax = 0x0000;
	      int86 (0x33, &raw mut inregs, &raw mut outregs);
	      have_mouse = ((outregs.x.ax & 0xffff) == 0xffff) as c_int;
	    }
	  if have_mouse != 0
	    { mouse_button_count = outregs.x.bx as c_int; }

	  /* C 3691-3694: `#ifndef HAVE_X_WINDOWS` - both arms (the `#else` arm is empty).  */
	  #[cfg (not (feature = "have_x_windows"))]
	  {
	  /* Save the cursor shape used outside Emacs.  */
	  outside_cursor = _farpeekw (_dos_ds! () as c_ushort, 0x460);
	  }
	}

      first_time = 0;

      stdin_stat = setmode (fileno (stdin), O_BINARY);
      return (stdin_stat != -1) as c_int;
    }
  else
    { return (setmode (fileno (stdin), O_BINARY) != -1) as c_int; }
}

/*  Restore status of standard input and Ctrl-C checking.  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn dos_ttcooked () -> c_int             // C 3708-3709
{
  let mut inregs: REGS = std::mem::zeroed ();
  let mut outregs: REGS = std::mem::zeroed ();

  setcbrk (break_stat);
  mouse_off ();

  /* C 3716-3724: `#ifndef HAVE_X_WINDOWS` - both arms (the `#else` arm is empty in C).  */
  #[cfg (not (feature = "have_x_windows"))]
  {
  /* Restore the cursor shape we found on startup.  */
  if outside_cursor != 0
    {
      inregs.h.ah = 1;
      inregs.x.cx = outside_cursor;
      int86 (0x10, &raw mut inregs, &raw mut outregs);
    }
  }

  return (setmode (fileno (stdin), stdin_stat) != -1) as c_int;
}

/* Run command as specified by ARGV in directory DIR.
   The command is run with input from TEMPIN, output to
   file TEMPOUT and stderr to TEMPERR.  */

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn run_msdos_command (argv: *mut *mut c_char, working_dir: *const c_char,
				 tempin: c_int, tempout: c_int, temperr: c_int,
				 envv: *mut *mut c_char) -> c_int   // C 3734-3736
{
  /* C 3738: `char *saveargv1, *saveargv2, …;` - declared without a value and assigned only
     on the `env == NULL` path (C 3763-3764), so PORTING.md 1.5's rule applies: the port
     initialises them (`&raw mut`/reads on the other path then see C's own uncertainty).  */
  let mut saveargv1: *mut c_char = core::ptr::null_mut ();
  let mut saveargv2: *mut c_char = core::ptr::null_mut ();
  let mut lowcase_argv0: *mut c_char;
  let mut pa: *mut c_char;
  let mut pl: *mut c_char;
  let mut oldwd: [c_char; MAXPATHLEN + 1] = [0; MAXPATHLEN + 1]; /* Fixed size is safe on MSDOS.  */
  let mut msshell: c_int;
  let mut result: c_int = -1;
  let mut inbak: c_int;
  let mut outbak: c_int;
  let mut errbak: c_int;
  let mut x: c_int = 0;
  let mut y: c_int = 0;
  let mut cmd: Lisp_Object;

  /* Get current directory as MSDOS cwd is not per-process.  */
  getwd (&raw mut oldwd[0]);

  /* If argv[0] is the shell, it might come in any lettercase.
     Since `Fmember' is case-sensitive, we need to downcase
     argv[0], even if we are on case-preserving filesystems.  */
  lowcase_argv0 = sa_stack_alloc (strlen (*argv.offset (0)) + 1) as *mut c_char;
  pa = *argv.offset (0);
  pl = lowcase_argv0;
  while *pa != 0
    {
      *pl = *pa;
      pa = pa.offset (1);
      if *pl >= b'A' as c_char && *pl <= b'Z' as c_char
	{ *pl += (b'a' - b'A') as c_char; }
      pl = pl.offset (1);
    }
  *pl = 0;		/* '\0' */

  cmd = Ffile_name_nondirectory (build_string (lowcase_argv0));
  msshell = (!NILP (Fmember (cmd, Fsymbol_value (Qmsdos_shells ())))
	     && strcmp (c"-c".as_ptr (), *argv.offset (1)) == 0) as c_int;
  if msshell != 0
    {
      saveargv1 = *argv.offset (1);
      saveargv2 = *argv.offset (2);
      *argv.offset (1) = c"/c".as_ptr () as *mut c_char;
      /* We only need to mirror slashes if a DOS shell will be invoked
	 not via `system' (which does the mirroring itself).  Yes, that
	 means DJGPP v1.x will lose here.  */
      if !(*argv.offset (2)).is_null () && !(*argv.offset (3)).is_null ()
	{
	  let mut p: *mut c_char
	    = sa_stack_alloc (strlen (*argv.offset (2)) + 1) as *mut c_char;

	  /* C's `strcpy (argv[2] = p, saveargv2)`: the store and the call (Deviations 21).  */
	  *argv.offset (2) = p;
	  strcpy (p, saveargv2);
	  while *p != 0 && isspace (*p as c_int) != 0
	    { p = p.offset (1); }
	  while *p != 0
	    {
	      if *p == b'/' as c_char
		{
		  *p = b'\\' as c_char;
		  p = p.offset (1);
		}
	      else
		{ p = p.offset (1); }
	    }
	}
    }

  /* C 3791's `goto done` is this labelled block (the `src/casefiddle.rs` 149 `'done:` shape).  */
  'done: {
  chdir (working_dir);
  inbak = dup (0);
  outbak = dup (1);
  errbak = dup (2);
  if inbak < 0 || outbak < 0 || errbak < 0
    { break 'done; } /* Allocation might fail due to lack of descriptors.  */

  if have_mouse > 0
    { mouse_get_xy (&raw mut x, &raw mut y); }

  if !noninteractive
    { dos_ttcooked (); }	/* do it here while 0 = stdin */

  dup2 (tempin, 0);
  dup2 (tempout, 1);
  dup2 (temperr, 2);

  if msshell != 0 && (*argv.offset (3)).is_null ()
    {
      /* MS-DOS native shells are too restrictive.  For starters, they
	 cannot grok commands longer than 126 characters.  In DJGPP v2
	 and later, `system' is much smarter, so we'll call it instead.  */

      let mut cmnd: *const c_char;

      /* A shell gets a single argument--its full command
	 line--whose original was saved in `saveargv2'.  */

      /* Don't let them pass empty command lines to `system', since
	 with some shells it will try to invoke an interactive shell,
	 which will hang Emacs.  */
      cmnd = saveargv2;
      while *cmnd != 0 && isspace (*cmnd as c_int) != 0
	{ cmnd = cmnd.offset (1); }
      if *cmnd != 0
	{
	  let save_env: *mut *mut c_char = SYS_ENVIRON! ();
	  let save_system_flags: c_int = __system_flags;

	  /* Request the most powerful version of `system'.  We need
	     all the help we can get to avoid calling stock DOS shells.  */
	  __system_flags = (__system_redirect
			     | __system_use_shell
			     | __system_allow_multiple_cmds
			     | __system_allow_long_cmds
			     | __system_handle_null_commands
			     | __system_emulate_chdir);

	  SYS_ENVIRON! () = envv;
	  result = system (cmnd);
	  __system_flags = save_system_flags;
	  SYS_ENVIRON! () = save_env;
	}
      else
	{ result = 0; }	/* emulate Unixy shell behavior with empty cmd line */
    }
  else
    { result = spawnve (P_WAIT, *argv.offset (0), argv, envv); }

  dup2 (inbak, 0);
  dup2 (outbak, 1);
  dup2 (errbak, 2);
  emacs_close (inbak);
  emacs_close (outbak);
  emacs_close (errbak);

  if !noninteractive
    { dos_ttraw (CURTTY! ()); }
  if have_mouse > 0
    {
      mouse_init ();
      mouse_moveto (x, y);
    }

  /* Some programs might change the meaning of the highest bit of the
     text attribute byte, so we get blinking characters instead of the
     bright background colors.  Restore that.  */
  if !noninteractive
    { bright_bg (); }
  }	/* 'done */

  chdir (&raw mut oldwd[0]);
  if msshell != 0
    {
      *argv.offset (1) = saveargv1;
      *argv.offset (2) = saveargv2;
    }
  return result;
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn croak (badfunc: *mut c_char)         // C 3876-3877
{
  fprintf (stderr, c"%s not yet implemented\r\n".as_ptr (), badfunc);
  reset_all_sys_modes ();
  exit (1);
}

/*
 * A few unimplemented functions that we silently ignore.
 */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn tcgetpgrp (fd: c_int) -> pid_t { return 0; }   // C 3887
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn setpgid (pid: c_int, pgid: c_int) -> c_int { return 0; }   // C 3888
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn setpriority (x: c_int, y: c_int, z: c_int) -> c_int { return 0; }   // C 3889
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn setsid () -> pid_t { return 0; }     // C 3890

/* Gnulib support and emulation.  */

/* C 3895-3904: `#if __DJGPP__ == 2 && __DJGPP_MINOR__ < 4` - the djgpp 2.3-and-older arm, the
   registered `djgpp_2_3` feature slice 1 named for the same condition (`include/msdos.rs` 26).
   The other arm is C's `#else`, i.e. the *system* `readlink`, which `unistd.in.rs` 39 already
   declares - so only the older arm defines anything here, and under that feature the definition
   and the mirror's declaration are the E0428 pair this ledger's Owed 2 records.  */
#[cfg (all (feature = "msdos", feature = "djgpp_2_3"))]
#[unsafe(no_mangle)]
pub unsafe fn readlink (name: *const c_char, dummy1: *mut c_char, dummy2: usize) -> ssize_t
{                                          // C 3896-3897
  /* `access' is much faster than `stat' on MS-DOS.  */
  if access (name, F_OK) == 0
    { *errno () = EINVAL; }
  return -1;
}

/* dir_pathname is set by sys_opendir and used in readlinkat and in
   fstatat, when they get a special FD of zero, which means use the
   last directory opened by opendir.  */
#[cfg (feature = "msdos")]
static mut dir_pathname: [c_char; MAXPATHLEN] = [0; MAXPATHLEN];

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn sys_opendir (dirname: *const c_char) -> *mut DIR   // C 3910-3911
{
  _fixpath (dirname, &raw mut dir_pathname[0]);
  return opendir (dirname);
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn readlinkat (fd: c_int, mut name: *const c_char, buffer: *mut c_char,
			  buffer_size: usize) -> ssize_t   // C 3917-3918
{
  /* Rely on a hack: an open directory is modeled as file descriptor 0,
     as in fstatat.  FIXME: Add proper support for readlinkat.  */
  let mut fullname: [c_char; MAXPATHLEN] = [0; MAXPATHLEN];

  if fd != AT_FDCWD
    {
      if strlen (&raw mut dir_pathname[0]) + strlen (name) + 1 >= MAXPATHLEN
	{
	  *errno () = ENAMETOOLONG;
	  return -1;
	}
      sprintf (&raw mut fullname[0], c"%s/%s".as_ptr (), &raw mut dir_pathname[0], name);
      name = &raw mut fullname[0];
    }

  return readlink (name, buffer, buffer_size);
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn openat (fd: c_int, mut path: *const c_char, oflag: c_int,
		      mode: c_int) -> c_int      // C 3939-3940
{
  /* Rely on a hack: an open directory is modeled as file descriptor 0,
     as in fstatat.  FIXME: Add proper support for openat.  */
  let mut fullname: [c_char; MAXPATHLEN] = [0; MAXPATHLEN];

  if fd != AT_FDCWD
    {
      if strlen (&raw mut dir_pathname[0]) + strlen (path) + 1 >= MAXPATHLEN
	{
	  *errno () = ENAMETOOLONG;
	  return -1;
	}
      sprintf (&raw mut fullname[0], c"%s/%s".as_ptr (), &raw mut dir_pathname[0], path);
      path = &raw mut fullname[0];
    }

  return open (path, oflag, mode);
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn fchmodat (fd: c_int, mut path: *const c_char, mode: mode_t,
			flags: c_int) -> c_int   // C 3960-3961
{
  /* Rely on a hack: an open directory is modeled as file descriptor 0,
     as in fstatat.  FIXME: Add proper support for openat.  */
  let mut fullname: [c_char; MAXPATHLEN] = [0; MAXPATHLEN];

  if fd != AT_FDCWD
    {
      if strlen (&raw mut dir_pathname[0]) + strlen (path) + 1 >= MAXPATHLEN
	{
	  *errno () = ENAMETOOLONG;
	  return -1;
	}

      sprintf (&raw mut fullname[0], c"%s/%s".as_ptr (), &raw mut dir_pathname[0], path);
      path = &raw mut fullname[0];
    }

  return chmod (path, mode);
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn careadlinkat (fd: c_int, filename: *const c_char, mut buffer: *mut c_char,
			    buffer_size: usize, alloc: *const allocator,
			    preadlinkat: Option<unsafe extern "C" fn (c_int, *const c_char,
								      *mut c_char,
								      usize) -> ssize_t>)
			    -> *mut c_char         // C 3982-3986
{
  if buffer.is_null ()
    {
      /* We don't support the fancy auto-allocation feature.  */
      if buffer_size == 0
	{ *errno () = ENOSYS; }
      else
	{ *errno () = EINVAL; }
      buffer = std::ptr::null_mut ();
    }
  else
    {
      let len: ssize_t = (preadlinkat.unwrap ()) (fd, filename, buffer, buffer_size);

      /* C's `len == buffer_size` compares a `ssize_t` with a `size_t`, i.e. as size_t
	 (Deviations 20); the `len < 0` arm short-circuits it.  */
      if len < 0 || len as usize == buffer_size
	{ buffer = std::ptr::null_mut (); }
      else
	{ *buffer.offset (len as isize) = 0; }	/* '\0' */
    }
  return buffer;
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn futimens (fd: c_int, times: *const timespec) -> c_int   // C 4009-4010
{
  let mut tm: *mut tm;
  let mut ft: ftime = std::mem::zeroed ();
  let mut t: time_t;

  block_input ();
  if (*times.offset (1)).tv_sec == UTIME_NOW
    { t = time (std::ptr::null_mut ()); }
  else
    { t = (*times.offset (1)).tv_sec; }

  tm = localtime (&raw mut t);
  ftime_bits! (tsec, &raw mut ft, min (29, ((*tm).tm_sec / 2) as isize) as c_int);
  ftime_bits! (min, &raw mut ft, (*tm).tm_min);
  ftime_bits! (hour, &raw mut ft, (*tm).tm_hour);
  ftime_bits! (day, &raw mut ft, (*tm).tm_mday);
  ftime_bits! (month, &raw mut ft, (*tm).tm_mon + 1);
  ftime_bits! (year, &raw mut ft, max (0, ((*tm).tm_year - 80) as isize) as c_int);
  unblock_input ();

  return setftime (fd, &raw mut ft);
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn utimensat (dirfd: c_int, mut pathname: *const c_char, times: *const timespec,
			 flags: c_int) -> c_int   // C 4034-4036
{
  let mut fd: c_int;
  let mut ret: c_int;
  let mut fullname: [c_char; MAXPATHLEN] = [0; MAXPATHLEN];

  /* Rely on a hack: dirfd in its current usage in Emacs is always
     AT_FDCWD.  */

  if dirfd != AT_FDCWD
    {
      if strlen (&raw mut dir_pathname[0]) + strlen (pathname) + 1 >= MAXPATHLEN
	{
	  *errno () = ENAMETOOLONG;
	  return -1;
	}
      sprintf (&raw mut fullname[0], c"%s/%s".as_ptr (), &raw mut dir_pathname[0], pathname);
      pathname = &raw mut fullname[0];
    }

  fd = open (pathname, O_WRONLY);

  if fd < 0
    { return -1; }

  ret = futimens (fd, times);
  close (fd);

  return ret;
}

/* Emulate faccessat(2).  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn faccessat (dirfd: c_int, mut path: *const c_char, mut mode: c_int,
			 mut flags: c_int) -> c_int   // C 4067-4068
{
  let mut fullname: [c_char; MAXPATHLEN] = [0; MAXPATHLEN];

  /* We silently ignore FLAGS.  */
  flags = flags;

  if dirfd != AT_FDCWD
      && !(IS_DIRECTORY_SEP (*path as c_int)
	   || IS_DEVICE_SEP (*path.offset (1) as c_int))
    {
      let lastc: c_char = dir_pathname[(strlen (&raw mut dir_pathname[0]) - 1) as usize];

      if strlen (&raw mut dir_pathname[0]) + strlen (path)
	  + (IS_DIRECTORY_SEP (lastc as c_int)) as usize >= MAXPATHLEN
	{
	  *errno () = ENAMETOOLONG;
	  return -1;
	}

      sprintf (&raw mut fullname[0], c"%s%s%s".as_ptr (), &raw mut dir_pathname[0],
	       if IS_DIRECTORY_SEP (lastc as c_int) { c"".as_ptr () }
	       else { c"/".as_ptr () }, path);
      path = &raw mut fullname[0];
    }

  if (mode & F_OK) != 0
      && IS_DIRECTORY_SEP (*path.offset ((strlen (path) - 1) as isize) as c_int)
    { mode |= D_OK; }

  return access (path, mode);
}

/* Emulate fstatat.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn fstatat (fd: c_int, mut name: *const c_char, st: *mut stat,
		       mut flags: c_int) -> c_int   // C 4100-4101
{
  /* Rely on a hack: an open directory is modeled as file descriptor 0.
     This is good enough for the current usage in Emacs, but is fragile.

     FIXME: Add proper support for fdopendir, fstatat, readlinkat.
     Gnulib does this and can serve as a model.  */
  let mut fullname: [c_char; MAXPATHLEN] = [0; MAXPATHLEN];

  flags = flags;

  if fd != AT_FDCWD
    {
      let lastc: c_char = dir_pathname[(strlen (&raw mut dir_pathname[0]) - 1) as usize];

      if strlen (&raw mut dir_pathname[0]) + strlen (name)
	  + (IS_DIRECTORY_SEP (lastc as c_int)) as usize >= MAXPATHLEN
	{
	  *errno () = ENAMETOOLONG;
	  return -1;
	}

      sprintf (&raw mut fullname[0], c"%s%s%s".as_ptr (), &raw mut dir_pathname[0],
	       if IS_DIRECTORY_SEP (lastc as c_int) { c"".as_ptr () }
	       else { c"/".as_ptr () }, name);
      name = &raw mut fullname[0];
    }

  /* C 4128-4132: `#if __DJGPP__ > 2 || __DJGPP_MINOR__ > 3` / `#else` - both arms.  The
     `#else` side is the djgpp 2.3-and-older one (the registered `djgpp_2_3`).  */
  #[cfg (not (feature = "djgpp_2_3"))]
  {
  return if (flags & AT_SYMLINK_NOFOLLOW) != 0 { lstat (name, st) } else { stat (name, st) };
  }
  #[cfg (feature = "djgpp_2_3")]
  {
  return stat (name, st);
  }
}

/* C 4135-4155: `#if __DJGPP__ == 2 && __DJGPP_MINOR__ < 4` - the same `djgpp_2_3` arm as
   C 3895's `readlink`.  */
/* Emulate the Posix unsetenv.  DJGPP v2.04 has this in the library.  */
#[cfg (all (feature = "msdos", feature = "djgpp_2_3"))]
#[unsafe(no_mangle)]
pub unsafe fn unsetenv (name: *const c_char) -> c_int   // C 4137-4138
{
  let var: *mut c_char;
  let name_len: usize;
  let retval: c_int;

  if name.is_null () || *name == 0 || !strchr (name, b'=' as c_int).is_null ()
    {
      *errno () = EINVAL;
      return -1;
    }

  /* DJGPP's 'putenv' deletes the entry if it doesn't include '='.  */
  putenv (name as *mut c_char);

  return 0;
}

/* C 4158-4242: `#ifndef HAVE_SELECT` - the MS-DOS `select` emulation.  Both arms are carried:
   the `#else` side is the system `select` (`lib/include/sys_select.rs` 39-50 already declares
   it), so nothing but this arm defines anything.  The feature `have_select` is **not registered**
   in `femacs/xtask/Cargo.toml` on this revision (the frozen table has `HAVE_SELECT 1`, so the
   other arm is the frozen configuration's) - this ledger's Owed 1 names it.  */
#[cfg (all (feature = "msdos", not (feature = "have_select")))]
include!("include/sysselect.rs");              // C 4159: `#include "sysselect.h"`

/* This yields the rest of the current time slice to the task manager.
   It should be called by any code which knows that it has nothing
   useful to do except idle.

   I don't use __dpmi_yield here, since versions of library before 2.02
   called Int 2Fh/AX=1680h there in a way that would wedge the DOS box
   on some versions of Windows 9X.  */

#[cfg (all (feature = "msdos", not (feature = "have_select")))]
#[unsafe(no_mangle)]
pub unsafe fn dos_yield_time_slice ()              // C 4169-4170
{
  let mut r: _go32_dpmi_registers = std::mem::zeroed ();

  r.x.ax = 0x1680;
  /* C's `r.x.ss = r.x.sp = r.x.flags = 0;` is a chained assignment (Deviations 21).  */
  r.x.flags = 0;
  r.x.sp = 0;
  r.x.ss = 0;
  _go32_dpmi_simulate_int (0x2f, &raw mut r);
  if r.h.al == 0x80
    { *errno () = ENOSYS; }
}

/* Only event queue is checked.  */
/* We don't have to call timer_check here
   because wait_reading_process_output takes care of that.  */
#[cfg (all (feature = "msdos", not (feature = "have_select")))]
#[unsafe(no_mangle)]
pub unsafe fn sys_select (nfds: c_int, rfds: *mut fd_set, wfds: *mut fd_set, efds: *mut fd_set,
			  timeout: *mut timespec, ignored: *mut c_void) -> c_int   // C 4184-4186
{
  let mut check_input: c_int;
  let mut t: timespec = std::mem::zeroed ();

  check_input = 0;
  if !rfds.is_null ()
    {
      check_input = FD_ISSET (0, rfds) as c_int;
      FD_ZERO (rfds);
    }
  if !wfds.is_null ()
    { FD_ZERO (wfds); }
  if !efds.is_null ()
    { FD_ZERO (efds); }

  if nfds != 1
    { emacs_abort (); }

  /* If we are looking only for the terminal, with no timeout,
     just read it and wait -- that's more efficient.  */
  if timeout.is_null ()
    {
      while !detect_input_pending ()
	{
	  dos_yield_time_slice ();
	}
    }
  else
    {
      let mut clnow: timespec;
      let mut cllast: timespec;
      let mut cldiff: timespec;

      gettime (&raw mut t);
      cllast = make_timespec (t.tv_sec, t.tv_nsec);

      while check_input == 0 || !detect_input_pending ()
	{
	  gettime (&raw mut t);
	  clnow = make_timespec (t.tv_sec, t.tv_nsec);
	  cldiff = timespec_sub (clnow, cllast);
	  /* Stop when timeout value is about to cross zero.  */
	  if timespec_cmp (*timeout, cldiff) <= 0
	    {
	      (*timeout).tv_sec = 0;
	      (*timeout).tv_nsec = 0;
	      return 0;
	    }
	  *timeout = timespec_sub (*timeout, cldiff);
	  cllast = clnow;
	  dos_yield_time_slice ();
	}
    }

  FD_SET (0, rfds);
  return 1;
}

/*
 * Define overlaid functions:
 *
 *	chdir -> sys_chdir
 *	tzset -> init_gettimeofday
 *	abort -> dos_abort
 */

/* C 4252-4279: `#ifdef chdir` - the condition tests gnulib's *macro* of that name, so the port
   spells it with the feature the macro's name lowercases to (P-86 ①); it is **not registered**
   on this revision and this ledger's Owed 1 names it.  Both arms are carried: the `#else` side
   is the mirror's own `chdir` declaration (`unistd.in.rs` 49), so only this arm defines
   anything, and C's `#undef chdir` has no counterpart (the port never spells `chdir` as a
   macro - one `## Deviations` entry).  */
#[cfg (all (feature = "msdos", feature = "chdir"))]
#[unsafe(no_mangle)]
pub unsafe fn sys_chdir (path: *const c_char) -> c_int   // C 4256-4257
{
  let mut len: c_int = strlen (path) as c_int;
  let mut tmp: *mut c_char = path as *mut c_char;

  if *tmp != 0 && *tmp.offset (1) == b':' as c_char
    {
      if getdisk () != tolower (*tmp as c_int) - b'a' as c_int
	{ setdisk (tolower (*tmp as c_int) - b'a' as c_int); }
      tmp = tmp.offset (2);	/* strip drive: KFS 1995-07-06 */
      len -= 2;
    }

  if len > 1 && (*tmp.offset ((len - 1) as isize) == b'/' as c_char)
    {
      let tmp1: *mut c_char = sa_stack_alloc (len as usize + 1) as *mut c_char;
      strcpy (tmp1, tmp);
      *tmp1.offset ((len - 1) as isize) = 0;
      tmp = tmp1;
    }
  return chdir (tmp);
}

/* C 4281-4299: `#ifdef tzset` - the same shape as C 4252's arm, for gnulib's `tzset` macro
   (`lib/include/time.in.rs` 119 declares the function the `#else` arm uses).  */
#[cfg (all (feature = "msdos", feature = "tzset"))]
#[unsafe(no_mangle)]
pub unsafe fn init_gettimeofday ()                 // C 4285-4286
{
  let mut ltm: time_t;
  let mut gtm: time_t;
  let mut lstm: *mut tm;

  tzset ();
  /* C's `ltm = gtm = time (NULL);` is a chained assignment (Deviations 21).  */
  gtm = time (std::ptr::null_mut ());
  ltm = gtm;
  /* C's `mktime (lstm = localtime (&ltm))` hoists the store (Deviations 31).  */
  lstm = localtime (&raw mut ltm);
  ltm = mktime (lstm);
  gtm = mktime (gmtime (&raw mut gtm));
  /* `time_rec` is a DJGPP `<time.h>` global with no mirror in this tree (Owed 5).  */
  time_rec.tm_hour = 99;	/* force gettimeofday to get date */
  time_rec.tm_isdst = (*lstm).tm_isdst;
  /* C's `dos_timezone_offset = time_rec.tm_gmtoff = (int)(gtm - ltm) / 60;` is a chained
     assignment (Deviations 21).  */
  time_rec.tm_gmtoff = ((gtm - ltm) as c_int / 60) as c_long;
  dos_timezone_offset = time_rec.tm_gmtoff as EMACS_INT;
}

#[cfg (feature = "msdos")]
unsafe fn msdos_abort ()                           // C 4301-4302
{
  dos_ttcooked ();
  ScreenSetCursor (10, 0);
  cputs (c"\r\n\nEmacs aborted!\r\n".as_ptr ());
  raise (SIGABRT);
  exit (2);
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn msdos_fatal_signal (sig: c_int)      // C 4311-4312
{
  if sig == SIGABRT
    { msdos_abort (); }
  else
    { raise (sig); }
}

#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe fn syms_of_msdos ()                     // C 4320-4321
{
  recent_doskeys = Fmake_vector (make_fixnum (NUM_RECENT_DOSKEYS as EMACS_INT), Qnil ());
  staticpro (&raw mut recent_doskeys);

  /* C 4326-4336: `#ifndef HAVE_X_WINDOWS` - both arms (the `#else` arm is empty in C).  */
  #[cfg (not (feature = "have_x_windows"))]
  {
  /* The following two are from xfns.c:  */
  DEFSYM! (Qreverse, "reverse");

  DEFVAR_LISP! (c"dos-unsupported-char-glyph", Vdos_unsupported_char_glyph,
		"Glyph to display instead of chars not supported by current codepage.\nThis variable is used only by MS-DOS terminals.  ");
  Vdos_unsupported_char_glyph = make_fixnum ('\u{7f}' as EMACS_INT);
  }

  defsubr (&raw mut Srecent_doskeys);
  defsubr (&raw mut Smsdos_long_file_names);
  defsubr (&raw mut Smsdos_downcase_filename);
  defsubr (&raw mut Smsdos_remember_default_colors);
  defsubr (&raw mut Smsdos_set_mouse_buttons);

  DEFSYM! (Qmsdos_menu_passive_face, "msdos-menu-passive-face");
  DEFSYM! (Qmsdos_menu_active_face, "msdos-menu-active-face");
  DEFSYM! (Qmsdos_menu_select_face, "msdos-menu-select-face");
  DEFSYM! (Qmsdos_shells, "msdos-shells");
}

/* C 4350: `#endif /* MSDOS */` - the file gate's own closing line.  Every item of this unit
   carries `#[cfg (feature = "msdos")]` instead, so the line has no counterpart; it is the end
   of `emacs/src/msdos.c` and of this port.  */
