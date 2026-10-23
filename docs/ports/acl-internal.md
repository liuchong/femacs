# emacs/lib/acl-internal.c (523 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] free_permission_context  (line 495)

## Macros defined — 1
- [x] ACL_INTERNAL_INLINE  (line 20, body a9f37094e6e8)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`lib/acl-internal.rs` is the live text of `emacs/lib/acl-internal.c`
(`tools/check-cond-eval.py`: 22 conditions, 0 unknown, 0 disagreements).  What the frozen
configuration keeps is C 20's qualifier and C 494-523's `free_permission_context`, whose
*own* body is empty here (its `#if USE_ACL` chain, C 497-522, is dead); everything else —
C 34-492's `#if USE_ACL && …` chain (the macOS `ACL_TYPE_EXTENDED` arm, the
Linux/FreeBSD/NetBSD/Cygwin `HAVE_ACL_FIRST_ENTRY` arm with its `__CYGWIN__` half, the
`HAVE_FACL && GETACL` arm with its `ACE_GETACL` half, the HP-UX `HAVE_GETACL` arm with its
`HAVE_ACLV_H` half, the AIX arm and the NonStop `HAVE_ACLSORT` arm) and C 28-32's
`#if defined __CYGWIN__` includes — is dead.

* C 20 is PORTING.md 10.12's wrapper shape: `pub const ACL_INTERNAL_INLINE: () =
  _GL_EXTERN_INLINE;` in this unit shadows the mirror's `ACL_INTERNAL_INLINE` (which is in
  `acl_internal_decls`, a whole-import module, so the definition wins — §0.1-10).  That
  definition is also why clang sees the header's `#ifndef` as false.
* `free_permission_context`'s dead body is kept as `#[cfg (feature = "use_acl")]` text
  (P-51): on this target the body is empty, and an empty Rust body would be read as a stub
  (`stub or empty body in fn free_permission_context`) — the cfg'd text is what the gate sees
  instead, exactly as C's own `#if USE_ACL` block stands there.  The same features as the
  header's mirror name the arms (`use_acl_have_acl_get_file`, `use_acl_getacl`).
* `docs/ports/line-drift-tolerance.tsv` carries `acl-internal 0.95` (C 523 vs about 50).

Verification: `build-tu lib/acl-internal.c` → 0 errors, no unit warnings;
`extract-abi.py --check` → OK; `nm -g build/femacs/acl-internal.o` → `T _free_permission_context`,
the same single definition the reference object has.
