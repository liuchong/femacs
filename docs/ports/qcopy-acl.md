# emacs/lib/qcopy-acl.c (125 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] qcopy_acl  (line 74)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0


# ----------------------------------------------------------------------
## Decisions

### Summary

`lib/qcopy-acl.rs` is the live text of `emacs/lib/qcopy-acl.c` for the frozen
configuration (`tools/check-cond-eval.py lib/qcopy-acl.c`: 6 conditions, 0 unknown,
0 disagreements), so `USE_XATTR` is undefined and only the `#else` arm of the copy is
written.  `--check` is OK and the object reproduces the reference's symbols:

    $ nm -g build/emacs-ref/lib/qcopy-acl.o     $ nm -g build/femacs/qcopy-acl.o
    T _qcopy_acl                                T _qcopy_acl
    U _get_permissions  U _set_permissions      the same three, plus Rust's panic
    U _free_permission_context                  entry points

### The dead arms (P-51)

`USE_XATTR` is undefined, so two arms are dead and are noted where C's `#if` stood:

* C 26-59's `#if USE_XATTR` block: `#include <attr/libattr.h>`, `#include <dirent.h>`,
  `<linux/xattr.h>`, the three `XATTR_NAME_*` defaults and the `is_attr_permissions`
  predicate.  This target has no `<attr/libattr.h>`, so the block cannot be written as
  compilable text even behind a feature (the `gettimeofday.c` Windows-arm class); the port
  keeps C's comment blocks and the ledger records it.
* C 79-113's `#ifdef USE_XATTR` arm inside `qcopy_acl`: the `chmod_or_fchmod` +
  `attr_copy_file`/`attr_copy_fd` path with its `EOPNOTSUPP` retry through
  `fdfile_has_aclinfo`, `aclinfo_free`, `S_ISDIR` and `_GL_DT_NOTDIR`.  It is recorded as a
  comment at C's position; `attr_copy_*` and `struct error_context` exist nowhere in the port.

### Deviations

1. `int ret;` (C 77) is a declaration whose only live assignment is C 120's
   `ret = set_permissions (…)`, so the port writes `let ret: c_int;` at C's position and
   assigns it below (PORTING.md 1.5's "every branch assigns" case).
2. `struct permission_context ctx;` (C 116) has no initializer and `get_permissions` writes
   it, so the port zero-initializes it (`std::mem::zeroed ()`, §1.5's first case).
3. The three entry points come from `acl-internal.h`'s whole-import module (the mirror's
   `use acl_internal_decls::*`), so this unit's calls and the mirror's declarations do not
   collide (PORTING.md 0.1-10).  `qcopy_acl` itself is non-static in C and is emitted with
   `#[unsafe(no_mangle)]`.
4. `docs/ports/line-drift-tolerance.tsv` carries `qcopy-acl 0.55`: C 125 vs the port's 64,
   the difference being the two dead `USE_XATTR` arms (about 69 lines, more than half the
   file) that this target never compiles.

### Verification

* `python3 tools/check-cond-eval.py lib/qcopy-acl.c` → `6 conditions, 0 unknown,
  0 disagreements [VERIFIED]`.
* `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target cargo run -q -p xtask -- build-tu lib/qcopy-acl.c`
  → `built lib/qcopy-acl.rs -> ../build/femacs/qcopy-acl.o`, 0 errors, no warnings from the
  unit.
* `python3 tools/extract-abi.py --check emacs/lib/qcopy-acl.c lib/qcopy-acl.rs` → OK.
* Symbol comparison against the frozen object (above).
* No behavioural differential probe was run (the round's workflow stops at
  `build-tu`/`--check`; executing `qcopy_acl` needs an ACL-bearing file system and the two
  `*-permissions` units, which are the next steps in this cluster).
