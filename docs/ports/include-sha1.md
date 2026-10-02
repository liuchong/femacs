# emacs/lib/sha1.h (136 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] GL_OPENSSL_NAME  (line 58, body 356a192b7913)
- [x] OPENSSL_API_COMPAT  (line 32, body 8530ea34c3aa)
- [x] SHA1_DIGEST_SIZE  (line 55, body 91032ad7bbcb)
- [x] SHA1_H  (line 20, body 356a192b7913)

## Types and their fields — 1
- [x] sha1_ctx: uint32_t A, uint32_t B, uint32_t C, uint32_t D, uint32_t E, uint32_t total, uint32_t buflen, uint32_t buffer

## File-scope objects — 0

## Decisions

- Header mirror (PORTING.md 0.1): struct + constants + declarations, with the declarations
  in a glob-imported `decls` module so the unit's own definitions shadow them
  (PORTING.md 0.1-10).  `unsafe extern "C"` declarations only.
- C's `#if HAVE_OPENSSL_SHA1` arm is dead (the macro is not in the frozen
  configuration) and is kept as text under `#[cfg (feature = "openssl_sha1")]` (P-51);
  its OpenSSL headers have no mirror.  `OPENSSL_API_COMPAT`'s `L` suffix is recorded in
  docs/ports/macro-body-notes.tsv, and the `_GL_U64_INLINE`-class spelling macros in
  docs/ports/excluded-macros.txt.
- The `#if !_GL_CONFIG_H_INCLUDED` guard is the `const _: () = assert! (…)` the
  timespec.rs mirror established; `size_t` is the prelude's `usize` (the port's rule).
