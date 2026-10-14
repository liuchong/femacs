# emacs/src/w32heap.h (69 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] NTHEAP_H_  (line 23, body da39a3ee5e6b)

## Types and their fields — 1
- [x] file_data: char* name, unsigned long size, HANDLE file, HANDLE file_mapping, unsigned char* file_base

## File-scope objects — 1
- [x] extern size_t reserved_heap_size  (line 33)

## Decisions

- **The header is unconditional, and this target compiles nothing from it.**  C 22-23 is the
  include guard, C 25 is `#include <windows.h>`, and C 31-67 are live text — no `#if` anywhere
  (`tools/check-cond-eval.py src/w32heap.h` -> `1 conditions, 0 unknown, 0 disagreements
  [VERIFIED]`, the guard).  The frozen configuration's platform object variables are empty
  (docs/baseline/frozen-build-sources.txt; docs/status.md 3.31), so `src/w32heap.c` has no
  translation unit here: `femacs/src/w32heap.rs` is the empty shape that names the exclusion (its
  ledger records the 24 deferred declarations of that file), and no unit `include!`s this mirror.
- **The platform's typedefs are spelled as their port primitives, with C's text in a `/// C:`
  line.**  `HANDLE` (C 53-54), `IMAGE_SECTION_HEADER`/`IMAGE_NT_HEADERS` (C 63, 67) and `DWORD_PTR`
  (C 37, 67) all come from `<windows.h>` (C 25), for which the port has no mirror.  So:
  `HANDLE` -> `*mut c_void` (an opaque handle is a pointer at the ABI); the two `IMAGE_*` names are
  only ever pointers here, and no mirror owns either record, so they are `*mut c_void` (PORTING.md
  P-55); `DWORD_PTR` -> `usize` (C's pointer-sized unsigned, the same row as `uintptr_t` in
  PORTING.md 1.1).  Arity, pointer depth, `const`-ness and the `int`/`void` returns are C's.
- **`struct file_data` is carried with the field order and the C member names C 50-56 gives it**,
  and its `HANDLE` members take the same `*mut c_void` reading.  The record's **layout is not
  asserted** anywhere: the assertions `xtask layout-check` compares come from a C compiler, and no
  compiler on this target has a `<windows.h>`.  A Windows target that freezes this header must add
  the record to `tools/gen-layout-fields.py` and measure it there (the 3.158 `struct font` shape,
  which is only possible because that record is in a header this target compiles).
- **`sbrk`'s `ptrdiff_t` is `isize`** (PORTING.md 1.1), and `size_t` is `usize`; `unsigned long` is
  `c_ulong` (C 52's `size` member and C 59's parameter).
- **Use-site copies that do *not* need retiring**: `src/vm-limit.rs` 95-99 declares
  `reserved_heap_size` inside its `#[cfg (feature = "windowsnt")]` arm — dead text on this target,
  with the comment "w32heap.h declares this" above it.  That is the P-51 dead-arm shape and stays;
  the arm's comment is still true now that the header has a mirror.
- **`--check` asks for less than fidelity does**: the extractor's item list for this header is the
  guard, the `file_data` record and the one `extern size_t` object; C 31-45 and C 58-67 are
  declarations, so they are beyond the checklist and are carried because they are live text.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/w32heap.h femacs/src/include/w32heap.rs`
  -> `OK emacs/src/w32heap.h -> femacs/src/include/w32heap.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/w32heap.h` -> `1 conditions, 0 unknown to the evaluator,
  0 disagreements` (the guard).
- `python3 tools/extract-abi.py emacs/src/w32heap.h` -> this ledger: 0 subrs, 0 functions defined,
  1 macro (the guard), 1 type (`file_data`, 5 fields), 1 file-scope object (`reserved_heap_size`).
- `rg -n "get_data_start|get_data_end|reserved_heap_size|mmap_realloc|mmap_free|mmap_alloc|report_temacs_memory_usage|sbrk|init_heap|open_input_file|open_output_file|close_file_data|find_section|rva_to_section" femacs`
  -> outside this mirror: `src/vm-limit.rs` 95-104 (its `windowsnt` arm), one copied C comment
  (`src/buffer.rs` 4468), and `nt/inc/include/ms-w32.rs` 164-165 (`DATA_START get_data_start ()`,
  the Windows port's own mirror).  `rg -n 'include/w32heap' femacs` -> no unit `include!`s the
  mirror.
- **Compile evidence**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask -- layout-check`
  -> `layout-check: 66 of 77 mirror(s) compiled, 11 platform-exclusive skipped (android-asset.rs,
  android.rs, androidgui.rs, androidterm.rs, dispextern.rs, gtkutil.rs, haiku_support.rs,
  haikugui.rs, haikuterm.rs, nsgui.rs, pgtkterm.rs)`, `layout probe ok`,
  `layout-check passed (see build/femacs/layout_asserts.rs)`.  **This mirror is among the 66** (it
  is not in that skip list), so the gate compiles it every round.  An earlier run of the same gate,
  with only `cygw32.rs` of this batch present, printed `layout probe ok`; a later one failed with
  `error[E0428]: the name Emacs_Cursor is defined multiple times … nsgui.rs:57:1 …
  dispextern.rs:62:1` - two mirrors of another lane, both of which had entered the skip list by the
  time of the passing run, and no diagnostic of either run named a mirror of this batch.  Nothing in this mirror needed a `cfg`.

- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 576 .rs files / 592 map targets / 0 findings (the file count includes mirrors other lanes landed in the same session).

## 2026-10-05: the five entry points move into `w32heap_decls` (3.513)

C 36-38/41-45's `mmap_realloc`/`mmap_free`/`mmap_alloc`/`sbrk`/`init_heap` are the five names
`src/w32heap.c` defines (C 100-151, 300-310), so their top-level copies were `E0428` in that unit
(the lane's flip, status 3.504: five of its 33 errors).  They live in a `w32heap_decls` module the
mirror glob-imports (P-39; `get_data_start`/`get_data_end`/`report_temacs_memory_usage`/
`reserved_heap_size` stay at the top level - that unit does not define them).  Same flip after the
move: **33 -> 27 errors**, the rest named in `docs/ports/w32heap.md`'s Owed.

## 2026-10-14: the four mmap-file entry points move into `w32heap_file_decls` (4.291)

The same surgery the 2026-10-05 section records, for the four names C 58-60 and C 67 declare:
`src/w32proc.c` **defines** `open_input_file` (C 2451), `rva_to_section` (C 2485) and
`close_file_data` (C 2512) - `w32heap.c` defines none of them - so the top-level declarations were
three `E0428`s in `src/w32proc.rs` the moment that unit's slice 12 landed.  Measured:
`probe src/w32proc.rs` read **40 errors (30 `E0428` + 9 `E0425` + 1 `compile_error!`), 0 anchored
in the unit** before the move (the three extra `E0428`s all anchored here, at this mirror's 112/114
and 132), and **37 (27 + 9 + 1)** after it - the lane's baseline, unchanged.  The four names sit in
`w32heap_file_decls`, which this mirror glob-imports (P-39); `open_output_file` (C 59) is no unit's
definition yet and moves with its siblings, because C 59 declares it beside them and the same defect
was waiting for its definer.  The module is **not** gated: the four declarations were ungated at the
top level, and a Windows-only gate would have narrowed what every configuration sees.
