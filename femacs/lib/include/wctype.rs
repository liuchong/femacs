// femacs: mirror of the frozen target's <wctype.h> (system header, P-26).
//
// regex_internal.h 145-149 renames `__wctype`/`__iswalnum`/`__iswctype`/`__towlower`/
// `__towupper` onto these (the `_REGEX_AVOID_UCHAR_H` arm); `wctype_t` at
// regex_internal.h 331 types `re_charset_t.char_classes`.  `wint_t`/`wchar_t` come
// from the <wchar.h> mirror the same unit includes (C's <wctype.h> gets them there too).


/// <wctype.h>'s character-class handle (the frozen target: `unsigned long`).
pub type wctype_t = u64;

unsafe extern "C" {
  /* wctype_t wctype (const char *property);  */
  pub fn wctype (property: *const std::ffi::c_char) -> wctype_t;
  /* int iswalnum (wint_t wc);  */
  pub fn iswalnum (wc: wint_t) -> c_int;
  /* int iswctype (wint_t wc, wctype_t type);  */
  pub fn iswctype (wc: wint_t, type_: wctype_t) -> c_int;
  /* wint_t towlower (wint_t wc);  */
  pub fn towlower (wc: wint_t) -> wint_t;
  /* wint_t towupper (wint_t wc);  */
  pub fn towupper (wc: wint_t) -> wint_t;
}
