# emacs/lib/md5.c (394 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 7
- [x] md5_init_ctx  (line 66)
- [x] set_uint32  (line 81, static)
- [x] md5_read_ctx  (line 89)
- [x] md5_finish_ctx  (line 103)
- [x] md5_buffer  (line 131)
- [x] md5_process_bytes  (line 147)
- [x] md5_process_block  (line 229)

## Macros defined — 16
- [x] CYCLIC  (line 276, body c0cf0216555a)
- [x] FF  (line 220, body 02a22ef3586b)
- [x] FG  (line 221, body 96bba341462e)
- [x] FH  (line 222, body 854f3daf6501)
- [x] FI  (line 223, body 400e01aff242)
- [x] GL_OPENSSL_INLINE  (line 26, body a9f37094e6e8)
- [x] OP  (line 310, body 939ec3bb12b2)
- [x] SWAP  (line 53, body dbd6c89c42dc)
- [x] UNALIGNED_P  (line 179, body e82ee352ed82)
- [x] WORDS_BIGENDIAN  (line 37, body 356a192b7913)
- [x] md5_buffer  (line 46, body f3cad0b1a5df)
- [x] md5_finish_ctx  (line 44, body 3781656f54c7)
- [x] md5_init_ctx  (line 41, body dc8a2e11f2fe)
- [x] md5_process_block  (line 42, body d5bd2fbe7c35)
- [x] md5_process_bytes  (line 43, body b08fdd23eff4)
- [x] md5_read_ctx  (line 45, body 185abf0dedba)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **`#if ! HAVE_OPENSSL_MD5` is the live arm.**  `HAVE_OPENSSL_MD5` is not in the frozen
  macro table, so C's `#if` reads 0 and C 56-387 - the whole gnulib implementation - is
  what the port carries; C 24-27's qualifier is dead text under
  `#[cfg (feature = "openssl_md5")]` (P-51).
- **C 33-46's `#ifdef _LIBC` arm is dead** (`_LIBC` is undefined).  `WORDS_BIGENDIAN` and
  the `#ifdef WORDS_BIGENDIAN` half of `SWAP` are kept under
  `#[cfg (feature = "words_bigendian")]` with a `bswap_32` declaration (P-51); the six
  renames are recorded in docs/ports/excluded-macros.txt (kind=dead-text) - in this
  configuration the port's definitions keep C's source names, which *are* the symbols
  (`#[unsafe(no_mangle)]`, no `export_name` needed, PORTING.md 10.13).
- **`md5.h` is its own mirror** (`femacs/lib/include/md5.rs`): `struct md5_ctx` (7 fields),
  `MD5_DIGEST_SIZE`/`MD5_BLOCK_SIZE`, the `__md5_*` renames and the declarations, in a
  glob-imported `decls` module so this unit's `fn md5_*` shadows them (PORTING.md 0.1-10).
- **`fillbuf`**: C's one-line `{ 0x80, 0 /* , 0, 0, … */ }` initializer is a Rust const
  block that sets `0x80` and leaves the rest zero.
- **`OP` is defined twice in C** (C 264-272 for round 1, C 310-317 for rounds 2-4, with
  `#undef OP` between them, both inside the loop body).  A `macro_rules!` cannot be
  redefined in one scope, so the port has one macro with C's two arms dispatched by
  arity, defined in the block C defines it in.  C's `cwp`, `words` and `correct_words` are
  the caller's locals, which a Rust macro cannot name (hygiene), so they are passed as
  trailing arguments - the port's own deviation.  `CYCLIC` is defined before `OP` (C
  defines it at C 276, after the first `OP`, but Rust's macro scope is textual).
- **`UNALIGNED_P`** stays inside the `if (len >= 64)` block C defines it in; C 178's
  `#if !(_STRING_ARCH_unaligned || _STRING_INLINE_unaligned)` arm is *live* for this
  target (neither macro is defined), so the unaligned-head loop is compiled in.
- **C's unsigned modular arithmetic is `wrapping_add`**, never `+` - the rounds, the byte
  counters and the length words all rely on wrap-around and the port's builds carry debug
  assertions (the mini-gmp precedent, PORTING.md 2.6).  Shifts stay C's `<<`/`>>`.
- **C 83's `set_uint32` is `static`**: the port's helper is a private `unsafe fn`.
- **docs rows**: `OPENSSL_API_COMPAT`'s `L` suffix (macro-body-notes.tsv), the six dead
  renames (excluded-macros.txt), the line count (line-drift-tolerance.tsv).
