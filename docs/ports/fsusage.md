# emacs/lib/fsusage.c (226 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] get_fs_usage  (line 114)

## Macros defined — 3
- [x] EXTRACT_TOP_BIT  (line 63, body b8667ceeba00)
- [x] PROPAGATE_ALL_ONES  (line 55, body ebf21ff277c4)
- [x] PROPAGATE_TOP_BIT  (line 73, body 3ba87fc5a24a)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/fsusage.c` @ `fd7afced403`; port `femacs/lib/fsusage.rs` (155 lines).  All checklist
items ticked; `build-tu lib/fsusage.c` -> `built lib/fsusage.rs -> ../build/femacs/fsusage.o`
(0 errors), `--check` -> `OK`.

- **Which arm is live** (the file is VERIFIED).  The frozen table has `STAT_STATFS2_BSIZE` = 1
  and `STAT_STATVFS`/`STAT_STATVFS64`/`STAT_STATFS2_FRSIZE`/`STAT_STATFS2_FSIZE`/`STAT_STATFS4`/
  `STATFS_TRUNCATES_BLOCK_COUNTS` undefined, so C 26's `#if STAT_STATVFS || STAT_STATVFS64` is
  false and C 28-49's `#else` include block is live, taking `<sys/mount.h>` (`HAVE_SYS_MOUNT_H`
  is 1) for `struct statfs`/`statfs`; inside `get_fs_usage` the live path is the
  `STAT_STATFS2_BSIZE` arm (C 162-171) plus C 215-220's block.  The reference object agrees:
  `nm build/emacs-ref/lib/fsusage.o` is `_get_fs_usage` (T) with `_statfs` (U) and nothing else.
- **Dead arms, named not written** (P-51): C 75-103's `#ifdef STAT_STATVFS` `statvfs_works` pair
  (both the constant-1 stub and the glibc uname/strverscmp probe), C 116-139's statvfs reads,
  C 141-151's `statvfs64` arm, C 153-160's `f_frsize` arm, C 173-186's
  `STATFS_TRUNCATES_BLOCK_COUNTS` fixup, C 188-195's `f_fsize` arm and C 197-207's SVR3
  `statfs (file, &fsd, sizeof fsd, 0)` arm.  Their types (`struct statvfs`, `statvfs64`,
  `f_spare`) are not on this target, so they are not even cfg'd text - the
  gettimeofday.c-class treatment.  Hence `line-drift-tolerance.tsv` records `fsusage 0.35`
  (C 226 vs 155, 31.4%).
- **`STAT_STATFS2_BSIZE` is the one place where C's `struct statfs fsd;` is a function-scope
  object** (`#elif` does not open a C scope), so the port declares it at that statement
  position and C 215-220's block reads the same binding; a Rust block would have hidden it.
- **`docs/ports/macro-body-notes.tsv` carries a `get_fs_usage` row** for
  `stat_statfs2_bsize,stat_statfs2_frsize,stat_statfs2_fsize,stat_statfs4`: C 211-213's
  `#if (defined ... \` spans three physical lines, and the tool drops only the first
  (`#`-prefixed) line, so the continuation lines' tokens reach the body comparison even though
  they are preprocessor condition text, not code.
- **Mirrors.**  `include/sys_mount.rs` is new: the measured `struct statfs` (2168 bytes, align 8)
  with the clang probe and every offset in the header comment, plus `fsid_t`, the three array
  lengths and the `statfs` prototype; `include/fsusage.rs` is new: `struct fs_usage` (C's
  `uintmax_t` fields are the port's `u64`, `bool` is Rust's `bool`) and the `get_fs_usage`
  prototype in a glob-imported module (PORTING.md 0.1-10).
- Not verified: gate-fast/gate-probes, ERT, and any behavioural test of `get_fs_usage` (it needs
  a real mount point).
