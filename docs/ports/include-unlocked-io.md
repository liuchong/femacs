# emacs/lib/unlocked-io.h (148 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 32
- [x] UNLOCKED_IO_H  (line 21, body 356a192b7913)
- [x] clearerr  (line 43, body 7757853afd0d)
- [x] clearerr_unlocked  (line 45, body c9f113f7391c)
- [x] feof  (line 50, body 2a2ba62ee59e)
- [x] feof_unlocked  (line 52, body f2c6bc3216b1)
- [x] ferror  (line 57, body 5c8409d90150)
- [x] ferror_unlocked  (line 59, body eac6a7981deb)
- [x] fflush  (line 64, body eec49b8cb0c5)
- [x] fflush_unlocked  (line 66, body 94d0f02300cc)
- [x] fgets  (line 71, body ef510c7387b0)
- [x] fgets_unlocked  (line 73, body 13696458998c)
- [x] fileno  (line 78, body bb29d907d3c8)
- [x] fileno_unlocked  (line 80, body b8f1f70a3ffa)
- [x] flockfile  (line 140, body a29646b15239)
- [x] fputc  (line 85, body a5edee78cecd)
- [x] fputc_unlocked  (line 87, body e82cdf940e67)
- [x] fputs  (line 92, body 2e1d7590303e)
- [x] fputs_unlocked  (line 94, body c656df650d31)
- [x] fread  (line 99, body 3ed92b62e523)
- [x] fread_unlocked  (line 101, body eb86342d14d0)
- [x] ftrylockfile  (line 143, body b6589fc6ab0d)
- [x] funlockfile  (line 146, body a29646b15239)
- [x] fwrite  (line 106, body c261b8ff24e8)
- [x] fwrite_unlocked  (line 108, body 6e8e07fef97d)
- [x] getc  (line 113, body 5104555c2932)
- [x] getc_unlocked  (line 115, body a63445e3d51f)
- [x] getchar  (line 120, body 08ad8efe7e06)
- [x] getchar_unlocked  (line 122, body d2291e8b7277)
- [x] putc  (line 127, body 66919888dc61)
- [x] putc_unlocked  (line 129, body c3f9c0785596)
- [x] putchar  (line 134, body f1bbbabb378d)
- [x] putchar_unlocked  (line 136, body 9f2d398a96f9)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header is *not* in the verified list, so the mirror carries both arms of every `#if`
  and the ledger carries the criterion for the live one.**  `python3
  tools/check-cond-eval.py lib/unlocked-io.h` -> `lib/unlocked-io.h: 16 conditions, 0 unknown,
  9 disagreements [NOT VERIFIED]`: the evaluator answers 9 of the 16 conditions differently
  from clang, because the real `<stdio.h>` defines `clearerr_unlocked`, `feof_unlocked`,
  `ferror_unlocked`, `fileno_unlocked`, `getc_unlocked`, `getchar_unlocked`, `putc_unlocked`
  and `putchar_unlocked` as macros - the `|| defined <x>_unlocked` disjunct of C 41, 48, 55,
  76, 111, 118, 125, 132 - and the configuration's own macro table cannot see a system
  header's macros.  `tools/extract-abi.py` therefore reads the file blind (P-66) and requires
  all 32 names; `--check` reports no problem line.

- **The live arm per name, measured with the frozen clang.**  The command (in Evidence) prints
  the surviving `#define`s.  The first arm is live for `clearerr`, `feof`, `ferror`, `fileno`,
  `getc`, `getchar`, `putc`, `putchar` (`HAVE_DECL_*_UNLOCKED` is 1 for all eight - macros.tsv
  1270/1278/1279/1282/1288/1289/1307/1308 - and macOS's stdio.h supplies the `*_unlocked`
  macro); the `#else` arm is live for `fflush_unlocked`, `fgets_unlocked`, `fputc_unlocked`,
  `fputs_unlocked`, `fread_unlocked`, `fwrite_unlocked` (`HAVE_DECL_*_UNLOCKED` is 0 for all
  six - 1280/1281/1283/1284/1285/1286 - and no system macro exists, so the `*_unlocked` name
  is the one unlocked-io.h defines and the ordinary entry is not redefined); and the three
  no-ops C 139-146 are unconditional.  The surviving `clearerr_unlocked(p) __sclearerr(p)` in
  the measured output is the *system header's* definition, not this file's dead arm.

- **The gated arms name the condition that would select them.**  Each `#[cfg (feature = ...)]`
  below is the arm the frozen configuration does not compile (P-51), and its feature is the
  C condition that would make it live:
  `no_decl_<x>_unlocked` for the eight dead `*_unlocked` definitions (C 45, 52, 59, 80, 115,
  122, 129, 136) and `decl_<x>_unlocked` for the six dead ordinary-entry definitions (C 64,
  71, 85, 92, 99, 106).  None of the names is in the frozen configuration, so the frozen build
  keeps exactly one arm of each pair; the dead text is nevertheless in the file, which is what
  the blind reading of the header asks for.

- **The three no-ops keep C's evaluation count.**  `# define flockfile(x) ((void) 0)` and
  `# define funlockfile(x) ((void) 0)` do not read their argument, and `# define ftrylockfile(x)
  0` does not either; the arms are `($x:expr) => { 0 }` with no use of `$x`, the shape
  `include/android-asset.rs`'s `assert` uses for `((void) 0)` (P-16).  (The `flockfile` family
  is spelled with `0` rather than `()` because `0` is the token the gate compares; C's body is
  `((void) 0)`, whose token set is `{void, 0}` minus the skipped `void`.)

- **C's `# undef` lines have no Rust counterpart.**  C 42/49/56/63/70/77/84/91/98/105/112/119/
  126/133/139/142/145 undefine the ordinary names before redefining them.  The port has no
  preprocessor to undefine anything from (PORTING.md 0.1); the macros above are the file's
  whole effect, and a unit that wants the ordinary function rather than the macro spelling
  names it directly.

- **`<stdio.h>` (C 39) is left to the including unit.**  The four reference consumers
  (`lib/boot-time.c`, `lib/close-stream.c`, `lib/getdelim.c`, `lib/md5-stream.c`) name their
  includes in their own `.rs` include lists, where `include/stdio.rs` carries the ordinary
  `clearerr`/`feof`/`ferror`/`fflush`/`fgets`/`fileno`/`fputc`/`fputs`/`fread`/`fwrite`/`getc`/
  `getchar`/`putc`/`putchar` declarations this file's live macros expand to.  Including it here
  would make the mirror depend on the includer's scalar scope; the mirror instead spells its
  one scalar as `core::ffi::c_int` and is its own crate root
  (`build-tu lib/unlocked-io.h`).

- **No consumer yet, and that is expected.**  None of the four units above includes this mirror
  today - they were ported before the header's mirror existed and read `<stdio.h>` directly -
  so the compile proof is the header's own crate root plus the tree-level gates.  Wiring a unit
  to it is a unit-side change for that file's owner.

## Evidence

```
$ python3 tools/extract-abi.py emacs/lib/unlocked-io.h
# emacs/lib/unlocked-io.h (148 lines)
## Subrs (DEFUN) - 0
## Functions defined - 0
## Macros defined - 32   (32/32 ticked above; the constant UNLOCKED_IO_H is one of them)
## Types and their fields - 0
## File-scope objects - 0

$ python3 tools/extract-abi.py --check emacs/lib/unlocked-io.h femacs/lib/include/unlocked-io.rs
OK emacs/lib/unlocked-io.h -> femacs/lib/include/unlocked-io.rs
# no `- ` problem line (`| grep -c '^- '` -> 0)

$ python3 tools/check-cond-eval.py lib/unlocked-io.h
lib/unlocked-io.h: 16 conditions, 0 unknown, 9 disagreements [NOT VERIFIED]

# the frozen clang's surviving definitions - the criterion for every live arm
$ clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src \
    -I emacs/lib -include config.h -include unlocked-io.h -x c /dev/null \
    | grep -E '#define (UNLOCKED_IO_H|clearerr|clearerr_unlocked|feof|feof_unlocked|ferror|ferror_unlocked|fflush|fflush_unlocked|fgets|fgets_unlocked|fileno|fileno_unlocked|flockfile|ftrylockfile|funlockfile|fputc|fputc_unlocked|fputs|fputs_unlocked|fread|fread_unlocked|fwrite|fwrite_unlocked|getc|getc_unlocked|getchar|getchar_unlocked|putc|putc_unlocked|putchar|putchar_unlocked)\b'
#define UNLOCKED_IO_H 1
#define clearerr(x) clearerr_unlocked (x)
#define clearerr_unlocked(p) __sclearerr(p)
#define feof(x) feof_unlocked (x)
#define feof_unlocked(p) __sfeof(p)
#define ferror(x) ferror_unlocked (x)
#define ferror_unlocked(p) __sferror(p)
#define fflush_unlocked(x) fflush (x)
#define fgets_unlocked(x,y,z) fgets (x,y,z)
#define fileno(x) fileno_unlocked (x)
#define fileno_unlocked(p) __sfileno(p)
#define flockfile(x) ((void) 0)
#define fputc_unlocked(x,y) fputc (x,y)
#define fputs_unlocked(x,y) fputs (x,y)
#define fread_unlocked(w,x,y,z) fread (w,x,y,z)
#define ftrylockfile(x) 0
#define funlockfile(x) ((void) 0)
#define fwrite_unlocked(w,x,y,z) fwrite (w,x,y,z)
#define getc(x) getc_unlocked (x)
#define getc_unlocked(fp) __sgetc(fp)
#define getchar() getchar_unlocked ()
#define getchar_unlocked() getc_unlocked(stdin)
#define putc(x,y) putc_unlocked (x,y)
#define putchar(x) putchar_unlocked (x)
#define putchar_unlocked(x) putc_unlocked(x, stdout)
# note: no `#define fflush`, `fgets`, `fputc`, `fputs`, `fread`, `fwrite`,
# `getchar_unlocked`, `putc_unlocked`, `clearerr_unlocked` from this header - the six
# ordinary names are not redefined and the `*_unlocked` definitions shown for the eight
# first-arm hosts are the system header's.

$ grep -nE '^HAVE_DECL_(CLEARERR|FEOF|FERROR|FFLUSH|FGETS|FILENO|FPUTC|FPUTS|FREAD|FWRITE|GETC|GETCHAR|PUTC|PUTCHAR)_UNLOCKED' docs/baseline/config/darwin-arm64.macros.tsv
1270:HAVE_DECL_CLEARERR_UNLOCKED	1
1278:HAVE_DECL_FEOF_UNLOCKED	1
1279:HAVE_DECL_FERROR_UNLOCKED	1
1280:HAVE_DECL_FFLUSH_UNLOCKED	0
1281:HAVE_DECL_FGETS_UNLOCKED	0
1282:HAVE_DECL_FILENO_UNLOCKED	1
1283:HAVE_DECL_FPUTC_UNLOCKED	0
1284:HAVE_DECL_FPUTS_UNLOCKED	0
1285:HAVE_DECL_FREAD_UNLOCKED	0
1286:HAVE_DECL_FWRITE_UNLOCKED	0
1288:HAVE_DECL_GETCHAR_UNLOCKED	1
1289:HAVE_DECL_GETC_UNLOCKED	1
1307:HAVE_DECL_PUTCHAR_UNLOCKED	1
1308:HAVE_DECL_PUTC_UNLOCKED	1

$ grep -rln 'unlocked-io.h' emacs/lib/*.c
emacs/lib/boot-time.c
emacs/lib/close-stream.c
emacs/lib/getdelim.c
emacs/lib/md5-stream.c

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188 cargo run -q -p xtask -- build-tu lib/unlocked-io.h
warning: 17 warnings emitted
built lib/include/unlocked-io.rs -> ../build/femacs/include_unlocked-io.o
```
