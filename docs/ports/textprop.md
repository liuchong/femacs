# emacs/src/textprop.c (2477 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 20
- [x] text-properties-at  (Ftext_properties_at, 1/2, doc 005f7ebedbe0, line 558)
- [x] get-text-property  (Fget_text_property, 2/3, doc bcdf4832e409, line 593)
- [x] get-char-property  (Fget_char_property, 2/3, doc b40a5c68df33, line 681)
- [x] get-char-property-and-overlay  (Fget_char_property_and_overlay, 2/3, doc c84142c6376f, line 695)
- [x] next-char-property-change  (Fnext_char_property_change, 1/2, doc fd2ce0e8072f, line 718)
- [x] previous-char-property-change  (Fprevious_char_property_change, 1/2, doc dc5f8ae8508c, line 744)
- [x] next-single-char-property-change  (Fnext_single_char_property_change, 2/4, doc aa8b4c8ce8a6, line 771)
- [x] previous-single-char-property-change  (Fprevious_single_char_property_change, 2/4, doc 5561a67e653f, line 857)
- [x] next-property-change  (Fnext_property_change, 1/3, doc e22c30ed40c5, line 952)
- [x] next-single-property-change  (Fnext_single_property_change, 2/4, doc e6b097569c86, line 1017)
- [x] previous-property-change  (Fprevious_property_change, 1/3, doc d17a41217112, line 1066)
- [x] previous-single-property-change  (Fprevious_single_property_change, 2/4, doc 9cf902edf10e, line 1114)
- [x] add-text-properties  (Fadd_text_properties, 3/4, doc 63ed4d9329bf, line 1296)
- [x] put-text-property  (Fput_text_property, 4/5, doc f58acd1bb0d9, line 1314)
- [x] set-text-properties  (Fset_text_properties, 3/4, doc a22c157d75cd, line 1330)
- [x] add-face-text-property  (Fadd_face_text_property, 3/5, doc aba49e9c629a, line 1345)
- [x] remove-text-properties  (Fremove_text_properties, 3/4, doc e573a2acb3e2, line 1554)
- [x] remove-list-of-text-properties  (Fremove_list_of_text_properties, 3/4, doc 90cab0a8c9e5, line 1683)
- [x] text-property-any  (Ftext_property_any, 4/5, doc 2e168afcd3be, line 1821)
- [x] text-property-not-all  (Ftext_property_not_all, 4/5, doc 8f7d03e2a686, line 1857)

## Functions defined — 26
- [x] text_read_only  (line 67, static)
- [x] modify_text_properties  (line 78, static)
- [x] CHECK_STRING_OR_BUFFER  (line 100, static)
- [x] validate_interval_range  (line 128)
- [x] validate_plist  (line 193, static)
- [x] interval_has_all_properties  (line 221, static)
- [x] interval_has_some_properties  (line 256, static)
- [x] interval_has_some_properties_list  (line 278, static)
- [x] property_value  (line 301, static)
- [x] set_properties  (line 319, static)
- [x] add_properties  (line 368, static)
- [x] remove_properties  (line 464, static)
- [x] interval_of  (line 523)
- [x] get_char_property_and_overlay  (line 619)
- [x] add_text_properties_1  (line 1170, static)
- [x] set_text_properties  (line 1386)
- [x] set_text_properties_1  (line 1469)
- [x] text_property_stickiness  (line 1901)
- [x] copy_text_properties  (line 1970)
- [x] text_property_list  (line 2045)
- [x] add_text_properties_from_list  (line 2100)
- [x] extend_property_ranges  (line 2123)
- [x] call_mod_hooks  (line 2166, static)
- [x] verify_interval_modification  (line 2184)
- [x] report_interval_modification  (line 2384)
- [x] syms_of_textprop  (line 2395)

## Macros defined — 2
- [x] PLIST_ELT_P  (line 56, body 598cfa05af84)
- [x] TMEM  (line 29, body e3dee28f08fb)

## Types and their fields — 0

## File-scope objects — 2
- [x] Lisp_Object interval_insert_behind_hooks  (line 60)
- [x] Lisp_Object interval_insert_in_front_hooks  (line 61)

## Decisions

- **What the file carries.** C's 20 `DEFUN`s (all via `DEFUN!`, PORTING.md 5.1), its 26
  functions (13 non-`static`: `validate_interval_range`, `interval_of`,
  `get_char_property_and_overlay`, `set_text_properties`, `set_text_properties_1`,
  `text_property_stickiness`, `copy_text_properties`, `text_property_list`,
  `add_text_properties_from_list`, `extend_property_ranges`,
  `verify_interval_modification`, `report_interval_modification`, `syms_of_textprop` —
  they carry `#[unsafe(no_mangle)]`; the 13 `static` helpers stay plain `unsafe fn`), the two
  file-local macros (`TMEM`, `PLIST_ELT_P`), the `enum property_set_type` (with the two
  `soft`/`hard` constants of C 125's anonymous enum), the two file-scope objects
  (`interval_insert_behind_hooks`, `interval_insert_in_front_hooks`) and the four
  `DEFVAR_LISP` variables this unit owns (`Vdefault_text_properties`,
  `Vchar_property_alias_alist`, `Vinhibit_point_motion_hooks`,
  `Vtext_property_default_nonsticky`) — status 3.29, so they are defined here as
  `#[unsafe(no_mangle)] pub static mut … = lisp_h_Qnil ()`, the casefiddle.rs shape.
- **No `#if` at all.** The file's only preprocessor lines are the four includes and the two
  `#define`s; `tools/check-cond-eval.py src/textprop.c` reports zero conditions, and it is in
  `docs/baseline/config/cond-eval-verified.txt`, so there is no dead arm to record (P-08) and
  none to carry (P-51).  `line-drift-tolerance.tsv` therefore needs no row: C 2477 vs the
  port's 2577 is 4.0%, inside the default 5%.
- **The P-70 same-commit mirror moves** (PORTING.md 0.1-10/P-21). (1) `intervals.h`'s
  "Defined in textprop.c" block (`copy_text_properties`, `set_text_properties`,
  `set_text_properties_1`, `text_property_list`, `add_text_properties_from_list`,
  `extend_property_ranges`, `get_char_property_and_overlay`,
  `text_property_stickiness`, `syms_of_textprop`) moved into `pub mod textprop_decls` +
  glob import in `src/include/intervals.rs`; the three prototypes of the "Declared in
  intervals.c" block that this file also defines (`verify_interval_modification`,
  `validate_interval_range`, `interval_of`) were already in `intervals_decls` + glob import.
  (2) `buffer.h`'s two `interval_insert_*_hooks` objects moved into `pub mod textprop_objs` +
  glob import in `src/include/buffer.rs`.  Both are recorded in
  `docs/ports/intervals.md` and `docs/ports/include-buffer.md`; `build-tu` was re-run for
  `src/intervals.c` and `src/insdel.c` (and their `--check`s) to prove no other consumer
  regressed.
- **`CHECK_FIXNUM_COERCE_MARKER` gained a second arm** in the buffer.rs mirror: C 136-137
  call it as `CHECK_FIXNUM_COERCE_MARKER (*begin)` on a pointer's pointee, and the existing
  `$x:ident` arm cannot spell a deref place expression.  The new `$x:expr` arm expands to the
  same statement (`$x = make_fixnum (unsafe { fix_position ($x) })`), so the macro's name
  stays in the port's text for the token check.  Recorded in
  `docs/ports/include-buffer.md`.
- **Reported mirror gaps (declared at the use site, not by editing the mirrors).** `lisp.rs`
  has no `nconc2` (lisp.h 4271), `plist_get` (4280), `prepare_to_modify_buffer_1` (4354) or
  `record_property_change` (5255).  All four are declared in this unit's own
  `unsafe extern "C"` block with C's prototype in a `/// C:` line (the insdel.rs/terminal.rs
  shape) and are listed here for the mirror owner.  `lisp.rs` still lacks `larger_vector` and
  `make_specified_string`, which this file does not use.
- **`ITREE_FOREACH` is expanded at its use site** (P-33, status 3.7; the five users are
  buffer.c, itree.c, editfns.c, textprop.c and xdisp.c).  `get_char_property_and_overlay`
  writes C's expansion: the `if (!t) { } else` guard, the in-place
  `let mut itree_local_iter_: itree_iterator = std::mem::zeroed ()` that C declares at the
  expansion site, `itree_iterator_start (…, ITREE_ASCENDING)` and the
  `while { node = itree_iterator_next (itree_iter_); !node.is_null () }` loop the macro's
  `for (…; ((n = itree_iterator_next (itree_iter_)));)` stands for.  The macro's name and the
  pasted `ASCENDING` argument cannot reappear, which is the recorded `macro-body-notes.tsv`
  row (the itree_insert_gap row's class); `use itree_order::*;` imports the enum's members as
  Rust 2024 requires (the itree.rs shape).
- **Control flow.** The three `goto retry` (C 1249, 1435, 1626) become `'retry: loop` with
  `continue 'retry` (PORTING.md 2.2, the doc.rs `Fdocumentation` shape); the `goto` token
  itself has no Rust site and is recorded per function in `docs/ports/macro-body-notes.tsv`.
  C 937-941's `else while (true) …` needs the explicit block Rust requires; C 1513's chained
  assignment `prev_changed = i = merge_interval_left (i)` becomes the block expression
  `prev_changed = { i = merge_interval_left (i); i }` (the menu.rs
  `*maps.offset (i) = { keymap = …; keymap }` shape), the only place Rust's lack of an
  assignment expression shows.
- **Parameters C rebinds.** `DEFUN!`'s pattern takes plain bindings, so the subrs whose
  bodies write a parameter (`Ftext_properties_at`, `Fnext_char_property_change`,
  `Fprevious_char_property_change`, `Fnext_single_char_property_change`,
  `Fprevious_single_char_property_change`, `Fnext_property_change`,
  `Fnext_single_property_change`, `Fprevious_property_change`,
  `Fprevious_single_property_change`, `Fremove_text_properties`,
  `Fremove_list_of_text_properties`, `Ftext_property_any`, `Ftext_property_not_all`) start
  with `let mut <name> = <name>;` at the top of the body (the data.rs/composite.rs shape,
  with that shape's scope rule: the rebinding must be outside the block that writes it).
  The plain functions `interval_of` and `verify_interval_modification` instead take
  `mut object`/`mut start`/`mut end` in their own parameter lists, and
  `add_text_properties_1`, `set_text_properties`, `set_text_properties_1`,
  `text_property_stickiness`, `get_char_property_and_overlay`, `call_mod_hooks`,
  `copy_text_properties`, `text_property_list`, `add_text_properties_from_list` and
  `extend_property_ranges` rebind
  the same way.  `&raw mut x` also needs the binding to be `mut`, which is why
  `Ftext_properties_at`'s `position`/`object` pair is rebound even though only the first is
  written.
- **Uninitialised locals (PORTING.md 1.5)** are zero-initialised: every `INTERVAL i`/
  `unchanged`/`prev`/`previous`/`next`/`interval`/`result` starts at `std::ptr::null_mut ()`,
  every `Lisp_Object` at `Qnil ()`, every `ptrdiff_t`/`bool` at `0`/`false`, and
  `struct sortvec items[2]` at `[std::mem::zeroed (); 2]`.  `bool ignore_previous_character`
  (C 1903) is assigned before its first read and starts at `false`.
  `validate_interval_range`'s `Lisp_Object n`, `add_properties`'s `this_cdr`,
  `text_property_stickiness`'s `front_sticky`/`defalt` and the several `Lisp_Object tem`
  declarations are the same zero-value shape.
- **Value spellings recorded in the shared tables.** `docs/ports/macro-body-notes.tsv` gains
  eight rows for this unit: the `ITREE_FOREACH` expansion (`itree_foreach,ascending`), the
  three `goto` rows (`add_text_properties_1`, `Fremove_text_properties`,
  `set_text_properties`), `syms_of_textprop`'s positional DEFVAR docstring (`doc`),
  `Fget_char_property`'s `0` overlay argument, and the two bodies (`set_text_properties_1`,
  `verify_interval_modification`) whose only `0`s are C's null-pointer literals.  C's
  `i != 0` / `prev = 0` / `w = 0` / `set_string_intervals (object, NULL)` and C's `!i` tests
  are the port's `.is_null ()` / `!i.is_null ()` / `std::ptr::null_mut ()` throughout (the
  insdel.rs/intervals.rs shape); the other `eassert (i != 0)` sites (C 1254, 1644, 1755)
  needed no row because their bodies carry other `0` tokens.  A per-token *count* comparison
  against C (the P-38 multiplicity blind spot) reports only the `add_text_properties_1`
  parameter-list artifact and those two recorded `0` rows.
- **The four use-site declarations** (above) and the `enum property_set_type`, whose Rust
  declaration is `#[repr(C)] #[derive(Clone, Copy, PartialEq, Eq)]` because C's enum is
  trivially copyable and the two comparisons (`set_type == TEXT_PROPERTY_REPLACE`,
  `== TEXT_PROPERTY_PREPEND`) need `PartialEq`; `soft`/`hard` are `pub const … : bool`.
  intervals.h 269-270's prototype of `verify_interval_modification` is the header's own
  declaration and reaches this unit through the mirror's `intervals_decls` glob import.
