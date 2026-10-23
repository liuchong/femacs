# emacs/src/gnutls.h (100 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_GNUTLS_DEFINED  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of src/gnutls.h.  `HAVE_GNUTLS` is undefined in the frozen configuration, so C 25-91's
`#ifdef HAVE_GNUTLS` block is dead - the `<gnutls/gnutls.h>`/`x509.h`/`crypto.h` includes,
`GNUTLS_EMACS_HANDSHAKES_LIMIT` (6000), the `gnutls_initstage_t` enum and its aliases, the two
`GNUTLS_EMACS_ERROR_*` codes and `GNUTLS_INITSTAGE` - and is recorded here instead of
transcribed (PORTING.md P-08).  The live inventory is the guard alone.

**One declaration is live**: C 98's `extern void syms_of_gnutls (void);` sits outside that block,
and src/emacs.c 2462 calls it unconditionally, so the mirror declares it in `gnutls_decls` + glob
import (P-21) - `src/gnutls.rs` defines it, and the definition shadows the import.  The
generated checklist does not list it (the extractor counts definitions, not declarations), which
is why it is called out here.  Found while writing this mirror: `src/emacs.rs`'s use-site
note pointed at `gnutls.h 118`, a line that does not exist in the header - corrected to 98 in the
same commit.

Evidence: `extract-abi.py --check emacs/src/gnutls.h src/include/gnutls.rs` -> `OK`;
`layout-check` passes with the mirror in the probe.
