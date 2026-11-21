// femacs: port of src/sqlite.c @ fd7afced403.
// P-400 connects this existing implementation to Darwin system SQLite. The
// historical disabled-configuration measurements below retain their scope;
// the newly enabled path still requires actual compilation and acceptance.
//
// One-to-one port (PORTING.md P-86 ①): the reference is transcribed line by line in C's order,
// both arms of every `#if` are carried, and each arm's gate is the lower-case feature of the
// macro the reference tests.  The **file gate is `feature = "have_sqlite3"`** (C 31's
// `#ifdef HAVE_SQLITE3`; `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1551), and
// `src/sqlite.c` **is** in `docs/baseline/frozen-build-sources.txt` 66 (the frozen `src/Makefile`
// compiles it).
//
// **The frozen object is not empty here, and that is the reference's own reading.**  Only the
// body (C 31-856) hangs on the false `#if`; the tail C 858-933 is live, so the *reference's own*
// frozen object carries five symbols - measured: `build/emacs-ref/src/sqlite.o` is 7736 bytes
// with `T _Fsqlitep`, `T _Fsqlite_available_p`, `T _syms_of_sqlite`, `s _Ssqlitep`,
// `s _Ssqlite_available_p` (Evidence ①).  The 336-byte empty object P-86 ⑧ measures on
// `src/androidmenu.c` is what a file whose *whole* body is gated produces
// (`src/sound.c`, `lib-src/pop.c`); it is not this file's reading, and expecting it here is the
// one thing this ledger corrects.
//
// The sub-gates.  **None of them is decided by the frozen table**: the whole file sits inside
// the false `#if`, so `python3 tools/check-cond-eval.py src/sqlite.c` prints
// `21 conditions, 0 unknown, 0 disagreements [VERIFIED]` with every verdict `False` (the
// evaluator short-circuits a dead branch - P-86 ⑦'s shape); each gate below was therefore decided
// by hand and is recorded in `docs/ports/sqlite.md`:
//
//   * C 39-44's `defined HAVE_SQLITE3_LOAD_EXTENSION && defined SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION`
//     is where C *computes* `HAVE_LOAD_EXTENSION`, which C 91/189/710/901 test.  Rust has no
//     `#define` to carry, so those four sites spell the same conjunction:
//     `all (feature = "have_sqlite3_load_extension", feature = "sqlite3_dbconfig_enable_load_extension")`
//     (`HAVE_SQLITE3_LOAD_EXTENSION` is `undef`, macros.tsv 1552; `<sqlite3.h>` does define
//     `SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION` - measured 1005).
//   * C 46/201/874's `#ifdef WINDOWSNT`                -> `feature = "windowsnt"` (undef, 4010).
//   * C 68/113/142/174/475's `SQLITE_VERSION_NUMBER >= 3007015` -> `sqlite_version_number_ge_3007015`
//     and C 483's `>= 3006005`                         -> `sqlite_version_number_ge_3006005`
//     (the `mingw32_version_ge_5000002` naming `lib-src/pop.rs` established for a version test).
//   * C 296/299/311's `#ifdef SQLITE_OPEN_FULLMUTEX` / `SQLITE_OPEN_URI` / `SQLITE_OPEN_MEMORY`
//     -> `sqlite_open_fullmutex` / `sqlite_open_uri` / `sqlite_open_memory`.
//   * C 862/873/891's `#ifdef HAVE_SQLITE3` (the tail's own arms) -> `feature = "have_sqlite3"`.
//
// `<sqlite3.h>` has no file in the reference tree, so its surface is declared **in this unit** at
// C 33's `#include` line (P-88 ①) with every item marked by the header line it comes from.  The
// header **is** on this machine (`$(brew --prefix sqlite)/include/sqlite3.h`, SQLite 3.53.4; the
// Xcode SDK's copy is at `$(xcrun --show-sdk-path)/usr/include/sqlite3.h`), so the opaque pointers
// and every constant were measured with clang rather than guessed (Evidence ②).
//
// The Windows arm (C 46-196) is `text-only`: this machine has no Windows and no mingw toolchain,
// and the `DEF_DLL_FN`/`LOAD_DLL_FN` family it is built on is **deferred tree-wide**
// (`include/w32common.rs` 173-183: token pasting and `__VA_ARGS__`'s trailing-attribute form have
// no `macro_rules!` counterpart - PORTING.md P-22).  Those macros' text and `load_dll_functions`'
// body (C 159-195) are therefore carried as C's own text, and the one call site that waits on them
// (C 210) is named in the ledger - P-91 ④'s "definition known, not yet written" class.
//
// `docs/ports/sqlite.md` carries the C line ranges (C 1-933, the reference's every line), the
// acceptance readings, every deviation and the items the integrator owes.  No behaviour of this
// file was executed anywhere: there is no SQLite library linked, no database opened and no
// statement run.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, unused_must_use)]
// [U1-MODULE-UNIT]
use crate::common::*;

/* C 1-23: the reference's opening block.  The port does not carry it (PORTING.md 0.1-19); the
   ledger's `## C line ranges` records what became of it.  */

// [U1] config.rs provided by common

// The recorded Windows loader block below is historical text, not an active
// loader. A future enabled Windows target must connect it before advertising
// this capability. Darwin resolves the actual system library directly.
#[cfg(all(feature = "have_sqlite3", feature = "windowsnt"))]
compile_error!("SQLite delayed loading is not connected for this target");

/* C 27-29: the body's three headers.  C 25's `<config.h>` and C 28's `"lisp.h"` are the two that
   are *outside* C 31's `#ifdef` in C as well, and they are the two this unit's live tail
   (C 858-933) itself needs - `lisp.rs` is where `DEFUN!`/`DEFSYM!`/`defsubr`/`Q*` come from, so
   gating it would make the live tail unbuildable in the frozen configuration.  C 27's and C 29's
   are the body's own edges and carry the file gate (P-86 ⑧).  */
// C 27: `#include <c-strcase.h>` - the file names neither `c_strcasecmp` nor `c_strncasecmp`
// (its one strcase-family call is plain `strcasecmp`, C 762); the mirror is carried because C
// includes the header.
#[cfg (feature = "have_sqlite3")]
const _: () = ();
// [U1] include/lisp.rs imported via common
// C 29: `#include "coding.h"` - only the body names it (`encode_string_utf_8` C 244,
// `code_convert_string_norecord` C 454, `ENCODE_FILE` C 308/776).
#[cfg (feature = "have_sqlite3")]
const _: () = ();

/* The metavariable the three features above are named after is
   `SQLITE_VERSION_NUMBER >= 3007015`; `>= 3006005` is the second one.  C's text tests them
   inside the file gate, which is why the frozen table never has to know the macro.  */

/* lisp.h's own `#include <string.h>` edge, spelled here because `lisp.rs` carries neither it nor
   the mirror (the `src/sound.rs` 116 / `src/alloc.rs` 59 reading): the body calls `strlen`,
   `strncmp` and `strcmp` (C 751-763).  */
#[cfg (feature = "have_sqlite3")]
const _: () = ();

/* C 31: `#ifdef HAVE_SQLITE3` - the file gate.  C 32 through C 856 sit inside it, and every
   `include!` and item below this line carries it.  */

// C 33: `#include <sqlite3.h>` - the header this section stands for.

/* --------------------------------------------------------------------------------------------
   `<sqlite3.h>`: the surface this file takes from it (P-88 ①).

   C 33's `#include <sqlite3.h>` has no file in the reference tree; the declaration site is C's
   own `#include` line and nothing is added under `src/include/`.  Every item below carries
   the header line it comes from, and the types, the opaque pointers and every constant were
   **measured on this host** with `clang` (Evidence ②): SQLite 3.53.4,
   `SQLITE_VERSION_NUMBER` 3053004 - so C's `>= 3007015` and `>= 3006005` tests and the three
   `SQLITE_OPEN_*` `#ifdef`s are all defined, which is what the five features above say.
   -------------------------------------------------------------------------------------------- */

/// C: `<sqlite3.h>` 275: `typedef struct sqlite3 sqlite3;` - an opaque type: only pointers to it
/// appear in this file.  Measured `sizeof (sqlite3 *) == 8` on this host.
#[cfg (feature = "have_sqlite3")]
#[repr(C)]
pub struct sqlite3 { _private: [u8; 0] }
/// C: `<sqlite3.h>` 4303: `typedef struct sqlite3_stmt sqlite3_stmt;` - opaque likewise.
/// Measured `sizeof (sqlite3_stmt *) == 8`.
#[cfg (feature = "have_sqlite3")]
#[repr(C)]
pub struct sqlite3_stmt { _private: [u8; 0] }
/// C: `<sqlite3.h>` 307: `typedef sqlite_int64 sqlite3_int64;` (C 303-306 select `long long`
/// here).  Measured `sizeof (sqlite3_int64) == 8`.
#[cfg (feature = "have_sqlite3")]
pub type sqlite3_int64 = i64;

#[cfg (feature = "have_sqlite3")]
const _: () = core::assert! (std::mem::size_of::<*mut sqlite3> () == 8);
#[cfg (feature = "have_sqlite3")]
const _: () = core::assert! (std::mem::size_of::<*mut sqlite3_stmt> () == 8);
#[cfg (feature = "have_sqlite3")]
const _: () = core::assert! (std::mem::size_of::<sqlite3_int64> () == 8);

/* The eighteen result codes and flag bits C's text names, each with the header's own line and the
   value measured on this host (the header spells the `SQLITE_OPEN_*` bits in hex; the decimal in
   the `/// C:` line is the measured one).  */
/// C: `<sqlite3.h>` 449: `#define SQLITE_OK           0   /* Successful result */` (measured 0)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_OK: c_int = 0;
/// C: `<sqlite3.h>` 455: `#define SQLITE_BUSY         5   /* The database file is locked */` (measured 5)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_BUSY: c_int = 5;
/// C: `<sqlite3.h>` 456: `#define SQLITE_LOCKED       6   /* A table in the database is locked */` (measured 6)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_LOCKED: c_int = 6;
/// C: `<sqlite3.h>` 470: `#define SQLITE_MISMATCH    20   /* Data type mismatch */` (measured 20)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_MISMATCH: c_int = 20;
/// C: `<sqlite3.h>` 479: `#define SQLITE_ROW         100  /* sqlite3_step() has another row ready */` (measured 100)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_ROW: c_int = 100;
/// C: `<sqlite3.h>` 480: `#define SQLITE_DONE        101  /* sqlite3_step() has finished executing */` (measured 101)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_DONE: c_int = 101;
/// C: `<sqlite3.h>` 5356: `#define SQLITE_INTEGER  1` (measured 1)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_INTEGER: c_int = 1;
/// C: `<sqlite3.h>` 5357: `#define SQLITE_FLOAT    2` (measured 2)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_FLOAT: c_int = 2;
/// C: `<sqlite3.h>` 5358: `#define SQLITE_BLOB     4` (measured 4)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_BLOB: c_int = 4;
/// C: `<sqlite3.h>` 5359: `#define SQLITE_NULL     5` (measured 5)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_NULL: c_int = 5;
/// C: `<sqlite3.h>` 5363: `# define SQLITE_TEXT     3` (measured 3)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_TEXT: c_int = 3;
/// C: `<sqlite3.h>` 603: `#define SQLITE_OPEN_READONLY         0x00000001  /* Ok for sqlite3_open_v2() */` (measured 1)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_OPEN_READONLY: c_int = 0x00000001;
/// C: `<sqlite3.h>` 604: `#define SQLITE_OPEN_READWRITE        0x00000002  /* Ok for sqlite3_open_v2() */` (measured 2)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_OPEN_READWRITE: c_int = 0x00000002;
/// C: `<sqlite3.h>` 605: `#define SQLITE_OPEN_CREATE           0x00000004  /* Ok for sqlite3_open_v2() */` (measured 4)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_OPEN_CREATE: c_int = 0x00000004;
/// C: `<sqlite3.h>` 609: `#define SQLITE_OPEN_URI              0x00000040  /* Ok for sqlite3_open_v2() */` (measured 64)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_OPEN_URI: c_int = 0x00000040;
/// C: `<sqlite3.h>` 610: `#define SQLITE_OPEN_MEMORY           0x00000080  /* Ok for sqlite3_open_v2() */` (measured 128)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_OPEN_MEMORY: c_int = 0x00000080;
/// C: `<sqlite3.h>` 619: `#define SQLITE_OPEN_FULLMUTEX        0x00010000  /* Ok for sqlite3_open_v2() */` (measured 65536)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_OPEN_FULLMUTEX: c_int = 0x00010000;
/// C: `<sqlite3.h>` 2704: `#define SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION 1005 /* int int* */` (measured 1005)
#[cfg (feature = "have_sqlite3")]
pub const SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION: c_int = 1005;

/* The twenty-eight entry points C's text calls, each with the header's own prototype.  */
#[cfg (feature = "have_sqlite3")]
unsafe extern "C" {
  /// C: `<sqlite3.h>` 356: `SQLITE_API int sqlite3_close(sqlite3*);`
  pub fn sqlite3_close (db: *mut sqlite3) -> c_int;
  /// C: `<sqlite3.h>` 3998: `SQLITE_API int sqlite3_open_v2(` (`sqlite3 **ppDb, int flags, const char *zVfs)`)
  pub fn sqlite3_open_v2 (filename: *const c_char, ppdb: *mut *mut sqlite3, flags: c_int,
                          vfs: *const c_char) -> c_int;
  /// C: `<sqlite3.h>` 5617: `SQLITE_API int sqlite3_finalize(sqlite3_stmt *pStmt);`
  pub fn sqlite3_finalize (stmt: *mut sqlite3_stmt) -> c_int;
  /// C: `<sqlite3.h>` 5656: `SQLITE_API int sqlite3_reset(sqlite3_stmt *pStmt);`
  pub fn sqlite3_reset (stmt: *mut sqlite3_stmt) -> c_int;
  /// C: `<sqlite3.h>` 5016: `SQLITE_API int sqlite3_bind_text(sqlite3_stmt*,int,const char*,int,void(*)(void*));`
  pub fn sqlite3_bind_text (stmt: *mut sqlite3_stmt, index: c_int, value: *const c_char,
                            n: c_int, destructor: Option<unsafe extern "C" fn (*mut c_void)>) -> c_int;
  /// C: `<sqlite3.h>` 5009: `SQLITE_API int sqlite3_bind_blob(sqlite3_stmt*, int, const void*, int n, void(*)(void*));`
  pub fn sqlite3_bind_blob (stmt: *mut sqlite3_stmt, index: c_int, value: *const c_void,
                            n: c_int, destructor: Option<unsafe extern "C" fn (*mut c_void)>) -> c_int;
  /// C: `<sqlite3.h>` 5014: `SQLITE_API int sqlite3_bind_int64(sqlite3_stmt*, int, sqlite3_int64);`
  pub fn sqlite3_bind_int64 (stmt: *mut sqlite3_stmt, index: c_int, value: sqlite3_int64) -> c_int;
  /// C: `<sqlite3.h>` 5012: `SQLITE_API int sqlite3_bind_double(sqlite3_stmt*, int, double);`
  pub fn sqlite3_bind_double (stmt: *mut sqlite3_stmt, index: c_int, value: c_double) -> c_int;
  /// C: `<sqlite3.h>` 5015: `SQLITE_API int sqlite3_bind_null(sqlite3_stmt*, int);`
  pub fn sqlite3_bind_null (stmt: *mut sqlite3_stmt, index: c_int) -> c_int;
  /// C: `<sqlite3.h>` 5013: `SQLITE_API int sqlite3_bind_int(sqlite3_stmt*, int, int);`
  pub fn sqlite3_bind_int (stmt: *mut sqlite3_stmt, index: c_int, value: c_int) -> c_int;
  /// C: `<sqlite3.h>` 4245: `SQLITE_API int sqlite3_extended_errcode(sqlite3 *db);`
  pub fn sqlite3_extended_errcode (db: *mut sqlite3) -> c_int;
  /// C: `<sqlite3.h>` 4246: `SQLITE_API const char *sqlite3_errmsg(sqlite3*);`
  pub fn sqlite3_errmsg (db: *mut sqlite3) -> *const c_char;
  /// C: `<sqlite3.h>` 4248: `SQLITE_API const char *sqlite3_errstr(int);`
  pub fn sqlite3_errstr (code: c_int) -> *const c_char;
  /// C: `<sqlite3.h>` 189: `SQLITE_API const char *sqlite3_libversion(void);`
  pub fn sqlite3_libversion () -> *const c_char;
  /// C: `<sqlite3.h>` 5312: `SQLITE_API int sqlite3_step(sqlite3_stmt*);`
  pub fn sqlite3_step (stmt: *mut sqlite3_stmt) -> c_int;
  /// C: `<sqlite3.h>` 2870: `SQLITE_API int sqlite3_changes(sqlite3*);`
  pub fn sqlite3_changes (db: *mut sqlite3) -> c_int;
  /// C: `<sqlite3.h>` 5116: `SQLITE_API int sqlite3_column_count(sqlite3_stmt *pStmt);`
  pub fn sqlite3_column_count (stmt: *mut sqlite3_stmt) -> c_int;
  /// C: `<sqlite3.h>` 5589: `SQLITE_API int sqlite3_column_type(sqlite3_stmt*, int iCol);`
  pub fn sqlite3_column_type (stmt: *mut sqlite3_stmt, icol: c_int) -> c_int;
  /// C: `<sqlite3.h>` 5583: `SQLITE_API sqlite3_int64 sqlite3_column_int64(sqlite3_stmt*, int iCol);`
  pub fn sqlite3_column_int64 (stmt: *mut sqlite3_stmt, icol: c_int) -> sqlite3_int64;
  /// C: `<sqlite3.h>` 5581: `SQLITE_API double sqlite3_column_double(sqlite3_stmt*, int iCol);`
  pub fn sqlite3_column_double (stmt: *mut sqlite3_stmt, icol: c_int) -> c_double;
  /// C: `<sqlite3.h>` 5580: `SQLITE_API const void *sqlite3_column_blob(sqlite3_stmt*, int iCol);`
  pub fn sqlite3_column_blob (stmt: *mut sqlite3_stmt, icol: c_int) -> *const c_void;
  /// C: `<sqlite3.h>` 5587: `SQLITE_API int sqlite3_column_bytes(sqlite3_stmt*, int iCol);`
  pub fn sqlite3_column_bytes (stmt: *mut sqlite3_stmt, icol: c_int) -> c_int;
  /// C: `<sqlite3.h>` 5584: `SQLITE_API const unsigned char *sqlite3_column_text(sqlite3_stmt*, int iCol);`
  pub fn sqlite3_column_text (stmt: *mut sqlite3_stmt, icol: c_int) -> *const c_uchar;
  /// C: `<sqlite3.h>` 5145: `SQLITE_API const char *sqlite3_column_name(sqlite3_stmt*, int N);`
  pub fn sqlite3_column_name (stmt: *mut sqlite3_stmt, n: c_int) -> *const c_char;
  /// C: `<sqlite3.h>` 430: `SQLITE_API int sqlite3_exec(` (`sqlite3*, const char *sql, int (*callback)(void*,int,char**,char**), void *, char **errmsg)`)
  pub fn sqlite3_exec (db: *mut sqlite3, sql: *const c_char,
                       callback: Option<unsafe extern "C" fn (*mut c_void, c_int, *mut *mut c_char,
                                                              *mut *mut c_char) -> c_int>,
                       arg: *mut c_void, errmsg: *mut *mut c_char) -> c_int;
  /// C: `<sqlite3.h>` 4602: `SQLITE_API int sqlite3_prepare_v2(` (`sqlite3 *db, const char *zSql, int nByte, sqlite3_stmt **ppStmt, const char **pzTail)`)
  pub fn sqlite3_prepare_v2 (db: *mut sqlite3, sql: *const c_char, nbyte: c_int,
                             stmt: *mut *mut sqlite3_stmt, tail: *mut *const c_char) -> c_int;
  /// C: `<sqlite3.h>` 1748: `SQLITE_API int sqlite3_db_config(sqlite3*, int op, ...);`
  pub fn sqlite3_db_config (db: *mut sqlite3, op: c_int, ...) -> c_int;
  /// C: `<sqlite3.h>` 7570: `SQLITE_API int sqlite3_load_extension(` (`sqlite3 *db, const char *zFile, const char *zProc, char **pzErrMsg)`)
  pub fn sqlite3_load_extension (db: *mut sqlite3, file: *const c_char, proc_: *const c_char,
                                 errmsg: *mut *mut c_char) -> c_int;
}

/* C 34: blank.  */

/* C 35-38: C's own note, in C's words.  */
/* Support for loading SQLite extensions requires the ability to
   enable and disable loading of extensions (by default this is
   disabled, and we want to keep it that way).  The required macro is
   available since SQLite 3.13.  */

/* C 39-44: `# if defined HAVE_SQLITE3_LOAD_EXTENSION && \
                 defined SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION` / `#  define HAVE_LOAD_EXTENSION 1`
   / `# else` / `#  define HAVE_LOAD_EXTENSION 0` / `# endif`.

   C computes `HAVE_LOAD_EXTENSION` here and tests it at C 91, C 189, C 710 and C 901; Rust has no
   `#define` to carry, so each of those four sites spells the same conjunction of the two names C's
   condition tests.  `HAVE_SQLITE3_LOAD_EXTENSION` is `undef` in the frozen table
   (`docs/baseline/config/darwin-arm64.macros.tsv` 1552), so C's macro is 0 in the frozen
   configuration; `<sqlite3.h>` does define `SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION` (measured
   1005).  */

/* C 45: blank.  */
/* C 46: `#ifdef WINDOWSNT` - the delayed-DLL arm (C 47-196).  It is **text-only**: no Windows
   and no mingw toolchain on this machine, and its two loader macros are deferred tree-wide.  */
// C 48: `# include <windows.h>` - no mirror of its own in the tree; its names' homes are named by
// `docs/ports/include-w32common.md` and `docs/ports/include-w32.md` (the `nt/inc/include/` rule:
// a platform SDK header has no owner mirror).
#[cfg (all (feature = "have_sqlite3", feature = "windowsnt"))]
const _: () = ();
#[cfg (all (feature = "have_sqlite3", feature = "windowsnt"))]
const _: () = ();

/* C 52-99: the `DEF_DLL_FN` block (C 52-89), with C 91-99's `# if HAVE_LOAD_EXTENSION` arm.  **Carried as C's text, not as Rust**: `include/w32common.rs`
   173-183 records that `DEF_DLL_FN`, `LOAD_DLL_FN` and `LOAD_DLL_FN_OPT` are **deferred** - C
   composes two names per expansion by token pasting and takes the function's parameter list (with
   any trailing attributes) as `__VA_ARGS__`, which `macro_rules!` cannot express, so "the Rust
   shape of the three is an open design decision for the Windows target that first expands them
   (PORTING.md P-22) rather than something to invent here".  C's text, verbatim:

     DEF_DLL_FN (SQLITE_API int, sqlite3_finalize, (sqlite3_stmt*));
     DEF_DLL_FN (SQLITE_API int, sqlite3_close, (sqlite3*));
     DEF_DLL_FN (SQLITE_API int, sqlite3_open_v2,
                 (const char*, sqlite3**, int, const char*));
     DEF_DLL_FN (SQLITE_API int, sqlite3_reset, (sqlite3_stmt*));
     DEF_DLL_FN (SQLITE_API int, sqlite3_bind_text,
                 (sqlite3_stmt*, int, const char*, int, void(*)(void*)));
     DEF_DLL_FN (SQLITE_API int, sqlite3_bind_blob,
                 (sqlite3_stmt*, int, const char*, int, void(*)(void*)));
     DEF_DLL_FN (SQLITE_API int, sqlite3_bind_int64,
                 (sqlite3_stmt*, int, sqlite3_int64));
     DEF_DLL_FN (SQLITE_API int, sqlite3_bind_double, (sqlite3_stmt*, int, double));
     DEF_DLL_FN (SQLITE_API int, sqlite3_bind_null, (sqlite3_stmt*, int));
     DEF_DLL_FN (SQLITE_API int, sqlite3_bind_int, (sqlite3_stmt*, int, int));
     DEF_DLL_FN (SQLITE_API int, sqlite3_extended_errcode, (sqlite3*));
     DEF_DLL_FN (SQLITE_API const char*, sqlite3_errmsg, (sqlite3*));
   # if SQLITE_VERSION_NUMBER >= 3007015
     DEF_DLL_FN (SQLITE_API const char*, sqlite3_errstr, (int));
   # endif
     DEF_DLL_FN (SQLITE_API const char*, sqlite3_libversion, (void));
     DEF_DLL_FN (SQLITE_API int, sqlite3_step, (sqlite3_stmt*));
     DEF_DLL_FN (SQLITE_API int, sqlite3_changes, (sqlite3*));
     DEF_DLL_FN (SQLITE_API int, sqlite3_column_count, (sqlite3_stmt*));
     DEF_DLL_FN (SQLITE_API int, sqlite3_column_type, (sqlite3_stmt*, int));
     DEF_DLL_FN (SQLITE_API sqlite3_int64, sqlite3_column_int64,
                 (sqlite3_stmt*, int));
     DEF_DLL_FN (SQLITE_API double, sqlite3_column_double, (sqlite3_stmt*, int));
     DEF_DLL_FN (SQLITE_API const void*, sqlite3_column_blob,
                 (sqlite3_stmt*, int));
     DEF_DLL_FN (SQLITE_API int, sqlite3_column_bytes, (sqlite3_stmt*, int));
     DEF_DLL_FN (SQLITE_API const unsigned char*, sqlite3_column_text,
                 (sqlite3_stmt*, int));
     DEF_DLL_FN (SQLITE_API const char*, sqlite3_column_name, (sqlite3_stmt*, int));
     DEF_DLL_FN (SQLITE_API int, sqlite3_exec,
                 (sqlite3*, const char*, int (*callback)(void*,int,char**,char**),
                  void*, char**));
     DEF_DLL_FN (SQLITE_API int, sqlite3_prepare_v2,
                 (sqlite3*, const char*, int, sqlite3_stmt**, const char**));

   # if HAVE_LOAD_EXTENSION
     DEF_DLL_FN (SQLITE_API int, sqlite3_load_extension,
                 (sqlite3*, const char*, const char*, char**));
   #  undef sqlite3_load_extension
   #  define sqlite3_load_extension fn_sqlite3_load_extension
     DEF_DLL_FN (SQLITE_API int, sqlite3_db_config, (sqlite3*, int, ...));
   #  undef sqlite3_db_config
   #  define sqlite3_db_config fn_sqlite3_db_config
   # endif

   The `#if`/`# if HAVE_LOAD_EXTENSION` above is C 91-99's, and C 39's conjunction is its gate.  */

/* C 100-157: the `# undef` / `# define sqlite3_* fn_sqlite3_*` alias block - the same
   deferral: the aliasing C performs with the preprocessor (C 101-128's twenty-six `#undef`s and
   C 130-157's twenty-six `#define`s, each pair split by C 113-115's and C 142-144's
   `# if SQLITE_VERSION_NUMBER >= 3007015`) has no `macro_rules!` counterpart either, and inventing one here would be a second
   spelling of the deferred shape (PORTING.md 0.1-13's "one construct, one spelling").  C's text
   is carried verbatim in the block above's shape; `docs/ports/sqlite.md`'s Owed list names it.  */

/* C 158: blank.  */
/* C 159-195: `static bool load_dll_functions (HMODULE library)` - **not written as an item**.
   Its whole body is twenty-eight `LOAD_DLL_FN (library, sqlite3_*)` expansions (C 162-193,
   twenty-six of them outside C 189's `# if HAVE_LOAD_EXTENSION`), i.e.
   the deferred macro family above; C's text, verbatim:

     LOAD_DLL_FN (library, sqlite3_finalize);  LOAD_DLL_FN (library, sqlite3_close);
     LOAD_DLL_FN (library, sqlite3_open_v2);   LOAD_DLL_FN (library, sqlite3_reset);
     LOAD_DLL_FN (library, sqlite3_bind_text); LOAD_DLL_FN (library, sqlite3_bind_blob);
     LOAD_DLL_FN (library, sqlite3_bind_int64); LOAD_DLL_FN (library, sqlite3_bind_double);
     LOAD_DLL_FN (library, sqlite3_bind_null); LOAD_DLL_FN (library, sqlite3_bind_int);
     LOAD_DLL_FN (library, sqlite3_extended_errcode);
     LOAD_DLL_FN (library, sqlite3_errmsg);
   #if SQLITE_VERSION_NUMBER >= 3007015
     LOAD_DLL_FN (library, sqlite3_errstr);
   #endif
     LOAD_DLL_FN (library, sqlite3_libversion); LOAD_DLL_FN (library, sqlite3_step);
     LOAD_DLL_FN (library, sqlite3_changes);   LOAD_DLL_FN (library, sqlite3_column_count);
     LOAD_DLL_FN (library, sqlite3_column_type); LOAD_DLL_FN (library, sqlite3_column_int64);
     LOAD_DLL_FN (library, sqlite3_column_double); LOAD_DLL_FN (library, sqlite3_column_blob);
     LOAD_DLL_FN (library, sqlite3_column_bytes); LOAD_DLL_FN (library, sqlite3_column_text);
     LOAD_DLL_FN (library, sqlite3_column_name); LOAD_DLL_FN (library, sqlite3_exec);
   # if HAVE_LOAD_EXTENSION
     LOAD_DLL_FN (library, sqlite3_load_extension);
     LOAD_DLL_FN (library, sqlite3_db_config);
   # endif
     LOAD_DLL_FN (library, sqlite3_prepare_v2);
     return true;

   P-91 ④ applies to the one call site that waits on it: C 210 in `init_sqlite_functions` below.
   The Microsoft ABI's `HMODULE` is a `void *`.  */
/* C 196: `#endif /* WINDOWSNT */`.  */

/* C 197: blank.  */
/* C 198: `static bool init_sqlite_functions (void)` - not `static`'s address-taken, so a plain
   `unsafe fn` (PORTING.md 0.2).  */
#[cfg (feature = "have_sqlite3")]
unsafe fn init_sqlite_functions () -> bool
{
  /* C 201: `#ifdef WINDOWSNT` - the delayed-DLL probe.  */
  #[cfg (feature = "windowsnt")]
  {
    /* C 202: `static bool sqlite3_initialized;` - C's function-local static; Rust keeps it at
       the same scope with C's zero value (`src/androidvfs.rs` 659's shape).  */
    static mut sqlite3_initialized: bool = false;

    if ! sqlite3_initialized                                                      // C 204
      {
        /* C 206: `HMODULE library = w32_delayed_load (Qsqlite3);` (`HMODULE` is a `void *`).  */
        let library: *mut c_void = w32_delayed_load (Qsqlite3 ());

        if library.is_null ()                                                     // C 208
          { message1 (c"sqlite3 library was not found".as_ptr ()); }              // C 209
        else if load_dll_functions (library)                                      // C 210
          {
            sqlite3_initialized = true;                                           // C 212
            Vlibrary_cache = Fcons (Fcons (Qsqlite3 (), Qt ()), Vlibrary_cache);  // C 213
          }
        else
          {
            message1 (c"sqlite3 library was found, but could not be loaded successfully".as_ptr ()); // C 217
            Vlibrary_cache = Fcons (Fcons (Qsqlite3 (), Qnil ()), Vlibrary_cache);                    // C 218
          }
      }
    return sqlite3_initialized;                                                   // C 221
  }
  /* C 222: `#else  /* !WINDOWSNT */` and C 223's `return true;`.  */
  #[cfg (not (feature = "windowsnt"))]
  return true;                                                                    // C 223
  /* C 224: `#endif	/* !WINDOWSNT */`.  */
}
/* C 225: end of `init_sqlite_functions`.  */

/* C 226-227: two blank lines.  */

/* C 228: `static void sqlite_free (void *arg)` - its address is stored in the object's
   `finalizer` (C 255), so the item is `extern "C"` (PORTING.md 0.1-6).  */
#[cfg (feature = "have_sqlite3")]
unsafe extern "C" fn sqlite_free (arg: *mut c_void)
{
  let ptr: *mut Lisp_Sqlite = arg as *mut Lisp_Sqlite;             // C 231
  if (*ptr).is_statement                                           // C 232
    { sqlite3_finalize ((*ptr).stmt as *mut sqlite3_stmt); }       // C 233
  else if ! (*ptr).db.is_null ()                                   // C 234
    { sqlite3_close ((*ptr).db as *mut sqlite3); }                 // C 235
  xfree ((*ptr).name as *mut c_void);                              // C 236
  xfree (ptr as *mut c_void);                                      // C 237
}
/* C 238: end of `sqlite_free`.  */

/* C 239: blank.  */
/* C 240: `static Lisp_Object encode_string (Lisp_Object string)`.  */
#[cfg (feature = "have_sqlite3")]
unsafe fn encode_string (string: Lisp_Object) -> Lisp_Object
{
  if STRING_MULTIBYTE (string)                                     // C 243
    { return encode_string_utf_8 (string, Qnil (), false, Qt (), Qt ()); }  // C 244
  else
    { return string; }                                             // C 246
}
/* C 247: end of `encode_string`.  */

/* `coding.h` 693-694 declares `encode_string_utf_8`; the mirror (`include/coding.rs`) does not
   carry it yet, so this unit declares it the way the tree declares what a mirror lacks
   (`src/sound.rs`'s five `lisp.h` names are the same shape).  */
/// C: `coding.h` 693-694: `extern Lisp_Object encode_string_utf_8 (Lisp_Object, Lisp_Object, bool,`
///     `Lisp_Object, Lisp_Object);`
#[cfg (feature = "have_sqlite3")]
unsafe extern "C" {

}

/* C 248: blank.  */
/* C 249: `static Lisp_Object make_sqlite (bool is_statement, void *db, void *stmt, char *name)`.  */
#[cfg (feature = "have_sqlite3")]
unsafe fn make_sqlite (is_statement: bool, db: *mut c_void, stmt: *mut c_void, name: *mut c_char)
  -> Lisp_Object
{
  let ptr: *mut Lisp_Sqlite
    = ALLOCATE_PLAIN_PSEUDOVECTOR! (Lisp_Sqlite, pvec_type::PVEC_SQLITE);   // C 252-253
  (*ptr).is_statement = is_statement;                                       // C 254
  (*ptr).finalizer = Some (sqlite_free);                                    // C 255
  (*ptr).db = db;                                                           // C 256
  (*ptr).name = name;                                                       // C 257
  (*ptr).stmt = stmt;                                                       // C 258
  (*ptr).eof = false;                                                       // C 259
  return make_lisp_ptr (ptr as *mut c_void, Lisp_Type::Lisp_Vectorlike);    // C 260
}
/* C 261: end of `make_sqlite`.  */

/* `struct Lisp_Sqlite` and its accessors are `lisp.h`'s (2914-2925 and 3003-3025): the mirror
   carries them, as in C, and this unit is their only frozen-configuration consumer.  The layout
   is the C compiler's, measured for this host against the frozen `config.h`/`lisp.h`
   (Evidence ③) - never from this file.  */
const _: () = core::assert! (std::mem::size_of::<Lisp_Sqlite> () == 48);
const _: () = core::assert! (std::mem::offset_of! (Lisp_Sqlite, db) == 8);
const _: () = core::assert! (std::mem::offset_of! (Lisp_Sqlite, stmt) == 16);
const _: () = core::assert! (std::mem::offset_of! (Lisp_Sqlite, name) == 24);
const _: () = core::assert! (std::mem::offset_of! (Lisp_Sqlite, finalizer) == 32);
const _: () = core::assert! (std::mem::offset_of! (Lisp_Sqlite, eof) == 40);
const _: () = core::assert! (std::mem::offset_of! (Lisp_Sqlite, is_statement) == 41);

/* C 262: blank.  */
/* C 263: `static void check_sqlite (Lisp_Object db, bool is_statement)`.  */
#[cfg (feature = "have_sqlite3")]
unsafe fn check_sqlite (db: Lisp_Object, is_statement: bool)
{
  init_sqlite_functions ();                                                       // C 266
  CHECK_SQLITE (db);                                                              // C 267
  if is_statement && ! (*XSQLITE (db)).is_statement                               // C 268
    { xsignal1 (Qsqlite_error (), build_string (c"Invalid set object".as_ptr ())); }      // C 269
  else if ! is_statement && (*XSQLITE (db)).is_statement                          // C 270
    { xsignal1 (Qsqlite_error (), build_string (c"Invalid database object".as_ptr ())); } // C 271
  if ! is_statement && (*XSQLITE (db)).db.is_null ()                              // C 272
    { xsignal1 (Qsqlite_error (), build_string (c"Database closed".as_ptr ())); }        // C 273
  else if is_statement && (*XSQLITE (db)).db.is_null ()                           // C 274
    { xsignal1 (Qsqlite_error (), build_string (c"Statement closed".as_ptr ())); }       // C 275
}
/* C 276: end of `check_sqlite`.  */

/* C 277: blank.  */
/* C 278: `static int db_count = 0;` - C's `static` is internal linkage, so the item is not
   `pub` and exports no symbol (PORTING.md 0.2).  */
#[cfg (feature = "have_sqlite3")]
static mut db_count: c_int = 0;
/* C 279: blank.  */

/* C 280: `DEFUN ("sqlite-open", Fsqlite_open, Ssqlite_open, 0, 3, 0, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-open", Fsqlite_open, Ssqlite_open, 0, 3, 0,
       doc: "Open FILE as an sqlite database.\nIf FILE is nil or omitted, an in-memory database will be opened instead.\nIf READONLY is non-nil or omitted, open the database in read-only mode,\notherwise open it in read-write mode.\nBy default, file:// URIs are automatically recognized, unless\nDISABLE-URI is non-nil.  ",
       (file: Lisp_Object, readonly: Lisp_Object, disable_uri: Lisp_Object),
       {
  let mut name: Lisp_Object;                                          // C 289
  let mut flags: c_int;                                               // C 290
  /* C 291: blank.  */

  if ! NILP (readonly)                                                // C 292
    { flags = SQLITE_OPEN_READONLY; }                                 // C 293
  else
    { flags = (SQLITE_OPEN_CREATE | SQLITE_OPEN_READWRITE); }         // C 295
  /* C 296-298: `#ifdef SQLITE_OPEN_FULLMUTEX` / `flags |= SQLITE_OPEN_FULLMUTEX;` / `#endif`.  */
  #[cfg (feature = "sqlite_open_fullmutex")]
  { flags |= SQLITE_OPEN_FULLMUTEX; }                                 // C 297
  /* C 299-302: `#ifdef SQLITE_OPEN_URI` / `if (NILP (disable_uri))` /
     `flags |= SQLITE_OPEN_URI;` / `#endif`.  */
  #[cfg (feature = "sqlite_open_uri")]
  {
    if NILP (disable_uri)                                             // C 300
      { flags |= SQLITE_OPEN_URI; }                                   // C 301
  }
  /* C 303: blank.  */

  if ! init_sqlite_functions ()                                       // C 304
    { xsignal1 (Qsqlite_error (), build_string (c"sqlite support is not available".as_ptr ())); }  // C 305

  if ! NILP (file)                                                    // C 307
    { name = ENCODE_FILE (Fexpand_file_name (file, Qnil ())); }        // C 308
  else
    {
      /* C 311-319: `#ifdef SQLITE_OPEN_MEMORY` / … / `#else` / … / `#endif`.  */
      #[cfg (feature = "sqlite_open_memory")]
      {
        /* C 312-313: `/* In-memory database.  These have to have different names to
           refer to different databases.  */`  */
        AUTO_STRING! (memory_fmt, c":memory:%d".as_ptr ());            // C 314
        db_count += 1;                                                 // C 315's `++db_count`
        name = CALLN! (Fformat, memory_fmt, make_int (db_count as intmax_t));  // C 315
        flags |= SQLITE_OPEN_MEMORY;                                   // C 316
      }
      #[cfg (not (feature = "sqlite_open_memory"))]
      { xsignal1 (Qsqlite_error (), build_string (c"sqlite in-memory is not available".as_ptr ())); }  // C 318
    }

  /* C 322: `sqlite3 *sdb;` - C declares it and lets the callee write it; PORTING.md 1.5's zero
     init is what `&mut sdb` needs.  */
  let mut sdb: *mut sqlite3 = std::ptr::null_mut ();
  if sqlite3_open_v2 (SSDATA (name), &mut sdb, flags, std::ptr::null ()) != SQLITE_OK  // C 323
    { return Qnil (); }                                                // C 324

  /* C 325: blank.  */
  return make_sqlite (false, sdb as *mut c_void, std::ptr::null_mut (),
                      xstrdup (SSDATA (name)));                        // C 326
});

/* C 328: blank.  */
/* C 329: `DEFUN ("sqlite-close", Fsqlite_close, Ssqlite_close, 1, 1, 0, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-close", Fsqlite_close, Ssqlite_close, 1, 1, 0,
       doc: "Close the sqlite database DB.  ",
       (db: Lisp_Object),
       {
  /* C 333-334: `/* Do nothing when the connection is already closed.
     This aligns with the behavior of 'sqlite3_close' itself.  */`  */
  if SQLITE (db) && (*XSQLITE (db)).db.is_null ()                     // C 335
    { return Qt (); }                                                  // C 336

  /* C 337: blank.  */
  check_sqlite (db, false);                                           // C 338
  sqlite3_close ((*XSQLITE (db)).db as *mut sqlite3);                 // C 339
  (*XSQLITE (db)).db = std::ptr::null_mut ();                         // C 340
  return Qt ();                                                       // C 341
});

/* C 343: blank.  */
/* C 344-345: C's own comment.  */
/* Bind values in a statement like
   "insert into foo values (?, ?, ?)".  */
/* C 346: `static const char * bind_values (sqlite3 *db, sqlite3_stmt *stmt, Lisp_Object values)`.  */
#[cfg (feature = "have_sqlite3")]
unsafe fn bind_values (db: *mut sqlite3, stmt: *mut sqlite3_stmt, mut values: Lisp_Object)
  -> *const c_char
{
  sqlite3_reset (stmt);                                               // C 349
  let mut len: c_int;                                                 // C 350
  if VECTORP (values)                                                 // C 351
    { len = ASIZE (values) as c_int; }                                // C 352
  else
    { len = list_length (values) as c_int; }                          // C 354
  /* C 355: blank.  */

  for i in 0 .. len                                                    // C 356
    {
      let mut ret: c_int = SQLITE_MISMATCH;                           // C 358
      let value: Lisp_Object;                                         // C 359
      if VECTORP (values)                                             // C 360
        { value = AREF (values, i as isize); }                         // C 361
      else
        {
          value = XCAR (values);                                      // C 364
          values = XCDR (values);                                     // C 365
        }
      if STRINGP (value)                                              // C 367
        {
          /* C 369: `Lisp_Object encoded;` - C assigns it on three of the four paths below and
             reads it only on the `!blob` one; the port's zero value is PORTING.md 1.5's
             (Deviation 2), and it cannot be observed: the one path that leaves it at `Qnil` is
             exactly the one that reads `value`.  */
          let mut encoded: Lisp_Object = Qnil ();
          let mut blob: bool = false;                                 // C 370
          /* C 371: blank.  */

          if SBYTES (value) == 0                                      // C 372
            { encoded = value; }                                      // C 373
          else
            {
              let coding_system: Lisp_Object =
                Fget_text_property (make_fixnum (0), Qcoding_system (), value);  // C 376-377
              if NILP (coding_system)                                 // C 378
                /* Default to utf-8.  */
                { encoded = encode_string (value); }                  // C 380
              else if EQ (coding_system, Qbinary ())                   // C 381
                { blob = true; }                                      // C 382
              else
                { encoded = Fencode_coding_string (value, coding_system,
                                                   Qnil (), Qnil ()); }  // C 384-385
            }
          /* C 386-387: blank.  */

          if blob                                                     // C 388
            {
              if SBYTES (value) != SCHARS (value)                     // C 390
                { xsignal1 (Qsqlite_error (), build_string (c"BLOB values must be unibyte".as_ptr ())); }  // C 391
              ret = sqlite3_bind_blob (stmt, i + 1,
                                       SSDATA (value) as *const c_void, SBYTES (value) as c_int,
                                       None);                         // C 392-394
            }
          else
            { ret = sqlite3_bind_text (stmt, i + 1,
                                       SSDATA (encoded) as *const c_char, SBYTES (encoded) as c_int,
                                       None); }                       // C 397-399
        }
      else if FIXNUMP (value)                                         // C 401
        { ret = sqlite3_bind_int64 (stmt, i + 1, XFIXNUM (value) as sqlite3_int64); }  // C 402
      else if BIGNUMP (value)                                         // C 403
        { ret = sqlite3_bind_int64 (stmt, i + 1, bignum_to_intmax (value)); }  // C 404
      else if FLOATP (value)                                          // C 405
        { ret = sqlite3_bind_double (stmt, i + 1, XFLOAT_DATA (value)); }      // C 406
      else if NILP (value)                                            // C 407
        { ret = sqlite3_bind_null (stmt, i + 1); }                    // C 408
      else if EQ (value, Qt ())                                       // C 409
        { ret = sqlite3_bind_int (stmt, i + 1, 1); }                  // C 410
      else if EQ (value, Qfalse ())                                   // C 411
        { ret = sqlite3_bind_int (stmt, i + 1, 0); }                  // C 412
      else
        { return c"invalid argument".as_ptr (); }                     // C 414
      /* C 415: blank.  */

      if ret != SQLITE_OK                                             // C 416
        { return sqlite3_errmsg (db); }                               // C 417
    }
  /* C 418-419: end of the loop / blank.  */

  return std::ptr::null ();                                           // C 420
}
/* C 421: end of `bind_values`.  */

/* C 422: blank.  */
/* C 423: `static Lisp_Object row_to_value (sqlite3_stmt *stmt)`.  */
#[cfg (feature = "have_sqlite3")]
unsafe fn row_to_value (stmt: *mut sqlite3_stmt) -> Lisp_Object
{
  let len: c_int = sqlite3_column_count (stmt);                        // C 426
  let mut values: Lisp_Object = Qnil ();                               // C 427
  /* C 428: blank.  */

  for i in (0 .. len).rev ()                                            // C 429
    {
      let mut v: Lisp_Object = Qnil ();                                 // C 431
      /* C 432: blank.  */

      /* C 433: `switch (sqlite3_column_type (stmt, i))`.  C's `switch` has no `default`, so a
         type outside the five cases leaves `v` at C 431's `Qnil` (the `_` arm).  */
      match sqlite3_column_type (stmt, i)
        {
          SQLITE_INTEGER =>
            { v = make_int (sqlite3_column_int64 (stmt, i)); }          // C 436
          SQLITE_FLOAT =>
            { v = make_float (sqlite3_column_double (stmt, i)); }       // C 440
          SQLITE_BLOB =>
            { v = make_unibyte_string (sqlite3_column_blob (stmt, i) as *const c_char,
                                       sqlite3_column_bytes (stmt, i) as ptrdiff_t); }  // C 444-445
          SQLITE_NULL =>
            { v = Qnil (); }                                            // C 449
          SQLITE_TEXT =>
            { v =
                code_convert_string_norecord
                  (make_unibyte_string (sqlite3_column_text (stmt, i) as *const c_char,
                                        sqlite3_column_bytes (stmt, i) as ptrdiff_t),
                   Qutf_8 (), false); }                                 // C 453-457
          _ => {}
        }
      /* C 459-460: blank.  */

      values = Fcons (v, values);                                       // C 461
    }
  /* C 462-463: end of the loop / blank.  */

  return values;                                                        // C 464
}
/* C 465: end of `row_to_value`.  */

/* C 466: blank.  */
/* C 467: `static Lisp_Object sqlite_prepare_errdata (int code, sqlite3 *sdb)`.  */
#[cfg (feature = "have_sqlite3")]
unsafe fn sqlite_prepare_errdata (code: c_int, sdb: *mut sqlite3) -> Lisp_Object
{
  let errcode: Lisp_Object = make_fixnum (code as EMACS_INT);           // C 470
  let errmsg: *const c_char = sqlite3_errmsg (sdb);                     // C 471
  let lerrmsg: Lisp_Object = if errmsg.is_null () { Qnil () } else { build_string (errmsg) };  // C 472
  let errstr: Lisp_Object;                                              // C 473
  let ext_errcode: Lisp_Object;                                         // C 474
  /* C 475-480: `#if SQLITE_VERSION_NUMBER >= 3007015` / … / `#else` / … / `#endif`.  */
  #[cfg (feature = "sqlite_version_number_ge_3007015")]
  { errstr = build_string (sqlite3_errstr (code)); }                    // C 476
  #[cfg (not (feature = "sqlite_version_number_ge_3007015"))]
  {
    /* C 478: `/* The internet says this is identical to sqlite3_errstr (code).  */`  */
    errstr = lerrmsg;                                                   // C 479
  }
  /* C 481: blank.  */
  /* C 482: `/* More details about what went wrong.  */`  */
  /* C 483-488: `#if SQLITE_VERSION_NUMBER >= 3006005` / … / `#else` / … / `#endif`.  */
  #[cfg (feature = "sqlite_version_number_ge_3006005")]
  { ext_errcode = make_fixnum (sqlite3_extended_errcode (sdb) as EMACS_INT); }  // C 484
  #[cfg (not (feature = "sqlite_version_number_ge_3006005"))]
  {
    /* C 486: `/* What value to use here?  */`  */
    ext_errcode = make_fixnum (0);                                      // C 487
  }
  /* C 489: blank.  */

  return list4 (errstr, lerrmsg, errcode, ext_errcode);                 // C 490
}
/* C 491: end of `sqlite_prepare_errdata`.  */

/* C 492: blank.  */
/* C 493: `DEFUN ("sqlite-execute", Fsqlite_execute, Ssqlite_execute, 2, 3, 0, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-execute", Fsqlite_execute, Ssqlite_execute, 2, 3, 0,
       doc: "Execute a non-select SQL statement.\nIf VALUES is non-nil, it should be a vector or a list of values\nto bind when executing a statement like\n\n   insert into foo values (?, ?, ...)\n\nValue is the number of affected rows.  ",
       (db: Lisp_Object, query: Lisp_Object, values: Lisp_Object),
       {
  check_sqlite (db, false);                                             // C 503
  CHECK_STRING (query);                                                 // C 504
  if ! (NILP (values) || CONSP (values) || VECTORP (values))            // C 505
    { xsignal1 (Qsqlite_error (), build_string (c"VALUES must be a list or a vector".as_ptr ())); }  // C 506
  /* C 507: blank.  */

  let sdb: *mut sqlite3 = (*XSQLITE (db)).db as *mut sqlite3;           // C 508
  let mut errmsg: Lisp_Object = Qnil ();                                // C 509
  let encoded: Lisp_Object = encode_string (query);                     // C 510
  let mut stmt: *mut sqlite3_stmt = std::ptr::null_mut ();              // C 511
  /* C 512: blank.  */

  /* C 513-515: `/* We only execute the first statement -- if there's several
     (separated by a semicolon), the subsequent statements won't be
     done.  */`  */
  let mut ret: c_int = sqlite3_prepare_v2 (sdb, SSDATA (encoded), -1, &mut stmt,
                                           std::ptr::null_mut ());      // C 516
  /* C 517-559 and C 561's `exit:` label: C jumps out of the two error paths, PORTING.md 2.2's
     labelled block.  The fall-through is C's (the `else` at C 558 is the last statement of the
     block and leaves the label's own body to run next).  */
  'exit: {
    if ret != SQLITE_OK                                                 // C 517
      {
        if ! stmt.is_null ()                                            // C 519
          {
            sqlite3_finalize (stmt);                                    // C 521
            sqlite3_reset (stmt);                                       // C 522
          }
        /* C 524: blank.  */

        errmsg = sqlite_prepare_errdata (ret, sdb);                     // C 525
        break 'exit;                                                    // C 526's `goto exit;`
      }
    /* C 528: blank.  */
    /* C 529: `/* Bind ? values.  */`  */
    if ! NILP (values)                                                  // C 530
      {
        let err: *const c_char = bind_values (sdb, stmt, values);       // C 532
        if ! err.is_null ()                                             // C 533
          {
            errmsg = build_string (err);                                // C 535
            break 'exit;                                                // C 536's `goto exit;`
          }
      }
    /* C 538: blank.  */

    ret = sqlite3_step (stmt);                                          // C 540
    /* C 541: blank.  */

    if ret == SQLITE_ROW                                                // C 542
      {
        let mut data: Lisp_Object = Qnil ();                            // C 544
        loop                                                            // C 545-547's do-while
          {
            data = Fcons (row_to_value (stmt), data);                   // C 546
            if sqlite3_step (stmt) != SQLITE_ROW { break; }             // C 547
          }

        sqlite3_finalize (stmt);                                        // C 549
        return Fnreverse (data);                                        // C 550
      }
    else if ret == SQLITE_OK || ret == SQLITE_DONE                      // C 552
      {
        let rows: Lisp_Object = make_fixnum (sqlite3_changes (sdb) as EMACS_INT);  // C 554
        sqlite3_finalize (stmt);                                        // C 555
        return rows;                                                    // C 556
      }
    else
      { errmsg = build_string (sqlite3_errmsg (sdb)); }                 // C 559
  }
  /* C 560: blank.  */
  /* C 561: `exit:`  */
  sqlite3_finalize (stmt);                                              // C 562
  xsignal1 (if ret == SQLITE_LOCKED || ret == SQLITE_BUSY              // C 563-565
            { Qsqlite_locked_error () } else { Qsqlite_error () },
            errmsg);
}
/* C 566: end of `Fsqlite_execute`.  */);

/* C 567: blank.  */
/* C 568: `static Lisp_Object column_names (sqlite3_stmt *stmt)`.  */
#[cfg (feature = "have_sqlite3")]
unsafe fn column_names (stmt: *mut sqlite3_stmt) -> Lisp_Object
{
  let mut columns: Lisp_Object = Qnil ();                               // C 571
  let count: c_int = sqlite3_column_count (stmt);                       // C 572
  for i in 0 .. count                                                     // C 573
    { columns = Fcons (build_string (sqlite3_column_name (stmt, i)), columns); }  // C 574
  /* C 575: blank.  */

  return Fnreverse (columns);                                           // C 576
}
/* C 577: end of `column_names`.  */

/* C 578: blank.  */
/* C 579: `DEFUN ("sqlite-select", Fsqlite_select, Ssqlite_select, 2, 4, 0, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-select", Fsqlite_select, Ssqlite_select, 2, 4, 0,
       doc: "Select data from the database DB that matches QUERY.\nIf VALUES is non-nil, it should be a list or a vector specifying the\nvalues that will be interpolated into a parameterized statement.\n\nBy default, the return value is a list, whose contents depend on\nthe value of the optional argument RETURN-TYPE.\n\nIf RETURN-TYPE is nil or omitted, the function returns a list of rows\nmatching QUERY.  If RETURN-TYPE is `full', the function returns a\nlist whose first element is the list of column names, and the rest\nof the elements are the rows matching QUERY.  If RETURN-TYPE is `set',\nthe function returns a set object that can be queried with functions\nlike `sqlite-next' etc., in order to get the data.  ",
       (db: Lisp_Object, query: Lisp_Object, values: Lisp_Object, return_type: Lisp_Object),
       {
  check_sqlite (db, false);                                             // C 596
  CHECK_STRING (query);                                                 // C 597
  /* C 598: blank.  */

  if ! (NILP (values) || CONSP (values) || VECTORP (values))            // C 599
    { xsignal1 (Qsqlite_error (), build_string (c"VALUES must be a list or a vector".as_ptr ())); }  // C 600
  /* C 601: blank.  */

  let sdb: *mut sqlite3 = (*XSQLITE (db)).db as *mut sqlite3;           // C 602
  let mut retval: Lisp_Object = Qnil ();                                // C 603
  let mut errmsg: Lisp_Object = Qnil ();                                // C 603
  let encoded: Lisp_Object = encode_string (query);                     // C 604
  /* C 605: blank.  */

  let mut stmt: *mut sqlite3_stmt = std::ptr::null_mut ();              // C 606
  let mut ret: c_int = sqlite3_prepare_v2 (sdb, SSDATA (encoded), SBYTES (encoded) as c_int,
                                           &mut stmt, std::ptr::null_mut ());  // C 607-608
  'exit: {
    if ret != SQLITE_OK                                                 // C 609
      {
        if ! stmt.is_null ()                                            // C 611
          { sqlite3_finalize (stmt); }                                  // C 612
        errmsg = sqlite_prepare_errdata (ret, sdb);                     // C 613
        break 'exit;                                                    // C 614's `goto exit;`
      }
    /* C 615: blank.  */

    /* C 617: `/* Query with parameters.  */`  */
    if ! NILP (values)                                                  // C 618
      {
        let err: *const c_char = bind_values (sdb, stmt, values);       // C 620
        if ! err.is_null ()                                             // C 621
          {
            sqlite3_finalize (stmt);                                    // C 623
            errmsg = build_string (err);                                // C 624
            break 'exit;                                                // C 625's `goto exit;`
          }
      }
    /* C 627: blank.  */

    /* C 629: `/* Return a handle to get the data.  */`  */
    if EQ (return_type, Qset ())                                        // C 630
      {
        retval = make_sqlite (true, sdb as *mut c_void, stmt as *mut c_void,
                              (*XSQLITE (db)).name);                    // C 632
        break 'exit;                                                    // C 633's `goto exit;`
      }
    /* C 635: blank.  */

    /* C 636: `/* Return the data directly.  */`  */
    let mut data: Lisp_Object = Qnil ();                                // C 637
    while sqlite3_step (stmt) == SQLITE_ROW                            // C 638
      { data = Fcons (row_to_value (stmt), data); }                     // C 639
    /* C 640: blank.  */

    if EQ (return_type, Qfull ())                                       // C 641
      { retval = Fcons (column_names (stmt), Fnreverse (data)); }        // C 642
    else
      { retval = Fnreverse (data); }                                    // C 644
    sqlite3_finalize (stmt);                                            // C 645
  }
  /* C 646: blank.  */
  /* C 647: `exit:`  */
  if ! NILP (errmsg)                                                    // C 648
    { xsignal1 (Qsqlite_error (), errmsg); }                            // C 649
  /* C 650: blank.  */

  return retval;                                                        // C 651
});
/* C 652: end of `Fsqlite_select`.  */

/* C 653: blank.  */
/* C 654: `static Lisp_Object sqlite_exec (sqlite3 *sdb, const char *query)`.  */
#[cfg (feature = "have_sqlite3")]
unsafe fn sqlite_exec (sdb: *mut sqlite3, query: *const c_char) -> Lisp_Object
{
  let ret: c_int = sqlite3_exec (sdb, query, None, std::ptr::null_mut (),
                                std::ptr::null_mut ());                 // C 657
  if ret != SQLITE_OK                                                   // C 658
    { return Qnil (); }                                                 // C 659
  /* C 660: blank.  */

  return Qt ();                                                         // C 661
}
/* C 662: end of `sqlite_exec`.  */

/* C 663: blank.  */
/* C 664: `DEFUN ("sqlite-execute-batch", Fsqlite_execute_batch, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-execute-batch", Fsqlite_execute_batch, Ssqlite_execute_batch, 2, 2, 0,
       doc: "Execute multiple SQL STATEMENTS in DB.\nSTATEMENTS is a string containing 0 or more SQL statements.  ",
       (db: Lisp_Object, statements: Lisp_Object),
       {
  check_sqlite (db, false);                                             // C 669
  CHECK_STRING (statements);                                            // C 670
  let encoded: Lisp_Object = encode_string (statements);                // C 671
  return sqlite_exec ((*XSQLITE (db)).db as *mut sqlite3,
                      SSDATA (encoded) as *const c_char);               // C 672
});

/* C 674: blank.  */
/* C 675: `DEFUN ("sqlite-transaction", Fsqlite_transaction, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-transaction", Fsqlite_transaction, Ssqlite_transaction, 1, 1, 0,
       doc: "Start a transaction in DB.  ",
       (db: Lisp_Object),
       {
  check_sqlite (db, false);                                             // C 679
  return sqlite_exec ((*XSQLITE (db)).db as *mut sqlite3, c"begin".as_ptr ());  // C 680
});

/* C 682: blank.  */
/* C 683: `DEFUN ("sqlite-commit", Fsqlite_commit, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-commit", Fsqlite_commit, Ssqlite_commit, 1, 1, 0,
       doc: "Commit a transaction in DB.  ",
       (db: Lisp_Object),
       {
  check_sqlite (db, false);                                             // C 687
  return sqlite_exec ((*XSQLITE (db)).db as *mut sqlite3, c"commit".as_ptr ());  // C 688
});

/* C 690: blank.  */
/* C 691: `DEFUN ("sqlite-rollback", Fsqlite_rollback, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-rollback", Fsqlite_rollback, Ssqlite_rollback, 1, 1, 0,
       doc: "Roll back a transaction in DB.  ",
       (db: Lisp_Object),
       {
  check_sqlite (db, false);                                             // C 695
  return sqlite_exec ((*XSQLITE (db)).db as *mut sqlite3, c"rollback".as_ptr ());  // C 696
});

/* C 698: blank.  */
/* C 699: `DEFUN ("sqlite-pragma", Fsqlite_pragma, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-pragma", Fsqlite_pragma, Ssqlite_pragma, 2, 2, 0,
       doc: "Execute PRAGMA in DB.  ",
       (db: Lisp_Object, pragma: Lisp_Object),
       {
  check_sqlite (db, false);                                             // C 703
  CHECK_STRING (pragma);                                                // C 704
  /* C 705: blank.  */

  return sqlite_exec ((*XSQLITE (db)).db as *mut sqlite3,
                      SSDATA (concat2 (build_string (c"PRAGMA ".as_ptr ()), pragma))
                      as *const c_char);                                // C 706-707
});

/* C 709: blank.  */
/* C 710: `#if HAVE_LOAD_EXTENSION` - C 39-40's computed macro, spelled as its conjunction.  */
/* C 711: `DEFUN ("sqlite-load-extension", Fsqlite_load_extension, …)`.  */
#[cfg (all (feature = "have_sqlite3", feature = "have_sqlite3_load_extension",
            feature = "sqlite3_dbconfig_enable_load_extension"))]
DEFUN! ("sqlite-load-extension", Fsqlite_load_extension, Ssqlite_load_extension, 2, 2, 0,
       doc: "Load an SQlite MODULE into DB.\nMODULE should be the name of an SQlite module's file, a\nshared library in the system-dependent format and having a\nsystem-dependent file-name extension.\n\nOnly modules on Emacs's list of allowed modules can be loaded.  ",
       (db: Lisp_Object, module: Lisp_Object),
       {
  check_sqlite (db, false);                                             // C 721
  CHECK_STRING (module);                                                // C 722
  /* C 723: blank.  */

  /* C 724: `/* Add names of useful and free modules here.  */`  */
  let allowlist: [*const c_char; 20] = [                                 // C 725-746
    c"base64".as_ptr (),
    c"cksumvfs".as_ptr (),
    c"compress".as_ptr (),
    c"csv".as_ptr (),
    c"csvtable".as_ptr (),
    c"fts3".as_ptr (),
    c"icu".as_ptr (),
    c"pcre".as_ptr (),
    c"percentile".as_ptr (),
    c"regexp".as_ptr (),
    c"rot13".as_ptr (),
    c"rtree".as_ptr (),
    c"sha1".as_ptr (),
    c"uuid".as_ptr (),
    c"vec0".as_ptr (),
    c"vector0".as_ptr (),
    c"vfslog".as_ptr (),
    c"vss0".as_ptr (),
    c"zipfile".as_ptr (),
    std::ptr::null ()
  ];
  let mut name: *mut c_char = SSDATA (Ffile_name_nondirectory (module));  // C 747
  /* C 748-749: `/* Possibly skip past a common prefix (libsqlite3_mod_ is used by
     Debian, see https://packages.debian.org/source/sid/sqliteodbc).  */`  */
  let prefix: *const c_char = c"libsqlite3_mod_".as_ptr ();              // C 750
  if strncmp (name, prefix, strlen (prefix)) == 0                       // C 751
    { name = name.add (strlen (prefix)); }                              // C 752
  /* C 753: blank.  */

  let mut do_allow: bool = false;                                       // C 754
  /* C 756: `for (const char **allow = allowlist; *allow; allow++)` - the array's NULL
     terminator is the loop's condition, so the port walks the same array.  */
  let mut allow_i: usize = 0;
  while ! allowlist[allow_i].is_null ()
    {
      let allow: *const c_char = allowlist[allow_i];
      let allow_len: ptrdiff_t = strlen (allow) as ptrdiff_t;           // C 758
      if allow_len < strlen (name) as ptrdiff_t                         // C 759
        && strncmp (allow, name, allow_len as usize) == 0               // C 760
        && (strcmp (name.offset (allow_len), c".so".as_ptr ()) == 0     // C 761
            || strcmp (name.offset (allow_len), c".dylib".as_ptr ()) == 0     // C 762
            || strcasecmp (name.offset (allow_len), c".dll".as_ptr ()) == 0)  // C 763
        {
          do_allow = true;                                              // C 765
          break;                                                        // C 766
        }
      allow_i += 1;
    }
  /* C 768: blank.  */

  if ! do_allow                                                         // C 770
    { xsignal1 (Qsqlite_error (), build_string (c"Module name not on allowlist".as_ptr ())); }  // C 771
  /* C 772: blank.  */

  /* C 773-775: `/* Expand all Lisp data explicitly, so as to avoid signaling an
     error while extension loading is enabled -- we don't want to
     "leak" this outside this function.  */`  */
  let sdb: *mut sqlite3 = (*XSQLITE (db)).db as *mut sqlite3;           // C 775
  let ext_fn: *mut c_char = SSDATA (ENCODE_FILE (Fexpand_file_name (module, Qnil ())));  // C 776
  /* C 777: `/* Temporarily enable loading extensions via the C API.  */`  */
  let mut result: c_int = sqlite3_db_config (sdb, SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION, 1,
                                             std::ptr::null_mut::<c_int> ());  // C 778-779
  if result == SQLITE_OK                                                // C 780
    {
      result = sqlite3_load_extension (sdb, ext_fn, std::ptr::null (),
                                       std::ptr::null_mut ());          // C 781
      /* C 782: `/* Disable loading extensions via C API.  */`  */
      sqlite3_db_config (sdb, SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION, 0,
                         std::ptr::null_mut::<c_int> ());               // C 784
      if result == SQLITE_OK                                            // C 785
        { return Qt (); }                                               // C 786
    }
  return Qnil ();                                                       // C 788
});
/* C 789: end of `Fsqlite_load_extension`.  */
/* C 790: `#endif /* HAVE_LOAD_EXTENSION */`.  */

/* `<strings.h>` 81 declares `strcasecmp`, which C 763 calls; no mirror carries it, so this unit
   declares it at its use site (P-88 ①; `src/term.rs` 209 and `src/sysdep.rs` 731 are the same
   shape).  */
/// C: `<strings.h>` 81: `int	 strcasecmp(const char *, const char *);`
#[cfg (all (feature = "have_sqlite3", feature = "have_sqlite3_load_extension",
            feature = "sqlite3_dbconfig_enable_load_extension"))]
unsafe extern "C" {
  pub fn strcasecmp (s1: *const c_char, s2: *const c_char) -> c_int;
}

/* C 791: blank.  */
/* C 792: `DEFUN ("sqlite-next", Fsqlite_next, Ssqlite_next, 1, 1, 0, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-next", Fsqlite_next, Ssqlite_next, 1, 1, 0,
       doc: "Return the next result set from SET.\nReturn nil when the statement has finished executing successfully.  ",
       (set: Lisp_Object),
       {
  check_sqlite (set, true);                                             // C 797
  /* C 798: blank.  */

  if (*XSQLITE (set)).eof                                               // C 799
    { return Qnil (); }                                                 // C 800
  /* C 801: blank.  */

  let ret: c_int = sqlite3_step ((*XSQLITE (set)).stmt as *mut sqlite3_stmt);  // C 802
  if ret != SQLITE_ROW && ret != SQLITE_OK && ret != SQLITE_DONE        // C 803
    { xsignal1 (Qsqlite_error (),
                build_string (sqlite3_errmsg ((*XSQLITE (set)).db as *mut sqlite3))); }  // C 804
  /* C 805: blank.  */

  if ret == SQLITE_DONE                                                 // C 806
    {
      (*XSQLITE (set)).eof = true;                                      // C 808
      return Qnil ();                                                   // C 809
    }
  /* C 810-811: blank.  */

  return row_to_value ((*XSQLITE (set)).stmt as *mut sqlite3_stmt);      // C 812
});

/* C 814: blank.  */
/* C 815: `DEFUN ("sqlite-columns", Fsqlite_columns, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-columns", Fsqlite_columns, Ssqlite_columns, 1, 1, 0,
       doc: "Return the column names of SET.  ",
       (set: Lisp_Object),
       {
  check_sqlite (set, true);                                             // C 819
  return column_names ((*XSQLITE (set)).stmt as *mut sqlite3_stmt);      // C 820
});

/* C 822: blank.  */
/* C 823: `DEFUN ("sqlite-more-p", Fsqlite_more_p, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-more-p", Fsqlite_more_p, Ssqlite_more_p, 1, 1, 0,
       doc: "Say whether there are any further results in SET.  ",
       (set: Lisp_Object),
       {
  check_sqlite (set, true);                                             // C 827
  /* C 828: blank.  */

  if (*XSQLITE (set)).eof                                               // C 829
    { return Qnil (); }                                                 // C 830
  else
    { return Qt (); }                                                   // C 832
});

/* C 834: blank.  */
/* C 835: `DEFUN ("sqlite-finalize", Fsqlite_finalize, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-finalize", Fsqlite_finalize, Ssqlite_finalize, 1, 1, 0,
       doc: "Mark this SET as being finished.\nThis will free the resources held by SET.  ",
       (set: Lisp_Object),
       {
  check_sqlite (set, true);                                             // C 840
  sqlite3_finalize ((*XSQLITE (set)).stmt as *mut sqlite3_stmt);        // C 841
  (*XSQLITE (set)).db = std::ptr::null_mut ();                          // C 842
  return Qt ();                                                         // C 843
});

/* C 845: blank.  */
/* C 846: `DEFUN ("sqlite-version", Fsqlite_version, …)`.  */
#[cfg (feature = "have_sqlite3")]
DEFUN! ("sqlite-version", Fsqlite_version, Ssqlite_version, 0, 0, 0,
       doc: "Return the version string of the SQLite library.\nSignal an error if SQLite support is not available.  ",
       (),
       {
  if ! init_sqlite_functions ()                                         // C 851
    { error (c"sqlite support is not available".as_ptr ()); }           // C 852
  return build_string (sqlite3_libversion ());                          // C 853
});

/* C 855: blank.  */
/* C 856: `#endif /* HAVE_SQLITE3 */` - the body's gate closes here, as in C: everything below is
   compiled in the frozen configuration too.  */

/* C 857: blank.  */
/* C 858: `DEFUN ("sqlitep", Fsqlitep, Ssqlitep, 1, 1, 0, …)` - outside the file gate, so the
   `DEFUN!` itself is not gated (its name and record exist in the frozen build, as C's do).  */
DEFUN! ("sqlitep", Fsqlitep, Ssqlitep, 1, 1, 0,
       doc: "Say whether OBJECT is an SQlite object.  ",
       (object: Lisp_Object),
       {
  /* C 862-866: `#ifdef HAVE_SQLITE3` / `return SQLITE (object)? Qt: Qnil;` / `#else` /
     `return Qnil;` / `#endif`.  */
  #[cfg (feature = "have_sqlite3")]
  { return if SQLITE (object) { Qt () } else { Qnil () }; }             // C 863
  #[cfg (not (feature = "have_sqlite3"))]
  { return Qnil (); }                                                   // C 865
});

/* C 868: blank.  */
/* C 869: `DEFUN ("sqlite-available-p", Fsqlite_available_p, …)`.  */
DEFUN! ("sqlite-available-p", Fsqlite_available_p, Ssqlite_available_p, 0, 0, 0,
       doc: "Return t if sqlite3 support is available in this instance of Emacs.",
       (),
       {
  /* C 873-885: the `#ifdef HAVE_SQLITE3` arm (with its own `#ifdef WINDOWSNT` at C 874-882) and
     the `#else` arm.  */
  #[cfg (feature = "have_sqlite3")]
  {
    #[cfg (feature = "windowsnt")]
    {
      let found: Lisp_Object = Fassq (Qsqlite3 (), Vlibrary_cache);     // C 875
      if CONSP (found)                                                  // C 876
        { return XCDR (found); }                                        // C 877
      else
        { return if init_sqlite_functions () { Qt () } else { Qnil () }; }  // C 879
    }
    #[cfg (not (feature = "windowsnt"))]
    { return Qt (); }                                                   // C 881
  }
  #[cfg (not (feature = "have_sqlite3"))]
  { return Qnil (); }                                                   // C 884
});

/* C 887: blank.  */
/* C 888: `void syms_of_sqlite (void)` - `lisp.h` 4417 declares it and it is the one symbol this
   unit exports in every configuration.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_sqlite ()
{
  /* C 891-911: `#ifdef HAVE_SQLITE3` - the fifteen `defsubr`s (C 892-908, with C 901-903's
     `#if HAVE_LOAD_EXTENSION` around `sqlite-load-extension`) and C 909-910's two `DEFSYM`s.  */
  #[cfg (feature = "have_sqlite3")]
  {
    defsubr (&raw mut Ssqlite_open);                                    // C 892
    defsubr (&raw mut Ssqlite_close);                                   // C 893
    defsubr (&raw mut Ssqlite_execute);                                 // C 894
    defsubr (&raw mut Ssqlite_select);                                  // C 895
    defsubr (&raw mut Ssqlite_execute_batch);                           // C 896
    defsubr (&raw mut Ssqlite_transaction);                             // C 897
    defsubr (&raw mut Ssqlite_commit);                                  // C 898
    defsubr (&raw mut Ssqlite_rollback);                                // C 899
    defsubr (&raw mut Ssqlite_pragma);                                  // C 900
    #[cfg (all (feature = "have_sqlite3_load_extension",
                feature = "sqlite3_dbconfig_enable_load_extension"))]
    defsubr (&raw mut Ssqlite_load_extension);                          // C 902
    defsubr (&raw mut Ssqlite_next);                                    // C 904
    defsubr (&raw mut Ssqlite_columns);                                 // C 905
    defsubr (&raw mut Ssqlite_more_p);                                  // C 906
    defsubr (&raw mut Ssqlite_finalize);                                // C 907
    defsubr (&raw mut Ssqlite_version);                                 // C 908
    DEFSYM! (Qset, "set");                                              // C 909
    DEFSYM! (Qfull, "full");                                            // C 910
  }
  /* C 911: `#endif`.  */
  defsubr (&raw mut Ssqlitep);                                          // C 912
  defsubr (&raw mut Ssqlite_available_p);                               // C 913
  /* C 914: blank.  */

  DEFSYM! (Qsqlite_error, "sqlite-error");                              // C 915
  Fput (Qsqlite_error (), Qerror_conditions (),
        list2 (Qsqlite_error (), Qerror ()));                           // C 916-917
  Fput (Qsqlite_error (), Qerror_message (),
        build_string (c"Database error".as_ptr ()));                    // C 918-919
  /* C 920: blank.  */

  DEFSYM! (Qsqlite_locked_error, "sqlite-locked-error");                // C 921
  Fput (Qsqlite_locked_error (), Qerror_conditions (),
        list3 (Qsqlite_locked_error (), Qsqlite_error (), Qerror ()));  // C 922-923
  Fput (Qsqlite_locked_error (), Qerror_message (),
        build_string (c"Database locked".as_ptr ()));                   // C 924-925
  /* C 926: blank.  */

  DEFSYM! (Qsqlitep, "sqlitep");                                        // C 927
  DEFSYM! (Qfalse, "false");                                            // C 928
  DEFSYM! (Qsqlite, "sqlite");                                          // C 929
  DEFSYM! (Qsqlite3, "sqlite3");                                        // C 930
  DEFSYM! (Qbinary, "binary");                                          // C 931
  DEFSYM! (Qcoding_system, "coding-system");                            // C 932
}
/* C 933: end of `syms_of_sqlite`, and of the reference file.  */
