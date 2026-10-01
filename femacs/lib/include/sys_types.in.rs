// femacs: port of lib/sys_types.in.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror: only `off_t`, which buffer.h names; the rest of the header is
// tracked in docs/ports/include-sys_types.in.md (PORTING.md P-26).

/// The frozen target's `off_t` (a 64-bit signed integer).
pub type off_t = i64;
