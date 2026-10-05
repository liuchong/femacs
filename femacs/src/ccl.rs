// femacs: port of src/ccl.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct, struct field, file-scope object, local), same control flow, same error
// model.  The five `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's three non-`static`
// definitions (`ccl_driver`, `setup_ccl_program`, `syms_of_ccl`) and its two non-`static`
// objects (`ccl_backtrace_table`, `ccl_backtrace_idx`, both inside `#ifdef CCL_DEBUG`) are
// external symbols, so they carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static`
// helpers (`GET_TRANSLATION_TABLE`, `resolve_symbol_ccl_program`,
// `ccl_get_compiled_code`) stay plain `unsafe fn`, and `ccl_debug_hook` (non-`static` in C,
// inside the `CCL_DEBUG` arm) is an exported `unsafe extern "C" fn` under that `#[cfg]`.
//
// **`ccl.h`'s mirror lands with this unit** (`femacs/src/include/ccl.rs`, the composite.rs
// precedent): `struct ccl_program`, `struct ccl_spec`, the five `CCL_STAT_*` constants,
// `CODING_SPEC_CCL_PROGRAM`, `CHECK_CCL_PROGRAM` and the two `extern` prototypes, the
// latter in `ccl_decls` + glob import because this unit defines them (PORTING.md 0.1-10).
// Landing it deletes the `struct ccl_spec` stand-in `femacs/src/include/coding.rs` carried
// for `coding.h` 447 (PORTING.md P-70 says the stand-in goes in the same commit as
// `ccl.rs`, or the two definitions are one `E0428` in a unit that includes both mirrors);
// `coding_system.ccl` becomes the ABI-equal `*mut c_void` with a `/// C:` note, which is
// P-72③'s rule for a header that only names the tag as a pointer.
//
// **`charset.h` has no mirror yet** (`docs/port-map.md` still lists
// `femacs/src/include/charset.rs` as unstarted), so the names this unit's live text takes
// from it are declared at their use site below with C's own bodies (PORTING.md 0.1-2,
// P-72, the chartab.rs shape); they move to the mirror when charset.c lands.  The same
// block carries `larger_vector` and `make_specified_string` (lisp.h, not mirrored that
// far) and `<intprops.h>`'s `INT_DIVIDE_OVERFLOW` (P-26, the canonicalize-lgpl.rs shape).
//
// **`ccl.c` is not in `docs/baseline/config/cond-eval-verified.txt`**, so the frozen
// macro table is not consulted for it and the `#ifdef CCL_DEBUG` arms are carried as
// `#[cfg (feature = "ccl_debug")]` text rather than recorded as dead (PORTING.md P-51,
// status 3.49; the `total` line-drift basis).  Those arms are C 607-609's `#pragma`,
// C 720-738's two `CCL_INVALID_CMD` definitions, C 844-855 (`CCL_DEBUG_BACKTRACE_LEN`,
// `ccl_backtrace_table`, `ccl_backtrace_idx`, `ccl_debug_hook`), 898-910 and 1805-1829
// (the two backtrace regions inside `ccl_driver`); `CCL_DEBUG` is a source-level macro,
// not a `config.h` key, so they are not in `xtask dead-arms` either.  The one `#if 0`
// arm (C 1848-1857, the "copy the remaining source data" text that names the
// non-existent `dst_bytes`) is dead for the tool as well and is recorded in
// `docs/ports/ccl.md` instead of transcribed.
//
// **`ccl_driver`'s five C labels** (`ccl_repeat`, `ccl_set_expr`, `ccl_expr_self`,
// `ccl_error_handler`, `ccl_finish`) have no Rust counterpart, so the port carries them
// as the `ccl_goto` values below: the whole `for (;;)` body is one `'ccl_repeat` loop
// that leaves with the label a C `goto` would reach, and the two labels *inside* the
// `switch` are an entry stage after the `match`, the shared statements appearing once
// (PORTING.md 2.2, the bytecode.rs/tparam.rs shape).  C's nine statement macros
// (`CCL_CALL_FOR_MAP_INSTRUCTION`, `CCL_SUCCESS`, `CCL_SUSPEND`, `CCL_INVALID_CMD`,
// `CCL_WRITE_CHAR`, `CCL_WRITE_STRING`, `CCL_READ_CHAR`, `CCL_DECODE_CHAR`,
// `CCL_ENCODE_CHAR`) capture `ccl_driver`'s locals and jump to those labels, and the
// five field aliases (`rrr`, `RRR`, `Rrr`, `ADDR`, `EXCMD`) are C's preprocessor
// substitution.  A `macro_rules!` resolves local variables *and labels* at its
// **definition** site, so these macros are defined inside the loop body, after the
// locals they capture and inside the label they jump to (the bytecode.rs P-57
// placement); every call site therefore keeps C's own shape, and `rrr = 7` — a macro
// expansion is not a place — is written `field2 = 7`, C's preprocessed text (the
// bytecode.rs `TOP = x` precedent).  `ASCENDING_ORDER`, `GET_CCL_RANGE`, `GET_CCL_CODE`,
// `IN_INT_RANGE` and the two mapping-stack macros reference no local of their own, so
// they stay at C's file-scope position; `GET_CCL_RANGE`'s body invokes
// `CCL_INVALID_CMD`, which the port resolves at the expansion site (macro names are
// looked up there, PORTING.md P-57).
//
// C's uninitialised locals are zero-initialised (PORTING.md 1.5); this file declares no
// function prototype of its own (only its file-scope objects), so PORTING.md 0.1-12 has
// nothing to drop here.  Every deviation is recorded in `docs/ports/ccl.md`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut, unused_labels, unused_braces)]

include!("include/config.rs");                 // C 26: `#include <config.h>`
include!("../lib/include/stdio.rs");           // C 28: `#include <stdio.h>`
include!("../lib/include/limits.in.rs");       // C 29: `#include <limits.h>`
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (memcpy, memset)
include!("include/lisp.rs");                   // C 31: `#include "lisp.h"`
include!("../lib/include/mini-gmp.rs");        // lisp.h's bignum.h chain (bignum.rs's members)
include!("include/bignum.rs");
include!("include/character.rs");              // C 32: `#include "character.h"`
include!("include/itree.rs");                  // buffer.h 28: `#include "itree.h"` (buffer.rs's members)
include!("include/systime.rs");                // termhooks.h 28: `#include "systime.h"`
include!("include/font.rs");                   // composite.h 28: `#include "font.h"`
include!("include/composite.rs");              // coding.h 284: `#include "composite.h"`
include!("include/buffer.rs");                 // composite.h 29: `#include "buffer.h"`
include!("include/intervals.rs");              // buffer.h 29: `#include "intervals.h"`
include!("include/frame.rs");                  // buffer.h 30: `#include "frame.h"`
include!("include/dispextern.rs");             // composite.h 29: `#include "dispextern.h"`
include!("include/window.rs");                 // buffer.h 31: `#include "window.h"`
include!("include/coding.rs");                 // C 35: `#include "coding.h"`
include!("include/ccl.rs");                    // C 34: `#include "ccl.h"`
include!("include/termhooks.rs");              // keyboard.h 24: `#include "termhooks.h"`
include!("include/keyboard.rs");               // C 36: `#include "keyboard.h"`
// C 33's `#include "charset.h"` has no mirror yet; its names are declared below.

/* The names C's headers give this unit that no mirror carries (the chartab.rs shape,
   PORTING.md 0.1-2/P-72).  `charset.h` is not mirrored yet (`docs/port-map.md` still lists
   `femacs/src/include/charset.rs` as unstarted), so every name this unit's live text takes
   from it is declared here with C's own body; the block moves to the mirror when charset.c
   lands (recorded in `docs/ports/ccl.md`).  `larger_vector` (lisp.h 4245) and
   `make_specified_string` (lisp.h 4566) are lisp.h declarations the mirror does not carry
   yet, and `<intprops.h>`'s `INT_DIVIDE_OVERFLOW` has no mirror at all (PORTING.md P-26,
   the canonicalize-lgpl.rs shape).  */

/* Methods for converting code points and characters of charsets.  */
// charset.h 130-146: `enum charset_method`.
pub type charset_method = c_int;
pub const CHARSET_METHOD_OFFSET: charset_method = 0;
pub const CHARSET_METHOD_MAP: charset_method = 1;
pub const CHARSET_METHOD_SUBSET: charset_method = 2;
pub const CHARSET_METHOD_SUPERSET: charset_method = 3;

// charset.h 62-75: `enum charset_attr_index`.  Only the two members DECODE_CHAR's
// `CHARSET_DECODER` and `charset_attributes_getter` reach are written here, in C's
// declaration order.
pub type charset_attr_index = c_int;
pub const charset_id: charset_attr_index = 0;
pub const charset_decoder: charset_attr_index = 4;

/// C: `struct charset` charset.h 148-243.  The six `bool_bf` members are one 1-byte
/// storage unit in C (`bool_bf` is `bool` on this target, `conf_post.h` 56-61), so the port
/// packs them into one byte named `bits` (PORTING.md 1.3); clang's record layout on the
/// frozen target puts `id` at 0, `dimension` at 4, `code_space` at 8, `code_space_mask` at
/// 72, the six bits at 80:0-80:5, `iso_final` at 84, `iso_revision` at 88, `emacs_mule_id`
/// at 92, `method` at 96, `min_code` at 100, `max_code` at 104, `char_index_offset` at 108,
/// `min_char` at 112, `max_char` at 116, `invalid_code` at 120, `fast_map` at 124,
/// `code_offset` at 316 and `sizeof (struct charset)` at 320/align 8 (measured; the ledger
/// carries the command and the numbers).
#[repr(C)]
pub struct charset
{
  /* Index to charset_table.  */
  pub id: c_int,

  /* Dimension of the charset: 1, 2, 3, or 4.  */
  pub dimension: c_int,

  /* Byte code range of each dimension.  <code_space>[4N] is a minimum
     byte code of the (N+1)th dimension, <code_space>[4N+1] is a
     maximum byte code of the (N+1)th dimension, <code_space>[4N+2] is
     (<code_space>[4N+1] - <code_space>[4N] + 1), <code_space>[4N+3]
     is the number of characters contained in the first through (N+1)th
     dimensions, except that there is no <code_space>[15].
     We get `char-index' of a `code-point' from this
     information.  */
  pub code_space: [c_int; 15],

  /* If B is a byte of Nth dimension of a code-point, the (N-1)th bit
     of code_space_mask[B] is set.  This array is used to quickly
     check if a code-point is in a valid range.  */
  pub code_space_mask: *mut u8,

  /* True if there's no gap in code-points.  */
  /// bool_bf code_linear_p : 1;
  /* True if the charset is treated as 96 chars in ISO-2022
     as opposed to 94 chars.  */
  /// bool_bf iso_chars_96 : 1;
  /* True if the charset is compatible with ASCII.  */
  /// bool_bf ascii_compatible_p : 1;
  /* True if the charset is supplementary.  */
  /// bool_bf supplementary_p : 1;
  /* True if all the code points are representable by Lisp_Int.  */
  /// bool_bf compact_codes_p : 1;
  /* True if the charset is unified with Unicode.  */
  /// bool_bf unified_p : 1;
  /// clang's record layout puts the six bits at 80:0-80:5 of this byte.
  pub bits: u8,

  /* ISO final byte of the charset: 48..127.  It may be -1 if the
     charset doesn't conform to ISO-2022.  */
  pub iso_final: c_int,

  /* ISO revision number of the charset.  */
  pub iso_revision: c_int,

  /* If the charset is identical to what supported by Emacs 21 and the
     priors, the identification number of the charset used in those
     version.  Otherwise, -1.  */
  pub emacs_mule_id: c_int,

  /* The method for encoding/decoding characters of the charset.  */
  pub method: charset_method,

  /* Minimum and Maximum code points of the charset.  */
  pub min_code: c_uint,
  pub max_code: c_uint,

  /* Offset value used by macros CODE_POINT_TO_INDEX and
      INDEX_TO_CODE_POINT.  */
  pub char_index_offset: c_uint,

  /* Minimum and Maximum character codes of the charset.  If the
     charset is compatible with ASCII, min_char is a minimum non-ASCII
     character of the charset.  If the method of charset is
     CHARSET_METHOD_OFFSET, even if the charset is unified, min_char
     and max_char doesn't change.  */
  pub min_char: c_int,
  pub max_char: c_int,

  /* The code returned by ENCODE_CHAR if a character is not encodable
     by the charset.  */
  pub invalid_code: c_uint,

  /* If the method of the charset is CHARSET_METHOD_MAP, this is a
     table of bits used to quickly and roughly guess if a character
     belongs to the charset.

     The first 64 elements are 512 bits for characters less than
     0x10000.  Each bit corresponds to 128-character block.  The last
     126 elements are 1008 bits for the greater characters
     (0x10000..0x3FFFFF).  Each bit corresponds to 4096-character
     block.

     If a bit is 1, at least one character in the corresponding block is
     in this charset.  */
  pub fast_map: [u8; 190],

  /* Offset value to calculate a character code from code-point, and
     visa versa.  */
  pub code_offset: c_int,
}

/// The six `bool_bf` members of `struct charset`, in charset.h's declaration order.  C
/// names them through the bitfields; Rust reads them by mask (PORTING.md 1.3).  The names
/// coincide with charset.h's own `CHARSET_*_P(charset)` accessor macros, which live in
/// Rust's macro namespace and so do not collide with these constants.
pub const CHARSET_CODE_LINEAR_P: u8 = 1 << 0;
pub const CHARSET_ISO_CHARS_96: u8 = 1 << 1;
pub const CHARSET_ASCII_COMPATIBLE_P: u8 = 1 << 2;
pub const CHARSET_SUPPLEMENTARY_P: u8 = 1 << 3;
pub const CHARSET_COMPACT_CODES_P: u8 = 1 << 4;
pub const CHARSET_UNIFIED_P: u8 = 1 << 5;

/// C: `struct charset_table` charset.h 246-262.
#[repr(C)]
pub struct charset_table
{
  pub start: *mut charset,
  pub size: c_int,
  pub used: c_int,
  pub attributes_table: Lisp_Object,
}

unsafe extern "C" {
  /// C: charset.h 265: `extern struct charset_table charset_table;` (charset.c defines it).
  pub static mut charset_table: charset_table;
  /// C: charset.h 546: `extern int charset_unicode;` (charset.c defines it).
  pub static mut charset_unicode: c_int;
  /// C: charset.h 554: `extern struct charset *char_charset (int, Lisp_Object, unsigned *);`
  pub fn char_charset (c: c_int, charset_list: Lisp_Object, code: *mut c_uint)
                       -> *mut charset;
  /// C: charset.h 557: `extern int decode_char (struct charset *, unsigned);`
  pub fn decode_char (charset: *mut charset, code: c_uint) -> c_int;
  /// C: lisp.h 4245: `extern Lisp_Object larger_vector (Lisp_Object, ptrdiff_t, ptrdiff_t);`
  /// (the lisp.rs mirror does not carry it yet; reported as a mirror gap in
  /// docs/ports/ccl.md).
  pub fn larger_vector (array: Lisp_Object, incr_min: ptrdiff_t, incr_max: ptrdiff_t)
                        -> Lisp_Object;
  /// C: lisp.h 4566-4567: `extern Lisp_Object make_specified_string (const char *,
  /// ptrdiff_t, ptrdiff_t, bool);` (likewise a mirror gap).
  pub fn make_specified_string (contents: *const c_char, nchars: ptrdiff_t,
				nbytes: ptrdiff_t, multibyte: bool) -> Lisp_Object;
}

// charset.h 282-296 and 313-340: the accessor macros `CHARSET_ATTR_ID`,
// `CHARSET_ATTR_DECODER`, `CHARSET_ATTRIBUTES` and `CHARSET_DECODER`, each with C's body.
macro_rules! CHARSET_ATTR_ID { ($attrs:expr) => { AREF ($attrs, charset_id as isize) }; }
macro_rules! CHARSET_ATTR_DECODER { ($attrs:expr) => { AREF ($attrs, charset_decoder as isize) }; }
macro_rules! CHARSET_ATTRIBUTES { ($charset:expr) => { charset_attributes_getter ($charset) }; }
macro_rules! CHARSET_DECODER {
  ($charset:expr) => { CHARSET_ATTR_DECODER! (CHARSET_ATTRIBUTES! ($charset)) }; }

/// charset.h 306-313: `INLINE Lisp_Object charset_attributes_getter (struct charset *)`.
/// It is a header inline, so C gives every translation unit its own copy; the port defines
/// it here until the mirror exists.
unsafe fn charset_attributes_getter (charset: *mut charset) -> Lisp_Object
{
  eassert! (ASIZE (charset_table.attributes_table) == charset_table.size as isize);
  let attrs: Lisp_Object = AREF (charset_table.attributes_table, (*charset).id as isize);
  eassert! (XFIXNUM (CHARSET_ATTR_ID! (attrs)) == (*charset).id as isize);
  return attrs;
}

// charset.h 267: `#define CHARSET_FROM_ID(id) (charset_table.start + (id))`.
macro_rules! CHARSET_FROM_ID { ($id:expr) => { charset_table.start.offset (($id) as isize) }; }
// charset.h 316: `#define CHARSET_ID(charset) ((charset)->id)`.
macro_rules! CHARSET_ID { ($charset:expr) => { (*$charset).id }; }

/* Return a character corresponding to the code-point CODE of CHARSET.
   Try some optimization before calling decode_char.  */

// charset.h 427-446: `DECODE_CHAR`, C's body with the bitfields read through the mirror's
// masks and the `min_code`/`max_code` comparisons kept in C's unsigned type.  The two
// operands are bound once, because C's `->` on an `expr` fragment has no Rust spelling
// (each C use reads the same place, so one binding keeps C's evaluation count).
macro_rules! DECODE_CHAR {
  ($charset:expr, $code:expr) => {
    {
      let charset: *mut charset = $charset;
      let code: c_int = $code;
      (if ASCII_CHAR_P (code as isize)
	  && (*charset).bits & CHARSET_ASCII_COMPATIBLE_P != 0
	 { code }
       else if (code as c_uint) < (*charset).min_code
	       || (code as c_uint) > (*charset).max_code
	 { -1 }
       else if (*charset).bits & CHARSET_UNIFIED_P != 0
	 { decode_char (charset, code as c_uint) }
       else if (*charset).method == CHARSET_METHOD_OFFSET
	 { if (*charset).bits & CHARSET_CODE_LINEAR_P != 0
	     { ((code as c_uint - (*charset).min_code) as c_int) + (*charset).code_offset }
	   else { decode_char (charset, code as c_uint) } }
       else if (*charset).method == CHARSET_METHOD_MAP
	 { if (*charset).bits & CHARSET_CODE_LINEAR_P != 0
	      && VECTORP (CHARSET_DECODER! (charset))
	     { XFIXNUM (AREF (CHARSET_DECODER! (charset),
			      (code as c_uint - (*charset).min_code) as isize)) as c_int }
	   else { decode_char (charset, code as c_uint) } }
       else { decode_char (charset, code as c_uint) })
    }
  };
}

/* <intprops.h>'s `INT_DIVIDE_OVERFLOW (a, b)`: true when a / b overflows, i.e. for
   INT_MIN / -1 (PORTING.md 2.6; the file has no mirror, P-26).  */
macro_rules! INT_DIVIDE_OVERFLOW {
  ($a:expr, $b:expr) => { ($a) == c_int::MIN && ($b) == -1 };
}


/* Table of registered CCL programs.  Each element is a vector of
   NAME, CCL_PROG, RESOLVEDP, and UPDATEDP, where NAME (symbol) is the
   name of the program, CCL_PROG (vector) is the compiled code of the
   program, RESOLVEDP (t or nil) is the flag to tell if symbols in
   CCL_PROG is already resolved to index numbers or not, UPDATEDP (t
   or nil) is the flat to tell if the CCL program is updated after it
   was once used.  */
static mut Vccl_program_table: Lisp_Object = lisp_h_Qnil ();

/* Return a hash table of id number ID.  */
macro_rules! GET_HASH_TABLE {
  ($id:expr) => { XHASH_TABLE (XCDR (AREF (Vtranslation_hash_table_vector, ($id) as isize))) };
}

/* CCL (Code Conversion Language) is a simple language which has
   operations on one input buffer, one output buffer, and 7 registers.
   The syntax of CCL is described in `ccl.el'.  Emacs Lisp function
   `ccl-compile' compiles a CCL program and produces a CCL code which
   is a vector of integers.  The structure of this vector is as
   follows: The 1st element: buffer-magnification, a factor for the
   size of output buffer compared with the size of input buffer.  The
   2nd element: address of CCL code to be executed when encountered
   with end of input stream.  The 3rd and the remaining elements: CCL
   codes.  */

/* Header of CCL compiled code */
pub const CCL_HEADER_BUF_MAG: c_int = 0;
pub const CCL_HEADER_EOF: c_int = 1;
pub const CCL_HEADER_MAIN: c_int = 2;

/* CCL code is a sequence of 28-bit integers.  Each contains a CCL
   command and/or arguments in the following format:

	|----------------- integer (28-bit) ------------------|
	|------- 17-bit ------|- 3-bit --|- 3-bit --|- 5-bit -|
	|--constant argument--|-register-|-register-|-command-|
	   ccccccccccccccccc      RRR        rrr       XXXXX
  or
	|------- relative address -------|-register-|-command-|
	       cccccccccccccccccccc          rrr       XXXXX
  or
	|------------- constant or other args ----------------|
                     cccccccccccccccccccccccccccc

   where `cc...c' is a 17-bit, 20-bit, or 28-bit integer indicating a
   constant value or a relative/absolute jump address, `RRR'
   and `rrr' are CCL register number, `XXXXX' is one of the following
   CCL commands.  */

pub const CCL_CODE_MAX: c_int = ((1 << (28 - 1)) - 1);
pub const CCL_CODE_MIN: c_int = (-1 - CCL_CODE_MAX);

/* CCL commands

   Each comment fields shows one or more lines for command syntax and
   the following lines for semantics of the command.  In semantics, IC
   stands for Instruction Counter.  */

pub const CCL_SetRegister: c_int = 0x00; /* Set register a register value:
					1:00000000000000000RRRrrrXXXXX
					------------------------------
					reg[rrr] = reg[RRR];
					*/

pub const CCL_SetShortConst: c_int = 0x01; /* Set register a short constant value:
					1:CCCCCCCCCCCCCCCCCCCCrrrXXXXX
					------------------------------
					reg[rrr] = CCCCCCCCCCCCCCCCCCC;
					*/

pub const CCL_SetConst: c_int = 0x02; /* Set register a constant value:
					1:00000000000000000000rrrXXXXX
					2:CONSTANT
					------------------------------
					reg[rrr] = CONSTANT;
					IC++;
					*/

pub const CCL_SetArray: c_int = 0x03; /* Set register an element of array:
					1:CCCCCCCCCCCCCCCCCRRRrrrXXXXX
					2:ELEMENT[0]
					3:ELEMENT[1]
					...
					------------------------------
					if (0 <= reg[RRR] < CC..C)
					  reg[rrr] = ELEMENT[reg[RRR]];
					IC += CC..C;
					*/

pub const CCL_Jump: c_int = 0x04; /* Jump:
					1:A--D--D--R--E--S--S-000XXXXX
					------------------------------
					IC += ADDRESS;
					*/

/* Note: If CC..C is greater than 0, the second code is omitted.  */

pub const CCL_JumpCond: c_int = 0x05; /* Jump conditional:
					1:A--D--D--R--E--S--S-rrrXXXXX
					------------------------------
					if (!reg[rrr])
					  IC += ADDRESS;
					*/


pub const CCL_WriteRegisterJump: c_int = 0x06; /* Write register and jump:
					1:A--D--D--R--E--S--S-rrrXXXXX
					------------------------------
					write (reg[rrr]);
					IC += ADDRESS;
					*/

pub const CCL_WriteRegisterReadJump: c_int = 0x07; /* Write register, read, and jump:
					1:A--D--D--R--E--S--S-rrrXXXXX
					2:A--D--D--R--E--S--S-rrrYYYYY
					-----------------------------
					write (reg[rrr]);
					IC++;
					read (reg[rrr]);
					IC += ADDRESS;
					*/
/* Note: If read is suspended, the resumed execution starts from the
   second code (YYYYY == CCL_ReadJump).  */

pub const CCL_WriteConstJump: c_int = 0x08; /* Write constant and jump:
					1:A--D--D--R--E--S--S-000XXXXX
					2:CONST
					------------------------------
					write (CONST);
					IC += ADDRESS;
					*/

pub const CCL_WriteConstReadJump: c_int = 0x09; /* Write constant, read, and jump:
					1:A--D--D--R--E--S--S-rrrXXXXX
					2:CONST
					3:A--D--D--R--E--S--S-rrrYYYYY
					-----------------------------
					write (CONST);
					IC += 2;
					read (reg[rrr]);
					IC += ADDRESS;
					*/
/* Note: If read is suspended, the resumed execution starts from the
   second code (YYYYY == CCL_ReadJump).  */

pub const CCL_WriteStringJump: c_int = 0x0A; /* Write string and jump:
					1:A--D--D--R--E--S--S-000XXXXX
					2:LENGTH
					3:000MSTRIN[0]STRIN[1]STRIN[2]
					...
					------------------------------
					if (M)
					  write_multibyte_string (STRING, LENGTH);
					else
					  write_string (STRING, LENGTH);
					IC += ADDRESS;
					*/

pub const CCL_WriteArrayReadJump: c_int = 0x0B; /* Write an array element, read, and jump:
					1:A--D--D--R--E--S--S-rrrXXXXX
					2:LENGTH
					3:ELEMENT[0]
					4:ELEMENT[1]
					...
					N:A--D--D--R--E--S--S-rrrYYYYY
					------------------------------
					if (0 <= reg[rrr] < LENGTH)
					  write (ELEMENT[reg[rrr]]);
					IC += LENGTH + 2; (... pointing at N+1)
					read (reg[rrr]);
					IC += ADDRESS;
					*/
/* Note: If read is suspended, the resumed execution starts from the
   Nth code (YYYYY == CCL_ReadJump).  */

pub const CCL_ReadJump: c_int = 0x0C; /* Read and jump:
					1:A--D--D--R--E--S--S-rrrYYYYY
					-----------------------------
					read (reg[rrr]);
					IC += ADDRESS;
					*/

pub const CCL_Branch: c_int = 0x0D; /* Jump by branch table:
					1:CCCCCCCCCCCCCCCCCCCCrrrXXXXX
					2:A--D--D--R--E-S-S[0]000XXXXX
					3:A--D--D--R--E-S-S[1]000XXXXX
					...
					------------------------------
					if (0 <= reg[rrr] < CC..C)
					  IC += ADDRESS[reg[rrr]];
					else
					  IC += ADDRESS[CC..C];
					*/

pub const CCL_ReadRegister: c_int = 0x0E; /* Read bytes into registers:
					1:CCCCCCCCCCCCCCCCCCCCrrrXXXXX
					2:CCCCCCCCCCCCCCCCCCCCrrrXXXXX
					...
					------------------------------
					while (CCC--)
					  read (reg[rrr]);
					*/

pub const CCL_WriteExprConst: c_int = 0x0F; /* write result of expression:
					1:00000OPERATION000RRR000XXXXX
					2:CONSTANT
					------------------------------
					write (reg[RRR] OPERATION CONSTANT);
					IC++;
					*/

/* Note: If the Nth read is suspended, the resumed execution starts
   from the Nth code.  */

pub const CCL_ReadBranch: c_int = 0x10; /* Read one byte into a register,
					and jump by branch table:
					1:CCCCCCCCCCCCCCCCCCCCrrrXXXXX
					2:A--D--D--R--E-S-S[0]000XXXXX
					3:A--D--D--R--E-S-S[1]000XXXXX
					...
					------------------------------
					read (read[rrr]);
					if (0 <= reg[rrr] < CC..C)
					  IC += ADDRESS[reg[rrr]];
					else
					  IC += ADDRESS[CC..C];
					*/

pub const CCL_WriteRegister: c_int = 0x11; /* Write registers:
					1:CCCCCCCCCCCCCCCCCCCrrrXXXXX
					2:CCCCCCCCCCCCCCCCCCCrrrXXXXX
					...
					------------------------------
					while (CCC--)
					  write (reg[rrr]);
					...
					*/

/* Note: If the Nth write is suspended, the resumed execution
   starts from the Nth code.  */

pub const CCL_WriteExprRegister: c_int = 0x12; /* Write result of expression
					1:00000OPERATIONRrrRRR000XXXXX
					------------------------------
					write (reg[RRR] OPERATION reg[Rrr]);
					*/

pub const CCL_Call: c_int = 0x13; /* Call the CCL program whose ID is
					CC..C or cc..c.
					1:CCCCCCCCCCCCCCCCCCCCFFFXXXXX
					[2:00000000cccccccccccccccccccc]
					------------------------------
					if (FFF)
					  call (cc..c)
					  IC++;
					else
					  call (CC..C)
					*/

pub const CCL_WriteConstString: c_int = 0x14; /* Write a constant or a string:
					1:CCCCCCCCCCCCCCCCCCCCrrrXXXXX
					[2:000MSTRIN[0]STRIN[1]STRIN[2]]
					[...]
					-----------------------------
					if (!rrr)
					  write (CC..C)
					else
					  if (M)
					    write_multibyte_string (STRING, CC..C);
					  else
					    write_string (STRING, CC..C);
					  IC += (CC..C + 2) / 3;
					*/

pub const CCL_WriteArray: c_int = 0x15; /* Write an element of array:
					1:CCCCCCCCCCCCCCCCCCCCrrrXXXXX
					2:ELEMENT[0]
					3:ELEMENT[1]
					...
					------------------------------
					if (0 <= reg[rrr] < CC..C)
					  write (ELEMENT[reg[rrr]]);
					IC += CC..C;
					*/

pub const CCL_End: c_int = 0x16; /* Terminate:
					1:00000000000000000000000XXXXX
					------------------------------
					terminate ();
					*/

/* The following two codes execute an assignment arithmetic/logical
   operation.  The form of the operation is like REG OP= OPERAND.  */

pub const CCL_ExprSelfConst: c_int = 0x17; /* REG OP= constant:
					1:00000OPERATION000000rrrXXXXX
					2:CONSTANT
					------------------------------
					reg[rrr] OPERATION= CONSTANT;
					*/

pub const CCL_ExprSelfReg: c_int = 0x18; /* REG1 OP= REG2:
					1:00000OPERATION000RRRrrrXXXXX
					------------------------------
					reg[rrr] OPERATION= reg[RRR];
					*/

/* The following codes execute an arithmetic/logical operation.  The
   form of the operation is like REG_X = REG_Y OP OPERAND2.  */

pub const CCL_SetExprConst: c_int = 0x19; /* REG_X = REG_Y OP constant:
					1:00000OPERATION000RRRrrrXXXXX
					2:CONSTANT
					------------------------------
					reg[rrr] = reg[RRR] OPERATION CONSTANT;
					IC++;
					*/

pub const CCL_SetExprReg: c_int = 0x1A; /* REG1 = REG2 OP REG3:
					1:00000OPERATIONRrrRRRrrrXXXXX
					------------------------------
					reg[rrr] = reg[RRR] OPERATION reg[Rrr];
					*/

pub const CCL_JumpCondExprConst: c_int = 0x1B; /* Jump conditional according to
					an operation on constant:
					1:A--D--D--R--E--S--S-rrrXXXXX
					2:OPERATION
					3:CONSTANT
					-----------------------------
					reg[7] = reg[rrr] OPERATION CONSTANT;
					if (!(reg[7]))
					  IC += ADDRESS;
					else
					  IC += 2
					*/

pub const CCL_JumpCondExprReg: c_int = 0x1C; /* Jump conditional according to
					an operation on register:
					1:A--D--D--R--E--S--S-rrrXXXXX
					2:OPERATION
					3:RRR
					-----------------------------
					reg[7] = reg[rrr] OPERATION reg[RRR];
					if (!reg[7])
					  IC += ADDRESS;
					else
					  IC += 2;
					*/

pub const CCL_ReadJumpCondExprConst: c_int = 0x1D; /* Read and jump conditional according
					  to an operation on constant:
					1:A--D--D--R--E--S--S-rrrXXXXX
					2:OPERATION
					3:CONSTANT
					-----------------------------
					read (reg[rrr]);
					reg[7] = reg[rrr] OPERATION CONSTANT;
					if (!reg[7])
					  IC += ADDRESS;
					else
					  IC += 2;
					*/

pub const CCL_ReadJumpCondExprReg: c_int = 0x1E; /* Read and jump conditional according
					to an operation on register:
					1:A--D--D--R--E--S--S-rrrXXXXX
					2:OPERATION
					3:RRR
					-----------------------------
					read (reg[rrr]);
					reg[7] = reg[rrr] OPERATION reg[RRR];
					if (!reg[7])
					  IC += ADDRESS;
					else
					  IC += 2;
					*/

pub const CCL_Extension: c_int = 0x1F; /* Extended CCL code
					1:ExtendedCOMMNDRrrRRRrrrXXXXX
					2:ARGUMENT
					3:...
					------------------------------
					extended_command (rrr,RRR,Rrr,ARGS)
				      */

/*
   Here after, Extended CCL Instructions.
   Bit length of extended command is 14.
   Therefore, the instruction code range is 0..16384(0x3fff).
 */

/* Read a multibyte character.
   A code point is stored into reg[rrr].  A charset ID is stored into
   reg[RRR].  */

pub const CCL_ReadMultibyteChar2: c_int = 0x00; /* Read Multibyte Character
					1:ExtendedCOMMNDRrrRRRrrrXXXXX  */

/* Write a multibyte character.
   Write a character whose code point is reg[rrr] and the charset ID
   is reg[RRR].  */

pub const CCL_WriteMultibyteChar2: c_int = 0x01; /* Write Multibyte Character
					1:ExtendedCOMMNDRrrRRRrrrXXXXX  */

/* Translate a character whose code point is reg[rrr] and the charset
   ID is reg[RRR] by a translation table whose ID is reg[Rrr].

   A translated character is set in reg[rrr] (code point) and reg[RRR]
   (charset ID).  */

pub const CCL_TranslateCharacter: c_int = 0x02; /* Translate a multibyte character
					1:ExtendedCOMMNDRrrRRRrrrXXXXX  */

/* Translate a character whose code point is reg[rrr] and the charset
   ID is reg[RRR] by a translation table whose ID is ARGUMENT.

   A translated character is set in reg[rrr] (code point) and reg[RRR]
   (charset ID).  */

pub const CCL_TranslateCharacterConstTbl: c_int = 0x03; /* Translate a multibyte character
					       1:ExtendedCOMMNDRrrRRRrrrXXXXX
					       2:ARGUMENT(Translation Table ID)
					    */

/* Iterate looking up MAPs for reg[rrr] starting from the Nth (N =
   reg[RRR]) MAP until some value is found.

   Each MAP is a Lisp vector whose element is number, nil, t, or
   lambda.
   If the element is nil, ignore the map and proceed to the next map.
   If the element is t or lambda, finish without changing reg[rrr].
   If the element is a number, set reg[rrr] to the number and finish.

   Detail of the map structure is described in the comment for
   CCL_MapMultiple below.  */

pub const CCL_IterateMultipleMap: c_int = 0x10; /* Iterate multiple maps
					1:ExtendedCOMMNDXXXRRRrrrXXXXX
					2:NUMBER of MAPs
					3:MAP-ID1
					4:MAP-ID2
					...
				     */

/* Map the code in reg[rrr] by MAPs starting from the Nth (N =
   reg[RRR]) map.

   MAPs are supplied in the succeeding CCL codes as follows:

   When CCL program gives this nested structure of map to this command:
	((MAP-ID11
	  MAP-ID12
	  (MAP-ID121 MAP-ID122 MAP-ID123)
	  MAP-ID13)
	 (MAP-ID21
	  (MAP-ID211 (MAP-ID2111) MAP-ID212)
	  MAP-ID22)),
   the compiled CCL codes has this sequence:
	CCL_MapMultiple (CCL code of this command)
	16 (total number of MAPs and SEPARATORs)
	-7 (1st SEPARATOR)
	MAP-ID11
	MAP-ID12
	-3 (2nd SEPARATOR)
	MAP-ID121
	MAP-ID122
	MAP-ID123
	MAP-ID13
	-7 (3rd SEPARATOR)
	MAP-ID21
	-4 (4th SEPARATOR)
	MAP-ID211
	-1 (5th SEPARATOR)
	MAP_ID2111
	MAP-ID212
	MAP-ID22

   A value of each SEPARATOR follows this rule:
	MAP-SET := SEPARATOR [(MAP-ID | MAP-SET)]+
	SEPARATOR := -(number of MAP-IDs and SEPARATORs in the MAP-SET)

   (*)....Nest level of MAP-SET must not be over than MAX_MAP_SET_LEVEL.

   When some map fails to map (i.e. it doesn't have a value for
   reg[rrr]), the mapping is treated as identity.

   The mapping is iterated for all maps in each map set (set of maps
   separated by SEPARATOR) except in the case that lambda is
   encountered.  More precisely, the mapping proceeds as below:

   At first, VAL0 is set to reg[rrr], and it is translated by the
   first map to VAL1.  Then, VAL1 is translated by the next map to
   VAL2.  This mapping is iterated until the last map is used.  The
   result of the mapping is the last value of VAL?.  When the mapping
   process reached to the end of the map set, it moves to the next
   map set.  If the next does not exit, the mapping process terminates,
   and regard the last value as a result.

   But, when VALm is mapped to VALn and VALn is not a number, the
   mapping proceed as below:

   If VALn is nil, the last map is ignored and the mapping of VALm
   proceed to the next map.

   In VALn is t, VALm is reverted to reg[rrr] and the mapping of VALm
   proceed to the next map.

   If VALn is lambda, move to the next map set like reaching to the
   end of the current map set.

   If VALn is a symbol, call the CCL program referred by it.
   Then, use reg[rrr] as a mapped value except for -1, -2 and -3.
   Such special values are regarded as nil, t, and lambda respectively.

   Each map is a Lisp vector of the following format (a) or (b):
	(a)......[STARTPOINT VAL1 VAL2 ...]
	(b)......[t VAL STARTPOINT ENDPOINT],
   where
	STARTPOINT is an offset to be used for indexing a map,
	ENDPOINT is a maximum index number of a map,
	VAL and VALn is a number, nil, t, or lambda.

   Valid index range of a map of type (a) is:
	STARTPOINT <= index < STARTPOINT + map_size - 1
   Valid index range of a map of type (b) is:
	STARTPOINT <= index < ENDPOINT	*/

pub const CCL_MapMultiple: c_int = 0x11; /* Mapping by multiple code conversion maps
					 1:ExtendedCOMMNDXXXRRRrrrXXXXX
					 2:N-2
					 3:SEPARATOR_1 (< 0)
					 4:MAP-ID_1
					 5:MAP-ID_2
					 ...
					 M:SEPARATOR_x (< 0)
					 M+1:MAP-ID_y
					 ...
					 N:SEPARATOR_z (< 0)
				      */

pub const MAX_MAP_SET_LEVEL: c_int = 30;

/* C 581-586's `typedef struct { int rest_length; int orig_val; } tr_stack;` plus the two
   `static` objects and the second static, in C's order.  `tr_stack` is a plain C record, so
   `Copy` is exact (PORTING.md 0.1-9's rule for C's trivially copyable records), and C's
   `static tr_stack mapping_stack[MAX_MAP_SET_LEVEL];` is zero-initialised like every C
   object with static storage duration.  */
#[repr(C)]
#[derive (Clone, Copy)]
pub struct tr_stack
{
  pub rest_length: c_int,
  pub orig_val: c_int,
}

static mut mapping_stack: [tr_stack; MAX_MAP_SET_LEVEL as usize] =
  [tr_stack { rest_length: 0, orig_val: 0 }; MAX_MAP_SET_LEVEL as usize];
static mut mapping_stack_pointer: *mut tr_stack = std::ptr::null_mut ();

/* If this variable is non-zero, it indicates the stack_idx
   of immediately called by CCL_MapMultiple. */
static mut stack_idx_of_map_multiple: c_int = 0;


/* C 594-604's `PUSH_MAPPING_STACK (restlen, orig)`; C's `do { … } while (0)` is a block
   expression (PORTING.md 2.1), and `mapping_stack_pointer++` is the pointer step C's
   post-increment performs.  */
macro_rules! PUSH_MAPPING_STACK {
  ($restlen:expr, $orig:expr) => {
    {
      (*mapping_stack_pointer).rest_length = ($restlen);
      (*mapping_stack_pointer).orig_val = ($orig);
      mapping_stack_pointer = mapping_stack_pointer.offset (1);
    }
  };
}

/* Work around GCC bug 123042
   https://gcc.gnu.org/bugzilla/show_bug.cgi?id=123042
   which causes GCC to mistakenly complain about
   popping the mapping stack.  */
// C 607-609's `#if 13 <= __GNUC__` / `# pragma GCC diagnostic ignored
// "-Wanalyzer-out-of-bounds"` / `#endif`: `__GNUC__` is 4 on the frozen target, so the
// guard is false and the pragma is not part of the frozen build.  A pragma has no Rust
// spelling, so its text is recorded in docs/ports/ccl.md rather than carried as a
// `#[cfg (feature = …)]` arm (P-51/status 3.49); this is also the one `#if` of this file
// the port does not put to the frozen macro table, since ccl.c is not in
// cond-eval-verified.txt (clang and tools/check-cond-eval.py agree it is false).

/* C 611-618's `POP_MAPPING_STACK (restlen, orig)`: `mapping_stack_pointer--` is the pointer
   step, and `restlen`/`orig` are places C assigns (they are `map_set_rest_length`, `op`,
   `reg[rrr]` at the call sites), which the macro keeps.  */
macro_rules! POP_MAPPING_STACK {
  ($restlen:expr, $orig:expr) => {
    {
      mapping_stack_pointer = mapping_stack_pointer.offset (-1);
      ($restlen) = (*mapping_stack_pointer).rest_length;
      ($orig) = (*mapping_stack_pointer).orig_val;
    }
  };
}

/* C 620-644's `CCL_CALL_FOR_MAP_INSTRUCTION (symbol, ret_ic)` captures `ccl_driver`'s
   `ccl_prog`, `ic`, `eof_ic` and `stack_idx`, declares `called_ccl` and jumps to
   `ccl_repeat`, so its definition is inside that function, after those locals and inside
   the loop label it jumps to (a `macro_rules!` resolves locals and labels at its
   definition site; see the note at the top of this unit).  */
pub const CCL_MapSingle: c_int = 0x12; /* Map by single code conversion map
					1:ExtendedCOMMNDXXXRRRrrrXXXXX
					2:MAP-ID
					------------------------------
					Map reg[rrr] by MAP-ID.
					If some valid mapping is found,
					  set reg[rrr] to the result,
					else
					  set reg[RRR] to -1.
				     */

pub const CCL_LookupIntConstTbl: c_int = 0x13; /* Lookup multibyte character by
				      integer key.  Afterwards R7 set
				      to 1 if lookup succeeded.
				      1:ExtendedCOMMNDRrrRRRXXXXXXXX
				      2:ARGUMENT(Hash table ID) */

pub const CCL_LookupCharConstTbl: c_int = 0x14; /* Lookup integer by multibyte
				       character key.  Afterwards R7 set
				       to 1 if lookup succeeded.
				       1:ExtendedCOMMNDRrrRRRrrrXXXXX
				       2:ARGUMENT(Hash table ID) */

/* CCL arithmetic/logical operators. */
pub const CCL_PLUS: c_int = 0x00; /* X = Y + Z */
pub const CCL_MINUS: c_int = 0x01; /* X = Y - Z */
pub const CCL_MUL: c_int = 0x02; /* X = Y * Z */
pub const CCL_DIV: c_int = 0x03; /* X = Y / Z */
pub const CCL_MOD: c_int = 0x04; /* X = Y % Z */
pub const CCL_AND: c_int = 0x05; /* X = Y & Z */
pub const CCL_OR: c_int = 0x06; /* X = Y | Z */
pub const CCL_XOR: c_int = 0x07; /* X = Y ^ Z */
pub const CCL_LSH: c_int = 0x08; /* X = Y << Z */
pub const CCL_RSH: c_int = 0x09; /* X = Y >> Z */
pub const CCL_LSH8: c_int = 0x0A; /* X = (Y << 8) | Z */
pub const CCL_RSH8: c_int = 0x0B; /* X = Y >> 8, r[7] = Y & 0xFF  */
pub const CCL_DIVMOD: c_int = 0x0C; /* X = Y / Z, r[7] = Y % Z */
pub const CCL_LS: c_int = 0x10; /* X = (X < Y) */
pub const CCL_GT: c_int = 0x11; /* X = (X > Y) */
pub const CCL_EQ: c_int = 0x12; /* X = (X == Y) */
pub const CCL_LE: c_int = 0x13; /* X = (X <= Y) */
pub const CCL_GE: c_int = 0x14; /* X = (X >= Y) */
pub const CCL_NE: c_int = 0x15; /* X = (X != Y) */

pub const CCL_DECODE_SJIS: c_int = 0x16; /* X = HIGHER_BYTE (DE-SJIS (Y, Z))
				   r[7] = LOWER_BYTE (DE-SJIS (Y, Z)) */
pub const CCL_ENCODE_SJIS: c_int = 0x17; /* X = HIGHER_BYTE (SJIS (Y, Z))
				   r[7] = LOWER_BYTE (SJIS (Y, Z) */


/* Terminate CCL program successfully.  */
// C 696-704's `CCL_SUCCESS` is defined inside `ccl_driver`: its body sets `ccl->status`
// and jumps to `ccl_finish`, and both are resolved at the macro's definition site (see the
// note at the top of this unit).

/* Suspend CCL program because of reading from empty input buffer or
   writing to full output buffer.  When this program is resumed, the
   same I/O command is executed.  */
// C 707-717's `CCL_SUSPEND (stat)` is defined inside `ccl_driver` (it writes `ic`,
// `ccl->status` and jumps to `ccl_finish`).

/* Terminate CCL program because of invalid command.  Should not occur
   in the normal case.  */
// C 720-738's two `CCL_INVALID_CMD` definitions (`#ifndef CCL_DEBUG` and its `#else` arm)
// are both defined inside `ccl_driver`, each behind its `#[cfg]` (ccl.c is not in
// cond-eval-verified.txt, so both arms are carried, P-51/status 3.49).

/* Use "&" rather than "&&" to suppress a bogus GCC warning; see
   <https://gcc.gnu.org/bugzilla/show_bug.cgi?id=43772>.  */
macro_rules! ASCENDING_ORDER {
  ($lo:expr, $med:expr, $hi:expr) =>
    { ((($lo) as isize) <= (($med) as isize)) & ((($med) as isize) <= (($hi) as isize)) };
}

// C 745-756's `GET_CCL_RANGE` and `GET_CCL_CODE` are defined inside `ccl_driver`: their
// bodies invoke `CCL_INVALID_CMD`, which is only in scope where that macro is (a
// `macro_rules!` call inside a macro body resolves where the outer macro is *defined*).

macro_rules! IN_INT_RANGE {
  ($val:expr) => { ASCENDING_ORDER! (INT_MIN, $val, INT_MAX) };
}

/* Encode one character CH to multibyte form and write to the current
   output buffer.  If CH is less than 256, CH is written as is.  */
// C 762-770's `CCL_WRITE_CHAR (ch)` is defined inside `ccl_driver` (it writes `dst`, reads
// `dst_end` and jumps to `ccl_error_handler`/`ccl_finish`).

/* Write a string at ccl_prog[IC] of length LEN to the current output
   buffer.  */
// C 774-791's `CCL_WRITE_STRING (len)` is defined inside `ccl_driver` (it writes `dst`,
// reads `dst_end`, `ccl_prog` and `ic`).

/* Read one byte from the current input buffer into Rth register.  */
// C 794-808's `CCL_READ_CHAR (r)` is defined inside `ccl_driver` (it reads `src`, `src_end`,
// `ccl` and `ic` and jumps to `ccl_repeat`/`ccl_error_handler`/`ccl_finish`).

/* Decode CODE by a charset whose id is ID.  If ID is 0, return CODE
   as is for backward compatibility.  Assume that we can use the
   variable `charset'.  */

// C 814-816's `CCL_DECODE_CHAR (id, code)` is defined inside `ccl_driver` (it writes the
// local `charset`).

/* Encode character C by some of charsets in CHARSET_LIST.  Set ID to
   the id of the used charset, ENCODED to the result of encoding.
   Assume that we can use the variable `charset'.  */

// C 822-834's `CCL_ENCODE_CHAR (c, charset_list, id, encoded)` is defined inside
// `ccl_driver` (it writes the local `charset` and its `id`/`encoded` places).

/* Execute CCL code on characters at SOURCE (length SRC_SIZE).  The
   resulting text goes to a place pointed by DESTINATION, the length
   of which should not exceed DST_SIZE.  As a side effect, how many
   characters are consumed and produced are recorded in CCL->consumed
   and CCL->produced, and the contents of CCL registers are updated.
   If SOURCE or DESTINATION is NULL, only operations on registers are
   permitted.  */

/* C 844-855's `#ifdef CCL_DEBUG` arm.  `CCL_DEBUG` is a source-level macro, not a `config.h`
   key, so the arm is carried behind `#[cfg (feature = "ccl_debug")]` (P-51/status 3.49) and
   is not part of `xtask dead-arms`.  `CCL_DEBUG_BACKTRACE_LEN` is a constant there, so it
   is one here too.  */
#[cfg (feature = "ccl_debug")]
pub const CCL_DEBUG_BACKTRACE_LEN: c_int = 256;

#[cfg (feature = "ccl_debug")]
#[unsafe(no_mangle)]
pub static mut ccl_backtrace_table: [c_int; CCL_DEBUG_BACKTRACE_LEN as usize] =
  [0; CCL_DEBUG_BACKTRACE_LEN as usize];

#[cfg (feature = "ccl_debug")]
#[unsafe(no_mangle)]
pub static mut ccl_backtrace_idx: c_int = 0;

/* C 849-853: `int ccl_debug_hook (int ic) { return ic; }`, C's non-`static` definition, so
   it carries `#[unsafe(no_mangle)]` (PORTING.md 0.2).  */
#[cfg (feature = "ccl_debug")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ccl_debug_hook (ic: c_int) -> c_int
{
  return ic;
}

/// C: `struct ccl_prog_stack` C 857-862.  It is declared in this unit (C declares it in the
/// .c file), so `tools/gen-layout-fields.py`'s `STRUCTS` list does not name it; clang,
/// against the frozen target, gives `sizeof` 16 with `ccl_prog` at 0, `ic` at 8 and
/// `eof_ic` at 12, and the assertion pins those.  The record is plainly copyable in C, so
/// `Copy` is exact (PORTING.md 0.1-9) and the 256-element array below can be zeroed.
#[repr(C)]
#[derive (Clone, Copy)]
pub struct ccl_prog_stack
{
  pub ccl_prog: *mut Lisp_Object,	/* Pointer to an array of CCL code.  */
  pub ic: c_int,			/* Instruction Counter.  */
  pub eof_ic: c_int,			/* Instruction Counter to jump on EOF.  */
}

const _: () = core::assert! (std::mem::size_of::<ccl_prog_stack> () == 16);
const _: () = core::assert! (std::mem::offset_of! (ccl_prog_stack, ccl_prog) == 0);
const _: () = core::assert! (std::mem::offset_of! (ccl_prog_stack, ic) == 8);
const _: () = core::assert! (std::mem::offset_of! (ccl_prog_stack, eof_ic) == 12);

/* For the moment, we only support depth 256 of stack.  */
static mut ccl_prog_stack_struct: [ccl_prog_stack; 256] =
  [ccl_prog_stack { ccl_prog: std::ptr::null_mut (), ic: 0, eof_ic: 0 }; 256];

/* Return a translation table of id number ID.  */
#[inline]
unsafe fn GET_TRANSLATION_TABLE (id: c_int) -> Lisp_Object
{
  return XCDR (AREF (Vtranslation_table_vector, id as isize));
}

/* C's five labels have no Rust counterpart, so the port names them with C's own label
   text in this enum: the `for (;;)` body is one `'ccl_repeat` loop that leaves with the
   label a C `goto` would reach, and `none` is the state of "no shared tail" (C's two
   labels *inside* the switch are an entry stage, PORTING.md 2.2).  */
#[derive (PartialEq)]
enum ccl_goto
{
  none,
  ccl_repeat,
  ccl_set_expr,
  ccl_expr_self,
  ccl_error_handler,
  ccl_finish,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ccl_driver (ccl: *mut ccl_program, source: *mut c_int,
				     destination: *mut c_int, src_size: c_int,
				     dst_size: c_int, charset_list: Lisp_Object)
{
  let reg: *mut c_int = (*ccl).reg.as_mut_ptr ();
  let mut ic: c_int = (*ccl).ic;
  /* PORTING.md 1.5: C declares `code = 0, field1, field2` and leaves the last two
     uninitialised; the port zero-initialises them.  */
  let mut code: c_int = 0;
  let mut field1: c_int = 0;
  let mut field2: c_int = 0;
  let mut ccl_prog: *mut Lisp_Object = (*ccl).prog;
  let mut src: *mut c_int = source;
  let mut src_end: *mut c_int = source.offset (src_size as isize);
  let mut dst: *mut c_int = destination;
  let mut dst_end: *mut c_int = destination.offset (dst_size as isize);
  /* PORTING.md 1.5: C leaves `jump_address` uninitialised.  */
  let mut jump_address: c_int = 0;
  /* PORTING.md 1.5: C declares `i = 0, j, op`.  */
  let mut i: c_int = 0;
  let mut j: c_int = 0;
  let mut op: c_int = 0;
  let mut stack_idx: c_int = (*ccl).stack_idx;
  /* Instruction counter of the current CCL code. */
  let mut this_ic: c_int = 0;
  /* PORTING.md 1.5: C leaves `charset` uninitialised (it is assigned before every use).  */
  let mut charset: *mut charset = std::ptr::null_mut ();
  let mut eof_ic: c_int = (*ccl).eof_ic;
  let mut eof_hit: c_int = 0;

  if (*ccl).buf_magnification == 0 /* We can't read/produce any bytes.  */
    { dst = std::ptr::null_mut (); }

  /* Set mapping stack pointer. */
  mapping_stack_pointer = (&raw mut mapping_stack) as *mut tr_stack;

  /* C 898-900's `#ifdef CCL_DEBUG` arm (carried behind the feature, see above).  */
  #[cfg (feature = "ccl_debug")]
  { ccl_backtrace_idx = 0; }

  /* C's `for (;;)` is this loop: `goto ccl_repeat` is `continue 'ccl_repeat`, the loop's
     own `break` (the quit test below) and `goto ccl_error_handler` leave with
     `ccl_goto::ccl_error_handler`, and `goto ccl_finish` with `ccl_goto::ccl_finish`
     (PORTING.md 2.2).  */
  let ccl_goto: ccl_goto = 'ccl_repeat: loop
    {
    /* ccl_repeat: */
      /* C 620-934: the five field aliases, the nine statement macros and
	 `GET_CCL_RANGE`/`GET_CCL_CODE`.  C defines the aliases at C 926 and the statement
	 macros at file scope, over this function's locals; a `macro_rules!` resolves local
	 variables *and labels* at its **definition** site, so the port defines them all
	 here — after the locals they capture, inside the `'ccl_repeat` label they jump to
	 and before the loop's first `GET_CCL_CODE` (C 913), which is where P-57's
	 definition-before-use order puts them (the bytecode.rs placement).  Every call site
	 below therefore keeps C's own shape.  A macro expansion is not a place, so C's four
	 `rrr = 7` are written `field2 = 7` — C's own preprocessed text (the bytecode.rs
	 `TOP = x` precedent).  */
      macro_rules! rrr { () => { field2 } }
      macro_rules! RRR { () => { field1 & 7 } }
      macro_rules! Rrr { () => { (field1 >> 3) & 7 } }
      macro_rules! ADDR { () => { field1 } }
      macro_rules! EXCMD { () => { field1 >> 6 } }

      /* C 620-644: `CCL_CALL_FOR_MAP_INSTRUCTION (symbol, ret_ic)`.  */
      macro_rules! CCL_CALL_FOR_MAP_INSTRUCTION {
	($symbol:expr, $ret_ic:expr) => {
	  {
	    /* C declares `struct ccl_program called_ccl;` uninitialised (PORTING.md 1.5).  */
	    let mut called_ccl: ccl_program = std::mem::zeroed ();
	    if stack_idx >= 256
		|| ! setup_ccl_program (&raw mut called_ccl, $symbol)
	      {
		if stack_idx > 0
		  {
		    ccl_prog = ccl_prog_stack_struct[0].ccl_prog;
		    ic = ccl_prog_stack_struct[0].ic;
		    eof_ic = ccl_prog_stack_struct[0].eof_ic;
		  }
		CCL_INVALID_CMD! ();
	      }
	    ccl_prog_stack_struct[stack_idx as usize].ccl_prog = ccl_prog;
	    ccl_prog_stack_struct[stack_idx as usize].ic = ($ret_ic);
	    ccl_prog_stack_struct[stack_idx as usize].eof_ic = eof_ic;
	    stack_idx += 1;
	    ccl_prog = called_ccl.prog;
	    ic = CCL_HEADER_MAIN;
	    eof_ic = XFIXNAT (*ccl_prog.offset (CCL_HEADER_EOF as isize)) as c_int;
	    continue 'ccl_repeat;
	  }
	};
      }

      /* C 696-704: `CCL_SUCCESS`.  */
      macro_rules! CCL_SUCCESS {
	() => {
	  {
	    (*ccl).status = CCL_STAT_SUCCESS;
	    break 'ccl_repeat ccl_goto::ccl_finish;
	  }
	};
      }

      /* C 707-717: `CCL_SUSPEND (stat)`.  `ic--` is C's pre-decrement.  */
      macro_rules! CCL_SUSPEND {
	($stat:expr) => {
	  {
	    ic -= 1;
	    (*ccl).status = $stat;
	    break 'ccl_repeat ccl_goto::ccl_finish;
	  }
	};
      }

      /* C 720-738: the two `CCL_INVALID_CMD` definitions; ccl.c is not in
	 cond-eval-verified.txt, so both arms are carried behind their `#[cfg]`
	 (P-51/status 3.49).  */
      #[cfg (not (feature = "ccl_debug"))]
      macro_rules! CCL_INVALID_CMD {
	() => {
	  {
	    (*ccl).status = CCL_STAT_INVALID_CMD;
	    break 'ccl_repeat ccl_goto::ccl_error_handler;
	  }
	};
      }

      #[cfg (feature = "ccl_debug")]
      macro_rules! CCL_INVALID_CMD {
	() => {
	  {
	    ccl_debug_hook (this_ic);
	    (*ccl).status = CCL_STAT_INVALID_CMD;
	    break 'ccl_repeat ccl_goto::ccl_error_handler;
	  }
	};
      }

      /* C 745-756: `GET_CCL_RANGE` and `GET_CCL_CODE`, defined here so that the
	 `CCL_INVALID_CMD` their bodies invoke is in scope (C has them before this function;
	 a `macro_rules!` call inside a macro body resolves at the outer macro's definition
	 site, so they cannot stay there).  C passes `ic++` where the port passes `ic` and
	 steps it after the call; the only difference would be the unobservable one of
	 `ic` already being stepped when the macro jumps out.  */
      macro_rules! GET_CCL_RANGE {
	($var:expr, $ccl_prog:expr, $ic:expr, $lo:expr, $hi:expr) => {
	  {
	    let prog_word: EMACS_INT = XFIXNUM (*$ccl_prog.offset (($ic) as isize));
	    if ! ASCENDING_ORDER! ($lo, prog_word, $hi)
	      { CCL_INVALID_CMD! (); }
	    $var = prog_word as _;
	  }
	};
      }

      macro_rules! GET_CCL_CODE {
	($code:expr, $ccl_prog:expr, $ic:expr) =>
	  { GET_CCL_RANGE! ($code, $ccl_prog, $ic, CCL_CODE_MIN, CCL_CODE_MAX) };
      }

      /* C 762-770: `CCL_WRITE_CHAR (ch)`.  `*dst++ = (ch)` is the store followed by the
	 pointer step C's post-increment performs.  */
      macro_rules! CCL_WRITE_CHAR {
	($ch:expr) => {
	  {
	    if dst.is_null ()
	      { CCL_INVALID_CMD! (); }
	    else if dst < dst_end
	      { *dst = ($ch); dst = dst.offset (1); }
	    else
	      { CCL_SUSPEND! (CCL_STAT_SUSPEND_BY_DST); }
	  }
	};
      }

      /* C 774-791: `CCL_WRITE_STRING (len)`.  The two `for` loops keep C's test/body/step
	 order (the dired.rs `while` shape) and `*dst++ = …` is again store-then-step.  */
      macro_rules! CCL_WRITE_STRING {
	($len:expr) => {
	  {
	    /* PORTING.md 1.5: C declares `int ccli;` uninitialised.  */
	    let mut ccli: c_int = 0;
	    if dst.is_null ()
	      { CCL_INVALID_CMD! (); }
	    else if dst.offset (($len) as isize) <= dst_end
	      {
		if XFIXNAT (*ccl_prog.offset (ic as isize)) & 0x1000000 != 0
		  {
		    ccli = 0;
		    while ccli < ($len)
		      {
			*dst = (XFIXNAT (*ccl_prog.offset (ic as isize + ccli as isize))
				& 0xFFFFFF) as c_int;
			dst = dst.offset (1);
			ccli += 1;
		      }
		  }
		else
		  {
		    ccli = 0;
		    while ccli < ($len)
		      {
			*dst = ((XFIXNAT (*ccl_prog.offset
					  (ic as isize + (ccli / 3) as isize))
				 >> ((2 - (ccli % 3)) * 8)) & 0xFF) as c_int;
			dst = dst.offset (1);
			ccli += 1;
		      }
		  }
	      }
	    else
	      { CCL_SUSPEND! (CCL_STAT_SUSPEND_BY_DST); }
	  }
	};
      }

      /* C 794-808: `CCL_READ_CHAR (r)`.  `r = *src++` is the load followed by the pointer
	 step, and `goto ccl_repeat` is this loop's `continue`.  `ccl->last_block` is the
	 mirror's `bits` byte (the `last_block` family of docs/ports/token-aliases.tsv).  */
      macro_rules! CCL_READ_CHAR {
	($r:expr) => {
	  {
	    if src.is_null ()
	      { CCL_INVALID_CMD! (); }
	    else if src < src_end
	      { $r = *src; src = src.offset (1); }
	    else if (*ccl).bits & CCL_PROGRAM_LAST_BLOCK != 0
	      {
		$r = -1;
		ic = (*ccl).eof_ic;
		continue 'ccl_repeat;
	      }
	    else
	      { CCL_SUSPEND! (CCL_STAT_SUSPEND_BY_SRC); }
	  }
	};
      }

      /* C 814-816: `CCL_DECODE_CHAR (id, code)`; C's comma expression is the block that
	 assigns `charset` and yields the decoded code.  */
      macro_rules! CCL_DECODE_CHAR {
	($id:expr, $code:expr) => {
	  (if ($id) == 0
	     { $code }
	   else
	     {
	       charset = CHARSET_FROM_ID! ($id);
	       DECODE_CHAR! (charset, $code)
	     })
	};
      }

      /* C 822-834: `CCL_ENCODE_CHAR (c, charset_list, id, encoded)`; `id` and `encoded`
	 are the places C assigns (`reg[RRR]`, `reg[rrr]` at the call sites).  */
      macro_rules! CCL_ENCODE_CHAR {
	($c:expr, $charset_list:expr, $id:expr, $encoded:expr) => {
	  {
	    /* PORTING.md 1.5: C declares `unsigned ncode;` uninitialised.  */
	    let mut ncode: c_uint = 0;

	    charset = char_charset ($c, $charset_list, &raw mut ncode);
	    if charset.is_null () && ! NILP ($charset_list)
	      { charset = char_charset ($c, Qnil (), &raw mut ncode); }
	    if !charset.is_null ()
	      {
		$id = CHARSET_ID! (charset);
		$encoded = ncode as c_int;
	      }
	  }
	};
      }

    /* C 905-910's `#ifdef CCL_DEBUG` arm.  `ccl_backtrace_idx++` is the index-first,
       increment-second pair C's post-increment performs.  */
    #[cfg (feature = "ccl_debug")]
    {
      ccl_backtrace_table[ccl_backtrace_idx as usize] = ic;
      ccl_backtrace_idx += 1;
      if ccl_backtrace_idx >= CCL_DEBUG_BACKTRACE_LEN
	{ ccl_backtrace_idx = 0; }
      ccl_backtrace_table[ccl_backtrace_idx as usize] = 0;
    }

      if !NILP (Vquit_flag) && NILP (Vinhibit_quit)
	{
	  /* We can't just signal Qquit, instead break the loop as if
             the whole data is processed.  Don't reset Vquit_flag, it
             must be handled later at a safer place.  */
	  if !src.is_null ()
	    { src = source.offset (src_size as isize); }
	  (*ccl).status = CCL_STAT_QUIT;
	  /* C's `break` leaves the `for (;;)` for `ccl_error_handler`.  */
	  break 'ccl_repeat ccl_goto::ccl_error_handler;
	}

      this_ic = ic;
      GET_CCL_CODE! (code, ccl_prog, ic); ic += 1;
      field1 = code >> 8;
      field2 = (code & 0xFF) >> 5;

      /* C's two `switch` labels inside the `switch (code & 0x1F)` body.  A Rust `match`
	 arm cannot be jumped into, so the arms that `goto` one of them set `tail` and the
	 shared statements run after the `match` (PORTING.md 2.2, the tparam.rs entry-stage
	 shape).  */
      let mut tail: ccl_goto = ccl_goto::none;

      match code & 0x1F
	{
	CCL_SetRegister =>	/* 00000000000000000RRRrrrXXXXX */
	  {
	    *reg.offset (rrr! () as isize) = *reg.offset (RRR! () as isize);
	  }

	CCL_SetShortConst =>	/* CCCCCCCCCCCCCCCCCCCCrrrXXXXX */
	  {
	    *reg.offset (rrr! () as isize) = field1;
	  }

	CCL_SetConst =>	/* 00000000000000000000rrrXXXXX */
	  {
	    *reg.offset (rrr! () as isize) =
	      XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int; ic += 1;
	  }

	CCL_SetArray =>	/* CCCCCCCCCCCCCCCCCCCCRRRrrrXXXXX */
	  {
	    i = *reg.offset (RRR! () as isize);
	    j = field1 >> 3;
	    if 0 <= i && i < j
	      {
		*reg.offset (rrr! () as isize) =
		  XFIXNUM (*ccl_prog.offset (ic as isize + i as isize)) as c_int;
	      }
	    ic += j;
	  }

	CCL_Jump =>		/* A--D--D--R--E--S--S-000XXXXX */
	  {
	    ic += ADDR! ();
	  }

	CCL_JumpCond =>	/* A--D--D--R--E--S--S-rrrXXXXX */
	  {
	    if *reg.offset (rrr! () as isize) == 0
	      { ic += ADDR! (); }
	  }

	CCL_WriteRegisterJump =>	/* A--D--D--R--E--S--S-rrrXXXXX */
	  {
	    i = *reg.offset (rrr! () as isize);
	    CCL_WRITE_CHAR! (i);
	    ic += ADDR! ();
	  }

	CCL_WriteRegisterReadJump =>	/* A--D--D--R--E--S--S-rrrXXXXX */
	  {
	    i = *reg.offset (rrr! () as isize);
	    CCL_WRITE_CHAR! (i);
	    ic += 1;
	    CCL_READ_CHAR! (*reg.offset (rrr! () as isize));
	    ic += ADDR! () - 1;
	  }

	CCL_WriteConstJump =>	/* A--D--D--R--E--S--S-000XXXXX */
	  {
	    i = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int;
	    CCL_WRITE_CHAR! (i);
	    ic += ADDR! ();
	  }

	CCL_WriteConstReadJump =>	/* A--D--D--R--E--S--S-rrrXXXXX */
	  {
	    i = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int;
	    CCL_WRITE_CHAR! (i);
	    ic += 1;
	    CCL_READ_CHAR! (*reg.offset (rrr! () as isize));
	    ic += ADDR! () - 1;
	  }

	CCL_WriteStringJump =>	/* A--D--D--R--E--S--S-000XXXXX */
	  {
	    j = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int; ic += 1;
	    CCL_WRITE_STRING! (j);
	    ic += ADDR! () - 1;
	  }

	CCL_WriteArrayReadJump =>	/* A--D--D--R--E--S--S-rrrXXXXX */
	  {
	    i = *reg.offset (rrr! () as isize);
	    j = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int;
	    if 0 <= i && i < j
	      {
		i = XFIXNUM (*ccl_prog.offset (ic as isize + 1 + i as isize)) as c_int;
		CCL_WRITE_CHAR! (i);
	      }
	    ic += j + 2;
	    CCL_READ_CHAR! (*reg.offset (rrr! () as isize));
	    ic += ADDR! () - (j + 2);
	  }

	CCL_ReadJump =>	/* A--D--D--R--E--S--S-rrrYYYYY */
	  {
	    CCL_READ_CHAR! (*reg.offset (rrr! () as isize));
	    ic += ADDR! ();
	  }

	CCL_ReadBranch | CCL_Branch =>	/* CCCCCCCCCCCCCCCCCCCCrrrXXXXX */
	  {
	    /* C's `CCL_ReadBranch` reads and `FALLTHROUGH`s into `CCL_Branch`'s statements; a
	       `match` has no fallthrough, so the arm selects the entry (PORTING.md 2.2).  */
	    if code & 0x1F == CCL_ReadBranch
	      { CCL_READ_CHAR! (*reg.offset (rrr! () as isize)); }
	    let ioff: c_int =
	      if 0 <= *reg.offset (rrr! () as isize)
		 && *reg.offset (rrr! () as isize) < field1
		{ *reg.offset (rrr! () as isize) }
	      else
		{ field1 };
	    let incr: c_int =
	      XFIXNUM (*ccl_prog.offset (ic as isize + ioff as isize)) as c_int;
	    ic += incr;
	  }

	CCL_ReadRegister =>	/* CCCCCCCCCCCCCCCCCCCCrrXXXXX */
	  {
	    loop
	      {
		CCL_READ_CHAR! (*reg.offset (rrr! () as isize));
		if field1 == 0 { break; }
		GET_CCL_CODE! (code, ccl_prog, ic); ic += 1;
		field1 = code >> 8;
		field2 = (code & 0xFF) >> 5;
	      }
	  }

	CCL_WriteExprConst =>  /* 1:00000OPERATION000RRR000XXXXX */
	  {
	    field2 = 7;
	    i = *reg.offset (RRR! () as isize);
	    j = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int;
	    op = field1 >> 6;
	    jump_address = ic + 1;
	    tail = ccl_goto::ccl_set_expr;
	  }

	CCL_WriteRegister =>	/* CCCCCCCCCCCCCCCCCCCrrrXXXXX */
	  {
	    loop
	      {
		i = *reg.offset (rrr! () as isize);
		CCL_WRITE_CHAR! (i);
		if field1 == 0 { break; }
		GET_CCL_CODE! (code, ccl_prog, ic); ic += 1;
		field1 = code >> 8;
		field2 = (code & 0xFF) >> 5;
	      }
	  }

	CCL_WriteExprRegister => /* 1:00000OPERATIONRrrRRR000XXXXX */
	  {
	    field2 = 7;
	    i = *reg.offset (RRR! () as isize);
	    j = *reg.offset (Rrr! () as isize);
	    op = field1 >> 6;
	    jump_address = ic;
	    tail = ccl_goto::ccl_set_expr;
	  }

	CCL_Call =>		/* 1:CCCCCCCCCCCCCCCCCCCCFFFXXXXX */
	  {
	    /* PORTING.md 1.5: C declares `slot` and `prog_id` uninitialised (`slot` is
	       assigned inside the test below).  */
	    let mut slot: Lisp_Object = lisp_h_Qnil ();
	    let mut prog_id: c_int = 0;

	    /* If FFF is nonzero, the CCL program ID is in the
               following code.  */
	    if rrr! () != 0
	      { prog_id = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int; ic += 1; }
	    else
	      { prog_id = field1; }

	    if stack_idx >= 256
		|| prog_id < 0
		|| prog_id >= ASIZE (Vccl_program_table) as c_int
		|| { slot = AREF (Vccl_program_table, prog_id as isize); !VECTORP (slot) }
		|| !VECTORP (AREF (slot, 1))
	      {
		if stack_idx > 0
		  {
		    ccl_prog = ccl_prog_stack_struct[0].ccl_prog;
		    ic = ccl_prog_stack_struct[0].ic;
		    eof_ic = ccl_prog_stack_struct[0].eof_ic;
		  }
		CCL_INVALID_CMD! ();
	      }

	    ccl_prog_stack_struct[stack_idx as usize].ccl_prog = ccl_prog;
	    ccl_prog_stack_struct[stack_idx as usize].ic = ic;
	    ccl_prog_stack_struct[stack_idx as usize].eof_ic = eof_ic;
	    stack_idx += 1;
	    ccl_prog = (*XVECTOR (AREF (slot, 1))).contents.as_mut_ptr ();
	    ic = CCL_HEADER_MAIN;
	    eof_ic = XFIXNAT (*ccl_prog.offset (CCL_HEADER_EOF as isize)) as c_int;
	  }

	CCL_WriteConstString => /* CCCCCCCCCCCCCCCCCCCCrrrXXXXX */
	  {
	    if rrr! () == 0
	      { CCL_WRITE_CHAR! (field1); }
	    else
	      {
		CCL_WRITE_STRING! (field1);
		ic += (field1 + 2) / 3;
	      }
	  }

	CCL_WriteArray =>	/* CCCCCCCCCCCCCCCCCCCCrrrXXXXX */
	  {
	    i = *reg.offset (rrr! () as isize);
	    if 0 <= i && i < field1
	      {
		j = XFIXNUM (*ccl_prog.offset (ic as isize + i as isize)) as c_int;
		CCL_WRITE_CHAR! (j);
	      }
	    ic += field1;
	  }

	CCL_End =>		/* 0000000000000000000000XXXXX */
	  {
	    if stack_idx > 0
	      {
		stack_idx -= 1;
		ccl_prog = ccl_prog_stack_struct[stack_idx as usize].ccl_prog;
		ic = ccl_prog_stack_struct[stack_idx as usize].ic;
		eof_ic = ccl_prog_stack_struct[stack_idx as usize].eof_ic;
		if eof_hit != 0
		  { ic = eof_ic; }
		continue 'ccl_repeat;   /* C's `break` leaves the switch and iterates.  */
	      }
	    if !src.is_null ()
	      { src = src_end; }
	    /* ccl->ic should points to this command code again to
               suppress further processing.  */
	    ic -= 1;
	    CCL_SUCCESS! ();
	  }

	CCL_ExprSelfConst => /* 00000OPERATION000000rrrXXXXX */
	  {
	    i = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int; ic += 1;
	    op = field1 >> 6;
	    tail = ccl_goto::ccl_expr_self;
	  }

	CCL_ExprSelfReg =>	/* 00000OPERATION000RRRrrrXXXXX */
	  {
	    i = *reg.offset (RRR! () as isize);
	    op = field1 >> 6;
	    tail = ccl_goto::ccl_expr_self;
	  }

	CCL_SetExprConst =>	/* 00000OPERATION000RRRrrrXXXXX */
	  {
	    i = *reg.offset (RRR! () as isize);
	    j = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int; ic += 1;
	    op = field1 >> 6;
	    jump_address = ic;
	    tail = ccl_goto::ccl_set_expr;
	  }

	CCL_SetExprReg =>	/* 00000OPERATIONRrrRRRrrrXXXXX */
	  {
	    i = *reg.offset (RRR! () as isize);
	    j = *reg.offset (Rrr! () as isize);
	    op = field1 >> 6;
	    jump_address = ic;
	    tail = ccl_goto::ccl_set_expr;
	  }

	CCL_ReadJumpCondExprConst | CCL_JumpCondExprConst =>
	  /* A--D--D--R--E--S--S-rrrXXXXX */
	  {
	    /* C's `CCL_ReadJumpCondExprConst` reads and `FALLTHROUGH`s into
	       `CCL_JumpCondExprConst`'s statements (PORTING.md 2.2).  */
	    if code & 0x1F == CCL_ReadJumpCondExprConst
	      { CCL_READ_CHAR! (*reg.offset (rrr! () as isize)); }
	    i = *reg.offset (rrr! () as isize);
	    jump_address = ic + ADDR! ();
	    op = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int; ic += 1;
	    j = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int; ic += 1;
	    field2 = 7;
	    tail = ccl_goto::ccl_set_expr;
	  }

	CCL_ReadJumpCondExprReg | CCL_JumpCondExprReg =>
	  /* A--D--D--R--E--S--S-rrrXXXXX */
	  {
	    /* The same entry selection as the two arms above.  */
	    if code & 0x1F == CCL_ReadJumpCondExprReg
	      { CCL_READ_CHAR! (*reg.offset (rrr! () as isize)); }
	    i = *reg.offset (rrr! () as isize);
	    jump_address = ic + ADDR! ();
	    op = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int; ic += 1;
	    GET_CCL_RANGE! (j, ccl_prog, ic, 0, 7); ic += 1;
	    j = *reg.offset (j as isize);
	    field2 = 7;
	    /* C falls through into `ccl_set_expr`.  */
	    tail = ccl_goto::ccl_set_expr;
	  }

	CCL_Extension =>
	  match EXCMD! ()
	    {
	    CCL_ReadMultibyteChar2 =>
	      {
		if src.is_null ()
		  { CCL_INVALID_CMD! (); }
		CCL_READ_CHAR! (i);
		CCL_ENCODE_CHAR! (i, charset_list, *reg.offset (RRR! () as isize),
				  *reg.offset (rrr! () as isize));
	      }

	    CCL_WriteMultibyteChar2 =>
	      {
		if dst.is_null ()
		  { CCL_INVALID_CMD! (); }
		i = CCL_DECODE_CHAR! (*reg.offset (RRR! () as isize),
				      *reg.offset (rrr! () as isize));
		CCL_WRITE_CHAR! (i);
	      }

	    CCL_TranslateCharacter =>
	      {
		i = CCL_DECODE_CHAR! (*reg.offset (RRR! () as isize),
				      *reg.offset (rrr! () as isize));
		op = translate_char (GET_TRANSLATION_TABLE (*reg.offset (Rrr! () as isize)), i);
		CCL_ENCODE_CHAR! (op, charset_list, *reg.offset (RRR! () as isize),
				  *reg.offset (rrr! () as isize));
	      }

	    CCL_TranslateCharacterConstTbl =>
	      {
		/* PORTING.md 1.5: C declares `eop` uninitialised.  */
		let mut eop: ptrdiff_t = 0;
		GET_CCL_RANGE! (eop, ccl_prog, ic, 0,
				(if VECTORP (Vtranslation_table_vector)
				   { ASIZE (Vtranslation_table_vector) }
				 else { -1 }));
		ic += 1;
		i = CCL_DECODE_CHAR! (*reg.offset (RRR! () as isize),
				      *reg.offset (rrr! () as isize));
		op = translate_char (GET_TRANSLATION_TABLE (eop as c_int), i);
		CCL_ENCODE_CHAR! (op, charset_list, *reg.offset (RRR! () as isize),
				  *reg.offset (rrr! () as isize));
	      }

	    CCL_LookupIntConstTbl =>
	      {
		/* PORTING.md 1.5: C declares `eop` and `h` uninitialised.  */
		let mut eop: ptrdiff_t = 0;
		let mut h: *mut Lisp_Hash_Table = std::ptr::null_mut ();
		GET_CCL_RANGE! (eop, ccl_prog, ic, 0,
				(if VECTORP (Vtranslation_hash_table_vector)
				   { ASIZE (Vtranslation_hash_table_vector) }
				 else { -1 }));
		ic += 1;
		h = GET_HASH_TABLE! (eop);

		eop = if FIXNUM_OVERFLOW_P (*reg.offset (RRR! () as isize) as EMACS_INT)
			{ -1 }
		      else
			{ hash_find (h, make_fixnum (*reg.offset (RRR! () as isize)
						     as EMACS_INT)) };
		if eop >= 0
		  {
		    /* PORTING.md 1.5: C declares `opl` and assigns it here.  */
		    let mut opl: Lisp_Object = lisp_h_Qnil ();
		    opl = HASH_VALUE (h, eop as hash_idx_t);
		    if ! (IN_INT_RANGE! (eop) && CHARACTERP (opl))
		      { CCL_INVALID_CMD! (); }
		    *reg.offset (RRR! () as isize) = charset_unicode;
		    *reg.offset (rrr! () as isize) = XFIXNUM (opl) as c_int;
		    *reg.offset (7) = 1; /* r7 true for success */
		  }
		else
		  { *reg.offset (7) = 0; }
	      }

	    CCL_LookupCharConstTbl =>
	      {
		/* PORTING.md 1.5: C declares `eop` and `h` uninitialised.  */
		let mut eop: ptrdiff_t = 0;
		let mut h: *mut Lisp_Hash_Table = std::ptr::null_mut ();
		GET_CCL_RANGE! (eop, ccl_prog, ic, 0,
				(if VECTORP (Vtranslation_hash_table_vector)
				   { ASIZE (Vtranslation_hash_table_vector) }
				 else { -1 }));
		ic += 1;
		i = CCL_DECODE_CHAR! (*reg.offset (RRR! () as isize),
				      *reg.offset (rrr! () as isize));
		h = GET_HASH_TABLE! (eop);

		eop = if FIXNUM_OVERFLOW_P (i as EMACS_INT)
			{ -1 }
		      else
			{ hash_find (h, make_fixnum (i as EMACS_INT)) };
		if eop >= 0
		  {
		    /* PORTING.md 1.5: C declares `opl` and assigns it here.  */
		    let mut opl: Lisp_Object = lisp_h_Qnil ();
		    opl = HASH_VALUE (h, eop as hash_idx_t);
		    if ! (FIXNUMP (opl) && IN_INT_RANGE! (XFIXNUM (opl)))
		      { CCL_INVALID_CMD! (); }
		    *reg.offset (RRR! () as isize) = XFIXNUM (opl) as c_int;
		    *reg.offset (7) = 1; /* r7 true for success */
		  }
		else
		  { *reg.offset (7) = 0; }
	      }

	    CCL_IterateMultipleMap =>
	      {
		/* PORTING.md 1.5: C declares `map`, `content`, `attrib`, `value`, `point`,
		   `size` and `fin_ic` without initialisers.  */
		let mut map: Lisp_Object = lisp_h_Qnil ();
		let mut content: Lisp_Object = lisp_h_Qnil ();
		let mut attrib: Lisp_Object = lisp_h_Qnil ();
		let mut value: Lisp_Object = lisp_h_Qnil ();
		let mut point: EMACS_INT = 0;
		let mut size: ptrdiff_t = 0;
		let mut fin_ic: c_int = 0;

		j = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int;
		ic += 1;			/* number of maps. */
		fin_ic = ic + j;
		op = *reg.offset (rrr! () as isize);
		if j > *reg.offset (RRR! () as isize) && j >= 0
		  {
		    ic += *reg.offset (RRR! () as isize);
		    i = *reg.offset (RRR! () as isize);
		  }
		else
		  {
		    *reg.offset (RRR! () as isize) = -1;
		    ic = fin_ic;
		    /* C's `break` leaves both switches, so the `for (;;)` iterates.  */
		    continue 'ccl_repeat;
		  }

		/* C's `for (;i < j;i++)`: `continue` jumps to the update step, so each
		   `continue` below advances `i` first (PORTING.md 2.2, the dired.rs shape).  */
		while i < j
		  {
		    if !VECTORP (Vcode_conversion_map_vector)
		      { i += 1; continue; }
		    size = ASIZE (Vcode_conversion_map_vector);
		    point = XFIXNUM (*ccl_prog.offset (ic as isize)) as EMACS_INT;
		    ic += 1;
		    if ! (0 <= point && point < size)
		      { i += 1; continue; }
		    map = AREF (Vcode_conversion_map_vector, point as isize);

		    /* Check map validity.  */
		    if !CONSP (map)
		      { i += 1; continue; }
		    map = XCDR (map);
		    if !VECTORP (map)
		      { i += 1; continue; }
		    size = ASIZE (map);
		    if size <= 1
		      { i += 1; continue; }

		    content = AREF (map, 0);

		    /* check map type,
		       [STARTPOINT VAL1 VAL2 ...] or
		       [t ELEMENT STARTPOINT ENDPOINT]  */
		    if FIXNUMP (content)
		      {
			point = XFIXNUM (content) as EMACS_INT;
			if !((point as isize) <= op as isize
			     && (op as isize) - point + 1 < size)
			  { i += 1; continue; }
			content = AREF (map, (op as isize) - point + 1);
		      }
		    else if EQ (content, Qt ())
		      {
			if size != 4
			  { i += 1; continue; }
			if FIXNUMP (AREF (map, 2))
			    && XFIXNUM (AREF (map, 2)) <= op as EMACS_INT
			    && FIXNUMP (AREF (map, 3))
			    && (op as EMACS_INT) < XFIXNUM (AREF (map, 3))
			  { content = AREF (map, 1); }
			else
			  { i += 1; continue; }
		      }
		    else
		      { i += 1; continue; }

		    if NILP (content)
		      { i += 1; continue; }
		    else if FIXNUMP (content) && IN_INT_RANGE! (XFIXNUM (content))
		      {
			*reg.offset (RRR! () as isize) = i;
			*reg.offset (rrr! () as isize) = XFIXNUM (content) as c_int;
			break;
		      }
		    else if EQ (content, Qt ()) || EQ (content, Qlambda ())
		      {
			*reg.offset (RRR! () as isize) = i;
			break;
		      }
		    else if CONSP (content)
		      {
			attrib = XCAR (content);
			value = XCDR (content);
			if ! (FIXNUMP (attrib) && FIXNUMP (value)
			      && IN_INT_RANGE! (XFIXNUM (value)))
			  { i += 1; continue; }
			*reg.offset (RRR! () as isize) = i;
			*reg.offset (rrr! () as isize) = XFIXNUM (value) as c_int;
			break;
		      }
		    else if SYMBOLP (content)
		      { CCL_CALL_FOR_MAP_INSTRUCTION! (content, fin_ic); }
		    else
		      { CCL_INVALID_CMD! (); }
		    i += 1;
		  }
		if i == j
		  { *reg.offset (RRR! () as isize) = -1; }
		ic = fin_ic;
	      }

	    CCL_MapMultiple =>
	      {
		/* PORTING.md 1.5: C leaves `map`, `content`, `attrib`, `value`, `point`,
		   `size`, `map_vector_size`, `map_set_rest_length` and `fin_ic` without
		   initialisers.  `current_ic` is C's own initialiser.  */
		let mut map: Lisp_Object = lisp_h_Qnil ();
		let mut content: Lisp_Object = lisp_h_Qnil ();
		let mut attrib: Lisp_Object = lisp_h_Qnil ();
		let mut value: Lisp_Object = lisp_h_Qnil ();
		let mut point: EMACS_INT = 0;
		let mut size: ptrdiff_t = 0;
		let mut map_vector_size: ptrdiff_t = 0;
		let mut map_set_rest_length: c_int = 0;
		let mut fin_ic: c_int = 0;
		let mut current_ic: c_int = this_ic;

		/* inhibit recursive call on MapMultiple. */
		if stack_idx_of_map_multiple > 0
		  {
		    if stack_idx_of_map_multiple <= stack_idx
		      {
			stack_idx_of_map_multiple = 0;
			mapping_stack_pointer = mapping_stack.as_mut_ptr ();
			CCL_INVALID_CMD! ();
		      }
		  }
		else
		  { mapping_stack_pointer = mapping_stack.as_mut_ptr (); }
		stack_idx_of_map_multiple = 0;

		/* Get number of maps and separators.  */
		map_set_rest_length = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int;
		ic += 1;

		fin_ic = ic + map_set_rest_length;
		op = *reg.offset (rrr! () as isize);

		if map_set_rest_length > *reg.offset (RRR! () as isize)
		    && *reg.offset (RRR! () as isize) >= 0
		  {
		    ic += *reg.offset (RRR! () as isize);
		    i = *reg.offset (RRR! () as isize);
		    map_set_rest_length -= i;
		  }
		else
		  {
		    ic = fin_ic;
		    *reg.offset (RRR! () as isize) = -1;
		    mapping_stack_pointer = mapping_stack.as_mut_ptr ();
		    /* C's `break` leaves both switches, so the `for (;;)` iterates.  */
		    continue 'ccl_repeat;
		  }

		if mapping_stack_pointer <= mapping_stack.as_mut_ptr ().offset (1)
		  {
		    /* Set up initial state. */
		    mapping_stack_pointer = mapping_stack.as_mut_ptr ();
		    PUSH_MAPPING_STACK! (0, op);
		    *reg.offset (RRR! () as isize) = -1;
		  }
		else
		  {
		    /* Recover after calling other ccl program. */
		    /* PORTING.md 1.5: C declares `orig_op` uninitialised.  */
		    let mut orig_op: c_int = 0;

		    POP_MAPPING_STACK! (map_set_rest_length, orig_op);
		    POP_MAPPING_STACK! (map_set_rest_length, *reg.offset (rrr! () as isize));
		    match op
		      {
		      -1 =>
			{
			  /* Regard it as Qnil. */
			  op = orig_op;
			  i += 1;
			  ic += 1;
			  map_set_rest_length -= 1;
			}
		      -2 =>
			{
			  /* Regard it as Qt. */
			  op = *reg.offset (rrr! () as isize);
			  i += 1;
			  ic += 1;
			  map_set_rest_length -= 1;
			}
		      -3 =>
			{
			  /* Regard it as Qlambda. */
			  op = orig_op;
			  i += map_set_rest_length;
			  ic += map_set_rest_length;
			  map_set_rest_length = 0;
			}
		      _ =>
			{
			  /* Regard it as normal mapping. */
			  i += map_set_rest_length;
			  ic += map_set_rest_length;
			  POP_MAPPING_STACK! (map_set_rest_length,
					      *reg.offset (rrr! () as isize));
			}
		      }
		  }
		if !VECTORP (Vcode_conversion_map_vector)
		  { CCL_INVALID_CMD! (); }
		map_vector_size = ASIZE (Vcode_conversion_map_vector);

		/* C's `do { … } while (1)`; the inner `for (;map_set_rest_length > 0;
		   i++, ic++, map_set_rest_length--)` keeps C's three update steps, which
		   `continue` also runs, so each `continue` below advances all three first.  */
		loop
		  {
		    while map_set_rest_length > 0
		      {
			point = XFIXNUM (*ccl_prog.offset (ic as isize)) as EMACS_INT;
			if point < 0
			  {
			    /* +1 is for including separator. */
			    point = -point + 1;
			    if mapping_stack_pointer
				>= mapping_stack.as_mut_ptr ()
				   .offset (MAX_MAP_SET_LEVEL as isize)
			      { CCL_INVALID_CMD! (); }
			    PUSH_MAPPING_STACK! ((map_set_rest_length as EMACS_INT
						  - point) as c_int,
						 *reg.offset (rrr! () as isize));
			    map_set_rest_length = point as c_int;
			    *reg.offset (rrr! () as isize) = op;
			    i += 1; ic += 1; map_set_rest_length -= 1;
			    continue;
			  }

			if point >= map_vector_size
			  { i += 1; ic += 1; map_set_rest_length -= 1; continue; }
			map = AREF (Vcode_conversion_map_vector, point as isize);

			/* Check map validity.  */
			if !CONSP (map)
			  { i += 1; ic += 1; map_set_rest_length -= 1; continue; }
			map = XCDR (map);
			if !VECTORP (map)
			  { i += 1; ic += 1; map_set_rest_length -= 1; continue; }
			size = ASIZE (map);
			if size <= 1
			  { i += 1; ic += 1; map_set_rest_length -= 1; continue; }

			content = AREF (map, 0);

			/* check map type,
			   [STARTPOINT VAL1 VAL2 ...] or
			   [t ELEMENT STARTPOINT ENDPOINT]  */
			if FIXNUMP (content)
			  {
			    point = XFIXNUM (content) as EMACS_INT;
			    if !((point as isize) <= op as isize
				 && (op as isize) - point + 1 < size)
			      { i += 1; ic += 1; map_set_rest_length -= 1; continue; }
			    content = AREF (map, (op as isize) - point + 1);
			  }
			else if EQ (content, Qt ())
			  {
			    if size != 4
			      { i += 1; ic += 1; map_set_rest_length -= 1; continue; }
			    if FIXNUMP (AREF (map, 2))
				&& XFIXNUM (AREF (map, 2)) <= op as EMACS_INT
				&& FIXNUMP (AREF (map, 3))
				&& (op as EMACS_INT) < XFIXNUM (AREF (map, 3))
			      { content = AREF (map, 1); }
			    else
			      { i += 1; ic += 1; map_set_rest_length -= 1; continue; }
			  }
			else
			  { i += 1; ic += 1; map_set_rest_length -= 1; continue; }

			if NILP (content)
			  { i += 1; ic += 1; map_set_rest_length -= 1; continue; }

			*reg.offset (RRR! () as isize) = i;
			if FIXNUMP (content) && IN_INT_RANGE! (XFIXNUM (content))
			  {
			    op = XFIXNUM (content) as c_int;
			    i += map_set_rest_length - 1;
			    ic += map_set_rest_length - 1;
			    POP_MAPPING_STACK! (map_set_rest_length,
						*reg.offset (rrr! () as isize));
			    map_set_rest_length += 1;
			  }
			else if CONSP (content)
			  {
			    attrib = XCAR (content);
			    value = XCDR (content);
			    if ! (FIXNUMP (attrib) && FIXNUMP (value)
				  && IN_INT_RANGE! (XFIXNUM (value)))
			      { i += 1; ic += 1; map_set_rest_length -= 1; continue; }
			    op = XFIXNUM (value) as c_int;
			    i += map_set_rest_length - 1;
			    ic += map_set_rest_length - 1;
			    POP_MAPPING_STACK! (map_set_rest_length,
						*reg.offset (rrr! () as isize));
			    map_set_rest_length += 1;
			  }
			else if EQ (content, Qt ())
			  {
			    op = *reg.offset (rrr! () as isize);
			  }
			else if EQ (content, Qlambda ())
			  {
			    i += map_set_rest_length;
			    ic += map_set_rest_length;
			    break;
			  }
			else if SYMBOLP (content)
			  {
			    if mapping_stack_pointer
				>= mapping_stack.as_mut_ptr ()
				   .offset (MAX_MAP_SET_LEVEL as isize)
			      { CCL_INVALID_CMD! (); }
			    PUSH_MAPPING_STACK! (map_set_rest_length,
						 *reg.offset (rrr! () as isize));
			    PUSH_MAPPING_STACK! (map_set_rest_length, op);
			    stack_idx_of_map_multiple = stack_idx + 1;
			    CCL_CALL_FOR_MAP_INSTRUCTION! (content, current_ic);
			  }
			else
			  { CCL_INVALID_CMD! (); }
			i += 1; ic += 1; map_set_rest_length -= 1;
		      }
		    if mapping_stack_pointer <= mapping_stack.as_mut_ptr ().offset (1)
		      { break; }
		    POP_MAPPING_STACK! (map_set_rest_length, *reg.offset (rrr! () as isize));
		    i += map_set_rest_length;
		    ic += map_set_rest_length;
		    POP_MAPPING_STACK! (map_set_rest_length, *reg.offset (rrr! () as isize));
		  }

		ic = fin_ic;
	    /* C has `reg[rrr] = op;` after the case's inner block, still inside the case.  */
	    *reg.offset (rrr! () as isize) = op;
	  }

	    CCL_MapSingle =>
	      {
		/* PORTING.md 1.5: C leaves `map`, `attrib`, `value`, `content` and `point`
		   without initialisers.  */
		let mut map: Lisp_Object = lisp_h_Qnil ();
		let mut attrib: Lisp_Object = lisp_h_Qnil ();
		let mut value: Lisp_Object = lisp_h_Qnil ();
		let mut content: Lisp_Object = lisp_h_Qnil ();
		let mut point: c_int = 0;
		j = XFIXNUM (*ccl_prog.offset (ic as isize)) as c_int;
		ic += 1;			/* map_id */
		op = *reg.offset (rrr! () as isize);
		if ! (VECTORP (Vcode_conversion_map_vector)
		      && j < ASIZE (Vcode_conversion_map_vector) as c_int)
		  {
		    *reg.offset (RRR! () as isize) = -1;
		    /* C's `break` leaves both switches, so the `for (;;)` iterates.  */
		    continue 'ccl_repeat;
		  }
		map = AREF (Vcode_conversion_map_vector, j as isize);
		if !CONSP (map)
		  {
		    *reg.offset (RRR! () as isize) = -1;
		    continue 'ccl_repeat;
		  }
		map = XCDR (map);
		if ! (VECTORP (map)
		      && 0 < ASIZE (map)
		      && FIXNUMP (AREF (map, 0))
		      && XFIXNUM (AREF (map, 0)) <= op as EMACS_INT
		      && (op as EMACS_INT) - XFIXNUM (AREF (map, 0)) + 1 < ASIZE (map))
		  {
		    *reg.offset (RRR! () as isize) = -1;
		    continue 'ccl_repeat;
		  }
		point = ((op as EMACS_INT) - XFIXNUM (AREF (map, 0)) + 1) as c_int;
		*reg.offset (RRR! () as isize) = 0;
		content = AREF (map, point as isize);
		if NILP (content)
		  { *reg.offset (RRR! () as isize) = -1; }
		else if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, content,
					     INT_MAX as EMACS_INT)
		  { *reg.offset (rrr! () as isize) = XFIXNUM (content) as c_int; }
		else if EQ (content, Qt ())
		  { }
		else if CONSP (content)
		  {
		    attrib = XCAR (content);
		    value = XCDR (content);
		    if !FIXNUMP (attrib)
			|| !TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, value,
						 INT_MAX as EMACS_INT)
		      /* C's `continue` goes on with the `for (;;)`.  */
		      { continue 'ccl_repeat; }
		    *reg.offset (rrr! () as isize) = XFIXNUM (value) as c_int;
		    /* C's `break` leaves both switches, so the `for (;;)` iterates.  */
		    continue 'ccl_repeat;
		  }
		else if SYMBOLP (content)
		  { CCL_CALL_FOR_MAP_INSTRUCTION! (content, ic); }
		else
		  { *reg.offset (RRR! () as isize) = -1; }
	      }

	    _ =>
	      { CCL_INVALID_CMD! (); }
	    }

	/* C 1782-1783's outer `switch (code & 0x1F)` default.  */
	_ =>
	  { CCL_INVALID_CMD! (); }
	}

      /* ccl_expr_self: C's label inside the switch (PORTING.md 2.2).  */
      if tail == ccl_goto::ccl_expr_self
	{
	  match op
	    {
	    CCL_PLUS =>
	      {
		let (sum, _) = (*reg.offset (rrr! () as isize)).overflowing_add (i);
		*reg.offset (rrr! () as isize) = sum;
	      }
	    CCL_MINUS =>
	      {
		let (diff, _) = (*reg.offset (rrr! () as isize)).overflowing_sub (i);
		*reg.offset (rrr! () as isize) = diff;
	      }
	    CCL_MUL =>
	      {
		let (prod, _) = (*reg.offset (rrr! () as isize)).overflowing_mul (i);
		*reg.offset (rrr! () as isize) = prod;
	      }
	    CCL_DIV =>
	      {
		if i == 0
		  { CCL_INVALID_CMD! (); }
		if ! INT_DIVIDE_OVERFLOW! (*reg.offset (rrr! () as isize), i)
		  { *reg.offset (rrr! () as isize) /= i; }
	      }
	    CCL_MOD =>
	      {
		if i == 0
		  { CCL_INVALID_CMD! (); }
		*reg.offset (rrr! () as isize) =
		  if i == -1 { 0 } else { *reg.offset (rrr! () as isize) % i };
	      }
	    CCL_AND =>
	      { *reg.offset (rrr! () as isize) &= i; }
	    CCL_OR =>
	      { *reg.offset (rrr! () as isize) |= i; }
	    CCL_XOR =>
	      { *reg.offset (rrr! () as isize) ^= i; }
	    CCL_LSH =>
	      {
		if i < 0
		  { CCL_INVALID_CMD! (); }
		*reg.offset (rrr! () as isize) =
		  if i < UINT_WIDTH
		    { ((*reg.offset (rrr! () as isize) as c_uint) << i) as c_int }
		  else { 0 };
	      }
	    CCL_RSH =>
	      {
		if i < 0
		  { CCL_INVALID_CMD! (); }
		*reg.offset (rrr! () as isize) =
		  *reg.offset (rrr! () as isize) >> min (i as isize, INT_WIDTH as isize - 1);
	      }
	    CCL_LSH8 =>
	      {
		*reg.offset (rrr! () as isize) =
		  ((*reg.offset (rrr! () as isize) as c_uint) << 8) as c_int;
		*reg.offset (rrr! () as isize) |= i;
	      }
	    CCL_RSH8 =>
	      {
		*reg.offset (7) = *reg.offset (rrr! () as isize) & 0xFF;
		*reg.offset (rrr! () as isize) >>= 8;
	      }
	    CCL_DIVMOD =>
	      {
		if i == 0
		  { CCL_INVALID_CMD! (); }
		if i == -1
		  {
		    *reg.offset (7) = 0;
		    /* C's `ckd_sub (&reg[rrr], 0, reg[rrr])`: the port spells the call
		       with the receiver first, so C's `0` operand stays where the family
		       check requires it (docs/ports/token-aliases.tsv's `ckd_sub`).  */
		    let (diff, _) =
		      c_int::overflowing_sub (0, *reg.offset (rrr! () as isize));
		    *reg.offset (rrr! () as isize) = diff;
		  }
		else
		  {
		    *reg.offset (7) = *reg.offset (rrr! () as isize) % i;
		    *reg.offset (rrr! () as isize) /= i;
		  }
	      }
	    CCL_LS =>
	      { *reg.offset (rrr! () as isize) =
		  (*reg.offset (rrr! () as isize) < i) as c_int; }
	    CCL_GT =>
	      { *reg.offset (rrr! () as isize) =
		  (*reg.offset (rrr! () as isize) > i) as c_int; }
	    CCL_EQ =>
	      { *reg.offset (rrr! () as isize) =
		  (*reg.offset (rrr! () as isize) == i) as c_int; }
	    CCL_LE =>
	      { *reg.offset (rrr! () as isize) =
		  (*reg.offset (rrr! () as isize) <= i) as c_int; }
	    CCL_GE =>
	      { *reg.offset (rrr! () as isize) =
		  (*reg.offset (rrr! () as isize) >= i) as c_int; }
	    CCL_NE =>
	      { *reg.offset (rrr! () as isize) =
		  (*reg.offset (rrr! () as isize) != i) as c_int; }
	    _ =>
	      { CCL_INVALID_CMD! (); }
	    }
	}

      /* ccl_set_expr: C's other label inside the switch.  */
      if tail == ccl_goto::ccl_set_expr
	{
	  match op
	    {
	    CCL_PLUS =>
	      {
		let (sum, _) = i.overflowing_add (j);
		*reg.offset (rrr! () as isize) = sum;
	      }
	    CCL_MINUS =>
	      {
		let (diff, _) = i.overflowing_sub (j);
		*reg.offset (rrr! () as isize) = diff;
	      }
	    CCL_MUL =>
	      {
		let (prod, _) = i.overflowing_mul (j);
		*reg.offset (rrr! () as isize) = prod;
	      }
	    CCL_DIV =>
	      {
		if j == 0
		  { CCL_INVALID_CMD! (); }
		if ! INT_DIVIDE_OVERFLOW! (i, j)
		  { i /= j; }
		*reg.offset (rrr! () as isize) = i;
	      }
	    CCL_MOD =>
	      {
		if j == 0
		  { CCL_INVALID_CMD! (); }
		*reg.offset (rrr! () as isize) = if j == -1 { 0 } else { i % j };
	      }
	    CCL_AND =>
	      { *reg.offset (rrr! () as isize) = i & j; }
	    CCL_OR =>
	      { *reg.offset (rrr! () as isize) = i | j; }
	    CCL_XOR =>
	      { *reg.offset (rrr! () as isize) = i ^ j; }
	    CCL_LSH =>
	      {
		if j < 0
		  { CCL_INVALID_CMD! (); }
		*reg.offset (rrr! () as isize) =
		  if j < UINT_WIDTH { ((i as c_uint) << j) as c_int } else { 0 };
	      }
	    CCL_RSH =>
	      {
		if j < 0
		  { CCL_INVALID_CMD! (); }
		*reg.offset (rrr! () as isize) =
		  i >> min (j as isize, INT_WIDTH as isize - 1);
	      }
	    CCL_LSH8 =>
	      {
		*reg.offset (rrr! () as isize) = (((i as c_uint) << 8) | j as c_uint) as c_int;
	      }
	    CCL_RSH8 =>
	      {
		*reg.offset (rrr! () as isize) = i >> 8;
		*reg.offset (7) = i & 0xFF;
	      }
	    CCL_DIVMOD =>
	      {
		if j == 0
		  { CCL_INVALID_CMD! (); }
		if j == -1
		  {
		    /* C's `ckd_sub (&reg[rrr], 0, reg[rrr])` (see the ccl_expr_self arm).  */
		    let (diff, _) =
		      c_int::overflowing_sub (0, *reg.offset (rrr! () as isize));
		    *reg.offset (rrr! () as isize) = diff;
		    *reg.offset (7) = 0;
		  }
		else
		  {
		    *reg.offset (rrr! () as isize) = i / j;
		    *reg.offset (7) = i % j;
		  }
	      }
	    CCL_LS =>
	      { *reg.offset (rrr! () as isize) = (i < j) as c_int; }
	    CCL_GT =>
	      { *reg.offset (rrr! () as isize) = (i > j) as c_int; }
	    CCL_EQ =>
	      { *reg.offset (rrr! () as isize) = (i == j) as c_int; }
	    CCL_LE =>
	      { *reg.offset (rrr! () as isize) = (i <= j) as c_int; }
	    CCL_GE =>
	      { *reg.offset (rrr! () as isize) = (i >= j) as c_int; }
	    CCL_NE =>
	      { *reg.offset (rrr! () as isize) = (i != j) as c_int; }
	    CCL_DECODE_SJIS =>
	      {
		i = (((i as c_uint) << 8) | j as c_uint) as c_int;
		i = SJIS_TO_JIS (i);
		*reg.offset (rrr! () as isize) = i >> 8;
		*reg.offset (7) = i & 0xFF;
	      }
	    CCL_ENCODE_SJIS =>
	      {
		i = (((i as c_uint) << 8) | j as c_uint) as c_int;
		i = JIS_TO_SJIS (i);
		*reg.offset (rrr! () as isize) = i >> 8;
		*reg.offset (7) = i & 0xFF;
	      }
	    _ =>
	      { CCL_INVALID_CMD! (); }
	    }
	  code &= 0x1F;
	  if code == CCL_WriteExprConst || code == CCL_WriteExprRegister
	    {
	      i = *reg.offset (rrr! () as isize);
	      CCL_WRITE_CHAR! (i);
	      ic = jump_address;
	    }
	  else if *reg.offset (rrr! () as isize) == 0
	    { ic = jump_address; }
	}

      /* C's `for (;;)` iterates here.  */
      continue 'ccl_repeat;
    };

  /* ccl_error_handler: C's forward `goto` target for `CCL_INVALID_CMD` and for the
     loop's own `break`.  The `CCL_DEBUG` arm's `goto ccl_finish` leaves the block.  */
  'ccl_error_handler: {
  if !destination.is_null ()
    {
      /* We can insert an error message only if DESTINATION is
         specified and we still have a room to store the message
         there.  */
      /* PORTING.md 1.5: C declares `msg` and `msglen` uninitialised.  */
      let mut msg: [c_char; 256] = [0; 256];
      let mut msglen: c_int = 0;

      if dst.is_null ()
	{ dst = destination; }

      match (*ccl).status
	{
	CCL_STAT_INVALID_CMD =>
	  {
	    msglen = sprintf (msg.as_mut_ptr (),
			      c"\nCCL: Invalid command %x (ccl_code = %x) at %d.".as_ptr (),
			      code as c_uint & 0x1F, code as c_uint, this_ic);
	    /* C 1805-1829's `#ifdef CCL_DEBUG` arm, carried behind the feature
	       (P-51/status 3.49).  C's own text names `dst_bytes`, which ccl.c never
	       declares (it is a leftover from the function's older signature), so the
	       carried text is a faithful copy of a dead arm that C itself cannot compile;
	       no configuration in this port compiles it either — `CCL_DEBUG` is a
	       source-level macro, not a `config.h` key, so it is not in `xtask dead-arms`.
	       `docs/ports/ccl.md` records the decision.  */
	    #[cfg (feature = "ccl_debug")]
	    {
	      let mut i: c_int = ccl_backtrace_idx - 1;
	      let mut j: c_int = 0;

	      if dst.offset (msglen as isize) <= (if dst_bytes != 0 { dst_end } else { src })
		{
		  memcpy (dst as *mut c_void, msg.as_ptr () as *const c_void, msglen as usize);
		  dst = dst.offset (msglen as isize);
		}

	      j = 0;
	      while j < CCL_DEBUG_BACKTRACE_LEN
		{
		  if i < 0 { i = CCL_DEBUG_BACKTRACE_LEN - 1; }
		  if ccl_backtrace_table[i as usize] == 0
		    { break; }
		  msglen = sprintf (msg.as_mut_ptr (), c" %d".as_ptr (),
				    ccl_backtrace_table[i as usize]);
		  if dst.offset (msglen as isize) > (if dst_bytes != 0 { dst_end } else { src })
		    { break; }
		  memcpy (dst as *mut c_void, msg.as_ptr () as *const c_void, msglen as usize);
		  dst = dst.offset (msglen as isize);
		  j += 1;
		  i -= 1;
		}
	      break 'ccl_error_handler;
	    }
	  }

	CCL_STAT_QUIT =>
	  {
	    msglen = if (*ccl).bits & CCL_PROGRAM_QUIT_SILENTLY != 0
		       { 0 }
		     else
		       { sprintf (msg.as_mut_ptr (), c"\nCCL: Quitted.".as_ptr ()) };
	  }

	_ =>
	  {
	    msglen = sprintf (msg.as_mut_ptr (), c"\nCCL: Unknown error type (%d)".as_ptr (),
			      (*ccl).status);
	  }
	}

      if msglen <= dst_end.offset_from (dst) as c_int
	{
	  i = 0;
	  while i < msglen
	    {
	      *dst = msg[i as usize] as c_int;
	      dst = dst.offset (1);
	      i += 1;
	    }
	}

      if (*ccl).status == CCL_STAT_INVALID_CMD
	{
	  /* C 1848-1857's `#if 0` arm (the "copy the remaining source data" text that
	     names the non-existent `dst_bytes`) is dead for the tool as well and is
	     recorded in docs/ports/ccl.md instead of transcribed.  */
	  /* Signal that we've consumed everything.  */
	  src = src_end;
	}
    }
  }

  /* ccl_finish: */
  (*ccl).ic = ic;
  (*ccl).stack_idx = stack_idx;
  (*ccl).prog = ccl_prog;
  (*ccl).consumed = src.offset_from (source) as c_int;
  if !dst.is_null ()
    { (*ccl).produced = dst.offset_from (destination) as c_int; }
  else
    { (*ccl).produced = 0; }
}

/* Resolve symbols in the specified CCL code (Lisp vector).  This
   function converts symbols of code conversion maps and character
   translation tables embedded in the CCL code into their ID numbers.

   The return value is a new vector in which all symbols are resolved,
   Qt if resolving of some symbol failed,
   or nil if CCL contains invalid data.  */

unsafe fn resolve_symbol_ccl_program (ccl: Lisp_Object) -> Lisp_Object
{
  /* PORTING.md 1.5: C declares `i`, `veclen` and the three Lisp_Objects without
     initialisers (`unresolved` is C's own initialiser).  */
  let mut i: c_int = 0;
  let mut veclen: c_int = 0;
  let mut unresolved: c_int = 0;
  let mut result: Lisp_Object = lisp_h_Qnil ();
  let mut contents: Lisp_Object = lisp_h_Qnil ();
  let mut val: Lisp_Object = lisp_h_Qnil ();

  if ! ((CCL_HEADER_MAIN as isize) < ASIZE (ccl) && ASIZE (ccl) <= INT_MAX as isize)
    { return Qnil (); }
  result = Fcopy_sequence (ccl);
  veclen = ASIZE (result) as c_int;

  i = 0;
  while i < veclen
    {
      contents = AREF (result, i as isize);
      if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, contents, INT_MAX as EMACS_INT)
	{ i += 1; continue; }
      else if CONSP (contents)
	      && SYMBOLP (XCAR (contents))
	      && SYMBOLP (XCDR (contents))
	{
	  /* This is the new style for embedding symbols.  The form is
	     (SYMBOL . PROPERTY).  (get SYMBOL PROPERTY) should give
	     an index number.  */
	  val = Fget (XCAR (contents), XCDR (contents));
	  if RANGED_FIXNUMP (0, val, INT_MAX as EMACS_INT)
	    { ASET (result, i as isize, val); }
	  else
	    { unresolved = 1; }
	  i += 1; continue;
	}
      else if SYMBOLP (contents)
	{
	  /* This is the old style for embedding symbols.  This style
             may lead to a bug if, for instance, a translation table
             and a code conversion map have the same name.  */
	  val = Fget (contents, Qtranslation_table_id ());
	  if RANGED_FIXNUMP (0, val, INT_MAX as EMACS_INT)
	    { ASET (result, i as isize, val); }
	  else
	    {
	      val = Fget (contents, Qcode_conversion_map_id ());
	      if RANGED_FIXNUMP (0, val, INT_MAX as EMACS_INT)
		{ ASET (result, i as isize, val); }
	      else
		{
		  val = Fget (contents, Qccl_program_idx ());
		  if RANGED_FIXNUMP (0, val, INT_MAX as EMACS_INT)
		    { ASET (result, i as isize, val); }
		  else
		    { unresolved = 1; }
		}
	    }
	  i += 1; continue;
	}
      return Qnil ();
    }

  if ! (0 <= XFIXNUM (AREF (result, CCL_HEADER_BUF_MAG as isize))
	&& ASCENDING_ORDER! (0, XFIXNUM (AREF (result, CCL_HEADER_EOF as isize)),
			     ASIZE (ccl)))
    { return Qnil (); }

  return if unresolved != 0 { Qt () } else { result };
}

/* Return the compiled code (vector) of CCL program CCL_PROG.
   CCL_PROG is a name (symbol) of the program or already compiled
   code.  If necessary, resolve symbols in the compiled code to index
   numbers.  If we failed to get the compiled code or to resolve
   symbols, return Qnil.  */

unsafe fn ccl_get_compiled_code (ccl_prog: Lisp_Object, idx: *mut ptrdiff_t) -> Lisp_Object
{
  /* PORTING.md 1.5: C declares `val` and `slot` without initialisers.  */
  let mut val: Lisp_Object = lisp_h_Qnil ();
  let mut slot: Lisp_Object = lisp_h_Qnil ();

  if VECTORP (ccl_prog)
    {
      val = resolve_symbol_ccl_program (ccl_prog);
      *idx = -1;
      return if VECTORP (val) { val } else { Qnil () };
    }
  if !SYMBOLP (ccl_prog)
    { return Qnil (); }

  val = Fget (ccl_prog, Qccl_program_idx ());
  if ! FIXNATP (val)
      || XFIXNUM (val) >= ASIZE (Vccl_program_table)
    { return Qnil (); }
  slot = AREF (Vccl_program_table, XFIXNUM (val) as isize);
  if ! VECTORP (slot)
      || ASIZE (slot) != 4
      || ! VECTORP (AREF (slot, 1))
    { return Qnil (); }
  *idx = XFIXNUM (val) as ptrdiff_t;
  if NILP (AREF (slot, 2))
    {
      val = resolve_symbol_ccl_program (AREF (slot, 1));
      if ! VECTORP (val)
	{ return Qnil (); }
      ASET (slot, 1, val);
      ASET (slot, 2, Qt ());
    }
  return AREF (slot, 1);
}

/* Setup fields of the structure pointed by CCL appropriately for the
   execution of CCL program CCL_PROG.  CCL_PROG is the name (symbol)
   of the CCL program or the already compiled code (vector).
   Return true if successful.

   If CCL_PROG is nil, just reset the structure pointed by CCL.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_ccl_program (ccl: *mut ccl_program,
					    mut ccl_prog: Lisp_Object) -> bool
{
  if ! NILP (ccl_prog)
    {
      /* PORTING.md 1.5: C declares `vp` without an initialiser.  */
      let mut vp: *mut Lisp_Vector = std::ptr::null_mut ();

      ccl_prog = ccl_get_compiled_code (ccl_prog, &raw mut (*ccl).idx);
      if ! VECTORP (ccl_prog)
	{ return false; }
      vp = XVECTOR (ccl_prog);
      (*ccl).size = (*vp).header.size as c_int;
      (*ccl).prog = (*vp).contents.as_mut_ptr ();
      (*ccl).eof_ic = XFIXNUM (*(*vp).contents.as_mut_ptr ()
			       .offset (CCL_HEADER_EOF as isize)) as c_int;
      (*ccl).buf_magnification = XFIXNUM (*(*vp).contents.as_mut_ptr ()
					  .offset (CCL_HEADER_BUF_MAG as isize)) as c_int;
      if (*ccl).idx >= 0
	{
	  /* PORTING.md 1.5: C declares `slot` and assigns it here.  */
	  let mut slot: Lisp_Object = lisp_h_Qnil ();

	  slot = AREF (Vccl_program_table, (*ccl).idx);
	  ASET (slot, 3, Qnil ());
	}
    }
  (*ccl).ic = CCL_HEADER_MAIN;
  memset ((*ccl).reg.as_mut_ptr () as *mut c_void, 0, std::mem::size_of::<[c_int; 8]> ());
  /* C's `ccl->last_block = false` and `ccl->quit_silently = false` are the mirror's
     `bits` byte (the two families of docs/ports/token-aliases.tsv).  */
  (*ccl).bits &= ! CCL_PROGRAM_LAST_BLOCK;
  (*ccl).status = 0;
  (*ccl).stack_idx = 0;
  (*ccl).bits &= ! CCL_PROGRAM_QUIT_SILENTLY;
  return true;
}


DEFUN! ("ccl-program-p", Fccl_program_p, Sccl_program_p, 1, 1, 0,
       doc: "Return t if OBJECT is a CCL program name or a compiled CCL program code.\nSee the documentation of `define-ccl-program' for the detail of CCL program.",
       (object: Lisp_Object),
       {
  /* PORTING.md 1.5: C declares `val` without an initialiser.  */
  let mut val: Lisp_Object = lisp_h_Qnil ();

  if VECTORP (object)
    {
      val = resolve_symbol_ccl_program (object);
      return if VECTORP (val) { Qt () } else { Qnil () };
    }
  if !SYMBOLP (object)
    { return Qnil (); }

  val = Fget (object, Qccl_program_idx ());
  return if ! FIXNATP (val)
	    || XFIXNUM (val) >= ASIZE (Vccl_program_table)
	   { Qnil () } else { Qt () };
});

DEFUN! ("ccl-execute", Fccl_execute, Sccl_execute, 2, 2, 0,
       doc: "Execute CCL-PROGRAM with registers initialized by REGISTERS.\n\nCCL-PROGRAM is a CCL program name (symbol)\nor compiled code generated by `ccl-compile' (for backward compatibility.\nIn the latter case, the execution overhead is bigger than in the former).\nNo I/O commands should appear in CCL-PROGRAM.\n\nREGISTERS is a vector of [R0 R1 ... R7] where RN is an initial value\nfor the Nth register.\n\nAs side effect, each element of REGISTERS holds the value of\nthe corresponding register after the execution.\n\nSee the documentation of `define-ccl-program' for a definition of CCL\nprograms.",
       (ccl_prog: Lisp_Object, reg: Lisp_Object),
       {
  /* PORTING.md 1.5: C declares `ccl` and `i` without initialisers.  */
  let mut ccl: ccl_program = std::mem::zeroed ();
  let mut i: c_int = 0;

  if ! setup_ccl_program (&raw mut ccl, ccl_prog)
    { error (c"Invalid CCL program".as_ptr ()); }

  CHECK_VECTOR (reg);
  if ASIZE (reg) != 8
    { error (c"Length of vector REGISTERS is not 8".as_ptr ()); }

  i = 0;
  while i < 8
    {
      /* PORTING.md 1.5: C declares `n` and lets `integer_to_intmax` store it.  */
      let mut n: intmax_t = 0;
      ccl.reg[i as usize] =
	if INTEGERP (AREF (reg, i as isize))
	  && integer_to_intmax (AREF (reg, i as isize), &raw mut n)
	  && INT_MIN as intmax_t <= n && n <= INT_MAX as intmax_t
	  { n as c_int }
	else
	  { 0 };
      i += 1;
    }

  ccl_driver (&raw mut ccl, std::ptr::null_mut (), std::ptr::null_mut (), 0, 0, Qnil ());
  maybe_quit ();
  if ccl.status != CCL_STAT_SUCCESS
    { error (c"Error in CCL program at %dth code".as_ptr (), ccl.ic); }

  i = 0;
  while i < 8
    {
      ASET (reg, i as isize, make_int (ccl.reg[i as usize] as intmax_t));
      i += 1;
    }
  return Qnil ();
});

DEFUN! ("ccl-execute-on-string", Fccl_execute_on_string, Sccl_execute_on_string,
       3, 5, 0,
       doc: "Execute CCL-PROGRAM with initial STATUS on STRING.\n\nCCL-PROGRAM is a symbol registered by `register-ccl-program',\nor a compiled code generated by `ccl-compile' (for backward compatibility,\nin this case, the execution is slower).\n\nRead buffer is set to STRING, and write buffer is allocated automatically.\n\nSTATUS is a vector of [R0 R1 ... R7 IC], where\n R0..R7 are initial values of corresponding registers,\n IC is the instruction counter specifying from where to start the program.\nIf R0..R7 are nil, they are initialized to 0.\nIf IC is nil, it is initialized to head of the CCL program.\n\nIf optional 4th arg CONTINUE is non-nil, keep IC on read operation\nwhen read buffer is exhausted, else, IC is always set to the end of\nCCL-PROGRAM on exit.\n\nIt returns the contents of write buffer as a string,\n and as side effect, STATUS is updated.\nIf the optional 5th arg UNIBYTE-P is non-nil, the returned string\nis a unibyte string.  By default it is a multibyte string.\n\nSee the documentation of `define-ccl-program' for the detail of CCL program.\nusage: (ccl-execute-on-string CCL-PROGRAM STATUS STRING &optional CONTINUE UNIBYTE-P)",
       (ccl_prog: Lisp_Object, status: Lisp_Object, str: Lisp_Object,
	contin: Lisp_Object, unibyte_p: Lisp_Object),
       {
  /* PORTING.md 1.5: C leaves `val`, `ccl`, `i`, `outbufsize`, the two pointers, the
     two string lengths, the three counters and `buf_magnification` without
     initialisers.  */
  let mut val: Lisp_Object = lisp_h_Qnil ();
  let mut ccl: ccl_program = std::mem::zeroed ();
  let mut i: c_int = 0;
  let mut outbufsize: ptrdiff_t = 0;
  let mut outbuf: *mut u8 = std::ptr::null_mut ();
  let mut outp: *mut u8 = std::ptr::null_mut ();
  let mut str_chars: ptrdiff_t = 0;
  let mut str_bytes: ptrdiff_t = 0;
  /* C 2132's `#define CCL_EXECUTE_BUF_SIZE 1024` is a constant in the port.  */
  const CCL_EXECUTE_BUF_SIZE: c_int = 1024;
  let mut source: [c_int; CCL_EXECUTE_BUF_SIZE as usize] =
    [0; CCL_EXECUTE_BUF_SIZE as usize];
  let mut destination: [c_int; CCL_EXECUTE_BUF_SIZE as usize] =
    [0; CCL_EXECUTE_BUF_SIZE as usize];
  let mut consumed_chars: ptrdiff_t = 0;
  let mut consumed_bytes: ptrdiff_t = 0;
  let mut produced_chars: ptrdiff_t = 0;
  let mut buf_magnification: c_int = 0;

  if ! setup_ccl_program (&raw mut ccl, ccl_prog)
    { error (c"Invalid CCL program".as_ptr ()); }

  CHECK_VECTOR (status);
  if ASIZE (status) != 9
    { error (c"Length of vector STATUS is not 9".as_ptr ()); }
  CHECK_STRING (str);

  str_chars = SCHARS (str);
  str_bytes = SBYTES (str);

  i = 0;
  while i < 8
    {
      if NILP (AREF (status, i as isize))
	{ ASET (status, i as isize, make_fixnum (0)); }
      /* PORTING.md 1.5: C declares `n` and lets `integer_to_intmax` store it.  */
      let mut n: intmax_t = 0;
      if INTEGERP (AREF (status, i as isize))
	  && integer_to_intmax (AREF (status, i as isize), &raw mut n)
	  && INT_MIN as intmax_t <= n && n <= INT_MAX as intmax_t
	{ ccl.reg[i as usize] = n as c_int; }
      i += 1;
    }
  /* PORTING.md 1.5: C declares `ic` and lets `integer_to_intmax` store it.  */
  let mut ic: intmax_t = 0;
  if INTEGERP (AREF (status, 8)) && integer_to_intmax (AREF (status, 8), &raw mut ic)
    {
      if (ccl.ic as intmax_t) < ic && ic < ccl.size as intmax_t
	{ ccl.ic = ic as c_int; }
    }

  buf_magnification = if ccl.buf_magnification != 0 { ccl.buf_magnification } else { 1 };
  outbufsize = str_bytes;
  /* C's `ckd_mul (…) || ckd_add (…)` with glibc's shared overflow check; the port
     stores the wrapped value, which is what the reference's <stdckdint.h>
     (`__builtin_mul_overflow`) does (PORTING.md 1.1/P-15, the bignum.rs shape).  */
  let (mul_value, mul_overflow) = outbufsize.overflowing_mul (buf_magnification as ptrdiff_t);
  outbufsize = mul_value;
  if mul_overflow
      || { let (add_value, add_overflow) = outbufsize.overflowing_add (256);
	   outbufsize = add_value; add_overflow }
    { memory_full_up (); }
  outbuf = xmalloc (outbufsize as usize) as *mut u8;
  outp = outbuf;

  consumed_chars = 0;
  consumed_bytes = 0;
  produced_chars = 0;
  loop
    {
      let mut p: *const u8 = SDATA (str).offset (consumed_bytes as isize) as *const u8;
      let endp: *const u8 = SDATA (str).offset (str_bytes as isize) as *const u8;
      /* C's `int j = 0` (C 2153).  */
      let mut j: c_int = 0;
      let mut src: *mut c_int;
      let mut src_size: c_int;

      if endp.offset_from (p) == str_chars - consumed_chars
	{
	  /* C's `source[j++] = *p++`: the store and the load use the old indices, then
	     both step (PORTING.md 2.2).  */
	  while j < CCL_EXECUTE_BUF_SIZE && p < endp
	    {
	      source[j as usize] = *p as c_int;
	      j += 1;
	      p = p.offset (1);
	    }
	}
      else
	{
	  while j < CCL_EXECUTE_BUF_SIZE && p < endp
	    {
	      source[j as usize] = string_char_advance (&raw mut p);
	      j += 1;
	    }
	}
      consumed_chars += j as ptrdiff_t;
      consumed_bytes = p.offset_from (SDATA (str) as *const u8);

      if consumed_bytes == str_bytes
	{ ccl.bits = if NILP (contin) { ccl.bits | CCL_PROGRAM_LAST_BLOCK }
		      else { ccl.bits & ! CCL_PROGRAM_LAST_BLOCK }; }
      src = source.as_mut_ptr ();
      src_size = j;
      loop
	{
	  let max_expansion: c_int = if NILP (unibyte_p) { MAX_MULTIBYTE_LENGTH } else { 1 };
	  /* PORTING.md 1.5: C declares `offset` and `shortfall` uninitialised.  */
	  let mut offset: ptrdiff_t = 0;
	  let mut shortfall: ptrdiff_t = 0;
	  ccl_driver (&raw mut ccl, src, destination.as_mut_ptr (), src_size,
		      CCL_EXECUTE_BUF_SIZE, Qnil ());
	  produced_chars += ccl.produced as ptrdiff_t;
	  offset = outp.offset_from (outbuf);
	  shortfall = (ccl.produced as ptrdiff_t) * (max_expansion as ptrdiff_t)
		      - (outbufsize - offset);
	  if shortfall > 0
	    {
	      outbuf = xpalloc (outbuf as *mut c_void, &raw mut outbufsize, shortfall, -1, 1)
		      as *mut u8;
	      outp = outbuf.offset (offset);
	    }
	  if NILP (unibyte_p)
	    {
	      j = 0;
	      while j < ccl.produced
		{
		  outp = outp.offset (CHAR_STRING (destination[j as usize], outp) as isize);
		  j += 1;
		}
	    }
	  else
	    {
	      j = 0;
	      while j < ccl.produced
		{
		  *outp = destination[j as usize] as u8;
		  outp = outp.offset (1);
		  j += 1;
		}
	    }
	  src = src.offset (ccl.consumed as isize);
	  src_size -= ccl.consumed;
	  if ccl.status != CCL_STAT_SUSPEND_BY_DST
	    { break; }
	}

      if ccl.status != CCL_STAT_SUSPEND_BY_SRC
	  || str_chars == consumed_chars
	{ break; }
    }

  if ccl.status == CCL_STAT_INVALID_CMD
    { error (c"Error in CCL program at %dth code".as_ptr (), ccl.ic); }
  if ccl.status == CCL_STAT_QUIT
    { error (c"CCL program interrupted at %dth code".as_ptr (), ccl.ic); }

  i = 0;
  while i < 8
    {
      ASET (status, i as isize, make_int (ccl.reg[i as usize] as intmax_t));
      i += 1;
    }
  ASET (status, 8, make_int (ccl.ic as intmax_t));

  val = make_specified_string (outbuf as *const c_char, produced_chars,
			       outp.offset_from (outbuf), NILP (unibyte_p));
  xfree (outbuf as *mut c_void);

  return val;
});

DEFUN! ("register-ccl-program", Fregister_ccl_program, Sregister_ccl_program,
       2, 2, 0,
       doc: "Register CCL program CCL-PROG as NAME in `ccl-program-table'.\nCCL-PROG should be a compiled CCL program (vector), or nil.\nIf it is nil, just reserve NAME as a CCL program name.\nReturn index number of the registered CCL program.",
       (name: Lisp_Object, ccl_prog: Lisp_Object),
       {
  let mut ccl_prog = ccl_prog;   // C assigns to the parameter below (C 2266).
  /* C's `len` is C's own initialiser; the other two are uninitialised in C
     (PORTING.md 1.5).  */
  let len: ptrdiff_t = ASIZE (Vccl_program_table);
  let mut idx: ptrdiff_t = 0;
  let mut resolved: Lisp_Object = lisp_h_Qnil ();

  CHECK_SYMBOL (name);
  resolved = Qnil ();
  if !NILP (ccl_prog)
    {
      CHECK_VECTOR (ccl_prog);
      resolved = resolve_symbol_ccl_program (ccl_prog);
      if NILP (resolved)
	{ error (c"Error in CCL program".as_ptr ()); }
      if VECTORP (resolved)
	{
	  ccl_prog = resolved;
	  resolved = Qt ();
	}
      else
	{ resolved = Qnil (); }
    }

  idx = 0;
  while idx < len
    {
      /* C declares `slot` and assigns it here.  */
      let mut slot: Lisp_Object = lisp_h_Qnil ();

      slot = AREF (Vccl_program_table, idx);
      if !VECTORP (slot)
	/* This is the first unused slot.  Register NAME here.  */
	{ break; }

      if EQ (name, AREF (slot, 0))
	{
	  /* Update this slot.  */
	  ASET (slot, 1, ccl_prog);
	  ASET (slot, 2, resolved);
	  ASET (slot, 3, Qt ());
	  return make_fixnum (idx);
	}
      idx += 1;
    }

  if idx == len
    /* Extend the table.  */
    { Vccl_program_table = larger_vector (Vccl_program_table, 1, -1); }

  ASET (Vccl_program_table, idx,
	CALLN! (Fvector, name, ccl_prog, resolved, Qt ()));

  Fput (name, Qccl_program_idx (), make_fixnum (idx));
  return make_fixnum (idx);
});

/* Register code conversion map.
   A code conversion map consists of numbers, Qt, Qnil, and Qlambda.
   The first element is the start code point.
   The other elements are mapped numbers.
   Symbol t means to map to an original number before mapping.
   Symbol nil means that the corresponding element is empty.
   Symbol lambda means to terminate mapping here.
*/

DEFUN! ("register-code-conversion-map", Fregister_code_conversion_map,
       Sregister_code_conversion_map,
       2, 2, 0,
       doc: "Register SYMBOL as code conversion map MAP.\nReturn index number of the registered map.",
       (symbol: Lisp_Object, map: Lisp_Object),
       {
  /* PORTING.md 1.5: C declares `len`, `i` and `idx` without initialisers.  */
  let mut len: ptrdiff_t = 0;
  let mut i: ptrdiff_t = 0;
  let mut idx: Lisp_Object = lisp_h_Qnil ();

  CHECK_SYMBOL (symbol);
  CHECK_VECTOR (map);
  if ! VECTORP (Vcode_conversion_map_vector)
    { error (c"Invalid code-conversion-map-vector".as_ptr ()); }

  len = ASIZE (Vcode_conversion_map_vector);

  i = 0;
  while i < len
    {
      let slot: Lisp_Object = AREF (Vcode_conversion_map_vector, i);

      if !CONSP (slot)
	{ break; }

      if EQ (symbol, XCAR (slot))
	{
	  idx = make_fixnum (i);
	  XSETCDR (slot, map);
	  Fput (symbol, Qcode_conversion_map (), map);
	  Fput (symbol, Qcode_conversion_map_id (), idx);
	  return idx;
	}
      i += 1;
    }

  if i == len
    { Vcode_conversion_map_vector = larger_vector (Vcode_conversion_map_vector,
						   1, -1); }

  idx = make_fixnum (i);
  Fput (symbol, Qcode_conversion_map (), map);
  Fput (symbol, Qcode_conversion_map_id (), idx);
  ASET (Vcode_conversion_map_vector, i, Fcons (symbol, map));
  return idx;
});


/* The two `DEFVAR_LISP` variables of this unit (PORTING.md 3.29/P-21: the unit that
   C's `DEFVAR_*` gives the storage to defines it, and the definition shadows the
   declaration `globals.rs`'s `variables` module carries).  */
#[unsafe(no_mangle)]
pub static mut Vcode_conversion_map_vector: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vtranslation_hash_table_vector: Lisp_Object = lisp_h_Qnil ();


#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_ccl ()
{
  staticpro (&raw const Vccl_program_table);
  Vccl_program_table = make_nil_vector (32);

  DEFSYM! (Qccl, c"ccl");
  DEFSYM! (Qcclp, c"cclp");

  /* Symbols of ccl program have this property, a value of the property
     is an index for Vccl_program_table. */
  DEFSYM! (Qccl_program_idx, c"ccl-program-idx");

  /* These symbols are properties which associate with code conversion
     map and their ID respectively.  */
  DEFSYM! (Qcode_conversion_map, c"code-conversion-map");
  DEFSYM! (Qcode_conversion_map_id, c"code-conversion-map-id");

  DEFVAR_LISP! (c"code-conversion-map-vector", Vcode_conversion_map_vector,
		"Vector of code conversion maps.");
  Vcode_conversion_map_vector = make_nil_vector (16);

  DEFVAR_LISP! (c"translation-hash-table-vector", Vtranslation_hash_table_vector,
		"Vector containing all translation hash tables ever defined.\nComprises pairs (SYMBOL . TABLE) where SYMBOL and TABLE were set up by calls\nto `define-translation-hash-table'.  The vector is indexed by the table id\nused by CCL.");
    Vtranslation_hash_table_vector = Qnil ();

  defsubr (&raw mut Sccl_program_p);
  defsubr (&raw mut Sccl_execute);
  defsubr (&raw mut Sccl_execute_on_string);
  defsubr (&raw mut Sregister_ccl_program);
  defsubr (&raw mut Sregister_code_conversion_map);
}
