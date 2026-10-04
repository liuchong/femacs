# emacs/exec/mipsfpu.h (82 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] FP_FR0  (line 67, body b6589fc6ab0d)
- [x] FP_FR1  (line 68, body 356a192b7913)
- [x] FP_FRE  (line 69, body 77de68daecd8)
- [x] _MIPSFPU_H_  (line 23, body da39a3ee5e6b)

## Types and their fields — 1
- [x] mips_elf_abi_flags: uint16_t version, uint8_t isa_level, uint8_t isa_rev, uint8_t gpr_size, uint8_t cpr1_size, uint8_t cpr2_size, uint8_t fp_abi, uint32_t isa_ext, uint32_t ases, uint32_t flags1, uint32_t flags2

## File-scope objects — 0

## Decisions

Mirror of exec/mipsfpu.h, complete for this target: `struct mips_elf_abi_flags` (the flag words
the loader reads out of a traced binary, C's comments and all), the three `FP_FR*` mode
constants, and the pair `mipsfpu.c` defines (`cpu_supports_fr0_p`, `determine_fpu_mode`).

C 24's `#include "exec.h"` is the `include!("exec.rs")` of the sibling mirror, which is where
`elf_header` (C 135-143's `EXEC_64` selection) comes from.  The layout asserts are arithmetic
from the field types, not measurements (no mipsel toolchain on this host; `exec/` is compiled by
no configuration the port builds).

Nothing compiles the mirror: `xtask layout-check` covers `src/include` and `lib/include`.

Evidence: `python3 tools/extract-abi.py --check emacs/exec/mipsfpu.h
femacs/exec/include/mipsfpu.rs` -> `OK` (6 ledger items).
