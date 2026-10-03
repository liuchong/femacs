# emacs/src/dired.c (1213 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 8
- [x] directory-files  (Fdirectory_files, 1/5, doc 20326978fc86, line 379)
- [x] directory-files-and-attributes  (Fdirectory_files_and_attributes, 1/6, doc 95cb528b21fa, line 407)
- [x] file-name-completion  (Ffile_name_completion, 2/3, doc bc5b9a3d9837, line 451)
- [x] file-name-all-completions  (Ffile_name_all_completions, 2/2, doc 01f665e62d91, line 486)
- [x] file-attributes  (Ffile_attributes, 1/2, doc 26dc0faff293, line 953)
- [x] file-attributes-lessp  (Ffile_attributes_lessp, 2/2, doc 34e2b65ae974, line 1137)
- [x] system-users  (Fsystem_users, 0/0, doc ffae00cd4131, line 1147)
- [x] system-groups  (Fsystem_groups, 0/0, doc f34dfc4149a9, line 1168)

## Functions defined — 13
- [x] dirent_namelen  (line 72, static)
- [x] dirent_type  (line 97, static)
- [x] open_directory  (line 107, static)
- [x] directory_files_internal_unwind  (line 178, static)
- [x] read_dirent  (line 188, static)
- [x] directory_files_internal  (line 218)
- [x] file_name_completion  (line 517, static)
- [x] scmp  (line 882, static)
- [x] file_name_completion_dirp  (line 910, static)
- [x] stat_uname  (line 924, static)
- [x] stat_gname  (line 939, static)
- [x] file_attributes  (line 1027, static)
- [x] syms_of_dired  (line 1186)

## Macros defined — 3
- [x] TRIVIAL_DIRECTORY_ENTRY  (line 645, body 0acec0adb551)
- [x] emacs_closedir  (line 50, body 448a0f12f99f)
- [x] emacs_readdir  (line 49, body 5d99e9e7c294)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/dired.c` @ `fd7afced403`; port `femacs/src/dired.rs` (1162 lines vs C's 1213,
1067 live - the 8.9% drift against the live baseline is declared in
`docs/ports/line-drift-tolerance.tsv`); the gate is
`python3 tools/extract-abi.py --check emacs/src/dired.c femacs/src/dired.rs` -> `OK`, and
`build-tu src/dired.c` prints `built`.  `src/dired.c` is in
`docs/baseline/frozen-build-sources.txt`, so the frozen configuration compiles it: there is no
3.31 exemption here and the full comparison applies.

### Dead text (not transcribed)

The frozen configuration leaves `MSDOS`, `WINDOWSNT`, `HAVE_ANDROID`, `ANDROID_STUBIFY`,
`_D_EXACT_NAMLEN` and `O_PATH` undefined, `HAVE_PWD_H`/`HAVE_STRUCT_DIRENT_D_TYPE`/
`HAVE_GETPWENT`/`HAVE_ENDPWENT`/`HAVE_GETGRENT`/`HAVE_ENDGRENT` defined
(`docs/baseline/config/darwin-arm64.macros.tsv`).  147 of the file's 1213 lines are dead and
are recorded here instead of transcribed (P-08; dired.c is a `src/` file, so no dead text is
kept as `#[cfg (feature = ...)]`):

- C 43-45: `#ifdef MSDOS`'s `#include "msdos.h"`.
- C 51-60: the `#else` of `#if !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)` -
  `"android.h"`, the Android `README`-comment, `typedef struct android_vdir emacs_dir;` and
  the two `android_*` `#define`s; C 47-50 (the `typedef DIR` and the two real `#define`s) is
  the live arm and is transcribed.
- C 62-64: `#ifdef WINDOWSNT`'s `extern int is_slow_fs (const char *);`.
- C 74-75: `dirent_namelen`'s `_D_EXACT_NAMLEN` arm.  This is not a condition-table gap: clang
  agrees the macro is undefined on the frozen target (the SDK's `<dirent.h>` defines no
  `_D_EXACT_NAMLEN`; `python3 tools/check-cond-eval.py src/dired.c` reports
  `25 conditions, 0 unknown, 0 disagreements [VERIFIED]`), so no
  `source-header-macros.tsv` row was needed and C 77's `return strlen (dp->d_name);` is the
  live arm.
- C 81-92: `#ifndef HAVE_STRUCT_DIRENT_D_TYPE`'s `enum { DT_UNKNOWN, DT_DIR, DT_LNK }`, its
  two `#elif`s and the `#error` (HAVE_STRUCT_DIRENT_D_TYPE is 1, so the three names are the
  system header's macros and are declared as constants below).
- C 101-102: `dirent_type`'s `#else return DT_UNKNOWN;` arm.
- C 113-146: `open_directory`'s `#if defined DOS_NT || (defined HAVE_ANDROID && ...)` arm
  (`fd = 0`, `opendir`, `opendir_errno`, the Android `again:` loop with its `goto`); C 148's
  `#else` body is the live one.
- C 169-175: `directory_files_internal_w32_unwind`.
- C 198-204: `read_dirent`'s `ENOENT`/`EACCES` arm.
- C 256-274: `directory_files_internal`'s `w32_save` block (and `Vw32_get_true_file_attributes`
  / `is_slow_fs` / `Qlocal` with it).
- C 293-294: the `case_table` assignment.
- C 359-361: the `Vw32_get_true_file_attributes = w32_save;` restore.
- C 629-639: the `#if 0` block in `file_name_completion` (a FIXME about `scmp` comparing an
  encoded with a decoded string).  This is a literal-false condition, dead for every
  configuration.
- C 926-927 and C 941-942: `stat_uname`'s `st_uname` and `stat_gname`'s `st_gname` arms.
- C 1042-1065: `file_attributes`'s `#if defined O_PATH && !defined HAVE_CYGWIN_O_PATH_BUG &&
  !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)` block (`emacs_openat`, `O_PATH`,
  `O_CLOEXEC`, `O_NOFOLLOW`, `close_file_unwind`, `sys_fstat`, `EBADF`).  Its `#else` tail is
  C's `if (err == EINVAL)` body, which is transcribed.
- C 1070-1075 and C 1078-1079: the two `w32_stat_get_owner_group` arms.

### Includes, and the names declared at their use site

The include list is C 21-41's through the mirrors those pull in, including lisp.h 30's
`<string.h>` (`strlen`/`strcmp`/`memcpy`/`strcpy`) and the buffer.h/coding.h members
(`itree`, `character`, `intervals`, `composite`).  Two C includes are live but have no
mirror - `<pwd.h>` (C 26) and `<grp.h>` (C 28) - and the mirrors do not carry seven names
this file uses, so all of them are declared at the use site (the terminal.rs/filelock.rs
shape): `getpwuid`/`getpwent`/`endpwent` with C's `struct passwd`, `getgrgid`/`getgrent`/
`endgrent` with C's `struct group`, `emacs_fstatat` (lisp.h 5303), `sys_faccessat` (5307),
`check_emacs_readlinkat` (4986), `file_attribute_errno` (4984),
`internal_condition_case_2` (4807), `plist_get` (4280) and `re_match_object`
(regex-emacs.h 50; the regex-emacs mirror does not carry it).

- **`struct dirent` is declared here, not in the dirent mirror.**  That mirror carries the
  opaque `DIR` and the six directory calls and its own note says "`struct dirent` is not
  mirrored yet - no ported unit reads its fields"; this is the first one that does, so the
  record is declared at its use site with the frozen target's measured layout:
  `sizeof = 1048`, align 8, `d_ino@0`, `d_seekoff@8`, `d_reclen@16`, `d_namlen@18`,
  `d_type@20`, `d_name@21` (1024 bytes; probe:
  `clang -std=gnu23` over `offsetof (struct dirent, …)` against the SDK).  Three `const _`
  assertions pin the size and the two offsets the unit reads, because no generated layout
  baseline covers a system header's record and a wrong offset would be invisible to every
  gate.  If the dirent mirror ever gains the record, this declaration is the one to drop.
- **C's `struct passwd`/`struct group`** are declared with the frozen target's layout
  (`pw_name@0`, `pw_uid@16`; `gr_name@0`, `gr_gid@16`), measured the same way; this unit reads
  only `pw_name`/`gr_name`.
- **`DT_UNKNOWN`/`DT_DIR`/`DT_LNK`** are `0`/`4`/`10` on this target (macros.tsv 781/776/778);
  no mirror carries them and C's own enum of those names is dead.
- **`DIRECTORY_SEP` (`'/'`), `IS_ANY_SEP` and `IS_DIRECTORY_SEP`** come from `config.h`
  (120, 2091, 2097 - the spliced `s/` header's values, not `config.rs`), so they are declared
  here as a constant and two `macro_rules!`.
- **`EAGAIN`** (35 on this target, measured with clang) is the one `<errno.h>` name the errno
  mirror does not carry.
- **`emacs_readdir`/`emacs_closedir`** keep C's names as `macro_rules!` (P-37, the lisp.rs
  `emacs_fclose` precedent).  The dirent mirror declares `readdir` as returning `*mut c_void`
  and `closedir` as taking `*mut DIR`, so each macro spells the cast C's implicit conversion
  makes at the call site (`readdir (...) as *mut dirent`; `... as *mut DIR` for the
  `record_unwind_protect_ptr` callback's `void *`).

### Structural deviations

- **`match` is a Rust keyword.**  C's parameter name (`directory_files_internal` and the two
  `directory-files*` subrs) keeps its spelling as `r#match` (PORTING.md 0.2); the token
  comparison folds the `r#` prefix.
- **`while (true)` -> `loop` in `read_dirent`.**  A Rust `while` loop's type is `()` even with
  a literal `true` condition, and this loop is the function's tail (its only exit is the
  `return dp;` inside), so `while true` would not be the function's `*mut dirent`.  The token
  row is in `docs/ports/macro-body-notes.tsv`; the loop's statements and order are C's.
- **C's `switch (dirent_type (dp))` -> `match`** (PORTING.md 2.2): C's arms keep their order
  and the shared `DT_LNK`/`DT_UNKNOWN` body; `case`/`default`/`switch` have no counterpart
  (notes row).  C's `break` still appears in the body (the entry loop), so it is not exempted.
- **C's `for (…; CONSP (tem); tem = XCDR (tem))` loops** (the two `completion-ignored-extensions`
  scans) become `while` loops with C's step written before every `continue`, because Rust's
  `continue` would otherwise skip the for-increment (PORTING.md 2.2; the rawmemchr/strnlen
  precedent).  Both of C's bodies end in `break` (the "found a match" exit, which skips the
  increment), so no step is needed at the body's end; that `break` leaves `tem` on the
  matching element, which is what the `if (CONSP (tem)) canexclude = 1;` below tests, and the
  third such loop (the `completion-regexp-list` scan, whose body has no `continue`) keeps the
  step at the body's end.
- **`r#match`'s `for (…; (dp = read_dirent (…)); )`** loops are `while { dp = …; !dp.is_null
  () }`, keeping C's assignment-in-the-condition shape (the filelock `while { … }` precedent).
- **Rust has no postfix increment**: `ind ++` is `ind += 1`.
- **Chained assignment**: C's `encoded_file = encoded_dir = Qnil;` is two statements.
- **`INT_TO_INTEGER (expr)` is `(EXPR_SIGNED (expr) ? make_int (expr) : make_uint (expr))`**
  (lisp.h 3052-3053), and the lisp.rs `INT_TO_INTEGER` is the signed arm
  (`make_int (expr as intmax_t)`).  The port therefore keeps C's two arms explicitly: the
  signed expressions (`s.st_size`, `s.st_dev`) use `INT_TO_INTEGER`, the unsigned ones
  (`s.st_nlink`, `s.st_uid`, `s.st_gid`, `s.st_ino`; `EXPR_SIGNED` false) use `make_uint`,
  which is the arm C's macro expands to.  The `as isize`/`as uintmax_t`/`as c_int` casts are
  C's own implicit conversions made explicit (the mirror's `off_t` is `i64` while `EMACS_INT`
  is `isize`).
- **`(void) directory_volatile;`** keeps the volatile copy of DIRECTORY alive (Bug#16986); the
  port reads it back (`let _ = directory_volatile;`) for the same effect.
- **`UNINIT`** (`Lisp_Object fileattrs UNINIT;`) is lisp.h's marker for "assigned before use";
  the port writes the zero initializer (PORTING.md 1.5) and the token row is in
  macro-body-notes.tsv (the `cons_to_unsigned`/`skip_chars` precedent).  The other locals C
  declares and then assigns are declared without an initializer where every path assigns
  before the first read (`let mut bestmatch: Lisp_Object;`, `open_directory`'s `fd`/
  `opendir_errno`, `file_attributes`'s `file_type`), and zero-initialized where C's own zero is
  the first statement (`ind`, `last`, `list`, `name`, `tem`, `elt`, `skip`, `cmp_len`,
  `matchcount`, `bestmatchsize`, `canexclude`, `user`-less `uname`/`gname`, `err`, `s`,
  `modes`).
- **`modes[sizeof "-rwxr-xr-x "]`** is `[c_char; 12]` (11 bytes plus the NUL) with `sizeof`'s
  value spelled out, the `lock_info_type.user` precedent; `make_string (modes, 10)` is C's ten
  significant bytes.
- **C's `enum`/`bool_bf`/union machinery does not appear**: this file has no union, no
  bitfield and no struct of its own (the ledger's "Types and their fields - 0").
- **Explicit conversions Rust will not insert**: `needsep as ptrdiff_t` and
  `directoryp as ptrdiff_t` (C's `bool` promotions, including the
  `(matchsize + directoryp as ptrdiff_t) < SCHARS (bestmatch)` comparison, where the cast is
  parenthesized so `<` is not read as a generic argument), `s.st_nlink as isize`,
  `(*dp).d_type as c_int`, `strlen (...) as ptrdiff_t`, `*s1 as c_uchar as c_int` for
  `downcase` (C's `(unsigned char) *s1++`), `DIRECTORY_SEP as c_char` for the `'/'` byte
  compare in `p1[elt_len]`, and `matchcount += if matchcount <= 1 { 1 } else { 0 }` for C's
  `matchcount += matchcount <= 1` (Rust has no `bool`-to-`int` conversion).
- **C's `? :` expressions** become `if`/`else` expressions (the two `DECODE_SYSTEM` arms, the
  two `make_uninit_*` arms, the `S_ISDIR` `Qt`/`Qnil` pair, the `Fsort` predicate and the four
  `Fcompare_strings` case tables).
- **C's comma-expression assignments inside a condition** (the two `(cmp = Fcompare_strings
  (…), BASE_EQ (Qt, cmp))` conjuncts) become block expressions, which preserves C's
  short-circuit (the `while { … }` precedent).
- **`macro_rules! TRIVIAL_DIRECTORY_ENTRY` sits where C defines the macro** (inside
  `file_name_completion`, C 644-646), because C's `#define` only applies from that point; C's
  `!strcmp (n, "x")` is the port's `strcmp (n, …) == 0`.
- **`specpdl_ptr = specpdl_ref_to_ptr (count);`** writes the mirror's `extern` `static mut
  specpdl_ptr` — the "discard the unwind protect" statement (this file does not call
  `unbind_to` to undo it, exactly as C does).
- **`keyboard.h` and `termhooks.h` are not included**: no name this unit's live text uses comes
  from them, and `completion_ignore_case` arrives through the generated `globals.rs`
  (`variables`), which lisp.h includes (P-21).
- **`syms_of_dired` ends with the same variable definition** (`Vcompletion_ignored_extensions`)
  that C's `DEFVAR_LISP` names; it is defined in this unit with `#[unsafe(no_mangle)] pub
  static mut` and shadows `globals.rs`'s declaration (P-21), which `tools/check-globals.py`
  checks (the unit is not among the rows it reports).

### Macro-body notes

`docs/ports/macro-body-notes.tsv` carries six rows for this unit: `directory_files_internal`
(`uninit`), `file_name_completion` (`switch,case,default`), `syms_of_dired` (`doc`, the
positional `DEFVAR_LISP!` docstring), `Ffile_name_completion` (`0`) and
`Ffile_name_all_completions` (`1`) - C's `ALL_FLAG` argument is a literal `0`/`1` for a `bool`
parameter, and the port spells them `false`/`true` - and `read_dirent` (`true`, C's
`while (true)` becoming Rust's `loop`).  No new `token-aliases.tsv` family and no
`excluded-macros.txt` row was needed: the three macros C defines in this file
(`emacs_readdir`, `emacs_closedir`, `TRIVIAL_DIRECTORY_ENTRY`) all keep their names and bodies.

### Condition table

Every `#if` in this file is decided by `docs/baseline/config/darwin-arm64.macros.tsv`:
`MSDOS` undef, `WINDOWSNT` undef, `HAVE_ANDROID` undef, `ANDROID_STUBIFY` undef,
`HAVE_PWD_H` 1, `HAVE_STRUCT_DIRENT_D_TYPE` 1, `_D_EXACT_NAMLEN` undef, `O_PATH` undef,
`HAVE_CYGWIN_O_PATH_BUG` undef, `HAVE_GETPWENT`/`HAVE_ENDPWENT`/`HAVE_GETGRENT`/`HAVE_ENDGRENT`
1.  `src/dired.c` is already in `docs/baseline/config/cond-eval-verified.txt`; re-running
`python3 tools/check-cond-eval.py src/dired.c` reports `25 conditions, 0 unknown, 0
disagreements [VERIFIED]`, so the extractor's branch selection for it is the clang-verified
one and no `source-header-macros.tsv` row was needed (see the `_D_EXACT_NAMLEN` note above).

### Line count

`docs/ports/line-drift-tolerance.tsv`'s `dired` row declares the `live` basis with a 0.10
tolerance: C's 147 dead lines here are recorded rather than carried, so the denominator is the
1067 live lines, and the port's 1143 (+7.1%) is the unit's fixed costs plus the declarations
the mirrors do not carry, less the 107 doc-comment lines that the eight `DEFUN!` docstrings
and the `DEFVAR_LISP!` docstring collapse into literals.
