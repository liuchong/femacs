# emacs/lib/sha1.c (362 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 7
- [x] sha1_init_ctx  (line 53)
- [x] set_uint32  (line 69, static)
- [x] sha1_read_ctx  (line 77)
- [x] sha1_finish_ctx  (line 92)
- [x] sha1_buffer  (line 120)
- [x] sha1_process_bytes  (line 135)
- [x] sha1_process_block  (line 223)

## Macros defined — 14
- [x] F1  (line 213, body e5f6a15fc3ac)
- [x] F2  (line 214, body 288547a9c667)
- [x] F3  (line 215, body b618a02dde65)
- [x] F4  (line 216, body 288547a9c667)
- [x] GL_OPENSSL_INLINE  (line 28, body a9f37094e6e8)
- [x] K1  (line 207, body c360311d5408)
- [x] K2  (line 208, body b1de5d68908c)
- [x] K3  (line 209, body 5a34df333088)
- [x] K4  (line 210, body fd5207cf03d2)
- [x] M  (line 245, body d8f74ca9d062)
- [x] R  (line 249, body dc41332fc1d2)
- [x] SWAP  (line 39, body ffbc50454325)
- [x] UNALIGNED_P  (line 168, body e82ee352ed82)
- [x] rol  (line 243, body 3b5dca057b64)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **`#if ! HAVE_OPENSSL_SHA1` is the live arm** (`HAVE_OPENSSL_SHA1` is not in the frozen
  macro table); C 27-29's qualifier is dead text under `#[cfg (feature = "openssl_sha1")]`.
- **`SWAP`'s arms are the reverse of md5.c's**: the big-endian arm is C's `(n)` and the
  live little-endian arm is `bswap_32 (n)`, which Rust spells `swap_bytes` (PORTING.md
  2.6); the pair is `#[cfg]`-split and C's macro body is not compared (two definitions).
- **`rol`, `M` and `R` are defined where C defines them** (inside `sha1_process_block`,
  before the loop), so no hoist was needed.  `R`'s `A`..`E` are the caller's locals, which
  the port passes as the macro's first five arguments - C's call sites pass them by name
  too, so the invocations are C's.  C's loop-local `tm` is dead state between `M`
  expansions (assigned before every read), so the port's `M!` has its own hygienic `tm`
  (C's `uint32_t tm;` statement is therefore not needed; the token stays in `M`'s body).
  `R`'s `while (0)` is a Rust block, so C's guard token `0` is recorded in
  docs/ports/macro-body-notes.tsv.
- **C's unsigned `+=`/`+` are `wrapping_add`** (the rounds and the byte counters rely on
  wrap-around; PORTING.md 2.6), and C's `a = ctx->A += a` (one statement) is a block
  expression.
- **`sha1.h`'s mirror** carries `struct sha1_ctx`, `SHA1_DIGEST_SIZE`, the dead OpenSSL
  constants and the declarations (glob-imported `decls`, PORTING.md 0.1-10).
- **docs rows**: the `0` guard token (macro-body-notes.tsv) and the line count
  (line-drift-tolerance.tsv).
