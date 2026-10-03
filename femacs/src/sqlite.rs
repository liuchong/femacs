// femacs: port of src/sqlite.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  Only the
// frozen configuration's live text exists here; see the dead-region note below.
//
// `HAVE_SQLITE3` is undefined (docs/baseline/config/darwin-arm64.macros.tsv 1520), so C 33's
// `#ifdef HAVE_SQLITE3` region - C 33-853 - is dead text: the `#include <sqlite3.h>` and its
// measured `SQLITE_VERSION_NUMBER` guards, `HAVE_LOAD_EXTENSION`'s definition, `struct
// Lisp_SQLite` with its `XSQLITE`/`SQLITE`/`SQLITE_SET` accessors and the `check_sqlite`/
// `init_sqlite_functions`/`column_names`/`bind_values` helpers, and the fifteen DEFUNs from
// `sqlite-open` to `sqlite-version`.  It is recorded in docs/ports/sqlite.md, not transcribed
// (P-08).  What is left is the tail C 855-933: `sqlitep`, `sqlite-available-p` and
// `syms_of_sqlite`, each of whose `#ifdef HAVE_SQLITE3` arms is dead too, so their live bodies
// are C's `#else` arms (`return Qnil;`).
//
// C 22's `#include <c-strcase.h>` has no mirror yet, and the live text calls neither
// `c_strcasecmp` nor `c_strncasecmp` (only the dead region did); the include is recorded in
// the ledger rather than mirrored for nothing.  C 24's `"coding.h"` is the same story: the
// coding-system calls (`coding_system_p`, `SYMBOLP (Vcoding_system_hash_table)`, ...) all sit
// inside the dead region (the atimer.rs precedent for a header whose names the live text never
// touches).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

include!("include/config.rs");
include!("include/lisp.rs");

/* The live tail starts here: C 855's `#endif /* HAVE_SQLITE3 */` closed the dead region.  */

DEFUN! ("sqlitep", Fsqlitep, Ssqlitep, 1, 1, 0,
       doc: "Say whether OBJECT is an SQlite object.",
       (object: Lisp_Object),
       {
  // C 862-866 is `#ifdef HAVE_SQLITE3`: `return SQLITE (object)? Qt: Qnil;` is the dead arm,
  // `return Qnil;` (C 867) the live one (docs/ports/sqlite.md).
  return Qnil ();
});

DEFUN! ("sqlite-available-p", Fsqlite_available_p, Ssqlite_available_p, 0, 0, 0,
       doc: "Return t if sqlite3 support is available in this instance of Emacs.",
       (),
       {
  // C 873-884's `#ifdef HAVE_SQLITE3` / `# ifdef WINDOWSNT` arms are dead; the live body is
  // C 886's `return Qnil;`.
  return Qnil ();
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_sqlite ()
{
  // C 891-907's `#ifdef HAVE_SQLITE3` arm holds fifteen `defsubr`s and two `DEFSYM`s; it is
  // dead text (docs/ports/sqlite.md).
  defsubr (&raw mut Ssqlitep);
  defsubr (&raw mut Ssqlite_available_p);

  DEFSYM! (Qsqlite_error, "sqlite-error");
  Fput (Qsqlite_error (), Qerror_conditions (),
	list2 (Qsqlite_error (), Qerror ()));
  Fput (Qsqlite_error (), Qerror_message (),
	build_string (c"Database error".as_ptr ()));

  DEFSYM! (Qsqlite_locked_error, "sqlite-locked-error");
  Fput (Qsqlite_locked_error (), Qerror_conditions (),
	list3 (Qsqlite_locked_error (), Qsqlite_error (), Qerror ()));
  Fput (Qsqlite_locked_error (), Qerror_message (),
	build_string (c"Database locked".as_ptr ()));

  DEFSYM! (Qsqlitep, "sqlitep");
  DEFSYM! (Qfalse, "false");
  DEFSYM! (Qsqlite, "sqlite");
  DEFSYM! (Qsqlite3, "sqlite3");
  DEFSYM! (Qbinary, "binary");
  DEFSYM! (Qcoding_system, "coding-system");
}
