# emacs/lib/boot-time.h (49 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _BOOT_TIME_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/boot-time.h` @ `fd7afced403`; port `lib/include/boot-time.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/lib/boot-time.h lib/include/boot-time.rs`
-> `OK`.

- Built for `src/filelock.c`, which includes `<boot-time.h>` at C 41 (gnulib's `get_boot_time`
  is what `get_boot_sec` calls); the unit that includes it brings `c_int`/`c_long` into scope
  itself (the `lib/include/stdbit.rs` rule), so this mirror imports nothing.
- The declaration sits in `pub mod boot_time_decls { use super::*; unsafe extern "C" { … } }`
  with a glob import after it (the tempname.rs shape): `lib/boot-time.c`'s port will define
  `get_boot_time`, and an explicit item shadows a glob import rather than colliding with it
  (PORTING.md 0.1-10, E0428 otherwise).
- C 24-26/45-47's `#ifdef __cplusplus` arm has no Rust counterpart.
- Compilation evidence before `filelock.c` existed: a probe crate of the including unit's shape
  (`use std::ffi::{c_char, c_int, c_long, c_void};` + this mirror) compiles with
  `rustc --emit=metadata`, so the mirror is not merely declaration-checked.
- Line count is 24 vs C's 49; the header is drift-exempt (`is_header_like`).
