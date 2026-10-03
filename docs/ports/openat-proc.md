# emacs/lib/openat-proc.c (154 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] openat_proc_name  (line 47)

## Macros defined — 1
- [x] PROC_SELF_FD_FORMAT  (line 61, body cb6ef94347f8)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/openat-proc.c` @ `fd7afced403`; port `femacs/lib/openat-proc.rs` (188 lines).  All
checklist items ticked; `build-tu lib/openat-proc.c` -> `built lib/openat-proc.rs ->
../build/femacs/openat-proc.o` (0 errors), `--check` -> `OK`; the object defines
`_openat_proc_name`, the reference object's only definition.

- **Which arm is live.**  `__KLIBC__`/`__MVS__` are undefined, so C 59-113's generic
  `/proc/self/fd` arm is the frozen build's text (the reference object calls `_access`,
  `_close`, `_malloc`, `_open`, `_sprintf`, `_strcpy`, `_strlen` - exactly the entries that arm
  names); C 33-38's two includes and C 114-150's klibc / z-OS arm are the non-frozen sides,
  kept as `#[cfg (feature = "klibc_or_mvs")]` text (P-51) whose entries this target has not got
  (`__libc_Back_ioFHToPath`, `_MAX_PATH`, `w_ioctl`, `_IOCC_GPN`, `_XOPEN_PATH_MAX`,
  `__e2a_l`).
- **`PROC_SELF_FD_FORMAT`** is a *constant* in the checklist (a string macro): the port carries
  C's bytes NUL included as `&[u8; 18]` (17 characters + NUL), so `size_of_val` is C's
  `sizeof PROC_SELF_FD_FORMAT` in the `PROC_SELF_FD_DIR_SIZE_BOUND` expression (18 - 2 + 11 =
  27) and `.as_ptr ()` is the `char *` the `sprintf` calls take.  C's literal concatenation
  `PROC_SELF_FD_FORMAT "../fd"` is written as the one literal it produces.
- **`intprops.h`'s `INT_STRLEN_BOUND`** has no mirror yet: its gnulib formula
  `((width - signed) * 146 + 484) / 485 + signed` lands in the unit with its only consumer
  (= 11 for `int`).
- **`openat-priv.h` mirror** is new: `SAFER_ALLOCA_MAX`, `SAFER_ALLOCA`,
  `OPENAT_BUFFER_SIZE` (= `SAFER_ALLOCA (PATH_MAX)` = 1024, the `PATH_MAX` arm being live),
  `EXPECTED_ERRNO`, and the `openat_proc_name`/`at_func2` prototypes in a glob-imported module.
- **Shared mirror**: `<unistd.h>`'s `F_OK`/`X_OK`/`W_OK`/`R_OK` moved into
  `include/unistd.in.rs` (tree-wide grep for same-named local declarations: only
  euidaccess.rs had them, and its four local constants were removed - an explicit item shadows
  a glob import, so that file kept working); `euidaccess` and the other unistd consumers were
  rebuilt.
- Line drift: `line-drift-tolerance.tsv` records `openat-proc 0.25` (C 154 vs 188, 22.1%).
- Not verified: gate-fast/gate-probes, ERT, and any behavioural test.
