# emacs/lib/acl_entries.c (68 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] acl_entries  (line 34)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0


## Decisions

- USE_ACL is 0 in the frozen configuration and HAVE_ACL_FIRST_ENTRY / HAVE_ACL_TYPE_EXTENDED are
  undefined, so C 41-64's two `acl_get_entry` loops are dead by measurement and the live body is
  the `int count = 0; if (acl != NULL) { } return count;` skeleton.  The tolerance row records
  that the port is 25% shorter than C for that reason.
- C 22's "acl-internal.h" is **not** mirrored as a file yet: this unit names only `acl_t`, which
  that header gets from <sys/acl.h>, so the new `sys_acl` mirror (measured: the four handle types
  are `void *`, the four scalar types are 32-bit ints, and the constant families) is included
  directly.  docs/status.md 3.23 records the deferral; the first heavier consumer
  (qcopy-acl.c/get-permissions.c, which need `struct permission_context`) brings that mirror.
- The new `acl` mirror carries lib/acl.h: the two flag constants, `struct aclinfo` (measured
  layout: 184 bytes, union at 28) and the declarations; its two xattr macros are C's `#else`
  arms here (HAVE_LINUX_XATTR_H is undefined), so they are no-ops spelled as empty macro_rules.
- `ssize_t` is spelled `c_long` in these mirrors (the port's `ssize_t` alias lives in the unistd
  mirror, which a system-header mirror does not include; C's `ssize_t` is `long` here).
- No call-relationship differences (D7): the live body has no call at all, as in C.
