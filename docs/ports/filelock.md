# emacs/src/filelock.c (840 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 5
- [x] lock-file  (Flock_file, 1/1, doc 2db5ac8f8e7d, line 677)
- [x] unlock-file  (Funlock_file, 1/1, doc 905cf24d58e8, line 700)
- [x] lock-buffer  (Flock_buffer, 0/1, doc ad3f4915370c, line 725)
- [x] unlock-buffer  (Funlock_buffer, 0/0, doc 5bd35eba75e2, line 744)
- [x] file-locked-p  (Ffile_locked_p, 1/1, doc 1368825b89e7, line 770)

## Functions defined — 16
- [x] get_boot_sec  (line 115, static)
- [x] rename_lock_file  (line 159, static)
- [x] create_lock_file  (line 202, static)
- [x] lock_file_1  (line 269, static)
- [x] within_one_second  (line 303, static)
- [x] read_lock_data  (line 319, static)
- [x] integer_prefixed  (line 352, static)
- [x] current_lock_owner  (line 386, static)
- [x] lock_if_free  (line 515, static)
- [x] make_lock_file_name  (line 537, static)
- [x] lock_file  (line 582, static)
- [x] unlock_file  (line 637, static)
- [x] unlock_file_handle_error  (line 654, static)
- [x] unlock_all_files  (line 663)
- [x] unlock_buffer  (line 763)
- [x] syms_of_filelock  (line 812)

## Macros defined — 1
- [x] VALID_PROCESS_ID  (line 362, body ed9c3292afbe)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/filelock.c` @ `fd7afced403`; port `src/filelock.rs` (911 lines vs C's 840,
811 live - the 12.3% drift is declared in `docs/ports/line-drift-tolerance.tsv`); the gate is
`python3 tools/extract-abi.py --check emacs/src/filelock.c src/filelock.rs` -> `OK`, and
`build-tu src/filelock.c` prints `built`.

### Dead text (not transcribed)

The frozen configuration leaves `WINDOWSNT`, `MSDOS` and `HAVE_ANDROID` undefined
(`docs/baseline/config/darwin-arm64.macros.tsv`), `ELOOP` is 62 and `HAVE_PWD_H` is 1.  The
dead arms are recorded here instead of transcribed:

- C 47-51: `#ifdef WINDOWSNT`'s three includes (`<share.h>`, `<sys/socket.h>`,
  `"w32common.h"`).
- C 161-162: `rename_lock_file`'s `return sys_rename_replace (old, new, force);` arm.  C 163's
  `#else` body is the live one and is transcribed.
- C 204-209: `create_lock_file`'s `int err = ENOSYS;` arm (with its four-line comment); C 210's
  `#else` initializer (`emacs_symlink (...) == 0 ? 0 : errno`) is the live one.
- C 55-57: `#ifdef HAVE_ANDROID`'s `#include "android.h"`.
- C 540-543 and C 546-556: `make_lock_file_name`'s `#if defined HAVE_ANDROID &&
  !defined ANDROID_STUBIFY` block (`char *name`, and the `android_is_special_directory` test
  over `"/assets"`/`"/content"`); C 548's `Fexpand_file_name` and C 558's `calln` are live.
- C 776-777: `Ffile_locked_p`'s `#ifdef MSDOS return Qnil;` arm; C 778's `#else` body is live.
- C 310-312: `#ifndef ELOOP / # define ELOOP (-1) / #endif` (ELOOP is defined).
- C 361-363: `#ifndef VALID_PROCESS_ID`'s guard around the macro (C 362) is live; the
  MS-Windows alternative the comment mentions is not text this file contains.

`#ifndef MSDOS` (C 53 ... C 660's `#endif`) guards the whole implementation, and its live
regions are transcribed (C 685-696 in `Flock_file`, C 704-721 in `Funlock_file`).  C 32-34's
`#ifdef HAVE_PWD_H` include is live but contributes no name this file uses, and no mirror
carries `<pwd.h>`; `<sys/file.h>` (C 36) is the same case (the doc.rs `<sys/file.h>`
precedent).  Both are left out; every name C takes from the remaining includes is either in a
mirror or declared at its use site (below).

### Structural deviations

- **`switch` -> if/else chain with a flag.**  C 429's `switch (owner->colon[0])` falls through
  from the `'\357'` (U+F022) arm into `':'`.  Rust has no fall-through, so the arm selection
  is an if/else chain and the shared `':'` body runs behind `fallthrough_colon` (the
  callint.rs/tparam.rs shared-body shape, PORTING.md 2.2).  Recorded in
  `docs/ports/macro-body-notes.tsv` for the four keywords with no counterpart.
- **`FOR_EACH_LIVE_BUFFER` has no Rust form yet (P-22).**  `unlock_all_files` writes out the
  expansion C's macro stands for - `for ((tail) = (Vbuffer_alist); (CONSP (tail) &&
  ((buf) = XCDR (XCAR (tail)), true)); (tail) = XCDR (tail))` - as
  `tail = Vbuffer_alist; while CONSP (tail) && { buf = XCDR (XCAR (tail)); true } { … }`,
  keeping C's `tail`/`buf` names and statement order (the ITREE_FOREACH precedent, P-33).
- **`TYPE_MAXIMUM` is a unit-local macro** (gnulib's `<intprops.h>`; no mirror carries it),
  the profiler.rs shape; `VALID_PROCESS_ID`'s `TYPE_MAXIMUM (pid_t)` is spelled
  `TYPE_MAXIMUM! (pid_t) as intmax_t`, because the C `(p) <= TYPE_MAXIMUM (pid_t)`
  comparison's `intmax_t` operand needs the conversion Rust will not insert.
- **`PRIdMAX` is a unit-local `const PRIdMAX: &str = "jd"`** (measured with clang on the
  frozen target: `printf ("%s", PRIdMAX)` -> `jd`), and the two format strings C builds by
  concatenation are written as the concatenated literals `".%jd:%jd"` / `".%jd"` - Rust
  cannot concatenate a const into a literal.  The missing `PRIdMAX` token is recorded in
  `docs/ports/macro-body-notes.tsv`.
- **`EDOM` is a unit-local `const EDOM: c_int = 33`** (the frozen target's `<errno.h>`;
  `docs/baseline/config/darwin-arm64.macros.tsv` 811).  `NEGATIVE_ERRNO`'s C value `EDOM < 0`
  needs the name for the constants rule, and the errno mirror does not carry it.
- **Names declared at the use site** (the terminal.rs/doc.rs shape): `emacs_fstatat`,
  `emacs_unlink`, `emacs_symlink`, `emacs_renameat_noreplace`, `emacs_rename`, `emacs_write`
  (lisp.h 5303-5334, which the lisp.rs mirror does not carry yet), `kill` (`<signal.h>`;
  signal.rs carries `raise` but not `kill`), `mkostemp` (`<stdlib.h>`), `snprintf`
  (`<stdio.h>`), `strtoimax` (`<inttypes.h>`), `memrchr` (gnulib's `<string.h>`; the ported
  `lib/memrchr.rs` emits the symbol) and `internal_condition_case_1` (lisp.h 4806).
- **Two parameters are re-bound, and two `fn` parameters need the raw identifier.**  C's
  `lock_file`/`make_lock_file_name` parameters are named `fn`, a Rust keyword, so they are
  `r#fn` (PORTING.md 0.2); `Flock_buffer`'s body rebinds its `file` parameter
  (`let mut file = file;`, the data.rs `XSETSYMBOL` precedent) and `make_lock_file_name`
  rebinds `r#fn`, exactly where C assigns to them.
- **`read_lock_data`'s `while ((nbytes = readlinkat (…)) < 0 && errno == EINVAL)`** keeps C's
  condition shape with Rust's `while { … }` block expression (the cm.rs/itree.rs precedent);
  the loop body is C's statement order.
- **`within_one_second`'s `ckd_sub (&diff, a, b)`** is `overflowing_sub`'s (value, flag) pair
  (P-15, the `ckd_sub` family in `docs/ports/token-aliases.tsv`), with the store kept C's way.
- **Uninitialized C locals get a zero initializer (PORTING.md 1.5)**: `timespec boot_time`
  (C's own `boot_time.tv_sec = 0` follows it), `lock_info_type local_owner`/`lock_info`/
  `locker`, the `struct stat st` of `rename_lock_file`, the `char lock_info_str[MAX_LFINFO+1]`
  buffer C leaves for `snprintf`, and `boot_time`/`lfinfo_end` in `current_lock_owner` (the two
  C writes on some paths only).
- **Two `static` helpers are `unsafe extern "C" fn`** because C hands their addresses to
  `internal_condition_case_1`: `unlock_file` and `unlock_file_handle_error` (PORTING.md 1.4,
  the profiler.c signal-handler shape).  `get_boot_sec`, `within_one_second`'s caller and the
  rest keep the `fn`/`unsafe fn` split.
- **`lock_info_type.user`'s size** is `MAX_LFINFO + 1 + sizeof " (pid )" - sizeof "."` in C
  (8199 bytes on this target); the port writes `MAX_LFINFO as usize + 1 + 8 - 2`, the two
  `sizeof`s of the string literals being 8 and 2.
- **The two `static char const …[]` locals** (`nonce_base` in `create_lock_file`,
  `replacement` in `lock_file`) keep their C text as `static …: [u8; N] = *b"…"` (a C string
  literal's bytes are `u8`), cast at the two uses; `sizeof` is unchanged.
- **The two `DEFVAR` variables** (`Vtemporary_file_directory`, `create_lockfiles`) are defined
  in the unit with `#[unsafe(no_mangle)] pub static mut` and shadow `globals.rs`'s
  declarations (P-21, the undo.rs shape); `create_lockfiles` starts `false` and
  `syms_of_filelock` sets it `true`, as C does.
- **C's function-scope `static`s** (`nonce_base`, `replacement`) are Rust function-scope
  `static`s, which Rust allows; they emit no symbol and are not C's file-scope objects (the
  ledger's last section is empty, as the extractor reports).

### Macro-body notes

`docs/ports/macro-body-notes.tsv` carries five rows for this unit: `lock_file_1` (`pridmax`),
`current_lock_owner` (`switch,case,default,fallthrough`), `unlock_all_files`
(`for_each_live_buffer`), `syms_of_filelock` (`doc`, the positional docstring of
`DEFVAR_LISP!`/`DEFVAR_BOOL!`) and `Ffile_locked_p` (`switch,case,break,default`).  No new
`token-aliases.tsv` family was needed.

### Condition table

Every `#if` in this file is decided by `docs/baseline/config/darwin-arm64.macros.tsv`:
`WINDOWSNT` undef, `MSDOS` undef, `HAVE_ANDROID` undef, `ANDROID_STUBIFY` undef, `ELOOP` 62,
`HAVE_PWD_H` 1.  `src/filelock.c` is already in `docs/baseline/config/cond-eval-verified.txt`,
so the extractor's branch selection for it is the clang-verified one; no condition-table gap
was found and no `source-header-macros.tsv` row was needed.

- **Integrator note (3.263): `snprintf` is declared by the mirror, not here.**  `lib/include/stdio.rs`
  gained the `<stdio.h>` entry so the `GLYPH_DEBUG` arms (which call it) compile - `xtask dead-arms`
  with the `glyph_debug` flip is what found it - and this unit's own declaration of the same name was
  what made the name ambiguous once the mirror carried it.  The declaration is gone; the mirror is
  the single declaration point (PORTING.md 0.1-3).
