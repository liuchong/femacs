# emacs/lib/sha3.h (132 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] SHA3_H  (line 20, body 356a192b7913)

## Types and their fields — 1
- [x] sha3_ctx: EVP_MD_CTX* evp_ctx, u64 state, uint8_t buffer, size_t buflen, size_t digestlen, size_t blocklen

## File-scope objects — 0

## Decisions

- Header mirror (PORTING.md 0.1): struct + constants + declarations, with the declarations
  in a glob-imported `decls` module so the unit's own definitions shadow them
  (PORTING.md 0.1-10).  `unsafe extern "C"` declarations only.
- C's `#if HAVE_OPENSSL_SHA3` arm is dead (the macro is not in the frozen
  configuration) and is kept as text under `#[cfg (feature = "openssl_sha3")]` (P-51);
  its OpenSSL headers have no mirror.  `OPENSSL_API_COMPAT`'s `L` suffix is recorded in
  docs/ports/macro-body-notes.tsv, and the `_GL_U64_INLINE`-class spelling macros in
  docs/ports/excluded-macros.txt.
- The `#if !_GL_CONFIG_H_INCLUDED` guard is the `const _: () = assert! (…)` the
  timespec.rs mirror established; `size_t` is the prelude's `usize` (the port's rule).
