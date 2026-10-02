# emacs/lib/u64.h (236 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 14
- [x] u64hilo  (line 85)
- [x] u64lo  (line 95)
- [x] u64getlo  (line 105)
- [x] u64size  (line 112)
- [x] u64not  (line 122)
- [x] u64lt  (line 132)
- [x] u64and  (line 139)
- [x] u64or  (line 149)
- [x] u64xor  (line 159)
- [x] u64plus  (line 169)
- [x] u64shl  (line 179)
- [x] u64shr  (line 197)
- [x] u64bswap  (line 215)
- [x] u64rol  (line 224)

## Macros defined — 17
- [x] U64_H  (line 21, body 356a192b7913)
- [x] _GL_U64_INLINE  (line 36, body a960df3ab827)
- [x] _GL_U64_MASK32  (line 66, body d83d1f43650a)
- [x] u64and  (line 56, body 972147ba8a7d)
- [x] u64bswap  (line 62, body 16f4748c6213)
- [x] u64getlo  (line 52, body 0d89011b6f3b)
- [x] u64hilo  (line 49, body 9d329e4f0320)
- [x] u64init  (line 79, body 961b108c1a68)
- [x] u64lo  (line 51, body fc95cbd4f673)
- [x] u64lt  (line 55, body 9334246808cf)
- [x] u64not  (line 54, body 77ada68d059f)
- [x] u64or  (line 57, body 88b66555f6f6)
- [x] u64plus  (line 59, body 8b6b47d0b958)
- [x] u64shl  (line 60, body 15e56cdcf69d)
- [x] u64shr  (line 61, body 3c88b36f9104)
- [x] u64size  (line 53, body 785783956ed2)
- [x] u64xor  (line 58, body efd3c21dede7)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- Header mirror (PORTING.md 0.1).  C 44's `#if defined UINT64_MAX && INT_MAX < UINT64_MAX`
  arm is live on the frozen target, so `u64` is `core::primitive::u64` (the stdint
  mirror's `uint64_t` would be a self-alias cycle) and the fourteen operations are
  `macro_rules!` with C's bodies; C 64-236's `#else` arm defines the *same* operations as
  `_GL_U64_INLINE` functions and a `struct u64`, which the frozen target never compiles -
  recorded here and in docs/ports/excluded-macros.txt (kind=dead-text) rather than carried
  (the `data_start` disposition).
- **`u64rol` is carried unconditionally**: the reference defines it only in the `#else`
  arm, yet sha512.c C 345-348 and sha3.c C 205-306 call it in this configuration, so the
  C reference needs it too; the port keeps C's body so those two units compile.
- `_GL_U64_INLINE`, `_GL_U64_MASK32` and the `ul`-suffixed mask token are recorded in the
  two ledger files (excluded-macros.txt / macro-body-notes.tsv).
