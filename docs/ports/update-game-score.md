# emacs/lib-src/update-game-score.c (506 lines)

# ----------------------------------------------------------------------
## Decisions

- **The frozen configuration builds no `update-game-score` object.**  `emacs/lib-src/Makefile.in`
  165 lists it only under `$(and $(use_gamedir), update-game-score${EXEEXT})`, and this target has no
  shared game directory, so the whole text is carried under the **build-level file gate**
  `feature = "use_gamedir"`, on every item *and on the include graph* (P-86 ⑧; the frozen
  `build-tu` object is the empty 336-byte one).
- **Two nested conditions, both arms carried** (P-86 ①): C 51-53's `#ifdef WINDOWSNT` include of
  `ntlib.h` is `feature = "windowsnt"`; C 129-135's `#ifdef HAVE_SHARED_GAME_DIR` in `get_prefix` is
  `feature = "have_shared_game_dir"`, whose *value* is a configure parameter the frozen table does
  not carry (the macro is `undef`), so the arm's path is a marked placeholder - **Owed 1**.
- **C 49's `<unlocked-io.h>` is where this file reaches `<stdio.h>`.**  The C file never includes
  `<stdio.h>` itself; `emacs/lib/unlocked-io.h` 39 is `# include <stdio.h>`, and that is where
  `fprintf`/`stdout`/`stderr`/`fopen`/`fdopen`/`fileno`/`fread`/`ferror`/`fclose`/`rename` come from.
  The port includes `lib/include/unlocked-io.rs` (the gnulib wrapper's mirror, which maps the
  `*_unlocked` macros onto the ordinary entries for this target) and `lib/include/stdio.in.rs` at
  C 49's position.  Finding this was the arm flip's first reading: with only the headers the C text
  itself writes, the probe reported `cannot find value stdout/stderr` and `cannot find type FILE`
  (4 + 4 + 1 errors), because `<stdio.h>` is nowhere else in the graph.
- **Use-site declarations (P-88 ①).** Four names no mirror carries are declared here with their
  `/// C:` prototypes: `<stdlib.h>`'s `exit` (the `movemail.rs` 96-99 reading), `<stdlib.h>`'s
  `qsort` (`stdlib.in.rs` 221 has only the BSD-signature `qsort_r`), `<string.h>`'s `strnlen`, and
  `<inttypes.h>`'s `intmax_t`/`strtoimax`.  `stdlib.in.rs` and the system `stdlib.rs` are both in the
  graph (the gnulib wrapper includes the system header), as C's own `<stdlib.h>` resolves that way.
- **Measured values** (with clang, no memory): `PRIdMAX` is `"j" "d"` on this target (so C 124's
  `"%"PRIdMAX` is the port's `c"%jd"`); `CHAR_BIT` is 8; `struct score_entry` is `sizeof` 16 with
  `score`@0 and `user_data`@8 (asserted in the unit); `EOVERFLOW` comes from the graph
  (`errno.rs`/`errno.in.rs` between them carry it), so the unit declares nothing extra for it.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-20 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 21-32 | the program's doc comment (setuid/setgid, the no-shared-dir fallback) | carried as the port's header prose |
| 34-49 | the fourteen includes (C 34 `<config.h>`, C 36-49 the rest; C 38 `<inttypes.h>` has no mirror, C 49 `<unlocked-io.h>` is where `<stdio.h>` arrives) | file gate; C 38's two names live at their use sites, C 49 brings the stdio mirrors |
| 51-53 | `#ifdef WINDOWSNT` + `#include "ntlib.h"` | `windowsnt` |
| 55-57 | `#ifndef min` / `# define min(a,b) …` / `#endif` | the port's `min!` (one Deviation) |
| 59-60 | `MAX_ATTEMPTS 5`, `MAX_DATA_LEN 1024` | file gate |
| 62-92 | the six forward declarations (C 62/74/75/85/87/89/91) + `struct score_entry` (C 77-80) + `MAX_SCORES` (C 83) | forward declarations dropped (0.1-12); the record and the macro under the file gate |
| 93-99 | `usage` (C 94-99) | file gate |
| 100-107 | `lose` (C 101-106) | file gate |
| 108-122 | `get_user_id` (C 109-121) | file gate |
| 123-138 | `get_prefix` (C 124-137, with C 129-135's shared-dir arm) | file gate; C 129-135 `have_shared_game_dir` |
| 139-170 | `normalize_integer` (C 141-170) | file gate |
| 171-284 | `main` (C 173-284: the getopt loop, the setuid/setgid checks, the score file, `push_score`, the max clamp, `write_scores`) | file gate |
| 285-300 | `read_score` (C 286-300) | file gate |
| 301-355 | `read_scores` (C 302-355) | file gate |
| 356-381 | `score_compare` (C 357-381) | file gate + C's `qsort` comparator linkage |
| 382-387 | `score_compare_reverse` (C 383-387) | file gate |
| 388-417 | `push_score` (C 389-417) | file gate |
| 418-424 | `sort_scores` (C 419-424) | file gate |
| 425-455 | `write_scores` (C 426-455, with C 440-444's `#ifndef DOS_NT` arm) | file gate; C 440 `not (dos_nt)` |
| 456-492 | `lock_file` (C 457-492: the `O_CREAT\|O_EXCL` loop, the one-hour stale-lock rule) | file gate |
| 493-505 | `unlock_file` (C 494-505) | file gate |
| 506 | the trailing `/* update-game-score.c ends here */` | carried as the port's last line |

## Evidence

The **arm flip is a real compile of the whole body** - this file's surface is POSIX and every reader
exists on this machine, which is why it is the best-evidenced class of the P-86 files (the Windows,
GTK and DJGPP units can only be parsed):

- frozen `build-tu lib-src/update-game-score.c` -> `built lib-src/update-game-score.rs ->
  ../build/femacs/update-game-score.o`, exit 0, **336 bytes** (the empty object);
- `probe … --cfg 'feature="use_gamedir"'` -> **exit 0, 0 errors, 6 warnings**, a **470,104-byte
  rlib** (warnings are mirror-side `clashing_extern_declarations`-class);
- `probe … --cfg 'feature="use_gamedir"' --cfg 'feature="have_shared_game_dir"'` -> **exit 0**, a
  470,400-byte rlib (the second arm of `get_prefix` compiles too);
- `extract-abi.py --check emacs/lib-src/update-game-score.c lib-src/update-game-score.rs` ->
  **no `missing` row at all**; the five token rows (`MAX_SCORES`, `get_user_id`, `normalize_integer`,
  `read_scores`, `lock_file`) are in `docs/ports/macro-body-notes.tsv` (status 3.473) and the 54.7%
  drift in `docs/ports/line-drift-tolerance.tsv`, so the check's remaining output is the frozen
  build's expectation, not a gap;
- the first arm reading is what found the `<stdio.h>` chain: with only the headers the C text writes,
  the probe fails with `cannot find value stdout/stderr` and `cannot find type FILE`.

## Deviations

1. C 55-57's `min` guard is the port's `min!` macro (same body; Rust needs a macro, not a
   preprocessor `#ifndef`).
2. C 62/74/75/85/87/89/91's forward declarations are dropped (PORTING.md 0.1-12).
3. C's uninitialised locals are zero-initialised (PORTING.md 1.5): `newscore`, `end`, `scores`,
   `scorecount`/`scorealloc`, `lockstate`, `st`, `buf` and `entry`.
4. `*z++ = '/'` (C 239) and `num -= !*num` (C 163) are two statements each; the `while (*++num == '0')
   continue;` of C 159-160 is a `loop` whose first statement is the pre-increment.
5. `struct score_entry` carries `#[derive(Clone, Copy)]` so C's `newscores[i] = *newscore` is a copy,
   not a move out of a raw pointer (`E0507`).
6. `buf.st_ctime` (C 494) is the macro `st_ctimespec.tv_sec` on this target, so the port names the
   member the mirror carries.
7. C 38's `<inttypes.h>` and `<string.h>`'s `strnlen` have no mirror: `intmax_t`, `strtoimax` and
   `strnlen` are declared at their use sites; `exit` and `qsort` likewise.
8. The `HAVE_SHARED_GAME_DIR` value is a placeholder (Owed 1).

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **`HAVE_SHARED_GAME_DIR`'s value** — the configure parameter (`--with-gameuser`'s directory) is a
   placeholder in the unit; the frozen table has the macro `undef`, so nothing here measured it.  A
   configure run with `--with-gameuser` supplies it; until then the `have_shared_game_dir` arm is
   *compiled* (Evidence) but not *correct*.
2. **The ratchet row and the manifest rows**: `lib-src/update-game-score.c`'s
   `docs/baseline/port-lines-pending.tsv` row is removed, the five token rows and the 56% tolerance
   row are written, and `use_gamedir`/`have_shared_game_dir` are registered - all in 3.473.

## Not claimed

- **Nothing runs.**  The two arm readings are *compile* readings (a 470,104- and a 470,400-byte rlib
  that no one linked or executed): no score file was read, written, locked or unlocked, no `setuid`
  path was exercised, no `qsort` order was observed and no user id was looked up.
- The frozen object is empty by construction (the file gate), so it says nothing about the body.
- The `have_shared_game_dir` arm's *argument* (the game directory) is a placeholder, so that arm's
  behaviour would be wrong on a real shared-dir build until Owed 1 closes.
- The 6 warnings of the first arm are mirror-side declaration clashes; they are not this unit's text
  and are not claimed to be harmless beyond being warnings.
