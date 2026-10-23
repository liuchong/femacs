# emacs/lib/_Noreturn.h (43 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _Noreturn  (line 41, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The criterion is the header itself, and the frozen compiler defines nothing.**  C 29's
  `#ifndef _Noreturn` asks whether the *macro* exists; C 30-31's first arm
  (`(!defined __cplusplus || defined __clang__) && 201112 <= __STDC_VERSION__`) is true for Apple
  clang 21 with `-std=gnu23`, and that arm's body says "`_Noreturn` works as-is" - it defines no
  macro.  Both measurements are in Evidence (`-dM` prints no `_Noreturn`, and the expansion of
  `_Noreturn void f (void);` is the bare keyword).
- **The three other arms are dead text with a feature each (P-51)**: C 33-37's
  `#define _Noreturn __attribute__ ((__noreturn__))` (GCC 2.8+/clang/SunPro),
  C 38-39's `#define _Noreturn __declspec (noreturn)` (MSVC) and C 40-41's empty
  `#define _Noreturn`.  Each is a no-op `macro_rules!` - the `attribute.rs` shape, because an
  attribute has no Rust counterpart (PORTING.md 8) and the port's spelling of "this function does
  not return" is `-> !` at the site.
- **The ungated item is a marker const, not an empty macro.**  The extractor's item list has the
  name `_Noreturn` (its recorded body is C 41's *empty* definition) and it refuses an
  empty-expansion macro (`macro _Noreturn has an empty body`), so the frozen arm - "no macro is
  defined" - is written as `pub const _Noreturn: () = ();`, the marker shape
  `lib/include/gnulib_config.rs` uses for `_GL_INLINE`/`_GL_EXTERN_INLINE`.  C's arms come first
  and the marker last, so `rustc` compiles the marker.
- **No translation unit of this configuration includes the header, so the mirror's compiler is a
  `/tmp` probe.**  Measured: no `#include <_Noreturn.h>` anywhere in the reference tree (only
  `ChangeLog`/`gnulib.mk`/`m4/gnulib-comp.m4` name the file), which is why the header's own use is
  `build-aux/snippet/`'s business and not this target's.  The port's use sites spell the attribute
  as `-> !` (measured: `lib-src/hexl.rs` 45, `lib-src/emacsclient.rs` 800,
  `src/data.rs` 115), and `lib-src/make-docfile.rs` 986 writes the literal
  `_Noreturn ` string - ported semantics, not a macro use.
- No shared-table row is needed: the name's body is exempt (four definitions in C), its presence is
  what `--check` compares, and the marker satisfies that.

## Evidence

```
$ printf '#include "_Noreturn.h"\n_Noreturn void f (void);\n' > /tmp/nr.c
$ clang -std=gnu23 -I emacs/lib -E /tmp/nr.c | tail -1
_Noreturn void f (void);
$ clang -std=gnu23 -I emacs/lib -dM -E -include _Noreturn.h -x c /dev/null | grep -i noreturn
(no output: the header defines no macro at all on this target)

$ python3 tools/extract-abi.py --check emacs/lib/_Noreturn.h lib/include/_Noreturn.rs
  (1 C macros have several definitions (configuration branches), body not compared: _Noreturn)
OK emacs/lib/_Noreturn.h -> lib/include/_Noreturn.rs

# the negative control: the name has to be there (the four C bodies exempt the body comparison)
$ grep -v "_Noreturn" lib/include/_Noreturn.rs > /tmp/gt-neg2/n2.rs
$ python3 tools/extract-abi.py --check emacs/lib/_Noreturn.h /tmp/gt-neg2/n2.rs
- missing macro _Noreturn
FAIL emacs/lib/_Noreturn.h -> /tmp/gt-neg2/n2.rs

# the mirror compiles, every dead feature is off, and the marker is the frozen arm.
# The probe is a transient /tmp crate (the errno.in/dirent.in practice, not committed): it
# `include!`s the mirror, asserts the three dead features off, and reads the marker.
$ cd /tmp && rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type bin -C panic=abort \
    -o probe_noreturn /tmp/femacs-gt-probe-noreturn/probe_noreturn.rs
$ /tmp/probe_noreturn
exit=0        # three `assert! (!cfg! (feature = …))` guards, the `: () = _Noreturn` read, and a
              # `fn () -> !` showing the port's spelling of the attribute

$ rg -n '#include <_Noreturn\.h>|#include "_Noreturn\.h"' emacs/
(no output: no C file includes it)
$ rg -n '_Noreturn\.h' emacs/ | grep -v ChangeLog | head -4
emacs/m4/gnulib-comp.m4:1300:  lib/_Noreturn.h
emacs/msdos/autogen/Makefile.in:134:@BUILDING_FOR_WINDOWSNT_TRUE@\t$(top_srcdir)/build-aux/snippet/_Noreturn.h \
emacs/lib/gnulib.mk.in:3152:_NORETURN_H=$(srcdir)/_Noreturn.h
emacs/lib/gnulib.mk.in:3154:EXTRA_DIST += _Noreturn.h

# the condition evaluator agrees with clang on this file (it is on the verified list)
$ python3 tools/check-cond-eval.py lib/_Noreturn.h
lib/_Noreturn.h: 4 conditions, 2 unknown, 0 disagreements [VERIFIED]
# the two unknown ones are the `__STDC_VERSION__` tests; the compiler measurement above is what
# decides which arm this configuration takes, and the mirror carries the other three regardless
# (`extract-abi.py` does not filter a macro list by branch liveness).
```
