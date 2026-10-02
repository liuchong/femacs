# emacs/lib/md5-stream.c (134 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] md5_stream  (line 60)

## Macros defined — 7
- [x] BLOCKSIZE  (line 51, body 1b3bbc30ac75)
- [x] WORDS_BIGENDIAN  (line 40, body 356a192b7913)
- [x] md5_finish_ctx  (line 47, body 3781656f54c7)
- [x] md5_init_ctx  (line 44, body dc8a2e11f2fe)
- [x] md5_process_block  (line 45, body d5bd2fbe7c35)
- [x] md5_process_bytes  (line 46, body b08fdd23eff4)
- [x] md5_stream  (line 48, body d4745ff2b574)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- `USE_UNLOCKED_IO`, `GNULIB_AF_ALG` and `_LIBC` are all undefined in the frozen
  configuration, so C 29-66's three arms are dead: the `_LIBC` rename block is kept as cfg'd
  items (including the `WORDS_BIGENDIAN` constant and the `md5_stream` → `__md5_stream`
  rename, which the ledger compares), and the af_alg arm's tokens are recorded in
  docs/ports/macro-body-notes.tsv (this target has no "af_alg.h").
- The two `goto process_partial_block` jumps become `break 'process_partial_block` out of a
  labelled block (PORTING.md 2.2), with `sum` declared outside so the partial-block code reads
  it exactly as C does.
- `struct md5_ctx ctx;` is written with `std::mem::zeroed ()` (C leaves it uninitialized and
  `md5_init_ctx` fills it).
- No call-relationship differences (D7): `feof`, `fread`, `ferror`, `md5_process_block`,
  `md5_process_bytes`, `md5_finish_ctx`, `free` appear in C's order.
