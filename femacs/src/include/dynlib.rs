// femacs: mirror of src/dynlib.h @ fd7afced403 ; PORTING.md 0.1.
//
// The header is one typedef, one function-pointer typedef, the prototypes and the
// `ATTRIBUTE_MAY_ALIAS` spellings (attributes have no counterpart, PORTING.md 0.3).  C 20's
// `<attribute.h>` include is the excluded-macro record's business.

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const DYNLIB_H: bool = true;

/// `typedef void *dynlib_handle_ptr;`
pub type dynlib_handle_ptr = *mut c_void;

/// `typedef void (ATTRIBUTE_MAY_ALIAS *dynlib_function_ptr) (void);`
pub type dynlib_function_ptr = Option<unsafe extern "C" fn ()>;

pub mod dynlib_decls {
  use super::*;

  unsafe extern "C" {
    pub fn dynlib_open (path: *const c_char) -> dynlib_handle_ptr;
    pub fn dynlib_open_for_eln (path: *const c_char) -> dynlib_handle_ptr;
    pub fn dynlib_close (h: dynlib_handle_ptr) -> c_int;
    pub fn dynlib_error () -> *const c_char;
    pub fn dynlib_sym (h: dynlib_handle_ptr, sym: *const c_char) -> *mut c_void;
    pub fn dynlib_func (h: dynlib_handle_ptr, sym: *const c_char) -> dynlib_function_ptr;
    pub fn dynlib_addr (ptr: Option<unsafe extern "C" fn ()>, file: *mut *const c_char,
                        sym: *mut *const c_char);
  }
}
use dynlib_decls::*;
