# emacs/lib/openat.h (137 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _GL_HEADER_OPENAT  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header is in the verified list, so the mirror carries the frozen text and the ledger
  carries the dead arms.**  `python3 tools/check-cond-eval.py lib/openat.h` ->
  `lib/openat.h: 11 conditions, 0 unknown, 0 disagreements [VERIFIED]` (line 118 of
  `docs/baseline/config/cond-eval-verified.txt`), so the evaluator's verdicts are the ones
  clang takes and `tools/extract-abi.py` needs no both-arm text (P-66):

  | C lines | arm | why it is dead |
  | --- | --- | --- |
  | 49-50 | `# define openat_permissive(Fd, File, Flags, Mode, Cwd_errno) openat (Fd, File, Flags, Mode)` | `HAVE_OPENAT` is undefined, so C 41's `#if !HAVE_OPENAT` took the declaration arm |
  | 51 | `# define openat_needs_fchdir() false` | the same |
  | 62-80 | `#if GNULIB_CHOWNAT`: `CHOWNAT_INLINE int chownat (…) { return fchownat (…, 0); }` and `lchownat` (`AT_SYMLINK_NOFOLLOW`) | `GNULIB_CHOWNAT` is not in the configuration, so the selector is 0 |
  | 82-100 | `#if GNULIB_CHMODAT`: `chmodat` / `lchmodat` | `GNULIB_CHMODAT` is not in the configuration |
  | 102-122 | `#if GNULIB_STATAT`: `statat` / `lstatat`, both `_GL_ATTRIBUTE_DEPRECATED` | `GNULIB_STATAT` is not in the configuration |
  | 22-26, 28-32, 36-38, 131-133 | the `_GL_CONFIG_H_INCLUDED` requirement, `<fcntl.h>`/`<sys/types.h>`/`<sys/stat.h>`/`<unistd.h>` and the C++ `extern "C"` guards | the includer names its own headers (PORTING.md 0.1-4); `__cplusplus` is not defined for a C translation unit |

- **`HAVE_OPENAT` is undefined, by measurement.**  `grep -n HAVE_OPENAT
  build/emacs-ref/src/config.h` prints nothing, and the macro is absent from
  `docs/baseline/config/darwin-arm64.macros.tsv` (`grep -n HAVE_OPENAT` prints nothing; the
  only `openat` entry is 4243 `_GL_OPENAT_ESYMLINK ELOOP`).  The evaluator's "0 unknown"
  confirms clang agrees with reading an absent selector as 0.

- **The four declarations are the header's substance, and `--check` does not ask for them.**
  Only *definitions* are compared, and the header defines nothing, so `--check`'s one item is
  the guard.  They are declared anyway because C declares them and because a consumer has to
  name them: `lib/utimensat.rs` 66-70 and `lib/openat-die.rs` 10-13 already carry hand copies
  (C's `openat.h` is that file's own `#include`, C 4), and the mirror is their home.  They sit
  in `openat_decls` with a glob import, so those units' own declarations shadow the glob rather
  than clashing with it when their owners wire them up (PORTING.md 0.1-10,
  `include/careadlinkat.rs`).

- **One spelling deviation, recorded: `mode_t` is written as `u16`.**  C's fifth
  `openat_permissive` parameter is `mode_t`, and the port has that name in
  `include/sys_types.in.rs` (`pub type mode_t = u16;`, measured from `__uint16_t`).  This
  mirror cannot include that file: `sys_types.in.rs` names `c_int`/`c_uint` in the *including
  unit's* scope, a mirror may not carry a `use` (E0252 against the includer's), and
  `build-tu lib/openat.h` compiles this mirror as the crate root.  The width is therefore
  written directly, ABI-identically to the port's alias.  Nothing in the port calls
  `openat_permissive`/`openat_needs_fchdir` (gnulib's `openat.c` is not in the frozen build: no
  `build/emacs-ref/lib/openat.o`), so this is documentation of the interface, not a call path.

- **The two hooks are `-> !`.**  `_Noreturn void openat_restore_fail (int)` / `openat_save_fail
  (int)` (C 55-56) is Rust's diverging return; the definitions are `lib/openat-die.rs`'s
  (`#[unsafe(export_name = "openat_save_fail")] pub unsafe extern "C" fn openat_save_fail
  (errnum: c_int)`), which is why the declarations are declarations.

- **`_GL_ATTRIBUTE_DEPRECATED` on C 108/115 has no counterpart** (PORTING.md 8: attributes are
  syntax and are dropped structurally), and both `statat`/`lstatat` are in the dead STATAT arm
  anyway.

## Evidence

```
$ python3 tools/extract-abi.py emacs/lib/openat.h
# emacs/lib/openat.h (137 lines)
## Subrs (DEFUN) - 0
## Functions defined - 0
## Macros defined - 1   (1/1 ticked above)
## Types and their fields - 0
## File-scope objects - 0

$ python3 tools/extract-abi.py --check emacs/lib/openat.h lib/include/openat.rs
OK emacs/lib/openat.h -> lib/include/openat.rs
# no `- ` problem line (`| grep -c '^- '` -> 0)

$ python3 tools/check-cond-eval.py lib/openat.h
lib/openat.h: 11 conditions, 0 unknown, 0 disagreements [VERIFIED]

$ grep -n HAVE_OPENAT build/emacs-ref/src/config.h        # no output, exit 1
$ grep -n HAVE_OPENAT docs/baseline/config/darwin-arm64.macros.tsv   # no output, exit 1
$ grep -in openat docs/baseline/config/darwin-arm64.macros.tsv
4243:_GL_OPENAT_ESYMLINK	ELOOP

$ grep -n 'mode_t = ' lib/include/sys_types.in.rs
70:pub type mode_t = u16;

# the units that already carry these prototypes by hand (the coordination point)
$ grep -rn 'openat_save_fail\|openat_restore_fail' lib/*.rs
lib/at-func.rs:96:        { openat_save_fail (*errno ()); }
lib/at-func.rs:119:        { openat_restore_fail (*errno ()); }
lib/openat-die.rs:15:/* C 6: `void openat_save_fail (int errnum) { abort (); }`.  `openat.h` is the system
lib/openat-die.rs:17:   exports `T _openat_save_fail`, tools/check-symbols.py).  `errnum` is unused in C too,
lib/openat-die.rs:19:#[unsafe(export_name = "openat_save_fail")]
lib/openat-die.rs:20:pub unsafe extern "C" fn openat_save_fail (errnum: c_int)
lib/openat-die.rs:25:/* C 7: `void openat_restore_fail (int errnum) { abort (); }`.  */
lib/openat-die.rs:26:#[unsafe(export_name = "openat_restore_fail")]
lib/openat-die.rs:27:pub unsafe extern "C" fn openat_restore_fail (errnum: c_int)
lib/utimensat.rs:68:  /* openat.h 55-56: _Noreturn void openat_save_fail (int) / openat_restore_fail (int);  */
lib/utimensat.rs:69:  pub fn openat_save_fail (err: c_int) -> !;
lib/utimensat.rs:70:  pub fn openat_restore_fail (err: c_int) -> !;
lib/utimensat.rs:148:    { openat_save_fail (*errno ()); }
lib/utimensat.rs:172:    { openat_restore_fail (*errno ()); }

$ ls build/emacs-ref/lib/ | grep openat
openat-die.o
openat-proc.o

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188 cargo run -q -p xtask -- build-tu lib/openat.h
warning: 2 warnings emitted
built lib/include/openat.rs -> ../build/femacs/include_openat.o
```
