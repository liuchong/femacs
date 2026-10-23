// femacs: port of exec/mipsfpu.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct field, macro, file-scope object, local), the same control flow and the same error model.
//
// **The frozen configuration compiles none of `exec/`** (`docs/baseline/frozen-build-sources.txt`:
// the loader belongs to the unexec dump path, and this build is `HAVE_PDUMPER=1`), and PORTING.md
// P-86 governs a file in that state: the whole body is transcribed as it stands - this file's
// previous revision was the three-line P-08 note.  The file has exactly one `#if` of its own,
// `cpu_supports_fr0_p`'s `__mips_isa_rev >= 6` probe (C 140-145), and both of its arms are carried
// under the feature `mips_isa_rev_ge_6` (the `android_api_ge_9` naming precedent, PORTING.md 0.1-7);
// `check-cond-eval` reads the file as 1 condition, 0 unknown.
//
// One name is the *target's*, not this host's: `ELIBBAD` is Linux's errno (this file is the loader
// of an OABI MIPS Linux system), and this host's `<errno.h>` does not define it, so it is declared
// here with its Linux value and marked UNVERIFIED (P-88 ①/②).  `ENOEXEC` is in the `<errno.h>`
// mirror, measured on this host (8).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar,
              c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");   // C 20: `#include <config.h>`
include!("../lib/include/errno.in.rs"); // C 21: `#include <errno.h>`
include!("include/mipsfpu.rs");         // C 23: `#include "mipsfpu.h"`

/* C 25-40: the "OABI MIPS systems support several different modes of execution …" commentary.  It
   is C's own text and is carried in the ledger rather than repeated here (P-86 ①).  */

/* C 42: "Various constants used throughout this code."  */

/* C 43-50: the eight floating-point ABI numbers.  */
/// C 43: `#define MIPS_ABI_FP_ANY		0	/* FP ABI doesn't matter */`
pub const MIPS_ABI_FP_ANY: c_int = 0;
/// C 44: `#define MIPS_ABI_FP_DOUBLE	1	/* -mdouble-float */`
pub const MIPS_ABI_FP_DOUBLE: c_int = 1;
/// C 45: `#define MIPS_ABI_FP_SINGLE	2	/* -msingle-float */`
pub const MIPS_ABI_FP_SINGLE: c_int = 2;
/// C 46: `#define MIPS_ABI_FP_SOFT	3	/* -msoft-float */`
pub const MIPS_ABI_FP_SOFT: c_int = 3;
/// C 47: `#define MIPS_ABI_FP_OLD_64	4	/* -mips32r2 -mfp64 */`
pub const MIPS_ABI_FP_OLD_64: c_int = 4;
/// C 48: `#define MIPS_ABI_FP_XX		5	/* -mfpxx */`
pub const MIPS_ABI_FP_XX: c_int = 5;
/// C 49: `#define MIPS_ABI_FP_64		6	/* -mips32r2 -mfp64 */`
pub const MIPS_ABI_FP_64: c_int = 6;
/// C 50: `#define MIPS_ABI_FP_64A		7	/* -mips32r2 -mfp64 -mno-odd-spreg */`
pub const MIPS_ABI_FP_64A: c_int = 7;

/* C 52-61: the ELF flag words the loader reads out of a traced binary.  `uint32_t` is the type of
   `elf_header.e_flags` (`exec/include/exec.rs` 62/107), which is what every use below masks.  */
/// C 52: `#define EF_MIPS_NOREORDER	1     /* A .noreorder directive was used.  */`
pub const EF_MIPS_NOREORDER: uint32_t = 1;
/// C 53: `#define EF_MIPS_PIC		2     /* Contains PIC code.  */`
pub const EF_MIPS_PIC: uint32_t = 2;
/// C 54: `#define EF_MIPS_CPIC		4     /* Uses PIC calling sequence.  */`
pub const EF_MIPS_CPIC: uint32_t = 4;
/// C 55: `#define EF_MIPS_XGOT		8`
pub const EF_MIPS_XGOT: uint32_t = 8;
/// C 56: `#define EF_MIPS_64BIT_WHIRL	16`
pub const EF_MIPS_64BIT_WHIRL: uint32_t = 16;
/// C 57: `#define EF_MIPS_ABI2		32`
pub const EF_MIPS_ABI2: uint32_t = 32;
/// C 58: `#define EF_MIPS_ABI_ON32	64`
pub const EF_MIPS_ABI_ON32: uint32_t = 64;
/// C 59: `#define EF_MIPS_FP64		512  /* Uses FP64 (12 callee-saved).  */`
pub const EF_MIPS_FP64: uint32_t = 512;
/// C 60: `#define EF_MIPS_NAN2008		1024  /* Uses IEEE 754-2008 NaN encoding.  */`
pub const EF_MIPS_NAN2008: uint32_t = 1024;
/// C 61: `#define EF_MIPS_ARCH		0xf0000000 /* MIPS architecture level.  */`
pub const EF_MIPS_ARCH: uint32_t = 0xf0000000;

/* C 63: `<errno.h>`'s `ELIBBAD`, the one errno this file names that this host's header does not
   define (it is Linux's, 80); the `<errno.h>` mirror cannot carry it, so C's line is declared here
   with the target's value and the standing UNVERIFIED mark.  */
/// C: `#define ELIBBAD 80   [Linux's <errno.h>; UNVERIFIED here - no Linux headers on this host]`
pub const ELIBBAD: c_int = 80;

/// C 68: `struct mode_description` - "Structure describing the requirements of a single
/// floating-point ABI."
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mode_description
{
  /// Whether or not the ABI only executes single precision
  /// instructions, and can operate in both 32-bit or 64-bit floating
  /// point mode.
  pub single: bool,

  /// Whether or not the ABI performs floating point operations in
  /// software, using integer registers.
  pub soft: bool,

  /// Whether or not the ABI requires the use of 64-bit floating point
  /// registers.
  pub fr1: bool,

  /// Whether or not the ABI requires the use of 64-bit floating point
  /// registers on NABI systems, and 32-bit ones on OABI systems.
  pub frdefault: bool,

  /// Whether or not this ABI requires single precision floating point
  /// emulation.
  pub fre: bool,
}

/// C 92-104: `static struct mode_description fpu_reqs[] = { [MIPS_ABI_FP_ANY] = { true, … }, … }` -
/// the designated initialisers are C's `MIPS_ABI_FP_*` indices, which run 0-7 in C's order (the
/// comment is "`fpu_reqs`…").  `static` in C is internal linkage, so the port keeps it a plain
/// `static mut` with no `#[unsafe(no_mangle)]` (PORTING.md 0.2).
static mut fpu_reqs: [mode_description; 8] =
  [ mode_description { single: true,  soft: true,  fr1: true,  frdefault: true,  fre: true,  },
    mode_description { single: false, soft: false, fr1: false, frdefault: true,  fre: true,  },
    mode_description { single: true,  soft: false, fr1: false, frdefault: false, fre: false, },
    mode_description { single: false, soft: true,  fr1: false, frdefault: false, fre: false, },
    mode_description { single: false, soft: false, fr1: false, frdefault: false, fre: false, },
    mode_description { single: false, soft: false, fr1: true,  frdefault: true,  fre: true,  },
    mode_description { single: false, soft: false, fr1: true,  frdefault: false, fre: false, },
    mode_description { single: false, soft: false, fr1: true,  frdefault: false, fre: true,  },
  ];

/// C 106-118: `static bool valid_abi_p (int abi)` - "Return whether or not the given
/// floating-point ABI is valid."
unsafe fn valid_abi_p (abi: c_int) -> bool
{
  /* C's `switch (abi)` - eight `case`s that all `return true` and a `default` that returns
     `false` - spelled as the `match` the port uses for `switch` (PORTING.md 2.2).  */
  match abi
    {
      MIPS_ABI_FP_ANY | MIPS_ABI_FP_DOUBLE | MIPS_ABI_FP_SINGLE | MIPS_ABI_FP_SOFT
      | MIPS_ABI_FP_OLD_64 | MIPS_ABI_FP_XX | MIPS_ABI_FP_64 | MIPS_ABI_FP_64A
	=> true,

      _ => false,
    }
}

/// C 120-133: `static int fp_mode_for_abi (int abi)` - "Return the floating point mode appropriate
/// for the specified floating point ABI."
unsafe fn fp_mode_for_abi (abi: c_int) -> c_int
{
  /* C: `struct mode_description *desc;` then `desc = &fpu_reqs[abi];`.  The port takes the address
     of the element with a raw pointer and `offset`, so a negative `abi` behaves the way C's
     subscript does instead of panicking (PORTING.md P-72).  */
  let desc: *mut mode_description;

  desc = (&raw mut fpu_reqs).cast::<mode_description> ().offset (abi as isize);

  if (*desc).fre
    { return FP_FRE; }          /* C's `return FP_FRE;` */
  else if (*desc).fr1
    { return FP_FR1; }

  return FP_FR0;
}

/// C 135-146: `bool cpu_supports_fr0_p (void)` - "Determine whether or not the CPU is capable of
/// operating in FR0 floating point mode."  The body is the file's one `#if`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_supports_fr0_p () -> bool
{
  /* C 140-145: `#if defined __mips_isa_rev && __mips_isa_rev >= 6` / `return true;` / `#else` /
     `return false;` / `#endif`.  The condition is a compiler builtin, not a measured
     configuration macro, so the port names its two arms with the feature `mips_isa_rev_ge_6`
     (PORTING.md 0.1-7) and the frozen target - which has no `__mips_isa_rev` - compiles the
     `#else` arm.  */
  #[cfg (feature = "mips_isa_rev_ge_6")]
  {
    return true;
  }
  #[cfg (not (feature = "mips_isa_rev_ge_6"))]
  {
    return false;
  }
}

/// C 148-293: `int determine_fpu_mode (elf_header *header, elf_header *interpreter, int *mode,
/// struct mips_elf_abi_flags *abiflags, struct mips_elf_abi_flags *abiflags1)` - "Determine the FPU
/// mode for the executable whose ELF header is HEADER …"  C's five locals are declared here
/// uninitialised; the port spells C's own zero values (PORTING.md 0.1-13).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn determine_fpu_mode (header: *mut elf_header, interpreter: *mut elf_header,
					     mode: *mut c_int,
					     abiflags: *mut mips_elf_abi_flags,
					     abiflags1: *mut mips_elf_abi_flags) -> c_int
{
  let mut exec_abi: c_int = 0;
  let mut interpreter_abi: c_int = 0;
  let mut exec_desc: *mut mode_description = std::ptr::null_mut ();
  let mut interpreter_desc: *mut mode_description = std::ptr::null_mut ();
  let mut common: mode_description =
    mode_description { single: false, soft: false, fr1: false, frdefault: false, fre: false };

  /* Figure out the executable's floating point ABI.  First, consult
     header->e_flags, and use the old 64-bit floating point ABI if it
     is specified.  */

  exec_abi = MIPS_ABI_FP_ANY;

  /* First, check HEADER->e_flags.  */

  if (*header).e_flags & EF_MIPS_FP64 != 0
    { exec_abi = MIPS_ABI_FP_OLD_64; }

  /* Next, use ABIFLAGS if it exists.  */

  if !abiflags.is_null () && valid_abi_p ((*abiflags).fp_abi as c_int)
    { exec_abi = (*abiflags).fp_abi as c_int; }
  else if !abiflags.is_null ()
    {
      *errno () = ENOEXEC;
      return 1;
    }

  /* Now determine that of the interpreter.  */

  interpreter_abi = MIPS_ABI_FP_ANY;

  if !interpreter.is_null ()
    {
      if (*interpreter).e_flags & EF_MIPS_FP64 != 0
	{ interpreter_abi = MIPS_ABI_FP_OLD_64; }

      /* C 241 tests `abiflags1` for `NULL` but then reads `abiflags->fp_abi` - the reference's own
	 typo, carried as it stands (bug for bug; the ledger's Deviations records it).  */
      if !abiflags1.is_null () && valid_abi_p ((*abiflags).fp_abi as c_int)
	{ interpreter_abi = (*abiflags).fp_abi as c_int; }
      else if !abiflags1.is_null ()
	{
	  *errno () = ELIBBAD;
	  return 1;
	}
    }

  /* If no interpreter flag is set, just return that of the
     executable.  */

  if interpreter.is_null ()
    {
      *mode = fp_mode_for_abi (exec_abi);
      return 0;
    }

  /* Otherwise, compare both ABIs and try to find one which will run
     both kinds of code.

     First, see if there's an easy way out: both ABIs are identical,
     or one ABI is MIPS_ABI_FP_ANY.  */

  if exec_abi == interpreter_abi
    {
      *mode = fp_mode_for_abi (exec_abi);
      return 0;
    }
  else if exec_abi == MIPS_ABI_FP_ANY
    {
      *mode = fp_mode_for_abi (interpreter_abi);
      return 0;
    }
  else if interpreter_abi == MIPS_ABI_FP_ANY
    {
      *mode = fp_mode_for_abi (exec_abi);
      return 0;
    }

  /* If that doesn't work, compare various characteristics of both
     ABIs and select an appropriate floating point mode.  */

  exec_desc = (&raw mut fpu_reqs).cast::<mode_description> ().offset (exec_abi as isize);
  interpreter_desc
    = (&raw mut fpu_reqs).cast::<mode_description> ().offset (interpreter_abi as isize);

  /* Merge both sets of requirements.  */
  common.single = (*exec_desc).single && (*interpreter_desc).single;
  common.soft = (*exec_desc).soft && (*interpreter_desc).soft;
  common.fr1 = (*exec_desc).fr1 && (*interpreter_desc).fr1;
  common.frdefault = (*exec_desc).frdefault && (*interpreter_desc).frdefault;
  common.fre = (*exec_desc).fre && (*interpreter_desc).fre;

  /* Default to a mode capable of running code expecting 32-bit
     registers.  */

  if (*header).e_flags & EF_MIPS_ABI2 == 0
    { *mode = FP_FR0; }
  else
    /* But in this case, use FR1.  */
    { *mode = FP_FR1; }

  if common.fre && !common.frdefault && !common.fr1
    /* Floating point emulation mode is required.  */
    { *mode = FP_FRE; }
  else if (common.fr1 && common.frdefault)
	   || (common.single && !common.frdefault)
	   || common.fr1
    /* 64-bit mode is required.  */
    { *mode = FP_FR1; }
  else if !common.fre && !common.frdefault
	   && !common.fr1 && !common.single
	   && !common.soft
    {
      /* The floating point modes specified are incompatible.  */
      *errno () = ELIBBAD;
      return -1;
    }

  return 0;
}
