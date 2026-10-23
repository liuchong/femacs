# emacs/exec/mipsel-user.h (46 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _MIPSEL_USER_H_  (line 23, body da39a3ee5e6b)

## Types and their fields — 1
- [x] mipsel_regs: uint64_t gregs, uint64_t lo, uint64_t hi, uint64_t cp0_epc, uint64_t cp0_badvaddr, uint64_t cp0_status, uint64_t cp0_cause

## File-scope objects — 0

## Decisions

Mirror of exec/mipsel-user.h, complete for this target: the include guard and `struct
mipsel_regs` (the 32 general purpose registers of a 32-bit mipsel system plus its six saved
special registers), whose fields C's own comments describe and the mirror keeps.

C 23's `#include <sys/user.h>` is recorded (no mirror for it, and the mirror names nothing from
it); `uint64_t` comes from the `<stdint.h>` mirror the file includes.

The layout asserts are **arithmetic from the field types**, not measurements: no mipsel
toolchain exists on this host and `exec/` is compiled by no configuration the port builds
(docs/baseline/frozen-build-sources.txt's batch-F section), so the four `const _` lines check
what the field types imply (256 bytes of registers, then six 8-byte words) and document them.

Evidence: `python3 tools/extract-abi.py --check emacs/exec/mipsel-user.h
exec/include/mipsel-user.rs` -> `OK` (1 ledger item).
