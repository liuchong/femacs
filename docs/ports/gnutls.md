# emacs/src/gnutls.c (3196 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 1
- [x] gnutls-available-p  (Fgnutls_available_p, 0/0, doc af3baad084da, line 2949)

## Functions defined — 1
- [x] syms_of_gnutls  (line 3019)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/gnutls.c` @ `fd7afced403`; port `femacs/src/gnutls.rs` (122 lines); the
gates are `build-tu src/gnutls.c` -> `built` and
`python3 tools/extract-abi.py --check emacs/src/gnutls.c femacs/src/gnutls.rs` -> `OK`.

- **Which text is live.**  `HAVE_GNUTLS` is `undef`, so 124 of 3,196 lines are live: C 19-28
  (`config.h`, `<errno.h>`, `<stdio.h>`, `lisp.h`, `process.h`, `gnutls.h`, `coding.h`,
  `buffer.h`, `pdumper.h`), `Fgnutls_available_p`'s `capabilities` initialization and return
  (its `#ifdef HAVE_GNUTLS` region, C 2965-3013, is dead) and `syms_of_gnutls`'s
  `libgnutls-version` `DEFVAR_LISP`, its `make_fixnum (-1)` assignment, one `defsubr` and the
  fifty-odd `DEFSYM`s of the certificate/key-exchange plist keys.
- **Dead text (recorded here, not transcribed).**  C 30-2336: the `<gnutls/gnutls.h>` include,
  the `Lisp_GnuTLS_*` records, the log-level and error-symbol tables, the dynlib loading in
  `init_gnutls_functions`, the certificate/key-exchange parsers and the four DEFUNs
  `gnutls-format-certificate`, `gnutls-ciphers`, `gnutls-digests` and `gnutls-hash-mac`.
  C 3036-3142: `syms_of_gnutls`'s `DEFVAR_*`s and callbacks for the same feature.
- **Include list.**  C 24-28's `process.h`, `gnutls.h`, `coding.h`, `buffer.h` and `pdumper.h`
  mirrors are not pulled in: the names the live text could take from them (the `Lisp_Process`
  accessors, `struct Lisp_GnuTLS_*`, the coding-system calls, `BVAR` and the `PDUMPER_*` hooks)
  all sit inside the dead region (the atimer.rs precedent).
- **`syms_of_gnutls`'s `doc` token** is the new `macro-body-notes.tsv` row (the `DEFVAR_LISP`
  invocation takes C's docstring positionally, as `syms_of_profiler`'s does).
- **Line-count basis.**  The `gnutls` row in `docs/ports/line-drift-tolerance.tsv` is a `live`
  row, tolerance 0.10 (124 live lines vs the port's 122).
