# emacs/lib/getopt-cdefs.in.h (74 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

WIP mirror: the header's four spelling macros (`__BEGIN_DECLS`, `__END_DECLS`,
`__GNUC_PREREQ`, `__THROW`) are attributes/extern-"C" markers with no Rust item; the frozen
configuration leaves `HAVE_SYS_CDEFS_H` undefined, so the `#include <sys/cdefs.h>` arm is dead.
