# emacs/lib/acl-internal.h (284 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 40
- [x] ACL_INTERNAL_INLINE  (line 64, body a960df3ab827)
- [x] GETACLCNT  (line 34, body edd9d226f036)
- [x] HAVE_ACL_EXTENDED_FILE  (line 110, body 7cb6efb98ba5)
- [x] HAVE_ACL_GET_FD  (line 81, body 7cb6efb98ba5)
- [x] HAVE_ACL_SET_FD  (line 90, body 7cb6efb98ba5)
- [x] HAVE_FCHMOD  (line 58, body 7cb6efb98ba5)
- [x] MIN_ACL_ENTRIES  (line 74, body 1b6453892473)
- [x] MODE_INSIDE_ACL  (line 235, body 356a192b7913)
- [x] NEW_ACE_ACCESS_ALLOWED_ACE_TYPE  (line 173, body b6589fc6ab0d)
- [x] NEW_ACE_ACCESS_DENIED_ACE_TYPE  (line 174, body 356a192b7913)
- [x] NEW_ACE_APPEND_DATA  (line 186, body 6b4f6bd0a150)
- [x] NEW_ACE_DELETE  (line 193, body 67e6c786c9b7)
- [x] NEW_ACE_DELETE_CHILD  (line 190, body 7ee145d98dd2)
- [x] NEW_ACE_EVERYONE  (line 182, body da533c139833)
- [x] NEW_ACE_EXECUTE  (line 189, body 10b33a7d9196)
- [x] NEW_ACE_GROUP  (line 180, body 3daa00115a66)
- [x] NEW_ACE_IDENTIFIER_GROUP  (line 181, body 7232895c5fbc)
- [x] NEW_ACE_OWNER  (line 179, body 987a3bdd4907)
- [x] NEW_ACE_READ_ACL  (line 194, body 07780177d9da)
- [x] NEW_ACE_READ_ATTRIBUTES  (line 191, body 32c72f830f90)
- [x] NEW_ACE_READ_DATA  (line 184, body 31d75aa49ab1)
- [x] NEW_ACE_READ_NAMED_ATTRS  (line 187, body d8113820d1e0)
- [x] NEW_ACE_SYNCHRONIZE  (line 197, body 2d91fb920a7c)
- [x] NEW_ACE_WRITE_ACL  (line 195, body cfff7b7252fd)
- [x] NEW_ACE_WRITE_ATTRIBUTES  (line 192, body bd60cfec3dba)
- [x] NEW_ACE_WRITE_DATA  (line 185, body e37ab362db30)
- [x] NEW_ACE_WRITE_NAMED_ATTRS  (line 188, body 58b829734a11)
- [x] NEW_ACE_WRITE_OWNER  (line 196, body ce064a43ecac)
- [x] OLD_ACE_GROUP  (line 177, body d228a45608c3)
- [x] OLD_ACE_OTHER  (line 178, body e0b28e99ef1e)
- [x] OLD_ACE_OWNER  (line 176, body ecdf501256ce)
- [x] OLD_ALLOW  (line 171, body b6589fc6ab0d)
- [x] OLD_DENY  (line 172, body 356a192b7913)
- [x] acl_entries  (line 127, body 76d9afa5bfaf)
- [x] acl_extended_file  (line 111, body eb883130e83d)
- [x] acl_free_text  (line 97, body ea5ba9412ff2)
- [x] acl_from_mode  (line 115, body 48a28489fbe0)
- [x] acl_get_fd  (line 83, body 48a28489fbe0)
- [x] acl_set_fd  (line 92, body eb883130e83d)
- [x] fchmod  (line 59, body eb883130e83d)

## Types and their fields — 1
- [x] permission_context: mode_t mode, acl_t acl, acl_t default_acl, bool acls_not_supported, int count, int ace_count, struct acl_entry entries, int count, struct acl aclv_entries, int aclv_count, bool have_u, struct acl entries, int count

## File-scope objects — 0


# ----------------------------------------------------------------------
## Decisions

### Summary

`femacs/lib/include/acl-internal.rs` mirrors `emacs/lib/acl-internal.h`.  The mirror is named
with C's hyphen (`acl-internal.rs`, the `save-cwd`/`stat-time`/`c-ctype` convention), because
that is what `tools/extract-abi.py`'s pairing and `xtask build-tu lib/acl-internal.h` look
for: the round-53 request wrote it with an underscore, which would have left the mirror
unpaired and unchecked.

`--check` is OK: all 40 macros and every `permission_context` member the checker asks for
have a counterpart, including the ones no target compiles.

### Blind conditions, and what that costs

`tools/check-cond-eval.py lib/acl-internal.h` reports **1 disagreement** of its 36 conditions
(C 63's `#ifndef ACL_INTERNAL_INLINE`: clang sees the macro already defined, because
`lib/acl-internal.c` 20 does `#define ACL_INTERNAL_INLINE _GL_EXTERN_INLINE` before including
this header, while the frozen macro table has no entry for it).  A file with a disagreement is
not allowed to use the table, so the checker keeps *both* branches of every condition here —
the checklist therefore demands all 40 macros and the struct members of every platform arm,
not just the frozen one.  That is why the dead arms are written out rather than noted away.

### The frozen configuration

`USE_ACL` is 0 (C 67-238 is dead in full), `HAVE_SYS_ACL_H`, `HAVE_FACL`,
`HAVE_ACL_LIBACL_H`, `HAVE_ACLV_H`, `HAVE_GETACL`, `HAVE_STATACL`, `HAVE_ACLSORT`,
`HAVE_ACL_GET_FILE`, `HAVE_ACL_GET_FD`, `HAVE_ACL_SET_FD`, `HAVE_ACL_FREE_TEXT`,
`HAVE_ACL_EXTENDED_FILE`, `HAVE_ACL_FROM_MODE`, `HAVE_ACL_FROM_TEXT`, `HAVE_ACL_ENTRIES`,
`HAVE_ACL_TYPE_EXTENDED`, `HAVE_ACL_COPY_EXT_NATIVE`, `HAVE_ACL_CREATE_ENTRY_NP` and
`ACE_GETACL` are all undefined, and `HAVE_FCHMOD` is 1 (which kills C 57-60's `#ifndef` arm).
What survives is `ACL_INTERNAL_INLINE`, `struct permission_context`'s `mode` member and the
three prototypes at C 280-282.

### Dead arms (PORTING.md P-51)

Every dead arm keeps C's text under a feature that names the non-frozen side of its own
condition: `use_acl_have_acl_get_file` (C 69's `HAVE_ACL_GET_FILE`: Linux/FreeBSD/macOS),
`use_acl_getacl` (C 150's `HAVE_FACL && GETACL`: Solaris, Cygwin < 2.5), `use_acl_ace_getacl`
(C 161's `ACE_GETACL`: Solaris 10 — the predicate plus the 24 `OLD_*`/`NEW_ACE_*` names),
`use_acl_have_getacl` (C 201: HP-UX), `use_acl_have_statacl` (C 219: older AIX),
`use_acl_have_aclsort` (C 225: NonStop Kernel), `have_facl` (C 33), `no_fchmod` (C 57) and
`have_aclv_h` (C 43's HP-UX arm, whose two declarations are kept).

* Those features are **recorded text**: `xtask`'s FLIPS list has no `USE_ACL` entry and
  `src/include/config.rs` has none of this header's condition constants, so nothing compiles
  the arms — which is also what this target does with them.  Because they are never compiled,
  the types only the other platforms' headers supply (`aclent_t`, `ace_t`, `struct acl`,
  `struct acl_entry`, `NACLENTRIES`, `NACLVENTRIES`) are written in C's spelling inside them,
  exactly as C's own arms name them (the `gettimeofday.c` Windows-arm precedent, taken the
  other way: there the arm was given a feature the gate flips, here it is not).
* The struct's members carry their *own* arm's feature rather than one shared `use_acl`: three
  C arms declare `count` and two declare `entries`, so a single gate would have to define the
  same field twice should it ever be turned on.
* `MODE_INSIDE_ACL` appears three times in C (C 120 macOS, C 154 Cygwin, C 235's `#ifndef`
  fallback).  The fallback is unconditional in the frozen text (`= 1`) and is the one the
  checker's constant list carries, so it is written ungated; the two others are gated with the
  arms that define them.

### The three prototypes and `ACL_INTERNAL_INLINE` go through a whole-import module

`get_permissions`, `set_permissions` and `free_permission_context` are *defined* by the ACL
units that include this header, and `lib/acl-internal.c` defines `ACL_INTERNAL_INLINE` itself
(to `_GL_EXTERN_INLINE`, PORTING.md 10.12's wrapper shape — which is what makes clang's
`#ifndef` false).  A definition may not share a module with the mirror's own declaration
(E0428), so all four live in `pub mod acl_internal_decls` with `use acl_internal_decls::*;`:
the unit's definitions shadow the glob import (PORTING.md 0.1-10, the `acl.rs`/`filemode.rs`
shape).

### Verification

* `python3 tools/extract-abi.py --check emacs/lib/acl-internal.h femacs/lib/include/acl-internal.rs`
  → OK (only the informational "MODE_INSIDE_ACL has several definitions, body not compared").
* Compilation: the mirror is not a crate root (its `c_int`/`_GL_INLINE`/`_GL_CONFIG_H_INCLUDED`
  come from the including unit, as the lib mirrors expect), so it was compiled through the same
  include-graph expansion `xtask` uses (dependencies first, nested `include!` lines neutralised)
  in a `/tmp` probe: `rustc --edition 2024 --crate-type=lib --emit=metadata` → exit 0, no
  warnings.  That probe is what caught the missing `#[repr(C)]` on `permission_context` (the
  `improper_ctypes` lint on the three prototypes), which is fixed.
* `python3 tools/extract-abi.py --check` on `acl.h`, `sys_acl.h`-side mirrors and the other
  ported units still OK (the mirror is new; nothing else changed).
* Not run: `gate-fast`/`gate-probes` (the user runs them).  No behavioural probe: this file is
  declarations and dead arms, so there is nothing to execute on this target.
