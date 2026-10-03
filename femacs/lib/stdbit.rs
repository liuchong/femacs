// femacs: port of lib/stdbit.c @ fd7afced403 (gnulib): C's LGPL header, the macro
// `_GL_STDBIT_INLINE = _GL_EXTERN_INLINE` and two includes - the C99 inline model's
// external-definition unit (PORTING.md 10.6).  The header's functions are Rust items in
// lib/include/stdbit.rs, so the macro is what survives.
include!("include/gnulib_config.rs");

/* lib/stdbit.c: the qualifier this translation unit uses (PORTING.md 10.6).  */
pub const _GL_STDBIT_INLINE: () = _GL_EXTERN_INLINE;
