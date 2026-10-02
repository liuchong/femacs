# emacs/lib/sha256.h (142 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] GL_OPENSSL_NAME  (line 60, body dd7c1a3d9d56)
- [x] OPENSSL_API_COMPAT  (line 31, body 8530ea34c3aa)
- [x] SHA256_H  (line 19, body 356a192b7913)

## Types and their fields — 1
- [x] sha256_ctx: uint32_t state, uint32_t total, size_t buflen, uint32_t buffer

## File-scope objects — 0

## Decisions

- Header mirror (PORTING.md 0.1): struct + constants + declarations, with the declarations
  in a glob-imported `decls` module so the unit's own definitions shadow them
  (PORTING.md 0.1-10).  `unsafe extern "C"` declarations only.
- C's `#if HAVE_OPENSSL_SHA256` arm is dead (the macro is not in the frozen
  configuration) and is kept as text under `#[cfg (feature = "openssl_sha256")]` (P-51);
  its OpenSSL headers have no mirror.  `OPENSSL_API_COMPAT`'s `L` suffix is recorded in
  docs/ports/macro-body-notes.tsv, and the `_GL_U64_INLINE`-class spelling macros in
  docs/ports/excluded-macros.txt.
- The `#if !_GL_CONFIG_H_INCLUDED` guard is the `const _: () = assert! (…)` the
  timespec.rs mirror established; `size_t` is the prelude's `usize` (the port's rule).
