# emacs/lib/readlink.c (114 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] readlink  (line 32)
- [x] rpl_readlink  (line 54)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# emacs/lib/readlink.c (114 lines) — port `femacs/lib/readlink.rs`

## Decisions

- **HAVE_READLINK is 1**: C 26-43's `#if !HAVE_READLINK` arm (the DJGPP/mingw32 substitute) is
  dead text kept under `#[cfg (feature = "no_readlink")]`, and C 45-114's `#else` arm (the live
  `rpl_readlink`) is what the frozen configuration compiles.  The dead arm's `readlink` and the
  live `rpl_readlink` therefore both exist in the port, as the ledger's two functions require.
- **READLINK_TRAILING_SLASH_BUG and READLINK_TRUNCATE_BUG are undefined** on this target, so
  C 56-70 and C 74-99 are dead as well (kept under their named features, P-51), and
  `# if defined __CYGWIN__` (C 101-109) is dead on darwin-arm64 (kept under `cygwin`).
- **`# undef readlink`** (C 47): the live wrapper calls the system entry, which the port keeps in
  a `system` module so C's name and the port's own `readlink` do not collide.
- The dead arm's `struct stat statbuf;` is uninitialized in C; Rust needs a value, so the port
  writes `std::mem::zeroed ()` - C's own comment says the contents are never read.
- No call-relationship differences (D7): every `readlink`, `stat`, `strlen`, `memset` and
  `memcpy` call site of C is present exactly once per arm.
