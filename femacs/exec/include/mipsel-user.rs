// femacs: mirror of exec/mipsel-user.h @ fd7afced403.
//
// The exec loader's MIPS register file: `struct mipsel_regs`, the 32 user-mode general purpose
// registers of a 32-bit mipsel system plus its six saved special registers.  The frozen
// configuration compiles nothing under `exec/` (docs/baseline/frozen-build-sources.txt: "the
// reference build has no exec/ directory and no rule reaches it"), and `xtask layout-check`
// covers `src/include` and `lib/include` only, so the mirror is text: it is the target the
// port-map names for this header, and its layout is C's own.

include!("../../../lib/include/stdint.in.rs");   // `uint64_t` (C 23's `<sys/user.h>` supplies it in C)

/* C 19-21's include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _MIPSEL_USER_H_: bool = true;

/* C 23: `#include <sys/user.h>` - the host header this file's name mirrors; the port has no
   mirror for it and the mirror names nothing from it, so the include is recorded (the
   binary-io.rs dead-arm note).  */

/* C 28-29's comment: "This file defines a structure containing user mode general purpose
   registers on 32-bit mipsel systems."  */
/// C 31-45: `struct mipsel_regs`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mipsel_regs
{
  /// General purpose registers.
  pub gregs: [uint64_t; 32],

  /// Saved special registers.
  pub lo: uint64_t,
  pub hi: uint64_t,
  pub cp0_epc: uint64_t,
  pub cp0_badvaddr: uint64_t,
  pub cp0_status: uint64_t,
  pub cp0_cause: uint64_t,
}

/* The layout is arithmetic from the field types (sixteen 8-byte-aligned `uint64_t` fields after
   a 32-element array of them), not a measurement: there is no mipsel toolchain on this host to
   run clang against, and `exec/mipsel-user.h` is compiled by no configuration the port builds
   (docs/baseline/frozen-build-sources.txt's batch-F section).  */
const _: () = assert! (std::mem::size_of::<mipsel_regs> () == 256 + 6 * 8);
const _: () = assert! (std::mem::offset_of! (mipsel_regs, gregs) == 0);
const _: () = assert! (std::mem::offset_of! (mipsel_regs, lo) == 256);
const _: () = assert! (std::mem::offset_of! (mipsel_regs, cp0_cause) == 256 + 5 * 8);
