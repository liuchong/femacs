// femacs: port of lib/dirent-private.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  gnulib's private `DIR` record, one arm per system that makes
// gnulib replace `<dirent.h>`: C 17-18 is the include guard, C 20-34 the "mingw" arm (which
// `#undef`s `DIR` and redefines it as this record) and C 36-63 the "MSVC" arm (which pulls
// `<windows.h>` and finds entries with `FindFirstFile`).  The file has no other content, and its
// only C consumers are gnulib's own `dirent.c`/`readdir.c`, which the frozen build does not
// compile (`docs/baseline/frozen-build-sources.txt`; the platform object variables are empty) -
// so no unit `include!`s this mirror and its acceptance is `--check` plus the record below.
//
// **Conditions.**  `tools/check-cond-eval.py lib/dirent-private.h` -> `2 conditions, 0 unknown to
// the evaluator, 2 disagreements`: the file is not on
// docs/baseline/config/cond-eval-verified.txt, so the extractor resolves nothing and *both* arms
// count as live for it.  The frozen macro table does decide the switch, though: `HAVE_DIRENT_H`
// is 1 (`docs/baseline/config/darwin-arm64.macros.tsv`), so C 21-34 is the arm this configuration
// would compile and C 36-63 is dead text - carried here under `#[cfg (feature = "msvc")]`
// (PORTING.md P-51 names the non-frozen side; `msvc` is the compiler the arm's own comment names).
//
// **The extractor's field list merges the two arms** (it sees `int fd_to_close, DIR *real_dirp,
// int fd_to_close, int status, HANDLE current, WIN32_FIND_DATA entry, char dir_name_mask`, with
// `fd_to_close` twice), because a mirror of this file has to answer for both.  The record below
// therefore keeps C's member order per arm and puts the MSVC-only members behind the same `cfg`,
// which is also what gives each configuration C's own layout.  Nothing asserts those layouts: no
// compiler on this target has `<windows.h>`, and the record is not compiled here at all (the
// `include/w32heap.rs` `file_data` rule, from the other side).
//
// `HANDLE` and `WIN32_FIND_DATA`/`WIN32_FIND_DATAA` are `<windows.h>`'s (C 39-43); the two names
// keep C's spelling and are left for a Windows target's unit to supply (P-72 rule 3, the
// `lwlib/include/xlwmenuP.rs` 15-23 reading).  `DIR` is `lib/include/dirent.rs` 9's opaque enum
// for this target.

// C 17-18: `#define _DIRENT_PRIVATE_H 1` - the include guard, with C's `1` as its value.
pub const _DIRENT_PRIVATE_H: c_int = 1;

/// C 34: `# define DIR struct gl_directory` - the *live* arm's redefinition of the system's
/// `DIR` (C 22 `# undef DIR` first).  A *type* macro, so it is a nullary `macro_rules!` whose body
/// names the record (the `include/w32gui.rs` `NativeRectangle` / `include/msdos.rs` `PIX_TYPE`
/// spelling).  `lib/include/dirent.in.rs` 107 spells the *same* redefinition for the other
/// configuration's fallback arm under its own `cfg`, which is why this one is left un-gated: the
/// two never stand together.
/// C: # define DIR struct gl_directory
macro_rules! DIR { () => { gl_directory }; }

/// C 24-31: the mingw arm's record - the file descriptor `closedir` needs, and the system's real
/// `DIR`.  This is the arm the frozen configuration would compile.
/// C: struct gl_directory
/// C: {
/// C:   int fd_to_close;
/// C:   DIR *real_dirp;
/// C: };
#[repr(C)]
pub struct gl_directory
{
  pub fd_to_close: c_int,

  pub real_dirp: *mut DIR,

  /* ---- C 45-63: the MSVC arm's members, dead on this target (see the file note) ---------- */

  /// C 55: `int status;` - "status, or error code to produce in next `readdir()` call".
  #[cfg (feature = "msvc")]
  pub status: c_int,

  /// C 57: `HANDLE current;` - the handle the directory is read through.
  /// C: `HANDLE current;`
  #[cfg (feature = "msvc")]
  pub current: *mut c_void,

  /// C 59: `WIN32_FIND_DATA entry;` - the found entry; the record is `<windows.h>`'s.
  /// C: `WIN32_FIND_DATA entry;`
  #[cfg (feature = "msvc")]
  pub entry: WIN32_FIND_DATA,

  /// C 62: `char dir_name_mask[1];` - the absolutized directory name plus separator and
  /// wildcards, a one-element array in C (the flexible-array idiom).
  #[cfg (feature = "msvc")]
  pub dir_name_mask: [c_char; 1],
}

/// C 38: `# define WIN32_LEAN_AND_MEAN` - the marker macro `<windows.h>` reads; the port spells a
/// marker as the constant that says "defined" (the `nt/inc/include/ms-w32.rs` 147 `_WINSOCK_H`
/// spelling).
/// C: # define WIN32_LEAN_AND_MEAN
#[cfg (feature = "msvc")]
pub const WIN32_LEAN_AND_MEAN: bool = true;

/// C 41-43: "don't assume that `UNICODE` is not defined" - C `#undef`s the name and points it at
/// the ANSI record.  A *type* macro, so a nullary `macro_rules!`; the target name is `<windows.h>`'s.
/// C: # undef WIN32_FIND_DATA
/// C: # define WIN32_FIND_DATA WIN32_FIND_DATAA
#[cfg (feature = "msvc")]
macro_rules! WIN32_FIND_DATA { () => { WIN32_FIND_DATAA }; }
