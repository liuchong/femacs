// femacs: mirror of exec/mipsfpu.h @ fd7afced403.
//
// The exec loader's MIPS floating-point probe: `struct mips_elf_abi_flags` (the ELF
// `EF_MIPS_ABI*`/`EF_MIPS_*` flag words the loader reads out of a traced binary), the three
// FP mode constants, and the pair `mipsfpu.c` defines.  The frozen configuration compiles
// nothing under `exec/` and `xtask layout-check` covers `src/include` and `lib/include` only,
// so the mirror is text: it is the target the port-map names for this header.

include!("exec.rs");                             // C 24: `#include "exec.h"` (`elf_header`)
include!("../../../lib/include/stdint.in.rs");   // `uint16_t`/`uint8_t`/`uint32_t` (C 26's `<stdint.h>`)

/* C 19-21's include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _MIPSFPU_H_: bool = true;

/// C 27-58: `struct mips_elf_abi_flags` - the fields are C's, comments and all.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mips_elf_abi_flags
{
  /// Version of flags structure.
  pub version: uint16_t,

  /// The level of the ISA: 1-5, 32, 64.
  pub isa_level: uint8_t,

  /// The revision of ISA: 0 for MIPS V and below, 1-n otherwise.
  pub isa_rev: uint8_t,

  /// The size of general purpose registers.
  pub gpr_size: uint8_t,

  /// The size of co-processor 1 registers.
  pub cpr1_size: uint8_t,

  /// The size of co-processor 2 registers.
  pub cpr2_size: uint8_t,

  /// The floating-point ABI.
  pub fp_abi: uint8_t,

  /// Mask of processor-specific extensions.
  pub isa_ext: uint32_t,

  /// Mask of ASEs used.
  pub ases: uint32_t,

  /// Mask of general flags.
  pub flags1: uint32_t,

  /// Mask of general flags.
  pub flags2: uint32_t,
}

/* Arithmetic from the field types, not a measurement: the two `uint16_t`-aligned halves put
   `isa_ext` at 8 and the four `uint32_t` masks after it, and no mipsel toolchain exists on this
   host to measure with (exec/ is compiled by no configuration the port builds).  */
const _: () = assert! (std::mem::size_of::<mips_elf_abi_flags> () == 24);
const _: () = assert! (std::mem::offset_of! (mips_elf_abi_flags, version) == 0);
const _: () = assert! (std::mem::offset_of! (mips_elf_abi_flags, isa_ext) == 8);
const _: () = assert! (std::mem::offset_of! (mips_elf_abi_flags, flags2) == 20);

/* C 63-65: `#define FP_FR0 0`, `#define FP_FR1 1`, `#define FP_FRE 3` - "Floating point
   modes.", in C's order.  */
pub const FP_FR0: c_int = 0;
pub const FP_FR1: c_int = 1;
pub const FP_FRE: c_int = 3;

/* C 69: "Defined in mipsfpu.c."  */
unsafe extern "C" {
  /// C 71: `extern bool cpu_supports_fr0_p (void);`
  pub fn cpu_supports_fr0_p () -> bool;
  /// C 72-74: `extern int determine_fpu_mode (elf_header *, elf_header *, int *,
  /// struct mips_elf_abi_flags *, struct mips_elf_abi_flags *);`
  pub fn determine_fpu_mode (first: *mut elf_header, second: *mut elf_header,
                             fp_abi: *mut c_int, in_flags: *mut mips_elf_abi_flags,
                             out_flags: *mut mips_elf_abi_flags) -> c_int;
}
