// femacs: mirror of src/ccl.h @ fd7afced403.
//
// Mirror (PORTING.md 0.1-2): types, constants, `macro_rules!` definitions and the
// function declarations in `ccl_decls` + glob import (PORTING.md 0.1-10, the composite.rs
// shape).  Nothing here emits a C-visible symbol.  `ccl.h` has no platform branch of its
// own (only its include guard), and `tools/check-cond-eval.py` reports zero disagreements
// for it, so the frozen configuration's branch selection is the live text.  The whole
// header lands with `ccl.c` (batch A): the five `CCL_STAT_*` constants, `struct
// ccl_program`, `struct ccl_spec`, `CODING_SPEC_CCL_PROGRAM` and `CHECK_CCL_PROGRAM`.
//
// `struct ccl_spec` is what closes PORTING.md P-70: `coding.rs` used to carry a zero-sized
// stand-in for it (coding.h 447 names the tag only as a pointer), and that stand-in is
// deleted in the same commit as this mirror, with `coding_system.ccl` degrading to the
// ABI-equal `*mut c_void` (P-72③).  Two same-named definitions in one unit would be an
// `E0428`.

// ccl.h 26-27: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_CCL_H: bool = true;

/* Macros for exit status of CCL program.  */
pub const CCL_STAT_SUCCESS: c_int = 0; /* Terminated successfully.  */
pub const CCL_STAT_SUSPEND_BY_SRC: c_int = 1; /* Terminated by empty input.  */
pub const CCL_STAT_SUSPEND_BY_DST: c_int = 2; /* Terminated by output buffer full.  */
pub const CCL_STAT_INVALID_CMD: c_int = 3; /* Terminated because of invalid
				     command.  */
pub const CCL_STAT_QUIT: c_int = 4; /* Terminated because of quit.  */

/* Structure to hold information about running CCL code.  Read
   comments in the file ccl.c for the detail of each field.  */
/// C: `struct ccl_program` ccl.h 37-65.  The two `bool_bf` members are one 1-byte storage
/// unit in C (`bool_bf` is `bool` on this target, `conf_post.h` 56-61), so the port packs
/// them into one byte named `bits` (PORTING.md 1.3); clang's record layout on the frozen
/// target puts `idx` at 0, `size` at 8, `prog` at 16, `ic` at 24, `eof_ic` at 28, `reg` at
/// 32, `status` at 64, `buf_magnification` at 68, `stack_idx` at 72, `consumed` at 76,
/// `produced` at 80, the two bits at 84:0-84:1 and `sizeof (struct ccl_program)` at
/// 88/align 8 (measured with clang against the frozen target; the ledger carries the
/// command and the numbers, and the `const _` assertions below pin them).
#[repr(C)]
pub struct ccl_program
{
  /* Index number of the CCL program.
     -1 means that the program was given
     by a vector, not by a program
     name.  */
  pub idx: ptrdiff_t,

  /* Size of the compiled code.  */
  pub size: c_int,

  /* Pointer into the compiled code.  */
  pub prog: *mut Lisp_Object,

  /* Instruction Counter (index for PROG).  */
  pub ic: c_int,

  /* Instruction Counter for end-of-file
     processing code.  */
  pub eof_ic: c_int,

  /* CCL registers, reg[7] is used for
     condition flag of relational
     operations.  */
  pub reg: [c_int; 8],

  /* Exit status of the CCL program.  */
  pub status: c_int,

  /* Output buffer magnification.  How
     many times bigger the output buffer
     should be than the input buffer.  */
  pub buf_magnification: c_int,

  /* How deep the call of CCL_Call is nested.  */
  pub stack_idx: c_int,

  pub consumed: c_int,
  pub produced: c_int,

  /* Set to true while processing the last
     block. */
  /// bool_bf last_block : 1;
  /* If true, don't append "CCL:
     Quitted" to the generated text when
     CCL program is quitted. */
  /// bool_bf quit_silently : 1;
  /// clang's record layout puts the two bits at 84:0-84:1 of this byte.
  pub bits: u8,
}

/// The two `bool_bf` members of `struct ccl_program`, in ccl.h's declaration order.  C
/// names them through the bitfields; Rust reads them by mask (PORTING.md 1.3), and
/// `docs/ports/token-aliases.tsv` carries the `last_block`/`quit_silently` families so the
/// reads and writes stay occurrence-checked.
pub const CCL_PROGRAM_LAST_BLOCK: u8 = 1 << 0;
pub const CCL_PROGRAM_QUIT_SILENTLY: u8 = 1 << 1;

const _: () = core::assert! (std::mem::size_of::<ccl_program> () == 88);
const _: () = core::assert! (std::mem::align_of::<ccl_program> () == 8);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, idx) == 0);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, size) == 8);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, prog) == 16);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, ic) == 24);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, eof_ic) == 28);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, reg) == 32);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, status) == 64);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, buf_magnification) == 68);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, stack_idx) == 72);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, consumed) == 76);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, produced) == 80);
const _: () = core::assert! (std::mem::offset_of! (ccl_program, bits) == 84);

/* This data type is used for the spec field of the structure
   coding_system.  */

/// C: `struct ccl_spec` ccl.h 69-71; clang gives `sizeof` 88/align 8 with `ccl` at 0.
#[repr(C)]
pub struct ccl_spec
{
  pub ccl: ccl_program,
}

const _: () = core::assert! (std::mem::size_of::<ccl_spec> () == 88);
const _: () = core::assert! (std::mem::offset_of! (ccl_spec, ccl) == 0);

// ccl.h 73: `#define CODING_SPEC_CCL_PROGRAM(coding) ((coding)->spec.ccl.ccl)`.  The union
// member `spec.ccl` and the struct member `.ccl` keep C's spelling; no unit in the port
// reaches this macro yet (coding.c is its first reader).
macro_rules! CODING_SPEC_CCL_PROGRAM {
  ($coding:expr) => { (*$coding).spec.ccl.ccl };
}

// Both prototypes of ccl.h are defined in ccl.c, so they live here + glob import (an
// explicit item shadows a glob import where a same-named declaration at this scope would
// be `E0428`, PORTING.md 0.1-10/P-21, the tempname.rs shape).
pub mod ccl_decls {
  use super::*;

    unsafe extern "C-unwind" {
    pub fn setup_ccl_program (ccl: *mut ccl_program, ccl_prog: Lisp_Object) -> bool;
    pub fn ccl_driver (ccl: *mut ccl_program, source: *mut c_int, destination: *mut c_int,
		       src_size: c_int, dst_size: c_int, charset_list: Lisp_Object);
    }
}
pub use self::ccl_decls::*;

// ccl.h 82-86: `#define CHECK_CCL_PROGRAM(x) do { if (NILP (Fccl_program_p (x)))
// wrong_type_argument (Qcclp, x); } while (false);`.  C's `do { … } while (false)` has no
// Rust counterpart, so the guard is a block; the loop's `false` is the one token without a
// match and `docs/ports/macro-body-notes.tsv` carries it.
macro_rules! CHECK_CCL_PROGRAM {
  ($x:expr) => {
    {
      if NILP (Fccl_program_p ($x))
	{ wrong_type_argument (Qcclp (), $x); }
    }
  };
}
