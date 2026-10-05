# emacs/lib/acl.h (118 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] _GL_ACL_H  (line 21, body 356a192b7913)
- [x] aclinfo_free  (line 91, body a29646b15239)
- [x] aclinfo_has_xattr  (line 90, body 7cb6efb98ba5)
- [x] aclinfo_scontext_free  (line 97, body a29646b15239)

## Types and their fields — 1
- [x] aclinfo: char* buf, ssize_t size, char* scontext, int scontext_err, int err, char _gl_acl_ch, <aggregate> u

## File-scope objects — 0

## Decisions (WIP, PORTING.md P-26)

`emacs/lib/acl.h` @ `fd7afced403`; port `femacs/lib/include/acl.rs`.  The mirror carries the two
flag constants, `struct aclinfo` with its measured layout and the declarations; the unchecked
boxes are the `aclinfo_has_xattr`/`aclinfo_free`/`aclinfo_scontext_free` macros, which C spells
as `false`/`((void) 0)` on this target and the port keeps as a constant and two empty
macro_rules (the `#if HAVE_LINUX_XATTR_H && HAVE_LISTXATTR` arm is dead: the first is
undefined).

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/acl.h femacs/lib/include/acl.rs
OK emacs/lib/acl.h -> femacs/lib/include/acl.rs            (0 problem rows)
```

The five boxes above are names the mirror carries: the split guard (`_GL_ACL_H`), the two
`aclinfo` macros C's live `#else` arm defines (`aclinfo_has_xattr` -> false, `aclinfo_free` ->
nothing) plus `aclinfo_scontext_free` and the `aclinfo` record itself.  No unit compiles this
header on the frozen target (`lib/acl.c` and the ACL-aware callers are not in
`docs/baseline/frozen-build-sources.txt`), so there is no `build-tu` consumer; the check above is
the evidence.

## 2026-10-05: the `#else` macros must not answer where the functions live (3.511)

C 88-104 declares `aclinfo_has_xattr`/`aclinfo_free` as *functions* under
`HAVE_LINUX_XATTR_H && HAVE_LISTXATTR` and defines the same names as macros in the `#else`; C 106-112
does the same for `aclinfo_scontext_free` under the narrower
`(… && (HAVE_SMACK || USE_SELINUX_SELINUX_H))`.  Only one arm is compiled in C, but a Rust
`macro_rules!` is an item: the three macros were ungated, so in an arm where
`lib/file-has-acl.rs` *defines and calls* the functions (C 142-154, 278-303) the macro answered
first and silently rewrote two live call sites (C 152 and 984).  They now carry the same
condition the unit's function definitions do (`use_linux_xattr`, and for the third one also
`any (have_smack, use_selinux_selinux_h)`), which is C's own `#if`.
