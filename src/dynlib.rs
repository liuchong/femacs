// femacs: port of src/dynlib.c @ fd7afced403.
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
include!("include/config.rs");
include!("../lib/include/gnulib_config.rs");
include!("include/dynlib.rs");

/* C 274's <dlfcn.h>: the four entries the POSIX arm names.  */
unsafe extern "C" {
  pub fn dlopen (file: *const c_char, mode: c_int) -> *mut c_void;
  pub fn dlsym (handle: *mut c_void, name: *const c_char) -> *mut c_void;
  pub fn dlclose (handle: *mut c_void) -> c_int;
  pub fn dlerror () -> *mut c_char;
}

/* <dlfcn.h>'s RTLD_* values (measured on the frozen target: RTLD_LAZY = 1, RTLD_NOW = 2,
   RTLD_GLOBAL = 8, RTLD_LOCAL = 4).  */
pub const RTLD_LAZY: c_int = 1;
pub const RTLD_NOW: c_int = 2;
pub const RTLD_LOCAL: c_int = 4;
pub const RTLD_GLOBAL: c_int = 8;

/* POSIX systems.  */

/* C 277-280's `dynlib_handle_ptr dynlib_open (const char *path)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dynlib_open (path: *const c_char) -> dynlib_handle_ptr
{
  return dlopen (path, RTLD_LAZY | RTLD_GLOBAL);
}

/* C 282-288's `dynlib_open_for_eln` is behind HAVE_NATIVE_COMP, which is undefined here.  */

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
  /* C 301-311's `#ifdef HAVE_DLADDR` arm is dead on this target (the macro the frozen
     configuration decides is undefined), so the `Dl_info`/`dladdr` text is not carried.  */
  let _ = funcptr;
}

/* C 313-317's `const char *dynlib_error (void)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dynlib_error () -> *const c_char
{
  return dlerror ();
}

/* C 319-326's `dynlib_close` is behind HAVE_NATIVE_COMP, which is undefined here.  */

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
