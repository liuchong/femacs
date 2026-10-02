// femacs: mirror of the frozen target's <langinfo.h> (system header, P-26).
//
// regex_internal.c 208 (the live arm) asks `nl_langinfo (CODESET)` for the locale's
// encoding; the frozen target's `CODESET` is the `nl_item` of that name.


/// <langinfo.h>'s `nl_item` (the frozen target: `int`).
pub type nl_item = i32;

/// The frozen target's `CODESET` (measured from the reference's clang preprocessor;
/// glibc's `_NL_CTYPE_CODESET_NAME` = 14).
pub const CODESET: nl_item = 14;

unsafe extern "C" {
  /* char *nl_langinfo (nl_item item);  */
  pub fn nl_langinfo (item: nl_item) -> *mut c_char;
}
