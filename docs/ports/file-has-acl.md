# emacs/lib/file-has-acl.c (986 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] file_has_aclinfo  (line 439)
- [x] fdfile_has_aclinfo  (line 459)
- [x] file_has_acl  (line 977)

## Macros defined — 1
- [x] USE_LINUX_XATTR  (line 45, body 7cb6efb98ba5)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`femacs/lib/file-has-acl.rs` is the live text of `emacs/lib/file-has-acl.c`
(`tools/check-cond-eval.py`: 32 conditions, 0 unknown, 0 disagreements).

* `HAVE_LINUX_XATTR_H` and `HAVE_LISTXATTR` are undefined, so `USE_LINUX_XATTR` is C 45's
  `false` and C 48-509's block (the `<selinux/selinux.h>`, `<stdckdint.h>`, `<arpa/inet.h>`,
  `<sys/xattr.h>` and `<linux/xattr.h>` includes, the five `XATTR_NAME_*` defaults and
  `fdfile_has_aclinfo`'s xattr path with `get_aclinfo`,
  `aclinfo_may_indicate_xattr`/`aclinfo_get_xattr` and `acl_nfs4_nontrivial`) is dead text.
* `USE_ACL` is 0, so C 384-424's helper block and the `#if USE_ACL` chain inside
  `fdfile_has_aclinfo`'s `#else` arm (C 519-964: the `HAVE_ACL_GET_FILE` arms for
  Linux/macOS/FreeBSD, the Solaris/Cygwin `GETACL` arm with its `ACE_GETACL` half, the HP-UX
  `HAVE_GETACL` arm with `HAVE_ACLV_H`, the AIX `ACLX_GET && ACL_AIX_WIP` arm and the NonStop
  `HAVE_ACLSORT` arm) are dead too.  What is live: C 37-40's assertion and the
  `UNKNOWN_SECURITY_CONTEXT` string (kept as a unit-local `static`, since the checker's
  file-scope-object list does not carry it but C defines it), C 42-46's `USE_LINUX_XATTR`,
  `file_has_aclinfo` (C 438-443), C 513-517's five `ai->…` initializations plus C 967's
  `return 0;`, and `file_has_acl` (C 976-986).
* `MAYBE_UNUSED` (C 459-463) is `attribute.h`'s attribute; the port keeps the signature and
  the `d_type` declaration and records the token in `macro-body-notes.tsv` for
  `fdfile_has_aclinfo`.
* `dirent.rs` gained two appends for this unit: `_GL_DT_NOTDIR` (dirent.in.h 102) and
  `IFTODT`, whose frozen target takes its own system header's definition (so dirent.in.h
  953-959's `S_ISREG (mode) ? DT_REG : …` fallback is dead); the macro returns `c_int`, which
  is what C's `DT_*` enum is.
* `docs/ports/line-drift-tolerance.tsv` carries `file-has-acl 0.90` (C 986 vs about 114).

Verification: `build-tu lib/file-has-acl.c` → 0 errors; `extract-abi.py --check` → OK;
`nm -g` on both objects → the same three definitions (`_fdfile_has_aclinfo`, `_file_has_acl`,
`_file_has_aclinfo`).  No behavioural probe: on this target all three return before any ACL
call (`fdfile_has_aclinfo` writes the `ENOTSUP` info and returns 0).
