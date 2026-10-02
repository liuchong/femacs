# emacs/lib/md5.h (181 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 14
- [ ] GL_OPENSSL_NAME  (line 95, body ac3478d69a3c)
- [ ] MD5_BLOCK_SIZE  (line 53, body c66c65175fec)
- [ ] MD5_DIGEST_SIZE  (line 52, body 1574bddb75c7)
- [ ] OPENSSL_API_COMPAT  (line 33, body 8530ea34c3aa)
- [ ] _MD5_H  (line 21, body 356a192b7913)
- [ ] __GNUC_PREREQ  (line 64, body b6589fc6ab0d)
- [ ] __THROW  (line 76, body da39a3ee5e6b)
- [ ] __md5_buffer  (line 81, body 3e65d9c7516c)
- [ ] __md5_finish_ctx  (line 82, body 25a2a4b55ca5)
- [ ] __md5_init_ctx  (line 83, body cca3edaee830)
- [ ] __md5_process_block  (line 84, body 2ed1d15f9334)
- [ ] __md5_process_bytes  (line 85, body f0baed9a05ef)
- [ ] __md5_read_ctx  (line 86, body d529aa46c5c8)
- [ ] __md5_stream  (line 87, body 4b3563d6f586)

## Types and their fields — 1
- [ ] md5_ctx: uint32_t A, uint32_t B, uint32_t C, uint32_t D, uint32_t total, uint32_t buflen, uint32_t buffer

## File-scope objects — 0

## Decisions (WIP - placeholder ledger)

Generated so that the inventory gate classifies `femacs/lib/include/md5.rs` as in progress
while the md5/sha unit is being ported (an existing mirror with *no* ledger reads as a
failure, not as work in progress).  The porter of `lib/md5.c` replaces this stub with the
real Decisions section.
