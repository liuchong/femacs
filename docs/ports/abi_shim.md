# femacs/src/abi_shim.rs (the C-ABI entry points the macOS Swift units import; no reference file)

This file is **not** the port of a reference file: C has no functions with these names.  It is the
core side's table of honouring the names the macOS Swift units import with `@_silgen_name`
(`tools/check-swift-imports.py`; the rulings are
`docs/reviews/2026-10-04-swift-import-rulings.md`).  Each entry is the C-ABI spelling of a C
construct the Swift lane reads or writes - a `struct frame` member, a `FRAME_*` accessor macro, or
a bitfield of `frames.bits1` - so the core has to provide it as a real symbol before the Swift
units can link.

The first line is the `DECLARED` pattern `check-target-names.py` accepts for a product that is not
a port (`^// femacs: generated mirror of the frozen build's`), which is why the file is under
`femacs/src/` yet appears in no row of `docs/port-inventory.md` or `docs/port-map.md`; both were
re-run to confirm it (see `## Evidence`).

## Exports - 10 (all `#[unsafe(no_mangle)] pub unsafe extern "C" fn`, `frame` received as `*mut c_void` by the lanes' habit; the ABI is the Swift declaration's)

| name | Swift import | providing Rust item | C text (line) |
| --- | --- | --- | --- |
| `frame_left_pos` | `nsmenu.swift` 324, `nsfns.swift` 644, `nsterm.swift` 1598 (`(UnsafeMutableRawPointer?) -> Int32`) | `femacs/src/abi_shim.rs` 56 | `int left_pos, top_pos;` frame.h 604 |
| `frame_top_pos` | `nsmenu.swift` 325, `nsfns.swift` 645, `nsterm.swift` 1599 (`-> Int32`) | `abi_shim.rs` 63 | frame.h 604 |
| `frame_title` | `nsfns.swift` 253 (`-> Lisp_Object`) | `abi_shim.rs` 70 | `Lisp_Object title;` frame.h 162 |
| `frame_icon_name` | `nsfns.swift` 254 (`-> Lisp_Object`) | `abi_shim.rs` 77 | `Lisp_Object icon_name;` frame.h 158 |
| `frame_param_alist` | `nsfns.swift` 255 (`-> Lisp_Object`) | `abi_shim.rs` 84 | `Lisp_Object param_alist;` frame.h 209 |
| `frame_selected_window` | `nsfns.swift` 498-499 (`-> Lisp_Object`) | `abi_shim.rs` 91 | `#define FRAME_SELECTED_WINDOW(f) f->selected_window` frame.h 1204 |
| `frame_menu_bar_vector` | `nsmenu.swift` 318-319 (`-> Lisp_Object`) | `abi_shim.rs` 98 | `Lisp_Object menu_bar_vector;` frame.h 234 |
| `frame_tool_bar_items` | `nsmenu.swift` 322-323 (`-> Lisp_Object`) | `abi_shim.rs` 105 | `Lisp_Object tool_bar_items;` frame.h 283 |
| `frame_menu_bar_items_used` | `nsmenu.swift` 314-315 (`-> Int32`) | `abi_shim.rs` 112 | `int menu_bar_items_used;` frame.h 307 |
| `frame_n_tool_bar_items` | `nsmenu.swift` 320-321 (`-> Int32`) | `abi_shim.rs` 119 | `int n_tool_bar_items;` frame.h 573 |

- [x] `frame_left_pos`
- [x] `frame_top_pos`
- [x] `frame_title`
- [x] `frame_icon_name`
- [x] `frame_param_alist`
- [x] `frame_selected_window`
- [x] `frame_menu_bar_vector`
- [x] `frame_tool_bar_items`
- [x] `frame_menu_bar_items_used`
- [x] `frame_n_tool_bar_items`

**The remaining names of the ruling's class 1 (55) are not listed as unchecked boxes here on
purpose**: a ledger that still carries an unchecked box is skipped by `xtask`'s
`finished_units()`, so the shim would never be built by `gate-fast`/`dead-arms` and the exports it
already provides would read as missing.  The live tracking of what is left is the ratchet
(`docs/baseline/swift-import-pending.txt`) plus the rulings doc; batches land in the order that
file gives.

## Decisions

- **A unit of its own, not the unit that owns the record.**  A header mirror is `include!`d into
  every consumer, so a definition written in a mirror is one definition per unit (`E0428`); and a
  `#[unsafe(no_mangle)]` name that stands beside a C file is rejected by `extract-abi.py` as an
  `invented exported symbol` (measured on a copy of `frame.rs` with one export appended: `FAIL …
  fn frame_left_pos carries #[unsafe(no_mangle)] but the reference does not define a non-static
  function of that name`).  A unit with no C counterpart is outside every `--check` pairing
  (`--check emacs/src/abi_shim.c …` answers `FileNotFoundError`), and `xtask` is the precedent for
  a product with no C source (status 3.241).
- **`build-tu` reaches it without a C file** through the `UNIT_DIRS` stem fallback:
  `build-tu src/abi_shim.c` builds `src/abi_shim.rs`, and the object lands in
  `build/femacs/abi_shim.o`, which is the tree `check-swift-imports.py` reads with `nm -g`.
- **The gate's wording is misleading, the signal is not**: once a name is provided, the tool
  prints `STALE <name>: nothing imports it any more` for its ratchet row (the row satisfied and the
  row orphaned land in the same set, `set(rows) - set(missing)`).  In this batch the ten names are
  still imported (three of them from three files); the rows simply have to go.  The integrator
  deletes them.
- **ABI = the Swift declaration.**  Where the lane's declaration and a C prototype differ, the
  Swift side wins: `UnsafeMutableRawPointer?` is received as `*mut c_void` (the lane's habit, and
  the same pointer C passes), `Int32` as `c_int`, `Bool` as `bool`, `UInt` as `c_ulong`, and the
  lanes' `Lisp_Object` typealias (`UnsafeMutableRawPointer?`, `nsfns.swift` 26) is the port's
  `Lisp_Object` (`Lisp_Word`, 8 bytes).

## Evidence

- **`build-tu`**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-abi cargo run -q -p xtask -- build-tu
  src/abi_shim.c` -> `built src/abi_shim.rs -> ../build/femacs/abi_shim.o`.
- **`nm -g` (the ten symbols, `nm` spelling against the Swift names)**: `T _frame_left_pos`,
  `T _frame_top_pos`, `T _frame_title`, `T _frame_icon_name`, `T _frame_param_alist`,
  `T _frame_selected_window`, `T _frame_menu_bar_vector`, `T _frame_tool_bar_items`,
  `T _frame_menu_bar_items_used`, `T _frame_n_tool_bar_items` - each with its `@_silgen_name` line
  in the table above.
- **`check-swift-imports.py`**: these ten names left the `unprovided` set, so the tool now lists
  `STALE frame_left_pos` … `STALE frame_n_tool_bar_items` (ten of the eleven `STALE` lines; the
  eleventh, `visible_bell`, is another lane's).  The round's tail line was
  `710 import(s) in 7 Swift file(s), 5566 core export(s), 554 unprovided, 535 ratchet row(s), 41
  failure(s)` - the Swift lanes are adding imports at the same time, so the only rows this batch
  owns are the ten `STALE` lines just named.
- **`check-target-names.py`**: `627 .rs files, 591 map targets, 0 finding(s)` - the `DECLARED`
  first line is what keeps `abi_shim.rs` out of rule R1.
- **`gen-port-inventory.py` / `gen-port-map.py`**: re-run with the file in the tree ->
  `wrote …: 590 files, done=583 wip=7 todo=0` and the map's six mode tables unchanged; neither
  grows a row for the shim (they iterate the reference tree's C sources).
- **`check-symbols.py` / `check-undef.py`**: `86 units compared, 1 mismatched` (the mismatch is
  `lib/regex`, another lane) - the shim is not compared, having no reference object; `696 unit
  object(s) …, 0 new` for the undefined-symbol gate.
