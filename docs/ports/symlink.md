# emacs/lib/symlink.c (57 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] rpl_symlink  (line 33)
- [x] symlink  (line 50)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# emacs/lib/symlink.c (57 lines) — port `lib/symlink.rs`

## Decisions

- **HAVE_SYMLINK is 1**: C 26-44's arm (`rpl_symlink`) is the live text and C 46-57's `#else`
  arm (the ENOSYS stub) is dead text kept under `#[cfg (feature = "no_symlink")]` (P-51).
- **`# undef symlink`** (C 29): the live wrapper calls the system entry, which the port keeps in
  a `system` module so C's name and the dead arm's `symlink` do not collide.
- C 38-40's `struct stat st;` is written with `std::mem::zeroed ()` for the same reason as
  lstat.c's; `strlen` is declared in-unit until string.in.h's mirror lands.
- No call-relationship differences (D7): `strlen`, `lstat`, `errno` and `symlink` appear once
  each, and the EEXIST/EOVERFLOW tests keep C's order.
