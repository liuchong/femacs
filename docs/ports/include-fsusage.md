# emacs/lib/fsusage.h (49 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] FSUSAGE_H_  (line 22, body da39a3ee5e6b)

## Types and their fields — 1
- [x] fs_usage: uintmax_t fsu_blocksize, uintmax_t fsu_blocks, uintmax_t fsu_bfree, uintmax_t fsu_bavail, bool fsu_bavail_top_bit_set, uintmax_t fsu_files, uintmax_t fsu_ffree

## File-scope objects — 0

## Decisions

Mirror of lib/fsusage.h, complete: `struct fs_usage` (all seven fields, C's `uintmax_t` -> the
port's `u64`, C's `bool` -> Rust's `bool`), the `get_fs_usage` prototype in a glob-imported
module (PORTING.md 0.1-10), and C 21-22's include guard `FSUSAGE_H_` kept as the marker constant
(the idx.rs shape).  `python3 tools/extract-abi.py --check emacs/lib/fsusage.h
lib/include/fsusage.rs` -> `OK`.
