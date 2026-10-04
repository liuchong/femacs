# emacs/lib-src/ntlib.h (96 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 29
- [x] _longjmp  (line 84, body db80fe1858b7)
- [x] access  (line 45, body 17982a145cd2)
- [x] chdir  (line 47, body 79abe245b19f)
- [x] chmod  (line 49, body 88298a118fc2)
- [x] close  (line 51, body c02131dc7c32)
- [x] creat  (line 53, body 41f39a55ac99)
- [x] dup  (line 56, body 5d156978e3d3)
- [x] dup2  (line 58, body c56b37f413ae)
- [x] execlp  (line 72, body 692eddfca573)
- [x] execvp  (line 73, body a9e6a75c7101)
- [x] fdopen  (line 74, body 3b19e8fce51e)
- [x] fileno  (line 76, body f3da3d29faba)
- [x] getcwd  (line 78, body cb36405f62bb)
- [x] getpid  (line 80, body 440456939c68)
- [x] getw  (line 79, body 993ed9b9f087)
- [x] index  (line 89, body e82adcb13528)
- [x] isatty  (line 81, body a9e4af0e4432)
- [x] locking  (line 82, body 01bf0f6865ea)
- [x] logb  (line 83, body feab96849a0d)
- [x] pclose  (line 86, body deebf0c78bc5)
- [x] pipe  (line 61, body 0b310b81d568)
- [x] popen  (line 85, body a2510de66665)
- [x] read  (line 63, body be410059a97a)
- [x] rindex  (line 90, body 74020e4c7b38)
- [x] rmdir  (line 65, body f69c092dbdd7)
- [x] umask  (line 87, body 9fbd57c76a6a)
- [x] unlink  (line 67, body 706a5717bd3c)
- [x] utime  (line 88, body e438a0c0d59f)
- [x] write  (line 69, body 72e37d6c70ea)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of lib-src/ntlib.h, complete for this target: the ten routines the MSVC/CRT environment
does not provide (declarations - the CRT and `ntlib.c` define them), the 29-entry rename table
that points the POSIX names at the CRT's underscore spellings, and the two bare `#undef`s.

C 18-23's six includes (`<pwd.h>`, `<malloc.h>`, `<direct.h>`, `<fcntl.h>`, `<io.h>`, `<stdio.h>`)
are recorded: an `#include` has no Rust counterpart and there is no mirror for any of them.
C 25-27's `#ifdef sleep / #undef sleep` and C 92-95's `#undef _WINSOCKAPI_` / `#undef _WINSOCK_H`
are bare undefs - the port has no `#undef`, so only the `sleep` declaration survives.

Each rename (`# undef access` / `# define access _access`, ...) is carried as a `macro_rules!`
that expands to the renamed target: a `#define` of a name has no Rust counterpart (PORTING.md
0.1), the C body's own token is what the gate compares, and nothing can call these macros because
the header is included only under `#ifdef WINDOWSNT` (movemail.c 86, pop.c 30,
update-game-score.c 52, ntlib.c) and the frozen configuration defines no `WINDOWSNT`.  The one
pair that maps *to* the POSIX spelling is C 84's `# define _longjmp longjmp`; `index`/`rindex`
map to `strchr`/`strrchr`.

Nothing compiles the mirror (`xtask layout-check` covers `src/include` and `lib/include` only,
and no ported unit includes it).

Evidence: `python3 tools/extract-abi.py --check emacs/lib-src/ntlib.h
femacs/lib-src/include/ntlib.rs` -> `OK` (39 ledger items).
