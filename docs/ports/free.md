# emacs/lib/free.c (53 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] rpl_free  (line 31)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- C 28's `#if !HAVE_FREE_POSIX` is live (the frozen configuration leaves `HAVE_FREE_POSIX`
  undefined: docs/baseline/config/darwin-arm64.macros.tsv 1312), so the replacement is
  compiled; C 30's inner `#if defined __GNUC__ && !defined __clang__` is dead on this target
  (`__clang__` is defined), and its LTO workaround is *named at its site* rather than carried
  as Rust text (P-51; the tempname.rs precedent) — its tokens (`err`, `err[2]`, `errno = 0`,
  `err[errno == 0]`) are in that comment, which is what the checker reads.
- The live arm's own `free (p)` is the **system** `free`: C 15's `# undef free` (between the
  signature and the body) removes gnulib's `#define free rpl_free` for the body.  The port
  calls the stdlib mirror's `free`, and the token-aliases.tsv `free`/`rpl_free` family
  records that the frozen build's *other* units call `_rpl_free`.
- `rpl_free` is a non-`static` definition → `#[unsafe(no_mangle)]` (P-52); the reference
  object confirms `T _rpl_free`.
- Line-count drift (C 53 vs the port's 45) is declared in docs/ports/line-drift-tolerance.tsv
  (the dead arm is named, not carried).
- Call-relationship differences (D7): none.
