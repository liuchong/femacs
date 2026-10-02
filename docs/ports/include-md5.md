# emacs/lib/md5.h (181 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 14
- [x] GL_OPENSSL_NAME  (line 95, body ac3478d69a3c)
- [x] MD5_BLOCK_SIZE  (line 53, body c66c65175fec)
- [x] MD5_DIGEST_SIZE  (line 52, body 1574bddb75c7)
- [x] OPENSSL_API_COMPAT  (line 33, body 8530ea34c3aa)
- [x] _MD5_H  (line 21, body 356a192b7913)
- [x] __GNUC_PREREQ  (line 64, body b6589fc6ab0d)
- [x] __THROW  (line 76, body da39a3ee5e6b)
- [x] __md5_buffer  (line 81, body 3e65d9c7516c)
- [x] __md5_finish_ctx  (line 82, body 25a2a4b55ca5)
- [x] __md5_init_ctx  (line 83, body cca3edaee830)
- [x] __md5_process_block  (line 84, body 2ed1d15f9334)
- [x] __md5_process_bytes  (line 85, body f0baed9a05ef)
- [x] __md5_read_ctx  (line 86, body d529aa46c5c8)
- [x] __md5_stream  (line 87, body 4b3563d6f586)

## Types and their fields — 1
- [x] md5_ctx: uint32_t A, uint32_t B, uint32_t C, uint32_t D, uint32_t total, uint32_t buflen, uint32_t buffer

## File-scope objects — 0

## Decisions

- Header mirror (PORTING.md 0.1): struct + constants + declarations, with the declarations
  in a glob-imported `decls` module so the unit's own definitions shadow them
  (PORTING.md 0.1-10).  `unsafe extern "C"` declarations only.
- C's `#if HAVE_OPENSSL_MD5` arm is dead (the macro is not in the frozen
  configuration) and is kept as text under `#[cfg (feature = "openssl_md5")]` (P-51);
  its OpenSSL headers have no mirror.  `OPENSSL_API_COMPAT`'s `L` suffix is recorded in
  docs/ports/macro-body-notes.tsv, and the `_GL_U64_INLINE`-class spelling macros in
  docs/ports/excluded-macros.txt.
- The `#if !_GL_CONFIG_H_INCLUDED` guard is the `const _: () = assert! (…)` the
  timespec.rs mirror established; `size_t` is the prelude's `usize` (the port's rule).
