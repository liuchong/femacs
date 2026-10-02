// femacs: mirror of the frozen target's <wchar.h> (system header, P-26/P-50).
//
// regex_internal.h 113-115 takes the `_REGEX_AVOID_UCHAR_H` arm (the frozen macro
// table has it at 1), so the port's wide characters are C's `wchar_t`/`wint_t`
// rather than gnulib's `char32_t`, and the `__mbrtowc`/`__wcrtomb` renames at
// regex_internal.h 151-152 point at the wide functions.


/// <wchar.h>'s signed wide character type (the frozen target: `int`).
pub type wchar_t = c_int;
/// <wchar.h>'s unsigned wide-character / wint_t type (the frozen target: `unsigned int`).
pub type wint_t = u32;

/// <wchar.h>'s `mbstate_t`: an opaque conversion state.  The frozen target's glibc
/// spells it as a struct holding an `int` count and a 4-byte array (8 bytes total);
/// the port only ever passes its address to `mbrtowc`/`wcrtomb`/`mbsinit`, so the
/// mirror keeps the same size and ABI, not the glibc-internal field names.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct mbstate_t {
  /// C: `int __count;` (the glibc-internal name `__count` is not spelled by the ported
  /// code, which only zeroes the object and passes it on).
  pub count: c_int,
  /// C: `union { unsigned int __wch; char __wchb[4]; } __value;`
  pub value: [u8; 4],
}

/// <wchar.h>'s end-of-file wide character.
pub const WEOF: wint_t = 0xffff_ffff;

unsafe extern "C" {
  /* size_t mbrtowc (wchar_t *pwc, const char *s, size_t n, mbstate_t *ps);  */
  pub fn mbrtowc (pwc: *mut wchar_t, s: *const std::ffi::c_char, n: usize,
                  ps: *mut mbstate_t) -> usize;
  /* size_t wcrtomb (char *s, wchar_t wc, mbstate_t *ps);  */
  pub fn wcrtomb (s: *mut std::ffi::c_char, wc: wchar_t, ps: *mut mbstate_t) -> usize;
  /* int mbsinit (const mbstate_t *ps);  */
  pub fn mbsinit (ps: *const mbstate_t) -> c_int;
}
