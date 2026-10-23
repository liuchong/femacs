# emacs/lib/string.in.h (1852 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] memeq  (line 429)
- [x] streq  (line 821)

## Macros defined — 45
- [x] GNULIB_defined_memeq  (line 436, body 356a192b7913)
- [x] GNULIB_defined_streq  (line 828, body 356a192b7913)
- [x] GNULIB_defined_strerror  (line 1663, body 356a192b7913)
- [x] GNULIB_defined_strnul  (line 1311, body 356a192b7913)
- [x] _  (line 45, body 25e0ac914a41)
- [x] _GL_ATTRIBUTE_DEALLOC  (line 101, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_DEALLOC_FREE  (line 193, body 877a21dfea86)
- [x] _GL_ATTRIBUTE_MALLOC  (line 126, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_NONNULL_IF_NONZERO  (line 138, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_NOTHROW  (line 159, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_PURE  (line 170, body da39a3ee5e6b)
- [x] _GL_MEMEQ_INLINE  (line 85, body a960df3ab827)
- [x] _GL_STREQ_INLINE  (line 88, body a960df3ab827)
- [x] _GL_STRNUL_INLINE  (line 91, body a960df3ab827)
- [x] ffsll  (line 346, body f41ce41b6a6c)
- [x] mbscasestr  (line 1543, body 513b473490c2)
- [x] mbschr  (line 1371, body c6922c40055a)
- [x] mbslen  (line 1339, body c2f47770068a)
- [x] mbspbrk  (line 1570, body fd46cdaa65ae)
- [x] mbspcasecmp  (line 1505, body e600fbd6c015)
- [x] mbsrchr  (line 1393, body b5b36e091458)
- [x] mbsstr  (line 1436, body 58de18e5a6f1)
- [x] memccpy  (line 371, body 130a4bdb1885)
- [x] memchr  (line 388, body 84eb7778b0ea)
- [x] memmem  (line 444, body dc751e99580d)
- [x] mempcpy  (line 481, body 7860178dcd9d)
- [x] memset_explicit  (line 551, body a83a88250240)
- [x] stpcpy  (line 614, body 52d32b0d404b)
- [x] stpncpy  (line 646, body 0b9b4e8dea42)
- [x] strcasestr  (line 1137, body 6c9f5fd25967)
- [x] strchrnul  (line 693, body f3a71d535fe7)
- [x] strdup  (line 801, body 3b8bb5c0e7b1)
- [x] strerror  (line 1670, body 77b998d9440f)
- [x] strerror_l  (line 1716, body ca57900a25c1)
- [x] strerror_r  (line 1688, body f91811d1abf3)
- [x] strerrorname_np  (line 1752, body c835f706dd66)
- [x] strncat  (line 837, body 0714e801482c)
- [x] strncpy  (line 864, body 1c298aa09c26)
- [x] strndup  (line 894, body 34cf6486c64b)
- [x] strnlen  (line 951, body 4a3c5ed6ae3a)
- [x] strnul  (line 1307, body a4a5a97b7a7b)
- [x] strsignal  (line 1803, body 6b50d9104c70)
- [x] strstr  (line 1090, body a4deb4ed1cfd)
- [x] strtok_r  (line 1210, body c811e83665b6)
- [x] strverscmp  (line 1826, body bbc27abcc9b9)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The criterion is the generated header, and it exists** - but not through a
  `GL_GENERATE_STRING_H_CONDITION` (there is no such variable; `grep -c '^GL_GENERATE_STRING_H'
  build/emacs-ref/lib/gnulib.mk` prints `0`).  `string.h` sits in the `string` module's
  `BUILT_SOURCES` and gnulib.mk 3838 is `string.h: string.in.h $(top_builddir)/config.status
  $(CXXDEFS_H) $(ARG_NONNULL_H) $(WARN_ON_USE_H)` - an unconditional rule, so the template is
  always substituted and `build/emacs-ref/lib/string.h` (92,367 bytes, 2,399 lines) is the
  criterion.  (The first draft of the mirror cited a `GL_GENERATE_STRING_H_CONDITION = 1` grep
  that does not reproduce; the reading is corrected here and in the mirror.)
- **The substitution table, resolved.**  The template has 111 distinct `@NAME@` placeholders;
  108 of them appear in the `string.h:` rule as `$(VAR)` references and 3 carry literal
  replacements (`GUARD_PREFIX` -> `GL`, `PRAGMA_SYSTEM_HEADER` -> `#pragma GCC system_header`,
  `PRAGMA_COLUMNS` -> empty).  Resolving each `$(VAR)` against `build/emacs-ref/lib/gnulib.mk`
  (the script and its output are in Evidence) gives **36 of the 108 as `1`, 70 as `0`/empty**,
  and 2 as `include_next`/`<string.h>`.  Of the **77** `@GNULIB_*@`/
  `@REPLACE_*@` selectors the template names, **16 are `1`**: `MDA_MEMCCPY`, `MDA_STRDUP`,
  `MEMEQ`, `MEMMEM`, `MEMPCPY`, `MEMRCHR`, `MEMSET_EXPLICIT`, `RAWMEMCHR`, `SIGDESCR_NP`,
  `STPCPY`, `STREQ`, `STRNLEN`, `STRNUL`, `FREE_POSIX`, `REPLACE_MEMMEM`, `REPLACE_FREE`; the
  other 61 are `0`/`0 && ...`, i.e. dead text on this target (P-51).  Because
  `tools/extract-abi.py` cannot evaluate a `@PLACEHOLDER@` condition, the item list above is the
  **union of every arm** (P-66) and the arms are decided by the *frozen configuration* instead
  (this is the `inttypes.in.rs`/`stdbit.in.rs` reading).
- **The live arms, and what each one needs.**
  * `GNULIB_MEMEQ`+`HAVE_DECL_MEMEQ = 0` (gnulib.mk 790) -> the generated header 970-985 is
    `#if 1 && !0` and **defines the function `memeq`**; `GNULIB_STREQ`+`HAVE_DECL_STREQ = 0`
    (802) -> 1362-1377 defines `streq`.  These are the template's only two C *function*
    definitions.
  * `GNULIB_MEMMEM`+`REPLACE_MEMMEM` -> generated header 998 `#if 1` / 999 `# if 1` /
    `# define memmem rpl_memmem` (the frozen `clang -dM` names it, Evidence).
  * `GNULIB_STRNUL` -> `# define strnul(s) _Generic (…)` (generated header 1503 region; `-dM`
    names it), plus the `GNULIB_defined_strnul` witness.
  * `GNULIB_FREE_POSIX`+`REPLACE_FREE` -> the generated header 733-740 block: C's comment says
    "We can't do '#define free rpl_free' here", and the arm **redefines
    `_GL_ATTRIBUTE_DEALLOC_FREE`** to `_GL_ATTRIBUTE_DEALLOC (rpl_free, 1)` (measured: `-dM`
    shows exactly that spelling).  The mirror answers that name from `attribute.rs`, whose
    `_GL_ATTRIBUTE_DEALLOC_FREE` is the empty attribute no-op its own header documents; the
    extractor records the name as one of its "several definitions ... body not compared" rows,
    which is where the two spellings are reconciled.
  * everything else that is `1` (`MDA_MEMCCPY`, `MDA_STRDUP`, `MEMPCPY`, `MEMRCHR`,
    `MEMSET_EXPLICIT`, `RAWMEMCHR`, `SIGDESCR_NP`, `STPCPY`, `STRNLEN`) only reaches
    `_GL_CXXALIAS_SYS`/`_GL_FUNCDECL_SYS` *declarations* of the system's functions, which the
    system half (`string.rs`) carries for the units that use them; the mirror records the
    renames without writing code for them (`clang -dM` has none of those names).
- **The six shapes the mirror uses** (45 macro names + 2 functions = the 47 boxes above):
  1. the include guard: `pub const _GL_STRING_H: c_int = 1;` (C 45; the extractor reads the
     placeholder's name as `_`, which `excluded-macros.txt`'s `_` row covers);
  2. the three `_GL_*_INLINE` qualifier spellings as **`macro_rules!`**, not `const` - C guards
     each with `#ifndef` (C 84-92) so that `lib/memeq.c`/`lib/streq.c`/`lib/strnul.c`'s own
     `_GL_EXTERN_INLINE` spelling wins in those units, and a second `const` in a unit that
     `include!`s this mirror is `E0428` while a macro and a const are different namespaces (the
     `_GL_STDC_*_INLINE` shape `stdbit.in.rs` uses - measured: the two `const`s were `E0428`
     until they became macros);
  3. the four `GNULIB_defined_*` witnesses as ungated `pub const ... : c_int = 1;` where the
     arm is live (`memeq`, `streq`, `strnul`) and as a `#[cfg]`-gated const where it is not
     (`strerror`);
  4. the five `_GL_ATTRIBUTE_*` names, answered by `include!("attribute.rs")` (the template
     inlines `lib/attribute.h`'s text at C 101/126/138/159/170/193);
  5. **the two function definitions, which moved here from `string.rs`** (the ownership ruling
     `stdbit.in.rs` states, status 3.181): `tools/extract-abi.py` reads a function body only out
     of the mirror's *own* text (`macros`/`constants`/`statics` follow the `include!` chain,
     `bodies` does not), and two definitions of one name in one crate is `E0428`.  C's own
     parameter names are kept (`__s1`, `__s2`, `__n`), as `lib/include/getopt*.rs` keeps theirs,
     which is also what the body-token comparison requires; `!memcmp (…)` is written `== 0`
     because Rust's `!` on a `c_int` is bitwise.  The eight units that call them now include the
     owner: `lib/{memeq,streq}.rs`, `lib/time_rz.rs`, `lib/sig2str.rs`, `lib/boot-time.rs`,
     `lib-src/{ebrowse,etags,make-docfile}.rs`;
  6. the **31 rename macros**: each sits in a `#if @REPLACE_X@`/`@GNULIB_X@` arm the frozen
     configuration does not take, so each is a `#[cfg (feature = "gnulib_<name>")] macro_rules!`
     carrying C's own rename tokens (the `include-dirent.in.md` gated-arm shape), and the one
     that *is* live (`memmem`) gets the frozen spelling as the ungated macro after it.  The
     feature spelling follows `include-errno.in.md`/`include-fcntl.in.md`; the `memmem`
     realization is `string.rs`'s `memmem_decl` (`#[link_name = "rpl_memmem"]`), because that is
     the symbol the frozen object exports.
- **`strnul`'s macro is deliberately *not* written here.**  `macro_rules!` shadow by textual
  order, and this mirror `include!`s `string.rs` first, whose `strnul!` (C 1302-1305's `_Generic`
  arm, spelled with the `void *` arm the port writes) and `_gl_strnul` (PORTING.md checklist 12's
  `_GL_STRNUL_INLINE` function) are the frozen spellings; a second `macro_rules! strnul` at this
  point would shadow them and break `lib/time_rz.rs`'s `strnul! (zone_copy)`.  The extractor
  follows the include chain for *macro names* (measured: `--check` reports no missing `strnul`),
  so the name is answered without a second definition.
- **What `--check` cannot see, and what pins it.**  The comparison is by name and body tokens;
  it cannot see whether a gated arm's *condition* is the right one, nor whether the frozen value
  is the one the compiler picks.  The pins are: the `-dM` dump of the frozen compiler (Evidence)
  for the two live macros; the generated header's own text for the live arms; and the negative
  verification below, which shows the value gate and the (dead) rename arms are live checks.
- **Negative verification** (each edit was applied to the mirror, `--check` run, and the file
  restored - the three commands are in Evidence):
  * `GNULIB_defined_streq` 1 -> 0 => `- constant GNULIB_defined_streq: C 1 vs Rust 0` and
    `- macro GNULIB_defined_streq does not mention 1`, exit 1;
  * `streq`'s body `return strcmp (__s1, __s2) == 0;` -> `return false;` =>
    `- function streq does not mention __s1, __s2, strcmp`, exit 1 (so the moved body *is*
    compared, and C's parameter names are part of it);
  * a **dead** arm, `memchr`'s `rpl_memchr` -> `rpl_nope` =>
    `- macro memchr does not mention rpl_memchr (C body: rpl_memchr)`, exit 1.
- **Not carried** (PORTING.md 0.1; the `include-dirent.in.md` "Not carried" paragraph): C 20-26's
  pragma prologue, C 28-45's double-inclusion guard machinery (the `_` box above is its only
  name), C 52-54's `_GL_CONFIG_H_INCLUDED` check, C 56's `<stddef.h>`, C 58-62's `<locale.h>`
  arm, the `#ifdef __cplusplus` arms, and the `_GL_*`-machinery text gnulib inlines from
  `c++defs.h`/`arg-nonnull.h`/`warn-on-use.h` (`_GL_FUNCDECL_*`/`_GL_CXXALIAS_*`/
  `_GL_CXXALIASWARN*`/`_GL_WARN_ON_USE*`), which the port spells as ordinary declarations in
  `string.rs` - those headers have no mirror of their own yet and their names are not in the
  extractor's list for this file.
- **No shared-table row is needed.**  Every one of the 47 names has a Rust item in this mirror or
  in the files it includes; nothing here is a candidate for `excluded-macros.txt`, and no
  `macro-body-notes.tsv` row was needed either - the two function bodies match C's tokens once
  C's own parameter names are kept.

## Evidence

```
$ ls -la build/emacs-ref/lib/string.h && wc -l build/emacs-ref/lib/string.h
-rw-r--r-- 1 liu staff 92367 Oct  3 04:44 build/emacs-ref/lib/string.h
    2399 build/emacs-ref/lib/string.h
$ grep -n 'string.h: string.in.h' build/emacs-ref/lib/gnulib.mk
3838:string.h: string.in.h $(top_builddir)/config.status $(CXXDEFS_H) $(ARG_NONNULL_H) $(WARN_ON_USE_H)
$ grep -c '^GL_GENERATE_STRING_H' build/emacs-ref/lib/gnulib.mk
0
$ sed -n '3834,3836p' build/emacs-ref/lib/gnulib.mk
BUILT_SOURCES += string.h

# We need the following in order to create <string.h> when the system

# the substitution table, resolved: read the `string.h:` rule's 108 `-e` lines out of
# gnulib.mk and resolve each $(VAR) with make's own last-assignment-wins rule
$ python3 - <<'PY'
import re
from pathlib import Path
mk = Path('build/emacs-ref/lib/gnulib.mk').read_text().splitlines()
vals = {}
for l in mk:
    m = re.match(r'^([A-Za-z0-9_]+)\s*=\s*(.*)$', l)
    if m:
        vals[m.group(1)] = m.group(2).strip()
start = next(i for i, l in enumerate(mk) if l.startswith('string.h: string.in.h'))
stop = next(i for i, l in enumerate(mk) if i > start and l.startswith('MOSTLYCLEANFILES'))
rule = '\n'.join(mk[start:stop])
pairs = re.findall(r"@''([A-Z0-9_]+)''@[|/]\$\(([A-Z0-9_]+)\)", rule)
live = [p for p, v in pairs if vals.get(v) == '1']
dead = [p for p, v in pairs if vals.get(v) in ('0', '')]
print(f"substitutions in the rule: {len(pairs)}")
print(f"=1 (live): {len(live)}   =0/empty (dead): {len(dead)}")
print("live GNULIB_/REPLACE_:", ' '.join(p for p in live if p.startswith(('GNULIB_', 'REPLACE_'))))
PY
substitutions in the rule: 108
=1 (live): 36   =0/empty (dead): 70
live GNULIB_/REPLACE_: GNULIB_MEMEQ GNULIB_MEMMEM GNULIB_MEMPCPY GNULIB_MEMRCHR GNULIB_MEMSET_EXPLICIT GNULIB_RAWMEMCHR GNULIB_STPCPY GNULIB_STREQ GNULIB_STRNLEN GNULIB_STRNUL GNULIB_SIGDESCR_NP GNULIB_MDA_MEMCCPY GNULIB_MDA_STRDUP GNULIB_FREE_POSIX REPLACE_MEMMEM REPLACE_FREE

$ grep -nE '^HAVE_DECL_(MEMEQ|STREQ) =' build/emacs-ref/lib/gnulib.mk
790:HAVE_DECL_MEMEQ = 0
802:HAVE_DECL_STREQ = 0
$ sed -n '969,985p' build/emacs-ref/lib/string.h
/* Are S1 and S2, of size N, bytewise equal?  */
#if 1 && !0
# if !GNULIB_defined_memeq
#  ifdef __cplusplus
extern "C" {
#  endif
_GL_MEMEQ_INLINE bool
memeq (void const *__s1, void const *__s2, size_t __n)
{
  return !memcmp (__s1, __s2, __n);
}
#  ifdef __cplusplus
}
#  endif
#  define GNULIB_defined_memeq 1
# endif
#endif

# what the frozen compiler's preprocessor actually defines, after <string.h>
$ printf '#include <config.h>\n#include <string.h>\n' > /tmp/str_probe.c
$ clang -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -dM -E /tmp/str_probe.c \
    | grep -E '^#define (memmem|strnul|GNULIB_defined_)' | sort
#define GNULIB_defined_memeq 1
#define GNULIB_defined_streq 1
#define GNULIB_defined_strnul 1
#define memmem rpl_memmem
#define strnul(s) _Generic (1 ? (s) : (void *) 99, void * : (char *) _gl_strnul (s), const void * : _gl_strnul (s))
(empty stderr)

# the free arm: the live REPLACE_FREE block redefines only the attribute, not the name
$ sed -n '730,741p' build/emacs-ref/lib/string.h
#if 1
# if (1 && !defined free \
      && !(defined __cplusplus && defined GNULIB_NAMESPACE))
/* We can't do '#define free rpl_free' here.  */
#  if defined __cplusplus && (__GLIBC__ + (__GLIBC_MINOR__ >= 14) > 2)
_GL_EXTERN_C void rpl_free (void *) _GL_ATTRIBUTE_NOTHROW;
#  else
_GL_EXTERN_C void rpl_free (void *);
#  endif
#  undef _GL_ATTRIBUTE_DEALLOC_FREE
#  define _GL_ATTRIBUTE_DEALLOC_FREE _GL_ATTRIBUTE_DEALLOC (rpl_free, 1)
# else

# the negative verification (each line applied, checked, then restored; the driver was a
# python one-liner that rewrites the mirror, runs the checker and writes the original back)
$ python3 tools/extract-abi.py --check emacs/lib/string.in.h lib/include/string.in.rs  # 1 kept
$ python3 tools/extract-abi.py --check emacs/lib/string.in.h lib/include/string.in.rs  # 0 -> FAIL
- macro GNULIB_defined_streq does not mention 1 (C body: 1)
- constant GNULIB_defined_streq: C 1 vs Rust 0
$ python3 tools/extract-abi.py --check emacs/lib/string.in.h lib/include/string.in.rs  # return false -> FAIL
- function streq does not mention __s1, __s2, strcmp
$ python3 tools/extract-abi.py --check emacs/lib/string.in.h lib/include/string.in.rs  # rpl_nope -> FAIL
- macro memchr does not mention rpl_memchr (C body: rpl_memchr)

# the mirror and the eight units that now include the owner
$ python3 tools/extract-abi.py --check emacs/lib/string.in.h lib/include/string.in.rs
OK emacs/lib/string.in.h -> lib/include/string.in.rs
$ cd femacs && for u in lib/memeq.c lib/streq.c lib/time_rz.c lib/sig2str.c lib/boot-time.c \
    lib-src/ebrowse.c lib-src/etags.c lib-src/make-docfile.c; do \
    cargo run -q -p xtask -- build-tu $u; done
built lib/memeq.rs -> ../build/femacs/memeq.o          (0 errors)
built lib/streq.rs -> ../build/femacs/streq.o          (0 errors)
built lib/time_rz.rs -> ../build/femacs/time_rz.o      (0 errors)
built lib/sig2str.rs -> ../build/femacs/sig2str.o      (0 errors)
built lib/boot-time.rs -> ../build/femacs/boot-time.o  (0 errors)
built lib-src/ebrowse.rs -> ../build/femacs/ebrowse.o  (0 errors)
built lib-src/etags.rs -> ../build/femacs/etags.o      (0 errors)
built lib-src/make-docfile.rs -> ../build/femacs/make-docfile.o  (0 errors)
$ cargo run -q -p xtask -- layout-check
layout-check passed (see build/femacs/layout_asserts.rs)
```
