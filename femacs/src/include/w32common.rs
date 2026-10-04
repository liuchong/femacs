// femacs: port of src/w32common.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header is the shared interface of the Windows builds:
// C 21-22 is the include guard, C 24 the one system header, C 26-33 the rounding and
// system-info accessors, C 35-48 the objects and the OS-subtype enum, C 51 the one cache
// routine, C 53-58 the `WINDOWSNT` arm, C 60-70 the DLL entry-point loader and C 77-97 the
// three macros that declare and load one, and C 99-101 the `HAVE_HARFBUZZ` arm.
//
// Conditions: `tools/check-cond-eval.py src/w32common.h` -> `3 conditions, 0 unknown,
// 0 disagreements [VERIFIED]` - the guard, `WINDOWSNT` (C 53) and `HAVE_HARFBUZZ` (C 99).
// `docs/baseline/config/darwin-arm64.macros.tsv` records both feature macros as `undef`, so
// C 54-58 and C 100-101 are **dead text** on this target; they are carried under
// `#[cfg (feature = "windowsnt")]` / `#[cfg (feature = "harfbuzz")]` (PORTING.md P-51 names the
// non-frozen side), and the extractor's item list agrees - none of their names is required.
//
// Nothing on this target compiles the header: the frozen configuration's platform object
// variables are empty (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`,
// docs/baseline/frozen-build-sources.txt), so no unit `include!`s this mirror, and the
// declarations are deferred to the Windows target together with their definitions.
//
// **The platform half is spelled here, and the deferrals are named.**  C 24 gets `SYSTEM_INFO`,
// `OSVERSIONINFO`, `DWORD_PTR`, `HINSTANCE`, `LPCSTR`, `HMODULE` and `GetProcAddress` from
// `<windows.h>`, and the port has no mirror of that header (the `nt/inc/include/ms-w32.rs`
// convention: the platform's own names have no owner mirror).  `DWORD_PTR` is the one of them
// the macros below *cast through*, and PORTING.md 1.1 fixes its width (`uintptr_t`'s row:
// pointer-sized unsigned), so the mirror declares that name and the two macros keep C's cast
// visible.  The two records C 35-36 declares **by value** are a different case: no compiler on
// this target has a `<sysinfoapi.h>`, so no Rust spelling of them could be measured, and the
// mirror carries no invented layout - those two objects are deferred and left unchecked in the
// checklist.  C 77-97's three macros compose the names `W32_PFN_##func`/`fn_##func` by token
// pasting, which Rust has no counterpart for; they are deferred too, with the exact reason
// recorded in docs/ports/include-w32common.md.

// C 21-22: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const W32COMMON_H: bool = true;

/// C 24's `<windows.h>` declares `DWORD_PTR` as the pointer-sized unsigned integer
/// (`typedef ULONG_PTR DWORD_PTR;`), which is the width PORTING.md 1.1 gives `uintptr_t`; the two
/// macros below cast through it (C 26-27), so the name is declared here and spelled `usize`.
/// A Windows target that grows a windows.h mirror must retire this copy rather than declare it
/// twice.
pub type DWORD_PTR = usize;

/// C 26-27: the two alignments.  C's `~x` is Rust's `!x`; C's `(DWORD_PTR)` casts are the
/// `as DWORD_PTR` above, so every token of C's bodies other than the operator spelling survives.
/// C: #define ROUND_UP(p, align)   (((DWORD_PTR)(p) + (align)-1) & ~((DWORD_PTR)(align)-1))
macro_rules! ROUND_UP {
  ($p:expr, $align:expr) => { ((($p) as DWORD_PTR + $align - 1) & !($align - 1)) };
}

/// C 27.  The parameter names are C's (`p`, `align`); see the `ROUND_UP` note above.
/// C: #define ROUND_DOWN(p, align) ((DWORD_PTR)(p) & ~((DWORD_PTR)(align)-1))
macro_rules! ROUND_DOWN {
  ($p:expr, $align:expr) => { (($p) as DWORD_PTR & !($align - 1)) };
}

/// C 29-33: the five accessors.  C's `sysinfo_cache.dwPageSize` and its two siblings read the
/// record C 35 declares (deferred above); a `macro_rules!` body is not resolved until it is
/// expanded, so these compile exactly as C's text stands, and their tokens are C's.
/// C: #define get_page_size()			sysinfo_cache.dwPageSize
macro_rules! get_page_size { () => { sysinfo_cache.dwPageSize }; }
/// C 30: `#define get_allocation_unit()		sysinfo_cache.dwAllocationGranularity`
macro_rules! get_allocation_unit { () => { sysinfo_cache.dwAllocationGranularity }; }
/// C 31: `#define get_processor_type()		sysinfo_cache.dwProcessorType`
macro_rules! get_processor_type { () => { sysinfo_cache.dwProcessorType }; }
/// C 32: `#define get_w32_major_version()  	w32_major_version`
macro_rules! get_w32_major_version { () => { w32_major_version }; }
/// C 33: `#define get_w32_minor_version()  	w32_minor_version`
macro_rules! get_w32_minor_version { () => { w32_minor_version }; }

/// C 37-41: the page mask and the three version numbers.  **`sysinfo_cache` (C 35) and
/// `osinfo_cache` (C 36) are deferred**: C declares a `<sysinfoapi.h>`/`<winver.h>` record *by
/// value*, the port has no mirror of either, and no compiler on this target could measure them -
/// so no Rust spelling is written rather than an unmeasured one (see the file note above).
/// C: extern DWORD_PTR      syspage_mask;
/// C: extern int    	      w32_major_version;
/// C: extern int    	      w32_minor_version;
/// C: extern int    	      w32_build_number;
unsafe extern "C" {
  pub static mut syspage_mask: DWORD_PTR;
  pub static mut w32_major_version: c_int;
  pub static mut w32_minor_version: c_int;
  pub static mut w32_build_number: c_int;
}

/// C 43-46: `enum { OS_SUBTYPE_9X = 1, OS_SUBTYPE_NT };` - the header's anonymous enum, whose
/// second member is 2 (C's implicit successor).  C leaves the type to the enum; the port spells
/// it `c_int` (PORTING.md 1.3's shape for an anonymous enum).
pub const OS_SUBTYPE_9X: c_int = 1;
pub const OS_SUBTYPE_NT: c_int = 2;

/// C 48: which of the two the running system is.
/// C: extern int os_subtype;
unsafe extern "C" {
  pub static mut os_subtype: c_int;

  /// C 51: `extern void cache_system_info (void);` - fills `sysinfo_cache`/`osinfo_cache`
  /// (both deferred above) and the three versions.
  pub fn cache_system_info ();
}

/// C 53-58: the `WINDOWSNT` arm, dead text on this target (the feature macro is `undef`), so it
/// is carried under the feature that names the other side (PORTING.md P-51).  `intmax_t` is
/// PORTING.md 1.1's `i64`.
/// C: extern char * w32_version_string (void);
/// C: extern bool w32_valid_process_id (intmax_t);
/// C: # define VALID_PROCESS_ID(p) w32_valid_process_id(p)
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn w32_version_string () -> *mut c_char;
  pub fn w32_valid_process_id (pid: i64) -> bool;
}
#[cfg (feature = "windowsnt")]
macro_rules! VALID_PROCESS_ID { ($p:expr) => { w32_valid_process_id ($p) }; }

/// C 60: `typedef void (* VOIDFNPTR) (void);` - the pointer type the loader returns.  A C
/// function pointer is nullable, so it is an `Option` of an `extern "C"` fn (PORTING.md P-27,
/// the `nt/inc/include/ms-w32.rs` `malloc_fn` spelling).
pub type VOIDFNPTR = Option<unsafe extern "C" fn ()>;

/// C 69's callee.  `<windows.h>` declares `FARPROC GetProcAddress (HMODULE, LPCSTR)` - two
/// pointer arguments and a function-pointer result, which is the shape below; the port has no
/// windows.h mirror, so the declaration lives here (the file note above records the retirement
/// this copy needs if such a mirror is added).
/// C:   return (VOIDFNPTR) GetProcAddress (handle, fname);
unsafe extern "C" {
  pub fn GetProcAddress (hmodule: *mut c_void, lpProcName: *const c_char) -> VOIDFNPTR;
}

/// C 65-70: `INLINE VOIDFNPTR get_proc_addr (HINSTANCE, LPCSTR)` - the header's one definition.
/// C's cast `(VOIDFNPTR) GetProcAddress (…)` is the port's type ascription (a Rust `as` cast to
/// an `Option<fn>` type does not compile), and C's `HINSTANCE`/`LPCSTR` are the platform's
/// pointer pair - `HMODULE`/`LPCSTR` in the declaration above.
/// C: INLINE VOIDFNPTR get_proc_addr (HINSTANCE handle, LPCSTR fname)
/// C: {
/// C:   return (VOIDFNPTR) GetProcAddress (handle, fname);
/// C: }
#[inline(always)]
pub unsafe fn get_proc_addr (handle: *mut c_void, fname: *const c_char) -> VOIDFNPTR
{
  let addr: VOIDFNPTR = GetProcAddress (handle, fname);
  return addr;
}

/* C 72-97: the three DLL-loading macros.  **Deferred - not written here.**  C composes two names
   per expansion by token pasting and takes the function's parameter list, with any trailing
   attributes, as `__VA_ARGS__`:

     C 77-79  #define DEF_DLL_FN(type, func, ...)  typedef type (CDECL *W32_PFN_##func) __VA_ARGS__; static W32_PFN_##func fn_##func
     C 82-89  #define LOAD_DLL_FN(lib, func)       do { fn_##func = (W32_PFN_##func) get_proc_addr (lib, #func); if (!fn_##func) return false; } while (false)
     C 92-97  #define LOAD_DLL_FN_OPT(lib, func)   do { fn_##func = (W32_PFN_##func) get_proc_addr (lib, #func); } while (false)

   Rust has no token pasting, and `__VA_ARGS__`'s trailing-attribute form has no `macro_rules!`
   counterpart, so the Rust shape of the three is an open design decision for the Windows target
   that first expands them (PORTING.md P-22) rather than something to invent here.  Their
   checklist boxes stay unchecked, and docs/ports/include-w32common.md records the
   docs/ports/excluded-macros.txt rows (kind=deferred) they need.  C's text is kept above so a
   reader sees what the mirror is not carrying.  */

/// C 35-36: the two caches the getters above read (`get_page_size ()` is
/// `sysinfo_cache.dwPageSize`).  Their types come from `<windows.h>`, which this target has not
/// got, so they are carried under the feature that names the other side (PORTING.md P-51) with
/// C's own type names - that target's `<windows.h>` supplies them, and the items are stripped
/// before name resolution here.
/// C: extern SYSTEM_INFO sysinfo_cache;
/// C: extern OSVERSIONINFO osinfo_cache;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut sysinfo_cache: SYSTEM_INFO;
  pub static mut osinfo_cache: OSVERSIONINFO;
}

/// C 99-101: the `HAVE_HARFBUZZ` arm, dead text on this target (the feature macro is `undef`),
/// carried under the feature that names the other side (PORTING.md P-51).  `HMODULE` is the
/// platform's module handle, a pointer at the ABI.
/// C: extern bool hbfont_init_w32_funcs (HMODULE);
#[cfg (feature = "harfbuzz")]
unsafe extern "C" {
  pub fn hbfont_init_w32_funcs (hmodule: *mut c_void) -> bool;
}
