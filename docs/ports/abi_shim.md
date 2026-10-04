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
| `frame_set_border_width` | `nsfns.swift` 273-274 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 127 | `int border_width;` frame.h 613 |
| `frame_set_child_frame_border_width` | `nsfns.swift` 262-263 (`(…, Int32)`) | `abi_shim.rs` 134 | `int child_frame_border_width;` frame.h 617 |
| `frame_set_internal_border_width` | `nsfns.swift` 264-265 (`(…, Int32)`) | `abi_shim.rs` 141 | `int internal_border_width;` frame.h 622 |
| `frame_set_menu_bar_lines` | `nsfns.swift` 486-487 (`(…, Int32)`) | `abi_shim.rs` 148 | `int menu_bar_lines;` frame.h 635 |
| `frame_set_tab_bar_lines` | `nsfns.swift` 492-493 (`(…, Int32)`) | `abi_shim.rs` 155 | `int tab_bar_lines;` frame.h 558 |
| `frame_set_tab_bar_height` | `nsfns.swift` 490-491 (`(…, Int32)`) | `abi_shim.rs` 162 | `int tab_bar_height;` frame.h 561 |
| `frame_set_background_pixel` | `nsfns.swift` 269-270 (`(…, UInt)`) | `abi_shim.rs` 169 | `unsigned long background_pixel;` frame.h 732 |
| `frame_set_foreground_pixel` | `nsfns.swift` 267-268 (`(…, UInt)`) | `abi_shim.rs` 176 | `unsigned long foreground_pixel;` frame.h 733 |
| `frame_explicit_name` | `nsfns.swift` 249 (`-> Bool`) | `abi_shim.rs` 194 | `bool_bf explicit_name : 1` frame.h 417 (`FRAME_EXPLICIT_NAME_SHIFT` 15) |
| `frame_set_explicit_name` | `nsfns.swift` 250-251 (`(…, Bool)`) | `abi_shim.rs` 201 | frame.h 417 |
| `frame_set_wants_modeline` | `nsfns.swift` 271-272 (`(…, **Bool**)`) | `abi_shim.rs` 211 | `bool_bf wants_modeline : 1` frame.h 402 (`FRAME_WANTS_MODELINE_SHIFT` 11) |
| `frame_set_output_method` | `nsfns.swift` 266 (`(…, Int32)`) | `abi_shim.rs` 222 | `ENUM_BF (output_method) output_method : 4;` frame.h 446 (shift 21, `FRAME_OUTPUT_METHOD_MASK` 0x0F) |
| `frame_tab_bar_redisplayed` | `nsfns.swift` 260-261 (`-> Bool`) | `abi_shim.rs` 233 | `bool_bf tab_bar_redisplayed : 1` frame.h 503 (shift 30) |
| `frame_tab_bar_resized` | `nsfns.swift` 257 (`-> Bool`) | `abi_shim.rs` 240 | `bool_bf tab_bar_resized : 1` frame.h 504 (shift 31) |
| `frame_set_tab_bar_resized` | `nsfns.swift` 258-259 (`(…, Bool)`) | `abi_shim.rs` 247 | frame.h 504 |
| `frame_tab_bar_window` | `nsfns.swift` 256 (`-> Lisp_Object`) | `abi_shim.rs` 263 | `Lisp_Object tab_bar_window;` frame.h 254 (**inside `#ifdef HAVE_WINDOW_SYSTEM`**) - the frozen answer is `Qnil` |
| `frame_set_external_menu_bar` | `nsfns.swift` 487-488 (`(…, Int32)`) | `abi_shim.rs` 275 | `#define FRAME_EXTERNAL_MENU_BAR(f) false` frame.h 1082 (the `#else` arm) - no-op |
| `frame_set_external_tool_bar` | `nsfns.swift` 493-494 (`(…, Int32)`) | `abi_shim.rs` 283 | `#define FRAME_EXTERNAL_TOOL_BAR(f) false` frame.h 1097 (the `#else` arm) - no-op |

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
- [x] `frame_set_border_width`
- [x] `frame_set_child_frame_border_width`
- [x] `frame_set_internal_border_width`
- [x] `frame_set_menu_bar_lines`
- [x] `frame_set_tab_bar_lines`
- [x] `frame_set_tab_bar_height`
- [x] `frame_set_background_pixel`
- [x] `frame_set_foreground_pixel`
- [x] `frame_explicit_name`
- [x] `frame_set_explicit_name`
- [x] `frame_set_wants_modeline`
- [x] `frame_set_output_method`
- [x] `frame_tab_bar_redisplayed`
- [x] `frame_tab_bar_resized`
- [x] `frame_set_tab_bar_resized`
- [x] `frame_tab_bar_window`
- [x] `frame_set_external_menu_bar`
- [x] `frame_set_external_tool_bar`

**The remaining names of the ruling's class 1 (37) are not listed as unchecked boxes here on
purpose**: a ledger that still carries an unchecked box is skipped by `xtask`'s
`finished_units()`, so the shim would never be built by `gate-fast`/`dead-arms` and the exports it
already provides would read as missing.  The live tracking of what is left is the ratchet
(`docs/baseline/swift-import-pending.txt`) plus the rulings doc; batches land in the order that
file gives.

## Decisions
- **One name of the batch list is held back: `frame_set_double_buffered`.**  Its C construct is
  `#define FRAME_DOUBLE_BUFFERED(f) ((f)->output_data.ns->double_buffered)` (nsterm.h 1052) inside
  `#if defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MIN_REQUIRED >= 101400`, and the frozen
  configuration has `HAVE_NS`, `NS_IMPL_COCOA` and `NS_IMPL_GNUSTEP` all **undef**
  (`docs/baseline/config/darwin-arm64.macros.tsv` 1481/2996/2997; the reference's own
  `build/emacs-ref/src/config.h` 1205/2181 agree), so C never sees that member and this mirror
  gates it `#[cfg (feature = "ns_impl_cocoa")]` (`include/nsterm.rs` 487-488).  A write body can
  therefore only exist *inside* that arm; exporting it ungated would invent a host for a member the
  frozen record does not have (P-08/P-51), and gating the export on `ns_impl_cocoa` would leave the
  ratchet row unpayable in a normal build.  Ruling requested; the other twenty-seven names of the
  batches are landed.
- **Two names of the round's batch list have no importer and were not landed**: the Swift tree has
  no `@_silgen_name("frame_set_left_pos")` / `@_silgen_name("frame_set_top_pos")` anywhere
  (`rg` over `femacs/platform/macos/*.swift` finds only the two *readers*, `frame_left_pos` /
  `frame_top_pos`) and `docs/baseline/swift-import-pending.txt` has no row for either, so exporting
  them would add C-ABI surface nobody imports.  They wait for a real import (or a ruling).

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
- **Batches 3 and 4 (the `frames.bits1` accessors and the three names status 3.242 fixed)**:
  `build-tu src/abi_shim.c` -> `built src/abi_shim.rs -> ../build/femacs/abi_shim.o`;
  `nm -g build/femacs/abi_shim.o` answers `T _frame_explicit_name`, `T _frame_set_explicit_name`,
  `T _frame_set_wants_modeline`, `T _frame_set_output_method`, `T _frame_tab_bar_redisplayed`,
  `T _frame_tab_bar_resized`, `T _frame_set_tab_bar_resized`, `T _frame_tab_bar_window`,
  `T _frame_set_external_menu_bar`, `T _frame_set_external_tool_bar` (28 `_frame_*` in all);
  `check-swift-imports.py` then prints `STALE (satisfied) <name>: provided now, so the row goes` for
  each of them (the ten new rows; the eight from batch 2 are still in the file, so that run lists
  eighteen of this shim's rows - the other two `STALE (satisfied)` lines, `ns_free_frame_resources`
  and `ns_set_offset`, are the nsterm lane's).
- **No reference symbol was invented** (status 3.251's rule): `nm -g build/emacs-ref/src/*.o`
  contains none of the shim's names (they are the Swift lanes' C-ABI spellings of C macros and
  members, which is what the ruling says), and `check-undef.py` reports `0` new undefined
  references from `abi_shim.o` (its current failure list is nineteen references out of `xdisp.o`,
  another lane's unit).
- **Batch 2 (the eight `frame_set_*` writers)**: `build-tu src/abi_shim.c` -> `built
  src/abi_shim.rs -> ../build/femacs/abi_shim.o`; `nm -g build/femacs/abi_shim.o` answers
  `T _frame_set_background_pixel`, `T _frame_set_border_width`,
  `T _frame_set_child_frame_border_width`, `T _frame_set_foreground_pixel`,
  `T _frame_set_internal_border_width`, `T _frame_set_menu_bar_lines`,
  `T _frame_set_tab_bar_height`, `T _frame_set_tab_bar_lines` (18 `_frame_*` in all), each against
  its `@_silgen_name` line in the table above; `check-swift-imports.py` then prints
  `STALE (satisfied) frame_set_background_pixel: provided now, so the row goes` and the same line
  for the other seven (its tail: `761 import(s) in 7 Swift file(s), 5599 core export(s), 573
  unprovided, 557 ratchet row(s), 36 failure(s)` - the Swift lanes are adding imports in the same
  window, so the eight `STALE (satisfied)` lines are this batch's whole signal).

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

- **Ruling (3.255, integrator): `frame_set_double_buffered` is exported under the same gate the
  member lives under.**  `#[cfg (feature = "ns_impl_cocoa")]` on the export, and the ratchet row
  stays with the reason "only a Cocoa build can clear this row" - the `macfont`/`nsfont` permanent
  dead-arm shape.  Exporting it ungated would invent a host for a member the frozen record does not
  have (P-08); deferring it to the NS round would push an already-measured row into the future.
  The general ruling for macro-name entries in this file is in `docs/status.md` 3.255 (B first:
  expand in Swift when the body is expressible there; A only when the expansion needs core-internal
  facts, which is when this shim exports the same-name C-ABI entry).
