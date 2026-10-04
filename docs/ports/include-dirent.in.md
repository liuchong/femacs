# emacs/lib/dirent.in.h (427 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 29
- [x] DIR  (line 110, body 3bcf922654d9)
- [x] DTTOIF  (line 416, body 7afbc3e4f664)
- [x] DT_BLK  (line 73, body c1dfd96eea8c)
- [x] DT_CHR  (line 67, body da4b9237bacc)
- [x] DT_DIR  (line 70, body 1b6453892473)
- [x] DT_FIFO  (line 64, body 356a192b7913)
- [x] DT_LNK  (line 79, body b1d5781111d8)
- [x] DT_REG  (line 76, body fe5dbbcea5ce)
- [x] DT_SOCK  (line 82, body 7b52009b64fd)
- [x] DT_UNKNOWN  (line 61, body b6589fc6ab0d)
- [x] DT_WHT  (line 85, body fa35e192121e)
- [x] GNULIB_defined_DIR  (line 114, body 356a192b7913)
- [x] GNULIB_defined_closedir  (line 164, body 356a192b7913)
- [x] GNULIB_defined_opendir  (line 187, body 356a192b7913)
- [x] GNULIB_defined_struct_dirent  (line 49, body 356a192b7913)
- [x] IFTODT  (line 407, body 6f5816900922)
- [x] _  (line 30, body d7a053ee120d)
- [x] _GL_ATTRIBUTE_DEALLOC  (line 125, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_MALLOC  (line 136, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_PURE  (line 146, body da39a3ee5e6b)
- [x] _GL_DIRENT_S_IFWHT  (line 402, body 527911afee16)
- [x] _GL_DIRENT_S_ISWHT  (line 397, body b6589fc6ab0d)
- [x] _GL_DT_NOTDIR  (line 102, body bd60cfec3dba)
- [x] closedir  (line 163, body 4eb78d2f721e)
- [x] dirfd  (line 273, body b5443c2df5e6)
- [x] fdopendir  (line 307, body 230778f55017)
- [x] opendir  (line 186, body 8491a898aa6d)
- [x] readdir  (line 226, body b1628b006168)
- [x] rewinddir  (line 248, body a78dda718cb1)

## Types and their fields — 1
- [x] dirent: char d_type, char d_name

## File-scope objects — 0

## Decisions

`emacs/lib/dirent.in.h` @ `fd7afced403`; port `femacs/lib/include/dirent.in.rs` (200 lines: a mirror that
`include!`s `dirent.rs` - the system half - and `attribute.rs`).

**The template.**  A gnulib template: `@GUARD_PREFIX@`, `@PRAGMA_SYSTEM_HEADER@`,
`@INCLUDE_NEXT@`, `@NEXT_DIRENT_H@`, `@HAVE_DIRENT_H@`, `@DIR_HAS_FD_MEMBER@` and the
`@GNULIB_*@`/`@REPLACE_*@`/`@HAVE_*@` selectors cannot be evaluated by
`tools/check-cond-eval.py`, so it stays off the verified list (P-66) and `extract-abi.py`
reads every arm as live.  The criterion is the generated `build/emacs-ref/lib/dirent.h` (974
lines: it also inlines the text the template's `/* … copied here.  */` comments stand for),
whose substitutions the Evidence diff prints: `@HAVE_DIRENT_H@` and `@DIR_HAS_FD_MEMBER@` are
1, every `@GNULIB_*@` is 0 except fdopendir's block (`#if 1`), `@REPLACE_*@` is 0 throughout
and `@HAVE_*@`/`@HAVE_DECL_*@` are 1.

**The live half.**  The nine `DT_*` values (C 60-86): the frozen `<dirent.h>` defines all of
them (`darwin-arm64.macros.tsv` 800-808) with exactly the values the template's fallbacks
spell, so one ungated constant answers for both arms (the `fcntl.in.rs` `FD_CLOEXEC` shape).
`_GL_DT_NOTDIR` (C 102) is live and stands in `dirent.rs`.  `@GNULIB_FDOPENDIR@`'s live arm
(C 307's block) is a `_GL_CXXALIAS_SYS` - the system `fdopendir` that `dirent.rs`'s
`dirent_decls` declares - and the system `IFTODT`/`DTTOIF` are what the port uses
(`dirent.rs`'s `IFTODT`); C 118-149's three `_GL_ATTRIBUTE_*` names are `attribute.rs`'s
no-ops, which is where the generated header's own copy of attribute.h's text lands.

**The dead text, arm by arm (P-51).**

| item (C line) | mirror | arm | why |
| --- | --- | --- | --- |
| `struct dirent` (C 44-48) | `#[cfg (feature = "no_have_dirent_h")] pub struct dirent` (`d_type`, `d_name`) | `#if !@HAVE_DIRENT_H@` | dead: that substitution is 1 |
| `GNULIB_defined_struct_dirent` (C 49) | gated `1` | same | dead |
| `struct gl_directory`, `DIR` (C 108/110), `GNULIB_defined_DIR` (C 114) | gated `pub enum gl_directory {}` + `macro_rules! DIR { () => { gl_directory } }` + gated `1` | `#if !@DIR_HAS_FD_MEMBER@` | dead: that substitution is 1; the live `DIR` is `dirent.rs`'s opaque enum, and a macro and a type are different namespaces |
| `closedir`/`GNULIB_defined_closedir` (C 163/164), `opendir`/`GNULIB_defined_opendir` (C 186/187), `readdir` (C 226), `rewinddir` (C 248), `dirfd` (C 273), `fdopendir` (C 307) | `#[cfg (feature = "replace_<name>")] macro_rules!` carrying C's `rpl_*` rename | each block's `# if @REPLACE_<name>@` arm | dead (`@REPLACE_*@` 0); `fdopendir`'s block is live but its rename arm is not |
| `_GL_DIRENT_S_ISWHT` (C 395/397), `_GL_DIRENT_S_IFWHT` (C 400/402) | two gated arms each (`no_iftodt_or_dttoif` + `s_iswht`/`s_ifwht`, and `not (…)` for the `#else`) | C 392's `#if ! (defined IFTODT && defined DTTOIF)` | dead: the system defines both `IFTODT` and `DTTOIF`; C's *recorded* definition is the `#else` arm (397/402), the `#ifdef` arms are the ones `darwin-arm64.macros.tsv` 3658/3628 records |
| `IFTODT` (C 407) | `#[cfg (feature = "no_iftodt")] macro_rules!` with C's `S_ISREG`-chain | `# ifndef IFTODT` | dead; the live macro is `dirent.rs`'s |
| `DTTOIF` (C 416) | `#[cfg (feature = "no_dttoif")] macro_rules!` with C's chain | `# ifndef DTTOIF` | dead, and nothing in the port names the system's `DTTOIF`, so the chain is its only spelling here |

**Not carried.**  The guard pair (C 17/426-427), C 32-39 (the `_GL_CONFIG_H_INCLUDED` check
and `<sys/types.h>`, which the port reaches through `dirent.rs`), C 53-59/101/118-120/128-131/
140-145/352-353/374-375/387-390's comments and C 87-99's `static_assert` chain have no Rust
counterpart (PORTING.md 0.1, P-08); C 340-364's `scandir` and C 366-384's `alphasort` blocks
define no name this header owns (their arms are `_GL_FUNCDECL_SYS` declarations and
`_GL_CXXALIAS_SYS_CAST` aliases).  The guard's own name reads as `_` to the extractor, which
`docs/ports/excluded-macros.txt` records, and the mirror writes the frozen spelling
`_GL_DIRENT_H`.

**The frozen record of `struct dirent` is elsewhere, and that is recorded, not hidden.**: the
generated header's live `struct dirent` is the system's (1048 bytes: `femacs/src/dired.rs` 79
measures and declares it at its use site); this mirror carries only the template's own
fallback record, gated.  A later convergence can move `src/dired.rs`'s copy into `dirent.rs`.

**The consumer.**  `femacs/lib/fdopendir.rs` 15 reads this mirror now (it named
`include/dirent.rs` before; its unit check stays `OK` and its build is the Evidence).  The
other four units that include the system half are the integrator's sweep:
`femacs/lib/dirfd.rs`, `femacs/lib/file-has-acl.rs`, `femacs/src/dired.rs` - whose own
`DT_UNKNOWN`/`DT_DIR`/`DT_LNK` and `struct dirent` (lines 74-79 and 96-98) become `E0428`s the
moment it switches - and `femacs/src/include/android.rs`, which is another lane's in-flight
file.

**The read caught a citation defect** (fixed in `dirent.rs`, this batch): its `IFTODT`
comment cited "dirent.in.h 953-959" and its `_GL_DT_NOTDIR` comment "dirent.in.h 98-102";
953-961 is the *generated* file's range and the template's are 406-413 and 102, so both now
name both files.

## Evidence

```
# the frozen substitutions (the generated header the port mirrors)
$ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/dirent.in.h) build/emacs-ref/lib/dirent.h
0a1
> /* DO NOT EDIT! GENERATED AUTOMATICALLY! */
20c21
< @PRAGMA_SYSTEM_HEADER@
---
> #pragma GCC system_header
22d22
< @PRAGMA_COLUMNS@
25,26c26,27
< #if @HAVE_DIRENT_H@
< # @INCLUDE_NEXT@ @NEXT_DIRENT_H@
---
> #if 1
> # include_next <dirent.h>
41c42
< #if !@HAVE_DIRENT_H@
---
> #if !1
104c105
< #if !@DIR_HAS_FD_MEMBER@
---
> #if !1
109c110
< #  if @HAVE_DIRENT_H@
---
> #  if 1
150a152,153
> /* C++ compatible function declaration macros. … (474 lines of the text the template's
>    `/* … copied here.  */` comments stand for) */

$ grep -n '^#if 0\|^#if 1' build/emacs-ref/lib/dirent.h    # the function blocks' arms
706:#if 0   (closedir)   729:#if 0   (opendir)    769:#if 0   (readdir)
811:#if 0   (rewinddir)  838:#if 0   (dirfd)      825:#if 1   (fdopendir)
# `# if 0` under 825 is @REPLACE_FDOPENDIR@ and `#  if !1 || !1` its @HAVE_FDOPENDIR@ /
# @HAVE_DECL_FDOPENDIR@ (both 1); the rest of the blocks are `#if 0` whole, with `#  if !1`
# inside them (@HAVE_*@ 1).

# the values and the live macros, measured with the frozen compiler
$ printf '' > /tmp/empty.c
$ clang -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib \
    -include config.h -include dirent.h -E -dM /tmp/empty.c \
    | grep -E '^(DT_|IFTODT|DTTOIF|_GL_)'
#define DT_BLK 6
#define DT_CHR 2
#define DT_DIR 4
#define DT_FIFO 1
#define DT_LNK 10
#define DT_REG 8
#define DT_SOCK 12
#define DT_UNKNOWN 0
#define DT_WHT 14
#define DTTOIF(dirtype) ((dirtype) << 12)
#define IFTODT(mode) (((mode) & 0170000) >> 12)
#define _GL_ATTRIBUTE_DEALLOC(f,i)
#define _GL_ATTRIBUTE_DEALLOC_FREE _GL_ATTRIBUTE_DEALLOC (free, 1)
#define _GL_ATTRIBUTE_MALLOC __attribute__ ((__malloc__))
#define _GL_ATTRIBUTE_PURE __attribute__ ((__pure__))
#define _GL_DIRENT_H
#define _GL_DT_NOTDIR 0x100
# no `GNULIB_defined_*` here: every arm that would define one is dead.
# `darwin-arm64.macros.tsv` 3628/3658 record S_IFWHT/S_ISWHT (the two arms the C 392 block
# selects between; both sit inside that dead block).

$ grep -n 'define IFTODT\|define IFTODT' build/emacs-ref/lib/dirent.h
954:# define IFTODT(mode) \
963:# define DTTOIF(dirtype) \

# the negative control: the check compares values and tokens, not just name presence
$ mkdir -p /tmp/gt-neg && cp femacs/lib/include/dirent.rs femacs/lib/include/attribute.rs \
      femacs/lib/include/dirent.in.rs /tmp/gt-neg/
$ sed 's/^pub const DT_DIR: c_int = 4;/pub const DT_DIR: c_int = 5;/' \
      femacs/lib/include/dirent.in.rs > /tmp/gt-neg/dirent.in.rs
$ python3 tools/extract-abi.py --check emacs/lib/dirent.in.h /tmp/gt-neg/dirent.in.rs
- macro DT_DIR does not mention 4 (C body: 4)
- constant DT_DIR: C 4 vs Rust 5
FAIL emacs/lib/dirent.in.h -> /tmp/gt-neg/dirent.in.rs
$ sed 's/if S_ISREG (\$mode) { DT_REG }/if ((\$mode) \& 0o170000) == 0o100000 { DT_REG }/' \
      femacs/lib/include/dirent.in.rs > /tmp/gt-neg/dirent.in2.rs
$ python3 tools/extract-abi.py --check emacs/lib/dirent.in.h /tmp/gt-neg/dirent.in2.rs
- macro IFTODT does not mention s_isreg (C body: (S_ISREG (mode) ? DT_REG : …)
FAIL emacs/lib/dirent.in.h -> /tmp/gt-neg/dirent.in2.rs

$ python3 tools/extract-abi.py --check emacs/lib/dirent.in.h femacs/lib/include/dirent.in.rs
  (2 C macros have a recorded Rust counterpart: _, _GL_ATTRIBUTE_PURE)
  (4 C macros have several definitions (configuration branches), body not compared:
   _GL_ATTRIBUTE_DEALLOC, _GL_ATTRIBUTE_MALLOC, _GL_DIRENT_S_IFWHT, _GL_DIRENT_S_ISWHT)
OK emacs/lib/dirent.in.h -> femacs/lib/include/dirent.in.rs

# the mirror compiles, with every value pinned and the dead arms asserted off
$ cd /tmp/femacs-gt-probe-dirent && rustc --edition 2024 --crate-type=lib --emit=obj \
    -C panic=abort dirent_probe.rs -o dirent_probe.o
exit=0        # the nine `DT_*` values + `_GL_DT_NOTDIR` + three `IFTODT!()` conversions +
              # `opendir`/`readdir`/`dirfd`/`fdopendir`/`rewinddir`/`closedir` used through
              # the declarations + the three `_GL_ATTRIBUTE_*` no-ops + 13
              # `assert! (!cfg! (feature = "…"))` guards (P-51)

# one consumer reads the mirror end to end
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-gt cargo run -q -p xtask -- build-tu lib/fdopendir.c
built lib/fdopendir.rs -> ../build/femacs/fdopendir.o
$ python3 tools/extract-abi.py --check emacs/lib/fdopendir.c femacs/lib/fdopendir.rs
OK emacs/lib/fdopendir.c -> femacs/lib/fdopendir.rs

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
$ python3 tools/extract-abi.py --all
checked 523 ported files: 516 done, 7 in progress, 0 failing
```
