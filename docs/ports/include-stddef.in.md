# emacs/lib/stddef.in.h (248 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 12
- [x] GNULIB_defined_max_align_t  (line 185, body 356a192b7913)
- [x] NULL  (line 118, body 069126f0d03f)
- [x] _  (line 123, body f41ae8c016c6)
- [x] _GCC_NULLPTR_T  (line 95, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_NOTHROW  (line 147, body da39a3ee5e6b)
- [x] _GL_HAS_BUILTIN_UNREACHABLE  (line 212, body b6589fc6ab0d)
- [x] _GL_STDDEF_ALIGNAS  (line 174, body da39a3ee5e6b)
- [x] _MAX_ALIGN_T  (line 85, body da39a3ee5e6b)
- [x] __CLANG_MAX_ALIGN_T_DEFINED  (line 184, body da39a3ee5e6b)
- [x] gl_unreachable  (line 234, body 683856e844b4)
- [x] max_align_t  (line 183, body a00698d5a52c)
- [x] unreachable  (line 241, body b0d93041dfd9)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The criterion is the generated header plus the frozen clang, because this is a template.**
  `lib/stddef.in.h` is *not* in `docs/baseline/config/cond-eval-verified.txt`:
  `python3 tools/check-cond-eval.py lib/stddef.in.h` -> `lib/stddef.in.h: 34 conditions,
  7 unknown to the evaluator, 9 disagreements [NOT VERIFIED]`.  All nine are the
  `@PLACEHOLDER@` and guard conditions (C 72/92/103/122/163/164/170/220/237), and their root is
  measured: the *probe* is ill-formed for clang, because the template's text still holds `@`:
  `printf '#include <config.h>\n#if ! defined (_@GUARD_PREFIX@_STDDEF_H)\n#define M 1\n#endif\n' >
  /tmp/pl.c && clang -dM -E -std=gnu23 … /tmp/pl.c` prints `error: missing ')' after 'defined'`,
  so clang's "False" for those lines says nothing, while the evaluator answers `unknown` for
  seven of them (it keeps both arms) and True for two (C 164/170, a cascade of C 163).  The
  extractor therefore reads the file blind (P-66) and asks for all 12 macro names, and the
  criterion for the arms is the generated header plus the direct `-dM` measurement below - not
  either side of that differential.  (The pre-fix reading was `10 disagreements`; the tenth was
  C 25's `#if __GNUC__ >= 3`, i.e. one of the lines status 3.202's marker fix removed.)
  `build/emacs-ref/lib/stddef.h` (249 lines, `/* DO NOT EDIT! GENERATED AUTOMATICALLY! */`)
  resolves the substitutions - `@GUARD_PREFIX@` `GL_`, `@HAVE_MAX_ALIGN_T@` 1,
  `@REPLACE_NULL@` 0, `@NULLPTR_T_NEEDS_STDDEF@` 1, `@HAVE_C_UNREACHABLE@` 1,
  `@STDDEF_NOT_IDEMPOTENT@` 0 - and the frozen clang's surviving `#define`s confirm each
  verdict (the command is in Evidence).

  C 122's split double-inclusion guard is the one line whose *evaluator* side is worth naming,
  and it is a table fact, not a target fact: `docs/baseline/config/darwin-arm64.macros.tsv`
  carries `_GL_STDDEF_H` (4247) because `tools/gen-cond-macros.py`'s probe includes
  `<stddef.h>`, which resolves to *this* header's generated copy and defines its own guard, so
  the evaluator takes the guard as already defined.  Measured on this checkout: `grep -n
  '_GL_STDDEF_H' docs/baseline/config/darwin-arm64.macros.tsv` -> `4247:_GL_STDDEF_H`, and
  `clang -dM -E … -include config.h -x c /dev/null | grep -c '_GL_STDDEF_H'` -> `0`, while
  `build/emacs-ref/lib/stddef.h` 123-124 defines it and 73 is the first `#ifndef`: the header's
  body is live on first inclusion.  Clang's half of the differential for that line is the
  ill-formed-probe artifact above, so the *generated* header, not the differential, is the
  criterion for the arms (the `-dM` measurement below is what decided them).

- **Live/dead, name by name (the ledger's own table).**

  | C lines | name | verdict | why |
  | --- | --- | --- | --- |
  | 72-74, 122-124, 246-247 | `_GL_STDDEF_H` | live (marker) | the split double-inclusion guard the `#include_next` needs; the mirror keeps the name as a `bool` marker |
  | 130-150 | `_GL_ATTRIBUTE_NOTHROW` | dead | the configuration defines it as `__attribute__ ((__nothrow__))` (measured); PORTING.md 8 gives an attribute no Rust counterpart |
  | 204-214 | `_GL_HAS_BUILTIN_UNREACHABLE` | dead | the configuration defines it as `__has_builtin (__builtin_unreachable)` (measured); clang answers 1 |
  | 216-218 | `gl_unreachable` | **live** | `_GL_HAS_BUILTIN_UNREACHABLE` is 1, so the arm is `__builtin_unreachable ()` (measured: that is the surviving definition) |
  | 237-244 | `unreachable` | dead | `@HAVE_C_UNREACHABLE@` is 1 (the surviving `unreachable()` is the *system* header's C23 spelling) |
  | 92-96 | `_GCC_NULLPTR_T` | dead | the generated condition ends `&& !1`; the measurement has no `_GCC_NULLPTR_T` |
  | 78-86 | `_MAX_ALIGN_T` | dead | the AIX 7.2 xlc arm (`defined _AIX` is false) |
  | 152-188 | `GNULIB_defined_max_align_t`, `__CLANG_MAX_ALIGN_T_DEFINED`, `_GL_STDDEF_ALIGNAS`, `max_align_t`, `rpl_max_align_t` | dead | `HAVE_MAX_ALIGN_T` is 1, so `#  if ! (1 \|\| (...))` is false; the surviving `__CLANG_MAX_ALIGN_T_DEFINED` is the *system* header's |
  | 102-120 | `NULL` | dead | `@REPLACE_NULL@` is 0; the surviving `NULL ((void*)0)` is the *system* header's |

  Each dead arm is `#[cfg (feature = "...")]` text (P-51) whose feature names the condition that
  would make it live: `replace_null`, `no_nullptr_t_needs_stddef`, `aix`,
  `no_have_max_align_t`, `no_c_unreachable`.  None is in the frozen configuration, so the frozen
  build compiles none of them.

- **What is recorded here instead of transcribed.**  (a) The `rpl_max_align_t` type, C 176-182:

      typedef union
      {
        char *__p _GL_STDDEF_ALIGNAS (char *);
        double __d _GL_STDDEF_ALIGNAS (double);
        long double __ld _GL_STDDEF_ALIGNAS (long double);
        long int __i _GL_STDDEF_ALIGNAS (long int);
      } rpl_max_align_t;

  Its members carry an attribute (`__attribute__ ((__aligned__ (__alignof__ (type))))`, C 171-172)
  rather than a Rust item, and the whole block is dead here; the frozen target's own
  `max_align_t` is the system's `long double` - 8 bytes, 8-aligned, measured and recorded by
  `include/scratch_buffer.rs`.  The mirror carries the three *names* the gate asks for
  (`max_align_t`, `_GL_STDDEF_ALIGNAS`, `GNULIB_defined_max_align_t`,
  `__CLANG_MAX_ALIGN_T_DEFINED`) as cfg-gated text whose expansion the ledger documents.
  (b) GCC's "special invocation convention", C 30-69: the `__need_wchar_t`/`__need_size_t`/
  `__need_ptrdiff_t`/`__need_NULL`/`__need_wint_t` arms, their `# undef`s and the
  `_GL_STDDEF_WINT_T` marker.  They exist so that ancient GCC headers can ask this file for one
  typedef at a time through `#include_next`; no Rust translation unit has that convention
  (PORTING.md 0.1-4 - the port's includes are the units' own lists), so the block is not ported.
  (c) C 25-28's `@PRAGMA_SYSTEM_HEADER@`/`@PRAGMA_COLUMNS@` (the generated header's
  `#pragma GCC system_header`) and C 126-128's `_GL_CONFIG_H_INCLUDED` requirement: no Rust
  counterpart (an inner attribute in an included file is E0753).

- **The extractor's `_` is this header's guard, not a macro.**  `#   define _@GUARD_PREFIX@_STDDEF_H`
  (C 123) is cut at the `@` by the extractor's `#define` scanner, so the ledger's macro list has
  a name `_` with body `@GUARD_PREFIX@_STDDEF_H` and its line 123.  `_` already owns a row of
  `docs/ports/excluded-macros.txt` (line 77, added for `lib/getopt.c`'s message macro), and the
  name-keyed table silences this artifact too.  The mirror spells the guard's real name,
  `_GL_STDDEF_H`, which the measurement shows defined with an empty body.

- **`_GL_ATTRIBUTE_NOTHROW` is a third spelling of one name.**  `include/attribute.rs` 184 and
  `include/sys_stat.in.rs` 54 both define it as the same empty macro; this mirror's copy is the
  header's own fallback arm.  A later `macro_rules!` of the same name shadows an earlier one
  (measured: two definitions plus one use print the second body), so co-inclusion cannot break -
  unlike the `const`-vs-`const` case, where a second top-level item is an E0428.

- **The mirror is its own compilation unit**, like the rest of this batch: absolute
  `core::ffi::` paths, no `use`, no `include!`, and `build-tu lib/stddef.in.h` reaches
  `lib/include/stddef.in.rs` as the crate root.  Nothing in the port includes it today:
  `<stddef.h>`'s substance on the frozen target is Rust's own types (`usize`, `isize`,
  `core::ffi`) plus the system header's `max_align_t`/`NULL`/`unreachable`, and a unit that
  needs those writes them directly (the `usize`/`isize` mapping of PORTING.md 1.1).

## Evidence

```
$ python3 tools/extract-abi.py emacs/lib/stddef.in.h
# emacs/lib/stddef.in.h (248 lines)
## Subrs (DEFUN) - 0
## Functions defined - 0
## Macros defined - 12   (12/12 ticked above)
## Types and their fields - 0
## File-scope objects - 0

$ python3 tools/extract-abi.py --check emacs/lib/stddef.in.h lib/include/stddef.in.rs
  (1 C macros have a recorded Rust counterpart: _)
  (8 C macros have several definitions (configuration branches), body not compared: GNULIB_defined_max_align_t, NULL, _GL_ATTRIBUTE_NOTHROW, _GL_HAS_BUILTIN_UNREACHABLE, _GL_STDDEF_ALIGNAS, __CLANG_MAX_ALIGN_T_DEFINED ...)
OK emacs/lib/stddef.in.h -> lib/include/stddef.in.rs
# no `- ` problem line (`| grep -c '^- '` -> 0)

$ python3 tools/check-cond-eval.py lib/stddef.in.h
DISAGREE lib/stddef.in.h:25 `if __GNUC__ >= 3` — clang False, evaluator True
DISAGREE(unknown) lib/stddef.in.h:72 `ifndef _@GUARD_PREFIX@_STDDEF_H` — clang False, evaluator keeps the branch live
DISAGREE(unknown) lib/stddef.in.h:92 `if !defined _GCC_NULLPTR_T && !@NULLPTR_T_NEEDS_STDDEF@` — clang False, evaluator keeps the branch live
DISAGREE(unknown) lib/stddef.in.h:103 `if (@REPLACE_NULL@  && (!defined _@GUARD_PREFIX@_STDDEF_H || defined _@GUARD_PREFIX@_STDDEF_WINT_T))` — clang False, evaluator keeps the branch live
DISAGREE(unknown) lib/stddef.in.h:122 `ifndef _@GUARD_PREFIX@_STDDEF_H` — clang False, evaluator keeps the branch live
DISAGREE(unknown) lib/stddef.in.h:163 `if ! (@HAVE_MAX_ALIGN_T@ || (defined _GCC_MAX_ALIGN_T && !defined __clang__))` — clang False, evaluator keeps the branch live
DISAGREE lib/stddef.in.h:164 `if !GNULIB_defined_max_align_t` — clang False, evaluator True
DISAGREE lib/stddef.in.h:170 `if defined __GNUC__ || (__clang_major__ >= 4)` — clang False, evaluator True
DISAGREE(unknown) lib/stddef.in.h:220 `elif !defined __cplusplus && @HAVE_C_UNREACHABLE@` — clang False, evaluator keeps the branch live
DISAGREE(unknown) lib/stddef.in.h:237 `if !defined __cplusplus && !@HAVE_C_UNREACHABLE@` — clang False, evaluator keeps the branch live
lib/stddef.in.h: 34 conditions, 7 unknown, 10 disagreements [NOT VERIFIED]

# the criterion: the generated header's substitutions
$ wc -l build/emacs-ref/lib/stddef.h
     249 build/emacs-ref/lib/stddef.h
# the generated copy's own line numbers (they are not the template's: the first line is the
# `/* DO NOT EDIT! GENERATED AUTOMATICALLY! */` banner and the two pragmas grew a line each)
$ grep -n 'if !defined _GCC_NULLPTR_T\|ifndef _GL_STDDEF_H\|define _GL_STDDEF_H\|! (1 ||\|!defined __cplusplus && 1\|!defined __cplusplus && !1\|define gl_unreachable\|define unreachable' build/emacs-ref/lib/stddef.h
73:# ifndef _GL_STDDEF_H
93:#  if !defined _GCC_NULLPTR_T && !1
123:#  ifndef _GL_STDDEF_H
124:#   define _GL_STDDEF_H
164:# if ! (1 || (defined _GCC_MAX_ALIGN_T && !defined __clang__))
218:# define gl_unreachable() __builtin_unreachable ()
221:#elif !defined __cplusplus && 1
235:# define gl_unreachable() abort ()
238:#if !defined __cplusplus && !1
242:#  define unreachable() gl_unreachable ()

# the measurement of the live arms
$ clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src \
    -I emacs/lib -include config.h -include stddef.h -x c /dev/null \
    | grep -E '#define (NULL|_GCC_NULLPTR_T|_GL_ATTRIBUTE_NOTHROW|_GL_HAS_BUILTIN_UNREACHABLE|_GL_STDDEF_ALIGNAS|_MAX_ALIGN_T|__CLANG_MAX_ALIGN_T_DEFINED|GNULIB_defined_max_align_t|gl_unreachable|max_align_t|unreachable|_GL_STDDEF_H)\b'
#define NULL ((void*)0)
#define _GL_ATTRIBUTE_NOTHROW __attribute__ ((__nothrow__))
#define _GL_HAS_BUILTIN_UNREACHABLE __has_builtin (__builtin_unreachable)
#define _GL_STDDEF_H 
#define __CLANG_MAX_ALIGN_T_DEFINED 
#define gl_unreachable() __builtin_unreachable ()
#define unreachable() __builtin_unreachable()
# note: no `_GCC_NULLPTR_T`, `max_align_t`, `GNULIB_defined_max_align_t`, `_GL_STDDEF_ALIGNAS`
# or `_MAX_ALIGN_T` - all four of this header's arms for them are dead; `NULL`,
# `__CLANG_MAX_ALIGN_T_DEFINED` and `unreachable` survive from the system header.

# the config.h-only control that settles the `_GL_STDDEF_H` disagreement
$ clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src \
    -I emacs/lib -include config.h -x c /dev/null | grep -E '_GL_STDDEF_H|gl_unreachable'
# no output: config.h alone defines neither, so the header's body is live on first inclusion
$ grep -n '_GL_STDDEF_H' docs/baseline/config/darwin-arm64.macros.tsv
4247:_GL_STDDEF_H	

$ grep -n '^_' docs/ports/excluded-macros.txt
77:_	syntax	`_` is not a Rust identifier (PORTING.md 0.2), so lib/getopt.c's message macro cannot be named in the port; …

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188 cargo run -q -p xtask -- build-tu lib/stddef.in.h
warning: 3 warnings emitted
built lib/include/stddef.in.rs -> ../build/femacs/include_stddef.in.o
```
