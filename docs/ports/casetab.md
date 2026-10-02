# emacs/src/casetab.c (303 lines)

## Subrs (DEFUN) — 5
- [x] Fcase_table_p
- [x] Fcurrent_case_table
- [x] Fset_case_table
- [x] Fset_standard_case_table
- [x] Fstandard_case_table

## Functions defined — 7
- [x] check_case_table
- [x] init_casetab_once
- [x] set_canon
- [x] set_case_table
- [x] set_identity
- [x] shuffle
- [x] syms_of_casetab

## Macros defined — 0

## Types — 0

## Externs declared — 0

## Globals defined — 2
- [x] Vascii_canon_table
- [x] Vascii_downcase_table

## Decisions

`emacs/src/casetab.c` @ `fd7afced403`; port `femacs/src/casetab.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/src/casetab.c femacs/src/casetab.rs` -> `OK`.

- Include order: config.h, lisp.h, buffer.h — with buffer.h's own character.h/itree.h —
  becomes `config.rs`, `lisp.rs`, `character.rs`, `itree.rs`, `buffer.rs` (0.1-4).
- Subrs: the five `DEFUN`s use `DEFUN!` in C's field order with the body as the last
  argument (`PORTING.md` 5.1); the three `map_char_table` callbacks are private
  `unsafe extern "C" fn`s so `map_char_table` can take their address.  C's three forward
  declarations of them (casetab.c 33-35) and of `set_case_table` (80) have no Rust
  counterpart: items are order-independent here.
- The two globals casetab.c defines (`Vascii_downcase_table`, `Vascii_canon_table`) are
  declared by lisp.h, so the port defines them with `#[unsafe(no_mangle)]` at the unit
  root, where they shadow lisp.rs's declaration (P-21/P-39; `spikes/shadow-probe.sh`).
  The two file-`static` ones (`Vascii_upcase_table`, `Vascii_eqv_table`) stay private
  statics; `staticpro (&raw const V…)` gives them the stable addresses C registers.
- Flexible array members: `XCHAR_TABLE (x)->extras[i]` is a FAM access, and the mirror
  declares FAMs as `[T; 0]` (so `sizeof`/following offsets match C).  Indexing a
  `[T; 0]` panics in Rust, so the port writes `FAM_ELEM! ((*XCHAR_TABLE (x)).extras, i)`
  (PORTING.md 1.3, P-40).  That defect was latent in seven pre-existing lisp.rs
  accessors (`AREF`, `ASET`, `aref_addr`, `gc_aset`, `CHAR_TABLE_REF_ASCII`,
  `set_char_table_extras`, `set_char_table_contents`) and is fixed in the same commit.
- Type conversions C leaves implicit are written out: `XFIXNUM` results become `c_int`
  in the `from`/`to` locals and `make_fixnum (from as EMACS_INT)` on the way back;
  `CHAR_TABLE_SET`'s `idx` is `isize`; the `'A'`/`'a'` char constants are `c_int` casts;
  `set_case_table (table, 0/1)` is `false`/`true`.
- `DEFUN!` bodies and the two `#[unsafe(no_mangle)] pub unsafe extern "C" fn`s
  (`init_casetab_once`, `syms_of_casetab`) keep C's shape, so the edition-2024
  `unsafe_op_in_unsafe_fn` lint warns inside them rather than an `unsafe {}` being added
  per statement (the first TU where this showed up in bulk).  `PORTING.md` P-43 fixed the
  policy: every mirror carries the same allow list, including `unsafe_op_in_unsafe_fn`,
  because review D's F12 found this file's 438 warnings and `doprnt.rs`'s 6 to be two
  answers to one question inside one commit.
- `Qcase_table`/`Qcase_table_p`/`Qchar_table_extra_slots` come from the generated
  `globals.rs`; `DEFSYM` is the empty macro C defines (lisp.h 2394), so the mirror writes
  `DEFSYM! (Qcase_table, "case-table");` with no effect, exactly as C does.
- Call-relationship differences (D7): none; every callee is either defined here or
  declared in lisp.rs's `other_units` module (`staticpro`, `map_char_table` were added
  from lisp.h 3919/4689 for this unit).
- Docstrings: the five `doc:` literals carry **make-docfile's value**, not the raw comment
  text — no leading/trailing whitespace, and C's `\"extra\"` becomes `"extra"` (the escape
  keeps only the escaped character).  The reviewer measured the tree: 1,838 of 1,849 DEFUN
  docs have trailing whitespace before `*/`, and 139 carry a `\`+non-`n/t` escape, so this
  convention is what makes `etc/DOC` reachable at all; `cinventory.doc_comment_value` is the
  one implementation both sides use (P-24's docstring comparison).
- Evidence: `--check` -> OK; `xtask build-tu src/casetab.c` -> `built src/casetab.rs ->
  build/femacs/casetab.o`, and the same unit again with `--cfg enable_checking` (P-38's
  checking build), so the DEFUN machinery and the C assertions are both exercised.
- Reviewer A built a **behavioural differential** for this file (a minimal model of
  chartab/alloc/print on both sides, `casetab.c` vs `casetab.rs`, 10 scenarios): 153 output
  lines identical, and 20 of 23 semantic mutants caught — including the `shuffle`/`set_canon`
  map callbacks, which need a table whose three `extras` are all nil (in
  `init_casetab_once`'s own call they are already filled, so those callbacks never run).
  Recorded in P-30 as the method for the remaining `.c` units.
