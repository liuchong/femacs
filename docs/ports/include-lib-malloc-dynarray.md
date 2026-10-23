# emacs/lib/malloc/dynarray.h (177 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] __dynarray_error_marker  (line 110, static)
- [x] __dynarray_error  (line 118, static)

## Macros defined — 1
- [x] _DYNARRAY_H  (line 95, body da39a3ee5e6b)

## Types and their fields — 2
- [x] dynarray_finalize_result: void* array, size_t length
- [x] dynarray_header: size_t used, size_t allocated, void* array

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`emacs/lib/malloc/dynarray.h` @ `fd7afced403`; port `lib/malloc/include/dynarray.rs`.
This is the shared glibc header.  The gnulib wrapper (`emacs/lib/dynarray.h`) reaches it in
*two* readings and both reach the same text: the `#if defined DYNARRAY_STRUCT …` arm includes
the generated `malloc/dynarray.gl.h`, and the `#else` arm includes `<malloc/dynarray.h>`; the
generated file differs from this header only by its `DO NOT EDIT!` first line and by the five
`libc_hidden_proto` lines (`diff build/emacs-ref/lib/malloc/dynarray.gl.h
emacs/lib/malloc/dynarray.h` is exactly those six lines).  The port therefore keeps the text
once, here, and `lib/include/dynarray.rs` includes it for the `#else` reading.

- **The two structs carry measured layout assertions.**  `docs/baseline/layout` has no row for
  `struct dynarray_header`/`struct dynarray_finalize_result`, so the numbers are the frozen
  target's clang values measured with a probe in `/tmp` (excluding the reference tree):
  `dynarray_header` sizeof 24 / alignof 8 / `allocated` at 8 / `array` at 16;
  `dynarray_finalize_result` sizeof 16 / alignof 8 / `length` at 8.  They are recorded as a
  measured deviation from the (absent) baseline and asserted in the mirror.
- **The five entries are declared in a module the mirror glob-imports** (`mod libc_dynarray`
  + `use libc_dynarray::*;`, PORTING.md 0.1-10's shape).  A unit that *defines* one of them
  (`lib/malloc/dynarray_*.c`) then shadows the glob import, which is C's "prototype +
  definition"; a crate-root declaration would be `E0428` for every defining unit.
- **The renamed symbol is spelled with `#[link_name]`.**  `lib/dynarray.h` 252-256 renames
  `__libc_dynarray_resize` to `_gl_dynarray_resize`, and the frozen build's objects export
  that name: `nm -g build/emacs-ref/lib/malloc/dynarray_resize.o` prints
  `T __gl_dynarray_resize` (Mach-O's spelling of the C symbol `_gl_dynarray_resize`).  Rust's
  `#[link_name]`/`#[unsafe(export_name)]` values are the C identifier, and the Mach-O object
  writer adds the leading `_`, so the paired port objects print the same
  `T __gl_dynarray_resize`.
- **`<stddef.h>`/`<string.h>` scalars.**  The header names no function of its own, but the
  structs name `void *`; the alias lives in `mod stddef { pub use std::ffi::c_void; }` that the
  mirror glob-imports, so the mirror is a compilable root on its own and an including unit's
  own `use std::ffi::c_void` shadows the glob (verified: an explicit import shadows a glob
  import; two explicit imports of one name are `E0252`).
- **The two `static inline` helpers are functions with bodies** (PORTING.md 0.1's practice,
  the include/scratch_buffer.rs shape): `#[inline(always)] pub unsafe fn`.  `header` emits no
  symbol, so the export rule does not apply to a header mirror.
- C 170-174's five `libc_hidden_proto` lines only bind symbols; there is nothing to port.
- Everything else in C 20-92 (a typical example) and C 176-204 (a second example) is
  documentation, carried verbatim above; the reference's opening block is not carried
  (PORTING.md 0.1-19).

## Evidence

- `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-dyn cargo run -q -p xtask -- build-tu
  lib/malloc/dynarray.h` -> `built lib/malloc/include/dynarray.rs ->
  ../build/femacs/include_dynarray.o`.
- `python3 tools/extract-abi.py --check emacs/lib/malloc/dynarray.h
  lib/malloc/include/dynarray.rs` -> `OK`.
- manual `rustc --edition 2024 --crate-type=lib --emit=metadata --crate-name dyn_check
  -C panic=abort -C codegen-units=1 -C opt-level=0 ../build/femacs/units/dynarray/_root.rs`
  -> exit 0.
- The mirror is also compiled through every unit that includes it (the five
  `lib/malloc/dynarray_*.c` units and `lib/regexec.rs`), each `build-tu` = `built`.
