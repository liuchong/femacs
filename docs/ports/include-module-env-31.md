# emacs/src/module-env-31.h (0 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **This file is a substituted fragment, not a header with its own includers.**  `configure`
  replaces the `@module_env_snippet_31@` placeholder in `src/emacs-module.in.h`'s
  `struct emacs_env_31` (C 146-235) with this file's **whole body**, so no translation unit
  `#include`s it and the generated header carries its text inline.  The judge is therefore
  `build/emacs-ref/src/emacs-module.h` (the `lib/dirent.in.h`/`lib/errno.in.h` reading, status
  3.159): `struct emacs_env_31` starts at the judge's line 1110, and this fragment's text is
  repeated in every later record.
- **Its members cannot be declared in this mirror.**  Rust pastes an `include!` where a macro or an
  item may appear; a struct's member list is neither, and the attempt is a hard error with the
  pinned toolchain (`error: unexpected token: `!` … note: macros cannot expand to struct fields`,
  measured and quoted in `include/emacs-module.in.rs`).  So this mirror is the fragment's **text**,
  and the Rust *spellings* of its members live in `include/emacs-module.in.rs`, in the eight
  `#[repr(C)]` records the placeholders would fill, in C's member order.
- **What this fragment contributes, measured from the judge**: no member at all; the record it first appears
  in is 320 bytes - the same record as `struct emacs_env_28`.
- **The mirror carries C's text verbatim** (each line prefixed with the reference's line number),
  so the fragment's own content has a home in the port even though it cannot be a declaration here.
  Nothing else is carried: the C file is empty (0 bytes).
- **The zero-item checklist is the point, not an omission**: the extractor finds no entry to ask
  for, so the disposition record above is what a reader gets, and the member-level facts are
  asserted in the parent mirror against the judge's compiler.

## Evidence
- `python3 tools/extract-abi.py --check emacs/src/module-env-31.h femacs/src/include/module-env-31.rs`
  -> `OK emacs/src/module-env-31.h -> femacs/src/include/module-env-31.rs` (no `- ` problem lines).
  The extractor's item list for this file is **empty** (`0 subrs, 0 functions defined, 0 macros,
  0 types, 0 file-scope objects`), because its text is a bare member list with no enclosing
  record: a fragment cannot be a translation unit, so there is nothing for the checklist to ask
  for - which is why the disposition record above is the whole mirror.
- `rg -n 'module-env-' emacs/src | grep -v '^emacs/src/module-env'` -> only `src/emacs-module.c`
  23, 41, 44 and 58, all of them prose in the release-procedure comment; no `#include` anywhere.
- `rg -n 'module-env-31' build/emacs-ref/src/emacs-module.h` -> the fragment's text sits inlined in
  the judge ('s record: `struct emacs_env_31` starts at judge line 1110, and the same text is
  repeated in every later record up to `struct emacs_env_32` (judge 1291).
- **Compile evidence**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask
  -- layout-check` -> `layout-check: 77 of 88 mirror(s) compiled, 11 platform-exclusive skipped`
  and `layout-check passed (see build/femacs/layout_asserts.rs)`.  This mirror is among the 77: it
  is an ordinary Rust file (comments only), so the single-crate probe compiles it.
- The member offsets the judge measures for this fragment are asserted by
  `include/emacs-module.in.rs` (see that ledger's probe reading): this fragment contributes
  no member at all.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 589 .rs files / 592 map targets / 0 findings.

