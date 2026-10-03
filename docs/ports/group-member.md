# emacs/lib/group-member.c (108 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 4
- [x] free_group_info  (line 40, static)
- [x] get_group_info  (line 47, static)
- [x] group_member  (line 74)
- [x] main  (line 98)

## Macros defined — 0

## Types and their fields — 1
- [x] group_info: gid_t* group, gid_t groupbuf

## File-scope objects — 0

## Decisions

- **`gid_t` joined the `<sys/types.h>` mirror** (femacs/lib/include/sys_types.in.rs): the group id
  type this file and getgroups.c take, four bytes on the frozen target (measured with its clang).
- **`getgroups`, `malloc` and `free` are declared in the unit**: C gets them from `<unistd.h>`
  (C 22) and `<stdlib.h>` (C 27), whose mirrors do not carry these three.
- **C 31's `enum { GROUPBUF_SIZE = 100 };` is a `pub const`** (the file uses it as an array
  length, which is all C uses it for).
- **`ckd_mul (&nbytes, n_group_slots, sizeof *gi->group)` is the `overflowing_mul` pair** (P-15,
  the token-aliases family), inverted as C inverts it (`if (! ckd_mul (...))`).
- **The `#ifdef TEST` arm (C 95-108) is kept as gated text** under `#[cfg (feature = "test")]`
  with its own `unsafe extern` block for `atoi`, `printf` and `exit`: TEST is the macro the
  reference's own harness defines, so the arm is dead here, and its `main` is `#[unsafe(no_mangle)]`
  because C defines it non-static in that arm (the same name the checker's export rule expects).
- **The line-count drift (C 108 vs the port's 128, 18.5%) is declared** in
  docs/ports/line-drift-tolerance.tsv with this file's reason.
- **Call-relationship differences (D7)**: none beyond the three libc entries above.

## 2026-10-04: the gated `printf` declaration goes

The `#[cfg (feature = "test")]` arm's `printf` declaration is deleted - the `stdio.rs` mirror
now carries it unconditionally, and the two would be `E0428` when the arm is compiled.
