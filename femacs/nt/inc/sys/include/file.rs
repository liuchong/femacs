// femacs: mirror of nt/inc/sys/file.h @ fd7afced403.
//
// The Windows port's shim: `sys/file.h` does not exist on NT, and only these five constants are
// needed from it.  Nothing in the frozen configuration includes it (no `nt/` unit is compiled),
// so the mirror is never compiled by the layout probe; the values are C's own.

/// C 6: `#define F_OK 0`
pub const F_OK: c_int = 0;
/// C 7: `#define X_OK 1`
pub const X_OK: c_int = 1;
/// C 8: `#define W_OK 2`
pub const W_OK: c_int = 2;
/// C 9: `#define R_OK 4`
pub const R_OK: c_int = 4;
/// C 10: `#define D_OK 8`
pub const D_OK: c_int = 8;
