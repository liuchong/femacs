# emacs/lib/canonicalize-lgpl.c (474 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 8
- [x] file_accessible  (line 101, static)
- [x] suffix_requires_dir_check  (line 119, static)
- [x] dir_check  (line 159, static)
- [x] get_path_max  (line 166, static)
- [x] realpath_stk  (line 192, static)
- [x] __realpath  (line 434)
- [x] __old_realpath  (line 455)
- [x] __canonicalize_file_name  (line 470)

## Macros defined — 13
- [x] DOUBLE_SLASH_IS_DISTINCT_ROOT  (line 93, body 7cb6efb98ba5)
- [x] GCC_LINT  (line 46, body 356a192b7913)
- [x] IF_LINT  (line 89, body da39a3ee5e6b)
- [x] _GL_ARG_NONNULL  (line 22, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_PURE  (line 47, body 625b6a33ea92)
- [x] __canonicalize_file_name  (line 49, body f604d7b37bb1)
- [x] __faccessat  (line 53, body 57a4050245e7)
- [x] __getcwd  (line 72, body b894cce57609)
- [x] __mempcpy  (line 74, body 2466f0af953f)
- [x] __pathconf  (line 75, body bc5360786dd9)
- [x] __readlink  (line 76, body cd11917efc07)
- [x] __realpath  (line 50, body ff2b5744be5f)
- [x] __strdup  (line 51, body e86842bac741)

## Types and their fields — 1
- [x] realpath_bufs: struct scratch_buffer rname, struct scratch_buffer extra, struct scratch_buffer link

## File-scope objects — 0


## Decisions

`emacs/lib/canonicalize-lgpl.c` @ `fd7afced403`; port `femacs/lib/canonicalize-lgpl.rs` (567 lines).
All 22 checklist items are ticked.  Evidence: `xtask build-tu lib/canonicalize-lgpl.c` →
`built canonicalize-lgpl.rs -> build/femacs/canonicalize-lgpl.o` (0 errors) and
`extract-abi.py --check` → `OK`.

- **Which arm is live.**  The frozen configuration leaves `FUNC_REALPATH_WORKS` undefined, so
  C 96-449 (`#if defined _LIBC || !FUNC_REALPATH_WORKS`) is the *live* text: the whole
  implementation is ported, not just the `canonicalize_file_name` wrapper.  C 43-83's
  `#ifdef _LIBC` arm is dead (glibc internals: `shlib-compat.h`, `GCC_LINT`,
  `_GL_ATTRIBUTE_PURE`, `libc_hidden_def`, `versioned_symbol`, `weak_alias`); the port keeps
  the two macros the checklist names (`GCC_LINT` as `1`, `_GL_ATTRIBUTE_PURE` recorded in
  `docs/ports/excluded-macros.txt` as a syntax-only attribute) and records the rest here.
- **Rename macros and `export_name`.**  C 48-51/53/72-76's `#define`s rename the *definitions*
  (`__realpath` → `realpath`, `__canonicalize_file_name` → `canonicalize_file_name`), so the
  port keeps the C names on the functions and emits the macro-expanded symbols with
  `#[unsafe(export_name = …)]` (the memrchr/mempcpy precedent, PORTING.md 10.13).  The nine
  macros are kept as `macro_rules!` with the translations they name, so their bodies and
  call sites keep C's tokens.
- **`goto error` → labelled block** (PORTING.md 2.2): C's two forward jumps are
  `break 'error` (the `'error` label keeps C's token, which `docs/ports/macro-body-notes.tsv`
  notes together with the `continue` of the two backup loops).  `switch (*end++)` in
  `suffix_requires_dir_check` is a `match` with C's arms in C's order (the same ledger row).
- **Zero-initialized locals** (PORTING.md 1.5): `struct realpath_bufs bufs;` is
  `std::mem::zeroed ()` before the three `scratch_buffer_init` calls C makes, and
  `idx_t end_idx IF_LINT (= 0);` is `let mut end_idx: idx_t = 0;` plus the `IF_LINT!`
  invocation C spells (the frozen arm of `IF_LINT` is the empty one).
- **`__set_errno` and `INT_ADD_OVERFLOW`.**  libc-config.h's `# define __set_errno(val)
  (errno = (val))` and intprops.h's `INT_ADD_OVERFLOW` are declared in the unit itself with
  their C prototypes as comments (the md5.rs shape for what the mirrors do not carry); the
  port's `errno ()` accessor comes from `include/errno.rs`.
- **Mirrors.**  Four new ones: `include-idx.md` (idx_t + IDX_MAX), `include-filename.md`
  (the Unix arm's eleven macros), `include-min-eloop-threshold.md` (MIN_ELOOP_THRESHOLD) and
  `include-scratch_buffer.md` (the struct + the three inline wrappers + the six macros);
  `lib/include/limits.in.rs` gained `PATH_MAX` (the frozen value is 1024).  The five extra
  libc entries this unit needs (`mempcpy`, `strdup`, `strcpy`, `pathconf`, `F_OK`/
  `_PC_PATH_MAX`) are declared in the unit, so no other agent's mirror was touched.
- **Not verified here**: gate-fast/gate-probes, the ERT suite, any link test, and the
  behavior of `realpath` itself.  The dead arms are text for the token gate only; they are
  never compiled by the frozen configuration.
