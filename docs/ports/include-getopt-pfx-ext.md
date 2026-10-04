# emacs/lib/getopt-pfx-ext.h (69 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 9
- [x] _GETOPT_PFX_EXT_H  (line 21, body 356a192b7913)
- [x] __GETOPT_CONCAT  (line 35, body 72559cbdecbc)
- [x] __GETOPT_ID  (line 37, body 1945c027c447)
- [x] __GETOPT_XCONCAT  (line 36, body 67afd7eb824c)
- [x] __getopt_argv_const  (line 63, body b42f3f6f796f)
- [x] __getopt_argv_const_is_empty  (line 61, body 356a192b7913)
- [x] _getopt_internal  (line 44, body 54b44a4776ad)
- [x] getopt_long  (line 42, body 300699dabff7)
- [x] getopt_long_only  (line 43, body ccd8a8086ce0)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

**Status: the unit is PORTED** (`femacs/lib/include/getopt-pfx-ext.rs`, 2026-10-04).

Criterion: the generated header's `__GETOPT_PREFIX` is `rpl_`, so C 28-34's arm is live - the three
paste macros (`__GETOPT_CONCAT`, `__GETOPT_XCONCAT`, `__GETOPT_ID`) rename `getopt_long`,
`getopt_long_only` and `_getopt_internal` by pasting the prefix onto their names - and C 60-63's
`__GETOPT_PREFIX` arm makes `__getopt_argv_const` empty with `__getopt_argv_const_is_empty` 1.

What the mirror carries: the guard (`_GETOPT_PFX_EXT_H`) and the two values
(`__getopt_argv_const` as an empty-macro marker, `__getopt_argv_const_is_empty` 1) - the third
const is why getopt-ext.rs's prototypes take `char **argv` and `lib/getopt1.rs`'s `ARGV_CAST` is
the identity.  What it cannot carry is recorded instead: identifier pasting has no Rust spelling
(`excluded-macros.txt`, kind=syntax), and the three renames are expressed at the declaration
(`#[link_name = "rpl_getopt_long"]`, getopt-ext.rs) and definition
(`#[unsafe(export_name = "rpl_getopt_long")]`, lib/getopt1.rs) sites, measured on the reference
object - so those three are `linkage` rows.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/getopt-pfx-ext.h femacs/lib/include/getopt-pfx-ext.rs
OK emacs/lib/getopt-pfx-ext.h -> femacs/lib/include/getopt-pfx-ext.rs            (0 problem rows)
$ nm build/emacs-ref/lib/getopt1.o | grep -E ' T _rpl_getopt'
0000000000000000 T _rpl_getopt_long
$ cd femacs && cargo run -q -p xtask -- build-tu lib/getopt1.c
built lib/getopt1.rs -> ../build/femacs/getopt1.o
```
