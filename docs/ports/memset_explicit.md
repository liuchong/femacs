# emacs/lib/memset_explicit.c (54 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] memset_explicit  (line 25)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- Blind file.  The frozen configuration takes the `HAVE_MEMSET_S` arm (macros.tsv 1432 is 1;
  `HAVE_EXPLICIT_MEMSET` is undefined and `HAVE_MEMSET_S_SUPPORTS_ZERO` is 1), so C's
  `if (len > 0)` guard around the call is *not* compiled and C 16-17 is the live text.  The
  other three arms (`explicit_memset`, the two `__asm__ volatile` compiler barriers and the
  `volatile_memset` function pointer) are named at their site (P-51).
- `memset_explicit` is a non-`static` definition → `#[unsafe(no_mangle)]` (P-52); the
  reference object confirms `T _memset_explicit` (gnulib's `<string.h>` does not rename it).
- **Mirror edit**: `lib/include/string.rs` gained the system `memset_s` declaration
  (`errno_t` is `int` and `rsize_t` is `size_t` on this target), which is the entry the live
  arm calls (PORTING.md 0.1-3: system entries belong to the mirror).
- Line-count drift (C 54 vs the port's 46) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
