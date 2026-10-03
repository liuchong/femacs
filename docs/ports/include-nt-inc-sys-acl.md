# emacs/nt/inc/sys/acl.h (25 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] ACL_H  (line 4, body da39a3ee5e6b)
- [x] ACL_TYPE_ACCESS  (line 13, body b6589fc6ab0d)
- [x] ACL_TYPE_DEFAULT  (line 14, body 356a192b7913)
- [x] NOMINMAX  (line 6, body 356a192b7913)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/sys/acl.h, complete for this target: the include guard, `NOMINMAX`, the `PSECURITY_DESCRIPTOR`/`acl_t`/`acl_type_t`/`acl_perm_t` typedefs, the two `ACL_TYPE_*` constants and the six `acl_*` prototypes.  **Two defects the probe found here** (both survived every text gate): the `<sys/types.h>` include was spelled `sys_types.rs.in`, while the file is `lib/include/sys_types.in.rs`, so the mirror would not have compiled; and `ssize_t` - which `acl_to_text` takes a pointer to - is not in that mirror at all (the port's units take it from `<unistd.h>`'s mirror), so the prototype names it the way this mirror's other system types are named: from the including unit.  **This ledger's name is disambiguated**: the plain `include-acl.md` belongs to `lib/acl.h`, so the stem rule's third candidate - the reference path carrying its own directories - gives this file `include-nt-inc-sys-acl.md`, and both tools (`tools/extract-abi.py`, `tools/gen-port-inventory.py`) accept a ledger only when its first line names the reference file (status 3.102).

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it - `spikes/f_inc_probe.rs`
compiles it instead, and that probe found three real defects that every text gate had passed
(docs/status.md 3.102).  Its values are C's own and need
no clang measurement: they are literals and typedefs from the Windows SDK's own spellings.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/sys/acl.h
femacs/nt/inc/sys/include/acl.rs` -> `OK`.
  `../spikes/f_inc_probe.rs` compiles this mirror: it is the only compiler it has.
