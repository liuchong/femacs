# emacs/src/insdel.c (2592 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 1
- [x] combine-after-change-execute  (Fcombine_after_change_execute, 0/0, doc 07bf78694564, line 2475)

## Functions defined — 51
- [x] move_gap_both  (line 94)
- [x] gap_left  (line 110, static)
- [x] gap_right  (line 173, static)
- [x] adjust_suspend_auto_hscroll  (line 226, static)
- [x] adjust_markers_for_delete  (line 249)
- [x] adjust_markers_for_insert  (line 288, static)
- [x] adjust_point  (line 329, static)
- [x] adjust_markers_for_replace  (line 342, static)
- [x] count_bytes  (line 388, static)
- [x] adjust_markers_bytepos  (line 413)
- [x] buffer_overflow  (line 459)
- [x] make_gap_larger  (line 467, static)
- [x] make_gap  (line 578)
- [x] make_gap_1  (line 607)
- [x] copy_text  (line 624)
- [x] insert  (line 681)
- [x] insert_and_inherit  (line 696)
- [x] insert_char  (line 711)
- [x] insert_string  (line 730)
- [x] insert_before_markers  (line 741)
- [x] insert_before_markers_and_inherit  (line 756)
- [x] insert_1_both  (line 891)
- [x] insert_from_string  (line 981)
- [x] insert_from_string_before_markers  (line 999)
- [x] insert_from_string_1  (line 1018, static)
- [x] insert_from_gap_1  (line 1123)
- [x] insert_from_gap  (line 1167)
- [x] insert_from_buffer  (line 1211)
- [x] insert_from_buffer_1  (line 1241, static)
- [x] adjust_after_replace  (line 1376, static)
- [x] adjust_after_insert  (line 1432)
- [x] replace_range  (line 1463)
- [x] replace_range_2  (line 1717)
- [x] del_range  (line 1823)
- [x] safe_del_range_1  (line 1835, static)
- [x] safe_del_range_2  (line 1845, static)
- [x] safe_del_range  (line 1854)
- [x] del_range_1  (line 1870)
- [x] del_range_byte  (line 1903)
- [x] del_range_both  (line 1945)
- [x] del_range_2  (line 1991)
- [x] modify_text  (line 2084)
- [x] run_undoable_change  (line 2101, static)
- [x] prepare_to_modify_buffer_1  (line 2125)
- [x] prepare_to_modify_buffer  (line 2196)
- [x] invalidate_buffer_caches  (line 2206)
- [x] reset_var_on_error  (line 2296, static)
- [x] signal_before_change  (line 2310, static)
- [x] signal_after_change  (line 2381)
- [x] Fcombine_after_change_execute_1  (line 2470, static)
- [x] syms_of_insdel  (line 2561)

## Macros defined — 6
- [x] FETCH_END  (line 2280, body 208da51fa3b4)
- [x] FETCH_START  (line 2277, body c8c225770937)
- [x] PRESERVE_START_END  (line 2271, body 1951f82b2e1e)
- [x] PRESERVE_VALUE  (line 2260, body f8fc64dbb6f1)
- [x] RESTORE_VALUE  (line 2264, body 3a07341a8990)
- [x] check_markers  (line 86, body cce2980cc6ca)

## Types and their fields — 2
- [x] rvoe_arg: Lisp_Object* location, bool errorp
- [x] safe_del_range_context: ptrdiff_t from, ptrdiff_t to

## File-scope objects — 2
- [x] static Lisp_Object combine_after_change_list  (line 54, static)
- [x] static Lisp_Object combine_after_change_buffer  (line 57, static)

# ----------------------------------------------------------------------
## Decisions

`emacs/src/insdel.c` @ `fd7afced403`; port `src/insdel.rs` (2,495 lines vs C's
2,280 live of 2,592 — the 9.0% drift against the **`live`** baseline is declared in
`docs/ports/line-drift-tolerance.tsv`).  `src/insdel.c` is in
`docs/baseline/frozen-build-sources.txt`, so the frozen configuration compiles it: there is
no 3.31 exemption and the full comparison applies.  The gates:

    build-tu src/insdel.c        -> built src/insdel.rs -> ../build/femacs/insdel.o
    extract-abi.py --check       -> OK emacs/src/insdel.c -> src/insdel.rs
    check-symbols.py --pending   -> check-symbols: 78 units compared, 0 mismatched, 1 still pending
    check-globals.py             -> check-globals: 242 units, 0 DEFVAR_* variables without a definition in their own unit
    ledger                       -> 62/62 rows checked, 0 unchecked

An extra compile with `--cfg enable_checking` (the repo's step 6) also succeeds
(`rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort -C codegen-units=1
-C opt-level=0 --cfg enable_checking …/insdel/_root.rs`), as P-57's class of defect
requires; the only diagnostic it prints is the pre-existing `unused doc comment` warning in
`include/lisp.rs`, not this unit.

### The condition table: the file is verified, so the dead arms are recorded, not carried

`python3 tools/check-cond-eval.py src/insdel.c` reports
`24 conditions, 0 unknown, 0 disagreements [VERIFIED]`, and `src/insdel.c` is in
`docs/baseline/config/cond-eval-verified.txt`, so P-08 applies: the port transcribes the
text the frozen configuration compiles and **records** the dead region here rather than
keeping it as `#[cfg (feature = …)]` text (the `bytecode.c`/`thread.c` treatment, not the
`callproc.c`/`timefns.c` one).  The dead lines, in C order:

* C 34-35: `#ifdef HAVE_TREE_SITTER` / `# include "treesit.h"` — `HAVE_TREE_SITTER` is
  `undef` in the frozen `config.h` (macros.tsv 1625), so no `ts_linecol` site and no
  `treesit_linecol_maybe`/`treesit_record_change` call is compiled.  The twelve in-function
  blocks are C 901-904 (`insert_1_both`), 958-963 (same function), 1035-1038
  (`insert_from_string_1`), 1102-1107 (same), 1128-1132 and 1150-1155
  (`insert_from_gap_1`), 1216-1220 and 1227-1233 (`insert_from_buffer`), 1524-1530
  (`replace_range`), 1669-1675 (same), 2002-2008 (`del_range_2`) and 2067-2071 (same).
* C 63-83: the `#ifdef MARKER_DEBUG` arm of `check_markers` (whose body walks
  `BUF_MARKERS` and calls `emacs_abort`); `MARKER_DEBUG` is not in the frozen table, so
  C 86's `#define check_markers() do { } while (0)` is the live text — the port keeps that
  macro (the one macro body the C text has no token for is the do-while's `0`;
  `docs/ports/macro-body-notes.tsv` records it).
* C 514-574 and the `#else` arm at C 597-599: `make_gap_smaller`, under
  `#if defined USE_MMAP_FOR_BUFFERS || defined REL_ALLOC || defined DOUG_LEA_MALLOC`; all
  three are `undef` (macros.tsv 3790/3261/762) and `src/Makefile.in` builds this file for
  the minimal configuration, so `make_gap`'s `else` is not compiled either.
* C 769-877 and the eight `#ifdef BYTE_COMBINING_DEBUG` blocks: `count_combining_before`
  and `count_combining_after`, plus every call of them — C 918-921 (`insert_1_both`),
  1057-1065 (`insert_from_string_1`), 1317-1325 (`insert_from_buffer_1`), 1381-1384
  (`adjust_after_replace`), 1621-1629 (`replace_range`), 1760-1768 (`replace_range_2`) and
  2017-2020 (`del_range_2`).  `BYTE_COMBINING_DEBUG` is not in the frozen table.

Three further C constructs have no Rust counterpart and are recorded as deviations below:
C 41-44's and C 59's forward declarations (which C needs only because C is
order-sensitive) and one include whose header has no mirror (C 23's `<intprops.h>`; C 30's
`"region-cache.h"` has been `include/region-cache.rs` since status 3.74).

### The five statement/expression macros: helper functions, not `macro_rules!`

C 2260-2282's `PRESERVE_VALUE`, `RESTORE_VALUE`, `PRESERVE_START_END`, `FETCH_START` and
`FETCH_END` are written on locals of `signal_before_change` (`preserve_ptr`,
`preserve_marker`, `start_marker`, `end_marker`, `start`, `end`) and on `preserve_ptr` of
`signal_after_change`, and a `macro_rules!` cannot name a caller's locals (PORTING.md 3.8,
found on the marker.c `CONSIDER` pair: the hygiene error is `cannot find value … not
accessible due to macro hygiene`).  Each macro is therefore an `unsafe fn` of the same
name whose parameters are exactly the names the C macro captures
(`PRESERVE_VALUE (preserve_ptr: *mut ptrdiff_t, preserve_marker: *mut Lisp_Object)`, and so
on), so that (a) every call site keeps C's call shape — `PRESERVE_VALUE (preserve_ptr,
&raw mut preserve_marker);` where C writes `PRESERVE_VALUE;` — and (b) the function body is
the same token multiset as the C macro body, which `extract-abi.py` compares through
`macro_body_of` (the function that answers for the macro name).  The `&mut`-style
parameters are the marker.c `&mut` cursor shape.  `check_markers` (C 86) needs no such
treatment because C itself defines it as an empty statement macro: the port keeps a
`macro_rules!` with C's empty block body, defined before its first use (P-57).

### Statement shapes Rust cannot write in one line

* `x++`/`x--` and the comma operators: `from -= i, to -= i;` is two `offset` assignments
  (C 154), `from += i, to += i;` likewise (C 212), `bufp++, pos_byte++;` likewise (C 872,
  dead), `*to_addr++ = c;` is `*to_addr = …; to_addr = to_addr.offset (1);` (C 642, 666),
  and `nbytes--`/`nchars++` are `-= 1`/`+= 1`.
* The `for` loops whose update step has a side effect are `while` loops with the update at
  the end of the body in C's position (`count_bytes`' two `for ( ; pos < endpos; pos++)`,
  the four `for (m = BUF_MARKERS (current_buffer); m; m = m->next)` marker walks
  — written as `m = BUF_MARKERS! (current_buffer);` plus `while !m.is_null () { … m =
  (*m).next; }`, the `marker.c` shape — and `Fcombine_after_change_execute`'s
  `for (tail = …; CONSP (tail); tail = XCDR (tail))`, where C's two `continue`s jump to
  the update step and the port therefore advances `tail` before each `continue`).
* `while (1)` is `while true`; `docs/ports/macro-body-notes.tsv` names the constant the two
  `gap_left`/`gap_right` loops cannot spell (the `itree_propagate_limit` precedent).
* C 1309's `chunk_expanded = chunk = 0;` is two assignments in C's order
  (`chunk = 0; chunk_expanded = 0;`), and C 2401's `CONSP (tmp = XCDR
  (Vbefore_change_functions))` — an assignment inside `signal_after_change`'s condition —
  is the same assignment in a block expression, at the same point of the `&&` chain.
* C 1888's `min (ZV, from + range_length)`, C 1682-1683's and C 2041-2042's
  `min (PT, to)`/`min (PT_BYTE, to_byte)` are the mirror's `min (isize, isize)`; C 596's `max` likewise.
* C 2241's `(line_beg > BUF_BEG (buf))` (a `bool` added to a `ptrdiff_t`) is
  `(line_beg > BUF_BEG (buf)) as ptrdiff_t`.
* C 1795-1796's `adjust_markers_bytepos (from, from_byte, from + inschars,
  from_byte + insbytes, true)` passes C's `true` to the prototype's `int to_z`
  (`lisp.h` 4361), so the port keeps the mirror-less parameter's `c_int` type and spells the
  argument `true as c_int`; the body then tests it as `to_z == 0` / `to_z != 0`, and the
  token `true` survives exactly as C writes it.  (The only other caller is `editfns.c` 4808,
  which passes C's `0` and which does not use this unit's declaration.)

### Uninitialised locals (PORTING.md 1.5) and the pointer casts the mirrors need

Zero-initialised because C declares them and assigns them later (or in a branch):
`to`/`from`/`i`/`new_s1` (`gap_left`), `to`/`from`/`i`/`new_s1` (`gap_right`),
`real_gap_loc`/`real_gap_loc_byte`/`old_gap_size`/`tem` (`make_gap_larger`),
`thislen`/`c` (`copy_text`), `len` (`insert_char`), `intervals` (`insert_from_string_1`,
`insert_from_buffer_1`), `chunk`/`chunk_expanded` (`insert_from_buffer_1`),
`nchars_del`/`nbytes_del` (`adjust_after_replace`, `replace_range_2`, `del_range_2`),
`from_byte`/`to_byte`/`nbytes_del`/`nchars_del`/`deletion` (`replace_range`), `inschars`/
`insbeg`/`insbuf`/`insbeg_bytes`/`insend_bytes`/`insbytes`/`insbeg_ptr`/
`new_is_multibyte` (`replace_range`), `context` (`safe_del_range_1`), `base_buffer`/
`temp` (`prepare_to_modify_buffer_1`), `start`/`end`/`start_marker`/`end_marker`/
`preserve_marker`/`rvoe_arg` (`signal_before_change`), `rvoe_arg`/`tmp`/
`save_insert_behind_hooks`/`save_insert_in_from_hooks` (`signal_after_change`),
`beg`/`end`/`change`/`begpos`/`endpos`/`tail` (`Fcombine_after_change_execute_1`'s subr)
and `str_` (`insert_char`, C's `unsigned char str[MAX_MULTIBYTE_LENGTH]`, of which only
`len` bytes are read).  C's `struct rvoe_arg rvoe_arg;` and `struct
safe_del_range_context context;` take C's own zero-value shape
(`rvoe_arg { location: std::ptr::null_mut (), errorp: false }`,
`safe_del_range_context { from: 0, to: 0 }`) and keep C's field assignments
(PORTING.md 0.1-13).

Casts the mirrors' parameter types ask for: `unchain_marker` takes `*mut c_void` in the
`lisp.h` mirror (`void unchain_marker (struct Lisp_Marker *)`), so the two call sites pass
`XMARKER (…) as *mut c_void` (the marker.rs shape); `buf_charpos_to_bytepos` is
`*mut c_void`-typed in the mirror, so its four call sites cast; `pdumper_object_p` takes
`*const c_void`; `memcpy`/`memmove` take `*mut/*const c_void`; and `set_text_properties`,
`graft_intervals_into_buffer` and `copy_intervals` are reached through the `intervals.h`
mirror unchanged.  `copy_text`'s byte count from the multibyte→unibyte branch is
`to_addr.offset_from (initial_to_addr)`, the same difference C computes.

### The bitfields this unit reads and writes

Two packed fields are touched, each with a token family in
`docs/ports/token-aliases.tsv` (the `local_if_set`/`red` precedent, PORTING.md 1.3):

* C 236's `w->suspend_auto_hscroll = 0` is
  `(*w).bits &= ! (1 << WINDOW_SUSPEND_AUTO_HSCROLL_SHIFT)` (window.rs 158; `struct
  window`'s nineteen `bool_bf` members live in the mirror's `bits` word).  Family
  `suspend_auto_hscroll`.
* C 2053's `!current_buffer->text->inhibit_shrinking` is
  `(*(*current_buffer).text).bits & BUFFER_TEXT_INHIBIT_SHRINKING == 0` (buffer.rs 392).
  Family `inhibit_shrinking`.

`MARKER_INSERTION_TYPE_P (m)` (C 303's `m->insertion_type`) uses the pre-existing
`insertion_type` family, and `CHAR_HEAD_P`/`LEADING_CODE_P`/`BYTES_BY_CHAR_HEAD` come from
the `character.h` mirror.

### Use-site declarations (none of them has a mirror)

Declared at the use site with C's own prototype in a `/// C:` line (the terminal.rs shape):
`chars_in_text` (lisp.h 4226), `clear_charpos_cache` (lisp.h 4954), `detach_marker`
(lisp.h 4957), `internal_catch_all` (lisp.h 4811), `update_search_regs` (lisp.h 4996-4997),
`find_newline_no_quit` (lisp.h 5040-5041), `make_buffer_string_both` (lisp.h 4927-4928),
`adjust_overlays_for_insert` (lisp.h 4939), `adjust_overlays_for_delete` (lisp.h 4940),
`report_overlay_modification` (lisp.h 4942-4943), `report_interval_modification`
(lisp.h 5261), `record_first_change` (lisp.h 5253 — defined in `undo.rs` with the same
name, but no mirror declares it), `bset_redisplay` (window.h 1169) and
`invalidate_region_cache` (region-cache.h 88-90, whose declaration is the mirror's now,
status 3.74).  `struct region_cache` stays opaque to every consumer, so
its three pointers in `invalidate_buffer_caches` are `*mut c_void` (P-72 rule 3) exactly as
`buffer.rs` already spells the three cache members.  Anything the mirrors do carry
(`BUF_MARKERS`, `GPT_ADDR`, …) is used as the mirror spells it, and the `F*` subrs come
from the generated `globals.rs` prototypes.  Two of the fourteen are **mirror gaps for the
integrator**: `bset_redisplay` is declared in `window.h` (whose mirror exists) and
`record_first_change` in `lisp.h` (whose mirror exists); neither mirror carries its
declaration, and this unit did not edit them (they are other units' files).

### C's own dead code and `#if` bookkeeping

Every `#if` in the file is one of the four platform/compiler predicates listed above; there
is no `#if 0` and no configuration-branch `#define`, so `extract-abi.py`'s macro-body
comparison covers all six macros.  `intprops.h` (C 23) is included but no name from it is
used; `region-cache.h` (C 30) contributes the one prototype above.

### `DEFVAR_*`/`DEFSYM!` storage and names

`syms_of_insdel` registers the two variables this file owns, and per status §3.29 the unit
defines them (`Vcombine_after_change_calls`, `inhibit_modification_hooks` as
`#[unsafe(no_mangle)] pub static mut`, initialised `Qnil`/`false`), because
`include/globals.rs` only declares the `globals.f_<vname>` storage.  `check-globals.py` is
the gate that sees this.  The `DEFSYM!`/`DEFVAR_*!` name arguments are `c"…"` literals per
status §3.38 (the three `DEFSYM`s and the two `DEFVAR_*`s), and the `DEFVAR_BOOL` docstring
is C's positionally (its line breaks become `\n` in the literal).

### Macro-body notes, token families and the line budget

`docs/ports/macro-body-notes.tsv` gains seventeen rows for this unit: `check_markers`
(the do-while `0`), `move_gap_both`/`make_gap_larger` (`0`/`1` passed to `gap_left`'s
`bool newgap`), `gap_left`/`gap_right` (`while (1)`), `adjust_suspend_auto_hscroll` (the
`= 0` write of the packed field), `insert`, `insert_and_inherit`, `insert_before_markers`,
`insert_before_markers_and_inherit`, `insert_from_string_before_markers` and
`insert_from_gap` (C's int literals passed to the `bool` parameters of `insert_1_both`,
`insert_from_string_1` and `graft_intervals_into_buffer`), `del_range` and `del_range_2`
(`del_range_1`'s `bool prepare`/`ret_string` and `make_buffer_string_both`'s `bool props`),
`signal_before_change` and `signal_after_change` (`rvoe_arg.errorp = 1`/`= 0` on the port's
`bool` field, and the `bool after` of `report_overlay_modification`), and `syms_of_insdel`
(`inhibit_modification_hooks = 0`, plus the positional docstrings' `doc:` label — the
`syms_of_data` precedent).  `docs/ports/token-aliases.tsv` gains the
`suspend_auto_hscroll` and `inhibit_shrinking` families (above).
`docs/ports/line-drift-tolerance.tsv` gains the `insdel` row (0.12, `live`).
`docs/ports/excluded-macros.txt` needed no row: no C macro of this file is dropped.

### Changes outside the unit (for integrator review)

1. `docs/ports/macro-body-notes.tsv`: the seventeen rows above (appended).
2. `docs/ports/token-aliases.tsv`: the two families above (appended).
3. `docs/ports/line-drift-tolerance.tsv`: the `insdel` row (appended).
4. Nothing else: no mirror, no `xtask`, no baseline file and no tool was edited.  The two
   mirror gaps found (`bset_redisplay` in `window.h`, `record_first_change` in `lisp.h`,
   both declared at the use site instead) are recorded above rather than fixed here.
