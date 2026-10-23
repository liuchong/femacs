# emacs/lib/regexec.c (4306 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 61
- [x] regexec  (line 187)
- [x] re_match  (line 271)
- [x] re_search  (line 281)
- [x] re_match_2  (line 292)
- [x] re_search_2  (line 304)
- [x] re_search_2_stub  (line 316, static)
- [x] re_search_stub  (line 363, static)
- [x] re_copy_regs  (line 454, static)
- [x] re_set_registers  (line 532)
- [x] re_search_internal  (line 580, static)
- [x] prune_impossible_nodes  (line 973, static)
- [x] acquire_init_state_context  (line 1073, static)
- [x] check_matching  (line 1115, static)
- [x] check_halt_node_context  (line 1247, static)
- [x] check_halt_state_context  (line 1265, static)
- [x] proceed_next_node  (line 1284, static)
- [x] push_fail_stack  (line 1391, static)
- [x] pop_fail_stack  (line 1420, static)
- [x] set_regs  (line 1450, static)
- [x] free_fail_stack_return  (line 1543, static)
- [x] update_regs  (line 1559, static)
- [x] sift_states_backward  (line 1630, static)
- [x] build_sifted_states  (line 1686, static)
- [x] clean_state_log_if_needed  (line 1742, static)
- [x] merge_state_array  (line 1768, static)
- [x] update_cur_sifted_state  (line 1794, static)
- [x] add_epsilon_src_nodes  (line 1842, static)
- [x] sub_epsilon_src_nodes  (line 1870, static)
- [x] check_dst_limits  (line 1918, static)
- [x] check_dst_limits_calc_pos_1  (line 1953, static)
- [x] check_dst_limits_calc_pos  (line 2034, static)
- [x] check_subexp_limits  (line 2063, static)
- [x] sift_states_bkref  (line 2151, static)
- [x] sift_states_iter_mb  (line 2249, static)
- [x] transit_state  (line 2278, static)
- [x] merge_state_with_log  (line 2333, static)
- [x] find_recover_state  (line 2411, static)
- [x] check_subexp_matching_top  (line 2441, static)
- [x] transit_state_mb  (line 2512, static)
- [x] transit_state_bkref  (line 2580, static)
- [x] get_subexp  (line 2692, static)
- [x] get_subexp_sub  (line 2845, static)
- [x] find_subexp_node  (line 2873, static)
- [x] check_arrival  (line 2896, static)
- [x] check_arrival_add_next_nodes  (line 3061, static)
- [x] check_arrival_expand_ecl  (line 3136, static)
- [x] check_arrival_expand_ecl_sub  (line 3187, static)
- [x] expand_bkref_cache  (line 3232, static)
- [x] build_trtable  (line 3320, static)
- [x] group_nodes_into_DFAstates  (line 3509, static)
- [x] check_node_accept_bytes  (line 3697, static)
- [x] check_node_accept  (line 3985, static)
- [x] extend_buffers  (line 4033, static)
- [x] match_ctx_init  (line 4095, static)
- [x] match_ctx_clean  (line 4129, static)
- [x] match_ctx_free  (line 4158, static)
- [x] match_ctx_add_entry  (line 4173, static)
- [x] search_cur_bkref_entry  (line 4221, static)
- [x] match_ctx_add_subtop  (line 4244, static)
- [x] match_ctx_add_sublast  (line 4272, static)
- [x] sift_ctx_init  (line 4298, static)

## Macros defined — 4
- [x] DYNARRAY_ELEMENT  (line 1439, body c7e59349a57f)
- [x] DYNARRAY_PREFIX  (line 1440, body 29e10af9356b)
- [x] DYNARRAY_STRUCT  (line 1438, body 581551ed867a)
- [x] STATE_NODE_CONTAINS  (line 1626, body ac9c3ea8235c)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **Entry points and exported symbols.** The frozen build's `lib/regex.o` exports
  `_rpl_regexec`, `_rpl_re_match`, `_rpl_re_search`, `_rpl_re_match_2`,
  `_rpl_re_search_2` and `_rpl_re_set_registers` for this file's six non-static
  definitions (verified with `nm -g`), so each keeps C's source name and emits the
  generated renamed symbol through `#[unsafe(export_name = "rpl_…")]` (PORTING.md 10.13,
  the regcomp.rs precedent); `nm -g build/femacs/regexec.o` shows exactly that set.
  `re_exec` (C 556-565) is dead (`_REGEX_RE_COMP` and `_LIBC` are both undefined) and has
  no port text.
- **The dynarray inclusion shim and the template's instantiation** (PORTING.md 10.13②). C
  1438-1441 defines `DYNARRAY_STRUCT`/`DYNARRAY_ELEMENT`/`DYNARRAY_PREFIX` and then textually
  includes `malloc/dynarray-skeleton.c`; Rust has no pre-processing include or `##` pasting,
  so the port keeps the three bindings as `macro_rules!` (the ledger's four macro rows),
  includes `malloc/dynarray-skeleton.rs` at that position, and instantiates the template's
  body - the prelude `macro_rules! dynarray_skeleton_body` - once, with the pasted
  `regmatch_list_*` names and the `regmatch_list`/`regmatch_list_u`/`regmatch_list_header`
  types as arguments (docs/ports/dynarray-skeleton.md records the parameterisation and the
  arms this binding selects: `DYNARRAY_ELEMENT_INIT`/`DYNARRAY_ELEMENT_FREE` and
  `DYNARRAY_FINAL_TYPE` undefined, `DYNARRAY_HAVE_SCRATCH` 1, so the `scratch` member, the
  empty `free__elements__`, C's `#else` resize arm and the `size_t *lengthp` finalize arm).
  C's anonymous `struct { size_t used; size_t allocated; DYNARRAY_ELEMENT *array; }` and
  anonymous `union { … } u;` need names in Rust, so the port gives them
  `regmatch_list_header`/`regmatch_list_u` (no layout change). The template's generic helpers
  are the four entries of `lib/malloc/dynarray_*.c`, and their declarations (with the shared
  header's two structs) now arrive from the `include/dynarray.rs` mirror that
  `regex_internal.h` 35's `#include <dynarray.h>` names, instead of from this unit's own
  `unsafe extern "C"` block. The generated `malloc/dynarray.gl.h` renames them, the mirror's
  declarations carry `#[link_name = "_gl_dynarray_…"]`, and `nm -u build/femacs/regexec.o`
  now shows the same undefined `_gl_dynarray_resize` the reference object has. The skeleton's
  `<errno.h>` is served by the `include/errno.rs` mirror (its one live use is
  `regmatch_list_finalize`'s `errno = ENOMEM`).
- **`goto` and labels** (PORTING.md 2.2). The file's ~80 `goto`s become labelled blocks and
  loops; each function's shape is recorded in `docs/ports/macro-body-notes.tsv`, one row per
  function, with the missing C keywords. The two `switch` dispatches keep C's case order as
  if/else-if chains (`re_search_internal`'s `match_kind`, `check_node_accept`'s
  `node->type`), `case 7`'s `goto forward_match_found_start_or_reached_end` is the shared
  tail the port writes once after the `match_kind == 7 || match_kind == 6` arm, and
  `check_node_accept`'s `FALLTHROUGH` is the shared `OP_UTF8_PERIOD || OP_PERIOD` arm.
  `expand_bkref_cache`'s backward `goto restart` is an outer `'restart` loop.
- **`do { … } while (ent++->more)`** appears five times (`check_dst_limits_calc_pos_1`,
  `get_subexp`, `sift_states_bkref`, `expand_bkref_cache`, and the two-increment form in
  `sift_states_bkref`). The body's `continue`s jump to that condition, so the port writes
  the body as a `'do_body` block whose `break 'do_body` stands for C's `continue`, and the
  loop's tail reads `more` and advances `ent` (and `enabled_idx`) explicitly.
- **`type` and other keyword-named C identifiers.** C locals/parameters named `type` are
  the Rust raw identifier `r#type` (PORTING.md 0.2), and the `re_token_t` field is read with
  the mirror's `type_ (…)`; the six functions whose C text declares
  `re_token_type_t type = …` name that type in `macro-body-notes.tsv` (the enum value is
  kept as the 8-bit integer the mirror exposes, compared against `OP_* as c_uint`).
- **Struct copies.** Four sites copy a whole C struct (`local_sctx = *sctx` in
  `sift_states_bkref`, `next_nodes = *log_nodes` in `merge_state_with_log`,
  `dest_nodes = *new_nodes` in `transit_state_mb`, plus `*cur_nodes = new_nodes` in
  `check_arrival_expand_ecl`, which is a plain assignment of the `Copy` mirror type); the
  three reads through a pointer are `std::ptr::read`, C's struct assignment.
- **Uninitialized locals** (PORTING.md 1.5). C leaves `char_len`/`elem_len`,
  `sifted_states`, `intersec`/`remains`, `accepts`, `dest_nodes`/`next_nodes`,
  `union_set`, `new_dests`, `except_nodes`, `cur_dest`, `sctx`/`local_sctx`/`mctx`,
  `prev_match` and the four dynarray-function `ok`/`result` results uninitialized on entry;
  the port gives each the deferred binding or `std::mem::zeroed ()` that C's first write
  overwrites. The three function-pointer-free structs the C interface passes by value are
  built with field lists or `zeroed ()` as the field order requires.
- **C's implicit conversions** are spelled, not assumed: `extend_buffers`'s `Idx` argument
  narrows `as c_int`, `max_mb_elem_len`'s update narrows `(to - from) as c_int`,
  `set_regs`'s `nmatch` widens `as usize`/`as Idx`, `re_search_internal`'s `nmatch` is the
  `size_t` parameter it is, the `offset < mctx.input.valid_len` test in its default arm is
  `as __re_size_t` (C's usual arithmetic conversions make that comparison unsigned), and
  `check_node_accept_bytes` converts `wint_t` to `wchar_t` and back through the two
  `wint_t`/`wchar_t` aliases the mirrors declare (darwin: `unsigned int`/`int`).
  `match_ctx_add_entry`'s `(from == to ? -1 : 0)` for a `bitset_word_t` field is
  `(-1isize) as bitset_word_t`.
- **`utf8_sb_map`** is `static const bitset_t` in regcomp.c 554 (owner:
  `lib/regcomp.rs`) and regexec.c 3551 reads it. Since the two files are separate
  objects in the port, this unit repeats the measured initializer
  (`[BITSET_WORD_MAX, BITSET_WORD_MAX, 0, 0]`) under `// C:` provenance; the read sits in
  the constant-false arm of `ASCII_CHARS % BITSET_WORD_BITS == 0` (128 % 64), so it is dead
  text and no symbol is needed.
- **Cross-unit statics.** `re_string_*`, `re_node_set_*`, `re_acquire_state*`,
  `free_state` and `build_wcs_buffer`/`build_wcs_upper_buffer`/`build_upper_buffer` are
  `static` in regex_internal.c, and `re_compile_fastmap` is regcomp.c's (renamed to
  `rpl_re_compile_fastmap` by the generated header); the port names each in its own
  `unsafe extern "C"` block with the C site in the comment, exactly as regcomp.rs does.
  `nm -u build/femacs/regexec.o` shows only those, the dynarray entry the live path uses
  (`_gl_dynarray_resize`, the same one the reference object undefineds) and libc.
- **Dead arms.** The file is in `docs/baseline/config/cond-eval-verified.txt`, so the
  `_LIBC` arms (C 157-160, 219-239, 276-278, 287-289, 299-301, 311-313, 338-343, 549-551,
  556-565, 3692-3694, 3778-3783, 3806-3895, 3921-3979) and the three `#if 0` blocks
  (C 119-123, 2293-2297, 2469-2509) are dead; the port carries no text for them and marks
  each spot, which also retires the `find_collation_sequence_value` prototype and
  definition, `__compat_regexec`, `re_exec` and `transit_state_sb`.
- **Line count.** C 4306 vs the port's 4359 (1.2%) after the dynarray instantiation moved
  into `lib/malloc/dynarray-skeleton.rs`'s macro; the tolerance row in
  `docs/ports/line-drift-tolerance.tsv` (`regexec 0.15`, whose reason still names the
  written-out instantiation and now needs a re-take) covers the labelled blocks, the
  if/else-if chains and the unit's fixed costs.
- **No new token families.** The file uses the existing `regs_allocated`, `type`,
  `constraint`, `halt`, `has_backref`, `has_constraint`, `context`, `no_sub`,
  `can_be_null`, `fastmap_accurate`, `newline_anchor`, `memcpy`/`sizeof` families from
  `docs/ports/token-aliases.tsv`; none was added or changed here.
