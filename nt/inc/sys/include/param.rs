// femacs: mirror of nt/inc/sys/param.h @ fd7afced403.
//
// This is the Windows port's shim header: `sys/param.h` does not exist on NT, so the C tree
// provides one with a single macro.  Nothing in the frozen configuration includes it (no
// `nt/` translation unit is compiled - docs/baseline/frozen-build-sources.txt's batch-F
// section - and no `src/` unit includes a `nt/inc/` header), so the mirror is never compiled by
// `xtask layout-check`; it exists so the port is complete and the mapping has a target.

// C 2: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _PARAM_H_: bool = true;

/// C 8: `#define NBPG 4096`
pub const NBPG: c_int = 4096;
