// femacs: mirror of src/getpagesize.h @ fd7afced403.
//
// The file has no live text on the frozen target.  Everything it contains sits under
// `#ifndef HAVE_GETPAGESIZE` (C 20), and the frozen configuration defines HAVE_GETPAGESIZE
// (config.h 411; docs/baseline/config/darwin-arm64.macros.tsv), so the whole chain - the
// `sysconf (_SC_PAGESIZE)` arm, the `HAVE_SYS_PARAM_H`/`EXEC_PAGESIZE`/`NBPG * CLSIZE`/
// `PAGESIZE` fallbacks and the `8192` punt - is dead text that the port records here instead of
// transcribing (PORTING.md P-08).  `tools/extract-abi.py` finds no subr, function, macro, type
// or object in the live text (0/0/0/0/0), and `#ifndef HAVE_GETPAGESIZE`'s `#define getpagesize
// ()` arms therefore define nothing the port must carry.
//
// The header is still included by sources the frozen build compiles (src/emacs.c,
// src/gmalloc.c, src/ralloc.c), which is why the mapping exists at all - but a unit that
// includes it gets nothing from it here, exactly as C does.  docs/ports/include-getpagesize.md
// records the dead arms for a configuration that lacks getpagesize.
