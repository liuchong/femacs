# emacs/lib/sha3.c (442 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 12
- [x] sha3_free_ctx  (line 69)
- [x] set_uint64  (line 78, static)
- [x] sha3_read_ctx  (line 84)
- [x] sha3_conclude_ctx  (line 107, static)
- [x] sha3_finish_ctx  (line 116)
- [x] sha3_process_bytes  (line 139)
- [x] sha3_process_block  (line 173)
- [x] sha3_free_ctx  (line 360)
- [x] sha3_read_ctx  (line 372)
- [x] sha3_finish_ctx  (line 392)
- [x] sha3_process_bytes  (line 423)
- [x] sha3_process_block  (line 436)

## Macros defined — 3
- [x] DEFINE_SHA3_BUFFER  (line 403, body 208c10e454be)
- [x] DEFINE_SHA3_INIT_CTX  (line 339, body cc31427b9166)
- [x] SWAP  (line 32, body dbd6c89c42dc)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **`#if ! HAVE_OPENSSL_SHA3` is the live arm.**  C 334-441's `#else` arm is the EVP
  implementation of the same functions; the OpenSSL EVP API has no mirror in the port and
  is not part of the frozen target's configuration, so that text is *not* carried (the
  `md5.h` OpenSSL arm's disposition) and this ledger records it.  Its function names are
  the live arm's, so nothing is missing from the inventory.
- **C 39-41's `HAVE_OPENSSL_SHA3` include** and the `evp_ctx` field: the field is kept in
  `sha3.h`'s mirror under `#[cfg (feature = "openssl_sha3")]`, which is what the header
  check's member-set comparison sees (PORTING.md P-24), while the live configuration
  compiles C 57-61's five fields.
- **`DEFINE_SHA3_INIT_CTX(SIZE)` and `DEFINE_SHA3_BUFFER(SIZE)`** paste
  `sha3_##SIZE##_*`; Rust has no token pasting, so the port's macros take the generated
  names (and the size constants) as arguments and the four call sites name them - C's
  numbers are the `SHA3_{224,256,384,512}_{DIGEST,BLOCK}_SIZE` constants those call sites
  pass.  Because the C functions are macro-generated they are not in cinventory's function
  list, so the port emits their symbols from *inside* the macros
  (`#[unsafe(no_mangle)]`, P-52's symbol rule) and the gate's `no_mangle`-for-non-static-
  functions rule - which reads the file text - sees the macro, not the expansions.
- **`SWAP`'s arms** are this file's: big-endian `u64bswap`, live little-endian `(n)`.
- **Theta/Rho/Pi/Chi/Iota** keep C's statement order and indexing; the `Chi` loop is C's
  `for (int j = 0; j < 25; j += 5)` as a `while`.  C's `ctx->buffer[ctx->buflen++] = 0x06;`
  (one statement) is the two statements the index-then-increment needs, with a comment.
- **`sha3.h`'s mirror** carries the struct, the eight size enums and the declarations.
