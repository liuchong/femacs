// femacs: port of src/w16select.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  The whole
// body sits inside C 29's `#ifdef MSDOS`; `MSDOS` is `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv` 2849, and `w16select.o` belongs to the MS-DOS
// build's own object list rather than this target's: `emacs/src/Makefile.in` 306 leaves
// `MSDOS_OBJ` empty and `emacs/msdos/sed1v2.inp` 136 rewrites that line to
// `= dosfns.o msdos.o w16select.o` when the MS-DOS make files edit the generated Makefile, while
// `docs/baseline/frozen-build-sources.txt` lists no `w16select.o` for this target.  So the text
// can never be *compiled* here, `tools/extract-abi.py`'s checklist for the file is empty (P-86 ②)
// and this unit's `## C line ranges` table in `docs/ports/w16select.md` is that clause's
// substitute.
//
// **The file gate is `feature = "msdos"`** (C 29's `MSDOS`, registered in
// `femacs/xtask/Cargo.toml` 400), on every item *and on the include graph* (P-86 ⑧: an ungated
// `include!` would break the frozen compile at once).  C 31's `#include <config.h>` sits outside
// the gate, as C has it.  C 29 and C 692's `#endif` are the file's *only* two preprocessor
// conditionals - `python3 tools/check-cond-eval.py src/w16select.c` reads
// `1 conditions, 0 unknown, 0 disagreements [VERIFIED]` - so the ledger's `## Gate mapping` table
// has one row for the file itself and one for the ungated `config.h` edge; there is no
// `WINDOWSNT`/`HAVE_X_WINDOWS` style sub-gate anywhere in this file (the text never tests another
// macro).
//
// **Acquisition: this piece is `text-only` (P-86 ③⑵).**  The machine has no DJGPP toolchain
// (`which -a i586-pc-msdosdjgpp-gcc i686-pc-msdosdjgpp-gcc djgpp-gcc` is empty, and no `djgpp` or
// `go32` include directory exists under `/opt/homebrew`, `/usr/local` or `/Library`), and the
// reference tree carries none of `<dpmi.h>`, `<go32.h>`, `<sys/farptr.h>`.  There is therefore
// **no arm flip on a DJGPP target** and **no measurement of any record, member, value or
// prototype of theirs**: every one is `UNVERIFIED` (documented DJGPP text, not a reading) and the
// ledger's Owed list names the toolchain.  What this unit carries instead is the frozen `build-tu`
// reading, the P-86 ② machine-checked range table, the `#if` verdict of
// `tools/check-cond-eval.py`, a per-item C citation and the per-file error census of the `msdos`
// arm.
//
// `docs/ports/w16select.md` carries the C line ranges, the acceptance readings, every deviation
// and the items the integrator owes.  No behaviour of this file was executed anywhere: there is
// no MS-DOS target to compile the arm against and no DOS to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

/* C 1-20 is the file's opening ownership/title block, dropped (PORTING.md 0.1-19); the port's
   first line is the revision line above.  C 22-25 and C 27 are C's own notes and are carried as
   C's text at their C position.  */
/* These functions work by using WinOldAp interface.  WinOldAp
   (WINOLDAP.MOD) is a Microsoft Windows extension supporting
   "old" (character-mode) application access to Dynamic Data Exchange,
   menus, and the Windows clipboard.  */

/* Adapted to DJGPP by Eli Zaretskii <eliz@gnu.org>  */

include!("include/config.rs");                 // C 31: `#include <config.h>`

/* ----------------------------------------------------------------------------------------------
   C 32-34: `#include <dpmi.h>`, `#include <go32.h>`, `#include <sys/farptr.h>`.  None of the
   three is in the reference tree (they are the DJGPP runtime's own headers) and this machine has
   no DJGPP toolchain, so the names this unit takes from them stand here (P-88 ①: the declaration
   site is C's own `#include` line, and no file is added under `femacs/src/include/`), each with
   the header's own text above it.  **No measurement was possible** - every record, member, value
   and prototype below is `UNVERIFIED` (the headers' documented text, not a reading); the ledger's
   Owed list names the toolchain, with the layout assertions the other SDK stand-ins owe.
   ---------------------------------------------------------------------------------------------- */
/// C 32's `<dpmi.h>`: `__dpmi_regs`, the register block `__dpmi_int` passes to a real-mode
/// interrupt.  **UNVERIFIED**: the views' order and the record's size are the header's documented
/// ones, not a reading; only the members this file's own call sites touch are carried (the
/// `src/msdos.rs` 304 section's shape for the same record).
/// C: struct __dpmi_regs { union { struct { unsigned long edi, …; } d; struct { unsigned short di, …, bx, _bx, dx, _dx, cx, _cx, ax, _ax, flags, es, ds, fs, gs, ip, cs, sp, ss; } x; struct { unsigned char …, al, ah, …; } h; } _regs; };
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct DpmiRegsX
{
  pub ax: c_ushort,                     // C 110/112/133/135/147/149/183/300/304/319/322/350/355/415/417/430/434: `regs.x.ax`
  pub bx: c_ushort,                     // C 175/294/353: `regs.x.bx`
  pub cx: c_ushort,                     // C 292/432: `regs.x.cx`
  pub dx: c_ushort,                     // C 290/320/322/351/434: `regs.x.dx`
  pub si: c_ushort,                     // C 291/431: `regs.x.si`
  pub es: c_ushort,                     // C 202/293/352: `regs.x.es`
  pub flags: c_ushort,                  // C 177: `regs.x.flags`
}
/// C 32's `<dpmi.h>`: the 8-bit halves (`regs.h.…`) of the same block.  **UNVERIFIED** for the
/// same reason; only the half of `ax` this file writes is carried.
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct DpmiRegsH
{
  pub ah: c_uchar,                      // C 174/201: `regs.h.ah`
}
#[cfg (feature = "msdos")]
#[repr(C)]
pub union __dpmi_regs
{
  pub x: DpmiRegsX,
  pub h: DpmiRegsH,
}
/// C 32's `<dpmi.h>`: the real-mode interrupt call (C 111, 134, 148, 176, 203, 295, 321, 354,
/// 416).
/// C: int __dpmi_int (int, __dpmi_regs *);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn __dpmi_int (vector: c_int, regs: *mut __dpmi_regs) -> c_int;
}
/// C 33's `<go32.h>`: the info block the transfer buffer's linear address and the DOS selector
/// come from.  **UNVERIFIED** (no DJGPP toolchain); only the three members this file names are
/// carried.
/// C: struct _go32_info_block { …; unsigned long size_of_transfer_buffer; …; int selector_for_linear_memory; …; unsigned long linear_address_of_transfer_buffer; … }; extern struct _go32_info_block _go32_info_block;
#[cfg (feature = "msdos")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct _go32_info_block
{
  pub size_of_transfer_buffer: c_ulong,             // C 160: `_go32_info_block.size_of_transfer_buffer`
  pub selector_for_linear_memory: c_int,            // C 248/254/367: `_dos_ds`
  pub linear_address_of_transfer_buffer: c_ulong,   // C 161: `__tb`
}
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub static mut _go32_info_block: _go32_info_block;
}
/// C 33's `<go32.h>`: `__tb`, the transfer buffer's linear address, which C's header defines as
/// the info block's field.  A *place* macro in C, so the port carries it as a nullary
/// `macro_rules!` (the `include/msdos.rs` 635 `_dos_ds` shape).  **UNVERIFIED** for the same
/// missing toolchain.
/// C: #define __tb (_go32_info_block.linear_address_of_transfer_buffer)
#[cfg (feature = "msdos")]
macro_rules! __tb { () => { _go32_info_block.linear_address_of_transfer_buffer } }
/// C 33's `<go32.h>`: `_dos_ds`, the default selector `_farsetsel`/`_farpokeb` read through.  The
/// same *place* macro treatment; C's own header defines it unconditionally, so no `#ifndef` arm
/// exists here (unlike `src/msdos.rs` 629's fallback).
/// C: #define _dos_ds _go32_info_block.selector_for_linear_memory
#[cfg (feature = "msdos")]
macro_rules! _dos_ds { () => { _go32_info_block.selector_for_linear_memory } }
/// C 33's `<go32.h>`: the two DOS-memory transfers this file makes (C 244, 281).
/// **UNVERIFIED**: the return type is the header's documented `void`, which is why no call below
/// reads one (the `src/dosfns.rs` 288 reading of the same pair; `src/msdos.rs` 603 spells
/// `dosmemput` `-> c_int`, and the ledger records that the two units' excerpts disagree).
/// C: void dosmemput (const void *, int, int);
/// C: void dosmemget (int, int, void *);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn dosmemput (buffer: *const c_void, length: c_int, offset: c_int);
  pub fn dosmemget (offset: c_int, length: c_int, memory: *mut c_void);
}
/// C 34's `<sys/farptr.h>`: the far-memory accessors.  `<sys/farptr.h>` spells them as inlines
/// over `%fs`, so there is no library symbol; the port still declares them `unsafe extern "C"`
/// (P-88 ①) and **every prototype is UNVERIFIED** - the widths follow the names' own byte/word
/// suffixes and the call sites' conversions, not a header this machine has not got.
/// C: void _farsetsel (unsigned short);
/// C: void _farpokeb (unsigned short, unsigned long, unsigned char);
/// C: void _farnspokeb (unsigned long, unsigned char);
/// C: unsigned char _farnspeekb (unsigned long);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn _farsetsel (selector: c_ushort);
  pub fn _farpokeb (selector: c_ushort, offset: c_ulong, value: c_uchar);
  pub fn _farnspokeb (offset: c_ulong, value: c_uchar);
  pub fn _farnspeekb (offset: c_ulong) -> c_uchar;
}

/* C 35-42: the file's own mirrors.  C 35's `lisp.h` is first, as PORTING.md 0.1-14 requires
   (`lisp.rs` carries the `eassert!`/`CHECK_*` machinery the others expand).  */
#[cfg (feature = "msdos")]
include!("include/lisp.rs");                   // C 35: `#include "lisp.h"`
/* C 36's `dispextern.h` note - "frame.h seems to want this" - is C's own text and is carried on
   the line below.  */
#[cfg (feature = "msdos")]
include!("include/dispextern.rs");             // C 36: `#include "dispextern.h"	/* frame.h seems to want this */`
/* C 37's `frame.h` note - "Need this to get the X window of selected_frame" - the same.  */
#[cfg (feature = "msdos")]
include!("include/frame.rs");                  // C 37: `#include "frame.h"	/* Need this to get the X window of selected_frame */`
#[cfg (feature = "msdos")]
include!("include/blockinput.rs");             // C 38: `#include "blockinput.h"`
#[cfg (feature = "msdos")]
include!("include/character.rs");              // C 39: `#include "character.h"`
#[cfg (feature = "msdos")]
include!("include/buffer.rs");                 // C 40: `#include "buffer.h"`
#[cfg (feature = "msdos")]
include!("include/coding.rs");                 // C 41: `#include "coding.h"`
#[cfg (feature = "msdos")]
include!("include/composite.rs");              // C 42: `#include "composite.h"`

/* ----------------------------------------------------------------------------------------------
   Names this unit reaches whose owners are elsewhere in the tree and which no mirror in this
   unit's graph declares: each stands here at its use site (P-88 ①) with the header's own text
   above it.  The ledger's `## Owed` names the mirror rows.
   ---------------------------------------------------------------------------------------------- */
/// C 454/556's `decode_live_frame` is `frame.h` 1476's prototype of `frame.c`'s definition, and no
/// mirror in this unit's graph carries it (the `src/fontset.rs` 105 / `src/term.rs` 153 reading of
/// the same gap).
/// C: frame.h: `struct frame *decode_live_frame (Lisp_Object);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn decode_live_frame (frame: Lisp_Object) -> *mut frame;
}
/// C 597's `setup_coding_system` is `coding.h` 686's prototype, and `include/coding.rs` carries
/// `encode_coding_object`/`decode_coding_object` but not this one (the `src/w32select.rs` 660
/// reading of the same gap).
/// C: coding.h: `extern void setup_coding_system (Lisp_Object, struct coding_system *);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn setup_coding_system (coding_system: Lisp_Object, coding: *mut coding_system);
}
/// C 530/533/536's `message3` is `lisp.h` 4392's prototype of `xdisp.c`'s definition.
/// C: lisp.h: `extern void message3 (Lisp_Object);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn message3 (m: Lisp_Object);
}
/// C 539's `sit_for` is `lisp.h` 4373's prototype of `dispnew.c`'s definition.
/// C: lisp.h: `extern Lisp_Object sit_for (Lisp_Object, bool, int);`
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn sit_for (value: Lisp_Object, reading: bool, display: c_int) -> Lisp_Object;
}

/* C 44-46: C's note that these prototypes and constants belong on a header file.  Carried as C's
   own text.  */
/* If ever some function outside this file will need to call any
   clipboard-related function, the following prototypes and constants
   should be put on a header file.  Right now, nobody else uses them.  */

/* C 48-58: the WinOldAp clipboard formats; only `CF_OEMTEXT` is read below, and every constant
   C defines is carried.  */
#[cfg (feature = "msdos")]
pub const CF_TEXT: c_int = 0x01;               // C 48: `#define CF_TEXT      0x01`
#[cfg (feature = "msdos")]
pub const CF_BITMAP: c_int = 0x02;             // C 49: `#define CF_BITMAP    0x02`
#[cfg (feature = "msdos")]
pub const CF_METAFILE: c_int = 0x03;           // C 50: `#define CF_METAFILE  0x03`
#[cfg (feature = "msdos")]
pub const CF_SYLK: c_int = 0x04;               // C 51: `#define CF_SYLK	     0x04`
#[cfg (feature = "msdos")]
pub const CF_DIF: c_int = 0x05;                // C 52: `#define CF_DIF	     0x05`
#[cfg (feature = "msdos")]
pub const CF_TIFF: c_int = 0x06;               // C 53: `#define CF_TIFF	     0x06`
#[cfg (feature = "msdos")]
pub const CF_OEMTEXT: c_int = 0x07;            // C 54: `#define CF_OEMTEXT   0x07` (C 217/235/564/571/594/665/665)
#[cfg (feature = "msdos")]
pub const CF_DIBBITMAP: c_int = 0x08;          // C 55: `#define CF_DIBBITMAP 0x08`
#[cfg (feature = "msdos")]
pub const CF_WINWRITE: c_int = 0x80;           // C 56: `#define CF_WINWRITE  0x80`
#[cfg (feature = "msdos")]
pub const CF_DSPTEXT: c_int = 0x81;            // C 57: `#define CF_DSPTEXT   0x81`
#[cfg (feature = "msdos")]
pub const CF_DSPBITMAP: c_int = 0x82;          // C 58: `#define CF_DSPBITMAP 0x82`

/* C 60-67: C's forward declarations of this file's own functions.  Rust has no forward
   declarations (PORTING.md 0.1-12: a declaration plus a definition is `E0428`, and items are
   order-independent), so the block has no counterpart; every definition stands at its C position.
   C: unsigned identify_winoldap_version (void);
   C: unsigned open_clipboard (void);
   C: unsigned empty_clipboard (void);
   C: unsigned set_clipboard_data (unsigned, void *, unsigned, int);
   C: unsigned get_clipboard_data_size (unsigned);
   C: unsigned get_clipboard_data (unsigned, void *, unsigned, int);
   C: unsigned close_clipboard (void);
   C: unsigned clipboard_compact (unsigned); */

/* C 69-74: the segment address and the size of the low-memory transfer buffer.  C's
   `static struct { … }` has no tag, so the port names the type after the variable (P-67).  */
/// C: static struct { unsigned long size; unsigned short rm_segment; } clipboard_xfer_buf_info;
#[cfg (feature = "msdos")]
#[repr(C)]
pub struct clipboard_xfer_buf_info
{
  pub size: c_ulong,
  pub rm_segment: c_ushort,
}
#[cfg (feature = "msdos")]
static mut clipboard_xfer_buf_info: clipboard_xfer_buf_info =
  clipboard_xfer_buf_info { size: 0, rm_segment: 0 };

/* C 76-82: C's note on why the last text we put into the clipboard is stored *after* it was
   encoded, carried as C's own text.  */
/* The last text we put into the clipboard.  This is used to prevent
   passing back our own text from the clipboard, instead of using the
   kill ring.  The former is undesirable because the clipboard data
   could be MULEtilated by inappropriately chosen
   (next-)selection-coding-system.  For this reason, we must store the
   text *after* it was encoded/Unix-to-DOS-converted.  */
#[cfg (feature = "msdos")]
static mut last_clipboard_text: *mut c_uchar = core::ptr::null_mut ();

/* C 84-85: the size of the storage allocated for the stored text.  */
#[cfg (feature = "msdos")]
static mut clipboard_storage_size: size_t = 0;

/* C 87-98: C's two notes on where the interrupt documentation comes from (the Microsoft Knowledge
   Base article Q67675 and Ralf Brown's Interrupt List), carried as C's own text.  */
/* C functions to access the Windows 3.1x clipboard from DOS apps.

   The information was obtained from the Microsoft Knowledge Base,
   article Q67675 and can be found at: [broken link  -- SK 2021-09-27]
   http://www.microsoft.com/kb/developr/win_dk/q67675.htm  */

/* See also Ralf Brown's Interrupt List.

   I also seem to remember reading about this in Dr. Dobbs Journal a
   while ago, but if you knew my memory...  :-)

   Dale P. Smith <dpsm@en.com> */

/* C 100-113: `identify_winoldap_version`.  */
/* Return the WinOldAp support version, or 0x1700 if not supported.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn identify_winoldap_version () -> c_uint
{
  let mut regs: __dpmi_regs = std::mem::zeroed ();            // C 104 (zero-initialised, PORTING.md 0.1-13)

  /* Calls Int 2Fh/AX=1700h
     Return Values   AX == 1700H: Clipboard functions not available
                        <> 1700H: AL = Major version number
				  AH = Minor version number */
  regs.x.ax = 0x1700;                                         // C 110
  __dpmi_int (0x2f, &raw mut regs);                           // C 111
  return regs.x.ax as c_uint;                                 // C 112
}

/* C 115-136: `open_clipboard`.  */
/* Open the clipboard, return non-zero if successful.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn open_clipboard () -> c_uint
{
  let mut regs: __dpmi_regs = std::mem::zeroed ();            // C 119

  /* Is WINOLDAP supported?  */
  /* Kludge alert!!  If WinOldAp is not supported, we return a 0,
     which is the same as ``Clipboard already open''.  Currently,
     this is taken as an error by all the functions that use
     `open_clipboard', but if somebody someday will use that ``open''
     clipboard, they will have interesting time debugging it...  */
  if identify_winoldap_version () == 0x1700                   // C 127
    { return 0; }                                             // C 128

  /* Calls Int 2Fh/AX=1701h
     Return Values   AX == 0: Clipboard already open
			<> 0: Clipboard opened */
  regs.x.ax = 0x1701;                                         // C 133
  __dpmi_int (0x2f, &raw mut regs);                           // C 134
  return regs.x.ax as c_uint;                                 // C 135
}

/* C 138-150: `empty_clipboard`.  */
/* Empty clipboard, return non-zero if successful.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn empty_clipboard () -> c_uint
{
  let mut regs: __dpmi_regs = std::mem::zeroed ();            // C 142

  /* Calls Int 2Fh/AX=1702h
     Return Values   AX == 0: Error occurred
			<> 0: OK, Clipboard emptied */
  regs.x.ax = 0x1702;                                         // C 147
  __dpmi_int (0x2f, &raw mut regs);                           // C 148
  return regs.x.ax as c_uint;                                 // C 149
}

/* C 152-185: `alloc_xfer_buf`.  C's `static` helper stays a plain `unsafe fn`.  */
/* Ensure we have a buffer in low memory with enough memory for data
   of size WANT_SIZE.  Return the linear address of the buffer.  */
#[cfg (feature = "msdos")]
unsafe fn alloc_xfer_buf (want_size: c_uint) -> c_ulong
{
  let mut regs: __dpmi_regs = std::mem::zeroed ();            // C 157

  /* If the usual DJGPP transfer buffer is large enough, use that.  */
  if want_size as c_ulong <= _go32_info_block.size_of_transfer_buffer   // C 160
    { return __tb! () & 0xfffff; }                            // C 161

  /* Don't even try to allocate more than 1MB of memory: DOS cannot
     possibly handle that (it will overflow the BX register below).  */
  if want_size > 0xfffff                                      // C 165
    { return 0; }                                             // C 166

  /* Need size rounded up to the nearest paragraph, and in
     paragraph units (1 paragraph = 16 bytes).  */
  clipboard_xfer_buf_info.size = ((want_size + 15) >> 4) as c_ulong;   // C 170

  /* The NT DPMI host crashes us if we free DOS memory via the
     DPMI service.  Work around by calling DOS allocate/free block.  */
  regs.h.ah = 0x48;                                           // C 174
  regs.x.bx = clipboard_xfer_buf_info.size as c_ushort;       // C 175
  __dpmi_int (0x21, &raw mut regs);                           // C 176
  if (regs.x.flags & 1) != 0                                  // C 177
    {
      clipboard_xfer_buf_info.size = 0;                       // C 179
      return 0;                                               // C 180
    }

  clipboard_xfer_buf_info.rm_segment = regs.x.ax;             // C 183
  return (((clipboard_xfer_buf_info.rm_segment as c_int) << 4) & 0xfffff) as c_ulong;   // C 184
}

/* C 187-206: `free_xfer_buf`.  */
/* Free our clipboard buffer.  We always free it after use, because
   keeping it leaves less free conventional memory for subprocesses.
   The clipboard buffer tends to be large in size, because for small
   clipboard data sizes we use the DJGPP transfer buffer.  */
#[cfg (feature = "msdos")]
unsafe fn free_xfer_buf ()
{
  /* If the size is 0, we used DJGPP transfer buffer, so don't free.  */
  if clipboard_xfer_buf_info.size != 0                        // C 195
    {
      let mut regs: __dpmi_regs = std::mem::zeroed ();        // C 197

      /* The NT DPMI host crashes us if we free DOS memory via
	 the DPMI service.  Work around by calling DOS free block.  */
      regs.h.ah = 0x49;                                       // C 201
      regs.x.es = clipboard_xfer_buf_info.rm_segment;         // C 202
      __dpmi_int (0x21, &raw mut regs);                       // C 203
      clipboard_xfer_buf_info.size = 0;                       // C 204
    }
}

/* C 208-305: `set_clipboard_data`.  C's `Size` is mutated by the two `while (Size--)` loops, so
   the port takes it `mut`; C's `while (Size--)` keeps the post-decrement's wrapping value (it is
   never read after the loop), so the `loop` below reproduces the test exactly (the
   `src/xdisp.rs` 15211 shape).  */
/* Copy data into the clipboard, return zero if successful.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_clipboard_data (Format: c_uint, Data: *mut c_void, mut Size: c_uint,
                                             Raw: c_int) -> c_uint
{
  let mut regs: __dpmi_regs = std::mem::zeroed ();            // C 212
  let mut truelen: c_uint;                                    // C 213
  let mut xbuf_addr: c_ulong;                                 // C 214
  let mut buf_offset: c_ulong;                                // C 214
  let mut dp: *mut c_uchar = Data as *mut c_uchar;            // C 215
  let dstart: *mut c_uchar = dp;                              // C 215

  if Format != CF_OEMTEXT as c_uint                            // C 217
    { return 3; }                                             // C 218

  /* need to know final size after '\r' chars are inserted (the
     standard CF_OEMTEXT clipboard format uses CRLF line endings,
     while Emacs uses just LF internally).  */
  truelen = Size + 1;		/* +1 for the terminating null */   // C 223

  if Raw == 0                                                 // C 225
    {
      /* avoid using strchr because it recomputes the length every time */
      loop                                                    // C 228
	{
	  dp = memchr (dp as *const c_void, b'\n' as c_int,
		       (Size as isize - (dp as isize - dstart as isize)) as usize) as *mut c_uchar;
	  if dp.is_null ()
	    { break; }
	  truelen += 1;                                           // C 230
	  dp = dp.offset (1);                                     // C 231
	}
    }

  if clipboard_compact (truelen) < truelen                    // C 235
    { return 1; }                                             // C 236

  xbuf_addr = alloc_xfer_buf (truelen);                       // C 238
  if xbuf_addr == 0                                           // C 238
    { return 1; }                                             // C 239

  /* Move the buffer into the low memory, convert LF into CR-LF if needed.  */
  if Raw != 0                                                 // C 242
    {
      dosmemput (Data, Size as c_int, xbuf_addr as c_int);    // C 244

      /* Terminate with a null, otherwise Windows does strange things
	 when the text size is an integral multiple of 32 bytes. */
      _farpokeb (_dos_ds! () as c_ushort, xbuf_addr + Size as c_ulong, 0);   // C 248
    }
  else
    {
      dp = Data as *mut c_uchar;                              // C 252
      buf_offset = xbuf_addr;                                 // C 253
      _farsetsel (_dos_ds! () as c_ushort);                   // C 254
      loop                                                    // C 255
	{
	  let old: c_uint = Size;
	  Size -= 1;
	  if old == 0
	    { break; }
	  /* Don't allow them to put binary data into the clipboard, since
	     it will cause yanked data to be truncated at the first null.  */
	  if *dp == 0                                            // C 259
	    { return 2; }                                         // C 260
	  if *dp == b'\n' as c_uchar                              // C 261
	    {
	      _farnspokeb (buf_offset, b'\r' as c_uchar);         // C 262
	      buf_offset += 1;
	    }
	  _farnspokeb (buf_offset, *dp);                          // C 263
	  buf_offset += 1;
	  dp = dp.offset (1);
	}

      /* Terminate with a null, otherwise Windows does strange things
	 when the text size is an integral multiple of 32 bytes. */
      _farnspokeb (buf_offset, 0);                            // C 268
    }

  /* Stash away the data we are about to put into the clipboard, so we
     could later check inside get_clipboard_data whether the clipboard
     still holds our data.  */
  if clipboard_storage_size < truelen as usize                // C 274
    {
      clipboard_storage_size = truelen as usize + 100;        // C 276
      last_clipboard_text =
	xrealloc (last_clipboard_text as *mut c_void, clipboard_storage_size) as *mut c_uchar;   // C 277-278
    }
  if ! last_clipboard_text.is_null ()                         // C 280
    { dosmemget (xbuf_addr as c_int, truelen as c_int, last_clipboard_text as *mut c_void); }   // C 281

  /* Calls Int 2Fh/AX=1703h with:
	             DX = WinOldAp-Supported Clipboard format
                     ES:BX = Pointer to data
                     SI:CX = Size of data in bytes
     Return Values   AX == 0: Error occurred
			<> 0: OK.  Data copied into the Clipboard.  */
  regs.x.ax = 0x1703;                                         // C 289
  regs.x.dx = Format as c_ushort;                             // C 290
  regs.x.si = (truelen >> 16) as c_ushort;                    // C 291
  regs.x.cx = (truelen & 0xffff) as c_ushort;                 // C 292
  regs.x.es = (xbuf_addr >> 4) as c_ushort;                   // C 293
  regs.x.bx = (xbuf_addr & 15) as c_ushort;                   // C 294
  __dpmi_int (0x2f, &raw mut regs);                           // C 295

  free_xfer_buf ();                                           // C 297

  /* If the above failed, invalidate the local copy of the clipboard.  */
  if regs.x.ax == 0                                           // C 300
    { *last_clipboard_text = 0; }                             // C 301

  /* Zero means success, otherwise (1, 2, or 3) it's an error.  */
  if regs.x.ax > 0 { 0 } else { 3 }                           // C 304
}

/* C 307-323: `get_clipboard_data_size`.  */
/* Return the size of the clipboard data of format FORMAT.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_clipboard_data_size (Format: c_uint) -> c_uint
{
  let mut regs: __dpmi_regs = std::mem::zeroed ();            // C 311

  /* Calls Int 2Fh/AX=1704h with:
		     DX = WinOldAp-Supported Clipboard format
     Return Values   DX:AX == Size of the data in bytes, including any
                              headers.
                           == 0 If data in this format is not in
			   the clipboard.  */
  regs.x.ax = 0x1704;                                         // C 319
  regs.x.dx = Format as c_ushort;                             // C 320
  __dpmi_int (0x2f, &raw mut regs);                           // C 321
  return ((regs.x.dx as c_uint) << 16) | (regs.x.ax as c_uint);   // C 322
}

/* C 325-404: `get_clipboard_data`.  The two `while (Size--)` loops are the `loop`s below.  */
/* Get clipboard data, return its length.
   Warning: this doesn't check whether DATA has enough space to hold
   SIZE bytes.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_clipboard_data (Format: c_uint, Data: *mut c_void, mut Size: c_uint,
                                             Raw: c_int) -> c_uint
{
  let mut regs: __dpmi_regs = std::mem::zeroed ();            // C 331
  let mut xbuf_addr: c_ulong;                                 // C 332
  let mut dp: *mut c_uchar = Data as *mut c_uchar;            // C 333

  if Format != CF_OEMTEXT as c_uint                            // C 335
    { return 0; }                                             // C 336

  if Size == 0                                                // C 338
    { return 0; }                                             // C 339

  xbuf_addr = alloc_xfer_buf (Size);                          // C 341
  if xbuf_addr == 0                                           // C 341
    { return 0; }                                             // C 342

  /* Calls Int 2Fh/AX=1705h with:
		     DX = WinOldAp-Supported Clipboard format
		     ES:BX = Pointer to data buffer to hold data
     Return Values   AX == 0: Error occurred (or data in this format is not
                              in the clipboard)
                        <> 0: OK  */
  regs.x.ax = 0x1705;                                         // C 350
  regs.x.dx = Format as c_ushort;                             // C 351
  regs.x.es = (xbuf_addr >> 4) as c_ushort;                   // C 352
  regs.x.bx = (xbuf_addr & 15) as c_ushort;                   // C 353
  __dpmi_int (0x2f, &raw mut regs);                           // C 354
  if regs.x.ax != 0                                           // C 355
    {
      let mut null_char: c_uchar = 0;                         // C 357
      let mut xbuf_beg: c_ulong = xbuf_addr;                  // C 358

      /* If last_clipboard_text is NULL, we don't want to slow down
	 the next loop by an additional test.  */
      let mut lcdp: *mut c_uchar =
	if last_clipboard_text.is_null () { &raw mut null_char } else { last_clipboard_text };   // C 362-363

      /* Copy data from low memory, remove CR
	 characters before LF if needed.  */
      _farsetsel (_dos_ds! () as c_ushort);                   // C 367
      loop                                                    // C 368
	{
	  let old: c_uint = Size;
	  Size -= 1;
	  if old == 0
	    { break; }
	  let c: c_uchar = _farnspeekb (xbuf_addr);              // C 370
	  xbuf_addr += 1;

	  if *lcdp == c                                          // C 372
	    { lcdp = lcdp.offset (1); }                           // C 373

	  *dp = c;                                                // C 375
	  dp = dp.offset (1);
	  if c == b'\r' as c_uchar && Raw == 0 && _farnspeekb (xbuf_addr) == b'\n' as c_uchar   // C 375
	    {
	      dp = dp.offset (-1);                                // C 377
	      *dp = b'\n' as c_uchar;                             // C 378
	      dp = dp.offset (1);
	      xbuf_addr += 1;                                     // C 379
	      if *lcdp == b'\n' as c_uchar                        // C 380
		{ lcdp = lcdp.offset (1); }                        // C 381
	    }
	  /* Windows reportedly rounds up the size of clipboard data
	     (passed in SIZE) to a multiple of 32, and removes trailing
	     spaces from each line without updating SIZE.  We therefore
	     bail out when we see the first null character.  */
	  else if c == 0                                          // C 387
	    { break; }                                            // C 388
	}

      /* If the text in clipboard is identical to what we put there
	 last time set_clipboard_data was called, pretend there's no
	 data in the clipboard.  This is so we don't pass our own text
	 from the clipboard (which might be troublesome if the killed
	 text includes null characters).  */
      if ! last_clipboard_text.is_null ()
	  && xbuf_addr - xbuf_beg == (lcdp as isize - last_clipboard_text as isize) as c_ulong   // C 396-397
	{ dp = (Data as *mut c_uchar).offset (1); }               // C 398
    }

  free_xfer_buf ();                                           // C 401

  return ((dp as isize - Data as isize) - 1) as c_uint;       // C 403
}

/* C 406-418: `close_clipboard`.  */
/* Close clipboard, return non-zero if successful.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_clipboard () -> c_uint
{
  let mut regs: __dpmi_regs = std::mem::zeroed ();            // C 410

  /* Calls Int 2Fh/AX=1708h
     Return Values   AX == 0: Error occurred
                        <> 0: OK */
  regs.x.ax = 0x1708;                                         // C 415
  __dpmi_int (0x2f, &raw mut regs);                           // C 416
  return regs.x.ax as c_uint;                                 // C 417
}

/* C 420-435: `clipboard_compact`.  */
/* Compact clipboard data so that at least SIZE bytes is available.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn clipboard_compact (Size: c_uint) -> c_uint
{
  let mut regs: __dpmi_regs = std::mem::zeroed ();            // C 424

  /* Calls Int 2Fh/AX=1709H with:
                     SI:CX = Desired memory size in bytes.
     Return Values   DX:AX == Number of bytes of largest block of free memory.
                           == 0 if error or no memory  */
  regs.x.ax = 0x1709;                                         // C 430
  regs.x.si = (Size >> 16) as c_ushort;                       // C 431
  regs.x.cx = (Size & 0xffff) as c_ushort;                    // C 432
  __dpmi_int (0x2f, &raw mut regs);                           // C 433
  return ((regs.x.dx as c_uint) << 16) | (regs.x.ax as c_uint);   // C 434
}

/* C 437-442: the three messages C stores in `static char[]` arrays; the port's spelling of a C
   string literal is `c"..."` (the `src/w32select.rs` 1439 `stdfmt_name` reading), so C's
   `sizeof (X) - 1` becomes `.to_bytes ().len ()` at the three use sites.  */
/// C: static char no_mem_msg[] = "(Not enough DOS memory to put saved text into clipboard.)";
#[cfg (feature = "msdos")]
static no_mem_msg: &core::ffi::CStr =
  c"(Not enough DOS memory to put saved text into clipboard.)";
/// C: static char binary_msg[] = "(Binary characters in saved text; clipboard data not set.)";
#[cfg (feature = "msdos")]
static binary_msg: &core::ffi::CStr =
  c"(Binary characters in saved text; clipboard data not set.)";
/// C: static char system_error_msg[] = "(Clipboard interface failure; clipboard data not set.)";
#[cfg (feature = "msdos")]
static system_error_msg: &core::ffi::CStr =
  c"(Clipboard interface failure; clipboard data not set.)";

/* C 444-545: `w16-set-clipboard-data`.  C's `goto done`/`goto error`/`goto unblock` are the
   labelled blocks below (PORTING.md 2.2): the outer `'flow` is `done`, and `'unblock` is the
   `unblock:` label the `error:` arm falls into (the `src/w32select.rs` 1369 shape).  */
#[cfg (feature = "msdos")]
DEFUN! ("w16-set-clipboard-data", Fw16_set_clipboard_data, Sw16_set_clipboard_data, 1, 2, 0,
       doc: "This sets the clipboard data to the given text.  ",   // C 445
       (string: Lisp_Object, frame: Lisp_Object),
       {
  let mut ok: c_uint = 1;                                     // C 448
  let mut put_status: c_uint = 0;                             // C 448
  let mut nbytes: c_int;                                      // C 449
  let mut no_crlf_conversion: c_int;                          // C 449
  let mut src: *mut c_uchar;                                  // C 450
  let mut dst: *mut c_uchar = core::ptr::null_mut ();         // C 450

  CHECK_STRING (string);                                      // C 452

  'flow: {                                                    // C 542: `done:`
  if ! FRAME_MSDOS_P (decode_live_frame (frame))              // C 454
    { break 'flow; }                                          // C 455: `goto done`

  block_input ();                                             // C 457

  'unblock: {                                                 // C 516: `unblock:`
    if open_clipboard () == 0                                 // C 459
      { ok = 0; }                                             // C 460: `goto error` -> the shared `ok = 0` (C 514)
    else
      {
	nbytes = SBYTES (string) as c_int;                    // C 462
	src = SDATA (string);                                 // C 463

	/* Do we need to encode this text?  */
	dst = src;                                            // C 466
	while dst < src.offset (nbytes as isize)              // C 466
	  {
	    if *dst == 0 || *dst >= 0x80                      // C 468
	      { break; }                                      // C 469
	    dst = dst.offset (1);                             // C 466's increment
	  }
	if dst >= src.offset (nbytes as isize)                // C 471
	  {
	    /* No multibyte characters in text.  We need not encode it, but we
	       will have to convert it to DOS CR-LF style.  */
	    no_crlf_conversion = 0;                           // C 475
	    Vlast_coding_system_used = Qraw_text ();          // C 476
	    dst = core::ptr::null_mut ();	/* so we don't try to free a random pointer */   // C 477
	  }
	else
	  {
	    /* We must encode contents of STRING according to what
	       clipboard-coding-system specifies.  */
	    let mut coding: coding_system = std::mem::zeroed ();   // C 483 (zero-initialised, PORTING.md 0.1-13)
	    let mut coding_system: Lisp_Object =
	      if NILP (Vnext_selection_coding_system) { Vselection_coding_system }
	      else { Vnext_selection_coding_system };              // C 484-486

	    setup_coding_system (Fcheck_coding_system (coding_system), &raw mut coding);   // C 488
	    coding.dst_bytes = (nbytes * 4) as ptrdiff_t;      // C 489
	    coding.destination = xmalloc (coding.dst_bytes as usize) as *mut u8;   // C 490
	    Vnext_selection_coding_system = Qnil ();           // C 491
	    /* `coding->mode |= CODING_MODE_LAST_BLOCK;` (C 492) and
	       `coding->common_flags &= ~CODING_ANNOTATION_MASK;` (C 602) write two bitfields the
	       mirror stores in one `bits: u32` (`include/coding.rs` 239-252 spells the shifts), so
	       each writer is the masked update below; the order is C's, and unlike the
	       `src/w32select.rs` 1302 spelling the common-flags write must *not* clear the mode
	       bits the preceding store set (the ledger's `## Deviations`).  */
	    coding.bits |= (CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT;   // C 492
	    dst = coding.destination;                          // C 493
	    encode_coding_object (&raw mut coding, string, 0, 0,
				  SCHARS (string), nbytes as ptrdiff_t, Qnil ());   // C 494-495
	    no_crlf_conversion = 1;                            // C 496
	    nbytes = coding.produced as c_int;                 // C 497
	    Vlast_coding_system_used = CODING_ID_NAME (coding.id as hash_idx_t);   // C 498
	    src = dst;                                         // C 499
	  }

	ok = (empty_clipboard () != 0                          // C 502
	      && { put_status =
		     set_clipboard_data (CF_OEMTEXT as c_uint, src as *mut c_void,
					 nbytes as c_uint, no_crlf_conversion);
		   put_status == 0 }) as c_uint;               // C 503-505

	if no_crlf_conversion == 0                             // C 507
	  { close_clipboard (); }                              // C 508

	if ok != 0                                             // C 510
	  { break 'unblock; }                                  // C 510: `goto unblock`

	ok = 0;                                                // C 512-514: `error:`
      }
  }

  xfree (dst as *mut c_void);                                 // C 517
  unblock_input ();                                           // C 518

  /* Notify user if the text is too large to fit into DOS memory.
     (This will happen somewhere after 600K bytes (470K in DJGPP v1.x),
     depending on user system configuration.)  If we just silently
     fail the function, people might wonder why their text sometimes
     doesn't make it to the clipboard.  */
  if put_status != 0                                          // C 525
    {
      match put_status                                        // C 527
	{
	  1 => { message3 (make_unibyte_string (no_mem_msg.as_ptr (),
						no_mem_msg.to_bytes ().len () as ptrdiff_t)); }   // C 529-530
	  2 => { message3 (make_unibyte_string (binary_msg.as_ptr (),
						binary_msg.to_bytes ().len () as ptrdiff_t)); }   // C 532-533
	  3 => { message3 (make_unibyte_string (system_error_msg.as_ptr (),
						system_error_msg.to_bytes ().len () as ptrdiff_t)); }   // C 535-536
	  _ => {}                                             // C 528's `switch` has no `default`
	}
      sit_for (make_fixnum (2), false, 2);                    // C 539
    }
  }

  /* C 542: `done:` */
  return if ok != 0 && put_status == 0 { string } else { Qnil () };   // C 544
});

/* C 547-625: `w16-get-clipboard-data`.  C's `goto unblock`/`goto closeclip`/`goto done` are the
   labelled blocks below: the outer `'flow` is `done`, `'unblock` is the `unblock:` label and
   `'closeclip` the `closeclip:` label the body falls into.  */
#[cfg (feature = "msdos")]
DEFUN! ("w16-get-clipboard-data", Fw16_get_clipboard_data, Sw16_get_clipboard_data, 0, 1, 0,
       doc: "This gets the clipboard data in text format.  ",   // C 548
       (frame: Lisp_Object),
       {
  let mut data_size: c_uint;                                  // C 551
  let mut truelen: c_uint;                                    // C 551
  let mut htext: *mut c_uchar = core::ptr::null_mut ();       // C 552
  let mut ret: Lisp_Object = Qnil ();                         // C 553
  let mut require_decoding: c_int = 0;                        // C 554

  'flow: {                                                    // C 622: `done:`
  if ! FRAME_MSDOS_P (decode_live_frame (frame))              // C 556
    { break 'flow; }                                          // C 557: `goto done`

  block_input ();                                             // C 559

  'unblock: {                                                 // C 619: `unblock:`
    if open_clipboard () == 0                                 // C 561
      { break 'unblock; }                                     // C 562: `goto unblock`

    'closeclip: {                                             // C 616: `closeclip:`
      data_size = get_clipboard_data_size (CF_OEMTEXT as c_uint);   // C 564
      if data_size == 0                                       // C 564
	{ break 'closeclip; }                                 // C 566: `goto closeclip`
      htext = xmalloc (data_size as usize) as *mut c_uchar;   // C 565
      if htext.is_null ()                                     // C 565
	{ break 'closeclip; }                                 // C 566: `goto closeclip`

      /* need to know final size after '\r' chars are removed because
	 we can't change the string size manually, and doing an extra
	 copy is silly */
      truelen = get_clipboard_data (CF_OEMTEXT as c_uint, htext as *mut c_void, data_size, 0);   // C 571
      if truelen == 0                                         // C 571
	{ break 'closeclip; }                                 // C 572: `goto closeclip`

      /* Do we need to decode it?  */
      {
	/* If the clipboard data contains any 8-bit Latin-1 code, we
	   need to decode it.  */
	let mut i: c_int = 0;                                 // C 578

	while (i as c_uint) < truelen                         // C 580 (C's int-vs-unsigned comparison)
	  {
	    if *htext.offset (i as isize) >= 0x80             // C 582
	      {
		require_decoding = 1;                         // C 584
		break;                                        // C 585
	      }
	    i += 1;                                           // C 580's increment
	  }
      }
      if require_decoding != 0                                // C 589
	{
	  let mut coding: coding_system = std::mem::zeroed ();   // C 591
	  let mut coding_system: Lisp_Object = Vnext_selection_coding_system;   // C 592

	  truelen = get_clipboard_data (CF_OEMTEXT as c_uint, htext as *mut c_void, data_size, 1);   // C 594
	  if NILP (coding_system)                             // C 595
	    { coding_system = Vselection_coding_system; }     // C 596
	  setup_coding_system (Fcheck_coding_system (coding_system), &raw mut coding);   // C 597
	  coding.source = htext as *const u8;                 // C 598
	  coding.bits |= (CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT;   // C 599
	  /* We explicitly disable composition handling because selection
	     data should not contain any composition sequence.  */
	  coding.bits &= !(CODING_ANNOTATION_MASK << CODING_SYSTEM_COMMON_FLAGS_SHIFT);   // C 602
	  decode_coding_object (&raw mut coding, Qnil (), 0, 0,
				truelen as ptrdiff_t, truelen as ptrdiff_t, Qt ());   // C 603
	  ret = coding.dst_object;                            // C 604
	  Vlast_coding_system_used = CODING_ID_NAME (coding.id as hash_idx_t);   // C 605
	}
      else
	{
	  ret = make_unibyte_string (htext as *const c_char, truelen as ptrdiff_t);   // C 609
	  Vlast_coding_system_used = Qraw_text ();            // C 610
	}

      xfree (htext as *mut c_void);                           // C 613
      Vnext_selection_coding_system = Qnil ();                // C 614
    }

    close_clipboard ();                                       // C 617
  }

  unblock_input ();                                           // C 620
  }

  return ret;                                                 // C 624
});

/* C 627-672: `w16-selection-exists-p`.  */
/* Support checking for a clipboard selection.  */

#[cfg (feature = "msdos")]
DEFUN! ("w16-selection-exists-p", Fw16_selection_exists_p, Sw16_selection_exists_p, 0, 2, 0,
       doc: "Whether there is an owner for the given X selection.\nSELECTION should be the name of the selection in question, typically\none of the symbols `PRIMARY', `SECONDARY', or `CLIPBOARD'.  (X expects\nthese literal upper-case names.)  The symbol nil is the same as\n`PRIMARY', and t is the same as `SECONDARY'.\n\nTERMINAL should be a terminal object or a frame specifying the X\nserver to query.  If omitted or nil, that stands for the selected\nframe's display, or the first available X display.  ",   // C 631-639
       (selection: Lisp_Object, terminal: Lisp_Object),
       {
  CHECK_SYMBOL (selection);                                   // C 642

  /* Return nil for SECONDARY selection.  For PRIMARY (or nil)
     selection, check if there is some text on the kill-ring;
     for CLIPBOARD, check if the clipboard currently has valid
     text format contents.

     The test for killed text on the kill-ring emulates the Emacs
     behavior on X, where killed text is also put into X selection
     by the X interface code.  (On MSDOS, killed text is only put
     into the clipboard if we run under Windows, so we cannot check
     the clipboard alone.)  */
  if (NILP (selection) || EQ (selection, QPRIMARY ()))       // C 654
      && ! NILP (Fsymbol_value (Fintern_soft (build_string (c"kill-ring".as_ptr ()),
					      Qnil ())))      // C 655-656
    { return Qt (); }                                         // C 657

  if EQ (selection, QCLIPBOARD ())                            // C 659
    {
      let mut val: Lisp_Object = Qnil ();                     // C 661

      if open_clipboard () != 0                               // C 663
	{
	  if get_clipboard_data_size (CF_OEMTEXT as c_uint) != 0   // C 665
	    { val = Qt (); }                                      // C 666
	  close_clipboard ();                                     // C 667
	}
      return val;                                             // C 669
    }
  return Qnil ();                                             // C 671
});

/* C 681-685 carries this unit's two `DEFVAR_LISP` variables: C's `DEFVAR_*` (lisp.h 3495-3530)
   defines the object (`globals.f_##vname`), so the port defines it here and `globals.rs`'s
   declaration is shadowed (docs/status.md 3.29, the `src/cmds.rs` 489-492 row).  C zero-initialises
   the `globals` object, so the initialisers are `lisp_h_Qnil ()`.  */
// C 681: `DEFVAR_LISP ("selection-coding-system", Vselection_coding_system, ...)`.
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub static mut Vselection_coding_system: Lisp_Object = lisp_h_Qnil ();
// C 685: `DEFVAR_LISP ("next-selection-coding-system", Vnext_selection_coding_system, ...)`.
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub static mut Vnext_selection_coding_system: Lisp_Object = lisp_h_Qnil ();

/* C 674-690: `syms_of_win16select`.  */
#[cfg (feature = "msdos")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_win16select ()              // C 674-675
{
  defsubr (&raw mut Sw16_set_clipboard_data);                 // C 677
  defsubr (&raw mut Sw16_get_clipboard_data);                 // C 678
  defsubr (&raw mut Sw16_selection_exists_p);                 // C 679

  DEFVAR_LISP! (c"selection-coding-system", Vselection_coding_system,
		"SKIP: real doc in select.el.");              // C 681-682
  Vselection_coding_system = intern (c"iso-latin-1-dos".as_ptr ());   // C 683

  DEFVAR_LISP! (c"next-selection-coding-system", Vnext_selection_coding_system,
		"SKIP: real doc in select.el.");              // C 685-686
  Vnext_selection_coding_system = Qnil ();                    // C 687

  DEFSYM! (QCLIPBOARD, "CLIPBOARD");                          // C 689
}

/* C 692: `#endif /* MSDOS */` - the file gate's own closing line.  Every item of this unit
   carries `#[cfg (feature = "msdos")]` instead, so the line has no counterpart; it is the end of
   `emacs/src/w16select.c` and of this port.  */
