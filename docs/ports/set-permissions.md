# emacs/lib/set-permissions.c (836 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] chmod_or_fchmod  (line 749)
- [x] set_permissions  (line 765)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`femacs/lib/set-permissions.rs` is the live text of `emacs/lib/set-permissions.c`
(`tools/check-cond-eval.py`: 25 conditions, 0 unknown, 0 disagreements).  `USE_ACL` is 0, so
the whole `#if USE_ACL` chain (C 29-472: the FreeBSD/Cygwin `HAVE_ACL_FROM_TEXT` arm, the
Solaris/Cygwin `HAVE_FACL && GETACL` arm with its `ACE_GETACL` half, the HP-UX `HAVE_GETACL`
arm with its `HAVE_ACLV_H` half, the AIX `ACLX_GET && ACL_AIX_WIP` arm, the older-AIX
`HAVE_STATACL` arm and the NonStop `HAVE_ACLSORT` arm), `set_acls` (C 476-735) and the two
`#if USE_ACL` blocks inside `set_permissions` (C 771-790 and C 800-820) are dead; the live
text is C 747-755's `chmod_or_fchmod` and C 792-835's `set_permissions` body.

* `HAVE_FCHMOD` (C 749's `if`) is not in the generated `src/include/config.rs` — its curated
  list stops short of the ACL/permission constants — so this unit declares it with the frozen
  value (`docs/baseline/config/darwin-arm64.macros.tsv`'s `HAVE_FCHMOD 1`).  A later round
  could move it (and `USE_ACL`, `HAVE_FACL`) into the curated list; until then each ACL unit
  that reads one records it locally, as here.
* C 767's `_GL_UNUSED` is an attribute, so the token has no Rust counterpart: the
  `macro-body-notes.tsv` row for `set_permissions` records it (the `_GL_ATTRIBUTE_PURE` class).
* `bool early_chmod;` (C 768) is assigned once in the live arm, so the port writes
  `let early_chmod: bool;` and keeps C's assignment (PORTING.md 1.5's second case); `ret` is
  declared twice in C (a block-scoped one in C 794 and C 799's), which the port keeps as two
  `let`s in their own scopes.
* `int saved_errno = ret ? errno : 0;` is the port's `if ret != 0 { *errno () } else { 0 }`
  (the errno mirror is an accessor, not an object).
* `docs/ports/line-drift-tolerance.tsv` carries `set-permissions 0.90` (C 836 vs about 116).

Verification: `build-tu lib/set-permissions.c` → 0 errors, no unit warnings;
`extract-abi.py --check` → OK; `nm -g` on both objects → the same two definitions
(`_chmod_or_fchmod`, `_set_permissions`).
