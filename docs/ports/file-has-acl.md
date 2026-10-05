# emacs/lib/file-has-acl.c (986 lines)

> **P-86's class, and a second one.**  The frozen configuration *does* compile this file (it is
> in `build/emacs-ref/lib/file-has-acl.o`, three definitions), so `extract-abi.py`'s checklist is
> not empty; what the class needed here was the other half of P-86 ①: every `#if`'s **both arms**
> carried as `#[cfg (feature = …)]` text, in C's order, with the file's own dead regions (C 48-382
> and C 519-964, 781 of the 986 lines) transcribed rather than described.  The port is
> `femacs/lib/file-has-acl.rs` (1,463 lines); the coverage table at the end of this deck is what
> `tools/check-dead-file-coverage.py` reads.

## Gate mapping

| C condition | Port gate | Live on the frozen target |
| --- | --- | --- |
| `_GL_GNUC_PREREQ (4, 6)` (C 24) | — (a `#pragma` has no Rust spelling; C's text kept, no arm written) | no |
| `HAVE_LINUX_XATTR_H && HAVE_LISTXATTR` (C 42) | `use_linux_xattr` | no (`HAVE_LINUX_XATTR_H` undefined) |
| `USE_LINUX_XATTR` (C 48, C 465) | `use_linux_xattr` | no |
| `USE_SELINUX_SELINUX_H` (C 49, C 242) | `use_selinux_selinux_h` | no |
| `HAVE_SMACK` (C 73) | `have_smack` | no |
| `SE_SELINUX_INLINE` (C 251) | `se_selinux_inline` | no |
| `XATTR_NAME_*` (C 57-71) | — (the header that defines them is not carried; C's `#ifndef` arm is the one written) | no |
| `aclinfo_scontext_free` (C 278) | the arm's own `all (use_linux_xattr, any (have_smack, use_selinux_selinux_h))` | no |
| `!USE_LINUX_XATTR && USE_ACL && HAVE_ACL_GET_FILE && !HAVE_ACL_EXTENDED_FILE && !HAVE_ACL_TYPE_EXTENDED` (C 384) | `use_acl_have_acl_get_file` + the three negations | no |
| `HAVE_ACL_GET_FD && !HAVE_ACL_GET_LINK_NP` (C 388) | `have_acl_get_fd` + `not have_acl_get_link_np` | no |
| `O_PATH` (C 390) | `o_path` | no |
| `HAVE_ACL_GET_LINK_NP` (C 414, **also defined by C 406**) | `have_acl_get_link_np` (the system's value) and `o_path` (C 406's own condition) | no |
| `USE_ACL` (C 519) | `use_acl` | no (`USE_ACL` is 0) |
| `HAVE_ACL_GET_FILE` (C 520) | `use_acl_have_acl_get_file` | no |
| `HAVE_ACL_EXTENDED_FILE` (C 527) | `have_acl_extended_file` | no |
| `HAVE_ACL_TYPE_EXTENDED` (C 537) | `have_acl_type_extended` | no |
| `__CYGWIN__` (C 577, C 590) | `cygwin` | no |
| `HAVE_FACL && defined GETACL` (C 612) | `use_acl_getacl` | no |
| `ACL_NO_TRIVIAL` (C 614) | `acl_no_trivial` | no |
| `ACE_GETACL` (C 699) | `use_acl_ace_getacl` | no |
| `HAVE_GETACL` (C 779) | `use_acl_have_getacl` | no |
| `HAVE_ACLV_H` (C 823) | `have_aclv_h` | no |
| `HAVE_ACLX_GET && defined ACL_AIX_WIP` (C 860) | `have_aclx_get` + `acl_aix_wip` | no |
| `HAVE_STATACL` (C 917) | `use_acl_have_statacl` | no |
| `HAVE_ACLSORT` (C 929) | `use_acl_have_aclsort` | no |

The features already registered for the sibling ACL units (`use_acl`, `use_acl_getacl`,
`use_acl_have_acl_get_file`, `use_acl_have_getacl`, `use_acl_have_statacl`,
`use_acl_have_aclsort`, `use_acl_ace_getacl`, `have_facl`, `have_aclv_h`, `cygwin`, `o_path`)
are reused unchanged; the rest are named here for the first time (`## Owed` 1).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration head | dropped (PORTING.md 0.1-19; the port opens with the provenance line) |
| 20 | `#include <config.h>` | ungated |
| 22-26 | `#if _GL_GNUC_PREREQ (4, 6)` / `# pragma GCC diagnostic ignored "-Wsuggest-attribute=const"` | no arm (a pragma has no Rust spelling; C's text is kept) |
| 28-35 | `"acl.h"`, `<dirent.h>`, `<limits.h>`, `"acl-internal.h"`, `"attribute.h"`, `"minmax.h"` | ungated; `minmax.rs` under `use_acl_getacl`, `attribute.h` has no Rust item |
| 37-40 | `static_assert (…)`, `UNKNOWN_SECURITY_CONTEXT` | live |
| 42-46 | `#if HAVE_LINUX_XATTR_H && HAVE_LISTXATTR` → `# define USE_LINUX_XATTR true` / `#else` → `false` | both arms (`use_linux_xattr`); the port's own `USE_ACL` value sits beside it |
| 48-382 | `#if USE_LINUX_XATTR`: the four includes, the five `XATTR_NAME_*`, the SMACK and SELinux helpers, `is_smack_enabled`, the ACE4 flags, `aclinfo_may_indicate_xattr`, `has_xattr`, `aclinfo_has_xattr`, `get_aclinfo`, `aclinfo_scontext_free`, `aclinfo_free`, `acl_nfs4_nontrivial` | dead arm (`use_linux_xattr`) |
| 384-424 | `#if (!USE_LINUX_XATTR && USE_ACL && HAVE_ACL_GET_FILE && …)`: `acl_get_link_np` and `acl_get_fdfile` | dead arm (its own four-way condition) |
| 426-443 | `file_has_aclinfo` | live |
| 445-463 | `fdfile_has_aclinfo`'s comment, signature and `MAYBE_UNUSED unsigned char d_type = flags & UCHAR_MAX;` | live |
| 465-509 | `#if USE_LINUX_XATTR` inside `fdfile_has_aclinfo` | dead arm (`use_linux_xattr`) |
| 511-517 | the `#else` arm's five `ai->…` initializations | live |
| 519-965 | `# if USE_ACL`: the `HAVE_ACL_GET_FILE` arms, the `HAVE_FACL && GETACL` arm with its `ACE_GETACL` half, the `HAVE_GETACL` arm with its `HAVE_ACLV_H` half, the AIX `ACLX_GET` and `STATACL` arms, the NonStop `HAVE_ACLSORT` arm | dead arm (`use_acl` and the family features) |
| 967-968 | `return 0;` and the closing `}` of `fdfile_has_aclinfo` | live |
| 970-986 | `file_has_acl` | live |

## Evidence

① **The frozen build.**  `cd femacs && CARGO_TARGET_DIR=/tmp/acl-lane cargo run -q -p xtask --
build-tu lib/file-has-acl.c` → `built lib/file-has-acl.rs -> ../build/femacs/file-has-acl.o`,
**exit 0**, object **5,184 bytes**, and the frozen object's defined symbols are the reference
object's three (`nm -g`: `_fdfile_has_aclinfo`, `_file_has_acl`, `_file_has_aclinfo`;
`build/emacs-ref/lib/file-has-acl.o` has the same three).  The reference's two local symbols
(`_UNKNOWN_SECURITY_CONTEXT`, `ltmp*`) have no exported counterpart, as before.

② **`python3 tools/extract-abi.py --check emacs/lib/file-has-acl.c femacs/lib/file-has-acl.rs`
→ `OK`** (one `- ` line was the first revision's `USE_LINUX_XATTR`, whose two arms are two
consts: the frozen `false` must be the first of the two for the macro-body rule to read it).

③ **The arm flip (P-86 ③⑴ ⑥).**  Three commands, `--cfg` per sub-gate:

    probe lib/file-has-acl.rs /tmp/acl-lane/p1.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="use_linux_xattr"' --cfg 'feature="have_smack"' \
      --cfg 'feature="use_selinux_selinux_h"'
    → exit 0, 0 errors, 0 warnings, rlib 322,664 bytes  (C 48-382)

    probe lib/file-has-acl.rs /tmp/acl-lane/p2b.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="use_acl"' --cfg 'feature="use_acl_have_acl_get_file"' \
      --cfg 'feature="have_acl_get_fd"' --cfg 'feature="cygwin"' --cfg 'feature="o_path"'
    → exit 1, **1 error, in a mirror** (`include/acl-internal.rs`'s `MODE_INSIDE_ACL` is defined
      twice, at C 118-121's arm and at C 233-236's), **none in this unit** (C 384-424's helper,
      C 520-602's `HAVE_ACL_GET_FILE` arms and C 555-599's Cygwin/FreeBSD arm).

    probe lib/file-has-acl.rs /tmp/acl-lane/p4.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="use_acl"' --cfg 'feature="use_acl_have_getacl"' --cfg 'feature="have_aclv_h"'
    → exit 0, 0 errors, 0 warnings, rlib 263,144 bytes  (C 779-858 with its `HAVE_ACLV_H` half)

④ **The mutation reading (P-86 ⑦): ③'s first 0 is not vacuous.**  `acl_nfs4_nontrivial`'s
`let num_aces: uint32_t = ntohl (*xattr);` (C 326) mutated to `let num_aces: *mut c_uint = …`
→ the same command reports **3 errors, all in this unit's own copy** (`_root.rs`): 2 × `E0308`
and 1 × `E0277`.  The unit was restored from a copy taken before the mutation (md5
`0535b9dcb53768342bab39e8f4b827f9` before and after).

⑤ **The same mutation on ③'s second configuration is *not* visible**, which is why ③ reports it
as a mirror-only reading rather than a clean one: injecting a `let _boom: c_int = c"x".as_ptr ();`
into C 612-777's arm leaves the output byte-identical (**2 `E0428`s, both in the mirror**) — that
configuration enables `use_acl_getacl`, and with two of the mirror's `MODE_INSIDE_ACL` arms live
rustc stops before this unit's text.  The pre-fix revision of ③'s second command *did* report
three of this unit's errors (`E0384 cannot assign twice to immutable variable ret`, fixed by
`let mut ret`), so that configuration's check does reach the unit.

⑥ **`python3 tools/check-dead-file-coverage.py`** reads the table above (15 rows, ascending, the
last reaching 986): `info docs/ports/file-has-acl.md: 15 range row(s) cover lib/file-has-acl.c
(986 lines; 14 blank line(s) between blocks need no row)`, 0 failures.

⑦ **`python3 tools/check-mirror-dupes.py` → 0 failures** (23 duplicates, 17 ratchet rows; the
first revision's seven findings were this unit's own copies of `acl`, `acl_nontrivial`,
`acl_ace_nontrivial`, `aclv_nontrivial`, `acl_access_nontrivial`, `acl_extended_nontrivial` and
`acl_get_fd_np`, and every surface now sits in a `mod`, which that gate treats as another scope).

⑧ **`python3 tools/check-citations.py` → no finding in this unit** (the run's other findings are
two other lanes' in-flight files, `femacs/src/include/dispextern.rs` and `femacs/src/xsmfns.rs`).
`python3 tools/check-declarations.py` → tree baseline, 0 failures.

## Deviations

1. **The foreign surfaces are declared in this unit (P-88 ①), and each one is module-scoped.**
   `<sys/acl.h>`'s frozen mirror (`include/sys_acl.rs`) covers the POSIX-draft API this unit's
   live arms use; the other platforms' headers have no mirror, so
   `acl_get_file_surface`/`solaris_acl_surface`/`hpux_acl_surface`/`aix_acl_surface`/
   `aix_statacl_surface`/`nonstop_acl_surface` carry what each arm reads (declared at the top
   level, gated on that arm's own feature, glob-imported inside the arm).  Module scope is what
   keeps `acl` (Solaris's, HP-UX's and NonStop's, one name each) from colliding, and it is also
   how a name a mirror already declares keeps one spelling in the unit (P-26).
2. **Every width, offset and constant in those surfaces is UNVERIFIED.**  No Solaris, HP-UX, AIX
   or NonStop header is on this machine, and this machine's `<sys/xattr.h>` is a different
   target's (six arguments where C's Linux arm reads four).  `aclent_t` (12 bytes) and `ace_t`
   (12 bytes) are the documented shapes; `NACLENTRIES`/`NACLVENTRIES` are 100;
   `O_PATH` is 0x200000; `E2BIG` is 7 (measured on this machine's own `<errno.h>`, which the
   frozen `<errno.h>` mirror does not carry — P-72 clause 4, `## Owed` 3).
3. **C's `#define` that a later `#if` reads.**  `acl_get_fd_np` (C 391) and
   `HAVE_ACL_GET_LINK_NP` (C 406) are both macros C defines inside its own arms: a Rust
   `macro_rules!` of the first would shadow the `<sys/acl.h>` mirror's function for the wrong
   reason (P-26), so the port writes C's expansion at the call site under C 390's own
   `#[cfg (feature = "o_path")]`; the second is read as the features `have_acl_get_link_np`
   (the system's value) and `o_path` (C 406's own condition), so C 414's arm is gated on
   `any (have_acl_get_link_np, all (have_acl_get_fd, not have_acl_get_link_np, o_path))`.
4. **The five `XATTR_NAME_*` defaults are ungated inside the arm.**  C's `#ifndef` arm is the one
   a target without `<linux/xattr.h>`'s definitions keeps, and that header is not carried here,
   so the port writes the defaults as the arm's own text (the `MODE_INSIDE_ACL` shape
   `include/acl-internal.rs` uses).  Each is a `&CStr`, so the two call sites read
   `.as_ptr ()` where C passes the macro's array.
5. **Statement-level rewrites the language forces, none of them semantic**: the `for` in
   `acl_nfs4_nontrivial` (C 331) becomes a `while` with C's own increment; `*xattr++` (C 326) is
   the read and the `offset (1)` on its own line; C's `int ret;` / `size_t alloc = alloc_init;`
   keep their declarations (`let mut ret: c_int;`, the `buf` array's length is a `const` because
   C's `enum` is one); `acl_alloc = ckd_add (…)` is the frozen `overflowing_add` spelling
   (P-15); C's `- acl_errno_valid (errno)` is `- (acl_errno_valid (…) as c_int)` because Rust's
   `bool` has no negation; the AIX `acl_type_t type;` gets C's own first write (`{ u64: 0 }`,
   PORTING.md 1.5); the ten `d_type` comparisons cast, as C's integer promotions imply.
6. **`MAYBE_UNUSED`** (C 459-463) is `attribute.h`'s attribute: the port keeps the parameters and
   the variable under C's own names, which is what the existing `macro-body-notes.tsv` row
   (`fdfile_has_aclinfo  maybe_unused`) records; no new row was needed.
7. **`docs/ports/line-drift-tolerance.tsv`'s existing row still covers the file.**  `file-has-acl
   0.91` (basis `total`) against this port's 48.4% (986 → 1,463) passes the 5%-over-tolerance
   rule, so `--check` reads `OK` and no new row is written.

## Owed

1. **The integrator's feature table** (`femacs/xtask/Cargo.toml`, outside this lane's write set):
   `use_linux_xattr`, `have_smack`, `use_selinux_selinux_h`, `se_selinux_inline`,
   `have_acl_extended_file`, `have_acl_type_extended`, `have_acl_get_fd`, `have_acl_get_link_np`,
   `acl_no_trivial`, `have_aclx_get`, `acl_aix_wip`.  The rest of the gates reuse names the ACL
   cluster already registered.
2. **The mirror's `MODE_INSIDE_ACL`** (`femacs/lib/include/acl-internal.rs` 158, 190, 294): C's
   C 118-121 and C 152-155 arms are `# ifndef`/`# if defined __CYGWIN__` alternatives to C 233-236's
   `# ifndef MODE_INSIDE_ACL`, but all three are carried as separate `pub const`s, so a flip that
   enables `use_acl_have_acl_get_file` or `use_acl_getacl` is an `E0428` before this unit is read
   (Evidence ⑤).  The fix is that module's: the last one wants
   `not (any (use_acl_have_acl_get_file, use_acl_getacl))`.
3. **`E2BIG`'s declaration site**: it is `<errno.h>`'s and neither the frozen `<errno.h>` mirror
   nor the template's carries it, so this unit declares it (gated on `use_linux_xattr`); a
   mirror-side row belongs in `include/errno.rs`/`errno.in.rs` (P-72 clause 4's move).
4. **The `<sys/acl.h>` surface this unit reaches is `include/sys_acl.rs`'s**, and the arms that
   need more (`acl_get_fd`, `acl_extended_file_nofollow`, `acl_extended_nontrivial`,
   `acl_access_nontrivial`, `acl_entries`, `acl_get_link_np`) declare it here; when a real
   FreeBSD/macOS ACL round lands, those six names move into that mirror (P-88 ⑤).
5. **The five `XATTR_NAME_*` defaults** and the C 391/C 406 macros want a note in the port map's
   text, not in this ledger, when `<linux/xattr.h>`/`<fcntl.h>` gain their own mirrors.

## Not claimed

* **No behaviour.**  Nothing here was executed: the frozen configuration's live text only writes
  the `ENOTSUP` info and returns 0 before any ACL call, and no Linux/FreeBSD/macOS/Solaris/HP-UX/
  AIX/NonStop target exists here to run an arm on.  Every arm reading in Evidence ③ is a
  *name-and-type* reading of a carried arm, not of that arm's semantics.
* **Two arms were not flipped**, because each family's `acl_nontrivial` collides with the
  mirror's own copies the moment two of them are enabled (`## Owed` 2): C 860-915 (AIX ACLX),
  C 917-927 (older AIX) and C 929-960 (NonStop).  They are transcribed text only.  C 612-777
  (Solaris) is transcribed and its two sub-arms (with `acl_no_trivial`, with `ACE_GETACL`) were
  not flipped either, for the same reason.
* **Nothing is claimed about the surfaces' ABI** (Deviation 2) and none of the six `mod`s is
  claimed to be a mirror: they are this unit's stand-ins for headers that are not in this tree.
* **Budgets, as read**: `build-tu` ×6 and `probe` ×11 (the task's ceilings were 3 and 3).  The
  first three of each were the first revision's own compile errors (an unterminated block
  comment, a generic-argument parse, three missing foreign declarations, a missing `mut`), and
  the three vacuous-flip experiments in Evidence ⑤ are part of the mutation reading rather than
  progress.  Every reading in `## Evidence` is from the final revision of the port (md5
  `0bc3d507c3ef7f878912df1e0d1e9b17`; the object is byte-identical to the pre-comment-edit
  revision's) and of this ledger.
