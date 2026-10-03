# emacs/src/intervals.c (2416 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 53
- [x] set_interval_left  (line 63, static)
- [x] set_interval_right  (line 69, static)
- [x] copy_interval_parent  (line 78, static)
- [x] create_root_interval  (line 87)
- [x] copy_properties  (line 118)
- [x] merge_properties  (line 132, static)
- [x] intervals_equal_1  (line 170, static)
- [x] intervals_equal  (line 224)
- [x] traverse_intervals_noorder  (line 235)
- [x] traverse_intervals  (line 256)
- [x] rotate_right  (line 279, static)
- [x] rotate_left  (line 330, static)
- [x] balance_an_interval  (line 375, static)
- [x] balance_possible_root_interval  (line 415, static)
- [x] balance_intervals_internal  (line 445, static)
- [x] balance_intervals  (line 458)
- [x] buffer_balance_intervals  (line 466, static)
- [x] split_interval_right  (line 490)
- [x] split_interval_left  (line 534)
- [x] interval_start_pos  (line 573, static)
- [x] find_interval  (line 599)
- [x] next_interval  (line 652)
- [x] previous_interval  (line 691)
- [x] update_interval  (line 740)
- [x] adjust_intervals_for_insertion  (line 802, static)
- [x] merge_properties_sticky  (line 1024, static)
- [x] delete_node  (line 1159, static)
- [x] delete_interval  (line 1193, static)
- [x] interval_deletion_adjustment  (line 1246, static)
- [x] adjust_intervals_for_deletion  (line 1307, static)
- [x] offset_intervals  (line 1358)
- [x] merge_interval_right  (line 1380, static)
- [x] merge_interval_left  (line 1436)
- [x] reproduce_interval  (line 1487, static)
- [x] reproduce_tree  (line 1513, static)
- [x] reproduce_tree_obj  (line 1521, static)
- [x] graft_intervals_into_buffer  (line 1566)
- [x] textget  (line 1706)
- [x] lookup_char_property  (line 1712)
- [x] temp_set_point_both  (line 1751)
- [x] temp_set_point  (line 1766)
- [x] set_point  (line 1776)
- [x] set_point_from_marker  (line 1784)
- [x] adjust_for_invis_intang  (line 1810, static)
- [x] set_point_both  (line 1849)
- [x] move_if_not_intangible  (line 2072)
- [x] get_property_and_range  (line 2135)
- [x] get_local_map  (line 2176)
- [x] copy_intervals  (line 2227)
- [x] copy_intervals_to_string  (line 2267)
- [x] compare_string_intervals  (line 2283)
- [x] set_intervals_multibyte_1  (line 2321, static)
- [x] set_intervals_multibyte  (line 2410)

## Macros defined — 2
- [x] SET_PARENT_POSITION  (line 725, body aebf6b01dcc1)
- [x] TMEM  (line 52, body e3dee28f08fb)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **What the file carries**: C's 53 functions (29 exported, 24 `static`), its two macros
  (`TMEM`, `SET_PARENT_POSITION`), no subr, no file-scope object.  The unit is in
  `docs/baseline/config/cond-eval-verified.txt`, so the branch selection is clang's and the
  one `#if` is the literal `#if 0` at C 1643-1648: **recorded here, not transcribed**
  (P-08), which is why `docs/ports/line-drift-tolerance.tsv` declares the `live` basis.
- **`intervals.h`'s mirror landed with this unit** (`femacs/src/include/intervals.rs`, the
  composite.rs/ccl.rs precedent).  This unit defines 24 of the header's prototypes, so that
  block of the header moved into an `intervals_decls` module + glob import (PORTING.md
  0.1-10); the three textprop.c defines (`verify_interval_modification`,
  `validate_interval_range`, `interval_of`) and alloc.c's `make_interval` stay where C
  declares them.  The mirror's six `bool_bf : 1` members are one `bits: u8` byte with the
  `INTERVAL_*_BIT` masks (PORTING.md 1.3/P-80); `docs/ports/token-aliases.tsv` gained the
  `up_obj`/`write_protect`/`visible`/`front_sticky`/`rear_sticky` families (with the
  occurrence check) so every read and write of them stays alive.  The mirror's ledger is
  `docs/ports/include-intervals.md`.
- **Two shapes in the mirror could not be copied literally.**  ① `LEAF_INTERVAL_P` and
  `BOTH_KIDS_P`: C writes the two field comparisons out (`(i)->left == NULL && …`), so the
  mirror spells them out too (`NULL_LEFT_CHILD!`/`NULL_RIGHT_CHILD!` would have hidden the
  tokens).  ② `TEXT_PROP_MEANS_INVISIBLE` was typed as `bool` in one arm and `c_int` in the
  other (it had never been expanded by a unit); C's value is an `int` -- xdisp.c compares it
  with `2` -- so both arms are `c_int` now and a boolean use site writes `!= 0` (this
  unit's only use).  `INTERVAL_PARENT_OR_NULL` and `RESET_INTERVAL` gained the `as INTERVAL`
  casts their own setters need (the mirror's uniform `((i) as *mut interval)` shape), and
  `interval_up` is `#[derive(Clone, Copy)]` because `copy_interval_parent` assigns `d->up =
  s->up` (PORTING.md 0.1-9).
- **`buffer.rs`'s mirror needed one integrator edit** (reported, not silently broadened):
  buffer.h declares `set_point`/`temp_set_point`/`set_point_both`/`temp_set_point_both`/
  `set_point_from_marker` (and buffer.c's `enlarge_buffer_text`) at file scope, and their
  own comment says the parameter names come from **intervals.c**'s definitions.  A
  root-scope declaration plus this unit's definition is one `E0428`, so that six-prototype
  block moved into a `buffer_point_decls` module + glob import (PORTING.md 0.1-10, the
  composite.rs/`window_redisplay_decls` shape, the P-70 "the change lands in the same commit
  as the definition" rule).  No other mirror declares those names (checked with
  `rg 'fn set_point|fn temp_set_point|…' femacs`).
- **`INTERVAL` is the mirror set's `*mut c_void`** (P-72), so where C writes `i->field` the
  port casts once, `(*((i) as *mut interval)).field`, and where one of the mirror's macros
  hands back a `struct interval *` to a call that wants `INTERVAL` the port writes
  `as INTERVAL`.  Neither changes a token C's text carries.
- **`goto` and `continue`** (PORTING.md 2.2/P-57): `adjust_intervals_for_insertion`'s two
  `goto check_done` are the `'check_done` labelled block's `break` (the casefiddle.rs
  `'done` shape) and that loop's two `continue`s are `break 'tail_step` inside the loop's
  labelled body block, whose own `break`s carry the loop's `'tail_loop` label (E0695:
  a `break` that would diverge through a labelled block must name it, the ccl.rs/bytecode.rs
  shape).  `merge_properties_sticky`'s two step loops have the same body block
  (`'right_step`/`'left_step`) and their two inner search loops a label
  (`'right_search`/`'left_search`) for the same reason.  `merge_interval_right`/`
  merge_interval_left` end with C's `emacs_abort ()` as the tail expression (it is `!`).
- **`while (1)`**: `balance_an_interval`'s loops with a `break` are Rust's `while true`
  (the `while_true` allow is already in the file's list); `find_interval` and
  `update_interval` end *in* the loop and leave it only by `return`, so theirs is `loop`
  (itree.rs's `itree_propagate_limit` spelling).  Both spellings lose C's `1`.
- **`%"pD"d`** (C 776): `pD` is `"t"` (lisp.h 150), so the port's literal is
  `"Point %td after end of properties"` (the bytecode.rs/region-cache.rs precedent); the
  two-character `pd` token is skipped by the body check and needs no note.
- **C int literals that are Rust booleans or Rust pointer zeros**: `eobp = 1`/`if (1)`,
  `use_left = 0`/`use_right = 0`, `lookup_char_property (…, 1)`, `return 0`/`return 1` and
  `buffer_intervals (buffer)->up_obj == 1` are recorded in `docs/ports/macro-body-notes.tsv`
  (the `Fset_case_table`/`getenv_internal` classes); every `x = 0` that assigns a null
  `INTERVAL` is written `0 as INTERVAL` so C's own `0` token survives (the
  atimer.rs/syntax.rs cast shape), which is why only the `1`s need rows.
- **`plist_get` is declared at its use site** and reported as a mirror gap: lisp.h 4280
  declares it (`extern Lisp_Object plist_get (Lisp_Object, Lisp_Object);`) and the `lisp.rs`
  mirror does not carry it yet (PORTING.md 0.1-2/P-72 rule 3).  Two more mirror shapes are
  handled with casts at the call site rather than by editing another unit's mirror:
  `buf_charpos_to_bytepos`'s first parameter is the mirror's ABI-equal `*mut c_void`
  (marker.rs 257/454, which the mirror's own comment justifies) so the four calls pass
  `current_buffer as *mut c_void`/`buffer as *mut c_void` (the insdel.rs shape), and
  `XMARKER (marker)->buffer` is likewise `*mut c_void`, so that comparison casts too.
- **`staticpro`/`alloca`: none**; no `DEFVAR_*` and no file-scope object, so
  `tools/check-globals.py` has nothing to check here.  C's three forward declarations
  (C 54-56) are dropped (PORTING.md 0.1-12) and C's uninitialised locals are zero-initialised
  (PORTING.md 1.5): `new`/`i`/`temp`/`B`/`c`/`successor`/`predecessor`/`migrate`/`this`/
  `parent`/`prev`/`to`/`from`/`toprev`/`fromprev` as `std::ptr::null_mut ()`, the `Lisp_Object`
  locals as `Qnil ()`, the `bool`s as `false`, the `ptrdiff_t`s as `0`.
- **Line count**: the port is 2567 lines against C's 2416 (live 2410), which
  `docs/ports/line-drift-tolerance.tsv` declares with the `live` basis and the reason; the
  per-function token comparison is the real guard and it is OK.

- **Function pointers**: `traverse_intervals`/`traverse_intervals_noorder` take C's
  `void (*function) (INTERVAL, void *)`, which the mirror declares as the NULL-able
  `Option<unsafe extern "C" fn (INTERVAL, *mut c_void)>` (`keymap.h`'s
  `map_keymap_function_t` shape, PORTING.md 5.2), and C's `(*function) (tree, arg)` is the
  port's `(function.unwrap ()) (tree, arg)` (the atimer.rs/json.rs shape).  No flexible array
  member and no `alloca` in this file.
- **Packed `bool_bf` reads/writes** all go through `interval_bits!` over the
  `INTERVAL_*_BIT` masks (PORTING.md 1.3/P-80): `copy_interval_parent`'s `d->up_obj =
  s->up_obj`, `graft_intervals_into_buffer`'s `->up_obj == 1`, and the
  `COPY_INTERVAL_CACHE`/`MERGE_INTERVAL_CACHE`/`RESET_INTERVAL` writes of
  `write_protect`/`visible`/`front_sticky`/`rear_sticky`; the occurrence-checked families are
  in `docs/ports/token-aliases.tsv`.

### Call-relationship differences (D7)

**None found.**  Every call in the C text appears in the port with the same callee and the
same argument order, and the port adds no call the reference does not make (the mirror's
`interval_bits!`, `GET_INTERVAL_OBJECT`, `SET_PARENT_POSITION` and `calln!` expansions are
C's own macros expanding at the same points, and `error`'s one variadic call keeps C's
`%td`/`ptrdiff_t` argument).  The two spellings that are not calls: C's `(*function) (…)` is
`.unwrap () (…)` (above), and `TEXT_PROP_MEANS_INVISIBLE`'s `int` value is tested with
`!= 0` at its one use site.
