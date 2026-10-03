# emacs/src/sqlite.c (933 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 2
- [x] sqlitep  (Fsqlitep, 1/1, doc f14047e38640, line 858)
- [x] sqlite-available-p  (Fsqlite_available_p, 0/0, doc 1cdc980fa9b9, line 869)

## Functions defined — 1
- [x] syms_of_sqlite  (line 889)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/sqlite.c` @ `fd7afced403`; port `femacs/src/sqlite.rs` (94 lines vs C's 933 - the
tolerance is `sqlite 0.90` in `docs/ports/line-drift-tolerance.tsv`); the gates are
`build-tu src/sqlite.c` -> `built` and
`python3 tools/extract-abi.py --check emacs/src/sqlite.c femacs/src/sqlite.rs` -> `OK`.

- **Which text is live.**  `HAVE_SQLITE3` is `undef` (macros.tsv 1520), so C 33-853 is dead:
  the `#include <sqlite3.h>` with the `SQLITE_VERSION_NUMBER >= 3007015` guards, the
  `HAVE_LOAD_EXTENSION` decision (`HAVE_SQLITE3_LOAD_EXTENSION` is `undef` too), `struct
  Lisp_SQLite` with `XSQLITE`/`SQLITE`/`SQLITE_SET`/`SSQLITE`/`SET_SQLITE`, the
  `check_sqlite`/`init_sqlite_functions`/`column_names`/`bind_values` helpers and the fifteen
  DEFUNs from `sqlite-open` to `sqlite-version`.  The live text is the tail C 855-933, and
  each of its three definitions keeps only its `#else` arm:
  `Fsqlitep`'s `return Qnil;` (C 867), `Fsqlite_available_p`'s `return Qnil;` (C 886) and
  `syms_of_sqlite`'s two `defsubr`s plus the six `DEFSYM`s and four `Fput`s (C 908-931).
- **The two live subrs have no `#ifdef`-free body**, which is why the port keeps the C comment
  where the dead arm was instead of a bare `return` - the ledger is the only place the missing
  430 lines are visible to a reader.
- **Include list.**  C 20's `config.h`, C 23's `lisp.h`: kept.  C 22's `<c-strcase.h>` has no
  mirror yet and the live text calls neither `c_strcasecmp` nor `c_strncasecmp`; C 24's
  `"coding.h"` is needed only by the dead region's `coding_system_p` /
  `Vcoding_system_hash_table` uses.  Both are recorded here rather than mirrored for nothing
  (the atimer.rs precedent for a header the live text never names).
- **`Qsqlite_error`/`Qsqlite_locked_error`** are generated symbols in `globals.rs` (the
  reference's `DEFSYM` list), so they and the four `Fput` calls need no local declaration.
