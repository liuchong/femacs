# emacs/lib/getopt-cdefs.in.h (74 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The header's four spelling macros (`__BEGIN_DECLS`, `__END_DECLS`, `__GNUC_PREREQ`, `__THROW`)
  are attributes and extern-"C" markers with no Rust item, and the frozen configuration leaves
  `HAVE_SYS_CDEFS_H` undefined, so C 30-32's `#include <sys/cdefs.h>` arm is dead (P-08) - the
  generated header `build/emacs-ref/lib/getopt-cdefs.h` is where that is measured.  The mirror
  is therefore a disposition record with no items, like `arg-nonnull.rs`.
- **Name**: this mirror and its ledger were first committed as `getopt-cdefs.rs` /
  `include-getopt-cdefs.md`, i.e. without the `.in` that every other template mirror carries
  (`getopt.in.rs`, `limits.in.rs`, `stdint.in.rs`, `sys_types.in.rs`, `time.in.rs`,
  `fcntl.in.rs`, `unistd.in.rs`, `sys_stat.in.rs`).  `docs/port-map.md` gives
  `lib/getopt-cdefs.in.h` the target `femacs/lib/include/getopt-cdefs.in.rs`, so the old name
  made the row read `todo` forever with the file already in the tree; both were renamed to the
  map's names (status 3.139), and the three places that named the old path
  (`getopt.in.rs`'s include, `getopt-core.rs`'s comment, `docs/ports/getopt.md`) went with it.
