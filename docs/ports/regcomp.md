# emacs/lib/regcomp.c (3814 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 57
- [x] re_compile_pattern  (line 210)
- [x] re_set_syntax  (line 247)
- [x] re_compile_fastmap  (line 257)
- [x] re_set_fastmap  (line 276, static)
- [x] re_set_fastmap_icase  (line 286, static)
- [x] re_compile_fastmap_iter  (line 307, static)
- [x] regcomp  (line 458)
- [x] regerror  (line 516)
- [x] free_dfa_content  (line 578, static)
- [x] regfree  (line 626)
- [x] re_compile_internal  (line 719, static)
- [x] init_dfa  (line 818, static)
- [x] init_word_char  (line 913, static)
- [x] free_workarea_compile  (line 961, static)
- [x] create_initial_state  (line 980, static)
- [x] optimize_utf8  (line 1065, static)
- [x] analyze  (line 1144, static)
- [x] postorder  (line 1207, static)
- [x] preorder  (line 1239, static)
- [x] optimize_subexps  (line 1272, static)
- [x] lower_subexps  (line 1303, static)
- [x] lower_subexp  (line 1325, static)
- [x] calc_first  (line 1363, static)
- [x] calc_next  (line 1385, static)
- [x] link_nfa_nodes  (line 1408, static)
- [x] duplicate_node_closure  (line 1468, static)
- [x] search_duplicated_node  (line 1575, static)
- [x] duplicate_node  (line 1593, static)
- [x] calc_inveclosure  (line 1609, static)
- [x] calc_eclosure  (line 1633, static)
- [x] calc_eclosure_iter  (line 1674, static)
- [x] fetch_token  (line 1758, static)
- [x] peek_token  (line 1767, static)
- [x] peek_token_bracket  (line 1999, static)
- [x] parse  (line 2105, static)
- [x] parse_reg_exp  (line 2139, static)
- [x] parse_branch  (line 2188, static)
- [x] parse_expression  (line 2233, static)
- [x] parse_sub_exp  (line 2465, static)
- [x] parse_dup_op  (line 2508, static)
- [x] parse_byte  (line 2659, static)
- [x] build_range_exp  (line 2672, static)
- [x] build_collating_symbol  (line 2772, static)
- [x] parse_bracket_exp  (line 3047, static)
- [x] parse_bracket_element  (line 3324, static)
- [x] parse_bracket_symbol  (line 3362, static)
- [x] build_equiv_class  (line 3409, static)
- [x] build_charclass  (line 3490, static)
- [x] build_charclass_op  (line 3566, static)
- [x] fetch_number  (line 3655, static)
- [x] free_charset  (line 3677, static)
- [x] create_tree  (line 3695, static)
- [x] create_token_tree  (line 3703, static)
- [x] mark_opt_subexp  (line 3740, static)
- [x] free_token  (line 3752, static)
- [x] free_tree  (line 3764, static)
- [x] duplicate_tree  (line 3777, static)

## Macros defined — 19
- [x] BRACKET_NAME_BUF_SIZE  (line 2651, body cb4e5208b4cd)
- [x] BUILD_CHARCLASS_LOOP  (line 3519, body 516381d2dd43)
- [x] REG_BADBR_IDX  (line 157, body ca3b92d1baca)
- [x] REG_BADPAT_IDX  (line 133, body 7e6e352feec5)
- [x] REG_BADRPT_IDX  (line 166, body 14993a6cb099)
- [x] REG_EBRACE_IDX  (line 154, body a453d3a08314)
- [x] REG_EBRACK_IDX  (line 148, body 05de9b7813e8)
- [x] REG_ECOLLATE_IDX  (line 136, body dad8184a3e44)
- [x] REG_ECTYPE_IDX  (line 139, body 87b619888fa9)
- [x] REG_EEND_IDX  (line 169, body 763a6ecac218)
- [x] REG_EESCAPE_IDX  (line 142, body 3cdb5989490a)
- [x] REG_EPAREN_IDX  (line 151, body 73cad2ce2e79)
- [x] REG_ERANGE_IDX  (line 160, body c98fc76d6153)
- [x] REG_ERPAREN_IDX  (line 175, body 99a26200bbed)
- [x] REG_ESIZE_IDX  (line 172, body 73ac2e403e3b)
- [x] REG_ESPACE_IDX  (line 163, body a6e5e0ad32ee)
- [x] REG_ESUBREG_IDX  (line 145, body 1e9e7892d5a2)
- [x] REG_NOERROR_IDX  (line 127, body b6589fc6ab0d)
- [x] REG_NOMATCH_IDX  (line 130, body e18067f80db5)

## Types and their fields — 0

## File-scope objects — 1
- [x] reg_syntax_t re_syntax_options  (line 236)

## Decisions

- **Entry point and file scope.** `re_syntax_options` is the file's one object
  (regcomp.rs 131); `BRACKET_NAME_BUF_SIZE` (C 2651) is emitted with the other file-scope
  names (regcomp.rs 309) rather than at its C position, because C's `#define` sits between
  two functions and Rust's `macro_rules!`/`const` order is the same either way — the name,
  the value (32) and the use sites are unchanged.
- **`REG_*_IDX` / `__re_error_msgid`.** C's `#define REG_X_IDX (REG_PREV_IDX + sizeof
  "message")` cannot be spelled (`sizeof` of a string literal): the port writes the
  measured byte offset next to the same predecessor and `__re_error_msgid` as one byte
  string with C's concatenated literals; `docs/ports/macro-body-notes.tsv` has the row.
- **`BUILD_CHARCLASS_LOOP`** (C 3519, inside `build_charclass`) is a `macro_rules!` in that
  function's body, at C's position: `macro_rules!` cannot name the caller's locals
  (`trans`/`i`/`sbcset`), so it takes them as parameters and every call site passes them
  (PORTING.md 2.1); C's `do { … } while (0)` is the block, and its two `for` loops are
  `while` loops with an explicit step (`macro-body-notes.tsv` row `BUILD_CHARCLASS_LOOP`).
- **`goto` tails** (PORTING.md 2.2). `parse_dup_op`'s five `goto parse_dup_op_espace` and
  `build_charclass_op`'s two `goto build_word_op_espace` are `break` out of a labelled
  block whose end *is* the tail. `parse_bracket_exp`'s two labels are
  `'parse_bracket_exp_free_return: loop { 'parse_bracket_exp_espace: { … } *err =
  REG_ESPACE; break; }`: the espace break lands on `*err = REG_ESPACE`, the free_return
  break skips it, and the normal path returns inside — C's control flow exactly.
- **Dead `_LIBC` arms** (regcomp.c 2788-3041, 3066-3080, 3412-3473): the frozen
  configuration has no `_LIBC`, so the six glibc collation helpers
  (`seek_collating_symbol_entry`, `lookup_collation_sequence_value`, the `_LIBC`
  `build_range_exp`/`build_collating_symbol`/`build_equiv_class` halves) and the
  `_NL_COLLATE_*`/`_NL_CTYPE_*` sources are not ported; each spot is marked by a comment
  (P-51/P-08).
- **`switch` arm order** (PORTING.md 2.2): `parse_bracket_exp`'s switch on
  `start_elem.type` and `parse_bracket_symbol`'s switch on `token->type` are if/else-if
  chains in C's case order, with `DEBUG_ASSERT (false)` where C has `default:` in the
  first one and the chain's fall-through end for `default: break;` in the second.
- **Uninitialized locals.** C leaves `sbcset`/`mbcset` (`parse_bracket_exp`) and
  `token_len`, `tree`, `ret` uninitialized; the port gives them the null/zero initializer
  or a deferred binding (`let mut token_len: c_int;`), the PORTING.md 1.5 shape.
- **Type spellings inside bodies.** `parse_byte`'s argument, `bitset_set`'s index and the
  `wchar_t`/`wint_t` pair get C's implicit conversions spelled as `as` casts;
  `token->type` reads are the mirror's `type_ (&mut …)` and its writes `set_type_ (…)`
  (the `type` family in `token-aliases.tsv`), `dfa->has_mb_node`/`mbcset->non_match` are
  the mirror's mask writers, and `BITSET_WORDS`/`SBC_MAX`/`RE_DUP_MAX` are the mirror's
  constants (the last one cast to `Idx` because C compares it with an `Idx`).
- **Mirror additions this file needed** (five files, all reported to the main agent):
  `lib/include/ctype.rs` gained `iscntrl`/`isgraph`/`isprint`/`ispunct`/`isxdigit`
  (C's `<ctype.h>` names the twelve classes `build_charclass` looks up — added after
  grepping the tree: no other declaration site exists);
  `lib/include/stdint.in.rs` gained `int_fast32_t`/`uint_fast32_t` (measured with the
  frozen target's compiler: both 4 bytes);
  `lib/include/limits.in.rs` gained `MB_LEN_MAX` (measured: 6, used by
  `re_compile_fastmap_iter`'s `char buf[MB_LEN_MAX]`);
  `lib/include/regex_internal.rs` gained `PartialEq, Eq` on `re_context_type` and
  `bracket_elem_type` (C compares both with `==`; derives change no layout);
  `lib/include/regex.rs`'s `regex.h` 210 `extern reg_syntax_t re_syntax_options`
  moved into `pub mod regex_decls` so this unit's *definition* of the same name can stand
  (PORTING.md 0.1-10; Rust forbids an extern static and a defined static in one scope).
- **The functions this unit calls but does not define** are `static` in the *aggregate*
  translation unit C builds for the regex cluster (`lib/regex.c` includes
  `regex_internal.c`, `regcomp.c` and `regexec.c` into one TU, so each file's `static`
  forward declarations cover the others' definitions). The port keeps one file per unit, so
  `regex_internal.c`'s and `regexec.c`'s definitions are named in this file's own
  `unsafe extern "C"` block (`re_string_construct`, `re_string_destruct`,
  `re_string_peek_byte_case`, `re_string_fetch_byte_case`, `re_dfa_add_node`, `free_state`,
  `re_node_set_alloc`/`init_1`/`init_2`/`init_copy`/`merge`/`insert`/`insert_last`/`contains`,
  `re_acquire_state_context`), each with its C site in the comment.
- **Exported symbols.** The frozen build's `lib/regex.o` exports the gnulib-renamed
  `_rpl_re_compile_pattern`, `_rpl_re_set_syntax`, `_rpl_re_compile_fastmap`, `_rpl_regcomp`,
  `_rpl_regerror`, `_rpl_regfree` and `_rpl_re_syntax_options` (verified with `nm`), so the
  port keeps C's source names on the items and emits those symbols through
  `#[unsafe(export_name = …)]` (PORTING.md 10.13, the `__mktime64`/`rpl_mktime` precedent);
  `nm -g build/femacs/regcomp.o` now shows exactly that set.
