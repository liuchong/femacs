# emacs/lib/at-func.c (142 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] AT_FUNC_NAME  (line 70)

## Macros defined — 4
- [x] CALL_FUNC  (line 44, body 945fd3c664b7)
- [x] FUNC_FAIL  (line 57, body 7984b0a0e139)
- [x] FUNC_RESULT  (line 51, body 46f8ab7c0cff)
- [x] VALIDATE_FLAG  (line 45, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`emacs/lib/at-func.c` @ `fd7afced403`; port `femacs/lib/at-func.rs` (144 lines); the gates are
`build-tu lib/at-func.c` -> `built` (`build/femacs/at-func.o`, no C-visible exports - the
template instantiates nothing; in C the includers define their own entries, so the reference has
no object of its own here) and
`python3 tools/extract-abi.py --check emacs/lib/at-func.c femacs/lib/at-func.rs` -> `OK`.  No
`build/emacs-ref/lib/at-func.o` exists, so `tools/check-symbols.py` does not compare this unit.

- **The 10.13② shape is a prelude that instantiates nothing.**  gnulib compiles this source
  only through inclusion: the five consumers `faccessat.c`, `fchmodat.c`, `fstatat.c`,
  `readlinkat.c` and `utimensat.c` each `# define` the `AT_FUNC_*` bindings and `#include
  "at-func.c"`.  A template with no instantiation of its own gets the `ftoastr.c` shape: the
  body is `macro_rules! at_func_body`, parameterised on the consumer's bindings, and this file
  invokes it nowhere.  C's own text has 1 function (`AT_FUNC_NAME`, C 70) and 0 objects of its
  own, which the port's 0 exports match.

- **The five consumers keep their own copies.**  They do not `include!("at-func.rs")`; each
  `femacs/lib/{faccessat,fchmodat,fstatat,readlinkat,utimensat}.rs` records the `AT_FUNC_*`
  parameters itself.  Four of them (`faccessat.rs`, `fchmodat.rs`, `fstatat.rs`,
  `readlinkat.rs`) keep only the parameter `macro_rules!` under
  `#[cfg (feature = "no_…")]`, because their `#include "at-func.c"` arm is dead under the
  frozen configuration, and name their expansion's tokens instead of writing it out.
  `utimensat.rs` is the one whose arm is live and it writes the expansion out itself (its
  `local_utimensat`, the openat_proc_name attempt then save_cwd/fchdir/restore_cwd) - see
  `docs/ports/{faccessat,fchmodat,fstatat,readlinkat,utimensat}.md`.

- **The standalone reading and where its names live.**  The frozen configuration resolves all
  6 of C's conditions (`tools/check-cond-eval.py lib/at-func.c` -> 6 conditions, 0 unknown, 0
  disagreements) with `GNULIB_SUPPORT_ONLY_AT_FDCWD`, `AT_FUNC_USE_F1_COND`,
  `AT_FUNC_RESULT` and `AT_FUNC_FAIL` all undefined, so `mod standalone` records the names
  that reading leaves: `CALL_FUNC` (C 44), `VALIDATE_FLAG` (C 45), `FUNC_RESULT` (C 51),
  `FUNC_FAIL` (C 57) and C 70's entry name `AT_FUNC_NAME`.  The module is the `ftoastr.rs`
  `standalone` decision - these are only the ledger's names; the body takes them as
  parameters.  `FUNC_FAIL`'s `#define FUNC_FAIL -1` is the ledger's *constant*, so it is
  `pub const FUNC_FAIL: c_int = -1;` (which also answers the macro check).

- **The dead arms are recorded, not carried.**  C 21-25's
  `GNULIB_SUPPORT_ONLY_AT_FDCWD` `<errno.h>` include and `ENOTSUP` fallback, C 32-42's
  conditional `CALL_FUNC`/`VALIDATE_FLAG`, C 48-52's `AT_FUNC_RESULT` and C 54-58's
  `AT_FUNC_FAIL` are dead; C 77-79's `errno = ENOTSUP; return FUNC_FAIL;` return is dead too.
  Each is a note or a `mod standalone` record here and is not transcribed, so the port carries
  C's live text only.

- **The parameterisation.**  `at_func_body!` takes `$name` (C 70's `AT_FUNC_NAME`),
  `$export` (the symbol), `$f1` (C 35's `AT_FUNC_F1`), `$post_params`/`$post_args`
  (C 70's `AT_FUNC_POST_FILE_PARAM_DECLS`/`AT_FUNC_POST_FILE_ARGS`) and `$func_result`/
  `$func_fail` (C 48-58's `FUNC_RESULT`/`FUNC_FAIL`, which a consumer may redefine as
  `AT_FUNC_RESULT`/`AT_FUNC_FAIL`).  The two `tt` parameters are spliced at the signature and
  the two call sites; an instance passes the Rust spelling of C's declaration/argument lists
  (this file instantiates none, so its own copy keeps C's token lists in the records above).

- **No `include!`s beyond `config.rs`/`errno.rs`.**  The body names `IS_ABSOLUTE_FILE_NAME`
  (filename.h), `openat_proc_name`/`EXPECTED_ERRNO`/`OPENAT_BUFFER_SIZE` (openat-priv.h),
  `save_cwd`/`free_cwd`/`restore_cwd` (save-cwd.h), `fchdir` (unistd.h), `free` (stdlib.h) and
  `AT_FDCWD` (fcntl.h), but it is never expanded here, so the port does not include those
  consumers' mirrors - they are included where the expansion is written (the `utimensat.rs`
  copy).

- **Line drift** is C 142 vs the port's 144 (1.4%), inside the default 5%; no
  `docs/ports/line-drift-tolerance.tsv` row is needed.
