// femacs: mirror of nt/inc/sys/mman.h @ fd7afced403.
//
// mman-win32's header, which the Windows build puts on the include path; `src/buffer.c`'s
// `#ifdef WINDOWSNT` arm is the reason `MAP_ANON` is needed there.  Nothing in the frozen
// configuration includes it, so the mirror is never compiled by the layout probe.

// C 7: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _SYS_MMAN_H_: bool = true;

/// C 17: `#define MAP_FILE 0`
pub const MAP_FILE: c_int = 0;
/// C 18: `#define MAP_SHARED 1`
pub const MAP_SHARED: c_int = 1;
/// C 19: `#define MAP_PRIVATE 2`
pub const MAP_PRIVATE: c_int = 2;
/// C 20: `#define MAP_TYPE 0xf`
pub const MAP_TYPE: c_int = 0xf;
/// C 21: `#define MAP_FIXED 0x10`
pub const MAP_FIXED: c_int = 0x10;
/// C 22: `#define MAP_ANONYMOUS 0x20`
pub const MAP_ANONYMOUS: c_int = 0x20;
/// C 23: `#define MAP_ANON MAP_ANONYMOUS`
pub const MAP_ANON: c_int = MAP_ANONYMOUS;
/// C 25: `#define MAP_FAILED ((void *)-1)`
pub const MAP_FAILED: *mut c_void = (-1i64) as usize as *mut c_void;
