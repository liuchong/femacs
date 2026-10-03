# emacs/lib/get-permissions.c (291 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] get_permissions  (line 33)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`femacs/lib/get-permissions.rs` is the live text of `emacs/lib/get-permissions.c`
(`tools/check-cond-eval.py`: 10 conditions, 0 unknown, 0 disagreements).  `USE_ACL` is 0, so
C 39-287's whole `#if USE_ACL && …` chain — the Linux/FreeBSD/macOS `HAVE_ACL_GET_FILE` arms
(with the macOS `ACL_TYPE_EXTENDED` half and FreeBSD's `ACL_TYPE_NFS4`), the Solaris/Cygwin
`GETACL` arm with its `ACE_GETACL` half, the HP-UX `HAVE_GETACL` arm with its `HAVE_ACLV_H`
half, the AIX `ACLX_GET && ACL_AIX_WIP` TODO, the older-AIX `HAVE_STATACL` arm and the
NonStop `HAVE_ACLSORT` arm — is dead, and only C 33-37's doc comment, the signature, C 38-39's
`memset`/`ctx->mode` pair and C 289's `return 0;` are written.  Because the evaluator decides
this file's conditions, P-51's dead-arm *text* is optional here (it is required where a
condition cannot be decided); the port keeps one note listing the arms' entry points instead,
and `docs/ports/line-drift-tolerance.tsv` carries `get-permissions 0.90`.

Deviations: C's `sizeof *ctx` is `std::mem::size_of::<permission_context> ()` (the `sizeof`
token family); the argument names `name`/`desc` are unused in the live arm, which the unit's
allow list covers; `get_permissions` is non-static in C and is emitted with
`#[unsafe(no_mangle)]`, and the mirror's declaration for it is shadowed by this definition
(the `acl_internal_decls` whole-import module, PORTING.md 0.1-10).

Verification: `build-tu lib/get-permissions.c` → 0 errors, no unit warnings;
`extract-abi.py --check` → OK; `nm -g build/femacs/get-permissions.o` → `T _get_permissions`.
No behavioural probe (there is no live ACL code to execute on this target).
