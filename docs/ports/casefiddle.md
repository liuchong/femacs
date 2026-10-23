# emacs/src/casefiddle.c (764 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 11
- [x] upcase  (Fupcase, 1/1, doc 78d4457b3aad, line 368)
- [x] downcase  (Fdowncase, 1/1, doc 5aba6fab22d2, line 383)
- [x] capitalize  (Fcapitalize, 1/1, doc 842696f52bc6, line 400)
- [x] upcase-initials  (Fupcase_initials, 1/1, doc a7b3311d749c, line 418)
- [x] upcase-region  (Fupcase_region, 2/3, doc 6e9c3280b58d, line 609)
- [x] downcase-region  (Fdowncase_region, 2/3, doc 14ac3de72fe7, line 621)
- [x] capitalize-region  (Fcapitalize_region, 2/3, doc 02c9efd1c59e, line 632)
- [x] upcase-initials-region  (Fupcase_initials_region, 2/3, doc 92d631864eb4, line 646)
- [x] upcase-word  (Fupcase_word, 1/1, doc a788a76b27b9, line 671)
- [x] downcase-word  (Fdowncase_word, 1/1, doc 03cce83128f8, line 684)
- [x] capitalize-word  (Fcapitalize_word, 1/1, doc ae3c16607079, line 696)

## Functions defined — 17
- [x] prepare_casing_context  (line 68, static)
- [x] case_ch_is_word  (line 96, static)
- [x] case_character_impl  (line 115, static)
- [x] case_single_character  (line 206, static)
- [x] case_character  (line 221, static)
- [x] make_char_unibyte  (line 242, static)
- [x] do_casify_natnum  (line 248, static)
- [x] do_casify_multibyte_string  (line 281, static)
- [x] ascii_casify_character  (line 318, static)
- [x] do_casify_unibyte_string  (line 328, static)
- [x] casify_object  (line 351, static)
- [x] do_casify_unibyte_region  (line 443, static)
- [x] do_casify_multibyte_region  (line 475, static)
- [x] casify_region  (line 529, static)
- [x] casify_pnc_region  (line 590, static)
- [x] casify_word  (line 661, static)
- [x] syms_of_casefiddle  (line 712)

## Macros defined — 0

## Types and their fields — 2
- [x] casing_context: Lisp_Object titlecase_char_table, Lisp_Object specialcase_char_tables, enum case_action flag, bool inbuffer, bool inword, bool downcase_last
- [x] casing_str_buf: unsigned char data, unsigned char len_chars, unsigned char len_bytes

## File-scope objects — 0

## Decisions

- **`enum case_action` (C 30) and the two Greek constants (C 197-198)** are C's integer
  enums, spelled as their C type plus one constant per member (PORTING.md 1.3, the
  coding.rs precedent: `pub type case_action = c_int;` + `CASE_UP` … `CASE_CAPITALIZE_UP`).
  C compares and subtracts them (`flag < CASE_CAPITALIZE`, `ctx->flag - was_inword`), which
  an integer type keeps one-to-one.
- **`struct casing_str_buf.data`'s bound** is C's `max (6, MAX_MULTIBYTE_LENGTH)` (C 102);
  the port folds it into `CASING_STR_BUF_DATA_LEN` because an array length must be a
  constant expression and the fold has no C name of its own (the `BUF_BYTES_MAX` row's
  reason).  C's `static_assert (offsetof (struct casing_str_buf, data) == 0)` is
  `static_assert! (std::mem::offset_of! (casing_str_buf, data) == 0)`.
- **`ckd_mul`/`ckd_add` (C 289-290)** are the `overflowing_*` pair with "the store happens
  only when the operation did not overflow" (PORTING.md 2.6/P-15, the ckd_add/ckd_mul
  families of token-aliases.tsv).  The `||` short circuit is kept as the two tuples plus a
  conditional `n`.
- **`calln (Vregion_extract_function, Qbounds)` (C 594)** is lisp.h 3462's `calln` macro,
  i.e. `CALLN (Ffuncall, Vregion_extract_function, Qbounds)`; the port calls the mirror's
  `calln!` with the same two arguments rather than casting the variable to a function
  pointer, which is what C's `calln` passes to `Ffuncall` as its first argument.
- **`goto done` (C 148)** becomes the labelled block `'done: { … }` whose `break 'done`
  lands on the same tail statements; `docs/ports/macro-body-notes.tsv` records that the
  `goto` token therefore has no Rust site (PORTING.md 2.2, the getdelim precedent).
- **`case_symbols_as_words = 0` (C 749)**: the variable is a `bool` in the generated
  globals, so the port writes `false`; the `syms_of_casefiddle` row of
  `docs/ports/macro-body-notes.tsv` records it together with the `doc:` label the
  `DEFVAR_*` mirrors consume positionally (the syms_of_data row's two classes).
- **`#ifdef HAVE_TREE_SITTER` (C 543-552, 572-579)**: `HAVE_TREE_SITTER` is `false` in the
  frozen configuration, so the two arms are kept as `#[cfg (feature = "have_tree_sitter")]` text
  (P-51) with the two treesit.h entries they call declared inside the arms, as the
  vm-limit.c dead arms do (status §3.2); a `const _: () = assert! (!cfg! (feature =
  "tree_sitter") || HAVE_TREE_SITTER);` ties the feature to the configuration (the
  buffer.rs precedent).  They are not in `xtask dead-arms`'s list, so nothing compiles them.
- **Call-relationship differences (D7)**: none.  Every callee is either defined here or
  already declared in the mirrors: `uniprop_table`, `multibyte_chars_in_text` (not used
  here), `string_byte_to_char`, `modify_text`, `replace_range_2`, `string_overflow`,
  `make_string_from_bytes`, `oblookup`, `save_match_data_load`, `string_overflow`,
  `record_insert`/`record_delete` (declared in lisp.rs's `other_units` group from
  lisp.h 4227-5252; `record_insert`/`record_delete` are defined by undo.rs and the
  declaration there is shadowed by that definition, PORTING.md 0.1-10), `scan_words` and
  `syntax_prefix_flag_p` (the new syntax.rs mirror), `SETUP_BUFFER_SYNTAX_TABLE`,
  `signal_after_change`, `update_compositions`, `Fset_case_table`, `Fmake_variable_buffer_local`.
- **Docstrings**: the eleven `doc:` literals carry make-docfile's value (no
  leading/trailing whitespace, `\"C\"` decoded to `"C"`, the ﬁ ligature kept as the same
  bytes); `cinventory.doc_comment_value` is the implementation both sides use.
- Evidence: `xtask build-tu src/casefiddle.c` -> `built src/casefiddle.rs ->
  ../build/femacs/casefiddle.o`; `tools/extract-abi.py --check emacs/src/casefiddle.c
  src/casefiddle.rs` -> OK.
