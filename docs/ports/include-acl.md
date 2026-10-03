# emacs/lib/acl.h (118 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [ ] _GL_ACL_H  (line 21, body 356a192b7913)
- [ ] aclinfo_free  (line 91, body a29646b15239)
- [ ] aclinfo_has_xattr  (line 90, body 7cb6efb98ba5)
- [ ] aclinfo_scontext_free  (line 97, body a29646b15239)

## Types and their fields — 1
- [ ] aclinfo: char* buf, ssize_t size, char* scontext, int scontext_err, int err, char _gl_acl_ch, <aggregate> u

## File-scope objects — 0

## Decisions (WIP, PORTING.md P-26)

`emacs/lib/acl.h` @ `fd7afced403`; port `femacs/lib/include/acl.rs`.  The mirror carries the two
flag constants, `struct aclinfo` with its measured layout and the declarations; the unchecked
boxes are the `aclinfo_has_xattr`/`aclinfo_free`/`aclinfo_scontext_free` macros, which C spells
as `false`/`((void) 0)` on this target and the port keeps as a constant and two empty
macro_rules (the `#if HAVE_LINUX_XATTR_H && HAVE_LISTXATTR` arm is dead: the first is
undefined).
