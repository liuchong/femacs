# emacs/lib/openat-die.c  →  lib/openat-die.rs (7 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] openat_save_fail  (line 6)
- [x] openat_restore_fail  (line 7)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file is on the verified list and the frozen table decides every condition in it; nothing
  is demanded of the port because the reference compiles no definition from it.
- the file's live text is its three includes and nothing else - no function, macro, type or object survives the frozen configuration's preprocessing (the `openat_save_fail`/`openat_restore_fail` definitions this gnulib module is named for are reached through `openat.h`, which this build leaves to the system header).
- Policy: docs/status.md 3.25 (a file the frozen configuration strips to its includes is
  replicated as an empty unit); the tolerance row records the line relation.

- **PORTING.md P-81**: this is one of gnulib's extern-inline carriers - its C text is
  the `#define <qualifier> _GL_EXTERN_INLINE` line and its includes, with the bodies
  in the header - so the reference object exports the *header's* functions and the unit
  re-exports them under C's own names (`#[unsafe(export_name = "…")]` inside
  `mod <stem>_unit`, each calling `super::…` so the module's own item cannot shadow the
  mirror's and recurse).  tools/check-symbols.py compares the result with the reference
  object symbol by symbol; this entry was on docs/baseline/symbol-pending.txt until it
  matched.
  C 6-7 are the two definitions themselves (`void openat_save_fail (int errnum)
  { abort (); }`), and the reference object exports both, so the port defines them at the
  unit's root under C's names.  They are one-liners, a shape cinventory's definition scan
  did not see (it keys on the name being the line's first token), which made these two
  exports read as invented; tools/cinventory.py now has `typed_def_name` for the
  same-line form, gated on the body's `{` sharing the physical line so no multi-line
  declarator is re-read.  The line's own tokens are exempted in
  docs/ports/macro-body-notes.tsv because the extractor's body span for a one-liner is
  the whole line.

## 2026-10-07: 行数容差 3.70 -> 3.80（3.900）

`docs/ports/line-drift-tolerance.tsv` 的这一行是在端口 32 行时写的（357.1%）。第二十三半
（status 3.886）给本文件补上了 P-86 1 的出处行（`// femacs: port of lib/openat-die.c @
fd7afced403.`），33 行的比值因此是 371.4%，越过 3.70 的容差——`--check` 从 `OK` 变 `FAIL`，
**一行之差**。容差按同一张表的规则上调到 3.80，并把原因写成同一行：7 行的基线放不下端口自己的
固定开销（出处行、`#![allow (…)]` 块、`abort () -> !` 声明、两个 `include!` 与它们的说明），
而**一行对 7 行就是 14 个百分点**——这张表在小文件上量的是比值，不是密度。
读数：`python3 tools/extract-abi.py --check emacs/lib/openat-die.c lib/openat-die.rs` **OK**。
