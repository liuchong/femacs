# emacs/oldXMenu/copyright.h (18 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of `oldXMenu/copyright.h`.  The file carries a permission notice and nothing else: 0
subr, 0 function, 0 macro, 0 type, 0 object.  The mirror is therefore the empty shape plus this
note, the `src/include/getpagesize.rs` and `nt/inc/sys/ioctl.rs` precedent, and the notice's
text is not part of it (PORTING.md 0.1-19).

The frozen configuration builds no window system (`USE_X_TOOLKIT`, `HAVE_X11` and
`HAVE_X_WINDOWS` are `undef` in docs/baseline/config/darwin-arm64.macros.tsv), so nothing
includes this header and no probe compiles it; the text gate is the whole acceptance
(docs/status.md 3.31/3.102).

Evidence: `python3 tools/extract-abi.py --check emacs/oldXMenu/copyright.h
femacs/oldXMenu/include/copyright.rs` -> `OK` (0 ledger item(s)).
