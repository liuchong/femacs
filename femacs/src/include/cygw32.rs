// femacs: port of src/cygw32.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header is the Cygwin side's interface: C 19-20 is the
// include guard, C 21-34 are the system and project headers it pulls in, and C 36-38 are the
// three declarations `src/cygw32.c` defines.  There is no `#if` anywhere in the file
// (`tools/check-cond-eval.py src/cygw32.h` -> `1 conditions, 0 unknown, 0 disagreements
// [VERIFIED]`, and that one condition is the guard), so every line the mirror carries is live
// text.
//
// Nothing on this target compiles the header: the frozen configuration's platform object
// variables are empty (`CYGWIN_OBJ=`, `MSDOS_OBJ=`, `WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ=` -
// docs/baseline/frozen-build-sources.txt), so `src/cygw32.c`'s port is the empty shape
// `femacs/src/cygw32.rs` records and no unit `include!`s this mirror.  The declarations are
// carried anyway, because the header's text is what a mirror mirrors (the `src/w32select.h`
// precedent, status 3.158); their definitions are deferred to the Cygwin target.
//
// The header's own `#include`s (C 21-34: <config.h>, <windef.h>, <sys/cygwin.h>, <wchar.h>,
// <signal.h>, <stdio.h>, <limits.h>, <errno.h>, <math.h>, <setjmp.h>, "lisp.h", "coding.h")
// contribute nothing to the three declarations below, so the mirror carries no `include!` of
// its own: on the target that compiles this header, a unit reaches the platform's names through
// its own includes (PORTING.md 0.1-9, the `include/w32inevt.rs` reading).

// C 19-20: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const CYGW32_H: bool = true;

/// C 36-38: the three entry points `src/cygw32.c` defines; this target's build has no such
/// translation unit (its empty unit records the exclusion).
/// C: extern void syms_of_cygw32 (void);
/// C: extern char * w32_strerror (int error_no);
/// C: extern LPBYTE w32_get_resource (const char * key, const char * name, LPDWORD type);
unsafe extern "C" {
  pub fn syms_of_cygw32 ();

  /// C 37.  `char *` is C's own pointee; the parameter is an errno value.
  pub fn w32_strerror (error_no: c_int) -> *mut c_char;

  /// C 38.  `LPBYTE` and `LPDWORD` come from the platform's <windef.h> (C 22), which the port
  /// has no mirror of: the port's `include/` carries no windows.h, so the pointer *shape* C
  /// writes is kept and the pointee is spelled with the type C's typedef names - `LPBYTE` is
  /// `unsigned char *`, `LPDWORD` is `DWORD *` (PORTING.md 1.1 spells C's `unsigned long`
  /// `c_ulong`, which is DWORD's own type on that platform).  A unit that needs the typedef
  /// names themselves takes them from that target's headers (PORTING.md 0.1-9).
  pub fn w32_get_resource (key: *const c_char, name: *const c_char,
                           type_: *mut c_ulong) -> *mut u8;
}
