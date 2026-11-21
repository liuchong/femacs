// femacs: port of src/dynlib.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
/* emacs/src/dynlib.c (@ 340 lines) is on the verified list: WINDOWSNT is undefined and
   HAVE_UNISTD_H is 1, so C 33-266's native-Windows arm (the `LoadLibrary`/`GetProcAddress`
   family, `os_subtype`, `w32common.h` and `w32.h`) is dead by measurement and the live text is
   C 268-345, the POSIX arm.  Inside it, HAVE_NATIVE_COMP is undefined (so `dynlib_open_for_eln`
   and `dynlib_close` are dead) and HAVE_DLADDR is undefined (so `dynlib_addr`'s body is just
   the two NULL stores, and the `Dl_info`/`dladdr` text is dead).  C 333-335's
   `#if !HAVE_DLFUNC / # define dlfunc dynlib_sym` is live (HAVE_DLFUNC is undefined), so the
   port keeps the macro and `dynlib_func` calls through it.  C 29's "dynlib.h" is the new dynlib
   mirror; C 274's <dlfcn.h> is not mirrored (the four entries the live arm names are declared
   in-unit, the getgroups.c shape).  */
use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};
// [U1] config.rs provided by common
// [U1] ../lib/include/gnulib_config.rs imported via common
// [U1] include/dynlib.rs imported via common

/* C 274's <dlfcn.h>: the four entries the POSIX arm names.  */
unsafe extern "C" {
  pub fn dlopen (file: *const c_char, mode: c_int) -> *mut c_void;
  pub fn dlsym (handle: *mut c_void, name: *const c_char) -> *mut c_void;
  pub fn dlclose (handle: *mut c_void) -> c_int;
  pub fn dlerror () -> *mut c_char;
}

#[cfg(feature = "have_dladdr")]
#[repr(C)]
struct Dl_info {
  dli_fname: *const c_char,
  dli_fbase: *mut c_void,
  dli_sname: *const c_char,
  dli_saddr: *mut c_void,
}
#[cfg(feature = "have_dladdr")]
unsafe extern "C" {
  fn dladdr (address: *const c_void, info: *mut Dl_info) -> c_int;
}
#[cfg(all(feature = "have_dladdr", target_pointer_width = "64"))]
const _: () = assert!(core::mem::size_of::<Dl_info>() == 32);

/* <dlfcn.h>'s RTLD_* values (measured on the frozen target: RTLD_LAZY = 1, RTLD_NOW = 2,
   RTLD_GLOBAL = 8, RTLD_LOCAL = 4).  */
pub const RTLD_LAZY: c_int = 1;
pub const RTLD_NOW: c_int = 2;
pub const RTLD_LOCAL: c_int = 4;
pub const RTLD_GLOBAL: c_int = 8;

#[cfg(feature = "femacs_gc_generational")]
unsafe extern "C" { fn femacs_gc_unknown_producer_loaded(); }

/* POSIX systems.  */

/* C 277-280's `dynlib_handle_ptr dynlib_open (const char *path)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dynlib_open (path: *const c_char) -> dynlib_handle_ptr
{
  // Module constructors may run inside dlopen, before module_init. They are
  // not compiled with the candidate's cons write barrier. Latch before entry,
  // including a failed load; returning cannot prove all aliases were released.
  #[cfg(feature = "femacs_gc_generational")]
  femacs_gc_unknown_producer_loaded();
  return dlopen (path, RTLD_LAZY | RTLD_GLOBAL);
}

/* P-346: carry the original native-only POSIX arm when actually selected. */
#[cfg(feature = "have_native_comp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dynlib_open_for_eln (path: *const c_char) -> dynlib_handle_ptr
{
  dlopen(path, RTLD_LAZY)
}

/* C 290-294's `void *dynlib_sym (dynlib_handle_ptr h, const char *sym)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dynlib_sym (h: dynlib_handle_ptr, sym: *const c_char) -> *mut c_void
{
  return dlsym (h, sym);
}

/* C 296-311's `void dynlib_addr (void (*funcptr) (void), const char **path,
   const char **sym)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dynlib_addr (funcptr: Option<unsafe extern "C" fn ()>,
                                      path: *mut *const c_char, sym: *mut *const c_char)
{
  *path = std::ptr::null ();
  *sym = std::ptr::null ();
  #[cfg(feature = "have_dladdr")]
  {
    let address = match funcptr {
      Some(function) => function as *const () as *const c_void,
      None => core::ptr::null(),
    };
    let mut info: Dl_info = core::mem::zeroed();
    if dladdr(address, &raw mut info) != 0 && !info.dli_fname.is_null() && !info.dli_sname.is_null() {
      *path = info.dli_fname;
      *sym = info.dli_sname;
    }
  }
  #[cfg(not(feature = "have_dladdr"))]
  let _ = funcptr;
}

/* C 313-317's `const char *dynlib_error (void)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dynlib_error () -> *const c_char
{
  return dlerror ();
}

#[cfg(feature = "have_native_comp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dynlib_close (handle: dynlib_handle_ptr) -> c_int
{
  // Original wrapper returns boolean success, unlike raw dlclose.
  (dlclose(handle) == 0) as c_int
}

/* C 333-335's `#if !HAVE_DLFUNC / # define dlfunc dynlib_sym / #endif`, the live arm.  */
macro_rules! dlfunc { ($h:expr, $sym:expr) => { dynlib_sym ($h, $sym) }; }

/* C 337-340's `dynlib_function_ptr dynlib_func (dynlib_handle_ptr h, const char *sym)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dynlib_func (h: dynlib_handle_ptr, sym: *const c_char)
  -> dynlib_function_ptr
{
  /* C's `(dynlib_function_ptr) dlfunc (h, sym)` is the pointer cast the port spells with the
     same type name.  */
  let f: dynlib_function_ptr = std::mem::transmute (dlfunc! (h, sym));
  return f;
}
