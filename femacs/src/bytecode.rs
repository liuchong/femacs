// femacs: port of src/bytecode.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  The two
// `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's six non-`static` definitions
// (`init_bc_thread`, `free_bc_thread`, `mark_bytecode`, `exec_byte_code`,
// `get_byte_code_arity`, `syms_of_bytecode`) are external symbols, so they carry
// `#[unsafe(no_mangle)]` (PORTING.md 0.2); the two `static` helpers (`bcall0`,
// `valid_sp`) stay plain `unsafe fn`.
//
// The computed-goto dispatcher (PORTING.md 2.3, this file's core problem): the frozen
// configuration defines `BYTE_CODE_THREADED` (the macro table has `__GNUC__` 4 and no
// `__STRICT_ANSI__`, and `!BYTE_CODE_SAFE` holds because `BYTE_CODE_SAFE` is `false`), so
// C compiles the threaded interpreter - labels plus `goto *targets[op = FETCH]`.  Rust has
// no label-as-value (`&&label`), so `BYTE_CODE_THREADED` is a structural condition and the
// port transcribes C's *switch-shaped* arm instead (decision P-03, "computed goto 只移植
// switch 版"): `CASE`/`NEXT`/`FIRST`/`CASE_DEFAULT`/`CASE_ABORT` keep their names and the
// `CASE (Bop):` call shape, but expand to `match` arms.  `NEXT` is C's "go to the next
// instruction", which in the switch-shaped arm is `break` out of the switch and round the
// `while (true)` loop again; the port's `NEXT` is the same back edge spelled `continue`.
// C's `goto L` are the switch body's shared tails; the port keeps each tail's statements
// once, at their C position, as the arms of a `match goto` run at the head of the dispatch
// loop, and each jump is `goto = L;`.  This is the "loop + state" spelling PORTING.md 2.2
// asks for, and the deviations are recorded in docs/ports/bytecode.md and
// docs/ports/macro-body-notes.tsv (the tokens C's live threaded text has that the
// switch-shaped port cannot spell).
//
// The dead regions are recorded in docs/ports/bytecode.md rather than transcribed (P-08;
// bytecode.c is a `src/` file compiled by the frozen build, so no dead text is kept as
// `#[cfg (feature = …)]`): C 29-33 (`BYTE_CODE_SAFE` is live - it is the `#else`-style
// `#ifndef` default - but C 41-44's `BYTE_CODE_THREADED` and C 45-68's `BYTE_CODE_METER`
// support are not), C 455-464 and 469-471 (`BC_REG_TOP`/`BC_REG_PC` are empty on this
// target and the `-Wclobbered` pragmas are clang-dead), C 484-486 / 560-566 / 768-780 /
// 1830-1851 (the `BYTE_CODE_METER` arms), C 594-601 (the switch-shaped `#define`s - the
// port transcribes their *meaning*, not the dead text) and C 603-616 (the threaded
// dispatch table).  Every `#if` is decided by docs/baseline/config/darwin-arm64.macros.tsv
// and `python3 tools/check-cond-eval.py src/bytecode.c` reports
// `14 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the extractor's branch
// selection is the clang-verified one; no source-header-macros.tsv row was needed.
//
// The `#include` list is C 25-29's through the mirrors those pull in: `config.h` (25),
// `lisp.h` (27, which brings `<stdio.h>`, `<setjmp.h>` and thread.h), `sysstdio.h` (28,
// `<stdio.h>`: `fprintf`, `stderr`), `buffer.h` (29) and `window.h` (30, which includes
// `dispextern.h`).  Names C declares in headers no mirror carries yet are declared at
// their use site (the terminal.rs/filelock.rs shape): `funcall_general`,
// `do_debug_on_call`, `internal_catch`, `internal_lisp_condition_case`,
// `temp_output_buffer_setup`, `temp_output_buffer_show`, `save_restriction_save`,
// `save_restriction_restore`, `restore_window_configuration`, `prog_ignore` and
// `call_debugger` (lisp.h/window.h), `push_handler` and `mark_memory` (lisp.h) and
// `fprintf` (the region-cache.rs shape).
//
// `struct bc_frame` (C 361-370) is defined here, as C defines it; `struct bc_thread_state`
// (thread.h 54-66) is not, so this unit reads `bc->fp` through the thread.h mirror's
// untyped `*mut c_void` member and casts at each use (PORTING.md P-55, the same deviation
// thread.rs records for `m_current_buffer`); the casts are spelled `(*bc).fp as *mut
// bc_frame` and are the only place the port departs from C's field type.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unreachable_code, while_true)]

/* Execution of byte code produced by bytecomp.el.
   Copyright (C) 1985-1988, 1993, 2000-2026 Free Software Foundation,
   Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

include!("include/config.rs");                 // C 25
include!("../lib/include/stdio.rs");           // C 28 sysstdio.h: FILE, fprintf, stderr
include!("../lib/include/string.rs");          // lisp.h 30: memset
include!("include/lisp.rs");                   // C 27 (brings <setjmp.h> and thread.h)
include!("include/character.rs");              // dispextern.h's MAX_CHAR, STRING_CHAR, ...
include!("include/itree.rs");                  // buffer.h 27: <itree.h>
include!("include/buffer.rs");                 // C 29
include!("include/dispextern.rs");             // window.h 23
include!("include/window.rs");                 // C 30

/* Names C declares in headers whose mirrors do not carry them yet (the terminal.rs shape);
   each keeps C's prototype in a `/// C:` line (PORTING.md P-55 for the pointer-only
   types).  */
unsafe extern "C" {
  /// C: lisp.h 4866: `Lisp_Object funcall_general (Lisp_Object fun, ptrdiff_t numargs, Lisp_Object *args);`
  pub fn funcall_general (fun: Lisp_Object, numargs: ptrdiff_t, args: *mut Lisp_Object)
    -> Lisp_Object;
  /// C: lisp.h 4865: `void do_debug_on_call (Lisp_Object code, specpdl_ref count);`
  pub fn do_debug_on_call (code: Lisp_Object, count: specpdl_ref);
  /// C: lisp.h 4803: `Lisp_Object internal_catch (Lisp_Object, Lisp_Object (*) (Lisp_Object), Lisp_Object);`
  pub fn internal_catch (tag: Lisp_Object, func: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>,
                         arg: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4804: `Lisp_Object internal_lisp_condition_case (Lisp_Object, Lisp_Object, Lisp_Object);`
  pub fn internal_lisp_condition_case (var: Lisp_Object, bodyform: Lisp_Object,
                                       handlers: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4703: `void temp_output_buffer_setup (const char *);`
  pub fn temp_output_buffer_setup (bufname: *const c_char);
  /// C: window.h 1227: `extern void temp_output_buffer_show (Lisp_Object);`
  pub fn temp_output_buffer_show (buf: Lisp_Object);
  /// C: lisp.h 4924: `extern Lisp_Object save_restriction_save (void);`
  pub fn save_restriction_save () -> Lisp_Object;
  /// C: lisp.h 4925: `extern void save_restriction_restore (Lisp_Object);`
  pub fn save_restriction_restore (data: Lisp_Object);
  /// C: window.h 1127: `extern void restore_window_configuration (Lisp_Object);`
  pub fn restore_window_configuration (configuration: Lisp_Object);
  /// C: lisp.h 4860: `extern void prog_ignore (Lisp_Object);`
  pub fn prog_ignore (body: Lisp_Object);
  /// C: lisp.h 4843: `extern Lisp_Object call_debugger (Lisp_Object arg);`
  pub fn call_debugger (arg: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4812: `extern struct handler *push_handler (Lisp_Object, enum handlertype);`
  pub fn push_handler (tag: Lisp_Object, type_: handlertype) -> *mut handler;
  /// C: lisp.h 4443: `extern void mark_memory (void const *start, void const *end);`
  pub fn mark_memory (start: *const c_void, end: *const c_void);
  /// C: lisp.h 4420: `extern void mark_object (Lisp_Object);`
  pub fn mark_object (obj: Lisp_Object);
  /// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);`
  pub fn FUNCTIONP (o: Lisp_Object) -> bool;
  /// C: lisp.h 4785: `extern Lisp_Object eval_sub (Lisp_Object);`
  pub fn eval_sub (form: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4808: `extern void set_internal (Lisp_Object, Lisp_Object, Lisp_Object, enum set_internal_bind);`
  pub fn set_internal (symbol: Lisp_Object, newval: Lisp_Object, r#let: Lisp_Object,
                       bindflag: Set_Internal_Bind);
  /// C: lisp.h 4921: `extern void record_unwind_protect_excursion (void);`
  pub fn record_unwind_protect_excursion ();
  /// C: stdio.h: `int fprintf (FILE *stream, const char *format, ...);` and
  /// `extern FILE *stderr;` (the region-cache.rs shape).
  pub fn fprintf (stream: *mut FILE, format: *const c_char, ...) -> c_int;
  #[cfg_attr (target_os = "macos", link_name = "__stderrp")]
  pub static mut stderr: *mut FILE;
}

/* Define BYTE_CODE_SAFE true to enable some minor sanity checking,
   useful for debugging the byte compiler.  It defaults to false.  */

macro_rules! BYTE_CODE_SAFE { () => { false } }

/* Define BYTE_CODE_METER to generate a byte-op usage histogram.  */
/* #define BYTE_CODE_METER */

/* C 41-44's `#define BYTE_CODE_THREADED` is the branch the port fixes (P-03): the
   switch-shaped dispatcher below is this port's interpreter, so the macro has no Rust
   counterpart and `docs/ports/excluded-macros.txt` records it (kind=config).  */

/*  Byte codes:  */

macro_rules! BYTE_CODES {
  () => {
DEFINE! (Bstack_ref, 0); /* Actually, Bstack_ref+0 is not implemented: use dup.  */
DEFINE! (Bstack_ref1, 1);
DEFINE! (Bstack_ref2, 2);
DEFINE! (Bstack_ref3, 3);
DEFINE! (Bstack_ref4, 4);
DEFINE! (Bstack_ref5, 5);
DEFINE! (Bstack_ref6, 6);
DEFINE! (Bstack_ref7, 7);
DEFINE! (Bvarref, 010);
DEFINE! (Bvarref1, 011);
DEFINE! (Bvarref2, 012);
DEFINE! (Bvarref3, 013);
DEFINE! (Bvarref4, 014);
DEFINE! (Bvarref5, 015);
DEFINE! (Bvarref6, 016);
DEFINE! (Bvarref7, 017);
DEFINE! (Bvarset, 020);
DEFINE! (Bvarset1, 021);
DEFINE! (Bvarset2, 022);
DEFINE! (Bvarset3, 023);
DEFINE! (Bvarset4, 024);
DEFINE! (Bvarset5, 025);
DEFINE! (Bvarset6, 026);
DEFINE! (Bvarset7, 027);
DEFINE! (Bvarbind, 030);
DEFINE! (Bvarbind1, 031);
DEFINE! (Bvarbind2, 032);
DEFINE! (Bvarbind3, 033);
DEFINE! (Bvarbind4, 034);
DEFINE! (Bvarbind5, 035);
DEFINE! (Bvarbind6, 036);
DEFINE! (Bvarbind7, 037);
DEFINE! (Bcall, 040);
DEFINE! (Bcall1, 041);
DEFINE! (Bcall2, 042);
DEFINE! (Bcall3, 043);
DEFINE! (Bcall4, 044);
DEFINE! (Bcall5, 045);
DEFINE! (Bcall6, 046);
DEFINE! (Bcall7, 047);
DEFINE! (Bunbind, 050);
DEFINE! (Bunbind1, 051);
DEFINE! (Bunbind2, 052);
DEFINE! (Bunbind3, 053);
DEFINE! (Bunbind4, 054);
DEFINE! (Bunbind5, 055);
DEFINE! (Bunbind6, 056);
DEFINE! (Bunbind7, 057);

DEFINE! (Bpophandler, 060);
DEFINE! (Bpushconditioncase, 061);
DEFINE! (Bpushcatch, 062);

DEFINE! (Bnth, 070);
DEFINE! (Bsymbolp, 071);
DEFINE! (Bconsp, 072);
DEFINE! (Bstringp, 073);
DEFINE! (Blistp, 074);
DEFINE! (Beq, 075);
DEFINE! (Bmemq, 076);
DEFINE! (Bnot, 077);
DEFINE! (Bcar, 0100);
DEFINE! (Bcdr, 0101);
DEFINE! (Bcons, 0102);
DEFINE! (Blist1, 0103);
DEFINE! (Blist2, 0104);
DEFINE! (Blist3, 0105);
DEFINE! (Blist4, 0106);
DEFINE! (Blength, 0107);
DEFINE! (Baref, 0110);
DEFINE! (Baset, 0111);
DEFINE! (Bsymbol_value, 0112);
DEFINE! (Bsymbol_function, 0113);
DEFINE! (Bset, 0114);
DEFINE! (Bfset, 0115);
DEFINE! (Bget, 0116);
DEFINE! (Bsubstring, 0117);
DEFINE! (Bconcat2, 0120);
DEFINE! (Bconcat3, 0121);
DEFINE! (Bconcat4, 0122);
DEFINE! (Bsub1, 0123);
DEFINE! (Badd1, 0124);
DEFINE! (Beqlsign, 0125);
DEFINE! (Bgtr, 0126);
DEFINE! (Blss, 0127);
DEFINE! (Bleq, 0130);
DEFINE! (Bgeq, 0131);
DEFINE! (Bdiff, 0132);
DEFINE! (Bnegate, 0133);
DEFINE! (Bplus, 0134);
DEFINE! (Bmax, 0135);
DEFINE! (Bmin, 0136);
DEFINE! (Bmult, 0137);

DEFINE! (Bpoint, 0140);
/* 0141 was Bmark in v17, Bsave_current_buffer in 18-19.  */
DEFINE! (Bsave_current_buffer_OBSOLETE, 0141);  /* Obsolete since 20. */
DEFINE! (Bgoto_char, 0142);
DEFINE! (Binsert, 0143);
DEFINE! (Bpoint_max, 0144);
DEFINE! (Bpoint_min, 0145);
DEFINE! (Bchar_after, 0146);
DEFINE! (Bfollowing_char, 0147);
DEFINE! (Bpreceding_char, 0150);
DEFINE! (Bcurrent_column, 0151);
DEFINE! (Bindent_to, 0152);
/* 0153 was Bscan_buffer in v17.  */
DEFINE! (Beolp, 0154);
DEFINE! (Beobp, 0155);
DEFINE! (Bbolp, 0156);
DEFINE! (Bbobp, 0157);
DEFINE! (Bcurrent_buffer, 0160);
DEFINE! (Bset_buffer, 0161);
DEFINE! (Bsave_current_buffer, 0162);
/* 0163 was Bset_mark in v17.  */
DEFINE! (Binteractive_p, 0164); /* Obsolete since Emacs-24.1.  */

DEFINE! (Bforward_char, 0165);
DEFINE! (Bforward_word, 0166);
DEFINE! (Bskip_chars_forward, 0167);
DEFINE! (Bskip_chars_backward, 0170);
DEFINE! (Bforward_line, 0171);
DEFINE! (Bchar_syntax, 0172);
DEFINE! (Bbuffer_substring, 0173);
DEFINE! (Bdelete_region, 0174);
DEFINE! (Bnarrow_to_region, 0175);
DEFINE! (Bwiden, 0176);
DEFINE! (Bend_of_line, 0177);

DEFINE! (Bconstant2, 0201);
DEFINE! (Bgoto, 0202);
DEFINE! (Bgotoifnil, 0203);
DEFINE! (Bgotoifnonnil, 0204);
DEFINE! (Bgotoifnilelsepop, 0205);
DEFINE! (Bgotoifnonnilelsepop, 0206);
DEFINE! (Breturn, 0207);
DEFINE! (Bdiscard, 0210);
DEFINE! (Bdup, 0211);

DEFINE! (Bsave_excursion, 0212);
DEFINE! (Bsave_window_excursion, 0213); /* Obsolete since Emacs-24.1.  */
DEFINE! (Bsave_restriction, 0214);
DEFINE! (Bcatch, 0215);		/* Obsolete since Emacs-25.  */

DEFINE! (Bunwind_protect, 0216);
DEFINE! (Bcondition_case, 0217);	/* Obsolete since Emacs-25.  */
DEFINE! (Btemp_output_buffer_setup, 0220); /* Obsolete since Emacs-24.1.  */
DEFINE! (Btemp_output_buffer_show, 0221);  /* Obsolete since Emacs-24.1.  */

/* 0222 was Bunbind_all, never used. */

DEFINE! (Bset_marker, 0223);
DEFINE! (Bmatch_beginning, 0224);
DEFINE! (Bmatch_end, 0225);
DEFINE! (Bupcase, 0226);
DEFINE! (Bdowncase, 0227);

DEFINE! (Bstringeqlsign, 0230);
DEFINE! (Bstringlss, 0231);
DEFINE! (Bequal, 0232);
DEFINE! (Bnthcdr, 0233);
DEFINE! (Belt, 0234);
DEFINE! (Bmember, 0235);
DEFINE! (Bassq, 0236);
DEFINE! (Bnreverse, 0237);
DEFINE! (Bsetcar, 0240);
DEFINE! (Bsetcdr, 0241);
DEFINE! (Bcar_safe, 0242);
DEFINE! (Bcdr_safe, 0243);
DEFINE! (Bnconc, 0244);
DEFINE! (Bquo, 0245);
DEFINE! (Brem, 0246);
DEFINE! (Bnumberp, 0247);
DEFINE! (Bintegerp, 0250);

/* 0252-0256 were relative jumps, apparently never used.  */

DEFINE! (BlistN, 0257);
DEFINE! (BconcatN, 0260);
DEFINE! (BinsertN, 0261);

/* Bstack_ref is code 0.  */
DEFINE! (Bstack_set,  0262);
DEFINE! (Bstack_set2, 0263);
DEFINE! (BdiscardN,   0266);

DEFINE! (Bswitch, 0267);

DEFINE! (Bconstant, 0300);
  }
}

/* C 261-266's `enum byte_code_op { #define DEFINE(name, value) name = value, BYTE_CODES
   #undef DEFINE };` cannot be spelled as a Rust enum: Rust has no X-macro in an enum body,
   so `DEFINE!` emits C's `name = value` as a file-scope constant instead, and `BYTE_CODES!
   ()` is the same list.  C's `enum` tag names nothing at its use sites, so it has no
   counterpart; the members are `ptrdiff_t` here because every use in C converts them to
   `ptrdiff_t` (`arg = op - Bvarref`, the `match` arms, `vectorp[op - Bconstant]`).  */
macro_rules! DEFINE { ($name:ident, $value:expr) => { pub const $name: ptrdiff_t = $value; } }
BYTE_CODES! ();

/* tools/cinventory.py reads the bare `BYTE_CODES` line inside C's enum body as if it were
   an enum member (the `#define DEFINE(name, value) name = value,` line defeats its member
   scanner), so the extractor's constant rule asks the port for `BYTE_CODES` with that
   value.  The name's real meaning is the macro above; nothing in this unit reads this
   record.  */
pub const BYTE_CODES: ptrdiff_t = 0;


DEFUN! ("byte-code", Fbyte_code, Sbyte_code, 3, 3, 0,
       doc: "Function used internally in byte-compiled code.\nThe first argument, BYTESTR, is a string of byte code;\nthe second, VECTOR, a vector of constants;\nthe third, MAXDEPTH, the maximum stack depth used in this function.\nIf the third argument is incorrect, Emacs may crash.",
       (bytestr: Lisp_Object, vector: Lisp_Object, maxdepth: Lisp_Object),
       {
  let mut bytestr = bytestr;   // C 315 rebinds the parameter below.

  if ! (STRINGP (bytestr) && VECTORP (vector) && FIXNATP (maxdepth))
    { error (c"Invalid byte-code".as_ptr ()); }

  if STRING_MULTIBYTE (bytestr)
    {
      /* BYTESTR must have been produced by Emacs 20.2 or earlier
	 because it produced a raw 8-bit string for byte-code and now
	 such a byte-code string is loaded as multibyte with raw 8-bit
	 characters converted to multibyte form.  Convert them back to
	 the original unibyte form.  */
      bytestr = Fstring_as_unibyte (bytestr);
    }
  let fun: Lisp_Object = CALLN! (Fmake_byte_code, Qnil (), bytestr, vector, maxdepth);
  return exec_byte_code (fun, 0, 0, std::ptr::null_mut ());
});

/* C's `static void bcall0 (Lisp_Object f)`: its address goes into
   `record_unwind_protect` below, so it is an `unsafe extern "C" fn` (PORTING.md 1.4); C's
   `calln (f)` is the port's `calln!` variadic macro.  */
unsafe extern "C" fn bcall0 (f: Lisp_Object)
{
  calln! (f);
}

/* The bytecode stack size in bytes.
   This is a fairly generous amount, but:
   - if users need more, we could allocate more, or just reserve the address
     space and allocate on demand
   - if threads are used more, then it might be a good idea to reduce the
     per-thread overhead in time and space
   - for maximum flexibility but with a small runtime penalty, we could allocate
     the stack in smaller chunks as needed
*/
macro_rules! BC_STACK_SIZE { () => { (512 * 1024 * std::mem::size_of::<Lisp_Object> ()) } }

/* Bytecode interpreter stack:

           |--------------|         --
           |fun           |           |                   ^ stack growth
           |saved_pc      |           |                   | direction
           |saved_top    -------      |
     fp--->|saved_fp     ----   |     | current frame
           |--------------|  |  |     | (called from bytecode in this example)
           |   (free)     |  |  |     |
     top-->| ...stack...  |  |  |     |
           : ...          :  |  |     |
           |incoming args |  |  |     |
           |--------------|  |  |   --
           |fun           |  |  |     |
           |saved_pc      |  |  |     |
           |saved_top     |  |  |     |
           |saved_fp      |<-   |     | previous frame
           |--------------|     |     |
           |   (free)     |     |     |
           | ...stack...  |<----      |
           : ...          :           |
           |incoming args |           |
           |--------------|         --
           :              :
*/

/* bytecode stack frame header (footer, actually) */
#[repr(C)]
pub struct bc_frame {
  pub saved_fp: *mut bc_frame,        /* previous frame pointer,
                                         NULL if bottommost frame */

  /* In a frame called directly from C, the following two members are NULL.  */
  pub saved_top: *mut Lisp_Object,           /* previous stack pointer */
  pub saved_pc: *const c_uchar,    /* previous program counter */

  pub fun: Lisp_Object,                  /* current function object */

  pub next_stack: [Lisp_Object; 0],	    /* data stack of next frame */
}
/* clang, against the frozen target (the struct's own four-pointer declaration plus
   `_Static_assert`s, `clang -std=gnu23`): `sizeof (struct bc_frame) == 32`, `saved_fp@0`,
   `saved_top@8`, `saved_pc@16`, `fun@24`, `next_stack@32`.  The port pins the same
   numbers.  The type is declared in this unit rather than in a mirror, so
   `tools/gen-layout-fields.py`'s `STRUCTS` list (which names the header a mirrored type
   comes from) does not carry it; docs/ports/bytecode.md records the measurement.  */
const _: () = assert! (std::mem::size_of::<bc_frame> () == 32);
const _: () = assert! (std::mem::offset_of! (bc_frame, saved_fp) == 0);
const _: () = assert! (std::mem::offset_of! (bc_frame, saved_top) == 8);
const _: () = assert! (std::mem::offset_of! (bc_frame, saved_pc) == 16);
const _: () = assert! (std::mem::offset_of! (bc_frame, fun) == 24);
const _: () = assert! (std::mem::offset_of! (bc_frame, next_stack) == 32);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_bc_thread (bc: *mut bc_thread_state)
{
  (*bc).stack = xmalloc (BC_STACK_SIZE! ()) as *mut c_char;
  (*bc).stack_end = (*bc).stack.offset (BC_STACK_SIZE! () as isize);
  /* Put a dummy header at the bottom to indicate the first free location.  */
  (*bc).fp = (*bc).stack as *mut c_void;
  memset ((*bc).fp, 0, std::mem::size_of::<bc_frame> ());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_bc_thread (bc: *mut bc_thread_state)
{
  xfree ((*bc).stack as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_bytecode (bc: *mut bc_thread_state)
{
  let mut fp: *mut bc_frame = (*bc).fp as *mut bc_frame;
  let mut top: *mut Lisp_Object = std::ptr::null_mut ();     /* stack pointer of topmost frame not known */
  loop
    {
      let next_fp: *mut bc_frame = (*fp).saved_fp;
      /* Only the dummy frame at the bottom has saved_fp = NULL.  */
      if next_fp.is_null ()
	{ break; }
      mark_object ((*fp).fun);
      let frame_base: *mut Lisp_Object =
	std::ptr::addr_of_mut! ((*next_fp).next_stack) as *mut Lisp_Object;
      if ! top.is_null ()
	{
	  /* The stack pointer of a frame is known: mark the part of the stack
	     above it conservatively.  This includes any outgoing arguments.  */
	  mark_memory (top.offset (1) as *const c_void, fp as *const c_void);
	  /* Mark the rest of the stack precisely.  */
	  mark_objects (frame_base, top.offset (1).offset_from (frame_base));
	}
      else
	{
	  /* The stack pointer is unknown -- mark everything conservatively.  */
	  mark_memory (frame_base as *const c_void, fp as *const c_void);
	}
      top = (*fp).saved_top;
      fp = next_fp;
    }
}

DEFUN! ("internal-stack-stats", Finternal_stack_stats, Sinternal_stack_stats,
       0, 0, 0,
       doc: "internal",
       (),
       {
  let mut bc: *mut bc_thread_state = &raw mut (*current_thread).bc;
  let mut nframes: c_int = 0;
  let mut nruns: c_int = 0;
  let mut fp: *mut bc_frame = (*bc).fp as *mut bc_frame;
  while ! fp.is_null ()
    {
      nframes += 1;
      if (*fp).saved_top.is_null ()
	{ nruns += 1; }
      fp = (*fp).saved_fp;
    }
  fprintf (stderr, c"%d stack frames, %d runs\n".as_ptr (), nframes, nruns);
  return Qnil ();
});

/* Whether a stack pointer is valid in the current frame.  */
unsafe fn valid_sp (bc: *mut bc_thread_state, sp: *mut Lisp_Object) -> bool
{
  let fp: *mut bc_frame = (*bc).fp as *mut bc_frame;
  return sp < fp as *mut Lisp_Object
    && sp.offset (1) >= std::ptr::addr_of_mut! ((*(*fp).saved_fp).next_stack)
		       as *mut Lisp_Object;
}

/* Execute the byte-code in FUN.  ARGS_TEMPLATE is the function arity
   encoded as an integer (the one in FUN is ignored), and ARGS, of
   size NARGS, should be a vector of the actual arguments.  The
   arguments in ARGS are pushed on the stack according to
   ARGS_TEMPLATE before executing FUN.  */

/* C's labels (`varref`, `varset`, `varbind`, `docall`, `dounbind`, `op_branch`,
   `pushhandler`, `exit`) have no Rust counterpart, so the port names them with C's own
   label text: `goto` holds the pending label and each arm that C sends to one sets it.
   The tail bodies live at the head of the dispatch loop, in C's order, and the jumping
   arms are the `match` arms that set `goto` (PORTING.md 2.2).  (`setup_frame:` is a Rust
   loop label, so it needs no constant.)  */
const NONE: c_int = 0;
const VARREF: c_int = 1;
const VARSET: c_int = 2;
const VARBIND: c_int = 3;
const DOCALL: c_int = 4;
const DOUNBIND: c_int = 5;
const OP_BRANCH: c_int = 6;
const PUSHHANDLER: c_int = 7;
const EXIT: c_int = 8;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exec_byte_code (mut fun: Lisp_Object, mut args_template: ptrdiff_t,
					 mut nargs: ptrdiff_t, mut args: *mut Lisp_Object)
  -> Lisp_Object
{
  let mut quitcounter: c_uchar = 1;
  let mut bc: *mut bc_thread_state = &raw mut (*current_thread).bc;

  /* Values used for the first stack record when called from C.  */
  let mut top: *mut Lisp_Object = std::ptr::null_mut ();
  let mut pc: *const c_uchar = std::ptr::null ();

  let mut bytestr: Lisp_Object = AREF (fun, CLOSURE_CODE as isize);

  /* C declares these three inside the `while (true)` body (C 549-551); the port hoists
     them because a tail runs at the head of the *next* iteration and reads them.  */
  let mut op: ptrdiff_t = 0;
  let mut arg: ptrdiff_t = 0;
  let mut type_: handlertype = CATCHER;
  let mut goto: c_int = NONE;

  /* C 271-297 defines these six macros at file scope, over the `top`/`pc` locals of
     `exec_byte_code`; `macro_rules!` hygiene resolves identifiers at the definition site,
     so the port defines them *inside* the function, after those locals, and P-57's
     definition-before-use order is kept (the first use is the argument push below).  */

  /* Fetch the next byte from the bytecode stream.  */

  macro_rules! FETCH { () => { { let v: ptrdiff_t = *pc as ptrdiff_t; pc = pc.offset (1); v } } }

  /* Fetch two bytes from the bytecode stream and make a 16-bit number
     out of them.  */

  macro_rules! FETCH2 {
    () => { { let v: ptrdiff_t = *pc as ptrdiff_t | (*pc.offset (1) as ptrdiff_t) << 8;
	      pc = pc.offset (2); v } }
  }

  /* Push X onto the execution stack.  The expression X should not
     contain TOP, to avoid competing side effects.  */

  macro_rules! PUSH { ($x:expr) => { { top = top.offset (1); *top = $x; } } }

  /* Pop a value off the execution stack.  */

  macro_rules! POP { () => { { let v = *top; top = top.offset (-1); v } } }

  /* Discard n values from the execution stack.  */

  macro_rules! DISCARD { ($n:expr) => { { top = top.offset (- ($n)); } } }

  /* Get the value which is at the top of the execution stack, but don't
     pop it.  */

  macro_rules! TOP { () => { *top } }

  'setup_frame: loop
    {
  eassert! (! STRING_MULTIBYTE (bytestr));
  eassert! (string_immovable_p (bytestr));
  /* FIXME: in debug mode (!NDEBUG, BYTE_CODE_SAFE or enabled checking),
     save the specpdl index on function entry and check that it is the same
     when returning, to detect unwind imbalances.  This would require adding
     a field to the frame header.  */

  let vector: Lisp_Object = AREF (fun, CLOSURE_CONSTANTS as isize);
  let maxdepth: Lisp_Object = AREF (fun, CLOSURE_STACK_DEPTH as isize);
  let mut const_length: ptrdiff_t = ASIZE (vector);
  let mut bytestr_length: ptrdiff_t = SCHARS (bytestr);
  let mut vectorp: *mut Lisp_Object =
    fam_base (core::ptr::addr_of_mut! ((*XVECTOR (vector)).contents));

  let max_stack: EMACS_INT = XFIXNAT (maxdepth);
  let frame_base: *mut Lisp_Object =
    fam_base (core::ptr::addr_of_mut! ((*((*bc).fp as *mut bc_frame)).next_stack));
  let fp: *mut bc_frame = frame_base.offset (max_stack) as *mut bc_frame;

  if std::ptr::addr_of_mut! ((*fp).next_stack) as *const c_char > (*bc).stack_end as *const c_char
    { error (c"Bytecode stack overflow".as_ptr ()); }

  /* Save the function object so that the bytecode and vector are
     held from removal by the GC. */
  (*fp).fun = fun;
  /* Save previous stack pointer and pc in the new frame.  If we came
     directly from outside, these will be NULL.  */
  (*fp).saved_top = top;
  (*fp).saved_pc = pc;
  (*fp).saved_fp = (*bc).fp as *mut bc_frame;
  (*bc).fp = fp as *mut c_void;

  top = frame_base.offset (-1);
  let mut bytestr_data: *const c_uchar = SDATA (bytestr);
  pc = bytestr_data;

  /* ARGS_TEMPLATE is composed of bit fields:
     bits 0..6    minimum number of arguments
     bits 7       1 iff &rest argument present
     bits 8..14   maximum number of arguments */
  let rest: bool = (args_template & 128) != 0;
  let mandatory: c_int = (args_template & 127) as c_int;
  let nonrest: ptrdiff_t = args_template >> 8;
  if ! (mandatory as ptrdiff_t <= nargs && (rest || nargs <= nonrest))
    { Fsignal (Qwrong_number_of_arguments (), list2 (Fcons (make_fixnum (mandatory as EMACS_INT),
							    make_fixnum (nonrest)),
						    make_fixnum (nargs))); }
  let pushedargs: ptrdiff_t = min (nonrest, nargs);
  { let mut i: ptrdiff_t = 0;
    while i < pushedargs
      { PUSH! (*args); i += 1; args = args.offset (1); } }
  if nonrest < nargs
    { PUSH! (Flist (nargs - nonrest, args)); }
  else
    { let mut i: ptrdiff_t = nargs - if rest { 1 } else { 0 };
      while i < nonrest
	{ PUSH! (Qnil ()); i += 1; } }

  /* C 574-601's `#ifdef BYTE_CODE_THREADED` arm defines `CASE`/`NEXT`/`FIRST`/
     `CASE_DEFAULT`/`CASE_ABORT` with `insn_` labels and `goto *targets[op = FETCH]`;
     Rust has no label-as-value, so the port transcribes the switch-shaped arm
     (`#define CASE(OP) case OP`, `#define NEXT break`, `#define FIRST switch (op)`,
     `#define CASE_DEFAULT case 255: default:`, `#define CASE_ABORT case 0`, P-03).
     C defines them inside the `while (true)` body; the port defines them once here, so
     the shared tails below and the `FIRST!` body are both after their definition
     (P-57).  The tokens the threaded text has and this text cannot spell are recorded in
     docs/ports/macro-body-notes.tsv.  */
  macro_rules! CASE { ($op:pat) => { $op } }
  macro_rules! NEXT { () => { continue } }
  macro_rules! FIRST { ($op:expr, { $($body:tt)* }) => { match $op { $($body)* } } }
  macro_rules! CASE_DEFAULT { () => { _ } }
  macro_rules! CASE_ABORT { () => { 0 } }

  'dispatch: loop
    {
      /* C's shared tails, in C's order: `varref`, `varset`, `varbind`, `docall`,
	 `dounbind`, `exit`, `op_branch` and `pushhandler`.  Each is entered with `goto` set
	 by the arm that C's text sends there.  */
      if goto != NONE
	{
	  let target: c_int = goto;
	  goto = NONE;
	  match target
	    {
	      /* C: `varref:` */
	      VARREF =>
		{
		  let v1: Lisp_Object = *vectorp.offset (arg);
		  let mut v2: Lisp_Object;
		  if ((*XBARE_SYMBOL (v1)).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
		      != SYMBOL_PLAINVAL
		      || { v2 = (*XBARE_SYMBOL (v1)).u.s.val.value; BASE_EQ (v2, Qunbound ()) }
		    { v2 = Fsymbol_value (v1); }
		  PUSH! (v2);
		  NEXT! ();
		}
	      /* C: `varset:` */
	      VARSET =>
		{
		  let sym: Lisp_Object = *vectorp.offset (arg);
		  let val: Lisp_Object = POP! ();
		  if ((*XBARE_SYMBOL (sym)).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
		      == SYMBOL_PLAINVAL
		      && ((*XBARE_SYMBOL (sym)).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK)
			 >> SYMBOL_TRAPPED_WRITE_SHIFT == 0
		    { SET_SYMBOL_VAL (XBARE_SYMBOL (sym), val); }
		  else
		    { set_internal (sym, val, Qnil (), SET_INTERNAL_SET); }
		  NEXT! ();
		}
	      /* C: `varbind:` */
	      VARBIND =>
		{
		  /* Specbind can signal and thus GC.  */
		  specbind (*vectorp.offset (arg), POP! ());
		  NEXT! ();
		}
	      /* C: `docall:` */
	      DOCALL =>
		{
		  DISCARD! (arg);
		  maybe_quit ();

		  lisp_eval_depth! () += 1;
		  if lisp_eval_depth! () as EMACS_INT > max_lisp_eval_depth
		    {
		      if max_lisp_eval_depth < 100
			{ max_lisp_eval_depth = 100; }
		      if lisp_eval_depth! () as EMACS_INT > max_lisp_eval_depth
			{ error (c"Lisp nesting exceeds `max-lisp-eval-depth'".as_ptr ()); }
		    }

		  let call_nargs: ptrdiff_t = arg;
		  let mut call_fun: Lisp_Object = TOP! ();
		  let call_args: *mut Lisp_Object = top.offset (1);

		  let count1: specpdl_ref
		    = record_in_backtrace (call_fun, call_args, call_nargs);
		  maybe_gc ();
		  if debug_on_next_call
		    { do_debug_on_call (Qlambda (), count1); }

		  let original_fun: Lisp_Object = call_fun;
		  /* Calls to symbols-with-pos don't need to be on the fast path.  */
		  if BARE_SYMBOL_P (call_fun)
		    { call_fun = (*XBARE_SYMBOL (call_fun)).u.s.function; }
		  if CLOSUREP (call_fun)
		    {
		      let template: Lisp_Object = AREF (call_fun, CLOSURE_ARGLIST as isize);
		      if FIXNUMP (template)
			{
			  /* Fast path for lexbound functions.  */
			  fun = call_fun;
			  bytestr = AREF (call_fun, CLOSURE_CODE as isize);
			  args_template = XFIXNUM (template);
			  nargs = call_nargs;
			  args = call_args;
			  /* C: `goto setup_frame` */
			  continue 'setup_frame;
			}
		    }

		  let mut val: Lisp_Object;
		  if SUBRP (call_fun) && ! NATIVE_COMP_FUNCTION_DYNP (call_fun)
		    { val = funcall_subr (XSUBR (call_fun), call_nargs, call_args); }
		  else
		    { val = funcall_general (original_fun, call_nargs, call_args); }

		  lisp_eval_depth! () -= 1;
		  if backtrace_debug_on_exit (specpdl_ptr! ().offset (-1))
		    { val = call_debugger (list2 (Qexit (), val)); }
		  specpdl_ptr! () = specpdl_ptr! ().offset (-1);

		  *top = val;
		  NEXT! ();
		}
	      /* C: `dounbind:` */
	      DOUNBIND =>
		{
		  unbind_to (specpdl_ref_add (SPECPDL_INDEX (), -arg), Qnil ());
		  NEXT! ();
		}
	      /* C: `exit:` (the `goto exit` in `Breturn`) */
	      EXIT =>
		{
		  break 'dispatch;
		}
	      /* C: `op_branch:` */
	      OP_BRANCH =>
		{
		  if BYTE_CODE_SAFE! () && ! (arg >= 0 && arg < bytestr_length)
		    { emacs_abort (); }
		  let new_pc: *const c_uchar = bytestr_data.offset (arg);
		  quitcounter = quitcounter.wrapping_add ((new_pc < pc) as c_uchar);
		  if quitcounter == 0
		    {
		      quitcounter = 1;
		      maybe_gc ();
		      maybe_quit ();
		    }
		  pc = new_pc;
		  NEXT! ();
		}
	      /* C: `pushhandler:` */
	      PUSHHANDLER =>
		{
		  let c: *mut handler = push_handler (POP! (), type_);
		  (*c).bytecode_dest = FETCH2! () as c_int;
		  (*c).bytecode_top = top;

		  if sys_setjmp (&raw mut (*c).jmp as *mut jmp_buf) != 0
		    {
		      /* No need to restore old quitcounter; just check at the next
			 backward branch.  */
		      quitcounter = (-1 as c_int) as c_uchar;
		      let c: *mut handler = handlerlist! ();
		      handlerlist! () = (*c).next;
		      top = (*c).bytecode_top;
		      arg = (*c).bytecode_dest as ptrdiff_t;
		      bc = &raw mut (*current_thread).bc;
		      let fp: *mut bc_frame = (*bc).fp as *mut bc_frame;

		      let fun: Lisp_Object = (*fp).fun;
		      let bytestr: Lisp_Object = AREF (fun, CLOSURE_CODE as isize);
		      let vector: Lisp_Object = AREF (fun, CLOSURE_CONSTANTS as isize);
		      bytestr_data = SDATA (bytestr);
		      vectorp = fam_base (core::ptr::addr_of_mut! ((*XVECTOR (vector)).contents));
		      if BYTE_CODE_SAFE! ()
			{
			  /* Only required for checking, not for execution.  */
			  const_length = ASIZE (vector);
			  bytestr_length = SCHARS (bytestr);
			}
		      pc = bytestr_data;
		      PUSH! ((*c).val);
		      /* C: `goto op_branch` */
		      goto = OP_BRANCH;
		      NEXT! ();
		    }

		  NEXT! ();
		}
	      _ => { }
	    }
	  continue 'dispatch;
	}

      if BYTE_CODE_SAFE! () && ! valid_sp (bc, top)
	{ emacs_abort (); }

      op = FETCH! ();

      /* The interpreter can be compiled one of two ways: as an
	 ordinary switch-based interpreter, or as a threaded
	 interpreter.  The threaded interpreter relies on GCC's
	 computed goto extension, so it is not available everywhere.
	 Threading provides a performance boost.  These macros are how
	 we allow the code to be compiled both ways.  */
      FIRST! (op,
	{
	CASE! (Bvarref7) => { arg = FETCH2! (); goto = VARREF; }
	CASE! (Bvarref)
	  | CASE! (Bvarref1)
	  | CASE! (Bvarref2)
	  | CASE! (Bvarref3)
	  | CASE! (Bvarref4)
	  | CASE! (Bvarref5) => { arg = op - Bvarref; goto = VARREF; }
	/* This seems to be the most frequently executed byte-code
	   among the Bvarref's, so avoid a goto here.  */
	CASE! (Bvarref6) => { arg = FETCH! (); goto = VARREF; }
	CASE! (Bgotoifnil) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    arg = FETCH2! ();
	    if NILP (v1)
	      { goto = OP_BRANCH; NEXT! (); }
	    NEXT! ();
	  }
	CASE! (Bcar) =>
	  {
	    if CONSP (TOP! ())
	      { *top = XCAR (TOP! ()); }
	    else if ! NILP (TOP! ())
	      {
		record_in_backtrace (Qcar (), top, 1);
		wrong_type_argument (Qlistp (), TOP! ());
	      }
	    NEXT! ();
	  }
	CASE! (Beq) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = if EQ (v1, TOP! ()) { Qt () } else { Qnil () };
	    NEXT! ();
	  }
	CASE! (Bmemq) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fmemq (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bcdr) =>
	  {
	    if CONSP (TOP! ())
	      { *top = XCDR (TOP! ()); }
	    else if ! NILP (TOP! ())
	      {
		record_in_backtrace (Qcdr (), top, 1);
		wrong_type_argument (Qlistp (), TOP! ());
	      }
	    NEXT! ();
	  }
	CASE! (Bvarset)
	  | CASE! (Bvarset1)
	  | CASE! (Bvarset2)
	  | CASE! (Bvarset3)
	  | CASE! (Bvarset4)
	  | CASE! (Bvarset5) => { arg = op - Bvarset; goto = VARSET; }
	CASE! (Bvarset7) => { arg = FETCH2! (); goto = VARSET; }
	CASE! (Bvarset6) => { arg = FETCH! (); goto = VARSET; }
	CASE! (Bdup) =>
	  {
	    let v1: Lisp_Object = TOP! ();
	    PUSH! (v1);
	    NEXT! ();
	  }
	/* ------------------ */
	CASE! (Bvarbind6) => { arg = FETCH! (); goto = VARBIND; }
	CASE! (Bvarbind7) => { arg = FETCH2! (); goto = VARBIND; }
	CASE! (Bvarbind)
	  | CASE! (Bvarbind1)
	  | CASE! (Bvarbind2)
	  | CASE! (Bvarbind3)
	  | CASE! (Bvarbind4)
	  | CASE! (Bvarbind5) => { arg = op - Bvarbind; goto = VARBIND; }
	CASE! (Bcall6) => { arg = FETCH! (); goto = DOCALL; }
	CASE! (Bcall7) => { arg = FETCH2! (); goto = DOCALL; }
	CASE! (Bcall)
	  | CASE! (Bcall1)
	  | CASE! (Bcall2)
	  | CASE! (Bcall3)
	  | CASE! (Bcall4)
	  | CASE! (Bcall5) => { arg = op - Bcall; goto = DOCALL; }
	CASE! (Bunbind6) => { arg = FETCH! (); goto = DOUNBIND; }
	CASE! (Bunbind7) => { arg = FETCH2! (); goto = DOUNBIND; }
	CASE! (Bunbind)
	  | CASE! (Bunbind1)
	  | CASE! (Bunbind2)
	  | CASE! (Bunbind3)
	  | CASE! (Bunbind4)
	  | CASE! (Bunbind5) => { arg = op - Bunbind; goto = DOUNBIND; }
	CASE! (Bgoto) =>
	  {
	    arg = FETCH2! ();
	    goto = OP_BRANCH;
	  }
	CASE! (Bgotoifnonnil) =>
	  {
	    arg = FETCH2! ();
	    if ! NILP (POP! ())
	      { goto = OP_BRANCH; NEXT! (); }
	    NEXT! ();
	  }
	CASE! (Bgotoifnilelsepop) =>
	  {
	    arg = FETCH2! ();
	    if NILP (TOP! ())
	      { goto = OP_BRANCH; NEXT! (); }
	    DISCARD! (1);
	    NEXT! ();
	  }
	CASE! (Bgotoifnonnilelsepop) =>
	  {
	    arg = FETCH2! ();
	    if ! NILP (TOP! ())
	      { goto = OP_BRANCH; NEXT! (); }
	    DISCARD! (1);
	    NEXT! ();
	  }
	CASE! (Breturn) =>
	  {
	    let saved_top: *mut Lisp_Object = (*( (*bc).fp as *mut bc_frame )).saved_top;
	    if ! saved_top.is_null ()
	      {
		let mut val: Lisp_Object = TOP! ();
		lisp_eval_depth! () -= 1;
		if backtrace_debug_on_exit (specpdl_ptr! ().offset (-1))
		  { val = call_debugger (list2 (Qexit (), val)); }
		specpdl_ptr! () = specpdl_ptr! ().offset (-1);
		top = saved_top;
		pc = (*( (*bc).fp as *mut bc_frame )).saved_pc;
		let fp: *mut bc_frame = (*( (*bc).fp as *mut bc_frame )).saved_fp;
		(*bc).fp = fp as *mut c_void;
		let fun: Lisp_Object = (*fp).fun;
		let bytestr: Lisp_Object = AREF (fun, CLOSURE_CODE as isize);
		let vector: Lisp_Object = AREF (fun, CLOSURE_CONSTANTS as isize);
		bytestr_data = SDATA (bytestr);
		vectorp = fam_base (core::ptr::addr_of_mut! ((*XVECTOR (vector)).contents));
		if BYTE_CODE_SAFE! ()
		  {
		    /* Only required for checking, not for execution.  */
		    const_length = ASIZE (vector);
		    bytestr_length = SCHARS (bytestr);
		  }
		*top = val;
		NEXT! ();
	      }
	    else
	      { /* C: `goto exit` */
		goto = EXIT;
		NEXT! (); }
	  }
	CASE! (Bdiscard) =>
	  {
	    DISCARD! (1);
	    NEXT! ();
	  }
	CASE! (Bconstant2) =>
	  {
	    PUSH! (*vectorp.offset (FETCH2! ()));
	    NEXT! ();
	  }
	CASE! (Bsave_excursion) =>
	  {
	    record_unwind_protect_excursion ();
	    NEXT! ();
	  }
	CASE! (Bsave_current_buffer_OBSOLETE) /* Obsolete since 20.  */
	  | CASE! (Bsave_current_buffer) =>
	  {
	    record_unwind_current_buffer ();
	    NEXT! ();
	  }
	CASE! (Bsave_window_excursion) => /* Obsolete since 24.1.  */
	  {
	    let count1: specpdl_ref = SPECPDL_INDEX ();
	    record_unwind_protect (Some (restore_window_configuration),
				   Fcurrent_window_configuration (Qnil ()));
	    *top = Fprogn (TOP! ());
	    unbind_to (count1, TOP! ());
	    NEXT! ();
	  }
	CASE! (Bsave_restriction) =>
	  {
	    record_unwind_protect (Some (save_restriction_restore),
				   save_restriction_save ());
	    NEXT! ();
	  }
	CASE! (Bcatch) =>		/* Obsolete since 25.  */
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = internal_catch (TOP! (), Some (eval_sub), v1);
	    NEXT! ();
	  }
	CASE! (Bpushcatch) =>	/* New in 24.4.  */
	  {
	    type_ = CATCHER;
	    goto = PUSHHANDLER;
	  }
	CASE! (Bpushconditioncase) => /* New in 24.4.  */
	  {
	    type_ = CONDITION_CASE;
	    goto = PUSHHANDLER;
	  }
	CASE! (Bpophandler) =>	/* New in 24.4.  */
	  {
	    handlerlist! () = (*handlerlist! ()).next;
	    NEXT! ();
	  }
	CASE! (Bunwind_protect) =>	/* FIXME: avoid closure for lexbind.  */
	  {
	    let handler: Lisp_Object = POP! ();
	    /* Support for a function here is new in 24.4.  */
	    record_unwind_protect (if FUNCTIONP (handler) { Some (bcall0) }
				   else { Some (prog_ignore) },
				   handler);
	    NEXT! ();
	  }
	CASE! (Bcondition_case) =>		/* Obsolete since 25.  */
	  {
	    let handlers: Lisp_Object = POP! ();
	    let body: Lisp_Object = POP! ();
	    *top = internal_lisp_condition_case (TOP! (), body, handlers);
	    NEXT! ();
	  }
	CASE! (Btemp_output_buffer_setup) => /* Obsolete since 24.1.  */
	  {
	    CHECK_STRING (TOP! ());
	    temp_output_buffer_setup (SSDATA (TOP! ()));
	    *top = Vstandard_output;
	    NEXT! ();
	  }
	CASE! (Btemp_output_buffer_show) => /* Obsolete since 24.1.  */
	  {
	    let v1: Lisp_Object = POP! ();
	    temp_output_buffer_show (TOP! ());
	    *top = v1;
	    /* pop binding of standard-output */
	    unbind_to (specpdl_ref_add (SPECPDL_INDEX (), -1), Qnil ());
	    NEXT! ();
	  }
	CASE! (Bnth) =>
	  {
	    let mut v2: Lisp_Object = POP! (); let v1: Lisp_Object = TOP! ();
	    if RANGED_FIXNUMP (0, v1, SMALL_LIST_LEN_MAX as EMACS_INT)
	      {
		let mut n: EMACS_INT = XFIXNUM (v1);
		while 0 < n && CONSP (v2)
		  { v2 = XCDR (v2); n -= 1; }
		if CONSP (v2)
		  { *top = XCAR (v2); }
		else if NILP (v2)
		  { *top = Qnil (); }
		else
		  {
		    record_in_backtrace (Qnth (), top, 2);
		    wrong_type_argument (Qlistp (), v2);
		  }
	      }
	    else
	      { *top = Fnth (v1, v2); }
	    NEXT! ();
	  }
	CASE! (Bsymbolp) =>
	  {
	    *top = if SYMBOLP (TOP! ()) { Qt () } else { Qnil () };
	    NEXT! ();
	  }
	CASE! (Bconsp) =>
	  {
	    *top = if CONSP (TOP! ()) { Qt () } else { Qnil () };
	    NEXT! ();
	  }
	CASE! (Bstringp) =>
	  {
	    *top = if STRINGP (TOP! ()) { Qt () } else { Qnil () };
	    NEXT! ();
	  }
	CASE! (Blistp) =>
	  {
	    *top = if CONSP (TOP! ()) || NILP (TOP! ()) { Qt () } else { Qnil () };
	    NEXT! ();
	  }
	CASE! (Bnot) =>
	  {
	    *top = if NILP (TOP! ()) { Qt () } else { Qnil () };
	    NEXT! ();
	  }
	CASE! (Bcons) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fcons (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Blist1) =>
	  {
	    *top = list1 (TOP! ());
	    NEXT! ();
	  }
	CASE! (Blist2) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = list2 (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Blist3) =>
	  {
	    DISCARD! (2);
	    *top = list3 (TOP! (), *top.offset (1), *top.offset (2));
	    NEXT! ();
	  }
	CASE! (Blist4) =>
	  {
	    DISCARD! (3);
	    *top = list4 (TOP! (), *top.offset (1), *top.offset (2), *top.offset (3));
	    NEXT! ();
	  }
	CASE! (BlistN) =>
	  {
	    let n: ptrdiff_t = FETCH! ();
	    DISCARD! (n - 1);
	    *top = Flist (n, top);
	    NEXT! ();
	  }
	CASE! (Blength) =>
	  {
	    *top = Flength (TOP! ());
	    NEXT! ();
	  }
	CASE! (Baref) =>
	  {
	    let idxval: Lisp_Object = POP! ();
	    let arrayval: Lisp_Object = TOP! ();
	    if ! FIXNUMP (idxval)
	      {
		record_in_backtrace (Qaref (), top, 2);
		wrong_type_argument (Qfixnump (), idxval);
	      }
	    let mut size: ptrdiff_t = 0;
	    if (VECTORP (arrayval) && { size = ASIZE (arrayval); true })
		|| (RECORDP (arrayval) && { size = PVSIZE (arrayval); true })
	      {
		let idx: ptrdiff_t = XFIXNUM (idxval);
		if idx >= 0 && idx < size
		  { *top = AREF (arrayval, idx); }
		else
		  {
		    record_in_backtrace (Qaref (), top, 2);
		    args_out_of_range (arrayval, idxval);
		  }
	      }
	    else
	      { *top = Faref (arrayval, idxval); }
	    NEXT! ();
	  }
	CASE! (Baset) =>
	  {
	    let newelt: Lisp_Object = POP! ();
	    let idxval: Lisp_Object = POP! ();
	    let arrayval: Lisp_Object = TOP! ();
	    if ! FIXNUMP (idxval)
	      {
		record_in_backtrace (Qaset (), top, 3);
		wrong_type_argument (Qfixnump (), idxval);
	      }
	    let mut size: ptrdiff_t = 0;
	    if (VECTORP (arrayval) && { size = ASIZE (arrayval); true })
		|| (RECORDP (arrayval) && { size = PVSIZE (arrayval); true })
	      {
		let idx: ptrdiff_t = XFIXNUM (idxval);
		if idx >= 0 && idx < size
		  {
		    ASET (arrayval, idx, newelt);
		    *top = newelt;
		  }
		else
		  {
		    record_in_backtrace (Qaset (), top, 3);
		    args_out_of_range (arrayval, idxval);
		  }
	      }
	    else
	      { *top = Faset (arrayval, idxval, newelt); }
	    NEXT! ();
	  }
	CASE! (Bsymbol_value) =>
	  {
	    *top = Fsymbol_value (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bsymbol_function) =>
	  {
	    *top = Fsymbol_function (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bset) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fset (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bfset) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Ffset (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bget) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fget (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bsubstring) =>
	  {
	    let v2: Lisp_Object = POP! (); let v1: Lisp_Object = POP! ();
	    *top = Fsubstring (TOP! (), v1, v2);
	    NEXT! ();
	  }
	CASE! (Bconcat2) =>
	  {
	    DISCARD! (1);
	    *top = Fconcat (2, top);
	    NEXT! ();
	  }
	CASE! (Bconcat3) =>
	  {
	    DISCARD! (2);
	    *top = Fconcat (3, top);
	    NEXT! ();
	  }
	CASE! (Bconcat4) =>
	  {
	    DISCARD! (3);
	    *top = Fconcat (4, top);
	    NEXT! ();
	  }
	CASE! (BconcatN) =>
	  {
	    let n: ptrdiff_t = FETCH! ();
	    DISCARD! (n - 1);
	    *top = Fconcat (n, top);
	    NEXT! ();
	  }
	CASE! (Bsub1) =>
	  {
	    *top = if FIXNUMP (TOP! ()) && XFIXNUM (TOP! ()) != MOST_NEGATIVE_FIXNUM
	      { make_fixnum (XFIXNUM (TOP! ()) - 1) }
	    else
	      { Fsub1 (TOP! ()) };
	    NEXT! ();
	  }
	CASE! (Badd1) =>
	  {
	    *top = if FIXNUMP (TOP! ()) && XFIXNUM (TOP! ()) != MOST_POSITIVE_FIXNUM
	      { make_fixnum (XFIXNUM (TOP! ()) + 1) }
	    else
	      { Fadd1 (TOP! ()) };
	    NEXT! ();
	  }
	CASE! (Beqlsign) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    if FIXNUMP (v1) && FIXNUMP (v2)
	      { *top = if BASE_EQ (v1, v2) { Qt () } else { Qnil () }; }
	    else
	      { *top = if arithcompare (v1, v2) & (Cmp_EQ as cmp_bits_t) != 0 { Qt () } else { Qnil () }; }
	    NEXT! ();
	  }
	CASE! (Bgtr) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    if FIXNUMP (v1) && FIXNUMP (v2)
	      { *top = if XFIXNUM (v1) > XFIXNUM (v2) { Qt () } else { Qnil () }; }
	    else
	      { *top = if arithcompare (v1, v2) & (Cmp_GT as cmp_bits_t) != 0 { Qt () } else { Qnil () }; }
	    NEXT! ();
	  }
	CASE! (Blss) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    if FIXNUMP (v1) && FIXNUMP (v2)
	      { *top = if XFIXNUM (v1) < XFIXNUM (v2) { Qt () } else { Qnil () }; }
	    else
	      { *top = if arithcompare (v1, v2) & (Cmp_LT as cmp_bits_t) != 0 { Qt () } else { Qnil () }; }
	    NEXT! ();
	  }
	CASE! (Bleq) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    if FIXNUMP (v1) && FIXNUMP (v2)
	      { *top = if XFIXNUM (v1) <= XFIXNUM (v2) { Qt () } else { Qnil () }; }
	    else
	      { *top = if arithcompare (v1, v2) & ((Cmp_LT | Cmp_EQ) as cmp_bits_t) != 0 { Qt () } else { Qnil () }; }
	    NEXT! ();
	  }
	CASE! (Bgeq) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    if FIXNUMP (v1) && FIXNUMP (v2)
	      { *top = if XFIXNUM (v1) >= XFIXNUM (v2) { Qt () } else { Qnil () }; }
	    else
	      { *top = if arithcompare (v1, v2) & ((Cmp_GT | Cmp_EQ) as cmp_bits_t) != 0 { Qt () } else { Qnil () }; }
	    NEXT! ();
	  }
	CASE! (Bdiff) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    let mut res: EMACS_INT = 0;
	    if FIXNUMP (v1) && FIXNUMP (v2)
		&& { res = XFIXNUM (v1).wrapping_sub (XFIXNUM (v2));
		     ! FIXNUM_OVERFLOW_P (res) }
	      { *top = make_fixnum (res); }
	    else
	      { *top = Fminus (2, top); }
	    NEXT! ();
	  }
	CASE! (Bnegate) =>
	  {
	    *top = if FIXNUMP (TOP! ()) && XFIXNUM (TOP! ()) != MOST_NEGATIVE_FIXNUM
	      { make_fixnum (- XFIXNUM (TOP! ())) }
	    else
	      { Fminus (1, top) };
	    NEXT! ();
	  }
	CASE! (Bplus) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    let mut res: EMACS_INT = 0;
	    if FIXNUMP (v1) && FIXNUMP (v2)
		&& { res = XFIXNUM (v1).wrapping_add (XFIXNUM (v2));
		     ! FIXNUM_OVERFLOW_P (res) }
	      { *top = make_fixnum (res); }
	    else
	      { *top = Fplus (2, top); }
	    NEXT! ();
	  }
	CASE! (Bmax) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    if FIXNUMP (v1) && FIXNUMP (v2)
	      {
		if XFIXNUM (v2) > XFIXNUM (v1)
		  { *top = v2; }
	      }
	    else
	      { *top = Fmax (2, top); }
	    NEXT! ();
	  }
	CASE! (Bmin) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    if FIXNUMP (v1) && FIXNUMP (v2)
	      {
		if XFIXNUM (v2) < XFIXNUM (v1)
		  { *top = v2; }
	      }
	    else
	      { *top = Fmin (2, top); }
	    NEXT! ();
	  }
	CASE! (Bmult) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    let mut res: intmax_t = 0;
	    if FIXNUMP (v1) && FIXNUMP (v2)
		&& ! { let r = (XFIXNUM (v1) as intmax_t).overflowing_mul (XFIXNUM (v2) as intmax_t);
		       if ! r.1 { res = r.0; }
		       r.1 }
		&& ! FIXNUM_OVERFLOW_P (res as EMACS_INT)
	      { *top = make_fixnum (res as EMACS_INT); }
	    else
	      { *top = Ftimes (2, top); }
	    NEXT! ();
	  }
	CASE! (Bquo) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    let mut res: EMACS_INT = 0;
	    if FIXNUMP (v1) && FIXNUMP (v2) && XFIXNUM (v2) != 0
		&& { res = XFIXNUM (v1) / XFIXNUM (v2);
		     ! FIXNUM_OVERFLOW_P (res) }
	      { *top = make_fixnum (res); }
	    else
	      { *top = Fquo (2, top); }
	    NEXT! ();
	  }
	CASE! (Brem) =>
	  {
	    let v2: Lisp_Object = POP! ();
	    let v1: Lisp_Object = TOP! ();
	    if FIXNUMP (v1) && FIXNUMP (v2) && XFIXNUM (v2) != 0
	      { *top = make_fixnum (XFIXNUM (v1) % XFIXNUM (v2)); }
	    else
	      { *top = Frem (v1, v2); }
	    NEXT! ();
	  }
	CASE! (Bpoint) =>
	  {
	    PUSH! (make_fixed_natnum (PT! ()));
	    NEXT! ();
	  }
	CASE! (Bgoto_char) =>
	  {
	    *top = Fgoto_char (TOP! ());
	    NEXT! ();
	  }
	CASE! (Binsert) =>
	  {
	    *top = Finsert (1, top);
	    NEXT! ();
	  }
	CASE! (BinsertN) =>
	  {
	    let n: ptrdiff_t = FETCH! ();
	    DISCARD! (n - 1);
	    *top = Finsert (n, top);
	    NEXT! ();
	  }
	CASE! (Bpoint_max) =>
	  {
	    PUSH! (make_fixed_natnum (ZV! ()));
	    NEXT! ();
	  }
	CASE! (Bpoint_min) =>
	  {
	    PUSH! (make_fixed_natnum (BEGV! ()));
	    NEXT! ();
	  }
	CASE! (Bchar_after) =>
	  {
	    *top = Fchar_after (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bfollowing_char) =>
	  {
	    PUSH! (Ffollowing_char ());
	    NEXT! ();
	  }
	CASE! (Bpreceding_char) =>
	  {
	    PUSH! (Fprevious_char ());
	    NEXT! ();
	  }
	CASE! (Bcurrent_column) =>
	  {
	    PUSH! (make_fixed_natnum (current_column ()));
	    NEXT! ();
	  }
	CASE! (Bindent_to) =>
	  {
	    *top = Findent_to (TOP! (), Qnil ());
	    NEXT! ();
	  }
	CASE! (Beolp) =>
	  {
	    PUSH! (Feolp ());
	    NEXT! ();
	  }
	CASE! (Beobp) =>
	  {
	    PUSH! (Feobp ());
	    NEXT! ();
	  }
	CASE! (Bbolp) =>
	  {
	    PUSH! (Fbolp ());
	    NEXT! ();
	  }
	CASE! (Bbobp) =>
	  {
	    PUSH! (Fbobp ());
	    NEXT! ();
	  }
	CASE! (Bcurrent_buffer) =>
	  {
	    PUSH! (Fcurrent_buffer ());
	    NEXT! ();
	  }
	CASE! (Bset_buffer) =>
	  {
	    *top = Fset_buffer (TOP! ());
	    NEXT! ();
	  }
	CASE! (Binteractive_p) =>	/* Obsolete since 24.1.  */
	  {
	    PUSH! (call0 (Qinteractive_p ()));
	    NEXT! ();
	  }
	CASE! (Bforward_char) =>
	  {
	    *top = Fforward_char (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bforward_word) =>
	  {
	    *top = Fforward_word (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bskip_chars_forward) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fskip_chars_forward (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bskip_chars_backward) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fskip_chars_backward (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bforward_line) =>
	  {
	    *top = Fforward_line (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bchar_syntax) =>
	  {
	    *top = Fchar_syntax (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bbuffer_substring) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fbuffer_substring (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bdelete_region) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fdelete_region (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bnarrow_to_region) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fnarrow_to_region (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bwiden) =>
	  {
	    PUSH! (Fwiden ());
	    NEXT! ();
	  }
	CASE! (Bend_of_line) =>
	  {
	    *top = Fend_of_line (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bset_marker) =>
	  {
	    let v2: Lisp_Object = POP! (); let v1: Lisp_Object = POP! ();
	    *top = Fset_marker (TOP! (), v1, v2);
	    NEXT! ();
	  }
	CASE! (Bmatch_beginning) =>
	  {
	    *top = Fmatch_beginning (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bmatch_end) =>
	  {
	    *top = Fmatch_end (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bupcase) =>
	  {
	    *top = Fupcase (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bdowncase) =>
	  {
	    *top = Fdowncase (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bstringeqlsign) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fstring_equal (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bstringlss) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fstring_lessp (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bequal) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fequal (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bnthcdr) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fnthcdr (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Belt) =>
	  {
	    let v2: Lisp_Object = POP! (); let mut v1: Lisp_Object = TOP! ();
	    if CONSP (v1) && RANGED_FIXNUMP (0, v2, SMALL_LIST_LEN_MAX as EMACS_INT)
	      {
		/* Like the fast case for Bnth, but with args reversed.  */
		let mut n: EMACS_INT = XFIXNUM (v2);
		while 0 < n && CONSP (v1)
		  { v1 = XCDR (v1); n -= 1; }
		if CONSP (v1)
		  { *top = XCAR (v1); }
		else if NILP (v1)
		  { *top = Qnil (); }
		else
		  {
		    record_in_backtrace (Qelt (), top, 2);
		    wrong_type_argument (Qlistp (), v1);
		  }
	      }
	    else
	      { *top = Felt (v1, v2); }
	    NEXT! ();
	  }
	CASE! (Bmember) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fmember (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bassq) =>
	  {
	    let v1: Lisp_Object = POP! ();
	    *top = Fassq (TOP! (), v1);
	    NEXT! ();
	  }
	CASE! (Bnreverse) =>
	  {
	    *top = Fnreverse (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bsetcar) =>
	  {
	    let newval: Lisp_Object = POP! ();
	    let cell: Lisp_Object = TOP! ();
	    if ! CONSP (cell)
	      {
		record_in_backtrace (Qsetcar (), top, 2);
		wrong_type_argument (Qconsp (), cell);
	      }
	    XSETCAR (cell, newval);
	    *top = newval;
	    NEXT! ();
	  }
	CASE! (Bsetcdr) =>
	  {
	    let newval: Lisp_Object = POP! ();
	    let cell: Lisp_Object = TOP! ();
	    if ! CONSP (cell)
	      {
		record_in_backtrace (Qsetcdr (), top, 2);
		wrong_type_argument (Qconsp (), cell);
	      }
	    XSETCDR (cell, newval);
	    *top = newval;
	    NEXT! ();
	  }
	CASE! (Bcar_safe) =>
	  {
	    *top = CAR_SAFE (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bcdr_safe) =>
	  {
	    *top = CDR_SAFE (TOP! ());
	    NEXT! ();
	  }
	CASE! (Bnconc) =>
	  {
	    DISCARD! (1);
	    *top = Fnconc (2, top);
	    NEXT! ();
	  }
	CASE! (Bnumberp) =>
	  {
	    *top = if NUMBERP (TOP! ()) { Qt () } else { Qnil () };
	    NEXT! ();
	  }
	CASE! (Bintegerp) =>
	  {
	    *top = if INTEGERP (TOP! ()) { Qt () } else { Qnil () };
	    NEXT! ();
	  }
	CASE_ABORT! () =>
	  {
	  /* Actually this is Bstack_ref with offset 0, but we use Bdup
	     for that instead.  */
	  /* CASE (Bstack_ref): */
	  error (c"Invalid byte opcode: op=%d, ptr=%td".as_ptr (),
		 *pc.offset (-1) as c_int, pc.offset (-1).offset_from (bytestr_data));
	  }
	  /* Handy byte-codes for lexical binding.  */
	CASE! (Bstack_ref1)
	  | CASE! (Bstack_ref2)
	  | CASE! (Bstack_ref3)
	  | CASE! (Bstack_ref4)
	  | CASE! (Bstack_ref5) =>
	  {
	    let v1: Lisp_Object = *top.offset (Bstack_ref - op);
	    PUSH! (v1);
	    NEXT! ();
	  }
	CASE! (Bstack_ref6) =>
	  {
	    let v1: Lisp_Object = *top.offset (- FETCH! ());
	    PUSH! (v1);
	    NEXT! ();
	  }
	CASE! (Bstack_ref7) =>
	  {
	    let v1: Lisp_Object = *top.offset (- FETCH2! ());
	    PUSH! (v1);
	    NEXT! ();
	  }
	CASE! (Bstack_set) =>
	  {
	  /* stack-set-0 = discard; stack-set-1 = discard-1-preserve-tos.  */
	    let ptr: *mut Lisp_Object = top.offset (- FETCH! ());
	    *ptr = POP! ();
	    NEXT! ();
	  }
	CASE! (Bstack_set2) =>
	  {
	    let ptr: *mut Lisp_Object = top.offset (- FETCH2! ());
	    *ptr = POP! ();
	    NEXT! ();
	  }
	CASE! (BdiscardN) =>
	  {
	    let mut n: ptrdiff_t = FETCH! ();
	    if n & 0x80 != 0
	      {
		n &= 0x7F;
		*top.offset (- n) = TOP! ();
	      }
	    DISCARD! (n);
	    NEXT! ();
	  }
        CASE! (Bswitch) =>
          {
            /* TODO: Perhaps introduce another byte-code for switch when the
	       number of cases is less, which uses a simple vector for linear
	       search as the jump table.  */
	    /* TODO: Instead of pushing the table in a separate
	       Bconstant op, use an immediate argument (maybe separate
	       switch opcodes for 1-byte and 2-byte constant indices).
	       This would also get rid of some hacks that assume each
	       Bswitch to be preceded by a Bconstant.  */
            let jmp_table: Lisp_Object = POP! ();
	    if BYTE_CODE_SAFE! () && ! HASH_TABLE_P (jmp_table)
              { emacs_abort (); }
            let v1: Lisp_Object = POP! ();
            let h: *mut Lisp_Hash_Table = XHASH_TABLE (jmp_table);
	    /* Do a linear search if there are few cases and the test is `eq'.
	       (The table is assumed to be sized exactly; all entries are
	       consecutive at the beginning.)
	       FIXME: 5 is arbitrarily chosen.  */
            if (*h).count <= 5 && NILP ((*(*h).test).cmpfn) && ! symbols_with_pos_enabled
              {
		eassume! ((*h).count >= 2);
		let mut i: ptrdiff_t = ((*h).count - 1) as ptrdiff_t;
		while i >= 0
		  {
		    if BASE_EQ (v1, HASH_KEY (h, i as hash_idx_t))
		      {
			arg = XFIXNUM (HASH_VALUE (h, i as hash_idx_t));
			/* C: `goto op_branch` (leaves the search) */
			goto = OP_BRANCH;
			break;
		      }
		    i -= 1;
		  }
              }
            else
	      {
		let i: ptrdiff_t = hash_find (h, v1);
		if i >= 0
		  {
		    arg = XFIXNUM (HASH_VALUE (h, i as hash_idx_t));
		    /* C: `goto op_branch` */
		    goto = OP_BRANCH;
		  }
	      }
	    NEXT! ();
          }
	CASE_DEFAULT! () =>
	  {
	  if BYTE_CODE_SAFE! ()
	      && ! (Bconstant <= op && op < Bconstant + const_length)
	    { emacs_abort (); }
	  PUSH! (*vectorp.offset (op - Bconstant));
	  NEXT! ();
	  }
	});
    }

      /* C's `exit:` (the `goto exit` in `Breturn`).  */
      (*bc).fp = (*( (*bc).fp as *mut bc_frame )).saved_fp as *mut c_void;

      let result: Lisp_Object = TOP! ();
      return result;
    }
}

/* `args_template' has the same meaning as in exec_byte_code() above.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_byte_code_arity (args_template: Lisp_Object) -> Lisp_Object
{
  eassert! (FIXNATP (args_template));
  let at: EMACS_INT = XFIXNUM (args_template);
  let rest: bool = (at & 128) != 0;
  let mandatory: c_int = (at & 127) as c_int;
  let nonrest: EMACS_INT = at >> 8;

  return Fcons (make_fixnum (mandatory as EMACS_INT),
		if rest { Qmany () } else { make_fixnum (nonrest) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_bytecode ()
{
  DEFSYM! (Qinteractive_p, c"interactive-p");

  defsubr (&raw mut Sbyte_code);
  defsubr (&raw mut Sinternal_stack_stats);

  /* C 1830-1851's `#ifdef BYTE_CODE_METER` arm (the `byte-code-meter` histogram and
     `byte-metering-on`) is dead in the frozen configuration and is recorded in
     docs/ports/bytecode.md rather than transcribed (P-08).  */
}
